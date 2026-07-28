"""The tflite Interpreter surface, against a real compiled model.

Most of this needs no accelerator: parsing, tensor metadata, index handling and
input staging all come from the model file. Only `invoke` touches hardware, so
the bit-exact check stays gated behind CORAL_TEST_* while everything else runs
anywhere the wheel is installed.

Run after `maturin develop`, or against an installed wheel:
    uv run pytest cay/cay-py/tests
"""

import os
import pathlib

import pytest

cay = pytest.importorskip("cay", reason="native wheel not installed")
np = pytest.importorskip("numpy")

_MODELS = pathlib.Path(__file__).parents[2] / "cay" / "tests" / "battery" / "models"


def _a_model() -> str:
    found = sorted(_MODELS.glob("*_edgetpu.tflite"))
    if not found:
        pytest.skip(f"no compiled models under {_MODELS}")
    return str(found[0])


@pytest.fixture(scope="module")
def interp():
    i = cay.make_interpreter(_a_model())
    i.allocate_tensors()
    return i


def test_input_details_carry_what_a_caller_needs(interp):
    details = interp.get_input_details()
    assert details, "model declares no inputs"
    first = details[0]
    assert set(first) >= {"index", "name", "shape", "dtype", "quantization"}
    assert first["index"] == 0
    scale, zero_point = first["quantization"]
    assert isinstance(scale, float) and isinstance(zero_point, int)


def test_output_details_report_shape_and_quantization(interp):
    details = interp.get_output_details()
    assert details, "model declares no outputs"
    for d in details:
        assert len(d["shape"]) == 4 and d["shape"][0] == 1, "shape is batch-major"
        assert d["dtype"] in (np.int8, np.uint8, np.int16, np.uint16, np.int32)
        scale, _ = d["quantization"]
        assert scale > 0, "a quantized output needs a positive scale"


def test_indices_do_not_collide_between_inputs_and_outputs(interp):
    ins = [d["index"] for d in interp.get_input_details()]
    outs = [d["index"] for d in interp.get_output_details()]
    assert set(ins).isdisjoint(outs), "one index space, as in tflite"
    assert outs[0] == len(ins), "outputs continue where inputs stop"


def test_set_tensor_accepts_a_numpy_array(interp):
    d = interp.get_input_details()[0]
    n = d["shape"][0] * d["shape"][1]
    interp.set_tensor(d["index"], np.zeros(n, dtype=d["dtype"]))


def test_set_tensor_rejects_the_wrong_size(interp):
    d = interp.get_input_details()[0]
    with pytest.raises(RuntimeError, match="expects"):
        interp.set_tensor(d["index"], np.zeros(3, dtype=d["dtype"]))


def test_out_of_range_indices_are_index_errors(interp):
    with pytest.raises(IndexError):
        interp.set_tensor(999, np.zeros(1, dtype=np.uint8))
    with pytest.raises(IndexError):
        interp.get_tensor(999)


def test_get_tensor_before_invoke_says_so():
    fresh = cay.make_interpreter(_a_model())
    fresh.allocate_tensors()
    idx = fresh.get_output_details()[0]["index"]
    with pytest.raises(RuntimeError, match="invoke"):
        fresh.get_tensor(idx)


@pytest.mark.skipif(not os.environ.get("CORAL_TEST_MODEL"), reason="needs an attached accelerator")
def test_invoke_is_bit_exact_against_the_reference():
    interp = cay.make_interpreter(os.environ["CORAL_TEST_MODEL"])
    interp.allocate_tensors()
    with open(os.environ["CORAL_TEST_INPUT"], "rb") as f:
        interp.set_tensor(interp.get_input_details()[0]["index"], f.read())
    interp.invoke()

    # Every output must come back shaped and typed as its details promised.
    for d in interp.get_output_details():
        arr = interp.get_tensor(d["index"])
        assert list(arr.shape) == list(d["shape"])
        assert arr.dtype == d["dtype"]

    with open(os.environ["CORAL_TEST_EXPECTED"], "rb") as f:
        expected = f.read()
    assert interp.get_output()[: len(expected)] == expected
