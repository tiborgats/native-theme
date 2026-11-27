# iOS host, run 45516795085 (2026-11-25): the list set on iOS 18 and iOS 26, every specimen pressed

[The run](https://github.com/tiborgats/native-theme/actions/runs/45516795085): `os-probe.yml`'s ios-host job (the
other jobs skipped) on `m2-fs4d` at `fb04b1fb`, with the passes `render,press`, the specimen set `lists`, the iPhone
simulator and no settings. The harness is this repository's `tools/ios-host` at `fb04b1fb`, which draws what it drew in
[run 45230061151](../2026-11-23-run-45230061151/README.md) and now (review RV-fs4d, should-fix 3 and 4, minor):

- puts the armed press's identifier (`press-<n>`) on a marker view over the press target (no background, no user
  interaction, an accessibility element), not on the target, and `IosHostUITests/PressTests.swift` presses at the
  element's centre as a coordinate (`coordinate(withNormalizedOffset: (0.5, 0.5)).press(forDuration:)`), held 8 s
  (4 s in run 45512794102 let the touch end before the screenshot in 11 and 40 presses on iOS 18.6);
- presses SwiftUI's `Toggle` in a `Form` again (at its `UISwitch`, the touch reported by the hosting view);
- lists a skipped press as a failure as well as in `[[press_skipped]]`;
- writes `"absent"` for a read UIKit does not give (`highlighted_at_capture` of the segmented control,
  `tracking_at_capture` of a view that is not a button), leaves `selected_at_screenshot` out where nothing has a
  selection to read, and gives SwiftUI's list cells no `default_*` reads;
- keeps XCUIAutomation's `XCUIElement.h` and `XCUICoordinate.h` (Xcode 16's XCTest headers only import them).

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15 (`environment.txt`) | Xcode 16.4 | iOS 18.6 (22G86), iPhone 16 Pro |
| `ios-host-macos-26/` | `ios-26` (parked) | macOS 26 (`environment.txt`) | Xcode 26.6 | iOS 26.5 (23F77), iPhone 17 Pro |

Kept here, per image, from the job's artifact: `environment.txt`, `run.txt`, `differ.txt`, `differ-levels.txt`,
`press-differ.txt`, `press-differ-levels.txt`, `simctl-io-help.txt`, `run1/index.toml`, `press1/index.toml`,
`solved/run1.toml`, `solved/press1.toml`, `headers/README.txt` and `headers/<sdk>/*XCUI*.h`, and `CAPTURES.sha256` (the
SHA-256 and path of every PNG of the artifact: the drawn images, the pages, the crops and the solved layers; the PNG
files stay in the run's artifact and are regenerable from the commit). `measure_lists.toml` and `changed_parts.toml` are
the outputs of run 45230061151's `analysis/measure_lists.py` and `analysis/changed_parts.py` (their docstrings give the
method) over each artifact folder:

`python3 -I docs/evidence/ios-host/2026-11-23-run-45230061151/analysis/measure_lists.py <artifact folder>`

(those scripts now say "not measured" where an index or a solved layer is missing, and `measure_lists.py` writes the
label's ink box as a TOML array; run 45230061151's `measure_lists.toml` was written again with it, the same but for
that array).

What the run shows:

- Both jobs finished green: `done: ok` in every render and press run, no crash report, every list specimen (the table
  row, the middle segment, the sidebar row, the middle tab bar item, the slider's thumb, the outline header, the list
  row's switch and the SwiftUI `Toggle`'s switch) pressed in light and dark over the three backdrops in both press runs
  (48 pressed renders each). iOS 18.6: one retake in the first press run, none in the second; the render runs
  byte-identical; the press runs differ in one file. iOS 26.5: no retake; the render runs differ in 2 of 205 files and
  the press runs in 2 of 145 (`differ.txt`, `press-differ.txt`).
- iOS 18.6 (`ios-13`): every read run 45230061151's outputs also have is equal (`measure_lists.toml`,
  `changed_parts.toml`), except the reads the host changed (the segment's `highlighted_at_capture`, `switch_form`'s
  `default_*`). The SwiftUI `Toggle` pressed: its changed pixels lie in the switch (`changed_parts.toml`
  `press1."<a>".switch_form.pressed` `changed_box_pt` from x 256.333 pt), its label equals the UIKit row's at rest and
  pressed within 3 levels, and its `UISwitch` reports `isHighlighted` false at the capture.
- `hover` is declared for iOS 15.0 and later on `XCUIElement` and `XCUICoordinate` in both SDKs
  (`headers/iphonesimulator/XCUIAutomation-XCUIElement.h`, `…-XCUICoordinate.h`); `simctl io` has no pointer
  operation. No hover pass ran.

The values are in `.superpowers/sdd/v0.8.0-plan/M2/FS4/measured.md`, sections "iOS (iOS-host pass)" and "switch
label", iOS.

## The SwiftUI `Toggle`'s pressed label on iOS 26.5, read again (M2.FS4c10)

Run 45230061151's `measure_lists.py` found each state's label ink box in that state's own capture, left of the
`UISwitch`'s frame. On iOS 26.5 the pressed switch's thumb grows left past that frame, so the pressed states' boxes took
the thumb too (`label_ink_box_px` [52, 29, 705, 148] and [52, 29, 711, 148] in light, 136 high in dark, where the label
is [52, 59, 169, 98]), and the `Toggle`'s pressed label compared equal to no UIKit state. `analysis/measure_lists.py`
is that script with one change: a state other than normal crops its label at the normal state's box (same run,
appearance and specimen). Run on both artifacts (downloaded with `gh run download 45516795085`): iOS 18.6's output is
byte for byte the committed one; iOS 26.5's differs only in those four boxes (now [52, 59, 169, 98]) and in
`press1.<appearance>.switch_form.{normal,pressed}.label_vs_uikit_row`, now `['normal', 'pressed']` in both appearances:
the SwiftUI `Toggle`'s label, at rest and pressed, equals the UIKit list row's at rest and pressed within 3 levels.
`ios-host-macos-26/measure_lists.toml` is the new output.
