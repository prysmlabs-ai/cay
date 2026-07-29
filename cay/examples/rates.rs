//! Effective bulk-out rate for a large transfer against the steady-state rate.
use std::time::Instant;

use cay::driver::Driver;
use cay::program::{extract_package, Program};

fn main() {
    let path = std::env::args().nth(1).expect("model");
    let model = std::fs::read(&path).expect("read");
    let program = Program::from_package(extract_package(&model).expect("pkg")).expect("parse");

    let params: usize = program.phases.iter().map(|p| p.parameters.len()).sum();
    let instructions: usize = program
        .phases
        .iter()
        .flat_map(|p| p.chunks.iter())
        .map(|c| c.len())
        .sum();
    let input: usize = program.inputs.iter().map(|i| i.bytes).sum();
    let output: usize = program.outputs.iter().map(|o| o.bytes).sum();

    let mut driver = Driver::open().expect("open");
    driver.run().expect("run");
    let inputs: Vec<Vec<u8>> = program
        .inputs
        .iter()
        .map(|i| vec![128u8; i.bytes])
        .collect();
    let refs: Vec<&[u8]> = inputs.iter().map(|v| v.as_slice()).collect();

    let start = Instant::now();
    driver.run_program(&program, &refs).expect("first");
    let first = start.elapsed().as_secs_f64();

    let mut steady = f64::MAX;
    for _ in 0..30 {
        let start = Instant::now();
        driver.run_program(&program, &refs).expect("steady");
        steady = steady.min(start.elapsed().as_secs_f64());
    }

    // The first inference carries the parameters; later ones do not.
    let extra_bytes = params as f64;
    let extra_time = first - steady;
    println!(
        "parameters {params} B, instructions {instructions} B, input {input} B, output {output} B"
    );
    println!("first inference   {:7.2} ms", first * 1e3);
    println!("steady inference  {:7.2} ms (best of 30)", steady * 1e3);
    println!(
        "the {:.1} MB of parameters cost {:.2} ms extra -> {:.1} MB/s on a large bulk-out",
        extra_bytes / 1e6,
        extra_time * 1e3,
        extra_bytes / 1e6 / extra_time
    );
}
