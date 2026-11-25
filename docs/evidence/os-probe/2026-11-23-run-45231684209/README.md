# OS probe, run 45231684209 (2026-11-23)

The controls pass of `tools/os-probe` (`native-theme-os-probe --button-pass --controls`) on branch `m2-fs4c` at commit
`08b46ea5`: run 45226132792's pass with the toolbar sheet as wide as the controls sheet (900 pt; at 320 pt macOS 26 put
its item in the overflow menu), a target whose view is in no window reported as its error, and the scroll views'
styles read once the window is shown and set again; dispatched by hand through `.github/workflows/os-probe.yml` (jobs
`probe`, passes `controls`, images macOS 15 and 26): <https://github.com/tiborgats/native-theme/actions/runs/45231684209>.
The run is green: both probe jobs passed and the iOS host job was skipped. GitHub deletes the run's artifacts after 90
days; this folder keeps them whole, as the run uploaded them (PNGs byte for byte).

| Folder | Preset | Runner | Xcode | Rust |
|---|---|---|---|---|
| `os-probe-macos-15-controls/` | `macos-monterey` | macOS 15.8.2 (24H232), image 20261118.0550.1 | Xcode 16.4 | rustc 1.100.0 |
| `os-probe-macos-26-controls/` | `macos-tahoe` | macOS 26.7.2 (25G332), image 20261117.0516.1 | Xcode 26.6 | rustc 1.100.0 |

Both jobs captured: 410 PNGs each (82 solved layers), no capture `unavailable`. Both fixes took: the scroll view set
to the overlay style reads `NSScrollerStyleLegacy` when the window is shown and `NSScrollerStyleOverlay` after it is set
again (`scroll_views` in `probe.toml`; the system's `preferredScrollerStyle` stays legacy), and on macOS 26 the toolbar
sheet's button lies in the toolbar (`toolbar_sheet.targets.toolbar_button`, [835, 8, 57, 36]).

## What is here

The layout of run 45226132792's folder: per image `button-pass-controls/probe.toml`, the captures and the solved layers
under `button-pass-controls/<appearance>/controls/` and `button-pass-controls/<appearance>/controls_toolbar/`,
`environment.txt`, `environment/wallpaper.txt`; `controls.toml` and `controls-toolbar_sheet.toml`, the output of run
45226132792's `analysis/measure_controls.py` over the image folder (run from the repository root:
`python3 docs/evidence/os-probe/2026-11-23-run-45226132792/analysis/measure_controls.py [--sheet toolbar_sheet] docs/evidence/os-probe/2026-11-23-run-45231684209/<image folder>`).
