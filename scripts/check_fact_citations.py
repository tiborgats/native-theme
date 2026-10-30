#!/usr/bin/env python3
"""What the facts' citations say (specification S17; the 0.8.0 plan's *How values are checked*). Needs the source cache.

Reads the facts as scripts/check_facts.py does (that script checks their form; this one assumes it and skips what it
cannot read), the source cache (`NATIVE_THEME_SOURCES`, else scripts/citations.py's DEFAULT_CACHE, or `--cache DIR`;
one checkout per `docs/facts/sources.toml` id, filled by scripts/update_sources.sh) and `docs/evidence/`.

Run from the repository root:

    python3 scripts/check_fact_citations.py [--cache DIR]
    python3 scripts/check_fact_citations.py --self-test

It prints one line per finding, `<file>:<line>: <what>`, a `# sdk:` line per citation of an SDK header, and a summary
line with the counts (on stderr). Exit 0 when there is no finding, 1 when there are some, 2 when the source cache
is missing, an input cannot be read or the command line is wrong. The full run of scripts/check_release.sh needs the
cache; its `--commit` mode skips this check, by name, only when the cache folder is absent.

Checks, per `published` and `read` citation (of a fact, a palette fact or a state-model entry):

- the quote occurs, whitespace-normalised (every run of whitespace one space, both ends trimmed), in the cited lines
  `l1..l2` of `<cache>/<id>/<path>`, the checkout being at its entry's commit (`git rev-parse HEAD`); for a
  `<id> #<JSON pointer>` source, in the string at the pointer of the snapshot's file: a `snapshot` entry of
  sources.toml names it (`file = "<name>"`, the file's name in `<cache>/<id>/`), and an entry naming none is a finding;
- `absent`: no listed name occurs in the cited lines, a name matched at identifier boundaries (it is not part of a
  longer identifier: letters, digits and `_`, and in `css` `-` too), so `NSRefreshControl` does not hold
  `RefreshControl`; `absent = []` (S17 *Absence*: the platform has no name for it) leaves the quote check of the
  cited definition only;
- a citation of an `sdk` entry (`xcode@…`) is not checked here: it is listed (`# sdk:`), as `os-probe.yml`'s macOS job
  checks it at that Xcode; a `distribution` entry is never cited (S17).

Per fact, the quote check (S17 *The quote check*): for each generation the fact covers that its `published` or `read`
citations serve (served_generations: its `preset:` generation, or each generation of its directory its citations'
entries serve, a `preset:` in `not_conditions` taken out; whether or not the preset's file exists yet, which only
check_facts.py's generation rule asks), and for a palette fact once over all its citations, every literal of `value`
equals a literal of the union of those citations' quotes, as written or after the citation's `conversion`. A value's
literals are S2.2's (a TOML number; the colour, length, duration, angle and font-size literals of a string or an
expression; a table's leaves, a suffixed key giving its number its unit); references and names are the verifier's.
A text value (check_facts.text_leaves: where the key's type is text) is no literal source: it equals, whitespace-
normalised and case-sensitive, a whole string one of those quotes holds by the text notations (quote_texts, below).

Per `measured` citation: the evidence file is committed (`git ls-files`) and exists, the run folder's `run.toml` has an
entry for the image, and, for a measured derivation (a citation with `inputs` of an expression value, S17 *A measured
derivation*), the evidence path names a list of pass tables in a TOML file (pass_problems): each holds the `inputs` as
quoted keys, `result`, and `conditions` (a list of S7.1's condition strings naming the pass's scheme) unless the fact's
`when` fixes the scheme, nothing else, every value spelled as a fact's `value` for its key's type
(check_facts.value_problems; `conversion` does not apply to a result); for each input and each scheme the fact covers,
two passes with the same conditions and other inputs that differ in that input beyond the quote check's precision and
whose results differ by more than `tolerance` (a per-leaf table: in some leaf). Its value is not compared with the
evidence directly. Otherwise the value at the evidence path equals the fact's within `tolerance` (S17 *Fields*): a
scalar as one literal, a value with leaves leaf by leaf. The evidence path names a table shaped like the value, and the
shape is compared exactly first (shape_problems: a table's keys the same, a list's length the same, a leaf where the
value has a leaf), so a one-layer shadow or `[]` does not pass against two layers; then each numeric or colour leaf
within its tolerance (one number for all, or a table from each leaf's path, every such leaf needing one), a bool or a
name exactly. The evidence's literals are read as a quote's are (its file type's notation), then converted. A
state-model entry's measured citation needs its path to exist.

`run.toml` (RUN_FIELDS), in every run folder that holds evidence or is cited, legacy runs excepted:
`[images.<image>]` per image folder of the run, each with `preset`, `source` and `os`, and `toolkit` where a toolkit
drew or answered; no other key, no entry without its folder. `source` is a `sources.toml` entry and `preset` one of
its generations. The builds are recorded, not matched (D1).

Every evidence file (a file under `docs/evidence/<harness>/<YYYY-MM-DD>-run-<id>/<image>/`, on disk or committed) is
cited by a fact's or a state-model entry's measured citation, or by a vector table (vector_table_evidence(), the hook
scripts/check_function_vectors.py's format fills at M1a; until then it counts none), or is an image a cited image
analysis's output names. An analysis output (a TOML or JSON evidence file of a committed analysis script, S17
*`evidence`*) names its input image by a top-level `input_image = "<repository path>"` (a list of them where it reads
several), each under `docs/evidence/`; a name that is no file is a finding (input_images()). Not evidence: any
README.md, a run's `run.toml`, its `analysis/` folder (ANALYSIS: the committed analysis scripts, which run.toml maps to
no image), and the files of the runs LEGACY_EVIDENCE lists (the plan's *What carries over*: cited
until M4, which empties the list; scripts/check_ledger.py --final reads it by name). Any other file under
`docs/evidence/` outside a run's image folder is a finding.

The closed lists (S17 leaves them to this script; a member is added as a class 1 change):

- Notations, by the cited file's type (notation_of): `css` (.css, .scss, .sass, .less, .html, .svg), `xaml` (.xaml),
  `ini` (.colors, .ini, .conf, `kdeglobals`, a name ending in `rc` without a suffix), `cpp` (.c, .cc, .cpp, .cxx, .h,
  .hh, .hpp, .m, .mm), `qml` (.qml, .js), `kotlin` (.kt, .kts), `android` (an .xml file in a `res/` folder: Android's
  resources), `data` (.toml, .json, .csv: an evidence file's), `text` (any other file, a JSON page's string). Each
  reads, in this order, and no span is read twice:
  - hex colours (every notation masks a `#<hex digits>` run): `#rgb`, `#rrggbb` everywhere; `#rgba`, `#rrggbbaa`
    (RGBA) in `css` and `data`; `#argb`, `#aarrggbb` (ARGB) in `xaml` and `android`; other lengths read nothing;
  - `data`: the model's own literal forms `srgb(r, g, b[, a])` and `display_p3(r, g, b[, a])` (fractions, S2.1), which
    a harness may write;
  - `css` and `data`: `rgb(r, g, b)`, `rgba(r, g, b, a)`, `rgb(r g b / a)`, the function names in any case
    (libadwaita writes `RGB(`; channels 0-255 or `%`, alpha 0-1 or `%`), SCSS's `rgba(<hex>, a)`; `css`: the names
    `black`, `white`, `transparent`;
  - `qml`: `Qt.rgba(r, g, b, a)` (fractions), and a quoted colour name of QML_NAMES in any case (`"transparent"`:
    the names the cited QML uses, valued as qtbase's `qcolor.cpp` table); `cpp`: `QColor(r, g, b[, a])` and the C++
    initialiser `{r, g, b[, a]}` (bytes); `ini`: a key's `r,g,b[,a]` (bytes);
  - `xaml`: a `TimeSpan` `[d.]h:m:s[.f]` (a duration), and a colour name of XAML_NAMES as an attribute's value or an
    element's content (`Color="Transparent"`: the names the cited theme resources use, valued as WinUI's
    `KnownColors.h`, so `Transparent` is white with alpha 0);
  - `kotlin` (Compose): `Color(r, g, b[, a])` and `Color(red = r, green = g, blue = b[, alpha = a])` (Int arguments,
    decimal or `0x`, are bytes; Float ones, with an `f`, fractions), an eight-digit `0xAARRGGBB` integer (ARGB, as in
    `Color(0xFF5C77FF)`; other `0x` integers read nothing), and `<n>.dp`, `<n>.sp` (px) and `<n>.em` (no literal);
    a number's `f`, `F` or `L` suffix is Kotlin's type, not a unit;
  - on a source that serves only Apple's presets (Run.apple_only: an `sdk` or `snapshot` entry whose generations are
    all `macos-*` and `ios-*`, a snapshot of an Apple documentation site serving none, or measured evidence of an
    image mapped to such a preset): `<n> pt`, `<n>pt`, `<w>x<h> pt` and `<w> × <h> pt` are logical points, px (two
    numbers for a size);
  - the number form, everywhere: `[-]digits[.digits]` or `.digits` with an optional exponent (`1e-3` is one number),
    bounded by non-identifier characters (`Qt6` holds no number), with an optional unit attached; a unit outside
    UNITS makes it no literal.
- Text notations (quote_texts; S17 *The quote check*): they only add candidate strings for a text value and take no
  span, so the number form still reads the `11` of `'Adwaita Sans 11'`. A quoted string (`"…"`, `'…'`); a list's entry
  (each `;`-separated piece of the quote after its first `:` or `=`, the whole piece without one, split at commas; an
  entry trimmed of whitespace, enclosing brackets and quotes: CSS's `font-family` lists, an INI key's `A;B;`, an
  unquoted INI value `Theme=breeze`, Qt's `QFont::toString`, whose first field is the family); a font description's
  families (each quoted string or entry read as Pango's `pango_font_description_from_string` reads it, gsettings' font
  keys: the families before an optional size, `<n>` or `<n>px`, and the style words PANGO_WORDS, pango@1.58.2
  pango/fonts.c:1294-1361; `'Adwaita Sans 11'` gives `Adwaita Sans`, `Cantarell Bold` `Cantarell`). A text value matches
  a whole candidate only: `Noto Sans` is not in `Noto Sans Mono`, nor `go-next` in `go-next-symbolic`.
- Units (UNITS) by dimension: px, dp, sp (Android's text size before its scaling) are px; pt; ms and s (×1000) are
  ms; deg; rem and em have none (a literal in them is no literal). A literal without a unit takes the value
  literal's dimension; with one it must be of it, px and pt being distinct. Apple's `pt` is a logical point, px (the
  previous item); everywhere else `pt` is a typographic point.
- Conversions (CONVERSIONS), applied in order to a quote's or an evidence value's unitless numbers:
  `percent_to_fraction` (÷ 100), `byte_to_fraction` (÷ 255), `complement` (1 − x), `seconds_to_ms` (× 1000, the
  result a duration).
- Equality: numbers to a relative 1e-4, colours as 8-bit sRGB channels with alpha to 1e-3 (S17); `srgb(…)` and
  `display_p3(…)` literals in their own space. A `display_p3(…)` value is the colour as Apple publishes it (S2.1: the
  preset cites the published value exactly and converts when the file is read), so it is compared in Display P3, not
  after a conversion; no pinned source writes a Display P3 notation today (WebKit's CSS and the probe evidence have
  none, and no Apple page is snapshotted yet), so the quote notations read none and such a value is held by its
  measured citations, whose `data` notation reads `display_p3(…)`; the first snapshot stating P3 values adds its
  notation here, as a class 1 change.
- Evidence path forms (S17 *`evidence`*), by the file's type: TOML a dotted key (a digit segment indexes an array),
  JSON a JSON pointer, INI `<group>/<key>` (a nested group as its bracketed names joined by `][`:
  `Colors:Header][Inactive/BackgroundNormal`), CSV `<first column>/<header>`.
- `run.toml` (RUN_FIELDS): `[images.<image>]` tables only, each `preset`, `source`, `os` required, `toolkit` optional.
"""
import csv
import io
import itertools
import json
import re
import subprocess
import sys
import tempfile
import tomllib
from dataclasses import dataclass
from pathlib import Path

import check_facts
import citations

LEGACY_EVIDENCE = [
    "docs/evidence/os-probe/2026-10-11-run-38129819806",
    "docs/evidence/os-probe/2026-10-14-run-38478334574",
    "docs/evidence/os-probe/2026-10-26-run-40320895749",
    "docs/evidence/os-probe/2026-10-26-run-40324458365",
    "docs/evidence/os-probe/2026-10-26-run-40348854007",
]

CONVERSIONS = {
    "percent_to_fraction": lambda x: x / 100,
    "byte_to_fraction": lambda x: x / 255,
    "complement": lambda x: 1 - x,
    "seconds_to_ms": lambda x: x * 1000,
}
# unit -> (dimension, factor to the dimension's canonical unit); None: no dimension (no literal)
UNITS = {"px": ("length", 1), "dp": ("length", 1), "sp": ("length", 1), "pt": ("pt", 1), "ms": ("duration", 1),
         "s": ("duration", 1000), "deg": ("angle", 1), "rem": None, "em": None}
VALUE_UNITS = {"px": "length", "pt": "pt", "ms": "duration", "deg": "angle"}
SUFFIX_DIMS = {"_px": "length", "_ms": "duration", "_deg": "angle", "_pt": "pt"}
RUN_FIELDS = ({"preset", "source", "os"}, {"toolkit"})
NUMBER_EQ = 1e-4
COLOUR_EQ = 1e-3
CSS_NAMES = {"black": (0, 0, 0, 255), "white": (255, 255, 255, 255), "transparent": (0, 0, 0, 0)}
# QML's colour names the cited QML uses, as Qt reads them (case-insensitive; qtbase@6.12.0
# src/gui/painting/qcolor.cpp:141-280)
QML_NAMES = {"transparent": (0, 0, 0, 0), "black": (0, 0, 0, 255), "white": (255, 255, 255, 255),
             "red": (255, 0, 0, 255), "green": (0, 128, 0, 255), "blue": (0, 0, 255, 255),
             "yellow": (255, 255, 0, 255), "orange": (255, 165, 0, 255), "brown": (165, 42, 42, 255),
             "grey": (128, 128, 128, 255), "lightgrey": (211, 211, 211, 255), "lightgreen": (144, 238, 144, 255),
             "darkblue": (0, 0, 139, 255), "darkred": (139, 0, 0, 255)}
# XAML's colour names the cited theme resources use (microsoft-ui-xaml@2.5.1
# src/dxaml/xcp/components/colors/inc/KnownColors.h:15, :76, :141, :145)
XAML_NAMES = {"Transparent": (255, 255, 255, 0), "Black": (0, 0, 0, 255), "White": (255, 255, 255, 255),
              "LightGray": (211, 211, 211, 255)}
APPLE_PRESETS = ("macos-", "ios-")
APPLE_SITES = ("developer.apple.com", "support.apple.com")
RUN_RX = re.compile(r"^\d{4}-\d\d-\d\d-run-.+$")
ANALYSIS = "analysis"   # a run's committed analysis scripts: not evidence, as its README.md and run.toml

NOTATIONS = {
    "css": (".css", ".scss", ".sass", ".less", ".html", ".svg"),
    "xaml": (".xaml",),
    "ini": (".colors", ".ini", ".conf"),
    "cpp": (".c", ".cc", ".cpp", ".cxx", ".h", ".hh", ".hpp", ".m", ".mm"),
    "qml": (".qml", ".js"),
    "kotlin": (".kt", ".kts"),
    "data": (".toml", ".json", ".csv"),
}


@dataclass(frozen=True)
class Lit:
    """A literal: a number (`dim`: None for unitless, "length", "pt", "duration", "angle", "number") or a colour
    (`dim`: "srgb" or "p3", `value`: four channels on the 0-255 scale)."""
    kind: str
    dim: str | None
    value: object

    def __str__(self):
        if self.kind == "colour":
            return f"{self.dim}({', '.join(f'{c:g}' for c in self.value)})"
        unit = {"length": "px", "pt": "pt", "duration": "ms", "angle": "deg"}.get(self.dim, "")
        return f"{self.value:g}{unit}"


def notation_of(path: str) -> str:
    name = path.rsplit("/", 1)[-1].lower()
    suffix = "." + name.rsplit(".", 1)[1] if "." in name else ""
    if suffix == ".xml" and ("/" + path).find("/res/") >= 0:
        return "android"
    for n, suffixes in NOTATIONS.items():
        if suffix in suffixes:
            return n
    if name == "kdeglobals" or (not suffix and name.endswith("rc")):
        return "ini"
    return "text"


# ------------------------------------------------------------------------------------------------- literals

def _hex(digits, order):
    if len(digits) in (3, 4):
        digits = "".join(c * 2 for c in digits)
    vals = [int(digits[i:i + 2], 16) for i in range(0, len(digits), 2)]
    if len(vals) == 3:
        return tuple(vals) + (255,)
    if order == "argb":
        return tuple(vals[1:]) + (vals[0],)
    return tuple(vals)


def _channel(s, alpha=False):
    s = s.strip()
    if s.endswith("%"):
        return float(s[:-1]) * 255 / 100
    return float(s) * 255 if alpha else float(s)


NUM = r"-?(?:\d+(?:\.\d+)?|\.\d+)"
EXP = r"(?:[eE][-+]?\d+)?"
KT_NUM = r"(?:0x[0-9A-Fa-f]+|\d+(?:\.\d+)?[fF]?|\.\d+[fF]?)"
HEX_RX = re.compile(r"(?<![\w&])#([0-9a-fA-F]+)\b")
CSS_FUNC_RX = re.compile(r"\brgba?\(\s*([^()]*)\)", re.I)
MODEL_COLOUR_RX = re.compile(rf"\b(srgb|display_p3)\(\s*({NUM})\s*,\s*({NUM})\s*,\s*({NUM})\s*(?:,\s*({NUM})\s*)?\)")
QT_RGBA_RX = re.compile(rf"\bQt\.rgba\(\s*({NUM})\s*,\s*({NUM})\s*,\s*({NUM})\s*,\s*({NUM})\s*\)")
QML_NAME_RX = re.compile(r"""(["'])(""" + "|".join(QML_NAMES) + r")\1", re.I)
XAML_NAME_RX = re.compile(r"""(?<=["'>])(""" + "|".join(XAML_NAMES) + r""")(?=["'<])""")
QCOLOR_RX = re.compile(r"\bQColor\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*(?:,\s*(\d{1,3})\s*)?\)")
BRACE_RX = re.compile(r"\{\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*(?:,\s*(\d{1,3})\s*)?\}")
BYTES_RX = re.compile(r"(?<![\w.,])(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})(?:\s*,\s*(\d{1,3}))?(?![\w.,])")
TIMESPAN_RX = re.compile(r"(?<![\w:.])(?:(\d+)\.)?(\d+):(\d+):(\d+(?:\.\d+)?)(?![\w:])")
KT_COLOR_RX = re.compile(rf"\bColor\(\s*(?:red\s*=\s*)?({KT_NUM})\s*,\s*(?:green\s*=\s*)?({KT_NUM})\s*,\s*"
                         rf"(?:blue\s*=\s*)?({KT_NUM})\s*(?:,\s*(?:alpha\s*=\s*)?({KT_NUM})\s*)?\)")
KT_HEX_RX = re.compile(r"\b0x([0-9A-Fa-f]+)(?:[uU]?L)?\b")
KT_UNIT_RX = re.compile(r"(?<![\w.])(\d+(?:\.\d+)?)\.(dp|sp|em)\b")
APPLE_PT_RX = re.compile(r"(?<![\w.])(\d+(?:\.\d+)?)(?:\s*[x×]\s*(\d+(?:\.\d+)?))?\s*pt\b")
NUMBER_RX = re.compile(rf"(?<![\w.])(-(?=[\d.]))?(\d+(?:\.\d+)?{EXP}|\.\d+{EXP})([A-Za-z]*)(?!\w)")
NAMES_RX = re.compile(r"(?<![\w-])(black|white|transparent)(?![\w-])")


def _kt_channels(groups):
    """Compose's Color(r, g, b[, a]): Int arguments are bytes, Float ones (an `f` suffix) fractions."""
    vals = [g for g in groups if g is not None]
    if any(v[-1] in "fF" or "." in v for v in vals if not v.lower().startswith("0x")):
        ch = [float(v.rstrip("fF")) * 255 for v in vals]
    else:
        ch = [float(int(v, 16) if v.lower().startswith("0x") else int(v)) for v in vals]
    return tuple(ch[:3]) + ((ch[3],) if len(ch) == 4 else (255,))


def quote_literals(text: str, notation: str, apple: bool = False) -> list:
    """Every literal of `text` read by `notation` (the closed lists of the docstring), in text order; `apple`: the
    text is a source that serves only Apple's presets, whose points are logical points (px)."""
    found = []   # (start, end, Lit or None): None masks a span that reads nothing
    taken = []

    def free(a, b):
        return all(b <= x or a >= y for x, y in taken)

    def add(a, b, lit):
        if free(a, b):
            taken.append((a, b))
            found.append((a, lit))

    for m in HEX_RX.finditer(text):
        d = m.group(1)
        lit = None
        if len(d) in (3, 6):
            lit = Lit("colour", "srgb", _hex(d, "rgba"))
        elif len(d) in (4, 8) and notation in ("css", "data"):
            lit = Lit("colour", "srgb", _hex(d, "rgba"))
        elif len(d) in (4, 8) and notation in ("xaml", "android"):
            lit = Lit("colour", "srgb", _hex(d, "argb"))
        add(m.start(), m.end(), lit)
    if notation == "data":
        for m in MODEL_COLOUR_RX.finditer(text):
            vals = [float(g) for g in m.groups()[1:] if g is not None]
            vals = vals + [1.0] if len(vals) == 3 else vals
            add(m.start(), m.end(), Lit("colour", "srgb" if m.group(1) == "srgb" else "p3",
                                        tuple(v * 255 for v in vals)))
    if notation in ("css", "data"):
        for m in CSS_FUNC_RX.finditer(text):
            args = m.group(1).strip()
            hm = re.fullmatch(rf"#([0-9a-fA-F]{{3}}|[0-9a-fA-F]{{6}})\s*,\s*({NUM}%?)", args)
            if hm:
                rgb = _hex(hm.group(1), "rgba")[:3]
                taken[:] = [(x, y) for x, y in taken if not (m.start() <= x and y <= m.end())]
                found[:] = [(p, lit) for p, lit in found if not (m.start() <= p < m.end())]
                add(m.start(), m.end(), Lit("colour", "srgb", rgb + (_channel(hm.group(2), True),)))
                continue
            parts = [p for p in re.split(r"\s*[,/]\s*|\s+", args) if p]
            if len(parts) in (3, 4) and all(re.fullmatch(rf"{NUM}%?", p) for p in parts):
                ch = [_channel(p) for p in parts[:3]]
                a = _channel(parts[3], True) if len(parts) == 4 else 255
                add(m.start(), m.end(), Lit("colour", "srgb", tuple(ch) + (a,)))
        if notation == "css":
            for m in NAMES_RX.finditer(text):
                add(m.start(), m.end(), Lit("colour", "srgb", CSS_NAMES[m.group(1)]))
    if notation == "qml":
        for m in QT_RGBA_RX.finditer(text):
            add(m.start(), m.end(), Lit("colour", "srgb", tuple(float(g) * 255 for g in m.groups())))
        for m in QML_NAME_RX.finditer(text):
            add(m.start(), m.end(), Lit("colour", "srgb", QML_NAMES[m.group(2).lower()]))
    if notation == "cpp":
        for rx in (QCOLOR_RX, BRACE_RX):
            for m in rx.finditer(text):
                vals = [float(g) for g in m.groups() if g is not None]
                if all(v <= 255 for v in vals):
                    add(m.start(), m.end(), Lit("colour", "srgb", tuple(vals[:3]) + ((vals[3],) if len(vals) == 4
                                                                                     else (255,))))
    if notation == "ini":
        for m in BYTES_RX.finditer(text):
            vals = [float(g) for g in m.groups() if g is not None]
            if all(v <= 255 for v in vals):
                add(m.start(), m.end(), Lit("colour", "srgb", tuple(vals[:3]) + ((vals[3],) if len(vals) == 4
                                                                                 else (255,))))
    if notation == "xaml":
        for m in TIMESPAN_RX.finditer(text):
            d, h, mi, s = m.groups()
            ms = ((int(d or 0) * 24 + int(h)) * 60 + int(mi)) * 60000 + float(s) * 1000
            add(m.start(), m.end(), Lit("num", "duration", ms))
        for m in XAML_NAME_RX.finditer(text):
            add(m.start(), m.end(), Lit("colour", "srgb", XAML_NAMES[m.group(1)]))
    if notation == "kotlin":
        for m in KT_COLOR_RX.finditer(text):
            ch = _kt_channels(m.groups())
            if all(0 <= c <= 255 for c in ch):
                add(m.start(), m.end(), Lit("colour", "srgb", ch))
        for m in KT_HEX_RX.finditer(text):
            add(m.start(), m.end(), Lit("colour", "srgb", _hex(m.group(1), "argb")) if len(m.group(1)) == 8 else None)
        for m in KT_UNIT_RX.finditer(text):
            unit = UNITS[m.group(2)]
            add(m.start(), m.end(), Lit("num", unit[0], float(m.group(1)) * unit[1]) if unit else None)
    if apple:
        for m in APPLE_PT_RX.finditer(text):
            add(m.start(), m.end(), Lit("num", "length", float(m.group(1))))
            if m.group(2):
                found.append((m.start(2), Lit("num", "length", float(m.group(2)))))
    for m in NUMBER_RX.finditer(text):
        if not free(m.start(), m.end()):
            continue
        x = float((m.group(1) or "") + m.group(2))
        unit = m.group(3)
        if not unit or (notation == "kotlin" and unit in ("f", "F", "L")):
            found.append((m.start(), Lit("num", None, x)))
        elif unit in UNITS and UNITS[unit] is not None:
            dim, factor = UNITS[unit]
            found.append((m.start(), Lit("num", dim, x * factor)))
    return [lit for _, lit in sorted(found, key=lambda t: t[0]) if lit is not None]


VALUE_TOKEN_RX = re.compile(rf"(?P<colour>\b(?:srgb|display_p3)\(\s*{NUM}\s*,\s*{NUM}\s*,\s*{NUM}\s*"
                            rf"(?:,\s*{NUM}\s*)?\))"
                            r"|(?P<hex>#[0-9a-fA-F]+)"
                            r"|(?P<path>@?[a-z_][a-z0-9_]*(?:\.@?[a-z0-9_][a-z0-9_]*)*)"
                            rf"|(?P<num>(?<![\w.]){NUM}(?:px|ms|deg|pt)?(?![\w.]))")


def string_literals(s: str) -> list:
    out = []
    for m in VALUE_TOKEN_RX.finditer(s):
        if m.lastgroup == "colour":
            name, args = m.group("colour").split("(", 1)
            vals = [float(x) for x in args.rstrip(")").split(",")]
            vals = vals + [1.0] if len(vals) == 3 else vals
            out.append(Lit("colour", "srgb" if name == "srgb" else "p3", tuple(v * 255 for v in vals)))
        elif m.lastgroup == "hex":
            d = m.group("hex")[1:]
            if len(d) in (6, 8):
                out.append(Lit("colour", "srgb", _hex(d, "rgba")))
        elif m.lastgroup == "num":
            t = m.group("num")
            u = re.search(r"(px|ms|deg|pt)$", t)
            out.append(Lit("num", VALUE_UNITS[u.group(1)] if u else "number", float(t[:u.start()] if u else t)))
    return out


def value_literals(value, dim=None) -> list:
    """S2.2's literals of a fact's value (references and names left out)."""
    if isinstance(value, bool):
        return []
    if isinstance(value, (int, float)):
        return [Lit("num", dim or "number", float(value))]
    if isinstance(value, str):
        return string_literals(value)
    if isinstance(value, list):
        return [lit for v in value for lit in value_literals(v)]
    if isinstance(value, dict):
        out = []
        for k, v in value.items():
            sub = next((d for suf, d in SUFFIX_DIMS.items() if k.endswith(suf)), None)
            out += value_literals(v, sub)
        return out
    return []


def convert(lits, names) -> list:
    """The literals as written and, for each unitless number, after the conversions in order."""
    out = list(lits)
    if not names:
        return out
    for lit in lits:
        if lit.kind != "num" or lit.dim is not None:
            continue
        x, dim = lit.value, None
        for n in names:
            x = CONVERSIONS[n](x)
            if n == "seconds_to_ms":
                dim = "duration"
        out.append(Lit("num", dim, x))
    return out


def converted_only(lits, names):
    """The literals after the conversions (an evidence value: the converted value replaces the read one)."""
    if not names:
        return list(lits)
    return [x for x in convert(lits, names)[len(lits):]] + [x for x in lits if x.kind != "num" or x.dim is not None]


def equal(v: Lit, q: Lit) -> bool:
    if v.kind != q.kind:
        return False
    if v.kind == "colour":
        return v.dim == q.dim and all(abs(a - b) <= COLOUR_EQ for a, b in zip(v.value, q.value))
    if q.dim is not None and q.dim != v.dim:
        return False
    a, b = v.value, q.value
    return a == b or abs(a - b) <= NUMBER_EQ * max(abs(a), abs(b))


def within(v: Lit, e: Lit, tol: float) -> bool:
    if v.kind != e.kind:
        return False
    if v.kind == "colour":
        return v.dim == e.dim and all(abs(a - b) <= tol + 1e-9 for a, b in zip(v.value, e.value))
    if e.dim is not None and e.dim != v.dim:
        return False
    return abs(v.value - e.value) <= tol + 1e-9 * max(1.0, abs(v.value))


def conversions_of(cite: dict) -> list:
    """The citation's conversions that CONVERSIONS names (check_facts.py reports the others)."""
    conv = cite.get("conversion")
    return [n for n in conv if isinstance(n, str) and n in CONVERSIONS] if isinstance(conv, list) else []


def normalise(s: str) -> str:
    return " ".join(s.split())


# ------------------------------------------------------------------------------------------------- text

DQUOTED_RX = re.compile(r'"([^"]*)"')
SQUOTED_RX = re.compile(r"'([^']*)'")
# Pango's words of a font description's style options (pango@1.58.2 pango/fonts.c:1294-1361, :1452): matched as
# field_matches() does, case-insensitively and with the map's hyphens optional, or as `<field>=<word or integer>`
PANGO_WORDS = ("Normal", "Roman", "Oblique", "Italic", "Small-Caps", "All-Small-Caps", "Petite-Caps",
               "All-Petite-Caps", "Unicase", "Title-Caps", "Thin", "Ultra-Light", "Extra-Light", "Light", "Semi-Light",
               "Demi-Light", "Book", "Regular", "Medium", "Semi-Bold", "Demi-Bold", "Bold", "Ultra-Bold",
               "Extra-Bold", "Heavy", "Black", "Ultra-Heavy", "Extra-Heavy", "Ultra-Black", "Extra-Black",
               "Ultra-Condensed", "Extra-Condensed", "Condensed", "Semi-Condensed", "Semi-Expanded", "Expanded",
               "Extra-Expanded", "Ultra-Expanded", "Not-Rotated", "South", "Upside-Down", "North", "Rotated-Left",
               "East", "Rotated-Right", "West", "With-Color", "Without-Color")
PANGO_FIELDS = ("weight", "style", "width", "variant", "gravity", "color")
_PANGO_KEYS = {w.lower() for w in PANGO_WORDS} | {w.lower().replace("-", "") for w in PANGO_WORDS}


def pango_word(w: str) -> bool:
    m = re.fullmatch(rf"({'|'.join(PANGO_FIELDS)})=(\S+)", w)
    if m:
        return m.group(2).lower() in _PANGO_KEYS or m.group(2).isdigit()
    return w.lower() in _PANGO_KEYS


def font_description_families(s: str) -> list:
    """The family list of a string read as a font description, as Pango's `pango_font_description_from_string` reads
    it (pango@1.58.2 pango/fonts.c:1617-1720; gsettings' font keys, `'Adwaita Sans 11'`): trailing `#features` and
    `@variations` words, then a size (a number, `px` allowed) and the style words before it are dropped, and the rest
    is split at commas."""
    words = s.split()
    for _ in range(2):
        if words and words[-1][:1] in ("#", "@"):
            words.pop()
    if words and re.fullmatch(r"\d+(?:\.\d+)?(?:px)?", words[-1]):
        words.pop()
    while words and pango_word(words[-1]):
        words.pop()
    rest = " ".join(words).strip().rstrip(",").strip()
    return [f.strip() for f in rest.split(",") if f.strip()]


def quote_texts(text: str) -> set:
    """The whole strings a quote holds by the closed text notations (S17 *The quote check*), whitespace-normalised:
    every quoted string (`"…"`, `'…'`); every entry of a list, a list being each `;`-separated piece of the quote
    after its first `:` or `=` (the whole piece without one) split at commas, an entry trimmed of whitespace,
    enclosing brackets and quotes (CSS's `font-family: "Adwaita Sans", sans-serif;`, an INI key's `A;B;`, Qt's
    `QFont::toString`, whose first field is the family: `font=Noto Sans,10,-1,5,400,…`, an unquoted INI value,
    `Theme=breeze`); and the families of each of those read as a font description (font_description_families)."""
    t = normalise(text)
    cands = [m.group(1) for rx in (DQUOTED_RX, SQUOTED_RX) for m in rx.finditer(t)]
    for piece in t.split(";"):
        m = re.search(r"[:=]", piece)
        for e in (piece[m.end():] if m else piece).split(","):
            e = e.strip().strip("[](){}").strip()
            if len(e) >= 2 and e[0] == e[-1] and e[0] in "\"'":
                e = e[1:-1]
            cands.append(e)
    out = set()
    for c in cands:
        c = normalise(c)
        if c:
            out.add(c)
            out.update(normalise(f) for f in font_description_families(c))
    return out


def without_paths(value, paths, prefix=""):
    """The value with the leaves at `paths` (S11's spelling) left out (None), so its literals are not read."""
    if prefix in paths:
        return None
    sub = (lambda k: f"{prefix}.{k}" if prefix else str(k))
    if isinstance(value, dict):
        return {k: without_paths(v, paths, sub(k)) for k, v in value.items()}
    if isinstance(value, list):
        return [without_paths(v, paths, sub(i)) for i, v in enumerate(value)]
    return value


# ------------------------------------------------------------------------------------------------- evidence

class EvidenceError(Exception):
    pass


def json_pointer(doc, pointer: str):
    if pointer == "":
        return doc
    cur = doc
    for raw in pointer.split("/")[1:]:
        tok = raw.replace("~1", "/").replace("~0", "~")
        if isinstance(cur, list):
            if not tok.isdigit() or int(tok) >= len(cur):
                raise EvidenceError(f"no element {tok!r}")
            cur = cur[int(tok)]
        elif isinstance(cur, dict):
            if tok not in cur:
                raise EvidenceError(f"no key {tok!r}")
            cur = cur[tok]
        else:
            raise EvidenceError(f"{tok!r} below a scalar")
    return cur


def evidence_value(path: Path, form: str):
    """The value at `form` in the evidence file (by its type), or raise EvidenceError."""
    name = path.name
    try:
        text = path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError) as err:
        raise EvidenceError(str(err)) from err
    kind = notation_of(name)
    if name.endswith(".toml"):
        try:
            cur = tomllib.loads(text)
        except tomllib.TOMLDecodeError as err:
            raise EvidenceError(f"not valid TOML: {err}") from err
        for seg in form.split("."):
            if isinstance(cur, list) and seg.isdigit() and int(seg) < len(cur):
                cur = cur[int(seg)]
            elif isinstance(cur, dict) and seg in cur:
                cur = cur[seg]
            else:
                raise EvidenceError(f"no {seg!r} at {form!r}")
        return cur
    if name.endswith(".json"):
        try:
            return json_pointer(json.loads(text), form)
        except json.JSONDecodeError as err:
            raise EvidenceError(f"not valid JSON: {err}") from err
    if name.endswith(".csv"):
        first, sep, header = form.rpartition("/")
        rows = list(csv.reader(io.StringIO(text)))
        if not sep or not rows or header not in rows[0]:
            raise EvidenceError(f"no column {header!r}")
        col = rows[0].index(header)
        for row in rows[1:]:
            if row and row[0] == first:
                if col >= len(row):
                    raise EvidenceError(f"row {first!r} has no {header!r} cell")
                return row[col]
        raise EvidenceError(f"no row {first!r}")
    if kind == "ini":
        group, sep, key = form.rpartition("/")
        if not sep:
            raise EvidenceError(f"{form!r} is not `<group>/<key>`")
        cur = None
        for line in text.splitlines():
            s = line.strip()
            if s.startswith("[") and s.endswith("]"):
                cur = s[1:-1]
            elif cur == group and "=" in s and not s.startswith(("#", ";")):
                k, v = s.split("=", 1)
                if k.strip() == key:
                    return v.strip()
        raise EvidenceError(f"no key {key!r} in group [{group}]")
    raise EvidenceError(f"{name}: not an evidence file type (TOML, JSON, INI, CSV)")


def leaves(value, prefix="", dim=None):
    """(path as S11 spells it, leaf, dimension) of every leaf of a value."""
    if isinstance(value, dict):
        for k, v in value.items():
            sub = next((d for suf, d in SUFFIX_DIMS.items() if k.endswith(suf)), None)
            yield from leaves(v, f"{prefix}.{k}" if prefix else k, sub)
    elif isinstance(value, list):
        for i, v in enumerate(value):
            yield from leaves(v, f"{prefix}.{i}" if prefix else str(i), None)
    else:
        yield prefix, value, dim


def at(value, path):
    cur = value
    for seg in path.split(".") if path else []:
        if isinstance(cur, list) and seg.isdigit() and int(seg) < len(cur):
            cur = cur[int(seg)]
        elif isinstance(cur, dict) and seg in cur:
            cur = cur[seg]
        else:
            raise KeyError(path)
    return cur


def fact_leaf_literal(leaf, dim):
    """The leaf's literal, or None when it compares exactly (a bool, a name, a keyword)."""
    if isinstance(leaf, bool):
        return None
    if isinstance(leaf, (int, float)):
        return Lit("num", dim or "number", float(leaf))
    if isinstance(leaf, str):
        lits = string_literals(leaf)
        if len(lits) == 1 and (check_facts.HEX_LITERAL.match(leaf) or check_facts.FUNC_COLOUR.match(leaf)
                               or check_facts.UNIT_LITERAL.match(leaf)):
            return lits[0]
    return None


def shape_problems(fact, ev, path=""):
    """Where the evidence's shape is not the value's: a table's keys, a list's length (S17 *Fields*: the evidence path
    names a table shaped like the value)."""
    where = repr(path) if path else "the value"
    sub = (lambda k: f"{path}.{k}" if path else str(k))
    if isinstance(fact, dict):
        if not isinstance(ev, dict):
            return [f"{where}: the evidence holds {ev!r}, the fact a table"]
        out = [f"the evidence has no leaf {sub(k)!r}" for k in fact if k not in ev]
        out += [f"the evidence has {sub(k)!r}, the fact does not" for k in ev if k not in fact]
        for k in fact:
            if k in ev:
                out += shape_problems(fact[k], ev[k], sub(k))
        return out
    if isinstance(fact, list):
        if not isinstance(ev, list):
            return [f"{where}: the evidence holds {ev!r}, the fact a list"]
        if len(ev) != len(fact):
            return [f"{where}: the evidence has {len(ev)} items, the fact {len(fact)}"]
        return [p for i, (f, e) in enumerate(zip(fact, ev)) for p in shape_problems(f, e, sub(i))]
    if isinstance(ev, (dict, list)):
        return [f"{where}: the evidence holds a {'table' if isinstance(ev, dict) else 'list'}, the fact a leaf"]
    return []


def compare(fact_value, ev_value, tolerance, conversion, notation, apple=False) -> list:
    """Problems of a measured value against the evidence (S17 *Fields*): its shape exactly, then leaf by leaf."""
    out = []
    items = list(leaves(fact_value))
    scalar = len(items) == 1 and items[0][0] == ""
    numeric = {p for p, leaf, d in items if fact_leaf_literal(leaf, d) is not None}
    if isinstance(tolerance, dict):
        if scalar:
            return ["a value without leaves takes one tolerance number"]
        out += [f"tolerance {k!r} is not a numeric or colour leaf of the value" for k in tolerance if k not in numeric]
        out += [f"no tolerance for the leaf {p!r}" for p in sorted(numeric - tolerance.keys())]
    shape = shape_problems(fact_value, ev_value)
    if shape:
        return out + shape
    for p, leaf, d in items:
        ev = at(ev_value, p)
        where = f"leaf {p!r}: " if p else ""
        flit = fact_leaf_literal(leaf, d)
        if flit is None:
            if ev != leaf:
                out.append(f"{where}the evidence holds {ev!r}, the fact {leaf!r}")
            continue
        if isinstance(ev, bool) or not isinstance(ev, (int, float, str)):
            out.append(f"{where}the evidence holds {ev!r}, not a number or a colour")
            continue
        elits = [Lit("num", None, float(ev))] if isinstance(ev, (int, float)) else quote_literals(ev, notation, apple)
        elits = converted_only(elits, conversion)
        if len(elits) != 1:
            out.append(f"{where}the evidence value {ev!r} holds {len(elits)} literals, not one")
            continue
        tol = tolerance.get(p) if isinstance(tolerance, dict) else tolerance
        if not isinstance(tol, (int, float)) or isinstance(tol, bool):
            continue
        if not within(flit, elits[0], tol):
            out.append(f"{where}the evidence holds {elits[0]}, the fact {flit} (tolerance {tol:g})")
    return out


def served_generations(item) -> set:
    """The generations a fact covers for the quote check, independent of the presets that exist (S17
    *Generations*): its `preset:` generation, else its directory's, less a `preset:` in `not_conditions`."""
    when = item.data.get("when") if isinstance(item.data.get("when"), dict) else {}
    conds = when.get("conditions") if check_facts.str_list(when.get("conditions")) else []
    nots = when.get("not_conditions") if check_facts.str_list(when.get("not_conditions")) else []
    pres = [c[7:] for c in conds if c.startswith("preset:")]
    out = set(pres[:1]) if pres else set(check_facts.PLATFORMS.get(item.platform, ()))
    return out - {c[7:] for c in nots if c.startswith("preset:")}


def same_value(x, y) -> bool:
    """Two values spelled as a fact's are the same: the same leaves, literals equal at the quote check's precision,
    other leaves exactly."""
    lx, ly = list(leaves(x)), list(leaves(y))
    if [p for p, _, _ in lx] != [p for p, _, _ in ly]:
        return False
    for (_, a, da), (_, b, db) in zip(lx, ly):
        fa, fb = fact_leaf_literal(a, da), fact_leaf_literal(b, db)
        if fa is not None and fb is not None:
            if not equal(fa, fb):
                return False
        elif a != b:
            return False
    return True


def results_differ(x, y, tolerance) -> bool:
    """The results differ by more than `tolerance` (one number, or per leaf: in some leaf, by more than its)."""
    lx, ly = list(leaves(x)), list(leaves(y))
    if [p for p, _, _ in lx] != [p for p, _, _ in ly]:
        return True
    for (p, a, da), (_, b, db) in zip(lx, ly):
        fa, fb = fact_leaf_literal(a, da), fact_leaf_literal(b, db)
        if fa is not None and fb is not None:
            t = tolerance.get(p, 0) if isinstance(tolerance, dict) else tolerance
            if isinstance(t, bool) or not isinstance(t, (int, float)) or not within(fa, fb, t):
                return True
        elif a != b:
            return True
    return False


def pass_pair(a, b, r, inputs, tolerance) -> bool:
    """Passes a and b differ in the input r alone (the other inputs and the conditions the same) and their results by
    more than the tolerance."""
    return (sorted(a.get("conditions", [])) == sorted(b.get("conditions", []))
            and all(same_value(a[o], b[o]) for o in inputs if o != r)
            and not same_value(a[r], b[r]) and results_differ(a["result"], b["result"], tolerance))


def vector_table_evidence(root: Path) -> set:
    """The evidence files the vector tables under native-theme/tests/vectors/ cite (S2.2 *Checked against their
    references*). The tables' format is scripts/check_function_vectors.py's (M1a), and its measured rows' evidence
    counts as cited here then; until the format exists no table cites any."""
    return set()


# ------------------------------------------------------------------------------------------------- checks

class Run:
    def __init__(self, ctx, cache: Path, committed=None, check_heads=True):
        self.ctx = ctx
        self.cache = Path(cache)
        self.check_heads = check_heads
        self.committed = committed if committed is not None else self.git_files()
        self._heads = {}
        self.findings = []
        self.sdk = []
        self.counts = {"quotes": 0, "generations": 0, "measured": 0, "evidence": 0, "legacy": 0}

    def git_files(self):
        try:
            out = subprocess.run(["git", "-C", str(self.ctx.root), "ls-files", "-z", "docs/evidence"],
                                 capture_output=True, text=True, check=True).stdout
        except (OSError, subprocess.CalledProcessError) as err:
            raise check_facts.InputError(f"git ls-files: {err}") from err
        return {p for p in out.split("\0") if p}

    def add(self, file, line, what):
        self.findings.append((file, line, what))

    def head_problem(self, sid, entry):
        if not self.check_heads:
            return None
        if sid not in self._heads:
            d = self.cache / sid
            if not (d / ".git").exists():
                self._heads[sid] = f"{sid} has no checkout in the source cache ({d}; scripts/update_sources.sh)"
            else:
                try:
                    head = subprocess.run(["git", "-C", str(d), "rev-parse", "HEAD"], capture_output=True, text=True,
                                          check=True).stdout.strip()
                except (OSError, subprocess.CalledProcessError):
                    head = "?"
                want = entry.get("commit")
                self._heads[sid] = None if head == want else (f"{sid}'s checkout is at {head}, not at its pin {want} "
                                                              f"(scripts/update_sources.sh)")
        return self._heads[sid]

    def apple_only(self, cite: dict) -> bool:
        """The citation serves only Apple's presets: an `sdk` or `snapshot` entry whose generations are all macOS
        and iOS presets (a snapshot serving none: one of an Apple documentation site), or measured evidence of an
        image mapped to one of them. Its points are logical points (px)."""
        if cite.get("kind") == "measured":
            gens = self.ctx.cite_generations(cite)
            return bool(gens) and all(g.startswith(APPLE_PRESETS) for g in gens)
        src = check_facts.parse_source(cite.get("source"))
        entry = self.ctx.sources.get(src[0]) if src else None
        if not isinstance(entry, dict) or entry.get("kind") not in ("sdk", "snapshot"):
            return False
        gens = entry.get("generations")
        if isinstance(gens, list) and gens:
            return all(isinstance(g, str) and g.startswith(APPLE_PRESETS) for g in gens)
        return entry.get("kind") == "snapshot" and str(entry.get("site", "")).startswith(APPLE_SITES)

    def cited_text(self, item, cite):
        """The text the citation's source names, or None (a finding added, or an SDK citation listed)."""
        c = cite.data
        src = check_facts.parse_source(c.get("source"))
        if not src or src[0] not in self.ctx.sources:
            return None
        sid, path, l1, l2, pointer = src
        entry = self.ctx.sources[sid]
        kind = entry.get("kind")
        if kind == "sdk":
            self.sdk.append((item.file, cite.line, c["source"]))
            return None
        if kind == "distribution":
            self.add(item.file, cite.ln("source"), f"{sid} is a distribution snapshot, which no citation names")
            return None
        if kind == "git":
            problem = self.head_problem(sid, entry)
            if problem:
                self.add(item.file, cite.ln("source"), problem)
                return None
        base = self.cache / sid
        if pointer is not None:
            name = entry.get("file")
            if not isinstance(name, str) or not name or "/" in name or name.startswith("."):
                self.add(item.file, cite.ln("source"), f"{sid}: its sources.toml entry names no `file` (the "
                                                       f"snapshot's file in the cache folder)")
                return None
            if not (base / name).is_file():
                self.add(item.file, cite.ln("source"), f"{sid}: no file {name} in the source cache")
                return None
            try:
                v = json_pointer(json.loads((base / name).read_text(encoding="utf-8")), pointer)
            except (OSError, UnicodeDecodeError, json.JSONDecodeError, EvidenceError) as err:
                self.add(item.file, cite.ln("source"), f"{c['source']}: {err}")
                return None
            if not isinstance(v, str):
                self.add(item.file, cite.ln("source"), f"{c['source']}: the pointer names no string")
                return None
            return v, "text"
        f = base / path
        try:
            lines = f.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError:
            self.add(item.file, cite.ln("source"), f"{sid}: no file {path} at the pin")
            return None
        if l2 > len(lines):
            self.add(item.file, cite.ln("source"), f"{c['source']}: lines {l1}-{l2} beyond the file's {len(lines)}")
            return None
        return "\n".join(lines[l1 - 1:l2]), notation_of(path)

    def check_published(self, item, cite):
        c = cite.data
        got = self.cited_text(item, cite)
        if got is None:
            return
        text, notation = got
        quote = c.get("quote")
        if isinstance(quote, str) and quote.strip():
            self.counts["quotes"] += 1
            if normalise(quote) not in normalise(text):
                self.add(item.file, cite.ln("quote"), f"the quote is not in {c['source']} at the pin")
        ident = r"[\w-]" if notation == "css" else r"\w"
        for name in c.get("absent", []) if isinstance(c.get("absent"), list) else []:
            if isinstance(name, str) and name and re.search(rf"(?<!{ident}){re.escape(name)}(?!{ident})", text):
                self.add(item.file, cite.ln("absent"), f"absent name {name!r} occurs in {c['source']}")

    def check_quote_literals(self, item):
        if item.table == "model" or "value" not in item.data:
            return
        value = item.data["value"]
        texts = [] if item.palette else check_facts.text_leaves(
            self.ctx, value, check_facts.key_info(self.ctx, item.data.get("key"))[4])
        vlits = value_literals(without_paths(value, {p for p, _ in texts}))
        if not vlits and not texts:
            return
        textual = [c for c in item.cites if c.data.get("kind") in ("published", "read")]
        if item.palette:
            groups = [("all its citations", textual)] if textual else []
        else:
            groups = []
            for g in sorted(served_generations(item)):
                cs = [c for c in textual if g in self.ctx.cite_generations(c.data)]
                if cs:
                    groups.append((g, cs))
        for g, cs in groups:
            self.counts["generations"] += 1
            union, text_union = [], set()
            for c in cs:
                src = check_facts.parse_source(c.data.get("source"))
                notation = "text" if not src or src[4] is not None else notation_of(src[1])
                conv = conversions_of(c.data)
                quote = c.data.get("quote") if isinstance(c.data.get("quote"), str) else ""
                union += convert(quote_literals(quote, notation, self.apple_only(c.data)), conv)
                text_union |= quote_texts(quote)
            for v in vlits:
                if not any(equal(v, q) for q in union):
                    self.add(item.file, item.ln("value"), f"the value's literal {v} is in no quote serving {g} (as "
                                                          f"written or converted)")
            for path, t in texts:
                if normalise(t) not in text_union:
                    self.add(item.file, item.ln("value"), f"the value's text {t!r}{f' ({path})' if path else ''} is "
                                                          f"no whole string of a quote serving {g} (a quoted string, "
                                                          f"a list's entry, a font description's family)")

    def check_measured(self, item, cite, runs):
        c = cite.data
        ev = check_facts.parse_evidence(c.get("evidence"))
        if not ev:
            return
        self.counts["measured"] += 1
        file, form, harness, run, image = ev
        runs.setdefault((harness, run), []).append((item, cite))
        if file not in self.committed:
            self.add(item.file, cite.ln("evidence"), f"{file} is not committed")
        p = self.ctx.root / file
        if not p.is_file():
            self.add(item.file, cite.ln("evidence"), f"{file} does not exist")
            return
        rt = self.ctx.run_toml(harness, run)
        images = rt.get("images") if isinstance(rt, dict) else None
        if not isinstance(images, dict) or image not in images:
            self.add(item.file, cite.ln("evidence"), f"the run's run.toml maps no image {image!r}")
        try:
            value = evidence_value(p, form)
        except EvidenceError as err:
            self.add(item.file, cite.ln("evidence"), f"{file}#{form}: {err}")
            return
        if item.table == "model" or "value" not in item.data:
            return
        v = item.data["value"]
        if "inputs" in c and isinstance(v, str) and v.startswith("="):
            for problem in self.pass_problems(item, cite, value, p):
                self.add(item.file, cite.ln("evidence"), f"{file}#{form}: {problem}")
            return
        conv = conversions_of(c)
        notation = "ini" if notation_of(p.name) == "ini" else "data"
        for problem in compare(item.data["value"], value, c.get("tolerance", 0), conv, notation, self.apple_only(c)):
            self.add(item.file, cite.ln("evidence"), f"{file}#{form}: {problem}")

    def pass_problems(self, item, cite, passes, path) -> list:
        """A measured derivation's evidence (S17 *A measured derivation*): a list of pass tables in a TOML file,
        each the varied references (`inputs`) as quoted keys, `result`, and `conditions` unless the fact's `when`
        fixes the scheme, every value spelled as a fact's `value`; for each input and each scheme the fact covers,
        two passes that differ in that input alone (beyond its quote-check precision) and whose results differ by
        more than `tolerance`. The result is a fact's value as written: no `conversion` applies to it."""
        c = cite.data
        inputs = c.get("inputs")
        if not check_facts.str_list(inputs) or not inputs:
            return []
        if path.suffix != ".toml":
            return ["a measured derivation's evidence is a TOML file"]
        if not isinstance(passes, list) or not passes or not all(isinstance(x, dict) for x in passes):
            return ["names no list of pass tables (a measured derivation, S17)"]
        ctx = self.ctx
        _, dim, _, _, typ = check_facts.key_info(ctx, item.data.get("key"))
        when = item.data.get("when") if isinstance(item.data.get("when"), dict) else {}
        conds = when.get("conditions") if check_facts.str_list(when.get("conditions")) else []
        nots = when.get("not_conditions") if check_facts.str_list(when.get("not_conditions")) else []
        fixed = [x[7:] for x in conds if x.startswith("scheme:")]
        schemes = set(fixed[:1]) if fixed else {"light", "dark"} - {x[7:] for x in nots if x.startswith("scheme:")}
        types = {r: check_facts.key_info(ctx, r) for r in inputs}
        out, scheme_of = [], []
        for i, ps in enumerate(passes):
            w = f"pass {i}"
            out += [f"{w}: {k!r} is neither one of `inputs` nor `result` or `conditions`" for k in ps
                    if k not in inputs and k not in ("result", "conditions")]
            for r in inputs:
                if r not in ps:
                    out.append(f"{w}: no {r!r}")
                else:
                    out += check_facts.value_problems(ctx, ps[r], types[r][1], f"{w}: {r!r}", types[r][4])
            if "result" not in ps:
                out.append(f"{w}: no `result`")
            else:
                out += check_facts.value_problems(ctx, ps["result"], dim, f"{w}: result", typ)
            sc = next(iter(schemes)) if len(schemes) == 1 else None
            if "conditions" in ps:
                cs = ps["conditions"]
                if not check_facts.str_list(cs):
                    out.append(f"{w}: conditions is not a list of condition strings")
                else:
                    out += [f"{w}: {x!r} is not a condition (S7.1)" for x in cs if not ctx.ck.cond_ok(x)]
                    named = [x[7:] for x in cs if x.startswith("scheme:")]
                    if len(named) > 1:
                        out.append(f"{w}: its conditions name two schemes")
                    elif named:
                        sc = named[0]
                    elif len(schemes) != 1:
                        out.append(f"{w}: its conditions name no scheme (the fact's `when` does not fix it)")
            elif len(schemes) != 1:
                out.append(f"{w}: no `conditions` (the fact's `when` does not fix the scheme)")
            scheme_of.append(sc)
        if out:
            return out
        tol = c.get("tolerance", 0)
        for r in inputs:
            for s in sorted(schemes):
                group = [ps for ps, sc in zip(passes, scheme_of) if sc == s]
                if not any(pass_pair(a, b, r, inputs, tol) for a, b in itertools.combinations(group, 2)):
                    out.append(f"no two passes under scheme:{s} that differ in {r!r} alone and whose results differ "
                               f"by more than the tolerance")
        return out

    def check_run_toml(self, harness, run):
        rel = f"docs/evidence/{harness}/{run}/run.toml"
        p = self.ctx.root / rel
        if not p.is_file():
            self.add(rel, 1, "missing: a run's run.toml maps each image folder to its preset (S17)")
            return
        text = p.read_text(encoding="utf-8")

        def ln(needle):
            return next((i for i, line in enumerate(text.splitlines(), 1) if needle in line), 1)

        data = self.ctx.run_toml(harness, run)
        if data is None:
            self.add(rel, 1, "not valid TOML")
            return
        for k in data:
            if k != "images":
                self.add(rel, ln(k), f"unknown table `{k}` (run.toml holds [images.<image>] only)")
        images = data.get("images", {}) if isinstance(data.get("images"), dict) else {}
        folder = self.ctx.root / "docs/evidence" / harness / run
        on_disk = {d.name for d in folder.iterdir() if d.is_dir() and d.name != ANALYSIS} if folder.is_dir() else set()
        required, optional = RUN_FIELDS
        for image, e in images.items():
            where = ln(f"[images.{image}]")
            if image not in on_disk:
                self.add(rel, where, f"image {image!r} has no folder in the run")
            if not isinstance(e, dict):
                self.add(rel, where, f"images.{image} is not a table")
                continue
            for f in sorted(required - e.keys()):
                self.add(rel, where, f"images.{image}: missing `{f}`")
            for f in sorted(e.keys() - required - optional):
                self.add(rel, where, f"images.{image}: unknown field `{f}`")
            src = self.ctx.sources.get(e["source"]) if isinstance(e.get("source"), str) else None
            if "source" in e and src is None:
                self.add(rel, where, f"images.{image}: source {e['source']!r} is not a [[source]] of sources.toml")
            elif src is not None and e.get("preset") not in (src.get("generations") if isinstance(
                    src.get("generations"), list) else []):
                self.add(rel, where, f"images.{image}: preset {e.get('preset')!r} is not among {e['source']}'s "
                                     f"generations")
        for image in sorted(on_disk - images.keys()):
            self.add(rel, 1, f"image folder {image!r} has no [images.{image}] entry")

    def input_images(self, cited):
        """The images the cited analysis outputs name (a top-level `input_image` in a TOML or JSON evidence file:
        a repository path, or a list of them), each a finding where it is no file."""
        out = set()
        for f in sorted(cited):
            p = self.ctx.root / f
            if p.suffix not in (".toml", ".json") or not p.is_file():
                continue
            try:
                text = p.read_text(encoding="utf-8")
                data = tomllib.loads(text) if p.suffix == ".toml" else json.loads(text)
            except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError, json.JSONDecodeError):
                continue
            names = data.get("input_image") if isinstance(data, dict) else None
            if names is None:
                continue
            for name in names if isinstance(names, list) else [names]:
                if not isinstance(name, str) or not name.startswith("docs/evidence/") or ".." in name.split("/"):
                    self.add(f, 1, f"input_image {name!r} is not a repository path under docs/evidence/")
                elif not (self.ctx.root / name).is_file() and name not in self.committed:
                    self.add(f, 1, f"{f} names the input image {name}, which does not exist")
                else:
                    out.add(name)
        return out

    def check_evidence_files(self, cited):
        root = self.ctx.root
        base = root / "docs/evidence"
        files = {p.relative_to(root).as_posix() for p in base.rglob("*") if p.is_file()} if base.is_dir() else set()
        files |= {p for p in self.committed if p.startswith("docs/evidence/")}
        legacy = tuple(x.rstrip("/") + "/" for x in LEGACY_EVIDENCE)
        runs = set()
        for f in sorted(files):
            parts = f.split("/")
            if parts[-1] == "README.md":
                continue
            if f.startswith(legacy):
                self.counts["legacy"] += 1
                continue
            if len(parts) == 5 and parts[4] == "run.toml" and RUN_RX.match(parts[3]):
                runs.add((parts[2], parts[3]))
                continue
            if len(parts) >= 6 and parts[4] == ANALYSIS and RUN_RX.match(parts[3]):
                runs.add((parts[2], parts[3]))
                continue
            if len(parts) < 6 or not RUN_RX.match(parts[3]):
                self.add(f, 1, "not in a run's image folder (docs/evidence/<harness>/<date>-run-<id>/<image>/)")
                continue
            runs.add((parts[2], parts[3]))
            self.counts["evidence"] += 1
            if f not in cited:
                self.add(f, 1, "evidence file cited by no fact, state-model entry or vector table")
        return runs

    def check(self):
        items, _, _ = check_facts.load(self.ctx)
        runs = {}
        for it in items:
            for cite in it.cites:
                kind = cite.data.get("kind")
                if kind in ("published", "read"):
                    self.check_published(it, cite)
                elif kind == "measured":
                    self.check_measured(it, cite, runs)
            self.check_quote_literals(it)
        cited = {check_facts.parse_evidence(c.data.get("evidence"))[0] for it in items for c in it.cites
                 if c.data.get("kind") == "measured" and check_facts.parse_evidence(c.data.get("evidence"))}
        cited |= vector_table_evidence(self.ctx.root)
        cited |= self.input_images(cited)
        all_runs = self.check_evidence_files(cited) | set(runs)
        legacy = {tuple(x.split("/")[2:4]) for x in LEGACY_EVIDENCE}
        for harness, run in sorted(all_runs - legacy):
            self.check_run_toml(harness, run)
        return items

    def summary(self, items):
        n = self.counts
        facts = sum(1 for it in items if it.table == "fact" and not it.palette)
        models = sum(1 for it in items if it.table == "model")
        palette = sum(1 for it in items if it.palette)
        vacuous = "no facts yet: " if not items else ""
        return (f"{vacuous}{facts} facts, {models} state-model entries, {palette} palette facts: {n['quotes']} quotes "
                f"checked at their pins, {n['generations']} literal checks per generation, {n['measured']} measured "
                f"citations, {len(self.sdk)} SDK citations left to os-probe.yml's macOS job; {n['evidence']} evidence "
                f"files ({n['legacy']} more in {len(LEGACY_EVIDENCE)} legacy runs): {len(self.findings)} findings")


# --------------------------------------------------------------------------------------------- self-test

SELF_TEST_EXTRA = {
    "docs/facts/gnome/more.toml": '''
[[fact]]
key = "button.body.shadow"
value = "absent"
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:10-11"
quote = "&:hover {"
absent = ["box-shadow"]

[[fact]]
key = "button.body.min_width"
value = "= mul(token.kde.frame_width, 0.5)"
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:12"
quote = "opacity: 50%;"
conversion = ["percent_to_fraction"]
''',
    "docs/facts/macos/text.toml": '''
[[fact]]
key = "button.label.text.font.size"
when = { conditions = ["preset:macos-tahoe"] }
value = "17px"
rows = []

[[fact.cite]]
kind = "published"
source = "hig-typography@2026-10-29 #/sections/1/text"
quote = "Body 17 pt"

[[fact]]
key = "button.label.text.font.weight"
value = 590
rows = []

[[fact.cite]]
kind = "published"
source = "xcode@16.4 MacOSX.platform/Developer/SDKs/MacOSX.sdk/fixture.h:3"
quote = "the fixture's weight 590"

[[fact.cite]]
kind = "published"
source = "hig-typography@2026-10-29 #/sections/2/text"
quote = "weight 590"
''',
    "docs/facts/windows/more.toml": '''
[[fact]]
key = "button.body.opacity"
value = 0.5
rows = []

[[fact.cite]]
kind = "measured"
evidence = "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/shadows.toml#button.disabled.opacity_percent"
conversion = ["percent_to_fraction"]
tolerance = 0
''',
    "cache/hig-typography@2026-10-29/page.json": '{"sections": [{"text": "Large Title 26 pt"}, '
                                                 '{"text": "The   Body 17 pt style."}, {"text": "weight 590"}]}',
    "cache/libadwaita@1.10.0/src/stylesheet/widgets/_buttons.scss": "\n" * 9 + "  &:hover {\n"
                                                                    "    background-color: $hover_color; }\n"
                                                                    "  opacity: 50%;\n",
    "cache/libadwaita@1.10.0/src/stylesheet/_colors.scss": "// colours\n\n$hover_color: alpha(currentColor, .07);\n",
    "cache/libadwaita@1.10.0/src/stylesheet/_fonts.scss": 'font-family: "Adwaita Sans", sans-serif;\n'
                                                          '/* "go-next-symbolic", "go-next" */\n'
                                                          "  /* 'Noto Sans Mono' */\n",
    "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-26/passes.toml": '''
[[button.primary.fill]]
"input.accent" = "#ff0000"
result = "#ff000080"
conditions = ["scheme:light"]

[[button.primary.fill]]
"input.accent" = "#0000ff"
result = "#0000ff80"
conditions = ["scheme:light"]

[[button.primary.fill]]
"input.accent" = "#ff0000"
result = "#ff000080"
conditions = ["scheme:dark"]

[[button.primary.fill]]
"input.accent" = "#0000ff"
result = "#0000ff80"
conditions = ["scheme:dark"]
''',
    "cache/breeze@6.8.2/colors/BreezeLight.colors": "[ColorEffects:Disabled]\nColor=56,56,56\n\n[Colors:Window]\n"
                                                    "BackgroundNormal=239,240,241\n",
    "cache/microsoft-ui-xaml@2.5.1/controls/dev/CommonStyles/Common_themeresources_any.xaml":
        '<ResourceDictionary>\n  <ResourceDictionary.ThemeDictionaries>\n    <ResourceDictionary x:Key="Dark">\n'
        '      <Color x:Key="SubtleFillColorSecondary">#0FFFFFFF</Color>\n',
    "cache/microsoft-ui-xaml@2.5.1/controls/dev/CommonStyles/Button_themeresources.xaml":
        '<ResourceDictionary>\n\n\n\n\n<VisualState x:Name="PointerOver">\n'
        '  <DoubleAnimation Storyboard.TargetProperty="Opacity" Duration="0:0:0.083" />\n</VisualState>\n'
        '<x:Double x:Key="Button_MinWidth">80</x:Double>\n',
    "cache/catppuccin@1.7.1/palette.json": "\n" * 119 + '        "hex": "#313244",\n',
    "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-15/probe.toml":
        "[macos.fonts.sizes]\nsystemFontSize = 13.0\n",
    "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-26/probe.toml":
        "[macos.fonts.sizes]\nsystemFontSize = 13.0\n",
    "docs/evidence/os-probe/2026-11-02-run-1/README.md": "The run's harness and commit.\n",
    "docs/evidence/kde-accent/2026-11-03-run-2/kde-breeze-light/kdeglobals":
        "[Colors:Header]\nBackgroundNormal=222,224,226\n\n[Colors:Header][Inactive]\nBackgroundNormal=227,229,231\n",
    "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/shadows.toml": '''
input_image = "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/button.png"

[button.rest]
layers = [ { x_px = 0, y_px = 1.2, blur_px = 2.5, spread_px = 0, color = "#00000025", inset = false },
           { x_px = 0, y_px = 4, blur_px = 8.6, spread_px = 0, color = "#0000001b", inset = false } ]

[button.disabled]
opacity_percent = 50
''',
    "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/button.png": "png",
    "docs/evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-15/renders/button.png": "legacy",
}

_SHADOW = re.search(r"value = \{ layers.*?\] \}", check_facts.SELF_TEST_FILES["docs/facts/windows/buttons.toml"],
                    re.S).group(0)
_SHADOW_TOL = re.search(r"tolerance = \{ \"layers.*\}", check_facts.SELF_TEST_FILES["docs/facts/windows/buttons.toml"]
                        ).group(0)

_PASSES = "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-26/passes.toml"

# (label, file, old, new, wanted findings as substrings, one per finding); old None: the file is replaced by new (None:
# removed); old a list of (old, new) pairs: each replaced
SELF_TEST_CASES = [
    ("libadwaita: a literal no quote holds", "docs/facts/gnome/buttons.toml", "0.07)", "0.08)",
     ["0.08 is in no quote serving adwaita"]),
    ("libadwaita: the quote at other lines", "docs/facts/gnome/buttons.toml", "_colors.scss:3", "_colors.scss:2",
     ["the quote is not in libadwaita@1.10.0 src/stylesheet/_colors.scss:2"]),
    ("libadwaita: lines beyond the file", "docs/facts/gnome/buttons.toml", "_colors.scss:3", "_colors.scss:3-9",
     ["lines 3-9 beyond the file's 3"]),
    ("libadwaita: a file not at the pin", "docs/facts/gnome/buttons.toml", "_colors.scss:3", "_colours.scss:3",
     ["no file src/stylesheet/_colours.scss at the pin"]),
    ("libadwaita: an absent name present", "docs/facts/gnome/more.toml", 'absent = ["box-shadow"]',
     'absent = ["box-shadow", "background-color"]', ["absent name 'background-color' occurs"]),
    ("libadwaita: a percentage without its conversion", "docs/facts/gnome/more.toml",
     'conversion = ["percent_to_fraction"]\n', "", ["0.5 is in no quote serving adwaita"]),
    ("Breeze: another colour", "docs/facts/kde/colors.toml", 'value = "#eff0f1"', 'value = "#eff0f2"',
     ["srgb(239, 240, 242, 255) is in no quote serving kde-breeze"]),
    ("Breeze: the byte list read as numbers too", "docs/facts/kde/colors.toml", 'value = "#eff0f1"',
     'value = "= alpha(role.view.background, 240)"', ["240 is in no quote serving kde-breeze"]),
    ("WinUI: #aarrggbb read as RGBA", "docs/facts/windows/buttons.toml", 'value = "#ffffff0f"', 'value = "#0fffffff"',
     ["srgb(15, 255, 255, 255) is in no quote serving windows-11"]),
    ("WinUI: another TimeSpan", "docs/facts/windows/buttons.toml", 'value = "83ms"', 'value = "84ms"',
     ["84ms is in no quote serving windows-11"]),
    ("WinUI: a TimeSpan's parts read as numbers", "docs/facts/windows/buttons.toml", 'value = "83ms"',
     'value = "0ms"', ["0ms is in no quote serving windows-11"]),
    ("WinUI: another minimum width", "docs/facts/windows/buttons.toml", 'value = "80px"', 'value = "88px"',
     ["88px is in no quote serving windows-11"]),
    ("WinUI: a point where the value is px", "docs/facts/windows/buttons.toml", ">80</x:Double>", ">80pt</x:Double>",
     ["the quote is not in", "80px is in no quote serving windows-11"]),
    ("shadow: a leaf beyond its tolerance", "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/"
     "shadows.toml", "y_px = 1.2", "y_px = 1.6", ["leaf 'layers.0.y_px': the evidence holds 1.6"]),
    ("shadow: a colour beyond its tolerance", "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/"
     "shadows.toml", '"#0000001b"', '"#0000001e"', ["leaf 'layers.1.color'"]),
    ("shadow: a bool that differs", "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/shadows.toml",
     "inset = false },\n", "inset = true },\n", ["leaf 'layers.0.inset': the evidence holds True"]),
    ("shadow: a leaf without its tolerance", "docs/facts/windows/buttons.toml", ', "layers.1.color" = 2 }', " }",
     ["no tolerance for the leaf 'layers.1.color'"]),
    ("shadow: a tolerance for no leaf", "docs/facts/windows/buttons.toml", '"layers.1.color" = 2 }',
     '"layers.1.color" = 2, "layers.2.color" = 2 }', ["tolerance 'layers.2.color' is not a numeric or colour leaf"]),
    ("shadow: a leaf the evidence lacks", "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/"
     "shadows.toml", "spread_px = 0, color = \"#00000025\"", "color = \"#00000025\"",
     ["the evidence has no leaf 'layers.0.spread_px'"]),
    ("macOS: 26 measures another size", "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-26/probe.toml",
     "13.0", "14.0", ["the evidence holds 14, the fact 13pt"]),
    ("macOS: a path the evidence lacks", "docs/facts/macos/buttons.toml", "macos-26/probe.toml#macos.fonts.sizes."
     "systemFontSize", "macos-26/probe.toml#macos.fonts.sizes.labelFontSize", ["no 'labelFontSize'"]),
    ("macOS: run.toml without the build", "docs/evidence/os-probe/2026-11-02-run-1/run.toml", 'os = "25G83"\n', "",
     ["images.os-probe-macos-26: missing `os`"]),
    ("macOS: run.toml's preset not the source's", "docs/evidence/os-probe/2026-11-02-run-1/run.toml",
     'source = "xcode@26.6"', 'source = "xcode@16.4"', ["preset 'macos-tahoe' is not among xcode@16.4's generations"]),
    ("macOS: run.toml with an unknown field", "docs/evidence/os-probe/2026-11-02-run-1/run.toml", 'os = "25G83"',
     'os = "25G83"\nxcode = "26.6"', ["unknown field `xcode`"]),
    ("macOS: an image folder without an entry", "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-14/x.toml",
     None, "a = 1\n", ["evidence file cited by no fact", "image folder 'os-probe-macos-14' has no [images"]),
    ("macOS: a run without run.toml", "docs/evidence/os-probe/2026-11-02-run-1/run.toml", None, None,
     ["missing: a run's run.toml", "maps no image 'os-probe-macos-15'", "maps no image 'os-probe-macos-26'",
      "maps no image 'os-probe-macos-26'"]),
    ("evidence: an uncited file", "docs/evidence/os-probe/2026-11-02-run-1/os-probe-macos-15/extra.txt", None, "x\n",
     ["evidence file cited by no fact"]),
    ("evidence: a file outside an image folder", "docs/evidence/os-probe/2026-11-02-run-1/notes.txt", None, "x\n",
     ["not in a run's image folder"]),
    ("evidence: a legacy file is cited", "docs/evidence/os-probe/2026-10-26-run-40348854007/os-probe-macos-26/x.png",
     None, "legacy", []),
    ("KDE accent: another inactive colour", "docs/evidence/kde-accent/2026-11-03-run-2/kde-breeze-light/kdeglobals",
     "BackgroundNormal=227,229,231", "BackgroundNormal=227,229,232", ["the evidence holds srgb(227, 229, 232, 255)"]),
    ("KDE accent: the active group's key", "docs/facts/kde/colors.toml", "Colors:Header][Inactive/BackgroundNormal",
     "Colors:Header/BackgroundNormal", ["the evidence holds srgb(222, 224, 226, 255)"]),
    ("KDE accent: a group the file lacks", "docs/facts/kde/colors.toml", "Colors:Header][Inactive/BackgroundNormal",
     "Colors:View][Inactive/BackgroundNormal", ["no key 'BackgroundNormal' in group [Colors:View][Inactive]"]),
    ("Catppuccin: another colour", "docs/facts/palettes/catppuccin.toml", 'value = "#313244"', 'value = "#313245"',
     ["is in no quote serving all its citations"]),
    ("Catppuccin: a quote not at the pin", "docs/facts/palettes/catppuccin.toml", "palette.json:120",
     "palette.json:119", ["the quote is not in catppuccin@1.7.1 palette.json:119"]),
    ("HIG: a pointer to a table", "docs/facts/macos/text.toml", "#/sections/1/text", "#/sections/1",
     ["the pointer names no string"]),
    ("HIG: a pointer to nothing", "docs/facts/macos/text.toml", "#/sections/1/text", "#/sections/3/text",
     ["no element '3'"]),
    ("HIG: a quote not in the page's string", "docs/facts/macos/text.toml", 'quote = "Body 17 pt"',
     'quote = "Body 17pt"', ["the quote is not in hig-typography@2026-10-29 #/sections/1/text"]),
    ("HIG: another size", "docs/facts/macos/text.toml", 'value = "17px"', 'value = "26px"',
     ["26px is in no quote serving macos-tahoe"]),
    ("HIG: a typographic point where Apple's is px", "docs/facts/macos/text.toml", 'value = "17px"',
     'value = "17pt"', ["17pt is in no quote serving macos-tahoe"]),
    ("HIG: a snapshot folder with two files", "cache/hig-typography@2026-10-29/other.json", None, "{}", []),
    ("HIG: the snapshot's file missing", "cache/hig-typography@2026-10-29/page.json", None, None,
     ["no file page.json", "no file page.json"]),
    ("HIG: a snapshot entry naming no file", "docs/facts/sources.toml", 'file = "page.json"\n', "",
     ["names no `file`", "names no `file`"]),
    ("macOS: a 15 value cited only from a 26 source", "docs/facts/macos/text.toml",
     'quote = "the fixture\'s weight 590"', 'quote = "the fixture\'s weight 510"',
     ["590 is in no quote serving macos-monterey"]),
    ("libadwaita: an absent name inside an identifier", "docs/facts/gnome/more.toml", 'absent = ["box-shadow"]',
     'absent = ["hover_col"]', []),
    ("shadow: two layers against three", "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/"
     "shadows.toml", 'inset = false } ]', 'inset = false },\n           { x_px = 0, y_px = 8, blur_px = 16, '
     'spread_px = 0, color = "#00000010", inset = false } ]', ["'layers': the evidence has 3 items, the fact 2"]),
    ("shadow: one layer against two", "docs/facts/windows/buttons.toml",
     [(_SHADOW, 'value = { layers = [ { x_px = 0, y_px = 1, blur_px = 2, spread_px = 0, color = "#00000024", '
                'inset = false } ] }'), (_SHADOW_TOL, "tolerance = 2")],
     None, ["'layers': the evidence has 2 items, the fact 1"]),
    ("shadow: no layers against two", "docs/facts/windows/buttons.toml",
     [(_SHADOW, "value = { layers = [] }"), (_SHADOW_TOL, "tolerance = 2")], None,
     ["'layers': the evidence has 2 items, the fact 0"]),
    ("shadow: a key the fact lacks", "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/"
     "shadows.toml", "inset = false },\n", "inset = false, offset_px = 1 },\n",
     ["the evidence has 'layers.0.offset_px', the fact does not"]),
    ("evidence: a conversion left out", "docs/facts/windows/more.toml", 'conversion = ["percent_to_fraction"]\n', "",
     ["the evidence holds 50, the fact 0.5"]),
    ("evidence: another converted value", "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/"
     "shadows.toml", "opacity_percent = 50", "opacity_percent = 51", ["the evidence holds 0.51, the fact 0.5"]),
    ("analysis: its input image missing", "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/"
     "button.png", None, None, ["names the input image docs/evidence/winui-host/2026-11-04-run-3/"
                                "winui-host-windows-11/button.png, which does not exist"]),
    ("analysis: an image its output does not name", "docs/evidence/winui-host/2026-11-04-run-3/winui-host-windows-11/"
     "shadows.toml", "/button.png", "/bezel.png", ["bezel.png, which does not exist",
                                                   "button.png:1: evidence file cited by no fact"]),
    # text values (S17 *The quote check*)
    ("text: a family the quote lacks", "docs/facts/gnome/text.toml", 'value = "Adwaita Sans"', 'value = "Cantarell"',
     ["the value's text 'Cantarell' is no whole string of a quote serving adwaita"]),
    ("text: Noto Sans is not Noto Sans Mono", "docs/facts/gnome/text.toml",
     [('value = "Adwaita Sans"', 'value = "Noto Sans"'), ("_fonts.scss:1\"\nquote = 'font-family: \"Adwaita Sans\", "
                                                          "sans-serif;'", "_fonts.scss:3\"\nquote = 'Noto Sans Mono'")],
     None, ["the value's text 'Noto Sans' is no whole string"]),
    ("text: go-next is not go-next-symbolic", "docs/facts/gnome/text.toml", 'fallbacks = ["go-next"]',
     'fallbacks = ["go-nex"]', ["'go-nex' (theme_icon.fallbacks.0) is no whole string"]),
    ("text: case-sensitive", "docs/facts/gnome/text.toml", 'value = "Adwaita Sans"', 'value = "adwaita sans"',
     ["'adwaita sans' is no whole string"]),
    ("text: a list entry", "docs/facts/gnome/text.toml", '["Adwaita Sans", "sans-serif"]',
     '["Adwaita Sans", "serif"]', ["'serif' (1) is no whole string"]),
    ("text: a number in a family is not a literal", "docs/facts/gnome/text.toml", 'value = "Adwaita Sans"',
     'value = "Adwaita Sans 2"', ["'Adwaita Sans 2' is no whole string"]),
    # absent = [] (S17 *Absence*): the quote check of the cited definition only
    ("absence: no name for it", "docs/facts/gnome/more.toml", 'absent = ["box-shadow"]', "absent = []", []),
    # measured derivations (S17)
    ("derivation: results that do not differ under dark", _PASSES, 'result = "#0000ff80"\nconditions = ["scheme:dark"]',
     'result = "#ff000080"\nconditions = ["scheme:dark"]', ["no two passes under scheme:dark that differ in "
                                                          "'input.accent' alone"]),
    ("derivation: results within the tolerance", _PASSES, 'result = "#0000ff80"\nconditions = ["scheme:light"]',
     'result = "#ff010080"\nconditions = ["scheme:light"]', ["no two passes under scheme:light"]),
    ("derivation: inputs within their precision", _PASSES, '"input.accent" = "#0000ff"\nresult = "#0000ff80"\n'
     'conditions = ["scheme:light"]', '"input.accent" = "#ff0000"\nresult = "#0000ff80"\nconditions = ["scheme:light"]',
     ["no two passes under scheme:light"]),
    ("derivation: passes differing in their conditions too", _PASSES, 'result = "#0000ff80"\nconditions = '
     '["scheme:light"]', 'result = "#0000ff80"\nconditions = ["scheme:light", "contrast_high"]',
     ["no two passes under scheme:light"]),
    ("derivation: a pass without conditions", _PASSES, 'result = "#0000ff80"\nconditions = ["scheme:light"]\n',
     'result = "#0000ff80"\n', ["pass 1: no `conditions`"]),
    ("derivation: a pass naming no scheme", _PASSES, 'conditions = ["scheme:light"]', 'conditions = []',
     ["pass 0: its conditions name no scheme"]),
    ("derivation: the scheme fixed by when", "docs/facts/macos/buttons.toml", '"preset:macos-tahoe"], variants',
     '"preset:macos-tahoe", "scheme:dark"], variants', []),
    ("derivation: a key that is no input", _PASSES, 'result = "#ff000080"\nconditions = ["scheme:light"]',
     'result = "#ff000080"\n"input.focus" = "#ffffff"\nconditions = ["scheme:light"]',
     ["pass 0: 'input.focus' is neither one of `inputs`"]),
    ("derivation: a result without its spelling", _PASSES, 'result = "#ff000080"\nconditions = ["scheme:light"]',
     'result = 5\nconditions = ["scheme:light"]', ["pass 0: result: a number where the key is a paint"]),
    ("derivation: an unknown condition", _PASSES, 'conditions = ["scheme:light"]',
     'conditions = ["scheme:light", "window_hidden"]', ["'window_hidden' is not a condition"]),
    ("derivation: a path to no list", "docs/facts/macos/buttons.toml", "passes.toml#button.primary.fill",
     "passes.toml#button.primary.fill.0", ["names no list of pass tables"]),
    # a run's analysis scripts are not evidence (the ledger's queue #30)
    ("analysis: a run's analysis script", "docs/evidence/os-probe/2026-11-02-run-1/analysis/measure.py", None,
     "print(1)\n", []),
    ("a distribution cited", "docs/facts/gnome/more.toml", 'source = "libadwaita@1.10.0 src/stylesheet/widgets/'
     '_buttons.scss:12"', 'source = "archlinux@2026-10-29 x:1"', ["is a distribution snapshot"]),
]

# (notation, text, the literals read)
SELF_TEST_NOTATIONS = [
    ("css", "#eff0f1 #abc", ["srgb(239, 240, 241, 255)", "srgb(170, 187, 204, 255)"]),
    ("css", "#0000001a #0008", ["srgb(0, 0, 0, 26)", "srgb(0, 0, 0, 136)"]),
    ("css", "rgba(0, 0, 0, 0.5) rgb(255 0 0 / 50%)", ["srgb(0, 0, 0, 127.5)", "srgb(255, 0, 0, 127.5)"]),
    ("css", "rgba(#000, .5) white transparent", ["srgb(0, 0, 0, 127.5)", "srgb(255, 255, 255, 255)",
                                                  "srgb(0, 0, 0, 0)"]),
    ("css", "alpha(currentColor, .07); margin: -1px", ["0.07", "-1px"]),
    ("xaml", "#0FFFFFFF #8FFF #eff0f1", ["srgb(255, 255, 255, 15)", "srgb(255, 255, 255, 136)",
                                         "srgb(239, 240, 241, 255)"]),
    ("xaml", 'Duration="0:0:0.083" KeyTime="1.0:0:1"', ["83ms", "8.6401e+07ms"]),
    ("text", "#0FFFFFFF #00000099 white", []),
    ("qml", "Qt.rgba(1, 0.5, 0, 0.25)", ["srgb(255, 127.5, 0, 63.75)"]),
    ("cpp", "QColor(61, 174, 233) { 239, 240, 241 } {1, 2, 300}", ["srgb(61, 174, 233, 255)",
                                                                   "srgb(239, 240, 241, 255)", "1", "2", "300"]),
    ("ini", "BackgroundNormal=239,240,241,128", ["srgb(239, 240, 241, 128)"]),
    ("text", "Qt6 holds 6px; 0x1F; v6.8 is 0.2s", ["6px", "200ms"]),
    ("text", "4dp 2sp 3pt 90deg 1rem 2em 50%", ["4px", "2px", "3pt", "90deg", "50"]),
    ("css", "RGB(0 0 6 / 80%) RGBA(255, 0, 0, 0.5)", ["srgb(0, 0, 6, 204)", "srgb(255, 0, 0, 127.5)"]),
    ("kotlin", "Color(red = 65, green = 14, blue = 11) Color(1f, 0f, 0f) "
               "Color(red = 0.5f, green = 0f, blue = 0f, alpha = 0.25f)",
     ["srgb(65, 14, 11, 255)", "srgb(255, 0, 0, 255)", "srgb(127.5, 0, 0, 63.75)"]),
    ("kotlin", "Color(0xFF5C77FF) val scrim = 0x1F000000; 0x1F", ["srgb(92, 119, 255, 255)", "srgb(0, 0, 0, 31)"]),
    ("kotlin", "24.0.sp 16.dp 0.38f 2L 1.5.em", ["24px", "16px", "0.38", "2"]),
    ("android", "<color>#80FF0000</color> #8F00 #eff0f1 16dp", ["srgb(255, 0, 0, 128)", "srgb(255, 0, 0, 136)",
                                                              "srgb(239, 240, 241, 255)", "16px"]),
    ("qml", 'color: "transparent"; color: "Red"; red: 1; text: "white space"',
     ["srgb(0, 0, 0, 0)", "srgb(255, 0, 0, 255)", "1"]),
    ("xaml", '<SolidColorBrush Color="Transparent" /><Color>White</Color> x:Key="Black" Fill="LightGray"',
     ["srgb(255, 255, 255, 0)", "srgb(255, 255, 255, 255)", "srgb(0, 0, 0, 255)", "srgb(211, 211, 211, 255)"]),
    ("text", "1e-3 and 2.5E+2", ["0.001", "250"]),
    ("data", "display_p3(1, 0.5, 0) srgb(0, 0, 1, 0.5)", ["p3(255, 127.5, 0, 255)", "srgb(0, 0, 255, 127.5)"]),
    ("text", "<default>'Adwaita Sans 11'</default>", ["11"]),
    ("xaml", '<CornerRadius x:Key="ControlCornerRadius">4</CornerRadius> CornerRadius="4"', ["4", "4"]),
]

# (quote, text value, whether a text notation reads it as a whole string; S17 *The quote check*)
SELF_TEST_TEXTS = [
    ('font-family: "Adwaita Sans", sans-serif;', "Adwaita Sans", True),
    ('font-family: "Adwaita Sans", sans-serif;', "sans-serif", True),
    ('font-family: "Adwaita Sans", sans-serif;', "Adwaita", False),
    ("font-family: Cantarell;", "Cantarell", True),
    ("<default>'Adwaita Sans 11'</default>", "Adwaita Sans", True),
    ("<default>'Adwaita Sans 11'</default>", "Adwaita Sans 11", True),
    ("<default>'Adwaita Sans 11'</default>", "Adwaita Sans 1", False),
    ("gtk-font-name = Cantarell Bold 11", "Cantarell", True),
    ("gtk-font-name = Cantarell SemiBold 11px", "Cantarell", True),
    ("'Noto Sans Mono 10'", "Noto Sans", False),
    ("font=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1", "Noto Sans", True),
    ("font=Noto Sans Mono,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1", "Noto Sans", False),
    ("Fonts=Cantarell;DejaVu Sans;", "DejaVu Sans", True),
    ('gtk_image_new_from_icon_name ("go-next-symbolic");', "go-next-symbolic", True),
    ('gtk_image_new_from_icon_name ("go-next-symbolic");', "go-next", False),
    ('@".AppleSystemUIFont"', ".AppleSystemUIFont", True),
    ('"adwaita sans"', "Adwaita Sans", False),
    ('"Adwaita   Sans"', "Adwaita Sans", True),
    ("'Cantarell Bold'", "Cantarell", True),
    ("'Noto Sans Mono'", "Noto Sans", False),
    ("'Cantarell Italic Light 15 @wght=200 #tnum=1'", "Cantarell", True),
    ("[Icons]\nTheme=breeze", "breeze", True),
    ("[Icons]\nTheme=breeze-dark", "breeze", False),
]

# (notation, text, the literals read) on a source that serves only Apple's presets: a point is a logical point (px)
SELF_TEST_APPLE = [
    ("text", "Body 17 pt; 44x44 pt; 44 × 44 pt; 6pt", ["17px", "44px", "44px", "44px", "44px", "6px"]),
    ("cpp", "NSFontWeightSemibold 0.3 // 13pt", ["0.3", "13px"]),
]


def self_test() -> int:
    failures = []
    for apple, cases in ((False, SELF_TEST_NOTATIONS), (True, SELF_TEST_APPLE)):
        for notation, text, want in cases:
            try:
                got = [str(x) for x in quote_literals(text, notation, apple)]
            except Exception as err:   # noqa: BLE001 - a self-test reports, it does not stop
                got = f"raised {err!r}"
            if got != want:
                failures.append(f"{notation}{' (Apple)' if apple else ''} {text!r}: got {got}, want {want}")
    for names, x, want in ((["percent_to_fraction"], 50, "0.5"), (["byte_to_fraction"], 255, "1"),
                           (["byte_to_fraction", "complement"], 51, "0.8"), (["seconds_to_ms"], 0.25, "250ms")):
        got = [str(v) for v in convert([Lit("num", None, x)], names)]
        if got != [f"{x:g}", want]:
            failures.append(f"conversion {names}: got {got}")
    for quote, text, want in SELF_TEST_TEXTS:
        if (text in quote_texts(quote)) != want:
            failures.append(f"text {text!r} in {quote!r}: got {not want}, want {want}")
    if sorted(CONVERSIONS) != ["byte_to_fraction", "complement", "percent_to_fraction", "seconds_to_ms"]:
        failures.append(f"conversions: got {sorted(CONVERSIONS)}")
    for path, want in (("a/b.scss", "css"), ("x.xaml", "xaml"), ("BreezeLight.colors", "ini"), ("kdeglobals", "ini"),
                       ("kwinrc", "ini"), ("kcolorscheme.cpp", "cpp"), ("NSButton.h", "cpp"), ("Button.qml", "qml"),
                       ("probe.toml", "data"), ("README.md", "text"), ("material3/Menu.kt", "kotlin"),
                       ("lib/java/com/google/android/material/color/res/values/colors.xml", "android"),
                       ("res/values/dimens.xml", "android"), ("data/fonts/fonts.xml", "text")):
        if notation_of(path) != want:
            failures.append(f"notation of {path}: got {notation_of(path)}")
    if not equal(Lit("num", "number", 1.0), Lit("num", None, 1.00009)) or equal(Lit("num", "number", 1.0),
                                                                                  Lit("num", None, 1.0002)):
        failures.append("number equality is not relative 1e-4")
    if not equal(Lit("colour", "srgb", (1, 2, 3, 4)), Lit("colour", "srgb", (1, 2, 3, 4.0009))) or equal(
            Lit("colour", "srgb", (1, 2, 3, 4)), Lit("colour", "srgb", (1, 2, 3, 4.002))):
        failures.append("colour equality is not 1e-3 per channel")
    if equal(Lit("num", "length", 13.0), Lit("num", "pt", 13.0)):
        failures.append("px and pt compare equal")
    if [str(x) for x in value_literals({"layers": [{"y_px": 1, "color": "#00000024", "inset": False}],
                                        "a": "= mix(token.elevation.1, srgb(1, 0, 0), 0.5)"})] != [
            "1px", "srgb(0, 0, 0, 36)", "srgb(255, 0, 0, 255)", "0.5"]:
        failures.append(f"value literals: got {[str(x) for x in value_literals({'a': '= mix(token.elevation.1, x)'})]}")
    with tempfile.TemporaryDirectory() as tmp:
        base = Path(tmp)
        files = {"t.toml": "[a.b]\nc = [1, { d = 2 }]\n", "t.json": '{"a": {"b/c": ["x", "y"]}}',
                 "kdeglobals": "[Colors:Header]\nA=1\n[Colors:Header][Inactive]\nA=2\n", "t.csv": "key,w,h\nbtn,3,4\n"}
        for name, text in files.items():
            (base / name).write_text(text, encoding="utf-8")
        for name, form, want in (("t.toml", "a.b.c.1.d", 2), ("t.json", "/a/b~1c/1", "y"),
                                 ("kdeglobals", "Colors:Header][Inactive/A", "2"),
                                 ("kdeglobals", "Colors:Header/A", "1"),
                                 ("t.csv", "btn/h", "4")):
            try:
                got = evidence_value(base / name, form)
            except EvidenceError as err:
                got = err
            if got != want:
                failures.append(f"evidence path {name}#{form}: got {got!r}")
        n = 0
        for case in [("good", None, None, None, [])] + SELF_TEST_CASES:
            label, path, old, new, want = case
            files = dict(check_facts.SELF_TEST_FILES)
            files.update(SELF_TEST_EXTRA)
            files["docs/facts/sources.toml"] = check_facts.SELF_TEST_SOURCES
            if path is not None:
                pairs = old if isinstance(old, list) else [(old, new)]
                if old is None:
                    if new is None:
                        files.pop(path)
                    else:
                        files[path] = new
                elif any(o not in files[path] for o, _ in pairs):
                    failures.append(f"{label}: fixture text {old!r} not found")
                    continue
                else:
                    for o, nw in pairs:
                        files[path] = files[path].replace(o, nw, 1)
            root = base / f"case{n}"
            n += 1
            check_facts.write_fixture(root, {k: v for k, v in files.items() if not k.startswith("cache/")})
            sources = files.pop("docs/facts/sources.toml")
            (root / check_facts.SOURCES).write_text(sources, encoding="utf-8")
            for k, v in files.items():
                if k.startswith("cache/"):
                    p = root / k
                    p.parent.mkdir(parents=True, exist_ok=True)
                    p.write_text(v, encoding="utf-8")
            committed = {p.relative_to(root).as_posix() for p in (root / "docs/evidence").rglob("*") if p.is_file()}
            ctx = check_facts.Context(root, core_text=check_facts.SELF_TEST_CORE,
                                      cat_text=check_facts.SELF_TEST_CATALOGUE, sources_text=sources,
                                      rows=check_facts.SELF_TEST_ROWS)
            run = Run(ctx, root / "cache", committed, check_heads=False)
            items = run.check()
            got = [f"{f}:{ln}: {w}" for f, ln, w in run.findings]
            if len(got) != len(want) or not all(any(w in g for g in got) for w in want):
                failures.append(f"{label}: got {got}, want {want}")
            if label == "good":
                if len(run.sdk) != 3 or run.counts["legacy"] != 1 or run.counts["measured"] != 6:
                    failures.append(f"good: {run.summary(items)}; sdk {run.sdk}")
        # the quote check does not depend on the presets that exist: none exist, a wrong literal still fails
        root = base / "case0"
        for f in (root / check_facts.PRESET_DIRS[0]).glob("*.toml"):
            f.unlink()
        buttons = root / "docs/facts/gnome/buttons.toml"
        buttons.write_text(buttons.read_text(encoding="utf-8").replace("0.07)", "0.08)"), encoding="utf-8")
        committed = {p.relative_to(root).as_posix() for p in (root / "docs/evidence").rglob("*") if p.is_file()}
        ctx = check_facts.Context(root, core_text=check_facts.SELF_TEST_CORE, cat_text=check_facts.SELF_TEST_CATALOGUE,
                                  sources_text=(root / check_facts.SOURCES).read_text(encoding="utf-8"),
                                  rows=check_facts.SELF_TEST_ROWS)
        run = Run(ctx, root / "cache", committed, check_heads=False)
        run.check()
        got = [w for _, _, w in run.findings]
        if ctx.presets or got != ["the value's literal 0.08 is in no quote serving adwaita (as written or converted)"]:
            failures.append(f"no presets: got {got} with presets {sorted(ctx.presets)}")
        buttons.write_text(buttons.read_text(encoding="utf-8").replace("0.08)", "0.07)"), encoding="utf-8")
        # an uncommitted evidence file, and the vacuous run
        run = Run(check_facts.fixture_context(root), root / "cache", set(), check_heads=False)
        run.check()
        if sum("is not committed" in w for _, _, w in run.findings) != 6:
            failures.append(f"uncommitted: got {run.findings}")
        root = base / "empty"
        check_facts.write_fixture(root, {})
        run = Run(check_facts.fixture_context(root), root / "cache", set(), check_heads=False)
        items = run.check()
        if run.findings or not run.summary(items).startswith("no facts yet: 0 facts"):
            failures.append(f"empty: {run.findings}, {run.summary(items)}")
        # the pin check: a checkout at its entry's commit, at another one, and none
        repo = base / "pins" / "libadwaita@1.10.0"
        repo.mkdir(parents=True)
        git = ["git", "-C", str(repo), "-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false"]
        sha = ""
        try:
            subprocess.run(git + ["init", "-q"], check=True, capture_output=True)
            subprocess.run(git + ["commit", "-q", "--allow-empty", "-m", "c"], check=True, capture_output=True)
            sha = subprocess.run(git + ["rev-parse", "HEAD"], check=True, capture_output=True, text=True).stdout.strip()
        except (OSError, subprocess.CalledProcessError) as err:
            failures.append(f"pin check: git: {err}")
        for sid, commit, want in (("libadwaita@1.10.0", sha, None), ("libadwaita@1.10.0", "0" * 40, "not at its pin"),
                                  ("gtk@4.24.1", sha, "has no checkout in the source cache")):
            got = Run(check_facts.fixture_context(root), base / "pins", set()).head_problem(sid, {"commit": commit})
            if (want is None) != (got is None) or (want and want not in got):
                failures.append(f"pin check {sid} at {commit[:8]}: got {got!r}, want {want!r}")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print(f"self-test ok ({len(SELF_TEST_NOTATIONS)} notation cases, {len(SELF_TEST_TEXTS)} text cases, "
              f"{len(SELF_TEST_CASES) + 1} fixture cases)")
    return 1 if failures else 0


def main(argv) -> int:
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    cache = None
    while args:
        arg = args.pop(0)
        if arg == "--cache" and args:
            cache = Path(args.pop(0))
        else:
            print(__doc__, file=sys.stderr)
            return 2
    cache = citations.cache_dir(cache)
    if not cache.is_dir():
        print(f"error: no source cache at {cache} (scripts/update_sources.sh fills it)", file=sys.stderr)
        return 2
    try:
        run = Run(check_facts.Context(Path(".")), cache)
        items = run.check()
    except (OSError, UnicodeDecodeError, check_facts.InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for f, ln, what in run.findings:
        print(f"{f}:{ln}: {what}")
    for f, ln, src in run.sdk:
        print(f"# sdk: {f}:{ln}: {src} (os-probe.yml's macOS job checks it at that Xcode)")
    print(run.summary(items), file=sys.stderr)
    return 1 if run.findings else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
