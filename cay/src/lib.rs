//! Userspace runtime for the Coral Edge TPU. Rust port of libedgetpu: it drives
//! the accelerator directly over USB with no TFLite delegate or vendor binary.

pub mod bulk_in;
pub mod csr;
pub mod dfu;
pub mod dma;
pub mod driver;
pub mod error;
pub mod ml;
pub mod program;
pub mod usb;
