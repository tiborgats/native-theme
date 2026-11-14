import sys
from PySide6.QtWidgets import QApplication, QLabel
from PySide6.QtGui import QFont, QFontInfo, QFontMetricsF
app = QApplication(sys.argv)
scr = app.primaryScreen()
f = QFont("Noto Sans", 10)
fi = QFontInfo(f)
lab = QLabel("Hxg"); lab.setFont(f)
dpr = scr.devicePixelRatio()
print("platform", app.platformName(), "dpr", dpr, "logicalDpi", scr.logicalDotsPerInch(), "physicalDpi", round(scr.physicalDotsPerInch(),2))
print("family", fi.family(), "pixelSize", fi.pixelSize(), "pointSizeF", fi.pointSizeF())
m = QFontMetricsF(f)
print("metrics height", m.height(), "ascent", m.ascent(), "-> physical px: pixelSize*dpr =", fi.pixelSize()*dpr, " height*dpr =", m.height()*dpr)
