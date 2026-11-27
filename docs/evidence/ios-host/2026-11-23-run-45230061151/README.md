# iOS host, run 45230061151 (2026-11-23): the list set on iOS 18

[The run](https://github.com/tiborgats/native-theme/actions/runs/45230061151): `os-probe.yml`'s ios-host job (the
other jobs skipped) on `m2-fs4d` at `befc13ab`, with the passes `render,press`, the specimen set `lists`, the iPhone
simulator and no settings (each job's `run.txt`). The harness is this repository's `tools/ios-host` at `befc13ab`. The
list set draws, each over the system background and over black and white (the solve's backdrops): a plain
`UITableView` row, a `UISegmentedControl` of three segments (the first selected), a `.sidebar` list row, a `UITabBar`
of three items (the first selected), a `.sidebar` outline-disclosure header, a `UISlider` at 0.5, an `.insetGrouped`
list cell (the card), an `.insetGrouped` list row with a trailing `UISwitch` (off, on, the switch disabled off and on,
the cell disabled off and on) and SwiftUI's `Toggle` in a `Form` (off, on, `.disabled(true)` off and on). The press
pass holds a real touch (`XCUIElement.press(forDuration: 4)`, `IosHostUITests/PressTests.swift`) on the row, the
middle segment, the middle tab bar item, the slider's thumb, the sidebar row, the outline header and the list row's
switch, in light and dark over the three backdrops, and the screenshot is taken while the touch is held; the card and
the SwiftUI `Toggle` are not pressed.

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.8.2 (24H232) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPhone 16 Pro (iPhone17,1) |
| not kept | `ios-26` | macOS 26.7.2 | Xcode 26.6, iphonesimulator 26.5 (23F81a) | iOS 26.5 (23F77), iPhone 17 Pro |

Kept here, of the image `ios-host-macos-15`, copied unchanged from the run's artifact `ios-host-macos-15`:
`environment.txt`, `run.txt`, `differ.txt` and `differ-levels.txt` (the two render runs compared),
`press-differ.txt` and `press-differ-levels.txt` (the two press runs compared), `run1/index.toml` and
`press1/index.toml` (the environment, the screen, a `[[render]]` per crop with UIKit's `[[render.list_cell]]` and
`[[render.view]]` reads), the crops over the system background (`run1/screen/…/system/`, `press1/screen/…/system/`),
and the layers solved from the crops over black and white (`solved/run1.toml`, `solved/press1.toml` and
`solved/{run1,press1}/…`, `tools/ios-host/solve.swift`: straight RGBA). The second runs, the drawn images, the pages,
the crops over black and white, the logs and the `.xcresult` bundles stay in the run's artifacts.

`analysis/measure_lists.py` and `analysis/changed_parts.py` (their docstrings give the method) read the kept files:

`python3 -I docs/evidence/ios-host/2026-11-23-run-45230061151/analysis/measure_lists.py docs/evidence/ios-host/2026-11-23-run-45230061151/ios-host-macos-15`

writes `ios-host-macos-15/measure_lists.toml` (per specimen, appearance and state: the part's solved fill, UIKit's
reads of the list cell, the switch row's label and switch, the card's corner, the SwiftUI `Toggle`'s label against
the UIKit row's), and `changed_parts.py` with the same argument writes `ios-host-macos-15/changed_parts.toml` (what
each press changed in its part, and the card's edge pixels). Both were run again from the kept files and give the
kept outputs byte for byte (Pillow).

What the run shows:

- The macOS 15 job finished green, `done: ok` in every run, no crash report. The two render runs are byte-identical
  (205 files). The two press runs differ only in `index.toml`'s page and press numbers: the first press run took six
  presses again (the light row over the system background twice, over black three times, the dark sidebar row once:
  "the touch ended before the screenshot arrived"), the second none. The solve found no inconsistent pixel in any
  layer; the tab bar's layers spread in alpha by up to 7 levels between the backdrops (the bar's material), every
  other layer by at most 3.
- The macOS 26 job (the parked `ios-26` preset's) failed. Its render runs finished `ok` and were byte-identical, but
  both press runs stopped after six presses (the row and the middle segment, light, over the three backdrops):
  `PressTests.swift` waited 900 s for an element with the identifier `press-6`, the seventh press (the light tab bar
  item over the system background, in the order the macOS 15 job's second press run numbers them), and "neither
  press-6 nor press-done" appeared (the job's log). The identifier the host sets on the tab bar item never reached
  the accessibility tree on iOS 26.5, as the identifier set on the `UISwitch` in SwiftUI's `Form` did not in run
  45220318789 (so that `Toggle` is not pressed). Its artifact (421 MB) is not kept.
- No focus pass ran (`focus: not run`: the passes were render and press), and the host has no pointer-hover pass:
  nothing here reads a hovered state.

The values are in `.superpowers/sdd/v0.8.0-plan/M2/FS4/measured.md`, sections "iOS (iOS-host pass)" and "switch
label", iOS.
