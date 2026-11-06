# OS probe, run 42215651196 (2026-11-06)

What macOS 15 reported to `tools/os-probe` (`native-theme-os-probe`) in the passes run 41195846264 ran on macOS 26
alone: P1 (the fonts' descriptors), P2 (the text styles' line heights), P4 (a selected table row's colours, with its
red-accent pass) and the controls pass (the pop-up button's title rect), for `macos-monterey` (pass MP-15rerun of the
0.8.0 measurement plan; U236, U238 and U144's macOS 15 read). Dispatched by hand through
`.github/workflows/os-probe.yml` on the harness commit `20d562599242413bbc075abbda34eab21834aaa2` (on
`v0.8.0-tools-probe-m2`), on 2026-11-06; the `probe` jobs ran on `macos-15` and `macos-26`:
<https://github.com/tiborgats/native-theme/actions/runs/42215651196>. GitHub deletes run artifacts after 90 days
(this run's on 2027-02-04); this folder keeps the macOS 15 files the facts can cite, as the run uploaded them (PNGs
byte for byte), and nothing else of the run. The run's macOS 26 job repeats run 41195846264's passes and is not kept.

`run.toml` records the image's builds: macOS 15.8.2 (24H232), runner image `macos-15-arm64` 20261104.0514.1, Xcode 16.4
(16F6), the probe built against the macOS 15.5 SDK (24F74) (`button-pass/environment.txt`); the screen at a backing
scale of 1 (`probe.toml` `[macos.environment]`). Every pass ran with the image's settings unless it names one: Reduce
Transparency on, Increase Contrast off, no accent colour set, desktop tinting on.

## What is here (`os-probe-macos-15/`)

- `probe.toml` (the main pass), `reduce-transparency-off/probe.toml` (`reduceTransparency` written false, read back 0),
  `increase-contrast-on/probe.toml` (`increaseContrast` written true, read back 1), `accent-red/probe.toml`
  (`AppleAccentColor` written 0, read back 0), each pass but the main one with its `settings.txt`.
- `renders/selection/<appearance>-<condition>.png` and the same under each of the three passes: the selected-row
  window's captures (`[macos.selection]`).
- `button-pass/probe.toml` and `environment.txt`: the button pass, whose buttons' fonts carry the same descriptor
  reads, and the job's build record.
- `fonts.toml`, `selection.toml`, `controls.toml`: the outputs of `analysis/read_fonts.py`, `measure_selection.py` and
  `read_controls.py`, copied unchanged from `2026-10-31-run-41195846264/analysis/` on `v0.8.0-rc1` (the facts lane's,
  2026-11-02) and run from the repository root with this folder as their argument. Each docstring is its method;
  `input_image` names every file it reads.

## What the passes read

This folder states no fact: converting a reading into a value is the facts' work.

- **P1 and P2, fonts** (U236's `token.text.keyed.<style>.font.{style,axes,features}`, `.line_height`,
  `small_system.font.{family,weight}`, `button.label.text.font.*`). `fonts.toml` decides `style`, `axes`, `features`
  and `line_height` for the same fonts as run 41195846264's macOS 26 `fonts.toml`: in each of the three probe passes 42
  fonts, 37 with `style`, 34 `axes`, 36 `features`, 32 `line_height`; all 76 of the button pass's. The small system
  font is `.AppleSystemUIFont`, weight trait 0, `kCTFontCSSWeightAttribute` 400 (`probe.toml`
  `[macos.fonts.named.systemFontOfSize_smallSystemFontSize]`).
- **U144** (the CSS weights `macos-monterey` lacked): `kCTFontCSSWeightAttribute` reads 400 for caption 2
  (`.SFNS-Medium`, weight trait 0), 600 for caption 1 emphasized (`.SFNS-Medium`, 0.3), 700 for title 3 emphasized
  (`.SFNS-Semibold`, 0.4) and 700 for headline emphasized (`.SFNS-Heavy`, 0.4), in `probe.toml`
  `[macos.fonts.preferredFontForTextStyle.caption2.descriptor]` and
  `[macos.fonts.preferredFontForTextStyle_emphasized.<style>.descriptor]`. They differ from macOS 26's 510, 510, 590
  and 860.
- **P4, the selected row** (U236's `role.selection.{foreground_inactive,link,disabled.foreground}` and the border).
  `selection.toml`: every sample matches its capture (`samples_match` true, 16 checks); over the emphasized selection
  `secondaryLabelColor` solves to `#feffffb3` (aqua) and `#fefffeb3` (darkAqua), `tertiaryLabelColor` to `#fffffe59`
  and `#ffffff59`, `linkColor` to `#0068daff` and `#419cffff` (opaque, drawn as outside the selection); no pass,
  appearance or condition has a stroke pixel at the selection's edge (`border = "absent"`).
- **The controls pass** (U238, `combo_box.body.padding.{left,top,bottom}` of the bordered pop-up). `controls.toml`,
  the same in every pass and appearance: mini 5, 1, 1; small 6, 1, 1; regular 8, 1, 3; large 8, 5, 7 (left, top,
  bottom, points).
