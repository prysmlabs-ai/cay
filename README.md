# cay

An open userspace runtime for the Coral Edge TPU, written in Rust. It runs
`_edgetpu.tflite` models on the USB Accelerator without libedgetpu, without
TensorFlow Lite, and without the Gasket kernel driver.

**Status**: byte-identical to the libedgetpu reference across 32 compiled
models. USB Accelerator only; PCIe and M.2 are not supported yet.

The stock stack has aged badly: libedgetpu is frozen against an old TensorFlow
ABI, pycoral wants an interpreter that current Python releases no longer ship,
and the PCIe kernel driver stopped building on modern kernels. cay talks to the
device over libusb from userspace, so none of that applies.

## Install

Python, as a drop-in for the pycoral interpreter:

```sh
pip install cay-py
```

The distribution is `cay-py` because `cay` is taken on PyPI; the import is
`cay` either way.

Rust, as a library:

```sh
cargo add cay
```

Or for the command-line tools below:

```sh
cargo install cay
```

You need a compiled model. cay executes the Edge TPU program that
`edgetpu_compiler` emits; it does not compile models itself.

## Use

The Python surface mirrors `tflite_runtime`'s `Interpreter`, so code written
against the Edge TPU delegate runs here with an import swap:

```python
from cay import make_interpreter

interp = make_interpreter("model_edgetpu.tflite")
interp.allocate_tensors()

inp = interp.get_input_details()[0]
interp.set_tensor(inp["index"], image)          # numpy array or bytes
interp.invoke()

for d in interp.get_output_details():
    arr = interp.get_tensor(d["index"])          # numpy, shaped and typed
    scale, zero_point = d["quantization"]
    real = (arr.astype("float32") - zero_point) * scale
```

Details carry `index`, `name`, `shape`, `dtype` and `quantization`, the same
keys tflite uses. Indices are positional handles, not tflite graph indices:
inputs occupy `0..n` and outputs continue from `n`.

**cay runs the Edge TPU program, not the whole graph.** Operators the compiler
left on the CPU do not execute here, so a stock SSD model yields the raw box
and score heads rather than decoded detections, and shapes keep any axis a
later CPU `Squeeze` would have removed. Post-process in the caller. Models
compiled without an on-CPU tail need nothing extra.

`set_input` and `get_output` remain for single-input, concatenated-output use.

From the shell:

```sh
cay-probe                                  # find and identify the device
cay-infer model_edgetpu.tflite out.bin     # single model
cay-pipeline out seg0.tflite seg1.tflite   # co-compiled segments, one device
```

`cay-pipeline` runs a multi-segment model on one open device, matching each
segment's declared inputs by name against a pool seeded with the initial
inputs. Segments run in the order given.

## What it is made of

| package | registry | job |
|---|---|---|
| `cay` | crates.io | USB transport, CSR and DMA handling, DFU, the execution engine, and `cay::program`, which reads the DarwiNN executable format |
| `cay-py` | PyPI | PyO3 bindings shaped like pycoral |

## Correctness

Every model in the battery runs through cay and through libedgetpu, and the
outputs must match byte for byte. The reference side uses
`preserve_all_tensors`, because LiteRT reuses an Edge TPU op's output buffers
for downstream ops and a plain `get_tensor` after invoke returns overwritten
data. Outputs are matched by tensor name: cay emits them in DMA-hint order,
which need not match the tflite node's order.

The battery covers classification, detection and segmentation models, single
segment and co-compiled multi-segment.

## Firmware

A fresh device enumerates in DFU mode and needs the Apex firmware blob before
it appears as an accelerator. Point `cay-flash` at the copy already on your
host:

```sh
cay-flash /path/to/apex_latest_single_ep.bin
```

cay neither bundles nor downloads firmware, and `flash()` in the Python module
takes the same explicit path.

## License

Apache-2.0. See `LICENSE`.

This is a licensed port. The runtime derives from
[libedgetpu](https://github.com/google-coral/libedgetpu), Apache-2.0, with the
TensorFlow Lite dependency removed; `NOTICE` records the attribution. Nothing
here derives from the closed Edge TPU compiler.
