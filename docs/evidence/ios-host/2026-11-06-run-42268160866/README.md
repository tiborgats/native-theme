# iOS host, run 42268160866 (2026-11-06)

[The run](https://github.com/tiborgats/native-theme/actions/runs/42268160866): `os-probe.yml`'s ios-host job with
`ios_host_passes=measure` and `ios_host_measure=behavior`, the measurement campaign's IP-behavior pass. The harness is
`tools/ios-host` on `v0.8.0-tools-ios-m2` at `a9c03143`: `run.sh` builds the `IosHost` app with the image's default
Xcode, boots a simulator of the newest iOS runtime of the SDK's major version on the newest "iPhone <N> Pro", and runs
the app's measure mode (`IosHost/Measure.swift`, the `behavior` pass) twice and compares the runs
(`compare.swift --list-only`).

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.8.2 (24H232) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPhone 16 Pro |
| `ios-host-macos-26/` | `ios-26` | macOS 26.7.2 (25G332) | Xcode 26.6 (17F113), iphonesimulator 26.5 (23F81a) | iOS 26.5 (23F77), iPhone 17 Pro |

Kept here, per image folder:
- `measure1/index.toml`: the environment;
  - `[[button_behavior]]`: each configuration style's button, the system type, close and info light: the pointer
    interaction, hover style, interactions, tool tip, spring loading, behavioral styles, role, menu values, layout
    margins, alignments, touch flags, content size category limits, large content viewer, accessibility traits, the
    intrinsic size and `touch_reach_pt` (how many whole points past each edge `point(inside:with:)` still answers true,
    probed at half a point past each whole point from 0 to 60; -1 is false at 0.5 points);
  - `[behavior_defaults]`: the stepper's autorepeat, continuity and wrapping, a new navigation item's back button
    display mode and title, the key window's layer speed;
  - a `[[scene_read]]` per scene: the navigation bars with a previous title that fits and one that does not, at 320
    and 200 points (their view trees, the back button's label among them); a navigation push with UIKit's animations
    on (the transition coordinator's `isAnimated`, `transitionDuration` and `completionCurve`, and when its completion
    ran); the caret of a text view first responder (its cursor views' classes, frames, tints and Core Animation
    animations: the blink keyframes; and every 50 ms for 12 s, a character typed at 6 s, each cursor view's
    presentation opacity, `isHidden`, `alpha` and `isBlinking`) and of a read-only text view that is not;
    `caret_after`, read after the capture: the text field's samples while it was first responder for 6 s, and the
    read-only text view's while it was;
  - a `[[capture]]` per crop;
- `measure1/screen/…`: the crops;
- `measure-differ-levels.txt`: the files that differ between the two runs, by how much.

Limits of this run, which the next behavior run (with `Measure.swift` after `a9c03143`) addresses: the samples read the
cursor view's own layer, while the blink animation runs on its first sublayer (`text_view_cursor_0_animations`, layer
`0.0`), so they show the opacity constant; and the text field's cursor views are read after it stopped being first
responder (no animations).

What the run shows (each job's run.txt): both runs finished `ok` on both images; the crops differ between the runs by at
most one level, but the caret's (the blink); the indexes differ only in times: when the push's completion ran, the
blink animation's begin time and the samples' times.
