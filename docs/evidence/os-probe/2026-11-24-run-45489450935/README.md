# OS probe, runs 45476265646 and 45489450935 (2026-11-24)

Two runs of the button, motion, events and focus-transition passes of `tools/os-probe` on branch `m2-fs4c`, images
macOS 15 (15.8.2, image 20261118.0550.1) and macOS 26 (26.7.2, image 20261117.0516.1), the images of runs 45345116763
and 45456990497. Dispatched by hand through `.github/workflows/os-probe.yml` (job `probe`, passes
`button,motion,events,focus-transition`):

- **Run 45476265646** (<https://github.com/tiborgats/native-theme/actions/runs/45476265646>), commit `cdd38830`
  (M2.FS4c8 commit A, `16f5cd73`, and the park point moved to the bottom margin): every pass parks on the middle of
  the content view's bottom margin; every move and every post point of a timeline checked on the probe's window
  (`window_at_target_is_ours`, `post_points_checked`, `park_quartz_xy`), `away_point` gone. Green on attempt 2. Its
  records are in `run-45476265646/`, with `run-45476265646/analysis/compare_timeline-run-42328425189-*.txt`: its motion
  and events passes against run 42328425189's (commit `00609f6a`'s evidence on `v0.8.0-rc1`, the last run of those
  passes), by `analysis/compare_timeline.py` before its four transition keys were added.
- **Run 45489450935** (<https://github.com/tiborgats/native-theme/actions/runs/45489450935>), commit `60ac80e4`
  (M2.FS4c8b): each pass parks at its own point (`Parking`, `park_point`): the button and controls sheets on the middle
  of the left margin again (10 pt in, half the height up: run 45456990497's point), the motion, events and transition
  passes on the middle of the bottom margin; a window that refuses a move or a park is named in the error by its
  owner, layer and bounds (`CGWindowListCopyWindowInfo`). Green on attempt 2: the macOS 15 events job was cancelled
  before it got a runner on attempt 1 ("The job was not acquired by Runner of type hosted even after multiple
  attempts"; no steps run) and passed on attempt 2. Its records are in `os-probe-macos-{15,26}-<pass>/`.

Only the `probe.toml`, `settings.txt`, `ffmpeg.txt` and environment records are kept, of both runs: no row is read
from their captures and recordings, which stay in the runs' artifacts.

## What is here

- `analysis/check_parking.py` and `check_parking.txt`: every move and park of every `probe.toml` here, how many on the
  probe's window, the park points and the views hit there, the motion pass's `park_quartz_xy`, and each move's error.
- `analysis/compare_timeline.py` (the run-to-run reads left out now include the transition pass's
  `focus_returned_ms`, `unfocus_returned_ms`, `recording_ended_ms` and `key_wait_steps`) and
  `compare_timeline-run-45476265646-macos-{15,26}-{motion,events,focus-transition,focus-transition-reduce-motion-off}.txt`.
- `analysis/compare_button.py` (run 45456990497's, unchanged) and `compare_button-run-{45456990497,45476265646}-macos-{15,26}.txt`.
- `analysis/hover_events.py` and `hover_events-macos-26.txt`: the mouse-entered and mouse-exited events dispatched
  while each button was hovered, per sheet, in runs 45345116763, 45456990497, 45476265646 and 45489450935.
- `analysis/drag_trials.py` and `drag_trials.txt`: the events pass's drag-start trials of runs 42328425189,
  45476265646 and 45489450935.

## What the runs read

- **The park points** (`check_parking.txt`). Run 45489450935: every move and park of the button, motion and events
  passes on the probe's window without an error (macOS 15: 184 in the button pass, macOS 26: 282), the content view hit
  at every park point: (10, 160) on the push sheet and (10, 216) on the style sheets, (260, 10) in the motion window,
  (280, 10) in the events window, (300, 10) on the transition pass's push sheet; the motion pass's `park_quartz_xy`
  (460, 558) in both appearances on both images, so every recording was made.
- **The motion, events and transition passes against run 45476265646** (`compare_timeline-run-45476265646-*`): beside
  the method texts, the motion pass reads the same on both images; the events pass differs in the macOS 15 0 ms-hold
  drag (below) and on macOS 26 in the drag-over's count of `tableView:validateDrop:` calls (aqua 11 -> 10, darkAqua
  11 -> 12; run 45476265646 against run 42328425189: 10 -> 11, 10 -> 13 on macOS 15 and 12 -> 11 on macOS 26, so it
  varies from run to run); the transition pass reads the same but for the macOS 15 Reduce Motion off pass's park error.
  Against run 42328425189 (`run-45476265646/analysis/`), run 45476265646's motion and events passes differ, beside the
  method texts and the `validateDrop:` counts above, only in the moves that went right of the window then (`move away:
  posted at (820.0, ...)`) and park inside it now, the `post_points_checked` and `park_quartz_xy` reads added, the tool
  tip's `moved_away` reads gone, the warm tool tip trial's `visible_before` (a tool tip panel visible before its move
  in run 42328425189, none now; why is not verified), and the 0 ms-hold drag.
- **A window refusing a park.** On macOS 15 with Reduce Motion off, the transition pass's park is refused in both runs:
  `windowNumberAtPoint:` at the park point (500, 558 in Quartz points; 300, 10 in the content view, where `hitTest:`
  gives the content view) gives another window than the probe's (40): window 16 in run 45476265646 (not named then),
  window 12 in run 45489450935, `owner "Finder", layer -2147483603, bounds [0.0, 0.0, 1024.0, 768.0]`: a window of
  Finder over the whole 1024 x 768 screen, at a layer far below the normal windows', so the window server found no
  window of the probe's above the desktop at that point when the park was read, though the probe's window frame
  (200, 200, 600, 348) holds it and the same pass with Reduce Motion on parks there. Why (the window's opening
  animation, which Reduce Motion off keeps, still running when the park is read, for one) is not verified. The pass
  records its recordings all the same; only the park is skipped, as its error says.
- **The macOS 26 glass buttons' hovers did not come back with the left-margin park point**
  (`compare_button-run-45456990497-macos-26.txt`, `hover_events-macos-26.txt`). At run 45456990497's park point, (10,
  216) on `styles_b` in both runs, the five glass buttons of `styles_b` hovered differ from their rest in 1 to 14 pixels
  by 1 byte (aqua) and in none (darkAqua), against 590 to 2552 pixels by 10 to 23 in run 45456990497, as in run
  45476265646, which parked on the bottom margin. In runs 45345116763 and 45456990497 AppKit dispatched a mouse-entered
  and a mouse-exited while each glass button was hovered; in runs 45476265646 and 45489450935 none, on any button of
  any sheet. Every other leaf of both macOS 26 button passes reads the same as run 45456990497's but the method and
  `states.normal` texts, whether the caret blinked between two captures (`caret.darkAqua.capture_3`), the glass
  buttons' and `mini-flat`'s mouse-entered and mouse-exited counts and the waits of the push sheet's clicks
  (`wait_steps`, 1 to 3 steps longer); the macOS 15 button pass reads the same as both earlier runs' but those texts. The image,
  the OS build, the environment records (`accessibility.txt`, `environment.txt`, `wallpaper.txt` but its LastUse
  times) and the park point are the same as run 45456990497's, and the button pass's moves are made as there (the
  commits since add only checks after a move); what stops the mouse-entered events is not settled.
- **The macOS 15 0 ms-hold drag** (`drag_trials.txt`). Of the drag-start trials only `hold_before_distance."0 ms"` on
  macOS 15 differs between the runs: a drag session started in run 42328425189 (drag pasteboard changed once, the
  selection kept, `0 +19`) and in run 45489450935, not in run 45476265646 (no change; the selection `1 +2`: the press
  started a new selection at index 1 and the 12 pt drag extended it by two characters). The press point (232, 276),
  the trial before it (10 pt after an 800 ms hold, a drag started in all three), `makeFirstResponder`, the created
  events, the single pump step and `NSDragAndDropTextDelay` (absent) are the same in all three, and runs 45476265646 and
  45489450935 ran the same code on the same image (20261118.0550.1); every other hold and every macOS 26 trial started
  the drag. So the 0 ms trial, whose mouse-down and first mouse-dragged are posted from the second thread with no time
  between them, starts the drag in some runs and not in others; why is not settled. The drag threshold (4 pt) and the
  absence of an earliest drag time (run 42328425189's reading) rest on the other trials and hold.
