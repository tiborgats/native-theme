"""A QMainWindow with a QMenuBar (File, Edit, View) and one QToolBar under breeze, offscreen:
the bar and toolbar at rest, a title hovered, a title with its menu open, the title rects.

Run: see README.md. Args: <light|dark> [proxy]. Every printed line is a pixel read back
from QWidget.grab() or a number Qt returned, except lines marked "computed" or "cfg".
"""
import os

from PySide6.QtCore import QEvent, QPoint, QPointF, QRect, Qt
from PySide6.QtGui import QAction, QMouseEvent, QPalette
from PySide6.QtTest import QTest
from PySide6.QtWidgets import QMainWindow, QToolBar, QWidget

from common import (HERE, PROXY, SCHEME, clear_log, cfg_colours, col_run, corners, header, ink, make_app, out,
                    print_log, px, qrect_str, rgb, row_run, wait)

app = make_app()
header(app)
pal = app.palette()
out("palette Window", pal.color(QPalette.Window).name(), "WindowText", pal.color(QPalette.WindowText).name(),
    "Highlight", pal.color(QPalette.Highlight).name(), "HighlightedText", pal.color(QPalette.HighlightedText).name())
cp = cfg_colours()
out("cfg [Colors:View] DecorationFocus", rgb(cp["Colors:View"]["DecorationFocus"]).name(),
    "DecorationHover", rgb(cp["Colors:View"]["DecorationHover"]).name(),
    "| [Colors:Header] BackgroundNormal", rgb(cp["Colors:Header"]["BackgroundNormal"]).name(),
    "| [Colors:Header][Inactive] BackgroundNormal",
    rgb(cp["Colors:Header][Inactive"]["BackgroundNormal"]).name() if cp.has_section("Colors:Header][Inactive") else None,
    "| [Colors:Window] BackgroundNormal", rgb(cp["Colors:Window"]["BackgroundNormal"]).name())
out("kwinrc in XDG_CONFIG_HOME:", os.path.exists(os.path.join(os.environ["XDG_CONFIG_HOME"], "kwinrc")))

mw = QMainWindow()
mw.setWindowTitle("menubar probe")
mb = mw.menuBar()
menus = {}
for t in ["File", "Edit", "View"]:
    m = mb.addMenu(t)
    m.addAction(f"{t} one")
    m.addAction(f"{t} two")
    menus[t] = m
tb = QToolBar("tools")
for t in ["New", "Open", "Save"]:
    tb.addAction(QAction(t, mw))
mw.addToolBar(tb)
central = QWidget()
mw.setCentralWidget(central)
mw.resize(480, 240)
mw.move(100, 100)
mw.show()
mw.activateWindow()
mw.raise_()
wait(800)
out(f"window active {mw.isActiveWindow()} geometry {qrect_str(mw.geometry())} | menubar in window"
    f" {qrect_str(QRect(mb.mapTo(mw, QPoint(0, 0)), mb.size()))} | toolbar in window"
    f" {qrect_str(QRect(tb.mapTo(mw, QPoint(0, 0)), tb.size()))} | central in window"
    f" {qrect_str(QRect(central.mapTo(mw, QPoint(0, 0)), central.size()))}")
out("attributes: mainwindow WA_StyledBackground", mw.testAttribute(Qt.WA_StyledBackground),
    "autoFillBackground", mw.autoFillBackground(), "| menubar autoFill", mb.autoFillBackground(),
    "| toolbar autoFill", tb.autoFillBackground())
out("palettes: mainwindow Window", mw.palette().color(QPalette.Window).name(),
    "| menubar Window", mb.palette().color(QPalette.Window).name(), "WindowText", mb.palette().color(QPalette.WindowText).name(),
    "| toolbar Window", tb.palette().color(QPalette.Window).name())

acts = {t: menus[t].menuAction() for t in menus}
for t, a in acts.items():
    out(f"title {t}: actionGeometry (menubar coords) {qrect_str(mb.actionGeometry(a))}")

# pointer far away
QTest.mouseMove(mw, QPoint(mw.width() - 10, mw.height() - 10))
wait(600)

# ---------------------------------------------------------------- a. at rest
out("==== a rest")
clear_log()
wimg = mw.grab().toImage()
wimg.save(os.path.join(HERE, f"window_rest_{'proxy_' if PROXY else ''}{SCHEME}.png"))
out(f"window grab {wimg.width()}x{wimg.height()}")
print_log(lambda e: e[0] == "CE_MenuBarItem", "rest ")
mbo = mb.mapTo(mw, QPoint(0, 0))
tbo = tb.mapTo(mw, QPoint(0, 0))
gf, ge, gv = (mb.actionGeometry(acts[t]) for t in ["File", "Edit", "View"])
mid_y = mbo.y() + gf.center().y()
between = mbo.x() + (gf.right() + ge.left()) // 2
out(f"rest: bar fill between File and Edit ({between},{mid_y}) {px(wimg, between, mid_y)};"
    f" right of View ({mbo.x() + gv.right() + 40},{mid_y}) {px(wimg, mbo.x() + gv.right() + 40, mid_y)};"
    f" bar near right end ({wimg.width() - 6},{mid_y}) {px(wimg, wimg.width() - 6, mid_y)}")
tb_mid = tbo.y() + tb.height() // 2
out(f"rest: toolbar fill right of the tools ({wimg.width() - 20},{tb_mid}) {px(wimg, wimg.width() - 20, tb_mid)};"
    f" central ({wimg.width() // 2},{wimg.height() - 20}) {px(wimg, wimg.width() // 2, wimg.height() - 20)}")
cx = wimg.width() - 20
y_end = tbo.y() + tb.height() + 6
out(f"rest: column x={cx} y=0..{y_end - 1} (menubar y={mbo.y()}..{mbo.y() + mb.height() - 1},"
    f" toolbar y={tbo.y()}..{tbo.y() + tb.height() - 1}): {col_run(wimg, cx, 0, y_end)}")
out(f"rest: column x=2 y=0..{y_end - 1}: {col_run(wimg, 2, 0, y_end)}")
for t, g in [("File", gf), ("Edit", ge), ("View", gv)]:
    bimg = mb.grab().toImage()
    bg = bimg.pixelColor(g.left() + 2, g.center().y())
    k = ink(bimg, QRect(g.left(), g.top(), g.width(), g.height()), bg)
    out(f"rest: title {t} rect {qrect_str(g)} bg ({g.left() + 2},{g.center().y()}) {bg.name()};"
        f" text ink bbox {k['bbox'] if k else None} farthest {k['farthest'] if k else None}"
        + (f"; ink left - rect left {k['bbox'][0] - g.left()}, rect right - ink right {g.right() - k['bbox'][2]},"
           f" ink top - rect top {k['bbox'][1] - g.top()}, rect bottom - ink bottom {g.bottom() - k['bbox'][3]}" if k else ""))
rest_bar = mb.grab().toImage()


def title_report(img, g, tag):
    fx = g.left() + 3
    cy = g.center().y()
    out(f"{tag}: rect {qrect_str(g)}; fill (ink-free) ({fx},{cy}) {px(img, fx, cy)}; ({g.right() - 3},{cy}) {px(img, g.right() - 3, cy)};"
        f" ({g.center().x()},{g.top() + 2}) {px(img, g.center().x(), g.top() + 2)}")
    out(f"{tag}: row y={cy} x={g.left() - 1}..{g.left() + 3}: {row_run(img, cy, g.left() - 1, g.left() + 4)};"
        f" x={g.right() - 3}..{g.right() + 1}: {row_run(img, cy, g.right() - 3, g.right() + 2)}")
    out(f"{tag}: column x={g.center().x()} y={g.top() - 1}..{g.top() + 3}: {col_run(img, g.center().x(), g.top() - 1, g.top() + 4)};"
        f" y={g.bottom() - 3}..{g.bottom() + 1}: {col_run(img, g.center().x(), g.bottom() - 3, g.bottom() + 2)}")
    corners(img, g, tag)
    fill = img.pixelColor(fx, cy)
    # 3 px in from each side, so the rounded fill's anti-aliased corners are not taken for ink
    k = ink(img, QRect(g.left() + 3, g.top() + 3, g.width() - 6, g.height() - 6), fill)
    out(f"{tag}: text ink vs fill {fill.name()}: bbox {k['bbox'] if k else None} farthest {k['farthest'] if k else None}"
        f" at {k['at'] if k else None} (exact count {k['same_count'] if k else None} of {k['ink_pixels'] if k else None})")


def real_move(w, pos):
    """QTest.mouseMove first (cursor, Enter, hover events), then the same move as a
    QMouseEvent whose source is MouseEventNotSynthesized: on the offscreen platform every
    move QTest or QCursor.setPos makes arrives as MouseEventSynthesizedByQt, and
    QMenuBar::mouseMoveEvent ignores synthesized moves without a button (its tablet guard),
    so without this the title never becomes the current action."""
    QTest.mouseMove(w, pos)
    wait(300)
    out(f"move to {pos.x()},{pos.y()}: after QTest.mouseMove activeAction",
        mb.activeAction().text() if mb.activeAction() else None)
    ev = QMouseEvent(QEvent.MouseMove, QPointF(pos), QPointF(w.mapToGlobal(pos)), Qt.NoButton, Qt.NoButton, Qt.NoModifier)
    out(f"sending MouseMove source {ev.source().name}")
    app.sendEvent(w, ev)
    wait(800)


# ---------------------------------------------------------------- b. hovered title
out("==== b File hovered, no menu open")
real_move(mb, gf.center())
out("hover: menubar activeAction", mb.activeAction().text() if mb.activeAction() else None,
    "| File menu visible", menus["File"].isVisible(), "| mouse buttons", int(app.mouseButtons().value))
clear_log()
img = mb.grab().toImage()
img.save(os.path.join(HERE, f"menubar_hover_{'proxy_' if PROXY else ''}{SCHEME}.png"))
print_log(lambda e: e[0] == "CE_MenuBarItem", "hover ")
title_report(img, gf, "hover File")
title_report(img, ge, "hover-time Edit (not hovered)")

# ---------------------------------------------------------------- c. menu open
out("==== c File clicked, its menu open")
QTest.mouseClick(mb, Qt.LeftButton, Qt.NoModifier, gf.center())
wait(800)
out("open: File menu visible", menus["File"].isVisible(), "| menubar activeAction",
    mb.activeAction().text() if mb.activeAction() else None, "| popup widget", app.activePopupWidget(),
    "| File menu geometry", qrect_str(menus["File"].geometry()))
clear_log()
img = mb.grab().toImage()
img.save(os.path.join(HERE, f"menubar_open_{'proxy_' if PROXY else ''}{SCHEME}.png"))
print_log(lambda e: e[0] == "CE_MenuBarItem", "open ")
title_report(img, gf, "open File")
title_report(img, ge, "open-time Edit (not hovered)")
# pointer onto Edit while File's menu is open: Qt moves the open menu to Edit
real_move(mb, ge.center())
out("open, pointer moved to Edit: File menu visible", menus["File"].isVisible(), "| Edit menu visible",
    menus["Edit"].isVisible(), "| menubar activeAction", mb.activeAction().text() if mb.activeAction() else None)
clear_log()
img = mb.grab().toImage()
print_log(lambda e: e[0] == "CE_MenuBarItem", "open-moved ")
title_report(img, ge, "open-moved Edit")
for m in menus.values():
    m.hide()
wait(300)
out("done")
