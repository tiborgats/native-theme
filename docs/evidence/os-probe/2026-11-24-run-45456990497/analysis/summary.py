"""Summary of a controls pass's new reads: per appearance normal.backdrop_shows, over_backdrop and its covers per
target, behind_part of the tables, the reveal scrolls' doubleValue, the release posts of each press."""
import sys, tomllib, pathlib
root = pathlib.Path(sys.argv[1])
d = tomllib.load(open(root / "probe.toml", "rb"))
c = d["macos"]["button_pass"]["controls"]
print("accessibility", d["macos"]["environment"]["accessibility"])
for sheet_name, sheet in [("controls", c), ("toolbar_sheet", c["toolbar_sheet"])]:
    print("==", sheet_name)
    for n, t in sheet["targets"].items():
        print(f"  target {n}: over_backdrop={t.get('over_backdrop')} covered_by={t.get('backdrop_covered_by')} error={t.get('error')}")
    for a in ["aqua", "darkAqua"]:
        bs = sheet[a]["normal"].get("backdrop_shows", {})
        print(" ", a, "backdrop_shows:", {k: (v.get("differing_pixels"), v.get("pixels"), "ERR" if "error" in v else "") for k, v in bs.items()})
        for st in ["hovered", "pressed"]:
            for n, s in sheet[a][st].items():
                if not isinstance(s, dict) or n == "cursor_parked": continue
                rv = [s.get(b, {}).get("reveal") for b in ["window", "over_black"]] if "window" in s else [s.get("reveal")]
                for r in rv:
                    if r: print(f"   {a}.{st}.{n} reveal before={r.get('verticalScroller_doubleValue_before')} scrolls={[ (x.get('wheel1'), x.get('verticalScroller_doubleValue_after')) for x in r.get('scrolls', [])]}")
                if st == "pressed":
                    w = s.get("window", {})
                    print(f"   {a}.pressed.{n}: wait_steps={w.get('wait_steps')} up={w.get('mouseUp_posted_to')} up_ours={w.get('mouseUp_window_is_ours')} esc={w.get('escape_posted_to')} err={w.get('error')}")
                kd = s.get("knob_drawn")
                if kd: print(f"   {a}.{st}.{n} knob_drawn={kd}")
for n in ["table_row", "source_list_row"]:
    t = c["targets"][n]
    print("== behind", n)
    for line in t.get("behind_part", []): print("   ", line)
    for line in t.get("enclosing_scroll_view_tree", []): print("   tree", line)
