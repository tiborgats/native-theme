#!/usr/bin/env python3
"""A preset's rules generated from its verified facts, as far as they transcribe (specification S10, S11, S17; the
0.8.0 plan's checkpoint D6: "whether each unit's preset rules are generated from its verified facts instead of
written … decided by how closely the slice's presets transcribe their facts"). Offline.

Reads the facts as scripts/check_facts.py loads them (`docs/facts/<platform>/*.toml` and its `state-models.toml`;
`imported/` and `staged/` are no facts), the core and the widget catalogue through it, and the enabled units as
scripts/check_facts_trace.py reads them (`scripts/widget_units.toml`, until it exists the slice). Writes a draft of the
preset's TOML (S11): `[preset]`, `[declare]`, the bases and `[[rules]]`. A draft: the preset's own task places and
completes it, and the Rust reader validates it there.

Run from the repository root:

    python3 scripts/generate_preset.py <preset-id> [--out FILE]   # the draft to FILE (default stdout)
    python3 scripts/generate_preset.py --self-test

On stderr: the transcription share (facts transcribed, shadowed, pending, left out by reason; per widget and per
unit), the rule and base-value counts, the pending facts' register ids, and the round trip's result. Exit 0, 1 when
the round trip finds a mismatch (a generator bug), 2 on an input it cannot read or a wrong command line.

Which facts (S17 *Generations*)
-------------------------------
The facts of the directory that covers the preset (check_facts.PLATFORMS) whose `when` names `preset:<id>` in
`conditions`, or names no preset there, and does not name `preset:<id>` in `not_conditions`. Their `when` loses
`preset:<id>` and every `not_conditions` preset (they hold for this preset), and a list emptied so goes.

What is left out, and why (each counted by reason)
--------------------------------------------------
- pending: a pending fact (S17 *A pending measurement*) states nothing; its id is listed, as the gate lists it.
- not held: a key the model does not hold yet (S17 *The gate*: the plan enables its units one by one): a key outside
  every enabled unit, or deferred or staged there (check_facts_trace.in_unit). An `input.…` key is held (S9.1's
  inputs are in M1a's code).
- unit not enabled: a fact whose `when` names a widget the model does not hold (a `context` or `not_context` entry,
  a `nearest` scope, a position's `among`, `previous`, `next`), counted by that widget (its unit): the reader's S12
  checks refuse a rule naming an undeclared widget, and the fact comes back when its unit is enabled and the preset
  is regenerated (check_facts_trace.py's rule for trace lines).
- read: a fact with a `read` citation (S17 *`read`*: the reference a key takes from a reader input), which D6 lists
  among the parts written by hand.
- accent run: a fact with `scheme_file` and `accent` (S17 *The accent run*), which the preset's own resolution never
  reaches.
- two files: a key whose facts lie in two files, whose order S17 *Which fact applies* does not give.
- an S11 spelling a fact alone does not give (spell()): a `Scalar` entry stated with a unit (S11: "its number in the
  unit its declaration names", which no fact states); `"absent"`, `"auto"` or `"none"` on a `Radius` corner (S11
  names `<corner>_px` for `Radius::Px` and the bare name for its other variants, neither for a `Value` keyword); a
  unit literal of another dimension than the key's, or under a key whose type the core does not give (its suffix is
  unknown); a keyed segment (`@…`) outside `role.@…`; a whole block and a leaf of it both in one base table (S11 states
  a block in a table whole or per leaf; S2.2 composes a base's `Fields` patch over a reference before every rule,
  which a rule cannot state, so neither form is the fact's).
- an input whose `when` is not a base (S9: an input has per-scheme and per-contrast bases and no rules): anything but
  at most one `scheme:` and one of `contrast_increased`, `contrast_high` in `conditions`; and a per-contrast base one
  of whose schemes no transcribed fact gives (S11: `[inputs.<name>.contrast.<c>]` states `light` and `dark`).
- shadowed: a fact that never applies, as a later fact of its key holds wherever it does (covers(): the later `when`'s
  every term is one of the earlier's), so the preset needs no rule for it. Counted apart: it is transcribed exactly by
  stating nothing.

S19.2's staged bases (`docs/facts/staged/inheritance-bases.toml`) are no facts: their count for the preset is printed
as left for hand-writing.

Bases and rules (S3.1, S8, S11)
-------------------------------
For every key, the facts in file order; the last whose `when` holds applies (S17 *Which fact applies*), and the
preset's bases apply before its rules, its rules in stated order, the later winning (S8 *Layers and order*). So:

- A fact's slots are the schemes it can hold under (both, or the one its `scheme:` condition names). It is alive in a
  slot where no later fact of its key covers it there (a later fact holding wherever it holds in that scheme), and
  shadowed where it is alive nowhere.
- The rest combination is the `when` that holds in every combination of a slot: `{}`, and for a colour role or an
  input `{ conditions = ["scheme:<s>"] }` too, as `colors.light` and `colors.dark` are the two bases (S3.1) and an
  input's bases are per scheme (S9). A colour role's fact at rest becomes the base value of each slot it is alive in
  (`[colors.<s>.…]`); a widget's or a token's fact at rest becomes the base value
  (`[widgets.<w>.<part>]`, `[widgets.<w>.values]`, `[components.<key>.<part>]`, `[tokens.<group>]`). The last fact at
  rest of a slot covers every earlier fact there, so every fact still alive comes after it, as a rule after its base.
- Every other alive fact becomes a rule. One alive in fewer slots than its `when` admits (a later scheme's base, or a
  later `scheme:` fact, covers it in the other) takes that slot's `scheme:` condition into its `conditions`
  (narrowed), so it holds exactly where it applied.
- Facts with an equal `when` (check_facts.canonical_when) and an equal target group into one rule: a fact joins the
  last rule with its `when` and target unless a later rule states its key or a key above or below it (a block and its
  leaves), or the rule's `set` already holds a key above or below it in TOML, and else starts a new rule at the end.
  So for every key the rules state its facts in their file order, and the last matching rule is the last matching
  fact (the round trip below checks it).
- Inputs (S9): the six cells (light, dark; no contrast, `contrast_increased`, `contrast_high`) each take the fact that
  applies there; `light` and `dark` the no-contrast cells', `contrast.<increased|high>` both schemes' where either
  differs from the no-contrast fact.

Targets and paths (S11): `role.<set>.<role>` is `surfaces.<set>.<role>` of `colors` (`role.<set>.disabled.<role>`
below it), `role.global.<…>` is `global.<…>`, `role.@<key>` is `keyed.<key>` (S3's `ColorRoles.keyed`, a map whose
key is a `Key`, bare, S11); `token.<path>` is `<path>` of `tokens`; `<widget>.<part>.<field…>` is `<field…>` of
target `<widget>.<part>`, `<widget>.values.<name>` of `<widget>.values`, `component.<key>.…` likewise; a whole part's
`"absent"` is `absent = true` and its `{ drawn_as = … }` is `drawn_as = …` (S6.3, S11); `input.<name>` is
`[inputs.<name>]`.

Spelling (S11's table; spell()): a unit literal under a one-unit key is its number under the suffixed key
(`min_height_px = 24`, `_ms`, `_deg`), and any other value of it, an expression, `"auto"`, `"absent"`, `"none"`,
under the suffixed key too; a `FontSize` key takes `_pt` or `_px` by the literal's unit and the bare name otherwise; a
`Radius` corner `<corner>_px` for a length or an expression and the bare name for a variant (`"follow_part"`, a
variant table); a map (`Vec<(K, V)>`), a table in the fact as in the preset (S11: "a TOML table, one key per entry,
never a list of pairs"), has its entries spelled by `V` (`{ button = "200ms" }` as `button_ms = 200`); every other value as written (colours, expressions,
references, `"absent"`, `"auto"`, numbers, bools, lists, blocks as tables, which keep their own suffixed keys).

Layout: `[preset] id`; `[declare]`; the bases, `inputs`, `colors.light`, `colors.dark`, `tokens`, `widgets`,
`components`, a table per part, colour-role set (`surfaces.<set>`, `global`, `keyed`), token group or input, each
value a dotted key below it; then each rule as `[[rules]]` with `when` and `target` and its patch as `[rules.set]`, a
dotted key per value. Every value has its fact's `<file>:<line>` in a comment above it (S11: "Every value carries
its fact's citation in a comment"; the fact holds the citations).

`[declare]` (S10, S17 *State models*): per widget or component the model holds, its state-model entry for the
preset (one naming `preset:<id>` replaces the shared one; a `flavor:` entry is `[declare.flavor.<f>.widget.<w>]`),
the S10 declarations it states: `available` (alone where false), `variants`, `sizes`, `default_size`,
`default_size_by_variant`, `densities`, `flags`, `exclusive`, `exclusive_variants`, `recency`, `requires_variants`,
`sizes_by_variant`, `sizes_by_context`; `axes`, `variant_sets`, `variant_products` and `not_drawn` are the gate's,
not declarations. `[declare] conditions` lists the `system_setting:` keys the entries' axes and the facts' `when`s
name, as `@<key>`. The other keyed namespaces S10 declares (`color_roles`, `durations`, `timings`, `params` with their
units, `text_styles`, `inputs`, `accent_inputs`, `surface_sets`, …) are left for hand-writing: no fact states their
units or defaults.

The round trip (the exactness proof)
------------------------------------
The draft is read back with tomllib: each transcribed fact's value read at its TOML path (in its base table, or in
its rule's `set` below that rule's `when` and `target` as read back) and turned back into the fact's spelling
(unspell(): the suffix into the value's unit, a map's entries likewise, `absent = true` into `"absent"`); a value
at no transcribed fact's path is a mismatch. Then (1) for every key the read-back sequence (bases, then the rules in
order) equals the transcribed facts' sequence, `when` by `when` (canonical_when) and value by value; and (2) for
every key, every distinct `when` of the facts of its widget (or of the colour roles, the tokens, the inputs) and
each scheme, that `when`'s minimal combination (its required names present and nothing else; ancestors as its
contexts list them; a lone element where it names no position) resolves under the generated bases and rules to the
value of the fact that applies there among all the key's facts, or to none where none holds (the
last in file order, after the state model's exclusive reduction and its recency sets' stated order, S7.2, S10,
S17); a combination where a pending or a left-out fact applies is skipped and counted, and a fact whose `when` no
element can hold (two sizes, a required name refused, a state its group drops) is counted apart. Any difference is a
mismatch.

    python3 scripts/generate_preset.py --self-test   # fixture facts against the expected draft, and each rule
"""
import contextlib
import io
import re
import sys
import tempfile
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

import check_facts
import check_facts_trace
from generate_platform_facts import toml_inline, toml_key

SCHEMES = ("light", "dark")
CONTRASTS = (("contrast_increased", "increased"), ("contrast_high", "high"))
SUFFIX_OF = {"length": "_px", "duration": "_ms", "angle": "_deg"}
UNIT_OF_SUFFIX = {"_px": "px", "_ms": "ms", "_deg": "deg", "_pt": "pt"}
MAP_TYPE = re.compile(r"Vec<\(\s*(\w+)\s*,\s*(.+)\)>")
DECLARE_KEYS = ("available", "variants", "sizes", "default_size", "default_size_by_variant", "densities", "flags",
                "exclusive", "exclusive_variants", "recency", "requires_variants", "sizes_by_variant",
                "sizes_by_context")
STAGED_BASES = Path("docs/facts/staged/inheritance-bases.toml")
SECTIONS = ("inputs", "colors", "tokens", "widgets", "components")
LONE = {"first": True, "last": True, "only": True, "parity": "odd"}

PENDING = "pending"
LEFT_OUT = "left out"
SHADOWED = "shadowed"
NOT_HELD = "not held by the model"
UNIT_NOT_ENABLED = "unit not enabled"
READ = "read input"
ACCENT_RUN = "accent run"
TWO_FILES = "key in two files"
SCALAR_UNIT = "Scalar with a unit (its declaration's)"
RADIUS_KEYWORD = "Radius corner keyword"
UNIT_MISMATCH = "unit of another dimension"
UNTYPED_UNIT = "unit literal under an untyped key"
KEYED_SEGMENT = "keyed segment outside role.@"
BASE_CLASH = "block and its leaf in one base table"
INPUT_WHEN = "input under a when S9 gives no base"
INPUT_CONTRAST = "per-contrast input lacking a scheme"


class InputError(Exception):
    pass


class Untranscribable(Exception):
    pass


@dataclass
class Fact:
    item: object
    key: str
    ckey: tuple
    when: dict
    kind: str
    widget: object
    typ: object
    dim: object
    status: str = ""          # "base", "rule", SHADOWED, PENDING or LEFT_OUT
    reason: str = ""
    target: str = ""          # the rule target (S11)
    section: str = ""         # the base section (SECTIONS)
    header: tuple = ()        # the base table below its section (and scheme)
    rel: tuple = ()           # the TOML keys below the target, the last one spelled
    dotted: tuple = ()        # the TOML keys below the base table, the last one spelled
    value: object = None      # the TOML value; an input's: [(its keys below the table, the value)]
    slots: tuple = ()         # the schemes it is alive in
    base_slots: tuple = ()    # the schemes whose base it gives (a colour role's), or SCHEMES
    rule_when: object = None  # the `when` of its rule (its own, or narrowed to the scheme it is alive in)

    @property
    def where(self):
        return f"{self.item.file}:{self.item.line}"


@dataclass
class Rule:
    when: dict
    target: str
    facts: list = field(default_factory=list)
    tree: dict = field(default_factory=dict)   # its `set` as nested tables, to refuse a key above or below another


class Leaf:
    """A value's place in a nested table (insert())."""

    def __init__(self, fact):
        self.fact = fact


# --------------------------------------------------------------------------------------------- which facts

def platform_of(preset: str):
    return next((p for p, ps in check_facts.PLATFORMS.items() if preset in ps), None)


def holds_for(item, preset) -> bool:
    when = item.data.get("when") if isinstance(item.data.get("when"), dict) else {}
    conds = when.get("conditions", []) if isinstance(when.get("conditions"), list) else []
    nots = when.get("not_conditions", []) if isinstance(when.get("not_conditions"), list) else []
    named = [c[7:] for c in conds if isinstance(c, str) and c.startswith("preset:")]
    return all(p == preset for p in named) and f"preset:{preset}" not in nots


def preset_when(when, preset) -> dict:
    """The `when` for this preset: `preset:<id>` out of `conditions`, every preset out of `not_conditions`."""
    out = {}
    for k, v in (when if isinstance(when, dict) else {}).items():
        if k == "conditions" and isinstance(v, list):
            v = [c for c in v if c != f"preset:{preset}"]
        elif k == "not_conditions" and isinstance(v, list):
            v = [c for c in v if not (isinstance(c, str) and c.startswith("preset:"))]
        if isinstance(v, list) and not v:
            continue
        out[k] = v
    return out


def held(units, key: str, kind: str) -> bool:
    """Whether the model holds the key (check_facts_trace.in_unit over the enabled units); an input always."""
    if kind == "input":
        return True
    return any(check_facts_trace.in_unit(key, kind, u) for u in units.values())


def unit_of(units, key: str, kind: str) -> str:
    if kind == "input":
        return "inputs (S9)"
    return next((name for name, u in units.items() if check_facts_trace.in_unit(key, kind, u)), "(not held)")


def group_of(f: Fact) -> str:
    if f.kind == "widget":
        return f.widget or "?"
    if f.kind == "token":
        return "token." + f.ckey[1] if len(f.ckey) > 1 else "token"
    return {"role": "colour roles", "input": "inputs"}.get(f.kind, f.kind)


def context_widgets(when) -> set:
    """The widgets a `when` names as an ancestor or a sibling."""
    out = set()
    for k in ("context", "not_context"):
        for e in when.get(k, []) if isinstance(when.get(k), list) else []:
            if isinstance(e, dict) and isinstance(e.get("widget"), str):
                out.add(e["widget"])
            scope = e.get("scope") if isinstance(e, dict) else None
            if isinstance(scope, dict) and isinstance(scope.get("nearest"), list):
                out |= {w for w in scope["nearest"] if isinstance(w, str)}
    pos = when.get("position") if isinstance(when.get("position"), dict) else {}
    if isinstance(pos.get("among"), list):
        out |= {w for w in pos["among"] if isinstance(w, str)}
    for k in ("previous", "next"):
        if isinstance(pos.get(k), dict) and isinstance(pos[k].get("widget"), str):
            out.add(pos[k]["widget"])
    return out


# ---------------------------------------------------------------------------------------------- spelling

def number(text: str):
    return float(text) if "." in text else int(text)


def spell(name, typ, dim, value):
    """(the TOML key, the TOML value) of a fact's value under its last key segment (S11's Spelling table)."""
    t = check_facts.strip_type(typ) if typ else None
    unit = check_facts.UNIT_LITERAL.match(value) if isinstance(value, str) else None
    is_expr = isinstance(value, str) and value.startswith("=")
    if t == "Radius":
        if unit:
            if unit.group(2) != "px":
                raise Untranscribable(UNIT_MISMATCH)
            return name + "_px", number(unit.group(1))
        if is_expr:
            return name + "_px", value
        if isinstance(value, str) and value in check_facts.KEYWORDS:
            raise Untranscribable(RADIUS_KEYWORD)
        return name, value
    if dim == "font_size":
        if unit:
            if unit.group(2) not in ("pt", "px"):
                raise Untranscribable(UNIT_MISMATCH)
            return f"{name}_{unit.group(2)}", number(unit.group(1))
        return name, value
    if dim in SUFFIX_OF:
        if unit:
            if unit.group(2) != check_facts.UNIT_OF[dim]:
                raise Untranscribable(UNIT_MISMATCH)
            return name + SUFFIX_OF[dim], number(unit.group(1))
        return name + SUFFIX_OF[dim], value
    if t == "Scalar" and unit:
        raise Untranscribable(SCALAR_UNIT)
    m = MAP_TYPE.fullmatch(t) if t else None
    if m and isinstance(value, dict):
        vtype = m.group(2).strip()
        vdim = check_facts.KEY_DIMS.get(check_facts.strip_type(vtype))
        table = {}
        for k, x in map_entries(value, vtype):
            ek, ev = spell(k, vtype, vdim, x)
            table[ek] = ev
        return name, table
    if unit:
        raise Untranscribable(UNTYPED_UNIT)
    return name, value


def unspell(name, value, typ):
    """(the fact's last key segment, the fact's value) of a TOML key and value: spell()'s inverse."""
    for suffix, unit in UNIT_OF_SUFFIX.items():
        if name.endswith(suffix):
            base = name[:-len(suffix)]
            if isinstance(value, (int, float)) and not isinstance(value, bool):
                return base, f"{value}{unit}"
            return base, value
    t = check_facts.strip_type(typ) if typ else None
    m = MAP_TYPE.fullmatch(t) if t else None
    if m and isinstance(value, dict):
        return name, dict(map_entries(value, m.group(2).strip()))
    return name, value


def map_entries(table, vtype):
    """A map's TOML table as (entry key, value) pairs in the fact's spelling: an entry's unit suffix into its value
    (S11's map row: `button_ms = 200` is the entry `button`, `"200ms"`)."""
    return [unspell(k, x, vtype) for k, x in table.items()]


def canonical(value, typ):
    """A fact's value as unspell() reads it back: a map written as a table with its entries' unit suffixes
    (`{ button_ms = 200 }`) as `{ button = "200ms" }`, check_facts.py taking a map's entries either way."""
    t = check_facts.strip_type(typ) if typ else None
    m = MAP_TYPE.fullmatch(t) if t else None
    return dict(map_entries(value, m.group(2).strip())) if m and isinstance(value, dict) else value


def fact_value(f: Fact):
    return canonical(f.item.data["value"], f.typ)


def normal(v):
    """A value as a comparable one: a unit literal by its number and unit, numbers by value."""
    if isinstance(v, bool):
        return ("bool", v)
    if isinstance(v, (int, float)):
        return ("num", float(v))
    if isinstance(v, str):
        m = check_facts.UNIT_LITERAL.match(v)
        return ("unit", float(m.group(1)), m.group(2)) if m else ("str", v)
    if isinstance(v, list):
        return ("list",) + tuple(normal(x) for x in v)
    if isinstance(v, dict):
        return ("table",) + tuple(sorted((k, normal(x)) for k, x in v.items()))
    return ("other", repr(v))


def place(f: Fact):
    """Set the fact's target, base table and TOML keys below them (the docstring's *Targets and paths*), its value
    spelled; Untranscribable where S11 gives no spelling."""
    segs = list(f.ckey)
    value = f.item.data["value"]
    if f.kind == "role":
        rest = segs[1:]
        if rest[0].startswith("@"):
            rel, header = ["keyed", rest[0][1:]] + rest[1:], ("keyed",)
        elif rest[0] == "global":
            rel, header = rest, ("global",)
        else:
            rel, header = ["surfaces"] + rest, ("surfaces", rest[0])
        f.target, f.section = "colors", "colors"
    elif f.kind == "token":
        rel, header = segs[1:], tuple(segs[1:2]) if len(segs) > 2 else ()
        f.target, f.section = "tokens", "tokens"
    elif f.kind == "input":
        rel, header = [], tuple(segs[1:])
        f.section = "inputs"
    else:
        if segs[0] == "component":
            head, rest, f.section = segs[:2], segs[2:], "components"
        else:
            head, rest, f.section = segs[:1], segs[1:], "widgets"
        f.target = ".".join(head + rest[:1])
        header = tuple(head[-1:] + rest[:1])
        rel = rest[1:]
        if not rel:
            if value == "absent":
                rel, value = ["absent"], True
            elif isinstance(value, dict) and set(value) == {"drawn_as"}:
                rel, value = ["drawn_as"], value["drawn_as"]
            else:
                raise Untranscribable(UNTYPED_UNIT)
            f.header, f.rel, f.dotted, f.value = header, tuple(rel), tuple(rel), value
            return
    if any(s.startswith("@") for s in (rel[2:] if f.kind == "role" else rel)):
        raise Untranscribable(KEYED_SEGMENT)
    f.header = header
    if f.kind == "input":
        f.value = value
        return
    name, value = spell(rel[-1], f.typ, f.dim, value)
    f.rel = tuple(rel[:-1]) + (name,)
    below = len(header) if f.kind in ("role", "token") else 0
    f.dotted, f.value = f.rel[below:], value


def insert(tree: dict, path: tuple, leaf) -> bool:
    """Add a leaf to a nested table; False where a key above or below it is already a value or a table."""
    node = tree
    for k in path[:-1]:
        nxt = node.get(k)
        if isinstance(nxt, Leaf):
            return False
        if nxt is None:
            nxt = node[k] = {}
        node = nxt
    if path[-1] in node:
        return False
    node[path[-1]] = leaf
    return True


# ------------------------------------------------------------------------------------------- the analysis

def scheme_of(when):
    conds = when.get("conditions", []) if isinstance(when.get("conditions"), list) else []
    found = [c[7:] for c in conds if isinstance(c, str) and c.startswith("scheme:")]
    return found


def slots_of(when) -> tuple:
    """The schemes a `when` can hold under."""
    named = scheme_of(when)
    nots = when.get("not_conditions", []) if isinstance(when.get("not_conditions"), list) else []
    if len(set(named)) > 1:
        return ()
    return tuple(s for s in SCHEMES if (not named or named[0] == s) and f"scheme:{s}" not in nots)


def covers(g: dict, f: dict, scheme: str) -> bool:
    """Whether `when` g holds wherever `when` f holds under the scheme: every term of g is one of f's (a scheme
    condition of g's the scheme itself)."""
    for k, v in g.items():
        if k in ("conditions", "not_conditions"):
            gs = set(v)
            if k == "conditions":
                if any(c.startswith("scheme:") and c != f"scheme:{scheme}" for c in gs):
                    return False
                gs.discard(f"scheme:{scheme}")
            else:
                if f"scheme:{scheme}" in gs:
                    return False
                gs = {c for c in gs if not c.startswith("scheme:")}
            if not gs <= set(f.get(k, [])):
                return False
        elif k in ("states", "not_states", "variants", "not_variants", "sizes", "not_sizes", "densities",
                   "not_densities"):
            if not set(v) <= set(f.get(k, [])):
                return False
        elif k in ("context", "not_context"):
            if any(e not in f.get(k, []) for e in v):
                return False
        elif k in ("position", "direction", "size_class", "latest"):
            if f.get(k) != v:
                return False
        else:
            return False
    return True


def is_rest(f: Fact, scheme: str) -> bool:
    if not f.when:
        return True
    return f.kind == "role" and f.when == {"conditions": [f"scheme:{scheme}"]}


def with_scheme(when: dict, scheme: str) -> dict:
    if "conditions" in when:
        return {k: (v + [f"scheme:{scheme}"] if k == "conditions" else v) for k, v in when.items()}
    return {"conditions": [f"scheme:{scheme}"], **when}


@dataclass
class Draft:
    preset: str
    platform: str
    facts: list
    rules: list
    bases: list               # (section, scheme or None, header, dotted, value, fact)
    declare: list             # (header, [(key, value)], item)
    conditions: list          # `[declare] conditions`
    models: dict              # widget: the state model's data the round trip reduces by
    left_models: list         # (widget, item): entries of widgets the model does not hold
    staged: int


def analyse(ctx, items, preset: str, units, staged: int) -> Draft:
    """The preset's facts, classified, and its bases and rules (the docstring's rules)."""
    platform = platform_of(preset)
    if platform is None:
        raise InputError(f"{preset!r} is no preset of check_facts.PLATFORMS")
    facts = []
    for it in items:
        if it.table != "fact" or it.platform != platform or not holds_for(it, preset):
            continue
        info = check_facts.key_info(ctx, it.data.get("key"))
        facts.append(Fact(it, it.data["key"], check_facts.canonical_key(it.data["key"]),
                          preset_when(it.data.get("when"), preset), info[3], info[2], info[4], info[1]))
    files = {}
    for f in facts:
        files.setdefault(f.ckey, set()).add(f.item.file)
    for f in facts:
        d = f.item.data
        if "pending" in d:
            f.status = PENDING
        elif not held(units, f.key, f.kind):
            f.status, f.reason = LEFT_OUT, NOT_HELD
        elif any(not held(units, w, "widget") for w in context_widgets(f.when)):
            off = sorted(w for w in context_widgets(f.when) if not held(units, w, "widget"))
            f.status, f.reason = LEFT_OUT, f"{UNIT_NOT_ENABLED} ({', '.join(off)})"
        elif any(c.data.get("kind") == "read" for c in f.item.cites):
            f.status, f.reason = LEFT_OUT, READ
        elif "scheme_file" in d or "accent" in d:
            f.status, f.reason = LEFT_OUT, ACCENT_RUN
        elif len(files[f.ckey]) > 1:
            f.status, f.reason = LEFT_OUT, TWO_FILES
        else:
            try:
                place(f)
            except Untranscribable as err:
                f.status, f.reason = LEFT_OUT, str(err)
    by_key = {}
    for f in facts:
        by_key.setdefault(f.ckey, []).append(f)
    for fs in by_key.values():
        if fs[0].kind == "input":
            inputs_of(fs)
            continue
        for i, f in enumerate(fs):
            if f.status:
                continue
            f.slots = tuple(s for s in slots_of(f.when) if not any(covers(g.when, f.when, s) for g in fs[i + 1:]))
            if not f.slots:
                f.status = SHADOWED
            elif f.kind == "role" and all(is_rest(f, s) for s in f.slots):
                f.status, f.base_slots = "base", f.slots
            elif f.kind != "role" and not f.when:
                f.status, f.base_slots = "base", SCHEMES
            else:
                f.status = "rule"
                f.rule_when = f.when if f.slots == slots_of(f.when) else with_scheme(f.when, f.slots[0])
    bases, tree = [], {}
    for f in facts:
        if f.status != "base":
            continue
        if f.kind == "input":
            entries = [(None, dotted, v) for dotted, v in f.value]
        elif f.kind == "role":
            entries = [(s, f.dotted, f.value) for s in f.base_slots]
        else:
            entries = [(None, f.dotted, f.value)]
        paths = [(f.section,) + ((s,) if s else ()) + f.header + tuple(dotted) for s, dotted, _ in entries]
        placed = []
        for p in paths:
            if not insert(tree, p, Leaf(f)):
                break
            placed.append(p)
        if len(placed) < len(paths):
            for p in placed:
                remove(tree, p)
            f.status, f.reason = LEFT_OUT, BASE_CLASH
            continue
        bases += [(f.section, s, f.header, tuple(dotted), v, f) for s, dotted, v in entries]
    rules = group_rules([f for f in facts if f.status == "rule"])
    declare, conditions, models, left_models = declarations(items, platform, preset, units, facts)
    return Draft(preset, platform, facts, rules, bases, declare, conditions, models, left_models, staged)


def remove(tree: dict, path: tuple):
    node = tree
    for k in path[:-1]:
        node = node.get(k) if isinstance(node, dict) else None
        if not isinstance(node, dict):
            return
    if isinstance(node.get(path[-1]), Leaf):
        del node[path[-1]]


def input_spellable(when) -> bool:
    if set(when) - {"conditions"}:
        return False
    conds = when.get("conditions", [])
    schemes = [c for c in conds if c.startswith("scheme:")]
    contrasts = [c for c in conds if c in dict(CONTRASTS)]
    return len(schemes) <= 1 and len(contrasts) <= 1 and len(schemes) + len(contrasts) == len(conds)


def cell_combo(scheme, contrast):
    return combo({"conditions": [f"scheme:{scheme}"] + ([contrast] if contrast else [])})


def inputs_of(fs: list):
    """An input's facts to its bases (S9; the docstring's *Bases and rules*): each cell's fact."""
    for f in fs:
        if not f.status and not input_spellable(f.when):
            f.status, f.reason = LEFT_OUT, INPUT_WHEN
    cells = {}
    for s in SCHEMES:
        for c in (None,) + tuple(x for x, _ in CONTRASTS):
            cells[(s, c)] = last_holding(fs, cell_combo(s, c), [], {})
    entries = {}
    for s in SCHEMES:
        f = cells[(s, None)]
        if f is not None and not f.status:
            entries.setdefault(id(f), []).append((s,))
    for cond, name in CONTRASTS:
        pair = [cells[(s, cond)] for s in SCHEMES]
        if all(p is cells[(s, None)] for p, s in zip(pair, SCHEMES)):
            continue
        if any(p is None or p.status for p in pair):
            for p, s in zip(pair, SCHEMES):
                if p is not None and not p.status and p is not cells[(s, None)]:
                    p.status, p.reason = LEFT_OUT, INPUT_CONTRAST
            continue
        for p, s in zip(pair, SCHEMES):
            entries.setdefault(id(p), []).append(("contrast", name, s))
    for f in fs:
        if f.status:
            continue
        if id(f) in entries:
            value = f.item.data["value"]
            f.status, f.base_slots, f.value = "base", SCHEMES, [(dotted, value) for dotted in entries[id(f)]]
        else:
            f.status = SHADOWED


def group_rules(facts: list) -> list:
    """The rules, in order (the docstring's grouping): a fact joins the last rule with its `when` and target unless a
    later rule states a key related to its own, or the rule's `set` cannot take it; else a new rule."""
    rules, last_of = [], {}
    last_key = {}       # a key: the last rule stating it
    last_below = {}     # a key: the last rule stating it or a key below it
    for f in facts:
        sig = (check_facts.canonical_when(f.rule_when), f.target)
        i = last_of.get(sig)
        later = max([last_key.get(f.ckey[:n], -1) for n in range(1, len(f.ckey) + 1)]
                    + [last_below.get(f.ckey, -1)])
        if i is None or later > i or not insert(rules[i].tree, f.rel, Leaf(f)):
            rules.append(Rule(f.rule_when, f.target))
            i = len(rules) - 1
            last_of[sig] = i
            insert(rules[i].tree, f.rel, Leaf(f))
        rules[i].facts.append(f)
        last_key[f.ckey] = i
        for n in range(1, len(f.ckey) + 1):
            last_below[f.ckey[:n]] = i
    return rules


def declarations(items, platform, preset, units, facts):
    """([declare] tables, `[declare] conditions`, the state models the round trip reduces by, the entries left out)
    from the platform's state models (S10, S17 *State models*)."""
    entries = {}
    for it in items:
        if it.table != "model" or it.platform != platform or not isinstance(it.data.get("widget"), str):
            continue
        when = it.data.get("when") if isinstance(it.data.get("when"), dict) else {}
        conds = when.get("conditions", []) if isinstance(when.get("conditions"), list) else []
        named = [c[7:] for c in conds if c.startswith("preset:")]
        if named and named != [preset]:
            continue
        flavor = next((c[7:] for c in conds if c.startswith("flavor:")), None)
        entries.setdefault(it.data["widget"], []).append((it, bool(named), flavor))
    declare, models, left, settings = [], {}, [], []
    for widget, es in entries.items():
        if not held(units, widget, "widget"):
            left += [(widget, it) for it, _, _ in es]
            continue
        plain = [e for e in es if e[2] is None]
        chosen = [e for e in plain if e[1]] or plain
        picks = chosen[-1:] + [e for e in es if e[2] is not None]
        for it, _, flavor in picks:
            d = it.data
            kind, name = ("component", widget[10:]) if widget.startswith("component.") else ("widget", widget)
            header = (("flavor", flavor) if flavor else ()) + (kind, name)
            kvs = [("available", False)] if d.get("available") is False else [
                (k, d[k]) for k in DECLARE_KEYS if k in d]
            declare.append((header, kvs, it))
            if flavor is None:
                models[widget] = d
            axes = d.get("axes") if isinstance(d.get("axes"), dict) else {}
            settings += [c for c in axes.get("conditions", []) if isinstance(c, str)]
    for f in facts:
        for k in ("conditions", "not_conditions"):
            settings += [c for c in f.when.get(k, []) if isinstance(c, str)]
    conditions = list(dict.fromkeys("@" + c[len("system_setting:"):] for c in settings
                                    if c.startswith("system_setting:")))
    return declare, conditions, models, left


# ---------------------------------------------------------------------------------------------- the draft

def dotted_key(keys) -> str:
    return ".".join(toml_key(k) for k in keys)


def render(draft: Draft) -> str:
    out = [f"# {draft.preset}: a draft generated by scripts/generate_preset.py from docs/facts/{draft.platform}/ "
           f"(S11).",
           "# Each value's comment names its fact. Not a validated preset: the preset's task places and completes it.",
           "", "[preset]", f"id = {toml_inline(draft.preset)}", ""]
    if draft.conditions:
        out += ["[declare]", f"conditions = {toml_inline(draft.conditions)}", ""]
    for header, kvs, it in draft.declare:
        out += [f"# {it.file}:{it.line}", f"[{dotted_key(('declare',) + header)}]"]
        out += [f"{toml_key(k)} = {toml_inline(v)}" for k, v in kvs] + [""]
    tables = {}
    for section, scheme, header, dotted, value, f in draft.bases:
        tables.setdefault((section, scheme, header), []).append((dotted, value, f))
    order = sorted(tables, key=lambda t: (SECTIONS.index(t[0]), SCHEMES.index(t[1]) if t[1] else -1))
    for t in order:
        section, scheme, header = t
        out.append(f"[{dotted_key((section,) + ((scheme,) if scheme else ()) + header)}]")
        for dotted, value, f in tables[t]:
            out += [f"# {f.where}", f"{dotted_key(dotted)} = {toml_inline(value)}"]
        out.append("")
    for r in draft.rules:
        out += ["[[rules]]", f"when = {toml_inline(r.when)}", f"target = {toml_inline(r.target)}", "", "[rules.set]"]
        for f in r.facts:
            out += [f"# {f.where}", f"{dotted_key(f.rel)} = {toml_inline(f.value)}"]
        out.append("")
    return "\n".join(out).rstrip("\n") + "\n"


# ------------------------------------------------------------------------------------------- combinations

def combo(d: dict) -> dict:
    """A combination: the element's conditions, flags, variants, size, density, direction, position, ancestors,
    size classes and recency answers."""
    return {"conditions": set(d.get("conditions", [])), "states": set(d.get("states", [])),
            "variants": set(d.get("variants", [])), "sizes": set(d.get("sizes", [])),
            "densities": set(d.get("densities", [])), "direction": d.get("direction", "ltr"),
            "position": d.get("position"), "ancestors": d.get("ancestors", []),
            "size_class": {a: set(d.get("size_class", {}).get(a, [])) for a in ("horizontal", "vertical")},
            "latest": dict(d.get("latest", {}))}


def reduced(states: set, groups: list) -> set:
    """The flags after the exclusive groups' drop (S10: each group keeps its first present flag)."""
    kept = set(states)
    for g in groups:
        kept -= set([x for x in g if x in kept][1:])
    return kept


def set_match(when, k, have: set) -> bool:
    return set(when.get(k, [])) <= have and not (set(when.get(f"not_{k}", [])) & have)


def ancestor_matches(e: dict, a: dict) -> bool:
    if any(k in e and e[k] != a.get(k) for k in ("widget", "part", "surface")):
        return False
    return all(set_match(e, k, a.get(k, set())) for k in ("variants", "states", "sizes"))


def context_holds(e: dict, ancestors: list) -> bool:
    scope = e.get("scope", "any")
    if isinstance(scope, dict):
        nearest = next((a for a in ancestors if a.get("widget") in scope.get("nearest", [])), None)
        return nearest is not None and ancestor_matches(e, nearest)
    return any(ancestor_matches(e, a) for a in ancestors)


def holds(when: dict, c: dict, groups: list, recency: dict) -> bool:
    """Whether a `when` holds in a combination (S7, S11): every required name present, no refused one, after the
    exclusive drop; the contexts, position, direction, size classes and recency answers as S7 matches them."""
    states = reduced(c["states"], groups)
    have = {"conditions": c["conditions"], "states": states, "variants": c["variants"], "sizes": c["sizes"],
            "densities": c["densities"]}
    if not all(set_match(when, k, v) for k, v in have.items()):
        return False
    if "direction" in when and when["direction"] != c["direction"]:
        return False
    sc = when.get("size_class", {})
    if not all(set_match(sc, a, c["size_class"][a]) for a in ("horizontal", "vertical")):
        return False
    wp = when.get("position")
    if isinstance(wp, dict):
        cp = c["position"] if isinstance(c["position"], dict) and c["position"].get("among", "same") == wp.get(
            "among", "same") else {}
        if any(k in wp and cp.get(k, LONE[k]) != wp[k] for k in LONE):
            return False
        if any(k in wp and cp.get(k) != wp[k] for k in ("previous", "next")):
            return False
    if not all(context_holds(e, c["ancestors"]) for e in when.get("context", [])):
        return False
    if any(context_holds(e, c["ancestors"]) for e in when.get("not_context", [])):
        return False
    for name, flag in when.get("latest", {}).items():
        present = [x for x in recency.get(name, []) if x in states]
        chosen = c["latest"].get(name) if c["latest"].get(name) in present else (present[0] if present else None)
        if chosen != flag:
            return False
    return True


def minimal(when: dict, scheme: str, groups: list, recency: dict):
    """The `when`'s minimal combination under a scheme (its required names and nothing else), or None where no
    element holds it."""
    conds = list(when.get("conditions", []))
    if any(x.startswith("scheme:") and x != f"scheme:{scheme}" for x in conds):
        return None
    latest = when.get("latest", {})
    d = {"conditions": conds + [f"scheme:{scheme}"], "states": list(when.get("states", [])) + list(latest.values()),
         "variants": when.get("variants", []), "sizes": when.get("sizes", []), "densities": when.get("densities", []),
         "direction": when.get("direction", "ltr"), "position": when.get("position"), "latest": latest,
         "size_class": when.get("size_class", {}),
         "ancestors": [{"widget": e.get("widget"), "part": e.get("part"), "surface": e.get("surface"),
                        "variants": set(e.get("variants", [])), "states": set(e.get("states", [])),
                        "sizes": set(e.get("sizes", []))} for e in when.get("context", [])]}
    if len(set(d["sizes"])) > 1 or len(set(d["densities"])) > 1:
        return None
    if any(len(set(sc)) > 1 for sc in d["size_class"].values() if isinstance(sc, list)):
        return None
    c = combo(d)
    return c if holds(when, c, groups, recency) else None


def last_holding(fs: list, c: dict, groups, recency):
    hit = None
    for f in fs:
        if holds(f.when, c, groups, recency):
            hit = f
    return hit


# --------------------------------------------------------------------------------------------- round trip

@dataclass
class Trip:
    compared: int = 0
    pending: int = 0
    left_out: int = 0
    impossible: int = 0
    mismatches: list = field(default_factory=list)


def lookup(data, path):
    node = data
    for k in path:
        if not isinstance(node, dict) or k not in node:
            return None, False
        node = node[k]
    return node, True


def leaves(node, path, expected: set, extra: list):
    """Walk a parsed table: every value not at or under an expected path is extra."""
    if path in expected:
        return
    if isinstance(node, dict):
        for k, v in node.items():
            leaves(v, path + (k,), expected, extra)
    else:
        extra.append(path)


def round_trip(draft: Draft, text: str) -> Trip:
    """The docstring's round trip: the draft read back gives every transcribed fact's value, in its order, and every
    fact's minimal combination the value of the fact that applies there."""
    trip = Trip()
    try:
        data = tomllib.loads(text)
    except tomllib.TOMLDecodeError as err:
        trip.mismatches.append(f"the draft is no valid TOML: {err}")
        return trip
    rules = data.get("rules", []) if isinstance(data.get("rules"), list) else []
    if len(rules) != len(draft.rules):
        trip.mismatches.append(f"{len(rules)} rules read back, {len(draft.rules)} generated")
    back = {}   # a key: {"base": {scheme or (scheme, contrast): value}, "rules": [(when, value)]}
    expected_base, expected_set = set(), {}
    for section, scheme, header, dotted, value, f in draft.bases:
        path = (section,) + ((scheme,) if scheme else ()) + header + dotted
        expected_base.add(path)
        node, found = lookup(data, path)
        if not found:
            trip.mismatches.append(f"{f.where}: {f.key}: no base value at {dotted_key(path)}")
            continue
        entry = back.setdefault(f.ckey, {"base": {}, "rules": []})
        if f.kind == "input":
            slot = (dotted[-1], None) if len(dotted) == 1 else (dotted[2], f"contrast_{dotted[1]}")
            entry["base"][slot] = node
        else:
            entry["base"][scheme] = back_value(f, node)
    for i, r in enumerate(rules[:len(draft.rules)]):
        g = draft.rules[i]
        if not isinstance(r, dict) or check_facts.canonical_when(r.get("when")) != check_facts.canonical_when(
                g.when) or r.get("target") != g.target:
            trip.mismatches.append(f"rule {i + 1}: its `when` or `target` reads back otherwise")
            continue
        expected_set[i] = set()
        for f in g.facts:
            expected_set[i].add(f.rel)
            node, found = lookup(r.get("set", {}), f.rel)
            if not found:
                trip.mismatches.append(f"{f.where}: {f.key}: rule {i + 1} states no {dotted_key(f.rel)}")
                continue
            value = back_value(f, node)
            back.setdefault(f.ckey, {"base": {}, "rules": []})["rules"].append((r.get("when"), value))
    extra = []
    for section in SECTIONS:
        if section in data:
            leaves(data[section], (section,), expected_base, extra)
    for i, r in enumerate(rules[:len(draft.rules)]):
        if i in expected_set and isinstance(r, dict):
            leaves(r.get("set", {}), (), expected_set[i], extra)
    trip.mismatches += [f"a value no fact gives: {dotted_key(p)}" for p in extra]
    by_key, whens = {}, {}
    for f in draft.facts:
        by_key.setdefault(f.ckey, []).append(f)
        whens.setdefault(group_of(f), {}).setdefault(check_facts.canonical_when(f.when), f.when)
    whens = {g: list(ws.values()) for g, ws in whens.items()}
    for ckey, fs in by_key.items():
        got = back.get(ckey, {"base": {}, "rules": []})
        want_rules = [(f.rule_when, fact_value(f)) for f in fs if f.status == "rule"]
        if [(check_facts.canonical_when(w), normal(v)) for w, v in got["rules"]] != [
                (check_facts.canonical_when(w), normal(v)) for w, v in want_rules]:
            trip.mismatches.append(f"{fs[0].key}: the rules read back state other values or another order")
        for f in fs:
            if f.status == "base" and f.kind != "input":
                for s in f.base_slots:
                    if normal(got["base"].get(s if f.kind == "role" else None)) != normal(fact_value(f)):
                        trip.mismatches.append(f"{f.where}: {f.key}: the base reads back otherwise")
        model = draft.models.get(fs[0].widget) if fs[0].widget else None
        groups = [g for g in model.get("exclusive", []) if isinstance(g, list)] if model else []
        recency = check_facts.recency_of(model) if model else {}
        for f in fs:
            if slots_of(f.when) and all(minimal(f.when, s, groups, recency) is None for s in SCHEMES):
                trip.impossible += 1
        for when in whens[group_of(fs[0])]:
            for s in SCHEMES:
                c = minimal(when, s, groups, recency)
                if c is None:
                    continue
                applies = last_holding(fs, c, groups, recency)
                if applies is None:
                    if preset_value(got, fs[0].kind, c, s, groups, recency) is not None:
                        trip.mismatches.append(f"{fs[0].key} in {when} under scheme {s}: the draft states a value "
                                               f"where no fact holds")
                    continue
                if applies.status == PENDING:
                    trip.pending += 1
                    continue
                if applies.status == LEFT_OUT:
                    trip.left_out += 1
                    continue
                trip.compared += 1
                if applies.status == SHADOWED:
                    trip.mismatches.append(f"{fs[0].key} in {when} under scheme {s}: the shadowed fact "
                                           f"{applies.where} applies")
                    continue
                value = preset_value(got, fs[0].kind, c, s, groups, recency)
                if normal(value) != normal(fact_value(applies)):
                    trip.mismatches.append(f"{fs[0].key} in {when} under scheme {s}: the draft gives {value!r}, "
                                           f"the fact that applies ({applies.where}) {applies.item.data['value']!r}")
    return trip


def back_value(f: Fact, node):
    """A read-back TOML value as the fact writes it (unspell; a whole part's `absent = true` and `drawn_as`)."""
    if f.kind == "widget" and len(f.ckey) == len(f.target.split(".")):
        return "absent" if f.rel == ("absent",) and node is True else {f.rel[-1]: node}
    return unspell(f.rel[-1], node, f.typ)[1]


def preset_value(got, kind, c, scheme, groups, recency):
    """The value the read-back bases and rules give a key in a combination (S8: the bases, then the rules in order,
    the last matching winning; an input's per-contrast base where its contrast holds, S9)."""
    if kind == "input":
        contrast = next((x for x, _ in CONTRASTS if x in c["conditions"]), None)
        return got["base"].get((scheme, contrast), got["base"].get((scheme, None)))
    value = got["base"].get(scheme if kind == "role" else None)
    for when, v in got["rules"]:
        if holds(when if isinstance(when, dict) else {}, c, groups, recency):
            value = v
    return value


# ------------------------------------------------------------------------------------------------- report

def share_lines(draft: Draft, units, trip: Trip) -> list:
    def tally(fs):
        n = len(fs)
        base = sum(1 for f in fs if f.status == "base")
        rule = sum(1 for f in fs if f.status == "rule")
        shadowed = sum(1 for f in fs if f.status == SHADOWED)
        pending = sum(1 for f in fs if f.status == PENDING)
        reasons = {}
        for f in fs:
            if f.status == LEFT_OUT:
                reasons[f.reason] = reasons.get(f.reason, 0) + 1
        done = base + rule + shadowed
        pct = f"{100 * done / n:.1f}%" if n else "-"
        pct_np = f"{100 * done / (n - pending):.1f}%" if n - pending else "-"
        left = ", ".join(f"{k} {v}" for k, v in sorted(reasons.items())) or "none"
        return (f"{n} facts: {done} transcribed ({pct}; {pct_np} of the non-pending: {base} as bases, {rule} in "
                f"rules, {shadowed} shadowed), {pending} pending, {n - done - pending} left out ({left})")
    out = [f"# {draft.preset}: {tally(draft.facts)}"]
    groups, unit_groups = {}, {}
    for f in draft.facts:
        groups.setdefault(group_of(f), []).append(f)
        unit_groups.setdefault(unit_of(units, f.key, f.kind), []).append(f)
    out += [f"#   {g}: {tally(fs)}" for g, fs in groups.items()]
    out += [f"#   unit {u}: {tally(fs)}" for u, fs in unit_groups.items()]
    nbase = len(draft.bases)
    out.append(f"# the draft: {len(draft.rules)} rules, {sum(len(r.facts) for r in draft.rules)} rule values, "
               f"{nbase} base values, {len(draft.declare)} [declare] tables")
    pend = {}
    for f in draft.facts:
        if f.status == PENDING:
            pend[f.item.data["pending"]] = pend.get(f.item.data["pending"], 0) + 1
    if pend:
        out.append("# pending (register id: facts): " + ", ".join(
            f"{k}: {v}" for k, v in sorted(pend.items(), key=lambda kv: int(kv[0][1:]))))
    for w, it in draft.left_models:
        out.append(f"# [declare] left out: {w} ({it.file}:{it.line}), a widget the model does not hold")
    out.append(f"# left for hand-writing: S19.2's staged bases for {draft.preset}: {draft.staged} "
               f"({STAGED_BASES}); the keyed namespaces of [declare] but `conditions`; `[preset] generation`")
    out.append(f"# round trip: {trip.compared} combinations compared, {len(trip.mismatches)} mismatches "
               f"({trip.pending} where a pending fact applies and {trip.left_out} where a left-out one does, skipped; "
               f"{trip.impossible} facts whose `when` no element holds: two sizes, a flag its exclusive group drops, "
               f"a name required and refused)")
    return out


def staged_count(root: Path, preset: str) -> int:
    p = root / STAGED_BASES
    if not p.exists():
        return 0
    try:
        data = tomllib.loads(p.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as err:
        raise InputError(f"{STAGED_BASES}: {err}") from err
    return sum(1 for b in data.get("base", []) if isinstance(b, dict) and b.get("preset") == preset)


def generate(ctx, preset: str):
    """(the draft, its text, its round trip)."""
    items, findings, _ = check_facts.load(ctx)
    if findings:
        raise InputError("; ".join(f"{f}:{ln}: {what}" for f, ln, what in findings))
    units = check_facts_trace.read_units(ctx.root)
    draft = analyse(ctx, items, preset, units, staged_count(ctx.root, preset))
    text = render(draft)
    return draft, text, round_trip(draft, text), units


# ---------------------------------------------------------------------------------------------- self-test

_C = '\n[[fact.cite]]\nkind = "published"\nsource = "libadwaita@1.10.0 a.scss:1"\nquote = "x"\n'
_READ = '\n[[fact.cite]]\nkind = "read"\nsource = "libadwaita@1.10.0 a.c:1"\nquote = "x"\n'


def _fact(key, value, when=None, cite=_C):
    w = f"when = {when}\n" if when else ""
    return f'\n[[fact]]\nkey = "{key}"\n{w}value = {value}\nrows = []\n{cite}'


def _pending(key, uid, when):
    return f'\n[[fact]]\nkey = "{key}"\nwhen = {when}\npending = "{uid}"\nrows = []\n'


SELF_TEST_FILES = {
    "docs/facts/sources.toml": check_facts.SELF_TEST_SOURCES,
    "docs/facts/gnome/state-models.toml": '''
[[model]]
widget = "button"
available = true
variants = ["flat", "primary", "@pill"]
sizes = []
densities = []
flags = ["@busy"]
axes = { states = ["hover", "pressed", "disabled", "checked"], conditions = ["scheme:light", "scheme:dark", \
"contrast_high", "system_setting:reduced_chrome"], contexts = [] }
exclusive = [["disabled", "pressed", "hover"]]
exclusive_variants = [["flat", "primary"]]
recency = { glow = ["checked", "hover"] }
variant_sets = [["flat", "@pill"]]
not_drawn = []
rows = []

[[model.cite]]
kind = "published"
source = "libadwaita@1.10.0 a.scss:1"
quote = "x"

[[model]]
widget = "toolbar"
available = true
variants = []
sizes = []
densities = []
axes = { states = [], conditions = [], contexts = [] }
exclusive = []
not_drawn = []
rows = []

[[model.cite]]
kind = "published"
source = "libadwaita@1.10.0 a.scss:1"
quote = "x"
''',
    "docs/facts/gnome/button.toml": "".join([
        _fact("button.body.min_width", '"16px"'),
        _fact("button.body.fill", '"#eeeeee"', '{ states = ["hover"] }'),
        _fact("button.body.fill", '"= surface.background"'),
        _fact("button.body.fill", '"= alpha(surface.foreground, 0.1)"', '{ states = ["hover"] }'),
        _fact("button.body.corners.top_left", '"4px"', '{ states = ["hover"] }'),
        _fact("button.body.fill", '"absent"', '{ variants = ["flat"] }'),
        _fact("button.body.fill", '"#dddddd"', '{ states = ["hover"] }'),
        _fact("button.body.corners.top_right", '"follow_part"', '{ states = ["hover"] }'),
        _fact("button.body.corners.bottom_left", '{ full = { maximum_px = "absent" } }', '{ variants = ["flat"] }'),
        _fact("button.body.corners.bottom_right", '"absent"', '{ states = ["checked"] }'),
        _fact("button.body.padding", "{ top_px = 5, right_px = 17, bottom_px = 5, left_px = 17 }"),
        _fact("button.body.shadow", '"absent"', '{ states = ["checked"] }'),
        _fact("button.body.shadow", '{ layers = [{ x_px = 0, y_px = 1, blur_px = 2, spread_px = 0, '
                                    'color = "#00000026", '
                                    'inset = false }] }', '{ states = ["checked"], latest = { glow = "hover" } }'),
        _fact("button.body.padding", "{ top_px = 1, right_px = 1, bottom_px = 1, left_px = 1 }",
              '{ states = ["checked"], variants = ["flat"] }'),
        _fact("button.body.padding", "{ top_px = 2, right_px = 2, bottom_px = 2, left_px = 2 }",
              '{ states = ["checked"] }'),
        _fact("button.body.padding", "{ top_px = 3, right_px = 3, bottom_px = 3, left_px = 3 }",
              '{ states = ["checked"], not_variants = ["flat"] }'),
        _fact("button.body.transitions", "[]"),
        _fact("button.body.corners.top_left", '"2px"', '{ states = ["hover", "pressed"] }'),
        _fact("button.body.min_width", '"= token.focus.width"', '{ variants = ["primary"] }'),
        _fact("button.body.min_width", '"auto"', '{ variants = ["@pill"] }'),
        _fact("button.body.fill", '"#000000"', '{ context = [{ scope = "any", widget = "toolbar", part = "bar" }] }'),
        _pending("button.body.fill", "U1", '{ states = ["pressed"] }'),
        _fact("button.label.text", '"= token.text.roles.body"'),
        _fact("button.label.text.font.weight", "700"),
        _fact("button.label.text.font.size", '"11pt"', '{ conditions = ["contrast_high"] }'),
        _fact("button.label.text", '"= token.text.roles.body"', '{ variants = ["primary"] }'),
        _fact("button.label.text.fill", '"= surface.foreground"', '{ variants = ["primary"] }'),
        _fact("button.mark", '"absent"', '{ variants = ["@pill"] }'),
        _fact("button.mark", '{ drawn_as = { widget = "button", part = "label" } }', '{ variants = ["flat"] }'),
        _fact("button.values.icon_text_gap", '"6px"'),
        _fact("button.values.repeat_delay", '"absent"'),
        _fact("button.body.interaction.state_layer.alpha", "0.08", '{ states = ["hover"], direction = "rtl" }'),
        _fact("pull_to_refresh.body.padding", "{ top_px = 0, right_px = 0, bottom_px = 0, left_px = 0 }"),
    ]),
    "docs/facts/gnome/roles.toml": "".join([
        _fact("role.window.background", '"#fafafb"', '{ conditions = ["scheme:light"] }'),
        _fact("role.window.background", '"#ffffff"', '{ conditions = ["contrast_high"] }'),
        _fact("role.window.background", '"#222226"', '{ conditions = ["scheme:dark"] }'),
        _fact("role.window.foreground", '"= alpha(#000006, 0.8)"'),
        _fact("role.global.accent", '"= input.accent"', cite=_READ),
        _fact("role.global.accent_ramp", "{}"),
        _fact("role.@overview_background", '"#fafafb"', '{ conditions = ["scheme:light"] }'),
        _fact("role.view.background", '"#eeeeee"', '{ conditions = ["window_inactive"] }'),
        _fact("input.accent", '"#3584e4"'),
        _fact("input.accent", '"#1c71d8"', '{ conditions = ["contrast_high", "scheme:light"] }'),
        _fact("input.selected", '"#ffffff"', '{ conditions = ["window_inactive"] }'),
    ]),
    "docs/facts/gnome/tokens.toml": "".join([
        _fact("token.motion.durations.button", '"200ms"'),
        _fact("token.motion.timings.ease", "{ cubic_bezier = [0.25, 0.1, 0.25, 1.0] }"),
        _fact("token.motion.speed_factor", "1"),
        _fact("token.motion.durations.button", '"0ms"', '{ conditions = ["system_setting:reduced_chrome"] }'),
        _fact('token.text.fallbacks.per_family.\\"Segoe Fluent Icons\\"', '["Segoe MDL2 Assets"]'),
        _fact("token.text.fallbacks.per_language", '{ "zh-Hans" = ["Noto Sans CJK SC"] }'),
        _fact("token.kde.frame_width", '"2px"'),
        _fact("token.focus.width", '"2px"'),
        _fact("token.icons.theme", '"Adwaita"'),
    ]),
}

SELF_TEST_EXPECTED = '''# adwaita: a draft generated by scripts/generate_preset.py from docs/facts/gnome/ (S11).
# Each value's comment names its fact. Not a validated preset: the preset's task places and completes it.

[preset]
id = "adwaita"

[declare]
conditions = ["@reduced_chrome"]

# docs/facts/gnome/state-models.toml:2
[declare.widget.button]
available = true
variants = ["flat", "primary", "@pill"]
sizes = []
densities = []
flags = ["@busy"]
exclusive = [["disabled", "pressed", "hover"]]
exclusive_variants = [["flat", "primary"]]
recency = { glow = ["checked", "hover"] }

[inputs.accent]
# docs/facts/gnome/roles.toml:87
light = "#3584e4"
# docs/facts/gnome/roles.toml:87
dark = "#3584e4"
# docs/facts/gnome/roles.toml:87
contrast.high.dark = "#3584e4"
# docs/facts/gnome/roles.toml:97
contrast.high.light = "#1c71d8"

[colors.light.surfaces.window]
# docs/facts/gnome/roles.toml:2
background = "#fafafb"
# docs/facts/gnome/roles.toml:35
foreground = "= alpha(#000006, 0.8)"

[colors.light.global]
# docs/facts/gnome/roles.toml:55
accent_ramp = {}

[colors.light.keyed]
# docs/facts/gnome/roles.toml:65
overview_background = "#fafafb"

[colors.dark.surfaces.window]
# docs/facts/gnome/roles.toml:24
background = "#222226"
# docs/facts/gnome/roles.toml:35
foreground = "= alpha(#000006, 0.8)"

[colors.dark.global]
# docs/facts/gnome/roles.toml:55
accent_ramp = {}

[tokens.motion]
# docs/facts/gnome/tokens.toml:2
durations.button_ms = 200
# docs/facts/gnome/tokens.toml:12
timings.ease = { cubic_bezier = [0.25, 0.1, 0.25, 1.0] }
# docs/facts/gnome/tokens.toml:22
speed_factor = 1

[tokens.text]
# docs/facts/gnome/tokens.toml:43
fallbacks.per_family."Segoe Fluent Icons" = ["Segoe MDL2 Assets"]
# docs/facts/gnome/tokens.toml:53
fallbacks.per_language = { zh-Hans = ["Noto Sans CJK SC"] }

[tokens.focus]
# docs/facts/gnome/tokens.toml:73
width_px = 2

[tokens.icons]
# docs/facts/gnome/tokens.toml:83
theme = "Adwaita"

[widgets.button.body]
# docs/facts/gnome/button.toml:2
min_width_px = 16
# docs/facts/gnome/button.toml:23
fill = "= surface.background"
# docs/facts/gnome/button.toml:110
padding = { top_px = 5, right_px = 17, bottom_px = 5, left_px = 17 }
# docs/facts/gnome/button.toml:175
transitions = []

[widgets.button.label]
# docs/facts/gnome/button.toml:235
text = "= token.text.roles.body"

[widgets.button.values]
# docs/facts/gnome/button.toml:310
icon_text_gap_px = 6
# docs/facts/gnome/button.toml:320
repeat_delay_ms = "absent"

[[rules]]
when = { states = ["hover"] }
target = "button.body"

[rules.set]
# docs/facts/gnome/button.toml:44
corners.top_left_px = 4

[[rules]]
when = { variants = ["flat"] }
target = "button.body"

[rules.set]
# docs/facts/gnome/button.toml:55
fill = "absent"
# docs/facts/gnome/button.toml:88
corners.bottom_left = { full = { maximum_px = "absent" } }

[[rules]]
when = { states = ["hover"] }
target = "button.body"

[rules.set]
# docs/facts/gnome/button.toml:66
fill = "#dddddd"
# docs/facts/gnome/button.toml:77
corners.top_right = "follow_part"

[[rules]]
when = { states = ["checked"] }
target = "button.body"

[rules.set]
# docs/facts/gnome/button.toml:120
shadow = "absent"
# docs/facts/gnome/button.toml:153
padding = { top_px = 2, right_px = 2, bottom_px = 2, left_px = 2 }

[[rules]]
when = { states = ["checked"], latest = { glow = "hover" } }
target = "button.body"

[rules.set]
# docs/facts/gnome/button.toml:131
shadow = { layers = [{ x_px = 0, y_px = 1, blur_px = 2, spread_px = 0, color = "#00000026", inset = false }] }

[[rules]]
when = { states = ["checked"], not_variants = ["flat"] }
target = "button.body"

[rules.set]
# docs/facts/gnome/button.toml:164
padding = { top_px = 3, right_px = 3, bottom_px = 3, left_px = 3 }

[[rules]]
when = { states = ["hover", "pressed"] }
target = "button.body"

[rules.set]
# docs/facts/gnome/button.toml:185
corners.top_left_px = 2

[[rules]]
when = { variants = ["primary"] }
target = "button.body"

[rules.set]
# docs/facts/gnome/button.toml:196
min_width_px = "= token.focus.width"

[[rules]]
when = { variants = ["@pill"] }
target = "button.body"

[rules.set]
# docs/facts/gnome/button.toml:207
min_width_px = "auto"

[[rules]]
when = { conditions = ["contrast_high"] }
target = "button.label"

[rules.set]
# docs/facts/gnome/button.toml:255
text.font.size_pt = 11

[[rules]]
when = { variants = ["primary"] }
target = "button.label"

[rules.set]
# docs/facts/gnome/button.toml:266
text = "= token.text.roles.body"

[[rules]]
when = { variants = ["primary"] }
target = "button.label"

[rules.set]
# docs/facts/gnome/button.toml:277
text.fill = "= surface.foreground"

[[rules]]
when = { variants = ["@pill"] }
target = "button.mark"

[rules.set]
# docs/facts/gnome/button.toml:288
absent = true

[[rules]]
when = { variants = ["flat"] }
target = "button.mark"

[rules.set]
# docs/facts/gnome/button.toml:299
drawn_as = { widget = "button", part = "label" }

[[rules]]
when = { states = ["hover"], direction = "rtl" }
target = "button.body"

[rules.set]
# docs/facts/gnome/button.toml:330
interaction.state_layer.alpha = 0.08

[[rules]]
when = { conditions = ["contrast_high", "scheme:light"] }
target = "colors"

[rules.set]
# docs/facts/gnome/roles.toml:13
surfaces.window.background = "#ffffff"

[[rules]]
when = { conditions = ["window_inactive"] }
target = "colors"

[rules.set]
# docs/facts/gnome/roles.toml:76
surfaces.view.background = "#eeeeee"

[[rules]]
when = { conditions = ["system_setting:reduced_chrome"] }
target = "tokens"

[rules.set]
# docs/facts/gnome/tokens.toml:32
motion.durations.button_ms = 0
'''


# every value form (S11's Spelling table): (name, the core's type, its dimension, the fact's value, the TOML key and
# value, or the reason it is left out)
SPELL_CASES = [
    ("min_height", "Value<Px>", "length", "24px", ("min_height_px", 24)),
    ("min_height", "Value<Px>", "length", "0.5px", ("min_height_px", 0.5)),
    ("min_height", "Value<Px>", "length", "= token.focus.width", ("min_height_px", "= token.focus.width")),
    ("max_width", "Value<Px>", "length", "auto", ("max_width_px", "auto")),
    ("max_width", "Value<Px>", "length", "absent", ("max_width_px", "absent")),
    ("delay", "Value<Ms>", "duration", "150ms", ("delay_ms", 150)),
    ("rotate", "Value<Deg>", "angle", "-90deg", ("rotate_deg", -90)),
    ("size", "Value<FontSize>", "font_size", "11pt", ("size_pt", 11)),
    ("letter_spacing", "Value<FontSize>", "font_size", "0px", ("letter_spacing_px", 0)),
    ("line_height", "Value<FontSize>", "font_size", "auto", ("line_height", "auto")),
    ("size", "Value<FontSize>", "font_size", "= mul(token.text.roles.body.font.size, 1.2)",
     ("size", "= mul(token.text.roles.body.font.size, 1.2)")),
    ("top_left", "Radius", None, "4px", ("top_left_px", 4)),
    ("top_left", "Radius", None, "= token.focus.width", ("top_left_px", "= token.focus.width")),
    ("top_left", "Radius", None, "ellipse", ("top_left", "ellipse")),
    ("top_left", "Radius", None, {"concentric": {"minimum_px": 2}}, ("top_left", {"concentric": {"minimum_px": 2}})),
    ("top_left", "Radius", None, "absent", RADIUS_KEYWORD),
    ("top_left", "Radius", None, "4ms", UNIT_MISMATCH),
    ("min_height", "Value<Px>", "length", "4ms", UNIT_MISMATCH),
    ("delay", "Value<Scalar>", None, "500ms", SCALAR_UNIT),
    ("border", "Value<Scalar>", None, 0.15, ("border", 0.15)),
    ("x", None, None, "4px", UNTYPED_UNIT),
    ("fill", "Value<Paint>", "paint", "#ffffff", ("fill", "#ffffff")),
    ("fill", "Value<Paint>", "paint", "srgb(0, 0, 0, 0.65)", ("fill", "srgb(0, 0, 0, 0.65)")),
    ("fill", "Value<Paint>", "paint", "= alpha(surface.foreground, 0.1)", ("fill", "= alpha(surface.foreground, 0.1)")),
    ("family", "Value<String>", "text", "Adwaita Sans", ("family", "Adwaita Sans")),
    ("shown", "Value<bool>", "bool", True, ("shown", True)),
    ("weight", "Value<u16>", "number", 700, ("weight", 700)),
    ("status_colors", "Vec<(Key, Value<Paint>)>", None, {"success": "= role.global.status.success.standalone"},
     ("status_colors", {"success": "= role.global.status.success.standalone"})),
    ("durations", "Vec<(Key, Value<Ms>)>", None, {"button": "200ms"}, ("durations", {"button_ms": 200})),
    ("durations", "Vec<(Key, Value<Ms>)>", None, {"button_ms": 200}, ("durations", {"button_ms": 200})),
    ("axes", "Vec<(AxisTag, AxisValue)>", None, {}, ("axes", {})),
    ("symbol_variants", "Vec<Key>", None, [], ("symbol_variants", [])),
    ("padding", "Sides<Value<Px>>", None, {"top_px": 5, "right_px": 17, "bottom_px": 5, "left_px": 17},
     ("padding", {"top_px": 5, "right_px": 17, "bottom_px": 5, "left_px": 17})),
    ("timing", "Timing", None, {"cubic_bezier": [0.25, 0.1, 0.25, 1.0]},
     ("timing", {"cubic_bezier": [0.25, 0.1, 0.25, 1.0]})),
]

# the matcher both sides of the round trip share (S7, S7.2, S10): (when, combination, exclusive groups, recency sets,
# whether it holds)
_TOOLBAR = {"widget": "toolbar", "part": "bar", "variants": set(), "states": set(), "sizes": set()}
_WINDOW = {"widget": "window", "part": "title_bar", "variants": set(), "states": set(), "sizes": set()}
HOLDS_CASES = [
    ({"states": ["hover"]}, {"states": ["hover", "pressed"]}, [["pressed", "hover"]], {}, False),
    ({"states": ["pressed"], "not_states": ["hover"]}, {"states": ["hover", "pressed"]}, [["pressed", "hover"]], {},
     True),
    ({"states": ["checked"], "latest": {"glow": "hover"}}, {"states": ["checked", "hover"]}, [],
     {"glow": ["checked", "hover"]}, False),
    ({"states": ["checked"], "latest": {"glow": "hover"}},
     {"states": ["checked", "hover"], "latest": {"glow": "hover"}}, [], {"glow": ["checked", "hover"]}, True),
    ({"latest": {"glow": "checked"}}, {"states": ["checked", "hover"]}, [], {"glow": ["checked", "hover"]}, True),
    ({"sizes": ["small", "regular"]}, {"sizes": ["regular"]}, [], {}, False),
    ({"conditions": ["contrast_high"], "not_conditions": ["window_inactive"]},
     {"conditions": ["contrast_high", "scheme:dark"]}, [], {}, True),
    ({"direction": "rtl"}, {}, [], {}, False),
    ({"position": {"among": "any", "first": False}}, {}, [], {}, False),
    ({"position": {"among": "any", "first": False}}, {"position": {"among": "any", "first": False}}, [], {}, True),
    ({"context": [{"scope": {"nearest": ["toolbar", "window"]}, "widget": "toolbar", "part": "bar"}]},
     {"ancestors": [_WINDOW, _TOOLBAR]}, [], {}, False),
    ({"context": [{"scope": "any", "widget": "toolbar", "part": "bar"}]}, {"ancestors": [_WINDOW, _TOOLBAR]}, [], {},
     True),
    ({"not_context": [{"scope": "any", "widget": "toolbar"}]}, {"ancestors": [_TOOLBAR]}, [], {}, False),
]

# what each fixture fact becomes (the docstring's *What is left out*), by its place in the fixture
SELF_TEST_STATUS = [
    ("button.body.fill", "#eeeeee", (SHADOWED, "")),
    ("button.body.fill", "= alpha(surface.foreground, 0.1)", (SHADOWED, "")),
    ("button.body.padding", {"top_px": 1, "right_px": 1, "bottom_px": 1, "left_px": 1}, (SHADOWED, "")),
    ("button.body.padding", {"top_px": 2, "right_px": 2, "bottom_px": 2, "left_px": 2}, ("rule", "")),
    ("button.body.corners.bottom_right", "absent", (LEFT_OUT, RADIUS_KEYWORD)),
    ("button.body.fill", "U1", (PENDING, "")),
    ("button.label.text.font.weight", 700, (LEFT_OUT, BASE_CLASH)),
    ("pull_to_refresh.body.padding", {"top_px": 0, "right_px": 0, "bottom_px": 0, "left_px": 0}, (LEFT_OUT, NOT_HELD)),
    ("role.window.background", "#ffffff", ("rule", "")),
    ("role.global.accent", "= input.accent", (LEFT_OUT, READ)),
    ("input.selected", "#ffffff", (LEFT_OUT, INPUT_WHEN)),
    ("token.kde.frame_width", "2px", (LEFT_OUT, NOT_HELD)),
    ("button.body.fill", "#000000", (LEFT_OUT, f"{UNIT_NOT_ENABLED} (toolbar)")),
]


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
    for name, typ, dim, value, want in SPELL_CASES:
        try:
            got = spell(name, typ, dim, value)
        except Untranscribable as err:
            got = str(err)
        if got != want:
            failures.append(f"spell({name!r}, {typ!r}, {value!r}): {got!r}, expected {want!r}")
        elif isinstance(want, tuple) and normal(unspell(want[0], want[1], typ)[1]) != normal(canonical(value, typ)):
            failures.append(f"unspell({want[0]!r}, {want[1]!r}): not {value!r}")
    for when, d, groups, recency, want in HOLDS_CASES:
        if holds(when, combo(d), groups, recency) != want:
            failures.append(f"holds({when}, {d}): not {want}")
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp) / "fixture"
        write(root, SELF_TEST_FILES)
        ctx = fixture_context(root)
        draft, text, trip, units = generate(ctx, "adwaita")
        # a fact set and its expected preset, byte for byte
        if text != SELF_TEST_EXPECTED:
            import difflib
            diff = "".join(difflib.unified_diff(SELF_TEST_EXPECTED.splitlines(True), text.splitlines(True),
                                                "expected", "generated"))
            failures.append(f"the fixture's draft differs:\n{diff}")
        # a pending fact left out, an untranscribable fact reported, each by its reason
        status = {(f.key, repr(f.item.data.get("value", f.item.data.get("pending")))): (f.status, f.reason)
                  for f in draft.facts}
        for key, value, want in SELF_TEST_STATUS:
            if status.get((key, repr(value))) != want:
                failures.append(f"{key} = {value!r}: {status.get((key, repr(value)))}, expected {want}")
        lines = share_lines(draft, units, trip)
        for needle in ("1 pending", "pending (register id: facts): U1: 1", f"{READ} 1", f"{NOT_HELD} 2",
                       "18 rules", f"{UNIT_NOT_ENABLED} (toolbar) 1", "[declare] left out: toolbar"):
            if not any(needle in line for line in lines):
                failures.append(f"the report lacks {needle!r}")
        # grouping and order: the second hover rule after the flat rule that states `fill` between them; a later
        # fact joins an earlier rule when no rule after it states its key
        summary = [(check_facts.canonical_when(r.when), r.target, [f.key for f in r.facts]) for r in draft.rules]
        hover = check_facts.canonical_when({"states": ["hover"]})
        flat = check_facts.canonical_when({"variants": ["flat"]})
        want = [(hover, "button.body", ["button.body.corners.top_left"]),
                (flat, "button.body", ["button.body.fill", "button.body.corners.bottom_left"]),
                (hover, "button.body", ["button.body.fill", "button.body.corners.top_right"])]
        if summary[:3] != want:
            failures.append(f"grouping and order: {summary[:3]}")
        narrowed = [r.when for r in draft.rules if r.target == "colors"]
        if {"conditions": ["contrast_high", "scheme:light"]} not in narrowed:
            failures.append(f"a role fact alive in one scheme is not narrowed to it: {narrowed}")
        # a state model to [declare]
        if [(h, [k for k, _ in kvs]) for h, kvs, _ in draft.declare] != [
                (("widget", "button"), ["available", "variants", "sizes", "densities", "flags", "exclusive",
                                        "exclusive_variants", "recency"])]:
            failures.append(f"[declare]: {[(h, kvs) for h, kvs, _ in draft.declare]}")
        if draft.conditions != ["@reduced_chrome"] or [w for w, _ in draft.left_models] != ["toolbar"]:
            failures.append(f"[declare]: conditions {draft.conditions}, left out {draft.left_models}")
        # the round trip: every combination as its fact gives it; and it fails a draft that is not the facts'
        if trip.mismatches or not trip.compared or not trip.pending or not trip.left_out or trip.impossible != 1:
            failures.append(f"round trip: {trip}")
        tampered = {
            "a base value changed": text.replace("min_width_px = 16", "min_width_px = 15"),
            "the last rule dropped": text[:text.rindex("[[rules]]")],
            "the rules reversed": "\n[[rules]]\n".join([text.split("\n[[rules]]\n")[0]] + list(reversed(
                text.split("\n[[rules]]\n")[1:]))),
            "a value no fact gives": text.replace("transitions = []\n", "transitions = []\nopacity = 1\n"),
            "no TOML": text + "[[rules]\n",
        }
        for what, bad in tampered.items():
            if not round_trip(draft, bad).mismatches:
                failures.append(f"round trip: {what}: passes")
        swapped = Draft(**{**draft.__dict__,
                           "rules": [draft.rules[2], draft.rules[1], draft.rules[0]] + draft.rules[3:]})
        if not round_trip(swapped, render(swapped)).mismatches:
            failures.append("round trip: rules out of the facts' order pass")
        try:
            generate(ctx, "no-such-preset")
            failures.append("an unknown preset: accepted")
        except InputError:
            pass
        bad = Path(tmp) / "bad"
        write(bad, {**SELF_TEST_FILES, "docs/facts/gnome/broken.toml": "[[fact]\n"})
        try:
            generate(fixture_context(bad), "adwaita")
            failures.append("a broken fact file: accepted")
        except InputError:
            pass
    with contextlib.redirect_stderr(io.StringIO()):
        if main(["generate_preset.py"]) != 2 or main(["generate_preset.py", "adwaita", "--bogus"]) != 2:
            failures.append("a wrong command line is not exit 2")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def main(argv) -> int:
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    if not args or args[0].startswith("-"):
        print(__doc__, file=sys.stderr)
        return 2
    preset, out = args.pop(0), None
    while args:
        a = args.pop(0)
        if a == "--out" and args:
            out = Path(args.pop(0))
        else:
            print(__doc__, file=sys.stderr)
            return 2
    try:
        ctx = check_facts.Context(Path("."))
        draft, text, trip, units = generate(ctx, preset)
    except (OSError, UnicodeDecodeError, InputError, check_facts.InputError, check_facts_trace.InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    if out is None:
        sys.stdout.write(text)
    else:
        out.write_text(text, encoding="utf-8")
    for line in share_lines(draft, units, trip):
        print(line, file=sys.stderr)
    for m in trip.mismatches:
        print(f"round trip: {m}", file=sys.stderr)
    return 1 if trip.mismatches else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
