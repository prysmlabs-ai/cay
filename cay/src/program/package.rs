//! Reads a DarwiNN `Package` and the executables it carries. Instruction
//! bitstreams stay opaque bytes — this layer never interprets the device ISA.

use crate::program::error::{Error, Result};
use crate::program::fb;
use crate::program::schema::{ExecutableRef, PackageRef};
use planus::ReadAsRoot;

const FILE_IDENTIFIER: [u8; 4] = *b"DWN1";

/// Reads a top-level package, rejecting buffers without the "DWN1" identifier.
pub fn parse_package(bytes: &[u8]) -> Result<PackageRef<'_>> {
    if bytes.get(4..8) != Some(&FILE_IDENTIFIER[..]) {
        return Err(Error::BadIdentifier);
    }
    Ok(PackageRef::read_as_root(bytes)?)
}

/// The serialized MultiExecutable buffer inside a package.
pub fn multi_executable_bytes<'a>(pkg: &PackageRef<'a>) -> Result<&'a [u8]> {
    pkg.serialized_multi_executable()?
        .ok_or(Error::MissingField("serialized_multi_executable"))
}

/// The serialized Executable buffers inside a MultiExecutable.
pub fn executable_blobs(multi_executable: &[u8]) -> Result<Vec<&[u8]>> {
    fb::executable_blobs(multi_executable)
}

/// Reads one serialized Executable.
pub fn parse_executable(bytes: &[u8]) -> Result<ExecutableRef<'_>> {
    Ok(ExecutableRef::read_as_root(bytes)?)
}

/// Finds the embedded DarwiNN package inside an Edge TPU `.tflite` by its "DWN1"
/// file identifier — the package is the edgetpu custom-op's `custom_options`.
/// The returned slice runs to end-of-file; the flatbuffer reader ignores the
/// trailing bytes.
pub fn extract_package(tflite: &[u8]) -> Option<&[u8]> {
    let id = tflite.windows(4).position(|w| w == &FILE_IDENTIFIER[..])?;
    id.checked_sub(4).map(|start| &tflite[start..])
}

/// Every embedded package, in file order — a model split around a host op the
/// Edge TPU can't run carries one per edge-TPU subgraph. Each slice runs to
/// end-of-file; the flatbuffer reader ignores the trailing bytes (including any
/// later package). A stray "DWN1" in parameter data is rejected by requiring the
/// slice to parse as a package with a MultiExecutable.
pub fn extract_packages(tflite: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut off = 0;
    while let Some(rel) = tflite[off..]
        .windows(4)
        .position(|w| w == &FILE_IDENTIFIER[..])
    {
        let id = off + rel;
        off = id + 4;
        let Some(start) = id.checked_sub(4) else {
            continue;
        };
        let slice = &tflite[start..];
        if parse_package(slice).is_ok_and(|pkg| multi_executable_bytes(&pkg).is_ok()) {
            out.push(slice);
        }
    }
    out
}
