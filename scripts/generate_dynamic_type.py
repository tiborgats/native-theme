#!/usr/bin/env python3
"""`dynamic_type`'s table and vector table, generated from the HIG's typography snapshot (specification S9.4's
`DynamicType`, ADD-A88, R25.15). Offline, given the source cache.

Reads `hig-typography@2026-02-09`'s `typography.json` from the source cache (`NATIVE_THEME_SOURCES`, else
scripts/citations.py's default; the file's sha256 checked against its `docs/facts/sources.toml` entry) and writes:

- `native-theme/src/platform/gen_dynamic_type.rs`: `ROLES`, the text roles of the tables' rows in their order;
  `DEFAULT`, the category of the tab the page marks `(default)`; `TABLES`, each category's table, smallest first,
  each row `(size, leading)` in points, each table with its JSON pointer; through `rustfmt`;
- `native-theme/tests/vectors/dynamic_type.tsv`: every row of the twelve tables as a vector row in
  scripts/check_function_vectors.py's format, route `published`: the role and the category as the model spells them,
  the DPI 72 (S9.4's `token.text.font_dpi` on `ios-*`, where a point is a px) as the operand `font_dpi`, the size and
  the leading in px; each row's `cite` its row's JSON pointer and its `quote` the row's five cells as the page writes
  them, joined by a space.

Run from the repository root:

    python3 scripts/generate_dynamic_type.py              # writes both files
    python3 scripts/generate_dynamic_type.py --check      # check_release.sh's step: writes nothing
    python3 scripts/generate_dynamic_type.py --self-test

`--check` prints each file that differs from what the snapshot gives and exits 1 when there is one. Every mode exits
2 on an input it cannot read or a page whose tables are not as below (it says what), and on a wrong command line.

The tables (S9.4)
-----------------
Twelve tabs: the seven after *iOS, iPadOS Dynamic Type sizes* (`xSmall` … `xxxLarge`, `Large (default)` among
them) and the five after *iOS, iPadOS larger accessibility type sizes* (`AX1` … `AX5`). Each tab holds one table
whose header row is `Style`, `Weight`, `Size (points)`, `Leading (points)`, `Emphasized weight` and whose rows are
the eleven styles in STYLES' order, each size and leading a whole number of points. A tab is UIKit's
`UIContentSizeCategory` of CATEGORIES: the seven standard ones by name, `AX1` … `AX5` the five accessibility
categories in order, smallest first, as UIKit's `preferredFont(forTextStyle:compatibleWith:)` returns them per
category: the script checks every table's sizes against the sizes the iOS host measured per category (EVIDENCE, each
`[[text_style]]` of the regular legibility weight) and refuses a page or an evidence file where they differ.
"""
import hashlib
import json
import subprocess
import sys
import tempfile
import textwrap
import tomllib
from pathlib import Path

import citations

SOURCE = "hig-typography@2026-02-09"
FILE = "typography.json"
SOURCES = Path("docs/facts/sources.toml")
RUST = Path("native-theme/src/platform/gen_dynamic_type.rs")
VECTORS = Path("native-theme/tests/vectors/dynamic_type.tsv")
HEADER = ["Style", "Weight", "Size (points)", "Leading (points)", "Emphasized weight"]
STANDARD = "iOS, iPadOS Dynamic Type sizes"
ACCESSIBILITY = "iOS, iPadOS larger accessibility type sizes"
# (the tab's title, `ContentSizeCategory` as the model spells it, its Rust variant), smallest first
CATEGORIES = (("xSmall", "extra_small", "ExtraSmall"), ("Small", "small", "Small"), ("Medium", "medium", "Medium"),
              ("Large (default)", "large", "Large"), ("xLarge", "extra_large", "ExtraLarge"),
              ("xxLarge", "extra_extra_large", "ExtraExtraLarge"),
              ("xxxLarge", "extra_extra_extra_large", "ExtraExtraExtraLarge"),
              ("AX1", "accessibility_medium", "AccessibilityMedium"),
              ("AX2", "accessibility_large", "AccessibilityLarge"),
              ("AX3", "accessibility_extra_large", "AccessibilityExtraLarge"),
              ("AX4", "accessibility_extra_extra_large", "AccessibilityExtraExtraLarge"),
              ("AX5", "accessibility_extra_extra_extra_large", "AccessibilityExtraExtraExtraLarge"))
# (the table's style, `TextRole` as the model spells it, its Rust variant, the style the evidence names), in the
# tables' order
STYLES = (("Large Title", "large_title", "LargeTitle", "large_title"), ("Title 1", "title1", "Title1", "title1"),
          ("Title 2", "title2", "Title2", "title2"), ("Title 3", "title3", "Title3", "title3"),
          ("Headline", "headline", "Headline", "headline"), ("Body", "body", "Body", "body"),
          ("Callout", "callout", "Callout", "callout"), ("Subhead", "subheadline", "Subheadline", "subheadline"),
          ("Footnote", "footnote", "Footnote", "footnote"), ("Caption 1", "caption", "Caption", "caption1"),
          ("Caption 2", "caption2", "Caption2", "caption2"))
DEFAULT_MARK = " (default)"
# the iOS host's runs, one per generation: each `[[text_style]]` the font UIKit returned for a style and category
EVIDENCE = (Path("docs/evidence/ios-host/2026-10-30-run-41020880059/ios-host-macos-15/run1/index.toml"),
            Path("docs/evidence/ios-host/2026-10-30-run-41020880059/ios-host-macos-26/run1/index.toml"))
DPI = 72


class InputError(Exception):
    pass


def text_of(node) -> str:
    """The text of a DocC node: its `text` runs, in order."""
    if isinstance(node, dict):
        own = node.get("text", "") if node.get("type") == "text" else ""
        return own + "".join(text_of(v) for k, v in node.items() if k != "text")
    if isinstance(node, list):
        return "".join(text_of(v) for v in node)
    return ""


def navigator_after(content: list, heading: str) -> tuple:
    """(index, node) of the tab navigator right after the heading `heading`."""
    starts = [i for i, n in enumerate(content) if n.get("type") == "heading" and n.get("text") == heading]
    if len(starts) != 1:
        raise InputError(f"{len(starts)} headings `{heading}`, not one")
    i = starts[0] + 1
    if i >= len(content) or content[i].get("type") != "tabNavigator":
        raise InputError(f"no tab navigator right after `{heading}`")
    return i, content[i]


def rows_of(table: dict, pointer: str) -> list:
    """[(size, leading, [the row's five cells], the row's pointer)] of a category's table, in STYLES' order."""
    if table.get("type") != "table" or not isinstance(table.get("rows"), list):
        raise InputError(f"{pointer} is no table")
    cells = [[text_of(c).strip() for c in row] for row in table["rows"]]
    if not cells or cells[0] != HEADER:
        raise InputError(f"{pointer}: header row {cells[:1]}, not {HEADER}")
    if [row[0] for row in cells[1:]] != [style for style, _, _, _ in STYLES]:
        raise InputError(f"{pointer}: styles {[row[0] for row in cells[1:]]}, not {[s for s, _, _, _ in STYLES]}")
    out = []
    for n, row in enumerate(cells[1:], 1):
        if len(row) != len(HEADER):
            raise InputError(f"{pointer}/rows/{n}: {len(row)} cells")
        if not row[2].isdigit() or not row[3].isdigit():
            raise InputError(f"{pointer}/rows/{n}: {row} has no whole size and leading")
        out.append((int(row[2]), int(row[3]), row, f"{pointer}/rows/{n}"))
    return out


def tables(doc) -> list:
    """[(the category's Rust variant, its spelling, whether it is the default, the table's pointer, its rows)],
    smallest first."""
    try:
        content = doc["primaryContentSections"][0]["content"]
    except (KeyError, IndexError, TypeError) as err:
        raise InputError(f"no #/primaryContentSections/0/content: {err}") from err
    base = "#/primaryContentSections/0/content"
    tabs = []
    for heading in (STANDARD, ACCESSIBILITY):
        i, nav = navigator_after(content, heading)
        for k, tab in enumerate(nav.get("tabs", [])):
            found = [(m, n) for m, n in enumerate(tab.get("content", [])) if n.get("type") == "table"]
            if len(found) != 1:
                raise InputError(f"{base}/{i}/tabs/{k}: {len(found)} tables, not one")
            m, table = found[0]
            pointer = f"{base}/{i}/tabs/{k}/content/{m}"
            tabs.append((tab.get("title"), pointer, rows_of(table, pointer)))
    titles = [title for title, _, _ in tabs]
    if titles != [title for title, _, _ in CATEGORIES]:
        raise InputError(f"the tabs {titles}, not {[t for t, _, _ in CATEGORIES]}")
    defaults = [title for title in titles if title.endswith(DEFAULT_MARK)]
    if len(defaults) != 1:
        raise InputError(f"{len(defaults)} tabs marked `{DEFAULT_MARK.strip()}`, not one")
    return [(variant, spelled, title in defaults, pointer, rows)
            for (title, pointer, rows), (_, spelled, variant) in zip(tabs, CATEGORIES)]


def check_evidence(found: list, evidence: dict) -> None:
    """Each table's sizes are the sizes UIKit returned per category in each evidence file ({path: parsed})."""
    for path, data in evidence.items():
        measured = {}
        for entry in data.get("text_style", []):
            if entry.get("legibility_weight") == "regular":
                measured[(entry.get("category"), entry.get("style"))] = entry.get("font", {}).get("size_pt")
        for _, category, _, pointer, rows in found:
            for (_, _, _, style), (size, _, _, _) in zip(STYLES, rows):
                got = measured.get((category, style))
                if got != size:
                    raise InputError(f"{path}: {style} at {category} measured {got}, the table {pointer} {size}")


def rust_file(found: list) -> str:
    out = [f"// Generated by scripts/generate_dynamic_type.py from {SOURCE}'s {FILE}; do not",
           "// edit by hand: run the generator.", "",
           "//! `dynamic_type`'s table (S9.4): each content size category's size and",
           "//! leading of each text style, in points, from the HIG's *iOS, iPadOS Dynamic",
           "//! Type sizes* and *iOS, iPadOS larger accessibility type sizes*.", "",
           "use crate::blocks::text::TextRole;", "use crate::resolve::ContentSizeCategory;", "",
           *textwrap.wrap("The text roles of the tables' rows, in their order (the *Style* column: "
                          + ", ".join(style for style, _, _, _ in STYLES) + ").", width=76,
                          initial_indent="/// ", subsequent_indent="/// "),
           f"pub(super) const ROLES: [TextRole; {len(STYLES)}] = ["]
    out += [f"    TextRole::{variant}," for _, _, variant, _ in STYLES]
    default = next(variant for variant, _, is_default, _, _ in found if is_default)
    out += ["];", "", "/// The default category, the tab the page marks *(default)*.",
            f"pub(super) const DEFAULT: ContentSizeCategory = ContentSizeCategory::{default};", "",
            "/// Each category's table, smallest first: each row `(size, leading)` in",
            "/// points, in [`ROLES`]' order.",
            f"pub(super) const TABLES: [(ContentSizeCategory, [(f32, f32); {len(STYLES)}]); {len(found)}] = ["]
    for variant, _, _, pointer, rows in found:
        out.append(f"    // `{SOURCE} {pointer}`")
        out.append(f"    (ContentSizeCategory::{variant}, [")
        out += [f"        ({size}.0, {leading}.0)," for size, leading, _, _ in rows]
        out.append("    ]),")
    out += ["];", ""]
    return "\n".join(out)


def rustfmt(code: str) -> str:
    try:
        run = subprocess.run(["rustfmt", "--edition", "2024", "--emit", "stdout"], input=code, text=True,
                             capture_output=True, check=False)
    except OSError as err:
        raise InputError(f"rustfmt: {err}") from err
    if run.returncode != 0:
        raise InputError(f"rustfmt refused the generated {RUST}:\n{run.stderr}")
    return run.stdout


def vectors_file(found: list) -> str:
    out = ["# function: dynamic_type", "# route: published", f"# source: {SOURCE}", "# generations: ios-26",
           "# inputs: role:enum<TextRole> category:enum<ContentSizeCategory> font_dpi:number",
           "# outputs: font_size:font_size line_height:font_size",
           f"# written by scripts/generate_dynamic_type.py from {SOURCE}'s {FILE}; do not edit.",
           f"# Every row of the twelve tables (the tabs after {STANDARD}",
           f"# and {ACCESSIBILITY}), the size and the leading in px at {DPI} DPI",
           "# (S9.4); the quote the row's five cells.",
           "role\tcategory\tfont_dpi\tfont_size\tline_height\tcite\tquote"]
    for _, category, _, _, rows in found:
        for (_, role, _, _), (size, leading, cells, pointer) in zip(STYLES, rows):
            out.append(f"{role}\t{category}\t{DPI}\t{size}px\t{leading}px\t{SOURCE} {pointer}\t{' '.join(cells)}")
    return "\n".join(out) + "\n"


def snapshot(root: Path, cache: Path) -> dict:
    try:
        sources = tomllib.loads((root / SOURCES).read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as err:
        raise InputError(f"{SOURCES}: {err}") from err
    entry = next((s for s in sources.get("source", []) if s.get("id") == SOURCE), None)
    if entry is None or entry.get("file") != FILE:
        raise InputError(f"{SOURCES}: no [[source]] `{SOURCE}` with file `{FILE}`")
    path = cache / SOURCE / FILE
    try:
        data = path.read_bytes()
    except OSError as err:
        raise InputError(f"{path}: {err}") from err
    if hashlib.sha256(data).hexdigest() != entry.get("sha256"):
        raise InputError(f"{path}: its sha256 is not {SOURCES}'s for {SOURCE}")
    try:
        return json.loads(data.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as err:
        raise InputError(f"{path}: {err}") from err


def evidence_files(root: Path) -> dict:
    out = {}
    for path in EVIDENCE:
        try:
            out[path] = tomllib.loads((root / path).read_text(encoding="utf-8"))
        except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as err:
            raise InputError(f"{path}: {err}") from err
    return out


def generate(doc, evidence: dict) -> dict:
    found = tables(doc)
    check_evidence(found, evidence)
    return {RUST: rustfmt(rust_file(found)), VECTORS: vectors_file(found)}


# ------------------------------------------------------------------------------------------------ self-test

def _cell(text):
    return [{"type": "paragraph", "inlineContent": [{"type": "text", "text": text}]}]


def _table(rows):
    return {"type": "table", "header": "row", "rows": [[_cell(c) for c in r] for r in [HEADER] + rows]}


def _rows(shift):
    """Synthetic rows: style n's size 10 + n + shift, its leading 4 more."""
    return [[style, "Regular", str(10 + n + shift), str(14 + n + shift), "Bold"]
            for n, (style, _, _, _) in enumerate(STYLES)]


def _page(titles=None, rows=None):
    titles = titles or [title for title, _, _ in CATEGORIES]
    tabs = [{"title": t, "content": [{"type": "heading", "text": t}, _table(rows or _rows(k))]}
            for k, t in enumerate(titles)]
    return {"primaryContentSections": [{"content": [
        {"type": "heading", "text": STANDARD}, {"type": "tabNavigator", "tabs": tabs[:7]},
        {"type": "heading", "text": ACCESSIBILITY}, {"type": "tabNavigator", "tabs": tabs[7:]}]}]}


def _evidence(shift_of=lambda k: k):
    return {"text_style": [
        {"style": ev, "category": spelled, "legibility_weight": weight, "font": {"size_pt": float(10 + n + shift_of(k))}}
        for k, (_, spelled, _) in enumerate(CATEGORIES) for n, (_, _, _, ev) in enumerate(STYLES)
        for weight in ("regular", "bold")]}


def self_test() -> int:
    failures = []
    found = tables(_page())
    if [(v, d) for v, _, d, _, _ in found][3] != ("Large", True) or any(d for _, _, d, _, _ in found[4:]):
        failures.append(f"the default: {[(v, d) for v, _, d, _, _ in found]}")
    if found[7][3] != "#/primaryContentSections/0/content/3/tabs/0/content/1":
        failures.append(f"AX1's pointer: {found[7][3]}")
    if [(s, lead) for s, lead, _, _ in found[0][4]][:2] != [(10, 14), (11, 15)]:
        failures.append(f"xSmall's rows: {found[0][4]}")
    rows = vectors_file(found).splitlines()
    want = ("title1\taccessibility_medium\t72\t18px\t22px\thig-typography@2026-02-09 "
            "#/primaryContentSections/0/content/3/tabs/0/content/1/rows/2\tTitle 1 Regular 18 22 Bold")
    if want not in rows or len([r for r in rows if not r.startswith("#")]) != 1 + 12 * len(STYLES):
        failures.append(f"vector rows: {rows[:14]}")
    rust = rust_file(found)
    if "DEFAULT: ContentSizeCategory = ContentSizeCategory::Large;" not in rust or "(10.0, 14.0)," not in rust:
        failures.append("the Rust table lacks the default or a row")
    try:
        check_evidence(found, {Path("e"): _evidence()})
    except InputError as err:
        failures.append(f"refuses evidence that agrees: {err}")
    renamed = [title for title, _, _ in CATEGORIES]
    renamed[3] = "Large"
    reordered = _rows(0)
    reordered[0], reordered[1] = reordered[1], reordered[0]
    refused = [
        ("a tab renamed", lambda: tables(_page(titles=renamed)), "the tabs"),
        ("styles in another order", lambda: tables(_page(rows=reordered)), "styles"),
        ("a fractional size", lambda: tables(_page(rows=[[r[0], r[1], "1.5", r[3], r[4]] for r in _rows(0)])),
         "whole size"),
        ("evidence that differs", lambda: check_evidence(found, {Path("e"): _evidence(lambda k: k + 1)}),
         "measured"),
    ]
    for what, run, needle in refused:
        try:
            run()
            failures.append(f"takes {what}")
        except InputError as err:
            if needle not in str(err):
                failures.append(f"refuses {what}, but as {err}")
    with tempfile.TemporaryDirectory() as tmp:
        root, cache = Path(tmp) / "repo", Path(tmp) / "cache"
        (cache / SOURCE).mkdir(parents=True)
        data = json.dumps(_page()).encode("utf-8")
        (cache / SOURCE / FILE).write_bytes(data)
        (root / SOURCES).parent.mkdir(parents=True)
        (root / SOURCES).write_text(f'[[source]]\nid = "{SOURCE}"\nfile = "{FILE}"\nsha256 = "{"0" * 64}"\n',
                                    encoding="utf-8")
        try:
            snapshot(root, cache)
            failures.append("takes a snapshot whose sha256 is not its entry's")
        except InputError:
            pass
        (root / SOURCES).write_text(f'[[source]]\nid = "{SOURCE}"\nfile = "{FILE}"\n'
                                    f'sha256 = "{hashlib.sha256(data).hexdigest()}"\n', encoding="utf-8")
        if snapshot(root, cache) != _page():
            failures.append("reads the snapshot otherwise")
    for f in failures:
        print(f"self-test FAIL: {f}", file=sys.stderr)
    if not failures:
        print("self-test ok", file=sys.stderr)
    return 1 if failures else 0


def main(argv) -> int:
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    if args not in ([], ["--check"]):
        print(__doc__, file=sys.stderr)
        return 2
    root = Path(".")
    try:
        files = generate(snapshot(root, citations.cache_dir()), evidence_files(root))
    except InputError as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    if args == ["--check"]:
        stale = [p for p, text in files.items()
                 if not (root / p).exists() or (root / p).read_text(encoding="utf-8") != text]
        for p in stale:
            print(f"{p}: differs from what {SOURCE} gives (run scripts/generate_dynamic_type.py)", file=sys.stderr)
        return 1 if stale else 0
    for p, text in files.items():
        (root / p).write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
