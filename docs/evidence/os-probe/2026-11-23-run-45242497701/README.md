# OS probe, run 45242497701 (2026-11-23)

The controls pass of `tools/os-probe` (`native-theme-os-probe --button-pass --controls`) on branch `m2-fs4c` at commit
`2250ca76` (FS4c5's fixes of the review of run 45231684209's pass), dispatched by hand through
`.github/workflows/os-probe.yml` (jobs `probe`, passes `controls`, images macOS 15 and 26):
<https://github.com/tiborgats/native-theme/actions/runs/45242497701>. The macOS 26 job got no runner on the first
attempt ("not acquired by Runner of type hosted", cancelled after 15 min) and was re-queued once with
`gh run rerun --failed`; on the second attempt both probe jobs are green. GitHub deletes the run's artifacts after 90
days; this folder keeps them whole, as the run uploaded them (PNGs byte for byte).

| Folder | Preset | Runner | Xcode | Rust |
|---|---|---|---|---|
| `os-probe-macos-15-controls/` | `macos-monterey` | macOS 15.8.2 (24H232), image 20261118.0550.1 | Xcode 16.4 | rustc 1.100.0 |
| `os-probe-macos-26-controls/` | `macos-tahoe` | macOS 26.7.2 (25G332), image 20261117.0516.1 | Xcode 26.6 | rustc 1.100.0 |

Both jobs captured 410 PNGs.

## Reduce Transparency off

Both images turn Reduce Transparency on: `environment/accessibility.txt` (new in this run) records the image's
`com.apple.universalaccess reduceTransparency` as 1 on both, as every earlier run's job log did (run 45231684209's
"Record the image's accessibility settings" step: 1 on both; run 40320895749's README: 1 on all four images then). Every
earlier controls run therefore measured the reduced-transparency look. This run turns it off for the controls pass
before the probe starts: `button-pass-controls/settings.txt` (image's 1, write exit 0, read back 0), and the probe's
own read `[macos.environment.accessibility] accessibilityDisplayShouldReduceTransparency` is `false` on both (it was
`true` in every earlier controls run).

## What is here

The layout of run 45231684209's folder, plus `environment/accessibility.txt` and `button-pass-controls/settings.txt`;
`controls.toml` and `controls-toolbar_sheet.toml` per image, the output of run 45226132792's
`analysis/measure_controls.py` (unchanged) over the image folder. `analysis/`:

- `compare_reads-run-45231684209-macos-{15,26}[-toolbar_sheet].txt`: run 45217592058's `analysis/compare_reads.py`,
  run 45231684209's reads against this run's;
- `compare_all.py` and `compare_all-run-45231684209-macos-{15,26}.txt`: every field of every key compared;
- `check_grey_in_part.py` and its outputs: the solved layer composited over the grey backdrop and over the window
  background, compared with those captures inside one target's part (the legacy scroller knob; the toolbar sheet's
  button).

Run from the repository root, e.g. `python3 -I docs/evidence/os-probe/2026-11-23-run-45242497701/analysis/check_grey_in_part.py docs/evidence/os-probe/2026-11-23-run-45242497701/os-probe-macos-26-controls toolbar_sheet toolbar_button`.

## What the run reads (the facts are measured.md's)

- **The overlay scroller's knob is still not drawn**, hovered or pressed, on either image: `flashScrollers` before each
  capture did not show it. The pass now says so: `<appearance>.{hovered,pressed}.scroller_knob_overlay.error`
  ("the knob is not drawn"), `knob_drawn.differing_pixels` 0; its presses waited 11–20 steps (no tracking). Commit
  `0a98826c` adds a posted line scroll before the flash (the next run).
- **The legacy knob** lies over the backdrops now (the scroll view and its clip view draw no background,
  `scroll_views.*.drawsBackground` and `contentView_drawsBackground` false), and its layer composites linearly (the
  check: 0 pixels of the part off by more than 2 levels over grey and over the window, both images, both
  appearances). Its presses tracked (`wait_steps` 1: the main thread was in the scroller's tracking until the capture
  thread was done).
- **macOS 26's toolbar sheet button is not a linear layer with Reduce Transparency off**: over the grey backdrop 1827–2049
  of its 2052 pixels differ from the black-and-white solve by more than 2 levels (largest 44.6–195.4); over macOS 15's
  sheet 0. The capsule that drew opaque #f2f2f2ff / #141414ff with Reduce Transparency on is a material that adapts
  to what lies behind it with it off.
- Reads that Reduce Transparency changes (against run 45231684209, `compare_all-*`): on both images the source-list
  row (rest and pressed), the real toolbar's button (fill and label), the dark table background, the pressed
  pop-up's open menu and the legacy knob; on macOS 26 also the dark text field's fill and label. Every other read is
  the same, but for `diff_from_normal` pixel counts.
- `over_backdrop` is now false for the default table row (`backdrop_covered_by` `NSClipView`, `NSScrollView`) and the
  toolbar button (outside the backdrop box); the source-list row's scroll view draws no background (true).
