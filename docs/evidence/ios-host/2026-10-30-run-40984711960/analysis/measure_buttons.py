#!/usr/bin/env python3
"""Measures the UIButton captures of the iOS host (tools/ios-host, spike I's run 40984711960).

Run from the repository root, once per image folder:

    python3 -I docs/evidence/ios-host/2026-10-30-run-40984711960/analysis/measure_buttons.py \
        docs/evidence/ios-host/2026-10-30-run-40984711960/ios-host-macos-26

Reads the folder's run1/index.toml (each cell's button frame, in points, inside the cell) and every screenshot crop
run1/screen/<appearance>/<style>/<size>/<state>.png it lists (the simulator's own screenshot, sRGB, 3 px per point),
and writes <folder>/analysis.toml. Needs Pillow.

Per cell (`[<appearance>.<style>.<size>.<state>]`), inside the button frame shrunk to its middle 70 % of the width
and 60 % of the height (the title's area, clear of the corners):
- `bezel`: the most frequent colour (the bezel's composite over the cell, or the cell where no bezel is drawn);
- `text`: the most frequent colour among the pixels that differ from `bezel` by more than 40 levels in some channel
  (the title's fully covered pixels; its antialiased edge pixels each have their own colour), `text_pixels` their
  count;
- `background`: the cell's pixel (4, 4), outside the button (systemBackground);
- `text_opaque` / `bezel_opaque`: whether the capture proves the colour opaque against what lies under it (`bezel`
  for the title, `background` for the bezel): some channel is 0 where the colour beneath is above 0, or 255 where it
  is below 255. A colour of alpha a over a beneath b gives a*c + (1 - a)*b per channel, which reaches 0 (or 255)
  there only when a = 1 (to the half level of 8-bit rounding). `false` means the capture does not decide it: the
  value is a composite until a capture over a second background solves it.
Captures over the glass styles are measured too, but their title is drawn with the glass's vibrancy, so the
composite rule above does not describe them (no fact cites their colours).

Comparisons (`[compare.…]`): `differing_pixels`, the number of pixels of two crops of one cell size that differ in
any channel: mini against small (`compare.<appearance>.<style>.<state>.mini_small`), each state against normal
(`compare.<appearance>.<style>.<size>.<state>_normal`), and the styles UIKit draws alike
(`compare.<appearance>.<size>.<state>.<style>_<other>`).
"""
import collections
import sys
import tomllib
from pathlib import Path

from PIL import Image

PAIRS = [("borderless", "plain"), ("bordered", "gray"), ("bordered_tinted", "tinted"),
         ("bordered_prominent", "filled")]
SCALE = 3


def hexc(c):
    return "#%02x%02x%02x" % tuple(c[:3])


def opaque(colour, beneath):
    return any((colour[i] == 0 and beneath[i] > 0) or (colour[i] == 255 and beneath[i] < 255) for i in range(3))


def pixels(path):
    with Image.open(path) as im:
        rgb = im.convert("RGB")
        data = rgb.tobytes()
        return rgb.size, [tuple(data[i:i + 3]) for i in range(0, len(data), 3)]


def differing(a, b):
    return sum(1 for p, q in zip(a, b) if p != q)


def main():
    folder = Path(sys.argv[1])
    index = tomllib.loads((folder / "run1" / "index.toml").read_text())
    screens = {}
    cells = {}
    images = []
    for r in index["render"]:
        path = folder / "run1" / r["screen"]
        images.append(str(path))
        (w, h), px = pixels(path)
        key = (r["appearance"], r["style"], r["size"], r["state"])
        screens[key] = px
        x, y, bw, bh = (v * SCALE for v in r["button_frame_pt"])
        box = (round(x + bw * 0.15), round(y + bh * 0.2), round(x + bw * 0.85), round(y + bh * 0.8))
        area = [px[j * w + i] for j in range(box[1], box[3]) for i in range(box[0], box[2])]
        bezel = collections.Counter(area).most_common(1)[0][0]
        ink = [p for p in area if max(abs(p[i] - bezel[i]) for i in range(3)) > 40]
        text, count = collections.Counter(ink).most_common(1)[0] if ink else ((0, 0, 0), 0)
        background = px[4 * w + 4]
        cells[key] = {
            "image": str(path), "bezel": hexc(bezel), "background": hexc(background), "text": hexc(text),
            "text_pixels": count, "text_opaque": opaque(text, bezel), "bezel_opaque": opaque(bezel, background),
        }
    out = ["# Written by analysis/measure_buttons.py (its docstring says what each value is).", ""]
    out.append("input_image = [")
    out += [f'  "{p}",' for p in images]
    out.append("]")
    out.append("")
    for key, c in cells.items():
        out.append("[%s]" % ".".join(key))
        for k, v in c.items():
            out.append(f'{k} = "{v}"' if isinstance(v, str) else f"{k} = {str(v).lower()}")
        out.append("")
    appearances = sorted({k[0] for k in cells}, reverse=True)
    styles = list(dict.fromkeys(k[1] for k in cells))
    sizes = list(dict.fromkeys(k[2] for k in cells))
    states = list(dict.fromkeys(k[3] for k in cells))
    for a in appearances:
        for s in styles:
            for st in states:
                out.append(f"[compare.{a}.{s}.{st}]")
                out.append(f"mini_small = {differing(screens[(a, s, 'mini', st)], screens[(a, s, 'small', st)])}")
                out.append("")
            for z in sizes:
                out.append(f"[compare.{a}.{s}.{z}]")
                for st in states[1:]:
                    out.append(f"{st}_normal = {differing(screens[(a, s, z, st)], screens[(a, s, z, 'normal')])}")
                out.append("")
        for z in sizes:
            for st in states:
                out.append(f"[compare.{a}.{z}.{st}]")
                for s, o in PAIRS:
                    out.append(f"{s}_{o} = {differing(screens[(a, s, z, st)], screens[(a, o, z, st)])}")
                out.append("")
    (folder / "analysis.toml").write_text("\n".join(out))


if __name__ == "__main__":
    main()
