#!/usr/bin/env python3
"""No `cfg` on the facts gate's path: its outcome is one for every target and feature set. Offline.

The facts gate (`native-theme/tests/facts_gate.rs`'s `facts_gate`) runs on one configuration only: CI's
`facts-gate` jobs on Linux with the default features, `scripts/check_release.sh` in one step, while the other
test steps and jobs skip it. That holds only while no code on its path depends on the target or the features. This
check finds that path from the sources and fails on any `cfg` along it but a test-only or a lint-only one.

The path
--------
Its roots are the gate's test files (`facts_gate.rs` and the files its `#[path]` modules name), the crate root
(`native-theme/src/lib.rs`), the derive crate (`native-theme-derive/src`, whose expansions are everywhere) and the
build script (`native-theme/build.rs`). From each module on it, every module it refers to joins it, found from the
`use` and `mod` graph, never from a list kept by hand:
  * every path a `use` tree names and every path in the code (`crate::`, `super::`, `self::`, `native_theme::`, a
    child module's or an imported name's), resolved through the modules' `mod` declarations, their items and their
    re-exports (`pub use`, globs among them) to the module that defines the item; a path that resolves no further
    reaches the module it stopped in;
  * every macro a module invokes, the modules that define a `macro_rules!` of that name;
  * a module off the path with an `impl` of a type of a module on the path, as it adds methods or trait impls
    to what the gate runs (an impl of a trait on the path for a type off it adds nothing the gate can name).
One edge is cut, in CUTS: a module on the path names another in a function alone, and that function's name appears
in no other module on the path, so nothing on the path calls it; the check fails where the edge is gone (the cut is
stale) or the name appears elsewhere on the path (the cut no longer holds).
Test-only items (a `cfg` whose predicate names `test`, `doctest` or `doc` alone, through `all`/`any`/`not`, or
`test` in an `all`) are left out, their bodies with them; a module declared under one is not on the path.

The rule
--------
Along the path, every `#[cfg(...)]`, `#![cfg(...)]`, `cfg!(...)` and `#[cfg_attr(...)]` fails but:
  * a test-only predicate (above), which is the same for every target and feature set;
  * a `cfg_attr` whose attributes are lint levels alone (`allow`, `expect`, `warn`, `deny`);
  * a `cfg` on the `mod` declaration of a module off the path (lib.rs's platform modules).
The `mod` declaration of a module on the path and the `pub use` lines a resolution goes through are checked by the
same rule, wherever they are; so is any read of `CARGO_FEATURE_` or `CARGO_CFG_` in the build script and the derive
crate. Comments and string literals are no code.

Run from the repository root:

    python3 scripts/check_gate_cfg.py            # the check; --list also prints the modules on the path
    python3 scripts/check_gate_cfg.py --self-test

Exit 0 when nothing on the path has such a `cfg`, 1 when something does (each finding a line, with why), 2 on a
source it cannot read or a wrong command line.
"""

from __future__ import annotations

import re
import sys
import tempfile
from dataclasses import dataclass, field
from pathlib import Path

WHY = (
    "the facts gate runs on one configuration only (CI's Linux facts-gate shards with the default features, "
    "check_release.sh's single gate step; the other test jobs skip it), so no code on its path may depend on "
    "the target or the features: move this off the path, or make it the same on every target and feature set, "
    "or gate on every configuration again"
)

TEST_ATOMS = {"test", "doctest", "doc"}

# The edges the path does not follow: (the module on the path, the module it names, the function that names it,
# why). Each holds only while that function's name appears in no other module on the path (checked).
CUTS = [
    (
        "icons::model",
        "icons::bundled",
        "icon_svg",
        "IconProvider::icon_svg, the bundled icon sets' SVG data: the gate loads no icon",
    ),
]
LINT_LEVELS = {"allow", "expect", "warn", "deny"}
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"


class InputError(Exception):
    pass


# --- lexing -------------------------------------------------------------------------------------------------------


def blank(text: str) -> str:
    """`text` with its comments and the contents of its string and character literals turned into spaces, each
    newline kept, so offsets and line numbers stay the source's."""
    out = list(text)
    i, n = 0, len(text)

    def clear(start: int, end: int) -> None:
        for k in range(start, end):
            if out[k] != "\n":
                out[k] = " "

    while i < n:
        c = text[i]
        if text.startswith("//", i):
            end = text.find("\n", i)
            end = n if end < 0 else end
            clear(i, end)
            i = end
        elif text.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif text.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            clear(i, j)
            i = j
        elif (m := re.match(r'(?:b|c)?r(#*)"', text[i:])) and (i == 0 or not re.match(r"\w", text[i - 1])):
            hashes = m.group(1)
            start = i + m.end()
            end = text.find('"' + hashes, start)
            end = n if end < 0 else end
            clear(start, end)
            i = end + 1 + len(hashes)
        elif c == '"' or (c in "bc" and text.startswith('"', i + 1) and (i == 0 or not re.match(r"\w", text[i - 1]))):
            j = i + (1 if c == '"' else 2)
            start = j
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            clear(start, min(j, n))
            i = j + 1
        elif c == "'":
            if text.startswith("\\", i + 1):
                end = text.find("'", i + 2)
                end = n if end < 0 else end
                clear(i + 1, end)
                i = end + 1
            elif i + 2 < n and text[i + 2] == "'":
                clear(i + 1, i + 2)
                i += 3
            else:
                i += 1
        else:
            i += 1
    return "".join(out)


def closing(text: str, start: int, open_: str = "{", close: str = "}") -> int:
    """The offset just after the bracket that closes the one at `start` (blanked text)."""
    depth = 0
    for k in range(start, len(text)):
        if text[k] == open_:
            depth += 1
        elif text[k] == close:
            depth -= 1
            if depth == 0:
                return k + 1
    return len(text)


def line_of(text: str, offset: int) -> int:
    return text.count("\n", 0, offset) + 1


# --- cfg predicates -----------------------------------------------------------------------------------------------


def parse_predicate(text: str):
    """A cfg predicate as a tree: ("atom", name) or (op, [children]); None where it does not read."""
    tokens = re.findall(r"[A-Za-z_][A-Za-z0-9_]*|=|\(|\)|,|\"[^\"]*\"|\S", text)
    pos = 0

    def node():
        nonlocal pos
        if pos >= len(tokens):
            return None
        name = tokens[pos]
        pos += 1
        if pos < len(tokens) and tokens[pos] == "(" and name in ("all", "any", "not"):
            pos += 1
            children = []
            while pos < len(tokens) and tokens[pos] != ")":
                child = node()
                if child is None:
                    return None
                children.append(child)
                if pos < len(tokens) and tokens[pos] == ",":
                    pos += 1
            pos += 1
            return (name, children)
        if pos < len(tokens) and tokens[pos] == "=":
            pos += 2
            return ("atom", name + "=")
        return ("atom", name)

    tree = node()
    return tree if pos >= len(tokens) else None


def test_only(tree) -> bool:
    """Whether a predicate is the same for every target and feature set: it names `test`, `doctest` or `doc` alone,
    or it is an `all` holding `test` (off in the gate's build, whatever else it names)."""
    if tree is None:
        return False
    kind, value = tree
    if kind == "atom":
        return value in TEST_ATOMS
    if kind == "all" and any(child == ("atom", "test") for child in value):
        return True
    return all(test_only(child) for child in value)


def compiled_out(tree) -> bool:
    """Whether a predicate holds in no build the gate makes: `test`, or an `all` holding `test`."""
    if tree is None:
        return False
    kind, value = tree
    if kind == "atom":
        return value == "test"
    if kind == "all":
        return any(compiled_out(child) for child in value)
    return False


def split_top(text: str) -> list[str]:
    parts, depth, start = [], 0, 0
    for k, c in enumerate(text):
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == "," and depth == 0:
            parts.append(text[start:k])
            start = k + 1
    parts.append(text[start:])
    return [part.strip() for part in parts if part.strip()]


@dataclass
class Cfg:
    offset: int
    end: int
    kind: str  # "cfg", "cfg_attr", "cfg!"
    text: str
    allowed: bool
    removes: bool  # a cfg attribute whose item no gate build holds


def cfgs(text: str) -> list[Cfg]:
    """Every cfg attribute and `cfg!` in blanked `text`."""
    found = []
    for m in re.finditer(r"#!?\[\s*(cfg_attr|cfg)\s*\(|\bcfg!\s*\(", text):
        paren = m.end() - 1
        end = closing(text, paren, "(", ")")
        inner = text[paren + 1 : end - 1]
        if m.group(1) == "cfg_attr":
            parts = split_top(inner)
            tree = parse_predicate(parts[0]) if parts else None
            lints = all(re.match(r"(\w+)\s*\(", part) and re.match(r"(\w+)", part).group(1) in LINT_LEVELS for part in parts[1:])
            allowed = bool(parts[1:]) and (lints or test_only(tree))
            found.append(Cfg(m.start(), end, "cfg_attr", text[m.start() : end + 1], allowed, False))
        else:
            tree = parse_predicate(inner)
            kind = "cfg" if m.group(1) else "cfg!"
            found.append(
                Cfg(m.start(), end, kind, text[m.start() : end + (1 if m.group(1) else 0)], test_only(tree),
                    kind == "cfg" and compiled_out(tree))
            )
    return found


def item_after(text: str, offset: int) -> tuple[int, int]:
    """The item an attribute ending at `offset` applies to: its start, past further attributes, and its end (its
    closing brace, or its `;`)."""
    k = offset
    while True:
        m = re.compile(r"\s*#!?\[").match(text, k)
        if not m:
            break
        k = closing(text, m.end() - 1, "[", "]")
    start = len(text) - len(text[k:].lstrip())
    depth = 0
    for j in range(start, len(text)):
        c = text[j]
        if c in "([":
            depth += 1
        elif c in ")]":
            depth -= 1
        elif depth == 0 and c == ";":
            return start, j + 1
        elif depth == 0 and c == "{":
            return start, closing(text, j)
    return start, len(text)


def without_test_items(text: str) -> str:
    """Blanked `text` with every item a compiled-out `cfg` attribute applies to blanked too, the attribute with it."""
    out = list(text)
    for cfg in cfgs(text):
        if not cfg.removes or cfg.kind != "cfg":
            continue
        _, end = item_after(text, cfg.end + 1)
        for k in range(cfg.offset, end):
            if out[k] != "\n":
                out[k] = " "
    return "".join(out)


# --- modules ------------------------------------------------------------------------------------------------------


@dataclass
class Use:
    path: list[str]
    name: str  # the name it binds, "*" for a glob, "" for `self`'s own
    public: bool
    offset: int


@dataclass
class Module:
    path: tuple[str, ...]
    file: Path | None
    text: str = ""
    raw: str = ""
    children: dict[str, "Module"] = field(default_factory=dict)
    parent: "Module | None" = None
    start: int = 0  # its text's offset in its file (an inline module's body)
    file_raw: str = ""
    decl_file: Path | None = None
    decl_raw: str = ""
    decl_text: str = ""
    decl_attrs: tuple[int, int] = (0, 0)
    items: set[str] = field(default_factory=set)
    macros: set[str] = field(default_factory=set)
    uses: list[Use] = field(default_factory=list)

    def name(self) -> str:
        return "::".join(self.path) or "crate"


def parse_use_tree(text: str) -> list[tuple[list[str], str]]:
    """A `use` tree's paths and the names they bind."""
    tokens = re.findall(rf"{IDENT}|::|\{{|\}}|,|\*", text)
    pos = 0

    def tree(prefix: list[str]) -> list[tuple[list[str], str]]:
        nonlocal pos
        out: list[tuple[list[str], str]] = []
        path = list(prefix)
        while pos < len(tokens):
            tok = tokens[pos]
            if tok == "{":
                pos += 1
                while pos < len(tokens) and tokens[pos] != "}":
                    out.extend(tree(path))
                    if pos < len(tokens) and tokens[pos] == ",":
                        pos += 1
                pos += 1
                return out
            if tok == "*":
                pos += 1
                return out + [(path, "*")]
            if tok == "::":
                pos += 1
                continue
            if tok in (",", "}"):
                break
            if tok == "as":
                pos += 1
                alias = tokens[pos] if pos < len(tokens) else ""
                pos += 1
                return out + [(path, alias)]
            pos += 1
            if tok == "self" and path:
                continue
            path.append(tok)
        if path:
            out.append((path, path[-1]))
        return out

    return tree([])


class Crate:
    def __init__(self, src: Path) -> None:
        self.src = src
        self.modules: list[Module] = []
        self.root = self.load((), src / "lib.rs", None)

    def load(self, path: tuple[str, ...], file: Path, parent: Module | None) -> Module:
        try:
            raw = file.read_text(encoding="utf-8")
        except OSError as error:
            raise InputError(f"{file}: {error}") from error
        module = Module(path=path, file=file, raw=raw, parent=parent, file_raw=raw)
        module.text = without_test_items(blank(raw))
        self.modules.append(module)
        self.index(module)
        return module

    def child_file(self, module: Module, name: str, attrs_raw: str) -> Path | None:
        explicit = re.search(r'#\[\s*path\s*=\s*"([^"]+)"', attrs_raw)
        assert module.file is not None
        base = module.file.parent if module.file.name in ("lib.rs", "mod.rs") else module.file.with_suffix("")
        if explicit:
            return module.file.parent / explicit.group(1)
        for candidate in (base / f"{name}.rs", base / name / "mod.rs"):
            if candidate.is_file():
                return candidate
        return None

    def index(self, module: Module) -> None:
        text = module.text
        decl = re.compile(rf"((?:#\[[^\]]*\]\s*)*)(?:pub(?:\s*\([^)]*\))?\s+)?\bmod\s+({IDENT})\s*(;|\{{)")
        for m in decl.finditer(text):
            name = m.group(2)
            attrs_raw = module.raw[m.start(1) : m.end(1)]
            if m.group(3) == ";":
                file = self.child_file(module, name, attrs_raw)
                if file is None:
                    raise InputError(f"{module.file}: `mod {name};` names no file")
                child = self.load(module.path + (name,), file, module)
            else:
                child = Module(path=module.path + (name,), file=module.file, parent=module, file_raw=module.file_raw)
                end = closing(text, m.end() - 1)
                child.text = text[m.end() : end - 1]
                child.raw = module.raw[m.end() : end - 1]
                child.start = module.start + m.end()
                self.modules.append(child)
                self.index(child)
            child.decl_file, child.decl_text = module.file, text
            child.decl_raw = module.file_raw
            child.decl_attrs = (module.start + m.start(1), module.start + m.end(1))
            module.children[name] = child
        module.items = set(re.findall(rf"\b(?:fn|struct|enum|trait|type|const|static|union)\s+({IDENT})", text))
        module.macros = set(re.findall(rf"\bmacro_rules!\s*({IDENT})", text))
        module.items |= module.macros
        for m in re.finditer(r"\buse\s+", text):
            end = text.find(";", m.end())
            if end < 0:
                continue
            before = text[max(0, m.start() - 20) : m.start()]
            public = bool(re.search(r"\bpub(?:\s*\([^)]*\))?\s*$", before))
            for path, name in parse_use_tree(text[m.end() : end]):
                module.uses.append(Use(path, name, public, m.start()))

    def macro_definers(self, name: str) -> list[Module]:
        return [module for module in self.modules if name in module.macros]


class Reach:
    """The modules on the gate's path, and the findings met while resolving it."""

    def __init__(self, crate: Crate) -> None:
        self.crate = crate
        self.reached: set[int] = set()
        self.queue: list[Module] = []
        self.through: set[tuple[int, int]] = set()  # (module, use offset) re-exports a resolution went through
        self.why: dict[int, str] = {}

    def add(self, module: Module, why: str) -> None:
        if id(module) not in self.reached:
            self.reached.add(id(module))
            self.why[id(module)] = why
            self.queue.append(module)

    def is_reached(self, module: Module) -> bool:
        return id(module) in self.reached

    def resolve(self, origin: Module | None, segments: list[str], seen: frozenset = frozenset()) -> list[Module]:
        """The modules a path from `origin` (None: a test file, which names the crate `native_theme`) ends in."""
        if not segments:
            return [origin] if origin else []
        first, rest = segments[0], segments[1:]
        root = self.crate.root
        if first in ("crate", "native_theme") and (first == "crate") == (origin is not None):
            return self.walk(root, rest, seen)
        if origin is None:
            return []
        if first == "super":
            return self.resolve_from(origin.parent or root, rest, seen)
        if first == "self":
            return self.walk(origin, rest, seen)
        if first in origin.children:
            return self.walk(origin.children[first], rest, seen)
        for use in origin.uses:
            if use.name == first and use.path and (id(origin), use.offset, first) not in seen:
                key = seen | {(id(origin), use.offset, first)}
                return [t for target in self.resolve(origin, use.path, key) for t in self.walk(target, rest, key)] \
                    if rest else self.resolve(origin, use.path, key)
        return []

    def resolve_from(self, module: Module, segments: list[str], seen: frozenset) -> list[Module]:
        if segments and segments[0] == "super":
            return self.resolve_from(module.parent or self.crate.root, segments[1:], seen)
        return self.walk(module, segments, seen)

    def walk(self, module: Module, segments: list[str], seen: frozenset) -> list[Module]:
        if not segments:
            return [module]
        first, rest = segments[0], segments[1:]
        if first in module.children:
            return self.walk(module.children[first], rest, seen)
        if first in module.items:
            return [module]
        found: list[Module] = []
        for use in module.uses:
            if not use.public or (id(module), use.offset, first) in seen:
                continue
            key = seen | {(id(module), use.offset, first)}
            if use.name == first:
                self.through.add((id(module), use.offset))
                for target in self.resolve(module, use.path, key):
                    found.extend(self.walk(target, rest, key) if rest else [target])
            elif use.name == "*":
                for target in self.resolve(module, use.path, key):
                    if first in target.items or first in target.children or any(
                        u.public and u.name in (first, "*") for u in target.uses
                    ):
                        self.through.add((id(module), use.offset))
                        found.extend(self.walk(target, segments, key))
        return found or [module]


def references(text: str) -> list[list[str]]:
    """The paths the code names, its `use` statements left out (their trees are followed as uses)."""
    text = re.sub(r"\buse\s+[^;]*;", lambda m: re.sub(r"[^\n]", " ", m.group(0)), text)
    paths = re.findall(rf"(?<![\w:]){IDENT}(?:\s*::\s*{IDENT})+", text)
    return [re.split(r"\s*::\s*", path) for path in paths]


def macro_calls(text: str) -> set[str]:
    return set(re.findall(rf"(?<![\w:])({IDENT})\s*!\s*[\(\[\{{]", text)) - {"macro_rules"}


def impl_names(text: str) -> list[list[list[str]]]:
    """Each `impl` header's trait and type paths, the type last."""
    out = []
    for m in re.finditer(r"(?m)^[ \t]*(?:unsafe\s+)?impl\b", text):
        brace = text.find("{", m.end())
        semi = text.find(";", m.end())
        if brace < 0 or (0 <= semi < brace):
            continue
        header = text[m.end() : brace]
        if header.lstrip().startswith("<"):
            header = header[closing(header, header.find("<"), "<", ">") :]
        header = re.split(r"\bwhere\b", header)[0]
        parts = re.split(r"\bfor\b", header)
        paths = []
        for part in parts:
            m2 = re.search(rf"{IDENT}(?:\s*::\s*{IDENT})*", re.sub(r"\b(?:dyn|mut|const)\b|&|'\w+", " ", part))
            if m2:
                paths.append(re.split(r"\s*::\s*", m2.group(0)))
        out.append(paths)
    return out


def path_roots(repo: Path) -> tuple[list[tuple[Path, str]], Path, list[Path]]:
    """The gate's test files (each with its blanked text), the crate's source, and the files every module's
    expansion or build reads (the derive crate, the build script)."""
    tests = repo / "native-theme/tests"
    gate = tests / "facts_gate.rs"
    try:
        raw = gate.read_text(encoding="utf-8")
    except OSError as error:
        raise InputError(f"{gate}: {error}") from error
    files = [gate] + [tests / path for path in re.findall(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]', raw)]
    derive = sorted((repo / "native-theme-derive/src").rglob("*.rs"))
    build = repo / "native-theme/build.rs"
    return [(f, f.read_text(encoding="utf-8")) for f in files if f.is_file()], repo / "native-theme/src", derive + (
        [build] if build.is_file() else []
    )


@dataclass
class Finding:
    file: Path
    line: int
    text: str

    def render(self, repo: Path) -> str:
        try:
            shown = self.file.relative_to(repo)
        except ValueError:
            shown = self.file
        return f"{shown}:{self.line}: `{self.text}`: {WHY}"


def check(repo: Path, cuts: list[tuple[str, str, str, str]] = CUTS) -> tuple[list[Finding], list[str]]:
    tests, src, always = path_roots(repo)
    crate = Crate(src)
    reach = Reach(crate)
    reach.add(crate.root, "the crate root")
    findings: list[Finding] = []

    cut_used: set[tuple[str, str]] = set()

    def link(origin: Module | None, target: Module, why: str) -> None:
        edge = (origin.name() if origin else "", target.name())
        if any(edge == (a, b) for a, b, _, _ in cuts):
            cut_used.add(edge)
            return
        reach.add(target, why)

    def follow(origin: Module | None, text: str, uses: list[Use], where: str) -> None:
        for use in uses:
            for target in reach.resolve(origin, use.path):
                link(origin, target, f"{where} uses {'::'.join(use.path)}")
        for path in references(text):
            for target in reach.resolve(origin, path):
                link(origin, target, f"{where} names {'::'.join(path)}")
        for name in macro_calls(text):
            for definer in crate.macro_definers(name):
                reach.add(definer, f"{where} invokes {name}!")

    for file, raw in tests:
        text = without_test_items(blank(raw))
        uses = []
        for m in re.finditer(r"\buse\s+", text):
            end = text.find(";", m.end())
            uses.extend(Use(p, n, False, m.start()) for p, n in parse_use_tree(text[m.end() : end]))
        follow(None, text, uses, file.name)
        findings.extend(Finding(file, line_of(text, c.offset), raw[c.offset : c.end + 1]) for c in cfgs(text) if not c.allowed)
    while True:
        while reach.queue:
            module = reach.queue.pop()
            own = module.text
            for child in module.children.values():
                if child.file == module.file:
                    own = own.replace(child.text, " " * len(child.text), 1)
            # The crate root's re-exports are its interface, no code of it: a path through one follows it.
            uses = [use for use in module.uses if not (module is crate.root and use.public)]
            follow(module, own, uses, module.name())
        grew = False
        for module in crate.modules:
            if reach.is_reached(module):
                continue
            for paths in impl_names(module.text):
                hit = [(p, t) for p in paths[-1:] for t in reach.resolve(module, p) if reach.is_reached(t) and t is not module]
                if hit:
                    reach.add(module, f"its impl is of {'::'.join(hit[0][0])} of {hit[0][1].name()}")
                    grew = True
                    break
        if not grew:
            break
    on_path = [module for module in crate.modules if reach.is_reached(module)]
    for source, target, function, why in cuts:
        shown = f"the cut {source} -> {target} ({function}: {why})"
        if (source, target) not in cut_used:
            findings.append(Finding(Path(__file__), 1, f"{shown} is stale: {source} is off the path or names {target} no more"))
            continue
        for module in on_path:
            if module.name() != source and re.search(rf"\b{function}\b", module.text):
                findings.append(Finding(module.file, line_of(module.text, re.search(rf"\b{function}\b", module.text).start()),
                                        f"{function}, which {shown} assumes nothing on the path calls"))
    for module in on_path:
        own = module.text
        for child in module.children.values():
            if child.file == module.file:
                own = own.replace(child.text, " " * len(child.text), 1)
        for c in cfgs(own):
            if c.allowed:
                continue
            start, _ = item_after(own, c.end + 1)
            declared = re.match(rf"(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+({IDENT})", own[start:])
            if declared and declared.group(1) in module.children and not reach.is_reached(
                module.children[declared.group(1)]
            ):
                continue
            at = module.start + c.offset
            findings.append(Finding(module.file, line_of(module.file_raw, at), module.file_raw[at : module.start + c.end + 1]))
        # The declarations that bring it in: its own and its ancestors', wherever they are.
        ancestor = module
        while ancestor.parent is not None and ancestor.decl_file is not None:
            if not reach.is_reached(ancestor.parent):
                start, end = ancestor.decl_attrs
                for c in cfgs(blank(ancestor.decl_raw[start:end])):
                    if not c.allowed:
                        at = start + c.offset
                        findings.append(Finding(ancestor.decl_file, line_of(ancestor.decl_raw, at), ancestor.decl_raw[at : start + c.end + 1]))
            ancestor = ancestor.parent
    for module in crate.modules:
        if reach.is_reached(module):
            continue
        for use in module.uses:
            if (id(module), use.offset) not in reach.through:
                continue
            window = module.text[max(0, use.offset - 400) : use.offset]
            attrs = re.search(r"((?:#\[[^\]]*\]\s*)*)(?:pub(?:\s*\([^)]*\))?\s*)?$", window)
            for c in cfgs(attrs.group(1) if attrs else ""):
                if not c.allowed:
                    findings.append(Finding(module.file, line_of(module.file_raw, module.start + use.offset), c.text + " (on a re-export)"))
    for file in always:
        try:
            raw = file.read_text(encoding="utf-8")
        except OSError as error:
            raise InputError(f"{file}: {error}") from error
        text = without_test_items(blank(raw))
        findings.extend(Finding(file, line_of(text, c.offset), raw[c.offset : c.end + 1]) for c in cfgs(text) if not c.allowed)
        for m in re.finditer(r"CARGO_(?:FEATURE|CFG)_", raw):
            findings.append(Finding(file, line_of(raw, m.start()), m.group(0)))
    unique = {(f.file, f.line, f.text): f for f in findings}
    names = sorted(f"{module.name()} ({reach.why[id(module)]})" for module in on_path)
    return [unique[k] for k in sorted(unique, key=lambda k: (str(k[0]), k[1]))], names


# --- self-test ----------------------------------------------------------------------------------------------------


def write_tree(base: Path, files: dict[str, str]) -> None:
    for name, text in files.items():
        path = base / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")


BASE = {
    "native-theme/tests/facts_gate.rs": (
        '#[path = "facts_gate/run.rs"]\nmod run;\nuse native_theme::gate::Preset;\n'
        "#[test]\nfn facts_gate() { let _ = native_theme::Layer::new(); }\n"
    ),
    "native-theme/tests/facts_gate/run.rs": "use native_theme::{Px, gate::When};\npub fn run() {}\n",
    "native-theme/src/lib.rs": (
        "mod layer;\nmod units;\npub mod gate;\nmod icons;\n"
        '#[cfg(feature = "watch")]\npub mod watch;\n'
        "#[cfg(test)]\nmod tests;\n"
        "pub use layer::Layer;\npub use units::*;\npub(crate) use icons::IconRole;\n"
        '/// ```\n/// # #[cfg(feature = "x")]\n/// ```\npub fn documented() {}\n'
    ),
    "native-theme/src/layer.rs": (
        "pub struct Layer;\nimpl Layer { pub fn new() -> Layer { Layer } }\n"
        '#[cfg_attr(not(target_os = "linux"), allow(dead_code))]\nfn reader_only() {}\n'
        'const NOTE: &str = "#[cfg(feature = \\"x\\")]";\n'
    ),
    "native-theme/src/units.rs": "pub struct Px(pub f32);\n",
    "native-theme/src/gate/mod.rs": (
        "mod tree;\npub use tree::When;\nuse crate::IconRole;\npub struct Preset;\n"
        "#[cfg(test)]\nmod tests {\n    #[cfg(feature = \"material-icons\")]\n    fn t() {}\n}\n"
        "#[cfg(all(test, target_os = \"linux\"))]\nfn linux_test() {}\n"
    ),
    "native-theme/src/gate/tree.rs": "pub struct When;\nfn f() { super::super::layer::Layer::new(); }\n",
    "native-theme/src/icons/mod.rs": (
        "mod model;\npub use model::IconRole;\n"
        '#[cfg(feature = "system-icons")]\npub fn load() {}\n'
    ),
    "native-theme/src/icons/model.rs": "pub enum IconRole { A }\n",
    "native-theme/src/watch.rs": '#[cfg(target_os = "linux")]\nfn linux() {}\n',
    "native-theme/src/tests.rs": '#[cfg(feature = "x")]\nfn t() {}\n',
    "native-theme-derive/src/lib.rs": "pub fn derive() {}\n#[cfg(test)]\nmod tests {}\n",
    "native-theme/build.rs": "fn main() {}\n",
}


def self_test() -> int:
    failures: list[str] = []

    def run(name: str, edits: dict[str, str], want: list[str], reached: list[str] | None = None,
            cuts: list[tuple[str, str, str, str]] | None = None) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            base = Path(tmp)
            write_tree(base, {**BASE, **edits})
            try:
                found, names = check(base, cuts or [])
            except InputError as error:
                failures.append(f"{name}: {error}")
                return
            got = [f"{f.file.relative_to(base) if f.file.is_relative_to(base) else f.file.name}:{f.line}" for f in found]
            if got != want:
                failures.append(f"{name}: expected {want}, got {got} ({[f.text for f in found]})")
            if reached is not None and not set(reached) <= {name.split(" (")[0] for name in names}:
                failures.append(f"{name}: expected {reached} on the path, got {names}")
            if found and WHY not in found[0].render(base):
                failures.append(f"{name}: the finding does not say why")

    run("clean: test-only, lint-only, off-path, comment and string cfgs pass", {}, [],
        ["crate", "gate", "gate::tree", "layer", "units", "icons::model"])
    run("a feature cfg on the path", {"native-theme/src/units.rs": 'pub struct Px(pub f32);\n#[cfg(feature = "x")]\nfn f() {}\n'},
        ["native-theme/src/units.rs:2"])
    run("cfg! in the code on the path", {"native-theme/src/gate/tree.rs": BASE["native-theme/src/gate/tree.rs"] + 'fn g() -> bool { cfg!(windows) }\n'},
        ["native-theme/src/gate/tree.rs:3"])
    run("a cfg_attr that is not lint-only", {"native-theme/src/icons/model.rs": '#[cfg_attr(feature = "serde", derive(Debug))]\npub enum IconRole { A }\n'},
        ["native-theme/src/icons/model.rs:1"])
    run("a module reached through super::super", {"native-theme/src/layer.rs": BASE["native-theme/src/layer.rs"] + '#[cfg(unix)]\nfn u() {}\n'},
        ["native-theme/src/layer.rs:6"])
    run("a module reached by a macro it defines", {
        "native-theme/src/lib.rs": BASE["native-theme/src/lib.rs"] + "mod key;\n",
        "native-theme/src/key.rs": 'macro_rules! key { () => {} }\n#[cfg(feature = "x")]\nfn k() {}\n',
        "native-theme/src/units.rs": "pub struct Px(pub f32);\nfn f() { key!(); }\n"},
        ["native-theme/src/key.rs:2"])
    run("an off-path module adding methods to a type on the path", {
        "native-theme/src/watch.rs": 'use crate::Layer;\nimpl Layer { fn watched() {} }\n#[cfg(target_os = "linux")]\nfn linux() {}\n'},
        ["native-theme/src/lib.rs:5", "native-theme/src/watch.rs:3"])
    run("a test file's own cfg", {"native-theme/tests/facts_gate/run.rs": BASE["native-theme/tests/facts_gate/run.rs"] + '#[cfg(windows)]\nfn w() {}\n'},
        ["native-theme/tests/facts_gate/run.rs:3"])
    run("a cfg on the declaration of a module on the path", {
        "native-theme/src/lib.rs": BASE["native-theme/src/lib.rs"].replace("mod units;", '#[cfg(feature = "u")]\nmod units;')},
        ["native-theme/src/lib.rs:2"])
    run("a cfg on a re-export a resolution goes through", {
        "native-theme/src/icons/mod.rs": 'mod model;\n#[cfg(feature = "i")]\npub use model::IconRole;\n'},
        ["native-theme/src/icons/mod.rs:3"])
    run("a feature read in the build script", {"native-theme/build.rs": 'fn main() { std::env::var("CARGO_FEATURE_X"); }\n'},
        ["native-theme/build.rs:1"])
    bundled = {
        "native-theme/src/icons/mod.rs": "mod model;\nmod bundled;\npub use model::IconRole;\n",
        "native-theme/src/icons/model.rs": "pub enum IconRole { A }\nimpl IconRole { pub fn icon_svg(self) { super::bundled::svg(); } }\n",
        "native-theme/src/icons/bundled.rs": '#[cfg(feature = "material-icons")]\npub fn svg() {}\n',
    }
    cut = [("icons::model", "icons::bundled", "icon_svg", "self-test")]
    run("an edge followed without its cut", bundled, ["native-theme/src/icons/bundled.rs:1"])
    run("an edge cut", bundled, [], cuts=cut)
    run("a cut whose function the path calls", {**bundled,
        "native-theme/src/units.rs": "pub struct Px(pub f32);\nfn f(r: crate::IconRole) { r.icon_svg(); }\n"},
        ["native-theme/src/units.rs:2"], cuts=cut)
    run("a stale cut", {}, ["check_gate_cfg.py:1"], cuts=cut)
    run("an impl of a trait on the path for a type off it", {
        "native-theme/src/layer.rs": BASE["native-theme/src/layer.rs"] + "pub trait Provider {}\n",
        "native-theme/src/watch.rs": 'struct Off;\nimpl crate::layer::Provider for Off {}\n#[cfg(target_os = "linux")]\nfn linux() {}\n'},
        [])
    run("a cfg in the derive crate", {"native-theme-derive/src/lib.rs": '#[cfg(feature = "d")]\npub fn derive() {}\n'},
        ["native-theme-derive/src/lib.rs:1"])
    if failures:
        for failure in failures:
            print(f"self-test FAIL: {failure}", file=sys.stderr)
        return 1
    print("check_gate_cfg self-test: every case passed")
    return 0


def main(argv: list[str]) -> int:
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    if args not in ([], ["--list"]):
        print(__doc__.split("\n\n")[0], file=sys.stderr)
        print("usage: check_gate_cfg.py [--list | --self-test]", file=sys.stderr)
        return 2
    repo = Path(__file__).resolve().parent.parent
    try:
        found, names = check(repo)
    except InputError as error:
        print(f"check_gate_cfg: {error}", file=sys.stderr)
        return 2
    if args == ["--list"]:
        for name in names:
            print(f"# on the path: {name}")
    for finding in found:
        print(finding.render(repo))
    print(
        f"the facts gate's path: {len(names)} modules, {len(found)} cfgs that depend on the target or the features",
        file=sys.stderr,
    )
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
