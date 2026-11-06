#!/usr/bin/env python3
"""Reads the push buttons' pixels from the os-probe button pass of run 40969040283 (macOS 26.6.2) and writes the
values the macOS facts cite (docs/facts/macos/button.toml, state-models.toml), as S17's analysis output.

Run from the repository root (Python 3.11+, the standard library only):

    python3 docs/evidence/os-probe/2026-10-29-run-40969040283/analysis/measure_button_pass.py > \
        docs/evidence/os-probe/2026-10-29-run-40969040283/os-probe-macos-26/button-pass/analysis.toml

It exits 1, writing nothing to stdout, when a region it reads is not as the method says (a fill that is not one
colour, a scale that is not 1).

What it reads, all under os-probe-macos-26/ of the run:

- button-pass/probe.toml: each button's frame (`in_window_frame_from_top_left_xywh`, points from the window
  frame's top left), the captures' size and the window's backing scale (1: a point is a pixel);
- button-pass/<appearance>/{normal,pressed,disabled}.png: the window captures (screencapture), whose bytes are sRGB:
  the same run's key-window calibration strip reads every sRGB patch exactly in the raw bytes
  (probe.toml, `[macos.key_window.<appearance>.calibration.*]`, `exact = true`), so the bytes are taken as they are
  and the PNG's embedded display profile is not applied;
- button-pass/<appearance>/clicked-regular-{plain,default}.png: the captures taken during a real mouse press;
- renders/<appearance>/push_button-regular{,-disabled}.png and the same files under reduce-transparency-off/: the
  offscreen renders of the main pass (Reduce Transparency on, the image's setting) and of the pass with it off;
- probe.toml: the push button cell's title rect per control size (`[macos.controls.<appearance>.<size>.
  push_button.cell]`).

What it writes, per appearance (`aqua`, `darkAqua`):

- `window_background`: the pixel 8 px left of the regular plain button, at its vertical centre;
- `<state>.<button>.fill`: the colour of the 4 x 7 px block from 3 to 6 px right of the button frame's left edge and
  from 3 px above to 3 px below its vertical centre (left of the label, clear of the corners at every size), which
  must be one colour (`fill_pixels`, all equal);
- `<state>.<button>.stroke`: "absent" when every pixel of the middle third of the frame's top and bottom pixel rows
  is the fill drawn at some coverage over `window_background` (coverage(): an antialiased edge, each channel within
  1.5 levels of the blend; `edge_min_coverage` the lowest), so the bezel draws no line of another colour along its
  edge; "drawn" otherwise;
- `<state>.<button>.shadow`: "absent" when every pixel of the ring 1 to 6 px outside the frame equals
  `window_background`: nothing is drawn outside the bezel;
- `all_states.stroke`, `all_states.shadow`: "absent" when every button in every state above is "absent";
- `clicked.<button>`: `differing_pixels` between the clicked capture and pressed.png inside the frame grown by 6 px,
  and `equals_pressed`;
- `offscreen.<file>.reduce_transparency_independent`: the main pass's offscreen render and the
  reduce-transparency-off pass's are pixel for pixel the same;
- `geometry.<size>`: `padding_left` (the title rect's x) and `padding_right` (the bounds' width less the title
  rect's x and width), in points, from probe.toml.
"""

import struct
import sys
import tomllib
import zlib
from pathlib import Path

RUN = "docs/evidence/os-probe/2026-10-29-run-40969040283/os-probe-macos-26"
APPEARANCES = ("aqua", "darkAqua")
STATES = ("normal", "pressed", "disabled")
CLICKED = ("regular-plain", "regular-default")
OFFSCREEN = ("push_button-regular.png", "push_button-regular-disabled.png")
SIZES = ("mini", "small", "regular", "large")


class Bad(Exception):
    pass


def read_png(path: Path):
    """(width, height, rows of (r, g, b, a) tuples) of an 8-bit, non-interlaced RGB or RGBA PNG."""
    data = path.read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise Bad(f"{path}: not a PNG")
    pos, idat, ihdr = 8, b"", None
    while pos < len(data):
        (length,) = struct.unpack(">I", data[pos:pos + 4])
        kind = data[pos + 4:pos + 8]
        body = data[pos + 8:pos + 8 + length]
        pos += 12 + length
        if kind == b"IHDR":
            ihdr = struct.unpack(">IIBBBBB", body)
        elif kind == b"IDAT":
            idat += body
        elif kind == b"IEND":
            break
    if ihdr is None:
        raise Bad(f"{path}: no IHDR")
    width, height, depth, colour, _, _, interlace = ihdr
    if depth != 8 or colour not in (2, 6) or interlace != 0:
        raise Bad(f"{path}: depth {depth}, colour type {colour}, interlace {interlace}: not read here")
    bpp = 4 if colour == 6 else 3
    raw = zlib.decompress(idat)
    stride = width * bpp
    rows, prev, i = [], bytearray(stride), 0
    for _ in range(height):
        f = raw[i]
        line = bytearray(raw[i + 1:i + 1 + stride])
        i += 1 + stride
        for x in range(stride):
            a = line[x - bpp] if x >= bpp else 0
            b = prev[x]
            c = prev[x - bpp] if x >= bpp else 0
            if f == 1:
                line[x] = (line[x] + a) & 0xFF
            elif f == 2:
                line[x] = (line[x] + b) & 0xFF
            elif f == 3:
                line[x] = (line[x] + (a + b) // 2) & 0xFF
            elif f == 4:
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                pred = a if pa <= pb and pa <= pc else (b if pb <= pc else c)
                line[x] = (line[x] + pred) & 0xFF
            elif f != 0:
                raise Bad(f"{path}: filter {f}")
        rows.append([tuple(line[x:x + bpp]) + ((255,) if bpp == 3 else ()) for x in range(0, stride, bpp)])
        prev = line
    return width, height, rows


def coverage(p, fill, bg):
    """The fraction t in [0, 1] for which p is the fill drawn at coverage t over bg, every channel within 1.5
    levels of `bg + t * (fill - bg)` (an antialiased edge pixel), or None when p is no such blend (another colour)."""
    c = max(range(3), key=lambda i: abs(fill[i] - bg[i]))
    if fill[c] == bg[c]:
        return 1.0 if p[:3] == fill[:3] else None
    t = (p[c] - bg[c]) / (fill[c] - bg[c])
    if not -0.01 <= t <= 1.01:
        return None
    if any(abs(p[i] - (bg[i] + t * (fill[i] - bg[i]))) > 1.5 for i in range(3)):
        return None
    return max(0.0, min(1.0, t))


def hexa(px) -> str:
    return "#%02x%02x%02x%02x" % px


def main() -> int:
    root = Path(".")
    run = root / RUN
    bp = tomllib.loads((run / "button-pass/probe.toml").read_text(encoding="utf-8"))["macos"]["button_pass"]
    if bp["window_backingScaleFactor"] != 1.0:
        raise Bad("the window's backing scale is not 1")
    buttons = {name: [int(v) for v in b["in_window_frame_from_top_left_xywh"]] for name, b in bp["buttons"].items()}
    out, images = [], []

    def image(rel):
        images.append(f"{RUN}/{rel}")
        return read_png(run / rel)

    for app in APPEARANCES:
        caps = {}
        for st in STATES + tuple(f"clicked-{c}" for c in CLICKED):
            w, h, px = image(f"button-pass/{app}/{st}.png")
            want = bp[app]["normal"]["pixels_wh"]
            if [w, h] != want:
                raise Bad(f"{app}/{st}.png is {w} x {h}, not {want}")
            caps[st] = px
        x, y, w, h = buttons["regular-plain"]
        bg = caps["normal"][y + h // 2][x - 8]
        out.append(f"[{app}]\nwindow_background = \"{hexa(bg)}\"\n")
        all_absent = {"stroke": True, "shadow": True}
        for st in STATES:
            px = caps[st]
            for name, (x, y, w, h) in buttons.items():
                cy = y + h // 2
                block = [px[yy][xx] for yy in range(cy - 3, cy + 4) for xx in range(x + 3, x + 7)]
                if len(set(block)) != 1:
                    raise Bad(f"{app} {st} {name}: the fill block is not one colour: {sorted(set(block))}")
                fill = block[0]
                third = w // 3
                edge = [px[yy][xx] for yy in (y, y + h - 1) for xx in range(x + third, x + w - third)]
                cover = [coverage(p, fill, bg) for p in edge]
                stroke = "absent" if all(c is not None for c in cover) else "drawn"
                ring = [px[yy][xx] for yy in range(y - 6, y + h + 6) for xx in range(x - 6, x + w + 6)
                        if not (x <= xx < x + w and y <= yy < y + h)]
                shadow = "absent" if all(p == bg for p in ring) else "drawn"
                all_absent["stroke"] &= stroke == "absent"
                all_absent["shadow"] &= shadow == "absent"
                low = min((c for c in cover if c is not None), default=None)
                out.append(f"[{app}.{st}.{name}]\nfill = \"{hexa(fill)}\"\nfill_pixels = {len(block)}\n"
                           f"stroke = \"{stroke}\"\nedge_pixels = {len(edge)}\n"
                           + (f"edge_min_coverage = {low:.3f}\n" if low is not None else "")
                           + f"shadow = \"{shadow}\"\nring_pixels = {len(ring)}\n")
        out.append(f"[{app}.all_states]\nstroke = \"{'absent' if all_absent['stroke'] else 'drawn'}\"\n"
                   f"shadow = \"{'absent' if all_absent['shadow'] else 'drawn'}\"\n")
        for c in CLICKED:
            x, y, w, h = buttons[c]
            a, b = caps[f"clicked-{c}"], caps["pressed"]
            diff = sum(1 for yy in range(y - 6, y + h + 6) for xx in range(x - 6, x + w + 6) if a[yy][xx] != b[yy][xx])
            out.append(f"[{app}.clicked.{c}]\ndiffering_pixels = {diff}\nequals_pressed = {str(diff == 0).lower()}\n")
        for f in OFFSCREEN:
            on = image(f"renders/{app}/{f}")
            off = image(f"reduce-transparency-off/renders/{app}/{f}")
            key = f.removesuffix(".png")
            out.append(f"[{app}.offscreen.{key}]\nreduce_transparency_independent = {str(on == off).lower()}\n")

    probe = tomllib.loads((run / "probe.toml").read_text(encoding="utf-8"))["macos"]["controls"]
    for size in SIZES:
        vals = set()
        for app in APPEARANCES:
            cell = probe[app][size]["push_button"]["cell"]
            bx, by, bw, bh = cell["bounds_xywh"]
            tx, ty, tw, th = cell["titleRectForBounds_xywh"]
            vals.add((tx, bw - tx - tw))
        if len(vals) != 1:
            raise Bad(f"{size}: the appearances' title insets differ: {vals}")
        left, right = vals.pop()
        out.append(f"[geometry.{size}]\npadding_left = {left}\npadding_right = {right}\n")

    head = ("# Written by ../../analysis/measure_button_pass.py (its docstring is the method); do not edit.\n"
            "input_image = [\n" + "".join(f"  \"{p}\",\n" for p in images) + "]\n\n")
    sys.stdout.write(head + "\n".join(out))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Bad as err:
        print(f"error: {err}", file=sys.stderr)
        sys.exit(1)
