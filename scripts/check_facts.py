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

It prints one line per finding, `<file>:<line>: <what>`, and a summary line with the counts (on stderr). Exit 0
when there is none, 1 when there are some, 2 on an input it cannot read (sources.toml, the core, the catalogue) or a
wrong command line. A fact file that is no valid TOML is a finding of that file.

Directories (S17 *Generations*): PLATFORMS maps each directory to the presets it covers: gnome (adwaita), kde
(kde-breeze), macos (macos-monterey, macos-tahoe), ios (ios-13, ios-26), windows (windows-11), material
(material-3). `palettes/` holds palette facts, `imported/` and `staged/` are skipped, any other directory or a
stray file is a finding (a README.md is allowed anywhere).

A `[[fact]]` (S17 *Fields*):

- `key`, `value`, `rows` and at least one `[[fact.cite]]` are required; `when` is optional; `scheme_file` and
  `accent` come both or neither, in `kde/` only (`scheme_file = "<sources.toml id> <path>"`, `accent` a colour
  literal). Any other field is a finding.
- `key` is a field path without TOML's unit suffix (a segment ending in `_px`, `_ms`, `_deg` or `_pt` is a finding):
  `<widget>.<part>.<field>[.<leaf>…]` and `<widget>.values.<name>[.<leaf>…]`, `component.<key>.<part>.<field>…`
  and `component.<key>.values.<name>…`, where the widget or component is a catalogue entry, the part one of its
  parts, the field one of that part's fields (the catalogue's list) and the leaves the core's sub-fields
  (check_spec_names.Checker.field_ok), the value one of the entry's widget values (the catalogue's name without its
  unit suffix; a value whose type is not a `Value<…>` may name leaves, unchecked); `role.<set>.<role>`,
  `role.<set>.disabled.<role>`, `role.global.<path>`, `role.@<key>` (Checker.role_ok); `token.<path>`
  (Checker.token_ok); `input.<path>`, where `input.kde_scheme.<set>.<key>`,
  `.<set>.inactive.<key>`, `.wm.<key>`, `.groups`, `.inactive_groups`, `.unset_keys` and `input.inactive.<set>.<key>`
  are checked against the core's surface sets, and any other input name only for its form: the presets declare their
  inputs (S9.1, S10), and the registry (S18) will hold them. `input.<part>…` and `input.values…` are the `input`
  widget's. The registry replaces these sources once it is generated (S18). A whole part's key, `<widget>.<part>` or
  `component.<key>.<part>` (S17, ADD-A121), takes `"absent"` (`PartPatch::Absent`) or `{ drawn_as = { widget, part,
  variants } }` (`PartPatch::DrawnAs`, S11's `drawn_as` table: `widget` a catalogue entry, `part` one of its parts,
  `variants` its variants), nothing else.
- `value` is spelled as S11's Spelling table spells a preset value, with the key's own unit in the value (S17): a
  TOML number, a TOML bool, `"auto"`, `"absent"`, `"none"`, a colour literal (`#rrggbb`, `#rrggbbaa`,
  `srgb(r, g, b[, a])`, `display_p3(r, g, b[, a])`), a length `"<n>px"`, a duration `"<n>ms"`, an angle
  `"<n>deg"`, a font size `"<n>pt"` or `"<n>px"`, a name (an enum variant in snake_case, or `@<key>`), an expression
  `"= …"` (S2.2's grammar: literals, references, calls of the core's function table, `active(<ref>)`, `srgb` and
  `display_p3` as literal forms; role, token, `surface.` and `element.` references are checked by name), a list, or a
  table (a block, or a one-key table for a variant with data) whose keys are snake_case and whose suffixed keys
  (`_px`, `_ms`, `_deg`, `_pt`) take a plain number or an expression, `"auto"`, `"absent"` or `"none"`. A number
  written as a string is a finding. Where the core's sketches type the key (PartStyle and its blocks, the widget
  value's type, Tokens, the colour roles), the value must be of that dimension (KEY_DIMS). Where they type it, or a
  leaf of it, as one of the core's enums (core_enums), a name is a unit variant's snake_case name, `@<key>` where a
  variant holds a `Key`, or a name of the enum a one-field tuple variant holds (enum_name_ok); a table is a one-key
  table naming a variant with data (S11), and the payload table of a variant with named fields states only those
  fields (`timing = { spring = { … } }` names `Timing::Spring`'s, its `damping_ratio` included, S2.7). An expression's
  `surface.` reference is a `SurfaceRole` path, `surface.<role>` or `surface.disabled.<role>` (S2.2).
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
  `{ nearest = [<widgets>] }`; `part` only with `widget`), `position` (POSITION_KEYS). A colour-role, token or input
  key's `when` has conditions only (S7: a `Colors` or `Tokens` rule may only have conditions). A `preset:` condition
  names a preset of the fact's directory.
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
`exclusive_variants`, `variant_sets` and `default_size_by_variant` optional (S10's declarations, S17; absent meaning
none); names are checked against the catalogue's entry and the core (variants, flags, sizes, densities, states,
conditions, context selectors as `when`'s, each `not_drawn` a `when`), every `exclusive` state is in `axes.states`
and in one group only, `default_size` is one of a non-empty `sizes`, `requires_variants` and `sizes_by_variant`'s
variants are among `variants`, every listed size among `sizes`, a `sizes_by_context` context a catalogue widget.
ADD-A121 (variant_declaration_problems): each `exclusive_variants` group names variants among `variants`, none twice
(a variant may be in several groups, S12); each `variant_sets` set holds two or more variants among `variants`, none
twice, at most one of each `exclusive_variants` group, and no set is listed twice (in any order); each
`default_size_by_variant` entry is `{ variant, size }`, its variant among `variants`, its size one that variant takes
(its `sizes_by_variant` entry's sizes, else `sizes`), and no two entries name one variant (S12). The generation rule
applies as to a fact.

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
FACT_OPTIONAL = {"when", "scheme_file", "accent"}
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
                      "variant_sets", "default_size_by_variant"}
MODEL_AVAILABLE_REQUIRED = {"variants", "sizes", "densities", "axes", "exclusive", "not_drawn"}
AXES_KEYS = {"states", "conditions", "contexts"}

WHEN_KEYS = {"conditions", "not_conditions", "context", "not_context", "position", "variants", "not_variants",
             "sizes", "not_sizes", "densities", "not_densities", "size_class", "direction", "states", "not_states"}
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
PART_PATCH = "PartPatch"   # the type key_info gives a whole part's key, `<widget>.<part>` (S17, ADD-A121)
DRAWN_AS_KEYS = {"widget", "part", "variants"}


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


def core_functions(text: str) -> set:
    """The function names of S2.2's **Functions** table."""
    m = re.search(r"\*\*Functions\*\*.*?\n\n(\|.*?)\n\n", text, re.S)
    if not m:
        raise InputError("core: no **Functions** table")
    names = set()
    for line in m.group(1).splitlines()[2:]:
        first = line.split("|")[1] if line.count("|") > 2 else ""
        names |= set(re.findall(r"`([a-z_][a-z0-9_]*)\(", first))
    return names


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

def key_info(ctx: Context, key: str):
    """(problem or None, dimension or None, widget name or None, kind, the core's type of the key or None) of a
    fact's key; kind is "widget", "role", "token" or "input"."""
    if not isinstance(key, str) or not key:
        return "the key is not a non-empty string", None, None, None, None
    segs = key.split(".")
    if not all(SEG.match(s) for s in segs):
        return f"key {key!r} is not a dotted path of snake_case segments", None, None, None, None
    suffixed = next((s for s in segs[1:] if s.endswith(tuple(SUFFIX_DIMS))), None)
    if suffixed:
        return (f"key {key!r}: `{suffixed}` carries TOML's unit suffix (a key names the field, the value carries "
                f"the unit, S17)"), None, None, None, None
    ck = ctx.ck

    def typed(type_str, rest, widget, kind):
        t = type_at(ctx, type_str, rest)
        return None, KEY_DIMS.get(strip_type(t)) if t else None, widget, kind, t

    if segs[0] == "role":
        rest = ".".join(segs[1:])
        if not rest or not ck.role_ok(rest):
            return f"key {key!r} names no colour role (S3)", None, None, "role", None
        if segs[1].startswith("@"):
            return None, "colour", None, "role", None
        if segs[1] == "global":
            return typed("GlobalRoles", segs[2:], None, "role")
        return typed("SurfaceRoles", segs[2:] if segs[2:3] != ["disabled"] else segs[3:], None, "role")
    if segs[0] == "token":
        rest = ".".join(segs[1:])
        if not rest or not ck.token_ok(rest):
            return f"key {key!r} names no token (S4)", None, None, "token", None
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
            while True:
                parse()
                k, v = take()
                if v == ")":
                    break
                if v != ",":
                    raise ValueError(f"expected `,` or `)` in {text}()")
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


def value_problems(ctx, value, dim, where="value", typ=None) -> list:
    """Problems of a value spelled for a key of dimension `dim`; `typ` is the key's core type (S11's text row reads
    it: a text value inside a table or a list is found by its type)."""
    kind = text_kind(typ)
    if kind:
        dim = "text"
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
    else:
        out += [(item.line, f"missing `{f}`") for f in sorted(FACT_REQUIRED - d.keys())]
        out += [(item.ln(f), f"`{f}` is not a fact's field") for f in sorted(d.keys() - FACT_REQUIRED - FACT_OPTIONAL)]
        dim, widget, kind, typ = None, None, None, None
        if "key" in d:
            problem, dim, widget, kind, typ = key_info(ctx, d["key"])
            if problem:
                out.append((item.ln("key"), problem))
        if "value" in d and typ == PART_PATCH:
            out += [(item.ln("value"), p) for p in part_patch_problems(ctx, d["value"], widget)]
        elif "value" in d:
            out += [(item.ln("value"), p) for p in value_problems(ctx, d["value"], dim, typ=typ)]
            if isinstance(d["value"], str) and text_leaves(ctx, d["value"], typ):
                out += [(c.ln("tolerance"), "a text value compares exactly: its measured citation's tolerance is 0 "
                                            "(S11)") for c in item.cites if c.data.get("kind") == "measured"
                        and "tolerance" in c.data and c.data["tolerance"] != 0]
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
        if any(c.data.get("kind") == "read" for c in item.cites):
            v = d.get("value")
            if not (isinstance(v, str) and re.fullmatch(r"=\s*input\.[a-z0-9_.@]+\s*", v)):
                out.append((item.ln("value"), "a fact with a `read` citation states a plain `= input.…` reference"))
    out += cite_list_problems(item, "fact")
    for c in item.cites:
        out += cite_problems(ctx, item, c)
    out += generation_problems(ctx, item)
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
    out += cite_list_problems(item, "model")
    for c in item.cites:
        out += cite_problems(ctx, item, c)
    out += generation_problems(ctx, item)
    return out


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
    if "variant_sets" in d:
        vs = d["variant_sets"]
        if not (isinstance(vs, list) and all(str_list(s) for s in vs)):
            out.append((item.ln("variant_sets"), "variant_sets is not a list of lists of variants"))
        else:
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
    }
    return findings, counts


def summary(counts, n):
    vacuous = "no facts yet: " if not counts["files"] else ""
    return (f"{vacuous}{counts['files']} fact files, {counts['facts']} facts, {counts['models']} state-model entries, "
            f"{counts['palette']} palette facts, {counts['cites']} citations; {counts['palettes']} bundled palettes "
            f"(new format): {n} findings")


# --------------------------------------------------------------------------------------------- self-test

SELF_TEST_CORE = '''
```rust
pub enum StateFlag { Hover, Pressed, Focused, Disabled, Checked, Shared(SharedFlag), Widget(Key) }
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
pub struct GlobalRoles { pub accent: Value<Color>, pub status: StatusRoles }
pub struct Tokens { pub text: TextTokens, pub kde: KdeTokens, pub motion: MotionTokens, pub focus: FocusIndicator,
                    pub icons: IconTokens, pub material: MaterialScheme }
pub struct IconTokens { pub theme: Value<String> }
pub struct MotionTokens { pub durations: Vec<(Key, Value<Ms>)>, pub speed_factor: Value<f32> }
pub struct PartStyle { pub fill: Value<Paint>, pub stroke: Value<Stroke>, pub corners: Value<Corners>,
                       pub shadow: Value<Shadow>, pub text: Value<TextStyle>, pub focus: Value<FocusIndicator>,
                       pub padding: Sides<Value<Px>>, pub min_width: Value<Px>, pub opacity: Value<Fraction>,
                       pub transitions: Vec<Transition>, pub glyph: Value<Glyph> }
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
pub struct FocusIndicator { pub width: Value<Px>, pub corners: X }
pub struct Corners { pub top_left: Radius }
pub struct Stroke { pub top: Edge }
pub struct Transform { pub scale: X }
pub struct Caret { pub width: X }
pub struct Behavior { pub focus_visible_when: X }
pub struct ScrollPhysics { pub friction: X }
pub struct TouchBehavior { pub slop: X }
pub struct FontFallbacks { pub families: Vec<String>, pub emoji: X }
pub struct TextRendering { pub hinting: X }
pub struct KdeTokens { pub disabled: KdeEffects, pub frame_width: Value<Px> }
pub struct KdeEffects { pub intensity_amount: Value<f32> }
pub struct MaterialScheme { pub variant: MaterialVariant, pub primary: X }
pub enum MaterialVariant { TonalSpot, FruitSalad, Cmf }
pub struct Transition { pub properties: Vec<PropertyGroup>, pub duration: Value<Ms>, pub timing: Value<Timing> }
pub enum PropertyGroup { Fill, Stroke }
pub enum Timing { CubicBezier(f32, f32, f32, f32), Linear, Spring { mass: f32, stiffness: f32, damping_ratio: f32,
                  rest_epsilon: f32 } }
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
| `mix(a, b, t)` | colour | mix |
| `kcu_mix(a, b, bias)`, `kcu_tint(a, b, amount)` | colour | KColorUtils |
| `kde_shade(c, role: KdeShadeRole, contrast)` | colour | KColorScheme::shade |
| `kirigami_lerp(a, b, balance)` | colour | Kirigami's ColorUtils.linearInterpolation |

A function is added only with its rule.
'''

SELF_TEST_CATALOGUE = '''
## Buttons and toggles

### button

- **Parts:**
  - `body` — the root: `fill`, `stroke`, `corners`, `shadow`, `padding`, `min_width`, `focus`, `transitions`.
  - `label` — the text: `text`.
  - `mark` — the mark: `glyph`.
- **Widget values:** `icon_text_gap_px: Value<Px>`, `repeat_delay_ms: Value<Ms>`.
- **Variants:** typed `flat`, `primary`; keyed `@pill`.
- **Flags:** keyed `@busy`.

### pull_to_refresh

- **Parts:**
  - `body` — the root: `padding`.
  - `indicator` — the indicator: `fill`, `shadow`.
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
    # the files
    ("a stray directory", "docs/facts/gtk/x.toml", None, "", ["not a facts directory"]),
    ("imported is skipped", "docs/facts/imported/rows.toml", None, "this is no TOML", []),
    ("staged is skipped", "docs/facts/staged/bases.toml", None, "this is no TOML", []),
    ("bad TOML", "docs/facts/gnome/buttons.toml", "[[fact]]\nkey", "[[fact]\nkey", ["not valid TOML"]),
    ("a fact table in state-models", "docs/facts/macos/state-models.toml", None, '[[fact]]\nkey = "button.body.fill"\n',
     ["unknown table `fact`"]),
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
        if ctx.functions != {"alpha", "mix", "kcu_mix", "kcu_tint", "kde_shade", "kirigami_lerp"}:
            failures.append(f"functions: got {sorted(ctx.functions)}")
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
                           ("= kirigami_lerp(role.window.background, role.view.foreground, 0.2)", [])):
            got = value_problems(ctx, expr, None)
            if len(got) != len(want) or not all(any(w in g for g in got) for w in want):
                failures.append(f"expression {expr!r}: got {got}, want {want}")
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
    print(summary(counts, len(findings)), file=sys.stderr)
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
