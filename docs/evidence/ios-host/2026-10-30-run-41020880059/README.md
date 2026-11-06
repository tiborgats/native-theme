# iOS host, run 41020880059 (2026-10-30)

[The run](https://github.com/tiborgats/native-theme/actions/runs/41020880059): `os-probe.yml`'s ios-host job, the iOS
host's second round (the iOS lane's second pass), green on both images. The harness is `tools/ios-host` on
`v0.8.0-tools` (the lane read it at `f2671ec1`; the run's own commit is on the run's page): `run.sh` builds the
`IosHost` app and its `IosHostUITests` press pass with the image's default Xcode, boots a simulator of the newest iOS
runtime of the SDK's major version on the newest "iPhone <N> Pro", runs the render pass twice and the press pass twice,
compares the runs (`compare.swift`), solves each layer from the captures over opaque black and opaque white
(`solve.swift`), and runs the focus pass once on an iPad Pro. `IosHost/IosHost.swift`'s header says what each pass
draws: every `UIButton.Configuration` style at each `buttonSize`, and at medium each style with a role, a corner style,
an image (and its placement), a subtitle, the activity indicator, a menu and its indicator; the system button types;
bar button items in a UIToolbar; a navigation bar's back button in each display mode; UIPasteControl; AVRoutePickerView;
in seven states (normal, `isHighlighted` with no touch, disabled, a toggle off, on, on and highlighted, focused), over
the system background, black and white (the glass styles also over a pattern), light and dark, with UIKit's
animations off; the press pass holds a real touch (XCUITest, 4 s) and captures while `isHighlighted` is true. Before
the pages the app writes UIKit's own values to `run1/index.toml`: the 58 UIColor class colours under each appearance,
contrast and level, the text styles' fonts at every content size category and legibility weight, symbol image sizes,
caret rects, and the motion and behaviour values UIKit exposes.

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.8.2 (24H232) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPhone 16 Pro; iPad Pro (12.9-inch) (3rd generation) |
| `ios-host-macos-26/` | `ios-26` | macOS 26.6.2 (25G83) | Xcode 26.6 (17F113), iphonesimulator 26.5 (23F81a) | iOS 26.5 (23F77), iPhone 17 Pro; iPad Pro (12.9-inch) (3rd generation) |

Kept here, per image folder: `run1/index.toml` (the render pass's index: the environment, the screen, UIKit's values,
and each cell's configuration, title, view tree and frame), `solved/run1.toml` (the solve's summary per pair of
captures: its pixel counts, the channels' alpha spread, the five commonest solved colours), `press1/index.toml` and
`focus1/index.toml` (the press and focus passes' indexes), `differ-levels.txt` and `press-differ-levels.txt` (the files
that differ between each pass's two runs, by how much), the press pass's medium crops of the non-glass styles and the
render pass's `isHighlighted` crops they are compared with (`press1/screen/…`, `run1/screen/…`), the activity indicator specimens' crops over black and white from both render runs (`run1/screen/…`, `run2/screen/…`: the bezel is read only where the runs agree outside the indicator), and `analysis.toml`,
the output of `analysis/read_buttons.py` over them (its docstring is the method; it names each file it reads as
`input_image`). The rest of the run (the captures over black and white, the `drawHierarchy` drawings, the full
screenshots, the solved layer PNGs, the second runs, the SDK headers, the logs, about 1.2 GB) stays in the run's
artifacts until GitHub deletes them.

What the run's two passes show (each job's run.txt): the render pass's runs differ in 600 of 10,631 files on iOS 18
(308 outside `draw/`) and 1,497 of 15,871 on iOS 26 (875 outside `draw/`): the activity indicator, which never stops,
Liquid Glass, which settles on a different look in each run on iOS 26 (`differ-levels.txt`), and on iOS 26 a few
bar-item pixels by at most two levels; no crop of a non-glass configuration style without the indicator differs. On iOS 18 the press pass captured over the system background only, so its press
is compared, not solved. The facts use no solved Liquid Glass value (the solve's single-layer check fails on it), and
no reading of a crop that differs between the runs.

The SDK headers the run copied (its `headers/README.txt` lists each with its SHA-256) are the pinned Xcodes' own, so
the facts cite them as `xcode@16.4` and `xcode@26.6` sources (`docs/facts/sources.toml`), with the line numbers of
those copies; `os-probe.yml`'s macOS job checks such citations at that Xcode.
