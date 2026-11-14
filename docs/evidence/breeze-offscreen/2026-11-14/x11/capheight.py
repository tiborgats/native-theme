import sys
from PySide6.QtGui import QImage
img = QImage(sys.argv[1]); x0,x1,y0,y1 = map(int, sys.argv[2:6])
rows = []
for y in range(y0, y1):
    lum = max(img.pixelColor(x, y).lightness() for x in range(x0, x1))
    if lum > 160: rows.append(y)
print(sys.argv[1].split('/')[-1], "bright rows", rows[0] if rows else None, "..", rows[-1] if rows else None, "height", (rows[-1]-rows[0]+1) if rows else 0)
