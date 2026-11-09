#!/usr/bin/env python3
"""The vector tables of the KDE platform functions (specification S2.2 *Checked against their references*), by the
`compiled_source` route: the pinned upstream sources compiled unchanged into a harness that calls them.

Writes these tables under `native-theme/tests/vectors/`:

- `kirigami_lerp.tsv`, `kirigami_lerp(a, b, balance)`: Kirigami's `ColorUtils::linearInterpolation`
  (`kirigami@6.31.0 src/platform/colorutils.cpp:61-80`) over Qt's `QColor` (`qtbase@6.12.0
  src/gui/painting/qcolor.cpp`);
- `kde_shade.tsv`, `kde_shade(c, role, contrast)`: `KColorScheme::shade(color, role, contrast)` with no chroma
  adjustment (`kcolorscheme@6.31.0 src/kcolorscheme.cpp:566-616`), the contrast step scaled as `KColorScheme::contrastF`
  scales it (`:526`), over KColorUtils' `shade` and `darken` and KHCY (`kguiaddons@6.31.0 src/colors/kcolorutils.cpp`,
  `src/colors/kcolorspaces.cpp`) and Qt's `QColor`;
- `kcu_mix.tsv`, `kcu_tint.tsv`, `kcu_shade.tsv`, `kcu_overlay.tsv`: `KColorUtils::mix`, `tint`, `shade` and
  `overlayColors` (`kguiaddons@6.31.0 src/colors/kcolorutils.cpp`); `overlayColors` paints with the system's Qt
  6.12.0 `QPainter` (its raster engine is not compiled from the pin);
- `kirigami_tint.tsv`: Kirigami's `ColorUtils::tintWithAlpha` (`colorutils.cpp:230-245`);
- `kde_effects.tsv`: KColorScheme's `StateEffects` (`kcolorscheme.cpp:22-97`) over an in-memory config holding the
  effect tokens, applied as `KColorSchemePrivate::initFromConfig` applies them to a foreground, decoration or
  background role (`:380-391`);
- `kde_selection_inactive.tsv`: the Selection set's Inactive start, `KColorScheme(QPalette::Inactive, Selection,
  config)` (`:304-321`, `:374-378`) over an in-memory config with the inactive effects disabled.
- `reader.qfont_from_string.tsv`, by the `public_api` route: the KDE reader's `QFont::fromString` (`qtbase@6.12.0
  src/gui/text/qfont.cpp:2246-2320`) called through the system's Qt 6.12.0 by `qfont.cpp`, on the strings Qt 6.12
  writes (`QFont::toString`) for Plasma's default fonts (`plasma-integration@6.8.2
  qt6/src/platformtheme/kfontsettingsdata.cpp`, read from the source cache) and typical choices, Qt 5's and earlier
  Qt 6 releases' strings, and each field count's and field's edge cases; each result read through `QFont`'s getters
  and `resolveMask()`. A string cell is a JSON string literal.
- `reader.kde_color_scheme.tsv`, by the `public_api` route: the KDE reader's scheme test, Plasma's own
  (`plasma-integration@6.8.2 qt6/src/platformtheme/khintssettings.cpp:469-485`, `xdg-desktop-portal-kde@6.8.2
  src/settings.cpp:259-273`): kdeglobals' `[Colors:Window] BackgroundNormal` written by the system's KConfig 6.31,
  the palette `KColorScheme::createApplicationPalette` (KColorScheme 6.31) builds from it, and Qt's `qGray` of its
  window colour against the threshold read from both pinned files (`readers.cpp`); each row checked against the
  system's plasma-integration platform theme, `QStyleHints::colorScheme()` over the same kdeglobals.
- `reader.qxcb_xft_dpi.tsv`, by the `public_api` route: the DPI Qt's xcb plugin (`qtbase@6.12.0
  src/plugins/platforms/xcb/qxcbscreen.cpp:298-314`, `:366-373`, `:856-876`) forces from the X resources, the
  system's Qt 6.12.0 run against an Xvfb server whose root window holds them (`readers.cpp`), each input also read
  back through `xrdb -query`, which the KDE reader reads.
- `kde_accent.tsv` and `kde_accent_wm.tsv`, by the `compiled_source` route: Plasma's colour scheme applier
  (`plasma-workspace@6.8.2 kcms/colors/colorsapplicator.cpp` and `.h`, compiled unchanged with `accent.cpp`) applied to
  generated scheme files and accents, each case's kdeglobals read back through the system's KColorScheme 6.31 as the
  KDE reader reads kdeglobals; the measured (scheme file, accent) pairs of `docs/facts/kde/accent.toml` re-run
  first, the run stopping on any fact that differs.
- `kirigami_label_icon_size.tsv`, by the `compiled_source` route: Kirigami's `IconSizes::sizeForLabels` and
  `roundedIconSize` (`kirigami@6.31.0 src/platform/units.cpp:297-326`, compiled unchanged with `icon.cpp` and the
  moc output of `units.h`, under Kirigami's static-build define), on the offscreen platform: the application font set
  to Noto Sans or Hack (Plasma's default and fixed-width fonts, which must be installed) at swept, Plasma's and seeded
  random point sizes, each height that font's `QFontMetricsF` height in the system's Qt 6.12.0, and integer heights
  called directly; the facts' input, Noto Sans 10 at 96 DPI, 17.6875 px giving 16 (ADD-A141), checked first, and each
  font's result checked against `roundedIconSize` of its truncated height.

The functions' inputs: the Breeze 6.8.2 colour schemes' colours (`breeze@6.8.2 colors/BreezeLight.colors`,
`BreezeDark.colors`, read from the source cache) at the arguments the facts pass (`docs/facts/kde/`), the branches'
boundaries, then seeded random inputs. The model's colours are f32 channels and its numbers f32: each enters Qt as
`QColor::fromRgbF` takes the channels and as the f32 widened to the function's double.

Each file is read from the source cache (`scripts/update_sources.sh`) as the commit `docs/facts/sources.toml` pins
holds it, through git's objects (`tools/pinned.py`), and compiled as it is; `stubs/` stands in for the headers the
projects' builds generate (export and logging macros) and `harness.cpp` calls the functions. `moc`, the compiler,
the linker and both harnesses run under bubblewrap with no network, writing only the build folder (the harnesses
nothing: they print their results). The reference build targets the x86-64 baseline (`-O2 -march=x86-64
-mtune=generic`), the target of the x86-64 packages `sources.toml`'s `arch` fields name, which has no fused
multiply-add, so `QColor::toRgb`'s `1.0f - (s * f)` is rounded twice. A second build adds `-mfma` (a build that fuses
it, as an x86-64-v3 or an ARM build may); the table holds inputs on which the two differ, so a port that fuses fails a
row.

The remaining sources Qt's and KDE's headers need are the system's: it requires g++, bubblewrap, Qt 6.12.0's
development headers (private ones included) and `moc`, and the KF6 Config 6.31 headers and libraries; it refuses
another Qt or KConfig. The readers' two tables also need the KF6 ColorScheme 6.31 headers and library, the
plasma-integration platform theme, Qt's xcb plugin, Xvfb and xrdb; the accent's two the KF6 GuiAddons and ColorScheme
6.31 headers and libraries and QtDBus's headers. Run from the repository root, the source cache's folder optional
(`--only-readers`: the readers' two tables alone; `--only-accent`: the accent's two alone; `--only-icon`:
`kirigami_label_icon_size.tsv` alone, which needs Qt's QML headers and the Noto Sans and Hack fonts):

    python3 -I tools/kde-vectors/generate.py [--only-readers | --only-accent | --only-icon] [source cache]
"""

import json
import random
import re
import struct
import subprocess
import sys
import tempfile
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "tools"))
import citations  # noqa: E402
import pinned  # noqa: E402

VECTORS = ROOT / "native-theme" / "tests" / "vectors"
QT_VERSION = "6.12.0"
# KConfig's minor, kconfig@6.31.0's: the headers' KCONFIG_VERSION_STRING.
KCONFIG_MINOR = "6.31"
SEED = 20261030
RANDOM_INPUTS = 20_000
# The x86-64 baseline: no fused multiply-add.
REFERENCE_FLAGS = ["-O2", "-march=x86-64", "-mtune=generic"]
FUSED_FLAGS = REFERENCE_FLAGS + ["-mfma", "-ffp-contract=fast"]

# Each source's sources.toml id and the files read from it.
FILES = {
    "qtbase@6.12.0": ["src/gui/painting/qcolor.cpp", "src/gui/painting/qcolor_p.h", "src/gui/painting/qdrawhelper_p.h"],
    "kirigami@6.31.0": ["src/platform/colorutils.cpp", "src/platform/colorutils.h"],
    "kcolorscheme@6.31.0": ["src/kcolorscheme.cpp", "src/kcolorscheme.h", "src/kcolorschemehelpers_p.h",
                            "src/kcolorschememanager_p.h", "src/kcolorschememodel.h"],
    "kguiaddons@6.31.0": ["src/colors/kcolorutils.cpp", "src/colors/kcolorutils.h", "src/colors/kcolorspaces.cpp",
                          "src/colors/kcolorspaces_p.h", "src/colors/kguiaddons_colorhelpers_p.h"],
}
COMPILED = [
    ("qtbase@6.12.0", "src/gui/painting/qcolor.cpp"),
    ("kirigami@6.31.0", "src/platform/colorutils.cpp"),
    ("kcolorscheme@6.31.0", "src/kcolorscheme.cpp"),
    ("kguiaddons@6.31.0", "src/colors/kcolorutils.cpp"),
    ("kguiaddons@6.31.0", "src/colors/kcolorspaces.cpp"),
]
QT_INCLUDE = Path("/usr/include/qt6")
KF6_INCLUDE = Path("/usr/include/KF6")
MOC = Path("/usr/lib/qt6/moc")
LIBS = ["-lQt6Quick", "-lQt6Qml", "-lQt6Gui", "-lQt6Core", "-lKF6ConfigCore", "-lKF6ConfigGui"]

# Breeze 6.8.2's inputs (BreezeLight.colors, BreezeDark.colors): the Header, Window and View sets' background and
# text at Kirigami's frame contrast 0.2.
BREEZE_PAIRS = [
    ((222, 224, 226), (35, 38, 41), 0.2),
    ((239, 240, 241), (35, 38, 41), 0.2),
    ((255, 255, 255), (35, 38, 41), 0.2),
    ((41, 44, 48), (252, 252, 252), 0.2),
    ((32, 35, 38), (252, 252, 252), 0.2),
    ((20, 22, 24), (252, 252, 252), 0.2),
]
# linearInterpolation takes the other colour's hue where one's is not above 0: a grey (hue undefined, -1) and a pure
# red (hue 0); and its balance's ends.
HUE_EDGES = [
    ((128, 128, 128), (61, 174, 233), 0.2),
    ((61, 174, 233), (128, 128, 128), 0.2),
    ((128, 128, 128), (200, 200, 200), 0.5),
    ((255, 0, 0), (61, 174, 233), 0.3),
    ((61, 174, 233), (255, 0, 0), 0.3),
    ((255, 0, 0), (0, 0, 0), 0.7),
    ((61, 174, 233), (35, 38, 41), 0.0),
    ((61, 174, 233), (35, 38, 41), 1.0),
]
# Alpha interpolates as the channels do.
ALPHA_PAIRS = [
    ((255, 0, 0, 0), (255, 0, 0, 255), 0.25),
    ((61, 174, 233, 128), (35, 38, 41, 255), 0.6),
]
# KColorScheme::shade's three branches by the colour's luma (a very dark colour, a very light one, the rest):
# Breeze Light's and Dark's window backgrounds, the ends, a saturated blue.
SHADE_COLORS = [(239, 240, 241), (32, 35, 38), (0, 0, 0), (3, 3, 3), (255, 255, 255), (250, 250, 250),
                (61, 174, 233), (128, 128, 128)]
# kdeglobals' contrast steps: Breeze's 4, the ends 0 and 10, one above the end and one below zero (clamped).
CONTRASTS = [4.0, 0.0, 7.0, 10.0, 12.0, -3.0]
SHADE_ROLES = ["light", "midlight", "mid", "dark", "shadow"]
RANDOM_LERP = 24
RANDOM_SHADE = 20
FUSED_ROWS = 8


def f32(x):
    """`x` rounded to an `f32`: the model's numbers are `f32`, and a function takes them as such."""
    return struct.unpack("f", struct.pack("f", x))[0]


def f32_text(x):
    """The shortest decimal that reads back as the `f32` `x`."""
    for digits in range(1, 18):
        text = f"{x:.{digits}g}"
        if f32(float(text)) == x:
            return text
    return repr(x)


def qt_version():
    out = subprocess.run(["pkg-config", "--modversion", "Qt6Gui"], capture_output=True, text=True, check=False)
    return out.stdout.strip()


def kconfig_version():
    """The KF6 Config headers' `KCONFIG_VERSION_STRING`; empty where they are missing."""
    try:
        header = (KF6_INCLUDE / "KConfig" / "kconfig_version.h").read_text(encoding="utf-8")
    except OSError:
        return ""
    found = re.search(r'#define KCONFIG_VERSION_STRING "([^"]+)"', header)
    return found.group(1) if found else ""


def build(cache: Path, work: Path, flags, name):
    """The harness, the pinned sources compiled with `flags`, sandboxed."""
    platform = cache / "kirigami@6.31.0" / "src" / "platform"
    moc = pinned.sandboxed([MOC, platform / "colorutils.h"], readable=[cache], capture_output=True, text=True,
                           check=True).stdout
    (work / "moc_colorutils.cpp").write_text(moc, encoding="utf-8")
    qt = [QT_INCLUDE, QT_INCLUDE / "QtCore", QT_INCLUDE / "QtGui", QT_INCLUDE / "QtQml", QT_INCLUDE / "QtQuick",
          QT_INCLUDE / "QtCore" / QT_VERSION, QT_INCLUDE / "QtCore" / QT_VERSION / "QtCore",
          QT_INCLUDE / "QtCore" / QT_VERSION / "QtCore" / "private",
          QT_INCLUDE / "QtGui" / QT_VERSION, QT_INCLUDE / "QtGui" / QT_VERSION / "QtGui",
          QT_INCLUDE / "QtGui" / QT_VERSION / "QtGui" / "private"]
    fetched = [cache / "qtbase@6.12.0" / "src/gui/painting", platform, cache / "kcolorscheme@6.31.0" / "src",
               cache / "kguiaddons@6.31.0" / "src/colors"]
    kf6 = [KF6_INCLUDE / "KConfig", KF6_INCLUDE / "KConfigCore", KF6_INCLUDE / "KConfigGui"]
    includes = [f"-I{p}" for p in [HERE / "stubs", work, *fetched, *qt, *kf6]]
    common = ["g++", "-std=c++20", "-fPIC", *flags, "-DQT_NO_DEBUG", "-DQT_USE_QSTRINGBUILDER", *includes]
    objects = []
    for sid, path in COMPILED + [(None, str(HERE / "harness.cpp"))]:
        source = cache / sid / path if sid else Path(path)
        obj = work / f"{name}-{Path(path).stem}.o"
        pinned.sandboxed([*common, "-c", source, "-o", obj], writable=[work], check=True)
        objects.append(str(obj))
    exe = work / name
    pinned.sandboxed(["g++", *flags, "-o", exe, *objects, *LIBS], writable=[work], check=True)
    return exe


def run(exe: Path, calls):
    """Each call's result, the QColor's channels as floats; the harness sandboxed, writing nothing."""
    text = "".join(line + "\n" for line in calls)
    out = pinned.sandboxed([exe], readable=[exe.parent], input=text, capture_output=True, text=True,
                           check=True).stdout
    results = [tuple(float(x) for x in line.split()) for line in out.splitlines()]
    if len(results) != len(calls):
        sys.exit(f"{exe.name}: {len(results)} results for {len(calls)} calls")
    return results


def rgba(c):
    return tuple(c) if len(c) == 4 else (*c, 255)


def hex_color(c):
    r, g, b, a = rgba(c)
    return f"#{r:02x}{g:02x}{b:02x}" + ("" if a == 255 else f"{a:02x}")


def srgb(result):
    return "srgb(" + ", ".join(f"{x:.9g}" for x in result) + ")"


def lerp_call(a, b, balance):
    return "lerp " + " ".join(str(x) for x in (*rgba(a), *rgba(b))) + f" {balance!r}"


def shade_call(c, role, contrast):
    return "shade " + " ".join(str(x) for x in rgba(c)) + f" {SHADE_ROLES.index(role)} {contrast!r}"


def chosen(rng, reference, fused, calls, inputs, fixed, wanted_random):
    """The fixed inputs, the random ones on which the fused build differs (up to FUSED_ROWS), and wanted_random
    others."""
    rows = list(range(fixed))
    differing = [i for i in range(fixed, len(calls)) if reference[i] != fused[i]]
    rows += differing[:FUSED_ROWS]
    others = [i for i in range(fixed, len(calls)) if i not in rows]
    rows += rng.sample(others, wanted_random)
    return rows, len(differing)


def write_table(path: Path, header, columns, rows):
    lines = [*header, "\t".join(columns), *("\t".join(row) for row in rows)]
    path.write_text("".join(line + "\n" for line in lines), encoding="utf-8")



# ---- KColorUtils, Kirigami's tint and KColorScheme's effects: colours as the model holds them (f32 channels).

BREEZE = "breeze@6.8.2"
BREEZE_SCHEMES = [("light", "colors/BreezeLight.colors"), ("dark", "colors/BreezeDark.colors")]
SETS = ["Window", "View", "Button", "Selection", "Tooltip", "Complementary", "Header"]
FOREGROUND_KEYS = ["ForegroundNormal", "ForegroundInactive", "ForegroundActive", "ForegroundLink",
                   "ForegroundVisited", "ForegroundNegative", "ForegroundNeutral", "ForegroundPositive"]
DECORATION_KEYS = ["DecorationFocus", "DecorationHover"]
BACKGROUND_KEYS = ["BackgroundNormal", "BackgroundAlternate"]
KEY_KINDS = [(k, "foreground") for k in FOREGROUND_KEYS] + [(k, "decoration") for k in DECORATION_KEYS] + \
            [(k, "background") for k in BACKGROUND_KEYS]
# KColorScheme's calculated backgrounds tint the background toward these at KColorUtils::tint's default 0.3
# (kcolorscheme.cpp:394-405), as the facts' `kcu_tint(<background>, <role>, 0.3)` do.
TINTED_KEYS = ["ForegroundActive", "ForegroundLink", "ForegroundVisited", "ForegroundNegative", "ForegroundNeutral",
               "ForegroundPositive"]
GROUPS = ["active", "disabled", "inactive"]
ROLE_KINDS = ["foreground", "decoration", "background"]
INTENSITY = ["none", "shade", "darken", "lighten"]
COLOR = ["none", "desaturate", "fade", "tint"]
CONTRAST = ["none", "fade", "tint"]
RANDOM_ROWS = 20
SIXTEEN_BIT = 0.1   # the share of random colours drawn with 16-bit channels (a function's own result as an input)
ONE = f32(1.0)


def c8(r, g, b, a=255):
    """A colour of 8-bit channels as the model holds it."""
    return tuple(f32(x / 255) for x in (r, g, b, a))


def c16(rgba16):
    return tuple(f32(x / 65535) for x in rgba16)


def colour_cell(c):
    """`#rrggbb[aa]` where every channel is an 8-bit value, else `srgb(r, g, b, a)` of the f32s."""
    bytes_ = [round(x * 255) for x in c]
    if all(0 <= v <= 255 and f32(v / 255) == x for v, x in zip(bytes_, c)):
        return hex_color(tuple(bytes_))
    return "srgb(" + ", ".join(f32_text(x) for x in c) + ")"


def colour_args(c):
    return " ".join(f32_text(x) for x in c)


def number_cell(x):
    return f32_text(x)


def bool_cell(x):
    return "true" if x else "false"


def result_cell(result):
    return srgb(result[1:])


def result_colour(result):
    """A result as the model holds it, for a later call's input."""
    return tuple(f32(x) for x in result[1:])


def read_scheme(cache: Path, path: str):
    """{group: {key: value}} of a Breeze scheme file, `[Colors:Header][Inactive]` as `Colors:Header][Inactive`."""
    groups = {}
    current = None
    for line in pinned.pinned(cache, BREEZE, path).decode("utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        if line.startswith("[") and line.endswith("]"):
            current = groups.setdefault(line[1:-1], {})
        elif current is not None and "=" in line:
            key, value = line.split("=", 1)
            current[key] = value
    return groups


def scheme_colour(groups, group, key):
    return c8(*(int(x) for x in groups[group][key].split(",")))


def scheme_effects(groups, group):
    """A `[ColorEffects:<group>]`'s tokens as KColorScheme reads Breeze's: (intensity, amount, color, amount,
    colour, contrast, amount)."""
    g = groups[f"ColorEffects:{group}"]
    return (INTENSITY[int(g["IntensityEffect"])], f32(float(g["IntensityAmount"])),
            COLOR[int(g["ColorEffect"])], f32(float(g["ColorAmount"])),
            c8(*(int(x) for x in g["Color"].split(","))),
            CONTRAST[int(g["ContrastEffect"])], f32(float(g["ContrastAmount"])))


def random_colour(rng, translucent=0.3):
    if rng.random() < SIXTEEN_BIT:
        rgb = [rng.randrange(65536) for _ in range(3)]
        a = 65535 if rng.random() >= translucent else rng.randrange(65536)
        return c16((*rgb, a))
    rgb = [rng.randrange(256) for _ in range(3)]
    a = 255 if rng.random() >= translucent else rng.randrange(256)
    return c8(*rgb, a)


def random_colour8(rng):
    return c8(*(rng.randrange(256) for _ in range(3)))


def run_spec(exe: Path, calls):
    """Each call's (spec, r, g, b, a); a call whose result is Qt's invalid colour gives spec 0."""
    return run(exe, calls)


def pick(rng, reference, fused, fixed, wanted_random):
    """The fixed rows (each a valid colour), the random ones on which the fused build differs (up to FUSED_ROWS),
    and wanted_random others, none an invalid colour; and the count of differing ones."""
    bad = [i for i in range(fixed) if reference[i][0] == 0]
    if bad:
        sys.exit(f"fixed inputs {bad} give Qt's invalid colour")
    valid = [i for i in range(fixed, len(reference)) if reference[i][0] != 0]
    differing = [i for i in valid if reference[i] != fused[i]]
    rows = list(range(fixed)) + differing[:FUSED_ROWS]
    taken = set(rows)
    others = [i for i in valid if i not in taken]
    rows += rng.sample(others, wanted_random)
    return rows, len(differing)


def header(function, sources, inputs, comment):
    return ([f"# function: {function}", "# route: compiled_source"] + [f"# source: {s}" for s in sources] +
            ["# generations: kde-breeze", "# generator: tools/kde-vectors", f"# inputs: {inputs}",
             "# outputs: result:colour"] + [f"# {line}" for line in comment])


def extended_note(reference, rows):
    n = sum(1 for i in rows if reference[i][0] == 5)
    return [f"{n} rows give an extended-RGB colour (half-precision channels)."] if n else []


def two_colour_table(rng, reference, fused, op, fixed_inputs, random_inputs, name, sources, third, comment):
    """A table of `f(a, b, x)`: two colours and a number."""
    inputs = fixed_inputs + random_inputs
    calls = [f"{op} {colour_args(a)} {colour_args(b)} {f32_text(x)}" for a, b, x in inputs]
    ref, fus = run_spec(reference, calls), run_spec(fused, calls)
    rows, differing = pick(rng, ref, fus, len(fixed_inputs), RANDOM_ROWS)
    a_name, b_name, x_name = third
    write_table(
        VECTORS / f"{name}.tsv",
        header(name, sources, f"{a_name}:colour {b_name}:colour {x_name}:number",
               comment(differing, len(random_inputs)) + extended_note(ref, rows)),
        [a_name, b_name, x_name, "result"],
        [[colour_cell(inputs[i][0]), colour_cell(inputs[i][1]), number_cell(inputs[i][2]), result_cell(ref[i])]
         for i in rows],
    )
    return len(rows)


def random_comment(what, differing, total):
    return [f"then seeded random inputs: up to {FUSED_ROWS} on which a build with fused multiply-adds gives another",
            f"colour ({differing} of {total}), and {RANDOM_ROWS} more. {what}"]


def kcu_tables(rng, reference, fused, schemes):
    counts = {}
    sources = ["kguiaddons@6.31.0", "qtbase@6.12.0"]
    blue, text = c8(61, 174, 233), c8(35, 38, 41)

    # kcu_mix: the frame colour (background, foreground, frame contrast) and its halves, the facts' 0.15 mixes.
    fixed = []
    firsts = []
    for _, groups in schemes:
        for s in SETS:
            bg = scheme_colour(groups, f"Colors:{s}", "BackgroundNormal")
            fg = scheme_colour(groups, f"Colors:{s}", "ForegroundNormal")
            fixed += [(bg, fg, f32(0.2)), (fg, bg, f32(0.15)), (bg, bg, f32(0.2))]
            firsts.append((bg, fg))
    frames = run_spec(reference, [f"kmix {colour_args(bg)} {colour_args(fg)} {f32_text(f32(0.2))}"
                                  for bg, fg in firsts])
    fixed += [(bg, result_colour(frame), f32(0.5)) for (bg, _), frame in zip(firsts, frames)]
    for t in [0.0, 1.0, -0.25, 1.25, 0.5, 1e-7, 0.99999994]:
        fixed.append((blue, text, f32(t)))
    fixed += [(c8(255, 0, 0, 0), c8(0, 0, 255), f32(0.25)), (c8(255, 0, 0, 128), c8(0, 0, 255, 64), f32(0.5)),
              (c8(255, 0, 0, 0), c8(0, 0, 255, 0), f32(0.5)), (c8(61, 174, 233, 204), c8(255, 255, 255), f32(0.3)),
              (c8(255, 255, 255, 1), c8(0, 0, 0, 2), f32(0.5))]
    randoms = [(random_colour(rng), random_colour(rng), f32(rng.random())) for _ in range(RANDOM_INPUTS)]
    counts["kcu_mix"] = two_colour_table(
        rng, reference, fused, "kmix", fixed, randoms, "kcu_mix", sources, ("a", "b", "bias"),
        lambda d, n: ["written by tools/kde-vectors/generate.py; do not edit. Breeze's sets: the frame colour",
                      "(background, foreground, 0.2), its half toward the background, the 0.15 mixes; the bias's",
                      "ends and beyond them; translucent and transparent inputs;"] + random_comment(
                          "A bias is an f32, passed to KColorUtils as that double.", d, n))

    # kcu_tint: KColorScheme's calculated backgrounds (0.3), the Selection set's inactive background (0.4), the
    # inactive contrast tint (0.1).
    fixed = []
    for _, groups in schemes:
        for s in SETS:
            bg = scheme_colour(groups, f"Colors:{s}", "BackgroundNormal")
            fixed += [(bg, scheme_colour(groups, f"Colors:{s}", k), f32(0.3)) for k in TINTED_KEYS]
        sel_bg = scheme_colour(groups, "Colors:Selection", "BackgroundNormal")
        for k in BACKGROUND_KEYS:
            fixed.append((scheme_colour(groups, "Colors:Window", k), sel_bg, f32(0.4)))
        fixed.append((scheme_colour(groups, "Colors:Window", "ForegroundNormal"),
                      scheme_colour(groups, "Colors:Window", "BackgroundNormal"), f32(0.1)))
    for t in [0.0, 1.0, -0.1, 1.1, 0.5, 0.9, 0.01, 0.99999994]:
        fixed.append((c8(239, 240, 241), blue, f32(t)))
    fixed += [(text, text, f32(0.3)), (c8(0, 0, 0), c8(255, 255, 255), f32(0.3)),
              (c8(255, 255, 255), c8(0, 0, 0), f32(0.3)), (c8(128, 128, 128), c8(255, 0, 0), f32(0.6)),
              (c8(239, 240, 241, 128), blue, f32(0.3)), (c8(239, 240, 241), c8(61, 174, 233, 64), f32(0.3))]
    randoms = [(random_colour(rng), random_colour(rng), f32(rng.random())) for _ in range(RANDOM_INPUTS)]
    counts["kcu_tint"] = two_colour_table(
        rng, reference, fused, "ktint", fixed, randoms, "kcu_tint", sources, ("a", "b", "amount"),
        lambda d, n: ["written by tools/kde-vectors/generate.py; do not edit. Breeze's sets: each background tinted",
                      "toward its active, link, visited, negative, neutral and positive text at 0.3, Window's",
                      "backgrounds toward Selection's at 0.4, the text toward the background at 0.1; the amount's",
                      "ends and beyond them; equal, black and white, grey and translucent inputs;"] + random_comment(
                          "An amount is an f32, passed to KColorUtils as that double.", d, n))

    # kcu_shade: no fact calls it; its luma and chroma offsets over each clamp and each hue sextant.
    colours = [c8(239, 240, 241), c8(32, 35, 38), blue, c8(128, 128, 128), c8(0, 0, 0), c8(255, 255, 255),
               c8(218, 68, 83), c8(39, 174, 96), c8(246, 116, 0), c8(155, 89, 182)]
    fixed = [(c, f32(ky), f32(kc)) for c in colours for ky in [-0.1, 0.1, 0.6, -1.5, 1.5] for kc in [0.0, 0.2, -0.3]]
    fixed += [(c8(*rgb), f32(0.1), f32(0.0)) for rgb in
              [(255, 0, 0), (255, 255, 0), (0, 255, 0), (0, 255, 255), (0, 0, 255), (255, 0, 255), (255, 0, 1)]]
    fixed += [(c8(61, 174, 233, 128), f32(0.1), f32(0.1)), (blue, f32(0.0), f32(2.0))]
    randoms = [(random_colour(rng), f32(rng.uniform(-1, 1)), f32(rng.uniform(-1, 1))) for _ in range(RANDOM_INPUTS)]
    calls = [f"kshade {colour_args(c)} {f32_text(ky)} {f32_text(kc)}" for c, ky, kc in fixed + randoms]
    ref, fus = run_spec(reference, calls), run_spec(fused, calls)
    rows, differing = pick(rng, ref, fus, len(fixed), RANDOM_ROWS)
    inputs = fixed + randoms
    write_table(
        VECTORS / "kcu_shade.tsv",
        header("kcu_shade", sources, "c:colour luma:number chroma:number",
               ["written by tools/kde-vectors/generate.py; do not edit. Breeze's colours, grey, black, white and",
                "saturated ones at luma and chroma offsets inside and beyond the clamps; each hue sextant; alpha;"] +
               random_comment("The offsets are f32s, passed to KColorUtils as those doubles.", differing,
                              len(randoms)) + extended_note(ref, rows)),
        ["c", "luma", "chroma", "result"],
        [[colour_cell(inputs[i][0]), number_cell(inputs[i][1]), number_cell(inputs[i][2]), result_cell(ref[i])]
         for i in rows],
    )
    counts["kcu_shade"] = len(rows)

    # kcu_overlay: no fact calls it; a paint at every alpha's branch over an opaque and a translucent base.
    fixed = []
    for base in [c8(239, 240, 241), c8(32, 35, 38)]:
        fixed += [(base, c8(61, 174, 233, a)) for a in [0, 1, 2, 64, 128, 200, 254, 255]]
        fixed += [(base, c8(0, 0, 0, 25)), (base, c8(255, 255, 255, 128))]
    fixed += [(c8(61, 174, 233, 100), c8(0, 0, 0, 128)), (c8(61, 174, 233, 0), c8(255, 255, 255, 64)),
              (c8(239, 240, 241), c16((40000, 20000, 1000, 30000))), (c8(239, 240, 241), c16((65535, 0, 0, 100)))]
    randoms = [(random_colour(rng), random_colour(rng, translucent=0.8)) for _ in range(RANDOM_INPUTS)]
    calls = [f"koverlay {colour_args(a)} {colour_args(b)}" for a, b in fixed + randoms]
    ref, fus = run_spec(reference, calls), run_spec(fused, calls)
    rows, differing = pick(rng, ref, fus, len(fixed), RANDOM_ROWS)
    inputs = fixed + randoms
    write_table(
        VECTORS / "kcu_overlay.tsv",
        header("kcu_overlay", sources, "under:colour over:colour",
               ["written by tools/kde-vectors/generate.py; do not edit. A paint at alpha 0, 1, 2, 64, 128, 200,",
                "254 and 255 over Breeze's light and dark window backgrounds, translucent and 16-bit inputs;"] +
               random_comment("QPainter's raster engine is the system's Qt 6.12.0 library, not compiled from the "
                              "pin.", differing, len(randoms)) + extended_note(ref, rows)),
        ["under", "over", "result"],
        [[colour_cell(inputs[i][0]), colour_cell(inputs[i][1]), result_cell(ref[i])] for i in rows],
    )
    counts["kcu_overlay"] = len(rows)
    return counts


def kirigami_tint_table(rng, reference, fused, schemes):
    # RoundButton's border: Qt.tint(textColor, Qt.alpha(<frame colour>, 0.8)), the frame colour kirigami_lerp's.
    pairs = []
    for _, groups in schemes:
        for s in ["Button", "Window", "View"]:
            pairs.append((scheme_colour(groups, f"Colors:{s}", "BackgroundNormal"),
                          scheme_colour(groups, f"Colors:{s}", "ForegroundNormal")))
    as8 = lambda c: tuple(round(x * 255) for x in c)
    frames = run(reference, [lerp_call(as8(bg), as8(fg), f32(0.2)) for bg, fg in pairs])
    fixed = [(fg, (*(f32(x) for x in frame[:3]), f32(0.8)), ONE) for (_, fg), frame in zip(pairs, frames)]
    text, blue = c8(35, 38, 41), c8(61, 174, 233)
    for tint_alpha in [255, 204, 128, 1, 0]:
        for alpha in [1.0, 0.0, 0.5, 0.99999994, 1e-7]:
            fixed.append((text, c8(61, 174, 233, tint_alpha), f32(alpha)))
    fixed += [(c8(35, 38, 41, 128), blue, f32(0.5)), (c8(35, 38, 41, 128), c8(61, 174, 233, 64), f32(1.0)),
              (c8(35, 38, 41, 0), c8(61, 174, 233, 64), f32(1.0)), (text, c16((65535, 0, 0, 65534)), ONE),
              (text, c16((65535, 0, 0, 1)), ONE)]
    randoms = []
    for _ in range(RANDOM_INPUTS):
        alpha = ONE if rng.random() < 0.3 else f32(rng.random())
        randoms.append((random_colour(rng), random_colour(rng, translucent=0.6), alpha))
    return two_colour_table(
        rng, reference, fused, "rtint", fixed, randoms, "kirigami_tint",
        ["kirigami@6.31.0", "qtbase@6.12.0"], ("c", "tint", "alpha"),
        lambda d, n: ["written by tools/kde-vectors/generate.py; do not edit. RoundButton's border over Breeze's",
                      "Button, Window and View sets (the text tinted by the frame colour, kirigami_lerp's at 0.2,",
                      "at alpha 0.8); the tint's alpha 1, 0.8, 0.5, 1/255 and 0 at alpha 1, 0, 0.5, just below 1",
                      "and just above 0; translucent and 16-bit inputs;"] + random_comment(
                          "An alpha is an f32, passed to Kirigami as that double.", d, n))


def effects_call(row):
    c, bg, group, kind, intensity, iamount, color, camount, ccolor, contrast, kamount, enabled = row
    return (f"effects {colour_args(c)} {colour_args(bg)} {GROUPS.index(group)} {ROLE_KINDS.index(kind)} "
            f"{INTENSITY.index(intensity)} {f32_text(iamount)} {COLOR.index(color)} {f32_text(camount)} "
            f"{colour_args(ccolor)} {CONTRAST.index(contrast)} {f32_text(kamount)} {int(enabled)}")


def kde_effects_table(rng, reference, fused, schemes):
    fixed = []
    for _, groups in schemes:
        disabled = scheme_effects(groups, "Disabled")
        inactive = scheme_effects(groups, "Inactive")
        enabled = groups["ColorEffects:Inactive"].get("Enable", "false") == "true"
        # The disabled group over every set but Selection (whose Disabled group starts from Window's colours).
        for s in [s for s in SETS if s != "Selection"]:
            bg = scheme_colour(groups, f"Colors:{s}", "BackgroundNormal")
            for key, kind in KEY_KINDS:
                fixed.append((scheme_colour(groups, f"Colors:{s}", key), bg, "disabled", kind, *disabled, False))
        # The inactive group: Window's colours as the file sets it (disabled in Breeze), then enabled; Header's
        # [Inactive] colours.
        bg = scheme_colour(groups, "Colors:Window", "BackgroundNormal")
        hbg = scheme_colour(groups, "Colors:Header][Inactive", "BackgroundNormal")
        for key, kind in KEY_KINDS:
            c = scheme_colour(groups, "Colors:Window", key)
            fixed.append((c, bg, "inactive", kind, *inactive, enabled))
            fixed.append((c, bg, "inactive", kind, *inactive, True))
            fixed.append((scheme_colour(groups, "Colors:Header][Inactive", key), hbg, "inactive", kind, *inactive,
                          True))
    # Every combination of the three effects, KColorScheme's unset amounts and colours (kcolorscheme.cpp:41-48),
    # the colour effects at a non-zero amount.
    text, window = c8(35, 38, 41), c8(239, 240, 241)
    for intensity in INTENSITY:
        for color in COLOR:
            for contrast in CONTRAST:
                camount = {"desaturate": -0.9, "fade": 0.3, "tint": 0.3}.get(color, 0.0)
                for kind, c in [("foreground", text), ("background", window)]:
                    fixed.append((c, window, "disabled", kind, intensity, f32(0.1), color, f32(camount),
                                  c8(56, 56, 56), contrast, f32(0.65), False))
    for kind in ROLE_KINDS:
        fixed.append((text, window, "active", kind, "darken", f32(0.1), "fade", f32(0.3), c8(56, 56, 56), "fade",
                      f32(0.65), True))
    # The amounts' edges.
    for intensity, amount in [("shade", -0.5), ("shade", 1.5), ("darken", 1.0), ("lighten", 1.0), ("lighten", -0.5)]:
        fixed.append((c8(61, 174, 233), window, "inactive", "background", intensity, f32(amount), "none",
                      f32(0.0), c8(112, 111, 110), "none", f32(0.0), True))
    for color, amount in [("desaturate", 1.0), ("desaturate", 2.0), ("fade", 0.0), ("fade", 1.0), ("tint", 1.0),
                          ("tint", 0.0)]:
        fixed.append((c8(61, 174, 233), window, "inactive", "background", "none", f32(0.0), color, f32(amount),
                      c8(112, 111, 110), "none", f32(0.0), True))
    for contrast, amount in [("fade", 1.0), ("tint", 1.0), ("fade", -0.5)]:
        fixed.append((text, window, "disabled", "foreground", "none", f32(0.0), "none", f32(0.0), c8(56, 56, 56),
                      contrast, f32(amount), False))
    randoms = []
    for _ in range(RANDOM_INPUTS):
        randoms.append((random_colour(rng, translucent=0.1), random_colour(rng, translucent=0.1),
                        rng.choice(["disabled", "disabled", "inactive", "inactive", "active"]),
                        rng.choice(ROLE_KINDS), rng.choice(INTENSITY), f32(rng.uniform(-0.5, 1.2)),
                        rng.choice(COLOR), f32(rng.uniform(-1, 1.2)), random_colour8(rng), rng.choice(CONTRAST),
                        f32(rng.uniform(-0.2, 1.1)), rng.random() < 0.5))
    inputs = fixed + randoms
    calls = [effects_call(row) for row in inputs]
    ref, fus = run_spec(reference, calls), run_spec(fused, calls)
    rows, differing = pick(rng, ref, fus, len(fixed), RANDOM_ROWS)
    write_table(
        VECTORS / "kde_effects.tsv",
        header("kde_effects", ["kcolorscheme@6.31.0", "kguiaddons@6.31.0", "qtbase@6.12.0"],
               "c:colour background:colour group:enum<KdeGroup> role_kind:enum<KdeRoleKind> "
               "intensity:enum<KdeIntensityEffect> intensity_amount:number color:enum<KdeColorEffect> "
               "color_amount:number color_color:colour contrast:enum<KdeContrastEffect> contrast_amount:number "
               "inactive_enabled:bool",
               ["written by tools/kde-vectors/generate.py; do not edit. The operands after role_kind are the",
                "row's group's token.kde.<group> fields (for group active, unread) and token.kde.inactive_enabled.",
                "Breeze's disabled effects over every set's roles but Selection's; its inactive effects over",
                "Window's roles as the file sets them (not enabled) and enabled, and over Header's [Inactive]",
                "roles; every combination of the three effects at KColorScheme's unset amounts; group active;",
                "the amounts' edges;"] + random_comment(
                    "The amounts are f32s, read by KColorScheme as those doubles.", differing, len(randoms))
               + extended_note(ref, rows)),
        ["c", "background", "group", "role_kind", "intensity", "intensity_amount", "color", "color_amount",
         "color_color", "contrast", "contrast_amount", "inactive_enabled", "result"],
        [[colour_cell(r[0]), colour_cell(r[1]), r[2], r[3], r[4], number_cell(r[5]), r[6], number_cell(r[7]),
          colour_cell(r[8]), r[9], number_cell(r[10]), bool_cell(r[11]), result_cell(ref[i])]
         for i in rows for r in [inputs[i]]],
    )
    return len(rows)


def kde_selection_inactive_table(rng, reference, fused, schemes):
    fixed = []
    for _, groups in schemes:
        sel_bg = scheme_colour(groups, "Colors:Selection", "BackgroundNormal")
        for key, kind in KEY_KINDS:
            for change in [True, False]:
                fixed.append((scheme_colour(groups, "Colors:Selection", key), scheme_colour(groups, "Colors:Window", key),
                              sel_bg, kind, change))
    fixed += [(c8(255, 255, 255), c8(239, 240, 241, 128), c8(61, 174, 233), "background", True),
              (c8(255, 255, 255), c8(0, 0, 0), c8(255, 255, 255), "background", True)]
    randoms = [(random_colour8(rng), random_colour8(rng), random_colour8(rng), rng.choice(ROLE_KINDS),
                rng.random() < 0.7) for _ in range(RANDOM_INPUTS)]
    inputs = fixed + randoms
    calls = [f"selinactive {colour_args(s)} {colour_args(w)} {colour_args(b)} {ROLE_KINDS.index(k)} {int(ch)}"
             for s, w, b, k, ch in inputs]
    ref, fus = run_spec(reference, calls), run_spec(fused, calls)
    rows, differing = pick(rng, ref, fus, len(fixed), RANDOM_ROWS)
    write_table(
        VECTORS / "kde_selection_inactive.tsv",
        header("kde_selection_inactive", ["kcolorscheme@6.31.0", "kguiaddons@6.31.0", "qtbase@6.12.0"],
               "selection:colour window:colour selection_background:colour role_kind:enum<KdeRoleKind> "
               "inactive_change_selection_color:bool",
               ["written by tools/kde-vectors/generate.py; do not edit. Breeze's Selection and Window roles, each",
                "key at its kind, with the switch on and off; a translucent window colour;"] + random_comment(
                    "The colours are 8-bit (KColorScheme reads them from a config).", differing, len(randoms))
               + extended_note(ref, rows)),
        ["selection", "window", "selection_background", "role_kind", "inactive_change_selection_color", "result"],
        [[colour_cell(r[0]), colour_cell(r[1]), colour_cell(r[2]), r[3], bool_cell(r[4]), result_cell(ref[i])]
         for i in rows for r in [inputs[i]]],
    )
    return len(rows)


def new_tables(rng, reference, fused, source_cache):
    schemes = [(name, read_scheme(source_cache, path)) for name, path in BREEZE_SCHEMES]
    counts = kcu_tables(rng, reference, fused, schemes)
    counts["kirigami_tint"] = kirigami_tint_table(rng, reference, fused, schemes)
    counts["kde_effects"] = kde_effects_table(rng, reference, fused, schemes)
    counts["kde_selection_inactive"] = kde_selection_inactive_table(rng, reference, fused, schemes)
    for name, n in counts.items():
        print(f"{name}.tsv: {n} rows")


# ---- QFont::fromString, the KDE reader's font entries (the public_api route: the system's Qt 6.12.0)

PLASMA_INTEGRATION = "plasma-integration@6.8.2"
FONT_DATA = "qt6/src/platformtheme/kfontsettingsdata.cpp"
QT_STYLE_HINTS = {"SansSerif": 0, "Helvetica": 0, "Serif": 1, "Times": 1, "TypeWriter": 2, "Courier": 2,
                  "OldEnglish": 3, "Decorative": 3, "System": 4, "AnyStyle": 5, "Cursive": 6, "Monospace": 7,
                  "Fantasy": 8}
QT_WEIGHTS = {"Thin": 100, "ExtraLight": 200, "Light": 300, "Normal": 400, "Medium": 500, "DemiBold": 600,
              "Bold": 700, "ExtraBold": 800, "Black": 900}
# Qt 5's QFont::Weight constants (0 to 99), each and the midpoints between neighbours, where the nearest is a tie
LEGACY_WEIGHTS = [0, 6, 12, 18, 19, 25, 37, 38, 50, 53, 54, 57, 60, 63, 69, 75, 78, 81, 84, 87, 93, 99, 100, 400,
                  -1, -6, -7, -1000]
# fields 1 to 15 of a Qt 6 string (field 0 the family), to vary one at a time
QT6_TAIL = ["10", "-1", "5", "400", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "1"]
QT5_TAIL = ["10", "-1", "5", "50", "0", "0", "0", "0", "0"]
# one field's variants: (field index, values); the field's value as written, whitespace and junk included
FIELD_VARIANTS = [
    (1, ["11", "10.5", "9.75", "0", "-1", "-0.5", " 10 ", "+10", ".5", "5.", "1e1", "1E+1", "1e", "abc", "", "nan",
         "inf", "-inf", "1e400", "10 ", " 10", "0x10", "1,5"]),
    (2, ["13", "0", "-1", "65535", "65536", "13.5", " 13 ", "+13", "0x10", "abc", "", "2147483647",
         "2147483648"]),
    (3, ["0", "1", "2", "3", "4", "5", "6", "7", "8", "255", "256", "-1"]),
    (4, ["1", "100", "350", "999", "1000", "1001", "0", "-5", "65536", "abc", "", " 700 ", "700.0"]),
    (5, ["1", "2", "3", "4", "-1", "x"]),
    (6, ["1", "2", "-1"]),
    (7, ["1"]),
    (8, ["1", "x"]),
    (10, ["1", "2", "3", "4", "5", "8", "-1"]),
    (11, ["1", "2"]),
    (12, ["1.5", "-2", "0.01", "0.015625", "0.02", "-0.01", "110", "abc"]),
    (13, ["2.7", "-1", "0.03"]),
    (14, ["50", "100", "125", "4000", "4001", "-1", "4096"]),
    (15, ["0", "2", "256", "65535", "65536", "-1"]),
]


def plasma_fonts(cache: Path):
    """Plasma's default fonts (kfontsettingsdata.cpp's DefaultFontData): (group, key, family, size, weight, hint)."""
    text = pinned.pinned(cache, PLASMA_INTEGRATION, FONT_DATA).decode("utf-8")
    rows = re.findall(r'\{(GeneralId|"WM"), "(\w+)", (DefaultFont|"[^"]+"), (\d+), QFont::(\w+), QFont::(\w+), "\w+"\}',
                      text)
    default = re.search(r'DefaultFont\[\] = "([^"]+)";', text).group(1)
    if len(rows) != 7 or not default:
        sys.exit(f"{PLASMA_INTEGRATION} {FONT_DATA}: {len(rows)} default fonts, not 7")
    return [(group.strip('"').replace("GeneralId", "General"), key, default if family == "DefaultFont" else
             family.strip('"'), int(size), QT_WEIGHTS[weight], QT_STYLE_HINTS[hint])
            for group, key, family, size, weight, hint in rows]


def qfont_build(work: Path):
    qt = [QT_INCLUDE, QT_INCLUDE / "QtCore", QT_INCLUDE / "QtGui"]
    exe = work / "qfont"
    pinned.sandboxed(["g++", "-std=c++20", "-fPIC", "-O2", *(f"-I{p}" for p in qt), "-o", exe, HERE / "qfont.cpp",
                      "-lQt6Gui", "-lQt6Core"], writable=[work], check=True)
    return exe


def qfont_run(exe: Path, lines):
    out = pinned.sandboxed([exe], readable=[exe.parent], input="".join(x + "\n" for x in lines),
                           capture_output=True, text=True, check=True).stdout.splitlines()
    if len(out) != len(lines):
        sys.exit(f"{exe.name}: {len(out)} results for {len(lines)} lines")
    return out


def hex_text(s):
    return "=" if s == "" else s.encode("utf-8").hex()


def unhex_text(cell):
    return None if cell == "-" else "" if cell == "=" else bytes.fromhex(cell).decode("utf-8")


def written_fonts(exe: Path, plasma):
    """The strings Qt 6.12 writes (QFont::toString, as KConfig writes a font entry) for Plasma's defaults and for
    the forms a user's choice takes: bold, italic, a fractional size, a pixel size, a style name, features, axes."""
    def w(family, pt, px, weight, style, hint, style_name=None, features=None, axes=None):
        return "\t".join(["w", hex_text(family), repr(float(pt)), str(px), str(weight), str(style), str(hint),
                          "-" if style_name is None else hex_text(style_name),
                          "-" if features is None else hex_text(features), "-" if axes is None else hex_text(axes)])
    specs = [w(family, size, 0, weight, 0, hint) for _, _, family, size, weight, hint in plasma]
    specs += [w(family, size, 0, weight, 0, hint, "Regular") for _, _, family, size, weight, hint in plasma[:2]]
    specs += [w("Noto Sans", 10, 0, 700, 0, 0), w("Noto Sans", 10, 0, 400, 1, 0, "Italic"),
              w("Noto Sans", 10.5, 0, 400, 0, 0), w("Noto Sans", 0, 13, 400, 0, 0),
              w("Noto Sans", 11, 0, 600, 0, 0, "SemiBold"), w("Inter", 10, 0, 400, 0, 5, None, "tnum=1,ss01=1"),
              w("Inter", 10, 0, 400, 0, 5, None, None, "wght=450,opsz=14"),
              w("Inter", 10, 0, 400, 0, 5, None, "liga=0", "wght=450.5"),
              w("Hack", 9, 0, 400, 0, 7), w("Noto Sans Mono", 10, 0, 400, 0, 7),
              w("Cantarell", 11, 0, 400, 2, 5), w("Noto Sans CJK JP", 10, 0, 400, 0, 5)]
    return [unhex_text(x) for x in qfont_run(exe, specs)]


def qfont_inputs(written):
    family = "Noto Sans"
    rows = list(written)
    # Qt 5's strings (10 fields, 11 with a style name) and earlier Qt 6 releases' (16, 17 with a style name)
    rows += [",".join([family, *QT5_TAIL]), ",".join([family, *QT5_TAIL, "Regular"]),
             ",".join(["Hack", *QT5_TAIL]), "Noto Sans,10,-1,5,75,0,0,0,0,0", "Noto Sans,10,-1,5,50,1,0,0,0,0",
             "Noto Sans,8,-1,5,50,0,0,0,0,0,Regular", "Noto Sans,-1,13,5,50,0,0,0,0,0",
             ",".join([family, *QT6_TAIL]), ",".join([family, *QT6_TAIL, "Regular"]),
             "Noto Sans,10,-1,5,700,0,0,0,0,0,0,0,0,0,0,1", "Noto Sans,-1,13,5,400,0,0,0,0,0,0,0,0,0,0,1"]
    # the field count's cases (fromString's `count`): 1, 2, 3 to 9 refused, 10 to 15, 16, 17, the feature and axis
    # counts and their entries
    base = [family, *QT6_TAIL]
    rows += ["", " ", "Noto Sans", "Noto Sans,11", "Noto Sans,0", "Noto Sans,abc", "Noto Sans,11,"]
    rows += [",".join(base[:n]) for n in range(3, 10)]
    rows += [",".join([*base[:10], "Regular"]), ",".join([*base[:10], ""])]
    rows += [",".join(base[:n]) for n in range(12, 16)]
    rows += [",".join([*base, "Regular", "1", "tnum=1", "1", "wght=450"]),
             ",".join([*base, "", "0", "0"]), ",".join([*base, "Bold", "0"]), ",".join([*base, "", "2", "tnum=1"]),
             ",".join([*base, "", "2", "tnum=1", "liga=0", "1", "wght=450.5"]),
             ",".join([*base, "", "1", "tnum", "0"]), ",".join([*base, "", "1", "tnu=1", "0"]),
             ",".join([*base, "", "1", "tnums=1", "0"]), ",".join([*base, "", "1", "tnum=x", "0"]),
             ",".join([*base, "", "1", "tnum=-1", "0"]), ",".join([*base, "", "1", "tnum=4294967295", "0"]),
             ",".join([*base, "", "1", "tnum=4294967296", "0"]), ",".join([*base, "", "1", " tnum=1", "0"]),
             ",".join([*base, "", "1", "tnum= 1 ", "0"]), ",".join([*base, "", "1", "tnum=1=2", "0"]),
             ",".join([*base, "", "1", "tnum=1", "1"]), ",".join([*base, "", "0", "1", "wght=abc"]),
             ",".join([*base, "", "0", "1", "wght=1e40"]), ",".join([*base, "", "0", "1", "wght=-1.5"]),
             ",".join([*base, "", "0", "2", "wght=450", "wght=500"]), ",".join([*base, "", "-1", "0"]),
             ",".join([*base, "", "x", "0"]), ",".join([*base, "", "0", "x"]), ",".join([*base, "", "0"]),
             ",".join([*base, "", "0", "0", "extra"])]
    # whitespace, the family's own characters, the weight scales
    rows += [" Noto Sans,10", "Noto Sans ,10", "\tNoto Sans,10\n", " Noto Sans,10", " ,10", ",10",
             "\"Noto Sans\",10", "'Noto Sans',10", "Noto Sans [Monotype],10", "Noto Sans CJK JP,10",
             "Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1 ", "Noto Sans, 10 ,-1,5, 400 ,0,0,0,0,0,0,0,0,0,0,1"]
    rows += [",".join([family, "10", "-1", "5", str(x), *QT5_TAIL[4:]]) for x in LEGACY_WEIGHTS]
    rows += [",".join([family, "10", "-1", "5", str(x), *QT5_TAIL[4:], "Regular"]) for x in [50, 75, 400]]
    rows += [",".join([family, "10", "-1", "5", str(x), *QT6_TAIL[4:10]]) for x in [50, 400, 700]]
    for k, values in FIELD_VARIANTS:
        for v in values:
            fields = [*base]
            fields[k] = v
            rows.append(",".join(fields))
            if k <= 8:
                fields = [family, *QT5_TAIL]
                fields[k] = v
                rows.append(",".join(fields))
    rows += ["Noto Sans,10,13,5,400,0,0,0,0,0,0,0,0,0,0,1", "Noto Sans,0,0,5,400,0,0,0,0,0,0,0,0,0,0,1",
             "Noto Sans,-1,-1,5,400,0,0,0,0,0,0,0,0,0,0,1"]
    seen, out = set(), []
    for r in rows:
        if r not in seen:
            seen.add(r)
            out.append(r)
    return out


QF_FAMILIES, QF_SIZE, QF_HINT, QF_STRATEGY, QF_WEIGHT, QF_STYLE, QF_UNDERLINE, QF_STRIKE, QF_FIXED, QF_STRETCH, \
    QF_CAPS, QF_LETTER, QF_WORD = 0x20000, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x100, 0x200, 0x400, 0x1000, 0x2000, \
    0x4000


def real_cell(text):
    """A real the harness printed "%.17g", as the shortest decimal that reads back as that double."""
    return repr(float(text))


def qfont_row(s, r):
    f = r.split("\t")
    ok, mask = f[0] == "1", int(f[1])
    if not ok:
        assert mask == 0 and f[17:] == ["-", "-", "-"], (s, r)

    def opt(bit, cell):
        return cell if mask & bit else "none"

    def opt_text(cell):
        value = unhex_text(cell)
        return "none" if value is None else string_cell(value)
    point, pixel = float(f[3]), int(f[4])
    return [
        string_cell(s), "true" if ok else "false",
        "none" if not mask & QF_FAMILIES else string_cell(unhex_text(f[2])),
        real_cell(f[3]) if mask & QF_SIZE and point > 0 else "none",
        str(pixel) if mask & QF_SIZE and pixel > 0 else "none",
        opt(QF_HINT, f[5]), opt(QF_WEIGHT, f[6]), opt(QF_STYLE, f[7]),
        opt(QF_UNDERLINE, "true" if f[8] == "1" else "false"), opt(QF_STRIKE, "true" if f[9] == "1" else "false"),
        opt(QF_FIXED, "true" if f[10] == "1" else "false"), opt(QF_CAPS, f[11]), opt(QF_LETTER, f[12]),
        opt(QF_LETTER, real_cell(f[13])), opt(QF_WORD, real_cell(f[14])), opt(QF_STRETCH, f[15]),
        opt(QF_STRATEGY, f[16]), opt_text(f[17]), opt_text(f[18]), opt_text(f[19]),
    ]


def string_cell(s):
    """A `string` cell: the JSON string literal of `s` (the proposed column type: any string, the empty one,
    surrounding whitespace and control characters included)."""
    return json.dumps(s, ensure_ascii=False)


def qfont_table(source_cache: Path, work: Path):
    exe = qfont_build(work)
    inputs = qfont_inputs(written_fonts(exe, plasma_fonts(source_cache)))
    results = qfont_run(exe, [hex_text(s) for s in inputs])
    rows, infinite = [], []
    for s, r in zip(inputs, results):
        row = qfont_row(s, r)
        # a number cell holds no infinity: an infinite point size ("inf", "1e400") is left to the description
        if row[3] in ("inf", "-inf", "nan"):
            infinite.append(s)
        else:
            rows.append(row)
    write_table(
        VECTORS / "reader.qfont_from_string.tsv",
        ["# function: reader.qfont_from_string",
         "# route: public_api",
         "# source: qtbase@6.12.0",
         "# generations: kde-breeze",
         "# generator: tools/kde-vectors",
         "# inputs: s:string",
         "# outputs: ok:bool family:option<string> point_size:option<number> pixel_size:option<number> "
         "style_hint:option<number> weight:option<number> style:option<number> underline:option<bool> "
         "strike_out:option<bool> fixed_pitch:option<bool> capitalization:option<number> "
         "letter_spacing_type:option<number> letter_spacing:option<number> word_spacing:option<number> "
         "stretch:option<number> style_strategy:option<number> style_name:option<string> features:option<string> "
         "axes:option<string>",
         f"# written by tools/kde-vectors/generate.py on Qt {qt_version()}; do not edit. Each row:",
         "# QFont::fromString(s) on a default-constructed QFont, read through QFont's getters. A string cell is a",
         "# JSON string literal. `none`: the call did not set the property (not in QFont::resolveMask(); for",
         "# style_name, features and axes, which carry no mask bit: a font holding sentinels kept them), so a",
         "# reader keeps its starting value there; point_size is `none` also where the size set is a pixel size,",
         "# and pixel_size where it is a point size. Integers are Qt's (QFont::StyleHint, the OpenType weight,",
         "# QFont::Style, QFont::Capitalization, QFont::SpacingType, the stretch in percent, QFont::StyleStrategy).",
         "# features and axes: the call's entries as tag=value joined by commas, by tag, \"\" for none. The inputs:",
         "# the strings Qt 6.12 writes for Plasma's default fonts (plasma-integration@6.8.2",
         "# qt6/src/platformtheme/kfontsettingsdata.cpp) and for typical choices, Qt 5's and earlier Qt 6",
         "# releases' strings, then each field count and each field's variants, whitespace and junk. Left out:",
         f"# {len(infinite)} inputs whose point size Qt sets to infinity (a number cell holds none):",
         *(f"# {string_cell(s)}" for s in infinite)],
        ["s", "ok", "family", "point_size", "pixel_size", "style_hint", "weight", "style", "underline",
         "strike_out", "fixed_pitch", "capitalization", "letter_spacing_type", "letter_spacing", "word_spacing",
         "stretch", "style_strategy", "style_name", "features", "axes"],
        rows,
    )
    print(f"reader.qfont_from_string.tsv: {len(rows)} rows")


# ---- The KDE reader's scheme test and its font DPI (the public_api route: the system's Qt 6.12.0, KConfig 6.31 and
# KColorScheme 6.31, plasma-integration's platform theme, Qt's xcb plugin on an Xvfb server)

PORTAL_KDE = "xdg-desktop-portal-kde@6.8.2"
PORTAL_SETTINGS = "src/settings.cpp"
HINTS = "qt6/src/platformtheme/khintssettings.cpp"
# KColorScheme's minor, kcolorscheme@6.31.0's: the headers' KCOLORSCHEME_VERSION_STRING.
KCOLORSCHEME_MINOR = "6.31"
XVFB = "Xvfb"
XRDB = "xrdb"
READER_LIBS = ["-lKF6ColorScheme", "-lKF6ConfigGui", "-lKF6ConfigCore", "-lQt6Gui", "-lQt6Core", "-lxcb", "-ldl"]
# Qt::ColorScheme's values (qnamespace.h): what QStyleHints::colorScheme() answers.
QT_COLOR_SCHEME = {1: "light", 2: "dark"}
# kconfiggroupgui.cpp:37-53: the values KConfig reads as an invalid colour (empty, "invalid") or hands to Qt's colour
# parser (a leading '#', no comma), which S9.3 (ADD-A140) has the reader read as unset.
KCONFIG_QT_PARSED = ["", "invalid", "Invalid", "#232629", "#eff0f1", "#000", "#fff", "#2326294d", "#zzzzzz", "#",
                     "black", "white", "transparent", "red", "35;38;41", "35 38 41", "35"]
DARK = "35,38,41"
# A dark colour's entry varied as KConfig reads its components (kconfiggroupgui.cpp:54-93): the forms it accepts give
# the dark colour, the others its default.
KCONFIG_FORMS = ["35, 38, 41", "35 ,38 ,41", "\t35,\t38,\t41", "35,38,41\t", " 35,38,41 ", "+35,+38,+41",
                 "035,038,041", "-0,-0,-0", "35,38,41,255", "35,38,41,0", "35,38,41,128", "35,38", "35,38,41,255,0",
                 "35,38,41,", ",35,38,41", "35,,41", "35,38,256", "35,38,41,256", "-1,38,41", "35,38,-1",
                 "35,38,0x29", "35,38,41.0", "35,38,4 1", "35,38,41 #", "35,38,99999999999", "35,38,2147483689",
                 "35,38,1e1", "3 5,38,41", "35,38,41 ", "35,38,٤١", "35,38,41\n", "35,38,41\r",
                 "35,38,41\v", "35,38,41\f", "35,38,41\x00"]
RANDOM_SCHEME = 40


def kcolorscheme_version():
    """The KF6 ColorScheme headers' `KCOLORSCHEME_VERSION_STRING`; empty where they are missing."""
    try:
        header = (KF6_INCLUDE / "KColorScheme" / "kcolorscheme_version.h").read_text(encoding="utf-8")
    except OSError:
        return ""
    found = re.search(r'#define KCOLORSCHEME_VERSION_STRING "([^"]+)"', header)
    return found.group(1) if found else ""


def scheme_threshold(cache: Path):
    """The grey below which Plasma calls a scheme dark: plasma-integration's determineColorScheme and the portal's
    readFdoColorScheme, read from the pins, which must agree."""
    found = []
    for sid, path, dark in [(PLASMA_INTEGRATION, HINTS, r"colorScheme = Qt::ColorScheme::Dark;"),
                            (PORTAL_KDE, PORTAL_SETTINGS, r"result = 1; // prefer dark")]:
        text = pinned.pinned(cache, sid, path).decode("utf-8")
        m = re.search(r"const int windowBackgroundGray = qGray\((?:systemPalette->|palette\.)window\(\)\.color\(\)"
                      r"\.rgb\(\)\);\s+(?:uint result = 0; // no preference\s+)?if \(windowBackgroundGray < (\d+)\) "
                      r"\{\s+" + re.escape(dark), text)
        if not m:
            sys.exit(f"{sid} {path}: no `qGray(... window() ...) < N` dark test")
        found.append(int(m.group(1)))
    if found[0] != found[1]:
        sys.exit(f"plasma-integration's threshold {found[0]} and the portal's {found[1]} differ")
    return found[0]


def breeze_backgrounds(cache: Path):
    """Every BackgroundNormal value Breeze 6.8.2's three schemes write, as written."""
    values = []
    for path in ["colors/BreezeLight.colors", "colors/BreezeDark.colors", "colors/BreezeClassic.colors"]:
        text = pinned.pinned(cache, BREEZE, path).decode("utf-8")
        values += re.findall(r"^BackgroundNormal=(.*)$", text, re.M)
    return values


def readers_build(work: Path):
    qt = [QT_INCLUDE, QT_INCLUDE / "QtCore", QT_INCLUDE / "QtGui", QT_INCLUDE / "QtCore" / QT_VERSION,
          QT_INCLUDE / "QtCore" / QT_VERSION / "QtCore", QT_INCLUDE / "QtGui" / QT_VERSION,
          QT_INCLUDE / "QtGui" / QT_VERSION / "QtGui"]
    kf6 = [KF6_INCLUDE / "KColorScheme", KF6_INCLUDE / "KConfig", KF6_INCLUDE / "KConfigCore",
           KF6_INCLUDE / "KConfigGui"]
    exe = work / "readers"
    pinned.sandboxed(["g++", "-std=c++20", "-fPIC", "-O2", *(f"-I{p}" for p in [*qt, *kf6]), "-o", exe,
                      HERE / "readers.cpp", *READER_LIBS], writable=[work], check=True)
    return exe


def text_line(s):
    """A harness input line: None as "-", else hex_text."""
    return "-" if s is None else hex_text(s)


def readers_run(exe: Path, mode, inputs):
    """The harness's lines for inputs, sandboxed; the xcb mode on an Xvfb server started in the sandbox."""
    script = 'mkdir -p /tmp/w && exec "$0" "$1" /tmp/w'
    if mode == "xcb":
        script = ('mkdir -p /tmp/w && Xvfb :7 -noreset -nolisten tcp -screen 0 1280x720x24 >/tmp/xvfb.log 2>&1 & x=$!; '
                  'n=0; while [ ! -S /tmp/.X11-unix/X7 ] && [ $n -lt 400 ]; do sleep 0.05; n=$((n+1)); done; '
                  'DISPLAY=:7 "$0" "$1" /tmp/w; s=$?; kill $x; wait $x 2>/dev/null; exit $s')
    lines = [text_line(s) for s in inputs]
    out = pinned.sandboxed(["sh", "-c", script, exe, mode], readable=[exe.parent],
                           input="".join(x + "\n" for x in lines), capture_output=True, text=True, check=False)
    if out.returncode != 0:
        sys.exit(f"{exe.name} {mode}: exit {out.returncode}: {out.stderr.strip()[-2000:]}")
    rows = out.stdout.splitlines()
    if len(rows) != len(lines):
        sys.exit(f"{exe.name} {mode}: {len(rows)} results for {len(lines)} inputs")
    return [r.split("\t") for r in rows]


def option_string_cell(s):
    return "none" if s is None else string_cell(s)


def threshold_neighbours(threshold):
    """Colours on both sides of qGray's threshold: greys, one channel stepped across it with the others held, and
    weighted sums just below the threshold that rounding the mean (not truncating it) would call light."""
    limit = 32 * threshold          # qGray(r, g, b) < threshold exactly where 11r + 16g + 5b < 32 * threshold
    rows = [(threshold - 2,) * 3, (threshold - 1,) * 3, (threshold,) * 3, (threshold + 1,) * 3]
    weights = (11, 16, 5)
    for held in [threshold, 255, 200, 128, 0]:
        for i in range(3):
            rest = sum(w * held for k, w in enumerate(weights) if k != i)
            need = -(-(limit - rest) // weights[i])   # the least channel value reaching the limit
            if 1 <= need <= 255:
                below = [held] * 3
                below[i] = need - 1
                above = list(below)
                above[i] = need
                rows += [tuple(below), tuple(above)]
    for target in [limit - 1, limit - 16, limit]:
        hit = next(((r, g, b) for r in range(255, -1, -1) for g in range(256)
                    for b in [(target - 11 * r - 16 * g) // 5] if 0 <= b <= 255 and 11 * r + 16 * g + 5 * b == target),
                   None)
        if hit:
            rows.append(hit)
    out = []
    for c in rows:
        if c not in out:
            out.append(c)
    return out


def kde_color_scheme_table(source_cache: Path, work: Path, rng):
    threshold = scheme_threshold(source_cache)
    exe = readers_build(work)
    entries = [None, *dict.fromkeys(breeze_backgrounds(source_cache)), "0,0,0", "255,255,255", "0,0,0,0",
               "255,255,255,0", DARK, "239,240,241"]
    entries += [",".join(map(str, c)) for c in threshold_neighbours(threshold)]
    entries += KCONFIG_FORMS
    for k in range(RANDOM_SCHEME):
        c = [rng.randrange(256) for _ in range(3)]
        while k % 2:   # half of them within a few steps of the threshold
            green = (32 * threshold - 11 * c[0] - 5 * c[2]) // 16 + rng.randrange(-2, 3)
            if 0 <= green <= 255:
                c[1] = green
                break
            c = [rng.randrange(256) for _ in range(3)]
        entries.append(",".join(map(str, c)))
    entries = list(dict.fromkeys(entries)) + [s for s in KCONFIG_QT_PARSED]
    palette = readers_run(exe, "palette", entries)
    plasma = readers_run(exe, "plasma", entries)
    rows, left_out = [], []
    for s, p, q in zip(entries, palette, plasma):
        has_key, read_back, valid, gray = p[0] == "1", p[1], p[2] == "1", int(p[7])
        if has_key != (s is not None) or (s is not None and unhex_text(read_back) != s):
            sys.exit(f"kdeglobals does not hold the input {s!r}: read back {read_back}")
        scheme = "dark" if gray < threshold else "light"
        if QT_COLOR_SCHEME.get(int(q[0])) != scheme or q[1:] != p[3:6]:
            sys.exit(f"{s!r}: plasma-integration's platform theme answers {q}, the palette {p}")
        if s is not None and (s in ("", "invalid") or s.startswith("#") or "," not in s):
            left_out.append((s, scheme, valid, p[3:6]))
        else:
            rows.append([option_string_cell(s), scheme])
    write_table(
        VECTORS / "reader.kde_color_scheme.tsv",
        ["# function: reader.kde_color_scheme",
         "# route: public_api",
         f"# source: {PLASMA_INTEGRATION}",
         f"# source: {PORTAL_KDE}",
         "# source: kcolorscheme@6.31.0",
         "# source: kconfig@6.31.0",
         "# source: qtbase@6.12.0",
         "# generations: kde-breeze",
         "# generator: tools/kde-vectors",
         "# inputs: background_normal:option<string>",
         "# outputs: scheme:enum<Scheme>",
         f"# written by tools/kde-vectors/generate.py on Qt {qt_version()}, KConfig {kconfig_version()} and",
         f"# KColorScheme {kcolorscheme_version()}; do not edit. Each row: kdeglobals' [Colors:Window]",
         "# BackgroundNormal (a JSON string literal, the value KConfig holds; `none`: the file has no such",
         "# key), written by KConfig into a kdeglobals with a [Colors:View] group, KColorScheme::",
         "# createApplicationPalette of it, and Plasma's test of the palette's active window colour: dark where",
         f"# qGray(rgb()) < {threshold} (the threshold read from plasma-integration's determineColorScheme and the",
         "# portal's readFdoColorScheme at their pins), else light. Each row is checked against the system's",
         "# plasma-integration platform theme (QStyleHints::colorScheme() on the offscreen platform over the same",
         "# kdeglobals). The inputs: unset, the BackgroundNormal values of Breeze 6.8.2's three schemes, black,",
         "# white and alpha, colours on both sides of the threshold, a dark colour's entry in each form KConfig",
         "# accepts or refuses as a colour, seeded random colours. Left out: "
         f"{len(left_out)} values KConfig reads as an",
         "# invalid colour or through Qt's colour parser, which S9.3 (ADD-A140) has the reader read as unset",
         "# (its default, light), with Plasma's answer and the colour:",
         *(f"# {string_cell(s)}: {scheme} ({'valid' if valid else 'invalid'}, rgb {','.join(c)})"
           for s, scheme, valid, c in left_out)],
        ["background_normal", "scheme"],
        rows,
    )
    print(f"reader.kde_color_scheme.tsv: {len(rows)} rows ({len(left_out)} left out)")
    return exe


# X resources: lines `<name>:\t<value>` as xrdb writes them, each value varied as Qt's xcb plugin reads Xft.dpi.
XFT_TAIL = "Xft.hinting:\t1\nXft.hintstyle:\thintslight\nXft.rgba:\trgb\n"
XFT_VALUES = ["96", "120", "144", "192", "288", "72", "1", "0", "-96", "+96", "096", " 96", "\t96", "96 ", "96\t",
              "96\r", "96\v", "96\f", "96.0", "96.4", "96.5", "95.5", "97.5", "96.49999999999999", "0.5", "0.4",
              "0.49999999999999994", "-0.4", "-0.5", "-1.5", "1e2", "1E2", "1e+2", "9.6e1", "96.", ".5e3", "+96.5",
              " 96.5 ", "96,5", "9 6", "96 dpi", "96dpi", "0x60", "0x60.0", "abc", "", "1e", "1e400", "1e-400",
              "0e5", "inf", "-inf", "nan", "Infinity", "2147483647", "2147483648", "-2147483648", "2147483647.4",
              "2147483647.5", "-2147483648.4", "99999999999", "1e10", "96 ", "٩٦",
              "96.000000"]
XFT_RESOURCES = [
    "", "Xft.dpi:\t96\n", "Xft.dpi:\t96", "Xcursor.size:\t24\nXcursor.theme:\tbreeze_cursors\nXft.antialias:\t1\n"
    "Xft.dpi:\t96\n" + XFT_TAIL, "Xcursor.size:\t24\nXft.antialias:\t1\n" + XFT_TAIL,
    "Xft.dpi: 96\n", "Xft.dpi:96\n", "Xft.dpi:  96\n", "xft.dpi:\t96\n", "Xft.DPI:\t96\n", "Xft.dpi :\t96\n",
    " Xft.dpi:\t96\n", "*.dpi:\t96\n", "*dpi:\t96\n", "Xft*dpi:\t96\n", "Xft.dpix:\t96\n", "XXft.dpi:\t96\n",
    "Xft.dpi:\t96\nXft.dpi:\t120\n", "Xft.dpi:\t120\nXft.dpi:\tabc\n", "Xft.dpi:\t120\nXft.dpi:\t0\n",
    "Xft.dpi:\t120\nXft.dpi:\t-1\n", "Xft.dpi:\t120\nXft.dpi:\t\n", "Xft.dpi:\t120\nXft.dpi: 144\n",
    "Xft.dpi:\t0\nXft.dpi:\t120\n", "Xft.dpi:\t120\r\nXft.hinting:\t1\r\n", "Xft.dpi:\t120\n\n\n",
    "\nXft.dpi:\t120", "Xft.dpi:\t120\\\nXft.hinting:\t1\n", "! Xft.dpi:\t96\n", "Xft.dpi:\t144\n" + XFT_TAIL,
    "Xft.antialias:\t1\nXft.dpi:\t96.5\n", "Xft.dpi:\t1e400\nXft.dpi:\t1e2\n", "Xft.dpi:\t120\nXft.dpi:\tnan\n",
]


def xrdb_version():
    out = subprocess.run([XRDB, "-version"], capture_output=True, text=True, check=False)
    return out.stdout.strip() or "version unknown"


def qxcb_xft_dpi_table(exe: Path):
    inputs = list(dict.fromkeys(XFT_RESOURCES + [f"Xft.dpi:\t{v}\n" for v in XFT_VALUES]))
    results = readers_run(exe, "xcb", inputs)
    rows, differing = [], []
    for s, r in zip(inputs, results):
        forced, logical, xrdb = int(r[0]), float(r[1]), r[2]
        if forced < 0 or logical != (forced if forced > 0 else 96):
            sys.exit(f"{s!r}: forcedDpi {forced} and logicalDpi {logical} disagree")
        if xrdb == "!" or (unhex_text(xrdb) or "") != s:
            differing.append((s, xrdb))
        rows.append([string_cell(s), str(forced) if forced > 0 else "none"])
    write_table(
        VECTORS / "reader.qxcb_xft_dpi.tsv",
        ["# function: reader.qxcb_xft_dpi",
         "# route: public_api",
         "# source: qtbase@6.12.0",
         "# generations: kde-breeze",
         "# generator: tools/kde-vectors",
         "# inputs: resources:string",
         "# outputs: dpi:option<number>",
         f"# written by tools/kde-vectors/generate.py on Qt {qt_version()}; do not edit. Each row: the X resources",
         "# (a JSON string literal), set as the root window's RESOURCE_MANAGER property of an Xvfb server (no",
         "# property for the empty string), and the DPI Qt's xcb plugin forces from them at start: QXcbScreen::",
         "# forcedDpi() of the system's libQt6XcbQpa, `none` where it forces none and Qt's logical DPI is its",
         "# fallback (every row's QPlatformScreen::logicalDpi() checked to be the forced DPI, else 96). The server",
         "# runs no XSETTINGS manager, so no Xft/DPI setting overrides the resources. The inputs: resources as xrdb",
         "# writes them, the Xft.dpi line's name spelt otherwise, several Xft.dpi lines, and one line's value in the",
         "# forms an integer and a decimal take, whitespace, signs, rounding, ranges and junk.",
         *([f"# `xrdb -query` ({xrdb_version()}), which the reader reads the resources from, prints each input",
            "# back unchanged."]
           if not differing else
           [f"# `xrdb -query` ({xrdb_version()}), which the reader reads the resources from, prints {len(differing)}",
            f"# of the {len(inputs)} inputs otherwise (the input, then what xrdb prints):",
            *(f"# {string_cell(s)} -> {'(xrdb failed)' if x == '!' else string_cell(unhex_text(x) or '')}"
              for s, x in differing)])],
        ["resources", "dpi"],
        rows,
    )
    print(f"reader.qxcb_xft_dpi.tsv: {len(rows)} rows ({len(differing)} that xrdb prints otherwise)")


def readers_tables(source_cache: Path, work: Path, rng):
    if not kcolorscheme_version().startswith(KCOLORSCHEME_MINOR + "."):
        sys.exit(f"KF6 ColorScheme {kcolorscheme_version() or '(none)'} found; the pin is KColorScheme "
                 f"{KCOLORSCHEME_MINOR}")
    for tool in (XVFB, XRDB):
        if subprocess.run(["sh", "-c", f"command -v {tool}"], capture_output=True, check=False).returncode != 0:
            sys.exit(f"{tool} not found: the xcb plugin's table runs it")
    exe = kde_color_scheme_table(source_cache, work, rng)
    qxcb_xft_dpi_table(exe)


# ---- The accent derivation: plasma-workspace's applyScheme() (`kde_accent`, `kde_accent_wm`), by the
# `compiled_source` route. kcms/colors/colorsapplicator.cpp and .h compiled unchanged with accent.cpp over the
# system's Qt 6.12.0, KConfig, KGuiAddons and KColorScheme 6.31; each case's kdeglobals read back through
# KColorScheme as the KDE reader reads kdeglobals.

PLASMA_WORKSPACE = "plasma-workspace@6.8.2"
APPLICATOR = ["kcms/colors/colorsapplicator.cpp", "kcms/colors/colorsapplicator.h"]
KGUIADDONS_MINOR = "6.31"
ACCENT_LIBS = ["-lKF6ColorScheme", "-lKF6GuiAddons", "-lKF6ConfigGui", "-lKF6ConfigCore", "-lQt6Gui", "-lQt6DBus",
               "-lQt6Core"]
ACCENT_TABLE_SOURCES = [PLASMA_WORKSPACE, "kcolorscheme@6.31.0", "kconfig@6.31.0", "kguiaddons@6.31.0",
                        "qtbase@6.12.0"]
ACCENT_FACTS = ROOT / "docs" / "facts" / "kde" / "accent.toml"
SUSE_CAFE = "SUSE-Cafe@2f66c168"
# applyScheme()'s twelve keys (colorsapplicator.cpp:165-176), in KdeSchemeKey's order (S4.8), and the four [WM]
# keys KdeWmKey names (S4.8; colorsapplicator.cpp:291-294).
SCHEME_KEYS = ["BackgroundNormal", "BackgroundAlternate", "ForegroundNormal", "ForegroundInactive", "ForegroundActive",
               "ForegroundLink", "ForegroundVisited", "ForegroundNegative", "ForegroundNeutral", "ForegroundPositive",
               "DecorationFocus", "DecorationHover"]
WM_KEYS = ["activeBackground", "activeForeground", "inactiveBackground", "inactiveForeground"]
# The facts' role field of each scheme key (the catalogue's key table, as docs/facts/kde/accent.toml names them).
ROLE_FIELDS = dict(zip(SCHEME_KEYS, ["background", "background_alternate", "foreground", "foreground_inactive",
                                     "foreground_active", "link", "link_visited", "negative", "neutral", "positive",
                                     "decoration_focus", "decoration_hover"]))
ACCENT_SEED = 20261109
RANDOM_ACCENTS = 12
RANDOM_ACCENT_ROWS = 12
# A case's later accent keeps at most this many of the rows its result changes (seeded).
LATER_ROWS = 16


def snake(name):
    return re.sub(r"(?<!^)([A-Z])", r"_\1", name).lower()


def accent_build(cache: Path, work: Path):
    qt = [QT_INCLUDE, QT_INCLUDE / "QtCore", QT_INCLUDE / "QtGui", QT_INCLUDE / "QtDBus",
          QT_INCLUDE / "QtCore" / QT_VERSION, QT_INCLUDE / "QtCore" / QT_VERSION / "QtCore",
          QT_INCLUDE / "QtGui" / QT_VERSION, QT_INCLUDE / "QtGui" / QT_VERSION / "QtGui"]
    kf6 = [KF6_INCLUDE / "KColorScheme", KF6_INCLUDE / "KConfig", KF6_INCLUDE / "KConfigCore",
           KF6_INCLUDE / "KConfigGui", KF6_INCLUDE / "KGuiAddons"]
    colors = cache / PLASMA_WORKSPACE / "kcms" / "colors"
    common = ["g++", "-std=c++20", "-fPIC", *REFERENCE_FLAGS, "-DQT_NO_DEBUG",
              *(f"-I{p}" for p in [colors, *qt, *kf6])]
    objects = []
    for source in [colors / "colorsapplicator.cpp", HERE / "accent.cpp"]:
        obj = work / f"accent-{source.stem}.o"
        pinned.sandboxed([*common, "-c", source, "-o", obj], writable=[work], check=True)
        objects.append(obj)
    exe = work / "accent"
    pinned.sandboxed(["g++", *REFERENCE_FLAGS, "-o", exe, *objects, *ACCENT_LIBS], writable=[work], check=True)
    return exe


def colour_text(text):
    """A harness colour, `invalid` or `r,g,b,a` (QColor's redF() … alphaF()), as the model holds it (f32s)."""
    return None if text == "invalid" else tuple(f32(float(x)) for x in text.split(","))


def accent_run(exe: Path, cases):
    """Each case (scheme bytes, accent: None or four f32s) applied: {"lists", "in", "wmin", "out", "wm", "raw"}."""
    lines = []
    for scheme, accent in cases:
        arg = "-" if accent is None else ",".join(f32_text(x) for x in accent)
        lines.append(f"{scheme.hex()} {arg}")
    out = pinned.sandboxed([exe], readable=[exe.parent], input="".join(x + "\n" for x in lines), capture_output=True,
                           text=True, check=True).stdout
    results, current = [], None
    for line in out.splitlines():
        f = line.split("\t")
        if current is None:
            current = {"in": {}, "out": {}, "wmin": {}, "wm": {}}
        if f[0] == "lists":
            current["groups"] = [] if f[1] == "-" else f[1].split(",")
            current["inactive_groups"] = [] if f[2] == "-" else f[2].split(",")
            current["unset"] = [] if f[3] == "-" else f[3].split(",")
            current["tint"] = None if f[4] == "-" else f32(float(f[4]))
            current["titlebar"] = f[5] == "1"
        elif f[0] in ("in", "out"):
            current[f[0]][(f[1], f[2])] = (colour_text(f[3]), colour_text(f[4]))
        elif f[0] in ("wmin", "wm"):
            current[f[0]][f[1]] = colour_text(f[2])
        elif f[0] == "raw":
            current["raw"] = bytes.fromhex(f[1]).decode("utf-8")
        elif f[0] == "end":
            results.append(current)
            current = None
    if len(results) != len(cases):
        sys.exit(f"{exe.name}: {len(results)} results for {len(cases)} cases")
    return results


# A scheme file as an ordered list of [group header, [(key, value)]], the preamble's lines under the header None.
def scheme_parse(text):
    groups = [[None, []]]
    for line in text.splitlines():
        s = line.strip()
        if s.startswith("[") and s.endswith("]"):
            groups.append([s[1:-1], []])
        elif s and not s.startswith("#") and "=" in s:
            k, v = s.split("=", 1)
            groups[-1][1].append((k, v))
    return groups


def scheme_text(groups):
    out = []
    for name, entries in groups:
        if name is not None:
            out.append(f"[{name}]")
        out += [f"{k}={v}" for k, v in entries]
        out.append("")
    return "\n".join(out).encode("utf-8")


def scheme_edit(groups, *, drop_groups=(), drop_keys=(), set_keys=(), add_groups=()):
    """A copy with whole groups dropped, (group, key)s dropped, (group, key, value)s set (the group appended where
    missing) and (group, entries) groups appended."""
    out = [[n, list(e)] for n, e in groups if n not in drop_groups]
    for g, k in drop_keys:
        for entry in out:
            if entry[0] == g:
                entry[1] = [(a, b) for a, b in entry[1] if a != k]
    for g, k, v in set_keys:
        entry = next((e for e in out if e[0] == g), None)
        if entry is None:
            entry = [g, []]
            out.append(entry)
        entry[1] = [(a, b) for a, b in entry[1] if a != k] + [(k, v)]
    for g, entries in add_groups:
        out.append([g, list(entries)])
    return out


def hex8(c):
    return tuple(round(x * 255) for x in c)


def accent_cases(cache: Path):
    """The cases: (name, scheme bytes, the accents, the sets whose rows are kept (None: all), the [WM] rows kept)."""
    light = scheme_parse(pinned.pinned(cache, BREEZE, "colors/BreezeLight.colors").decode("utf-8"))
    dark = scheme_parse(pinned.pinned(cache, BREEZE, "colors/BreezeDark.colors").decode("utf-8"))
    classic = scheme_parse(pinned.pinned(cache, BREEZE, "colors/BreezeClassic.colors").decode("utf-8"))
    yellow, violet, none = c8(229, 165, 10), c8(94, 44, 165), c8(0, 0, 0, 0)
    measured = [yellow, violet, none]
    more = [c8(61, 174, 233), c8(255, 255, 255), c8(0, 0, 0), c8(255, 0, 0, 0), c8(61, 174, 233, 128),
            (f32(0.2), f32(0.55), f32(0.9), ONE), (f32(0.0), f32(0.0), f32(0.0), f32(1e-6))]
    tint = ("General", "TintFactor")
    titlebar = ("General", "TitlebarIsAccentColored", "true")
    header_inactive = "Colors:Header][Inactive"
    window_bg = ("Colors:Window", "BackgroundNormal")
    cases = [
        ("BreezeLight", light, measured + more, None, True),
        ("BreezeDark", dark, measured + more[1:3] + more[4:5], None, True),
        ("BreezeClassic (the title-bar option, no Header group)", classic, measured + more[:2], None, True),
        ("BreezeDark tinting (TintFactor 0.15)", scheme_edit(dark, set_keys=[(*tint, "0.15")]),
         measured + more[:2], None, True),
        ("BreezeLight tinting (0.3) with the title-bar option",
         scheme_edit(light, set_keys=[(*tint, "0.3"), titlebar]), measured, None, True),
        ("BreezeLight, the title-bar option by accentActiveTitlebar",
         scheme_edit(light, set_keys=[("General", "accentActiveTitlebar", "true")]), [yellow], ["Header"], True),
        ("BreezeLight, TitlebarIsAccentColored false over accentActiveTitlebar true",
         scheme_edit(light, set_keys=[("General", "accentActiveTitlebar", "true"),
                                      ("General", "TitlebarIsAccentColored", "false")]), [yellow], ["Header"], True),
        ("BreezeDark, the title-bar option, a Header group without [Inactive]",
         scheme_edit(dark, drop_groups=[header_inactive], set_keys=[titlebar]), measured, ["Header", "Window"], True),
        ("a SUSE-Cafe-shaped file: tinting 0.15, the title-bar option, a Header group of two keys",
         scheme_edit(dark, drop_groups=["Colors:Header", header_inactive],
                     set_keys=[(*tint, "0.15"), titlebar],
                     add_groups=[("Colors:Header", [("BackgroundNormal", "41,44,48"),
                                                    ("BackgroundAlternate", "49,54,59")])]),
         measured, None, True),
        ("BreezeLight without a Header group", scheme_edit(light, drop_groups=["Colors:Header", header_inactive]),
         [yellow, violet, none], ["Header", "Window"], True),
        ("BreezeClassic tinting (0.2)", scheme_edit(classic, set_keys=[(*tint, "0.2")]), [yellow, none],
         ["Header", "Window"], True),
        ("BreezeLight without View and Button groups",
         scheme_edit(light, drop_groups=["Colors:View", "Colors:Button"]), [yellow, violet, none],
         ["View", "Button", "Selection"], False),
        ("BreezeLight without View and Button groups, tinting 0.15",
         scheme_edit(light, drop_groups=["Colors:View", "Colors:Button"], set_keys=[(*tint, "0.15")]),
         [yellow, none], ["View", "Button", "Selection"], False),
        ("BreezeLight without a Window group, the title-bar option",
         scheme_edit(light, drop_groups=["Colors:Window"], set_keys=[titlebar]), [yellow, violet, none],
         ["Window", "Header"], True),
        ("BreezeLight without a Window group, tinting 0.15",
         scheme_edit(light, drop_groups=["Colors:Window"], set_keys=[(*tint, "0.15")]), [yellow, none],
         ["Window", "Header"], True),
        ("BreezeClassic without a Window group", scheme_edit(classic, drop_groups=["Colors:Window"]),
         [yellow, none], ["Window", "Header"], True),
        ("BreezeDark with unset keys",
         scheme_edit(dark, drop_keys=[("Colors:View", "BackgroundNormal"), ("Colors:Window", "ForegroundLink"),
                                      ("Colors:Window", "DecorationHover"), ("Colors:Header", "ForegroundNormal"),
                                      ("Colors:Header", "BackgroundNormal"), ("Colors:Selection", "ForegroundActive"),
                                      ("Colors:Tooltip", "ForegroundLink"), (header_inactive, "BackgroundAlternate"),
                                      (header_inactive, "ForegroundLink"), ("Colors:Button", "BackgroundNormal")]),
         [yellow, violet, none], None, True),
        ("BreezeDark with unset keys, tinting 0.15, the title-bar option",
         scheme_edit(dark, drop_keys=[("Colors:View", "BackgroundNormal"), ("Colors:Window", "ForegroundLink"),
                                      ("Colors:Window", "BackgroundNormal"), ("Colors:Header", "ForegroundNormal"),
                                      ("Colors:Selection", "ForegroundActive"), (header_inactive, "ForegroundLink"),
                                      (header_inactive, "BackgroundAlternate")],
                     set_keys=[(*tint, "0.15"), titlebar]),
         [yellow, none], None, True),
        ("BreezeLight with [Inactive] groups in Window (four keys) and View",
         scheme_edit(light, add_groups=[
             ("Colors:Window][Inactive", [("BackgroundNormal", "230,231,232"), ("ForegroundNormal", "60,63,66"),
                                          ("ForegroundLink", "20,100,160"), ("DecorationFocus", "200,0,0")]),
             ("Colors:View][Inactive", [(k, "250,250,250" if k.startswith("Background") else "50,50,50")
                                        for k in SCHEME_KEYS])]),
         [yellow, none], ["Window", "View", "Header"], False),
        ("the same, tinting 0.15",
         scheme_edit(light, set_keys=[(*tint, "0.15")], add_groups=[
             ("Colors:Window][Inactive", [("BackgroundNormal", "230,231,232"), ("ForegroundNormal", "60,63,66"),
                                          ("ForegroundLink", "20,100,160"), ("DecorationFocus", "200,0,0")])]),
         [yellow, none], ["Window", "Header"], False),
        ("BreezeLight, Window without its backgrounds (the Header table's)",
         scheme_edit(light, drop_keys=[window_bg, ("Colors:Window", "BackgroundAlternate")]),
         [yellow, none], ["Window", "Header"], True),
        ("the same with the title-bar option and no Header [Inactive]",
         scheme_edit(light, drop_keys=[window_bg, ("Colors:Window", "BackgroundAlternate")],
                     drop_groups=[header_inactive], set_keys=[titlebar]),
         [yellow, none], ["Window", "Header"], True),
        ("BreezeLight, a Header [Inactive] but no Window group",
         scheme_edit(light, drop_groups=["Colors:Window"]), [yellow, none], ["Header"], False),
        ("translucent backgrounds (View, Button, Window, Header)",
         scheme_edit(light, set_keys=[("Colors:View", "BackgroundNormal", "255,255,255,128"),
                                      ("Colors:Button", "BackgroundNormal", "252,252,252,0"),
                                      ("Colors:Window", "BackgroundNormal", "239,240,241,200"),
                                      ("Colors:Header", "BackgroundNormal", "222,224,226,100"), titlebar]),
         [yellow, violet], ["Selection", "Button", "Header", "View", "Window"], True),
        ("a Header group only in its [Inactive] subgroup, an empty Tooltip group",
         scheme_edit(light, drop_groups=["Colors:Header", "Colors:Tooltip"],
                     add_groups=[("Colors:Tooltip", []),
                                 ("Colors:Header][Inactive", [("BackgroundNormal", "200,201,202")])]),
         [yellow, none], ["Header", "Tooltip"], False),
        ("tint factor 0",
         scheme_edit(dark, set_keys=[(*tint, "0")]), [yellow], ["Window", "Header"], True),
        ("tint factor 1", scheme_edit(dark, set_keys=[(*tint, "1")]), [yellow], ["Window", "Header"], True),
        ("tint factor unreadable", scheme_edit(dark, set_keys=[(*tint, "much")]), [yellow], ["Header"], True),
        ("tint factor empty", scheme_edit(dark, set_keys=[(*tint, "")]), [violet], ["Header"], True),
        ("BreezeDark tinting (0.15), Window colours with a zero red or blue channel",
         scheme_edit(dark, set_keys=[(*tint, "0.15"), ("Colors:Window", "ForegroundLink", "0,128,255"),
                                     ("Colors:Window", "ForegroundPositive", "0,200,0"),
                                     ("Colors:Window", "ForegroundNeutral", "255,128,0"),
                                     ("Colors:Window", "BackgroundNormal", "0,0,0"),
                                     ("Colors:Window", "DecorationFocus", "0,0,255")]),
         [c8(0, 128, 255), c8(0, 255, 0), yellow], ["Window"], True),
        ("BreezeDark without a [WM] group, the inactive effects on (the start values KColorScheme gives)",
         scheme_edit(dark, drop_groups=["WM"], set_keys=[("ColorEffects:Inactive", "Enable", "true")]),
         [yellow, violet, none], [], True),
        ("the same, tinting 0.15, the title-bar option",
         scheme_edit(dark, drop_groups=["WM"], set_keys=[("ColorEffects:Inactive", "Enable", "true"),
                                                         (*tint, "0.15"), titlebar]),
         [yellow, none], [], True),
        ("a [WM] group of other values",
         scheme_edit(classic, set_keys=[("WM", "activeBackground", "10,20,30"),
                                        ("WM", "inactiveForeground", "200,100,50,128"), (*tint, "0.25")]),
         [yellow, none], [], True),
    ]
    return [(name, scheme_text(groups), accents, sets, wm) for name, groups, accents, sets, wm in cases]


def accent_lists(r):
    groups = [snake(s) for s in SETS if s in r["groups"]]
    inactive = [snake(s) for s in SETS if s in r["inactive_groups"]]
    unset = []
    for s in SETS:
        for part in ("", "inactive."):
            unset += [f"{snake(s)}.{part}{snake(k)}" for k in SCHEME_KEYS if f"{s}.{part}{k}" in r["unset"]]
    return [json.dumps(",".join(x)) for x in (groups, inactive, unset)]


def accent_rows(case, accent, r, sets, wm):
    """The case's kde_accent and kde_accent_wm rows: (inputs, result) cell lists."""
    groups, inactive_groups, unset = accent_lists(r)
    tint = "none" if r["tint"] is None else f32_text(r["tint"])
    titlebar = bool_cell(r["titlebar"])
    a_rows, wm_rows = [], []
    for s in SETS:
        if sets is not None and s not in sets:
            continue
        for k in SCHEME_KEYS:
            for g, i in (("active", 0), ("inactive", 1)):
                result = r["out"][(s, k)][i]
                if result is None:
                    sys.exit(f"{case}: {s} {k} {g} reads as an invalid colour")
                inputs = [snake(s), snake(k), g, colour_cell(accent), colour_cell(r["in"][(s, k)][0]),
                          colour_cell(r["in"][(s, k)][1]), colour_cell(r["in"][(s, "BackgroundNormal")][0]),
                          colour_cell(r["in"][("Window", k)][0]),
                          colour_cell(r["in"][("Window", "BackgroundNormal")][0]),
                          colour_cell(r["in"][("View", "BackgroundNormal")][0]),
                          groups, inactive_groups, unset, tint, titlebar]
                a_rows.append((inputs, srgb(result)))
    if wm:
        for k in WM_KEYS:
            result = r["wm"][k]
            if result is None:
                sys.exit(f"{case}: [WM] {k} reads as an invalid colour")
            inputs = [snake(k), colour_cell(accent), colour_cell(r["wmin"][k]),
                      colour_cell(r["in"][("Window", "BackgroundNormal")][0]), groups, unset, tint, titlebar]
            wm_rows.append((inputs, srgb(result)))
    return a_rows, wm_rows


def accent_facts_check(cache: Path, exe: Path):
    """The measured (scheme file, accent) pairs of accent.toml re-run: each fact's value equals the harness's (8-bit),
    or the run stops naming the differences. Returns the numbers of pairs, facts and comparisons."""
    facts = tomllib.loads(ACCENT_FACTS.read_text(encoding="utf-8"))["fact"]
    pairs = {}
    for f in facts:
        pairs.setdefault((f["scheme_file"], f["accent"]), []).append(f)
    cases, keys = [], []
    for (scheme_file, accent), group in pairs.items():
        sid, path = scheme_file.split(" ", 1)
        text = pinned.pinned(cache, sid, path)
        a = tuple(int(accent[i:i + 2], 16) for i in range(1, len(accent), 2))
        colour = c8(*a)
        runs = [colour] if a[3:] != (0,) else [None, colour]   # no AccentColor as measured, and #00000000
        for run_accent in runs:
            cases.append((text, run_accent))
            keys.append((scheme_file, accent, run_accent, group))
    results = accent_run(exe, cases)
    wrong, compared = [], 0
    for (scheme_file, accent, run_accent, group), r in zip(keys, results):
        for f in group:
            parts = f["key"].split(".")
            if parts[0] == "role" and parts[1].startswith("@wm_"):
                got = r["wm"][WM_KEYS[[snake(k) for k in WM_KEYS].index(parts[1][4:])]]
            elif parts[0] == "role":
                key = next(k for k, v in ROLE_FIELDS.items() if v == parts[2])
                got = r["out"][(SETS[[snake(s) for s in SETS].index(parts[1])], key)][0]
            else:
                key = next(k for k in SCHEME_KEYS if snake(k) == parts[3])
                got = r["out"][(SETS[[snake(s) for s in SETS].index(parts[2])], key)][1]
            want = f["value"].lower()
            have = None if got is None else hex_color(hex8(got))
            exact = got is not None and all(abs(x * 255 - round(x * 255)) < 1e-4 for x in got)
            compared += 1
            if have != want or not exact:
                wrong.append(f"{scheme_file} {accent} (run with {'no AccentColor' if run_accent is None else accent}): "
                             f"{f['key']} is {want} in the facts, {have} here")
    if wrong:
        sys.exit("the measured pairs differ from the harness:\n" + "\n".join(wrong))
    return len(pairs), len(facts), compared


def accent_tables(source_cache: Path, work: Path, rng):
    if not kcolorscheme_version().startswith(KCOLORSCHEME_MINOR + "."):
        sys.exit(f"KF6 ColorScheme {kcolorscheme_version() or '(none)'} found; the pin is KColorScheme "
                 f"{KCOLORSCHEME_MINOR}")
    if not kguiaddons_version().startswith(KGUIADDONS_MINOR + "."):
        sys.exit(f"KF6 GuiAddons {kguiaddons_version() or '(none)'} found; the pin is KGuiAddons {KGUIADDONS_MINOR}")
    cache = work / "accent-sources"
    pinned.materialise(source_cache, [(PLASMA_WORKSPACE, p) for p in APPLICATOR], cache)
    exe = accent_build(cache, work)
    pairs, n_facts, compared = accent_facts_check(source_cache, exe)
    # The transforms' constants as the system's Qt computed them (the behavioural description quotes them).
    print(pinned.sandboxed([exe, "constants"], readable=[exe.parent], capture_output=True, text=True,
                           check=True).stdout, end="")
    cases = []
    for name, scheme, accents, sets, wm in accent_cases(source_cache):
        cases += [(name, scheme, a, sets, wm) for a in accents]
    # Seeded random accents (8-bit and 16-bit) over the first five cases' schemes, some of their rows kept.
    mains = accent_cases(source_cache)[:5]
    for _ in range(RANDOM_ACCENTS):
        name, scheme, _, _, _ = rng.choice(mains)
        accent = c8(*(rng.randrange(256) for _ in range(3))) if rng.random() < 0.7 else \
            c16((*(rng.randrange(65536) for _ in range(3)), 65535))
        cases.append((name + ", a seeded random accent", scheme, accent, "random", True))
    results = accent_run(exe, [(scheme, accent) for _, scheme, accent, _, _ in cases])
    a_rows, wm_rows, seen, earlier = [], [], {}, {}
    for (name, scheme, accent, sets, wm), r in zip(cases, results):
        a, w = accent_rows(name, accent, r, None if sets == "random" else sets, wm)
        if sets == "random":
            a = rng.sample(a, RANDOM_ACCENT_ROWS)
        else:
            # A later accent of the case: only the rows whose result differs from every earlier accent's.
            later = name in earlier
            before = earlier.setdefault(name, {})
            kept = []
            for table, rows in (("a", a), ("w", w)):
                out = []
                for inputs, result in rows:
                    call = (table, inputs[0], inputs[1], inputs[2]) if table == "a" else (table, inputs[0])
                    if call not in before or result not in before[call]:
                        out.append((inputs, result))
                    before.setdefault(call, set()).add(result)
                kept.append(out)
            a, w = kept
            if later and len(a) > LATER_ROWS:
                a = rng.sample(a, LATER_ROWS)
        for table, rows in (("kde_accent", a), ("kde_accent_wm", w)):
            for inputs, result in rows:
                key = (table, tuple(inputs))
                if key in seen and seen[key] != result:
                    sys.exit(f"{table}: the inputs {inputs} give {seen[key]} and {result}: the operands do not "
                             "determine the result")
                if key in seen:
                    continue
                seen[key] = result
                (a_rows if table == "kde_accent" else wm_rows).append(inputs + [result])
    names = [f"  {name}" for name in dict.fromkeys(c[0] for c in cases if c[3] != "random")]
    built = [f"written by tools/kde-vectors/generate.py on Qt {QT_VERSION}, KConfig {kconfig_version()},",
             f"KGuiAddons {kguiaddons_version()} and KColorScheme {kcolorscheme_version()}; do not edit. Each case:",
             "a scheme file and an accent, applyScheme(file, a fresh kdeglobals, KConfig::Normal, accent)",
             "(colorsapplicator.cpp compiled unchanged), the kdeglobals synced and opened afresh."]
    cases_text = ["A row already listed is not repeated; no two rows with the same inputs differ. The",
                  f"{pairs} measured (file, accent) pairs of docs/facts/kde/accent.toml ({n_facts} facts; the pairs with no",
                  "AccentColor run both as measured, with no accent argument, and with #00000000) were re-run",
                  "and equal it; SUSE-Cafe's file (GPL-2.0) is read for that check alone. The cases:",
                  *names,
                  f"then {RANDOM_ACCENTS} seeded random accents over the first five, {RANDOM_ACCENT_ROWS} rows each.",
                  "Each case's first accent gives its rows (of the sets it names); a later accent those whose",
                  f"result differs from every earlier accent's, at most {LATER_ROWS} of them (seeded)."]
    lists_text = ["the lists JSON strings, their names comma-separated (groups and inactive_groups the sets,",
                  "unset_keys the scheme inputs below input.kde_scheme, `<set>.<key>`, `<set>.inactive.<key>`);",
                  "tint_factor token.kde.tint_factor (none: the file has no TintFactor), titlebar_is_accent_colored",
                  "the token. Every operand is read over the scheme file as the KDE reader reads it."]
    write_table(
        VECTORS / "kde_accent.tsv",
        header("kde_accent", ACCENT_TABLE_SOURCES,
               "set:enum<SurfaceSet> key:enum<KdeSchemeKey> group:enum<KdeGroup> accent:colour value:colour "
               "inactive_value:colour background:colour window_value:colour window_background:colour "
               "view_background:colour groups:string inactive_groups:string unset_keys:string "
               "tint_factor:option<number> titlebar_is_accent_colored:bool",
               built + ["The result: KColorScheme's colour of the key in the set over it, the active state for",
                        "group active, the inactive one with the inactive effects and the Selection set's inactive",
                        "change off for group inactive (the set's [Inactive] value, else its active one).",
                        "The operands: accent input.accent (#00000000: none); value input.kde_scheme.<set>.<key>,",
                        "inactive_value .<set>.inactive.<key>, background .<set>.background_normal, window_value",
                        ".window.<key>, window_background .window.background_normal, view_background",
                        ".view.background_normal (KColorScheme over the scheme file: a key a group lacks from its",
                        "defaults, Header's from [Colors:Window]);"] + lists_text + cases_text),
        ["set", "key", "group", "accent", "value", "inactive_value", "background", "window_value",
         "window_background", "view_background", "groups", "inactive_groups", "unset_keys", "tint_factor",
         "titlebar_is_accent_colored", "result"],
        a_rows,
    )
    write_table(
        VECTORS / "kde_accent_wm.tsv",
        header("kde_accent_wm", ACCENT_TABLE_SOURCES,
               "key:enum<KdeWmKey> accent:colour value:colour window_background:colour groups:string "
               "unset_keys:string tint_factor:option<number> titlebar_is_accent_colored:bool",
               built + ["The result: the written [WM] key as KConfig reads a colour.",
                        "The operands: accent input.accent (#00000000: none); value input.kde_scheme.wm.<key> (the",
                        "file's [WM] key, else KColorScheme's Header colours over the file: Normal background and",
                        "foreground, Inactive background and foreground with the file's inactive effects, S9.1);",
                        "window_background input.kde_scheme.window.background_normal;"]
               + [lists_text[0].replace(" and inactive_groups", ""), *lists_text[1:]] + cases_text),
        ["key", "accent", "value", "window_background", "groups", "unset_keys", "tint_factor",
         "titlebar_is_accent_colored", "result"],
        wm_rows,
    )
    print(f"kde_accent.tsv: {len(a_rows)} rows; kde_accent_wm.tsv: {len(wm_rows)} rows; the {pairs} measured pairs' "
          f"{n_facts} facts equal ({compared} comparisons)")


KIRIGAMI = "kirigami@6.31.0"
UNITS = ["src/platform/units.cpp", "src/platform/units.h", "src/platform/platformpluginfactory.h"]
ICON_LIBS = ["-lQt6Qml", "-lQt6Gui", "-lQt6Core"]
ICON_SEED = 20261109
# The step boundaries of roundedIconSize (units.cpp:297-320): a truncated height below 16 is kept, 16 to 63 steps
# down to 16, 22, 32 or 48, 64 and above is kept.
ICON_STEPS = [16, 22, 32, 48, 64]
# The integers called directly (sizeForLabels passes roundedIconSize the truncated height, which for an integer
# height is that integer): each boundary, one below and one above, zero, negatives, and large heights up to 2^24, the
# largest integer from which every smaller one is an f32.
ICON_INTEGERS = sorted({0, 1, -1, -17, 8, 100, 128, 1000, 16_777_216} |
                       {b + d for b in ICON_STEPS for d in (-1, 0, 1)})
# The facts' input: Plasma's default font, Noto Sans 10 (plasma-integration@6.8.2
# qt6/src/platformtheme/kfontsettingsdata.cpp:44, :47), at 96 DPI, whose height the KDE render host read as 17.6875 px
# (U400; ADD-A141); then Plasma's smallest readable font, Noto Sans 8 (:53), its fixed font, Hack 10 (:48), and nearby
# sizes.
NOTO = "Noto Sans"
HACK = "Hack"
ICON_FACT = ((10.0, NOTO), 17.6875, 16)
ICON_FONTS = [(10.0, NOTO), (8.0, NOTO), (9.0, NOTO), (11.0, NOTO), (12.0, NOTO), (10.0, HACK)]
# The sweep for each boundary's both sides: Noto Sans and Hack from 1 pt to 48 pt in 1/8 pt steps. Qt sizes a font
# in whole pixels, so one family's heights lie about 1.36 px (Noto Sans) or 1.16 px (Hack) apart; Hack's fall in each
# boundary's first pixel, which Noto Sans's miss.
ICON_SWEEP = [(k / 8, family) for family in (NOTO, HACK) for k in range(8, 385)]
RANDOM_ICON = 12


def icon_build(cache: Path, work: Path):
    """The harness with Kirigami's units.cpp compiled unchanged (and its moc output), sandboxed. KIRIGAMI_BUILD_TYPE_STATIC
    is Kirigami's own static build's define (its CMakeLists.txt), under which Units::create builds the default units
    instead of asking a platform plugin; the harness never calls create."""
    platform = cache / KIRIGAMI / "src" / "platform"
    moc = pinned.sandboxed([MOC, platform / "units.h"], readable=[cache], capture_output=True, text=True,
                           check=True).stdout
    (work / "moc_units.cpp").write_text(moc, encoding="utf-8")
    qt = [QT_INCLUDE, QT_INCLUDE / "QtCore", QT_INCLUDE / "QtGui", QT_INCLUDE / "QtQml", QT_INCLUDE / "QtQuick",
          QT_INCLUDE / "QtQuickControls2"]
    includes = [f"-I{p}" for p in [HERE / "stubs", work, platform, *qt]]
    common = ["g++", "-std=c++20", "-fPIC", *REFERENCE_FLAGS, "-DQT_NO_DEBUG", "-DKIRIGAMI_BUILD_TYPE_STATIC",
              *includes]
    objects = []
    for source in [platform / "units.cpp", HERE / "icon.cpp"]:
        obj = work / f"icon-{source.stem}.o"
        pinned.sandboxed([*common, "-c", source, "-o", obj], writable=[work], check=True)
        objects.append(obj)
    exe = work / "icon"
    pinned.sandboxed(["g++", *REFERENCE_FLAGS, "-o", exe, *objects, *ICON_LIBS], writable=[work], check=True)
    return exe


def icon_run(exe: Path, calls):
    """The harness's output line for each call, sandboxed (its HOME in the sandbox's own /tmp)."""
    out = pinned.sandboxed([exe, "/tmp"], readable=[exe.parent], input="".join(c + "\n" for c in calls),
                           capture_output=True, text=True, check=False)
    if out.returncode != 0:
        sys.exit(f"{exe.name}: exit {out.returncode}: {out.stderr.strip()[-2000:]}")
    lines = out.stdout.splitlines()
    if len(lines) != len(calls):
        sys.exit(f"{exe.name}: {len(lines)} results for {len(calls)} calls")
    return lines


def icon_table(source_cache: Path, work: Path, rng):
    cache = work / "icon-sources"
    pinned.materialise(source_cache, [(KIRIGAMI, p) for p in UNITS], cache)
    exe = icon_build(cache, work)
    sweep = [f for f in ICON_SWEEP if f not in ICON_FONTS]
    random_fonts = []
    while len(random_fonts) < RANDOM_ICON:
        f = (rng.randrange(5 * 64, 40 * 64) / 64, rng.choice([NOTO, HACK]))
        if f not in ICON_FONTS and f not in ICON_SWEEP and f not in random_fonts:
            random_fonts.append(f)
    calls = ICON_FONTS + sweep + random_fonts
    fonts = {}
    for f, line in zip(calls, icon_run(exe, [f"f {pt!r} {family}" for pt, family in calls])):
        height, size = line.split()
        fonts[f] = (float(height), int(size))
    for (pt, family), (height, _) in fonts.items():
        if f32(height) != height:
            sys.exit(f"{family} {pt} pt: the height {height!r} is no f32, which the model's length is")
    f, height, size = ICON_FACT
    if fonts[f] != (height, size):
        sys.exit(f"{f[1]} {f[0]} pt at 96 DPI: height {fonts[f][0]!r}, size {fonts[f][1]}; the facts' input is "
                 f"{height!r} giving {size}")
    # Each font's result against roundedIconSize of its truncated height: a differing one would mean the units did
    # not follow the font (or the conversion is not the truncation the description states).
    truncated = sorted({int(h) for h, _ in fonts.values()} | set(ICON_INTEGERS))
    integer = dict(zip(truncated, (int(x) for x in icon_run(exe, [f"i {n}" for n in truncated]))))
    for (pt, family), (h, s) in fonts.items():
        if integer[int(h)] != s:
            sys.exit(f"{family} {pt} pt: sizeForLabels {s}, roundedIconSize({int(h)}) {integer[int(h)]}")
    # Each boundary's both sides among the sweep's heights: the largest below it, the smallest at or above it, and
    # the largest below the next integer (a fraction above the boundary truncated back to it).
    heights = sorted({h for h, _ in fonts.values()})
    boundary = []
    for b in ICON_STEPS:
        below = [h for h in heights if h < b]
        above = [h for h in heights if h >= b]
        if not below or not above:
            sys.exit(f"the sweep's heights do not reach both sides of {b}")
        boundary += [max(below), min(above), max(h for h in heights if h < b + 1)]
    by_height = {h: s for h, s in fonts.values()}
    rows, seen = [], set()

    def add(h, s):
        if h not in seen:
            seen.add(h)
            rows.append([f"{int(h) if h.is_integer() else repr(h)}px", f"{s}px"])

    for f in ICON_FONTS:
        add(*fonts[f])
    for n in ICON_INTEGERS:
        add(float(n), integer[n])
    for h in boundary:
        add(h, by_height[h])
    for f in random_fonts:
        add(*fonts[f])
    write_table(
        VECTORS / "kirigami_label_icon_size.tsv",
        ["# function: kirigami_label_icon_size",
         "# route: compiled_source",
         f"# source: {KIRIGAMI}",
         "# generations: kde-breeze",
         "# generator: tools/kde-vectors",
         "# inputs: font_height:length",
         "# outputs: size:length",
         "# written by tools/kde-vectors/generate.py; do not edit. Kirigami's units.cpp compiled unchanged. A",
         "# fractional height is IconSizes::sizeForLabels() with the application font set to Noto Sans or Hack at",
         "# some point size, the height that font's QFontMetricsF height (the system's Qt 6.12.0, offscreen, 96 DPI);",
         "# an integer height calls IconSizes::roundedIconSize with it, which is what sizeForLabels passes for an",
         "# integer height. First the facts' input (Noto Sans 10 pt, 17.6875 px), Noto Sans 8, 9, 11 and 12 pt and",
         "# Hack 10 pt, then the integers at each step boundary (16, 22, 32, 48, 64) and beside it, zero, negatives",
         "# and large heights, then each boundary's both sides from a sweep of both families, 1 pt to 48 pt by",
         "# 1/8 pt (the largest height below it, the smallest at or above it, the largest below the next integer),",
         f"# then {RANDOM_ICON} seeded random fonts (either family, a multiple of 1/64 pt from 5 pt to 40 pt)."],
        ["font_height", "size"],
        rows,
    )
    print(f"kirigami_label_icon_size.tsv: {len(rows)} rows; {ICON_FACT[0][1]} {ICON_FACT[0][0]} pt is "
          f"{ICON_FACT[1]} px, giving {ICON_FACT[2]}")


def kguiaddons_version():
    """The KF6 GuiAddons headers' `KGUIADDONS_VERSION_STRING`; empty where they are missing."""
    try:
        text = (KF6_INCLUDE / "KGuiAddons" / "kguiaddons_version.h").read_text(encoding="utf-8")
    except OSError:
        return ""
    found = re.search(r'#define KGUIADDONS_VERSION_STRING "([^"]+)"', text)
    return found.group(1) if found else ""


def main():
    if qt_version() != QT_VERSION:
        sys.exit(f"Qt {qt_version() or '(none)'} found; the pin is Qt {QT_VERSION}")
    if not kconfig_version().startswith(KCONFIG_MINOR + "."):
        sys.exit(f"KF6 Config {kconfig_version() or '(none)'} found; the pin is KConfig {KCONFIG_MINOR}")
    only = {a for a in sys.argv[1:] if a in ("--only-readers", "--only-accent", "--only-icon")}
    args = [a for a in sys.argv[1:] if a not in only]
    if len(only) > 1:
        sys.exit("--only-readers, --only-accent and --only-icon exclude each other")
    source_cache = citations.cache_dir(args[0] if args else None)
    rng = random.Random(SEED)
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        if "--only-readers" in only:
            readers_tables(source_cache, tmp, random.Random(SEED + 1))
            return
        if "--only-accent" in only:
            accent_tables(source_cache, tmp, random.Random(ACCENT_SEED))
            return
        if "--only-icon" in only:
            icon_table(source_cache, tmp, random.Random(ICON_SEED))
            return
        cache = tmp / "sources"
        pinned.materialise(source_cache, [(sid, path) for sid, paths in FILES.items() for path in paths], cache)
        reference = build(cache, tmp, REFERENCE_FLAGS, "reference")
        fused = build(cache, tmp, FUSED_FLAGS, "fused")

        lerp_inputs = [(a, b, f32(t)) for a, b, t in BREEZE_PAIRS + HUE_EDGES + ALPHA_PAIRS]
        fixed = len(lerp_inputs)
        for _ in range(RANDOM_INPUTS):
            a = tuple(rng.randrange(256) for _ in range(3))
            b = tuple(rng.randrange(256) for _ in range(3))
            lerp_inputs.append((a, b, f32(rng.random())))
        calls = [lerp_call(*row) for row in lerp_inputs]
        ref, fus = run(reference, calls), run(fused, calls)
        rows, differing = chosen(rng, ref, fus, calls, lerp_inputs, fixed, RANDOM_LERP)
        if differing == 0:
            sys.exit("no input on which a fused build differs: the reference build may fuse")
        write_table(
            VECTORS / "kirigami_lerp.tsv",
            ["# function: kirigami_lerp",
             "# route: compiled_source",
             "# source: kirigami@6.31.0",
             "# source: qtbase@6.12.0",
             "# generations: kde-breeze",
             "# generator: tools/kde-vectors",
             "# inputs: a:colour b:colour balance:number",
             "# outputs: result:colour",
             "# written by tools/kde-vectors/generate.py; do not edit. Breeze's frame inputs, the hue rule's edges,",
             "# the balance's ends, alpha, then seeded random inputs: up to 8 on which a build fusing QColor::toRgb's",
             f"# `1.0f - (s * f)` gives another colour ({differing} of {RANDOM_INPUTS}), and {RANDOM_LERP} more.",
             "# A balance is an f32, passed to Kirigami as that double."],
            ["a", "b", "balance", "result"],
            [[hex_color(lerp_inputs[i][0]), hex_color(lerp_inputs[i][1]), f32_text(lerp_inputs[i][2]),
              srgb(ref[i])] for i in rows],
        )

        shade_inputs = [(c, role, f32(k)) for c in SHADE_COLORS for k in CONTRASTS for role in SHADE_ROLES]
        fixed = len(shade_inputs)
        for _ in range(RANDOM_INPUTS):
            c = tuple(rng.randrange(256) for _ in range(3))
            shade_inputs.append((c, rng.choice(SHADE_ROLES), float(rng.randrange(11))))
        calls = [shade_call(*row) for row in shade_inputs]
        ref, fus = run(reference, calls), run(fused, calls)
        rows, differing = chosen(rng, ref, fus, calls, shade_inputs, fixed, RANDOM_SHADE)
        write_table(
            VECTORS / "kde_shade.tsv",
            ["# function: kde_shade",
             "# route: compiled_source",
             "# source: kcolorscheme@6.31.0",
             "# source: kguiaddons@6.31.0",
             "# source: qtbase@6.12.0",
             "# generations: kde-breeze",
             "# generator: tools/kde-vectors",
             "# inputs: c:colour role:enum<KdeShadeRole> contrast:number",
             "# outputs: result:colour",
             "# written by tools/kde-vectors/generate.py; do not edit. Each role at each contrast step for the",
             "# branches' colours, then seeded random inputs: up to 8 on which a build with fused multiply-adds",
             f"# gives another colour ({differing} of {RANDOM_INPUTS}), and {RANDOM_SHADE} more."],
            ["c", "role", "contrast", "result"],
            [[hex_color(shade_inputs[i][0]), shade_inputs[i][1], f32_text(shade_inputs[i][2]), srgb(ref[i])]
             for i in rows],
        )

        new_tables(rng, reference, fused, source_cache)
        qfont_table(source_cache, tmp)
        readers_tables(source_cache, tmp, random.Random(SEED + 1))
        accent_tables(source_cache, tmp, random.Random(ACCENT_SEED))
        icon_table(source_cache, tmp, random.Random(ICON_SEED))

if __name__ == "__main__":
    main()
