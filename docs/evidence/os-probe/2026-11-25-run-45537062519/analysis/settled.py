#!/usr/bin/env python3
"""The toolbar sheet's settled captures: per appearance, state and backdrop, the capture taken (settle.settled_at of
settle.captures), the backdrop named and read, and the error where none settled.

Run from the repository root (Python 3.11+, the standard library only):

    python3 <run>/analysis/settled.py <controls pass probe.toml>..."""
import sys
import tomllib

def walk(d, path, out):
    if isinstance(d, dict):
        if "settle" in d:
            out.append((path, d))
        for k, v in d.items():
            walk(v, path + "." + k, out)

for f in sys.argv[1:]:
    t = tomllib.load(open(f, "rb"))["macos"]["button_pass"]["controls"]["toolbar_sheet"]
    print(f)
    rows = []
    walk(t, "toolbar_sheet", rows)
    for path, d in rows:
        s = d["settle"]
        err = f" ERROR {d['error']}" if "error" in d else ""
        print(f"  {path}: backdrop {d.get('backdrop')} read {d.get('backdrop_raw')}, "
              f"settled at {s.get('settled_at')} of {s.get('captures')}{err}")
