#!/usr/bin/env python3
"""Import today's docs/platform-facts.md rows into docs/facts/imported/ (the 0.8.0 plan's *What carries over*).

The output is lane input, not facts: the facts gate and scripts/check_fact_citations.py do not read `imported/`
(specification S17), and scripts/check_facts.py skips it. A lane turns each entry into a verified fact like any other,
or records it as not a model value, and removes it; the folder is empty at the plan's M4, when this script is deleted.

Run from the repository root:

    python3 scripts/import_platform_facts.py             # writes docs/facts/imported/<platform>.toml
    python3 scripts/import_platform_facts.py --check     # exit 1 when the files differ from a fresh run
    python3 scripts/import_platform_facts.py --self-test

It prints the counts per platform. Exit 0 on success (with --check: the files equal a fresh run), 1 when --check
finds a difference, 2 on an input it cannot read or a wrong command line.

Rows
----
The source is `docs/platform-facts.md` at commit fc6109d1 (SOURCE_PIN; the file there is the file in the tree, which
the script does not check: a later edit of the document makes the pin wrong, and the document is not edited before
the switch, the plan's *Re-pin*). Its tables from `## Chapter 1` up to `## Appendix` are read; the conventions before
Chapter 1 and the verification appendix are not rows. A table is a header line, a separator line and the lines after
it that start with `|`; cells are split at `|` not written `\\|`, which is a literal bar. A row with another number of
cells than its header is an input error.

- Chapter 2 (the cross-platform mapping): a column whose header starts with `macOS`, `Windows`, `KDE`, `GNOME` or
  `Material` (`Windows Fluent`, `KDE Breeze`, ...) is that platform's (PLATFORM_COLUMNS); a table without one is an
  input error. Each row gives one entry per platform column: the first cell is the property, the platform's cell the
  text. Another column (2.19's `What it is`) is written as `context`, `<header>: <cell>`.
- Chapter 1 (what each OS provides): the platform is the `### 1.N <OS>` section's; each row gives one entry, its first
  cell the property and its other cells the text, joined by ` | `, with the header's other cells as `columns`.

A property whose whole cell is one backtick span is written without the backticks; every other text is written as the
document has it, links included (their targets are relative to `docs/`). `evidence` lists each link of the text whose
target lies under `docs/evidence/`, as a repository path with its fragment.

Rows and targets
----------------
A gap-map row names a platform-facts cell in its own platform's file
(`docs/todo_v0.7.1_model-completeness-gap-map-<platform>.md`): a table line whose first cell is a live row of that
platform (the rows of the map's Appendix A, without the withdrawn ones, as scripts/check_spec_trace.py reads them)
names, in its platform's column (a Chapter 1 row: when its section is that platform's), the cells that its text cites:

- by line: `platform-facts.md@fc6109d1:<l>` or `:<l1>-<l2>`, with `, :<l>` continuations; another commit than the
  pin is an input error;
- by section and lines: `platform-facts[.md] §<n> cell (line <l>)` or `cells (lines <l1>-<l2>, ...)`;
- by section and property: `platform-facts[.md] §<n>` followed by a backticked name, or by a bare name and the word
  `cell`: the row of section `<n>` (the heading numbered exactly so) whose property is the name, or else the one row
  whose property ends with `.<name>`; a name no row or several rows answer is not resolved. A section named without a
  property (`platform-facts §2.13`) names no cell.

Each gap map's table of its platform's "(none)" cells (a table whose first header cell is `platform-facts line` or
`Line(s)`) names, per line: its first cell's line numbers and ranges (`1170, 1171`, `1041–1043`, `:1040`), the class
(the `a/b`, `a / b` or `Class` column) and the reason (the last cell). Where the class says `b` (`b`, `a (12–15) / b
(26)`), the live rows of the platform that the reason names name those cells; the class and the reason are written
as `none_cell` either way. A cited line that holds no cell of the platform is counted as unresolved; in a
(none)-cell table only a single line is (a range or list may span lines that are no table row).

`rows` lists the rows that name the cell in the order of the gap map, `named_at` where (`<gap map>:<line>`), and
`targets` the targets of their lines in the platform's trace (`docs/todo_v0.7.1_model-completeness-spec-trace-
<platform>.md`), as the trace writes them, for the lines of a target kind; a verdict line is listed in `verdicts`
(`<row>: <kind>`, with its note). An entry with no target has a `note` saying why: no row names the cell, or its
rows' lines are verdicts only. The script never writes a key of its own.

Output
------
One file per platform with a column or a section (macos, windows, kde, gnome, material; platform-facts has no iOS
column), `[[row]]` entries in document order, each with `line` (`platform-facts.md@fc6109d1:<n>`), `section`,
`property`, `text` and, where they apply, `columns`, `context`, `evidence`, then `rows`, `named_at`, `targets`,
`verdicts`, `none_cell`, `note`. `--check` compares the files with a fresh run, and a file in `imported/` that the run
does not write is a difference; it holds until a lane removes its first entry.
"""
import re
import sys
import tempfile
import tomllib
from pathlib import Path

import check_spec_trace

DOCS = Path("docs")
FACTS_MD = DOCS / "platform-facts.md"
SOURCE_PIN = "fc6109d1"
CITE = f"platform-facts.md@{SOURCE_PIN}"
OUT = DOCS / "facts" / "imported"
MAP = DOCS / "todo_v0.7.1_model-completeness-gap-map.md"
# platform slug: (its Appendix A name, its Chapter 1 section name, its Chapter 2 column header's first word)
PLATFORMS = {
    "macos": ("macOS", "macOS", "macOS"),
    "windows": ("Windows", "Windows", "Windows"),
    "kde": ("KDE", "KDE", "KDE"),
    "gnome": ("GNOME", "GNOME", "GNOME"),
    "material": ("Material", None, "Material"),
    "ios": ("iOS", None, None),
}
PLATFORM_COLUMNS = {v[2]: k for k, v in PLATFORMS.items() if v[2]}
CHAPTER1_SECTIONS = {v[1]: k for k, v in PLATFORMS.items() if v[1]}
NONE_TABLE_HEADS = ("platform-facts line", "Line(s)")
CLASS_HEADS = ("a/b", "a / b", "Class")

LINE_REF = re.compile(r"platform-facts(?:\.md)?@([0-9a-f]+):(\d+)(?:[-–](\d+))?((?:,\s*:\d+(?:[-–]\d+)?)*)")
CONT = re.compile(r":(\d+)(?:[-–](\d+))?")
SECTION_REF = re.compile(r"platform-facts(?:\.md)? §(\d+(?:\.\d+)*)")
SECTION_LINES = re.compile(r"\s+cells?\s+\(lines?\s+(\d+)(?:[-–](\d+))?")
SECTION_NAME = re.compile(r"\s+(?:`([^`]+)`|([a-z_][a-z0-9_.]*) cell\b)")
LINE_LIST = re.compile(r"^:?(\d+)(?:[-–](\d+))?((?:,\s*:?\d+(?:[-–]\d+)?)*)")
LINK = re.compile(r"\[[^\]]*\]\(([^)\s]+)\)")
HEADING = re.compile(r"^(#{2,5}) (.+?)\s*$")


class InputError(Exception):
    pass


def cells(line: str) -> list[str]:
    return check_spec_trace.cells(line)


def unbacktick(text: str) -> str:
    if len(text) > 2 and text[0] == text[-1] == "`" and "`" not in text[1:-1]:
        return text[1:-1]
    return text


def evidence_links(text: str) -> list[str]:
    out = []
    for m in LINK.finditer(text):
        target = m.group(1)
        if re.match(r"^[a-z]+:", target) or target.startswith("#"):
            continue
        path = (Path("docs") / target.split("#")[0]).as_posix()
        parts = []
        for seg in path.split("/"):
            if seg == "..":
                if parts:
                    parts.pop()
            elif seg not in (".", ""):
                parts.append(seg)
        path = "/".join(parts)
        if path.startswith("docs/evidence/"):
            frag = target.split("#", 1)[1] if "#" in target else ""
            ref = path + ("#" + frag if frag else "")
            if ref not in out:
                out.append(ref)
    return out


def read_rows(text: str):
    """(entries, sections): every row cell of Chapters 1 and 2 as an entry dict with its platform; sections maps a
    section number ("2.3", "1.1.4") to its row lines."""
    entries, sections = [], {}
    lines = text.splitlines()
    chapter, section, ch1_platform = None, None, None
    i = 0
    while i < len(lines):
        line = lines[i]
        m = HEADING.match(line)
        if m:
            title = m.group(2)
            if m.group(1) == "##":
                chapter = 1 if title.startswith("Chapter 1") else 2 if title.startswith("Chapter 2") else None
                if title.startswith("Appendix"):
                    break
                section = None
            elif chapter:
                section = title
                if chapter == 1 and m.group(1) == "###":
                    word = re.sub(r"^[\d.]+\s+", "", title)
                    if word not in CHAPTER1_SECTIONS:
                        raise InputError(f"{FACTS_MD}:{i + 1}: Chapter 1 section {title!r} names no platform")
                    ch1_platform = CHAPTER1_SECTIONS[word]
            i += 1
            continue
        if not (chapter and line.startswith("|")):
            i += 1
            continue
        header = cells(line)
        if i + 1 >= len(lines) or not re.fullmatch(r"[\s|:-]+", lines[i + 1]):
            raise InputError(f"{FACTS_MD}:{i + 1}: a table without a separator line")
        number = section.split()[0] if section else None
        if chapter == 2:
            cols = {k: PLATFORM_COLUMNS[h.split()[0]] for k, h in enumerate(header)
                    if k and h and h.split()[0] in PLATFORM_COLUMNS}
            if not cols:
                raise InputError(f"{FACTS_MD}:{i + 1}: a Chapter 2 table without a platform column")
        i += 2
        while i < len(lines) and lines[i].startswith("|"):
            row = cells(lines[i])
            n = i + 1
            if len(row) != len(header):
                raise InputError(f"{FACTS_MD}:{n}: {len(row)} cells, its header has {len(header)}")
            sections.setdefault(number, []).append(n)
            prop = unbacktick(row[0])
            if chapter == 2:
                context = [f"{header[k]}: {row[k]}" for k in range(1, len(row)) if k not in cols]
                for k, platform in cols.items():
                    e = {"platform": platform, "line": n, "section": section, "property": prop, "text": row[k]}
                    if context:
                        e["context"] = " · ".join(context)
                    entries.append(e)
            else:
                text_ = " | ".join(row[1:])
                entries.append({"platform": ch1_platform, "line": n, "section": section, "property": prop,
                                "text": text_, "columns": " | ".join(header[1:])})
            i += 1
    return entries, sections


def line_numbers(first: str, rest: str) -> list[int]:
    out = []
    for m in [re.match(r"(\d+)(?:[-–](\d+))?", first)] + list(CONT.finditer(rest)):
        a = int(m.group(1))
        b = int(m.group(2)) if m.group(2) else a
        out.extend(range(a, b + 1))
    return out


def references(text: str, sections: dict, props: dict) -> tuple[list[int], int, int]:
    """(the platform-facts lines a gap-map line cites, how many references named no row, how many a section only)."""
    out, unresolved, section_only = [], 0, 0
    for m in LINE_REF.finditer(text):
        if not SOURCE_PIN.startswith(m.group(1)) and not m.group(1).startswith(SOURCE_PIN):
            raise InputError(f"a citation of platform-facts.md at {m.group(1)}, not at {SOURCE_PIN}")
        out += line_numbers(m.group(2) + (f"-{m.group(3)}" if m.group(3) else ""), m.group(4))
    for m in SECTION_REF.finditer(text):
        number, after = m.group(1), text[m.end():]
        ml = SECTION_LINES.match(after)
        if ml:
            a = int(ml.group(1))
            b = int(ml.group(2)) if ml.group(2) else a
            got = [n for n in range(a, b + 1) if n in sections.get(number, ())]
            out += got
            unresolved += 0 if got else 1
            continue
        mn = SECTION_NAME.match(after)
        if not mn:
            section_only += 1
            continue
        name = unbacktick(mn.group(1) or mn.group(2))
        rows = [n for n in sections.get(number, ()) if props.get(n) == name]
        if not rows:
            rows = [n for n in sections.get(number, ()) if props.get(n, "").endswith("." + name)]
        if len(rows) == 1:
            out += rows
        else:
            unresolved += 1
    return out, unresolved, section_only


def id_finder(ids: set[str]):
    alts = "|".join(re.escape(x) for x in sorted(ids, key=len, reverse=True))
    return re.compile(rf"(?<![\w.-])({alts})(?![\w])")


def gap_map_names(platform: str, map_text: str, sections: dict, props: dict, live: set[str]):
    """{line: [(row, where)]}, {line: none_cell}, (unresolved, section-only references) for one platform's gap map."""
    path = DOCS / f"todo_v0.7.1_model-completeness-gap-map-{platform}.md"
    named, none_cells, unresolved, section_only = {}, {}, 0, 0
    finder = id_finder(live) if live else None
    in_none, class_col = False, None
    for n, line in enumerate(map_text.splitlines(), 1):
        if not line.startswith("|"):
            in_none = False
            continue
        c = cells(line)
        where = f"{path.name}:{n}"
        if c[0] in NONE_TABLE_HEADS:
            in_none = True
            class_col = next((k for k, h in enumerate(c) if h in CLASS_HEADS), None)
            if class_col is None:
                raise InputError(f"{where}: a (none)-cell table without a class column")
            continue
        if in_none:
            if re.fullmatch(r"[\s:-]*", c[0]):
                continue
            m = LINE_LIST.match(c[0])
            if not m:
                raise InputError(f"{where}: no platform-facts line in {c[0]!r}")
            nums = line_numbers(m.group(1) + (f"-{m.group(2)}" if m.group(2) else ""),
                                m.group(3).replace(", ", ", :").replace("::", ":"))
            cls, reason = c[class_col], c[-1]
            rows = finder.findall(reason) if finder and re.search(r"\bb\b", cls) else []
            hits = [x for x in nums if x in props]
            unresolved += 0 if hits or len(nums) > 1 else 1
            for x in hits:
                none_cells[x] = f"{cls}: {reason}"
                for r in rows:
                    named.setdefault(x, []).append((r, where))
            continue
        if c[0] not in live:
            continue
        refs, unres, only = references(line, sections, props)
        unresolved += unres
        section_only += only
        for x in refs:
            if x in props:
                named.setdefault(x, []).append((c[0], where))
            else:
                unresolved += 1
    return named, none_cells, (unresolved, section_only)


def build(root: Path) -> tuple[dict, dict]:
    """({platform: TOML text}, {platform: counts})."""
    try:
        text = (root / FACTS_MD).read_text(encoding="utf-8")
        map_text = (root / MAP).read_text(encoding="utf-8")
    except OSError as err:
        raise InputError(str(err)) from err
    entries, sections = read_rows(text)
    props = {e["line"]: e["property"] for e in entries}
    try:
        all_rows = check_spec_trace.map_rows(map_text)
    except check_spec_trace.InputError as err:
        raise InputError(f"{MAP}: {err}") from err
    named, none_cells, traces, unresolved = {}, {}, {}, {}
    for platform, (name, _, _) in PLATFORMS.items():
        gm = root / DOCS / f"todo_v0.7.1_model-completeness-gap-map-{platform}.md"
        tr = root / DOCS / f"todo_v0.7.1_model-completeness-spec-trace-{platform}.md"
        try:
            gm_text = gm.read_text(encoding="utf-8")
            trace = check_spec_trace.trace_lines(tr.read_text(encoding="utf-8"))
        except OSError as err:
            raise InputError(str(err)) from err
        except check_spec_trace.InputError as err:
            raise InputError(f"{tr}: {err}") from err
        live = all_rows[name] - check_spec_trace.withdrawn(gm_text, all_rows[name])
        # a Chapter 1 line is this platform's only in its own section
        own = {e["line"]: e["property"] for e in entries if e["platform"] == platform}
        named[platform], none_cells[platform], unresolved[platform] = gap_map_names(platform, gm_text, sections,
                                                                                      own, live)
        traces[platform] = {}
        for row, target, kind in trace:
            traces[platform].setdefault(row, []).append((target, kind))
    out, counts = {}, {}
    for platform in PLATFORMS:
        mine = [e for e in entries if e["platform"] == platform]
        if not mine:
            continue
        blocks = []
        n_target = 0
        for e in mine:
            hits = named[platform].get(e["line"], [])
            rows = list(dict.fromkeys(r for r, _ in hits))
            where = list(dict.fromkeys(w for _, w in hits))
            targets, verdicts = [], []
            for r in rows:
                for target, kind in traces[platform].get(r, []):
                    if kind in check_spec_trace.KINDS:
                        if target not in targets:
                            targets.append(target)
                    else:
                        v = f"{r}: {kind}" + ("" if target == "-" else f" {target}")
                        if v not in verdicts:
                            verdicts.append(v)
            fields = [("line", f"{CITE}:{e['line']}"), ("section", e["section"]), ("property", e["property"]),
                      ("text", e["text"])]
            if "columns" in e:
                fields.append(("columns", e["columns"]))
            if "context" in e:
                fields.append(("context", e["context"]))
            ev = evidence_links(e["text"])
            if ev:
                fields.append(("evidence", ev))
            fields.append(("rows", rows))
            if where:
                fields.append(("named_at", where))
            fields.append(("targets", targets))
            if verdicts:
                fields.append(("verdicts", verdicts))
            if e["line"] in none_cells[platform]:
                fields.append(("none_cell", none_cells[platform][e["line"]]))
            if not targets:
                fields.append(("note", "no gap-map row names this cell" if not rows
                               else "its rows' trace lines are verdicts only" if verdicts
                               else "its rows have no trace line"))
            else:
                n_target += 1
            blocks.append("[[row]]\n" + "".join(f"{k} = {toml_value(v)}\n" for k, v in fields))
        out[platform] = HEADER.format(platform=platform) + "\n" + "\n".join(blocks)
        counts[platform] = (len(mine), n_target) + unresolved[platform]
    return out, counts


HEADER = """\
# Today's platform-facts.md cells for {platform}, from {cite}: lane input, not facts (specification S17;
# the 0.8.0 plan's *What carries over*). The facts gate and check_fact_citations.py do not read this folder; a lane
# turns each entry into a verified fact or records it as not a model value, and removes it. Written by
# scripts/import_platform_facts.py, whose docstring states the rules. `text` is the cell as written, its links
# relative to docs/; `targets` are the trace's, of the gap-map `rows` that name the cell.
""".replace("{cite}", CITE)


def toml_string(s: str) -> str:
    out = []
    for ch in s:
        if ch == "\\":
            out.append("\\\\")
        elif ch == '"':
            out.append('\\"')
        elif ch == "\n":
            out.append("\\n")
        elif ch == "\t":
            out.append("\\t")
        elif ord(ch) < 0x20 or ord(ch) == 0x7F:
            out.append(f"\\u{ord(ch):04X}")
        else:
            out.append(ch)
    return '"' + "".join(out) + '"'


def toml_value(v) -> str:
    if isinstance(v, str):
        return toml_string(v)
    if isinstance(v, list):
        if not v:
            return "[]"
        return "[\n" + "".join(f"    {toml_string(x)},\n" for x in v) + "]"
    raise TypeError(v)


def write(root: Path, out: dict, check: bool) -> list[str]:
    """The differences (with check) or the files written."""
    base = root / OUT
    diffs = []
    want = {f"{p}.toml": text for p, text in out.items()}
    have = {p.name: p for p in base.iterdir() if p.is_file()} if base.is_dir() else {}
    for name, text in want.items():
        p = base / name
        old = p.read_text(encoding="utf-8") if p.is_file() else None
        if old != text:
            diffs.append(f"{(OUT / name).as_posix()}: differs from a fresh run" if old is not None
                         else f"{(OUT / name).as_posix()}: missing")
            if not check:
                base.mkdir(parents=True, exist_ok=True)
                p.write_text(text, encoding="utf-8")
    for name in sorted(set(have) - set(want)):
        diffs.append(f"{(OUT / name).as_posix()}: not written by a fresh run")
    return diffs


SELF_TEST_FACTS = """\
# Facts

### Conventions
| Tag | Meaning |
|---|---|
| (API) | an API |

## Chapter 1: What OSes Provide

### 1.1 macOS

#### 1.1.3 Geometry

| Property | Source | Value |   |
|---|---|---|---|
| Control corner radius | measured | 5px | ✅ ([probe](evidence/os-probe/run-1/os-probe-macos-14/probe.toml#L7)) |

### 1.4 GNOME

#### 1.4.1 Fonts

| Key | Default |
|---|---|
| `font-name` | Cantarell 11 \\| Adwaita Sans 11 |

## Chapter 2: Cross-Platform Property Mapping

### 2.3 Button

| Property | macOS | Windows | KDE | GNOME | Material |
|---|---|---|---|---|---|
| `background_color` | `controlColor` | **(Fluent)** "Button" | ButtonBackground | **(Adwaita CSS)** | **(M3 token)** |
| `border.corner_radius` | ← `defaults.border.corner_radius` | 4 | 5 | 6 | 20 |
| `min_width` | **(none)** — sizes to content | (none) | 80 | (none) | 64 |

### 2.19 Text Scale

| Role | What it is | macOS | Windows Fluent | KDE (Kirigami heading) | GNOME libadwaita | Material |
|---|---|---|---|---|---|---|
| `caption` | small text | 10pt | 12px | 8pt | 9pt | 12 |

## Appendix: Verification Sources

| What | Source |
|---|---|
| everything | here |
"""

SELF_TEST_MAP = """\
## Appendix A. Rows per capability
- macOS: MAC-B1, MAC-B11†, MAC-G1, MAC-B2
- Windows: W1-001
- KDE: K1.1
- GNOME: GN-1.01, GN-1.02, GN-1.38
- Material: MAT-B4
- iOS: B-1
## Appendix B
"""

SELF_TEST_GAP_MAPS = {
    "macos": """\
| ID | Element | Missing | Source | Model |
|---|---|---|---|---|
| MAC-B1 | button | fill per state (platform-facts §2.3 `background_color`) | M | `button.background_color` |
| MAC-B11† | button | `min_width` is (none) (platform-facts.md@fc6109d1:34) | M | `button.min_width` |
| MAC-G1 | radius | corner radius (platform-facts.md@fc6109d1:16, :33) | M | none |
| MAC-B2 | button | Withdrawn (review 2026-10-28): platform-facts.md@fc6109d1:32 | M | none |

### C. The macOS "(none)" cells

| platform-facts line | Property | a/b | Row, or reason |
|---|---|---|---|
| 34 | button `min_width` | b | MAC-B11†. |
""",
    "windows": "| W1-001 | button | radius (platform-facts §2.3 corner_radius cell; platform-facts §2.3 `nope`) | P | none |\n",
    "kde": """\
| K1.1 | font | (platform-facts.md §1.4.1) | P | none |

| platform-facts line | a/b | Row, or reason |
|---|---|---|
| 34 `min_width` | a | sizes to content; MAC-B11† is macOS's (K1.1) |
""",
    "gnome": """\
| GN-1.01 | font | the font key (`docs/platform-facts.md@fc6109d1:24`) | P | none |
| GN-1.02 | text | caption (platform-facts §2.19 `caption`) | P | none |
| GN-1.38 | button HC | ring | P | none |
| K1.1 | 2 | cross-check: platform-facts.md@fc6109d1:32 |
""",
    "material": """\
| MAT-B4 | button | padding | src | none | platform-facts.md §2.3 cells (lines 33-34) |

| Line(s) | a/b | Row, or reason |
|---|---|---|
| 32–33 | a | as the cell |
""",
    "ios": "| B-1 | x | platform-facts §2.3 `min_width` | P | none |\n",
}

SELF_TEST_TRACES = {
    "macos": ("| MAC-B1 | `button.body.fill` | field |\n| MAC-B1 | `button.body.fill @ {hover}` | rule |\n"
              "| MAC-B11† | `button.body.min_width` | field |\n| MAC-G1 | - | out (R1.1) |\n"),
    "windows": "| W1-001 | `button.body.corners` | field |\n",
    "kde": "| K1.1 | - | readers (R18) |\n",
    "gnome": "| GN-1.01 | `token.text.body` | token |\n| GN-1.02 | `token.text.caption` | token |\n"
             "| GN-1.38 | `button.body.stroke @ cond contrast_high` | rule |\n",
    "material": "| MAT-B4 | `button.body.padding` | field |\n",
    "ios": "| B-1 | - | out (R1.1) |\n",
}

# (platform, line, field, the value wanted)
SELF_TEST_WANT = [
    ("macos", 16, "section", "1.1.3 Geometry"),
    ("macos", 16, "property", "Control corner radius"),
    ("macos", 16, "text", "measured | 5px | ✅ ([probe](evidence/os-probe/run-1/os-probe-macos-14/probe.toml#L7))"),
    ("macos", 16, "columns", "Source | Value | "),
    ("macos", 16, "evidence", ["docs/evidence/os-probe/run-1/os-probe-macos-14/probe.toml#L7"]),
    ("macos", 16, "rows", ["MAC-G1"]),
    ("macos", 16, "verdicts", ["MAC-G1: out (R1.1)"]),
    ("macos", 16, "note", "its rows' trace lines are verdicts only"),
    ("gnome", 24, "text", "Cantarell 11 | Adwaita Sans 11"),
    ("gnome", 24, "targets", ["token.text.body"]),
    ("macos", 32, "property", "background_color"),
    ("macos", 32, "text", "`controlColor`"),
    ("macos", 32, "rows", ["MAC-B1"]),
    ("macos", 32, "targets", ["button.body.fill", "button.body.fill @ {hover}"]),
    ("windows", 32, "rows", []),
    ("windows", 32, "note", "no gap-map row names this cell"),
    ("macos", 33, "rows", ["MAC-G1"]),
    ("macos", 33, "named_at", ["todo_v0.7.1_model-completeness-gap-map-macos.md:5"]),
    ("windows", 33, "targets", ["button.body.corners"]),
    ("material", 33, "none_cell", "a: as the cell"),
    ("material", 33, "targets", ["button.body.padding"]),
    ("material", 34, "targets", ["button.body.padding"]),
    ("macos", 34, "rows", ["MAC-B11†"]),
    ("macos", 34, "named_at", ["todo_v0.7.1_model-completeness-gap-map-macos.md:4",
                               "todo_v0.7.1_model-completeness-gap-map-macos.md:12"]),
    ("macos", 34, "none_cell", "b: MAC-B11†."),
    ("macos", 34, "targets", ["button.body.min_width"]),
    ("kde", 34, "rows", []),
    ("kde", 34, "none_cell", "a: sizes to content; MAC-B11† is macOS's (K1.1)"),
    ("gnome", 40, "context", "What it is: small text"),
    ("gnome", 40, "targets", ["token.text.caption"]),
    ("windows", 40, "text", "12px"),
]


def self_test() -> int:
    failures = []
    with tempfile.TemporaryDirectory() as tmp:
        base = Path(tmp)

        def run(facts=SELF_TEST_FACTS, maps=None):
            root = base / f"case{run.n}"
            run.n += 1
            (root / DOCS).mkdir(parents=True)
            (root / FACTS_MD).write_text(facts, encoding="utf-8")
            (root / MAP).write_text(SELF_TEST_MAP, encoding="utf-8")
            for p in PLATFORMS:
                gm = (maps or SELF_TEST_GAP_MAPS)[p]
                (root / DOCS / f"todo_v0.7.1_model-completeness-gap-map-{p}.md").write_text(gm, encoding="utf-8")
                (root / DOCS / f"todo_v0.7.1_model-completeness-spec-trace-{p}.md").write_text(
                    "| Row | Target | Kind |\n|---|---|---|\n" + SELF_TEST_TRACES[p], encoding="utf-8")
            return root, build(root)
        run.n = 0

        root, (out, counts) = run()
        if sorted(out) != ["gnome", "kde", "macos", "material", "windows"]:
            failures.append(f"files: got {sorted(out)}")
        data = {p: tomllib.loads(t)["row"] for p, t in out.items()}
        by_line = {(p, r["line"]): r for p, rows in data.items() for r in rows}
        for platform, line, key, want in SELF_TEST_WANT:
            got = by_line.get((platform, f"{CITE}:{line}"), {}).get(key)
            if got != want:
                failures.append(f"{platform} line {line} `{key}`: got {got!r}, want {want!r}")
        lines = {p: [r["line"] for r in rows] for p, rows in data.items()}
        if lines["macos"] != [f"{CITE}:{n}" for n in (16, 32, 33, 34, 40)]:
            failures.append(f"macos lines (Chapter 1 of another OS, the conventions or the appendix read?): "
                            f"{lines['macos']}")
        if counts != {"macos": (5, 2, 0, 0), "windows": (4, 1, 1, 0), "kde": (4, 0, 0, 1), "gnome": (5, 2, 0, 0),
                      "material": (4, 2, 0, 0)}:
            failures.append(f"counts: got {counts}")
        if write(root, out, check=False) != [f"{OUT.as_posix()}/{p}.toml: missing" for p in out]:
            failures.append("first write: not every file missing")
        if write(root, out, check=True):
            failures.append(f"check after write: got {write(root, out, check=True)}")
        (root / OUT / "macos.toml").write_text(out["macos"].replace("5px", "6px"), encoding="utf-8")
        (root / OUT / "ios.toml").write_text("", encoding="utf-8")
        got = write(root, out, check=True)
        if got != [f"{OUT.as_posix()}/macos.toml: differs from a fresh run",
                   f"{OUT.as_posix()}/ios.toml: not written by a fresh run"]:
            failures.append(f"check after an edit: got {got}")
        bad_cases = [
            ("a row with a cell too many", SELF_TEST_FACTS.replace("| 64 |", "| 64 | 65 |"), None),
            ("another commit", None, "fc6109d1:34", "caeddd6b:34"),
            ("a table without a platform column", SELF_TEST_FACTS.replace(
                "| Property | macOS | Windows | KDE | GNOME | Material |",
                "| Property | Mac | Win | Plasma | Gnome | M3 |"), None),
            ("a Chapter 1 section of no platform", SELF_TEST_FACTS.replace("### 1.4 GNOME", "### 1.4 Haiku"), None),
        ]
        for case in bad_cases:
            label = case[0]
            if len(case) == 3:
                facts, maps = case[1], None
            else:
                facts = SELF_TEST_FACTS
                maps = dict(SELF_TEST_GAP_MAPS)
                maps["macos"] = maps["macos"].replace(case[2], case[3])
            try:
                run(facts, maps)
                failures.append(f"{label}: accepted")
            except InputError:
                pass
        if toml_string('a"b\\c\u0001') != '"a\\"b\\\\c\\u0001"':
            failures.append(f"toml_string: got {toml_string(chr(97) + chr(34))}")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print(f"self-test ok ({len(SELF_TEST_WANT)} fields, 4 refusals)")
    return 1 if failures else 0


def main(argv: list[str]) -> int:
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    if args not in ([], ["--check"]):
        print(__doc__, file=sys.stderr)
        return 2
    check = args == ["--check"]
    root = Path(".")
    try:
        out, counts = build(root)
        diffs = write(root, out, check)
    except (OSError, UnicodeDecodeError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for platform, (n, with_target, unresolved, section_only) in counts.items():
        print(f"{platform}: {n} rows, {with_target} with targets, {n - with_target} without; gap-map references: "
              f"{unresolved} named no cell, {section_only} a section only")
    if check:
        for d in diffs:
            print(d)
        return 1 if diffs else 0
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
