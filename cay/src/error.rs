use std::fmt;

#[derive(Debug)]
pub enum Error {
    Usb(rusb::Error),
    /// No device matching the Coral DFU bootloader id was open-able.
    DeviceNotFound,
    /// The device exposes no DFU interface (class 0xFE, subclass 1).
    NoDfuInterface,
    /// A DFU GetStatus reply was not the expected 6 bytes.
    ShortStatus,
    /// A CSR read returned the wrong number of bytes.
    ShortRegister,
    /// A DMA transfer notification exceeded the active or total byte count.
    InvalidTransfer,
    /// A CSR poll did not reach its expected value in time.
    Timeout(u32),
    /// An event/interrupt read returned fewer bytes than the descriptor size.
    ShortEvent,
    /// A model failed to parse into an inference program.
    Parse(String),
    /// The accelerator's HIB error status was non-zero after a run.
    HardwareError {
        status: u64,
        first: u64,
    },
    /// The device reported an error mid-download.
    DownloadFailed {
        block: u16,
        result: u8,
        state: u8,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Usb(e) => write!(f, "usb error: {e}"),
            Error::DeviceNotFound => write!(f, "no Coral DFU device found"),
            Error::NoDfuInterface => write!(f, "device has no DFU interface"),
            Error::ShortStatus => write!(f, "short DFU status reply"),
            Error::ShortRegister => write!(f, "short CSR read reply"),
            Error::InvalidTransfer => write!(f, "DMA transfer notification out of range"),
            Error::Timeout(off) => write!(f, "CSR poll timeout at offset {off:#x}"),
            Error::ShortEvent => write!(f, "short event/interrupt read"),
            Error::Parse(m) => write!(f, "parse error: {m}"),
            Error::HardwareError { status, first } => {
                write!(f, "HIB error status {status:#x} (first {first:#x})")
            }
            Error::DownloadFailed {
                block,
                result,
                state,
            } => write!(
                f,
                "DFU download failed at block {block} (result {result}, state {state})"
            ),
        }
    }
}

impl std::error::Error for Error {}

impl From<rusb::Error> for Error {
    fn from(e: rusb::Error) -> Self {
        Error::Usb(e)
    }
}

impl From<cay_program::Error> for Error {
    fn from(e: cay_program::Error) -> Self {
        Error::Parse(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
