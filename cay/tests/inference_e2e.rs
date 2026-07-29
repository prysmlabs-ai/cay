// End-to-end bit-exact inference against a captured reference. Hardware-gated:
// runs only when CORAL_TEST_MODEL / CORAL_TEST_INPUT / CORAL_TEST_EXPECTED point
// at an Edge TPU model, a uint8 input, and the reference output (e.g. captured
// from LiteRT + the edgetpu delegate). Requires a Coral in runtime mode.

#[test]
fn bit_exact_inference() {
    let (Ok(model_path), Ok(input_path), Ok(expected_path)) = (
        std::env::var("CORAL_TEST_MODEL"),
        std::env::var("CORAL_TEST_INPUT"),
        std::env::var("CORAL_TEST_EXPECTED"),
    ) else {
        return;
    };

    let model = std::fs::read(model_path).expect("read model");
    let pkg = cay::program::extract_package(&model).expect("no DWN1 package");
    let program = cay::program::Program::from_package(pkg).expect("build program");
    let input = std::fs::read(input_path).expect("read input");
    let expected = std::fs::read(expected_path).expect("read reference");

    let mut driver = cay::driver::Driver::open().expect("chip bring-up");
    driver.run().expect("run");
    let outs = driver.run_program(&program, &[&input]).expect("inference");
    // Outputs concatenated in program order; the reference is captured the same
    // way (single output for a classifier, raw heads in hint order for detection).
    let flat: Vec<u8> = outs.concat();

    assert!(
        flat.len() >= expected.len(),
        "output {} shorter than reference {}",
        flat.len(),
        expected.len()
    );
    assert_eq!(
        &flat[..expected.len()],
        &expected[..],
        "inference output not bit-exact vs reference"
    );
}
