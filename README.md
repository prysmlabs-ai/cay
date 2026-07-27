# cay

An open userspace runtime for the Coral Edge TPU, written in Rust. It runs
`_edgetpu.tflite` models on the USB Accelerator without libedgetpu, without
TensorFlow Lite, and without the Gasket kernel driver.

**Status**: byte-identical to the libedgetpu reference across 32 compiled
models. USB Accelerator only; PCIe and M.2 are not supported yet.

The stock stack has aged badly: libedgetpu is frozen against an old TensorFlow
ABI, pycoral wants an interpreter that current Python releases no longer ship,
and the PCIe kernel driver stopped building on modern kernels. cay talks to
the device over libusb from userspace, so none of that applies.

## Install

Python, as a drop-in for the pycoral interpreter:

```sh
pip install cay
```

Rust:

```sh
cargo add cay
```

You need a compiled model. cay executes the Edge TPU program that
`edgetpu_compiler` emits; it does not compile models itself.

## Use

The Python surface mirrors the pycoral calls that matter:

```python
from coral_rt import make_interpreter

interp = make_interpreter("model_edgetpu.tflite")
interp.set_input(image_bytes)
interp.invoke()
out = interp.get_output()
```

`input_size` and `output_size` report the byte counts the program expects.
`invoke` concatenates the output tensors in program order: one for a
classifier, the raw heads for a detector.

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

| crate | job |
|---|---|
| `cay-program` | reads the DarwiNN executable format: package, programs, parameter patching, tensor relayout |
| `cay` | USB transport, CSR and DMA handling, DFU, and the execution engine |
| `cay-py` | PyO3 bindings shaped like pycoral |

## Correctness

Every model in the battery runs through cay and through libedgetpu, and
the outputs must match byte for byte. The reference side uses
`preserve_all_tensors`, because LiteRT reuses an Edge TPU op's output buffers
for downstream ops and a plain `get_tensor` after invoke returns overwritten
data. Outputs are matched by tensor name: cay emits them in DMA-hint
order, which need not match the tflite node's order.

The battery covers classification, detection and segmentation models, single
segment and co-compiled multi-segment.

## Firmware

A fresh device enumerates in DFU mode and needs the Apex firmware blob before
it appears as an accelerator. Point `cay-flash` at the copy already on your
host:

```sh
cay-flash /path/to/apex_latest_single_ep.bin
```

cay neither bundles nor downloads firmware, and `flash()` in the Python
module takes the same explicit path.

## License

Apache-2.0. See `LICENSE`.

This is a licensed port. The runtime derives from
[libedgetpu](https://github.com/google-coral/libedgetpu), Apache-2.0, with the
TensorFlow Lite dependency removed; `NOTICE` records the attribution. Nothing
here derives from the closed Edge TPU compiler.
