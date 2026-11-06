// Calls the pinned sources' functions, each compiled unchanged beside this
// file (generate.py fetches them and builds the harness). Reads one call per
// line on stdin, writes the result's QColor channels as floats, unrounded.
//
//   lerp <r> <g> <b> <a> <r> <g> <b> <a> <balance>
//       Kirigami's ColorUtils::linearInterpolation(one, two, balance).
//   shade <r> <g> <b> <a> <role> <contrast>
//       KColorScheme::shade(color, role, 0.1 * contrast), with no chroma
//       adjustment: contrast is kdeglobals' [KDE] contrast step, scaled as
//       KColorScheme::contrastF scales it.
//
// The colours of these two are 8-bit channels, as QColor(r, g, b, a) takes
// them; each prints "<r> <g> <b> <a>".
//
// The calls below take each colour as four reals, the model's f32 channels,
// entered as QColor::fromRgbF(r, g, b, a) enters them, and each number as an
// f32 widened to the double the function takes. Each prints
// "<spec> <r> <g> <b> <a>": the result's QColor spec (1 Rgb, 5 ExtendedRgb)
// and its channels as that spec holds them (toRgb() for another spec).
//
//   kmix <c1> <c2> <bias>          KColorUtils::mix(c1, c2, bias)
//   ktint <base> <color> <amount>  KColorUtils::tint(base, color, amount)
//   kshade <c> <luma> <chroma>     KColorUtils::shade(c, luma, chroma)
//   koverlay <base> <paint>        KColorUtils::overlayColors(base, paint)
//                                  (CompositionMode_SourceOver, its default)
//   rtint <target> <tint> <alpha>  Kirigami's ColorUtils::tintWithAlpha
//   effects <c> <background> <group> <kind> <intensity> <intensity_amount>
//           <color> <color_amount> <color_color> <contrast> <contrast_amount>
//           <inactive_enabled>
//       KColorScheme's StateEffects of the group (0 active, 1 disabled,
//       2 inactive) read from an in-memory config holding the effect tokens
//       as kdeglobals' [ColorEffects:<group>] keys, applied as KColorScheme
//       applies them to a role of the kind (0 foreground, 1 decoration:
//       brush(c, background); 2 background: brush(c)).
//   selinactive <selection> <window> <selection_background> <kind> <change>
//       KColorScheme(QPalette::Inactive, Selection, config) over an in-memory
//       config holding the three colours ([Colors:Selection][Inactive],
//       [Colors:Window][Inactive], [Colors:Selection] BackgroundNormal) and
//       [ColorEffects:Inactive] ChangeSelectionColor=<change>, Enable=false
//       (no effects over it): ForegroundNormal, DecorationFocus or
//       BackgroundNormal by the kind. Its colours must be 8-bit (a config
//       stores 8-bit colours); the harness fails on another.

#include <QBrush>
#include <QColor>
#include <QLoggingCategory>

#include <KConfigGroup>
#include <KSharedConfig>

#include <cstdio>
#include <cstdlib>
#include <cstring>

#include "colorutils.h"
#include "kcolorscheme.h"
#include "kcolorschemehelpers_p.h"
#include "kcolorschememanager_p.h"
#include "kcolorutils.h"

Q_LOGGING_CATEGORY(KirigamiPlatform, "kf.kirigami.platform")
Q_LOGGING_CATEGORY(KCOLORSCHEME, "kf.colorscheme")

// kcolorscheme.cpp's defaultConfig() names it; the calls here pass their own
// config, so it is never reached and kcolorschememanager.cpp is not built.
KColorSchemeManagerPrivate::ContrastPreference KColorSchemeManagerPrivate::contrastPreference()
{
    std::abort();
}

static void write(const QColor &color)
{
    const QColor rgb = color.toRgb();
    std::printf("%.9g %.9g %.9g %.9g\n", rgb.redF(), rgb.greenF(), rgb.blueF(), rgb.alphaF());
}

static void writeSpec(const QColor &color)
{
    if (!color.isValid()) {
        std::printf("0 0 0 0 0\n");
        return;
    }
    const QColor held = color.spec() == QColor::ExtendedRgb ? color : color.toRgb();
    std::printf("%d %.9g %.9g %.9g %.9g\n", int(held.spec()), held.redF(), held.greenF(), held.blueF(), held.alphaF());
}

static bool readColor(QColor *color)
{
    float c[4];
    if (std::scanf("%f %f %f %f", &c[0], &c[1], &c[2], &c[3]) != 4) {
        return false;
    }
    *color = QColor::fromRgbF(c[0], c[1], c[2], c[3]);
    return true;
}

static bool readNumber(double *number)
{
    float x;
    if (std::scanf("%f", &x) != 1) {
        return false;
    }
    *number = double(x);
    return true;
}

static bool readInt(int *n)
{
    return std::scanf("%d", n) == 1;
}

// A colour written to the config, failing where it does not read back as the
// same QColor (a config stores 8-bit channels).
static void writeColor(KConfigGroup group, const char *key, const QColor &color)
{
    group.writeEntry(key, color);
    if (group.readEntry(key, QColor()).rgba64() != color.rgba64()) {
        std::fprintf(stderr, "%s does not read back as written\n", key);
        std::exit(3);
    }
}

static void writeNumber(KConfigGroup group, const char *key, double number)
{
    group.writeEntry(key, number);
    if (group.readEntry(key, 0.0) != number) {
        std::fprintf(stderr, "%s does not read back as written\n", key);
        std::exit(3);
    }
}

static KSharedConfigPtr memoryConfig()
{
    // An empty name with SimpleConfig is KConfig's in-memory config: no file
    // read or written.
    return KSharedConfig::openConfig(QString(), KConfig::SimpleConfig);
}

static int effects()
{
    QColor c;
    QColor background;
    int group;
    int kind;
    int intensity;
    double intensityAmount;
    int color;
    double colorAmount;
    QColor colorColor;
    int contrast;
    double contrastAmount;
    int enabled;
    if (!readColor(&c) || !readColor(&background) || !readInt(&group) || !readInt(&kind) || !readInt(&intensity)
        || !readNumber(&intensityAmount) || !readInt(&color) || !readNumber(&colorAmount) || !readColor(&colorColor)
        || !readInt(&contrast) || !readNumber(&contrastAmount) || !readInt(&enabled)) {
        return 2;
    }
    const QPalette::ColorGroup states[] = {QPalette::Active, QPalette::Disabled, QPalette::Inactive};
    if (group < 0 || group > 2 || kind < 0 || kind > 2) {
        return 2;
    }
    KSharedConfigPtr config = memoryConfig();
    for (const char *name : {"ColorEffects:Disabled", "ColorEffects:Inactive"}) {
        config->deleteGroup(QString::fromLatin1(name));
    }
    if (group != 0) {
        KConfigGroup cfg(config, group == 1 ? QStringLiteral("ColorEffects:Disabled") : QStringLiteral("ColorEffects:Inactive"));
        if (group == 2) {
            cfg.writeEntry("Enable", enabled != 0);
        }
        cfg.writeEntry("IntensityEffect", intensity);
        writeNumber(cfg, "IntensityAmount", intensityAmount);
        cfg.writeEntry("ColorEffect", color);
        writeNumber(cfg, "ColorAmount", colorAmount);
        writeColor(cfg, "Color", colorColor);
        cfg.writeEntry("ContrastEffect", contrast);
        writeNumber(cfg, "ContrastAmount", contrastAmount);
    }
    const StateEffects stateEffects(states[group], config);
    if (kind == 2) {
        writeSpec(stateEffects.brush(QBrush(c)).color());
    } else {
        writeSpec(stateEffects.brush(QBrush(c), QBrush(background)).color());
    }
    return 0;
}

static int selectionInactive()
{
    QColor selection;
    QColor window;
    QColor selectionBackground;
    int kind;
    int change;
    if (!readColor(&selection) || !readColor(&window) || !readColor(&selectionBackground) || !readInt(&kind)
        || !readInt(&change) || kind < 0 || kind > 2) {
        return 2;
    }
    KSharedConfigPtr config = memoryConfig();
    for (const char *name : {"ColorEffects:Inactive", "Colors:Selection", "Colors:Window"}) {
        config->deleteGroup(QString::fromLatin1(name));
    }
    KConfigGroup effects(config, QStringLiteral("ColorEffects:Inactive"));
    effects.writeEntry("Enable", false);
    effects.writeEntry("ChangeSelectionColor", change != 0);
    KConfigGroup selectionGroup(config, QStringLiteral("Colors:Selection"));
    writeColor(selectionGroup, "BackgroundNormal", selectionBackground);
    const char *keys[] = {"ForegroundNormal", "DecorationFocus", "BackgroundNormal"};
    writeColor(selectionGroup.group(QStringLiteral("Inactive")), keys[kind], selection);
    writeColor(KConfigGroup(config, QStringLiteral("Colors:Window")).group(QStringLiteral("Inactive")), keys[kind], window);
    const KColorScheme scheme(QPalette::Inactive, KColorScheme::Selection, config);
    if (kind == 0) {
        writeSpec(scheme.foreground(KColorScheme::NormalText).color());
    } else if (kind == 1) {
        writeSpec(scheme.decoration(KColorScheme::FocusColor).color());
    } else {
        writeSpec(scheme.background(KColorScheme::NormalBackground).color());
    }
    return 0;
}

int main()
{
    char op[16];
    while (std::scanf("%15s", op) == 1) {
        if (std::strcmp(op, "lerp") == 0) {
            int a[4];
            int b[4];
            double balance;
            if (std::scanf("%d %d %d %d %d %d %d %d %lf", &a[0], &a[1], &a[2], &a[3], &b[0], &b[1], &b[2], &b[3], &balance) != 9) {
                return 2;
            }
            ColorUtils utils;
            write(utils.linearInterpolation(QColor(a[0], a[1], a[2], a[3]), QColor(b[0], b[1], b[2], b[3]), balance));
        } else if (std::strcmp(op, "shade") == 0) {
            int c[4];
            int role;
            double contrast;
            if (std::scanf("%d %d %d %d %d %lf", &c[0], &c[1], &c[2], &c[3], &role, &contrast) != 6) {
                return 2;
            }
            write(KColorScheme::shade(QColor(c[0], c[1], c[2], c[3]), KColorScheme::ShadeRole(role), 0.1 * contrast));
        } else if (std::strcmp(op, "kmix") == 0 || std::strcmp(op, "ktint") == 0 || std::strcmp(op, "rtint") == 0) {
            QColor one;
            QColor two;
            double x;
            if (!readColor(&one) || !readColor(&two) || !readNumber(&x)) {
                return 2;
            }
            if (op[0] == 'r') {
                ColorUtils utils;
                writeSpec(utils.tintWithAlpha(one, two, x));
            } else if (op[1] == 'm') {
                writeSpec(KColorUtils::mix(one, two, x));
            } else {
                writeSpec(KColorUtils::tint(one, two, x));
            }
        } else if (std::strcmp(op, "kshade") == 0) {
            QColor c;
            double luma;
            double chroma;
            if (!readColor(&c) || !readNumber(&luma) || !readNumber(&chroma)) {
                return 2;
            }
            writeSpec(KColorUtils::shade(c, luma, chroma));
        } else if (std::strcmp(op, "koverlay") == 0) {
            QColor base;
            QColor paint;
            if (!readColor(&base) || !readColor(&paint)) {
                return 2;
            }
            writeSpec(KColorUtils::overlayColors(base, paint));
        } else if (std::strcmp(op, "effects") == 0) {
            if (const int status = effects()) {
                return status;
            }
        } else if (std::strcmp(op, "selinactive") == 0) {
            if (const int status = selectionInactive()) {
                return status;
            }
        } else {
            return 2;
        }
        std::fflush(stdout);
    }
    return 0;
}
