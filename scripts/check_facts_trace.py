#!/usr/bin/env python3
"""Every live trace row of the checked units is carried by a fact or carries a verdict (specification S17, S18, S20;
the 0.8.0 plan's M1a *Facts tooling* and M3's per-unit gate). Offline.

Reads the traceability (`docs/todo_v0.7.1_model-completeness-spec-trace-<platform>.md`, S20's `| Row | Target | Kind
|` table, as scripts/check_spec_trace.py reads it), the gap maps (which rows are live: check_spec_trace.py's
Appendix A rows less the withdrawn ones), the facts and state-model entries (`docs/facts/<platform>/*.toml`, as
scripts/check_facts.py loads them; `imported/` and `staged/` are no facts), which presets exist
(check_facts.existing_presets: the bundled presets, `native-theme/src/presets/<id>.toml`) and the units
(`scripts/widget_units.toml`, scripts/generate_widgets.py's list, below).

Run from the repository root:

    python3 scripts/check_facts_trace.py --enabled       # the enabled units (check_release.sh's step, from M1a)
    python3 scripts/check_facts_trace.py --unit <unit>   # one unit (M3's per-unit gate)
    python3 scripts/check_facts_trace.py                 # every trace line, whatever its target (M4's gate)
    python3 scripts/check_facts_trace.py --enabled --preset kde-breeze   # as if kde-breeze's preset existed
    python3 scripts/check_facts_trace.py --self-test
    python3 scripts/check_facts_trace.py --facts         # per preset and directory, the facts of the enabled units,
                                                         # each later unit's and the unplaced ones, and each fact's
                                                         # place per preset (facts_report)

`--preset <id>` (repeatable, beside any of the first three) checks a preset of check_facts.PLATFORMS as if it
existed: its platform's lines are findings as they would be once the preset is bundled, so a facts lane sees them
before the preset task writes the file; the report marks the preset `(assumed)`. An unknown id is exit 2.

It prints one line per finding, tab-separated (the platform, the row ID, the target, what is wrong), informational
lines starting `# ` (per platform: its presets, how many of the checked trace lines and rows the facts already carry),
and a summary line (on stderr). Exit 0 when there is no finding, 1 when there are some, 2 on an input it cannot read,
an unknown unit or a wrong command line.

The rule
--------
A trace line (row, target, kind) is checked when its row is live (a row of the gap map's Appendix A that is not
withdrawn; a line naming anything else is check_spec_trace.py's finding and is skipped here), its platform has a preset
that exists (the facts gate checks the presets that exist; no preset, nothing is asked of that platform; one `--preset`
names counts as existing), and its target lies in a checked unit (with no option, every target). It passes when its kind
is a verdict (S20: `out (R1.1)` … `readers (R18)`: the line carries a verdict), or when a fact or a state-model entry of
the platform's facts directory lists its row in `rows` (S17: `rows` lists the gap-map rows the fact carries) and carries
that line: each line of a row is counted separately (the ledger's queue #58), so a role fact naming a row carries the
row's role line and not its `list.disclosure` or `keycap` lines. A fact (a pending fact too) carries a line when its
`key` (a state-model entry: its `widget`) and one of the line's target paths (below; a `[declare.widget.<w>]` or
`[declare.component.<k>]` head, a flavour's `[declare.flavor.<f>.widget.<w>]` too, names `<w>` or `component.<k>`:
`[declare.flavor.expressive.widget.button] sizes_by_variant: @fab default_size regular`, ADD-A131) are related, one a
leading part of the other segment by segment, a unit suffix ignored (`button.body.stroke.top.paint` carries
`button.body.{fill,stroke}`, the state model's `button` carries `button.body.fill @ {hover}`); a target naming a core
struct's field (`FocusIndicator.offset`, `PartStyle.cursor`, an UpperCamel head) is carried by a key whose path passes
through that struct to that field (key_types: `token.focus.width` and `button.body.focus.width` both carry
`FocusIndicator.width`). A key's segments are read as check_facts.key_segments reads them, a quoted one whole
(`token.text.fallbacks.per_family.".AppleSystemUIFont"` ends in one segment; S17, ADD-A123). A line whose target names
no model path (a reader's `reader:…`, a `<widget>` placeholder) is carried by any fact or entry of its row. The rows are
compared as the gap maps write them, `†` included.

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

The units: `scripts/widget_units.toml` maps each enabled unit to its catalogue entries (`slice = ["button"]`), a
widget name or a `component.<key>`, and may hold `[prefixes]`, `<unit> = [...]`, the target prefixes a unit holds
beside them, written by hand (the slice's colour roles and tokens: the plan's M1a scope, S4.1 motion, S4.2 caret and
behaviour, S4.4 text, S4.5 icons, the focus indicator, the elevation ramp and the size classes' thresholds, which S2.5's
shadows and S8's query read (the M1a design, decision 9), and S4.6's groups the button and the roles use,
`pointer_repeat`, `color`, `opacity`); `[staged.<unit>]`, `"<path>" = "<unit>"`, paths a unit's prefixes hold that it
stages to a later unit, lying in no unit until then (`token.behavior.scroll`, to the `scroll_view` unit; spec
ADD-A120); `[deferred.<widget>]`, `<value> = "<unit>"`, values a unit defers to a later unit
(`button.values.menu_placement`, until M3's placement unit; the M1a design's decision 1); and `[hand_written]`,
`<unit> = <enabled>`, the units written by hand, which hold no entries here; and `[unit_names]`, `<milestone> =
[...]`, units named before the file lists them (M3a's widget units). A `[staged]` or `[deferred]` value names a
`[[later]]` unit, a `[hand_written]` one, one `[unit_names]` lists or a catalogue entry (check_waiting(), which
scripts/generate_widgets.py mirrors): any other value is an input error naming the key and the value, as is any
other shape, and so is a missing file. The facts gate reads the same file
by the same rule (`native-theme/tests/facts_gate/units.rs`).

The depth (S6.1, S17; ADD-A147): `[deferred.<widget>]` may name a part's field (`"body.transitions"`); a `[staged]`
path may hold a `*` segment (any one) or `@*` (any keyed one); `[kept.<unit>]`, `<preset> = [paths]`, names the entries
of a staged keyed map that lie in the unit on that preset all the same (the preset one of check_facts.ALL_PRESETS, each
path under one of the unit's `[staged]` paths: read_units refuses any other); `[at_depth]`, `<key prefix> =
[conditions]`, the conditions at depth for those keys that a later unit holds for every other; and `[[later]]` tables the later
units in their order (read_later; Later). A name of an entry is at depth where no later unit not enabled holds it,
and a fact is of the enabled units by the predicate of the class Depth (its docstring: decided exactly over the
combinations at depth; any other fact attributed to a unit by the names it lists, or unplaced), which
scripts/check_ledger.py and scripts/generate_preset.py read too. A trace line is checked by `--enabled` or `--unit <widget unit>` where one
alternative of its selector names only names at depth (line_later(): its variants and flags for the target's entry
on every preset of its platform, its conditions for the target's path, a size no default in force, a density, a
direction), so an M3a unit is asked for its rows at depth and no more; `--unit <later unit>` checks the lines one of
whose alternatives names names of that unit and of units before it alone (S17: a line needing two units is the
last's, whose gate can check it).

    python3 scripts/check_facts_trace.py --self-test   # each rule on fixture gap maps, traces, facts and units
"""
import contextlib
import io
import json
import re
import sys
import tempfile
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

import check_facts
import check_spec_names
import check_spec_trace
import spec_catalogue

DOCS = Path("docs")
UNITS = Path("scripts/widget_units.toml")
# The units file's table of unit names listed before their units are (M3a's widget units).
NAMED = "unit_names"
# The units file's tables that are no unit.
TABLES = ("deferred", "hand_written", "prefixes", "staged", "kept", "at_depth", "later", NAMED)
# A later unit's kinds (S6.1, S17; ADD-A147): a unit of depth, a parking unit.
LATER_KINDS = ("depth", "parking")
# The names a later unit holds per catalogue entry.
ENTRY_NAMES = ("variants", "states")
UNIT_SUFFIX = re.compile(r"_(?:px|ms|deg|pt)$")
CONTEXT_RX = re.compile(r"context\s+(?:any\s+|nearest\s+\{([^}]*)\}\s*=\s*)?"
                        r"(component\.[a-z_][a-z0-9_]*|[a-z_][a-z0-9_]*)")
DECLARE_RX = re.compile(r"\[declare\.(?:flavor\.[a-z_][a-z0-9_]*\.)?((?:widget\.|component\.)[a-z_][a-z0-9_]*)\]")
PATH_HEAD = re.compile(r"@?[A-Za-z_][A-Za-z0-9_]*(?:\.(?:\{[^}]*\}|@?[A-Za-z0-9_]+))*")


class InputError(Exception):
    pass


@dataclass
class Unit:
    name: str
    entries: tuple
    deferred: tuple = ()
    kept: dict = field(default_factory=dict)   # {preset: (paths)}: the keyed entries at depth its stagings cover


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


def deferred_path(widget: str, name: str) -> str:
    """The path a `[deferred.<widget>]` key defers: `<widget>.<part>.<field>` for a part's field (`body.transitions`),
    else the widget value `<widget>.values.<name>`."""
    return f"{widget}.{name}" if "." in name else f"{widget}.values.{name}"


def under(path: list, prefix: list) -> bool:
    """Whether `path` starts with `prefix`, segment by segment, a `*` segment of the prefix standing for any one, a
    `@*` for any keyed one (`role.@*`, the keyed colour roles)."""
    return len(path) >= len(prefix) and all(p == "*" or p == s or p == "@*" and s.startswith("@")
                                            for p, s in zip(prefix, path))


def read_units(root: Path) -> dict:
    """{name: Unit}: scripts/widget_units.toml's enabled units, each with its catalogue entries and its `prefixes`,
    each holding no path it stages (`staged`) and no value a unit defers (`deferred`)."""
    p = root / UNITS
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
    prefixes = data.get("prefixes", {})
    if not isinstance(prefixes, dict) or not all(
            isinstance(ps, list) and all(isinstance(e, str) and e for e in ps) for ps in prefixes.values()):
        raise InputError(f"{UNITS}: `prefixes` is not a table of unit names to lists of target prefixes")
    staged = data.get("staged", {})
    if not isinstance(staged, dict) or not all(
            isinstance(ps, dict) and all(isinstance(u, str) for u in ps.values()) for ps in staged.values()):
        raise InputError(f"{UNITS}: `staged` is not a table of unit names to tables of paths to units")
    kept = data.get("kept", {})
    if not isinstance(kept, dict) or not all(
            isinstance(ps, dict) and all(str_list(ks) for ks in ps.values()) for ps in kept.values()):
        raise InputError(f"{UNITS}: `kept` is not a table of unit names to tables of presets to lists of paths")
    check_waiting(root, data)
    dpaths =tuple(deferred_path(w, v) for w, vs in deferred.items() for v in vs)
    out = {}
    for name, entries in data.items():
        if name in TABLES:
            continue
        if not isinstance(entries, list) or not all(isinstance(e, str) and e for e in entries):
            raise InputError(f"{UNITS}: unit `{name}` is not a list of widget names or target prefixes")
        out[name] = Unit(name, tuple(entries) + tuple(prefixes.get(name, ())),
                         dpaths + tuple(staged.get(name, {})),
                         {p: tuple(ks) for p, ks in kept.get(name, {}).items()})
    for table in ("prefixes", "staged", "kept"):
        for name in data.get(table, {}):
            if name not in out:
                raise InputError(f"{UNITS}: `{table}.{name}` names no enabled unit")
    for name, presets in kept.items():
        for preset, paths in presets.items():
            if preset not in check_facts.ALL_PRESETS:
                raise InputError(f"{UNITS}: `kept.{name}.{preset}` names no preset "
                                 f"({', '.join(check_facts.ALL_PRESETS)})")
            for path in paths:
                if not any(under(segs(path), segs(s)) for s in staged.get(name, {})):
                    raise InputError(f"{UNITS}: `kept.{name}.{preset}` keeps `{path}`, which no `[staged.{name}]` "
                                     f"path covers")
    return out


def waiting_values(data: dict) -> list:
    """[(the key as the units file spells it, the unit it names)]: each `[staged.<unit>]` path and each
    `[deferred.<widget>]` value or part's field, in the file's order."""
    staged = data.get("staged", {}) if isinstance(data.get("staged"), dict) else {}
    deferred = data.get("deferred", {}) if isinstance(data.get("deferred"), dict) else {}
    out = [(f'staged.{u}."{p}"', v) for u, paths in staged.items() if isinstance(paths, dict)
           for p, v in paths.items()]
    return out + [(f'deferred.{w}.{n}' if "." not in n else f'deferred.{w}."{n}"', v)
                  for w, vs in deferred.items() if isinstance(vs, dict) for n, v in vs.items()]


def check_waiting(root: Path, data: dict) -> None:
    """InputError, naming the file, the key and the value, for a `[staged]` or `[deferred]` value that names none of
    the units a path may wait for (S6.1, ADD-A147): a `[[later]]` unit, a `[hand_written]` unit, a unit `[unit_names]`
    lists (M3a's widget units, until the units file lists them with their entries) or a catalogue entry (the
    catalogue read only for a value none of the others names); and for a `[unit_names]` that is no table of lists of
    names."""
    named = data.get(NAMED, {})
    if not isinstance(named, dict) or not all(str_list(v) for v in named.values()):
        raise InputError(f"{UNITS}: `{NAMED}` is not a table of lists of unit names")
    later = data.get("later", [])
    known = {t.get("name") for t in later if isinstance(t, dict)} if isinstance(later, list) else set()
    known |= set(data.get("hand_written", {})) if isinstance(data.get("hand_written"), dict) else set()
    known |= {u for v in named.values() for u in v}
    unknown = [(k, v) for k, v in waiting_values(data) if v not in known]
    if not unknown:
        return
    try:
        entries = {e.name for e in spec_catalogue.read(root / spec_catalogue.CATALOGUE)}
    except (OSError, UnicodeDecodeError) as err:
        key, value = unknown[0]
        raise InputError(f"{UNITS}: `{key}` = {value!r} names no later, hand-written or listed unit, and the catalogue "
                         f"that may name it an entry cannot be read: {err}") from err
    for key, value in unknown:
        if value not in entries:
            raise InputError(f"{UNITS}: `{key}` = {value!r} names no later unit, no hand-written unit, no unit "
                             f"`{NAMED}` lists and no catalogue entry")


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


def in_unit(path: str, kind: str, unit: Unit, preset: str | None = None) -> bool:
    """Whether a path lies in the unit: under one of its entries or prefixes and under no path it stages or defers,
    unless the preset keeps it (`[kept]`, an entry under that staged path; with no preset, any preset)."""
    p = segs(path)
    kept = unit.kept.get(preset, ()) if preset else [k for ks in unit.kept.values() for k in ks]
    for d in unit.deferred:
        ds = segs(d)
        if under(p, ds) and not any(under(p, segs(k)) and under(segs(k), ds) for k in kept):
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


# ----------------------------------------------------------------------------------------- the names at depth

@dataclass
class Later:
    """A later unit (S6.1, S17; ADD-A147): one `[[later]]` table of the units file. It holds, model-wide, its
    `conditions` and `directions`, the sizes beyond an element's default in force and the densities where `sizes`,
    and per catalogue entry its groups of names, `{entry: ((presets or None, {"variants": set, "states": set}),
    …)}`, a group on the presets it names, on every preset where it names none."""
    name: str
    kind: str
    enabled: bool
    conditions: tuple = ()
    directions: tuple = ()
    sizes: bool = False
    names: dict = field(default_factory=dict)

    def holds(self, entry: str, preset: str, kind: str, name: str) -> bool:
        """Whether it holds the entry's variant or state flag (`kind`, ENTRY_NAMES) on the preset."""
        return any((presets is None or preset in presets) and name in got.get(kind, ())
                   for presets, got in self.names.get(entry, ()))


def str_list(value) -> bool:
    return isinstance(value, list) and all(isinstance(x, str) and x for x in value)


def read_later(root: Path) -> tuple:
    """(the later units in the file's order, `[at_depth]`: {key prefix: the conditions at depth for its keys}) of
    scripts/widget_units.toml; InputError on any other shape: a unit's `name` (one per unit), `kind` (LATER_KINDS),
    `enabled`, `conditions`, `directions` and `sizes`, and each other key a catalogue entry's list of groups, each
    holding `variants`, `states` (ENTRY_NAMES) and `presets` alone."""
    p = root / UNITS
    try:
        data = tomllib.loads(p.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as err:
        raise InputError(f"{UNITS}: {err}") from err
    at_depth = data.get("at_depth", {})
    if not isinstance(at_depth, dict) or not all(str_list(v) for v in at_depth.values()):
        raise InputError(f"{UNITS}: `at_depth` is not a table of key prefixes to lists of conditions")
    tables = data.get("later", [])
    if not isinstance(tables, list) or not all(isinstance(t, dict) for t in tables):
        raise InputError(f"{UNITS}: `later` is not an array of [[later]] tables")
    out, seen = [], set()
    for t in tables:
        name, kind, enabled = t.get("name"), t.get("kind"), t.get("enabled")
        if not isinstance(name, str) or not name or name in seen:
            raise InputError(f"{UNITS}: a [[later]] table has no `name`, or one another has: {name!r}")
        seen.add(name)
        if kind not in LATER_KINDS or not isinstance(enabled, bool):
            raise InputError(f"{UNITS}: later unit `{name}`: `kind` is none of {LATER_KINDS}, or `enabled` no "
                             f"boolean")
        for k in ("conditions", "directions"):
            if not str_list(t.get(k, [])):
                raise InputError(f"{UNITS}: later unit `{name}`: `{k}` is not a list of names")
        if not isinstance(t.get("sizes", False), bool):
            raise InputError(f"{UNITS}: later unit `{name}`: `sizes` is no boolean")
        names = {}
        for entry, groups in t.items():
            if entry in ("name", "kind", "enabled", "conditions", "directions", "sizes"):
                continue
            if not isinstance(groups, list) or not all(
                    isinstance(g, dict) and g and set(g) <= {"presets", *ENTRY_NAMES} and all(
                        str_list(v) for v in g.values()) for g in groups):
                raise InputError(f"{UNITS}: later unit `{name}`: `{entry}` is not a list of groups holding "
                                 f"`presets`, `variants` and `states`")
            names[entry] = tuple((tuple(g["presets"]) if "presets" in g else None,
                                  {k: frozenset(g.get(k, ())) for k in ENTRY_NAMES}) for g in groups)
        out.append(Later(name, kind, enabled, tuple(t.get("conditions", ())), tuple(t.get("directions", ())),
                         t.get("sizes", False), names))
    return out, at_depth


def waiting_paths(root: Path) -> list:
    """[(path, unit)]: each path `[staged.<unit>]` stages and each `[deferred.<widget>]` defers (deferred_path()), with
    the later unit it waits for, in the file's order (read_units checks their shape; check_waiting() the units they
    name)."""
    try:
        data = tomllib.loads((root / UNITS).read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as err:
        raise InputError(f"{UNITS}: {err}") from err
    check_waiting(root, data)
    out = [(path, unit) for table in data.get("staged", {}).values() for path, unit in table.items()]
    return out + [(deferred_path(w, name), unit) for w, vs in data.get("deferred", {}).items()
                  for name, unit in vs.items()]


# A combination (S17 *State models*): its variants, its state flags after the exclusive groups' drop, its
# conditions, its size (None: it takes none; PENDING_SIZE: its default is pending, so no stated size holds) and
# density, its direction.
PENDING_SIZE = "<pending default>"
LONE = {"first": True, "last": True, "only": True, "parity": "odd"}   # an element with no sibling (S7)


@dataclass(frozen=True)
class Combo:
    variants: frozenset
    states: frozenset
    conditions: frozenset
    size: str | None = None
    density: str | None = None
    direction: str = "ltr"
    ancestors: tuple = ()      # (widget, part, surface, variants, states, sizes) of each, the outermost first
    siblings: bool = False     # inside a host, where siblings can stand in any position


def ancestors_of(context) -> tuple:
    """A state model's `axes.contexts` entry (one selector, or a list of them, the outermost first) as the ancestors
    of a combination drawn in it."""
    chain = context if isinstance(context, list) else [context]
    return tuple((e.get("widget"), e.get("part"), e.get("surface"), frozenset(listed(e, "variants")),
                  frozenset(listed(e, "states")), frozenset(listed(e, "sizes")))
                 for e in chain if isinstance(e, dict))


def context_entry_holds(e: dict, ancestors: tuple) -> bool:
    """Whether a `when`'s context entry holds among the ancestors (S7, S8): `scope = "any"`, any of them; a `nearest`
    scope, the innermost whose widget it names."""
    def matches(a):
        widget, part, surface, variants, states, sizes = a
        if any(k in e and e[k] != v for k, v in (("widget", widget), ("part", part), ("surface", surface))):
            return False
        return all(listed(e, k) <= have and not listed(e, "not_" + k) & have
                   for k, have in (("variants", variants), ("states", states), ("sizes", sizes)))
    scope = e.get("scope", "any")
    if isinstance(scope, dict):
        nearest = next((a for a in reversed(ancestors) if a[0] in listed(scope, "nearest")), None)
        return nearest is not None and matches(nearest)
    return any(matches(a) for a in ancestors)


def listed(when, k) -> set:
    v = when.get(k) if isinstance(when, dict) else None
    return {x for x in v if isinstance(x, str)} if isinstance(v, list) else set()


@dataclass(frozen=True)
class Match:
    """A `when` read once for combo_holds(): per kind its positive and negative names, whether it holds in no
    combination at all (`never`: a size class, which no unit draws), its `latest` flags, its context entries and
    negative ones, whether its position needs a sibling, its direction."""
    lists: tuple
    never: bool
    latest: tuple
    context: tuple = ()
    not_context: tuple = ()
    sibling: bool = False
    direction: str = "ltr"


MATCH_KINDS = ("conditions", "states", "variants", "sizes", "densities")


def compiled(when) -> Match:
    """The `when` as combo_holds() reads it."""
    when = when if isinstance(when, dict) else {}
    lists = tuple((frozenset(listed(when, k)), frozenset(listed(when, "not_" + k))) for k in MATCH_KINDS)
    sc = when.get("size_class") if isinstance(when.get("size_class"), dict) else {}
    pos = when.get("position")
    sibling = isinstance(pos, dict) and (any(k in pos and pos[k] != LONE[k] for k in LONE) or "previous" in pos
                                         or "next" in pos)
    latest = when.get("latest") if isinstance(when.get("latest"), dict) else {}
    entries = lambda k: tuple(e for e in when.get(k, []) if isinstance(e, dict)) \
        if isinstance(when.get(k), list) else ()  # noqa: E731
    return Match(lists, any(listed(sc, a) for a in ("horizontal", "vertical")), tuple(sorted(latest.items())),
                 entries("context"), entries("not_context"), sibling,
                 when.get("direction", "ltr") if isinstance(when.get("direction"), str) else "ltr")


def combo_holds(when, c: Combo, recency: dict) -> bool:
    """Whether a `when` (or its Match) holds in a combination (S11's matching): every positive list held, no negative
    one met, its `latest` flags present and made latest by one order (check_facts.orders_possible), its direction the
    combination's, its context entries held among the combination's ancestors and none of its negative ones, and a
    position other than an element's alone only where siblings can stand (inside a host)."""
    m = when if isinstance(when, Match) else compiled(when)
    if m.never:
        return False
    sizes = frozenset([c.size]) if c.size else frozenset()
    densities = frozenset([c.density]) if c.density else frozenset()
    for (pos, neg), have in zip(m.lists, (c.conditions, c.states, c.variants, sizes, densities)):
        if not pos <= have or neg & have:
            return False
    if m.latest:
        latest = dict(m.latest)
        if not (set(latest.values()) <= c.states and check_facts.orders_possible(latest, c.states, recency)):
            return False
    if m.direction != c.direction or (m.sibling and not c.siblings):
        return False
    if not all(context_entry_holds(e, c.ancestors) for e in m.context):
        return False
    return not any(context_entry_holds(e, c.ancestors) for e in m.not_context)


def condition_kind(condition: str) -> str:
    """A condition's kind, one value of which a combination holds at most (S7.1, S12)."""
    if condition in check_facts.CONTRAST:
        return "contrast"
    head, sep, rest = condition.partition(":")
    if head == "container_size":
        return "container_size:" + rest.split(":", 1)[0]
    return head if sep else condition


def preset_when(when, preset) -> dict:
    """A fact's `when` for a preset it holds for: `preset:<id>` out of `conditions`, every preset out of
    `not_conditions`, a list emptied so gone."""
    out = {}
    for k, v in (when if isinstance(when, dict) else {}).items():
        if k == "conditions" and isinstance(v, list):
            v = [c for c in v if c != f"preset:{preset}"]
        elif k == "not_conditions" and isinstance(v, list):
            v = [c for c in v if not (isinstance(c, str) and c.startswith("preset:"))]
        if isinstance(v, list) and not v:
            continue
        out[k] = v
    return out


def for_preset(when, preset) -> bool:
    """Whether a fact or state-model entry holds for the preset (S17 *Generations*)."""
    named = {c[7:] for c in listed(when, "conditions") if c.startswith("preset:")}
    return (not named or named == {preset}) and f"preset:{preset}" not in listed(when, "not_conditions")


def context_names(when) -> set:
    """The widgets a `when`'s positive lists name as an ancestor or a sibling (its `context` entries and their
    `nearest` scopes, its position's `among`, `previous` and `next`), the units a fact belongs to beside its key's
    (S17, ADD-A128); a negative list adds none."""
    out = set()
    for e in when.get("context", []) if isinstance(when.get("context"), list) else []:
        if isinstance(e, dict):
            out |= {e["widget"]} if isinstance(e.get("widget"), str) else set()
            scope = e.get("scope")
            out |= listed(scope, "nearest") if isinstance(scope, dict) else set()
    pos = when.get("position") if isinstance(when.get("position"), dict) else {}
    out |= listed(pos, "among")
    for k in ("previous", "next"):
        if isinstance(pos.get(k), dict) and isinstance(pos[k].get("widget"), str):
            out.add(pos[k]["widget"])
    return out


def entry_of(key: str) -> str:
    """The catalogue entry a key, a target path or a state model's `widget` names: `component.<key>` under
    `component.`, else its first segment (a widget, or `role`, `token`, `input`)."""
    seg = key.split(".")
    return f"{seg[0]}.{seg[1]}" if seg[0] == "component" and len(seg) > 1 else seg[0]


HOSTLESS = "identities"   # the unit of a fact drawn only in a context naming no widget (the identities, with hosts)
UNPLACED = "unplaced"   # a fact not of the enabled units that no unit's names place (S17: reported apart, under no unit)


class Depth:
    """The names at depth and the predicate of a fact's units (S17 *While the units are built*; ADD-A147), over the
    facts' state models (`items`, check_facts.load()'s) and the units file.

    The names at depth of a catalogue entry on a preset are every name of its state models and facts that no later
    unit not enabled holds there (the units file's `[[later]]`): every name lies in exactly one place, the entry's
    Depth line or one later unit, which scripts/check_facts.py checks, so the complement is the Depth line. A
    condition is at depth for a key unless a unit not enabled holds it and `[at_depth]` names it for none of the
    key's prefixes; the scheme, `contrast_high`, `full_keyboard_access` (ADD-A148) and the preset's default flavour no
    unit holds.

    The state model in force (S10's *Later names*): the default flavour's entry for the preset (its own over the
    platform's shared one) while the flavours unit is not enabled, each flavour's beside it under `flavor:<f>` once
    it is; an element holding only variants at depth, at its default size in force (its `sizes_by_variant` entry's
    default, else the widget's; none where it takes none) and no density while the sizes unit is not enabled. Its
    combinations at depth (combos()): each variant set the entry draws (check_facts.drawn_set_lists, the single
    variants and none) of variants at depth that `requires_variants` allows, each state combination it draws
    (check_facts.state_combinations) of flags at depth, each scheme it lists and each choice of one value or none per
    kind of its other axis conditions at depth, no context but the `axes.contexts` entries whose widgets are enabled,
    `ltr` and `rtl` where its unit is enabled, less those a `not_drawn` entry names; enumerated whole, once per
    (platform, entry, preset), with no cap.

    The predicate (of_enabled()): a fact is of the enabled units when its key lies in an enabled unit (not staged or
    deferred), every widget its positive lists name lies in one, and its `when` holds in at least one combination at
    depth (combo_holds()) where its part is drawn: the last presence fact of the part (`<widget>.<part>`) holding
    there is not `absent` or a `drawn_as`, a pending one counting as drawn; each distinct `when` decided once per
    (platform, entry or part, preset). A key of no widget (a colour role, an input, a token) holds where its
    conditions are at depth for it and of distinct kinds (one value of each per combination).

    A fact not of the enabled units is attributed by names, with no search (unit()): to the unit `[staged]`,
    `[deferred]` or its widget names where no enabled unit holds its key; to a widget its positive lists name that no
    enabled unit holds; where it names no context and its entry draws a variant it names, or its part, only inside a
    host context, to that host's unit (host_unit(); HOSTLESS for a context naming no widget); else, of the later
    units not enabled that hold a name its `when` lists positively (a variant, a state or keyed flag, a condition or
    flavour, a size no default in force, a density, a direction, a context naming no widget), to the last in the
    units file's order. A fact none of these places is UNPLACED, reported under no unit: a fact that holds nowhere is
    found when its units are enabled, by the predicate."""

    def __init__(self, root: Path, items, units=None, later=None):
        self.units = read_units(root) if units is None else units
        self.later, self.at_depth = read_later(root) if later is None else later
        self.waiting = waiting_paths(root)
        self.order = {u.name: i for i, u in enumerate(self.later)}
        self.models, self.presences = {}, {}
        for it in items:
            if getattr(it, "palette", False):
                continue
            d = it.data
            if it.table == "model" and isinstance(d.get("widget"), str):
                self.models.setdefault((it.platform, d["widget"]), []).append(d)
            elif it.table == "fact" and isinstance(d.get("key"), str) and (
                    d["key"].count(".") == entry_of(d["key"]).count(".") + 1):
                self.presences.setdefault((it.platform, d["key"]), []).append(d)
        self._space, self._combos, self._sets, self._presence, self._drawn = {}, {}, {}, {}, {}
        self._held, self._holds, self._hosts = {}, {}, {}

    # the names

    def off(self, entry: str, preset: str, kind: str, name: str) -> str | None:
        """The first later unit not enabled that holds the entry's variant or state flag on the preset; None where the
        name is at depth."""
        return next((u.name for u in self.later if not u.enabled and u.holds(entry, preset, kind, name)), None)

    def condition_off(self, condition: str, key: str) -> str | None:
        """The first later unit not enabled that holds the condition, unless `[at_depth]` names it for a prefix of
        `key`; None where it is at depth."""
        if condition.startswith(("scheme:", "preset:")):
            return None
        if any(condition in cs and under(segs(key), segs(prefix)) for prefix, cs in self.at_depth.items()):
            return None
        return next((u.name for u in self.later if not u.enabled and condition in u.conditions), None)

    def direction_off(self, direction: str) -> str | None:
        return next((u.name for u in self.later if not u.enabled and direction in u.directions), None)

    def sizes_off(self) -> str | None:
        """The sizes unit's name while it is not enabled."""
        return next((u.name for u in self.later if u.sizes and not u.enabled), None)

    def unit_off(self, name: str) -> str | None:
        """The later unit `name` while it is not enabled."""
        return next((u.name for u in self.later if u.name == name and not u.enabled), None)

    def held(self, path: str, preset: str | None = None) -> bool:
        """Whether a key or widget lies in an enabled unit on the preset (in_unit), an S9 input always."""
        if path.startswith("input."):
            return True
        if (path, preset) not in self._held:
            self._held[(path, preset)] = any(in_unit(path, "", u, preset) for u in self.units.values())
        return self._held[(path, preset)]

    # the state model in force

    def entries(self, platform: str, widget: str, preset: str) -> list:
        """[(the flavour condition or None, the entry)]: the default flavour's entry in force for the preset, its own
        over the platform's shared one, then each flavour's (space() reads them while the flavours unit is
        enabled)."""
        es = [d for d in self.models.get((platform, widget), [])
              if for_preset(d.get("when"), preset) and d.get("available", True) is not False]
        flavor = [(d, next((c for c in listed(d.get("when"), "conditions") if c.startswith("flavor:")), None))
                  for d in es]
        plain = [d for d, f in flavor if f is None]
        named = [d for d in plain if any(c.startswith("preset:") for c in listed(d.get("when"), "conditions"))]
        out = [(None, (named or plain)[-1])] if plain else []
        return out + [(f, d) for d, f in flavor if f is not None]

    def model(self, platform: str, widget: str, preset: str):
        """The default flavour's entry in force for the preset, or None."""
        found = self.entries(platform, widget, preset)
        return found[0][1] if found and found[0][0] is None else None

    def default_size(self, model, variants) -> str | None:
        """The default size in force of an element holding `variants` (S10): its first `sizes_by_variant` entry's
        whose variant it holds, else the widget's; None where it takes no size, PENDING_SIZE where the default is
        pending."""
        entry = next((e for e in model.get("sizes_by_variant", []) if isinstance(e, dict)
                      and e.get("variant") in variants and isinstance(e.get("sizes"), list)), None)
        sizes = entry["sizes"] if entry else model.get("sizes", [])
        if not sizes:
            return None
        default = entry.get("default_size", model.get("default_size")) if entry else model.get("default_size")
        return default if isinstance(default, str) else PENDING_SIZE

    def drawn_sets(self, model) -> list:
        """Every variant set the entry draws, whatever unit holds its names: none, each single variant, each listed set
        and each set its products give (check_facts.drawn_set_lists), less those `requires_variants` excludes
        (requires_ok) and those a `not_drawn` entry stating only variants removes; computed once per entry."""
        k = id(model)
        if k not in self._sets:
            variants = {v for v in model.get("variants", []) if isinstance(v, str)}
            sets_listed, sets_products = check_facts.drawn_set_lists(model)
            undrawn = [(listed(x, "variants"), listed(x, "not_variants")) for x in model.get("not_drawn", [])
                       if isinstance(x, dict) and x and set(x) <= {"variants", "not_variants"}]
            sets = {frozenset()} | {frozenset([v]) for v in variants} | sets_listed | sets_products
            self._sets[k] = (model, [s for s in sets if check_facts.requires_ok(model, s)
                                     and not any(a <= s and not n & s for a, n in undrawn)])
        return self._sets[k][1]

    def combos(self, platform: str, widget: str, preset: str) -> tuple:
        """([every combination at depth of the widget's state model in force on the preset], [each one's recency
        sets]): each choice of a variant set, a size, a density, a state combination, a condition vector, a context
        and a direction of space(), less those a `not_drawn` entry names, enumerated whole once (no cap)."""
        k = (platform, widget, preset)
        if k not in self._combos:
            cs, rs = [], []
            for rec, sets, states, vectors, sizes_of, densities, nd, contexts, directions in self.space(
                    platform, widget, preset):
                for s in sorted(sets, key=lambda x: (len(x), sorted(x))):
                    for size in sizes_of[s]:
                        for density in densities:
                            for st in sorted(states, key=lambda x: (len(x), sorted(x))):
                                for cv in vectors:
                                    for ancestors, siblings in contexts:
                                        for direction in directions:
                                            c = Combo(s, st, cv, size, density, direction, ancestors, siblings)
                                            if not any(combo_holds(x, c, rec) for x in nd):
                                                cs.append(c)
                                                rs.append(rec)
            self._combos[k] = (cs, rs)
        return self._combos[k]

    def space(self, platform: str, widget: str, preset: str) -> list:
        """Per entry in force (combos()), computed once: (its recency sets, its variant sets of names at depth, its
        state combinations so, its condition vectors, {variant set: its sizes in force}, its densities in force, its
        `not_drawn` entries as Matches, its contexts (ancestors, whether siblings stand there): none, and each
        `axes.contexts` entry whose widgets are enabled, its directions: `ltr`, and `rtl` where the unit holding it is
        enabled)."""
        k = (platform, widget, preset)
        if k in self._space:
            return self._space[k]
        out = []
        flavours_on = not any(not u.enabled and any(c.startswith("flavor:") for c in u.conditions)
                              for u in self.later)
        for flavor, model in self.entries(platform, widget, preset):
            if flavor is not None and (not flavours_on or self.condition_off(flavor, widget)):
                continue
            rec = check_facts.recency_of(model)
            variants = {v for v in model.get("variants", []) if isinstance(v, str)
                        and not self.off(widget, preset, "variants", v)}
            sets = [s for s in self.drawn_sets(model) if s <= variants]
            states = [frozenset(s) for s in check_facts.state_combinations(model)
                      if not any(self.off(widget, preset, "states", f) for f in s)]
            axes = model.get("axes") if isinstance(model.get("axes"), dict) else {}
            axis = [c for c in axes.get("conditions", []) if isinstance(c, str)]
            schemes = [c for c in axis if c.startswith("scheme:")] or ["scheme:light", "scheme:dark"]
            kinds = {}
            for c in axis:
                if not c.startswith("scheme:") and not self.condition_off(c, widget):
                    kinds.setdefault(condition_kind(c), []).append(c)
            vectors = [frozenset([s] + ([flavor] if flavor else [])) for s in schemes]
            for values in kinds.values():
                vectors = [v | extra for v in vectors for extra in [frozenset()] + [frozenset([x]) for x in values]]
            sized = self.sizes_off() is None
            densities = [None] + ([d for d in model.get("densities", []) if isinstance(d, str)] if sized else [])
            nd = [m for m in (compiled(x) for x in model.get("not_drawn", []) if isinstance(x, dict) and x)
                  if not m.never]
            sizes_of = {}
            for s in sets:
                if sized:
                    entry = next((e for e in model.get("sizes_by_variant", []) if isinstance(e, dict)
                                  and e.get("variant") in s and isinstance(e.get("sizes"), list)), None)
                    sizes = [z for z in (entry["sizes"] if entry else model.get("sizes", [])) if isinstance(z, str)]
                    sizes_of[s] = sizes or [None]
                else:
                    sizes_of[s] = [self.default_size(model, s)]
            chains = [c for c in axes.get("contexts", []) if isinstance(c, (dict, list))]
            held = [ancestors_of(c) for c in chains
                    if all(isinstance(a[0], str) and self.held(a[0]) for a in ancestors_of(c))]
            contexts = [((), False)] + [(a, True) for a in held if a]
            directions = ["ltr"] + [d for d in ("rtl",) if self.direction_off(d) is None]
            out.append((rec, sets, states, vectors, sizes_of, densities, nd, contexts, directions))
        self._space[k] = out
        return out

    def host_chains(self, platform: str, widget: str, preset: str) -> list:
        """The `axes.contexts` entries of the widget's default state model in force on the preset, in order."""
        model = self.model(platform, widget, preset) or {}
        axes = model.get("axes") if isinstance(model.get("axes"), dict) else {}
        return [c for c in axes.get("contexts", []) if isinstance(c, (dict, list))]

    def presence_facts(self, platform: str, part_key: str, preset: str) -> list:
        """[(its `when` for the preset read as a Match, the fact)] of the part's presence facts holding for the preset,
        in order (the accent run's left out: they apply in that run alone), computed once."""
        k = (platform, part_key, preset)
        if k not in self._presence:
            self._presence[k] = [(compiled(preset_when(d.get("when"), preset)), d)
                                 for d in self.presences.get((platform, part_key), [])
                                 if for_preset(d.get("when"), preset) and "scheme_file" not in d and "accent" not in d]
        return self._presence[k]

    @staticmethod
    def draws(fact: dict) -> bool:
        """Whether a presence fact leaves its part drawn: its value is not `absent` or a `drawn_as` (S6.3); a pending
        one does (S17)."""
        value = None if "pending" in fact else fact.get("value")
        return not (value == "absent" or isinstance(value, dict) and "drawn_as" in value)

    @staticmethod
    def drawn_in(facts: list, c: Combo, rec: dict) -> bool:
        """Whether the part resolves drawn in the combination: the last of its presence facts `facts` holding there
        draws it (draws()), or none holds there."""
        last = next((d for w, d in reversed(facts) if combo_holds(w, c, rec)), None)
        return last is None or Depth.draws(last)

    def drawn(self, platform: str, widget: str, part: str, preset: str) -> list:
        """The places in combos() of the combinations at depth where the part is drawn (drawn_in()), computed once."""
        k = (platform, part, preset)
        if k not in self._drawn:
            cs, rs = self.combos(platform, widget, preset)
            facts = self.presence_facts(platform, part, preset)
            self._drawn[k] = [i for i, (c, r) in enumerate(zip(cs, rs)) if self.drawn_in(facts, c, r)]
        return self._drawn[k]

    @staticmethod
    def part_of(key: str) -> str | None:
        """`<entry>.<part>` of a part's field's key (entry_of()); None for a widget value, a part's own key or a key of
        no widget."""
        entry = entry_of(key)
        seg = key[len(entry) + 1:].split(".")
        return f"{entry}.{seg[0]}" if len(seg) > 1 and seg[0] != "values" else None

    def holds_at(self, platform: str, key: str, when: dict, preset: str) -> bool:
        """Whether a fact of a widget's key (its `when` for the preset) holds in a combination at depth (combos()) where
        its part is drawn (drawn()): every such combination read, no cap; each distinct `when` decided once per
        (platform, entry or part, preset)."""
        widget, part = entry_of(key), self.part_of(key)
        k = (platform, preset, part or widget, json.dumps(when, sort_keys=True))
        if k not in self._holds:
            m = compiled(when)
            cs, rs = self.combos(platform, widget, preset)
            places = self.drawn(platform, widget, part, preset) if part else range(len(cs))
            self._holds[k] = not m.never and any(combo_holds(m, cs[i], rs[i]) for i in places)
        return self._holds[k]

    def key_unit(self, key: str) -> str:
        """The unit a key no enabled unit holds waits for: the one the longest `[staged]` or `[deferred]` path above it
        names, else its entry (entry_of(): an entry of a unit not enabled)."""
        p = segs(key)
        found = [(len(segs(path)), unit) for path, unit in self.waiting if under(p, segs(path))]
        return max(found, key=lambda x: x[0])[1] if found else entry_of(key)

    def of_enabled(self, platform: str, key: str, when, preset: str) -> bool:
        """Whether a fact of the platform holding for the preset (its `when` as written) is of the enabled units (the
        class's docstring): its key and the widgets its positive lists name held, its names at depth, and its `when`
        holding in a combination at depth where its part is drawn (holds_at())."""
        when = preset_when(when, preset)
        if not self.held(key, preset) or not all(self.held(w) for w in context_names(when)):
            return False
        widget = entry_of(key)
        if any(self.condition_off(c, key) for c in listed(when, "conditions")):
            return False
        if widget in ("role", "token", "input"):
            kinds = [condition_kind(c) for c in listed(when, "conditions")]
            return len(kinds) == len(set(kinds))
        latest = when.get("latest") if isinstance(when.get("latest"), dict) else {}
        if any(self.off(widget, preset, "variants", v) for v in listed(when, "variants")) or any(
                self.off(widget, preset, "states", s) for s in listed(when, "states") | set(latest.values())):
            return False
        return self.holds_at(platform, key, when, preset)

    def unit(self, platform: str, key: str, when, preset: str) -> str | None:
        """None where a fact of the platform holding for the preset (its `when` as written) is of the enabled units
        (of_enabled()); else the unit it waits for, by names (the class's docstring), or UNPLACED."""
        when = preset_when(when, preset)
        if not self.held(key, preset):
            return self.key_unit(key)
        for w in sorted(context_names(when)):
            if not self.held(w):
                return w
        if self.of_enabled(platform, key, when, preset):
            return None
        widget = entry_of(key)
        entry = widget not in ("role", "token", "input")
        # the host rule runs before the attribution by names, so a fact that also needs a later unit's name is the
        # host's; whether the attribution over the drawn sets keeps this order is ledger queue #289's task
        if entry and not (isinstance(when.get("context"), list) and when["context"]):
            host = self.host_unit(platform, key, listed(when, "variants"), preset)
            if host:
                return host
        found = [self.condition_off(c, key) for c in listed(when, "conditions")]
        if entry:
            latest = when.get("latest") if isinstance(when.get("latest"), dict) else {}
            found += [self.off(widget, preset, "variants", v) for v in listed(when, "variants")]
            found += [self.off(widget, preset, "states", s) for s in listed(when, "states") | set(latest.values())]
            if isinstance(when.get("direction"), str):
                found.append(self.direction_off(when["direction"]))
            if self.sizes_off() and (listed(when, "densities") or listed(when, "sizes") - self.size_defaults(
                    platform, widget, preset, when)):
                found.append(self.sizes_off())
            contexts = when.get("context") if isinstance(when.get("context"), list) else []
            if any(isinstance(e, dict) and not context_names({"context": [e]}) for e in contexts):
                found.append(self.unit_off(HOSTLESS))
        found = [u for u in found if u]
        return max(found, key=self.order.get) if found else UNPLACED

    def size_defaults(self, platform: str, widget: str, preset: str, when: dict) -> set:
        """The sizes the sizes unit does not hold for a `when`: the default in force (default_size()) of each variant set
        holding its variants that its flavour's entry draws (the default flavour's where it names none)."""
        flavour = next((c for c in sorted(listed(when, "conditions")) if c.startswith("flavor:")), None)
        model = next((d for f, d in self.entries(platform, widget, preset) if f == flavour), None)
        if model is None:
            return set()
        variants = listed(when, "variants")
        return {self.default_size(model, s) for s in self.drawn_sets(model) if variants <= s}

    def host_unit(self, platform: str, key: str, variants: set, preset: str) -> str | None:
        """The unit of the host inside which alone the key's entry draws one of `variants` (the first in sorted order so
        drawn, variant_host()), else its part (part_host()); None where it draws them outside a host."""
        widget, part = entry_of(key), self.part_of(key)
        for v in sorted(variants):
            host = self.variant_host(platform, widget, v, preset)
            if host:
                return host
        return self.part_host(platform, part, preset) if part else None

    def variant_host(self, platform: str, widget: str, variant: str, preset: str) -> str | None:
        """Where the widget's default state model in force on the preset draws no variant set holding `variant`
        (drawn_sets()) outside a host context, the unit of the first `axes.contexts` entry, of those whose widgets are
        not all enabled, inside which it draws one: the entry's innermost widget, HOSTLESS where it names none; else
        None. Read from the state model's structure alone (its sets and its `not_drawn` entries stating only variants
        and contexts), once per (platform, entry, preset, variant)."""
        k = (platform, widget, variant, preset)
        if k not in self._hosts:
            model = self.model(platform, widget, preset)
            found = None
            if model is not None:
                sets = [s for s in self.drawn_sets(model) if variant in s]
                gone = [compiled(x) for x in model.get("not_drawn", []) if isinstance(x, dict) and x
                        and set(x) <= {"variants", "not_variants", "context", "not_context"}]

                def drawn(ancestors):
                    return any(not any(combo_holds(m, Combo(s, frozenset(), frozenset(), ancestors=ancestors,
                                                            siblings=bool(ancestors)), {}) for m in gone)
                               for s in sets)

                if sets and not drawn(()):
                    for chain in self.host_chains(platform, widget, preset):
                        ancestors = ancestors_of(chain)
                        if all(isinstance(a[0], str) and self.held(a[0]) for a in ancestors) or not drawn(ancestors):
                            continue
                        named = [a[0] for a in ancestors if isinstance(a[0], str)]
                        found = named[-1] if named else HOSTLESS
                        break
            self._hosts[k] = found
        return self._hosts[k]

    def part_host(self, platform: str, part: str, preset: str) -> str | None:
        """Where the part's presence facts draw it only inside a host context (after the last one with no `when` that
        does not draw it, draws(), every one that draws it names a context, and one does): the widget the first such
        fact's first context entry names, HOSTLESS where it names none; else None."""
        facts = [d for _, d in self.presence_facts(platform, part, preset)]
        base = max((i for i, d in enumerate(facts) if not preset_when(d.get("when"), preset) and not self.draws(d)),
                   default=None)
        if base is None:
            return None
        contexts = [preset_when(d.get("when"), preset).get("context") for d in facts[base + 1:] if self.draws(d)]
        if not contexts or not all(isinstance(c, list) and c for c in contexts):
            return None
        first = contexts[0][0]
        return first["widget"] if isinstance(first, dict) and isinstance(first.get("widget"), str) else HOSTLESS


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


def carried_rows(ctx, items=None) -> dict:
    """{platform: {row: the keys of the facts and state-model entries whose `rows` list it}}: a fact's `key` (a
    pending fact's too), a state-model entry's `widget`; `items` check_facts.load()'s, loaded where not given."""
    if items is None:
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


SELECTOR_NAME = re.compile(r"(?<![!\w])(?:variants \{(?P<vs>[^}]*)\}|variant (?P<v>@?\w+)|cond (?P<c>[\w:@]+)"
                           r"|size (?P<z>\w+)|density (?P<d>\w+)|dir (?P<dir>\w+)|\{(?P<s>[^}]*)\})"
                           r"|^(?P<bare>@?[a-z_][a-z0-9_]*)$")


def selector_alternatives(target: str) -> list:
    """The names of each alternative of a trace target's selector (the catalogue's *Selector notation*: ` / ` between
    alternatives, a bare name continuing a variant), the target cell up to its first backtick: [{kind: {names}}],
    kinds `variants`, `states`, `conditions`, `sizes`, `densities`, `directions`; the names of an ancestor (after
    `context`) left out, its widget's unit deciding it (line_in)."""
    code = target.split("`", 1)[0]
    if " @ " not in code:
        return [{}]
    selector = code.split(" @ ", 1)[1]
    out = []
    for alt in selector.split(" / "):
        own = alt.split(" context ", 1)[0].split("context ", 1)[0] if not alt.startswith("context") else ""
        names = {k: set() for k in ("variants", "states", "conditions", "sizes", "densities", "directions")}
        for m in SELECTOR_NAME.finditer(own.strip()):
            if m["vs"]:
                names["variants"] |= {x.strip() for x in m["vs"].split(",")}
            for k, g in (("variants", "v"), ("variants", "bare"), ("conditions", "c"), ("sizes", "z"),
                         ("densities", "d"), ("directions", "dir")):
                if m[g]:
                    names[k].add(m[g])
            if m["s"]:
                names["states"] |= {x.strip() for x in m["s"].split(",")}
        out.append(names)
    return out


def line_later(line: Line, depth, presets: list) -> list:
    """Per alternative of the line's selector, the later units not enabled holding its names on every preset of the
    line's platform (`presets`; a name at depth on one of them is at depth for the line): its variants and state
    flags for the target's entry, its conditions for the target's path (Depth), a size no default in force of the
    entry's state model on those presets, a density, a direction."""
    paths = target_paths(line.target)
    path = paths[0] if paths else ""
    entry = entry_of(path)
    defaults = {c.size for p in presets for c in depth.combos(line.platform, entry, p)[0]} if presets else set()
    out = []
    for names in selector_alternatives(line.target):
        units = set()
        for kind in ("variants", "states"):
            for n in names.get(kind, ()):
                held = [depth.off(entry, p, kind, n) for p in presets]
                if held and all(held):
                    units.add(held[0])
        units |= {u for c in names.get("conditions", ()) if (u := depth.condition_off(c, path))}
        if names.get("sizes") and depth.sizes_off() and not names["sizes"] & defaults:
            units.add(depth.sizes_off())
        if names.get("densities") and depth.sizes_off():
            units.add(depth.sizes_off())
        units |= {u for d in names.get("directions", ()) if (u := depth.direction_off(d))}
        out.append(units)
    return out


def check(ctx, units, enabled=(), assumed=(), later=None):
    """(findings as (platform, row, target, what), {platform: Tally}, {platform: existing presets}).
    `units` None checks every line; `enabled` are the other enabled units (a context widget may lie there);
    `assumed` the presets checked as if they existed (`--preset`). With `units`, a line is checked where one
    alternative of its selector names only names at depth (line_later()), or, with `later` (a later unit's name), where
    one names names of that unit, the last in the file's order of the units it names (S17, ADD-A147)."""
    lines = read_lines(ctx.root)
    items, _, _ = check_facts.load(ctx)
    carried = carried_rows(ctx, items)
    depth = Depth(ctx.root, items) if units is not None else None
    existing = check_facts.existing_presets(ctx.root) | set(assumed)
    widgets = set(ctx.entries)
    presets = {p: sorted(set(ps) & existing) for p, ps in check_facts.PLATFORMS.items()}
    findings, tallies = [], {p: Tally() for p in check_facts.PLATFORMS}
    for line in lines:
        if units is not None and not line_in(line, list(units), list(enabled), widgets):
            continue
        if depth is not None:
            alts = line_later(line, depth, list(check_facts.PLATFORMS[line.platform]))
            last = (lambda a: max(a, key=depth.order.get)) if later else None
            if not any((later in a and last(a) == later) if later else not a for a in alts):
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


def facts_report(ctx) -> list:
    """`--facts`' lines (S17's predicate, ADD-A147; informational, no finding): per bundled preset, the facts of the
    enabled units and how many are pending, per later unit the facts it holds (Depth.unit(); a key no enabled unit
    holds by the unit `[staged]`, `[deferred]` or its widget names) and how many are unplaced; per facts directory the
    facts of the enabled units on any of its presets and how many are pending, and the facts unplaced on one of them,
    each fact once; every unplaced fact by preset, place, key and `when` for the preset (S17: reported under no
    unit); and, for the facts gate's cross-check, one line per fact and preset it holds for (the accent run's left
    out), sorted by preset, file and line: `fact<TAB><preset><TAB><file>:<line of its first line><TAB><place>`, the
    file relative to the repository root and the place `enabled`, `unplaced` or the unit Depth.unit() names."""
    items, _, _ = check_facts.load(ctx)
    depth = Depth(ctx.root, items)
    existing = check_facts.existing_presets(ctx.root)
    out, unplaced, places = [], [], []
    for platform, presets in check_facts.PLATFORMS.items():
        on_path, lost = {}, set()
        for preset in [p for p in presets if p in existing]:
            units, enabled, pending, alone = {}, 0, 0, 0
            for it in items:
                d = it.data
                if (it.table != "fact" or it.platform != platform or it.palette or "scheme_file" in d
                        or "accent" in d or not for_preset(d.get("when"), preset)):
                    continue
                unit = depth.unit(platform, d["key"], d.get("when"), preset)
                place = (it.file, it.line)
                on_path[place] = on_path.get(place, False) or unit is None
                places.append((preset, Path(it.file).as_posix(), it.line,
                               "enabled" if unit is None else "unplaced" if unit == UNPLACED else unit))
                if unit is None:
                    enabled += 1
                    pending += "pending" in d
                elif unit == UNPLACED:
                    alone += 1
                    lost.add(place)
                    when = json.dumps(preset_when(d.get("when"), preset), sort_keys=True)
                    unplaced.append(f"{preset}\t{it.file}:{it.line}\t{d['key']}\t{when}")
                else:
                    units[unit] = units.get(unit, 0) + 1
            listing = ", ".join(f"{u} {n}" for u, n in sorted(units.items(), key=lambda x: -x[1]))
            out.append(f"{preset}: {enabled} facts of the enabled units ({pending} pending); later units: {listing}; "
                       f"{alone} unplaced")
            # the machine-read form the facts gate compares its own report with (NT_FACTS_GATE_UNITS)
            out.append(f"unit\t{preset}\t(enabled)\t{enabled}\t{pending}")
            out += [f"unit\t{preset}\t{u}\t{n}\t0" for u, n in sorted(units.items())]
            if alone:
                out.append(f"unit\t{preset}\t(unplaced)\t{alone}\t0")
        pending_places = {(it.file, it.line) for it in items if it.platform == platform and "pending" in it.data}
        pending = sum(1 for place, held in on_path.items() if held and place in pending_places)
        if on_path:
            out.append(f"docs/facts/{platform}: {sum(on_path.values())} facts of the enabled units ({pending} pending), "
                       f"{len(lost)} unplaced")
    return (out + [f"unplaced: {s}" for s in unplaced] + [f"{len(unplaced)} unplaced facts (per preset)"]
            + [f"fact\t{preset}\t{file}:{line}\t{place}" for preset, file, line, place in sorted(places)])


def report(tallies, presets, scope, assumed=()):
    out = []
    for p, t in tallies.items():
        named = [f"{x} (assumed)" if x in assumed else x for x in presets[p]]
        state = f"{', '.join(named)} exist{'s' if len(presets[p]) == 1 else ''}" if presets[p] else \
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


# The fixtures' units file: the slice as the repository's file states it, the scroll behaviour staged to the
# containers unit (the plan's *Every unit listed*) so that no fixture needs the catalogue.
SELF_TEST_UNITS = '''slice = ["button"]
[prefixes]
slice = ["role", "token.motion", "token.caret", "token.behavior", "token.text", "token.icons", "token.focus",
         "token.elevation", "token.size_classes",
         "token.params.pointer_repeat", "token.params.color", "token.params.opacity"]
[staged.slice]
"token.behavior.scroll" = "containers"
[deferred.button]
menu_placement = "placement"
scaffold_placement = "placement"
[hand_written]
placement = false
[unit_names]
m3a = ["containers"]
'''


# The depth's fixture (synthetic names and values): a units file with later units, a state model and facts.
DEPTH_UNITS = '''slice = ["button"]
[prefixes]
slice = ["role", "token.text"]
[staged.slice]
"token.text.keyed" = "text_styles"
"token.text.keyed.*.weight" = "accessibility_text"
[kept.slice]
"adwaita" = ["token.text.keyed.body"]
[deferred.button]
"body.transitions" = "motion"
[at_depth]
"role.selection" = ["window_inactive"]
[[later]]
name = "styles"
kind = "depth"
enabled = false
button = [{ variants = ["flat"] }]
[[later]]
name = "states"
kind = "depth"
enabled = false
button = [{ states = ["open"] }]
[[later]]
name = "sizes"
kind = "depth"
enabled = false
sizes = true
[[later]]
name = "window_activity"
kind = "depth"
enabled = false
conditions = ["window_inactive"]
[[later]]
name = "identities"
kind = "depth"
enabled = false
button = [{ variants = ["back"] }]
[[later]]
name = "ipad"
kind = "depth"
enabled = false
conditions = ["idiom:pad"]
button = [{ presets = ["adwaita"], states = ["hover"] }]
[[later]]
name = "text_styles"
kind = "parking"
enabled = false
[[later]]
name = "right_to_left"
kind = "depth"
enabled = false
directions = ["rtl"]
[[later]]
name = "accessibility_text"
kind = "depth"
enabled = false
[[later]]
name = "motion"
kind = "depth"
enabled = false
'''
DEPTH_TOOLBAR = {"scope": "any", "widget": "toolbar", "part": "bar"}
DEPTH_OSD = {"scope": "any", "variants": ["@osd"]}
DEPTH_MODEL = {"widget": "button", "available": True,
               "variants": ["flat", "toggle", "primary", "tool", "back", "@inner", "@nowhere"],
               "sizes": ["regular", "small"], "default_size": "regular", "densities": [],
               "axes": {"states": ["hover", "pressed", "open"],
                        "conditions": ["scheme:light", "scheme:dark", "window_inactive", "idiom:pad"],
                        "contexts": [DEPTH_TOOLBAR, DEPTH_OSD]},
               "exclusive": [], "variant_sets": [["flat", "toggle"]],
               "not_drawn": [{"variants": ["toggle"], "not_variants": ["flat"]},
                             {"variants": ["tool"], "not_context": [DEPTH_TOOLBAR]},
                             {"variants": ["back"], "not_context": [DEPTH_TOOLBAR]},
                             {"variants": ["@inner"], "not_context": [DEPTH_OSD]},
                             {"variants": ["@nowhere"]}]}
DEPTH_CASES = [   # (preset, key, when, the unit; None: of the enabled units)
    ("adwaita", "button.body.fill", {}, None),
    ("adwaita", "button.body.fill", {"variants": ["primary"], "states": ["pressed"]}, None),
    ("adwaita", "button.body.fill", {"variants": ["flat"]}, "styles"),
    # a fact needing two later units is the last's in the file's order
    ("adwaita", "button.body.fill", {"states": ["open"], "variants": ["flat"]}, "states"),
    ("adwaita", "button.body.fill", {"variants": ["flat"], "sizes": ["large"]}, "sizes"),
    ("adwaita", "button.body.fill", {"conditions": ["idiom:pad", "window_inactive"]}, "ipad"),
    ("adwaita", "button.body.fill", {"states": ["hover"]}, "ipad"),
    ("kde-breeze", "button.body.fill", {"states": ["hover"]}, None),
    ("adwaita", "button.body.fill", {"not_states": ["open"], "not_variants": ["flat"]}, None),
    ("adwaita", "button.body.fill", {"sizes": ["small"]}, "sizes"),
    ("adwaita", "button.body.fill", {"sizes": ["regular"]}, None),
    ("adwaita", "button.body.fill", {"conditions": ["window_inactive"]}, "window_activity"),
    ("adwaita", "button.body.fill", {"direction": "rtl", "variants": ["flat"]}, "right_to_left"),
    # names at depth alone, drawn in no combination at depth: no unit's name places them
    ("adwaita", "button.body.fill", {"variants": ["toggle"]}, "unplaced"),
    ("adwaita", "button.body.fill", {"states": ["no_such_flag"]}, "unplaced"),
    ("adwaita", "button.body.fill", {"variants": ["@nowhere"]}, "unplaced"),
    ("adwaita", "role.window.background", {"conditions": ["scheme:light", "scheme:dark"]}, "unplaced"),
    # a variant drawn only inside a host: the host's unit, before its own names' (a context naming no widget:
    # the identities); a fact naming a context: its widget's
    ("adwaita", "button.body.fill", {"variants": ["tool"]}, "toolbar"),
    ("adwaita", "button.body.fill", {"variants": ["tool"], "conditions": ["window_inactive"]}, "toolbar"),
    ("adwaita", "button.body.fill", {"variants": ["back"]}, "toolbar"),
    ("adwaita", "button.body.fill", {"variants": ["@inner"]}, "identities"),
    ("adwaita", "button.body.fill", {"variants": ["primary"], "context": [DEPTH_TOOLBAR]}, "toolbar"),
    ("adwaita", "button.body.fill", {"context": [DEPTH_OSD]}, "identities"),
    # a part drawn only inside a host is the host's; one drawn only under a later variant names no unit
    ("adwaita", "button.badge.fill", {}, "toolbar"),
    ("adwaita", "button.mark.glyph", {}, "unplaced"),
    ("adwaita", "button.mark.glyph", {"variants": ["flat"]}, "styles"),
    ("adwaita", "button.body.transitions", {}, "motion"),
    ("adwaita", "role.selection.background", {"conditions": ["window_inactive"]}, None),
    ("adwaita", "role.window.background", {"conditions": ["window_inactive"]}, "window_activity"),
    ("adwaita", "role.window.background", {"not_conditions": ["window_inactive"]}, None),
    ("adwaita", "token.text.keyed.body.size", {}, None),
    ("adwaita", "token.text.keyed.body.weight", {}, "accessibility_text"),
    ("kde-breeze", "token.text.keyed.body.size", {}, "text_styles"),
    ("adwaita", "token.text.keyed.title.size", {}, "text_styles"),
]


def depth_cases(base: Path) -> list:
    """The depth's predicate and the trace lines' selector names (S17, ADD-A147) on DEPTH_UNITS."""
    from types import SimpleNamespace
    failures = []
    root = base / "depth"
    (root / UNITS).parent.mkdir(parents=True, exist_ok=True)
    (root / UNITS).write_text(DEPTH_UNITS, encoding="utf-8")
    items = [SimpleNamespace(table="model", platform=p, data=DEPTH_MODEL) for p in ("gnome", "kde")]
    items += [SimpleNamespace(table="fact", platform=p, data={"key": "button.mark", "value": "absent"})
              for p in ("gnome", "kde")]
    items += [SimpleNamespace(table="fact", platform=p, data={"key": "button.mark", "value": "present",
                                                              "when": {"variants": ["flat"]}})
              for p in ("gnome", "kde")]
    items += [SimpleNamespace(table="fact", platform=p, data={"key": "button.badge", "value": "absent"})
              for p in ("gnome", "kde")]
    items += [SimpleNamespace(table="fact", platform=p, data={"key": "button.badge", "pending": "synthetic",
                                                              "when": {"context": [DEPTH_TOOLBAR]}})
              for p in ("gnome", "kde")]
    depth = Depth(root, items)
    platform = {"adwaita": "gnome", "kde-breeze": "kde"}
    for preset, key, when, want in DEPTH_CASES:
        got = depth.unit(platform[preset], key, when, preset)
        if got != want:
            failures.append(f"depth: {preset} {key} {when}: got {got!r}, want {want!r}")
        if depth.of_enabled(platform[preset], key, when, preset) != (want is None):
            failures.append(f"depth: of_enabled {preset} {key} {when} disagrees with unit()")
    combos = depth.combos("gnome", "button", "adwaita")[0]
    if {c.size for c in combos} != {"regular"} or any(c.variants & {"flat", "tool", "back", "@inner", "@nowhere"}
                                                     or c.states & {"open", "hover"} or c.ancestors
                                                     or "idiom:pad" in c.conditions for c in combos):
        failures.append(f"depth: the combinations at depth hold a name beyond it: {combos[:4]}")
    for target, want in (
            ("button.body.fill @ {hover} variant flat", [{"variants": {"flat"}, "states": {"hover"}}]),
            ("button.body.fill @ variant flat / @gray / variants {a,b} cond window_inactive size mini",
             [{"variants": {"flat"}}, {"variants": {"@gray"}},
              {"variants": {"a", "b"}, "conditions": {"window_inactive"}, "sizes": {"mini"}}]),
            ("button.body.fill @ variant tool context toolbar.bar {hover}", [{"variants": {"tool"}}]),
            ("button.body.fill", [{}])):
        got = [{k: v for k, v in alt.items() if v} for alt in selector_alternatives(target)]
        if got != want:
            failures.append(f"selector_alternatives({target!r}): got {got}, want {want}")
    line = Line("gnome", "X", "button.body.fill @ {hover} / variant flat", "rule")
    if line_later(line, depth, ["adwaita"]) != [{"ipad"}, {"styles"}] or line_later(
            line, depth, ["adwaita", "kde-breeze"]) != [set(), {"styles"}]:
        failures.append(f"line_later: got {line_later(line, depth, ['adwaita'])}")
    # a key under `component.` is of the entry `component.<key>`: its state model, its parts' presence, its later names
    # and, where no enabled unit holds it, the unit it waits for
    units = DEPTH_UNITS.replace('slice = ["button"]', 'slice = ["button", "component.led"]', 1).replace(
        'button = [{ variants = ["flat"] }]', 'button = [{ variants = ["flat"] }]\n"component.led" = [{ variants = '
                                              '["blink"] }]', 1)
    (root / UNITS).write_text(units, encoding="utf-8")
    led = {"widget": "component.led", "available": True, "variants": ["blink"], "sizes": [], "densities": [],
           "axes": {"states": ["hover"], "conditions": ["scheme:light", "scheme:dark"], "contexts": []},
           "exclusive": [], "not_drawn": []}
    items = [SimpleNamespace(table="model", platform="kde", data=led),
             SimpleNamespace(table="fact", platform="kde", data={"key": "component.led.lamp", "value": "absent"}),
             SimpleNamespace(table="fact", platform="kde", data={"key": "component.led.lamp", "value": "present",
                                                                 "when": {"variants": ["blink"]}})]
    depth = Depth(root, items)
    for key, when, want in (("component.led.body.fill", {}, None),
                            ("component.led.body.fill", {"states": ["hover"]}, None),
                            ("component.led.body.fill", {"variants": ["blink"]}, "styles"),
                            ("component.led.lamp.fill", {}, "unplaced"),
                            ("component.led.lamp.fill", {"variants": ["blink"]}, "styles"),
                            ("component.dock.body.fill", {}, "component.dock")):
        got = depth.unit("kde", key, when, "kde-breeze")
        if got != want or depth.of_enabled("kde", key, when, "kde-breeze") != (want is None):
            failures.append(f"depth: a component's key {key} {when}: got {got!r}, want {want!r}")
    for key, want in (("component.led.lamp.fill", "component.led.lamp"), ("component.led.values.x", None),
                      ("button.body.fill", "button.body"), ("component.led.lamp", None)):
        if Depth.part_of(key) != want:
            failures.append(f"depth: part_of({key!r}): got {Depth.part_of(key)!r}, want {want!r}")
    for bad in ('[[later]]\nname = "x"\nkind = "other"\nenabled = false\n',
                '[[later]]\nname = "x"\nkind = "depth"\nenabled = false\nbutton = ["flat"]\n',
                '[[later]]\nname = "x"\nkind = "depth"\nenabled = false\n[[later]]\nname = "x"\nkind = "depth"\n'
                'enabled = false\n', '[at_depth]\n"role" = "window_inactive"\n'):
        (root / UNITS).write_text('slice = ["button"]\n' + bad, encoding="utf-8")
        try:
            read_later(root)
            failures.append(f"read_later accepted {bad!r}")
        except InputError:
            pass
    return failures


def write_fixture(root: Path, presets=(), units=SELF_TEST_UNITS, extra=None):
    files = {f"docs/{check_spec_trace.MAP.name}": SELF_TEST_MAP,
             "docs/todo_v0.7.1_model-completeness-gap-map-gnome.md": SELF_TEST_GNOME_MAP,
             "docs/todo_v0.7.1_model-completeness-spec-trace-gnome.md": SELF_TEST_GNOME_TRACE}
    for slug, row in SELF_TEST_PLATFORMS.items():
        files[f"docs/todo_v0.7.1_model-completeness-gap-map-{slug}.md"] = ""
        files[f"docs/todo_v0.7.1_model-completeness-spec-trace-{slug}.md"] = SELF_TEST_OTHER_TRACE.format(row=row)
    files.update(SELF_TEST_FACTS)
    files.update(extra or {})
    for p in presets:
        files[f"{check_facts.PRESET_DIR.as_posix()}/{p}.toml"] = ""
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
                               sources_text=check_facts.SELF_TEST_SOURCES, rows=check_facts.SELF_TEST_ROWS,
                               evidence=root)


def self_test() -> int:
    failures = []
    with tempfile.TemporaryDirectory() as tmp:
        base = Path(tmp)
        n = [0]

        def run(presets=(), units=SELF_TEST_UNITS, unit=None, extra=None, every=False, assumed=()):
            root = base / f"case{n[0]}"
            n[0] += 1
            write_fixture(root, presets, units, extra)
            ctx = fixture_context(root)
            known = read_units(root)
            chosen = None if every else ([known[unit]] if unit else list(known.values()))
            others = [u for k, u in known.items() if unit and k != unit]
            found, tallies, pre = check(ctx, chosen, others, assumed)
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
        # --preset: a preset that does not exist checked as if it did, the report saying so
        found, tallies, pre = run(assumed=("adwaita",))
        line = report(tallies, pre, "slice", ("adwaita",))[0]
        if rows_of(found) != want or len(found) != 3 or not line.startswith("gnome (adwaita (assumed) exists): 5 "):
            failures.append(f"--preset adwaita: got {found}, {line!r}")
        with contextlib.redirect_stderr(io.StringIO()):
            if main(["check_facts_trace.py", "--preset", "no-such-preset"]) != 2 or main(
                    ["check_facts_trace.py", "--preset"]) != 2 or main(["check_facts_trace.py", "--unit"]) != 2:
                failures.append("--preset: an unknown preset or a wrong command line is not exit 2")
        # a fact carrying the row passes it; the imported folder carries nothing
        fact = SELF_TEST_FACTS["docs/facts/gnome/button.toml"]
        extra = {"docs/facts/gnome/tokens.toml": "\n".join(
            fact.replace('"button.body.fill"', f'"{k}"').replace('"GN-1.1"', f'"{r}"') for k, r in (
                ("token.motion.durations.button", "GN-1.3"), ("token.behavior.scroll_lines_per_step", "GN-1.5"),
                ("token.params.opacity.disabled", "GN-1.9†")))}
        found, _, _ = run(presets=("adwaita",), extra=extra)
        if found:
            failures.append(f"carried by a fact: got {found}")
        # --facts' line per fact and preset, sorted by preset, file and line (a line number as a number): a fact of the
        # enabled units, one a staging names a unit for, one unplaced, one holding for two presets
        tokens = "\n".join(fact.replace('"button.body.fill"', f'"{k}"') for k in (
            "token.behavior.scroll.deceleration", "token.motion.durations.button"))
        root = base / "facts-lines"
        write_fixture(root, presets=("adwaita", "macos-tahoe", "macos-monterey"),
                      extra={"docs/facts/gnome/tokens.toml": tokens})
        got = [x for x in facts_report(fixture_context(root)) if x.startswith("fact\t")]
        want = ["fact\tadwaita\tdocs/facts/gnome/button.toml:2\tenabled",
                "fact\tadwaita\tdocs/facts/gnome/roles.toml:2\tenabled",
                "fact\tadwaita\tdocs/facts/gnome/tokens.toml:2\tcontainers",
                "fact\tadwaita\tdocs/facts/gnome/tokens.toml:13\tenabled",
                "fact\tmacos-monterey\tdocs/facts/macos/button.toml:2\tunplaced",
                "fact\tmacos-tahoe\tdocs/facts/macos/button.toml:2\tunplaced"]
        if got != want:
            failures.append(f"--facts' fact lines: got {got}, want {want}")
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
                ("[declare.flavor.expressive.widget.button] sizes_by_variant: @fab default_size regular",
                 {"button"}, True),
                ("[declare.flavor.expressive.widget.button] sizes_by_variant: @fab default_size regular",
                 {"toolbar"}, False),
                ("[declare.widget.button] sizes_by_variant: @pill default_size regular", {"button"}, True),
                ("[declare.widget.button] sizes_by_variant: @pill default_size regular", {"toolbar"}, False),
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
        # the units file: another unit, its deferral; the slice keeps the roles and tokens its prefixes hold
        units = SELF_TEST_UNITS.replace('slice = ["button"]\n', 'slice = ["button"]\nbadges = ["badge", "window"]\n',
                                        1)
        found, _, _ = run(presets=("adwaita",), units=units, unit="badges")
        if rows_of(found) != [("gnome", "GN-1.6"), ("gnome", "GN-1.7")]:
            failures.append(f"unit badges: got {found}")
        found, _, _ = run(presets=("adwaita",), units=units, unit="slice")
        if rows_of(found) != [("gnome", "GN-1.3"), ("gnome", "GN-1.5"), ("gnome", "GN-1.7"), ("gnome", "GN-1.9†")]:
            failures.append(f"unit slice from the file (a badge rule under context button touches it): got {found}")
        # a rule under a context widget waits for that widget's unit, then both units check it
        units_bars = SELF_TEST_UNITS.replace('slice = ["button"]\n', 'slice = ["button"]\nbars = ["toolbar"]\n', 1)
        for unit, want in ((None, True), ("bars", True), ("slice", True)):
            found, _, _ = run(presets=("adwaita",), units=units_bars, unit=unit)
            if (("gnome", "GN-1.10") in rows_of(found)) != want:
                failures.append(f"context toolbar enabled, unit {unit}: got {found}")
        found, _, _ = run(presets=("adwaita",))
        if ("gnome", "GN-1.10") in rows_of(found):
            failures.append(f"context toolbar not enabled: GN-1.10 checked: {found}")
        units_open = SELF_TEST_UNITS.replace('[deferred.button]\nmenu_placement = "placement"\n'
                                             'scaffold_placement = "placement"\n', "")
        found, _, _ = run(presets=("adwaita",), units=units_open, unit="slice")
        if ("gnome", "GN-1.4") not in rows_of(found) or ("gnome", "GN-1.10") in rows_of(found):
            failures.append(f"no deferral in the file: GN-1.4's menu_placement not checked: {found}")
        # the slice without its prefixes holds the button alone: the token rows are not checked
        found, _, _ = run(presets=("adwaita",), units='slice = ["button"]\n', unit="slice")
        if {("gnome", "GN-1.3"), ("gnome", "GN-1.5"), ("gnome", "GN-1.9†")} & set(rows_of(found)):
            failures.append(f"the slice without prefixes: token rows checked: {found}")
        for bad in ('slice = "button"\n', 'slice = ["button"]\ndeferred = ["x"]\n',
                    'slice = ["button"]\ndeferred = { button = ["menu_placement"] }\n',
                    'slice = ["button"]\nhand_written = ["placement"]\n', "slice = [\n",
                    'slice = ["button"]\nprefixes = ["role"]\n', 'slice = ["button"]\n[prefixes]\nbars = ["role"]\n',
                    'slice = ["button"]\n[staged]\nslice = ["token.behavior.scroll"]\n',
                    'slice = ["button"]\n[prefixes]\nslice = ["token"]\n[staged.slice]\n"token.text.keyed" = "button"\n'
                    '[kept.slice]\n"adwaitta" = ["token.text.keyed.body"]\n',
                    'slice = ["button"]\n[prefixes]\nslice = ["token"]\n[staged.slice]\n"token.text.keyed" = "button"\n'
                    '[kept.slice]\n"adwaita" = ["token.text.keyd.body"]\n', None):
            try:
                if (base / "bad").exists():
                    for f in (base / "bad").rglob("widget_units.toml"):
                        f.unlink()
                write_fixture(base / "bad", units=bad)
                read_units(base / "bad")
                failures.append(f"units file {bad!r}: accepted")
            except InputError:
                pass
        # a `[staged]` or `[deferred]` value names a later, hand-written or listed unit or a catalogue entry; any other
        # is refused, by read_units and waiting_paths alike, naming the key and the value
        catalogue = "## Containers\n\n### scroll_view\n\n- **Parts:**\n  - `body` — the root: `fill`.\n"
        for label, units, files, want in (
                ("a misspelled later unit", '[staged.slice]\n"token.motion.durations" = "motoin"\n'
                                            '[[later]]\nname = "motion"\nkind = "depth"\nenabled = false\n', {},
                 '`staged.slice."token.motion.durations"` = \'motoin\''),
                ("a misspelled hand-written unit", '[deferred.button]\nmenu_placement = "placemnet"\n', {},
                 "`deferred.button.menu_placement` = 'placemnet'"),
                ("a part's field to a misspelled unit", '[deferred.button]\n"body.transitions" = "motoin"\n', {},
                 '`deferred.button."body.transitions"` = \'motoin\''),
                ("no catalogue to name an entry", '[staged.slice]\n"token.behavior.scroll" = "scroll_view"\n', {},
                 "cannot be read"),
                ("an entry the catalogue lacks", '[staged.slice]\n"token.behavior.scroll" = "scroll_veiw"\n',
                 {spec_catalogue.CATALOGUE.as_posix(): catalogue}, "'scroll_veiw' names no later unit"),
                ("a catalogue entry", '[staged.slice]\n"token.behavior.scroll" = "scroll_view"\n',
                 {spec_catalogue.CATALOGUE.as_posix(): catalogue}, None),
                ("a listed unit", '[staged.slice]\n"token.behavior.scroll" = "containers"\n'
                                  '[unit_names]\nm3a = ["containers"]\n', {}, None),
                ("unit names no table of lists", '[unit_names]\nm3a = "containers"\n', {}, "`unit_names` is not")):
            root = base / f"waiting{n[0]}"
            n[0] += 1
            write_fixture(root, units='slice = ["button"]\n[prefixes]\nslice = ["token"]\n'
                                      '[hand_written]\nplacement = false\n' + units, extra=files)
            for reader in (read_units, waiting_paths):
                try:
                    reader(root)
                    if want is not None:
                        failures.append(f"{label}: {reader.__name__} accepted it")
                except InputError as err:
                    if want is None or want not in str(err) or str(UNITS) not in str(err):
                        failures.append(f"{label}: {reader.__name__}: {err}, want {want!r}")
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
        write_fixture(base / "units")
        unit = read_units(base / "units")["slice"]
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
        failures += depth_cases(base)
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def main(argv) -> int:
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    if args == ["--facts"]:
        try:
            lines = facts_report(check_facts.Context(Path(".")))
        except (OSError, UnicodeDecodeError, InputError, check_facts.InputError) as err:
            print(f"error: {err}", file=sys.stderr)
            return 2
        print("\n".join(lines))
        return 0
    assumed, rest = [], []
    while args:
        a = args.pop(0)
        if a == "--preset" and args:
            assumed.append(args.pop(0))
        else:
            rest.append(a)
    args = rest
    if args not in ([], ["--enabled"]) and not (len(args) == 2 and args[0] == "--unit"):
        print(__doc__, file=sys.stderr)
        return 2
    unknown = [p for p in assumed if p not in check_facts.ALL_PRESETS]
    if unknown:
        print(f"error: unknown preset {', '.join(unknown)} (presets: {', '.join(check_facts.ALL_PRESETS)})",
              file=sys.stderr)
        return 2
    try:
        known = read_units(Path("."))
        waiting = [u.name for u in read_later(Path("."))[0] if not u.enabled]
        others, later = [], None
        if not args:
            units, scope = None, "all"
        elif args[0] == "--enabled":
            units, scope = list(known.values()), "enabled-unit"
        elif args[1] in known:
            units, scope = [known[args[1]]], f"`{args[1]}`"
            others = [u for name, u in known.items() if name != args[1]]
        elif args[1] in waiting:
            units, scope, later = list(known.values()), f"`{args[1]}`", args[1]
        else:
            print(f"error: unknown unit `{args[1]}` (units: {', '.join(list(known) + waiting)})", file=sys.stderr)
            return 2
        ctx = check_facts.Context(Path("."))
        findings, tallies, presets = check(ctx, units, others, assumed, later)
    except (OSError, UnicodeDecodeError, InputError, check_facts.InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for platform, row, target, what in findings:
        print(f"{platform}\t{row}\t{target}\t{what}")
    if units is not None:
        print(f"# units: {', '.join(u.name for u in units)}")
    for line in report(tallies, presets, scope, assumed):
        print(f"# {line}")
    print(summary(findings, tallies, presets, scope), file=sys.stderr)
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
