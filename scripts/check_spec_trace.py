#!/usr/bin/env python3
"""List every v0.7.1 gap-map row the specification's traceability does not place.

The gap map (docs/todo_v0.7.1_model-completeness-gap-map.md and its six
platform files) lists what the platforms draw that the model cannot express;
the specification's traceability (docs/todo_v0.7.1_model-completeness-spec-
trace-<platform>.md) gives each of those rows its target in the new model, or
its verdict when the row is out of scope (the rationale's R24). This script
checks that every row has one.

Run from the repository root:

    python3 scripts/check_spec_trace.py
    python3 scripts/check_spec_trace.py --self-test

It prints one line per problem, tab-separated: the platform, the row ID, and
what is wrong. Exit 0 when there is none, 1 when there is, 2 on a malformed
input.

Matching rule
-------------
Rows. The gap map's rows are the IDs of its Appendix A (*Rows per
capability*), one `- <Platform>: ID, ID, ...` line per platform and
capability; every row has at least one capability, so the union is every row.
A row is withdrawn when a table line of its platform file starts with the
row's ID cell and contains `Withdrawn (review`; withdrawn rows need no target.

Traceability. Each platform's trace file holds a table `| Row | Target |
Kind |`; a row needing several targets has several lines. A line's Kind is a
target kind or a verdict (KINDS, VERDICTS below, the specification's S20). A
target kind needs a target; a verdict's target is `-`, or `- (note)` naming
the part of the row it covers, except that `readers (R18)` may name the read
the later reader makes.

Problems: a row without a line; a line for a row the map does not have, or
for a withdrawn row; an unknown kind; a target kind without a target, or a
verdict other than `readers (R18)` with one.
"""
import re
import sys
from pathlib import Path

DOCS = Path("docs")
MAP = DOCS / "todo_v0.7.1_model-completeness-gap-map.md"
PLATFORMS = {
    "macOS": "macos",
    "Windows": "windows",
    "KDE": "kde",
    "GNOME": "gnome",
    "Material": "material",
    "iOS": "ios",
}
KINDS = {
    "field", "rule", "role", "token", "component", "input", "reader", "placement",
    "system-control", "condition", "flag",
}
VERDICTS = {
    "out (R1.1)", "out (R1.2)", "out (R1.4)", "preset (R1.3)", "newest (R1.5)", "reference (R1.6)", "readers (R18)",
}


class InputError(Exception):
    pass


def cells(line: str) -> list[str]:
    """A table line's cells; `\\|` is a literal bar inside a cell, as in Markdown tables."""
    return [c.strip().replace("\\|", "|") for c in re.split(r"(?<!\\)\|", line.strip().strip("|"))]


def map_rows(map_text: str) -> dict[str, set[str]]:
    start = map_text.find("## Appendix A")
    end = map_text.find("## Appendix B", start)
    if start < 0 or end < 0:
        raise InputError("the gap map has no Appendix A")
    rows = {name: set() for name in PLATFORMS}
    for m in re.finditer(r"^- (\w+): (.+)$", map_text[start:end], re.M):
        if m.group(1) not in rows:
            raise InputError(f"Appendix A: unknown platform {m.group(1)}")
        rows[m.group(1)].update(x.strip() for x in m.group(2).split(","))
    for name, ids in rows.items():
        if not ids:
            raise InputError(f"Appendix A: no rows for {name}")
    return rows


def withdrawn(platform_text: str, ids: set[str]) -> set[str]:
    out = set()
    for line in platform_text.splitlines():
        if line.startswith("| ") and "Withdrawn (review" in line:
            first = cells(line)[0]
            if first in ids:
                out.add(first)
    return out


def trace_lines(trace_text: str) -> list[tuple[str, str, str]]:
    lines = trace_text.splitlines()
    header = next((i for i, l in enumerate(lines) if cells(l)[:3] == ["Row", "Target", "Kind"]), None)
    if header is None:
        raise InputError("no `| Row | Target | Kind |` table")
    out = []
    for line in lines[header + 2:]:
        if not line.startswith("|"):
            break
        c = cells(line)
        if len(c) != 3:
            raise InputError(f"not three cells: {line}")
        out.append((c[0], c[1].strip("`"), c[2]))
    return out


def problems(rows: set[str], gone: set[str], trace: list[tuple[str, str, str]]) -> list[tuple[str, str]]:
    found = []
    traced = set()
    for row, target, kind in trace:
        traced.add(row)
        if row not in rows:
            found.append((row, "not a row of the gap map"))
        elif row in gone:
            found.append((row, "withdrawn, needs no target"))
        if kind in KINDS:
            if target in ("", "-"):
                found.append((row, f"kind {kind} without a target"))
        elif kind in VERDICTS:
            if kind != "readers (R18)" and not (target == "-" or target.startswith("- (")):
                found.append((row, f"verdict {kind} with a target"))
        else:
            found.append((row, f"unknown kind {kind!r}"))
    for row in sorted(rows - gone - traced):
        found.append((row, "no target"))
    return found


def check(docs: Path) -> list[tuple[str, str, str]]:
    rows = map_rows((docs / MAP.name).read_text(encoding="utf-8"))
    out = []
    for name, slug in PLATFORMS.items():
        platform = (docs / f"todo_v0.7.1_model-completeness-gap-map-{slug}.md").read_text(encoding="utf-8")
        trace_path = docs / f"todo_v0.7.1_model-completeness-spec-trace-{slug}.md"
        try:
            trace = trace_lines(trace_path.read_text(encoding="utf-8"))
        except InputError as err:
            raise InputError(f"{trace_path}: {err}") from err
        gone = withdrawn(platform, rows[name])
        out += [(name, row, what) for row, what in problems(rows[name], gone, trace)]
    return out


def self_test() -> int:
    map_text = """
## Appendix A. Rows per capability
#### C01 — Shadow geometry
- macOS: MAC-1, MAC-2
- Windows: W1-001
- KDE: K1.1
- GNOME: GN-1.01
- Material: MAT-B1
- iOS: B-1
#### C02 — Elevation
- macOS: MAC-2, MAC-3†
## Appendix B
- macOS: MAC-9
"""
    platform = """
| MAC-1 | window | shadow | src | none |
| MAC-3† | menu | Withdrawn (review 2026-10-28): duplicate of MAC-2 | src | none |
| MAC-2, MAC-3† | 2 | Withdrawn (review in a cross-check line, not a row |
"""
    trace = """
| Row | Target | Kind |
|---|---|---|
| MAC-1 | `window.body.shadow` | field |
| MAC-2 | `menu.body.fill @ {a\\|b}` | rule |
| MAC-3† | `menu.body.shadow` | field |
| MAC-9 | - | out (R1.1) |
| MAC-2 | - (its panel) | out (R1.1) |
| MAC-2 | `reader:macos:x` | readers (R18) |
| MAC-1 | - | rule |
| MAC-1 | `menu` | out (R1.1) |
| MAC-1 | `x` | guess |
"""
    failures = []
    rows = map_rows(map_text)
    if rows["macOS"] != {"MAC-1", "MAC-2", "MAC-3†"} or rows["KDE"] != {"K1.1"}:
        failures.append(f"rows: got {rows}")
    gone = withdrawn(platform, rows["macOS"])
    if gone != {"MAC-3†"}:
        failures.append(f"withdrawn: got {gone}")
    got = problems(rows["macOS"], gone, trace_lines(trace))
    want = [
        ("MAC-3†", "withdrawn, needs no target"),
        ("MAC-9", "not a row of the gap map"),
        ("MAC-1", "kind rule without a target"),
        ("MAC-1", "verdict out (R1.1) with a target"),
        ("MAC-1", "unknown kind 'guess'"),
    ]
    if got != want:
        failures.append(f"problems: got {got}, want {want}")
    for bad, label in ((map_text.replace("## Appendix A", "## Appendix"), "no Appendix A"),
                       (map_text.replace("- KDE: K1.1", "- Haiku: K1.1"), "unknown platform")):
        try:
            map_rows(bad)
            failures.append(f"{label}: accepted")
        except InputError:
            pass
    for bad, label in ((trace.replace("| Row |", "| ID |"), "no header"),
                       (trace.replace("| field |\n", "|\n", 1), "two cells")):
        try:
            trace_lines(bad)
            failures.append(f"{label}: accepted")
        except InputError:
            pass
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def main(argv: list[str]) -> int:
    if argv[1:] == ["--self-test"]:
        return self_test()
    if argv[1:]:
        print(__doc__, file=sys.stderr)
        return 2
    try:
        found = check(DOCS)
    except (OSError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for platform, row, what in found:
        print(f"{platform}\t{row}\t{what}")
    print(f"{len(found)} traceability problems", file=sys.stderr)
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
