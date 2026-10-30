# OS probe, run 41006842699 (2026-10-30)

What macOS 26 reported to `tools/os-probe` (`native-theme-os-probe`) in the probe's second extension for the 0.8.0
plan (the button pass solving each button's layer), dispatched by hand through `.github/workflows/os-probe.yml` on
the harness commit `c46cb7baf2b2340a25bd9062e40b75e150ad2979` ("tools(os-probe): the button pass solves each button's
layer, in every condition, bezel style and size", on `v0.8.0-tools`), on 2026-10-30:
<https://github.com/tiborgats/native-theme/actions/runs/41006842699>. GitHub deletes run artifacts after 90 days;
this folder keeps the files the macOS facts (`docs/facts/macos/`) cite, as the run uploaded them (PNGs byte for byte),
and nothing else of the run.

The run had two jobs, `macos-26` and `macos-15`. Only the `macos-26` job's files are here (`os-probe-macos-26/`,
`macos-tahoe`'s facts); `macos-15`'s join when `macos-monterey`'s facts cite them. `run.toml` records the image's
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
