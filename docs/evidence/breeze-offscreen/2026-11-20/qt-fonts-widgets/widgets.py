import sys
from PySide6.QtCore import Qt, QPoint, QTimer, QEventLoop
from PySide6.QtWidgets import QApplication, QWidget, QSlider, QToolButton, QStyle, QStyleOptionSlider
from PySide6.QtTest import QTest
app = QApplication(sys.argv)
print("style", app.style().name())
def wait(ms=300):
    loop = QEventLoop(); QTimer.singleShot(ms, loop.quit); loop.exec()
w = QWidget(); w.resize(400, 200)
s = QSlider(Qt.Horizontal, w); s.setGeometry(20, 20, 200, 30); s.setRange(0, 100); s.setValue(50); s.setFocusPolicy(Qt.NoFocus)
t = QToolButton(w); t.setGeometry(20, 100, 80, 34); t.setText("Bold"); t.setAutoRaise(True); t.setCheckable(True); t.setChecked(True); t.setFocusPolicy(Qt.NoFocus)
w.show(); wait(); print("focus widget", app.focusWidget())
def handle_rect():
    o = QStyleOptionSlider(); s.initStyleOption(o)
    return s.style().subControlRect(QStyle.CC_Slider, o, QStyle.SC_SliderHandle, s)
def row(img, y, x0, x1):
    return [img.pixelColor(x, y).name(QColor.HexArgb if False else 0) for x in range(x0, x1)]
from PySide6.QtGui import QColor
def hexs(img, pts):
    return [img.pixelColor(x, y).name() for (x, y) in pts]
# move pointer away
QTest.mouseMove(w, QPoint(380, 180)); wait()
hr = handle_rect(); cy = hr.center().y(); print("slider handle rect", hr.x(), hr.y(), hr.width(), hr.height())
img = s.grab().toImage()
print("slider rest row y=%d x=%d..%d:" % (cy, hr.left(), hr.left()+5), hexs(img, [(x, cy) for x in range(hr.left(), hr.left()+6)]), "centre", img.pixelColor(hr.center()).name(), "column x=%d y=%d..%d:" % (hr.center().x(), hr.top(), hr.top()+4), hexs(img, [(hr.center().x(), y) for y in range(hr.top(), hr.top()+5)]))
QTest.mouseMove(s, hr.center()); wait()
img = s.grab().toImage()
print("slider hover underMouse", s.underMouse(), "row:", hexs(img, [(x, cy) for x in range(hr.left(), hr.left()+6)]), "centre", img.pixelColor(hr.center()).name(), "column:", hexs(img, [(hr.center().x(), y) for y in range(hr.top(), hr.top()+5)]))
QTest.mouseMove(w, QPoint(380, 180)); wait()
img = t.grab().toImage(); my = t.height()//2
print("checked flat toolbutton %dx%d row y=%d x=0..7:" % (t.width(), t.height(), my), hexs(img, [(x, my) for x in range(0, 8)]), "column x=%d y=0..7:" % (t.width()//2), hexs(img, [(t.width()//2, y) for y in range(0, 8)]), "fill", img.pixelColor(8, my).name())
