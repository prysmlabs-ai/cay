//! Reader for the DarwiNN Edge TPU executable format. Port of libedgetpu's
//! executable schema: a compiled model's custom-op payload is a `Package` whose
//! executables carry opaque instruction bitstreams the driver DMAs to the device.

#[rustfmt::skip]
#[allow(clippy::all, dead_code)]
mod executable_generated;

pub mod error;
mod fb;
mod package;
mod patch;
mod program;
mod relayout;
mod streams;

pub use error::{Error, Result};
pub use executable_generated::platforms::darwinn as schema;
pub use package::{
    executable_blobs, extract_package, extract_packages, multi_executable_bytes, parse_executable,
    parse_package,
};
pub use patch::{
    copy_u32, field_sites, link_input, link_output, link_parameter, link_scratch, read_u32,
    FieldSite,
};
pub use program::{Action, Buffer, InputSpec, Phase, Program};
pub use relayout::{relayout, transform_signed};
pub use streams::{inference_streams, Streams};
