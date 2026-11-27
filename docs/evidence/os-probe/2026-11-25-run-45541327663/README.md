# OS probe, run 45541327663 (2026-11-25)

The button, motion, events and focus-transition passes of `tools/os-probe` on branch `m2-fs4e`, commit `e1140cda`
(M2.FS4c9's head: the button pass with `hover_moves`), images macOS 15 and macOS 26 (`environment.txt` the same as run
45537062519's). Dispatched by hand through `.github/workflows/os-probe.yml` with the inputs of the two runs that lost
the macOS 26 glass hovers, 45476265646 and 45489450935 (job `probe`, passes `button,motion,events,focus-transition`,
images `["macos-15", "macos-26"]`): <https://github.com/tiborgats/native-theme/actions/runs/45541327663>, green on
attempt 1, all eight jobs.

Only the button jobs' `probe.toml` and environment records are kept; `CAPTURES.sha256` names every PNG of the two
button artifacts. The motion, events and transition jobs' records and recordings stay in the run's artifacts: no read
here is taken from them.

## What is here

- `analysis/hover_moves-macos-26.txt`: run 45537062519's `analysis/hover_moves.py` over this run's macOS 26 button
  pass.

## What the run reads

- **The macOS 26 `styles_b` glass hovers are kept beside the motion, events and focus-transition passes.** The
  regular `hovered` state and each of the three ways of moving the cursor read run 45456990497's figures (aqua 590,
  1061, 1500, 1758, 2503 pixels differ from rest, by at most 10; darkAqua 594, 1090, 1548, 1808, 2552, by at most 22 or
  23) and a mouse-entered on every glass button (40 of 40 lines); `hover_moves-macos-26.txt` is line for line run
  45537062519's. So the pattern FS4c9 noted (the runs that lost the hovers ran the button pass beside those passes)
  is not the cause: with the same job set the hovers are kept. Each pass of each image is a job of its own on a
  runner of its own (the workflow's `probe` matrix), so a neighbouring pass shares no machine with the button pass in
  any run. **The loss is not reproduced in two runs (45537062519 and this one), its cause not settled.**
- `hover_moves` stays: it is the one record that would tell, in a run that loses the hovers again, whether any way of
  moving the cursor keeps them; it costs 30 captures per appearance pair of the button pass.
