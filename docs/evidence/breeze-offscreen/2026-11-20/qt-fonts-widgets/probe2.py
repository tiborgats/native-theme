import sys
from PySide6.QtCore import QUrl, QObject
from PySide6.QtGui import QGuiApplication
from PySide6.QtQuick import QQuickView
from PySide6.QtQuickControls2 import QQuickStyle
app = QGuiApplication(sys.argv)
QQuickStyle.setStyle("org.kde.desktop")
v = QQuickView()
v.setSource(QUrl.fromLocalFile(sys.argv[1]))
if v.status() != QQuickView.Ready:
    for e in v.errors(): print("QML error", e.toString())
    sys.exit(1)
root = v.rootObject()
for name in ("h1", "h2", "small"):
    o = root.findChild(QObject, name)
    fnt = o.property("font")
    print(name, "font pointSizeF", fnt.pointSizeF(), "pixelSize", fnt.pixelSize(), "fontInfo.pixelSize", o.property("infoPx"), "fontInfo.pointSize", o.property("infoPt"), "implicitHeight", o.property("implicitHeight"))
