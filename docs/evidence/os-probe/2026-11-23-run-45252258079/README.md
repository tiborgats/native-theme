# OS probe, run 45252258079 (2026-11-23)

The controls pass of `tools/os-probe` on branch `m2-fs4c` at commit `0a98826c`: run 45242497701's pass (Reduce
Transparency off, the scroll views without a background) with the scrollers shown before each hovered and pressed
capture of a knob by a line scroll down and one up, posted at the cursor on the probe's own window, the view scrolled
to its middle again, then `flashScrollers` (`<state>.<target>.<backdrop>.reveal`); dispatched by hand through
`.github/workflows/os-probe.yml` (jobs `probe`, passes `controls`, images macOS 15 and 26):
<https://github.com/tiborgats/native-theme/actions/runs/45252258079>. Green: both probe jobs passed on the first
attempt, the iOS host job skipped. GitHub deletes the run's artifacts after 90 days; this folder keeps them whole, as
the run uploaded them (PNGs byte for byte). The images, Xcodes and Rust are run 45242497701's (its README's table);
Reduce Transparency off as there (`button-pass-controls/settings.txt`: image's 1, read back 0;
`accessibilityDisplayShouldReduceTransparency = false`). Both jobs captured 410 PNGs.

## What is here

Run 45242497701's layout; `controls.toml` and `controls-toolbar_sheet.toml` per image by run 45226132792's
`analysis/measure_controls.py`. `analysis/`: `compare_all.py` (run 45242497701's) and
`compare_all-run-45242497701-macos-{15,26}[-toolbar_sheet].txt`; `check_grey_in_part.py` (run 45242497701's) and
`check_grey_in_part-macos-{15,26}-scroller_knob_{legacy,overlay}.txt`.

## What the run reads (the facts are measured.md's)

- **The overlay knob is drawn** in every hovered and pressed capture on both images (`knob_drawn.differing_pixels`
  468 on macOS 15, 228 on 26; no state error), and every press of it tracked (`wait_steps` 1). The scroll left the
  knob in the middle (`verticalScroller_doubleValue_after_scroll` 0.5 everywhere). At rest (`normal`, no scroll) it
  is not drawn (`drawn_box` none).
- Both knobs' layers composite linearly inside the knob's part (0 pixels off by more than 2 levels over grey and over
  the window, both images, both appearances).
- Against run 45242497701 every read is the same but the overlay knob's hovered and pressed ones (and one pixel
  count of macOS 26's toolbar sheet, `diff_from_normal` 1832 → 1834): the posted scroll changes nothing else.
- Not seen: an overlay knob shown but not hovered (the scroll shows it with the cursor already over it), so whether
  the hovered knob differs from a merely shown one is not read here; the knob's part is the scroller's
  `rectForPart:` read at rest, and the drawn box fills it (macOS 15 [428, 360, 12, 39], 26 [431, 379, 6, 38]), so
  pixels an expanded knob draws outside it are not read.
