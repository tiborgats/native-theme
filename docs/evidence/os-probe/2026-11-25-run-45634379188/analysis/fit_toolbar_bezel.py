#!/usr/bin/env python3
"""The toolbar-bezel button's shape in the real toolbar (M2.FS4c13): what it draws round its view at rest, hovered and
pressed over the opaque title bar, its box, its edge, its corners and whether the backdrops reach it.

Run from the repository root (Python 3.11+, the standard library only; `measure_controls.py` beside it gives the PNG
reader and `edge_radius`):

    python3 <run>/analysis/fit_toolbar_bezel.py <capture folder> <sheet: controls|toolbar_states_sheet> OUTPUT

<capture folder> is an image's `button-pass-controls` folder of the run's artifact (the PNGs are not kept in the
repository: `CAPTURES.sha256` names them); its probe.toml gives, under `[macos.button_pass.controls]` (with
`toolbar_states_sheet`, its `toolbar_states_sheet` table), each target's view
(`targets.<t>.view_in_window_from_top_left_xywh`; the backing scale is 1, a point is a pixel) and each state's
captures (`<appearance>.normal.file`, `hovered.<t>.file`, `pressed.<t>.window.file`, and their `over_<backdrop>.file`).
The targets read: `toolbar_button` on the controls sheet; every specimen on the toolbar states sheet.

Per `<appearance>.<state>.<target>`, in the capture over the window's own background, inside the view grown by 3 px
on each side:
- `outside`: per row the pixel 2 px left of the view (the title bar there; it is not uniform on macOS 26);
  `outside_top_left`, `outside_bottom_left` the first and last rows' (x, y in the window's pixels from its top left);
- `view_fill`: the commonest colour of the view inset by 3 px (`view_fill_share` its share); `mode` "filled" where
  it differs from the middle row's `outside` by more than 2 levels in a channel, else "ring" (nothing fills the
  view: what is drawn is an outline);
- a pixel's strength: "filled", the largest difference from its row's `outside` in the direction the fill differs
  in (per channel, toward the fill); "ring", the largest channel difference from it either way. `edge_peak_levels`:
  along the view's middle row from the left, the largest strength of the first pixel above 2 and the two after it;
  `half_levels`: the threshold of the reads below, half the fill's strength ("filled": half coverage of the body,
  a stroke counted with it) or half `edge_peak_levels` ("ring": half coverage of the outline);
- `box` (x, y, w, h): left and right the first pixel of strength at least `half_levels` along the view's middle row
  from either side, top and bottom the same along the box's middle column;
- `fill`: the commonest colour of the box inset by 3 px ("none (the title bar)" in ring mode), `fill_share` its
  share; `label_pixels` how many pixels of that inset box differ from it by more than 24 levels in a channel;
- `edge_<side>`: the first four pixels from the box's edge inward along its middle row or column (a stroke, where
  one is drawn, is the first; the rest the fill or, in ring mode, the title bar);
- `corner_<corner>`: per row from the box's top or bottom edge, over half the box's height, the pixels from the
  box's side inward before the first of strength at least `half_levels`, and `measure_controls.py`'s `edge_radius`
  of those counts (`radius_px` [smallest, largest], `rms_px`);
- `backdrop_reaches`: per backdrop (black, grey, white), the pixels of the view grown by 3 px that differ by more
  than 2 in a channel between the capture over that backdrop and the capture over the window's background (0 where
  the title bar is opaque under the view, so a read here is that composite: no layer is solved from it).
- on the toolbar states sheet, at rest, for each specimen but `toolbar_button`: `differs_from_toolbar_button`, the
  pixels of its view grown by 3 px that differ by more than 2 in a channel from those at the same offset round
  `toolbar_button`'s view in the same capture (where the two views have one size).
"""

import sys
from collections import Counter
from pathlib import Path

import tomllib

sys.path.insert(0, str(Path(__file__).resolve().parent))

from measure_controls import edge_radius, hexc, read_png  # noqa: E402

GROW = 3
SPECIMENS = ("toolbar_button", "toolbar_button_disabled", "toolbar_toggle_on", "toolbar_toggle_off", "toolbar_image")


class Bad(Exception):
    pass


class Image:
    def __init__(self, path: Path):
        self.w, self.h, self.rows = read_png(path)

    def px(self, x, y):
        return self.rows[y][x][:3]


def diff(a, b):
    return max(abs(p - q) for p, q in zip(a, b))


def longest(lines):
    runs = []
    for v in lines:
        if runs and v == runs[-1][-1] + 1:
            runs[-1].append(v)
        else:
            runs.append([v])
    return max(runs, key=len) if runs else None


def shape(img: Image, view):
    vx, vy, vw, vh = (round(v) for v in view)
    y0, y1 = vy - GROW, vy + vh + GROW
    x0, x1 = vx - GROW, vx + vw + GROW
    if vx - 2 < 0 or y0 < 0 or y1 > img.h or x1 > img.w:
        return {"error": "the view grown by 3 px does not lie within the capture"}
    outside = {y: img.px(vx - 2, y) for y in range(y0, y1)}
    out = {"outside_top_left": hexc(outside[y0]), "outside_bottom_left": hexc(outside[y1 - 1])}
    inner = Counter(img.px(x, y) for y in range(vy + 3, vy + vh - 3) for x in range(vx + 3, vx + vw - 3))
    if not inner:
        return {"error": "the view is too small"}
    fill, count = inner.most_common(1)[0]
    out["view_fill"] = hexc(fill)
    out["view_fill_share"] = round(count / sum(inner.values()), 3)
    mid_y, mid_x = vy + vh // 2, vx + vw // 2
    ring = diff(fill, outside[mid_y]) <= 2
    sign = [0 if ring else (1 if f > o else -1 if f < o else 0) for f, o in zip(fill, outside[mid_y])]

    def strength(p, o):
        if ring:
            return diff(p, o)
        return max(s * (a - b) for s, a, b in zip(sign, p, o) if s != 0)

    first = next((x for x in range(x0, mid_x) if strength(img.px(x, mid_y), outside[mid_y]) > 2), None)
    out["mode"] = "ring" if ring else "filled"
    if first is None:
        out["box"] = "nothing drawn on the view's middle row"
        return out
    peak = max(strength(img.px(x, mid_y), outside[mid_y]) for x in range(first, first + 3))
    half = peak / 2 if ring else strength(fill, outside[mid_y]) / 2
    out["edge_peak_levels"] = peak
    out["half_levels"] = half

    def scan(points):
        return next((k for k, (x, y) in enumerate(points) if strength(img.px(x, y), outside[y]) >= half), None)

    left = scan([(x, mid_y) for x in range(x0, mid_x)])
    right = scan([(x, mid_y) for x in range(x1 - 1, mid_x, -1)])
    if left is None or right is None:
        out["box"] = "no edge on the view's middle row"
        return out
    bx0, bx1 = x0 + left, x1 - 1 - right
    col = (bx0 + bx1) // 2
    top = scan([(col, y) for y in range(y0, mid_y)])
    bottom = scan([(col, y) for y in range(y1 - 1, mid_y, -1)])
    if top is None or bottom is None:
        out["box"] = "no edge on the box's middle column"
        return out
    by0, by1 = y0 + top, y1 - 1 - bottom
    out["box"] = [bx0, by0, bx1 - bx0 + 1, by1 - by0 + 1]
    body = Counter(img.px(x, y) for y in range(by0 + 3, by1 - 2) for x in range(bx0 + 3, bx1 - 2))
    if body:
        f, n = body.most_common(1)[0]
        out["fill"] = "none (the title bar)" if ring else hexc(f)
        out["fill_share"] = round(n / sum(body.values()), 3)
        out["label_pixels"] = sum(k for c, k in body.items() if diff(c, f) > 24)
    my, mx = (by0 + by1) // 2, (bx0 + bx1) // 2
    out["edge_left"] = [hexc(img.px(bx0 + k, my)) for k in range(4)]
    out["edge_right"] = [hexc(img.px(bx1 - k, my)) for k in range(4)]
    out["edge_top"] = [hexc(img.px(mx, by0 + k)) for k in range(4)]
    out["edge_bottom"] = [hexc(img.px(mx, by1 - k)) for k in range(4)]
    span = (by1 - by0 + 1) // 2
    for corner, rows, from_left in (("top_left", range(by0, by0 + span), True),
                                    ("top_right", range(by0, by0 + span), False),
                                    ("bottom_left", range(by1, by1 - span, -1), True),
                                    ("bottom_right", range(by1, by1 - span, -1), False)):
        counts = []
        for y in rows:
            cols = range(bx0, bx1 + 1) if from_left else range(bx1, bx0 - 1, -1)
            n = 0
            for x in cols:
                if strength(img.px(x, y), outside[y]) >= half:
                    break
                n += 1
            counts.append(n)
        radius, rms = edge_radius(counts)
        out[f"corner_{corner}"] = counts
        out[f"corner_{corner}_radius_px"] = radius
        out[f"corner_{corner}_rms_px"] = round(rms, 3)
    return out


def reaches(window: Image, other: Image, view):
    vx, vy, vw, vh = (round(v) for v in view)
    return sum(1 for y in range(vy - GROW, vy + vh + GROW) for x in range(vx - GROW, vx + vw + GROW)
               if diff(window.px(x, y), other.px(x, y)) > 2)


def same_place(img: Image, view, other):
    """The pixels of `view` grown by 3 px that differ by more than 2 in a channel from those at the same offset round
    `other` in the same capture, where the two views have one size; else why not compared."""
    if [round(v) for v in view[2:]] != [round(v) for v in other[2:]]:
        return f"not compared: the views are {view[2]} x {view[3]} and {other[2]} x {other[3]}"
    vx, vy, vw, vh = (round(v) for v in view)
    dx, dy = round(other[0]) - vx, round(other[1]) - vy
    return sum(1 for y in range(vy - GROW, vy + vh + GROW) for x in range(vx - GROW, vx + vw + GROW)
               if diff(img.px(x, y), img.px(x + dx, y + dy)) > 2)


def value(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return str(v)
    if isinstance(v, list):
        return "[" + ", ".join(value(x) for x in v) + "]"
    return '"' + str(v) + '"'


def main():
    if len(sys.argv) != 4 or sys.argv[2] not in ("controls", "toolbar_states_sheet"):
        raise Bad("usage: fit_toolbar_bezel.py <capture folder> <controls|toolbar_states_sheet> OUTPUT")
    root, sheet, output = Path(sys.argv[1]), sys.argv[2], Path(sys.argv[3])
    c = tomllib.loads((root / "probe.toml").read_text())["macos"]["button_pass"]["controls"]
    if sheet != "controls":
        c = c[sheet]
    if c.get("window_backingScaleFactor") != 1.0:
        raise Bad(f"backing scale {c.get('window_backingScaleFactor')!r}: the reads assume 1")
    names = ("toolbar_button",) if sheet == "controls" else SPECIMENS
    lines = [f"# fit_toolbar_bezel.py over {root.parent.name}/{root.name}/probe.toml, sheet {sheet}", ""]
    for appearance in ("aqua", "darkAqua"):
        a = c[appearance]
        for name in names:
            t = c["targets"].get(name, {})
            a_view = c["targets"].get("toolbar_button", {}).get("view_in_window_from_top_left_xywh")
            view = t.get("view_in_window_from_top_left_xywh")
            if not isinstance(view, list):
                lines.append(f'"{appearance}.{name}" = {{ error = "{t.get("error", "no view")}" }}')
                continue
            states = {"normal": a["normal"]}
            if isinstance(a.get("hovered", {}).get(name), dict):
                states["hovered"] = a["hovered"][name]
            if isinstance(a.get("pressed", {}).get(name), dict):
                states["pressed"] = a["pressed"][name]
            for state_name, state in states.items():
                own = state.get("window", state)
                window = Image(root / own["file"])
                m = shape(window, view)
                m["backdrop_reaches"] = [reaches(window, Image(root / state[f"over_{b}"]["file"]), view)
                                         for b in ("black", "grey", "white")]
                if state_name == "normal" and name != "toolbar_button" and isinstance(a_view, list):
                    m["differs_from_toolbar_button"] = same_place(window, view, a_view)
                body = ", ".join(f"{k} = {value(v)}" for k, v in m.items())
                lines.append(f'"{appearance}.{state_name}.{name}" = {{ {body} }}')
        lines.append("")
    output.write_text("\n".join(lines))


if __name__ == "__main__":
    try:
        main()
    except (Bad, KeyError, OSError) as e:
        print(f"fit_toolbar_bezel.py: {e}", file=sys.stderr)
        sys.exit(1)
