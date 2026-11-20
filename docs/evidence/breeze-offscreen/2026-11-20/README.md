# Breeze rendered offscreen, 2026-11-20

What the installed Breeze style painted, and what Qt reported, for issue #4's leftovers
([rationale](../../../issues/0004_kde-leftovers-rationale.md)): popup-menu items
(hovered, pressed), menu-bar titles (hovered, menu open), the menu bar and tool bar at
rest, a hovered item-view row with and without the view's focus, a slider handle and a
checked flat tool button, and the pixel size Qt Quick gives a point-sized font. Run by
hand on a Linux desktop on 2026-11-20; nothing was drawn on the desktop itself. The
images the scripts write are not kept.

| | |
|---|---|
| Packages (`pacman -Q`) | breeze 6.8.2-1.1, qt6-base 6.12.0-5.1, qt6-declarative 6.12.0, pyside6 6.12.0-3.1, plasma-integration 6.8.2-1.1, kirigami and qqc2-desktop-style 6.31.0; Python 3.14.9 |
| Style | the installed `breeze` widget style; `org.kde.desktop` for Qt Quick (each output's first lines) |
| Qt | platform `offscreen`, `QT_SCALE_FACTOR=1`, logical DPI 96, dpr 1 |
| Platform theme | Plasma's, `QT_QPA_PLATFORMTHEME=kde` |
| Colours and fonts | `cfg-light/kdeglobals` and `cfg-dark/kdeglobals`, rebuilt as `../2026-11-13/README.md` says (the same sha256) |
| Breeze's own settings | defaults: no `breezerc`, no `kwinrc`, `XDG_CONFIG_DIRS` names an empty folder `empty/`, `HOME` a private empty folder `home/` |

`run.sh <script> <light|dark> [proxy]` sets that environment (with `cfg-light/`,
`cfg-dark/`, `empty/` and `home/` beside it) and runs the script under
`nice -n 19 ionice -c 3`. Lines marked `computed (not measured)` are arithmetic on the
palette or on `kdeglobals`' colours, not readings.

## Menus, the menu bar, item views

- `common.py`: shared helpers. With the `proxy` argument the application style is a
  `QProxyStyle` wrapping Breeze that logs the `QStyle::State` flags of every
  `CE_MenuItem`, `CE_MenuBarItem` and `PE_PanelItemViewItem` draw call (`state log:`
  lines), then lets Breeze paint.
- `menus.py` → `menus_<scheme>.txt`: a text-only `QMenu` (Hide, Edit with shortcut
  Ctrl+E, Mark) at rest, with the pointer on Edit, and with the left button held on Edit
  (grabbed before the release); then a menu with a 16 px `#ff0000` icon on Print and no
  checkable items, at rest and hovered. Per state: the fill, each edge's row and column,
  the corner pixels and three diagonal steps inward, a 6 × 6 patch at each corner, and
  the ink of the text and the shortcut.
- `menubar.py` → `menubar_<scheme>.txt`: a `QMainWindow` with a `QMenuBar` (File, Edit,
  View), one `QToolBar` and an empty central widget: the column through the menu bar and
  tool bar at rest, the bar's fill, the title rectangles and their text ink, File
  hovered, File clicked so its menu is open, and the pointer then moved onto Edit.
- `views.py` → `views_<scheme>.txt`: an active window with a `QLineEdit` above a
  `QListWidget` and a `QTreeWidget`; the pointer on row 3 (unselected) of each view, once
  with the line edit focused and once with the view focused.
- `*_<scheme>_proxy.txt`: the same with the logging proxy. Apart from the `state log:`
  lines, the style line and the object addresses Python prints for the open popup, each
  equals its plain output line for line, so the proxy does not change what Breeze paints.

On the offscreen platform a move made by `QTest.mouseMove` or `QCursor.setPos` reaches the
widget as synthesized by Qt, and `QMenuBar` ignores a synthesized move without a button;
`menubar.py` therefore sends each menu-bar move again as an unsynthesized `QMouseEvent`
(`real_move`), and prints both steps. Popup menus and item views take `QTest.mouseMove`
as it is.

To repeat, from this folder:

```sh
for s in light dark; do
  for p in menus menubar views; do
    ./run.sh $p.py $s > ${p}_$s.txt
    ./run.sh $p.py $s proxy > ${p}_${s}_proxy.txt
  done
done
```

## Fonts in Qt Quick, a slider handle, a checked tool button (`qt-fonts-widgets/`)

Run in the same environment with the light configuration (and the dark one for
`widgets.py`, whose output has a `## light` and a `## dark` section):

- `probe.py probe.qml` → `out_light.txt`: under `org.kde.desktop`, a plain `Text` at
  10 pt, a QQC2 `Label`, a `Text` in `Kirigami.Theme.defaultFont` and a QQC2 `Button`
  "OK": each font's `QFontInfo` pixel size and point size, the item's implicit size, and
  the screen's logical DPI.
- `probe2.py probe2.qml` → `out2_light.txt`: Kirigami `Heading` levels 1 and 2 and
  `Kirigami.Theme.smallFont`, the same readings.
- `widgets.py` → `out_widgets.txt`: a `QSlider` handle without the focus at rest and
  with the pointer on it (the handle's rectangle, a row and a column of pixels through
  its edge, its centre), and a checked `autoRaise` `QToolButton` (a row and a column
  through its frame, its fill).
