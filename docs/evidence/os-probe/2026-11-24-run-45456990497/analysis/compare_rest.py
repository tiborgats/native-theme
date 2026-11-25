#!/usr/bin/env python3
"""The rest captures (normal and highlighted, over the window's background and each backdrop, and their layers) of two
controls passes compared pixel by pixel: per file the pixels that differ and the largest difference of one byte.

Run from the repository root (Python 3.11+, the standard library only):

    python3 <run>/analysis/compare_rest.py <old pass dir> <new pass dir>

each pass dir an image folder's button-pass-controls (or -accent) folder. The PNGs are decoded with
measure_controls.py's read_png (beside this script)."""
import importlib.util
import pathlib
import sys

here = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("measure_controls", here / "measure_controls.py")
measure = importlib.util.module_from_spec(spec)
spec.loader.exec_module(measure)

old_root, new_root = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
differing_files = 0
for old in sorted(old_root.glob("*/*/*.png")):
    if not old.name.startswith(("normal", "highlighted")):
        continue
    rel = old.relative_to(old_root)
    new = new_root / rel
    if not new.exists():
        print(f"{rel}: missing in the new run")
        differing_files += 1
        continue
    ow, oh, orows = measure.read_png(old)
    nw, nh, nrows = measure.read_png(new)
    if (ow, oh) != (nw, nh):
        print(f"{rel}: size {ow}x{oh} -> {nw}x{nh}")
        differing_files += 1
        continue
    pixels, largest = 0, 0
    for orow, nrow in zip(orows, nrows):
        for p, q in zip(orow, nrow):
            if p != q:
                pixels += 1
                largest = max(largest, max(abs(a - b) for a, b in zip(p, q)))
    if pixels:
        differing_files += 1
    print(f"{rel}: {pixels} pixels differ, largest byte difference {largest}")
print(f"files that differ: {differing_files}")
