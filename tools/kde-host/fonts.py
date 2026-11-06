#!/usr/bin/env python3
"""The font reads of the KDE harness (KH-reads of the 0.8.0 measurement plan).

Writes <out>/fonts.toml: the screen's DPI, and for the application's fonts under
the `kde` platform theme (the general, fixed, small, toolbar and menu fonts
QPlatformTheme reports) and for Noto Sans 10 and Hack 10 asked by name, the
font Qt resolved (QFontInfo) and its QFontMetrics (height, which is ascent plus
descent, ascent, descent, leading, line spacing, x-height, cap height, average
character width). Run by tools/kde-host/run.sh with the container's PySide6:

    python3 tools/kde-host/fonts.py <out>
"""

import sys
from pathlib import Path

import PySide6
from PySide6.QtCore import qVersion
from PySide6.QtGui import QFont, QFontDatabase, QFontInfo, QFontMetrics, QFontMetricsF, QGuiApplication
from PySide6.QtWidgets import QApplication


def toml(value):
    return '"' + str(value).replace("\\", "\\\\").replace('"', '\\"') + '"'


def font_lines(name, font):
    info = QFontInfo(font)
    metrics = QFontMetrics(font)
    exact = QFontMetricsF(font)
    return [
        "",
        "[[font]]",
        f"name = {toml(name)}",
        f"requested = {toml(font.toString())}",
        f"resolved_family = {toml(info.family())}",
        f"resolved_style = {toml(info.styleName())}",
        f"resolved_point_size_f = {info.pointSizeF()}",
        f"resolved_pixel_size = {info.pixelSize()}",
        f"resolved_weight = {int(info.weight().value) if hasattr(info.weight(), 'value') else int(info.weight())}",
        f"height = {metrics.height()}",
        f"ascent = {metrics.ascent()}",
        f"descent = {metrics.descent()}",
        f"leading = {metrics.leading()}",
        f"line_spacing = {metrics.lineSpacing()}",
        f"x_height = {metrics.xHeight()}",
        f"cap_height = {metrics.capHeight()}",
        f"average_char_width = {metrics.averageCharWidth()}",
        f"height_f = {exact.height()}",
        f"ascent_f = {exact.ascent()}",
        f"descent_f = {exact.descent()}",
        f"leading_f = {exact.leading()}",
    ]


def main():
    if len(sys.argv) != 2:
        print("usage: fonts.py <out>", file=sys.stderr)
        return 2
    out = Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    app = QApplication(sys.argv[:1])
    screen = QGuiApplication.primaryScreen()
    lines = [
        "# Written by tools/kde-host/fonts.py.",
        "",
        "[host]",
        f"qt = {toml(qVersion())}",
        f"pyside6 = {toml(PySide6.__version__)}",
        f"platform = {toml(QGuiApplication.platformName())}",
        f"logical_dpi_y = {screen.logicalDotsPerInchY()}",
        f"physical_dpi_y = {screen.physicalDotsPerInchY()}",
        f"device_pixel_ratio = {screen.devicePixelRatio()}",
        f"application_font = {toml(app.font().toString())}",
    ]
    for name, kind in [
        ("general", QFontDatabase.SystemFont.GeneralFont),
        ("fixed", QFontDatabase.SystemFont.FixedFont),
        ("small", QFontDatabase.SystemFont.SmallestReadableFont),
        ("title", QFontDatabase.SystemFont.TitleFont),
    ]:
        lines += font_lines(f"system.{name}", QFontDatabase.systemFont(kind))
    lines += font_lines("application", app.font())
    lines += font_lines("Noto Sans 10", QFont("Noto Sans", 10))
    lines += font_lines("Hack 10", QFont("Hack", 10))
    (out / "fonts.toml").write_text("\n".join(lines) + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
