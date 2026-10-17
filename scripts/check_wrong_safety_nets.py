#!/usr/bin/env python3
"""Check docs/inheritance-rules.toml's [wrong_safety_nets] against its rules.

[wrong_safety_nets] records the inherited values known to be wrong on some
platform, and a [uniform] rule that gives one says so with a
`# see wrong_safety_nets` comment. This script checks that the comments and
the entries name each other, and that every entry is inherited by a rule the
file states.

Run from the repository root:

    python3 scripts/check_wrong_safety_nets.py
    python3 scripts/check_wrong_safety_nets.py --self-test

It prints one line per finding and exits 0 when there is none, 1 when there
are some, 2 on an input it cannot read.

Rule
----
1. A [uniform] rule whose line carries `# see wrong_safety_nets` names a
   [wrong_safety_nets] key, and that entry's `inherits_from` is the rule's
   source.
2. A [wrong_safety_nets] key that is a [uniform] rule carries the comment on
   its [uniform] line.
3. A key that is no [uniform] rule is inherited by [font_inheritance] (the key
   is `<font>.<part>` for a `<font>` in `widgets_with_font`, and
   `inherits_from` is `defaults.font.<part>`) or by [border_inheritance] (the
   key is `<widget>.border.<part>` for a `<widget>` in `widgets_with_border`
   and a `<part>.inherits_from` rule, and `inherits_from` is that rule's
   source; `corner_radius` on a `corner_radius_lg_widgets` widget takes
   `defaults.border.corner_radius_lg`), or its entry states `removed`: the
   inheritance once existed and no rule gives it now.
4. Every entry states `inherits_from` and `why_wrong` as strings.
"""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path

RULES = Path("docs/inheritance-rules.toml")
MARK = "# see wrong_safety_nets"
UNIFORM_LINE = re.compile(r'^"([^"]+)"\s*=\s*"([^"]+)"(.*)$')


class InputError(Exception):
    """A rules file the check cannot read."""


def marked_uniform_rules(text: str) -> dict[str, tuple[str, bool]]:
    """Map each [uniform] key to (source, carries the mark), from the raw text:
    TOML parsing drops the comments."""
    rules: dict[str, tuple[str, bool]] = {}
    in_uniform = False
    for line in text.splitlines():
        stripped = line.strip()
        if stripped.startswith("["):
            in_uniform = stripped == "[uniform]"
            continue
        if not in_uniform:
            continue
        m = UNIFORM_LINE.match(stripped)
        if m:
            key, source, rest = m.groups()
            rules[key] = (source, MARK in rest)
    if not rules:
        raise InputError("no [uniform] rules")
    return rules


def findings(text: str) -> list[str]:
    try:
        doc = tomllib.loads(text)
    except tomllib.TOMLDecodeError as err:
        raise InputError(f"not TOML: {err}") from err
    nets = doc.get("wrong_safety_nets")
    if not isinstance(nets, dict) or not nets:
        raise InputError("no [wrong_safety_nets] entries")
    uniform = marked_uniform_rules(text)
    fonts = set(doc.get("font_inheritance", {}).get("widgets_with_font", []))
    border = doc.get("border_inheritance", {})
    bordered = set(border.get("widgets_with_border", []))
    lg = set(border.get("corner_radius_lg_widgets", []))
    border_rules = {
        k.removesuffix(".inherits_from"): v
        for k, v in border.items()
        if k.endswith(".inherits_from") and isinstance(v, str)
    }
    if not fonts or not bordered or not border_rules:
        raise InputError("no [font_inheritance] or [border_inheritance] rules")

    out: list[str] = []
    for key, entry in nets.items():
        if not isinstance(entry, dict):
            out.append(f"{key}: not a table")
            continue
        source = entry.get("inherits_from")
        if not isinstance(source, str) or not isinstance(entry.get("why_wrong"), str):
            out.append(f"{key}: needs `inherits_from` and `why_wrong` strings")
            continue
        if key in uniform:
            rule_source, marked = uniform[key]
            if rule_source != source:
                out.append(f"{key}: inherits_from {source!r}, [uniform] says {rule_source!r}")
            if not marked:
                out.append(f"{key}: its [uniform] rule has no `{MARK}` comment")
            continue
        if "removed" in entry:
            continue
        head, _, part = key.rpartition(".")
        if head in fonts:
            if source != f"defaults.font.{part}":
                out.append(f"{key}: [font_inheritance] gives defaults.font.{part}, not {source!r}")
            continue
        widget, _, sub = key.partition(".border.")
        if sub and widget in bordered and sub in border_rules:
            want = border_rules[sub]
            if sub == "corner_radius" and widget in lg:
                want = "defaults.border.corner_radius_lg"
            if source != want:
                out.append(f"{key}: [border_inheritance] gives {want}, not {source!r}")
            continue
        out.append(f"{key}: no rule in the file inherits it, and its entry states no `removed`")
    for key, (_, marked) in uniform.items():
        if marked and key not in nets:
            out.append(f"{key}: [uniform] says `{MARK}`, but [wrong_safety_nets] has no such key")
    return out


SELF_TEST = """
[border_inheritance]
widgets_with_border = ["button", "popover"]
"line_width.inherits_from" = "defaults.border.line_width"
"corner_radius.inherits_from" = "defaults.border.corner_radius"
corner_radius_lg_widgets = ["popover"]

[font_inheritance]
widgets_with_font = ["button.font"]

[uniform]
"button.background_color" = "defaults.background_color"  # see wrong_safety_nets
"button.hover_background" = "defaults.background_color"  # see wrong_safety_nets
"menu.hover_background" = "defaults.selection_background"
"tab.background_color" = "defaults.background_color"  # see wrong_safety_nets

[wrong_safety_nets]

[wrong_safety_nets."button.background_color"]
inherits_from = "defaults.background_color"
why_wrong = "ok"

[wrong_safety_nets."button.hover_background"]
inherits_from = "defaults.surface_color"
why_wrong = "source differs"

[wrong_safety_nets."menu.hover_background"]
inherits_from = "defaults.selection_background"
why_wrong = "unmarked"

[wrong_safety_nets."button.font.color"]
inherits_from = "defaults.font.color"
why_wrong = "ok, font"

[wrong_safety_nets."popover.border.corner_radius"]
inherits_from = "defaults.border.corner_radius_lg"
why_wrong = "ok, the lg widget"

[wrong_safety_nets."button.border.line_width"]
inherits_from = "defaults.border.color"
why_wrong = "wrong border source"

[wrong_safety_nets."list.alternate_row_background"]
inherits_from = "list.background_color"
why_wrong = "ok, removed"
removed = "once"

[wrong_safety_nets."card.background_color"]
inherits_from = "defaults.surface_color"
why_wrong = "no rule"

[wrong_safety_nets."link.font.color"]
inherits_from = "defaults.link_color"
"""


def self_test() -> int:
    want = [
        "button.hover_background: inherits_from 'defaults.surface_color', [uniform] says 'defaults.background_color'",
        "menu.hover_background: its [uniform] rule has no `# see wrong_safety_nets` comment",
        "button.border.line_width: [border_inheritance] gives defaults.border.line_width, not 'defaults.border.color'",
        "card.background_color: no rule in the file inherits it, and its entry states no `removed`",
        "link.font.color: needs `inherits_from` and `why_wrong` strings",
        "tab.background_color: [uniform] says `# see wrong_safety_nets`, but [wrong_safety_nets] has no such key",
    ]
    failures = []
    got = findings(SELF_TEST)
    if got != want:
        failures.append(f"rule: got {got}, want {want}")
    for bad, label in (
        (SELF_TEST.split("[wrong_safety_nets]")[0], "no entries"),
        ("[wrong_safety_nets\n", "not TOML"),
        (SELF_TEST.replace("[uniform]", "[uniform_x]"), "no [uniform]"),
    ):
        try:
            findings(bad)
            failures.append(f"{label}: accepted")
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
    try:
        out = findings(RULES.read_text(encoding="utf-8"))
    except (OSError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for line in out:
        print(line)
    print(f"{len(out)} [wrong_safety_nets] findings", file=sys.stderr)
    return 1 if out else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
