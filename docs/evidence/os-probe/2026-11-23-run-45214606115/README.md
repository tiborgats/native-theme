# OS probe workflow, run 45214606115 (2026-11-23): the labelled switch and the DisclosureGroup on macOS

[The run](https://github.com/tiborgats/native-theme/actions/runs/45214606115): `os-probe.yml` (`jobs=probe,switch-label`,
`passes=symbols`, `images=["macos-15","macos-26"]`) on `m2-fs4d` at `53b749a8`. The switch-label job's host,
`tools/swiftui-host/SwitchLabel.swift`, draws what it drew in [run 45210835145](../2026-11-23-run-45210835145/README.md)
(items 34 pt tall in place of 40, so the window fits the screen) and, first, a collapsed SwiftUI
`DisclosureGroup("Label", isExpanded:)`, whose chevron and label it hovers and presses where the rest capture shows them
(`[[disclosure_parts]]`). The symbols pass ran only to try the workflow's new check that a macOS probe pass wrote a
capture (green on both images: macOS 26 wrote 4 of its 5 toolbar captures, one `screencapture` exited 1, which
`symbols/probe.toml` records); its output is not kept here.

| Folder | Preset | Runner | Xcode, SDK |
|---|---|---|---|
| `switch-label-macos-15/` | `macos-monterey` | macOS 15.8.2 | Xcode 16.4, macosx 15.5 (24F74) |
| `switch-label-macos-26/` | `macos-tahoe` | macOS 26.7.2 | Xcode 26.6, macosx 26.5 (25F70) |

Kept here, per image folder: the job's whole artifact and `measure.toml`, the output of

`python3 -I docs/evidence/os-probe/2026-11-23-run-45214606115/analysis/measure_switch_label.py docs/evidence/os-probe/2026-11-23-run-45214606115/switch-label-macos-15`

(and `…-macos-26`); its docstring gives the method, the DisclosureGroup's part included. Both jobs finished green,
`failures = []`, `notes = []`. The Toggle reads agree with run 45210835145's. The values are in
`.superpowers/sdd/v0.8.0-plan/M2/FS4/measured.md`, sections "switch label" and "expander (SwiftUI DisclosureGroup)".
