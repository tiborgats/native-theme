#!/usr/bin/env python3
"""The vector tables of the GNOME reader's two LGPL-derived helpers (the 0.8.0 plan's D10; specification S2.2
*Checked against their references*, S9.5): Pango's font-description parser and libadwaita's accent snapping.

Writes these tables under `native-theme/tests/vectors/`:

- `reader.pango_font_description.tsv`, by the `public_api` route: Pango's `pango_font_description_from_string`
  (`pango@1.58.2 pango/fonts.c:1613-1729`) called through the system's Pango 1.58.2, the pinned release (it refuses
  another), each result read through Pango's public getters. The inputs: the GNOME font settings' defaults
  (`gsettings-desktop-schemas@51.0`, read from the source cache), typical `font-name`, `document-font-name`,
  `monospace-font-name` and `titlebar-font` values, then every style word of Pango's tables, the `<field>=` forms,
  the size forms and the family list's and whitespace's edges.
- `reader.adw_accent_nearest.tsv`, by the `compiled_source` route: libadwaita's
  `adw_accent_color_nearest_from_rgba` (`libadwaita@1.10.0 src/adw-accent-color.c:159-199`) and the OKLab and OKLCH
  conversions it calls (`src/adw-color-utils.c`), compiled unchanged with `accent.c`, which calls it. The inputs:
  libadwaita's nine accents (`adw_accent_color_to_rgba`, the same build), Plasma's accent palette
  (`plasma-workspace@6.8.2 kcms/colors/ui/AccentColorUI.qml`), the portal's typical colours, then, for each hue
  threshold and the chroma threshold, adjacent pairs of `f32` colours the build snaps to different accents (found by
  bisection on one channel's bits with the harness), and seeded random colours. The reference build targets the
  x86-64 baseline (`-O2 -march=x86-64 -mtune=generic`, no fused multiply-add, the target of the x86-64 package
  `sources.toml`'s `arch` field names); a second build adds `-mfma -ffp-contract=fast`, and the header counts the
  rows on which the two differ.

The model's portal colour is three doubles (`org.freedesktop.appearance` `accent-color`, `(ddd)`); libadwaita takes
them into a `GdkRGBA`'s floats (`src/adw-settings-impl-portal.c:124-141`), so the table's inputs are `f32`s, written
as the shortest decimal that reads back as each.

The harnesses, the compiler and the linker run under bubblewrap with no network (`tools/pinned.py`), writing only the
build folder. Requires gcc, bubblewrap, `pkg-config`, Pango 1.58.2's and GTK 4's development headers. Run from the
repository root, the source cache's folder optional:

    python3 -I tools/gnome-reader-vectors/generate.py [source cache]
"""

import json
import math
import random
import re
import struct
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "tools"))
import citations  # noqa: E402
import pinned  # noqa: E402

VECTORS = ROOT / "native-theme" / "tests" / "vectors"
PANGO_VERSION = "1.58.2"
LIBADWAITA = "libadwaita@1.10.0"
SCHEMAS = "gsettings-desktop-schemas@51.0"
PLASMA = "plasma-workspace@6.8.2"
SEED = 20261106
REFERENCE_FLAGS = ["-O2", "-march=x86-64", "-mtune=generic"]
FUSED_FLAGS = REFERENCE_FLAGS + ["-mfma", "-ffp-contract=fast"]
ADW_FILES = ["src/adw-accent-color.c", "src/adw-accent-color.h", "src/adw-accent-color-private.h",
             "src/adw-color-utils.c", "src/adw-color-utils-private.h"]
ADW_COMPILED = ["src/adw-accent-color.c", "src/adw-color-utils.c"]
ACCENTS = ["blue", "teal", "green", "yellow", "orange", "red", "pink", "purple", "slate"]


def pkg_version(name):
    out = subprocess.run(["pkg-config", "--modversion", name], capture_output=True, text=True, check=False)
    return out.stdout.strip()


def pkg_flags(*args):
    out = subprocess.run(["pkg-config", *args], capture_output=True, text=True, check=True)
    return out.stdout.split()


def write_table(path: Path, header, columns, rows):
    lines = [*header, "\t".join(columns), *("\t".join(row) for row in rows)]
    path.write_text("".join(line + "\n" for line in lines), encoding="utf-8")
    print(f"{path.name}: {len(rows)} rows")


def string_cell(s):
    """A `string` cell: the JSON string literal of `s` (the proposed column type: any string, the empty one,
    surrounding whitespace and control characters included)."""
    return json.dumps(s, ensure_ascii=False)


def option(cell):
    return "none" if cell is None else cell


# ---- Pango's font-description parser

# typical gsettings font values (org.gnome.desktop.interface font-name, document-font-name, monospace-font-name;
# org.gnome.desktop.wm.preferences titlebar-font), as distributions and users set them
TYPICAL_FONTS = [
    "Cantarell 11", "Cantarell Bold 11", "Cantarell 15px", "Cantarell Bold Italic 11", "Source Code Pro 10",
    "Monospace 11", "Sans 10", "Noto Sans 10", "Noto Sans Bold 10", "Noto Sans Mono 10", "Noto Sans CJK JP 10",
    "DejaVu Sans 9", "DejaVu Sans Mono 10", "DejaVu Sans Mono Book 10", "DejaVu Sans Bold 9", "Ubuntu 11",
    "Ubuntu Bold 11", "Ubuntu Mono 13", "Ubuntu Sans 11", "Ubuntu Sans Medium 11", "Ubuntu Sans Mono 13",
    "Fira Sans Heavy 14", "Fira Code Retina 10", "Inter 11", "Inter Variable 10.5", "Inter Display Semi-Bold 12",
    "JetBrains Mono 10", "Liberation Sans 9", "Roboto Medium 10", "Adwaita Sans 10.5", "Adwaita Sans Italic 11",
    "Cantarell Italic Light 15 @wght=200 #tnum=1", "Inter 11 @wght=450", "Inter 11 #tnum,ss01",
    "Inter 11 #tnum=1,cv05=1 @opsz=32", "Recursive Sans Linear 11 @MONO=0.5,CASL=1",
]
# every word of Pango's tables (fonts.c:1294-1361), each after a family and before a size, and the bare `Normal`
STYLE_WORDS = [
    "Normal", "Roman", "Oblique", "Italic",
    "Small-Caps", "All-Small-Caps", "Petite-Caps", "All-Petite-Caps", "Unicase", "Title-Caps",
    "Thin", "Ultra-Light", "Extra-Light", "Light", "Semi-Light", "Demi-Light", "Book", "Regular", "Medium",
    "Semi-Bold", "Demi-Bold", "Bold", "Ultra-Bold", "Extra-Bold", "Heavy", "Black", "Ultra-Heavy", "Extra-Heavy",
    "Ultra-Black", "Extra-Black",
    "Ultra-Condensed", "Extra-Condensed", "Condensed", "Semi-Condensed", "Semi-Expanded", "Expanded",
    "Extra-Expanded", "Ultra-Expanded",
    "Not-Rotated", "South", "Upside-Down", "North", "Rotated-Left", "East", "Rotated-Right", "West",
    "With-Color", "Without-Color",
]
# how a word matches: ASCII case ignored, a hyphen of the table's word optional, nothing else
WORD_FORMS = [
    "Sans bold 11", "Sans BOLD 11", "Sans SemiBold 11", "Sans semibold 11", "Sans Semi-bold 11", "Sans SEMI-BOLD 11",
    "Sans Semi Bold 11", "Sans Semi--Bold 11", "Sans -Bold 11", "Sans Bold- 11", "Sans Bol 11", "Sans Bolder 11",
    "Sans UltraBlack 11", "Sans Ultra 11", "Sans smallcaps 11", "Sans NORMAL 11", "Sans normal 11", "Sans Nor-mal 11",
    "Sans Italic Bold 11", "Sans Bold Light 11", "Sans Light Bold 11", "Sans Italic Oblique 11",
    "Sans Bold Normal Italic 11", "Sans Bold Mono 11", "Noto Sans Bold Mono 10",
    "Sans Bold Italic Condensed Small-Caps Rotated-Left With-Color 11",
    "Sans Bold,Italic 11", "Sans Bold ,Italic 11", "Sans,Bold 11", "Bold 11", "Bold", "Bold Italic", "Sans Bold",
    "Sans 11 Bold", "Italic 11", "Normal 11", "Normal",
]
# the `<field>=` forms (find_field, fonts.c:1411-1447): the prefix case-sensitive, then a table word or an integer
FIELD_FORMS = [
    "Sans weight=650 11", "Sans weight=Bold 11", "Sans weight=bold 11", "Sans weight=semibold 11",
    "Sans weight=Normal 11", "Sans weight=Regular 11", "Sans weight=0 11", "Sans weight=1000 11",
    "Sans weight=5000 11", "Sans weight=+700 11", "Sans weight=-1 11", "Sans weight= 11", "Sans weight=7x 11",
    "Sans weight=0x10 11", "Sans weight=2147483647 11", "Sans weight=2147483648 11", "Sans Weight=700 11",
    "Sans weight 11", "Sans weight=Italic 11", "Sans style=italic 11", "Sans style=Oblique 11", "Sans style=1 11",
    "Sans style=2 11", "Sans style=5 11", "Sans style=Bold 11", "Sans variant=small-caps 11", "Sans variant=3 11",
    "Sans width=Condensed 11", "Sans width=condensed 11", "Sans width=750 11", "Sans width=1 11",
    "Sans width=3000 11", "Sans stretch=Condensed 11", "Sans gravity=east 11", "Sans gravity=2 11",
    "Sans color=With-Color 11", "Sans color=1 11", "Sans weight=650 style=italic 11",
]
# the size (parse_size, fonts.c:1495-1519)
SIZE_FORMS = [
    "Sans 11", "Sans 11.5", "Sans 10.3", "Sans 10.25", "Sans 0.0001", "Sans 0.0004", "Sans 0.0005",
    "Sans 7.00048828125", "Sans 7.000488281", "Sans 11px", "Sans 11.5px", "Sans 11PX", "Sans 11Px", "Sans 11 px",
    "Sans 11pt", "Sans px", "Sans .5", "Sans 5.", "Sans .", "Sans +11", "Sans -11", "Sans -0", "Sans 0", "Sans 0px",
    "Sans 00011", "Sans 1e1", "Sans 1E1px", "Sans 1e", "Sans 1e+1", "Sans 1000000", "Sans 1000000.0001",
    "Sans 1e6", "Sans 1e7", "Sans 999999.9999", "Sans inf", "Sans nan", "Sans infinity", "Sans 0x10",
    "Sans 0x1p3px", "Sans 0x", "Sans 1,5", "Sans 1.5.5", "Sans 11 11", "11", "11px", "Sans11",
]
# the family list, the variations and features words, and whitespace (getword, fonts.c:1478-1493; the trim and
# split, :1696-1726)
FAMILY_FORMS = [
    "Cantarell, Sans 11", "Cantarell,Sans 11", " Cantarell , Sans , 11", "Cantarell,Sans,11", "Cantarell,Sans,",
    "Sans,,Serif 10", "Sans, ,Serif 10", ",Sans 10", "Sans, 10", "Sans,10", "Sans,, 10", ", 11", ",, 11", ",",
    "Sans , Bold 11", "'Cantarell' 11", "\"Cantarell\" 11", "Sans 11", "Sans  11", "Noto Sans 日本語 10",
    "  Sans   11  ", "\tSans\t11\t", "Sans\n11", "Sans\r\n11", "Sans\f11", "Sans\v11", "Sans\v 11", "", " ",
    "\t", "Sans", "Sans Serif", "@wght=200", "#tnum", "Sans @wght=200", "Sans #tnum", "Sans 11 #tnum @wght=200",
    "Sans 11 @wght=200 #tnum", "Sans @a=1 @b=2", "Sans #a #b", "Sans @wght=200,wdth=50 #tnum",
    "Sans 11 @wght=200,wdth=50", "Sans 11,@wght=1", "Sans,#tnum", "Sans 11#tnum", "Sans 11 #", "Sans 11 @",
    "Sans 11 # tnum", "Sans Bold 11 @wght=200 #tnum=1 extra", "Sans #tnum 11",
    # strtod and strtol skip leading C white space, of which only a vertical tab can be inside a word
    "Sans \v11", "Sans \v\v+11px", "Sans weight=\v700 11", "Sans weight=\v-0 11",
]


def schema_defaults(cache: Path):
    """The defaults of the four font keys, as gsettings-desktop-schemas states them."""
    found = []
    for path, keys in [("schemas/org.gnome.desktop.interface.gschema.xml.in",
                        ["font-name", "document-font-name", "monospace-font-name"]),
                       ("schemas/org.gnome.desktop.wm.preferences.gschema.xml.in", ["titlebar-font"])]:
        text = pinned.pinned(cache, SCHEMAS, path).decode("utf-8")
        for key in keys:
            m = re.search(rf'<key name="{key}" type="s">\s*<default>\'([^\']*)\'</default>', text)
            if m is None:
                sys.exit(f"{SCHEMAS} {path}: no default for {key}")
            found.append(m.group(1))
    return found


def run_pango(exe: Path, inputs):
    text = "".join(("=" if s == "" else s.encode("utf-8").hex()) + "\n" for s in inputs)
    out = pinned.sandboxed([exe], readable=[exe.parent], input=text, capture_output=True, text=True,
                           check=True).stdout.splitlines()
    if len(out) != len(inputs):
        sys.exit(f"{exe.name}: {len(out)} results for {len(inputs)} inputs")
    return [line.split("\t") for line in out]


def unhex(cell):
    if cell == "-":
        return None
    if cell == "=":
        return ""
    return bytes.fromhex(cell).decode("utf-8")


MASK_FAMILY, MASK_SIZE, MASK_GRAVITY, MASK_VARIATIONS, MASK_FEATURES, MASK_COLOR = 1, 1 << 5, 1 << 6, 1 << 7, \
    1 << 8, 1 << 9


def pango_row(s, r):
    mask = int(r[0])
    family, variations, features = unhex(r[1]), unhex(r[10]), unhex(r[11])
    # what the mask says and what the getters answer agree (the harness's view of the description)
    assert (family is not None) == bool(mask & MASK_FAMILY), (s, r)
    assert (variations is not None) == bool(mask & MASK_VARIATIONS), (s, r)
    assert (features is not None) == bool(mask & MASK_FEATURES), (s, r)
    return [
        string_cell(s),
        option(None if family is None else string_cell(family)),
        r[2], r[3], r[4], r[5],
        option(r[6] if mask & MASK_GRAVITY else None),
        option(r[7] if mask & MASK_COLOR else None),
        option(r[8] if mask & MASK_SIZE else None),
        "true" if r[9] == "1" else "false",
        option(None if variations is None else string_cell(variations)),
        option(None if features is None else string_cell(features)),
    ]


def pango_table(cache: Path, work: Path):
    exe = work / "pango"
    pinned.sandboxed(["gcc", "-std=gnu11", "-O2", "-o", exe, HERE / "pango.c", *pkg_flags("--cflags", "--libs",
                                                                                            "pango")],
                     writable=[work], check=True)
    defaults = schema_defaults(cache)
    seen = set()
    inputs = []
    for s in [*defaults, *TYPICAL_FONTS, *(f"Sans {w} 11" for w in STYLE_WORDS), *WORD_FORMS, *FIELD_FORMS,
              *SIZE_FORMS, *FAMILY_FORMS]:
        if s not in seen:
            seen.add(s)
            inputs.append(s)
    results = run_pango(exe, inputs)
    write_table(
        VECTORS / "reader.pango_font_description.tsv",
        ["# function: reader.pango_font_description",
         "# route: public_api",
         "# source: pango@1.58.2",
         "# source: glib@2.90.1",
         "# generations: adwaita",
         "# generator: tools/gnome-reader-vectors",
         "# inputs: s:string",
         "# outputs: family:option<string> style:number variant:number weight:number width:number "
         "gravity:option<number> color:option<number> size:option<number> absolute:bool variations:option<string> "
         "features:option<string>",
         f"# written by tools/gnome-reader-vectors/generate.py on Pango {pkg_version('pango')} and GLib "
         f"{pkg_version('glib-2.0')}; do not edit.",
         "# Each row: pango_font_description_from_string(s) read through Pango's getters. A string cell is a JSON",
         "# string literal. `none`: the field is not in pango_font_description_get_set_fields (style, variant,",
         "# weight and width always are). style, variant, weight, width, gravity and color are Pango's integers",
         "# (PangoStyle, PangoVariant, PangoWeight, PangoWidth, PangoGravity, PangoFontColor); size is in Pango",
         "# units (1/1024 of a point, or of a pixel where absolute is true). The inputs: the four GNOME font",
         "# settings' defaults (gsettings-desktop-schemas@51.0), typical values, every word of Pango's tables, the",
         "# word, field, size, family-list and whitespace forms."],
        ["s", "family", "style", "variant", "weight", "width", "gravity", "color", "size", "absolute", "variations",
         "features"],
        [pango_row(s, r) for s, r in zip(inputs, results)],
    )


# ---- libadwaita's accent snapping

def f32(x):
    return struct.unpack("f", struct.pack("f", x))[0]


def f32_text(x):
    """The shortest decimal that reads back as the `f32` `x`."""
    for digits in range(1, 18):
        text = f"{x:.{digits}g}"
        if f32(float(text)) == x:
            return text
    return repr(x)


def bits(x):
    return struct.unpack("<I", struct.pack("<f", x))[0]


def from_bits(b):
    return struct.unpack("<f", struct.pack("<I", b))[0]


def approx_oklch(r, g, b):
    """OKLCH of an sRGB colour in double precision (Björn Ottosson's published OKLab matrices), only to pick
    candidates near a threshold; every row's accent is the harness's."""
    def linear(v):
        return v / 12.92 if v < 0.04045 else ((v + 0.055) / 1.055) ** 2.4
    lr, lg, lb = linear(r), linear(g), linear(b)
    l = (0.4122214708 * lr + 0.5363325363 * lg + 0.0514459929 * lb) ** (1 / 3)
    m = (0.2119034982 * lr + 0.6806995451 * lg + 0.1073969566 * lb) ** (1 / 3)
    s = (0.0883024619 * lr + 0.2817188376 * lg + 0.6299787005 * lb) ** (1 / 3)
    a = 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s
    bb = 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s
    return math.hypot(a, bb), math.degrees(math.atan2(bb, a)) % 360


def build_accent(cache: Path, work: Path, flags, name):
    src = cache / LIBADWAITA / "src"
    includes = [f"-I{p}" for p in [HERE / "stubs", src, Path("/usr/include/libadwaita-1")]]
    exe = work / name
    pinned.sandboxed(["gcc", "-std=gnu11", *flags, "-DADWAITA_COMPILATION", *includes, *pkg_flags("--cflags", "gtk4"),
                      "-o", exe, HERE / "accent.c", *(src / Path(p).name for p in ADW_COMPILED),
                      *pkg_flags("--libs", "gtk4"),
                      "-lm"], writable=[work], check=True)
    return exe


def run_accent(exe: Path, calls):
    text = "".join(c + "\n" for c in calls)
    out = pinned.sandboxed([exe], readable=[exe.parent], input=text, capture_output=True, text=True,
                           check=True).stdout.splitlines()
    if len(out) != len(calls):
        sys.exit(f"{exe.name}: {len(out)} results for {len(calls)} calls")
    return out


def nearest(exe, colours):
    return [int(x) for x in run_accent(exe, ["nearest " + " ".join(f32_text(v) for v in c) for c in colours])]


def hex_colour(h):
    return tuple(f32(int(h[i:i + 2], 16) / 255) for i in (1, 3, 5))


def plasma_palette(cache: Path):
    text = pinned.pinned(cache, PLASMA, "kcms/colors/ui/AccentColorUI.qml").decode("utf-8")
    found = re.findall(r'"(#[0-9a-fA-F]{6})"', text)
    if len(found) < 9:
        sys.exit(f"{PLASMA} kcms/colors/ui/AccentColorUI.qml: {len(found)} accent colours")
    return found


def bisect_pairs(exe, starts):
    """For each (colour, channel, lo, hi) whose ends snap differently, the adjacent f32 pair on that channel
    between them that does: bisection on the channel's bit pattern (monotonic for non-negative floats)."""
    states = []
    for colour, k, lo, hi in starts:
        states.append([colour, k, bits(lo), bits(hi)])
    while True:
        active = [s for s in states if s[3] - s[2] > 1]
        if not active:
            break
        mids = []
        for colour, k, lo, hi in active:
            c = list(colour)
            c[k] = from_bits((lo + hi) // 2)
            mids.append(tuple(c))
        ends = []
        for colour, k, lo, _ in active:
            c = list(colour)
            c[k] = from_bits(lo)
            ends.append(tuple(c))
        got_mid, got_lo = nearest(exe, mids), nearest(exe, ends)
        for s, m, l in zip(active, got_mid, got_lo):
            middle = (s[2] + s[3]) // 2
            if m == l:
                s[2] = middle
            else:
                s[3] = middle
    pairs = []
    for colour, k, lo, hi in states:
        a, b = list(colour), list(colour)
        a[k], b[k] = from_bits(lo), from_bits(hi)
        pairs.append((tuple(a), tuple(b)))
    return pairs


HUE_THRESHOLDS = [10, 35, 75.5, 115, 175, 230, 280, 345]
CHROMA_THRESHOLD = 0.04
PER_THRESHOLD = 3
RANDOM_BYTE_ROWS = 30
RANDOM_FLOAT_ROWS = 20
FUSED_SEARCH = 20_000


def accent_table(cache: Path, source_cache: Path, work: Path, rng):
    """`cache` holds the materialised libadwaita files, `source_cache` is the source cache (Plasma's palette)."""
    reference = build_accent(cache, work, REFERENCE_FLAGS, "accent")
    fused = build_accent(cache, work, FUSED_FLAGS, "accent-fused")

    named = [tuple(f32(float(x)) for x in line.split())
             for line in run_accent(reference, [f"named {i}" for i in range(9)])]
    fixed = [(c, f"libadwaita's {ACCENTS[i]}") for i, c in enumerate(named)]
    fixed += [(hex_colour(h), f"Plasma's {h}") for h in plasma_palette(source_cache)]
    # the KDE accent evidence's two (Breeze's default #3daee9 is in the palette), other desktops' accents, extremes
    for h in ["#5e2ca5", "#e5a50a", "#0078d4", "#007aff", "#e95420", "#ffffff", "#000000", "#808080",
              "#ff0000", "#00ff00", "#0000ff"]:
        fixed.append((hex_colour(h), h))

    # candidates near each threshold (by the double-precision estimate), then a channel whose ±0.01 crosses it
    samples = [tuple(f32(rng.random()) for _ in range(3)) for _ in range(200_000)]
    estimates = [approx_oklch(*c) for c in samples]
    starts = []
    for t in HUE_THRESHOLDS:
        near = [c for c, (ch, h) in zip(samples, estimates) if ch > 0.06 and abs(h - t) < 0.3][:PER_THRESHOLD * 4]
        starts += [(c, f"hue {t}") for c in near]
    near = [c for c, (ch, h) in zip(samples, estimates) if abs(ch - CHROMA_THRESHOLD) < 0.0005][:PER_THRESHOLD * 6]
    starts += [(c, "chroma 0.04") for c in near]
    trials = []
    for c, why in starts:
        for k in range(3):
            lo, hi = f32(max(0.0, c[k] - 0.01)), f32(min(1.0, c[k] + 0.01))
            a, b = list(c), list(c)
            a[k], b[k] = lo, hi
            trials.append((c, k, lo, hi, tuple(a), tuple(b), why))
    got_a = nearest(reference, [t[4] for t in trials])
    got_b = nearest(reference, [t[5] for t in trials])
    chosen, taken = [], {}
    for t, x, y in zip(trials, got_a, got_b):
        if x != y and taken.get(t[6], 0) < PER_THRESHOLD and not any(t[0] is c for c, *_ in chosen):
            chosen.append(t)
            taken[t[6]] = taken.get(t[6], 0) + 1
    pairs = bisect_pairs(reference, [(c, k, lo, hi) for c, k, lo, hi, *_ in chosen])
    boundary = [(colour, why) for (a, b), t in zip(pairs, chosen) for colour, why in [(a, t[6]), (b, t[6])]]

    randoms = [(tuple(f32(rng.randrange(256) / 255) for _ in range(3)), "random byte")
               for _ in range(RANDOM_BYTE_ROWS)]
    randoms += [(tuple(f32(rng.random()) for _ in range(3)), "random") for _ in range(RANDOM_FLOAT_ROWS)]

    rows = fixed + boundary + randoms
    colours = [c for c, _ in rows]
    ref, fus = nearest(reference, colours), nearest(fused, colours)
    differing = sum(1 for x, y in zip(ref, fus) if x != y)
    search = [tuple(f32(rng.random()) for _ in range(3)) for _ in range(FUSED_SEARCH)]
    search_differing = sum(1 for x, y in zip(nearest(reference, search), nearest(fused, search)) if x != y)
    missing = sorted(set(HUE_THRESHOLDS) - {float(w.split()[1]) for w in taken if w.startswith("hue")})
    if missing or "chroma 0.04" not in taken:
        sys.exit(f"no boundary pair found for {missing or 'the chroma threshold'}")
    write_table(
        VECTORS / "reader.adw_accent_nearest.tsv",
        ["# function: reader.adw_accent_nearest",
         "# route: compiled_source",
         "# source: libadwaita@1.10.0",
         "# generations: adwaita",
         "# generator: tools/gnome-reader-vectors",
         "# inputs: red:number green:number blue:number",
         "# outputs: accent:enum<AdwAccentColor>",
         "# written by tools/gnome-reader-vectors/generate.py; do not edit. Each row:",
         "# adw_accent_color_nearest_from_rgba of the GdkRGBA (red, green, blue, 1), the channels f32s, its",
         "# AdwAccentColor in snake_case. The inputs: libadwaita's nine accents, Plasma's accent palette and other",
         f"# typical colours ({len(fixed)} rows); for each hue threshold and the chroma threshold, adjacent f32",
         f"# colours on one channel that snap to different accents ({len(boundary)} rows); seeded random colours",
         f"# ({len(randoms)}). A build fusing multiply-adds snaps {differing} of these rows and {search_differing}",
         f"# of {FUSED_SEARCH} random colours differently."],
        ["red", "green", "blue", "accent"],
        [[f32_text(c[0]), f32_text(c[1]), f32_text(c[2]), ACCENTS[a]] for c, a in zip(colours, ref)],
    )


def main():
    if pkg_version("pango") != PANGO_VERSION:
        sys.exit(f"Pango {pkg_version('pango') or '(none)'} found; the pin is Pango {PANGO_VERSION}")
    source_cache = citations.cache_dir(sys.argv[1] if len(sys.argv) > 1 else None)
    rng = random.Random(SEED)
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        cache = tmp / "sources"
        pinned.materialise(source_cache, [(LIBADWAITA, p) for p in ADW_FILES], cache)
        pango_table(source_cache, tmp)
        accent_table(cache, source_cache, tmp, rng)


if __name__ == "__main__":
    main()
