//! Machine-learning command layer for the Coral runtime device. Port of
//! libedgetpu's UsbMlCommands register access: a CSR is read or written with a
//! vendor control transfer whose 32-bit offset is split across wValue (low 16)
//! and wIndex (high 16); request id 1 is 32-bit, 0 is 64-bit.

use std::time::Duration;

use rusb::{request_type, Direction, GlobalContext, Recipient, RequestType, UsbContext};

use crate::error::{Error, Result};
use crate::usb;

const REQ_CSR_64: u8 = 0;
const REQ_CSR_32: u8 = 1;
const TIMEOUT: Duration = Duration::from_secs(2);

/// The control-transfer fields for one CSR access, split out so the encoding is
/// testable without a device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CsrTransfer {
    pub request_type: u8,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub length: usize,
}

fn transfer(direction: Direction, offset: u32, width: usize) -> CsrTransfer {
    CsrTransfer {
        request_type: request_type(direction, RequestType::Vendor, Recipient::Device),
        request: if width == 8 { REQ_CSR_64 } else { REQ_CSR_32 },
        value: (offset & 0xffff) as u16,
        index: (offset >> 16) as u16,
        length: width,
    }
}

pub fn read_transfer(offset: u32, width: usize) -> CsrTransfer {
    transfer(Direction::In, offset, width)
}

pub fn write_transfer(offset: u32, width: usize) -> CsrTransfer {
    transfer(Direction::Out, offset, width)
}

/// Bulk endpoint addresses in single-bulk-out mode.
pub const BULK_OUT: u8 = 0x01;
pub const BULK_IN: u8 = 0x81;
pub const EVENT_IN: u8 = 0x82;
pub const INTERRUPT_IN: u8 = 0x83;

/// Stream kinds carried by the 8-byte bulk-out packet header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorTag {
    Instructions = 0,
    InputActivations = 1,
    Parameters = 2,
    OutputActivations = 3,
}

/// The 8-byte header preceding each stream on the single bulk-out endpoint: the
/// stream length as a little-endian u32, then the tag in the low nibble of byte 4.
pub fn packet_header(tag: DescriptorTag, length: u32) -> [u8; 8] {
    let mut header = [0u8; 8];
    header[..4].copy_from_slice(&length.to_le_bytes());
    header[4] = (tag as u8) & 0x0f;
    header
}

/// A device→host event descriptor (16 bytes on the event endpoint): the device
/// address of a completed DMA, its byte length, and the stream tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventDescriptor {
    pub offset: u64,
    pub length: u32,
    pub tag: u8,
}

/// Parses a 16-byte event descriptor.
pub fn parse_event(bytes: &[u8]) -> Option<EventDescriptor> {
    let b: [u8; 16] = bytes.get(..16)?.try_into().ok()?;
    Some(EventDescriptor {
        offset: u64::from_le_bytes(b[0..8].try_into().unwrap()),
        length: u32::from_le_bytes(b[8..12].try_into().unwrap()),
        tag: b[12] & 0x0f,
    })
}

/// Parses a 4-byte interrupt word.
pub fn parse_interrupt(bytes: &[u8]) -> Option<u32> {
    let b: [u8; 4] = bytes.get(..4)?.try_into().ok()?;
    Some(u32::from_le_bytes(b))
}

/// A CSR channel over the open runtime device.
pub struct Csr {
    handle: rusb::DeviceHandle<GlobalContext>,
}

impl Csr {
    /// Opens the runtime-mode Coral (18d1:9302).
    pub fn open() -> Result<Self> {
        let handle = rusb::open_device_with_vid_pid(usb::RUNTIME_VENDOR, usb::RUNTIME_PRODUCT)
            .ok_or(Error::DeviceNotFound)?;
        let _ = handle.set_active_configuration(1);
        Ok(Self { handle })
    }

    pub fn read32(&self, offset: u32) -> Result<u32> {
        let t = read_transfer(offset, 4);
        let mut buf = [0u8; 4];
        if self.handle.read_control(
            t.request_type,
            t.request,
            t.value,
            t.index,
            &mut buf,
            TIMEOUT,
        )? != 4
        {
            return Err(Error::ShortRegister);
        }
        Ok(u32::from_le_bytes(buf))
    }

    pub fn read64(&self, offset: u32) -> Result<u64> {
        let t = read_transfer(offset, 8);
        let mut buf = [0u8; 8];
        if self.handle.read_control(
            t.request_type,
            t.request,
            t.value,
            t.index,
            &mut buf,
            TIMEOUT,
        )? != 8
        {
            return Err(Error::ShortRegister);
        }
        Ok(u64::from_le_bytes(buf))
    }

    pub fn write32(&self, offset: u32, value: u32) -> Result<()> {
        let t = write_transfer(offset, 4);
        self.handle.write_control(
            t.request_type,
            t.request,
            t.value,
            t.index,
            &value.to_le_bytes(),
            TIMEOUT,
        )?;
        Ok(())
    }

    pub fn write64(&self, offset: u32, value: u64) -> Result<()> {
        let t = write_transfer(offset, 8);
        self.handle.write_control(
            t.request_type,
            t.request,
            t.value,
            t.index,
            &value.to_le_bytes(),
            TIMEOUT,
        )?;
        Ok(())
    }

    /// Claims the ML interface so the bulk endpoints can be used.
    pub fn claim(&self, interface: u8) -> Result<()> {
        self.handle.claim_interface(interface)?;
        Ok(())
    }

    /// Writes the 8-byte stream header to the single bulk-out endpoint.
    pub fn write_header(&self, tag: DescriptorTag, length: u32) -> Result<()> {
        self.handle
            .write_bulk(BULK_OUT, &packet_header(tag, length), TIMEOUT)?;
        Ok(())
    }

    /// Writes a stream to the single bulk-out endpoint; returns bytes sent.
    pub fn bulk_out(&self, data: &[u8]) -> Result<usize> {
        Ok(self.handle.write_bulk(BULK_OUT, data, TIMEOUT)?)
    }

    /// Reads output activations from the bulk-in endpoint; returns bytes read.
    pub fn bulk_in(&self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.handle.read_bulk(BULK_IN, buf, TIMEOUT)?)
    }

    /// Reads one event descriptor from the event endpoint.
    pub fn read_event(&self) -> Result<EventDescriptor> {
        let mut buf = [0u8; 16];
        let n = self.handle.read_bulk(EVENT_IN, &mut buf, TIMEOUT)?;
        parse_event(&buf[..n]).ok_or(Error::ShortEvent)
    }

    /// Reads one raw interrupt word from the interrupt endpoint.
    pub fn read_interrupt(&self) -> Result<u32> {
        let mut buf = [0u8; 4];
        let n = self
            .handle
            .read_interrupt(INTERRUPT_IN, &mut buf, TIMEOUT)?;
        parse_interrupt(&buf[..n]).ok_or(Error::ShortEvent)
    }

    /// Clears any halted (stalled) endpoints so a failed transfer leaves the
    /// device recoverable rather than wedged off the bus.
    pub fn clear_halts(&self) {
        for ep in [BULK_OUT, BULK_IN, EVENT_IN, INTERRUPT_IN] {
            let _ = self.handle.clear_halt(ep);
        }
    }

    /// Issues a USB port reset. Recovers a device wedged badly enough that CSR
    /// control transfers time out — a host-level reset that does not depend on the
    /// accelerator's own state, so it works short of a physical replug.
    pub fn reset(&self) -> Result<()> {
        self.handle.reset()?;
        Ok(())
    }

    /// True when the device enumerated at USB2 High Speed. libedgetpu caps
    /// bulk-in reads at 256 B and shortens the outfeed chunk for this speed
    /// (b/73181174), so the runtime must know it.
    pub fn is_high_speed(&self) -> bool {
        self.handle.device().speed() == rusb::Speed::High
    }

    /// Raw libusb handle for the async bulk-in path ([`crate::bulk_in`]).
    pub fn raw_handle(&self) -> *mut rusb::ffi::libusb_device_handle {
        self.handle.as_raw()
    }

    /// Raw libusb context backing this handle, for `handle_events`.
    pub fn raw_context(&self) -> *mut rusb::ffi::libusb_context {
        GlobalContext::default().as_raw()
    }
}
