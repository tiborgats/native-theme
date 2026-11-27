#!/usr/bin/env python3
"""Read every preset comment's citation, and count them by kind.

A preset value's comment cites the docs/platform-facts.md row it comes from
in one of two forms: by line, `# platform-facts.md:1192 (§2.3): ...`, or by
section alone, `# platform-facts.md (§2.3): ...`, above
`hover_background = ...`. A line number drifts with every edit above it, so
this script compares no citation with the facts file's rows: it reads each
citation of either form and its kind, and fails on a preset it cannot read.
A third form, `# platform-facts.md §2.3 <key>: ...`, with neither a number
nor parentheses, is read by neither this script nor another.

Until the cut it reads both kinds of preset: the 0.7.2 TOMLs by the rule
below, and the 0.8.0 builders, native-theme/src/next/presets/*.rs, by the
rule "Builders" further down.

Run from the repository root:

    python3 scripts/check_preset_citations.py
    python3 scripts/check_preset_citations.py --self-test

It prints, on stderr, the counts of every preset: for a TOML its citations,
and how many of them cite named keys, a value and rows; for a builder its
citations, and how many of them cite platform-facts.md, `(uncited)` and
another source. Exit 0 when it reads every preset, 2 on an input it cannot
read: a file that is not UTF-8 or TOML, a facts file with no Chapter 2
rows, a TOML preset that cites `platform-facts.md:N` or
`platform-facts.md (§x)` where the rule reads no citation, or a builder
citation the rule "Builders" refuses.

Rule
----
Presets: each native-theme/src/presets/*.toml.

Citations. Every `platform-facts.md` or `docs/platform-facts.md` in a
comment, followed either by `:` and a list of lines, `N`, `N-M` or further
`, :K`, each optionally followed by its sections, ` (§2.3)` or
` (§2.1.3, §2.1.6)`, or by its sections alone, ` (§2.6)` or
` (§2.6, §2.15)`.

Kinds. A citation cites one of three things:

- named: the keys its comment line names, by one of the forms
  `no K / K: ...`, `K not stated: ...`, `K no source ...`, `K inherits X ...`
  and `K ← X; K ← X: ...` (a `K` relative to the widget, or `defaults.K`,
  `*` a wildcard; `no source` is a mark, not a key). A line with no form of
  its own, inside a comment whose first line has one and cites nothing,
  takes that line's keys (`# hover_text_color ... inherit font.color
  (linkColor,` then `# platform-facts.md:1694) ...`);
- value: else the value straight below its comment, when it is the line's
  first citation and the line starts with it (after a `❓`, `no source:`,
  `soft_option,` or `no macOS 26 source (` mark), or the comment's first
  citation wherever it sits (`# role (docs/platform-facts.md:1056)`);
- rows: else nothing: a citation mid-sentence after another, or above a
  blank line or a table header.

Builders
--------
Presets: each native-theme/src/next/presets/*.rs but mod.rs.

Citations. A citation is a `//` comment (not `///` or `//!`) that starts
with the keys it cites, each in backticks, joined by `, `, ` and ` or
`, also `, the list going on to the next line after a trailing comma, then
optionally one word (`corners`) or words that start with `in` (`in the
view that has the focus`), then `:` and its source; the source runs over the `//` lines below it up to the
next citation or the comment's end, the lines joined by a space (spec rule
4: a comment is not stretched over the lines below it). A key is a model
key, lower case (`list.hover_border_color`), its alternatives in braces
(`defaults.{danger,warning}_text_color`); a comment that starts with
another backticked word (`L2`) is no citation.

Kinds. By the start of the source:

- facts: `platform-facts.md` or `docs/platform-facts.md`, by its sections
  in parentheses, `(§2.3)` or `(§2.1.3, §2.3)`, or bare, `§2.13`, the
  third form the 0.7.2 TOMLs carry;
- uncited: `0.7.2 <file> (uncited)`, a value the 0.7.2 TOML `<file>`
  (native-theme/src/presets/<file>) leaves uncited;
- other: any other source (an upstream file at its version, another
  document of the repository).

Every `platform-facts.md` in a builder's comments, a citation's or not, is
read: one followed by `:` and a number cites a line, which the rule refuses
(spec rule 4); one followed by parentheses must hold `§` sections and
nothing else; and every section either form names must be a `###` or
`####` heading of the facts file. A citation with no source, a `(uncited)`
not in the form above, or a TOML that does not exist is refused too.
"""

from __future__ import annotations

import re
import sys
import tempfile
import tomllib
from collections import Counter
from pathlib import Path

PRESETS = Path("native-theme/src/presets")
BUILDERS = Path("native-theme/src/next/presets")
FACTS = Path("docs/platform-facts.md")

SECTIONS = r"\(§\d+(?:\.\d+)*(?:, §\d+(?:\.\d+)*)*\)"
PIECE = rf"\d+(?:-\d+)?(?: {SECTIONS})?"
CITATION = re.compile(
    rf"(?<![\w.-])(?:docs/)?platform-facts\.md(?::{PIECE}(?:,\s*:{PIECE})*| {SECTIONS})"
)
LEADING = (
    "platform-facts.md:",
    "docs/platform-facts.md:",
    "platform-facts.md (§",
    "docs/platform-facts.md (§",
)
CITES = re.compile(r"platform-facts\.md(?::\d| \(§)")
KEY = r"[a-z_][a-z0-9_.*]*"
KEYS = rf"{KEY}(?:(?:\s*/\s*|,\s*|,? and ){KEY})*"
NAMED = (
    re.compile(rf"^no ({KEYS})\s*:"),
    re.compile(rf"^({KEYS}) not stated\b"),
    re.compile(rf"^({KEYS}) no source\b"),
    re.compile(rf"^({KEYS}) inherits? {KEY}"),
)
ARROW = re.compile(rf"(?:^|;\s*){KEY} ← {KEY}")
MARKER = re.compile(r"^(?:soft_option,?\s*)?(?:no (?:macOS 26 )?source(?: ❓)?\s*[:(]?\s*)?")
HEADER = re.compile(r"\[([A-Za-z0-9_.-]+)\]")
VALUE = re.compile(r"([A-Za-z0-9_.]+)\s*=")
ROW = re.compile(r"\|\s*`([a-z0-9_.]+)`\s*\|")
CHAPTER = re.compile(r"##\s+Chapter\s+(\d+)\b")
OTHER_H2 = re.compile(r"##\s")
SECTION = re.compile(r"#{3,4}\s+(\d+\.\d+(?:\.\d+)?)\s")


class InputError(Exception):
    """A preset or facts file the rule cannot read."""


class Facts:
    """The facts file's Chapter 2 rows, counted: a facts file with none is an
    input error."""

    def __init__(self, text: str):
        self.rows = 0
        self.sections: set[str] = set()
        chapter: str | None = None
        section: str | None = None
        for line in text.splitlines():
            m = CHAPTER.match(line)
            if m:
                chapter, section = m.group(1), None
            elif OTHER_H2.match(line):
                chapter, section = None, None
            m = SECTION.match(line)
            if m:
                section = m.group(1)
                self.sections.add(section)
            if chapter == "2" and section is not None and ROW.match(line):
                self.rows += 1
        if not self.rows:
            raise InputError(f"{FACTS}: no Chapter 2 rows")


def comment_text(line: str) -> str:
    return re.sub(r"^#\s*(?:❓\s*)?", "", line.strip())


def named(text: str) -> bool:
    """Whether a comment names keys as its subject: `no K / K:`,
    `K not stated`, `K no source`, `K inherits X`, `K ← X`."""
    split = lambda keys: [k for k in re.split(r"\s*/\s*|,\s*|,? and ", keys) if k]
    for form in NAMED:
        m = form.match(text)
        if m and "source" not in split(m.group(1)):
            return True
    return bool(ARROW.search(text))


def check_preset(name: str, text: str) -> Counter:
    """One preset's citations, counted by kind."""
    try:
        tomllib.loads(text)
    except tomllib.TOMLDecodeError as err:
        raise InputError(f"{name}: {err}") from err
    counts: Counter = Counter()
    lines = text.splitlines()
    block_start = 0
    block_cited = False
    for i, line in enumerate(lines):
        stripped = line.strip()
        if not stripped.startswith("#"):
            continue
        if i == 0 or not lines[i - 1].strip().startswith("#"):
            block_start, block_cited = i, False
        cites = CITATION.findall(stripped)
        if not cites:
            continue
        end = i
        while end + 1 < len(lines) and lines[end + 1].strip().startswith("#"):
            end += 1
        below = lines[end + 1].strip() if end + 1 < len(lines) else ""
        value_below = bool(VALUE.match(below)) and not HEADER.fullmatch(below)
        text_ = comment_text(stripped)
        subject = named(text_)
        if not subject and i > block_start and not block_cited:
            subject = named(comment_text(lines[block_start]))
        leading = MARKER.sub("", text_, count=1).startswith(LEADING)
        for k in range(len(cites)):
            counts["citations"] += 1
            if subject:
                kind = "named"
            elif ((leading and k == 0) or not block_cited) and value_below:
                kind = "value"
            else:
                kind = "rows"
            block_cited = True
            counts[kind] += 1
    return counts


def count_citations(presets: dict[str, str]) -> dict[str, Counter]:
    per_preset: dict[str, Counter] = {}
    for name, text in presets.items():
        counts = check_preset(name, text)
        per_preset[name] = counts
        if not counts["citations"] and CITES.search(text):
            raise InputError(f"{name}: cites platform-facts.md, and the rule reads none of it")
    return per_preset


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

RS_COMMENT = re.compile(r"^\s*//(?![/!])\s?(.*)$")
RS_ANY_COMMENT = re.compile(r"^\s*//[/!]?\s?(.*)$")
RS_KEY = r"`[a-z_{][a-z0-9_.{},]*`"
RS_KEYS = rf"{RS_KEY}(?:(?:, also |, | and ){RS_KEY})*"
RS_HEAD = re.compile(rf"^({RS_KEYS})((?: in [^:`]*| [a-z]+)?):\s*(.*)$")
RS_KEYS_GO_ON = re.compile(rf"^{RS_KEYS},$")
RS_FACTS = re.compile(
    r"(?<![\w.-])(?:docs/)?platform-facts\.md"
    r"(?::(?P<line>\d+)|\s+\((?P<paren>§[^)]*)\)|\s+(?P<bare>§\d+(?:\.\d+)*))"
)
RS_SECTIONS = re.compile(r"§(\d+(?:\.\d+)*)(?:, §(\d+(?:\.\d+)*))*")
RS_LEADS_WITH_FACTS = re.compile(r"^(?:docs/)?platform-facts\.md\s+(?:\(§|§)")
RS_UNCITED = re.compile(r"^0\.7\.2 (\S+) \(uncited\)")


def comment_runs(lines: list[str], pattern: re.Pattern) -> list[list[tuple[int, str]]]:
    """The runs of consecutive comment lines `pattern` matches, each line's
    number (from 1) and text without its marker."""
    runs: list[list[tuple[int, str]]] = []
    current: list[tuple[int, str]] = []
    for number, line in enumerate(lines, start=1):
        m = pattern.match(line)
        if m:
            current.append((number, m.group(1).strip()))
        elif current:
            runs.append(current)
            current = []
    if current:
        runs.append(current)
    return runs


def builder_citations(name: str, text: str) -> list[tuple[int, str]]:
    """The citations of a builder: each one's line and source, its lines
    joined by a space."""
    out: list[tuple[int, str]] = []
    for run in comment_runs(text.splitlines(), RS_COMMENT):
        current: tuple[int, list[str]] | None = None
        i = 0
        while i < len(run):
            number, line = run[i]
            head = line
            j = i
            while RS_KEYS_GO_ON.match(head) and j + 1 < len(run):
                j += 1
                head = f"{head} {run[j][1]}"
            m = RS_HEAD.match(head)
            if m:
                if current is not None:
                    out.append((current[0], " ".join(current[1]).strip()))
                current = (number, [m.group(3)])
                i = j + 1
                continue
            if current is not None:
                current[1].append(line)
            i += 1
        if current is not None:
            out.append((current[0], " ".join(current[1]).strip()))
    return out


def check_builder_facts(name: str, text: str, facts: Facts) -> None:
    """Every `platform-facts.md` in the builder's comments, as the rule
    "Builders" reads it."""
    for run in comment_runs(text.splitlines(), RS_ANY_COMMENT):
        joined = " ".join(line for _, line in run)
        for m in RS_FACTS.finditer(joined):
            where = f"{name}:{run[0][0]}"
            if m.group("line") is not None:
                raise InputError(f"{where}: cites platform-facts.md:{m.group('line')}, a line")
            cited = m.group("paren") or m.group("bare")
            if not RS_SECTIONS.fullmatch(cited):
                raise InputError(f"{where}: platform-facts.md ({cited}) names no sections alone")
            for section in re.findall(r"§(\d+(?:\.\d+)*)", cited):
                if section not in facts.sections:
                    raise InputError(f"{where}: platform-facts.md has no section §{section}")


def check_builder(name: str, text: str, facts: Facts, presets: Path) -> Counter:
    """One builder's citations, counted by kind."""
    check_builder_facts(name, text, facts)
    counts: Counter = Counter()
    for number, source in builder_citations(name, text):
        where = f"{name}:{number}"
        counts["citations"] += 1
        uncited = RS_UNCITED.match(source)
        if not source:
            raise InputError(f"{where}: a citation with no source")
        if RS_LEADS_WITH_FACTS.match(source):
            counts["facts"] += 1
        elif uncited:
            if not (presets / uncited.group(1)).is_file():
                raise InputError(f"{where}: no 0.7.2 preset {presets / uncited.group(1)}")
            counts["uncited"] += 1
        else:
            counts["other"] += 1
        if "(uncited)" in source and not uncited:
            raise InputError(f"{where}: `(uncited)` outside the form `0.7.2 <file> (uncited)`")
    return counts


def count_builder_citations(builders: dict[str, str], facts: Facts, presets: Path) -> dict[str, Counter]:
    return {name: check_builder(name, text, facts, presets) for name, text in builders.items()}


def load_builders(root: Path) -> dict[str, str]:
    out = {path.name: read_text(path) for path in sorted(root.glob("*.rs")) if path.name != "mod.rs"}
    if not out:
        raise InputError(f"{root}: no builders")
    return out


SELF_TEST_FACTS = """# Title
## Chapter 1: What OSes Provide
| `font` | Body text |
## Chapter 2: Cross-Platform Property Mapping
### 2.3 Button
| Property | KDE |
| `background_color` | x |
## Appendix
"""

SELF_TEST_FACTS_WITHOUT_ROWS = """# Title
## Chapter 1: What OSes Provide
| `font` | Body text |
## Chapter 2: Cross-Platform Property Mapping
### 2.3 Button
| Property | KDE |
## Appendix
"""

SELF_TEST_PRESET = """[light.defaults.font]
# platform-facts.md:9 (§2.1.1): a value, by line
size_px = 10.0
# platform-facts.md (§2.1.1): a value, by section
family = "Noto Sans"
[light.button]
# hover_background not stated: platform-facts.md:18 (§2.3), a named key, by line
# hover_text_color not stated: platform-facts.md (§2.3), a named key, by section
# no disabled_text_color: platform-facts.md (§2.1.6, §2.3), by two sections
background_color = "#000000"
# docs/platform-facts.md (§2.3) and platform-facts.md:14, :16-17 (§2.3): a value, then rows
border.color = "#000000"
# the text's role (docs/platform-facts.md (§2.3))
font.color = "#000000"
# the hover fill (platform-facts.md:14), and
# platform-facts.md (§2.3): a value, the line's first citation after another line's
active_background = "#000000"
# the pressed fill (platform-facts.md:14), and
# no macOS 26 source (platform-facts.md (§2.3)): a marked value after another line's
disabled_background = "#000000"
# hover_text_color inherits font.color (linkColor,
# platform-facts.md:9) a named key, from the comment's first line
hover_text_color = "#000000"
# platform-facts.md (§2.3): above a table header
[light.link]
# platform-facts.md (§2.3): above a blank line

padding_top_px = 5.0
# platform-facts.md §2.3 padding_left: the third form, not read
padding_left_px = 6.0
# ❓ platform-facts.md:16-17 (§2.3): a marked value
padding_horizontal_px = 6.0
"""

SELF_TEST_BUILDER = """//! The module: `docs/platform-facts.md` by section, no citation.
/// A doc comment citing docs/platform-facts.md §2.3, checked, not counted.
const A: u8 = 0;
    // `defaults.text_color`: platform-facts.md (§2.3): a section, the source going on
    // to this line (BreezeLight.colors:137, breeze v6.7.5)
    // `defaults.danger_text_color`, `warning_text_color`,
    // `info_text_color`: platform-facts.md
    // (§2.3): a key list and a source split over lines
    // `toolbar.background_color`: platform-facts.md §2.3 background_color: the third form
    // `defaults.border.color`, also `input.font.color`: 0.7.2 p.toml
    // (uncited)
    // `menu.active_text_color` and `menu.bar_hover_text_color`: 0.7.2 p.toml (uncited)
    // `scrollbar.thumb` in the view that has the focus: docs/issues/x.md (S3): another source
    // `L2`: no citation, and so is this line
    // `list.hover_border_color`: "a quote", another source
    // `defaults.{danger,warning}_text_color`: platform-facts.md (§2.3), braces
    // `inputs.palette` and `inputs.accent` are not applied yet: no citation
"""


def self_test() -> int:
    failures = []
    Facts(SELF_TEST_FACTS)
    counts = count_citations({"p.toml": SELF_TEST_PRESET})
    want = {"citations": 16, "named": 4, "value": 9, "rows": 3}
    if dict(counts["p.toml"]) != want:
        failures.append(f"counts: got {dict(counts['p.toml'])}, want {want}")
    for label, preset in (
        ("a preset that does not parse", "[light\n"),
        ("a preset whose citations by line the rule reads none of", 'note = "platform-facts.md:9"\n'),
        ("a preset whose citations by section the rule reads none of", 'note = "platform-facts.md (§2.3)"\n'),
    ):
        try:
            count_citations({"p.toml": preset})
            failures.append(f"{label}: accepted")
        except InputError:
            pass
    try:
        Facts(SELF_TEST_FACTS_WITHOUT_ROWS)
        failures.append("a facts file with no Chapter 2 rows: accepted")
    except InputError:
        pass
    with tempfile.TemporaryDirectory() as root:
        (Path(root) / "p.toml").write_bytes(b"# \xff\n")
        try:
            load_presets(Path(root))
            failures.append("a preset that is not UTF-8: accepted")
        except InputError:
            pass
    facts = Facts(SELF_TEST_FACTS)
    with tempfile.TemporaryDirectory() as root:
        (Path(root) / "p.toml").write_text("", encoding="utf-8")
        counts = count_builder_citations({"p.rs": SELF_TEST_BUILDER}, facts, Path(root))
        want = {"citations": 8, "facts": 4, "uncited": 2, "other": 2}
        if dict(counts["p.rs"]) != want:
            failures.append(f"builder counts: got {dict(counts['p.rs'])}, want {want}")
        for label, builder in (
            ("a builder citing a line", "// `a`: platform-facts.md:12 (§2.3)\n"),
            ("a builder citing a line in a doc comment", "/// x (docs/platform-facts.md:12)\n"),
            ("a builder citing a section the facts lack", "// `a`: platform-facts.md (§2.4)\n"),
            ("a builder citing a bare section the facts lack", "//! platform-facts.md §9\n"),
            ("a builder citing more than sections", "// `a`: platform-facts.md (§2.3, rows)\n"),
            ("a builder citation with no source", "// `a`:\n"),
            ("a builder citing a TOML that does not exist", "// `a`: 0.7.2 q.toml (uncited)\n"),
            ("a builder's `(uncited)` in another form", "// `a`: p.toml (uncited)\n"),
        ):
            try:
                count_builder_citations({"p.rs": builder}, facts, Path(root))
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
        facts = Facts(read_text(FACTS))
        counts = count_citations(load_presets(PRESETS))
        builder_counts = count_builder_citations(load_builders(BUILDERS), facts, PRESETS)
    except (OSError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for name, c in counts.items():
        print(
            f"{name}: {c['citations']} citations, {c['named']} named, "
            f"{c['value']} value, {c['rows']} rows",
            file=sys.stderr,
        )
    for name, c in builder_counts.items():
        print(
            f"{BUILDERS / name}: {c['citations']} citations, {c['facts']} platform-facts.md, "
            f"{c['uncited']} uncited, {c['other']} other",
            file=sys.stderr,
        )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
