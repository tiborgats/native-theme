# OS probe, run 45456990497 (2026-11-24)

The controls pass of `tools/os-probe` on branch `m2-fs4c` at commit `7dc853dd` (M2.FS4c7): every cursor move made only
where `windowNumberAtPoint:` at its target, read before, gives the probe's window (`window_at_target`,
`window_at_target_is_ours`); the cursor parked inside the probe's window on the middle of its content view's left margin
(the point each sheet reads its backdrop at) where `hitTest:` gives the content view or the backdrop box
(`view_at_park_point`, `park_point_in_content_xy`), not right of the window; no NULL event posted; the accent the
probe's process sees recorded (`[macos.environment.accent]`) and, for the red-accent pass, checked by the workflow.
Dispatched by hand through `.github/workflows/os-probe.yml` (jobs `probe`, passes `controls,controls-accent,button`,
images macOS 15 and 26): <https://github.com/tiborgats/native-theme/actions/runs/45456990497>. macOS 15.8.2 (image
20261118.0550.1, Xcode 16.4) and macOS 26.7.2 (image 20261117.0516.1, Xcode 26.6), rustc 1.100.0: the images of run
45345116763. Reduce Transparency off for both controls passes (`settings.txt`: image's 1, read back 0; the probe reads
`accessibilityDisplayShouldReduceTransparency = false`).

Green on attempt 3: five probe jobs passed on attempt 1; the macOS 15 button job was cancelled before it got a runner
on attempts 1 and 2 (no steps run, queued 33 and 15 minutes) and passed on attempt 3; the iOS host job skipped.

## What is here

- `os-probe-macos-{15,26}-controls/`, `os-probe-macos-{15,26}-controls-accent/`: the controls pass as uploaded, with
  and without the red accent, each with `controls.toml` and `controls-toolbar_sheet.toml` by `analysis/measure_controls.py`.
- `os-probe-macos-{15,26}-button/`: the button pass, run to check that it still parks, hovers and presses with the new
  moves: its `probe.toml` and environment records only, its PNGs not kept (no row is read from it).
- `analysis/`: `measure_controls.py`, `compare_all.py` and `summary.py` (run 45345116763's, unchanged), with
  `compare_all-run-45345116763-*.txt` and `summary-*.txt`; `compare_rest.py` (new: the rest captures, normal and
  highlighted over every backdrop and their layers, compared pixel by pixel with an earlier run's) and
  `compare_rest-run-45345116763-*.txt`; `compare_button.py` (new: every leaf of two button passes' `[macos.button_pass]`,
  the run-to-run reads and the parking records left out) and `compare_button-run-45345116763-*.txt`.

## What the run reads

- **The park point.** At every `normal.cursor`, `hovered.cursor_parked` and `pressed.cursor_parked` of both sheets, both
  appearances, both images and both controls passes: `window_at_target_is_ours = true`, `window_under_cursor_is_ours =
  true` (run 45345116763: false at every `cursor_parked`), `view_at_park_point = ["NSView"]` (the content view: the
  backdrop box is hidden for the window's own background), at (10, 284) in the controls sheet's content view on macOS 15,
  (10, 286) on macOS 26, (10, 60) in the toolbar sheet's.
- **No target hovered at the park point.** The controls sheet's rest captures (`normal*`, `highlighted*`, over the
  window's background and each backdrop, and their layers) are pixel for pixel those of run 45345116763 on both images,
  with and without the accent (`compare_rest-*`: 0 files differ on macOS 15; on macOS 26 only toolbar-sheet files
  differ), and every key of both images' `controls.toml` reads the same (`compare_all-*`: empty), the macOS 15 toolbar
  sheet's too.
- **The macOS 26 toolbar sheet differs, and not by the park point alone.** Its toolbar button's captures over the
  backdrops change between the runs (`compare_rest-run-45345116763-macos-26*.txt`): at the button's centre, aqua
  `normal-over-white` reads #545453 in run 45345116763 and #fdfdfd now (over black #131313 in both); darkAqua
  `normal-over-grey` #585858 then and #ececec now. The button's glass takes a dark look over the black backdrop and a
  light one over the white, and the capture over the next backdrop catches it before or after it switches: the
  red-accent pass of this run differs from run 45345116763's in other files (aqua `normal-over-grey`, darkAqua
  `highlighted-over-grey`) than the default pass does. So on macOS 26 the toolbar button's layer solved from the black
  and white captures is not one fixed layer, and `controls-toolbar_sheet.toml`'s macOS 26 reads of it (fill, edges,
  label, run 45345116763's and this run's alike) do not measure a fixed colour. Why the switch lands at another
  backdrop in this run (the cursor's mouse-entered events inside the window, or timing alone) is not verified.
- **The accent the probe sees.** Controls pass: `AppleAccentColor` absent (objectForKey: nil), `controlAccentColor`
  #007affff in both appearances. Red-accent pass: `AppleAccentColor = 0.0`, `controlAccentColor` #e0383eff (aqua) and
  #ff5257ff (darkAqua), on both images: the red accent's reads of run 40348854007, which the workflow's check takes.
- **What is drawn behind each part.** The content view's subviews under a target now go through the same reading as
  the views above them; no target's `over_backdrop`, `backdrop_covered_by` or `normal.backdrop_shows` changes
  (`summary-*.txt` against run 45345116763's: the content view's line reworded, the presses' `wait_steps` 2-4 and the
  pop-up's 8-12 as before).
- **The button pass** (`compare_button-run-45345116763-macos-{15,26}.txt`): every move and park on the probe's window
  (no `window_at_target_is_ours`, `window_under_cursor_is_ours` or `mouseUp_window_is_ours` false, no
  `release_by_watchdog`, `view_at_park_point = ["NSView"]` at every park). Of 86238 leaves (macOS 15) and 144647
  (macOS 26), beside the move and parking records and the counts of dispatched mouse events, only the method text
  differs (the new parking and moves) and `caret.darkAqua.capture_3.changed_from_capture_0` (macOS 15 40 -> 0 pixels,
  macOS 26 0 -> 42: whether the caret blinked between captures 0 and 3, four captures 6 pump steps apart).
