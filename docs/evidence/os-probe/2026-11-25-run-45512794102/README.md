# OS probe workflow, run 45512794102 (2026-11-25): the labelled switch and the DisclosureGroup on macOS, delivery recorded

[The run](https://github.com/tiborgats/native-theme/actions/runs/45512794102): `os-probe.yml` (`jobs=ios-host,switch-label`,
`images=["macos-15","macos-26"]`, the ios-host inputs `ios_host_passes=render,press`, `ios_host_set=lists`) on
`m2-fs4d` at `73a0b08d`. The ios-host jobs' evidence is in
[`../../ios-host/2026-11-25-run-45512794102/`](../../ios-host/2026-11-25-run-45512794102/README.md). The switch-label
job's host, `tools/swiftui-host/SwitchLabel.swift` at `73a0b08d`, draws what it drew in
[run 45214606115](../2026-11-23-run-45214606115/README.md) and now (review RV-fs4d, must-fix 1 and 2):

- parks the cursor on the middle of its content view's left margin (`park_point_in_content_pt`, where hitTest: gives
  the content view: `view_at_park_point = "Flipped"`), not off its window;
- posts every mouse event only where `NSWindow.windowNumber(at:belowWindowWithWindowNumber: 0)` gives its own window,
  and hovers and presses only in a key window of an active application;
- records, through a local event monitor that returns each event unchanged (the window accepting mouse-moved events),
  every mouse-moved, -entered, -exited, left-down and left-up event the application received: per capture those since
  the post before it (`[[capture]] events`), per release those after the mouse-up and the park (`[[release]]`);
- reads its accessibility settings (`[accessibility]`), and the job records the VM's (`settings.txt`).

| Folder | Preset | Runner | Xcode, SDK |
|---|---|---|---|
| `switch-label-macos-15/` | `macos-monterey` | macOS 15.8.2 (24H232) | Xcode 16.4, macosx 15.5 (24F74) |
| `switch-label-macos-26/` | `macos-tahoe` | macOS 26.7.2 (25G332) | Xcode 26.6, macosx 26.5 (25F70) |

Kept here, per image folder, from the job's artifact: `environment.txt`, `settings.txt`, `build.log`, `run.log`,
`switch-label.toml`, and `CAPTURES.sha256` (the SHA-256 and path of each of the 18 captures; the PNG files stay in the
run's artifact and are regenerable from the commit), and `measure.toml`, the output of

`python3 -I docs/evidence/os-probe/2026-11-25-run-45512794102/analysis/measure_switch_label.py <artifact folder>`

over the artifact's folder (run again over it: byte-identical). Its docstring gives the method. Against run
45214606115's script it searches the label on both sides of the switch (`label_side`), says "not measured" where a crop
falls outside a capture or is empty, and writes a capture's 0 changed pixels as unchanged only where the host recorded
the event's delivery (`delivery`), else `"not measured (delivery not recorded)"`.

What the run shows:

- Both jobs finished green, `failures = []`, `notes = []`. Every read run 45214606115's `measure.toml` also has is
  equal here (968 keys on macOS 15, 824 on macOS 26), and the host's `[[colours]]` are the same.
- Every hover and press was delivered: each hovered capture lists a `mouseMoved` and each pressed one a
  `leftMouseDown` in the host's window at the target point, hitTest: giving `PlatformSwitch` at a Toggle's switch and
  the DisclosureGroup's `NSHostingView` at its chevron and its label. So the 0 changed pixels of every hovered Toggle
  (label and switch), of the hovered DisclosureGroup (label and chevron), of the DisclosureGroup's label pressed, and of
  the dark chevron pressed are measured: the hover and the press reached the view and changed nothing there.
- The mouse-up after a Toggle's press is not in its `[[release]]` (`delivery = "not recorded"`; its `events` hold only
  the park's `mouseMoved`): the monitor did not see it, why is not verified (the switch's own mouse tracking is a
  guess). The DisclosureGroup's mouse-ups are recorded. No measured row rests on a release.
- The VM's settings (`settings.txt`, the host's `[accessibility]` agreeing): Reduce Transparency on (`1`), Reduce
  Motion on (`1`), Increase Contrast, Differentiate Without Color, desktop tinting, `AppleInterfaceStyle` and the accent
  absent (the default blue: `controlAccentColor` #007affff) on both images. The earlier switch-label runs recorded
  none (they ran on macOS 15.8.2 and 26.7.2 too; whether their VMs had the same settings is not recorded).

The values are in `.superpowers/sdd/v0.8.0-plan/M2/FS4/measured.md`, sections "switch label" and "expander (SwiftUI
DisclosureGroup)".
