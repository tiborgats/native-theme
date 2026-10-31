#!/usr/bin/env python3
"""The typed widgets' Rust declarations, generated from the widget catalogue (specification S5, S6.1, ADD-A114; the
0.8.0 plan's M1a, the M1a design's task 6). Offline.

Reads `scripts/widget_units.toml` (the units, below) and the catalogue
(`docs/todo_v0.7.1_model-completeness-spec-widgets.md`) through scripts/spec_catalogue.py's API only, and writes, for
the widgets of the enabled units, `native-theme/src/next/widgets/gen_<widget>.rs` (the `ThemeParts` derive's input
`<Widget>Theme`, one `#[part(...)]` per part; the widget's values struct `<Widget>Values`; the entry's own enums) and
`native-theme/src/next/widgets/gen_index.rs` (`WidgetKind`, `Variant`, `TypedPart`, `WidgetsPatch`,
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

    [deferred.button]               # values a unit defers, per widget, each to the named later unit (S6.1):
    menu_placement = "placement"    # left out until that unit's commit enables it

    [hand_written]                  # the units written by hand, never generated, each enabled or not
    placement = false

A deferral names a value of the widget's entry (its Rust name, or as the catalogue writes it), and a unit that is
hand-written or a unit list, and not enabled: a deferral to an enabled unit, to an unknown unit, of an unknown value
or for a widget no enabled unit holds is an input error. A unit's entry that is no typed widget of the catalogue (a
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
- **Variants**: the sub-bullets starting `typed`, their first sentence's names outside parentheses; the keyed ones
  are the presets' (S10).
- **Content variants** (spec_catalogue.content_variants, from *Entry format*'s list of pairs, not from the entry):
  the widget's own, typed and keyed (`@custom_child`) as the list writes them, in `WidgetInfo.content_variants`.
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
OUT_DIR = Path("native-theme/src/next/widgets")
INDEX = "gen_index.rs"
HAND_WRITTEN_FILES = ("mod.rs", "tests.rs")

IDENT = re.compile(r"[a-z_][a-z0-9_]*")
CAMEL = re.compile(r"[A-Z][A-Za-z0-9]*")
ENUM_DEF = re.compile(r"([A-Z][A-Za-z0-9]*) \{ ([A-Z][A-Za-z0-9]*(?:, [A-Z][A-Za-z0-9]*)*) \}")
SUFFIXES = {"Px": "_px", "Ms": "_ms", "Deg": "_deg"}
AUTO_MARK = "(`Auto`"
PLAIN = {"bool", "u32", "u16", "f32", "String"}
FOCUS_VISIBLE_WHEN = ("focus_visible_when", "Value<FocusVisibleWhen>")
# The model's own types a widget value may hold: the module that defines each, and for a block how a patch sets it
# (S6.1: "fields" field by field, "set" whole) and whether its list merges by its items' keys.
MODEL_TYPES = {
    "Px": ("crate::next::units", None, False),
    "Ms": ("crate::next::units", None, False),
    "Deg": ("crate::next::units", None, False),
    "Fraction": ("crate::next::units", None, False),
    "Key": ("crate::next::key", None, False),
    "FocusVisibleWhen": ("crate::next::blocks::focus", "set", False),
    "PresentationTransition": ("crate::next::blocks::motion", "fields", True),
    "Placement": ("crate::next::placement", "fields", False),
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
class Units:
    enabled: dict              # unit name -> [entry names]
    deferred: dict             # widget -> {value name: unit}
    hand_written: dict         # unit name -> enabled

    def widgets(self) -> list:
        return [w for entries in self.enabled.values() for w in entries]

    def is_enabled(self, unit: str) -> bool:
        return unit in self.enabled or self.hand_written.get(unit, False)


def parse_units(text: str) -> Units:
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
    enabled = {}
    for name, entries in data.items():
        if name in ("deferred", "hand_written"):
            continue
        if not isinstance(entries, list) or not all(isinstance(e, str) and e for e in entries):
            raise InputError(f"{UNITS}: unit `{name}` is not a list of catalogue entries")
        if name in hand:
            raise InputError(f"{UNITS}: unit `{name}` is both generated and hand-written")
        enabled[name] = entries
    units = Units(enabled, deferred, hand)
    seen = set()
    for w in units.widgets():
        if w in seen:
            raise InputError(f"{UNITS}: `{w}` is in two units")
        seen.add(w)
    for widget, values in deferred.items():
        if widget not in seen:
            raise InputError(f"{UNITS}: `deferred.{widget}`: no enabled unit holds `{widget}`")
        for value, unit in values.items():
            if unit not in enabled and unit not in hand:
                raise InputError(f"{UNITS}: `deferred.{widget}.{value}` names no unit, `{unit}`")
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


def split_type(ty: str):
    """(wrapper, inner) of `Value<T>`, `Vec<T>`, `Sides<T>`; (None, ty) otherwise."""
    m = re.fullmatch(r"(Value|Vec|Sides|Option)<(.+)>", ty)
    return (m.group(1), m.group(2)) if m else (None, ty)


def read_widget(entry, units: Units, shared: dict, out_shared: dict) -> Widget:
    w = entry.name
    where = f"catalogue line {entry.line}, `{w}`"
    deferred = units.deferred.get(w, {})

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

    # values and their enums
    item = entry.item("Widget values")
    bullets = sub_bullets(item) if item else []
    enums, values = [], []
    declared = list(entry.values)
    added = None
    if not any(v.name == FOCUS_VISIBLE_WHEN[0] for v in declared):
        added = spec_catalogue.Value(FOCUS_VISIBLE_WHEN[0], FOCUS_VISIBLE_WHEN[1], entry.line)
        declared.append(added)
    if item:
        for _, _, span in spec_catalogue.depth0_tokens(item.text)[0]:
            m = ENUM_DEF.fullmatch(span.strip())
            if m:
                enums.append(EnumDecl(m.group(1), m.group(2).split(", "), "", item.line))
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

    # typed variants
    variants = []
    item = entry.item("Variants")
    for _, text in sub_bullets(item) if item else []:
        if not text.startswith("typed"):
            continue
        for name in depth0_names(first_sentence(text)):
            if not IDENT.fullmatch(name):
                raise InputError(f"{where}: typed variant `{name}` is no identifier")
            if name not in variants:
                variants.append(name)
    return Widget(w, entry.line, parts, values, enums, variants, deferred)


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
    return out + ["}", ""]


def value_type(v: ValueDecl) -> tuple:
    """(Rust type, `#[parts(...)]` marks, names to import)."""
    wrapper, inner = split_type(v.ty)
    marks, imports = [], set()
    if wrapper == "Sides":
        imports |= {("crate::next::units", "Sides"), ("crate::next::units", split_type(inner)[1]),
                    ("crate::next::value", "Value")}
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
            imports.add(("crate::next::value", "Value"))
    if v.may_be_absent:
        marks.append("may_be_absent")
    if v.auto:
        marks.append("auto")
    return v.ty, marks, imports


def uses(imports) -> list:
    by_module = {}
    for module, name in imports:
        by_module.setdefault(module, set()).add(name)
    out = []
    for module in sorted(by_module):
        names = sorted(by_module[module])
        out.append(f"use {module}::{names[0]};" if len(names) == 1 else f"use {module}::{{{', '.join(names)}}};")
    return out


def widget_file(w: Widget, shared_names: set) -> str:
    W = camel(w.name)
    imports = {("crate::next::blocks::part_style", "PartStyle")}
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
    for unit in sorted(set(w.deferred.values())):
        names = [f"`{v}`" for v, u in w.deferred.items() if u == unit]
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


def index_file(widgets: list, shared: dict) -> str:
    out = [HEADER] + [x.replace("///", "//!", 1) for x in wrap_doc(
        "The index over the enabled units' typed widgets (S1, S6.1): their kinds, typed variants and parts, their "
        "patches, their slots, their readers and the registry's list.", "")] + ["", "use native_theme_derive::ThemeParts;",
                                                                            ""]
    imports = {("crate::next::blocks", "Apply"), ("crate::next::blocks", "MergeConflict"),
               ("crate::next::registry", "WidgetInfo"), ("super", "PartPatch"), ("super", "WidgetSlot"),
               ("super", "WidgetTable"), ("crate::next::toml::read", "FromToml"),
               ("crate::next::toml::read", "Reader"), ("crate::next::toml::read", "Spelling"),
               ("crate::next::toml::layer", "read_widget")}
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
    out += ["/// A typed widget's part: one variant per typed widget, each holding that",
            "/// widget's part enum (`TypedPart::Button(ButtonPart::Label)`), so a part",
            "/// paired with the wrong widget does not compile (S1).",
            "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]", "pub enum TypedPart {"]
    for w in widgets:
        W = camel(w.name)
        out += [f"    /// A part of `{w.name}`.", f"    {W}({W}Part),"]
    out += ["}", ""]
    out += ["/// The typed widgets' patches (S1): per widget, its parts' patches and its",
            "/// values' patch, each `None` where the patch leaves it as it is.",
            "#[derive(Clone, Debug, Default, PartialEq)]", "#[non_exhaustive]", "pub struct WidgetsPatch {"]
    for w in widgets:
        out += [f"    /// `{w.name}`'s patch.", f"    pub {w.name}: Option<{camel(w.name)}Patch>,"]
    out += ["}", ""]
    out += ["/// A typed widget's values' patch, one variant per typed widget (S6.1).",
            "#[derive(Clone, Debug, PartialEq)]", "pub enum WidgetValuesPatch {"]
    for w in widgets:
        W = camel(w.name)
        out += [f"    /// `{w.name}`'s.", f"    {W}({W}ValuesPatch),"]
    out += ["}", ""]
    for e in shared.values():
        out += enum_code(e)
    dead = ('#[cfg_attr(\n    not(test),\n    expect(\n        dead_code,\n        reason = "S8\'s resolution applies '
            'the bases and rules (M1a task 10)"\n    )\n)]')
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
        out.append(f"            WidgetValuesPatch::{camel(w.name)}(patch) => self.{w.name}.apply_values(patch),")
    out += ["        }", "    }", "}", ""]
    out += ["impl TypedPart {", "    /// The part `part` of `widget`, by its bare name.",
            "    pub(crate) fn from_names(widget: WidgetKind, part: &str) -> Option<TypedPart> {",
            "        match widget {"]
    for w in widgets:
        W = camel(w.name)
        out.append(f"            WidgetKind::{W} => {W}Part::from_name(part).map(TypedPart::{W}),")
    out += ["        }", "    }", "}", ""]
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
                   f".map(WidgetValuesPatch::{W}),")
    out += ["        }", "    }", "}", ""]
    out += ["impl Apply<WidgetsPatch> for WidgetSlots {",
            "    fn apply(&mut self, patch: WidgetsPatch) -> Result<(), MergeConflict> {",
            f"        let WidgetsPatch {{ {', '.join(w.name for w in widgets)} }} = patch;"]
    for w in widgets:
        out += [f"        if let Some(patch) = {w.name} {{", f"            self.{w.name}.apply_widget(patch.into_parts())?;",
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
                "        content_variants: &[" + ", ".join(f'"{v}"' for v in w.content_variants) + "],", "    },"]
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


def generate(root: Path) -> dict:
    """{file name: content} the units and the catalogue give."""
    try:
        units = parse_units((root / UNITS).read_text(encoding="utf-8"))
        text = (root / spec_catalogue.CATALOGUE).read_text(encoding="utf-8")
    except OSError as err:
        raise InputError(str(err)) from err
    entries = {e.name: e for e in spec_catalogue.entries(text) if e.kind == "typed"}
    shared_table = spec_catalogue.shared_types(text)
    pairs = spec_catalogue.content_variants(text)
    shared, widgets = {}, []
    for name in units.widgets():
        entry = entries.get(name)
        if entry is None:
            raise InputError(f"{UNITS}: `{name}` is no typed widget of the catalogue")
        widget = read_widget(entry, units, shared_table, shared)
        widget.content_variants = pairs.get(name, [])
        widgets.append(widget)
    out = {f"gen_{w.name}.rs": rustfmt(widget_file(w, set(shared)), f"gen_{w.name}.rs") for w in widgets}
    out[INDEX] = rustfmt(index_file(widgets, shared), INDEX)
    return out


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
  - `spot: Placement` — where it goes.
  - `place: Value<Placement>` — where it is placed.
  - `presentations: Vec<PresentationTransition>` — events `Appear` (a note), `@spin`.
- **Variants:**
  - typed (shared): `flat` (a note, `not_this`), `round`. Then `prose`.
  - keyed, `adwaita`: `@raised`.
- **may_be_absent:**
  - parts: `mark` (`adwaita`). - fields: `body.stroke` (all). - values: `turn` (all), `spot` (all). `side` is not.
"""

SELF_TEST_UNITS = """\
slice = ["knob"]

[deferred.knob]
spot = "placement"

[hand_written]
placement = false
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
        ("place", "place", "Value<Placement>", False, False, "Where it is placed."),
        ("presentations", "presentations", "Vec<PresentationTransition>", False, False, "Events `Appear`, `@spin`."),
        ("focus_visible_when", "focus_visible_when", "Value<FocusVisibleWhen>", False, False,
         "When focus becomes visible (S2.11)."),
    ])
    expect("enums", [(e.name, e.variants, e.doc) for e in w.enums],
           [("Turn", ["Left", "RightOrUp"], "The values of `knob`'s `turn`: the turn.")])
    expect("shared", [(e.name, e.variants, e.doc) for e in shared.values()],
           [("Side", ["Start", "End"], "A shared value type of the catalogue: a note.")])
    expect("variants", w.variants, ["flat", "round"])
    expect("words", words("RightOrUp"), "Right or up.")
    expect("drop groups", drop_groups("a (b (c)) `(d)` , e"), "a `(d)`, e")
    code = widget_file(w, set(shared))
    for needle in ['#[part(root, fields(fill, stroke), absent(stroke))]', '#[part(element, fields(text))]',
                   '#[part(may_be_absent, parent = "label", fields(glyph))]', "pub gap: Value<Px>,",
                   '#[parts(block = "fields", keyed)]', "#[parts(auto)]\n    pub reach: Value<Px>,",
                   "#[parts(may_be_absent)]\n    pub turn: Value<Turn>,",
                   '#[parts(block = "set")]\n    pub focus_visible_when: Value<FocusVisibleWhen>,',
                   "use super::gen_index::Side;", "Deferred (scripts/widget_units.toml): `spot` to `placement`.",
                   "pub enum Turn {", "    /// Right or up.\n    RightOrUp,"]:
        if needle not in code:
            failures.append(f"widget file lacks {needle!r}:\n{code}")
    index = index_file([w], shared)
    for needle in ["pub enum WidgetKind {\n    /// `knob`.\n    Knob,", "    /// `round`, a typed variant of `knob`.",
                   "    Knob(KnobPart),", "pub knob: Option<KnobPatch>,", "variants: &[Variant::Flat, Variant::Round],",
                   "pub enum Side {"]:
        if needle not in index:
            failures.append(f"index lacks {needle!r}:\n{index}")

    for label, text, needle in [
        ("deferral to an enabled unit", 'slice = ["knob"]\n[deferred.knob]\nspot = "placement"\n'
                                        '[hand_written]\nplacement = true\n', "which is enabled"),
        ("deferral to a unit list", 'slice = ["knob"]\nmore = ["x"]\n[deferred.knob]\nspot = "more"\n',
         "which is enabled"),
        ("deferral to no unit", 'slice = ["knob"]\n[deferred.knob]\nspot = "nowhere"\n', "names no unit"),
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
        found = differences(ROOT, files)
        for line in found:
            print(line)
        print(f"generate_widgets: {len(files)} files, {len(found)} differing or stale", file=sys.stderr)
        return 1 if found else 0
    write(ROOT, files)
    print(f"generate_widgets: wrote {', '.join(sorted(files))}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
