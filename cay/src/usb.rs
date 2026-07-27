//! USB transport for the Coral Edge TPU. The accelerator enumerates in one of
//! two states: an unflashed DFU bootloader (Global Unichip 1a6e:089a) that needs
//! the Apex firmware pushed, and the runtime device (Google 18d1:9302) that runs
//! inference.

use std::fmt;

pub const DFU_VENDOR: u16 = 0x1a6e;
pub const DFU_PRODUCT: u16 = 0x089a;
pub const RUNTIME_VENDOR: u16 = 0x18d1;
pub const RUNTIME_PRODUCT: u16 = 0x9302;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Unflashed bootloader; needs the Apex firmware before it can run.
    Dfu,
    /// Firmware loaded; ready for inference.
    Runtime,
}

/// Maps a USB (vendor, product) to a Coral state, or `None` for other devices.
pub fn classify(vendor: u16, product: u16) -> Option<State> {
    match (vendor, product) {
        (DFU_VENDOR, DFU_PRODUCT) => Some(State::Dfu),
        (RUNTIME_VENDOR, RUNTIME_PRODUCT) => Some(State::Runtime),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct Found {
    pub state: State,
    pub bus: u8,
    pub address: u8,
}

impl fmt::Display for Found {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Coral {:?} on bus {} addr {}",
            self.state, self.bus, self.address
        )
    }
}

/// Enumerates attached Coral accelerators in either state.
pub fn find_coral() -> rusb::Result<Vec<Found>> {
    let mut out = Vec::new();
    for dev in rusb::devices()?.iter() {
        let desc = dev.device_descriptor()?;
        if let Some(state) = classify(desc.vendor_id(), desc.product_id()) {
            out.push(Found {
                state,
                bus: dev.bus_number(),
                address: dev.address(),
            });
        }
    }
    Ok(out)
}
