import sys
import tomllib
from collections import defaultdict


def hexof(color):
    if isinstance(color, dict):
        return color.get("hex", "?")
    return str(color)


rows = defaultdict(list)
for path in sys.argv[1:]:
    with open(path, "rb") as handle:
        data = tomllib.load(handle)
    image = "macos-15" if "macos-15" in path else "macos-26"
    for read in data.get("hover_read", []):
        appearance, specimen, _ = read["page"].split("/")
        backdrop = read["cell"].split("/")[0]
        samples = read.get("sample", [])
        rest = samples[0] if samples and samples[0]["phase"] == "rest" else None
        hovered = [s for s in samples if s["phase"] != "rest"]
        over = [s for s in hovered if s["pointer_over"]]
        def look(s):
            label = s.get("label")
            text = hexof(label.get("text_color")) if isinstance(label, dict) else "-"
            effects = sorted({
                f"{l['delegate']}:{l['compositing_filter']}:bg={hexof(l['background'])}:vbg={hexof(l['view_background'])}:op={l['opacity']}:sh={l['shadow_opacity']}"
                for l in s.get("pointer_layers", [])
                if l["delegate"] not in ("_UIPortalView",) and (l["delegate"] != "absent" or l["compositing_filter"] != "absent")
            })
            return text, s.get("label_ink"), s.get("label_background"), effects
        key = (image, specimen, appearance, backdrop)
        rows[key].append({
            "press": read["press_number"],
            "held": read["held"],
            "left": read.get("pointer_left_after_s"),
            "rest": look(rest) if rest else None,
            "over": [(round(s["t_s"], 3), *look(s)[:3], s.get("label_changed_px"), s.get("label_largest_change_levels")) for s in over],
            "after": sorted({look(s)[1:3] for s in hovered if not s["pointer_over"]}),
            "effects": sorted({e for s in hovered for e in look(s)[3]}),
        })

for key in sorted(rows):
    print("==", " ".join(key))
    for row in rows[key]:
        print(f"  #{row['press']} held={row['held']} left={row['left']}")
        print(f"    rest text/ink/bg: {row['rest'][:3] if row['rest'] else None}")
        print(f"    over: {row['over']}")
        print(f"    after-leave ink/bg: {row['after']}")
        for effect in row["effects"]:
            print(f"    effect {effect}")
