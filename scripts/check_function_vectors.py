#!/usr/bin/env python3
"""The platform functions' and placement strategies' vector tables (specification S2.2 *Checked against their
references*, S13; the 0.8.0 plan's *How values are checked*, M1a's *Facts tooling*). Offline.

Reads the vector tables `native-theme/tests/vectors/*.tsv`, the tests `native-theme/tests/platform_vectors.rs`, the
Rust sources of the `platform` module (`native-theme/src/next/platform/`, `native-theme/src/platform/` after the
switch) and of the `placement` module (`native-theme/src/next/placement/`, `src/placement/`), the facts and the new
model's presets as scripts/check_facts.py reads them, `docs/facts/sources.toml` and the core's function table and
enums (the specification, through check_facts.py).

Run from the repository root:

    python3 scripts/check_function_vectors.py
    python3 scripts/check_function_vectors.py --self-test

It prints one line per finding, `<file>:<line>: <what>`, informational lines starting `# ` (what passed vacuously,
the platform functions called but not yet implemented, tables waiting for their implementation), and a summary line
with the counts (on stderr). Exit 0 when there is no finding, 1 when there are some, 2 on an input it cannot read
(sources.toml, the core, the catalogue) or a wrong command line.

The vector-table file format
----------------------------
One table per platform function, `native-theme/tests/vectors/<function>.tsv` (`adw_standalone.tsv`), and per
placement strategy, `native-theme/tests/vectors/place.<strategy>.tsv`, the strategy as S13's `PlacementStrategy`
variant in snake_case (`place.gtk_popover.tsv`, `place.keyboard_avoid.tsv`; a variant with data takes its data as an
enum input column). UTF-8, lines ending in `\\n`, tab-separated.

1. Header lines, before anything else (after the column header row every line is a row, so a row may start with a
   `#rrggbb` colour). A line starting `#` is a header field when it reads `# <name>: <value>`
   with `<name>` lower-case letters and `_`; HEADER_NAMES are the only names, so a misspelled field is a finding
   (a free comment is any other `#` line, and must not itself start with `# <word>:`). The fields:
   - `function` (once): the table's function, the file's stem (`adw_standalone`, `place.gtk_popover`).
   - `route` (once): how the rows trace to the reference (S2.2: one route per reference), one of ROUTES:
     `public_api` (the reference's public API called by a CI tool), `observable_behaviour` (its behaviour observed
     through public API), `compiled_source` (the pinned upstream source compiled unchanged with a harness calling
     the internal function), together the generated routes; `measured` (a closed reference: inputs and outputs
     measured, each row with its evidence); `published` (the platform publishes the table: Apple's tracking; each
     row with its citation).
   - `source` (repeatable): a `docs/facts/sources.toml` `[[source]]` id, the reference at its pin. A generated or
     published table names at least one; a measured table may name the entries its runs measured (their `run.toml`
     names them per image).
   - `generations` (once): the preset IDs the table serves, comma-separated (`adwaita` or `macos-tahoe, ios-26`).
   - `generator` (once; required on the generated routes, not allowed on `published`): the CI tool that wrote the
     table, a repository path (`tools/gnome-vectors`); its pin is the table's `source` entries.
   - `inputs` (once): the input columns, space-separated `<name>:<type>`: first the call's arguments in S2.2's
     parameter order (`c:colour scheme:enum<Scheme>`), then the operands the function reads itself, which S2.2 lists
     as its operands (`m3_role` reads `input.accent`, …); a strategy's inputs are `PlacementInput`'s fields.
   - `outputs` (once): the output columns, the same form (`result:colour`; a strategy's `frame:rect`).
   - `tolerance` (repeatable; on `measured` only, where it is required): `<output column> <number >= 0>` for each
     output column of a numeric, colour or composite type, in the column type's canonical unit (a colour's per
     8-bit channel, S17). Generated rows compare unrounded, colours on the 0-255 scale to 1e-3, and published rows
     at the quote check's precision (S2.2, S17): neither states one.
2. The column header row: the input columns, the output columns, then the route's columns, then (a strategy's
   table) PLACEMENT_COLUMNS, each name once:
   - generated routes: optional `source`, the header's entry that wrote the row where the header names several;
   - `published`: `cite` (`<sources.toml id> <path>:<line or l1-l2>` or `<id> #<JSON pointer>`, the id one of the
     header's `source` entries), `quote` (the cited text, its tabs and newlines written as spaces: the quote check
     normalises whitespace), optional `conversion` (check_fact_citations.CONVERSIONS' names, comma-separated, or
     `-`); check_fact_citations.py checks what they say;
   - `measured`: `evidence` (`docs/evidence/<harness>/<YYYY-MM-DD>-run-<id>/<image>/<file>#<path>`, S17's form: the
     table there holds the inputs the harness set or read and the outputs it measured);
   - a strategy's table: `preset` (the preset the row serves), `value` (the placed value's field path,
     `<widget>.values.<name>`), `when` (S11's `when` of the rule placing it, a TOML inline table, `{}` for none):
     the row's `PlacementParams` are that preset's, resolved under that `when` (S2.2), not columns.
3. The rows, one per line, as many cells as columns, none empty. A cell is spelled by its column's TYPES:
   `colour` S2.1's literal forms (`#rrggbb`, `#rrggbbaa`, `srgb(r, g, b[, a])`, `display_p3(r, g, b[, a])`); a
   generated output colour is `srgb(…)` or `display_p3(…)` with the reference's unrounded channels (`#…` rounds
   them to bytes); `length` `<n>px`, `duration` `<n>ms`, `angle` `<n>deg`, `font_size` `<n>px` (logical px: a font
   size reaches a function converted and scaled, S2.2), `number` `<n>`, where `<n>` is a decimal number with an
   optional exponent (`1.5e-05`), never NaN or infinite; `bool` `true` or `false`; `text` the string as written;
   `enum<E>` a variant in snake_case, of the core's enum `E` where the core defines it; `name` `[@]<snake_case>`
   (a declared key: `m3_role`'s role); `rect`, `size`, `point`, `sides` JSON arrays of 4, 2, 2 and 4 numbers (x, y,
   width, height; width, height; x, y; top, right, bottom, left), `shadow` a JSON array of layer objects (S2.5's
   ShadowLayer: `x`, `y`, `blur`, `spread` numbers, `color` a colour literal, `inset` a bool); `option<T>` `none`
   or a `T`. A table has at least one row.

What is checked
---------------
- The form above, for every table under `native-theme/tests/vectors/` (a file there that is no `.tsv` is a
  finding, a README.md excepted).
- Coverage: every `pub` and `pub(crate)` function of the `platform` module (its `.rs` files; items of a
  `#[cfg(test)]` module are not the module's) has a table `<fn>.tsv` and a test `#[test] fn vectors_<fn>` in
  `native-theme/tests/platform_vectors.rs`, and so does every function S2.2's table lists as a platform function
  (every row but the model's own arithmetic, ARITHMETIC: `alpha`, `with_alpha`, `mix`, `mul`, `div`, `add`, `sub`,
  `min`, `max`, `floor`, `round`, `pick`) that is implemented there; the functions implementing S9.4's
  scaling curves live in `platform` too and are covered by the same rule. Once the `placement` module exists, every
  `PlacementStrategy` variant has `place.<strategy>.tsv` and `vectors_place_<strategy>` (S13: each strategy
  implemented). A table or test of anything else is a finding (the model's arithmetic has unit tests, S2.2), except a
  table of an S2.2 platform function not implemented yet, which is reported (a clean-room implementer receives the
  table first, the plan's D10).
- Calls: every platform function a fact (`docs/facts/<platform>/*.toml`, as check_facts.py loads them) or a preset
  (`native-theme/src/next/presets/*.toml`) calls in an expression is implemented and has a table. A function only
  facts call and that is not implemented yet is reported as called, not yet implemented; it is a finding once a
  preset calls it. A call of a name outside S2.2's table is check_facts.py's finding.
- Generations (S2.2), on a table whose form holds (a form finding is fixed first): a row serves the generations its generator's pin serves (the generated routes: the `sources`
  entries' generations, the row's own `source` where it has one), its citation's entry's (`published`), or the
  preset its evidence run's `run.toml` maps its image to (`measured`). Every generation that calls the function (a
  preset file's own ID; a fact's covered generations as check_facts.py's generation rule takes them: its `preset:`
  one, else its directory's presets that exist) is in `# generations:`, and each of those is served by some row. A
  call's enum arguments select rows: a literal variant or name selects the rows with that cell, any other argument
  (a reference, `scheme`) every variant of the core's enum (any row where the core does not define it). For each
  generation calling the function and each variant combination it selects, some row of that combination serves the
  generation, and every row of that combination serves it. A strategy's row serves its `preset`.

Not yet checked (the plan enables them with their milestones): that every placed value of every preset naming a
strategy has rows (from M3's placement unit, when presets spell a `Placement`); the rows' citations and evidence,
which check_fact_citations.py checks through table_evidence() and the citation columns.

API: tables(root) -> list[Table]; table_evidence(root) -> set of the evidence files the measured tables' rows cite
(check_fact_citations.py's vector_table_evidence hook).

    python3 scripts/check_function_vectors.py --self-test   # each rule on fixture tables, Rust snippets, facts
"""
import json
import math
import re
import sys
import tempfile
import tomllib
from dataclasses import dataclass, field
from itertools import product
from pathlib import Path

import check_facts

VECTORS = Path("native-theme/tests/vectors")
TESTS = Path("native-theme/tests/platform_vectors.rs")
PLATFORM_DIRS = (Path("native-theme/src/next/platform"), Path("native-theme/src/platform"))
PLACEMENT_DIRS = (Path("native-theme/src/next/placement"), Path("native-theme/src/placement"))
# S2.2's model's own arithmetic, no platform function (`with_alpha` ADD-A123's, `round` ADD-A122's)
ARITHMETIC = {"alpha", "with_alpha", "mix", "mul", "div", "add", "sub", "min", "max", "floor", "round", "pick"}
GENERATED = ("public_api", "observable_behaviour", "compiled_source")
ROUTES = GENERATED + ("measured", "published")
HEADER_NAMES = {"function", "route", "source", "generations", "generator", "inputs", "outputs", "tolerance"}
REPEATABLE = {"source", "tolerance"}
ROUTE_COLUMNS = {
    # route: (required, optional), in this order after the outputs
    **{r: ((), ("source",)) for r in GENERATED},
    "published": (("cite", "quote"), ("conversion",)),
    "measured": (("evidence",), ()),
}
PLACEMENT_COLUMNS = ("preset", "value", "when")
PLACE = "place."
SCALAR_TYPES = {"colour", "length", "duration", "angle", "font_size", "number", "bool", "text", "name"}
COMPOSITE_TYPES = {"rect": 4, "size": 2, "point": 2, "sides": 4, "shadow": None}
TOLERANCE_TYPES = {"colour", "length", "duration", "angle", "font_size", "number"} | set(COMPOSITE_TYPES)
UNIT_OF = {"length": "px", "duration": "ms", "angle": "deg", "font_size": "px"}
NUMBER = r"-?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?"
NUMBER_RX = re.compile(rf"^{NUMBER}$")
HEADER_RX = re.compile(r"^# ([a-z_]+): ?(.*)$")
COLUMN_RX = re.compile(r"^[a-z_][a-z0-9_]*$")
TYPE_RX = re.compile(r"^(?:option<(?P<opt>.+)>|enum<(?P<enum>[A-Z][A-Za-z0-9]*)>|(?P<plain>[a-z_]+))$")
NAME_RX = re.compile(r"^@?[a-z_][a-z0-9_]*$")
SNAKE_RX = re.compile(r"^[a-z_][a-z0-9_]*$")
VALUE_PATH_RX = re.compile(r"^[a-z_][a-z0-9_]*\.values\.[a-z_][a-z0-9_]*$")
GENERATOR_RX = re.compile(r"^[A-Za-z0-9_.-]+(?:/[A-Za-z0-9_.-]+)*/?$")
PUB_FN_RX = re.compile(r"\bpub(?:\(crate\))?\s+(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?(?:extern\s+\"[^\"]*\"\s+)?"
                       r"fn\s+([A-Za-z_][A-Za-z0-9_]*)")
FN_RX = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*[(<]")


class InputError(Exception):
    pass


# --------------------------------------------------------------------------------------------- the tables

@dataclass
class Column:
    name: str
    type: str          # as written: colour, enum<Scheme>, option<rect>, …
    role: str          # "input", "output", "route", "placement"


@dataclass
class Table:
    file: str          # repository path
    stem: str
    header: dict = field(default_factory=dict)     # name -> [(line, value)]
    columns: list = field(default_factory=list)    # [Column]
    rows: list = field(default_factory=list)       # [(line, {column: cell})]
    findings: list = field(default_factory=list)   # [(line, what)]

    def one(self, name):
        v = self.header.get(name)
        return v[0][1] if v else None

    def line_of(self, name):
        v = self.header.get(name)
        return v[0][0] if v else 1

    @property
    def route(self):
        return self.one("route")

    @property
    def inputs(self):
        return [c for c in self.columns if c.role == "input"]

    @property
    def outputs(self):
        return [c for c in self.columns if c.role == "output"]

    @property
    def sources(self):
        return [v for _, v in self.header.get("source", [])]

    @property
    def generations(self):
        g = self.one("generations")
        return [x.strip() for x in g.split(",") if x.strip()] if g else []


def parse_columns(spec: str, role: str):
    """[Column] of an `# inputs:` or `# outputs:` value, or a problem string."""
    out = []
    for part in spec.split():
        name, sep, typ = part.partition(":")
        if not sep or not COLUMN_RX.match(name):
            return f"`{part}` is not `<name>:<type>`"
        problem = type_problem(typ)
        if problem:
            return f"`{part}`: {problem}"
        out.append(Column(name, typ, role))
    if not out:
        return "no column"
    return out


def type_problem(typ: str):
    m = TYPE_RX.match(typ)
    if not m:
        return f"`{typ}` is not a column type"
    if m.group("opt"):
        inner = m.group("opt")
        return "`option<option<…>>`" if inner.startswith("option<") else type_problem(inner)
    if m.group("enum"):
        return None
    if m.group("plain") not in SCALAR_TYPES and m.group("plain") not in COMPOSITE_TYPES:
        return f"`{typ}` is not a column type (one of {', '.join(sorted(SCALAR_TYPES | set(COMPOSITE_TYPES)))}, " \
               f"enum<E>, option<T>)"
    return None


def base_type(typ: str) -> str:
    m = TYPE_RX.match(typ)
    if m and m.group("opt"):
        return base_type(m.group("opt"))
    return typ


def enum_of(typ: str):
    m = TYPE_RX.match(base_type(typ))
    return m.group("enum") if m else None


def colour_problem(cell: str, unrounded: bool):
    if check_facts.HEX_LITERAL.match(cell):
        return "a generated output colour is written unrounded, `srgb(…)` or `display_p3(…)`" if unrounded else None
    m = re.match(rf"^(srgb|display_p3)\(\s*({NUMBER})\s*,\s*({NUMBER})\s*,\s*({NUMBER})\s*(?:,\s*({NUMBER})\s*)?\)$",
                 cell)
    if m:
        return None
    return f"`{cell}` is not a colour literal (S2.1)"


def cell_problem(cell: str, typ: str, ctx, unrounded=False):
    """A cell's problem for its column type, or None."""
    m = TYPE_RX.match(typ)
    if m and m.group("opt"):
        return None if cell == "none" else cell_problem(cell, m.group("opt"), ctx, unrounded)
    if m and m.group("enum"):
        e = m.group("enum")
        if not SNAKE_RX.match(cell):
            return f"`{cell}` is not a variant in snake_case"
        variants = ctx.enums.get(e) if ctx is not None else None
        if variants is not None and cell not in variants:
            return f"`{cell}` is not a variant of {e} ({', '.join(variants)})"
        return None
    if typ == "colour":
        return colour_problem(cell, unrounded)
    if typ in UNIT_OF:
        unit = UNIT_OF[typ]
        if not (cell.endswith(unit) and NUMBER_RX.match(cell[:-len(unit)])):
            return f"`{cell}` is not a {typ.replace('_', ' ')} (`<n>{unit}`)"
        return None
    if typ == "number":
        return None if NUMBER_RX.match(cell) else f"`{cell}` is not a number"
    if typ == "bool":
        return None if cell in ("true", "false") else f"`{cell}` is not `true` or `false`"
    if typ == "text":
        return None
    if typ == "name":
        return None if NAME_RX.match(cell) else f"`{cell}` is not a name (`[@]snake_case`)"
    if typ in COMPOSITE_TYPES:
        try:
            v = json.loads(cell)
        except json.JSONDecodeError:
            return f"`{cell}` is not JSON ({typ})"
        if typ == "shadow":
            if not isinstance(v, list) or not all(isinstance(x, dict) for x in v):
                return f"`{cell}` is not a JSON array of shadow layers"
            for layer in v:
                extra = set(layer) - {"x", "y", "blur", "spread", "color", "inset"}
                if extra:
                    return f"shadow layer key(s) {sorted(extra)} (S2.5's ShadowLayer)"
                if any(k in layer and not finite(layer[k]) for k in ("x", "y", "blur", "spread")):
                    return "a shadow layer's x, y, blur and spread are numbers"
                if "color" in layer and (not isinstance(layer["color"], str) or colour_problem(layer["color"], False)):
                    return "a shadow layer's color is a colour literal"
                if "inset" in layer and not isinstance(layer["inset"], bool):
                    return "a shadow layer's inset is a bool"
            return None
        n = COMPOSITE_TYPES[typ]
        if not isinstance(v, list) or len(v) != n or not all(finite(x) for x in v):
            return f"`{cell}` is not a JSON array of {n} numbers ({typ})"
        return None
    return f"no type `{typ}`"


def finite(x):
    return isinstance(x, (int, float)) and not isinstance(x, bool) and math.isfinite(x)


def read_table(path: Path, rel: str, ctx) -> Table:
    t = Table(rel, path.name[:-len(".tsv")])
    try:
        text = path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError) as err:
        t.findings.append((1, f"cannot be read: {err}"))
        return t
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    i = 0
    while i < len(lines) and lines[i].startswith("#"):
        m = HEADER_RX.match(lines[i])
        if m:
            name, value = m.group(1), m.group(2).strip()
            if name not in HEADER_NAMES:
                t.findings.append((i + 1, f"`# {name}:` is no header field ({', '.join(sorted(HEADER_NAMES))}; a "
                                          f"free comment does not start with `# <word>:`)"))
            elif name in t.header and name not in REPEATABLE:
                t.findings.append((i + 1, f"`# {name}:` twice"))
            else:
                t.header.setdefault(name, []).append((i + 1, value))
        i += 1
    for name in ("function", "route", "generations", "inputs", "outputs"):
        if name not in t.header:
            t.findings.append((1, f"no `# {name}:` line"))
    if t.one("function") is not None and t.one("function") != t.stem:
        t.findings.append((t.line_of("function"), f"`# function: {t.one('function')}` is not the file's stem "
                                                  f"`{t.stem}`"))
    route = t.route
    if route is not None and route not in ROUTES:
        t.findings.append((t.line_of("route"), f"route `{route}` is not one of {', '.join(ROUTES)}"))
        route = None
    for ln, sid in t.header.get("source", []):
        if sid not in ctx.sources:
            t.findings.append((ln, f"source `{sid}` is no [[source]] of docs/facts/sources.toml"))
    if route in GENERATED + ("published",) and not t.sources:
        t.findings.append((1, f"a `{route}` table names its reference's `# source:` entries"))
    if route in GENERATED and "generator" not in t.header:
        t.findings.append((1, f"a `{route}` table names its `# generator:`"))
    if route == "published" and "generator" in t.header:
        t.findings.append((t.line_of("generator"), "a `published` table has no generator: its rows are cited"))
    gen = t.one("generator")
    if gen is not None and (not GENERATOR_RX.match(gen) or ".." in gen.split("/")):
        t.findings.append((t.line_of("generator"), f"generator `{gen}` is not a repository path"))
    gens = t.generations
    if "generations" in t.header:
        if not gens:
            t.findings.append((t.line_of("generations"), "no generation"))
        for g in gens:
            if g not in check_facts.ALL_PRESETS:
                t.findings.append((t.line_of("generations"), f"`{g}` is not a platform preset"))
        if len(set(gens)) != len(gens):
            t.findings.append((t.line_of("generations"), "a generation twice"))
    columns_ok = True
    for name in ("inputs", "outputs"):
        if name in t.header:
            cols = parse_columns(t.one(name), name[:-1])
            if isinstance(cols, str):
                t.findings.append((t.line_of(name), f"`# {name}:` {cols}"))
                columns_ok = False
            else:
                t.columns += cols
        else:
            columns_ok = False
    tolerances = {}
    for ln, v in t.header.get("tolerance", []):
        parts = v.split()
        if len(parts) != 2 or not NUMBER_RX.match(parts[1]) or float(parts[1]) < 0:
            t.findings.append((ln, f"`# tolerance: {v}` is not `<output column> <number >= 0>`"))
            continue
        if parts[0] in tolerances:
            t.findings.append((ln, f"tolerance of `{parts[0]}` twice"))
        tolerances[parts[0]] = float(parts[1])
    if route is not None and route != "measured" and tolerances:
        t.findings.append((t.line_of("tolerance"), f"a `{route}` table states no tolerance (generated rows compare "
                                                   f"unrounded, published ones at the quote check's precision)"))
    if route == "measured":
        outs = {c.name: c for c in t.outputs}
        for name in tolerances:
            if name not in outs:
                t.findings.append((t.line_of("tolerance"), f"tolerance of `{name}`, which is no output column"))
        for c in t.outputs:
            if base_type(c.type) in TOLERANCE_TYPES and c.name not in tolerances:
                t.findings.append((t.line_of("outputs"), f"measured output `{c.name}` has no `# tolerance:` line"))
    names = [c.name for c in t.columns]
    if len(set(names)) != len(names):
        t.findings.append((t.line_of("inputs"), "a column name twice"))
    # the column header row
    if i >= len(lines):
        t.findings.append((len(lines) or 1, "no column header row"))
        return t
    header_line = i + 1
    got = lines[i].split("\t")
    if route is not None and columns_ok:
        req, opt = ROUTE_COLUMNS[route]
        extra = [c for c in got[len(names):] if c not in PLACEMENT_COLUMNS]
        want_route = list(req) + [c for c in opt if c in extra]
        for c in want_route:
            t.columns.append(Column(c, "route", "route"))
        if t.stem.startswith(PLACE):
            t.columns += [Column(c, "placement", "placement") for c in PLACEMENT_COLUMNS]
        want = [c.name for c in t.columns]
        if got != want:
            t.findings.append((header_line, f"column header row {got} is not {want} (inputs, outputs, the "
                                             f"`{route}` route's columns{', the placement columns' if t.stem.startswith(PLACE) else ''})"))
            return t
    else:
        return t
    rows = 0
    for j in range(i + 1, len(lines)):
        ln = j + 1
        cells = lines[j].split("\t")
        if len(cells) != len(t.columns):
            t.findings.append((ln, f"{len(cells)} cells, not {len(t.columns)}"))
            continue
        row = dict(zip(names_of(t.columns), cells))
        rows += 1
        t.rows.append((ln, row))
        for c in t.columns:
            cell = row[c.name]
            if cell == "" or cell != cell.strip():
                t.findings.append((ln, f"`{c.name}`: an empty cell, or one with surrounding whitespace"))
                continue
            if c.role in ("input", "output"):
                problem = cell_problem(cell, c.type, ctx, unrounded=(c.role == "output" and route in GENERATED))
                if problem:
                    t.findings.append((ln, f"`{c.name}`: {problem}"))
            else:
                problem = route_cell_problem(t, c.name, cell, ctx)
                if problem:
                    t.findings.append((ln, f"`{c.name}`: {problem}"))
    if not rows:
        t.findings.append((header_line, "no row"))
    return t


def names_of(columns):
    return [c.name for c in columns]


def route_cell_problem(t: Table, name: str, cell: str, ctx):
    if name == "source":
        return None if cell in t.sources else f"`{cell}` is not one of the header's `# source:` entries"
    if name == "cite":
        src = check_facts.parse_source(cell)
        if not src:
            return f"`{cell}` is not `<id> <path>:<line or l1-l2>` or `<id> #<JSON pointer>`"
        return None if src[0] in t.sources else f"`{src[0]}` is not one of the header's `# source:` entries"
    if name == "quote":
        return None
    if name == "conversion":
        if cell == "-":
            return None
        bad = [x.strip() for x in cell.split(",") if x.strip() not in check_facts.conversions()]
        return f"conversion {bad} not in {sorted(check_facts.conversions())}" if bad else None
    if name == "evidence":
        return None if check_facts.parse_evidence(cell) else \
            f"`{cell}` is not `docs/evidence/<harness>/<YYYY-MM-DD>-run-<id>/<image>/<file>#<path>`"
    if name == "preset":
        return None if cell in check_facts.ALL_PRESETS else f"`{cell}` is not a platform preset"
    if name == "value":
        return None if VALUE_PATH_RX.match(cell) else f"`{cell}` is not `<widget>.values.<name>`"
    if name == "when":
        try:
            w = tomllib.loads(f"w = {cell}")["w"]
        except tomllib.TOMLDecodeError:
            return f"`{cell}` is not a TOML inline table"
        if not isinstance(w, dict):
            return f"`{cell}` is not a TOML inline table"
        extra = set(w) - check_facts.WHEN_KEYS
        return f"`when` key(s) {sorted(extra)} (S11)" if extra else None
    return f"no column `{name}`"


def tables(root: Path, ctx) -> list:
    """Every table under VECTORS, read (its form findings in Table.findings), and the stray files as findings."""
    out, stray = [], []
    d = root / VECTORS
    if not d.is_dir():
        return out, stray
    for p in sorted(d.rglob("*")):
        rel = p.relative_to(root).as_posix()
        if p.is_dir():
            stray.append((rel, 1, "a folder under the vectors directory"))
        elif p.suffix != ".tsv" or p.parent != d:
            if p.name != "README.md":
                stray.append((rel, 1, "not a vector table (`native-theme/tests/vectors/<function>.tsv`)"))
        else:
            out.append(read_table(p, rel, ctx))
    return out, stray


def table_evidence(root: Path, ctx=None) -> set:
    """The evidence files (repository paths) the measured tables' rows cite (check_fact_citations.py's hook)."""
    out = set()
    d = Path(root) / VECTORS
    if not d.is_dir():
        return out
    for p in sorted(d.glob("*.tsv")):
        try:
            lines = p.read_text(encoding="utf-8").split("\n")
        except (OSError, UnicodeDecodeError):
            continue
        start = next((i for i, ln in enumerate(lines) if not ln.startswith("#")), len(lines))
        body = [ln for ln in lines[start:] if ln]
        if not body:
            continue
        cols = body[0].split("\t")
        if "evidence" not in cols:
            continue
        k = cols.index("evidence")
        for line in body[1:]:
            cells = line.split("\t")
            ev = check_facts.parse_evidence(cells[k]) if k < len(cells) else None
            if ev:
                out.add(ev[0])
    return out


# ------------------------------------------------------------------------------------------------ the Rust

def strip_rust_comments(text: str) -> str:
    """The text with `//` and `/* */` comments blanked (newlines kept), string literals left as they are."""
    out, i, n = [], 0, len(text)
    while i < n:
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            out.append(" " * (j - i))
            i = j
        elif text.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif text.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            out.append("".join(c if c == "\n" else " " for c in text[i:j]))
            i = j
        elif text[i] == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            out.append(text[i:j + 1])
            i = j + 1
        else:
            out.append(text[i])
            i += 1
    return "".join(out)


def without_cfg_test(text: str) -> str:
    """The text with every item under `#[cfg(test)]` blanked (its braces matched), newlines kept."""
    out = text
    for m in reversed(list(re.finditer(r"#\[cfg\(test\)\]", text))):
        start = m.start()
        brace = text.find("{", m.end())
        semi = text.find(";", m.end())
        if brace < 0 or (0 <= semi < brace):
            end = semi + 1 if semi >= 0 else len(text)
        else:
            depth, j = 0, brace
            while j < len(text):
                if text[j] == "{":
                    depth += 1
                elif text[j] == "}":
                    depth -= 1
                    if depth == 0:
                        break
                j += 1
            end = j + 1
        out = out[:start] + "".join(c if c == "\n" else " " for c in out[start:end]) + out[end:]
    return out


def pub_functions(root: Path, dirs) -> dict:
    """{name: (file, line)} of every `pub` and `pub(crate)` fn in the .rs files of the first existing dir of dirs."""
    out = {}
    for d in dirs:
        if not (root / d).is_dir():
            continue
        for p in sorted((root / d).rglob("*.rs")):
            rel = p.relative_to(root).as_posix()
            text = without_cfg_test(strip_rust_comments(p.read_text(encoding="utf-8")))
            for m in PUB_FN_RX.finditer(text):
                out.setdefault(m.group(1), (rel, text.count("\n", 0, m.start()) + 1))
    return out


def module_exists(root: Path, dirs) -> bool:
    return any((root / d).is_dir() for d in dirs)


def test_functions(root: Path) -> dict:
    """{name: line} of every `#[test]` fn of TESTS (its attributes directly above it)."""
    p = root / TESTS
    if not p.is_file():
        return {}
    out = {}
    attrs = []
    for i, line in enumerate(strip_rust_comments(p.read_text(encoding="utf-8")).split("\n"), 1):
        s = line.strip()
        if not s:
            continue
        if s.startswith("#["):
            attrs.append(s)
            continue
        m = FN_RX.match(line)
        if m and any(re.match(r"#\[\s*test\s*\]", a) for a in attrs):
            out.setdefault(m.group(1), i)
        attrs = []
    return out


# ------------------------------------------------------------------------------------------------ the calls

@dataclass
class Call:
    function: str
    args: list            # per argument: a literal name (enum variant or keyed name) or None
    file: str
    line: int
    generations: set      # the generations that use it
    preset: bool          # a preset file's call (else a fact's)


def parse_calls(expr: str) -> list:
    """[(function, [literal name or None per argument])] of every call in an S2.2 expression (`= …` stripped)."""
    toks = []
    for m in check_facts.TOKEN_RX.finditer(expr):
        toks.append((m.lastgroup, m.group(m.lastgroup)))
    out = []
    pos = [0]

    def take():
        t = toks[pos[0]] if pos[0] < len(toks) else (None, None)
        pos[0] += 1
        return t

    def peek():
        return toks[pos[0]] if pos[0] < len(toks) else (None, None)

    def parse():
        """Parse one expression; return its literal name or None."""
        kind, text = take()
        if kind != "path":
            if kind in ("hex", "num"):
                return None
            raise ValueError(text)
        if peek() == ("p", "("):
            take()
            args = []
            while True:
                if peek() == ("p", ")"):
                    take()
                    break
                args.append(parse())
                k, v = take()
                if v == ")":
                    break
                if v != ",":
                    raise ValueError(v)
            if text not in check_facts.LITERAL_FORMS and text != "active":
                out.append((text, args))
            return None
        if "." not in text and text not in check_facts.RESERVED:
            return text
        return None

    try:
        parse()
    except ValueError:
        return out
    return out


def expressions(value, line_of=None):
    """Every `= …` string in a TOML value (strings, lists, tables), the `=` stripped."""
    if isinstance(value, str):
        return [value.strip()[1:]] if value.strip().startswith("=") else []
    if isinstance(value, list):
        return [e for v in value for e in expressions(v)]
    if isinstance(value, dict):
        return [e for v in value.values() for e in expressions(v)]
    return []


def collect_calls(ctx, platform_functions: set) -> list:
    """Every call of a platform function in the facts and the new model's presets."""
    out = []
    items, _, _ = check_facts.load(ctx)
    for it in items:
        if it.table != "fact" or it.palette:
            continue
        gens, _ = ctx.covered(it)
        for expr in expressions(it.data.get("value")):
            for fn, args in parse_calls(expr):
                if fn in platform_functions:
                    out.append(Call(fn, args, it.file, it.ln("value"), set(gens), False))
    for d in check_facts.PRESET_DIRS:
        pd = ctx.root / d
        if not pd.is_dir():
            continue
        for p in sorted(pd.glob("*.toml")):
            rel = p.relative_to(ctx.root).as_posix()
            text = p.read_text(encoding="utf-8")
            try:
                data = tomllib.loads(text)
            except tomllib.TOMLDecodeError:
                continue   # the preset's own test refuses it
            gens = {p.stem} if p.stem in check_facts.ALL_PRESETS else set()
            for expr in expressions(data):
                at = text.find(expr)
                line = text.count("\n", 0, at) + 1 if at >= 0 else 1
                for fn, args in parse_calls(expr):
                    if fn in platform_functions:
                        out.append(Call(fn, args, rel, line, gens, True))
    return out


# ------------------------------------------------------------------------------------------------ the checks

class Context:
    """check_facts.Context (sources, core enums and functions, facts, presets) plus this script's reads."""

    def __init__(self, root=Path("."), **kw):
        self.fc = check_facts.Context(root, **kw)
        self.root = self.fc.root
        self.sources = self.fc.sources
        self.enums = {k: list(v) for k, v in self.fc.enums.items()}
        self.platform_functions = set(self.fc.functions) - ARITHMETIC


def served(t: Table, row: dict, ctx) -> set:
    """The generations a row serves (S2.2: its generator's pin, its citation or its evidence)."""
    route = t.route
    if route in GENERATED:
        ids = [row["source"]] if "source" in row else t.sources
        gens = set()
        for sid in ids:
            g = ctx.sources.get(sid, {}).get("generations", [])
            gens |= {x for x in g if isinstance(x, str)} if isinstance(g, list) else set()
        return gens
    if route == "published":
        return ctx.fc.cite_generations({"kind": "published", "source": row.get("cite")})
    if route == "measured":
        return ctx.fc.cite_generations({"kind": "measured", "evidence": row.get("evidence")})
    return set()


def generation_findings(t: Table, calls: list, ctx) -> list:
    out = []
    if t.findings or not t.rows:
        return out   # the form first
    header_gens = set(t.generations)
    rows = [(ln, row, served(t, row, ctx)) for ln, row in t.rows]
    for ln, row, gens in rows:
        if t.stem.startswith(PLACE) and row.get("preset") in check_facts.ALL_PRESETS and row["preset"] not in gens:
            out.append((ln, f"the row's preset `{row['preset']}` is not among the generations it serves "
                            f"({', '.join(sorted(gens)) or 'none'})"))
    for g in t.generations:
        if not any(g in gens for _, _, gens in rows):
            out.append((t.line_of("generations"), f"no row serves `{g}`"))
    inputs = t.inputs
    for call in calls:
        missing = sorted(call.generations - header_gens)
        if missing:
            out.append((t.line_of("generations"), f"{call.file}:{call.line} calls `{call.function}` for "
                                                  f"{', '.join(missing)}, which `# generations:` does not list"))
        if len(call.args) > len(inputs):
            out.append((t.line_of("inputs"), f"{call.file}:{call.line} calls `{call.function}` with "
                                             f"{len(call.args)} arguments, the table has {len(inputs)} inputs"))
            continue
        # the enum columns the call's arguments select, each with the values it uses
        choices = []
        for k, arg in enumerate(call.args):
            col = inputs[k]
            typ = base_type(col.type)
            if not (enum_of(col.type) or typ == "name"):
                continue
            if arg is not None:
                choices.append((col.name, [arg]))
            elif enum_of(col.type) in ctx.enums:
                choices.append((col.name, ctx.enums[enum_of(col.type)]))
        for combo in product(*[[(name, v) for v in vals] for name, vals in choices]):
            match = [(ln, gens) for ln, row, gens in rows if all(row.get(n) == v for n, v in combo)]
            what = ", ".join(f"{n} = {v}" for n, v in combo)
            for g in sorted(call.generations):
                if not any(g in gens for _, gens in match):
                    out.append((t.line_of("generations"), f"{call.file}:{call.line} calls `{call.function}` for "
                                                          f"`{g}`{' with ' + what if what else ''}: no row serves it"))
                for ln, gens in match:
                    if g not in gens:
                        out.append((ln, f"the row{' (' + what + ')' if what else ''} does not serve `{g}`, which "
                                        f"{call.file}:{call.line} calls `{call.function}` for"))
    return sorted(set(out))


def check(ctx):
    """(findings as (file, line, what), info lines, counts)."""
    root = ctx.root
    findings, info = [], []
    tabs, stray = tables(root, ctx)
    findings += stray
    by_stem = {t.stem: t for t in tabs}
    for t in tabs:
        findings += [(t.file, ln, w) for ln, w in t.findings]
    pub = pub_functions(root, PLATFORM_DIRS)
    tests = test_functions(root)
    required = {}   # table stem -> (why, file, line)
    for fn, (f, ln) in pub.items():
        required[fn] = (f"`pub` function `{fn}` of the platform module", f, ln)
    strategies = []
    if module_exists(root, PLACEMENT_DIRS):
        strategies = ctx.enums.get("PlacementStrategy", [])
        for s in strategies:
            required[PLACE + s] = (f"placement strategy `{s}` (S13)", next(
                (d.as_posix() for d in PLACEMENT_DIRS if (root / d).is_dir())), 1)
    for stem, (why, f, ln) in sorted(required.items()):
        if stem not in by_stem:
            findings.append((f, ln, f"{why} has no vector table {(VECTORS / (stem + '.tsv')).as_posix()}"))
        test = "vectors_" + stem.replace(".", "_")
        if test not in tests:
            findings.append((f, ln, f"{why} has no test `#[test] fn {test}` in {TESTS.as_posix()}"))
    waiting = []
    for stem, t in sorted(by_stem.items()):
        if stem in required:
            continue
        if stem in ARITHMETIC:
            findings.append((t.file, 1, f"`{stem}` is the model's own arithmetic: unit tests from S2.2's definition, "
                                        f"no vector table"))
        elif stem in ctx.platform_functions:
            waiting.append(stem)
        else:
            findings.append((t.file, 1, f"`{stem}` is no platform function of S2.2's table, `pub` function of the "
                                        f"platform module or placement strategy"))
    expected_tests = {"vectors_" + s.replace(".", "_") for s in set(required) | set(waiting)}
    for name, ln in sorted(tests.items()):
        if name.startswith("vectors_") and name not in expected_tests:
            findings.append((TESTS.as_posix(), ln, f"test `{name}` has no implemented function or strategy with a "
                                                   f"vector table"))
    calls = collect_calls(ctx.fc, ctx.platform_functions)
    called = {}
    for c in calls:
        called.setdefault(c.function, []).append(c)
    for t in tabs:
        findings += [(t.file, ln, w) for ln, w in generation_findings(t, called.get(t.stem, []), ctx)]
    unimplemented = []
    for fn, cs in sorted(called.items()):
        if fn in pub:
            continue
        by_preset = [c for c in cs if c.preset]
        if by_preset:
            for c in by_preset:
                findings.append((c.file, c.line, f"calls `{fn}`, a platform function not implemented in the "
                                                 f"platform module (S2.2)"))
        else:
            first = cs[0]
            files = sorted({c.file for c in cs})
            unimplemented.append(f"{fn} ({len(cs)} call{'s' if len(cs) != 1 else ''} in "
                                 f"{', '.join(files)}; first {first.file}:{first.line})")
    if not pub and not strategies:
        info.append("no platform function implemented and no placement module yet: the table and test rules pass "
                    "vacuously")
    if unimplemented:
        info.append("called by facts, not yet implemented (a finding once a preset calls it): "
                    + "; ".join(unimplemented))
    if waiting:
        info.append("tables waiting for their implementation: " + ", ".join(waiting))
    counts = {"tables": len(tabs), "functions": len(pub), "strategies": len(strategies), "tests": len(tests),
              "calls": len(calls)}
    return sorted(set(findings)), info, counts


def summary(counts, n):
    return (f"{counts['tables']} vector tables, {counts['functions']} platform functions implemented, "
            f"{counts['strategies']} placement strategies, {counts['tests']} tests, {counts['calls']} platform-function "
            f"calls in facts and presets: {n} findings")


# --------------------------------------------------------------------------------------------- self-test

SELF_TEST_CORE = check_facts.SELF_TEST_CORE.replace(
    "pub enum Scheme { Light, Dark }",
    "pub enum Scheme { Light, Dark }\npub enum TrackingDesign { SfPro, NewYork }",
).replace(
    "| `kcu_mix(a, b, bias)`, `kcu_tint(a, b, amount)` | colour | KColorUtils |",
    "| `kcu_mix(a, b, bias)`, `kcu_tint(a, b, amount)` | colour | KColorUtils |\n"
    "| `adw_standalone(c, scheme)` | colour | libadwaita |\n"
    "| `sf_tracking(font_size, design: TrackingDesign)` | length | Apple's tables |\n"
    "| `m3_role(role)` | colour | material-color-utilities |\n"
    "| `mul(x, y)` | number | arithmetic |",
)

SELF_TEST_RUN = '''
[images.os-probe-macos-26]
preset = "macos-tahoe"
source = "xcode@26.6"
os = "25A1"
'''

ADW_TABLE = '''# function: adw_standalone
# route: public_api
# source: libadwaita@1.10.0
# generations: adwaita
# generator: tools/gnome-vectors
# inputs: c:colour scheme:enum<Scheme>
# outputs: result:colour
# the grid of spike G, two rows here
c\tscheme\tresult
#3584e4\tlight\tsrgb(0.0117647061, 0.392156869, 0.835294127, 1)
#3584e4\tdark\tsrgb(0.470588237, 0.682352960, 0.929411769, 1)
'''

SF_TABLE = '''# function: sf_tracking
# route: published
# source: hig-typography@2026-10-29
# generations: macos-tahoe
# inputs: font_size:font_size design:enum<TrackingDesign>
# outputs: tracking:length
font_size\tdesign\ttracking\tcite\tquote
17px\tsf_pro\t-0.43px\thig-typography@2026-10-29 #/sections/3/rows/9\t17 -0.43
17px\tnew_york\t0px\thig-typography@2026-10-29 #/sections/4/rows/9\t17 0
'''

PLACE_TABLE = '''# function: place.below
# route: measured
# generations: macos-tahoe
# inputs: anchor:rect size:size window:rect pointer:option<point>
# outputs: frame:rect
# tolerance: frame 0.5
anchor\tsize\twindow\tpointer\tframe\tevidence\tpreset\tvalue\twhen
[10, 10, 80, 24]\t[120, 60]\t[0, 0, 800, 600]\tnone\t[10, 34, 120, 60]\tdocs/evidence/os-probe/2026-10-30-run-1/os-probe-macos-26/place.toml#below.0\tmacos-tahoe\tcombo_box.values.popup_placement\t{ variants = ["@pull_down"] }
'''

SELF_TEST_FILES = {
    "native-theme/src/next/platform/mod.rs": "//! The platform functions.\npub mod adw_standalone;\npub mod sf_tracking;\n",
    "native-theme/src/next/platform/adw_standalone.rs": '''//! libadwaita's standalone accent.
/// The standalone colour (`pub fn not_this()` in a doc comment is no function).
pub fn adw_standalone(c: Color, scheme: Scheme) -> Color {
    to_oklab(c)
}

fn to_oklab(c: Color) -> Color {
    c
}

#[cfg(test)]
mod tests {
    pub fn helper_in_tests() {}
}
''',
    "native-theme/src/next/platform/sf_tracking.rs": "pub(crate) fn sf_tracking(size: f32, design: TrackingDesign) -> f32 {\n    0.0\n}\n",
    "native-theme/src/next/placement/mod.rs": "pub fn place() {}\n",
    "native-theme/tests/platform_vectors.rs": '''//! The vector tests.
#[test]
fn vectors_adw_standalone() {}

#[test]
#[allow(clippy::float_cmp)]
fn vectors_sf_tracking() {}

#[test]
fn vectors_place_below() {}

fn vectors_helper_not_a_test() {}
''',
    "native-theme/tests/vectors/adw_standalone.tsv": ADW_TABLE,
    "native-theme/tests/vectors/sf_tracking.tsv": SF_TABLE,
    "native-theme/tests/vectors/place.below.tsv": PLACE_TABLE,
    "docs/evidence/os-probe/2026-10-30-run-1/run.toml": SELF_TEST_RUN,
    "docs/facts/gnome/roles.toml": '''
[[fact]]
key = "role.global.accent"
value = "= adw_standalone(role.global.accent, scheme)"
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/stylesheet/_colors.scss:1"
quote = "$accent_color: oklab(from $accent_bg_color …);"
''',
    "docs/facts/material/roles.toml": '''
[[fact]]
key = "role.global.accent"
value = "= m3_role(primary)"
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/x.scss:1"
quote = "x"
''',
    "native-theme/src/next/presets/adwaita.toml": '''
[colors]
accent = "= adw_standalone(input.accent, scheme)"
fill = "= mul(token.x, 2)"
''',
    "native-theme/src/next/presets/macos-tahoe.toml": '''
[tokens.text.keyed.body]
letter_spacing_px = "= sf_tracking(font_size, sf_pro)"
''',
}

KDE_ROLES = '''
[[fact]]
key = "role.window.background"
value = "= kirigami_lerp(role.window.background, kde_shade(role.window.foreground, light, 4), 0.2)"
rows = []

[[fact.cite]]
kind = "published"
source = "breeze@6.8.2 x.cpp:1"
quote = "x"
'''


def write_fixture(root: Path, files: dict, presets=("adwaita", "macos-tahoe")):
    for rel, text in files.items():
        if text is None:
            continue
        p = root / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text, encoding="utf-8")
    (root / check_facts.SOURCES).parent.mkdir(parents=True, exist_ok=True)
    (root / check_facts.SOURCES).write_text(check_facts.SELF_TEST_SOURCES, encoding="utf-8")
    pdir = root / check_facts.PRESET_DIRS[0]
    pdir.mkdir(parents=True, exist_ok=True)
    for p in presets:
        if not (pdir / f"{p}.toml").exists():
            (pdir / f"{p}.toml").write_text("", encoding="utf-8")


def fixture_context(root: Path):
    return Context(root, core_text=SELF_TEST_CORE, cat_text=check_facts.SELF_TEST_CATALOGUE,
                   sources_text=check_facts.SELF_TEST_SOURCES, rows=check_facts.SELF_TEST_ROWS)


ADW = "native-theme/tests/vectors/adw_standalone.tsv"
SF = "native-theme/tests/vectors/sf_tracking.tsv"
PLC = "native-theme/tests/vectors/place.below.tsv"
RS = "native-theme/src/next/platform/adw_standalone.rs"
TS = "native-theme/tests/platform_vectors.rs"
TAHOE = "native-theme/src/next/presets/macos-tahoe.toml"
ADWP = "native-theme/src/next/presets/adwaita.toml"

# (label, file, old, new, the findings wanted: substrings, one per finding); new None with old None deletes the file
SELF_TEST_CASES = [
    ("coverage: an implemented function without a table", ADW, None, None,
     ["has no vector table native-theme/tests/vectors/adw_standalone.tsv"]),
    ("coverage: an implemented function without a test", TS, "fn vectors_adw_standalone()", "fn vectors_adw()",
     ["has no test `#[test] fn vectors_adw_standalone`", "test `vectors_adw` has no implemented function"]),
    ("coverage: a test without #[test]", TS, "#[test]\nfn vectors_adw_standalone", "fn vectors_adw_standalone",
     ["has no test `#[test] fn vectors_adw_standalone`"]),
    ("coverage: a pub(crate) helper needs a table and a test", RS, "\nfn to_oklab", "\npub(crate) fn to_oklab",
     ["`pub` function `to_oklab` of the platform module has no vector table",
      "`pub` function `to_oklab` of the platform module has no test"]),
    ("coverage: a placement strategy without a table", PLC, None, None,
     ["placement strategy `below` (S13) has no vector table"]),
    ("coverage: a table of the model's arithmetic", "native-theme/tests/vectors/mix.tsv", None,
     ADW_TABLE.replace("adw_standalone", "mix"), ["`mix` is the model's own arithmetic"]),
    ("coverage: a table of no function", "native-theme/tests/vectors/blur.tsv", None,
     ADW_TABLE.replace("adw_standalone", "blur"), ["`blur` is no platform function"]),
    ("coverage: a stray file", "native-theme/tests/vectors/notes.txt", None, "x",
     ["not a vector table"]),
    ("calls: a preset calls a function not implemented", TAHOE, "sf_tracking(font_size, sf_pro)",
     "sf_tracking(m3_role(primary), sf_pro)", ["calls `m3_role`, a platform function not implemented"]),
    ("form: a misspelled header field", ADW, "# generator:", "# generater:",
     ["`# generater:` is no header field", "names its `# generator:`"]),
    ("form: a header field twice", ADW, "# route: public_api", "# route: public_api\n# route: public_api",
     ["`# route:` twice"]),
    ("form: the function is not the stem", ADW, "# function: adw_standalone", "# function: adw",
     ["is not the file's stem"]),
    ("form: an unknown route", ADW, "public_api", "public", ["route `public` is not one of"]),
    ("form: an unknown source", ADW, "# source: libadwaita@1.10.0", "# source: libadwaita@1.9",
     ["source `libadwaita@1.9` is no [[source]]"]),
    ("form: a generated table without a source", ADW, "# source: libadwaita@1.10.0\n", "",
     ["names its reference's `# source:` entries"]),
    ("form: a published table with a generator", SF, "# route: published", "# route: published\n# generator: x",
     ["a `published` table has no generator"]),
    ("form: a bad column type", ADW, "c:colour", "c:color", ["`c:color`: `color` is not a column type"]),
    ("form: a column without a type", ADW, "c:colour", "c", ["`c` is not `<name>:<type>`"]),
    ("form: a generated output colour rounded", ADW, "srgb(0.0117647061, 0.392156869, 0.835294127, 1)",
     "#0364d5", ["a generated output colour is written unrounded"]),
    ("form: a bad colour cell", ADW, "#3584e4\tlight", "#3584e\tlight", ["`#3584e` is not a colour literal"]),
    ("form: an enum cell outside the core's enum", ADW, "#3584e4\tdark", "#3584e4\tdim",
     ["`dim` is not a variant of Scheme"]),
    ("form: a length without its unit", SF, "17px\tsf_pro", "17\tsf_pro", ["`17` is not a font size (`<n>px`)"]),
    ("form: a wrong cell count", ADW, "\tdark\t", "\tdark\t\t", ["4 cells, not 3"]),
    ("form: the column header row", ADW, "c\tscheme\tresult", "c\tresult\tscheme", ["column header row"]),
    ("form: no row", SF, SF_TABLE.split("font_size\tdesign")[1].split("\n", 1)[1], "", ["no row"]),
    ("form: a published row's source not in the header", SF, "0px\thig-typography@2026-10-29 #/",
     "0px\txcode@26.6 #/", ["`xcode@26.6` is not one of the header's `# source:` entries"]),
    ("form: a bad conversion", SF, "font_size\tdesign\ttracking\tcite\tquote\n17px\tsf_pro\t-0.43px\t"
     "hig-typography@2026-10-29 #/sections/3/rows/9\t17 -0.43\n17px\tnew_york\t0px\thig-typography@2026-10-29 "
     "#/sections/4/rows/9\t17 0",
     "font_size\tdesign\ttracking\tcite\tquote\tconversion\n17px\tsf_pro\t-0.43px\t"
     "hig-typography@2026-10-29 #/sections/3/rows/9\t17 -0.43\t-\n17px\tnew_york\t0px\thig-typography@2026-10-29 "
     "#/sections/4/rows/9\t17 0\tpercent", ["conversion ['percent']"]),
    ("form: a measured output without tolerance", PLC, "# tolerance: frame 0.5\n", "",
     ["measured output `frame` has no `# tolerance:` line"]),
    ("form: a tolerance on a generated table", ADW, "# outputs: result:colour",
     "# outputs: result:colour\n# tolerance: result 1", ["a `public_api` table states no tolerance"]),
    ("form: a bad evidence path", PLC, "docs/evidence/os-probe/2026-10-30-run-1/", "docs/evidence/os-probe/run-1/",
     ["is not `docs/evidence/"]),
    ("form: a bad rect", PLC, "[10, 34, 120, 60]", "[10, 34, 120]", ["is not a JSON array of 4 numbers (rect)"]),
    ("form: a bad when", PLC, '{ variants = ["@pull_down"] }', '{ variant = ["@pull_down"] }',
     ["`when` key(s) ['variant']"]),
    ("form: a bad placed value", PLC, "combo_box.values.popup_placement", "combo_box.popup_placement",
     ["is not `<widget>.values.<name>`"]),
    ("generations: a placement row for another preset", PLC, "\tmacos-tahoe\tcombo_box", "\tadwaita\tcombo_box",
     ["the row's preset `adwaita` is not among the generations it serves"]),
    ("generations: the generator's pin serves another generation", ADW, "# source: libadwaita@1.10.0",
     "# source: breeze@6.8.2", ["no row serves `adwaita`"]
     + ["calls `adw_standalone` for `adwaita` with scheme = light: no row serves it"] * 2
     + ["calls `adw_standalone` for `adwaita` with scheme = dark: no row serves it"] * 2
     + ["the row (scheme = light) does not serve `adwaita`"] * 2
     + ["the row (scheme = dark) does not serve `adwaita`"] * 2),
    ("generations: a calling generation not listed", ADW, "# generations: adwaita", "# generations: kde-breeze",
     ["calls `adw_standalone` for adwaita, which `# generations:` does not list",
      "calls `adw_standalone` for adwaita, which `# generations:` does not list", "no row serves `kde-breeze`"]),
    ("generations: a variant the call selects has no row", ADW,
     "#3584e4\tdark\tsrgb(0.470588237, 0.682352960, 0.929411769, 1)\n", "",
     ["with scheme = dark: no row serves it", "with scheme = dark: no row serves it"]),
    ("generations: a literal enum argument selects its rows", TAHOE, "sf_pro)", "new_york)", []),
    ("generations: a literal enum argument without rows", SF,
     "17px\tnew_york\t0px\thig-typography@2026-10-29 #/sections/4/rows/9\t17 0\n", "", []),
]


def self_test() -> int:
    failures = []
    with tempfile.TemporaryDirectory() as tmp:
        base = Path(tmp)
        n = [0]

        def run(files, presets=("adwaita", "macos-tahoe")):
            root = base / f"case{n[0]}"
            n[0] += 1
            write_fixture(root, files, presets)
            got, info, counts = check(fixture_context(root))
            return [f"{f}:{ln}: {w}" for f, ln, w in got], info, counts

        got, info, counts = run(SELF_TEST_FILES)
        if got:
            failures.append("good fixtures: got " + "; ".join(got))
        if not any("m3_role (1 call in docs/facts/material/roles.toml" in i for i in info):
            failures.append(f"good fixtures: m3_role not reported as called, not implemented: {info}")
        if counts != {"tables": 3, "functions": 2, "strategies": 1, "tests": 3, "calls": 4}:
            failures.append(f"good fixtures: counts {counts}")
        for label, path, old, new, want in SELF_TEST_CASES:
            files = dict(SELF_TEST_FILES)
            if old is None:
                files[path] = new
            else:
                if old not in files[path]:
                    failures.append(f"{label}: fixture text {old!r} not found")
                    continue
                files[path] = files[path].replace(old, new, 1)
            got, _, _ = run(files)
            unmatched = list(got)
            ok = len(got) == len(want)
            for w in want:
                hit = next((g for g in unmatched if w in g), None)
                if hit is None:
                    ok = False
                else:
                    unmatched.remove(hit)
            if not ok:
                failures.append(f"{label}: got {got}, want {want}")
        # a literal enum argument selects only its rows: with new_york's row gone and the preset calling new_york
        files = dict(SELF_TEST_FILES)
        files[SF] = files[SF].replace("17px\tnew_york\t0px\thig-typography@2026-10-29 #/sections/4/rows/9\t17 0\n", "")
        files[TAHOE] = files[TAHOE].replace("sf_pro)", "new_york)")
        got, _, _ = run(files)
        if len(got) != 1 or "with design = new_york: no row serves it" not in got[0]:
            failures.append(f"literal enum argument without rows, called: got {got}")
        # the vacuous run: nothing implemented, facts calling a function
        files = {k: v for k, v in SELF_TEST_FILES.items() if not k.startswith("native-theme/")}
        got, info, counts = run(files, presets=())
        if got or not any("pass vacuously" in i for i in info) or not any(
                i.startswith("called by facts, not yet implemented") and "adw_standalone" in i and "m3_role" in i
                for i in info):
            failures.append(f"vacuous: got {got}, {info}")
        if summary(counts, 0) != ("0 vector tables, 0 platform functions implemented, 0 placement strategies, 0 "
                                  "tests, 2 platform-function calls in facts and presets: 0 findings"):
            failures.append(f"vacuous summary: {summary(counts, 0)}")
        # ADD-A121's KDE functions, called by facts and not implemented: listed, not findings
        kde = dict(files)
        kde["docs/facts/kde/roles.toml"] = KDE_ROLES
        got, info, _ = run(kde, presets=())
        if got or not any(i.startswith("called by facts, not yet implemented") and "kde_shade (1 call" in i
                          and "kirigami_lerp (1 call" in i for i in info):
            failures.append(f"kde_shade and kirigami_lerp called by facts: got {got}, {info}")
        # a table waiting for its implementation is reported, not a finding
        files = dict(files)
        files[ADW] = ADW_TABLE
        got, info, _ = run(files, presets=())
        if got or "tables waiting for their implementation: adw_standalone" not in info:
            failures.append(f"waiting table: got {got}, {info}")
        # the API
        root = base / "case0"
        ev = table_evidence(root)
        if ev != {"docs/evidence/os-probe/2026-10-30-run-1/os-probe-macos-26/place.toml"}:
            failures.append(f"table_evidence: got {ev}")
        if parse_calls(" adw_standalone(mix(role.x, #fff, 0.5), scheme)") != [("mix", [None, None, None]),
                                                                              ("adw_standalone", [None, None])]:
            failures.append(f"parse_calls: got {parse_calls(' adw_standalone(mix(role.x, #fff, 0.5), scheme)')}")
        if parse_calls(" sf_tracking(font_size, sf_pro)") != [("sf_tracking", [None, "sf_pro"])]:
            failures.append(f"parse_calls literal: got {parse_calls(' sf_tracking(font_size, sf_pro)')}")
        if parse_calls(" active(role.window.background)") != [] or parse_calls(" srgb(1, 0, 0)") != []:
            failures.append("parse_calls: active() and srgb() are no calls")
        if sorted(pub_functions(root, PLATFORM_DIRS)) != ["adw_standalone", "sf_tracking"]:
            failures.append(f"pub_functions: got {sorted(pub_functions(root, PLATFORM_DIRS))}")
        if sorted(test_functions(root)) != ["vectors_adw_standalone", "vectors_place_below", "vectors_sf_tracking"]:
            failures.append(f"test_functions: got {sorted(test_functions(root))}")
        ctx = fixture_context(root)
        if ctx.platform_functions != {"kcu_mix", "kcu_tint", "kde_shade", "kirigami_lerp", "adw_standalone",
                                      "sf_tracking", "m3_role"}:
            failures.append(f"platform functions: got {sorted(ctx.platform_functions)}")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print(f"self-test ok ({len(SELF_TEST_CASES) + 5} fixture cases)")
    return 1 if failures else 0


def main(argv) -> int:
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    if args:
        print(__doc__, file=sys.stderr)
        return 2
    try:
        ctx = Context(Path("."))
        findings, info, counts = check(ctx)
    except (OSError, UnicodeDecodeError, check_facts.InputError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for f, ln, what in findings:
        print(f"{f}:{ln}: {what}")
    for line in info:
        print(f"# {line}")
    print(summary(counts, len(findings)), file=sys.stderr)
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
