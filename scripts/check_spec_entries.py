#!/usr/bin/env python3
"""Gate 3: every typed entry has every item of the entry format, in order, and a root part that is a container:
not listed as a may-be-absent part, not marked *(element)*; every part bullet names a PartStyle field.
Every may_be_absent item (typed entries and keyed components) gives each name it lists its own preset list (review 10:
one parenthesis covering several names granted absence where the platform draws the name), states no reason of the
form "the preset states none", and the role-record and token tables grant absence per name with presets or a
criterion.

    python3 scripts/check_spec_entries.py [catalogue.md]
"""
import re
import sys
from pathlib import Path

CAT = Path(sys.argv[1] if len(sys.argv) > 1 else "docs/todo_v0.7.1_model-completeness-spec-widgets.md")
ITEMS = ["Kind", "Available", "Parts", "Widget values", "Variants", "Sizes and densities", "Flags", "Exclusive groups",
         "may_be_absent", "State-model source", "Today's fields", "Notes"]
FIELDS = {"fill", "stroke", "corners", "shape", "shadow", "opacity", "transform", "text", "icon", "glyph", "focus",
          "padding", "margin", "min_width", "min_height", "max_width", "max_height", "cursor", "touch_target", "bindings",
          "transitions"}
TYPED_ZONES = ("Window and chrome", "Overlays", "Buttons and toggles", "Fields", "Indicators", "Containers",
               "Typed widgets the model lacks today (alphabetical)")

PRESET_RX = re.compile(r"^(?:adwaita|kde-breeze|macos-\*|macos-monterey|macos-tahoe|windows-11|ios-\*|ios-13|ios-26|"
                       r"material-3)$")
NAME_RX = re.compile(r"^[a-z_][a-z0-9_]*(?:\.(?:[a-z_][a-z0-9_]*|\{[a-z0-9_, ]+\}))*$")
LIST_HINT = re.compile(r"\(|`(?:adwaita|kde-breeze|macos-[^`]*|windows-11|ios-[^`]*|material-3)`|every preset|"
                       r"\bunder\b|\bby (?:a )?rules?\b|\boutside\b|\bon every\b|\bwhere\b")
NOT_LISTED_BEFORE = re.compile(r"(?:\bunder|\bon|\boutside|\bin|\bby|\bof|\bas|\bthan|\bfor|\bwith|\bat|\bto|"
                               r"\bnot|\bnamed?|\bvariant|\bcontext|\bcond|\bflag|\bstate|\bthe|=)\s*$")
BAD_REASON = re.compile(r"presets? that draws? none|preset states? none|\bstates none\b", re.I)


def depth0_tokens(text):
    """Depth-0 backtick spans (start, end, name) and depth-0 parenthesis groups (start, end)."""
    names, groups = [], []
    depth, i, n = 0, 0, len(text)
    gstart = None
    while i < n:
        c = text[i]
        if c == "`":
            j = text.find("`", i + 1)
            if j < 0:
                break
            if depth == 0:
                names.append((i, j + 1, text[i + 1:j]))
            i = j + 1
            continue
        if c == "(":
            if depth == 0:
                gstart = i
            depth += 1
        elif c == ")" and depth:
            depth -= 1
            if depth == 0:
                groups.append((gstart, i + 1))
        i += 1
    return names, groups


def absence_problems(name, text):
    """`text`: the may_be_absent item's text after its label, joined into one line."""
    out = []
    if BAD_REASON.search(text):
        out.append(f"{name}: may_be_absent gives a preset's statement as the reason ({BAD_REASON.search(text).group(0)!r}); "
                   f"name the platforms that have no such thing")
    # sections by label
    secs = [m for m in re.finditer(r"\b(parts|fields|values):", text)]
    bounds = [(m.end(), secs[k + 1].start() if k + 1 < len(secs) else len(text), m.group(1)) for k, m in enumerate(secs)]
    for a, b, lab in bounds:
        seg = text[a:b]
        names, _ = depth0_tokens(seg)
        listed = []
        for s, e, nm in names:
            if not NAME_RX.match(nm) or PRESET_RX.match(nm):
                continue
            before = seg[:s]
            if NOT_LISTED_BEFORE.search(before):
                continue
            listed.append((s, e, nm))
        for k, (s, e, nm) in enumerate(listed):
            if "{" in nm:
                out.append(f"{name}: {lab} `{nm}` covers several names with one preset list")
            nxt = listed[k + 1][0] if k + 1 < len(listed) else len(seg)
            between = seg[e:nxt]
            if not LIST_HINT.search(between):
                if k + 1 < len(listed):
                    out.append(f"{name}: {lab} `{nm}` shares `{listed[k + 1][2]}`'s preset list (one list per name)")
                else:
                    out.append(f"{name}: {lab} `{nm}` names no presets")
    return out

t = CAT.read_text()
problems = []
entries = 0
zone = None
secs = list(re.finditer(r"^(##|###) (.+)$", t, re.M))
for i, m in enumerate(secs):
    if m.group(1) == "##":
        zone = m.group(2).strip()
        continue
    if zone not in TYPED_ZONES:
        continue
    name = m.group(2).strip()
    end = secs[i + 1].start() if i + 1 < len(secs) else len(t)
    body = t[m.end():end]
    entries += 1
    labels = [re.sub(r":$", "", x).strip() for x in re.findall(r"^- \*\*([^*]+?)\*\*", body, re.M)]
    got = [x for x in labels if x in ITEMS]
    if got != ITEMS:
        missing = [x for x in ITEMS if x not in got]
        order = "" if not missing else f" missing {missing}"
        problems.append(f"{name}: items {got if not missing else ''}{order}".rstrip())
    mp = re.search(r"^- \*\*Parts:\*\*([^\n]*)((?:\n(?!- \*\*|#)[^\n]*)*)", body, re.M)
    if not mp:
        problems.append(f"{name}: no Parts item")
        continue
    if mp.group(1).strip():
        problems.append(f"{name}: Parts not in one-sub-bullet-per-part form")
    bullets = [b for b in re.split(r"\n  - ", mp.group(2)) if b.strip()]
    if not bullets:
        problems.append(f"{name}: no part")
        continue
    root_m = re.match(r"\s*`([a-z_]+)`", bullets[0])
    if not root_m:
        problems.append(f"{name}: the first part bullet does not start with a part name")
        continue
    root = root_m.group(1)
    head = bullets[0].split("—")[0]
    if "*(element" in head:
        problems.append(f"{name}: root `{root}` marked *(element)*")
    if re.match(r"\s*`[a-z_]+`,", bullets[0]):
        problems.append(f"{name}: the root bullet names several parts")
    for b in bullets:
        if not set(re.findall(r"`([^`]+)`", b)) & FIELDS:
            problems.append(f"{name}: part bullet names no field: {b.strip()[:80]!r}")
    ma = re.search(r"^- \*\*may_be_absent[^\n]*((?:\n(?!- \*\*|#)[^\n]*)*)", body, re.M)
    if ma:
        mtext = re.search(r"\*\*may_be_absent[^\n]*", body).group(0) + ma.group(1)
        # the parts list: from 'parts:' to 'fields:' / 'values:' / end
        pm = re.search(r"parts:(.*?)(?:\bfields:|\bvalues:|$)", mtext, re.S)
        if pm and re.search(r"`" + re.escape(root) + r"`", pm.group(1)):
            problems.append(f"{name}: root `{root}` listed as a may-be-absent part")
        problems += absence_problems(name, " ".join(x.strip() for x in mtext.split("\n")).split("**", 2)[-1])
comps = 0
for m in re.finditer(r"^#### component\.([a-z_][a-z0-9_]*)[^\n]*\n(.*?)(?=^#{2,4} |\Z)", t, re.M | re.S):
    comps += 1
    body = m.group(2)
    ma = re.search(r"^- \*\*may_be_absent:?\*\*:?([^\n]*(?:\n(?!- \*\*|#)[^\n]*)*)", body, re.M)
    if not ma:
        problems.append(f"component.{m.group(1)}: no may_be_absent item")
        continue
    problems += absence_problems("component." + m.group(1), " ".join(x.strip() for x in ma.group(1).split("\n")))
# the role record's and the token table's absence grants: one name per grant, with presets or a criterion
lines = t.splitlines()
zone = None
for i, line in enumerate(lines, 1):
    if line.startswith("## "):
        zone = line[3:]
    if not zone or not zone.startswith(("Colour roles", "Tokens")) or not line.startswith("| `"):
        continue
    cells = [c.strip() for c in line.strip().strip("|").split("|")]
    if len(cells) < 3:
        continue
    grant = next((c for c in cells[1:] if re.search(r"may_be_absent|may be absent|^every preset|^each preset", c, re.I)), None)
    if grant is None:
        continue
    nms = [x for x in re.findall(r"`([^`]+)`", cells[0])]
    if len(nms) > 1 and not all(re.search(r"`" + re.escape(x) + r"`", grant) for x in nms):
        problems.append(f"table line {i}: one absence grant for {len(nms)} names ({', '.join(nms)})")
    if re.fullmatch(r"(?:each field )?may_be_absent\.?", grant):
        problems.append(f"table line {i}: `{nms[0] if nms else '?'}` may_be_absent with no presets or criterion")
print(f"# {comps} keyed components checked")
print(f"# {entries} typed entries checked")
for p in problems:
    print(p)
print(f"# {len(problems)} problems")
sys.exit(1 if problems else 0)
