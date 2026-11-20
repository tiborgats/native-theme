import sys
from PySide6.QtCore import QUrl, QObject
from PySide6.QtGui import QGuiApplication, QFont, QFontInfo, QFontMetricsF
from PySide6.QtQuick import QQuickView
from PySide6.QtQuickControls2 import QQuickStyle
app = QGuiApplication(sys.argv)
QQuickStyle.setStyle("org.kde.desktop")
print("style", QQuickStyle.name())
f = app.font()
print("app font", f.family(), "pointSizeF", f.pointSizeF(), "pixelSize", f.pixelSize())
fi = QFontInfo(f)
print("QFontInfo(app font) pixelSize", fi.pixelSize(), "pointSizeF", fi.pointSizeF())
v = QQuickView()
v.setSource(QUrl.fromLocalFile(sys.argv[1]))
if v.status() != QQuickView.Ready:
    for e in v.errors(): print("QML error", e.toString())
    sys.exit(1)
root = v.rootObject()
for name in ("plain", "label", "kirigamiDefault", "button"):
    o = root.findChild(QObject, name)
    fnt = o.property("font")
    extra = ""
    if name != "button":
        extra = f"fontInfo.pixelSize {o.property('infoPx')} fontInfo.pointSize {o.property('infoPt')} contentHeight {o.property('contentHeight')}"
    print(name, "font pointSizeF", fnt.pointSizeF(), "pixelSize", fnt.pixelSize(), "QFontInfo.pixelSize", QFontInfo(fnt).pixelSize(), "implicitHeight", o.property("implicitHeight"), "implicitWidth", o.property("implicitWidth"), extra)
print("screen logicalDpiY", app.primaryScreen().logicalDotsPerInchY(), "dpr", app.primaryScreen().devicePixelRatio())
