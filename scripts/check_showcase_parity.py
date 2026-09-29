#!/usr/bin/env python3
"""Compare the three showcases element by element: sizes, positions, colours.

The gpui, iced and egui showcases draw one shared set of elements, listed once
in docs/showcase-elements.toml. Under the same theme the three must lay each
element out alike and paint it in the same colours, because all three follow
the theme data. This script is the mechanical gate for that: it reads what the
showcases report about their own layout, and what their captures show, and
fails on every difference not recorded, with its reason, in the `[parity]`
table of docs/showcase-exceptions.toml. There is no silent tolerance.

Run from the repository root:

    python3 scripts/check_showcase_parity.py DUMP_DIR [--content-offset X,Y]
    python3 scripts/check_showcase_parity.py --check-list
    python3 scripts/check_showcase_parity.py --self-test

Requires: Python 3.11+ (tomllib) and Pillow.

Inputs
------
DUMP_DIR holds, for each showcase kind, preset and variant it was run under:

    <kind>-<preset>-<variant>.json   the layout dump (required)
    <kind>-<preset>-<variant>.png    the capture (optional)
    <kind>-<preset>-<variant>.offset where the content sits in a framed
                                     capture (optional, see below)

`kind` is `gpui`, `iced` or `egui`; `variant` is `light` or `dark`; the
preset is the rest of the name (`kde-breeze`, `adwaita`, ...).

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
  end of the report, not counted as a difference;
- size: `w` and `h`;
- position: `x` and `y` relative to the element's parent, the element the
  list names in `parent`; for `parent = "window"` the content origin;
- colour, where the captures are there: every `sample` point the list gives
  the element, sampled in each capture (see docs/showcase-elements.toml for
  the point syntax). A colour is `#rrggbb`.

Numbers are compared after rounding to 1/100 of a logical pixel: the dumps are
f32 layout results, and a difference below that is float arithmetic, not
layout. Colours are compared exactly.

Exceptions
----------
docs/showcase-exceptions.toml's `[parity]` table lists what may differ, one
entry per element property, with its reason: a toolkit limit with its
upstream citation, or a detail the theme leaves unstated.

    [parity]
    "basic.spinner.present" = "..."
    "chrome.splitter.w" = "..."
    "basic.checkbox.checked.label.text" = "..."

The key is `<element id>.<property>`, where the property is `present`, `x`,
`y`, `w`, `h` or the name of one of the element's samples. An exception holds
for every preset and variant. A key naming no element or property of the list
is an error; an exception that matched no difference in the run is listed at
the end, so a stale one is seen.

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

KINDS = ("gpui", "iced", "egui")
VARIANTS = ("light", "dark")
# The parent of a top-level element: the window's content area.
WINDOW = "window"
# The showcase window's content size in logical pixels
# (scripts/capture_window.sh:32-33).
WINDOW_SIZE = (1280, 720)
# Element properties that are not samples.
GEOMETRY = ("present", "x", "y", "w", "h")
# Decimal places numbers are compared at (1/100 logical pixel).
PLACES = 2

ID = re.compile(r"^[a-z][a-z0-9_]*(\.[a-z0-9_]+)*$")
SAMPLE = re.compile(r"^[a-z][a-z0-9_]*$")
DUMP_NAME = re.compile(r"^(gpui|iced|egui)-(.+)-(light|dark)\.json$")
OFFSET = re.compile(r"^\s*\+?(\d+)\s*[+,]\s*(\d+)\s*$")


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
        unknown = set(row) - {"id", "name", "parent", "leaves", "states", "sample", "part", "when"}
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


def load_parity_exceptions(path, elements):
    """The `[parity]` table: "<id>.<property>" -> reason."""
    data = load_toml(path)
    table = data.get("parity", {})
    if not isinstance(table, dict):
        raise Failure(f"{path}: [parity] must be a table")
    exceptions = {}
    for key, reason in table.items():
        if not isinstance(reason, str) or not reason.strip():
            raise Failure(f"{path}: [parity] {key}: an exception needs a non-empty reason")
        ident, _, prop = key.rpartition(".")
        element = elements.get(ident)
        if element is None:
            raise Failure(f"{path}: [parity] {key}: `{ident}` is not an element of the list")
        if prop not in GEOMETRY and prop not in element["samples"]:
            raise Failure(
                f"{path}: [parity] {key}: `{prop}` is neither one of {', '.join(GEOMETRY)} "
                f"nor a sample of {ident}"
            )
        exceptions[key] = reason
    return exceptions


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


def load_run(directory, kinds, cli_offset):
    """(preset, variant) -> kind -> {"scale", "elements", "capture"}."""
    if not os.path.isdir(directory):
        raise Failure(f"not a directory: {directory}")
    groups = {}
    for name in sorted(os.listdir(directory)):
        match = DUMP_NAME.match(name)
        if not match or match.group(1) not in kinds:
            continue
        kind, preset, variant = match.groups()
        stem = os.path.join(directory, name[: -len(".json")])
        dump = load_dump(stem + ".json", kind, preset, variant)
        png = stem + ".png"
        dump["capture"] = (
            load_capture(png, dump["scale"], stem + ".offset", cli_offset)
            if os.path.isfile(png)
            else None
        )
        groups.setdefault((preset, variant), {})[kind] = dump
    if not groups:
        raise Failure(f"{directory}: no <kind>-<preset>-<variant>.json dumps")
    return groups


# ---------------------------------------------------------------------------
# Measuring
# ---------------------------------------------------------------------------


def hex_colour(rgb):
    return "#{:02x}{:02x}{:02x}".format(*rgb[:3])


def sample_colour(capture, scale, rect, spec):
    """The colour a sample point shows, or a note saying why there is none."""
    image, pixels, (ox, oy) = capture["image"], capture["pixels"], capture["offset"]
    content_w = round(WINDOW_SIZE[0] * scale)
    content_h = round(WINDOW_SIZE[1] * scale)
    if spec[0] == "point":
        _, fx, fy, dx, dy = spec
        px = math.floor((rect["x"] + fx * rect["w"] + dx) * scale)
        py = math.floor((rect["y"] + fy * rect["h"] + dy) * scale)
        if not (0 <= px < content_w and 0 <= py < content_h):
            return "outside"
        return hex_colour(pixels[ox + px, oy + py])
    # A glyph sample: the pixel of the box that differs most from the box's
    # most frequent colour (the ground the text is drawn on); the first such
    # pixel in raster order.
    _, x0, y0, x1, y1 = spec
    left = max(0, math.floor((rect["x"] + x0 * rect["w"]) * scale))
    top = max(0, math.floor((rect["y"] + y0 * rect["h"]) * scale))
    right = min(content_w, math.ceil((rect["x"] + x1 * rect["w"]) * scale))
    bottom = min(content_h, math.ceil((rect["y"] + y1 * rect["h"]) * scale))
    if right <= left or bottom <= top:
        return "outside"
    box = image.crop((ox + left, oy + top, ox + right, oy + bottom))
    counts = box.getcolors(maxcolors=box.size[0] * box.size[1])
    if not counts:
        return "outside"
    ground = max(counts, key=lambda c: (c[0], c[1]))[1]
    best, best_distance = None, 0
    data = box.load()
    for y in range(box.size[1]):
        for x in range(box.size[0]):
            p = data[x, y]
            distance = sum((a - b) ** 2 for a, b in zip(p, ground))
            if distance > best_distance:
                best, best_distance = p, distance
    return hex_colour(best) if best is not None else "no glyph"


def number(value):
    return f"{round(value, PLACES):.{PLACES}f}".rstrip("0").rstrip(".")


def relative(element, rects):
    """The element's position relative to its parent, or None when the
    parent is not drawn."""
    rect = rects[element["id"]]
    parent = element["parent"]
    if parent == WINDOW:
        return rect["x"], rect["y"]
    if parent not in rects:
        return None
    return rect["x"] - rects[parent]["x"], rect["y"] - rects[parent]["y"]


# ---------------------------------------------------------------------------
# Comparing
# ---------------------------------------------------------------------------


def compare(groups, elements, exceptions, kinds):
    """Every difference, as (group, key, {kind: value}, reason or None);
    a reason of None is an unexcepted difference. Also the exception keys
    that matched, and the conditional elements no kind drew, per group."""
    rows, used, unseen = [], set(), {}

    def differ(group, key, values, exceptable=True):
        reason = exceptions.get(key) if exceptable else None
        if reason is not None:
            used.add(key)
        rows.append((group, key, values, reason))

    for (preset, variant), dumps in sorted(groups.items()):
        group = f"{preset}/{variant}"
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
            if not drawn and element["when"] is not None:
                unseen.setdefault(group, []).append(ident)
                continue
            if len(drawn) != len(present):
                differ(
                    group,
                    f"{ident}.present",
                    {k: ("yes" if k in drawn else "no") for k in present},
                )
            if len(drawn) < 2:
                continue
            measured = {}
            for k in drawn:
                rects = dumps[k]["elements"]
                rect = rects[ident]
                values = {"w": number(rect["w"]), "h": number(rect["h"])}
                at = relative(element, rects)
                if at is not None:
                    values["x"], values["y"] = number(at[0]), number(at[1])
                if dumps[k]["capture"] is not None:
                    for name, spec in element["samples"].items():
                        values[name] = sample_colour(dumps[k]["capture"], dumps[k]["scale"], rect, spec)
                measured[k] = values
            for prop in ("x", "y", "w", "h", *element["samples"]):
                values = {k: v[prop] for k, v in measured.items() if prop in v}
                if len(values) >= 2 and len(set(values.values())) > 1:
                    differ(group, f"{ident}.{prop}", values)
    return rows, used, unseen


def print_table(rows, kinds):
    header = ("preset/variant", "element.property", *kinds, "status")
    table = [header]
    for group, key, values, reason in rows:
        status = f"excepted: {reason}" if reason is not None else "DIFFERS"
        table.append((group, key, *(values.get(k, "-") for k in kinds), status))
    widths = [max(len(r[i]) for r in table) for i in range(len(header) - 1)]
    for r in table:
        cells = [c.ljust(w) for c, w in zip(r, widths)]
        print("  ".join(cells + [r[-1]]).rstrip())


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


def self_test_capture(path, scale, panel, label, fill=FILL, frame=None):
    """A capture of `panel` (framed in LINE, filled with `fill`) holding
    `label` (a glyph-like stroke in INK); `frame` = (dx, dy) draws a window
    frame round the content."""
    w, h = round(WINDOW_SIZE[0] * scale), round(WINDOW_SIZE[1] * scale)
    content = Image.new("RGB", (w, h), BACKGROUND)
    draw = ImageDraw.Draw(content)
    x, y, pw, ph = (round(v * scale) for v in (panel["x"], panel["y"], panel["w"], panel["h"]))
    draw.rectangle((x, y, x + pw - 1, y + ph - 1), fill=fill, outline=LINE, width=max(1, round(scale)))
    lx, ly, lw, lh = (round(v * scale) for v in (label["x"], label["y"], label["w"], label["h"]))
    # An anti-aliased edge, then the solid stem.
    draw.line((lx + 1, ly + 1, lx + 1, ly + lh - 2), fill=(120, 122, 123))
    draw.line((lx + 2, ly + 1, lx + 2, ly + lh - 2), fill=INK)
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
    panel = {"x": 20.0, "y": 30.0, "w": 200.0, "h": 60.0}
    label = {"x": 32.0, "y": 40.0, "w": 60.0, "h": 16.0}
    failures = []

    def scenario(
        name, expect_unexcepted, expect_keys=(), change=None, exceptions="", frame=None, absent_keys=()
    ):
        with tempfile.TemporaryDirectory() as tmp:
            elements_path = os.path.join(tmp, "elements.toml")
            with open(elements_path, "w", encoding="utf-8") as f:
                f.write(SELF_TEST_ELEMENTS)
            exceptions_path = os.path.join(tmp, "exceptions.toml")
            with open(exceptions_path, "w", encoding="utf-8") as f:
                f.write("[parity]\n" + exceptions)
            elements = load_elements(elements_path, known)
            excepted = load_parity_exceptions(exceptions_path, elements)
            run = os.path.join(tmp, "run")
            os.mkdir(run)
            for kind in KINDS:
                rects = {"t.panel": dict(panel), "t.panel.label": dict(label)}
                fill = FILL
                if change is not None:
                    fill = change(kind, rects) or FILL
                stem = os.path.join(run, f"{kind}-test-preset-light")
                dump = {
                    "kind": kind,
                    "preset": "test-preset",
                    "variant": "light",
                    "scale": 1.0,
                    "elements": rects,
                }
                with open(stem + ".json", "w", encoding="utf-8") as f:
                    json.dump(dump, f)
                shown_panel = rects.get("t.panel", panel)
                shown_label = rects.get("t.panel.label", label)
                self_test_capture(stem + ".png", 1.0, shown_panel, shown_label, fill, frame)
                if frame is not None:
                    with open(stem + ".offset", "w", encoding="utf-8") as f:
                        f.write(f"+{frame[0]}+{frame[1]}\n")
            groups = load_run(run, KINDS, None)
            rows, _, _ = compare(groups, elements, excepted, KINDS)
        unexcepted = [key for _, key, _, reason in rows if reason is None]
        ok = (
            bool(unexcepted) == expect_unexcepted
            and all(k in unexcepted for k in expect_keys)
            and not any(k in unexcepted for k in absent_keys)
        )
        if not expect_unexcepted:
            ok = ok and not unexcepted
        print(f"  {'pass' if ok else 'FAIL'}: {name}")
        if not ok:
            failures.append(name)
            print_table(rows, KINDS)

    def wider(kind, rects):
        if kind == "egui":
            rects["t.panel"]["w"] += 1.0

    def recoloured(kind, rects):
        return (250, 250, 250) if kind == "iced" else None

    def dropped(kind, rects):
        if kind == "iced":
            del rects["t.panel.label"]

    def moved_together(kind, rects):
        if kind == "gpui":
            for rect in rects.values():
                rect["x"] += 5.0

    def moved_child(kind, rects):
        if kind == "gpui":
            rects["t.panel.label"]["y"] += 1.0

    def tip_in_one(kind, rects):
        if kind == "egui":
            rects["t.panel.tip"] = {"x": 40.0, "y": 95.0, "w": 80.0, "h": 24.0}

    def tip_in_all(kind, rects):
        rects["t.panel.tip"] = {"x": 40.0, "y": 95.0, "w": 80.0, "h": 24.0}

    print("check_showcase_parity self-test:")
    scenario("identical dumps and captures pass", False)
    scenario("identical, framed captures with .offset sidecars pass", False, frame=(65, 81))
    scenario("a 1 px size difference fails", True, ("t.panel.w",), wider)
    scenario("a colour difference fails", True, ("t.panel.fill",), recoloured)
    scenario(
        "an excepted difference passes",
        False,
        change=wider,
        exceptions='"t.panel.w" = "self-test reason"\n',
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
    parser.add_argument("dump_dir", nargs="?", help="directory of <kind>-<preset>-<variant>.json/.png")
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
    args = parser.parse_args()

    if args.self_test:
        return self_test()
    kinds = tuple(k for k in args.kinds.split(",") if k)
    if not kinds or any(k not in KINDS for k in kinds) or len(set(kinds)) != len(kinds):
        raise Failure(f"--kinds takes names from {', '.join(KINDS)}")
    elements = load_elements(args.elements, registry_paths(load_toml(REGISTRY)))
    exceptions = load_parity_exceptions(args.exceptions, elements)
    if args.check_list:
        print(f"{len(elements)} elements, {len(exceptions)} parity exceptions: valid")
        return 0
    if args.dump_dir is None:
        parser.error("a dump directory is required (or --check-list / --self-test)")
    if len(kinds) < 2:
        raise Failure("comparing needs at least two kinds")

    groups = load_run(args.dump_dir, kinds, args.content_offset)
    rows, used, unseen = compare(groups, elements, exceptions, kinds)
    if rows:
        print_table(rows, kinds)
    for group, idents in sorted(unseen.items()):
        print(f"\n{group}: {len(idents)} conditional element(s) no kind drew (their `when` did not hold):")
        print("\n".join(f"  {ident}  [{elements[ident]['when']}]" for ident in idents))
    unexcepted = sum(1 for row in rows if row[3] is None)
    stale = sorted(set(exceptions) - used)
    if stale:
        print("\nParity exceptions that matched no difference in this run:")
        print("\n".join("  " + key for key in stale))
    print(
        f"\n{len(groups)} preset/variant group(s), {len(rows)} difference(s), "
        f"{len(rows) - unexcepted} excepted, {unexcepted} not excepted."
    )
    return 1 if unexcepted else 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Failure as error:
        print(f"check_showcase_parity: {error}", file=sys.stderr)
        sys.exit(2)
