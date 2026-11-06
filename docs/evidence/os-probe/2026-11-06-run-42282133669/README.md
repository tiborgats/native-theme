# OS probe, run 42282133669 (2026-11-06)

The reads pass again on macOS 26, for two readings run 42272240639 did not decide (MP-text's mirroring, U318, and
U104's plain-view focus ring), dispatched by hand through `.github/workflows/os-probe.yml` on the harness commit
`2f4bca36b5136ec9e45f0712bdfacbef9f386b83` (on `v0.8.0-tools-probe-m2`), on 2026-11-06, with `jobs=probe`,
`images=["macos-26", "macos-15"]` and `passes=reads,reads-rtl`:
<https://github.com/tiborgats/native-theme/actions/runs/42282133669>. Only the macos-26 job's files are here (the
macos-15 job had not run when this folder was committed). This runner was macOS 26.6.2 (runner image macos26
20261026.0455.2), not run 42272240639's 26.7.2. The screen at a backing scale of 1.

## What is here (`os-probe-macos-26/`)

- `reads/probe.toml`, `settings.txt` (keyboard navigation on, restored after): the reads pass with the process's
  own language (`preferredLanguages` `en-US`), and of its files `mirroring/` and `focus_view/`; the pass's other
  sections repeat run 42272240639's and their files are left in the artifact.
- `reads-rtl/probe.toml`, `settings.txt`, `mirroring/`: the reads pass given `-AppleLanguages (he)
  -AppleTextDirection YES -NSForceRightToLeftWritingDirection YES` in its arguments (no setting written).

## What the passes read

This folder states no fact.

- **Mirroring** (`[macos.reads.mirroring]`). With the right-to-left arguments `NSApp`'s and each image view's
  `userInterfaceLayoutDirection` is 1 (right to left), with the language alone (run 42272240639) 0. Compared pixel
  for pixel with `reads/mirroring/`: `chevron.forward`, `arrow.forward`, `arrow.uturn.backward` and `list.bullet` draw
  as the left-to-right image flipped horizontally, `text.alignleft` and `star` as the same image, at 1 and 2 pixels a
  point.
- **The plain view's focus ring** (`[macos.reads.focus_view]`). The view (first responder, focus ring mask its
  bounds) draws no ring: in both appearances the focused captures, with focus ring type default (0) and exterior (2),
  are the same pixels as the unfocused one.
