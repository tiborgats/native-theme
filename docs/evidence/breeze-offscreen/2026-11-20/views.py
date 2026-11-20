"""A hovered, unselected row of a QListWidget and of a QTreeWidget under breeze, offscreen,
in an active window, once with a QLineEdit holding the keyboard focus and once with the
view focused.

Run: see README.md. Args: <light|dark> [proxy]. Every printed line is a pixel read back
from QWidget.grab() or a number Qt returned, except lines marked "computed".
"""
import os

from PySide6.QtCore import QPoint, QRect
from PySide6.QtGui import QColor, QPalette
from PySide6.QtTest import QTest
from PySide6.QtWidgets import QHBoxLayout, QLineEdit, QListWidget, QTreeWidget, QTreeWidgetItem, QVBoxLayout, QWidget

from common import (HERE, PROXY, SCHEME, clear_log, col_run, corners, header, ink, make_app, out, over, print_log,
                    px, qrect_str, row_run, wait)

app = make_app()
header(app)
pal = app.palette()
base = pal.color(QPalette.Active, QPalette.Base)
hl_a = pal.color(QPalette.Active, QPalette.Highlight)
hl_i = pal.color(QPalette.Inactive, QPalette.Highlight)
out("palette Active Base", base.name(), "Text", pal.color(QPalette.Active, QPalette.Text).name(),
    "| Active Highlight", hl_a.name(), "| Inactive Highlight", hl_i.name(),
    "| Inactive Base", pal.color(QPalette.Inactive, QPalette.Base).name())
for name, h in [("Active", hl_a), ("Inactive", hl_i)]:
    f = over(h, 0.3, base)
    out(f"computed (not measured): {name} Highlight@0.3 over Base = {f}; outline {name} Highlight@0.8 over that fill"
        f" = {over(h, 0.8, QColor(f))}")

win = QWidget()
lay = QVBoxLayout(win)
le = QLineEdit("focus here")
lay.addWidget(le)
row = QHBoxLayout()
lw = QListWidget()
for i in range(8):
    lw.addItem(f"List row {i}")
tw = QTreeWidget()
tw.setHeaderLabels(["Name"])
for i in range(8):
    QTreeWidgetItem(tw, [f"Tree row {i}"])
row.addWidget(lw)
row.addWidget(tw)
lay.addLayout(row)
win.resize(480, 320)
win.move(100, 100)
win.show()
win.activateWindow()
win.raise_()
wait(800)
out(f"window active {win.isActiveWindow()} | list selectionMode {lw.selectionMode().name} | tree selectionMode {tw.selectionMode().name}")

HOVER_ROW = 3


def measure(view, tag):
    idx = view.model().index(HOVER_ROW, 0)
    r = view.visualRect(idx)
    vp = view.viewport()
    # pointer away, then onto the row's text-free right part
    QTest.mouseMove(win, QPoint(win.width() - 5, 5))
    wait(300)
    pos = QPoint(r.right() - 12, r.center().y())
    QTest.mouseMove(vp, pos)
    wait(800)
    out(f"{tag}: focusWidget {type(app.focusWidget()).__name__ if app.focusWidget() else None} | view hasFocus {view.hasFocus()}"
        f" | window active {win.isActiveWindow()} | viewport underMouse {vp.underMouse()} | pointer (viewport) {pos.x()},{pos.y()}"
        f" | currentIndex row {view.currentIndex().row()} | selected rows {[i.row() for i in view.selectionModel().selectedIndexes()]}")
    clear_log()
    img = vp.grab().toImage()
    img.save(os.path.join(HERE, f"{tag.replace(' ', '_')}_{'proxy_' if PROXY else ''}{SCHEME}.png"))
    print_log(lambda e: e[0] == "PE_PanelItemViewItem", f"{tag} ")
    out(f"{tag}: visualRect row {HOVER_ROW} (viewport coords) {qrect_str(r)}; viewport {vp.width()}x{vp.height()}")
    cy = r.center().y()
    fx = r.right() - 20
    out(f"{tag}: fill (text-free) ({fx},{cy}) {px(img, fx, cy)}; ({r.right() - 40},{cy}) {px(img, r.right() - 40, cy)};"
        f" base outside the rows ({vp.width() - 4},{vp.height() - 4}) {px(img, vp.width() - 4, vp.height() - 4)}")
    out(f"{tag}: row y={cy} x={r.left()}..{r.left() + 6}: {row_run(img, cy, r.left(), r.left() + 7)};"
        f" x={r.right() - 6}..{r.right()}: {row_run(img, cy, r.right() - 6, r.right() + 1)}")
    out(f"{tag}: column x={fx} y={r.top() - 1}..{r.top() + 4}: {col_run(img, fx, r.top() - 1, r.top() + 5)};"
        f" y={r.bottom() - 4}..{r.bottom() + 1}: {col_run(img, fx, r.bottom() - 4, r.bottom() + 2)}")
    # the painted highlight's extent on the row through the centre and the column at fx
    bgc = img.pixelColor(vp.width() - 4, vp.height() - 4)
    # scanned inside the item's own columns only (a tree's branch area left of it holds the branch lines)
    xs = [x for x in range(r.left(), min(vp.width(), r.right() + 1)) if img.pixelColor(x, cy) != bgc]
    ys = [y for y in range(max(0, r.top() - 3), min(vp.height(), r.bottom() + 4)) if img.pixelColor(fx, y) != bgc]
    if xs and ys:
        hr = QRect(xs[0], ys[0], xs[-1] - xs[0] + 1, ys[-1] - ys[0] + 1)
        out(f"{tag}: painted highlight extent (pixels differing from the base sample) {qrect_str(hr)}")
        corners(img, hr, tag)
        fill = img.pixelColor(fx, cy)
        k = ink(img, QRect(hr.left() + 2, hr.top() + 2, hr.width() - 4, hr.height() - 4), fill)
        out(f"{tag}: text ink vs fill: bbox {k['bbox'] if k else None} farthest {k['farthest'] if k else None}")
    else:
        out(f"{tag}: no painted highlight found around row {HOVER_ROW}")


for view, vname in [(lw, "list"), (tw, "tree")]:
    le.setFocus()
    wait(400)
    measure(view, f"{vname} lineedit-focused")
    view.setFocus()
    wait(400)
    measure(view, f"{vname} view-focused")
out("done")
