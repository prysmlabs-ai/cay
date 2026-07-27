//! Runs a multi-subgraph model as a pipeline of co-compiled edge-TPU segments on
//! one open device: each segment's declared inputs are gathered by name from a
//! tensor pool (seeded with the initial inputs), and its outputs feed the pool
//! for the segments that follow. The final segment's outputs are written out.
//!
//! Usage: coral-pipeline <out_prefix> <seg0.tflite> <seg1.tflite> ... <name=input.bin> ...
//! Segments run in the order given; boundary tensors are matched by name.

use std::collections::HashMap;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out_path = match args.first() {
        Some(o) => o.clone(),
        None => {
            eprintln!("usage: coral-pipeline <out_prefix> <seg.tflite> ... <name=input.bin> ...");
            std::process::exit(2);
        }
    };

    let mut segments = Vec::new();
    let mut pool: HashMap<String, Vec<u8>> = HashMap::new();
    for arg in &args[1..] {
        if arg.ends_with(".tflite") {
            let model = fs::read(arg).expect("read model");
            let pkg = cay_program::extract_package(&model).expect("no DWN1 package in model");
            segments.push(cay_program::Program::from_package(pkg).expect("build program"));
        } else if let Some((name, path)) = arg.split_once('=') {
            pool.insert(name.to_string(), fs::read(path).expect("read input"));
        }
    }
    if segments.is_empty() {
        eprintln!("no segment models given");
        std::process::exit(2);
    }

    let driver = cay::driver::Driver::open().expect("chip bring-up");
    driver.run().expect("run");

    let mut last: Vec<(String, Vec<u8>)> = Vec::new();
    for (s, prog) in segments.iter().enumerate() {
        let inputs_data: Vec<Vec<u8>> = prog
            .inputs
            .iter()
            .map(|spec| pool.get(&spec.name).cloned().unwrap_or_default())
            .collect();
        let input_refs: Vec<&[u8]> = inputs_data.iter().map(Vec::as_slice).collect();
        let outs = driver
            .run_program(prog, &input_refs)
            .expect("segment inference");
        eprintln!(
            "segment {s}: {} in -> {} out (params 0x{:x})",
            prog.inputs.len(),
            prog.outputs.len(),
            driver.resident_param(),
        );
        last.clear();
        for (spec, out) in prog.outputs.iter().zip(&outs) {
            pool.insert(spec.name.clone(), out.clone());
            last.push((spec.name.clone(), out.clone()));
        }
    }

    for (i, (name, out)) in last.iter().enumerate() {
        eprintln!("output[{i}] name={name} {} bytes", out.len());
        let path = if last.len() == 1 {
            out_path.clone()
        } else {
            format!("{out_path}.{i}.bin")
        };
        fs::write(&path, out).expect("write output");
    }
}
