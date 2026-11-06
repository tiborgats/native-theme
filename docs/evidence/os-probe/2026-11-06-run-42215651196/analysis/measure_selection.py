#!/usr/bin/env python3
"""Measures the macOS selected table row of the os-probe pass P4 (`[macos.selection]`) as the facts' values: the
colours AppKit draws text in over an emphasized selection, solved from two selection backgrounds, and whether the
selection draws an outline. The facts lane wrote it (2026-11-02, spec S17 *`evidence`*: an analysis output for a value
computed from read values).

Run from the repository root (Python 3.11+, the standard library only):

    python3 docs/evidence/os-probe/<run>/analysis/measure_selection.py docs/evidence/os-probe/<run>/os-probe-macos-26

It reads, under that image folder, `probe.toml` (the image's accent), `accent-red/probe.toml` (`AppleAccentColor` 0),
`reduce-transparency-off/probe.toml` and `increase-contrast-on/probe.toml` (each `[macos.selection]`: the method,
the geometry, the sample points and, per appearance and condition, the samples' `raw` pixels and the capture's file),
and each pass's captures `renders/selection/<appearance>-<condition>.png` (8-bit RGBA, one pixel a point: the
probe's window backing scale is 1). It writes `selection.toml` into the image folder:

- `[selection.check.<pass>.<appearance>.<condition>]`: `samples_match`, true where every sample's `raw` pixel of the
  probe equals the capture's pixel at its `pixel_xy`, so the samples and the captures read the same image.
- `[selection.<appearance>]`, the emphasized row (`condition` emphasized: the table first responder, the row view
  `isEmphasized`) of the image's and the red accent's passes, whose selections are two backgrounds, B1 and B2 (each
  column's `r1_<column>_background` sample). A glyph drawn in a colour F of alpha a composites to C = a F + (1 - a)
  B, so per channel a = 1 - (C1 - C2) / (B1 - B2), read on the channels where |B1 - B2| >= 100 (two levels of
  quantisation then move a by at most 0.02), and a is their mean; then F = (C1 - (1 - a) B1) / a per channel, clamped
  to 0..255 and rounded, and the alpha byte round(255 a). The glyph `C` is the column's `r1_<column>_glyph` sample
  (the centre of its middle full block, U+2588). Written per column: `foreground_inactive` (secondaryLabelColor),
  `disabled_foreground` (tertiaryLabelColor), `link` (linkColor), as `#rrggbbaa`; and `[…solve.<column>]` with
  each channel's a.
- `[selection.edges.<pass>.<appearance>.<condition>]`: the selection's outline. The selected row's rect is
  `selected_row_window_xywh` (window points, origin at the frame's bottom left, as `rectOfRow:` in the window), in
  capture rows top = H - (y + h), bottom = H - y - 1 for the capture's height H. Read: every pixel of the bands of
  rows top - 3 .. top + 3 and bottom - 3 .. bottom + 3 across the scroll view's width (`window_frame_xywh` less the
  scroll view's 20-point margins), and of the columns 3 either side of the drawn selection's left and right ends on
  the row's middle row, outside every text field's frame grown by one pixel (`geometry` `*_field_window_xywh`), each
  tested against the segment from O, the table's background (the capture's pixel at the `r0_labelColor_background`
  sample point), to S, the selection's fill (the `r1_labelColor_background` sample): a pixel whose distance from that
  segment is more than 3 levels in a channel is a stroke pixel (`stroke_pixels`, `max_off_segment`). A drawn outline
  or separator at the selection's edge adds such pixels; the selection's own antialiased edge does not.
  `border = "absent"` where no pass, appearance or condition has a stroke pixel (`[selection.border]`).

`input_image` names every file read, as repository paths.
"""

import struct
import sys
import tomllib
import zlib
from pathlib import Path

PASSES = (("main", ""), ("accent_red", "accent-red/"), ("reduce_transparency_off", "reduce-transparency-off/"),
          ("increase_contrast_on", "increase-contrast-on/"))
COLUMNS = (("foreground_inactive", "secondaryLabelColor"), ("disabled_foreground", "tertiaryLabelColor"),
           ("link", "linkColor"))
MARGIN = 20  # the scroll view's origin in the window, `[macos.selection] method`: "an NSScrollView at [20, 20, …]"


def read_png(path: Path):
    data = path.read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise ValueError(f"{path}: not a PNG")
    pos, idat, ihdr = 8, b"", None
    while pos < len(data):
        (length,) = struct.unpack(">I", data[pos:pos + 4])
        kind, body = data[pos + 4:pos + 8], data[pos + 8:pos + 8 + length]
        pos += 12 + length
        if kind == b"IHDR":
            ihdr = struct.unpack(">IIBBBBB", body)
        elif kind == b"IDAT":
            idat += body
        elif kind == b"IEND":
            break
    width, height, depth, colour, _, _, interlace = ihdr
    if depth != 8 or colour not in (2, 6) or interlace != 0:
        raise ValueError(f"{path}: not an 8-bit non-interlaced RGB(A) PNG")
    bpp = 4 if colour == 6 else 3
    raw, stride = zlib.decompress(idat), width * bpp
    rows, prev, i = [], bytearray(stride), 0
    for _ in range(height):
        f, line = raw[i], bytearray(raw[i + 1:i + 1 + stride])
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
                line[x] = (line[x] + (a if pa <= pb and pa <= pc else (b if pb <= pc else c))) & 0xFF
            elif f != 0:
                raise ValueError(f"{path}: filter {f}")
        rows.append([tuple(line[x:x + bpp]) + ((255,) if bpp == 3 else ()) for x in range(0, stride, bpp)])
        prev = line
    return width, height, rows


def rgba(s):
    s = s.lstrip("#")
    return tuple(int(s[k:k + 2], 16) for k in range(0, 8, 2))


def hexa(c):
    return "#" + "".join(f"{v:02x}" for v in c)


def off_segment(p, o, s):
    """The largest channel distance of p from the segment o..s."""
    d = [s[k] - o[k] for k in range(3)]
    n = sum(v * v for v in d)
    t = 0.0 if n == 0 else max(0.0, min(1.0, sum((p[k] - o[k]) * d[k] for k in range(3)) / n))
    return max(abs(p[k] - (o[k] + t * d[k])) for k in range(3))


def solve(c1, b1, c2, b2):
    alphas = [1 - (c1[k] - c2[k]) / (b1[k] - b2[k]) for k in range(3) if abs(b1[k] - b2[k]) >= 100]
    a = sum(alphas) / len(alphas)
    f = [max(0, min(255, round((c1[k] - (1 - a) * b1[k]) / a))) for k in range(3)]
    return tuple(f) + (round(255 * a),), alphas


def main():
    if len(sys.argv) != 2:
        print(__doc__.split("\n\n")[1], file=sys.stderr)
        return 2
    image = Path(sys.argv[1])
    inputs, sel = [], {}
    for name, rel in PASSES:
        inputs.append(f"{rel}probe.toml")
        sel[name] = tomllib.loads((image / f"{rel}probe.toml").read_text())["macos"]["selection"]
    out = {"check": {}, "edges": {}}
    strokes = 0
    for name, rel in PASSES:
        s = sel[name]
        geo = s["geometry"]
        fields = [[round(c) for c in v] for k, v in geo.items() if k.endswith("_field_window_xywh")]
        rx, ry, rw, rh = (round(v) for v in geo["selected_row_window_xywh"])
        for appearance in ("aqua", "darkAqua"):
            for condition in ("emphasized", "unemphasized"):
                run = s[appearance][condition]
                inputs.append(f"{rel}{run['file']}")
                w, h, rows = read_png(image / f"{rel}{run['file']}")
                match = all(rows[v["pixel_xy"][1]][v["pixel_xy"][0]] == rgba(v["raw"]) for v in run["samples"].values())
                out["check"].setdefault(name, {}).setdefault(appearance, {})[condition] = {"samples_match": match}

                def in_field(x, y):
                    for fx, fy, fw, fh in fields:
                        top = h - (fy + fh)
                        if fx - 1 <= x <= fx + fw and top - 1 <= y <= top + fh:
                            return True
                    return False

                o = rgba(run["samples"]["r0_labelColor_background"]["raw"])
                sfill = rgba(run["samples"]["r1_labelColor_background"]["raw"])
                top, bottom = h - (ry + rh), h - ry - 1
                x0, x1 = MARGIN, w - MARGIN
                pts = [(x, y) for y in list(range(top - 3, top + 4)) + list(range(bottom - 3, bottom + 4))
                       for x in range(x0, x1)]
                mid = (top + bottom) // 2
                drawn = [x for x in range(x0, x1) if rows[mid][x][:3] != o[:3]]
                if drawn:
                    for end in (drawn[0], drawn[-1]):
                        pts += [(x, y) for x in range(end - 3, end + 4) for y in range(top, bottom + 1)]
                pts = [(x, y) for x, y in set(pts) if not in_field(x, y)]
                offs = [off_segment(rows[y][x], o, sfill) for x, y in pts]
                n = sum(1 for v in offs if v > 3)
                strokes += n
                out["edges"].setdefault(name, {}).setdefault(appearance, {})[condition] = {
                    "pixels": len(pts), "stroke_pixels": n, "max_off_segment": round(max(offs), 2),
                    "outside": hexa(o), "selection": hexa(sfill)}
    for appearance in ("aqua", "darkAqua"):
        m, r = sel["main"][appearance]["emphasized"]["samples"], sel["accent_red"][appearance]["emphasized"]["samples"]
        res, sol = {}, {}
        for key, column in COLUMNS:
            c, a = solve(rgba(m[f"r1_{column}_glyph"]["raw"]), rgba(m[f"r1_{column}_background"]["raw"]),
                         rgba(r[f"r1_{column}_glyph"]["raw"]), rgba(r[f"r1_{column}_background"]["raw"]))
            res[key] = hexa(c)
            sol[column] = {"channel_alphas": [round(v, 4) for v in a]}
        res["solve"] = sol
        out[appearance] = res
    if strokes == 0:
        out["border"] = {"border": "absent"}
    lines = ["# Output of analysis/measure_selection.py (its docstring is the method), file selection.toml.",
             "# input_image: every file it reads, as a repository path (spec S17 *`evidence`*).",
             "input_image = [" + ", ".join(f'"{image.as_posix()}/{i}"' for i in inputs) + "]"]
    emit(lines, ["selection"], out)
    p = image / "selection.toml"
    p.write_text("\n".join(lines) + "\n")
    tomllib.loads(p.read_text())
    return 0


def val(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return repr(v)
    if isinstance(v, str):
        return f'"{v}"'
    return "[" + ", ".join(val(x) for x in v) + "]"


def emit(lines, path, table):
    scalars = {k: v for k, v in table.items() if not isinstance(v, dict)}
    if scalars:
        lines.append(f"\n[{'.'.join(path)}]")
        lines += [f"{k} = {val(v)}" for k, v in scalars.items()]
    for k, v in table.items():
        if isinstance(v, dict):
            emit(lines, path + [k], v)


if __name__ == "__main__":
    sys.exit(main())
