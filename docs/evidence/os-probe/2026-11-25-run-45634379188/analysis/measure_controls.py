#!/usr/bin/env python3
"""Measures the controls' layers of the os-probe controls pass (`--button-pass --controls`) and writes what each
control's part draws at rest, hovered, pressed and highlighted, as `measure_button_layers.py` (run 41006842699) reads
the push button's. Run 45198750893's script, extended for run 45205409285: the corners over every row, with the
half-coverage edge fit, and the titled disclosure button's title; and for run 45208260830: the label taken out of a
translucent fill (`label`, `measure_button_layers.py`'s `colour_of`), in a title rect per target; and for run
45226132792: the pixels around the text field's part (`around_<side>`, its focus ring) and the toolbar sheet
(`--sheet toolbar_sheet`); and for run 45345116763: the radio buttons and the disabled check boxes and radio buttons
(labelled: their mark or dot inside the indicator), a check box's or radio button's `title_in_window_from_top_left_xywh` taken as its label rect only where
it lies within the part (the title lies beside the indicator and is read as `<key>_title`), and `--pass-dir` (the pass's folder, default button-pass-controls; the red-accent pass's is
button-pass-controls-accent); and for run 45542850219 (M2.FS4c10): `--sheet toolbar_items_sheet`, the toolbar items
sheet's image items, each target's part its item view's bounds; and for run 45634379188 (M2.FS4c13): `--sheet
toolbar_states_sheet`, the toolbar button beside itself disabled, as a toggle on and off and with an image and no title,
over an opaque title bar (the toolbar's labelled targets the four titled specimens).

Run from the repository root (Python 3.11+, the standard library only):

    python3 <run>/analysis/measure_controls.py [--pass-dir DIR] [--sheet toolbar_sheet] <run>/<image folder> [OUTPUT]

OUTPUT defaults to <image>/controls.toml (with `--sheet toolbar_sheet`, <image>/controls-toolbar_sheet.toml: the same
reads of `[macos.button_pass.controls.toolbar_sheet]`, the toolbar button in a window whose content view lies under its
transparent title bar, so its layers are solved over the backdrops; the label rect from its own `controls` entry); an earlier run's folder is read with OUTPUT set outside it, so the file that
run recorded is not rewritten.

It reads <image>/button-pass-controls/probe.toml, `[macos.button_pass.controls]`: `targets.<t>` (each target's view and
part in the window frame's points from its top left; the runner's backing scale is 1, so a point is a pixel), `tabs`,
and per appearance (aqua, darkAqua) the states `normal`, `highlighted` (highlight: YES on the NSButtons),
`hovered.<t>` and `pressed.<t>`, each with its layer (`layer.file`, `layer.file_rect_px_xywh`: the layer PNG covers
that rect of the window) and `diff_from_normal`. The layer is the probe's: per pixel the controls' own straight colour
and alpha, solved from the captures over black and white (`layer_method`).

Per `<appearance>.<state>.<target>` (state: normal, highlighted, hovered, pressed) it writes, from the layer, inside
the target's part (`part_in_window_from_top_left_xywh`; for the table rows the row's rect, for the tab the tab's rect
the probe found, for the check boxes the cell's image rect, the indicator):

- `drawn_box`: the bounds (x, y, w, h) of the part's pixels of alpha >= 4/255, in the window's pixels;
- `fill`: the most frequent straight colour and alpha (exact bytes) among the drawn box's pixels inset by 2 px on
  each side, "#rrggbbaa"; "absent" where its alpha is under 3/255; `fill_share`, the fraction of those pixels within
  2 levels of it in every channel and alpha;
- `edge_<side>` (top, bottom, left, right): the first two pixels from the drawn box's edge inward along its middle
  column or row, as read (the stroke, where one is drawn, is the first);
- `ink`: the straight colour of the pixel of the drawn box farthest (largest channel difference, alpha included)
  from the fill among its pixels of alpha >= 128/255 (all its pixels where none is): the label's or the glyph's
  darkest or lightest pixel; `ink_pixels`, how many pixels of the box differ from the fill by more than 24 levels in
  some channel or alpha; `common`, the box's four commonest straight colours of alpha >= 4/255 with their counts;
- `label`, `label_pixels`, `label_rect`, `label_rect_from` (the labelled targets only: the check boxes (their
  checkmark), the disclosure and push disclosure buttons (the title, the chevron), the push button, the pop-up, the
  text field, the segment, the table rows, the tab and the tabs, the toolbar button): the colour of the label inside
  its title rect, read as `measure_button_layers.py` (run 41006842699, `colour_of`, ported unchanged) reads a button's
  title: the fill passed is the `fill` above where its alpha is 3/255 or more and `fill_share` 0.35 or more, none
  where its alpha is under 3/255 ("absent"), and no label is read where the share is under 0.35 (`label =
  "non_uniform_fill"`); where the fill is translucent, the label solved from the rect's pixels of the largest alpha
  (within 2/255 of it) as the colour that, drawn over the fill, gives them (alpha (a - f) / (1 - f), colour
  (P - F (1 - alpha)) / alpha, P and F premultiplied); where the fill is opaque (alpha >= 250/255), the pixel farthest
  from the fill, opaque ("none" where it lies within 48 levels, rgb distance), the rect's pixels of alpha under
  250/255 left out (they lie outside the bezel: its antialiased corners and shadow, which an inset rect can reach;
  a cell's title rect inside an opaque bezel holds none); where there is no fill, the straight
  colour of the pixels of the largest alpha ("none" under 8/255). The title rect (`label_rect`, x y w h in the
  window's pixels; `label_rect_from` says which): the target's own `title_in_window_from_top_left_xywh` where the
  probe records it (`title`); else the control's `cell_titleRectForBounds_xywh`, where not empty, at the control's
  `in_window_frame_from_top_left_xywh` origin, whole points as the button script takes them, where that frame is
  the target's view and the rect lies within the target's part (`cell`; a check box's title lies beside its part,
  the indicator); else the drawn box inset by 2 px on each side, the rect the fill is read in (`inset`:
  it leaves out a stroke and the 1 px a part's edge may be off). For hovered, highlighted and pressed,
  `label_unchanged`: whether `label` is the normal state's;
- for the box (`box`) only, `corner_top_left` etc.: per row from the drawn box's edge, the pixels whose alpha is
  under 0.98 times the row's median alpha, counted from the side inward, over every row of the corner (half the
  drawn box's shorter side; run 45198750893's analysis read at most 12), and `radius_px`, the circle radius in
  0.25 px steps up to that many rows whose missing area per row (16 samples a row) fits those counts best (least
  squares), `rms_px`; and `corner_edge_top_left` etc., the edge at half coverage: per row, the pixels from the side
  inward before the first whose alpha is at least half the alpha of the outermost pixel of the straight side at
  the box's middle row (the stroke where one is drawn, else the fill), over the same rows, and the circle radii in
  0.25 px steps whose pixel-centre model (a pixel is missing where its centre lies outside a circle of that radius
  tangent to both sides) gives those counts with the least squared error: `radius_px` [smallest, largest] of the
  radii that share that error, `rms_px`;
- for the titled disclosure button (`disclosure`), `<appearance>.<state>.disclosure_title`: the same reads inside
  its title's rect (`title_in_window_from_top_left_xywh`, the cell's titleRectForBounds:);
- per tab of the tab view (`tabs.<label>`, the rect the probe found), `<appearance>.normal.tab_<label>`, the same
  reads at rest, with `selected`;
- guards: the window's backing scale must be 1 (else nothing is written); a target whose part the probe reports as
  an error, or whose part does not lie within its view, gets one `error` line; `highlighted.<target>` is written
  only for the targets in `cells_highlighted` (the cells `highlight:` lit);
- for the text field (`text_field`), `around_<side>` (top, bottom, left, right): along the part's middle column or row,
  the pixels from 2 inside the part's edge to 8 outside it, read outward, as "#rrggbbaa" (the bezel's edge, then
  whatever is drawn outside the field: the focus ring of the pressed field); `around_px_offsets`, each pixel's offset
  from the part's edge (negative inside, 0 the first pixel outside);
- `diff_from_normal`: the probe's own (pixels differing from the normal capture over the window's background inside
  the view's frame grown by 6 pt, and the largest byte difference), for hovered and pressed.

It exits 1, writing nothing, when a file it needs is missing or not as expected. It writes <image>/controls.toml.
"""

import struct
import sys
import tomllib
import zlib
from collections import Counter
from math import ceil, sqrt
from pathlib import Path


class Bad(Exception):
    pass


# The targets whose label (or checkmark, chevron, text) `label` reads, and the `controls` entry of a target named
# otherwise.
LABELLED = ("checkbox_off", "checkbox_on", "disclosure", "push_disclosure", "push", "popup", "text_field", "segment",
            "table_row", "source_list_row", "tab", "toolbar_button", "radio_off", "radio_on", "checkbox_disabled_off",
            "checkbox_disabled_on", "radio_disabled_off", "radio_disabled_on", "toolbar_button_disabled",
            "toolbar_toggle_on", "toolbar_toggle_off")
CONTROL_OF = {"segment": "segmented", "slider_knob": "slider"}


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


def hexc(c):
    return "#" + "".join(f"{v:02x}" for v in c)


def delta(a, b):
    return max(abs(p - q) for p, q in zip(a, b))


class Layer:
    def __init__(self, path: Path, origin):
        self.w, self.h, self.rows = read_png(path)
        self.ox, self.oy = origin

    def px(self, x, y):
        lx, ly = x - self.ox, y - self.oy
        if 0 <= lx < self.w and 0 <= ly < self.h:
            return self.rows[ly][lx]
        return None


def corner_radius(counts):
    """The radius in 0.25 px steps whose missing area per row fits `counts` best, and the fit's rms."""
    best = None
    for q in range(1, 4 * max(1, len(counts)) + 1):
        r = q / 4
        err = 0.0
        for k, n in enumerate(counts):
            area = 0.0
            for s in range(16):
                y = k + (s + 0.5) / 16
                if y < r:
                    area += (r - sqrt(max(0.0, r * r - (r - y) ** 2))) / 16
            err += (area - n) ** 2
        if best is None or err < best[1]:
            best = (r, err)
    return best[0], sqrt(best[1] / max(1, len(counts)))


def edge_radius(counts):
    """The radii in 0.25 px steps whose pixel-centre circle model gives `counts` with the least squared error."""
    rows = len(counts)
    results = []
    for q in range(1, 4 * rows + 1):
        r = q / 4
        err = 0
        for k, n in enumerate(counts):
            yc = k + 0.5
            model = 0 if yc >= r else max(0, ceil(r - sqrt(r * r - (r - yc) ** 2) - 0.5))
            err += (model - n) ** 2
        results.append((err, r))
    best = min(e for e, _ in results)
    radii = [r for e, r in results if e == best]
    return [min(radii), max(radii)], sqrt(best / max(1, rows))


def hexa(p):
    return "#%02x%02x%02x%02x" % tuple(max(0, min(255, round(v))) for v in p)


def colour_of(layer, rect, fill):
    """measure_button_layers.py's (run 41006842699) colour_of, unchanged but for a pixel outside the layer, which is
    (0, 0, 0, 0) as that script's layer gives it: the title's colour, or None."""
    x0, y0, w, h = rect
    pix = [layer.px(x, y) or (0, 0, 0, 0) for y in range(y0, y0 + h) for x in range(x0, x0 + w)]
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


class OpaqueOnly:
    """A layer whose pixels of alpha under 250/255 read as the (opaque) fill: over an opaque fill the title rect's
    pixels outside the bezel (its antialiased corners, its shadow) are left out of `colour_of`'s farthest pixel, as a
    title rect inside an opaque bezel holds none."""

    def __init__(self, layer, fill):
        self.layer, self.fill = layer, fill

    def px(self, x, y):
        p = self.layer.px(x, y)
        return p if p is not None and p[3] >= 250 else self.fill


# The targets whose surroundings `around` reads (the focus ring), and how far inside and outside the part's edge.
AROUND = ("text_field",)
AROUND_INSIDE = 2
AROUND_OUTSIDE = 8


def around(layer: Layer, part):
    """`around_<side>`: the pixels along the part's middle column or row from AROUND_INSIDE inside its edge to
    AROUND_OUTSIDE outside it, read outward (see the docstring)."""
    x0, y0, w, h = (round(v) for v in part)
    x1, y1 = x0 + w, y0 + h
    mx, my = x0 + w // 2, y0 + h // 2
    offsets = list(range(-AROUND_INSIDE, AROUND_OUTSIDE))
    out = {"around_px_offsets": offsets}
    for side, at in (("top", lambda k: (mx, y0 - 1 - k)), ("bottom", lambda k: (mx, y1 + k)),
                     ("left", lambda k: (x0 - 1 - k, my)), ("right", lambda k: (x1 + k, my))):
        row = []
        for k in offsets:
            p = layer.px(*at(k))
            row.append(hexc(p) if p is not None else "outside")
        out[f"around_{side}"] = row
    return out


def measure(layer: Layer, part, with_corners: bool, label_from=None):
    x0, y0, w, h = (round(v) for v in part)
    pts = [(x, y) for y in range(y0, y0 + h) for x in range(x0, x0 + w)]
    drawn = [(x, y) for (x, y) in pts if (p := layer.px(x, y)) is not None and p[3] >= 4]
    out = {}
    if not drawn:
        return {"drawn_box": "none"}
    bx0, by0 = min(x for x, _ in drawn), min(y for _, y in drawn)
    bx1, by1 = max(x for x, _ in drawn) + 1, max(y for _, y in drawn) + 1
    out["drawn_box"] = [bx0, by0, bx1 - bx0, by1 - by0]
    inner = [layer.px(x, y) for y in range(by0 + 2, by1 - 2) for x in range(bx0 + 2, bx1 - 2)]
    inner = [p for p in inner if p is not None]
    if not inner:
        inner = [layer.px(x, y) for (x, y) in drawn]
    fill, _ = Counter(inner).most_common(1)[0]
    share = sum(1 for p in inner if delta(p, fill) <= 2) / len(inner)
    out["fill"] = "absent" if fill[3] < 3 else hexc(fill)
    out["fill_share"] = round(share, 3)
    mx, my = (bx0 + bx1) // 2, (by0 + by1) // 2
    for side, a, b in (("top", (mx, by0), (mx, by0 + 1)), ("bottom", (mx, by1 - 1), (mx, by1 - 2)),
                       ("left", (bx0, my), (bx0 + 1, my)), ("right", (bx1 - 1, my), (bx1 - 2, my))):
        pa, pb = layer.px(*a), layer.px(*b)
        out[f"edge_{side}"] = [hexc(pa) if pa else "outside", hexc(pb) if pb else "outside"]
    box = [layer.px(x, y) for y in range(by0, by1) for x in range(bx0, bx1)]
    box = [p for p in box if p is not None]
    solid = [p for p in box if p[3] >= 128] or box
    far = max(solid, key=lambda p: delta(p, fill))
    out["ink"] = hexc(far)
    out["ink_pixels"] = sum(1 for p in box if delta(p, fill) > 24)
    out["common"] = [[hexc(c), n] for c, n in Counter(p for p in box if p[3] >= 4).most_common(4)]
    if label_from is not None:
        kind, rect = label_from
        if rect is None:
            kind, rect = "inset", [bx0 + 2, by0 + 2, bx1 - bx0 - 4, by1 - by0 - 4]
        out["label_rect"] = list(rect)
        out["label_rect_from"] = kind
        if fill[3] >= 3 and share < 0.35:
            out["label"] = "non_uniform_fill"
        else:
            col, n = colour_of(OpaqueOnly(layer, fill) if fill[3] >= 250 else layer, rect,
                               fill if fill[3] >= 3 else None)
            out["label"] = col if col is not None else "none"
            out["label_pixels"] = n
    if with_corners:
        rows = min(by1 - by0, bx1 - bx0) // 2
        refs = {"left": layer.px(bx0, my), "right": layer.px(bx1 - 1, my)}
        for name, ys, xs in (("top_left", range(by0, by1), range(bx0, bx1)),
                             ("top_right", range(by0, by1), range(bx1 - 1, bx0 - 1, -1)),
                             ("bottom_left", range(by1 - 1, by0 - 1, -1), range(bx0, bx1)),
                             ("bottom_right", range(by1 - 1, by0 - 1, -1), range(bx1 - 1, bx0 - 1, -1))):
            counts = []
            for y in list(ys)[:rows]:
                row = [layer.px(x, y) for x in xs]
                alphas = sorted(p[3] for p in row if p is not None and p[3] >= 8)
                if not alphas:
                    break
                median = alphas[len(alphas) // 2]
                n = 0
                for p in row:
                    if p is not None and p[3] >= 0.98 * median:
                        break
                    n += 1
                counts.append(n)
            radius, rms = corner_radius(counts)
            out[f"corner_{name}"] = {"missing_px_per_row": counts, "radius_px": radius, "rms_px": round(rms, 3)}
            ref = refs["left" if name.endswith("left") else "right"]
            if ref is None or ref[3] < 4:
                out[f"corner_edge_{name}"] = "no straight side at the middle row"
                continue
            edge = []
            for y in list(ys)[:rows]:
                n = 0
                for x in xs:
                    p = layer.px(x, y)
                    if p is not None and p[3] >= 0.5 * ref[3]:
                        break
                    n += 1
                edge.append(n)
            radii, rms = edge_radius(edge)
            out[f"corner_edge_{name}"] = {"reference_alpha": ref[3], "missing_px_per_row": edge, "radius_px": radii,
                                          "rms_px": round(rms, 3)}
    return out


def toml_value(v):
    if isinstance(v, str):
        return '"' + v + '"'
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return str(v)
    if isinstance(v, list):
        return "[" + ", ".join(toml_value(x) for x in v) + "]"
    if isinstance(v, dict):
        return "{ " + ", ".join(f"{k} = {toml_value(x)}" for k, x in v.items()) + " }"
    raise Bad(f"cannot write {v!r}")


def main():
    args = sys.argv[1:]
    pass_dir = "button-pass-controls"
    if args[:1] == ["--pass-dir"]:
        if len(args) < 2:
            raise Bad("--pass-dir takes a folder name")
        pass_dir, args = args[1], args[2:]
    sheet = None
    if args[:1] == ["--sheet"]:
        if len(args) < 2 or args[1] not in ("toolbar_sheet", "toolbar_items_sheet", "toolbar_states_sheet"):
            raise Bad("--sheet takes toolbar_sheet, toolbar_items_sheet or toolbar_states_sheet")
        sheet, args = args[1], args[2:]
    if len(args) not in (1, 2):
        raise Bad("usage: measure_controls.py [--sheet toolbar_sheet] <run>/<image folder> [OUTPUT]")
    image = Path(args[0])
    default = "controls.toml" if sheet is None else f"controls-{sheet}.toml"
    output = Path(args[1]) if len(args) == 2 else image / default
    root = image / pass_dir
    probe = tomllib.loads((root / "probe.toml").read_text())
    c = probe["macos"]["button_pass"]["controls"]
    if sheet is not None:
        if sheet not in c:
            raise Bad(f"no {sheet} in the controls pass")
        c = c[sheet]
    if "error" in c:
        raise Bad(f"the pass reports {c['error']}")
    if c.get("window_backingScaleFactor") != 1.0:
        raise Bad(f"backing scale {c.get('window_backingScaleFactor')!r}: the reads assume 1 (a point is a pixel)")
    targets = c["targets"]
    controls = c.get("controls", {})
    where = "" if sheet is None else f", [macos.button_pass.controls.{sheet}]"
    lines = [f"# measure_controls.py over {image.name}/{pass_dir}/probe.toml{where}", ""]

    def label_from(t_name, t):
        """The title rect of a labelled target (see the docstring), None for the drawn box inset by 2 px."""
        if t_name not in LABELLED:
            return None
        title = t.get("title_in_window_from_top_left_xywh")
        px, py, pw, ph = t["part_in_window_from_top_left_xywh"]
        if isinstance(title, list):
            r = [round(v) for v in title]
            within = px <= r[0] and py <= r[1] and r[0] + r[2] <= px + pw and r[1] + r[3] <= py + ph
            if within or not t_name.startswith(("checkbox", "radio")):
                return "title", r
        ctl = controls.get(CONTROL_OF.get(t_name, t_name), {})
        r = ctl.get("cell_titleRectForBounds_xywh")
        frame = ctl.get("in_window_frame_from_top_left_xywh")
        if r and r[2] > 0 and r[3] > 0 and frame == t["view_in_window_from_top_left_xywh"]:
            rect = [int(frame[0]) + int(r[0]), int(frame[1]) + int(r[1]), int(r[2]), int(r[3])]
            if px <= rect[0] and py <= rect[1] and rect[0] + rect[2] <= px + pw and rect[1] + rect[3] <= py + ph:
                return "cell", rect
        return "inset", None

    normal_labels = {}
    layers = {}

    def layer_of(state):
        lay = state.get("layer", {})
        f = lay.get("file")
        if not isinstance(f, str) or f.startswith("unavailable"):
            return None, f"no layer: {f!r} {lay.get('error', '')}"
        key = (f,)
        if key not in layers:
            xywh = lay["file_rect_px_xywh"]
            layers[key] = Layer(root / f, (xywh[0], xywh[1]))
        return layers[key], None

    for appearance in ("aqua", "darkAqua"):
        a = c.get(appearance)
        if not isinstance(a, dict):
            raise Bad(f"{appearance}: {a!r}")
        for state_name in ("normal", "highlighted", "hovered", "pressed"):
            for t_name, t in targets.items():
                key = f"{appearance}.{state_name}.{t_name}"
                if "part_in_window_from_top_left_xywh" not in t:
                    if state_name == "normal":
                        lines.append(f'"{key}" = {{ error = "{t.get("error", "no part")}" }}')
                    continue
                vx, vy, vw, vh = t["view_in_window_from_top_left_xywh"]
                px, py, pw, ph = t["part_in_window_from_top_left_xywh"]
                if px < vx or py < vy or px + pw > vx + vw or py + ph > vy + vh:
                    if state_name == "normal":
                        lines.append(f'"{key}" = {{ error = "the part does not lie within its view" }}')
                    continue
                if state_name == "highlighted" and t_name not in a["highlighted"].get("cells_highlighted", []):
                    continue
                if state_name in ("normal", "highlighted"):
                    state = a[state_name]
                else:
                    state = a[state_name].get(t_name)
                    if state is None:
                        continue
                layer, why = layer_of(state)
                if layer is None:
                    lines.append(f'"{key}" = {{ error = "{why}" }}')
                    continue
                m = measure(layer, t["part_in_window_from_top_left_xywh"], t_name == "box", label_from(t_name, t))
                if t_name in AROUND:
                    m.update(around(layer, t["part_in_window_from_top_left_xywh"]))
                if "label" in m:
                    if state_name == "normal":
                        normal_labels[(appearance, t_name)] = m["label"]
                    elif (appearance, t_name) in normal_labels:
                        m["label_unchanged"] = m["label"] == normal_labels[(appearance, t_name)]
                if state_name in ("hovered", "pressed"):
                    d = state.get("diff_from_normal")
                    if isinstance(d, dict):
                        m["diff_from_normal"] = {"differing_pixels": d["differing_pixels"],
                                                 "max_byte_delta": d["max_byte_delta"]}
                    else:
                        m["diff_from_normal"] = str(d)
                if state_name == "pressed":
                    m["key_at_capture"] = all(state.get(b, {}).get("key_at_capture", False)
                                              for b in ("over_black", "over_white"))
                lines.append(f'"{key}" = ' + toml_value(m))
                title = t.get("title_in_window_from_top_left_xywh")
                if isinstance(title, list):
                    m = measure(layer, title, False, ("title", [round(v) for v in title]))
                    lines.append(f'"{key}_title" = ' + toml_value(m))
        tabs = c.get("tabs", {})
        layer, why = layer_of(a["normal"])
        for label, tab in tabs.items():
            if label == "method" or "in_window_from_top_left_xywh" not in tab:
                continue
            key = f"{appearance}.normal.tab_{label}"
            if layer is None:
                lines.append(f'"{key}" = {{ error = "{why}" }}')
            else:
                m = measure(layer, tab["in_window_from_top_left_xywh"], False, ("inset", None))
                m["selected"] = tab["selected"]
                lines.append(f'"{key}" = ' + toml_value(m))
        if appearance == "aqua":
            lines.append("")
            lines.append("# tabs: " + toml_value({k: v for k, v in tabs.items() if k != "method"}))
        lines.append("")
    output.write_text("\n".join(lines) + "\n")


if __name__ == "__main__":
    try:
        main()
    except (Bad, KeyError, OSError) as e:
        print(f"measure_controls.py: {e}", file=sys.stderr)
        sys.exit(1)
