# OS probe, run 45542850219 (2026-11-25)

The controls pass of `tools/os-probe` and the switch-label job on branch `m2-fs4e`, commit `1b9affae` (M2.FS4c10: the
controls pass's new toolbar items sheet; the switch-label host's `notes` and `failures` written before its first
table), images macOS 15 and macOS 26 (`environment.txt` the same as run 45537062519's). Dispatched by hand through
`.github/workflows/os-probe.yml` (jobs `probe,switch-label`, passes `controls`, images `["macos-15", "macos-26"]`):
<https://github.com/tiborgats/native-theme/actions/runs/45542850219>, green on attempt 1.

Only the controls jobs' `probe.toml`, `settings.txt` and environment records are kept; `CAPTURES.sha256` names every
PNG of the two controls artifacts. The switch-label jobs' `switch-label.toml` were read (not kept): on both images
`notes` and `failures` parse as top-level keys (`[]` and `[]`), no longer as keys of the last `[[release]]`.

## What is here

- `analysis/measure_controls.py`: run 45456990497's, `--sheet toolbar_items_sheet` allowed; its output
  `os-probe-macos-{15,26}-controls/controls-toolbar_items_sheet.toml`.
- `analysis/check_grey_in_part.py`: run 45252258079's for any sheet; `check_grey_in_part-macos-{15,26}-toolbar_items_sheet-<target>.txt`
  (for the unselected and disabled items it stops at the first state the target lacks, so those files hold the aqua
  lines only).

## The toolbar items sheet

Three image items (the symbol image, labels "Selected", "Unselected", "Disabled") in the toolbar of the toolbar sheet's
window (content view under the transparent title bar), every item selectable, the first selected
(`selectedItemIdentifier` reads `selected` when shown, per appearance and after each press), the last disabled; the
display mode reads 1 (icon and label) and `toolbarStyle` 0 (automatic) on both images; `isBordered` false on every
item. The targets are AppKit's `NSToolbarItemViewer`s, found on both images (macOS 15 52 pt tall, macOS 26 66 pt), each
`over_backdrop`. No `NSTextField` lies below any item view (the `labels.<item>` lists are empty), so the label colours
are read from the layers only.

- **The selected item draws a fill and tints its icon and label with the accent.** macOS 15: fill #0000000d /
  #ffffff0d (layer), icon and label #0070f5f9 / #0d87fff9; macOS 26: fill #dcdcdc16 / #1e1e1ea2, icon and label
  #0070f5ff / #0d87fffe. Neither fill is one fixed layer over the backdrops: composited over the grey backdrop the
  black-and-white solve misses the grey capture in 2592 / 2579 of the part's pixels by up to 7.0 / 6.8 levels on macOS
  15, in 3945 / 3687 by up to 76.1 / 14.5 on macOS 26; over the window's own background it matches (0 px / 45 px over 2
  on macOS 15, 0 / 3415 px, max 3.9, on macOS 26). So macOS 15's fill is a close approximation and macOS 26's is the
  look over this window's background only.
- **Hovered and pressed, the selected item draws nothing different** (0 pixels differ from rest, both images, both
  appearances; the press held, `key_at_capture` true).
- **An unselected item hovered:** macOS 15 draws the selected item's fill, #0000000d / #ffffff0d (3236 pixels differ,
  by at most 13; over grey off by up to 7.0); macOS 26 draws nothing (0 pixels differ).
- **The disabled item:** among its pixels #00000040 / #ffffff40 are the commonest glyph-like ones on both images
  (38 / 59 pixels; beside #ffffff3f, #ffffff1a and, on macOS 26, #000000ff), where the enabled unselected item's
  `ink` is #0000008c / #ffffff8c; the script's `ink` key reads #ffffff3f / #ffffff1a
  on macOS 15 and #000000ff on macOS 26 (67 px, both appearances), which are not the glyph colour: so the disabled
  label and icon colour is read here only as those commonest pixels, not as a fitted label read.

## The fitted reads (M2.FS4c11)

`analysis/fit_toolbar_items.py` (its docstring gives the method) reads the same artifacts again (downloaded from the
run; the PNGs `CAPTURES.sha256` names), writing `os-probe-macos-{15,26}-controls/toolbar_items_fitted.toml`:

- **The disabled item's label** (the lowest run of ink rows in the capture over the window, the label "Disabled"):
  #00000040 / #ffffff40 on macOS 15 and macOS 26, from every backdrop pair alike (black and white, black and grey,
  grey and white), each layer composited over the third backdrop within 1.1 levels and over the window's background
  within 0.6; so it is one fixed layer. (The commonest-pixel read above, #00000040 / #ffffff40, is the same value.)
  The enabled unselected item's label read the same way: #0000008c / #ffffff80 (macOS 15), #0000008d / #fefefe81
  (macOS 26), within 1.3 levels; the `ink` key's #ffffff8c in darkAqua is not the label's.
- **macOS 26, the selected item:** a rounded rect of the item view's width, 65 x 58 px (from 4 px below the view's
  top), corner radius 7 px (`edge_radius`: [6.75, 7.0] and [7.25, 8.0] at the four corners, rms 0), a lighter top rim
  and a soft shadow outside it. Its interior, the commonest colour of the rect inset by 3 px (81-83 % of it), is not
  one layer: over black #131313 in both appearances, over grey #c3c3c3 / #4d4d4d, over white #fcfcfc / #707070, over
  the window's own background #fcfcfc (over #ffffff) / #1a1a1a (over #1b1b1b); the rim's outermost pixel there
  #ffffff (2 px) / #313131 (1 px). The black-and-white solve (#dcdcdc16 / #1e1e1ea2) misses the interior over grey by 59 / 11 levels,
  so the composite over the window's background is the only read that holds where the toolbar lies.
