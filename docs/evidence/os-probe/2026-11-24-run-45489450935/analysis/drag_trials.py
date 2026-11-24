#!/usr/bin/env python3
"""The events pass's drag-start trials, one line each: per trial (distance_after_hold: 800 ms held, then the distance;
hold_before_distance: the hold, then 12 pt), the drag pasteboard's change count (1: a drag session started), the
source's selection after, the pump steps and whether every CGEvent was created, with the press point and the image.

Run from the repository root (Python 3.11+, the standard library only):

    python3 <run>/analysis/drag_trials.py <events pass probe.toml>..."""
import sys
import tomllib

for f in sys.argv[1:]:
    d = tomllib.load(open(f, "rb"))
    ds = d["macos"]["button_pass"]["events"]["drag_start"]
    print(f"{f}: image {d['probe'].get('runner_image_version')}, press {ds.get('press_quartz_xy')}")
    for group in ("distance_after_hold", "hold_before_distance"):
        for k, v in ds.get(group, {}).items():
            print(f"  {group} {k}: drag_pasteboard_changes {v.get('drag_pasteboard_changes')},"
                  f" selectedRange_after {v.get('selectedRange_after')}, pump_steps {v.get('pump_steps')},"
                  f" events_created {v.get('events_created')}{', error ' + v['error'] if 'error' in v else ''}")
