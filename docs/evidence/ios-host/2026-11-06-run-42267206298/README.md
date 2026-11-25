# iOS host, run 42267206298 (2026-11-06)

[The run](https://github.com/tiborgats/native-theme/actions/runs/42267206298): `os-probe.yml`'s ios-host job with
`ios_host_passes=render` and `ios_host_set=campaign`, the measurement campaign's IP-states, IP-sizes, IP-system and
IP-popup passes. The harness is `tools/ios-host` on `v0.8.0-tools-ios-m2` at `1f30fc2e`: the render pass as in run
41020880059, drawing the campaign set (`IosHost.swift`, `campaignSpecimens`): every configuration style and the system
type disabled while selected and while highlighted (`+states_disabled_combinations`); the corner styles on filled,
gray and glass at mini, small and large, and `dynamic` on the other glass styles at every size; an image, an image
alone, a subtitle and the primary role on every style at mini, small and large; the pop-up indicator on filled, gray,
plain and glass at those sizes; the system type with an image, an image alone and a menu; a pop-up button (its menu
changing the selection) under each of two selections at its own width and at 200 and 60 points.

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.8.2 (24H232) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPhone 16 Pro |
| `ios-host-macos-26/` | `ios-26` | macOS 26.7.2 (25G332) | Xcode 26.6 (17F113), iphonesimulator 26.5 (23F81a) | iOS 26.5 (23F77), iPhone 17 Pro |

Kept here, per image folder: `run1/index.toml` (each cell's configuration, title, view tree and frame),
`solved/run1.toml` (the solve's summary per pair over black and white) and `differ-levels.txt`. The crops, drawings,
screenshots, solved layers and second run (about 670 MB) stay in the run's artifacts until GitHub deletes them.

What the run shows (each job's run.txt): on iOS 18 the two runs are byte-identical (13,159 files); on iOS 26 they differ
in 678 of 20,421 files, 356 outside `draw/` (all of the glass and prominent glass specimens: Liquid Glass, as in run
41020880059), and compare.swift
accepts them. Both jobs report a failure: on pages of the pop-up buttons (`+pop_up_selected_*`) the button, 200 points
wide or grown with the longer selection, overflowed its third of the screen into the page margins, so the margin check
failed after 5 retakes (sync/done lists the pages; the pages were kept). The facts take no pop-up reading from this run:
`IosHost.swift` after `8baca1fe` draws those buttons one cell per row, and the pop-up pass is its own run
(`ios_host_set=popup`). Every other specimen's pages passed the checks.

## What this branch keeps

This branch keeps only the files the facts cite, the files the committed analyses read, and those analyses
(`analysis/`) with their outputs. The full run stays in CI's artifacts until GitHub deletes them;
and the files the iOS host's branch committed for it stay on `v0.8.0-tools-ios-m2`, under this folder's path. Kept here:

- `analysis/render_read_buttons.py`
- `ios-host-macos-15/differ-levels.txt`
- `ios-host-macos-15/render_read_buttons.toml`
- `ios-host-macos-15/run1/index.toml`
- `ios-host-macos-15/solved/run1.toml`
- `ios-host-macos-26/differ-levels.txt`
- `ios-host-macos-26/render_read_buttons.toml`
- `ios-host-macos-26/run1/index.toml`
- `ios-host-macos-26/solved/run1.toml`
- `run.toml`
