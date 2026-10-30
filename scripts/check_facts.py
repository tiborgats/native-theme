#!/usr/bin/env python3
"""The facts' form (specification S17; the 0.8.0 plan's M0, "The facts checkers early"). Offline.

Reads every fact file `docs/facts/<platform>/*.toml` (the platform directories below, and `palettes/`), each
platform's `state-models.toml`, `docs/facts/sources.toml`, the core (its Rust sketches, as
scripts/check_spec_names.py reads them, and its function table) and the widget catalogue (through
scripts/spec_catalogue.py). `docs/facts/imported/` is the plan's lane input and `docs/facts/staged/` holds S19.2's
staged inheritance bases: neither is facts, and both are skipped.
scripts/check_fact_citations.py checks what the citations say; this script checks how the files are written.

Run from the repository root:

    python3 scripts/check_facts.py
    python3 scripts/check_facts.py --self-test

It prints one line per finding, `<file>:<line>: <what>`, and on stderr a `# variant sets:` line per state-model
entry stating `variant_sets` or `variant_products` (its widget's number of sets, below), a `# recency orders:` line
per entry stating `recency` (the gate's runs per order, below) and a summary line with the counts. Exit 0
when there is none, 1 when there are some, 2 on an input it cannot read (sources.toml, the core, the catalogue) or a
wrong command line. A fact file that is no valid TOML is a finding of that file.

Directories (S17 *Generations*): PLATFORMS maps each directory to the presets it covers: gnome (adwaita), kde
(kde-breeze), macos (macos-monterey, macos-tahoe), ios (ios-13, ios-26), windows (windows-11), material
(material-3). `palettes/` holds palette facts, `imported/` and `staged/` are skipped, any other directory or a
stray file is a finding (a README.md is allowed anywhere).

A `[[fact]]` (S17 *Fields*):

- `key`, `value`, `rows` and at least one `[[fact.cite]]` are required (a pending fact: below); `when` is optional;
  `scheme_file` and `accent` come both or neither, in `kde/` only (`scheme_file = "<sources.toml id> <path>"`,
  `accent` a colour literal). Any other field is a finding.
- `key` is a field path without TOML's unit suffix (a segment ending in `_px`, `_ms`, `_deg` or `_pt` is a finding):
  `<widget>.<part>.<field>[.<leaf>…]` and `<widget>.values.<name>[.<leaf>…]`, `component.<key>.<part>.<field>…`
  and `component.<key>.values.<name>…`, where the widget or component is a catalogue entry, the part one of its
  parts, the field one of that part's fields (the catalogue's list) and the leaves the core's sub-fields
  (check_spec_names.Checker.field_ok), the value one of the entry's widget values (the catalogue's name without its
  unit suffix; a value whose type is not a `Value<…>` may name leaves, unchecked); `role.<set>.<role>`,
  `role.<set>.disabled.<role>`, `role.global.<path>`, `role.@<key>` (Checker.role_ok); `token.<path>`
  (Checker.token_ok), a duration or timing `token.motion.durations.<key>` or `token.motion.timings.<key>` (or the
  whole map), stated whole; an accent ramp role `role.global.accent_ramp.<key>` (`GlobalRole::AccentRamp(Key)`, S3;
  ADD-A122) or the whole ramp; `input.<path>`, where `input.kde_scheme.<set>.<key>`,
  `.<set>.inactive.<key>`, `.wm.<key>`, `.groups`, `.inactive_groups`, `.unset_keys` and `input.inactive.<set>.<key>`
  are checked against the core's surface sets, and any other input name only for its form: the presets declare their
  inputs (S9.1, S10), and the registry (S18) will hold them. A key is read as a TOML dotted key (key_segments): an
  entry of a map keyed by text (TEXT_MAP: a `Vec<(String, …)>` or `Vec<(LanguageTag, …)>`, `per_family`,
  `per_language`) is its text as TOML writes it, bare where TOML allows (`Cantarell`, `zh-Hans`) and quoted, `"…"` or
  `'…'`, where it does not (`token.text.fallbacks.per_family."Segoe Fluent Icons"`), a quoted segment being one
  segment whatever it holds (S17, S11; ADD-A123); an unterminated quote, an empty segment, a quote inside a bare
  segment, a quoted segment that is no such entry and an entry TOML cannot write bare are findings. Two keys equal as
  TOML keys are one key (canonical_key). `input.<part>…` and `input.values…` are the `input`
  widget's. The registry replaces these sources once it is generated (S18). A whole part's key, `<widget>.<part>` or
  `component.<key>.<part>` (S17, ADD-A121), takes `"absent"` (`PartPatch::Absent`) or `{ drawn_as = { widget, part,
  variants } }` (`PartPatch::DrawnAs`, S11's `drawn_as` table: `widget` a catalogue entry, `part` one of its parts,
  `variants` its variants), nothing else. A key never goes inside one of the core's enums (enum_inside): an enum
  block is stated whole, by a reference or S11's spelling of its variant (S17, S6.1; ADD-A122).
- S4.1's motion keys (ADD-A122; motion_key_problems), where the checker can tell: a duration's or timing's key keeps
  no word of the closed list MOTION_WORDS (`animation`, `duration`, `time`, `ms`, `transition`, `timing`, `curve`,
  `easing`, `spring`, each also in its plural) or MOTION_PAIRS (`key_spline`, `cubic_bezier`) unless it is made only
  of them (Breeze's `animations`), and starts with no toolkit prefix of MOTION_PREFIXES (`adw`, `gtk`, `q`, `k`, `ns`,
  `ui`, `c`, `md_sys_motion`, `units`); a duration key of a platform whose row of the catalogue's
  *token.motion.durations* table lists every key (motion_duration_keys: the rows naming no family, DURATION_FAMILY:
  `by family`, `one entry per`; DURATION_ROWS maps each row to its directory) is one of the row's backticked names.
  The catalogue's timings are prose with families, so a timing key is checked by the words and prefixes only.
- `value` is spelled as S11's Spelling table spells a preset value, with the key's own unit in the value (S17): a
  TOML number, a TOML bool, `"auto"`, `"absent"`, `"none"`, a colour literal (`#rrggbb`, `#rrggbbaa`,
  `srgb(r, g, b[, a])`, `display_p3(r, g, b[, a])`), a length `"<n>px"`, a duration `"<n>ms"`, an angle
  `"<n>deg"`, a font size `"<n>pt"` or `"<n>px"`, a name (an enum variant in snake_case, or `@<key>`), an expression
  `"= …"` (S2.2's grammar: literals, references, calls of the core's function table with the arity its signature
  gives, `alpha(c, a)` and `with_alpha(c, a)` two arguments, `pick(index, c0, c1, …)` three or more (core_functions;
  ADD-A123), `active(<ref>)`, `srgb` and `display_p3` as literal forms; role, token, `surface.` and `element.`
  references are checked by name), a list, or a table (a block, or a one-key table for a variant with data) whose
  keys are snake_case and whose suffixed keys (`_px`, `_ms`, `_deg`, `_pt`) take a plain number or an expression,
  `"auto"`, `"absent"` or `"none"`. A number written as a string is a finding. Where the core's sketches type the key (PartStyle and its blocks, the widget
  value's type, Tokens, the colour roles), the value must be of that dimension (KEY_DIMS). Where they type it, or a
  leaf of it, as one of the core's enums (core_enums), a name is a unit variant's snake_case name, `@<key>` where a
  variant holds a `Key`, or a name of the enum a one-field tuple variant holds (enum_name_ok); a table is a one-key
  table naming a variant with data (S11), and the payload table of a variant with named fields states only those
  fields (`timing = { spring = { … } }` names `Timing::Spring`'s, its `damping_ratio` included, S2.7). A platform
  curve names a member of the core's closed enums (S2.7; ADD-A123): `{ platform = { qt = "<QtEasing>" } }`,
  `{ platform = { adw = "<AdwEasing>" } }` or `{ platform = "kirigami_velocity" }`, so a curve outside `QtEasing` or
  `AdwEasing` is a finding (whether the quote names it is the verifier's, S17). Where the key's type is such an enum
  and no variant of it holds one scalar (literal_enum: a `Timing`, as S4.1's sketch types `token.motion.timings.<key>`;
  not a `Radius`, whose `Px` a length spells), a number, a bool, a list, a colour or a unit literal is a finding. An
  expression's `surface.` reference is a `SurfaceRole` path, `surface.<role>` or `surface.disabled.<role>` (S2.2).
- a text value (S11's text row; ADD-A120): where the key's type is `String` (a `Value<String>`: a font family, an
  icon theme), any TOML string is the text, except `"auto"`, `"absent"`, `"none"` and an expression, which must be
  one reference (S12 refuses a call) and, where it is a `token.` or `role.` path the sketches type, a text field's;
  a number, bool, list or table is a finding. The key's type is followed into the value (text_leaves): a
  `Vec<String>`'s entries (a family list) and a plain `String` inside a table, a struct's field or an enum variant's
  payload as S11 spells it (`{ theme_icon = { name = "go-next-symbolic" } }`: the core's enums, core_enums), are
  always text. A free string where the type is not text is a finding. A text value's measured citation has
  tolerance 0.
- `rows` lists gap-map row IDs as the gap maps write them (`†` included; ROW_RX): each must be a row of a gap map
  (`docs/todo_v0.7.1_model-completeness-gap-map-<platform>.md`), and none twice. `[]` is allowed (a combination no
  row names).
- `when` is S11's: the keys WHEN_KEYS; conditions by S7.1's names (Checker.cond_ok), states (StateFlag, SharedFlag,
  `@<key>`), variants of the key's widget, sizes (ControlSize or `@<key>`), densities (Density or `@<key>`), size
  classes, `direction` (`ltr`, `rtl`), `context`/`not_context` entries (CONTEXT_KEYS; `scope` `"any"` or
  `{ nearest = [<widgets>] }`; `part` only with `widget`), `position` (POSITION_KEYS), `latest` (S11's
  `latest = { <set> = "<flag>" }`, `Selector.latest`, S7.2; ADD-A124: a non-empty table, each name a key (S1's key
  grammar, KEY_RX) and each flag a state flag that the `when`'s own `not_states` does not refuse, as `latest` implies
  the flag; latest_problems: in every state-model entry of the key's widget the fact can hold under
  (applicable_entries: the entries whose `preset:`/`flavor:` conditions the `when`'s include, if any, else the shared
  entries and those the `when` does not refuse), the set is one of the entry's `recency` sets and the flag a member
  of it, and some such entry exists). A colour-role, token or input key's `when` has conditions only (S7: a `Colors`
  or `Tokens` rule may only have conditions). A `preset:` condition names a preset of the fact's directory.
- A transition (transition_problems: every `Transition` table the key's type reaches in the value; S2.7, S11, S12;
  ADD-A124): its `from` is stated in a single transition (`"none"` for any state) and may be left out of a list's
  entry only (`None`); as a list it is not empty, names no flag twice and no keyed flag the key's widget does not
  declare (its names are StateFlag's by the type rule); a transition naming `state_layer` is on a part whose
  catalogue fields hold `interaction`. `PartStyle.interaction`'s keys (`interaction.state_layer.<field>`,
  `interaction.ripple.<field>`, S2.12) are field paths as any (Checker.field_ok), their values typed by the core's
  `Interaction`, `StateLayerStyle` and `RippleStyle` (`ripple.effect` a `RippleEffect`).
- `Radius::Ellipse` (ellipse_problems; S2.4, S12; ADD-A124), where the facts show it: the corners one `Corners` value
  holds (the Radius fields of the core's `Corners`), as the leaf facts `….corners.<corner>` or one whole `….corners`
  table under one `when` (equal as S11 reads it, canonical_when) in one file and run state, are `"ellipse"` on all
  four or on none, and a `….corners.curve` beside them is not `continuous`. A corner stated under another `when` is
  not this `when`'s, so an Ellipse whose other corners only a broader fact states is a finding.
- each citation (`[[fact.cite]]`) by `kind`: `published` requires `source`, `quote`, may have `conversion`, `absent`;
  `read` requires `source`, `quote`, may have `conversion`; `measured` requires `evidence`, `tolerance`, may have
  `conversion`, `inputs`; nothing else. `inputs` (S17 *A measured derivation*) is on a measured citation of an
  expression value (`"= …"`) only, and such a citation always has it, without `conversion`: a non-empty list of the
  expression's references as written there, none twice (check_fact_citations.py checks its evidence passes). `source` is
  `<id> <path>:<line or l1-l2>` or `<id> #<JSON pointer>`, its id a `[[source]]` of sources.toml; `quote` a non-empty
  string; `conversion` a list of check_fact_citations.CONVERSIONS' names; `absent` a list of non-empty names, `[]` where
  the platform has no name for what it lacks (S17 *Absence*, ADD-A120); `tolerance` a number >= 0, or a table from a
  leaf path to such a number; `evidence` `docs/evidence/<harness>/<YYYY-MM-DD>-run-<id>/<image>/<file>#<path>`. A fact
  with a `read` citation states a plain `= input.…` reference (S17 *`read`*).
- A pending fact (S17 *A pending measurement*, ADD-A122): `pending = "U<n>"` in place of `value` (exactly one of
  them), no `[[fact.cite]]`; `key`, `pending` and `rows` required, `when`, `scheme_file` and `accent` as a fact's; the
  id `U` and a number (PENDING_RX), an Id of the ledger's *Unmeasured register* (scripts/check_ledger.py checks that
  it names a row). Its key, `rows` and `when` are checked as a fact's; the generation rule asks nothing of it. A
  pending fact has no source position and is exempt from source order, which no checker reads (the verifier's), but
  not from *Which fact applies* (pending_problems): a pending fact that a later fact of its key in its file and of its
  run (the same `scheme_file` and `accent`, or neither) shadows on a combination both `when`s may hold in is a
  finding. Two `when`s cannot hold together (whens_overlap) where one requires a name the other refuses; they name two
  sizes, densities, directions or size classes on one axis, or two values of a one-valued condition (`scheme:`,
  `preset:`, `flavor:`, `idiom:`, `input:`, `posture:`, `device_model:`, one container's `container_size:`); their
  positions differ in `first`, `last`, `only` or `parity`; one requires a context the other refuses; or, by the
  widget's state-model entries of the platform (any of them), they name two states of one exclusive group, two
  variants of one `exclusive_variants` group, or two or more variants no listed or product set holds. It is
  order-aware (S17: a fact under `when.latest` holds only under the orders it names, a fact without it under every
  order; ADD-A124): two `when`s cannot hold together where they name two latest flags of one recency set, where a
  latest flag is refused by `not_states` (`latest` implies its flag), or where no one order of the flags can make
  every latest flag its set's latest while every flag either names is present (orders_possible: each latest flag set
  after the other present members of its set, those every entry gives it, without a cycle). Anything else
  may hold together (`not_drawn` is not read: a shadowing there is a finding to narrow).
- The generation rule (S17 *Generations*): a fact with `preset:<id>` in `when.conditions` has every citation serving
  that generation; a fact without one has citations that together serve each generation of its directory among the
  presets that exist (existing_presets(); a `preset:` in `not_conditions` takes that generation out). A citation's
  generations: its source entry's `generations`; a measured citation's, the preset its run's `run.toml` maps its image
  to (S17 *`evidence`*; check_fact_citations.py checks the rest of run.toml).

A `[[model]]` in `<platform>/state-models.toml` (S17 *State models*): `widget` (a catalogue entry: a typed widget or
`component.<key>`), `available` (a bool), `rows` and at least one `[[model.cite]]` (as a fact's) are required; `when`
may only hold `conditions` with `preset:` or `flavor:` ones. With `available = false` no other declaration. With
`true`, `variants`, `sizes`, `densities`, `axes` (`{ states, conditions, contexts }`), `exclusive` and `not_drawn`
are required, and `default_size`, `flags`, `requires_variants`, `sizes_by_variant`, `sizes_by_context`,
`exclusive_variants`, `variant_sets`, `variant_products`, `default_size_by_variant` and `recency` optional (S10's
declarations, S17; absent meaning none); names are checked against the catalogue's entry and the core (variants, flags, sizes, densities, states,
conditions, context selectors as `when`'s, each `not_drawn` a `when`), every `exclusive` state is in `axes.states`
and in one group only, `default_size` is one of a non-empty `sizes`, `requires_variants` and `sizes_by_variant`'s
variants are among `variants`, every listed size among `sizes`, a `sizes_by_context` context a catalogue widget.
ADD-A121 (variant_declaration_problems): each `exclusive_variants` group names variants among `variants`, none twice
(a variant may be in several groups, S12); each `variant_sets` set holds two or more variants among `variants`, none
twice, at most one of each `exclusive_variants` group, and no set is listed twice (in any order); each
`default_size_by_variant` entry is `{ variant, size }`, its variant among `variants`, its size one that variant takes
(its `sizes_by_variant` entry's sizes, else `sizes`), and no two entries name one variant (S12). ADD-A122
(product_problems): `variant_products` is a list of products, each a non-empty list of groups `{ any = [...] }` (at
most one of its variants, or none) or `{ one = [...] }` (exactly one), each naming variants among `variants`, no
product naming one variant in two of its groups (or twice in one); its sets (product_sets) are every choice from each
group, those of two or more variants, less those holding two variants of an `exclusive_variants` group and those
`requires_variants` excludes (a set holding none of the variants it lists: any one of them keeps it, S10,
ADD-A123), a set two products give being one set; a listed `variant_sets` set that a product gives is a finding (each
set is stated once), and the exclusive groups check the listed sets only. The number of sets per entry
(drawn_variant_sets: the listed and the products' sets, less those a `not_drawn` entry stating only `variants` and
`not_variants` removes whole and the listed sets `requires_variants` excludes, each counted once; a listed set it
excludes is no finding, as S17 trims the combinations by it) is printed on stderr, so the expansion is reviewed by its
count. ADD-A124 (recency_problems): `recency` is a table of named sets, each name a key (S1) and each set two or more
flags (S12), each a state flag of `axes.states`, none twice, and no two sets of the entry order two flags oppositely
(S12: so the answer under no reported order is one order). The gate's runs per order (recency_orders, S17): the
entry's state combinations (state_combinations: every subset of `axes.states` reduced by `exclusive`, each group
keeping its first present flag, less those a `not_drawn` entry stating only `states` and `not_states` removes),
each run once per order of its flags that belong to a recency set, the orders giving every set the same latest flag
counted once (combination_orders; at most n! for n such flags); the runs, the combinations, the most orders of one
combination and its n are printed on stderr, and their sums in the summary. The generation rule applies as to a
fact.

A palette fact (`docs/facts/palettes/<project>.toml`, S17 *Palette facts*): `key = "<project>.<flavour>.<name>"`
(`<project>` the file's stem), a colour literal as `value`, at least one citation; no `rows`, `when`,
`scheme_file` or `accent`; its sources' entries are palette entries (`generations = []`, `palette = true`). Every
bundled palette (bundled_palettes(): a file of PRESET_DIRS whose `[preset] kind` is `"palette"`) has every
`[palette.sources]` value a palette-fact key, every key of `[palette.sources]` and `[palette.picks]` the TOML path of a
string it states holding a colour literal, and every such string in exactly one of the two (S11, S14). Today no
bundled palette exists, so this rule checks nothing on the repository; the self-test runs it.

existing_presets() is the one function that says which presets exist: the stems of the new model's presets,
`native-theme/src/next/presets/<id>.toml` (`presets/` after the switch), that are platform presets, so today's
presets in `native-theme/src/presets/` do not count (the M1a design's decision 8). The directory may be absent or
empty: then no preset exists and the generation rule asks nothing of a fact without `preset:`.

    python3 scripts/check_facts.py --self-test   # each rule on inline fixtures: a passing and a failing case
"""
import itertools
import math
import re
import sys
import tempfile
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

import check_spec_names
import spec_catalogue

FACTS = Path("docs/facts")
SOURCES = FACTS / "sources.toml"
EVIDENCE = Path("docs/evidence")
CORE = Path("docs/todo_v0.7.1_model-completeness-spec.md")
CATALOGUE = Path("docs/todo_v0.7.1_model-completeness-spec-widgets.md")
GAP_MAPS = [Path(f"docs/todo_v0.7.1_model-completeness-gap-map-{p}.md")
            for p in ("gnome", "ios", "kde", "macos", "material", "windows")]
PRESET_DIRS = (Path("native-theme/src/next/presets"),)
PLATFORMS = {
    "gnome": ("adwaita",),
    "kde": ("kde-breeze",),
    "macos": ("macos-monterey", "macos-tahoe"),
    "ios": ("ios-13", "ios-26"),
    "windows": ("windows-11",),
    "material": ("material-3",),
}
PALETTES = "palettes"
IMPORTED = "imported"
STAGED = "staged"
STATE_MODELS = "state-models.toml"
ALL_PRESETS = tuple(p for ps in PLATFORMS.values() for p in ps)

FACT_REQUIRED = {"key", "value", "rows", "cite"}
PENDING_REQUIRED = {"key", "pending", "rows"}
FACT_OPTIONAL = {"when", "scheme_file", "accent"}
PENDING_RX = re.compile(r"^U[1-9][0-9]*$")   # an Unmeasured register Id (S17 *A pending measurement*)
PALETTE_REQUIRED = {"key", "value", "cite"}
CITE_FIELDS = {
    # kind: (required, optional)
    "published": ({"source", "quote"}, {"conversion", "absent"}),
    "read": ({"source", "quote"}, {"conversion"}),
    "measured": ({"evidence", "tolerance"}, {"conversion", "inputs"}),
}
MODEL_REQUIRED = {"widget", "available", "rows", "cite"}
MODEL_DECLARATIONS = {"variants", "sizes", "densities", "axes", "exclusive", "not_drawn", "default_size", "flags",
                      "requires_variants", "sizes_by_variant", "sizes_by_context", "exclusive_variants",
                      "variant_sets", "variant_products", "default_size_by_variant", "recency"}
MODEL_AVAILABLE_REQUIRED = {"variants", "sizes", "densities", "axes", "exclusive", "not_drawn"}
AXES_KEYS = {"states", "conditions", "contexts"}

WHEN_KEYS = {"conditions", "not_conditions", "context", "not_context", "position", "variants", "not_variants",
             "sizes", "not_sizes", "densities", "not_densities", "size_class", "direction", "states", "not_states",
             "latest"}
CONTEXT_KEYS = {"scope", "widget", "part", "surface", "sizes", "not_sizes", "variants", "not_variants", "states",
                "not_states"}
POSITION_KEYS = {"among", "first", "last", "only", "parity", "previous", "next"}
SIBLING_KEYS = {"widget", "part", "variants", "not_variants", "states", "not_states"}
SIZE_CLASS_KEYS = {"horizontal", "not_horizontal", "vertical", "not_vertical"}

ROW_RX = re.compile(r"^[A-Z][A-Za-z0-9]*(?:[-.][A-Za-z0-9]+)*†?$")
SOURCE_RX = re.compile(r"^(\S+) (?:#(.*)|(\S+):(\d+)(?:-(\d+))?)$")
EVIDENCE_RX = re.compile(r"^docs/evidence/([^/#]+)/(\d{4}-\d\d-\d\d-run-[^/#]+)/([^/#]+)/([^#]+)#(.+)$")
SEG = re.compile(r"^@?[a-z0-9][a-z0-9_]*$")
NAME = re.compile(r"^@?[a-z_][a-z0-9_]*$")
KEYED = re.compile(r"^@[a-z_][a-z0-9_]*$")
NUM = r"-?\d+(?:\.\d+)?"
NUMBER_STRING = re.compile(r"^\s*[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?\s*$")
UNIT_LITERAL = re.compile(rf"^({NUM})(px|ms|deg|pt)$")
HEX_LITERAL = re.compile(r"^#(?:[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$")
FUNC_COLOUR = re.compile(rf"^(srgb|display_p3)\(\s*({NUM})\s*,\s*({NUM})\s*,\s*({NUM})\s*(?:,\s*({NUM})\s*)?\)$")
KEYWORDS = {"auto", "absent", "none"}
RESERVED = {"base", "scheme", "font_size", "line_height", "body_font_height", "text_scale", "true", "false"}
LITERAL_FORMS = {"srgb", "display_p3"}
# the dimension of a unit literal in a value (S2.2): px is a length or a font size, pt a font size
UNIT_DIMS = {"px": ("length", "font_size"), "pt": ("font_size",), "ms": ("duration",), "deg": ("angle",)}
SUFFIX_DIMS = {"_px": "length", "_ms": "duration", "_deg": "angle", "_pt": "font_size"}
# the core's scalar types and their dimensions (S1, S2.1, S2.2); a type not listed gives no dimension check
KEY_DIMS = {"Px": "length", "Length": "length", "Ms": "duration", "Duration": "duration", "Deg": "angle",
            "Angle": "angle", "FontSize": "font_size", "Color": "colour", "Paint": "paint", "Fraction": "number",
            "f32": "number", "f64": "number", "u8": "number", "u16": "number", "u32": "number", "i32": "number",
            "bool": "bool", "String": "text"}
SCALAR_DIMS = {"length", "duration", "angle", "font_size", "colour", "number", "bool", "text"}
UNIT_OF = {"length": "px", "duration": "ms", "angle": "deg", "font_size": "pt"}
WRAPPER = re.compile(r"(?:Value|Option|Box)<(.+)>")
PRODUCT_GROUPS = {"any", "one"}   # a variant product's groups (S17 *Variant products*, ADD-A122)
# S4.1's motion keys (ADD-A122): the closed list of words that only say the value is a duration, a curve or a motion,
# each also in its plural, the two-word ones as word pairs; and the toolkit prefixes a snake_case key can still show
MOTION_WORDS = {"animation", "duration", "time", "ms", "transition", "timing", "curve", "easing", "spring"}
MOTION_WORDS |= {w + "s" for w in MOTION_WORDS - {"ms"}}
MOTION_PAIRS = (("key", "spline"), ("key", "splines"), ("cubic", "bezier"), ("cubic", "beziers"))
MOTION_PREFIXES = ("adw", "gtk", "q", "k", "ns", "ui", "c", "md_sys_motion", "units")
# the catalogue's *token.motion.durations* table: each row's platform as the facts directory it is
DURATION_ROWS = {"libadwaita": "gnome", "kde-breeze": "kde", "windows-11": "windows", "macos-*": "macos",
                 "ios-*": "ios", "material-3": "material"}
DURATION_FAMILY = ("by family", "one entry per")   # a row naming a family: its keys are not all listed
PART_PATCH = "PartPatch"   # the type key_info gives a whole part's key, `<widget>.<part>` (S17, ADD-A121)
# a map keyed by text (S11; ADD-A123): its key types, an entry's key a TOML bare key or a quoted one, and the stand-in
# segment key_info walks the core's types with in its place
TEXT_KEY_TYPES = ("String", "LanguageTag")
TEXT_MAP = re.compile(rf"Vec<\(\s*(?:{'|'.join(TEXT_KEY_TYPES)})\s*,\s*(.+)\)>")
TOML_BARE = re.compile(r"^[A-Za-z0-9_-]+$")
TEXT_ENTRY = "text_entry"
DRAWN_AS_KEYS = {"widget", "part", "variants"}
KEY_RX = re.compile(r"^[a-z0-9][a-z0-9_]*$")   # S1's key grammar: a recency set's name (S10; ADD-A124)


class InputError(Exception):
    pass


# ------------------------------------------------------------------------------------------------- the core

def split_top(body: str) -> list:
    """`body` split at its top-level commas (outside <>, (), [] and {})."""
    out, cur, d = [], "", 0
    for c in body + ",":
        if c in "<([{":
            d += 1
        elif c in ">)]}":
            d -= 1
        if c == "," and d == 0:
            out.append(cur)
            cur = ""
        else:
            cur += c
    return out


def fields_of(body: str, pub=True) -> dict:
    """{field: type} of a struct body (`pub` fields) or of an enum variant's braces (no `pub`)."""
    fields = {}
    for piece in split_top(body):
        fm = re.match(r"\s*pub (\w+)\s*:\s*(.+?)\s*$" if pub else r"\s*(\w+)\s*:\s*(.+?)\s*$", piece, re.S)
        if fm:
            fields[fm.group(1)] = " ".join(fm.group(2).split())
    return fields


def bodies(code: str, kind: str):
    """(name, body) of every `pub <kind> <name> { … }` of the code."""
    for m in re.finditer(rf"pub {kind} (\w+)[^{{;(]*\{{", code):
        i, depth = m.end(), 1
        while i < len(code) and depth:
            depth += code[i] == "{"
            depth -= code[i] == "}"
            i += 1
        yield m.group(1), code[m.end():i - 1]


def core_structs(text: str) -> dict:
    """struct name -> {field: type} from the core's Rust sketches (comments stripped, fields split at top level);
    an enum's variant with named fields is a struct too, `<Enum>::<variant>` (the variant in snake_case, as S11
    spells it)."""
    code = check_spec_names.strip_comments(check_spec_names.rust_blocks(text))
    out = {name: fields_of(body) for name, body in bodies(code, "struct")}
    for name, payloads in core_enums(text).items():
        for variant, (form, payload) in payloads.items():
            if form == "struct":
                out[f"{name}::{variant}"] = payload
    return out


def core_enums(text: str) -> dict:
    """enum name -> {variant in snake_case: ("unit", None) | ("tuple", type) | ("struct", {field: type})}."""
    code = check_spec_names.strip_comments(check_spec_names.rust_blocks(text))
    out = {}
    for name, body in bodies(code, "enum"):
        variants = {}
        for piece in split_top(body):
            vm = re.match(r"\s*([A-Z]\w*)\s*(.*?)\s*$", piece, re.S)
            if not vm:
                continue
            rest = vm.group(2)
            if rest.startswith("{") and rest.endswith("}"):
                payload = ("struct", fields_of(rest[1:-1], pub=False))
            elif rest.startswith("(") and rest.endswith(")"):
                payload = ("tuple", " ".join(rest[1:-1].split()))
            else:
                payload = ("unit", None)
            variants[check_spec_names.snake(vm.group(1))] = payload
        out[name] = variants
    return out


def core_functions(text: str) -> dict:
    """{name: (the fewest arguments, the most or None)} of S2.2's **Functions** table, each function's arity its
    signature's (`alpha(c, a)` and `with_alpha(c, a)` two; `pick(index, c0, c1, …)` two or more)."""
    m = re.search(r"\*\*Functions\*\*.*?\n\n(\|.*?)\n\n", text, re.S)
    if not m:
        raise InputError("core: no **Functions** table")
    out = {}
    for line in m.group(1).splitlines()[2:]:
        first = line.split("|")[1] if line.count("|") > 2 else ""
        for name, params in re.findall(r"`([a-z_][a-z0-9_]*)\(([^)`]*)\)", first):
            ps = [x.strip() for x in params.split(",") if x.strip()]
            out[name] = (len(ps) - 1, None) if ps and ps[-1] in ("…", "...") else (len(ps), len(ps))
    return out


def motion_duration_keys(cat_text: str) -> dict:
    """{facts directory: the duration keys its row of the catalogue's *token.motion.durations* table lists}, for each
    row that names no family (DURATION_FAMILY): a row that does lists its keys incompletely and is left out, as is
    every platform while the catalogue has no such table. A key is a backticked snake_case name of the row's Keys
    cell; a row of another platform is an input error."""
    m = re.search(r"\*\*token\.motion\.durations\*\*.*?\n\n(\|.*?)(?:\n\n|\Z)", cat_text, re.S)
    if not m:
        return {}
    out = {}
    for line in m.group(1).splitlines()[2:]:
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) < 2:
            continue
        label = cells[0]
        if label not in DURATION_ROWS:
            raise InputError(f"catalogue: *token.motion.durations* has a row {label!r}, not one of "
                             f"{', '.join(DURATION_ROWS)}")
        if any(f in cells[1] for f in DURATION_FAMILY):
            continue
        out[DURATION_ROWS[label]] = set(re.findall(r"`([a-z][a-z0-9_]*)`", cells[1]))
    return out


def gap_map_rows(root: Path) -> set | None:
    """Every row ID of the platform gap maps (first cell of a table row matching ROW_RX), or None without them."""
    out = set()
    found = False
    for p in GAP_MAPS:
        if not (root / p).exists():
            continue
        found = True
        for line in (root / p).read_text(encoding="utf-8").splitlines():
            m = re.match(r"^\|\s*([^|]+?)\s*\|", line)
            if m and ROW_RX.match(m.group(1)) and re.search(r"\d", m.group(1)):
                out.add(m.group(1))
    return out if found else None


def existing_presets(root: Path) -> set:
    """The platform presets that exist: the stems of the new model's native-theme/src/next/presets/<id>.toml."""
    out = set()
    for d in PRESET_DIRS:
        if (root / d).is_dir():
            out |= {p.stem for p in (root / d).glob("*.toml") if p.stem in ALL_PRESETS}
    return out


def bundled_palettes(root: Path) -> list:
    """(path, text, data) of every file of PRESET_DIRS whose [preset] kind is "palette" (S11, S14)."""
    out = []
    for d in PRESET_DIRS:
        if not (root / d).is_dir():
            continue
        for p in sorted((root / d).glob("*.toml")):
            text = p.read_text(encoding="utf-8")
            try:
                data = tomllib.loads(text)
            except tomllib.TOMLDecodeError:
                continue
            if isinstance(data.get("preset"), dict) and data["preset"].get("kind") == "palette":
                out.append((p.relative_to(root).as_posix(), text, data))
    return out


# ---------------------------------------------------------------------------------------------- the files

@dataclass
class Cite:
    line: int
    data: dict
    lines: dict = field(default_factory=dict)

    def ln(self, name):
        return self.lines.get(name, self.line)


@dataclass
class Item:
    """A [[fact]] or a [[model]] with its citations."""
    file: str
    line: int
    table: str            # "fact" or "model"
    platform: str         # a PLATFORMS key or "palettes"
    data: dict
    cites: list
    lines: dict = field(default_factory=dict)

    def ln(self, name):
        return self.lines.get(name, self.line)

    @property
    def palette(self):
        return self.platform == PALETTES


def items_of(file: str, platform: str, text: str, data: dict, table: str) -> list:
    """The file's items with the line of each header and of each field (the first `<field> =` in its span)."""
    heads = []   # (line, is_cite)
    for i, line in enumerate(text.splitlines(), 1):
        m = re.match(rf"^\s*\[\[\s*{table}(\.cite)?\s*\]\]", line)
        if m:
            heads.append((i, bool(m.group(1))))
    lines = text.splitlines()

    def field_lines(start, end):
        out = {}
        for j in range(start, end):
            fm = re.match(r'^\s*"?([A-Za-z0-9_]+)"?\s*=', lines[j])
            if fm and fm.group(1) not in out:
                out[fm.group(1)] = j + 1
        return out

    spans = []
    for k, (ln, is_cite) in enumerate(heads):
        end = heads[k + 1][0] - 1 if k + 1 < len(heads) else len(lines)
        spans.append((ln, is_cite, field_lines(ln, end)))
    raw = data.get(table, [])
    out = []
    pos = 0
    for entry in raw if isinstance(raw, list) else []:
        while pos < len(spans) and spans[pos][1]:
            pos += 1
        head = spans[pos] if pos < len(spans) else (1, False, {})
        pos += 1
        cites = []
        raw_cites = entry.get("cite", []) if isinstance(entry, dict) else []
        for c in raw_cites if isinstance(raw_cites, list) else []:
            if not isinstance(c, dict):
                continue   # a finding of fact_problems / model_problems
            if pos < len(spans) and spans[pos][1]:
                cites.append(Cite(spans[pos][0], c, spans[pos][2]))
                pos += 1
            else:
                cites.append(Cite(head[0], c))
        out.append(Item(file, head[0], table, platform, entry if isinstance(entry, dict) else {}, cites, head[2]))
    return out


def parse_source(s):
    """(id, path, l1, l2, pointer) of a citation's `source`, or None."""
    m = SOURCE_RX.match(s) if isinstance(s, str) else None
    if not m:
        return None
    if m.group(2) is not None:
        return m.group(1), None, None, None, m.group(2)
    l1 = int(m.group(4))
    return m.group(1), m.group(3), l1, int(m.group(5)) if m.group(5) else l1, None


def parse_evidence(s):
    """(file, path, harness, run, image) of a measured citation's `evidence`, or None."""
    m = EVIDENCE_RX.match(s) if isinstance(s, str) else None
    if not m:
        return None
    harness, run, image, rest, path = m.groups()
    return f"docs/evidence/{harness}/{run}/{image}/{rest}", path, harness, run, image


class Context:
    """What the checks read: the sources, the core's and the catalogue's names, the existing presets, the runs."""

    def __init__(self, root=Path("."), core_text=None, cat_text=None, sources_text=None, presets=None, rows=None):
        self.root = Path(root)
        try:
            core_text = core_text if core_text is not None else (self.root / CORE).read_text(encoding="utf-8")
            cat_text = cat_text if cat_text is not None else (self.root / CATALOGUE).read_text(encoding="utf-8")
            sources_text = sources_text if sources_text is not None else (self.root / SOURCES).read_text(
                encoding="utf-8")
            self.sources_data = tomllib.loads(sources_text)
        except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as err:
            raise InputError(str(err)) from err
        try:
            self.ck = check_spec_names.Checker(core_text=core_text, cat_text=cat_text)
        except SystemExit as err:
            raise InputError(str(err)) from err
        self.entries = {e.name: e for e in spec_catalogue.entries(cat_text)}
        self.structs = core_structs(core_text)
        self.enums = core_enums(core_text)
        self.functions = core_functions(core_text)
        self.motion_durations = motion_duration_keys(cat_text)
        entries = self.sources_data.get("source", [])
        if not isinstance(entries, list):
            raise InputError(f"{SOURCES}: `source` is not an array of [[source]] tables")
        self.sources = {e["id"]: e for e in entries if isinstance(e, dict) and isinstance(e.get("id"), str)}
        self.presets = existing_presets(self.root) if presets is None else set(presets)
        self.rows = gap_map_rows(self.root) if rows is None else rows
        self._runs = {}

    def run_toml(self, harness, run):
        """The parsed run.toml of a run folder, or None when it is missing or no valid TOML."""
        key = (harness, run)
        if key not in self._runs:
            p = self.root / EVIDENCE / harness / run / "run.toml"
            try:
                self._runs[key] = tomllib.loads(p.read_text(encoding="utf-8"))
            except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError):
                self._runs[key] = None
        return self._runs[key]

    def cite_generations(self, cite: dict) -> set:
        """The generations a citation serves: its source entry's, or the preset its run maps its image to."""
        if cite.get("kind") == "measured":
            ev = parse_evidence(cite.get("evidence"))
            if not ev:
                return set()
            rt = self.run_toml(ev[2], ev[3])
            images = rt.get("images", {}) if isinstance(rt, dict) else {}
            img = images.get(ev[4]) if isinstance(images, dict) else None
            preset = img.get("preset") if isinstance(img, dict) else None
            return {preset} if isinstance(preset, str) else set()
        src = parse_source(cite.get("source"))
        entry = self.sources.get(src[0]) if src else None
        gens = entry.get("generations", []) if entry else []
        return {g for g in gens if isinstance(g, str)} if isinstance(gens, list) else set()

    def covered(self, item: Item):
        """(the generations the item covers, its `preset:` generation or None)."""
        if item.palette:
            return set(), None
        when = item.data.get("when") if isinstance(item.data.get("when"), dict) else {}
        conds = when.get("conditions", []) if isinstance(when.get("conditions"), list) else []
        nots = when.get("not_conditions", []) if isinstance(when.get("not_conditions"), list) else []
        pres = [c[7:] for c in conds if isinstance(c, str) and c.startswith("preset:")]
        out = set(pres[:1]) if pres else set(PLATFORMS.get(item.platform, ())) & self.presets
        out -= {c[7:] for c in nots if isinstance(c, str) and c.startswith("preset:")}
        return out, (pres[0] if pres else None)


def load(ctx: Context):
    """(items, findings, file count): every fact file and state-models file under docs/facts."""
    findings, items, nfiles = [], [], 0
    base = ctx.root / FACTS
    if not base.is_dir():
        return items, findings, nfiles
    for p in sorted(base.iterdir()):
        rel = p.relative_to(ctx.root).as_posix()
        if p.is_file():
            if p.name not in ("sources.toml", "README.md"):
                findings.append((rel, 1, "not a facts directory's file (docs/facts holds sources.toml and the "
                                         "platform, palettes, imported and staged directories)"))
            continue
        if p.name in (IMPORTED, STAGED):
            continue
        if p.name not in PLATFORMS and p.name != PALETTES:
            findings.append((rel, 1, f"not a facts directory (one of {', '.join(PLATFORMS)}, {PALETTES}, "
                                     f"{IMPORTED}, {STAGED})"))
            continue
        for f in sorted(p.rglob("*")):
            frel = f.relative_to(ctx.root).as_posix()
            if f.is_dir():
                if f.parent == p:
                    findings.append((frel, 1, "a folder inside a facts directory"))
                continue
            if f.name == "README.md":
                continue
            if f.suffix != ".toml" or f.parent != p:
                findings.append((frel, 1, "not a fact file (`<directory>/<area>.toml`)"))
                continue
            nfiles += 1
            try:
                text = f.read_text(encoding="utf-8")
            except (OSError, UnicodeDecodeError) as err:
                raise InputError(f"{frel}: cannot be read: {err}") from err
            try:
                data = tomllib.loads(text)
            except tomllib.TOMLDecodeError as err:
                m = re.search(r"line (\d+)", str(err))
                findings.append((frel, int(m.group(1)) if m else 1, f"not valid TOML: {err}"))
                continue
            is_model = f.name == STATE_MODELS and p.name != PALETTES
            table = "model" if is_model else "fact"
            for k in data:
                if k != table:
                    findings.append((frel, 1, f"unknown table `{k}` (a {'state-models' if is_model else 'fact'} "
                                              f"file holds [[{table}]] only)"))
            items += items_of(frel, p.name, text, data, table)
    return items, findings, nfiles


# ------------------------------------------------------------------------------------------------- keys

def key_segments(key: str):
    """(the segments, the indices of the quoted ones) of a fact's key read as a TOML dotted key, or a problem: a
    segment is bare (up to the next `.`) or a TOML string, `"…"` (its escapes read as TOML reads them) or `'…'`,
    which is one segment whatever it holds (`token.text.fallbacks.per_family."Segoe Fluent Icons"`; S17, S11,
    ADD-A123); an unterminated quote, an empty segment or a quote inside a bare one is a problem."""
    segs, quoted, i = [], set(), 0
    while True:
        if i >= len(key) or key[i] == ".":
            return f"key {key!r} has an empty segment"
        if key[i] in "\"'":
            q = key[i]
            j = i + 1
            while j < len(key) and key[j] != q:
                j += 2 if q == '"' and key[j] == "\\" else 1
            if j >= len(key):
                return f"key {key!r}: the quote at {key[i:]!r} is not terminated (a quoted segment, S17)"
            try:
                segs.append(tomllib.loads(f"k = {key[i:j + 1]}")["k"])
            except tomllib.TOMLDecodeError:
                return f"key {key!r}: {key[i:j + 1]} is not a TOML string (a quoted segment, S17)"
            quoted.add(len(segs) - 1)
            i = j + 1
            if i < len(key) and key[i] != ".":
                return f"key {key!r}: a quoted segment ends at a `.` or the key's end"
        else:
            j = key.find(".", i)
            j = len(key) if j < 0 else j
            if any(q in key[i:j] for q in "\"'"):
                return f"key {key!r}: a quote inside the segment {key[i:j]!r} (a quoted segment is quoted whole, S17)"
            segs.append(key[i:j])
            i = j
        if i == len(key):
            return segs, quoted
        i += 1


def canonical_key(key):
    """A fact's key as the segments it names (two keys equal as TOML keys are one, S11, S17), or the key itself where
    it does not read."""
    parsed = key_segments(key) if isinstance(key, str) else None
    return tuple(parsed[0]) if isinstance(parsed, tuple) else key


def key_info(ctx: Context, key: str):
    """(problem or None, dimension or None, widget name or None, kind, the core's type of the key or None) of a
    fact's key; kind is "widget", "role", "token" or "input". A segment that is no snake_case name (a quoted one, a
    TOML bare key such as `Cantarell`) is an entry of a map keyed by text (TEXT_MAP: `per_family`, `per_language`;
    S17, S11, ADD-A123), the core's types walked with TEXT_ENTRY in its place."""
    if not isinstance(key, str) or not key:
        return "the key is not a non-empty string", None, None, None, None
    parsed = key_segments(key)
    if isinstance(parsed, str):
        return parsed, None, None, None, None
    segs, quoted = parsed
    for i, seg in enumerate(segs):
        if i not in quoted and SEG.match(seg):
            continue
        prefix = key_info(ctx, ".".join(segs[:i])) if i else (None,) * 5
        if not (prefix[0] is None and prefix[4] and TEXT_MAP.fullmatch(strip_type(prefix[4]))):
            if i in quoted:
                return (f"key {key!r}: the quoted segment {seg!r} is not an entry of a map keyed by text (S17, S11: "
                        f"`per_family`'s families)"), None, None, None, None
            return f"key {key!r} is not a dotted path of snake_case segments", None, None, None, None
        if i not in quoted and not TOML_BARE.match(seg):
            return (f"key {key!r}: the entry {seg!r} is bare where TOML needs it quoted (S17, S11)", None, None,
                    None, None)
        segs[i] = TEXT_ENTRY
    suffixed = next((s for s in segs[1:] if s.endswith(tuple(SUFFIX_DIMS))), None)
    if suffixed:
        return (f"key {key!r}: `{suffixed}` carries TOML's unit suffix (a key names the field, the value carries "
                f"the unit, S17)"), None, None, None, None
    ck = ctx.ck

    def typed(type_str, rest, widget, kind):
        inside = enum_inside(ctx, type_str, rest)
        if inside:
            return (f"key {key!r}: `{inside[1]}` is inside {inside[0]}, an enum, which a fact states whole (a "
                    f"reference or S11's spelling of its variant; S17, S6.1)"), None, widget, kind, None
        t = type_at(ctx, type_str, rest)
        return None, KEY_DIMS.get(strip_type(t)) if t else None, widget, kind, t

    if segs[0] == "role":
        rest = ".".join(segs[1:])
        if not rest or not ck.role_ok(rest):
            return f"key {key!r} names no colour role (S3)", None, None, "role", None
        if segs[1].startswith("@"):
            return None, "colour", None, "role", None
        if segs[1] == "global":
            if segs[2:3] == ["accent_ramp"] and len(segs) > 4:
                return (f"key {key!r}: an accent ramp role is `role.global.accent_ramp.<key>` "
                        f"(`GlobalRole::AccentRamp(Key)`, S3; ADD-A122)"), None, None, "role", None
            return typed("GlobalRoles", segs[2:], None, "role")
        return typed("SurfaceRoles", segs[2:] if segs[2:3] != ["disabled"] else segs[3:], None, "role")
    if segs[0] == "token":
        rest = ".".join(segs[1:])
        if not rest or not ck.token_ok(rest):
            return f"key {key!r} names no token (S4)", None, None, "token", None
        if segs[1:2] == ["motion"] and segs[2:3] in (["durations"], ["timings"]) and len(segs) > 4:
            return (f"key {key!r}: a duration or timing is `token.motion.{segs[2]}.<key>`, stated whole (S4.1, "
                    f"S17)"), None, None, "token", None
        return typed("Tokens", segs[1:], None, "token")
    if segs[0] == "input" and not (len(segs) > 1 and "input" in ctx.entries and (
            segs[1] == "values" or any(segs[1] in p.names for p in ctx.entries["input"].parts))):
        problem = input_problem(ctx, segs[1:])
        return (f"key {key!r}: {problem}" if problem else None), None if problem else "colour", None, "input", None
    if segs[0] == "component":
        if len(segs) < 2:
            return f"key {key!r} names no component", None, None, "widget", None
        widget, rest = f"component.{segs[1]}", segs[2:]
    else:
        widget, rest = segs[0], segs[1:]
    entry = ctx.entries.get(widget)
    if entry is None:
        return f"key {key!r}: {widget} is not a catalogue entry", None, None, "widget", None
    if len(rest) == 1 and rest[0] != "values" and any(rest[0] in p.names for p in entry.parts):
        return None, None, widget, "widget", PART_PATCH
    if len(rest) < 2:
        return (f"key {key!r} is not `{widget}.<part>.<field>`, `{widget}.<part>` or `{widget}.values.<name>`", None,
                widget, "widget", None)
    if rest[0] == "values":
        norm = check_spec_names.norm_value
        value = next((v for v in entry.values if norm(v.name) == norm(rest[1])), None)
        if value is None:
            return f"key {key!r}: `{rest[1]}` is not a widget value of {widget}", None, widget, "widget", None
        if len(rest) > 2 and (value.type or "").startswith("Value<"):
            return f"key {key!r}: {widget}'s value `{rest[1]}` has no leaves", None, widget, "widget", None
        if not value.type:
            return None, None, widget, "widget", None
        return typed(value.type, rest[2:], widget, "widget")
    part = next((p for p in entry.parts if rest[0] in p.names), None)
    if part is None:
        return f"key {key!r}: `{rest[0]}` is not a part of {widget}", None, widget, "widget", None
    fld = check_spec_names.norm_value(rest[1])
    if fld not in {check_spec_names.norm_value(f) for f in part.fields}:
        return (f"key {key!r}: `{rest[1]}` is not a field of {widget}'s part `{rest[0]}`", None, widget, "widget",
                None)
    if not ck.field_ok(rest[1:]):
        return (f"key {key!r}: `{'.'.join(rest[1:])}` is not a field path (the core's blocks)", None, widget, "widget",
                None)
    return typed("PartStyle", rest[1:], widget, "widget")


def motion_key_problems(ctx, key, platform) -> list:
    """S4.1's rule for a duration's or timing's key (ADD-A122), where the checker can tell: the key keeps no word of
    MOTION_WORDS (or MOTION_PAIRS) unless it is made only of them, starts with no toolkit prefix of MOTION_PREFIXES,
    and, a duration key of a platform whose catalogue row lists every key (motion_duration_keys), is one of them."""
    m = re.fullmatch(r"token\.motion\.(durations|timings)\.([a-z0-9_]+)", key) if isinstance(key, str) else None
    if not m:
        return []
    name = m.group(2)
    words = name.split("_")
    dropped = [w in MOTION_WORDS for w in words]
    for i in range(len(words) - 1):
        if (words[i], words[i + 1]) in MOTION_PAIRS:
            dropped[i] = dropped[i + 1] = True
    out = []
    if any(dropped) and not all(dropped):
        kept = [w for w, d in zip(words, dropped) if d]
        out.append(f"key {key!r}: `{name}` keeps {', '.join(f'`{w}`' for w in kept)}, which S4.1's rule drops (a word "
                   f"that only says the value is a duration, a curve or a motion)")
    prefix = next((p for p in MOTION_PREFIXES if name == p or name.startswith(p + "_")), None)
    if prefix:
        out.append(f"key {key!r}: `{name}` keeps the toolkit's prefix `{prefix}`, which S4.1's rule drops")
    listed = ctx.motion_durations.get(platform)
    if m.group(1) == "durations" and listed is not None and name not in listed:
        out.append(f"key {key!r}: `{name}` is not a key the catalogue's *token.motion.durations* row for {platform}/ "
                   f"lists (S4.1)")
    return out


def input_problem(ctx, segs):
    if not segs:
        return "names no input"
    sets = ctx.ck.v["surface_sets"]
    if segs[0] == "kde_scheme":
        if len(segs) == 2 and segs[1] in ("groups", "inactive_groups", "unset_keys"):
            return None
        if len(segs) == 3 and segs[1] == "wm":
            return None
        if len(segs) >= 3 and segs[1] in sets and (len(segs) == 3 or (len(segs) == 4 and segs[2] == "inactive")):
            return None
        return "not `input.kde_scheme.<set>.<key>`, `.<set>.inactive.<key>`, `.wm.<key>` or one of the lists (S9.1)"
    if segs[0] == "inactive":
        if len(segs) == 3 and segs[1] in sets:
            return None
        return "not `input.inactive.<set>.<key>` (S9.1)"
    return None


def strip_type(t: str) -> str:
    """The type inside its `Value<…>`, `Option<…>` and `Box<…>` wrappers."""
    t = (t or "").strip()
    while True:
        m = WRAPPER.fullmatch(t)
        if not m:
            return t
        t = m.group(1).strip()


def type_at(ctx, type_str, segs):
    """The core's type, wrappers kept, of the leaf `segs` names under a core type, or None where the sketches do
    not say."""
    t, segs = (type_str or "").strip(), list(segs)
    for _ in range(64):
        if not segs:
            return t or None
        t = strip_type(t)
        m = re.fullmatch(r"Sides<(.+)>", t)
        if m:
            segs, t = segs[1:], m.group(1).strip()
            continue
        m = re.fullmatch(r"Vec<\(\s*\w+\s*,\s*(.+)\)>", t)
        if m:
            segs, t = segs[1:], m.group(1).strip()
            continue
        if t in ctx.structs:
            fields = ctx.structs[t]
            name = segs[0] if segs[0] in fields else check_spec_names.norm_value(segs[0])
            if name not in fields:
                return None
            segs, t = segs[1:], fields[name]
            continue
        return None
    return None


def enum_inside(ctx, type_str, segs):
    """(the core enum, the segment) where the path `segs` under a core type goes inside an enum, or None: an enum
    block, as any enum, is stated whole, by a reference or S11's spelling of its variant (S17, S6.1; ADD-A122)."""
    t, segs = (type_str or "").strip(), list(segs)
    for _ in range(64):
        if not segs:
            return None
        t = strip_type(t)
        if t in ctx.enums:
            return t, segs[0]
        m = re.fullmatch(r"Sides<(.+)>", t) or re.fullmatch(r"Vec<\(\s*\w+\s*,\s*(.+)\)>", t)
        if m:
            segs, t = segs[1:], m.group(1).strip()
            continue
        if t in ctx.structs:
            fields = ctx.structs[t]
            name = segs[0] if segs[0] in fields else check_spec_names.norm_value(segs[0])
            if name not in fields:
                return None
            segs, t = segs[1:], fields[name]
            continue
        return None
    return None


def type_dim(ctx, type_str, segs):
    """The dimension of the leaf `segs` names under a core type, or None where the sketches do not say."""
    t = type_at(ctx, type_str, segs)
    return KEY_DIMS.get(strip_type(t)) if t else None


def text_kind(t):
    """"plain" for a plain `String` (always text, S11), "text" for a `Value<String>` (or `Option<String>`, whose
    keywords and `=` keep their meaning), else None."""
    if not t or strip_type(t) != "String":
        return None
    return "plain" if t.strip() == "String" else "text"


def child_type(ctx, t, key):
    """The core's type of a table's key under the table's type: a struct's field (its unit suffix dropped), or an
    enum's variant payload (a one-key table, S11), or None."""
    s = strip_type(t)
    if s in ctx.structs:
        fields = ctx.structs[s]
        return fields.get(key) or fields.get(re.sub(r"_(px|ms|deg|pt)$", "", key))
    payload = ctx.enums.get(s, {}).get(key)
    if payload and payload[0] == "struct":
        return f"{s}::{key}"
    if payload and payload[0] == "tuple":
        return payload[1]
    return None


def enum_name_ok(ctx, e, name, depth=0):
    """A name spells a value of the core's enum `e` (S11): a unit variant's snake_case name, `@<key>` where a variant
    holds a `Key`, or a name of an enum a one-field tuple variant holds (`StateFlag::Shared(SharedFlag)`)."""
    variants = ctx.enums.get(e, {})
    if name.startswith("@"):
        return any(form == "tuple" and payload == "Key" for form, payload in variants.values())
    if variants.get(name, ("", None))[0] == "unit":
        return True
    return depth < 8 and any(form == "tuple" and payload in ctx.enums and enum_name_ok(ctx, payload, name, depth + 1)
                             for form, payload in variants.values())


def item_type(t):
    """The item type of a `Vec<T>` (not of a keyed `Vec<(K, T)>`), or None."""
    m = re.fullmatch(r"Vec<(.+)>", strip_type(t))
    return m.group(1).strip() if m and not m.group(1).strip().startswith("(") else None


def text_leaves(ctx, value, t, path=""):
    """(path as S11 spells it, string) of every text value in `value` under the core's type `t` (S11's text row)."""
    kind = text_kind(t)
    if isinstance(value, str):
        if kind == "plain" or (kind == "text" and not value.startswith("=") and value not in KEYWORDS):
            return [(path, value)]
        return []
    sub = (lambda k: f"{path}.{k}" if path else str(k))
    if isinstance(value, list):
        it = item_type(t)
        return [x for i, v in enumerate(value) for x in text_leaves(ctx, v, it, sub(i))] if it else []
    if isinstance(value, dict) and t:
        return [x for k, v in value.items() for x in text_leaves(ctx, v, child_type(ctx, t, k), sub(k))]
    return []


# ----------------------------------------------------------------------------------------------- values

TOKEN_RX = re.compile(r"\s*(?:(?P<hex>#[0-9a-zA-Z]+)|(?P<num>-?\d+(?:\.\d+)?(?:px|ms|deg|pt)?)(?![\w.])"
                      r"|(?P<path>@?[a-z_][a-z0-9_]*(?:\.@?[a-z0-9_][a-z0-9_]*)*)|(?P<p>[(),]))")


def expression_problems(ctx, expr: str) -> list:
    toks, i, s = [], 0, expr
    while i < len(s):
        if s[i:].strip() == "":
            break
        m = TOKEN_RX.match(s, i)
        if not m or m.end() == i:
            return [f"expression {expr!r}: cannot read {s[i:].strip()[:20]!r} (S2.2's grammar)"]
        kind = m.lastgroup
        toks.append((kind, m.group(kind)))
        i = m.end()
    out = []
    pos = [0]

    def peek():
        return toks[pos[0]] if pos[0] < len(toks) else (None, None)

    def take():
        t = peek()
        pos[0] += 1
        return t

    def parse():
        kind, text = take()
        if kind == "hex":
            if not HEX_LITERAL.match(text):
                out.append(f"`{text}` is not a colour literal (#rrggbb, #rrggbbaa)")
            return
        if kind == "num":
            return
        if kind != "path":
            raise ValueError(f"expected a literal, a reference or a call, found {text!r}")
        if peek() == ("p", "("):
            take()
            if text in LITERAL_FORMS:
                n = 0
                while True:
                    k, v = take()
                    if k != "num" or not re.fullmatch(NUM, v):
                        raise ValueError(f"{text}() takes number literals")
                    n += 1
                    k, v = take()
                    if v == ")":
                        break
                    if v != ",":
                        raise ValueError(f"expected `,` or `)` in {text}()")
                if n not in (3, 4):
                    out.append(f"{text}() takes three or four numbers")
                return
            if text == "active":
                k, v = take()
                if k != "path" or ref_problem(ctx, v) or v.split(".")[0] not in ("role", "surface"):
                    out.append(f"active() takes a colour role reference, not {v!r}")
                if take() != ("p", ")"):
                    raise ValueError("active() takes one reference")
                return
            if "." in text or text not in ctx.functions:
                out.append(f"`{text}` is not a function of S2.2's table")
            n = 0
            while True:
                parse()
                n += 1
                k, v = take()
                if v == ")":
                    break
                if v != ",":
                    raise ValueError(f"expected `,` or `)` in {text}()")
            lo, hi = ctx.functions.get(text, (n, n))
            if n < lo or (hi is not None and n > hi):
                want = f"{lo} or more" if hi is None else str(lo)
                out.append(f"`{text}` takes {want} argument{'s' * (want != '1')} (S2.2's table), not {n}")
            return
        problem = ref_problem(ctx, text)
        if problem:
            out.append(problem)

    try:
        parse()
        if pos[0] != len(toks):
            raise ValueError(f"unexpected {toks[pos[0]][1]!r}")
    except ValueError as err:
        return [f"expression {expr!r}: {err}"]
    return [f"expression {expr!r}: {p}" for p in out]


def ref_problem(ctx, path):
    segs = path.split(".")
    head = segs[0]
    ck = ctx.ck
    if head in RESERVED:
        return None if len(segs) == 1 else f"`{path}`: `{head}` takes no path"
    if len(segs) == 1:
        return None if NAME.match(head) else f"`{path}` is not a name"
    if head == "role":
        if segs[1:3] == ["global", "accent_ramp"] and len(segs) > 4:
            return f"`{path}`: an accent ramp role is `role.global.accent_ramp.<key>` (S3; ADD-A122)"
        return None if ck.role_ok(".".join(segs[1:])) else f"`{path}` names no colour role (S3)"
    if head == "token":
        return None if ck.token_ok(".".join(segs[1:])) else f"`{path}` names no token (S4)"
    if head == "surface":
        role = segs[2:] if segs[1] == "disabled" else segs[1:]
        return None if len(role) == 1 and role[0] in ck.v["surface_roles"] else f"`{path}` names no surface role"
    if head == "element":
        return None if len(segs) == 2 and segs[1] in ck.v["element_data"] else f"`{path}` names no element data"
    if head in ("input", "part", "values"):
        return None
    if head == "widget":
        return None if len(segs) >= 3 else f"`{path}` is not `widget.<part>.<field>`"
    return f"`{path}` is not a reference (S2.2)"


def dim_name(dim):
    return "a text value" if dim == "text" else f"a {dim.replace('_', ' ')}"


def expression_refs(expr: str) -> list:
    """The references of an expression as written there (its paths that are not a call's name), in order."""
    toks = [(m.lastgroup, m.group(m.lastgroup), m.end()) for m in TOKEN_RX.finditer(expr)]
    out = []
    for i, (kind, text, _) in enumerate(toks):
        if kind == "path" and not (i + 1 < len(toks) and toks[i + 1][:2] == ("p", "(")) and text not in out:
            out.append(text)
    return out


def text_expression_problems(ctx, expr: str, where: str) -> list:
    """A text value's expression is one reference to a text field (S11's text row; S12 refuses a call)."""
    m = re.fullmatch(r"\s*(@?[a-z_][a-z0-9_]*(?:\.@?[a-z0-9_][a-z0-9_]*)*)\s*", expr)
    if not m:
        return [f"{where}: a text value's expression is one reference to a text field (S11; S12 refuses a call)"]
    ref = m.group(1)
    if ref.split(".")[0] in ("token", "role"):
        problem, dim = key_info(ctx, ref)[:2]
        if not problem and dim is not None and dim != "text":
            return [f"{where}: `{ref}` is a {dim.replace('_', ' ')}, not a text field"]
    return []


def literal_enum(ctx, typ):
    """The core enum a value of type `typ` is, where S11 spells it only as a variant (its name or a one-key table, a
    reference, a keyword): a `Timing` (S4.1's `token.motion.timings.<key>`, S2.7; ADD-A123), a `PlatformCurve`; None
    for any other type, and for an enum with a variant holding one scalar, whose literal spells it (`Radius::Px`)."""
    e = strip_type(typ)
    variants = ctx.enums.get(e)
    if variants is None or any(form == "tuple" and strip_type(payload) in KEY_DIMS
                               for form, payload in variants.values()):
        return None
    return e


def value_problems(ctx, value, dim, where="value", typ=None) -> list:
    """Problems of a value spelled for a key of dimension `dim`; `typ` is the key's core type (S11's text row reads
    it: a text value inside a table or a list is found by its type)."""
    kind = text_kind(typ)
    if kind:
        dim = "text"
    e = literal_enum(ctx, typ) if dim is None else None
    if e and (isinstance(value, (bool, int, float, list)) or (isinstance(value, str) and (
            HEX_LITERAL.match(value) or FUNC_COLOUR.match(value) or UNIT_LITERAL.match(value)))):
        return [f"{where}: {value!r} where the key is a {e}, an enum (S11: a variant's snake_case name or a one-key "
                f"table naming it)"]
    if isinstance(value, str) and dim == "text":
        if kind == "plain":
            return []
        if value.startswith("="):
            out = [f"{where}: {p}" for p in expression_problems(ctx, value[1:])]
            return out or text_expression_problems(ctx, value[1:], where)
        return []
    if isinstance(value, bool):
        if dim not in (None, "bool"):
            return [f"{where}: a bool where the key is {dim_name(dim)}"]
        return []
    if isinstance(value, (int, float)):
        if isinstance(value, float) and not math.isfinite(value):
            return [f"{where}: a non-finite number"]
        if dim in UNIT_OF:
            return [f"{where}: a {dim.replace('_', ' ')} is spelled with its unit (\"{value}{UNIT_OF[dim]}\")"]
        if dim in ("colour", "paint", "bool", "text"):
            return [f"{where}: a number where the key is {dim_name(dim)}"]
        return []
    if isinstance(value, str):
        s = value
        if s.startswith("="):
            return [f"{where}: {p}" for p in expression_problems(ctx, s[1:])]
        if s in KEYWORDS:
            return []
        if HEX_LITERAL.match(s) or FUNC_COLOUR.match(s):
            if dim not in (None, "colour", "paint"):
                return [f"{where}: a colour where the key is {dim_name(dim)}"]
            return []
        m = UNIT_LITERAL.match(s)
        if m:
            if dim is not None and dim not in UNIT_DIMS[m.group(2)]:
                return [f"{where}: {s!r} is not a {dim.replace('_', ' ')}"]
            return []
        if NUMBER_STRING.match(s):
            return [f"{where}: {s!r}: a number is a TOML number, a length, duration, angle or font size a string "
                    f"with S2.2's unit"]
        if NAME.match(s):
            if dim in SCALAR_DIMS:
                return [f"{where}: the name {s!r} where the key is {dim_name(dim)}"]
            e = strip_type(typ)
            if e in ctx.enums and not enum_name_ok(ctx, e, s):
                return [f"{where}: {s!r} is not a unit variant of {e} (S11: its snake_case name)"]
            return []
        if s.startswith("#"):
            return [f"{where}: {s!r} is not a colour literal (#rrggbb, #rrggbbaa)"]
        return [f"{where}: {s!r} is not a value as S11 spells it (a free string is a text value only where the "
                f"key's type is text)"]
    if isinstance(value, list):
        if dim == "text":
            return [f"{where}: a list where the key is a text value"]
        it = item_type(typ)
        out = []
        for i, v in enumerate(value):
            out += value_problems(ctx, v, None, f"{where}[{i}]", it)
        return out
    if isinstance(value, dict):
        if dim in SCALAR_DIMS:
            return [f"{where}: a table where the key is {dim_name(dim)}"]
        out = []
        e = strip_type(typ)
        if e in ctx.enums:
            data = sorted(k for k, (form, _) in ctx.enums[e].items() if form != "unit")
            if len(value) != 1 or next(iter(value)) not in data:
                out.append(f"{where}: a variant of {e} with data is a one-key table naming it ({', '.join(data)}; S11)")
        elif "::" in e and e in ctx.structs:
            fields = ctx.structs[e]
            out += [f"{where}.{k}: `{k}` is not a field of {e}" for k in value
                    if k not in fields and re.sub(r"_(px|ms|deg|pt)$", "", k) not in fields]
        for k, v in value.items():
            w = f"{where}.{k}"
            if not NAME.match(k):
                out.append(f"{w}: the key {k!r} is not snake_case")
                continue
            sub = next((d for suf, d in SUFFIX_DIMS.items() if k.endswith(suf)), None)
            if sub:
                if isinstance(v, bool) or not (isinstance(v, (int, float)) or (
                        isinstance(v, str) and (v.startswith("=") or v in KEYWORDS))):
                    out.append(f"{w}: a suffixed key takes a plain number, an expression, \"auto\", \"absent\" or "
                               f"\"none\"")
                    continue
                if isinstance(v, str) and v.startswith("="):
                    out += value_problems(ctx, v, None, w)
                elif isinstance(v, float) and not math.isfinite(v):
                    out.append(f"{w}: a non-finite number")
                continue
            out += value_problems(ctx, v, None, w, child_type(ctx, typ, k) if typ else None)
        return out
    return [f"{where}: {type(value).__name__} is not a value"]


# ------------------------------------------------------------------------------------------------- when

def str_list(v):
    return isinstance(v, list) and all(isinstance(x, str) for x in v)


def when_problems(ctx, when, widget, platform, only_conditions=False, where="when") -> list:
    if not isinstance(when, dict):
        return [f"{where} is not a table"]
    ck = ctx.ck
    out = [f"{where}: unknown key `{k}`" for k in when if k not in WHEN_KEYS]
    if only_conditions:
        out += [f"{where}: `{k}` on a colour-role, token or input key (only conditions, S7)"
                for k in when if k in WHEN_KEYS and k not in ("conditions", "not_conditions")]
    widgets = [widget] if widget else []
    for k in ("conditions", "not_conditions"):
        if k in when:
            if not str_list(when[k]):
                out.append(f"{where}.{k} is not a list of strings")
                continue
            for c in when[k]:
                if c.startswith("!") or not ck.cond_ok(c):
                    out.append(f"{where}.{k}: {c!r} is not a condition (S7.1, S11)")
                elif c.startswith("preset:") and platform in PLATFORMS and c[7:] not in PLATFORMS[platform]:
                    out.append(f"{where}.{k}: {c!r} is not a preset of {platform}/ ({', '.join(PLATFORMS[platform])})")
    for k in ("states", "not_states"):
        if k in when:
            out += names_problems(when[k], f"{where}.{k}", lambda s: not s.startswith("!") and ck.flag_ok(s, widgets),
                                  "a state flag")
    for k in ("variants", "not_variants"):
        if k in when:
            out += names_problems(when[k], f"{where}.{k}",
                                  lambda s: not s.startswith("!") and ck.variant_ok(s, widgets),
                                  f"a variant of {widget}")
    for k in ("sizes", "not_sizes"):
        if k in when:
            out += names_problems(when[k], f"{where}.{k}", lambda s: s in ck.v["sizes"] or KEYED.match(s), "a size")
    for k in ("densities", "not_densities"):
        if k in when:
            out += names_problems(when[k], f"{where}.{k}", lambda s: s in ck.v["densities"] or KEYED.match(s),
                                  "a density")
    if "size_class" in when:
        sc = when["size_class"]
        if not isinstance(sc, dict):
            out.append(f"{where}.size_class is not a table")
        else:
            for k, v in sc.items():
                if k not in SIZE_CLASS_KEYS:
                    out.append(f"{where}.size_class: unknown key `{k}`")
                else:
                    out += names_problems(v, f"{where}.size_class.{k}", lambda s: s in ck.v["size_classes"],
                                          "a size class")
    if "direction" in when and when["direction"] not in ("ltr", "rtl"):
        out.append(f"{where}.direction is not \"ltr\" or \"rtl\"")
    for k in ("context", "not_context"):
        if k in when:
            if not isinstance(when[k], list):
                out.append(f"{where}.{k} is not a list of tables")
                continue
            for i, c in enumerate(when[k]):
                out += context_problems(ctx, c, f"{where}.{k}[{i}]")
    if "position" in when:
        out += position_problems(ctx, when["position"], f"{where}.position")
    if "latest" in when:
        out += latest_form_problems(ctx, when, widgets, where)
    return out


def latest_form_problems(ctx, when, widgets, where) -> list:
    """A `when.latest` table's form (S11, S7.2; ADD-A124): `{ <set> = "<flag>" }`, each set's name a key (S1) and its
    flag a state flag that the `when`'s own `not_states` does not refuse (`latest` implies the flag, S7.2)."""
    lt = when["latest"]
    if not isinstance(lt, dict) or not lt or not all(isinstance(v, str) for v in lt.values()):
        return [f"{where}.latest is not a table of recency sets and their latest flags (S11: latest = "
                f"{{ <set> = \"<flag>\" }})"]
    nots = when.get("not_states") if str_list(when.get("not_states")) else []
    out = []
    for name, flag in lt.items():
        if not KEY_RX.match(name):
            out.append(f"{where}.latest: {name!r} is not a recency set's name (a key, S1)")
        if flag.startswith("!") or not ctx.ck.flag_ok(flag, widgets):
            out.append(f"{where}.latest.{name}: {flag!r} is not a state flag")
        elif flag in nots:
            out.append(f"{where}.latest.{name}: {flag!r} is among the when's not_states, which `latest` contradicts "
                       f"(it implies the flag, S7.2)")
    return out


def names_problems(v, where, ok, what):
    if not str_list(v):
        return [f"{where} is not a list of strings"]
    return [f"{where}: {s!r} is not {what}" for s in v if not ok(s)]


def widget_ok(ctx, w):
    return isinstance(w, str) and w in ctx.entries


def part_problems(ctx, d, where):
    out = []
    w = d.get("widget")
    if "widget" in d and not widget_ok(ctx, w):
        out.append(f"{where}.widget: {w!r} is not a catalogue entry")
    if "part" in d:
        if "widget" not in d:
            out.append(f"{where}.part without widget (S11)")
        elif widget_ok(ctx, w) and not any(d["part"] in p.names for p in ctx.entries[w].parts):
            out.append(f"{where}.part: {d['part']!r} is not a part of {w}")
    ws = [w] if widget_ok(ctx, w) else []
    for k in ("variants", "not_variants"):
        if k in d:
            out += names_problems(d[k], f"{where}.{k}", lambda s: ctx.ck.variant_ok(s, ws), "a variant")
    for k in ("states", "not_states"):
        if k in d:
            out += names_problems(d[k], f"{where}.{k}", lambda s: ctx.ck.flag_ok(s, ws), "a state flag")
    return out


def context_problems(ctx, c, where):
    if not isinstance(c, dict):
        return [f"{where} is not a table"]
    out = [f"{where}: unknown key `{k}`" for k in c if k not in CONTEXT_KEYS]
    scope = c.get("scope", "any")
    if scope != "any":
        if not (isinstance(scope, dict) and set(scope) == {"nearest"} and isinstance(scope["nearest"], list)):
            out.append(f"{where}.scope is not \"any\" or {{ nearest = [<widgets>] }}")
        else:
            out += [f"{where}.scope.nearest: {w!r} is not a catalogue entry" for w in scope["nearest"]
                    if not widget_ok(ctx, w)]
    if "surface" in c and c["surface"] not in ctx.ck.v["surface_sets"]:
        out.append(f"{where}.surface: {c['surface']!r} is not a surface set")
    for k in ("sizes", "not_sizes"):
        if k in c:
            out += names_problems(c[k], f"{where}.{k}", lambda s: s in ctx.ck.v["sizes"] or KEYED.match(s), "a size")
    out += part_problems(ctx, {k: v for k, v in c.items() if k in SIBLING_KEYS}, where)
    return out


def position_problems(ctx, p, where):
    if not isinstance(p, dict):
        return [f"{where} is not a table"]
    out = [f"{where}: unknown key `{k}`" for k in p if k not in POSITION_KEYS]
    among = p.get("among", "same")
    if among not in ("same", "any") and not (isinstance(among, list) and all(widget_ok(ctx, w) for w in among)):
        out.append(f"{where}.among is not \"same\", \"any\" or a list of widgets")
    for k in ("first", "last", "only"):
        if k in p and not isinstance(p[k], bool):
            out.append(f"{where}.{k} is not a bool")
    if "parity" in p and p["parity"] not in ("odd", "even"):
        out.append(f"{where}.parity is not \"odd\" or \"even\"")
    for k in ("previous", "next"):
        if k in p:
            if not isinstance(p[k], dict):
                out.append(f"{where}.{k} is not a table")
                continue
            out += [f"{where}.{k}: unknown key `{x}`" for x in p[k] if x not in SIBLING_KEYS]
            out += part_problems(ctx, p[k], f"{where}.{k}")
    return out


# ---------------------------------------------------------------------------------------------- citations

def conversions():
    import check_fact_citations
    return check_fact_citations.CONVERSIONS


def cite_problems(ctx, item: Item, cite: Cite) -> list:
    c = cite.data
    out = []
    kind = c.get("kind")
    if not isinstance(kind, str) or kind not in CITE_FIELDS:
        return [(cite.ln("kind"), f"citation kind {kind!r} is not one of {', '.join(CITE_FIELDS)}")]
    required, optional = CITE_FIELDS[kind]
    out += [(cite.line, f"{kind} citation: missing `{f}`") for f in sorted(required - c.keys())]
    out += [(cite.ln(f), f"{kind} citation: `{f}` is not allowed (S17)") for f in sorted(c.keys() - required - optional
                                                                                         - {"kind"})]
    if "source" in c:
        src = parse_source(c["source"])
        if not src:
            out.append((cite.ln("source"), f"source {c['source']!r} is not `<id> <path>:<line or l1-l2>` or "
                                           f"`<id> #<JSON pointer>`"))
        else:
            if src[0] not in ctx.sources:
                out.append((cite.ln("source"), f"source id {src[0]!r} is not a [[source]] of sources.toml"))
            elif item.palette and not (ctx.sources[src[0]].get("generations") == []
                                       and ctx.sources[src[0]].get("palette") is True):
                out.append((cite.ln("source"), f"a palette fact's source {src[0]!r} is not a palette entry "
                                               f"(generations = [], palette = true)"))
            if src[1] is not None and (src[1].startswith("/") or ".." in src[1].split("/")):
                out.append((cite.ln("source"), f"source path {src[1]!r} is not relative to its checkout"))
            if src[2] is not None and src[3] < src[2]:
                out.append((cite.ln("source"), f"source range {src[2]}-{src[3]} runs backwards"))
            if src[4] is not None and src[4] and not src[4].startswith("/"):
                out.append((cite.ln("source"), f"JSON pointer {src[4]!r} does not start with `/`"))
    if "quote" in c and not (isinstance(c["quote"], str) and c["quote"].strip()):
        out.append((cite.ln("quote"), "quote is not a non-empty string"))
    if "conversion" in c:
        conv = c["conversion"]
        if not str_list(conv) or not conv:
            out.append((cite.ln("conversion"), "conversion is not a non-empty list of names"))
        else:
            known = conversions()
            out += [(cite.ln("conversion"), f"conversion {x!r} is not one of {', '.join(known)}") for x in conv
                    if x not in known]
    if "absent" in c and not (str_list(c["absent"]) and all(x.strip() for x in c["absent"])):
        out.append((cite.ln("absent"), "absent is not a list of names (`[]` where the platform has no name for it)"))
    if kind == "measured":
        out += inputs_problems(ctx, item, cite)
    if "tolerance" in c:
        t = c["tolerance"]
        if isinstance(t, dict):
            for k, v in t.items():
                if isinstance(v, bool) or not isinstance(v, (int, float)) or not v >= 0:
                    out.append((cite.ln("tolerance"), f"tolerance {k!r} is not a number >= 0"))
        elif isinstance(t, bool) or not isinstance(t, (int, float)) or not t >= 0:
            out.append((cite.ln("tolerance"), "tolerance is not a number >= 0 or a table of them"))
    if "evidence" in c and (not parse_evidence(c["evidence"]) or ".." in c["evidence"].split("#")[0].split("/")):
        out.append((cite.ln("evidence"), f"evidence {c['evidence']!r} is not "
                                         f"`docs/evidence/<harness>/<date>-run-<id>/<image>/<file>#<path>`"))
    return out


def inputs_problems(ctx, item: Item, cite: Cite) -> list:
    """S17 *A measured derivation*: `inputs` only on an expression value's measured citation, and such a citation
    always with them, each a reference of the expression as written there, none twice."""
    c = cite.data
    value = item.data.get("value") if item.table == "fact" and not item.palette else None
    expr = isinstance(value, str) and value.startswith("=")
    if "inputs" not in c:
        return [(cite.line, "measured citation of an expression value: missing `inputs` (the references the harness "
                            "varied, S17 *A measured derivation*)")] if expr else []
    if not expr:
        return [(cite.ln("inputs"), "`inputs` is only on a measured citation of an expression value (S17)")]
    inputs = c["inputs"]
    if not str_list(inputs) or not inputs:
        return [(cite.ln("inputs"), "inputs is not a non-empty list of references")]
    refs = expression_refs(value[1:])
    out = [(cite.ln("conversion"), "a measured derivation has no `conversion` beside `inputs` (S17)")] \
        if "conversion" in c else []
    out += [(cite.ln("inputs"), f"input {x!r} is not a reference of the expression {value!r}") for x in inputs
           if x not in refs]
    out += [(cite.ln("inputs"), f"input {x!r} listed twice") for x in sorted({x for x in inputs
                                                                             if inputs.count(x) > 1})]
    return out


def generation_problems(ctx, item: Item) -> list:
    if item.palette:
        return []
    covered, preset = ctx.covered(item)
    gens = [(c, ctx.cite_generations(c.data)) for c in item.cites]
    out = []
    if preset:
        for c, g in gens:
            if preset not in g:
                out.append((c.line, f"this citation does not serve {preset} (it serves "
                                    f"{', '.join(sorted(g)) or 'no generation'})"))
        return out
    served = set().union(*(g for _, g in gens)) if gens else set()
    missing = sorted(covered - served)
    if missing:
        out.append((item.line, f"no citation serves {', '.join(missing)} ({item.platform}/ covers "
                               f"{', '.join(sorted(covered))}; split by `preset:` or cite a source for each)"))
    return out


def rows_problems(ctx, item: Item) -> list:
    rows = item.data.get("rows")
    if not str_list(rows):
        return [(item.ln("rows"), "rows is not a list of gap-map row IDs")]
    out = []
    for r in rows:
        if not ROW_RX.match(r) or not re.search(r"\d", r):
            out.append((item.ln("rows"), f"row {r!r} is not a gap-map row ID"))
        elif ctx.rows is not None and r not in ctx.rows:
            out.append((item.ln("rows"), f"row {r!r} is in no gap map (as the gap maps write it, † included)"))
    out += [(item.ln("rows"), f"row {r!r} listed twice") for r in sorted({r for r in rows if rows.count(r) > 1})]
    return out


def fact_problems(ctx, item: Item) -> list:
    d = item.data
    out = []
    if item.palette:
        out += [(item.line, f"palette fact: missing `{f}`") for f in sorted(PALETTE_REQUIRED - d.keys())]
        out += [(item.ln(f), f"palette fact: `{f}` is not allowed (no rows, when, scheme_file or accent)")
                for f in sorted(d.keys() - PALETTE_REQUIRED)]
        stem = Path(item.file).stem
        key = d.get("key")
        if "key" in d and not (isinstance(key, str) and re.fullmatch(
                rf"{re.escape(stem)}\.[a-z0-9][a-z0-9_-]*\.[a-z0-9][a-z0-9_-]*", key)):
            out.append((item.ln("key"), f"palette key {key!r} is not `{stem}.<flavour>.<name>`"))
        if "value" in d and not (isinstance(d["value"], str) and (HEX_LITERAL.match(d["value"])
                                                                  or FUNC_COLOUR.match(d["value"]))):
            out.append((item.ln("value"), f"palette value {d['value']!r} is not a colour literal"))
    elif "pending" in d:
        out += [(item.line, f"pending fact: missing `{f}`") for f in sorted(PENDING_REQUIRED - d.keys())]
        out += [(item.ln(f), f"`{f}` is not a fact's field") for f in sorted(d.keys() - FACT_REQUIRED - FACT_OPTIONAL
                                                                          - PENDING_REQUIRED)]
        if "value" in d:
            out.append((item.ln("value"), "a pending fact has `pending` in place of `value`: exactly one of them "
                                          "(S17 *A pending measurement*)"))
        if "cite" in d:
            out.append((item.ln("cite"), "a pending fact has no [[fact.cite]] (S17 *A pending measurement*)"))
        pid = d["pending"]
        if not (isinstance(pid, str) and PENDING_RX.match(pid)):
            out.append((item.ln("pending"), f"pending {pid!r} is not an Unmeasured register Id (`U` and a number, "
                                            f"S17)"))
        _, widget, kind, _ = fact_key(ctx, item, out)
        return out + fact_shared_problems(ctx, item, widget, kind)
    else:
        out += [(item.line, f"missing `{f}`") for f in sorted(FACT_REQUIRED - d.keys())]
        out += [(item.ln(f), f"`{f}` is not a fact's field") for f in sorted(d.keys() - FACT_REQUIRED - FACT_OPTIONAL)]
        dim, widget, kind, typ = fact_key(ctx, item, out)
        if "value" in d and typ == PART_PATCH:
            out += [(item.ln("value"), p) for p in part_patch_problems(ctx, d["value"], widget)]
        elif "value" in d:
            out += [(item.ln("value"), p) for p in value_problems(ctx, d["value"], dim, typ=typ)]
            out += [(item.ln("value"), p) for p in transition_problems(ctx, d["key"], d["value"], typ, widget)]
            if isinstance(d["value"], str) and text_leaves(ctx, d["value"], typ):
                out += [(c.ln("tolerance"), "a text value compares exactly: its measured citation's tolerance is 0 "
                                            "(S11)") for c in item.cites if c.data.get("kind") == "measured"
                        and "tolerance" in c.data and c.data["tolerance"] != 0]
        out += fact_shared_problems(ctx, item, widget, kind)
        if any(c.data.get("kind") == "read" for c in item.cites):
            v = d.get("value")
            if not (isinstance(v, str) and re.fullmatch(r"=\s*input\.[a-z0-9_.@]+\s*", v)):
                out.append((item.ln("value"), "a fact with a `read` citation states a plain `= input.…` reference"))
    out += cite_list_problems(item, "fact")
    for c in item.cites:
        out += cite_problems(ctx, item, c)
    out += generation_problems(ctx, item)
    return out


def transition_tables(ctx, value, t, in_list=False, path="value"):
    """(path, table, whether it is an entry of a list) of every `Transition` table in `value` under the core's type
    `t` (S2.7)."""
    s = strip_type(t) if t else None
    if isinstance(value, dict) and s == "Transition":
        yield path, value, in_list
    elif isinstance(value, list):
        it = item_type(t) if t else None
        for i, v in enumerate(value):
            yield from transition_tables(ctx, v, it, True, f"{path}[{i}]") if it else ()
    elif isinstance(value, dict) and t:
        for k, v in value.items():
            yield from transition_tables(ctx, v, child_type(ctx, t, k), False, f"{path}.{k}")


def part_fields(ctx, key):
    """The catalogue's field list of the part a widget or component key names (`<widget>.<part>.…`), or None."""
    parsed = key_segments(key) if isinstance(key, str) else None
    if not isinstance(parsed, tuple):
        return None
    segs = parsed[0]
    widget, rest = (f"component.{segs[1]}", segs[2:]) if segs[0] == "component" and len(segs) > 1 else (segs[0],
                                                                                                         segs[1:])
    entry = ctx.entries.get(widget)
    part = next((p for p in entry.parts if rest and rest[0] in p.names), None) if entry else None
    return [check_spec_names.norm_value(f) for f in part.fields] if part else None


def transition_problems(ctx, key, value, typ, widget) -> list:
    """A transition's `from` (`Transition.from`, S2.7, S11, S12; ADD-A124): stated in a single transition (`"none"` for
    any state), left out of a list's entry only; never an empty list, no flag twice, no keyed flag the widget does not
    declare; and a `state_layer` transition only on a part whose fields hold `interaction` (S12)."""
    out = []
    fields = part_fields(ctx, key)
    for path, tr, in_list in transition_tables(ctx, value, typ):
        frm = tr.get("from")
        if "from" not in tr and not in_list:
            out.append(f"{path}: a single transition states `from` (S11: \"none\" for any state)")
        if isinstance(frm, list):
            if not frm:
                out.append(f"{path}.from is an empty list (it would apply from no state, S12)")
            out += [f"{path}.from names {f!r} twice (S12)" for f in sorted({f for f in frm if isinstance(f, str)
                                                                             and frm.count(f) > 1})]
            out += [f"{path}.from: {f!r} is not a flag {widget} declares (S12)" for f in dict.fromkeys(frm)
                    if widget and isinstance(f, str) and KEYED.match(f) and not ctx.ck.flag_ok(f, [widget])]
        props = tr.get("properties") if isinstance(tr.get("properties"), list) else []
        if "state_layer" in props and fields is not None and "interaction" not in fields:
            out.append(f"{path}: a `state_layer` transition on a part whose fields lack `interaction` (S12)")
    return out


def fact_key(ctx, item: Item, out: list):
    """(dimension, widget, kind, the core's type) of a fact's key, its problems appended to `out` (S4.1's motion keys
    among them)."""
    d = item.data
    if "key" not in d:
        return None, None, None, None
    problem, dim, widget, kind, typ = key_info(ctx, d["key"])
    if problem:
        out.append((item.ln("key"), problem))
    out += [(item.ln("key"), p) for p in motion_key_problems(ctx, d["key"], item.platform)]
    return dim, widget, kind, typ


def fact_shared_problems(ctx, item: Item, widget, kind) -> list:
    """What a fact and a pending fact share: `rows`, `when` and the accent run's fields."""
    d = item.data
    out = []
    if "rows" in d:
        out += rows_problems(ctx, item)
    if "when" in d:
        out += [(item.ln("when"), p) for p in when_problems(ctx, d["when"], widget, item.platform,
                                                           only_conditions=kind in ("role", "token", "input"))]
    sf, acc = "scheme_file" in d, "accent" in d
    if sf != acc:
        out.append((item.ln("scheme_file" if sf else "accent"), "scheme_file and accent come both or neither"))
    if (sf or acc) and item.platform != "kde":
        out.append((item.ln("scheme_file" if sf else "accent"), "scheme_file and accent are the kde/ accent "
                                                                 "run's only"))
    if sf:
        m = re.fullmatch(r"(\S+) (\S+)", d["scheme_file"]) if isinstance(d["scheme_file"], str) else None
        if not m:
            out.append((item.ln("scheme_file"), "scheme_file is not `<sources.toml id> <path>`"))
        elif m.group(1) not in ctx.sources:
            out.append((item.ln("scheme_file"), f"scheme_file's entry {m.group(1)!r} is not a [[source]]"))
    if acc and not (isinstance(d["accent"], str) and (HEX_LITERAL.match(d["accent"])
                                                      or FUNC_COLOUR.match(d["accent"]))):
        out.append((item.ln("accent"), f"accent {d['accent']!r} is not a colour literal"))
    return out


def cite_list_problems(item: Item, table: str) -> list:
    raw = item.data.get("cite")
    if "cite" not in item.data:
        return []
    if not isinstance(raw, list) or not all(isinstance(c, dict) for c in raw):
        return [(item.ln("cite"), f"cite is not an array of [[{table}.cite]] tables")]
    return [] if raw else [(item.line, f"no [[{table}.cite]]")]


def model_problems(ctx, item: Item) -> list:
    d = item.data
    out = [(item.line, f"missing `{f}`") for f in sorted(MODEL_REQUIRED - d.keys())]
    out += [(item.ln(f), f"`{f}` is not a state-model entry's field") for f in
            sorted(d.keys() - MODEL_REQUIRED - MODEL_DECLARATIONS - {"when"})]
    widget = d.get("widget")
    entry = ctx.entries.get(widget) if isinstance(widget, str) else None
    if "widget" in d and entry is None:
        out.append((item.ln("widget"), f"widget {widget!r} is not a catalogue entry"))
    ws = [widget] if entry else []
    if "when" in d:
        when = d["when"]
        if not isinstance(when, dict) or set(when) - {"conditions"} or not str_list(when.get("conditions")) or any(
                not c.startswith(("preset:", "flavor:")) for c in when["conditions"]):
            out.append((item.ln("when"), "a state-model entry's when holds only `preset:` or `flavor:` conditions"))
        else:
            out += [(item.ln("when"), p) for p in when_problems(ctx, when, widget, item.platform)]
    if "rows" in d:
        out += rows_problems(ctx, item)
    avail = d.get("available")
    if "available" in d and not isinstance(avail, bool):
        out.append((item.ln("available"), "available is not a bool"))
    decl = d.keys() & MODEL_DECLARATIONS
    if avail is False:
        out += [(item.ln(k), f"`{k}` on an entry with available = false (no other declaration)") for k in sorted(decl)]
    elif avail is True:
        out += [(item.line, f"missing `{k}` (an available widget's entry states it)")
                for k in sorted(MODEL_AVAILABLE_REQUIRED - d.keys())]
        ck = ctx.ck
        variants = d.get("variants", []) if str_list(d.get("variants", [])) else []
        sizes = d.get("sizes", []) if str_list(d.get("sizes", [])) else []
        if "variants" in d:
            out += [(item.ln("variants"), p) for p in names_problems(variants, "variants",
                                                                       lambda s: ck.variant_ok(s, ws), "a variant")]
        if "sizes" in d:
            out += [(item.ln("sizes"), p) for p in names_problems(sizes, "sizes",
                                                                    lambda s: s in ck.v["sizes"] or KEYED.match(s),
                                                                    "a size")]
        if "densities" in d:
            out += [(item.ln("densities"), p) for p in names_problems(
                d["densities"], "densities", lambda s: s in ck.v["densities"] or KEYED.match(s), "a density")]
        if "flags" in d:
            out += [(item.ln("flags"), p) for p in names_problems(d["flags"], "flags", lambda s: ck.flag_ok(s, ws),
                                                                    "a flag of the entry")]
        states = []
        if "axes" in d:
            axes = d["axes"]
            if not isinstance(axes, dict) or set(axes) != AXES_KEYS:
                out.append((item.ln("axes"), "axes is not { states, conditions, contexts }"))
            else:
                states = axes["states"] if str_list(axes["states"]) else []
                out += [(item.ln("axes"), p) for p in names_problems(axes["states"], "axes.states",
                                                                       lambda s: ck.flag_ok(s, ws), "a state flag")]
                out += [(item.ln("axes"), p) for p in names_problems(
                    axes["conditions"], "axes.conditions",
                    lambda s: ck.cond_ok(s) and not s.startswith("!"), "a condition")]
                if not isinstance(axes["contexts"], list):
                    out.append((item.ln("axes"), "axes.contexts is not a list of context selectors"))
                else:
                    for i, c in enumerate(axes["contexts"]):
                        out += [(item.ln("axes"), p) for p in context_problems(ctx, c, f"axes.contexts[{i}]")]
        if "exclusive" in d:
            ex = d["exclusive"]
            if not (isinstance(ex, list) and all(str_list(g) for g in ex)):
                out.append((item.ln("exclusive"), "exclusive is not a list of lists of states"))
            else:
                seen = set()
                for g in ex:
                    for s in g:
                        if "axes" in d and s not in states:
                            out.append((item.ln("exclusive"), f"exclusive: {s!r} is not in axes.states"))
                        if s in seen:
                            out.append((item.ln("exclusive"), f"exclusive: {s!r} is in two groups"))
                        seen.add(s)
        if "not_drawn" in d:
            nd = d["not_drawn"]
            if not isinstance(nd, list):
                out.append((item.ln("not_drawn"), "not_drawn is not a list of whens"))
            else:
                for i, w in enumerate(nd):
                    out += [(item.ln("not_drawn"), p) for p in when_problems(ctx, w, widget, item.platform,
                                                                              where=f"not_drawn[{i}]")]
        if "default_size" in d:
            if not str_list(sizes) or not sizes:
                out.append((item.ln("default_size"), "default_size where sizes is empty"))
            elif d["default_size"] not in sizes:
                out.append((item.ln("default_size"), f"default_size {d['default_size']!r} is not among sizes"))
        if "requires_variants" in d:
            out += [(item.ln("requires_variants"), p) for p in names_problems(
                d["requires_variants"], "requires_variants", lambda s: s in variants, "among variants")]
        for k, sub in (("sizes_by_variant", "variant"), ("sizes_by_context", "context")):
            if k not in d:
                continue
            if not isinstance(d[k], list) or not all(isinstance(x, dict) and set(x) == {sub, "sizes"} for x in d[k]):
                out.append((item.ln(k), f"{k} is not a list of {{ {sub}, sizes }}"))
                continue
            for x in d[k]:
                if sub == "variant" and x[sub] not in variants:
                    out.append((item.ln(k), f"{k}: {x[sub]!r} is not among variants"))
                if sub == "context" and not widget_ok(ctx, x[sub]):
                    out.append((item.ln(k), f"{k}: {x[sub]!r} is not a catalogue entry"))
                out += [(item.ln(k), p) for p in names_problems(x["sizes"], f"{k}.sizes", lambda s: s in sizes,
                                                                  "among sizes")]
        out += variant_declaration_problems(item, variants, sizes)
        out += recency_problems(item, states, ws, ck)
    out += cite_list_problems(item, "model")
    for c in item.cites:
        out += cite_problems(ctx, item, c)
    out += generation_problems(ctx, item)
    return out


def recency_of(d) -> dict:
    """{name: [flags in the stated order]} of an entry's `recency` (S10, S17; ADD-A124), its well-formed sets only."""
    r = d.get("recency")
    return {k: v for k, v in r.items() if str_list(v)} if isinstance(r, dict) else {}


def recency_problems(item: Item, states, ws, ck) -> list:
    """`recency` (S10, S12, S17; ADD-A124): a table of named sets, each name a key (S1) and each set two or more flags,
    each a state flag of `axes.states`, none twice; no two sets ordering two flags oppositely."""
    d = item.data
    if "recency" not in d:
        return []
    r, ln = d["recency"], item.ln("recency")
    if not isinstance(r, dict) or not all(str_list(v) for v in r.values()):
        return [(ln, "recency is not a table of named sets, each a list of flags (S10, S17)")]
    out = []
    for name, flags in r.items():
        where = f"recency.{name}"
        if not KEY_RX.match(name):
            out.append((ln, f"{where}: {name!r} is not a key (S1)"))
        if len(set(flags)) < 2:
            out.append((ln, f"{where} holds fewer than two flags (S12)"))
        out += [(ln, f"{where} names {f!r} twice (S12)") for f in sorted({f for f in flags if flags.count(f) > 1})]
        for f in dict.fromkeys(flags):
            if f.startswith("!") or not ck.flag_ok(f, ws):
                out.append((ln, f"{where}: {f!r} is not a state flag"))
            elif "axes" in d and f not in states:
                out.append((ln, f"{where}: {f!r} is not in axes.states"))
    for (n1, f1), (n2, f2) in itertools.combinations(r.items(), 2):
        for a, b in itertools.combinations([f for f in dict.fromkeys(f1) if f in f2], 2):
            if f2.index(a) > f2.index(b):
                out.append((ln, f"recency: {n1} and {n2} order {a!r} and {b!r} oppositely (S12: a widget's sets agree "
                                f"on their orders)"))
    return out


def state_combinations(d) -> set:
    """The state combinations of an available entry (S17 *State models*): every subset of `axes.states` reduced by
    `exclusive` (each group keeping its first present flag), less those a `not_drawn` entry stating only `states` and
    `not_states` removes, each counted once."""
    axes = d.get("axes") if isinstance(d.get("axes"), dict) else {}
    states = list(dict.fromkeys(axes.get("states", []))) if str_list(axes.get("states", [])) else []
    ex = d.get("exclusive") if isinstance(d.get("exclusive"), list) else []
    groups = [g for g in ex if str_list(g)]
    nd = d.get("not_drawn") if isinstance(d.get("not_drawn"), list) else []
    gone = [(set(w.get("states", [])), set(w.get("not_states", []))) for w in nd
            if isinstance(w, dict) and w and set(w) <= {"states", "not_states"} and all(str_list(w[k]) for k in w)]
    out = set()
    for n in range(len(states) + 1):
        for sub in itertools.combinations(states, n):
            kept = set(sub)
            for g in groups:
                kept -= set([f for f in g if f in kept][1:])
            k = frozenset(kept)
            if not any(a <= k and not (no & k) for a, no in gone):
                out.add(k)
    return out


def combination_orders(k, sets) -> tuple:
    """(orders, flags) of a state combination (S17): one order per order of its flags that belong to a recency set, the
    orders giving every set the same latest flag counted once, and the number of those flags (n, the bound n!)."""
    members = [f for f in sorted(k) if any(f in s for s in sets.values())]
    seen = set()
    for perm in itertools.permutations(members):
        pos = {f: i for i, f in enumerate(perm)}
        seen.add(tuple(max((f for f in s if f in pos), key=pos.get, default=None) for s in sets.values()))
    return len(seen), len(members)


def recency_orders(d) -> tuple:
    """(state combinations, the gate's runs over them, the most orders of one combination, the most flags of the sets one
    combination holds) of an entry with recency sets (S17: one run per order of the present flags, at most n!)."""
    sets = recency_of(d)
    combos = state_combinations(d)
    counts = [combination_orders(k, sets) for k in combos]
    return (len(combos), sum(c for c, _ in counts), max((c for c, _ in counts), default=0),
            max((n for _, n in counts), default=0))


def variant_declaration_problems(item: Item, variants, sizes) -> list:
    """`exclusive_variants`, `variant_sets` and `default_size_by_variant` (S10, S12, S17; ADD-A121)."""
    d = item.data
    out = []
    groups = []
    if "exclusive_variants" in d:
        ev = d["exclusive_variants"]
        if not (isinstance(ev, list) and all(str_list(g) for g in ev)):
            out.append((item.ln("exclusive_variants"), "exclusive_variants is not a list of lists of variants"))
        else:
            groups = ev
            for g in ev:
                out += [(item.ln("exclusive_variants"), f"exclusive_variants: {v!r} is not among variants")
                        for v in g if v not in variants]
                out += [(item.ln("exclusive_variants"), f"exclusive_variants: a group names {v!r} twice")
                        for v in sorted({v for v in g if g.count(v) > 1})]
    generated = set()
    if "variant_products" in d:
        problems, generated = product_problems(d, variants, groups)
        out += [(item.ln("variant_products"), p) for p in problems]
    if "variant_sets" in d:
        vs = d["variant_sets"]
        if not (isinstance(vs, list) and all(str_list(s) for s in vs)):
            out.append((item.ln("variant_sets"), "variant_sets is not a list of lists of variants"))
        else:
            out += [(item.ln("variant_sets"), f"variant_sets: {s} is also a set variant_products gives (a set is "
                                              f"listed once, S17)") for s in vs if frozenset(s) in generated]
            seen = set()
            for s in vs:
                where = f"variant_sets: {s}"
                if len(set(s)) < 2:
                    out.append((item.ln("variant_sets"), f"{where} holds fewer than two variants"))
                out += [(item.ln("variant_sets"), f"{where}: {v!r} is not among variants") for v in s
                        if v not in variants]
                out += [(item.ln("variant_sets"), f"{where} names {v!r} twice") for v in sorted({v for v in s
                                                                                               if s.count(v) > 1})]
                out += [(item.ln("variant_sets"), f"{where} holds {', '.join(sorted(set(s) & set(g)))} of one "
                                                  f"exclusive_variants group") for g in groups
                        if len(set(s) & set(g)) > 1]
                if frozenset(s) in seen:
                    out.append((item.ln("variant_sets"), f"{where} is listed twice"))
                seen.add(frozenset(s))
    if "default_size_by_variant" in d:
        dv = d["default_size_by_variant"]
        if not isinstance(dv, list) or not all(isinstance(x, dict) and set(x) == {"variant", "size"}
                                               and isinstance(x["variant"], str) and isinstance(x["size"], str)
                                               for x in dv):
            out.append((item.ln("default_size_by_variant"), "default_size_by_variant is not a list of "
                                                            "{ variant, size }"))
        else:
            by_variant = {x["variant"]: x["sizes"] for x in d.get("sizes_by_variant", [])
                          if isinstance(x, dict) and isinstance(x.get("variant"), str) and str_list(x.get("sizes"))} \
                if isinstance(d.get("sizes_by_variant"), list) else {}
            names = [x["variant"] for x in dv]
            for x in dv:
                v, size = x["variant"], x["size"]
                if v not in variants:
                    out.append((item.ln("default_size_by_variant"), f"default_size_by_variant: {v!r} is not among "
                                                                    f"variants"))
                elif size not in by_variant.get(v, sizes):
                    out.append((item.ln("default_size_by_variant"), f"default_size_by_variant: {v!r} does not take "
                                                                    f"the size {size!r}"))
            out += [(item.ln("default_size_by_variant"), f"default_size_by_variant: two entries name {v!r}")
                    for v in sorted({v for v in names if names.count(v) > 1})]
    return out


def products_ok(vp) -> bool:
    """`variant_products` has its form: a list of products, each a non-empty list of groups, each `{ any = [...] }`
    or `{ one = [...] }` naming one or more variants."""
    return isinstance(vp, list) and all(
        isinstance(prod, list) and prod and all(
            isinstance(g, dict) and len(g) == 1 and next(iter(g)) in PRODUCT_GROUPS and str_list(next(iter(g.values())))
            and next(iter(g.values())) for g in prod) for prod in vp)


def requires_ok(d, s) -> bool:
    """The set is not one `requires_variants` excludes: where it lists variants, the set holds one of them (any one:
    the forms the platform draws the widget only as, S10, ADD-A123)."""
    req = d.get("requires_variants") if str_list(d.get("requires_variants")) else []
    return not req or bool(set(req) & s)


def product_sets(d, groups) -> set:
    """The sets the entry's `variant_products` give (S17 *Variant products*): every choice of one variant or none from
    each `any` group and one from each `one` group, those of two or more variants, less those that hold two variants
    of an `exclusive_variants` group (`groups`) or that `requires_variants` excludes; a set two products give is one."""
    vp = d.get("variant_products")
    out = set()
    if not products_ok(vp):
        return out
    for prod in vp:
        choices = [([None] if k == "any" else []) + list(v) for g in prod for k, v in g.items()]
        for pick in itertools.product(*choices):
            chosen = [v for v in pick if v is not None]
            sset = frozenset(chosen)
            if len(sset) < 2 or len(sset) != len(chosen):
                continue
            if any(len(sset & set(g)) > 1 for g in groups) or not requires_ok(d, sset):
                continue
            out.add(sset)
    return out


def product_problems(d, variants, groups):
    """(problems, the sets the products give) of `variant_products` (S17, ADD-A122): its form, every member among
    `variants`, no product naming one variant in two of its groups (or twice in one)."""
    vp = d["variant_products"]
    if not products_ok(vp):
        return ["variant_products is not a list of products, each a non-empty list of groups `{ any = [...] }` or "
                "`{ one = [...] }` (S17)"], set()
    out = []
    for i, prod in enumerate(vp):
        members = [v for g in prod for vs in g.values() for v in vs]
        out += [f"variant_products[{i}]: {v!r} is not among variants" for v in dict.fromkeys(members)
                if v not in variants]
        out += [f"variant_products[{i}] names {v!r} in two of its groups or twice in one (S17)"
                for v in sorted({v for v in members if members.count(v) > 1})]
    return out, product_sets(d, groups)


def drawn_variant_sets(d):
    """(listed, from products, in all) counts of an entry's variant sets: the listed ones and the products' (each
    trimmed as product_sets trims), less the sets a `not_drawn` entry stating only `variants` and `not_variants`
    removes whole and the listed sets `requires_variants` excludes (S17: the combinations are the sets less those it
    excludes; holding one of its forms keeps a set, S10, ADD-A123); a set the list and the products both give is
    counted once."""
    groups = d.get("exclusive_variants") if isinstance(d.get("exclusive_variants"), list) else []
    groups = [g for g in groups if str_list(g)]
    vs = d.get("variant_sets")
    listed = {frozenset(x) for x in vs if str_list(x)} if isinstance(vs, list) else set()
    products = product_sets(d, groups)
    nd = d.get("not_drawn") if isinstance(d.get("not_drawn"), list) else []
    gone = [(set(w.get("variants", [])), set(w.get("not_variants", []))) for w in nd
            if isinstance(w, dict) and w and set(w) <= {"variants", "not_variants"}
            and all(str_list(w[k]) for k in w)]

    def drawn(sset):
        return not any(a <= sset and not (n & sset) for a, n in gone)

    listed = {x for x in listed if drawn(x) and requires_ok(d, x)}
    products = {x for x in products if drawn(x)}
    return len(listed), len(products), len(listed | products)


def part_patch_problems(ctx, value, widget) -> list:
    """A whole part's value (S17, ADD-A121): `"absent"` or `{ drawn_as = { widget, part, variants } }` (S11)."""
    if value == "absent":
        return []
    da = value.get("drawn_as") if isinstance(value, dict) and set(value) == {"drawn_as"} else None
    if not isinstance(da, dict):
        return [f"the key names a whole part of {widget}: its value is \"absent\" or "
                f"{{ drawn_as = {{ widget, part, variants }} }} (S11, S17)"]
    out = [f"value.drawn_as: unknown key `{k}`" for k in da if k not in DRAWN_AS_KEYS]
    if "widget" not in da:
        return out + ["value.drawn_as: missing `widget`"]
    return out + part_problems(ctx, da, "value.drawn_as")


# ------------------------------------------------------------------------------------------- pending facts

def when_lists(when, k):
    v = when.get(k, []) if isinstance(when, dict) else []
    return set(v) if str_list(v) else set()


def whens_overlap(a, b, model) -> bool:
    """Whether some combination may hold both `when`s (S17 *Which fact applies*): False only where they cannot, by
    S7's matching (a name one requires and the other refuses; two sizes, densities, schemes, presets, flavours,
    idioms, input kinds, postures, device models or one container's two sizes; two directions, parities or
    `first`/`last`/`only` values; a context one requires and the other refuses) and by the widget's state models
    (`model`: two states of one exclusive group, which the reduction leaves one of; two variants of one
    `exclusive_variants` group; two or more variants no listed or product set holds)."""
    a = a if isinstance(a, dict) else {}
    b = b if isinstance(b, dict) else {}
    la = {k: v for k, v in a.get("latest", {}).items() if isinstance(v, str)} if isinstance(a.get("latest"), dict) \
        else {}
    lb = {k: v for k, v in b.get("latest", {}).items() if isinstance(v, str)} if isinstance(b.get("latest"), dict) \
        else {}
    if any(la[s] != lb[s] for s in la.keys() & lb.keys()):
        return False
    latest = {**la, **lb}
    held = {}
    for k in ("states", "variants", "conditions", "sizes", "densities"):
        held[k] = when_lists(a, k) | when_lists(b, k)
        if k == "states":
            held[k] |= set(latest.values())
        if held[k] & (when_lists(a, f"not_{k}") | when_lists(b, f"not_{k}")):
            return False
    if len(held["sizes"]) > 1 or len(held["densities"]) > 1:
        return False
    kinds = {}
    for c in held["conditions"]:
        head, _, arg = c.partition(":")
        if head in ("scheme", "preset", "flavor", "idiom", "input", "posture", "device_model"):
            kinds.setdefault(head, set()).add(arg)
        elif head == "container_size":
            container, _, size = arg.partition(":")
            kinds.setdefault(f"container_size:{container}", set()).add(size)
    if any(len(v) > 1 for v in kinds.values()):
        return False
    if "direction" in a and "direction" in b and a["direction"] != b["direction"]:
        return False
    sa = a.get("size_class") if isinstance(a.get("size_class"), dict) else {}
    sb = b.get("size_class") if isinstance(b.get("size_class"), dict) else {}
    for axis in ("horizontal", "vertical"):
        on = when_lists(sa, axis) | when_lists(sb, axis)
        if len(on) > 1 or on & (when_lists(sa, f"not_{axis}") | when_lists(sb, f"not_{axis}")):
            return False
    pa = a.get("position") if isinstance(a.get("position"), dict) else {}
    pb = b.get("position") if isinstance(b.get("position"), dict) else {}
    if any(k in pa and k in pb and pa[k] != pb[k] for k in ("first", "last", "only", "parity")):
        return False
    for x, y in ((a, b), (b, a)):
        ctx_x = x.get("context") if isinstance(x.get("context"), list) else []
        not_y = y.get("not_context") if isinstance(y.get("not_context"), list) else []
        if any(c in not_y for c in ctx_x):
            return False
    if model is not None:
        states, groups, sets = model[:3]
        recency = model[3] if len(model) > 3 else {}
        if latest and not orders_possible(latest, held["states"], recency):
            return False
        if any(len(held["states"] & g) > 1 for g in states):
            return False
        if any(len(held["variants"] & g) > 1 for g in groups):
            return False
        if len(held["variants"]) > 1 and not any(held["variants"] <= x for x in sets):
            return False
    return True


def orders_possible(latest, held, recency) -> bool:
    """Whether one order of the flags (`Element.state_order`, S7.2) can make each `latest` flag its set's latest while
    the flags `held` are present: each latest flag set after every other held member of its set (the members every
    entry gives the set, `recency`), with no cycle."""
    after = {f: set() for f in held}
    for name, flag in latest.items():
        after.setdefault(flag, set()).update(g for g in recency.get(name, ()) if g in held and g != flag)
    done, active = set(), set()

    def acyclic(f):
        if f in done:
            return True
        if f in active:
            return False
        active.add(f)
        ok = all(acyclic(g) for g in after.get(f, ()))
        active.discard(f)
        done.add(f)
        return ok
    return all(acyclic(f) for f in list(after))


def widget_models(items) -> dict:
    """{(platform, widget): (exclusive state groups, exclusive variant groups, variant sets, recency sets)} over the
    widget's state-model entries of the platform (every entry's, the listed and the products' sets; each recency set's
    members those every entry declaring it gives)."""
    out = {}
    for it in items:
        d = it.data
        if it.table != "model" or d.get("available") is not True or not isinstance(d.get("widget"), str):
            continue
        states, groups, sets, recency = out.setdefault((it.platform, d["widget"]), ([], [], set(), {}))
        for name, members in recency_of(d).items():
            recency[name] = recency[name] & set(members) if name in recency else set(members)
        ex = d.get("exclusive") if isinstance(d.get("exclusive"), list) else []
        states += [set(g) for g in ex if str_list(g)]
        ev = d.get("exclusive_variants") if isinstance(d.get("exclusive_variants"), list) else []
        ev = [g for g in ev if str_list(g)]
        groups += [set(g) for g in ev]
        vs = d.get("variant_sets") if isinstance(d.get("variant_sets"), list) else []
        sets |= {frozenset(x) for x in vs if str_list(x)} | product_sets(d, ev)
    return out


def pending_problems(ctx, items) -> list:
    """S17 *Which fact applies*: a pending fact that a later fact of its key, in its file and of its run (the accent
    run's `scheme_file` and `accent`, or neither), shadows on a combination both `when`s may hold in (whens_overlap)
    is a finding: the combination would pass with the later fact's value."""
    models = widget_models(items)
    facts = {}
    for it in items:
        if it.table == "fact" and not it.palette and isinstance(it.data.get("key"), str):
            facts.setdefault(it.file, []).append(it)
    out = []
    for file, fs in facts.items():
        for i, pend in enumerate(fs):
            d = pend.data
            if "pending" not in d:
                continue
            widget = key_info(ctx, d["key"])[2]
            model = models.get((pend.platform, widget)) if widget else None
            run = (d.get("scheme_file"), d.get("accent"))
            for later in fs[i + 1:]:
                ld = later.data
                if canonical_key(ld["key"]) != canonical_key(d["key"]) or (
                        ld.get("scheme_file"), ld.get("accent")) != run:
                    continue
                if whens_overlap(d.get("when"), ld.get("when"), model):
                    out.append((file, pend.line, f"pending fact {d['key']!r} ({d['pending']}) is shadowed by the later "
                                                 f"fact at line {later.line} on a combination both `when`s hold in "
                                                 f"(S17 *Which fact applies*: narrow its `when` or place it after)"))
    return out


def model_entries(items) -> dict:
    """{(platform, widget): [the widget's available state-model entries of the platform]}."""
    out = {}
    for it in items:
        d = it.data
        if it.table == "model" and d.get("available") is True and isinstance(d.get("widget"), str):
            out.setdefault((it.platform, d["widget"]), []).append(it)
    return out


def entry_conditions(it) -> set:
    w = it.data.get("when")
    return set(w["conditions"]) if isinstance(w, dict) and str_list(w.get("conditions")) else set()


def applicable_entries(entries, when) -> list:
    """The state-model entries a fact's `when` can hold under (S17: an entry with `preset:` or `flavor:` replaces the
    shared one there): those whose conditions the `when`'s all include, if any; else the shared entries and every
    entry whose conditions the `when` does not refuse (a `not_conditions` member, or another value of its kind)."""
    conds, nots = when_lists(when, "conditions"), when_lists(when, "not_conditions")
    specific = [e for e in entries if entry_conditions(e) and entry_conditions(e) <= conds]
    if specific:
        return specific
    fixed = {c.partition(":")[0]: c for c in conds if c.startswith(("preset:", "flavor:"))}
    return [e for e in entries if not any(c in nots or fixed.get(c.partition(":")[0], c) != c
                                          for c in entry_conditions(e))]


def latest_problems(ctx, items) -> list:
    """A fact's `when.latest` against its widget's state models (S12, S17; ADD-A124): each set a recency set and each
    flag a member of it in every entry the fact can hold under (applicable_entries)."""
    models = model_entries(items)
    out = []
    for it in items:
        when = it.data.get("when")
        if it.table != "fact" or it.palette or not isinstance(when, dict) or not isinstance(when.get("latest"), dict):
            continue
        widget = key_info(ctx, it.data.get("key"))[2]
        if not widget:
            continue
        entries = applicable_entries(models.get((it.platform, widget), []), when)
        if not entries:
            out.append((it.file, it.ln("when"), f"when.latest: no state model of {widget} in {it.platform}/ that the "
                                                f"fact can hold under declares recency sets (S17)"))
            continue
        for name, flag in when["latest"].items():
            for e in entries:
                sets = recency_of(e.data)
                if name not in sets:
                    out.append((it.file, it.ln("when"), f"when.latest: {name!r} is not a recency set of {widget}'s "
                                                        f"state model ({e.file}:{e.line}; S12)"))
                elif flag not in sets[name]:
                    out.append((it.file, it.ln("when"), f"when.latest.{name}: {flag!r} is not a member of the set "
                                                        f"({e.file}:{e.line}; S12)"))
    return out


def canonical_when(when):
    """A `when` as a comparable value: its lists of names as sets (two `when`s equal as S11 reads them)."""
    if isinstance(when, dict):
        return tuple(sorted((k, canonical_when(v)) for k, v in when.items()))
    if isinstance(when, list):
        if str_list(when):
            return ("set",) + tuple(sorted(set(when)))
        return ("list",) + tuple(canonical_when(v) for v in when)
    return when


def ellipse_problems(ctx, items) -> list:
    """`Radius::Ellipse` (S2.4, S12; ADD-A124), where the facts show it: the corners of one `Corners` value (the leaf
    facts of a `….corners.<corner>` path, or a whole `….corners` table) under one `when`, in one file and run, state
    `"ellipse"` on all four corners or none, and their curve, where a fact states it, is not `continuous`. The corners
    are the Radius fields of the core's `Corners`."""
    corners = [k for k, v in ctx.structs.get("Corners", {}).items() if strip_type(v) == "Radius"]
    groups = {}
    for it in items:
        d = it.data
        if it.table != "fact" or it.palette or "value" not in d or not isinstance(d.get("key"), str):
            continue
        parsed = key_segments(d["key"])
        if not isinstance(parsed, tuple) or "corners" not in parsed[0][-2:]:
            continue
        segs = parsed[0]
        i = len(segs) - 1 - segs[::-1].index("corners")
        gk = (it.file, d.get("scheme_file"), d.get("accent"), tuple(segs[:i + 1]), canonical_when(d.get("when")))
        g = groups.setdefault(gk, {})
        if i == len(segs) - 2:
            g[segs[-1]] = (d["value"], it.ln("value"))
        elif isinstance(d["value"], dict):
            for k, v in d["value"].items():
                g[re.sub(r"_px$", "", k)] = (v, it.ln("value"))
    out = []
    for (file, _, _, prefix, _), g in groups.items():
        path = ".".join(prefix)
        on = [c for c in corners if c in g and g[c][0] == "ellipse"]
        if not on:
            continue
        line = g[on[0]][1]
        for c in corners:
            if c not in g:
                out.append((file, line, f"{path}: `ellipse` on {', '.join(on)} and no fact under the same when states "
                                        f"{c} (S12: an Ellipse on all four corners or none)"))
            elif g[c][0] != "ellipse":
                out.append((file, g[c][1], f"{path}.{c}: {g[c][0]!r} beside `ellipse` on {', '.join(on)} under the "
                                           f"same when (S12: an Ellipse on all four corners or none)"))
        if "curve" in g and g["curve"][0] == "continuous":
            out.append((file, g["curve"][1], f"{path}.curve: `continuous` with `ellipse` corners (S12: an Ellipse "
                                             f"corner is Circular)"))
    return out


# ------------------------------------------------------------------------------------------------ palettes

def colour_strings(data, prefix=""):
    """TOML path -> string of every string leaf holding a colour literal, outside [preset], [declare], [palette]."""
    out = {}
    if isinstance(data, dict):
        for k, v in data.items():
            if not prefix and k in ("preset", "declare", "palette", "rules"):
                continue
            out.update(colour_strings(v, f"{prefix}{k}" if not prefix else f"{prefix}.{k}"))
    elif isinstance(data, list):
        for i, v in enumerate(data):
            out.update(colour_strings(v, f"{prefix}.{i}"))
    elif isinstance(data, str) and re.search(r"#[0-9a-fA-F]{6}\b|#[0-9a-fA-F]{8}\b|\b(?:srgb|display_p3)\(", data):
        out[prefix] = data
    return out


def palette_problems(path, text, data, fact_keys) -> list:
    lines = text.splitlines()

    def ln(needle):
        for i, line in enumerate(lines, 1):
            if needle in line:
                return i
        return 1

    pal = data.get("palette", {}) if isinstance(data.get("palette"), dict) else {}
    sources = pal.get("sources", {}) if isinstance(pal.get("sources"), dict) else {}
    picks = pal.get("picks", {}) if isinstance(pal.get("picks"), dict) else {}
    stated = colour_strings(data)
    out = []
    for k, v in sources.items():
        if not isinstance(v, str) or v not in fact_keys:
            out.append((path, ln(f'"{k}"'), f"[palette.sources] {k!r} = {v!r} is not a palette-fact key"))
    for name, table in (("sources", sources), ("picks", picks)):
        for k in table:
            if k not in stated:
                out.append((path, ln(f'"{k}"'), f"[palette.{name}] {k!r} is not a stated colour literal's path"))
    for k in sorted(stated):
        n = (k in sources) + (k in picks)
        if n != 1:
            leaf = k.rsplit(".", 1)[-1]
            out.append((path, ln(leaf), f"colour {k!r} is in {'both' if n else 'neither'} of [palette.sources] and "
                                        f"[palette.picks]"))
    return out


# ------------------------------------------------------------------------------------------------- check

def check(ctx: Context):
    """(findings as (file, line, what), counts)."""
    items, findings, nfiles = load(ctx)
    for it in items:
        problems = model_problems(ctx, it) if it.table == "model" else fact_problems(ctx, it)
        findings += [(it.file, ln, what) for ln, what in problems]
    findings += pending_problems(ctx, items)
    findings += latest_problems(ctx, items)
    findings += ellipse_problems(ctx, items)
    fact_keys = {it.data.get("key") for it in items if it.palette and isinstance(it.data.get("key"), str)}
    palettes = bundled_palettes(ctx.root)
    for path, text, data in palettes:
        findings += palette_problems(path, text, data, fact_keys)
    counts = {
        "files": nfiles,
        "facts": sum(1 for it in items if it.table == "fact" and not it.palette),
        "models": sum(1 for it in items if it.table == "model"),
        "palette": sum(1 for it in items if it.palette),
        "cites": sum(len(it.cites) for it in items),
        "palettes": len(palettes),
        "pending": sum(1 for it in items if it.table == "fact" and "pending" in it.data),
        "variant_sets": [(it.file, it.line, it.data["widget"], drawn_variant_sets(it.data)) for it in items
                         if it.table == "model" and isinstance(it.data.get("widget"), str)
                         and ("variant_sets" in it.data or "variant_products" in it.data)],
        "recency_orders": [(it.file, it.line, it.data["widget"], recency_orders(it.data)) for it in items
                           if it.table == "model" and isinstance(it.data.get("widget"), str)
                           and it.data.get("available") is True and recency_of(it.data)],
    }
    return findings, counts


def summary(counts, n):
    vacuous = "no facts yet: " if not counts["files"] else ""
    orders = counts.get("recency_orders", [])
    return (f"{vacuous}{counts['files']} fact files, {counts['facts']} facts ({counts['pending']} pending), "
            f"{counts['models']} state-model entries ({len(orders)} with recency sets: "
            f"{sum(o[3][1] for o in orders)} runs per order of {sum(o[3][0] for o in orders)} state combinations), "
            f"{counts['palette']} palette facts, {counts['cites']} citations; {counts['palettes']} bundled palettes "
            f"(new format): {n} findings")


# --------------------------------------------------------------------------------------------- self-test

SELF_TEST_CORE = '''
```rust
pub enum StateFlag { Hover, Pressed, Focused, FocusVisible, Disabled, Checked, Shared(SharedFlag), Widget(Key) }
pub enum SharedFlag { Pulling, Refreshing }
pub enum Condition { WindowInactive, ContrastHigh, Scheme(Scheme), Flavor(Key), Preset(PresetId) }
pub enum Scheme { Light, Dark }
pub enum InputKind { Touch, Mouse }
pub enum Idiom { Phone, Pad }
pub enum Posture { Tabletop }
pub enum ControlSize { Small, Regular, Large }
pub enum Density { Compact, Keyed(Key) }
pub enum SizeClass { Compact, Regular }
pub enum SurfaceSet { Window, View, Header, Button }
pub struct SurfaceRoles { pub background: Value<Color>, pub foreground: Value<Color> }
pub struct GlobalRoles { pub accent: Value<Color>, pub status: StatusRoles, pub accent_ramp: Vec<(Key, Value<Color>)> }
pub struct Tokens { pub text: TextTokens, pub kde: KdeTokens, pub motion: MotionTokens, pub focus: FocusIndicator,
                    pub icons: IconTokens, pub material: MaterialScheme }
pub struct IconTokens { pub theme: Value<String> }
pub struct MotionTokens { pub durations: Vec<(Key, Value<Ms>)>, pub timings: Vec<(Key, Timing)>,
                          pub speed_factor: Value<f32> }
pub struct PartStyle { pub fill: Value<Paint>, pub stroke: Value<Stroke>, pub corners: Value<Corners>,
                       pub shadow: Value<Shadow>, pub text: Value<TextStyle>, pub focus: Value<FocusIndicator>,
                       pub padding: Sides<Value<Px>>, pub min_width: Value<Px>, pub opacity: Value<Fraction>,
                       pub transitions: Vec<Transition>, pub glyph: Value<Glyph>,
                       pub interaction: Value<Interaction> }
pub struct Interaction { pub state_layer: StateLayerStyle, pub ripple: RippleStyle }
pub struct StateLayerStyle { pub color: Value<Color>, pub alpha: Value<Fraction> }
pub struct RippleStyle { pub effect: Value<RippleEffect>, pub color: Value<Color>, pub alpha: Value<Fraction>,
                         pub bounded: bool, pub max_radius: Value<Px>, pub padding: Value<Px> }
pub enum RippleEffect { AndroidPatterned }
pub struct ShadowLayer { pub x: Value<Px>, pub y: Value<Px>, pub blur: Value<Px>, pub spread: Value<Px>,
                         pub color: Value<Color>, pub inset: bool }
pub struct ElementData { pub progress: f32 }
pub struct TextRoles { pub body: X }
pub struct TextTokens { pub roles: X, pub keyed: X, pub scaling: X, pub fallbacks: FontFallbacks }
pub struct TextStyle { pub font: Font, pub fill: Value<Paint> }
pub struct Font { pub family: Value<String>, pub size: Value<FontSize>, pub weight: Value<u16> }
pub struct IconStyle { pub size: Value<Px> }
pub struct Glyph { pub source: GlyphSource, pub stroke: X }
pub enum GlyphSource { ThemeIcon { name: String, fallbacks: Vec<String> }, NamedSystemImage { name: String } }
pub struct GlyphStroke { pub width: X }
pub struct FocusIndicator { pub width: Value<Px>, pub corners: X, pub transition: Value<Transition> }
pub struct Corners { pub top_left: Radius, pub top_right: Radius, pub bottom_right: Radius, pub bottom_left: Radius,
                     pub curve: CornerCurve }
pub enum Radius { Px(Value<Px>), Full { maximum: Value<Px> }, Ellipse, FollowPart }
pub enum CornerCurve { Circular, Continuous }
pub struct Stroke { pub top: Edge }
pub struct Transform { pub scale: X }
pub struct Caret { pub width: X }
pub struct Behavior { pub focus_visible_when: X }
pub struct ScrollPhysics { pub friction: X }
pub struct TouchBehavior { pub slop: X }
pub struct FontFallbacks { pub families: Vec<String>, pub emoji: X, pub per_language: Vec<(LanguageTag, Vec<String>)>,
                           pub per_family: Vec<(String, Vec<String>)> }
pub struct TextRendering { pub hinting: X }
pub struct KdeTokens { pub disabled: KdeEffects, pub frame_width: Value<Px> }
pub struct KdeEffects { pub intensity_amount: Value<f32> }
pub struct MaterialScheme { pub variant: MaterialVariant, pub primary: X }
pub enum MaterialVariant { TonalSpot, FruitSalad, Cmf }
pub struct Transition { pub properties: Vec<PropertyGroup>, pub duration: Value<Ms>, pub timing: Value<Timing>,
                        pub from: Option<Vec<StateFlag>> }
pub enum PropertyGroup { Fill, Stroke, StrokeWidth, StateLayer }
pub enum TransitionKind { Interpolate, Keyed { key: Key } }
pub enum Timing { CubicBezier(f32, f32, f32, f32), Linear, Spring { mass: f32, stiffness: f32, damping_ratio: f32,
                  rest_epsilon: f32 }, Platform(PlatformCurve) }
pub enum PlatformCurve { KirigamiVelocity, Qt(QtEasing), Adw(AdwEasing) }
pub enum QtEasing { InQuad, OutQuad, InOutQuad, InCubic, OutCubic, InOutCubic, InOutSine, OutExpo }
pub enum AdwEasing { EaseInCubic, EaseOutCubic, EaseInOutSine, EaseInExpo, EaseOutExpo }
pub enum KdeShadeRole { Light, Mid }
pub struct Material { pub blur: X }
pub enum PlacementStrategy { Below }
pub enum PresentationEvent { Appear, Keyed(Key) }
pub enum PresentationKind { Fade }
pub enum FocusVisibleWhen { Always }
```

**Functions** (a closed set):

| Function | Result | Definition |
|---|---|---|
| `alpha(c, a)` | colour | alpha |
| `with_alpha(c, a)` | colour | the alpha set |
| `mix(a, b, t)` | colour | mix |
| `kcu_mix(a, b, bias)`, `kcu_tint(a, b, amount)` | colour | KColorUtils |
| `kde_shade(c, role: KdeShadeRole, contrast)` | colour | KColorScheme::shade |
| `kirigami_lerp(a, b, balance)` | colour | Kirigami's ColorUtils.linearInterpolation |
| `floor(x)`, `round(x)` | number | toward negative infinity; to the nearest, a half away from zero |
| `pick(index, c0, c1, …)` | colour | the colour at `index` |

A function is added only with its rule.
'''

SELF_TEST_CATALOGUE = '''
## Buttons and toggles

### button

- **Parts:**
  - `body` — the root: `fill`, `stroke`, `corners`, `shadow`, `padding`, `min_width`, `focus`, `transitions`,
    `interaction`.
  - `label` — the text: `text`.
  - `mark` — the mark: `glyph`.
- **Widget values:** `icon_text_gap_px: Value<Px>`, `repeat_delay_ms: Value<Ms>`.
- **Variants:** typed `flat`, `primary`; keyed `@pill`.
- **Flags:** keyed `@busy`.

### pull_to_refresh

- **Parts:**
  - `body` — the root: `padding`.
  - `indicator` — the indicator: `fill`, `shadow`, `transitions`.
- **Widget values:** `trigger_distance_px: Value<Px>`.
- **Variants:** none.
- **Flags:** shared `pulling`, `refreshing`.

### toolbar

- **Parts:**
  - `bar` — the root: `fill`.

## Keyed components

#### component.led (kde)

- **Parts:**
  - `body` — the root: `fill`.

## Tokens

**token.motion.durations** (`Vec<(Key, Value<Ms>)>`, S4.1). A scaled duration.

| Platform | Keys (one per duration the platform states) | Rows |
|---|---|---|
| libadwaita | `button`; by family, the tab bar's | GN-3.18 |
| kde-breeze | `animations` (Breeze's `Animations`), `message_widget` | K1.17 |
'''

SELF_TEST_SOURCES = '''
[[source]]
id = "libadwaita@1.10.0"
kind = "git"
generations = ["adwaita"]

[[source]]
id = "breeze@6.8.2"
kind = "git"
generations = ["kde-breeze"]

[[source]]
id = "microsoft-ui-xaml@2.5.1"
kind = "git"
generations = ["windows-11"]

[[source]]
id = "xcode@16.4"
kind = "sdk"
generations = ["macos-monterey", "ios-13"]

[[source]]
id = "xcode@26.6"
kind = "sdk"
generations = ["macos-tahoe", "ios-26"]

[[source]]
id = "archlinux@2026-10-29"
kind = "distribution"
generations = ["kde-breeze", "adwaita"]

[[source]]
id = "hig-typography@2026-10-29"
kind = "snapshot"
generations = ["macos-tahoe", "ios-26"]
file = "page.json"

[[source]]
id = "catppuccin@1.7.1"
kind = "git"
generations = []
palette = true
'''

SELF_TEST_ROWS = {"GN-1.29", "W3-02", "W1-001", "K1.154", "MAC-B08", "MAC-P4-8†", "C-10"}

# The example facts of the specification's walk-throughs (review reports 14-16), one file each, and the run.toml
# files their measured citations need. Each case below edits one of them.
SELF_TEST_FILES = {
    "docs/facts/gnome/buttons.toml": '''
[[fact]]
key = "button.body.fill"
when = { states = ["hover"], variants = ["flat"] }
value = "= alpha(surface.foreground, 0.07)"
rows = ["GN-1.29"]

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:10-11"
quote = "&:hover { background-color: $hover_color; }"

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/stylesheet/_colors.scss:3"
quote = "$hover_color: alpha(currentColor, .07);"
''',
    "docs/facts/gnome/text.toml": '''
[[fact]]
key = "button.label.text.font.family"
value = "Adwaita Sans"
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/stylesheet/_fonts.scss:1"
quote = 'font-family: "Adwaita Sans", sans-serif;'

[[fact]]
key = "token.text.fallbacks.families"
value = ["Adwaita Sans", "sans-serif"]
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/stylesheet/_fonts.scss:1"
quote = 'font-family: "Adwaita Sans", sans-serif;'

[[fact]]
key = "button.mark.glyph.source"
value = { theme_icon = { name = "go-next-symbolic", fallbacks = ["go-next"] } }
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/stylesheet/_fonts.scss:2"
quote = '/* "go-next-symbolic", "go-next" */'
''',
    "docs/facts/kde/colors.toml": '''
[[fact]]
key = "role.window.background"
value = "#eff0f1"
rows = ["K1.154"]

[[fact.cite]]
kind = "published"
source = "breeze@6.8.2 colors/BreezeLight.colors:5"
quote = "BackgroundNormal=239,240,241"

[[fact]]
key = "role.header.background"
when = { conditions = ["window_inactive"] }
value = "#e3e5e7"
rows = []
scheme_file = "breeze@6.8.2 colors/BreezeLight.colors"
accent = "#3daee9"

[[fact.cite]]
kind = "measured"
evidence = "docs/evidence/kde-accent/2026-11-03-run-2/kde-breeze-light/kdeglobals#Colors:Header][Inactive/BackgroundNormal"
tolerance = 0
''',
    "docs/facts/windows/buttons.toml": '''
[[fact]]
key = "button.body.fill"
when = { states = ["hover"], variants = ["flat"] }
value = "#ffffff0f"
rows = ["W1-001"]

[[fact.cite]]
kind = "published"
source = "microsoft-ui-xaml@2.5.1 controls/dev/CommonStyles/Common_themeresources_any.xaml:4"
quote = '<Color x:Key="SubtleFillColorSecondary">#0FFFFFFF</Color>'

[[fact]]
key = "button.values.repeat_delay"
value = "83ms"
rows = []

[[fact.cite]]
kind = "published"
source = "microsoft-ui-xaml@2.5.1 controls/dev/CommonStyles/Button_themeresources.xaml:7"
quote = '<DoubleAnimation Storyboard.TargetProperty="Opacity" Duration="0:0:0.083" />'

[[fact]]
key = "button.body.min_width"
value = "80px"
rows = []

[[fact.cite]]
kind = "published"
source = "microsoft-ui-xaml@2.5.1 controls/dev/CommonStyles/Button_themeresources.xaml:9"
quote = '<x:Double x:Key="Button_MinWidth">80</x:Double>'

[[fact]]
key = "button.body.shadow"
value = { layers = [ { x_px = 0, y_px = 1, blur_px = 2, spread_px = 0, color = "#00000024", inset = false },
                     { x_px = 0, y_px = 4, blur_px = 8, spread_px = 0, color = "#0000001a", inset = false } ] }
rows = ["W3-02"]

[[fact.cite]]
kind = "measured"
evidence = "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/shadows.toml#button.rest"
tolerance = { "layers.0.x_px" = 0, "layers.0.y_px" = 0.5, "layers.0.blur_px" = 1, "layers.0.spread_px" = 0, "layers.0.color" = 2, "layers.1.x_px" = 0, "layers.1.y_px" = 0.5, "layers.1.blur_px" = 1, "layers.1.spread_px" = 0, "layers.1.color" = 2 }
''',
    "docs/facts/macos/buttons.toml": '''
[[fact]]
key = "button.label.text.font.size"
value = "13pt"
rows = ["MAC-B08"]

[[fact.cite]]
kind = "measured"
evidence = "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-15/probe.toml#macos.fonts.sizes.systemFontSize"
tolerance = 0

[[fact.cite]]
kind = "measured"
evidence = "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-26/probe.toml#macos.fonts.sizes.systemFontSize"
tolerance = 0

[[fact]]
key = "button.body.fill"
when = { conditions = ["preset:macos-tahoe"], variants = ["primary"] }
value = "= alpha(input.accent, 0.5)"
rows = []

[[fact.cite]]
kind = "measured"
evidence = "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-26/passes.toml#button.primary.fill"
inputs = ["input.accent"]
tolerance = 1
''',
    "docs/facts/macos/state-models.toml": '''
[[model]]
widget = "pull_to_refresh"
when = { conditions = ["preset:macos-tahoe"] }
available = false
rows = []

[[model.cite]]
kind = "published"
source = "xcode@26.6 MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers/NSScrollView.h:20-120"
quote = "@interface NSScrollView : NSView <NSTextFinderBarContainer>"
absent = ["refreshControl", "NSRefreshControl"]

[[model]]
widget = "button"
when = { conditions = ["preset:macos-monterey"] }
available = true
variants = ["flat", "@pill"]
sizes = ["small", "regular", "large"]
default_size = "regular"
densities = []
axes = { states = ["hover", "pressed", "disabled", "focused"], conditions = ["window_inactive", "scheme:dark"], contexts = [{ widget = "toolbar", part = "bar" }] }
exclusive = [["disabled", "pressed", "hover"]]
not_drawn = [{ states = ["hover"], conditions = ["window_inactive"] }]
sizes_by_variant = [{ variant = "@pill", sizes = ["regular"] }]
recency = { glow = ["focused", "hover"] }
rows = ["MAC-B08"]

[[model.cite]]
kind = "published"
source = "xcode@16.4 MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers/NSButton.h:20-90"
quote = "@interface NSButton : NSControl"
''',
    "docs/facts/palettes/catppuccin.toml": '''
[[fact]]
key = "catppuccin.mocha.surface0"
value = "#313244"

[[fact.cite]]
kind = "published"
source = "catppuccin@1.7.1 palette.json:120"
quote = '"hex": "#313244",'
''',
    "docs/evidence/os-probe/2026-11-02-run-1/run.toml": '''
[images.os-probe-macos-15]
preset = "macos-monterey"
source = "xcode@16.4"
os = "24G830"

[images.os-probe-macos-26]
preset = "macos-tahoe"
source = "xcode@26.6"
os = "25G83"
''',
    "docs/evidence/kde-accent/2026-11-03-run-2/run.toml": '''
[images.kde-breeze-light]
preset = "kde-breeze"
source = "archlinux@2026-10-29"
os = "Plasma 6.8.2"
toolkit = "Qt 6.12.0"
''',
    "docs/evidence/winui-host/2026-11-04-run-3/run.toml": '''
[images.winui-host-windows-11]
preset = "windows-11"
source = "microsoft-ui-xaml@2.5.1"
os = "26100.4946"
toolkit = "WindowsAppSDK 2.5.1"
''',
    "native-theme/src/next/presets/catppuccin-mocha.toml": '''
[preset]
id = "catppuccin-mocha"
kind = "palette"

[colors.dark.surfaces.window]
background = "#313244"
foreground = "#cdd6f4"

[palette.sources]
"colors.dark.surfaces.window.background" = "catppuccin.mocha.surface0"

[palette.picks]
"colors.dark.surfaces.window.foreground" = "the guide's text colour range"
''',
}


def write_fixture(root: Path, files: dict):
    for rel, text in files.items():
        p = root / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text, encoding="utf-8")
    (root / FACTS).mkdir(parents=True, exist_ok=True)
    (root / SOURCES).write_text(SELF_TEST_SOURCES, encoding="utf-8")
    pdir = root / PRESET_DIRS[0]
    pdir.mkdir(parents=True, exist_ok=True)
    for p in ALL_PRESETS:
        (pdir / f"{p}.toml").write_text("", encoding="utf-8")


def fixture_context(root: Path):
    return Context(root, core_text=SELF_TEST_CORE, cat_text=SELF_TEST_CATALOGUE, sources_text=SELF_TEST_SOURCES,
                   rows=SELF_TEST_ROWS)


_SBV = 'sizes_by_variant = [{ variant = "@pill", sizes = ["regular"] }]'
_CITE = ('\n[[fact.cite]]\nkind = "published"\n'
         'source = "libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:10-11"\n'
         'quote = "&:hover { background-color: $hover_color; }"\n')
_PART = '[[fact]]\nkey = "button.mark"\nvalue = "absent"\nrows = []\n' + _CITE
_SPRING_TIMING = "{ spring = { mass = 1, stiffness = 100, damping_ratio = 1, rest_epsilon = 0.001 } }"
_SPRING = ('[[fact]]\nkey = "button.body.transitions"\nvalue = [{ properties = ["fill"], duration_ms = 200, timing = '
           + _SPRING_TIMING + ' }]\nrows = []\n' + _CITE)
_CMF = '[[fact]]\nkey = "token.material.variant"\nvalue = "cmf"\nrows = []\n' + _CITE
_FILL = '[[fact]]\nkey = "button.body.fill"\nwhen = { states = ["hover"] }\nvalue = "#ffffff"\nrows = []\n' + _CITE
_PENDING = '\n[[fact]]\nkey = "button.body.fill"\nwhen = { states = ["pressed"] }\npending = "U1"\nrows = []\n'
_LATER = '\n[[fact]]\nkey = "button.body.fill"\nwhen = { states = ["hover"] }\nvalue = "#eeeeee"\nrows = []\n' + _CITE
_MAC_CITE = ('\n[[fact.cite]]\nkind = "published"\n'
             'source = "xcode@16.4 MacOSX.platform/Developer/SDKs/MacOSX.sdk/x.h:1"\nquote = "x"\n')
_MAC_PENDING = ('[[fact]]\nkey = "button.body.fill"\nwhen = { conditions = ["preset:macos-monterey"], states = '
                '["pressed"] }\npending = "U2"\nrows = []\n\n[[fact]]\nkey = "button.body.fill"\nwhen = { conditions = '
                '["preset:macos-monterey"], states = ["hover"] }\nvalue = "#ffffff"\nrows = []\n' + _MAC_CITE)
_MOTION = '[[fact]]\nkey = "token.motion.durations.button"\nvalue = "200ms"\nrows = []\n' + _CITE
_RAMP = '[[fact]]\nkey = "role.global.accent_ramp.light1"\nvalue = "#ff0000"\nrows = []\n' + _CITE
_FALLBACK = ('[[fact]]\nkey = \'token.text.fallbacks.per_family."Segoe Fluent Icons"\'\nvalue = ["Segoe MDL2 Assets"]\n'
             'rows = []\n' + _CITE)
_UNTYPED = "'Segoe MDL2 Assets' is not a value as S11 spells it"   # a key that does not read types no value
_TIMING = ('[[fact]]\nkey = "token.motion.timings.in_out_sine"\nvalue = { platform = { qt = "in_out_sine" } }\n'
           'rows = []\n' + _CITE)
_PRODUCTS = _SBV + '\nvariant_products = [[{ any = ["flat"] }, { one = ["@pill"] }]]'
# ADD-A124: recency sets, `when.latest`, `Transition.from`, `Radius::Ellipse` and the interaction block
_RECENCY = 'recency = { glow = ["focused", "hover"] }'
_MAC_ID = "xcode@16.4 MacOSX.platform/Developer/SDKs/MacOSX.sdk/x.h:1"
_LATEST = ('[[fact]]\nkey = "button.body.fill"\nwhen = { conditions = ["preset:macos-monterey"], latest = { glow = '
           '"hover" } }\nvalue = "#ffffff"\nrows = []\n' + _MAC_CITE)
_LATEST_PENDING = ('[[fact]]\nkey = "button.body.fill"\nwhen = { conditions = ["preset:macos-monterey"], latest = { glow '
                   '= "hover" } }\npending = "U3"\nrows = []\n\n[[fact]]\nkey = "button.body.fill"\nwhen = { conditions = '
                   '["preset:macos-monterey"], latest = { glow = "focused" } }\nvalue = "#ffffff"\nrows = []\n' + _MAC_CITE)
_FROM = ('[[fact]]\nkey = "button.body.transitions"\nvalue = [{ properties = ["fill"], duration_ms = 150, timing = '
         '"linear", from = ["pressed", "focused"] }, { properties = ["state_layer"], duration_ms = 15, timing = "linear" }]'
         '\nrows = []\n' + _CITE)
_FOCUS_TRANSITION = ('[[fact]]\nkey = "token.focus.transition"\nvalue = { properties = ["fill"], duration_ms = 100, timing = '
                     '"linear", from = "none" }\nrows = []\n' + _CITE)
_ELLIPSE = "".join(f'[[fact]]\nkey = "button.body.corners.{c}"\nwhen = {{ variants = ["flat"] }}\nvalue = "ellipse"\n'
                   f'rows = []\n{_CITE}\n' for c in ("top_left", "top_right", "bottom_right", "bottom_left")) + (
           '[[fact]]\nkey = "button.body.corners.curve"\nwhen = { variants = ["flat"] }\nvalue = "circular"\nrows = []\n'
           + _CITE)
_INTERACTION = ('[[fact]]\nkey = "button.body.interaction.state_layer.alpha"\nwhen = { states = ["hover"] }\nvalue = 0.08\n'
                'rows = []\n' + _CITE + '\n[[fact]]\nkey = "button.body.interaction.ripple.effect"\nvalue = '
                '"android_patterned"\nrows = []\n' + _CITE)

# (label, file, old, new, the findings wanted: substrings, one per finding)
SELF_TEST_CASES = [
    # libadwaita derivation over two files
    ("libadwaita: an unknown function", "docs/facts/gnome/buttons.toml", "alpha(surface", "alfa(surface",
     ["`alfa` is not a function of S2.2's table"]),
    ("libadwaita: an unknown part", "docs/facts/gnome/buttons.toml", '"button.body.fill"', '"button.bezel.fill"',
     ["`bezel` is not a part of button"]),
    ("libadwaita: a variant of another widget", "docs/facts/gnome/buttons.toml", '["flat"]', '["outline"]',
     ["'outline' is not a variant of button"]),
    ("libadwaita: a row in no gap map", "docs/facts/gnome/buttons.toml", '["GN-1.29"]', '["GN-1.29", "GN-9.99"]',
     ["'GN-9.99' is in no gap map"]),
    ("libadwaita: a source not pinned", "docs/facts/gnome/buttons.toml", "libadwaita@1.10.0 src/stylesheet/_colors",
     "libadwaita@1.9.0 src/stylesheet/_colors", ["'libadwaita@1.9.0' is not a [[source]]"]),
    ("libadwaita: a path out of its checkout", "docs/facts/gnome/buttons.toml",
     "libadwaita@1.10.0 src/stylesheet/_colors", "libadwaita@1.10.0 ../stylesheet/_colors",
     ["is not relative to its checkout"]),
    ("libadwaita: a source without lines", "docs/facts/gnome/buttons.toml", "_colors.scss:3", "_colors.scss",
     ["is not `<id> <path>:<line or l1-l2>`"]),
    # Breeze
    ("Breeze: a role on states", "docs/facts/kde/colors.toml", 'value = "#eff0f1"',
     'when = { states = ["hover"] }\nvalue = "#eff0f1"', ["`states` on a colour-role, token or input key"]),
    ("Breeze: an unknown role", "docs/facts/kde/colors.toml", '"role.window.background"', '"role.window.bg"',
     ["names no colour role"]),
    ("Breeze: a colour as a number", "docs/facts/kde/colors.toml", 'value = "#eff0f1"', "value = 15724785",
     ["a number where the key is a colour"]),
    ("Breeze: a short hex", "docs/facts/kde/colors.toml", 'value = "#eff0f1"', 'value = "#eff"',
     ["is not a colour literal"]),
    # WinUI
    ("WinUI: a duration without its unit", "docs/facts/windows/buttons.toml", 'value = "83ms"', "value = 83",
     ["a duration is spelled with its unit"]),
    ("WinUI: a duration in seconds", "docs/facts/windows/buttons.toml", 'value = "83ms"', 'value = "0.083s"',
     ["is not a value as S11 spells it"]),
    ("WinUI: an unknown widget value", "docs/facts/windows/buttons.toml", "button.values.repeat_delay",
     "button.values.repeat_pause", ["`repeat_pause` is not a widget value of button"]),
    ("WinUI: a key with its unit suffix", "docs/facts/windows/buttons.toml", '"button.body.min_width"',
     '"button.body.min_width_px"', ["`min_width_px` carries TOML's unit suffix"]),
    ("WinUI: a widget value with its unit suffix", "docs/facts/windows/buttons.toml", '"button.values.repeat_delay"',
     '"button.values.repeat_delay_ms"', ["`repeat_delay_ms` carries TOML's unit suffix"]),
    ("WinUI: a length as a number", "docs/facts/windows/buttons.toml", 'value = "80px"', "value = 80",
     ["a length is spelled with its unit"]),
    ("WinUI: a number in a string", "docs/facts/windows/buttons.toml", 'value = "80px"', 'value = "80"',
     ["a number is a TOML number"]),
    ("WinUI: a length in ms", "docs/facts/windows/buttons.toml", 'value = "80px"', 'value = "80ms"',
     ["'80ms' is not a length"]),
    ("WinUI: a shadow leaf with its unit", "docs/facts/windows/buttons.toml", "y_px = 4,", 'y_px = "4px",',
     ["a suffixed key takes a plain number"]),
    ("WinUI: a negative tolerance", "docs/facts/windows/buttons.toml", '"layers.0.color" = 2', '"layers.0.color" = -2',
     ["tolerance 'layers.0.color' is not a number >= 0"]),
    ("WinUI: a measured citation with a quote", "docs/facts/windows/buttons.toml", "tolerance = { ",
     'quote = "x"\ntolerance = { ', ["measured citation: `quote` is not allowed"]),
    ("WinUI: an unknown conversion", "docs/facts/windows/buttons.toml", "tolerance = { ",
     'conversion = ["percent"]\ntolerance = { ', ["conversion 'percent' is not one of"]),
    ("WinUI: a read fact stating a literal", "docs/facts/windows/buttons.toml", 'kind = "published"\nsource = '
     '"microsoft-ui-xaml@2.5.1 controls/dev/CommonStyles/Common', 'kind = "read"\nsource = '
     '"microsoft-ui-xaml@2.5.1 controls/dev/CommonStyles/Common', ["states a plain `= input.…` reference"]),
    # macOS shared by 15 and 26
    ("macOS: a generation not served", "docs/facts/macos/buttons.toml",
     '\n[[fact.cite]]\nkind = "measured"\nevidence = "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-26',
     '\n[[fact.cite]]\nkind = "measured"\nevidence = "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-15',
     ["no citation serves macos-tahoe"]),
    ("macOS: a preset fact cites another generation", "docs/facts/macos/buttons.toml", 'value = "13pt"',
     'when = { conditions = ["preset:macos-tahoe"] }\nvalue = "13pt"', ["does not serve macos-tahoe"]),
    ("macOS: a preset of another directory", "docs/facts/macos/buttons.toml", 'value = "13pt"',
     'when = { conditions = ["preset:ios-26"] }\nvalue = "13pt"', ["'preset:ios-26' is not a preset of macos/",
                                                                   "does not serve ios-26",
                                                                   "does not serve ios-26"]),
    ("macOS: a font size in deg", "docs/facts/macos/buttons.toml", 'value = "13pt"', 'value = "13deg"',
     ["'13deg' is not a font size"]),
    ("macOS: no citation", "docs/facts/macos/buttons.toml", "\n[[fact.cite]]", "\n[[fact.ref]]",
     ["`ref` is not a fact's field", "no citation serves macos-monterey"]),
    # pull_to_refresh absent on macos-tahoe, and an available entry
    ("pull_to_refresh: a declaration while unavailable", "docs/facts/macos/state-models.toml",
     "available = false\n", 'available = false\nvariants = ["flat"]\n',
     ["`variants` on an entry with available = false"]),
    ("pull_to_refresh: an unknown widget", "docs/facts/macos/state-models.toml", 'widget = "pull_to_refresh"',
     'widget = "pull_down"', ["widget 'pull_down' is not a catalogue entry"]),
    ("pull_to_refresh: when with states", "docs/facts/macos/state-models.toml",
     'when = { conditions = ["preset:macos-tahoe"] }', 'when = { states = ["hover"] }',
     ["holds only `preset:` or `flavor:` conditions", "no citation serves macos-monterey"]),
    ("pull_to_refresh: absent = [] (no name for it)", "docs/facts/macos/state-models.toml",
     'absent = ["refreshControl", "NSRefreshControl"]', "absent = []", []),
    ("pull_to_refresh: an empty absent name", "docs/facts/macos/state-models.toml",
     'absent = ["refreshControl", "NSRefreshControl"]', 'absent = [""]', ["absent is not a list of names"]),
    ("model: absent on a read citation", "docs/facts/macos/state-models.toml",
     'kind = "published"\nsource = "xcode@26.6', 'kind = "read"\nsource = "xcode@26.6',
     ["read citation: `absent` is not allowed"]),
    ("model: an available entry without axes", "docs/facts/macos/state-models.toml",
     'axes = { states = ["hover", "pressed", "disabled", "focused"], conditions = ["window_inactive", "scheme:dark"], '
     'contexts = [{ widget = "toolbar", part = "bar" }] }\n', "",
     ["missing `axes`"]),
    ("model: an exclusive state outside the axes", "docs/facts/macos/state-models.toml",
     '[["disabled", "pressed", "hover"]]', '[["disabled", "pressed", "checked"]]', ["'checked' is not in axes.states"]),
    ("model: a default size not among sizes", "docs/facts/macos/state-models.toml", 'default_size = "regular"',
     'default_size = "mini"', ["default_size 'mini' is not among sizes"]),
    ("model: a context part of another widget", "docs/facts/macos/state-models.toml",
     'contexts = [{ widget = "toolbar", part = "bar" }]', 'contexts = [{ widget = "toolbar", part = "label" }]',
     ["'label' is not a part of toolbar"]),
    ("model: a sizes_by_variant variant not declared", "docs/facts/macos/state-models.toml",
     '{ variant = "@pill", sizes', '{ variant = "primary", sizes', ["'primary' is not among variants"]),
    ("model: a not_drawn condition unknown", "docs/facts/macos/state-models.toml",
     'not_drawn = [{ states = ["hover"], conditions = ["window_inactive"] }]',
     'not_drawn = [{ states = ["hover"], conditions = ["window_hidden"] }]', ["'window_hidden' is not a condition"]),
    ("model: an unknown size", "docs/facts/macos/state-models.toml", '["small", "regular", "large"]',
     '["small", "regular", "huge"]', ["'huge' is not a size"]),
    # a state model's variant declarations (S10, S12, S17; ADD-A121)
    ("model: variant groups, sets and defaults", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\nexclusive_variants = [["flat", "@pill"]]\ndefault_size_by_variant = [{ variant = "@pill", size = '
     '"regular" }]', []),
    ("model: a variant set", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\nvariant_sets = [["flat", "@pill"]]', []),
    ("model: an exclusive group naming an undeclared variant", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\nexclusive_variants = [["flat", "primary"]]', ["exclusive_variants: 'primary' is not among variants"]),
    ("model: an exclusive group naming a variant twice", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\nexclusive_variants = [["flat", "flat"]]', ["a group names 'flat' twice"]),
    ("model: a variant set of one exclusive group", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\nexclusive_variants = [["flat", "@pill"]]\nvariant_sets = [["flat", "@pill"]]',
     ["holds @pill, flat of one exclusive_variants group"]),
    ("model: a variant set of one variant", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\nvariant_sets = [["flat"]]', ["holds fewer than two variants"]),
    ("model: a variant set naming an undeclared variant", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\nvariant_sets = [["flat", "primary"]]', ["'primary' is not among variants"]),
    ("model: a variant set listed twice", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\nvariant_sets = [["flat", "@pill"], ["@pill", "flat"]]', ["is listed twice"]),
    ("model: a variant set not a list of lists", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\nvariant_sets = ["flat", "@pill"]', ["variant_sets is not a list of lists of variants"]),
    ("model: a default size the variant does not take", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\ndefault_size_by_variant = [{ variant = "@pill", size = "small" }]',
     ["default_size_by_variant: '@pill' does not take the size 'small'"]),
    ("model: a default size for an undeclared variant", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\ndefault_size_by_variant = [{ variant = "primary", size = "small" }]',
     ["default_size_by_variant: 'primary' is not among variants"]),
    ("model: two default sizes for one variant", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\ndefault_size_by_variant = [{ variant = "flat", size = "small" }, { variant = "flat", size = "large" }]',
     ["two entries name 'flat'"]),
    ("model: a default size entry without its size", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\ndefault_size_by_variant = [{ variant = "flat" }]', ["is not a list of { variant, size }"]),
    ("model: variant groups while unavailable", "docs/facts/macos/state-models.toml", "available = false\n",
     'available = false\nexclusive_variants = [["flat", "@pill"]]\n',
     ["`exclusive_variants` on an entry with available = false"]),
    # a whole part's fact (S17, ADD-A121)
    ("part: absent", "docs/facts/gnome/part.toml", None, _PART, []),
    ("part: drawn as another widget's part", "docs/facts/gnome/part.toml", None,
     _PART.replace('"absent"', '{ drawn_as = { widget = "toolbar", part = "bar" } }'), []),
    ("part: drawn as a widget with its variants", "docs/facts/gnome/part.toml", None,
     _PART.replace('"absent"', '{ drawn_as = { widget = "button", variants = ["flat", "@pill"] } }'), []),
    ("part: a component's part absent", "docs/facts/gnome/part.toml", None,
     _PART.replace('"button.mark"', '"component.led.body"'), []),
    ("part: neither absent nor drawn_as", "docs/facts/gnome/part.toml", None, _PART.replace('"absent"', '"none"'),
     ["its value is \"absent\" or { drawn_as"]),
    ("part: drawn_as a part of another widget", "docs/facts/gnome/part.toml", None,
     _PART.replace('"absent"', '{ drawn_as = { widget = "toolbar", part = "label" } }'),
     ["'label' is not a part of toolbar"]),
    ("part: drawn_as without a widget", "docs/facts/gnome/part.toml", None,
     _PART.replace('"absent"', '{ drawn_as = { part = "bar" } }'), ["missing `widget`"]),
    ("part: drawn_as with an unknown key", "docs/facts/gnome/part.toml", None,
     _PART.replace('"absent"', '{ drawn_as = { widget = "toolbar", parts = "bar" } }'), ["unknown key `parts`"]),
    ("part: drawn_as a variant of no widget", "docs/facts/gnome/part.toml", None,
     _PART.replace('"absent"', '{ drawn_as = { widget = "toolbar", variants = ["flat"] } }'),
     ["'flat' is not a variant"]),
    ("part: a part the widget lacks", "docs/facts/gnome/part.toml", None,
     _PART.replace('"button.mark"', '"button.bezel"'),
     ["is not `button.<part>.<field>`, `button.<part>` or `button.values.<name>`"]),
    # a spring by its damping ratio, and enum names, read from the core (S2.7, S11; ADD-A121)
    ("spring: by its damping ratio", "docs/facts/gnome/motion.toml", None, _SPRING, []),
    ("spring: a damping coefficient", "docs/facts/gnome/motion.toml", None,
     _SPRING.replace("damping_ratio = 1", "damping = 20"), ["`damping` is not a field of Timing::spring"]),
    ("spring: a variant the timing lacks", "docs/facts/gnome/motion.toml", None,
     _SPRING.replace("{ spring", "{ sprung"),
     ["a variant of Timing with data is a one-key table naming it"]),
    ("spring: a unit variant", "docs/facts/gnome/motion.toml", None,
     _SPRING.replace(_SPRING_TIMING, '"linear"'), []),
    ("spring: a name of no variant", "docs/facts/gnome/motion.toml", None,
     _SPRING.replace(_SPRING_TIMING, '"lineal"'), ["'lineal' is not a unit variant of Timing"]),
    ("spring: a property group of none", "docs/facts/gnome/motion.toml", None, _SPRING.replace('["fill"]', '["fil"]'),
     ["value[0].properties[0]: 'fil' is not a unit variant of PropertyGroup"]),
    ("Material: the CMF variant", "docs/facts/gnome/motion.toml", None, _CMF, []),
    ("Material: a variant of none", "docs/facts/gnome/motion.toml", None, _CMF.replace('"cmf"', '"cmff"'),
     ["'cmff' is not a unit variant of MaterialVariant"]),
    # text values (S11's text row)
    ("text: a family as a number", "docs/facts/gnome/text.toml", 'value = "Adwaita Sans"', "value = 11",
     ["a number where the key is a text value"]),
    ("text: a family as a list", "docs/facts/gnome/text.toml", 'value = "Adwaita Sans"', 'value = ["Adwaita Sans"]',
     ["a list where the key is a text value"]),
    ("text: a free string where the key is no text", "docs/facts/gnome/text.toml", '"button.label.text.font.family"',
     '"button.label.text.font.size"', ["'Adwaita Sans' is not a value as S11 spells it"]),
    ("text: a call", "docs/facts/gnome/text.toml", 'value = "Adwaita Sans"',
     'value = "= mix(token.icons.theme, token.icons.theme, 0.5)"', ["one reference to a text field"]),
    ("text: a reference to a text field", "docs/facts/gnome/text.toml", 'value = "Adwaita Sans"',
     'value = "= token.icons.theme"', []),
    ("text: a reference to a number", "docs/facts/gnome/text.toml", 'value = "Adwaita Sans"',
     'value = "= token.kde.disabled.intensity_amount"', ["is a number, not a text field"]),
    ("text: a list entry not text", "docs/facts/gnome/text.toml", '["Adwaita Sans", "sans-serif"]',
     '["Adwaita Sans", 3]', ["value[1]: a number where the key is a text value"]),
    ("text: a glyph name in a variant", "docs/facts/gnome/text.toml", '{ name = "go-next-symbolic", fallbacks',
     '{ name = "auto", fallbacks', []),
    ("text: a glyph name outside its variant", "docs/facts/gnome/text.toml", "{ theme_icon = { name",
     "{ symbol = { name", ["a variant of GlyphSource with data is a one-key table naming it",
                           "'go-next-symbolic' is not a value as S11 spells it",
                           "value.symbol.fallbacks[0]: 'go-next' is not a value"]),
    ("text: a measured family with a tolerance", "docs/facts/macos/family.toml", None,
     '[[fact]]\nkey = "button.label.text.font.family"\nvalue = ".AppleSystemUIFont"\nrows = []\n\n'
     '[[fact.cite]]\nkind = "measured"\nevidence = "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-15/'
     'probe.toml#macos.fonts.family"\ntolerance = 0\n\n'
     '[[fact.cite]]\nkind = "measured"\nevidence = "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-26/'
     'probe.toml#macos.fonts.family"\ntolerance = 1\n', ["its measured citation's tolerance is 0"]),
    # measured derivations (S17)
    ("derivation: no inputs", "docs/facts/macos/buttons.toml", 'inputs = ["input.accent"]\n', "",
     ["missing `inputs`"]),
    ("derivation: inputs on a literal value", "docs/facts/macos/buttons.toml", "systemFontSize\"\ntolerance = 0\n",
     'systemFontSize"\ntolerance = 0\ninputs = ["input.accent"]\n', ["`inputs` is only on a measured citation"]),
    ("derivation: an input the expression lacks", "docs/facts/macos/buttons.toml", 'inputs = ["input.accent"]',
     'inputs = ["input.accent", "input.focus"]', ["'input.focus' is not a reference of the expression"]),
    ("derivation: a conversion beside inputs", "docs/facts/macos/buttons.toml", 'inputs = ["input.accent"]',
     'inputs = ["input.accent"]\nconversion = ["percent_to_fraction"]', ["no `conversion` beside `inputs`"]),
    ("derivation: an input twice", "docs/facts/macos/buttons.toml", 'inputs = ["input.accent"]',
     'inputs = ["input.accent", "input.accent"]', ["'input.accent' listed twice"]),
    ("derivation: inputs on a published citation", "docs/facts/gnome/buttons.toml", 'quote = "$hover_color',
     'inputs = ["surface.foreground"]\nquote = "$hover_color', ["published citation: `inputs` is not allowed"]),
    # KDE accent run
    ("KDE accent: scheme_file without accent", "docs/facts/kde/colors.toml", 'accent = "#3daee9"\n', "",
     ["scheme_file and accent come both or neither"]),
    ("KDE accent: a scheme file entry not pinned", "docs/facts/kde/colors.toml",
     'scheme_file = "breeze@6.8.2 colors', 'scheme_file = "breeze@6.8.1 colors',
     ["'breeze@6.8.1' is not a [[source]]"]),
    ("KDE accent: an evidence path out of form", "docs/facts/kde/colors.toml", "2026-11-03-run-2/kde-breeze-light/",
     "2026-11-03/kde-breeze-light/", ["is not `docs/evidence/<harness>/<date>-run-<id>/<image>/<file>#<path>`",
                                      "no citation serves kde-breeze"]),
    ("KDE accent: an image run.toml maps elsewhere", "docs/evidence/kde-accent/2026-11-03-run-2/run.toml",
     'preset = "kde-breeze"', 'preset = "adwaita"', ["no citation serves kde-breeze"]),
    # accent fields outside kde/
    ("accent outside kde", "docs/facts/gnome/buttons.toml", 'rows = ["GN-1.29"]',
     'rows = ["GN-1.29"]\nscheme_file = "libadwaita@1.10.0 x.css"\naccent = "#3584e4"',
     ["are the kde/ accent run's only"]),
    # Catppuccin palette fact and the bundled palette
    ("Catppuccin: rows on a palette fact", "docs/facts/palettes/catppuccin.toml", 'value = "#313244"',
     'value = "#313244"\nrows = []', ["`rows` is not allowed"]),
    ("Catppuccin: a key of another project", "docs/facts/palettes/catppuccin.toml", '"catppuccin.mocha.surface0"',
     '"dracula.mocha.surface0"', ["is not `catppuccin.<flavour>.<name>`",
                                  "'catppuccin.mocha.surface0' is not a palette-fact key"]),
    ("Catppuccin: a source that serves a generation", "docs/facts/palettes/catppuccin.toml", "catppuccin@1.7.1",
     "breeze@6.8.2", ["is not a palette entry"]),
    ("Catppuccin: a value not a colour", "docs/facts/palettes/catppuccin.toml", 'value = "#313244"',
     'value = "surface0"', ["is not a colour literal"]),
    ("palette: a source naming no fact", "native-theme/src/next/presets/catppuccin-mocha.toml",
     '= "catppuccin.mocha.surface0"', '= "catppuccin.mocha.surface9"', ["is not a palette-fact key"]),
    ("palette: a colour in both", "native-theme/src/next/presets/catppuccin-mocha.toml",
     '"colors.dark.surfaces.window.foreground" = "the guide', '"colors.dark.surfaces.window.background" = "the guide',
     ["is in both", "is in neither"]),
    ("palette: a pick naming no colour", "native-theme/src/next/presets/catppuccin-mocha.toml",
     '"colors.dark.surfaces.window.foreground" = "the guide', '"colors.dark.surfaces.window.fg" = "the guide',
     ["'colors.dark.surfaces.window.fg' is not a stated colour literal's path", "is in neither"]),
    # a pending measurement (S17, ADD-A122)
    ("pending: a pending fact after the facts that hold where it does", "docs/facts/gnome/pending.toml", None,
     _FILL + _PENDING, []),
    ("pending: with a value", "docs/facts/gnome/pending.toml", None,
     _FILL + _PENDING.replace('pending = "U1"', 'pending = "U1"\nvalue = "#000000"'),
     ["`pending` in place of `value`: exactly one of them"]),
    ("pending: with a citation", "docs/facts/gnome/pending.toml", None, _FILL + _PENDING + _CITE,
     ["a pending fact has no [[fact.cite]]"]),
    ("pending: an id not of the register's form", "docs/facts/gnome/pending.toml", None,
     _FILL + _PENDING.replace('"U1"', '"U01"'), ["pending 'U01' is not an Unmeasured register Id"]),
    ("pending: an id not a string", "docs/facts/gnome/pending.toml", None,
     _FILL + _PENDING.replace('"U1"', '1'), ["pending 1 is not an Unmeasured register Id"]),
    ("pending: without rows", "docs/facts/gnome/pending.toml", None, _FILL + _PENDING.replace("rows = []\n", ""),
     ["pending fact: missing `rows`"]),
    ("pending: a when naming no state", "docs/facts/gnome/pending.toml", None,
     _FILL + _PENDING.replace('"pressed"', '"pushed"'), ["'pushed' is not a state flag"]),
    ("pending: shadowed by a later fact holding everywhere", "docs/facts/gnome/pending.toml", None,
     _PENDING + _LATER.replace('when = { states = ["hover"] }\n', ""), ["is shadowed by the later fact at line 8"]),
    ("pending: shadowed where both hold (no exclusive group on gnome/)", "docs/facts/gnome/pending.toml", None,
     _PENDING + _LATER, ["is shadowed by the later fact"]),
    ("pending: a later fact refusing its state", "docs/facts/gnome/pending.toml", None,
     _PENDING + _LATER.replace('states = ["hover"]', 'not_states = ["pressed"]'), []),
    ("pending: a later fact under the other scheme", "docs/facts/gnome/pending.toml", None,
     _PENDING.replace('when = {', 'when = { conditions = ["scheme:dark"],')
     + _LATER.replace('when = {', 'when = { conditions = ["scheme:light"],'), []),
    ("pending: a later fact of another key", "docs/facts/gnome/pending.toml", None,
     _PENDING + _LATER.replace('"button.body.fill"', '"button.body.stroke"').replace('"#eeeeee"', '"absent"'), []),
    ("pending: two pending facts naming one row", "docs/facts/gnome/pending.toml", None,
     _FILL + _PENDING + _PENDING.replace('"pressed"', '"focused"').replace("button.body.fill", "button.body.stroke"),
     []),
    ("pending: a later state of one exclusive group (macos-monterey's model)", "docs/facts/macos/pending.toml", None,
     _MAC_PENDING, []),
    ("pending: a later fact on a variant set the model does not draw", "docs/facts/macos/pending.toml", None,
     _MAC_PENDING.replace('states = ["hover"]', 'variants = ["flat", "@pill"]'), []),
    ("pending: a later fact on a single variant", "docs/facts/macos/pending.toml", None,
     _MAC_PENDING.replace('states = ["hover"]', 'variants = ["flat"]'), ["is shadowed by the later fact"]),
    # a state model's variant products (S17, ADD-A122)
    ("products: a product's set", "docs/facts/macos/state-models.toml", _SBV, _PRODUCTS, []),
    ("products: beside a listed set", "docs/facts/macos/state-models.toml", _SBV,
     _SBV + '\nvariant_products = [[{ any = ["flat"] }, { one = ["@pill"] }], '
     '[{ one = ["flat"] }, { one = ["@pill"] }]]\nvariant_sets = []', []),
    ("products: a set also listed", "docs/facts/macos/state-models.toml", _SBV,
     _PRODUCTS + '\nvariant_sets = [["@pill", "flat"]]', ["is also a set variant_products gives"]),
    ("products: a set the exclusive groups refuse is trimmed, not listed twice", "docs/facts/macos/state-models.toml",
     _SBV, _PRODUCTS + '\nexclusive_variants = [["flat", "@pill"]]', []),
    ("products: a variant not declared", "docs/facts/macos/state-models.toml", _SBV,
     _PRODUCTS.replace('{ any = ["flat"] }', '{ any = ["flat", "primary"] }'),
     ["variant_products[0]: 'primary' is not among variants"]),
    ("products: one variant in two groups", "docs/facts/macos/state-models.toml", _SBV,
     _PRODUCTS.replace('{ any = ["flat"] }', '{ any = ["flat", "@pill"] }'),
     ["variant_products[0] names '@pill' in two of its groups"]),
    ("products: a group of neither form", "docs/facts/macos/state-models.toml", _SBV,
     _PRODUCTS.replace('{ one = ["@pill"] }', '{ all = ["@pill"] }'), ["variant_products is not a list of products"]),
    ("products: an empty group", "docs/facts/macos/state-models.toml", _SBV,
     _PRODUCTS.replace('{ one = ["@pill"] }', '{ one = [] }'), ["variant_products is not a list of products"]),
    ("products: on an unavailable entry", "docs/facts/macos/state-models.toml", "available = false\n",
     'available = false\nvariant_products = [[{ one = ["flat"] }, { one = ["@pill"] }]]\n',
     ["`variant_products` on an entry with available = false"]),
    # S4.1's motion keys (ADD-A122)
    ("motion: a key the rule names", "docs/facts/gnome/motion.toml", None, _MOTION, []),
    ("motion: a family's key where the row names a family", "docs/facts/gnome/motion.toml", None,
     _MOTION.replace("durations.button", "durations.tab_bar"), []),
    ("motion: a dropped word kept", "docs/facts/gnome/motion.toml", None,
     _MOTION.replace("durations.button", "durations.scroll_animation"), ["keeps `animation`"]),
    ("motion: a dropped word pair kept", "docs/facts/gnome/motion.toml", None,
     _MOTION.replace("durations.button", "timings.control_key_spline").replace('"200ms"', '"linear"'),
     ["keeps `key`, `spline`"]),
    ("motion: a name of dropped words only", "docs/facts/kde/motion.toml", None,
     _MOTION.replace("durations.button", "durations.animations").replace(
         "libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:10-11", "breeze@6.8.2 kstyle/breezestyle.cpp:1"), []),
    ("motion: a toolkit prefix kept", "docs/facts/gnome/motion.toml", None,
     _MOTION.replace("durations.button", "durations.adw_toast_overlay_show"), ["keeps the toolkit's prefix `adw`"]),
    ("motion: a key the platform's row does not list", "docs/facts/kde/motion.toml", None,
     _MOTION.replace("durations.button", "durations.breeze_state").replace(
         "libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:10-11", "breeze@6.8.2 kstyle/breezestyle.cpp:1"),
     ["`breeze_state` is not a key the catalogue's *token.motion.durations* row for kde/ lists"]),
    ("motion: a spring's word kept", "docs/facts/gnome/motion.toml", None,
     _MOTION.replace("durations.button", "timings.spring_default_spatial").replace('"200ms"', '"linear"'),
     ["keeps `spring`"]),
    ("motion: a duration below its key", "docs/facts/gnome/motion.toml", None,
     _MOTION.replace("durations.button", "durations.button.x"),
     ["a duration or timing is `token.motion.durations.<key>`"]),
    # a map keyed by text: its entry's key as TOML writes it, a quoted segment one segment (S11, S17; ADD-A123)
    ("text key: a quoted family", "docs/facts/gnome/fallbacks.toml", None, _FALLBACK, []),
    ("text key: a family in single quotes, a dot in it", "docs/facts/gnome/fallbacks.toml", None,
     _FALLBACK.replace("\'token.text.fallbacks.per_family.\"Segoe Fluent Icons\"\'",
                       '"token.text.fallbacks.per_family.\'.AppleSystemUIFont\'"'), []),
    ("text key: a bare family", "docs/facts/gnome/fallbacks.toml", None,
     _FALLBACK.replace('"Segoe Fluent Icons"', "Cantarell"), []),
    ("text key: a bare language tag", "docs/facts/gnome/fallbacks.toml", None,
     _FALLBACK.replace('per_family."Segoe Fluent Icons"', "per_language.zh-Hans"), []),
    ("text key: an unterminated quote", "docs/facts/gnome/fallbacks.toml", None,
     _FALLBACK.replace('"Segoe Fluent Icons"', '"Segoe Fluent Icons'), ["is not terminated", _UNTYPED]),
    ("text key: a family that needs its quotes", "docs/facts/gnome/fallbacks.toml", None,
     _FALLBACK.replace('"Segoe Fluent Icons"', "Segoe Fluent Icons"), ["is bare where TOML needs it quoted", _UNTYPED]),
    ("text key: a quote inside a bare segment", "docs/facts/gnome/fallbacks.toml", None,
     _FALLBACK.replace('"Segoe Fluent Icons"', 'Segoe"Fluent"'), ["a quote inside the segment", _UNTYPED]),
    ("text key: a quoted segment outside a text map", "docs/facts/gnome/fallbacks.toml", None,
     _FALLBACK.replace("fallbacks.per_family", '"fallbacks".per_family'),
     ["the quoted segment 'fallbacks' is not an entry of a map keyed by text", _UNTYPED]),
    ("text key: a quoted segment below the entry", "docs/facts/gnome/fallbacks.toml", None,
     _FALLBACK.replace('Icons"', 'Icons"."x"'), ["the quoted segment 'x' is not an entry", _UNTYPED]),
    ("text key: an empty segment", "docs/facts/gnome/fallbacks.toml", None,
     _FALLBACK.replace("per_family.", "per_family.."), ["has an empty segment", _UNTYPED]),
    # S4.1's MotionTokens: a timing key is a Timing, a platform curve one of the core's (S2.7; ADD-A123)
    ("timing: a Qt curve", "docs/facts/gnome/motion.toml", None, _TIMING, []),
    ("timing: a libadwaita curve", "docs/facts/gnome/motion.toml", None,
     _TIMING.replace('qt = "in_out_sine"', 'adw = "ease_in_out_sine"'), []),
    ("timing: Kirigami's curve", "docs/facts/gnome/motion.toml", None,
     _TIMING.replace('{ qt = "in_out_sine" }', '"kirigami_velocity"'), []),
    ("timing: a cubic Bézier", "docs/facts/gnome/motion.toml", None,
     _TIMING.replace('{ platform = { qt = "in_out_sine" } }', "{ cubic_bezier = [0.4, 0, 0.2, 1] }"), []),
    ("timing: a Qt curve outside QtEasing", "docs/facts/gnome/motion.toml", None,
     _TIMING.replace('"in_out_sine" }', '"in_out_back" }'), ["'in_out_back' is not a unit variant of QtEasing"]),
    ("timing: a libadwaita curve outside AdwEasing", "docs/facts/gnome/motion.toml", None,
     _TIMING.replace('qt = "in_out_sine"', 'adw = "ease_out"'), ["'ease_out' is not a unit variant of AdwEasing"]),
    ("timing: a toolkit outside PlatformCurve", "docs/facts/gnome/motion.toml", None,
     _TIMING.replace("qt = ", "gtk = "), ["a variant of PlatformCurve with data is a one-key table"]),
    ("timing: a duration as a timing", "docs/facts/gnome/motion.toml", None,
     _TIMING.replace('{ platform = { qt = "in_out_sine" } }', '"200ms"'), ["where the key is a Timing, an enum"]),
    ("timing: a key MotionTokens lacks", "docs/facts/gnome/motion.toml", None,
     _TIMING.replace("motion.timings.", "motion.curves."), ["names no token"]),
    # GlobalRole::AccentRamp (S3, ADD-A122), an enum stated whole (S6.1, S17), the negate conversion
    ("accent ramp: one entry", "docs/facts/windows/ramp.toml", None,
     _RAMP.replace("libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:10-11",
                   "microsoft-ui-xaml@2.5.1 controls/dev/CommonStyles/Common_themeresources_any.xaml:4"), []),
    ("accent ramp: none (an empty ramp)", "docs/facts/gnome/ramp.toml", None,
     _RAMP.replace("accent_ramp.light1", "accent_ramp").replace('"#ff0000"', "[]"), []),
    ("accent ramp: below an entry", "docs/facts/gnome/ramp.toml", None,
     _RAMP.replace("accent_ramp.light1", "accent_ramp.light1.red"), ["an accent ramp role is"]),
    ("enum: a key inside an enum block", "docs/facts/gnome/text.toml", '"button.mark.glyph.source"',
     '"button.mark.glyph.source.theme_icon"', ["`theme_icon` is inside GlyphSource, an enum",
                                                "'go-next-symbolic' is not a value", "'go-next' is not a value"]),
    ("negate: a negated margin", "docs/facts/windows/buttons.toml", "quote = '<x:Double",
     'conversion = ["negate"]\nquote = \'<x:Double', []),
    # the files
    ("a stray directory", "docs/facts/gtk/x.toml", None, "", ["not a facts directory"]),
    ("imported is skipped", "docs/facts/imported/rows.toml", None, "this is no TOML", []),
    ("staged is skipped", "docs/facts/staged/bases.toml", None, "this is no TOML", []),
    ("bad TOML", "docs/facts/gnome/buttons.toml", "[[fact]]\nkey", "[[fact]\nkey", ["not valid TOML"]),
    ("a fact table in state-models", "docs/facts/macos/state-models.toml", None, '[[fact]]\nkey = "button.body.fill"\n',
     ["unknown table `fact`"]),
    # a state model's recency sets (S10, S12, S17; ADD-A124); the good fixture's model states one
    ("recency: two sets in one order", "docs/facts/macos/state-models.toml", _RECENCY,
     'recency = { glow = ["focused", "hover"], lift = ["pressed", "focused", "hover"] }', []),
    ("recency: two sets ordering two flags oppositely", "docs/facts/macos/state-models.toml", _RECENCY,
     'recency = { glow = ["focused", "hover"], lift = ["hover", "pressed", "focused"] }',
     ["glow and lift order 'focused' and 'hover' oppositely"]),
    ("recency: a set of one flag", "docs/facts/macos/state-models.toml", _RECENCY, 'recency = { glow = ["hover"] }',
     ["recency.glow holds fewer than two flags"]),
    ("recency: a flag twice", "docs/facts/macos/state-models.toml", _RECENCY,
     'recency = { glow = ["hover", "focused", "hover"] }', ["recency.glow names 'hover' twice"]),
    ("recency: a flag outside the axes", "docs/facts/macos/state-models.toml", _RECENCY,
     'recency = { glow = ["checked", "hover"] }', ["recency.glow: 'checked' is not in axes.states"]),
    ("recency: an unknown flag", "docs/facts/macos/state-models.toml", _RECENCY,
     'recency = { glow = ["@glowing", "hover"] }', ["recency.glow: '@glowing' is not a state flag"]),
    ("recency: a name outside the key grammar", "docs/facts/macos/state-models.toml", _RECENCY,
     'recency = { Glow = ["focused", "hover"] }', ["'Glow' is not a key"]),
    ("recency: not a table of lists", "docs/facts/macos/state-models.toml", _RECENCY, 'recency = ["focused", "hover"]',
     ["recency is not a table of named sets"]),
    ("recency: on an unavailable entry", "docs/facts/macos/state-models.toml", "available = false\n",
     'available = false\n' + _RECENCY + '\n', ["`recency` on an entry with available = false"]),
    # a fact's `when.latest` (S11, S12, S17; ADD-A124)
    ("latest: a set and flag the model declares", "docs/facts/macos/latest.toml", None, _LATEST, []),
    ("latest: beside the flag's own states", "docs/facts/macos/latest.toml", None,
     _LATEST.replace('latest = {', 'states = ["focused", "hover"], latest = {'), []),
    ("latest: a set the model does not declare", "docs/facts/macos/latest.toml", None,
     _LATEST.replace("glow =", "lift ="), ["'lift' is not a recency set of button's state model"]),
    ("latest: a flag not in the set", "docs/facts/macos/latest.toml", None, _LATEST.replace('"hover" }', '"pressed" }'),
     ["when.latest.glow: 'pressed' is not a member of the set"]),
    ("latest: an unknown flag", "docs/facts/macos/latest.toml", None, _LATEST.replace('"hover" }', '"hovr" }'),
     ["when.latest.glow: 'hovr' is not a state flag", "'hovr' is not a member of the set"]),
    ("latest: a flag the when's not_states refuses", "docs/facts/macos/latest.toml", None,
     _LATEST.replace('latest = {', 'not_states = ["hover"], latest = {'), ["is among the when's not_states"]),
    ("latest: not a table", "docs/facts/macos/latest.toml", None,
     _LATEST.replace('latest = { glow = "hover" }', 'latest = "hover"'), ["when.latest is not a table"]),
    ("latest: no state model for the generation", "docs/facts/macos/latest.toml", None,
     _LATEST.replace("macos-monterey", "macos-tahoe").replace("xcode@16.4", "xcode@26.6"),
     ["no state model of button in macos/"]),
    ("latest: on a colour-role key", "docs/facts/gnome/latest.toml", None,
     _FILL.replace('"button.body.fill"', '"role.view.background"').replace('states = ["hover"]',
                                                                          'latest = { glow = "hover" }'),
     ["`latest` on a colour-role, token or input key"]),
    # which fact applies, order-aware (S17 *Which fact applies*; ADD-A124): a fact under `latest` holds only under
    # the orders it names, a fact without it under every order
    ("latest: a pending fact and a later one on another latest flag", "docs/facts/macos/pending.toml", None,
     _LATEST_PENDING, []),
    ("latest: a pending fact shadowed by a later fact without latest", "docs/facts/macos/pending.toml", None,
     _LATEST_PENDING.replace(', latest = { glow = "focused" }', ""), ["is shadowed by the later fact"]),
    ("latest: a pending fact shadowed on the same latest flag", "docs/facts/macos/pending.toml", None,
     _LATEST_PENDING.replace('glow = "focused"', 'glow = "hover"'), ["is shadowed by the later fact"]),
    # a transition's `from` (S2.7, S11, S12; ADD-A124)
    ("from: a list entry's from, and one left out", "docs/facts/gnome/from.toml", None, _FROM, []),
    ("from: an empty list", "docs/facts/gnome/from.toml", None, _FROM.replace('["pressed", "focused"]', "[]"),
     ["value[0].from is an empty list"]),
    ("from: a flag twice", "docs/facts/gnome/from.toml", None, _FROM.replace('"focused"]', '"pressed"]'),
     ["value[0].from names 'pressed' twice"]),
    ("from: a keyed flag the widget does not declare", "docs/facts/gnome/from.toml", None,
     _FROM.replace('"focused"]', '"@glowing"]'), ["value[0].from: '@glowing' is not a flag button declares"]),
    ("from: a keyed flag the widget declares", "docs/facts/gnome/from.toml", None, _FROM.replace('"focused"]', '"@busy"]'),
     []),
    ("from: no state flag", "docs/facts/gnome/from.toml", None, _FROM.replace('"focused"]', '"focussed"]'),
     ["'focussed' is not a unit variant of StateFlag"]),
    ("from: a single transition's any state", "docs/facts/gnome/from.toml", None, _FOCUS_TRANSITION, []),
    ("from: a single transition without it", "docs/facts/gnome/from.toml", None,
     _FOCUS_TRANSITION.replace(', from = "none"', ""), ["value: a single transition states `from`"]),
    ("state layer: a transition on a part without interaction", "docs/facts/gnome/from.toml", None,
     _FROM.replace("button.body.transitions", "pull_to_refresh.indicator.transitions"),
     ["value[1]: a `state_layer` transition on a part whose fields lack `interaction`"]),
    ("stroke width: a property group", "docs/facts/gnome/from.toml", None, _FROM.replace('["state_layer"]',
                                                                                      '["stroke_width"]'), []),
    # Radius::Ellipse (S2.4, S12; ADD-A124)
    ("ellipse: all four corners, circular", "docs/facts/gnome/ellipse.toml", None, _ELLIPSE, []),
    ("ellipse: on three corners", "docs/facts/gnome/ellipse.toml", None,
     _ELLIPSE.replace('corners.bottom_left"\nwhen = { variants = ["flat"] }\nvalue = "ellipse"',
                      'fill"\nwhen = { variants = ["flat"] }\nvalue = "#ffffff"', 1),
     ["and no fact under the same when states bottom_left"]),
    ("ellipse: beside another radius", "docs/facts/gnome/ellipse.toml", None,
     _ELLIPSE.replace('corners.top_right"\nwhen = { variants = ["flat"] }\nvalue = "ellipse"', 'corners.top_right"\nwhen '
                      '= { variants = ["flat"] }\nvalue = { full = { maximum_px = "absent" } }', 1),
     ["button.body.corners.top_right: {'full': {'maximum_px': 'absent'}} beside `ellipse`"]),
    ("ellipse: a continuous curve", "docs/facts/gnome/ellipse.toml", None,
     _ELLIPSE.replace('value = "circular"', 'value = "continuous"'), ["`continuous` with `ellipse` corners"]),
    ("ellipse: a whole corners table on some corners", "docs/facts/gnome/ellipse.toml", None,
     _CMF.replace('"token.material.variant"', '"button.body.corners"').replace(
         'value = "cmf"', 'value = { top_left = "ellipse", top_right = "ellipse", bottom_right = "ellipse", '
                          'bottom_left_px = 4, curve = "circular" }'), ["bottom_left: 4 beside `ellipse`"]),
    ("ellipse: no unit variant of that name", "docs/facts/gnome/ellipse.toml", None,
     _ELLIPSE.replace('"ellipse"', '"elipse"'), ["'elipse' is not a unit variant of Radius"] * 4),
    # PartStyle.interaction (S2.11, S2.12; ADD-A124)
    ("interaction: the state layer's alpha and the ripple's effect", "docs/facts/gnome/interaction.toml", None,
     _INTERACTION, []),
    ("interaction: a field the state layer lacks", "docs/facts/gnome/interaction.toml", None,
     _INTERACTION.replace("state_layer.alpha", "state_layer.opacity"), ["is not a field path"]),
    ("interaction: a block the interaction lacks", "docs/facts/gnome/interaction.toml", None,
     _INTERACTION.replace("interaction.state_layer.alpha", "interaction.hover.alpha"), ["is not a field path"]),
    ("interaction: an effect no RippleEffect names", "docs/facts/gnome/interaction.toml", None,
     _INTERACTION.replace('"android_patterned"', '"patterned"'), ["'patterned' is not a unit variant of RippleEffect"]),
    ("interaction: a part whose fields lack it", "docs/facts/gnome/interaction.toml", None,
     _INTERACTION.replace("button.body.interaction.state_layer", "button.label.interaction.state_layer"),
     ["`interaction` is not a field of button's part `label`"]),
]


def self_test() -> int:
    failures = []
    with tempfile.TemporaryDirectory() as tmp:
        base = Path(tmp)

        def run(files):
            root = base / f"case{run.n}"
            run.n += 1
            write_fixture(root, files)
            got, _ = check(fixture_context(root))
            return [f"{f}:{ln}: {w}" for f, ln, w in got]
        run.n = 0

        got = run(SELF_TEST_FILES)
        if got:
            failures.append("good fixtures: got " + "; ".join(got))
        for label, path, old, new, want in SELF_TEST_CASES:
            files = dict(SELF_TEST_FILES)
            if old is None:
                files[path] = new
            else:
                if old not in files[path]:
                    failures.append(f"{label}: fixture text {old!r} not found")
                    continue
                files[path] = files[path].replace(old, new, 1)
            got = run(files)
            if len(got) != len(want) or not all(any(w in g for g in got) for w in want):
                failures.append(f"{label}: got {got}, want {want}")
        # the vacuous run: no fact file, no bundled palette
        root = base / "empty"
        write_fixture(root, {})
        got, counts = check(fixture_context(root))
        if got or not summary(counts, 0).startswith("no facts yet: 0 fact files"):
            failures.append(f"empty: got {got}, {summary(counts, len(got))}")
        # the closed lists the docstring names
        ctx = fixture_context(root)
        if ctx.functions != {"alpha": (2, 2), "with_alpha": (2, 2), "mix": (3, 3), "kcu_mix": (3, 3),
                             "kcu_tint": (3, 3), "kde_shade": (3, 3), "kirigami_lerp": (3, 3), "floor": (1, 1),
                             "round": (1, 1), "pick": (3, None)}:
            failures.append(f"functions: got {ctx.functions}")
        if ctx.presets != set(ALL_PRESETS):
            failures.append(f"existing presets: got {sorted(ctx.presets)}")
        # only the new model's presets exist: today's directory and a stem that is no platform preset do not count
        root = base / "presets"
        if existing_presets(root):
            failures.append(f"existing presets, no directory: got {sorted(existing_presets(root))}")
        for rel in ("native-theme/src/presets/adwaita.toml", PRESET_DIRS[0] / "macos-tahoe.toml",
                    PRESET_DIRS[0] / "nord.toml"):
            (root / rel).parent.mkdir(parents=True, exist_ok=True)
            (root / rel).write_text("", encoding="utf-8")
        if existing_presets(root) != {"macos-tahoe"}:
            failures.append(f"existing presets, today's and the new model's: got {sorted(existing_presets(root))}")
        types = {k: key_info(ctx, k)[4] for k in ("button.label.text.font.family", "token.text.fallbacks.families",
                                                   "button.mark.glyph.source", "token.icons.theme")}
        if types != {"button.label.text.font.family": "Value<String>", "token.text.fallbacks.families": "Vec<String>",
                     "button.mark.glyph.source": "GlyphSource", "token.icons.theme": "Value<String>"}:
            failures.append(f"key types: got {types}")
        got = text_leaves(ctx, {"theme_icon": {"name": "a", "fallbacks": ["b", "c"]}}, "GlyphSource")
        if got != [("theme_icon.name", "a"), ("theme_icon.fallbacks.0", "b"), ("theme_icon.fallbacks.1", "c")]:
            failures.append(f"text leaves: got {got}")
        if expression_refs(" alpha(input.accent, mix(role.view.background, input.accent, 0.5))") != [
                "input.accent", "role.view.background"]:
            failures.append(f"expression refs: got {expression_refs('alpha(input.accent, 0.5)')}")
        dims = {k: key_info(ctx, k)[1] for k in ("button.body.min_width", "button.label.text.font.size",
                                                  "button.body.padding.top", "button.values.icon_text_gap",
                                                  "token.motion.durations.button", "role.view.foreground",
                                                  "button.body.shadow", "token.kde.disabled.intensity_amount")}
        want = {"button.body.min_width": "length", "button.label.text.font.size": "font_size",
                "button.body.padding.top": "length", "button.values.icon_text_gap": "length",
                "token.motion.durations.button": "duration", "role.view.foreground": "colour",
                "button.body.shadow": None, "token.kde.disabled.intensity_amount": "number"}
        if dims != want:
            failures.append(f"key dimensions: got {dims}")
        for key, problem in (("input.accent", None), ("input.kde_scheme.header.inactive.background_normal", None),
                             ("input.kde_scheme.groups", None), ("input.inactive.view.foreground_normal", None),
                             ("input.kde_scheme.nope.background_normal", "not `input.kde_scheme"),
                             ("component.led.body.fill", None), ("component.lamp.body.fill", "not a catalogue entry"),
                             ("button.label.text.nope", "is not a field path"), ("Button.body.fill", "snake_case")):
            got = key_info(ctx, key)[0]
            if (problem is None) != (got is None) or (problem and problem not in got):
                failures.append(f"key {key}: got {got!r}, want {problem!r}")
        for expr, want in (("= mix(role.view.background, #ffffff, 0.5)", []),
                           ("= alpha(srgb(0.1, 0.2, 0.3), 1.5)", []),
                           ("= active(role.view.background)", []),
                           ("= kcu_tint(surface.background, token.focus, 0.5", ["expected `,` or `)`"]),
                           ("= srgb(1, 2)", ["three or four numbers"]),
                           ("= element.nope", ["names no element data"]),
                           ("= surface.nope", ["names no surface role"]),
                           ("= surface.disabled.foreground", []),
                           ("= surface.disabled", ["names no surface role"]),
                           ("= surface.disabled.nope", ["names no surface role"]),
                           ("= kde_shade(role.window.background, light, 4)", []),
                           ("= kirigami_lerp(role.window.background, role.view.foreground, 0.2)", []),
                           ("= round(mul(token.kde.frame_width, 0.5))", ["`mul` is not a function"]),
                           ("= round(token.kde.frame_width)", []),
                           ("= mix(role.global.accent_ramp.light1, role.view.background, 0.5)", []),
                           ("= role.global.accent_ramp.light1.x", ["an accent ramp role is"]),
                           # each function's arity its signature's in S2.2's table (`with_alpha`, ADD-A123)
                           ("= with_alpha(role.view.background, 0.5)", []),
                           ("= with_alpha(role.view.background)", ["`with_alpha` takes 2 arguments"]),
                           ("= with_alpha(role.view.background, 0.5, 1)", ["`with_alpha` takes 2 arguments"]),
                           ("= alpha(role.view.background)", ["`alpha` takes 2 arguments"]),
                           ("= round(token.kde.frame_width, 2)", ["`round` takes 1 argument (S2.2's table), not 2"]),
                           ("= pick(1, #ffffff, #000000, #ff0000)", []),
                           ("= pick(1, #ffffff)", ["`pick` takes 3 or more arguments"])):
            got = value_problems(ctx, expr, None)
            if len(got) != len(want) or not all(any(w in g for g in got) for w in want):
                failures.append(f"expression {expr!r}: got {got}, want {want}")
    # the sets variant products give, counted once (S17 *Variant products*: the specification's example form)
    style = {"any": ["primary", "flat", "@raised"]}
    d = {"variant_products": [[{"any": ["icon_only", "with_icon"]}, {"any": ["pill", "circular"]}, style],
                              [{"one": ["with_icon"]}, {"one": ["@image_text_box"]}, {"any": ["pill", "circular"]},
                               style],
                              [{"one": ["menu"]}, {"any": ["icon_only", "with_icon"]}, {"any": ["flat"]}]],
         "exclusive_variants": [["icon_only", "with_icon"]]}
    # the first product: 3 x 3 x 4 choices less the empty one and the 2 + 2 + 3 single ones (28); the second: every
    # one of its 3 x 4 choices (12); the third: 3 x 2 less the lone `menu` (5); none shared
    for case, want in ((d, (0, 45, 45)),
                       ({**d, "variant_sets": [["pill", "primary"], ["toggle", "primary"]]}, (2, 45, 46)),
                       ({**d, "variant_products": d["variant_products"] + [[{"one": ["pill"]}, {"one": ["primary"]}]]},
                        (0, 45, 45)),
                       ({**d, "requires_variants": ["menu"]}, (0, 5, 5)),
                       ({**d, "requires_variants": ["menu", "@image_text_box"]}, (0, 17, 17)),
                       ({**d, "requires_variants": ["menu"], "variant_sets": [["pill", "primary"], ["menu", "pill"]]},
                        (1, 5, 6)),
                       ({**d, "not_drawn": [{"variants": ["menu"]}, {"variants": ["pill"], "states": ["hover"]}]},
                        (0, 40, 40)),
                       ({**d, "exclusive_variants": [["icon_only", "with_icon"], ["pill", "primary"]]},
                        (0, 41, 41))):
        if drawn_variant_sets(case) != want:
            failures.append(f"variant set counts: got {drawn_variant_sets(case)}, want {want} for {case}")
    # two whens a combination may hold together (S17 *Which fact applies*)
    model = ([{"disabled", "pressed", "hover"}], [{"primary", "flat"}], {frozenset({"flat", "@pill"})})
    for a, b, want in (({}, {"states": ["hover"]}, True),
                       ({"states": ["hover"]}, {"not_states": ["hover"]}, False),
                       ({"states": ["hover"]}, {"states": ["pressed"]}, False),
                       ({"conditions": ["scheme:dark"]}, {"conditions": ["scheme:light"]}, False),
                       ({"conditions": ["preset:macos-tahoe"]}, {"conditions": ["contrast_high"]}, True),
                       ({"sizes": ["small"]}, {"sizes": ["large"]}, False),
                       ({"direction": "ltr"}, {"direction": "rtl"}, False),
                       ({"size_class": {"horizontal": ["compact"]}}, {"size_class": {"horizontal": ["regular"]}},
                        False),
                       ({"position": {"first": True}}, {"position": {"first": False}}, False),
                       ({"context": [{"widget": "toolbar"}]}, {"not_context": [{"widget": "toolbar"}]}, False),
                       ({"variants": ["primary"]}, {"variants": ["flat"]}, False),
                       ({"variants": ["flat"]}, {"variants": ["@pill"]}, True),
                       ({"variants": ["@pill"]}, {"variants": ["primary"]}, False),
                       ({"conditions": ["container_size:sidebar:small"]},
                        {"conditions": ["container_size:sidebar:large"]}, False)):
        if whens_overlap(a, b, model) != want:
            failures.append(f"whens {a} and {b}: got {not want}, want {want}")
    # order-aware (ADD-A124): one latest flag per set, the flag held, and one order of the flags for every set
    model = ([], [], set(), {"glow": {"focused", "hover"}, "lift": {"pressed", "focused", "hover"}})
    for a, b, want in (({"latest": {"glow": "hover"}}, {}, True),
                       ({"latest": {"glow": "hover"}}, {"latest": {"glow": "focused"}}, False),
                       ({"latest": {"glow": "hover"}}, {"latest": {"glow": "hover"}, "states": ["focused"]}, True),
                       ({"latest": {"glow": "hover"}}, {"not_states": ["hover"]}, False),
                       ({"latest": {"glow": "hover"}}, {"latest": {"lift": "focused"}}, False),
                       ({"latest": {"glow": "hover"}}, {"latest": {"lift": "pressed"}}, True),
                       ({"latest": {"glow": "hover"}}, {"latest": {"lift": "hover"}, "states": ["pressed"]}, True),
                       ({"latest": {"glow": "focused"}, "states": ["hover"]}, {"latest": {"lift": "hover"}}, False)):
        if whens_overlap(a, b, model) != want:
            failures.append(f"whens {a} and {b}: got {not want}, want {want}")
    # the gate's runs per order (S17: at most n! orders per combination, those giving every set one latest counted once)
    d = {"axes": {"states": ["hover", "focused", "pressed", "disabled"]}, "exclusive": [["disabled", "pressed"]],
         "not_drawn": [{"states": ["disabled", "hover"]}],
         "recency": {"glow": ["focused", "hover"], "lift": ["pressed", "focused", "hover"]}}
    # the 16 subsets reduce to 12 (disabled drops pressed), less the two holding disabled and hover: 10; one order each
    # but {hover, focused}, {hover, pressed} and {focused, pressed} (2 each) and {hover, focused, pressed} (4: lift's
    # latest any of the three, and with pressed glow's either): 16 runs
    if recency_orders(d) != (10, 16, 4, 3):
        failures.append(f"recency orders: got {recency_orders(d)}, want (10, 16, 4, 3)")
    if combination_orders(frozenset({"hover", "focused", "pressed"}), d["recency"]) != (4, 3):
        failures.append(f"combination orders: got {combination_orders(frozenset({'hover', 'focused', 'pressed'}), d['recency'])}")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print(f"self-test ok ({len(SELF_TEST_CASES) + 1} fixture cases)")
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
        findings, counts = check(ctx)
    except (OSError, UnicodeDecodeError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for f, ln, what in findings:
        print(f"{f}:{ln}: {what}")
    for f, ln, widget, (listed, products, total) in counts["variant_sets"]:
        print(f"# variant sets: {f}:{ln}: {widget}: {total} ({listed} listed, {products} from variant_products)",
              file=sys.stderr)
    for f, ln, widget, (combos, runs, most, n) in counts["recency_orders"]:
        print(f"# recency orders: {f}:{ln}: {widget}: {runs} runs per order of {combos} state combinations (at most "
              f"{most} orders of {n} flags in one, the bound {n}! = {math.factorial(n)})", file=sys.stderr)
    print(summary(counts, len(findings)), file=sys.stderr)
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
