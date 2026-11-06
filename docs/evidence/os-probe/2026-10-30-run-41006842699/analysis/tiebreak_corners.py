#!/usr/bin/env python3
"""The tie-break's corner measurement of the os-probe button pass of run 41006842699 (macOS 26.6.2): the corner radii
the macOS facts (docs/facts/macos/button.toml) cite, replacing the per-row circle fit of measure_button_layers.py,
which the tie-break between the lane and the verifier found unstable (fits of 7.5 to 9.25 px on a 13 px high capsule,
a different radius per corner of one symmetric button, rms up to 2.6 px).

Run from the repository root (Python 3.11+, the standard library only), after measure_button_layers.py, once per
image folder (IMAGE as measure_button_layers.py takes it: `os-probe-macos-26`, the default, or `os-probe-macos-15`,
which os-probe-macos-26/ below stands for):

    python3 docs/evidence/os-probe/2026-10-30-run-41006842699/analysis/tiebreak_corners.py [IMAGE]

It reads, under os-probe-macos-26/ of the run, the analysis output `layers-<pass>-<sheet>.toml` (each button's
`drawn_box` and `fill`, measure_button_layers.py's) and the layers `<pass dir>/<appearance>/[<sheet>/]<state>-layer.png`
they were read from (each pixel's straight colour and alpha, solved by the probe over black and white backdrops), and
writes `tiebreak-corners.toml`, per `<pass>.<sheet>.<appearance>.<state>.<button>` where `fill` is a colour:

- `box`: the drawn box (x, y, w, h) as measure_button_layers.py read it;
- `read_from = "outline"` where the button draws no fill but a 1 px line of one colour round its box
  (measure_button_layers.py's `stroke_<side>` each `{ width = 1 }` with one `edge` colour: the toolbar bezel's
  normal and disabled looks): the line's colour stands for the fill below, and in each row of the corner square every
  pixel inward of the line's first pixel of at least half its alpha counts as covered (the bezel's interior), so the
  fit reads the outline's outer edge;
- per corner (`top_left`, `top_right`, `bottom_right`, `bottom_left`) a table: the corner square of side
  n = min(w, h) // 2 at that corner of the box; each pixel's coverage c = min(alpha / fill alpha, 1); pixels of the
  label or an image left out (`masked`: a straight colour more than 8 levels from the fill's in a channel at an
  alpha of at least half the fill's, or an alpha more than 3/255 above the fill's); `fit_px`, the radius r, in
  0.05 px steps from 0 to n, of the circular corner whose coverage of each pixel (16 x 16 samples) fits the
  unmasked pixels' c best by least squares, and `rms` (in coverage, 0..1); `area_px`, the radius of the circle
  whose cut-off area (1 - pi/4) r^2 equals the unmasked pixels' summed missing coverage (a second reading of the
  same pixels);
- `radius`: the value as a fact spells it, only where every corner's rms is at most 0.06 and the four corners' fits
  spread by at most 1 px (the fit passes), else "unfit" (the corner is no circle the pixels show: a gradient, a stroke
  or a shadow over it; or the four corners are not one radius, which the mean would hide: before the tie-break of
  macos-monterey's slice a dark disabled shadowless square bezel's top fits of 4.05 px and bottom fits of 0 px, its
  top edge line masked, were read as one 2 px corner): where it passes,
  `{ full = { maximum_px = "absent" } }` (a capsule or a circle) where the fit is within 0.75 px of half the box's
  smaller side on every corner, else the four fits' mean rounded to 0.25 px (`"<r>px"`, `"0px"` under 0.375 px). The
  capsule reading needs the fit to pass as any radius does: before the tie-break of macos-monterey's slice it did
  not, and read a 46 x 16 px mini push bezel of macOS 15 (rms 0.2-0.56, its edge line and shadow over the corner)
  as a capsule;
- `spread_px`: the largest difference between the four corners' fits.

The precision the method supports, from the readings themselves: the four corners of one symmetric button agree to
0.05-0.15 px, the fit and the area reading of one corner to 0.1-0.15 px; the circular model fits the opaque bezels to
an rms of about 0.02 (2 % coverage) and the translucent ones to about 0.03 (their alpha's 8-bit steps are 1/19 of the
fill's), so a circular and a continuous corner curve are not told apart at backing scale 1. The radius depends on
the bezel's drawing: the accent-filled push bezel (default, a checked toggle, primary) fits 0.6-0.75 px smaller
than the translucent one of the same button and box (regular 5.4 against 6.15), which the capture over grey
confirms is no alpha artefact (the layer composited over #808080 matches it within 1 level at the corner pixels).

It exits 1, writing nothing, when a file it needs is missing or not as expected.
"""

import struct
import sys
import tomllib
import zlib
from math import pi, sqrt
from pathlib import Path

IMAGE = sys.argv[1] if len(sys.argv) > 1 else "os-probe-macos-26"
if IMAGE not in ("os-probe-macos-26", "os-probe-macos-15"):
    print(f"tiebreak_corners: unknown image folder {IMAGE!r}", file=sys.stderr)
    sys.exit(2)
RUN = Path(f"docs/evidence/os-probe/2026-10-30-run-41006842699/{IMAGE}")
PASSES = {"key": "button-pass", "not_key": "button-pass-not-key", "inactive": "button-pass-inactive",
          "contrast": "button-pass-increase-contrast-on"}
SHEETS = ("push", "styles_a", "styles_b", "styles_c")
APPEARANCES = ("aqua", "darkAqua")
STATES = ("normal", "pressed", "disabled")
CORNERS = ("top_left", "top_right", "bottom_right", "bottom_left")
SAMPLES = 16


class Bad(Exception):
    pass


def read_png(path: Path):
    """(width, height, rows of (r, g, b, a)) of an 8-bit, non-interlaced RGBA PNG."""
    if not path.is_file():
        raise Bad(f"{path}: missing")
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
    width, height, depth, ctype, _, _, interlace = ihdr
    if depth != 8 or ctype != 6 or interlace != 0:
        raise Bad(f"{path}: not 8-bit non-interlaced RGBA")
    raw = zlib.decompress(idat)
    stride, bpp = width * 4, 4
    rows, prev, i = [], bytearray(stride), 0
    for _ in range(height):
        ftype = raw[i]
        line = bytearray(raw[i + 1:i + 1 + stride])
        i += 1 + stride
        for x in range(stride):
            a = line[x - bpp] if x >= bpp else 0
            b = prev[x]
            c = prev[x - bpp] if x >= bpp else 0
            if ftype == 1:
                line[x] = (line[x] + a) & 255
            elif ftype == 2:
                line[x] = (line[x] + b) & 255
            elif ftype == 3:
                line[x] = (line[x] + (a + b) // 2) & 255
            elif ftype == 4:
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                line[x] = (line[x] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)) & 255
            elif ftype != 0:
                raise Bad(f"{path}: filter {ftype}")
        rows.append([tuple(line[x:x + 4]) for x in range(0, stride, 4)])
        prev = line
    return width, height, rows


def circle_coverage(r: float, n: int):
    """Coverage of each pixel of an n x n corner square (the corner at (0, 0)) by a circular corner of radius r."""
    g = [(k + 0.5) / SAMPLES for k in range(SAMPLES)]
    out = []
    for y in range(n):
        # per sample row, the corner's edge: x >= r - sqrt(r^2 - (r - py)^2) is inside (above the circle's centre)
        edges = []
        for sy in g:
            py = y + sy
            edges.append(r - sqrt(max(r * r - (r - py) ** 2, 0.0)) if py < r else 0.0)
        row = []
        for x in range(n):
            inside = 0
            for e in edges:
                for sx in g:
                    if x + sx >= e:
                        inside += 1
            row.append(inside / (SAMPLES * SAMPLES))
        out.append(row)
    return out


_COVER = {}


def cover(r: float, n: int):
    key = (round(r, 2), n)
    if key not in _COVER:
        _COVER[key] = circle_coverage(key[0], n)
    return _COVER[key]


def corner_square(rows, box, corner, n):
    """The n x n pixels at a corner of the box, flipped so the corner is at (0, 0)."""
    x0, y0, w, h = box
    out = []
    for j in range(n):
        y = y0 + j if corner.startswith("top") else y0 + h - 1 - j
        line = []
        for i in range(n):
            x = x0 + i if corner.endswith("left") else x0 + w - 1 - i
            line.append(rows[y][x])
        out.append(line)
    return out


def fit_corner(square, fill, n, outline=False):
    fr, fg, fb, fa = fill
    pts = []
    for j in range(n):
        inside = False
        for i in range(n):
            r, g, b, a = square[j][i]
            if outline and inside:
                pts.append((j, i, 1.0))
                continue
            if a > fa + 3 or (a >= fa / 2 and max(abs(r - fr), abs(g - fg), abs(b - fb)) > 8):
                continue
            if outline and a >= fa / 2:
                inside = True
            pts.append((j, i, min(a / fa, 1.0)))
    masked = n * n - len(pts)
    if not pts:
        return None
    missing = sum(1 - c for _, _, c in pts)
    area = sqrt(missing / (1 - pi / 4))
    best = None
    steps = int(n / 0.05) + 1
    for k in range(steps):
        r = k * 0.05
        cv = cover(r, n)
        err = sum((cv[j][i] - c) ** 2 for j, i, c in pts)
        if best is None or err < best[1]:
            best = (r, err)
    rms = sqrt(best[1] / len(pts))
    return {"fit_px": round(best[0], 2), "rms": round(rms, 4), "area_px": round(area, 3), "masked": masked}


def parse_fill(s):
    if not isinstance(s, str) or not s.startswith("#") or len(s) != 9:
        return None
    v = [int(s[k:k + 2], 16) for k in (1, 3, 5, 7)]
    return tuple(v) if v[3] >= 3 else None


def quarter(v: float) -> str:
    q = round(v * 4) / 4
    if q < 0.375:
        return "0px"
    t = f"{q:.2f}".rstrip("0").rstrip(".")
    return f"{t}px"


def main() -> int:
    out = {}
    images = []
    try:
        for pname, pdir in PASSES.items():
            for sheet in SHEETS:
                lp = RUN / f"layers-{pname}-{sheet}.toml"
                if not lp.is_file():
                    raise Bad(f"{lp}: missing")
                layers = tomllib.loads(lp.read_text())[pname][sheet]
                for app in APPEARANCES:
                    for state in STATES:
                        sub = "" if sheet == "push" else f"{sheet}/"
                        img = RUN / pdir / app / f"{sub}{state}-layer.png"
                        _, _, rows = read_png(img)
                        images.append(str(img))
                        for button, m in layers[app][state].items():
                            fill = parse_fill(m.get("fill"))
                            outline = False
                            strokes = [m.get(f"stroke_{side}") for side in ("top", "right", "bottom", "left")]
                            if fill is None and m.get("fill") == "absent" and all(
                                    isinstance(st, dict) and st.get("width") == 1 for st in strokes) and len(
                                    {st["edge"] for st in strokes}) == 1:
                                fill, outline = parse_fill(strokes[0]["edge"]), True
                            if fill is None:
                                continue
                            x, y, w, h = m["drawn_box"][:4]
                            n = min(w, h) // 2
                            res = {"box": [x, y, w, h]}
                            if outline:
                                res["read_from"] = "outline"
                            fits = {}
                            for corner in CORNERS:
                                f = fit_corner(corner_square(rows, (x, y, w, h), corner, n), fill, n, outline)
                                if f is None:
                                    break
                                fits[corner] = f
                                res[corner] = f
                            if len(fits) < 4:
                                res["radius"] = "unfit"
                            else:
                                vals = [fits[c]["fit_px"] for c in CORNERS]
                                res["spread_px"] = round(max(vals) - min(vals), 2)
                                half = min(w, h) / 2
                                if not all(fits[c]["rms"] <= 0.06 for c in CORNERS) or res["spread_px"] > 1.0:
                                    res["radius"] = "unfit"
                                elif all(v >= half - 0.75 for v in vals):
                                    res["radius"] = {"full": {"maximum_px": "absent"}}
                                else:
                                    res["radius"] = quarter(sum(vals) / 4)
                            out.setdefault(pname, {}).setdefault(sheet, {}).setdefault(app, {}).setdefault(
                                state, {})[button] = res
    except Bad as e:
        print(f"tiebreak_corners: {e}", file=sys.stderr)
        return 1
    lines = ["# Written by analysis/tiebreak_corners.py (its docstring is the method); do not edit.", "",
             "input_image = ["] + [f'  "{p}",' for p in images] + ["]", ""]

    def val(v):
        if isinstance(v, dict):
            return "{ " + ", ".join(f"{k} = {val(x)}" for k, x in v.items()) + " }"
        if isinstance(v, list):
            return "[" + ", ".join(val(x) for x in v) + "]"
        if isinstance(v, str):
            return f'"{v}"'
        return repr(v)

    for pname, sheets in out.items():
        for sheet, apps in sheets.items():
            for app, states in apps.items():
                for state, buttons in states.items():
                    for button, res in buttons.items():
                        lines.append(f'[{pname}.{sheet}.{app}.{state}."{button}"]')
                        for k, v in res.items():
                            lines.append(f"{k} = {val(v)}")
                        lines.append("")
    (RUN / "tiebreak-corners.toml").write_text("\n".join(lines))
    return 0


if __name__ == "__main__":
    sys.exit(main())
