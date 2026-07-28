"""Download the model battery listed in manifest.json into models/, verifying
sha256. Re-running skips files already present with the right hash.

Arguments narrow the set to filenames containing any of them, which is how CI
pulls one model instead of the whole battery:

    python fetch.py                       # everything
    python fetch.py ssd_mobilenet_v1      # both ssd_mobilenet_v1 models
"""

import hashlib
import json
import sys
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
MODELS = HERE / "models"


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def fetch_one(url, dest, want):
    """Downloads dest from url (verifying sha256); skips if already valid.
    Returns 1 on a sha mismatch, else 0."""
    if dest.exists() and sha256(dest) == want:
        print(f"ok    {dest.name}")
        return 0
    print(f"fetch {dest.name}")
    urllib.request.urlretrieve(url, dest)
    got = sha256(dest)
    if got != want:
        print(f"  SHA MISMATCH: got {got}, want {want}")
        return 1
    return 0


def main():
    manifest = json.loads((HERE / "manifest.json").read_text())
    MODELS.mkdir(exist_ok=True)
    base = manifest["source"]
    wanted = sys.argv[1:]
    bad = 0
    hits = 0

    def keep(name):
        return not wanted or any(w in name for w in wanted)

    for entry in manifest["models"]:
        if not keep(entry["file"]):
            continue
        hits += 1
        # Most models share the test_data base; a few (e.g. YOLO) carry a full url.
        url = entry.get("url", f"{base}/{entry['file']}")
        bad += fetch_one(url, MODELS / entry["file"], entry["sha256"])
    # Pipeline segments live under a subdir in the source tree (src != file).
    for pl in manifest.get("pipelines", []):
        for seg in pl["segments"]:
            if not keep(seg["file"]):
                continue
            hits += 1
            bad += fetch_one(f"{base}/{seg['src']}", MODELS / seg["file"], seg["sha256"])

    # An unmatched filter would leave models/ empty and every test skipping.
    if wanted and not hits:
        print(f"no model in the manifest matches {wanted}")
        bad += 1
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
