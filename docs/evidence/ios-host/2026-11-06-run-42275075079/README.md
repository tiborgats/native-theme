# iOS host, run 42275075079 (2026-11-06)

[The run](https://github.com/tiborgats/native-theme/actions/runs/42275075079): `os-probe.yml`'s ios-host job with
`ios_host_passes=render,press` and `ios_host_settings=increase_contrast`, the measurement campaign's IP-a11y pass under
Increase Contrast. The harness is `tools/ios-host` on `v0.8.0-tools-ios-m2` at `7bb1e784`: `run.sh` turns Increase
Contrast on (`simctl ui <device> increase_contrast enabled`, exit 0, run.txt) after the simulator's boot, then runs the
render pass (the base specimen set, as in run 41020880059) and the press pass twice each, compares and solves them.

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.8.2 (24H232) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPhone 16 Pro |

Kept here, per image folder: `run1/index.toml` (its `[motion]` records what UIAccessibility reported:
`darker_system_colors = true`), `solved/run1.toml`, `differ-levels.txt`, `press1/index.toml`,
`press-differ-levels.txt` and `solved/press1.toml`. The crops, drawings, screenshots and second runs stay in the run's
artifacts.

What the macos-15 job shows (run.txt): both render runs and both press runs finished `ok`; the render runs differ in 668
of 10,631 files, 352 outside `draw/`, all of them activity indicator captures (it turns without end), and compare.swift
accepts them; the press runs differ in one file, the index.

The macos-26 job (`ios-26`) was still running when this folder was committed (started 20:01 UTC, still in progress at
22:43 UTC); its folder is added when it finishes, or the pass is run again for it.
