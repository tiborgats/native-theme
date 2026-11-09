// Calls Kirigami's IconSizes (kirigami@6.31.0 src/platform/units.cpp, compiled
// unchanged beside this file with the moc output of units.h; generate.py
// fetches them and builds it) on the offscreen platform. Reads one call per
// line on stdin and writes one line per call:
//
//   i <n>      IconSizes::roundedIconSize(n), n an int; prints the result.
//   f <pt> <family>
//              the application font set to the family at <pt> points
//              (QGuiApplication::setFont, whose ApplicationFontChange the
//              Units object's event filter receives and rebuilds its font
//              metrics from), then IconSizes::sizeForLabels(); prints
//              "<height> <result>", <height> the application font's
//              QFontMetricsF::height() as the Units object reads it ("%.17g").
//              It fails where the family is not installed (a missing family
//              is never substituted).
//
// The process runs with its own HOME and XDG folders under the folder its
// first argument names and with Qt's scaling and DPI variables unset, so no
// configuration of the user's reaches the font metrics; the offscreen
// screen's logical DPI is 96, which it checks.

#include <QFont>
#include <QFontInfo>
#include <QFontMetricsF>
#include <QGuiApplication>
#include <QScreen>

#include <cerrno>
#include <clocale>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <string>
#include <sys/stat.h>

#include "units.h"

namespace
{

// Units' constructor is protected: Kirigami builds it through Units::create.
class HarnessUnits : public Kirigami::Platform::Units
{
public:
    HarnessUnits()
        : Units(nullptr)
    {
    }
};

void isolate(const std::string &home)
{
    for (const char *name : {"QT_SCALE_FACTOR", "QT_SCREEN_SCALE_FACTORS", "QT_FONT_DPI", "QT_ENABLE_HIGHDPI_SCALING",
                             "QT_USE_PHYSICAL_DPI", "QT_AUTO_SCREEN_SCALE_FACTOR", "QT_QPA_PLATFORMTHEME",
                             "QT_STYLE_OVERRIDE", "KDE_FULL_SESSION", "XDG_CURRENT_DESKTOP", "DBUS_SESSION_BUS_ADDRESS",
                             "FONTCONFIG_FILE", "FONTCONFIG_PATH"}) {
        unsetenv(name);
    }
    mkdir(home.c_str(), 0700);
    setenv("HOME", home.c_str(), 1);
    for (const auto &[name, sub] : {std::pair{"XDG_CONFIG_HOME", "/config"}, std::pair{"XDG_DATA_HOME", "/data"},
                                    std::pair{"XDG_CACHE_HOME", "/cache"}, std::pair{"XDG_RUNTIME_DIR", "/runtime"}}) {
        const std::string path = home + sub;
        mkdir(path.c_str(), 0700);
        setenv(name, path.c_str(), 1);
    }
    setenv("QT_QPA_PLATFORM", "offscreen", 1);
}

} // namespace

int main(int argc, char **argv)
{
    if (argc != 2) {
        std::cerr << "usage: icon <folder for HOME>\n";
        return 2;
    }
    isolate(std::string(argv[1]) + "/home");
    QGuiApplication app(argc, argv);
    // QGuiApplication sets the locale from the environment: the numbers read and print in C's.
    std::setlocale(LC_ALL, "C");
    if (QGuiApplication::primaryScreen()->logicalDotsPerInchY() != 96.0) {
        std::cerr << "the offscreen screen's logical DPI is not 96\n";
        return 3;
    }
    HarnessUnits units;
    const Kirigami::Platform::IconSizes *sizes = units.iconSizes();
    std::string line;
    while (std::getline(std::cin, line)) {
        if (line.size() < 3 || line[1] != ' ' || (line[0] != 'i' && line[0] != 'f')) {
            std::cerr << "a call is `i <n>` or `f <pt> <family>`\n";
            return 2;
        }
        const char *text = line.c_str() + 2;
        char *end = nullptr;
        errno = 0;
        if (line[0] == 'i') {
            const long n = std::strtol(text, &end, 10);
            if (errno != 0 || *end != '\0' || n < -2147483647L - 1 || n > 2147483647L) {
                std::cerr << "not an int: " << text << "\n";
                return 2;
            }
            std::printf("%d\n", sizes->roundedIconSize(static_cast<int>(n)));
            continue;
        }
        const double pt = std::strtod(text, &end);
        if (errno != 0 || *end != ' ' || end[1] == '\0' || !(pt > 0.0)) {
            std::cerr << "not `<pt> <family>`: " << text << "\n";
            return 2;
        }
        const QString family = QString::fromUtf8(end + 1);
        QFont font(family);
        font.setPointSizeF(pt);
        QGuiApplication::setFont(font);
        if (QFontInfo(QGuiApplication::font()).family() != family) {
            std::cerr << "not installed: " << (end + 1) << "\n";
            return 3;
        }
        const double height = QFontMetricsF(QGuiApplication::font()).height();
        std::printf("%.17g %d\n", height, sizes->sizeForLabels());
    }
    std::fflush(stdout);
    return std::ferror(stdout) ? 1 : 0;
}
