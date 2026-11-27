# OS probe, run 45663039135 (2026-11-25)

The controls pass of `tools/os-probe` on branch `m2-fs4i`, commit `c29b57c42` (M2.FS4c14: four more specimens in the
toolbar states sheet), images macOS 15 (15.8.2, 24H232) and macOS 26 (26.7.2, 25G332). Dispatched by hand through
`.github/workflows/os-probe.yml` (jobs `probe`, passes `controls` and `controls-accent`, images
`["macos-15", "macos-26"]`): <https://github.com/tiborgats/native-theme/actions/runs/45663039135>. Attempt 1, all four
jobs green. Reduce Transparency was turned off on every job before the probe (`settings.txt`: the image's 1, written
false, read back 0; `accessibilityDisplayShouldReduceTransparency` false in each `probe.toml`), Increase Contrast off
(`environment/accessibility.txt` the same as run 45634379188's on each image). The `controls` jobs ran with the image's
accent (`AppleAccentColor` absent, `controlAccentColor` #007aff in both appearances); the `controls-accent` jobs with
the red accent, set as run 45345116763 set it, the only way the repository's runs set a non-default accent
(`defaults write -g AppleAccentColor -int 0` before the probe; `settings.txt`: the image's absent, written 0, read
back 0; the probe's process reads `AppleAccentColor = 0.0`, `controlAccentColor` #e0383e / #ff5257, which the
workflow checks).

Only `probe.toml`, `settings.txt`, `environment.txt` and the environment records are kept; `CAPTURES.sha256` names
every PNG of the four artifacts (8840, 2210 a job).

## What is here

- `analysis/measure_controls.py`: run 45634379188's, the four new specimens added (the two titled ones labelled) and
  `icon` (its docstring gives the method): on each specimen with an image, the image's colour inside the image's own
  rect, the `NSButtonImageView` subview's on macOS 15, the cell's `imageRectForBounds:` on macOS 26 (whose button has
  no such subview), so neither the title nor the title bar round the bezel is read. Its outputs
  `os-probe-macos-{15,26}-controls{,-accent}/controls-toolbar_states_sheet.toml`.
- `analysis/fit_toolbar_bezel.py`: run 45634379188's, the four new specimens added; its outputs
  `toolbar_bezel_fitted-toolbar_states_sheet.toml` in each folder.

The controls sheet itself was captured too (it is the same pass) but not measured here: no read of this lane needs it.
Every read below is in the window's pixels at backing scale 1, light / dark (aqua / darkAqua), the opaque composite
over the title bar as in run 45634379188 (`backdrop_reaches` 0, 0, 0 for every specimen and state).

## The toolbar states sheet

Run 45634379188's sheet with four more specimens after its five, each built as A (`NSBezelStyleToolbar`, "Button",
sized to fit) and given the button pass's symbol image: `toolbar_image_on` (`NSButtonTypePushOnPushOff`, state on,
`NSImageOnly`, empty title), `toolbar_image_disabled` (`setEnabled: NO`, `NSImageOnly`), `toolbar_titled_image_on`
(state on, the image beside the title, `NSImageLeading`) and `toolbar_titled_image_disabled` (disabled,
`NSImageLeading`). The on specimens are hovered and pressed, the disabled ones neither.

**The five earlier specimens read as in run 45634379188:** every `fill`, `label` and `ink` of
`controls-toolbar_states_sheet.toml` and every `fill`, `view_fill`, `mode`, `edge_top`, `edge_bottom`,
`backdrop_reaches` and `differs_from_toolbar_button` of `toolbar_bezel_fitted-toolbar_states_sheet.toml` are equal on
both images (macOS 15: the files equal but for the positions; macOS 26: the `common` counts and some corner rows of the
low-contrast rest capsule differ by one pixel, the title bar's gradient lying under the items at other x positions).
The items sit further left (A at x 263 on macOS 15, 256 on macOS 26).

**macOS 26 overflowed one specimen:** `toolbar_titled_image_disabled` is in no window
(`targets.toolbar_titled_image_disabled.error = "unavailable: the view is in no window"`): the eight items before it
(the last ending at x 795 of the 900 pt window) left no room for its 82 pt, so AppKit moved it to the toolbar's overflow
menu. Its reads are missing on macOS 26 in both jobs; on macOS 15 (76 pt items) all nine are placed.

### The icon colour (`icon`)

The glyph's box (`icon_box`: 14 x 14 px on macOS 15, 19 x 18 px on macOS 26, one row and on macOS 26 one column less
where disabled) lies inside the rect read, touching none of its edges, on every specimen and state.

| Specimen | State | macOS 15 | macOS 26 |
|---|---|---|---|
| image, no title (FS4c13's, for reference) | rest | #676767 / #aeaeae | #4d4d4d / #e9e9e9 |
| image, no title | hovered | #666666 / #aeaeae | #494949 / #ebebeb |
| image on, no title | rest | #4b4b4b / #e3e3e3 | #ffffff / #ffffff |
| image on, no title | hovered | #3b3b3b / #e7e7e7 | #ffffff / #ffffff |
| image on, no title | pressed | #3b3b3b / #e9e9e9 | #ffffff / #ffffff |
| titled with the image, on | rest, hovered, pressed | as the image on, every state | as the image on, every state |
| image disabled, no title | rest | #afafaf / #565656 | #bfbfbf / #666666 |
| titled with the image, disabled | rest | #afafaf / #565656 | not read (overflowed) |

The fills under them are the on toggle's of run 45634379188 (macOS 15 #dfdfdf / #414141, #c6c6c6 / #5a5a5a,
#b9b9b9 / #676767; macOS 26 the accent #007aff, #006df2 / #148eff, #0061e6 / #239dff) and, disabled, the rest fill
(macOS 15 none, the title bar #ececec / #343434; macOS 26 the rest capsule #ffffff / #252525). The titled on
specimen's label is the on toggle's of run 45634379188 (macOS 15 #424242 / #ececec at rest, #343434 / #efefef hovered,
#343434 / #f0f0f0 pressed; macOS 26 #ffffff), its disabled label the disabled A's (macOS 15 #acacac / #585858).

- macOS 15: the icon is not the label's colour, at rest (#676767 against #606060) nor on (#4b4b4b against #424242,
  #3b3b3b against #343434, dark #e3e3e3 against #ececec) nor disabled (#afafaf against #acacac, #565656 against
  #585858); the on icon is darker (light) or lighter (dark) than the off icon, as the on label is.
- macOS 26: on the accent the icon turns white, #ffffff in every state and both appearances, as the label does; the
  disabled icon is the disabled label's colour, #bfbfbf / #666666.

### The on toggle under the red accent

- macOS 15: every read of the states sheet under the red accent equals the default accent's (48 of 48 entries of
  `controls-toolbar_states_sheet.toml` identical): the on toggle's grey fill (#dfdfdf / #414141 at rest, #c6c6c6 /
  #5a5a5a hovered, #b9b9b9 / #676767 pressed), its label and its icon do not follow the accent.
- macOS 26 (read beside it, the same job on the other image): the on fills follow the accent, the icons and labels stay
  #ffffff. At rest the fill is `controlAccentColor` itself (#e0383e / #ff5257); hovered #d32b31 / #ff666b, pressed
  #c71f25 / #ff757a. Over both accents the shades are the rest fill shifted by a fixed amount per channel, clamped to
  0–255: light −13 hovered and −25 pressed, dark +20 hovered and +35 pressed (blue #007aff: #006df2, #0061e6 / #148eff,
  #239dff; red #e0383e: #d32b31, #c71f25 / #ff5257: #ff666b, #ff757a; every channel exact).
