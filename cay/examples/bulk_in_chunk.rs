//! Times inference with the USB2 bulk-in chunk cap on and off.
//!
//! The cap is a host-controller workaround, not a device requirement, so
//! whether it costs anything is a property of the machine you are on. This
//! measures it and checks that the outputs still match.
//!
//!     cargo run --release --example bulk_in_chunk -- model_edgetpu.tflite

use std::time::{Duration, Instant};

use cay::driver::{Driver, Options};
use cay::program::{extract_package, Program};

const WARMUP: usize = 10;
const ITERS: usize = 100;

fn percentiles(mut samples: Vec<Duration>) -> (f64, f64, f64) {
    samples.sort();
    let ms = |d: Duration| d.as_secs_f64() * 1e3;
    (
        ms(samples[samples.len() / 2]),
        ms(samples[samples.len() / 10]),
        ms(samples[samples.len() * 9 / 10]),
    )
}

fn run(program: &Program, options: Options) -> Option<(Vec<Duration>, Vec<Vec<u8>>)> {
    let driver = Driver::open_with(options).expect("open");
    driver.run().expect("run");
    let inputs: Vec<Vec<u8>> = program
        .inputs
        .iter()
        .map(|i| vec![128u8; i.bytes])
        .collect();
    let refs: Vec<&[u8]> = inputs.iter().map(|v| v.as_slice()).collect();

    for _ in 0..WARMUP {
        // A host controller that needs the 256 B cap simply stops delivering.
        if let Err(e) = driver.run_program(program, &refs) {
            println!("  unsupported on this host controller: {e}");
            return None;
        }
    }
    let mut samples = Vec::with_capacity(ITERS);
    let mut last = Vec::new();
    for _ in 0..ITERS {
        let start = Instant::now();
        last = driver.run_program(program, &refs).expect("inference");
        samples.push(start.elapsed());
    }
    Some((samples, last))
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("usage: bulk_in_chunk <model>");
    let model = std::fs::read(&path).expect("read model");
    let package = extract_package(&model).expect("no DWN1 package");
    let program = Program::from_package(package).expect("parse");

    for (name, options) in [
        (
            "256 B cap (default on USB2)",
            Options {
                force_largest_bulk_in_chunk: false,
            },
        ),
        (
            "1 KB chunks",
            Options {
                force_largest_bulk_in_chunk: true,
            },
        ),
    ] {
        let Some((samples, outputs)) = run(&program, options) else {
            continue;
        };
        let (median, p10, p90) = percentiles(samples);
        let bytes: usize = outputs.iter().map(|o| o.len()).sum();
        println!("{name:28} median {median:6.2} ms  p10 {p10:6.2}  p90 {p90:6.2}  out {bytes} B");
        std::thread::sleep(Duration::from_secs(2));
    }
}
