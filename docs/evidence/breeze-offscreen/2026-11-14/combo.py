import sys
from PySide6.QtWidgets import QApplication, QComboBox, QWidget, QVBoxLayout
from PySide6.QtCore import QPoint, Qt
from PySide6.QtTest import QTest
app = QApplication(sys.argv)
w = QWidget(); lay = QVBoxLayout(w); c = QComboBox(); c.addItems(["Apple", "Banana", "Cherry", "Damson"]); lay.addWidget(c)
w.resize(300, 200); w.show(); app.processEvents()
c.showPopup(); app.processEvents()
v = c.view(); 
def px(img, x, y): return img.pixelColor(x, y).name()
def report(tag):
    app.processEvents()
    img = v.viewport().grab().toImage()
    for row in range(4):
        r = v.visualRect(v.model().index(row, 0))
        y = r.center().y()
        # background sample at far right inside the row, text sample: darkest/lightest pixel along the row
        bg = px(img, r.right() - 6, y)
        cols = [img.pixelColor(x, yy) for x in range(r.left()+4, r.left()+60) for yy in range(r.top()+3, r.bottom()-2)]
        darkest = min(cols, key=lambda c: c.lightness()).name()
        lightest = max(cols, key=lambda c: c.lightness()).name()
        print(tag, "row", row, "rect", r.x(), r.y(), r.width(), r.height(), "fill", bg, "text darkest", darkest, "lightest", lightest, "current" if v.currentIndex().row()==row else "")
    img.save(f"combo_{tag}.png")
print("style", app.style().objectName(), "Window", app.palette().window().color().name(), "popup view class", type(v).__name__, "SH_ComboBox_Popup", app.style().styleHint(app.style().StyleHint.SH_ComboBox_Popup, None, c))
report("opened")
r2 = v.visualRect(v.model().index(2, 0))
QTest.mouseMove(v.viewport(), r2.center()); app.processEvents(); QTest.qWait(300)
report("hover_row2")
