# iOS host, run 42275075079 (2026-11-06)

[The run](https://github.com/tiborgats/native-theme/actions/runs/42275075079): `os-probe.yml`'s ios-host job with
`ios_host_passes=render,press` and `ios_host_settings=increase_contrast`, the measurement campaign's IP-a11y pass under
Increase Contrast. The harness is `tools/ios-host` on `v0.8.0-tools-ios-m2` at `7bb1e784`: `run.sh` turns Increase
Contrast on (`simctl ui <device> increase_contrast enabled`, exit 0, run.txt) after the simulator's boot, then runs the
render pass (the base specimen set, as in run 41020880059) and the press pass twice each, compares and solves them.

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.8.2 (24H232) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPhone 16 Pro |
| `ios-host-macos-26/` | `ios-26` | macOS 26.7.2 (25G332) | Xcode 26.6 (17F113), iphonesimulator 26.5 (23F81a) | iOS 26.5 (23F77), iPhone 17 Pro |

Kept here, per image folder: `run1/index.toml` (its `[motion]` records what UIAccessibility reported:
`darker_system_colors = true`), `solved/run1.toml`, `differ-levels.txt`, `press1/index.toml`,
`press-differ-levels.txt` and `solved/press1.toml`. The crops, drawings, screenshots and second runs stay in the run's
artifacts.

What the macos-15 job shows (run.txt): both render runs and both press runs finished `ok`; the render runs differ in 668
of 10,631 files, 352 outside `draw/`, all of them activity indicator captures (it turns without end), and compare.swift
accepts them; the press runs differ in one file, the index.

What the macos-26 job shows (run.txt; it reports a failure): all four runs finished `ok` with `darker_system_colors =
true`; the render runs differ in 1,471 of 15,871 files, 843 outside `draw/`, and compare.swift fails on one of them:
`screen/light/prominent_glass+image_star/medium/system/disabled.png`, 8,914 pixels by at most 3 levels (Liquid Glass
over the system background, past the glass tolerance of 2). The others are the activity indicator, Liquid Glass over
black, white or the pattern, bar items and back buttons, which compare.swift accepts. The facts read no crop of that
specimen and state from this run. The press runs differ in 6 files, 5 outside `draw/`, accepted (`--list-only`).
