# Breeze rendered offscreen, 2026-11-13

What the installed Breeze widget style painted when Qt drew real widgets offscreen,
read back pixel by pixel. Recorded for issue #4's second comment
([rationale](../../../issues/0004_kde-hand-overrides-rationale.md) §2, §9); the KDE
cells of `docs/platform-facts.md` cite the outputs by line. Run by hand on a Linux
desktop on 2026-11-13; nothing was drawn on the desktop itself. The images the scripts
write are not kept.

| | |
|---|---|
| Style | the installed Breeze 6.8.2 `breeze` widget style (`out_light.txt:1` records the style's name only) |
| Qt | Qt 6.12 through PySide6 6.12.0, platform `offscreen`, scale 1, logical DPI 96 (`out_light.txt:4`) |
| Platform theme | Plasma's, `QT_QPA_PLATFORMTHEME=kde` |
| Colours and fonts | `cfg-light/kdeglobals` and `cfg-dark/kdeglobals`, rebuilt below: Breeze's colour schemes at breeze `f0b1d753` (the same at `v6.8.2` but for translated names) with Noto Sans fonts and `widgetStyle=breeze` appended |
| Breeze's own settings | their defaults: no `breezerc`, and `XDG_CONFIG_DIRS` names an empty folder, so no system file is read |

No output records the Breeze, Qt and PySide versions: they are the installed packages'
as the rationale (§2) records them. The plasma-integration version was not recorded.

- `measure.py` wrote `out_light.txt` and `out_dark.txt`: scroll bars (a list and a plain
  scroll area, with and without the focus, at rest and hovered), a popup menu with an
  icon, a check box item, a separator and a submenu, a text-only menu, item view rows
  (hovered, selected, selected without the focus, selected and hovered), a table with
  its grid and header, a tree, push buttons, pixel metrics and font metrics. Each line
  names what it samples and where; a `saved <name>.png` line names an image that is not
  kept. The two files have the same lines in the same order.
- `tree_check.py` wrote `tree_light.txt` and `tree_dark.txt`: what a tree view paints
  between and around its rows. The 1 px lines in the dark tree with the focus
  (`out_dark.txt:143`, `tree_dark.txt:3`) are the current item's frame; without the
  focus there is no line between rows (`tree_dark.txt:5`).
- `combo.py` (a combo box's open popup, at rest and with the pointer on its third row)
  and the scripts in `x11/` (Qt's scale and a 10 pt font's size on a private X server
  with `Xft.dpi` set; the egui showcase's text there) ran too, but their printed output
  was not saved. The readings cited from them are in the later run directory beside
  this one, taken with the same scripts (`combo.py` with its text colour printed in hex).

## Rebuilding the configuration

The runs read a private `kdeglobals`: Breeze's colour scheme at the pinned commit with
these ten lines appended (a blank line, then):

```ini
[General]
font=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
menuFont=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
toolBarFont=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
smallestReadableFont=Noto Sans,8,-1,5,400,0,0,0,0,0,0,0,0,0,0,1

[KDE]
widgetStyle=breeze
```

followed by one blank line. `x11/cfg/kdeglobals` is the dark one. To rebuild all three:

```sh
B=https://raw.githubusercontent.com/KDE/breeze/f0b1d7534aa2356d7336241d0c7051522e8a6b68/colors
mkdir -p cfg-light cfg-dark x11/cfg empty x11/empty
curl -fsSL "$B/BreezeLight.colors" > cfg-light/kdeglobals
curl -fsSL "$B/BreezeDark.colors" > cfg-dark/kdeglobals
for f in cfg-light/kdeglobals cfg-dark/kdeglobals; do
  printf '\n[General]\nfont=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\nmenuFont=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\ntoolBarFont=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\nsmallestReadableFont=Noto Sans,8,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n\n[KDE]\nwidgetStyle=breeze\n\n' >> "$f"
done
cp cfg-dark/kdeglobals x11/cfg/kdeglobals
sha256sum cfg-light/kdeglobals cfg-dark/kdeglobals
# af054447132ed59055ed2cf20acc058c2c6d64e8d6095411085f786a9d6534d7  cfg-light/kdeglobals
# c7e850c6ab7f5f9e2c2f5eea024cff9b3db155c094914c18f9128013e3951de5  cfg-dark/kdeglobals
```

## Repeating the run

From a copy of this folder outside the repository (the scripts write their images
beside themselves or into the working folder), after rebuilding the configuration:

```sh
for s in light dark; do
  export XDG_CONFIG_HOME="$PWD/cfg-$s" XDG_CONFIG_DIRS="$PWD/empty"
  export QT_QPA_PLATFORM=offscreen QT_QPA_PLATFORMTHEME=kde
  python3 measure.py "$s" > "out_$s.txt"
  python3 tree_check.py "$s" > "tree_$s.txt"
done
```

`x11/shot.sh <showcase binary> <Xft.dpi> <out.png>` starts Xvfb on display `:97` (one
1920 × 1080 × 24 screen, `-nolisten tcp`), sets `Xft.dpi` with `xrdb -nocpp -merge`,
runs the showcase with `--theme kde-breeze --variant dark --capture`,
`XDG_SESSION_TYPE=x11`, `x11/cfg` and `x11/empty` and no Wayland variables for 20 s,
and takes a screenshot with ImageMagick's `import`; it writes the resource it loads to
`x11/res`. `x11/capheight.py <png> <x0> <x1> <y0> <y1>` prints the height of the bright
rows in that box. `x11/qtfont.py` prints Qt's device pixel ratio, logical DPI and the
pixel size of a 10 pt Noto Sans.
