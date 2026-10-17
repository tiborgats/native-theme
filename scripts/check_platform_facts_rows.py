#!/usr/bin/env python3
"""List every registry field that has no row in docs/platform-facts.md.

docs/property-registry.toml names each theme field; docs/platform-facts.md
Chapter 2 gives each field a row whose cells say, per platform, where its
value comes from. A field without a row has no source a preset can cite, so
this script lists them.

Run from the repository root:

    python3 scripts/check_platform_facts_rows.py
    python3 scripts/check_platform_facts_rows.py --self-test

It prints one line per field without a row, tab-separated: the registry
field, its section, and the row name it was looked for under. Exit 0 when
the list is empty, 1 when it is not, 2 on a malformed input.

Matching rule
-------------
Sections. A registry table belongs to the section of the last `# §2.x`
heading comment (a comment line that starts with `# §2.`) above its
`[table]` header with no field line between the two; a table without one is
an input error. `[defaults]` is the exception: the registry's `# §2.1.x` comments
inside it do not split its fields the way the facts file splits its tables
(`defaults.border.color`'s row is in §2.1.3, under the registry's §2.1.6), so
a `defaults` field is looked for in all of §2.1.

Rows. A facts section is a `### 2.x` or `#### 2.x.y` heading; its rows are
the backticked names in the first column of every table under it, up to the
next heading. `### 2.1` and its `#### 2.1.x` sections form §2.1.

Field names. A scalar field matches the row named after it with its unit
suffix (`_px`, `_ms`, `_pt`) removed. A sub-struct field is expanded into
its parts, each matching `<field>.<part>`:

- `border`: `border.color`, `border.line_width`, ... The four per-side
  paddings match their axis rows, as the facts file's conventions point 2
  states: `padding_top` and `padding_bottom` match `border.padding_vertical`,
  `padding_left` and `padding_right` match `border.padding_horizontal`; an
  axis shorthand key (`padding_vertical`) matches its own row.
  `corner_radius_lg` and `opacity` are defaults-only ("defaults only" in
  `_structures.Border`) and are skipped on widgets; the padding keys are
  "widget borders only" and are skipped on `defaults`.
- `font` and every `*_font`: `<field>.family`, `<field>.size` (from `size_pt`
  and `size_px`), `<field>.weight`, `<field>.style`, `<field>.color`. A row
  named after the whole font (`font`, `item_font`) counts for all its parts:
  the facts file gives a font one row when no platform differs part by part.
  `defaults.font` and `defaults.mono_font` are the exception: §2.1.1 and
  §2.1.2 name their parts bare (`family`, `size`, ...), so they are looked
  for bare, in their own subsection.
- `icon_sizes`: its parts, bare, in §2.1.8 (`toolbar`, `small`, ...).
- A `text_scale_entry` field matches the row named after the field (§2.19's
  rows are the roles: `caption`, `section_heading`, ...).

`[theme_variant]` is skipped: its fields are structural (`icon_set` is a
field of `Theme`, not of a theme mode), not a §2 table.
"""

from __future__ import annotations

import re
import sys
import tempfile
from pathlib import Path

REGISTRY = Path("docs/property-registry.toml")
FACTS = Path("docs/platform-facts.md")

SKIPPED_TABLES = {"theme_variant"}
DEFAULTS_ONLY_BORDER = {"corner_radius_lg", "opacity"}
SIDE_TO_AXIS = {
    "padding_top": "padding_vertical",
    "padding_bottom": "padding_vertical",
    "padding_left": "padding_horizontal",
    "padding_right": "padding_horizontal",
}
BARE_PART_SECTIONS = {
    ("defaults", "font"): "2.1.1",
    ("defaults", "mono_font"): "2.1.2",
    ("defaults", "icon_sizes"): "2.1.8",
}

SECTION_COMMENT = re.compile(r"#\s*§(2\.\d+(?:\.\d+)?)\b")
TABLE_HEADER = re.compile(r"\[([A-Za-z0-9_.]+)\]")
FIELD_LINE = re.compile(r'([a-z_0-9]+)\s*=\s*"([a-z_0-9]+)"')
FACTS_HEADING = re.compile(r"#{3,4}\s+(2\.\d+(?:\.\d+)?)\s")
OTHER_HEADING = re.compile(r"#{1,4}\s")
BACKTICKED = re.compile(r"`([^`]+)`")


class InputError(Exception):
    """A registry or facts file the rule cannot read."""


def strip_unit(key: str) -> str:
    return re.sub(r"_(px|ms|pt)$", "", key)


def parse_registry(text: str):
    """Return (structures, fields): structures maps a struct name to its keys,
    fields lists (table, key, type, section) for every non-structure table."""
    structures: dict[str, list[str]] = {}
    fields: list[tuple[str, str, str, str | None]] = []
    section: str | None = None
    table: str | None = None
    table_section: str | None = None
    for line in text.splitlines():
        stripped = line.strip()
        if stripped.startswith("#"):
            m = SECTION_COMMENT.match(stripped)
            if m and stripped.startswith("# §2."):
                section = m.group(1)
            continue
        m = TABLE_HEADER.fullmatch(stripped)
        if m:
            table = m.group(1)
            table_section = section
            section = None
            if table.startswith("_structures."):
                structures[table.split(".", 1)[1]] = []
            continue
        m = FIELD_LINE.match(stripped)
        if m and table is not None:
            section = None
            key, typ = m.groups()
            if table.startswith("_structures."):
                structures[table.split(".", 1)[1]].append(key)
            else:
                fields.append((table, key, typ, table_section))
    for needed in ("Border", "Font", "IconSizes"):
        if needed not in structures:
            raise InputError(f"registry has no [_structures.{needed}]")
    if not fields:
        raise InputError("registry has no widget tables")
    return structures, fields


def parse_facts(text: str) -> dict[str, set[str]]:
    """Map each §2 section ('2.3', '2.1.3') to the first-column names of its tables."""
    rows: dict[str, set[str]] = {}
    current: str | None = None
    for line in text.splitlines():
        m = FACTS_HEADING.match(line)
        if m:
            current = m.group(1)
            rows.setdefault(current, set())
            continue
        if OTHER_HEADING.match(line):
            if not line.startswith("####"):
                current = None
            continue
        if current and line.startswith("|"):
            cells = line.split("|")
            if len(cells) < 3:
                continue
            first = cells[1].strip()
            if not first or set(first) <= {"-", ":", " "}:
                continue
            rows[current].update(BACKTICKED.findall(first))
    if not rows:
        raise InputError("facts file has no §2 sections")
    return rows


def section_rows(rows: dict[str, set[str]], section: str) -> set[str]:
    if section == "2.1" or section.startswith("2.1."):
        out: set[str] = set()
        for key, names in rows.items():
            if key == "2.1" or key.startswith("2.1."):
                out |= names
        return out
    return rows.get(section, set())


def expected_rows(table: str, key: str, typ: str, structures) -> list[tuple[str, str, tuple[str, ...]]]:
    """(registry field, row name, alternative row names that also satisfy it)."""
    out = []
    if typ == "border":
        for part in structures["Border"]:
            leaf = strip_unit(part)
            if table != "defaults" and leaf in DEFAULTS_ONLY_BORDER:
                continue
            if table == "defaults" and leaf.startswith("padding_"):
                continue
            row = f"{key}.{SIDE_TO_AXIS.get(leaf, leaf)}"
            out.append((f"{table}.{key}.{leaf}", row, ()))
    elif typ == "font":
        bare = (table, key) in BARE_PART_SECTIONS
        seen = set()
        for part in structures["Font"]:
            leaf = strip_unit(part)
            if leaf in seen:
                continue
            seen.add(leaf)
            row = leaf if bare else f"{key}.{leaf}"
            out.append((f"{table}.{key}.{leaf}", row, () if bare else (key,)))
    elif typ == "icon_sizes":
        for part in structures["IconSizes"]:
            leaf = strip_unit(part)
            out.append((f"{table}.{key}.{leaf}", leaf, ()))
    else:
        out.append((f"{table}.{key}", strip_unit(key), ()))
    return out


def missing_rows(registry_text: str, facts_text: str) -> list[tuple[str, str, str]]:
    structures, fields = parse_registry(registry_text)
    rows = parse_facts(facts_text)
    missing = []
    for table, key, typ, section in fields:
        if table in SKIPPED_TABLES:
            continue
        if section is None:
            raise InputError(f"[{table}] has no `# §2.x` comment above it")
        look_in = BARE_PART_SECTIONS.get((table, key), "2.1" if table == "defaults" else section)
        if (table, key) in BARE_PART_SECTIONS:
            have = rows.get(look_in, set())
        else:
            have = section_rows(rows, look_in)
        for field, row, alternatives in expected_rows(table, key, typ, structures):
            if row in have or any(alt in have for alt in alternatives):
                continue
            missing.append((field, look_in, row))
    return missing


def self_test() -> int:
    registry = """
[_structures.Font]
family = "string"
size_pt = "f32"
size_px = "f32"
color = "color"
[_structures.Border]
color = "color"
corner_radius_lg_px = "f32"    # defaults only
padding_top_px = "f32"
padding_horizontal_px = "f32"
[_structures.IconSizes]
toolbar_px = "f32"
# §2.1 Global Defaults
[defaults]
# §2.1.1 Base Font
font = "font"
# §2.1.6 Global Geometry
border = "border"
# §2.1.8 Icon Sizes
icon_sizes = "icon_sizes"
# §2.3 Button
[button]
hover_background = "color"
font = "font"
border = "border"
# §2.4 Text Input
[input]
item_font = "font"
[theme_variant]
icon_set = "enum"
"""
    facts = """
### 2.1 Global Defaults
#### 2.1.1 Base Font
| Property | macOS |
|---|---|
| `family` | x |
| `size` | x |
| `color` | x |
#### 2.1.3 Base Colors
| Property | macOS |
|---|---|
| `border.color` | x |
#### 2.1.8 Icon Sizes
| Property | macOS |
|---|---|
| `toolbar` | x |
### 2.3 Button
| Property | macOS |
|---|---|
| `font` | x |
| `border.color` | x |
| `border.padding_vertical` | x |
### 2.4 Text Input
| Property | macOS |
|---|---|
| `item_font.family` | x |
## Appendix
| `hover_background` | not a §2 row |
"""
    got = missing_rows(registry, facts)
    want = [
        ("defaults.border.corner_radius_lg", "2.1", "border.corner_radius_lg"),
        ("button.hover_background", "2.3", "hover_background"),
        ("button.border.padding_horizontal", "2.3", "border.padding_horizontal"),
        ("input.item_font.size", "2.4", "item_font.size"),
        ("input.item_font.color", "2.4", "item_font.color"),
    ]
    failures = []
    if got != want:
        failures.append(f"rule: got {got}, want {want}")
    for bad_registry, label in ((registry.replace("[_structures.Border]", "[_structures.Edge]"), "no Border"),
                                (registry.replace("# §2.3 Button", "# Button"), "no § comment")):
        try:
            missing_rows(bad_registry, facts)
            failures.append(f"{label}: accepted")
        except InputError:
            pass
    with tempfile.TemporaryDirectory() as tmp:
        empty = Path(tmp) / "facts.md"
        empty.write_text("# nothing\n", encoding="utf-8")
        try:
            missing_rows(registry, empty.read_text(encoding="utf-8"))
            failures.append("empty facts: accepted")
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
        missing = missing_rows(REGISTRY.read_text(encoding="utf-8"), FACTS.read_text(encoding="utf-8"))
    except (OSError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for field, section, row in missing:
        print(f"{field}\t§{section}\t{row}")
    print(f"{len(missing)} registry fields without a platform-facts row", file=sys.stderr)
    return 1 if missing else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
