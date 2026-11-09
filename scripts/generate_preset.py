#!/usr/bin/env python3
"""A preset's rules generated from its verified facts, as far as they transcribe (specification S10, S11, S17; the
0.8.0 plan's checkpoint D6: "whether each unit's preset rules are generated from its verified facts instead of
written … decided by how closely the slice's presets transcribe their facts"). Offline.

Reads the facts as scripts/check_facts.py loads them (`docs/facts/<platform>/*.toml` and its `state-models.toml`;
`imported/` and `staged/` are no facts), the core and the widget catalogue through it, and the enabled units as
scripts/check_facts_trace.py reads them (`scripts/widget_units.toml`, its `[prefixes]` and `[staged]` too). Writes the
preset's TOML (S11): `[preset]`, `[declare]`, the bases and `[[rules]]`, with an empty block for each part written by
hand (below); `--write` writes it over the bundled preset, carrying that preset's hand-written blocks into it, and the
Rust reader validates it there; where the round trip finds a mismatch, `--write` writes nothing.

Run from the repository root:

    python3 scripts/generate_preset.py <preset-id> [--out FILE]   # the generated text to FILE (default stdout)
    python3 scripts/generate_preset.py <preset-id> --write        # native-theme/src/presets/<id>.toml
    python3 scripts/generate_preset.py <preset-id> --check        # whether --write would change that file
    python3 scripts/generate_preset.py --self-test

The bundled presets are those of check_facts.PRESET_DIR, `native-theme/src/presets/`.

On stderr: the transcription share (facts transcribed, shadowed, pending, left out by reason; per widget and per
unit), the rule and base-value counts, the pending facts' register ids, the facts gaps (a field no fact gives where
the reader needs it), the model fields the preset leaves unstated (below), and the round trip's result. Exit 0, 1 when
the round trip finds a mismatch (a generator bug), `--check` finds the bundled preset differs (a stale hand-written
citation included) or `--write` or `--check` finds a hand-written citation whose fact it cannot find (below), 2 on an
input it cannot read, a hand-written part `--write` would lose (below) or a wrong command line.

The model fields the preset leaves unstated (unstated_fields())
---------------------------------------------------------------
The model is read from its Rust sources as text, no build needed (read_model(): every `pub struct` of
check_facts.MODEL_DIR but its tests, and each `widgets/gen_<widget>.rs` scripts/generate_widgets.py writes, its
parts' `#[part(fields(…))]` and its values struct). Each field of the colour roles per scheme, the tokens and each
typed widget's parts and values with no base value and no `[pending]` path in the preset as `--write` writes it (the
bundled preset's hand-written blocks carried in) is listed by its TOML path, grouped by the preset's facts at, above or
below it: no fact; facts, none at rest (rules alone state it, and the reader needs a base, S11); a fact at rest that
gives no base (left out, by its reason, or pending and not listed). The rules, the inputs (S9) and the entries
`[declare]`'s keyed namespaces declare are not checked: loading the preset in Rust stays the full check.

The hand-written parts (`--write`)
----------------------------------
S11 and S17 name what no fact states, which the preset's task writes by hand: `[preset] generation`, the `[declare]`
keyed namespaces (`text_styles`, `durations`, `params` with their units, …; every `[declare]` key but `conditions` and
the state models) and the `read` facts as bases (S17 *`read`*: the reference a key takes from a reader input). The
generated text holds one block per part, where its lines go: `preset` in `[preset]` after `id`; `declare` in
`[declare]` after `conditions`, before the state models (its lines may open `[declare.params.<group>]` tables); and
`bases <table>` at the start of each base table a `read` fact's key lies in (read_tables(): a colour role's per scheme
it holds in at rest, any other key's where its `when` is empty); and `rule <key> <when>` as the `set` of a rule of
its own for a `read` fact under another `when` (S17 *`read`*, *Which fact applies*: the rule under the fact's `when`,
placed in the facts' order among its key's rules, so the fact applies exactly where it holds and no later one does;
the base beneath it is what the key's other facts give, a pending one's path listed, S17, S19.1). A block is the lines between `# hand-written: begin <part>` and `# hand-written: end <part>`,
markers included in the generated text, the lines inside written by hand, each value with its citation comment.

`--write` regenerates every line outside the blocks and carries each block's lines into the new text exactly, so no
hand-written line is dropped, changed or doubled, by this form:
- the text's fifth line, `# generated lines: sha256 <hex>`, states the digest of every other line outside the
  blocks (markers included). `--write` refuses a bundled preset whose lines outside the blocks have another digest:
  a line edited, added or removed there by hand, which the regeneration would drop;
- a block is found by its part's markers alone, each part once; a marker out of place, a part twice or one never
  ended is refused, as is a non-empty block whose part the new text has no place for (its `read` fact gone): a person
  moves or deletes those lines;
- the result is verified before it is written (verify()): each committed block's lines are its part's block in the
  result, the same lines in order, no block holds lines the committed file's does not, the digest holds, and the
  text is valid TOML, so a hand-written key the generator now writes too is a duplicate key, refused.
Without a bundled preset `--write` writes the generated text, its blocks empty. `--check` compares what `--write`
would write with the bundled file, byte for byte.

The hand-written citations (Citations)
--------------------------------------
A hand-written value carries its fact's citation too (S11: "Every value carries its fact's citation in a comment"),
`<file>:<line>` naming the fact's `[[fact]]` (or `[[model]]`) header, which moves when the facts files change.
`--write` makes each citation it can judge true, in place: it keeps one that names a fact of the preset (the facts
above, a state model's too) naming its subject, and points a stale one at the first such fact in the file it names,
else in the platform's files in their order; it changes nothing else on the line. Their subjects:
- a `[declare]` list entry, `"<name>",  # <citation>`, of a keyed namespace (`text_styles = [`, a
  `[declare.params.<group>]` table's `keys = [`): a fact naming the entry where the model holds the namespace's keys
  (NAMED_BY): its key holding it (`role.@<name>` for `color_roles`, `token.text.keyed.<name>.…`,
  `token.params.<group>.<name>`, …), its value at the namespace's field (`symbol_scales`: an IconStyle `scale`), a key
  of its table there (`symbolic_colors`), or a `when` naming `flavor:<name>` (the default flavour, the first: also a
  state model naming no flavour);
- a `read` fact's value in a `bases <table>` or `rule <key> <when>` block, its citation the comment line straight
  above `<key> = …`: the `read` fact read_tables() places at that table and key;
- `[preset] generation`, `# docs/facts/sources.toml:<line> (<source id>, …)`: that `[[source]]` entry's header.
A stale citation whose fact none is found for (no fact names the entry, a namespace NAMED_BY has no rule for, a
generation comment naming no source id) is never guessed: it is listed (`citation: …`) and the run exits 1, `--write`
writing the rest. `--check` lists each stale citation (`stale citation: …`) and fails, as `--write` would change the
file. Other citations in the blocks (in a comment's prose) are not judged.

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
- unit not enabled: a fact whose `when`'s positive lists name a widget the model does not hold (a `context` entry, its
  `nearest` scope, a position's `among`, `previous`, `next`), counted by that widget (its unit): the reader's S12
  checks refuse a rule naming an undeclared widget, and the fact comes back when its unit is enabled and the preset
  is regenerated (check_facts_trace.py's rule for trace lines). A negative list adds no unit (S17, ADD-A128): a
  `not_context` entry naming such a widget holds, as no such widget exists, and is left out of the fact's `when`.
- These two come first, a pending fact of a key or unit not held counting as left out, as the gate skips it.
- read: a fact with a `read` citation (S17 *`read`*: the reference a key takes from a reader input), which D6 lists
  among the parts written by hand.
- accent run: a fact with `scheme_file` and `accent` (S17 *The accent run*), which the preset's own resolution never
  reaches.
- two files: a key whose facts lie in two files, whose order S17 *Which fact applies* does not give; the accent
  run's facts count in no key's files, as they apply in that run alone.
- an S11 spelling a fact alone does not give (spell()): a `Scalar` entry stated with a unit outside a `token.params`
  group whose literals share one unit (param_units(): S11's "its number in the unit its declaration names", the
  group's one unit, S10, so `token.params.pointer_repeat.delay = "500ms"` is `params.pointer_repeat.delay = 500`, the
  hand-written `[declare.params.pointer_repeat]` declaring `unit = "ms"`); `"absent"`, `"auto"` or `"none"` on a `Radius` corner (S11
  names `<corner>_px` for `Radius::Px` and the bare name for its other variants, neither for a `Value` keyword); a
  unit literal of another dimension than the key's, or under a key whose type the core does not give (its suffix is
  unknown); a keyed segment (`@…`) outside `role.@…`; a whole block and a leaf of it both in one base table (S11 states
  a block in a table whole or per leaf; S2.2 composes a base's `Fields` patch over a reference before every rule,
  which a rule cannot state, so neither form is the fact's).
- an input whose `when` is not a base (S9: an input has per-scheme and per-contrast bases and no rules): anything but
  at most one `scheme:` and one of `contrast_increased`, `contrast_high` in `conditions`.
- a set-whole block's leaf its siblings do not join: a leaf of an `Option<B>` struct block (`token.behavior.key_repeat`),
  which S6.1 sets whole and S11 states as one table, whose base table or rule would not state every leaf, as the
  leaves' facts have different `when`s (split_blocks()); S17 composes per-leaf facts into one value per combination,
  which the gate does, and this draft writes it only where the leaves fall in one base or rule.
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
- A widget's or a token's key with no fact at rest whose rules cover an axis, one fact under each scheme alone or (a
  widget's field) under each of its sizes alone or under a variant alone and its complement alone, gets a base too
  (covering_bases()): the reader needs a value for every field. Those rules replace the base in every combination
  where every element holds one of them: always for the schemes and for a variant and its complement, and for the
  sizes where the widget states `default_size` (S10: "the size of an element that states none") and each variant whose
  entry takes no size has a stated fact under it alone, its elements holding no size. Its value is the light scheme's
  fact's, the default size's, or the complement's, the round trip checking that it never shows (its minimal
  combinations take the size an element holds, S10, ADD-A131: of the sizes it takes, its variant entry's or the
  widget's less those its context entry leaves out, the default in force, the context entry's, the variant entry's
  or the widget's, else, the default pending or refused by the `when`, the first the `when` allows, a variant whose
  entry takes one added where the widget's own are all refused or do not hold the size the `when` names, and which
  no `not_drawn` entry excludes; element_size()).
  Where the widget states no `default_size`, an element stating no size holds none (iOS's button), and no fact gives
  the field's value there: no base is written (a size's fact would be a value invented for the element with no size),
  the field is left unstated, so the reader reports it as S12's missing field, and the gap is reported as a facts gap.
- Facts with an equal `when` (check_facts.canonical_when) and an equal target group into one rule: a fact joins the
  last rule with its `when` and target unless a later rule states its key or a key above or below it (a block and its
  leaves), or the rule's `set` already holds a key above or below it in TOML, and else starts a new rule at the end.
  So for every key the rules state its facts in their file order, and the last matching rule is the last matching
  fact (the round trip below checks it).
- Inputs (S9): the six cells (light, dark; no contrast, `contrast_increased`, `contrast_high`) each take the fact that
  applies there; `light` and `dark` the no-contrast cells', `contrast.<increased|high>.<scheme>` each scheme's where
  it differs from that scheme's no-contrast fact and is transcribed (S11: a contrast table states `light`, `dark` or
  both, a scheme it leaves out taking the per-scheme value there, ADD-A132); a pending contrast cell states nothing
  and is never listed, the per-scheme value standing beneath it (S17).

Targets and paths (S11): `role.<set>.<role>` is `surfaces.<set>.<role>` of `colors` (`role.<set>.disabled.<role>`
below it), `role.global.<…>` is `global.<…>`, `role.@<key>` is `keyed.<key>` (S3's `ColorRoles.keyed`, a map whose
key is a `Key`, bare, S11); `token.<path>` is `<path>` of `tokens`; `<widget>.<part>.<field…>` is `<field…>` of
target `<widget>.<part>`, `<widget>.values.<name>` of `<widget>.values`, `component.<key>.…` likewise; a whole part's
`"absent"` is `absent = true`, its `"present"` `absent = false` and its `{ drawn_as = … }` is `drawn_as = …` (S6.3,
S11, S17); `input.<name>` is `[inputs.<name>]`.

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
`densities`, `flags`, `exclusive`, `exclusive_variants`, `recency`, `requires_variants`, `sizes_by_variant`,
`sizes_by_context` (each entry with its `default_size`; an entry whose context names a widget the model does not
hold left out, held_contexts()), and `data`, per part (`values` for the widget's values),
the element data the expressions of its transcribed facts read (S8, S10: an `element.` operand but the root's inputs,
`body_font_height`, another part's font metric `widget.<part>.<metric>`), those facts' citations above it, on the
widget's own declaration, in force under each flavour (element_data()); a pending default size (`default_size = {
pending = "U<n>" }`, the widget's or an entry's, S10's *A pending default*) is stated nowhere and listed in
`[pending]` by its
declaration's path, `declare.widget.<w>.default_size`, `….sizes_by_variant.<variant>.default_size`,
`….sizes_by_context.<n>.default_size`, a flavour's under `declare.flavor.<f>.` (S17, ADD-A131;
pending_defaults()); `axes`, `variant_sets`, `variant_products` and `not_drawn` are the gate's,
not declarations. A declared variant that facts name, every one of them left out as a unit it belongs to is not
enabled (its key's, or a widget its `when` names), is left out of `variants` and `exclusive_variants`
(off_variants()), as the facts gate leaves it out of the state model it checks the declarations against (S17,
ADD-A128); a variant no fact names is declared, and one S12 exempts stays (`requires_variants`, `sizes_by_variant`,
the catalogue's content variants). `[declare] conditions` lists the `system_setting:` keys the entries' axes and the facts' `when`s
name, as `@<key>`. The other keyed namespaces S10 declares (`color_roles`, `durations`, `timings`, `params` with their
units, `text_styles`, `inputs`, `accent_inputs`, `surface_sets`, …) are left for hand-writing: no fact states their
units or defaults.

The round trip (the exactness proof)
------------------------------------
The draft is read back with tomllib: each transcribed fact's value read at its TOML path (in its base table, or in its
rule's `set` below that rule's `when` and `target` as read back) and turned back into the fact's spelling (unspell():
the suffix into the value's unit, a map's entries likewise, `absent = true` into `"absent"`, `absent = false` into
`"present"`); a value at no transcribed fact's path is a mismatch. Then (1) for every key the read-back sequence (bases,
then the rules in order) equals the transcribed facts' sequence, `when` by `when` (canonical_when) and value by value;
and (2) for every key, every distinct `when` of the facts of its widget (or of the colour roles, the tokens, the inputs)
and each scheme, that `when`'s minimal combination (its required names present and nothing else; ancestors as its
contexts list them; a lone element where it names no position) resolves under the generated bases and rules (the last
matching rule winning; a keyed list, S6.1's transitions, bindings and presentations, the base's items and each matching
rule's in order merged by their keys, keyed_merge(), each transition split per group, as S17 states such a fact, the
merged list) to the value of the fact that applies there among all the key's facts, or to none where none holds (the
last in file order, after the state model's exclusive reduction and its recency sets' stated order, S7.2, S10, S17); a
combination where a pending or a left-out fact applies is skipped and counted, and a fact whose `when` no element can
hold (two sizes, a required name refused, a state its group drops) is counted apart. Any difference is a mismatch: a
keyed-list fact that leaves out an item of the base its rule merges over is one, as the merge keeps that item.

    python3 scripts/generate_preset.py --self-test   # fixture facts against the expected draft, and each rule
"""
import contextlib
import hashlib
import io
import json
import re
import sys
import tempfile
import tomllib
from dataclasses import dataclass, field, replace
from pathlib import Path

import check_facts
import check_facts_trace
import check_preset_citations
from generate_platform_facts import toml_inline, toml_key

SCHEMES = ("light", "dark")
CONTRASTS = (("contrast_increased", "increased"), ("contrast_high", "high"))
SUFFIX_OF = {"length": "_px", "duration": "_ms", "angle": "_deg"}
UNIT_OF_SUFFIX = {"_px": "px", "_ms": "ms", "_deg": "deg", "_pt": "pt"}
MAP_TYPE = re.compile(r"Vec<\(\s*(\w+)\s*,\s*(.+)\)>")
DECLARE_KEYS = ("available", "variants", "sizes", "default_size", "densities", "flags",
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
SET_WHOLE_SPLIT = "a set-whole block's leaf its siblings do not join"
SET_WHOLE_PENDING = "a set-whole block's leaf beside a pending sibling"
NO_STRUCT_BLOCK = ("Paint",)   # S2.2: "A `Value<Paint>` is no struct block"
# S8, S10: the element data an expression reads, which its part declares in `data`: an `element.` operand but the
# root's inputs, `body_font_height`, and another part's font metric `widget.<part>.<metric>`
DATA_OPERAND = re.compile(r"(?<![\w.])(?:element\.(\w+)|(body_font_height)|widget\.(\w+)\.(\w+))(?![\w.])")
ROOT_INPUTS = ("window_width", "window_height", "screen_width", "screen_height")
FONT_METRICS = ("char_width", "max_char_width", "font_x_height", "font_height", "font_underline_offset",
                "font_underline_thickness", "font_strikeout_offset", "font_strikeout_thickness", "font_size",
                "line_height")


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
    whole: bool = False       # a table at a `Value` struct block: `{ replace = { … } }` in a rule (whole_block())
    param_unit: str = ""      # a keyed param's group unit, its unit literal's (param_units()), its number in it
    hand: str = ""            # a `read` fact whose `when` is no base: its rule's hand-written part (read_tables())

    @property
    def where(self):
        return f"{self.item.file}:{self.item.line}"


@dataclass
class Rule:
    when: dict
    target: str
    facts: list = field(default_factory=list)
    tree: dict = field(default_factory=dict)   # its `set` as nested tables, to refuse a key above or below another
    hand: str = ""            # the hand-written part its `set` is, a `read` fact's (read_tables()); no facts then


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


def entry_widgets(e) -> set:
    """The widgets a context entry names: its `widget` and its `nearest` scope's."""
    out = set()
    if isinstance(e, dict) and isinstance(e.get("widget"), str):
        out.add(e["widget"])
    scope = e.get("scope") if isinstance(e, dict) else None
    if isinstance(scope, dict) and isinstance(scope.get("nearest"), list):
        out |= {w for w in scope["nearest"] if isinstance(w, str)}
    return out


def context_widgets(when) -> set:
    """The widgets a `when`'s positive lists name as an ancestor or a sibling (its `context` entries and their `nearest`
    scopes, its position's), the units a fact belongs to beside its key's (S17, ADD-A128). A negative list adds none:
    `not_context X` holds wherever X's unit is not enabled, as no X exists there (without_unheld_negations())."""
    out = set()
    for e in when.get("context", []) if isinstance(when.get("context"), list) else []:
        out |= entry_widgets(e)
    pos = when.get("position") if isinstance(when.get("position"), dict) else {}
    if isinstance(pos.get("among"), list):
        out |= {w for w in pos["among"] if isinstance(w, str)}
    for k in ("previous", "next"):
        if isinstance(pos.get(k), dict) and isinstance(pos[k].get("widget"), str):
            out.add(pos[k]["widget"])
    return out


def without_unheld_negations(when, units) -> dict:
    """`when` without its `not_context` entries naming a widget the model does not hold: no such widget exists, so the
    negation holds and the fact applies everywhere else, in its key's unit alone (S17, ADD-A128), as the facts gate reads
    it; a rule keeping the entry would name an undeclared widget, which S12 refuses. A list emptied so goes."""
    entries = when.get("not_context")
    if not isinstance(entries, list):
        return when
    kept = [e for e in entries if all(held(units, w, "widget") for w in entry_widgets(e))]
    out = {k: v for k, v in when.items() if k != "not_context"}
    if kept:
        out["not_context"] = kept
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


def takes_suffix(typ, suffix) -> bool:
    """Whether a key of type `typ` is spelled with the unit suffix `suffix` (spell()): a length's, duration's or
    angle's own, a font size's `_px` or `_pt`, a `Radius` corner's `_px`; a key of no such type keeps a name that ends
    so as it is (a map's key `edge_px` of a unitless value)."""
    t = check_facts.strip_type(typ) if typ else None
    if t == "Radius":
        return suffix == "_px"
    dim = check_facts.KEY_DIMS.get(t) if t else None
    if dim == "font_size":
        return suffix in ("_px", "_pt")
    return SUFFIX_OF.get(dim) == suffix


def unspell(name, value, typ):
    """(the fact's last key segment, the fact's value) of a TOML key and value: spell()'s inverse."""
    for suffix, unit in UNIT_OF_SUFFIX.items():
        if name.endswith(suffix) and takes_suffix(typ, suffix):
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


PARAM_UNITS = ("px", "ms", "deg")


def param_units(facts) -> dict:
    """{group: unit}: each `token.params` group whose facts state unit literals all of one unit, which S10's
    declaration names (`[declare.params.<group>] unit`, one unit per group, S4.6) and S11 spells each value in, its
    number alone (`params.pointer_repeat.delay = 500` for `500ms`); a group mixing units has none, its literals
    left out as S11 gives them no spelling."""
    seen = {}
    for f in facts:
        value = f.item.data.get("value")
        unit = check_facts.UNIT_LITERAL.match(value) if isinstance(value, str) else None
        if f.kind == "token" and len(f.ckey) == 4 and f.ckey[1] == "params" and unit:
            seen.setdefault(f.ckey[2], set()).add(unit.group(2))
    return {g: next(iter(us)) for g, us in seen.items() if len(us) == 1 and next(iter(us)) in PARAM_UNITS}


def place(f: Fact):
    """Set the fact's target, base table and TOML keys below them (the docstring's *Targets and paths*), its value
    spelled; Untranscribable where S11 gives no spelling. A pending fact has no value: its key is spelled by its type
    (its unit's suffix), as its `[pending]` entry spells it."""
    segs = list(f.ckey)
    value = f.item.data.get("value")
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
            if "pending" in f.item.data:
                # a pending presence: no value, its key the part's (S17, ADD-A131)
                f.header, f.rel, f.dotted, f.value = header, (), (), None
                return
            if value == "absent":
                rel, value = ["absent"], True
            elif value == "present":
                rel, value = ["absent"], False
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
    if f.param_unit:
        # a keyed param (S4.6): its number in the unit its group declares (S10, S11), no suffix
        f.rel = tuple(rel)
        below = len(header)
        f.dotted, f.value = f.rel[below:], number(check_facts.UNIT_LITERAL.match(value).group(1))
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
    models: dict              # (widget, flavour or None): the state model's data the round trip reduces by
    left_models: list         # (widget, item): entries of widgets the model does not hold
    staged: int
    dropped: list = field(default_factory=list)   # (widget, [variant]): variants only units not enabled name
    pending: list = field(default_factory=list)   # (the listed path as S11 spells it, register id or ids, the first pending fact)
    gaps: list = field(default_factory=list)  # (widget, key, why): a field no stated fact gives where an element is drawn
    definitions: list = field(default_factory=list)  # (section, header, dotted, value, the defining section)
    read_tables: list = field(default_factory=list)  # (section, scheme or None, header): a `read` fact's base table
    read_rules: list = field(default_factory=list)   # `read` facts whose `when` is no base (read_tables())
    items: list = field(default_factory=list)        # every item check_facts.load() reads (the citations', Citations)


def accent_run(d) -> bool:
    """Whether a fact is the accent run's (S17 *The accent run*: `scheme_file` and `accent`)."""
    return "scheme_file" in d or "accent" in d


def key_files(facts) -> dict:
    """{key: the files its facts lie in}, the accent run's left out: they apply in the accent run alone (S17 *The
    accent run*), so no order between files is needed for them in the preset's own resolution."""
    files = {}
    for f in facts:
        if not accent_run(f.item.data):
            files.setdefault(f.ckey, set()).add(f.item.file)
    return files


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
        when = without_unheld_negations(preset_when(it.data.get("when"), preset), units)
        facts.append(Fact(it, it.data["key"], check_facts.canonical_key(it.data["key"]), when, info[3], info[2],
                          info[4], info[1]))
    files = key_files(facts)
    groups = param_units(facts)
    for f in facts:
        value = f.item.data.get("value")
        if (f.kind == "token" and len(f.ckey) == 4 and f.ckey[1] == "params" and f.ckey[2] in groups
                and isinstance(value, str) and check_facts.UNIT_LITERAL.match(value)):
            f.param_unit = groups[f.ckey[2]]
    for f in facts:
        d = f.item.data
        if not held(units, f.key, f.kind):
            f.status, f.reason = LEFT_OUT, NOT_HELD
        elif any(not held(units, w, "widget") for w in context_widgets(f.when)):
            off = sorted(w for w in context_widgets(f.when) if not held(units, w, "widget"))
            f.status, f.reason = LEFT_OUT, f"{UNIT_NOT_ENABLED} ({', '.join(off)})"
        elif "pending" in d:
            f.status = PENDING
        elif any(c.data.get("kind") == "read" for c in f.item.cites):
            f.status, f.reason = LEFT_OUT, READ
        elif accent_run(d):
            f.status, f.reason = LEFT_OUT, ACCENT_RUN
        elif len(files[f.ckey]) > 1:
            f.status, f.reason = LEFT_OUT, TWO_FILES
        else:
            try:
                place(f)
                f.whole = whole_block(ctx, f)
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
    tables, read_rules = read_tables(facts)
    rules = group_rules([f for f in facts if f.status == "rule" or f.hand])
    while True:
        split = split_blocks(ctx, bases, rules)
        if not split:
            break
        for f in split:
            f.status, f.reason = LEFT_OUT, SET_WHOLE_SPLIT
        bases = [b for b in bases if b[5] not in split]
        rules = group_rules([f for f in facts if f.status == "rule" or f.hand])
    declare, conditions, models, left_models, dropped, waiting = declarations(items, platform, preset, units, facts,
                                                                              ctx.content_variants)
    covering, gaps = covering_bases(facts, models, tree)
    bases += covering
    # a covering base can state a set-whole block's leaf alone: split again, the base dropped (its rules stay)
    split = split_blocks(ctx, bases, [])
    for f in split:
        if f.status == "base":
            f.status, f.reason = LEFT_OUT, SET_WHOLE_SPLIT
    bases = [b for b in bases if b[5] not in split]
    pending_blocks = {f.ckey[:-1] for f in facts if f.status == PENDING and set_whole_block(ctx, f)}
    for f in facts:
        if f.reason == SET_WHOLE_SPLIT and f.ckey[:-1] in pending_blocks:
            f.reason = SET_WHOLE_PENDING
    listed = pending_table(ctx, facts, bases, models=models, off=dict(dropped)) + waiting
    return Draft(preset, platform, facts, rules, bases, declare, conditions, models, left_models, staged, dropped,
                 listed, gaps, definitions(preset, tree), tables, read_rules, items)


def read_tables(facts) -> tuple:
    """The base tables the `read` facts' hand-written values go in (S17 *`read`*), each once, in the facts' order:
    a colour role's under each scheme it holds in at rest, any other key's where its `when` is empty. A `read` fact
    whose `when` is no base takes a rule of its own under that `when`, its `set` the hand-written part
    `rule <key> <when>` (the fact's `hand`; group_rules() places the rule in the facts' order, so a later fact of its
    key still applies after it, and the bases beneath it are the other facts', a pending one's listed); one whose key
    no rule can state is returned, reported."""
    tables, rules = [], []
    for f in facts:
        if f.reason != READ:
            continue
        g = Fact(f.item, f.key, f.ckey, f.when, f.kind, f.widget, f.typ, f.dim)
        try:
            place(g)
        except Untranscribable:
            rules.append(f)
            continue
        slots = slots_of(f.when)
        if f.kind == "role" and slots and all(is_rest(f, s) for s in slots):
            at = [(g.section, s, g.header) for s in slots]
        elif f.kind != "role" and not f.when:
            at = [(g.section, None, g.header)]
        else:
            place(f)
            f.rule_when, f.hand = f.when, f"rule {f.key} {toml_inline(f.when)}"
            continue
        tables += [t for t in at if t not in tables]
    return tables, rules


def definitions(preset: str, tree: dict) -> list:
    """The values the specification itself defines for the preset, which no fact states (S9.4's `font_dpi = 72` on
    `macos-*` and `ios-*`, S4.5's `animated_visuals = {}` on every preset but `windows-11`; S11, S17, ADD-A125,
    ADD-A131), as check_preset_citations.DEFINITIONS lists them: each a base value carrying its section as its
    comment (`# S9.4`). A fact stating such a key is refused by check_facts.py, so its base path is free."""
    out = []
    for section, (path, value, presets) in check_preset_citations.DEFINITIONS.items():
        if not presets(preset):
            continue
        keys = tuple(path.split("."))
        if not insert(tree, keys, Leaf(None)):
            raise InputError(f"{path}: a fact states the value {section} defines")
        out.append((keys[0], keys[1:2], keys[2:], value, section))
    return out


def unsuffixed(path: tuple) -> tuple:
    """A TOML path as a listed path compares (the reader's listed_path): its last key's unit suffix left out."""
    if not path:
        return path
    last = path[-1]
    for suffix in UNIT_OF_SUFFIX:
        if last.endswith(suffix):
            return path[:-1] + (last[:-len(suffix)],)
    return path


def whole_part(section, dotted, value) -> bool:
    """Whether a base states its part whole (S6.3): `absent = true` or `drawn_as`, not `absent = false`, which draws
    the part with its own fields."""
    if section not in ("widgets", "components"):
        return False
    return tuple(dotted) == ("drawn_as",) or (tuple(dotted) == ("absent",) and value is True)


def based_paths(bases) -> list:
    """Each base's path as a listed path compares (unsuffixed()); a whole part's (`absent = true`, `drawn_as`) the
    part's, which holds every field of it (S6.3); a drawn part's (`absent = false`) its own, `<part>.absent`, which
    no field's path lies above or below."""
    out = []
    for section, scheme, header, dotted, value, _ in bases:
        whole = whole_part(section, dotted, value)
        out.append(unsuffixed((section,) + ((scheme,) if scheme else ()) + header + (() if whole else tuple(dotted))))
    return out


def by_ckey(facts) -> dict:
    """{a key: its facts, in their order}: what alive() and ruled_beneath() read of `facts`, each key's list
    found once."""
    out = {}
    for f in facts:
        out.setdefault(f.ckey, []).append(f)
    return out


def alive(f: Fact, facts) -> tuple:
    """The schemes a fact applies in somewhere, no later fact of its key covering it there (S17 *Which fact
    applies*: the last in file order)."""
    same = [g for g in facts if g.ckey == f.ckey]
    at = next((i for i, g in enumerate(same) if g is f), len(same))
    later = same[at + 1:]
    return tuple(s for s in slots_of(f.when) if not any(covers(g.when, f.when, s) for g in later))


def ruled_beneath(f: Fact, facts, scheme, models=(), off=()) -> bool:
    """Whether earlier rules of the fact's key hold wherever the fact holds under the scheme, so the preset's own
    layer states the key beneath it there (S17: a listed path has "no fact or base beneath" it "in some combination"):
    one rule covering the fact's `when` (covers()), or, for a widget's fact, one at each element and size it holds
    for under each state model of `models` (elements_beneath(), the variants `off` left out), as a key stated per
    size alone is beneath every fact of a sized element and a rule refusing variants is beneath one of a variant
    exclusive with them."""
    same = [g for g in facts if g.ckey == f.ckey]
    at = next((i for i, g in enumerate(same) if g is f), len(same))
    earlier = [g for g in same[:at] if g.status == "rule"]
    if any(covers(g.rule_when or {}, f.when, scheme) for g in earlier):
        return True
    return bool(models) and all(elements_beneath(f, earlier, scheme, model, off) for model in models)


VARIANT_FIELDS = ("variants", "exclusive_variants", "variant_sets", "variant_products", "requires_variants",
                  "not_drawn")
_variant_options = {}   # variant_options() by what it reads: the model's VARIANT_FIELDS and `off`


def variant_options(model, off=()) -> list:
    """The variant sets a state model's elements hold (S17 *State models*, *Variant products*, as the gate's
    model::variant_options gives them): none, each variant, each set `variant_sets` lists and each set
    `variant_products` gives (every choice of one variant or none from each `any` group and one from each `one`
    group, those of two or more variants, none holding two of an `exclusive_variants` group), less those
    `requires_variants` excludes, those holding a variant `off` names (left out with its unit, off_variants()) and
    those a `not_drawn` entry of variants alone refuses; each once, sorted. Computed once per distinct input: the
    model's VARIANT_FIELDS, the only ones read, by their text, and `off` as a set, as it is read; each call gets its
    own lists."""
    key = (repr([model.get(k) for k in VARIANT_FIELDS]), frozenset(off))
    if key not in _variant_options:
        _variant_options[key] = computed_variant_options(model, off)
    return [list(held) for held in _variant_options[key]]


def computed_variant_options(model, off=()) -> list:
    """variant_options() computed."""
    variants = [v for v in model.get("variants", []) if isinstance(v, str)]
    groups = [set(g) for g in model.get("exclusive_variants", []) if isinstance(g, list)]
    sets = [[]] + [[v] for v in variants] + [s for s in model.get("variant_sets", []) if isinstance(s, list)]
    for product in model.get("variant_products", []):
        partial = [[]]
        for group in product if isinstance(product, list) else []:
            one = isinstance(group, dict) and "one" in group
            members = group.get("one" if one else "any", []) if isinstance(group, dict) else []
            partial = [held + extra for held in partial for extra in ([] if one else [[]]) + [[v] for v in members]]
        sets += [p for p in partial if len(p) >= 2 and len(set(p)) == len(p)
                 and not any(len(g & set(p)) > 1 for g in groups)]
    required = [v for v in model.get("requires_variants", []) if isinstance(v, str)]
    undrawn = [x for x in model.get("not_drawn", []) if isinstance(x, dict) and x
               and set(x) <= {"variants", "not_variants"}]
    out = []
    for held in sets:
        held = sorted(held)
        if held in out or (required and not set(required) & set(held)) or set(off) & set(held):
            continue
        if any(set_match(x, "variants", set(held)) for x in undrawn):
            continue
        out.append(held)
    return out


def elements_beneath(f: Fact, earlier: list, scheme: str, model, off=()) -> bool:
    """Whether at every element the widget's fact `f` holds for, and at each size it takes, an earlier rule of
    `earlier` holds wherever `f` does (covers(), the element's variants and size stated, the others refused). The
    elements are the gate's (S17 *State models*): each variant set variant_options() gives that `f`'s `variants` and
    `not_variants` admit, at the sizes its variant's `sizes_by_variant` entry takes, else the widget's (S10), those
    `f` names or does not refuse; one taking none (an unsized variant's) holds no size, so no size's rule is
    beneath it. False for a model with a `sizes_by_context` entry taking no size, which this does not follow, and
    where no element holds the fact."""
    if f.kind != "widget" or not isinstance(model, dict):
        return False
    if any(isinstance(e, dict) and not e.get("sizes") for e in model.get("sizes_by_context", [])):
        return False
    variants = [v for v in model.get("variants", []) if isinstance(v, str) and v not in off]
    wanted, refused = set(f.when.get("variants", [])), set(f.when.get("not_variants", []))
    named, refused_sizes = f.when.get("sizes"), set(f.when.get("not_sizes", []))
    entries = [e for e in model.get("sizes_by_variant", []) if isinstance(e, dict) and isinstance(e.get("sizes"), list)]
    widget_sizes = [z for z in model.get("sizes", []) if isinstance(z, str)]
    all_sizes = list(dict.fromkeys(widget_sizes + [z for e in entries for z in e["sizes"] if isinstance(z, str)]))
    held = False
    for held_set in variant_options(model, off):
        if not wanted <= set(held_set) or refused & set(held_set):
            continue
        entry = next((e for e in entries if e.get("variant") in held_set), None)
        taken = [z for z in (entry["sizes"] if entry else widget_sizes) if isinstance(z, str)]
        sizes = [z for z in taken if z not in refused_sizes and (named is None or z in named)]
        if taken and not sizes or not taken and named:
            continue
        held = True
        others = sorted(set(variants) - set(held_set))
        for z in sizes or [None]:
            at = {**f.when, "variants": held_set, "not_variants": others,
                  "sizes": [z] if z else [], "not_sizes": [y for y in all_sizes if y != z]}
            if not any(covers(g.rule_when or {}, at, scheme) for g in earlier):
                return False
    return held


def pending_table(ctx, facts, bases, block_of=None, models=None, off=None) -> list:
    """The preset's `[pending]` table (S11, S17 *A pending fact with nothing beneath it*, ADD-A128): each path a
    pending fact of the preset states where it applies (no later fact covering it everywhere) and the draft writes no
    base at it, above it (a block stated whole, a whole part) or below it, so the preset's own layer leaves it unstated
    where that fact applies; a colour role's per scheme the fact applies in; a pending leaf of a set-whole block lists
    the block, which its bases cannot state without it. Each with its pending facts' register id, or the list of
    their distinct ids in fact order where they differ (S11: `"tokens.behavior.key_repeat" = ["<id>", "<id>"]`, the
    leaves of a field set whole pending under different ids), and the first such fact: [(path, id or ids, fact)],
    the path spelled as S11 writes its key, unit suffix included. Earlier rules of the key holding wherever the
    pending fact does are beneath it too (ruled_beneath(), the state models `models` giving the elements and the
    sizes each takes, the variants `off` names per widget left out with their units), in that scheme. The list of
    ids is broader than S11's letter ("a list of ids where a field S6.1 sets whole has leaves pending under different ids"): every path whose
    pending facts carry several ids takes one, as a plain field pending under different ids per combination has no
    other encoding (iOS's and macOS's drafts; S11, ADD-A131: one id is the string, a list of one never written). A
    pending input is listed per scheme, as `inputs.<name>.<scheme>`, at each scheme where it applies with no
    contrast condition and the input's base states no per-scheme value there; a pending contrast base is never
    listed, the per-scheme value standing beneath it (S11, S17, ADD-A132). A pending presence (a whole part's fact)
    is listed by its part's path, `widgets.<widget>.<part>`, where no presence is beneath it: a base of the part's
    presence (`absent = true` or `false`, `drawn_as`), or an earlier rule of its key holding wherever it does; its fields' bases are
    none (S6.3, S17, ADD-A131). A component's (`components.<key>.<part>`) is refused (InputError) until components
    are typed, as the reader refuses that key."""
    block_of = block_of or set_whole_block
    based = based_paths(bases)
    same = by_ckey(facts)
    out, seen = [], {}
    for f in facts:
        if f.status != PENDING:
            continue
        presence = False
        if f.kind == "input":
            contrast = any(c in dict(CONTRASTS) for c in f.when.get("conditions", []))
            paths = [] if contrast else [("inputs",) + tuple(f.ckey[1:]) + (s,) for s in alive(f, same[f.ckey])]
        else:
            try:
                place(f)
            except Untranscribable:
                continue
            presence = f.kind == "widget" and not f.dotted
            if presence and f.section == "components":
                raise InputError(f"{f.key} ({f.item.data['pending']}): a component's pending presence, "
                                 f"`components.<key>.<part>`, has no [pending] key the reader takes until components "
                                 f"are typed (S11, S17; ledger #150)")
            held = models_holding(models or {}, f.widget, f.when)
            left_out = (off or {}).get(f.widget, ())
            slots = tuple(s for s in alive(f, same[f.ckey])
                          if not ruled_beneath(f, same[f.ckey], s, held, left_out))
            below = f.dotted[:-1] if block_of(ctx, f) else f.dotted
            schemes = slots if f.kind == "role" else (None,)
            paths = [(f.section,) + ((scheme,) if scheme else ()) + f.header + tuple(below)
                     for scheme in (schemes if slots else ())]
        for path in paths:
            key = unsuffixed(path)
            if presence:
                beneath = key in based or key + ("absent",) in based
            else:
                beneath = any(b[:len(key)] == key or key[:len(b)] == b for b in based)
            if beneath:
                continue
            uid = f.item.data["pending"]
            if key in seen:
                if uid not in seen[key][1]:
                    seen[key][1].append(uid)
                continue
            seen[key] = [dotted_key(path), [uid], f]
            out.append(seen[key])
    return [(path, uids[0] if len(uids) == 1 else uids, f) for path, uids, f in out]


def redrawn_pending(facts, bases) -> list:
    """The pending facts of a part's field whose part the draft's base states whole (`absent = true` or `drawn_as`)
    and that a fact draws again (`"present"`, S6.3's `absent = false`): no listed path, as the whole base states the
    field (based_paths()), and no stated value, so loading refuses the field as S12's missing one "judged under each
    combination in which its part resolves drawn" where that fact draws it. S17 gives such a field no encoding (task
    15c's review, minor 10): reported, the draft stating nothing for it. [(key, register id, fact)]"""
    wholes = {tuple(header) for section, _, header, dotted, value, _ in bases if whole_part(section, dotted, value)}
    present = {f.ckey for f in facts if f.kind == "widget" and f.status not in (PENDING, SHADOWED)
               and f.item.data.get("value") == "present"}
    same = by_ckey(facts)
    out = []
    for f in facts:
        if f.status != PENDING or f.kind != "widget" or not alive(f, same[f.ckey]):
            continue
        try:
            place(f)
        except Untranscribable:
            continue
        part = f.ckey[:len(f.target.split("."))]
        if len(f.ckey) > len(part) and tuple(f.header) in wholes and part in present:
            out.append((f.key, f.item.data["pending"], f))
    return out


def covering_bases(facts, models, tree) -> tuple:
    """The bases of keys whose facts are all rules that together cover every element (S11, S8): a widget's or a
    token's key with no fact at rest, one fact under each scheme alone (`{ conditions = ["scheme:<s>"] }`), or for a
    widget's field one under each of its sizes alone (`{ sizes = ["<size>"] }`), or one under a variant alone and one
    under its complement alone (`{ variants = ["<v>"] }`, `{ not_variants = ["<v>"] }`). The reader needs a base for
    every field (S11: a field left out is missing). Where every element holds one of the axis' values (a scheme
    always; a variant or its complement always; a size where the widget states `default_size`, S10), the axis' rules
    replace the base in every combination, so the base is one of those facts' values: the light scheme's, the default
    size's, or the complement's (the value of an element without the variant), S11 stating a value shared across
    schemes once outside the colour roles; the round trip checks that it never shows. The sizes are every size an
    element takes, the widget's and its `sizes_by_variant` entries' (S10, ADD-A131). Where the widget's default is
    pending, no element stating no size is drawn, and the base is the first size's. A variant whose entry takes no
    size draws its elements with none (S10: "an unsized element resolves with no size"), which no size's rule
    reaches: its fact alone (`{ variants = ["<v>"] }`) covers them, as the complement's covers the elements without
    it. A pending fact states nothing (S17 *A pending measurement*), so it covers no element: where it is the only
    fact of some elements, a base would show there, a value no fact gives (S10 rejects another size's value as
    invented), and the field stays listed instead (S17 *A pending fact with nothing beneath it*). Where the widget
    states sizes and no `default_size`, or a variant's entry takes no size and no stated fact under that variant alone
    covers its elements, an element holds a value no stated fact gives: no base, the field left unstated (S12's missing
    field when read), and reported as a facts gap but where a pending fact under that variant alone is to give it,
    which lists the field (pending_table()). The base is at rest under every flavour, so the sizes are read from each
    state model of the widget, its own and each flavour's (S10): a base where every one gives the same, else a gap. A
    part's presence (a fact of the part itself) takes no base from a variant and its complement: it needs none, a part
    no patch makes absent being drawn (S6.3). (the bases, the gaps as (widget, key, why))."""
    by_key = {}
    for f in facts:
        if f.kind in ("widget", "token"):
            by_key.setdefault(f.ckey, []).append(f)
    out, gaps = [], []
    for fs in by_key.values():
        if any(f.status == "base" for f in fs):
            continue
        rules = [f for f in fs if f.status == "rule"]

        def alone(when):
            want = check_facts.canonical_when(when)
            return next((f for f in rules if check_facts.canonical_when(f.rule_when) == want), None)

        chosen = None
        per_scheme = [alone({"conditions": [f"scheme:{s}"]}) for s in SCHEMES]
        if all(per_scheme):
            chosen = per_scheme[0]
        elif rules and rules[0].kind == "widget":
            complement = None
            for f in rules if rules[0].dotted else []:
                variants = f.rule_when.get("variants")
                if set(f.rule_when) == {"variants"} and isinstance(variants, list) and len(variants) == 1:
                    complement = alone({"not_variants": variants})
                    if complement:
                        break

            def sized(model):
                """(the base the sizes' rules give the elements `model` reduces by, why it gives none)."""
                sizes = [z for z in model.get("sizes", []) if isinstance(z, str)]
                entries = [e for e in model.get("sizes_by_variant", []) if isinstance(e, dict)
                           and isinstance(e.get("sizes"), list)]
                taken = list(dict.fromkeys(sizes + [z for e in entries for z in e["sizes"] if isinstance(z, str)]))
                per_size = {z: alone({"sizes": [z]}) for z in taken}
                unsized = [e.get("variant") for e in entries if not e["sizes"]]
                uncovered = [v for v in unsized if not (isinstance(v, str) and alone({"variants": [v]}))]
                if not (sizes and all(per_size.values())):
                    return None, None
                default = model.get("default_size")
                if isinstance(default, str) and default not in per_size:
                    # S12 refuses a default size the widget does not take, so this never shows
                    return None, (f"its `default_size` {default!r} is none of the sizes its elements take, which "
                                  f"S12 refuses")
                if isinstance(default, str) and not uncovered:
                    return per_size[default], None
                if isinstance(default, dict) and not uncovered:
                    # a pending default: no element states no size and is drawn (S10's *A pending default*), so
                    # every drawn element holds one of the sizes, or an unsized variant its fact covers, and the
                    # base never shows
                    return per_size[sizes[0]], None
                if isinstance(default, (str, dict)):
                    unmeasured = [v for v in uncovered if not any(
                        f.status == PENDING and check_facts.canonical_when(f.when)
                        == check_facts.canonical_when({"variants": [v]}) for f in fs)]
                    if unmeasured:
                        return None, (f"its variant {', '.join(map(str, unmeasured))} takes no size and no fact "
                                      f"under it alone gives the value of its elements")
                    return None, None
                return None, (f"`{rules[0].widget}` states sizes and no `default_size`: no fact gives the value of "
                              f"an element with no size")

            if complement:
                chosen = complement
            else:
                # the base is at rest under every flavour, so each flavour's state model, the widget's own
                # included, takes it (S10): one base where every model gives the same
                held = [(flavor, model) for (widget, flavor), model in models.items() if widget == rules[0].widget]
                found = [(flavor, *sized(model)) for flavor, model in held or [(None, {})]]
                why = [f"under the flavour {flavor}: {w}" if flavor else w for flavor, _, w in found if w]
                given = {id(base) for _, base, _ in found}
                gaps += [(rules[0].widget, rules[0].key, w) for w in why]
                if not why and len(given) > 1:
                    gaps.append((rules[0].widget, rules[0].key,
                                 "its flavours' state models give its base different values"))
                elif not why:
                    chosen = found[0][1]
        if chosen is None:
            continue
        path = (chosen.section,) + chosen.header + tuple(chosen.dotted)
        if insert(tree, path, Leaf(chosen)):
            out.append((chosen.section, None, chosen.header, tuple(chosen.dotted), chosen.value, chosen))
    return out, gaps


def set_whole_block(ctx, f: Fact):
    """The leaves of the block S6.1 sets whole that `f`'s key is a leaf of (an `Option<B>` of a struct `B`), or None."""
    if f.kind not in ("token", "widget") or len(f.ckey) < 2:
        return None
    parent = ".".join(f.key.split(".")[:-1])
    info = check_facts.key_info(ctx, parent)
    if info[0] is not None:   # a key the core does not type: no block
        return None
    typ = info[4]
    m = re.fullmatch(r"Option<\s*(\w+)\s*>", typ or "")
    fields = ctx.structs.get(m.group(1)) if m else None
    return tuple(fields) if isinstance(fields, dict) else None


def whole_block(ctx, f: Fact) -> bool:
    """Whether `f`'s value is a table at a `Value` struct block, the whole block (S17), which a rule's `set` states as
    `{ replace = { … } }`, a bare table there being a `Fields` patch, and the preset's own bases as the table (S2.2,
    S11, ADD-A125). A `Value<Paint>` is no struct block (S2.2), an enum block is spelt by its variant."""
    m = re.fullmatch(r"Value<\s*(\w+)\s*>", (f.typ or "").strip())
    return bool(m) and m.group(1) in ctx.structs and m.group(1) not in NO_STRUCT_BLOCK and isinstance(f.value, dict)


def split_blocks(ctx, bases, rules, block_of=None) -> list:
    """The facts whose set-whole block (an `Option<B>` struct, which S6.1 sets whole and S11 states as one table) a
    base table or a rule's `set` states with a leaf missing: the leaves stated per leaf in the facts (S17) join into one
    table only where they fall in one base or one rule, so such a fact is left out and reported, never written with
    its block incomplete."""
    block_of = block_of or set_whole_block
    groups = {}
    for b in bases:
        f = b[5]
        leaves = block_of(ctx, f)
        if leaves:
            groups.setdefault(("base", b[0], b[1], b[2], tuple(b[3][:-1])), (leaves, []))[1].append(f)
    for i, r in enumerate(rules):
        for f in r.facts:
            leaves = block_of(ctx, f)
            if leaves:
                groups.setdefault(("rule", i, f.rel[:-1]), (leaves, []))[1].append(f)
    out = []
    for leaves, fs in groups.values():
        stated = {f.ckey[-1] for f in fs}
        if not set(leaves) <= stated:
            out += fs
    return out


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
        for s in SCHEMES:
            p = cells[(s, cond)]
            if p is None or p.status or p is cells[(s, None)]:
                continue
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
    later rule states a key related to its own, or the rule's `set` cannot take it; else a new rule. A `read` fact
    with a hand-written rule (its `hand`) has that rule alone, which no fact joins."""
    rules, last_of = [], {}
    last_key = {}       # a key: the last rule stating it
    last_below = {}     # a key: the last rule stating it or a key below it
    for f in facts:
        if f.hand:
            rules.append(Rule(f.rule_when, f.target, hand=f.hand))
            last_key[f.ckey] = len(rules) - 1
            for n in range(1, len(f.ckey) + 1):
                last_below[f.ckey[:n]] = len(rules) - 1
            continue
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


def declarations(items, platform, preset, units, facts, content):
    """([declare] tables, `[declare] conditions`, the state models the round trip reduces by, the entries left out,
    the variants left out per widget, the pending default sizes' listed paths) from the platform's state models (S10,
    S17 *State models*)."""
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
    declare, models, left, settings, dropped, listed = [], {}, [], [], [], []
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
            off = off_variants(widget, d, facts, widget_content(content, widget))
            if off:
                kvs = [(k, without_variants(k, v, off)) for k, v in kvs]
                dropped.append((widget, sorted(off)))
            kvs = held_contexts(kvs, units)
            kvs, waiting = pending_defaults(header, kvs, it)
            listed += waiting
            data, read_by = element_data(widget, facts) if flavor is None else ({}, [])
            if data:
                kvs.append(("data", data))
            declare.append((header, kvs, it, read_by))
            models[(widget, flavor)] = d
            axes = d.get("axes") if isinstance(d.get("axes"), dict) else {}
            settings += [c for c in axes.get("conditions", []) if isinstance(c, str)]
    for f in facts:
        for k in ("conditions", "not_conditions"):
            settings += [c for c in f.when.get(k, []) if isinstance(c, str)]
    conditions = list(dict.fromkeys("@" + c[len("system_setting:"):] for c in settings
                                    if c.startswith("system_setting:")))
    return declare, conditions, models, left, dropped, listed


def element_data(widget: str, facts) -> tuple:
    """The `data` of `widget`'s declaration (S8, S10): per part (`values` for the widget's values), the element data
    the expressions of its transcribed facts read (DATA_OPERAND), each once in the order first read, and those facts,
    whose citations the line carries. A flavour's section states none: the widget's own is in force under each."""
    data, read_by = {}, []
    for f in facts:
        if f.status not in ("base", "rule") or not f.key.startswith(widget + "."):
            continue
        owner = f.key[len(widget) + 1:].split(".", 1)[0]
        names = [name for text in expression_texts(f.value) for name in data_names(text)]
        if names:
            held = data.setdefault(owner, [])
            held += [n for n in dict.fromkeys(names) if n not in held]
            read_by.append(f)
    return data, read_by


def expression_texts(value) -> list:
    """Every expression (`"= …"`) a fact's value holds, at any depth."""
    if isinstance(value, str):
        return [value] if value.startswith("=") else []
    if isinstance(value, dict):
        return [t for v in value.values() for t in expression_texts(v)]
    if isinstance(value, list):
        return [t for v in value for t in expression_texts(v)]
    return []


def data_names(text: str) -> list:
    """The `data` names an expression reads (DATA_OPERAND), as S10 spells them."""
    out = []
    for m in DATA_OPERAND.finditer(text):
        element, body, part, metric = m.groups()
        if element and element not in ROOT_INPUTS:
            out.append(element)
        elif body:
            out.append(body)
        elif part and metric in FONT_METRICS:
            out.append(f"widget.{part}.{metric}")
    return out


@dataclass
class DeclaredAt:
    """A state model's place, as a fact's `where` names its own: the citation of a listed declaration."""
    where: str


def held_contexts(kvs: list, units) -> list:
    """A declaration's `sizes_by_context` without its entries whose `context` names a widget the model does not hold
    (its `widget` or its `nearest` scope's, entry_widgets()): S12 refuses a declaration naming an undeclared widget,
    and no element has an ancestor the model does not hold, so the entry leaves no element's sizes out (S10); the
    entry comes back when its unit is enabled and the preset is regenerated, as a fact's `when` naming it does."""
    out = []
    for k, v in kvs:
        if k == "sizes_by_context" and isinstance(v, list):
            v = [e for e in v if not (isinstance(e, dict) and any(
                not held(units, w, "widget") for w in entry_widgets(e.get("context"))))]
        out.append((k, v))
    return out


def pending_defaults(header: tuple, kvs: list, it) -> tuple:
    """A declaration's pending default sizes taken out (S10's *A pending default*, S17, ADD-A131): the state model's
    `default_size = { pending = "U<n>" }`, the widget's, a `sizes_by_variant` entry's or a `sizes_by_context` entry's,
    is stated nowhere in the draft and listed in `[pending]` by its declaration's path, as S11 spells it:
    `declare.<header>.default_size`, `declare.<header>.sizes_by_variant.<variant>.default_size` (`@` included),
    `declare.<header>.sizes_by_context.<n>.default_size` (`<n>` its position from 0), the header a flavour's
    `flavor.<f>.widget.<w>` where the entry is one. ([(key, value)] without them, [(path, id, citation)])."""
    def pending(value):
        return value.get("pending") if isinstance(value, dict) and isinstance(value.get("pending"), str) else None

    prefix = ".".join(("declare",) + header)
    cite = DeclaredAt(f"{it.file}:{it.line}")
    out, listed = [], []
    for k, v in kvs:
        if k == "default_size" and pending(v):
            listed.append((f"{prefix}.default_size", pending(v), cite))
            continue
        if k in ("sizes_by_variant", "sizes_by_context") and isinstance(v, list):
            entries = []
            for i, e in enumerate(v):
                if isinstance(e, dict) and pending(e.get("default_size")):
                    name = e.get("variant") if k == "sizes_by_variant" else str(i)
                    listed.append((f"{prefix}.{k}.{name}.default_size", pending(e["default_size"]), cite))
                    e = {x: y for x, y in e.items() if x != "default_size"}
                entries.append(e)
            v = entries
        out.append((k, v))
    return out, listed


def widget_content(content: dict, widget: str) -> list:
    """The content variants of `widget` and of its parts (spec_catalogue.content_variants keys an *(element)* part's
    pairs `<widget>.<part>`), as the gate's content_variants gives them."""
    out = list(content.get(widget, []))
    for target, names in content.items():
        if target.startswith(widget + "."):
            out += names
    return out


VARIANT_LISTS = ("variants", "not_variants")
EXEMPT_VARIANTS = ("requires_variants", "sizes_by_variant")


def named_variants(when, widget) -> set:
    """The variants of `widget` a fact's `when` names: its own lists (a fact of the widget's keys), and a context's or
    position sibling's naming the widget."""
    out = set()
    for k in VARIANT_LISTS:
        out |= {v for v in when.get(k, []) if isinstance(v, str)} if isinstance(when.get(k), list) else set()
    return out


def context_variants(when, widget) -> set:
    out = set()
    for k in ("context", "not_context"):
        for e in when.get(k, []) if isinstance(when.get(k), list) else []:
            if isinstance(e, dict) and e.get("widget") == widget:
                for vk in VARIANT_LISTS:
                    out |= {v for v in e.get(vk, []) if isinstance(v, str)}
    pos = when.get("position") if isinstance(when.get("position"), dict) else {}
    for k in ("previous", "next"):
        e = pos.get(k)
        if isinstance(e, dict) and e.get("widget") == widget:
            for vk in VARIANT_LISTS:
                out |= {v for v in e.get(vk, []) if isinstance(v, str)}
    return out


def off_variants(widget, model, facts, content) -> set:
    """The declared variants of `widget` that facts name and every one of them is left out as a unit it belongs to is
    not enabled (its key's, or a widget its `when` names): the facts gate leaves such a variant out of the state model
    it checks the declarations against (S17, ADD-A128), so the draft declares it neither; it comes back with the unit,
    as its rules do. A variant no fact names is declared. A variant S12 exempts stays: one `requires_variants` or
    `sizes_by_variant` names, and a content variant the connector sets (the catalogue's pairs, S8)."""
    declared = {v for v in model.get("variants", []) if isinstance(v, str)}
    exempt = set(content)
    exempt |= {v for v in model.get("requires_variants", []) if isinstance(v, str)}
    for k in EXEMPT_VARIANTS[1:]:
        exempt |= {e.get("variant") for e in model.get(k, []) if isinstance(e, dict)}
    named_in, named_off = set(), set()
    for f in facts:
        if f.reason == ACCENT_RUN:   # the accent run's facts name no variant the gate's projection reads
            continue
        names = context_variants(f.when, widget)
        if f.kind == "widget" and f.widget == widget:
            names |= named_variants(f.when, widget)
        if f.status == LEFT_OUT and (f.reason == NOT_HELD or f.reason.startswith(UNIT_NOT_ENABLED)):
            named_off |= names
        else:
            named_in |= names
    return (declared & named_off) - named_in - exempt


def without_variants(key, value, off):
    """A declaration with the variants `off` taken out: from `variants`, and from each `exclusive_variants` group (a
    group left with fewer than two goes, as it excludes nothing)."""
    if key == "variants" and isinstance(value, list):
        return [v for v in value if v not in off]
    if key == "exclusive_variants" and isinstance(value, list):
        groups = [[v for v in g if v not in off] for g in value if isinstance(g, list)]
        return [g for g in groups if len(g) >= 2]
    return value


# ---------------------------------------------------------------------------------------------- the draft

def dotted_key(keys) -> str:
    return ".".join(toml_key(k) for k in keys)


def render(draft: Draft) -> str:
    """The preset's generated text, each hand-written part an empty block (hand_parts()), its digest stated."""
    out = [f"# {draft.preset} (S11): generated by scripts/generate_preset.py from docs/facts/{draft.platform}/, each "
           f"value's comment naming its fact.",
           "# The lines between `# hand-written: begin <part>` and `# hand-written: end <part>` state what no fact "
           "states,",
           "# written by hand (S11, S17: `[preset] generation`, the `[declare]` keyed namespaces, the `read` facts as "
           "bases);",
           "# `generate_preset.py <id> --write` regenerates every other line and carries them over.",
           "", "[preset]", f"id = {toml_inline(draft.preset)}", *block("preset"), "", "[declare]"]
    if draft.conditions:
        out.append(f"conditions = {toml_inline(draft.conditions)}")
    out += [*block("declare"), ""]
    for header, kvs, it, read_by in draft.declare:
        out += [f"# {it.file}:{it.line}", f"[{dotted_key(('declare',) + header)}]"]
        for k, v in kvs:
            if k == "data":
                out.append("# " + ", ".join(f.where for f in read_by))
            out.append(f"{toml_key(k)} = {toml_inline(v)}")
        out.append("")
    if draft.pending:
        out.append("[pending]")
        for path, uid, f in draft.pending:
            out += [f"# {f.where}", f"{toml_key(path)} = {toml_inline(uid)}"]
        out.append("")
    tables = {}
    for section, scheme, header, dotted, value, f in draft.bases:
        tables.setdefault((section, scheme, header), []).append((dotted, value, f.where))
    for section, header, dotted, value, defined in draft.definitions:
        tables.setdefault((section, None, header), []).append((dotted, value, defined))
    for t in draft.read_tables:
        tables.setdefault(t, [])
    order = sorted(tables, key=lambda t: (SECTIONS.index(t[0]), SCHEMES.index(t[1]) if t[1] else -1))
    for t in order:
        section, scheme, header = t
        path = dotted_key((section,) + ((scheme,) if scheme else ()) + header)
        out.append(f"[{path}]")
        if t in draft.read_tables:
            out += block(f"bases {path}")
        for dotted, value, comment in tables[t]:
            out += [f"# {comment}", f"{dotted_key(dotted)} = {toml_inline(value)}"]
        out.append("")
    for r in draft.rules:
        out += ["[[rules]]", f"when = {toml_inline(r.when)}", f"target = {toml_inline(r.target)}", "", "[rules.set]"]
        if r.hand:
            out += block(r.hand)
        for f in r.facts:
            value = {"replace": f.value} if f.whole else f.value
            out += [f"# {f.where}", f"{dotted_key(f.rel)} = {toml_inline(value)}"]
        out.append("")
    text = "\n".join(out).rstrip("\n") + "\n"
    _, generated, _ = hand_parts(text)
    lines = text.split("\n")
    return "\n".join(lines[:4] + [DIGEST + digest(generated)] + lines[4:])


# ------------------------------------------------------------------------------------- the hand-written parts

HAND = "# hand-written: "
BEGIN, END = "begin ", "end "
DIGEST = "# generated lines: sha256 "


def block(part: str) -> list:
    """An empty hand-written block for `part`: its two marker lines."""
    return [f"{HAND}{BEGIN}{part}", f"{HAND}{END}{part}"]


def digest(generated: list) -> str:
    return hashlib.sha256("\n".join(generated).encode("utf-8")).hexdigest()


def hand_parts(text: str) -> tuple:
    """A preset's text split (the docstring's *The hand-written parts*): ({part: its block's lines}, the generated
    lines, every line outside the blocks but the digest's, markers included, the stated digest or None).
    InputError on a malformed block: a marker that is neither, an `end` naming another part or none open, a `begin`
    inside a block or never ended, a part twice, a digest line twice."""
    blocks, generated, stated, part = {}, [], None, None
    for n, line in enumerate(text.split("\n"), 1):
        if line.startswith(HAND):
            rest = line[len(HAND):]
            if rest.startswith(BEGIN) and part is None and rest[len(BEGIN):]:
                part = rest[len(BEGIN):]
                if part in blocks:
                    raise InputError(f"line {n}: the hand-written part `{part}` twice")
                blocks[part] = []
            elif rest.startswith(END) and part is not None and rest[len(END):] == part:
                part = None
            else:
                raise InputError(f"line {n}: `{line}` opens or ends no hand-written part here"
                                 + (f" (`{part}` is open)" if part is not None else ""))
            generated.append(line)
        elif part is not None:
            blocks[part].append(line)
        elif line.startswith(DIGEST):
            if stated is not None:
                raise InputError(f"line {n}: a second `{DIGEST.strip()}` line")
            stated = line[len(DIGEST):]
        else:
            generated.append(line)
    if part is not None:
        raise InputError(f"the hand-written part `{part}` is never ended")
    return blocks, generated, stated


def carried(text: str, committed, refresh=None) -> str:
    """The generated `text` with the committed preset's hand-written blocks carried into it (`--write`), each block
    as `refresh` gives it where given (Citations.refresh(): its citations made true); the text itself without one.
    InputError where a hand-written line would be lost or doubled: the committed file's generated lines are not
    those its digest states (a line edited, added or removed outside the blocks), a non-empty block's part the new
    text has no place for (a `read` fact gone), or the result fails verify() against the committed file with the
    refreshed blocks."""
    if committed is None:
        return text
    blocks, generated, stated = hand_parts(committed)
    if refresh is not None:
        blocks = refresh(blocks)
        committed = with_blocks(committed, blocks)
    if stated is None:
        raise InputError(f"no `{DIGEST.strip()}` line: the preset was not written by --write")
    if digest(generated) != stated:
        raise InputError("a line outside the hand-written blocks is not what the generator wrote (its digest "
                         "differs): --write would lose it; move it into its part's block, or restore the line")
    places = hand_parts(text)[0]
    lost = [p for p, lines in blocks.items() if p not in places and any(x.strip() for x in lines)]
    if lost:
        raise InputError(f"the hand-written parts {lost} have no place in the generated text (their facts gone?): "
                         f"move or remove their lines by hand")
    out = []
    for line in text.split("\n"):
        out.append(line)
        if line.startswith(HAND + BEGIN):
            out += blocks.get(line[len(HAND + BEGIN):], [])
    result = "\n".join(out)
    problems = verify(committed, result)
    if problems:
        raise InputError("; ".join(problems))
    return result


def verify(committed: str, result: str) -> list:
    """What carried() must hold of its result, each a problem: every committed block's lines in the result's block
    of its part, the same lines in order, once (none dropped, changed or doubled), no line in a block the committed
    file does not have, the result's generated lines as its digest states, and the result valid TOML (a hand-written
    key the generator also writes is a duplicate key)."""
    try:
        before = hand_parts(committed)[0]
        after, generated, stated = hand_parts(result)
    except InputError as err:
        return [str(err)]
    problems = [f"the hand-written part `{p}` is not carried exactly: {len(lines)} lines committed, "
                f"{len(after.get(p, []))} written" for p, lines in before.items()
                if any(x.strip() for x in lines) and after.get(p) != lines]
    problems += [f"the hand-written part `{p}` holds lines the committed preset's does not"
                 for p, lines in after.items() if lines and before.get(p) != lines]
    if stated != digest(generated):
        problems.append("the generated lines are not those the digest states")
    try:
        tomllib.loads(result)
    except tomllib.TOMLDecodeError as err:
        problems.append(f"not valid TOML: {err}")
    return problems


def with_blocks(text: str, blocks: dict) -> str:
    """`text` with the lines of each hand-written part `blocks` holds replaced by its lines there."""
    out, skipping = [], False
    for line in text.split("\n"):
        if line.startswith(HAND + END):
            skipping = False
        elif skipping:
            continue
        out.append(line)
        if line.startswith(HAND + BEGIN) and line[len(HAND + BEGIN):] in blocks:
            out += blocks[line[len(HAND + BEGIN):]]
            skipping = True
    return "\n".join(out)


# ---------------------------------------------------------------------------------- the hand-written citations

ENTRY_CITE = re.compile(r'^(?P<head>\s+"(?P<name>[^"]+)",\s+# )(?P<file>docs/facts/[\w./-]+\.toml):(?P<line>\d+)$')
ABOVE_CITE = re.compile(r"^# (?P<file>docs/facts/[\w./-]+\.toml):(?P<line>\d+)$")
SOURCES = "docs/facts/sources.toml"
SOURCE_CITE = re.compile(rf"^# (?P<file>{re.escape(SOURCES)}):(?P<line>\d+)(?P<rest>.*)$")
SOURCE_ID = re.compile(r"^ \((?P<id>[^,)\s]+)[,)]")
LIST_OPEN = re.compile(r"^(\w+) = \[$")
PARAMS_TABLE = re.compile(r"^\[declare\.params\.(\w+)\]$")
# how a fact names an entry of a `[declare]` keyed namespace (S10), by the model field the namespace keys:
# ("key", segments) its key starts with the segments, `{}` the entry's own (one or more, `inactive.view.…`) and `@{}`
# its keyed segment; ("value", field) its key's last segment is the field (S10: `symbol_scales`, IconStyle.scale) and
# its value the entry; ("map", field) likewise, the entry a key of its value's table; ("flavor", None) its `when`
# names `flavor:<entry>` in its conditions, and the default flavour (the list's first, S10) is also named by a state
# model naming none. A `[declare.params.<group>]` table's `keys`: ("key", ("token", "params", <group>, "{}")).
NAMED_BY = {
    "color_roles": (("key", ("role", "@{}")),),
    "surface_sets": (("key", ("role", "{}")),),
    "accent_ramp": (("key", ("role", "global", "accent_ramp", "{}")),),
    "inputs": (("key", ("input", "{}")),),
    "text_styles": (("key", ("token", "text", "keyed", "{}")),),
    "elevation": (("key", ("token", "elevation", "{}")),),
    "timings": (("key", ("token", "motion", "timings", "{}")),),
    "durations": (("key", ("token", "motion", "durations", "{}")),),
    "icon_contexts": (("key", ("token", "icons", "sizes", "keyed", "{}")), ("key", ("token", "icons", "sizes", "{}"))),
    "symbol_scales": (("value", "scale"),),
    "icon_state_effects": (("value", "state_effect"),),
    "symbolic_colors": (("map", "symbolic_colors"),),
    "rendering_engines": (("value", "engine"),),
    "lcd_filters": (("value", "lcd_filter"),),
    "spec_versions": (("value", "spec_version"),),
    "flavors": (("flavor", None),),
}


def naming_index(items: list, rule) -> dict:
    """{an entry: the items naming it by `rule` (NAMED_BY), in their order}; the default flavour's state models under
    None."""
    kind, at = rule
    out = {}
    for it in items:
        d, names = it.data, []
        if kind == "flavor":
            when = d.get("when") if isinstance(d.get("when"), dict) else {}
            conds = when.get("conditions", []) if isinstance(when.get("conditions"), list) else []
            names = [c[7:] for c in conds if isinstance(c, str) and c.startswith("flavor:")]
            if not names and it.table == "model":
                names = [None]
        elif it.table == "fact" and isinstance(d.get("key"), str):
            parsed = check_facts.key_segments(d["key"])
            segs = parsed[0] if isinstance(parsed, tuple) else []
            if kind == "key":
                i = len(at) - 1
                if len(segs) > i and tuple(segs[:i]) == at[:i]:
                    if at[i] == "@{}":
                        names = [segs[i][1:]] if segs[i].startswith("@") else []
                    else:
                        names = [".".join(segs[i:j]) for j in range(i + 1, len(segs) + 1)]
            elif segs and segs[-1] == at:
                v = d.get("value")
                values = [v] if kind == "value" else list(v) if isinstance(v, dict) else []
                names = [x[1:] if x.startswith("@") else x for x in values if isinstance(x, str)]
        for name in dict.fromkeys(names):
            out.setdefault(name, []).append(it)
    return out


def read_places(facts) -> dict:
    """{(the hand-written part, the dotted key its line states): the `read` facts written there}, as read_tables()
    places them: a base table's (each scheme's of a colour role at rest) or the fact's own rule's."""
    out = {}
    for f in facts:
        if f.reason != READ:
            continue
        if f.hand:
            places = [(f.hand, dotted_key(f.rel))]
        else:
            g = Fact(f.item, f.key, f.ckey, f.when, f.kind, f.widget, f.typ, f.dim)
            try:
                place(g)
            except Untranscribable:
                continue
            schemes = slots_of(f.when) if f.kind == "role" else (None,)
            places = [(f"bases {dotted_key((g.section,) + ((s,) if s else ()) + g.header)}", dotted_key(g.dotted))
                      for s in schemes]
        for p in places:
            out.setdefault(p, []).append(f.item)
    return out


def source_lines(text: str) -> dict:
    """{a source id: the line of its `[[source]]` header} of sources.toml's text, as a fact is cited by its
    `[[fact]]` header's."""
    out, header = {}, None
    for n, line in enumerate(text.split("\n"), 1):
        if line.startswith("["):
            header = n if line.strip() == "[[source]]" else None
        m = re.match(r'^id = "([^"]+)"$', line)
        if m and header is not None:
            out.setdefault(m.group(1), header)
    return out


class Citations:
    """The hand-written blocks' citation comments made true (the docstring's *The hand-written citations*):
    refresh() gives each part's lines with every stale citation whose fact it finds pointed at that fact;
    `refreshed` lists those, `findings` those whose fact it cannot find."""

    def __init__(self, draft: Draft, sources_text: str):
        self.preset = draft.preset
        self.own = [it for it in draft.items if it.platform == draft.platform and holds_for(it, draft.preset)]
        self.position = {id(it): n for n, it in enumerate(self.own)}
        self.reads = read_places(draft.facts)
        self.sources = source_lines(sources_text)
        self.index, self.refreshed, self.findings = {}, [], []

    def refresh(self, blocks: dict) -> dict:
        return {part: self.refreshed_part(part, lines) for part, lines in blocks.items()}

    def named(self, ns: str, name: str, first: bool):
        """The preset's items naming the entry `name` of the namespace `ns`, in their order (`first`: the list's
        first entry), or None where no rule (NAMED_BY) names the namespace's entries."""
        group = re.fullmatch(r"params\.(\w+)", ns)
        rules = (("key", ("token", "params", group.group(1), "{}")),) if group else NAMED_BY.get(ns)
        if rules is None:
            return None
        found = []
        for rule in rules:
            if rule not in self.index:
                self.index[rule] = naming_index(self.own, rule)
            found += self.index[rule].get(name, []) + (self.index[rule].get(None, []) if first else [])
        return sorted({id(it): it for it in found}.values(), key=lambda it: self.position[id(it)])

    def judged(self, part: str, subject: str, cited: tuple, found: list) -> tuple:
        """The citation `cited` of a subject the items `found` name, kept where it is the header of one of them,
        else the first of them in its file, else the first; None where none does (a finding)."""
        if any(cited == (it.file, it.line) for it in found):
            return cited
        if not found:
            self.findings.append(f"{self.preset}: `{part}` {subject}: {cited[0]}:{cited[1]}: no fact names it")
            return None
        pick = next((it for it in found if it.file == cited[0]), found[0])
        self.refreshed.append(f"{self.preset}: `{part}` {subject}: {cited[0]}:{cited[1]} names no fact of it, "
                              f"{pick.file}:{pick.line} does")
        return pick.file, pick.line

    def refreshed_part(self, part: str, lines: list) -> list:
        """A hand-written part's lines, each citation of a subject judged()."""
        out, ns, group, count = list(lines), None, None, 0
        for i, line in enumerate(lines):
            if part == "declare":
                opened, params = LIST_OPEN.match(line), PARAMS_TABLE.match(line)
                if params:
                    group = params.group(1)
                elif line.startswith("["):
                    group = None
                if opened:
                    ns = f"params.{group}" if opened.group(1) == "keys" and group else opened.group(1)
                    count = 0
                elif line == "]":
                    ns = None
                m = ENTRY_CITE.match(line)
                if m and ns is not None:
                    count += 1
                    cited = (m.group("file"), int(m.group("line")))
                    found = self.named(ns, m.group("name"), count == 1)
                    subject = f"{ns} {m.group('name')!r}"
                    if found is None:
                        self.findings.append(f"{self.preset}: `{part}` {subject}: no rule finds the fact naming an "
                                             f"entry of `{ns}` (NAMED_BY)")
                        continue
                    got = self.judged(part, subject, cited, found)
                    if got is not None:
                        out[i] = f"{m.group('head')}{got[0]}:{got[1]}"
            elif part == "preset":
                m = SOURCE_CITE.match(line)
                if not m:
                    continue
                named = SOURCE_ID.match(m.group("rest"))
                cited, sid = (m.group("file"), int(m.group("line"))), named.group("id") if named else None
                if sid is None or sid not in self.sources:
                    self.findings.append(f"{self.preset}: `{part}` {line[2:]}: names no source id of {SOURCES} "
                                         f"(`# {SOURCES}:<line> (<id>)`)")
                    continue
                if cited[1] != self.sources[sid]:
                    self.refreshed.append(f"{self.preset}: `{part}` {sid}: {SOURCES}:{cited[1]} is not its entry, "
                                          f"{SOURCES}:{self.sources[sid]} is")
                    out[i] = f"# {SOURCES}:{self.sources[sid]}{m.group('rest')}"
            elif part.startswith(("bases ", "rule ")):
                m = ABOVE_CITE.match(line)
                if not m:
                    continue
                key = lines[i + 1].split("=", 1)[0].strip() if i + 1 < len(lines) and "=" in lines[i + 1] else ""
                got = self.judged(part, repr(key), (m.group("file"), int(m.group("line"))),
                                  self.reads.get((part, key), []))
                if got is not None:
                    out[i] = f"# {got[0]}:{got[1]}"
        return out


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


def holds(when: dict, c: dict, groups: list, recency: dict, states=None) -> bool:
    """Whether a `when` holds in a combination (S7, S11): every required name present, no refused one, after the
    exclusive drop (`states`, the combination's flags after it, where the caller has them); the contexts, position,
    direction, size classes and recency answers as S7 matches them."""
    states = reduced(c["states"], groups) if states is None else states
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


def element_size(when: dict, model, ancestors=()):
    """The size an element of the `when`'s minimal combination holds and the variants it holds (S10, ADD-A131), or
    (None, variants) where no element holds the `when`'s sizes. The size the `when` names; else, of the sizes the
    element takes (its first `sizes_by_variant` entry's whose variant it holds, else the widget's `sizes`, less those
    the first `sizes_by_context` entry whose selector its ancestors match leaves out), the default in force (that
    context entry's `default_size`, else the variant entry's, else the widget's) where it is one of them and the
    `when` does not refuse it, else the first the `when` does not refuse: an element stating that size, as one whose
    default is pending states one (S10's *A pending default*: an element stating none is not available); none where
    the element takes none. Where an element holding only the `when`'s variants takes no size the `when` allows (a
    `not_sizes` refusing the widget's every size, or a size the `when` names that it does not take), the first
    `sizes_by_variant` entry whose variant the `when` does not refuse, no `exclusive_variants` group shares with the
    `when`'s, and no `not_drawn` entry of variants alone excludes with them, and whose sizes it allows (or which takes
    the named sizes), adds its variant: the sizes that variant alone takes."""
    named = list(when.get("sizes", []))
    variants = list(when.get("variants", []))
    if not model:
        return named, variants
    refused = set(when.get("not_sizes", []))
    entries = [e for e in model.get("sizes_by_variant", []) if isinstance(e, dict) and isinstance(e.get("sizes"), list)]
    by_context = next((e for e in model.get("sizes_by_context", []) if isinstance(e, dict)
                       and isinstance(e.get("context"), dict) and context_holds(e["context"], list(ancestors))), None)

    def taken(entry):
        sizes = entry["sizes"] if entry else [z for z in model.get("sizes", []) if isinstance(z, str)]
        if by_context is not None:
            sizes = [z for z in sizes if z in by_context.get("sizes", [])]
        return sizes

    def size_of(entry):
        sizes = taken(entry)
        if named:
            return named if all(z in sizes for z in named) else None
        if not sizes:
            return []
        default = next((x["default_size"] for x in (by_context, entry) if x is not None and "default_size" in x),
                       model.get("default_size"))
        if isinstance(default, str) and default in sizes and default not in refused:
            return [default]
        allowed = [z for z in sizes if z not in refused]
        return allowed[:1] or None

    entry = next((e for e in entries if e.get("variant") in variants), None)
    got = size_of(entry)
    if got is not None or entry is not None:
        return got, variants
    not_variants = set(when.get("not_variants", []))
    groups = [set(g) for g in model.get("exclusive_variants", []) if isinstance(g, list)]
    undrawn = [x for x in model.get("not_drawn", []) if isinstance(x, dict) and x
               and set(x) <= {"variants", "not_variants"}]
    for e in entries:
        v = e.get("variant")
        if v in not_variants or any(v in g and g & set(variants) for g in groups):
            continue
        if any(set_match(x, "variants", set(variants) | {v}) for x in undrawn):
            continue
        got = size_of(e)
        if got:
            return got, variants + [v]
    return None, variants


def minimal(when: dict, scheme: str, groups: list, recency: dict, model=None):
    """The `when`'s minimal combination under a scheme (its required names and nothing else, and the size an element
    holds and the variant that size needs, element_size()), or None where no element holds it."""
    conds = list(when.get("conditions", []))
    if any(x.startswith("scheme:") and x != f"scheme:{scheme}" for x in conds):
        return None
    latest = when.get("latest", {})
    ancestors = [{"widget": e.get("widget"), "part": e.get("part"), "surface": e.get("surface"),
                  "variants": set(e.get("variants", [])), "states": set(e.get("states", [])),
                  "sizes": set(e.get("sizes", []))} for e in when.get("context", [])]
    sizes, variants = element_size(when, model, ancestors)
    if sizes is None:
        return None
    d = {"conditions": conds + [f"scheme:{scheme}"], "states": list(when.get("states", [])) + list(latest.values()),
         "variants": variants, "sizes": sizes,
         "densities": when.get("densities", []),
         "direction": when.get("direction", "ltr"), "position": when.get("position"), "latest": latest,
         "size_class": when.get("size_class", {}),
         "ancestors": ancestors}
    if len(set(d["sizes"])) > 1 or len(set(d["densities"])) > 1:
        return None
    if any(len(set(sc)) > 1 for sc in d["size_class"].values() if isinstance(sc, list)):
        return None
    c = combo(d)
    return c if holds(when, c, groups, recency) else None


def when_key(when) -> str:
    """A `when` as an exact key: equal keys for equal `when`s (its tables' keys sorted, as holds() reads a table by
    its keys; its lists in their order)."""
    return json.dumps(when, sort_keys=True)


class Combinations:
    """The round trip's combinations, each computed once: a `when`'s minimal combination under a scheme for a widget
    (minimal(), reduced by the state model model_of() gives), and whether a `when` holds in it (holds()), by what
    each is computed from: the widget, the `when`s by when_key(), the scheme. A `when`'s key is found once per
    object, which is kept, so its id is never another's."""

    def __init__(self, models: dict):
        self.models, self.keys, self.reductions, self.combinations, self.held = models, {}, {}, {}, {}

    def key(self, when) -> str:
        found = self.keys.get(id(when))
        if found is None:
            found = self.keys[id(when)] = (when, when_key(when))
        return found[1]

    def reduction(self, widget, when) -> tuple:
        """(the model, its exclusive groups, its recency sets) a combination of `when` is reduced by."""
        k = (widget, self.key(when))
        if k not in self.reductions:
            model = model_of(self.models, widget, when)
            groups = [g for g in model.get("exclusive", []) if isinstance(g, list)] if model else []
            self.reductions[k] = model, groups, check_facts.recency_of(model) if model else {}
        return self.reductions[k][1:]

    def combination(self, widget, when, scheme):
        """minimal() of `when` under the scheme, reduced as reduction() gives."""
        k = (widget, self.key(when), scheme)
        if k not in self.combinations:
            self.reduction(widget, when)
            model, groups, recency = self.reductions[k[:2]]
            c = minimal(when, scheme, groups, recency, model)
            self.combinations[k] = c, (reduced(c["states"], groups) if c is not None else None)
        return self.combinations[k][0]

    def holding(self, widget, when, scheme):
        """holds() of a `when` in combination() of `when` under the scheme (not None), as a function of it."""
        k = (widget, self.key(when), scheme)
        groups, recency = self.reduction(widget, when)
        c, states = self.combinations[k]
        held, keys = self.held.setdefault(k, {}), self.keys

        def holding(other) -> bool:
            found = keys.get(id(other))
            other_key = found[1] if found is not None else self.key(other)
            if other_key not in held:
                held[other_key] = holds(other, c, groups, recency, states)
            return held[other_key]

        return holding

    def holds(self, widget, when, scheme, other) -> bool:
        """holds() of the `when` `other` in combination() of `when` under the scheme (not None)."""
        return self.holding(widget, when, scheme)(other)


def last_holding(fs: list, c: dict, groups, recency):
    states = reduced(c["states"], groups)
    return next((f for f in reversed(fs) if holds(f.when, c, groups, recency, states)), None)


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
    table = data.get("pending", {}) if isinstance(data.get("pending"), dict) else {}
    if table != {path: uid for path, uid, _ in draft.pending}:
        trip.mismatches.append(f"the [pending] table reads back otherwise: {table}")
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
    for section, header, dotted, value, defined in draft.definitions:
        path = (section,) + header + dotted
        expected_base.add(path)
        node, found = lookup(data, path)
        if not found or normal(node) != normal(value):
            trip.mismatches.append(f"{dotted_key(path)}: the value {defined} defines reads back otherwise")
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
            if f.whole:
                if not (isinstance(node, dict) and set(node) == {"replace"}):
                    trip.mismatches.append(f"{f.where}: {f.key}: rule {i + 1} states the whole block without "
                                           f"`replace`")
                    continue
                node = node["replace"]
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
    at = Combinations(draft.models)
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
        widget = fs[0].widget
        for f in fs:
            if slots_of(f.when) and all(at.combination(widget, f.when, s) is None for s in SCHEMES):
                trip.impossible += 1
        for when in whens[group_of(fs[0])]:
            for s in SCHEMES:
                c = at.combination(widget, when, s)
                if c is None:
                    continue
                groups, recency = at.reduction(widget, when)
                holding = at.holding(widget, when, s)
                applies = next((f for f in reversed(fs) if holding(f.when)), None)
                if applies is None:
                    if preset_value(got, fs[0].kind, c, s, groups, recency, fs[0].typ, holding) is not None:
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
                value = preset_value(got, fs[0].kind, c, s, groups, recency, fs[0].typ, holding)
                if normal(value) != normal(resolved_list(fact_value(applies), fs[0].typ)):
                    trip.mismatches.append(f"{fs[0].key} in {when} under scheme {s}: the draft gives {value!r}, "
                                           f"the fact that applies ({applies.where}) {applies.item.data['value']!r}")
    return trip


def models_holding(models: dict, widget, when: dict) -> list:
    """The state models the elements a fact's `when` holds for are reduced by: its flavour's where it names one
    (model_of()), else the widget's own and each flavour's `not_conditions` does not refuse (S10)."""
    conds = when.get("conditions", []) if isinstance(when.get("conditions"), list) else []
    if any(isinstance(c, str) and c.startswith("flavor:") for c in conds):
        model = model_of(models, widget, when)
        return [model] if model is not None else []
    nots = when.get("not_conditions", []) if isinstance(when.get("not_conditions"), list) else []
    refused = {c[7:] for c in nots if isinstance(c, str) and c.startswith("flavor:")}
    return [model for (held, flavor), model in models.items() if held == widget and flavor not in refused]


def model_of(models: dict, widget, when: dict):
    """The state model a combination of `when` is reduced by: its flavour's entry where it names a flavour the
    widget has one for (S10), else the widget's own; None for no widget."""
    if not widget:
        return None
    conds = when.get("conditions", []) if isinstance(when.get("conditions"), list) else []
    flavor = next((c[7:] for c in conds if isinstance(c, str) and c.startswith("flavor:")), None)
    return models.get((widget, flavor)) or models.get((widget, None))


def back_value(f: Fact, node):
    """A read-back TOML value as the fact writes it (unspell; a whole part's `absent = true`, `absent = false` and
    `drawn_as`)."""
    if f.kind == "widget" and len(f.ckey) == len(f.target.split(".")):
        if f.rel == ("absent",) and isinstance(node, bool):
            return "absent" if node else "present"
        return {f.rel[-1]: node}
    if f.param_unit and isinstance(node, (int, float)) and not isinstance(node, bool):
        return f"{node}{f.param_unit}"
    return unspell(f.rel[-1], node, f.typ)[1]


def preset_value(got, kind, c, scheme, groups, recency, typ=None, holding=None):
    """The value the read-back bases and rules give a key in a combination (S8: the bases, then the rules in order,
    the last matching winning; an input's per-contrast base where its contrast holds, S9); a keyed list (S6.1) the
    base merged with every matching rule's list in order, by its items' keys (keyed_merge()). `holding(when)`, where
    given, is holds() in the combination, `groups` and `recency` (Combinations.holds())."""
    if kind == "input":
        contrast = next((x for x, _ in CONTRASTS if x in c["conditions"]), None)
        return got["base"].get((scheme, contrast), got["base"].get((scheme, None)))
    if holding is None:
        states = reduced(c["states"], groups)

        def holding(when):
            return holds(when, c, groups, recency, states)
    base = got["base"].get(scheme if kind == "role" else None)
    matching = [v for when, v in got["rules"] if holding(when if isinstance(when, dict) else {})]
    item_key = KEYED_LISTS.get(check_facts.strip_type(typ)) if typ else None
    if item_key is None:
        return matching[-1] if matching else base
    merged = None
    for v in ([base] if base is not None else []) + matching:
        merged = keyed_merge(merged or [], v, item_key) if isinstance(v, list) else v
    return merged


def transition_key(item):
    """A transition's key in a list (S6.1, ADD-A124, ADD-A126): its property group, its `from` set (`None`, left out
    or `"none"`, any state; two sets of the same flags one key) and its Breeze test (none for another engine or a
    stated zero), as `Transition::merge_keyed` reads them; a transition is split per group first (split_groups())."""
    props = item.get("properties")
    origin = item.get("from")
    origin = frozenset(origin) if check_facts.str_list(origin) else None
    stated, duration = check_facts.unit_value(item, "duration")
    name, payload = check_facts.variant_of(item.get("engine"))
    test = None
    if name in ("breeze", "breeze_tab_bar") and isinstance(payload, dict) and not (
            stated and check_facts.stated_zero(duration)):
        test = tuple(frozenset(payload.get(k, [])) if check_facts.str_list(payload.get(k, [])) else normal(
            payload.get(k)) for k in ("any", "none", "conditions"))
    return "transition", normal(props), origin, test


def binding_key(item):
    """A binding's key in a list (S6.1, ADD-A122): every `Surface(_)` binding one key, a role binding its role."""
    name, payload = check_facts.variant_of(item)
    if name == "role" and isinstance(payload, dict):
        return "role", normal(payload.get("role"))
    return ("surface",) if name == "surface" else ("item", normal(item))


def presentation_key(item):
    """A presentation's key in a list (S6.1): its event."""
    return "presentation", normal(item.get("event") if isinstance(item, dict) else item)


# S6.1's keyed lists, by the core's type of the field, each with its items' key
KEYED_LISTS = {"Vec<Transition>": transition_key, "Vec<Binding>": binding_key,
               "Vec<PresentationTransition>": presentation_key}


def split_groups(items: list, item_key) -> list:
    """A list's items as the merge keys them: a transition listing several property groups split per group, each
    with the others' fields (S6.1; `split_by_group`)."""
    if item_key is not transition_key:
        return list(items)
    out = []
    for item in items:
        props = item.get("properties") if isinstance(item, dict) else None
        if isinstance(props, list) and len(props) > 1:
            out += [{**item, "properties": [p]} for p in props]
        else:
            out.append(item)
    return out


def keyed_merge(earlier: list, later: list, item_key) -> list:
    """S6.1's keyed-list merge: each later item replaces the earlier item of its key, the merged list holding the
    earlier items not replaced, in their order, then the later items, in theirs (`merge_keyed_by`)."""
    held = split_groups(earlier, item_key)
    for item in split_groups(later, item_key):
        k = item_key(item) if isinstance(item, dict) else ("item", normal(item))
        held = [x for x in held if (item_key(x) if isinstance(x, dict) else ("item", normal(x))) != k]
        held.append(item)
    return held


def resolved_list(value, typ):
    """A fact's value as the resolution gives it (S17: a keyed list is stated as the merged list): a keyed list
    merged into an empty one, its transitions split per group; any other value as it is."""
    item_key = KEYED_LISTS.get(check_facts.strip_type(typ)) if typ else None
    return keyed_merge([], value, item_key) if item_key is not None and isinstance(value, list) else value


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
               f"{nbase} base values, {len(draft.definitions)} values the specification defines "
               f"({', '.join(d[4] for d in draft.definitions) or 'none'}), {len(draft.declare)} [declare] tables")
    pend = {}
    for f in draft.facts:
        if f.status == PENDING:
            pend[f.item.data["pending"]] = pend.get(f.item.data["pending"], 0) + 1
    if pend:
        out.append("# pending (register id: facts): " + ", ".join(
            f"{k}: {v}" for k, v in sorted(pend.items(), key=lambda kv: int(kv[0][1:]))))
    if draft.pending:
        out.append(f"# [pending]: {len(draft.pending)} listed paths, pending with nothing beneath: " + ", ".join(
            f"{path} ({uid if isinstance(uid, str) else ', '.join(uid)})" for path, uid, _ in draft.pending))
    redrawn = redrawn_pending(draft.facts, draft.bases)
    if redrawn:
        out.append(f"# [pending]: {len(redrawn)} pending fields of a part the base states whole and a `present` fact "
                   f"draws again, which S17 gives no encoding, left without a value: "
                   + ", ".join(f"{key} ({uid}, {f.where})" for key, uid, f in redrawn))
    for w, it in draft.left_models:
        out.append(f"# [declare] left out: {w} ({it.file}:{it.line}), a widget the model does not hold")
    for w, vs in draft.dropped:
        out.append(f"# [declare] {w}: variants left out, only facts of units not enabled naming them: {', '.join(vs)}")
    out.append(f"# left for hand-writing: S19.2's staged bases for {draft.preset}: {draft.staged} "
               f"({STAGED_BASES}); the keyed namespaces of [declare] but `conditions`; `[preset] generation`; "
               f"the `read` facts as bases, in {len(draft.read_tables)} tables, and in "
               f"{sum(1 for r in draft.rules if r.hand)} rules of their own")
    for f in draft.read_rules:
        out.append(f"# left for hand-writing: {f.key} ({f.where}), a `read` fact whose key no rule states: its rule, "
                   f"which no hand-written part holds")
    for widget, key, why in draft.gaps:
        out.append(f"# facts gap: {key}: {why}")
    out.append(f"# round trip: {trip.compared} combinations compared, {len(trip.mismatches)} mismatches "
               f"({trip.pending} where a pending fact applies and {trip.left_out} where a left-out one does, skipped; "
               f"{trip.impossible} facts whose `when` no element holds: two sizes, a flag its exclusive group drops, "
               f"a name required and refused)")
    return out


# ---------------------------------------------------------------------------------------- the model's fields

MODEL_STRUCT = re.compile(r"pub struct (\w+)\s*(<[^{;(]*>)?\s*\{")
PART_FIELD = re.compile(r"#\[part\(((?:[^()]|\([^()]*\))*)\)\]\s*(\w+)\s*:\s*PartStyle\b")
PART_FIELDS = re.compile(r"\bfields\(([^()]*)\)")
VALUES_FIELD = re.compile(r"\bvalues\s*:\s*(\w+)")
GENERIC = re.compile(r"(\w+)<(.+)>")
WRAPPERS = re.compile(r"(?:Value|Option|Box)<(.+)>")
SURFACE_MAP = re.compile(r"SurfaceMap<(.+)>")
EVERY_PRESET_SETS = ("window", "selection")   # S3: "`Window` and `Selection` every preset declares"
NO_FACT = "NO FACT"
ONLY_CONDITIONAL = "ONLY CONDITIONAL"
BASE_EXISTS = "BASE EXISTS"
FIELD_GROUPS = {
    NO_FACT: "no fact or pending fact of the preset at, above or below the path",
    ONLY_CONDITIONAL: "facts there, none at rest: rules alone state it, and the reader needs a base",
    BASE_EXISTS: "a fact at rest there, which gives no base (left out, or pending and not listed)",
}


@dataclass
class Model:
    """The model's types as its Rust sources write them (read_model())."""
    structs: dict             # struct name: (its generic parameters, {field: type})
    widgets: dict             # typed widget: ({part: [its fields]}, its values struct's name)
    ambiguous: set = field(default_factory=set)   # struct names written twice, with other fields
    folder: Path = Path()


def without_comments(code: str) -> str:
    return re.sub(r"//[^\n]*", "", re.sub(r"/\*.*?\*/", "", code, flags=re.S))


def without_attributes(code: str) -> str:
    """`code` without its `#[…]` attributes (brackets balanced)."""
    out, i = [], 0
    while (j := code.find("#[", i)) >= 0:
        out.append(code[i:j])
        k, depth = j + 2, 1
        while k < len(code) and depth:
            depth += code[k] == "["
            depth -= code[k] == "]"
            k += 1
        i = k
    out.append(code[i:])
    return "".join(out)


def model_structs(code: str) -> dict:
    """{name: (generic parameters, {pub field: type})} of the `pub struct`s of comment- and attribute-free Rust."""
    out = {}
    for m in MODEL_STRUCT.finditer(code):
        i, depth = m.end(), 1
        while i < len(code) and depth:
            depth += code[i] == "{"
            depth -= code[i] == "}"
            i += 1
        params = tuple(p.strip() for p in check_facts.split_top(m.group(2)[1:-1])) if m.group(2) else ()
        out[m.group(1)] = (params, check_facts.fields_of(code[m.end():i - 1]))
    return out


def widget_parts(code: str) -> tuple:
    """({part: [fields]}, the values struct's name) of a generated widget file's `<Widget>Theme` (comments stripped,
    attributes kept): each `#[part(…)] <part>: PartStyle` with its `fields(…)`, and `values: <W>Values`."""
    parts = {}
    for m in PART_FIELD.finditer(code):
        listed = PART_FIELDS.search(m.group(1))
        parts[m.group(2)] = [x.strip() for x in listed.group(1).split(",") if x.strip()] if listed else []
    values = VALUES_FIELD.search(code)
    return parts, values.group(1) if values else None


def read_model(root: Path) -> Model:
    """The model as its Rust sources write it (check_facts.MODEL_DIR, its tests left out): every `pub struct` with
    its `pub` fields and their types, and each generated widget (`widgets/gen_<widget>.rs`, scripts/
    generate_widgets.py's output) with its parts' fields and its values struct. Read as text, so no build is needed;
    a folder without them gives an empty model."""
    folder = check_facts.MODEL_DIR
    model = Model({}, {}, set(), folder)
    for p in sorted((root / folder).rglob("*.rs")):
        if p.name.endswith("tests.rs") or "tests" in p.relative_to(root / folder).parts:
            continue
        code = without_comments(p.read_text(encoding="utf-8"))
        for name, struct in model_structs(without_attributes(code)).items():
            if name in model.structs and model.structs[name] != struct:
                model.ambiguous.add(name)
            model.structs[name] = struct
        if p.parent.name == "widgets" and p.name.startswith("gen_") and p.name != "gen_index.rs":
            parts, values = widget_parts(code)
            if parts:
                model.widgets[p.stem[len("gen_"):]] = (parts, values)
    return model


def bare_type(typ: str) -> str:
    t = typ.strip()
    while (m := WRAPPERS.fullmatch(t)):
        t = m.group(1).strip()
    return t


def struct_fields(model: Model, typ: str):
    """{field: type} of the struct `typ` names under its wrappers (`Value`, `Option`, `Box`), its generic parameters
    put in (`Sides<Value<Px>>`); None for any other type (a scalar, an enum, a list, a map)."""
    t = bare_type(typ)
    m = GENERIC.fullmatch(t)
    name, args = (m.group(1), [a.strip() for a in check_facts.split_top(m.group(2))]) if m else (t, [])
    if name not in model.structs:
        return None
    if name in model.ambiguous:
        raise InputError(f"{model.folder}: the struct {name} is written twice with other fields")
    params, fields = model.structs[name]
    put = dict(zip(params, args))
    return {f: put.get(ft, ft) for f, ft in fields.items()}


def stated_at(table: dict, name: str):
    """The value a TOML table states for a field, under its name or a unit suffix of it (S11)."""
    for suffix in ("",) + tuple(UNIT_OF_SUFFIX):
        if name + suffix in table:
            return table[name + suffix]
    return None


def unstated_fields(model: Model, text: str) -> list:
    """The model paths a preset's bases leave unstated, as TOML paths (`widgets.button.icon.icon.rendering_mode`,
    `colors.light.global.scrim`, `tokens.elevation`): each struct field of the colour roles under each scheme (the
    surface sets `[declare] surface_sets` lists, else those the bases state and `window` and `selection`, S3), of the
    tokens and of each typed widget's parts (the fields its `#[part]` lists) and values, with no value at its path
    and no `[pending]` path at or above it. A table there is walked below: a struct block's fields, a map's entries
    whose values are structs (S11: a block stated per leaf, an entry stated whole); any other value states the field
    whole. A part stated `absent = true` or `drawn_as` states its fields (S6.3); a part with no table but a `[pending]`
    path below it has its other fields walked. An unstated field is listed alone, not its fields below. The rules are
    not read: the reader needs a base for every field (S11). Not checked: the inputs (S9) and the entries
    `[declare]`'s keyed namespaces declare."""
    data = tomllib.loads(text)
    pending = {unsuffixed(tuple(k.split("."))) for k in data.get("pending", {})}
    out = []

    def walk(path, typ, node):
        bare = unsuffixed(path)
        if any(bare[:len(q)] == q for q in pending):
            return
        if node is None:
            if not any(len(q) > len(bare) and q[:len(bare)] == bare for q in pending):
                out.append(path)
                return
            node = {}
        if not isinstance(node, dict):
            return
        entry = MAP_TYPE.fullmatch(bare_type(typ))
        if entry:
            if struct_fields(model, entry.group(2)) is not None:
                for k, v in node.items():
                    walk(path + (k,), entry.group(2), v)
            return
        for name, ftype in (struct_fields(model, typ) or {}).items():
            walk(path + (name,), ftype, stated_at(node, name))

    roles = struct_fields(model, "ColorRoles") or {}
    declared = data.get("declare", {}).get("surface_sets") if isinstance(data.get("declare"), dict) else None
    colors = data.get("colors", {}) if isinstance(data.get("colors"), dict) else {}
    for scheme in SCHEMES if roles else ():
        node = colors.get(scheme)
        if not isinstance(node, dict):
            out.append(("colors", scheme))
            continue
        surfaces = node.get("surfaces", {}) if isinstance(node.get("surfaces"), dict) else {}
        sets = declared if isinstance(declared, list) else sorted(set(surfaces) | set(EVERY_PRESET_SETS))
        for name, typ in roles.items():
            inner = SURFACE_MAP.fullmatch(bare_type(typ))
            if inner:
                for s in sets:
                    walk(("colors", scheme, name, s), inner.group(1), surfaces.get(s))
            else:
                walk(("colors", scheme, name), typ, node.get(name))
    if "Tokens" in model.structs:
        walk(("tokens",), "Tokens", data.get("tokens"))
    style = struct_fields(model, "PartStyle") or {}
    widgets = data.get("widgets", {}) if isinstance(data.get("widgets"), dict) else {}
    for w, (parts, values) in model.widgets.items():
        node = widgets.get(w, {}) if isinstance(widgets.get(w), dict) else {}
        for part, fields in parts.items():
            here = node.get(part)
            at = ("widgets", w, part)
            if here is None and any(at[:len(q)] == q for q in pending):
                continue
            if here is None and not any(len(q) > len(at) and q[:len(at)] == at for q in pending):
                out.append(at)
                continue
            here = {} if here is None else here
            if not isinstance(here, dict) or here.get("absent") is True or "drawn_as" in here:
                continue
            for name in fields:
                walk(("widgets", w, part, name), style.get(name, ""), stated_at(here, name))
        if values:
            walk(("widgets", w, "values"), values, node.get("values"))
    return out


def fact_path(f: Fact) -> tuple:
    """A fact's key as the TOML path its base would take, the scheme left out of a colour role's (place())."""
    segs = f.ckey
    if f.kind == "role":
        rest = segs[1:]
        if rest[0].startswith("@"):
            return ("colors", "keyed", rest[0][1:]) + rest[1:]
        return ("colors",) + (rest if rest[0] == "global" else ("surfaces",) + rest)
    if f.kind == "token":
        return ("tokens",) + segs[1:]
    if f.kind == "input":
        return ("inputs",) + segs[1:]
    return ("components",) + segs[1:] if segs[0] == "component" else ("widgets",) + segs


def field_groups(facts, paths) -> list:
    """[(group, path, facts, pending, the at-rest facts' statuses)] of the unstated paths (unstated_fields()), by the
    preset's facts at, above or below each (the accent run's left out: it applies in that run alone): NO_FACT where
    none, ONLY_CONDITIONAL where none is at rest, else BASE_EXISTS."""
    keyed = [(fact_path(f), f) for f in facts if not accent_run(f.item.data)]
    out = []
    for path in paths:
        bare = unsuffixed(path)
        bare = bare[:1] + bare[2:] if bare[0] == "colors" else bare
        near = [f for p, f in keyed if p[:len(bare)] == bare or bare[:len(p)] == p]
        pending = sum(1 for f in near if "pending" in f.item.data)
        rest = [f for f in near if any(is_rest(f, s) for s in SCHEMES)]
        statuses = {}
        for f in rest:
            what = f.status + (f": {f.reason}" if f.reason else "")
            statuses[what] = statuses.get(what, 0) + 1
        group = NO_FACT if not near else ONLY_CONDITIONAL if not rest else BASE_EXISTS
        out.append((group, path, len(near) - pending, pending, statuses))
    return out


def field_lines(model: Model, draft: Draft, text: str) -> list:
    """The report's lines of the model fields the preset leaves unstated, grouped (field_groups())."""
    if "Tokens" not in model.structs:
        return [f"# unstated model fields: no model sources in {model.folder}"]
    groups = field_groups(draft.facts, unstated_fields(model, text))
    counts = {g: sum(1 for x in groups if x[0] == g) for g in FIELD_GROUPS}
    out = [f"# unstated model fields: {len(groups)} ("
           + ", ".join(f"{g.lower()} {n}" for g, n in counts.items())
           + f"): the fields of the model ({model.folder}) with no base value and no [pending] path in the preset "
           f"as --write writes it"]
    for g, why in FIELD_GROUPS.items():
        if counts[g]:
            out.append(f"# == {g}: {counts[g]} ({why})")
            for _, path, facts, pending, statuses in (x for x in groups if x[0] == g):
                at_rest = (" (at rest: " + ", ".join(f"{k} {n}" for k, n in statuses.items()) + ")") if statuses \
                    else ""
                out.append(f"#   {dotted_key(path)}  facts={facts} pending={pending}{at_rest}")
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


def _pending(key, uid, when=None):
    w = f"when = {when}\n" if when else ""
    return f'\n[[fact]]\nkey = "{key}"\n{w}pending = "{uid}"\nrows = []\n'


SELF_TEST_FILES = {
    "scripts/widget_units.toml": check_facts_trace.SELF_TEST_UNITS,
    "docs/facts/sources.toml": check_facts.SELF_TEST_SOURCES,
    "docs/facts/gnome/state-models.toml": '''
[[model]]
widget = "button"
available = true
variants = ["flat", "primary", "@pill", "@raised", "@lifted"]
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
        _fact("button.body.fill", '"#000000"', '{ context = [{ scope = "any", widget = "toolbar", part = "bar" }], '
                                               'not_variants = ["@raised"] }'),
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
        _pending("pull_to_refresh.body.opacity", "U9", '{ states = ["hover"] }'),
        _fact("button.body.min_width", '"30px"', '{ variants = ["@lifted"], not_context = [{ scope = "any", '
                                                 'widget = "toolbar", part = "bar" }] }'),
        _fact("button.mark.glyph", '{ source = { named_system_image = { name = "circle" } }, stroke = "absent" }'),
        _fact("button.mark.glyph", '{ source = { named_system_image = { name = "checkmark" } }, stroke = "absent" }',
              '{ states = ["checked"] }'),
        _fact("button.body.fill", '{ layers = ["#ff0000"] }', '{ variants = ["primary"] }'),
        _fact("button.body.min_width", '"= add(element.font_underline_thickness, widget.label.font_height)"',
              '{ states = ["pressed"] }'),
        _fact("button.values.icon_text_gap", '"= max(element.width, element.window_width)"',
              '{ states = ["checked"] }'),
        _fact("button.body.min_height", '"= element.progress"', '{ context = [{ scope = "any", '
                                                               'widget = "toolbar", part = "bar" }] }'),
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
        _fact("role.global.accent", '"= input.selected"', '{ conditions = ["window_inactive"] }', cite=_READ),
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
        _pending("token.text.fallbacks.families", "U8"),
    ]),
}

SELF_TEST_EXPECTED = '''# adwaita (S11): generated by scripts/generate_preset.py from docs/facts/gnome/, each value's comment naming its fact.
# The lines between `# hand-written: begin <part>` and `# hand-written: end <part>` state what no fact states,
# written by hand (S11, S17: `[preset] generation`, the `[declare]` keyed namespaces, the `read` facts as bases);
# `generate_preset.py <id> --write` regenerates every other line and carries them over.
# generated lines: sha256 <digest>

[preset]
id = "adwaita"
# hand-written: begin preset
# hand-written: end preset

[declare]
conditions = ["@reduced_chrome"]
# hand-written: begin declare
# hand-written: end declare

# docs/facts/gnome/state-models.toml:2
[declare.widget.button]
available = true
variants = ["flat", "primary", "@pill", "@lifted"]
sizes = []
densities = []
flags = ["@busy"]
exclusive = [["disabled", "pressed", "hover"]]
exclusive_variants = [["flat", "primary"]]
recency = { glow = ["checked", "hover"] }
# docs/facts/gnome/button.toml:400, docs/facts/gnome/button.toml:411
data = { body = ["font_underline_thickness", "widget.label.font_height"], values = ["width"] }

[pending]
# docs/facts/gnome/tokens.toml:93
"tokens.text.fallbacks.families" = "U8"

[inputs.accent]
# docs/facts/gnome/roles.toml:87
light = "#3584e4"
# docs/facts/gnome/roles.toml:87
dark = "#3584e4"
# docs/facts/gnome/roles.toml:97
contrast.high.light = "#1c71d8"

[colors.light.surfaces.window]
# docs/facts/gnome/roles.toml:2
background = "#fafafb"
# docs/facts/gnome/roles.toml:35
foreground = "= alpha(#000006, 0.8)"

[colors.light.global]
# hand-written: begin bases colors.light.global
# hand-written: end bases colors.light.global
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
# hand-written: begin bases colors.dark.global
# hand-written: end bases colors.dark.global
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
# S4.5
animated_visuals = {}

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

[widgets.button.mark]
# docs/facts/gnome/button.toml:368
glyph = { source = { named_system_image = { name = "circle" } }, stroke = "absent" }

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
# docs/facts/gnome/button.toml:389
fill = { layers = ["#ff0000"] }

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
when = { variants = ["@lifted"] }
target = "button.body"

[rules.set]
# docs/facts/gnome/button.toml:357
min_width_px = 30

[[rules]]
when = { states = ["checked"] }
target = "button.mark"

[rules.set]
# docs/facts/gnome/button.toml:378
glyph = { replace = { source = { named_system_image = { name = "checkmark" } }, stroke = "absent" } }

[[rules]]
when = { states = ["pressed"] }
target = "button.body"

[rules.set]
# docs/facts/gnome/button.toml:400
min_width_px = "= add(element.font_underline_thickness, widget.label.font_height)"

[[rules]]
when = { states = ["checked"] }
target = "button.values"

[rules.set]
# docs/facts/gnome/button.toml:411
icon_text_gap_px = "= max(element.width, element.window_width)"

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
when = { conditions = ["window_inactive"] }
target = "colors"

[rules.set]
# hand-written: begin rule role.global.accent { conditions = ["window_inactive"] }
# hand-written: end rule role.global.accent { conditions = ["window_inactive"] }

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
    ("pull_to_refresh.body.opacity", "U9", (LEFT_OUT, NOT_HELD)),
    ("token.text.fallbacks.families", "U8", (PENDING, "")),
    ("button.body.min_width", "30px", ("rule", "")),
]


# a model's Rust sources (read_model()) and a preset stating every field of it (unstated_fields())
MODEL_FILES = {
    "native-theme/src/roles.rs": """
/// The colour roles (S3), with a `// comment` in a doc line.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
pub struct ColorRoles {
    /// The declared surface sets' roles.
    #[parts(block = "fields")]
    pub surfaces: SurfaceMap<SurfaceRoles>,
    pub global: GlobalRoles, // the global roles
    pub keyed: Vec<(Key, Value<Color>)>,
}
pub struct SurfaceRoles { pub background: Value<Color>, pub disabled: DisabledRoles }
pub struct DisabledRoles { pub background: Value<Color> }
/* a block comment: pub struct GlobalRoles { pub other: Value<Color> } */
pub struct GlobalRoles { pub accent: Value<Color>, pub accent_ramp: Vec<(Key, Value<Color>)> }
""",
    "native-theme/src/tokens.rs": """
pub struct Tokens { pub focus: FocusIndicator, pub text: TextTokens }
pub struct FocusIndicator { pub width: Value<Px>, pub offset: Sides<Value<Px>> }
pub struct TextTokens { pub keyed: Vec<(Key, TextStyle)>, pub font_dpi: Value<f32> }
pub struct TextStyle { pub size: Value<FontSize>, pub weight: Value<u16> }
""",
    "native-theme/src/units.rs": "pub struct Sides<T> { pub top: T, pub right: T, pub bottom: T, pub left: T }\n",
    "native-theme/src/blocks/part_style.rs": """
pub struct PartStyle { pub fill: Value<Paint>, pub padding: Sides<Value<Px>>, pub icon: Value<IconStyle> }
pub struct IconStyle { pub size: Value<Px>, pub weight: Value<u16> }
""",
    "native-theme/src/widgets/gen_button.rs": """
#[derive(ThemeParts)]
#[parts(widget)]
pub(crate) struct ButtonTheme {
    /// The bezel.
    #[part(root, fields(fill, padding), absent(fill))]
    body: PartStyle,
    /// The image.
    #[part(may_be_absent, fields(icon))]
    icon: PartStyle,
    /// The mark.
    #[part(may_be_absent, fields(fill))]
    mark: PartStyle,
    values: ButtonValues,
}
pub struct ButtonValues { pub gap: Value<Px> }
""",
    "native-theme/src/widgets/tests.rs": "pub struct Tokens { pub other: Value<Px> }\n",
}
FIELDS_COLORS = """
[colors.{s}.surfaces.window]
background = "#ffffff"
disabled.background = "#eeeeee"

[colors.{s}.surfaces.view]
background = "#fafafa"
disabled = {{ background = "#dddddd" }}

[colors.{s}.global]
accent = "#3584e4"
accent_ramp = {{}}

[colors.{s}.keyed]
overview = "#000000"
"""
FIELDS_PRESET = """
[declare]
surface_sets = ["window", "view"]

[pending]
"tokens.text.keyed.title.weight" = "U1"
"widgets.button.icon.icon.size_px" = "U2"
""" + FIELDS_COLORS.format(s="light") + FIELDS_COLORS.format(s="dark") + """
[tokens.focus]
width_px = 2
offset = { top_px = 1, right_px = 1, bottom_px = 1, left_px = 1 }

[tokens.text]
font_dpi = 72
keyed.body = { size_pt = 11, weight = 400 }
keyed.title.size_pt = 15

[widgets.button.body]
fill = "#ffffff"
padding.top_px = 4
padding.right_px = 4
padding.bottom_px = 4
padding.left_px = 4

[widgets.button.icon]
icon.weight = 400

[widgets.button.mark]
absent = true

[widgets.button.values]
gap_px = 6
"""
# a line of FIELDS_PRESET removed (each scheme's), and the paths unstated_fields() then gives
FIELDS_CASES = [
    ("width_px = 2", [("tokens", "focus", "width")]),
    ("padding.left_px = 4", [("widgets", "button", "body", "padding", "left")]),
    ("gap_px = 6", [("widgets", "button", "values", "gap")]),
    ('"widgets.button.icon.icon.size_px" = "U2"', [("widgets", "button", "icon", "icon", "size")]),
    ('"tokens.text.keyed.title.weight" = "U1"', [("tokens", "text", "keyed", "title", "weight")]),
    ("absent = true", [("widgets", "button", "mark", "fill")]),
    ("[widgets.button.icon]\nicon.weight = 400", [("widgets", "button", "icon", "icon", "weight")]),
    ("accent_ramp = {}", [("colors", "light", "global", "accent_ramp"), ("colors", "dark", "global", "accent_ramp")]),
    ('surface_sets = ["window", "view"]', [("colors", "light", "surfaces", "selection"),
                                          ("colors", "dark", "surfaces", "selection")]),
    ('disabled.background = "#eeeeee"', [("colors", "light", "surfaces", "window", "disabled"),
                                         ("colors", "dark", "surfaces", "window", "disabled")]),
]


def fixture_context(root: Path):
    return check_facts.Context(root, core_text=check_facts.SELF_TEST_CORE, cat_text=check_facts.SELF_TEST_CATALOGUE,
                               sources_text=check_facts.SELF_TEST_SOURCES, rows=check_facts.SELF_TEST_ROWS,
                               evidence=root)


def write(root: Path, files: dict):
    for rel, text in files.items():
        p = root / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text, encoding="utf-8")


HAND_PRESET = '''# docs/facts/sources.toml:1
generation = "libadwaita-1.10"'''
HAND_DECLARE = """text_styles = [
    "title_1",  # docs/facts/gnome/tokens.toml:1
]

[declare.params.opacity]
# S4.6: one unit per group
unit = "fraction"
keys = [
    "dim",  # docs/facts/gnome/tokens.toml:2
]"""
HAND_ACCENT = '''# docs/facts/gnome/roles.toml:3
accent = "= input.accent"'''


def hand_written_cases(text: str) -> list:
    """--write's form (the docstring's *The hand-written parts*): the blocks carried exactly and the result written
    again unchanged; each way of losing or doubling a hand-written line refused."""
    failures = []
    filled = {"preset": HAND_PRESET, "declare": HAND_DECLARE, "bases colors.light.global": HAND_ACCENT,
              "bases colors.dark.global": HAND_ACCENT}
    committed = text
    for part, lines in filled.items():
        marker = f"{HAND}{BEGIN}{part}\n"
        committed = committed.replace(marker, marker + lines + "\n")
    try:
        once = carried(text, committed)
        twice = carried(text, once)
    except InputError as err:
        return [f"--write refuses a well-formed preset: {err}"]
    if once != committed or twice != committed:
        failures.append("--write does not reproduce a preset whose generated lines are unchanged, byte for byte")
    if carried(text, None) != text:
        failures.append("--write without a bundled preset does not write the generated text")
    # a regeneration that changes a generated line keeps every block
    regenerated = text.replace('accent_ramp = {}', 'accent_ramp = { a = "#000000" }')
    regenerated = regenerated.replace(hand_parts(text)[2], digest(hand_parts(regenerated)[1]))
    kept = carried(regenerated, committed)
    if hand_parts(kept)[0] != hand_parts(committed)[0] or "accent_ramp = { a" not in kept:
        failures.append("--write over changed facts does not keep the blocks and take the new generated lines")
    accent = 'accent = "= input.accent"'
    refused = [
        ("a hand-written line edited outside its block", committed.replace(
            "[declare]\n", '[declare]\ndurations = ["button"]\n'), "its digest differs"),
        ("a hand-written line moved out of its block", committed.replace(
            f"{accent}\n{HAND}{END}bases colors.dark.global", f"{HAND}{END}bases colors.dark.global\n{accent}"),
         "its digest differs"),
        ("a part twice",
         committed.replace(f"{BEGIN}preset", f"{BEGIN}declare").replace(f"{END}preset", f"{END}declare"), "twice"),
        ("a block never ended", committed.replace(f"{HAND}{END}preset\n", ""), "opens or ends no"),
        ("no digest", "\n".join(x for x in committed.split("\n") if not x.startswith(DIGEST)), "not written by"),
    ]
    # the new facts give no place for a block (the dark scheme's `read` fact gone)
    placeless = text.replace("".join(f"{x}\n" for x in block("bases colors.dark.global")), "")
    placeless = placeless.replace(hand_parts(text)[2], digest(hand_parts(placeless)[1]))
    refused = [(what, bad, needle, text) for what, bad, needle in refused] + [
        ("a block whose part has no place", committed, "no place", placeless)]
    for what, bad, needle, generated in refused:
        try:
            carried(generated, bad)
            failures.append(f"--write takes {what}")
        except InputError as err:
            if needle not in str(err):
                failures.append(f"--write refuses {what}, but as {err}")
    # the verification of a result: a hand-written line dropped, changed or doubled, and a duplicate key
    title = '    "title_1",  # docs/facts/gnome/tokens.toml:1\n'
    broken = [
        ("a line dropped", committed.replace(title, "")),
        ("a line changed", committed.replace('"title_1"', '"title_2"')),
        ("a line doubled", committed.replace(title, title + title)),
        ("a key the generator writes too", committed.replace(
            f"{HAND}{END}bases colors.light.global", f"accent_ramp = {{}}\n{HAND}{END}bases colors.light.global")),
    ]
    for what, result in broken:
        if not verify(committed, result):
            failures.append(f"verify() takes {what}")
    if verify(committed, committed):
        failures.append(f"verify() refuses an exact carry: {verify(committed, committed)}")
    return failures


# the fixture's hand-written blocks with their citations, (the committed lines, the lines --write writes) each: a
# true citation kept (a later fact naming the entry), stale ones pointed at the fact naming their subject in the
# file they name (another file's where it names none), and ones whose fact none is found for kept and listed
CITED_BLOCKS = {
    "preset": ("# docs/facts/sources.toml:1 (libadwaita@1.10.0, its release)\ngeneration = \"libadwaita-1.10\"",
               "# docs/facts/sources.toml:2 (libadwaita@1.10.0, its release)\ngeneration = \"libadwaita-1.10\""),
    "declare": ('''durations = [
    "button",  # docs/facts/gnome/tokens.toml:32
]
timings = [
    "ease",  # docs/facts/gnome/tokens.toml:2
    "missing",  # docs/facts/gnome/tokens.toml:12
]
color_roles = [
    "overview_background",  # docs/facts/gnome/button.toml:2
]
blend_modes = [
    "multiply",  # docs/facts/gnome/tokens.toml:2
]''', '''durations = [
    "button",  # docs/facts/gnome/tokens.toml:32
]
timings = [
    "ease",  # docs/facts/gnome/tokens.toml:12
    "missing",  # docs/facts/gnome/tokens.toml:12
]
color_roles = [
    "overview_background",  # docs/facts/gnome/roles.toml:65
]
blend_modes = [
    "multiply",  # docs/facts/gnome/tokens.toml:2
]'''),
    "bases colors.light.global": ('# docs/facts/gnome/roles.toml:3\naccent = "= input.accent"',
                                  '# docs/facts/gnome/roles.toml:45\naccent = "= input.accent"'),
    "bases colors.dark.global": ('# docs/facts/gnome/roles.toml:45\naccent = "= input.accent"',
                                 '# docs/facts/gnome/roles.toml:45\naccent = "= input.accent"'),
    'rule role.global.accent { conditions = ["window_inactive"] }': (
        '# docs/facts/gnome/roles.toml:45\nglobal.accent = "= input.accent"',
        '# docs/facts/gnome/roles.toml:119\nglobal.accent = "= input.accent"'),
}


def _item(file, line, data, table="fact"):
    return check_facts.Item(f"docs/facts/kde/{file}.toml", line, table, "kde", data, [])


# NAMED_BY's rules over synthetic items: (the entry line's namespace and name, its cited file and line, the line it
# is refreshed to, None for kept, "finding" for none found)
NAMING_ITEMS = [
    _item("roles", 5, {"key": "role.@hover_tint", "value": "#000000"}),
    _item("tokens", 7, {"key": "role.@hover_tint", "value": "#111111"}),
    _item("tokens", 9, {"key": "token.text.keyed.heading.font.size", "value": "13pt",
                        "when": {"conditions": ["preset:adwaita"]}}),
    _item("tokens", 11, {"key": "token.text.keyed.heading.font.size", "value": "12pt"}),
    _item("inputs", 13, {"key": "input.inactive.view.background_normal", "value": "#ffffff"}),
    _item("tokens", 15, {"key": "token.params.layout.grid_unit", "value": "18px"}),
    _item("button", 17, {"key": "button.icon.icon.scale", "value": "@large"}),
    _item("tokens", 19, {"key": "token.icons.sizes.toolbar.symbolic_colors", "value": {"accent": "#000000"}}),
    _item("tokens", 21, {"key": "token.icons.sizes.keyed.main_toolbar.size", "value": "22px"}),
    _item("state-models", 23, {"widget": "button"}, "model"),
    _item("button", 25, {"key": "button.body.fill", "value": "#000000",
                         "when": {"conditions": ["flavor:kirigami"]}}),
    _item("state-models", 27, {"widget": "button", "when": {"conditions": ["flavor:kirigami"]}}, "model"),
]
NAMING_CASES = [
    ("color_roles", "hover_tint", "tokens:7", None),
    ("color_roles", "hover_tint", "tokens:8", "tokens:7"),
    ("color_roles", "hover_tint", "button:8", "roles:5"),
    ("text_styles", "heading", "tokens:9", "tokens:11"),
    ("inputs", "inactive.view.background_normal", "inputs:1", "inputs:13"),
    ("params.layout", "grid_unit", "tokens:1", "tokens:15"),
    ("params.shape", "grid_unit", "tokens:15", "finding"),
    ("symbol_scales", "large", "tokens:1", "button:17"),
    ("symbolic_colors", "accent", "tokens:1", "tokens:19"),
    ("icon_contexts", "main_toolbar", "tokens:1", "tokens:21"),
    ("flavors", "widgets", "state-models:1", "state-models:23"),
    ("flavors", "widgets", "button:1", "state-models:23"),
    ("flavors", "kirigami", "state-models:1", "state-models:27"),
    ("flavors", "kirigami", "state-models:23", "state-models:27"),
    ("durations", "heading", "tokens:11", "finding"),
    ("blend_modes", "multiply", "tokens:11", "finding"),
]


def naming_cases() -> list:
    """Citations over NAMING_ITEMS on kde-breeze: each NAMING_CASES entry in a list of its own namespace (the
    flavours' first entry the default, `widgets` before `kirigami`), its citation kept (None), refreshed, or kept
    and listed ("finding")."""
    failures = []
    draft = Draft("kde-breeze", "kde", [], [], [], [], [], {}, [], 0, items=NAMING_ITEMS)
    for ns, name, cited, want in NAMING_CASES:
        cites = Citations(draft, "")
        head = [f"[declare.params.{ns[7:]}]", "keys = ["] if ns.startswith("params.") else [f"{ns} = ["]
        first = ['    "widgets",  # docs/facts/kde/state-models.toml:23'] if ns == "flavors" and name != "widgets" else []
        entry = f'    "{name}",  # docs/facts/kde/{{}}.toml:{{}}'
        got = cites.refresh({"declare": head + first + [entry.format(*cited.split(":")), "]"]})["declare"][-2]
        expected = entry.format(*(cited if want in (None, "finding") else want).split(":"))
        if got != expected or bool(cites.findings) != (want == "finding") or bool(cites.refreshed) != (
                want not in (None, "finding")):
            failures.append(f"citations: {ns} {name!r} citing {cited}: {got!r}, refreshed {cites.refreshed}, "
                            f"findings {cites.findings}")
    return failures


def citation_cases(root: Path) -> list:
    """The hand-written citations (the docstring's *The hand-written citations*): --check fails on a stale one and
    lists it, --write points each stale one at its fact and changes nothing else, keeps and lists one whose fact none
    is found for and exits 1, and --check passes once none is left."""
    failures = []
    write(root, SELF_TEST_FILES)
    path = root / "native-theme/src/presets/adwaita.toml"
    path.parent.mkdir(parents=True)
    with contextlib.redirect_stderr(io.StringIO()):
        run(fixture_context(root), "adwaita", "--write", None)
    text = path.read_text(encoding="utf-8")
    if set(hand_parts(text)[0]) != set(CITED_BLOCKS):
        return [f"citations: the fixture's parts are {sorted(hand_parts(text)[0])}"]

    def filled(which):
        out = text
        for part, lines in CITED_BLOCKS.items():
            marker = f"{HAND}{BEGIN}{part}\n"
            out = out.replace(marker, marker + lines[which] + "\n")
        return out

    stale, fresh = filled(0), filled(1)
    path.write_text(stale, encoding="utf-8")
    with contextlib.redirect_stderr(io.StringIO()) as err:
        code = run(fixture_context(root), "adwaita", "--check", None)
    said = err.getvalue()
    listed = [x for x in said.split("\n") if x.startswith("stale citation: ")]
    if code != 1 or len(listed) != 5 or path.read_text(encoding="utf-8") != stale or any(
            needle not in said for needle in ("timings 'ease': docs/facts/gnome/tokens.toml:2 names no fact of it, "
                                              "docs/facts/gnome/tokens.toml:12 does",
                                              "citation: adwaita: `declare` timings 'missing'",
                                              "`blend_modes` (NAMED_BY)")):
        failures.append(f"--check over stale citations: exit {code}, said {said!r}")
    with contextlib.redirect_stderr(io.StringIO()) as err:
        code = run(fixture_context(root), "adwaita", "--write", None)
    said = err.getvalue()
    if code != 1 or path.read_text(encoding="utf-8") != fresh or "hand-written citations refreshed: 5" not in said:
        failures.append(f"--write over stale citations: exit {code}, written as expected "
                        f"{path.read_text(encoding='utf-8') == fresh}, said {said!r}")
    for gone in ('    "missing",  # docs/facts/gnome/tokens.toml:12\n',
                 'blend_modes = [\n    "multiply",  # docs/facts/gnome/tokens.toml:2\n]\n'):
        fresh = fresh.replace(gone, "")
    path.write_text(fresh, encoding="utf-8")
    with contextlib.redirect_stderr(io.StringIO()) as err:
        code = run(fixture_context(root), "adwaita", "--check", None)
    if code != 0 or "citation" in err.getvalue():
        failures.append(f"--check over true citations: exit {code}, said {err.getvalue()!r}")
    unnamed = fresh.replace(" (libadwaita@1.10.0, its release)", "")
    path.write_text(unnamed, encoding="utf-8")
    with contextlib.redirect_stderr(io.StringIO()) as err:
        code = run(fixture_context(root), "adwaita", "--check", None)
    if code != 1 or "names no source id" not in err.getvalue():
        failures.append(f"--check over a generation comment naming no source: exit {code}, said {err.getvalue()!r}")
    return failures


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
    # a keyed param group's unit literals (S4.6, S10, S11): their number in the group's one unit, read back with it;
    # a group mixing units has none
    class Item:
        def __init__(self, value):
            self.data, self.file, self.line, self.cites = {"value": value}, "tokens.toml", 1, []

    def param(group, key, value):
        return Fact(Item(value), f"token.params.{group}.{key}", ("token", "params", group, key), {}, "token", None,
                    "Scalar", None)

    params = [param("pointer_repeat", "delay", "500ms"), param("pointer_repeat", "interval", "250ms"),
              param("mixed", "a", "2px"), param("mixed", "b", "3ms"), param("opacity", "dim", 0.5)]
    if param_units(params) != {"pointer_repeat": "ms"}:
        failures.append(f"param_units: {param_units(params)}")
    delay = params[0]
    delay.param_unit = "ms"
    place(delay)
    if (delay.header, delay.dotted, delay.value, back_value(delay, delay.value)) != (
            ("params",), ("pointer_repeat", "delay"), 500, "500ms"):
        failures.append(f"a keyed param's unit literal: {delay.header} {delay.dotted} {delay.value!r}")
    # a key's files (S17 *Which fact applies*): an accent-run fact applies in the accent run alone (S17 *The accent
    # run*), so its file is none of the key's; two files of the preset's own run are
    def in_file(key, file, **run):
        f = param("x", "y", "#ffffff")
        f.key, f.ckey, f.item.file = key, tuple(key.split(".")), file
        f.item.data.update(run)
        return f

    accent = {"scheme_file": "breeze@6.8.2 colors/BreezeLight.colors", "accent": "#e5a50a"}
    got = key_files([in_file("role.window.background", "roles.toml"),
                     in_file("role.window.background", "accent.toml", **accent),
                     in_file("role.view.background", "roles.toml"), in_file("role.view.background", "other.toml")])
    want = {("role", "window", "background"): {"roles.toml"},
            ("role", "view", "background"): {"roles.toml", "other.toml"}}
    if got != want:
        failures.append(f"key_files: {got}, expected {want}")
    # a set-whole block's leaves: whole in one base, or in one rule; split between a base and a rule, both left out
    def leaf(name, rel):
        return Fact(None, f"token.behavior.key_repeat.{name}", ("token", "behavior", "key_repeat", name), {}, "token",
                    None, None, "duration", rel=rel)

    def repeat_block(_ctx, f):
        return ("delay", "interval") if f.ckey[:3] == ("token", "behavior", "key_repeat") else None

    delay, interval = leaf("delay", ("behavior", "key_repeat", "delay_ms")), leaf("interval", ("behavior", "key_repeat",
                                                                                               "interval_ms"))
    def base(f):
        return "tokens", None, ("behavior",), ("key_repeat", f.rel[-1]), 1, f

    whole_rule = Rule({}, "tokens", [delay, interval])
    cases = [([base(delay), base(interval)], [], []), ([], [whole_rule], []),
             ([base(delay)], [Rule({"states": ["hover"]}, "tokens", [interval])], [delay, interval])]
    for bases, rules, want in cases:
        got = split_blocks(None, bases, rules, repeat_block)
        if [f.key for f in got] != [f.key for f in want]:
            failures.append(f"split_blocks: {[f.key for f in got]}, expected {[f.key for f in want]}")
    # the values the specification defines (S9.4, S4.5): `font_dpi` on the Apple presets, `animated_visuals` on every
    # preset but windows-11, each a base value of its own; a fact stating the key leaves no room for it
    for preset, want in (("macos-tahoe", ["S9.4", "S4.5"]), ("ios-26", ["S9.4", "S4.5"]), ("windows-11", []),
                         ("kde-breeze", ["S4.5"])):
        got = [(d[4], d[0], d[1], d[2], d[3]) for d in definitions(preset, {})]
        spelt = {"S9.4": ("tokens", ("text",), ("font_dpi",), 72),
                 "S4.5": ("tokens", ("icons",), ("animated_visuals",), {})}
        if got != [(s,) + spelt[s] for s in want]:
            failures.append(f"definitions({preset!r}): {got}, expected {want}")
    try:
        definitions("adwaita", {"tokens": {"icons": {"animated_visuals": Leaf(None)}}})
        failures.append("definitions: a fact's base at a defined key not refused")
    except InputError:
        pass
    # unspell: a suffix only where the key's type takes it; element_size: the size an element holds
    if unspell("edge_px", 1, "Value<Scalar>") != ("edge_px", 1) or unspell("gap_px", 6, "Value<Px>") != ("gap", "6px"):
        failures.append("unspell: a suffix stripped where the type takes none, or kept where it takes one")
    # (S10, ADD-A131): the variant entry's sizes, else the widget's, kept by the context entry; the default in force
    # (context, variant, widget), else, pending or refused, the first size the `when` allows; a variant whose entry
    # takes a size the `when` allows where the widget's own sizes are all refused; none for an unsized element
    sized_model = {"sizes": ["small", "regular"], "default_size": "regular", "exclusive_variants": [["flat", "primary"]],
                   "sizes_by_variant": [{"variant": "flat", "sizes": ["small", "regular"], "default_size": "small"},
                                        {"variant": "primary", "sizes": ["mini", "large"],
                                         "default_size": {"pending": "U9"}}],
                   "sizes_by_context": [{"context": {"widget": "toolbar", "part": "bar"}, "sizes": ["small", "large"],
                                         "default_size": "small"}]}
    bar = [{"widget": "toolbar", "part": "bar", "surface": None, "variants": set(), "states": set(), "sizes": set()}]
    cases = [({}, (), (["regular"], [])), ({"variants": ["flat"]}, (), (["small"], ["flat"])),
             ({"sizes": ["small"]}, (), (["small"], [])), ({"variants": ["primary"]}, (), (["mini"], ["primary"])),
             ({"not_sizes": ["regular"]}, (), (["small"], [])),
             ({"not_sizes": ["regular", "small"]}, (), (["mini"], ["primary"])),
             ({"not_sizes": ["regular", "small"], "variants": ["flat"]}, (), (None, ["flat"])),
             ({"not_sizes": ["regular", "small"], "not_variants": ["primary"]}, (), (None, [])),
             ({}, bar, (["small"], [])), ({"variants": ["primary"]}, bar, (["large"], ["primary"])),
             ({"sizes": ["mini"]}, (), (["mini"], ["primary"])), ({"sizes": ["mini"], "variants": ["flat"]}, (),
                                                                  (None, ["flat"])),
             ({"sizes": ["small"]}, bar, (["small"], []))]
    for when, ancestors, want in cases:
        got = element_size(when, sized_model, ancestors)
        if got != want:
            failures.append(f"element_size({when}, {ancestors}): {got}, expected {want}")
    # a variant no element holding the `when`'s variants is drawn with (a `not_drawn` entry of variants alone) adds
    # no size
    undrawn_model = dict(sized_model, not_drawn=[{"variants": ["primary"]}])
    for when, want in (({"not_sizes": ["regular", "small"]}, (None, [])), ({"sizes": ["mini"]}, (None, []))):
        got = element_size(when, undrawn_model)
        if got != want:
            failures.append(f"element_size({when}) under not_drawn: {got}, expected {want}")
    if element_size({}, {"sizes": []}) != ([], []):
        failures.append(f"element_size of an unsized element: {element_size({}, {'sizes': []})}")
    # held_contexts: a `sizes_by_context` entry naming a widget the model does not hold is left out, the others kept
    contexts = [("sizes", ["regular"]), ("sizes_by_context", [
        {"context": {"widget": "toolbar"}, "sizes": ["regular"]},
        {"context": {"scope": {"nearest": ["toolbar"]}, "widget": "button"}, "sizes": ["regular"]},
        {"context": {"widget": "button"}, "sizes": ["small"]}])]
    slice_units = {"slice": check_facts_trace.Unit("slice", ("button",))}
    try:
        got = held_contexts(contexts, slice_units)
    except (KeyError, TypeError) as err:
        got = repr(err)
    if got != [("sizes", ["regular"]), ("sizes_by_context", [{"context": {"widget": "button"}, "sizes": ["small"]}])]:
        failures.append(f"held_contexts: {got}")
    # pending_defaults: each pending default size taken out of [declare] and listed by its declaration's path
    kvs = [("sizes", ["regular"]), ("default_size", {"pending": "U1"}),
           ("sizes_by_variant", [{"variant": "@gray", "sizes": ["mini"], "default_size": {"pending": "U2"}},
                                 {"variant": "flat", "sizes": ["mini"], "default_size": "mini"}]),
           ("sizes_by_context", [{"context": {"widget": "toolbar"}, "sizes": ["regular"],
                                  "default_size": {"pending": "U3"}}])]
    cited = type("Item", (), {"file": "s.toml", "line": 7})()
    kept, listed = pending_defaults(("flavor", "f", "widget", "button"), kvs, cited)
    head = "declare.flavor.f.widget.button"
    if kept != [("sizes", ["regular"]), ("sizes_by_variant", [{"variant": "@gray", "sizes": ["mini"]},
                                                              {"variant": "flat", "sizes": ["mini"],
                                                               "default_size": "mini"}]),
                ("sizes_by_context", [{"context": {"widget": "toolbar"}, "sizes": ["regular"]}])] or [
            (path, uid, c.where) for path, uid, c in listed] != [
            (f"{head}.default_size", "U1", "s.toml:7"), (f"{head}.sizes_by_variant.@gray.default_size", "U2", "s.toml:7"),
            (f"{head}.sizes_by_context.0.default_size", "U3", "s.toml:7")]:
        failures.append(f"pending_defaults: {kept}, {[(path, uid) for path, uid, _ in listed]}")
    # pending_table: a pending leaf of a set-whole block lists the block; a pending fact with a base beneath, or its
    # block's, lists nothing
    class _Item:
        def __init__(self, uid):
            self.data, self.file, self.line, self.cites = {"pending": uid}, "t.toml", 1, []

    def pending_fact(key, uid, dim="duration"):
        return Fact(_Item(uid), key, tuple(key.split(".")), {}, "token", None, None, dim, status=PENDING)

    interval = pending_fact("token.behavior.key_repeat.interval", "U3")
    touch = pending_fact("token.behavior.touch.delay", "U4")
    got = [(path, uid) for path, uid, _ in pending_table(None, [interval, touch], [
        ("tokens", None, ("behavior",), ("touch", "delay_ms"), 1, touch)], repeat_block)]
    if got != [("tokens.behavior.key_repeat", "U3")]:
        failures.append(f"pending_table: {got}, expected the key repeat block alone")
    # the leaves of a block set whole pending under different ids list it with both, in fact order
    delay = pending_fact("token.behavior.key_repeat.delay", "U2")
    got = [(path, uid) for path, uid, _ in pending_table(None, [delay, interval], [], repeat_block)]
    if got != [("tokens.behavior.key_repeat", ["U2", "U3"])]:
        failures.append(f"pending_table: {got}, expected the key repeat block with U2 and U3")
    # a base above the path (a block stated whole, a whole part) or below it lists nothing; a colour role is listed
    # per scheme its base leaves unstated; a pending fact a later fact covers lists nothing; an input is listed per
    # scheme its base leaves unstated, a pending contrast base never; one under an earlier rule of its key holding
    # wherever it does lists nothing, one under an earlier rule holding in less is listed; a pending presence is
    # listed by its part's path beside its fields' bases, and not under a base of its presence
    def any_fact(key, uid, kind, when=None, widget=None, status=PENDING):
        f = Fact(_Item(uid), key, tuple(key.split(".")), when or {}, kind, widget, None, None, status=status)
        if status != PENDING:
            f.item.data = {"value": 1}
        return f

    families = any_fact("token.text.fallbacks.families", "U5", "token")
    caret = any_fact("token.caret", "U6", "token")
    status = any_fact("role.global.status", "U7", "role")
    drag = any_fact("token.behavior.drag_time", "U8", "token")
    later = any_fact("token.behavior.drag_time", "", "token", status="rule")
    mode = any_fact("button.icon.icon.rendering_mode", "U9", "widget", widget="button")
    accent, scheme = any_fact("input.accent", "U10", "input"), any_fact("input.scheme", "U11", "input")
    bases = [("tokens", None, ("text",), ("fallbacks",), {}, families),
             ("tokens", None, ("caret",), ("width_px",), 1, caret),
             ("colors", "light", ("global",), ("status",), {}, status),
             ("widgets", None, ("button", "icon"), ("absent",), True, mode),
             ("inputs", None, ("scheme",), ("light",), "#000000", scheme)]
    earlier = any_fact("token.text.caret_blink", "", "token", {"states": ["focus"]}, status="rule")
    earlier.rule_when = earlier.when
    under = any_fact("token.text.caret_blink", "U12", "token", {"states": ["focus", "hover"]})
    narrower = any_fact("token.behavior.double_click", "", "token", {"states": ["focus", "hover"]}, status="rule")
    narrower.rule_when = narrower.when
    over = any_fact("token.behavior.double_click", "U13", "token", {"states": ["focus"]})
    selected = any_fact("input.selected", "U14", "input", {"conditions": ["contrast_high"]})
    label = any_fact("button.label", "U15", "widget", widget="button")
    icon = any_fact("button.icon", "U16", "widget", widget="button")
    bases.append(("widgets", None, ("button", "label"), ("padding",), {}, label))
    facts = [families, caret, status, drag, later, mode, accent, scheme, earlier, under, narrower, over, selected,
             label, icon]
    got = [(path, uid) for path, uid, _ in pending_table(None, facts, bases, repeat_block)]
    if got != [("colors.dark.global.status", "U7"), ("inputs.accent.light", "U10"), ("inputs.accent.dark", "U10"),
               ("inputs.scheme.dark", "U11"), ("tokens.behavior.double_click", "U13"),
               ("widgets.button.label", "U15")]:
        failures.append(f"pending_table: {got}, expected the dark scheme's status role, the accent input per scheme, "
                        f"the scheme input's dark, the double click under a narrower rule and the label's presence")
    # a part's base `absent = false` (`"present"`, S6.3) is a presence beneath its pending presence, and states none
    # of its fields: the pending rendering mode is listed, and no field is reported as drawn again over it
    drawn = [b if b[5] is not mode else b[:4] + (False, mode) for b in bases]
    got = [(path, uid) for path, uid, _ in pending_table(None, [mode, icon], drawn, repeat_block)]
    if got != [("widgets.button.icon.icon.rendering_mode", "U9")]:
        failures.append(f"pending_table under a drawn part's base: {got}, expected the icon's rendering mode alone")
    present = any_fact("button.icon", "", "widget", {"states": ["checked"]}, widget="button", status="rule")
    present.item.data = {"value": "present"}
    if redrawn_pending([mode, present], drawn):
        failures.append("redrawn_pending: a field reported under a drawn part's base")
    # macOS's icon-only padding: a pending fact under a variant whose elements take every size has one earlier rule
    # per size beneath it at each, so it lists nothing; one under a variant whose entry takes no size, or a model
    # whose variant sets give one of its elements such a variant, is listed
    def sized(when, uid, status=PENDING):
        f = any_fact("button.body.min_width", uid, "widget", when, widget="button", status=status)
        f.rule_when = f.when
        return f

    per_size = [sized({"sizes": [z]}, "", "rule") for z in ("small", "regular")]
    icon_only, route_fact = sized({"variants": ["icon_only"]}, "U18"), sized({"variants": ["route"]}, "U19")
    route_model = {"variants": ["icon_only", "route"], "sizes": ["small", "regular"], "default_size": "regular",
                   "sizes_by_variant": [{"variant": "route", "sizes": []}]}
    for model, want in ((route_model, [("widgets.button.body.min_width", "U19")]),
                        ({**route_model, "variant_sets": [["icon_only", "route"]]},
                         [("widgets.button.body.min_width", ["U18", "U19"])])):
        got = [(path, uid) for path, uid, _ in pending_table(None, per_size + [icon_only, route_fact], [],
                                                             repeat_block, {("button", None): model})]
        if got != want:
            failures.append(f"pending_table over per-size rules ({model.get('variant_sets')}): {got}, expected "
                            f"{want}")
    # macos-monterey's menu button: a model whose variant sets are products, every element of the menu variant
    # taking a size, has the per-size rules beneath its pending fact there (the gate finds no listed path); the
    # route button's glyph: an earlier rule refusing the help and menu variants holds wherever the route variant,
    # exclusive with both, does, so it is beneath that pending fact, and the help variant's is listed
    menu_fact = sized({"variants": ["menu"]}, "U20")
    products = {**route_model, "variants": ["icon_only", "route", "menu", "help", "flat"],
                "exclusive_variants": [["menu", "route", "help", "flat"], ["icon_only", "route"]],
                "variant_products": [[{"any": ["flat", "help"]}, {"any": ["icon_only"]}],
                                     [{"one": ["menu"]}, {"any": ["icon_only"]}]]}
    got = [(path, uid) for path, uid, _ in pending_table(None, per_size + [menu_fact, route_fact], [],
                                                         repeat_block, {("button", None): products})]
    if got != [("widgets.button.body.min_width", "U19")]:
        failures.append(f"pending_table over per-size rules and variant products: {got}, expected the route "
                        f"variant's U19 alone")
    not_help = sized({"not_variants": ["help", "menu"]}, "", "rule")
    help_fact, route_glyph = sized({"variants": ["help"]}, "U21"), sized({"variants": ["route"]}, "U22")
    got = [(path, uid) for path, uid, _ in pending_table(None, [not_help, help_fact, route_glyph], [],
                                                         repeat_block, {("button", None): products})]
    if got != [("widgets.button.body.min_width", "U21")]:
        failures.append(f"pending_table under a rule refusing the variants exclusive with its own: {got}, expected "
                        f"the help variant's U21 alone")
    # a component's pending presence is refused, as the reader refuses its key until components are typed
    try:
        pending_table(None, [any_fact("component.rail.item", "U17", "widget")], [])
        failures.append("pending_table: a component's pending presence listed, expected InputError")
    except InputError:
        pass
    # a pending field of a part its base states absent is reported where a `present` fact draws the part again
    got = [(key, uid) for key, uid, _ in redrawn_pending(facts, bases)]
    if got:
        failures.append(f"redrawn_pending: {got}, expected none without a `present` fact")
    present = any_fact("button.icon", "", "widget", {"states": ["checked"]}, widget="button", status="rule")
    present.item.data = {"value": "present"}
    got = [(key, uid) for key, uid, _ in redrawn_pending(facts + [present], bases)]
    if got != [("button.icon.icon.rendering_mode", "U9")]:
        failures.append(f"redrawn_pending: {got}, expected the icon's rendering mode")
    # the round trip reduces a flavour's combinations by its own state model
    flavored = {("button", None): {"exclusive": []}, ("button", "compact"): {"exclusive": [["pressed"]]}}
    got = [model_of(flavored, "button", {"conditions": ["flavor:compact"]}), model_of(flavored, "button", {}),
           model_of(flavored, "button", {"conditions": ["flavor:other"]}), model_of(flavored, None, {})]
    if got != [{"exclusive": [["pressed"]]}, {"exclusive": []}, {"exclusive": []}, None]:
        failures.append(f"model_of: {got}")
    # off_variants: a variant only skipped facts name is left out (a key not held, a unit not enabled, an accent-run
    # fact naming nothing), one a kept fact names too stays, as does one S12 exempts (requires_variants,
    # sizes_by_variant, an element part's content variant); without_variants trims an exclusive group
    def named(variant, status="rule", reason="", key="button.body.fill", widget="button"):
        return Fact(None, key, tuple(key.split(".")), {"variants": [variant]}, "widget", widget, None, None,
                    status=status, reason=reason)

    skipped = [named("@both", LEFT_OUT, NOT_HELD), named("@both"), named("@unheld", LEFT_OUT, NOT_HELD),
               named("@unit", LEFT_OUT, f"{UNIT_NOT_ENABLED} (toolbar)"), named("@required", LEFT_OUT, NOT_HELD),
               named("@sized", LEFT_OUT, NOT_HELD), named("@content", LEFT_OUT, NOT_HELD),
               named("@accent", LEFT_OUT, ACCENT_RUN), named("@accent", LEFT_OUT, NOT_HELD), named("@pending", PENDING)]
    model = {"variants": ["flat", "@both", "@unheld", "@unit", "@required", "@sized", "@content", "@accent",
                          "@pending"],
             "requires_variants": ["@required"], "sizes_by_variant": [{"variant": "@sized", "sizes": []}]}
    got = off_variants("button", model, skipped, widget_content({"button.icon": ["@content"]}, "button"))
    if got != {"@unheld", "@unit", "@accent"}:
        failures.append(f"off_variants: {sorted(got)}, expected @accent, @unheld and @unit")
    trimmed = without_variants("exclusive_variants", [["flat", "primary", "@unit"], ["@unit", "@unheld"],
                                                      ["flat", "@both"]], {"@unit", "@unheld"})
    if trimmed != [["flat", "primary"], ["flat", "@both"]]:
        failures.append(f"without_variants: {trimmed}")
    # a key whose rules cover every scheme, or every size, gets a base from one of them; one covering less gets none
    def rule_fact(key, when, value):
        f = Fact(None, key, tuple(key.split(".")), when, "widget", "button", None, None, status="rule",
                 section="widgets", header=("button", "body"), dotted=(key.split(".")[-1] + "_px",), value=value)
        f.rule_when = when
        return f

    by_scheme = [rule_fact("button.body.min_height", {"conditions": [f"scheme:{x}"]}, v)
                 for x, v in (("light", 24), ("dark", 25))]
    by_size = [rule_fact("button.body.min_width", {"sizes": [z]}, v) for z, v in (("small", 20), ("regular", 30))]
    partial = [rule_fact("button.body.margin", {"sizes": ["small"]}, 1)]
    sized = {("button", None): {"sizes": ["small", "regular"], "default_size": "regular"}}
    covering, gaps = covering_bases(by_scheme + by_size + partial, sized, {})
    got = [(b[3], b[4]) for b in covering]
    if got != [(("min_height_px",), 24), (("min_width_px",), 30)] or gaps:
        failures.append(f"covering_bases: {got}, {gaps} reported, expected light's min_height and the default size's "
                        f"min_width, no gap")
    # iOS's button: sizes and no `default_size`, so an element stating no size holds none and no fact gives its
    # value: no base written, the gap reported
    unsized = {("button", None): {"sizes": ["small", "regular"]}}
    by_size = [rule_fact("button.body.min_width", {"sizes": [z]}, v) for z, v in (("small", 20), ("regular", 30))]
    covering, gaps = covering_bases(by_size, unsized, {})
    if covering or [g[:2] for g in gaps] != [("button", "button.body.min_width")]:
        failures.append(f"covering_bases without a default size: {[(b[3], b[4]) for b in covering]} written, {gaps} "
                        f"reported, expected nothing written and button.body.min_width's gap")
    # a default size the widget does not take (S12 refuses it): no base, and the gap reported, never nothing
    stray = {("button", None): {"sizes": ["small", "regular"], "default_size": "large"}}
    covering, gaps = covering_bases(by_size, stray, {})
    if covering or [g[:2] for g in gaps] != [("button", "button.body.min_width")]:
        failures.append(f"covering_bases with a stray default size: {[(b[3], b[4]) for b in covering]} written, "
                        f"{gaps} reported, expected nothing written and button.body.min_width's gap")
    # a flavour's state model (material-3's expressive sizes): the base is at rest under every flavour, so a
    # flavour whose default size differs leaves no base and a gap, one agreeing keeps it
    for compact, want_base, want_gap in (("regular", 30, False), ("small", None, True)):
        flavoured = {("button", None): {"sizes": ["small", "regular"], "default_size": "regular"},
                     ("button", "compact"): {"sizes": ["small", "regular"], "default_size": compact}}
        covering, gaps = covering_bases(by_size, flavoured, {})
        got = [b[4] for b in covering]
        if got != ([want_base] if want_base else []) or bool(gaps) != want_gap:
            failures.append(f"covering_bases under a flavour defaulting to {compact}: {got} written, {gaps} reported, "
                            f"expected {want_base} and {'a gap' if want_gap else 'no gap'}")
    # a pending fact naming no size is beneath the sizes' rules where every flavour's elements take a size they
    # state: a flavour taking another size, or one the fact names, decides it
    pending_width = rule_fact("button.body.min_width", {}, None)
    pending_width.status = PENDING
    wide = {("button", None): {"sizes": ["small", "regular"]}, ("button", "wide"): {"sizes": ["small", "large"]}}
    beneath = [ruled_beneath(f, by_size + [f], "light", models_holding(wide, "button", f.when)) for f in (
        pending_width, replace(pending_width, when={"not_conditions": ["flavor:wide"]}),
        replace(pending_width, when={"conditions": ["flavor:wide"]}))]
    if beneath != [False, True, False]:
        failures.append(f"ruled_beneath over each flavour's state model: {beneath}, expected [False, True, False]")
    # macOS's route button: a variant whose entry takes no size draws its elements with none, so the sizes' rules
    # cover them only beside a stated fact under that variant alone; a pending one covers nothing (the field listed,
    # no gap), and none at all is a gap
    route = {("button", None): {"sizes": ["small", "regular"], "default_size": "regular",
                                "sizes_by_variant": [{"variant": "route", "sizes": []}]}}
    pending_route = rule_fact("button.body.min_width", {"variants": ["route"]}, None)
    pending_route.status = PENDING
    for extra, want_base, want_gap in (([rule_fact("button.body.min_width", {"variants": ["route"]}, 12)], 30, False),
                                       ([pending_route], None, False), ([], None, True)):
        covering, gaps = covering_bases(by_size + extra, route, {})
        got = [b[4] for b in covering]
        if got != ([want_base] if want_base else []) or bool(gaps) != want_gap:
            failures.append(f"covering_bases with an unsized variant ({len(extra)} route fact(s), "
                            f"{[f.status for f in extra]}): {got} written, {gaps} reported, expected "
                            f"{want_base} and {'a gap' if want_gap else 'no gap'}")
    # Windows' repeat delay: a variant's fact and its complement's cover every element, the complement's value the
    # base; with the variant's pending, nothing is written; a part's presence takes no base from them
    def pair(key, dotted, value):
        out = [rule_fact(key, {"variants": ["@repeat"]}, 500), rule_fact(key, {"not_variants": ["@repeat"]}, value)]
        for f in out:
            f.dotted = dotted
        return out

    covering, gaps = covering_bases(pair("button.values.repeat_delay", ("repeat_delay_ms",), "absent"), {}, {})
    if [b[4] for b in covering] != ["absent"] or gaps:
        failures.append(f"covering_bases over a variant and its complement: {[b[4] for b in covering]} written, "
                        f"{gaps} reported, expected the complement's absent")
    half = pair("button.values.repeat_delay", ("repeat_delay_ms",), "absent")
    half[0].status = PENDING
    covering, gaps = covering_bases(half, {}, {})
    if covering or gaps:
        failures.append(f"covering_bases over a pending variant's fact and its complement: {covering} written, {gaps} "
                        f"reported, expected nothing")
    covering, gaps = covering_bases(pair("button.menu_indicator", (), True), {}, {})
    if covering or gaps:
        failures.append(f"covering_bases over a presence's variant and complement: {covering} written, expected none")
    for when, d, groups, recency, want in HOLDS_CASES:
        if holds(when, combo(d), groups, recency) != want:
            failures.append(f"holds({when}, {d}): not {want}")
    # the computed-once results: variant_options() follows a model changed in place, `off` and every field it reads,
    # and a caller changing its lists changes no later call's
    model = {"variants": ["flat", "help"]}
    first = variant_options(model)
    first[1].append("help")
    model["variants"].append("icon_only")
    model["exclusive_variants"] = [["flat", "help"]]
    model["variant_products"] = [[{"any": ["flat", "help"]}, {"any": ["icon_only"]}]]
    got = [first, variant_options({"variants": ["flat", "help"]}), variant_options(model),
           variant_options(model, ["help"]), variant_options(model, ("help",)), variant_options({**model, "not_drawn": [
               {"variants": ["icon_only"]}]}), variant_options({**model, "requires_variants": ["help"]})]
    want = [[[], ["flat", "help"], ["help"]], [[], ["flat"], ["help"]],
            [[], ["flat"], ["help"], ["icon_only"], ["flat", "icon_only"], ["help", "icon_only"]],
            [[], ["flat"], ["icon_only"], ["flat", "icon_only"]], [[], ["flat"], ["icon_only"], ["flat", "icon_only"]],
            [[], ["flat"], ["help"]], [["help"], ["help", "icon_only"]]]
    if got != want:
        failures.append(f"variant_options computed once: {got}, expected {want}")
    # Combinations: equal `when`s share their results whatever their tables' key order, unequal ones do not, a
    # `when` given anew (another object at a freed one's id) is keyed by its own content, and every result is
    # minimal()'s and holds()' own
    at, sizes = Combinations({("button", None): {"sizes": ["small", "regular"], "default_size": "regular"}}), []
    for when in ({"states": ["hover"], "sizes": ["small"]}, {"sizes": ["small"], "states": ["hover"]},
                 {"states": ["hover"]}, {}):
        c = at.combination("button", when, "light")
        sizes.append(sorted(c["sizes"]))
        if c != minimal(when, "light", [], {}, {"sizes": ["small", "regular"], "default_size": "regular"}):
            failures.append(f"Combinations.combination({when}): not minimal()'s")
    if sizes != [["small"], ["small"], ["regular"], ["regular"]] or len(at.combinations) != 3:
        failures.append(f"Combinations.combination: sizes {sizes}, {len(at.combinations)} computed, expected 3")
    base = {"states": ["hover", "pressed"]}
    at.combination("button", base, "dark")
    for other, want in (({"states": ["hover"]}, True), ({"states": ["focus"]}, False), ({"not_states": ["hover"]},
                                                                                         False), ({}, True)):
        for _ in range(2):
            if at.holds("button", base, "dark", dict(other)) != want:
                failures.append(f"Combinations.holds({other}) in {base}: not {want}")
    if at.combination("button", {"sizes": ["small", "regular"]}, "light") is not None:
        failures.append("Combinations.combination of two sizes: an element holds it")
    # the model's fields a preset leaves unstated: none where it states each, the field where one line is gone; the
    # sources' tests, comments and attributes are no model
    with tempfile.TemporaryDirectory() as tmp:
        write(Path(tmp), MODEL_FILES)
        try:
            model = read_model(Path(tmp))
            got = (sorted(model.widgets["button"][0].items()), model.widgets["button"][1], model.structs["Sides"][0],
                   unstated_fields(model, FIELDS_PRESET))
        except (KeyError, InputError) as err:
            got = repr(err)
        want = ([("body", ["fill", "padding"]), ("icon", ["icon"]), ("mark", ["fill"])], "ButtonValues", ("T",), [])
        if got != want:
            failures.append(f"read_model and unstated_fields over the fixture model: {got}, expected {want}")
        for line, want in FIELDS_CASES if got == want else []:
            text = FIELDS_PRESET.replace(line + "\n", "")
            got = unstated_fields(model, text) if text != FIELDS_PRESET else f"no line {line!r}"
            if got != want:
                failures.append(f"unstated_fields without {line!r}: {got}, expected {want}")
        if field_lines(Model({}, {}, set(), Path("src")), None, "") != [
                "# unstated model fields: no model sources in src"]:
            failures.append("field_lines without a model: not said")
    # each unstated path grouped by the facts there: none (the accent run's counting none), conditional ones, one at
    # rest that gives no base
    def near(key, kind, when=None, status="rule", reason="", **data):
        f = Fact(_Item("") if "pending" in data else None, key, tuple(key.split(".")), when or {}, kind, None, None,
                 None, status=status, reason=reason)
        if f.item is None:
            f.item = _Item("")
            f.item.data = {"value": 1, **data}
        else:
            f.item.data = data
        return f

    facts = [near("token.focus.width", "token", {"states": ["hover"]}),
             near("role.global.accent", "role", status=LEFT_OUT, reason=READ),
             near("role.global.scrim", "role", status=LEFT_OUT, reason=ACCENT_RUN, scheme_file="kde x", accent="#000"),
             near("token.text.keyed.title.weight", "token", status=PENDING, pending="U1")]
    got = field_groups(facts, [("tokens", "focus", "width"), ("colors", "dark", "global", "accent"),
                               ("colors", "light", "global", "scrim"), ("tokens", "text", "keyed")])
    want = [(ONLY_CONDITIONAL, ("tokens", "focus", "width"), 1, 0, {}),
            (BASE_EXISTS, ("colors", "dark", "global", "accent"), 1, 0, {f"{LEFT_OUT}: {READ}": 1}),
            (NO_FACT, ("colors", "light", "global", "scrim"), 0, 0, {}),
            (BASE_EXISTS, ("tokens", "text", "keyed"), 0, 1, {PENDING: 1})]
    if got != want:
        failures.append(f"field_groups: {got}, expected {want}")
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp) / "fixture"
        write(root, SELF_TEST_FILES)
        ctx = fixture_context(root)
        draft, text, trip, units = generate(ctx, "adwaita")
        # a fact set and its expected preset, byte for byte, its digest that of its generated lines
        expected = SELF_TEST_EXPECTED.replace("<digest>", digest(hand_parts(SELF_TEST_EXPECTED)[1]))
        if text != expected:
            import difflib
            diff = "".join(difflib.unified_diff(expected.splitlines(True), text.splitlines(True),
                                                "expected", "generated"))
            failures.append(f"the fixture's draft differs:\n{diff}")
        failures += hand_written_cases(text)
        # a whole `Value` struct block in a rule read back without `replace` is a mismatch (S2.2, S11)
        whole = 'glyph = { replace = { source = { named_system_image = { name = "checkmark" } }, stroke = "absent" } }'
        bare = 'glyph = { source = { named_system_image = { name = "checkmark" } }, stroke = "absent" }'
        if not any("without `replace`" in m for m in round_trip(draft, text.replace(whole, bare)).mismatches):
            failures.append("round trip: a whole block in a rule without `replace` reads back as the fact's")
        # a pending fact left out, an untranscribable fact reported, each by its reason
        status = {(f.key, repr(f.item.data.get("value", f.item.data.get("pending")))): (f.status, f.reason)
                  for f in draft.facts}
        for key, value, want in SELF_TEST_STATUS:
            if status.get((key, repr(value))) != want:
                failures.append(f"{key} = {value!r}: {status.get((key, repr(value)))}, expected {want}")
        lines = share_lines(draft, units, trip)
        for needle in ("2 pending", "pending (register id: facts): U1: 1, U8: 1",
                       "[pending]: 1 listed paths, pending with nothing beneath: tokens.text.fallbacks.families (U8)", f"{READ} 2", f"{NOT_HELD} 3",
                       "23 rules", "the `read` facts as bases, in 2 tables, and in 1 rules of their own", f"{UNIT_NOT_ENABLED} (toolbar) 2", "[declare] left out: toolbar"):
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
        if [(h, [k for k, _ in kvs]) for h, kvs, _, _ in draft.declare] != [
                (("widget", "button"), ["available", "variants", "sizes", "densities", "flags", "exclusive",
                                        "exclusive_variants", "recency", "data"])]:
            failures.append(f"[declare]: {[(h, kvs) for h, kvs, _, _ in draft.declare]}")
        # the element data the transcribed facts read, per part: a root input and a left-out fact's need none
        data = dict(draft.declare[0][1]).get("data") if draft.declare else None
        if data != {"body": ["font_underline_thickness", "widget.label.font_height"], "values": ["width"]}:
            failures.append(f"[declare] data: {data}")
        # a variant only the toolbar's left-out fact names is left out of [declare], every other kept
        if draft.dropped != [("button", ["@raised"])]:
            failures.append(f"[declare]: variants left out {draft.dropped}, expected only button's @raised")
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
        # iOS's pattern end to end: a key stated per size alone, one variant's rule over it, and no `default_size`:
        # no base is written (no fact gives an element with no size its value) and the gap is reported; with a
        # `default_size` the default size's value is the base, which never shows
        model = SELF_TEST_FILES["docs/facts/gnome/state-models.toml"]
        sizes = "sizes = []\ndensities = []\nflags = [\"@busy\"]"
        sized_facts = SELF_TEST_FILES["docs/facts/gnome/button.toml"] + "".join([
            _fact("button.body.opacity", "0.5", '{ sizes = ["small"] }'),
            _fact("button.body.opacity", "0.75", '{ sizes = ["regular"] }'),
            _fact("button.body.opacity", "0.25", '{ variants = ["flat"] }')])
        for default, shows in (("", True), ('default_size = "regular"\n', False)):
            root = Path(tmp) / f"sized{int(shows)}"
            sized_model = model.replace(sizes, f'sizes = ["small", "regular"]\n{default}' + sizes[len("sizes = []\n"):],
                                        1)
            write(root, {**SELF_TEST_FILES, "docs/facts/gnome/state-models.toml": sized_model,
                         "docs/facts/gnome/button.toml": sized_facts})
            sized_draft, sized_text, sized_trip, sized_units = generate(fixture_context(root), "adwaita")
            want = [("button", "button.body.opacity")] if shows else []
            if sized_trip.mismatches or [g[:2] for g in sized_draft.gaps] != want:
                failures.append(f"a key per size, default size {not shows}: {sized_draft.gaps} reported, round trip "
                                f"{sized_trip}")
            lines = share_lines(sized_draft, sized_units, sized_trip)
            gap = ("# facts gap: button.body.opacity: `button` states sizes and no `default_size`: no fact gives the "
                   "value of an element with no size")
            if (gap in lines) != shows:
                failures.append(f"a key per size, default size {not shows}: the report's gap line: {lines}")
            base = sized_text.split("\n[[rules]]\n")[0]
            if ("opacity = " in base) == shows:
                failures.append(f"a key per size, default size {not shows}: the base table states opacity "
                                f"{not shows}")
            if shows:
                # the size's value written as a base is a value no fact gives where an element states no size
                invented = base.replace("[widgets.button.body]\n", "[widgets.button.body]\nopacity = 0.5\n", 1)
                if invented == base or not round_trip(sized_draft, invented + sized_text[len(base):]).mismatches:
                    failures.append("a key per size, no default size: a base from a size's fact passes")
        # a pending default size end to end: listed in [pending] by its declaration's path, stated nowhere in
        # [declare], the round trip reading the table back
        root = Path(tmp) / "pending_default"
        pending_model = model.replace(sizes, 'sizes = ["small", "regular"]\ndefault_size = { pending = "U9" }\n'
                                      + sizes[len("sizes = []\n"):], 1)
        write(root, {**SELF_TEST_FILES, "docs/facts/gnome/state-models.toml": pending_model,
                     "docs/facts/gnome/button.toml": sized_facts})
        pending_draft, pending_text, pending_trip, _ = generate(fixture_context(root), "adwaita")
        declared = pending_text.split("[declare.widget.button]\n", 1)[-1].split("\n\n", 1)[0]
        if (pending_trip.mismatches or '"declare.widget.button.default_size" = "U9"' not in pending_text
                or "default_size" in declared):
            failures.append(f"a pending default size: {pending_trip}, [declare.widget.button] {declared!r}")
        # a part drawn again over its absence (S17: `value = "present"`, S6.3's `absent = false`): a rule of its own,
        # read back as `present`
        root = Path(tmp) / "present"
        write(root, {**SELF_TEST_FILES, "docs/facts/gnome/button.toml": SELF_TEST_FILES["docs/facts/gnome/button.toml"]
                     + _fact("button.mark", '"present"', '{ variants = ["@pill"], states = ["hover"] }')})
        present_draft, present_text, present_trip, _ = generate(fixture_context(root), "adwaita")
        redrawn = [(f.status, f.reason) for f in present_draft.facts if f.item.data.get("value") == "present"]
        rule = ('[[rules]]\nwhen = { variants = ["@pill"], states = ["hover"] }\ntarget = "button.mark"\n\n'
                '[rules.set]\n'
                f'# docs/facts/gnome/button.toml:{SELF_TEST_FILES["docs/facts/gnome/button.toml"].count(chr(10)) + 2}\n'
                'absent = false\n')
        if present_trip.mismatches or redrawn != [("rule", "")] or rule not in present_text:
            failures.append(f"a `present` fact: {redrawn}, round trip {present_trip}, its rule written "
                            f"{rule in present_text}")
        # a keyed list (S6.1, S17): a rule's transitions merge over the base's by key, so a fact that leaves out an
        # item of the base mismatches, the merge keeping it, and one stating the merged list, in either order of the
        # items it restates, passes
        def transition(group, ms):
            return (f'{{ properties = ["{group}"], kind = "interpolate", duration_ms = {ms}, delay_ms = 0, '
                    f'timing = "linear", engine = "gtk_css" }}')

        listed = SELF_TEST_FILES["docs/facts/gnome/button.toml"].replace(_fact("button.body.transitions", "[]"), _fact(
            "button.body.transitions", f'[{transition("fill", 100)}, {transition("stroke", 200)}]'), 1)
        for n, (rule_list, mismatched) in enumerate((
                (f'[{transition("fill", 50)}]', True),
                (f'[{transition("stroke", 200)}, {transition("fill", 50)}]', False),
                (f'[{transition("fill", 50)}, {transition("stroke", 200)}]', False))):
            root = Path(tmp) / f"keyed{n}"
            write(root, {**SELF_TEST_FILES, "docs/facts/gnome/button.toml": listed + _fact(
                "button.body.transitions", rule_list, '{ states = ["hover"] }')})
            keyed_trip = generate(fixture_context(root), "adwaita")[2]
            found = [m for m in keyed_trip.mismatches if m.startswith("button.body.transitions in ")]
            if bool(found) != mismatched or len(found) != len(keyed_trip.mismatches):
                failures.append(f"a keyed list's rule {rule_list}: mismatches {keyed_trip.mismatches}, expected "
                                f"{'one' if mismatched else 'none'}")
        merges = [
            ("a transition split per group, `from` sets compared as sets, a stated zero keyed with no test",
             [{"properties": ["fill", "stroke"], "from": ["hover", "pressed"], "duration_ms": 0,
               "engine": {"breeze": {"any": ["hover"], "none": [], "conditions": []}}}],
             [{"properties": ["stroke"], "from": ["pressed", "hover"], "duration_ms": 0, "engine": "gtk_css"}],
             transition_key,
             [{"properties": ["fill"], "from": ["hover", "pressed"], "duration_ms": 0,
               "engine": {"breeze": {"any": ["hover"], "none": [], "conditions": []}}},
              {"properties": ["stroke"], "from": ["pressed", "hover"], "duration_ms": 0, "engine": "gtk_css"}]),
            ("a Breeze test is a key of its own, `from` left out the same as \"none\"",
             [{"properties": ["fill"], "duration_ms": 10, "engine": {"breeze": {"any": ["hover"]}}}],
             [{"properties": ["fill"], "from": "none", "duration_ms": 10, "engine": {"breeze": {"any": ["pressed"]}}}],
             transition_key,
             [{"properties": ["fill"], "duration_ms": 10, "engine": {"breeze": {"any": ["hover"]}}},
              {"properties": ["fill"], "from": "none", "duration_ms": 10, "engine": {"breeze": {"any": ["pressed"]}}}]),
            ("a surface binding replaces a surface binding, a role binding its role's",
             [{"surface": "card"}, {"role": {"role": {"surface": "foreground"}, "value": "#000000"}}],
             [{"role": {"role": {"surface": "foreground"}, "value": "#ffffff"}}, {"surface": "view"}],
             binding_key,
             [{"role": {"role": {"surface": "foreground"}, "value": "#ffffff"}}, {"surface": "view"}]),
            ("a presentation by its event", [{"event": "show", "steps": [1]}, {"event": "hide", "steps": [2]}],
             [{"event": "show", "steps": [3]}], presentation_key,
             [{"event": "hide", "steps": [2]}, {"event": "show", "steps": [3]}]),
        ]
        for what, earlier, later, item_key, want in merges:
            if normal(keyed_merge(earlier, later, item_key)) != normal(want):
                failures.append(f"keyed_merge: {what}: {keyed_merge(earlier, later, item_key)}")
        # --write writes nothing where the round trip finds a mismatch (a generator bug, forced here) and says so,
        # over a committed preset the new text differs from; it writes the preset where it finds none
        root = Path(tmp) / "mismatch"
        button = SELF_TEST_FILES["docs/facts/gnome/button.toml"]
        write(root, {**SELF_TEST_FILES, "docs/facts/gnome/button.toml": button.replace('"16px"', '"18px"', 1)})
        written = root / "native-theme/src/presets/adwaita.toml"
        written.parent.mkdir(parents=True)
        with contextlib.redirect_stderr(io.StringIO()):
            code = run(fixture_context(root), "adwaita", "--write", None)
        stale = written.read_text(encoding="utf-8") if written.exists() else None
        if code != 0 or stale is None:
            failures.append(f"--write of the committed preset: exit {code}, written {stale is not None}")
        write(root, SELF_TEST_FILES)
        honest = round_trip

        def mismatched(draft, text):
            return Trip(compared=honest(draft, text).compared, mismatches=["forced"])

        globals()["round_trip"] = mismatched
        try:
            with contextlib.redirect_stderr(io.StringIO()) as err:
                code = run(fixture_context(root), "adwaita", "--write", None)
        finally:
            globals()["round_trip"] = honest
        said = err.getvalue()
        if (code != 1 or written.read_text(encoding="utf-8") != stale or "round trip: forced" not in said
                or "adwaita.toml not written: the round trip finds a mismatch" not in said or "--write`" in said):
            failures.append(f"--write over a mismatched round trip: exit {code}, rewritten "
                            f"{written.read_text(encoding='utf-8') != stale}, said {said!r}")
        with contextlib.redirect_stderr(io.StringIO()):
            code = run(fixture_context(root), "adwaita", "--write", None)
        if code != 0 or written.read_text(encoding="utf-8") == stale:
            failures.append(f"--write over an exact round trip: exit {code}, "
                            f"written {written.read_text(encoding='utf-8') != stale}")
        failures += citation_cases(Path(tmp) / "cites")
        failures += naming_cases()
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
    preset, out, mode = args.pop(0), None, None
    while args:
        a = args.pop(0)
        if a == "--out" and args and mode is None:
            out, mode = Path(args.pop(0)), "out"
        elif a in ("--write", "--check") and mode is None:
            mode = a
        else:
            print(__doc__, file=sys.stderr)
            return 2
    try:
        ctx = check_facts.Context(Path("."))
    except (OSError, UnicodeDecodeError, InputError, check_facts.InputError, check_facts_trace.InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    return run(ctx, preset, mode, out)


def run(ctx, preset: str, mode, out) -> int:
    """main() over the facts `ctx` reads: the preset generated and, by `mode`, printed, written to `out`, written
    over the bundled preset (`--write`; nothing where the round trip finds a mismatch) or compared with it
    (`--check`); its exit code."""
    unread = None
    try:
        draft, text, trip, units = generate(ctx, preset)
        path = ctx.root / check_facts.PRESET_DIR / f"{preset}.toml"
        committed = path.read_text(encoding="utf-8") if path.exists() else None
        cites = None
        if mode in ("--write", "--check"):
            cites = Citations(draft, (ctx.root / SOURCES).read_text(encoding="utf-8"))
            text = carried(text, committed, cites.refresh)
            written = text
        else:
            try:
                written = carried(text, committed)
            except InputError as err:
                written, unread = text, err
        model = read_model(ctx.root)
        fields = field_lines(model, draft, written)
    except (OSError, UnicodeDecodeError, InputError, check_facts.InputError, check_facts_trace.InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    if mode is None:
        sys.stdout.write(text)
    elif mode == "out":
        out.write_text(text, encoding="utf-8")
    elif mode == "--write":
        if not trip.mismatches:
            path.write_text(text, encoding="utf-8")
    elif text != committed:
        for line in cites.refreshed:
            print(f"stale citation: {line}", file=sys.stderr)
        for line in cites.findings:
            print(f"citation: {line}", file=sys.stderr)
        print(f"{check_facts.PRESET_DIR / preset}.toml is not what --write writes from the facts: run "
              f"`python3 scripts/generate_preset.py {preset} --write`", file=sys.stderr)
        return 1
    for line in share_lines(draft, units, trip):
        print(line, file=sys.stderr)
    if unread is not None:
        print(f"# the bundled preset's hand-written parts not read for the fields below: {unread}", file=sys.stderr)
    for line in fields:
        print(line, file=sys.stderr)
    for m in trip.mismatches:
        print(f"round trip: {m}", file=sys.stderr)
    if mode == "--write" and trip.mismatches:
        print(f"{check_facts.PRESET_DIR / preset}.toml not written: the round trip finds a mismatch", file=sys.stderr)
    if cites is not None and mode == "--write" and cites.refreshed:
        print(f"# hand-written citations refreshed: {len(cites.refreshed)}", file=sys.stderr)
    for line in cites.findings if cites is not None else []:
        print(f"citation: {line}", file=sys.stderr)
    return 1 if trip.mismatches or (cites is not None and cites.findings) else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
