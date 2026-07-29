//! Asynchronous bulk-out writer.
//!
//! A synchronous write blocks the caller until the device has taken every byte,
//! which leaves the bus idle while the accelerator computes: the outfeed for an
//! inference arrives later than the bytes themselves need on the wire, and
//! nothing can be pushed during the difference. Submitting writes and collecting
//! their completions later lets that window carry the next stream.
//!
//! libusb processes transfers on one endpoint in submission order, so a stream's
//! header stays ahead of its data and streams stay separated, which the device's
//! protocol requires.

use std::os::raw::{c_int, c_void};
use std::time::{Duration, Instant};

use rusb::ffi::{self, constants::LIBUSB_TRANSFER_COMPLETED};

use crate::error::{Error, Result};

struct State {
    /// Writes still queued with the device.
    inflight: usize,
    /// A transfer completed with anything other than success.
    failed: bool,
}

/// Completion callback: one fewer write in flight.
extern "system" fn on_complete(xfer: *mut ffi::libusb_transfer) {
    unsafe {
        let st = (*xfer).user_data as *mut State;
        if (*xfer).status != LIBUSB_TRANSFER_COMPLETED {
            (*st).failed = true;
        }
        (*st).inflight -= 1;
    }
}

/// Writes queued on the bulk-out endpoints, completed by [`flush`](Self::flush).
///
/// Each submission owns its bytes, because the caller is free to drop the source
/// the moment `submit` returns.
pub struct BulkOut {
    ctx: *mut ffi::libusb_context,
    handle: *mut ffi::libusb_device_handle,
    state: *mut State,
    transfers: Vec<*mut ffi::libusb_transfer>,
    #[allow(dead_code)]
    buffers: Vec<Box<[u8]>>,
}

// SAFETY: `Send` moves exclusive ownership, and `&mut self` on every method rules
// out shared access. The state, transfers and buffers are allocated and freed
// here; `ctx` and `handle` are libusb objects, which rusb also treats as `Send`.
// Completion callbacks are the one other writer of the state, and they run only
// from this writer's own pumping, on the private context `Csr::open` opens.
unsafe impl Send for BulkOut {}

impl BulkOut {
    /// # Safety
    /// `handle` and `ctx` must be valid libusb pointers that outlive the writer.
    pub unsafe fn new(
        handle: *mut ffi::libusb_device_handle,
        ctx: *mut ffi::libusb_context,
    ) -> Self {
        Self {
            ctx,
            handle,
            state: Box::into_raw(Box::new(State {
                inflight: 0,
                failed: false,
            })),
            transfers: Vec::new(),
            buffers: Vec::new(),
        }
    }

    /// Queues `data` on `endpoint` and returns without waiting for the device.
    pub fn submit(&mut self, endpoint: u8, data: &[u8]) -> Result<()> {
        let mut owned: Box<[u8]> = data.to_vec().into_boxed_slice();
        unsafe {
            let xfer = ffi::libusb_alloc_transfer(0);
            if xfer.is_null() {
                return Err(Error::Usb(rusb::Error::NoMem));
            }
            ffi::libusb_fill_bulk_transfer(
                xfer,
                self.handle,
                endpoint,
                owned.as_mut_ptr(),
                owned.len() as c_int,
                on_complete,
                self.state as *mut c_void,
                0, // no per-write timeout; flush() bounds the whole wait
            );
            if ffi::libusb_submit_transfer(xfer) != 0 {
                ffi::libusb_free_transfer(xfer);
                return Err(Error::Usb(rusb::Error::Io));
            }
            (*self.state).inflight += 1;
            self.transfers.push(xfer);
        }
        self.buffers.push(owned);
        Ok(())
    }

    /// True while the device has not taken every queued write.
    pub fn pending(&self) -> bool {
        unsafe { (*self.state).inflight > 0 }
    }

    /// Waits for every queued write to complete, then releases them.
    pub fn flush(&mut self, timeout: Duration) -> Result<()> {
        let start = Instant::now();
        unsafe {
            while (*self.state).inflight > 0 {
                if (*self.state).failed {
                    self.abort();
                    return Err(Error::InvalidTransfer);
                }
                let tv = libc::timeval {
                    tv_sec: 0,
                    tv_usec: 50_000,
                };
                if ffi::libusb_handle_events_timeout(self.ctx, &tv) < 0 {
                    self.abort();
                    return Err(Error::Usb(rusb::Error::Io));
                }
                if start.elapsed() > timeout {
                    self.abort();
                    return Err(Error::Usb(rusb::Error::Timeout));
                }
            }
            let failed = (*self.state).failed;
            self.release();
            if failed {
                return Err(Error::InvalidTransfer);
            }
        }
        Ok(())
    }

    /// Cancels queued writes and waits for their callbacks. libusb owns a transfer
    /// until it completes, so freeing one still in flight is undefined.
    fn abort(&mut self) {
        unsafe {
            for &xfer in &self.transfers {
                ffi::libusb_cancel_transfer(xfer);
            }
            let start = Instant::now();
            while (*self.state).inflight > 0 && start.elapsed() < Duration::from_secs(1) {
                let tv = libc::timeval {
                    tv_sec: 0,
                    tv_usec: 50_000,
                };
                ffi::libusb_handle_events_timeout(self.ctx, &tv);
            }
        }
        self.release();
    }

    /// Frees completed transfers and the bytes they referenced.
    fn release(&mut self) {
        unsafe {
            for &xfer in &self.transfers {
                ffi::libusb_free_transfer(xfer);
            }
        }
        self.transfers.clear();
        self.buffers.clear();
        unsafe {
            (*self.state).failed = false;
        }
    }
}

impl Drop for BulkOut {
    fn drop(&mut self) {
        self.abort();
        unsafe {
            drop(Box::from_raw(self.state));
        }
    }
}
