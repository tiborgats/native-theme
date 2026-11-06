# iOS host, run 42275053139 (2026-11-06)

[The run](https://github.com/tiborgats/native-theme/actions/runs/42275053139): `os-probe.yml`'s ios-host job with
`ios_host_passes=focus` and `ios_host_settings=full_keyboard_access`, the measurement campaign's IP-focus pass. The
harness is `tools/ios-host` on `v0.8.0-tools-ios-m2` at `7bb1e784`: `run.sh` writes com.apple.Accessibility's
`FullKeyboardAccessEnabled` (`simctl spawn defaults write`, exit 0 on both images, run.txt) on the iPad simulator after
its boot, then runs the app's focus mode once: each configuration style at medium and the system type, as UIKit makes
them and as `FocusableButton` (a UIButton subclass whose `canBecomeFocused` is true, the `+focusable_subclass`
specimens), normal and focused (a focus update requested to the button), over the system background, light and dark.

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.8.2 (24H232) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPad Pro (12.9-inch) (3rd generation) |
| `ios-host-macos-26/` | `ios-26` | macOS 26.6.2 (25G83) | Xcode 26.6 (17F113), iphonesimulator 26.5 (23F81a) | iOS 26.5 (23F77), iPad Pro (12.9-inch) (3rd generation) |

Kept here, per image folder: `focus1/index.toml` (the screen with `focus_system`, and a `[[render]]` per cell with
`can_become_focused`, `is_focused`, the configuration and the view tree) and `focus1/screen/<appearance>/<specimen>/
medium|default/system/{normal,focused}.png` (the crops). The `drawHierarchy` drawings and the screenshots stay in the
run's artifacts.

What the run shows: with Full Keyboard Access written, every button UIKit makes is `can_become_focused = true` and
`is_focused = true` in its focused cell, on both images (run 41020880059, without the setting, found them all false),
and its focused crop differs from its normal one. The `+focusable_subclass` specimens give the same reads. The pass
ran once (no second run to compare).
