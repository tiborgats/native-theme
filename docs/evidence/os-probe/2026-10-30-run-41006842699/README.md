# OS probe, run 41006842699 (2026-10-30)

What macOS 26 reported to `tools/os-probe` (`native-theme-os-probe`) in the probe's second extension for the 0.8.0
plan (the button pass solving each button's layer), dispatched by hand through `.github/workflows/os-probe.yml` on
the harness commit `c46cb7baf2b2340a25bd9062e40b75e150ad2979` ("tools(os-probe): the button pass solves each button's
layer, in every condition, bezel style and size", on `v0.8.0-tools`), on 2026-10-30:
<https://github.com/tiborgats/native-theme/actions/runs/41006842699>. GitHub deletes run artifacts after 90 days;
this folder keeps the files the macOS facts (`docs/facts/macos/`) cite, as the run uploaded them (PNGs byte for byte),
and nothing else of the run.

The run had two jobs, `macos-26` and `macos-15`: the `macos-26` job's files are in `os-probe-macos-26/`
(`macos-tahoe`'s facts), the `macos-15` job's in `os-probe-macos-15/` (`macos-monterey`'s facts; its own section
below). `run.toml` records the image's
builds: macOS 26.6.2 (25G83), runner image `macos26` 20261021.0445.8, Xcode 26.6 (17F113), the probe built against
the macOS 26.5 SDK (25F70); the screen 1024 x 768 at a backing scale of 1 (`probe.toml` `[macos.environment]`).
Every pass ran with the image's settings unless it names one: Reduce Transparency on, Reduce Motion on, Increase
Contrast off, no accent colour set.

## What is here (`os-probe-macos-26/`)

- `probe.toml`, `accent-red/probe.toml`: the main pass (NSColors per appearance, fonts, controls, and the token reads
  `[macos.tokens]`: NSEvent's intervals, the caret-blink and key-repeat user defaults, NSAnimationContext's duration,
  a new push button's periodic delay), with the image's accent and with `AppleAccentColor` 0 (red).
- `<pass>/probe.toml` of the button passes (`[macos.button_pass]`, whose `method`, `backdrops`, `layer_method` and
  `states` keys describe them): `button-pass` (the window key and main, the application active),
  `button-pass-not-key` (another window of the probe key), `button-pass-inactive` (the application deactivated),
  `button-pass-increase-contrast-on`, `button-pass-accent-red`, `button-pass-focus` (Full Keyboard Access on, each
  push button made first responder). Each has the sheets `push` (push buttons: plain, default, pushOnPushOff off and
  on, per control size) and `styles_a`, `styles_b`, `styles_c` (every mapped bezel style, the 26 border shapes, tint
  prominences, glass, destructive, NSComboButton and image buttons, per control size), the states normal, pressed
  (`highlight:` YES) and disabled, the toolbar and caret windows and the windowless geometry (`[…geometry]`).
- `<pass>/<appearance>/[<sheet>/]<state>-layer.png`: each state's layer, its straight colour and alpha solved by the
  probe per pixel from the captures over black and white backdrops; for the Increase Contrast pass also
  `<state>-over-grey.png`, the capture over grey, against which the analysis checks what it reads there (that pass's
  1 px edge line is not one linear overlay).
- `button-pass-focus/<appearance>/unfocused-layer.png`, `focused-<size>-<kind>-layer.png`: the push sheet without
  and with each button focused (the latter the button's region only).
- `button-pass[-accent-red]/<appearance>/caret/caret-{0,1}.png`: two captures of a text view's caret as it blinks.
- `layers-<pass>-<sheet>.toml`, `focus.toml`, `caret.toml`, `geometry.toml`, `derivations.toml`: the output of
  `analysis/measure_button_layers.py` (its docstring is the method; `input_image` names every file it reads): per
  pass, sheet, appearance, state and button the fill, stroke, corners, label and icon colour and shadow; the focus
  ring; the caret; the cell geometry; the measured derivations' passes.
- `tiebreak-corners.toml`: the output of `analysis/tiebreak_corners.py`, the corner radii the facts cite (a
  least-squares circular corner per corner square, read from the same layers; the tie-break between the lane and its
  verifier replaced the per-row fit of `layers-*.toml`'s `corner_*`, which was unstable on capsules and translucent
  bezels).
- `tiebreak-derivations.toml`: the output of `analysis/tiebreak_derivations.py`, the passes of the accent-following
  looks (default, checked, primary, tinted; normal, pressed and disabled) under the image's accent and the red one,
  measured with `measure_button_layers.py`'s own method, and the dark caret's colour as NSColor reports it. For it the
  accent-red pass's `pressed-layer.png` and `disabled-layer.png` of the push and styles_b sheets are here too (byte
  for byte as the run uploaded them).

## Reading the layers

The probe solved each layer from the two captures' raw bytes (sRGB, as spike M's calibration strip showed) and
checked it against the capture over grey and over the window's own background (`check_grey`, `check_window`). A
layer is the button's own paint: a translucent bezel is stated as its colour and alpha, not as its composite over
the window background. Known limits (the probe's report): Increase Contrast's layer is not one linear overlay at its
edge line; the caret width is read from pixels only; pressed toolbar items without a view and NSComboButton pressed
were not captured; corner radii are circle fits.

## The macos-15 half (`os-probe-macos-15/`)

The `macos-15` job of the same run and harness commit: macOS 15.7.9 (24G830), runner image `macos15` 20260907.0337.1,
Xcode 16.4 (16F6), the probe built against the macOS 15.5 SDK (24F74), the screen 1024 x 768 at a backing scale of 1
(`probe.toml` `[os]`, `[probe]`, `[macos.environment]`; `run.toml`). The image's settings as on macos-26: Reduce
Transparency on, Reduce Motion on, Increase Contrast off, the image's accent (controlAccentColor #007aff in both
appearances). The passes are the macos-26 job's; what macOS 15 lacks the probe did not draw and says so
(`"unavailable: … needs macOS 26"`: the `extraLarge` size, `glass`, `primary`, `tinted` and the border shapes).

What is here, the files `macos-monterey`'s facts cite and the inputs of the analysis output they cite (byte for byte
as the run uploaded them):

- `probe.toml`, `accent-red/probe.toml`, `reduce-transparency-off/probe.toml`, `increase-contrast-on/probe.toml`: the
  main pass (NSColors, fonts, the token reads) with the image's settings, with the accent red, with Reduce
  Transparency off and with Increase Contrast on (the colour roles, the text styles, the tokens);
- `<pass>/probe.toml` of the six button passes and, for the analysis, `<pass>/<appearance>/[<sheet>/]<state>-layer.png`
  of the key, not-key, inactive and Increase Contrast passes (with `<state>-over-grey.png` for the last), the
  accent-red pass's push and styles_b layers, and `button-pass[-accent-red]/<appearance>/caret/caret-{0,1}.png`;
- the analysis output, written by the three scripts of `analysis/` run with the image folder as their argument
  (`… os-probe-macos-15`; their docstrings are the method, the same as for macos-26, and run without it they rewrite
  the macos-26 files byte for byte): `layers-<pass>-<sheet>.toml`, `geometry.toml`, `caret.toml`,
  `tiebreak-corners.toml`, `tiebreak-derivations.toml`.

What the macOS 15 readings show, and how the facts take them (docs/facts/macos/button.toml's macos-monterey header):

- the push bezel and those drawn like it draw an edge line and a shadow inside the frame (their alignment insets),
  which the layer solve does not separate: those strokes and shadows are pending (ledger U135, U136); the circle
  fit of their corners has an rms of 0.2-0.56 coverage, so the facts take a corner only where every corner's rms is
  at most 0.06, the tie-break's own criterion, the capsule case included (U142): `tiebreak-corners.toml` still
  spells such a corner as `tiebreak_corners.py` reads it;
- the default button, a checked toggle (at rest and pressed) and most square bezels draw a gradient (`fill =
  "non_uniform"`): their fill and label are pending (U141), and measure_button_layers.py writes no
  `derivations.toml` (the default fill reads a gradient, the primary and tinted buttons are missing, and the key
  pass's light captures show no caret);
  `tiebreak-derivations.toml` holds the uniform cases (a disabled checked toggle, which reads the same under both
  accents) and the caret;
- the caret shows in one capture only (the accent-red pass, light): the key pass's and the dark captures are
  identical pairs (`caret.toml` `changed_pixels = 0`), so the caret's colour is read from textInsertionPointColor
  under both accents (`tiebreak-derivations.toml` `caret_color_light`, `caret_color_dark`);
- the focus ring lies partly inside the frame and the focus analysis reads uneven sides at every size, so every
  focus key is pending (U140): `focus.toml` and the focus pass's layers are not committed, as no fact cites them.

Not committed, as nothing cites them: `environment/` (the wallpaper record), `sdk-headers/` (the facts cite the
headers as `xcode@16.4`, which `os-probe.yml`'s macOS job checks), `renders/`, the captures over black and white,
`app-bundle/`, `tinting-off/`, `not-key/` and the hovered and clicked captures (the state model cites their
`diff_from_normal` and `diff_from_pressed` counts in `button-pass/probe.toml`).

