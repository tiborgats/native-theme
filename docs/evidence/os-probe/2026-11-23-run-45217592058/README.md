# OS probe, run 45217592058 (2026-11-23)

The controls pass of `tools/os-probe` (`native-theme-os-probe --button-pass --controls`) on macOS 26 only, on branch
`m2-fs4c` at commit `39ce861d` (the pass of run 45208260830, unchanged), dispatched by hand through
`.github/workflows/os-probe.yml` to repeat the macOS 26 job of run 45208260830, which captured nothing:
<https://github.com/tiborgats/native-theme/actions/runs/45217592058>. Its first attempt's macOS 26 job was never given
a runner; the second attempt (`gh run rerun --failed`) is green. GitHub deletes the run's artifacts after 90 days; this
folder keeps them whole, as the run uploaded them (PNGs byte for byte).

| Folder | Preset | Runner | Xcode | Rust |
|---|---|---|---|---|
| `os-probe-macos-26-controls/` | `macos-tahoe` | macOS 26.7.2 (25G332), image 20261117.0516.1 | Xcode 26.6 | rustc 1.100.0 |

**The job captured.** It holds 310 PNGs (155 per appearance, 62 of them solved layers), as many as run 45208260830's
macOS 15 job, and no capture in its `probe.toml` is `unavailable`.

## What is here

The layout of run 45208260830's folder (see its README): `button-pass-controls/probe.toml`, the captures and the solved
layers under `button-pass-controls/<appearance>/controls/`, `environment.txt`, `environment/wallpaper.txt`, and
`controls.toml`, the output of run 45208260830's `analysis/measure_controls.py` over the image folder (run from the
repository root: `python3 docs/evidence/os-probe/2026-11-23-run-45208260830/analysis/measure_controls.py
docs/evidence/os-probe/2026-11-23-run-45217592058/os-probe-macos-26-controls
docs/evidence/os-probe/2026-11-23-run-45217592058/os-probe-macos-26-controls/controls.toml`).

`analysis/`:

- `compare_reads.py`: two `controls.toml` files compared key by key (drawn box, fill, label, ink, edges, whether a
  hover changed nothing); `compare_reads-run-45198750893-macos-26.txt`, run 45198750893's macOS 26 reads (as re-read by
  run 45208260830's `analysis/run-45198750893-os-probe-macos-26-controls.toml`) against this run's: 80 of the 104 keys
  of both read the same; the 24 that differ are the targets whose part moved or grew between the two commits (the
  disclosure button, widened; the switch, moved to the fifth column; the toolbar button, its part now the whole 36 px
  view, its fill, label and ink the same) and the slider knob, which run 45198750893 read as an error (its part lay
  outside the view).
- `diff_layers.py`: the pixels of a region that differ between two solved layers; `diff_layers-slider_knob.txt`, the
  normal layer against the pressed slider's, per appearance, over the slider's view grown by 6 pt, the knob's part and
  one column of the track's filled part.
