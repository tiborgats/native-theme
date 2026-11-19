#!/usr/bin/env python3
"""Reads the iOS host's behavior pass of run 42275028573 (tools/ios-host, `Measure.swift`'s `behavior` pass) into the
values the facts cite: the text caret, the button reads that are no look, and the navigation item's back-button mode.

Run from the repository root, once per image folder:

    python3 -I docs/evidence/ios-host/2026-11-06-run-42275028573/analysis/behavior_values.py \
        docs/evidence/ios-host/2026-11-06-run-42275028573/ios-host-macos-26

Reads the folder's measure1/index.toml and the caret crops measure1/screen/{light,dark}/caret/default/system/
text_view.png (a UITextView first responder over the system background); writes <folder>/behavior_values.toml. Needs
Pillow. Every value is spelled as a fact's value for its key; durations in seconds (`_s`, the facts convert them).

`[caret]` (the `caret`, `caret_text_field` and `caret_read_only_focused` scenes, light and dark):
- `color`: the hex of the drawn caret view's tint (`text_view_cursor_0_tint`, the `UIStandardTextCursorView`), required
  equal in every scene, appearance and editable view (the text view's and the text field's cursor view);
  `color_drawn`: the commonest colour among the crop's opaque pixels whose channels differ by more than 30 levels (the
  caret is the crop's only chromatic content), in each crop where the caret was captured on (`color_drawn_<appearance>`;
  a crop holding no such pixel caught the blink off and gives no reading).
- the blink, from the cursor view's blink animation (`text_view_cursor_0_animations`, key `_uitcvba`, a
  CAKeyframeAnimation on `opacity` of the cursor view's first sublayer), required identical in every scene, appearance
  and editable view: `blink_on_s` is the animation's duration times the first key time whose value is 0 (the cycle's
  visible part, shown and then fading out, as the GNOME facts state GTK's), `blink_off_s` the duration less that (hidden,
  then fading in, to the cycle's end, where the value is 1 again); `blink_style` "fade" where a key value lies strictly
  between 0 and 1 (the caret passes through partial opacity), else "hard"; `keyframe_values`, `keyframe_times`,
  `calculation_mode`, `duration_s` record the animation.
- `blink_timeout`: "none" where the animation's repeat count is at least 3.0e38 cycles (the float maximum, which no
  session reaches: 1e38 cycles of 1 s) and the text view's 12 s of samples (`text_view_samples`, every 50 ms) show the
  blink sublayer at opacity 0 and at 1 within their last 2 s; else "unread". `samples_span_s` records the span.
- `solid_while_typing`: true where, in the text view's samples, the blink animation's begin time after the sample that
  types a character (`typed = true`, 6 s in) differs from the one before (the blink restarts at the key) and every
  sample from 0.05 s to 0.45 s after the key shows the sublayer at opacity 1 (the cycle's opaque part, which a key
  keeps restarting); else false. `restart_begin_before`, `restart_begin_after` record the begin times.
- `shown_in_read_only`: false where the read-only text view first responder (`caret_read_only_focused`) has no blink
  animation on its cursor view, the cursor view's frame is empty (zero width) and every one of its samples has the
  cursor view hidden; true where a sample shows it visible; else "unread".

`[buttons]` (`[[button_behavior]]`, every configuration style the pass makes and the system type `UIButton(type:
.system)`, the base; `styles` lists them), each value required equal over them:
- `cursor`: "absent" where `pointer_interaction_enabled` is false and `pointer_style_provider` false (the button sets
  no pointer of its own).
- `touch_target` and `hit_extension`: "absent" (`hit_extension` a table of four "absent" sides) where `touch_reach_pt`
  is -1 on every side (`point(inside:with:)` false half a point past each edge: the hit area is the bounds).
- `help_tooltip_on_click`: false where `tool_tip` is "absent" and `tool_tip_interaction` false (no tool tip at all).
`[forms.<style>]` (`close`, `info_light`): the same readings where they hold, and `touch_reach_pt` as read (a whole
number of points: the probe's resolution, which bounds the extension to within a point).

`[navigation]`: `back_display_mode` "default" where `[behavior_defaults]`' `navigation_item_back_button_display_mode`
(a new UINavigationItem's `backButtonDisplayMode`) is 0, `UINavigationItemBackButtonDisplayModeDefault` (UIKit's
UINavigationItem.h: `Default = 0`, `Generic = 1`, `Minimal = 2`); "generic", "minimal" for 1, 2.
"""
import sys
import tomllib
from collections import Counter
from pathlib import Path

from PIL import Image

FLT_MAX_FLOOR = 3.0e38
CROP = "measure1/screen/{a}/caret/default/system/text_view.png"


def toml_value(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, int):
        return str(v)
    if isinstance(v, float):
        return repr(round(v, 6))
    if isinstance(v, str):
        return '"' + v.replace("\\", "\\\\").replace('"', '\\"') + '"'
    if isinstance(v, list):
        return "[" + ", ".join(toml_value(x) for x in v) + "]"
    if isinstance(v, dict):
        return "{ " + ", ".join(f"{k} = {toml_value(x)}" for k, x in v.items()) + " }"
    raise TypeError(type(v))


def one(values, what):
    vals = list(values)
    first = vals[0]
    for v in vals[1:]:
        if v != first:
            sys.exit(f"{what} differs: {first!r} against {v!r}")
    return first


def blink_animation(anims, what):
    a = [x for x in anims if x.get("key") == "_uitcvba"]
    if len(a) != 1:
        sys.exit(f"{what}: {len(a)} blink animations")
    a = a[0]
    return {k: a[k] for k in ("class", "key_path", "duration_s", "repeat_count", "values", "key_times",
                               "calculation_mode", "autoreverses", "speed")}


def main(folder):
    folder = Path(folder)
    root = Path.cwd()
    index_path = folder / "measure1/index.toml"
    index = tomllib.loads(index_path.read_text())
    reads = index["scene_read"]
    inputs = [index_path]
    out = {}

    # --- caret
    caret = {}
    tints, blinks = [], []
    for s in reads:
        for view in ("text_view", "text_field"):
            if s["scene"] in ("caret", "caret_text_field") and f"{view}_cursor_0_class" in s:
                if s[f"{view}_cursor_0_class"] != "UIStandardTextCursorView":
                    sys.exit(f"{s['scene']} {view}: cursor view {s[f'{view}_cursor_0_class']}")
                tints.append(s[f"{view}_cursor_0_tint"]["hex"])
                blinks.append(blink_animation(s[f"{view}_cursor_0_animations"], f"{s['scene']} {view}"))
    caret["color"] = one(tints, "the cursor view's tint")[:7]
    for a in ("light", "dark"):
        p = folder / CROP.format(a=a)
        inputs.append(p)
        with Image.open(p) as im:
            px = Counter(im.convert("RGBA").getdata())
        chroma = [(c, n) for c, n in px.items() if c[3] == 255 and max(c[:3]) - min(c[:3]) > 30]
        if chroma:
            c = max(chroma, key=lambda x: x[1])[0]
            caret[f"color_drawn_{a}"] = "#%02x%02x%02x" % c[:3]
    b = one(blinks, "the blink animation")
    dur, vals, times = b["duration_s"], b["values"], b["key_times"]
    first_zero = next(t for v, t in zip(vals, times) if v == 0)
    caret["blink_on_s"] = dur * first_zero
    caret["blink_off_s"] = dur - dur * first_zero
    caret["blink_style"] = "fade" if any(0 < v < 1 for v in vals) else "hard"
    caret["duration_s"] = dur
    caret["keyframe_values"] = [float(v) for v in vals]
    caret["keyframe_times"] = [float(t) for t in times]
    caret["calculation_mode"] = b["calculation_mode"]
    caret["repeat_count"] = float(b["repeat_count"])

    tv = next(s for s in reads if s["scene"] == "caret" and s["appearance"] == "light")
    samples = [x for x in tv["text_view_samples"] if "views" in x]
    span = samples[-1]["t"] - samples[0]["t"]
    caret["samples_span_s"] = span
    last = [x["views"][0]["sublayers"][0]["opacity"] for x in samples if x["t"] >= samples[-1]["t"] - 2.0]
    blinking_at_end = 0.0 in last and 1.0 in last
    caret["blink_timeout"] = "none" if b["repeat_count"] >= FLT_MAX_FLOOR and blinking_at_end else "unread"

    typed = next(x["t"] for x in tv["text_view_samples"] if x.get("typed"))
    before = [x for x in samples if x["t"] < typed][-1]["views"][0]["sublayers"][0]["animation_begin_times"]
    after_samples = [x for x in samples if typed + 0.05 <= x["t"] <= typed + 0.45]
    after = after_samples[0]["views"][0]["sublayers"][0]["animation_begin_times"]
    solid = all(x["views"][0]["sublayers"][0]["opacity"] == 1.0 for x in after_samples) and len(after_samples) >= 5
    caret["solid_while_typing"] = bool(before != after and solid)
    caret["restart_begin_before"] = [float(x) for x in before]
    caret["restart_begin_after"] = [float(x) for x in after]

    ro = [s for s in reads if s["scene"] == "caret_read_only_focused"]
    shown = []
    for s in ro:
        if not s["read_only_focused_is_first_responder"]:
            sys.exit("the read-only text view is not first responder")
        anim = [x for x in s["read_only_focused_cursor_0_animations"] if x.get("key") == "_uitcvba"]
        empty = s["read_only_focused_cursor_0_frame_pt"][2] == 0
        hidden = all(x["views"][0]["hidden"] is True for x in s["read_only_focused_samples"] if "views" in x)
        visible = any(x["views"][0]["hidden"] is False for x in s["read_only_focused_samples"] if "views" in x)
        shown.append(False if (not anim and empty and hidden) else (True if visible else "unread"))
    caret["shown_in_read_only"] = one(shown, "the read-only caret")
    out["caret"] = caret

    # --- buttons
    def readings(bb):
        r = {}
        if bb["pointer_interaction_enabled"] is False and bb["pointer_style_provider"] is False:
            r["cursor"] = "absent"
        reach = bb["touch_reach_pt"]
        if all(reach[s] == -1 for s in ("top", "left", "bottom", "right")):
            r["touch_target"] = "absent"
            r["hit_extension"] = {f"{s}_px": "absent" for s in ("top", "right", "bottom", "left")}
        if bb["tool_tip"] == "absent" and bb["tool_tip_interaction"] is False:
            r["help_tooltip_on_click"] = False
        return r

    bbs = index["button_behavior"]
    base = [bb for bb in bbs if bb["style"] not in ("close", "info_light")]
    rs = [readings(bb) for bb in base]
    buttons = {"styles": [bb["style"] for bb in base]}
    for k in ("cursor", "touch_target", "hit_extension", "help_tooltip_on_click"):
        buttons[k] = one([r.get(k, "unread") for r in rs], f"buttons {k}")
    out["buttons"] = buttons
    for bb in bbs:
        if bb["style"] in ("close", "info_light"):
            r = readings(bb)
            r["touch_reach_pt"] = {s: bb["touch_reach_pt"][s] for s in ("top", "right", "bottom", "left")}
            r["pointer_interaction_enabled"] = bb["pointer_interaction_enabled"]
            out[f"forms.{bb['style']}"] = r

    # --- navigation
    mode = index["behavior_defaults"]["navigation_item_back_button_display_mode"]
    out["navigation"] = {"back_display_mode": {0: "default", 1: "generic", 2: "minimal"}[mode],
                         "back_display_mode_read": mode}

    lines = [f"# Output of analysis/behavior_values.py (its docstring is the method) over {folder.as_posix()}.",
             "input_image = ["]
    lines += [f'    "{p.as_posix()}",' for p in inputs]
    lines.append("]")
    for table, vals in out.items():
        lines.append("")
        lines.append(f"[{table}]")
        for k, v in vals.items():
            lines.append(f"{k} = {toml_value(v)}")
    (folder / "behavior_values.toml").write_text("\n".join(lines) + "\n")


if __name__ == "__main__":
    main(sys.argv[1])
