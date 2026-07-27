// Integration test against a real edgetpu-compiled model. Skipped unless
// CORAL_TEST_MODEL points at an Edge TPU `.tflite`; run it with e.g.
//   CORAL_TEST_MODEL=.../mobilenet_v2_1.0_224_quant_edgetpu.tflite \
//     cargo test -p cay_program --test real_model -- --nocapture

#[test]
fn parses_real_edgetpu_model() {
    let Ok(path) = std::env::var("CORAL_TEST_MODEL") else {
        return;
    };
    let bytes = std::fs::read(&path).expect("read model");

    let pkg_bytes = cay_program::extract_package(&bytes).expect("no DWN1 package in model");
    let pkg = cay_program::parse_package(pkg_bytes).expect("parse package");
    let multi = cay_program::multi_executable_bytes(&pkg).expect("multi-executable");
    let blobs = cay_program::executable_blobs(multi).expect("executable blobs");
    assert!(!blobs.is_empty(), "no executables");

    let mut instr_bytes = 0usize;
    let mut param_bytes = 0usize;
    let mut field_offsets = 0usize;
    let mut inputs = 0usize;
    let mut outputs = 0usize;

    for blob in &blobs {
        let exec = cay_program::parse_executable(blob).expect("parse executable");

        let mut ib_bytes = 0usize;
        let mut fos = 0usize;
        if let Some(instr) = exec.instruction_bitstreams().expect("instructions") {
            for ib in instr.iter() {
                let ib = ib.expect("instruction bitstream");
                ib_bytes += ib.bitstream().expect("bitstream").map_or(0, <[u8]>::len);
                fos += ib
                    .field_offsets()
                    .expect("field offsets")
                    .map_or(0, |v| v.len());
            }
        }
        let params = exec
            .parameters()
            .expect("parameters")
            .map_or(0, <[u8]>::len);
        let ins = exec.input_layers().expect("inputs").map_or(0, |v| v.len());
        let outs = exec
            .output_layers()
            .expect("outputs")
            .map_or(0, |v| v.len());

        eprintln!(
            "executable type={:?}: {ins} in, {outs} out, {params} params, {ib_bytes} instr bytes, {fos} field-offsets, scratch {}",
            exec.type_().expect("type"),
            exec.scratch_size_bytes().expect("scratch"),
        );

        instr_bytes += ib_bytes;
        param_bytes += params;
        field_offsets += fos;
        inputs += ins;
        outputs += outs;
    }

    eprintln!(
        "TOTAL: {} executables, {instr_bytes} instr bytes, {param_bytes} param bytes, {field_offsets} field-offsets, {inputs} inputs, {outputs} outputs",
        blobs.len()
    );

    assert!(instr_bytes > 0, "no instruction bytes");
    assert!(param_bytes > 0, "no parameters across executables");
    assert!(field_offsets > 0, "no field offsets to link");
    assert!(inputs > 0 && outputs > 0, "missing input/output layers");
}

#[test]
fn assembles_inference_streams_from_real_model() {
    let Ok(path) = std::env::var("CORAL_TEST_MODEL") else {
        return;
    };
    let bytes = std::fs::read(&path).expect("read model");
    let pkg = cay_program::extract_package(&bytes).expect("no DWN1 package");

    let streams = cay_program::inference_streams(pkg).expect("assemble streams");
    eprintln!(
        "streams: {} param bytes, {} exec-instruction bytes, input {} bytes, output {} bytes",
        streams.parameters.len(),
        streams.exec_instructions.len(),
        streams.input_bytes,
        streams.output_bytes,
    );

    // mobilenet_v2 224x224x3 uint8 in, 1001-class uint8 out, ~3.9 MB weights.
    assert_eq!(streams.parameters.len(), 3_949_120);
    assert!(!streams.exec_instructions.is_empty());
    assert_eq!(streams.input_bytes, 224 * 224 * 3);
    assert!(streams.output_bytes > 0);
}

#[test]
fn patches_real_model_field_offsets() {
    let Ok(path) = std::env::var("CORAL_TEST_MODEL") else {
        return;
    };
    let bytes = std::fs::read(&path).expect("read model");
    let pkg = cay_program::extract_package(&bytes).expect("no DWN1 package");
    let package = cay_program::parse_package(pkg).expect("parse package");
    let multi = cay_program::multi_executable_bytes(&package).expect("multi-executable");
    let blobs = cay_program::executable_blobs(multi).expect("executable blobs");

    // Every field offset the compiler emitted must be a valid, patchable bit
    // position in its instruction bitstream — the sites the driver links.
    let mut checked = 0usize;
    for blob in &blobs {
        let exec = cay_program::parse_executable(blob).expect("parse executable");
        let Some(ibs) = exec.instruction_bitstreams().expect("instructions") else {
            continue;
        };
        for ib in ibs.iter() {
            let ib = ib.expect("instruction bitstream");
            let sites = cay_program::field_sites(ib).expect("field sites");
            if sites.is_empty() {
                continue;
            }
            let mut buf = ib.bitstream().expect("bitstream").expect("bytes").to_vec();
            for site in &sites {
                cay_program::copy_u32(&mut buf, site.offset_bit, 0xdead_beef).expect("patch");
                assert_eq!(
                    cay_program::read_u32(&buf, site.offset_bit).expect("read back"),
                    0xdead_beef,
                    "offset_bit {}",
                    site.offset_bit
                );
                checked += 1;
            }
        }
    }
    assert!(checked > 0, "no field offsets patched");
    eprintln!("patched + verified {checked} real field offsets");
}

#[test]
fn parses_multi_output_model() {
    // Detection models (e.g. SSD, raw-head YOLO) expose more than one raw
    // edge-TPU output head. Point CORAL_MULTIOUT_MODEL at such a `.tflite`.
    let Ok(path) = std::env::var("CORAL_MULTIOUT_MODEL") else {
        return;
    };
    let bytes = std::fs::read(&path).expect("read model");
    let pkg = cay_program::extract_package(&bytes).expect("no DWN1 package");
    let program = cay_program::Program::from_package(pkg).expect("build program");

    eprintln!(
        "input {} bytes, {} outputs, scratch {}",
        program.inputs.iter().map(|i| i.bytes).sum::<usize>(),
        program.outputs.len(),
        program.scratch_bytes,
    );
    for o in &program.outputs {
        eprintln!(
            "  {:?}: raw {} bytes, logical {} bytes, element {} bytes, {}",
            o.name,
            o.bytes,
            o.logical_bytes,
            o.element_bytes,
            if o.layout.is_some() {
                "tiled"
            } else {
                "linear"
            },
        );
    }

    assert!(
        program.outputs.len() >= 2,
        "expected a multi-output model, got {}",
        program.outputs.len()
    );
    for o in &program.outputs {
        assert!(!o.name.is_empty(), "output has no name");
        assert!(
            o.logical_bytes > 0,
            "output {:?} has no logical size",
            o.name
        );
        // The de-tiled tensor must fit within the raw (padded/tiled) DMA buffer.
        assert!(
            o.logical_bytes <= o.bytes,
            "output {:?}: logical {} exceeds raw {}",
            o.name,
            o.logical_bytes,
            o.bytes
        );
        assert!(
            o.element_bytes > 0,
            "output {:?} has no element size",
            o.name
        );
    }
}

#[test]
fn inspects_real_model_dma_hints() {
    let Ok(path) = std::env::var("CORAL_TEST_MODEL") else {
        return;
    };
    let bytes = std::fs::read(&path).expect("read model");
    let pkg = cay_program::extract_package(&bytes).expect("no DWN1 package");
    let package = cay_program::parse_package(pkg).expect("parse package");
    let multi = cay_program::multi_executable_bytes(&package).expect("multi-executable");
    let blobs = cay_program::executable_blobs(multi).expect("executable blobs");

    for blob in &blobs {
        let exec = cay_program::parse_executable(blob).expect("parse executable");
        let ty = exec.type_().expect("type");
        match exec.dma_hints().expect("dma_hints") {
            None => eprintln!("{ty:?}: no dma_hints"),
            Some(h) => {
                let det = h.fully_deterministic().expect("deterministic");
                eprintln!("{ty:?}: fully_deterministic={det}");
                let Some(hints) = h.hints().expect("hints") else {
                    continue;
                };
                for (hi, hint) in hints.iter().enumerate() {
                    let hint = hint.expect("hint");
                    let dir = hint.direction().expect("direction");
                    match hint.any_hint().expect("any_hint") {
                        Some(cay_program::schema::AnyHintRef::DmaDescriptorHint(d)) => {
                            let meta = d.meta().expect("meta");
                            let desc = meta.map(|m| m.desc().expect("desc"));
                            let name = meta.and_then(|m| m.name().expect("name"));
                            eprintln!(
                                "  {hi}: {dir:?} DmaDescriptor desc={desc:?} name={name:?} off={} size={}",
                                d.offset_in_bytes().expect("off"),
                                d.size_in_bytes().expect("size"),
                            );
                        }
                        Some(cay_program::schema::AnyHintRef::InstructionHint(i)) => eprintln!(
                            "  {hi}: {dir:?} Instruction chunk={}",
                            i.instruction_chunk_index().expect("chunk")
                        ),
                        Some(cay_program::schema::AnyHintRef::InterruptHint(_)) => {
                            eprintln!("  {hi}: {dir:?} Interrupt")
                        }
                        Some(cay_program::schema::AnyHintRef::FenceHint(_)) => {
                            eprintln!("  {hi}: {dir:?} Fence")
                        }
                        None => eprintln!("  {hi}: {dir:?} (none)"),
                    }
                }
            }
        }
    }
}
