#!/usr/bin/env python3
"""Gate 3: every typed entry has every item of the entry format, in order, and a root part that is a container:
not listed as a may-be-absent part, not marked *(element)*; every part bullet names a PartStyle field.
Every may_be_absent item (typed entries and keyed components) gives each name it lists its own preset list (review 10:
one parenthesis covering several names granted absence where the platform draws the name), states no reason of the
form "the preset states none", and the role-record and token tables grant absence per name with presets or a
criterion. In a typed entry (ADD-A146): no part bullet says another part of the entry belongs to, nests in or is
queried with an element (a part's nesting is its own bullet's parent, *Nesting*); the Variants item is `none` or
sub-bullets alone, none starting `typed`, `keyed` or `On`, and every one starting with `on` has the Entry format's
head, the part it names, if any, an *(element)* part of the entry; every name a may_be_absent section's first
sentence lists, as the generator reads it, is a part (`parts:`), a `part.field` of that part's field list or a member
of it, named by identifiers (`fields:`), or a value of the entry, never a dotted member (`values:`). The entries,
their items and parts are read by spec_catalogue.py.

    python3 scripts/check_spec_entries.py [catalogue.md]
    python3 scripts/check_spec_entries.py --self-test
"""
import re
import sys
from pathlib import Path

from spec_catalogue import (CATALOGUE, ENTRY_ITEMS, IDENT, absence_sections, depth0_find, depth0_tokens, entries,
                            variant_bullets, variants_form)

FIELDS = {"fill", "stroke", "corners", "shape", "shadow", "opacity", "transform", "text", "icon", "glyph", "focus",
          "padding", "margin", "min_width", "min_height", "max_width", "max_height", "cursor", "touch_target", "bindings",
          "transitions", "shown", "interaction"}

PRESET_RX = re.compile(r"^(?:adwaita|kde-breeze|macos-\*|macos-monterey|macos-tahoe|windows-11|ios-\*|ios-13|ios-26|"
                       r"material-3)$")
NAME_RX = re.compile(r"^[a-z_][a-z0-9_]*(?:\.(?:[a-z_][a-z0-9_]*|\{[a-z0-9_, ]+\}))*$")
LIST_HINT = re.compile(r"\(|`(?:adwaita|kde-breeze|macos-[^`]*|windows-11|ios-[^`]*|material-3)`|every preset|"
                       r"\bunder\b|\bby (?:a )?rules?\b|\boutside\b|\bon every\b|\bwhere\b")
NOT_LISTED_BEFORE = re.compile(r"(?:\bunder|\bon|\boutside|\bin|\bby|\bof|\bas|\bthan|\bfor|\bwith|\bat|\bto|"
                               r"\bnot|\bnamed?|\bvariant|\bcontext|\bcond|\bflag|\bstate|\bthe|=)\s*$")
BAD_REASON = re.compile(r"presets? that draws? none|preset states? none|\bstates none\b", re.I)
# a part bullet naming other parts as belonging to, nesting in or queried with an element: nesting is read from each
# part's own bullet only (Entry format, *Nesting*; ADD-A146)
OTHER_PARTS = re.compile(r"`([a-z_][a-z0-9_]*)`((?:,? (?:and )?`[a-z_][a-z0-9_]*`)*)\s+"
                         r"(belongs? to|nests? in|(?:is|are) queried with)\b")


def absence_problems(name, text):
    """`text`: the may_be_absent item's text after its label, joined into one line."""
    out = []
    if BAD_REASON.search(text):
        out.append(f"{name}: may_be_absent gives a preset's statement as the reason ({BAD_REASON.search(text).group(0)!r}); "
                   f"name the platforms that have no such thing")
    for lab, a, b in absence_sections(text):
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


def first_sentence(text):
    end = depth0_find(text, lambda t, i: t[i] == "." and (i + 1 == len(t) or t[i + 1] == " "))
    return text if end < 0 else text[:end]


def absence_name_problems(e, text):
    """A typed entry's may_be_absent names, as the generator reads them (each section's first sentence, its names
    outside parentheses): `parts:` a part, `fields:` a `part.field` of the part's field list (or a member of that
    block, `mark.glyph.fill`, as S6.3's `Glyph.stroke`, each of its names an identifier: the blocks' members are
    no table of this checker's, so which member a block has is not checked), `values:` a value of the entry, never a
    dotted member (a compound is listed once, by its own name; Entry format, ADD-A146)."""
    out = []
    parts = {n: p for p in e.parts for n in p.names}
    values = {v.name for v in e.values} | {re.sub(r"_(?:px|ms|deg)$", "", v.name) for v in e.values}
    for lab, a, b in absence_sections(text):
        for _, _, nm in depth0_tokens(first_sentence(text[a:b].strip(" -")))[0]:
            if lab == "parts":
                bad = None if nm in parts else "is no part of the entry"
            elif lab == "fields":
                pname, _, rest = nm.partition(".")
                fname, dot, member = rest.partition(".")
                if pname not in parts or fname not in parts[pname].fields:
                    bad = "is no `part.field` of the entry"
                elif dot and not all(IDENT.fullmatch(m) for m in member.split(".")):
                    bad = "names a member of the field that is no name"
                else:
                    bad = None
            else:
                bad = "is a dotted member" if "." in nm else None if nm in values else "is no value of the entry"
            if bad:
                out.append(f"{e.name}: may_be_absent {lab} `{nm}` {bad}"
                           + (": list the compound once, its members' presets in its parentheses"
                              if bad == "is a dotted member" else ""))
    return out


def part_nesting_problems(e):
    """A part bullet that names other parts of the entry as belonging to, nesting in or queried with an element."""
    out = []
    names = {n for p in e.parts for n in p.names}
    for part in e.parts:
        flat = " ".join(x.strip() for x in part.lines)
        for m in OTHER_PARTS.finditer(flat):
            subjects = [m.group(1)] + re.findall(r"`([a-z_][a-z0-9_]*)`", m.group(2))
            others = [s for s in subjects if s in names and s not in part.names]
            if others:
                out.append(f"{e.name}: part `{'/'.join(part.names)}` (line {part.line}) says "
                           f"{', '.join(f'`{s}`' for s in others)} {m.group(3)}: a part's nesting is its own "
                           "bullet's parent (Entry format, *Nesting*)")
    return out


def variant_problems(e):
    """The Variants item not in the Entry format's form (neither `none` nor sub-bullets alone, a sub-bullet starting
    `typed`, `keyed` or `On`), its sub-bullets starting with `on` whose head is not the Entry format's, and those on a
    part that is no *(element)* part of the entry."""
    item = bullet_item(e, "Variants")
    if item is None:
        return []
    out = [f"{e.name}: {problem}" for problem in variants_form(item)]
    elements = {n for p in e.parts if p.element for n in p.names}
    for b in variant_bullets(item):
        if not b.form:
            out.append(f"{e.name}: Variants sub-bullet at line {b.line} starts with `on` but its head is not `on the "
                       f"widget, typed`, `` on `<part>`, typed `` or the same with `keyed, `<preset>``: "
                       f"{b.text[:60]!r}")
        elif b.part is not None and b.part not in elements:
            out.append(f"{e.name}: Variants sub-bullet at line {b.line} is on `{b.part}`, no *(element)* part of the "
                       "entry")
    return out


def bullet_item(e, label):
    """The entry's item `label` written as its own `- **label:**` bullet."""
    return next((x for x in e.items if x.bullet and x.label == label), None)


def entry_problems(e):
    """A typed entry: its items in the entry format's order, its parts and root, its may_be_absent item."""
    problems = []
    name = e.name
    got = [x.label for x in e.items if x.bullet and x.label in ENTRY_ITEMS]
    if got != ENTRY_ITEMS:
        missing = [x for x in ENTRY_ITEMS if x not in got]
        order = "" if not missing else f" missing {missing}"
        problems.append(f"{name}: items {got if not missing else ''}{order}".rstrip())
    parts = bullet_item(e, "Parts")
    if parts is None:
        problems.append(f"{name}: no Parts item")
        return problems
    if parts.lines[0].strip():
        problems.append(f"{name}: Parts not in one-sub-bullet-per-part form")
    if not e.parts:
        problems.append(f"{name}: no part")
        return problems
    root_part = e.parts[0]
    if not root_part.names:
        problems.append(f"{name}: the first part bullet does not start with a part name")
        return problems
    root = root_part.names[0]
    if root_part.element:
        problems.append(f"{name}: root `{root}` marked *(element)*")
    if len(root_part.names) > 1:
        problems.append(f"{name}: the root bullet names several parts")
    for part in e.parts:
        if not set(re.findall(r"`([^`]+)`", part.text)) & FIELDS:
            problems.append(f"{name}: part bullet names no field: {part.text.strip()[:80]!r}")
    problems += part_nesting_problems(e)
    problems += variant_problems(e)
    absent = bullet_item(e, "may_be_absent")
    if absent:
        # the parts list: from 'parts:' to 'fields:' / 'values:' / end
        pm = re.search(r"parts:(.*?)(?:\bfields:|\bvalues:|$)", absent.text, re.S)
        if pm and re.search(r"`" + re.escape(root) + r"`", pm.group(1)):
            problems.append(f"{name}: root `{root}` listed as a may-be-absent part")
        problems += absence_problems(name, absent.text)
        problems += absence_name_problems(e, absent.text)
    return problems


def component_problems(e):
    absent = bullet_item(e, "may_be_absent")
    if absent is None:
        return [f"{e.name}: no may_be_absent item"]
    return absence_problems(e.name, absent.text)


def table_problems(t):
    """The role record's and the token table's absence grants: one name per grant, with presets or a criterion."""
    problems = []
    zone = None
    for i, line in enumerate(t.splitlines(), 1):
        if line.startswith("## "):
            zone = line[3:]
        if not zone or not zone.startswith(("Colour roles", "Tokens")) or not line.startswith("| `"):
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) < 3:
            continue
        grant = next((c for c in cells[1:] if re.search(r"may_be_absent|may be absent|^every preset|^each preset", c,
                                                         re.I)), None)
        if grant is None:
            continue
        nms = [x for x in re.findall(r"`([^`]+)`", cells[0])]
        if len(nms) > 1 and not all(re.search(r"`" + re.escape(x) + r"`", grant) for x in nms):
            problems.append(f"table line {i}: one absence grant for {len(nms)} names ({', '.join(nms)})")
        if re.fullmatch(r"(?:each field )?may_be_absent\.?", grant):
            problems.append(f"table line {i}: `{nms[0] if nms else '?'}` may_be_absent with no presets or criterion")
    return problems


def check(t):
    """(keyed components checked, typed entries checked, problems)."""
    es = entries(t)
    typed = [e for e in es if e.kind == "typed"]
    comps = [e for e in es if e.kind == "component"]
    problems = []
    for e in typed:
        problems += entry_problems(e)
    for e in comps:
        problems += component_problems(e)
    problems += table_problems(t)
    return len(comps), len(typed), problems


SELF_TEST_ENTRY = """\
## Fields

### good

- **Kind:** typed widget.
- **Available:** every preset.
- **Parts:**
  - `body` — the root: `fill`, `stroke`.
  - `label` *(element)* — the text: `text`.
- **Widget values:** `gap_px: Value<Px>`.
- **Variants:** none.
- **Sizes and densities:** none.
- **Flags:** none.
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:**
  - parts: `label` (`adwaita`; `macos-*` by a rule).
  - fields: `body.stroke` (`adwaita`), `label.text` (every preset under `flat`).
  - values: `gap_px` (all but `windows-11`).
- **State-model source:** measured.
- **Today's fields:** none.
- **Notes:** none.
"""

SELF_TEST_COMPONENT = """\
## Keyed components

#### component.good (kde)

- **Kind:** keyed component.
- **Parts:**
  - `body` — the root: `fill`.
- **Variants/sizes:** none.
- **may_be_absent:** fields: `body.fill` (`kde-breeze`).
"""

SELF_TEST_TABLES = """\
## Colour roles and text ramp

| `accent` | the accent | every preset states it |
| `link` | a link | may_be_absent: `adwaita` |
| `glow`, `halo` | may_be_absent: `glow` (`adwaita`), `halo` (`macos-*`) | x |
| `shine`, `gleam` | may_be_absent (`adwaita`) | x |
| `dim` | may_be_absent | x |
"""


def self_test() -> int:
    failures = []
    good = SELF_TEST_ENTRY

    def expect(label, text, want):
        got = check(text)[2]
        if got != want:
            failures.append(f"{label}: got {got}, want {want}")

    def edit(text, old, new):
        if old not in text:
            failures.append(f"fixture: {old!r} not found")
        return text.replace(old, new)

    if check(good) != (0, 1, []):
        failures.append(f"good entry: got {check(good)}")
    expect("item missing", edit(good, "- **Available:** every preset.\n", ""),
           ["good: items  missing ['Available']"])
    expect("items out of order", edit(edit(good, "- **Notes:** none.\n", ""), "- **Kind:**",
                                      "- **Notes:** none.\n- **Kind:**"),
           ["good: items ['Notes', 'Kind', 'Available', 'Parts', 'Widget values', 'Variants', 'Sizes and densities', "
            "'Flags', 'Exclusive groups', 'may_be_absent', 'State-model source', \"Today's fields\"]"])
    expect("no Parts item", good[:good.index("- **Parts:**")] + good[good.index("- **Widget values:**"):],
           ["good: items  missing ['Parts']", "good: no Parts item"])
    expect("Parts inline", edit(good, "- **Parts:**\n", "- **Parts:** `body`.\n"),
           ["good: Parts not in one-sub-bullet-per-part form"])
    bullets = "  - `body` — the root: `fill`, `stroke`.\n  - `label` *(element)* — the text: `text`.\n"
    expect("no part", edit(good, bullets, ""), ["good: no part"])
    expect("root without a name", edit(good, "  - `body` — the root:", "  - the root:"),
           ["good: the first part bullet does not start with a part name"])
    expect("root an element", edit(good, "`body` — the root", "`body` *(element)* — the root"),
           ["good: root `body` marked *(element)*"])
    expect("root several parts", edit(good, "`body` — the root", "`body`, `frame` — the root"),
           ["good: the root bullet names several parts"])
    no_text = "good: may_be_absent fields `label.text` is no `part.field` of the entry"
    expect("part without a field", edit(good, "the text: `text`.", "the text."),
           ["good: part bullet names no field: '`label` *(element)* — the text.'", no_text])
    expect("part naming `shown` alone", edit(good, "the text: `text`.", "the text: `shown`."), [no_text])
    expect("part naming `interaction` alone", edit(good, "the text: `text`.", "the text: `interaction`."),
           [no_text])
    expect("root absent", edit(good, "parts: `label` (", "parts: `body` ("),
           ["good: root `body` listed as a may-be-absent part"])
    expect("preset's statement as the reason", edit(good, "(`adwaita`), `label.text`",
                                                    "(the preset states none), `label.text`"),
           ["good: may_be_absent gives a preset's statement as the reason ('preset states none'); "
            "name the platforms that have no such thing"])
    expect("several names, one list", edit(good, "`body.stroke` (`adwaita`)", "`body.{fill,stroke}` (`adwaita`)"),
           ["good: fields `body.{fill,stroke}` covers several names with one preset list",
            "good: may_be_absent fields `body.{fill,stroke}` is no `part.field` of the entry"])
    expect("another part said to belong", edit(good, "the root: `fill`, `stroke`.",
                                               "the root: `fill`, `stroke`. `label` belongs to it."),
           ["good: part `body` (line 8) says `label` belongs to: a part's nesting is its own bullet's parent "
            "(Entry format, *Nesting*)"])
    expect("others said to nest and be queried", edit(good, "the root: `fill`, `stroke`.",
                                                      "the root: `fill`, `stroke`. `label` and `icon` nest in it; "
                                                      "`label` is queried with it."),
           ["good: part `body` (line 8) says `label` nest in: a part's nesting is its own bullet's parent "
            "(Entry format, *Nesting*)",
            "good: part `body` (line 8) says `label` is queried with: a part's nesting is its own bullet's parent "
            "(Entry format, *Nesting*)"])
    expect("its own element named", edit(good, "the text: `text`.",
                                         "the text: `text`. `label` belongs to the widget's element."), [])
    variants = ("- **Variants:**\n  - on the widget, typed (a note): `flat`.\n  - on `label`, keyed, `adwaita`: "
                "`@dim`.\n  - Under `adwaita` a note.\n")
    expect("Variants sub-bullets", edit(good, "- **Variants:** none.\n", variants), [])
    expect("a Variants head of another form", edit(good, "- **Variants:** none.\n",
                                                   variants.replace("on `label`, keyed", "on `label` keyed")),
           ["good: Variants sub-bullet at line 13 starts with `on` but its head is not `on the widget, typed`, "
            "`` on `<part>`, typed `` or the same with `keyed, `<preset>``: 'on `label` keyed, `adwaita`: `@dim`.'"])
    expect("Variants none with a note", edit(good, "- **Variants:** none.\n", "- **Variants:** none (a note).\n"),
           [])
    old_heads = ("- **Variants:**\n  - typed (shared): `flat`.\n  - On the widget, typed: `pill`.\n"
                 "  - keyed, `adwaita`: `@dim`.\n")
    rejects = ("a form the Entry format rejects: a sub-bullet listing variants starts `on the widget` or "
               "`` on `<part>` ``")
    expect("Variants sub-bullets of the old forms", edit(good, "- **Variants:** none.\n", old_heads),
           [f"good: the Variants sub-bullet at line 12 starts with `typed`, {rejects}",
            f"good: the Variants sub-bullet at line 13 starts with `On`, {rejects}",
            f"good: the Variants sub-bullet at line 14 starts with `keyed`, {rejects}"])
    expect("Variants inline", edit(good, "- **Variants:** none.\n", "- **Variants:** typed `flat`; keyed `@dim`.\n"),
           ["good: the Variants item at line 11 is neither `none` nor sub-bullets (Entry format): "
            "'typed `flat`; keyed `@dim`.'"])
    expect("Variants none and sub-bullets", edit(good, "- **Variants:** none.\n", "- **Variants:** none.\n"
                                                 "  - on the widget, typed: `flat`.\n"),
           ["good: the Variants item at line 11 has text after its label and sub-bullets: the item is `none` or "
            "sub-bullets alone (Entry format)"])
    for label, part in [("Variants on the root", "body"), ("Variants on no part", "lable")]:
        expect(label, edit(good, "- **Variants:** none.\n", variants.replace("on `label`", f"on `{part}`")),
               [f"good: Variants sub-bullet at line 13 is on `{part}`, no *(element)* part of the entry"])
    expect("absent member no name", edit(good, "`body.stroke` (`adwaita`)", "`body.stroke.Width` (`adwaita`)"),
           ["good: may_be_absent fields `body.stroke.Width` names a member of the field that is no name"])
    expect("absent member empty", edit(good, "`body.stroke` (`adwaita`)", "`body.stroke.` (`adwaita`)"),
           ["good: may_be_absent fields `body.stroke.` names a member of the field that is no name"])
    expect("absent no part", edit(good, "parts: `label` (", "parts: `lable` ("),
           ["good: may_be_absent parts `lable` is no part of the entry"])
    expect("absent block member", edit(good, "`body.stroke` (`adwaita`)", "`body.stroke.width` (`adwaita`)"), [])
    expect("absent no field", edit(good, "`body.stroke` (`adwaita`)", "`body.text` (`adwaita`)"),
           ["good: may_be_absent fields `body.text` is no `part.field` of the entry"])
    expect("absent dotted member", edit(good, "`gap_px` (all", "`gap_px.top` (all"),
           ["good: may_be_absent values `gap_px.top` is a dotted member: list the compound once, its members' "
            "presets in its parentheses"])
    expect("absent unsuffixed value", edit(good, "`gap_px` (all", "`gap` (all"), [])
    expect("absent no value", edit(good, "`gap_px` (all", "`gaps` (all"),
           ["good: may_be_absent values `gaps` is no value of the entry"])
    expect("shared preset list", edit(good, "`body.stroke` (`adwaita`), `label.text`", "`body.stroke`, `label.text`"),
           ["good: fields `body.stroke` shares `label.text`'s preset list (one list per name)"])
    expect("no presets", edit(good, "`gap_px` (all but `windows-11`)", "`gap_px`"),
           ["good: values `gap_px` names no presets"])
    if check(SELF_TEST_COMPONENT) != (1, 0, []):
        failures.append(f"good component: got {check(SELF_TEST_COMPONENT)}")
    expect("component's may_be_absent not a bullet of its own",
           edit(SELF_TEST_COMPONENT, "none.\n- **may_be_absent:**", "none. **may_be_absent:**"),
           ["component.good: no may_be_absent item"])
    expect("component, no presets", edit(SELF_TEST_COMPONENT, " (`kde-breeze`)", ""),
           ["component.good: fields `body.fill` names no presets"])
    expect("tables", SELF_TEST_TABLES, ["table line 6: one absence grant for 2 names (shine, gleam)",
                                        "table line 7: `dim` may_be_absent with no presets or criterion"])
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def main(argv):
    if argv[1:] == ["--self-test"]:
        return self_test()
    comps, typed, problems = check(Path(argv[1] if len(argv) > 1 else CATALOGUE).read_text())
    print(f"# {comps} keyed components checked")
    print(f"# {typed} typed entries checked")
    for p in problems:
        print(p)
    print(f"# {len(problems)} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
