// The KDE reader's scheme test's references, called through the system's
// libraries: Qt 6.12.0, KConfig 6.31 and KColorScheme 6.31 (generate.py
// refuses another Qt or KConfig).
//
// Reads one input per line on stdin, its UTF-8 bytes in hex, "=" for the empty
// string, "-" for no input (a key the file leaves unset). The first argument
// names the mode; each mode writes one line per input, tab-separated:
//
// "palette": the input is the value of kdeglobals' [Colors:Window]
//   BackgroundNormal. KConfig writes a file holding it ("-": no such key),
//   with a [Colors:View] group, then the file is opened afresh and
//   KColorScheme::createApplicationPalette builds the palette from it, as
//   plasma-integration builds the application palette from kdeglobals
//   (khintssettings.cpp:445-446). Prints
//     <has key> <value read back> <valid> <red> <green> <blue> <alpha> <gray>
//   <has key> 1 where the reopened file holds the key; <value read back> the
//   key's value as a string, in hex ("-" without the key), to show the file
//   holds the input; <valid> QColor::isValid() of the palette's active window
//   colour; red, green, blue its QColor::rgb() channels, alpha its alpha();
//   <gray> qGray(rgb()), Qt's qrgb.h.
//
// "plasma": the same kdeglobals in a fresh XDG_CONFIG_HOME, read by a child
//   process's QGuiApplication on the offscreen platform with the platform theme
//   "kde", plasma-integration's KDEPlasmaPlatformTheme6 as installed. Prints
//     <scheme> <red> <green> <blue>
//   <scheme> QStyleHints::colorScheme() (0 unknown, 1 light, 2 dark), which
//   plasma-integration answers from KHintsSettings::determineColorScheme;
//   red, green, blue QGuiApplication::palette()'s active window colour.

#include <QByteArray>
#include <QColor>
#include <QDir>
#include <QGuiApplication>
#include <QPalette>
#include <QString>
#include <QStyleHints>

#include <KColorScheme>
#include <KConfig>
#include <KConfigGroup>
#include <KSharedConfig>

#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <string>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>

static void putHex(const QByteArray &bytes)
{
    if (bytes.isEmpty()) {
        std::fputs("=", stdout);
        return;
    }
    for (const char c : bytes) {
        std::printf("%02x", static_cast<unsigned char>(c));
    }
}

// The input line: false for "-", else its bytes.
static bool input(const std::string &line, QByteArray *out)
{
    out->clear();
    if (line == "-") {
        return false;
    }
    if (line == "=") {
        return true;
    }
    for (size_t i = 0; i + 1 < line.size(); i += 2) {
        unsigned int byte = 0;
        if (std::sscanf(line.c_str() + i, "%2x", &byte) != 1) {
            std::fprintf(stderr, "bad input line\n");
            std::exit(2);
        }
        out->append(static_cast<char>(byte));
    }
    return true;
}

// kdeglobals at path, written by KConfig: a [Colors:View] group (so that
// plasma-integration builds its palette from this file) and, where the input
// has a value, [Colors:Window] BackgroundNormal.
static void writeKdeglobals(const QString &path, bool has, const QByteArray &value)
{
    KConfig config(path, KConfig::SimpleConfig);
    config.group(QStringLiteral("Colors:View")).writeEntry("BackgroundNormal", QStringLiteral("255,255,255"));
    if (has) {
        config.group(QStringLiteral("Colors:Window")).writeEntry("BackgroundNormal", QString::fromUtf8(value));
    }
    if (!config.sync()) {
        std::fprintf(stderr, "cannot write %s\n", qPrintable(path));
        std::exit(2);
    }
}

static int palette(const char *dir)
{
    std::string line;
    int n = 0;
    while (std::getline(std::cin, line)) {
        QByteArray value;
        const bool has = input(line, &value);
        const QString path = QStringLiteral("%1/kdeglobals-%2").arg(QString::fromLocal8Bit(dir)).arg(n++);
        writeKdeglobals(path, has, value);
        const KSharedConfigPtr config = KSharedConfig::openConfig(path, KConfig::SimpleConfig);
        const KConfigGroup window = config->group(QStringLiteral("Colors:Window"));
        const bool hasKey = window.hasKey("BackgroundNormal");
        const QPalette pal = KColorScheme::createApplicationPalette(config);
        const QColor c = pal.color(QPalette::Active, QPalette::Window);
        const QRgb rgb = c.rgb();
        std::printf("%d\t", hasKey ? 1 : 0);
        if (hasKey) {
            putHex(window.readEntry("BackgroundNormal", QString()).toUtf8());
        } else {
            std::fputs("-", stdout);
        }
        std::printf("\t%d\t%d\t%d\t%d\t%d\t%d\n", c.isValid() ? 1 : 0, qRed(rgb), qGreen(rgb), qBlue(rgb), c.alpha(),
                    qGray(rgb));
    }
    return 0;
}

// Runs body in a child process, its stdout the parent's; exits on a failed child.
template<typename F>
static void inChild(F body)
{
    std::fflush(stdout);
    const pid_t pid = fork();
    if (pid < 0) {
        std::perror("fork");
        std::exit(2);
    }
    if (pid == 0) {
        const int status = body();
        std::fflush(stdout);
        _exit(status);
    }
    int status = 0;
    if (waitpid(pid, &status, 0) != pid || !WIFEXITED(status) || WEXITSTATUS(status) != 0) {
        std::fprintf(stderr, "child failed: %d\n", status);
        std::exit(2);
    }
}

static void isolate(const QString &home)
{
    for (const char *name : {"QT_SCALE_FACTOR", "QT_SCREEN_SCALE_FACTORS", "QT_FONT_DPI", "QT_ENABLE_HIGHDPI_SCALING",
                             "QT_USE_PHYSICAL_DPI", "QT_AUTO_SCREEN_SCALE_FACTOR", "QT_QPA_PLATFORMTHEME",
                             "QT_STYLE_OVERRIDE", "KDE_FULL_SESSION", "XDG_CURRENT_DESKTOP", "SNAP",
                             "DBUS_SESSION_BUS_ADDRESS"}) {
        unsetenv(name);
    }
    for (const char *sub : {"config", "data", "cache", "runtime", "config-dirs", "data-dirs"}) {
        QDir().mkpath(home + u'/' + QLatin1String(sub));
    }
    const auto set = [&](const char *name, const char *sub) {
        setenv(name, qPrintable(home + u'/' + QLatin1String(sub)), 1);
    };
    setenv("HOME", qPrintable(home), 1);
    set("XDG_CONFIG_HOME", "config");
    set("XDG_DATA_HOME", "data");
    set("XDG_CACHE_HOME", "cache");
    set("XDG_RUNTIME_DIR", "runtime");
    set("XDG_CONFIG_DIRS", "config-dirs");
    set("XDG_DATA_DIRS", "data-dirs");
    chmod(qPrintable(home + u"/runtime"), 0700);
}

static int plasma(const char *dir)
{
    std::string line;
    int n = 0;
    while (std::getline(std::cin, line)) {
        QByteArray value;
        const bool has = input(line, &value);
        const QString home = QStringLiteral("%1/home-%2").arg(QString::fromLocal8Bit(dir)).arg(n++);
        inChild([&] {
            isolate(home);
            writeKdeglobals(home + u"/config/kdeglobals", has, value);
            setenv("QT_QPA_PLATFORM", "offscreen", 1);
            setenv("QT_QPA_PLATFORMTHEME", "kde", 1);
            int argc = 1;
            char name[] = "kde-vectors";
            char *argv[] = {name, nullptr};
            QGuiApplication app(argc, argv);
            const QColor c = QGuiApplication::palette().color(QPalette::Active, QPalette::Window);
            const QRgb rgb = c.rgb();
            std::printf("%d\t%d\t%d\t%d\n", static_cast<int>(QGuiApplication::styleHints()->colorScheme()), qRed(rgb),
                        qGreen(rgb), qBlue(rgb));
            return 0;
        });
    }
    return 0;
}

int main(int argc, char **argv)
{
    if (argc != 3) {
        std::fprintf(stderr, "usage: readers palette|plasma <writable folder>\n");
        return 2;
    }
    const std::string mode = argv[1];
    if (mode == "palette") {
        return palette(argv[2]);
    }
    if (mode == "plasma") {
        return plasma(argv[2]);
    }
    std::fprintf(stderr, "unknown mode %s\n", argv[1]);
    return 2;
}
