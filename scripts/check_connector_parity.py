#!/usr/bin/env python3
"""Check that the connectors offer the same functions, under the model's names.

The iced, gpui and egui connectors each offer free functions that read a
value of a `ResolvedTheme`. A value documented once in
docs/platform-facts.md should be found under one name in all three, the
model's own, and a function one connector offers should be in its siblings
too, under one signature. So this script lists each connector's `pub fn`s,
their parameters and the leaves each reads, and fails on an accessor named
unlike its leaf, or a function present in one connector and absent from a
sibling or present there under another signature, unless
scripts/connector_parity_exceptions.toml lists it with its reason.

Run from the repository root:

    python3 scripts/check_connector_parity.py
    python3 scripts/check_connector_parity.py --self-test

Exit 0 when every function passes or is excepted and no exception is stale,
1 on a violation or a stale exception, 2 on an input it cannot read (a model
field the type model lacks, a `Resolved*` type no model source declares, a
connector in which it finds no one-field accessor, a malformed exception
file). It prints how many functions each class holds per connector, the
names each class holds, and the public modules it does not compare.

The naming rule
---------------
A one-field accessor at the crate root -- a public free function whose result
comes from exactly one `ResolvedTheme` leaf (possibly converted to the
toolkit's type or unit, or scaled by the text-scaling factor) -- is named
after that leaf's path: drop the leading `defaults.`, join the remaining
segments with `_`, and drop a trailing unit suffix (`_px`, `_ms`, `_pt`),
since the accessor returns the toolkit's type. `defaults.info_text_color` is
`info_text_color`, `scrollbar.groove_width` is `scrollbar_groove_width`,
`tooltip.show_delay_ms` is `tooltip_show_delay`.

The sibling rule
----------------
Every function compared, of every class, is held to it. A one-field accessor
named after its leaf stands for that leaf: a sibling has it, under the same
signature, or the exception list names the sibling and the accessor with the
reason the sibling has none or another, `kind = "absent"` where the sibling
has no function of that name and `kind = "unlike"` where it has one that is
not that accessor or takes other parameters (gpui's `font_family(&Arc<str>)`
beside iced's and egui's `font_family(resolved)`). Any other function --
`outside`, `inputs`, `group`, `multi`, `undecided`, or a one-field one the
naming rule excepts -- stands for its name: each sibling has a function of
that name with the same signature, or is excepted, `absent` where it has
none, `unlike` where its signature differs.

The signature is the parameter types, lifetimes dropped (a lifetime-only
generic, `Native<'_>`, is the bare type), each type the core crate's or std's
kept by its name and every other one read as `_`: a toolkit's own type
(`Hsla`, `iced_core::Color`, `egui::Ui`) is what a sibling takes in its place
by construction, and the return type is the toolkit's for the same reason. A
type is the core crate's when the crate declares it `pub` and the function
names it with no path, or with one that starts at `native_theme`, and does
not take it as a generic parameter, declare it in its file, or import it, or
a glob of it, from another crate: iced's
`iced_core::Theme` is a toolkit's, though the core crate has a `Theme`. Where
the connectors that have the function disagree, the signature most of them
share is the reference, on a tie the first connector's in `CONNECTORS` order,
and each other one needs `unlike`.

What is scanned
---------------
The functions at the crate root: every `pub fn` at column 0 of each
connector's `src/lib.rs`, and every one a column-0 `pub use <module>::{..};`
(or `pub use <module>::<name>;`, `as` renames and `*` too) of `lib.rs`
re-exports from a module the same `lib.rs` declares (`src/<module>.rs` or
`src/<module>/mod.rs`), so egui's `accessors.rs` and `ext.rs` and a module a
later re-export names are read without a list here. A `pub use` of a local
module the script cannot follow (a nested path or group) is an input error.

And the column-0 `pub fn`s of each `pub mod` (not `pub(crate)`) of `lib.rs`
that a sibling also declares (`icons`, in all three), compared as
`module::name`: a module of one name has one job in every connector. A public
module no sibling declares (iced's `styles`, gpui's `geometry`, egui's
`convert`, ...) holds the toolkit's own style builders and conversions, which
no sibling can share; it is listed with its function count, not compared.
Methods and private functions are not read. Comments and string literals are
blanked before any match, so a doc comment naming a leaf reads nothing.

Deciding "one leaf"
-------------------
The type model is read from the core crate's sources: the `Resolved*` structs
of native-theme/src/model/resolved.rs, border.rs and font.rs, and the widget
structs of model/widgets/mod.rs, whose `#[theme(nested, resolved_type = ...)]`
fields are the nested structs of the generated `Resolved*Theme`. A path that
ends on a struct is a group, any other a leaf, and a path the model does not
have is an input error, so a renamed model field cannot leave the script
matching nothing. The type names a signature keeps are every `pub struct`,
`enum`, `trait` and `type` of native-theme/src.

A function is classified by its parameters and its body:

  outside    takes no `ResolvedTheme` (a conversion, a constructor, a
             `&SystemTheme` reader);
  inputs     takes an input besides the theme and the accessibility
             preferences, so its result is not one leaf's;
  group      reads a whole group (a border, a font, the icon sizes);
  multi      reads two leaves or more;
  one-field  reads exactly one leaf and passes the theme to nothing else;
  undecided  passes the theme on whole and reads at most one leaf, or reads
             nothing: the script cannot see what the callee reads.

A body's leaves are the `<theme>.<path>` chains on the theme parameter and on
the locals bound to one of its groups (`let c = &resolved.combo_box;`); a
chain ends at a method call or at its leaf. An undecided function at the
crate root, and a one-field one the naming rule does not fit (a widget
builder that reads one leaf, a derived flag), needs an exception entry
(`kind = "undecided"`, `kind = "name"`); nothing is silently skipped.

Exceptions
----------
scripts/connector_parity_exceptions.toml, `[[exception]]` entries with
`connector`, `name` (each a string or a list of them), `kind` and a non-empty
`reason`; an entry stands for every (connector, name) pair it lists, and a
pair has one entry. An exception is stale, and fails the check, when its
function no longer exists, or when it no longer needs one: a `name` exception
whose function is no longer a misnamed one-field accessor, an `undecided` one
whose function is now decided, an `absent` or `unlike` one whose function the
connector now has as its siblings do, or no sibling has any more, or where
the other kind fits, `unlike` for `absent` and the reverse.
"""

import argparse
import os
import re
import sys
import tomllib

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)

EXCEPTIONS = os.path.join(SCRIPT_DIR, "connector_parity_exceptions.toml")

MODEL_FILES = (
    "native-theme/src/model/resolved.rs",
    "native-theme/src/model/border.rs",
    "native-theme/src/model/font.rs",
    "native-theme/src/model/widgets/mod.rs",
)

CONNECTORS = {
    "iced": "connectors/native-theme-iced/src/lib.rs",
    "gpui": "connectors/native-theme-gpui/src/lib.rs",
    "egui": "connectors/native-theme-egui/src/lib.rs",
}

CORE_SOURCES = "native-theme/src"

ROOT_TYPE = "ResolvedTheme"
UNIT_SUFFIXES = ("_px", "_ms", "_pt")
KINDS = ("name", "absent", "unlike", "undecided")
CLASSES = ("one-field", "group", "multi", "inputs", "outside", "undecided")


class Failure(Exception):
    """An input the script cannot read."""


# ---------------------------------------------------------------------------
# Rust source handling
# ---------------------------------------------------------------------------


def blank(source):
    """`source` with comments, string and char literals replaced by spaces.

    Newlines are kept, so offsets and columns still match the file.
    """
    out = []
    i, n = 0, len(source)

    def spaces(text):
        return "".join("\n" if c == "\n" else " " for c in text)

    while i < n:
        c = source[i]
        if source.startswith("//", i):
            j = source.find("\n", i)
            j = n if j < 0 else j
            out.append(spaces(source[i:j]))
            i = j
        elif source.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if source.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif source.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            out.append(spaces(source[i:j]))
            i = j
        elif c == "r" and re.match(r'r#*"', source[i:]) and (
            i == 0 or not (source[i - 1].isalnum() or source[i - 1] == "_")
        ):
            hashes = re.match(r"r(#*)\"", source[i:]).group(1)
            close = '"' + hashes
            j = source.find(close, i + 2 + len(hashes))
            j = n if j < 0 else j + len(close)
            out.append(spaces(source[i:j]))
            i = j
        elif c == '"':
            j = i + 1
            while j < n and source[j] != '"':
                j += 2 if source[j] == "\\" else 1
            j = min(j + 1, n)
            out.append(spaces(source[i:j]))
            i = j
        elif c == "'":
            m = re.match(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^\\'\n])'", source[i:])
            if m:
                out.append(spaces(m.group(0)))
                i += len(m.group(0))
            else:
                out.append(c)
                i += 1
        else:
            out.append(c)
            i += 1
    return "".join(out)


def balanced(text, start, open_, close):
    """Index one past the bracket closing the one at `start`, or -1."""
    depth = 0
    for j in range(start, len(text)):
        if text[j] == open_:
            depth += 1
        elif text[j] == close:
            depth -= 1
            if depth == 0:
                return j + 1
    return -1


def split_top(text, sep=","):
    """`text` split on `sep` outside (), [], {} and <>."""
    parts, depth, cur = [], 0, []
    prev = ""
    for c in text:
        if c in "([{<":
            depth += 1
        elif c in ")]}" or (c == ">" and prev != "-"):
            depth -= 1
        if c == sep and depth == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(c)
        prev = c
    parts.append("".join(cur))
    return [p.strip() for p in parts if p.strip()]


# ---------------------------------------------------------------------------
# The type model
# ---------------------------------------------------------------------------

STRUCT_RE = re.compile(r"\bpub struct (\w+)\s*\{")
FIELD_RE = re.compile(r"^\s*pub (\w+)\s*:\s*([^,]+),", re.M)
NESTED_RE = re.compile(r'#\[theme\([^\]]*\bnested\b[^\]]*resolved_type\s*=\s*"(\w+)"')


def base_type(text):
    """The struct name a field type names: paths, `&`, `Option<>` dropped."""
    text = text.strip()
    m = re.fullmatch(r"Option\s*<(.+)>", text)
    if m:
        text = m.group(1).strip()
    return text.lstrip("&").split("::")[-1].strip()


def parse_model(sources):
    """{struct: {field: type}} from the model's sources.

    `Resolved*` structs are read as written. A widget struct `XTheme` with a
    `#[derive(... ThemeWidget ...)]` stands for its generated `ResolvedXTheme`:
    a field marked nested takes its `resolved_type`, any other is a leaf.
    """
    model = {}
    for source in sources:
        text = blank(source)
        for m in STRUCT_RE.finditer(text):
            name = m.group(1)
            end = balanced(text, m.end() - 1, "{", "}")
            if end < 0:
                raise Failure(f"struct {name}: no closing brace")
            body_raw = source[m.end() : end - 1]
            body = text[m.end() : end - 1]
            header = text[max(0, text.rfind("\n\n", 0, m.start())) : m.start()]
            if name.startswith("Resolved"):
                model[name] = {
                    f.group(1): base_type(f.group(2)) for f in FIELD_RE.finditer(body)
                }
            elif "ThemeWidget" in header and name.endswith("Theme"):
                fields, previous = {}, 0
                for f in FIELD_RE.finditer(body):
                    # the attributes between the previous field and this one
                    nested = NESTED_RE.search(body_raw[previous : f.start()])
                    fields[f.group(1)] = nested.group(1) if nested else "leaf"
                    previous = f.end()
                model["Resolved" + name] = fields
    if ROOT_TYPE not in model:
        raise Failure(f"the model sources declare no {ROOT_TYPE}")
    # a `Resolved*` type the sources do not declare would read as a leaf
    for struct, fields in model.items():
        for field, type_ in fields.items():
            if type_.startswith("Resolved") and type_ not in model:
                raise Failure(f"{struct}.{field}: type {type_} is not in the model sources")
    return model


def resolve_path(model, segments, type_=ROOT_TYPE):
    """(path, is_leaf) for the field chain `segments` names from `type_`.

    The chain ends at its first leaf; a segment the current struct lacks
    raises `KeyError`.
    """
    path = []
    for seg in segments:
        fields = model.get(type_)
        if fields is None:
            break
        if seg not in fields:
            raise KeyError(seg)
        path.append(seg)
        type_ = fields[seg]
    return path, type_ not in model


def leaf_name(path):
    """The accessor name the naming rule gives the leaf at `path`."""
    segs = list(path)
    if segs and segs[0] == "defaults":
        segs = segs[1:]
    name = "_".join(segs)
    for suffix in UNIT_SUFFIXES:
        if name.endswith(suffix):
            return name[: -len(suffix)]
    return name


# ---------------------------------------------------------------------------
# Connector functions
# ---------------------------------------------------------------------------

FN_RE = re.compile(r"^pub (?:const )?fn (\w+)", re.M)


class Fn:
    def __init__(self, name, line, cls, leaves, groups, detail="", ptypes=(), module=None,
                 foreign=frozenset(), core=frozenset()):
        self.name = name
        self.line = line
        self.cls = cls
        self.leaves = leaves
        self.groups = groups
        self.detail = detail
        self.ptypes = tuple(ptypes)
        self.module = module
        # names that are not the core crate's where the function is written:
        # its generic parameters, what its file imports from elsewhere or
        # declares itself (`GLOB` for a glob import of another crate); and
        # the names the file imports from a core path
        self.foreign = frozenset(foreign)
        self.core = frozenset(core)

    @property
    def leaf(self):
        return next(iter(self.leaves)) if len(self.leaves) == 1 else None

    @property
    def key(self):
        """The path the function is reached by: `name` at the crate root,
        `module::name` in a public module."""
        return f"{self.module}::{self.name}" if self.module else self.name


# Types a parameter keeps in a signature beside the core crate's own: the
# primitives and the std types the connectors' signatures name. Any other
# type is a toolkit's (`Hsla`, `iced_core::Color`, `egui::Ui`) and reads as
# `_`, since a sibling takes its own toolkit's type there by construction.
STD_TYPES = frozenset("""
    bool char str String f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128
    usize Option Result Vec Box Arc Rc Cow Duration Instant NonZeroU32 Path
    PathBuf Fn FnMut FnOnce Into AsRef Sized
""".split())

CORE_TYPE_RE = re.compile(r"^\s*pub (?:struct|enum|trait|type) (\w+)", re.M)


def core_types(sources):
    """The type names the core crate's sources declare `pub`."""
    return {m.group(1) for source in sources for m in CORE_TYPE_RE.finditer(blank(source))}


TYPE_RE = re.compile(r"\b((?:[A-Za-z_]\w*\s*::\s*)*)([A-Za-z_]\w*)\b")
STD_ROOTS = ("std", "core", "alloc")


def signature(f, shared):
    """`f`'s parameter types, each with its lifetimes dropped and every type
    that is neither the core crate's (`shared`) nor std's (`STD_TYPES`) read
    as `_`: a path that does not start at `native_theme` (or std) is a
    toolkit's (`iced_core::Theme`), and so is a bare name the function takes
    as a generic parameter, or its file declares, imports from another crate
    or may take from another crate's glob, though the core crate declares
    one of that name. The evidence is lexical, one file's: a core type
    written through a module-relative or aliased path (`theme::Theme` after
    `use native_theme::theme;`, `nt::Theme` after `use native_theme as nt;`)
    reads as `_` too. That errs loud, never silent: it can only make two
    signatures that agree look different, a false `unlike` the run reports,
    and no connector writes a core type that way today."""

    def one(m):
        prefix, name = m.group(1), m.group(2)
        if name in ("mut", "dyn", "impl"):
            return m.group(0)
        if prefix:
            root = prefix.split("::")[0].strip()
            if root == "native_theme" and name in shared:
                return name
            return name if root in STD_ROOTS and name in STD_TYPES else "_"
        if name in f.foreign or (GLOB in f.foreign and name not in f.core):
            return "_"
        return name if name in shared or name in STD_TYPES else "_"

    parts = []
    for text in f.ptypes:
        text = re.sub(r"\s*\+\s*'\w+|'\w+\s*,\s*|'\w+\s*", "", text)
        text = re.sub(r"<\s*>", "", text)
        text = TYPE_RE.sub(one, text)
        text = re.sub(r"\s+", " ", text).strip()
        text = re.sub(r"\s*([&<>,()\[\]])\s*", r"\1", text).replace(",", ", ")
        parts.append(text)
    return "(" + ", ".join(parts) + ")"


IMPORT_RE = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);", re.M)


GLOB = "*"
LOCAL_TYPE_RE = re.compile(
    r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|union|trait|type)\s+(\w+)", re.M
)


def scope_names(text):
    """(foreign, core) of a (blanked) source file: the names it brings into
    scope from another crate's path -- every `use` whose root is not
    `native_theme`, `crate`, `self` or `super`, with `GLOB` for a glob of one,
    whose names the script cannot list -- and the types it declares itself,
    which shadow a core one of the same name; and the names it imports
    explicitly from a core path, which a glob does not shadow."""
    foreign, core = set(), set()
    for m in IMPORT_RE.finditer(text):
        tree = m.group(1)
        root = tree.split("::")[0].strip()
        names = {item.group(2) or item.group(1) for item in re.finditer(
            r"(\w+)(?:\s+as\s+(\w+))?\s*(?=[,}]|$)", tree)}
        if root in ("native_theme", "crate", "self", "super"):
            core |= names
            continue
        foreign |= names
        if re.search(r"(?:::|[{,])\s*\*", tree):
            foreign.add(GLOB)
    foreign |= set(LOCAL_TYPE_RE.findall(text))
    return foreign, core


def generic_names(text):
    """The type parameters of a `<...>` generic list (lifetimes left out)."""
    return {p.split(":")[0].strip() for p in split_top(text) if not p.startswith("'")}


def is_theme_type(text):
    return bool(re.fullmatch(r"&\s*(?:'\w+\s+)?(?:[\w:]+::)?" + ROOT_TYPE, text.strip()))


def is_prefs_type(text):
    return bool(re.fullmatch(r"&\s*(?:'\w+\s+)?(?:[\w:]+::)?AccessibilityPreferences", text.strip()))


def chain_at(text, pos):
    """The `.a.b.c` segments starting at `pos` and whether each is a call."""
    segs = []
    m = re.compile(r"\s*\.\s*([A-Za-z_]\w*)")
    while True:
        s = m.match(text, pos)
        if not s:
            break
        after = re.match(r"\s*(::\s*<[^>]*>\s*)?\(", text[s.end() :])
        segs.append((s.group(1), bool(after)))
        pos = s.end()
    return segs, pos


def classify_body(model, body, root, label):
    """(leaves, groups, escapes) the body reads through `root`.

    A local bound to a group of the theme (`let c = &resolved.combo_box;`)
    reads through its group's path; its own binding is not a read.
    """

    def resolve(name, segs):
        try:
            return resolve_path(model, segs)
        except KeyError as error:
            raise Failure(f"{label}: `{name}.{'.'.join(segs)}`: "
                          f"no field `{error.args[0]}` in the model")

    roots, bindings = {root: []}, set()
    for m in re.finditer(r"\blet\s+(\w+)\s*=\s*&?\s*(\w+)\b", body):
        alias, src = m.group(1), m.group(2)
        if src not in roots:
            continue
        segs, end = chain_at(body, m.end())
        if not segs or any(call for _, call in segs) or not re.match(r"\s*;", body[end:]):
            continue
        path, is_leaf = resolve(src, roots[src] + [s for s, _ in segs])
        if not is_leaf:
            roots[alias] = path
            bindings.add(m.start(2))

    leaves, groups, escapes = set(), set(), False
    for name, prefix in roots.items():
        for m in re.finditer(r"(?<![\w.])" + re.escape(name) + r"\b", body):
            if m.start() in bindings or re.search(r"\blet\s+(?:mut\s+)?$", body[: m.start()]):
                continue
            segs, _ = chain_at(body, m.end())
            fields = []
            for seg, call in segs:
                if call:
                    break
                fields.append(seg)
            if not fields:
                # passed on whole, or a method called on it
                if name == root:
                    escapes = True
                else:
                    groups.add(".".join(prefix))
                continue
            path, is_leaf = resolve(name, prefix + fields)
            (leaves if is_leaf else groups).add(".".join(path))
    return leaves, groups, escapes


def scan(model, source, label, module=None):
    """The column-0 `pub fn`s of one source file, classified; `module` names
    the public module they are reached through, None at the crate root."""
    text = blank(source)
    imported, core = scope_names(text)
    fns = []
    for m in FN_RE.finditer(text):
        name = m.group(1)
        line = text.count("\n", 0, m.start()) + 1
        pos = m.end()
        generics = set()
        if text[pos : pos + 1] == "<":
            end = balanced(text, pos, "<", ">")
            generics = generic_names(text[pos + 1 : end - 1]) if end > 0 else set()
            pos = end
        open_ = text.find("(", pos)
        close = balanced(text, open_, "(", ")")
        if open_ < 0 or close < 0:
            raise Failure(f"{label}:{line}: `{name}` has no parameter list")
        body_open = text.find("{", close)
        body_close = balanced(text, body_open, "{", "}")
        if body_open < 0 or body_close < 0:
            raise Failure(f"{label}:{line}: `{name}` has no body")
        params = []
        for p in split_top(text[open_ + 1 : close - 1]):
            pname, _, ptype = p.partition(":")
            params.append((pname.strip().removeprefix("mut ").strip(), ptype.strip()))
        theme = [p for p, t in params if is_theme_type(t)]
        others = [p for p, t in params if not is_theme_type(t) and not is_prefs_type(t)]
        ptypes = [t for _, t in params]
        where = f"{label}:{line} `{name}`"
        if not theme:
            fns.append(Fn(name, line, "outside", set(), set(), ptypes=ptypes, module=module,
                          foreign=imported | generics, core=core))
            continue
        if len(theme) > 1 or others:
            fns.append(Fn(name, line, "inputs", set(), set(), ", ".join(others),
                          ptypes=ptypes, module=module, foreign=imported | generics, core=core))
            continue
        leaves, groups, escapes = classify_body(
            model, text[body_open + 1 : body_close - 1], theme[0], where
        )
        if groups:
            cls = "group"
        elif len(leaves) > 1:
            cls = "multi"
        elif escapes or not leaves:
            cls = "undecided"
        else:
            cls = "one-field"
        fns.append(Fn(name, line, cls, leaves, groups, ptypes=ptypes, module=module,
                      foreign=imported | generics, core=core))
    return fns


def merge(fns):
    """{key: Fn}; a name defined twice (cfg variants) must classify alike."""
    out = {}
    for f in fns:
        seen = out.get(f.key)
        if seen is None:
            out[f.key] = f
        elif (seen.cls, seen.leaves, seen.ptypes) != (f.cls, f.leaves, f.ptypes):
            out[f.key] = Fn(f.name, seen.line, "undecided", set(), set(),
                            "defined twice, read differently", f.ptypes, f.module)
    return out


MOD_RE = re.compile(r"^(?:pub(?:\([^)]*\))?\s+)?mod (\w+)\s*;", re.M)
USE_RE = re.compile(r"^pub use (\w+)::(.*?);", re.M | re.S)
ITEM_RE = re.compile(r"\s*(\w+|\*)(?:\s+as\s+(\w+))?\s*")


def crate_root_fns(model, lib_rel, read_rel):
    """The `pub fn`s at the root of the crate whose `lib.rs` is `lib_rel`:
    its own and those it re-exports from its modules (`read_rel` reads a
    path relative to the repository root)."""
    source = read_rel(lib_rel)
    fns = scan(model, source, lib_rel)
    text = blank(source)
    mods = set(MOD_RE.findall(text))
    src = os.path.dirname(lib_rel)
    for m in USE_RE.finditer(text):
        module, items = m.group(1), m.group(2).strip()
        if module not in mods:
            continue  # another crate's item (`native_theme::...`)
        where = f"{lib_rel}:{text.count(chr(10), 0, m.start()) + 1}"
        if items.startswith("{") and items.endswith("}"):
            items = items[1:-1]
        wanted = {}
        for item in filter(str.strip, items.split(",")):
            parsed = ITEM_RE.fullmatch(item)
            if parsed is None:
                raise Failure(f"{where}: cannot follow `pub use {module}::{item.strip()}`")
            wanted[parsed.group(1)] = parsed.group(2) or parsed.group(1)
        rel, module_source = module_file(src, module, read_rel, where)
        for f in scan(model, module_source, rel):
            if "*" in wanted or f.name in wanted:
                f.name = wanted.get(f.name, f.name)
                fns.append(f)
    return fns


def module_file(src, module, read_rel, where):
    """(path, source) of the module `module` a `lib.rs` in `src` declares."""
    rels = [os.path.join(src, module + ".rs"), os.path.join(src, module, "mod.rs")]
    for rel in rels:
        try:
            return rel, read_rel(rel)
        except Failure:
            continue
    raise Failure(f"{where}: no source for module `{module}` ({' or '.join(rels)})")


PUB_MOD_RE = re.compile(r"^pub mod (\w+)\s*;", re.M)


def public_modules(model, lib_rel, read_rel):
    """{module: [Fn]}: the column-0 `pub fn`s of each module the crate whose
    `lib.rs` is `lib_rel` declares `pub mod` (not `pub(crate)`)."""
    text = blank(read_rel(lib_rel))
    src = os.path.dirname(lib_rel)
    out = {}
    for m in PUB_MOD_RE.finditer(text):
        module = m.group(1)
        where = f"{lib_rel}:{text.count(chr(10), 0, m.start()) + 1}"
        rel, source = module_file(src, module, read_rel, where)
        out[module] = scan(model, source, rel, module)
    return out


# ---------------------------------------------------------------------------
# The check
# ---------------------------------------------------------------------------


def load_exceptions(text, label):
    try:
        data = tomllib.loads(text)
    except tomllib.TOMLDecodeError as error:
        raise Failure(f"{label}: {error}")
    def strings(value):
        """`value` as a list of non-empty strings: one string, or a list."""
        items = [value] if isinstance(value, str) else value
        if not isinstance(items, list) or not items or not all(
            isinstance(s, str) and s.strip() for s in items
        ):
            return None
        return items

    entries = {}
    for i, e in enumerate(data.get("exception", [])):
        where = f"{label}: exception {i + 1}"
        for key in ("kind", "reason"):
            if not isinstance(e.get(key), str) or not e[key].strip():
                raise Failure(f"{where}: `{key}` must be a non-empty string")
        connectors, names = strings(e.get("connector")), strings(e.get("name"))
        for key, value in (("connector", connectors), ("name", names)):
            if value is None:
                raise Failure(f"{where}: `{key}` must be a non-empty string, "
                              "or a non-empty list of them")
        extra = set(e) - {"connector", "name", "kind", "reason"}
        if extra:
            raise Failure(f"{where}: unknown key(s) {', '.join(sorted(extra))}")
        for connector in connectors:
            if connector not in CONNECTORS:
                raise Failure(f"{where}: unknown connector `{connector}`")
        if e["kind"] not in KINDS:
            raise Failure(f"{where}: kind must be one of {', '.join(KINDS)}")
        for connector in connectors:
            for name in names:
                key = (connector, name)
                if key in entries:
                    raise Failure(f"{where}: a second entry for {key[0]} `{key[1]}`")
                entries[key] = {"connector": connector, "name": name,
                                "kind": e["kind"], "reason": e["reason"]}
    return entries


def reference_signature(have, shared):
    """The signature the connectors in `have` ({connector: Fn}) are held to:
    the one most of them share, on a tie the first connector's in
    `CONNECTORS` order."""
    sigs = {c: signature(f, shared) for c, f in have.items()}
    order = [c for c in CONNECTORS if c in sigs] + [c for c in sigs if c not in CONNECTORS]
    counts = {}
    for c in order:
        counts[sigs[c]] = counts.get(sigs[c], 0) + 1
    best = max(counts.values())
    return next(sigs[c] for c in order if counts[sigs[c]] == best), sigs


def check(connectors, exceptions, shared=frozenset()):
    """(report lines, violations, stale) for {connector: {key: Fn}}.

    `shared` names the core crate's types, which a signature keeps.
    """
    report, violations, stale = [], [], []
    used = set()

    def excepted(connector, name, kind):
        e = exceptions.get((connector, name))
        if e is not None and e["kind"] == kind:
            used.add((connector, name))
            return True
        return False

    # accessors that stand for their leaf, per connector: {derived name: Fn}
    held = {}
    for connector, fns in connectors.items():
        counts = {c: 0 for c in CLASSES}
        named = held.setdefault(connector, {})
        for f in fns.values():
            counts[f.cls] += 1
            if f.module is not None:
                continue
            if f.cls == "undecided" and not excepted(connector, f.name, "undecided"):
                violations.append(
                    f"{connector}: `{f.name}` (line {f.line}) is undecided "
                    f"({f.detail or 'passes the theme on, or reads no leaf'}); "
                    "except it with its reason"
                )
            if f.cls != "one-field":
                continue
            want = leaf_name(f.leaf.split("."))
            if f.name == want:
                named[want] = f
            elif not excepted(connector, f.name, "name"):
                violations.append(
                    f"{connector}: `{f.name}` (line {f.line}) reads `{f.leaf}` "
                    f"and is named unlike it: `{want}`"
                )
        if counts["one-field"] == 0:
            raise Failure(f"{connector}: no one-field accessor found; the scan reads nothing")
        report.append(
            f"{connector:5s} pub fn {len(fns):3d}   "
            + "   ".join(f"{c} {counts[c]:2d}" for c in CLASSES)
        )

    # The sibling rule: (connector, key) -> (kind, what) for each connector
    # that lacks a sibling's function, or has it under another signature.
    needs = {}
    every = {}
    for connector, named in held.items():
        for name, f in named.items():
            every.setdefault(name, {})[connector] = f
    for name in sorted(every):
        offered = ", ".join(sorted(every[name]))
        leaf = next(iter(every[name].values())).leaf
        ref, sigs = reference_signature(every[name], shared)
        holders = ", ".join(c for c in connectors if sigs.get(c) == ref)
        for connector, f in every[name].items():
            if sigs[connector] != ref:
                needs[(connector, name)] = (
                    "unlike", f"`{name}{sigs[connector]}` (line {f.line}) is not the "
                    f"`{name}{ref}` {holders} offer(s)")
        for connector, fns in connectors.items():
            if connector in every[name]:
                continue
            other = fns.get(name)
            if other is None:
                needs[(connector, name)] = (
                    "absent", f"no `{name}` (`{leaf}`), which {offered} offer(s)")
            else:
                needs[(connector, name)] = (
                    "unlike", f"`{name}` (line {other.line}) is {other.cls}, not the "
                    f"accessor of `{leaf}` that {offered} offer(s)")
    keys = sorted({k for fns in connectors.values() for k in fns} - set(every))
    for key in keys:
        have = {c: fns[key] for c, fns in connectors.items() if key in fns}
        ref, sigs = reference_signature(have, shared)
        holders = ", ".join(c for c in have if sigs[c] == ref)
        cls = next(f.cls for c, f in have.items() if sigs[c] == ref)
        for connector in connectors:
            if connector not in have:
                needs[(connector, key)] = (
                    "absent", f"no `{key}`, the {cls} function `{key}{ref}` {holders} offer(s)")
            elif sigs[connector] != ref:
                needs[(connector, key)] = (
                    "unlike", f"`{key}{sigs[connector]}` (line {have[connector].line}) is "
                    f"not the `{key}{ref}` {holders} offer(s)")
    for (connector, key), (kind, what) in sorted(needs.items()):
        if not excepted(connector, key, kind):
            violations.append(f"{connector}: {what}; add it or except it with its reason")

    for (connector, name), e in sorted(exceptions.items()):
        if (connector, name) in used:
            continue
        fns = connectors.get(connector, {})
        if e["kind"] in ("absent", "unlike"):
            need = needs.get((connector, name))
            if need is not None:
                why = f"the connector has {'no' if need[0] == 'absent' else 'a'} " \
                      f"function of that name: kind `{need[0]}`"
            elif not any(name in other for c, other in connectors.items() if c != connector):
                why = "no sibling offers it"
            else:
                why = "the connector offers it as its siblings do"
            stale.append(f"{connector}: `{name}` excepted as {e['kind']}, but {why}")
        elif name not in fns:
            stale.append(f"{connector}: `{name}` excepted ({e['kind']}), "
                         "but no such function exists")
        else:
            stale.append(f"{connector}: `{name}` excepted ({e['kind']}), "
                         f"but it is `{fns[name].cls}` and needs no exception")
    report.append(
        f"one-field accessors named after their leaf: {len(every)} distinct; "
        f"other functions compared by name and signature: {len(keys)}; "
        f"sibling differences {len(needs)}; "
        f"exceptions {len(exceptions)} ({len(used)} in use)"
    )
    return report, violations, stale


def read(path):
    try:
        with open(path, encoding="utf-8") as f:
            return f.read()
    except OSError as error:
        raise Failure(f"cannot read {path}: {error}")


def gather(model, read_rel, libs=CONNECTORS):
    """({connector: {key: Fn}}, {connector: {module: [Fn]}}): the functions
    compared, the crate root's and those of each public module a sibling
    publishes too, and the public modules no sibling publishes."""
    roots, modules = {}, {}
    for connector, lib in libs.items():
        roots[connector] = crate_root_fns(model, lib, read_rel)
        modules[connector] = public_modules(model, lib, read_rel)
    publishers = {}
    for connector, mods in modules.items():
        for module in mods:
            publishers.setdefault(module, set()).add(connector)
    connectors, unshared = {}, {}
    for connector in libs:
        fns = list(roots[connector])
        for module, module_fns in modules[connector].items():
            if len(publishers[module]) > 1:
                fns += module_fns
            else:
                unshared.setdefault(connector, {})[module] = module_fns
        connectors[connector] = merge(fns)
    return connectors, unshared


def class_lines(connectors):
    """What each class holds, per connector: one line per non-empty class."""
    lines = []
    for connector, fns in connectors.items():
        for cls in CLASSES:
            names = sorted(k for k, f in fns.items() if f.cls == cls)
            if names:
                lines.append(f"  {connector:5s} {cls:9s} {', '.join(names)}")
    return lines


def core_sources(root):
    src = os.path.join(root, CORE_SOURCES)
    for base, _, files in sorted(os.walk(src)):
        for name in sorted(files):
            if name.endswith(".rs"):
                yield read(os.path.join(base, name))


def run(root, exceptions_path, verbose):
    model = parse_model(read(os.path.join(root, p)) for p in MODEL_FILES)
    shared = core_types(core_sources(root))
    connectors, unshared = gather(model, lambda rel: read(os.path.join(root, rel)))
    exceptions = load_exceptions(read(exceptions_path), os.path.relpath(exceptions_path, root))
    report, violations, stale = check(connectors, exceptions, shared)
    if verbose:
        for connector, fns in connectors.items():
            for f in sorted(fns.values(), key=lambda f: (f.module or "", f.line)):
                what = ", ".join(sorted(f.leaves | f.groups)) or f.detail
                print(f"  {connector} {f.cls:9s} {f.key}{signature(f, shared)}  {what}")
    print("\n".join(report))
    print("\nWhat each class holds (a public module's functions as `module::name`):")
    print("\n".join(class_lines(connectors)))
    print("\nPublic modules no sibling publishes, not compared (each toolkit's own "
          "style builders and conversions):")
    for connector, mods in unshared.items():
        print(f"  {connector:5s} " + ", ".join(
            f"{module} ({len(fns)} pub fn)" for module, fns in sorted(mods.items())))
        if verbose:
            for module, fns in sorted(mods.items()):
                for f in fns:
                    print(f"        {f.key}{signature(f, shared)}")
    if stale:
        print("\nStale exceptions:")
        print("\n".join("  " + line for line in stale))
    if violations:
        print("\nViolations:")
        print("\n".join("  " + line for line in violations))
    if violations or stale:
        return 1
    print("\nEvery accessor is named after its leaf, and every function is in every "
          "connector under one signature, or excepted.")
    return 0


# ---------------------------------------------------------------------------
# Self-test
# ---------------------------------------------------------------------------

SELF_MODEL = '''
pub struct ResolvedFontSpec {
    pub family: Arc<str>,
    pub size: f32,
    pub weight: u16,
}
pub struct ResolvedDefaultsBorder {
    pub color: Rgba,
    pub corner_radius: f32,
}
pub struct ResolvedDefaults {
    pub font: ResolvedFontSpec,
    pub line_height: f32,
    pub info_text_color: Rgba,
    pub border: ResolvedDefaultsBorder,
}
pub struct ResolvedTheme {
    pub defaults: ResolvedDefaults,
    pub tooltip: super::widgets::ResolvedTooltipTheme,
    pub list: super::widgets::ResolvedListTheme,
}

#[derive(Clone, ThemeWidget)]
pub struct TooltipTheme {
    /// Delay before the tooltip shows, in milliseconds.
    pub show_delay_ms: Option<f32>,
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
}

#[derive(Clone, ThemeWidget)]
pub struct ListTheme {
    #[serde(rename = "header_height_px")]
    pub header_height_px: Option<f32>,
}
'''

SELF_ICED = '''
/// Reads `resolved.defaults.line_height` -- in a comment, so nothing.
pub fn line_height(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.line_height
}
pub fn info_text_color(resolved: &ResolvedTheme) -> Color {
    to_color(resolved.defaults.info_text_color)
}
pub fn font_family(resolved: &ResolvedTheme) -> &str {
    &resolved.defaults.font.family
}
pub fn font_size(resolved: &ResolvedTheme, prefs: &AccessibilityPreferences) -> f32 {
    scaled_text_size(resolved.defaults.font.size, prefs)
}
pub fn tooltip_show_delay(resolved: &ResolvedTheme) -> std::time::Duration {
    std::time::Duration::from_millis(resolved.tooltip.show_delay_ms as u64)
}
pub fn list_header_height(r: &ResolvedTheme) -> f32 {
    r.list.header_height_px
}
pub fn swatch(resolved: &ResolvedTheme) -> Swatch {
    let b = &resolved.defaults.border;
    Swatch::new(b.color, b.corner_radius)
}
pub fn padded(resolved: &ResolvedTheme, extra: f32) -> f32 {
    resolved.defaults.border.corner_radius + extra
}
pub fn font(resolved: &ResolvedTheme) -> &ResolvedFontSpec {
    &resolved.defaults.font
}
pub fn to_weight(css: u16) -> Weight {
    Weight::from(css)
}
'''

# A function that passes the theme on whole: undecided.
ESCAPING = '''
pub fn is_dark(resolved: &ResolvedTheme) -> bool {
    is_dark_resolved(resolved)
}
'''


def self_test():
    model = parse_model([SELF_MODEL])
    failures = []

    def expect(label, got, want):
        if got != want:
            failures.append(f"{label}: got {got!r}, want {want!r}")

    # the naming rule, including the two G2 leaves in both spellings
    for path, want in (
        ("defaults.info_text_color", "info_text_color"),
        ("defaults.border.corner_radius", "border_corner_radius"),
        ("scrollbar.groove_width", "scrollbar_groove_width"),
        ("tooltip.show_delay_ms", "tooltip_show_delay"),
        ("tooltip.show_delay", "tooltip_show_delay"),
        ("list.header_height_px", "list_header_height"),
        ("list.header_height", "list_header_height"),
        ("defaults.font.size", "font_size"),
    ):
        expect(f"leaf_name({path})", leaf_name(path.split(".")), want)

    expect("model: tooltip.font is a group",
           resolve_path(model, ["tooltip", "font"])[1], False)
    expect("model: tooltip.show_delay_ms is a leaf",
           resolve_path(model, ["tooltip", "show_delay_ms"])[1], True)

    def connectors(iced):
        fns = merge(scan(model, iced, "iced"))
        return {"iced": fns, "gpui": dict(fns), "egui": dict(fns)}

    clean = connectors(SELF_ICED)
    classes = {n: f.cls for n, f in clean["iced"].items()}
    expect("classes", classes, {
        "line_height": "one-field",
        "info_text_color": "one-field",
        "font_family": "one-field",
        "font_size": "one-field",
        "tooltip_show_delay": "one-field",
        "list_header_height": "one-field",
        "swatch": "multi",
        "padded": "inputs",
        "font": "group",
        "to_weight": "outside",
    })
    _, violations, stale = check(clean, {})
    expect("the clean tree passes", (violations, stale), ([], []))

    # a planted violation: an accessor named unlike its leaf
    renamed = connectors(SELF_ICED.replace("fn info_text_color", "fn info_foreground_color"))
    _, violations, _ = check(renamed, {})
    expect("a misnamed accessor fails",
           any("`info_foreground_color`" in v and "info_text_color" in v for v in violations),
           True)
    named = {("iced", "info_foreground_color"): {"kind": "name", "reason": "x"},
             ("gpui", "info_foreground_color"): {"kind": "name", "reason": "x"},
             ("egui", "info_foreground_color"): {"kind": "name", "reason": "x"}}
    _, violations, stale = check(renamed, named)
    expect("an excepted misnamed accessor passes", (violations, stale), ([], []))

    # a sibling without the accessor
    lacking = dict(clean)
    lacking["egui"] = {n: f for n, f in clean["egui"].items() if n != "line_height"}
    _, violations, _ = check(lacking, {})
    expect("an absent sibling fails",
           [v for v in violations if v.startswith("egui: no `line_height`")] != [], True)
    absent = {("egui", "line_height"): {"kind": "absent", "reason": "x"}}
    _, violations, stale = check(lacking, absent)
    expect("an excepted absent sibling passes", (violations, stale), ([], []))
    _, _, stale = check(clean, absent)
    expect("an absent exception the sibling now meets is stale", len(stale), 1)

    # a sibling with a function of that name that is not the accessor
    unlike = dict(clean)
    unlike["gpui"] = dict(clean["gpui"])
    unlike["gpui"]["line_height"] = Fn("line_height", 1, "outside", set(), set())
    _, violations, _ = check(unlike, {})
    expect("a same-name function of another kind fails",
           [v for v in violations if v.startswith("gpui: `line_height` (line 1) is outside")] != [],
           True)
    _, violations, stale = check(unlike, {("gpui", "line_height"): {"kind": "unlike", "reason": "x"}})
    expect("an excepted same-name function passes", (violations, stale), ([], []))
    _, violations, stale = check(unlike, absent | {("gpui", "line_height"): {"kind": "absent", "reason": "x"}})
    expect("an absent exception where the sibling has the name is stale",
           any("gpui: `line_height` excepted as absent" in line for line in stale), True)
    _, _, stale = check(lacking, {("egui", "line_height"): {"kind": "unlike", "reason": "x"}})
    expect("an unlike exception where the sibling lacks the name is stale",
           any("kind `absent`" in line for line in stale), True)

    # the sibling rule holds every class: a function of each class that one
    # connector lacks fails, passes excepted, and its exception goes stale
    # once the connector has it again
    every_class = connectors(SELF_ICED + ESCAPING)
    undecided = {(c, "is_dark"): {"kind": "undecided", "reason": "x"} for c in every_class}
    expect("every class is in the fixture",
           sorted({f.cls for f in every_class["iced"].values()}), sorted(CLASSES))
    for name, cls in (("swatch", "multi"), ("padded", "inputs"), ("font", "group"),
                      ("to_weight", "outside"), ("is_dark", "undecided")):
        expect(f"the fixture's `{name}` is {cls}", every_class["iced"][name].cls, cls)
        lacking = dict(every_class)
        lacking["egui"] = {n: f for n, f in every_class["egui"].items() if n != name}
        _, violations, _ = check(lacking, undecided)
        expect(f"a {cls} function absent from a sibling fails",
               [v for v in violations if v.startswith(f"egui: no `{name}`")] != [], True)
        absent = undecided | {("egui", name): {"kind": "absent", "reason": "x"}}
        _, violations, stale = check(lacking, absent)
        expect(f"an excepted absent {cls} function passes", (violations, stale), ([], []))
        _, _, stale = check(every_class, absent)
        expect(f"an absent exception for a {cls} function the sibling has is stale",
               any(line.startswith(f"egui: `{name}` excepted as absent") for line in stale), True)

    # a sibling with the function under another signature: the odd one out,
    # and on a tie the later connector in CONNECTORS order, is `unlike`
    changed = SELF_ICED.replace("extra: f32)", "extra: f32, more: bool)")
    unlike_sig = dict(clean)
    unlike_sig["gpui"] = merge(scan(model, changed, "gpui"))
    _, violations, _ = check(unlike_sig, {})
    expect("a sibling with another signature fails", [v for v in violations if "padded" in v], [
        "gpui: `padded(&_, f32, bool)` (line 25) is not the `padded(&_, f32)` iced, egui "
        "offer(s); add it or except it with its reason"])
    _, violations, stale = check(unlike_sig, {("gpui", "padded"): {"kind": "unlike", "reason": "x"}})
    expect("an excepted signature passes", (violations, stale), ([], []))
    _, _, stale = check(unlike_sig, {("gpui", "padded"): {"kind": "absent", "reason": "x"}})
    expect("an absent exception where the sibling has another signature is stale",
           any("kind `unlike`" in line for line in stale), True)
    tie = dict(unlike_sig)
    tie["egui"] = {n: f for n, f in clean["egui"].items() if n != "padded"}
    _, violations, _ = check(tie, {})
    expect("on a tie the later connector is the unlike one",
           sorted(v.split(":")[0] for v in violations if "padded" in v), ["egui", "gpui"])
    expect("on a tie the earlier connector's signature is the reference",
           any(v.startswith("gpui: `padded(&_, f32, bool)`") for v in violations), True)
    _, _, stale = check(clean, {("gpui", "padded"): {"kind": "unlike", "reason": "x"}})
    expect("an unlike exception whose signature matches again is stale", stale,
           ["gpui: `padded` excepted as unlike, but the connector offers it as its siblings do"])

    # a one-field accessor named after its leaf is held to the signature too:
    # a scaled `line_height` beside an unscaled one is another function
    shared = {"ResolvedTheme", "AccessibilityPreferences"}
    scaled = SELF_ICED.replace(
        "pub fn line_height(resolved: &native_theme::theme::ResolvedTheme) -> f32 {",
        "pub fn line_height(resolved: &ResolvedTheme, prefs: &AccessibilityPreferences) -> f32 {")
    one_field_sig = dict(clean)
    one_field_sig["gpui"] = merge(scan(model, scaled, "gpui"))
    expect("the scaled `line_height` is still one-field",
           one_field_sig["gpui"]["line_height"].cls, "one-field")
    _, violations, _ = check(one_field_sig, {}, shared)
    expect("a one-field accessor under another signature fails", violations, [
        "gpui: `line_height(&ResolvedTheme, &AccessibilityPreferences)` (line 3) is not the "
        "`line_height(&ResolvedTheme)` iced, egui offer(s); add it or except it with its reason"])
    unlike_lh = {("gpui", "line_height"): {"kind": "unlike", "reason": "x"}}
    _, violations, stale = check(one_field_sig, unlike_lh, shared)
    expect("an excepted one-field signature passes", (violations, stale), ([], []))
    _, _, stale = check(clean, unlike_lh, shared)
    expect("a one-field unlike exception whose signature matches again is stale", stale,
           ["gpui: `line_height` excepted as unlike, but the connector offers it as its siblings do"])

    # a signature keeps the core crate's types and std's, and reads a
    # toolkit's type, a path and a lifetime away
    f = Fn("f", 1, "outside", set(), set(),
           ptypes=["&'a iced_core::Color", "Option<&str>", "&mut egui::Ui",
                   "native_theme::theme::IconSet", "impl Into<ElementId> + 'static"])
    expect("signature", signature(f, {"IconSet"}),
           "(&_, Option<&str>, &mut _, IconSet, impl Into<_>)")
    # a lifetime-only generic is the bare type, as the same type without one
    expect("a lifetime-only generic", signature(Fn("g", 1, "outside", set(), set(),
           ptypes=["Native<'_>", "Native<'a>", "Native"]), set()), "(_, _, _)")
    # a toolkit's type named like a core one is the toolkit's: written with
    # its path, taken as a generic parameter, or imported from its crate
    themed = merge(scan(model, '''
use iced_core::Theme as IcedTheme;
use iced_core::{Color, Theme};
pub fn by_path(a: &native_theme::theme::Theme, b: &iced_core::Theme) -> u8 { 0 }
pub fn by_generic<'a, Theme: Clone>(a: Theme, b: Option<&'a str>) -> u8 { 0 }
pub fn by_import(a: Theme, b: IcedTheme, c: Color) -> u8 { 0 }
''', "t"))
    expect("a toolkit's `Theme` beside the core's",
           [signature(themed[n], {"Theme"}) for n in ("by_path", "by_generic", "by_import")],
           ["(&Theme, &_)", "(_, Option<&str>)", "(_, _, _)"])
    # a glob import from another crate may bring a `Theme` in, and a type the
    # file declares shadows the core's: both are the toolkit's
    for source, want, label in (
        ("use iced_core::*;\npub fn f(a: &Theme) -> u8 { 0 }",
         "(&_)", "a glob import from another crate"),
        ("use iced_core::{Color, *};\npub fn f(a: &Theme) -> u8 { 0 }",
         "(&_)", "a glob inside a group"),
        ("use iced_core::*;\nuse native_theme::theme::Theme;\npub fn f(a: &Theme) -> u8 { 0 }",
         "(&Theme)", "a glob beside an explicit core import"),
        ("type Theme = iced_core::Theme;\npub fn f(a: &Theme) -> u8 { 0 }",
         "(&_)", "a local type alias"),
        ("pub struct Theme;\npub fn f(a: &Theme) -> u8 { 0 }",
         "(&_)", "a local struct"),
        ("pub(crate) enum Theme { A }\npub fn f(a: &Theme) -> u8 { 0 }",
         "(&_)", "a local enum"),
    ):
        expect(label, signature(merge(scan(model, source, "t"))["f"], {"Theme"}), want)
    expect("a core `Theme` imported from native_theme stays the core's",
           signature(merge(scan(model, "use native_theme::theme::Theme;\n"
                                "pub fn f(a: &Theme) -> u8 { 0 }", "t"))["f"], {"Theme"}),
           "(&Theme)")
    expect("core_types", sorted(core_types(["pub struct A;\npub(crate) struct B;\n"
                                            "pub enum C {}\n// pub struct D;\npub trait E {}"])),
           ["A", "C", "E"])

    # an exception entry takes a list of connectors and of names
    listed = load_exceptions('[[exception]]\nconnector = ["iced", "egui"]\n'
                             'name = ["a", "b"]\nkind = "absent"\nreason = "x"\n', "t")
    expect("a listed entry stands for every pair", sorted(listed),
           [("egui", "a"), ("egui", "b"), ("iced", "a"), ("iced", "b")])
    for planted, label in (
        ('connector = ["iced", "iced"]\nname = "a"', "a pair listed twice"),
        ('connector = []\nname = "a"', "an empty list"),
        ('connector = "iced"\nname = ["a", 1]', "a list holding a non-string"),
    ):
        try:
            load_exceptions(f'[[exception]]\n{planted}\nkind = "absent"\nreason = "x"\n', "t")
            failures.append(f"{label}: no error")
        except Failure:
            pass

    # public modules: one a sibling publishes too is compared by its path,
    # one no sibling publishes is listed, a `pub(crate)` one is not read
    modules = {
        "iced/src/lib.rs": "pub mod icons;\npub mod styles;\npub(crate) mod inner;\n" + SELF_ICED,
        "iced/src/icons.rs": "pub fn to_handle(data: &IconData) -> Handle { todo() }",
        "iced/src/styles.rs": "pub fn button(r: &ResolvedTheme) -> Style { r.defaults.border.color }",
        "gpui/src/lib.rs": "pub mod icons;\n" + SELF_ICED,
        "gpui/src/icons/mod.rs": "pub fn to_handle(data: &IconData) -> Source { todo() }",
        "egui/src/lib.rs": SELF_ICED,
    }

    def read_modules(rel):
        if rel not in modules:
            raise Failure(f"cannot read {rel}")
        return modules[rel]

    libs = {c: f"{c}/src/lib.rs" for c in ("iced", "gpui", "egui")}
    gathered, unshared = gather(model, read_modules, libs)
    expect("a shared module's function is compared by its path",
           [c for c in libs if "icons::to_handle" in gathered[c]], ["iced", "gpui"])
    expect("a module no sibling publishes is listed, not compared",
           ({c: sorted(m) for c, m in unshared.items()}, "styles::button" in gathered["iced"]),
           ({"iced": ["styles"]}, False))
    _, violations, _ = check(gathered, {})
    expect("a shared module's function absent from a sibling fails",
           any(v.startswith("egui: no `icons::to_handle`") for v in violations), True)

    # the crate root: lib.rs's own functions and those it re-exports
    tree = {
        "src/lib.rs": """
mod acc;
#[cfg(feature = "x")]
pub(crate) mod hidden;
pub use acc::{
    line_height, info_text_color as info_foreground_color,
};
pub use native_theme::Rgba;
pub use hidden::*;
pub fn to_weight(css: u16) -> Weight { Weight::from(css) }
""",
        "src/acc.rs": SELF_ICED,
        "src/hidden/mod.rs": "pub fn swatch_of(r: &ResolvedTheme) -> Rgba { r.defaults.border.color }",
    }

    def read_tree(rel):
        if rel not in tree:
            raise Failure(f"cannot read {rel}")
        return tree[rel]

    root = sorted(f.name for f in crate_root_fns(model, "src/lib.rs", read_tree))
    expect("the crate root follows its re-exports", root,
           ["info_foreground_color", "line_height", "swatch_of", "to_weight"])
    for planted, label in (
        ("pub use acc::{line_height, sub::{font}};", "a nested re-export"),
        ("mod gone;\npub use gone::font;", "a re-exported module without a source"),
    ):
        tree["src/lib.rs"] += planted
        try:
            crate_root_fns(model, "src/lib.rs", read_tree)
            failures.append(f"{label}: no error")
        except Failure:
            pass
        tree["src/lib.rs"] = tree["src/lib.rs"].removesuffix(planted)

    # undecided: the theme passed on whole
    escaping = connectors(SELF_ICED + ESCAPING)
    expect("an escaping function is undecided", escaping["iced"]["is_dark"].cls, "undecided")
    _, violations, _ = check(escaping, {})
    expect("an undecided function without an exception fails",
           sum("`is_dark`" in v for v in violations), 3)

    # a stale exception: the function is gone
    _, _, stale = check(clean, {("iced", "gone"): {"kind": "name", "reason": "x"}})
    expect("an exception naming no function is stale", len(stale), 1)

    # a model field the model lacks is an input error, not a silent miss
    try:
        scan(model, "pub fn x(r: &ResolvedTheme) -> f32 { r.defaults.no_such_leaf }", "t")
        failures.append("an unknown field: no error")
    except Failure:
        pass

    # fail closed: a type the model lacks, a connector the scan finds nothing in
    try:
        parse_model([SELF_MODEL.replace("pub struct ResolvedDefaultsBorder", "pub struct Elsewhere")])
        failures.append("a Resolved* type absent from the model: no error")
    except Failure:
        pass
    try:
        empty = dict(clean)
        empty["egui"] = {n: f for n, f in clean["egui"].items() if f.cls != "one-field"}
        check(empty, {})
        failures.append("a connector with no one-field accessor: no error")
    except Failure:
        pass

    for line in failures:
        print("  FAIL " + line)
    if failures:
        print(f"self-test: {len(failures)} failure(s)")
        return 1
    print("self-test: every scenario passes")
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--root", default=PROJECT_ROOT,
                        help="repository root to read instead of this one "
                        "(a scratch copy, to try a change)")
    parser.add_argument("--exceptions", default=None,
                        help="exception file to read instead of "
                        "<root>/scripts/connector_parity_exceptions.toml")
    parser.add_argument("--self-test", action="store_true",
                        help="run the built-in scenarios instead of the check")
    parser.add_argument("-v", "--verbose", action="store_true",
                        help="list every function with its class and leaves")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    exceptions = args.exceptions or os.path.join(
        args.root, os.path.relpath(EXCEPTIONS, PROJECT_ROOT)
    )
    return run(args.root, exceptions, args.verbose)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Failure as error:
        print(f"check_connector_parity: {error}", file=sys.stderr)
        sys.exit(2)
