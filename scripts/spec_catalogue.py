#!/usr/bin/env python3
"""Read the v0.7.1 widget catalogue (docs/todo_v0.7.1_model-completeness-spec-widgets.md) into its entries.

The catalogue's typed widgets are the `### <name>` sections under the H2 sections TYPED_SECTIONS names (and any H2
starting "Typed widgets"); its keyed components are the `#### component.<key> (<platform>, …)` sections under
"## Keyed components". Every other section is prose or a table and holds no entry. An entry runs from its heading to
the next `##`, `###` or `####` heading. A `###` heading in a typed section that is no identifier is still an entry,
named as written, so a checker sees it (catalogue_index leaves it out: no path can name it).

An entry's items are its bold labels, `- **Parts:** …`, written one per bullet in a typed entry and several to a line
in many components (`- **Variants/sizes:** none. **Flags:** …`, a continuation line `  **Exclusive groups:** …`). An
item starts at a line that ITEM matches (up to two spaces, an optional `- `, then `**`), holds each bold label on
that line with the text up to the next label, and the last label on the line also takes every following line up to
the next such line or heading. Lines before an entry's first item (its *From group …* note) belong to no item.

The **Parts** item lists one part per sub-bullet (`  - `), its continuation lines indented four spaces:

    - `body` — the root, …: `fill`, `stroke`, …           a part, its description, then its fields
    - `segment` *(element)* — one segment …: `fill`, …     an *(element)* part
    - `title`, in `header` — …: `text`, …                   a part nested in another (its parent)
    - `prefix`, `suffix` — …: `text`, …                     two parts sharing one field list
    - `year` *(element: Material's year cells, …)*: `fill`   no description: the fields follow the head's colon

A part's head is its text before its first ` — ` outside parentheses and backticks, or, without one, before its first
`:` there; its names are the head's backticked names outside parentheses, but the one after `in ` (the parent). Its
fields, the catalogue's normative field list (Entry format), are each list `a` (note), `b` per state (note) and `c`
that starts right after the ` — ` (an entry with no description) or after a `:` outside parentheses, in the bullet's
first sentence (up to a `.` outside parentheses followed by a space): "`layer` — the root: …; it holds `body`, …:
`bindings`" reads `bindings`; a spinner body's two lists, its own and "; the container of …: `fill`, …", are both
read. A list written inside parentheses is not read (menu's `group` has none other). The first part is the root. A
line of the item indented less than four spaces that is not a bullet (a sentence after the list) belongs to no part.

The **Widget values** item declares each value as a backticked `name: Type`, or a bare `name` in a list that ends in a
typed one (`rows`, `columns: Value<u32>`: both `Value<u32>`), or a bare `name` with a note (`focus_visible_when`
(S2.11's …), whose type the entry format gives). A declaration is such a list opening a clause; parse_values states
the rule. Variants, sizes and flags are prose (typed and keyed names per preset, with notes): their items' text and
backticked names are exposed as they stand, not interpreted.

    python3 scripts/spec_catalogue.py [catalogue.md]   # one line per entry: line, kind, name, parts, values
    python3 scripts/spec_catalogue.py --self-test

API (run from the repository root, or pass a path):

    entries(text) -> list[Entry]          every entry in document order
    read(path=CATALOGUE) -> list[Entry]   entries() of a file
    catalogue_index(text) -> dict         per entry name: kind, line and the backticked names of its parts, widget
                                          values, variants and flags items (check_spec_names.py's vocabulary)
    names_in(text) -> set[str]            the identifiers inside text's backtick spans (`@k` keeps its `@`)
    depth0_tokens(text)                   the backtick spans and parenthesis groups outside parentheses
    absence_sections(text)                a may_be_absent item's `parts:`, `fields:`, `values:` sections

    Entry: name ("button", "component.led"), kind ("typed" | "component"), line (the heading's), section (its H2),
           platform (a component's first heading word in parentheses, else None), heading, body_lines
           [(line, text)], items [Item], parts [Part], values [Value]; body (text), item(label) -> Item | None
    Item:  label (the bold text without its trailing colon), line, lines (the text after the label, then each
           continuation line as written), bullet (the label opens a `- **` line); text (lines stripped and joined
           by spaces), names() (names_in(text)), line_at(offset in text)
    Part:  names, parent (None or a part name), element (marked *(element…)*), note (the *(…)* text or None),
           fields, line, lines (as written, the first without its `  - `); text (lines joined by newlines)
    Value: name (as written, unit suffix included: `icon_text_gap_px`), type (its own or its list's, None where
           neither states one), line
"""
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

CATALOGUE = Path("docs/todo_v0.7.1_model-completeness-spec-widgets.md")
TYPED_SECTIONS = ("Window and chrome", "Overlays", "Buttons and toggles", "Fields", "Indicators", "Containers")
ENTRY_ITEMS = ["Kind", "Available", "Parts", "Widget values", "Variants", "Sizes and densities", "Flags",
               "Exclusive groups", "may_be_absent", "State-model source", "Today's fields", "Notes"]

IDENT = re.compile(r"[a-z_][a-z0-9_]*")
ITEM = re.compile(r"^\s{0,2}(?:- )?\*\*([^*]+?)\*\*")
TYPED_HEADING = re.compile(r"^### (.+?)\s*$")
COMPONENT_HEADING = re.compile(r"^#### component\.([a-z_][a-z0-9_]*)(.*)$")

# check_spec_names.py's vocabulary: which items' names count as a widget's parts, values, variants and flags
LABELS = {
    "parts": re.compile(r"part", re.I),
    "values": re.compile(r"value", re.I),
    "variants": re.compile(r"variant", re.I),
    "flags": re.compile(r"flag", re.I),
}


def is_typed_section(title):
    return title is not None and (title in TYPED_SECTIONS or title.startswith("Typed widgets"))


def is_component_section(title):
    return title is not None and title.startswith("Keyed components")


def names_in(text: str) -> set[str]:
    out = set()
    for span in re.findall(r"`([^`]+)`", text):
        for tok in re.findall(r"@?[a-z_][a-z0-9_]*", span):
            out.add(tok)
    return out


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


def depth0_find(text, pred, start=0):
    """The first index from `start` outside parentheses and backtick spans where pred(text, i) holds, else -1."""
    depth, i, n = 0, start, len(text)
    while i < n:
        c = text[i]
        if c == "`":
            j = text.find("`", i + 1)
            if j < 0:
                return -1
            i = j + 1
            continue
        if c == "(":
            depth += 1
        elif c == ")" and depth:
            depth -= 1
        elif depth == 0 and pred(text, i):
            return i
        i += 1
    return -1


def absence_sections(text):
    """A may_be_absent item's sections: (label, start, end) for each `parts:`, `fields:`, `values:` in text, its
    range the text after the label up to the next one."""
    secs = list(re.finditer(r"\b(parts|fields|values):", text))
    return [(m.group(1), m.end(), secs[k + 1].start() if k + 1 < len(secs) else len(text)) for k, m in enumerate(secs)]


@dataclass
class Item:
    label: str
    line: int
    lines: list[str]
    bullet: bool

    @property
    def text(self) -> str:
        return " ".join(x.strip() for x in self.lines)

    def names(self) -> set[str]:
        return names_in(self.text)

    def line_at(self, offset: int) -> int:
        """The source line holding `offset` of `text`."""
        pos = 0
        for k, x in enumerate(self.lines):
            pos += len(x.strip()) + 1
            if offset < pos:
                return self.line + k
        return self.line + len(self.lines) - 1


@dataclass
class Part:
    names: list[str]
    parent: str | None
    element: bool
    note: str | None
    fields: list[str]
    line: int
    lines: list[str]

    @property
    def text(self) -> str:
        return "\n".join(self.lines)


@dataclass
class Value:
    name: str
    type: str | None
    line: int


@dataclass
class Entry:
    name: str
    kind: str
    line: int
    section: str
    platform: str | None
    heading: str
    body_lines: list[tuple[int, str]] = field(default_factory=list)
    items: list[Item] = field(default_factory=list)
    parts: list[Part] = field(default_factory=list)
    values: list[Value] = field(default_factory=list)

    @property
    def body(self) -> str:
        return "\n".join(x for _, x in self.body_lines)

    def item(self, label: str) -> Item | None:
        return next((x for x in self.items if x.label == label), None)


def parse_items(body_lines):
    items, cur = [], None
    for n, line in body_lines:
        if ITEM.match(line):
            # a line may hold several items ("**Variants:** none. **Flags:** `@x`"): each segment to its label
            segs = re.split(r"\*\*([^*]+?)\*\*", line)
            pairs = list(zip(segs[1::2], segs[2::2]))
            for k, (lab, txt) in enumerate(pairs):
                label = re.sub(r":$", "", lab).strip()
                if not lab.endswith(":") and txt.startswith(":"):
                    txt = txt[1:]
                items.append(Item(label, n, [txt], k == 0 and line.startswith("- **")))
            cur = items[-1]
            continue
        if cur is not None:
            cur.lines.append(line)
    return items


def span_list(text, i):
    """The backtick spans (offset, content) of the list `a` (note), `b` and `c` starting at text[i] (spaces
    skipped); a span may carry a qualifier before its note (`text` per state (…))."""
    out = []
    n = len(text)
    while True:
        while i < n and text[i] == " ":
            i += 1
        if i >= n or text[i] != "`":
            return out
        j = text.find("`", i + 1)
        if j < 0:
            return out
        out.append((i, text[i + 1:j]))
        i = j + 1
        while i < n and text[i] == " ":
            i += 1
        if text.startswith("and ", i):
            i += 4
            continue
        q = re.match(r"[a-z]+(?: [a-z]+){0,2} (?=\()", text[i:])
        if q:
            i += q.end()
        if i < n and text[i] == "(":
            depth = 0
            while i < n:
                depth += text[i] == "("
                depth -= text[i] == ")"
                i += 1
                if depth == 0:
                    break
        while i < n and text[i] == " ":
            i += 1
        if text.startswith(",", i):
            i += 1
        elif text.startswith("and ", i):
            i += 4
        else:
            return out


def field_list(text, i):
    """The part-field names of the list starting at text[i]."""
    return [s for _, s in span_list(text, i) if IDENT.fullmatch(s)]


def parse_part(line, lines):
    flat = " ".join(x.strip() for x in lines)
    dash = depth0_find(flat, lambda t, i: t.startswith(" — ", i))
    colon = depth0_find(flat, lambda t, i: t[i] == ":")
    if dash >= 0:
        head = flat[:dash]
    else:
        head = flat[:colon] if colon >= 0 else flat
    # a field list follows the dash (no description) or a colon outside parentheses, in the bullet's first sentence;
    # it may state several (spinner's body: its own fields, then "; the container of Material's …: `fill`, …")
    end = depth0_find(flat, lambda t, k: t[k] == "." and (k + 1 == len(t) or t[k + 1] == " "))
    end = len(flat) if end < 0 else end
    fields, i = [], (dash + 3 if dash >= 0 else colon + 1 if colon >= 0 else -1)
    while 0 <= i < end:
        fields += [f for f in field_list(flat, i) if f not in fields]
        i = depth0_find(flat, lambda t, k: t[k] == ":", i)
        i = i + 1 if i >= 0 else i
    names, parent = [], None
    if flat.startswith("`"):
        for s, e, nm in depth0_tokens(head)[0]:
            if not IDENT.fullmatch(nm):
                continue
            if re.search(r"\bin\s*$", head[:s]):
                parent = parent or nm
            else:
                names.append(nm)
    mnote = re.search(r"\*\((.*?)\)\*", head)
    note = mnote.group(1) if mnote else None
    return Part(names, parent, bool(note is not None and note.startswith("element")), note, fields, line, lines)


def parse_parts(item):
    bullets, cur = [], None
    for k, raw in enumerate(item.lines[1:], 1):
        if raw.startswith("  - "):
            cur = (item.line + k, [raw[4:]])
            bullets.append(cur)
        elif cur is not None and raw.startswith("    "):
            cur[1].append(raw)
        else:
            cur = None
    return [parse_part(n, lines) for n, lines in bullets]


def parse_values(item):
    """A declaration is a list of spans `name: Type` or `name` opening a clause: the item's start, a sub-bullet, or
    the text after a `;` or `. `; inside a parenthetical note (sheet's `top_padding_px` note declares the formula's
    four parameters) only a list that states a type. A name without a type takes the next typed one's in its list
    (`rows`, `columns: Value<u32>`). A list followed straight by another span (`adwaita` `restore`, a preset and its
    keys) declares nothing, nor does a list without a type followed by prose (`row_order` orders …), and a name the
    item declares again is listed once."""
    text = item.text
    starts = [(0, 0)]
    k = depth = 0
    while k < len(text):
        if text[k] == "`":
            j = text.find("`", k + 1)
            k = len(text) if j < 0 else j + 1
            continue
        if text[k] == "(":
            depth += 1
        elif text[k] == ")" and depth:
            depth -= 1
        elif text[k] == ";" or text.startswith(". ", k):
            starts.append((k + 1, depth))
        elif text.startswith(" - `", k):
            starts.append((k + 3, depth))
        k += 1
    out, seen = [], set()
    for s, depth in starts:
        decl = []
        for off, span in span_list(text, s):
            m = re.fullmatch(r"([a-z_][a-z0-9_]*)(?:\s*:\s*(.+))?", span.strip())
            if not m:
                break
            decl.append((off, m.group(1), m.group(2)))
        if depth and not any(own for _, _, own in decl):
            continue
        if decl:
            after = text[text.find("`", decl[-1][0] + 1) + 1:].lstrip(" ")
            if after.startswith("`"):
                continue
            if not any(own for _, _, own in decl) and after and after[0] not in "(;.,":
                continue
        typ = None
        for off, name, own in reversed(decl):
            typ = own or typ
            if off not in seen:
                seen.add(off)
                out.append((off, Value(name, own or typ, item.line_at(off))))
    out.sort(key=lambda x: x[0])
    typed = {v.name for _, v in out if v.type}
    values, names = [], set()
    for _, v in out:
        if v.name in names or (v.type is None and v.name in typed):
            continue
        names.add(v.name)
        values.append(v)
    return values


def entries(text: str) -> list[Entry]:
    out = []
    section = None
    cur = None
    for n, line in enumerate(text.splitlines(), 1):
        if line.startswith(("## ", "### ", "#### ")):
            cur = None
            if line.startswith("## "):
                section = line[3:].strip()
                continue
            m = TYPED_HEADING.match(line)
            if m and is_typed_section(section):
                cur = Entry(m.group(1), "typed", n, section, None, line[4:].strip())
                out.append(cur)
                continue
            m = COMPONENT_HEADING.match(line)
            if m and is_component_section(section):
                mp = re.match(r"\s*\(([^,)]+)", m.group(2))
                cur = Entry("component." + m.group(1), "component", n, section,
                            mp.group(1).strip() if mp else None, line[5:].strip())
                out.append(cur)
            continue
        if cur is not None:
            cur.body_lines.append((n, line))
    for e in out:
        e.items = parse_items(e.body_lines)
        parts = e.item("Parts")
        e.parts = parse_parts(parts) if parts else []
        values = e.item("Widget values")
        e.values = parse_values(values) if values else []
    return out


def read(path=CATALOGUE) -> list[Entry]:
    return entries(Path(path).read_text(encoding="utf-8"))


def catalogue_index(text: str) -> dict:
    widgets: dict[str, dict] = {}
    for e in entries(text):
        if e.kind == "typed" and not IDENT.fullmatch(e.name):
            continue
        w = widgets.setdefault(e.name, {"kind": e.kind, "line": e.line, "parts": set(), "values": set(),
                                        "variants": set(), "flags": set()})
        for it in e.items:
            if "may_be_absent" in it.label or "Today" in it.label or "State-model" in it.label:
                continue
            keys = [k for k, rx in LABELS.items() if rx.search(it.label)]
            if keys:
                names = it.names()
                for k in keys:
                    w[k] |= names
    return widgets


# ---------------------------------------------------------------------------------------------- self-test

SELF_TEST_CATALOGUE = """\
# Catalogue

## Entry format

### not_an_entry

- **Parts:**
  - `ghost` — never read: `fill`.

## Buttons and toggles

### button

*From group 2's catalogue.*

- **Kind:** typed widget: every platform.
- **Parts:**
  - `body` — the root, the bezel (a note: `not_a_field`): `fill`, `stroke` (its width, `x.y`),
    `corners`. `label` and `icon` belong to it.
  - `label` *(element; states hover)* — text: `text` per state (w), `opacity` and `focus`; box: `padding`. No: `margin`.
  - `icon`, in `label` — the image, it holds `x`: `icon`, `opacity`.
  - `prefix`, `suffix` — the affixes: `text`.
  - `year` *(element: the year cells, Q1.2)*: `fill`, `stroke`.
  A sentence after the list naming `margin`.
- **Widget values:** `icon_text_gap_px: Value<Px>` (Q1; `= token.x`); `rows`, `columns: Value<u32>`; `adwaita` `x`;
  `kind: Value<Key>` (a key; `inner_a`, `inner_b: Value<Ms>`; `false` if none), `Kind { A, B }`. `spare` counts.
  `rows` (again). `focus_visible_when` (S2.11).
- **Variants:** typed `flat`, `pill`; keyed `adwaita` `@osd`.
- **Flags:** shared `empty`; keyed `@busy`; state flags used: hover.
- **may_be_absent:** parts: `icon` (`adwaita`).
- **Today's fields:** `font` → `label.text.font`.

### Malformed Heading

- **Parts:**
  - `ghost` — indexed by no name: `fill`.

## Keyed components

### KDE

#### component.led (kde)

- **Kind:** keyed component.
- **Parts:**
  - `body` — the root: `fill`.
- **Widget values:** none.
- **Variants/sizes:** none. **Flags:** keyed `@on`; state flags used: disabled.
  **Exclusive groups:** none.
- **may_be_absent**: none.

#### component.bar (windows, Windows Community Toolkit)

- **Parts:**
  - `strip` — the root: `fill`.

## Colour roles and text ramp

#### component.not_here (kde)
"""


def self_test() -> int:
    failures = []

    def expect(label, got, want):
        if got != want:
            failures.append(f"{label}: got {got!r}, want {want!r}")

    es = entries(SELF_TEST_CATALOGUE)
    expect("entries", [(e.name, e.kind, e.line, e.section, e.platform) for e in es], [
        ("button", "typed", 12, "Buttons and toggles", None),
        ("Malformed Heading", "typed", 33, "Buttons and toggles", None),
        ("component.led", "component", 42, "Keyed components", "kde"),
        ("component.bar", "component", 52, "Keyed components", "windows"),
    ])
    b, _, led, bar = es
    expect("button items", [(i.label, i.line, i.bullet) for i in b.items], [
        ("Kind", 16, True), ("Parts", 17, True), ("Widget values", 25, True), ("Variants", 28, True),
        ("Flags", 29, True), ("may_be_absent", 30, True), ("Today's fields", 31, True),
    ])
    expect("button items in format order", [i.label for i in b.items if i.label in ENTRY_ITEMS],
           ["Kind", "Parts", "Widget values", "Variants", "Flags", "may_be_absent", "Today's fields"])
    expect("item text", b.item("Variants").text, "typed `flat`, `pill`; keyed `adwaita` `@osd`.")
    expect("item names", b.item("Flags").names(), {"empty", "@busy"})
    expect("item missing", b.item("Notes"), None)
    expect("led items", [(i.label, i.line, i.bullet) for i in led.items], [
        ("Kind", 44, True), ("Parts", 45, True), ("Widget values", 47, True), ("Variants/sizes", 48, True),
        ("Flags", 48, False), ("Exclusive groups", 49, False), ("may_be_absent", 50, True),
    ])
    expect("label colon outside the bold", led.item("may_be_absent").text, "none. ")
    expect("several labels on a line", led.item("Variants/sizes").text, "none.")
    expect("button parts", [(p.names, p.parent, p.element, p.fields, p.line) for p in b.parts], [
        (["body"], None, False, ["fill", "stroke", "corners"], 18),
        (["label"], None, True, ["text", "opacity", "focus", "padding"], 20),
        (["icon"], "label", False, ["icon", "opacity"], 21),
        (["prefix", "suffix"], None, False, ["text"], 22),
        (["year"], None, True, ["fill", "stroke"], 23),
    ])
    expect("part note", [p.note for p in b.parts],
           [None, "element; states hover", None, None, "element: the year cells, Q1.2"])
    expect("part text", b.parts[0].text, "`body` — the root, the bezel (a note: `not_a_field`): `fill`, `stroke` "
           "(its width, `x.y`),\n    `corners`. `label` and `icon` belong to it.")
    no_name = parse_part(1, ["the root — no name: `fill`."])
    expect("part without a name", (no_name.names, no_name.fields), ([], ["fill"]))
    expect("component parts", [(p.names, p.fields) for p in led.parts + bar.parts],
           [(["body"], ["fill"]), (["strip"], ["fill"])])
    expect("values", [(v.name, v.type, v.line) for v in b.values], [
        ("icon_text_gap_px", "Value<Px>", 25), ("rows", "Value<u32>", 25), ("columns", "Value<u32>", 25),
        ("kind", "Value<Key>", 26), ("inner_a", "Value<Ms>", 26), ("inner_b", "Value<Ms>", 26),
        ("focus_visible_when", None, 27),
    ])
    expect("no values", led.values, [])
    idx = catalogue_index(SELF_TEST_CATALOGUE)
    expect("index names", list(idx), ["button", "component.led", "component.bar"])
    expect("index parts", idx["button"]["parts"],
           {"body", "not_a_field", "fill", "stroke", "x", "y", "corners", "label", "icon", "text", "opacity",
            "focus", "padding", "margin", "prefix", "suffix", "year"})
    expect("index values", idx["button"]["values"],
           {"icon_text_gap_px", "rows", "columns", "kind", "token", "x", "adwaita", "inner_a", "inner_b", "false", "spare", "focus_visible_when",
           "alue", "u32", "ey", "s", "ind"})   # names_in reads lowercase runs: `Value<Px>` gives `alue`
    expect("index variants", idx["button"]["variants"], {"flat", "pill", "adwaita", "@osd"})
    expect("index flags", idx["button"]["flags"], {"empty", "@busy"})
    expect("index skips may_be_absent and today", "font" in idx["button"]["parts"] | idx["button"]["values"], False)
    expect("index combined label", (idx["component.led"]["variants"], idx["component.led"]["flags"]),
           (set(), {"@on"}))
    names, groups = depth0_tokens("`a` (`b` (`c`)) `d`")
    expect("depth0 names", [x[2] for x in names], ["a", "d"])
    expect("depth0 groups", groups, [(4, 15)])
    t = "parts: `icon` (`adwaita`); fields: `body.fill` (all); values: `gap`."
    expect("absence sections", [(lab, t[a:b]) for lab, a, b in absence_sections(t)],
           [("parts", " `icon` (`adwaita`); "), ("fields", " `body.fill` (all); "), ("values", " `gap`.")])
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def main(argv: list[str]) -> int:
    if argv[1:] == ["--self-test"]:
        return self_test()
    if len(argv) > 2 or (argv[1:] and argv[1].startswith("-")):
        print(__doc__, file=sys.stderr)
        return 2
    try:
        es = read(argv[1] if argv[1:] else CATALOGUE)
    except OSError as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for e in es:
        print(f"{e.line}\t{e.kind}\t{e.name}\t{','.join('/'.join(p.names) for p in e.parts)}\t"
              f"{','.join(v.name for v in e.values)}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
