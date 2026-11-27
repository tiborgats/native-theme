#!/usr/bin/env python3
"""Generate the Qt colour test vectors of the KDE reader's colour helpers.

Writes native-theme/src/kde/qt_color_vectors.rs, the tables
`native-theme/src/kde/colors.rs`'s tests check its ports of three Qt and KDE
colour functions against:

- `Qt.tint(base, Qt.alpha(tint, alpha))`, `QQuickColorValueType::tint`
  (qtdeclarative `src/quick/util/qquickvaluetypes.cpp:57-75` at v6.11.2),
  run by Qt itself in a QML engine;
- `KColorUtils::mix(a, b, bias)` (kguiaddons `src/colors/kcolorutils.cpp:144-165`
  at 7c766f6f) and Kirigami's `ColorUtils::linearInterpolation(a, b, balance)`
  (kirigami `src/platform/colorutils.cpp:61-80` at 8319acc7), neither of which
  PySide6 ships: their bodies are transcribed below line for line, and every
  `QColor` step in them (`redF`, `hueF`, `fromRgbF`, `fromHsvF`, the HSV
  conversions, the 16-bit storage) is Qt's own `QColor`.

The KDE inputs are opaque 8-bit colours, as kdeglobals gives them. Each table
holds the Breeze inputs, the edge cases of `linearInterpolation`'s hue rule
(grey, pure red) and, from 20 000 seeded random inputs, up to 40 on which a
naive port (the same formula in f64 on the 8-bit channels, rounded once to 16
bits) gives another colour than Qt, and 24 more. linearInterpolation's table
also holds up to 8 on which Qt's own conversion from HSV, rounding
`1.0f - (s * f)` twice instead of once as Qt 6.11.2's build does (a fused
multiply-add), gives another colour. Every output is the 16-bit colour
`QColor` holds.

Requires PySide6 6.11.2 (Qt 6.11.2), the version the tables were generated
with; the script refuses another, and a build whose QColor::toRgb does not
fuse `1.0f - (s * f)` (no unfused-distinguishing input found). Run from
anywhere:

    python3 -I scripts/generate_qt_color_vectors.py

With `--qt-lighter` the script writes instead
native-theme/tests/vectors/qt_lighter.tsv, the table of the clean-room
`qt_lighter`: `QColor(r, g, b, a).lighter(factor)` called through PySide6, and
what that `QColor` reports, its 8-bit channels and the 16-bit ones it holds.
No Qt code is transcribed here: the rows are Qt's own results. The table holds
Breeze's selection colour at 110, edge colours at the edge factors, alphas, a
factor whose product with the value passes 32 bits, inputs on which a build
that does not fuse a multiply-add gives another colour (found outside this
script, with the describer's model), and, from 20 000 seeded random inputs, up
to 40 on which the plain rule (the HSV value times factor/100, clamped, in f64
on the 8-bit channels) gives another colour than Qt, and 24 more. That mode has
its own pin, PySide6 6.12.0 (Qt 6.12.0), which it refuses to run without; the
default mode, its pin and its tables are unchanged:

    python3 -I scripts/generate_qt_color_vectors.py --qt-lighter

With `--qt-tint` the script writes instead
native-theme/tests/vectors/qt_tint.tsv, the table of the clean-room
`qt_tint`: `Qt.tint(base, Qt.alpha(tint, factor))` run by Qt's QML engine, and
what the result reports, its 8-bit channels and the 16-bit ones it holds. No
Qt code is transcribed here: the rows are Qt's own results. The table holds
Breeze's two switch inputs at 0.9; on Breeze Light's pair and on QT_TINT's row
32 the factors at which the tint's 8-bit alpha passes 0 to 1 and 254 to 255
(the 16-bit alphas found by asking Qt), 0, 1 and two outside [0, 1]; Breeze's
pairs on bases of alpha 0 and 128; then, from 20 000 seeded random opaque
pairs at the switch's factor 0.9, up to 40 on which `naive_tint` gives another
colour, and 24 more; the same from 20 000 seeded random pairs at random
factors in [0, 1); then 24 random translucent bases at random factors. That
mode has its own pin, PySide6 6.12.0 (Qt 6.12.0):

    python3 -I scripts/generate_qt_color_vectors.py --qt-tint
"""

import json
import math
import os
import random
import struct
import sys

os.environ.setdefault("QT_QPA_PLATFORM", "offscreen")

from PySide6 import QtCore, QtGui, QtQml  # noqa: E402

QT_VERSION = "6.11.2"
OUTPUT = os.path.join(
    os.path.dirname(os.path.abspath(__file__)),
    os.pardir, "native-theme", "src", "kde", "qt_color_vectors.rs",
)
RANDOM_INPUTS = 20_000
NAIVE_FAILURES = 40
UNFUSED_FAILURES = 8
OTHERS = 24
SEED = 20261017

# Breeze v6.7.5's inputs (BreezeLight.colors, BreezeDark.colors): the Button
# set's text and background for the switch track; the Header, Window and View
# sets' background and text at frameContrast 0.2 for the edges.
BREEZE_TINT = [
    ((35, 38, 41), (252, 252, 252), 0.9),
    ((252, 252, 252), (41, 44, 48), 0.9),
]
BREEZE_PAIRS = [
    ((222, 224, 226), (35, 38, 41), 0.2),
    ((239, 240, 241), (35, 38, 41), 0.2),
    ((255, 255, 255), (35, 38, 41), 0.2),
    ((41, 44, 48), (252, 252, 252), 0.2),
    ((32, 35, 38), (252, 252, 252), 0.2),
    ((20, 22, 24), (252, 252, 252), 0.2),
]
# linearInterpolation takes the other colour's hue where one's is not above 0:
# a grey (hue undefined, -1) and a pure red (hue 0).
HUE_EDGES = [
    ((128, 128, 128), (61, 174, 233), 0.2),
    ((61, 174, 233), (128, 128, 128), 0.2),
    ((128, 128, 128), (200, 200, 200), 0.5),
    ((255, 0, 0), (61, 174, 233), 0.3),
    ((61, 174, 233), (255, 0, 0), 0.3),
    ((255, 0, 0), (0, 0, 0), 0.7),
]


def f32(x):
    """`x` rounded to a C `float`."""
    return struct.unpack("f", struct.pack("f", x))[0]


def qcolor(rgb):
    color = QtGui.QColor(*rgb)
    channels = color.rgba64()
    stored = (channels.red(), channels.green(), channels.blue())
    if stored != tuple(c * 0x101 for c in rgb):
        sys.exit(f"QColor{rgb} is not stored as each channel times 0x101: {stored}")
    return color


def rgba64(color):
    channels = color.toRgb().rgba64()
    return (channels.red(), channels.green(), channels.blue(), channels.alpha())


def lerp(a, b, f):
    return a + (b - a) * f


def kcolorutils_mix(c1, c2, bias):
    """kcolorutils.cpp:144-165, line for line (`qreal` is a Python float)."""
    if bias <= 0.0:
        return c1
    if bias >= 1.0:
        return c2
    if math.isnan(bias):
        return c1
    a = lerp(c1.alphaF(), c2.alphaF(), bias)
    if a <= 0.0:
        return QtGui.QColor(QtCore.Qt.GlobalColor.transparent)

    def channel(x1, x2):
        # `c1.redF() * c1.alphaF()` is a float product.
        mixed = lerp(f32(x1 * c1.alphaF()), f32(x2 * c2.alphaF()), bias)
        return min(max(mixed, 0.0), 1.0) / a

    return QtGui.QColor.fromRgbF(
        channel(c1.redF(), c2.redF()),
        channel(c1.greenF(), c2.greenF()),
        channel(c1.blueF(), c2.blueF()),
        a,
    )


def kirigami_linear_interpolation(one, two, balance):
    """colorutils.cpp:61-80, line for line (`double` is a Python float)."""
    source_hue = max(one.hueF() if one.hueF() > 0.0 else two.hueF(), 0.0)
    target_hue = max(two.hueF() if two.hueF() > 0.0 else one.hueF(), 0.0)
    hue = math.fmod(lerp(source_hue, target_hue, balance), 1.0)
    saturation = min(max(lerp(one.saturationF(), two.saturationF(), balance), 0.0), 1.0)
    value = min(max(lerp(one.valueF(), two.valueF(), balance), 0.0), 1.0)
    alpha = min(max(lerp(one.alphaF(), two.alphaF(), balance), 0.0), 1.0)
    # fromHsvF takes floats: PySide6 narrows each double as C++ does.
    return QtGui.QColor.fromHsvF(hue, saturation, value, alpha)


def qt_tints(cases):
    """`Qt.tint(base, Qt.alpha(tint, alpha))` for each case, run by Qt's QML engine."""
    inputs = [["#%02x%02x%02x" % base, "#%02x%02x%02x" % tint, alpha] for base, tint, alpha in cases]
    source = (
        "import QtQuick\n"
        "QtObject {\n"
        "    property var inputs: %s\n"
        "    property string outputs: JSON.stringify(inputs.map(function (t) {\n"
        "        var c = Qt.tint(Qt.color(t[0]), Qt.alpha(Qt.color(t[1]), t[2]));\n"
        "        return [c.r, c.g, c.b, c.a];\n"
        "    }))\n"
        "}\n"
    ) % json.dumps(inputs)
    component = QtQml.QQmlComponent(ENGINE)
    component.setData(source.encode(), QtCore.QUrl())
    item = component.create()
    if item is None:
        sys.exit("\n".join(e.toString() for e in component.errors()))
    out = []
    for channels in json.loads(item.property("outputs")):
        # Each is `QColor::redF()` and the others: a 16-bit channel over
        # 65535 as a float.
        stored = [round(c * 65535) for c in channels]
        if any(f32(s / 65535) != c for s, c in zip(stored, channels)):
            sys.exit(f"a tint component is not a 16-bit channel: {channels}")
        out.append(tuple(stored))
    return out


def naive_tint(base, tint, alpha):
    return tuple(round((t / 255 * alpha + b / 255 * (1 - alpha)) * 65535) for b, t in zip(base, tint))


def naive_mix(a, b, bias):
    return tuple(round(lerp(x / 255, y / 255, bias) * 65535) for x, y in zip(a, b))


def naive_linear_interpolation(one, two, balance):
    import colorsys

    def hsv(rgb):
        h, s, v = colorsys.rgb_to_hsv(*(c / 255 for c in rgb))
        return (h if s > 0 else -1.0), s, v

    (h1, s1, v1), (h2, s2, v2) = hsv(one), hsv(two)
    source = max(h1 if h1 > 0 else h2, 0.0)
    target = max(h2 if h2 > 0 else h1, 0.0)
    rgb = colorsys.hsv_to_rgb(
        math.fmod(lerp(source, target, balance), 1.0),
        min(max(lerp(s1, s2, balance), 0.0), 1.0),
        min(max(lerp(v1, v2, balance), 0.0), 1.0),
    )
    return tuple(round(c * 65535) for c in rgb)


def unfused_hsv_to_rgb(color):
    """`QColor::toRgb` from HSV (qcolor.cpp:2222-2292) with every float
    operation rounded on its own. Each is one double operation rounded to a
    float, which is the float operation's own result."""
    hue = color.hsvHueF()
    hue16 = 65535 if hue == -1.0 else round(hue * 36000)
    s16 = round(color.hsvSaturationF() * 65535)
    v16 = round(color.valueF() * 65535)
    if s16 == 0 or hue16 == 65535:
        return (v16, v16, v16)
    h = 0.0 if hue16 == 36000 else f32(hue16 / 6000.0)
    s, v = f32(s16 / 65535.0), f32(v16 / 65535.0)
    i = int(h)
    f = f32(h - i)
    p = f32(v * f32(1.0 - s))
    q = f32(v * f32(1.0 - f32(s * f)))
    t = f32(v * f32(1.0 - f32(s * f32(1.0 - f))))
    rgb = {0: (v, t, p), 1: (q, v, p), 2: (p, v, t), 3: (p, q, v), 4: (t, p, v), 5: (v, p, q)}[i]
    return tuple(int(f32(f32(x * 65535.0) + 0.5)) for x in rgb)


def pick(rng, fixed, random_cases, qt_results, naive, unfused=None):
    """The fixed cases, then up to NAIVE_FAILURES random ones a naive port
    gets wrong, up to UNFUSED_FAILURES on which `unfused` differs from Qt,
    then OTHERS more."""
    failing, others, unfused_failing = [], [], []
    for case, result in zip(random_cases, qt_results):
        if unfused is not None and unfused(*case) != tuple(result[:3]):
            unfused_failing.append((case, result))
        elif naive(*case) != tuple(result[:3]):
            failing.append((case, result))
        else:
            others.append((case, result))
    rng.shuffle(others)
    picked = fixed + failing[:NAIVE_FAILURES] + unfused_failing[:UNFUSED_FAILURES] + others[:OTHERS]
    return picked, len(failing), len(unfused_failing)


def random_rgb(rng):
    return tuple(rng.randrange(256) for _ in range(3))


def rows(name, doc, picked):
    lines = [doc, f"const {name}: [QtVector; {len(picked)}] = ["]
    for (a, b, f), out in picked:
        lines.append(f"    ({list(a)}, {list(b)}, {f!r}, {list(out)}),")
    lines.append("];")
    return "\n".join(lines)


def main():
    if QtCore.qVersion() != QT_VERSION:
        sys.exit(f"this script's tables are Qt {QT_VERSION}'s; this is Qt {QtCore.qVersion()}")
    rng = random.Random(SEED)

    tint_random = [(random_rgb(rng), random_rgb(rng), rng.random()) for _ in range(RANDOM_INPUTS)]
    tint_out = qt_tints(BREEZE_TINT + tint_random)
    tint_fixed = list(zip(BREEZE_TINT, tint_out[: len(BREEZE_TINT)]))
    tint, tint_failing, _ = pick(rng, tint_fixed, tint_random, tint_out[len(BREEZE_TINT):], naive_tint)

    def table(function, naive, unfused=None):
        fixed_cases = BREEZE_PAIRS + HUE_EDGES
        cases = [(random_rgb(rng), random_rgb(rng), rng.random()) for _ in range(RANDOM_INPUTS)]
        run = lambda c: rgba64(function(qcolor(c[0]), qcolor(c[1]), c[2]))  # noqa: E731
        fixed = [(c, run(c)) for c in fixed_cases]
        return pick(rng, fixed, cases, [run(c) for c in cases], naive, unfused)

    def unfused_linear_interpolation(one, two, balance):
        return unfused_hsv_to_rgb(kirigami_linear_interpolation(qcolor(one), qcolor(two), balance))

    mix, mix_failing, _ = table(kcolorutils_mix, naive_mix)
    lerp_hsv, lerp_failing, lerp_unfused = table(
        kirigami_linear_interpolation, naive_linear_interpolation, unfused_linear_interpolation
    )
    if lerp_unfused == 0:
        sys.exit(
            "this Qt rounds `1.0f - (s * f)` in QColor::toRgb twice: the reference build "
            "contracts it into one fused multiply-add, which the port's mul_add and the "
            "table's unfused rows are checked against"
        )

    header = f"""\
// @generated by scripts/generate_qt_color_vectors.py with PySide6 {QT_VERSION}
// (Qt {QT_VERSION}); do not edit. Each row is (a, b, factor, the 16-bit red,
// green, blue and alpha QColor holds for the result), the inputs opaque 8-bit
// colours. Of {RANDOM_INPUTS} seeded random inputs, a naive port (the formula
// in f64 on the 8-bit channels, rounded once to 16 bits) gave another colour
// than Qt on {tint_failing} for tint, {mix_failing} for mix and {lerp_failing}
// for linearInterpolation; each table holds up to {NAIVE_FAILURES} of them.
// QColor's conversion from HSV with `1.0f - (s * f)` rounded twice, not once
// as Qt's build rounds it, gave another colour on {lerp_unfused}; the
// linearInterpolation table holds up to {UNFUSED_FAILURES} of them."""
    tables = [
        rows(
            "QT_TINT",
            "/// `Qt.tint(a, Qt.alpha(b, factor))`, run by Qt's QML engine.",
            tint,
        ),
        rows(
            "KCOLORUTILS_MIX",
            "/// `KColorUtils::mix(a, b, factor)`, over Qt's `QColor`.",
            mix,
        ),
        rows(
            "KIRIGAMI_LINEAR_INTERPOLATION",
            "/// Kirigami's `ColorUtils::linearInterpolation(a, b, factor)`, over Qt's `QColor`.",
            lerp_hsv,
        ),
    ]
    with open(OUTPUT, "w", encoding="utf-8") as out:
        out.write(header + "\n\n" + "\n\n".join(tables) + "\n")
    print(f"wrote {os.path.normpath(OUTPUT)}: tint {len(tint)} ({tint_failing} naive failures), "
          f"mix {len(mix)} ({mix_failing}), linearInterpolation {len(lerp_hsv)} ({lerp_failing}, "
          f"{lerp_unfused} unfused)")


LIGHTER_QT_VERSION = "6.12.0"
LIGHTER_OUTPUT = os.path.join(
    os.path.dirname(os.path.abspath(__file__)),
    os.pardir, "native-theme", "tests", "vectors", "qt_lighter.tsv",
)
LIGHTER_SEED = 20261122
LIGHTER_PLAIN_FAILURES = 40
LIGHTER_OTHERS = 24
# Breeze's highlight, `[Colors:Selection]` `BackgroundNormal`: 61,174,233 in
# both native-theme/tests/fixtures/kde/breeze-light.ini and breeze-dark.ini.
LIGHTER_BREEZE = (61, 174, 233, 255)
# Its lighter(110) as issue #4 measured it on Breeze Light
# (docs/issues/0004_kde-leftovers-rationale.md, L3).
LIGHTER_BREEZE_MEASURED = "#44bfffff"
LIGHTER_FACTORS = [0, -10, 50, 100, 101, 110, 150, 200, 1000]
LIGHTER_COLOURS = [
    (0, 0, 0, 255),        # black
    (255, 255, 255, 255),  # white
    (1, 1, 1, 255),        # greys
    (64, 64, 64, 255),
    (128, 128, 128, 255),
    (192, 192, 192, 255),
    (254, 254, 254, 255),
    (255, 0, 0, 255),      # the primaries and secondaries
    (0, 255, 0, 255),
    (0, 0, 255, 255),
    (255, 255, 0, 255),
    (0, 255, 255, 255),
    (255, 0, 255, 255),
    (232, 100, 50, 255),   # the value overflows at 110, 150 and 200
    (200, 120, 40, 255),   # at 150 and 200, not at 110
    (150, 60, 90, 255),    # at 200 only
    (0, 90, 180, 255),     # fully saturated, value below the maximum
    (255, 128, 0, 255),    # fully saturated, value at the maximum
    (128, 128, 129, 255),  # nearly achromatic
    (200, 201, 200, 255),
]
LIGHTER_ALPHA_COLOURS = [(61, 174, 233), (255, 255, 255), (232, 100, 50)]
LIGHTER_ALPHAS = [0, 128]
LIGHTER_ALPHA_FACTORS = [50, 110, 200]
# 65537 times the largest value is 2^32 - 1; 65538 times it passes 2^32.
LIGHTER_WIDE = [((255, 0, 0, 255), 65537), ((255, 0, 0, 255), 65538),
                ((255, 255, 255, 255), 65538), ((61, 174, 233, 255), 2147483647)]
# Inputs at 110 on which a build that rounds the conversion back to RGB's
# products separately, instead of in one fused multiply-add, gives another
# 8-bit colour (found with the describer's model, outside this script), and
# the colour PySide6 6.12.0 gave for each when this table was made: the script
# refuses a build that gives another.
LIGHTER_FUSED = {
    (4, 5, 20): "#040516ff", (5, 20, 4): "#051604ff", (9, 15, 222): "#0a10f4ff",
    (10, 15, 50): "#0b1037ff", (4, 20, 5): "#041605ff", (5, 4, 20): "#050416ff",
    (9, 222, 15): "#0af410ff", (10, 50, 15): "#0b3710ff",
}


def lighter_row(rgba, factor):
    """The row of `QColor(rgba).lighter(factor)`, as Qt reports it."""
    color = QtGui.QColor(*rgba).lighter(factor)
    if not color.isValid() or color.spec() != QtGui.QColor.Spec.Rgb:
        sys.exit(f"QColor{rgba}.lighter({factor}) is not a valid RGB colour")
    stored = color.rgba64()
    channels8 = (color.red(), color.green(), color.blue(), color.alpha())
    channels16 = (stored.red(), stored.green(), stored.blue(), stored.alpha())
    return (rgba, factor, channels8, channels16)


def plain_lighter(rgb, factor):
    """The plain rule, this script's own: the HSV value times factor/100,
    clamped to 1, in f64 on the 8-bit channels, rounded to 8 bits."""
    import colorsys

    h, s, v = colorsys.rgb_to_hsv(*(c / 255 for c in rgb))
    return tuple(round(c * 255) for c in colorsys.hsv_to_rgb(h, s, min(v * factor / 100, 1.0)))


def hex8(channels):
    return "#" + "".join(f"{c:02x}" for c in channels)


def lighter_main():
    import PySide6

    if QtCore.qVersion() != LIGHTER_QT_VERSION or PySide6.__version__ != LIGHTER_QT_VERSION:
        sys.exit(f"qt_lighter.tsv is PySide6 {LIGHTER_QT_VERSION}'s (Qt {LIGHTER_QT_VERSION}); "
                 f"this is PySide6 {PySide6.__version__} (Qt {QtCore.qVersion()})")

    fixed = [(LIGHTER_BREEZE, 110)]
    fixed += [(c, f) for c in LIGHTER_COLOURS for f in LIGHTER_FACTORS]
    fixed += [((*c, a), f) for c in LIGHTER_ALPHA_COLOURS for a in LIGHTER_ALPHAS for f in LIGHTER_ALPHA_FACTORS]
    fixed += LIGHTER_WIDE
    fixed += [((*c, 255), 110) for c in LIGHTER_FUSED]
    fixed = list(dict.fromkeys(fixed))
    fixed_rows = [lighter_row(c, f) for c, f in fixed]

    breeze = hex8(fixed_rows[0][2])
    if breeze != LIGHTER_BREEZE_MEASURED:
        sys.exit(f"Breeze's highlight at 110 is {breeze} here, not the measured {LIGHTER_BREEZE_MEASURED}")
    for (rgba, factor, channels8, _) in fixed_rows:
        expected = LIGHTER_FUSED.get(rgba[:3]) if rgba[3] == 255 and factor == 110 else None
        if expected is not None and hex8(channels8) != expected:
            sys.exit(f"QColor{rgba}.lighter(110) is {hex8(channels8)} here, not {expected}: this build "
                     "does not fuse the multiply-adds the table's reference build fuses")

    rng = random.Random(LIGHTER_SEED)
    failing, others = [], []
    for _ in range(RANDOM_INPUTS):
        rgba = (rng.randrange(256), rng.randrange(256), rng.randrange(256), rng.randrange(256))
        factor = rng.randint(1, 400)
        row = lighter_row(rgba, factor)
        (failing if plain_lighter(rgba[:3], factor) != row[2][:3] else others).append(row)
    rng.shuffle(others)
    picked = fixed_rows + failing[:LIGHTER_PLAIN_FAILURES] + others[:LIGHTER_OTHERS]

    def srgb(channels16):
        return "srgb(" + ", ".join(f"{f32(c / 65535):.9g}" for c in channels16) + ")"

    lines = [
        "# function: qt_lighter",
        "# route: public_api",
        f"# source: qtbase@{LIGHTER_QT_VERSION}",
        "# generations: kde-breeze",
        "# generator: scripts/generate_qt_color_vectors.py",
        "# inputs: c:colour factor:number",
        "# outputs: result:colour stored:colour",
        f"# written by scripts/generate_qt_color_vectors.py --qt-lighter with PySide6 {LIGHTER_QT_VERSION}",
        f"# (Qt {LIGHTER_QT_VERSION}); do not edit. Each row is QColor(c).lighter(factor): `result` the",
        "# 8-bit red(), green(), blue() and alpha() it reports, `stored` the 16-bit channels it holds,",
        "# each over 65535 as a float. Breeze's selection colour (61,174,233, the same in Breeze Light",
        "# and Breeze Dark) at 110; black, white, greys, the primaries and secondaries, colours whose",
        "# value overflows at 110, 150 and 200, fully saturated and nearly achromatic ones at the",
        f"# factors {', '.join(str(f) for f in LIGHTER_FACTORS)}; alphas 0 and 128; factors whose",
        f"# product with the value passes 32 bits; {len(LIGHTER_FUSED)} inputs on which a build that does not fuse",
        "# the multiply-adds of the conversion back to RGB gives another colour; then, of",
        f"# {RANDOM_INPUTS} seeded random inputs (seed {LIGHTER_SEED}; factors 1 to 400), up to",
        f"# {LIGHTER_PLAIN_FAILURES} on which the plain rule (the HSV value times factor/100, clamped, in",
        f"# f64 on the 8-bit channels) gives another colour than Qt ({len(failing)} did), and"
        f" {LIGHTER_OTHERS} more.",
        "c\tfactor\tresult\tstored",
    ]
    for rgba, factor, channels8, channels16 in picked:
        lines.append(f"{hex8(rgba)}\t{factor}\t{hex8(channels8)}\t{srgb(channels16)}")
    with open(LIGHTER_OUTPUT, "w", encoding="utf-8") as out:
        out.write("\n".join(lines) + "\n")
    print(f"wrote {os.path.normpath(LIGHTER_OUTPUT)}: {len(picked)} rows ({len(fixed_rows)} fixed, "
          f"{min(len(failing), LIGHTER_PLAIN_FAILURES)} of {len(failing)} plain-rule failures, "
          f"{min(len(others), LIGHTER_OTHERS)} more)")


TINT_QT_VERSION = "6.12.0"
TINT_OUTPUT = os.path.join(
    os.path.dirname(os.path.abspath(__file__)),
    os.pardir, "native-theme", "tests", "vectors", "qt_tint.tsv",
)
TINT_SEED = 20261123
TINT_NAIVE_FAILURES = 40
TINT_OTHERS = 24
TINT_TRANSLUCENT_RANDOM = 24
# QT_TINT's row 32 (native-theme/src/kde/qt_color_vectors.rs), where Qt
# returns the tint itself.
TINT_ROW32 = ((202, 173, 23), (53, 236, 164))
TINT_OUTSIDE = [-0.25, 1.25]


def tint_run_qml(cases):
    """cases: (base rgba 8-bit, tint rgb 8-bit, factor). Returns per case
    (base as Qt stored it, the tint's 16-bit alpha after Qt.alpha, the 16-bit
    result)."""
    inputs = [["#%02x%02x%02x%02x" % (b[3], b[0], b[1], b[2]), "#%02x%02x%02x" % t, f] for b, t, f in cases]
    source = (
        "import QtQuick\n"
        "QtObject {\n"
        "    property var inputs: %s\n"
        "    property string outputs: JSON.stringify(inputs.map(function (t) {\n"
        "        var base = Qt.color(t[0]);\n"
        "        var over = Qt.alpha(Qt.color(t[1]), t[2]);\n"
        "        var c = Qt.tint(base, over);\n"
        "        return [[base.r, base.g, base.b, base.a], over.a, [c.r, c.g, c.b, c.a]];\n"
        "    }))\n"
        "}\n"
    ) % json.dumps(inputs)
    component = QtQml.QQmlComponent(ENGINE)
    component.setData(source.encode(), QtCore.QUrl())
    item = component.create()
    if item is None:
        sys.exit("\n".join(e.toString() for e in component.errors()))

    def to16(values):
        stored = [round(v * 65535) for v in values]
        if any(f32(s / 65535) != v for s, v in zip(stored, values)):
            sys.exit(f"a component is not a 16-bit channel: {values}")
        return tuple(stored)

    out = []
    for (b, t, f), (base, over_a, c) in zip(cases, json.loads(item.property("outputs"))):
        base16 = to16(base)
        if base16 != tuple(x * 0x101 for x in b):
            sys.exit(f"base {b} is stored as {base16}, not each channel times 0x101")
        out.append((base16, to16([over_a])[0], to16(c)))
    return out


def tint_main():
    import PySide6

    if QtCore.qVersion() != TINT_QT_VERSION or PySide6.__version__ != TINT_QT_VERSION:
        sys.exit(f"qt_tint.tsv is PySide6 {TINT_QT_VERSION}'s (Qt {TINT_QT_VERSION}); "
                 f"this is PySide6 {PySide6.__version__} (Qt {QtCore.qVersion()})")

    # The 16-bit alphas whose 8-bit alpha() Qt reports as 0 and as 255.
    alpha8 = [QtGui.QColor.fromRgba64(0, 0, 0, x).alpha() for x in range(65536)]
    last0 = max(x for x in range(65536) if alpha8[x] == 0)
    first255 = min(x for x in range(65536) if alpha8[x] == 255)
    # The description's claim: alpha() is the nearest integer to x/257 (no ties exist).
    mismatch = [x for x in range(65536) if alpha8[x] != (2 * x + 257) // 514]
    print(f"8-bit alpha 0 up to 16-bit {last0}, 255 from {first255}; "
          f"nearest-integer(x/257) mismatches: {len(mismatch)}")

    threshold_factors = [x / 65535 for x in (last0, last0 + 1, first255 - 1, first255)]
    pairs = [((*BREEZE_TINT[0][0], 255), BREEZE_TINT[0][1]), ((*TINT_ROW32[0], 255), TINT_ROW32[1])]
    fixed = [((*b, 255), t, f) for b, t, f in BREEZE_TINT]
    fixed += [(b, t, f) for b, t in pairs for f in threshold_factors + [0.0, 1.0] + TINT_OUTSIDE]
    fixed += [((*BREEZE_TINT[0][0], a), BREEZE_TINT[0][1], 0.9) for a in (0, 128)]
    fixed += [((*BREEZE_TINT[1][0], a), BREEZE_TINT[1][1], 0.9) for a in (0, 128)]

    rng = random.Random(TINT_SEED)
    at09 = [((*random_rgb(rng), 255), random_rgb(rng), 0.9) for _ in range(RANDOM_INPUTS)]
    anyf = [((*random_rgb(rng), 255), random_rgb(rng), rng.random()) for _ in range(RANDOM_INPUTS)]
    translucent = [((*random_rgb(rng), rng.randrange(256)), random_rgb(rng), rng.random())
                   for _ in range(TINT_TRANSLUCENT_RANDOM)]

    fixed_out = tint_run_qml(fixed)
    # The threshold factors must land on the intended 16-bit alphas.
    for f, x in zip(threshold_factors, (last0, last0 + 1, first255 - 1, first255)):
        got = tint_run_qml([((0, 0, 0, 255), (0, 0, 0), f)])[0][1]
        if got != x:
            sys.exit(f"Qt.alpha(c, {f!r}) holds alpha {got}, not {x}")

    def pick_tints(cases):
        results = tint_run_qml(cases)
        failing, others = [], []
        for case, res in zip(cases, results):
            b, t, f = case
            (failing if naive_tint(b[:3], t, f) != res[2][:3] else others).append((case, res))
        rng.shuffle(others)
        return failing[:TINT_NAIVE_FAILURES] + others[:TINT_OTHERS], len(failing)

    picked09, failing09 = pick_tints(at09)
    pickedf, failingf = pick_tints(anyf)
    translucent_rows = list(zip(translucent, tint_run_qml(translucent)))
    rows = list(zip(fixed, fixed_out)) + picked09 + pickedf + translucent_rows

    # Branch census, from Qt's own outputs: the result equals the tint as Qt.alpha left it, or the base.
    as_tint = sum(1 for (b, t, f), (base16, ta, c) in rows if c == (*(x * 0x101 for x in t), ta))
    as_base = sum(1 for (b, t, f), (base16, ta, c) in rows if c == base16)

    def srgb(channels16):
        return "srgb(" + ", ".join(f"{f32(c / 65535):.9g}" for c in channels16) + ")"

    lines = [
        "# function: qt_tint",
        "# route: public_api",
        f"# source: qtdeclarative@{TINT_QT_VERSION}",
        f"# source: qtbase@{TINT_QT_VERSION}",
        "# generations: kde-breeze",
        "# generator: scripts/generate_qt_color_vectors.py",
        "# inputs: base:colour tint:colour factor:number",
        "# outputs: result:colour stored:colour",
        f"# written by scripts/generate_qt_color_vectors.py --qt-tint with PySide6 {TINT_QT_VERSION}",
        f"# (Qt {TINT_QT_VERSION}); do not edit. Each row is Qt.tint(base, Qt.alpha(tint, factor)) run by Qt's",
        "# QML engine: `result` the 8-bit red(), green(), blue() and alpha() the result reports, `stored`",
        "# the 16-bit channels it holds, each over 65535 as a float. `tint`'s own alpha is replaced by",
        "# Qt.alpha (written ff). Breeze's switch inputs (the Button set's text tinted by its background,",
        "# BreezeLight.colors and BreezeDark.colors) at 0.9; on Breeze Light's pair and on 202,173,23 /",
        f"# 53,236,164 the factors {last0}/65535, {last0 + 1}/65535, {first255 - 1}/65535 and {first255}/65535",
        "# (the tint's 8-bit alpha passing 0 to 1 and 254 to 255), 0, 1, -0.25 and 1.25; Breeze's pairs",
        f"# on bases of alpha 0 and 128; then, of {RANDOM_INPUTS} seeded random opaque pairs (seed {TINT_SEED})",
        f"# at the switch's 0.9, up to {TINT_NAIVE_FAILURES} on which a naive port (the formula in f64 on the",
        f"# 8-bit channels, rounded once to 16 bits) gives another colour than Qt ({failing09} did) and",
        f"# {TINT_OTHERS} more; the same of {RANDOM_INPUTS} pairs at random factors in [0, 1) ({failingf} did);",
        f"# then {TINT_TRANSLUCENT_RANDOM} random bases of random alpha at random factors.",
        "base\ttint\tfactor\tresult\tstored",
    ]
    for (b, t, f), (base16, ta, c) in rows:
        channels8 = QtGui.QColor.fromRgba64(*c)
        result = (channels8.red(), channels8.green(), channels8.blue(), channels8.alpha())
        lines.append(f"{hex8(b)}\t{hex8((*t, 255))}\t{f!r}\t{hex8(result)}\t{srgb(c)}")
    with open(TINT_OUTPUT, "w", encoding="utf-8") as out:
        out.write("\n".join(lines) + "\n")
    print(f"wrote {os.path.normpath(TINT_OUTPUT)}: {len(rows)} rows ({len(fixed)} fixed, {len(picked09)} at 0.9 "
          f"[{failing09} naive failures], {len(pickedf)} at random factors [{failingf} naive failures], "
          f"{len(translucent_rows)} translucent); rows equal to the tint: {as_tint}, to the base: {as_base}")


APP = QtGui.QGuiApplication(sys.argv[:1])
ENGINE = QtQml.QQmlEngine()

if __name__ == "__main__":
    if sys.argv[1:] == ["--qt-lighter"]:
        lighter_main()
    elif sys.argv[1:] == ["--qt-tint"]:
        tint_main()
    else:
        main()
