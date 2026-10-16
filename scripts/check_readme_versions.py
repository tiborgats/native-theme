#!/usr/bin/env python3
"""Check that every README depends on this workspace's release.

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

Exit 0 when every dependency names the workspace's `major.minor`, 1 when one
names another, 2 on an input it cannot read (a workspace version not of the
form major.minor.patch, a README missing, a crate's README that states no
dependency on its own crate, so the check would pass vacuously).

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
compare.
"""

import argparse
import os
import re
import sys
import tomllib

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)

WORKSPACE = "Cargo.toml"

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
    return ["README.md"] + [f"{member}/README.md" for member in members]


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
            if os.path.dirname(rel) in own:
                raise
    for directory in own:
        if f"{directory}/README.md" not in readmes:
            raise Failure(f"{WORKSPACE}: `{directory}` is no workspace member")
    findings, checked = [], 0
    for rel, text in texts.items():
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
    return minor, findings, checked


def read(path):
    try:
        with open(path, encoding="utf-8") as f:
            return f.read()
    except OSError as error:
        raise Failure(f"cannot read {path}: {error}")


def run(root):
    minor, findings, checked = check(lambda rel: read(os.path.join(root, rel)))
    print(f"workspace version {minor}.x; {checked} `native-theme*` dependencies in README `toml` blocks")
    if findings:
        print("\nREADMEs that depend on another release than the workspace's:")
        print("\n".join("  " + line for line in findings))
        return 1
    print("Every README depends on this release.")
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

    minor, findings, checked = check(own_dependency=own, read_rel=tree())
    expect("the clean tree passes", (minor, findings, checked), ("0.7", [], 4))

    # a README naming another minor fails, in every block it states one
    _, findings, _ = check(own_dependency=own, read_rel=tree(**{
        "connectors/native-theme-iced/README.md": SELF_ICED.replace('"0.7"', '"0.6"'),
    }))
    expect("a README naming 0.6 under a 0.7 workspace fails", findings, [
        "connectors/native-theme-iced/README.md:7: `native-theme-iced` is stated as 0.6, this release is 0.7",
        "connectors/native-theme-iced/README.md:20: `native-theme` is stated as 0.6, this release is 0.7",
        "connectors/native-theme-iced/README.md:21: `native-theme-iced` is stated as 0.6, this release is 0.7",
    ])
    # the bump: the workspace moves to 0.8 and every README still names 0.7
    _, findings, _ = check(own_dependency=own, read_rel=tree(**{"Cargo.toml": SELF_WORKSPACE.replace("0.7.0", "0.8.0")}))
    expect("a bump the READMEs did not follow fails", len(findings), 4)
    # a root README that states a dependency is checked too
    _, findings, _ = check(own_dependency=own, read_rel=tree(**{"README.md": '```toml\nnative-theme = "0.6"\n```\n'}))
    expect("the root README is checked", findings,
           ["README.md:2: `native-theme` is stated as 0.6, this release is 0.7"])

    fails("a workspace version that is not major.minor.patch",
          tree(**{"Cargo.toml": SELF_WORKSPACE.replace('"0.7.0"', '"1"')}))
    fails("a crate README without its own dependency",
          tree(**{"native-theme/README.md": "# native-theme\n"}))
    fails("a Quick start whose block states another crate only",
          tree(**{"connectors/native-theme-iced/README.md":
                  SELF_ICED.replace("native-theme-iced = {", "iced-other = {", 1)}))
    fails("a crate README that is missing",
          tree(**{"connectors/native-theme-iced/README.md": None}))

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
