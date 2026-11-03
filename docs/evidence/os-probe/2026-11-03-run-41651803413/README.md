# OS probe, run 41651803413 (2026-11-03)

What macOS 26 reported to `tools/os-probe` (`native-theme-os-probe`) in a run for `macos-tahoe`'s preset task 16:
the passes of the earlier runs, a new pass with Reduce Transparency off and desktop tinting off, paired with the
existing Reduce Transparency off pass (tinting on, the image's), and AVKit's `AVRoutePickerView.h` among the SDK header
copies. Dispatched through `.github/workflows/os-probe.yml` on the harness commit `f89dfe0b` (on
`v0.8.0-tools-probe-m2`), on 2026-11-03, with `jobs=probe` and `images=["macos-26"]`:
<https://github.com/tiborgats/native-theme/actions/runs/41651803413>. GitHub deletes run artifacts after 90 days; this
folder keeps the files the macOS facts cite, as the run uploaded them, and nothing else of the run.

`run.toml` records the image's builds: macOS 26.6.2 (25G83), runner image `macos26` 20261026.0455.2, Xcode 26.6
(17F113), the probe built against the macOS 26.5 SDK (25F70). The desktop picture is the image's solid black
(`environment/wallpaper.txt`: `/System/Library/Desktop Pictures/Solid Colors/Black.png`). Every pass ran with the
image's settings unless it names one: Reduce Transparency on, Reduce Motion on, Increase Contrast off, no accent colour
set, desktop tinting on (`AppleReduceDesktopTinting` unset).

## What is here (`os-probe-macos-26/`)

The folder as the facts cite it (the macOS facts lane t16b, 2026-11-03). Files no fact cites were left out, as the
citation checker requires every evidence file to be cited (spec S17 *`evidence`*), and stay in the run's artifact:
the other passes, each pass's `settings.txt` and renders, `environment/`, and `sdk-headers/` (the header copies, their
copy lines and SHA-256 sums in `sdk-headers/xcode.txt`; `AVKit/AVRoutePickerView.h`, SHA-256 `f7107a79…`, is cited
at `xcode@26.6` as every SDK header is, which `os-probe.yml`'s macOS job checks).

- `reduce-transparency-off/probe.toml`: Reduce Transparency off (`reduceTransparency` written false, read back 0),
  desktop tinting on.
- `reduce-transparency-off-tinting-off/probe.toml`: Reduce Transparency off and desktop tinting off
  (`AppleReduceDesktopTinting` written true, read back 1; both restored after the pass).

## What the pair reads

No named NSColor differs between the two passes (`[macos.colors.<appearance>]`): the role facts read from named
colours hold with tinting on and off, so they cite both passes (MAC-X27). What is drawn differs, which no role fact
states: in dark mode the window background, title bar and list header are drawn `#1b1b1b` raw (`#242424` sRGB) with
tinting on and `#1e1e1e` raw (`#282828`) with it off, where the drawn calibration background equals
`windowBackgroundColor` exactly (`[macos.key_window.darkAqua.calibration]` `exact = true`); dark control fills rise 2
to 4 per channel; in light mode only the sidebar changes (`#dddddd` to `#e9e9e9` raw). The tinted values follow the
desktop picture (here black), so they are no platform constant: the window's tinted drawing is
`window.content.fill @ !cond system_setting:wallpaper_tinting` (MAC-W08, the `window` unit's).
