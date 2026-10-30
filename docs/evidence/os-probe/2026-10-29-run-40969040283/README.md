# OS probe, run 40969040283 (2026-10-29)

What macOS 26 reported to `tools/os-probe` (`native-theme-os-probe`) in spike M of the 0.8.0 plan (the macOS
probe's button pass), dispatched by hand through `.github/workflows/os-probe.yml` on `v0.8.0-tools-spike-m` at
`7c9b6e50c0841c30c6ab1ddac1a16e3272029f62` ("tools(os-probe): a button pass in the key window", merged into
`v0.8.0-tools`), on 2026-10-29: <https://github.com/tiborgats/native-theme/actions/runs/40969040283>. GitHub deletes
run artifacts after 90 days; this folder keeps the files the macOS facts (`docs/facts/macos/`) cite, as the run
uploaded them (PNGs byte for byte), and nothing else of the run.

The run had two jobs, `macos-26` and `macos-15`. Only the `macos-26` job's files are here (`os-probe-macos-26/`,
`macos-tahoe`'s facts, M1a); `macos-15`'s join when `macos-monterey`'s facts cite them (M1b). `run.toml` records
the image's builds: macOS 26.6.2 (25G83), runner image `macos26` 20261013.0424.1, Xcode 26.6 (17F113), the probe
built against the macOS 26.5 SDK (25F70), Rust 1.99.1; the screen 1024 x 768 at a backing scale of 1, no GPU
reported (`button-pass/environment.txt` of the artifact, `probe.toml` `[macos.environment]`).

## What is here (`os-probe-macos-26/`)

- `probe.toml`: the main pass, with the image's settings: Reduce Transparency on and Reduce Motion on (the image's
  defaults, `[macos.environment.accessibility]`), Increase Contrast off, no accent colour set. The NSColors per
  appearance (`[macos.colors.<appearance>]`, sRGB `#rrggbbaa`), the fonts (`[macos.fonts]`), the controls' metrics
  (`[macos.controls]`) and the key-window section with its sRGB calibration strip (`[macos.key_window]`).
- `reduce-transparency-off/probe.toml`, `increase-contrast-on/probe.toml`: the same probe with Reduce Transparency
  off, and with Increase Contrast on; each pass's `settings.txt` (in the artifact) records the write and its
  read-back (`reduceTransparency` 0; `increaseContrast` 1).
- `renders/<appearance>/push_button-regular{,-disabled}.png` and the same under `reduce-transparency-off/`: the
  offscreen renders of the regular push button (a never-key borderless window, transparent background) in the main
  pass and with Reduce Transparency off.
- `button-pass/probe.toml`: the button pass (`--button-pass`, `[macos.button_pass]`, whose `method` and `states`
  keys describe it): a key, main window of the active application holding push buttons (`bezelStyle` push, title
  "Button") per control size (mini, small, regular, large, extraLarge) and kind (plain; default, the Return key
  equivalent; `pushOnPushOff` off and on), captured per appearance (aqua, darkAqua) in the states normal, pressed
  (`highlight:` YES), disabled (`setEnabled:` NO), hovered (the cursor over one button, a mouse-moved event posted)
  and clicked (a real mouse-down posted, captured during the press, at the regular size); `diff_from_normal` counts
  the pixels each capture changes inside each button's frame grown by 6 pt. The pass ran with the image's settings
  (Reduce Transparency on, `[macos.environment.accessibility]`), and every capture was taken with the window key
  and the application active (`all_captures_key = true`).
- `button-pass/<appearance>/{normal,pressed,disabled,clicked-regular-plain,clicked-regular-default}.png`: the
  captures the analysis reads (the hovered captures are not kept: their `diff_from_normal` in `probe.toml` is the
  evidence, 0 differing pixels for every button).
- `button-pass/analysis.toml`: the output of `analysis/measure_button_pass.py` (its docstring is the method; it names
  every image it reads in `input_image`): each button's fill per appearance and state, whether a stroke or a shadow
  is drawn, the clicked captures against `highlight:`, and the push button cell's title insets per size.

## Reading the captures

The captures' bytes are sRGB: the same run's key-window calibration strip reads each sRGB patch exactly in the raw
bytes (`probe.toml` `[macos.key_window.<appearance>.calibration.*]`, `exact = true`), while the PNG's embedded
display profile would shift them (`colorAtXY_sRGB` there). The analysis takes the bytes as they are.

A capture holds what the window server composited: a translucent bezel is recorded as its colour over the window
background (`windowBackgroundColor`, #ffffff in aqua and #1e1e1e in darkAqua, `window_background` in the analysis),
not as the layer's own colour and alpha.
