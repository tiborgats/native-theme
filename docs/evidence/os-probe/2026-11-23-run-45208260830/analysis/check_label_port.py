#!/usr/bin/env python3
"""Checks measure_controls.py's `label` against measure_button_layers.py's own `measure_button` on a push button.

Run from the repository root (Python 3.11+, the standard library and git):

    python3 docs/evidence/os-probe/2026-11-23-run-45208260830/analysis/check_label_port.py <run>/<image folder> CONTROLS

It takes measure_button_layers.py as run 41006842699 kept it (commit 5d48e80f, the folder
docs/evidence/os-probe/2026-10-30-run-41006842699/analysis/, a folder this branch does not hold), runs its
functions from `read_png` to `measure_button` unchanged on the push button (`targets.push`) of the image's controls pass,
per appearance and state (normal, highlighted, pressed; the button script's own frame is the part, its cell the
`controls.push` entry with its title rect), and prints that script's `label`, `label_pixels` and `fill` beside the
ones CONTROLS (measure_controls.py's output over the same folder) holds. It exits 1 where a label differs.
"""

import subprocess
import sys
import tomllib
from pathlib import Path

SRC = "5d48e80f:docs/evidence/os-probe/2026-10-30-run-41006842699/analysis/measure_button_layers.py"


def main():
    image, controls_out = Path(sys.argv[1]), Path(sys.argv[2])
    text = subprocess.run(["git", "show", SRC], check=True, capture_output=True, text=True).stdout
    body = text[text.index("class Bad(Exception):"):text.index("def emit(")]
    ns = {}
    exec("import struct, sys, tomllib, zlib\nfrom collections import Counter\nfrom math import sqrt\n"
         "from pathlib import Path\nSIDES = ('top', 'right', 'bottom', 'left')\n"
         "CORNERS = ('top_left', 'top_right', 'bottom_right', 'bottom_left')\n" + body, ns)
    root = image / "button-pass-controls"
    c = tomllib.loads((root / "probe.toml").read_text())["macos"]["button_pass"]["controls"]
    ours = tomllib.loads(controls_out.read_text())
    part = [round(v) for v in c["targets"]["push"]["part_in_window_from_top_left_xywh"]]
    frame = c["controls"]["push"]["in_window_frame_from_top_left_xywh"]
    if [round(v) for v in frame] != part:
        print(f"push frame {frame} is not its part {part}")
        return 1
    bad = 0
    for ap in ("aqua", "darkAqua"):
        for st in ("normal", "highlighted", "pressed"):
            state = c[ap][st] if st != "pressed" else c[ap]["pressed"]["push"]
            lay = state["layer"]
            layer = ns["Layer"](root / lay["file"], tuple(lay["file_rect_px_xywh"][:2]))
            res = ns["measure_button"](layer, part, c["controls"]["push"], None)
            mine = ours.get(f"{ap}.{st}.push", {})
            same = res.get("label") == mine.get("label") and res.get("label_pixels") == mine.get("label_pixels")
            bad += not same
            print(f"{ap}.{st}.push: measure_button_layers label {res.get('label')} ({res.get('label_pixels')} px, "
                  f"fill {res.get('fill')}); measure_controls label {mine.get('label')} "
                  f"({mine.get('label_pixels')} px, fill {mine.get('fill')}, rect {mine.get('label_rect_from')}) "
                  f"{'same' if same else 'DIFFERENT'}")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
