#!/usr/bin/env python3
"""Compare the three showcases element by element: sizes, positions, colours.

The gpui, iced and egui showcases draw one shared set of elements, listed once
in docs/showcase-elements.toml. Under the same theme the three must lay each
element out alike and paint it in the same colours, because all three follow
the theme data. This script is the mechanical gate for that: it reads what the
showcases report about their own layout, and what their captures show, and
fails on every difference not recorded, with its reason, in the `[parity]`
section of docs/showcase-exceptions.toml. The only tolerances are the three
rules below, each documented with its sources, and the run counts what they
cover.

Run from the repository root:

    python3 scripts/check_showcase_parity.py [RUN=]DUMP_DIR... [--content-offset X,Y]
    python3 scripts/check_showcase_parity.py --check-list
    python3 scripts/check_showcase_parity.py --self-test
    python3 scripts/check_showcase_parity.py --merge OUT [--write] --proposal FILE... [RUN=]DUMP_DIR...

Requires: Python 3.11+ (tomllib) and Pillow.

Inputs
------
A DUMP_DIR holds, for each showcase kind, preset and variant it was run
under, directly or in a subdirectory one level down (`<dir>/adwaita-light/`):

    <kind>-<preset>-<variant>.json   the layout dump (required)
    <kind>-<preset>-<variant>.png    the capture (optional)
    <kind>-<preset>-<variant>.offset where the content sits in a framed
                                     capture (optional, see below)

`kind` is `gpui`, `iced` or `egui`; `variant` is `light` or `dark`; the
preset is the rest of the name (`kde-breeze`, `adwaita`, ...).

Each DUMP_DIR is one run of the showcases. A run is `rest` (the window as
it opens, no pointer over it) unless the directory is given as
`hover=DIR` (the pointer held over an element) or `menu=DIR` (a menu open,
the showcases' `--open-menu`); an exception can be scoped to a run (see
Exceptions). Several directories are compared in one call, each on its
own; a kind, preset and variant given twice in one run is an error.

The layout dump, written by a showcase run with `--dump-layout <file>` after
its first settled frame, is one JSON object:

    {
      "kind": "gpui" | "iced" | "egui",
      "preset": "kde-breeze",
      "variant": "light" | "dark",
      "scale": 1.0,
      "elements": {
        "<element id>": {"x": 0.0, "y": 0.0, "w": 1280.0, "h": 720.0},
        ...
      }
    }

Every rectangle is in logical pixels, in window-content coordinates: (0, 0)
is the top-left corner of the window's content area, below and right of any
frame the window manager draws. `scale` is the number of physical pixels per
logical pixel the capture was taken at. A dump holds an entry for every
element of the list the showcase draws, and only those.

A capture is either the content alone (exactly 1280 x 720 logical pixels
times `scale`, the showcase window's size, scripts/capture_window.sh:32-33)
or the window with its frame. For a framed capture the content's offset, in
physical pixels, comes from the `.offset` file beside it -- the
`+<dx>+<dy>` that scripts/capture_window.sh's identity check prints, or
`<dx>,<dy>` -- else from `--content-offset X,Y`; with neither the script
stops.

What is compared
----------------
For each preset and variant, across the three kinds:

- presence: every element of the list must be in every dump (`present`);
  an id no list element has is an error. An element the list marks with
  `when` (on screen only under a condition: an open menu, a hover) is
  compared only where some dump holds it; one no dump holds is listed at the
  end of the report, not counted as a difference. So is an element without
  `when` that no dump holds: the three agree it is not there, which is no
  difference between them, and the listing shows it for the list to be
  fixed;
- size: `w` and `h`;
- position: `x` and `y` relative to the element's parent, the element the
  list names in `parent`; for `parent = "window"` the content origin;
- colour, where the captures are there: every `sample` point the list gives
  the element, sampled in each capture (see docs/showcase-elements.toml for
  the point syntax). A colour is `#rrggbb`. A sample is taken only where it
  is visible: its point, or all of its glyph box, inside the window's
  content and inside every ancestor the list marks `clip = true` (an area
  that shows its descendants only within its own rectangle, as a scrolled
  page's panel does), each rectangle from the same dump. Elsewhere its value
  is `not visible`: the sample is left out of the comparison for that
  showcase, not a difference, and the run counts the element samples it left
  out so under "not visible". Geometry is compared wherever the dumps hold
  the element, visible or not, so every showcase dumps off-screen elements
  too.

Numbers are compared under the rules below, colours too; what no rule covers
is compared exactly (numbers at 1/100 of a logical pixel: the dumps are f32
layout results, and a difference below that is float arithmetic, not layout).

Rules
-----
Three toolkits draw the same theme through three renderers, and three kinds
of difference come from the renderers, not from the theme data. Each rule
below covers one of them for every element, so no element needs an exception
for it; a difference a rule covers is not reported (the summary line counts
them per rule). Everything else is an exception with its reason, or a
failure.

R-snap -- geometry is compared on edges rounded to the pixel grid.
    What each toolkit paints lands on whole device pixels, but the dumps
    report the layout before that rounding in iced and egui:
    - gpui snaps every element's outer box after layout, rounding its
      absolute edges in device pixels, ties toward zero (gpui-pre 0.3.6
      src/taffy.rs:270-345, the design; :371-374, the rounding;
      src/util.rs:128-132, `round_half_toward_zero`), and rounds a measured
      text size up to whole device pixels (src/taffy.rs:412-414); its glyphs
      sit on whole device pixels vertically (`SUBPIXEL_VARIANTS_Y` = 1,
      src/text_system.rs:52, applied in src/window.rs:4661-4666). Its dump
      therefore holds whole pixels.
    - egui lays out in steps of 1/32 point (emath 0.36.2
      src/gui_rounding.rs:18) and rounds rectangles, right-angled line
      segments and text positions to physical pixels when it paints; all
      three are on by default (epaint 0.36.2 src/tessellator.rs:685-700,
      :734-736; rectangles :1830-1861, text :2024-2025; the rounding,
      `(x * pixels_per_point).round()`, emath src/gui_rounding.rs:68-70).
      Its dump holds the unrounded layout.
    - iced keeps fractions: iced_core 0.14.0's layout (src/layout.rs,
      src/layout/) rounds nothing, and its dump reports them. Its wgpu
      renderer rounds a quad's edges only as it paints, when `Quad::snap` is
      set (iced_core src/renderer.rs:89-99, `snap: cfg!(feature = "crisp")`,
      `crisp` in iced 0.14.0's default features, Cargo.toml:64-67;
      iced_wgpu 0.14.0 src/shader/quad/solid.wgsl:38-40), and cosmic-text
      truncates a glyph's y to a whole pixel (cosmic-text 0.15.0
      src/layout.rs:84); iced_tiny_skia draws a quad at its fractional
      bounds (engine.rs:65).
    So the comparator rounds each edge of each rectangle -- left, top,
    right = x + w, bottom = y + h, in window coordinates, as gpui does -- to
    the device pixel grid of the dump's `scale` (whole logical pixels at
    scale 1.0, the captures' scale), ties up (as egui's `f32::round` and
    iced's shader round a positive coordinate; gpui's edges are already
    whole), and compares w and h as rounded right - left and rounded
    bottom - top, x and y as the rounded left and top edges minus the
    parent's. A 0.4 px difference in an edge passes unless the two edges
    fall on the two sides of a pixel's middle, where they paint a pixel
    apart. Widths and offsets are never rounded themselves: two boxes whose
    edges land on the same pixels agree even where their fractional widths
    round apart (19.6 + 200.5 against 20.4 + 199.4), and a child agrees
    with itself wherever its parent's fractions put the parent's edge. In
    the report, a geometry value R-snap moved shows the rounded value with
    the dump's in parentheses, `223 (222.25)`: the rounded one is what was
    compared.
    Since both edges round, a fractional size or offset paints as one whole
    number or the next depending on where the box sits: a 0.6 px offset
    from a parent edge at 20.0 puts the child's edge at 20.6, a pixel in
    (21); from a parent edge at 19.7 it puts it at 20.3, on the parent's
    pixel (20). That is what the renderer paints, not an error of the rule,
    and the same fractional layout can therefore show as 0 under one
    preset and 1 under another.
    Every sample is taken on the same rounded edges: a point's fractions and
    offsets, and a glyph box's fractions, are of the rectangle between them,
    so a glyph box of a row at y = 39.6, h = 16 covers the painted rows 40
    to 55, not the neighbouring row 39 a floor of 39.6 would add.

R-shape -- a text run's width may differ by up to max(1 px, 2 %).
    Each toolkit shapes text with its own shaper -- gpui with cosmic-text
    0.19 on Linux, iced with cosmic-text 0.15, egui with epaint's own text
    layout -- and each hints and kerns on its own; the theme states the
    font (family, size, weight), not the shaper, so the same string comes
    out a little wider or narrower in each. The width only: a text run's
    left, top and height are the layout's and stay exact (after R-snap).
    The tolerance is taken on the R-snap widths, of the widest of them. A
    text run is an element whose rectangle is its text box (the list's
    header: "a label's is its text box"), named in TEXT_RUNS below by the
    list's `name`; `--check-list` fails on a name there no element has.

R-glyph -- colour samples within a per-channel tolerance.
    A glyph sample (`"glyph"`, `{ glyph = [...] }`: text, a check mark, an
    arrow, a spinner's arc) passes when the three showcases' most
    contrasting pixels are within 8 per channel of each other: glyph
    anti-aliasing and the gamma each rasteriser blends coverage in (gpui's
    atlas with its own glyph dilation, `glyph_dilation_for_color`, gpui-pre
    0.3.6 src/window.rs:4673; cosmic-text/swash in iced; epaint's font
    atlas in egui) move the darkest pixel of a stem by a few levels. A
    point sample (a fill, a border, a line) passes within 1 per channel:
    a translucent stated colour blended over its ground rounds to 8 bits
    in each renderer, and may land one level apart. The comparator holds
    no theme values, so both tolerances hold between the showcases'
    samples; a sample that is not a colour (`outside`, `no glyph`) is
    compared exactly.

Exceptions
----------
docs/showcase-exceptions.toml's `[parity]` section lists what may differ, in
groups: each `[[parity.group]]` states a reason once -- a toolkit limit with
its upstream citation, or a detail the theme leaves unstated -- and the keys
of the element properties it excepts.

    [parity]

    [[parity.group]]
    reason = "..."
    keys = [
        "chrome.splitter.w",
        "material:basic.expander.details.header.y",
    ]

    [[parity.group]]
    reason = "... | ..."
    keys = [
        "adwaita/dark:basic.checkboxes.checked.label.text",
        "menu@kde-breeze:chrome.menu_bar.theme.h",
    ]

A reason may join several with ` | ` (a key several proposals gave different
reasons, see Merging proposals), so no single reason contains ` | `. The
section holds groups only: the flat `key = reason` form, one reason repeated
per key, is refused there (it is read in proposal files alone), so the file
has one form, the one --merge --write writes. A group with no keys, a key
listed twice (in one group or two) and a group holding anything but `reason`
and `keys` are errors.

The key is `[<run>@][<preset>[/<variant>]:]<element id>.<property>`:

- `<element id>.<property>`, the property `present`, `x`, `y`, `w`, `h` or
  the name of one of the element's samples, is the difference excepted;
- `<preset>:` or `<preset>/<variant>:` scopes the exception to that preset,
  or to that preset in that variant; without it the exception holds under
  every preset and variant. A theme's silence (a size or a colour the
  preset leaves unstated, each toolkit keeping its own default) is per
  preset, and so is its exception; an unscoped key is for a toolkit limit
  that holds under every theme;
- `<run>@` (`rest`, `hover` or `menu`) scopes it to that run (see Inputs);
  without it the exception holds in every run.

The preset is a file name of native-theme/src/presets/ without `.toml`. Two
keys for the same element property whose scopes overlap (`x.w` and
`material:x.w`; `hover@x.w` and `material:x.w`) are an error, so at most one
exception holds for a difference; so is a key naming no element, property,
preset, variant or run. An exception whose scope the run compared but that
matched no difference is listed at the end, so a stale one is seen.

Merging proposals
-----------------
`--merge OUT --proposal FILE...` writes the minimal `[parity]` groups the
dumps need to OUT, from proposed exceptions: each FILE holds keys of the
form above as `key = reason` pairs (at its top level or in a `[parity]`
table) or in `[[parity.group]]` entries, and the `--exceptions` file's
groups are read as one more proposal. The dumps are compared with no
exception; then, for each element property that differs somewhere:

- a proposed key no difference in its scope needs is dropped (listed), and
  so is one naming no element or property of the list;
- the differences the proposals' scopes cover are excepted by the fewest
  keys that hold exactly where the property differs among the runs, presets
  and variants compared (among those where the element is compared at all:
  a menu row differing in every menu run gets an unscoped key): unscoped
  when it differs everywhere, else `<preset>:` where it differs in both
  variants, `<preset>/<variant>:` where in one, each with a `<run>@` where
  it differs in some runs and not in others;
- each key's reasons are the proposals' reasons for it, a proposal's
  ` | `-joined reason read as the reasons it joins, each once, sorted;
- keys with the same reasons form one group, whose reason joins them with
  ` | `. The groups follow the element list's order of their first key,
  and a group's keys the list's order of elements and properties, then
  the key's text.

So feed the merge every run and every preset the gate will compare, or its
keys are scoped to the ones it saw. A difference no proposal covers is
listed and the exit status is 1; OUT is written either way, never the
exceptions file itself.

`--write` (with --merge) also writes the groups into the `--exceptions` file:
it replaces everything from the file's `[parity]` line to its end, which must
be the file's last section, with the same groups under a comment naming the
command, and leaves every byte above that line as it is. The file's groups
are proposals to the merge, and a merge's reasons are sorted, so a second run
over the same inputs writes the same bytes.

Exit status: 0 when every difference is excepted, 1 when one is not (or a dump
or capture is missing), 2 when an input cannot be read or is malformed.
"""

import argparse
import json
import math
import os
import re
import sys
import tempfile
import tomllib

try:
    from PIL import Image, ImageDraw
except ImportError:  # pragma: no cover - reported, not raised
    Image = None
    ImageDraw = None

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)

ELEMENTS = os.path.join(PROJECT_ROOT, "docs", "showcase-elements.toml")
EXCEPTIONS = os.path.join(PROJECT_ROOT, "docs", "showcase-exceptions.toml")
REGISTRY = os.path.join(PROJECT_ROOT, "docs", "property-registry.toml")
PRESETS = os.path.join(PROJECT_ROOT, "native-theme", "src", "presets")

KINDS = ("gpui", "iced", "egui")
VARIANTS = ("light", "dark")
# The runs a dump directory can be (see the module's docstring); the first is
# a bare DUMP_DIR's.
RUNS = ("rest", "hover", "menu")
# The parent of a top-level element: the window's content area.
WINDOW = "window"
# The showcase window's content size in logical pixels
# (scripts/capture_window.sh:32-33).
WINDOW_SIZE = (1280, 720)
# Element properties that are not samples.
GEOMETRY = ("present", "x", "y", "w", "h")
# Decimal places numbers are compared at (1/100 logical pixel).
PLACES = 2

# The rules (see the module's docstring), each on by default.
RULES = ("snap", "shape", "glyph")
# R-shape: a text run's width may differ by up to max(SHAPE_PX, SHAPE_SHARE x width).
SHAPE_PX = 1.0
SHAPE_SHARE = 0.02
# R-glyph: per-channel tolerance of a glyph sample and of a point sample.
GLYPH_CHANNEL = 8
POINT_CHANNEL = 1
# R-shape: the list's `name`s of the elements whose rectangle is a text box.
TEXT_RUNS = frozenset(
    {
        "Button label",
        "Checkbox label",
        "Drop-down text",
        "Expander body",
        "Expander title",
        "Input text",
        "Label",
        "Link",
        "Menu shortcut",
        "Radio label",
        "Section heading",
        "Side panel label",
        "Status text",
        "Switch label",
        "Text",
        "Text area text",
        "Tooltip text",
        "Widget Info hint",
        "Widget Info leaf",
        "Widget Info route",
        "Widget Info section",
        "Widget Info title",
    }
)

ID = re.compile(r"^[a-z][a-z0-9_]*(\.[a-z0-9_]+)*$")
SAMPLE = re.compile(r"^[a-z][a-z0-9_]*$")
DUMP_NAME = re.compile(r"^(gpui|iced|egui)-(.+)-(light|dark)\.json$")
OFFSET = re.compile(r"^\s*\+?(\d+)\s*[+,]\s*(\d+)\s*$")
# An exception key: [<run>@][<preset>[/<variant>]:]<element id>.<property>.
KEY = re.compile(r"^(?:([a-z]+)@)?(?:([a-z0-9-]+)(?:/([a-z]+))?:)?([^@:/]+)$")
RUN_DIR = re.compile(r"^(" + "|".join(RUNS) + r")=(.+)$")
# The line the exceptions file's generated section starts at (--merge --write).
PARITY_HEADER = re.compile(r"^\[parity\][ \t]*(?:#[^\n]*)?$", re.MULTILINE)
# What joins the reasons of one key several proposals gave.
REASON_SEPARATOR = " | "


class Failure(Exception):
    """An input the script cannot use; exit status 2."""


# ---------------------------------------------------------------------------
# The registry, the element list and the exceptions
# ---------------------------------------------------------------------------


def load_toml(path):
    try:
        with open(path, "rb") as f:
            return tomllib.load(f)
    except OSError as err:
        raise Failure(f"could not read {path}: {err}") from err
    except tomllib.TOMLDecodeError as err:
        raise Failure(f"{path} is not valid TOML: {err}") from err


def registry_paths(registry):
    """Every leaf path docs/property-registry.toml names.

    A field of a structure type (`font`, `border`, `icon_sizes`,
    `text_scale_entry`) is a path itself (`button.font`) and so is each of its
    sub-fields (`button.font.color`, `button.border.corner_radius_px`).
    """
    structures = {}
    for name, fields in registry.get("_structures", {}).items():
        structures[name] = list(fields)
    by_type = {
        "font": structures.get("Font", []),
        "border": structures.get("Border", []),
        "icon_sizes": structures.get("IconSizes", []),
        "text_scale_entry": structures.get("TextScaleEntry", []),
    }
    paths = set()
    for widget, fields in registry.items():
        if widget.startswith("_") or not isinstance(fields, dict):
            continue
        for field, kind in fields.items():
            path = f"{widget}.{field}"
            paths.add(path)
            for sub in by_type.get(kind, []):
                paths.add(f"{path}.{sub}")
    return paths


def parse_sample(where, name, spec):
    """A sample point: `[fx, fy]`, `[fx, fy, dx, dy]`, `"glyph"` or
    `{ glyph = [x0, y0, x1, y1] }`. Returns ("point", fx, fy, dx, dy) or
    ("glyph", x0, y0, x1, y1), fractions of the element's rectangle and
    offsets in logical pixels."""
    if spec == "glyph":
        return ("glyph", 0.0, 0.0, 1.0, 1.0)
    if isinstance(spec, dict) and set(spec) == {"glyph"}:
        box = spec["glyph"]
        if (
            isinstance(box, list)
            and len(box) == 4
            and all(isinstance(v, (int, float)) for v in box)
            and 0 <= box[0] < box[2] <= 1
            and 0 <= box[1] < box[3] <= 1
        ):
            return ("glyph", *(float(v) for v in box))
        raise Failure(f"{where}: sample `{name}`: glyph box must be [x0, y0, x1, y1] fractions, x0 < x1, y0 < y1")
    if (
        isinstance(spec, list)
        and len(spec) in (2, 4)
        and all(isinstance(v, (int, float)) and not isinstance(v, bool) for v in spec)
    ):
        fx, fy = float(spec[0]), float(spec[1])
        dx, dy = (float(spec[2]), float(spec[3])) if len(spec) == 4 else (0.0, 0.0)
        if not (0 <= fx <= 1 and 0 <= fy <= 1):
            raise Failure(f"{where}: sample `{name}`: fx and fy are fractions of the rectangle, 0 to 1")
        return ("point", fx, fy, dx, dy)
    raise Failure(
        f"{where}: sample `{name}` must be [fx, fy], [fx, fy, dx, dy], \"glyph\" or {{ glyph = [x0, y0, x1, y1] }}"
    )


def load_elements(path, leaves_known):
    """The element list, in file order: id -> element dict."""
    data = load_toml(path)
    rows = data.get("element")
    if not isinstance(rows, list) or not rows:
        raise Failure(f"{path}: no [[element]] entries")
    elements = {}
    for n, row in enumerate(rows, 1):
        where = f"{path}: element #{n}"
        if not isinstance(row, dict):
            raise Failure(f"{where}: not a table")
        unknown = set(row) - {"id", "name", "parent", "leaves", "states", "sample", "part", "when", "clip"}
        if unknown:
            raise Failure(f"{where}: unknown keys {sorted(unknown)}")
        ident = row.get("id")
        if not isinstance(ident, str) or not ID.match(ident) or ident == WINDOW:
            raise Failure(f"{where}: `id` must be a dotted lower-case name, not {ident!r}")
        where = f"{path}: {ident}"
        if ident in elements:
            raise Failure(f"{where}: the id is listed twice")
        for key in ("name", "parent"):
            if not isinstance(row.get(key), str) or not row[key]:
                raise Failure(f"{where}: `{key}` must be a non-empty string")
        for key in ("leaves", "states"):
            value = row.get(key)
            if not isinstance(value, list) or not all(isinstance(v, str) and v for v in value):
                raise Failure(f"{where}: `{key}` must be a list of non-empty strings")
        if len(set(row["leaves"])) != len(row["leaves"]):
            raise Failure(f"{where}: a leaf is listed twice")
        if not row["states"]:
            raise Failure(f"{where}: `states` names at least the state shown")
        if not isinstance(row.get("part", False), bool):
            raise Failure(f"{where}: `part` must be true or false")
        if not isinstance(row.get("clip", False), bool):
            raise Failure(f"{where}: `clip` must be true or false")
        if "when" in row and (not isinstance(row["when"], str) or not row["when"]):
            raise Failure(f"{where}: `when` must be a non-empty string")
        for leaf in row["leaves"]:
            if leaf not in leaves_known:
                raise Failure(f"{where}: `{leaf}` is not a docs/property-registry.toml path")
        samples = {}
        for name, spec in row.get("sample", {}).items():
            if not SAMPLE.match(name) or name in GEOMETRY:
                raise Failure(f"{where}: sample name `{name}` is reserved or malformed")
            samples[name] = parse_sample(where, name, spec)
        elements[ident] = {
            "id": ident,
            "name": row["name"],
            "parent": row["parent"],
            "leaves": row["leaves"],
            "states": row["states"],
            "samples": samples,
            "when": row.get("when"),
            "clip": row.get("clip", False),
        }
    for ident, element in elements.items():
        parent = element["parent"]
        if parent != WINDOW and parent not in elements:
            raise Failure(f"{path}: {ident}: parent `{parent}` is not an element of the list")
        seen = {ident}
        while parent != WINDOW:
            if parent in seen:
                raise Failure(f"{path}: {ident}: its parents form a cycle")
            seen.add(parent)
            parent = elements[parent]["parent"]
    return elements


def preset_names(directory=PRESETS):
    """The presets native-theme ships: its presets directory's TOML files."""
    try:
        names = os.listdir(directory)
    except OSError as err:
        raise Failure(f"could not read {directory}: {err}") from err
    return {name[: -len(".toml")] for name in names if name.endswith(".toml")}


class BadKey(Exception):
    """An exception key that names no element, property, preset, variant or run."""


def parse_key(key, elements, presets):
    """An exception key as (scope, target): scope = (run, preset, variant),
    each None where the key leaves it open, target = "<id>.<property>"."""
    match = KEY.match(key)
    if not match:
        raise BadKey("expected [<run>@][<preset>[/<variant>]:]<element id>.<property>")
    run, preset, variant, target = match.groups()
    if run is not None and run not in RUNS:
        raise BadKey(f"`{run}` is not a run ({', '.join(RUNS)})")
    if preset is not None and preset not in presets:
        raise BadKey(f"`{preset}` is not a preset of native-theme/src/presets")
    if variant is not None and variant not in VARIANTS:
        raise BadKey(f"`{variant}` is not a variant ({', '.join(VARIANTS)})")
    ident, _, prop = target.rpartition(".")
    element = elements.get(ident)
    if element is None:
        raise BadKey(f"`{ident}` is not an element of the list")
    if prop not in GEOMETRY and prop not in element["samples"]:
        raise BadKey(f"`{prop}` is neither one of {', '.join(GEOMETRY)} nor a sample of {ident}")
    return (run, preset, variant), target


def key_text(scope, target):
    run, preset, variant = scope
    prefix = f"{run}@" if run is not None else ""
    if preset is not None:
        prefix += f"{preset}/{variant}:" if variant is not None else f"{preset}:"
    return prefix + target


def holds(scope, group):
    """Whether an exception of `scope` holds in `group` = (run, preset, variant)."""
    return all(s is None or s == g for s, g in zip(scope, group))


def overlaps(a, b):
    """Whether two scopes hold together somewhere."""
    return all(x is None or y is None or x == y for x, y in zip(a, b))


def parity_entries(path, data, flat):
    """The (key, reason, where) entries of a file's `[parity]` table: those of
    its `[[parity.group]]` entries, each key with its group's reason, and,
    with `flat`, its `key = reason` pairs too."""
    table = data.get("parity", {})
    if not isinstance(table, dict):
        raise Failure(f"{path}: [parity] must be a table")
    groups = table.get("group", [])
    if not isinstance(groups, list):
        raise Failure(f"{path}: [parity] group must be [[parity.group]] entries")
    entries = []
    for n, group in enumerate(groups, 1):
        where = f"[[parity.group]] #{n}"
        if not isinstance(group, dict) or set(group) != {"reason", "keys"}:
            raise Failure(f"{path}: {where}: a group holds `reason` and `keys`, nothing else")
        reason, keys = group["reason"], group["keys"]
        if not isinstance(reason, str) or not reason.strip():
            raise Failure(f"{path}: {where}: a group needs a non-empty reason")
        if not isinstance(keys, list) or not all(isinstance(k, str) for k in keys):
            raise Failure(f"{path}: {where}: `keys` must be a list of exception keys")
        if not keys:
            raise Failure(f"{path}: {where}: the group has no keys")
        entries += [(key, reason, where) for key in keys]
    for key, reason in table.items():
        if key == "group":
            continue
        if not flat:
            raise Failure(
                f"{path}: [parity] {key}: exceptions are listed in [[parity.group]] entries "
                "(reason, keys), not as key = reason"
            )
        if not isinstance(reason, str) or not reason.strip():
            raise Failure(f"{path}: [parity] {key}: an exception needs a non-empty reason")
        entries.append((key, reason, "[parity]"))
    return entries


def load_parity_exceptions(path, elements, presets):
    """The `[[parity.group]]` entries: key -> {"scope", "target", "reason",
    "group"}, `group` the entry's `[[parity.group]] #n`."""
    exceptions = {}
    for key, reason, where in parity_entries(path, load_toml(path), flat=False):
        if key in exceptions:
            raise Failure(f"{path}: {where}: `{key}` is listed twice (also in {exceptions[key]['group']})")
        try:
            scope, target = parse_key(key, elements, presets)
        except BadKey as err:
            raise Failure(f"{path}: {where}: {key}: {err}") from err
        for other, entry in exceptions.items():
            if entry["target"] == target and overlaps(entry["scope"], scope):
                raise Failure(f"{path}: {where}: {key}: its scope overlaps `{other}`'s ({entry['group']})")
        exceptions[key] = {"scope": scope, "target": target, "reason": reason, "group": where}
    return exceptions


def find_exception(exceptions, target, group):
    """The key of the exception that holds for `target` in `group`, or None."""
    for key, entry in exceptions.items():
        if entry["target"] == target and holds(entry["scope"], group):
            return key
    return None


# ---------------------------------------------------------------------------
# Dumps and captures
# ---------------------------------------------------------------------------


def is_number(value):
    return isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value)


def load_dump(path, kind, preset, variant):
    try:
        with open(path, encoding="utf-8") as f:
            data = json.load(f)
    except OSError as err:
        raise Failure(f"could not read {path}: {err}") from err
    except json.JSONDecodeError as err:
        raise Failure(f"{path} is not valid JSON: {err}") from err
    if not isinstance(data, dict):
        raise Failure(f"{path}: not a JSON object")
    for key, expected in (("kind", kind), ("preset", preset), ("variant", variant)):
        if data.get(key) != expected:
            raise Failure(f"{path}: `{key}` is {data.get(key)!r}, the file name says {expected!r}")
    scale = data.get("scale")
    if not is_number(scale) or scale <= 0:
        raise Failure(f"{path}: `scale` must be a positive number")
    rects = data.get("elements")
    if not isinstance(rects, dict):
        raise Failure(f"{path}: `elements` must be an object")
    for ident, rect in rects.items():
        if (
            not isinstance(rect, dict)
            or set(rect) != {"x", "y", "w", "h"}
            or not all(is_number(rect[k]) for k in rect)
            or rect["w"] < 0
            or rect["h"] < 0
        ):
            raise Failure(f"{path}: {ident}: a rectangle is {{x, y, w, h}}, numbers, w and h not negative")
    return {"scale": float(scale), "elements": rects}


def read_offset(path):
    try:
        with open(path, encoding="utf-8") as f:
            text = f.read()
    except OSError as err:
        raise Failure(f"could not read {path}: {err}") from err
    match = OFFSET.match(text)
    if not match:
        raise Failure(f"{path}: expected `+<dx>+<dy>` or `<dx>,<dy>`, found {text.strip()!r}")
    return int(match.group(1)), int(match.group(2))


def load_capture(png, scale, offset_file, cli_offset):
    """The capture's pixels and the content's offset in it."""
    if Image is None:
        raise Failure("Pillow is required to read captures (pip install Pillow)")
    try:
        image = Image.open(png).convert("RGB")
    except OSError as err:
        raise Failure(f"could not read {png}: {err}") from err
    content = (round(WINDOW_SIZE[0] * scale), round(WINDOW_SIZE[1] * scale))
    if os.path.isfile(offset_file):
        offset = read_offset(offset_file)
    elif image.size == content:
        offset = (0, 0)
    elif cli_offset is not None:
        offset = cli_offset
    else:
        raise Failure(
            f"{png} is {image.size[0]}x{image.size[1]}, not the {content[0]}x{content[1]} content, "
            f"and neither {os.path.basename(offset_file)} nor --content-offset says where the content is"
        )
    if offset[0] + content[0] > image.size[0] or offset[1] + content[1] > image.size[1]:
        raise Failure(f"{png}: the content at +{offset[0]}+{offset[1]} does not fit in the capture")
    return {"image": image, "pixels": image.load(), "offset": offset}


def parse_run_dir(text):
    """A DUMP_DIR argument as (run, directory)."""
    match = RUN_DIR.match(text)
    if match and not os.path.isdir(text):
        return match.group(1), match.group(2)
    return RUNS[0], text


def load_run(directory, kinds, cli_offset, run=RUNS[0], groups=None):
    """(run, preset, variant) -> kind -> {"scale", "elements", "capture"},
    from the dumps in `directory` and in its subdirectories, added to
    `groups` when given."""
    if not os.path.isdir(directory):
        raise Failure(f"not a directory: {directory}")
    groups = {} if groups is None else groups
    found = 0
    folders = [directory] + sorted(
        path for path in (os.path.join(directory, n) for n in os.listdir(directory)) if os.path.isdir(path)
    )
    for folder in folders:
        for name in sorted(os.listdir(folder)):
            match = DUMP_NAME.match(name)
            if not match or match.group(1) not in kinds:
                continue
            kind, preset, variant = match.groups()
            stem = os.path.join(folder, name[: -len(".json")])
            group = groups.setdefault((run, preset, variant), {})
            if kind in group:
                raise Failure(f"{stem}.json: a second {kind} dump of {run}@{preset}/{variant}")
            dump = load_dump(stem + ".json", kind, preset, variant)
            png = stem + ".png"
            dump["capture"] = (
                load_capture(png, dump["scale"], stem + ".offset", cli_offset)
                if os.path.isfile(png)
                else None
            )
            group[kind] = dump
            found += 1
    if not found:
        raise Failure(f"{directory}: no <kind>-<preset>-<variant>.json dumps")
    return groups


# ---------------------------------------------------------------------------
# Measuring
# ---------------------------------------------------------------------------


def hex_colour(rgb):
    return "#{:02x}{:02x}{:02x}".format(*rgb[:3])


NOT_VISIBLE = "not visible"


def visible_area(ident, elements, rects, scale):
    """The window-content rectangle, as (left, top, right, bottom) R-snap
    edges in logical pixels, in which the element `ident` shows: the window's
    content, cut to every ancestor the list marks `clip` that the dump holds."""
    left, top, right, bottom = 0.0, 0.0, float(WINDOW_SIZE[0]), float(WINDOW_SIZE[1])
    parent = elements[ident]["parent"]
    while parent != WINDOW:
        if elements[parent]["clip"] and parent in rects:
            l, t, r, b = snapped_edges(rects[parent], scale)
            left, top, right, bottom = max(left, l), max(top, t), min(right, r), min(bottom, b)
        parent = elements[parent]["parent"]
    return left, top, right, bottom


def sample_colour(capture, scale, rect, spec, area=None):
    """The colour a sample point shows, or a note saying why there is none:
    `not visible` where the point, or a glyph box, is not all inside `area`
    (logical left, top, right, bottom; `visible_area`). Fractions are taken
    of the rectangle's R-snap edges, where it is painted, not of the dump's
    fractional ones."""
    image, pixels, (ox, oy) = capture["image"], capture["pixels"], capture["offset"]
    content_w = round(WINDOW_SIZE[0] * scale)
    content_h = round(WINDOW_SIZE[1] * scale)
    left, top, right, bottom = snapped_edges(rect, scale)
    w, h = right - left, bottom - top
    if area is None:
        area = (0.0, 0.0, float(WINDOW_SIZE[0]), float(WINDOW_SIZE[1]))
    a_left, a_top, a_right, a_bottom = (round(v * scale) for v in area)
    if spec[0] == "point":
        _, fx, fy, dx, dy = spec
        px = math.floor((left + fx * w + dx) * scale)
        py = math.floor((top + fy * h + dy) * scale)
        if not (a_left <= px < a_right and a_top <= py < a_bottom):
            return NOT_VISIBLE
        if not (0 <= px < content_w and 0 <= py < content_h):
            return "outside"
        return hex_colour(pixels[ox + px, oy + py])
    # A glyph sample: the pixel of the box that differs most from the ground
    # the text is drawn on; the first such pixel in raster order. The ground is
    # the most frequent colour of the box's outermost ring of pixels, where a
    # line box or icon box shows what lies behind its glyph: in a tight box a
    # filled glyph (an expander's 12x12 triangle) can cover as many pixels as
    # its ground, so the whole box's most frequent colour could be the glyph.
    # Where the ring ties, the whole box's count decides.
    _, x0, y0, x1, y1 = spec
    box = (
        math.floor((left + x0 * w) * scale),
        math.floor((top + y0 * h) * scale),
        math.ceil((left + x1 * w) * scale),
        math.ceil((top + y1 * h) * scale),
    )
    if box[0] < a_left or box[1] < a_top or box[2] > a_right or box[3] > a_bottom:
        return NOT_VISIBLE
    left, top, right, bottom = (
        max(0, box[0]),
        max(0, box[1]),
        min(content_w, box[2]),
        min(content_h, box[3]),
    )
    if right <= left or bottom <= top:
        return "outside"
    box = image.crop((ox + left, oy + top, ox + right, oy + bottom))
    counts = box.getcolors(maxcolors=box.size[0] * box.size[1])
    if not counts:
        return "outside"
    data = box.load()
    bw, bh = box.size
    ring = {}
    for y in range(bh):
        for x in range(bw):
            if x in (0, bw - 1) or y in (0, bh - 1):
                ring[data[x, y]] = ring.get(data[x, y], 0) + 1
    whole = {colour: count for count, colour in counts}
    ground = max(ring, key=lambda c: (ring[c], whole.get(c, 0), c))
    best, best_distance = None, 0
    for y in range(box.size[1]):
        for x in range(box.size[0]):
            p = data[x, y]
            distance = sum((a - b) ** 2 for a, b in zip(p, ground))
            if distance > best_distance:
                best, best_distance = p, distance
    return hex_colour(best) if best is not None else "no glyph"


def number(value):
    return f"{round(value, PLACES):.{PLACES}f}".rstrip("0").rstrip(".")


def snap(value, scale):
    """R-snap: a coordinate on the device pixel grid of `scale`, ties up."""
    return math.floor(value * scale + 0.5) / scale


def snapped_edges(rect, scale):
    """R-snap: the rectangle's left, top, right and bottom edges, each rounded
    to the pixel grid on its own. So a size is not rounded but follows from
    where both its edges land: a 0.6 px wide box from x = 10.2 paints no
    column (10.2 and 10.8 both round to 10), from x = 10.3 one (10.3 rounds
    to 10, 10.9 to 11), as each toolkit that snaps an edge paints it."""
    left, top = rect["x"], rect["y"]
    right, bottom = left + rect["w"], top + rect["h"]
    return tuple(snap(v, scale) for v in (left, top, right, bottom))


def geometry(element, rects, scale, snapped):
    """The element's w and h and, where its parent is drawn, its x and y
    relative to the parent; with `snapped`, from its edges rounded to the
    pixel grid (R-snap)."""

    def edges(rect):
        if snapped:
            return snapped_edges(rect, scale)
        left, top = rect["x"], rect["y"]
        return left, top, left + rect["w"], top + rect["h"]

    left, top, right, bottom = edges(rects[element["id"]])
    values = {"w": right - left, "h": bottom - top}
    parent = element["parent"]
    if parent == WINDOW:
        values["x"], values["y"] = left, top
    elif parent in rects:
        parent_left, parent_top, _, _ = edges(rects[parent])
        values["x"], values["y"] = left - parent_left, top - parent_top
    return values


def rgb(value):
    """A `#rrggbb` sample as (r, g, b), or None for a note (`outside`, ...)."""
    if isinstance(value, str) and re.fullmatch(r"#[0-9a-f]{6}", value):
        return tuple(int(value[i : i + 2], 16) for i in (1, 3, 5))
    return None


def within_channels(values, tolerance):
    """Whether every channel of the colours differs by at most `tolerance`."""
    colours = [rgb(v) for v in values]
    if any(c is None for c in colours):
        return False
    return all(max(c[i] for c in colours) - min(c[i] for c in colours) <= tolerance for i in range(3))


# ---------------------------------------------------------------------------
# Comparing
# ---------------------------------------------------------------------------


def covered(element, prop, raw, ruled, samples, rules):
    """The rule that covers a difference in `prop`, or None. `raw` holds the
    dumps' values (numbers) or the samples (colours), `ruled` the R-snap
    values of a geometry property."""
    if prop in ("x", "y", "w", "h"):
        values = ruled if "snap" in rules else raw
        if "snap" in rules and len({number(v) for v in values.values()}) == 1:
            return "R-snap"
        if prop == "w" and "shape" in rules and element["name"] in TEXT_RUNS:
            widest = max(values.values())
            if widest - min(values.values()) <= max(SHAPE_PX, SHAPE_SHARE * widest):
                return "R-shape"
        return None
    if "glyph" not in rules:
        return None
    if samples[prop][0] == "glyph":
        return "R-glyph (glyph)" if within_channels(raw.values(), GLYPH_CHANNEL) else None
    return "R-glyph (point)" if within_channels(raw.values(), POINT_CHANNEL) else None


def group_label(group):
    run, preset, variant = group
    return f"{preset}/{variant}" if run == RUNS[0] else f"{run}@{preset}/{variant}"


def compare(groups, elements, exceptions, kinds, rules=RULES):
    """Every difference, as (group, key, {kind: value}, reason or None,
    exceptable); a reason of None is an unexcepted difference. Also the
    exception keys that matched, the elements no kind drew, per group, how
    many differences each rule covered, and every (group, "<id>.<property>")
    compared."""
    rows, used, unseen, ruled_out, compared = [], set(), {}, {}, set()
    by_target = {}
    for key, entry in exceptions.items():
        by_target.setdefault(entry["target"], {})[key] = entry

    def differ(group, key, values, exceptable=True):
        reason = None
        if exceptable:
            match = find_exception(by_target.get(key, {}), key, group)
            if match is not None:
                used.add(match)
                reason = exceptions[match]["reason"]
        rows.append((group, key, values, reason, exceptable))

    for group, dumps in sorted(groups.items()):
        missing = [k for k in kinds if k not in dumps]
        if missing:
            differ(group, "(dump)", {k: ("yes" if k in dumps else "missing") for k in kinds}, False)
        present = [k for k in kinds if k in dumps]
        captured = [k for k in present if dumps[k]["capture"] is not None]
        if captured and len(captured) != len(present):
            differ(group, "(capture)", {k: ("yes" if k in captured else "missing") for k in present}, False)
        for kind in present:
            for ident in dumps[kind]["elements"]:
                if ident not in elements:
                    differ(group, f"{ident} (not in the list)", {kind: "drawn"}, False)
        for ident, element in elements.items():
            drawn = [k for k in present if ident in dumps[k]["elements"]]
            if not drawn:
                unseen.setdefault(group, []).append(ident)
                continue
            compared.add((group, f"{ident}.present"))
            if len(drawn) != len(present):
                differ(
                    group,
                    f"{ident}.present",
                    {k: ("yes" if k in drawn else "no") for k in present},
                )
            if len(drawn) < 2:
                continue
            measured, snapped = {}, {}
            for k in drawn:
                rects, scale = dumps[k]["elements"], dumps[k]["scale"]
                values = geometry(element, rects, scale, False)
                snapped[k] = geometry(element, rects, scale, True)
                if dumps[k]["capture"] is not None:
                    area = visible_area(ident, elements, rects, scale)
                    for name, spec in element["samples"].items():
                        values[name] = sample_colour(dumps[k]["capture"], scale, rects[ident], spec, area)
                measured[k] = values
            for prop in ("x", "y", "w", "h", *element["samples"]):
                if any(v.get(prop) == NOT_VISIBLE for v in measured.values()):
                    ruled_out[NOT_VISIBLE] = ruled_out.get(NOT_VISIBLE, 0) + 1
                raw = {k: v[prop] for k, v in measured.items() if prop in v and v[prop] != NOT_VISIBLE}
                if len(raw) < 2:
                    continue
                compared.add((group, f"{ident}.{prop}"))
                shown = {k: number(v) if is_number(v) else v for k, v in raw.items()}
                if len(set(shown.values())) == 1:
                    continue
                ruled = {k: snapped[k][prop] for k in raw if prop in snapped[k]}
                rule = covered(element, prop, raw, ruled, element["samples"], rules)
                if rule is not None:
                    ruled_out[rule] = ruled_out.get(rule, 0) + 1
                    continue
                if "snap" in rules:
                    for k, value in ruled.items():
                        if number(value) != shown[k]:
                            shown[k] = f"{number(value)} ({shown[k]})"
                differ(group, f"{ident}.{prop}", shown)
    return rows, used, unseen, ruled_out, compared


def print_table(rows, kinds):
    header = ("preset/variant", "element.property", *kinds, "status")
    table = [header]
    for group, key, values, reason, _ in rows:
        status = f"excepted: {reason}" if reason is not None else "DIFFERS"
        table.append((group_label(group), key, *(values.get(k, "-") for k in kinds), status))
    widths = [max(len(r[i]) for r in table) for i in range(len(header) - 1)]
    for r in table:
        cells = [c.ljust(w) for c, w in zip(r, widths)]
        print("  ".join(cells + [r[-1]]).rstrip())


# ---------------------------------------------------------------------------
# Merging proposals
# ---------------------------------------------------------------------------


def load_proposals(path, parity_only=False):
    """The (key, reason) pairs a proposal file holds: its top-level strings
    (unless `parity_only`), its `[parity]` table's `key = reason` pairs and
    its `[[parity.group]]` entries."""
    data = load_toml(path)
    pairs = [] if parity_only else [(k, v) for k, v in data.items() if not isinstance(v, dict)]
    for key, reason in pairs:
        if not isinstance(reason, str) or not reason.strip():
            raise Failure(f"{path}: {key}: a proposal needs a non-empty reason")
    pairs += [(key, reason) for key, reason, _ in parity_entries(path, data, flat=True)]
    return pairs


def reason_parts(reason):
    """The reasons a reason text joins with REASON_SEPARATOR, each stripped."""
    return [part.strip() for part in reason.split(REASON_SEPARATOR) if part.strip()]


def group_text(reason, keys):
    lines = ["[[parity.group]]", f"reason = {json.dumps(reason, ensure_ascii=False)}", "keys = ["]
    lines += [f"    {json.dumps(key)}," for key in keys]
    return "\n".join(lines + ["]"])


def parity_section(grouped, runs, labels):
    """The generated `[parity]` section: `grouped` = [(reason, [key])], in
    order, compared in `runs` under `labels` (preset/variant)."""
    head = [
        "[parity]",
        "# Generated by `python3 scripts/check_showcase_parity.py --merge OUT --write --proposal FILE...",
        "# [RUN=]DUMP_DIR...` against the runs " + ", ".join(runs) + " under",
        "# " + ", ".join(labels) + ".",
        "# Everything from the [parity] line down is rewritten by the next such run; the groups here are",
        "# one more proposal to it, so a group a difference still needs is kept.",
    ]
    return "\n\n".join(["\n".join(head), *(group_text(reason, keys) for reason, keys in grouped)]) + "\n"


def rewrite_parity_section(path, section):
    """Replace everything from the `[parity]` line of `path` down with
    `section`, the text above it unchanged. Returns whether the file changed."""
    try:
        with open(path, encoding="utf-8", newline="") as f:
            text = f.read()
    except OSError as err:
        raise Failure(f"could not read {path}: {err}") from err
    headers = list(PARITY_HEADER.finditer(text))
    if len(headers) != 1:
        raise Failure(f"{path}: expected one `[parity]` line to rewrite from, found {len(headers)}")
    prefix = text[: headers[0].start()]
    try:
        before, whole = tomllib.loads(prefix), tomllib.loads(text)
    except tomllib.TOMLDecodeError as err:
        raise Failure(f"{path} is not valid TOML: {err}") from err
    if {k: v for k, v in whole.items() if k != "parity"} != before:
        raise Failure(f"{path}: the [parity] section is not the file's last; --write rewrites it to the end")
    new = prefix + section
    if new == text:
        return False
    tmp = path + ".tmp"
    try:
        with open(tmp, "w", encoding="utf-8", newline="") as f:
            f.write(new)
        os.replace(tmp, path)
    except OSError as err:
        raise Failure(f"could not write {path}: {err}") from err
    return True


def minimal_scopes(occurs, universe):
    """The fewest non-overlapping scopes that together hold in every group of
    `occurs` and in no other group of `universe` (broadest first, greedy)."""

    def cover(scope):
        return {g for g in universe if holds(scope, g)}

    candidates = set()
    for run, preset, variant in occurs:
        for r in (None, run):
            candidates.update({(r, None, None), (r, preset, None), (r, preset, variant)})
    ranked = sorted(
        ((scope, cover(scope)) for scope in candidates),
        key=lambda sc: (sum(v is not None for v in sc[0]), -len(sc[1]), key_text(sc[0], "")),
    )
    chosen, remaining = [], set(occurs)
    for scope, covering in ranked:
        if not covering or not covering <= occurs or not covering & remaining:
            continue
        if any(overlaps(scope, other) for other in chosen):
            continue
        chosen.append(scope)
        remaining -= covering
    return chosen


def merge(groups, elements, presets, sources, kinds, out, write=None):
    """Write the minimal `[parity]` groups the dumps need, from the proposals
    in `sources` (path, [(key, reason)]), to `out` and, with `write`, as the
    `[parity]` section of that exceptions file; see the module's docstring.
    Returns the exit status."""
    rows, _, _, _, compared = compare(groups, elements, {}, kinds)
    occurs, universe, loose = {}, {}, []
    for group, target in compared:
        universe.setdefault(target, set()).add(group)
    for row in rows:
        group, key, _, _, exceptable = row
        if exceptable:
            occurs.setdefault(key, set()).add(group)
        else:
            loose.append(row)
    proposals, dropped = {}, []
    for path, pairs in sources:
        for key, reason in pairs:
            try:
                scope, target = parse_key(key, elements, presets)
            except BadKey as err:
                dropped.append((path, key, str(err)))
                continue
            proposals.setdefault(target, []).append((scope, reason.strip(), path, key))
    order = {ident: n for n, ident in enumerate(elements)}

    def sort_key(target):
        ident, _, prop = target.rpartition(".")
        props = [*GEOMETRY, *elements[ident]["samples"]]
        return order[ident], props.index(prop)

    by_reasons, excepted, count = {}, {}, 0
    for target in sorted(proposals, key=sort_key):
        found = occurs.get(target, set())
        needed = set()
        for scope, _, path, key in proposals[target]:
            mine = {g for g in found if holds(scope, g)}
            if not mine:
                dropped.append((path, key, "no difference in its scope needs it"))
            needed |= mine
        if not needed:
            continue
        scopes = minimal_scopes(needed, universe[target])
        for scope in sorted(scopes, key=lambda s: key_text(s, target)):
            parts = set()
            for their_scope, reason, _, _ in proposals[target]:
                if any(holds(scope, g) and holds(their_scope, g) for g in needed):
                    parts.update(reason_parts(reason))
            by_reasons.setdefault(REASON_SEPARATOR.join(sorted(parts)), []).append(key_text(scope, target))
            count += 1
        excepted[target] = needed
    uncovered = loose + [row for row in rows if row[4] and row[0] not in excepted.get(row[1], set())]

    runs = [r for r in RUNS if any(g[0] == r for g in groups)]
    labels = sorted({f"{preset}/{variant}" for _, preset, variant in groups})
    section = parity_section(list(by_reasons.items()), runs, labels)
    provenance = [
        "# The minimal [parity] groups, written by scripts/check_showcase_parity.py --merge from",
        *(f"#   {path}" for path, _ in sources),
    ]
    try:
        with open(out, "w", encoding="utf-8") as f:
            f.write("\n".join(provenance) + "\n" + section)
    except OSError as err:
        raise Failure(f"could not write {out}: {err}") from err
    load_parity_exceptions(out, elements, presets)
    changed = None
    if write is not None:
        changed = rewrite_parity_section(write, section)
        load_parity_exceptions(write, elements, presets)

    if dropped:
        print("Proposed keys dropped:")
        print("\n".join(f"  {key}  ({why}; {path})" for path, key, why in dropped))
    if uncovered:
        print("\nDifferences no proposal covers:")
        print_table(uncovered, kinds)
    print(
        f"\n{out}: {count} key(s) in {len(by_reasons)} group(s) for {len(excepted)} element properties; "
        f"{len(dropped)} proposal(s) dropped, {len(uncovered)} difference(s) no proposal covers."
    )
    if write is not None:
        print(f"{write}: [parity] section {'rewritten' if changed else 'unchanged'}.")
    return 1 if uncovered else 0


# ---------------------------------------------------------------------------
# Self-test
# ---------------------------------------------------------------------------

SELF_TEST_ELEMENTS = """
[[element]]
id = "t.panel"
name = "Panel"
parent = "window"
leaves = ["card.background_color", "card.border.color"]
states = ["Normal"]
sample = { fill = [0.5, 0.5], border = [0.0, 0.5, 0.5, 0.0] }
clip = true

[[element]]
id = "t.panel.label"
name = "Label"
parent = "t.panel"
leaves = ["defaults.font", "defaults.font.color"]
states = ["Normal"]
sample = { text = "glyph" }

[[element]]
id = "t.panel.tip"
name = "Tooltip"
parent = "t.panel"
leaves = ["tooltip.background_color"]
states = ["Normal"]
when = "the panel is hovered"
"""

BACKGROUND = (239, 240, 241)
FILL = (255, 255, 255)
LINE = (188, 190, 191)
INK = (35, 38, 39)


def self_test_capture(path, scale, panel, label, fill=FILL, frame=None, ink=INK):
    """A capture of `panel` (framed in LINE, filled with `fill`) holding
    `label` (a glyph-like stroke in `ink`); `frame` = (dx, dy) draws a window
    frame round the content."""
    w, h = round(WINDOW_SIZE[0] * scale), round(WINDOW_SIZE[1] * scale)
    content = Image.new("RGB", (w, h), BACKGROUND)
    draw = ImageDraw.Draw(content)

    def painted(rect):
        # Where a renderer that snaps edges paints the rectangle (R-snap).
        left, top = rect["x"], rect["y"]
        edges = (left, top, left + rect["w"], top + rect["h"])
        return tuple(math.floor(v * scale + 0.5) for v in edges)

    x, y, right, bottom = painted(panel)
    draw.rectangle((x, y, right - 1, bottom - 1), fill=fill, outline=LINE, width=max(1, round(scale)))
    lx, ly, _, lbottom = painted(label)
    # An anti-aliased edge, then the solid stem.
    draw.line((lx + 1, ly + 1, lx + 1, lbottom - 2), fill=(120, 122, 123))
    draw.line((lx + 2, ly + 1, lx + 2, lbottom - 2), fill=ink)
    if frame is None:
        content.save(path)
        return
    framed = Image.new("RGB", (w + frame[0] + 20, h + frame[1] + 20), (40, 40, 40))
    framed.paste(content, frame)
    framed.save(path)


def self_test():
    if Image is None:
        raise Failure("Pillow is required (pip install Pillow)")
    known = registry_paths(load_toml(REGISTRY))
    presets = {"test-preset", "other-preset"}
    panel = {"x": 20.0, "y": 30.0, "w": 200.0, "h": 60.0}
    label = {"x": 32.0, "y": 40.0, "w": 60.0, "h": 16.0}
    failures = []

    def report(name, ok, rows=()):
        print(f"  {'pass' if ok else 'FAIL'}: {name}")
        if not ok:
            failures.append(name)
            if rows:
                print_table(rows, KINDS)

    def write_file(path, text):
        with open(path, "w", encoding="utf-8") as f:
            f.write(text)

    def write_run(directory, change=None, frame=None, combos=(("test-preset", "light"),)):
        """Dumps and captures of every kind under each (preset, variant) of
        `combos`, each in a subdirectory of its own, as the capture scripts
        write them; `change(kind, rects, preset, variant)` alters them."""
        for preset, variant in combos:
            folder = os.path.join(directory, f"{preset}-{variant}")
            os.makedirs(folder)
            for kind in KINDS:
                rects = {"t.panel": dict(panel), "t.panel.label": dict(label)}
                paint = (change(kind, rects, preset, variant) if change is not None else None) or {}
                stem = os.path.join(folder, f"{kind}-{preset}-{variant}")
                dump = {"kind": kind, "preset": preset, "variant": variant, "scale": 1.0, "elements": rects}
                write_file(stem + ".json", json.dumps(dump))
                self_test_capture(
                    stem + ".png",
                    1.0,
                    rects.get("t.panel", panel),
                    rects.get("t.panel.label", label),
                    paint.get("fill", FILL),
                    frame,
                    paint.get("ink", INK),
                )
                if frame is not None:
                    write_file(stem + ".offset", f"+{frame[0]}+{frame[1]}\n")

    def load_test_elements(tmp):
        path = os.path.join(tmp, "elements.toml")
        write_file(path, SELF_TEST_ELEMENTS)
        return load_elements(path, known)

    def parity(*grouped):
        """A `[parity]` section of the groups (reason, [key])."""
        return "[parity]\n" + "".join(group_text(reason, keys) + "\n" for reason, keys in grouped)

    def scenario(
        name,
        expect_unexcepted,
        expect_keys=(),
        change=None,
        exceptions=(),
        frame=None,
        absent_keys=(),
        run=RUNS[0],
        combos=(("test-preset", "light"),),
    ):
        with tempfile.TemporaryDirectory() as tmp:
            elements = load_test_elements(tmp)
            exceptions_path = os.path.join(tmp, "exceptions.toml")
            write_file(exceptions_path, parity(*exceptions))
            excepted = load_parity_exceptions(exceptions_path, elements, presets)
            directory = os.path.join(tmp, "run")
            write_run(directory, change, frame, combos)
            groups = load_run(directory, KINDS, None, run)
            rows = compare(groups, elements, excepted, KINDS)[0]
        unexcepted = [row[1] for row in rows if row[3] is None]
        ok = (
            bool(unexcepted) == expect_unexcepted
            and all(k in unexcepted for k in expect_keys)
            and not any(k in unexcepted for k in absent_keys)
        )
        report(name, ok, rows)

    def sampled(name, rect, spec, ground, boxes, expect):
        """`spec` sampled on `rect` in a capture of `ground` with each
        (x0, y0, x1, y1, colour) of `boxes` painted over it, inclusive device
        pixels at scale 1, must show `expect`."""
        image = Image.new("RGB", WINDOW_SIZE, ground)
        draw = ImageDraw.Draw(image)
        for *box, colour in boxes:
            draw.rectangle(box, fill=colour)
        capture = {"image": image, "pixels": image.load(), "offset": (0, 0)}
        got = sample_colour(capture, 1.0, rect, parse_sample(name, "sample", spec))
        report(name, got == expect)
        if got != expect:
            print(f"    expected {expect}\n    got      {got}")

    def rejected(name, exceptions, grouped=True):
        """An exceptions section `--check-list` must refuse: groups
        (reason, keys) or, with `grouped` false, the text after `[parity]`."""
        with tempfile.TemporaryDirectory() as tmp:
            elements = load_test_elements(tmp)
            path = os.path.join(tmp, "exceptions.toml")
            write_file(path, parity(*exceptions) if grouped else "[parity]\n" + exceptions)
            try:
                load_parity_exceptions(path, elements, presets)
            except Failure:
                report(name, True)
                return
        report(name, False)

    def merge_quietly(tmp, proposals, change, runs, combos, write=None):
        """--merge from `proposals` against dumps of `runs` x `combos` made
        in `tmp` (once), to tmp/merged.toml; its groups as [(reason, keys)]."""
        elements = load_test_elements(tmp)
        groups = {}
        for run in runs:
            directory = os.path.join(tmp, run)
            if not os.path.isdir(directory):
                write_run(directory, lambda *a, run=run: change(run, *a), None, combos)
            load_run(directory, KINDS, None, run, groups)
        sources = []
        for n, text in enumerate(proposals):
            path = os.path.join(tmp, f"proposal-{n}.toml")
            write_file(path, text)
            sources.append((path, load_proposals(path)))
        if write is not None:
            sources.append((write, load_proposals(write, parity_only=True)))
        out = os.path.join(tmp, "merged.toml")
        with open(os.devnull, "w", encoding="utf-8") as quiet:
            stdout, sys.stdout = sys.stdout, quiet
            try:
                merge(groups, elements, presets, sources, KINDS, out, write)
            finally:
                sys.stdout = stdout
        return [(g["reason"], g["keys"]) for g in load_toml(out).get("parity", {}).get("group", [])]

    def merged(name, proposals, expect, change, runs, combos):
        """--merge's groups from `proposals` against dumps of `runs` x
        `combos` must be exactly `expect`, [(reason, keys)] in order."""
        with tempfile.TemporaryDirectory() as tmp:
            got = merge_quietly(tmp, proposals, change, runs, combos)
        report(name, got == expect)
        if got != expect:
            print(f"    expected {expect}\n    got      {got}")

    def read_file(path):
        with open(path, encoding="utf-8", newline="") as f:
            return f.read()

    def wider(kind, rects, *_):
        if kind == "egui":
            rects["t.panel"]["w"] += 1.0

    def recoloured(kind, rects, *_):
        return {"fill": (250, 250, 250)} if kind == "iced" else None

    def edge_off(by):
        def change(kind, rects, *_):
            if kind == "egui":
                rects["t.panel"]["w"] += by

        return change

    def straddling(kind, rects, *_):
        # Left edges 20, 20.4, 19.6 and right edges 220, 219.8, 220.1: all
        # round to 20 and 220, while the widths 200, 199.4 and 200.5 would
        # round to 200, 199 and 201.
        rects["t.panel"]["x"], rects["t.panel"]["w"] = {
            "gpui": (20.0, 200.0),
            "iced": (20.4, 199.4),
            "egui": (19.6, 200.5),
        }[kind]

    def parent_fraction(kind, rects, *_):
        # The parent's left edge 19.6 rounds to 20; the child's 32.4 to 32:
        # 12 from the parent, as in gpui, though 32.4 - 19.6 = 12.8 would
        # round to 13.
        if kind == "iced":
            rects["t.panel"]["x"], rects["t.panel"]["w"] = 19.6, 200.4
            rects["t.panel.label"]["x"] = 32.4

    def text_wider(share):
        def change(kind, rects, *_):
            rects["t.panel.label"]["w"] = 150.0
            if kind == "iced":
                rects["t.panel.label"]["w"] *= 1 + share

        return change

    def text_moved(kind, rects, *_):
        if kind == "iced":
            rects["t.panel.label"]["x"] += 1.2

    def ink_off(by):
        def change(kind, rects, *_):
            return {"ink": tuple(c + by for c in INK)} if kind == "gpui" else None

        return change

    def fill_off(by):
        def change(kind, rects, *_):
            return {"fill": tuple(c - by for c in FILL)} if kind == "egui" else None

        return change

    def dropped(kind, rects, *_):
        if kind == "iced":
            del rects["t.panel.label"]

    def label_clipped(kind, rects, *_):
        # Below the panel that clips it, in every kind: its glyph, drawn in
        # the capture in a colour of its own in gpui, is not visible.
        rects["t.panel.label"]["y"] = 95.0
        return {"ink": tuple(c + 60 for c in INK)} if kind == "gpui" else None

    def dropped_everywhere(kind, rects, *_):
        del rects["t.panel.label"]

    def moved_together(kind, rects, *_):
        if kind == "gpui":
            for rect in rects.values():
                rect["x"] += 5.0

    def moved_child(kind, rects, *_):
        if kind == "gpui":
            rects["t.panel.label"]["y"] += 1.0

    def tip_in_one(kind, rects, *_):
        if kind == "egui":
            rects["t.panel.tip"] = {"x": 40.0, "y": 95.0, "w": 80.0, "h": 24.0}

    def tip_in_all(kind, rects, *_):
        rects["t.panel.tip"] = {"x": 40.0, "y": 95.0, "w": 80.0, "h": 24.0}

    def wider_in(*where):
        """egui's panel 1 px wider where (run, preset, variant) matches one of
        `where`, None matching any."""

        def change(run, kind, rects, preset, variant):
            if any(holds(scope, (run, preset, variant)) for scope in where):
                wider(kind, rects)

        return change

    both = (("test-preset", "light"), ("test-preset", "dark"))
    three = (*both, ("other-preset", "light"))

    print("check_showcase_parity self-test:")
    scenario("identical dumps and captures pass", False)
    scenario("identical, framed captures with .offset sidecars pass", False, frame=(65, 81))
    scenario("a 1 px size difference fails", True, ("t.panel.w",), wider)
    scenario("a colour difference fails", True, ("t.panel.fill",), recoloured)
    scenario(
        "an excepted difference passes",
        False,
        change=wider,
        exceptions=(("self-test reason", ["t.panel.w"]),),
    )
    scenario("an element missing in one kind fails", True, ("t.panel.label.present",), dropped)
    scenario(
        "a parent moved with its child: the parent differs, the child does not",
        True,
        ("t.panel.x",),
        moved_together,
        absent_keys=("t.panel.label.x", "t.panel.label.y"),
    )
    scenario("a child moved inside its parent fails", True, ("t.panel.label.y",), moved_child)
    scenario("a conditional element drawn by every kind is compared and passes", False, change=tip_in_all)
    scenario("a conditional element drawn by one kind only fails", True, ("t.panel.tip.present",), tip_in_one)
    scenario("an element no kind draws, though the list has no `when` for it, passes", False, change=dropped_everywhere)
    scenario("R-snap: an edge 0.4 px off passes", False, change=edge_off(0.4))
    scenario("R-snap: an edge 1.2 px off fails", True, ("t.panel.w",), edge_off(1.2))
    scenario("R-snap: edges that round alike pass, though the widths round apart", False, change=straddling)
    scenario(
        "R-snap: a child whose edges round alike passes, though its parent's fractions differ",
        False,
        change=parent_fraction,
    )
    scenario("R-shape: a text run 1.5 % wider passes", False, change=text_wider(0.015))
    scenario("R-shape: a text run 3 % wider fails", True, ("t.panel.label.w",), text_wider(0.03))
    scenario("R-shape: a text run's left edge 1.2 px off fails", True, ("t.panel.label.x",), text_moved)
    scenario("R-glyph: a glyph sample 7 per channel off passes", False, change=ink_off(7))
    scenario("R-glyph: a glyph sample 9 per channel off fails", True, ("t.panel.label.text",), ink_off(9))
    scenario("R-glyph: a fill 1 per channel off passes", False, change=fill_off(1))
    scenario("R-glyph: a fill 2 per channel off fails", True, ("t.panel.fill",), fill_off(2))
    scenario(
        "a sample outside a `clip` ancestor is not visible, and not a difference",
        False,
        change=label_clipped,
    )
    # A row highlight painted on rows 40 to 55 (y = 39.6, h = 16 rounded)
    # over a darker ground, a glyph stem in it: a floor of 39.6 would take in
    # row 39, the ground, farther from the highlight than the glyph is.
    sampled(
        "R-snap: a fractional text box samples the glyph, not the row beside its rounded edge",
        {"x": 32.0, "y": 39.6, "w": 60.0, "h": 16.0},
        "glyph",
        INK,
        ((32, 40, 91, 55, LINE), (40, 43, 40, 52, FILL)),
        hex_colour(FILL),
    )
    # A 12x12 icon box whose filled glyph covers 100 of its 144 pixels: the
    # box's most frequent colour is the glyph, its outer ring the ground.
    sampled(
        "R-glyph: a glyph covering most of a tight box is the sample, its ring the ground",
        {"x": 100.0, "y": 100.0, "w": 12.0, "h": 12.0},
        "glyph",
        BACKGROUND,
        ((101, 101, 110, 110, FILL),),
        hex_colour(FILL),
    )
    # A framed panel painted from (20, 30) to (219, 89), its edges 19.6,
    # 29.6, 220 and 89.6 rounded: a floor would sample the ground outside.
    framed_panel = ((20, 30, 219, 89, LINE), (21, 31, 218, 88, FILL))
    fractional_panel = {"x": 19.6, "y": 29.6, "w": 200.4, "h": 60.0}
    sampled(
        "R-snap: a border sample on a fractional top edge hits the rounded border row",
        fractional_panel,
        [0.5, 0.0],
        BACKGROUND,
        framed_panel,
        hex_colour(LINE),
    )
    sampled(
        "R-snap: a border sample on a fractional left edge hits the rounded border column",
        fractional_panel,
        [0.0, 0.5],
        BACKGROUND,
        framed_panel,
        hex_colour(LINE),
    )
    sampled(
        "R-snap: a border sample inside a fractional bottom edge hits the rounded border row",
        fractional_panel,
        [0.5, 1.0, 0.0, -0.5],
        BACKGROUND,
        framed_panel,
        hex_colour(LINE),
    )

    scenario(
        "a preset-scoped exception passes under its preset",
        False,
        change=wider,
        exceptions=(("self-test reason", ["test-preset:t.panel.w"]),),
        combos=both,
    )
    scenario(
        "a preset-scoped exception does not hide the difference under another preset",
        True,
        ("t.panel.w",),
        wider,
        (("self-test reason", ["other-preset:t.panel.w"]),),
    )
    scenario(
        "a preset/variant-scoped exception passes in its variant",
        False,
        change=wider,
        exceptions=(("self-test reason", ["test-preset/light:t.panel.w"]),),
    )
    scenario(
        "a preset/variant-scoped exception does not hide the difference in the other variant",
        True,
        ("t.panel.w",),
        wider,
        (("self-test reason", ["test-preset/light:t.panel.w"]),),
        combos=both,
    )
    scenario(
        "a run-scoped exception passes in its run",
        False,
        change=wider,
        exceptions=(("self-test reason", ["hover@t.panel.w"]),),
        run="hover",
    )
    scenario(
        "a run-scoped exception does not hide the difference in another run",
        True,
        ("t.panel.w",),
        wider,
        (("self-test reason", ["hover@t.panel.w"]),),
    )
    scenario(
        "a run- and preset-scoped exception passes in its run under its preset",
        False,
        change=wider,
        exceptions=(("self-test reason", ["menu@test-preset/light:t.panel.w"]),),
        run="menu",
    )
    scenario(
        "a group's keys each except their own difference",
        False,
        change=lambda *a: wider(*a) or recoloured(*a),
        exceptions=(("self-test reason", ["t.panel.w", "t.panel.fill"]),),
    )
    rejected("--check-list refuses an unknown preset", (("r", ["no-such-preset:t.panel.w"]),))
    rejected("--check-list refuses an unknown variant", (("r", ["test-preset/dim:t.panel.w"]),))
    rejected("--check-list refuses an unknown run", (("r", ["drag@t.panel.w"]),))
    rejected("--check-list refuses a variant without its preset", (("r", ["light:t.panel.w"]),))
    rejected("--check-list refuses an unknown element", (("r", ["t.panel.frame.w"]),))
    rejected("--check-list refuses an unknown property", (("r", ["test-preset:t.panel.colour"]),))
    rejected(
        "--check-list refuses overlapping scopes of one property in one group",
        (("r", ["t.panel.w", "test-preset:t.panel.w"]),),
    )
    rejected(
        "--check-list refuses overlapping scopes of one property across groups",
        (("r", ["t.panel.w"]), ("s", ["test-preset/light:t.panel.w"])),
    )
    rejected(
        "--check-list refuses a run scope overlapping a preset scope across groups",
        (("r", ["hover@t.panel.w"]), ("s", ["test-preset:t.panel.w"])),
    )
    rejected("--check-list refuses a key listed twice in one group", (("r", ["t.panel.w", "t.panel.w"]),))
    rejected("--check-list refuses a key listed in two groups", (("r", ["t.panel.w"]), ("s", ["t.panel.w"])))
    rejected("--check-list refuses a group with no keys", (("r", []),))
    rejected("--check-list refuses a group with a blank reason", ((" ", ["t.panel.w"]),))
    rejected("--check-list refuses a group without a reason", '[[parity.group]]\nkeys = ["t.panel.w"]\n', False)
    rejected(
        "--check-list refuses a group with a field other than reason and keys",
        '[[parity.group]]\nreason = "r"\nkeys = ["t.panel.w"]\nnote = "n"\n',
        False,
    )
    rejected("--check-list refuses a flat key = reason in the exceptions file", '"t.panel.w" = "r"\n', False)

    merged(
        "--merge drops a key no difference needs and scopes a kept one to its preset/variant",
        ['"t.panel.w" = "wider"\n"t.panel.h" = "taller"\n'],
        [("wider", ["test-preset/light:t.panel.w"])],
        wider_in((None, "test-preset", "light")),
        (RUNS[0],),
        three,
    )
    merged(
        "--merge scopes to a preset where both variants differ and joins distinct reasons, sorted",
        ['"t.panel.w" = "wider"\n', '[parity]\n"test-preset:t.panel.w" = "one more"\n"t.panel.w" = "wider"\n'],
        [("one more | wider", ["test-preset:t.panel.w"])],
        wider_in((None, "test-preset", None)),
        (RUNS[0],),
        three,
    )
    merged(
        "--merge leaves a difference found everywhere unscoped",
        ['"test-preset:t.panel.w" = "wider"\n', '"other-preset:t.panel.w" = "wider"\n'],
        [("wider", ["t.panel.w"])],
        wider_in((None, None, None)),
        (RUNS[0], "hover"),
        three,
    )
    merged(
        "--merge scopes a difference found in one run only to that run",
        ['"t.panel.w" = "wider"\n'],
        [("wider", ["hover@t.panel.w"])],
        wider_in(("hover", None, None)),
        (RUNS[0], "hover"),
        three,
    )
    merged(
        "--merge keeps a proposal's scope: a difference outside it stays uncovered",
        ['"other-preset:t.panel.w" = "wider"\n'],
        [("wider", ["other-preset:t.panel.w"])],
        wider_in((None, None, None)),
        (RUNS[0],),
        three,
    )

    def moved_and_wider(run, kind, rects, *_):
        if kind == "egui":
            for prop in ("w", "h"):
                rects["t.panel"][prop] += 1.0
            for prop in ("x", "y"):
                rects["t.panel.label"][prop] += 1.0

    merged(
        "--merge groups the keys of one reason set, whatever order and form the proposals gave it in",
        [
            '"t.panel.w" = "shared"\n"t.panel.h" = "shared"\n"t.panel.label.y" = "b"\n"t.panel.label.x" = "a"\n',
            '[parity]\n"t.panel.label.x" = "b | a"\n\n[[parity.group]]\nreason = "a"\nkeys = ["t.panel.label.y"]\n',
        ],
        [("shared", ["t.panel.w", "t.panel.h"]), ("a | b", ["t.panel.label.x", "t.panel.label.y"])],
        moved_and_wider,
        (RUNS[0],),
        three,
    )

    prefix = (
        "# Head comment.\n[gpui]\nFoo = \"bar\"   # odd  spacing, kept\r\n\n[egui]\n\n"
        "# The parity section's own documentation, kept.\n"
    )
    stale = parity(("wider", ["t.panel.w"]), ("stale", ["t.panel.x"]))
    with tempfile.TemporaryDirectory() as tmp:
        exceptions_path = os.path.join(tmp, "exceptions.toml")
        write_file(exceptions_path, prefix + stale)
        proposals = ['"t.panel.w" = "more"\n"t.panel.h" = "taller"\n']
        change = wider_in((None, None, None))
        got = merge_quietly(tmp, proposals, change, (RUNS[0],), three, exceptions_path)
        first = read_file(exceptions_path)
        merge_quietly(tmp, proposals, change, (RUNS[0],), three, exceptions_path)
        second = read_file(exceptions_path)
        written = [(g["reason"], g["keys"]) for g in load_toml(exceptions_path)["parity"]["group"]]
        out = read_file(os.path.join(tmp, "merged.toml"))
    expect = [("more | wider", ["t.panel.w"])]
    report(
        "--merge --write rewrites only the [parity] section, the text above it byte for byte",
        first.startswith(prefix + "[parity]\n") and written == expect and got == expect,
    )
    report("--merge --write run twice changes nothing", first == second and out.endswith(first[len(prefix) :]))

    def refused_rewrite(name, text):
        with tempfile.TemporaryDirectory() as tmp:
            path = os.path.join(tmp, "exceptions.toml")
            write_file(path, text)
            try:
                rewrite_parity_section(path, parity())
            except Failure:
                report(name, read_file(path) == text)
                return
        report(name, False)

    refused_rewrite("--write refuses a file whose [parity] section is not the last", stale + '\n[gpui]\nX = "y"\n')
    refused_rewrite("--write refuses a file with no [parity] line", prefix)
    if failures:
        print(f"self-test FAILED: {len(failures)} scenario(s)")
        return 1
    print("self-test passed")
    return 0


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def parse_offset(text):
    match = OFFSET.match(text)
    if not match:
        raise argparse.ArgumentTypeError("expected X,Y (physical pixels)")
    return int(match.group(1)), int(match.group(2))


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "dump_dirs",
        nargs="*",
        metavar="[RUN=]DUMP_DIR",
        help=f"directory of <kind>-<preset>-<variant>.json/.png; RUN is one of {', '.join(RUNS)} "
        f"(default {RUNS[0]})",
    )
    parser.add_argument(
        "--content-offset",
        type=parse_offset,
        metavar="X,Y",
        help="where the content sits in a framed capture that has no .offset file",
    )
    parser.add_argument(
        "--kinds",
        default=",".join(KINDS),
        help="the showcase kinds to compare, comma-separated (default: all three)",
    )
    parser.add_argument("--elements", default=ELEMENTS, help="the element list")
    parser.add_argument("--exceptions", default=EXCEPTIONS, help="the exceptions file")
    parser.add_argument("--check-list", action="store_true", help="validate the list and exceptions only")
    parser.add_argument("--self-test", action="store_true", help="run the built-in scenarios")
    parser.add_argument(
        "--merge",
        metavar="OUT",
        help="write the minimal [parity] groups the dumps need, from the --proposal files, to OUT",
    )
    parser.add_argument(
        "--proposal",
        action="append",
        default=[],
        metavar="FILE",
        help="a TOML file of proposed exceptions, for --merge (repeatable)",
    )
    parser.add_argument(
        "--write",
        action="store_true",
        help="with --merge: also rewrite the [parity] section of the exceptions file in place",
    )
    args = parser.parse_args()

    if args.self_test:
        return self_test()
    kinds = tuple(k for k in args.kinds.split(",") if k)
    if not kinds or any(k not in KINDS for k in kinds) or len(set(kinds)) != len(kinds):
        raise Failure(f"--kinds takes names from {', '.join(KINDS)}")
    presets = preset_names()
    elements = load_elements(args.elements, registry_paths(load_toml(REGISTRY)))
    exceptions = load_parity_exceptions(args.exceptions, elements, presets)
    names = {element["name"] for element in elements.values()}
    stale_runs = sorted(TEXT_RUNS - names)
    if stale_runs:
        raise Failure(f"TEXT_RUNS names what no element of {args.elements} is named: {', '.join(stale_runs)}")
    if args.check_list:
        runs = sum(1 for element in elements.values() if element["name"] in TEXT_RUNS)
        scoped = sum(1 for entry in exceptions.values() if entry["scope"] != (None, None, None))
        groups = len({entry["group"] for entry in exceptions.values()})
        print(
            f"{len(elements)} elements ({runs} text runs), {len(exceptions)} parity exceptions "
            f"({scoped} scoped) in {groups} group(s): valid"
        )
        return 0
    if not args.dump_dirs:
        parser.error("a dump directory is required (or --check-list / --self-test)")
    if len(kinds) < 2:
        raise Failure("comparing needs at least two kinds")
    if args.proposal and args.merge is None:
        parser.error("--proposal is read by --merge")
    if args.write and args.merge is None:
        parser.error("--write rewrites the exceptions file from --merge's result")

    groups = {}
    for text in args.dump_dirs:
        run, directory = parse_run_dir(text)
        load_run(directory, kinds, args.content_offset, run, groups)
    if args.merge is not None:
        if not args.proposal:
            parser.error("--merge needs at least one --proposal file")
        out = os.path.realpath(args.merge)
        if out in (os.path.realpath(EXCEPTIONS), os.path.realpath(args.exceptions)):
            raise Failure(f"--merge writes a proposal, never the exceptions file {args.merge} itself")
        sources = [(path, load_proposals(path)) for path in args.proposal]
        sources.append((args.exceptions, load_proposals(args.exceptions, parity_only=True)))
        return merge(
            groups, elements, presets, sources, kinds, args.merge, args.exceptions if args.write else None
        )

    rows, used, unseen, ruled_out, _ = compare(groups, elements, exceptions, kinds)
    if rows:
        print_table(rows, kinds)
    for group, idents in sorted(unseen.items()):
        print(f"\n{group_label(group)}: {len(idents)} element(s) no kind drew:")
        print(
            "\n".join(
                f"  {ident}  [{elements[ident]['when'] or 'no `when`: the list says it is always drawn'}]"
                for ident in idents
            )
        )
    unexcepted = sum(1 for row in rows if row[3] is None)
    applicable = {
        key for key, entry in exceptions.items() if any(holds(entry["scope"], group) for group in groups)
    }
    stale = sorted(applicable - used)
    if stale:
        print("\nParity exceptions that matched no difference in this run:")
        print("\n".join("  " + key for key in stale))
    covered_by = ", ".join(f"{rule} {count}" for rule, count in sorted(ruled_out.items())) or "none"
    print(
        f"\n{len(groups)} preset/variant group(s), {len(rows)} difference(s), "
        f"{len(rows) - unexcepted} excepted, {unexcepted} not excepted; "
        f"covered by a rule: {covered_by}."
    )
    return 1 if unexcepted else 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Failure as error:
        print(f"check_showcase_parity: {error}", file=sys.stderr)
        sys.exit(2)
