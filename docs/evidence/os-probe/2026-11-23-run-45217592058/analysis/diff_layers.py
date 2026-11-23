"""The pixels of a region that differ between two solved layers (straight RGBA PNGs of one image folder).

Prints the count of pixels in the region (x, y, w, h in the layer's pixels) whose straight colour or alpha differs by
more than 2 of 255 in a channel, their bounds, and with COLUMNS given, per row of those bounds the first COLUMNS pixels
as old>new #rrggbbaa.

Usage: python3 -I diff_layers.py OLD-layer.png NEW-layer.png x,y,w,h [COLUMNS]
"""
import sys

from PIL import Image


def main():
    a = Image.open(sys.argv[1]).convert("RGBA")
    b = Image.open(sys.argv[2]).convert("RGBA")
    x0, y0, w, h = (int(v) for v in sys.argv[3].split(","))
    pa, pb = a.load(), b.load()
    count, bounds = 0, None
    for y in range(y0, y0 + h):
        for x in range(x0, x0 + w):
            if max(abs(i - j) for i, j in zip(pa[x, y], pb[x, y])) > 2:
                count += 1
                bounds = (x, y, x, y) if bounds is None else (
                    min(bounds[0], x), min(bounds[1], y), max(bounds[2], x), max(bounds[3], y))
    print(f"region [{x0}, {y0}, {w}, {h}]: {count} pixels differ, bounds (x0, y0, x1, y1) {bounds}")
    if bounds is not None and len(sys.argv) > 4:
        columns = int(sys.argv[4])
        for y in range(bounds[1], bounds[3] + 1):
            cells = ("%02x%02x%02x%02x>%02x%02x%02x%02x" % (pa[x, y] + pb[x, y])
                     for x in range(bounds[0], min(bounds[2] + 1, bounds[0] + columns)))
            print(y, " ".join(cells))


if __name__ == "__main__":
    main()
