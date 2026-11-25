"""The macOS labelled switch (SwiftUI Toggle with .toggleStyle(.switch)) from
os-probe.yml's switch-label job (tools/swiftui-host/SwitchLabel.swift):

    python3 -I measure_switch_label.py <switch-label-out>

Reads switch-label.toml and the captures. Per appearance and capture, per
Toggle item: the label's view (the item's CGDrawingView, "0.0") cropped from
the capture at the backing scale; `bg` its commonest pixel; `ink` the pixel
farthest from bg (the glyphs' core colour over bg); `same_as` the
comparison items (each cropped to its own whole frame) (Text in the foreground styles primary to quaternary, Text
at the system font of 11 to 15 pt, AppKit's NSTextField label) whose ink
box in the same capture is pixel-identical to the label's, and those within
NEAR levels in every channel (`within_3_levels_of`: on macOS 26 the off
switch's shadow lightens or darkens the label's pixels by up to 3 levels) (the same font, size and
colour drawn the same way); the label's ink box in points from the item's
left; the label side (its ink left of the switch's frame or not); the label's
view (CGDrawingView "0.0", where it has one) and the layout gap (the switch's
PlatformViewHost minX less that view's maxX); the ink gap (the switch's
leftmost pixel differing from bg less the label's rightmost ink pixel + 1),
in points. Also, per Toggle, whether the label crop and the switch's frame
differ between the rest capture and the hovered and pressed captures
(`label_changed_from_rest_px`, `switch_changed_from_rest_px`: the number of
pixels that differ; the switch's shows that the hover or press reached it).
Prints TOML.
"""
import sys
import tomllib
from collections import Counter
from PIL import Image

out = sys.argv[1]
data = tomllib.load(open(f"{out}/switch-label.toml", "rb"))
scale = data["environment"]["backing_scale"]
items = {item["name"]: item for item in data["item"]}


def px(rect):
    x, y, w, h = rect
    return (round(x * scale), round(y * scale), round((x + w) * scale), round((y + h) * scale))


def view(item, path):
    return next((v for v in item["views"] if v["path"] == path), None)


def label_rect(item):
    """The area the label can be in: the item's frame left of its switch
    (macOS 26 draws the label in the hosting view itself, with no view of its
    own), or the whole frame of an item without a switch."""
    x, y, w, h = item["frame_pt"]
    switch = item["switch_frame_pt"]
    right = switch[0] if isinstance(switch, list) else x + w
    return [x, y, right - x, h]


def crop(image, rect):
    box = px(rect)
    if box[2] > image.width or box[3] > image.height:
        return None
    return image.crop(box).convert("RGBA")


def hexa(p):
    return "#" + "".join(f"{c:02x}" for c in p)


def dist(a, b):
    return sum((x - y) ** 2 for x, y in zip(a[:3], b[:3])) ** 0.5


def ink_of(img):
    """bg, the farthest pixel from it, and the ink box: the columns and rows
    holding a pixel more than INK levels (Euclidean, RGB) from bg."""
    pixels = list(img.get_flattened_data())
    bg = Counter(pixels).most_common(1)[0][0]
    ink = max(pixels, key=lambda p: dist(p, bg))
    cols = [x for x in range(img.width) if any(dist(img.getpixel((x, y)), bg) > INK for y in range(img.height))]
    rows = [y for y in range(img.height) if any(dist(img.getpixel((x, y)), bg) > INK for x in range(img.width))]
    box = (cols[0], rows[0], cols[-1] + 1, rows[-1] + 1) if cols and rows else None
    return bg, ink, box


INK = 24
# The macOS 26 switch's shadow reaches the label's pixels by up to 3 levels.
NEAR = 3

references = [n for n in items if n.startswith("text_") or n.startswith("label_field")]
toggles = [n for n in items if n.startswith("toggle_")]
captures = {(c["appearance"], c["state"]): c for c in data["capture"]}
for (appearance, state), capture in captures.items():
    image = Image.open(f"{out}/{capture['file']}")
    crops = {n: crop(image, label_rect(items[n])) for n in items if label_rect(items[n])}
    for name in toggles:
        item = items[name]
        label = crops.get(name)
        if label is None:
            continue
        bg, ink, box = ink_of(label)
        glyphs = label.crop(box) if box else None
        levels = {}
        for r in references:
            if crops.get(r) is None or glyphs is None:
                continue
            ref_box = ink_of(crops[r])[2]
            if ref_box and crops[r].crop(ref_box).size == glyphs.size:
                pairs = zip(crops[r].crop(ref_box).get_flattened_data(), glyphs.get_flattened_data())
                levels[r] = max(max(abs(p - q) for p, q in zip(a, b)) for a, b in pairs)
        same = sorted(r for r, level in levels.items() if level == 0)
        near = sorted(r for r, level in levels.items() if level <= NEAR)
        print(f'[["{appearance}".{state}.{name}]]')
        print(f'bg = "{hexa(bg)}"\nink = "{hexa(ink)}"\nsame_as = {same}\nwithin_{NEAR}_levels_of = {near}')
        lx, ly, lw, lh = label_rect(item)
        if box:
            print(f"ink_box_pt_from_item_left = [{box[0] / scale + lx - item['frame_pt'][0]}, {box[2] / scale + lx - item['frame_pt'][0]}]")
        switch = item["switch_frame_pt"]
        host = next((v for v in item["views"] if "PlatformViewHost" in v["class"]), None)
        label_view = view(item, "0.0")
        if label_view and label_view["class"] == "CGDrawingView":
            lv = label_view["frame_pt"]
            print(f"label_view_pt = {lv}")
            if host:
                print(f"layout_gap_host_pt = {host['frame_pt'][0] - (lv[0] + lv[2])}")
        if host:
            print(f"switch_host_pt = {host['frame_pt']}")
        if isinstance(switch, list) and box:
            side = "leading (left)" if lx + box[2] / scale <= switch[0] else "trailing (right)"
            print(f'label_side = "{side}"')
            region = crop(image, switch)
            cols = [x for x in range(region.width)
                    if any(dist(region.getpixel((x, y)), bg) > 8 for y in range(region.height))]
            if cols:
                track_left = switch[0] + cols[0] / scale
                ink_right = lx + box[2] / scale
                print(f"switch_drawn_left_pt = {track_left}\nink_gap_pt = {track_left - ink_right}")
        rest = captures.get((appearance, "normal"))
        if rest and state != "normal":
            base = crop(Image.open(f"{out}/{rest['file']}"), label_rect(item))
            changed = sum(1 for a, b in zip(base.get_flattened_data(), label.get_flattened_data()) if a != b)
            print(f"label_changed_from_rest_px = {changed}")
            if isinstance(switch, list):
                before = crop(Image.open(f"{out}/{rest['file']}"), switch)
                after = crop(image, switch)
                moved = sum(1 for a, b in zip(before.get_flattened_data(), after.get_flattened_data()) if a != b)
                print(f"switch_changed_from_rest_px = {moved}")
        print()


def composite(colour, bg):
    """`colour` (#rrggbbaa) over the opaque bg, 8-bit."""
    value = [int(colour[i:i + 2], 16) for i in (1, 3, 5, 7)]
    a = value[3] / 255
    return tuple(round(c * a + b * (1 - a)) for c, b in zip(value[:3], bg[:3]))


# The DisclosureGroup (the expander's header), where the host drew one: per
# capture, its label and its chevron as the rest capture placed them
# ([[disclosure_parts]], grown by 2 pt): the label's ink and the comparison
# items within NEAR levels of it (as the Toggles'); the chevron's bg, ink and
# the system colour (the host's [[colours]] of the appearance) whose
# composite over bg is nearest the ink, with that distance; and per part the
# pixels that differ from the rest capture.
parts = {p["appearance"]: p["parts"] for p in data.get("disclosure_parts", []) if isinstance(p["parts"], dict)}
colours = {c["appearance"]: c for c in data.get("colours", [])}
for (appearance, state), capture in captures.items():
    if appearance not in parts:
        continue
    image = Image.open(f"{out}/{capture['file']}")
    rest = Image.open(f"{out}/{captures[(appearance, 'normal')]['file']}")
    crops = {n: crop(image, label_rect(items[n])) for n in references}
    print(f'[["{appearance}".{state}.disclosure]]')
    for name in ("label", "chevron"):
        x, y, w, h = parts[appearance][f"{name}_pt"]
        rect = [x - 2, y - 2, w + 4, h + 4]
        here = crop(image, rect)
        bg, ink, box = ink_of(here)
        print(f'{name}_rect_pt = {rect}\n{name}_bg = "{hexa(bg)}"\n{name}_ink = "{hexa(ink)}"')
        base = crop(rest, rect)
        changed = sum(1 for a, b in zip(base.get_flattened_data(), here.get_flattened_data()) if a != b)
        print(f"{name}_changed_from_rest_px = {changed}")
        if name == "label" and box:
            glyphs = here.crop(box)
            near = []
            for r in references:
                if crops.get(r) is None:
                    continue
                ref_box = ink_of(crops[r])[2]
                if ref_box and crops[r].crop(ref_box).size == glyphs.size:
                    pairs = zip(crops[r].crop(ref_box).get_flattened_data(), glyphs.get_flattened_data())
                    if max(max(abs(p - q) for p, q in zip(a, b)) for a, b in pairs) <= NEAR:
                        near.append(r)
            print(f"label_within_{NEAR}_levels_of = {sorted(near)}")
        if name == "chevron" and appearance in colours:
            options = {k: v for k, v in colours[appearance].items() if k != "appearance"}
            best = min(options, key=lambda k: dist(composite(options[k], bg), ink))
            print(f'chevron_nearest_colour = "{best}" # {options[best]} over bg: '
                  f'{hexa(composite(options[best], bg) + (255,))}, distance {dist(composite(options[best], bg), ink):.1f}')
    print()
