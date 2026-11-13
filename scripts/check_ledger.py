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
included, its words the line's after the dash; a bullet without a backticked
span is an input the rule cannot read. Several items may share a name.

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
opening words are the item's first words (a catalogue item also when they are
its name alone, whole, so items sharing a name are told apart by longer
opening words); a labelled row names the item with that label.

Findings: an item without a row, or with several (a row naming several items
counts for none); a row naming no item (closed or moved: remove or update the
row) or several (lengthen its opening words); an unknown Source; an Item of
neither form; a Destination not in DESTINATIONS; a row going `after the plan's
releases`, `not this plan's work` or `settled by the model` without a Reason.
(`after the plan's releases`, formerly `after 0.8.0`: past 0.8.0 and every
follow-up release of the plan's D13, so never one of them.)

The Unmeasured register (every run)
-----------------------------------
The ledger's `## Unmeasured register` table (`| Id | Kind | Key and
combination | Attempts and evidence | Options | Decision |`): a missing
section or table, an Id not `U` and a number, an Id two rows share, and a
Kind that is neither `b` (a measurement to make: a harness or probe pass, an
analysis of committed evidence, a source read; its lane's queue) nor `d` (a
decision: a value with no route, or a choice; the maintainer's) are findings
(the plan's *How values are checked*, ADD-A146). Every S17
pending fact (a `[[fact]]` with `pending` of `docs/facts/<platform>/*.toml`,
the folders of NOT_PLATFORMS left out) and every pending declaration (a
state model's `default_size = { pending = "U<n>" }`, the widget's or a
`sizes_by_variant` or `sizes_by_context` entry's, S10's *A pending default*,
ADD-A131) names a row: one whose id is no row's Id is a finding. Several may
name one row.

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
- The enabled units (S17, S21.3; ADD-A135, the plan's D13): a release
  passes this gate over the units `scripts/widget_units.toml` enables, read
  and matched as scripts/check_facts_trace.py reads and matches them
  (read_units, in_unit; an `input.…` key that is no part or value of the
  `input` widget is an S9 input, never that widget, and the model's core,
  lying in every enabled unit's set). A pending fact is of the
  enabled units where its key lies in one and S17's predicate holds it on a
  preset of its platform (ADD-A147; check_facts_trace.Depth: its `when`
  holds in a combination of the names at depth its state model draws, its
  part drawn there); a state model's pending entry
  (S10's pending default size) where its widget does (S17: it counts as a
  pending fact of its widget's unit).
- The Unmeasured register: a `b` row of the enabled units is a finding, a
  measurement not made; a `d` row of them with an empty Decision is a
  finding. A row is of a unit not enabled where every pending fact or entry
  naming it is, and at least one does; a row none names is the enabled
  units'.
- No S17 pending fact or declaration of the enabled units: each is a
  finding, whether or not its id names a row (each release is made with
  none).
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

import check_facts_trace
import spec_catalogue

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
NEEDS_REASON = {"settled by the model", "after the plan's releases", "not this plan's work"}
DESTINATIONS = {
    "M0", "M1a", "M1b", "D6", "M4", "M5", "M6",
    *(f"M2.{n}" for n in range(1, 8)),
    *(f"M3 unit {u}" for u in UNITS),
    *(f"lane {p}" for p in LANES),
    *(f"spike {s}" for s in SPIKES),
    *NEEDS_REASON,
}
TRIAGE_HEADER = ["Source", "Heading", "Item", "Destination", "Reason"]
UNMEASURED_HEADER = ["Id", "Kind", "Key and combination", "Attempts and evidence", "Options", "Decision"]
KINDS = ("b", "d")   # a measurement to make (the lane's backlog); a decision (the maintainer's)

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
ID = re.compile(r"U[1-9][0-9]*")
PENDING = re.compile(r"^\s*pending\s*=")


class InputError(Exception):
    pass


class Item(NamedTuple):
    source: str
    heading: str
    line: int
    words: tuple[str, ...] = ()
    label: str = ""
    name: str = ""


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
        out.append(Item("catalogue", "## Open items", n, tuple(line[2:].split()), name=m.group(0)))
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
    if item.name and opening == (item.name,):
        return True
    return bool(opening) and item.words[:len(opening)] == opening


def describe(item: Item) -> str:
    if item.label:
        return f"{item.heading} {item.label}"
    if item.name:
        return f"{item.heading} “{item.name}”"
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


def register_rows(ledger: str, ledger_text: str) -> tuple[list[tuple[int, list[str]]] | None, list]:
    """(the Unmeasured register's rows, or None, and the problem when it has no section or table)."""
    sec = section(ledger_text.splitlines(), "## Unmeasured register")
    if sec is None:
        return None, [(ledger, 1, "no `## Unmeasured register` section")]
    try:
        rows = table(sec, UNMEASURED_HEADER)
    except InputError as err:
        raise InputError(f"{ledger}: {err}") from err
    if rows is None:
        return None, [(ledger, sec[0][0] - 1, "the unmeasured register has no table")]
    return rows, []


def unmeasured_problems(ledger: str, ledger_text: str, final: bool = True,
                        outside: frozenset | set = frozenset()) -> list[tuple[str, int, str]]:
    """The register's Ids and Kinds (every run): `U` and a number, no two rows sharing one, each Kind `b` or `d`;
    with `final`, also every `b` row (a measurement not made) and every `d` row with an empty Decision, but a row of
    a unit not enabled (its Id in `outside`: outside_ids())."""
    rows, found = register_rows(ledger, ledger_text)
    seen = set()
    for n, c in rows or []:
        rid, kind, key, decision = c[0], c[1], c[2], c[5]
        if not ID.fullmatch(rid):
            found.append((ledger, n, f"unmeasured id {rid!r} is not U and a number"))
        elif rid in seen:
            found.append((ledger, n, f"unmeasured id {rid} is used twice"))
        seen.add(rid)
        if kind not in KINDS:
            found.append((ledger, n, f"unmeasured {rid}: kind {kind!r} is neither b (a measurement to make) nor d "
                                     f"(a decision)"))
        elif final and rid not in outside:
            if kind == "b":
                found.append((ledger, n, f"unmeasured {key!r} is a measurement not made"))
            elif not decision:
                found.append((ledger, n, f"unmeasured {key!r} has no decision"))
    return found


def pending_facts(texts: dict[str, str]) -> list[tuple[str, int, str, object]]:
    """(file, line, key, id) of every S17 pending fact (a `[[fact]]` with `pending`) of the platform fact files."""
    out = []
    for name, text in texts.items():
        try:
            data = tomllib.loads(text)
        except tomllib.TOMLDecodeError as err:
            raise InputError(f"{name}: {err}") from err
        facts = [f for f in data.get("fact", []) if isinstance(f, dict) and "pending" in f]
        lines = [n for n, line in enumerate(text.splitlines(), 1) if PENDING.match(line)]
        for k, fact in enumerate(facts):
            out.append((name, lines[k] if len(lines) == len(facts) else 1, str(fact.get("key")), fact["pending"]))
    return out


DEFAULT_PENDING = re.compile(r"\bdefault_size\s*=\s*\{\s*pending\s*=\s*\"([^\"]*)\"")


def pending_declarations(texts: dict[str, str]) -> list[tuple[str, int, str, object]]:
    """(file, line, declaration, id) of every pending default size of the state models (`[[model]]` tables of the
    platform fact files; S10's *A pending default*, ADD-A131): the widget's `default_size = { pending = "U<n>" }` and
    each `sizes_by_variant` entry's (named by its variant) and `sizes_by_context` entry's (by its position), as S17
    names them in `[pending]`: `<widget>.default_size`, `<widget>.sizes_by_variant.<variant>.default_size`,
    `<widget>.sizes_by_context.<n>.default_size`. Each is placed on the first line not yet taken that states its id
    in a `default_size = { pending = … }`, else on line 1."""
    out = []
    for name, text in texts.items():
        try:
            data = tomllib.loads(text)
        except tomllib.TOMLDecodeError as err:
            raise InputError(f"{name}: {err}") from err
        found = [(n, m.group(1)) for n, line in enumerate(text.splitlines(), 1)
                 for m in DEFAULT_PENDING.finditer(line)]
        taken: set[int] = set()
        for model in data.get("model", []):
            if not isinstance(model, dict):
                continue
            widget = str(model.get("widget"))
            decls = [(f"{widget}.default_size", model.get("default_size"))]
            for k, sub in (("sizes_by_variant", "variant"), ("sizes_by_context", None)):
                entries = model.get(k) if isinstance(model.get(k), list) else []
                decls += [(f"{widget}.{k}.{e.get(sub) if sub else i}.default_size", e.get("default_size"))
                          for i, e in enumerate(entries) if isinstance(e, dict)]
            for decl, value in decls:
                if not (isinstance(value, dict) and "pending" in value):
                    continue
                pid = value["pending"]
                line = next((n for n, i in found if i == pid and n not in taken), 1)
                taken.add(line)
                out.append((name, line, decl, pid))
    return out


def unit_path(entry: tuple, input_parts: frozenset | set = frozenset()) -> tuple[str, str]:
    """(path, kind) by which a pending fact or declaration lies in a unit (check_facts_trace.in_unit): a fact's key,
    an `input.…` key of kind "input" unless it names a part or the values of the `input` widget (`input_parts`); a
    declaration's widget (S17: a state model's pending entry counts as a pending fact of its widget's unit)."""
    key = str(entry[2])
    if entry[4:5] == ("pending declaration",):
        return key.split(".", 1)[0], ""
    segs = key.split(".")
    if segs[0] == "input" and not (len(segs) > 1 and (segs[1] == "values" or segs[1] in input_parts)):
        return key, "input"
    return key, ""


class Predicate(NamedTuple):
    """The predicate of a pending fact's units beyond its key's (S17, ADD-A147; check_facts_trace.Depth): its
    `when` holds in a combination of the names at depth that its state model draws, its part drawn there, on a preset
    of its platform. `whens`: (file, line) -> (platform, `when`) of each pending fact."""
    depth: object
    whens: dict

    def holds(self, entry: tuple) -> bool:
        found = self.whens.get(entry[:2])
        if found is None:
            return True
        platform, when = found
        return any(self.depth.of_enabled(platform, str(entry[2]), when, preset)
                   for preset in check_facts_trace.check_facts.PLATFORMS.get(platform, ())
                   if check_facts_trace.for_preset(when, preset))


def enabled(entry: tuple, units: list | None, input_parts: frozenset | set = frozenset(),
            predicate: Predicate | None = None) -> bool:
    """Whether a pending fact or declaration is of the enabled units (`units`; None: every entry is): its key, or a
    declaration's widget, lies in one, and a pending fact is so by `predicate` where it is given."""
    if units is None:
        return True
    path, kind = unit_path(entry, input_parts)
    if kind != "input" and not any(check_facts_trace.in_unit(path, kind, u) for u in units):
        return False
    return predicate is None or entry[4:5] == ("pending declaration",) or predicate.holds(entry)


def outside_ids(pending: list, units: list | None, input_parts: frozenset | set = frozenset(),
                predicate: Predicate | None = None) -> set:
    """The register Ids of units not enabled: named by pending facts or declarations, every one not enabled."""
    named: dict[object, bool] = {}
    for entry in pending:
        named[entry[3]] = named.get(entry[3], False) or enabled(entry, units, input_parts, predicate)
    return {pid for pid, any_enabled in named.items() if isinstance(pid, str) and not any_enabled}


def pending_whens(texts: dict[str, str]) -> dict:
    """{(file, line): (platform, `when`)} of every pending fact pending_facts() gives, its platform the facts
    directory its file lies in; InputError where a file's `pending =` lines are not one per pending fact, so no
    fact's line is known."""
    out = {}
    for name, text in texts.items():
        data = tomllib.loads(text)
        facts = [f for f in data.get("fact", []) if isinstance(f, dict) and "pending" in f]
        lines = [n for n, line in enumerate(text.splitlines(), 1) if PENDING.match(line)]
        if len(lines) != len(facts):
            raise InputError(f"{name}: {len(lines)} `pending =` lines for {len(facts)} pending facts: no pending "
                             f"fact's line, and so its `when`, is known")
        platform = Path(name).parent.name
        for line, fact in zip(lines, facts):
            when = fact.get("when") if isinstance(fact.get("when"), dict) else {}
            out[(name, line)] = (platform, when)
    return out


def pending_problems(ledger: str, ledger_text: str, pending: list, final: bool, units: list | None = None,
                     input_parts: frozenset | set = frozenset(),
                     predicate: Predicate | None = None) -> list[tuple[str, int, str]]:
    """S17 *A pending measurement*: on every run, a pending fact whose id names no row of the register; with
    `final`, every pending fact of the enabled units (`units`, None for every entry; a release is made with none)."""
    rows, _ = register_rows(ledger, ledger_text)
    ids = {c[0] for _, c in rows or []}
    found = []
    for entry in pending:
        name, line, key, pid, *kind = entry
        what = kind[0] if kind else "pending fact"
        if not isinstance(pid, str) or pid not in ids:
            found.append((name, line, f"{what} {key!r} names {pid!r}, no row of the ledger's Unmeasured register"))
        if final and enabled(entry, units, input_parts, predicate):
            found.append((name, line, f"{what} {key!r} ({pid}): a release is made with no S17 {what} of its "
                                      f"enabled units"))
    return found


def input_parts_of(cat_text: str | None) -> frozenset:
    """The `input` widget's part names in the catalogue (none without the catalogue or the entry)."""
    if cat_text is None:
        return frozenset()
    entry = {e.name: e for e in spec_catalogue.entries(cat_text)}.get("input")
    return frozenset(n for p in entry.parts for n in p.names) if entry else frozenset()


def enabled_units() -> list:
    """scripts/widget_units.toml's enabled units (check_facts_trace.read_units)."""
    try:
        return list(check_facts_trace.read_units(Path(".")).values())
    except check_facts_trace.InputError as err:
        raise InputError(str(err)) from err


def depth_predicate(fact_texts: dict[str, str]) -> Predicate:
    """The Predicate over the repository's state models and presence facts (check_facts.load) and the units file."""
    cf = check_facts_trace.check_facts
    try:
        items, _, _ = cf.load(cf.Context(Path(".")))
        return Predicate(check_facts_trace.Depth(Path("."), items), pending_whens(fact_texts))
    except (check_facts_trace.InputError, cf.InputError) as err:
        raise InputError(str(err)) from err


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
- `button` on `ios-*`: one
- `button` on `macos-*`: two
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

| Id | Kind | Key and combination | Attempts and evidence | Options | Decision |
|---|---|---|---|---|---|
| U1 | d | `button.body.fill` {hover} | run 1 | real hardware | catalogue correction |
| U2 | d | `menu.body.shadow` | run 2 | another image | |

## Spikes and drift runs

## Triage

Prose with | a bar.

| Source | Heading | Item | Destination | Reason |
|---|---|---|---|---|
| docs/todo.md | ## Section one | “Orphan under the section” · l. 3 | M0 | |
| docs/todo.md | ### Alpha | “Research the thing and write” | M1a | |
| docs/todo.md | ### Alpha | “Research the thing and” | M1b | |
| docs/todo.md | ### Alpha | “Research the thing and fix” | M2.8 | |
| docs/todo.md | ### Alpha | “Indented item kept under” | after the plan's releases | |
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
| catalogue | ## Open items | “`button` on `ios-*`:” | lane ios | |
| catalogue | ## Open items | “`button`” | lane macos | |
| catalogue | ## Open items | “`button` on `macos-*`:” | lane macos | |

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
        ("catalogue", "## Open items", 5, "`context_help`: whether"),
        ("catalogue", "## Open items", 7, "`ink_toolbar` and"),
        ("catalogue", "## Open items", 8, "`button` on"),
        ("catalogue", "## Open items", 9, "`button` on"),
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
    if rows[9].reason != "The | model carries it." or len(rows) != 25:
        failures.append(f"rows: got {len(rows)}, the tenth {rows[9]}")
    got = problems("L", rows, items)
    todo_md, cat_md = str(SOURCES["docs/todo.md"]), str(SOURCES["catalogue"])
    want = [
        ("L", 19, "names 2 items of docs/todo.md (lines 5, 6): lengthen its opening words"),
        ("L", 20, "unknown destination 'M2.8'"),
        ("L", 21, "destination \"after the plan's releases\" without a reason"),
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
        ("L", 40, "names 2 items of catalogue (lines 8, 9): lengthen its opening words"),
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
    head = ("## Unmeasured register\n\n| Id | Kind | Key and combination | Attempts and evidence | Options | "
            "Decision |\n|---|---|---|---|---|---|\n")
    ids = (head + "| U1 | d | a | run 1 | none | kept |\n| U1 | d | b | run 2 | none | kept |\n"
           "| V3 | d | c | run 3 | none | kept |\n")
    if unmeasured_problems("L", ids) != [("L", 6, "unmeasured id U1 is used twice"),
                                         ("L", 7, "unmeasured id 'V3' is not U and a number")]:
        failures.append(f"unmeasured ids: got {unmeasured_problems('L', ids)}")
    if unmeasured_problems("L", ledger, final=False):
        failures.append(f"unmeasured, every run (the Decision is --final's): got "
                        f"{unmeasured_problems('L', ledger, False)}")
    if unmeasured_problems("L", ids, final=False) != [("L", 6, "unmeasured id U1 is used twice"),
                                                      ("L", 7, "unmeasured id 'V3' is not U and a number")]:
        failures.append(f"unmeasured ids, every run: got {unmeasured_problems('L', ids, False)}")
    if unmeasured_problems("L", "# Ledger\n", final=False) != [("L", 1, "no `## Unmeasured register` section")]:
        failures.append("unmeasured, every run: a ledger without the section passed")
    # the two kinds of row (the plan's *How values are checked*, ADD-A146): a Kind neither `b` nor `d` on every run;
    # with --final a `b` row is a measurement not made, a `d` row needs its Decision, a row of a unit not enabled
    # neither
    kinds = (head + "| U1 | b | a | run 1 | a probe pass | |\n| U2 | d | b | run 2 | a choice | kept |\n"
             "| U3 | d | c | run 3 | a choice | |\n| U4 | x | e | run 4 | none | kept |\n| U5 | | f | run 5 | none | |\n"
             "| U6 | b | g | run 6 | a source read | measured since |\n")
    bad_kinds = [("L", 8, "unmeasured U4: kind 'x' is neither b (a measurement to make) nor d (a decision)"),
                 ("L", 9, "unmeasured U5: kind '' is neither b (a measurement to make) nor d (a decision)")]
    if unmeasured_problems("L", kinds, final=False) != bad_kinds:
        failures.append(f"unmeasured kinds, every run: got {unmeasured_problems('L', kinds, False)}")
    want_kinds = [("L", 5, "unmeasured 'a' is a measurement not made"), ("L", 7, "unmeasured 'c' has no decision"),
                  *bad_kinds, ("L", 10, "unmeasured 'g' is a measurement not made")]
    if unmeasured_problems("L", kinds) != want_kinds:
        failures.append(f"unmeasured kinds, --final: got {unmeasured_problems('L', kinds)}")
    if unmeasured_problems("L", kinds, True, {"U1", "U3", "U6"}) != bad_kinds:
        failures.append(f"unmeasured kinds, --final, units not enabled: got "
                        f"{unmeasured_problems('L', kinds, True, {'U1', 'U3', 'U6'})}")
    if unmeasured_problems("L", ledger.replace("| U2 | d |", "| U2 | b |"), True, {"U1"}) != [
            ("L", 7, "unmeasured '`menu.body.shadow`' is a measurement not made")]:
        failures.append("unmeasured kinds, --final: a b row of the enabled units passed")
    if unmeasured_problems("L", ids.replace("| Id | Kind |", "| Id |")) != [
            ("L", 1, "the unmeasured register has no table")]:
        failures.append("unmeasured kinds: a table whose header lacks the Kind column was taken for the register's")
    # S17's pending facts against the register: every run, an id naming no row; --final, every pending fact
    pending_text = """[[fact]]
key = "button.body.fill"
value = "#ffffff"

[[fact]]
key = "button.body.fill"
when = { states = ["pressed"] }
pending = "U1"
rows = []

[[fact]]
key = "button.body.stroke"
  pending = "U9"
rows = []
"""
    pending = pending_facts({"f.toml": pending_text, "g.toml": "[[fact]]\nkey = 'x'\npending = 3\n"})
    if pending != [("f.toml", 8, "button.body.fill", "U1"), ("f.toml", 13, "button.body.stroke", "U9"),
                   ("g.toml", 3, "x", 3)]:
        failures.append(f"pending facts: got {pending}")
    got = pending_problems("L", ledger, pending, False)
    if got != [("f.toml", 13, "pending fact 'button.body.stroke' names 'U9', no row of the ledger's Unmeasured "
                              "register"),
               ("g.toml", 3, "pending fact 'x' names 3, no row of the ledger's Unmeasured register")]:
        failures.append(f"pending, every run: got {got}")
    got = pending_problems("L", ledger, pending[:1], True)
    if got != [("f.toml", 8, "pending fact 'button.body.fill' (U1): a release is made with no S17 pending fact of "
                             "its enabled units")]:
        failures.append(f"pending, --final: got {got}")
    if pending_problems("L", ledger, [], True) or pending_problems("L", ledger, pending[:1], False):
        failures.append("pending: no pending fact, or one naming a row, is a finding")
    # S10's pending default sizes (ADD-A131): read from the state models, each checked as a pending fact is
    models_text = """[[model]]
widget = "button"
default_size = { pending = "U1" }
sizes_by_variant = [{ variant = "@gray", sizes = ["mini", "small"], default_size = { pending = "U9" } },
                    { variant = "@fab", sizes = ["regular"], default_size = "regular" }]
sizes_by_context = [{ context = { widget = "toolbar" }, sizes = ["regular"], default_size = { pending = "U1" } }]

[[model]]
widget = "switch"
default_size = "regular"
"""
    decls = pending_declarations({"s.toml": models_text})
    if decls != [("s.toml", 3, "button.default_size", "U1"),
                 ("s.toml", 4, "button.sizes_by_variant.@gray.default_size", "U9"),
                 ("s.toml", 6, "button.sizes_by_context.0.default_size", "U1")]:
        failures.append(f"pending declarations: got {decls}")
    got = pending_problems("L", ledger, [(*d, "pending declaration") for d in decls], False)
    if got != [("s.toml", 4, "pending declaration 'button.sizes_by_variant.@gray.default_size' names 'U9', no row of "
                             "the ledger's Unmeasured register")]:
        failures.append(f"pending declarations, every run: got {got}")
    got = pending_problems("L", ledger, [(*decls[0], "pending declaration")], True)
    if got != [("s.toml", 3, "pending declaration 'button.default_size' (U1): a release is made with no S17 pending "
                             "declaration of its enabled units")]:
        failures.append(f"pending declarations, --final: got {got}")
    if pending_declarations({"s.toml": models_text.replace("{ pending = \"U1\" }", "\"small\"")}) != [
            ("s.toml", 4, "button.sizes_by_variant.@gray.default_size", "U9")]:
        failures.append("pending declarations: a stated default size is no pending declaration")
    # --final over the enabled units (S17, S21.3; ADD-A135): a pending fact by its key's unit, a state model's
    # pending entry by its widget's, an `input.…` S9 input never the `input` widget's but the model's core, of the
    # enabled units by its conditions (S17, ADD-A147); a register row of a unit not enabled where every pending fact
    # naming it is
    units = [check_facts_trace.Unit("slice", ("button", "input", "role", "token.motion"),
                                    ("token.motion.scroll",))]
    scoped = [("f.toml", 8, "button.body.fill", "U1"), ("f.toml", 9, "menu.body.shadow", "U2"),
              ("f.toml", 10, "input.accent", "U2"), ("f.toml", 11, "input.field.fill", "U3"),
              ("f.toml", 12, "token.motion.scroll.delay", "U4"), ("f.toml", 13, "checkbox.body.fill", "U5"),
              ("s.toml", 3, "switch.default_size", "U4", "pending declaration"),
              ("s.toml", 4, "button.sizes_by_variant.@gray.default_size", "U5", "pending declaration")]
    got = [(f, n) for f, n, _ in pending_problems("L", ledger, scoped, True, units, frozenset({"field"}))
           if "a release is made" in _]
    if got != [("f.toml", 8), ("f.toml", 10), ("f.toml", 11), ("s.toml", 4)]:
        failures.append(f"pending, --final over the enabled units: got {got}")
    if pending_problems("L", ledger, scoped[:1], True, []) or not pending_problems("L", ledger, scoped[:1], True):
        failures.append("pending, --final: no unit enabled, or every entry (units None), read wrong")
    got = outside_ids(scoped, units, frozenset({"field"}))
    if got != {"U4"}:
        failures.append(f"outside ids: got {sorted(got)}, want ['U4']")
    if outside_ids(scoped, None) or outside_ids([("f.toml", 1, "menu.body.fill", 7)], units):
        failures.append("outside ids: every entry enabled, or a malformed id, gave an id")
    # the predicate of a fact's units (S17, ADD-A147): a pending fact whose `when` a later unit's name holds in is that
    # unit's, its row too where every pending fact naming it is; a pending declaration keeps its widget's unit

    class Later:
        def of_enabled(self, platform, key, when, preset):
            return "flat" not in when.get("variants", ())

    predicate = Predicate(Later(), {("docs/facts/gnome/f.toml", 8): ("gnome", {"variants": ["flat"]}),
                                    ("docs/facts/gnome/f.toml", 9): ("gnome", {})})
    deep = [("docs/facts/gnome/f.toml", 8, "button.body.fill", "U1"),
            ("docs/facts/gnome/f.toml", 9, "button.body.fill", "U2"),
            ("s.toml", 4, "button.default_size", "U5", "pending declaration")]
    got = [(f, n) for f, n, _ in pending_problems("L", ledger, deep, True, units, frozenset(), predicate)
           if "a release is made" in _]
    if got != [("docs/facts/gnome/f.toml", 9), ("s.toml", 4)] or outside_ids(deep, units, frozenset(),
                                                                             predicate) != {"U1"}:
        failures.append(f"pending, --final, the depth's predicate: got {got}")
    if pending_whens({"docs/facts/kde/b.toml": '[[fact]]\nkey = "button.body.fill"\nwhen = { states = ["hover"] }\n'
                      'pending = "U1"\nrows = []\n'}) != {("docs/facts/kde/b.toml", 4): ("kde", {"states": ["hover"]})}:
        failures.append("pending_whens: a pending fact's platform and when read wrong")
    try:
        pending_whens({"docs/facts/kde/b.toml": '[[fact]]\nkey = "button.body.fill"\npending = "U1"\n'
                                                '[note]\npending = "a key of another table"\n'})
        failures.append("pending_whens: `pending =` lines not one per pending fact, not refused")
    except InputError as err:
        if "2 `pending =` lines for 1 pending facts" not in str(err):
            failures.append(f"pending_whens: refused with {err}")
    if unmeasured_problems("L", ledger, True, {"U2"}) or unmeasured_problems("L", ledger, True, {"U1"}) != [
            ("L", 7, "unmeasured '`menu.body.shadow`' has no decision")]:
        failures.append("unmeasured, --final: a row of a unit not enabled failed, or one of the enabled units passed")
    try:
        parts = input_parts_of(cat)
    except Exception as err:   # noqa: BLE001 - a self-test reports, it does not stop
        parts = err
    if parts != frozenset() or input_parts_of(None) != frozenset():
        failures.append(f"input parts: a catalogue without an `input` entry gave {parts}")
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
        (lambda: pending_facts({"f.toml": "[[fact]\n"}), "bad TOML, pending"),
        (lambda: pending_declarations({"s.toml": "[[model]\n"}), "bad TOML, pending declaration"),
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
        sources = read_sources()
        items = extract(sources)
        found = problems(str(ledger), rows, items)
        fact_texts = platform_fact_texts(FACTS)
        pending = pending_facts(fact_texts)
        declarations = pending_declarations(fact_texts)
        units = enabled_units() if final else None
        parts = input_parts_of(sources.get("catalogue")) if final else frozenset()
        predicate = depth_predicate(fact_texts) if final else None
        every = pending + [(*d, "pending declaration") for d in declarations]
        found += unmeasured_problems(str(ledger), ledger_text, final, outside_ids(every, units, parts, predicate))
        found += pending_problems(str(ledger), ledger_text, pending, final, units, parts, predicate)
        found += pending_problems(str(ledger), ledger_text, [(*d, "pending declaration") for d in declarations],
                                  final, units, parts, predicate)
        if final:
            found += check_final(str(ledger), ledger_text, staged)
    except (OSError, UnicodeDecodeError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for name, line, what in found:
        print(f"{name}:{line}: {what}")
    per_source = ", ".join(f"{s} {sum(1 for i in items if i.source == s)}" for s in SOURCES)
    print(f"{len(rows)} triage rows, {len(items)} items ({per_source}), {len(pending)} pending facts, "
          f"{len(declarations)} pending declarations"
          f"{', --final' if final else ''}: {len(found)} findings", file=sys.stderr)
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
