# iOS host, run 40984711960 (2026-10-30)

[The run](https://github.com/tiborgats/native-theme/actions/runs/40984711960): `os-probe.yml`'s ios-host job (spike I
of the 0.8.0 plan), at commit `9b94ca3c9ea020c4faef7d4a7680c9cedccad1cf` on `v0.8.0-tools-spike-i` (merged into
`v0.8.0-tools` as `ea7f184a`). The harness is `tools/ios-host` at that commit: `run.sh` builds the `IosHost` app with
the image's default Xcode, boots a simulator of the newest iOS runtime of the SDK's major version on the newest
"iPhone <N> Pro", runs the app twice and compares the runs; `IosHost/IosHost.swift` draws a UIButton in each
`UIButton.Configuration` style at each `buttonSize` in five states (normal; `isHighlighted = true` without a touch;
disabled; `changesSelectionAsPrimaryAction` off; the same with `isSelected`), the title "Button", under the light
and the dark appearance, each in a 96 pt cell of `systemBackground`, with UIKit's animations off and two seconds to
settle, and records the simulator's own screenshot of each page (`xcrun simctl io screenshot`) cropped to each cell.

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.7.9 (24G830) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPhone 16 Pro |
| `ios-host-macos-26/` | `ios-26` | macOS 26.6.2 (25G83) | Xcode 26.6 (17F113), iphonesimulator 26.5 (23F81a) | iOS 26.5 (23F77), iPhone 17 Pro |

Kept here: each folder's `run1/index.toml` (the environment, the screen, and each cell's button frame and intrinsic
size) and `run1/screen/` (the 320 and 480 screenshot crops, 3 px per point, sRGB), and `analysis.toml`, the output of
`analysis/measure_buttons.py` over them (its docstring says what each value is). The run's second pass, the app's own
`drawHierarchy` drawings, the full-page screenshots and the logs stay in the run's artifacts (`ios-host-macos-15`,
`ios-host-macos-26`, until GitHub deletes them). The second pass matches the first: byte for byte on iOS 18; on iOS 26
the screenshots differ only in five `prominent_clear_glass` large crops, by at most one level in at most twelve
pixels (the job's `differ-levels.txt`).

The job's `environment.txt` printed the SDK header's styles and sizes (`grep -n` of the iphonesimulator SDK's
`UIKit.framework/Headers/UIButtonConfiguration.h`), which the state models quote: on Xcode 26.6, lines 23-26
(`UIButtonConfigurationSizeMedium,` … `UIButtonConfigurationSizeLarge,`), 78-81, 83-86 (the eight styles) and 88-91
(the four glass styles, `API_AVAILABLE(ios(26.0), tvos(26.0))`); on Xcode 16.4, lines 22-25, 77-80 and 82-85.

What the run cannot show (spike I's findings, the ledger's row I): `isHighlighted` draws nothing on `prominentGlass`
and dims only the title on `glass` and `clearGlass`, so the glass press needs a real touch; `mini` draws as `small` on
iPhone; clear glass needs content behind it to show; and every colour in a crop is the composite over the cell's
`systemBackground` (or the bezel), so a translucent fill needs a capture over a second background.
