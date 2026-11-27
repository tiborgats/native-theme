#!/usr/bin/env python3
"""Two runs' motion, events or transition pass compared: every leaf of the table at the dotted path given (for
example macos.button_pass.motion) by its dotted path, the leaves that differ (old -> new) and those only one holds,
leaving out the reads that change from run to run whatever the probe does (times counted from starting screencapture,
pump steps, ticker flips, file sizes, window numbers, dispatched-event counts) and the parking's own reads. A posted
event's log line is compared without its times ("<label>: <outcome>"); an array of timed samples by its length only.

Run from the repository root (Python 3.11+, the standard library only):

    python3 <run>/analysis/compare_timeline.py <old probe.toml> <new probe.toml> <dotted path>"""
import re
import sys
import tomllib

VOLATILE = {
    "wait_steps", "steps", "pump_steps", "events", "window_at_target", "file_bytes", "ticker_flips", "ended_ms",
    "ran_ms", "returned_ms", "planned_ms", "shown_after_ms", "rested_ms", "focus_at_ms", "unfocus_at_ms",
    "pump_lead", "pump_hold", "pump_finish", "longest_ms", "mouseUp_window_under", "mouseUp_window_owner_pid",
    "focus_returned_ms", "unfocus_returned_ms", "recording_ended_ms", "key_wait_steps",
}
PARKING = {
    "target_screen_xy", "target_quartz_xy", "mouseLocation_screen_xy", "window_under_cursor_is_ours",
    "window_at_target_is_ours", "park_point_in_content_xy", "view_at_park_point", "CGWarpMouseCursorPosition",
    "mouseMoved_created", "mouseMoved", "mouseEntered", "mouseExited", "cursorUpdate", "other",
}
TIMED = re.compile(r"^[0-9.]+ ms \(planned [0-9]+ ms\): ")


def leaves(value, path, out):
    if isinstance(value, dict):
        for k, v in value.items():
            leaves(v, f"{path}.{k}" if path else k, out)
    elif isinstance(value, list) and value and all(isinstance(v, dict) for v in value):
        for i, v in enumerate(value):
            leaves(v, f"{path}[{i}]", out)
    elif isinstance(value, list) and value and all(isinstance(v, str) and TIMED.match(v) for v in value):
        out[path] = [TIMED.sub("", v) for v in value]
    elif isinstance(value, list) and path.rsplit(".", 1)[-1] == "samples":
        out[path] = f"{len(value)} samples"
    else:
        out[path] = value
    return out


def at(doc, dotted):
    for key in dotted.split("."):
        doc = doc[key]
    return doc


old = leaves(at(tomllib.load(open(sys.argv[1], "rb")), sys.argv[3]), "", {})
new = leaves(at(tomllib.load(open(sys.argv[2], "rb")), sys.argv[3]), "", {})
skip = VOLATILE | PARKING
kept = lambda p: not any(part.split("[")[0] in skip for part in p.split("."))
differ = [p for p in sorted(set(old) & set(new)) if kept(p) and old[p] != new[p]]
only_old = [p for p in sorted(set(old) - set(new)) if kept(p)]
only_new = [p for p in sorted(set(new) - set(old)) if kept(p)]
for p in differ:
    print(f"{p}: {old[p]!r} -> {new[p]!r}")
for p in only_old:
    print(f"{p}: only in the old run")
for p in only_new:
    print(f"{p}: only in the new run")
print(f"{len(set(old) & set(new))} common leaves, {len(differ)} differ, {len(only_old)} only old, {len(only_new)} only new")
