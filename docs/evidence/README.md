# Evidence

Recorded outputs that `docs/platform-facts.md` and the presets cite, kept in
the repository: GitHub deletes a workflow run's artifacts after 90 days, and a
run by hand leaves no record anywhere else. Each run directory holds that
run's artifacts and a `README.md` of its own that states what produced them
(the runner images, or the machine's versions for a run by hand) and what the
run reported. A recorded file is not edited after it is added: a later
reading is a new run directory. A recorded file's own
`docs/platform-facts.md:N` citations (the Windows `probe.toml`s' notes, for
one) refer to the facts file at that run's commit.

## `os-probe/`

What each GitHub runner image's OS reported to `tools/os-probe`
(`native-theme-os-probe`, never published), dispatched by hand through
`.github/workflows/os-probe.yml`: one directory per image, holding
`probe.toml` and, on macOS, the control renders under `renders/aqua/` and
`renders/darkAqua/` (and, from run 40320895749, the key window's captures under
`renders/key_window/` and one directory per extra pass).

| Run | Date | Commit | Why |
|-----|------|--------|-----|
| [`2026-10-11-run-38129819806/`](os-probe/2026-10-11-run-38129819806/) ([run](https://github.com/tiborgats/native-theme/actions/runs/38129819806)) | 2026-10-11 | `05f6c11d` on `v0.7.0-rc1` | the platform-fidelity review, M2 Step 3 of `docs/archive/todo_v0.7.0_code-review-plan.md` |
| [`2026-10-14-run-38478334574/`](os-probe/2026-10-14-run-38478334574/) ([run](https://github.com/tiborgats/native-theme/actions/runs/38478334574)) | 2026-10-14 | `5ce19fcd` on `v0.7.0-rc1` | two more readings, the macOS table header height and the Windows hover time (§6 of the GPUI Kit 0.7.3 specification); every value the first run reported is the same |
| [`2026-10-26-run-40320895749/`](os-probe/2026-10-26-run-40320895749/) ([run](https://github.com/tiborgats/native-theme/actions/runs/40320895749)) | 2026-10-26 | `96199f83` on `v0.7.0-rc1` | a key window on macOS (the sidebar material, the title bar, the list header and the selected segment, with a calibration strip) and passes with Reduce Transparency off, Increase Contrast on and the probe as an app bundle (the D round of `docs/archive/todo_v0.7.0_code-review-plan.md`); every value the previous run reported is the same |
| [`2026-10-26-run-40324458365/`](os-probe/2026-10-26-run-40324458365/) ([run](https://github.com/tiborgats/native-theme/actions/runs/40324458365)) | 2026-10-26 | `891e66dd` on `v0.7.0-rc1` | three more macOS readings: the window when it is not key, the passes with wallpaper tinting off, and the wallpaper itself (solid black on every image); every value the previous run reported is the same |
| [`2026-10-26-run-40348854007/`](os-probe/2026-10-26-run-40348854007/) ([run](https://github.com/tiborgats/native-theme/actions/runs/40348854007)) | 2026-10-26 | `d14ce711` on `v0.7.0-rc1` | disabled and enabled switches and sliders in the macOS key window, disabled renders of the push button, checkbox, radio, switch and slider, and a pass with a red accent (D10, D12 of the D round); every value the previous run reported is the same |

A citation links the run directory it reads, with the file and, for
`probe.toml`, the line (`evidence/os-probe/<run>/<image>/probe.toml#L29-L33`),
so each cell states which run it cites.

## `breeze-offscreen/`

What the installed Breeze style painted when Qt drew its widgets offscreen, read back
from the pixels, and how Qt and the egui showcase size a font on a private X server with
`Xft.dpi` set; runs by hand, one directory per date, with the scripts and their printed
output. Breeze's colour scheme files are not copied; each README says how to rebuild
the configuration the run read.

| Run | Date | Why |
|-----|------|-----|
| [`2026-11-13/`](breeze-offscreen/2026-11-13/) | 2026-11-13 | issue #4's second comment ([rationale](../issues/0004_kde-hand-overrides-rationale.md) §9): the menu's, the list's and the scroll bar's colours, the header, the grid, the default button, the menu's metrics and the message icon in the facts' KDE cells |
| [`2026-11-14/`](breeze-offscreen/2026-11-14/) | 2026-11-14 | the readings 2026-11-13 did not save: the combo box popup, Qt's scale on X11, and the egui showcase's text before and after the font DPI of 0.7.1 |

A citation links the file and its line (`evidence/breeze-offscreen/2026-11-13/out_light.txt#L27`).
