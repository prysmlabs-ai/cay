use std::collections::HashMap;
use std::fs;

fn main() {
    let mut args = std::env::args().skip(1);
    let (model_path, out_path) = match (args.next(), args.next()) {
        (Some(m), Some(o)) => (m, o),
        _ => {
            eprintln!("usage: coral-infer <model.tflite> <output.bin> <name=input.bin> ...");
            std::process::exit(2);
        }
    };

    let model = fs::read(&model_path).expect("read model");
    // A model split around a host op the Edge TPU can't run carries one package
    // per edge-TPU subgraph. Run each, feeding its inputs by name from what the
    // caller supplied — the host ops between them are already reflected in those
    // input tensors (this binary runs the accelerator parts, not the host ops).
    let packages = cay_program::extract_packages(&model);
    if packages.is_empty() {
        eprintln!("no DWN1 package in model");
        std::process::exit(1);
    }
    let programs: Vec<cay_program::Program> = packages
        .iter()
        .map(|p| cay_program::Program::from_package(p).expect("build program"))
        .collect();

    let mut provided = HashMap::new();
    for arg in args {
        if let Some((name, path)) = arg.split_once('=') {
            provided.insert(name.to_string(), fs::read(path).expect("read input"));
        }
    }
    eprintln!(
        "program: {} subgraph(s), {} inputs / {} outputs",
        programs.len(),
        programs.iter().map(|p| p.inputs.len()).sum::<usize>(),
        programs.iter().map(|p| p.outputs.len()).sum::<usize>(),
    );

    let driver = cay::driver::Driver::open().expect("chip bring-up");
    driver.run().expect("run");

    // CORAL_ITERS>1 re-runs the whole model on the one open device: the second
    // pass onward must skip re-streaming cached weights (resident fingerprint
    // unchanged) yet stay bit-exact — the parameter-cache-reuse check.
    let iters: usize = std::env::var("CORAL_ITERS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let mut named: Vec<(String, Vec<u8>)> = Vec::new();
    for k in 0..iters.max(1) {
        named.clear();
        let before = driver.resident_param();
        for program in &programs {
            let inputs_data: Vec<Vec<u8>> = program
                .inputs
                .iter()
                .map(|spec| provided.get(&spec.name).cloned().unwrap_or_default())
                .collect();
            let input_refs: Vec<&[u8]> = inputs_data.iter().map(Vec::as_slice).collect();
            match driver.run_program(program, &input_refs) {
                Ok(outs) => {
                    for (spec, out) in program.outputs.iter().zip(outs) {
                        named.push((spec.name.clone(), out));
                    }
                }
                Err(e) => {
                    eprintln!("infer failed: {e}");
                    std::process::exit(1);
                }
            }
        }
        if iters > 1 {
            let now = driver.resident_param();
            eprintln!(
                "iter {k}: resident params 0x{now:x}{}",
                if k > 0 && before != 0 && before == now {
                    " (weights reused)"
                } else {
                    ""
                }
            );
        }
    }

    for (i, (name, out)) in named.iter().enumerate() {
        let argmax = out
            .iter()
            .enumerate()
            .max_by_key(|(_, v)| **v)
            .map(|(i, _)| i)
            .unwrap_or(0);
        eprintln!(
            "output[{i}] name={name} {} bytes, argmax {argmax}",
            out.len()
        );
        let path = if named.len() == 1 {
            out_path.clone()
        } else {
            format!("{out_path}.{i}.bin")
        };
        fs::write(&path, out).expect("write output");
    }
}
