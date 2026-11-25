# iOS host, run 45622616324 (2026-11-25): the iPad pointer hover (U113), not measured

The hover pass of `tools/ios-host` (M2.FS4c11) on branch `m2-fs4f`, commit `a1310abe7`, dispatched by hand through
`.github/workflows/os-probe.yml` (jobs `ios-host`, `ios_host_passes=hover`, `ios_host_set=lists`,
`ios_host_device=ipad`, images `["macos-15", "macos-26"]`):
<https://github.com/tiborgats/native-theme/actions/runs/45622616324>, **red on both images** (the pass fails where a
target saw no hover, as designed). Simulators: iOS 18.6 (22G86) and iOS 26.5 (23F77), iPad Pro (12.9-inch) (3rd
generation), Xcode 16.4 on macos-15.

Kept: the macos-15 job's `run.txt`, `environment.txt`, `hover{1,2}/index.toml`, `hover-differ-levels.txt` and the
UI test's event lines (`hover1-test-events.log`); the macos-26 job's log lines on the pass
(`ios-host-macos-26-job-log-excerpt.txt`). `CAPTURES.sha256` names the one PNG of the macos-15 artifact (the warm-up
screenshot); no hover capture was taken.

## What the run shows

- `XCUICoordinate.hover()` runs on the iPad simulator: every hover is "Synthesize event" in the UI test's log, no
  error (`hover1-test-events.log`).
- The pointer does not stay: of 24 targets per run (4 specimens x 3 backdrops x 2 appearances) the
  UIHoverGestureRecognizer on the specimen's view began for 3 (macos-15, hover 1), 0 (hover 2), 5 and 2 (macos-26),
  and each time it ended within the 1.5 s the capture waits (`the touch ended before the capture`, `run.txt`); every
  other target is `[[hover_not_seen]]` after 30 s. No capture was kept, so no hover colour is measured.
- UIKit does draw a pointer effect where the pointer arrives: in the three `[[motion]]` tables of phase `press`
  (macos-15 `hover1/index.toml`: `light/segmented_control/default` `white/hovered`, `light/bar_plain/default`
  `system/hovered`, `dark/bar_plain/default` `white/hovered`) a `_UIPointerEffectPlatterView` layer appears over the
  hovered segment (55 x 32 pt at x 83.5 pt, scale 1.026-1.036 easing back to 0.998) with a shadow animation
  (`shadowOpacity` to 0.12, `shadowRadius` 3, `shadowOffset` (0, 2.5), 0.41 s), and is gone by the sample at
  0.52 s. So the hover look is a transient pointer effect under a synthesized hover, too short for the screenshot the
  pass takes.

Not settled: the four HOVER colours (`expander.header_label`, `segmented_control.label`, `sidebar.item_label`,
`toolbar.button_label`); whether the sidebar row and the outline header draw any hover (their recognizers began only
on macos-26 and no effect layer was recorded for them).
