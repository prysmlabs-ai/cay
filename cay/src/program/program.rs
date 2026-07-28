//! A linkable, hint-driven inference program built from a parsed package. Each
//! phase (parameter-caching, then execution) carries its instruction-bitstream
//! chunks, the field sites to link in each, and the ordered DMA hint actions the
//! host follows to stream data over the single bulk endpoint. Supports models
//! with multiple output tensors (e.g. detection heads).

use crate::program::error::{Error, Result};
use crate::program::package::{
    executable_blobs, multi_executable_bytes, parse_executable, parse_package,
};
use crate::program::patch::{field_sites, FieldSite};
use crate::program::schema::{
    AnyHintRef, AnyLayerRef, DataType, Description, Direction, ExecutableRef, ExecutableType,
    OutputLayout,
};

/// FNV-1a hash, used to fingerprint a phase's parameter blob.
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Buffer {
    Parameter,
    Input,
    Output,
    Scratch,
}

#[derive(Debug, Clone)]
pub enum Action {
    /// Send instruction-bitstream chunk `usize`.
    Instruction(usize),
    /// Stream a buffer region in (infeed) or out (outfeed). `name` selects which
    /// output for multi-output models.
    Dma {
        buffer: Buffer,
        name: String,
        offset: usize,
        size: usize,
        infeed: bool,
    },
    /// Wait for the terminating completion event.
    Interrupt,
    Fence,
}

#[derive(Debug, Clone)]
pub struct OutputSpec {
    pub name: String,
    /// Size of the raw (tiled/padded) DMA buffer the accelerator writes.
    pub bytes: usize,
    pub scale: f32,
    pub zero_point: i32,
    /// Tile→row-major map; `None` when the tensor is already linear (x or y == 1).
    pub layout: Option<OutputLayout>,
    /// z_dim * bytes_per_element — the size of one (y, x) element block.
    pub element_bytes: usize,
    /// Unpadded x*y*z*bytes size of the logical tensor.
    pub logical_bytes: usize,
    /// True for signed layers, whose output the accelerator emits biased-unsigned;
    /// the sign bit of each scalar is flipped back after relayout.
    pub signed: bool,
    /// Bytes per scalar element (1 for 8-bit, 2 for 16-bit), the sign-flip stride.
    pub scalar_bytes: usize,
    /// True for a recurrent state buffer (name ends `_variable_output`): the
    /// accelerator writes it but it is not a user-visible output.
    pub variable: bool,
    /// Logical NHWC dims `[H, W, C]` of the output tensor (cay_program y, x, z).
    pub dims: [usize; 3],
}

#[derive(Debug, Clone)]
pub struct InputSpec {
    pub name: String,
    /// Size of the DMA buffer fed for this input.
    pub bytes: usize,
    pub scale: f32,
    pub zero_point: i32,
    /// True for signed layers, fed biased-unsigned; the sign bit of each scalar
    /// is flipped before the DMA.
    pub signed: bool,
    /// Bytes per scalar element, the sign-flip stride.
    pub scalar_bytes: usize,
}

#[derive(Debug, Clone)]
pub struct Phase {
    pub chunks: Vec<Vec<u8>>,
    pub chunk_sites: Vec<Vec<FieldSite>>,
    pub actions: Vec<Action>,
    /// This executable's own parameters. Each executable carries its own set: a
    /// ParameterCaching phase streams weights to be cached, and an ExecutionOnly
    /// phase may still stream per-execution weights of its own.
    pub parameters: Vec<u8>,
    /// True for the ParameterCaching executable.
    pub caches_parameters: bool,
    /// Non-zero marks the weights as cacheable. Co-compiled segments share one
    /// token (it names the caching group, not the individual weight set), so it
    /// alone cannot tell two segments' caches apart — `param_fingerprint` does.
    pub parameter_caching_token: u64,
    /// Content hash of this phase's parameters: the identity a driver matches to
    /// decide the same weights are already resident on-chip (0 = no parameters).
    pub param_fingerprint: u64,
    /// False when the DMA order isn't fixed at compile time; such a phase runs
    /// under the device-driven descriptor mode instead of the static hints.
    pub fully_deterministic: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub phases: Vec<Phase>,
    /// Input tensors in declaration order; index 0 is the primary activation.
    pub inputs: Vec<InputSpec>,
    pub outputs: Vec<OutputSpec>,
    pub scratch_bytes: usize,
}

/// Instruction-bitstream chunks paired with the field sites to link in each.
type LinkedChunks = (Vec<Vec<u8>>, Vec<Vec<FieldSite>>);

fn read_chunks(exec: ExecutableRef<'_>) -> Result<LinkedChunks> {
    let mut chunks = Vec::new();
    let mut sites = Vec::new();
    if let Some(ibs) = exec.instruction_bitstreams()? {
        for ib in ibs.iter() {
            let ib = ib?;
            chunks.push(ib.bitstream()?.map(<[u8]>::to_vec).unwrap_or_default());
            sites.push(field_sites(ib)?);
        }
    }
    Ok((chunks, sites))
}

fn buffer_of(desc: Option<Description>) -> Buffer {
    match desc {
        Some(Description::BaseAddressParameter) => Buffer::Parameter,
        Some(Description::BaseAddressInputActivation) => Buffer::Input,
        Some(Description::BaseAddressOutputActivation) => Buffer::Output,
        _ => Buffer::Scratch,
    }
}

struct InLayerInfo {
    scale: f32,
    zero_point: i32,
    signed: bool,
    scalar_bytes: usize,
}

/// Numerics and signedness of the input layer named `name`. Signed inputs are
/// fed biased-unsigned, so their sign bit is flipped before the DMA.
fn input_layer_info(exec: ExecutableRef<'_>, name: &str) -> Result<Option<InLayerInfo>> {
    let Some(v) = exec.input_layers()? else {
        return Ok(None);
    };
    for layer in v.iter() {
        let layer = layer?;
        if layer.name()?.map(str::to_owned).as_deref() != Some(name) {
            continue;
        }
        let (scale, zero_point) = match layer.numerics()? {
            Some(n) => (n.dequantization_factor()?, n.zero_point()?),
            None => (1.0, 0),
        };
        let dt = layer.data_type()?;
        return Ok(Some(InLayerInfo {
            scale,
            zero_point,
            signed: is_signed(dt),
            scalar_bytes: bytes_per_element(dt),
        }));
    }
    Ok(None)
}

fn bytes_per_element(dt: DataType) -> usize {
    match dt {
        DataType::FixedPoint8 | DataType::SignedFixedPoint8 => 1,
        DataType::FixedPoint16
        | DataType::SignedFixedPoint16
        | DataType::Half
        | DataType::Bfloat => 2,
        DataType::SignedFixedPoint32 | DataType::Single => 4,
    }
}

/// Whether the accelerator emits this type biased-unsigned, needing a sign flip.
/// Mirrors libedgetpu's `SignedDataType`, which excludes SignedFixedPoint32.
fn is_signed(dt: DataType) -> bool {
    matches!(
        dt,
        DataType::SignedFixedPoint8 | DataType::SignedFixedPoint16
    )
}

struct OutLayerInfo {
    scale: f32,
    zero_point: i32,
    layout: Option<OutputLayout>,
    element_bytes: usize,
    logical_bytes: usize,
    signed: bool,
    scalar_bytes: usize,
    variable: bool,
    dims: [usize; 3],
}

const VARIABLE_SUFFIX: &str = "_variable_output";

/// Output layers as (name, padded DMA size), used when the DMA hints carry no
/// outfeed (device-driven descriptor models) so the buffers can still be sized.
fn enumerate_output_layers(exec: ExecutableRef<'_>) -> Result<Vec<(String, usize)>> {
    let mut out = Vec::new();
    if let Some(v) = exec.output_layers()? {
        for layer in v.iter() {
            let layer = layer?;
            let name = layer.name()?.map(str::to_owned).unwrap_or_default();
            let bytes = usize::try_from(layer.size_bytes()?).unwrap_or(0);
            out.push((name, bytes));
        }
    }
    Ok(out)
}

/// Numerics, tiling layout, and element/logical sizes for the output layer named
/// `name`. The layout is present only for tensors tiled in both x and y.
fn output_layer_info(exec: ExecutableRef<'_>, name: &str) -> Result<Option<OutLayerInfo>> {
    let Some(v) = exec.output_layers()? else {
        return Ok(None);
    };
    for layer in v.iter() {
        let layer = layer?;
        if layer.name()?.map(str::to_owned).as_deref() != Some(name) {
            continue;
        }
        let (scale, zero_point) = match layer.numerics()? {
            Some(n) => (n.dequantization_factor()?, n.zero_point()?),
            None => (1.0, 0),
        };
        let dt = layer.data_type()?;
        let elem = bytes_per_element(dt);
        let x = usize::try_from(layer.x_dim()?).unwrap_or(1);
        let y = usize::try_from(layer.y_dim()?).unwrap_or(1);
        let z = usize::try_from(layer.z_dim()?).unwrap_or(1);
        let layout = match layer.any_layer()? {
            Some(AnyLayerRef::OutputLayer(ol)) => match ol.layout()? {
                Some(l) => Some(OutputLayout::try_from(l)?),
                None => None,
            },
            _ => None,
        };
        return Ok(Some(OutLayerInfo {
            scale,
            zero_point,
            layout,
            element_bytes: z * elem,
            logical_bytes: x * y * z * elem,
            signed: is_signed(dt),
            scalar_bytes: elem,
            variable: name.ends_with(VARIABLE_SUFFIX),
            dims: [y, x, z],
        }));
    }
    Ok(None)
}

struct HintIo {
    inputs: Vec<(String, usize)>,
    outputs: Vec<(String, usize)>,
    fully_deterministic: bool,
}

fn read_actions(exec: ExecutableRef<'_>) -> Result<(Vec<Action>, HintIo)> {
    let mut actions = Vec::new();
    let mut io = HintIo {
        inputs: Vec::new(),
        outputs: Vec::new(),
        fully_deterministic: true,
    };

    let Some(hints) = exec.dma_hints()? else {
        return Ok((actions, io));
    };
    io.fully_deterministic = hints.fully_deterministic()?;
    let Some(hs) = hints.hints()? else {
        return Ok((actions, io));
    };
    for h in hs.iter() {
        let h = h?;
        let infeed = matches!(h.direction()?, Direction::Infeed);
        match h.any_hint()? {
            Some(AnyHintRef::InstructionHint(i)) => actions.push(Action::Instruction(
                usize::try_from(i.instruction_chunk_index()?).unwrap_or(0),
            )),
            Some(AnyHintRef::DmaDescriptorHint(d)) => {
                let meta = d.meta()?;
                let desc = match meta {
                    Some(m) => Some(m.desc()?),
                    None => None,
                };
                let name = match meta {
                    Some(m) => m.name()?.map(str::to_owned).unwrap_or_default(),
                    None => String::new(),
                };
                let buffer = buffer_of(desc);
                let offset = usize::try_from(d.offset_in_bytes()?).unwrap_or(0);
                let size = usize::try_from(d.size_in_bytes()?).unwrap_or(0);
                match buffer {
                    // A model can have several inputs, each possibly written by
                    // multiple DMAs at different offsets; keep one entry per tensor
                    // sized to its furthest extent.
                    Buffer::Input => {
                        let extent = offset.saturating_add(size);
                        match io.inputs.iter_mut().find(|(n, _)| *n == name) {
                            Some((_, prev)) => *prev = (*prev).max(extent),
                            None => io.inputs.push((name.clone(), extent)),
                        }
                    }
                    // An output can also be written by several DMAs at different
                    // offsets; keep one entry per tensor sized to its furthest extent.
                    Buffer::Output => {
                        let extent = offset.saturating_add(size);
                        match io.outputs.iter_mut().find(|(n, _)| *n == name) {
                            Some((_, prev)) => *prev = (*prev).max(extent),
                            None => io.outputs.push((name.clone(), extent)),
                        }
                    }
                    _ => {}
                }
                actions.push(Action::Dma {
                    buffer,
                    name,
                    offset,
                    size,
                    infeed,
                });
            }
            Some(AnyHintRef::InterruptHint(_)) => actions.push(Action::Interrupt),
            Some(AnyHintRef::FenceHint(_)) => actions.push(Action::Fence),
            None => {}
        }
    }
    Ok((actions, io))
}

impl Program {
    /// Builds the ordered inference program (parameter-caching phase first).
    pub fn from_package(pkg_bytes: &[u8]) -> Result<Program> {
        let pkg = parse_package(pkg_bytes)?;
        let multi = multi_executable_bytes(&pkg)?;
        let blobs = executable_blobs(multi)?;

        let mut prog = Program::default();
        let mut param_phases = Vec::new();
        let mut exec_phases = Vec::new();

        for blob in &blobs {
            let exec = parse_executable(blob)?;
            let ty = exec.type_()?;
            let (chunks, chunk_sites) = read_chunks(exec)?;
            let (actions, io) = read_actions(exec)?;

            prog.scratch_bytes = prog
                .scratch_bytes
                .max(usize::try_from(exec.scratch_size_bytes()?).unwrap_or(0));

            for (name, bytes) in io.inputs {
                if prog.inputs.iter().any(|i| i.name == name) {
                    continue;
                }
                let info = input_layer_info(exec, &name)?;
                let (scale, zero_point, signed, scalar_bytes) = match info {
                    Some(i) => (i.scale, i.zero_point, i.signed, i.scalar_bytes),
                    None => (1.0, 0, false, 1),
                };
                prog.inputs.push(InputSpec {
                    name,
                    bytes,
                    scale,
                    zero_point,
                    signed,
                    scalar_bytes,
                });
            }
            // Device-driven models carry no outfeed hints; fall back to the
            // declared output layers so their buffers can be sized and linked.
            let outs = if io.outputs.is_empty() {
                enumerate_output_layers(exec)?
            } else {
                io.outputs
            };
            for (name, bytes) in outs {
                let info = output_layer_info(exec, &name)?;
                let (
                    scale,
                    zero_point,
                    layout,
                    element_bytes,
                    logical_bytes,
                    signed,
                    scalar_bytes,
                    variable,
                    dims,
                ) = match info {
                    Some(i) => (
                        i.scale,
                        i.zero_point,
                        i.layout,
                        i.element_bytes,
                        i.logical_bytes,
                        i.signed,
                        i.scalar_bytes,
                        i.variable,
                        i.dims,
                    ),
                    None => (1.0, 0, None, 1, bytes, false, 1, false, [0, 0, 0]),
                };
                prog.outputs.push(OutputSpec {
                    name,
                    bytes,
                    scale,
                    zero_point,
                    layout,
                    element_bytes,
                    logical_bytes,
                    signed,
                    scalar_bytes,
                    variable,
                    dims,
                });
            }
            let parameters = match exec.parameters()? {
                Some(p) => p.to_vec(),
                None => Vec::new(),
            };

            let caches_parameters = ty == ExecutableType::ParameterCaching;
            let param_fingerprint = if parameters.is_empty() {
                0
            } else {
                fnv1a(&parameters)
            };
            let phase = Phase {
                chunks,
                chunk_sites,
                actions,
                parameters,
                caches_parameters,
                parameter_caching_token: exec.parameter_caching_token()?,
                param_fingerprint,
                fully_deterministic: io.fully_deterministic,
            };
            if caches_parameters {
                param_phases.push(phase);
            } else {
                exec_phases.push(phase);
            }
        }

        prog.phases = param_phases;
        prog.phases.extend(exec_phases);
        if prog.phases.is_empty() {
            return Err(Error::MissingField("no executables"));
        }
        Ok(prog)
    }
}
