import sys
from PySide6.QtWidgets import QApplication, QComboBox, QWidget, QVBoxLayout, QStyle, QStyleOptionComboBox
from PySide6.QtCore import QPoint, Qt, QObject, QEvent
from PySide6.QtGui import QCursor
from PySide6.QtTest import QTest
app = QApplication(sys.argv)
class Combo(QComboBox):
    # initStyleOption is protected: a subclass reads the state Qt hands the style
    def opt_state(self):
        o = QStyleOptionComboBox(); self.initStyleOption(o); s = o.state
        names = [n for n, f in (("Enabled", QStyle.StateFlag.State_Enabled), ("MouseOver", QStyle.StateFlag.State_MouseOver),
                 ("HasFocus", QStyle.StateFlag.State_HasFocus), ("On", QStyle.StateFlag.State_On), ("Sunken", QStyle.StateFlag.State_Sunken),
                 ("KeyboardFocusChange", QStyle.StateFlag.State_KeyboardFocusChange), ("Active", QStyle.StateFlag.State_Active)) if s & f]
        return "|".join(names)
w = QWidget(); lay = QVBoxLayout(w); c = Combo(); c.addItems(["Apple", "Banana", "Cherry", "Damson"]); lay.addWidget(c)
w.resize(300, 200); w.show(); app.processEvents()
g = c.geometry()
away = QPoint(g.center().x(), 2) if g.top() > 6 else QPoint(g.center().x(), w.height() - 3)
assert not g.contains(away)
WATCH = {QEvent.Type.MouseMove: "MouseMove", QEvent.Type.Enter: "Enter", QEvent.Type.Leave: "Leave",
         QEvent.Type.HoverEnter: "HoverEnter", QEvent.Type.HoverLeave: "HoverLeave", QEvent.Type.HoverMove: "HoverMove"}
class Log(QObject):
    on = False; seen = []
    def eventFilter(self, obj, ev):
        if self.on and ev.type() in WATCH:
            e = (WATCH[ev.type()], type(obj).__name__ + ("#" + obj.objectName() if obj.objectName() else "") + (" (the combo box)" if obj is c else ""))
            if not self.seen or self.seen[-1] != e: self.seen.append(e)
        return False
log = Log(); app.installEventFilter(log)
def watch(tag, fn):
    log.seen = []; log.on = True; fn(); app.processEvents(); QTest.qWait(300); log.on = False
    print(tag, "events", "; ".join(f"{t} -> {o}" for t, o in log.seen) or "none")
def report(tag):
    app.processEvents()
    img = c.grab().toImage()
    y = img.height() // 2
    pen = " ".join(img.pixelColor(x, y).name() for x in range(0, 8))
    fill = img.pixelColor(img.width() // 2, y).name()
    pop = QApplication.activePopupWidget()
    print(tag, "grab", img.width(), img.height(), "dpr", img.devicePixelRatio(), "left edge x=0..7 at y", y, ":", pen,
          "| fill at", img.width() // 2, y, ":", fill, "| underMouse", c.underMouse(), "hasFocus", c.hasFocus(),
          "option state", c.opt_state(), "| popup", type(pop).__name__ if pop else "none", "cursor", QCursor.pos().x(), QCursor.pos().y())
    img.save(f"combo_trigger_{tag}.png")
print("style", app.style().objectName(), "Window", app.palette().window().color().name(), "Highlight", app.palette().highlight().color().name(),
      "SH_ComboBox_Popup", app.style().styleHint(QStyle.StyleHint.SH_ComboBox_Popup, None, c),
      "combo geometry in window", g.x(), g.y(), g.width(), g.height(), "away point", away.x(), away.y())
watch("1_closed_away", lambda: QTest.mouseMove(w, away))
report("1_closed_away")
watch("2_closed_over", lambda: QTest.mouseMove(c, c.rect().center()))
report("2_closed_over")
watch("3_open_away_move", lambda: QTest.mouseMove(w, away))
watch("3_open_away_show", lambda: c.showPopup())
pop = QApplication.activePopupWidget()
pg = pop.geometry() if pop else None
print("popup", type(pop).__name__ if pop else "none", "global geometry", *((pg.x(), pg.y(), pg.width(), pg.height()) if pg else ()),
      "combo global", c.mapToGlobal(QPoint(0, 0)).x(), c.mapToGlobal(QPoint(0, 0)).y(), "away global", w.mapToGlobal(away).x(), w.mapToGlobal(away).y(),
      "away inside popup", pg.contains(w.mapToGlobal(away)) if pg else None,
      "trigger centre inside popup", pg.contains(c.mapToGlobal(c.rect().center())) if pg else None)
report("3_open_away")
watch("4a_open_move_to_combo", lambda: QTest.mouseMove(c, c.rect().center()))
report("4a_open_move_to_combo")
if pop and pop.isVisible():
    # 4a left the cursor at the trigger's centre: move it away first, through the popup, so the next move changes the position
    watch("4b_prep_popup_move_away", lambda: QTest.mouseMove(pop, pop.mapFromGlobal(w.mapToGlobal(away))))
    report("4b_prep_popup_move_away")
    watch("4b_open_move_to_popup", lambda: QTest.mouseMove(pop, pop.mapFromGlobal(c.mapToGlobal(c.rect().center()))))
    report("4b_open_move_to_popup")
else:
    print("4b_open_move_to_popup not run: popup visible", pop.isVisible() if pop else None)
