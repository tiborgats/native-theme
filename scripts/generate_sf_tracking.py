#!/usr/bin/env python3
"""`sf_tracking`'s tables and vector table, generated from the HIG's typography snapshots (specification S2.2's
`sf_tracking`, ADD-A54, ADD-A136; ledger queue #154, #176). Offline, given the source cache.

Reads SNAPSHOTS' `typography.json`, the HIG's Typography page from macOS 26's and iOS 26's lifetime
(`hig-typography@2026-02-09`) and from macOS 15's and iOS 18's (`hig-typography@2025-08-15`), from the source cache
(`NATIVE_THEME_SOURCES`, else scripts/citations.py's default; each file's sha256 checked against its
`docs/facts/sources.toml` entry) and writes:

- `native-theme/src/platform/gen_sf_tracking.rs`: the three designs' tables, `SF_PRO`, `SF_PRO_ROUNDED` and
  `NEW_YORK`, each row `(size in points, tracking in 1/1000 em)`, each table with its JSON pointer in each snapshot,
  through `rustfmt`;
- `native-theme/tests/vectors/sf_tracking.tsv`: every row of the three tables as a vector row in
  scripts/check_function_vectors.py's format, route `published`: the size as `font_size` in px at 72 DPI (S9.4's
  `token.text.font_dpi` on `macos-*`, where a point is a px), the DPI as the operand `font_dpi` the function reads
  as `round_pt` does, and the tracking `t · size / 1000` px, written exactly; each row's `cite` its row's JSON
  pointer in each snapshot, separated by ` ; `, and its `quote` the row's three cells as the pages write them,
  joined by a space; `# generations:` the snapshots' entries' generations.

ADD-A136: the snapshots' tables must be equal row by row (each size and its 1/1000 em tracking, and the cells
quoted), so each row cites both snapshots and serves the generations of both. Where one differs, S2.2 has the
function select its table by an enum operand naming the release (a class 2 change made then): the script refuses
the pages and says which row differs; it does not choose one.

Run from the repository root:

    python3 scripts/generate_sf_tracking.py              # writes both files
    python3 scripts/generate_sf_tracking.py --check      # check_release.sh's step: writes nothing
    python3 scripts/generate_sf_tracking.py --self-test

`--check` prints each file that differs from what the snapshots give and exits 1 when there is one. Every mode exits
2 on an input it cannot read or a page whose tables are not as below (it says what), and on a wrong command line.

The tables (S2.2)
-----------------
SF Pro's is *macOS tracking values* (the table after that heading); SF Pro Rounded's and New York's are the tabs of
those names after *iOS, iPadOS, visionOS tracking values*. Each table's header row is `Size (points)`, `Tracking
(1/1000 em)`, `Tracking (points)`; a size is a whole number of points, a tracking in 1/1000 em a whole number with
its sign (`+41`, `0`, `-6`), the sizes strictly increasing. The function reads the 1/1000 em column: the points column
is that value times the size rounded to two places, which this script does not read. S2.2 states that the iOS tab
*SF Pro*'s 1/1000 em column equals the macOS table's: the script checks it, row by row, and refuses the page where
it does not.
"""
import hashlib
import json
import subprocess
import sys
import tempfile
import tomllib
from decimal import Decimal
from pathlib import Path

import citations

# the snapshots, the first the one the Rust tables' values are read from (the others equal it)
SNAPSHOTS = ("hig-typography@2026-02-09", "hig-typography@2025-08-15")
FILE = "typography.json"
SOURCES = Path("docs/facts/sources.toml")
RUST = Path("native-theme/src/platform/gen_sf_tracking.rs")
VECTORS = Path("native-theme/tests/vectors/sf_tracking.tsv")
HEADER = ["Size (points)", "Tracking (1/1000 em)", "Tracking (points)"]
MACOS = "macOS tracking values"
IOS = "iOS, iPadOS, visionOS tracking values"
# (Rust constant, `TrackingDesign` as expressions spell it, the iOS tab's title)
DESIGNS = (("SF_PRO", "sf_pro", "SF Pro"), ("SF_PRO_ROUNDED", "sf_pro_rounded", "SF Pro Rounded"),
           ("NEW_YORK", "new_york", "New York"))
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


def table_after(content: list, heading: str) -> tuple:
    """(index, node) of the first node after the heading `heading` that is a table or a tab navigator."""
    starts = [i for i, n in enumerate(content) if n.get("type") == "heading" and n.get("text") == heading]
    if len(starts) != 1:
        raise InputError(f"{len(starts)} headings `{heading}`, not one")
    for i in range(starts[0] + 1, len(content)):
        if content[i].get("type") in ("table", "tabNavigator"):
            return i, content[i]
        if content[i].get("type") == "heading":
            break
    raise InputError(f"no table after `{heading}`")


def rows_of(table: dict, pointer: str) -> list:
    """[(size, tracking in 1/1000 em, [the row's three cells], the row's pointer)] of a tracking table."""
    if table.get("type") != "table" or not isinstance(table.get("rows"), list):
        raise InputError(f"{pointer} is no table")
    cells = [[text_of(c).strip() for c in row] for row in table["rows"]]
    if not cells or cells[0] != HEADER:
        raise InputError(f"{pointer}: header row {cells[:1]}, not {HEADER}")
    out = []
    for n, row in enumerate(cells[1:], 1):
        if len(row) != 3:
            raise InputError(f"{pointer}/rows/{n}: {len(row)} cells")
        size, per_mille = row[0], row[1].removeprefix("+")
        if not size.isdigit() or not per_mille.removeprefix("-").isdigit():
            raise InputError(f"{pointer}/rows/{n}: {row} is not a whole size and a whole signed tracking")
        if out and int(size) <= out[-1][0]:
            raise InputError(f"{pointer}/rows/{n}: size {size} after {out[-1][0]}")
        out.append((int(size), int(per_mille), row, f"{pointer}/rows/{n}"))
    if not out:
        raise InputError(f"{pointer}: no row")
    return out


def tables(doc) -> dict:
    """{Rust constant: (the table's pointer, its rows)} from the page."""
    try:
        content = doc["primaryContentSections"][0]["content"]
    except (KeyError, IndexError, TypeError) as err:
        raise InputError(f"no #/primaryContentSections/0/content: {err}") from err
    base = "#/primaryContentSections/0/content"
    i, mac = table_after(content, MACOS)
    out = {"SF_PRO": (f"{base}/{i}", rows_of(mac, f"{base}/{i}"))}
    j, nav = table_after(content, IOS)
    if nav.get("type") != "tabNavigator":
        raise InputError(f"{base}/{j}: no tab navigator after `{IOS}`")
    tabs = {}
    for k, tab in enumerate(nav.get("tabs", [])):
        found = [(m, n) for m, n in enumerate(tab.get("content", [])) if n.get("type") == "table"]
        if len(found) != 1:
            raise InputError(f"{base}/{j}/tabs/{k}: {len(found)} tables, not one")
        m, n = found[0]
        pointer = f"{base}/{j}/tabs/{k}/content/{m}"
        tabs[tab.get("title")] = (pointer, rows_of(n, pointer))
    for const, _, title in DESIGNS:
        if title not in tabs:
            raise InputError(f"{base}/{j}: no tab `{title}`")
        if const != "SF_PRO":
            out[const] = tabs[title]
    ios = [(s, t) for s, t, _, _ in tabs["SF Pro"][1]]
    macos = [(s, t) for s, t, _, _ in out["SF_PRO"][1]]
    if ios != macos:
        diff = [(a, b) for a, b in zip(ios, macos) if a != b] or f"{len(ios)} rows against {len(macos)}"
        raise InputError(f"the iOS tab SF Pro's 1/1000 em column is not the macOS table's (S2.2): {diff}")
    return out


def merge(found: dict) -> dict:
    """{Rust constant: ([the table's pointer in each snapshot], [(size, tracking, [the quoted cells], [the row's
    pointer in each snapshot])])} from {snapshot id: tables()}, the snapshots in SNAPSHOTS' order; refuses tables
    that differ (ADD-A136: S2.2's enum operand would then select one)."""
    ids = list(found)
    first = found[ids[0]]
    out = {}
    for const, _, title in DESIGNS:
        rows = []
        for sid in ids[1:]:
            mine, theirs = first[const][1], found[sid][const][1]
            a, b = [(s, t) for s, t, _, _ in mine], [(s, t) for s, t, _, _ in theirs]
            if a != b:
                diff = [(x, y) for x, y in zip(a, b) if x != y] or f"{len(b)} rows against {len(a)}"
                raise InputError(f"{title}'s table in {sid} differs from {ids[0]}'s: {diff} (ADD-A136: S2.2 then "
                                 "selects the table by an enum operand naming the release; not generated)")
            for (_, _, c1, _), (_, _, c2, ptr) in zip(mine, theirs):
                if c1 != c2:
                    raise InputError(f"{title}'s row {ptr} in {sid} is written {c2}, in {ids[0]} {c1}: no quote "
                                     "both hold")
        for n, (size, per_mille, cells, _) in enumerate(first[const][1]):
            rows.append((size, per_mille, cells, [found[sid][const][1][n][3] for sid in ids]))
        out[const] = ([found[sid][const][0] for sid in ids], rows)
    return out


def cited(ids: list, pointers: list) -> str:
    return " ; ".join(f"{sid} {pointer}" for sid, pointer in zip(ids, pointers))


def rust_file(merged: dict, ids: list) -> str:
    out = [f"// Generated by scripts/generate_sf_tracking.py from the {FILE} of",
           f"// {' and '.join(ids)}, whose tables are", "// equal; do not edit by hand: run the generator.", "",
           "//! `sf_tracking`'s tables (S2.2): each row a size in points and its tracking",
           "//! in 1/1000 em, the sizes increasing.", ""]
    for const, _, title in DESIGNS:
        pointers, rows = merged[const]
        what = f"*{MACOS}*" if const == "SF_PRO" else f"*{IOS}*, the tab *{title}*"
        out += [f"/// {title}'s tracking: the HIG's {what}"]
        out += [f"/// {'(' if k == 0 else ''}`{sid} {pointer}`{').' if k == len(ids) - 1 else ','}"
                for k, (sid, pointer) in enumerate(zip(ids, pointers))]
        out += [f"pub(super) const {const}: [(f32, f32); {len(rows)}] = ["]
        out += [f"    ({size}.0, {per_mille}.0)," for size, per_mille, _, _ in rows]
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


def vectors_file(merged: dict, ids: list, generations: list) -> str:
    out = ["# function: sf_tracking", "# route: published", *[f"# source: {sid}" for sid in ids],
           f"# generations: {', '.join(generations)}",
           "# inputs: font_size:font_size design:enum<TrackingDesign> font_dpi:number", "# outputs: tracking:length",
           f"# written by scripts/generate_sf_tracking.py from {' and '.join(ids)}'s {FILE}; do not edit.",
           f"# Every row of the three tables (SF Pro: {MACOS}; SF Pro Rounded and New York: the tabs",
           f"# after {IOS}), the size in px at {DPI} DPI (S9.4), the tracking t * size / 1000 px",
           "# from the 1/1000 em column t; each row cites both snapshots, whose tables are equal (ADD-A136);",
           "# the quote the row's three cells.",
           "font_size\tdesign\tfont_dpi\ttracking\tcite\tquote"]
    for const, design, _ in DESIGNS:
        for size, per_mille, cells, pointers in merged[const][1]:
            tracking = Decimal(per_mille) * Decimal(size) / Decimal(1000)
            out.append(f"{size}px\t{design}\t{DPI}\t{format(tracking.normalize(), 'f')}px\t{cited(ids, pointers)}\t"
                       f"{' '.join(cells)}")
    return "\n".join(out) + "\n"


def snapshot(root: Path, cache: Path, sid: str) -> tuple:
    """(the parsed page, its entry's generations) of the snapshot `sid`."""
    try:
        sources = tomllib.loads((root / SOURCES).read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as err:
        raise InputError(f"{SOURCES}: {err}") from err
    entry = next((s for s in sources.get("source", []) if s.get("id") == sid), None)
    if entry is None or entry.get("file") != FILE:
        raise InputError(f"{SOURCES}: no [[source]] `{sid}` with file `{FILE}`")
    gens = entry.get("generations")
    if not isinstance(gens, list) or not gens or not all(isinstance(g, str) for g in gens):
        raise InputError(f"{SOURCES}: `{sid}` serves no generation")
    path = cache / sid / FILE
    try:
        data = path.read_bytes()
    except OSError as err:
        raise InputError(f"{path}: {err}") from err
    if hashlib.sha256(data).hexdigest() != entry.get("sha256"):
        raise InputError(f"{path}: its sha256 is not {SOURCES}'s for {sid}")
    try:
        return json.loads(data.decode("utf-8")), gens
    except (UnicodeDecodeError, json.JSONDecodeError) as err:
        raise InputError(f"{path}: {err}") from err


def generate(snapshots: dict) -> dict:
    """The files from {snapshot id: (page, generations)}, in SNAPSHOTS' order."""
    ids = list(snapshots)
    merged = merge({sid: tables(doc) for sid, (doc, _) in snapshots.items()})
    generations = [g for sid in ids for g in snapshots[sid][1]]
    return {RUST: rustfmt(rust_file(merged, ids)), VECTORS: vectors_file(merged, ids, generations)}


# ------------------------------------------------------------------------------------------------ self-test

def _cell(text):
    return [{"type": "paragraph", "inlineContent": [{"type": "text", "text": text}]}]


def _table(rows):
    return {"type": "table", "header": "row", "rows": [[_cell(c) for c in r] for r in [HEADER] + rows]}


def _page(mac_rows, ios_rows, rounded_rows):
    tabs = [{"title": t, "content": [{"type": "heading", "text": t}, _table(r)]}
            for t, r in (("SF Pro", ios_rows), ("SF Pro Rounded", rounded_rows), ("New York", rounded_rows))]
    return {"primaryContentSections": [{"content": [
        {"type": "heading", "text": IOS}, {"type": "tabNavigator", "tabs": tabs},
        {"type": "heading", "text": MACOS}, _table(mac_rows)]}]}


MAC = [["6", "+41", "+0.24"], ["12", "0", "0.0"], ["13", "-6", "-0.08"]]


def self_test() -> int:
    failures = []
    new, old = SNAPSHOTS
    found = tables(_page(MAC, MAC, [["6", "+87", "+0.51"], ["8", "+72", "+0.57"]]))
    if [(s, t) for s, t, _, _ in found["SF_PRO"][1]] != [(6, 41), (12, 0), (13, -6)]:
        failures.append(f"SF Pro's rows: {found['SF_PRO'][1]}")
    if found["SF_PRO"][0] != "#/primaryContentSections/0/content/3" or \
            found["NEW_YORK"][0] != "#/primaryContentSections/0/content/1/tabs/2/content/1":
        failures.append(f"pointers: {found['SF_PRO'][0]}, {found['NEW_YORK'][0]}")
    # the older page one node later: a note before the iOS heading
    later = _page(MAC, MAC, [["6", "+87", "+0.51"], ["8", "+72", "+0.57"]])
    later["primaryContentSections"][0]["content"].insert(0, {"type": "paragraph", "inlineContent": []})
    merged = merge({new: found, old: tables(later)})
    rows = vectors_file(merged, [new, old], ["macos-tahoe", "ios-26", "macos-monterey", "ios-13"]).splitlines()
    want = (f"6px\tsf_pro\t72\t0.246px\t{new} #/primaryContentSections/0/content/3/rows/1 ; "
            f"{old} #/primaryContentSections/0/content/4/rows/1\t6 +41 +0.24")
    if want not in rows or "13px\tsf_pro\t72\t-0.078px" not in "\n".join(rows) or \
            "12px\tsf_pro\t72\t0px" not in "\n".join(rows) or \
            "# generations: macos-tahoe, ios-26, macos-monterey, ios-13" not in rows or \
            f"# source: {old}" not in rows:
        failures.append(f"vector rows: {rows}")
    rust = rust_file(merged, [new, old])
    if rust.count("(6.0, 41.0),") != 1 or "(13.0, -6.0)," not in rust or \
            f"/// `{old} #/primaryContentSections/0/content/4`)." not in rust:
        failures.append(f"the Rust tables lack a row or a pointer: {rust}")
    renamed = _page(MAC, MAC, MAC)
    renamed["primaryContentSections"][0]["content"][3]["rows"][0] = [_cell("Size"), _cell("x"), _cell("y")]
    refused = [
        ("the iOS SF Pro column differing", lambda: tables(_page(MAC, [["6", "+40", "+0.24"]] + MAC[1:], MAC)),
         "not the macOS"),
        ("a size not increasing", lambda: tables(_page([MAC[1], MAC[0]], [MAC[1], MAC[0]], MAC)), "after"),
        ("a fractional tracking", lambda: tables(_page([["6", "+4.1", "+0.24"]], [["6", "+4.1", "+0.24"]], MAC)),
         "whole"),
        ("another header row", lambda: tables(renamed), "header row"),
        ("the snapshots' trackings differing", lambda: merge({new: tables(_page(MAC, MAC, MAC)), old: tables(
            _page(MAC, MAC, [["6", "+40", "+0.24"]] + MAC[1:]))}), "enum operand"),
        ("the snapshots' sizes differing", lambda: merge({new: tables(_page(MAC, MAC, MAC)), old: tables(
            _page(MAC[:2], MAC[:2], MAC))}), "differs"),
        ("a quoted cell written otherwise", lambda: merge({new: tables(_page(MAC, MAC, MAC)), old: tables(
            _page([["6", "+41", "+0.25"]] + MAC[1:], [["6", "+41", "+0.25"]] + MAC[1:], MAC))}), "no quote"),
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
        (cache / new).mkdir(parents=True)
        data = json.dumps(_page(MAC, MAC, MAC)).encode("utf-8")
        (cache / new / FILE).write_bytes(data)
        (root / SOURCES).parent.mkdir(parents=True)
        entry = f'[[source]]\nid = "{new}"\nfile = "{FILE}"\ngenerations = ["macos-tahoe"]\n'
        (root / SOURCES).write_text(entry + f'sha256 = "{"0" * 64}"\n', encoding="utf-8")
        try:
            snapshot(root, cache, new)
            failures.append("takes a snapshot whose sha256 is not its entry's")
        except InputError:
            pass
        (root / SOURCES).write_text(entry + f'sha256 = "{hashlib.sha256(data).hexdigest()}"\n', encoding="utf-8")
        if snapshot(root, cache, new) != (_page(MAC, MAC, MAC), ["macos-tahoe"]):
            failures.append("reads the snapshot otherwise")
        (root / SOURCES).write_text(entry.replace('["macos-tahoe"]', "[]")
                                    + f'sha256 = "{hashlib.sha256(data).hexdigest()}"\n', encoding="utf-8")
        try:
            snapshot(root, cache, new)
            failures.append("takes a snapshot serving no generation")
        except InputError:
            pass
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
        files = generate({sid: snapshot(root, citations.cache_dir(), sid) for sid in SNAPSHOTS})
    except InputError as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    if args == ["--check"]:
        stale = [p for p, text in files.items()
                 if not (root / p).exists() or (root / p).read_text(encoding="utf-8") != text]
        for p in stale:
            print(f"{p}: differs from what {' and '.join(SNAPSHOTS)} give (run scripts/generate_sf_tracking.py)",
                  file=sys.stderr)
        return 1 if stale else 0
    for p, text in files.items():
        (root / p).write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
