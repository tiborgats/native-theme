# Breeze and the font DPI, readings of 2026-11-14

The readings issue #4's second comment needs that the run of
[`2026-11-13/`](../2026-11-13/) did not save: a combo box popup's colours, Qt's scale on
an X server with `Xft.dpi` set, and the egui showcase's text there before and after
0.7.1 made the font DPI 96 on every Linux desktop. Taken with that run's scripts,
copied here as they ran (`combo.py` prints its text colour in hex), against the
`kdeglobals` files that run's README rebuilds. The images are not kept.

| | |
|---|---|
| Date | 2026-11-14 |
| native-theme | before: `387fc449`; after: `cbc54fb8` (the egui showcase built at each) |
| Versions | Breeze 6.8.2, plasma-integration 6.8.2 (the distribution's package query: `breeze 6.8.2-1.1`, `plasma-integration 6.8.2-1.1`); PySide6 and Qt as printed by `python3 -c "import PySide6, PySide6.QtCore as c; print(PySide6.__version__, c.qVersion())"`: 6.12.0, 6.12.0; Xvfb 21.1.26 (the package query, `xorg-server-xvfb 21.1.26-1.1`: Xvfb prints no version), xrdb 1.2.3, ImageMagick 7.1.2-35 |
| Environment | from a copy of this folder outside the repository. `combo.py` and `x11/qtfont.py` under `env -i` with `HOME`, `PATH` and only: `QT_QPA_PLATFORM` (`offscreen`, `xcb`), `QT_QPA_PLATFORMTHEME=kde`, `XDG_CONFIG_HOME`, `XDG_CONFIG_DIRS` (an empty folder), and for `qtfont.py` `DISPLAY=:97`; `x11/shot.sh` under `XDG_CURRENT_DESKTOP=KDE` with `WINIT_X11_SCALE_FACTOR` unset (it sets `XDG_SESSION_TYPE=x11`, `x11/cfg`, `x11/empty` and drops the Wayland variables itself). Xvfb on display `:97`, 1920 × 1080 × 24 |

- `combo_light.txt`, `combo_dark.txt`: `combo.py` under `cfg-light` / `cfg-dark`,
  `QT_QPA_PLATFORM=offscreen`, `QT_QPA_PLATFORMTHEME=kde`, `XDG_CONFIG_DIRS` an empty
  folder.
- `x11/qtfont-out.txt`: `x11/qtfont.py` at `Xft.dpi` 96, 120, 144 and 192.
- `x11/capheight-out.txt`: the showcase's capitals at `Xft.dpi` 96 and 144, before and
  after, each line with the build's commit and the box.

## Commands

Every reading was taken on 2026-11-14. The configuration (`cfg-light/`, `cfg-dark/`,
`x11/cfg/`, `empty/`, `x11/empty/`) was rebuilt as the
[`2026-11-13/` README](../2026-11-13/README.md#rebuilding-the-configuration) says, from
copies of `BreezeLight.colors` and `BreezeDark.colors` at breeze `f0b1d753` taken
earlier instead of the `curl` lines; `sha256sum` gave the two sums that README lists.

**1. The combo box popup.** `combo.py` is the `2026-11-13/` copy with its lines 19-20
replaced by four lines that print the darkest and the lightest pixel of each row's text
area as hex colours (lines 19-22 here).

```sh
mkdir -p empty
for s in light dark; do
  env -i HOME="$HOME" PATH="$PATH" QT_QPA_PLATFORM=offscreen QT_QPA_PLATFORMTHEME=kde \
    XDG_CONFIG_HOME="$PWD/cfg-$s" XDG_CONFIG_DIRS="$PWD/empty" python3 combo.py > "combo_$s.txt"
done
```

**2. Qt on X11.**

```sh
mkdir -p x11/empty
Xvfb :97 -screen 0 1920x1080x24 -nolisten tcp -noreset >/dev/null 2>&1 & XP=$!
sleep 1
for d in 96 120 144 192; do
  printf 'Xft.dpi: %s\n' "$d" | DISPLAY=:97 xrdb -nocpp -merge -
  echo "Xft.dpi $d"
  env -i HOME="$HOME" PATH="$PATH" DISPLAY=:97 QT_QPA_PLATFORM=xcb QT_QPA_PLATFORMTHEME=kde XDG_CONFIG_HOME="$PWD/x11/cfg" XDG_CONFIG_DIRS="$PWD/x11/empty" \
    python3 x11/qtfont.py
done > x11/qtfont-out.txt
kill $XP
```

**3. The egui showcase's capitals.** The example `showcase-egui`, built twice with

```sh
cargo build -p native-theme-egui --example showcase-egui --release --all-features
```

on 2026-11-14: before, from the sources at `387fc449`; after, at `cbc54fb8`, the commit
that makes the font DPI 96 on every Linux desktop (the tree also held the merge
`26393b7a`, which changes only `docs/`). Each `target/release/examples/showcase-egui` was
copied out of `target/` as `showcase-egui-before` / `showcase-egui-after`. Then, with
`$BIN` the folder that holds them:

```sh
for label in before after; do
  for dpi in 96 144; do
    env -u WINIT_X11_SCALE_FACTOR XDG_CURRENT_DESKTOP=KDE \
      x11/shot.sh "$BIN/showcase-egui-$label" "$dpi" "egui-$dpi-$label.png"
  done
done
```

and `python3 x11/capheight.py egui-<dpi>-<label>.png <x0> <x1> <y0> <y1>` on each image,
with the box `x11/capheight-out.txt` records above the line it printed. The label is
`Theme`, the side panel's label above the theme drop-down, set in egui's
`TextStyle::Body` (the body font); its one capital, T, has no descender or accent. `x0..x1`
covers the T alone; `y0..y1` covers the label's line with 2 px to spare above (the `h`
reaches one row above the T) and 3 px below, and no other text lies in the box. At
`Xft.dpi` 96 the two builds' screenshots are byte-identical, so the same box serves both.
