#!/usr/bin/env python3
"""Measures the buttons' layers of the os-probe button pass of run 41006842699 (macOS 26.6.2 in its macos-26 job,
macOS 15.7.9 in its macos-15 job) and writes the values the macOS facts cite (docs/facts/macos/button.toml,
tokens.toml), as S17's analysis output.

Run from the repository root (Python 3.11+, the standard library only), once per image folder:

    python3 docs/evidence/os-probe/2026-10-30-run-41006842699/analysis/measure_button_layers.py [IMAGE]

IMAGE is the run's image folder, `os-probe-macos-26` (the default: macos-tahoe's) or `os-probe-macos-15`
(macos-monterey's); below, os-probe-macos-26/ stands for it. What one image's probe marks unavailable (macOS 15's
`extraLarge`, `glass`, `primary`, `tinted` and the border shapes: "unavailable: … needs macOS 26") it does not
measure: a geometry size that is no table and a derivation whose button the sheet lacks are left out.

It writes its output, in parts, to os-probe-macos-26/ of the run: `layers-<pass>-<sheet>.toml` (the buttons'
measurements per pass and sheet, the accent-red pass's only through `derivations.toml`), `focus.toml`, `caret.toml`,
`geometry.toml`, `derivations.toml`.

What it reads, all under os-probe-macos-26/ of the run (the probe's own method texts are in each probe.toml,
`[macos.button_pass]` `method`, `backdrops`, `layer_method`):

- <pass>/probe.toml of the passes `button-pass` (key: the window key and main, the application active),
  `button-pass-not-key` (another window of the probe key: the sheet neither key nor main, the application active),
  `button-pass-inactive` (the application deactivated: the sheet neither key nor main), `button-pass-accent-red`
  (AppleAccentColor 0), `button-pass-increase-contrast-on` (increaseContrast 1): each button's frame in its layer
  (`[…sheets.<sheet>.<appearance>.<state>.layer.buttons.<button>] frame_px_xywh`), its cell's title and image rects
  (`[…sheets.<sheet>.buttons.<button>] cell_titleRectForBounds_xywh`, `cell_imageRectForBounds_xywh`, points from
  the frame's top left; the screen's backing scale is 1, so a point is a pixel) and the layer's file;
- the layers `<pass>/<appearance>/[<sheet>/]<state>-layer.png`: per pixel the button sheet's own straight colour
  and alpha, solved by the probe from the captures over black and white (`layer_method`);
- for the Increase Contrast pass also `<state>-over-grey.png`, the capture over the grey backdrop (#808080), as its
  layer is not one linear overlay everywhere (the probe's `check_grey`): each value read from that layer is checked
  at the pixels it is read from against that capture (`*_grey_delta`, the largest byte difference of the layer
  composited over #808080 and the capture);
- button-pass-focus/<appearance>/unfocused-layer.png and focused-<size>-<kind>-layer.png (the focused button's
  region only, `[…focused.<button>.layer] file_rect_px_xywh`): the focus ring;
- button-pass[-accent-red]/<appearance>/caret/caret-{0,1}.png: two captures of a text view's caret as it blinks.

What it writes, per `<pass>.<sheet>.<appearance>.<state>.<button>` (pass: key, not_key, inactive, accent_red,
contrast; sheet: push, styles_a, styles_b, styles_c; state: normal, pressed, disabled), from the button's layer:

- `drawn_box`: the bounds (x, y, w, h, pixels) of the frame's pixels of alpha >= 4/255;
- `fill`: the most frequent straight colour and alpha (exact bytes) among the pixels of the drawn box inset by 2 px
  on each side (a circle's box likewise), "#rrggbbaa"; "absent" where that colour's alpha is under 3/255 (no bezel:
  the box is the label's or a stroke's); `fill_share`, the fraction of those pixels within 2 levels of it in every
  channel (the label and an icon are the rest); "non_uniform" where the share is under 0.35;
- `stroke`: "absent" where, on each side, along the box's middle column or row, the first and the second pixel from
  the edge are the fill (the second within 3 levels in every channel and alpha; the first the same straight colour
  within 3 levels, at an alpha no larger than the fill's plus 3: the fill at the edge's coverage), or both pixels'
  alpha under 8/255 where the fill is absent; else per side `stroke_<side> = { width = 1, edge = "#rrggbbaa" }`
  where only the first pixel differs, or "unclassified"; `edge_<side>`, the first two pixels as read;
- `corner_<corner>`: per corner (top_left, top_right, bottom_right, bottom_left) of the drawn box, per row from that
  corner's edge (up to half the box's height, ending after two rows under 0.02 or at 40 rows), the missing coverage
  summed from the box's side inward to the first pixel at 0.98 coverage (coverage: alpha over the row's median alpha
  of its pixels of alpha >= 8/255, clamped to 0..1), as the probe's corner profile is read; `radius_px`, the circle
  radius in 0.25 px steps whose missing area per row (integrated over the row, 16 samples) fits those rows best
  (least squares), `rms_px`; `radius`, the value as a fact spells it: `{ full = { maximum_px = "absent" } }` where
  the radius is within 0.75 px of half the box's smaller side (a capsule or a circle), else "<radius>px"; only where
  `fill` is a colour;
- `label`, `icon`: the colour of the title (the cell's title rect) and of the image (its image rect): where the fill
  is translucent, solved from the pixels of the largest alpha (within 2/255 of it) as the colour that, drawn over the
  fill, gives them (alpha (a - f) / (1 - f), colour (P - F (1 - alpha)) / alpha, P and F premultiplied); where the
  fill is opaque, the pixel farthest from the fill, opaque; where the fill is absent, the straight colour of the
  pixels of the largest alpha; `label_pixels`, how many pixels it is read from;
- `drawn_only_content`: true where the fill is absent and every drawn pixel lies in the title or image rect grown by
  1 px: nothing but the label or the image is drawn, so `stroke` is "absent" too;
- `shadow`: "absent" where no pixel of the frame grown by 6 px, outside the frame, has an alpha >= 3/255, else
  "drawn" (`shadow_pixels`);

Per appearance, `focus.<appearance>.<button>` for the push sheet's buttons of the focus pass (the Full Keyboard Access
pass, `isFullKeyboardAccessEnabled` true): along the frame's middle row and middle column, from 6 px outside the
frame inward, the pixels whose premultiplied colour or alpha differs from the unfocused layer's by more than 2/255:
`outside_px` (how many lie outside the frame on each side, the same on all four sides or "uneven") and `inside_px`
(inside it); `ring` = the straight colour and alpha of the ring where it lies outside the frame (the most frequent);
`kind` = "ring" where the ring runs round all four sides; `width` = "<outside + inside>px"; `offset` = "-<inside>px"
(the ring's inner edge inside the box's edge, as S2.11's offset measures it outward); `outer_radius_px` (the ring's
outer corner: the corner fit above on the focused layer's alpha at the changed pixels, the box the frame grown by
`outside_px`) and `radius` = "follow_part" where it is within 0.75 px of
the button's own corner radius (its unfocused layer) plus `outside_px`.

Per appearance and pass (key, accent_red), `caret.<pass>.<appearance>`: the pixels changed between caret-0.png and
caret-1.png, their bounds; `width` = "<n>px", n the largest number of columns, in one of the two captures, that are one
colour from the bounds' top to its bottom other than the text view's background (2 px left of the bounds): fully
covered by the caret; `colors_seen`, those columns' colours as captured over the text view's
background (`colors_seen`, every such column's colour in either capture; a capture taken while the caret fades
shows a blend with the background).

Per sheet and button of the key pass, `geometry.<sheet>.<button>` from its probe.toml (points, a point a pixel):
`min_height` (the intrinsic content size's height), `padding_left`, `padding_top` (the title rect's x and y), `padding_right`, `padding_bottom` (the bounds less
the title rect's far edges), and `icon_text_gap` (the title rect's x less the image rect's right edge) where both
rects are drawn. Per control size, `geometry.sizes.<size>`: `min_width` (the intrinsic width of a push button with an
empty title, `[…geometry.sizes.<size>.empty_title]`), `font_size` (in points, a bare number) and `font_family` (the label font the sheets set,
`systemFontOfSize:` of `systemFontSizeForControlSize:`, `font_control_common`).

`derivation.<name>`, the passes of a measured derivation (S17), per scheme: `"input.accent"` (controlAccentColor of
that appearance, read by the main probe of the same job with the image's accent, probe.toml, and with
AppleAccentColor 0, accent-red/probe.toml) and `result`, the value measured above with that accent:
`default_fill_<scheme>` (push regular-default normal fill), `primary_fill_<scheme>` (styles_b regular-primary),
`tinted_label_<scheme>` (styles_b regular-tinted label), `caret_color_light` (the caret's colour at its full
opacity: the pixel at (46, 58) of the caret capture in which it differs from the text view's background at (43, 58);
the dark captures of the key pass show the caret only while it fades, so no dark pass is written). A derivation whose
button reads no colour in a pass (macOS 15's default push bezel, a gradient: `fill = "non_uniform"`) is not written.
Where the two
captures of a pass are identical (macOS 15's key pass and dark passes: the caret shows in neither), `caret.<pass>.
<appearance>` is `changed_pixels = 0`, `caret = "not_captured"`, and `caret_color_light` is not written.

It exits 1, writing nothing, when a file it needs is missing or not as expected.
"""

import struct
import sys
import tomllib
import zlib
from collections import Counter
from math import sqrt
from pathlib import Path

IMAGE = sys.argv[1] if len(sys.argv) > 1 else "os-probe-macos-26"
if IMAGE not in ("os-probe-macos-26", "os-probe-macos-15"):
    print(f"measure_button_layers: unknown image folder {IMAGE!r}", file=sys.stderr)
    sys.exit(2)
RUN = f"docs/evidence/os-probe/2026-10-30-run-41006842699/{IMAGE}"
PASSES = {"key": "button-pass", "not_key": "button-pass-not-key", "inactive": "button-pass-inactive",
          "accent_red": "button-pass-accent-red", "contrast": "button-pass-increase-contrast-on"}
SHEETS = ("push", "styles_a", "styles_b", "styles_c")
APPEARANCES = ("aqua", "darkAqua")
STATES = ("normal", "pressed", "disabled")
SIDES = ("top", "right", "bottom", "left")
CORNERS = ("top_left", "top_right", "bottom_right", "bottom_left")


class Bad(Exception):
    pass


def read_png(path: Path):
    """(width, height, rows of (r, g, b, a)) of an 8-bit, non-interlaced RGB or RGBA PNG."""
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
        if f == 1:
            for x in range(bpp, stride):
                line[x] = (line[x] + line[x - bpp]) & 0xFF
        elif f == 2:
            for x in range(stride):
                line[x] = (line[x] + prev[x]) & 0xFF
        elif f == 3:
            for x in range(stride):
                a = line[x - bpp] if x >= bpp else 0
                line[x] = (line[x] + (a + prev[x]) // 2) & 0xFF
        elif f == 4:
            for x in range(stride):
                a = line[x - bpp] if x >= bpp else 0
                b = prev[x]
                c = prev[x - bpp] if x >= bpp else 0
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                pred = a if pa <= pb and pa <= pc else (b if pb <= pc else c)
                line[x] = (line[x] + pred) & 0xFF
        elif f != 0:
            raise Bad(f"{path}: filter {f}")
        rows.append([tuple(line[x:x + bpp]) + ((255,) if bpp == 3 else ()) for x in range(0, stride, bpp)])
        prev = line
    return width, height, rows


class Layer:
    def __init__(self, path: Path, origin=(0, 0)):
        self.w, self.h, self.rows = read_png(path)
        self.ox, self.oy = origin

    def px(self, x, y):
        x, y = x - self.ox, y - self.oy
        if 0 <= x < self.w and 0 <= y < self.h:
            return self.rows[y][x]
        return (0, 0, 0, 0)


def hexa(p):
    return "#%02x%02x%02x%02x" % tuple(max(0, min(255, round(v))) for v in p)


def near(p, q, d):
    return all(abs(a - b) <= d for a, b in zip(p, q))


def same_paint(p, q, d=3):
    """Two straight pixels the same colour and alpha within d levels; two nearly transparent pixels are equal."""
    if p[3] < 8 and q[3] < 8:
        return True
    return near(p, q, d)


def drawn_box(layer, fr):
    x0, y0, w, h = fr
    xs, ys, n = [], [], 0
    for y in range(y0, y0 + h):
        for x in range(x0, x0 + w):
            if layer.px(x, y)[3] >= 4:
                xs.append(x)
                ys.append(y)
                n += 1
    if not n:
        return None
    return (min(xs), min(ys), max(xs) - min(xs) + 1, max(ys) - min(ys) + 1, n)


def mode_fill(layer, box):
    x0, y0, w, h = box[:4]
    pix = [layer.px(x, y) for y in range(y0 + 2, y0 + h - 2) for x in range(x0 + 2, x0 + w - 2)]
    if not pix:
        return None, 0.0, []
    m, _ = Counter(pix).most_common(1)[0]
    share = sum(1 for p in pix if near(p, m, 2)) / len(pix)
    return m, share, [(x, y) for y in range(y0 + 2, y0 + h - 2) for x in range(x0 + 2, x0 + w - 2)
                      if near(layer.px(x, y), m, 2)]


def edges(layer, box, fill):
    x0, y0, w, h = box[:4]
    cx, cy = x0 + w // 2, y0 + h // 2
    lines = {"top": [(cx, y0), (cx, y0 + 1)], "bottom": [(cx, y0 + h - 1), (cx, y0 + h - 2)],
             "left": [(x0, cy), (x0 + 1, cy)], "right": [(x0 + w - 1, cy), (x0 + w - 2, cy)]}
    out = {}
    for side, (a, b) in lines.items():
        e0, e1 = layer.px(*a), layer.px(*b)
        if fill is None:
            if e0[3] < 8 and e1[3] < 8:
                cls = "absent"
            elif e1[3] < 8:
                cls = {"width": 1, "edge": hexa(e0)}
            else:
                cls = "unclassified"
        else:
            at_cov = (e0[3] <= fill[3] + 3) and (e0[3] < 8 or near(e0[:3], fill[:3], 3))
            if same_paint(e1, fill) and (same_paint(e0, fill) or at_cov):
                cls = "absent"
            elif same_paint(e1, fill):
                cls = {"width": 1, "edge": hexa(e0)}
            else:
                cls = "unclassified"
        out[side] = (cls, [hexa(e0), hexa(e1)])
    return out


def row_median(vals):
    v = sorted(vals)
    return v[len(v) // 2] if v else None


def corner_profile(layer, box, corner):
    x0, y0, w, h = box[:4]
    top = corner.startswith("top")
    left = corner.endswith("left")
    half = w // 2
    prof, flat = [], 0
    for i in range(min(40, (h + 1) // 2)):
        y = y0 + i if top else y0 + h - 1 - i
        alphas = [layer.px(x, y)[3] for x in range(x0, x0 + w) if layer.px(x, y)[3] >= 8]
        med = row_median(alphas)
        if not med:
            prof.append(float(half))
            continue
        inset = 0.0
        for j in range(half):
            x = x0 + j if left else x0 + w - 1 - j
            cov = min(1.0, max(0.0, layer.px(x, y)[3] / med))
            if cov >= 0.98:
                break
            inset += 1.0 - cov
        prof.append(inset)
        if inset < 0.02:
            flat += 1
            if flat >= 2:
                break
        else:
            flat = 0
    return prof


def row_area(r, i):
    """The area a circle of radius r in a corner leaves uncovered in pixel row i (from the corner's edge)."""
    s = 0.0
    for k in range(16):
        y = i + (k + 0.5) / 16
        if y < r:
            s += r - sqrt(max(0.0, r * r - (r - y) * (r - y)))
    return s / 16


def fit(prof):
    best = None
    for step in range(1, 161):
        r = step * 0.25
        sse = sum((row_area(r, i) - m) ** 2 for i, m in enumerate(prof))
        if best is None or sse < best[1]:
            best = (r, sse)
    return best[0], sqrt(best[1] / len(prof))


def colour_of(layer, rect, fill):
    """The title's or image's colour (see the docstring), or None."""
    x0, y0, w, h = rect
    pix = [layer.px(x, y) for y in range(y0, y0 + h) for x in range(x0, x0 + w)]
    if not pix:
        return None, 0
    if fill is None:
        amax = max(p[3] for p in pix)
        if amax < 8:
            return None, 0
        top = [p for p in pix if p[3] >= amax - 2]
        return hexa([sum(p[c] for p in top) / len(top) for c in range(3)] + [amax]), len(top)
    fa = fill[3] / 255
    if fill[3] >= 250:
        far = max(pix, key=lambda p: sum((p[c] - fill[c]) ** 2 for c in range(3)))
        if sum((far[c] - fill[c]) ** 2 for c in range(3)) < 48 ** 2:
            return None, 0
        return hexa(list(far[:3]) + [255]), 1
    sol = []
    for p in pix:
        a = p[3] / 255
        if a <= fa + 2 / 255:
            continue
        al = (a - fa) / (1 - fa)
        col = [(p[c] / 255 * a - fill[c] / 255 * fa * (1 - al)) / al for c in range(3)]
        sol.append((al, col))
    if not sol:
        return None, 0
    amax = max(s[0] for s in sol)
    top = [s for s in sol if s[0] >= amax - 2 / 255]
    col = [255 * sum(s[1][c] for s in top) / len(top) for c in range(3)]
    return hexa(col + [255 * amax]), len(top)


def shadow(layer, fr):
    x0, y0, w, h = fr
    n = 0
    for y in range(y0 - 6, y0 + h + 6):
        for x in range(x0 - 6, x0 + w + 6):
            if x0 <= x < x0 + w and y0 <= y < y0 + h:
                continue
            if layer.px(x, y)[3] >= 3:
                n += 1
    return n


def grey_delta(layer, grey, pts):
    d = 0
    for x, y in pts:
        p, g = layer.px(x, y), grey.px(x, y)
        a = p[3] / 255
        for c in range(3):
            d = max(d, abs(p[c] * a + 128 * (1 - a) - g[c]))
    return round(d, 3)


def toml_val(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return repr(v)
    if isinstance(v, str):
        return '"' + v + '"'
    if isinstance(v, list):
        return "[" + ", ".join(toml_val(x) for x in v) + "]"
    if isinstance(v, dict):
        return "{ " + ", ".join(f"{k} = {toml_val(x)}" for k, x in v.items()) + " }"
    raise Bad(f"value {v!r}")


def measure_button(layer, fr, cell, grey):
    out = {}
    box = drawn_box(layer, fr)
    if box is None:
        out["fill"] = "absent"
        out["stroke"] = "absent"
        out["shadow"] = "absent" if shadow(layer, fr) == 0 else "drawn"
        return out
    out["drawn_box"] = list(box)
    m, share, fill_pts = mode_fill(layer, box)
    fill = None
    if m is None or m[3] < 3:
        out["fill"] = "absent"
    elif share < 0.35:
        out["fill"] = "non_uniform"
        out["fill_share"] = round(share, 3)
    else:
        fill = m
        out["fill"] = hexa(m)
        out["fill_share"] = round(share, 3)
        if grey is not None:
            out["fill_grey_delta"] = grey_delta(layer, grey, fill_pts)
    fx, fy = fr[0], fr[1]
    content = [(fx + int(r[0]) - 1, fy + int(r[1]) - 1, int(r[2]) + 2, int(r[3]) + 2)
               for r in (cell.get("cell_titleRectForBounds_xywh"), cell.get("cell_imageRectForBounds_xywh"))
               if r and r[2] > 0 and r[3] > 0]
    label_only = fill is None and out["fill"] == "absent" and any(
        c[0] <= box[0] and c[1] <= box[1] and box[0] + box[2] <= c[0] + c[2] and box[1] + box[3] <= c[1] + c[3]
        for c in content)
    if label_only:
        out["drawn_only_content"] = True
        out["stroke"] = "absent"
    elif out["fill"] != "non_uniform":
        es = edges(layer, box, fill)
        if all(c == "absent" for c, _ in es.values()):
            out["stroke"] = "absent"
        for side in SIDES:
            cls, e = es[side]
            out[f"edge_{side}"] = e
            if out.get("stroke") != "absent":
                out[f"stroke_{side}"] = cls
    if fill is not None:
        for c in CORNERS:
            prof = corner_profile(layer, box, c)
            r, rms = fit(prof)
            spelled = ({"full": {"maximum_px": "absent"}} if abs(r - min(box[2], box[3]) / 2) <= 0.75
                       else f"{r:g}px")
            out[f"corner_{c}"] = {"insets_px": [round(v, 3) for v in prof], "radius_px": r,
                                  "rms_px": round(rms, 3), "radius": spelled}
    for name, key in (("label", "cell_titleRectForBounds_xywh"), ("icon", "cell_imageRectForBounds_xywh")):
        rect = cell.get(key)
        if not rect or rect[2] <= 0 or rect[3] <= 0:
            continue
        r = (fx + int(rect[0]), fy + int(rect[1]), int(rect[2]), int(rect[3]))
        col, n = colour_of(layer, r, fill if out["fill"] not in ("absent", "non_uniform") else None)
        if out["fill"] == "non_uniform" or col is None:
            continue
        out[name] = col
        out[f"{name}_pixels"] = n
        if grey is not None:
            pts = [(x, y) for y in range(r[1], r[1] + r[3]) for x in range(r[0], r[0] + r[2])
                   if layer.px(x, y)[3] >= 0.9 * max(layer.px(xx, yy)[3] for yy in range(r[1], r[1] + r[3])
                                                     for xx in range(r[0], r[0] + r[2]))]
            out[f"{name}_grey_delta"] = grey_delta(layer, grey, pts)
    n = shadow(layer, fr)
    out["shadow"] = "absent" if n == 0 else "drawn"
    out["shadow_pixels"] = n
    return out


def emit(lines, path, table):
    lines.append(f"[{path}]")
    for k, v in table.items():
        lines.append(f"{k} = {toml_val(v)}")
    lines.append("")


FILES = {}


def out_file(name):
    return FILES.setdefault(name, {"lines": [], "images": []})


def main():
    root = Path(RUN)
    results = {}
    for pname, pdir in PASSES.items():
        probe = tomllib.loads((root / pdir / "probe.toml").read_text())
        sheets = probe["macos"]["button_pass"]["sheets"]
        for sheet in SHEETS:
            s = sheets[sheet]
            images = out_file(f"layers-{pname}-{sheet}")["images"]
            images.append(f"{RUN}/{pdir}/probe.toml")
            for ap in APPEARANCES:
                for st in STATES:
                    if pname == "accent_red" and (sheet not in ("push", "styles_b") or st != "normal"):
                        continue  # the derivations read only these
                    lay = s[ap][st]["layer"]
                    rel = Path(pdir) / lay["file"]
                    layer = Layer(root / rel)
                    images.append(f"{RUN}/{rel}")
                    grey = None
                    if pname == "contrast":
                        grel = Path(pdir) / s[ap][st]["over_grey"]["file"]
                        grey = Layer(root / grel)
                        images.append(f"{RUN}/{grel}")
                    for b, bt in lay["buttons"].items():
                        cell = s["buttons"][b]
                        res = measure_button(layer, bt["frame_px_xywh"], cell, grey)
                        results[(pname, sheet, ap, st, b)] = res
    for (pname, sheet, ap, st, b), res in results.items():
        if pname != "accent_red":
            emit(out_file(f"layers-{pname}-{sheet}")["lines"], f"{pname}.{sheet}.{ap}.{st}.{b}", res)
    # focus
    lines, images = out_file("focus")["lines"], out_file("focus")["images"]
    images.append(f"{RUN}/button-pass-focus/probe.toml")
    fprobe = tomllib.loads((root / "button-pass-focus" / "probe.toml").read_text())
    push = fprobe["macos"]["button_pass"]["sheets"]["push"]
    for ap in APPEARANCES:
        urel = Path("button-pass-focus") / push[ap]["unfocused"]["layer"]["file"]
        unf = Layer(root / urel)
        images.append(f"{RUN}/{urel}")
        for b, ft in push[ap]["focused"].items():
            if not isinstance(ft, dict) or "layer" not in ft:
                continue
            rel = Path("button-pass-focus") / ft["layer"]["file"]
            rx, ry, rw, rh = ft["layer"]["file_rect_px_xywh"]
            foc = Layer(root / rel, (rx, ry))
            images.append(f"{RUN}/{rel}")
            fr = ft["layer"]["buttons"][b]["frame_px_xywh"]
            fx, fy, fw, fh = fr

            def changed(x, y):
                p, q = foc.px(x, y), unf.px(x, y)
                pa = [p[c] * p[3] / 255 for c in range(3)] + [p[3]]
                qa = [q[c] * q[3] / 255 for c in range(3)] + [q[3]]
                return any(abs(u - v) > 2 for u, v in zip(pa, qa))

            cy, cx = fy + fh // 2, fx + fw // 2
            lines_ = {"left": [(fx - 6 + i, cy) for i in range(12)], "right": [(fx + fw + 5 - i, cy) for i in range(12)],
                      "top": [(cx, fy - 6 + i) for i in range(12)], "bottom": [(cx, fy + fh + 5 - i) for i in range(12)]}
            outs, ins, ring = {}, {}, Counter()
            for side, pts in lines_.items():
                ch = [changed(x, y) for x, y in pts]
                outs[side] = sum(ch[:6])
                ins[side] = sum(ch[6:])
                for (x, y), c in zip(pts[:6], ch[:6]):
                    if c:
                        ring[foc.px(x, y)] += 1
            res = {}
            o = set(outs.values())
            i_ = set(ins.values())
            res["outside_px"] = o.pop() if len(o) == 1 else "uneven"
            res["inside_px"] = i_.pop() if len(i_) == 1 else "uneven"
            res["ring"] = hexa(ring.most_common(1)[0][0]) if ring else "none"
            res["kind"] = "ring" if all(v > 0 for v in outs.values()) else "not_a_ring"
            if isinstance(res["outside_px"], int) and isinstance(res["inside_px"], int):
                res["width"] = f'{res["outside_px"] + res["inside_px"]}px'
                res["offset"] = f'-{res["inside_px"]}px' if res["inside_px"] else "0px"
                ubox = drawn_box(unf, fr)
                uprof = corner_profile(unf, ubox, "top_left")
                ur, _ = fit(uprof)
                fbox = (fx - res["outside_px"], fy - res["outside_px"], fw + 2 * res["outside_px"],
                        fh + 2 * res["outside_px"])

                class Diff:
                    def px(self, x, y):
                        return (0, 0, 0, foc.px(x, y)[3] if changed(x, y) else 0)

                prof = corner_profile(Diff(), fbox, "top_left")
                orr, rms = fit(prof)
                res["button_radius_px"] = ur
                res["outer_radius_px"] = orr
                res["outer_rms_px"] = round(rms, 3)
                res["radius"] = ("follow_part" if abs(orr - (ur + res["outside_px"])) <= 0.75 else "other")
            emit(lines, f"focus.{ap}.{b}", res)
    # caret
    lines, images = out_file("caret")["lines"], out_file("caret")["images"]
    no_caret = set()
    for pname in ("key", "accent_red"):
        for ap in APPEARANCES:
            rels = [Path(PASSES[pname]) / ap / "caret" / f"caret-{n}.png" for n in (0, 1)]
            c0, c1 = (Layer(root / r) for r in rels)
            images += [f"{RUN}/{r}" for r in rels]
            pts = [(x, y) for y in range(c0.h) for x in range(c0.w) if c0.px(x, y) != c1.px(x, y)]
            if not pts:
                # the two captures are identical: the caret shows in neither (macOS 15's key pass), so the pass
                # gives no caret reading; it is written so, and the derivation below leaves the pass out
                no_caret.add((pname, ap))
                emit(lines, f"caret.{pname}.{ap}", {"changed_pixels": 0, "caret": "not_captured"})
                continue
            x0, x1 = min(p[0] for p in pts), max(p[0] for p in pts)
            y0, y1 = min(p[1] for p in pts), max(p[1] for p in pts)
            full_cols, colours = 0, Counter()
            for im in (c0, c1):
                bg = im.px(x0 - 2, y0)
                n = 0
                for x in range(x0, x1 + 1):
                    col = [im.px(x, y) for y in range(y0, y1 + 1)]
                    if len(set(col)) == 1 and col[0] != bg:
                        n += 1
                        colours[col[0]] += 1
                full_cols = max(full_cols, n)
            emit(lines, f"caret.{pname}.{ap}", {
                "bounds_px_xywh": [x0, y0, x1 - x0 + 1, y1 - y0 + 1], "changed_pixels": len(pts),
                "width": f"{full_cols}px", "colors_seen": sorted("#%02x%02x%02x" % c[:3] for c in colours),
            })
    # geometry (key pass)
    lines, images = out_file("geometry")["lines"], out_file("geometry")["images"]
    images.append(f"{RUN}/{PASSES['key']}/probe.toml")
    kprobe = tomllib.loads((root / PASSES["key"] / "probe.toml").read_text())
    for sheet in SHEETS:
        for b, bt in kprobe["macos"]["button_pass"]["sheets"][sheet]["buttons"].items():
            t, i = bt.get("cell_titleRectForBounds_xywh"), bt.get("cell_imageRectForBounds_xywh")
            w, h = bt["frame_xywh"][2], bt["frame_xywh"][3]
            g = {"min_height": f"{bt['intrinsicContentSize_wh'][1]:g}px"}
            if t and t[2] > 0:
                g.update({"padding_left": f"{t[0]:g}px", "padding_top": f"{t[1]:g}px",
                          "padding_right": f"{w - t[0] - t[2]:g}px", "padding_bottom": f"{h - t[1] - t[3]:g}px"})
            if t and t[2] > 0 and i and i[2] > 0:
                g["icon_text_gap"] = f"{t[0] - (i[0] + i[2]):g}px"
            emit(lines, f"geometry.{sheet}.{b}", g)
    for size, gs in kprobe["macos"]["button_pass"]["geometry"]["sizes"].items():
        if not isinstance(gs, dict):
            continue  # "unavailable: … needs macOS 26"
        emit(lines, f"geometry.sizes.{size}", {
            "min_width": f"{gs['empty_title']['intrinsicContentSize_wh'][0]:g}px",
            "font_size": gs["font_control_common"]["pointSize"],
            "font_family": gs["font_control_common"]["familyName"]})
    # measured derivations: the accent's passes
    lines, images = out_file("derivations")["lines"], out_file("derivations")["images"]
    for pname in ("key", "accent_red"):
        for sheet in ("push", "styles_b"):
            for ap in APPEARANCES:
                images.append(f"{RUN}/{PASSES[pname]}/{ap}/{'' if sheet == 'push' else sheet + '/'}normal-layer.png")
        images.append(f"{RUN}/{PASSES[pname]}/probe.toml")
        images += [f"{RUN}/{PASSES[pname]}/aqua/caret/caret-{n}.png" for n in (0, 1)]
    acc = {"key": tomllib.loads((root / "probe.toml").read_text()),
           "accent_red": tomllib.loads((root / "accent-red" / "probe.toml").read_text())}
    images_probe = [f"{RUN}/probe.toml", f"{RUN}/accent-red/probe.toml"]
    der = {}
    for name, (sheet, b, st, field) in {"default_fill": ("push", "regular-default", "normal", "fill"),
                                        "primary_fill": ("styles_b", "regular-primary", "normal", "fill"),
                                        "tinted_label": ("styles_b", "regular-tinted", "normal", "label")}.items():
        if ("key", sheet, "aqua", st, b) not in results:
            continue  # the image's sheet lacks the button (macOS 15: no tint prominence)
        if any(not str(results[(pname, sheet, ap, st, b)].get(field)).startswith("#")
               for pname in ("key", "accent_red") for ap in APPEARANCES):
            continue  # no colour read: macOS 15's gradient bezel (fill "non_uniform") has no derivation here
        for ap, scheme in (("aqua", "light"), ("darkAqua", "dark")):
            for pname in ("key", "accent_red"):
                a = acc[pname]["macos"]["colors"][ap]["controlAccentColor"]
                r = results[(pname, sheet, ap, st, b)][field]
                if not (a.endswith("ff") and r.endswith("ff")):
                    raise Bad(f"{name}: not opaque")
                der.setdefault(f"{name}_{scheme}", []).append(
                    {'"input.accent"': a[:7], "result": r[:7], "conditions": [f"scheme:{scheme}"]})
    for pname, ap in (("key", "aqua"), ("accent_red", "aqua")):
        if no_caret & {("key", "aqua"), ("accent_red", "aqua")}:
            break  # a pass shows no caret: no light caret derivation (two passes are needed)
        a = acc[pname]["macos"]["colors"][ap]["controlAccentColor"]
        rel = Path(PASSES[pname]) / ap / "caret"
        c0, c1 = (Layer(root / rel / f"caret-{n}.png") for n in (0, 1))
        seen = {c0.px(46, 58), c1.px(46, 58)} - {c0.px(43, 58)}
        if len(seen) != 1:
            raise Bad(f"caret {pname} {ap}: {seen}")
        der.setdefault("caret_color_light", []).append(
            {'"input.accent"': a[:7], "result": hexa(seen.pop())[:7], "conditions": ["scheme:light"]})
    for name, passes in der.items():
        for ps in passes:
            lines.append(f"[[derivation.{name}]]")
            for k, v in ps.items():
                lines.append(f"{k} = {toml_val(v)}")
            lines.append("")
    images += images_probe
    for name, f in FILES.items():
        if not f["lines"]:
            continue
        head = [f"# Output of analysis/measure_button_layers.py (its docstring is the method), file {name}.toml.",
                "# input_image: every file this part reads (layers, captures, and the probe.toml files whose frames,",
                "# cell rects and colours it takes).", "input_image = ["]
        head += [f'    "{x}",' for x in dict.fromkeys(f["images"])] + ["]", ""]
        (root / f"{name}.toml").write_text("\n".join(head + f["lines"]), encoding="utf-8")


if __name__ == "__main__":
    try:
        main()
    except (Bad, OSError, KeyError) as e:
        print(f"measure_button_layers: {e}", file=sys.stderr)
        sys.exit(1)
