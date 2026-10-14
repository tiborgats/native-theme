#!/usr/bin/env python3
"""Check that the connectors' accessors are named after the model's fields.

The iced, gpui and egui connectors each offer free functions that read a
value of a `ResolvedTheme`. A value documented once in
docs/platform-facts.md should be found under one name in all three, the
model's own, so this script lists each connector's `pub fn` names and the
leaves each reads, and fails on an accessor named unlike its leaf or present
in one connector and absent from a sibling, unless
scripts/connector_parity_exceptions.toml lists it with its reason.

Run from the repository root:

    python3 scripts/check_connector_parity.py
    python3 scripts/check_connector_parity.py --self-test

Exit 0 when every accessor passes or is excepted and no exception is stale,
1 on a violation or a stale exception, 2 on an input it cannot read (a model
field the type model lacks, a `Resolved*` type no model source declares, a
connector in which it finds no one-field accessor, a malformed exception
file).

The naming rule
---------------
A one-field accessor -- a public free function whose result comes from
exactly one `ResolvedTheme` leaf (possibly converted to the toolkit's type or
unit, or scaled by the text-scaling factor) -- is named after that leaf's
path: drop the leading `defaults.`, join the remaining segments with `_`, and
drop a trailing unit suffix (`_px`, `_ms`, `_pt`), since the accessor returns
the toolkit's type. `defaults.info_text_color` is `info_text_color`,
`scrollbar.groove_width` is `scrollbar_groove_width`, `tooltip.show_delay_ms`
is `tooltip_show_delay`.

The sibling rule
----------------
A one-field accessor present in one connector and absent from a sibling fails
unless the exception list names the sibling and the accessor with the reason
the sibling has none: `kind = "absent"` where the sibling has no function of
that name, `kind = "unlike"` where it has one that is not that accessor (a
different signature or meaning, gpui's `font_family(&Arc<str>)` beside iced's
and egui's `font_family(resolved)`).

What is scanned
---------------
The functions at the crate root, where the accessors live: every `pub fn` at
column 0 of each connector's `src/lib.rs`, and every one a column-0
`pub use <module>::{..};` (or `pub use <module>::<name>;`, `as` renames and
`*` too) of `lib.rs` re-exports from a module the same `lib.rs` declares
(`src/<module>.rs` or `src/<module>/mod.rs`), so egui's `accessors.rs` and
`ext.rs` and a module a later re-export names are read without a list here.
A `pub use` of a local module the script cannot follow (a nested path or
group) is an input error. Methods, private functions and the modules that are
not re-exported (`styles`, `geometry`, `palette`, ...) are style builders and
conversions, not accessors. Comments and string literals are blanked before any match, so a doc
comment naming a leaf reads nothing.

Deciding "one leaf"
-------------------
The type model is read from the core crate's sources: the `Resolved*` structs
of native-theme/src/model/resolved.rs, border.rs and font.rs, and the widget
structs of model/widgets/mod.rs, whose `#[theme(nested, resolved_type = ...)]`
fields are the nested structs of the generated `Resolved*Theme`. A path that
ends on a struct is a group, any other a leaf, and a path the model does not
have is an input error, so a renamed model field cannot leave the script
matching nothing.

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
chain ends at a method call or at its leaf. Only one-field accessors are held
to the two rules. An undecided function, and a one-field function the rule
does not fit (a widget builder that reads one leaf, a derived flag), needs an
exception entry (`kind = "undecided"`, `kind = "name"`); nothing is silently
skipped, and the script prints how many functions it classified each way.

Exceptions
----------
scripts/connector_parity_exceptions.toml, one `[[exception]]` per
(connector, name) with `kind` and a non-empty `reason`. An exception is stale,
and fails the check, when its function no longer exists, or when it no longer
needs one: a `name` exception whose function is no longer a misnamed one-field
accessor, an `undecided` one whose function is now decided, an `absent` one
whose accessor the connector now has or no sibling has any more, or whose
function of that name the connector now has (or no longer has) where the
other kind fits, `unlike` for `absent` and the reverse.
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
    def __init__(self, name, line, cls, leaves, groups, detail=""):
        self.name = name
        self.line = line
        self.cls = cls
        self.leaves = leaves
        self.groups = groups
        self.detail = detail

    @property
    def leaf(self):
        return next(iter(self.leaves)) if len(self.leaves) == 1 else None


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


def scan(model, source, label):
    """The crate-root `pub fn`s of one source file, classified."""
    text = blank(source)
    fns = []
    for m in FN_RE.finditer(text):
        name = m.group(1)
        line = text.count("\n", 0, m.start()) + 1
        pos = m.end()
        if text[pos : pos + 1] == "<":
            pos = balanced(text, pos, "<", ">")
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
        where = f"{label}:{line} `{name}`"
        if not theme:
            fns.append(Fn(name, line, "outside", set(), set()))
            continue
        if len(theme) > 1 or others:
            fns.append(Fn(name, line, "inputs", set(), set(), ", ".join(others)))
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
        fns.append(Fn(name, line, cls, leaves, groups))
    return fns


def merge(fns):
    """{name: Fn}; a name defined twice (cfg variants) must classify alike."""
    out = {}
    for f in fns:
        seen = out.get(f.name)
        if seen is None:
            out[f.name] = f
        elif (seen.cls, seen.leaves) != (f.cls, f.leaves):
            out[f.name] = Fn(f.name, seen.line, "undecided", set(), set(),
                             "defined twice, read differently")
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
        rels = [os.path.join(src, module + ".rs"), os.path.join(src, module, "mod.rs")]
        for rel in rels:
            try:
                module_source = read_rel(rel)
            except Failure:
                continue
            for f in scan(model, module_source, rel):
                if "*" in wanted or f.name in wanted:
                    f.name = wanted.get(f.name, f.name)
                    fns.append(f)
            break
        else:
            raise Failure(f"{where}: no source for module `{module}` ({' or '.join(rels)})")
    return fns


# ---------------------------------------------------------------------------
# The check
# ---------------------------------------------------------------------------


def load_exceptions(text, label):
    try:
        data = tomllib.loads(text)
    except tomllib.TOMLDecodeError as error:
        raise Failure(f"{label}: {error}")
    entries = {}
    for i, e in enumerate(data.get("exception", [])):
        where = f"{label}: exception {i + 1}"
        for key in ("connector", "name", "kind", "reason"):
            if not isinstance(e.get(key), str) or not e[key].strip():
                raise Failure(f"{where}: `{key}` must be a non-empty string")
        extra = set(e) - {"connector", "name", "kind", "reason"}
        if extra:
            raise Failure(f"{where}: unknown key(s) {', '.join(sorted(extra))}")
        if e["connector"] not in CONNECTORS:
            raise Failure(f"{where}: unknown connector `{e['connector']}`")
        if e["kind"] not in KINDS:
            raise Failure(f"{where}: kind must be one of {', '.join(KINDS)}")
        key = (e["connector"], e["name"])
        if key in entries:
            raise Failure(f"{where}: a second entry for {key[0]} `{key[1]}`")
        entries[key] = e
    return entries


def check(connectors, exceptions):
    """(report lines, violations, stale) for {connector: {name: Fn}}."""
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

    every = {}
    for connector, named in held.items():
        for name, f in named.items():
            every.setdefault(name, {})[connector] = f
    for name in sorted(every):
        for connector, fns in connectors.items():
            if connector in every[name]:
                continue
            other = fns.get(name)
            if not excepted(connector, name, "absent" if other is None else "unlike"):
                others = ", ".join(sorted(every[name]))
                leaf = next(iter(every[name].values())).leaf
                if other is None:
                    what = f"no `{name}` (`{leaf}`), which {others} offer(s)"
                else:
                    what = (f"`{name}` (line {other.line}) is {other.cls}, not the "
                            f"accessor of `{leaf}` that {others} offer(s)")
                violations.append(f"{connector}: {what}; add it or except it with its reason")

    for (connector, name), e in sorted(exceptions.items()):
        if (connector, name) in used:
            continue
        fns = connectors.get(connector, {})
        if e["kind"] in ("absent", "unlike"):
            if name not in every:
                why = "no connector offers it"
            elif connector in every[name]:
                why = "the connector offers it"
            elif e["kind"] == "absent":
                why = "the connector has a function of that name: kind `unlike`"
            else:
                why = "the connector has no function of that name: kind `absent`"
            stale.append(f"{connector}: `{name}` excepted as {e['kind']}, but {why}")
        elif name not in fns:
            stale.append(f"{connector}: `{name}` excepted ({e['kind']}), "
                         "but no such function exists")
        else:
            stale.append(f"{connector}: `{name}` excepted ({e['kind']}), "
                         f"but it is `{fns[name].cls}` and needs no exception")
    report.append(
        f"one-field accessors named after their leaf: {len(every)} distinct; "
        f"exceptions {len(exceptions)} ({len(used)} in use)"
    )
    return report, violations, stale


def read(path):
    try:
        with open(path, encoding="utf-8") as f:
            return f.read()
    except OSError as error:
        raise Failure(f"cannot read {path}: {error}")


def run(root, exceptions_path, verbose):
    model = parse_model(read(os.path.join(root, p)) for p in MODEL_FILES)
    connectors = {
        connector: merge(crate_root_fns(model, lib, lambda rel: read(os.path.join(root, rel))))
        for connector, lib in CONNECTORS.items()
    }
    exceptions = load_exceptions(read(exceptions_path), os.path.relpath(exceptions_path, root))
    report, violations, stale = check(connectors, exceptions)
    if verbose:
        for connector, fns in connectors.items():
            for f in sorted(fns.values(), key=lambda f: f.line):
                what = ", ".join(sorted(f.leaves | f.groups)) or f.detail
                print(f"  {connector} {f.cls:9s} {f.name}  {what}")
    print("\n".join(report))
    if stale:
        print("\nStale exceptions:")
        print("\n".join("  " + line for line in stale))
    if violations:
        print("\nViolations:")
        print("\n".join("  " + line for line in violations))
    if violations or stale:
        return 1
    print("\nEvery accessor is named after its leaf, in every connector or excepted.")
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
    escaping = connectors(SELF_ICED + '''
pub fn is_dark(resolved: &ResolvedTheme) -> bool {
    is_dark_resolved(resolved)
}
''')
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
