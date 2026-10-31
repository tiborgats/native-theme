#!/usr/bin/env python3
"""The vector tables of the KDE platform functions (specification S2.2 *Checked against their references*), by the
`compiled_source` route: the pinned upstream sources compiled unchanged into a harness that calls them.

Writes `native-theme/tests/vectors/kirigami_lerp.tsv` and `native-theme/tests/vectors/kde_shade.tsv`:

- `kirigami_lerp(a, b, balance)`: Kirigami's `ColorUtils::linearInterpolation`
  (`kirigami@6.31.0 src/platform/colorutils.cpp:61-80`) over Qt's `QColor` (`qtbase@6.12.0
  src/gui/painting/qcolor.cpp`);
- `kde_shade(c, role, contrast)`: `KColorScheme::shade(color, role, contrast)` with no chroma adjustment
  (`kcolorscheme@6.31.0 src/kcolorscheme.cpp:566-616`), the contrast step scaled as `KColorScheme::contrastF` scales
  it (`:526`), over KColorUtils' `shade` and `darken` and KHCY (`kguiaddons@6.31.0 src/colors/kcolorutils.cpp`,
  `src/colors/kcolorspaces.cpp`) and Qt's `QColor`.

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
another Qt or KConfig. Run from the repository root, the source cache's folder optional:

    python3 -I tools/kde-vectors/generate.py [source cache]
"""

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
    moc = pinned.sandboxed([MOC, platform / "colorutils.h"], capture_output=True, text=True, check=True).stdout
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
    out = pinned.sandboxed([exe], input=text, capture_output=True, text=True, check=True).stdout
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


def main():
    if qt_version() != QT_VERSION:
        sys.exit(f"Qt {qt_version() or '(none)'} found; the pin is Qt {QT_VERSION}")
    if not kconfig_version().startswith(KCONFIG_MINOR + "."):
        sys.exit(f"KF6 Config {kconfig_version() or '(none)'} found; the pin is KConfig {KCONFIG_MINOR}")
    source_cache = citations.cache_dir(sys.argv[1] if len(sys.argv) > 1 else None)
    rng = random.Random(SEED)
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
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


if __name__ == "__main__":
    main()
