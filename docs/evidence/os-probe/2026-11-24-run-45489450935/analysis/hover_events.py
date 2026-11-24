#!/usr/bin/env python3
"""The mouse-entered and mouse-exited events AppKit dispatched while the button pass hovered each button: per sheet and
appearance, the hovered cases, the events summed over them (cursor.events of each hovered case) and the cases that got
a mouse-entered, with each glass button's diff_from_normal (differing pixels, largest byte difference).

Run from the repository root (Python 3.11+, the standard library only):

    python3 <run>/analysis/hover_events.py <button pass probe.toml>..."""
import sys
import tomllib

for f in sys.argv[1:]:
    sheets = tomllib.load(open(f, "rb"))["macos"]["button_pass"]["sheets"]
    print(f)
    for sheet, s in sheets.items():
        if not isinstance(s, dict):
            continue
        for ap in ("aqua", "darkAqua"):
            hovered = s.get(ap, {}).get("hovered", {})
            cases = {k: v for k, v in hovered.items() if k != "cursor_parked" and isinstance(v, dict)}
            if not cases:
                continue
            events = [c.get("cursor", {}).get("events", {}) for c in cases.values()]
            entered = [k for k, c in cases.items() if c.get("cursor", {}).get("events", {}).get("mouseEntered", 0)]
            print(f"  {sheet} {ap}: {len(cases)} hovered, mouseEntered {sum(e.get('mouseEntered', 0) for e in events)},"
                  f" mouseExited {sum(e.get('mouseExited', 0) for e in events)}, entered on {entered}")
            for k, c in cases.items():
                if k.endswith("-glass"):
                    d = c.get("diff_from_normal", {}).get(k, {})
                    print(f"    {k}: {d.get('differing_pixels')} pixels differ, max {d.get('max_byte_delta')}")
