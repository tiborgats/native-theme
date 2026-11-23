#!/usr/bin/env python3
"""Measures the controls' layers of the os-probe controls pass (`--button-pass --controls`) and writes what each
control's part draws at rest, hovered, pressed and highlighted, as `measure_button_layers.py` (run 41006842699) reads
the push button's.

Run from the repository root (Python 3.11+, the standard library only):

    python3 <run>/analysis/measure_controls.py <run>/<image folder>

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
- for the box (`box`) only, `corner_top_left` etc.: per row from the drawn box's edge, the pixels whose alpha is
  under 0.98 times the row's median alpha, counted from the side inward (at most 12 rows), and `radius_px`, the
  circle radius in 0.25 px steps whose missing area per row (16 samples a row) fits those counts best (least
  squares), `rms_px`;
- per tab of the tab view (`tabs.<label>`, the rect the probe found), `<appearance>.normal.tab_<label>`, the same
  reads at rest, with `selected`;
- `diff_from_normal`: the probe's own (pixels differing from the normal capture over the window's background inside
  the view's frame grown by 6 pt, and the largest byte difference), for hovered and pressed.

It exits 1, writing nothing, when a file it needs is missing or not as expected. It writes <image>/controls.toml.
"""

import struct
import sys
import tomllib
import zlib
from collections import Counter
from math import sqrt
from pathlib import Path


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
    for q in range(1, 81):
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


def measure(layer: Layer, part, with_corners: bool):
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
    if with_corners:
        for name, ys, xs in (("top_left", range(by0, by1), range(bx0, bx1)),
                             ("top_right", range(by0, by1), range(bx1 - 1, bx0 - 1, -1)),
                             ("bottom_left", range(by1 - 1, by0 - 1, -1), range(bx0, bx1)),
                             ("bottom_right", range(by1 - 1, by0 - 1, -1), range(bx1 - 1, bx0 - 1, -1))):
            counts = []
            for y in list(ys)[:12]:
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
    if len(sys.argv) != 2:
        raise Bad("usage: measure_controls.py <run>/<image folder>")
    image = Path(sys.argv[1])
    root = image / "button-pass-controls"
    probe = tomllib.loads((root / "probe.toml").read_text())
    c = probe["macos"]["button_pass"]["controls"]
    if "error" in c:
        raise Bad(f"the pass reports {c['error']}")
    targets = c["targets"]
    lines = [f"# measure_controls.py over {image.name}/button-pass-controls/probe.toml", ""]
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
                if state_name in ("normal", "highlighted"):
                    state = a[state_name]
                else:
                    state = a[state_name].get(t_name)
                    if state is None:
                        continue
                layer, why = layer_of(state)
                key = f"{appearance}.{state_name}.{t_name}"
                if layer is None:
                    lines.append(f'"{key}" = {{ error = "{why}" }}')
                    continue
                m = measure(layer, t["part_in_window_from_top_left_xywh"], t_name == "box")
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
        tabs = c.get("tabs", {})
        layer, why = layer_of(a["normal"])
        for label, tab in tabs.items():
            if label == "method" or "in_window_from_top_left_xywh" not in tab:
                continue
            key = f"{appearance}.normal.tab_{label}"
            if layer is None:
                lines.append(f'"{key}" = {{ error = "{why}" }}')
            else:
                m = measure(layer, tab["in_window_from_top_left_xywh"], False)
                m["selected"] = tab["selected"]
                lines.append(f'"{key}" = ' + toml_value(m))
        if appearance == "aqua":
            lines.append("")
            lines.append("# tabs: " + toml_value({k: v for k, v in tabs.items() if k != "method"}))
        lines.append("")
    (image / "controls.toml").write_text("\n".join(lines) + "\n")


if __name__ == "__main__":
    try:
        main()
    except (Bad, KeyError, OSError) as e:
        print(f"measure_controls.py: {e}", file=sys.stderr)
        sys.exit(1)
