"""Per iOS version, part, appearance and backdrop: the hover reads' count, the
latest read with the pointer still over the target, the label's ink and the
label box's commonest colour at rest and in the reads with the pointer over
it, and the pointer-effect views found in the window.

    python3 hover_table.py ios-host-macos-*/hover*/index.toml
"""

import sys
import tomllib
from collections import defaultdict

table = defaultdict(lambda: {"reads": 0, "over": 0.0, "ink_rest": set(), "ink_over": set(),
                             "bg_rest": set(), "bg_over": set(), "effects": set()})
for path in sys.argv[1:]:
    version = "18.6" if "macos-15" in path else "26.5"
    with open(path, "rb") as handle:
        data = tomllib.load(handle)
    for read in data.get("hover_read", []):
        appearance, part, _ = read["page"].split("/")
        backdrop = read["cell"].split("/")[0]
        row = table[(version, part, appearance, backdrop)]
        row["reads"] += 1
        for sample in read["sample"]:
            if sample["phase"] == "rest":
                row["ink_rest"].add(sample.get("label_ink"))
                row["bg_rest"].add(sample.get("label_background"))
                continue
            if sample["pointer_over"]:
                row["over"] = max(row["over"], sample["t_s"])
                row["ink_over"].add(sample.get("label_ink"))
                row["bg_over"].add(sample.get("label_background"))
            row["effects"] |= {layer["view"] for layer in sample["pointer_layers"]
                               if layer["layer"] == "0" and "Portal" not in layer["view"]}

for key in sorted(table):
    row = table[key]
    print(*key, f"reads={row['reads']}", f"over_until_s={row['over']:.3f}",
          "ink rest", sorted(row["ink_rest"]), "over", sorted(row["ink_over"]),
          "background rest", sorted(row["bg_rest"]), "over", sorted(row["bg_over"]),
          "effects", sorted(row["effects"]))
