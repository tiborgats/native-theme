"""The iOS host's list set (tools/ios-host, `set=lists`) from os-probe.yml's
ios-host job:

    python3 -I measure_lists.py <ios-host-<image> folder>

Reads press1/index.toml and run1/index.toml (the [[render]] entries of the
system backdrop: the cell's points and pixels, the specimen's frame in the
cell, UIKit's [[render.list_cell]] reads and the [[render.view]] tree) and
solved/press1/, solved/run1/ (the layer solved from the captures over black
and white, solve.swift: straight RGBA, alpha the channels' mean). Per
specimen, appearance and state, for its part:

- table_row, sidebar_row, outline_header, card, switch_row: the first list
  cell's frame (UIKit's `frame_pt`);
- segmented_control, tab_bar: the middle third of the specimen's frame (the
  middle segment, the middle item);
- switch_form: the first list cell where UIKit reports one, else the
  specimen's frame;
- slider: the thumb's rect (UIKit's thumbRect, `slider_thumb_rect_pt`);

`fill` is the commonest solved RGBA in the part inset by INSET pt and its
share of those pixels; `fill_screen` the commonest pixel of the capture over
the system background (`screen/…/system/<state>.png`) in the same rect;
`differs_from_normal` the number of the part's solved pixels that differ by
more than 2 levels from the normal state's. For the switch rows, the
label (the UILabel whose text is "Label"): its font and text colour as UIKit
reports them, its frame, its intrinsic width, and the UISwitch's frame; the
gap from the label's text end (frame x + intrinsic width) to the switch. For
the card, the corner: per pixel row from the part's top, the first column
whose solved alpha reaches half the fill's, and the radius of the circle
through those points (least squares, rows down to the radius).
Prints TOML.
"""
import sys
import tomllib
from collections import Counter
from pathlib import Path
from PIL import Image

root = Path(sys.argv[1])
INSET = 2.0


def hexa(p):
    return "#" + "".join(f"{c:02x}" for c in p)


def renders(run):
    path = root / run / "index.toml"
    if not path.exists():
        print(f'# {run}: not measured (no {run}/index.toml)\n')
        return []
    data = tomllib.load(open(path, "rb"))
    return [r for r in data.get("render", []) if r.get("backdrop") == "system"]


def part(render):
    """The part's rect in the specimen's points."""
    x, y, w, h = render["frame_pt"]
    name = render["specimen"]
    cells = render.get("list_cell", [])
    if "slider_thumb_rect_pt" in render:
        return render["slider_thumb_rect_pt"]
    if name in ("segmented_control", "tab_bar"):
        return [w / 3, 0, w / 3, h]
    if cells:
        return cells[0]["frame_pt"]
    return [0, 0, w, h]


def region(image, render, rect, inset):
    scale = render["cell_px"][2] / render["cell_pt"][2]
    fx, fy = render["frame_pt"][0], render["frame_pt"][1]
    x, y, w, h = rect
    box = (round((fx + x + inset) * scale), round((fy + y + inset) * scale),
           round((fx + x + w - inset) * scale), round((fy + y + h - inset) * scale))
    return image.crop(box), scale


def commonest(img):
    pixels = list(img.convert("RGBA").get_flattened_data())
    value, count = Counter(pixels).most_common(1)[0]
    return value, count / max(1, len(pixels))


def corner(img, alpha_level):
    """Per row from the top, the first column whose alpha reaches
    alpha_level; the radius of the circle tangent to the top and left edges
    through those points (least squares over r)."""
    points = []
    for y in range(img.height // 2):
        for x in range(img.width // 2):
            if img.getpixel((x, y))[3] >= alpha_level:
                points.append((x, y))
                break
    best = None
    for tenth in range(10, 10 * min(img.width, img.height) // 2):
        r = tenth / 10
        used = [(x, y) for x, y in points if y + 0.5 < r]
        if len(used) < 3:
            continue
        err = sum(((r - (x + 0.5)) ** 2 + (r - (y + 0.5)) ** 2 - r * r) ** 2 for x, y in used) / len(used)
        if best is None or err < best[1]:
            best = (r, err)
    return points, best


def label_glyphs(render, screen_path):
    """The label's glyphs in the capture over the system background: in
    the first list cell, left of the UISwitch (from the view tree), the
    columns and rows holding a pixel more than INK levels from the cell's
    commonest pixel; the crop of that box and the box in pixels of the
    capture."""
    cells = render.get("list_cell", [])
    toggle = next((v for v in render.get("view", []) if "is_on" in v), None)
    if not cells or toggle is None:
        return None
    x, y, w, h = cells[0]["frame_pt"]
    rect = [x, y, toggle["frame_pt"][0] - x, h]
    img, _ = region(Image.open(screen_path).convert("RGBA"), render, rect, 0)
    bg = commonest(img)[0]
    far = lambda p: sum((a - b) ** 2 for a, b in zip(p[:3], bg[:3])) ** 0.5 > INK
    cols = [c for c in range(img.width) if any(far(img.getpixel((c, r))) for r in range(img.height))]
    rows = [r for r in range(img.height) if any(far(img.getpixel((c, r))) for c in range(img.width))]
    if not cols:
        return None
    box = (cols[0], rows[0], cols[-1] + 1, rows[-1] + 1)
    return img.crop(box), box


INK = 24
labels = {}

for run in ("press1", "run1"):
    entries = renders(run)
    normals = {(r["appearance"], r["specimen"]): r for r in entries if r["state"] == "normal"}
    for render in entries:
        key = f'{run}."{render["appearance"]}".{render["specimen"]}.{render["state"]}'
        solved_path = root / "solved" / run / render["appearance"] / render["specimen"] / render["size"] / f'{render["state"]}.png'
        screen_path = root / run / render["screen"]
        print(f"[{key}]")
        rect = part(render)
        print(f"part_pt = {[round(v, 3) for v in rect]}")
        if render.get("press_number") is not None:
            value = render.get("highlighted_at_capture", "absent")
            print(f'highlighted_at_capture = {str(value).lower() if isinstance(value, bool) else chr(34) + value + chr(34)}')
        if solved_path.exists():
            solved = Image.open(solved_path).convert("RGBA")
            inner, scale = region(solved, render, rect, INSET)
            fill, share = commonest(inner)
            print(f'fill = "{hexa(fill)}"\nfill_share = {share:.3f}')
            normal = normals.get((render["appearance"], render["specimen"]))
            if normal is not None and normal is not render:
                npath = root / "solved" / run / render["appearance"] / render["specimen"] / render["size"] / "normal.png"
                if npath.exists():
                    base, _ = region(Image.open(npath).convert("RGBA"), normal, part(normal), INSET)
                    if base.size == inner.size:
                        moved = sum(1 for a, b in zip(base.get_flattened_data(), inner.get_flattened_data())
                                    if max(abs(p - q) for p, q in zip(a, b)) > 2)
                        print(f"differs_from_normal_px = {moved}")
            if render["specimen"] == "card":
                whole, _ = region(solved, render, rect, 0)
                points, best = corner(whole, fill[3] / 2)
                print(f"corner_first_columns_px = {[x for x, _ in points[:40]]}")
                if best:
                    print(f"corner_radius_px = {best[0]}\ncorner_radius_pt = {best[0] / scale:.2f}\ncorner_fit_mse = {best[1]:.3f}")
        else:
            print('fill = "no solved layer"')
        if screen_path.exists():
            shot, _ = region(Image.open(screen_path), render, rect, INSET)
            value, share = commonest(shot)
            print(f'fill_screen = "{hexa(value)}"\nfill_screen_share = {share:.3f}')
        for cell in render.get("list_cell", [])[:1]:
            for name in ("background_color", "background_corner_radius_pt", "background_stroke_width_pt",
                         "background_stroke_color", "text_font", "text_color", "default_disabled_text_color",
                         "state_highlighted", "state_selected", "state_disabled"):
                if name in cell:
                    value = cell[name]
                    if isinstance(value, dict) and "size_pt" in value:
                        value = f'{value["name"]} {value["size_pt"]} pt, weight trait {value["weight"]}'
                    elif isinstance(value, dict):
                        value = value.get("hex", value)
                    print(f"uikit_{name} = {value!r}".replace("'", '"').replace("True", "true").replace("False", "false"))
        views = render.get("view", [])
        label = next((v for v in views if v.get("text") == "Label"), None)
        toggle = next((v for v in views if "is_on" in v), None)
        if label:
            font = label.get("font")
            color = label.get("text_color")
            font = f'{font["name"]} {font["size_pt"]} pt, weight trait {font["weight"]}' if isinstance(font, dict) else font
            print(f'label_font = "{font}"')
            print(f'label_text_color = "{color.get("hex") if isinstance(color, dict) else color}"')
            print(f'label_frame_pt = {label["frame_pt"]}\nlabel_intrinsic_pt = {label.get("intrinsic_size_pt")}')
            if toggle:
                end = label["frame_pt"][0] + label["intrinsic_size_pt"][0]
                print(f'switch_frame_pt = {toggle["frame_pt"]}\nswitch_is_on = {str(toggle["is_on"]).lower()}')
                print(f'switch_is_enabled = {str(toggle["is_enabled"]).lower()}')
                print(f'gap_text_end_to_switch_pt = {toggle["frame_pt"][0] - end:.3f}')
        if render["specimen"] in ("switch_row", "switch_form") and screen_path.exists():
            glyphs = label_glyphs(render, screen_path)
            labels[(run, render["appearance"], render["specimen"], render["state"])] = glyphs
            if glyphs is not None:
                print(f"label_ink_box_px = {list(glyphs[1])}")
        print()

# The SwiftUI Form Toggle's label against the UIKit list row's, per run,
# appearance and state: the UIKit states whose label glyphs equal the
# SwiftUI one's within 3 levels in every channel.
for (run, appearance, specimen, state), glyphs in sorted(labels.items()):
    if specimen != "switch_form" or glyphs is None:
        continue
    same = []
    for (run2, appearance2, specimen2, state2), other in labels.items():
        if (run2, appearance2, specimen2) != (run, appearance, "switch_row") or other is None:
            continue
        if other[0].size == glyphs[0].size:
            level = max(max(abs(p - q) for p, q in zip(a, b))
                        for a, b in zip(other[0].get_flattened_data(), glyphs[0].get_flattened_data()))
            if level <= 3:
                same.append(state2)
    print(f'[{run}."{appearance}".switch_form.{state}.label_vs_uikit_row]\nequal_within_3_levels_to = {sorted(same)}\n')
