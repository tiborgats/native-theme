# Evidence

Recorded outputs that `docs/platform-facts.md` and the presets cite, kept in
the repository because GitHub deletes a workflow run's artifacts after 90
days. Each run directory holds that run's artifacts and a `README.md` of
its own that states the runner images and what the run reported. A recorded
file is not edited after it is added: a later reading is a new run directory.
A recorded file's own `docs/platform-facts.md:N` citations (the Windows
`probe.toml`s' notes, for one) refer to the facts file at that run's commit.

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
| [`2026-10-31-run-41195846264/`](os-probe/2026-10-31-run-41195846264/) ([run](https://github.com/tiborgats/native-theme/actions/runs/41195846264)) | 2026-10-31 | `18d2a5d8` on `v0.8.0-tools-probe-m2` | four macOS 26 passes for the `macos-tahoe` gaps of the 0.8.0 plan: each font's descriptor and Core Text reads (CSS weight, variation, features), the text styles' line heights, a selected table row's label colours, and the focus ring's dash, corners and transition (a screen recording, `analysis/measure_focus_ring.py`); macos-26 only |
| [`2026-11-06-run-42215651196/`](os-probe/2026-11-06-run-42215651196/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42215651196)) | 2026-11-06 | `20d56259` on `v0.8.0-tools-probe-m2` | run 41195846264's font, line-height, selected-row and controls passes on macOS 15 for `macos-monterey` (U236, U238, U144), with the facts lane's `read_fonts.py`, `measure_selection.py` and `read_controls.py` over them; macos-15 kept only |
| [`2026-11-06-run-42272240639/`](os-probe/2026-11-06-run-42272240639/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42272240639)) | 2026-11-06 | `4b84d361` on `v0.8.0-tools-probe-m2` | the 0.8.0 measurement plan's macOS-probe passes on macOS 26 (26.7.2) and 15: the symbol configurations and image sizes per place, the reads pass (timing functions, route button, plain-view focus ring, text rendering, fallbacks, pop-up and combo box geometry, mirroring), the button pass's default and image sheets, disabled-and-pressed state, patterned backdrop, offscreen 2x renders, key-not-main and main-not-key windows, the focus pass under the red accent, the caret recording, and Increase Contrast with the red accent |
| [`2026-11-06-run-42282133669/`](os-probe/2026-11-06-run-42282133669/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42282133669)) | 2026-11-06 | `2f4bca36` on `v0.8.0-tools-probe-m2` | the reads pass on macOS 26 with a right-to-left layout direction (the directional symbols' mirroring) and the plain view's focus ring with the exterior ring type; macos-26 only |

A citation links the run directory it reads, with the file and, for
`probe.toml`, the line (`evidence/os-probe/<run>/<image>/probe.toml#L29-L33`),
so each cell states which run it cites.
