# OS probe, run 40320895749 (2026-10-26)

What each GitHub runner image's OS reported to `tools/os-probe` (`native-theme-os-probe`),
dispatched by hand on `v0.7.0-rc1` at `96199f83` through `.github/workflows/os-probe.yml`
(https://github.com/tiborgats/native-theme/actions/runs/40320895749), on 2026-10-26. Kept here
because GitHub deletes run artifacts after 90 days, and `docs/platform-facts.md` cites them
(the D round of `docs/todo_code-review-plan.md`: the sidebar, the title bar, the list header
and the selected segment of a key window on macOS, and Increase Contrast).

| Directory | OS reported | Runner image version |
|-----------|-------------|----------------------|
| `os-probe-macos-14` | macOS 14.8.9 (23J631) | 20260831.0302.1 |
| `os-probe-macos-15` | macOS 15.7.9 (24G830) | 20260907.0337.1 |
| `os-probe-macos-26` | macOS 26.6.2 (25G83) | 20261013.0424.1 |
| `os-probe-xcode-27` | macOS 27.0.1 (26A434), GitHub's preview image | 20261023.0307.1 |
| `os-probe-windows-2022` | Windows Server 2022 Datacenter 21H2, build 20348.5761 | 20261022.344.1 |
| `os-probe-windows-2025` | Windows Server 2025 Datacenter 24H2, build 26100.33650 | 20261021.278.1 |
| `os-probe-windows-11-arm` | Windows 11 25H2, build 26200.9711 (the registry's `ProductName` says "Windows 10 Enterprise", as it does on every Windows 11) | 20261022.194.1 |

## What is the same as before

Every value the previous run reported ([`2026-10-14-run-38478334574/`](../2026-10-14-run-38478334574/))
is the same here, apart from three things:
- the runner image versions of macOS 26, xcode-27 and the three Windows images;
- the Windows builds' update revisions (`UBR`);
- the Windows `sys_colors` notes, which cite the facts file's lines as of this run's commit
  (`docs/platform-facts.md:358-368`; the previous run cites `:355-365`).

The pattern colours' object addresses differ too, as they always do.

The macOS control renders under `renders/aqua/` and `renders/darkAqua/` are byte-identical to
the previous run's on macOS 14, 15 and xcode-27. On macOS 26, 34 of the 68 files differ in their
bytes but not in a single pixel.

## What is new: `[macos.key_window]`

A titled window made key in a regular, active application. The section's `method` states every
step. It contains:
- an `NSVisualEffectView` of material `sidebar` with `state` active;
- an `NSTableView` with its header;
- an `NSSegmentedControl` with segment 0 selected;
- a calibration strip of `NSBox` patches.

The window is captured per appearance with `screencapture -l` (`renders/key_window/aqua.png`,
`darkAqua.png`). Each sample has two readings:
- `raw`, the PNG's own bytes, which are the evidence;
- `colorAtXY_sRGB`, kept to show the colour-space defect that run 40317706382 had (it read the
  capture's pixels as gamma-1.8 calibrated RGB). It is not evidence.

**Calibration.** Each capture has nine sRGB patches (`srgb_000000` … `srgb_0000ff`, filled with
`colorWithSRGBRed:green:blue:alpha:`). All 288 read exact in the 32 captures (4 images × 4
passes × 2 appearances), so every capture's `raw` samples are the drawn sRGB values.

The two system-colour patches (`windowBackgroundColor`, `controlBackgroundColor`) compare the
drawn colour with the NSColor the probe reads. 26 of their 64 readings differ, and every
difference is that the window draws the colour otherwise than the NSColor reads:
- On macOS 14 and 15, `windowBackgroundColor` draws #f6f6f6 / #282828 with Reduce Transparency
  on (the main, Increase Contrast and app-bundle passes; the light one matches where Increase
  Contrast's NSColor also reads #f6f6f6) and #eaeaea / #242424 with it off, where the NSColor
  reads #ececec / #323232: 14 of the 26.
- With Reduce Transparency off, the dark `controlBackgroundColor` draws #1b1b1b where it reads
  #1e1e1e (all four images), and so does the dark `windowBackgroundColor` on macOS 26 and
  xcode-27: 6.
- With Increase Contrast on, the dark `controlBackgroundColor` draws #1e1e1e where it reads
  #202020 (all four images), and so does the dark `windowBackgroundColor` on macOS 26 and
  xcode-27: 6.

No sRGB patch, channel or image fails.

**Key status.** `isKeyWindow` and `NSApp_isActive` are true at every capture of the main pass,
the Reduce Transparency pass and the Increase Contrast pass, on all four images.
- `[macos.key_window.activation] activated_by` records only the two activation attempts before
  the window is made key, `activateIgnoringOtherApps:` and then `-activate`, each pumped 20 steps.
- Where it reads `none`, neither attempt made the application active within those steps (10 of
  the 16 passes). The application became active in the key-wait loop instead, which calls
  `activateIgnoringOtherApps:` again at each step it is not active: `key_wait_reactivations`
  reads 1 and `key_wait_steps` 1 in those passes.
- `none` therefore does not mean key without active. Each capture's own `isKeyWindow` and
  `NSApp_isActive` are true.

**The app-bundle pass** (`app-bundle/`) is not evidence for a key window. The probe ran as an
ad-hoc-signed `.app` started by `open -W -n`; `run.txt` records each step's exit status.
- It was active and key before its captures: `activated_by = "activateIgnoringOtherApps:"`,
  `key_wait_steps = 0`.
- It was neither at them: `isKeyWindow = false` and `NSApp_isActive = false` under
  `[macos.key_window.aqua]` and `[macos.key_window.darkAqua]`.
- Its captures draw a window that is not key: a grey title bar, and on macOS 26 a grey selected
  segment.

**The passes.**

| Directory | Settings | What it records |
|-----------|----------|-----------------|
| `.` (the image's own) | `reduceTransparency` 1, `increaseContrast` unset | the main pass |
| `reduce-transparency-off/` | `reduceTransparency` 0 | `settings.txt`: the write's exit status and the value read back. Every NSColor the probe reads is as in the main pass; the captures' materials change. |
| `increase-contrast-on/` | `increaseContrast` 1, `reduceTransparency` restored to 1 | `settings.txt`. 81 NSColors change on macOS 14, 15 and xcode-27, 82 on macOS 26 (labelColor #000000d8 → #000000ff, separatorColor #00000019 → #00000059, selectedContentBackgroundColor #0064e1 → #0040dd, `probe.toml:308`, `:319`, `:324`). |
| `app-bundle/` | the image's own, restored | `run.txt`, `stdout.txt`, `stderr.txt`. Not key at its captures (above). |

**macOS 14 and 15 agree.** Every sample of every pass reads the same on both.

**macOS 26 and xcode-27 agree**, except in three samples:
- the dark selected segment, #077dff on 26 and #067dff on 27;
- with Reduce Transparency off, the dark title bar, #1b1b1b on 26 and #232323 on 27;
- with Reduce Transparency off, the dark list header, the same two values.
