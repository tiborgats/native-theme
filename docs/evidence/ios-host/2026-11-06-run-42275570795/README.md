# iOS host, run 42275570795 (2026-11-06)

[The run](https://github.com/tiborgats/native-theme/actions/runs/42275570795): `os-probe.yml`'s ios-host job with
`ios_host_passes=press`, the measurement campaign's IP-motion and IP-toolbar passes. The harness is `tools/ios-host` on
`v0.8.0-tools-ios-m2` at `8baca1fe`: the press pass (`IosHost/IosHost.swift`, `press` mode, driven by
`IosHostUITests/PressTests.swift`) as in run 41020880059, with two additions:
- each pressed button's layers are recorded as `[[motion]]` tables: at rest when it is armed, at 0, 0.02, 0.05, 0.1,
  0.2, 0.35, 0.5 and 1 s after the touch went down, and at 0 to 0.45 s after its release; each sample lists every Core
  Animation animation on the button's layer tree (its class, key path, duration, timing function, from and to values,
  keyframes, a spring's mass, stiffness, damping, initial velocity, settling and perceptual durations and bounce) and the
  presentation transform, opacity, bounds and position of each layer that is animated, transformed or not opaque;
- the bar button items in their UIToolbar are pressed too (`bar_*`), a long press recognizer of no minimum duration on
  the toolbar reporting the touch.

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.8.2 (24H232) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPhone 16 Pro |
| `ios-host-macos-26/` | `ios-26` | macOS 26.7.2 (25G332) | Xcode 26.6 (17F113), iphonesimulator 26.5 (23F81a) | iOS 26.5 (23F77), iPhone 17 Pro |

Kept here, per image folder: `press1/index.toml` (a `[[render]]` per press capture with `highlighted_at_capture` and
`held_until_screenshot`, and the `[[motion]]` tables), `press1/screen/…` (the press crops; on iOS 26 the glass styles'
crops at medium only, the others at mini, small and large stay in the artifacts), `solved/press1.toml` and, on iOS 18,
`press-differ-levels.txt`.

What the run shows (each job's run.txt): on iOS 18 both press runs finished `ok` and wrote the same file set (one file
differs, the index: the presses' numbers and pages, from retakes); every bar item was pressed and captured. On iOS 26
the first press run finished `ok`, bar items included; the second ended with one failure (`prominent_clear_glass` large
over black, `pressed_selected`: the touch ended before the screenshot arrived after 5 retakes), so the two runs wrote
different file sets and were not compared (the job reports a failure). The facts read the first run.
