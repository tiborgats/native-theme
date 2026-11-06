# OS probe, run 42272240639 (2026-11-06)

What macOS 26 and macOS 15 reported to `tools/os-probe` (`native-theme-os-probe`) in the passes of the 0.8.0
measurement plan's macOS-probe rows (MP-symbols, MP-fonts, MP-accent, MB-default, MB-disabledpress, MB-image,
MB-combobutton, MP-pattern, MP-scale2, MP-windows, MP-toolbar, MP-timing, MP-route, MP-text, MP-controls and the caret
of MP-record), dispatched by hand through `.github/workflows/os-probe.yml` on the harness commit
`4b84d3611292a1d071669d6aa4c2dc86121001b4` (on `v0.8.0-tools-probe-m2`), on 2026-11-06, with `jobs=probe`,
`images=["macos-26", "macos-15"]` and `passes=probe,symbols,reads,reads-accent,reads-rtl,probe-contrast-accent,button,
button-focus-accent,button-key-not-main,button-main-not-key,button-inactive-contrast,focus-transition`:
<https://github.com/tiborgats/native-theme/actions/runs/42272240639>. GitHub deletes run artifacts after 90 days; this
folder keeps the files the macOS facts can cite, as the run uploaded them (PNGs and recordings byte for byte), and
nothing else of the run (left in the artifact: the button pass's per-backdrop captures, the plain sheets' layers its
earlier runs already hold, the hover captures but the combo buttons', the focus recordings' frames, the SDK headers).

`run.toml` records the images' builds. macOS 26 is **26.7.2 (25G332)** on this image (runner image macos26
20261102.0473.1), not the 26.6.2 of the earlier runs; Xcode 26.6 (17F113), the macOS 26.5 SDK. macOS 15 is 15.8.2
(24H232), Xcode 16.4 (16F6), the macOS 15.5 SDK. Both screens are at a backing scale of 1. Every pass ran with the
image's settings unless its `settings.txt` names one (Reduce Transparency on, Reduce Motion on, Increase Contrast off,
no accent colour, keyboard navigation unset).

This folder states no fact: converting a reading into a value is the facts' work. Each probe.toml's `method` keys
state how it read.

## What is here (per `os-probe-macos-26/` and `os-probe-macos-15/`)

- `probe.toml`: the main pass. New in it: each control's cell `lineBreakMode`, `wraps`, `truncatesLastVisibleLine`,
  `usesSingleLineMode`, `isScrollable` and the control's (`[macos.controls.<appearance>.<size>.<kind>.cell]`, U178);
  the line-height pass over `userFixedPitchFontOfSize:`, `monospacedSystemFontOfSize:weight:` and
  `monospacedDigitSystemFontOfSize:weight:` (`[macos.line_height.fonts.named.*]`, U325, U601); the leading-trait fonts
  with `familyName`, `weightTrait` and the descriptor table (`[macos.line_height.fonts.*.tightLeading]`,
  `.looseLeading`, U311).
- `symbols/`: the symbols pass (`--symbols`, `[macos.symbols]`; U102, U324, U613): per place the view's reads, its
  symbol configuration where AppKit has a getter, renders of the place with the symbol at 1 and 2 pixels a point
  (`<appearance>/*.png`, `toolbar/`, `status_item/`), the glyph and the six closest configurations of a rendered
  reference grid (`reference_grid.rows`: 69 point sizes × 9 weights × 3 scales); `mode_symbol_renders`, the
  multicolour symbol's colours.
- `reads/`, `reads-accent-red/` (`AppleAccentColor` 0), `reads-rtl/probe.toml` (the reads pass, `--reads`,
  `[macos.reads]`; keyboard navigation on in the first two): `timing` (U301), `route_picker` (U323), `focus_view`
  (U104's plain view), `text_rendering` (U317), `fallbacks` (U319), `controls` (U167–U172, U176, U191, U192, U194),
  `mirroring` (U318); with their renders and captures.
- `increase-contrast-on-accent-red/`: the main pass with Increase Contrast on under `AppleAccentColor` 0, its
  `probe.toml`, `settings.txt` and `renders/selection/` (U235's second selection colour).
- `button-pass/`: the key mode's `probe.toml` and `environment.txt`, and of its captures: the layers of the new sheets
  `default_a`–`default_c` (each style with the Return key equivalent, U92, U132) and `image_a`–`image_c` (the symbol
  beside the title on each style, U99, U139); every sheet's `disabled_pressed-layer.png` (U91, U131);
  `normal-over-pattern.png` (U98); `<state>-render-{1x,2x}.png` of the plain sheets and the toolbar's items (MP-scale2:
  U90, U130, U135, U136, U142, U143; MP-toolbar: U94, U134); the combo buttons' hover and click captures (U96, U137).
- `button-pass-key-not-main/`, `button-pass-main-not-key/`, `button-pass-inactive-increase-contrast-on/`: those modes'
  `probe.toml`, `settings.txt` and layers (U93, U133).
- `button-pass-focus-accent-red/`: the focus pass under `AppleAccentColor` 0, `probe.toml`, `settings.txt` and the
  focused and unfocused layers (U104).
- `button-pass-focus-transition/`: `probe.toml` (`[macos.button_pass.focus_transition.caret]`), `settings.txt`, and
  per appearance `transition-caret.mov` (the screen recording), `.crop` and `-frames.csv` (U316); the frames are cut
  from the recording with the `.crop` as the workflow's ffmpeg step does.

## What the passes read

- **Symbols.** The best match (score 0: the same pixels) of a plain push or borderless button's image is 9 pt at
  mini, 11 pt at small, 13 pt at regular, large and extra large, weight regular, scale medium, on both images, with no
  `symbolConfiguration` on the button. A table cell's image view gets one from AppKit on macOS 26 (`pointSize` 11, 13,
  15 for the small, medium and large row sizes, `weight: 0.23`; it matches 11/13/15 pt medium medium) and none on
  macOS 15 (it matches 11/13/15 pt regular medium); the same in the source-list, inset and plain styles. The
  bordered toolbar item's image matches 20 pt regular medium at 1x (score 0 on macOS 26, 0.0015 on macOS 15; the
  plain item's image view draws nothing in its offscreen render on macOS 26); the status item's 13 pt medium medium
  (1x, score 0.0021); the alert's icon view is 64 × 64 pt for every alert style. Disabled, the button's glyph keeps its shape (13 pt
  regular) at a lower coverage (max 0.22 against 0.45); the toggle's on state keeps the same image. The multicolour
  symbol draws in one colour at several alphas in the buttons (`opaque_colors` 0). The deprecated toolbar size mode
  reads back regular after `small` was set.
- **Reads.** The five named timing functions' control points are the same on both images (`default` (0.25, 0.1),
  (0.25, 1)); `NSAnimationContext`'s duration 0.25 and timing function nil, inside a grouping too. The route button's
  colours are `#000000d8` (light) and `#ffffffd8` (dark) normal and highlighted, `#5db4ffff` active, the same under
  the red accent. No focus ring is drawn round the plain focusable view on either image (its focused and unfocused
  captures are the same pixels), with keyboard navigation on. Text renders with greyscale antialiasing (no coloured
  pixel at 1x or 2x), `AppleFontSmoothing` and `CGFontRenderingFontSmoothingDisabled` absent,
  `AppleAntiAliasingThreshold` 4. The cascade list holds 41 descriptors for each language read. Under
  `-AppleLanguages (he)` alone the unlocalised probe stays left to right (`userInterfaceLayoutDirection` 0, the
  symbols drawn the same), so the mirroring reading is not decided by this run.
- **Buttons.** `disabled_pressed` draws as `disabled` (`diff_from_disabled` 0 pixels on macOS 15; at most 1 byte on 8
  glass buttons on macOS 26), and AppKit reports no cell highlighted. A posted click on a combo button at regular
  size draws no change from the normal capture at the capture's moment on either image (`diff_from_normal` 0), so its
  pressed look is not captured. Its subviews (`buttons.<case>.subviews`) give the title label's and the menu
  indicator's frames on macOS 15 (`NSSegmentItemLabelView`, `NSSegmentItemImageView`); on macOS 26 they are a
  segmented control holding a SwiftUI hosting view, with no label or indicator view. In the default sheets the window's `defaultButtonCell` is the first default button
  (`mini-flexible_push`). The key-not-main mode's sheets read key and not main, the main-not-key mode's main and not
  key, the inactive mode's neither, the application inactive.
