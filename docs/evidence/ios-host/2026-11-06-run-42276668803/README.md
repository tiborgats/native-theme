# iOS host, run 42276668803 (2026-11-06)

[The run](https://github.com/tiborgats/native-theme/actions/runs/42276668803): `os-probe.yml`'s ios-host job with
`ios_host_passes=render` and `ios_host_device=ipad`, the measurement campaign's IP-ipad pass. The harness is
`tools/ios-host` on `v0.8.0-tools-ios-m2` at `8baca1fe`: `run.sh` boots the last "iPad Pro" the runtime lists instead
of the iPhone and runs the render pass (the base specimen set, as in run 41020880059) twice, compares and solves it.

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.8.2 (24H232) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPad Pro (12.9-inch) (3rd generation) |

Kept here, per image folder: `run1/index.toml` (its `[screen]` records `interface_idiom = 1`, the pad, and
`focus_system = true`), `solved/run1.toml` and `differ-levels.txt`. The crops, drawings, screenshots and second run stay
in the run's artifacts. The render pass's focused state was requested without Full Keyboard Access, as in run
41020880059; the focused state with it is run 42275053139.

What the macos-15 job shows (run.txt): both render runs finished `ok`; they differ in 620 of 10,631 files, 308 outside
`draw/`, all of them activity indicator captures, and compare.swift accepts them.

The macos-26 job (`ios-26`) was still running when this folder was committed (started 20:52 UTC, still in progress at
22:43 UTC); its folder is added when it finishes, or the pass is run again for it.
