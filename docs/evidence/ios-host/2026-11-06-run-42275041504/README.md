# iOS host, run 42275041504 (2026-11-06)

[The run](https://github.com/tiborgats/native-theme/actions/runs/42275041504): `os-probe.yml`'s ios-host job on
macos-26 only, with `ios_host_passes=measure`, `ios_host_measure=symbols` and
`ios_host_settings=bold_text,button_shapes`: a check that `run.sh`'s accessibility settings take effect. The harness is
`tools/ios-host` on `v0.8.0-tools-ios-m2` at `7bb1e784`. `run.sh` wrote com.apple.Accessibility's
`EnhancedTextLegibilityEnabled` and `ButtonShapesEnabled` (`simctl spawn defaults write`, exit 0, run.txt) after the
simulator's boot.

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-26/` | `ios-26` | macOS 26.6.2 (25G83) | Xcode 26.6 (17F113), iphonesimulator 26.5 (23F81a) | iOS 26.5 (23F77), iPhone 17 Pro |

Kept here: `measure1/index.toml`, whose `[environment]` records what UIAccessibility then reported to the app:
`bold_text = true`, `button_shapes = true`, `darker_system_colors = false`. The symbols pass's reads in it were made
under those settings; the facts take the symbols pass's values from run 42269041636, which ran without them.
