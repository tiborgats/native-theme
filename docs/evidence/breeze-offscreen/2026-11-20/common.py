"""Shared helpers for menus.py, menubar.py and views.py (2026-11-20 offscreen run).

Every script takes <light|dark> [proxy]. With "proxy" the application style is a
QProxyStyle wrapping "breeze" that records the QStyle::State flags Breeze receives;
without it the plain breeze style paints and nothing is logged.
"""
import configparser
import os
import sys

from PySide6.QtCore import QEventLoop, QRect, QTimer
from PySide6.QtGui import QColor
from PySide6.QtWidgets import QApplication, QProxyStyle, QStyle, QStyleFactory

SCHEME = sys.argv[1]
PROXY = len(sys.argv) > 2 and sys.argv[2] == "proxy"
HERE = os.path.dirname(os.path.abspath(__file__))

STATE_NAMES = ["State_Enabled", "State_Raised", "State_Sunken", "State_Off", "State_NoChange", "State_On",
               "State_DownArrow", "State_Horizontal", "State_HasFocus", "State_Top", "State_Bottom",
               "State_FocusAtBorder", "State_AutoRaise", "State_MouseOver", "State_UpArrow", "State_Selected",
               "State_Active", "State_Window", "State_Open", "State_Children", "State_Item", "State_Sibling",
               "State_Editing", "State_KeyboardFocusChange", "State_ReadOnly", "State_Small", "State_Mini"]


def state_str(state):
    names = []
    for n in STATE_NAMES:
        f = getattr(QStyle.StateFlag, n)
        if f.value and (state & f) == f:
            names.append(n[6:])
    try:
        v = state.value
    except AttributeError:
        v = int(state)
    return f"0x{v:08x} " + "|".join(names)


LOG = []


class LogStyle(QProxyStyle):
    """Records the state of the controls the scripts ask about, then lets breeze paint."""

    def drawControl(self, element, option, painter, widget=None):
        if element in (QStyle.ControlElement.CE_MenuBarItem, QStyle.ControlElement.CE_MenuItem):
            text = getattr(option, "text", "?")
            LOG.append((element.name, text, state_str(option.state), QRect(option.rect), type(option).__name__,
                        type(widget).__name__ if widget is not None else None))
        return super().drawControl(element, option, painter, widget)

    def drawPrimitive(self, element, option, painter, widget=None):
        if element == QStyle.PrimitiveElement.PE_PanelItemViewItem:
            idx = getattr(option, "index", None)
            row = idx.row() if idx is not None and idx.isValid() else None
            LOG.append((element.name, row, state_str(option.state), QRect(option.rect), type(option).__name__,
                        type(widget).__name__ if widget is not None else None))
        return super().drawPrimitive(element, option, painter, widget)


def make_app():
    app = QApplication(sys.argv[:1])
    if PROXY:
        base = QStyleFactory.create("breeze")
        app.setStyle(LogStyle(base))
    return app


def out(*a):
    print(f"[{SCHEME}{' proxy' if PROXY else ''}]", *a, flush=True)


def header(app):
    st = app.style()
    base = st.baseStyle() if PROXY else st
    out("platform", app.platformName(), "| style", st.metaObject().className(),
        "| base style", base.objectName(), base.metaObject().className())
    scr = app.primaryScreen()
    out("screen logicalDPI", scr.logicalDotsPerInch(), "dpr", scr.devicePixelRatio(),
        "geometry", scr.geometry().width(), "x", scr.geometry().height())


def print_log(filter_fn=None, tag=""):
    if not PROXY:
        return
    seen = 0
    for e in LOG:
        if filter_fn is None or filter_fn(e):
            r = e[3]
            out(f"{tag}state log: {e[0]} item={e[1]!r} state={e[2]} rect=({r.x()},{r.y()},{r.width()}x{r.height()})"
                f" option={e[4]} widget={e[5]}")
            seen += 1
    if seen == 0:
        out(f"{tag}state log: no matching draw call recorded")


def clear_log():
    LOG.clear()


def wait(ms):
    loop = QEventLoop()
    QTimer.singleShot(ms, loop.quit)
    loop.exec()


def hexc(c):
    return c.name(QColor.NameFormat.HexArgb) if c.alpha() < 255 else c.name()


def px(img, x, y):
    if 0 <= x < img.width() and 0 <= y < img.height():
        return hexc(img.pixelColor(x, y))
    return "outside"


def row_run(img, y, x0, x1):
    return [px(img, x, y) for x in range(x0, x1)]


def col_run(img, x, y0, y1):
    return [px(img, x, y) for y in range(y0, y1)]


def patch(img, x0, y0, w=6, h=6):
    return " / ".join(" ".join(px(img, x, y) for x in range(x0, x0 + w)) for y in range(y0, y0 + h))


def diff(a, b):
    return max(abs(a.red() - b.red()), abs(a.green() - b.green()), abs(a.blue() - b.blue()))


def ink(img, rect, bg, thresh=40):
    """Pixels differing from bg by more than thresh inside rect: bbox, the pixel farthest
    from bg (with how many pixels have exactly that colour) and the column runs."""
    bgc = QColor(bg)
    pts = []
    far = None
    for y in range(rect.top(), rect.bottom() + 1):
        for x in range(rect.left(), rect.right() + 1):
            if not (0 <= x < img.width() and 0 <= y < img.height()):
                continue
            c = img.pixelColor(x, y)
            d = diff(c, bgc)
            if d > thresh:
                pts.append((x, y, hexc(c)))
                if far is None or d > far[0]:
                    far = (d, hexc(c), x, y)
    if not pts:
        return None
    xs = [p[0] for p in pts]
    ys = [p[1] for p in pts]
    count = sum(1 for p in pts if p[2] == far[1])
    return dict(bbox=(min(xs), min(ys), max(xs), max(ys)), farthest=far[1], at=(far[2], far[3]),
                same_count=count, ink_pixels=len(pts), cols=sorted(set(xs)))


def runs(cols, gap=8):
    """Split sorted ink columns into runs separated by more than gap blank columns."""
    out_runs = []
    for c in cols:
        if out_runs and c - out_runs[-1][1] <= gap + 1:
            out_runs[-1][1] = c
        else:
            out_runs.append([c, c])
    return out_runs


def cfg_colours():
    """The kdeglobals the run reads (XDG_CONFIG_HOME), as configured text, not a measurement."""
    cp = configparser.ConfigParser(interpolation=None, strict=False)
    cp.optionxform = str
    cp.read(os.path.join(os.environ["XDG_CONFIG_HOME"], "kdeglobals"))
    return cp


def rgb(s):
    r, g, b = (int(v) for v in s.split(",")[:3])
    return QColor(r, g, b)


def mix(a, b, bias):
    """KColorUtils::mix for opaque colours: linear per channel (computed, not measured)."""
    return QColor(round(a.red() + (b.red() - a.red()) * bias), round(a.green() + (b.green() - a.green()) * bias),
                  round(a.blue() + (b.blue() - a.blue()) * bias)).name()


def over(fg, alpha, bg):
    """fg at alpha painted over opaque bg (computed, not measured)."""
    return QColor(round(alpha * fg.red() + (1 - alpha) * bg.red()), round(alpha * fg.green() + (1 - alpha) * bg.green()),
                  round(alpha * fg.blue() + (1 - alpha) * bg.blue())).name()


def corners(img, g, tag):
    """The four corner pixels of rect g, two diagonal steps inward from each, and a 6x6
    patch at each corner that starts one pixel outside the rect."""
    L, T, R, B = g.left(), g.top(), g.right(), g.bottom()
    out(f"{tag}: corner TL ({L},{T}) {px(img, L, T)} diag+1 {px(img, L + 1, T + 1)} diag+2 {px(img, L + 2, T + 2)} diag+3 {px(img, L + 3, T + 3)}")
    out(f"{tag}: corner TR ({R},{T}) {px(img, R, T)} diag+1 {px(img, R - 1, T + 1)} diag+2 {px(img, R - 2, T + 2)} diag+3 {px(img, R - 3, T + 3)}")
    out(f"{tag}: corner BL ({L},{B}) {px(img, L, B)} diag+1 {px(img, L + 1, B - 1)} diag+2 {px(img, L + 2, B - 2)} diag+3 {px(img, L + 3, B - 3)}")
    out(f"{tag}: corner BR ({R},{B}) {px(img, R, B)} diag+1 {px(img, R - 1, B - 1)} diag+2 {px(img, R - 2, B - 2)} diag+3 {px(img, R - 3, B - 3)}")
    out(f"{tag}: patch TL x={L - 1}..{L + 4} y={T - 1}..{T + 4} (rows top to bottom): {patch(img, L - 1, T - 1)}")
    out(f"{tag}: patch TR x={R - 4}..{R + 1} y={T - 1}..{T + 4}: {patch(img, R - 4, T - 1)}")
    out(f"{tag}: patch BL x={L - 1}..{L + 4} y={B - 4}..{B + 1}: {patch(img, L - 1, B - 4)}")
    out(f"{tag}: patch BR x={R - 4}..{R + 1} y={B - 4}..{B + 1}: {patch(img, R - 4, B - 4)}")


def qrect_str(r):
    return f"x={r.x()} y={r.y()} w={r.width()} h={r.height()}"


