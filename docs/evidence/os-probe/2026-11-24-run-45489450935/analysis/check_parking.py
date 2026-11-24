#!/usr/bin/env python3
"""Every cursor move and park one probe.toml records, summed up: per pass table under [macos.button_pass], the tables
holding park_point_in_content_xy (a park) or window_at_target_is_ours (a move), how many were made on the probe's
window (window_at_target_is_ours and window_under_cursor_is_ours true, no error), the park points in the content view
and the views hit there, the motion pass's park_quartz_xy, and every error of a move or park in full.

Run from the repository root (Python 3.11+, the standard library only):

    python3 <run>/analysis/check_parking.py <probe.toml>"""
import sys
import tomllib


def walk(value, path, out):
    if isinstance(value, dict):
        if "park_point_in_content_xy" in value or "window_at_target_is_ours" in value:
            out.append((path, value))
        for k, v in value.items():
            walk(v, f"{path}.{k}" if path else k, out)
    return out


doc = tomllib.load(open(sys.argv[1], "rb"))["macos"]["button_pass"]
moves = walk(doc, "", [])
ok = [p for p, t in moves if t.get("window_at_target_is_ours") is True
      and t.get("window_under_cursor_is_ours") is True and "error" not in t]
parks = sorted({(tuple(t["park_point_in_content_xy"]), tuple(t.get("view_at_park_point", [])))
                for _, t in moves if "park_point_in_content_xy" in t})
quartz = []
def find_quartz(value, path):
    if isinstance(value, dict):
        for k, v in value.items():
            if k == "park_quartz_xy":
                quartz.append((path, v))
            find_quartz(v, f"{path}.{k}" if path else k)
find_quartz(doc, "")
print(f"{len(moves)} moves and parks, {len(ok)} on the probe's window without an error")
for point, views in parks:
    print(f"park point {list(point)} in the content view, view hit {list(views)}")
for path, v in quartz:
    print(f"{path}.park_quartz_xy = {v}")
for path, t in moves:
    if "error" in t:
        print(f"{path}.error = {t['error']}")
