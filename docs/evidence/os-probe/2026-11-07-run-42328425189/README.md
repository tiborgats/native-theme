# OS probe, run 42328425189 (2026-11-07)

What macOS 26 and macOS 15 reported to `tools/os-probe` (`native-theme-os-probe`) in the 0.8.0 measurement plan's
MP-events, MP-record (the rest of it after run 42272240639's caret), MP-colorspace and MP-highlight passes, dispatched
by hand through `.github/workflows/os-probe.yml` on the harness commit `662dc715f2487b0d933e688010dc7777daf2be6d` (on
`v0.8.0-tools-probe-m2`), on 2026-11-07, with `jobs=probe`, `images=["macos-26", "macos-15"]` and
`passes=colors,events,motion,motion-reduce-motion-off,colors-highlight-red`:
<https://github.com/tiborgats/native-theme/actions/runs/42328425189>. The workflow ran each pass as a job of its own
(artifact `os-probe-<image>-<pass>`), so on macOS 26 the passes ran on two builds: `colors/` and
`button-pass-motion-reduce-motion-off/` on macOS 26.7.2 (25G332, runner image macos26 20261102.0473.1), the others on
macOS 26.6.2 (25G83, macos26 20261026.0455.2); macOS 15 is 15.8.2 (24H232) throughout (`run.toml`; each probe.toml's
`[os]`). Every screen was at a backing scale of 1. The image's settings hold but where a folder's `settings.txt` says
otherwise (the images have Reduce Motion on).

GitHub deletes run artifacts after 90 days. This folder keeps the files the facts can cite, as the run uploaded them
(PNGs and recordings byte for byte), and nothing else of the run. Left in the artifacts: the recordings' frames
(`<stem>-frames/NNNNN.png`, which ffmpeg 9.0.2 cut from each `.mov` with its `.crop`, `-fps_mode passthrough`, as the
workflow's step *Cut the motion and events recordings into frames* does; their times are the `-frames.csv` kept
here), the `ffmpeg.txt` logs and the wallpaper records.

This folder states no fact: converting a reading into a value is the facts' work. Each probe.toml's `method` keys
state how it read; every time is counted from starting `screencapture`, and a recording starts some time after that,
so its own times are not the probe's (the ticker in the motion and drag recordings marks the probe's 5 ms steps).

## What is here (per `os-probe-macos-26/` and `os-probe-macos-15/`)

- `colors/`, `colors-highlight-red/` (`--colors`, `[macos.colors]`): every NSColor per appearance (the main pass's
  colour table) with `AppleHighlightColor`, `AppleAccentColor` and `AppleAquaColorVariant`; the second with
  `AppleHighlightColor` written as `1.000000 0.733333 0.721569 Red` (`settings.txt`), for U163. `color_space/` holds
  the window captured per appearance with Core Animation colour animations paused at 0, 0.5 and 1 of their run over
  black and white, and static reference squares (U158); `color_space.toml` is
  `analysis/read_color_space.py`'s reading of them.
- `button-pass-events/` (`--button-pass --events`, `[macos.button_pass.events]`; U320–U322, U329, U600, U607–U609):
  the user defaults read; `double_click`, two posted clicks at stepped distances and gaps; `drag_start`, a press on
  selected text held and moved by stepped times and distances; `tool_tip`, the tool tip's window watched for; per
  appearance `typing-pointer.mov` (the cursor drawn, typing into a text view under it) and `drag-over.mov` with the
  window captures `drag-over-table-on.png`, `-table-above.png`, `-text.png`, `-spring-early.png` and
  `-spring-late.png`, each with its `.crop` and `-frames.csv`.
- `button-pass-motion/` (the image's Reduce Motion, on) and `button-pass-motion-reduce-motion-off/` (`--button-pass
  --motion`, `[macos.button_pass.motion]`, keyboard navigation on; U103, U160, U181, U182, U302, U328, U332, U606,
  U612): per appearance `press-push.mov`, `press-glass.mov` (macOS 26 only), `radio.mov`, `search-cancel.mov`,
  `focus-interrupt.mov`, `focus-deactivate.mov`, `caret-long.mov`, `caret-mixed.mov` and `caret-read-only.mov`, each
  with its `.crop` and `-frames.csv`; probe.toml holds each recording's posted events and actions with their times and,
  for the presses, the radio and the focus recording, the sampled view's alpha and presentation layers.

## What the passes read

- **Colour interpolation** (`color_space.toml`). Every paused square's captured colour is the one premultiplied
  interpolation in sRGB's encoded values gives, within half a byte, on both images and in both appearances (red to
  blue at 0.2 alpha, half way, over black: captured `[128, 0, 25]`, premultiplied sRGB `[127.5, 0, 25.5]`, straight
  sRGB `[76.5, 0, 76.5]`; opaque red to blue, half way: captured `[128, 0, 128]`, linear light `[187.5, 0, 187.5]`).
  For the opaque pair straight and premultiplied interpolation coincide, and `nearest` names the first of the tie,
  `straight_srgb`; for every other animated square it is `premultiplied_srgb`. The reference squares
  come out as their sRGB colours composited over the backdrop (within a byte), so the capture adds no colour
  conversion.
- **The highlight setting.** With `AppleHighlightColor` written, `highlightColor` reads the same (`#ffffffff` aqua,
  `#b4b4b4ff` darkAqua) on both images; `selectedTextBackgroundColor` and `selectedControlColor` change (aqua
  `#b3d7ffff` to `#f6c4c5ff`, darkAqua `#3f638bff` to `#8b5759ff`), and so do `textInsertionPointColor` and a text
  view's `insertionPointColor` (`#007affff` to `#e0383eff` aqua, `#ff5257ff` darkAqua).
- **Double clicks.** Every posted mouse-down read `clickCount` 1, the second click too, at 0 pt and 100 ms: the window
  server gives a posted event the click state its poster sets, so this pass does not measure the double-click distance
  or height (U321, U608, U329's `double_click_height`).
- **Drag start.** After an 800 ms hold a drag session started (the drag pasteboard changed) at 4 pt of movement and
  not at 1 to 3 pt (the selection was replaced instead); with 12 pt of movement it started after every hold, 0 ms
  included; the same on both images.
- **Tool tip.** The tool tip's window (`NSToolTipPanel`) was visible 1591 ms (macOS 26) and 1516 ms (macOS 15) after the
  cursor reached the button, 1590 and 1506 ms after a 5 s rest away, and 8 and 17 ms when the cursor came back 300 ms
  after the first tip; `NSInitialToolTipDelay` is absent.
- **Drag over.** The table drew a filled row (on) and a line (above) in `#0064e1ff` (aqua) and `#0059d1ff` (darkAqua)
  capture bytes on both images; the text view drew an insertion point at the end of its text (the drop point, its centre, lies below the text); the spring-loaded button
  drew pressed in the 9 s capture and was on in the 11 s one (`spring_state_after` 1), `com.apple.springing.delay` 0.5;
  nothing was dropped (both texts unchanged).
- **Typing.** In macOS 26's aqua `typing-pointer` frames (the others not yet read) the cursor is drawn until the first
  key and not after it, comes back after the 1 pt mouse move and goes again at the next key; `CGCursorIsVisible` read false from the first key to the end, so
  it does not follow the cursor's return.
- **Motion.** Every recording finished (`recording = "exit 0"`); their frames are the facts' to read.
