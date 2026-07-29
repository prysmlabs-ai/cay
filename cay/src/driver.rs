//! Chip bring-up for the Coral runtime device. Port of libedgetpu's UsbDriver
//! open path (the Beagle top-level handler reset / quit-reset, InitializeChip)
//! and RunController: CSR reads/writes over the ML control endpoint that take the
//! accelerator out of reset and to the run state. SCU control registers are
//! read-modify-written by bit field per beagle_csr_helper.h.

use std::time::Duration;

use crate::bulk_in::BulkInPool;
use crate::csr;
use crate::dma::{DmaChunker, Processing};
use crate::error::{Error, Result};
use crate::ml::{Csr, DescriptorTag, EventDescriptor};

use crate::program::{Action, Buffer, Program};

const RUN: u64 = 1;
const HALT: u64 = 2;

// Largest single bulk-out transfer (libedgetpu's default).
const MAX_BULK_OUT: usize = 1024 * 1024;

// Upper bound on data descriptors drained before the terminating interrupt event.
const MAX_EVENTS: usize = 256;

// Bulk-in reads kept posted so the outfeed always has a buffer (libedgetpu's
// usb_bulk_in_queue_capacity default).
const BULK_IN_QUEUE_CAPACITY: usize = 32;

// Ceiling on how long to wait for one output tensor to stream back. Kept well
// under the ten seconds after which a host like Frigate declares a detector
// stuck and SIGKILLs it: giving up first turns a stalled inference into one
// failed call, where being killed mid-transfer takes the device off the bus
// until someone replugs it.
const OUTPUT_TIMEOUT: Duration = Duration::from_secs(4);

// Upper bound on device descriptors processed in one descriptor-mode inference.
const MAX_DESCRIPTORS: usize = 8192;

const POLL_ATTEMPTS: u32 = 2000;
const POLL_INTERVAL: Duration = Duration::from_millis(1);

fn get_bits(raw: u32, start: u32, width: u32) -> u32 {
    (raw >> start) & ((1 << width) - 1)
}

fn set_bits(raw: u32, start: u32, width: u32, value: u32) -> u32 {
    let mask = ((1u32 << width) - 1) << start;
    (raw & !mask) | ((value << start) & mask)
}

pub struct Driver {
    csr: Csr,
    hardware_clock_gated: bool,
    /// Bulk-in read size: 256 B on USB2 High Speed (matches the shortened
    /// outfeed chunk), 1 KB otherwise.
    bulk_in_chunk: usize,
    /// Fingerprint of the weights currently resident on-chip, so a repeat of the
    /// same model skips re-streaming them (0 = none resident). Keyed on content,
    /// not the caching token, since co-compiled segments share one token.
    /// Cleared whenever the device is reset (driver.cc current_parameter_caching_token_).
    resident_param: std::cell::Cell<u64>,
}

impl Drop for Driver {
    fn drop(&mut self) {
        self.close();
    }
}

/// Tunables that change how the driver talks to the device.
#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    /// Read bulk-in in 1 KB chunks even on a USB2 High Speed link.
    ///
    /// The 256-byte cap is a host-controller workaround (libedgetpu's
    /// `usb_force_largest_bulk_in_chunk_size`, b/73181174): it lets a controller
    /// that dislikes short packets know every response is exactly 256 bytes.
    /// Controllers that do not need it pay four times the transfer count for an
    /// outfeed. Measure before setting this — a controller that needs the
    /// workaround corrupts large outputs without it.
    pub force_largest_bulk_in_chunk: bool,
}

impl Driver {
    /// Opens the runtime device and brings the chip out of reset, up to and
    /// including InitializeChip. Use `run` to move the scalar core to running.
    pub fn open() -> Result<Self> {
        Self::open_with(Options::default())
    }

    pub fn open_with(options: Options) -> Result<Self> {
        let csr = Csr::open()?;
        // CSR access is device-recipient control transfers and needs no claimed
        // interface; the bulk endpoints do, so claim best-effort here.
        let _ = csr.claim(0);
        // A process that died mid-transfer leaves endpoints stalled, and the
        // stall survives it: every read the next process posts fails. Clearing
        // on the way in is what lets a fresh open recover the device.
        csr.clear_halts();
        let bulk_in_chunk = if csr.is_high_speed() && !options.force_largest_bulk_in_chunk {
            256
        } else {
            1024
        };
        let mut d = Self {
            csr,
            hardware_clock_gated: false,
            bulk_in_chunk,
            resident_param: std::cell::Cell::new(0),
        };
        d.top_level_open()?;
        d.disable_hardware_clock_gate()?;
        d.enable_reset()?;
        d.quit_reset()?;
        d.enable_hardware_clock_gate()?;
        d.initialize_chip()?;
        d.enable_interrupts()?;
        Ok(d)
    }

    fn poll32(&self, offset: u32, pred: impl Fn(u32) -> bool) -> Result<()> {
        for _ in 0..POLL_ATTEMPTS {
            if pred(self.csr.read32(offset)?) {
                return Ok(());
            }
            std::thread::sleep(POLL_INTERVAL);
        }
        Err(Error::Timeout(offset))
    }

    fn poll64(&self, offset: u32, pred: impl Fn(u64) -> bool) -> Result<()> {
        for _ in 0..POLL_ATTEMPTS {
            if pred(self.csr.read64(offset)?) {
                return Ok(());
            }
            std::thread::sleep(POLL_INTERVAL);
        }
        Err(Error::Timeout(offset))
    }

    fn top_level_open(&mut self) -> Result<()> {
        let mut v = self.csr.read32(csr::SCU_CTRL_0)?;
        v = set_bits(v, 8, 3, 0); // rg_pcie_inact_phy_mode
        v = set_bits(v, 11, 3, 0); // rg_usb_inact_phy_mode
        self.csr.write32(csr::SCU_CTRL_0, v)?;

        let gate = self.csr.read32(csr::SCU_CTRL_2)?;
        self.hardware_clock_gated = get_bits(gate, 18, 2) == 1;
        Ok(())
    }

    fn disable_hardware_clock_gate(&mut self) -> Result<()> {
        if !self.hardware_clock_gated {
            return Ok(());
        }
        let v = self.csr.read32(csr::SCU_CTRL_2)?;
        self.csr.write32(csr::SCU_CTRL_2, set_bits(v, 18, 2, 0x2))?; // force clock on
        self.hardware_clock_gated = false;
        Ok(())
    }

    fn enable_hardware_clock_gate(&mut self) -> Result<()> {
        if self.hardware_clock_gated {
            return Ok(());
        }
        let v = self.csr.read32(csr::SCU_CTRL_2)?;
        self.csr.write32(csr::SCU_CTRL_2, set_bits(v, 18, 2, 0x1))?; // enable GCB gating
        self.hardware_clock_gated = true;
        Ok(())
    }

    fn enable_reset(&self) -> Result<()> {
        let v = self.csr.read32(csr::SCU_CTRL_3)?;
        if get_bits(v, 22, 2) == 0x3 {
            return Ok(());
        }
        self.csr.write32(csr::SCU_CTRL_3, set_bits(v, 22, 2, 0x3))?;
        self.poll32(csr::SCU_CTRL_3, |x| get_bits(x, 8, 2) == 0x2)?;
        self.csr.write32(csr::GCBB_CREDIT0, 0xF)?;
        self.csr.write32(csr::GCBB_CREDIT0, 0x0)?;
        Ok(())
    }

    fn quit_reset(&self) -> Result<()> {
        let mut v = self.csr.read32(csr::SCU_CTRL_3)?;
        v = set_bits(v, 22, 2, 0b10); // rg_force_sleep
        v = set_bits(v, 28, 2, 0); // rg_gcb_clkdiv = 500 MHz
        v = set_bits(v, 30, 1, 0); // rg_axi_clk_125m = 250 MHz
        v = set_bits(v, 31, 1, 0); // rg_8051_clk_250m = 500 MHz
        self.csr.write32(csr::SCU_CTRL_3, v)?;
        self.poll32(csr::SCU_CTRL_3, |x| get_bits(x, 8, 2) == 0x0)?;
        self.poll64(csr::SCALAR_CORE_RUN_CONTROL, |x| x == 0)?;
        self.csr.write64(csr::IDLE_REGISTER, 0x1)?;
        self.csr.write64(csr::TILECONFIG0, 0x7F)?;
        self.poll64(csr::TILECONFIG0, |x| x == 0x7F)?;
        self.csr.write64(csr::DEEP_SLEEP, 0x1E02)?;
        Ok(())
    }

    fn initialize_chip(&self) -> Result<()> {
        let _ = self.csr.read32(csr::OMC0_00)?; // efuse revision, informational
        self.csr.write64(csr::DESCR_EP, 0xF0)?; // single-ep descriptor set
        self.csr.write64(csr::MULTI_BO_EP, 0)?; // single bulk-out endpoint
                                                // Outfeed chunk must match the bulk-in read size the host posts.
        let outfeed = if self.bulk_in_chunk <= 256 {
            0x20
        } else {
            0x80
        };
        self.csr.write64(csr::OUTFEED_CHUNK_LENGTH, outfeed)?;
        Ok(())
    }

    /// Moves the scalar core and tiles to the run state.
    pub fn run(&self) -> Result<()> {
        self.run_control(RUN)
    }

    /// Halts the scalar core and tiles.
    pub fn halt(&self) -> Result<()> {
        self.run_control(HALT)
    }

    fn run_control(&self, state: u64) -> Result<()> {
        for &reg in &csr::SCALAR_CORE_RUN_CONTROLS {
            self.csr.write64(reg, state)?;
        }
        self.csr.write64(csr::TILECONFIG0, 0x7F)?;
        self.poll64(csr::TILECONFIG0, |x| x == 0x7F)?;
        for &reg in &csr::TILE_RUN_CONTROLS {
            self.csr.write64(reg, state)?;
        }
        Ok(())
    }

    /// Enables the fatal-error and top-level interrupt controllers, then the
    /// thermal, MBIST, and bus-error sources — the full DoEnableInterrupts
    /// sequence BeagleTopLevelInterruptManager runs at bring-up. Read-modify-write
    /// preserves the surrounding bit fields.
    pub fn enable_interrupts(&self) -> Result<()> {
        self.csr.write64(csr::FATAL_ERR_INT_CONTROL, 1)?;
        for &reg in &csr::TOP_LEVEL_INT_CONTROL {
            self.csr.write64(reg, 1)?;
        }
        let d4 = self.csr.read32(csr::OMC0_D4)?;
        self.csr.write32(csr::OMC0_D4, d4 | 0x8000_0000)?; // thm_warn_en
        let bist = self.csr.read32(csr::RAMBIST_CTRL_1)?;
        self.csr.write32(csr::RAMBIST_CTRL_1, bist & !0x0070_0000)?; // mbist int
        let scu7 = self.csr.read32(csr::SCU_CTR_7)?;
        self.csr.write32(csr::SCU_CTR_7, scu7 & !0x000c_0000)?; // failure masks
        self.csr.write32(csr::SLV_ABM_EN, 1)?;
        self.csr.write32(csr::MST_ABM_EN, 1)?;
        self.csr.write32(csr::SLV_ERR_RESP_ISR_MASK, 3)?;
        self.csr.write32(csr::MST_ERR_RESP_ISR_MASK, 3)?;
        let d8 = self.csr.read32(csr::OMC0_D8)?;
        self.csr.write32(csr::OMC0_D8, d8 | 0x8000_0000)?; // sd_en
        Ok(())
    }

    /// The scalar-core run-control register: 0 out of reset/idle, 1 running.
    pub fn scalar_core_state(&self) -> Result<u64> {
        self.csr.read64(csr::SCALAR_CORE_RUN_CONTROL)
    }

    /// Streams a tagged buffer to the single bulk-out endpoint: the 8-byte header
    /// then the payload, chunked to the max bulk-out transfer size.
    pub fn submit_stream(&self, tag: DescriptorTag, data: &[u8]) -> Result<()> {
        self.csr.write_header(tag, data.len() as u32)?;
        let mut chunker = DmaChunker::new(Processing::Committed, data.len());
        while chunker.has_next_chunk() {
            let chunk = chunker.next_chunk_upto(MAX_BULK_OUT);
            let sent = self
                .csr
                .bulk_out(&data[chunk.offset..chunk.offset + chunk.len])?;
            chunker.notify_transfer(sent)?;
        }
        Ok(())
    }

    /// Reads one event descriptor from the event endpoint.
    pub fn read_event(&self) -> Result<EventDescriptor> {
        self.csr.read_event()
    }

    /// Runs a hint-driven inference program; returns one buffer per output tensor
    /// in `prog.outputs` order. On failure recovers the device so a timeout is not
    /// terminal.
    /// Runs one inference. `inputs[i]` supplies data for `prog.inputs[i]` (index 0
    /// is the primary activation). Returns one buffer per tensor in `prog.outputs`.
    /// On failure recovers the device so a timeout is not terminal.
    pub fn run_program(&self, prog: &Program, inputs: &[&[u8]]) -> Result<Vec<Vec<u8>>> {
        let result = if prog.phases.iter().all(|p| p.fully_deterministic) {
            self.run_hints(prog, inputs)
        } else {
            self.run_descriptors(prog, inputs)
        };
        if result.is_err() {
            // A port reset drops the SRAM-resident firmware, leaving the device
            // in DFU or off the bus, so it waits until the chip stops answering
            // control transfers and nothing else can reach it.
            self.csr.clear_halts();
            if self.halt().is_err() {
                let _ = self.csr.reset();
            }
            self.resident_param.set(0);
        }
        result
    }

    /// Leaves the chip idle for whoever opens it next, so a process that exits
    /// between inferences does not hand the next `open` a running core.
    pub fn close(&self) {
        let _ = self.halt();
        self.csr.clear_halts();
    }

    /// Fingerprint of the weights currently resident on-chip (0 = none). A repeat
    /// inference of the same model skips re-streaming them.
    pub fn resident_param(&self) -> u64 {
        self.resident_param.get()
    }

    /// Streams each linked instruction chunk to the device.
    fn send_chunks(&self, chunks: &[Vec<u8>]) -> Result<()> {
        for chunk in chunks {
            self.submit_stream(DescriptorTag::Instructions, chunk)?;
        }
        Ok(())
    }

    /// De-tiles and sign-corrects each raw output buffer into its row-major tensor.
    fn finalize_outputs(&self, prog: &Program, outputs: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>> {
        let mut logical = Vec::with_capacity(prog.outputs.len());
        for (spec, raw) in prog.outputs.iter().zip(outputs) {
            let mut tensor = match &spec.layout {
                Some(layout) => crate::program::relayout(layout, &raw, spec.element_bytes)?,
                None => {
                    let n = spec.logical_bytes.min(raw.len());
                    raw[..n].to_vec()
                }
            };
            if spec.signed {
                crate::program::transform_signed(&mut tensor, spec.scalar_bytes);
            }
            logical.push(tensor);
        }
        Ok(logical)
    }

    /// Static-hint execution: the DMA order is fixed at compile time. Links the
    /// buffer host-pointers into each phase's instruction chunks (the
    /// NopAddressSpace identity mapping libedgetpu uses over USB), then follows the
    /// DMA hints — infeed out, each output tensor in.
    fn run_hints(&self, prog: &Program, inputs: &[&[u8]]) -> Result<Vec<Vec<u8>>> {
        // One padded buffer per declared input (a multi-subgraph segment takes
        // several activations). Pad each up to its DMA size; the accelerator reads
        // the full extent but the compute ignores the trailing bytes (libedgetpu
        // ScatterInput). The activation (index 0) arrives in the model's
        // representation, which libedgetpu forwards unmodified — verified on the
        // wire: MoveNet's signed activation is sent raw, not sign-flipped. Only
        // signed weight-constant inputs are flipped.
        let input_bufs: Vec<Vec<u8>> = prog
            .inputs
            .iter()
            .enumerate()
            .map(|(i, spec)| {
                let mut v = inputs.get(i).copied().unwrap_or(&[]).to_vec();
                if i > 0 && spec.signed {
                    crate::program::transform_signed(&mut v, spec.scalar_bytes);
                }
                if v.len() < spec.bytes {
                    v.resize(spec.bytes, 0);
                }
                v
            })
            .collect();
        let mut scratch = vec![0u8; prog.scratch_bytes.max(1)];
        let mut outputs: Vec<Vec<u8>> = prog
            .outputs
            .iter()
            .map(|o| vec![0u8; o.bytes.max(1)])
            .collect();

        let input_addrs: Vec<u64> = input_bufs.iter().map(|b| b.as_ptr() as u64).collect();
        let scratch_addr = scratch.as_ptr() as u64;
        let output_addrs: Vec<u64> = outputs.iter().map(|b| b.as_ptr() as u64).collect();

        // Pre-post the read pool before any bulk-out so the outfeed can drain as
        // the accelerator computes (see bulk_in).
        // Safe: the handle and context live in `self.csr` for the whole call.
        let mut pool = unsafe {
            BulkInPool::new(
                self.csr.raw_handle(),
                self.csr.raw_context(),
                crate::ml::BULK_IN,
                BULK_IN_QUEUE_CAPACITY,
                self.bulk_in_chunk,
            )?
        };

        for phase in &prog.phases {
            // If this cache-load phase's exact weights are already resident
            // on-chip from a prior inference, skip re-streaming them — keep them
            // resident across calls. Match on the content fingerprint, not the
            // token, which co-compiled segments share.
            if phase.caches_parameters
                && phase.parameter_caching_token != 0
                && phase.param_fingerprint != 0
                && phase.param_fingerprint == self.resident_param.get()
            {
                continue;
            }
            let phase_param_addr = phase.parameters.as_ptr() as u64;
            let mut chunks = phase.chunks.clone();
            for (chunk, sites) in chunks.iter_mut().zip(&phase.chunk_sites) {
                if prog.scratch_bytes > 0 {
                    crate::program::link_scratch(chunk, sites, scratch_addr)?;
                }
                // Link the parameter base to this phase's own weights. A phase
                // without its own weights (execution reading the on-chip cache)
                // leaves the field at base 0.
                if !phase.parameters.is_empty() {
                    crate::program::link_parameter(chunk, sites, phase_param_addr)?;
                }
                for (spec, &addr) in prog.inputs.iter().zip(&input_addrs) {
                    crate::program::link_input(chunk, sites, &spec.name, &[addr])?;
                }
                for (spec, &addr) in prog.outputs.iter().zip(&output_addrs) {
                    crate::program::link_output(chunk, sites, &spec.name, &[addr])?;
                }
            }

            // One transfer at a time, in hint order (libedgetpu's single-endpoint
            // rule): a bulk-out issued while a bulk-in is outstanding deadlocks the
            // device's data path.
            for action in &phase.actions {
                match action {
                    Action::Instruction(i) => {
                        if let Some(chunk) = chunks.get(*i) {
                            self.submit_stream(DescriptorTag::Instructions, chunk)?;
                        }
                    }
                    Action::Dma {
                        buffer,
                        name,
                        offset,
                        size,
                        infeed,
                    } => {
                        if *infeed {
                            // Scratch round-trips over USB: it is fed back to the
                            // device tagged as an input activation. A DMA feeding an
                            // input selects the buffer whose name it names.
                            let selected: Option<(DescriptorTag, &[u8])> = match buffer {
                                Buffer::Parameter => {
                                    Some((DescriptorTag::Parameters, &phase.parameters))
                                }
                                Buffer::Input => {
                                    prog.inputs.iter().position(|s| s.name == *name).map(|i| {
                                        (DescriptorTag::InputActivations, input_bufs[i].as_slice())
                                    })
                                }
                                Buffer::Scratch => {
                                    Some((DescriptorTag::InputActivations, scratch.as_slice()))
                                }
                                _ => None,
                            };
                            if let Some((tag, data)) = selected {
                                let end = offset.saturating_add(*size).min(data.len());
                                self.submit_stream(tag, &data[*offset..end])?;
                            }
                        } else if matches!(buffer, Buffer::Output) {
                            if let Some(idx) = prog.outputs.iter().position(|o| o.name == *name) {
                                let end = offset.saturating_add(*size).min(outputs[idx].len());
                                let got = pool.collect(end - *offset, OUTPUT_TIMEOUT)?;
                                outputs[idx][*offset..end].copy_from_slice(&got);
                            }
                        } else if matches!(buffer, Buffer::Scratch) {
                            let end = offset.saturating_add(*size).min(scratch.len());
                            let got = pool.collect(end - *offset, OUTPUT_TIMEOUT)?;
                            scratch[*offset..end].copy_from_slice(&got);
                        }
                    }
                    Action::Interrupt => {
                        for _ in 0..MAX_EVENTS {
                            if self.read_event()?.tag >= 4 {
                                break;
                            }
                        }
                    }
                    Action::Fence => {}
                }
            }
            // Weights streamed to the chip this phase are now resident.
            if phase.caches_parameters && phase.parameter_caching_token != 0 {
                self.resident_param.set(phase.param_fingerprint);
            }
        }

        // Output fully drained; cancel the remaining posted reads.
        drop(pool);

        // libedgetpu checks this after each run; a non-zero status means the
        // accelerator faulted and the output activations are not trustworthy.
        let hib = self.csr.read64(csr::HIB_ERROR_STATUS)?;
        if hib != 0 {
            let first = self.csr.read64(csr::HIB_FIRST_ERROR_STATUS).unwrap_or(0);
            return Err(Error::HardwareError { status: hib, first });
        }

        self.finalize_outputs(prog, outputs)
    }

    /// Device-driven execution for models whose DMA order isn't fixed at compile
    /// time. Enables descriptors on every endpoint; the accelerator then emits a
    /// descriptor `(device address, length, tag)` for each DMA it wants, which the
    /// host resolves to a buffer by address and either sends (infeed) or reads
    /// (outfeed). Instructions are streamed proactively at each phase boundary —
    /// the device never generates descriptors for them.
    fn run_descriptors(&self, prog: &Program, inputs: &[&[u8]]) -> Result<Vec<Vec<u8>>> {
        self.csr.write64(csr::DESCR_EP, 0xFF)?;

        // Inputs padded to their declared DMA size; scratch and outputs are
        // written by the device (scratch also round-trips back in).
        let input_bufs: Vec<Vec<u8>> = prog
            .inputs
            .iter()
            .enumerate()
            .map(|(i, spec)| {
                let mut v = inputs.get(i).copied().unwrap_or(&[]).to_vec();
                // Signed weight-constant inputs (a recurrent model's state) are
                // held as signed storage; flip each scalar's sign bit to the
                // device's biased-unsigned form (libedgetpu TransformSignedDataType).
                // The activation (index 0) is already device-ready — leave it.
                if i > 0 && spec.signed {
                    crate::program::transform_signed(&mut v, spec.scalar_bytes);
                }
                if v.len() < spec.bytes {
                    v.resize(spec.bytes, 0);
                }
                v
            })
            .collect();
        let mut scratch = vec![0u8; prog.scratch_bytes.max(1)];
        let mut output_bufs: Vec<Vec<u8>> = prog
            .outputs
            .iter()
            .map(|o| vec![0u8; o.bytes.max(1)])
            .collect();

        // Address → buffer registry for descriptor matching.
        #[derive(Clone, Copy)]
        enum Kind {
            Param(usize),
            Input(usize),
            Scratch,
            Output(usize),
        }
        let mut regs: Vec<(u64, u64, Kind)> = Vec::new();
        for (pi, ph) in prog.phases.iter().enumerate() {
            if !ph.parameters.is_empty() {
                regs.push((
                    ph.parameters.as_ptr() as u64,
                    ph.parameters.len() as u64,
                    Kind::Param(pi),
                ));
            }
        }
        for (i, b) in input_bufs.iter().enumerate() {
            regs.push((b.as_ptr() as u64, b.len() as u64, Kind::Input(i)));
        }
        let scratch_addr = scratch.as_ptr() as u64;
        regs.push((scratch_addr, scratch.len() as u64, Kind::Scratch));
        for (i, b) in output_bufs.iter().enumerate() {
            regs.push((b.as_ptr() as u64, b.len() as u64, Kind::Output(i)));
        }

        // Link every buffer address into every chunk (each chunk patches only the
        // field sites it carries), then keep the linked chunks per phase.
        let input_addrs: Vec<(String, u64)> = prog
            .inputs
            .iter()
            .zip(&input_bufs)
            .map(|(spec, b)| (spec.name.clone(), b.as_ptr() as u64))
            .collect();
        let output_addrs: Vec<u64> = output_bufs.iter().map(|b| b.as_ptr() as u64).collect();
        let mut phase_chunks: Vec<Vec<Vec<u8>>> = Vec::with_capacity(prog.phases.len());
        for ph in &prog.phases {
            let mut chunks = ph.chunks.clone();
            let pa = ph.parameters.as_ptr() as u64;
            for (chunk, sites) in chunks.iter_mut().zip(&ph.chunk_sites) {
                if prog.scratch_bytes > 0 {
                    crate::program::link_scratch(chunk, sites, scratch_addr)?;
                }
                if !ph.parameters.is_empty() {
                    crate::program::link_parameter(chunk, sites, pa)?;
                }
                for (name, addr) in &input_addrs {
                    crate::program::link_input(chunk, sites, name, &[*addr])?;
                }
                for (spec, &addr) in prog.outputs.iter().zip(&output_addrs) {
                    crate::program::link_output(chunk, sites, &spec.name, &[addr])?;
                }
            }
            phase_chunks.push(chunks);
        }

        let mut pool = unsafe {
            BulkInPool::new(
                self.csr.raw_handle(),
                self.csr.raw_context(),
                crate::ml::BULK_IN,
                BULK_IN_QUEUE_CAPACITY,
                self.bulk_in_chunk,
            )?
        };

        // Send the first phase, then follow the device's descriptors. A fence
        // (tag >= 4) ends a phase: send the next phase's instructions or stop.
        let mut phase_idx = 0;
        self.send_chunks(&phase_chunks[0])?;
        let out_tag = DescriptorTag::OutputActivations as u8;
        let param_tag = DescriptorTag::Parameters as u8;
        for _ in 0..MAX_DESCRIPTORS {
            let desc = self.read_event()?;
            if desc.tag >= 4 {
                phase_idx += 1;
                if phase_idx < phase_chunks.len() {
                    self.send_chunks(&phase_chunks[phase_idx])?;
                    continue;
                }
                break;
            }
            let Some((base, _, kind)) = regs
                .iter()
                .find(|(b, s, _)| desc.offset >= *b && desc.offset < b + s)
                .copied()
            else {
                continue;
            };
            let off = (desc.offset - base) as usize;
            let len = desc.length as usize;
            if desc.tag == out_tag {
                let got = pool.collect(len, OUTPUT_TIMEOUT)?;
                let dst = match kind {
                    Kind::Scratch => &mut scratch[off..off + len],
                    Kind::Output(oi) => &mut output_bufs[oi][off..off + len],
                    _ => continue,
                };
                dst.copy_from_slice(&got);
            } else {
                let tag = if desc.tag == param_tag {
                    DescriptorTag::Parameters
                } else {
                    DescriptorTag::InputActivations
                };
                let data: &[u8] = match kind {
                    Kind::Param(pi) => &prog.phases[pi].parameters,
                    Kind::Input(ii) => &input_bufs[ii],
                    Kind::Scratch => &scratch,
                    Kind::Output(_) => continue,
                };
                let end = (off + len).min(data.len());
                self.submit_stream(tag, &data[off..end])?;
            }
        }

        drop(pool);
        let _ = self.csr.write64(csr::DESCR_EP, 0xF0);

        let hib = self.csr.read64(csr::HIB_ERROR_STATUS)?;
        if hib != 0 {
            let first = self.csr.read64(csr::HIB_FIRST_ERROR_STATUS).unwrap_or(0);
            return Err(Error::HardwareError { status: hib, first });
        }
        self.finalize_outputs(prog, output_bufs)
    }
}

#[cfg(test)]
mod tests {
    use super::{get_bits, set_bits};

    #[test]
    fn bitfield_roundtrip_preserves_neighbors() {
        // scu_ctrl_3 reset value; set rg_force_sleep (bits[22,2]) to 0b10.
        let v = set_bits(0x8005_0410, 22, 2, 0b10);
        assert_eq!(get_bits(v, 22, 2), 0b10);
        assert_eq!(get_bits(v, 8, 2), get_bits(0x8005_0410, 8, 2));
        assert_eq!(get_bits(v, 31, 1), 1); // top bit of the reset value untouched
    }

    #[test]
    fn set_bits_clears_only_its_field() {
        let v = set_bits(0xffff_ffff, 8, 3, 0);
        assert_eq!(get_bits(v, 8, 3), 0);
        assert_eq!(get_bits(v, 11, 3), 0b111);
        assert_eq!(get_bits(v, 5, 3), 0b111);
    }
}
