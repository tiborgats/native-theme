// Calls Qt's public QFont::fromString on each input and prints what it set,
// through QFont's public getters: the system's Qt 6.12.0, the pinned release
// (generate.py refuses another).
//
// Reads one input per line on stdin: the string's UTF-8 bytes in hex, or "="
// for the empty string. Writes one line per input, tab-separated:
//
//   <ok> <mask> <family> <pointSizeF> <pixelSize> <styleHint> <weight> <style>
//   <underline> <strikeOut> <fixedPitch> <capitalization> <letterSpacingType>
//   <letterSpacing> <wordSpacing> <stretch> <styleStrategy> <styleName>
//   <features> <axes>
//
// <ok> is fromString's return (1 or 0); <mask> QFont::resolveMask() of a
// default-constructed QFont after the call, the properties the call set.
// Strings are their UTF-8 bytes in hex, "=" when empty; reals "%.17g".
//
// The style name, the features and the variable axes are set without a mask
// bit (fromString assigns, clears, or leaves them), so a second QFont that
// starts with sentinels in all three (a style name, a feature and an axis
// fromString never writes) tells whether the call touched them: <styleName>
// is "-" where that font's style name is still the sentinel, else the
// string; <features> and <axes> are "-" where the sentinel is still there,
// else the call's entries as "<tag>=<value>" joined by ",", in QFont's order
// (by tag), "=" for none.
//
// A line "w<TAB><family><TAB><pointSizeF><TAB><pixelSize><TAB><weight><TAB>
// <style><TAB><styleHint><TAB><styleName><TAB><features><TAB><axes>" instead
// builds a QFont with those setters (a size or a style name only where it is
// positive or not "-"; the strings in hex; features and axes "-" or the hex of
// "<tag>=<value>" joined by ",") and prints the hex of its toString(): the
// strings Qt 6.12 writes, as KConfig writes a font entry.

#include <QByteArray>
#include <QFont>
#include <QList>
#include <QString>
#include <QStringList>

#include <cstdio>
#include <cstring>
#include <optional>
#include <string>

static const char SentinelName[] = "\x01sentinel";
static constexpr QFont::Tag SentinelTag("\x01\x01\x01\x01");

static void putHex(const QString &s)
{
    const QByteArray bytes = s.toUtf8();
    if (bytes.isEmpty()) {
        std::fputs("=", stdout);
        return;
    }
    for (const char c : bytes) {
        std::printf("%02x", static_cast<unsigned char>(c));
    }
}

static bool unhex(const std::string &hex, QByteArray *out)
{
    if (hex.size() % 2 != 0) {
        return false;
    }
    out->clear();
    for (size_t i = 0; i < hex.size(); i += 2) {
        unsigned int byte = 0;
        if (std::sscanf(hex.c_str() + i, "%2x", &byte) != 1) {
            return false;
        }
        out->append(static_cast<char>(byte));
    }
    return true;
}

static QString tagText(QFont::Tag tag)
{
    return QString::fromLatin1(tag.toString());
}

static bool unhexString(const std::string &hex, QString *out)
{
    QByteArray bytes;
    if (hex != "=" && !unhex(hex, &bytes)) {
        return false;
    }
    *out = QString::fromUtf8(bytes);
    return true;
}

// The "w" line's font, its toString() printed in hex; false on a malformed line.
static bool writeFont(const std::string &line)
{
    QStringList fields = QString::fromStdString(line).split(u'\t');
    if (fields.size() != 10) {
        return false;
    }
    QString family;
    if (!unhexString(fields[1].toStdString(), &family)) {
        return false;
    }
    QFont font;
    font.setFamily(family);
    if (fields[2].toDouble() > 0) {
        font.setPointSizeF(fields[2].toDouble());
    }
    if (fields[3].toInt() > 0) {
        font.setPixelSize(fields[3].toInt());
    }
    font.setWeight(QFont::Weight(fields[4].toInt()));
    font.setStyle(QFont::Style(fields[5].toInt()));
    font.setStyleHint(QFont::StyleHint(fields[6].toInt()));
    if (fields[7] != u"-") {
        QString name;
        if (!unhexString(fields[7].toStdString(), &name)) {
            return false;
        }
        font.setStyleName(name);
    }
    for (int k = 8; k <= 9; ++k) {
        if (fields[k] == u"-") {
            continue;
        }
        QString entries;
        if (!unhexString(fields[k].toStdString(), &entries)) {
            return false;
        }
        for (const QString &entry : entries.split(u',')) {
            const QStringList pair = entry.split(u'=');
            const std::optional<QFont::Tag> tag = pair.size() == 2 ? QFont::Tag::fromString(pair[0]) : std::nullopt;
            if (!tag) {
                return false;
            }
            if (k == 8) {
                font.setFeature(*tag, pair[1].toUInt());
            } else {
                font.setVariableAxis(*tag, pair[1].toFloat());
            }
        }
    }
    putHex(font.toString());
    std::fputs("\n", stdout);
    return true;
}

int main()
{
    static char line[65536];
    while (std::fgets(line, sizeof line, stdin) != nullptr) {
        std::string hex(line);
        while (!hex.empty() && (hex.back() == '\n' || hex.back() == '\r')) {
            hex.pop_back();
        }
        if (hex.rfind("w\t", 0) == 0) {
            if (!writeFont(hex)) {
                return 2;
            }
            std::fflush(stdout);
            continue;
        }
        QByteArray bytes;
        if (hex != "=" && !unhex(hex, &bytes)) {
            return 2;
        }
        const QString input = QString::fromUtf8(bytes);

        QFont font;
        const bool ok = font.fromString(input);
        const uint mask = font.resolveMask();

        QFont probe;
        probe.setStyleName(QString::fromLatin1(SentinelName));
        probe.setFeature(SentinelTag, 7);
        probe.setVariableAxis(SentinelTag, 7.0f);
        probe.fromString(input);

        std::printf("%d\t%u\t", ok ? 1 : 0, mask);
        putHex(font.family());
        std::printf("\t%.17g\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d"
                    "\t%.17g\t%.17g\t%d\t%d\t",
                    font.pointSizeF(), font.pixelSize(), int(font.styleHint()), int(font.weight()), int(font.style()),
                    int(font.underline()), int(font.strikeOut()), int(font.fixedPitch()), int(font.capitalization()),
                    int(font.letterSpacingType()), font.letterSpacing(), font.wordSpacing(), font.stretch(),
                    int(font.styleStrategy()));
        if (probe.styleName() == QString::fromLatin1(SentinelName)) {
            std::fputs("-", stdout);
        } else {
            putHex(probe.styleName());
        }
        std::fputs("\t", stdout);
        if (probe.isFeatureSet(SentinelTag)) {
            std::fputs("-", stdout);
        } else {
            QStringList entries;
            for (const QFont::Tag tag : probe.featureTags()) {
                entries << tagText(tag) + u'=' + QString::number(probe.featureValue(tag));
            }
            putHex(entries.join(u','));
        }
        std::fputs("\t", stdout);
        if (probe.isVariableAxisSet(SentinelTag)) {
            std::fputs("-", stdout);
        } else {
            QStringList entries;
            for (const QFont::Tag tag : probe.variableAxisTags()) {
                entries << tagText(tag) + u'=' + QString::number(double(probe.variableAxisValue(tag)), 'g', 9);
            }
            putHex(entries.join(u','));
        }
        std::fputs("\n", stdout);
        std::fflush(stdout);
    }
    return 0;
}
