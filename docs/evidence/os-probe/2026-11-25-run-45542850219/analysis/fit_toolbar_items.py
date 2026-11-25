#!/usr/bin/env python3
"""The toolbar items sheet's fitted reads (M2.FS4c11): the labels of the unselected and the disabled image item, and,
where the selected item's fill is not one layer (macOS 26), the selected item's rounded rect over each backdrop.

Run from the repository root (Python 3.11+, the standard library only; `measure_controls.py` beside it gives the PNG
reader, `colour_of` and `edge_radius`):

    python3 <run>/analysis/fit_toolbar_items.py <capture folder> <probe.toml> OUTPUT

<capture folder> is an image's `button-pass-controls` folder of the run's artifact (the PNGs are not kept in the
repository: `CAPTURES.sha256` names them), <probe.toml> that folder's probe.toml. Per appearance (aqua, darkAqua) it
reads the normal state's four captures of the sheet (`<appearance>/controls_toolbar_items/normal.png`, over the
window's own background, and `normal-over-{black,grey,white}.png`) and each target's part
(`[macos.button_pass.controls.toolbar_items_sheet.targets.<t>]` `part_in_window_from_top_left_xywh`; the backing
scale is 1, a point is a pixel).

`<appearance>.<item>` for the unselected and the disabled item, the label:
- `label_rect`: x, y, w, h in the window's pixels: in the capture over the window, the part's pixels of alpha 255
  that differ from the part's background (its pixel 3 rows above its bottom edge, mid-width) by more than 24 levels in
  a channel; their rows form runs of consecutive rows, the last (lowest) run is the label's (the one above it the
  icon's); the rect spans those rows and the columns holding such pixels in them, grown by 1 px on each side;
- per backdrop pair (black and white, black and grey, grey and white): the layer solved per pixel from the two
  captures (1 - alpha the mean over the channels of (P2 - P1) / (b2 - b1), the colour (P1 - (1 - alpha) b1) / alpha;
  b the backdrop's level, 0, 128 or 255), `label` read from it as `measure_controls.py`'s `colour_of` reads a label
  with no fill (the mean colour of the rect's pixels of the largest alpha, within 2/255 of it, and that alpha),
  `label_pixels` how many, `max_error_over_window` and `max_error_over_<third>`: the largest channel difference
  between the layer composited over the window's background (that pixel, `window_background`) or over the third
  backdrop and the capture there, over the rect's pixels. A fixed layer gives the same label from every pair and
  errors of a level or two.

`<appearance>.selected_rect` for the selected item (written where `--rect` is given: its fill composited over the
backdrops is not one layer): in the capture over black, the part's pixels whose red level is at least half the
commonest non-zero level there (`black_interior`): `box` (x, y, w, h): the longest run of consecutive rows holding
such pixels across at least half the part's width, and of columns holding them down at least half its height; per backdrop (window, black,
grey, white) `interior` the commonest colour of the box inset by 3 px (`interior_share` its share), `rim` the box's
outermost pixel and the next along its middle column from the top (the rim), and `outside` the pixel 2 px above the
box's top edge there; `corner_<corner>`: per row from the box's edge, the pixels before the first of red level at
least half `black_interior`, over 29 rows, and `measure_controls.py`'s `edge_radius` of those counts
(`radius_px` [smallest, largest], `rms_px`).
"""

import sys
from collections import Counter
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from measure_controls import colour_of, edge_radius, hexc, read_png  # noqa: E402

LEVEL = {"black": 0, "grey": 128, "white": 255}
PAIRS = (("black", "white"), ("black", "grey"), ("grey", "white"))


class Image:
    def __init__(self, path: Path):
        self.w, self.h, self.rows = read_png(path)

    def px(self, x, y):
        return self.rows[y][x]


class Solved:
    """A layer of straight (r, g, b, a) pixels, as `colour_of` reads one."""

    def __init__(self, pixels):
        self.pixels = pixels

    def px(self, x, y):
        return self.pixels.get((x, y))


def part_of(toml_text: str, target: str):
    head = f"[macos.button_pass.controls.toolbar_items_sheet.targets.{target}]"
    start = toml_text.index(head)
    for line in toml_text[start:].splitlines()[1:]:
        if line.startswith("["):
            break
        if line.startswith("part_in_window_from_top_left_xywh"):
            return [round(float(v)) for v in line.split("=", 1)[1].strip(" []").split(",")]
    raise SystemExit(f"{target}: no part_in_window_from_top_left_xywh")


def label_rect(window: Image, part):
    x0, y0, w, h = part
    background = window.px(x0 + w // 2, y0 + h - 3)
    rows, cols = {}, {}
    for y in range(y0, y0 + h):
        for x in range(x0, x0 + w):
            p = window.px(x, y)
            if p[3] == 255 and max(abs(p[c] - background[c]) for c in range(3)) > 24:
                rows.setdefault(y, []).append(x)
    if not rows:
        return None
    runs = []
    for y in sorted(rows):
        if runs and y == runs[-1][-1] + 1:
            runs[-1].append(y)
        else:
            runs.append([y])
    last = runs[-1]
    xs = [x for y in last for x in rows[y]]
    return [min(xs) - 1, last[0] - 1, max(xs) - min(xs) + 3, last[-1] - last[0] + 3]


def solve(first: Image, second: Image, a: str, b: str, rect):
    x0, y0, w, h = rect
    la, lb = LEVEL[a], LEVEL[b]
    out = {}
    for y in range(y0, y0 + h):
        for x in range(x0, x0 + w):
            p, q = first.px(x, y), second.px(x, y)
            keep = sum((q[c] - p[c]) / (lb - la) for c in range(3)) / 3
            alpha = min(1.0, max(0.0, 1 - keep))
            if alpha <= 0:
                out[(x, y)] = (0, 0, 0, 0)
                continue
            colour = [min(255.0, max(0.0, (p[c] - keep * la) / alpha)) for c in range(3)]
            out[(x, y)] = tuple(colour) + (255 * alpha,)
    return out


def composite_error(layer, backdrop, capture: Image, rect):
    x0, y0, w, h = rect
    worst = 0.0
    for y in range(y0, y0 + h):
        for x in range(x0, x0 + w):
            p = layer[(x, y)]
            a = p[3] / 255
            under = backdrop(x, y)
            seen = capture.px(x, y)
            for c in range(3):
                worst = max(worst, abs(a * p[c] + (1 - a) * under[c] - seen[c]))
    return round(worst, 1)


def label_reads(caps, part):
    rect = label_rect(caps["window"], part)
    if rect is None:
        return {"error": "no label pixels"}
    window_bg = caps["window"].px(part[0] + part[2] // 2, part[1] + part[3] - 3)
    out = {"label_rect": rect, "window_background": hexc(window_bg[:3])}
    for a, b in PAIRS:
        layer = solve(caps[a], caps[b], a, b, rect)
        colour, count = colour_of(Solved(layer), rect, None)
        third = next(k for k in LEVEL if k not in (a, b))
        level = LEVEL[third]
        out[f"{a}_{b}.label"] = colour or "none"
        out[f"{a}_{b}.label_pixels"] = count
        out[f"{a}_{b}.max_error_over_window"] = composite_error(
            layer, lambda x, y: window_bg, caps["window"], rect)
        out[f"{a}_{b}.max_error_over_{third}"] = composite_error(
            layer, lambda x, y, v=level: (v, v, v), caps[third], rect)
    return out


def rect_reads(caps, part):
    x0, y0, w, h = part
    black = caps["black"]
    levels = Counter(black.px(x, y)[0] for y in range(y0, y0 + h) for x in range(x0, x0 + w))
    interior = next((v for v, _ in levels.most_common() if v > 0), None)
    if interior is None:
        return {"error": "nothing drawn over black"}
    half = interior / 2
    inside = {(x, y) for y in range(y0, y0 + h) for x in range(x0, x0 + w) if black.px(x, y)[0] >= half}

    def longest(lines):
        runs = []
        for v in lines:
            if runs and v == runs[-1][-1] + 1:
                runs[-1].append(v)
            else:
                runs.append([v])
        return max(runs, key=len) if runs else None

    ys = longest([y for y in range(y0, y0 + h) if sum((x, y) in inside for x in range(x0, x0 + w)) * 2 >= w])
    xs = longest([x for x in range(x0, x0 + w) if sum((x, y) in inside for y in range(y0, y0 + h)) * 2 >= h])
    if ys is None or xs is None:
        return {"error": "no rect over black"}
    bx0, bx1, by0, by1 = xs[0], xs[-1], ys[0], ys[-1]
    out = {"black_interior": interior, "box": [bx0, by0, bx1 - bx0 + 1, by1 - by0 + 1]}
    mid = (bx0 + bx1) // 2
    for name, cap in caps.items():
        inner = Counter(cap.px(x, y)[:3] for y in range(by0 + 3, by1 - 2) for x in range(bx0 + 3, bx1 - 2))
        colour, count = inner.most_common(1)[0]
        out[f"{name}.interior"] = hexc(colour)
        out[f"{name}.interior_share"] = round(count / sum(inner.values()), 3)
        out[f"{name}.rim"] = [hexc(cap.px(mid, by0)[:3]), hexc(cap.px(mid, by0 + 1)[:3])]
        out[f"{name}.outside"] = hexc(cap.px(mid, by0 - 2)[:3]) if by0 >= 2 else "outside"
    span = 29
    for corner, ys, from_left in (("top_left", range(by0, by0 + span), True),
                                  ("top_right", range(by0, by0 + span), False),
                                  ("bottom_left", range(by1, by1 - span, -1), True),
                                  ("bottom_right", range(by1, by1 - span, -1), False)):
        counts = []
        for y in ys:
            xs = range(bx0, bx1 + 1) if from_left else range(bx1, bx0 - 1, -1)
            n = 0
            for x in xs:
                if black.px(x, y)[0] >= half:
                    break
                n += 1
            counts.append(n)
        radius, rms = edge_radius(counts)
        out[f"corner_{corner}"] = counts
        out[f"corner_{corner}_radius_px"] = radius
        out[f"corner_{corner}_rms_px"] = round(rms, 3)
    return out


def value(v):
    if isinstance(v, str):
        return f'"{v}"'
    if isinstance(v, list):
        return "[" + ", ".join(value(x) for x in v) + "]"
    return str(v)


def main():
    args = [a for a in sys.argv[1:] if a != "--rect"]
    if len(args) != 3:
        raise SystemExit(__doc__)
    folder, toml_path, output = Path(args[0]), Path(args[1]), Path(args[2])
    text = toml_path.read_text(encoding="utf-8")
    lines = ["# fit_toolbar_items.py's reads (its docstring gives the method)", ""]
    for appearance in ("aqua", "darkAqua"):
        sheet = folder / appearance / "controls_toolbar_items"
        caps = {"window": Image(sheet / "normal.png")}
        for name in LEVEL:
            caps[name] = Image(sheet / f"normal-over-{name}.png")
        for item in ("unselected", "disabled"):
            lines.append(f"[{appearance}.{item}]")
            lines += [f'"{k}" = {value(v)}' if "." in k else f"{k} = {value(v)}"
                      for k, v in label_reads(caps, part_of(text, item)).items()]
            lines.append("")
        if "--rect" in sys.argv:
            lines.append(f"[{appearance}.selected_rect]")
            lines += [f'"{k}" = {value(v)}' if "." in k else f"{k} = {value(v)}"
                      for k, v in rect_reads(caps, part_of(text, "selected")).items()]
            lines.append("")
    output.write_text("\n".join(lines), encoding="utf-8")


if __name__ == "__main__":
    main()
