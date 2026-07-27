//! Host-side inference streams assembled from a parsed package: the cached
//! parameters (from the parameter-caching executable), the execution
//! instructions, and the input/output byte sizes from the layer metadata.

use crate::error::{Error, Result};
use crate::package::{executable_blobs, multi_executable_bytes, parse_executable, parse_package};
use crate::schema::{ExecutableRef, ExecutableType};

#[derive(Debug, Default, Clone)]
pub struct Streams {
    pub parameters: Vec<u8>,
    pub exec_instructions: Vec<u8>,
    pub input_bytes: usize,
    pub output_bytes: usize,
}

fn instructions(exec: ExecutableRef<'_>) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    if let Some(v) = exec.instruction_bitstreams()? {
        for ib in v.iter() {
            if let Some(bs) = ib?.bitstream()? {
                out.extend_from_slice(bs);
            }
        }
    }
    Ok(out)
}

fn first_layer_bytes(exec: ExecutableRef<'_>, output: bool) -> Result<usize> {
    let layers = if output {
        exec.output_layers()?
    } else {
        exec.input_layers()?
    };
    let Some(v) = layers else { return Ok(0) };
    match v.iter().next() {
        Some(layer) => Ok(usize::try_from(layer?.size_bytes()?).unwrap_or(0)),
        None => Ok(0),
    }
}

/// Extracts the inference streams from a serialized package. Handles both the
/// parameter-caching split (weights in a PARAMETER_CACHING executable, work in an
/// EXECUTION_ONLY one) and a single STAND_ALONE executable.
pub fn inference_streams(pkg_bytes: &[u8]) -> Result<Streams> {
    let pkg = parse_package(pkg_bytes)?;
    let multi = multi_executable_bytes(&pkg)?;
    let blobs = executable_blobs(multi)?;

    let mut s = Streams::default();
    for blob in &blobs {
        let exec = parse_executable(blob)?;
        match exec.type_()? {
            ExecutableType::ParameterCaching => {
                if let Some(p) = exec.parameters()? {
                    s.parameters = p.to_vec();
                }
            }
            _ => {
                s.exec_instructions = instructions(exec)?;
                s.input_bytes = first_layer_bytes(exec, false)?;
                s.output_bytes = first_layer_bytes(exec, true)?;
                if s.parameters.is_empty() {
                    if let Some(p) = exec.parameters()? {
                        s.parameters = p.to_vec();
                    }
                }
            }
        }
    }

    if s.exec_instructions.is_empty() {
        return Err(Error::MissingField("execution instructions"));
    }
    Ok(s)
}
