#!/usr/bin/env python3
"""The button pass's hover_moves: per glass button, appearance and way of moving the cursor, the mouse events AppKit
dispatched (before the post too, for warp_wait_post), the move's error and diff_from_normal (differing pixels, largest
byte difference); and the regular hovered state's figures beside them.

Run from the repository root (Python 3.11+, the standard library only):

    python3 <run>/analysis/hover_moves.py <button pass probe.toml>..."""
import sys
import tomllib

def ev(e):
    return f"moved {e.get('mouseMoved')} entered {e.get('mouseEntered')} exited {e.get('mouseExited')}"

for f in sys.argv[1:]:
    sheets = tomllib.load(open(f, "rb"))["macos"]["button_pass"]["sheets"]
    print(f)
    for sheet, s in sheets.items():
        if not isinstance(s, dict):
            continue
        for ap in ("aqua", "darkAqua"):
            a = s.get(ap, {})
            moves = a.get("hover_moves")
            if not isinstance(moves, dict):
                continue
            print(f"  {sheet} {ap}")
            for case, per in moves.items():
                if case == "cursor_parked" or not isinstance(per, dict):
                    continue
                h = a.get("hovered", {}).get(case, {})
                d = h.get("diff_from_normal", {}).get(case, {})
                print(f"    {case} hovered: {ev(h.get('cursor', {}).get('events', {}))}, "
                      f"{d.get('differing_pixels')} px, max {d.get('max_byte_delta')}")
                for how, m in per.items():
                    c = m.get("cursor", {})
                    d = m.get("diff_from_normal", {}).get(case, {})
                    pre = c.get("events_before_post")
                    pre = f" (before the post: {ev(pre)})" if pre else ""
                    err = f" ERROR {c['error']}" if "error" in c else ""
                    print(f"      {how}: {ev(c.get('events', {}))}{pre}, {d.get('differing_pixels')} px, "
                          f"max {d.get('max_byte_delta')}{err}")
