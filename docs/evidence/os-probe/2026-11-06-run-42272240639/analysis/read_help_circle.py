#!/usr/bin/env python3
"""Reads the help bezel's glyph and the circular bezel's circle from the os-probe button pass's layer readings
(MB-help: U145, U314, U614; MB-circle: U159 of the 0.8.0 measurement plan), as the facts' inputs: per control size,
state and appearance, the help glyph's bounds against the button's frame (the space round it, its size) and its
colour, and the circle's bounds against the circular button's frame. Written for the measurement campaign
(2026-11-07, spec S17 *`evidence`*: an analysis output computed from values a harness read).

Run from the repository root (Python 3.11+, the standard library only):

    python3 docs/evidence/os-probe/<run>/analysis/read_help_circle.py docs/evidence/os-probe/<run>/os-probe-macos-26

It reads `button-pass/probe.toml` under that image folder: the key mode's `[macos.button_pass.sheets.styles_b]`
(the help buttons, `<size>-help`) and `.styles_a` (`<size>-circular`), per appearance and state the
`layer.buttons.<case>` table the probe computed from the black and white captures (its `layer_method` says how):
`frame_px_xywh`, the button's frame in the capture; `bezel_px_xywh`, the bounds of the pixels of alpha at least half
the body's; `ink`, the bezel's pixels whose straight colour or alpha differs from the fill's by more than 48 of 255
in a channel, their bounds (`bounds_px_xywh`) and the one farthest from the fill (`farthest_from_fill`, the glyph's
colour where it covers a whole pixel); `fill`, `fill_mode` and `body_alpha`. The capture's pixels are points (the
screens were at a backing scale of 1, `window_backingScaleFactor`, checked here), y from the top. It writes
`help_circle.toml` into the image folder:

- `[help.<appearance>.<state>.<size>]`: `frame`, `glyph` (the ink's bounds) as `[x, y, w, h]` in points from the
  window's top left; `glyph_inset_tlbr`, the glyph's distance from the frame's top, left, bottom and right edges
  (`glyph.y - frame.y`, `glyph.x - frame.x`, `frame.y + frame.h - glyph.y - glyph.h`,
  `frame.x + frame.w - glyph.x - glyph.w`); `glyph_pixels`; `glyph_color` (`farthest_from_fill`); `fill`,
  `fill_mode`, `body_alpha` as the probe gives them.
- `[circle.<appearance>.<state>.<size>]`: `frame`, `circle` (the bezel's bounds) and `circle_margin_tlbr`, the
  circle's distance from the frame's edges, as above; `fill`.

A state whose table has no layer reading (the hovered and clicked states, captured over the window only) is left
out.

The probe's `ink` counts every bezel pixel far from the fill, so on macOS 15, whose help bezel draws a shaded rim, it
holds the rim as well as the glyph (its bounds are the circle's). The key mode's layer PNGs are not committed, so the
glyph is also read per pixel from the layer PNGs the run committed: the key-not-main, main-not-key and
inactive-with-Increase-Contrast modes' `<appearance>/styles_b/<state>-layer.png` and the red-accent focus mode's
`<appearance>/styles_b/unfocused-layer.png` (a window key and main, its buttons not focused). Per help button, of the
pixels whose centre lies within the bezel's inscribed circle less 1.5 px (the circle centred in `bezel_px_xywh`, its
radius half the smaller side), clear of the rim: `glyph_core`, the bounds of those whose straight colour or alpha
differs from the probe's `fill` by more than 48 of 255 in a channel (the probe's ink test), and `glyph_extent`, by
more than 8 (the glyph's antialiased edge too), each `[x, y, w, h]` with its insets from the frame (`*_inset_tlbr`)
and pixel count; `glyph_color`, the straight colour farthest from the fill (the largest channel difference).
These go in `[glyph.<mode>.<appearance>.<state>.<size>]`. The PNGs are decoded with the standard library (8-bit
RGBA, not interlaced, which the probe writes). `input` and `input_layers` name the files read, as repository paths.
"""

import math
import struct
import sys
import tomllib
import zlib
from pathlib import Path

SIZES = ("mini", "small", "regular", "large", "extraLarge")

# The modes whose styles_b layer PNGs are committed: folder, the states and their files.
LAYER_MODES = (
    ("key_not_main", "button-pass-key-not-main", ("normal", "pressed", "disabled", "disabled_pressed")),
    ("main_not_key", "button-pass-main-not-key", ("normal", "pressed", "disabled", "disabled_pressed")),
    ("inactive_increase_contrast_on", "button-pass-inactive-increase-contrast-on",
     ("normal", "pressed", "disabled", "disabled_pressed")),
    ("focus_accent_red", "button-pass-focus-accent-red", ("unfocused",)),
)
CORE_DIFF = 48
EXTENT_DIFF = 8
RIM_PX = 1.5


def read_png(path):
    """An 8-bit RGBA, non-interlaced PNG as (width, height, rows of bytes)."""
    data = path.read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise SystemExit(f"{path}: not a PNG")
    pos, idat, width = 8, b"", None
    while pos < len(data):
        length, kind = struct.unpack(">I4s", data[pos:pos + 8])
        body = data[pos + 8:pos + 8 + length]
        if kind == b"IHDR":
            width, height, depth, colour, _, _, interlace = struct.unpack(">IIBBBBB", body)
            if (depth, colour, interlace) != (8, 6, 0):
                raise SystemExit(f"{path}: depth {depth}, colour type {colour}, interlace {interlace}")
        elif kind == b"IDAT":
            idat += body
        pos += 12 + length
    raw = zlib.decompress(idat)
    stride = width * 4
    rows, prev, i = [], bytearray(stride), 0
    for _ in range(height):
        f, line = raw[i], bytearray(raw[i + 1:i + 1 + stride])
        i += 1 + stride
        for x in range(stride):
            a = line[x - 4] if x >= 4 else 0
            b = prev[x]
            c = prev[x - 4] if x >= 4 else 0
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
    return width, height, rows


def hex_rgba(px):
    return "#" + "".join(f"{v:02x}" for v in px)


def parse_hex(text):
    t = text.lstrip("#")
    return tuple(int(t[i:i + 2], 16) for i in range(0, 8, 2))


def glyph_reading(rows, button):
    """The help glyph inside the bezel's circle, clear of its rim, against the fill."""
    fill = parse_hex(button["fill"])
    frame = button["frame_px_xywh"]
    bx, by, bw, bh = button["bezel_px_xywh"]
    cx, cy, r = bx + bw / 2, by + bh / 2, min(bw, bh) / 2 - RIM_PX
    found = {"core": [], "extent": []}
    farthest, far_diff = None, -1
    for y in range(by, by + bh):
        row = rows[y]
        for x in range(bx, bx + bw):
            if math.hypot(x + 0.5 - cx, y + 0.5 - cy) > r:
                continue
            px = tuple(row[4 * x:4 * x + 4])
            diff = max(abs(p - q) for p, q in zip(px, fill))
            if diff > EXTENT_DIFF:
                found["extent"].append((x, y))
            if diff > CORE_DIFF:
                found["core"].append((x, y))
            if diff > far_diff:
                farthest, far_diff = px, diff
    out = {}
    for name, pts in found.items():
        if not pts:
            out[f"glyph_{name}"] = "none"
            continue
        xs, ys = [p[0] for p in pts], [p[1] for p in pts]
        box = [min(xs), min(ys), max(xs) - min(xs) + 1, max(ys) - min(ys) + 1]
        out[f"glyph_{name}"] = box
        out[f"glyph_{name}_inset_tlbr"] = insets(frame, box)
        out[f"glyph_{name}_pixels"] = len(pts)
    out["glyph_color"] = hex_rgba(farthest) if farthest else "none"
    out["fill"] = button["fill"]
    out["frame"] = frame
    out["bezel"] = button["bezel_px_xywh"]
    return out


def insets(outer, inner):
    """The distance of `inner` from `outer`'s top, left, bottom and right edges (both `[x, y, w, h]`, y down)."""
    ox, oy, ow, oh = outer
    ix, iy, iw, ih = inner
    return [iy - oy, ix - ox, (oy + oh) - (iy + ih), (ox + ow) - (ix + iw)]


def toml_value(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return repr(v)
    if isinstance(v, str):
        return '"' + v.replace("\\", "\\\\").replace('"', '\\"') + '"'
    if isinstance(v, list):
        return "[" + ", ".join(toml_value(x) for x in v) + "]"
    raise TypeError(type(v))


def emit(lines, path, table):
    scalars = {k: v for k, v in table.items() if not isinstance(v, dict)}
    subs = {k: v for k, v in table.items() if isinstance(v, dict)}
    if scalars:
        lines.append("")
        lines.append("[" + ".".join(f'"{p}"' if not p.replace("_", "").isalnum() else p for p in path) + "]")
        for k, v in scalars.items():
            lines.append(f"{k} = {toml_value(v)}")
    for k, v in subs.items():
        emit(lines, path + [k], v)


def main():
    if len(sys.argv) != 2:
        print(__doc__.split("\n\n")[1], file=sys.stderr)
        return 2
    image = Path(sys.argv[1])
    source = image / "button-pass" / "probe.toml"
    with source.open("rb") as f:
        probe = tomllib.load(f)
    bp = probe["macos"]["button_pass"]
    out = {"help": {}, "circle": {}}
    for kind, sheet_name, suffix in (("help", "styles_b", "help"), ("circle", "styles_a", "circular")):
        sheet = bp["sheets"][sheet_name]
        scale = sheet.get("window_backingScaleFactor")
        if scale != 1.0:
            raise SystemExit(f"{source}: {sheet_name} window_backingScaleFactor is {scale!r}, not 1")
        for appearance in ("aqua", "darkAqua"):
            for state, captures in sheet[appearance].items():
                if not isinstance(captures, dict) or "layer" not in captures:
                    continue
                buttons = captures["layer"].get("buttons", {})
                for size in SIZES:
                    b = buttons.get(f"{size}-{suffix}")
                    if not isinstance(b, dict):
                        continue
                    frame = b["frame_px_xywh"]
                    row = {"frame": frame}
                    if kind == "help":
                        ink = b.get("ink", {})
                        if "bounds_px_xywh" in ink:
                            glyph = ink["bounds_px_xywh"]
                            row["glyph"] = glyph
                            row["glyph_inset_tlbr"] = insets(frame, glyph)
                            row["glyph_pixels"] = ink["pixels"]
                            row["glyph_color"] = ink["farthest_from_fill"]
                        else:
                            row["glyph"] = "none: no ink pixels"
                        for key in ("fill", "fill_mode", "body_alpha"):
                            if key in b:
                                row[key] = b[key]
                    else:
                        circle = b["bezel_px_xywh"]
                        row["circle"] = circle
                        row["circle_margin_tlbr"] = insets(frame, circle)
                        if "fill" in b:
                            row["fill"] = b["fill"]
                    out[kind].setdefault(appearance, {}).setdefault(state, {})[size] = row
    layers_read = []
    glyph = {}
    for mode, folder, states in LAYER_MODES:
        mode_toml = image / folder / "probe.toml"
        if not mode_toml.exists():
            continue
        with mode_toml.open("rb") as f:
            sheet = tomllib.load(f)["macos"]["button_pass"]["sheets"]["styles_b"]
        for appearance in ("aqua", "darkAqua"):
            for state in states:
                png = image / folder / appearance / "styles_b" / f"{state}-layer.png"
                captures = sheet.get(appearance, {}).get(state)
                if not png.exists() or not isinstance(captures, dict) or "layer" not in captures:
                    continue
                _, _, rows = read_png(png)
                layers_read.append(png.as_posix())
                buttons = captures["layer"]["buttons"]
                for size in SIZES:
                    b = buttons.get(f"{size}-help")
                    if isinstance(b, dict) and "bezel_px_xywh" in b and "fill" in b:
                        glyph.setdefault(mode, {}).setdefault(appearance, {}).setdefault(state, {})[size] = \
                            glyph_reading(rows, b)
    lines = [
        "# The help bezel's glyph and the circular bezel's circle, read from the button pass's layer readings by",
        "# analysis/read_help_circle.py (its docstring says how); points from the window's top left, y down.",
        f"input = {toml_value(source.as_posix())}",
        f"input_layers = {toml_value(layers_read)}",
    ]
    emit(lines, ["help"], out["help"])
    emit(lines, ["circle"], out["circle"])
    emit(lines, ["glyph"], glyph)
    target = image / "help_circle.toml"
    target.write_text("\n".join(lines) + "\n")
    with target.open("rb") as f:
        tomllib.load(f)
    print(f"wrote {target}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
