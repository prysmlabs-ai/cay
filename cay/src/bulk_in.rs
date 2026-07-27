//! Asynchronous bulk-in read pool for the output-activation endpoint, ported
//! from libedgetpu's UsbDriver. Read buffers are submitted *before* the input
//! streams out so the outfeed always has somewhere to drain; with only a
//! blocking read posted after the send, the outfeed FIFO backs up and
//! backpressures the compute pipeline, corrupting large outputs.
//!
//! libusb async: `submit_transfer` queues each read, `handle_events` runs the
//! completion callbacks. Single-threaded — callback and collect loop never run
//! concurrently — so the state behind a raw pointer needs no locking.

use std::os::raw::{c_int, c_void};
use std::time::{Duration, Instant};

use rusb::ffi::{self, constants::LIBUSB_TRANSFER_COMPLETED};

use crate::error::{Error, Result};

struct State {
    /// Bytes received so far, in on-the-wire order across all buffers.
    data: Vec<u8>,
    /// Reads still queued with the device.
    inflight: usize,
    /// Set while tearing down: completed reads are not re-queued.
    stop: bool,
    /// A resubmit failed; the collect loop should give up.
    failed: bool,
}

/// Completion callback: append the received bytes and re-queue the buffer so the
/// outfeed keeps draining, unless we are stopping.
extern "system" fn on_complete(xfer: *mut ffi::libusb_transfer) {
    unsafe {
        let st = (*xfer).user_data as *mut State;
        if (*xfer).status == LIBUSB_TRANSFER_COMPLETED {
            let n = (*xfer).actual_length as usize;
            if n > 0 {
                let src = std::slice::from_raw_parts((*xfer).buffer, n);
                (*st).data.extend_from_slice(src);
            }
            if !(*st).stop {
                if ffi::libusb_submit_transfer(xfer) != 0 {
                    (*st).failed = true;
                    (*st).inflight -= 1;
                }
                return;
            }
        }
        // Cancelled, errored, or stopping: this read is no longer flying.
        (*st).inflight -= 1;
    }
}

/// A pool of pre-submitted bulk-in reads over one endpoint. Buffers cycle: each
/// completion re-queues its buffer, so `count * chunk` bytes of read capacity
/// stay outstanding until [`collect`](Self::collect) has drained the output.
pub struct BulkInPool {
    ctx: *mut ffi::libusb_context,
    transfers: Vec<*mut ffi::libusb_transfer>,
    // Backing storage the transfers point into; held for the pool's life.
    #[allow(dead_code)]
    buffers: Vec<Vec<u8>>,
    state: *mut State,
    // Bytes already handed out by collect().
    cursor: usize,
}

impl BulkInPool {
    /// Allocates `count` reads of `chunk` bytes on `endpoint` and submits them
    /// all, so they are outstanding before any output streams back.
    ///
    /// # Safety
    /// `handle` and `ctx` must be valid libusb pointers that outlive the pool.
    pub unsafe fn new(
        handle: *mut ffi::libusb_device_handle,
        ctx: *mut ffi::libusb_context,
        endpoint: u8,
        count: usize,
        chunk: usize,
    ) -> Result<Self> {
        let state = Box::into_raw(Box::new(State {
            data: Vec::new(),
            inflight: 0,
            stop: false,
            failed: false,
        }));
        let mut buffers: Vec<Vec<u8>> = (0..count).map(|_| vec![0u8; chunk]).collect();
        let mut transfers: Vec<*mut ffi::libusb_transfer> = Vec::with_capacity(count);
        unsafe {
            for buf in buffers.iter_mut() {
                let xfer = ffi::libusb_alloc_transfer(0);
                if xfer.is_null() {
                    for &x in &transfers {
                        ffi::libusb_free_transfer(x);
                    }
                    drop(Box::from_raw(state));
                    return Err(Error::Usb(rusb::Error::NoMem));
                }
                ffi::libusb_fill_bulk_transfer(
                    xfer,
                    handle,
                    endpoint,
                    buf.as_mut_ptr(),
                    chunk as c_int,
                    on_complete,
                    state as *mut c_void,
                    0, // no per-read timeout; collect() bounds the whole wait
                );
                transfers.push(xfer);
            }
            for &xfer in &transfers {
                if ffi::libusb_submit_transfer(xfer) == 0 {
                    (*state).inflight += 1;
                } else {
                    (*state).failed = true;
                }
            }
        }
        Ok(Self {
            ctx,
            transfers,
            buffers,
            state,
            cursor: 0,
        })
    }

    /// Returns the next `n` bytes of the output stream, pumping libusb events
    /// until they have arrived or `timeout` elapses.
    pub fn collect(&mut self, n: usize, timeout: Duration) -> Result<Vec<u8>> {
        let start = Instant::now();
        unsafe {
            while (*self.state).data.len() - self.cursor < n {
                if (*self.state).failed {
                    return Err(Error::InvalidTransfer);
                }
                let tv = libc::timeval {
                    tv_sec: 0,
                    tv_usec: 100_000,
                };
                if ffi::libusb_handle_events_timeout(self.ctx, &tv) < 0 {
                    return Err(Error::Usb(rusb::Error::Io));
                }
                if start.elapsed() > timeout {
                    return Err(Error::Timeout(u32::from(super::ml::BULK_IN)));
                }
            }
            let data = &(*self.state).data;
            let out = data[self.cursor..self.cursor + n].to_vec();
            self.cursor += n;
            Ok(out)
        }
    }
}

impl Drop for BulkInPool {
    fn drop(&mut self) {
        unsafe {
            (*self.state).stop = true;
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
            for &xfer in &self.transfers {
                ffi::libusb_free_transfer(xfer);
            }
            drop(Box::from_raw(self.state));
        }
    }
}
