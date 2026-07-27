//! USB DFU firmware download for the Coral bootloader. Port of libedgetpu's
//! UsbDfuCommands: flashing the Apex image makes the DFU device (1a6e:089a)
//! re-enumerate as the runtime device (18d1:9302).

use std::time::Duration;

use rusb::{request_type, Direction, GlobalContext, Recipient, RequestType};

use crate::error::{Error, Result};
use crate::usb;

const DFU_DNLOAD: u8 = 1;
const DFU_GETSTATUS: u8 = 3;

const STATE_DFU_IDLE: u8 = 2;
const STATE_DOWNLOAD_IDLE: u8 = 5;

const DFU_FUNCTIONAL_DESCRIPTOR: u8 = 0x21;
const DFU_INTERFACE_CLASS: u8 = 0xfe;
const DFU_INTERFACE_SUBCLASS: u8 = 0x01;

/// Fallback used when the DFU functional descriptor is absent from the parsed
/// interface; every control transfer stays at or below ep0's max packet size.
const DEFAULT_TRANSFER_SIZE: u16 = 1024;

const TIMEOUT: Duration = Duration::from_secs(2);

/// Reads the DFU functional descriptor's `wTransferSize` (bytes 5..7) from the
/// class-specific descriptor bytes that follow the interface descriptor.
pub fn parse_transfer_size(extra: &[u8]) -> Option<u16> {
    let mut cursor = 0usize;
    while cursor + 1 < extra.len() {
        let length = extra[cursor] as usize;
        if length == 0 {
            return None;
        }
        if extra[cursor + 1] == DFU_FUNCTIONAL_DESCRIPTOR && cursor + 7 <= extra.len() {
            return Some(u16::from_le_bytes([extra[cursor + 5], extra[cursor + 6]]));
        }
        cursor += length;
    }
    None
}

type Handle = rusb::DeviceHandle<GlobalContext>;

fn find_dfu_interface(handle: &Handle) -> Result<(u8, u16)> {
    let config = handle.device().config_descriptor(0)?;
    for iface in config.interfaces() {
        for desc in iface.descriptors() {
            if desc.class_code() == DFU_INTERFACE_CLASS
                && desc.sub_class_code() == DFU_INTERFACE_SUBCLASS
                && desc.num_endpoints() == 0
            {
                let transfer_size =
                    parse_transfer_size(desc.extra()).unwrap_or(DEFAULT_TRANSFER_SIZE);
                return Ok((desc.interface_number(), transfer_size));
            }
        }
    }
    Err(Error::NoDfuInterface)
}

fn download(handle: &Handle, interface: u8, transfer_size: u16, firmware: &[u8]) -> Result<()> {
    let out = request_type(Direction::Out, RequestType::Class, Recipient::Interface);
    let get = request_type(Direction::In, RequestType::Class, Recipient::Interface);

    let mut block: u16 = 0;
    let mut sent = 0usize;
    loop {
        let chunk = (transfer_size as usize).min(firmware.len() - sent);
        handle.write_control(
            out,
            DFU_DNLOAD,
            block,
            interface as u16,
            &firmware[sent..sent + chunk],
            TIMEOUT,
        )?;

        let mut status = [0u8; 6];
        if handle.read_control(
            get,
            DFU_GETSTATUS,
            0,
            interface as u16,
            &mut status,
            TIMEOUT,
        )? != 6
        {
            return Err(Error::ShortStatus);
        }
        let result = status[0];
        let poll =
            u32::from(status[1]) | (u32::from(status[2]) << 8) | (u32::from(status[3]) << 16);
        let state = status[4];

        if result == 0 && state == STATE_DOWNLOAD_IDLE {
            sent += chunk;
        } else if chunk == 0 && result == 0 && state == STATE_DFU_IDLE {
            return Ok(());
        } else {
            return Err(Error::DownloadFailed {
                block,
                result,
                state,
            });
        }

        if poll > 0 {
            std::thread::sleep(Duration::from_millis(u64::from(poll)));
        }
        block = block.wrapping_add(1);
    }
}

/// Flashes `firmware` onto the attached Coral DFU bootloader and resets it so it
/// re-enumerates in runtime mode.
pub fn flash(firmware: &[u8]) -> Result<()> {
    let handle = rusb::open_device_with_vid_pid(usb::DFU_VENDOR, usb::DFU_PRODUCT)
        .ok_or(Error::DeviceNotFound)?;
    let (interface, transfer_size) = find_dfu_interface(&handle)?;
    handle.claim_interface(interface)?;
    download(&handle, interface, transfer_size, firmware)?;
    let _ = handle.reset();
    Ok(())
}
