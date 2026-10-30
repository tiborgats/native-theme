#!/usr/bin/env python3
"""Check the 0.8.0 ledger's triage: every open item has one row, every row one item.

The plan's M0 (docs/todo_v0.7.1_model-completeness-plan.md, *Triage*) gives
each open item of five sources a destination in the ledger's `## Triage`
section (docs/todo_v0.7.1_model-completeness-ledger.md). This script extracts
the items again and matches the ledger's rows to them; with `--final` it also
checks the rest of the plan's M4 gate.

Run from the repository root:

    python3 scripts/check_ledger.py
    python3 scripts/check_ledger.py --ledger <copy of the ledger>
    python3 scripts/check_ledger.py --final --staged-file <path>
    python3 scripts/check_ledger.py --self-test

It prints one line per finding, `<file>:<line>: <what>`, and a summary line
with the counts. Exit 0 when there is none, 1 when there are some, 2 on an
input it cannot read or a wrong command line.

Rows
----
The ledger's `## Triage` section holds a table `| Source | Heading | Item |
Destination | Reason |`; `\\|` is a literal bar inside a cell. Source is one
of SOURCES. Heading is the item's heading line as written, its hashes
included. Item is the item's opening words between `“` and `”` (followed, for
docs/todo.md, by ` · l. N`, the line number, informational and not checked),
or a parenthesised label for an item without a quotable opening.

Items
-----
docs/todo.md: each `- [ ]` line, indented or not. Its heading is the nearest
`### ` line since the last `## ` line, else that `## ` line; `####` and
deeper start no heading. Its words are the line's after the box and the
`[tag]` (a `[...]` not followed by `(`: a leading Markdown link is words).
An item with no words is a finding of its own: no row can name it.

catalogue (docs/todo_v0.7.1_model-completeness-spec-widgets.md): each line
starting `- ` in `## Open items` (to the next `## ` line), under the heading
`## Open items`, named by the line's first backticked span, backticks
included; a bullet without one is an input the rule cannot read.

ROADMAP.md: in `## Later — the connectors`, each `### ` line (Item
`(subsection)`) and each line starting `- ` after it, its words the line's
after the dash; a bullet before the first `### ` line takes the `## ` line as
its heading. `## Also in the 0.7.x releases` is one item, its words the
section's first non-blank line; a bullet or `### ` line in that section is an
input the rule cannot read.

The two connector plans have no item convention: PLAN_ITEMS holds their items
by heading line and label, each an item while its heading line is in the
file (a missing file has none).

Matching. A row names an item when Source and Heading are equal and the row's
opening words are the item's first words (a catalogue item: its name, whole);
a labelled row names the item with that label.

Findings: an item without a row, or with several (a row naming several items
counts for none); a row naming no item (closed or moved: remove or update the
row) or several (lengthen its opening words); an unknown Source; an Item of
neither form; a Destination not in DESTINATIONS; a row going `after 0.8.0`,
`not this plan's work` or `settled by the model` without a Reason.

--final (the plan's M4 gate, from M4 on)
----------------------------------------
Also:

- `docs/facts/imported/` absent or empty: each file in it is a finding.
- S19.2's staged inheritance file gone. Neither the plan nor the
  specification names its path (`convert_inheritance_rules.py` writes it at
  M1a), so `--final` needs `--staged-file PATH`; the file existing is a
  finding.
- The legacy evidence list empty: `scripts/check_fact_citations.py` assigns
  it at the top level as `LEGACY_EVIDENCE = <literal list, tuple, set or
  dict>`. A missing script (the plan keeps it from M0 on), a missing or
  non-literal assignment, and a non-empty list are findings.
- The ledger's `## Unmeasured register` table (`| Key and combination |
  Attempts and evidence | Options | Decision |`): a row with an empty
  Decision is a finding, and so is a missing section or table.
- Every measured fact with a drift comparison. A measured fact is a
  `[[fact]]` of `docs/facts/<platform>/*.toml` (not `imported/`, not
  `palettes/`, not `staged/`, S19.2's staged inheritance bases) with a `[[fact.cite]]` of `kind = "measured"`; M4's drift run
  records each one's comparison in the ledger, which has no per-fact table for
  it yet. Until it has one and this script matches each measured fact
  (platform, key, `when`) to exactly one of its lines with the run's URL, the
  rule is not checked and is always a finding, stating the number of measured
  facts.
"""

from __future__ import annotations

import ast
import re
import sys
import tempfile
import tomllib
from pathlib import Path
from typing import NamedTuple

DOCS = Path("docs")
LEDGER = DOCS / "todo_v0.7.1_model-completeness-ledger.md"
SOURCES = {
    "docs/todo.md": DOCS / "todo.md",
    "catalogue": DOCS / "todo_v0.7.1_model-completeness-spec-widgets.md",
    "ROADMAP.md": Path("ROADMAP.md"),
    "docs/todo_gpui-full-theme.md": DOCS / "todo_gpui-full-theme.md",
    "docs/todo_iced-full-theme-geometry.md": DOCS / "todo_iced-full-theme-geometry.md",
}
PLAN_ITEMS = {
    "docs/todo_gpui-full-theme.md": [
        ("# Full per-widget theming for the gpui connector", "(whole plan)"),
        ("### What native-theme provides vs what the connector uses", "(gap table)"),
        ("### Execution plan", "(Option F2, Steps 1–5)"),
    ],
    "docs/todo_iced-full-theme-geometry.md": [
        ("# Iced Connector: Full Theme Geometry Support", "(whole plan)"),
        ("## Summary: Functions to Implement", "(remaining functions)"),
        ("### Font Configuration Helper", "(apply_fonts)"),
        ("### Widget Metrics Helpers (already partially exist)", "(checkbox_size … tooltip_padding)"),
        ("### Spacing Helpers", "(spacing)"),
    ],
}
UNITS = [
    "tokens and conditions", "placement", "readers", "window and chrome", "overlays", "buttons and toggles",
    "fields", "indicators", "containers", "typed widgets batch", "keyed components",
]
LANES = ["gnome", "kde", "macos", "ios", "windows", "material", "palettes"]
SPIKES = ["M", "G", "K", "I", "W", "A"]
NEEDS_REASON = {"settled by the model", "after 0.8.0", "not this plan's work"}
DESTINATIONS = {
    "M0", "M1a", "M1b", "D6", "M4", "M5", "M6",
    *(f"M2.{n}" for n in range(1, 8)),
    *(f"M3 unit {u}" for u in UNITS),
    *(f"lane {p}" for p in LANES),
    *(f"spike {s}" for s in SPIKES),
    *NEEDS_REASON,
}
TRIAGE_HEADER = ["Source", "Heading", "Item", "Destination", "Reason"]
UNMEASURED_HEADER = ["Key and combination", "Attempts and evidence", "Options", "Decision"]

IMPORTED = DOCS / "facts" / "imported"
FACTS = DOCS / "facts"
NOT_PLATFORMS = {"imported", "palettes", "staged"}
CITATIONS = Path("scripts/check_fact_citations.py")
LEGACY = "LEGACY_EVIDENCE"

TODO_ITEM = re.compile(r"^\s*- \[ \] ?(.*)$")
TAG = re.compile(r"^\[[^\]]+\](?!\()\s*(.*)$")
QUOTED = re.compile(r"^“([^”]*)”(?: · l\. \d+)?$")
LABEL = re.compile(r"^\(.+\)$")
NAME = re.compile(r"`[^`]+`")


class InputError(Exception):
    pass


class Item(NamedTuple):
    source: str
    heading: str
    line: int
    words: tuple[str, ...] = ()
    label: str = ""
    exact: bool = False


class Row(NamedTuple):
    line: int
    source: str
    heading: str
    item: str
    destination: str
    reason: str


def cells(line: str) -> list[str]:
    """A table line's cells; `\\|` is a literal bar inside a cell, as in Markdown tables."""
    return [c.strip().replace("\\|", "|") for c in re.split(r"(?<!\\)\|", line.strip().strip("|"))]


def section(lines: list[str], title: str) -> list[tuple[int, str]] | None:
    """The numbered lines after the line `title` up to the next `## ` line; None without one."""
    for i, line in enumerate(lines):
        if line.rstrip() == title:
            out = []
            for n, l in enumerate(lines[i + 1:], i + 2):
                if l.startswith("## "):
                    break
                out.append((n, l))
            return out
    return None


def table(sec: list[tuple[int, str]], header: list[str]) -> list[tuple[int, list[str]]] | None:
    """The rows of the table in `sec` whose header is `header`; None without one."""
    start = next((k for k, (_, l) in enumerate(sec) if l.startswith("|") and cells(l) == header), None)
    if start is None:
        return None
    out = []
    for n, line in sec[start + 2:]:
        if not line.startswith("|"):
            break
        c = cells(line)
        if len(c) != len(header):
            raise InputError(f"line {n}: {len(c)} cells, the table has {len(header)}")
        out.append((n, c))
    return out


def triage_rows(ledger_text: str) -> list[Row]:
    sec = section(ledger_text.splitlines(), "## Triage")
    if sec is None:
        raise InputError("no `## Triage` section")
    rows = table(sec, TRIAGE_HEADER)
    if rows is None:
        raise InputError("`## Triage` has no `| Source | Heading | Item | Destination | Reason |` table")
    return [Row(n, *c) for n, c in rows]


def todo_items(text: str) -> list[Item]:
    out = []
    h2 = h3 = None
    for n, line in enumerate(text.splitlines(), 1):
        if line.startswith("## "):
            h2, h3 = "## " + line[3:].strip(), None
            continue
        if line.startswith("### "):
            h3 = "### " + line[4:].strip()
            continue
        m = TODO_ITEM.match(line)
        if not m:
            continue
        rest = m.group(1)
        t = TAG.match(rest)
        if t:
            rest = t.group(1)
        heading = h3 or h2
        if heading is None:
            raise InputError(f"line {n}: an open item before any `## ` heading")
        out.append(Item("docs/todo.md", heading, n, tuple(rest.split())))
    return out


def catalogue_items(text: str) -> list[Item]:
    out = []
    for n, line in section(text.splitlines(), "## Open items") or []:
        if not line.startswith("- "):
            continue
        m = NAME.search(line)
        if not m:
            raise InputError(f"line {n}: an Open items bullet without a backticked name")
        out.append(Item("catalogue", "## Open items", n, (m.group(0),), exact=True))
    return out


def roadmap_items(text: str) -> list[Item]:
    lines = text.splitlines()
    out = []
    later = "## Later — the connectors"
    heading = later
    for n, line in section(lines, later) or []:
        if line.startswith("### "):
            heading = "### " + line[4:].strip()
            out.append(Item("ROADMAP.md", heading, n, label="(subsection)"))
        elif line.startswith("- "):
            out.append(Item("ROADMAP.md", heading, n, tuple(line[2:].split())))
    also = "## Also in the 0.7.x releases"
    sec = section(lines, also)
    if sec is not None:
        for n, line in sec:
            if line.startswith(("- ", "### ")):
                raise InputError(f"line {n}: `{also}` has a bullet or subsection; its one-item rule cannot read it")
        first = next(((n, l) for n, l in sec if l.strip()), None)
        if first is None:
            raise InputError(f"`{also}` has no text")
        out.append(Item("ROADMAP.md", also, first[0], tuple(first[1].split())))
    return out


def plan_items(source: str, text: str | None) -> list[Item]:
    if text is None:
        return []
    lines = [l.rstrip() for l in text.splitlines()]
    out = []
    for heading, label in PLAN_ITEMS[source]:
        if heading in lines:
            out.append(Item(source, heading, lines.index(heading) + 1, label=label))
    return out


def extract(texts: dict[str, str | None]) -> list[Item]:
    """Every item of the sources; `texts` maps each source to its text, None for a missing connector plan."""
    out = []
    for source, read in (("docs/todo.md", todo_items), ("catalogue", catalogue_items), ("ROADMAP.md", roadmap_items)):
        try:
            out += read(texts[source])
        except InputError as err:
            raise InputError(f"{SOURCES[source]}: {err}") from err
    for source in PLAN_ITEMS:
        out += plan_items(source, texts[source])
    return out


def names(opening: tuple[str, ...] | None, label: str, row: Row, item: Item) -> bool:
    if row.source != item.source or row.heading != item.heading:
        return False
    if opening is None:
        return item.label == label
    if item.label:
        return False
    if item.exact:
        return item.words == opening
    return bool(opening) and item.words[:len(opening)] == opening


def describe(item: Item) -> str:
    if item.label:
        return f"{item.heading} {item.label}"
    words = " ".join(item.words[:6])
    return f"{item.heading} “{words}{' …' if len(item.words) > 6 else ''}”"


def problems(ledger: str, rows: list[Row], items: list[Item]) -> list[tuple[str, int, str]]:
    found = []
    hits = [0] * len(items)
    for row in rows:
        if row.source not in SOURCES:
            found.append((ledger, row.line, f"unknown source {row.source!r}"))
        else:
            q = QUOTED.match(row.item)
            if q:
                opening, label = tuple(q.group(1).split()), ""
            elif LABEL.match(row.item):
                opening, label = None, row.item
            else:
                opening = label = None
            if label is None:
                found.append((ledger, row.line, f"item {row.item!r} is neither “opening words” nor a (label)"))
            else:
                named = [k for k, item in enumerate(items) if names(opening, label, row, item)]
                if len(named) == 1:
                    hits[named[0]] += 1
                elif not named:
                    found.append((ledger, row.line, f"names no open item under {row.heading!r} of {row.source} "
                                                    "(closed or moved: remove or update the row)"))
                else:
                    where = ", ".join(str(items[k].line) for k in named)
                    found.append((ledger, row.line, f"names {len(named)} items of {row.source} (lines {where}): "
                                                    "lengthen its opening words"))
        if row.destination not in DESTINATIONS:
            found.append((ledger, row.line, f"unknown destination {row.destination!r}"))
        elif row.destination in NEEDS_REASON and not row.reason:
            found.append((ledger, row.line, f"destination {row.destination!r} without a reason"))
    for item, n in zip(items, hits):
        if not item.words and not item.label:
            found.append((str(SOURCES[item.source]), item.line, f"an open item with no words under {item.heading!r}: "
                                                                 "no row can name it (write it out or remove it)"))
        elif n != 1:
            what = "no triage row" if n == 0 else f"{n} triage rows"
            found.append((str(SOURCES[item.source]), item.line, f"{what}: {describe(item)}"))
    return found


def legacy_problems(name: str, text: str | None) -> list[tuple[str, int, str]]:
    if text is None:
        return [(name, 1, f"missing: the plan keeps it from M0 on, and M4 needs its {LEGACY} empty")]
    try:
        tree = ast.parse(text)
    except SyntaxError as err:
        raise InputError(f"{name}: {err}") from err
    for node in tree.body:
        if isinstance(node, ast.Assign):
            targets = node.targets
        elif isinstance(node, ast.AnnAssign):
            targets = [node.target]
        else:
            continue
        if not any(isinstance(t, ast.Name) and t.id == LEGACY for t in targets):
            continue
        try:
            value = ast.literal_eval(node.value) if node.value is not None else None
        except ValueError:
            value = None
        if not isinstance(value, (list, tuple, set, dict)):
            return [(name, node.lineno, f"{LEGACY} is not a literal list, tuple, set or dict")]
        if value:
            return [(name, node.lineno, f"{LEGACY} lists {len(value)} entries; M4 empties it")]
        return []
    return [(name, 1, f"no top-level {LEGACY} assignment")]


def unmeasured_problems(ledger: str, ledger_text: str) -> list[tuple[str, int, str]]:
    sec = section(ledger_text.splitlines(), "## Unmeasured register")
    if sec is None:
        return [(ledger, 1, "no `## Unmeasured register` section")]
    try:
        rows = table(sec, UNMEASURED_HEADER)
    except InputError as err:
        raise InputError(f"{ledger}: {err}") from err
    if rows is None:
        return [(ledger, sec[0][0] - 1, "the unmeasured register has no table")]
    return [(ledger, n, f"unmeasured {c[0]!r} has no decision") for n, c in rows if not c[3]]


def measured_facts(texts: dict[str, str]) -> int:
    count = 0
    for name, text in texts.items():
        try:
            data = tomllib.loads(text)
        except tomllib.TOMLDecodeError as err:
            raise InputError(f"{name}: {err}") from err
        for fact in data.get("fact", []):
            if any(c.get("kind") == "measured" for c in fact.get("cite", [])):
                count += 1
    return count


def final_problems(ledger: str, ledger_text: str, imported: list[str], staged: str, staged_exists: bool,
                   citations: str | None, measured: int) -> list[tuple[str, int, str]]:
    found = [(f, 1, f"{IMPORTED}/ must be empty at M4") for f in imported]
    if staged_exists:
        found.append((staged, 1, "S19.2's staged inheritance file must be gone at M4"))
    found += legacy_problems(str(CITATIONS), citations)
    found += unmeasured_problems(ledger, ledger_text)
    lines = ledger_text.splitlines()
    drift = next((n for n, l in enumerate(lines, 1) if l.rstrip() == "## Spikes and drift runs"), 1)
    found.append((ledger, drift, f"drift comparison not checked: the ledger has no per-fact drift table to match "
                                 f"the {measured} measured facts against"))
    return found


def read_sources() -> dict[str, str | None]:
    texts: dict[str, str | None] = {}
    for source, path in SOURCES.items():
        if source in PLAN_ITEMS and not path.exists():
            texts[source] = None
        else:
            texts[source] = path.read_text(encoding="utf-8")
    return texts


def check_final(ledger: str, ledger_text: str, staged: Path) -> list[tuple[str, int, str]]:
    imported = sorted(str(p) for p in IMPORTED.rglob("*") if not p.is_dir()) if IMPORTED.is_dir() else []
    citations = CITATIONS.read_text(encoding="utf-8") if CITATIONS.exists() else None
    return final_problems(ledger, ledger_text, imported, str(staged), staged.exists(), citations,
                          measured_facts(platform_fact_texts(FACTS)))


def platform_fact_texts(facts_dir: Path) -> dict[str, str]:
    """{path: text} of every `<platform>/*.toml` under facts_dir, the folders of NOT_PLATFORMS left out."""
    facts = {}
    if facts_dir.is_dir():
        for d in sorted(p for p in facts_dir.iterdir() if p.is_dir() and p.name not in NOT_PLATFORMS):
            for f in sorted(d.glob("*.toml")):
                facts[str(f)] = f.read_text(encoding="utf-8")
    return facts


def self_test() -> int:
    todo = """# Todo
## Section one
- [ ] [1.0] Orphan under the section heading only
### Alpha
- [ ] [v0.7.x] Research the thing and write it
- [ ] Research the thing and fix it
  - [ ] Indented item kept under Alpha
#### Research
- [ ] Deeper heading keeps the alpha heading
- [x] Closed item is not one
## Section two
### Beta
- [ ] Beta item without a row
- [ ] [`docs/x.md`](docs/x.md) needs a rewrite
- [ ] [v0.7.x] [the guide](g.md) is stale
- [ ]
"""
    cat = """## Entry format
- `not_open`: outside the section
## Open items

- `context_help`: whether Breeze's shadow
  applies, continued
- `ink_toolbar` and `ink_part`: whether
## Entry
"""
    roadmap = """## Later — the connectors

Intro.

### Sub one

Text.

- first bullet with enough words
- second bullet with enough words
## Also in the 0.7.x releases

[`docs/todo.md`](docs/todo.md) tags 29 open items
and more.

## Beyond
- not an item
"""
    gpui = "# Full per-widget theming for the gpui connector\n\n### Execution plan  \n"
    texts = {"docs/todo.md": todo, "catalogue": cat, "ROADMAP.md": roadmap,
             "docs/todo_gpui-full-theme.md": gpui, "docs/todo_iced-full-theme-geometry.md": None}
    ledger = """# Ledger
## Unmeasured register

| Key and combination | Attempts and evidence | Options | Decision |
|---|---|---|---|
| `button.body.fill` {hover} | run 1 | real hardware | catalogue correction |
| `menu.body.shadow` | run 2 | another image | |

## Spikes and drift runs

## Triage

Prose with | a bar.

| Source | Heading | Item | Destination | Reason |
|---|---|---|---|---|
| docs/todo.md | ## Section one | “Orphan under the section” · l. 3 | M0 | |
| docs/todo.md | ### Alpha | “Research the thing and write” | M1a | |
| docs/todo.md | ### Alpha | “Research the thing and” | M1b | |
| docs/todo.md | ### Alpha | “Research the thing and fix” | M2.8 | |
| docs/todo.md | ### Alpha | “Indented item kept under” | after 0.8.0 | |
| docs/todo.md | ### Alpha | “Deeper heading keeps the” | M3 unit readers | |
| docs/todo.md | ### Alpha | “Deeper heading keeps the alpha” | M3 unit seats | |
| docs/todo.md | ### Alpha | “Research the thing and wri” | spike M | |
| docs/todo.md | ### Research | “Deeper heading keeps the” | M4 | |
| catalogue | ## Open items | “`context_help`” | settled by the model | The \\| model carries it. |
| catalogue | ## Open items | “`ink`” | not this plan's work | Upstream. |
| ROADMAP.md | ### Sub one | (subsection) | M5 | |
| ROADMAP.md | ### Sub one | “first bullet with enough” | M6 | |
| ROADMAP.md | ### Sub one | “second bullet with” | lane linux | |
| ROADMAP.md | ## Also in the 0.7.x releases | “[`docs/todo.md`](docs/todo.md) tags 29” | lane palettes | |
| docs/todo_gpui-full-theme.md | # Full per-widget theming for the gpui connector | (whole plan) | spike A | |
| docs/todo_gpui-full-theme.md | ### What native-theme provides vs what the connector uses | (gap table) | M2.1 | |
| docs/todo_iced-full-theme-geometry.md | ### Spacing Helpers | (spacing) | D6 | |
| docs/elsewhere.md | ### Alpha | “Research” | M0 | |
| docs/todo.md | ### Alpha | Research the thing | lane kde | |
| docs/todo.md | ### Beta | “[`docs/x.md`](docs/x.md) needs” | M0 | |
| docs/todo.md | ### Beta | “[the guide](g.md) is” | M0 | |

After the table.
"""
    failures = []
    items = extract(texts)
    got_items = [(i.source, i.heading, i.line, i.label or " ".join(i.words[:2])) for i in items]
    want_items = [
        ("docs/todo.md", "## Section one", 3, "Orphan under"),
        ("docs/todo.md", "### Alpha", 5, "Research the"),
        ("docs/todo.md", "### Alpha", 6, "Research the"),
        ("docs/todo.md", "### Alpha", 7, "Indented item"),
        ("docs/todo.md", "### Alpha", 9, "Deeper heading"),
        ("docs/todo.md", "### Beta", 13, "Beta item"),
        ("docs/todo.md", "### Beta", 14, "[`docs/x.md`](docs/x.md) needs"),
        ("docs/todo.md", "### Beta", 15, "[the guide](g.md)"),
        ("docs/todo.md", "### Beta", 16, ""),
        ("catalogue", "## Open items", 5, "`context_help`"),
        ("catalogue", "## Open items", 7, "`ink_toolbar`"),
        ("ROADMAP.md", "### Sub one", 5, "(subsection)"),
        ("ROADMAP.md", "### Sub one", 9, "first bullet"),
        ("ROADMAP.md", "### Sub one", 10, "second bullet"),
        ("ROADMAP.md", "## Also in the 0.7.x releases", 13, "[`docs/todo.md`](docs/todo.md) tags"),
        ("docs/todo_gpui-full-theme.md", "# Full per-widget theming for the gpui connector", 1, "(whole plan)"),
        ("docs/todo_gpui-full-theme.md", "### Execution plan", 3, "(Option F2, Steps 1–5)"),
    ]
    if got_items != want_items:
        failures.append(f"items: got {got_items}, want {want_items}")
    rows = triage_rows(ledger)
    if rows[9].reason != "The | model carries it." or len(rows) != 22:
        failures.append(f"rows: got {len(rows)}, the tenth {rows[9]}")
    got = problems("L", rows, items)
    todo_md, cat_md = str(SOURCES["docs/todo.md"]), str(SOURCES["catalogue"])
    want = [
        ("L", 19, "names 2 items of docs/todo.md (lines 5, 6): lengthen its opening words"),
        ("L", 20, "unknown destination 'M2.8'"),
        ("L", 21, "destination 'after 0.8.0' without a reason"),
        ("L", 23, "unknown destination 'M3 unit seats'"),
        ("L", 24, "names no open item under '### Alpha' of docs/todo.md (closed or moved: remove or update the row)"),
        ("L", 25, "names no open item under '### Research' of docs/todo.md "
                  "(closed or moved: remove or update the row)"),
        ("L", 27, "names no open item under '## Open items' of catalogue (closed or moved: remove or update the row)"),
        ("L", 30, "unknown destination 'lane linux'"),
        ("L", 33, "names no open item under '### What native-theme provides vs what the connector uses' of "
                  "docs/todo_gpui-full-theme.md (closed or moved: remove or update the row)"),
        ("L", 34, "names no open item under '### Spacing Helpers' of docs/todo_iced-full-theme-geometry.md "
                  "(closed or moved: remove or update the row)"),
        ("L", 35, "unknown source 'docs/elsewhere.md'"),
        ("L", 36, "item 'Research the thing' is neither “opening words” nor a (label)"),
        (todo_md, 9, "2 triage rows: ### Alpha “Deeper heading keeps the alpha heading”"),
        (todo_md, 13, "no triage row: ### Beta “Beta item without a row”"),
        (todo_md, 16, "an open item with no words under '### Beta': no row can name it (write it out or remove it)"),
        (cat_md, 7, "no triage row: ## Open items “`ink_toolbar`”"),
        (str(SOURCES["docs/todo_gpui-full-theme.md"]), 3, "no triage row: ### Execution plan (Option F2, Steps 1–5)"),
    ]
    if got != want:
        failures.append("problems:\n  got  " + "\n       ".join(map(str, got))
                        + "\n  want " + "\n       ".join(map(str, want)))
    clean = [r for r in rows if not any(p[0] == "L" and p[1] == r.line for p in want)]
    left = [p for p in problems("L", clean, items) if p[0] == "L"]
    if left:
        failures.append(f"the passing rows: got {left}")

    legacy_cases = [
        (None, [("c.py", 1, "missing: the plan keeps it from M0 on, and M4 needs its LEGACY_EVIDENCE empty")]),
        ("LEGACY_EVIDENCE = []\n", []),
        ("x = 1\nLEGACY_EVIDENCE: frozenset[str] = set()\n", []),
        ("x = 1\nLEGACY_EVIDENCE: list[str] = [\n    'docs/evidence/os-probe/r1',\n]\n",
         [("c.py", 2, "LEGACY_EVIDENCE lists 1 entries; M4 empties it")]),
        ("LEGACY_EVIDENCE = sorted(RUNS)\n",
         [("c.py", 1, "LEGACY_EVIDENCE is not a literal list, tuple, set or dict")]),
        ("def f():\n    LEGACY_EVIDENCE = []\n", [("c.py", 1, "no top-level LEGACY_EVIDENCE assignment")]),
    ]
    for text, want_legacy in legacy_cases:
        got_legacy = legacy_problems("c.py", text)
        if got_legacy != want_legacy:
            failures.append(f"legacy {text!r}: got {got_legacy}, want {want_legacy}")
    got_unmeasured = unmeasured_problems("L", ledger)
    if got_unmeasured != [("L", 7, "unmeasured '`menu.body.shadow`' has no decision")]:
        failures.append(f"unmeasured: got {got_unmeasured}")
    if unmeasured_problems("L", "# Ledger\n") != [("L", 1, "no `## Unmeasured register` section")]:
        failures.append("unmeasured: a ledger without the section passed")
    if unmeasured_problems("L", "## Unmeasured register\n\nProse.\n")[0][2] != "the unmeasured register has no table":
        failures.append("unmeasured: a register without a table passed")
    facts = """[[fact]]
key = "button.body.fill"
value = "#ffffff"
[[fact.cite]]
kind = "published"
[[fact.cite]]
kind = "measured"
[[fact]]
key = "button.body.radius"
value = 6
[[fact.cite]]
kind = "read"
"""
    if measured_facts({"f.toml": facts, "s.toml": "[[widget]]\nname = 'button'\n"}) != 1:
        failures.append("measured facts: not 1")
    with tempfile.TemporaryDirectory() as tmp:
        for folder in ("gnome", "imported", "palettes", "staged"):
            (Path(tmp) / folder).mkdir()
            (Path(tmp) / folder / "a.toml").write_text(facts, encoding="utf-8")
        got = sorted(Path(f).parent.name for f in platform_fact_texts(Path(tmp)))
        if got != ["gnome"]:
            failures.append(f"platform folders: got {got}, want ['gnome'] (imported, palettes, staged are none)")
    got_final = final_problems("L", ledger, ["docs/facts/imported/a.toml"], "staged.toml", True, "LEGACY_EVIDENCE = ()",
                               1)
    want_final = [
        ("docs/facts/imported/a.toml", 1, "docs/facts/imported/ must be empty at M4"),
        ("staged.toml", 1, "S19.2's staged inheritance file must be gone at M4"),
        ("L", 7, "unmeasured '`menu.body.shadow`' has no decision"),
        ("L", 9, "drift comparison not checked: the ledger has no per-fact drift table to match the 1 measured facts "
                 "against"),
    ]
    if got_final != want_final:
        failures.append(f"final: got {got_final}, want {want_final}")
    if final_problems("L", ledger.replace("| |\n", "| keep |\n", 1), [], "s", False, "LEGACY_EVIDENCE = []", 0) != [
            ("L", 9, "drift comparison not checked: the ledger has no per-fact drift table to match the 0 measured "
                     "facts against")]:
        failures.append("final: the drift rule passed or another rule failed on a clean input")

    bad_inputs = [
        (lambda: triage_rows(ledger.replace("## Triage", "## Sorting")), "no Triage section"),
        (lambda: triage_rows(ledger.replace("| Reason |", "| Why |")), "no triage table"),
        (lambda: triage_rows(ledger.replace("| M1a | |", "| M1a |")), "four cells"),
        (lambda: extract({**texts, "ROADMAP.md": roadmap.replace("and more.", "- a bullet")}), "Also bullet"),
        (lambda: extract({**texts, "catalogue": cat.replace("`ink_toolbar` and `ink_part`", "ink")}), "no name"),
        (lambda: extract({**texts, "docs/todo.md": "- [ ] before any heading\n"}), "item before a heading"),
        (lambda: measured_facts({"f.toml": "[[fact]\n"}), "bad TOML"),
        (lambda: legacy_problems("c.py", "LEGACY_EVIDENCE = [\n"), "bad Python"),
    ]
    for call, label in bad_inputs:
        try:
            call()
            failures.append(f"{label}: accepted")
        except InputError:
            pass
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def main(argv: list[str]) -> int:
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    ledger, final, staged = LEDGER, False, None
    while args:
        arg = args.pop(0)
        if arg == "--final":
            final = True
        elif arg in ("--ledger", "--staged-file") and args:
            value = Path(args.pop(0))
            if arg == "--ledger":
                ledger = value
            else:
                staged = value
        else:
            print(__doc__, file=sys.stderr)
            return 2
    if final != (staged is not None):
        print("error: --final and --staged-file PATH go together (the plan names no path for S19.2's staged file)",
              file=sys.stderr)
        return 2
    try:
        ledger_text = ledger.read_text(encoding="utf-8")
        try:
            rows = triage_rows(ledger_text)
        except InputError as err:
            raise InputError(f"{ledger}: {err}") from err
        items = extract(read_sources())
        found = problems(str(ledger), rows, items)
        if final:
            found += check_final(str(ledger), ledger_text, staged)
    except (OSError, UnicodeDecodeError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for name, line, what in found:
        print(f"{name}:{line}: {what}")
    per_source = ", ".join(f"{s} {sum(1 for i in items if i.source == s)}" for s in SOURCES)
    print(f"{len(rows)} triage rows, {len(items)} items ({per_source}){', --final' if final else ''}: "
          f"{len(found)} findings", file=sys.stderr)
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
