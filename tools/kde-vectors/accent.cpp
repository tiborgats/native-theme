// The accent derivation's reference: plasma-workspace@6.8.2's
// kcms/colors/colorsapplicator.cpp and .h, compiled unchanged beside this
// file (generate.py fetches them at the pin), over the system's Qt 6.12.0,
// KConfig 6.31, KGuiAddons 6.31 and KColorScheme 6.31 (generate.py refuses
// others).
//
// Reads one case per line on stdin: `<scheme> <accent>`, the scheme file's
// bytes in hex, then "-" for no accent (applyScheme's accentColor argument
// left out, kdeglobals holding no AccentColor) or `r,g,b,a`, four floats
// handed to QColor::fromRgbF, the colour passed as applyScheme's accentColor.
// For each case it writes the scheme file into the sandbox's own /tmp, opens
// a fresh, empty kdeglobals there, calls applyScheme(scheme, kdeglobals,
// KConfig::Normal, accent), syncs it, and reopens the written file. It prints,
// each line tab-separated, a colour as `invalid` or `r,g,b,a` (QColor's redF()
// … alphaF(), %.9g):
//
//   lists <groups> <inactive groups> <unset keys> <tint factor> <title bar>
//       over the scheme file as KConfig reads it: the seven sets
//       (Window, View, Button, Selection, Tooltip, Complementary, Header)
//       with a [Colors:<Set>] group, those whose group has an [Inactive]
//       group, each comma-separated ("-" for none); the unset keys of the
//       groups the file has, `<set>.<Key>` and `<set>.inactive.<Key>`; the
//       [General] TintFactor as KConfig reads it (%.17g, "-" where the file
//       has none); the title-bar option, TitlebarIsAccentColored, else
//       accentActiveTitlebar, else false (S4.8), 0 or 1.
//   in <set> <key> <active> <inactive>
//       the scheme input: KColorScheme over the scheme file, the active state,
//       and the inactive state with the inactive effects and the Selection
//       set's inactive change turned off in memory (the file's [Inactive]
//       value, else as KColorScheme reads a key it lacks).
//   wmin <key> <colour>
//       the [WM] start value S9.1 defines: the file's [WM] key, else
//       KColorScheme's Header colours over the file (Normal background and
//       foreground; Inactive background and foreground with the file's
//       inactive effects).
//   out <set> <key> <active> <inactive>
//       the same reading as `in` over the written kdeglobals.
//   wm <key> <colour>
//       the written kdeglobals' [WM] key, as KConfig reads a colour.
//   raw <hex>
//       the written kdeglobals' bytes.
//   end
//
// With the argument "constants" it instead prints the constants of the two
// colour transforms applyScheme's tint uses (colorsapplicator.cpp:105-106),
// as the system's Qt library computed them: each one's matrix, nine singles
// row by row as QColorMatrix::map applies them, and the seven parameters of
// its input and output transfer functions and their inverses; then the alphas
// QColor::setAlphaF(0.7) and (0.6) store, read back. Each a C99 hex float.

#include <clocale>
#include <cstdio>
#include <iostream>
#include <optional>
#include <string>

#include <QByteArray>
#include <QColor>
#include <QColorSpace>
#include <QColorTransform>
#include <QCoreApplication>
#include <QFile>
#include <QString>
#include <QStringList>

#include <KColorScheme>
#include <KConfig>
#include <KConfigGroup>
#include <KSharedConfig>

#include <private/qcolorspace_p.h>
#include <private/qcolortransform_p.h>
#include <private/qcolortransferfunction_p.h>
#include <private/qcolormatrix_p.h>

#include "colorsapplicator.h"

// colorsapplicator.cpp's two transforms (:105-106), which have external linkage there.
extern QColorTransform toLinearSRGB;
extern QColorTransform fromLinearSRGB;

namespace {

const char *const SETS[] = {"Window", "View", "Button", "Selection", "Tooltip", "Complementary", "Header"};
const KColorScheme::ColorSet SET_IDS[] = {KColorScheme::Window, KColorScheme::View, KColorScheme::Button,
                                          KColorScheme::Selection, KColorScheme::Tooltip,
                                          KColorScheme::Complementary, KColorScheme::Header};
const char *const KEYS[] = {"BackgroundNormal", "BackgroundAlternate", "ForegroundNormal", "ForegroundInactive",
                            "ForegroundActive", "ForegroundLink", "ForegroundVisited", "ForegroundNegative",
                            "ForegroundNeutral", "ForegroundPositive", "DecorationFocus", "DecorationHover"};
const char *const WM_KEYS[] = {"activeBackground", "activeForeground", "inactiveBackground", "inactiveForeground"};

std::string colour(const QColor &c)
{
    if (!c.isValid())
        return "invalid";
    char buf[128];
    std::snprintf(buf, sizeof buf, "%.9g,%.9g,%.9g,%.9g", double(c.redF()), double(c.greenF()), double(c.blueF()),
                  double(c.alphaF()));
    return buf;
}

// The colour of `key` in a KColorScheme: KColorScheme's roles of the twelve keys.
QColor role(const KColorScheme &s, int key)
{
    switch (key) {
    case 0: return s.background(KColorScheme::NormalBackground).color();
    case 1: return s.background(KColorScheme::AlternateBackground).color();
    case 2: return s.foreground(KColorScheme::NormalText).color();
    case 3: return s.foreground(KColorScheme::InactiveText).color();
    case 4: return s.foreground(KColorScheme::ActiveText).color();
    case 5: return s.foreground(KColorScheme::LinkText).color();
    case 6: return s.foreground(KColorScheme::VisitedText).color();
    case 7: return s.foreground(KColorScheme::NegativeText).color();
    case 8: return s.foreground(KColorScheme::NeutralText).color();
    case 9: return s.foreground(KColorScheme::PositiveText).color();
    case 10: return s.decoration(KColorScheme::FocusColor).color();
    default: return s.decoration(KColorScheme::HoverColor).color();
    }
}

// The active and the effect-free inactive reading of every set and key over the file at `path`.
void readBack(const char *tag, const QString &path)
{
    KSharedConfigPtr cfg = KSharedConfig::openConfig(path, KConfig::SimpleConfig);
    KConfigGroup effects(cfg, QStringLiteral("ColorEffects:Inactive"));
    effects.writeEntry("Enable", false);
    effects.writeEntry("ChangeSelectionColor", false);
    for (int s = 0; s < 7; ++s) {
        const KColorScheme active(QPalette::Active, SET_IDS[s], cfg);
        const KColorScheme inactive(QPalette::Inactive, SET_IDS[s], cfg);
        for (int k = 0; k < 12; ++k) {
            std::cout << tag << '\t' << SETS[s] << '\t' << KEYS[k] << '\t' << colour(role(active, k)) << '\t'
                      << colour(role(inactive, k)) << '\n';
        }
    }
}

void lists(const QString &path)
{
    KSharedConfigPtr cfg = KSharedConfig::openConfig(path, KConfig::SimpleConfig);
    QStringList groups, inactive, unset;
    for (const char *set : SETS) {
        const QString name = QStringLiteral("Colors:") + QLatin1String(set);
        if (!cfg->hasGroup(name))
            continue;
        groups << QLatin1String(set);
        const KConfigGroup g(cfg, name);
        for (const char *key : KEYS) {
            if (!g.hasKey(key))
                unset << QLatin1String(set) + QLatin1Char('.') + QLatin1String(key);
        }
        if (g.hasGroup(QStringLiteral("Inactive"))) {
            inactive << QLatin1String(set);
            const KConfigGroup i = g.group(QStringLiteral("Inactive"));
            for (const char *key : KEYS) {
                if (!i.hasKey(key))
                    unset << QLatin1String(set) + QStringLiteral(".inactive.") + QLatin1String(key);
            }
        }
    }
    const KConfigGroup general(cfg, QStringLiteral("General"));
    std::string tint = "-";
    if (general.hasKey("TintFactor")) {
        char buf[64];
        std::snprintf(buf, sizeof buf, "%.17g", general.readEntry<qreal>("TintFactor", 0.15));
        tint = buf;
    }
    const bool titlebar = general.readEntry("TitlebarIsAccentColored", general.readEntry("accentActiveTitlebar", false));
    const auto list = [](const QStringList &l) { return l.isEmpty() ? std::string("-") : l.join(u',').toStdString(); };
    std::cout << "lists\t" << list(groups) << '\t' << list(inactive) << '\t' << list(unset) << '\t' << tint << '\t'
              << (titlebar ? 1 : 0) << '\n';
}

void wmStart(const QString &path)
{
    KSharedConfigPtr cfg = KSharedConfig::openConfig(path, KConfig::SimpleConfig);
    const KConfigGroup wm(cfg, QStringLiteral("WM"));
    const KColorScheme normal(QPalette::Normal, KColorScheme::Header, cfg);
    const KColorScheme inactive(QPalette::Inactive, KColorScheme::Header, cfg);
    const QColor defaults[] = {normal.background().color(), normal.foreground().color(),
                               inactive.background().color(), inactive.foreground().color()};
    for (int k = 0; k < 4; ++k) {
        const QColor c = wm.hasKey(WM_KEYS[k]) ? wm.readEntry(WM_KEYS[k], QColor()) : defaults[k];
        std::cout << "wmin\t" << WM_KEYS[k] << '\t' << colour(c) << '\n';
    }
}

bool write(const QString &path, const QByteArray &bytes)
{
    QFile f(path);
    return f.open(QFile::WriteOnly | QFile::Truncate) && f.write(bytes) == bytes.size();
}

void hexFloat(const char *name, float x)
{
    std::printf("%s\t%a\n", name, double(x));
}

void matrix(const char *name, const QColorMatrix &m)
{
    std::printf("%s\t%a %a %a\t%a %a %a\t%a %a %a\n", name, double(m.r.x), double(m.g.x), double(m.b.x),
                double(m.r.y), double(m.g.y), double(m.b.y), double(m.r.z), double(m.g.z), double(m.b.z));
}

void function(const char *name, const QColorTransferFunction &f)
{
    std::printf("%s\t%a %a %a %a %a %a %a\n", name, double(f.m_a), double(f.m_b), double(f.m_c), double(f.m_d),
                double(f.m_e), double(f.m_f), double(f.m_g));
}

int constants()
{
    // The transforms applyScheme's tint uses, as the library computed them (colorsapplicator.cpp:105-106).
    for (const auto &[name, t] : {std::pair{"to_linear", &toLinearSRGB}, std::pair{"from_linear", &fromLinearSRGB}}) {
        const QColorTransformPrivate *d = QColorTransformPrivate::get(*t);
        matrix(name, d->colorMatrix);
        function((std::string(name) + "_in").c_str(), d->colorSpaceIn->trc[0].fun());
        function((std::string(name) + "_in_inverse").c_str(), d->colorSpaceIn->trc[0].fun().inverted());
        function((std::string(name) + "_out").c_str(), d->colorSpaceOut->trc[0].fun());
        function((std::string(name) + "_out_inverse").c_str(), d->colorSpaceOut->trc[0].fun().inverted());
    }
    QColor c(0, 0, 0);
    c.setAlphaF(0.7);
    hexFloat("alpha_0.7", c.alphaF());
    c.setAlphaF(0.6);
    hexFloat("alpha_0.6", c.alphaF());
    return 0;
}

} // namespace

int main(int argc, char **argv)
{
    QCoreApplication app(argc, argv);
    // QCoreApplication sets the locale from the environment: the numbers print in C's.
    std::setlocale(LC_ALL, "C");
    if (argc > 1 && std::string(argv[1]) == "constants")
        return constants();
    std::string line;
    int n = 0;
    while (std::getline(std::cin, line)) {
        const auto space = line.find(' ');
        if (space == std::string::npos) {
            std::cerr << "a case is `<scheme hex> <accent>`\n";
            return 2;
        }
        const QByteArray scheme = QByteArray::fromHex(QByteArray::fromStdString(line.substr(0, space)));
        const std::string accentText = line.substr(space + 1);
        std::optional<QColor> accent;
        if (accentText != "-") {
            float r, g, b, a;
            if (std::sscanf(accentText.c_str(), "%f,%f,%f,%f", &r, &g, &b, &a) != 4) {
                std::cerr << "an accent is `-` or `r,g,b,a`\n";
                return 2;
            }
            accent = QColor::fromRgbF(r, g, b, a);
        }
        ++n;
        const QString dir = QStringLiteral("/tmp/");
        const QString schemePath = dir + QStringLiteral("scheme-%1.colors").arg(n);
        const QString inputsPath = dir + QStringLiteral("inputs-%1.colors").arg(n);
        const QString wmPath = dir + QStringLiteral("wm-%1.colors").arg(n);
        const QString outPath = dir + QStringLiteral("kdeglobals-%1").arg(n);
        const QString readPath = dir + QStringLiteral("read-%1").arg(n);
        if (!write(schemePath, scheme) || !write(inputsPath, scheme) || !write(wmPath, scheme)) {
            std::cerr << "cannot write the scheme file\n";
            return 2;
        }
        lists(inputsPath);
        readBack("in", inputsPath);
        wmStart(wmPath);
        {
            KSharedConfigPtr out = KSharedConfig::openConfig(outPath, KConfig::SimpleConfig);
            applyScheme(schemePath, out.data(), KConfig::Normal, accent);
            out->sync();
        }
        QFile written(outPath);
        if (!written.open(QFile::ReadOnly)) {
            std::cerr << "applyScheme wrote no kdeglobals\n";
            return 2;
        }
        const QByteArray bytes = written.readAll();
        if (!write(readPath, bytes)) {
            std::cerr << "cannot copy the kdeglobals\n";
            return 2;
        }
        readBack("out", readPath);
        {
            KSharedConfigPtr cfg = KSharedConfig::openConfig(readPath, KConfig::SimpleConfig);
            const KConfigGroup wm(cfg, QStringLiteral("WM"));
            for (const char *key : WM_KEYS)
                std::cout << "wm\t" << key << '\t' << colour(wm.readEntry(key, QColor())) << '\n';
        }
        std::cout << "raw\t" << bytes.toHex().toStdString() << '\n' << "end" << std::endl;
    }
    return 0;
}
