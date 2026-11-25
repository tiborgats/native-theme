# iOS host, run 42269041636 (2026-11-06)

[The run](https://github.com/tiborgats/native-theme/actions/runs/42269041636): `os-probe.yml`'s ios-host job with
`ios_host_passes=measure` and `ios_host_measure=roles,fonts,symbols`, the measurement campaign's first iOS passes
(IP-roles, IP-fonts, IP-symbols and IP-default-size of the campaign's plan). The harness is `tools/ios-host` on
`v0.8.0-tools-ios-m2` at `3fc2d9bf`: `run.sh` builds the `IosHost` app with the image's default Xcode, boots a
simulator of the newest iOS runtime of the SDK's major version on the newest "iPhone <N> Pro", and runs the app's
measure mode (`IosHost/Measure.swift`; its header says what each pass reads and draws) twice, compares the two runs
(`compare.swift --list-only`) and solves the crops captured over opaque black and white (`solve.swift`).

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.8.2 (24H232) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPhone 16 Pro |
| `ios-host-macos-26/` | `ios-26` | macOS 26.6.2 (25G83) | Xcode 26.6 (17F113), iphonesimulator 26.5 (23F81a) | iOS 26.5 (23F77), iPhone 17 Pro |

Kept here, per image folder:
- `measure1/index.toml`: the environment; what UIKit reports before any scene
  - `[[active_appearance_color]]`: every UIColor class colour (and `tintColor`) resolved with `activeAppearance`
    active and inactive, in light and dark;
  - `[[list_background]]`, `[[list_content]]`: the list background and content configurations updated for normal,
    selected, highlighted, selected and highlighted, disabled, selected and disabled, focused;
  - `[[font_style]]`: each text style, the extra large titles among them, at the large category, and its tight, loose,
    bold and italic variants: the font, the descriptor's attributes, the variation axes and values, the feature
    settings and (once per font name) the features;
  - `[font_sizes]`, `[[font_role]]`: UIKit's font size class values and the fonts of the class methods, the designs,
    the widths and UIKit's own controls;
  - `[[cascade]]`, `[[font_for_string]]`: the body font's default cascade list per language, and the font CoreText
    picks per script;
  - `[[configuration_default]]`: each configuration style's values as a new configuration states them (`buttonSize`
    among them);
  - `[[button_symbol]]`: each style's image, image alone, pop-up indicator and pop-up button at every `buttonSize`, and
    the system types: the current image (its description names the symbol), its size and symbol configurations, the
    image views' frames and configurations;
  
  then a `[[scene_read]]` per scene (its reads and `[[scene_read.view]]` view trees) and a `[[capture]]` per crop (its
  rect, whether two screenshots agreed, and the crop's pixels counted by chroma);
- `measure1/screen/<appearance>/<scene>/<part>/<backdrop>/<region>.png`: the crops of the simulator's screenshots;
- `solved/measure1.toml`: the solve's summary per pair of crops over black and white;
- `measure-differ-levels.txt`: the files that differ between the two runs, by how much.

The screenshots (`pages/`), the second run, the solved layer PNGs, the SDK headers and the logs stay in the run's
artifacts until GitHub deletes them.

The scenes (light and dark each): the tint normal and dimmed (`tintAdjustmentMode`), a row of a plain and a filled
button and a switch; the same row with `traitOverrides.activeAppearance` active and inactive; a label, a text field, a
segmented control, a switch, a slider and a stepper, enabled and disabled; a UICollectionView list in each appearance
(plain, grouped, inset grouped, sidebar, sidebar plain) with a cell normal, selected, highlighted, disabled and selected
and disabled (a list cell subclass whose configuration state is disabled); a text view first responder with 8
characters selected, over each backdrop, and three that are not first responder with the same selection; a text view
with UIFindInteraction's navigator up and "find" searched, over each backdrop; an alert, an action sheet, a popover (its
adaptive presentation style `none`) and a medium-detent sheet over each backdrop, a row of buttons behind them; text
in the body, caption 1 and large title styles; the configuration styles' title and subtitle labels, an overlong word
and long titles in a button at most 100 points wide; the controls that place a symbol of their own.

What the run shows (each job's run.txt): both runs of the measure mode finished `ok` on both images. The two runs
differ outside `pages/` and `index.toml` only by at most two levels in a crop on iOS 18; on iOS 26 also in the
`button_titles` scene's glass and prominent glass crops (Liquid Glass adapts to its backdrop, as in run 41020880059).
The two runs' indexes are identical on iOS 18 and differ on iOS 26 only in the crop statistics of those crops. The macos-15 job reports a failure:
`run.sh`'s wait on the app's process found the process ended once the app had written `done` (`ok`) and the first
run's output was kept whole; `run.sh` checks `done` before that failure since. The facts read no crop that differs
between the runs by more than a level.

A limit of this run: `Measure.swift` at `3fc2d9bf` wrote the numbers inside dictionaries and arrays (the variation
axes, the variations, the feature settings and features, the attributed text's attributes) with six significant
digits, so the axis identifiers (four-character codes, `wdth` = 2003072104) are rounded there (2003070000); the axes'
names, minimums, defaults and maximums are whole. The next fonts run writes NSNumber's own decimal form.

## What this branch keeps

This branch keeps only the files the facts cite, the files the committed analyses read, and those analyses
(`analysis/`) with their outputs. The full run stays in CI's artifacts until GitHub deletes them;
and the files the iOS host's branch committed for it stay on `v0.8.0-tools-ios-m2`, under this folder's path. Kept here:

- `analysis/roles_text_highlights.py`
- `analysis/symbols_read.py`
- `ios-host-macos-15/measure-differ-levels.txt`
- `ios-host-macos-15/measure1/index.toml`
- `ios-host-macos-15/measure1/screen/dark/find/default/black/text.png`
- `ios-host-macos-15/measure1/screen/dark/find/default/white/text.png`
- `ios-host-macos-15/measure1/screen/dark/text_selection/focused/black/text.png`
- `ios-host-macos-15/measure1/screen/dark/text_selection/focused/white/text.png`
- `ios-host-macos-15/measure1/screen/dark/text_selection/unfocused/black/text.png`
- `ios-host-macos-15/measure1/screen/dark/text_selection/unfocused/white/text.png`
- `ios-host-macos-15/measure1/screen/light/find/default/black/text.png`
- `ios-host-macos-15/measure1/screen/light/find/default/white/text.png`
- `ios-host-macos-15/measure1/screen/light/text_selection/focused/black/text.png`
- `ios-host-macos-15/measure1/screen/light/text_selection/focused/white/text.png`
- `ios-host-macos-15/measure1/screen/light/text_selection/unfocused/black/text.png`
- `ios-host-macos-15/measure1/screen/light/text_selection/unfocused/white/text.png`
- `ios-host-macos-15/roles_text_highlights.toml`
- `ios-host-macos-15/solved/measure1.toml`
- `ios-host-macos-15/symbols_read.toml`
- `ios-host-macos-26/measure-differ-levels.txt`
- `ios-host-macos-26/measure1/index.toml`
- `ios-host-macos-26/measure1/screen/dark/find/default/black/text.png`
- `ios-host-macos-26/measure1/screen/dark/find/default/white/text.png`
- `ios-host-macos-26/measure1/screen/dark/text_selection/focused/black/text.png`
- `ios-host-macos-26/measure1/screen/dark/text_selection/focused/white/text.png`
- `ios-host-macos-26/measure1/screen/dark/text_selection/unfocused/black/text.png`
- `ios-host-macos-26/measure1/screen/dark/text_selection/unfocused/white/text.png`
- `ios-host-macos-26/measure1/screen/light/find/default/black/text.png`
- `ios-host-macos-26/measure1/screen/light/find/default/white/text.png`
- `ios-host-macos-26/measure1/screen/light/text_selection/focused/black/text.png`
- `ios-host-macos-26/measure1/screen/light/text_selection/focused/white/text.png`
- `ios-host-macos-26/measure1/screen/light/text_selection/unfocused/black/text.png`
- `ios-host-macos-26/measure1/screen/light/text_selection/unfocused/white/text.png`
- `ios-host-macos-26/roles_text_highlights.toml`
- `ios-host-macos-26/solved/measure1.toml`
- `ios-host-macos-26/symbols_read.toml`
- `run.toml`
