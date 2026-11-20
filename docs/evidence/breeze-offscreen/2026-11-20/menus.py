"""Popup menus (QMenu) under breeze, offscreen: hovered item, pressed item, geometry.

Run: see README.md. Args: <light|dark> [proxy]. Every printed line is a pixel read back
from QWidget.grab() or a number Qt returned, except lines marked "computed" or "cfg".
"""
from PySide6.QtCore import QPoint, QRect, Qt
from PySide6.QtGui import QColor, QFontMetrics, QIcon, QKeySequence, QPalette, QPixmap
from PySide6.QtTest import QTest
from PySide6.QtWidgets import QMenu

import os

from common import (HERE, PROXY, SCHEME, clear_log, cfg_colours, corners, header, ink, make_app, mix, out, over, print_log, px,
                    qrect_str, rgb, row_run, runs, wait)

app = make_app()
header(app)
pal = app.palette()
out("palette Window", pal.color(QPalette.Window).name(), "WindowText", pal.color(QPalette.WindowText).name(),
    "Highlight", pal.color(QPalette.Highlight).name(), "HighlightedText", pal.color(QPalette.HighlightedText).name())
cp = cfg_colours()
v_focus = rgb(cp["Colors:View"]["DecorationFocus"])
v_hover = rgb(cp["Colors:View"]["DecorationHover"])
out("cfg [Colors:View] DecorationFocus", v_focus.name(), "DecorationHover", v_hover.name(),
    "| [Colors:Selection] ForegroundNormal", rgb(cp["Colors:Selection"]["ForegroundNormal"]).name())


def show(menu):
    menu.popup(QPoint(200, 200))
    wait(500)
    out(f"menu visible {menu.isVisible()} geometry {qrect_str(menu.geometry())} isActiveWindow {menu.isActiveWindow()}")


def grab(menu, tag):
    clear_log()
    img = menu.grab().toImage()
    path = f"{tag}_{'proxy_' if PROXY else ''}{SCHEME}.png"
    img.save(os.path.join(HERE, path))
    out(f"{tag}: grabbed {img.width()}x{img.height()} (saved {path})")
    return img


def item_report(img, menu, act, tag, panel):
    g = menu.actionGeometry(act)
    cy = g.center().y()
    cx = g.center().x()
    out(f"{tag}: actionGeometry {qrect_str(g)} (menu widget {menu.width()}x{menu.height()}, origin = panel's top-left)")
    fx = g.left() + 8
    out(f"{tag}: fill (ink-free, left of the text and icon) ({fx},{cy}) {px(img, fx, cy)}; ({g.right() - 6},{cy}) {px(img, g.right() - 6, cy)};"
        f" ({fx},{g.top() + 4}) {px(img, fx, g.top() + 4)}; geometric centre ({cx},{cy}) {px(img, cx, cy)} (may hit text)")
    out(f"{tag}: row y={cy} x={g.left() - 1}..{g.left() + 3} (left edge): {row_run(img, cy, g.left() - 1, g.left() + 4)};"
        f" x={g.right() - 3}..{g.right() + 1} (right edge): {row_run(img, cy, g.right() - 3, g.right() + 2)}")
    out(f"{tag}: column x={cx} y={g.top() - 1}..{g.top() + 3} (top edge): {[px(img, cx, y) for y in range(g.top() - 1, g.top() + 4)]};"
        f" y={g.bottom() - 3}..{g.bottom() + 1} (bottom edge): {[px(img, cx, y) for y in range(g.bottom() - 3, g.bottom() + 2)]}")
    corners(img, g, tag)
    fill = img.pixelColor(fx, cy)
    k = ink(img, QRect(g.left() + 2, g.top() + 2, g.width() - 4, g.height() - 4), fill)
    if not k:
        out(f"{tag}: no ink found in the item")
        return g, None
    rs = runs(k["cols"], gap=4)
    out(f"{tag}: ink vs fill {fill.name()}: bbox {k['bbox']} farthest {k['farthest']} at {k['at']}"
        f" (pixels of exactly that colour {k['same_count']} of {k['ink_pixels']}); column runs {rs}")
    for i, (x0, x1) in enumerate(rs):
        kk = ink(img, QRect(x0, g.top() + 2, x1 - x0 + 1, g.height() - 4), fill)
        out(f"{tag}: ink run {i} x={x0}..{x1}: bbox {kk['bbox']} farthest {kk['farthest']} at {kk['at']}"
            f" (exact count {kk['same_count']} of {kk['ink_pixels']})")
    return g, k


# ------------------------------------------------------------------ text-only menu
out("==== 1 text-only menu: Hide / Edit (Ctrl+E) / Mark")
menu = QMenu()
a_hide = menu.addAction("Hide")
a_edit = menu.addAction("Edit")
a_edit.setShortcut(QKeySequence("Ctrl+E"))
a_mark = menu.addAction("Mark")
show(menu)
for a, n in [(a_hide, "Hide"), (a_edit, "Edit"), (a_mark, "Mark")]:
    out(f"text menu: actionGeometry {n}: {qrect_str(menu.actionGeometry(a))}")
# rest: pointer outside the menu
QTest.mouseMove(menu, QPoint(menu.width() + 40, menu.height() + 40))
wait(500)
out("rest: activeAction", menu.activeAction().text() if menu.activeAction() else None)
img = grab(menu, "menu_text_rest")
gh = menu.actionGeometry(a_hide)
panel = img.pixelColor(gh.right() - 6, gh.center().y())
out(f"rest: panel sample ({gh.right() - 6},{gh.center().y()}) {panel.name()}")
_, k_rest = item_report(img, menu, a_edit, "rest Edit", panel)
kh = ink(img, QRect(gh.left() + 1, gh.top() + 2, gh.width() - 2, gh.height() - 4), panel)
out(f"rest Hide: ink vs panel: bbox {kh['bbox'] if kh else None} farthest {kh['farthest'] if kh else None}")

# a. hovered: pointer moved onto Edit, no button down
ge = menu.actionGeometry(a_edit)
QTest.mouseMove(menu, ge.center())
wait(800)
out("hover: activeAction", menu.activeAction().text() if menu.activeAction() else None,
    "| mouse buttons", int(app.mouseButtons().value))
img = grab(menu, "menu_text_hover")
print_log(lambda e: e[0] == "CE_MenuItem", "hover ")
item_report(img, menu, a_edit, "hover Edit", panel)
fp = QColor(panel)
out(f"computed (not measured): DecorationFocus@0.3 over panel {fp.name()} = {over(v_focus, 0.3, fp)};"
    f" mix(DecorationFocus, WindowText, 0.15) = {mix(v_focus, pal.color(QPalette.WindowText), 0.15)}")

# b. pressed: left button held down on Edit
QTest.mousePress(menu, Qt.LeftButton, Qt.NoModifier, ge.center())
wait(500)
out("pressed: activeAction", menu.activeAction().text() if menu.activeAction() else None,
    "| menu still visible", menu.isVisible(), "| app mouseButtons", int(app.mouseButtons().value))
img = grab(menu, "menu_text_pressed")
print_log(lambda e: e[0] == "CE_MenuItem", "pressed ")
item_report(img, menu, a_edit, "pressed Edit", panel)
out(f"computed (not measured): HighlightedText {pal.color(QPalette.HighlightedText).name()} @0.7 over"
    f" DecorationFocus {v_focus.name()} = {over(pal.color(QPalette.HighlightedText), 0.7, v_focus)};"
    f" WindowText @0.7 over the hover fill measured above is not computed here")
# release off the items so no action fires, then close
QTest.mouseRelease(menu, Qt.LeftButton, Qt.NoModifier, ge.center())
wait(300)
menu.hide()
wait(300)

# ------------------------------------------------------------------ icon menu
out("==== 2 icon menu: Print (16 px #ff0000 icon) / Edit / Exit, no checkable items")
icon_pm = QPixmap(16, 16)
icon_pm.fill(QColor("#ff0000"))
menu2 = QMenu()
b_print = menu2.addAction(QIcon(icon_pm), "Print")
b_edit = menu2.addAction("Edit")
b_exit = menu2.addAction("Exit")
show(menu2)
QTest.mouseMove(menu2, QPoint(menu2.width() + 40, menu2.height() + 40))
wait(500)
for a, n in [(b_print, "Print(icon)"), (b_edit, "Edit"), (b_exit, "Exit")]:
    out(f"icon menu: actionGeometry {n}: {qrect_str(menu2.actionGeometry(a))}")
img = grab(menu2, "menu_icon_rest")
gp = menu2.actionGeometry(b_print)
panel2 = img.pixelColor(gp.right() - 6, gp.center().y())
out(f"icon menu: panel sample ({gp.right() - 6},{gp.center().y()}) {panel2.name()}")
red = [(x, y) for y in range(gp.top(), gp.bottom() + 1) for x in range(gp.left(), gp.right() + 1)
       if img.pixelColor(x, y).name() == "#ff0000"]
if red:
    out(f"icon menu: icon (#ff0000) bbox x={min(r[0] for r in red)}..{max(r[0] for r in red)}"
        f" y={min(r[1] for r in red)}..{max(r[1] for r in red)} count {len(red)}")
    ix1 = max(r[0] for r in red)
    kp = ink(img, QRect(ix1 + 1, gp.top() + 2, gp.right() - ix1 - 2, gp.height() - 4), panel2)
    out(f"icon menu: Print text ink bbox {kp['bbox'] if kp else None}; first ink column minus icon right edge"
        f" {kp['bbox'][0] - ix1 if kp else None}")
else:
    out("icon menu: no #ff0000 pixel found in the Print row")
gd = menu2.actionGeometry(b_edit)
kd = ink(img, QRect(gd.left() + 1, gd.top() + 2, gd.width() - 2, gd.height() - 4), panel2)
out(f"icon menu: Edit (no icon) text ink bbox {kd['bbox'] if kd else None}")
fm = QFontMetrics(menu2.font())
out(f"menu font {menu2.font().toString()} | leftBearing 'P' {fm.leftBearing('P')} 'E' {fm.leftBearing('E')}"
    f" 'H' {fm.leftBearing('H')} | height {fm.height()}")
# hover the icon item too, to see the highlight and the icon together
QTest.mouseMove(menu2, gp.center())
wait(800)
out("icon menu hover: activeAction", menu2.activeAction().text() if menu2.activeAction() else None)
img = grab(menu2, "menu_icon_hover")
print_log(lambda e: e[0] == "CE_MenuItem", "icon hover ")
item_report(img, menu2, b_print, "icon hover Print", panel2)
red = [(x, y) for y in range(gp.top(), gp.bottom() + 1) for x in range(gp.left(), gp.right() + 1)
       if img.pixelColor(x, y).name() == "#ff0000"]
out(f"icon hover: #ff0000 pixels in the Print row: {len(red)}"
    + (f" bbox x={min(r[0] for r in red)}..{max(r[0] for r in red)} y={min(r[1] for r in red)}..{max(r[1] for r in red)}" if red else ""))
menu2.hide()
wait(300)
out("done")
