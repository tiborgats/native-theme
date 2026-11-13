#!/usr/bin/env python3
"""List every bundled preset's definition comment that does not hold.

A value the specification defines (S11, S17), not a fact, carries the
defining section as its comment, `# S9.4` or `# S4.5`, in place of a fact's
citation: a trailing comment on its line or a comment line straight above
it. DEFINITIONS, the closed list, maps each section to the TOML path it
defines, the value, and the presets it is stated on: S9.4's
`tokens.text.font_dpi = 72` on `macos-*` and `ios-*` (ADD-A125), S4.5's
`tokens.icons.animated_visuals = {}` on every preset but `windows-11`
(ADD-A131). The comment is accepted for that key, with that value, on those
presets alone: on another key, another value or another preset it is a
finding. S10's `# S10` marks a keyed map stated empty ("a keyed namespace
declares only the entries a base states, a map with none stated empty",
ADD-A147): EMPTIED_MAPS lists the maps whose entries the units file stages
(`scripts/widget_units.toml`'s `[staged]` paths with `[kept]` entries), and
the comment is accepted on one of them with the value `{}`, on any preset.
A comment naming another section is no definition comment.

The presets are the bundled ones, `native-theme/src/presets/*.toml`.

Run from the repository root:

    python3 scripts/check_preset_citations.py
    python3 scripts/check_preset_citations.py --self-test

It prints one line per finding, tab-separated: the preset and the comment's
line, the key path, and what is wrong; then, on stderr, the count. Exit 0
when there is none, 1 when there are some, 2 on an input it cannot read: a
folder without presets, or a file that is not UTF-8 or TOML.
"""

from __future__ import annotations

import re
import sys
import tempfile
import tomllib
from pathlib import Path

PRESETS = Path("native-theme/src/presets")
# the values the specification defines (S11, S17): section -> (TOML path, value, the presets stating it)
DEFINITIONS = {
    "S9.4": ("tokens.text.font_dpi", 72, lambda p: p.startswith(("macos-", "ios-"))),
    "S4.5": ("tokens.icons.animated_visuals", {}, lambda p: p != "windows-11"),
}
# S10 (ADD-A147): (the keyed maps a preset states empty where no base states an entry, the value)
EMPTIED_MAPS = (("colors.light.keyed", "colors.dark.keyed", "tokens.text.keyed", "tokens.icons.sizes.keyed",
                 "tokens.elevation", "tokens.params"), {})
SECTIONS = (*DEFINITIONS, "S10")
DEFINITION_COMMENT = re.compile(r"^#\s*(S\d+(?:\.\d+)*)\s*$")
HEADER = re.compile(r"\[([A-Za-z0-9_.-]+)\]")
VALUE = re.compile(r"([A-Za-z0-9_.]+)\s*=")


class InputError(Exception):
    """A preset the rule cannot read."""



def definition_findings(name: str, text: str) -> list:
    """The definition comments (`# S9.4`, `# S4.5`: DEFINITIONS) of a preset that are not on their key, with their
    value, on a preset of theirs; (preset:line, key path, 0, what) each."""
    try:
        data = tomllib.loads(text)
    except tomllib.TOMLDecodeError as err:
        raise InputError(f"{name}: {err}") from err
    preset = Path(name).stem
    lines = text.splitlines()
    table: list[str] = []
    pending: tuple[int, str] | None = None   # a comment line's section, waiting for the key below it
    out = []

    def judge(n, section, key_path):
        if section == "S10":
            maps, empty = EMPTIED_MAPS
            if key_path not in maps:
                out.append((f"{name}:{n}", key_path or "(no key)", 0,
                            "`# S10` marks a keyed map stated empty, not this key"))
                return
            path, value, on = key_path, empty, lambda p: True
        else:
            path, value, on = DEFINITIONS[section]
        if key_path != path:
            out.append((f"{name}:{n}", key_path or "(no key)", 0,
                        f"`# {section}` defines `{path}`, not this key"))
            return
        if not on(preset):
            out.append((f"{name}:{n}", key_path, 0, f"`# {section}` is not accepted on {preset}"))
        got: object = data
        for seg in path.split("."):
            got = got.get(seg) if isinstance(got, dict) else None
        if got != value:
            out.append((f"{name}:{n}", key_path, 0, f"`# {section}` defines {value!r}, not {got!r}"))

    for n, line in enumerate(lines, 1):
        stripped = line.strip()
        if stripped.startswith("#"):
            m = DEFINITION_COMMENT.match(stripped)
            if m and m.group(1) in SECTIONS:
                pending = (n, m.group(1))
            continue
        if not stripped:
            continue
        header = HEADER.match(stripped)
        if header and stripped.startswith("["):
            table = header.group(1).split(".")
            if pending:
                judge(pending[0], pending[1], None)
                pending = None
            continue
        key = VALUE.match(stripped)
        key_path = ".".join(table + key.group(1).split(".")) if key else None
        if pending:
            judge(pending[0], pending[1], key_path)
            pending = None
        trailing = re.search(r"\s#\s*(S\d+(?:\.\d+)*)\s*$", line)
        if trailing and trailing.group(1) in SECTIONS:
            judge(n, trailing.group(1), key_path)
    if pending:
        judge(pending[0], pending[1], None)
    return out


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError as err:
        raise InputError(f"{path}: not UTF-8 ({err})") from err


def load_presets(root: Path) -> dict[str, str]:
    out = {path.name: read_text(path) for path in sorted(root.glob("*.toml"))}
    if not out:
        raise InputError(f"{root}: no presets")
    return out


def self_test() -> int:
    failures = []
    # values the specification defines (S11, S17; ADD-A125, ADD-A131): accepted on their key, value and presets alone
    for label, preset, text, want in (
        ("S4.5 on its key, trailing", "adwaita.toml", "[tokens.icons]\nanimated_visuals = {}  # S4.5\n", []),
        ("S4.5 above its key", "kde-breeze.toml", "[tokens]\n# S4.5\nicons.animated_visuals = {}\n", []),
        ("S9.4 on an Apple preset", "ios-26.toml", "[tokens.text]\nfont_dpi = 72  # S9.4\n", []),
        ("S4.5 on windows-11", "windows-11.toml", "[tokens.icons]\nanimated_visuals = {}  # S4.5\n",
         [("windows-11.toml:2", "tokens.icons.animated_visuals", 0, "`# S4.5` is not accepted on windows-11")]),
        ("S4.5 on another key", "adwaita.toml", "[tokens.icons]\ntheme = \"Adwaita\"  # S4.5\n",
         [("adwaita.toml:2", "tokens.icons.theme", 0, "`# S4.5` defines `tokens.icons.animated_visuals`, not this "
                                                      "key")]),
        ("S4.5 on another value", "adwaita.toml",
         "[tokens.icons]\n# S4.5\nanimated_visuals = { accept = { duration_ms = 1 } }\n",
         [("adwaita.toml:2", "tokens.icons.animated_visuals", 0, "`# S4.5` defines {}, not {'accept': "
                                                                  "{'duration_ms': 1}}")]),
        ("S9.4 off the Apple presets", "adwaita.toml", "[tokens.text]\nfont_dpi = 72  # S9.4\n",
         [("adwaita.toml:2", "tokens.text.font_dpi", 0, "`# S9.4` is not accepted on adwaita")]),
        ("a section above no key", "adwaita.toml", "# S4.5\n[tokens.icons]\n",
         [("adwaita.toml:1", "(no key)", 0, "`# S4.5` defines `tokens.icons.animated_visuals`, not this key")]),
        ("another section, no definition comment", "adwaita.toml", "[tokens.text]\nfont_dpi = 96  # S12\n", []),
        # a keyed map stated empty (S10, ADD-A147): on one of EMPTIED_MAPS with `{}`, on any preset
        ("S10 on a keyed map", "ios-13.toml", "[colors.light]\n# S10\nkeyed = {}\n[tokens]\nparams = {}  # S10\n", []),
        ("S10 on another key", "adwaita.toml", "[tokens.icons]\n# S10\ntheme = {}\n",
         [("adwaita.toml:2", "tokens.icons.theme", 0, "`# S10` marks a keyed map stated empty, not this key")]),
        ("S10 on a map with an entry", "adwaita.toml", "[tokens]\nelevation = { level0 = \"absent\" }  # S10\n",
         [("adwaita.toml:2", "tokens.elevation", 0, "`# S10` defines {}, not {'level0': 'absent'}")])):
        got = definition_findings(preset, text)
        if got != want:
            failures.append(f"definitions, {label}: got {got}, want {want}")
    with tempfile.TemporaryDirectory() as root:
        (Path(root) / "p.toml").write_bytes(b"# \xff\n")
        try:
            load_presets(Path(root))
            failures.append("a preset that is not UTF-8: accepted")
        except InputError:
            pass
    with tempfile.TemporaryDirectory() as root:
        (Path(root) / "p.toml").write_bytes(b"# \xff\n")
        try:
            load_presets(Path(root))
            failures.append("a preset that is not UTF-8: accepted")
        except InputError:
            pass
    with tempfile.TemporaryDirectory() as root:
        try:
            load_presets(Path(root))
            failures.append("a folder without presets: accepted")
        except InputError:
            pass
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
    out = []
    try:
        for name, text in load_presets(PRESETS).items():
            out += definition_findings(name, text)
    except (OSError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for here, key_path, _, what in out:
        print(f"{here}\t{key_path}\t{what}")
    print(f"{len(out)} definition comments that do not hold", file=sys.stderr)
    return 1 if out else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
