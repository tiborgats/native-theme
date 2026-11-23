# OS probe, run 45226132792 (2026-11-23)

The controls pass of `tools/os-probe` (`native-theme-os-probe --button-pass --controls`) on branch `m2-fs4c` at commit
`a9c0e2ee`: run 45208260830's pass with the text field and the split divider pressed (the first responder read after
each press), two scroll views (a vertical scroller set to the legacy style and one set to the overlay style, the knob
hovered and pressed, the styles read back) and the toolbar sheet (`toolbar_sheet`: the toolbar button in a window
whose content view lies under its transparent title bar, so its layer is solved over the backdrops), dispatched by
hand through `.github/workflows/os-probe.yml` (jobs `probe`, passes `controls`, images macOS 15 and 26):
<https://github.com/tiborgats/native-theme/actions/runs/45226132792>. The run is green: both probe jobs passed and the
iOS host job was skipped. GitHub deletes the run's artifacts after 90 days; this folder keeps them whole, as the run
uploaded them (PNGs byte for byte).

| Folder | Preset | Runner | Xcode | Rust |
|---|---|---|---|---|
| `os-probe-macos-15-controls/` | `macos-monterey` | macOS 15.8.2 (24H232), image 20261118.0550.1 | Xcode 16.4 | rustc 1.100.0 |
| `os-probe-macos-26-controls/` | `macos-tahoe` | macOS 26.7.2 (25G332), image 20261117.0516.1 | Xcode 26.6 | rustc 1.100.0 |

Both jobs captured: 410 PNGs each (82 solved layers), no capture `unavailable`.

Two things the pass did not do as meant, both fixed in the next commit and run:

- **The overlay scroller is a legacy one.** Both runners' `NSScroller` `preferredScrollerStyle` is
  `NSScrollerStyleLegacy`, and the scroll view set to the overlay style reads back `NSScrollerStyleLegacy` once the
  window is shown (`scroll_views` in `probe.toml`): its reads (`*.scroller_knob_overlay`) are a second legacy scroller's.
- **On macOS 26 the toolbar sheet's button is in the toolbar's overflow menu.** The 320 pt wide window shows the
  overflow chevron instead of the item, so the button lies in no window (its part, converted with no window to
  convert into, reads `[0, 97, 57, 23]`: the window's bottom-left corner), and its hover and press went to that empty
  corner; its `controls-toolbar_sheet.toml` reads are not of the button. macOS 15 shows the button.

## What is here

The layout of run 45208260830's folder (see its README): per image `button-pass-controls/probe.toml`, the captures and
the solved layers under `button-pass-controls/<appearance>/controls/` and, the toolbar sheet's,
`button-pass-controls/<appearance>/controls_toolbar/`, `environment.txt`, `environment/wallpaper.txt`;
`controls.toml` and `controls-toolbar_sheet.toml`, the output of `analysis/measure_controls.py` over the image folder.

`analysis/measure_controls.py`: run 45208260830's script with two additions (its docstring is its method): the pixels
around the text field's part (`around_<side>`: along the part's middle column or row, from 2 px inside its edge to 8 px
outside, which holds the focus ring of the pressed field), and `--sheet toolbar_sheet`, the same reads of the toolbar
sheet into `controls-toolbar_sheet.toml`. Run from the repository root:
`python3 docs/evidence/os-probe/2026-11-23-run-45226132792/analysis/measure_controls.py [--sheet toolbar_sheet] <run>/<image folder> [OUTPUT]`.
Over run 45217592058's macOS 26 folder it gives that run's `controls.toml` again, the text field's `around_*` added.
