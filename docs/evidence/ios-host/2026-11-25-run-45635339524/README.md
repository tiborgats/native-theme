# iOS host, run 45635339524 (2026-11-25): the iPad pointer hover (U113), read by the app

The hover pass of `tools/ios-host` (M2.FS4c12, the second approach) on branch `m2-fs4g`, commit `a15fd2727`,
dispatched by hand through `.github/workflows/os-probe.yml` (jobs `ios-host`, `ios_host_passes=hover`,
`ios_host_set=lists`, `ios_host_device=ipad`, images `["macos-15", "macos-26"]`):
<https://github.com/tiborgats/native-theme/actions/runs/45635339524>, **red on both images**: the pass fails
where a target got no read that counts, as designed. Simulators: iOS 18.6 (22G86) on macos-15 (Xcode build 16F6)
and iOS 26.5 (23F77) on macos-26 (Xcode build 17F113), iPad Pro (12.9-inch) (3rd generation).

The run before it, 45632490596 on `28149fd57`, recorded nothing: the UI test hovered the target through the element,
which fails the test once the app takes the identifier away, so the fix moved the hover to a point of the app.

## What the pass does now

The app reads each target itself once the UIHoverGestureRecognizer on the specimen's view begins, without waiting
for a simulator screenshot. The UI test hovers the target again and again until the app arms the next one. Each
read is one `[[hover_read]]` in `hover{1,2}/index.toml`:
- a rest read, taken when the target is armed;
- reads asked for 0, 0.05, 0.1, 0.2, 0.3 and 0.5 s after the pointer arrived. Each read draws the window, so the
  actual times (`t_s`) run later, the first at 0.0-0.15 s.

Each read records:
- the window drawn over the cell (drawHierarchy without a screen update, `draw/`, not committed);
- the label nearest the hovered point: its properties, its box, its ink (the pixel farthest from the box's
  commonest colour) and that commonest colour, and its pixels changed from the rest read;
- the pixels changed anywhere in the cell;
- every view in the window whose class names a pointer effect, with its layers.

A target counts as read where the pointer was still over it at a read 0.1 s or later; otherwise the read is kept
and the target is retaken.

Kept here:
- `ios-host-macos-{15,26}/hover{1,2}/index.toml`;
- each job's `run.txt` and `environment.txt`;
- `hover_table.py` and its output, `hover-table.txt`: per version, part, appearance and backdrop;
- `hover_aggregate.py` and its output, `hover-reads-summary.txt`: per read.

`CAPTURES.sha256` names the 310 PNGs of the two artifacts, every one a drawHierarchy read under `draw/`. No
screenshot was taken.

## What the run shows

- The pointer stays 0.001-0.62 s (`pointer_left_after_s`). Of 44 reads, 15 had the pointer still over the target
  at a read 0.1 s or later. The other 81 of the 96 targets (24 per pass, two passes per image) are
  `[[hover_not_seen]]`: no read that counts within 60 s of being armed.
- **Label colour.** In every read with the pointer over the target, the label's ink is its rest ink. This holds
  for every part, both appearances, every backdrop and both versions, with one exception: iOS 26.5's bar item over
  black, whose ink dims from #f4f3f4 to #eeedee and then #e4e4e4 under the lens. The label's `textColor` never
  changes, and neither does `isHighlighted`, its alpha or its layer opacity.
- **Segmented control.**
  - iOS 18.6: a `_UIPointerEffectPlatterView` lifts the hovered segment through `_UIPortalView`s, at scale up to
    1.070. No fill shows in the label's box: its commonest colour stays at the rest colour, #eeeeef in light (the
    system and white backdrops) and #1c1c1f and #dedee1 in dark (the system and white backdrops). A plus-lighter
    `UIView` appears in the effect. No layer or view of any pointer effect has a background colour, in any read
    on either version.
  - iOS 26.5: a `_UIPointerLensView` fades in over the segment as well. It is a 55 x 28 pt capsule (corner radius
    14 pt) built from a `CABackdropLayer` with a filter and SDF layers, so it is glass, not a fill. Its opacity was
    still rising when the pointer left (up to 0.94). The label's box changes with it:
    - light: #eeeeef to #ddddde over the system backdrop and #dcdcdd over white;
    - dark: #1c1c1f to #2c2c2f over black and #2a2a2d over the system backdrop, and #dedee1 to #e0e0e3 over white.

    The lens had not settled.
- **Toolbar button (`bar_plain`).**
  - iOS 18.6: a platter and a `_UIPointerInteractionAssistantEffectContainerView` lift the item. The label's ink
    stays #007aff and the box's commonest colour stays #f7f8f7.
  - iOS 26.5: a lens fades in. Over white, the box goes from #fdfdfd to #eaeaea while the ink stays at
    #191918-#171716. Over black, see the exception above. It had not settled (the pointer stayed up to 0.46 s).
- **Sidebar row and outline header.** No pixel of the cell changes in any read, and no pointer-effect view covers
  the cell, on both versions. Some reads list a lens view; it lies elsewhere in the window, and the cell's changed
  pixels are 0. The pointer stayed up to 0.105 s (outline header, iOS 18.6), 0.218 s (outline header, iOS 26.5)
  and 0.009 s (sidebar row, both).

Not settled:
- the settled look of iOS 26.5's lens: its colour at full opacity, with the pointer resting;
- whether the sidebar row draws any hover. The pointer stayed at most 0.009 s on it, too short to tell;
- whether the outline header draws a hover after 0.22 s;
- the dark appearance's toolbar item on both versions, and the dark outline header and the sidebar row over the
  system backdrop on iOS 18.6: no read.
