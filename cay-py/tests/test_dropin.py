"""End-to-end test of the pycoral-style drop-in against a captured reference.

Hardware-gated: runs only when CORAL_TEST_MODEL / CORAL_TEST_INPUT /
CORAL_TEST_EXPECTED point at an Edge TPU model, a uint8 input, and the reference
output (e.g. from LiteRT + the edgetpu delegate). Requires a Coral in runtime
mode and the `cay` wheel installed (maturin develop).
"""

import os


def test_dropin_bit_exact():
    model = os.environ.get("CORAL_TEST_MODEL")
    input_path = os.environ.get("CORAL_TEST_INPUT")
    expected_path = os.environ.get("CORAL_TEST_EXPECTED")
    if not (model and input_path and expected_path):
        return  # skipped without a device + fixtures

    import cay  # scoped: native wheel, only present when the device test runs

    interp = cay.make_interpreter(model)
    with open(input_path, "rb") as f:
        interp.set_input(f.read())
    interp.invoke()
    out = interp.get_output()

    with open(expected_path, "rb") as f:
        expected = f.read()
    assert out[: len(expected)] == expected, "drop-in output not bit-exact vs reference"


if __name__ == "__main__":
    test_dropin_bit_exact()
    print("ok")
