#!/usr/bin/env python3
"""The typed widgets' Rust declarations, generated from the widget catalogue (specification S5, S6.1, ADD-A114; the
0.8.0 plan's M1a, the M1a design's task 6). Offline.

Reads `scripts/widget_units.toml` (the units, below) and the catalogue
(`docs/todo_v0.7.1_model-completeness-spec-widgets.md`) through scripts/spec_catalogue.py's API only, and writes, for
the widgets of the enabled units, `native-theme/src/widgets/gen_<widget>.rs` (the `ThemeParts` derive's input
`<Widget>Theme`, one `#[part(...)]` per part; the widget's values struct `<Widget>Values`; the entry's own enums) and
`native-theme/src/widgets/gen_index.rs` (`WidgetKind`, `Variant`, `TypedPart`, `WidgetsPatch`,
`WidgetValuesPatch`, the shared value types the units use, the widgets' slots and the registry's list). Each file
goes through `rustfmt` from stdin. The other files of the widgets module (`mod.rs`, `tests.rs`) are written by hand
and never touched; a `gen_*.rs` the units do not produce is stale.

Run from the repository root:

    python3 scripts/generate_widgets.py              # writes the generated files
    python3 scripts/generate_widgets.py --check      # check_release.sh's step: writes nothing
    python3 scripts/generate_widgets.py --self-test

`--check` prints each generated file that differs from what the generator would write, or that is stale, and exits
1 when there is one. Every mode exits 2 on an input it cannot read or a catalogue construct it does not handle (it
says which), and on a wrong command line. Until the catalogue is archived the catalogue is the source; after that the
generated code is, and this script is deleted (the plan's scripts table).

The units
---------
`scripts/widget_units.toml` (also read by scripts/check_facts_trace.py):

    slice = ["button"]              # an enabled unit: the catalogue entries it holds (here typed widgets only)

    [prefixes]                      # the target prefixes a unit holds beside them, written by hand, never
    slice = ["role", "token.text"]  # generated (check_facts_trace.py and the facts gate read them)

    [staged.slice]                  # paths those prefixes hold that a unit stages to a later unit, lying in no
    "token.behavior.scroll" = "scroll_view"  # unit until then (read as the prefixes are)

    [deferred.button]               # values a unit defers, per widget, each to the named later unit (S6.1):
    menu_placement = "placement"    # left out until that unit's commit enables it

    [hand_written]                  # the units written by hand, never generated, each enabled or not
    placement = false

The depth (S6.1, ADD-A147): `[deferred.<widget>]` may name a part's field, `"<part>.<field>"`, which the part's
`fields(…)` then leaves out; `[[later]]` tables name the later units (scripts/check_facts_trace.py reads them in full),
of which this reads each unit's name, `enabled`, `conditions`, `sizes` and the variants it holds per entry. An enabled
entry's **Depth** item (spec_catalogue.depth_names) lists its variants at depth: `Variant`, `WidgetInfo.variants` and
`content_variants` hold those and the variants of the enabled later units, each other typed variant named in a
generated comment with the later unit holding it (one neither lists nor holds, or one both do, is an input error);
the index writes `SIZES_UNIT` and `FLAVOURS_UNIT`, the enabled flags of the unit with `sizes = true` and of the one
holding the `flavor:` conditions; an enum of the entry no value at depth uses is left out with its values. `--check`
also reads each Depth item against the *Today's fields* selectors naming a target in the entry (todays_names(),
depth_check()): a variant or state flag a selector names that the item does not list is a difference.

A deferral names a value of the widget's entry (its Rust name, or as the catalogue writes it), and a unit that is not
enabled; a deferral and a staging name a later unit, a hand-written unit, a unit `[unit_names]` lists (M3a's widget
units) or a catalogue entry: a deferral to an enabled unit, a deferral or staging to any other name, a deferral of an
unknown value or for a widget no enabled unit holds is an input error. A unit's entry that is no typed widget of the catalogue (a
component, a target prefix) is an input error too: the generator emits typed widgets only.

What it reads from an entry
---------------------------
- **Parts** (Entry.parts): each name a part, the first the root; its parent, its *(element)* mark and its field list
  (the field names must be `PartStyle`'s: the derive asserts it at compile time). Its doc is its description, the
  text after ` — ` up to the field list's colon, parenthetical groups dropped. A part whose text marks a field `Auto`
  is refused: per-part `Auto` marks are not generated yet.
- **Widget values** (Entry.values): each name and type; a value with no type but `focus_visible_when` is refused,
  `focus_visible_when` being `Value<FocusVisibleWhen>` on every widget (Entry format), added where the entry does not
  list it. A scalar's unit suffix (`_px`, `_ms`, `_deg`) is dropped from the Rust name (S1) and must match its type.
  Its doc is the text after ` — ` in its bullet, parenthetical groups dropped, to the first `;` or `. `. A value whose
  declaration is followed by a group starting `` (`Auto` `` is marked `auto` (Entry format), which only a `Value<Px>`
  may be. An enum the
  item defines (`IconPlacement { Leading, Trailing, Top, Bottom }`) is generated in the widget's file; a type of the
  catalogue's *Shared value types* table (spec_catalogue.shared_types) that lists only variants is generated in the
  index; a type of the model is one of MODEL_TYPES; any other type is refused.
- **may_be_absent** (absence_sections): the first sentence of each section, its names outside parentheses: `parts:`
  whole parts, `fields:` `<part>.<field>`, `values:` values (as the catalogue writes them). A name that is none of
  the entry's is refused, and so is a name listed after a section's first sentence (prose may mention one there), and
  a record value (`Placement`, `Value<Placement>`), whose `Value` members, at any
  depth, the mark allows `Absent` too (S6.3): those members' marks are not generated yet.
- **Variants** (spec_catalogue.variant_bullets): the sub-bullets starting with the word `on`, their head the text up
  to the first colon outside parentheses: `on the widget, typed` gives the widget's typed variants, `` on `<part>`,
  typed `` an *(element)* part's own, each the names outside parentheses of the first sentence after the colon; the
  keyed ones (`` keyed, `<preset>` ``) are the presets' (S10), and a sub-bullet not starting with `on` is a note. An
  enabled widget's entry with no **Variants** item, an item not in the Entry format's form
  (spec_catalogue.variants_form: neither `none` nor sub-bullets alone, or a sub-bullet whose head reads as a head of
  variants in a rejected form), a head of another form, or a part that is no *(element)* part of the entry, is
  refused, never read in part, and so, for now, is an enabled widget with a part's own typed variants (`WidgetInfo`
  lists the widget's only).
- **Mirroring** (S2.4, ADD-A146): an enum of the entry or of *Shared value types* whose variant's name holds the word
  `Start`, `End`, `Leading` or `Trailing` (alone or as a word of its `UpperCamelCase` name) gets a `Mirror` that swaps
  it with its counterpart, each such word swapped (`TopEnd` and `TopStart`), and a widget value of such an enum is
  mirrored with the widget's values. An enum lacking a counterpart is refused, in every mode, whether or not an
  enabled unit uses it: every enum a typed entry defines, in any item and at any depth (enum_definitions), and every
  type of the table are checked. `Axes<…>` is a type the
  generator does not handle yet (its `vertical` member is not mirrored, S2.4).
- **Content variants** (spec_catalogue.content_variants, from *Entry format*'s list of pairs, not from the entry):
  the widget's own and its *(element)* parts' (a `<widget>.<part>` pair), typed and keyed (`@custom_child`) as the
  list writes them, in `WidgetInfo.content_variants`, each with its part (`None`: the widget's own). A pair naming a
  part the widget lacks, or one that is not an *(element)* part, is refused.
"""
import re
import subprocess
import sys
import tempfile
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

import spec_catalogue

ROOT = Path(".")
UNITS = Path("scripts/widget_units.toml")
OUT_DIR = Path("native-theme/src/widgets")
INDEX = "gen_index.rs"
HAND_WRITTEN_FILES = ("mod.rs", "tests.rs")

IDENT = re.compile(r"[a-z_][a-z0-9_]*")
CAMEL = re.compile(r"[A-Z][A-Za-z0-9]*")
ENUM_DEF = re.compile(r"([A-Z][A-Za-z0-9]*) \{ ([A-Z][A-Za-z0-9]*(?:, [A-Z][A-Za-z0-9]*)*) \}")
SUFFIXES = {"Px": "_px", "Ms": "_ms", "Deg": "_deg"}
AUTO_MARK = "(`Auto`"
PLAIN = {"bool", "u32", "u16", "f32", "String"}
FOCUS_VISIBLE_WHEN = ("focus_visible_when", "Value<FocusVisibleWhen>")
# The model's own types a widget value may hold: the module that defines each (its path in the crate, uses()
# prefixing it), and for a block how a patch sets it (S6.1: "fields" field by field, "set" whole) and whether its list
# merges by its items' keys.
MODEL_TYPES = {
    "Px": ("units", None, False),
    "Ms": ("units", None, False),
    "Deg": ("units", None, False),
    "Fraction": ("units", None, False),
    "Key": ("key", None, False),
    "FocusVisibleWhen": ("blocks::focus", "set", False),
    "PresentationTransition": ("blocks::motion", "fields", True),
    "Placement": ("placement", "fields", False),
}
HEADER = """\
// Generated by scripts/generate_widgets.py from the widget catalogue
// (docs/todo_v0.7.1_model-completeness-spec-widgets.md) and
// scripts/widget_units.toml; do not edit by hand: change the catalogue or the
// units and run the generator.
"""


class InputError(Exception):
    pass


# ------------------------------------------------------------------------------------------------- the units

@dataclass
class Later:
    name: str
    enabled: bool
    conditions: list           # the conditions it holds model-wide
    sizes: bool                # it holds the sizes beyond the default in force and the densities
    names: dict                # entry -> {"variants": set, "states": set}, every preset's groups together


@dataclass
class Units:
    enabled: dict              # unit name -> [entry names]
    deferred: dict             # widget -> {value name or `<part>.<field>`: unit}
    hand_written: dict         # unit name -> enabled
    later: list = field(default_factory=list)   # [Later], in the file's order

    def widgets(self) -> list:
        return [w for entries in self.enabled.values() for w in entries]

    def is_enabled(self, unit: str) -> bool:
        return (unit in self.enabled or self.hand_written.get(unit, False)
                or any(u.name == unit and u.enabled for u in self.later))

    def holder(self, entry: str, kind: str, name: str):
        """The later unit holding the entry's variant or state flag (on some preset), or None."""
        return next((u for u in self.later if name in u.names.get(entry, {}).get(kind, ())), None)

    def flag_of(self, what: str) -> bool:
        """The enabled flag of the sizes unit (`what` "sizes": the unit with `sizes = true`) or of the flavours unit
        (`what` "flavours": the unit holding the `flavor:` conditions); an input error where not exactly one is."""
        found = [u for u in self.later if (u.sizes if what == "sizes"
                                           else any(c.startswith("flavor:") for c in u.conditions))]
        if len(found) != 1:
            raise InputError(f"{UNITS}: {len(found)} later units are the {what} unit; one is (S6.1, S8, S16)")
        return found[0].enabled


def parse_units(text: str, entries=frozenset()) -> Units:
    """The units `text` states; `entries` the catalogue's entry names, which a `[staged]` or `[deferred]` value may
    name beside a later, hand-written or `[unit_names]` unit (scripts/check_facts_trace.py's check_waiting)."""
    try:
        data = tomllib.loads(text)
    except tomllib.TOMLDecodeError as err:
        raise InputError(f"{UNITS}: {err}") from err
    deferred = data.get("deferred", {})
    hand = data.get("hand_written", {})
    if not isinstance(deferred, dict) or not all(
            isinstance(vs, dict) and all(isinstance(u, str) for u in vs.values()) for vs in deferred.values()):
        raise InputError(f"{UNITS}: `deferred` is not a table of widgets to tables of value names to units")
    if not isinstance(hand, dict) or not all(isinstance(v, bool) for v in hand.values()):
        raise InputError(f"{UNITS}: `hand_written` is not a table of unit names to booleans")
    for table in ("prefixes", "staged"):
        if not isinstance(data.get(table, {}), dict):
            raise InputError(f"{UNITS}: `{table}` is not a table of unit names")
    staged = data.get("staged", {})
    if not all(isinstance(ps, dict) and all(isinstance(u, str) for u in ps.values()) for ps in staged.values()):
        raise InputError(f"{UNITS}: `staged` is not a table of unit names to tables of paths to units")
    named = data.get("unit_names", {})
    if not isinstance(named, dict) or not all(
            isinstance(us, list) and all(isinstance(u, str) and u for u in us) for us in named.values()):
        raise InputError(f"{UNITS}: `unit_names` is not a table of lists of unit names")
    enabled = {}
    for name, held in data.items():
        if name in ("deferred", "hand_written", "prefixes", "staged", "kept", "at_depth", "later", "unit_names"):
            continue
        if not isinstance(held, list) or not all(isinstance(e, str) and e for e in held):
            raise InputError(f"{UNITS}: unit `{name}` is not a list of catalogue entries")
        if name in hand:
            raise InputError(f"{UNITS}: unit `{name}` is both generated and hand-written")
        enabled[name] = held
    later = []
    for t in data.get("later", []) if isinstance(data.get("later", []), list) else [None]:
        if not isinstance(t, dict) or not isinstance(t.get("name"), str) or not isinstance(t.get("enabled"), bool):
            raise InputError(f"{UNITS}: a [[later]] table has no `name` or no boolean `enabled`")
        names = {}
        for entry, groups in t.items():
            if entry in ("name", "kind", "enabled", "conditions", "directions", "sizes"):
                continue
            if not isinstance(groups, list) or not all(isinstance(g, dict) for g in groups):
                raise InputError(f"{UNITS}: later unit `{t['name']}`: `{entry}` is not a list of groups")
            held = names.setdefault(entry, {"variants": set(), "states": set()})
            for g in groups:
                for k in held:
                    held[k] |= set(g.get(k, []))
        if t["name"] in enabled or t["name"] in hand or any(u.name == t["name"] for u in later):
            raise InputError(f"{UNITS}: later unit `{t['name']}` is named twice")
        later.append(Later(t["name"], t["enabled"], list(t.get("conditions", [])), t.get("sizes") is True, names))
    units = Units(enabled, deferred, hand, later)
    seen = set()
    for w in units.widgets():
        if w in seen:
            raise InputError(f"{UNITS}: `{w}` is in two units")
        seen.add(w)
    waits = set(hand) | {u.name for u in later} | {u for us in named.values() for u in us} | set(entries)
    for unit_name, paths in staged.items():
        for path, unit in paths.items():
            if unit not in waits:
                raise InputError(f'{UNITS}: `staged.{unit_name}."{path}"` = {unit!r} names no later unit, no '
                                 f"hand-written unit, no unit `unit_names` lists and no catalogue entry")
    for widget, values in deferred.items():
        if widget not in seen:
            raise InputError(f"{UNITS}: `deferred.{widget}`: no enabled unit holds `{widget}`")
        for value, unit in values.items():
            if unit not in waits:
                raise InputError(f"{UNITS}: `deferred.{widget}.{value}` = {unit!r} names no later unit, no "
                                 f"hand-written unit, no unit `unit_names` lists and no catalogue entry")
            if units.is_enabled(unit):
                raise InputError(f"{UNITS}: `deferred.{widget}.{value}` defers to `{unit}`, which is enabled: "
                                 "its commit ends the deferral")
    return units


# --------------------------------------------------------------------------------------------- reading prose

def flat(lines) -> str:
    return " ".join(x.strip() for x in lines)


def drop_groups(text: str) -> str:
    """text without its parenthetical groups outside backticks, spaces tidied."""
    _, groups = spec_catalogue.depth0_tokens(text)
    out, last = [], 0
    for start, end in groups:
        out.append(text[last:start])
        last = end
    out.append(text[last:])
    text = re.sub(r"\s+", " ", "".join(out))
    return re.sub(r" ([,.;:])", r"\1", text).strip()


def first_sentence(text: str) -> str:
    end = spec_catalogue.depth0_find(text, lambda t, i: t[i] == "." and (i + 1 == len(t) or t[i + 1] == " "))
    return text if end < 0 else text[:end]


def listed_after_first(text: str) -> list:
    """The names of text's sentences after the first that list names and nothing else (prose that mentions a name
    is no list): a section's names belong in its first sentence."""
    rest = text[len(first_sentence(text)):].lstrip(". ")
    out = []
    while rest:
        sentence = first_sentence(rest)
        names = depth0_names(sentence)
        words = re.sub(r"`[^`]*`", "", drop_groups(sentence))
        if names and not re.sub(r"\b(and|or)\b|[\s,;:.-]", "", words):
            out += names
        rest = rest[len(sentence):].lstrip(". ")
    return out


def depth0_names(text: str) -> list:
    return [name for _, _, name in spec_catalogue.depth0_tokens(text)[0]]


def sentence_doc(text: str) -> str:
    """A doc line from catalogue prose: capitalised, ending in a full stop, brackets escaped for rustdoc."""
    text = capitalised(text.strip().rstrip(".,;:").strip())
    if not text:
        return ""
    return text.replace("[", "\\[").replace("]", "\\]") + "."


def capitalised(text: str) -> str:
    """text with its first letter capitalised where its first word is all lowercase (`iOS`, `UIKit's` kept)."""
    first = text.split(" ", 1)[0]
    return text[:1].upper() + text[1:] if first and first == first.lower() else text


def uncapitalised(text: str) -> str:
    """text with its first letter lowercased where its first word is a plain capitalised word (`UIKit's` kept)."""
    first = text.split(" ", 1)[0]
    return text[:1].lower() + text[1:] if re.fullmatch(r"[A-Z][a-z'-]*", first) else text


def described(text: str, stop: str) -> str:
    """The description after a bullet's ` — `, its groups dropped, cut at the first of `stop`'s characters."""
    dash = spec_catalogue.depth0_find(text, lambda t, i: t.startswith(" — ", i))
    if dash < 0:
        return ""
    rest = drop_groups(text[dash + 3:])
    cut = spec_catalogue.depth0_find(rest, lambda t, i: t[i] in stop and (t[i] != "." or i + 1 == len(t)
                                                                          or t[i + 1] == " "))
    return rest if cut < 0 else rest[:cut]


def words(camel: str) -> str:
    """`LongPressOrSecondaryClick` -> `Long press or secondary click.`"""
    parts = re.findall(r"[A-Z][a-z0-9]*", camel)
    text = " ".join([parts[0]] + [p.lower() for p in parts[1:]]) if parts else camel
    return text + "."


def unsuffixed(name: str) -> str:
    """`gap_px` -> `gap`: a name without its unit's suffix (S1)."""
    for suffix in SUFFIXES.values():
        if name.endswith(suffix):
            return name[: -len(suffix)]
    return name


def camel(snake: str) -> str:
    return "".join(w[:1].upper() + w[1:] for w in snake.split("_"))


def sub_bullets(item) -> list:
    """An item's sub-bullets (`  - ` lines and their continuations), each (line, flat text)."""
    out = []
    for k, raw in enumerate(item.lines[1:], 1):
        if raw.startswith("  - "):
            out.append([item.line + k, [raw[4:]]])
        elif out and raw.startswith("    "):
            out[-1][1].append(raw)
    return [(n, flat(lines)) for n, lines in out]


# --------------------------------------------------------------------------------------------- the model

@dataclass
class PartDecl:
    name: str
    parent: str | None
    element: bool
    fields: list
    doc: str
    root: bool = False
    may_be_absent: bool = False
    absent: list = field(default_factory=list)


@dataclass
class EnumDecl:
    name: str
    variants: list
    doc: str
    line: int


@dataclass
class ValueDecl:
    written: str
    rust: str
    ty: str
    doc: str
    may_be_absent: bool = False
    auto: bool = False


@dataclass
class Widget:
    name: str
    line: int
    parts: list
    values: list
    enums: list
    variants: list
    deferred: dict
    content_variants: list = field(default_factory=list)
    part_variants: dict = field(default_factory=dict)
    later_variants: list = field(default_factory=list)   # [(typed variant, the later unit holding it)]


# The model's types a widget value may hold that the query mirrors in right-to-left (S2.4): a presentation's start
# or end edge and its translation, a placement's sides and alignments; and every `Sides`.
MIRRORED = {"PresentationTransition", "Placement"}

# The words of a variant's name that name a horizontal end of the layout, each with its counterpart: the query swaps
# them in right-to-left wherever a variant's name holds one (S2.4, ADD-A146), so `TopEnd` and `TopStart` are a pair.
END_WORDS = {"Start": "End", "End": "Start", "Leading": "Trailing", "Trailing": "Leading"}


def counterpart(variant: str) -> str | None:
    """`variant` with each of its words END_WORDS names swapped (`TopEnd` -> `TopStart`); None where it holds none
    (`BothEnds`)."""
    words_of = re.findall(r"[A-Z][a-z0-9]*", variant)
    if not any(w in END_WORDS for w in words_of):
        return None
    return "".join(END_WORDS.get(w, w) for w in words_of)


def mirror_pairs(e) -> list:
    """The pairs of `e`'s variants the query swaps in right-to-left (S2.4), each once, in the enum's order. A variant
    holding a word of a pair whose counterpart the enum lacks is refused: the query returns the counterpart for it."""
    pairs, seen = [], set()
    for v in e.variants:
        other = counterpart(v)
        if other is None or v in seen:
            continue
        if other not in e.variants:
            raise InputError(f"catalogue line {e.line}: `{e.name}::{v}` names an end of the layout and `{e.name}` "
                             f"lacks its counterpart `{other}`, which the query returns for it in right-to-left (S2.4)")
        pairs.append((v, other))
        seen |= {v, other}
    return pairs


def mirrors(ty: str, aligned=frozenset()) -> bool:
    """Whether the query mirrors a value of type `ty` in right-to-left (S2.4); `aligned` names the widget's enums
    with variants naming an end of the layout."""
    wrapper, inner = split_type(ty)
    name = split_type(inner)[1]
    return wrapper == "Sides" or name in MIRRORED or name in aligned


def split_type(ty: str):
    """(wrapper, inner) of `Value<T>`, `Vec<T>`, `Sides<T>`; (None, ty) otherwise."""
    m = re.fullmatch(r"(Value|Vec|Sides|Option)<(.+)>", ty)
    return (m.group(1), m.group(2)) if m else (None, ty)


def entry_enums(entry) -> list:
    """The enums the entry's **Widget values** item defines (`IconPlacement { Leading, Trailing, Top, Bottom }`), each
    refused where a variant naming an end of the layout lacks its counterpart (S2.4)."""
    item = entry.item("Widget values")
    enums = []
    for start, _, span in spec_catalogue.depth0_tokens(item.text)[0] if item else []:
        m = ENUM_DEF.fullmatch(span.strip())
        if m:
            enums.append(EnumDecl(m.group(1), m.group(2).split(", "), "", item.line_at(start)))
            mirror_pairs(enums[-1])
    return enums


ENUM_SPAN = re.compile(r"([A-Z][A-Za-z0-9]*)\s*\{(.*)\}", re.S)
ENUM_VARIANT = re.compile(r"([A-Z][A-Za-z0-9]*)\s*(?:\(.*\)|\{.*\})?", re.S)


def top_pieces(text: str) -> list:
    """text split at its commas outside brackets of every kind, each piece stripped, empty pieces left out."""
    pieces, depth, cur = [], 0, []
    for c in text:
        depth += c in "([{<"
        depth -= c in ")]}>"
        if c == "," and depth == 0:
            pieces.append("".join(cur))
            cur = []
        else:
            cur.append(c)
    pieces.append("".join(cur))
    return [p.strip() for p in pieces if p.strip()]


def enum_definitions(entry) -> list:
    """Every enum the entry defines, in any item and at any depth of its prose: each backtick span written
    `Name { A, B(…), C { … } }` (spaces optional, a data variant's payload left out), each piece a capitalised
    variant (a record's `field: Type` pieces are none)."""
    out = []
    first = entry.body_lines[0][0] if entry.body_lines else entry.line
    text = "\n".join(line for _, line in entry.body_lines)
    for m in re.finditer(r"`([^`]+)`", text):
        d = ENUM_SPAN.fullmatch(" ".join(m.group(1).split()))
        variants = [ENUM_VARIANT.fullmatch(p) for p in top_pieces(d.group(2))] if d else []
        if variants and all(variants):
            out.append(EnumDecl(d.group(1), [v.group(1) for v in variants], "",
                                first + text.count("\n", 0, m.start())))
    return out


def read_widget(entry, units: Units, shared: dict, out_shared: dict) -> Widget:
    w = entry.name
    where = f"catalogue line {entry.line}, `{w}`"
    deferred = {k: u for k, u in units.deferred.get(w, {}).items() if "." not in k}
    deferred_fields = {k: u for k, u in units.deferred.get(w, {}).items() if "." in k}

    # parts
    parts = []
    for p in entry.parts:
        text = flat(p.lines)
        if "`Auto`" in text:
            raise InputError(f"{where}: part `{'/'.join(p.names)}` marks a field `Auto`: per-part `Auto` marks are "
                             "not generated yet")
        if not p.names:
            raise InputError(f"{where}: a part at line {p.line} has no name")
        doc = sentence_doc(described(text, ":"))
        for name in p.names:
            parts.append(PartDecl(name, p.parent, p.element, list(p.fields), doc or f"`{name}`."))
    if not parts:
        raise InputError(f"{where}: no parts")
    parts[0].root = True
    by_name = {p.name: p for p in parts}
    for p in parts:
        if p.parent is not None and p.parent not in by_name:
            raise InputError(f"{where}: part `{p.name}` nests in `{p.parent}`, no part of the entry")
    for name in deferred_fields:
        pname, _, fname = name.partition(".")
        part = by_name.get(pname)
        if part is None or fname not in part.fields:
            raise InputError(f'{UNITS}: `deferred.{w}."{name}"` is no field of a part of `{w}`')
        part.fields.remove(fname)

    # values and their enums
    item = entry.item("Widget values")
    bullets = sub_bullets(item) if item else []
    values = []
    declared = list(entry.values)
    added = None
    if not any(v.name == FOCUS_VISIBLE_WHEN[0] for v in declared):
        added = spec_catalogue.Value(FOCUS_VISIBLE_WHEN[0], FOCUS_VISIBLE_WHEN[1], entry.line)
        declared.append(added)
    enums = entry_enums(entry)
    enum_names = {e.name for e in enums}
    for v in declared:
        ty = v.type
        if ty is None:
            if v.name != FOCUS_VISIBLE_WHEN[0]:
                raise InputError(f"{where}: value `{v.name}` states no type")
            ty = FOCUS_VISIBLE_WHEN[1]
        rust = v.name
        wrapper, inner = split_type(ty)
        unit = inner if wrapper == "Value" else None
        suffix = SUFFIXES.get(unit or "")
        if suffix:
            if not rust.endswith(suffix):
                raise InputError(f"{where}: value `{v.name}: {ty}` lacks its unit's suffix `{suffix}` (S1)")
            rust = rust[: -len(suffix)]
        elif any(rust.endswith(s) for s in SUFFIXES.values()):
            raise InputError(f"{where}: value `{v.name}: {ty}` carries a unit suffix its type has not (S1)")
        if v.name in deferred or rust in deferred:
            continue
        auto = False
        if v is added:
            doc = "When focus becomes visible (S2.11)."
        else:
            bullet = next((t for n, t in reversed(bullets) if n <= v.line), "")
            doc = sentence_doc(described(bullet, ";.")) or f"`{rust}`."
            span = f"`{v.name}: {ty}`"
            at = bullet.find(span)
            auto = at >= 0 and bullet[at + len(span):].lstrip().startswith(AUTO_MARK)
            if auto and ty != "Value<Px>":
                raise InputError(f"{where}: value `{v.name}: {ty}` is marked `Auto`, which only a length may be "
                                 "(S6.3)")
        values.append(ValueDecl(v.name, rust, ty, doc, auto=auto))
    for name in deferred:
        if not any(name in (v.name, unsuffixed(v.name)) for v in declared):
            raise InputError(f"{UNITS}: `deferred.{w}.{name}` is no value of `{w}`")
    for e in enums:
        user = next((v for v in values if v.ty == f"Value<{e.name}>"), None)
        e.doc = (f"The values of `{w}`'s `{user.rust}`: {uncapitalised(user.doc)}" if user
                 else f"A type of `{w}`'s values.")

    # the types each value names
    for v in values:
        wrapper, inner = split_type(v.ty)
        if wrapper == "Sides":
            wrapper2, inner2 = split_type(inner)
            if wrapper2 != "Value" or inner2 not in SUFFIXES:
                raise InputError(f"{where}: value `{v.written}: {v.ty}`: a `Sides` of lengths only")
            continue
        if wrapper not in (None, "Value", "Vec"):
            raise InputError(f"{where}: value `{v.written}: {v.ty}`: type not handled")
        if inner in PLAIN or inner in MODEL_TYPES or inner in enum_names:
            continue
        if inner in shared:
            cell_line, cell = shared[inner]
            names = depth0_names(cell)
            if not names or not all(CAMEL.fullmatch(n) for n in names) or not cell.startswith("`"):
                raise InputError(f"catalogue line {cell_line}: shared type `{inner}` is no list of variants")
            groups = spec_catalogue.depth0_tokens(cell)[1]
            note = cell[groups[0][0] + 1:groups[0][1] - 1] if groups else ""
            out_shared[inner] = EnumDecl(inner, names, sentence_doc(
                f"a shared value type of the catalogue{': ' + note if note else ''}"), cell_line)
            mirror_pairs(out_shared[inner])
            continue
        raise InputError(f"{where}: value `{v.written}: {v.ty}`: `{inner}` is no type the generator knows")

    # may_be_absent
    item = entry.item("may_be_absent")
    if item:
        text = item.text
        for label, start, end in spec_catalogue.absence_sections(text):
            section = text[start:end].strip(" -")
            names = depth0_names(first_sentence(section))
            listed = listed_after_first(section)
            if listed:
                raise InputError(f"{where}: may_be_absent's `{label}:` lists {', '.join(f'`{n}`' for n in listed)} "
                                 "after its first sentence, which only the first sentence may")
            for name in names:
                if label == "parts":
                    part = by_name.get(name)
                    if part is None or part.root:
                        raise InputError(f"{where}: may_be_absent names `{name}`, no part but the root")
                    part.may_be_absent = True
                elif label == "fields":
                    if name in deferred_fields:
                        continue
                    pname, _, fname = name.partition(".")
                    part = by_name.get(pname)
                    if part is None or fname not in part.fields:
                        raise InputError(f"{where}: may_be_absent names `{name}`, no field of a part")
                    part.absent.append(fname)
                else:
                    value = next((v for v in values if name in (v.written, v.rust)), None)
                    if value is None:
                        if any(unsuffixed(name) == unsuffixed(d) for d in deferred):
                            continue
                        raise InputError(f"{where}: may_be_absent names `{name}`, no value of the entry")
                    wrapper = split_type(value.ty)[0]
                    if wrapper == "Vec":
                        raise InputError(f"{where}: may_be_absent names `{name}`, a list, never absent (S6.3)")
                    model = MODEL_TYPES.get(split_type(value.ty)[1])
                    if wrapper is None or (model is not None and model[1] == "fields"):
                        raise InputError(f"{where}: may_be_absent names `{name}: {value.ty}`, a record: its "
                                         "members' marks (S6.3, at any depth) are not generated yet")
                    value.may_be_absent = True

    # typed variants, the widget's own and its *(element)* parts'
    variants, part_variants = [], {}
    item = entry.item("Variants")
    if item is None:
        raise InputError(f"{where}: no **Variants** item, which every typed entry states (Entry format)")
    form = spec_catalogue.variants_form(item)
    if form:
        raise InputError(f"{where}: {form[0]}")
    for bullet in spec_catalogue.variant_bullets(item):
        if not bullet.form:
            raise InputError(f"{where}: the Variants sub-bullet at line {bullet.line} starts with `on` but its head "
                             "is not `on the widget, typed`, `` on `<part>`, typed `` or `keyed, `<preset>`` "
                             "(Entry format)")
        if bullet.keyed:
            continue
        if bullet.part is not None:
            part = by_name.get(bullet.part)
            if part is None or not part.element:
                raise InputError(f"{where}: the Variants sub-bullet at line {bullet.line} is on `{bullet.part}`, no "
                                 "*(element)* part of the entry")
        held = variants if bullet.part is None else part_variants.setdefault(bullet.part, [])
        for name in bullet.names:
            if not IDENT.fullmatch(name):
                raise InputError(f"{where}: typed variant `{name}` is no identifier")
            if name not in held:
                held.append(name)
    variants, later_variants = depth_variants(entry, units, variants, where)
    used = {split_type(v.ty)[1] for v in values}
    enums = [e for e in enums if e.name in used]
    return Widget(w, entry.line, parts, values, enums, variants, {**deferred, **deferred_fields},
                  part_variants=part_variants, later_variants=later_variants)


SELECTOR_WORDS = ("context", "cond", "size", "density", "variant", "variants", "pos", "dir", "latest", "among")


def todays_names(entries, widget: str) -> dict:
    """{"variants": {name: line}, "states": {name: line}} the selectors of the catalogue's *Today's fields* mappings
    name for a target in the entry `widget` (the Entry format's Depth item; ADD-A147): in its own item every backticked
    span holding ` @ ` or starting `@ `, in another entry's item each such span whose target starts with the entry's
    name; after the `@`, `variant <v>` (`variant a, b` continuing while a comma's next word is a name, not a
    selector's keyword) and `variants {a,b}` name variants, a `{a,b}` not after `!` or `variants` names state flags."""
    out = {"variants": {}, "states": {}}
    for e in entries:
        it = e.item("Today's fields")
        if it is None:
            continue
        for m in re.finditer(r"`([^`]*)`", it.text):
            span = m.group(1)
            if span.startswith("@ "):
                target, selector = "", span[2:]
            elif " @ " in span:
                target, selector = span.split(" @ ", 1)
            else:
                continue
            if e.name != widget and not re.match(rf"{re.escape(widget)}(\.|$)", target.strip()):
                continue
            line = it.line_at(m.start())
            for v in re.finditer(r"(?<![!\w])variants \{([^}]*)\}", selector):
                for n in v.group(1).split(","):
                    out["variants"].setdefault(n.strip(), line)
            for v in re.finditer(r"(?<![!\w])variant (@?[a-z_][a-z0-9_]*(?:, @?[a-z_][a-z0-9_]*)*)", selector):
                for n in v.group(1).split(", "):
                    if n in SELECTOR_WORDS:
                        break
                    out["variants"].setdefault(n, line)
            for g in re.finditer(r"(?<![!\w])\{([^}]*)\}", selector):
                if selector[:g.start()].rstrip().endswith("variants"):
                    continue
                for n in g.group(1).split(","):
                    out["states"].setdefault(n.strip(), line)
    return out


def depth_check(entry, entries) -> list:
    """`--check`'s reading of an entry's **Depth** item against the catalogue (ADD-A147): each variant and state flag
    a *Today's fields* selector names for a target in the entry (todays_names()) that the item does not list."""
    depth = spec_catalogue.depth_names(entry)
    if depth is None:
        return []
    named = todays_names(entries, entry.name)
    return [f"catalogue line {line}: `{entry.name}`'s *Today's fields* selector names the {kind[:-1]} `{n}`, which "
            f"its Depth item (line {depth['line']}) does not list"
            for kind in ("variants", "states") for n, line in named[kind].items() if n not in depth[kind]]


def depth_variants(entry, units: Units, typed: list, where: str) -> tuple:
    """(the entry's typed variants at depth, [(each other typed variant, the later unit holding it)]): those its
    **Depth** item lists (spec_catalogue.depth_names) and those an enabled later unit holds for it (S6.1, ADD-A147).
    Refused: an entry with no Depth item, a typed name it lists that is no typed variant of the entry, a typed variant
    neither it lists nor a later unit holds, and one it lists that a later unit holds too."""
    depth = spec_catalogue.depth_names(entry)
    if depth is None:
        raise InputError(f"{where}: no **Depth** item, which every typed entry of an enabled unit states (Entry "
                         "format, ADD-A147)")
    listed = [v for v in depth["variants"] if not v.startswith("@")]
    for v in listed:
        if v not in typed:
            raise InputError(f"catalogue line {depth['line']}, `{entry.name}`: the Depth item lists `{v}`, no typed "
                             "variant of the entry")
    kept, later = [], []
    for v in typed:
        unit = units.holder(entry.name, "variants", v)
        if v in listed and unit is not None:
            raise InputError(f"catalogue line {depth['line']}, `{entry.name}`: `{v}` is on the Depth item and in the "
                             f"later unit `{unit.name}` ({UNITS})")
        if v in listed or (unit is not None and unit.enabled):
            kept.append(v)
        elif unit is None:
            raise InputError(f"{where}: the typed variant `{v}` is neither on the Depth item nor in a later unit of "
                             f"{UNITS}")
        else:
            later.append((v, unit.name))
    return kept, later


# ----------------------------------------------------------------------------------------------- the output

def wrap_doc(text: str, indent: str) -> list:
    width = 80 - len(indent) - 4
    lines, cur = [], ""
    for word in text.split():
        if cur and len(cur) + 1 + len(word) > width:
            lines.append(cur)
            cur = word
        else:
            cur = f"{cur} {word}" if cur else word
    if cur:
        lines.append(cur)
    return [f"{indent}/// {x}" for x in lines]


def enum_code(e: EnumDecl) -> list:
    out = wrap_doc(e.doc, "")
    out += ["#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]", "#[parts(toml)]",
            f"pub enum {e.name} {{"]
    for v in e.variants:
        out += wrap_doc(words(v), "    ") + [f"    {v},"]
    out += ["}", ""]
    ends = mirror_pairs(e)
    if ends:
        out += ["/// Its start and end swapped for a right-to-left layout (S2.4).", f"impl Mirror for {e.name} {{",
                "    fn mirror(&mut self) {", "        *self = match *self {"]
        for a, b in ends:
            out += [f"            {e.name}::{a} => {e.name}::{b},", f"            {e.name}::{b} => {e.name}::{a},"]
        if 2 * len(ends) < len(e.variants):
            out.append("            other => other,")
        out += ["        };", "    }", "}", ""]
    return out


def value_type(v: ValueDecl) -> tuple:
    """(Rust type, `#[parts(...)]` marks, names to import)."""
    wrapper, inner = split_type(v.ty)
    marks, imports = [], set()
    if wrapper == "Sides":
        imports |= {("units", "Sides"), ("units", split_type(inner)[1]), ("value", "Value")}
    else:
        model = MODEL_TYPES.get(inner)
        if model:
            module, kind, keyed = model
            imports.add((module, inner))
            if kind:
                marks.append(f'block = "{kind}"')
                imports.add((module, f"Resolved{inner}"))
                if kind == "fields" and wrapper is None:
                    imports.add((module, f"{inner}Patch"))
            if keyed and wrapper == "Vec":
                marks.append("keyed")
        if wrapper == "Value":
            imports.add(("value", "Value"))
    if v.may_be_absent:
        marks.append("may_be_absent")
    if v.auto:
        marks.append("auto")
    return v.ty, marks, imports


def uses(imports) -> list:
    """The `use` lines of (module, name) pairs: a `super` module as it is, a model's module under `crate`."""
    by_module = {}
    for module, name in imports:
        if module != "super" and not module.startswith("super::"):
            module = f"crate::{module}"
        by_module.setdefault(module, set()).add(name)
    out = []
    for module in sorted(by_module):
        names = sorted(by_module[module])
        out.append(f"use {module}::{names[0]};" if len(names) == 1 else f"use {module}::{{{', '.join(names)}}};")
    return out


def widget_file(w: Widget, shared_names: set) -> str:
    W = camel(w.name)
    imports = {("blocks::part_style", "PartStyle")}
    body = []
    body += wrap_doc(f"`{w.name}`'s parts and values, as the catalogue lists them: the `ThemeParts` derive's input "
                     "(S5), from which it writes the part enum, the patch and the tables.", "")
    body += ["#[derive(ThemeParts)]", "#[parts(widget)]",
             '#[expect(\n    dead_code,\n    reason = "the derive reads the fields\' names and types; no value is built"\n)]',
             f"pub(crate) struct {W}Theme {{"]
    for p in w.parts:
        attrs = []
        if p.root:
            attrs.append("root")
        if p.element:
            attrs.append("element")
        if p.may_be_absent:
            attrs.append("may_be_absent")
        if p.parent:
            attrs.append(f'parent = "{p.parent}"')
        attrs.append(f"fields({', '.join(p.fields)})")
        if p.absent:
            attrs.append(f"absent({', '.join(p.absent)})")
        body += wrap_doc(p.doc, "    ") + [f"    #[part({', '.join(attrs)})]", f"    {p.name}: PartStyle,"]
    body += ["    values: " + f"{W}Values,", "}", ""]
    doc = f"`{w.name}`'s widget values (S5)."
    values_deferred = {v: u for v, u in w.deferred.items() if "." not in v}
    for unit in sorted(set(values_deferred.values())):
        names = [f"`{v}`" for v, u in values_deferred.items() if u == unit]
        listed = names[0] if len(names) == 1 else ", ".join(names[:-1]) + " and " + names[-1]
        doc += (f" Its {listed} {'are' if len(names) > 1 else 'is'} deferred to the `{unit}` unit, whose commit "
                "adds them (S6.1).")
    body += wrap_doc(doc, "")
    body += ["#[derive(Clone, Debug, PartialEq, ThemeParts)]", "#[parts(block)]", "#[non_exhaustive]",
             f"pub struct {W}Values {{"]
    for v in w.values:
        ty, marks, more = value_type(v)
        imports |= more
        body += wrap_doc(v.doc, "    ")
        if marks:
            body.append(f"    #[parts({', '.join(marks)})]")
        body.append(f"    pub {v.rust}: {ty},")
    body += ["}", ""]
    for e in w.enums:
        body += enum_code(e)
        if mirror_pairs(e):
            imports.add(("query::mirror", "Mirror"))
    used_shared = sorted({split_type(v.ty)[1] for v in w.values} & shared_names)
    for name in used_shared:
        imports.add(("super::gen_index", name))
    head = HEADER
    if w.deferred:
        note = "Deferred (scripts/widget_units.toml): " + ", ".join(f"`{v}` to `{u}`"
                                                                   for v, u in w.deferred.items()) + "."
        head += "//\n" + "\n".join(x.replace("///", "//", 1) for x in wrap_doc(note, "")) + "\n"
    lines = [head] + [x.replace("///", "//!", 1) for x in wrap_doc(
        f"`{w.name}` (the catalogue): its parts, its widget values and its own types.", "")] + [
             "", "use native_theme_derive::ThemeParts;", ""] + uses(imports) + [""] + body
    return "\n".join(lines) + "\n"


def index_file(widgets: list, shared: dict, flags: dict) -> str:
    out = [HEADER] + [x.replace("///", "//!", 1) for x in wrap_doc(
        "The index over the enabled units' typed widgets (S1, S6.1): their kinds, typed variants and parts, their "
        "patches, their slots, their readers and the registry's list.", "")] + ["", "use native_theme_derive::ThemeParts;",
                                                                            ""]
    imports = {("blocks", "Apply"), ("blocks", "MergeConflict"), ("query", "Element"), ("query", "Widget"),
               ("query::mirror", "Mirror"), ("resolve", "ResolvedTheme"), ("selector", "Direction"),
               ("selector", "Selector"), ("registry", "WidgetInfo"), ("super", "PartPatch"), ("super", "WidgetSlot"),
               ("super", "WidgetTable"), ("toml::read", "FromToml"), ("toml::read", "Reader"),
               ("toml::read", "Spelling"), ("toml::layer", "read_widget")}
    exports = []
    for w in widgets:
        W = camel(w.name)
        imports.add((f"super::gen_{w.name}", f"{W}Theme"))
        exports.append((f"super::gen_{w.name}", sorted([f"{W}Part", f"{W}Patch", f"{W}Values", f"{W}ValuesPatch",
                                                         f"Resolved{W}Values"] + [e.name for e in w.enums])))
    out += uses(imports)
    for module, names in exports:
        out.append(f"pub use {module}::{{{', '.join(names)}}};")
    out.append("")
    out += ["/// The typed widgets, one variant per widget of an enabled unit.",
            "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]", "pub enum WidgetKind {"]
    for w in widgets:
        out += [f"    /// `{w.name}`.", f"    {camel(w.name)},"]
    out += ["}", ""]
    holders = {}
    for w in widgets:
        for v in w.variants:
            holders.setdefault(v, []).append(w.name)
    out += wrap_doc("The catalogue's typed variants of the enabled units' widgets, one enum (S7); a widget "
                    "declares the ones it has.", "") + ["#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]",
                                                         "#[parts(toml)]", "pub enum Variant {"]
    for v, ws in holders.items():
        out += [f"    /// `{v}`, a typed variant of " + ", ".join(f"`{x}`" for x in ws) + ".", f"    {camel(v)},"]
    out += ["}", ""]
    for w in widgets:
        if w.later_variants:
            note = (f"`{w.name}`'s typed variants beyond its depth, each a member from its later unit's commit (S6.1, "
                    "ADD-A147; scripts/widget_units.toml): " + ", ".join(f"`{v}` ({u})" for v, u in w.later_variants)
                    + ".")
            out += [x.replace("///", "//", 1) for x in wrap_doc(note, "")] + [""]
    out += wrap_doc("Whether the sizes unit is enabled (S6.1, S8, S10; scripts/widget_units.toml): while it is not, "
                    "an element takes its default size in force alone, and the query answers `Later` for another "
                    "size it takes and for a declared density.", "")
    out += [f"pub(crate) const SIZES_UNIT: bool = {'true' if flags['sizes'] else 'false'};", ""]
    out += wrap_doc("Whether the flavours unit is enabled (S6.1, S16; scripts/widget_units.toml): while it is not, "
                    "`Theme::set_flavor` refuses every declared flavour but the default and `Theme::flavors` lists "
                    "the default alone.", "")
    out += [f"pub(crate) const FLAVOURS_UNIT: bool = {'true' if flags['flavours'] else 'false'};", ""]
    out += ["/// A typed widget's part: one variant per typed widget, each holding that",
            "/// widget's part enum (`TypedPart::Button(ButtonPart::Label)`), so a part",
            "/// paired with the wrong widget does not compile (S1).",
            "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]", "pub enum TypedPart {"]
    for w in widgets:
        W = camel(w.name)
        out += [f"    /// A part of `{w.name}`.", f"    {W}({W}Part),"]
    out += ["}", ""]
    out += ["/// The typed widgets' patches (S1): per widget, its parts' patches and its",
            "/// values' patch, each `None` where the patch leaves it as it is; boxed, so",
            "/// a layer's value does not grow with the catalogue.",
            "#[derive(Clone, Debug, Default, PartialEq)]", "#[non_exhaustive]", "pub struct WidgetsPatch {"]
    for w in widgets:
        out += [f"    /// `{w.name}`'s patch.", f"    pub {w.name}: Option<Box<{camel(w.name)}Patch>>,"]
    out += ["}", ""]
    out += ["/// A typed widget's values' patch, one variant per typed widget (S6.1), boxed,",
            "/// so a rule's value does not grow with a widget's values.",
            "#[derive(Clone, Debug, PartialEq)]", "pub enum WidgetValuesPatch {"]
    for w in widgets:
        W = camel(w.name)
        out += [f"    /// `{w.name}`'s.", f"    {W}(Box<{W}ValuesPatch>),"]
    out += ["}", ""]
    for e in shared.values():
        out += enum_code(e)
    dead = ('#[cfg_attr(\n    not(test),\n    allow(\n        dead_code,\n        reason = "only the tests apply '
            'the widgets\' patches through the slots (S6.1\'s merge, part by part): resolution keeps each base and rule '
            'a step of its own, for S8\'s lazy composition (M1a tasks 10 and 12); an `allow`, as Rust 1.88 (the MSRV) counts what dead code under an `expect` uses as used, and stable does not"\n    )\n)]')
    out += ["/// The typed widgets' patches applied in order (S8: the bases, then the rules",
            "/// on a part): each part's slot and each widget's values' patch merged.",
            "#[derive(Clone, Debug, Default, PartialEq)]", dead, "pub(crate) struct WidgetSlots {"]
    for w in widgets:
        W = camel(w.name)
        out += [f"    /// `{w.name}`'s.", f"    pub(crate) {w.name}: WidgetSlot<{W}Part, {W}ValuesPatch>,"]
    out += ["}", ""]
    out += [dead, "impl WidgetSlots {", "    /// Applies `patch` over `part`'s slot.",
            "    pub(crate) fn apply_part(&mut self, part: TypedPart, patch: PartPatch) -> Result<(), MergeConflict> {",
            "        match part {"]
    for w in widgets:
        out.append(f"            TypedPart::{camel(w.name)}(part) => self.{w.name}.apply_part(part, patch),")
    out += ["        }", "    }", "", "    /// Merges `patch` over its widget's values' patches.",
            "    pub(crate) fn apply_values(&mut self, patch: WidgetValuesPatch) -> Result<(), MergeConflict> {",
            "        match patch {"]
    for w in widgets:
        out.append(f"            WidgetValuesPatch::{camel(w.name)}(patch) => self.{w.name}.apply_values(*patch),")
    out += ["        }", "    }", "}", ""]
    out += ["impl TypedPart {", "    /// The part `part` of `widget`, by its bare name.",
            "    pub(crate) fn from_names(widget: WidgetKind, part: &str) -> Option<TypedPart> {",
            "        match widget {"]
    for w in widgets:
        W = camel(w.name)
        out.append(f"            WidgetKind::{W} => {W}Part::from_name(part).map(TypedPart::{W}),")
    out += ["        }", "    }", "", "    /// The widget the part belongs to.", "    pub(crate) fn widget(self) -> WidgetKind {",
            "        match self {"]
    for w in widgets:
        W = camel(w.name)
        out.append(f"            TypedPart::{W}(_) => WidgetKind::{W},")
    out += ["        }", "    }", "", "    /// The part's bare name.", "    pub(crate) fn name(self) -> &'static str {",
            "        match self {"]
    for w in widgets:
        out.append(f"            TypedPart::{camel(w.name)}(part) => part.name(),")
    out += ["        }", "    }", "}", ""]
    for w in widgets:
        W = camel(w.name)
        out += [f"impl Widget for {W}Values {{", f"    type Values = Resolved{W}Values;", "",
                f"    const KIND: WidgetKind = WidgetKind::{W};", "",
                "    fn evaluate(", "        theme: &ResolvedTheme,", "        element: &Element<'_>,",
                "        applies: &dyn Fn(&Selector) -> bool,", "        direction: Direction,",
                f"    ) -> Option<Resolved{W}Values> {{",
                f"        theme.widget_values::<{W}Values>(Self::KIND, element, applies, direction)", "    }", "}", ""]
        aligned = frozenset(e.name for e in list(w.enums) + list(shared.values()) if mirror_pairs(e))
        mirrored = [v.rust for v in w.values if mirrors(v.ty, aligned)]
        out += [f"/// `{w.name}`'s values mirrored for a right-to-left layout (S2.4): its `Sides`, presentations,",
                "/// placements and the values naming a start or an end.", f"impl Mirror for Resolved{W}Values {{"]
        if mirrored:
            out += ["    fn mirror(&mut self) {"] + [f"        self.{name}.mirror();" for name in mirrored] + ["    }"]
        else:
            out += ["    fn mirror(&mut self) {}"]
        out += ["}", ""]
    out += ["impl WidgetsPatch {",
            "    /// Reads the table of the widget `name` into its patch (S11); `false` where",
            "    /// no typed widget is named `name`.",
            "    pub(crate) fn read_widget(",
            "        &mut self,", "        name: &str,", "        item: &::toml::Value,", "        at: &str,",
            "        cx: &mut Reader,", "    ) -> bool {", "        match name {"]
    for w in widgets:
        out.append(f'            "{w.name}" => read_widget(&mut self.{w.name}, item, at, cx),')
    out += ["            _ => return false,", "        }", "        true", "    }", "}", ""]
    out += ["impl WidgetValuesPatch {", "    /// The values' patch of `widget` that `item` states (S11).",
            "    pub(crate) fn read(",
            "        widget: WidgetKind,", "        item: &::toml::Value,", "        at: &str,", "        cx: &mut Reader,",
            "    ) -> Option<WidgetValuesPatch> {", "        match widget {"]
    for w in widgets:
        W = camel(w.name)
        out.append(f"            WidgetKind::{W} => {W}ValuesPatch::read(item, Spelling::Bare, at, cx)"
                   f".map(|values| WidgetValuesPatch::{W}(Box::new(values))),")
    out += ["        }", "    }", "}", ""]
    out += ["impl Apply<WidgetsPatch> for WidgetSlots {",
            "    fn apply(&mut self, patch: WidgetsPatch) -> Result<(), MergeConflict> {",
            f"        let WidgetsPatch {{ {', '.join(w.name for w in widgets)} }} = patch;"]
    for w in widgets:
        out += [f"        if let Some(patch) = {w.name} {{", f"            self.{w.name}.apply_widget((*patch).into_parts())?;",
                "        }"]
    out += ["        Ok(())", "    }", "}", ""]
    out += ["/// The typed widgets' tables, in `WidgetKind`'s order, for the field-path",
            "/// registry (S6.1).",
            "pub(crate) const WIDGETS: &[WidgetInfo] = &["]
    for w in widgets:
        W = camel(w.name)
        out += ["    WidgetInfo {", f"        kind: WidgetKind::{W},", f'        name: "{w.name}",',
                f"        parts: <{W}Theme as WidgetTable>::PARTS,", f"        values: <{W}Theme as WidgetTable>::VALUES,",
                "        variants: &[" + ", ".join(f"Variant::{camel(v)}" for v in w.variants) + "],",
                "        content_variants: &[" + ", ".join(content_variant(p, v) for p, v in w.content_variants)
                + "],", "    },"]
    out += ["];"]
    return "\n".join(out) + "\n"


def rustfmt(code: str, name: str) -> str:
    try:
        run = subprocess.run(["rustfmt", "--edition", "2024", "--emit", "stdout"], input=code, text=True,
                             capture_output=True, check=False)
    except OSError as err:
        raise InputError(f"rustfmt: {err}") from err
    if run.returncode != 0:
        raise InputError(f"rustfmt refused the generated {name}:\n{run.stderr}")
    return run.stdout


def content_variants_of(widget, pairs: dict) -> list:
    """The content variants the catalogue's pairs give `widget`: [(part or None, variant)], its own first, then each
    *(element)* part's in the part order."""
    out = [(None, v) for v in pairs.get(widget.name, [])]
    parts = {p.name: p for p in widget.parts}
    named = [t.split(".", 1)[1] for t in pairs if t.startswith(widget.name + ".")]
    for part in named:
        if part not in parts:
            raise InputError(f"the content-variant pairs name `{widget.name}.{part}`, a part `{widget.name}` lacks")
        if not parts[part].element:
            raise InputError(f"the content-variant pairs name `{widget.name}.{part}`, not an *(element)* part")
    for p in widget.parts:
        out += [(p.name, v) for v in pairs.get(f"{widget.name}.{p.name}", [])]
    return out


def refuse_part_variants(widget) -> None:
    """An *(element)* part's own typed variants are read but not generated yet: `WidgetInfo` lists the widget's."""
    if widget.part_variants:
        listed = ", ".join(f"`{p}`" for p in widget.part_variants)
        raise InputError(f"catalogue line {widget.line}, `{widget.name}`: the typed variants on {listed} are a "
                         "part's own, which are not generated yet (`WidgetInfo` lists the widget's)")


def content_variant(part, variant: str) -> str:
    """One `WidgetInfo.content_variants` item: `(None, "<variant>")` or `(Some("<part>"), "<variant>")`."""
    return f'(None, "{variant}")' if part is None else f'(Some("{part}"), "{variant}")'


def generate(root: Path) -> dict:
    """{file name: content} the units and the catalogue give."""
    try:
        text = (root / spec_catalogue.CATALOGUE).read_text(encoding="utf-8")
        units = parse_units((root / UNITS).read_text(encoding="utf-8"),
                            {e.name for e in spec_catalogue.entries(text)})
    except OSError as err:
        raise InputError(str(err)) from err
    typed = [e for e in spec_catalogue.entries(text) if e.kind == "typed"]
    entries = {e.name: e for e in typed}
    shared_table = spec_catalogue.shared_types(text)
    pairs = spec_catalogue.content_variants(text)
    shared, widgets = {}, []
    for name in units.widgets():
        entry = entries.get(name)
        if entry is None:
            raise InputError(f"{UNITS}: `{name}` is no typed widget of the catalogue")
        widget = read_widget(entry, units, shared_table, shared)
        widget.content_variants = depth_content(entry, units, content_variants_of(widget, pairs))
        refuse_part_variants(widget)
        widgets.append(widget)
    flags = {what: units.flag_of(what) for what in ("sizes", "flavours")}
    for name, (line, cell) in shared_table.items():
        names = depth0_names(cell)
        if cell.startswith("`") and names and all(CAMEL.fullmatch(n) for n in names):
            mirror_pairs(EnumDecl(name, names, "", line))
    for entry in typed:
        for e in enum_definitions(entry):
            mirror_pairs(e)
    out = {f"gen_{w.name}.rs": rustfmt(widget_file(w, set(shared)), f"gen_{w.name}.rs") for w in widgets}
    out[INDEX] = rustfmt(index_file(widgets, shared, flags), INDEX)
    return out


def depth_content(entry, units: Units, pairs: list) -> list:
    """The content variants of `pairs` at depth (S6.1, ADD-A147): each the entry's Depth item lists or an enabled
    later unit holds; one a later unit not enabled holds left out until its unit; one neither is an input error."""
    depth = spec_catalogue.depth_names(entry) or {"variants": []}
    out = []
    for part, v in pairs:
        unit = units.holder(entry.name, "variants", v)
        if v in depth["variants"] or (unit is not None and unit.enabled):
            out.append((part, v))
        elif unit is None:
            raise InputError(f"catalogue line {entry.line}, `{entry.name}`: the content variant `{v}` is neither on "
                             f"the Depth item nor in a later unit of {UNITS}")
    return out


def depth_findings(root: Path) -> list:
    """depth_check() of each enabled unit's entry."""
    try:
        text = (root / spec_catalogue.CATALOGUE).read_text(encoding="utf-8")
    except OSError as err:
        raise InputError(str(err)) from err
    entries = [e for e in spec_catalogue.entries(text)]
    try:
        units = parse_units((root / UNITS).read_text(encoding="utf-8"), {e.name for e in entries})
    except OSError as err:
        raise InputError(str(err)) from err
    return [x for e in entries if e.name in units.widgets() for x in depth_check(e, entries)]


def differences(root: Path, files: dict) -> list:
    out = []
    folder = root / OUT_DIR
    for name, content in files.items():
        path = folder / name
        if not path.exists() or path.read_text(encoding="utf-8") != content:
            out.append(f"{OUT_DIR / name}: differs from what the catalogue states (run scripts/generate_widgets.py)")
    for path in sorted(folder.glob("gen_*.rs")):
        if path.name not in files:
            out.append(f"{OUT_DIR / path.name}: stale, no enabled unit produces it")
    return out


def write(root: Path, files: dict) -> None:
    folder = root / OUT_DIR
    for path in folder.glob("gen_*.rs"):
        if path.name not in files:
            path.unlink()
    for name, content in files.items():
        (folder / name).write_text(content, encoding="utf-8")


# ------------------------------------------------------------------------------------------------ self-test

SELF_TEST_CATALOGUE = """\
# Catalogue

## Shared value types

| Type | Variants or fields |
|---|---|
| `Side` | `Start`, `End` (a note) |
| `Record` | `{ a: Value<Px> }` |
| `Corner` | `TopStart`, `TopEnd`, `BothEnds` |

## Buttons and toggles

### knob

- **Kind:** typed widget.
- **Parts:**
  - `body` — the root (a note): `fill`, `stroke`.
  - `label` *(element)* — the text: `text`.
  - `mark`, in `label` — a mark: `glyph`.
- **Widget values** (`KnobValues`):
  - `gap_px: Value<Px>` — between mark and label (a note); more.
  - `reach_px: Value<Px>` (`Auto` where the platform sizes it by its content) — how far it reaches.
  - `side: Value<Side>` — which side.
  - `turn: Value<Turn>`, `Turn { Left, RightOrUp }` — the turn. More.
  - `anchor: Value<Anchor>`, `Anchor { Inline, TopEnd, BottomLeadingEnd, TopStart, BottomTrailingStart }` — where it lies.
  - `edge: Value<Edge>`, `Edge { Start, End }` — which edge.
  - `corner: Value<Corner>` — which corner.
  - `spot: Placement` — where it goes.
  - `place: Value<Placement>` — where it is placed.
  - `presentations: Vec<PresentationTransition>` — events `Appear` (a note), `@spin`.
- **Variants:**
  - on the widget, typed (shared): `flat` (a note, `not_this`), `round`. Then `prose`.
  - on the widget, keyed, `adwaita`: `@raised`.
  - Under `material-3` a note: `not_read`.
  - on the widget, typed: `sharp`.
- **may_be_absent:**
  - parts: `mark` (`adwaita`). - fields: `body.stroke` (all). - values: `turn` (all), `spot` (all). `side` is not.
- **Today's fields:** `color` → `body.fill @ variant round`; `pressed` → `body.fill @ {pressed}`.
- **Depth** (a note): variants `flat`, `round` (a note, `not_this`), `sharp`; states `pressed`; the scheme and
  `contrast_high`. Prose.
"""

SELF_TEST_UNITS = """\
slice = ["knob"]

[deferred.knob]
spot = "placement"

[hand_written]
placement = false

[[later]]
name = "flavours"
kind = "depth"
enabled = false
conditions = ["flavor:other"]

[[later]]
name = "sizes"
kind = "depth"
enabled = true
sizes = true
"""
# The same units with `sharp` in a later unit and the Depth item left without it (S6.1, ADD-A147).
SELF_TEST_LATER = SELF_TEST_UNITS + """
[[later]]
name = "styles"
kind = "depth"
enabled = false
knob = [{ variants = ["sharp"] }]
"""


def self_test() -> int:
    failures = []

    def expect(label, got, want):
        if got != want:
            failures.append(f"{label}: got {got!r}, want {want!r}")

    def refused(label, units, catalogue, needle):
        try:
            u = parse_units(units)
            e = spec_catalogue.entries(catalogue)[0]
            read_widget(e, u, spec_catalogue.shared_types(catalogue), {})
        except InputError as err:
            if needle not in str(err):
                failures.append(f"{label}: refused with {err}, want {needle!r}")
            return
        failures.append(f"{label}: not refused")

    def generated(units, catalogue):
        """generate() over a scratch root holding `units` and `catalogue`."""
        with tempfile.TemporaryDirectory() as tmp:
            for path, content in [(UNITS, units), (spec_catalogue.CATALOGUE, catalogue)]:
                (Path(tmp) / path).parent.mkdir(parents=True, exist_ok=True)
                (Path(tmp) / path).write_text(content, encoding="utf-8")
            return generate(Path(tmp))

    def generate_refused(label, units, catalogue, needle):
        try:
            generated(units, catalogue)
        except InputError as err:
            if needle not in str(err):
                failures.append(f"{label}: refused with {err}, want {needle!r}")
            return
        failures.append(f"{label}: not refused")

    units = parse_units(SELF_TEST_UNITS)
    expect("units", (units.enabled, units.deferred, units.hand_written),
           ({"slice": ["knob"]}, {"knob": {"spot": "placement"}}, {"placement": False}))
    entry = spec_catalogue.entries(SELF_TEST_CATALOGUE)[0]
    shared = {}
    w = read_widget(entry, units, spec_catalogue.shared_types(SELF_TEST_CATALOGUE), shared)
    expect("parts", [(p.name, p.root, p.element, p.parent, p.fields, p.may_be_absent, p.absent, p.doc)
                     for p in w.parts],
           [("body", True, False, None, ["fill", "stroke"], False, ["stroke"], "The root."),
            ("label", False, True, None, ["text"], False, [], "The text."),
            ("mark", False, False, "label", ["glyph"], True, [], "A mark.")])
    expect("values", [(v.written, v.rust, v.ty, v.may_be_absent, v.auto, v.doc) for v in w.values], [
        ("gap_px", "gap", "Value<Px>", False, False, "Between mark and label."),
        ("reach_px", "reach", "Value<Px>", False, True, "How far it reaches."),
        ("side", "side", "Value<Side>", False, False, "Which side."),
        ("turn", "turn", "Value<Turn>", True, False, "The turn."),
        ("anchor", "anchor", "Value<Anchor>", False, False, "Where it lies."),
        ("edge", "edge", "Value<Edge>", False, False, "Which edge."),
        ("corner", "corner", "Value<Corner>", False, False, "Which corner."),
        ("place", "place", "Value<Placement>", False, False, "Where it is placed."),
        ("presentations", "presentations", "Vec<PresentationTransition>", False, False, "Events `Appear`, `@spin`."),
        ("focus_visible_when", "focus_visible_when", "Value<FocusVisibleWhen>", False, False,
         "When focus becomes visible (S2.11)."),
    ])
    expect("enums", [(e.name, e.variants, e.doc) for e in w.enums],
           [("Turn", ["Left", "RightOrUp"], "The values of `knob`'s `turn`: the turn."),
            ("Anchor", ["Inline", "TopEnd", "BottomLeadingEnd", "TopStart", "BottomTrailingStart"],
             "The values of `knob`'s `anchor`: where it lies."),
            ("Edge", ["Start", "End"], "The values of `knob`'s `edge`: which edge.")])
    expect("shared", [(e.name, e.variants, e.doc) for e in shared.values()],
           [("Side", ["Start", "End"], "A shared value type of the catalogue: a note."),
            ("Corner", ["TopStart", "TopEnd", "BothEnds"], "A shared value type of the catalogue.")])
    expect("variants", (w.variants, w.part_variants), (["flat", "round", "sharp"], {}))
    # the depth (S6.1, ADD-A147): a typed variant a later unit holds is no member, a Depth item's names checked
    # against the *Today's fields* selectors, the units' flags written
    shallow = SELF_TEST_CATALOGUE.replace("`round` (a note, `not_this`), `sharp`;", "`round` (a note, `not_this`);")
    w3 = read_widget(spec_catalogue.entries(shallow)[0], parse_units(SELF_TEST_LATER),
                     spec_catalogue.shared_types(shallow), {})
    expect("depth: variants", (w3.variants, w3.later_variants), (["flat", "round"], [("sharp", "styles")]))
    expect("depth: the units' flags", (parse_units(SELF_TEST_LATER).flag_of("sizes"),
                                       parse_units(SELF_TEST_LATER).flag_of("flavours")), (True, False))
    entries = spec_catalogue.entries(SELF_TEST_CATALOGUE)
    expect("depth: today's names", todays_names(entries, "knob"),
           {"variants": {"round": entries[0].item("Today's fields").line},
            "states": {"pressed": entries[0].item("Today's fields").line}})
    expect("depth: checked", depth_check(entries[0], entries), [])
    narrow = spec_catalogue.entries(SELF_TEST_CATALOGUE.replace("states `pressed`;", "states `hover`;"))
    expect("depth: a selector's flag the Depth item lacks", len(depth_check(narrow[0], narrow)), 1)
    for label, unit_text, catalogue, needle in (
            ("a typed variant in no place", SELF_TEST_UNITS, shallow, "neither on the Depth item nor in a later unit"),
            ("a typed variant in two places", SELF_TEST_LATER, SELF_TEST_CATALOGUE, "on the Depth item and in the"),
            ("no Depth item", SELF_TEST_UNITS, SELF_TEST_CATALOGUE.split("- **Depth**")[0], "no **Depth** item"),
            ("a part field no part lists", SELF_TEST_UNITS + '[deferred.x]\n', SELF_TEST_CATALOGUE, "")):
        if not needle:
            continue
        try:
            read_widget(spec_catalogue.entries(catalogue)[0], parse_units(unit_text),
                        spec_catalogue.shared_types(catalogue), {})
            failures.append(f"{label}: not refused")
        except InputError as err:
            if needle not in str(err):
                failures.append(f"{label}: refused with {err}")
    deferred_field = SELF_TEST_UNITS.replace('spot = "placement"', 'spot = "placement"\n"body.stroke" = "styles"')
    deferred_field += '\n[[later]]\nname = "styles"\nkind = "depth"\nenabled = false\n'
    w4 = read_widget(spec_catalogue.entries(SELF_TEST_CATALOGUE)[0], parse_units(deferred_field),
                     spec_catalogue.shared_types(SELF_TEST_CATALOGUE), {})
    expect("a deferred part field", [p.fields for p in w4.parts if p.root], [["fill"]])
    expect("counterparts", [counterpart(v) for v in ["Start", "Leading", "TopEnd", "EndColumn", "TrailingInline",
                                                     "BothEnds", "Inline", "StartEnd"]],
           ["End", "Trailing", "TopStart", "StartColumn", "LeadingInline", None, None, "EndStart"])
    expect("mirror pairs", [mirror_pairs(e) for e in w.enums + list(shared.values())],
           [[], [("TopEnd", "TopStart"), ("BottomLeadingEnd", "BottomTrailingStart")], [("Start", "End")],
            [("Start", "End")], [("TopStart", "TopEnd")]])
    try:
        mirror_pairs(EnumDecl("Lone", ["Inline", "TopEnd"], "", 7))
        failures.append("an end without its counterpart: not refused")
    except InputError as err:
        if "lacks its counterpart `TopStart`" not in str(err):
            failures.append(f"an end without its counterpart: refused with {err}")
    parted = SELF_TEST_CATALOGUE.replace("  - on the widget, typed: `sharp`.",
                                         "  - on `label`, typed (its own): `bold`, `thin`.\n"
                                         "  - on `label`, keyed, `adwaita`: `@dim`.").replace(
        "`round` (a note, `not_this`), `sharp`;", "`round` (a note, `not_this`);")
    w2 = read_widget(spec_catalogue.entries(parted)[0], units, spec_catalogue.shared_types(parted), {})
    expect("a part's own variants", (w2.variants, w2.part_variants), (["flat", "round"], {"label": ["bold", "thin"]}))
    try:
        refuse_part_variants(w2)
        failures.append("a part's own variants generated: not refused")
    except InputError as err:
        if "a part's own, which are not generated yet" not in str(err):
            failures.append(f"a part's own variants generated: refused with {err}")
    expect("content variants", content_variants_of(w, {"knob": ["bare", "@own"], "knob.label": ["with_note"],
                                                        "dial": ["other"]}),
           [(None, "bare"), (None, "@own"), ("label", "with_note")])
    for label, pairs, needle in [("a part the widget lacks", {"knob.none": ["x"]}, "a part `knob` lacks"),
                                 ("a plain part", {"knob.mark": ["x"]}, "not an *(element)* part")]:
        try:
            content_variants_of(w, pairs)
            failures.append(f"{label}: not refused")
        except InputError as err:
            if needle not in str(err):
                failures.append(f"{label}: refused with {err}, want {needle!r}")
    expect("words", words("RightOrUp"), "Right or up.")
    expect("drop groups", drop_groups("a (b (c)) `(d)` , e"), "a `(d)`, e")
    code = widget_file(w, set(shared))
    for needle in ['#[part(root, fields(fill, stroke), absent(stroke))]', '#[part(element, fields(text))]',
                   '#[part(may_be_absent, parent = "label", fields(glyph))]', "pub gap: Value<Px>,",
                   '#[parts(block = "fields", keyed)]', "#[parts(auto)]\n    pub reach: Value<Px>,",
                   "#[parts(may_be_absent)]\n    pub turn: Value<Turn>,",
                   '#[parts(block = "set")]\n    pub focus_visible_when: Value<FocusVisibleWhen>,',
                   "use super::gen_index::{Corner, Side};", "Deferred (scripts/widget_units.toml): `spot` to `placement`.",
                   "pub enum Turn {", "    /// Right or up.\n    RightOrUp,",
                   "use crate::blocks::part_style::PartStyle;", "use crate::units::Px;",
                   "use crate::query::mirror::Mirror;", "impl Mirror for Anchor {",
                   "            Anchor::TopEnd => Anchor::TopStart,\n            Anchor::TopStart => Anchor::TopEnd,\n"
                   "            Anchor::BottomLeadingEnd => Anchor::BottomTrailingStart,\n"
                   "            Anchor::BottomTrailingStart => Anchor::BottomLeadingEnd,\n            other => other,",
                   "            Edge::Start => Edge::End,\n            Edge::End => Edge::Start,\n        };"]:
        if needle not in code:
            failures.append(f"widget file lacks {needle!r}:\n{code}")
    if "impl Mirror for Turn" in code:
        failures.append(f"widget file mirrors `Turn`, which names no end:\n{code}")
    index = index_file([w], shared, {"sizes": True, "flavours": False})
    for needle in ["pub(crate) const SIZES_UNIT: bool = true;", "pub(crate) const FLAVOURS_UNIT: bool = false;"]:
        if needle not in index:
            failures.append(f"index lacks {needle!r}")
    for needle in ["pub enum WidgetKind {\n    /// `knob`.\n    Knob,", "    /// `round`, a typed variant of `knob`.",
                   "    Knob(KnobPart),", "pub knob: Option<Box<KnobPatch>>,",
                   "variants: &[Variant::Flat, Variant::Round, Variant::Sharp],",
                   "pub enum Side {", "use crate::blocks::{Apply, MergeConflict};",
                   "impl Mirror for Corner {", "            Corner::TopStart => Corner::TopEnd,",
                   "        self.side.mirror();\n        self.anchor.mirror();\n        self.edge.mirror();\n"
                   "        self.corner.mirror();\n        self.place.mirror();\n        self.presentations.mirror();"]:
        if needle not in index:
            failures.append(f"index lacks {needle!r}:\n{index}")
    if "not_read" in index or "Variant::Bold" in index:
        failures.append(f"index lists a note's or a part's variant:\n{index}")

    for label, text, needle in [
        ("deferral to an enabled unit", 'slice = ["knob"]\n[deferred.knob]\nspot = "placement"\n'
                                        '[hand_written]\nplacement = true\n', "which is enabled"),
        ("deferral to a unit list", 'slice = ["knob"]\nmore = ["x"]\n[deferred.knob]\nspot = "more"\n',
         "`deferred.knob.spot` = 'more' names no later unit"),
        ("deferral to no unit", 'slice = ["knob"]\n[deferred.knob]\nspot = "nowhere"\n',
         "`deferred.knob.spot` = 'nowhere' names no later unit"),
        ("staging to no unit", 'slice = ["knob"]\n[staged.slice]\n"token.motion.durations" = "motoin"\n'
                               '[[later]]\nname = "motion"\nenabled = false\n',
         '`staged.slice."token.motion.durations"` = \'motoin\' names no later unit'),
        ("staging to a catalogue entry not given", 'slice = ["knob"]\n[staged.slice]\n"token.x" = "dial"\n',
         "= 'dial' names no later unit"),
        ("unit names no table of lists", 'slice = ["knob"]\n[unit_names]\nm3a = "fields"\n', "`unit_names` is not"),
        ("deferral of no widget", 'slice = ["knob"]\n[deferred.dial]\nspot = "x"\n[hand_written]\nx = false\n',
         "no enabled unit holds"),
        ("old deferral form", 'slice = ["knob"]\ndeferred = { knob = ["spot"] }\n', "tables of value names"),
        ("unit both ways", 'slice = ["knob"]\n[hand_written]\nslice = true\n', "both generated and hand-written"),
        ("widget twice", 'a = ["knob"]\nb = ["knob"]\n', "in two units"),
    ]:
        try:
            parse_units(text)
            failures.append(f"{label}: not refused")
        except InputError as err:
            if needle not in str(err):
                failures.append(f"{label}: refused with {err}, want {needle!r}")
    try:
        named = parse_units('slice = ["knob"]\n[staged.slice]\n"token.x" = "dial"\n"token.y" = "fields"\n'
                            '[unit_names]\nm3a = ["fields"]\n', entries={"knob", "dial"})
        expect("a staging to a catalogue entry and to a listed unit", sorted(named.enabled), ["slice"])
    except InputError as err:
        failures.append(f"a staging to a catalogue entry and to a listed unit: refused with {err}")
    refused("deferral of no value", 'slice = ["knob"]\n[deferred.knob]\nnope = "placement"\n'
                                    '[hand_written]\nplacement = false\n', SELF_TEST_CATALOGUE, "no value of `knob`")
    refused("undeferred record absent", 'slice = ["knob"]\n', SELF_TEST_CATALOGUE, "a record: its members")
    try:
        deferring = parse_units('slice = ["knob"]\n[deferred.knob]\nspot = "placement"\ngap = "placement"\n'
                                '[hand_written]\nplacement = false\n')
        catalogue = SELF_TEST_CATALOGUE.replace("`spot` (all)", "`spot` (all), `gap_px` (all)")
        kept = read_widget(spec_catalogue.entries(catalogue)[0], deferring, spec_catalogue.shared_types(catalogue), {})
        expect("deferral by its Rust name", "gap" in [v.rust for v in kept.values], False)
    except InputError as err:
        failures.append(f"deferral by its Rust name, listed suffixed: refused with {err}")
    refused("names after the first sentence", SELF_TEST_UNITS,
            SELF_TEST_CATALOGUE.replace("`side` is not.", "`side` (all), `gap_px`."), "after its first sentence")
    refused("record value absent", 'slice = ["knob"]\n[deferred.knob]\nspot = "placement"\n'
                                   '[hand_written]\nplacement = false\n',
            SELF_TEST_CATALOGUE.replace("`spot` (all)", "`place` (all)"), "a record: its members")
    refused("unknown type", SELF_TEST_UNITS, SELF_TEST_CATALOGUE.replace("Value<Side>", "Value<Nope>"),
            "`Nope` is no type")
    refused("record as shared enum", SELF_TEST_UNITS, SELF_TEST_CATALOGUE.replace("Value<Side>", "Value<Record>"),
            "is no list of variants")
    refused("suffix missing", SELF_TEST_UNITS, SELF_TEST_CATALOGUE.replace("`gap_px:", "`gap:"),
            "lacks its unit's suffix")
    refused("absent unknown part", SELF_TEST_UNITS, SELF_TEST_CATALOGUE.replace("parts: `mark`", "parts: `nope`"),
            "no part but the root")
    refused("absent root", SELF_TEST_UNITS, SELF_TEST_CATALOGUE.replace("parts: `mark`", "parts: `body`"),
            "no part but the root")
    refused("absent unknown field", SELF_TEST_UNITS, SELF_TEST_CATALOGUE.replace("`body.stroke`", "`body.text`"),
            "no field of a part")
    refused("absent list", SELF_TEST_UNITS, SELF_TEST_CATALOGUE.replace("values: `turn` (all)",
                                                                        "values: `presentations` (all)"),
            "a list, never absent")
    refused("auto part", SELF_TEST_UNITS, SELF_TEST_CATALOGUE.replace("— the text: `text`.",
                                                                      "— the text: `text` (`Auto` here)."),
            "per-part `Auto` marks")
    refused("auto not a length", SELF_TEST_UNITS,
            SELF_TEST_CATALOGUE.replace("`turn: Value<Turn>`,", "`turn: Value<Turn>` (`Auto` here),"),
            "only a length may be")
    refused("untyped value", SELF_TEST_UNITS, SELF_TEST_CATALOGUE.replace("`side: Value<Side>`", "`side` (a note)"),
            "states no type")
    refused("a Variants head of another form", SELF_TEST_UNITS,
            SELF_TEST_CATALOGUE.replace("on the widget, typed: `sharp`", "on the knob, typed: `sharp`"),
            "starts with `on` but its head")
    refused("variants on a plain part", SELF_TEST_UNITS,
            SELF_TEST_CATALOGUE.replace("on the widget, typed: `sharp`", "on `mark`, typed: `sharp`"),
            "no *(element)* part of the entry")
    refused("an end without its counterpart", SELF_TEST_UNITS,
            SELF_TEST_CATALOGUE.replace("`Edge { Start, End }`", "`Edge { Start, Middle }`"),
            "`Edge` lacks its counterpart `End`")
    for label, old, new, needle in [
        ("a Variants sub-bullet starting `typed`", "  - on the widget, typed: `sharp`.", "  - typed: `sharp`.",
         "the Variants sub-bullet at line 35 heads variants with 'typed', a form the Entry format rejects"),
        ("a Variants sub-bullet starting `On`", "  - on the widget, typed: `sharp`.",
         "  - On the widget, typed: `sharp`.", "the Variants sub-bullet at line 35 heads variants with 'On the"),
        ("a Variants sub-bullet `Typed (…)`", "  - on the widget, typed: `sharp`.", "  - Typed (shared): `sharp`.",
         "the Variants sub-bullet at line 35 heads variants with 'Typed (shared)'"),
        ("a Variants sub-bullet `the widget, typed`", "  - on the widget, typed: `sharp`.",
         "  - the widget, typed: `sharp`.", "the Variants sub-bullet at line 35 heads variants with 'the widget"),
        ("Variants inline", "- **Variants:**\n", "- **Variants:** typed `sharp`.\n",
         "the Variants item at line 31 has text after its label and sub-bullets"),
        ("no Variants item", "- **Variants:**\n", "- **Typed variants:**\n", "no **Variants** item"),
    ]:
        if old not in SELF_TEST_CATALOGUE:
            failures.append(f"fixture: {old!r} not found")
        refused(label, SELF_TEST_UNITS, SELF_TEST_CATALOGUE.replace(old, new), needle)

    try:
        expect("generate", sorted(generated(SELF_TEST_UNITS, SELF_TEST_CATALOGUE)), ["gen_index.rs", "gen_knob.rs"])
    except InputError as err:
        failures.append(f"generate: refused with {err}")
    dial = SELF_TEST_CATALOGUE + "\n### dial\n\n- **Kind:** typed widget.\n- **Widget values:**\n  - "
    for label, line, needle in [
        ("its enum", "`column: Value<Column>`, `Column { EndColumn, Middle }` — which column.",
         "catalogue line 46: `Column::EndColumn` names an end of the layout and `Column` lacks its counterpart "
         "`StartColumn`"),
        ("its enum in another item", "`column: Value<Key>` — which column.\n- **Notes:** its rows are\n  "
         "`Column { Middle,\n  EndColumn }`.", "catalogue line 48: `Column::EndColumn`"),
        ("its enum inside parentheses", "`column: Value<Column>` (its type, `Column { EndColumn, Middle }`) — which.",
         "catalogue line 46: `Column::EndColumn`"),
        ("its enum without spaces", "`column: Value<Column>`, `Column {Above, Leading}` — which column.",
         "`Column::Leading` names an end of the layout and `Column` lacks its counterpart `Trailing`"),
        ("its enum with a data variant", "`column: Value<Column>`, `Column { Custom(Key), Leading }` — which.",
         "`Column::Leading` names an end of the layout and `Column` lacks its counterpart `Trailing`"),
    ]:
        generate_refused(f"an entry no unit holds, {label} without its counterpart", SELF_TEST_UNITS,
                         dial + line + "\n", needle)
    expect("enum definitions", [(e.name, e.variants) for e in enum_definitions(spec_catalogue.entries(
        "## Fields\n\n### dial\n\n- **Notes:** `Mode { Plain, Ring { width_px: Value<Px> }, Custom(Key), }`, a "
        "record `Spot { at: Value<Px> }`, `Lone { A, … }`.\n")[0])], [("Mode", ["Plain", "Ring", "Custom"])])
    generate_refused("a shared type no unit uses, without its counterpart", SELF_TEST_UNITS,
                     SELF_TEST_CATALOGUE.replace("| `Corner` |", "| `Lone` | `Inline`, `TopEnd` |\n| `Corner` |"),
                     "catalogue line 9: `Lone::TopEnd` names an end of the layout and `Lone` lacks its counterpart "
                     "`TopStart`")
    generate_refused("an enabled widget's part's own variants", SELF_TEST_UNITS, parted,
                     "the typed variants on `label` are a part's own, which are not generated yet")

    with tempfile.TemporaryDirectory() as tmp:
        folder = Path(tmp) / OUT_DIR
        folder.mkdir(parents=True)
        for name, content in [("gen_same.rs", "a\n"), ("gen_differs.rs", "b\n"), ("gen_stale.rs", "c\n"),
                              ("mod.rs", "d\n")]:
            (folder / name).write_text(content, encoding="utf-8")
        expect("differences", differences(Path(tmp), {"gen_same.rs": "a\n", "gen_differs.rs": "B\n",
                                                       "gen_missing.rs": "e\n"}), [
            f"{OUT_DIR / 'gen_differs.rs'}: differs from what the catalogue states (run scripts/generate_widgets.py)",
            f"{OUT_DIR / 'gen_missing.rs'}: differs from what the catalogue states (run scripts/generate_widgets.py)",
            f"{OUT_DIR / 'gen_stale.rs'}: stale, no enabled unit produces it",
        ])

    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def main(argv) -> int:
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    if args not in ([], ["--check"]):
        print(__doc__, file=sys.stderr)
        return 2
    try:
        files = generate(ROOT)
    except InputError as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    if args == ["--check"]:
        try:
            found = differences(ROOT, files) + depth_findings(ROOT)
        except InputError as err:
            print(f"error: {err}", file=sys.stderr)
            return 2
        for line in found:
            print(line)
        print(f"generate_widgets: {len(files)} files, {len(found)} differing or stale", file=sys.stderr)
        return 1 if found else 0
    write(ROOT, files)
    print(f"generate_widgets: wrote {', '.join(sorted(files))}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
