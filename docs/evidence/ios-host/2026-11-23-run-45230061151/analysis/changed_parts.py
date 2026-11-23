"""What a press changed in the iOS host's list set (tools/ios-host,
`set=lists`), and the inset-grouped card's edge, from os-probe.yml's ios-host
job:

    python3 -I changed_parts.py <ios-host-<image> folder>

Uses measure_lists.py's part rects (the first list cell's frame; the middle
third of the segmented control and of the tab bar; UIKit's thumbRect for the
slider) and its solved layers (solved/press1/, solved/run1/: straight RGBA
solved from the captures over black and white). Per pressed specimen and
appearance, over the whole part (no inset): `changed_px`, the pixels whose
solved RGBA differs by more than 2 levels in any channel from the normal
state's; `max_level_change`, the largest such difference in the
part; `changed_box_pt`, their bounding box in the part's points (x, y, w,
h from the part's top left); `normal_ink` and `pressed_ink`, among the
changed pixels, the one farthest (RGB distance, alpha weighted) from the
part's commonest normal colour, in each state; `pressed_commonest_changed`,
the commonest pressed RGBA among the changed pixels.

For the card (run1, normal): the solved RGBA at the part's horizontal middle
on the part's outermost three pixel rows at the top and the bottom and at its
vertical middle on the three outermost columns at the left and the right, and
the fill (the commonest RGBA inset 2 pt), so a stroke would show as edge
pixels that differ from the fill.
Prints TOML.
"""
import sys
import tomllib
from collections import Counter
from pathlib import Path
from PIL import Image

root = Path(sys.argv[1])


def hexa(p):
    return "#" + "".join(f"{c:02x}" for c in p)


def renders(run):
    data = tomllib.load(open(root / run / "index.toml", "rb"))
    return [r for r in data.get("render", []) if r.get("backdrop") == "system"]


def part(render):
    x, y, w, h = render["frame_pt"]
    cells = render.get("list_cell", [])
    if "slider_thumb_rect_pt" in render:
        return render["slider_thumb_rect_pt"]
    if render["specimen"] in ("segmented_control", "tab_bar"):
        return [w / 3, 0, w / 3, h]
    if cells:
        return cells[0]["frame_pt"]
    return [0, 0, w, h]


def crop(path, render, rect, inset=0.0):
    scale = render["cell_px"][2] / render["cell_pt"][2]
    fx, fy = render["frame_pt"][0], render["frame_pt"][1]
    x, y, w, h = rect
    box = (round((fx + x + inset) * scale), round((fy + y + inset) * scale),
           round((fx + x + w - inset) * scale), round((fy + y + h - inset) * scale))
    return Image.open(path).convert("RGBA").crop(box), scale


def solved(run, render):
    return root / "solved" / run / render["appearance"] / render["specimen"] / render["size"] / f'{render["state"]}.png'


def distance(p, q):
    return sum(((a * p[3] - b * q[3]) / 255) ** 2 for a, b in zip(p[:3], q[:3])) ** 0.5 + abs(p[3] - q[3])


entries = renders("press1")
normals = {(r["appearance"], r["specimen"]): r for r in entries if r["state"] == "normal"}
for render in entries:
    if render["state"] == "normal":
        continue
    normal = normals.get((render["appearance"], render["specimen"]))
    if normal is None or not solved("press1", render).exists() or not solved("press1", normal).exists():
        continue
    a, scale = crop(solved("press1", normal), normal, part(normal))
    b, _ = crop(solved("press1", render), render, part(render))
    if a.size != b.size:
        print(f'# {render["appearance"]} {render["specimen"]}: part sizes differ, skipped')
        continue
    pa, pb = list(a.get_flattened_data()), list(b.get_flattened_data())
    fill = Counter(pa).most_common(1)[0][0]
    changed = [i for i, (p, q) in enumerate(zip(pa, pb)) if max(abs(s - t) for s, t in zip(p, q)) > 2]
    print(f'[press1."{render["appearance"]}".{render["specimen"]}.{render["state"]}]')
    level = max((max(abs(s - t) for s, t in zip(p, q)) for p, q in zip(pa, pb)), default=0)
    print(f"normal_commonest = \"{hexa(fill)}\"\nchanged_px = {len(changed)}\nmax_level_change = {level}")
    if changed:
        xs = [i % a.width for i in changed]; ys = [i // a.width for i in changed]
        box = [min(xs) / scale, min(ys) / scale, (max(xs) + 1 - min(xs)) / scale, (max(ys) + 1 - min(ys)) / scale]
        print(f"changed_box_pt = {[round(v, 3) for v in box]}")
        n = max((pa[i] for i in changed), key=lambda p: distance(p, fill))
        m = max((pb[i] for i in changed), key=lambda p: distance(p, fill))
        print(f'normal_ink = "{hexa(n)}"\npressed_ink = "{hexa(m)}"')
        common = Counter(pb[i] for i in changed).most_common(1)[0]
        print(f'pressed_commonest_changed = "{hexa(common[0])}"\npressed_commonest_changed_px = {common[1]}')
    print()

for render in renders("run1"):
    if render["specimen"] != "card" or render["state"] != "normal" or not solved("run1", render).exists():
        continue
    rect = part(render)
    img, scale = crop(solved("run1", render), render, rect)
    inner, _ = crop(solved("run1", render), render, rect, 2.0)
    fill = Counter(inner.get_flattened_data()).most_common(1)[0][0]
    mx, my = img.width // 2, img.height // 2
    print(f'[run1."{render["appearance"]}".card.normal.edges]')
    print(f'fill = "{hexa(fill)}"')
    print(f"top_rows = {[hexa(img.getpixel((mx, y))) for y in range(3)]}")
    print(f"bottom_rows = {[hexa(img.getpixel((mx, img.height - 1 - y))) for y in range(3)]}")
    print(f"left_columns = {[hexa(img.getpixel((x, my))) for x in range(3)]}")
    print(f"right_columns = {[hexa(img.getpixel((img.width - 1 - x, my))) for x in range(3)]}")
    print()
