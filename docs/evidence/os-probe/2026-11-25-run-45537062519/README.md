# OS probe, run 45537062519 (2026-11-25)

The button and controls passes of `tools/os-probe` on branch `m2-fs4e`, commit `8fb4e706` (M2.FS4c9), images macOS 15
and macOS 26 (26.7.2, build 25G332; `environment.txt` the same as runs 45456990497's and 45489450935's, `wallpaper.txt`
differing only in its LastUse times). Dispatched by hand through `.github/workflows/os-probe.yml` (job `probe`, passes
`button,controls`): <https://github.com/tiborgats/native-theme/actions/runs/45537062519>, green on attempt 1.

Only the `probe.toml`, `settings.txt` and environment records are kept; `CAPTURES.sha256` names every PNG of the run's
artifacts, which stay there.

## What is here

- `analysis/hover_moves.py` and `hover_moves-macos-26.txt`: the button pass's new `hover_moves` (per glass button,
  appearance and way of moving the cursor: `warp_post`, the warp and the mouse-moved at once, as every move of the
  passes; `post`, the mouse-moved alone; `warp_wait_post`, the warp, 400 ms, then the mouse-moved), beside the regular
  `hovered` state's figures.
- `analysis/settled.py` and `settled.txt`: the toolbar sheet's settled captures (ten captures 300 ms apart per state
  and backdrop, the first that is pixel for pixel the one before it taken).

## What the run reads

- **The macOS 26 glass hovers are back, with the moves unchanged.** The regular `hovered` state of `styles_b` (moves
  as in runs 45476265646 and 45489450935: warp and mouse-moved at once) reads run 45456990497's figures exactly: aqua
  590, 1061, 1500, 1758, 2503 pixels differ from the rest, by at most 10; darkAqua 594, 1090, 1548, 1808, 2552, by at
  most 22 or 23; a mouse-entered on every glass button. Each of the three ways of moving reads the same figures and a
  mouse-entered on every glass button, `warp_wait_post` dispatching no event in the 400 ms between its warp and its
  mouse-moved. So in this run the warp's local-event suppression does not swallow the hover's mouse-moved, and
  nothing in the way the cursor is moved separates a run that loses the hovers from one that keeps them: the loss of
  runs 45476265646 and 45489450935 is not reproduced, and its cause is not settled. Not verified, noted for the next
  run that loses them: the runs that kept the hovers (45345116763, 45456990497, this one) ran the button pass beside the
  controls passes; the two that lost them ran it beside the motion, events and focus-transition passes.
- **The toolbar sheet settles at once.** On both images, at every state (normal, highlighted, hovered, pressed), both
  appearances and every backdrop, the second capture is pixel for pixel the first (`settled_at = 2` of 10, no error):
  after the sheet's settle steps the toolbar button's look no longer changes within 300 ms. Each capture names its
  backdrop and reads it (`backdrop`, `backdrop_raw`). The re-check of `measured.md`'s macOS 26 toolbar-sheet rows
  against these reads is not done yet.

## The toolbar rows re-checked against these reads (M2.FS4c10)

Added after the run: `os-probe-macos-{15,26}-controls/controls.toml` and `controls-toolbar_sheet.toml`, written by run
45456990497's `analysis/measure_controls.py` (and `--sheet toolbar_sheet`) from this run's artifacts, and
`analysis/check_grey_in_part-macos-{15,26}-toolbar_sheet-toolbar_button.txt`, by run 45252258079's
`analysis/check_grey_in_part.py` on the settled toolbar-sheet captures.

- **The toolbar sheet's glass button on macOS 26 is still not a fixed layer.** Settled, the black-and-white solve
  composited over the grey backdrop differs from the grey capture in 2029–2045 of the part's pixels by up to 188–195
  levels (aqua) and 1921–1928 by up to 32–48.5 (darkAqua), as in run 45242497701 before the captures were settled; on
  macOS 15 0 pixels by more than 2. So the capsule adapts to what lies behind it, not to the moment it is captured, and
  no colour, alpha or label colour is solved for it from the backdrops: `measured.md`'s tahoe "button_label HOVER over
  the backdrops" row stays not settled.
- **The real toolbar's button (the controls sheet, opaque title bar) reads as before** on both images: macOS 26 fills
  rest #ffffffff / #252525ff, hovered #f2f2f2ff / #393939ff, pressed #e6e6e6ff / #484848ff, labels rest #4d4d4dff /
  #e9e9e9ff, hovered #494949ff / #ebebebff, pressed #454545ff / #ecececff; macOS 15 fills #ecececff / #343434ff,
  #dfdfdfff / #414141ff, #d2d2d2ff / #4e4e4eff, labels #606060ff / #b4b4b4ff (rest and hovered), #343434ff / #eeeeeeff
  pressed: every value of `measured.md`'s rows is kept.
