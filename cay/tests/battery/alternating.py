"""Reproduction: an inference fails depending on which model ran before it.

The battery runs every model once and passes, so it never sees this. What
provokes it is running two different models in alternation, each in its own
process, the way anything that restarts a detector does:

    A ok    B ok    A FAILS    B ok

The same model repeated four times always passes, so the state that breaks the
next run belongs to the model, not to the endpoint or the process. cay
re-streams parameters on every fresh handle — `resident_param` starts at zero —
so what survives is on the chip.

    uv run python cay/tests/battery/alternating.py [rounds]
"""

import pathlib
import subprocess
import sys
import tempfile

_HERE = pathlib.Path(__file__).resolve().parent
_MODELS = _HERE / "models"
_CAY_INFER = _HERE.parents[2] / "target" / "release" / "cay-infer"

# Two models with different parameter sizes; the larger one is what fails after
# the smaller has run.
_A = "ssd_mobilenet_v2_coco_quant_postprocess_edgetpu.tflite"
_B = "mobilenet_v2_1.0_224_inat_bird_quant_edgetpu.tflite"


def run(model: str) -> bool:
    """True when a single inference in a fresh process succeeds.

    The output goes to a real file: cay-infer cannot write its result to
    /dev/null, and using it here reported device failures that were the
    harness's own doing.
    """
    with tempfile.NamedTemporaryFile(suffix=".bin") as out:
        result = subprocess.run(
            [str(_CAY_INFER), str(_MODELS / model), out.name],
            capture_output=True,
            text=True,
            timeout=60,
        )
    return result.returncode == 0


def main() -> int:
    rounds = int(sys.argv[1]) if len(sys.argv) > 1 else 3
    if not _CAY_INFER.exists():
        print(f"build it first: cargo build --release ({_CAY_INFER} missing)")
        return 2

    failures = []
    print("same model repeated:")
    for i in range(4):
        ok = run(_A)
        print(f"  A{i + 1} {'ok' if ok else 'FAILED'}")
        if not ok:
            failures.append(f"A{i + 1} of the repeated run")

    print("alternating:")
    for i in range(rounds):
        for name, model in (("A", _A), ("B", _B)):
            ok = run(model)
            print(f"  {name} {'ok' if ok else 'FAILED'}")
            if not ok:
                failures.append(f"{name} in round {i + 1}")

    if failures:
        print(f"\n{len(failures)} failure(s): {', '.join(failures)}")
        return 1
    print("\nno failures")
    return 0


if __name__ == "__main__":
    sys.exit(main())
