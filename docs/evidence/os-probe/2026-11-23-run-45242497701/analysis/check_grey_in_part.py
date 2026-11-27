"""The solved layer checked against the over-grey and window captures inside one target's part.

Per <appearance>.<state> of the target: the layer (`layer.file`, placed at `layer.file_rect_px_xywh`) composited
over the grey backdrop's read (`over_grey.backdrop_raw`) and over the window capture's (`backdrop_raw`, the window
background, which is not uniform where a title bar lies under the part, so that check is indicative only), compared
with those captures inside the target's part: the pixels whose bytes differ by more than 2 and the largest
difference. A layer that composites linearly over its backdrops (a fixed colour and alpha per pixel) gives a
difference of at most a few levels; a material that adapts to what lies behind it does not.

Usage (repository root): python3 -I check_grey_in_part.py <image folder> <sheet: controls|toolbar_sheet> <target>
"""
import importlib.util, sys, tomllib
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "mc", Path(__file__).resolve().parents[2] / "2026-11-23-run-45226132792/analysis/measure_controls.py")
mc = importlib.util.module_from_spec(spec); spec.loader.exec_module(mc)

image, sheet, target = Path(sys.argv[1]), sys.argv[2], sys.argv[3]
root = image / "button-pass-controls"
doc = tomllib.load(open(root / "probe.toml", "rb"))["macos"]["button_pass"]["controls"]
if sheet == "toolbar_sheet":
    doc = doc["toolbar_sheet"]
part = [round(v) for v in doc["targets"][target]["part_in_window_from_top_left_xywh"]]


def rgba(hexs):
    return [int(hexs[i:i + 2], 16) for i in (1, 3, 5, 7)]


for ap in ("aqua", "darkAqua"):
    for state in ("normal", "hovered", "pressed"):
        s = doc[ap][state] if state == "normal" else doc[ap][state][target]
        layer = s["layer"]
        lw, lh, lp = mc.read_png(root / layer["file"])
        ox, oy = layer["file_rect_px_xywh"][:2]
        out = []
        for key in ("over_grey", "window"):
            sub = s.get(key, s) if key == "over_grey" else (s["window"] if "window" in s else s)
            cap = sub["file"]
            cw, ch, cp = mc.read_png(root / cap)
            bg = rgba(sub["backdrop_raw"])
            over, worst = 0, 0
            for y in range(part[1], part[1] + part[3]):
                for x in range(part[0], part[0] + part[2]):
                    r, g, b, a = lp[y - oy][x - ox]
                    c3 = cp[y][x]
                    d = max(abs(([r, g, b][c] * a + bg[c] * (255 - a)) / 255 - c3[c]) for c in range(3))
                    worst = max(worst, d)
                    over += d > 2
            out.append(f"{key}: {over} px over 2, max {worst:.1f}")
        print(f"{ap}.{state}.{target}: " + "; ".join(out))
