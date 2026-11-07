#!/usr/bin/env python3
"""Reads Core Animation's colour interpolation from the os-probe colours pass (`--colors`, `[macos.colors.color_space]`;
U158 of the 0.8.0 measurement plan): per square the captured colour, against what each way of interpolating would
draw. Written for the measurement campaign (2026-11-07, spec S17 *`evidence`*: an analysis output computed from
values a harness captured).

Run from the repository root (Python 3.11+, the standard library only):

    python3 docs/evidence/os-probe/<run>/analysis/read_color_space.py docs/evidence/os-probe/<run>/os-probe-macos-26/colors

It reads `probe.toml` in that folder (`squares_from_window_top_left`: each square's case, fraction, backdrop, from and
to colours and rect in the capture's points from the window's top left) and `color_space/<appearance>.png` (the
window captured with `screencapture -o -l`; the screens were at a backing scale of 1, so a point is a pixel). Per
square it averages the bytes of the square's inner pixels (the square less 3 px on each side) and writes
`color_space.toml` into the folder, `[color_space.<appearance>."<square>"]`:

- `captured`: the mean `[r, g, b]` bytes, rounded to a tenth;
- `straight_srgb`, `premultiplied_srgb`, `straight_linear`, `premultiplied_linear`: the bytes a square would show if
  the animation's value at the square's fraction were the from and to colours mixed at that fraction (straight:
  colour and alpha mixed separately; premultiplied: the colour times its alpha mixed, then divided by the mixed alpha)
  in sRGB's encoded values or in linear light (the sRGB transfer function undone before, redone after), composited
  over the square's backdrop in sRGB's encoded values (source-over); and `nearest`, the one with the smallest largest
  channel difference from `captured`, with that difference.

The references (static squares) show how a known sRGB colour comes out of the capture; their `straight_srgb` is that
colour composited over the backdrop. `input` names the files read.
"""

import struct
import sys
import tomllib
import zlib
from pathlib import Path

INSET = 3


def read_png(path):
    data = path.read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise SystemExit(f"{path}: not a PNG")
    pos, idat = 8, b""
    while pos < len(data):
        length, kind = struct.unpack(">I4s", data[pos:pos + 8])
        body = data[pos + 8:pos + 8 + length]
        if kind == b"IHDR":
            width, height, depth, colour, _, _, interlace = struct.unpack(">IIBBBBB", body)
            if depth != 8 or colour not in (2, 6) or interlace != 0:
                raise SystemExit(f"{path}: depth {depth}, colour type {colour}, interlace {interlace}")
            bpp = 4 if colour == 6 else 3
        elif kind == b"IDAT":
            idat += body
        pos += 12 + length
    raw = zlib.decompress(idat)
    stride = width * bpp
    rows, prev, i = [], bytearray(stride), 0
    for _ in range(height):
        f, line = raw[i], bytearray(raw[i + 1:i + 1 + stride])
        i += 1 + stride
        for x in range(stride):
            a = line[x - bpp] if x >= bpp else 0
            b = prev[x]
            c = prev[x - bpp] if x >= bpp else 0
            if f == 1:
                line[x] = (line[x] + a) & 255
            elif f == 2:
                line[x] = (line[x] + b) & 255
            elif f == 3:
                line[x] = (line[x] + (a + b) // 2) & 255
            elif f == 4:
                pa, pb, pc = abs(b - c), abs(a - c), abs(a + b - 2 * c)
                line[x] = (line[x] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)) & 255
        rows.append(bytes(line))
        prev = line
    return width, height, bpp, rows


def to_linear(v):
    return v / 12.92 if v <= 0.04045 else ((v + 0.055) / 1.055) ** 2.4


def to_srgb(v):
    v = min(max(v, 0.0), 1.0)
    return v * 12.92 if v <= 0.0031308 else 1.055 * v ** (1 / 2.4) - 0.055


def parse_hex(text):
    t = text.lstrip("#")
    return [int(t[i:i + 2], 16) / 255 for i in range(0, 8, 2)]


def mix(c0, c1, f, premultiplied, linear):
    rgb0, a0 = c0[:3], c0[3]
    rgb1, a1 = c1[:3], c1[3]
    if linear:
        rgb0 = [to_linear(v) for v in rgb0]
        rgb1 = [to_linear(v) for v in rgb1]
    a = a0 + (a1 - a0) * f
    if premultiplied:
        p = [x0 * a0 + (x1 * a1 - x0 * a0) * f for x0, x1 in zip(rgb0, rgb1)]
        rgb = [v / a if a > 0 else 0.0 for v in p]
    else:
        rgb = [x0 + (x1 - x0) * f for x0, x1 in zip(rgb0, rgb1)]
    if linear:
        rgb = [to_srgb(v) for v in rgb]
    return rgb, a


def over(rgb, a, backdrop):
    return [round((c * a + backdrop * (1 - a)) * 255, 1) for c in rgb]


def toml_value(v):
    if isinstance(v, (int, float)):
        return repr(v)
    if isinstance(v, str):
        return '"' + v.replace("\\", "\\\\").replace('"', '\\"') + '"'
    if isinstance(v, list):
        return "[" + ", ".join(toml_value(x) for x in v) + "]"
    raise TypeError(type(v))


def main():
    if len(sys.argv) != 2:
        print(__doc__.split("\n\n")[1], file=sys.stderr)
        return 2
    folder = Path(sys.argv[1])
    with (folder / "probe.toml").open("rb") as f:
        cs = tomllib.load(f)["macos"]["colors"]["color_space"]
    lines = ["# Core Animation's colour interpolation read from the colours pass's captures by",
             "# analysis/read_color_space.py (its docstring says how).",
             f"input = {toml_value((folder / 'probe.toml').as_posix())}"]
    for appearance in ("aqua", "darkAqua"):
        png = folder / "color_space" / f"{appearance}.png"
        if not png.exists():
            continue
        width, height, bpp, rows = read_png(png)
        lines.append(f"input_{appearance} = {toml_value(png.as_posix())}")
        for text in cs["squares_from_window_top_left"]:
            name, rest = text.split(": from ", 1)
            from_hex, rest = rest.split(" to ", 1)
            to_hex, rest = rest.split(", x ", 1)
            nums = rest.replace(" y ", " ").replace(" w ", " ").replace(" h ", " ").split()
            x, y, w, h = (round(float(n)) for n in nums)
            backdrop = 0.0 if name.endswith("over_black") else 1.0
            fraction = float(name.split(" at ")[1].split(" ")[0]) if " at " in name else 0.0
            px = []
            for yy in range(y + INSET, y + h - INSET):
                row = rows[yy]
                for xx in range(x + INSET, x + w - INSET):
                    px.append(row[bpp * xx:bpp * xx + 3])
            captured = [round(sum(p[i] for p in px) / len(px), 1) for i in range(3)]
            c0, c1 = parse_hex(from_hex), parse_hex(to_hex)
            lines.append("")
            lines.append(f'[color_space.{appearance}."{name}"]')
            lines.append(f"captured = {toml_value(captured)}")
            best = None
            for key, pre, lin in (("straight_srgb", False, False), ("premultiplied_srgb", True, False),
                                  ("straight_linear", False, True), ("premultiplied_linear", True, True)):
                rgb, a = mix(c0, c1, fraction, pre, lin)
                expect = over(rgb, a, backdrop)
                lines.append(f"{key} = {toml_value(expect)}")
                diff = max(abs(p - q) for p, q in zip(captured, expect))
                if best is None or diff < best[1]:
                    best = (key, diff)
            lines.append(f"nearest = {toml_value(best[0])}")
            lines.append(f"nearest_max_byte_difference = {round(best[1], 1)!r}")
    target = folder / "color_space.toml"
    target.write_text("\n".join(lines) + "\n")
    with target.open("rb") as f:
        tomllib.load(f)
    print(f"wrote {target}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
