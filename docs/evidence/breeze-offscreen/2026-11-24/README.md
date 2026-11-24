# Breeze rendered offscreen, 2026-11-24

What the installed Breeze style painted, and what Qt reported, for a disabled text-only
button in a main window's top tool bar (M2.FS4b c5i1, `toolbar.button_label(DISABLED)`).
Run by hand on a Linux desktop on 2026-11-24; nothing was drawn on the desktop itself.

| | |
|---|---|
| Packages (`pacman -Q`) | breeze 6.8.3-1.1, qt6-base 6.12.0-5.1, pyside6 6.12.0-3.1, plasma-integration 6.8.3-1.1; Python 3.14.10 |
| Style | the installed `breeze` widget style (each output's first line) |
| Qt | platform `offscreen`, `QT_SCALE_FACTOR=1`, logical DPI 96, dpr 1 |
| Platform theme | Plasma's, `QT_QPA_PLATFORMTHEME=kde` |
| Colours and fonts | `cfg-light/kdeglobals` and `cfg-dark/kdeglobals`, rebuilt as `../2026-11-13/README.md` says (the same sha256) |
| Breeze's own settings | defaults: no `breezerc`, no `kwinrc`, `XDG_CONFIG_DIRS` names an empty folder, `HOME` a private empty folder |

`toolbar_disabled.py` → `toolbar_disabled_<scheme>.txt`: a `QMainWindow` with one top
`QToolBar` of three text-only actions (New, Edit, Save), Edit disabled. It prints the
palettes Qt reports for the main window, the tool bar and each tool button (Window,
WindowText and ButtonText, Active and Disabled) and, for each button, the fill at its
corner and its label's ink read back from `QWidget.grab()` (`ink` of
`../2026-11-20/common.py`: the pixel farthest from the fill and how many pixels have
exactly that colour).

To repeat it: copy `toolbar_disabled.py` beside `../2026-11-20/common.py` and
`../2026-11-20/run.sh` outside the repository, rebuild the configuration there, and run
`./run.sh toolbar_disabled.py light` and `… dark`.

## What the run reads

- The tool bar carries Breeze's tools-area palette (`breeze_has_toolsarea_palette`
  true): Window #dee0e2 / #292c30, the Header set's, and Disabled WindowText #95979a /
  #6d6f72, where the main window's Disabled WindowText is #a0a1a3 / #686a6c.
- The disabled button's palette is in its Disabled group, and its label is drawn in
  that WindowText: ink #95979a / #6d6f72, 19 pixels of exactly that colour, in both
  schemes. The enabled buttons' labels read #232629 / #fcfcfc, the Active WindowText.
