#!/usr/bin/env python3
"""List every preset comment whose platform-facts row is not the value it cites.

A preset value's comment cites the docs/platform-facts.md row it comes from
by line number: `# platform-facts.md:1192 (§2.3): ...` above
`hover_background = ...`. A line number drifts with every edit above it, and
the value stays right while its citation names a neighbouring row. This
script lists each citation whose row does not name the value, or the key,
it cites.

Run from the repository root:

    python3 scripts/check_preset_citations.py
    python3 scripts/check_preset_citations.py --self-test

It prints one line per finding, tab-separated: the preset and the comment's
line, the key path, the cited line, and what is wrong; then, on stderr, the
counts of every preset. Exit 0 when there is none, 1 when there are some, 2
on an input it cannot read: a file that is not UTF-8 or TOML, or a preset
that cites `platform-facts.md:N` where the rule reads no citation.

Rule
----
Presets: each native-theme/src/presets/*.toml.

Citations. Every `platform-facts.md:` or `docs/platform-facts.md:` in a
comment, followed by a list of lines, `N`, `N-M` or further `, :K`, each
optionally followed by its sections, ` (§2.3)` or ` (§2.1.3, §2.1.6)`. A
section states where its own lines are and those before it that state none.

Subjects. A citation cites one of three things:

- the keys its comment line names, by one of the forms `no K / K: ...`,
  `K not stated: ...`, `K no source ...`, `K inherits X ...` and
  `K ← X; K ← X: ...` (a `K` relative to the widget, or `defaults.K`, `*` a
  wildcard; `no source` is a mark, not a key). A line with no form of its
  own, inside a comment whose first line has one and cites nothing, takes
  that line's keys (`# hover_text_color ... inherit font.color (linkColor,`
  then `# platform-facts.md:1694) ...`). A row of `X` holds too: the key
  takes its value;
- else the value straight below its comment, when it is the line's first
  citation and the line starts with it (after a `❓`, `no source:`,
  `soft_option,` or `no macOS 26 source (` mark), or the comment's first
  citation wherever it sits (`# role (docs/platform-facts.md:1056)`);
- else nothing: a citation mid-sentence after another, or above a blank
  line or a table header, whose rows are checked as rows, not compared.

Rows. A facts line is a row when its first cell is one backticked name
(`` | `border.color` | ... ``). A cited line in a Chapter 2 section must be
a row, in the section stated for it or a subsection of it. A cited line
outside the Chapter 2 sections names a platform's own key (kdeglobals
`font`) or a convention, not a property, and is not compared.

Values. The values a value citation covers run from its value to the next
comment that cites a row, through the tables of the same widget.

Names. A value `key` in table `[light.<widget>.<sub>]` is named
`<sub>.<key>` (`[light.button.border] color` is `border.color`), its unit
suffix (`_px`, `_ms`, `_pt`) removed and a dotted key walked into; the
`[layout]` table's keys are named bare. As check_platform_facts_rows.py
reads the facts file, a value also matches:

- in `defaults.font`, `defaults.mono_font` and `defaults.icon_sizes`, the
  bare part (`size`, `dialog`): §2.1.1, §2.1.2 and §2.1.8 name them so;
- in a text_scale entry, the row named after the entry (`caption`);
- in a font, the row named after the whole font (`font`, `body_font`), where
  the section has no row for the part: the facts file gives a font one row
  when no platform differs part by part;
- a per-side padding, its axis row (`padding_left` matches
  `border.padding_horizontal`).

Cross-row citations, each true of the value by what it states, and each
only where the cited row's section has no row of the value's own:

- a `disabled_*` value cites `disabled_opacity`: the platform has no disabled
  colour, so the preset's is the enabled one at that opacity;
- a state's fill or line cites the widget's own row for it (an
  `unchecked_background` or `hover_background` cites `background_color`, an
  `unchecked_border_color` cites `border.color`): the platform draws that
  state with the widget's own fill or line;
- a row of a citation with several rows names a key its widget leaves out:
  the comment says why it is left out (`no outline` cites `border.color`
  beside `line_width = 0`); a row in another section than the value's is
  another widget's.

A citation holds when its value, or a key its comment names, matches one of
its rows and, where it cites several rows for a value, each row of the
value's section matches a value it covers. EXCEPTIONS lists the rest by
preset, key path and cited row name, each with the reason it holds; an
exception no citation uses is a finding.
"""

from __future__ import annotations

import re
import sys
import tempfile
import tomllib
from collections import Counter
from pathlib import Path

PRESETS = Path("native-theme/src/presets")
FACTS = Path("docs/platform-facts.md")

SEVERITY_TEXT_RANGE = (
    "the range spans the four severity text colours, between which the fills' rows lie; the "
    "dark variant's `info_color` and `info_text_color` follow `info_color`'s own citation"
)
EXCEPTIONS: dict[tuple[str, str, str], str] = {
    ("windows-11.toml", "dark.defaults.danger_text_color", "info_color"): SEVERITY_TEXT_RANGE,
    ("windows-11.toml", "dark.defaults.danger_text_color", "info_text_color"): SEVERITY_TEXT_RANGE,
}

PIECE = r"\d+(?:-\d+)?(?: \(§\d+(?:\.\d+)*(?:, §\d+(?:\.\d+)*)*\))?"
CITATION = re.compile(
    rf"(?<![\w.-])(?:docs/)?platform-facts\.md:({PIECE}(?:,\s*:{PIECE})*)"
)
KEY = r"[a-z_][a-z0-9_.*]*"
KEYS = rf"{KEY}(?:(?:\s*/\s*|,\s*|,? and ){KEY})*"
NAMED = (
    re.compile(rf"^no ({KEYS})\s*:"),
    re.compile(rf"^({KEYS}) not stated\b"),
    re.compile(rf"^({KEYS}) no source\b"),
    re.compile(rf"^({KEYS}) inherits? ({KEY})"),
)
ARROW = re.compile(rf"(?:^|;\s*)({KEY}) ← ({KEY})")
MARKER = re.compile(r"^(?:soft_option,?\s*)?(?:no (?:macOS 26 )?source(?: ❓)?\s*[:(]?\s*)?")
PART = re.compile(r"(\d+)(?:-(\d+))?(?: \(([^)]*)\))?")
HEADER = re.compile(r"\[([A-Za-z0-9_.-]+)\]")
VALUE = re.compile(r"([A-Za-z0-9_.]+)\s*=")
ROW = re.compile(r"\|\s*`([a-z0-9_.]+)`\s*\|")
CHAPTER = re.compile(r"##\s+Chapter\s+(\d+)\b")
OTHER_H2 = re.compile(r"##\s")
SECTION = re.compile(r"#{3,4}\s+(\d+\.\d+(?:\.\d+)?)\s")

BARE_PARTS = {"font", "mono_font", "icon_sizes"}
SIDE_TO_AXIS = {
    "padding_top": "padding_vertical",
    "padding_bottom": "padding_vertical",
    "padding_left": "padding_horizontal",
    "padding_right": "padding_horizontal",
}
STATES = ("unchecked", "hover")


class InputError(Exception):
    """A preset or facts file the rule cannot read."""


class Facts:
    """The facts file's Chapter 2 lines, and its rows with their sections."""

    def __init__(self, text: str):
        self.rows: dict[int, tuple[str, str | None]] = {}
        self.section_rows: dict[str | None, set[str]] = {}
        self.chapter2: set[int] = set()
        chapter: str | None = None
        section: str | None = None
        for n, line in enumerate(text.splitlines(), 1):
            m = CHAPTER.match(line)
            if m:
                chapter, section = m.group(1), None
            elif OTHER_H2.match(line):
                chapter, section = None, None
            m = SECTION.match(line)
            if m:
                section = m.group(1)
            if chapter != "2" or section is None:
                continue
            self.chapter2.add(n)
            m = ROW.match(line)
            if m:
                self.rows[n] = (m.group(1), section)
                self.section_rows.setdefault(section, set()).add(m.group(1))
        if not self.rows:
            raise InputError(f"{FACTS}: no Chapter 2 rows")


def strip_unit(key: str) -> str:
    return re.sub(r"_(px|ms|pt)$", "", key)


def widget_of(table: list[str]) -> list[str]:
    return table[:2] if table[:1] in (["light"], ["dark"]) else table[:1]


def scope(table: list[str]) -> tuple[str, list[str]]:
    """A table's widget and the sub-table path inside it; `[layout]` has neither."""
    scoped = table[1:] if table[:1] in (["light"], ["dark"]) else table
    if scoped and scoped[0] != "layout":
        return scoped[0], scoped[1:]
    return "", []


def names(table: list[str], key: str, section_rows: set[str]) -> set[str]:
    """The row names a value `key` in `table` matches, cross-row rules included;
    `section_rows` names the rows of the cited row's section."""
    widget, sub = scope(table)
    path = sub + [strip_unit(part) for part in key.split(".")]
    out = {".".join(path)}
    if widget == "defaults" and sub and sub[0] in BARE_PARTS:
        out.add(".".join(path[1:]))
    if widget == "text_scale" and sub:
        out.add(sub[0])
    if len(path) >= 2 and path[-2].endswith("font") and ".".join(path) not in section_rows:
        out.add(".".join(path[:-1]))
    if path[-1] in SIDE_TO_AXIS:
        out.add(".".join(path[:-1] + [SIDE_TO_AXIS[path[-1]]]))
    last = path[-1]
    if ".".join(path) in section_rows:
        return out
    if last.startswith("disabled_"):
        out.add("disabled_opacity")
    for state in STATES:
        if last == f"{state}_background":
            out.add(".".join(path[:-1] + ["background_color"]))
        if last == f"{state}_border_color":
            out.add(".".join(path[:-1] + ["border.color"]))
    return out


def present(preset: dict, table: list[str]) -> set[str]:
    """The row names of every key the table's widget sets, cross-row rules left out."""
    node: object = preset
    for part in widget_of(table):
        node = node.get(part, {}) if isinstance(node, dict) else {}
    out: set[str] = set()

    def walk(node: object, path: list[str]) -> None:
        if not isinstance(node, dict):
            parts = [strip_unit(part) for part in path]
            for n in range(1, len(parts) + 1):
                out.add(".".join(parts[:n]))
            if parts and parts[-1] in SIDE_TO_AXIS:
                out.add(".".join(parts[:-1] + [SIDE_TO_AXIS[parts[-1]]]))
            return
        for key, value in node.items():
            walk(value, path + [key])

    walk(node, [])
    return out


def cited_lines(m: re.Match) -> list[tuple[int, list[str]]]:
    """Each cited line with the sections stated for it."""
    out: list[tuple[int, list[str]]] = []
    pending: list[int] = []
    for part in PART.finditer(m.group(1)):
        lo = int(part.group(1))
        pending += range(lo, int(part.group(2) or lo) + 1)
        if part.group(3):
            stated = [s.removeprefix("§") for s in part.group(3).split(", ")]
            out += ((n, stated) for n in pending)
            pending = []
    out += ((n, []) for n in pending)
    return out


def comment_text(line: str) -> str:
    return re.sub(r"^#\s*(?:❓\s*)?", "", line.strip())


def named(text: str) -> tuple[list[str], list[str]] | None:
    """The keys a comment names as its subject, and the keys it says they take
    their value from: `no K / K:`, `K not stated`, `K no source`,
    `K inherits X`, `K ← X`."""
    split = lambda keys: [k for k in re.split(r"\s*/\s*|,\s*|,? and ", keys) if k]
    for form in NAMED:
        m = form.match(text)
        if m and "source" not in split(m.group(1)):
            return split(m.group(1)), [m.group(2)] if form.groups == 2 else []
    arrows = ARROW.findall(text)
    if arrows:
        return [k for k, _ in arrows], [x for _, x in arrows]
    return None


def covered(lines: list[str], i: int, table: list[str]):
    """The (key, table) values a citation whose block ends at line `i` covers:
    from the value straight below to the next comment that cites a row,
    through the tables of the same widget."""
    values: list[tuple[str, list[str]]] = []
    current = table
    for later in lines[i + 1 :]:
        s = later.strip()
        h = HEADER.fullmatch(s)
        if h:
            current = h.group(1).split(".")
            if not values or widget_of(current) != widget_of(values[0][1]):
                break
            continue
        if s.startswith("#"):
            if CITATION.search(s):
                break
            continue
        v = VALUE.match(s)
        if v:
            values.append((v.group(1), current))
        elif not values:
            break
    return values


def subject_names(table: list[str], key: str, section_rows: set[str]) -> set[str]:
    """`names` for a key a comment names: relative to the widget, or
    `defaults.` and absolute, and `*` a wildcard."""
    if key.startswith("defaults."):
        table, key = ["light", "defaults"], key.removeprefix("defaults.")
    else:
        table = widget_of(table)
    if "*" not in key:
        return names(table, key, section_rows)
    widget, sub = scope(table)
    pattern = re.compile(".".join(sub + [key]).replace(".", r"\.").replace("*", r"\w*") + "$")
    out = {row for row in section_rows if pattern.match(row)}
    if key.startswith("disabled_"):
        out.add("disabled_opacity")
    return out


def check_preset(name: str, text: str, facts: Facts, exceptions: dict, used: set, counts: Counter):
    """Yield (comment line, key path, cited line, what is wrong) for one preset."""
    try:
        preset = tomllib.loads(text)
    except tomllib.TOMLDecodeError as err:
        raise InputError(f"{name}: {err}") from err
    lines = text.splitlines()
    table: list[str] = []
    block_start = 0
    block_cited = False
    for i, line in enumerate(lines):
        stripped = line.strip()
        h = HEADER.fullmatch(stripped)
        if h:
            table = h.group(1).split(".")
            continue
        if not stripped.startswith("#"):
            continue
        if i == 0 or not lines[i - 1].strip().startswith("#"):
            block_start, block_cited = i, False
        cites = list(CITATION.finditer(stripped))
        if not cites:
            continue
        end = i
        while end + 1 < len(lines) and lines[end + 1].strip().startswith("#"):
            end += 1
        below = lines[end + 1].strip() if end + 1 < len(lines) else ""
        text_ = comment_text(stripped)
        subject = named(text_)
        if subject is None and i > block_start and not block_cited:
            subject = named(comment_text(lines[block_start]))
        leading = MARKER.sub("", text_, count=1).startswith(("platform-facts.md:", "docs/platform-facts.md:"))
        for k, m in enumerate(cites):
            counts["citations"] += 1
            if subject is not None:
                kind = "named"
            elif (leading and k == 0) or not block_cited:
                kind = "value"
            else:
                kind = "rows"
            block_cited = True
            values = covered(lines, end, table) if kind == "value" and VALUE.match(below) and not HEADER.fullmatch(below) else []
            if kind == "value" and not values:
                kind = "rows"
            if kind == "value":
                key, key_table = values[0]
                key_path = ".".join(key_table + [key])
            else:
                key, key_table = "", table
                key_path = ".".join(table + (["/".join(subject[0])] if subject else []))
            wrong = []
            rows = []
            for n, stated in cited_lines(m):
                if n not in facts.chapter2:
                    continue
                if n not in facts.rows:
                    wrong.append((n, "the cited line is not a row"))
                    continue
                row, section = facts.rows[n]
                if stated and not any(
                    section and (section == s or section.startswith(s + ".")) for s in stated
                ):
                    shown = ", ".join(f"§{s}" for s in stated)
                    wrong.append((n, f"`{row}` is in §{section}, not {shown}"))
                rows.append((n, row, facts.section_rows.get(section, set())))
            excused = {r[1] for r in rows if (name, key_path, r[1]) in exceptions}
            used.update((name, key_path, row) for row in excused)
            compared = [r for r in rows if r[1] not in excused]
            if kind == "rows" or not compared:
                pass
            elif kind == "named":
                keys = subject[0] + subject[1]
                if not any(row in subject_names(table, k, sr) for _, row, sr in compared for k in keys):
                    shown = ", ".join(f"`{row}`" for _, row, _ in compared)
                    wrong.append((compared[0][0], f"{shown}, not the key the comment names"))
            elif not any(row in names(key_table, key, sr) for _, row, sr in compared):
                shown = ", ".join(f"`{row}`" for _, row, _ in compared)
                wrong.append((compared[0][0], f"{shown}, not the value"))
            elif len(rows) > 1:
                leaves = present(preset, key_table)
                matched = {facts.rows[n][1] for n, row, sr in compared if row in names(key_table, key, sr)}
                for n, row, sr in compared:
                    if facts.rows[n][1] in matched and row in leaves and not any(
                        row in names(t, k, sr) for k, t in values
                    ):
                        wrong.append((n, f"`{row}` names no value the citation covers"))
            if wrong:
                counts["flagged"] += 1
            elif kind == "rows" or not rows:
                counts["not compared"] += 1
            elif excused:
                counts["excepted"] += 1
            else:
                counts["held"] += 1
            yield from ((f"{name}:{i + 1}", key_path, n, what) for n, what in wrong)


def findings(presets: dict[str, str], facts: Facts, exceptions: dict) -> tuple[list, dict]:
    out = []
    used: set = set()
    per_preset: dict[str, Counter] = {}
    for name, text in presets.items():
        counts: Counter = Counter()
        out += check_preset(name, text, facts, exceptions, used, counts)
        per_preset[name] = counts
        if not counts["citations"] and re.search(r"platform-facts\.md:\d", text):
            raise InputError(f"{name}: cites platform-facts.md, and the rule reads none of it")
    for preset, key_path, row in sorted(set(exceptions) - used):
        out.append((preset, key_path, 0, f"an exception for `{row}` that no citation uses"))
    return out, per_preset


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError as err:
        raise InputError(f"{path}: not UTF-8 ({err})") from err


def load_presets(root: Path) -> dict[str, str]:
    out = {path.name: read_text(path) for path in sorted(root.glob("*.toml"))}
    if not out:
        raise InputError(f"{root}: no presets")
    return out


SELF_TEST_FACTS = """# Title
## Chapter 1: What OSes Provide
| `font` | Body text |
## Chapter 2: Cross-Platform Property Mapping
### 2.1 Global Defaults
#### 2.1.1 Base Font
| Property | KDE |
|---|---|
| `size` | 10 |
#### 2.1.6 Geometry
| `disabled_opacity` | 0.5 |
### 2.3 Button
| Property | KDE |
| `background_color` | x |
| `border.color` | x |
| `border.padding_horizontal` | 6 |
| `border.padding_vertical` | 5 |
| `hover_background` | x |
| `font` | x |
| `font.color` | x |
| `border.line_width` | x |
| `disabled_opacity` | x |
| `disabled_background` | x |
| `hover_text_color` | x |
## Appendix
"""

SELF_TEST_PRESET = """[light.defaults.font]
# platform-facts.md:9 (§2.1.1): a bare part
size_px = 10.0
# platform-facts.md:3 (§1.3.1): Chapter 1, not compared
family = "Noto Sans"
[light.button]
# platform-facts.md:18 (§2.3): holds
hover_background = "#000000"
# platform-facts.md:17 (§2.3): drifted
background_color = "#000000"
# platform-facts.md:11 (§2.1.6): disabled at the opacity
disabled_background = "#000000"
# platform-facts.md:14 (§2.3): a state's fill
unchecked_background = "#000000"
# platform-facts.md:14 (§2.1): the wrong section
active_background = "#000000"
# hover_text_color inherits font.color (linkColor,
# platform-facts.md:20) and so on
hover_text_color = "#000000"
# platform-facts.md:13: the table header, not a row
primary_background = "#000000"
# platform-facts.md:15 (§2.3): describes the table
[light.button.border]
# platform-facts.md:16-17 (§2.3): both axes
padding_left_px = 6.0

padding_top_px = 5.0
# ❓ platform-facts.md:16-17 (§2.3): one axis only
padding_horizontal_px = 6.0
# platform-facts.md:15, :21 (§2.3): no outline, the colour left out
line_width_px = 0.0
# platform-facts.md:21 (§2.3): excepted
corner_radius_px = 1.0
[light.button.font]
# platform-facts.md:19 (§2.1.1, §2.3): the font's row where the part has its own
color = "#000000"
# platform-facts.md:19: the font's row for a part without one
size_px = 14.0
[light.input]
# platform-facts.md:11 (§2.1.6), :14 (§2.3): a section per line
disabled_background = "#000000"
background_color = "#000000"
# platform-facts.md:15, :17 (§2.3): a row its widget sets, past the next citation
border.color = "#000000"
# platform-facts.md:17 (§2.3): the next citation
border.padding_vertical_px = 5.0
[light.link]
# platform-facts.md:22 (§2.3): the opacity row, where the section has the value's own row
disabled_background = "#000000"
# platform-facts.md:14 (§2.3): the fill's row, where the section has the value's own row
hover_background = "#000000"
# hover_text_color not stated: platform-facts.md:24 (§2.3): a key the comment names
# platform-facts.md:14 (§2.3): the next line's own
background_color = "#000000"
# active_text_color not stated: platform-facts.md:23 (§2.3): drifted
# no disabled_text_color / visited_text_color: platform-facts.md:11 (§2.1.6), the opacity dims them
# no macOS 26 source (platform-facts.md:20): another preset's value
font.color = "#000000"
# the text's role (docs/platform-facts.md:21)
border.line_width_px = 1.0
"""

SELF_TEST_EXCEPTIONS = {
    ("p.toml", "light.button.border.corner_radius_px", "border.line_width"): "test",
    ("p.toml", "light.input.border_color", "border.color"): "test, unused",
}


def self_test() -> int:
    facts = Facts(SELF_TEST_FACTS)
    got, counts = findings({"p.toml": SELF_TEST_PRESET}, facts, SELF_TEST_EXCEPTIONS)
    want = [
        ("p.toml:9", "light.button.background_color", 17, "`border.padding_vertical`, not the value"),
        ("p.toml:15", "light.button.active_background", 14, "`background_color` is in §2.3, not §2.1"),
        ("p.toml:15", "light.button.active_background", 14, "`background_color`, not the value"),
        ("p.toml:20", "light.button.primary_background", 13, "the cited line is not a row"),
        ("p.toml:28", "light.button.border.padding_horizontal_px", 17,
         "`border.padding_vertical` names no value the citation covers"),
        ("p.toml:35", "light.button.font.color", 19, "`font`, not the value"),
        ("p.toml:43", "light.input.border.color", 17,
         "`border.padding_vertical` names no value the citation covers"),
        ("p.toml:48", "light.link.disabled_background", 22, "`disabled_opacity`, not the value"),
        ("p.toml:50", "light.link.hover_background", 14, "`background_color`, not the value"),
        ("p.toml:55", "light.link.active_text_color", 23,
         "`disabled_background`, not the key the comment names"),
        ("p.toml", "light.input.border_color", 0, "an exception for `border.color` that no citation uses"),
    ]
    want_counts = {"citations": 27, "held": 15, "not compared": 2, "excepted": 1, "flagged": 9}
    failures = []
    if got != want:
        failures.append(f"rule: got {got}, want {want}")
    if dict(counts["p.toml"]) != want_counts:
        failures.append(f"counts: got {dict(counts['p.toml'])}, want {want_counts}")
    for label, preset in (
        ("a preset that does not parse", "[light\n"),
        ("a preset whose citations the rule reads none of", 'note = "platform-facts.md:9"\n'),
    ):
        try:
            findings({"p.toml": preset}, facts, {})
            failures.append(f"{label}: accepted")
        except InputError:
            pass
    with tempfile.TemporaryDirectory() as root:
        (Path(root) / "p.toml").write_bytes(b"# \xff\n")
        try:
            load_presets(Path(root))
            failures.append("a preset that is not UTF-8: accepted")
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
        facts = Facts(read_text(FACTS))
        out, counts = findings(load_presets(PRESETS), facts, EXCEPTIONS)
    except (OSError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for here, key_path, n, what in out:
        print(f"{here}\t{key_path}\tplatform-facts.md:{n}\t{what}")
    for name, c in counts.items():
        print(
            f"{name}: {c['citations']} citations, {c['held']} hold, "
            f"{c['excepted']} excepted, {c['not compared']} not compared, "
            f"{c['flagged']} flagged",
            file=sys.stderr,
        )
    print(f"{len(out)} preset citations that do not hold", file=sys.stderr)
    return 1 if out else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
