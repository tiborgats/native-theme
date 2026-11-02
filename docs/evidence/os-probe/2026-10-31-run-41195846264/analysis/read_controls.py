#!/usr/bin/env python3
"""Reads the macOS pop-up button's content row from the os-probe controls pass (`[macos.controls]`) as the facts'
values: the distance from the control's alignment rect to its cell's title rect on the start, top and bottom, per
control size, which is `combo_box.body.padding`'s start, top and bottom (the catalogue's `combo_box` content row,
ADD-A127: the content's space, holding the label, starts at the body's content box; the title rect is the box AppKit
lays the title in). The facts lane wrote it (2026-11-02, spec S17 *`evidence`*: an analysis output for a value
computed from values a harness read, as the button pass's padding is).

Run from the repository root (Python 3.11+, the standard library only):

    python3 docs/evidence/os-probe/<run>/analysis/read_controls.py docs/evidence/os-probe/<run>/os-probe-macos-26

It reads, under that image folder, `probe.toml`, `reduce-transparency-off/probe.toml` and
`increase-contrast-on/probe.toml`: per appearance and control size, `[macos.controls.<appearance>.<size>.pop_up_button]`
(`alignmentRectInsets_tlbr` and `isFlipped`, after `sizeToFit`, and `frame_after_sizeToFit_xywh`) and its `.cell`
(`bounds_xywh`, `titleRectForBounds_xywh`), the probe's method in `[macos.controls] method`. The title rect is in the
cell's bounds, which are the control's (at the origin, the frame's size); the control is flipped, so y grows downward
from its top, and the alignment rect is the bounds less the insets (top, left, bottom, right). It writes
`controls.toml` into the image folder, per pass, appearance and size, `[controls.<pass>.<appearance>.<size>.pop_up_button]`:

- `padding_left = title.x - inset.left`, `padding_top = title.y - inset.top` and
  `padding_bottom = (bounds.h - inset.bottom) - (title.y + title.h)` in points (a logical pixel), where the control
  is flipped and its cell's bounds are at the origin with the frame's size; no key otherwise.

`input_image` names every file read, as repository paths.
"""

import sys
import tomllib
from pathlib import Path

PASSES = (("main", "probe.toml"), ("reduce_transparency_off", "reduce-transparency-off/probe.toml"),
          ("increase_contrast_on", "increase-contrast-on/probe.toml"))
SIZES = ("mini", "small", "regular", "large")


def main():
    if len(sys.argv) != 2:
        print(__doc__.split("\n\n")[1], file=sys.stderr)
        return 2
    image = Path(sys.argv[1])
    lines = ["# Output of analysis/read_controls.py (its docstring is the method), file controls.toml.",
             "# input_image: every file it reads, as a repository path (spec S17 *`evidence`*).",
             "input_image = [" + ", ".join(f'"{image.as_posix()}/{rel}"' for _, rel in PASSES) + "]"]
    for name, rel in PASSES:
        controls = tomllib.loads((image / rel).read_text())["macos"]["controls"]
        for appearance in ("aqua", "darkAqua"):
            for size in SIZES:
                p = controls[appearance][size]["pop_up_button"]
                it, il, ib, _ = p["alignmentRectInsets_tlbr"]
                _, _, fw, fh = p["frame_after_sizeToFit_xywh"]
                bx, by, bw, bh = p["cell"]["bounds_xywh"]
                tx, ty, tw, th = p["cell"]["titleRectForBounds_xywh"]
                lines.append(f"\n[controls.{name}.{appearance}.{size}.pop_up_button]")
                if p.get("isFlipped") is True and (bx, by, bw, bh) == (0.0, 0.0, fw, fh):
                    lines.append(f"padding_left = {tx - il!r}")
                    lines.append(f"padding_top = {ty - it!r}")
                    lines.append(f"padding_bottom = {(bh - ib) - (ty + th)!r}")
    out = image / "controls.toml"
    out.write_text("\n".join(lines) + "\n")
    tomllib.loads(out.read_text())
    return 0


if __name__ == "__main__":
    sys.exit(main())
