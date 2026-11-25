# OS probe, run 45208260830 (2026-11-23)

The controls pass of `tools/os-probe` (`native-theme-os-probe --button-pass --controls`) again, on branch `m2-fs4c`
at commit `63257682` (the pass of run 45198750893 with every target's part read once the window is shown and settled,
the pop-up among the `highlight:` cases, the titled disclosure button widened to 160 pt with its title rect recorded,
and the switch in the fifth column), dispatched by hand through `.github/workflows/os-probe.yml`:
<https://github.com/tiborgats/native-theme/actions/runs/45208260830>. The run is green: both probe jobs passed and the
iOS host job was skipped. GitHub deletes the run's artifacts after 90 days; this folder keeps them whole, as the run
uploaded them (PNGs byte for byte).

| Folder | Preset | Runner | Xcode | Rust |
|---|---|---|---|---|
| `os-probe-macos-15-controls/` | `macos-monterey` | macOS 15.8.2 (24H232), image 20261118.0550.1 | Xcode 16.4 | rustc 1.100.0 |
| `os-probe-macos-26-controls/` | `macos-tahoe` | macOS 26.7.2 (25G332), image 20261117.0516.1 | Xcode 26.6 | rustc 1.100.0 |

**The macOS 26 job captured nothing.** Every one of its 248 captures is `capture = "unavailable: screencapture exited
with exit status: 1"` in its `probe.toml`, so it holds no PNG and every layer is `unavailable: the black or white
capture or its backdrop is missing`. Its step passed, as the workflow fails the step only on a build failure, a probe
exit or a missing `probe.toml`. The job ran on the same runner image as run 45198750893's macOS 26 job, which captured,
with the same grants; why `screencapture` failed is not recorded (the probe keeps its exit status, not its output). Its
`probe.toml` still holds what the pass reads without a capture: the controls, the targets' parts, the tabs, and the
key and cursor reads.

## What is here

The layout of run 45198750893's folder (see its README): per image `button-pass-controls/probe.toml`, the captures and
the solved layers under `button-pass-controls/<appearance>/controls/` (macOS 15 only), `environment.txt`,
`environment/wallpaper.txt`, and `controls.toml`, the output of `analysis/measure_controls.py` over the image folder.

`analysis/`:

- `measure_controls.py`: run 45205409285's script (that run's artifacts were not kept; its folder held only the
  script), with the label taken out of a translucent fill (`label`, `measure_button_layers.py`'s `colour_of`) in a
  title rect per target. Its docstring is its method. Run from the repository root:
  `python3 docs/evidence/os-probe/2026-11-23-run-45208260830/analysis/measure_controls.py <run>/<image folder> [OUTPUT]`.
- `run-45198750893-os-probe-macos-{15,26}-controls.toml`: the same script over run 45198750893's image folders, with
  OUTPUT set here, so that run's own `controls.toml` is not rewritten.
- `check_label_port.py`: runs `measure_button_layers.py` as run 41006842699 kept it (commit `5d48e80f`, read with `git
  show`) on the push button of a controls pass, its own `measure_button` unchanged, and compares its `label` with
  `measure_controls.py`'s; `check_label_port-*.txt`, its output over run 45198750893's two image folders and this run's
  macOS 15 folder: the same label and pixel count in every appearance and state (exit 0 each).
