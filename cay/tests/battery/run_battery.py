"""Bit-exact battery: run each Edge TPU model through cay and through
libedgetpu (LiteRT + the edgetpu delegate) and require byte-identical output.

The reference must use preserve_all_tensors: LiteRT reuses an edge-TPU op's
output buffers for downstream ops, so a plain get_tensor after invoke returns
overwritten data. Outputs are matched by tensor name, since cay emits them
in DMA-hint order which need not match the tflite node's output order.
"""

import contextlib
import json
import os
import platform
import subprocess
import sys
import tempfile
from pathlib import Path

import numpy as np
from ai_edge_litert.interpreter import Interpreter, load_delegate

_EDGETPU_LIB = {"Darwin": "libedgetpu.1.0.dylib", "Linux": "libedgetpu.so.1.0"}
_EDGETPU_DIRS = {
    "Darwin": ["/opt/homebrew/lib", "/usr/local/lib"],
    "Linux": ["/usr/lib/aarch64-linux-gnu", "/usr/lib/x86_64-linux-gnu", "/usr/lib"],
}


def locate_edgetpu():
    """libedgetpu path for the reference oracle: CORAL_EDGETPU_LIB, else the
    system install (homebrew / apt)."""
    override = os.environ.get("CORAL_EDGETPU_LIB")
    if override and Path(override).is_file():
        return override
    name = _EDGETPU_LIB.get(platform.system())
    for d in _EDGETPU_DIRS.get(platform.system(), []):
        if name and (Path(d) / name).is_file():
            return str(Path(d) / name)
    raise RuntimeError("libedgetpu not found; set CORAL_EDGETPU_LIB")


HERE = Path(__file__).resolve().parent
MODELS = HERE / "models"
WORK = HERE / "work"
CORAL_INFER = HERE.parents[2] / "target" / "debug" / "coral-infer"
CORAL_PIPELINE = HERE.parents[2] / "target" / "debug" / "coral-pipeline"


def edgetpu_op_outputs(interp):
    """Names of every tensor produced by an edgetpu-custom-op node, in node order."""
    byidx = {t["index"]: t for t in interp.get_tensor_details()}
    names = []
    for op in interp._get_ops_details():
        if "edgetpu" in op["op_name"].lower():
            names += [byidx[o]["name"] for o in op["outputs"] if o in byidx]
    return names


def make_interp(model_path):
    return Interpreter(
        model_path=str(model_path),
        experimental_delegates=[load_delegate(locate_edgetpu())],
        experimental_preserve_all_tensors=True,
    )


def _oracle(interp, input_bytes):
    interp.allocate_tensors()
    inp = interp.get_input_details()[0]
    q = np.frombuffer(input_bytes, dtype=np.uint8).reshape(inp["shape"]).astype(inp["dtype"])
    interp.set_tensor(inp["index"], q)
    interp.invoke()
    n_subgraphs = sum(1 for op in interp._get_ops_details() if "edgetpu" in op["op_name"].lower())
    byidx = {t["index"]: t for t in interp.get_tensor_details()}
    byname = {t["name"]: t for t in interp.get_tensor_details()}
    out = {
        n: interp.get_tensor(byname[n]["index"]).reshape(-1).view(np.uint8).tobytes()
        for n in edgetpu_op_outputs(interp)
    }
    # cay runs the edge-TPU package, whose inputs are the op's input tensors
    # in the layer's representation (after any CPU pre-op like a quantize), not the
    # model input. Feed exactly those — the activation and any state constants.
    inputs = {}
    for op in interp._get_ops_details():
        if "edgetpu" not in op["op_name"].lower():
            continue
        for ti in op["inputs"]:
            if ti not in byidx:
                continue
            with contextlib.suppress(ValueError, RuntimeError):
                inputs[byidx[ti]["name"]] = (
                    interp.get_tensor(ti).reshape(-1).view(np.uint8).tobytes()
                )
    return out, inp["dtype"].__name__, n_subgraphs, inputs


def reference(model_path, input_bytes):
    """Reference in-process. Device-driven models (non-deterministic hints) raise
    at invoke; the caller retries them under reference_subprocess."""
    return _oracle(make_interp(model_path), input_bytes)


def reference_subprocess(model_path):
    """Reference for a descriptor-mode model in an isolated process with
    USB_ENABLE_BULK_DESCRIPTORS_FROM_DEVICE set — libedgetpu needs it, and it must
    not be set for the deterministic models (it makes their oracle far slower).
    Returns (outputs, in_dtype, n_subgraphs, inputs)."""
    wd = Path(tempfile.mkdtemp())
    env = dict(os.environ, USB_ENABLE_BULK_DESCRIPTORS_FROM_DEVICE="1")
    proc = subprocess.run(
        [sys.executable, __file__, "--oracle", str(model_path), str(wd)],
        env=env,
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip()[-400:] or "oracle subprocess failed")
    index = json.loads((wd / "index.json").read_text())
    out = {n: (wd / f"out_{i}.bin").read_bytes() for i, n in enumerate(index["outputs"])}
    inputs = {n: (wd / f"cin_{i}.bin").read_bytes() for i, n in enumerate(index["inputs"])}
    return out, index["in_dtype"], index["n_sub"], inputs


def oracle_entrypoint():
    # Self-contained so descriptor-mode models never touch the in-process
    # delegate (its libusb worker crashes on the non-deterministic-hint error).
    _, _, model, workdir = sys.argv
    interp = make_interp(model)
    interp.allocate_tensors()
    inp = interp.get_input_details()[0]
    nb = int(np.prod(inp["shape"])) * np.dtype(inp["dtype"]).itemsize
    input_bytes = np.random.default_rng(7).integers(0, 256, size=nb, dtype=np.uint8).tobytes()
    out, in_dtype, n_sub, inputs = _oracle(interp, input_bytes)
    wd = Path(workdir)
    index = {"in_dtype": in_dtype, "n_sub": n_sub, "outputs": [], "inputs": []}
    for i, (name, data) in enumerate(out.items()):
        (wd / f"out_{i}.bin").write_bytes(data)
        index["outputs"].append(name)
    for i, (name, data) in enumerate(inputs.items()):
        (wd / f"cin_{i}.bin").write_bytes(data)
        index["inputs"].append(name)
    (wd / "index.json").write_text(json.dumps(index))


def run_coral(model_path, out_prefix, inputs, iters=1):
    """Returns {name: bytes} from coral-infer, fed the edge-TPU-op inputs by name.
    With iters>1 the model runs that many times on the one open device and the
    LAST (cached-weight-reuse) inference is what gets compared.
    Names/sizes print to stderr; a single output writes to <prefix>, multiple to
    <prefix>.<i>.bin."""
    in_args = []
    for i, (name, data) in enumerate(inputs.items()):
        p = Path(f"{out_prefix}.in.{i}")
        p.write_bytes(data)
        in_args.append(f"{name}={p}")
    env = dict(os.environ, CORAL_ITERS=str(iters)) if iters > 1 else None
    proc = subprocess.run(
        [str(CORAL_INFER), str(model_path), str(out_prefix), *in_args],
        capture_output=True,
        text=True,
        env=env,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or proc.stdout.strip())
    order = []
    for line in proc.stderr.splitlines():
        # "output[0] name=NAME 173056 bytes, argmax ..."
        if line.startswith("output[") and " name=" in line:
            idx = int(line[line.index("[") + 1 : line.index("]")])
            name = line.split(" name=", 1)[1].split(" ", 1)[0]
            order.append((idx, name))
    named = {}
    for idx, name in order:
        path = Path(out_prefix) if len(order) == 1 else Path(f"{out_prefix}.{idx}.bin")
        named[name] = path.read_bytes()
    return named


def run_pipeline(segment_paths, out_prefix, inputs):
    """Runs coral-pipeline over the segments in order, feeding the initial inputs
    by name; returns the final segment's outputs as {name: bytes}."""
    in_args = []
    for i, (name, data) in enumerate(inputs.items()):
        p = Path(f"{out_prefix}.pin.{i}")
        p.write_bytes(data)
        in_args.append(f"{name}={p}")
    proc = subprocess.run(
        [str(CORAL_PIPELINE), str(out_prefix), *[str(s) for s in segment_paths], *in_args],
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or proc.stdout.strip())
    order = []
    for line in proc.stderr.splitlines():
        if line.startswith("output[") and " name=" in line:
            idx = int(line[line.index("[") + 1 : line.index("]")])
            order.append((idx, line.split(" name=", 1)[1].split(" ", 1)[0]))
    named = {}
    for idx, name in order:
        path = Path(out_prefix) if len(order) == 1 else Path(f"{out_prefix}.{idx}.bin")
        named[name] = path.read_bytes()
    return named


def pipeline_check(manifest, rng, width):
    """Multi-subgraph gate: a model split into co-compiled segments, run
    seg0->seg1 through cay, must reproduce the full model's edge-TPU output."""
    passed = failed = 0
    for pl in manifest.get("pipelines", []):
        ref_model = MODELS / pl["reference"]
        segs = [MODELS / s["file"] for s in pl["segments"]]
        name = " -> ".join(s.stem[-14:] for s in segs)
        try:
            nb = input_nbytes(ref_model)
            img = rng.integers(0, 256, size=nb, dtype=np.uint8).tobytes()
            ref, _, _, inputs = reference(ref_model, img)
            got = run_pipeline(segs, WORK / f"{ref_model.stem}.pipe", inputs)
            mism = [n for n in ref if n not in got or got[n] != ref[n]]
            if not mism:
                print(f"PASS  pipeline {name:<{width - 9}}  [{len(segs)} segments == full model]")
                passed += 1
            else:
                d = next(iter(mism))
                nd = sum(
                    1
                    for i in range(min(len(got.get(d, b"")), len(ref[d])))
                    if got.get(d, b"")[i] != ref[d][i]
                )
                print(f"FAIL  pipeline {name:<{width - 9}}  '{d}' {nd} bytes differ")
                failed += 1
        except Exception as e:
            print(f"ERROR pipeline {name:<{width - 9}}  {e}")
            failed += 1
    return passed, failed


def input_nbytes(model_path):
    interp = Interpreter(
        model_path=str(model_path),
        experimental_delegates=[load_delegate(locate_edgetpu())],
    )
    interp.allocate_tensors()
    inp = interp.get_input_details()[0]
    return int(np.prod(inp["shape"])) * np.dtype(inp["dtype"]).itemsize


def main():
    WORK.mkdir(exist_ok=True)
    include_quarantined = "--all" in sys.argv
    only = next((a for a in sys.argv[1:] if not a.startswith("-")), None)
    manifest_full = json.loads((HERE / "manifest.json").read_text())
    manifest = manifest_full["models"]
    entries = [e for e in manifest if include_quarantined or not e.get("quarantine")]
    if only:
        entries = [e for e in entries if only in e["file"]]
    models = [MODELS / e["file"] for e in entries]
    skipped = [e["file"] for e in manifest if e.get("quarantine") and not include_quarantined]
    rng = np.random.default_rng(7)
    width = max(len(m.name) for m in models)
    for s in skipped:
        print(f"SKIP  {s:<{width}}  (quarantined — pass --all to include)")
    passed = failed = 0
    for entry in entries:
        m = MODELS / entry["file"]
        try:
            if entry.get("descriptor"):
                # Device-driven model: everything comes from an isolated process.
                ref, in_dtype, n_sub, inputs = reference_subprocess(m)
            else:
                nb = input_nbytes(m)
                inp_bytes = rng.integers(0, 256, size=nb, dtype=np.uint8).tobytes()
                ref, in_dtype, n_sub, inputs = reference(m, inp_bytes)
        except Exception as e:
            if "Failed to load delegate" in str(e):
                print(
                    "\nDEVICE UNAVAILABLE — stopping (the accelerator is not in "
                    "runtime mode; a hung model can de-enumerate it)."
                )
                break
            print(f"ERROR {m.name:<{width}}  {e}")
            failed += 1
            continue
        try:
            iters = entry.get("iters", 1)
            got = run_coral(m, WORK / f"{m.stem}.out", inputs, iters=iters)
            mism = [n for n in ref if n not in got or got[n] != ref[n]]
            tag = "signed-in" if in_dtype == "int8" else "uint8-in"
            reuse = f", {iters}x cache-reuse" if iters > 1 else ""
            if not mism:
                print(
                    f"PASS  {m.name:<{width}}  [{n_sub} subgraph(s), {len(ref)} out, {tag}{reuse}]"
                )
                passed += 1
            else:
                d = next(iter(mism))
                nd = sum(
                    1
                    for i in range(min(len(got.get(d, b"")), len(ref[d])))
                    if got.get(d, b"")[i] != ref[d][i]
                )
                print(
                    f"FAIL  {m.name:<{width}}  {len(mism)}/{len(ref)} tensors differ; "
                    f"first '{d}' {nd} bytes ({tag}, {n_sub} subgraph(s))"
                )
                failed += 1
        except Exception as e:
            print(f"ERROR {m.name:<{width}}  {e}")
            failed += 1
    if not only:
        pp, pf = pipeline_check(manifest_full, rng, width)
        passed += pp
        failed += pf
    print(f"\n{passed} passed, {failed} failed of {passed + failed}")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--oracle":
        oracle_entrypoint()
    else:
        main()
