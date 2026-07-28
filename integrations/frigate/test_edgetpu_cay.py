"""The parts of the Frigate detector that run without Frigate or hardware.

The plugin is one file by necessity — Frigate discovers detectors by importing
every module in its plugins package — so the test loads it with the Frigate and
cv2 imports stubbed. What is worth covering here is the flatbuffer walk and the
SSD decode: everything else in the file is Frigate glue that only means
something inside Frigate.

    uv run pytest cay/integrations/frigate
"""

import importlib.util
import pathlib
import sys
import types

import pytest

np = pytest.importorskip("numpy")

_PLUGIN = pathlib.Path(__file__).with_name("edgetpu_cay.py")
_MODELS = pathlib.Path(__file__).parents[2] / "cay" / "tests" / "battery" / "models"


class ModelTypeEnum(str):
    """Stands in for Frigate's enum; the plugin only reads `ssd` off it."""

    ssd = "ssd"


def _stub(name, **attrs):
    module = types.ModuleType(name)
    module.__dict__.update(attrs)
    return module


@pytest.fixture(scope="module")
def plugin():
    """The plugin module, with everything it imports from Frigate faked out."""
    stubs = {
        "cay": _stub("cay", make_interpreter=lambda path: None),
        "cv2": _stub("cv2", dnn=_stub("cv2.dnn", NMSBoxes=lambda *a, **k: [])),
        "pydantic": _stub("pydantic", ConfigDict=dict, Field=lambda **k: None),
        "frigate": _stub("frigate"),
        "frigate.detectors": _stub("frigate.detectors"),
        "frigate.detectors.detection_api": _stub("...", DetectionApi=object),
        "frigate.detectors.detector_config": _stub(
            "...", BaseDetectorConfig=object, ModelTypeEnum=ModelTypeEnum
        ),
    }
    saved = {k: sys.modules.get(k) for k in stubs}
    sys.modules.update(stubs)
    try:
        spec = importlib.util.spec_from_file_location("edgetpu_cay_under_test", _PLUGIN)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        yield module
    finally:
        for key, value in saved.items():
            if value is None:
                sys.modules.pop(key, None)
            else:
                sys.modules[key] = value


def _model(name):
    path = _MODELS / name
    if not path.exists():
        pytest.skip(f"{name} not fetched; see cay/tests/battery/fetch.py")
    return str(path)


def test_reads_the_anchors_and_scales_the_default_model_was_built_with(plugin):
    anchors, options = plugin.read_ssd_postprocess(
        _model("ssd_mobilenet_v2_coco_quant_postprocess_edgetpu.tflite")
    )
    assert anchors.shape == (1917, 4)
    assert anchors.dtype == np.float32
    # Anchors are (y_center, x_center, height, width), normalized.
    assert (anchors[:, 2:] > 0).all()
    assert anchors[:, :2].min() > -0.2 and anchors[:, :2].max() < 1.2
    # The TF Object Detection API's box encoding scales.
    assert (options["y_scale"], options["x_scale"]) == (10.0, 10.0)
    assert (options["h_scale"], options["w_scale"]) == (5.0, 5.0)
    assert options["num_classes"] == 90


def test_reads_a_model_with_a_different_anchor_count(plugin):
    """The anchor count is per-model, which is why it is read and not assumed."""
    anchors, _ = plugin.read_ssd_postprocess(
        _model("ssdlite_mobiledet_coco_qat_postprocess_edgetpu.tflite")
    )
    assert anchors.shape == (2034, 4)


def test_reads_anchors_a_model_stored_quantized(plugin):
    """Some exports keep the anchors as uint8 with a scale, not float32."""
    anchors, options = plugin.read_ssd_postprocess(
        _model("ssd_mobilenet_v1_coco_quant_no_nms_edgetpu.tflite")
    )
    assert anchors.shape == (1917, 4)
    assert anchors.dtype == np.float32
    assert (anchors[:, 2:] > 0).all()
    assert anchors[:, :2].min() > -0.2 and anchors[:, :2].max() < 1.2
    assert (options["y_scale"], options["h_scale"]) == (10.0, 5.0)


def test_a_model_with_no_postprocess_op_says_so(plugin, tmp_path):
    not_a_detector = tmp_path / "classifier.tflite"
    not_a_detector.write_bytes(
        pathlib.Path(_model("mobilenet_v1_1.0_224_quant_edgetpu.tflite")).read_bytes()
    )
    with pytest.raises(ValueError, match="TFLite_Detection_PostProcess"):
        plugin.read_ssd_postprocess(str(not_a_detector))


def test_decoding_an_all_zero_encoding_returns_the_anchor_itself(plugin):
    """A zero offset means the box is the anchor: same center, same size."""
    detector = plugin.CayDetector.__new__(plugin.CayDetector)
    detector.y_scale = detector.x_scale = 10.0
    detector.h_scale = detector.w_scale = 5.0

    anchors = np.array([[0.5, 0.25, 0.2, 0.4]], dtype=np.float32)
    boxes = detector._decode(np.zeros((1, 4), dtype=np.float32), anchors)

    ymin, xmin, ymax, xmax = boxes[0]
    assert (ymin, ymax) == pytest.approx((0.4, 0.6))
    assert (xmin, xmax) == pytest.approx((0.05, 0.45))


def test_decoding_moves_and_scales_the_box_by_the_encoding(plugin):
    detector = plugin.CayDetector.__new__(plugin.CayDetector)
    detector.y_scale = detector.x_scale = 10.0
    detector.h_scale = detector.w_scale = 5.0

    anchors = np.array([[0.5, 0.5, 0.2, 0.2]], dtype=np.float32)
    # ty=10 shifts the center down by a full anchor height; th=5 scales the
    # height by e.
    encodings = np.array([[10.0, 0.0, 5.0, 0.0]], dtype=np.float32)
    ymin, xmin, ymax, xmax = detector._decode(encodings, anchors)[0]

    assert (ymin + ymax) / 2 == pytest.approx(0.7)
    assert ymax - ymin == pytest.approx(0.2 * np.e, rel=1e-5)
    assert xmax - xmin == pytest.approx(0.2)
