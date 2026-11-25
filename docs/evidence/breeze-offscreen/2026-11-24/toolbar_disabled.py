"""A QMainWindow with a top QToolBar holding three text-only actions, the middle one disabled,
under breeze, offscreen: the palettes Qt reports (the main window's, the tool bar's and each
tool button's, Active and Disabled WindowText and ButtonText, Window) and the ink of each
button's label read back from QWidget.grab().

Run: ./run.sh toolbar_disabled.py <light|dark>. Every printed line is a pixel read back from
QWidget.grab() or a number Qt returned.
"""
from PySide6.QtCore import QPoint, QRect
from PySide6.QtGui import QAction, QPalette
from PySide6.QtTest import QTest
from PySide6.QtWidgets import QMainWindow, QToolBar, QToolButton, QWidget

from common import header, ink, make_app, out, px, qrect_str, wait

app = make_app()
header(app)

mw = QMainWindow()
mw.setWindowTitle("toolbar probe")
tb = QToolBar("tools")
acts = {}
for t in ["New", "Edit", "Save"]:
    a = QAction(t, mw)
    tb.addAction(a)
    acts[t] = a
acts["Edit"].setEnabled(False)
mw.addToolBar(tb)
mw.setCentralWidget(QWidget())
mw.resize(480, 240)
mw.move(100, 100)
mw.show()
mw.activateWindow()
mw.raise_()
QTest.mouseMove(mw, QPoint(mw.width() - 10, mw.height() - 10))
wait(800)


def pal_str(p):
    return (f"Window A {p.color(QPalette.Active, QPalette.Window).name()} D {p.color(QPalette.Disabled, QPalette.Window).name()}"
            f" | WindowText A {p.color(QPalette.Active, QPalette.WindowText).name()} D {p.color(QPalette.Disabled, QPalette.WindowText).name()}"
            f" | ButtonText A {p.color(QPalette.Active, QPalette.ButtonText).name()} D {p.color(QPalette.Disabled, QPalette.ButtonText).name()}")


out(f"window active {mw.isActiveWindow()} | toolbar area top {mw.toolBarArea(tb).name}"
    f" | toolbar in window {qrect_str(QRect(tb.mapTo(mw, QPoint(0, 0)), tb.size()))}")
out("palette mainwindow:", pal_str(mw.palette()))
out("palette toolbar:", pal_str(tb.palette()), "| breeze_has_toolsarea_palette", tb.property("breeze_has_toolsarea_palette"))
img = mw.grab().toImage()
for t, a in acts.items():
    w = tb.widgetForAction(a)
    assert isinstance(w, QToolButton)
    r = QRect(w.mapTo(mw, QPoint(0, 0)), w.size())
    bg = px(img, r.left() + 1, r.top() + 1)
    out(f"button {t}: enabled {w.isEnabled()} autoRaise {w.autoRaise()} style {w.toolButtonStyle().name}"
        f" rect {qrect_str(r)} | palette {pal_str(w.palette())}"
        f" | current group {w.palette().currentColorGroup().name}")
    out(f"button {t}: fill at its corner +1 {bg} | label ink {ink(img, r.adjusted(2, 2, -2, -2), bg)}")
