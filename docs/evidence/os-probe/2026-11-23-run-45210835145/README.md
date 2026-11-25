# OS probe workflow, run 45210835145 (2026-11-23): the labelled switch on macOS

[The run](https://github.com/tiborgats/native-theme/actions/runs/45210835145): `os-probe.yml`'s switch-label job
(`jobs=ios-host,switch-label`, `images=["macos-15","macos-26"]`) on `m2-fs4d` at `70c80753`: SwiftUI's
`Toggle("Label", isOn:)` with `.toggleStyle(.switch)`, built from `tools/swiftui-host/SwitchLabel.swift` (its header
says what it draws and how it hovers and presses) and run in a key window of an active application, in aqua and
darkAqua. The same run's ios-host job is kept in `../../ios-host/2026-11-23-run-45210835145/`.

| Folder | Preset | Runner | Xcode, SDK |
|---|---|---|---|
| `switch-label-macos-15/` | `macos-monterey` | macOS 15.8.2 (24H232) | Xcode 16.4, macosx 15.5 (24F74) |
| `switch-label-macos-26/` | `macos-tahoe` | macOS 26.7.2 (25G332) | Xcode 26.6, macosx 26.5 (25F70) |

Kept here, per image folder: the job's whole artifact (`switch-label.toml`: the environment, the system colours, each
item's frame and view tree, each capture; `<appearance>/<state>.png`; `environment.txt`, `build.log`, `run.log`) and
`measure.toml`, the output of

`python3 -I docs/evidence/os-probe/2026-11-23-run-45210835145/analysis/measure_switch_label.py docs/evidence/os-probe/2026-11-23-run-45210835145/switch-label-macos-15`

(and `…-macos-26`); its docstring gives the method. Both jobs finished green, `failures = []`, `notes = []`. On macOS 15
the window is 681 pt tall on a 768 pt screen, so the last item (`label_field_disabled`) lies partly below it and is not
read; on macOS 26 likewise (674 pt). The values are in `.superpowers/sdd/v0.8.0-plan/M2/FS4/measured.md`, section
"switch label".
