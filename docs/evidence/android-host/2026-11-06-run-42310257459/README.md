# Android host, run 42310257459 (2026-11-06): AH-pass's re-press and the all-hosts passes

[The run](https://github.com/tiborgats/native-theme/actions/runs/42310257459): `os-probe.yml`'s android-host job
(dispatched with `-f jobs=android-host`), green. The harness is `tools/android-host` on `v0.8.0-tools-hosts-m1b` at
`d959763b`, with the same build, emulator and system image as run
[42307873328](../2026-11-06-run-42307873328/README.md), whose MeasureTest and RippleTest files this run wrote again
(not kept here). The job's two runs wrote the same `button-group-*.csv`. Their `all-hosts.toml` files differ only in
the frame counts (by up to 8) and the elapsed times (by 1 ms).

- `measure/button-group-repress.csv`, `measure/button-group-release.csv` (U186, `ButtonGroupTest`): a light
  `ButtonGroup` of three Buttons with `animateWidth`, run on the test's virtual clock (16 ms frames), with each frame's
  three widths in px. When pressed, the middle button widens from 177 to 203 px in 8 frames, and its neighbours
  narrow to 164. In `release`, the button is released after 40 frames and runs back to 177 from the third frame after
  the release (frames 43-50). In `repress`, it is pressed again three frames after the release, at 201 px on its way
  back. It stays at 201 for two frames and is at 203 from the fourth. Released after 40 more frames, it runs back as in
  `release`. The widths are whole pixels, so a velocity at the re-press shows only to that resolution.
- `measure/all-hosts.toml` (U187, U190, `AllHostsTest`): MainActivity's all-hosts mode, on the real clock. It counts
  the frames Compose's frame clock delivers while an infinite animation runs, and logs the activity's lifecycle, its
  window focus and a filled button's interactions. A Tab key focuses the button (`interaction Focus`). When the
  notification shade opens, the window loses focus (`onWindowFocusChanged false`), the button emits no Unfocus, and
  the frames go on (45 in 2 s, against 90 in 1.5 s before). After Home, the activity pauses, loses window focus and is
  stopped about 1.2 s later. 44 frames came in those 2.5 s; the step's total does not tell whether any came after
  `onStop`. When brought back, it starts and resumes, and the frames go on (134 in 2.5 s).

Not kept (only on the run's page, for 90 days): run 2, the repeated MeasureTest and RippleTest files, CaptureTest's
PNGs, the logs and `index.toml`.
