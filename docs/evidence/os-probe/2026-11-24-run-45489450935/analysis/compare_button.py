#!/usr/bin/env python3
"""Two button passes' probe.toml compared: every leaf of [macos.button_pass] by its dotted path, the leaves that differ
(old -> new) and those only one holds, leaving out the reads that change from run to run whatever the probe does
(times, pump and wait steps, window numbers, the counts of dispatched mouse events, a capture's file size) and the
records the parking change adds or changes: a move's own reads and the counts of the mouse events AppKit dispatched after it.

Run from the repository root (Python 3.11+, the standard library only):

    python3 <run>/analysis/compare_button.py <old probe.toml> <new probe.toml>"""
import sys
import tomllib

VOLATILE = {
    "wait_steps", "steps", "pump_steps", "events", "mouseUp_window_under", "window_at_target",
    "mouseUp_window_owner_pid", "release_by_watchdog", "file_bytes",
}
PARKING = {
    "target_screen_xy", "target_quartz_xy", "mouseLocation_screen_xy", "window_under_cursor_is_ours",
    "window_at_target_is_ours", "park_point_in_content_xy", "view_at_park_point", "CGWarpMouseCursorPosition",
    "mouseMoved_created", "mouseMoved", "mouseEntered", "mouseExited", "cursorUpdate", "other",
}


def leaves(value, path, out):
    if isinstance(value, dict):
        for k, v in value.items():
            leaves(v, f"{path}.{k}" if path else k, out)
    elif isinstance(value, list) and value and all(isinstance(v, dict) for v in value):
        for i, v in enumerate(value):
            leaves(v, f"{path}[{i}]", out)
    else:
        out[path] = value
    return out


old = leaves(tomllib.load(open(sys.argv[1], "rb"))["macos"]["button_pass"], "", {})
new = leaves(tomllib.load(open(sys.argv[2], "rb"))["macos"]["button_pass"], "", {})
skip = VOLATILE | PARKING
kept = lambda p: p.rsplit(".", 1)[-1].split("[")[0] not in skip
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
