"""The macOS labelled switch (SwiftUI Toggle with .toggleStyle(.switch)) from
os-probe.yml's switch-label job (tools/swiftui-host/SwitchLabel.swift):

    python3 -I measure_switch_label.py <switch-label-out>

Reads switch-label.toml and the captures. Per appearance and capture, per
Toggle item, in the item's whole frame cropped from the capture at the
backing scale: `bg` the crop's commonest pixel; the label's ink found on
each side of the switch's frame (the item's frame left of the switch and
right of it, each searched for columns and rows holding a pixel more than
INK levels, Euclidean RGB, from bg); `label_side` the side holding ink
("leading (left)", "trailing (right)", "both" or "none"); on the one side
with ink: `ink` the pixel there farthest from bg (the glyphs' core colour over
bg), `ink_box_pt_from_item_left`, and `same_as` the comparison items (Text
in the foreground styles primary to quaternary, Text at the system font of 11
to 15 pt, AppKit's NSTextField label; each cropped to its own whole frame)
whose ink box in the same capture is pixel-identical to the label's, and
those within NEAR levels in every channel (`within_3_levels_of`: on macOS 26
the off switch's shadow lightens or darkens the label's pixels by up to 3
levels); the label's view (CGDrawingView "0.0", where it has one) and the
layout gap (the switch's PlatformViewHost minX less that view's maxX); for a
leading label, the ink gap: the switch's drawn left edge (its frame's
leftmost column holding a pixel more than SWITCH_EDGE levels from bg) less
the label's rightmost ink, in points.

Per hovered and pressed capture: whether the host recorded the event's
delivery (`delivery`): an event the capture lists (`events`, the host's
local monitor: what the application received since the post before the
capture) of the posted type (hover: mouseMoved or mouseEntered; press:
leftMouseDown) in the host's window at the capture's `target_pt` within one
backing pixel, with the class of the view hitTest: found there; and the
pixels of the label's side and of the switch's frame that differ from the
rest capture (`label_changed_from_rest_px`, `switch_changed_from_rest_px`).
A count of 0 is written as unchanged only where the delivery is recorded;
else `…_changed_from_rest = "not measured (delivery not recorded)"`. Per
[[release]], whether the mouse-up's delivery was recorded likewise.

The DisclosureGroup (the expander's header), where the host drew one: per
capture, its label and its chevron as the rest capture placed them
([[disclosure_parts]], grown by 2 pt): the label's ink and the comparison
items within NEAR levels of it (as the Toggles'); the chevron's bg, ink and
the system colour (the host's [[colours]] of the appearance) whose composite
over bg is nearest the ink, with that distance; and per part the pixels that
differ from the rest capture, written as the Toggles' are.
Prints TOML. Where a crop falls outside a capture or holds no pixel, its
reads are "not measured" with the reason.
"""
import sys
import tomllib
from collections import Counter
from PIL import Image

out = sys.argv[1]
data = tomllib.load(open(f"{out}/switch-label.toml", "rb"))
scale = data["environment"]["backing_scale"]
items = {item["name"]: item for item in data["item"]}

INK = 24
# The macOS 26 switch's shadow reaches the label's pixels by up to 3 levels.
NEAR = 3
# The threshold the switch's drawn edge is found with, lower than INK: a
# choice of this script (run 45210835145's analysis used it), not a measured
# value.
SWITCH_EDGE = 8
HOVER_EVENTS = ("mouseMoved", "mouseEntered")
PRESS_EVENTS = ("leftMouseDown",)


def px(rect):
    x, y, w, h = rect
    return (round(x * scale), round(y * scale), round((x + w) * scale), round((y + h) * scale))


def view(item, path):
    return next((v for v in item["views"] if v["path"] == path), None)


def crop(image, rect):
    """The rect's pixels, or None where it falls outside the image or is
    empty."""
    box = px(rect)
    if box[0] < 0 or box[1] < 0 or box[2] > image.width or box[3] > image.height:
        return None
    if box[2] <= box[0] or box[3] <= box[1]:
        return None
    return image.crop(box).convert("RGBA")


def hexa(p):
    return "#" + "".join(f"{c:02x}" for c in p)


def dist(a, b):
    return sum((x - y) ** 2 for x, y in zip(a[:3], b[:3])) ** 0.5


def commonest(img):
    return Counter(img.get_flattened_data()).most_common(1)[0][0]


def ink_box(img, bg, threshold=INK):
    cols = [x for x in range(img.width) if any(dist(img.getpixel((x, y)), bg) > threshold for y in range(img.height))]
    rows = [y for y in range(img.height) if any(dist(img.getpixel((x, y)), bg) > threshold for x in range(img.width))]
    return (cols[0], rows[0], cols[-1] + 1, rows[-1] + 1) if cols and rows else None


def ink_of(img):
    """bg, the farthest pixel from it, and the ink box."""
    bg = commonest(img)
    ink = max(img.get_flattened_data(), key=lambda p: dist(p, bg))
    return bg, ink, ink_box(img, bg)


def sides(item):
    """The item's frame left and right of its switch's frame (each None
    where empty), or the whole frame as the left side of an item without a
    switch."""
    x, y, w, h = item["frame_pt"]
    switch = item["switch_frame_pt"]
    if not isinstance(switch, list):
        return [x, y, w, h], None
    left = [x, y, switch[0] - x, h] if switch[0] > x else None
    right_x = switch[0] + switch[2]
    right = [right_x, y, x + w - right_x, h] if x + w > right_x else None
    return left, right


def changed(before, after):
    if before is None or after is None or before.size != after.size:
        return None
    return sum(1 for a, b in zip(before.get_flattened_data(), after.get_flattened_data()) if a != b)


def delivery(capture, kinds):
    """The first event of `kinds` the capture lists in the host's window at
    its target within one backing pixel, as text, or None."""
    target = capture.get("target_pt")
    if not isinstance(target, list):
        return None
    for event in capture.get("events", []):
        at = event.get("location_pt")
        if event["type"] in kinds and isinstance(at, list) and \
                all(abs(a - b) <= 1 / scale for a, b in zip(at, target)):
            return f'{event["type"]} at {at} in window {event["window_number"]}, hit {event["hit"]}'
    return None


def report_change(name, count, delivered):
    if count is None:
        print(f'{name}_changed_from_rest = "not measured (no crop of the same size in both captures)"')
    elif count == 0 and delivered is None:
        print(f'{name}_changed_from_rest = "not measured (delivery not recorded)"')
    else:
        print(f"{name}_changed_from_rest_px = {count}")


def expected(state):
    if state.startswith("hovered_"):
        return HOVER_EVENTS
    if state.startswith("pressed_"):
        return PRESS_EVENTS
    return None


def near_references(glyphs, crops):
    levels = {}
    for r in references:
        if crops.get(r) is None:
            continue
        ref_box = ink_of(crops[r])[2]
        if ref_box and crops[r].crop(ref_box).size == glyphs.size:
            pairs = zip(crops[r].crop(ref_box).get_flattened_data(), glyphs.get_flattened_data())
            levels[r] = max(max(abs(p - q) for p, q in zip(a, b)) for a, b in pairs)
    return levels


references = [n for n in items if n.startswith("text_") or n.startswith("label_field")]
toggles = [n for n in items if n.startswith("toggle_")]
captures = {(c["appearance"], c["state"]): c for c in data["capture"]}
for (appearance, state), capture in captures.items():
    image = Image.open(f"{out}/{capture['file']}")
    rest_capture = captures.get((appearance, "normal"))
    rest = Image.open(f"{out}/{rest_capture['file']}") if rest_capture else None
    crops = {n: crop(image, items[n]["frame_pt"]) for n in references}
    kinds = expected(state)
    delivered = delivery(capture, kinds) if kinds else None
    for name in toggles:
        item = items[name]
        print(f'[["{appearance}".{state}.{name}]]')
        whole = crop(image, item["frame_pt"])
        if whole is None:
            print('label = "not measured (the item frame falls outside the capture)"\n')
            continue
        bg = commonest(whole)
        print(f'bg = "{hexa(bg)}"')
        left, right = sides(item)
        found = {}
        for side, rect in (("leading (left)", left), ("trailing (right)", right)):
            region = crop(image, rect) if rect else None
            box = ink_box(region, bg) if region is not None else None
            if box:
                found[side] = (rect, region, box)
        side = "none" if not found else "both" if len(found) == 2 else next(iter(found))
        print(f'label_side = "{side}"')
        label_rect = None
        if len(found) == 1:
            label_rect, region, box = found[side]
            glyphs = region.crop(box)
            ink = max(glyphs.get_flattened_data(), key=lambda p: dist(p, bg))
            levels = near_references(glyphs, crops)
            same = sorted(r for r, level in levels.items() if level == 0)
            near = sorted(r for r, level in levels.items() if level <= NEAR)
            from_left = label_rect[0] - item["frame_pt"][0]
            print(f'ink = "{hexa(ink)}"\nsame_as = {same}\nwithin_{NEAR}_levels_of = {near}')
            print(f"ink_box_pt_from_item_left = [{box[0] / scale + from_left}, {box[2] / scale + from_left}]")
            switch = item["switch_frame_pt"]
            if side == "leading (left)" and isinstance(switch, list):
                edge_region = crop(image, switch)
                cols = [] if edge_region is None else [
                    x for x in range(edge_region.width)
                    if any(dist(edge_region.getpixel((x, y)), bg) > SWITCH_EDGE for y in range(edge_region.height))]
                if cols:
                    track_left = switch[0] + cols[0] / scale
                    ink_right = label_rect[0] + box[2] / scale
                    print(f"switch_drawn_left_pt = {track_left}\nink_gap_pt = {track_left - ink_right}")
        host = next((v for v in item["views"] if "PlatformViewHost" in v["class"]), None)
        label_view = view(item, "0.0")
        if label_view and label_view["class"] == "CGDrawingView":
            lv = label_view["frame_pt"]
            print(f"label_view_pt = {lv}")
            if host:
                print(f"layout_gap_host_pt = {host['frame_pt'][0] - (lv[0] + lv[2])}")
        if host:
            print(f"switch_host_pt = {host['frame_pt']}")
        if kinds and capture.get("target") == name:
            print(f'delivery = "{delivered if delivered else "not recorded"}"')
        if rest is not None and state != "normal" and capture.get("target") == name:
            if label_rect is not None:
                report_change("label", changed(crop(rest, label_rect), crop(image, label_rect)), delivered)
            switch = item["switch_frame_pt"]
            if isinstance(switch, list):
                report_change("switch", changed(crop(rest, switch), crop(image, switch)), delivered)
        print()


def composite(colour, bg):
    """`colour` (#rrggbbaa) over the opaque bg, 8-bit."""
    value = [int(colour[i:i + 2], 16) for i in (1, 3, 5, 7)]
    a = value[3] / 255
    return tuple(round(c * a + b * (1 - a)) for c, b in zip(value[:3], bg[:3]))


parts = {p["appearance"]: p["parts"] for p in data.get("disclosure_parts", []) if isinstance(p["parts"], dict)}
colours = {c["appearance"]: c for c in data.get("colours", [])}
for (appearance, state), capture in captures.items():
    if appearance not in parts or (appearance, "normal") not in captures:
        continue
    image = Image.open(f"{out}/{capture['file']}")
    rest = Image.open(f"{out}/{captures[(appearance, 'normal')]['file']}")
    crops = {n: crop(image, items[n]["frame_pt"]) for n in references}
    kinds = expected(state)
    delivered = delivery(capture, kinds) if kinds else None
    print(f'[["{appearance}".{state}.disclosure]]')
    if kinds and capture.get("target", "").startswith("disclosure"):
        print(f'delivery = "{delivered if delivered else "not recorded"}"')
    for name in ("label", "chevron"):
        x, y, w, h = parts[appearance][f"{name}_pt"]
        rect = [x - 2, y - 2, w + 4, h + 4]
        here = crop(image, rect)
        if here is None:
            print(f'{name} = "not measured (its rect falls outside the capture)"')
            continue
        bg, ink, box = ink_of(here)
        print(f'{name}_rect_pt = {rect}\n{name}_bg = "{hexa(bg)}"\n{name}_ink = "{hexa(ink)}"')
        if state != "normal" and capture.get("target", "").startswith("disclosure"):
            report_change(name, changed(crop(rest, rect), here), delivered)
        if name == "label" and box:
            levels = near_references(here.crop(box), crops)
            print(f"label_within_{NEAR}_levels_of = {sorted(r for r, level in levels.items() if level <= NEAR)}")
        if name == "chevron" and appearance in colours:
            options = {k: v for k, v in colours[appearance].items() if k != "appearance"}
            best = min(options, key=lambda k: dist(composite(options[k], bg), ink))
            print(f'chevron_nearest_colour = "{best}" # {options[best]} over bg: '
                  f'{hexa(composite(options[best], bg) + (255,))}, distance {dist(composite(options[best], bg), ink):.1f}')
    print()

for release in data.get("release", []):
    up = None
    for event in release.get("events", []):
        if event["type"] == "leftMouseUp" and isinstance(event.get("location_pt"), list):
            up = f'leftMouseUp at {event["location_pt"]} in window {event["window_number"]}, hit {event["hit"]}'
            break
    print(f'[["{release["appearance"]}".release.{release["target"]}]]')
    print(f'delivery = "{up if up else "not recorded"}"\n')
