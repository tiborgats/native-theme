# OS probe, run 45198750893 (2026-11-23)

What macOS 15 and macOS 26 drew for the controls other than the push buttons, at rest, hovered, pressed and
highlighted, in the controls pass of `tools/os-probe` (`native-theme-os-probe --button-pass --controls`): the
measurement M2.FS4 of the 0.8.0 plan lists for the preset rows no source states (the macOS part of its "no source"
rows). Dispatched by hand through `.github/workflows/os-probe.yml` on branch `m2-fs4c` (commit `c4f33eef`), with
`jobs=probe`, `passes=controls`, `images=["macos-15","macos-26"]`:
<https://github.com/tiborgats/native-theme/actions/runs/45198750893>. Both jobs green, the probe exited 0 on each.
GitHub deletes the run's artifacts after 90 days; this folder keeps them whole, as the run uploaded them (PNGs byte for
byte).

| Folder | Preset | Runner | Xcode | Rust |
|---|---|---|---|---|
| `os-probe-macos-15-controls/` | `macos-monterey` | macOS 15.8.2 (24H232) | Xcode 16.4 | rustc 1.100.0 |
| `os-probe-macos-26-controls/` | `macos-tahoe` | macOS 26.7.2 (25G332) | Xcode 26.6 | rustc 1.100.0 |

Each pass ran with the image's settings (`environment/`, and `[macos.environment]` of `probe.toml`).

## What is here

- `<image>/button-pass-controls/probe.toml`: the pass. `[macos.button_pass.controls]` `method` says what the window
  holds and how each state is taken; `targets.<t>` the part each target is, in the window frame's points from its top
  left; `tabs` the tab view's tab rects; per appearance (`aqua`, `darkAqua`) `normal`, `highlighted`, `hovered.<t>`
  and `pressed.<t>`, each with its captures, its solved layer (`layer.file`) and `diff_from_normal`.
- `<image>/button-pass-controls/<appearance>/controls/`: the captures (`<state>[-over-<backdrop>].png`) and the
  solved layers (`<state>-layer.png`).
- `<image>/button-pass-controls/environment.txt`: the job's build record.
- `<image>/controls.toml`: the output of `analysis/measure_controls.py`, run from the repository root with the image
  folder as its argument. Its docstring is its method.
