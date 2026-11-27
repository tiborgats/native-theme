# OS probe, run 45634379188 (2026-11-25)

The controls pass of `tools/os-probe` on branch `m2-fs4h`, commit `4d453fbd` (M2.FS4c13: the controls pass's new
toolbar states sheet), images macOS 15 (15.8.2, 24H232) and macOS 26 (26.7.2, 25G332), `environment.txt` the same as
run 45542850219's. Dispatched by hand through `.github/workflows/os-probe.yml` (jobs `probe`, passes `controls`, images
`["macos-15", "macos-26"]`): <https://github.com/tiborgats/native-theme/actions/runs/45634379188>. Attempt 1: macOS 26
green; macOS 15 cancelled before it started ("The job was not acquired by Runner of type hosted even after multiple
attempts", GitHub's macOS capacity). Attempt 2 (`gh run rerun --failed`): macOS 15 green. Reduce Transparency was
turned off on both images before the probe (`settings.txt`: the image's 1, written false, read back 0;
`accessibilityDisplayShouldReduceTransparency` false in `probe.toml`), Increase Contrast off, the accent the default.

Only `probe.toml`, `settings.txt` and the environment records are kept; `CAPTURES.sha256` names every PNG of the two
controls artifacts (3700).

## What is here

- `analysis/measure_controls.py`: run 45542850219's, `--sheet toolbar_states_sheet` allowed and the titled specimens
  labelled; its outputs `os-probe-macos-{15,26}-controls/controls.toml` (the controls sheet) and
  `controls-toolbar_states_sheet.toml`.
- `analysis/fit_toolbar_bezel.py` (new; its docstring gives the method): the shape the toolbar-bezel button draws in
  the real toolbar (box, edge pixels, corner radii, whether a backdrop reaches it), at rest, hovered and pressed;
  `os-probe-macos-{15,26}-controls/toolbar_bezel_fitted-controls.toml` (the controls sheet's toolbar button) and
  `toolbar_bezel_fitted-toolbar_states_sheet.toml` (every specimen).

Every read below is in the window's pixels at backing scale 1 (a point is a pixel), light / dark (aqua / darkAqua).

## The toolbar states sheet

A titled window made as the controls sheet's (title bar opaque, `styleMask_fullSizeContentView` and
`titlebarAppearsTransparent` false), its toolbar holding five specimens, each the view of an item of its own with a
flexible space item between each two, each built as the controls sheet's toolbar button (`NSBezelStyleToolbar`,
"Button", sized to fit): `toolbar_button` (that button again, momentary push-in), `toolbar_button_disabled`
(`setEnabled: NO`), `toolbar_toggle_on` and `toolbar_toggle_off` (`NSButtonTypePushOnPushOff`, state on and off),
`toolbar_image` (the symbol image, empty title, `NSImageOnly`). The flexible spaces did not spread the items: AppKit
placed them side by side at the toolbar's right end (macOS 15 every 72 px, macOS 26 every 73 px), and each draws its
own bezel (the captures show five separate shapes on both images). Every specimen's view is found and none is in
error; per appearance the buttons read back as built (`specimens`: the on toggle's state 1, the disabled one's
`isEnabled` false), and the on toggle's state is 1 again after each press (`state_after_reset`). The title bar is
opaque under every specimen: in every state the captures over the three backdrops equal the capture over the window
there (`backdrop_reaches` 0, 0, 0), so each read is the composite over the title bar, opaque, as the existing
`toolbar_button` rows are; no layer is solved from it.

**The toolbar button is the same in both sheets and the same as in runs 45537062519 and 45542850219:** its `fill` and
`label` at rest, highlighted, hovered and pressed are equal in the controls sheet, in the states sheet and in run
45537062519's `controls.toml` on both images; the shape reads (`toolbar_bezel_fitted-*`) are equal in both sheets.

### macOS 15

- **The rest ring is the bezel's outline.** At rest the titled specimens (`toolbar_button`, the disabled one, the off
  toggle) draw a 1 px line, #dfdfdf / #414141, on a box of 55 x 22 px (from 1 px inside the view's left edge and 1 px
  below its top) with corners of radius 4.75–5.0 px (`edge_radius`, rms 0, all four corners), inside it nothing (the
  title bar, #ececec / #343434). Hovered and pressed, the bezel fills exactly that box with exactly those corners
  (#dfdfdf / #414141 hovered, #d2d2d2 / #4e4e4e pressed), and no line of another colour lies on its edge. So the ring
  is the outermost pixel of the bezel's own shape drawn at rest, in the hover fill's colour, which the hover fill then
  covers: an outline of the bezel, not a separate ring round it. The image-only specimen draws none at rest (its box
  is the glyph's, 8 x 11 px), and its hovered fill has the same shape (40 x 22 px, radius 4.75–5.0). Not verified:
  whether a setting the probe does not read ("Show toolbar button shapes") draws the ring; the image-only button's
  rest without it, on the same image, says the ring belongs to the titled button.
- **Disabled:** the ring as at rest, no fill; the label #acacac / #585858 (enabled #606060 / #b4b4b4).
- **The on toggle:** at rest a fill of A's hover colour, #dfdfdf / #414141, over the same 55 x 22 px box, radius
  4.75–5.0, no outline of another colour, label #424242 / #ececec; hovered #c6c6c6 / #5a5a5a, label #343434 /
  #efefef; pressed #b9b9b9 / #676767, label #343434 / #f0f0f0 (the press held, `key_at_capture` true;
  `diff_from_normal` 1198 px).
- **The off toggle at rest is A:** 0 pixels of its view (grown by 3 px) differ from A's, both appearances.
- **The image and no title:** at rest no fill and no ring, the glyph's farthest pixel from the title bar #676767 /
  #aeaeae; hovered A's hover fill, #dfdfdf / #414141, glyph #666666 / #aeaeae.

### macOS 26

- **The rest capsule:** fill #ffffff / #252525 over the view's whole width, 57 x 35 px from y 9 (light; its top edge
  is 7 levels from the title bar) and 57 x 36 px (dark); corner radius fits light 15.5–16.0 (top) and 16.75–17.0
  (bottom), rms 0.24–0.49, dark 16.25–17.5, rms 0.24–0.33; the on toggle, a box of the same view size (57 x 36 px) at a
  contrast of 228–248 levels, fits 17.75–18.0 at every corner: half the 36 px height, a capsule. The rest fits lie under 18 where the
  contrast is 7 (light) or 10 (dark) levels, so the edge's half-coverage step is coarse there. Outside it the title
  bar, #fcfcfc–#f9f9f9 (light, top to bottom) / #1b1b1b.
- **The rim (dark only):** a 1 px line on the capsule's edge, #3e3e3e at rest (then #262626, then the fill), #525252
  hovered over #393939, #616161 pressed over #484848: in each state the fill plus 25 levels. In light the edge pixel
  is the fill (#ffffff, #f2f2f2, #e6e6e6). Not settled as one fixed layer: no backdrop reaches it, and as one
  source-over white layer it would need alpha 0.115, 0.126 and 0.137 over the three fills.
- **Disabled:** the rest capsule and rim unchanged; the label #bfbfbf / #666666 (enabled #4d4d4d / #e9e9e9).
- **The on toggle:** the capsule filled with the accent, #007aff in both appearances, label #ffffff; hovered #006df2 /
  #148eff, pressed #0061e6 / #239dff, the label #ffffff throughout; no rim (the edge pixel within 9 levels of the
  fill, its antialiasing).
- **The off toggle at rest is A:** 0 / 1 pixels differ.
- **The image and no title:** A's fills over its own 39 x 36 px capsule, #ffffff / #252525 at rest (with the dark
  rim), #f2f2f2 / #393939 hovered; the glyph #4d4d4d / #e9e9e9 at rest, #494949 / #ebebeb hovered: A's label colours.
