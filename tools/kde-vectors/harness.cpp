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
// The colours are 8-bit channels, as QColor(r, g, b, a) takes them.

#include <QColor>
#include <QLoggingCategory>

#include <cstdio>
#include <cstdlib>
#include <cstring>

#include "colorutils.h"
#include "kcolorscheme.h"
#include "kcolorschememanager_p.h"

Q_LOGGING_CATEGORY(KirigamiPlatform, "kf.kirigami.platform")
Q_LOGGING_CATEGORY(KCOLORSCHEME, "kf.colorscheme")

// kcolorscheme.cpp's defaultConfig() names it; KColorScheme::shade never
// reaches it, so kcolorschememanager.cpp is not built.
KColorSchemeManagerPrivate::ContrastPreference KColorSchemeManagerPrivate::contrastPreference()
{
    std::abort();
}

static void write(const QColor &color)
{
    const QColor rgb = color.toRgb();
    std::printf("%.9g %.9g %.9g %.9g\n", rgb.redF(), rgb.greenF(), rgb.blueF(), rgb.alphaF());
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
        } else {
            return 2;
        }
        std::fflush(stdout);
    }
    return 0;
}
