# OS probe, run 45345116763 (2026-11-24)

The controls pass of `tools/os-probe` on branch `m2-fs4c` at commit `3edfe19e` (M2.FS4c6): what is drawn behind each
part read and recorded (`targets.<t>.behind_part`, a table's `enclosing_scroll_view_tree`), over_backdrop checked by
the data (`<appearance>.normal.backdrop_shows`), every posted event's own-window check, the radio buttons (hovered and
pressed) and the disabled check boxes and radio buttons (at rest); dispatched by hand through
`.github/workflows/os-probe.yml` (jobs `probe`, passes `controls,controls-accent,button`, images macOS 15 and 26):
<https://github.com/tiborgats/native-theme/actions/runs/45345116763>. Green: all six probe jobs passed on the first
attempt, the iOS host job skipped. macOS 15.8.2 (image 20261118.0550.1, Xcode 16.4) and macOS 26.7.2 (image
20261117.0516.1, Xcode 26.6), rustc 1.100.0. Reduce Transparency off for both controls passes (`settings.txt`: image's
1, read back 0; `accessibilityDisplayShouldReduceTransparency = false`, which the step now checks); Reduce Motion on
(`environment/accessibility.txt` `reduceMotion: 1`; the probe reads `accessibilityDisplayShouldReduceMotion = true`);
`-g com.apple.swipescrolldirection` absent on both images.

## What is here

- `os-probe-macos-{15,26}-controls/`: the controls pass as uploaded (450 PNGs each), with `controls.toml` and
  `controls-toolbar_sheet.toml` by `analysis/measure_controls.py`.
- `os-probe-macos-{15,26}-controls-accent/`: the same pass with the red accent (`AppleAccentColor` written 0 before
  the pass, read back 0: `button-pass-controls-accent/settings.txt`), 450 PNGs each, measured the same way
  (`--pass-dir button-pass-controls-accent`).
- `os-probe-macos-{15,26}-button/`: the button pass, run only to check that its presses still release with the new
  `click_at` (green; every mouse-up posted to the HID event tap with `mouseUp_window_is_ours = true`, no
  `release_by_watchdog`): its `probe.toml` and environment records only, its PNGs not kept (no row is read from it).
- `analysis/`: `measure_controls.py` (run 45226132792's, extended: the new targets, a check box's or radio button's
  title read as `<key>_title`, `--pass-dir`; it reads run 45252258079's folders byte for byte as that run's files);
  `compare_all.py` (run 45242497701's) and `compare_all-run-45252258079-*.txt`; `summary.py` and `summary-*.txt` (the
  new reads per target: over_backdrop and its covers, backdrop_shows, the reveal's scrolls, the releases);
  `accent_mix.py` and `accent_mix.txt` (the accent test below).

## What the run reads (the facts are measured.md's)

- **Against run 45252258079** every key of both images' `controls.toml` and `controls-toolbar_sheet.toml` reads the
  same (fill, label, ink, edges, drawn boxes, hover zero-ness), but one pixel count (macOS 26 toolbar sheet,
  `aqua.hovered.toolbar_button` `diff_from_normal` 1826 -> 1828): the release posted by the main thread's timer and
  the new checks change no capture.
- **The source-list row's cover**: its scroll view's first subview, below the clip view, is an `NSVisualEffectView`,
  material 7, blendingMode behindWindow, the full scroll view; the table's `backgroundColor` reads `#f6f6f6cc`
  (style 3, effectiveStyle 3). Behind-window blending samples what lies behind the window, not the backdrop box, so
  the box never shows: `backdrop_shows.source_list_row` 0 of 4800 pixels differ, both images, both appearances;
  `over_backdrop` false now. The default-style table: `_NSScrollViewContentBackgroundView` holding an
  `NSVisualEffectView` (material 18, withinWindow), the table's `backgroundColor` `#ffffffff`, the clip and scroll
  view drawing their background.
- **The legacy knob**: the legacy scroller draws its knob slot behind the knob, opaque on macOS 15 in light
  (`backdrop_shows.scroller_knob_legacy` 0 of 570; in run 45252258079 the slot outside the knob reads #fafafa over black and white alike) and
  translucent elsewhere (570 of 570 dark on 15, 418 of 418 on 26): `over_backdrop` false, covered by "NSScroller
  (legacy knob slot)"; its layer is the knob over its slot.
- **The scroll sign**: from `doubleValue` 0.5, the line scroll with `wheel1` +1 leaves 0.458 and the one with -1 0.5
  again, every reveal on both images: a positive `wheel1` scrolls toward the document's top.
- **The knob drawn**: `knob_drawn` counts inside the knob's own rect read after the captures (macOS 15 legacy
  [205, 361, 15, 38] 570 pixels, overlay [428, 360, 12, 39] 468; macOS 26 legacy [206, 379, 11, 38] 418, overlay
  [431, 379, 6, 38] 228), in every hovered and pressed capture.
