"""Coral Edge TPU detector backed by cay, a userspace runtime over libusb.

cay runs the Edge TPU program and nothing else, so an SSD graph's CPU tail —
two DEQUANTIZE ops and TFLite_Detection_PostProcess — does not execute. The
model returns box encodings and per-class scores; the decode below is that
tail.

Install:  pip install cay-py
Config:

    detectors:
      coral:
        type: cay

    model:
      path: /path/to/ssd_mobilenet_v2_coco_quant_postprocess_edgetpu.tflite
      model_type: ssd
      width: 300
      height: 300
"""

import atexit
import logging
import struct
from typing import Literal

import cv2
import numpy as np
from frigate.detectors.detection_api import DetectionApi
from frigate.detectors.detector_config import BaseDetectorConfig, ModelTypeEnum
from pydantic import ConfigDict, Field

import cay

logger = logging.getLogger(__name__)

DETECTOR_KEY = "cay"


# Flatbuffer reader for the tflite schema: a table holds a backwards offset to
# its vtable, the vtable gives each field's byte offset, and a vector is a
# length followed by its elements. Field indices are from schema.fbs.

_MODEL_OPCODES, _MODEL_SUBGRAPHS, _MODEL_BUFFERS = 1, 2, 4
_OPCODE_CUSTOM_CODE = 1
_SUBGRAPH_TENSORS, _SUBGRAPH_OPERATORS = 0, 3
_TENSOR_SHAPE, _TENSOR_TYPE, _TENSOR_BUFFER, _TENSOR_QUANTIZATION = 0, 1, 2, 4
_QUANT_SCALE, _QUANT_ZERO_POINT = 2, 3
_OP_OPCODE_INDEX, _OP_INPUTS, _OP_CUSTOM_OPTIONS = 0, 1, 5
_BUFFER_DATA = 0

# TensorType, for the constants this file reads.
_TENSOR_DTYPES = {0: np.float32, 2: np.int32, 3: np.uint8, 9: np.int8}

_POSTPROCESS_OP = "TFLite_Detection_PostProcess"


def _u16(b, p):
    return int.from_bytes(b[p : p + 2], "little")


def _u32(b, p):
    return int.from_bytes(b[p : p + 4], "little")


def _i32(b, p):
    return int.from_bytes(b[p : p + 4], "little", signed=True)


def _field(b, table, index):
    """Position of a table field, or None when the field was not written."""
    vtable = table - _i32(b, table)
    if 4 + index * 2 >= _u16(b, vtable):
        return None
    offset = _u16(b, vtable + 4 + index * 2)
    return table + offset if offset else None


def _vector(b, table, index):
    """(position of the first element, count) for a vector field."""
    p = _field(b, table, index)
    if p is None:
        return None, 0
    start = p + _u32(b, p)
    return start + 4, _u32(b, start)


def _member(b, start, i):
    """The i-th table in a vector of tables."""
    p = start + i * 4
    return p + _u32(b, p)


def _uint_field(b, table, index, default=0):
    p = _field(b, table, index)
    return default if p is None else _u32(b, p)


def _byte_field(b, table, index, default=0):
    """A single-byte field. TensorType is a byte enum, not a word."""
    p = _field(b, table, index)
    return default if p is None else b[p]


def _string_field(b, table, index):
    p = _field(b, table, index)
    if p is None:
        return None
    start = p + _u32(b, p)
    return b[start + 4 : start + 4 + _u32(b, start)].decode()


def _int_vector(b, table, index):
    start, count = _vector(b, table, index)
    return [] if start is None else list(struct.unpack_from(f"<{count}i", b, start))


def _byte_vector(b, table, index):
    start, count = _vector(b, table, index)
    return b"" if start is None else b[start : start + count]


def _read_constant(model, tensors_at, buffers_at, tensor_index):
    """A constant tensor as float32, dequantized if the model stored it quantized.

    Some exports store anchors as uint8 with a scale, so the declared type
    decides and not the byte count.
    """
    tensor = _member(model, tensors_at, tensor_index)
    shape = _int_vector(model, tensor, _TENSOR_SHAPE)
    dtype = _TENSOR_DTYPES.get(_byte_field(model, tensor, _TENSOR_TYPE))
    if dtype is None:
        raise ValueError("unsupported tensor type for a model constant")

    buffer = _member(model, buffers_at, _uint_field(model, tensor, _TENSOR_BUFFER))
    values = np.frombuffer(_byte_vector(model, buffer, _BUFFER_DATA), dtype)
    if values.size != int(np.prod(shape)):
        raise ValueError(f"constant declares shape {shape} but holds {values.size}")
    values = values.reshape(shape)
    if dtype is np.float32:
        return values

    quantization = _table_field(model, tensor, _TENSOR_QUANTIZATION)
    if quantization is None:
        raise ValueError("a quantized constant with no quantization parameters")
    scale = _float_vector(model, quantization, _QUANT_SCALE)
    zero_point = _long_vector(model, quantization, _QUANT_ZERO_POINT)
    return (values.astype(np.float32) - (zero_point[0] if zero_point else 0)) * (
        scale[0] if scale else 1.0
    )


def _table_field(b, table, index):
    p = _field(b, table, index)
    return None if p is None else p + _u32(b, p)


def _float_vector(b, table, index):
    start, count = _vector(b, table, index)
    return [] if start is None else list(struct.unpack_from(f"<{count}f", b, start))


def _long_vector(b, table, index):
    start, count = _vector(b, table, index)
    return [] if start is None else list(struct.unpack_from(f"<{count}q", b, start))


# An operator's custom options are FlexBuffers, where a packed type byte is
# (type << 2) | log2(byte width).
_FLEX_INT, _FLEX_FLOAT, _FLEX_BOOL, _FLEX_MAP = 1, 3, 26, 9


def _flex_scalar(data, pos, width, kind):
    raw = data[pos : pos + width]
    if kind == _FLEX_FLOAT:
        return struct.unpack("<f" if width == 4 else "<d", raw)[0]
    if kind == _FLEX_BOOL:
        return any(raw)
    return int.from_bytes(raw, "little", signed=(kind == _FLEX_INT))


def _flexbuffer_map(data):
    """A flat FlexBuffers map of scalars, as a dict.

    The root sits at the end of the buffer, a map points at its vector of
    values, and the keys vector hangs off the three slots preceding it.
    """
    root_width = data[-1]
    packed = data[-2]
    if packed >> 2 != _FLEX_MAP:
        raise ValueError("custom options are not a FlexBuffers map")

    root = len(data) - 2 - root_width
    values = root - int.from_bytes(data[root : root + root_width], "little")
    width = 1 << (packed & 3)
    count = int.from_bytes(data[values - width : values], "little")

    keys_width_at = values - width * 2
    keys_width = int.from_bytes(data[keys_width_at : keys_width_at + width], "little")
    keys_at = values - width * 3
    keys = keys_at - int.from_bytes(data[keys_at : keys_at + width], "little")

    types = data[values + count * width : values + count * width + count]
    options = {}
    for i in range(count):
        key_at = keys + i * keys_width
        key = key_at - int.from_bytes(data[key_at : key_at + keys_width], "little")
        name = data[key : data.index(b"\x00", key)].decode()
        packed_type = types[i]
        options[name] = _flex_scalar(
            data, values + i * width, 1 << (packed_type & 3), packed_type >> 2
        )
    return options


def read_ssd_postprocess(model_path):
    """The anchors and box scales an SSD model's postprocess op was built with.

    Raises when the model carries no such op, meaning it is not an SSD graph
    this detector can finish.
    """
    with open(model_path, "rb") as f:
        model = f.read()

    root = _u32(model, 0)
    opcodes_at, opcode_count = _vector(model, root, _MODEL_OPCODES)
    custom_codes = [
        _string_field(model, _member(model, opcodes_at, i), _OPCODE_CUSTOM_CODE)
        for i in range(opcode_count)
    ]
    subgraphs_at, _ = _vector(model, root, _MODEL_SUBGRAPHS)
    subgraph = _member(model, subgraphs_at, 0)
    tensors_at, _ = _vector(model, subgraph, _SUBGRAPH_TENSORS)
    buffers_at, _ = _vector(model, root, _MODEL_BUFFERS)
    operators_at, operator_count = _vector(model, subgraph, _SUBGRAPH_OPERATORS)

    for i in range(operator_count):
        op = _member(model, operators_at, i)
        if custom_codes[_uint_field(model, op, _OP_OPCODE_INDEX)] != _POSTPROCESS_OP:
            continue

        # The op's inputs are (box encodings, class scores, anchors).
        anchors = _read_constant(
            model, tensors_at, buffers_at, _int_vector(model, op, _OP_INPUTS)[2]
        )
        return anchors, _flexbuffer_map(_byte_vector(model, op, _OP_CUSTOM_OPTIONS))

    raise ValueError(f"{model_path} has no {_POSTPROCESS_OP} op")


class CayDetectorConfig(BaseDetectorConfig):
    """Coral Edge TPU through cay, an open userspace runtime — no libedgetpu."""

    model_config = ConfigDict(title="Coral Edge TPU (cay)")

    type: Literal[DETECTOR_KEY]
    device: str | None = Field(
        default=None,
        title="Device",
        description=(
            "Reserved for choosing among several accelerators. "
            "cay opens the first USB device it finds."
        ),
    )


class CayDetector(DetectionApi):
    type_key = DETECTOR_KEY
    supported_models = [ModelTypeEnum.ssd]  # noqa: RUF012 — shape set by DetectionApi

    def __init__(self, detector_config: CayDetectorConfig):
        self.model_type = detector_config.model.model_type
        if self.model_type != ModelTypeEnum.ssd:
            raise ValueError(
                f"cay supports {ModelTypeEnum.ssd.value} models, not {self.model_type}"
            )

        path = detector_config.model.path
        self.interpreter = cay.make_interpreter(path)
        self.interpreter.allocate_tensors()

        self.input_index = self.interpreter.get_input_details()[0]["index"]
        outputs = self.interpreter.get_output_details()

        # Last axis tells the heads apart: four numbers per anchor is the box
        # encoding. Names differ between model vintages, shapes do not.
        boxes = next((d for d in outputs if d["shape"][-1] == 4), None)
        scores = next((d for d in outputs if d["shape"][-1] != 4), None)
        if boxes is None or scores is None:
            raise ValueError(
                "expected an SSD box head and a score head, got shapes "
                f"{[d['shape'] for d in outputs]}"
            )
        self.boxes_index, self.scores_index = boxes["index"], scores["index"]
        self.boxes_scale, self.boxes_zero_point = boxes["quantization"]
        self.scores_scale, self.scores_zero_point = scores["quantization"]
        self.columns = scores["shape"][-1]

        self.anchors, options = read_ssd_postprocess(path)
        self.y_scale = options["y_scale"]
        self.x_scale = options["x_scale"]
        self.h_scale = options["h_scale"]
        self.w_scale = options["w_scale"]
        self.iou_threshold = options.get("nms_iou_threshold", 0.6)

        if len(self.anchors) != scores["shape"][-2]:
            raise ValueError(
                f"model declares {len(self.anchors)} anchors but the score head "
                f"has {scores['shape'][-2]} rows"
            )

        # A host that walks away mid-inference leaves the accelerator wanting a
        # physical replug.
        atexit.register(self.interpreter.close)

        self.min_score = 0.4
        self.max_detections = 20

        # One comparison per anchor instead of a multiply across every class.
        # The scale is positive, so the ordering survives.
        self.min_score_quantized = int(
            np.ceil(self.min_score / self.scores_scale + self.scores_zero_point)
        )

        logger.info(
            f"cay: {len(self.anchors)} anchors, {self.columns - 1} classes, "
            f"box scales y={self.y_scale} x={self.x_scale} "
            f"h={self.h_scale} w={self.w_scale}"
        )

    def _decode(self, encodings, anchors):
        """Box encodings against their anchors, as normalized ymin/xmin/ymax/xmax.

        Anchors are (y_center, x_center, height, width); the encoding is the
        offset from the anchor over the scales the model was trained with.
        """
        y_center = encodings[:, 0] / self.y_scale * anchors[:, 2] + anchors[:, 0]
        x_center = encodings[:, 1] / self.x_scale * anchors[:, 3] + anchors[:, 1]
        half_h = np.exp(encodings[:, 2] / self.h_scale) * anchors[:, 2] / 2
        half_w = np.exp(encodings[:, 3] / self.w_scale) * anchors[:, 3] / 2
        return np.stack(
            [
                y_center - half_h,
                x_center - half_w,
                y_center + half_h,
                x_center + half_w,
            ],
            axis=1,
        )

    def detect_raw(self, tensor_input):
        self.interpreter.set_tensor(self.input_index, tensor_input)
        self.interpreter.invoke()

        detections = np.zeros((self.max_detections, 6), np.float32)

        # Column 0 is background.
        scores_quantized = self.interpreter.get_tensor(self.scores_index).reshape(-1, self.columns)[
            :, 1:
        ]
        best_quantized = scores_quantized.max(axis=1)
        keep = best_quantized >= self.min_score_quantized
        if not keep.any():
            return detections

        class_ids = scores_quantized[keep].argmax(axis=1)
        scores = (
            best_quantized[keep].astype(np.float32) - self.scores_zero_point
        ) * self.scores_scale

        encodings = (
            self.interpreter.get_tensor(self.boxes_index).reshape(-1, 4)[keep].astype(np.float32)
            - self.boxes_zero_point
        ) * self.boxes_scale
        boxes = self._decode(encodings, self.anchors[keep])

        # Fast NMS: one class per box, suppression across all of them. cv2
        # wants x, y, width, height.
        widths = boxes[:, 3] - boxes[:, 1]
        heights = boxes[:, 2] - boxes[:, 0]
        indices = cv2.dnn.NMSBoxes(
            np.stack([boxes[:, 1], boxes[:, 0], widths, heights], axis=1).tolist(),
            scores.tolist(),
            self.min_score,
            self.iou_threshold,
        )
        if len(indices) == 0:
            return detections

        indices = np.array(indices).ravel()[: self.max_detections]
        detections[: len(indices), 0] = class_ids[indices]
        detections[: len(indices), 1] = scores[indices]
        detections[: len(indices), 2:] = np.clip(boxes[indices], 0.0, 1.0)
        return detections
