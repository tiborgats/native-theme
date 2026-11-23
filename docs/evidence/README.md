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
| [`2026-10-31-run-41195846264/`](os-probe/2026-10-31-run-41195846264/) ([run](https://github.com/tiborgats/native-theme/actions/runs/41195846264)) | 2026-10-31 | `18d2a5d8` on `v0.8.0-tools-probe-m2` | the unemphasized table selection on macOS 26, for the label colour on an unfocused selection (issue #4, step 7); only `probe.toml`, the two unemphasized renders and the README are kept here |
| [`2026-11-06-run-42215651196/`](os-probe/2026-11-06-run-42215651196/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42215651196)) | 2026-11-06 | `20d56259` on `v0.8.0-tools-probe-m2` | the unemphasized table selection on macOS 15, for the label colour on an unfocused selection (issue #4, step 7); only `probe.toml`, the two unemphasized renders and the README are kept here |
| [`2026-11-23-run-45198750893/`](os-probe/2026-11-23-run-45198750893/) ([run](https://github.com/tiborgats/native-theme/actions/runs/45198750893)) | 2026-11-23 | `c4f33eef` on `m2-fs4c` | the controls pass on macOS 15 and 26: check boxes, a pop-up, disclosure buttons, a text field, tables, a segmented control, a slider, a switch, a tab view, a split view, a box and a toolbar button, hovered and pressed (M2.FS4 of the 0.8.0 plan) |
| [`2026-11-23-run-45210835145/`](os-probe/2026-11-23-run-45210835145/) ([run](https://github.com/tiborgats/native-theme/actions/runs/45210835145)) | 2026-11-23 | `70c80753` on `m2-fs4d` | the switch-label job: SwiftUI's `Toggle` with the switch style on macOS 15 and 26, at rest, hovered and pressed, off and on, enabled and disabled, with reference labels (M2.FS4 of the 0.8.0 plan) |
| [`2026-11-23-run-45214606115/`](os-probe/2026-11-23-run-45214606115/) ([run](https://github.com/tiborgats/native-theme/actions/runs/45214606115)) | 2026-11-23 | `53b749a8` on `m2-fs4d` | the switch-label job again, with a SwiftUI `DisclosureGroup` (the expander's header) hovered and pressed on macOS 15 and 26 (M2.FS4 of the 0.8.0 plan) |

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
| [`2026-11-18/`](breeze-offscreen/2026-11-18/) | 2026-11-18 | decision 10 of the 0.7.2 KDE steps 4 and 5: a combo box's own frame closed and open, with the pointer away and over it (the pen and the fill) |

A citation links the file and its line (`evidence/breeze-offscreen/2026-11-13/out_light.txt#L27`).

## `widget-icons/`

What the installed freedesktop icon themes and the bundled Lucide and Material sets give
gpui-component's 106 `IconName`s through the gpui connector's lookup, at 16 and 24 px,
whether a mask of one colour carries each, and how long the lookup takes beside the gpui
showcase's own reload of its icons: the measurement before the connector's widget icons
were built ([specification](../archive/todo_v0.7.2_gpui-widget-icons-spec.md), Part C). Runs by
hand, one directory per date, written by the gpui showcase's ignored test
`widget_icons_listing`, which was removed once its run was recorded (commit `c89dfe85` holds
it); each holds `listing.md`, a table, and a `README.md`. No drawing of
an installed icon theme is kept: each icon is named, with its size, whether it is one colour
and a hash of its bytes; the sheets of a run were read in the repository's `target/` and
are not kept.

| Run | Date | Why |
|-----|------|-----|
| [`2026-11-19/`](widget-icons/2026-11-19/) | 2026-11-19 | the seven stop conditions of the specification's C.5 |

## `winui-host/`

What `tools/winui-host` (on the parked branch `v0.8.0-tools-hosts-m1b`,
never published), a WinUI 3 application dispatched through that branch's
`.github/workflows/os-probe.yml` (its winui-host job, which this branch's
workflow does not have), read on a GitHub Windows runner. Kept here: the file a citation reads, with the run's own `README.md`
and `run.toml`, copied unchanged from this repository's commit `efbffffe`.
The run's README describes the run as the evidence repository keeps it, and
names recordings, a text-scale run and the OS probe's run of the same day
(`docs/evidence/os-probe/2026-11-06-run-42307873328/`) that are not kept here.

| Run | Date | Commit | Why |
|-----|------|--------|-----|
| [`2026-11-06-run-42308526155/`](winui-host/2026-11-06-run-42308526155/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42308526155)) | 2026-11-06 | `7bad463f` on `v0.8.0-tools-hosts-m1b` | `GetCaretBlinkTime`, windows-11's caret blink ([specification](../archive/todo_v0.7.2_caret-blink-spec.md) Part 0) |

A citation links the file and its line
(`evidence/winui-host/2026-11-06-run-42308526155/winui-host-windows-11-arm/frames.toml#L248`).

## `ios-host/`

What `tools/ios-host` (on the parked branch `v0.8.0-tools-ios-m2`, never
published), an iOS application run in the iOS Simulator by that branch's
`.github/workflows/os-probe.yml` (its ios-host job, which this branch's
workflow does not have), read on a GitHub macOS runner. Kept here, copied
unchanged from this repository's commit
`72fcf6a7`: the run's own `README.md` and `run.toml`, the script that derives
the cited values (`analysis/behavior_values.py`, its docstring the method),
and of the image `ios-host-macos-15` (the iOS 18.6 simulator, preset
`ios-13`) the cited `behavior_values.toml` and the inputs the script reads
(`measure1/index.toml` and the two caret crops). The run's other image,
`ios-host-macos-26`, is the parked `ios-26` preset's and is not kept, nor are
the image's other crops and `measure-differ-levels.txt`, which the run's
README lists as kept, nor the README of run 42268160866 it links to: they
are in the evidence repository and in this repository's commit `b38c1795`
(on `v0.8.0-tools-ios-m2`).

| Run | Date | Commit | Why |
|-----|------|--------|-----|
| [`2026-11-06-run-42275028573/`](ios-host/2026-11-06-run-42275028573/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42275028573)) | 2026-11-06 | `7bb1e784` on `v0.8.0-tools-ios-m2` | ios-13's caret blink ([specification](../archive/todo_v0.7.2_caret-blink-spec.md) Part 0) |

`python3 -I docs/evidence/ios-host/2026-11-06-run-42275028573/analysis/behavior_values.py docs/evidence/ios-host/2026-11-06-run-42275028573/ios-host-macos-15`
writes `behavior_values.toml` again from the kept inputs (Pillow). A citation
links the file and its line
(`evidence/ios-host/2026-11-06-run-42275028573/ios-host-macos-15/behavior_values.toml#L11`).
