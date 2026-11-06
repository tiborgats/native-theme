# iOS host, run 42275028573 (2026-11-06)

[The run](https://github.com/tiborgats/native-theme/actions/runs/42275028573): `os-probe.yml`'s ios-host job with
`ios_host_passes=measure` and `ios_host_measure=behavior`, the measurement campaign's IP-behavior pass, second run. The
harness is `tools/ios-host` on `v0.8.0-tools-ios-m2` at `7bb1e784`; what it draws and reads is as in
[run 42268160866](../2026-11-06-run-42268160866/README.md), with the two limits stated there removed: each caret sample
records, per cursor view, every sublayer's presentation opacity and its animations' begin times (the blink runs on the
first sublayer, key `_uitcvba`), and the text field's and the read-only text view's cursor views are read while each is
first responder (`[[scene_read]]` `caret_text_field` and `caret_read_only_focused`).

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.8.2 (24H232) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPhone 16 Pro |
| `ios-host-macos-26/` | `ios-26` | macOS 26.7.2 (25G332) | Xcode 26.6 (17F113), iphonesimulator 26.5 (23F81a) | iOS 26.5 (23F77), iPhone 17 Pro |

Kept here, per image folder: `measure1/index.toml` (the environment, `[[button_behavior]]`, `[behavior_defaults]`, a
`[[scene_read]]` per scene and per caret read, a `[[capture]]` per crop), `measure1/screen/…` (the crops) and
`measure-differ-levels.txt`. The screenshots, the second run and the logs stay in the run's artifacts.

What the run shows (each job's run.txt): both runs finished `ok` on both images. Between the runs the crops differ by at
most one level but the caret's (the blink) and, on iOS 26, the navigation push's: in the first run the push had not
completed when the scene was read and captured 2.5 s after it started (`push_completed_after_s` absent), in the second
it completed after 2.0 s; on iOS 18 it completed after 0.53 and 0.52 s. The transition coordinator's reads
(`animated`, `duration_s` 0.35, `completion_curve` 7) are the same in all four runs. The indexes differ otherwise only
in times (the samples' times, the blink animations' begin times).
