"""Follow-up: what the tree view paints between/around rows (branch lines, focus outline)."""
import os, sys
from PySide6.QtCore import QEventLoop, QPoint, QTimer
from PySide6.QtWidgets import QApplication, QTreeWidget, QTreeWidgetItem, QListWidget, QStyle
from PySide6.QtGui import QPalette
S = sys.argv[1]; OUT = os.path.dirname(os.path.abspath(__file__))
app = QApplication(sys.argv[:1])
def wait(ms):
    l = QEventLoop(); QTimer.singleShot(ms, l.quit); l.exec()
def runs(img, y, x0, x1):
    r = []; p = None
    for x in range(x0, x1):
        c = img.pixelColor(x, y).name()
        if c != p: r.append((x, c)); p = c
    return r
def cruns(img, x, y0, y1):
    r = []; p = None
    for y in range(y0, y1):
        c = img.pixelColor(x, y).name()
        if c != p: r.append((y, c)); p = c
    return r
t = QTreeWidget(); t.setColumnCount(2); t.setHeaderLabels(["Name", "Value"])
for i in range(6): t.addTopLevelItem(QTreeWidgetItem([f"Node {i}", f"{i}"]))
t.resize(300, 220); t.move(500, 300); t.show(); t.activateWindow(); wait(400)
vo = t.viewport().mapTo(t, QPoint(0, 0))
r0 = t.visualItemRect(t.topLevelItem(0))
print(f"[{S}] tree hasFocus {t.hasFocus()} currentIndex row {t.currentIndex().row()} col {t.currentIndex().column()}"
      f" selected {len(t.selectedItems())}; viewport at {vo.x()},{vo.y()}; row0 rect {r0.x()},{r0.y()},{r0.width()}x{r0.height()};"
      f" indentation {t.indentation()} rootIsDecorated {t.rootIsDecorated()}")
img = t.grab().toImage(); img.save(f"{OUT}/tree_focused_{S}.png")
cy = vo.y() + r0.center().y()
print(f"[{S}] focused: row0 centre y={cy} runs x=0..30: {runs(img, cy, 0, 30)}")
print(f"[{S}] focused: column x={vo.x() + 60} runs: {cruns(img, vo.x() + 60, vo.y(), vo.y() + 3 * r0.height())}")
print(f"[{S}] focused: column x={vo.x() + 9 - vo.x() + vo.x()} (branch line candidate) runs: {cruns(img, 9, vo.y(), vo.y() + 6 * r0.height() + 4)}")
t.clearFocus(); wait(400)
img = t.grab().toImage(); img.save(f"{OUT}/tree_unfocused_{S}.png")
print(f"[{S}] after clearFocus hasFocus {t.hasFocus()}: column x={vo.x() + 60} runs: {cruns(img, vo.x() + 60, vo.y(), vo.y() + 3 * r0.height())}")
print(f"[{S}] unfocused: row y={vo.y() + 1} runs: {runs(img, vo.y() + 1, 0, t.width())}")
# expected values from the source formulas, for comparison
p = app.palette()
def mix(a, b, f):
    return '#%02x%02x%02x' % tuple(round(x + (y - x) * f) for x, y in [(a.red(), b.red()), (a.green(), b.green()), (a.blue(), b.blue())])
print(f"[{S}] mix(Base, Text, 0.25) = {mix(p.color(QPalette.Base), p.color(QPalette.Text), 0.25)}; Highlight.lighter(110) = {p.color(QPalette.Highlight).lighter(110).name()}")
