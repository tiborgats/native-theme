"""Measure what the Breeze widget style paints, offscreen.

Run: XDG_CONFIG_HOME=<cfg-dir> XDG_CONFIG_DIRS=<empty> QT_QPA_PLATFORM=offscreen
     QT_QPA_PLATFORMTHEME=kde python3 measure.py <light|dark>
Every line printed is a measurement; PNGs land next to this script.
"""
import os
import sys

from PySide6.QtCore import QEventLoop, QPoint, QRect, QSize, Qt, QTimer
from PySide6.QtGui import (QColor, QFont, QFontInfo, QFontMetrics, QFontMetricsF,
                           QGuiApplication, QIcon, QImage, QPainter, QPalette, QPixmap)
from PySide6.QtTest import QTest
from PySide6.QtWidgets import (QAbstractItemView, QApplication, QCheckBox, QComboBox,
                               QDialog, QHBoxLayout, QLineEdit, QListWidget, QMenu,
                               QPushButton, QScrollArea, QStyle, QStyleOption,
                               QStyleOptionSlider, QStyleOptionViewItem, QTableWidget,
                               QTableWidgetItem, QTreeWidget, QTreeWidgetItem, QWidget)

SCHEME = sys.argv[1]
OUT = os.path.dirname(os.path.abspath(__file__))
os.environ.setdefault("QT_SCALE_FACTOR", "1")

app = QApplication(sys.argv[:1])
style = app.style()
pal = app.palette()


def out(*a):
    print(f"[{SCHEME}]", *a, flush=True)


def wait(ms):
    loop = QEventLoop()
    QTimer.singleShot(ms, loop.quit)
    loop.exec()


def hexc(c):
    return c.name(QColor.HexArgb) if c.alpha() < 255 else c.name()


def px(img, x, y):
    return hexc(img.pixelColor(x, y))


def save(img, name):
    path = os.path.join(OUT, f"{name}_{SCHEME}.png")
    img.save(path)
    out(f"saved {path} size {img.width()}x{img.height()}")
    return img


def lum(c):
    return 0.299 * c.red() + 0.587 * c.green() + 0.114 * c.blue()


def blend(fg, alpha, bg):
    f, b = QColor(fg), QColor(bg)
    r = round(alpha * f.red() + (1 - alpha) * b.red())
    g = round(alpha * f.green() + (1 - alpha) * b.green())
    bl = round(alpha * f.blue() + (1 - alpha) * b.blue())
    return QColor(r, g, bl).name()


def diff(a, b):
    return max(abs(a.red() - b.red()), abs(a.green() - b.green()), abs(a.blue() - b.blue()))


def glyph_extreme(img, rect, bg, thresh=40):
    """Darkest and lightest pixel differing from bg by more than thresh, plus bbox."""
    bgc = QColor(bg)
    dark = light = None
    xs, ys = [], []
    for y in range(rect.top(), rect.bottom() + 1):
        for x in range(rect.left(), rect.right() + 1):
            c = img.pixelColor(x, y)
            if diff(c, bgc) > thresh:
                xs.append(x)
                ys.append(y)
                if dark is None or lum(c) < lum(dark[0]):
                    dark = (c, x, y)
                if light is None or lum(c) > lum(light[0]):
                    light = (c, x, y)
    if not xs:
        return None
    return dict(dark=(hexc(dark[0]), dark[1], dark[2]), light=(hexc(light[0]), light[1], light[2]),
                bbox=(min(xs), min(ys), max(xs), max(ys)))


def row_run(img, y, x0, x1):
    return [px(img, x, y) for x in range(x0, x1)]


def col_run(img, x, y0, y1):
    return [px(img, x, y) for y in range(y0, y1)]


def place(w, x=600, y=400):
    w.move(x, y)
    w.show()
    w.activateWindow()
    w.raise_()
    wait(300)


out("platform", QGuiApplication.platformName(), "| style", style.objectName(), style.metaObject().className())
out("palette Window", pal.color(QPalette.Window).name(), "WindowText", pal.color(QPalette.WindowText).name(),
    "Base", pal.color(QPalette.Base).name(), "Text", pal.color(QPalette.Text).name(),
    "Button", pal.color(QPalette.Button).name(), "Highlight", pal.color(QPalette.Highlight).name(),
    "Mid", pal.color(QPalette.Mid).name(), "Inactive Highlight", pal.color(QPalette.Inactive, QPalette.Highlight).name(),
    "Inactive HighlightedText", pal.color(QPalette.Inactive, QPalette.HighlightedText).name())
out("app font", app.font().toString(), "| QFontInfo family", QFontInfo(app.font()).family(),
    "pointSizeF", QFontInfo(app.font()).pointSizeF(), "pixelSize", QFontInfo(app.font()).pixelSize())
scr = app.primaryScreen()
out("screen logicalDPI", scr.logicalDotsPerInch(), "physicalDPI", scr.physicalDotsPerInch(), "dpr", scr.devicePixelRatio())
out("cursor pos", QGuiApplication.primaryScreen().geometry(), )

WIN = pal.color(QPalette.Window)
BASE = pal.color(QPalette.Base)
TEXT = pal.color(QPalette.Text)
WTEXT = pal.color(QPalette.WindowText)

# ---------------------------------------------------------------- 1. scroll bars
out("==== 1 scroll bar")


def measure_scrollbar(owner, tag, view_colour, sep_fg):
    img = save(owner.grab().toImage(), f"sb_{tag}")
    sb = owner.verticalScrollBar()
    o = sb.mapTo(owner, QPoint(0, 0))
    out(f"{tag}: scrollbar visible {sb.isVisible()} geometry-in-owner x={o.x()} y={o.y()} w={sb.width()} h={sb.height()}"
        f" range {sb.minimum()}..{sb.maximum()} value {sb.value()}")
    vp = owner.viewport()
    vo = vp.mapTo(owner, QPoint(0, 0))
    out(f"{tag}: viewport-in-owner x={vo.x()} y={vo.y()} w={vp.width()} h={vp.height()}"
        f" | viewport sample (x={vo.x() + vp.width() - 5}, y={vo.y() + vp.height() - 5}) = {px(img, vo.x() + vp.width() - 5, vo.y() + vp.height() - 5)}")
    # handle: scan the scrollbar's middle column from the top for the handle span
    cx = o.x() + sb.width() // 2
    col = col_run(img, cx, o.y(), o.y() + sb.height())
    track = px(img, cx, o.y() + sb.height() - 15)
    out(f"{tag}: track sample (x={cx}, y={o.y() + sb.height() - 15}) = {track}  expected viewport {view_colour}")
    # also two more track samples, near the edge of the bar opposite the separator
    out(f"{tag}: track samples x={o.x() + 3},{o.x() + sb.width() - 3} at y={o.y() + sb.height() - 15}:"
        f" {px(img, o.x() + 3, o.y() + sb.height() - 15)} {px(img, o.x() + sb.width() - 3, o.y() + sb.height() - 15)}")
    hy = [i for i, c in enumerate(col) if c != track]
    out(f"{tag}: middle column non-track rows (relative to bar top): {hy[:3]}...{hy[-3:] if hy else []} count {len(hy)}")
    if hy:
        top, bot = hy[0], hy[-1]
        mid = o.y() + (top + bot) // 2
        rowc = row_run(img, mid, o.x(), o.x() + sb.width())
        out(f"{tag}: row through handle middle y={mid}, x={o.x()}..{o.x() + sb.width() - 1}: {rowc}")
        nontrack = [i for i, c in enumerate(rowc) if c != track]
        out(f"{tag}: non-track columns in that row (relative): {nontrack}")
        out(f"{tag}: handle fill at centre (x={cx}, y={mid}) = {px(img, cx, mid)}; handle span rows {top}..{bot}"
            f" (length {bot - top + 1})")
    # separator: first column of the bar, at the bottom (beside the track)
    sy = o.y() + sb.height() - 15
    out(f"{tag}: separator column x={o.x()} at y={sy} = {px(img, o.x(), sy)};"
        f" x={o.x() - 1} (viewport side) = {px(img, o.x() - 1, sy)}; x={o.x() + 1} = {px(img, o.x() + 1, sy)}"
        f" | expected {sep_fg} @0.2 over {view_colour} = {blend(sep_fg, 0.2, view_colour)}")
    return sb, o


lw = QListWidget()
for i in range(100):
    lw.addItem(f"Row {i}")
lw.resize(300, 200)
place(lw)
out("list: isActiveWindow", lw.isActiveWindow(), "hasFocus", lw.hasFocus())
measure_scrollbar(lw, "list_focused", BASE.name(), TEXT.name())
lw.clearFocus()
wait(800)
out("list after clearFocus: isActiveWindow", lw.isActiveWindow(), "hasFocus", lw.hasFocus(),
    "focusWidget", app.focusWidget())
sb_l, o_l = measure_scrollbar(lw, "list", BASE.name(), TEXT.name())

sa = QScrollArea()
content = QWidget()
content.setFixedSize(200, 2000)
sa.setWidget(content)
sa.resize(300, 200)
place(sa, 1000, 400)
out("scrollarea: isActiveWindow", sa.isActiveWindow(), "hasFocus", sa.hasFocus(), "focusWidget", app.focusWidget())
measure_scrollbar(sa, "scrollarea_focused", WIN.name(), TEXT.name())
sa.clearFocus()
wait(800)
out("scrollarea after clearFocus: hasFocus", sa.hasFocus(), "focusWidget", app.focusWidget())
sb_s, o_s = measure_scrollbar(sa, "scrollarea", WIN.name(), TEXT.name())
out("scrollarea: viewport backgroundRole", sa.viewport().backgroundRole(), "autoFill", sa.viewport().autoFillBackground(),
    "content backgroundRole", content.backgroundRole(), "content autoFill", content.autoFillBackground())

out("metrics: PM_ScrollBarExtent", style.pixelMetric(QStyle.PM_ScrollBarExtent),
    "PM_ScrollBarSliderMin", style.pixelMetric(QStyle.PM_ScrollBarSliderMin))

# hand-built QStyleOptionSlider, at rest and hovered on the slider, widget = None
for name, st, act in [("rest", QStyle.State_Enabled, QStyle.SC_None),
                      ("hover", QStyle.State_Enabled | QStyle.State_MouseOver, QStyle.SC_ScrollBarSlider)]:
    opt = QStyleOptionSlider()
    opt.rect = QRect(0, 0, 21, 200)
    opt.palette = pal
    opt.orientation = Qt.Vertical
    opt.minimum, opt.maximum, opt.pageStep, opt.singleStep = 0, 100, 50, 1
    opt.sliderPosition = opt.sliderValue = 0
    opt.state = st
    opt.subControls = QStyle.SC_All
    opt.activeSubControls = act
    opt.direction = Qt.LeftToRight
    img = QImage(21, 200, QImage.Format_ARGB32)
    img.fill(BASE)
    p = QPainter(img)
    style.drawComplexControl(QStyle.CC_ScrollBar, opt, p, None)
    p.end()
    save(img, f"sb_option_{name}")
    sl = style.subControlRect(QStyle.CC_ScrollBar, opt, QStyle.SC_ScrollBarSlider, None)
    my = sl.center().y()
    out(f"option {name}: slider rect {sl.x()},{sl.y()},{sl.width()}x{sl.height()}; row y={my}: {row_run(img, my, 0, 21)}")
    out(f"option {name}: handle centre (x=10,y={my}) = {px(img, 10, my)}; track (x=10,y=190) = {px(img, 10, 190)};"
        f" separator (x=0,y=190) = {px(img, 0, 190)} (canvas filled with Base {BASE.name()})")

# real hover on the list's scroll bar handle through QTest.mouseMove (synthesised window-system event)
sl_opt = QStyleOptionSlider()
sl_opt.initFrom(sb_l)
sl_opt.orientation = Qt.Vertical
sl_opt.minimum, sl_opt.maximum, sl_opt.pageStep = sb_l.minimum(), sb_l.maximum(), sb_l.pageStep()
sl_opt.sliderPosition = sl_opt.sliderValue = sb_l.value()
sl_opt.subControls = QStyle.SC_All
hr = style.subControlRect(QStyle.CC_ScrollBar, sl_opt, QStyle.SC_ScrollBarSlider, sb_l)
out(f"list real hover: slider rect {hr.x()},{hr.y()},{hr.width()}x{hr.height()}, moving pointer to {hr.center().x()},{hr.center().y()}")
QTest.mouseMove(sb_l, hr.center())
wait(800)
QTest.mouseMove(sb_l, hr.center() + QPoint(0, 1))
wait(800)
out("list real hover: sb.underMouse()", sb_l.underMouse())
img = save(lw.grab().toImage(), "sb_list_hovered")
hmid = o_l.y() + hr.center().y()
hcx = o_l.x() + sb_l.width() // 2
out(f"list real hover: handle centre (x={hcx}, y={hmid}) = {px(img, hcx, hmid)}; row: {row_run(img, hmid, o_l.x(), o_l.x() + 21)}")
QTest.mouseMove(lw.viewport(), QPoint(20, 20))
wait(800)

# ---------------------------------------------------------------- 2. popup menu
out("==== 2 popup menu")


def first_non(img, y, ref, x0, x1, step=1):
    rng = range(x0, x1, step) if step > 0 else range(x0, x1, -1)
    for x in rng:
        if px(img, x, y) != ref:
            return x
    return None


def measure_menu(menu, tag, items, active, panel_probe_action):
    menu.popup(QPoint(200, 200))
    wait(300)
    menu.setActiveAction(active)
    wait(800)
    img = save(menu.grab().toImage(), tag)
    W, H = img.width(), img.height()
    out(f"{tag}: image {W}x{H} format {img.format()} hasAlpha {img.hasAlphaChannel()}; menu font {menu.font().toString()}"
        f" family {QFontInfo(menu.font()).family()} pixelSize {QFontInfo(menu.font()).pixelSize()}")
    for a, n in items:
        g = menu.actionGeometry(a)
        out(f"{tag}: actionGeometry {n}: x={g.x()} y={g.y()} w={g.width()} h={g.height()}")
    gq = menu.actionGeometry(panel_probe_action)
    panel = px(img, W - 12, gq.center().y())
    out(f"{tag}: panel sample (x={W - 12}, y={gq.center().y()}) = {panel}; (x={W // 2}, y={gq.top() + 1}) = {px(img, W // 2, gq.top() + 1)};"
        f" (x=10, y={H - 6}) = {px(img, 10, H - 6)}")
    out(f"{tag}: left edge row y={gq.center().y()} x=0..5: {row_run(img, gq.center().y(), 0, 6)}; right edge x={W - 6}..{W - 1}:"
        f" {row_run(img, gq.center().y(), W - 6, W)}")
    out(f"{tag}: top edge column x={W // 2} y=0..5: {col_run(img, W // 2, 0, 6)}; bottom y={H - 6}..{H - 1}: {col_run(img, W // 2, H - 6, H)}")
    out(f"{tag}: corner (0,0)..(3,3) diagonal: {[px(img, i, i) for i in range(4)]}")
    # active item
    go = menu.actionGeometry(active)
    my = go.center().y()
    hl = first_non(img, my, panel, 1, W - 1)
    hr = first_non(img, my, panel, W - 3, 0, -1)
    hx = go.right() - 20
    col = col_run(img, hx, go.top() - 3, go.bottom() + 4)
    rows = [go.top() - 3 + i for i, c in enumerate(col) if c != panel]
    out(f"{tag}: active row y={my}: highlight columns {hl}..{hr} (width {hr - hl + 1}); pixels x={hl - 1}..{hl + 2}: {row_run(img, my, hl - 1, hl + 3)};"
        f" x={hr - 2}..{hr + 1}: {row_run(img, my, hr - 2, hr + 2)}")
    out(f"{tag}: active column x={hx} y={go.top() - 3}..{go.bottom() + 3}: first 5 {col[:5]} last 5 {col[-5:]}; highlight rows {rows[0]}..{rows[-1]} (height {len(rows)})")
    fill = px(img, hx, my)
    out(f"{tag}: active fill (x={hx}, y={my}) = {fill}")
    ge = glyph_extreme(img, QRect(hl + 2, go.top() + 2, go.width() - 30, go.height() - 4), fill)
    out(f"{tag}: active text glyphs vs fill: {ge}")
    if ge:
        out(f"{tag}: highlight left edge {hl} -> active text ink left {ge['bbox'][0]}: {ge['bbox'][0] - hl} px;"
            f" first char '{active.text()[0]}' leftBearing {QFontMetrics(menu.font()).leftBearing(active.text()[0])}")
    return img, panel


icon_pm = QPixmap(16, 16)
icon_pm.fill(QColor("#ff0000"))
menu = QMenu()
a_open = menu.addAction("Edit")
a_save = menu.addAction(QIcon(icon_pm), "Print")
a_wrap = menu.addAction("Lock toolbars")
a_wrap.setCheckable(True)
a_props = menu.addAction("Properties")
sep = menu.addSeparator()
sub = menu.addMenu("More")
sub.addAction("Inner")
a_quit = menu.addAction("Exit")
items = [(a_open, "Edit(active)"), (a_save, "Print(icon)"), (a_wrap, "Lock toolbars(checkable)"), (a_props, "Properties"),
         (sep, "separator"), (sub.menuAction(), "More(submenu)"), (a_quit, "Exit")]
mimg, panel = measure_menu(menu, "menu", items, a_open, a_props)
W = mimg.width()
# normal (non-active) text
gp = menu.actionGeometry(a_props)
ge2 = glyph_extreme(mimg, QRect(gp.x() + 1, gp.top() + 2, gp.width() - 30, gp.height() - 4), panel)
out(f"menu: normal text glyphs (Properties) {ge2}")
# icon item
gs = menu.actionGeometry(a_save)
red = [(x, y) for y in range(gs.top(), gs.bottom() + 1) for x in range(gs.left(), gs.right() + 1)
       if mimg.pixelColor(x, y).name() == "#ff0000"]
if red:
    ix0, ix1 = min(r[0] for r in red), max(r[0] for r in red)
    iy0, iy1 = min(r[1] for r in red), max(r[1] for r in red)
    out(f"menu: icon (pure #ff0000) bbox x={ix0}..{ix1} y={iy0}..{iy1} (item row y={gs.top()}..{gs.bottom()})")
    gsx = glyph_extreme(mimg, QRect(ix1 + 1, gs.top() + 2, 50, gs.height() - 4), panel)
    out(f"menu: Print text glyphs right of icon: {gsx}")
    if gsx:
        out(f"menu: icon right edge x={ix1} -> text ink left x={gsx['bbox'][0]}: blank columns between = {gsx['bbox'][0] - ix1 - 1};"
            f" 'P' leftBearing {QFontMetrics(menu.font()).leftBearing('P')}")
else:
    out("menu: icon: no #ff0000 pixel found")
# check box item
gc = menu.actionGeometry(a_wrap)
gcx = glyph_extreme(mimg, QRect(gc.x() + 1, gc.top() + 1, 40, gc.height() - 2), panel, thresh=20)
out(f"menu: check box item, non-panel bbox in x={gc.x() + 1}..{gc.x() + 40}: {gcx}")
# separator line
gsep = menu.actionGeometry(sep)
scol = col_run(mimg, W // 2, gsep.top(), gsep.bottom() + 1)
out(f"menu: separator column x={W // 2} rows {gsep.top()}..{gsep.bottom()}: {scol}")
for i, c in enumerate(scol):
    if c != panel:
        y = gsep.top() + i
        xs = [x for x in range(1, W - 2) if px(mimg, x, y) != panel]
        out(f"menu: separator line y={y} colour {c}; non-panel x (excluding border columns) {xs[0]}..{xs[-1]};"
            f" pixels x={xs[0] - 1}..{xs[0] + 1}: {row_run(mimg, y, xs[0] - 1, xs[0] + 2)}, x={xs[-1] - 1}..{xs[-1] + 1}: {row_run(mimg, y, xs[-1] - 1, xs[-1] + 2)}")
menu.hide()
wait(200)

# plain menu, text only
menu2 = QMenu()
b1 = menu2.addAction("Hide")
b2 = menu2.addAction("Edit")
b3 = menu2.addAction("Mark")
m2, p2 = measure_menu(menu2, "menu_textonly", [(b1, "Hide"), (b2, "Edit(active)"), (b3, "Mark")], b2, b1)
g1 = menu2.actionGeometry(b1)
ga = glyph_extreme(m2, QRect(1, g1.top() + 2, g1.width() - 10, g1.height() - 4), p2)
out(f"menu_textonly: non-active 'Hide' text ink bbox {ga['bbox'] if ga else None} (menu left edge x=0, item rect x={g1.x()})")
menu2.hide()
wait(200)
# ---------------------------------------------------------------- 3. item views
out("==== 3 item views")


def view_option(state, group_active=True, text="Row text"):
    o = QStyleOptionViewItem()
    o.initFrom(lw)
    o.rect = QRect(10, 10, 200, 24)
    o.state = state
    o.palette = pal
    o.text = text
    o.features = QStyleOptionViewItem.HasDisplay
    o.viewItemPosition = QStyleOptionViewItem.OnlyOne
    o.showDecorationSelected = True
    o.widget = lw
    o.font = lw.font()
    o.displayAlignment = Qt.AlignLeft | Qt.AlignVCenter
    return o


cases = [
    ("hover", QStyle.State_Enabled | QStyle.State_Active | QStyle.State_MouseOver),
    ("selected_active", QStyle.State_Enabled | QStyle.State_Active | QStyle.State_Selected | QStyle.State_HasFocus),
    ("selected_inactive", QStyle.State_Enabled | QStyle.State_Selected),
    ("selected_hover", QStyle.State_Enabled | QStyle.State_Active | QStyle.State_Selected | QStyle.State_MouseOver),
]
for name, st in cases:
    # primitive only
    img = QImage(220, 44, QImage.Format_ARGB32)
    img.fill(BASE)
    p = QPainter(img)
    style.drawPrimitive(QStyle.PE_PanelItemViewItem, view_option(st, text=""), p, lw)
    p.end()
    save(img, f"view_prim_{name}")
    out(f"view prim {name}: fill centre (x=180,y=22) = {px(img, 180, 22)}; row y=22 x=6..16: {row_run(img, 22, 6, 17)};"
        f" column x=180 y=6..14: {col_run(img, 180, 6, 15)}")
    # full item (panel + text) through CE_ItemViewItem
    img = QImage(220, 44, QImage.Format_ARGB32)
    img.fill(BASE)
    p = QPainter(img)
    style.drawControl(QStyle.CE_ItemViewItem, view_option(st), p, lw)
    p.end()
    save(img, f"view_item_{name}")
    f = px(img, 180, 22)
    ge = glyph_extreme(img, QRect(14, 12, 120, 20), f)
    out(f"view item {name}: fill (x=180,y=22) = {f}; text glyphs {ge}")

# real QListWidget states
lw2 = QListWidget()
for i in range(6):
    lw2.addItem(f"Item number {i}")
lw2.resize(260, 200)
place(lw2, 200, 200)
lw2.setFocus()
lw2.setCurrentRow(1)
wait(300)
out("real list: isActiveWindow", lw2.isActiveWindow(), "hasFocus", lw2.hasFocus())
img = save(lw2.grab().toImage(), "view_list_selected_focused")
r1 = lw2.visualItemRect(lw2.item(1))
vo = lw2.viewport().mapTo(lw2, QPoint(0, 0))
yy = vo.y() + r1.center().y()
xx = vo.x() + r1.right() - 20
f = px(img, xx, yy)
out(f"real list selected+focused: row rect {r1.x()},{r1.y()},{r1.width()}x{r1.height()}; fill (x={xx},y={yy}) = {f};"
    f" text {glyph_extreme(img, QRect(vo.x() + 4, vo.y() + r1.top() + 2, 120, r1.height() - 4), f)}")
# column through rows 0..3 at x right of the text, any lines between rows?
colx = vo.x() + r1.right() - 20
out(f"real list column x={colx} rows {vo.y()}..{vo.y() + 4 * r1.height()}: {sorted(set(col_run(img, colx, vo.y(), vo.y() + 4 * r1.height())))}")
out(f"real list rows 2..3 column x={colx}: {col_run(img, colx, vo.y() + 2 * r1.height() - 2, vo.y() + 2 * r1.height() + 3)}")
# make it inactive: activate another window
other = QWidget()
other.resize(100, 100)
place(other, 1500, 100)
wait(300)
out("real list after other window activated: isActiveWindow", lw2.isActiveWindow(), "other active", other.isActiveWindow())
img = save(lw2.grab().toImage(), "view_list_selected_inactive")
f = px(img, xx, yy)
out(f"real list selected, window inactive: fill (x={xx},y={yy}) = {f};"
    f" text {glyph_extreme(img, QRect(vo.x() + 4, vo.y() + r1.top() + 2, 120, r1.height() - 4), f)}")
other.hide()
lw2.activateWindow()
wait(300)
# real hover over row 3
r3 = lw2.visualItemRect(lw2.item(3))
QTest.mouseMove(lw2.viewport(), r3.center())
wait(800)
img = save(lw2.grab().toImage(), "view_list_hover")
out(f"real list hover row 3 (active window {lw2.isActiveWindow()}): fill (x={xx},y={vo.y() + r3.center().y()}) = "
    f"{px(img, xx, vo.y() + r3.center().y())}")
QTest.mouseMove(lw2.viewport(), QPoint(5, 190))
wait(300)

# table
tw = QTableWidget(3, 3)
for r in range(3):
    for c in range(3):
        tw.setItem(r, c, QTableWidgetItem(f"r{r}c{c}"))
tw.setHorizontalHeaderLabels(["Name", "Value", "Size"])
tw.resize(400, 200)
place(tw, 300, 300)
img = save(tw.grab().toImage(), "view_table")
out("table showGrid", tw.showGrid(), "gridStyle", tw.gridStyle(), "palette Mid", tw.palette().color(QPalette.Mid).name(),
    "app palette Mid", pal.color(QPalette.Mid).name())
so = QStyleOption()
so.initFrom(tw)
gh = style.styleHint(QStyle.SH_Table_GridLineColor, so, tw)
gh0 = style.styleHint(QStyle.SH_Table_GridLineColor)
out(f"styleHint SH_Table_GridLineColor with option+widget: raw {gh} -> {hexc(QColor.fromRgba(gh & 0xffffffff))};"
    f" without option: raw {gh0} -> {hexc(QColor.fromRgba(gh0 & 0xffffffff))}")
vo = tw.viewport().mapTo(tw, QPoint(0, 0))
ry = tw.rowViewportPosition(0) + tw.rowHeight(0) - 1
cxm = tw.columnViewportPosition(1) + tw.columnWidth(1) // 2
out(f"table viewport at {vo.x()},{vo.y()}; row0 height {tw.rowHeight(0)}; column x={vo.x() + cxm} around row0/row1 boundary"
    f" y={vo.y() + ry - 2}..{vo.y() + ry + 2}: {col_run(img, vo.x() + cxm, vo.y() + ry - 2, vo.y() + ry + 3)}")
cx_line = tw.columnViewportPosition(0) + tw.columnWidth(0) - 1
yb = tw.rowViewportPosition(1) + tw.rowHeight(1) // 2
out(f"table row y={vo.y() + yb} around col0/col1 boundary x={vo.x() + cx_line - 2}..{vo.x() + cx_line + 2}:"
    f" {row_run(img, vo.y() + yb, vo.x() + cx_line - 2, vo.x() + cx_line + 3)}")
# header
hh = tw.horizontalHeader()
himg = save(hh.grab().toImage(), "header_table")
s0w = hh.sectionSize(0)
out(f"table header size {hh.width()}x{hh.height()} section0 width {s0w}")
out(f"table header column x={s0w - 10}: {col_run(himg, s0w - 10, 0, hh.height())}")
out(f"table header row y={hh.height() // 2} x={s0w - 4}..{s0w + 3}: {row_run(himg, hh.height() // 2, s0w - 4, s0w + 4)}")
hf = px(himg, s0w - 10, hh.height() // 2)
out(f"table header section fill (x={s0w - 10}, y={hh.height() // 2}) = {hf}")
out(f"table header text glyphs: {glyph_extreme(himg, QRect(0, 1, s0w - 2, hh.height() - 3), hf)}")

tree = QTreeWidget()
tree.setColumnCount(2)
tree.setHeaderLabels(["Name", "Value"])
for i in range(6):
    tree.addTopLevelItem(QTreeWidgetItem([f"Node {i}", f"{i}"]))
tree.resize(300, 220)
place(tree, 500, 300)
img = save(tree.grab().toImage(), "view_tree")
vo = tree.viewport().mapTo(tree, QPoint(0, 0))
rh = tree.visualItemRect(tree.topLevelItem(0)).height()
cxx = vo.x() + tree.columnWidth(0) - 20
out(f"tree row height {rh}; colours in column x={cxx} over rows 0..4: {sorted(set(col_run(img, cxx, vo.y(), vo.y() + 5 * rh)))}")
th = tree.header()
thimg = save(th.grab().toImage(), "header_tree")
tw0 = th.sectionSize(0)
out(f"tree header size {th.width()}x{th.height()} section0 width {tw0}")
out(f"tree header column x={tw0 - 10}: {col_run(thimg, tw0 - 10, 0, th.height())}")
out(f"tree header row y={th.height() // 2} x={tw0 - 4}..{tw0 + 3}: {row_run(thimg, th.height() // 2, tw0 - 4, tw0 + 4)}")
tf = px(thimg, tw0 - 10, th.height() // 2)
out(f"tree header section fill (x={tw0 - 10}, y={th.height() // 2}) = {tf}; text glyphs {glyph_extreme(thimg, QRect(0, 1, tw0 - 2, th.height() - 3), tf)}")

# ---------------------------------------------------------------- 4. buttons and other sizes
out("==== 4 push button")
fm = QFontMetrics(app.font())
out("QFontMetrics(app font).height()", fm.height(), "ascent", fm.ascent(), "descent", fm.descent(), "lineSpacing", fm.lineSpacing())
dlg = QDialog()
lay = QHBoxLayout(dlg)
b_ok = QPushButton("OK")
b_long = QPushButton("A much longer label")
b_def = QPushButton("Default")
b_plain = QPushButton("Plain")
for b in (b_ok, b_long, b_def, b_plain):
    b.setAutoDefault(False)
    lay.addWidget(b)
b_def.setDefault(True)
dlg.resize(600, 80)
place(dlg, 100, 600)
out("dialog active", dlg.isActiveWindow(), "b_def.isDefault", b_def.isDefault())
for b, n in [(b_ok, "OK"), (b_long, "A much longer label")]:
    sh = b.sizeHint()
    out(f"QPushButton('{n}').sizeHint() = {sh.width()}x{sh.height()}; text width {fm.horizontalAdvance(n)}")
for b, n in [(b_plain, "plain"), (b_def, "default")]:
    img = save(b.grab().toImage(), f"button_{n}")
    w, h = img.width(), img.height()
    out(f"button {n}: size {w}x{h}; row y={h // 2} x=0..7: {row_run(img, h // 2, 0, 8)}; column x={w // 2} y=0..7: "
        f"{col_run(img, w // 2, 0, 8)}; bottom y={h - 8}..{h - 1}: {col_run(img, w // 2, h - 8, h)}")
    f = px(img, 8, h // 2)
    out(f"button {n}: fill sample (x=8, y={h // 2}) = {f}; text glyphs {glyph_extreme(img, QRect(8, 4, w - 16, h - 8), f)}")
le = QLineEdit()
cb = QComboBox()
cb.addItem("Item")
ch = QCheckBox("x")
for wdg, n in [(le, "QLineEdit()"), (cb, "QComboBox(one item)"), (ch, "QCheckBox('x')")]:
    s = wdg.sizeHint()
    out(f"{n}.sizeHint() = {s.width()}x{s.height()}")
out("PM_IndicatorWidth", style.pixelMetric(QStyle.PM_IndicatorWidth), "PM_IndicatorHeight", style.pixelMetric(QStyle.PM_IndicatorHeight))

# ---------------------------------------------------------------- 5. metrics
out("==== 5 metrics")
for n in ["PM_MessageBoxIconSize", "PM_ScrollBarExtent", "PM_ScrollBarSliderMin", "PM_MenuHMargin", "PM_MenuVMargin",
          "PM_SmallIconSize", "PM_ButtonMargin", "PM_DefaultFrameWidth", "PM_FocusFrameHMargin", "PM_MenuPanelWidth"]:
    out(f"{n}: no widget {style.pixelMetric(getattr(QStyle, n))}; QMenu {style.pixelMetric(getattr(QStyle, n), None, menu)}")
out("SH_ScrollBar_Transient", style.styleHint(QStyle.SH_ScrollBar_Transient), "with scroll bar widget",
    style.styleHint(QStyle.SH_ScrollBar_Transient, None, sb_l))

# ---------------------------------------------------------------- 6. fonts
out("==== 6 fonts")
f10 = QFont("Noto Sans", 10)
fmf = QFontMetricsF(f10)
fi = QFontInfo(f10)
out(f"QFont('Noto Sans',10): QFontInfo family {fi.family()} pointSizeF {fi.pointSizeF()} pixelSize {fi.pixelSize()}")
out(f"QFontMetricsF: height {fmf.height()} ascent {fmf.ascent()} descent {fmf.descent()} lineSpacing {fmf.lineSpacing()} leading {fmf.leading()}")
out("logicalDotsPerInch", scr.logicalDotsPerInch())
