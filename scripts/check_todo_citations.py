#!/usr/bin/env python3
"""List every preset citation in docs/todo.md that does not hold.

docs/todo.md cites a preset value by its TOML key path, which a preset edit
cannot shift: `adwaita.toml` `[light.button] active_background`. This script
lists each key-path citation that names no key of its preset at the tree it
runs on, and each preset line citation (`adwaita.toml:122`) that no commit
dates: a line number moves with every edit above it.

Run from the repository root:

    python3 scripts/check_todo_citations.py
    python3 scripts/check_todo_citations.py --self-test

It prints one line per finding, tab-separated: docs/todo.md and the line, the
citation, and what is wrong. Exit 0 when there is none, 1 when there are
some, 2 on an input it cannot read.

Rule
----
Presets: each native-theme/src/presets/*.toml, named by its file name
(`adwaita.toml`, with or without a directory before it).

Entries. A line starting with `#` or with a list item (`- `, indented or not)
starts an entry; a citation looks for its preset inside its entry only.

Key-path citations. A backticked `[table.path]`, `[table.path] key` or
`[table.path] key, key` whose table path starts with `light`, `dark` or
`layout` (`[light.defaults.border] color`; a dotted key walks into a table).
It resolves in the presets named since the entry's previous key-path
citation, or, where none is named since, in the ones that citation resolved
in; a citation with no preset named before it in its entry is a finding. The
table and every key must exist in each of those presets. A backtick opening
`[light`, `[dark` or `[layout` that its line does not close is a finding too:
a citation broken across lines is one the rule cannot read.

Line citations. A preset file name followed by `:N` (`adwaita.toml:122`,
`windows-11.toml:27-28`) is a finding unless its line dates it to a commit,
`at <sha>` (seven or more hex digits), as docs/todo.md's header dates its own
line numbers. A bare `:N` continuing a line citation is not looked for: the
file it continues is not on its line.
"""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path

TODO = Path("docs/todo.md")
PRESETS = Path("native-theme/src/presets")

ENTRY = re.compile(r"^(#|\s*- )")
PRESET_NAME = re.compile(r"(?<![\w-])([a-z0-9][a-z0-9-]*)\.toml\b(?::(\d+(?:-\d+)?))?")
KEY_PATH = re.compile(
    r"`\[((?:light|dark|layout)(?:\.[a-z0-9_]+)*)\]"
    r"(?: ([a-z0-9_]+(?:\.[a-z0-9_]+)*(?:, [a-z0-9_]+(?:\.[a-z0-9_]+)*)*))?`"
)
DATED = re.compile(r"\bat [0-9a-f]{7,40}\b")
SPAN = re.compile(r"`[^`]*`?")


class InputError(Exception):
    """A preset or document the rule cannot read."""


def resolves(preset: dict, table: str, key: str | None) -> bool:
    node: object = preset
    parts = table.split(".") + (key.split(".") if key else [])
    for part in parts:
        if not isinstance(node, dict) or part not in node:
            return False
        node = node[part]
    return key is not None or isinstance(node, dict)


def findings(todo: str, presets: dict[str, dict]) -> list[tuple[int, str, str]]:
    out = []
    named: list[str] = []
    context: list[str] = []
    for n, line in enumerate(todo.splitlines(), 1):
        if ENTRY.match(line):
            named, context = [], []
        dated = DATED.search(line) is not None
        for m in SPAN.finditer(line):
            if not m.group(0).endswith("`") or len(m.group(0)) == 1:
                if re.match(r"`\[(light|dark|layout)\b", m.group(0)):
                    out.append((n, m.group(0), "a key-path citation its line does not close"))
        tokens = sorted(
            [(m.start(), "name", m) for m in PRESET_NAME.finditer(line)]
            + [(m.start(), "key", m) for m in KEY_PATH.finditer(line)],
            key=lambda t: t[0],
        )
        for _, kind, m in tokens:
            if kind == "name":
                name = m.group(1)
                if name not in presets:
                    continue
                if name not in named:
                    named.append(name)
                if m.group(2) and not dated:
                    out.append((n, m.group(0), "a preset line citation no commit dates"))
                continue
            if named:
                context, named = named, []
            cite = m.group(0)
            if not context:
                out.append((n, cite, "no preset named before it in its entry"))
                continue
            keys = m.group(2).split(", ") if m.group(2) else [None]
            for preset in context:
                for key in keys:
                    if not resolves(presets[preset], m.group(1), key):
                        what = f"[{m.group(1)}]" + (f" {key}" if key else "")
                        out.append((n, cite, f"{preset}.toml has no {what}"))
    return out


def load_presets(root: Path) -> dict[str, dict]:
    out = {}
    for path in sorted(root.glob("*.toml")):
        try:
            out[path.stem] = tomllib.loads(path.read_text(encoding="utf-8"))
        except tomllib.TOMLDecodeError as err:
            raise InputError(f"{path}: {err}") from err
    if not out:
        raise InputError(f"{root}: no presets")
    return out


def self_test() -> int:
    presets = {
        "adwaita": tomllib.loads(
            '[light.button]\nactive_background = "#c8c8c8"\n'
            '[light.button.border]\ncolor = "#000000"\n'
            '[dark.button]\nactive_background = "#2e2e32"\n'
            "[layout]\nwidget_gap_px = 6.0\n"
        ),
        "kde-breeze": tomllib.loads('[light.button]\nhover_background = "#fcfcfc"\n'),
    }
    todo = """# Heading
- [ ] `adwaita.toml` `[light.button] active_background`, `[dark.button] active_background`
      and `[light.button] border.color`, `[layout] widget_gap_px`, `[light.button]`
      then `[light.button] hover_background`
- [ ] `native-theme/src/presets/adwaita.toml:122`, and
      `kde-breeze.toml:8` at 2ca74471; `adwaita.toml`, `kde-breeze.toml`
      (`[light.button] active_background, hover_background`)
      `Cargo.toml:3`, `macos-sonoma.toml:79`, `[light.slider]` in `[Colors:Button]`
- [ ] `[dark.button] active_background` with no preset before it
- [ ] `adwaita.toml` `[light.button]
      active_background`
"""
    want = [
        (4, "`[light.button] hover_background`", "adwaita.toml has no [light.button] hover_background"),
        (5, "adwaita.toml:122", "a preset line citation no commit dates"),
        (7, "`[light.button] active_background, hover_background`",
         "adwaita.toml has no [light.button] hover_background"),
        (7, "`[light.button] active_background, hover_background`",
         "kde-breeze.toml has no [light.button] active_background"),
        (8, "`[light.slider]`", "adwaita.toml has no [light.slider]"),
        (8, "`[light.slider]`", "kde-breeze.toml has no [light.slider]"),
        (9, "`[dark.button] active_background`", "no preset named before it in its entry"),
        (10, "`[light.button]", "a key-path citation its line does not close"),
    ]
    failures = []
    got = findings(todo, presets)
    if got != want:
        failures.append(f"rule: got {got}, want {want}")
    if findings("- [ ] `adwaita.toml` at 1234567, `adwaita.toml:1` at 1234567\n", presets):
        failures.append("a dated line citation: reported")
    if not findings("- [ ] `adwaita.toml:1` at HEAD\n", presets):
        failures.append("an undated line citation: accepted")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def main(argv: list[str]) -> int:
    if argv[1:] == ["--self-test"]:
        return self_test()
    if argv[1:]:
        print(__doc__, file=sys.stderr)
        return 2
    try:
        out = findings(TODO.read_text(encoding="utf-8"), load_presets(PRESETS))
    except (OSError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for line, cite, what in out:
        print(f"{TODO}:{line}\t{cite}\t{what}")
    print(f"{len(out)} preset citations that do not hold", file=sys.stderr)
    return 1 if out else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
