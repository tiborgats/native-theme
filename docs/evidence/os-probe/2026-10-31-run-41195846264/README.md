# OS probe, run 41195846264 (2026-10-31)

What macOS 26 reported to `tools/os-probe` (`native-theme-os-probe`) in the probe's third extension for the 0.8.0 plan
(four measurement passes for the `macos-tahoe` gaps: the fonts' descriptors, the text styles' line heights, a selected
table row's colours and the focus ring), dispatched by hand through `.github/workflows/os-probe.yml` on the harness
commit `18d2a5d81cc5a171a1df0c02eac39760e3aa36b5` ("tools(os-probe): a ticker box flips at every run-loop step while
the focus transition records, so the recording keeps every frame the ring moves in", on `v0.8.0-tools-probe-m2`), on
2026-10-31, with `jobs=probe` and `images=["macos-26"]`:
<https://github.com/tiborgats/native-theme/actions/runs/41195846264>. GitHub deletes run artifacts after 90 days; this
folder keeps the files the macOS facts can cite, as the run uploaded them (PNGs and recordings byte for byte), and
nothing else of the run.

`run.toml` records the image's builds: macOS 26.6.2 (25G83), runner image `macos26` 20261021.0445.8, Xcode 26.6
(17F113), the probe built against the macOS 26.5 SDK (25F70); the screen 1024 x 768 at a backing scale of 1
(`probe.toml` `[macos.environment]`). Every pass ran with the image's settings unless it names one: Reduce
Transparency on, Reduce Motion on, Increase Contrast off, no accent colour set, keyboard navigation
(`AppleKeyboardUIMode`) unset.

The two earlier runs of the same passes, 41189405926 (`b309d4af`) and 41192705332 (`9dafb0ca`), are not kept. In the
first the focus recordings were timed by run-loop steps that ran slower than planned, so the unfocus fell at the
recording's end, and the selected row's samples beside the glyphs fell on the glyphs; in the second, timed by the
clock, the recordings kept a frame only about every 100 ms while the ring moved. Their font and line-height readings
are this run's.

## What is here (`os-probe-macos-26/`)

- `probe.toml` (the main pass), `reduce-transparency-off/probe.toml`, `increase-contrast-on/probe.toml`,
  `accent-red/probe.toml` (`AppleAccentColor` 0), each with its `settings.txt`: the fonts, line heights and selected
  row below, with the rest of the main pass as before.
- `button-pass/probe.toml` and `environment.txt`: the button pass, whose buttons' fonts
  (`[macos.button_pass.sheets.<sheet>.buttons.<button>.font]`) carry the same descriptor reads.
- `renders/selection/<appearance>-<condition>.png` and the same under `reduce-transparency-off/`,
  `increase-contrast-on/`, `accent-red/`: the selected-row window's captures.
- `button-pass-focus/probe.toml`, `settings.txt` and `<appearance>/unfocused-layer.png`,
  `focused-<size>-<kind>-layer.png`: the focus pass (keyboard navigation on, each push button made first responder),
  the layers the probe solved.
- `button-pass-focus-transition/` (the image's Reduce Motion, on) and
  `button-pass-focus-transition-reduce-motion-off/` (`reduceMotion` false): `probe.toml`
  (`[macos.button_pass.focus_transition]`), `settings.txt`, and per appearance and button `transition-<button>.mov`
  (the screen recording), `.crop` (ffmpeg's crop, the button's frame grown by 12 pt), `-frames/NNNNN.png` (the
  recording's frames cut to it) and `-frames.csv` (their times); `ffmpeg.txt` in the first folder records the ffmpeg
  install and each cut.
- `focus-ring.toml`, `focus-transition.toml`: the output of `analysis/measure_focus_ring.py` (its docstring is the
  method; `input_image` names every file it reads), run from the repository root with this folder as its argument.

## The passes and the gaps they read

The rows are those of the `macos-tahoe` gap classification's "(c) needs a measurement", passes P1, P2, P4 and P5. This
folder states no fact: converting a reading into a value is the spec's and the facts' work.

- **P1, font descriptor reads** (122 rows: `tokens.text.keyed.<28 styles>.font.{style,width,axes,features}`,
  `small_system.font.{family,weight}`, U146's four weights,
  `widgets.button.label.text.font.{style,width,axes,features}`). Each font's `descriptor` table in `probe.toml` (and
  the two other passes'): `[macos.fonts.preferredFontForTextStyle.<style>.descriptor]`,
  `[…preferredFontForTextStyle_emphasized.<style>.descriptor]`, `[…named.<font>.descriptor]` (label, message, palette,
  menu bar, control content, and `systemFontOfSize_smallSystemFontSize`, the small system font, with its `familyName`
  and `weightTrait`); the buttons' in `button-pass/probe.toml`. The reads (`[macos.fonts] descriptor_method`): the
  symbolic traits, `NSFontSlantTrait`, `NSFontWidthTrait`, `italicAngle`, the descriptor's attributes,
  `CTFontCopyVariation` at the font's size and at twice it, `CTFontCopyVariationAxes`, the feature settings
  (descriptor and `CTFontCopyFeatureSettings`), `kCTFontOpticalSizeAttribute` and `kCTFontCSSWeightAttribute`. U146:
  the CSS weight is 510 for caption 2 (`.SFNS-Medium`, weight trait 0), 510 for caption 1 emphasized (`.SFNS-Medium`,
  0.3), 590 for title 3 emphasized (`.SFNS-Semibold`, 0.4) and 860 for headline emphasized (`.SFNS-Heavy`, 0.4), the
  same in the three passes; the small system font's 400. No text style and no named font the facts key has feature
  settings or a variation attribute in its descriptor (`unavailable: …`, as the OS answered;
  `monospacedDigitSystemFontOfSize:weight:` alone has a feature setting); their variation holds `wght` where it is not
  the default and `opsz`, which is the point size, at least the axis minimum of 17, at both sizes read.
- **P2, line height** (29 rows: `tokens.text.keyed.<28>.line_height_pt`, `widgets.button.label.text.line_height_pt`).
  `[macos.line_height.fonts]` in `probe.toml` (and the two other passes'), per style and named font and per control
  size's system font (`systemFontOfSize_systemFontSizeForControlSize.<size>`, the button label's), plain and with the
  tight and loose leading traits: `defaultLineHeightForFont:`, a two-line layout's line fragments and advance, and
  NSTextField's fitting heights (`[macos.line_height] method`). Where a font refuses a leading trait its
  `symbolicTraits_set` lacks it (the named fonts, subheadline and footnote loose).
- **P4, selected-row colours** (8 rows: `colors.{light,dark}.surfaces.selection.{foreground_inactive,link,border,
  disabled.foreground}`). `[macos.selection]` in `probe.toml`: per appearance, the selected row emphasized (the table
  first responder: `row_view.isEmphasized` true, `interiorBackgroundStyle` emphasized) and not, each column's glyph
  and the row's background beside it (`samples`), the colours NSColor resolves outside any background style
  (`reference`), and the captures. The glyphs are opaque composites over the selection: a translucent colour's own
  alpha follows from two backgrounds, the image's accent here and the red one in `accent-red/probe.toml`. In the
  emphasized row the label colours draw white over the selection (`labelColor` `#ffffffff`; `secondaryLabelColor`,
  `tertiaryLabelColor` lighter over it), while `linkColor` and `gridColor` draw as outside it; a disabled label draws
  as an enabled one, in both rows. The `r1_bottom_edge_NN` strip shows no separator or grid line at the row's edge
  (the inset style's selection).
- **P5, the focus ring** (4 rows, U100: `widgets.button.body.focus.{dash,corners.curve,shadow,transition}`).
  `focus-ring.toml` from the focus pass's layers: per appearance and push button, the ring along each side's middle
  (`profile_<side>`), its band's alpha and gaps (`band_gaps` 0 and one alpha along every straight side: a solid ring),
  what is drawn outside the ring (`beyond_ring_pixels` 0 everywhere, within the 6 px the layer reaches), and each
  outer corner fitted by a circle and by a superellipse (mini to regular: the circle within 0.07 px rms; the large and
  extra-large capsules' corners fit neither as well, circle 0.28 and 0.37 px, superellipse n 2.5 at 0.13 and 0.17 px).
  The ring reads the same as run 41006842699's (width 4 px, 3 outside the frame). `focus-transition.toml` from the
  recordings: per pass, appearance and button each frame's change from the unfocused first frame, and the focus-in and
  focus-out edges. With the ticker the recordings keep a frame every 33 ms. Focus in, the ring arrives in 217 to 250
  ms through two steps of about 100 ms each in six of the eight recordings (shares about 0.2 to 0.5, then 0.75 to 0.92
  of the final change), the second step's changed bounds (77 x 36 or 79 x 40 px) larger than the final ring's (73 to
  75 x 32 px), so the ring settles inward; the first recording of each pass (aqua, regular plain) shows the ring
  complete within one frame (33 ms). Focus out, the ring is gone within one frame (17 to 50 ms) in every recording.
  The two Reduce Motion settings read the same.
