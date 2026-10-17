#!/usr/bin/env python3
"""Check that every README depends on this workspace's release, and on every crate its code names.

A README's `toml` blocks are what an application copies into its manifest, so
a `native-theme = "0.6"` left behind by a version bump sends every reader to
the previous release. This script reads the workspace manifest's
`[workspace.package] version`, which every crate takes
(`version.workspace = true`), and requires every `native-theme` and
`native-theme-*` dependency a README states in a `toml` block to name its
`major.minor`: the requirement an application writes for this release.

Run from the repository root:

    python3 scripts/check_readme_versions.py
    python3 scripts/check_readme_versions.py --self-test

Exit 0 when every dependency names the workspace's `major.minor` and the code
names only stated crates, 1 when one names another or the code names a crate
no block states (below), 2 on an input it cannot read (a workspace version not of the
form major.minor.patch, the root README or a crate's missing, a crate's
README that states no dependency on its own crate, so the check would pass
vacuously).

The release bump moves the READMEs
----------------------------------
The rule compares against the version the workspace states, not the one it
will state: until the release commit bumps `[workspace.package] version`,
the READMEs name the current minor, and the bump fails this check until the
same commit moves every README to the new one. One commit moves them all, and
the check holds them there.

What is read
------------
The root README and the README of every workspace member that has one. In each,
a fenced block whose info string is `toml` is read line by line; a line
`<name> = "<version>"`, `<name> = { version = "<version>", .. }` or
`<key> = { package = "<name>", version = "<version>", .. }` is a dependency
on `<name>`, and one on `native-theme` or `native-theme-*` is checked. A
dependency without a version of its own (`{ workspace = true }`, `{ path = .. }`)
states none and is not checked. Other code blocks, and text outside a block,
are not dependencies.

Each crate an application depends on directly (`OWN_DEPENDENCY`) must state
its own dependency: in its `## Quick start` section where the README has one
with a `toml` block, anywhere in a `toml` block otherwise (the core README
installs with `cargo add` and states its features' block under *Feature
flags*). A README that lost its block would otherwise pass with nothing to
compare. The root README must exist too; a member without one (a tool that
is not published) is skipped.

The crates a README's code names
--------------------------------
A README's Rust blocks compile as doctests of their crate, which sees that
crate's own dependencies: a block naming `gpui::` passes there through the
connector's `gpui` dependency while an application that copied the Quick
start has no crate of that name. So the second rule reads the same READMEs'
Rust blocks (a fence whose info string is empty or starts with `rust`, the
doctest's hidden `# ` lines included, comments and string literals left out)
and requires every crate a path names first -- the root of a `use` tree, and
the first segment of any other `name::` path whose name the block does not
bind itself (a `use` binding, an alias, a `{self}` import) -- to be a
dependency the README states before that block: a key of a `[dependencies]`,
`[build-dependencies]` or `[dev-dependencies]` table in a `toml` block above
it, or a crate a `cargo add` line of a shell block above it adds (the core
README installs that way), with `-` read as `_`. A reader copies the Quick start and the blocks in
order, so a crate a later section states is one the earlier code did not tell
them to add. `std`, `core`, `alloc`, `crate`, `self`,
`super` and the primitive types need none; a name beginning with an upper-case
letter is a type, not a crate, and is not checked. A finding exits 1.

The rule reads the code as text, not as a parse, and has three blind spots,
none of which occurs in a README today: a lowercase module a glob import
(`use x::*;`) brings in reads as a crate (a false finding), as does a
lowercase module the block declares itself (`mod m { .. }` then `m::f`); and
a path inside a macro body is read as any other path, though the macro may
not expand it as one.
"""

import argparse
import os
import re
import sys
import tomllib

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)

WORKSPACE = "Cargo.toml"
ROOT_README = "README.md"

# The crates an application names in its own manifest, by directory: each
# README states the dependency on its crate. native-theme-derive's README
# says it is internal to native-theme and states none.
OWN_DEPENDENCY = (
    ("native-theme", "native-theme"),
    ("native-theme-build", "native-theme-build"),
    ("connectors/native-theme-iced", "native-theme-iced"),
    ("connectors/native-theme-gpui", "native-theme-gpui"),
    ("connectors/native-theme-egui", "native-theme-egui"),
    ("connectors/native-theme-egui-widgets", "native-theme-egui-widgets"),
)

QUICK_START = "Quick start"


class Failure(Exception):
    """An input the script cannot read."""


def workspace_minor(text, label):
    """The `major.minor` of the workspace manifest's `[workspace.package] version`."""
    try:
        data = tomllib.loads(text)
    except tomllib.TOMLDecodeError as error:
        raise Failure(f"{label}: {error}")
    version = data.get("workspace", {}).get("package", {}).get("version")
    m = re.fullmatch(r"(\d+)\.(\d+)\.\d+(?:[-+].*)?", version if isinstance(version, str) else "")
    if m is None:
        raise Failure(f"{label}: `[workspace.package] version` is {version!r}, "
                      "not of the form major.minor.patch")
    return f"{m.group(1)}.{m.group(2)}"


def workspace_readmes(text, label):
    """The README paths to read, relative to the root: the root's and each member's."""
    try:
        members = tomllib.loads(text).get("workspace", {}).get("members", [])
    except tomllib.TOMLDecodeError as error:
        raise Failure(f"{label}: {error}")
    if not members:
        raise Failure(f"{label}: `[workspace] members` lists no crate")
    return [ROOT_README] + [f"{member}/README.md" for member in members]


DEP_RE = re.compile(r"\s*([A-Za-z0-9_-]+)\s*=\s*(.*)")
QUOTED_RE = re.compile(r'"([^"]*)"')


def dependency_on(line):
    """(package, version) the dependency line states, or None.

    A line naming no version of its own (`{ workspace = true }`) is None.
    """
    m = DEP_RE.fullmatch(line.split("#", 1)[0].rstrip())
    if m is None:
        return None
    key, value = m.group(1), m.group(2).strip()
    if value.startswith('"'):
        version = QUOTED_RE.match(value)
        return (key, version.group(1)) if version else None
    if value.startswith("{"):
        version = re.search(r'\bversion\s*=\s*"([^"]*)"', value)
        if version is None:
            return None
        package = re.search(r'\bpackage\s*=\s*"([^"]*)"', value)
        return (package.group(1) if package else key, version.group(1))
    return None


def is_workspace_crate(package):
    return package == "native-theme" or package.startswith("native-theme-")


def read_blocks(text):
    """(dependencies, sections) of a README's `toml` blocks: the dependencies
    on this workspace's crates as [(line, section, package, version)], and the
    `## ` headings a `toml` block stands under."""
    found, sections, heading, block = [], set(), "", None
    for number, line in enumerate(text.splitlines(), 1):
        fence = line.lstrip()
        if fence.startswith("```"):
            block = None if block is not None else fence[3:].strip() == "toml"
            if block:
                sections.add(heading)
            continue
        if block is None:
            if line.startswith("## "):
                heading = line[3:].strip()
        elif block:
            dep = dependency_on(line)
            if dep and is_workspace_crate(dep[0]):
                found.append((number, heading, dep[0], dep[1]))
    return found, sections


TABLE_RE = re.compile(r"\[\s*(?:[^\]]*\.)?(?:dev-|build-)?dependencies\s*\]")


SHELL_INFO = {"sh", "bash", "shell", "console", "zsh", "fish"}
# `cargo add` options that take a value: the token after one is not a crate.
CARGO_ADD_VALUED = {"-F", "--features", "--rename", "-p", "--package", "--path",
                    "--git", "--branch", "--tag", "--rev", "--registry", "--target",
                    "--manifest-path", "--lockfile-path", "--config", "-Z"}


def cargo_add_crates(line):
    """The crate names, as code names them, a `cargo add` command line adds."""
    tokens = line.split()
    if tokens[:1] == ["$"]:
        tokens = tokens[1:]
    if tokens[:2] != ["cargo", "add"]:
        return []
    crates, rename, i = [], None, 2
    while i < len(tokens):
        token = tokens[i]
        flag, eq, value = token.partition("=")
        if token.startswith("-"):
            if flag in CARGO_ADD_VALUED and not eq:
                value = tokens[i + 1] if i + 1 < len(tokens) else ""
                i += 1
            if flag == "--rename":
                rename = value
        else:
            crates.append(token.split("@", 1)[0])
        i += 1
    if rename and len(crates) == 1:
        crates = [rename]
    return [crate.replace("-", "_") for crate in crates]


def stated_crates(text):
    """{crate: first line} of the crates a README states a dependency on:
    each key of a dependency table in a `toml` block, and each crate a
    `cargo add` line of a shell block adds, `-` read as `_`, as code names
    the crate, with the line that first states it."""
    names, block, table = {}, None, False
    for number, line in enumerate(text.splitlines(), 1):
        fence = line.lstrip()
        if fence.startswith("```"):
            if block is not None:
                block = None
            else:
                info = fence[3:].strip()
                block = "toml" if info == "toml" else "shell" if info in SHELL_INFO else ""
            table = False
            continue
        if block == "shell":
            for crate in cargo_add_crates(line):
                names.setdefault(crate, number)
            continue
        if block != "toml":
            continue
        stripped = line.split("#", 1)[0].strip()
        if stripped.startswith("["):
            table = TABLE_RE.fullmatch(stripped) is not None
            continue
        if table:
            m = DEP_RE.fullmatch(stripped)
            if m:
                names.setdefault(m.group(1).replace("-", "_"), number)
    return names


def rust_blocks(text):
    """[(first line number, [code lines])] of a README's Rust blocks: a fence
    whose info string is empty or starts with `rust`, as rustdoc reads it."""
    blocks, current, start = [], None, 0
    for number, line in enumerate(text.splitlines(), 1):
        fence = line.lstrip()
        if fence.startswith("```"):
            if current is None:
                info = fence[3:].strip()
                current = [] if info == "" or info.startswith("rust") else False
                start = number + 1
            else:
                if current is not False:
                    blocks.append((start, current))
                current = None
            continue
        if current is not None and current is not False:
            current.append(line)
    return blocks


NEED_NO_DEPENDENCY = {
    "std", "core", "alloc", "crate", "self", "super",
    "bool", "char", "str", "f32", "f64", "i8", "i16", "i32", "i64", "i128",
    "isize", "u8", "u16", "u32", "u64", "u128", "usize",
}
RAW_STRING_RE = re.compile(r'\br(#*)"(.*?)"\1', re.S)
STRING_RE = re.compile(r'"(?:\\.|[^"\\\n])*"')
USE_RE = re.compile(r"\buse\s+([^;]*);", re.S)
PATH_RE = re.compile(r"(?<![\w.:$])([a-z_][a-z0-9_]*)::")


def blank(match):
    """The match with every character but a newline blanked, so positions keep their lines."""
    return re.sub(r"[^\n]", " ", match.group(0))


def code_of(lines):
    """A block's code as rustdoc compiles it, with comments and string literals blanked."""
    shown = []
    for line in lines:
        stripped = line.lstrip()
        if stripped == "#" or stripped.startswith("# "):
            line = line.replace("#", "", 1)
        shown.append(line)
    code = "\n".join(shown)
    code = RAW_STRING_RE.sub(blank, code)
    code = STRING_RE.sub(blank, code)
    return re.sub(r"//[^\n]*", blank, code)


def split_top(text):
    """The comma-separated parts of a `use` group, braces nested."""
    parts, depth, part = [], 0, ""
    for ch in text:
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(part)
            part = ""
        else:
            part += ch
    parts.append(part)
    return [p.strip() for p in parts if p.strip()]


def use_bindings(tree, parent=None):
    """The names a `use` tree binds."""
    tree = " ".join(tree.split())
    group = re.fullmatch(r"((?:\w+\s*::\s*)*)\{(.*)\}", tree)
    if group:
        segments = [x.strip() for x in group.group(1).split("::") if x.strip()]
        last = segments[-1] if segments else parent
        names = set()
        for part in split_top(group.group(2)):
            names |= use_bindings(part, last)
        return names
    alias = re.fullmatch(r"(.*)\s+as\s+(\w+)", tree)
    if alias:
        return set() if alias.group(2) == "_" else {alias.group(2)}
    segments = [x.strip() for x in tree.split("::")]
    if segments[-1] == "self":
        name = segments[-2] if len(segments) > 1 else parent
        return {name} if name else set()
    return set() if segments[-1] == "*" else {segments[-1]}


def named_crates(lines):
    """[(line offset, crate)] of the crates a block's code names first."""
    code = code_of(lines)
    found, bound = [], set()
    for m in USE_RE.finditer(code):
        tree = m.group(1).strip()
        root = re.match(r"(?:::)?\s*(\w+)", tree)
        if root and not tree.startswith("{"):
            found.append((code.count("\n", 0, m.start(1)), root.group(1)))
        bound |= use_bindings(tree)
    rest = USE_RE.sub(blank, code)
    for m in PATH_RE.finditer(rest):
        if m.group(1) not in bound:
            found.append((rest.count("\n", 0, m.start(1)), m.group(1)))
    return [(offset, name) for offset, name in found
            if name not in NEED_NO_DEPENDENCY and not name[:1].isupper()]


def unstated_crates(rel, text):
    """Findings for the crates a README's code names that no `toml` block
    above that code depends on."""
    stated = stated_crates(text)
    findings = []
    for start, lines in rust_blocks(text):
        for offset, name in named_crates(lines):
            if stated.get(name, start) >= start:
                findings.append(f"{rel}:{start + offset}: the code names `{name}`, "
                                "which no `toml` block above it in the README depends on")
    return findings


def own_dependency_problem(rel, crate, deps, sections):
    """Why the README states no dependency on its own crate, or None."""
    own = [d for d in deps if d[2] == crate]
    if QUICK_START in sections:
        own = [d for d in own if d[1] == QUICK_START]
        where = f"its `## {QUICK_START}` `toml` block"
    else:
        where = "any `toml` block"
    if own:
        return None
    return f"{rel}: states no `{crate}` dependency in {where}, so the check would pass vacuously"


def check(read_rel, own_dependency=OWN_DEPENDENCY):
    """(minor, findings, checked) for the tree `read_rel` reads (paths relative
    to its root), `own_dependency` naming the READMEs that state their crate."""
    manifest = read_rel(WORKSPACE)
    minor = workspace_minor(manifest, WORKSPACE)
    readmes = workspace_readmes(manifest, WORKSPACE)
    own = dict(own_dependency)
    texts = {}
    for rel in readmes:
        try:
            texts[rel] = read_rel(rel)
        except Failure:
            # a member without a README of its own (a tool) states nothing;
            # the root's and every crate's must be there
            if rel == ROOT_README or os.path.dirname(rel) in own:
                raise
    for directory in own:
        if f"{directory}/README.md" not in readmes:
            raise Failure(f"{WORKSPACE}: `{directory}` is no workspace member")
    findings, checked, unstated = [], 0, []
    for rel, text in texts.items():
        unstated += unstated_crates(rel, text)
        deps, sections = read_blocks(text)
        directory = os.path.dirname(rel)
        if directory in own:
            problem = own_dependency_problem(rel, own[directory], deps, sections)
            if problem:
                raise Failure(problem)
        for line, _, package, version in deps:
            checked += 1
            if version != minor:
                findings.append(f"{rel}:{line}: `{package}` is stated as {version}, "
                                f"this release is {minor}")
    return minor, findings, checked, unstated


def read(path):
    try:
        with open(path, encoding="utf-8") as f:
            return f.read()
    except OSError as error:
        raise Failure(f"cannot read {path}: {error}")


def run(root):
    minor, findings, checked, unstated = check(lambda rel: read(os.path.join(root, rel)))
    print(f"workspace version {minor}.x; {checked} `native-theme*` dependencies in README `toml` blocks")
    if findings:
        print("\nREADMEs that depend on another release than the workspace's:")
        print("\n".join("  " + line for line in findings))
    if unstated:
        print("\nREADME code naming a crate the README does not depend on:")
        print("\n".join("  " + line for line in unstated))
    if findings or unstated:
        return 1
    print("Every README depends on this release, and on every crate its code names.")
    return 0


# ---------------------------------------------------------------------------
# Self-test
# ---------------------------------------------------------------------------

SELF_WORKSPACE = '''
[workspace]
members = ["native-theme", "connectors/native-theme-iced", "tools/probe"]

[workspace.package]
version = "0.7.0"
'''

SELF_CORE = '''# native-theme

```sh
cargo add native-theme
```

## Feature flags

```toml
[dependencies]
native-theme = { version = "0.7", features = ["native"] }
```
'''

SELF_ICED = '''# native-theme-iced

## Quick start

```toml
[dependencies]
native-theme-iced = { version = "0.7", features = ["native"] }   # the connector
iced = "0.14"
```

native-theme = "0.5"

```rust,ignore
let native_theme = "0.5";
```

## Runtime theme change

```toml
native-theme = "0.7"
theme = { package = "native-theme-iced", version = "0.7" }
local = { workspace = true }
```
'''


SELF_CODE = '''# native-theme-kit

## Quick start

```toml
[dependencies]
kit = "1"
native-theme-iced = { version = "0.7" }
```

```rust,no_run
use kit::svg::{self, Svg};
use native_theme_iced::{styles, native_theme as nt};
# fn f(w: &kit::Window) {
let s = styles::button();
let h = svg::Handle::from_memory(Vec::new());
let x = nt::detect::invalidate_caches();
let y = std::time::Duration::ZERO;
let z = u64::from(1u8);
// gpui::named_in_a_comment();
let q = "gpui::named_in_a_string";
let t = Theme::from_toml(r##"
gpui::named_in_a_raw_string
"##);
# }
```

```toml
other = "1"
```

```rust
use other::Thing;
# let w: gpui::Window;
let b = gpui_component::button::Button::new("ok");
```

```text
gpui::named_in_a_text_block
```
'''


SELF_LATE = '''# late

```rust
let a = later::f();
```

```toml
[dependencies]
later = "1"
```

```rust
let b = later::g();
```
'''


def self_test():
    failures = []

    def expect(label, got, want):
        if got != want:
            failures.append(f"{label}: got {got!r}, want {want!r}")

    own = (("native-theme", "native-theme"),
           ("connectors/native-theme-iced", "native-theme-iced"))

    def tree(**overrides):
        files = {
            "Cargo.toml": SELF_WORKSPACE,
            "README.md": "# root\n",
            "native-theme/README.md": SELF_CORE,
            "connectors/native-theme-iced/README.md": SELF_ICED,
        }
        files.update(overrides)

        def read_rel(rel):
            if files.get(rel) is None:
                raise Failure(f"cannot read {rel}")
            return files[rel]
        return read_rel

    def fails(label, read_rel):
        try:
            check(read_rel, own)
            failures.append(f"{label}: no error")
        except Failure:
            pass

    for line, want in (
        ('native-theme = "0.7"', ("native-theme", "0.7")),
        ('native-theme-iced = { version = "0.7", features = ["native"] }',
         ("native-theme-iced", "0.7")),
        ('gpui = { package = "gpui-pre", version = "0.3.5" }', ("gpui-pre", "0.3.5")),
        ('native-theme = { workspace = true }', None),
        ('beta = "0.6.4"        # or gamma directly', ("beta", "0.6.4")),
        ('# native-theme = "0.1"', None),
        ('[dependencies]', None),
    ):
        expect(f"dependency_on({line})", dependency_on(line), want)

    expect("read_blocks", read_blocks(SELF_ICED), ([
        (7, "Quick start", "native-theme-iced", "0.7"),
        (20, "Runtime theme change", "native-theme", "0.7"),
        (21, "Runtime theme change", "native-theme-iced", "0.7"),
    ], {"Quick start", "Runtime theme change"}))
    expect("workspace_minor", workspace_minor(SELF_WORKSPACE, "w"), "0.7")
    expect("workspace_minor of a pre-release",
           workspace_minor('[workspace.package]\nversion = "1.2.0-rc.1"\n', "w"), "1.2")

    minor, findings, checked, unstated = check(own_dependency=own, read_rel=tree())
    expect("the clean tree passes", (minor, findings, checked, unstated), ("0.7", [], 4, []))

    # a README naming another minor fails, in every block it states one
    _, findings, _, _ = check(own_dependency=own, read_rel=tree(**{
        "connectors/native-theme-iced/README.md": SELF_ICED.replace('"0.7"', '"0.6"'),
    }))
    expect("a README naming 0.6 under a 0.7 workspace fails", findings, [
        "connectors/native-theme-iced/README.md:7: `native-theme-iced` is stated as 0.6, this release is 0.7",
        "connectors/native-theme-iced/README.md:20: `native-theme` is stated as 0.6, this release is 0.7",
        "connectors/native-theme-iced/README.md:21: `native-theme-iced` is stated as 0.6, this release is 0.7",
    ])
    # the bump: the workspace moves to 0.8 and every README still names 0.7
    _, findings, _, _ = check(own_dependency=own, read_rel=tree(**{"Cargo.toml": SELF_WORKSPACE.replace("0.7.0", "0.8.0")}))
    expect("a bump the READMEs did not follow fails", len(findings), 4)
    # a root README that states a dependency is checked too
    _, findings, _, _ = check(own_dependency=own, read_rel=tree(**{"README.md": '```toml\nnative-theme = "0.6"\n```\n'}))
    expect("the root README is checked", findings,
           ["README.md:2: `native-theme` is stated as 0.6, this release is 0.7"])

    # the crates a README's code names: its own toml tables', std and the
    # primitives, its own bindings; a hidden line counts; a comment, a string,
    # a non-Rust block and a dependency outside a table do not
    expect("use_bindings", use_bindings("a::b::{c, d::{e, f as g}, self, h as _}"),
           {"c", "e", "g", "b"})
    expect("stated_crates", stated_crates(SELF_CODE), {"kit": 7, "native_theme_iced": 8})
    expect("unstated_crates", unstated_crates("r.md", SELF_CODE), [
        "r.md:33: the code names `other`, which no `toml` block above it in the README depends on",
        "r.md:34: the code names `gpui`, which no `toml` block above it in the README depends on",
        "r.md:35: the code names `gpui_component`, which no `toml` block above it in the README depends on",
    ])
    for line, want in (
        ("cargo add native-theme", ["native_theme"]),
        ("$ cargo add native-theme@0.7 --features native", ["native_theme"]),
        ("cargo add -F watch iced iced_aw", ["iced", "iced_aw"]),
        ("cargo add gpui-pre --rename gpui", ["gpui"]),
        ("cargo build", []),
    ):
        expect(f"cargo_add_crates({line})", cargo_add_crates(line), want)
    expect("a `cargo add` line states its crate",
           stated_crates("```sh\ncargo add native-theme\n```\n"), {"native_theme": 2})
    # a crate stated only in a block below the code that first names it: a
    # reader who stops at that code has not been told to depend on it
    expect("a crate stated only below its first use", unstated_crates("r.md", SELF_LATE), [
        "r.md:4: the code names `later`, which no `toml` block above it in the README depends on",
    ])
    _, _, _, unstated = check(own_dependency=own, read_rel=tree(**{
        "README.md": "# root\n\n```rust\nlet w = gpui::div();\n```\n",
    }))
    expect("the root README's code is checked", unstated,
           ["README.md:4: the code names `gpui`, which no `toml` block above it in the README depends on"])

    fails("a workspace version that is not major.minor.patch",
          tree(**{"Cargo.toml": SELF_WORKSPACE.replace('"0.7.0"', '"1"')}))
    fails("a crate README without its own dependency",
          tree(**{"native-theme/README.md": "# native-theme\n"}))
    fails("a Quick start whose block states another crate only",
          tree(**{"connectors/native-theme-iced/README.md":
                  SELF_ICED.replace("native-theme-iced = {", "iced-other = {", 1)}))
    fails("a crate README that is missing",
          tree(**{"connectors/native-theme-iced/README.md": None}))
    fails("a root README that is missing", tree(**{"README.md": None}))

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
                        help="repository root to read instead of this one")
    parser.add_argument("--self-test", action="store_true",
                        help="run the built-in scenarios instead of the check")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    return run(args.root)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Failure as error:
        print(f"check_readme_versions: {error}", file=sys.stderr)
        sys.exit(2)
