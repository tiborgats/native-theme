#!/usr/bin/env python3
"""Every live trace row of the checked units is carried by a fact or carries a verdict (specification S17, S18, S20;
the 0.8.0 plan's M1a *Facts tooling* and M3's per-unit gate). Offline.

Reads the traceability (`docs/todo_v0.7.1_model-completeness-spec-trace-<platform>.md`, S20's `| Row | Target | Kind
|` table, as scripts/check_spec_trace.py reads it), the gap maps (which rows are live: check_spec_trace.py's
Appendix A rows less the withdrawn ones), the facts and state-model entries (`docs/facts/<platform>/*.toml`, as
scripts/check_facts.py loads them; `imported/` and `staged/` are no facts), which presets exist
(check_facts.existing_presets: the new model's `native-theme/src/next/presets/<id>.toml`) and the units
(`scripts/widget_units.toml`, scripts/generate_widgets.py's list; until it exists, the built-in slice below).

Run from the repository root:

    python3 scripts/check_facts_trace.py --enabled       # the enabled units (check_release.sh's step, from M1a)
    python3 scripts/check_facts_trace.py --unit <unit>   # one unit (M3's per-unit gate)
    python3 scripts/check_facts_trace.py                 # every trace line, whatever its target (M4's gate)
    python3 scripts/check_facts_trace.py --self-test

It prints one line per finding, tab-separated (the platform, the row ID, the target, what is wrong), informational
lines starting `# ` (per platform: its presets, how many of the checked trace lines and rows the facts already carry),
and a summary line (on stderr). Exit 0 when there is no finding, 1 when there are some, 2 on an input it cannot read,
an unknown unit or a wrong command line.

The rule
--------
A trace line (row, target, kind) is checked when its row is live (a row of the gap map's Appendix A that is not
withdrawn; a line naming anything else is check_spec_trace.py's finding and is skipped here), its platform has a
preset that exists (the facts gate checks the presets that exist; no preset, nothing is asked of that platform), and
its target lies in a checked unit (with no option, every target). It passes when its kind is a verdict (S20:
`out (R1.1)` … `readers (R18)`: the line carries a verdict), or when a fact or a state-model entry of the platform's
facts directory lists its row in `rows` (S17: `rows` lists the gap-map rows the fact carries) and carries that line:
each line of a row is counted separately (the ledger's queue #58), so a role fact naming a row carries the row's role
line and not its `list.disclosure` or `keycap` lines. A fact (a pending fact too) carries a line when its `key` (a
state-model entry: its `widget`) and one of the line's target paths (below; a `[declare.widget.<w>]` or
`[declare.component.<k>]` head names `<w>` or `component.<k>`) are related, one a leading part of the other segment by
segment, a unit suffix ignored (`button.body.stroke.top.paint` carries `button.body.{fill,stroke}`, the state model's
`button` carries `button.body.fill @ {hover}`); a target naming a core struct's field (`FocusIndicator.offset`,
`PartStyle.cursor`, an UpperCamel head) is carried by a key whose path passes through that struct to that field
(key_types: `token.focus.width` and `button.body.focus.width` both carry `FocusIndicator.width`). A key's segments are
read as check_facts.key_segments reads them, a quoted one whole (`token.text.fallbacks.per_family.".AppleSystemUIFont"`
ends in one segment; S17, ADD-A123). A line whose target
names no model path (a reader's `reader:…`, a `<widget>` placeholder) is carried by any fact or entry of its row. The
rows are compared as the gap maps write them, `†` included.

Where a target lies (target_paths): the target cell up to its first backtick (the code span; what follows is a note),
cut at its selector (` @ `), split at commas outside braces into items; each item's leading dotted path, its brace
groups expanded (`window.frame.{corners,shadow}` is two paths). A path lies in a unit when it starts with one of the
unit's entries, segment by segment (`button` holds `button.body.fill`, `token.text` holds `token.text.keyed.body`),
or is itself a leading part of one (`token.params` touches `token.params.opacity`), a segment's unit suffix (`_px`,
`_ms`, `_deg`, `_pt`) ignored (S1); and it is not deferred: a deferred path or anything under it lies in no unit
until it is enabled. An `input.…` target of kind `input` is an S9 input, never the `input` widget. A line lies in
the checked units when one of its paths lies in an enabled unit, every widget its selectors name as a context
(`context tab.tab`, `context any layout`, each name of `context nearest {toolbar, window} = toolbar.bar`; a word
that is no catalogue entry is prose) lies in an enabled unit too, and a checked unit holds the path or one of those
widgets: the model cannot state a rule under a context it does not hold (`WidgetKind` spans the enabled units), so
`button.body.fill @ context tab.tab` waits for the `tab` unit, and `--unit tab` checks it then. `--enabled` checks
every enabled unit; `--unit <unit>` that unit, the file's other units counting as enabled.

The units: `scripts/widget_units.toml` maps each enabled unit to its entries (`slice = ["button"]`), a widget name, a
`component.<key>` or a target prefix, and may hold `[deferred.<widget>]`, `<value> = "<unit>"`, values a unit
defers to a later unit (`button.values.menu_placement`, until M3's placement unit; the M1a design's decision 1),
and `[hand_written]`, `<unit> = <enabled>`, the units written by hand, which hold no entries here.
scripts/generate_widgets.py checks the deferrals' units; any other shape is an input error. The slice unit holds, besides its widgets, the colour roles and the slice's tokens (SLICE_EXTRA: the
plan's M1a scope, S4.1 motion, S4.2 caret and behaviour, S4.4 text, S4.5 icons, the focus indicator, the elevation
ramp and the size classes' thresholds, which S2.5's shadows and S8's query read (the M1a design, decision 9), and S4.6's
groups the plan names, `pointer_repeat`, `color`, `opacity`), and keeps `token.behavior.scroll` staged to the
`scroll_view` unit (SLICE_STAGED; spec ADD-A120). Until the file exists the units are SLICE alone, with button's two
`Placement` values deferred.

    python3 scripts/check_facts_trace.py --self-test   # each rule on fixture gap maps, traces, facts and units
"""
import re
import sys
import tempfile
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

import check_facts
import check_spec_names
import check_spec_trace

DOCS = Path("docs")
UNITS = Path("scripts/widget_units.toml")
SLICE_NAME = "slice"
SLICE_WIDGETS = ("button",)
SLICE_EXTRA = ("role", "token.motion", "token.caret", "token.behavior", "token.text", "token.icons", "token.focus",
               "token.elevation", "token.size_classes",
               "token.params.pointer_repeat", "token.params.color", "token.params.opacity")
SLICE_STAGED = ("token.behavior.scroll",)
SLICE_DEFERRED = {"button": ("menu_placement", "scaffold_placement")}
UNIT_SUFFIX = re.compile(r"_(?:px|ms|deg|pt)$")
CONTEXT_RX = re.compile(r"context\s+(?:any\s+|nearest\s+\{([^}]*)\}\s*=\s*)?"
                        r"(component\.[a-z_][a-z0-9_]*|[a-z_][a-z0-9_]*)")
DECLARE_RX = re.compile(r"\[declare\.((?:widget\.|component\.)[a-z_][a-z0-9_]*)\]")
PATH_HEAD = re.compile(r"@?[A-Za-z_][A-Za-z0-9_]*(?:\.(?:\{[^}]*\}|@?[A-Za-z0-9_]+))*")


class InputError(Exception):
    pass


@dataclass
class Unit:
    name: str
    entries: tuple
    deferred: tuple = ()


@dataclass
class Line:
    platform: str      # the facts directory and trace slug (gnome, kde, …)
    row: str
    target: str
    kind: str


@dataclass
class Tally:
    lines: int = 0
    rows: set = field(default_factory=set)
    carried: int = 0
    carried_rows: set = field(default_factory=set)
    verdicts: int = 0
    open: int = 0
    open_rows: set = field(default_factory=set)


# ---------------------------------------------------------------------------------------------- the units

def segs(path: str) -> list:
    return [UNIT_SUFFIX.sub("", s) for s in path.split(".")]


def slice_unit(widgets=SLICE_WIDGETS, deferred=None) -> Unit:
    deferred = SLICE_DEFERRED if deferred is None else deferred
    dpaths = tuple(f"{w}.values.{v}" for w, vs in deferred.items() for v in vs)
    return Unit(SLICE_NAME, tuple(widgets) + SLICE_EXTRA, dpaths + SLICE_STAGED)


def read_units(root: Path) -> dict:
    """{name: Unit}: scripts/widget_units.toml's units, or the built-in slice while the file does not exist."""
    p = root / UNITS
    if not p.exists():
        return {SLICE_NAME: slice_unit()}
    try:
        data = tomllib.loads(p.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as err:
        raise InputError(f"{UNITS}: {err}") from err
    deferred = data.get("deferred", {})
    if not isinstance(deferred, dict) or not all(
            isinstance(w, str) and isinstance(vs, dict) and all(isinstance(u, str) for u in vs.values())
            for w, vs in deferred.items()):
        raise InputError(f"{UNITS}: `deferred` is not a table of widgets to tables of value names to units")
    hand = data.get("hand_written", {})
    if not isinstance(hand, dict) or not all(isinstance(v, bool) for v in hand.values()):
        raise InputError(f"{UNITS}: `hand_written` is not a table of unit names to booleans")
    dpaths = tuple(f"{w}.values.{v}" for w, vs in deferred.items() for v in vs)
    out = {}
    for name, entries in data.items():
        if name in ("deferred", "hand_written"):
            continue
        if not isinstance(entries, list) or not all(isinstance(e, str) and e for e in entries):
            raise InputError(f"{UNITS}: unit `{name}` is not a list of widget names or target prefixes")
        if name == SLICE_NAME:
            out[name] = Unit(name, tuple(entries) + SLICE_EXTRA, dpaths + SLICE_STAGED)
        else:
            out[name] = Unit(name, tuple(entries), dpaths)
    return out


# --------------------------------------------------------------------------------------------- the targets

def expand(path: str) -> list:
    m = re.search(r"\{([^}]*)\}", path)
    if not m:
        return [path]
    out = []
    for alt in m.group(1).split(","):
        out += expand(path[:m.start()] + alt.strip() + path[m.end():])
    return out


def split_depth0(text: str, sep=",") -> list:
    out, depth, cur = [], 0, []
    for c in text:
        if c in "{(":
            depth += 1
        elif c in "})" and depth:
            depth -= 1
        if c == sep and depth == 0:
            out.append("".join(cur))
            cur = []
        else:
            cur.append(c)
    out.append("".join(cur))
    return out


def target_paths(target: str) -> list:
    """The dotted paths a trace target names (the docstring's *Where a target lies*)."""
    code = target.split("`", 1)[0]
    code = code.split(" @ ", 1)[0]
    out = []
    for item in split_depth0(code):
        m = PATH_HEAD.match(item.strip())
        if m:
            out += expand(m.group(0))
    return out


def in_unit(path: str, kind: str, unit: Unit) -> bool:
    p = segs(path)
    for d in unit.deferred:
        ds = segs(d)
        if p[:len(ds)] == ds:
            return False
    for e in unit.entries:
        es = segs(e)
        if es == ["input"] and kind == "input":
            continue   # an S9 input, not the `input` widget
        n = min(len(p), len(es))
        if p[:n] == es[:n]:
            return True
    return False


def context_widgets(target: str, widgets: set) -> set:
    """The catalogue widgets (and `component.<key>`s) a target's selectors name as a context (`context tab.tab`,
    `context any layout`, `context nearest {toolbar, window} = toolbar.bar`); a word that is no catalogue entry is
    prose and ignored."""
    out = set()
    for m in CONTEXT_RX.finditer(target):
        names = [m.group(2)] + ([x.strip() for x in m.group(1).split(",")] if m.group(1) else [])
        out |= {n for n in names if n in widgets}
    return out


def line_in(line: Line, scope: list, enabled: list, widgets: set) -> bool:
    """Whether a line lies in the scope's units: one of its paths does, every context widget its selectors name is
    enabled (in `enabled` or the scope), and the scope holds its path or one of those widgets."""
    paths = target_paths(line.target)
    contexts = context_widgets(line.target, widgets)
    every = enabled + scope
    if not any(in_unit(p, line.kind, u) for u in every for p in paths):
        return False
    if not all(any(in_unit(w, "", u) for u in every) for w in contexts):
        return False
    return any(in_unit(x, line.kind, u) for u in scope for x in paths + sorted(contexts))


# ---------------------------------------------------------------------------------------------- the check

def read_lines(root: Path) -> list:
    """Every trace line of a live row, per platform."""
    docs = root / DOCS
    try:
        rows = check_spec_trace.map_rows((docs / check_spec_trace.MAP.name).read_text(encoding="utf-8"))
        out = []
        for name, slug in check_spec_trace.PLATFORMS.items():
            platform_text = (docs / f"todo_v0.7.1_model-completeness-gap-map-{slug}.md").read_text(encoding="utf-8")
            trace = check_spec_trace.trace_lines(
                (docs / f"todo_v0.7.1_model-completeness-spec-trace-{slug}.md").read_text(encoding="utf-8"))
            gone = check_spec_trace.withdrawn(platform_text, rows[name])
            live = rows[name] - gone
            out += [Line(slug, row, target, kind) for row, target, kind in trace if row in live]
    except (OSError, UnicodeDecodeError, check_spec_trace.InputError) as err:
        raise InputError(str(err)) from err
    return out


def carried_rows(ctx) -> dict:
    """{platform: {row: the keys of the facts and state-model entries whose `rows` list it}}: a fact's `key` (a
    pending fact's too), a state-model entry's `widget`."""
    items, _, _ = check_facts.load(ctx)
    out = {}
    for it in items:
        if it.palette:
            continue
        rows = it.data.get("rows")
        key = it.data.get("widget" if it.table == "model" else "key")
        if isinstance(rows, list) and isinstance(key, str):
            for r in rows:
                if isinstance(r, str):
                    out.setdefault(it.platform, {}).setdefault(r, set()).add(key)
    return out


def key_segs(key: str) -> list:
    """A fact's key as its segments (check_facts.key_segments: a quoted segment is one), or split at its dots where it
    does not read (check_facts.py reports it)."""
    parsed = check_facts.key_segments(key)
    return list(parsed[0]) if isinstance(parsed, tuple) else key.split(".")


def related(a, b) -> bool:
    """One path is a leading part of the other, segment by segment (a unit suffix ignored); a path is a dotted
    string or a list of segments."""
    sa = segs(a) if isinstance(a, str) else [UNIT_SUFFIX.sub("", x) for x in a]
    sb = segs(b) if isinstance(b, str) else [UNIT_SUFFIX.sub("", x) for x in b]
    n = min(len(sa), len(sb))
    return sa[:n] == sb[:n]


def key_types(ctx, key: str) -> list:
    """(core struct, the key's segments below it) for each of the core's structs the key's path passes through
    (check_facts.type_at's walk): a part's field from `PartStyle`, a token from `Tokens`, a colour role from
    `GlobalRoles` or `SurfaceRoles`; [] for a widget value, an input or a key the sketches do not type."""
    s = key_segs(key)
    if s[0] == "token":
        t, rest = "Tokens", s[1:]
    elif s[0] == "role" and len(s) > 1:
        t, rest = ("GlobalRoles", s[2:]) if s[1] == "global" else ("SurfaceRoles",
                                                                   [x for x in s[2:] if x != "disabled"])
    elif s[0] == "input":
        return []
    else:
        rest = s[3:] if s[0] == "component" else s[2:]
        if not rest or s[1 + (s[0] == "component")] == "values":
            return []
        t = "PartStyle"
    out = []
    for _ in range(64):
        t = check_facts.strip_type(t)
        m = re.fullmatch(r"Sides<(.+)>", t) or re.fullmatch(r"Vec<\(\s*\w+\s*,\s*(.+)\)>", t)
        if m and rest:
            rest, t = rest[1:], m.group(1).strip()
            continue
        if t not in ctx.structs:
            break
        out.append((t, rest))
        if not rest:
            break
        fields = ctx.structs[t]
        name = rest[0] if rest[0] in fields else check_spec_names.norm_value(rest[0])
        if name not in fields:
            break
        rest, t = rest[1:], fields[name]
    return out


def line_carried(ctx, line: Line, keys) -> bool:
    """Whether a fact or state-model entry whose `rows` list the line's row carries the line (each line of a row
    separately; the ledger's queue #58): its key (a state-model entry's widget) and one of the line's target paths
    are related, the key lying under the target (`button.body.fill` under `button.body.{fill,stroke}`) or the target
    under the key (a state model's `button`, under `[declare.widget.button]` too); a target naming a core struct's
    field (`FocusIndicator.offset`, `PartStyle.cursor`) is carried by a key whose path passes through that struct to
    that field (key_types). A line whose target names no path of the model (a reader's `reader:…`, a `<widget>`
    placeholder) is carried by any fact or entry of its row, as nothing in it names a key."""
    paths = [p for p in target_paths(line.target) if p.split(".")[0] != "reader"]
    paths += [f"{m.group(1).replace('widget.', '', 1)}" for m in DECLARE_RX.finditer(line.target)]
    if not paths:
        return bool(keys)
    for k in keys:
        for p in paths:
            head = p.split(".")[0]
            if head[:1].isupper():
                if any(t == head and related(rest, p.split(".")[1:]) for t, rest in key_types(ctx, k)):
                    return True
            elif related(key_segs(k), p):
                return True
    return False


def check(ctx, units, enabled=()):
    """(findings as (platform, row, target, what), {platform: Tally}, {platform: existing presets}).
    `units` None checks every line; `enabled` are the other enabled units (a context widget may lie there)."""
    lines = read_lines(ctx.root)
    carried = carried_rows(ctx)
    existing = check_facts.existing_presets(ctx.root)
    widgets = set(ctx.entries)
    presets = {p: sorted(set(ps) & existing) for p, ps in check_facts.PLATFORMS.items()}
    findings, tallies = [], {p: Tally() for p in check_facts.PLATFORMS}
    for line in lines:
        if units is not None and not line_in(line, list(units), list(enabled), widgets):
            continue
        t = tallies[line.platform]
        t.lines += 1
        t.rows.add(line.row)
        if line.kind in check_spec_trace.VERDICTS:
            t.verdicts += 1
            continue
        if line_carried(ctx, line, carried.get(line.platform, {}).get(line.row, ())):
            t.carried += 1
            t.carried_rows.add(line.row)
            continue
        t.open += 1
        t.open_rows.add(line.row)
        if presets[line.platform]:
            findings.append((line.platform, line.row, line.target, "in no fact's or state-model entry's `rows` of "
                                                                   f"docs/facts/{line.platform}/, and no verdict"))
    return findings, tallies, presets


def report(tallies, presets, scope):
    out = []
    for p, t in tallies.items():
        state = f"{', '.join(presets[p])} exist{'s' if len(presets[p]) == 1 else ''}" if presets[p] else \
            f"no preset exists ({', '.join(check_facts.PLATFORMS[p])}): not checked"
        out.append(f"{p} ({state}): {t.lines} {scope} trace lines over {len(t.rows)} rows; facts carry {t.carried} "
                   f"lines ({len(t.carried_rows)} rows), {t.verdicts} verdict lines, {t.open} lines "
                   f"({len(t.open_rows)} rows) not yet carried")
    return out


def summary(findings, tallies, presets, scope):
    checked = [p for p in presets if presets[p]]
    vacuous = "no preset exists, nothing checked: " if not checked else ""
    return (f"{vacuous}{sum(t.lines for t in tallies.values())} {scope} trace lines on {len(checked)} platform(s) "
            f"with presets: {len(findings)} findings")


# --------------------------------------------------------------------------------------------- self-test

SELF_TEST_MAP = """
## Appendix A. Rows per capability
- macOS: MAC-1
- Windows: W1-001
- KDE: K1.1
- GNOME: GN-1.1, GN-1.2, GN-1.3, GN-1.4, GN-1.5, GN-1.6, GN-1.7, GN-1.8, GN-1.9†, GN-1.10
- Material: MAT-1
- iOS: B-1
## Appendix B
"""
SELF_TEST_GNOME_MAP = """
| GN-1.8 | button | Withdrawn (review 2026-10-28): duplicate | src | none |
"""
SELF_TEST_GNOME_TRACE = """
| Row | Target | Kind |
|---|---|---|
| GN-1.1 | `button.body.fill @ {hover} variant flat` | rule |
| GN-1.2 | `role.window.{background,foreground}` | role |
| GN-1.3 | `token.motion.durations.button, token.motion.timings.ease` | token |
| GN-1.4 | `button.values.menu_placement` | placement |
| GN-1.4 | `token.behavior.scroll.deceleration` | token |
| GN-1.5 | `token.behavior.scroll_lines_per_step` | token |
| GN-1.6 | `window.frame.shadow` (the `button` beneath) | field |
| GN-1.6 | `input.accent` | input |
| GN-1.7 | - (its panel) | out (R1.1) |
| GN-1.7 | `badge.body.{fill,corners} @ context button` | rule |
| GN-1.8 | `button.body.stroke` | field |
| GN-1.9† | `token.params.{opacity,layout}` | token |
| GN-1.10 | `button.body.fill @ context nearest {toolbar, window} = toolbar.bar` (window: no entry here) | rule |
| GN-9 | `button.body.fill` | field |
"""
SELF_TEST_OTHER_TRACE = """
| Row | Target | Kind |
|---|---|---|
| {row} | `button.body.fill` | field |
"""
SELF_TEST_FACTS = {
    "docs/facts/gnome/button.toml": '''
[[fact]]
key = "button.body.fill"
value = "#ffffff"
rows = ["GN-1.1"]

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 a.scss:1"
quote = "x"
''',
    "docs/facts/gnome/state-models.toml": '''
[[model]]
widget = "button"
available = true
rows = ["GN-1.2"]

[[model.cite]]
kind = "published"
source = "libadwaita@1.10.0 a.scss:1"
quote = "x"
''',
    "docs/facts/gnome/roles.toml": '''
[[fact]]
key = "role.window.background"
value = "#ffffff"
rows = ["GN-1.2"]

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 a.scss:1"
quote = "x"
''',
    "docs/facts/imported/gnome.toml": '''
[[row]]
rows = ["GN-1.3"]
''',
    "docs/facts/macos/button.toml": '''
[[fact]]
key = "button.body.fill"
value = "#ffffff"
rows = ["MAC-1"]

[[fact.cite]]
kind = "published"
source = "xcode@26.6 a.h:1"
quote = "x"
''',
}
SELF_TEST_PLATFORMS = {"macos": "MAC-1", "windows": "W1-001", "kde": "K1.1", "material": "MAT-1", "ios": "B-1"}


def write_fixture(root: Path, presets=(), units=None, extra=None):
    files = {f"docs/{check_spec_trace.MAP.name}": SELF_TEST_MAP,
             "docs/todo_v0.7.1_model-completeness-gap-map-gnome.md": SELF_TEST_GNOME_MAP,
             "docs/todo_v0.7.1_model-completeness-spec-trace-gnome.md": SELF_TEST_GNOME_TRACE}
    for slug, row in SELF_TEST_PLATFORMS.items():
        files[f"docs/todo_v0.7.1_model-completeness-gap-map-{slug}.md"] = ""
        files[f"docs/todo_v0.7.1_model-completeness-spec-trace-{slug}.md"] = SELF_TEST_OTHER_TRACE.format(row=row)
    files.update(SELF_TEST_FACTS)
    files.update(extra or {})
    for p in presets:
        files[f"{check_facts.PRESET_DIRS[0].as_posix()}/{p}.toml"] = ""
    if units is not None:
        files[UNITS.as_posix()] = units
    for rel, text in files.items():
        if text is None:
            continue
        p = root / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text, encoding="utf-8")


def fixture_context(root: Path):
    return check_facts.Context(root, core_text=check_facts.SELF_TEST_CORE, cat_text=check_facts.SELF_TEST_CATALOGUE,
                               sources_text=check_facts.SELF_TEST_SOURCES, rows=check_facts.SELF_TEST_ROWS)


def self_test() -> int:
    failures = []
    with tempfile.TemporaryDirectory() as tmp:
        base = Path(tmp)
        n = [0]

        def run(presets=(), units=None, unit=None, extra=None, every=False):
            root = base / f"case{n[0]}"
            n[0] += 1
            write_fixture(root, presets, units, extra)
            ctx = fixture_context(root)
            known = read_units(root)
            chosen = None if every else ([known[unit]] if unit else list(known.values()))
            others = [u for k, u in known.items() if unit and k != unit]
            found, tallies, pre = check(ctx, chosen, others)
            return [(p, r, w) for p, r, _, w in found], tallies, pre

        def rows_of(found):
            return sorted({(p, r) for p, r, _ in found})

        # the vacuous run: no preset exists, nothing is a finding, the tallies still count
        found, tallies, pre = run()
        g = tallies["gnome"]
        if found or (g.lines, len(g.rows), g.carried, g.verdicts, g.open) != (5, 5, 2, 0, 3):
            failures.append(f"vacuous: {found}, gnome {g}")
        if not summary(found, tallies, pre, "slice").startswith("no preset exists, nothing checked: 10 slice"):
            failures.append(f"vacuous summary: {summary(found, tallies, pre, 'slice')}")
        if not report(tallies, pre, "slice")[0].startswith("gnome (no preset exists (adwaita): not checked): 5 slice"):
            failures.append(f"vacuous report: {report(tallies, pre, 'slice')[0]}")
        # adwaita exists: the slice's open lines are findings, macos (no preset) is not checked
        found, tallies, pre = run(presets=("adwaita",))
        want = [("gnome", "GN-1.3"), ("gnome", "GN-1.5"), ("gnome", "GN-1.9†")]
        if rows_of(found) != want or len(found) != 3:
            failures.append(f"adwaita, slice: got {found}, want rows {want}, one finding per line")
        # a fact carrying the row passes it; the imported folder carries nothing
        fact = SELF_TEST_FACTS["docs/facts/gnome/button.toml"]
        extra = {"docs/facts/gnome/tokens.toml": "\n".join(
            fact.replace('"button.body.fill"', f'"{k}"').replace('"GN-1.1"', f'"{r}"') for k, r in (
                ("token.motion.durations.button", "GN-1.3"), ("token.behavior.scroll_lines_per_step", "GN-1.5"),
                ("token.params.opacity.disabled", "GN-1.9†")))}
        found, _, _ = run(presets=("adwaita",), extra=extra)
        if found:
            failures.append(f"carried by a fact: got {found}")
        # a fact carries only its row's lines its key lies on (the ledger's queue #58): a button fact naming the
        # token rows carries none of their lines, and the state model's button none of GN-1.2's role line
        extra = {"docs/facts/gnome/tokens.toml": fact.replace('"GN-1.1"', '"GN-1.3", "GN-1.5", "GN-1.9†"'),
                 "docs/facts/gnome/roles.toml": None}
        found, tallies, _ = run(presets=("adwaita",), extra=extra)
        want = [("gnome", "GN-1.2"), ("gnome", "GN-1.3"), ("gnome", "GN-1.5"), ("gnome", "GN-1.9†")]
        if rows_of(found) != want or tallies["gnome"].carried != 1:
            failures.append(f"a row's other lines: got {found}, {tallies['gnome']}")
        for target, keys, want in (
                ("list.disclosure.glyph.fill @ {selected} variant @column_view", {"role.@shade_mid"}, False),
                ("role.@shade_mid, role.@shade_dark (KDE's 3-D shades, K3.53)", {"role.@shade_mid"}, True),
                ("keycap.key.fill (= role.window.background) @ context component.command_bar", {"role.@shade_dark"},
                 False),
                ("button.body.{fill,stroke} @ {hover}", {"button.body.stroke.top.paint"}, True),
                ("button.body.padding_px @ variant flat", {"button.body.padding.top"}, True),
                ("button.body.fill @ {hover}", {"button"}, True),
                ("toolbar.bar.fill", {"button"}, False),
                ("[declare.widget.button] default_size_by_variant: @pill regular", {"button"}, True),
                ("[declare.component.led] available", {"component.led"}, True),
                ("PartStyle.min_width per part", {"button.body.min_width"}, True),
                ("PartStyle.min_width per part", {"button.body.fill"}, False),
                ("FocusIndicator.width (the ring)", {"token.focus.width"}, True),
                ("FocusIndicator.width (the ring)", {"button.body.focus.width"}, True),
                ("FocusIndicator.corners", {"token.focus.width"}, False),
                ("reader:kde:kdeglobals [KDE] contrast", {"token.params.color.contrast"}, True),
                ("reader:kde:kdeglobals [KDE] contrast", set(), False),
                ("token.text.fallbacks.per_family", {'token.text.fallbacks.per_family."Segoe UI"'}, True),
                ("FontFallbacks.per_family", {'token.text.fallbacks.per_family.".AppleSystemUIFont"'}, True)):
            if line_carried(fixture_context(base / "case0"), Line("kde", "K1.1", target, "rule"), keys) != want:
                failures.append(f"line {target!r} carried by {sorted(keys)}: got {not want}")
        # every line: the deferred, the staged, other widgets', input and the verdict lines' rows
        found, tallies, _ = run(presets=("adwaita", "macos-tahoe"), every=True)
        want = [("gnome", "GN-1.3"), ("gnome", "GN-1.4"), ("gnome", "GN-1.5"), ("gnome", "GN-1.6"),
                ("gnome", "GN-1.7"), ("gnome", "GN-1.9†"), ("gnome", "GN-1.10")]
        if rows_of(found) != sorted(want) or tallies["gnome"].verdicts != 1 or tallies["macos"].carried != 1:
            failures.append(f"every line: got {found}, {tallies['gnome']}, {tallies['macos']}")
        # the units file: another unit, its deferral; the slice from the file keeps the roles and tokens
        units = ('slice = ["button"]\nbadges = ["badge", "window"]\n[deferred.button]\n'
                 'menu_placement = "placement"\n[hand_written]\nplacement = false\n')
        found, _, _ = run(presets=("adwaita",), units=units, unit="badges")
        if rows_of(found) != [("gnome", "GN-1.6"), ("gnome", "GN-1.7")]:
            failures.append(f"unit badges: got {found}")
        found, _, _ = run(presets=("adwaita",), units=units, unit="slice")
        if rows_of(found) != [("gnome", "GN-1.3"), ("gnome", "GN-1.5"), ("gnome", "GN-1.7"), ("gnome", "GN-1.9†")]:
            failures.append(f"unit slice from the file (a badge rule under context button touches it): got {found}")
        # a rule under a context widget waits for that widget's unit, then both units check it
        units_bars = 'slice = ["button"]\nbars = ["toolbar"]\n'
        for unit, want in ((None, True), ("bars", True), ("slice", True)):
            found, _, _ = run(presets=("adwaita",), units=units_bars, unit=unit)
            if (("gnome", "GN-1.10") in rows_of(found)) != want:
                failures.append(f"context toolbar enabled, unit {unit}: got {found}")
        found, _, _ = run(presets=("adwaita",))
        if ("gnome", "GN-1.10") in rows_of(found):
            failures.append(f"context toolbar not enabled: GN-1.10 checked: {found}")
        units_open = 'slice = ["button"]\n'
        found, _, _ = run(presets=("adwaita",), units=units_open, unit="slice")
        if ("gnome", "GN-1.4") not in rows_of(found) or ("gnome", "GN-1.10") in rows_of(found):
            failures.append(f"no deferral in the file: GN-1.4's menu_placement not checked: {found}")
        for bad in ('slice = "button"\n', 'slice = ["button"]\ndeferred = ["x"]\n',
                    'slice = ["button"]\ndeferred = { button = ["menu_placement"] }\n',
                    'slice = ["button"]\nhand_written = ["placement"]\n', "slice = [\n"):
            try:
                write_fixture(base / "bad", units=bad)
                read_units(base / "bad")
                failures.append(f"units file {bad!r}: accepted")
            except InputError:
                pass
        # target_paths and in_unit
        for target, want in (
                ("button.body.fill @ {hover}", ["button.body.fill"]),
                ("window.frame.{corners,stroke} @ cond x", ["window.frame.corners", "window.frame.stroke"]),
                ("a.b, c.d` (note, `e.f`)", ["a.b", "c.d"]),
                ("role.@overview_background", ["role.@overview_background"]),
                ("reader:kde:kdeglobals [General]", ["reader"]),
                ("- (its panel)", [])):
            if target_paths(target) != want:
                failures.append(f"target_paths({target!r}): got {target_paths(target)}, want {want}")
        unit = slice_unit()
        for path, kind, want in (("button.body.min_width_px", "field", True), ("token.params", "token", True),
                                 ("token.params.layout", "token", False), ("buttons.body", "field", False),
                                 ("button.values.scaffold_placement", "placement", False),
                                 ("token.behavior.scroll", "token", False),
                                 ("token.behavior.scroll_px_per_step", "token", True)):
            if in_unit(path, kind, unit) != want:
                failures.append(f"in_unit({path!r}): got {not want}")
        # a key's quoted segment is one segment, its dots included (S17, ADD-A123)
        got = key_types(fixture_context(base / "case0"), 'token.text.fallbacks.per_family.".AppleSystemUIFont"')
        if ("FontFallbacks", ["per_family", ".AppleSystemUIFont"]) not in got:
            failures.append(f"key_types of a quoted segment: got {got}")
        widget_input = Unit("fields", ("input",))
        if in_unit("input.accent", "input", widget_input) or not in_unit("input.body.fill", "field", widget_input):
            failures.append("in_unit: an S9 input is not the input widget")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def main(argv) -> int:
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    if args not in ([], ["--enabled"]) and not (len(args) == 2 and args[0] == "--unit"):
        print(__doc__, file=sys.stderr)
        return 2
    try:
        known = read_units(Path("."))
        others = []
        if not args:
            units, scope = None, "all"
        elif args[0] == "--enabled":
            units, scope = list(known.values()), "enabled-unit"
        elif args[1] in known:
            units, scope = [known[args[1]]], f"`{args[1]}`"
            others = [u for name, u in known.items() if name != args[1]]
        else:
            print(f"error: unknown unit `{args[1]}` (units: {', '.join(known)})", file=sys.stderr)
            return 2
        ctx = check_facts.Context(Path("."))
        findings, tallies, presets = check(ctx, units, others)
    except (OSError, UnicodeDecodeError, InputError, check_facts.InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for platform, row, target, what in findings:
        print(f"{platform}\t{row}\t{target}\t{what}")
    if units is not None:
        print(f"# units: {', '.join(u.name for u in units)}"
              f"{'' if (Path('.') / UNITS).exists() else ' (the built-in slice: scripts/widget_units.toml does not exist yet)'}")
    for line in report(tallies, presets, scope):
        print(f"# {line}")
    print(summary(findings, tallies, presets, scope), file=sys.stderr)
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
