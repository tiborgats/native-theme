#!/usr/bin/env python3
"""Generate the platform-facts document from the verified facts (specification S17: "`docs/platform-facts.md` is
generated from these files"; S18; the 0.8.0 plan's M1a *Facts tooling*). Offline.

Reads the facts as scripts/check_facts.py loads them: `docs/facts/<platform>/*.toml` and `docs/facts/palettes/`, the
facts the lanes wrote and their verifiers checked (`docs/facts/imported/`, the lane input, and `docs/facts/staged/`
are no facts and are not read; a state-model file holds declarations, not values, and is not read either), and the
widget catalogue (through scripts/spec_catalogue.py) for the units.

Run from the repository root:

    python3 scripts/generate_platform_facts.py [--out PATH]           # write the document
    python3 scripts/generate_platform_facts.py [--out PATH] --check   # exit 1 when PATH differs from a fresh run
    python3 scripts/generate_platform_facts.py --self-test

PATH defaults to `docs/platform-facts.generated.md`: until the switch replaces today's hand-written
`docs/platform-facts.md` with this output (the plan's *The transition*), the script never writes that file. Exit 0 on
success, 1 when `--check` finds a difference, 2 when a fact file cannot be read (check_facts.py names it) or on a
wrong command line.

The document
------------
One section per unit, in this order: the colour roles (`role.…`), the tokens (`token.…`), the inputs (S9's
`input.…`; `input.<part>…` and `input.values…` are the `input` widget's), then the catalogue's sections in its order
(its H2 sections are M3's units: window and chrome, overlays, buttons and toggles, fields, indicators, containers, the
typed widgets the model lacks today, the keyed components), then any key the catalogue does not name (`Other`), then
the palette facts. Inside a section, one table per group: a widget or a component (`button`, `component.led`), a
colour-role set (`role.window`, `role.global`; every keyed role `role.@…` in one group), a token group (`token.motion`;
`token.params.<group>` per keyed scalar group), an input group (`input.kde_scheme`).

A table has the columns Key, Combination and one per preset any of its facts covers, in the presets' order
(check_facts.ALL_PRESETS). A row is a key and a combination: the fact's `when` without its `preset:` conditions,
written as an inline TOML table with its keys sorted (`—` for none), and for the accent run's facts its
`scheme_file` and `accent`. A fact covers its `preset:` generation, else every preset of its directory
(check_facts.PLATFORMS, whether or not the preset exists yet), less a `preset:` in `not_conditions`. A preset's cell
holds the fact that applies there, the last in file order for that key and combination (S17 *Which fact applies*;
the files in path order): its value as an inline TOML value, then one line per citation, `published`/`read` with
its source, `measured` with its evidence and tolerance. Rows sort by key, then combination (none first). A palette
table has the columns Key, Value and Citations. Markdown escapes: `|` as `\\|`.

    python3 scripts/generate_platform_facts.py --self-test   # fixture facts against the expected document
"""
import math
import re
import sys
import tempfile
from pathlib import Path

import check_facts

OUT = Path("docs/platform-facts.generated.md")
ROLES, TOKENS, INPUTS, OTHER, PALETTES = "Colour roles", "Tokens", "Inputs", "Other", "Palettes"
BARE_KEY = re.compile(r"^[A-Za-z0-9_-]+$")


class InputError(Exception):
    pass


# --------------------------------------------------------------------------------------------- TOML inline

def toml_key(k: str) -> str:
    return k if BARE_KEY.match(k) else toml_string(k)


def toml_string(s: str) -> str:
    out = s.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n").replace("\t", "\\t")
    return f'"{out}"'


def toml_inline(v, sort=False) -> str:
    """A TOML value written inline (tables `{ a = 1 }`, their keys sorted where `sort`)."""
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, int):
        return str(v)
    if isinstance(v, float):
        if math.isnan(v) or math.isinf(v):
            return "nan" if math.isnan(v) else ("inf" if v > 0 else "-inf")
        return repr(v)
    if isinstance(v, str):
        return toml_string(v)
    if isinstance(v, list):
        return "[" + ", ".join(toml_inline(x, sort) for x in v) + "]"
    if isinstance(v, dict):
        items = sorted(v.items()) if sort else v.items()
        if not v:
            return "{}"
        return "{ " + ", ".join(f"{toml_key(k)} = {toml_inline(x, sort)}" for k, x in items) + " }"
    return toml_string(str(v))


# ---------------------------------------------------------------------------------------------- the facts

def covered_presets(item) -> list:
    """The presets a fact covers, whether or not they exist (the docstring's rule), in ALL_PRESETS order."""
    when = item.data.get("when") if isinstance(item.data.get("when"), dict) else {}
    conds = when.get("conditions") if isinstance(when.get("conditions"), list) else []
    nots = when.get("not_conditions") if isinstance(when.get("not_conditions"), list) else []
    pres = [c[7:] for c in conds if isinstance(c, str) and c.startswith("preset:")]
    out = set(pres[:1]) if pres else set(check_facts.PLATFORMS.get(item.platform, ()))
    out -= {c[7:] for c in nots if isinstance(c, str) and c.startswith("preset:")}
    return [p for p in check_facts.ALL_PRESETS if p in out]


def combination(item) -> str:
    when = item.data.get("when") if isinstance(item.data.get("when"), dict) else {}
    w = {}
    for k, v in when.items():
        if k in ("conditions", "not_conditions") and isinstance(v, list):
            v = [c for c in v if not (isinstance(c, str) and c.startswith("preset:"))]
            if not v:
                continue
        w[k] = v
    text = toml_inline(w, sort=True) if w else ""
    if "scheme_file" in item.data or "accent" in item.data:
        run = f"accent run: {item.data.get('scheme_file')}, {item.data.get('accent')}"
        text = f"{text}; {run}" if text else run
    return text or "—"


def citation(cite: dict) -> str:
    kind = cite.get("kind")
    if kind == "measured":
        tol = cite.get("tolerance")
        return f"measured {cite.get('evidence')} (± {toml_inline(tol, sort=True)})"
    return f"{kind} {cite.get('source')}"


def code(text: str) -> str:
    return f"`` {text} ``" if "`" in text else f"`{text}`"


def cell(text: str) -> str:
    return text.replace("|", "\\|")


def place(key: str, entries: dict, input_parts: set):
    """(section, group) of a fact's key."""
    segs = key.split(".")
    head = segs[0]
    if head == "role" and len(segs) > 1:
        return ROLES, "role.@…" if segs[1].startswith("@") else f"role.{segs[1]}"
    if head == "token" and len(segs) > 1:
        if segs[1] == "params" and len(segs) > 2:
            return TOKENS, f"token.params.{segs[2]}"
        return TOKENS, f"token.{segs[1]}"
    if head == "input" and len(segs) > 1 and segs[1] not in input_parts and segs[1] != "values":
        return INPUTS, f"input.{segs[1]}"
    name = f"component.{segs[1]}" if head == "component" and len(segs) > 1 else head
    entry = entries.get(name)
    if entry is None:
        return OTHER, name
    return entry.section, name


def section_order(entries: dict) -> list:
    out = [ROLES, TOKENS, INPUTS]
    for e in entries.values():
        if e.section not in out:
            out.append(e.section)
    return out + [OTHER, PALETTES]


def anchor(title: str) -> str:
    return re.sub(r"\s", "-", re.sub(r"[^\w\s-]", "", title.lower()))


def generate(ctx) -> str:
    items, findings, _ = check_facts.load(ctx)
    if findings:
        raise InputError("; ".join(f"{f}:{ln}: {w}" for f, ln, w in findings))
    entries = ctx.entries
    input_entry = entries.get("input")
    input_parts = {n for p in input_entry.parts for n in p.names} if input_entry else set()
    tables = {}       # section -> group -> {(key, comb): {preset: item}}
    palettes = {}     # project -> [item]
    for it in items:
        if it.table != "fact":
            continue
        key = it.data.get("key")
        if not isinstance(key, str):
            continue
        if it.palette:
            palettes.setdefault(key.split(".")[0], []).append(it)
            continue
        section, group = place(key, entries, input_parts)
        row = tables.setdefault(section, {}).setdefault(group, {}).setdefault((key, combination(it)), {})
        for p in covered_presets(it):
            row[p] = it
    out = ["# Platform facts", "",
           "<!-- Generated by scripts/generate_platform_facts.py from docs/facts/; do not edit. -->", "",
           "Every value the platforms' verified facts state (`docs/facts/<platform>/*.toml`, specification S17), one "
           "section per unit and one table per widget, colour-role set or token group. A row is a key and a "
           "combination (the fact's `when` without its `preset:` conditions); a preset's cell holds the value of "
           "the fact that applies there (the last in file order) and its citations. The facts' files are the "
           "source: this document is regenerated from them.", ""]
    sections = [s for s in section_order(entries) if s in tables or (s == PALETTES and palettes)]
    if not sections:
        out += ["No fact yet.", ""]
    else:
        out += [f"- [{s}](#{anchor(s)})" for s in sections] + [""]
    for s in sections:
        out += [f"## {s}", ""]
        if s == PALETTES:
            for project in sorted(palettes):
                out += [f"### {code(project)}", "", "| Key | Value | Citations |", "|---|---|---|"]
                last = {}
                for it in palettes[project]:
                    last[it.data["key"]] = it
                for key in sorted(last):
                    it = last[key]
                    cites = "<br>".join(cell(citation(c.data)) for c in it.cites)
                    out.append(f"| {cell(code(key))} | {cell(code(toml_inline(it.data.get('value'))))} | {cites} |")
                out.append("")
            continue
        for group in sorted(tables[s]):
            rows = tables[s][group]
            presets = [p for p in check_facts.ALL_PRESETS if any(p in r for r in rows.values())]
            out += [f"### {code(group)}", "",
                    "| Key | Combination | " + " | ".join(presets) + " |",
                    "|---|---|" + "---|" * len(presets)]
            for (key, comb) in sorted(rows, key=lambda kc: (kc[0], kc[1] != "—", kc[1])):
                cells = []
                for p in presets:
                    it = rows[(key, comb)].get(p)
                    if it is None:
                        cells.append("")
                        continue
                    parts = [code(toml_inline(it.data.get("value")))] + [citation(c.data) for c in it.cites]
                    cells.append("<br>".join(cell(x) for x in parts))
                combo = comb if comb == "—" else code(comb)
                out.append(f"| {cell(code(key))} | {cell(combo)} | " + " | ".join(cells) + " |")
            out.append("")
    return "\n".join(out).rstrip("\n") + "\n"


# --------------------------------------------------------------------------------------------- self-test

SELF_TEST_FILES = {
    "docs/facts/gnome/button.toml": '''
[[fact]]
key = "button.body.min_width"
value = "16px"
rows = ["GN-1.29"]

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:21"
quote = "min-width: 16px;"

[[fact]]
key = "button.body.fill"
when = { variants = ["flat"], states = ["hover"] }
value = "= alpha(surface.foreground, 0.05)"
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 a.scss:1"
quote = "x"

[[fact]]
key = "button.body.fill"
when = { states = ["hover"], variants = ["flat"] }
value = "= alpha(surface.foreground, 0.07)"
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 a.scss:2"
quote = "x"
''',
    "docs/facts/gnome/roles.toml": '''
[[fact]]
key = "role.window.background"
when = { conditions = ["scheme:dark"] }
value = "#222226"
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 b.scss:3"
quote = "x"

[[fact]]
key = "role.@overview_background"
value = "#fafafb"
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 b.scss:4"
quote = "x | y"
''',
    "docs/facts/macos/button.toml": '''
[[fact]]
key = "button.body.min_width"
value = "20px"
rows = []

[[fact.cite]]
kind = "measured"
evidence = "docs/evidence/os-probe/2026-10-30-run-1/os-probe-macos-26/probe.toml#button.min_width"
tolerance = 0.5

[[fact]]
key = "button.body.min_width"
when = { conditions = ["preset:macos-monterey"] }
value = "18px"
rows = []

[[fact.cite]]
kind = "published"
source = "xcode@16.4 a.h:1"
quote = "x"
''',
    "docs/facts/gnome/tokens.toml": '''
[[fact]]
key = "token.params.opacity.disabled"
value = 0.5
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 c.scss:5"
quote = "x"

[[fact]]
key = "token.motion.timings.ease"
value = { cubic_bezier = [0.25, 0.1, 0.25, 1.0] }
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 c.scss:6"
quote = "x"
''',
    "docs/facts/imported/gnome.toml": '''
[[row]]
text = "not a fact"
''',
    "docs/facts/palettes/catppuccin.toml": '''
[[fact]]
key = "catppuccin.mocha.base"
value = "#1e1e2e"

[[fact.cite]]
kind = "published"
source = "catppuccin@1.7.1 palette.json:3"
quote = "x"
''',
}

SELF_TEST_EXPECTED = '''# Platform facts

<!-- Generated by scripts/generate_platform_facts.py from docs/facts/; do not edit. -->

Every value the platforms' verified facts state (`docs/facts/<platform>/*.toml`, specification S17), one section per unit and one table per widget, colour-role set or token group. A row is a key and a combination (the fact's `when` without its `preset:` conditions); a preset's cell holds the value of the fact that applies there (the last in file order) and its citations. The facts' files are the source: this document is regenerated from them.

- [Colour roles](#colour-roles)
- [Tokens](#tokens)
- [Buttons and toggles](#buttons-and-toggles)
- [Palettes](#palettes)

## Colour roles

### `role.@…`

| Key | Combination | adwaita |
|---|---|---|
| `role.@overview_background` | — | `"#fafafb"`<br>published libadwaita@1.10.0 b.scss:4 |

### `role.window`

| Key | Combination | adwaita |
|---|---|---|
| `role.window.background` | `{ conditions = ["scheme:dark"] }` | `"#222226"`<br>published libadwaita@1.10.0 b.scss:3 |

## Tokens

### `token.motion`

| Key | Combination | adwaita |
|---|---|---|
| `token.motion.timings.ease` | — | `{ cubic_bezier = [0.25, 0.1, 0.25, 1.0] }`<br>published libadwaita@1.10.0 c.scss:6 |

### `token.params.opacity`

| Key | Combination | adwaita |
|---|---|---|
| `token.params.opacity.disabled` | — | `0.5`<br>published libadwaita@1.10.0 c.scss:5 |

## Buttons and toggles

### `button`

| Key | Combination | adwaita | macos-monterey | macos-tahoe |
|---|---|---|---|---|
| `button.body.fill` | `{ states = ["hover"], variants = ["flat"] }` | `"= alpha(surface.foreground, 0.07)"`<br>published libadwaita@1.10.0 a.scss:2 |  |  |
| `button.body.min_width` | — | `"16px"`<br>published libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:21 | `"18px"`<br>published xcode@16.4 a.h:1 | `"20px"`<br>measured docs/evidence/os-probe/2026-10-30-run-1/os-probe-macos-26/probe.toml#button.min_width (± 0.5) |

## Palettes

### `catppuccin`

| Key | Value | Citations |
|---|---|---|
| `catppuccin.mocha.base` | `"#1e1e2e"` | published catppuccin@1.7.1 palette.json:3 |
'''


def fixture_context(root: Path):
    return check_facts.Context(root, core_text=check_facts.SELF_TEST_CORE, cat_text=check_facts.SELF_TEST_CATALOGUE,
                               sources_text=check_facts.SELF_TEST_SOURCES, rows=check_facts.SELF_TEST_ROWS)


def write(root: Path, files: dict):
    for rel, text in files.items():
        p = root / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text, encoding="utf-8")


def self_test() -> int:
    failures = []
    with tempfile.TemporaryDirectory() as tmp:
        base = Path(tmp)
        root = base / "good"
        write(root, SELF_TEST_FILES)
        got = generate(fixture_context(root))
        if got != SELF_TEST_EXPECTED:
            import difflib
            diff = "".join(difflib.unified_diff(SELF_TEST_EXPECTED.splitlines(True), got.splitlines(True),
                                                "expected", "generated"))
            failures.append(f"the fixture document differs:\n{diff}")
        # --check: the written file passes, an edited one does not, a missing one does not
        out = base / "out.md"
        out.write_text(got, encoding="utf-8")
        if check_file(fixture_context(root), out) != 0:
            failures.append("--check: a fresh file differs")
        out.write_text(got.replace("\"16px\"", "\"15px\""), encoding="utf-8")
        if check_file(fixture_context(root), out) != 1:
            failures.append("--check: an edited file passes")
        if check_file(fixture_context(root), base / "missing.md") != 1:
            failures.append("--check: a missing file passes")
        # no fact: the document says so
        empty = base / "empty"
        write(empty, {"docs/facts/sources.toml": ""})
        if "No fact yet." not in generate(fixture_context(empty)):
            failures.append("no fact: not said")
        # a fact file that is no TOML is an input error, not a silently shorter document
        bad = base / "bad"
        write(bad, {**SELF_TEST_FILES, "docs/facts/gnome/broken.toml": "[[fact]\n"})
        try:
            generate(fixture_context(bad))
            failures.append("a broken fact file: accepted")
        except InputError:
            pass
        if toml_inline({"b": [1, "x|y"], "a": True}, sort=True) != '{ a = true, b = [1, "x|y"] }':
            failures.append(f"toml_inline: {toml_inline({'b': [1, 'x|y'], 'a': True}, sort=True)}")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def check_file(ctx, out: Path, quiet=True) -> int:
    fresh = generate(ctx)
    try:
        current = out.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        current = None
    if current != fresh:
        if not quiet:
            print(f"{out}: {'missing' if current is None else 'differs from a fresh run'} "
                  f"(python3 scripts/generate_platform_facts.py --out {out})", file=sys.stderr)
        return 1
    return 0


def main(argv) -> int:
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    out, check = OUT, False
    while args:
        a = args.pop(0)
        if a == "--check":
            check = True
        elif a == "--out" and args:
            out = Path(args.pop(0))
        else:
            print(__doc__, file=sys.stderr)
            return 2
    try:
        ctx = check_facts.Context(Path("."))
        if check:
            status = check_file(ctx, out, quiet=False)
            if status == 0:
                print(f"{out}: up to date", file=sys.stderr)
            return status
        text = generate(ctx)
    except (OSError, UnicodeDecodeError, InputError, check_facts.InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    out.write_text(text, encoding="utf-8")
    tables = text.count("\n### ")
    rows = sum(1 for line in text.splitlines() if line.startswith("| `"))
    print(f"wrote {out}: {tables} tables, {rows} rows", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
