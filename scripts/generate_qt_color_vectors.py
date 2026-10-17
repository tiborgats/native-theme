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
with; the script refuses another. Run from anywhere:

    python3 -I scripts/generate_qt_color_vectors.py
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


APP = QtGui.QGuiApplication(sys.argv[:1])
ENGINE = QtQml.QQmlEngine()

if __name__ == "__main__":
    main()
