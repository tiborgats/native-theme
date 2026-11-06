#!/usr/bin/env python3
"""Reads the UIButton measurements of the iOS host's run 41020880059 (tools/ios-host) into the values the facts cite.

Run from the repository root, once per image folder:

    python3 -I docs/evidence/ios-host/2026-10-30-run-41020880059/analysis/read_buttons.py \
        docs/evidence/ios-host/2026-10-30-run-41020880059/ios-host-macos-26

Reads the folder's run1/index.toml (the host's render pass: per cell, the button's `UIButton.Configuration` as
`updated(for:)` resolves it in the cell's state, its title font and colour, and its view tree), solved/run1.toml (the
host's solve of each layer from the captures over opaque black and opaque white, tools/ios-host/solve.swift),
differ-levels.txt (the files that differ between the render pass's two runs), press1/index.toml with the press
pass's crops and the render pass's highlighted crops it compares, and focus1/index.toml (the iPad's focus pass).
Writes <folder>/analysis.toml. Needs Pillow (the press comparison only).

Cells (`[cells.<appearance>.<specimen>.<size>.<state>]`): the system-background column of every configuration
specimen (a style alone, or with one variant: `<style>__<variant>`, the host's `+` written `__`). Values are spelled as
a fact's value for its key (lengths unitless, in points, which are logical px; font sizes unitless, in points):
- `fill`: the bezel's colour, `#rrggbbaa`, or "absent" where the solve finds no bezel (the commonest solved colour
  covers less than half the frame's pixels). The solved layer's commonest colour among the covering pixels
  (solved/run1.toml `common`) is the drawn colour; where the configuration's `resolved_background_color` composites
  over black and over white to within one level of that solved colour's composites in every channel, the
  configuration's exact colour is written (`fill_source = "configuration"`), else the solved one (`"solve"`).
  `fill_solved`, `fill_configuration` and `fill_pixels` record both readings. Only cells whose solve passes the
  host's single-layer check (`alpha_spread_levels` at most 2), whose crops do not differ between the runs
  (differ-levels.txt) and that are no glass style and show no activity indicator (it never stops) get a `fill`: otherwise `fill = "unread"` and `unread` says why.
- `label_fill`: the title's colour, `title_color` (`UILabel.textColor` in the state), `label_check` whether a
  solved colour within 3 levels of the title composited over `fill` occurs among the solve's commonest colours
  (the title's fully covered pixels), on the cells with a read `fill` and a title (3 levels: a solve of a low alpha amplifies the capture's rounding).
- `font_family`, `font_size`: the title font's family and point size (`title_font`).
- `padding_top`, `padding_left`, `padding_bottom`, `padding_right`: the configuration's `content_insets_pt`
  (top, leading, bottom, trailing; the capture is left to right, so leading is left).
- `corner`: the radius the view tree draws the bezel with: the corner radius of the first view holding the bezel's
  colour (iOS 18) or of the button's own layer where it states one (iOS 26), as a number, or
  `{ full = { maximum_px = "absent" } }` where it is half the frame's smaller side (to 0.01 pt: a capsule); where the
  cell draws no bezel, the same specimen's selected cell's (its configuration is the same; `corner_from`).
  `corner_curve`: that view's layer `cornerCurve`.
- `stroke`: "absent" where the configuration's resolved stroke colour has alpha 0 or its width is 0.
- `shadow`: "absent" where no view of the tree has a shadow opacity above 0.
- `opacity`: the button view's alpha times its layer opacity.
- `icon_placement`: the configuration's `image_placement`.
- `icon_text_gap`, `subtitle_gap`: the configuration's `image_padding_pt` and `title_padding_pt` (every cell);
  `icon_text_gap_frames` (an image leading the title) the gap between the image view's and the title's frames, and
  `subtitle_gap_frames` (the subtitle specimen) the gap between the two labels' frames.
- `subtitle_fill`, `subtitle_font_family`, `subtitle_font_size` (the subtitle specimen): the second label's colour and
  font.
- `icon_fill` (an image specimen): the image view's tint where its composite over `fill` occurs in the solve.
- `menu_indicator_gap`, `menu_indicator_margin_left`, `menu_indicator_margin_right`, `menu_indicator_anchor`,
  `menu_indicator_size` (a drawn indicator): the gap from the title's frame to the indicator's, the space between the
  indicator and the trailing content inset (none is set before it: 0), "inline" where the indicator is centred (to a device pixel) on the
  button's height, "auto" (its box is its image's).
- `label` (an image with no title): "absent" where the view tree holds no UILabel.
- `menu_indicator`: "drawn" where the view tree holds an image view the specimen sets no image for (a menu's
  indicator), else "absent".

`[same]`: the readings two cells share, for the state model: `mini_small.<appearance>.<style>.<state>` true where
mini's and small's configuration, title, view tree and solve summary are equal; `pairs.<appearance>.<style>.<size>.<state>`
true where borderless, bordered, borderedTinted and borderedProminent read as plain, gray, tinted and filled.

`[press.<appearance>.<specimen>.<state>]`: the press pass (a real touch held while the screenshot was taken) against
the render pass's `isHighlighted` cell (no touch): `max_level_difference` and `pixels_over_2_levels` over the button's
frame and 2 pt around it, both crops from the system column at medium; `title_color_same` and `held` (the touch was
down and `isHighlighted` true at the capture).

`[forms.<appearance>.<specimen>.<state>]`: the specimens that are no configuration (the system types, the bar
items, the back button, the paste control, the route picker), system column: `fill` by the cells' rule (a bar
item's or back button's from its own subtree of the view tree, `fill_views`, the bar's material being the bar's),
or "unread" with `unread` saying why; `label_fill`, `font_family`, `font_size`, `font_weight` (UIFont's weight
trait) of the item's title label; `icon_fill`, the glyph's colour: the solve's commonest colour where no bezel is
drawn (`glyph_solved`), else the glyph view's tint where its composite over the bezel occurs in the solve
(`icon_check`), or a bar item's glyph view's tint; `padding_*` of a UIButton type (its glyph views inside the
bounds); `min_width`, `min_height` "auto" where the frame is the intrinsic size; `stroke`, `shadow`, `opacity` from
the item's subtree. The title fonts' `font_weight` in `[cells]` likewise.

`[text_styles.<style>.<legibility>]`: each text style's font at the default category (L): `line_height` its
ascender less its descender plus its leading (the line spacing TextKit lays it out with), `weight` its weight trait,
`name`.

`[focus]`: the iPad's focus pass: `focus_system`, `interface_idiom`, and `can_become_focused_any` (whether any button
could become focused).
"""
import sys
import tomllib
from pathlib import Path

from PIL import Image

SCALE = 3
PAIRS = {"borderless": "plain", "bordered": "gray", "bordered_tinted": "tinted", "bordered_prominent": "filled"}
GLASS = {"glass", "prominent_glass", "clear_glass", "prominent_clear_glass"}


def hexrgba(h):
    h = h.lstrip("#")
    return [int(h[i:i + 2], 16) for i in (0, 2, 4, 6)]


def composite(c, under):
    a = c[3] / 255
    return [c[i] * a + under * (1 - a) for i in range(3)]


def over(top, bottom):
    """top composited over bottom, both straight RGBA bytes: straight RGBA floats."""
    at, ab = top[3] / 255, bottom[3] / 255
    ao = at + ab * (1 - at)
    if ao == 0:
        return [0, 0, 0, 0]
    return [(top[i] * at + bottom[i] * ab * (1 - at)) / ao for i in range(3)] + [ao * 255]


def close(a, b, levels):
    return all(abs(x - y) <= levels for x, y in zip(a, b))


def toml_value(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return repr(round(v, 4)) if isinstance(v, float) else str(v)
    if isinstance(v, dict):
        return "{ " + ", ".join(f"{k} = {toml_value(x)}" for k, x in v.items()) + " }"
    return '"' + str(v).replace("\\", "\\\\").replace('"', '\\"') + '"'


def stable_outside_indicator(folder, x, crops):
    """Whether run1's and run2's crops over black and white agree outside the activity indicator's frame (2 pt
    around it): run2/screen/... is committed beside run1/screen/... for these specimens."""
    ind = [w for w in x["view"] if w["class"] == "UIActivityIndicatorView"]
    if not ind:
        return False
    fx, fy = x["frame_pt"][0], x["frame_pt"][1]
    ix, iy, iw, ih = ind[0]["frame_pt"]
    box = ((fx + ix - 2) * SCALE, (fy + iy - 2) * SCALE, (fx + ix + iw + 2) * SCALE, (fy + iy + ih + 2) * SCALE)
    for crop in crops:
        a_path, b_path = folder / "run1" / crop, folder / "run2" / crop
        with Image.open(a_path) as ia, Image.open(b_path) as ib:
            a, b = ia.convert("RGB"), ib.convert("RGB")
            if a.size != b.size:
                return False
            w, h = a.size
            da, db = a.tobytes(), b.tobytes()
            for yy in range(h):
                for xx in range(w):
                    if box[0] <= xx < box[2] and box[1] <= yy < box[3]:
                        continue
                    i = (yy * w + xx) * 3
                    if da[i:i + 3] != db[i:i + 3]:
                        return False
    return True


def safe(name):
    return name.replace("+", "__")


def main(folder):
    folder = Path(folder)
    root = Path.cwd()
    rel = folder.resolve().relative_to(root.resolve()).as_posix()
    index = tomllib.loads((folder / "run1/index.toml").read_text())
    solved = tomllib.loads((folder / "solved/run1.toml").read_text())
    press = tomllib.loads((folder / "press1/index.toml").read_text())
    focus = tomllib.loads((folder / "focus1/index.toml").read_text())
    differ = set()
    for line in (folder / "differ-levels.txt").read_text().splitlines():
        if line.startswith("./") and ":" in line:
            differ.add(line[2:line.index(":")])
    layers = {(x["appearance"], x["specimen"], x["size"], x["state"]): x for x in solved.get("layer", [])}
    renders = index["render"]
    system = {(x["appearance"], x["specimen"], x["size"], x["state"]): x
              for x in renders if x["backdrop"] == "system" and x["kind"] == "configuration"}
    out = {}

    def bezel_view(x):
        views = x["view"]
        rootv = views[0]
        if rootv["corner_radius_pt"] > 0:
            return rootv
        for v in views[1:]:
            if v["background"] != "absent" and v["background"]["hex"][-2:] != "00":
                return v
        return None

    for key, x in sorted(system.items()):
        app, spec, size, state = key
        style = x["style"]
        c = x["configuration"]
        cell = {}
        reasons = []
        layer = layers.get(key)
        crops = [x["screen"].replace("/system/", f"/{b}/") for b in ("black", "white")]
        if style in GLASS:
            reasons.append("a Liquid Glass style: the solve's single-layer model does not describe it")
        if "activity_indicator" in spec and style not in ("plain", "gray", "tinted", "filled"):
            reasons.append("the indicator's crops of the aliases and glass styles are not compared between the runs")
        elif "activity_indicator" in spec:
            stable = stable_outside_indicator(folder, x, crops)
            cell["stable_outside_indicator"] = stable
            if not stable:
                reasons.append("the two runs' crops differ outside the activity indicator")
        if layer is None:
            reasons.append("no solved layer")
        elif layer["alpha_spread_levels"] > 2:
            reasons.append(f"the solve's alpha spread is {layer['alpha_spread_levels']} levels")
        elif layer["inconsistent"] > 0:
            reasons.append(f"{layer['inconsistent']} pixels are no layer (white below black)")
        for crop in crops:
            if crop in differ and "activity_indicator" not in spec:
                reasons.append(f"{crop} differs between the runs")
        frame = x["frame_pt"]
        frame_px = frame[2] * frame[3] * SCALE * SCALE
        cfg_bg = hexrgba(c["resolved_background_color"]["hex"])
        if reasons:
            cell["fill"] = "unread"
            cell["unread"] = "; ".join(reasons)
        else:
            top = layer["common"][0]
            solved_c = hexrgba(top["rgba"])
            cell["fill_pixels"] = top["pixels"]
            cell["fill_solved"] = top["rgba"]
            cell["fill_configuration"] = c["resolved_background_color"]["hex"]
            if top["pixels"] < 0.5 * frame_px:
                cell["fill"] = "absent"
                cell["fill_source"] = "solve"
            else:
                agree = all(close(composite(cfg_bg, u), composite(solved_c, u), 1) for u in (0, 255))
                cell["fill"] = c["resolved_background_color"]["hex"] if agree else top["rgba"]
                cell["fill_source"] = "configuration" if agree else "solve"
        title = x["title_color"]["hex"]
        cell["label_fill"] = title
        if layer is not None and cell["fill"] != "unread" and "title_none" not in spec:
            base = hexrgba(cell["fill"]) if cell.get("fill", "absent") not in ("absent", "unread") else [0, 0, 0, 0]
            want = over(hexrgba(title), base)
            cell["label_check"] = any(close(hexrgba(q["rgba"]), want, 3) for q in layer["common"])
        cell["font_family"] = x["title_font"]["family"]
        cell["font_size"] = x["title_font"]["size_pt"]
        cell["font_weight"] = x["title_font"]["weight"]
        top_, lead, bottom, trail = c["content_insets_pt"]
        cell["padding_top"], cell["padding_left"], cell["padding_bottom"], cell["padding_right"] = top_, lead, bottom, trail
        v = bezel_view(x)
        src = "this cell"
        if v is None:
            alt = system.get((app, spec, size, "selected"))
            v = bezel_view(alt) if alt else None
            src = "the selected cell"
        if v is not None:
            r = v["corner_radius_pt"]
            if abs(r - min(frame[2], frame[3]) / 2) <= 0.01:
                cell["corner"] = {"full": {"maximum_px": "absent"}}
            else:
                cell["corner"] = r
            cell["corner_curve"] = v["corner_curve"]
            cell["corner_from"] = f"{src}, view {v['path']} ({v['class']})"
        sc = hexrgba(c["resolved_stroke_color"]["hex"])
        if sc[3] == 0 or c["stroke_width_pt"] == 0:
            cell["stroke"] = "absent"
        if style not in GLASS and all(w["shadow_opacity"] == 0 for w in x["view"]):
            cell["shadow"] = "absent"
        cell["opacity"] = x["view"][0]["alpha"] * x["view"][0]["layer_opacity"]
        cell["icon_placement"] = c["image_placement"]
        leaves = [w for w in x["view"] if w["class"] in ("UILabel", "UIImageView", "UIActivityIndicatorView")]
        if leaves and abs(frame[2] - x["intrinsic_size_pt"][0]) <= 0.01 and abs(frame[3] - x["intrinsic_size_pt"][1]) <= 0.01:
            ox = [w["frame_pt"][0] for w in leaves]
            oy = [w["frame_pt"][1] for w in leaves]
            ex = [w["frame_pt"][0] + w["frame_pt"][2] for w in leaves]
            ey = [w["frame_pt"][1] + w["frame_pt"][3] for w in leaves]
            width = max(ex) - min(ox) + lead + trail
            height = max(ey) - min(oy) + top_ + bottom
            cell["content_width"], cell["content_height"] = round(width, 3), round(height, 3)
            if abs(width - frame[2]) <= 0.01:
                cell["min_width"] = "auto"
            if abs(height - frame[3]) <= 0.01:
                cell["min_height"] = "auto"
        cell["icon_text_gap"] = c["image_padding_pt"]
        cell["subtitle_gap"] = c["title_padding_pt"]
        labels = [w for w in x["view"] if w["class"] == "UILabel"]
        images_ = [w for w in x["view"] if w["class"] == "UIImageView"]
        if "image_star" in spec and images_ and labels:
            im, lb = images_[0]["frame_pt"], labels[0]["frame_pt"]
            if c["image_placement"] == "leading":
                cell["icon_text_gap_frames"] = round(lb[0] - (im[0] + im[2]), 3)
        if "title_none" in spec:
            cell["label"] = "absent" if not labels else "drawn"
        indicators = [w for w in images_ if "image_star" not in spec and "activity_indicator" not in spec]
        cell["menu_indicator"] = "drawn" if indicators else "absent"
        if indicators and labels:
            ind, lb = indicators[0]["frame_pt"], labels[0]["frame_pt"]
            cell["menu_indicator_gap"] = round(ind[0] - (lb[0] + lb[2]), 3)
            cell["menu_indicator_margin_right"] = round(frame[2] - trail - (ind[0] + ind[2]), 3)
            cell["menu_indicator_margin_left"] = 0.0
            mid = ind[1] + ind[3] / 2
            cell["menu_indicator_anchor"] = "inline" if abs(mid - frame[3] / 2) <= 1 / SCALE else "unread"
            cell["menu_indicator_size"] = "auto"
        if "image_star" in spec and images_ and cell.get("fill") not in (None, "unread"):
            g = images_[0]
            base = hexrgba(cell["fill"]) if cell["fill"] != "absent" else [0, 0, 0, 0]
            want = over(hexrgba(g["tint"]["hex"]), base)
            if layer is not None and any(close(hexrgba(q["rgba"]), want, 3) for q in layer["common"]) \
                    and g["tint"]["hex"] != cell["fill"]:
                cell["icon_fill"] = g["tint"]["hex"]
        if "subtitle" in spec and len(labels) >= 2:
            sub = labels[1]
            cell["subtitle_fill"] = sub["text_color"]["hex"]
            cell["subtitle_font_family"] = sub["font"]["family"]
            cell["subtitle_font_size"] = sub["font"]["size_pt"]
            cell["subtitle_font_weight"] = sub["font"]["weight"]
            t0 = labels[0]["frame_pt"]
            cell["subtitle_gap_frames"] = round(sub["frame_pt"][1] - (t0[1] + t0[3]), 3)
        out.setdefault(app, {}).setdefault(safe(spec), {}).setdefault(size, {})[state] = cell

    def summary(x):
        y = dict(x)
        for k in ("draw", "screen", "page", "cell_px", "cell_pt", "specimen", "style", "size"):
            y.pop(k, None)
        y["configuration"] = {k: v for k, v in x["configuration"].items() if k != "button_size"}
        return y

    same = {"mini_small": {}, "pairs": {}}
    for (app, spec, size, state), x in system.items():
        if size == "mini" and "+" not in spec:
            y = system.get((app, spec, "small", state))
            la, lb = layers.get((app, spec, "mini", state)), layers.get((app, spec, "small", state))
            ok = y is not None and summary(x) == summary(y) and (la is None) == (lb is None)
            if ok and la is not None:
                ok = la["common"] == lb["common"] and la["opaque"] == lb["opaque"] and la["partial"] == lb["partial"]
            same["mini_small"].setdefault(app, {}).setdefault(spec, {})[state] = ok
        base = spec.split("+")[0]
        if base in PAIRS:
            other = PAIRS[base] + spec[len(base):]
            y = system.get((app, other, size, state))
            la, lb = layers.get((app, spec, size, state)), layers.get((app, other, size, state))
            ok = y is not None and summary({**x, "configuration": {k: v for k, v in x["configuration"].items()}}) == \
                summary({**y, "configuration": {k: v for k, v in y["configuration"].items()}})
            if ok and la is not None and lb is not None:
                ok = la["common"] == lb["common"]
            same["pairs"].setdefault(app, {}).setdefault(safe(spec), {}).setdefault(size, {})[state] = ok

    forms_out = {}
    for x in renders:
        if x["backdrop"] != "system" or x["kind"] == "configuration" or x["state"] == "focused":
            continue
        app, spec, state = x["appearance"], x["specimen"], x["state"]
        cell = {}
        layer = layers.get((app, spec, x["size"], state))
        crops = [x["screen"].replace("/system/", f"/{b}/") for b in ("black", "white")]
        reasons = []
        if layer is None:
            reasons.append("no solved layer")
        else:
            if layer["alpha_spread_levels"] > 2:
                reasons.append(f"the solve's alpha spread is {layer['alpha_spread_levels']} levels")
            if layer["inconsistent"] > 0:
                reasons.append(f"{layer['inconsistent']} pixels are no layer (white below black)")
        for crop in crops:
            if crop in differ:
                reasons.append(f"{crop} differs between the runs")
        frame = x["frame_pt"]
        views = x.get("view", [])
        # the button's own subtree: a bar item's _UIButtonBarButton, the route picker's UIButton, else the root
        sub = views
        for anchor in ("_UIButtonBarButton", "UIButton"):
            roots = [w for w in views if w["class"] == anchor and w["path"] != "0"]
            if roots and x["kind"] in ("bar_button_item", "navigation_bar", "route_picker"):
                rp = roots[0]["path"]
                sub = [w for w in views if w["path"] == rp or w["path"].startswith(rp + ".")]
                break
        masks = [w["path"] for w in sub if w["class"] == "_UIBackButtonMaskView"]
        sub = [w for w in sub if not any(w["path"] == m or w["path"].startswith(m + ".") for m in masks)]
        hosted = x["kind"] in ("bar_button_item", "navigation_bar") and len(sub) == len(views)
        if hosted:
            cell["hosted"] = True
            reasons.append("hosted by SwiftUI: the view tree holds none of the item's parts, and the bar's Liquid Glass "
                           "platter is no single layer")
        if x["kind"] == "paste_control":
            reasons.append("UIPasteControl draws in a remote slot view: the view tree holds none of its parts, and the "
                           "solve finds no single layer (its normal state) or splits the fill over several colours")
        if x["kind"] == "bar_button_item" and state in ("selected", "selected_highlighted"):
            reasons.append("a selected bar item draws its highlight as an image over the bar's material")
        if reasons:
            cell["fill"] = "unread"
            cell["unread"] = "; ".join(reasons)
        else:
            top = layer["common"][0]
            cell["fill_solved"], cell["fill_pixels"] = top["rgba"], top["pixels"]
            cell["fill"] = top["rgba"] if top["pixels"] >= 0.5 * frame[2] * frame[3] * SCALE * SCALE else "absent"
            if cell["fill"] == "absent":
                cell["glyph_solved"] = top["rgba"]
        if x["kind"] in ("bar_button_item", "navigation_bar") and not hosted and not any("selected highlight" in r or "highlight as an image" in r for r in reasons):
            backs = [w for w in sub if w["background"] != "absent" and w["background"]["hex"][-2:] != "00"]
            cell["fill_views"] = "absent" if not backs else backs[0]["background"]["hex"]
            cell["fill"] = cell["fill_views"]
            cell.pop("unread", None)
        labels = [w for w in sub if w["class"] in ("UILabel", "UIButtonLabel") and w.get("text") not in (None, "absent", "Title")
                  and w["frame_pt"][2] > 0]
        if labels:
            cell["label_fill"] = labels[0]["text_color"]["hex"]
            cell["font_family"] = labels[0]["font"]["family"]
            cell["font_size"] = labels[0]["font"]["size_pt"]
            cell["font_weight"] = labels[0]["font"]["weight"]
        glyphs = [w for w in sub if w["class"] == "UIImageView" and w["frame_pt"][2] > 0 and w["tint"]["hex"] != "#ffffffff"]
        if glyphs and not hosted:
            g = glyphs[-1]
            cell["icon_tint"] = g["tint"]["hex"]
            if layer is not None and not reasons:
                base = hexrgba(cell["fill"]) if cell["fill"] not in ("absent",) else [0, 0, 0, 0]
                want = over(hexrgba(g["tint"]["hex"]), base)
                cell["icon_check"] = any(close(hexrgba(q["rgba"]), want, 3) for q in layer["common"])
            if cell.get("fill") == "absent" and "glyph_solved" in cell:
                cell["icon_fill"] = cell["glyph_solved"]
            elif cell.get("icon_check") and cell.get("fill") != cell["icon_tint"]:
                cell["icon_fill"] = cell["icon_tint"]
            elif x["kind"] in ("bar_button_item", "navigation_bar") and cell.get("fill") == "absent":
                cell["icon_fill"] = cell["icon_tint"]
        if not hosted and views:
            cell["stroke"] = "absent" if all(w["border_width_pt"] == 0 for w in sub) else "drawn"
            cell["shadow"] = "absent" if all(w["shadow_opacity"] == 0 for w in sub) else "drawn"
            cell["opacity"] = sub[0]["alpha"] * sub[0]["layer_opacity"]
        if x["kind"] == "button_type" and views:
            leaves = [w for w in views[1:] if w["frame_pt"][2] > 0 and w["frame_pt"][3] > 0 and w["frame_pt"][0] >= 0
                      and w["frame_pt"][1] >= 0 and w["frame_pt"][0] + w["frame_pt"][2] <= frame[2] + 0.01
                      and w["frame_pt"][1] + w["frame_pt"][3] <= frame[3] + 0.01]
            if leaves:
                cell["padding_top"] = round(min(w["frame_pt"][1] for w in leaves), 3)
                cell["padding_left"] = round(min(w["frame_pt"][0] for w in leaves), 3)
                cell["padding_bottom"] = round(frame[3] - max(w["frame_pt"][1] + w["frame_pt"][3] for w in leaves), 3)
                cell["padding_right"] = round(frame[2] - max(w["frame_pt"][0] + w["frame_pt"][2] for w in leaves), 3)
                if abs(frame[2] - x["intrinsic_size_pt"][0]) <= 0.01 and abs(frame[3] - x["intrinsic_size_pt"][1]) <= 0.01:
                    cell["min_width"] = "auto"
                    cell["min_height"] = "auto"
        forms_out.setdefault(app, {}).setdefault(safe(spec), {})[state] = cell

    text_out = {}
    for t in index["text_style"]:
        if t["category"] != "large":
            continue
        f = t["font"]
        text_out.setdefault(t["style"], {})[t["legibility_weight"]] = {
            "line_height": round(f["ascender_pt"] - f["descender_pt"] + f["leading_pt"], 3),
            "weight": f["weight"], "name": f["name"]}

    images = sorted({f"{rel}/run1/index.toml", f"{rel}/solved/run1.toml", f"{rel}/differ-levels.txt",
                     f"{rel}/press-differ-levels.txt", f"{rel}/press1/index.toml", f"{rel}/focus1/index.toml"})
    for (app, spec, size, state), x in system.items():
        if "activity_indicator" in spec and x["style"] in ("plain", "gray", "tinted", "filled"):
            for b in ("black", "white"):
                crop = x["screen"].replace("/system/", f"/{b}/")
                images.extend([f"{rel}/run1/{crop}", f"{rel}/run2/{crop}"])
    pressed = {}
    by_key = {(x["appearance"], x["specimen"], x["size"], x["state"], x["backdrop"]): x for x in renders}
    for p in press["render"]:
        if p["backdrop"] != "system" or p["size"] != "medium" or p["style"] in GLASS or p["kind"] != "configuration":
            continue
        st = {"pressed": "highlighted", "pressed_selected": "selected_highlighted"}[p["state"]]
        r = by_key[(p["appearance"], p["specimen"], "medium", st, "system")]
        a_path, b_path = folder / "press1" / p["screen"], folder / "run1" / r["screen"]
        images.extend([f"{rel}/press1/{p['screen']}", f"{rel}/run1/{r['screen']}"])
        with Image.open(a_path) as ia, Image.open(b_path) as ib:
            a, b = ia.convert("RGB"), ib.convert("RGB")
            pad = 2 * SCALE
            fa = [round(v * SCALE) for v in p["frame_pt"]]
            fb = [round(v * SCALE) for v in r["frame_pt"]]
            ca = a.crop((fa[0] - pad, fa[1] - pad, fa[0] + fa[2] + pad, fa[1] + fa[3] + pad)).tobytes()
            cb = b.crop((fb[0] - pad, fb[1] - pad, fb[0] + fb[2] + pad, fb[1] + fb[3] + pad)).tobytes()
        diffs = [max(abs(ca[i + k] - cb[i + k]) for k in range(3)) for i in range(0, min(len(ca), len(cb)), 3)]
        pressed.setdefault(p["appearance"], {}).setdefault(safe(p["specimen"]), {})[p["state"]] = {
            "max_level_difference": max(diffs) if len(ca) == len(cb) else -1,
            "pixels_over_2_levels": sum(1 for d in diffs if d > 2) if len(ca) == len(cb) else -1,
            "title_color_same": p["title_color"]["hex"] == r["title_color"]["hex"],
            "held": bool(p["highlighted_at_capture"] and p["held_until_screenshot"]),
        }
    fr = [x for x in focus["render"] if "can_become_focused" in x]
    focus_out = {
        "focus_system": focus["screen"]["focus_system"],
        "interface_idiom": focus["screen"]["interface_idiom"],
        "can_become_focused_any": any(x["can_become_focused"] for x in fr),
        "is_focused_any": any(x["is_focused"] for x in fr),
        "phone_focus_system": index["screen"]["focus_system"],
        "phone_interface_idiom": index["screen"]["interface_idiom"],
    }

    lines = [f"# Output of analysis/read_buttons.py (its docstring is the method) over {rel}.",
             "input_image = ["] + [f'    "{n}",' for n in sorted(set(images))] + ["]", ""]

    def emit(prefix, table):
        scalars = {k: v for k, v in table.items() if not isinstance(v, dict) or "full" in v}
        nested = {k: v for k, v in table.items() if isinstance(v, dict) and "full" not in v}
        if scalars:
            lines.append(f"[{prefix}]")
            for k, v in scalars.items():
                lines.append(f"{k} = {toml_value(v)}")
            lines.append("")
        for k, v in nested.items():
            emit(f"{prefix}.{k}", v)

    emit("cells", out)
    emit("same", same)
    emit("press", pressed)
    emit("focus", focus_out)
    emit("forms", forms_out)
    emit("text_styles", text_out)
    (folder / "analysis.toml").write_text("\n".join(lines) + "\n")
    tomllib.loads((folder / "analysis.toml").read_text())
    return 0


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(__doc__)
        sys.exit(2)
    sys.exit(main(sys.argv[1]))
