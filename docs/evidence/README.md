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

A citation links the run directory it reads, with the file and, for
`probe.toml`, the line (`evidence/os-probe/<run>/<image>/probe.toml#L29-L33`),
so each cell states which run it cites.

### The 0.8.0 measurement campaign's host runs (`v0.8.0-tools-hosts-m1b`)

The passes of the 0.8.0 measurement plan for the WinUI, Android and KDE hosts and the probe on `windows-11-arm`. Each
run directory's `README.md` says which register rows its files read, what each file holds and what was left on the
run's page; its `run.toml` maps its folders to presets.

| Run | Date | Commit | Why |
|-----|------|--------|-----|
| [`os-probe/2026-11-06-run-42307873328/`](os-probe/2026-11-06-run-42307873328/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42307873328)) | 2026-11-06 | `a4b2af0a` | WP-reads, the probe's half: the keyboard SPIs, the caret timeout, Mouse Vanish and the registry values behind them (U503, U505, U508) |
| [`winui-host/2026-11-06-run-42308526155/`](winui-host/2026-11-06-run-42308526155/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42308526155)) | 2026-11-06 | `7bad463f` | WP-reads' host half, WH-defaults, WH-motion, WH-captures and the all-hosts passes on the WinUI host |
| [`android-host/2026-11-06-run-42307873328/`](android-host/2026-11-06-run-42307873328/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42307873328)) | 2026-11-06 | `a4b2af0a` | AH-pass: font fallback, popup-menu attributes, focus visibility, the press ripple repeated (U554, U557, U564, U70, U71) |
| [`android-host/2026-11-06-run-42310257459/`](android-host/2026-11-06-run-42310257459/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42310257459)) | 2026-11-06 | `d959763b` | AH-pass's ButtonGroup re-press (U186) and the all-hosts passes on Android (U187, U190) |
| [`kde-host/2026-11-06-run-42307873328/`](kde-host/2026-11-06-run-42307873328/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42307873328)) | 2026-11-06 | `a4b2af0a` | KH-reads: Noto Sans 10's metrics, Kirigami's sizeForLabels, fontconfig's sorted matches (U400, U404) |
