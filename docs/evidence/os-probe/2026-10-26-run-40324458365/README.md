# OS probe, run 40324458365 (2026-10-26)

What each GitHub runner image's OS reported to `tools/os-probe` (`native-theme-os-probe`),
dispatched by hand on `v0.7.0-rc1` at `891e66dd` through `.github/workflows/os-probe.yml`
(https://github.com/tiborgats/native-theme/actions/runs/40324458365), on 2026-10-26. Kept here
because GitHub deletes run artifacts after 90 days, and `docs/platform-facts.md` cites them. The
run adds three readings to the previous one
([`2026-10-26-run-40320895749/`](../2026-10-26-run-40320895749/), whose README describes the
key-window section, the calibration strip and the other passes):
- a window that is not key;
- wallpaper tinting off;
- the wallpaper.

The runner images are the previous run's: the table there gives each image's OS and version.

## What is the same as before

The main pass, the Reduce Transparency pass, the Increase Contrast pass and the app-bundle pass
report every value the previous run did, on all seven images. The one exception is the
activation timing of macOS 15's Reduce Transparency and Increase Contrast passes: the attempt
and step counts under `[macos.key_window.activation]` differ, and key status does not. Every
render under `renders/`, the key window's captures included, is byte-identical to the previous
run's.

## What is new

**`environment/wallpaper.txt`** (macOS). The desktop picture is solid black on all four images:
- System Events names `/System/Library/Desktop Pictures/Solid Colors/Black.png` on macOS 14, 15,
  26 and xcode-27.
- The wallpaper store (`~/Library/Application Support/com.apple.wallpaper/Store/Index.plist`)
  lists that file for the display on macOS 14 and 15. On macOS 26 and xcode-27 it records an
  image choice without a file list.

**`tinting-off/`**: wallpaper tinting off ("Allow wallpaper tinting in windows" unchecked, the
global default `AppleReduceDesktopTinting`).
- `settings.txt`: the image has no value (absent); the pass writes true (exit 0) and reads back
  1, with Reduce Transparency restored to the image's 1 and Increase Contrast to absent.
- On all four images, every key-window sample and both system-colour patches draw the same as in
  the main pass, in both appearances: tinting off changes nothing on these runners.
- That shows the setting took in the defaults database. It cannot rule out tinting that the
  window server applies, or one that needs a process the pass does not restart. With a black
  wallpaper, a tint toward the wallpaper would also darken the light window background, and the
  light background draws lighter than its NSColor reads (below).

**`not-key/`**: the probe's window captured while a second window of the probe, 240 × 120 pt at
[900, 200], is key (`--not-key`, `[macos.key_window.not_key]`).
- Before the captures: `isKeyWindow = false`, `other_window_isKeyWindow = true`,
  `NSApp_isActive = true`.
- At each capture: `isKeyWindow = false`, `NSApp_isActive = true`, under
  `[macos.key_window.aqua]` and `[macos.key_window.darkAqua]`. The application stays active, and
  the window shows its not-key look.
- macOS 14 and 15 agree, as do macOS 26 and xcode-27:

| Sample | macOS 14, 15: not key (key) | macOS 26, 27: not key (key) |
|--------|-----------------------------|-----------------------------|
| title bar | #ececec / #141414 (#ffffff / #262626) | #ffffff / #1e1e1e (the same) |
| selected segment | #ffffff / #666666 (the same) | #cfcfcf / #494949 (#0077f9 / #077dff; 27 dark #067dff) |
| sidebar, `state` active | #f6f6f6 / #383838 (the same) | the same |
| list header | #ffffff / #1e1e1e (the same) | the same |
| window background | #f6f6f6 / #282828 (the same) | #ffffff / #1e1e1e (the same) |

**Calibration.** All 432 sRGB patches read exact: 9 per capture × 48 captures (4 images × 6
passes × 2 appearances). Every capture's `raw` samples are the drawn sRGB values.

34 of the 96 system-colour readings differ from the NSColor the probe reads:
- the previous run's 26, the same in every pass that run had;
- 4 in each of the two new passes on macOS 14 and 15: `windowBackgroundColor` draws #f6f6f6 /
  #282828 where it reads #ececec / #323232.

On macOS 26 and xcode-27 the two new passes draw both patches as they read.

## The window background on macOS 14 and 15

A box filled with `windowBackgroundColor`, and the window's own background, draw #f6f6f6 /
#282828 with Reduce Transparency on, key or not, with tinting on or off. They draw #eaeaea /
#242424 with Reduce Transparency off. The NSColor reads #ececec / #323232 in every pass. On macOS
26 and xcode-27 the colour draws as it reads, #ffffff / #1e1e1e.

What the runs do not show:
- whether the drawn value depends on the wallpaper (every image's is black, and no run changes
  it);
- what makes the drawn colour differ from the read one.
