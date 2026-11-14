# 0.7.1: the KDE fixes from issue #4 — implementation plan

- Rationale: [`../issues/0004_kde-hand-overrides-rationale.md`](../issues/0004_kde-hand-overrides-rationale.md)
  (approved 2026-11-13). Specification:
  [`todo_v0.7.1_kde-hand-overrides-spec.md`](todo_v0.7.1_kde-hand-overrides-spec.md)
  (Parts A to E; checked in three review rounds and a confirming check).
- Written: 2026-11-14. Status: **implemented on 2026-11-14**, on `v0.7.1-rc2`
  (commits `46e0ed6e` to `431673d9`, then the close-out commit that moved this
  plan and the specification here from `docs/`). The maintainer approved it the
  same day, with the version kept at 0.7.1 and the specification's open points
  1 and 2 decided as the specification has them (§7). Two departures from the
  text below: T5 ran in two halves, the source files' citations beside D3
  (`75d4a6ec`) and `docs/todo.md`'s after the documentation merge (`3c722c1e`,
  `431673d9`); and D3's new backlog entry says the KDE reader writes the
  button's and the menu's hover text, where the specification's sentence named
  the button's only, which T2 had made untrue. The paths below are those the
  two documents had in `docs/`.
- Where: branch `v0.7.1-rc2` (pull request #10), which stands at `387fc449`, the
  commit the 0.7.1 release was prepared at. The pull request is not merged by
  this plan.

This plan says who does what, in which order, and how each task is known to be
done. What each task changes is the specification's; the plan repeats none of
it.

## 1. Shape of the work

Three lanes in three worktrees of the same repository, so that no agent's
half-done files reach another's tests or commits. The lanes meet by merges the
orchestrator makes (`git merge --no-ff`, so every commit keeps its hash).

| Lane | Worktree and branch | Tasks | Cargo |
|---|---|---|---|
| core | `.claude/worktrees/v071`, `v0.7.1-rc2` | P1, P0, T1, T2, then the merges, T5, T7 | yes |
| gpui | `.claude/worktrees/v071-gpui`, `v0.7.1-rc2-gpui`, made from P0's commit | T3 | yes, `-p native-theme-gpui` only, at low priority |
| docs | `.claude/worktrees/v071-docs`, `v0.7.1-rc2-docs`, made from P0's commit | D1, D2, D3 | none |

```
core   P1 → P0 → T1 → [merge D1, first push] → T2 ──────→ [merge gpui, docs] → T5 → T7
                 └ T1 review                  └ T2 review
gpui        T3 ───────────────────────────────→ T3 review-hard ─┘
docs        D1 ──────────┘  D2 (after T1) ────→ D3 (after T2's review) ─┘
```

Worktree paths are relative to the main checkout. Every merge into a worktree
is made while no agent works in it.

The gpui lane has a build folder of its own (Cargo keeps one per worktree
path). The maintainer accepted that (2026-11-14). When its branch is merged the
orchestrator reads the folder's path from `cargo metadata` (`build_directory`)
in that worktree, checks that no build is running, deletes the folder, removes
the two extra worktrees, and reports it.

Why this order (each from the specification):

- T1 before T2: both edit the core, and Part B's tests in `kde/mod.rs` follow
  Part A's removals there.
- T3 beside them: Part D's files are the gpui connector's own. What it shares
  with Parts A and B (`CHANGELOG.md`, one sentence of the gpui README) is merged
  by the orchestrator, by the quoted texts.
- D1 first in its lane: it writes row 717's cell and the 2026-11-13 evidence
  folder that D2 builds on. Its facts rows are merged into the core lane between
  T1 and T2, while no agent works there, so that T2's tests and preset comments
  read rows that already say what they cite.
- D2 after T1: it needs the "before" binary (P1) and the "after" binary, built
  at T1's commit.
- D3 after T2: its backlog entries cite preset keys T2 adds and name T1's and
  T2's commits; the docs branch takes a merge of `v0.7.1-rc2` first.
- T5 after every source edit is merged; T7 after T5.

## 2. Tasks

A task that changes files ends in one commit on its lane's branch (no
attribution line), made only when its done-criterion holds. Every agent stages
its own paths by name; `git commit -a` and `git add -A` are not used. Agent
types are the maintainer's delegation table's.

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| P1 | Build the egui showcase at `387fc449` with the command of C.5 (b) 3 and copy the binary to the input folder's `bin/` (§5) as the "before" binary, with a text file beside it naming the commit. No commit. | orchestrator | Part C, C.5 (b) 3 | the copy and its note exist |
| P0 | Commit the three documents (this plan's status line set to approved); create the two extra worktrees from that commit. | orchestrator | — | the five documentation checkers pass (§4) |
| T1 | Font DPI. Creates the changelog's `### Fixed` heading after `### Changed`. | `implement` | header; Part A | Part A's *Gates* commands pass, the two stamp checks excepted (§4); the new tests fail before the change and pass after; no public item is removed |
| D1 | Facts rows (all of C.1; row 717 without its measured clause), `docs/inheritance-rules.toml` (C.3), the evidence folder of 2026-11-13 (C.5 (a)). Runs beside T1. | `worker` | header; Part C: C.1, C.2, C.3, C.5 (a) | `check_platform_facts_rows.py`, `check_section_refs.py`, `check_preset_citations.py`, `check_wrong_safety_nets.py` pass; every kwin and Qt link was opened by C.1's rule; `grep -rnE '/tmp/|/home/|/run/media' docs/evidence/breeze-offscreen` finds nothing |
| — | **Between T1 and T2**, with no agent in the core worktree (T2 waits for D1 if D1 is not finished): build the egui showcase at T1's commit and copy it to `bin/` as the "after" binary with its note; merge `v0.7.1-rc2-docs` (D1) into `v0.7.1-rc2` and run the core's tests that read the two documentation files (the `resolve::inheritance` and `presets::documented_sizes` tests and the `platform_facts_xref` test); run `cargo audit`, `scripts/check_readme_versions.py`, `check_widget_coverage.py`, `check_connector_parity.py`, the gpui and iced connectors' clippy legs as `ci.yml` runs them, and a test build of the core with `kde` alone and with `portal` alone (`cargo test -p native-theme --features <one> --no-run`); CI runs all of these, Part A edits those two connectors and narrows a feature gate, and its own gates cover none of them; push `v0.7.1-rc2`; CI runs in the background. | orchestrator | Part C, C.5 (b) 3; `ci.yml` | binary and note exist; the three tests, the audit, the three checkers, the clippy legs and the two test builds pass; CI started |
| D2 | The implementation-time runs, never on the real desktop (the showcase runs on a private X server, the Qt runs offscreen): combo box popup, Qt font sizes, the showcase's capitals before and after. Writes the run folder with its README, row 717's measured clause, and the edit of `docs/evidence/README.md`, in one commit (the readings and the cell that quotes them land together, which satisfies C.5 (b)'s order). | `worker` | header; Part C: C.5 (b), the README text at the end of C.5, and C.1's row 717 for the measured clause | the run folder holds the outputs and its README; the four facts checkers pass; the privacy grep of D1 finds nothing |
| T2 | KDE values: reader, preset, tests, and the connector texts Part B owns. Not its §5 item 7 (the `atlas.rs` re-pointing), which is T5's. | `implement` | header; Part B | Part B's *Gates* commands pass, the two stamp checks excepted, the claim-line test among them; the reader's new tests fail before and pass after |
| T3 | gpui connector: the restore, `to_hsla`, the README, the `list_item_selected` doc, the upstream request drafts. Measures the exhaustive test's run time first and applies the header's rule (open point 3). Its branch has no `### Fixed` heading yet (T1 creates one on the core branch): it adds its own after `### Changed`, with its entry under it. | `implement` | header; Part D | every command of Part D's *Gates* for the gpui connector passes in its own worktree, every command with the `gpui-fast` feature and the two stamp checks excepted; tests labelled "fails today" fail before the change and pass after; the measured run time is reported |
| — | Merge `v0.7.1-rc2-gpui` (T3, reviewed, its fix included) into `v0.7.1-rc2` after T2's commit. `CHANGELOG.md` is the expected conflict: one `### Fixed` heading is kept, with the entries in the order T1's, T2's, T3's. Any other conflict (the gpui README is the other file two lanes edit) is resolved by the quoted texts and reported. Then run all of Part D's *Gates* commands on the merged tree, those with the `gpui-fast` feature and the claim-line test among them. A failure that shows only on the merged tree is fixed by T3's author on `v0.7.1-rc2` and reviewed again. | orchestrator | Part D, *Gates* | they pass, the two stamp checks excepted |
| D3 | Backlog (`docs/todo.md`, `docs/archive/todo_closed.md`), the first rationale's two sentences, `ROADMAP.md`. Not C.4 (a) 12 (the drifting citations), which is T5's. Each "*Closed … in …*" line names the commit that closed it: the last commit of T1 (its review fix included) for the font DPI items, of T2 for the KDE value items, D1's for the facts-only ones. D3 starts when T1's and T2's reviews have passed and any fix is committed; the docs branch first takes a merge of `v0.7.1-rc2`, so the checkers see T2's preset. | `worker` | header; Part C: C.4, C.6 | all five documentation checkers pass with no finding; the closed items are in the archive verbatim |
| T5 | After the docs and gpui branches are merged: the sweep of ungated line citations. Part E's list, Part B's §5 item 7, Part C's C.4 (a) 12, and the older `kde-breeze.toml` citations Part B's *Gates* section lists as moved further by its inserts. Each is found by the text it cites, not by the line number the specification gives for it, since earlier tasks have moved those lines. | `worker` | header; Part E, E.2 | every listed citation points at the item it names; `cargo check --workspace --all-targets` passes; the five documentation checkers pass; anything not found is reported |
| T7 | Close-out, E.3 steps 1 to 8, with three additions. Before step 6: a rehearsal of the parity step's layout half, without the desktop (each showcase started on the private X server with `--dump-layout`, then `scripts/check_showcase_parity.py` on the dumps; with the presets the capture scripts use; a showcase that does not run there is reported and skipped). It runs without a window manager and with software rendering, so a difference it shows is fixed only where this work's changes explain it, and is otherwise reported while the capture goes ahead. It compares geometry only; the comparator's colour samples, the scroll bar track among them, need the real captures and are first seen in step 6. Between steps 6 and 7: the orchestrator opens the new KDE captures beside the previous ones and tells the maintainer what changed. In step 7's commit: the specification and this plan move to `docs/archive/`, `docs/archive/README.md` gains their row, the links between the three documents are rewritten for the new location, and the status lines of all three say implemented. | orchestrator; the maintainer for the capture window | Part E, E.3 | the full `scripts/check_release.sh` at the last commit passes with no failure, and CI is green |

After T7 the orchestrator runs `~/.claude/scripts/sweep-target.sh` on the
worktree and reports. **Merging the pull request, tagging, publishing, the
GitHub release, the pull request's title and description, posting the reply to
the reporter and sending the four upstream requests are the maintainer's
decisions and are not part of this plan.**

## 3. Reviews

Every code commit is reviewed. The review runs while the lane's next task
works: the reviewer reads the commit (`git show`, the commit range), never the
working tree, and runs no cargo. A must-fix is a new commit by the task's
author, reviewed by the same reviewer; in the core lane it is made after T2's
commit, with Part A's gates, so that T2 is not interrupted. A second must-fix on the same point falls under §6's take-over rule.
The reviewer gets the diff and the specification part, not the author's report.

| Commit | Reviewer | Why that one |
|---|---|---|
| T1 (core: detection, readers, watcher) | `review` | mostly removals; the risk is a leftover reference or a feature combination that warns |
| T2 (reader derivations, preset, tests) | `review` | values and tests against Part B and the rationale's tables |
| T3 (the connector's observer) | `review-hard`, before its branch is merged | the restore changes the control flow of an observer that reacts to its own writes; a miss there loops or silently skips |
| D1, D2, D3, T5 (documentation, data, comments) | the checkers, then the orchestrator reads the diff | the gates validate the data; the texts were reviewed in the specification |

**A change to Part A's code after the "after" binary is built** (a review fix or
a fix CI demands): if it is confined to code compiled only on macOS or Windows,
the binary and D2's readings stand, and the run folder's README keeps naming
the commit the binary was built at while D3's closing line names T1's last
commit; otherwise the binary is rebuilt and D2's before-and-after reading is
repeated before D3 quotes it.

## 4. Gates

- **While working**, an agent runs the tests of the module it is changing.
  **Before its commit**, it runs its part's *Gates* commands once, in the
  foreground, through the build slot wrapper. Each part's table also lists the
  two release stamps; they are stale from T1 until T7 and are no task's
  done-criterion (Part E, E.1).
- **Documentation checkers** (no cargo): `scripts/check_platform_facts_rows.py`,
  `check_section_refs.py`, `check_preset_citations.py`,
  `check_wrong_safety_nets.py`, `check_todo_citations.py`.
- **`scripts/check_release.sh --commit` is not run.** Every check it makes is
  made by a part's gates on the merged tree, by the checks before the first
  push, or by the full run that precedes the second push.
- **The minimum supported Rust version** is checked by CI alone (the release
  script has no such check). The briefs of T1, T2 and T3 say: no language or
  library feature newer than the edited crate's own `rust-version`.
- **Full `scripts/check_release.sh`:** twice, both in T7: E.3 step 3, before the
  second push (one expected failure, the provenance stamp; step 2 has renewed
  the other), and step 8, at the stamped commit (none). The second runs beside
  the last CI run.
- **CI** (the pull request's checks): after the push that follows T1 (in the
  background; it is the early check of the code compiled only on macOS and
  Windows), after E.3 step 3, and after the stamp commit. A red run emails the
  maintainer; no push is made that is expected to be red. If a run is red, the
  orchestrator tells the maintainer what failed and why, and has the author of
  the failing change fix it (reviewed as in §3) as the lane's next commit, after
  the task then running there has committed; the fix is pushed once the checks
  that preceded the first push pass again.
- **While CI runs before the capture window**, the three showcases are built for
  release (E.3 step 5) and the parity rehearsal runs. This is a deliberate
  departure from E.3's "each step starts only when the one before it is green":
  the builds and the rehearsal commit nothing, and step 6 still waits for CI.

## 5. Rules every agent brief carries

- The brief gives the specification's header and the task's part. The part is
  the whole task: nothing beyond it, no refactoring, no formatting churn. A
  needed change the part does not name is reported, not made.
- The header's rules hold for every edit: an edit to a file shared between
  parts is applied by its quoted text or its test's name, and the lines the
  showcase's claim test pins do not move.
- **The worktree.** The brief names the lane's worktree by its absolute path.
  Every command starts with `cd` into it or uses `git -C` and
  `--manifest-path`. The folder the session starts in, the main checkout, is
  another branch (0.8.0) with the same relative paths: nothing is read, run or
  written there, and a check that passes there proves nothing. The agent's report quotes
  `git -C <worktree> log -1 --oneline`.
- Source files are edited with the Edit and Write tools only; a hook that blocks
  an edit is obeyed, never bypassed.
- No `unwrap`, `expect`, `panic!` or panicking index in non-test code; no
  `unsafe`. No public item is removed.
- No invented values. No code copied from KDE, Qt or any LGPL or GPL source:
  each derivation is written from the specification's description with the
  helpers the reader already has.
- Cargo runs through the slot and low-priority wrappers, in the foreground;
  `build-dir` and the target directory are never set. At most two cargo
  processes run at once in total: the core lane's and the gpui lane's.
- No `find /` or other search of the whole file system. Reads and writes stay
  inside the lane's worktree, Cargo's home and build folder, the session's
  scratch folder, and the input folder below.
- Nothing appears on the maintainer's desktop.
- A new test is written first and seen to fail. A changelog entry is one
  physical line. The agent stages its own paths by name, commits once, does not
  push, and reports once: the commit, the files, the gate commands with their
  result lines, and everything it was unsure of.
- **Something the maintainer must hear before a commit** (a link that does not
  hold in D1, a reading that differs from the rationale's in D2): the agent
  leaves the work uncommitted and reports. The orchestrator tells the maintainer
  and commits without waiting, unless the difference contradicts a decision of
  the rationale; then §6's stop rule holds.

**Inputs outside the repository.** The downloaded upstream sources and the
measurement scripts and outputs that Part C reads (the specification's
`<scratch>/issue4/`) are kept in the session's durable scratch folder, as
`<scratch-keep>/issue4-0.7.1/issue4/`, which survives a restart; the two
binaries and their notes go to `<scratch-keep>/issue4-0.7.1/bin/`. The briefs
give the absolute paths; no committed file does.

## 6. Limits and stop rules

- **Agents at once:** at most one in each lane, so three, plus one reviewer. At
  most one Fable agent (`review-hard`) at a time.
- **A gate failed twice** by the same task is not dispatched a third time: the
  orchestrator takes the task over, re-specifies it, or sends it once as
  `implement-hard`, and says which.
- **A task that needs more than its part names** (a model field, a change in
  another crate, a decision the rationale did not take) stops, and the
  maintainer is asked. The same holds if upstream moves during the work: a new
  GPUI Kit or GPUI Fast release, or a new advisory, is reported with what it
  costs before anything is changed for it.
- **A fix that the full check, the rehearsal or the asset script demands** (E.3
  names the parity step as the likeliest stop) is a code change after T5: it is
  made by an `implement` agent, reviewed by a `review` agent, passes the gates
  of the part whose files it touches, and T5's citations in every file it
  touched are checked again; then E.3 resumes at step 2. Nothing is committed
  from a stopped asset run. The maintainer is told what stopped it, what was
  changed, and whether another capture window is needed.
- **The capture window** is asked for once, when E.3's steps 1 to 5 and the
  rehearsal are green, with the measured desktop time (4 to 6 minutes at the
  last release).

## 7. The maintainer's part

**Before any code, with the approval of this plan:**

1. **The version.** The work writes "0.7.1" into rustdoc, the facts file, the
   changelog, the roadmap and the backlog, some of it under stamped paths. If
   the release should carry another number, it is decided now: the
   specification and this plan are then amended (the texts, and a version task
   before T1) and re-checked before P0. Changing it after T7 costs a second
   capture window.
2. **Open point 1 of the specification:** egui sliders outside the connector's
   Slider role lose contrast with Breeze's scroll bar handle colour (their rail
   goes from 3.7:1 to 1.4:1 against the window in light mode). The
   specification keeps the rationale's decision and adds a paragraph to the egui
   README. The alternative, holding `scrollbar.thumb_color` back, removes the
   thumb's fix from Part B (reader, preset, tests, changelog entry, the README
   paragraph) and from Part C (the backlog closing of the thumb's KDE half, the
   roadmap sentence) and leaves the thumb a backlog item; if chosen, the
   specification is amended and re-checked before P0.
3. **Open point 2:** the specification neither makes the gpui showcase's private
   colour conversion call the new `to_hsla` nor lists `to_hsla` in the README's
   *Core concepts*. That stands unless the maintainer says otherwise.

**During the work, the maintainer is told:**

- a kwin or Qt link in a facts row that does not hold at its tag (D1);
- a measured reading that differs from the rationale's (D2);
- the exhaustive test's measured run time and whether it became an ignored test
  (T3);
- a red CI run, and anything that stops a task under §6.

**Near the end:** what changed in the KDE screenshots, and one capture window
(E.3 step 6).

**After T7:** whether to merge, tag and publish; whether the pull request's
title and description are updated; whether the reply to the reporter and the
four upstream requests are sent.

## 8. Time

Measured, on this machine and at GitHub: the full `check_release.sh` takes about
23 minutes; the asset script 30 to 37 minutes, of which 4 to 6 need the desktop;
a CI run 21 to 27 minutes. The tasks' own durations are estimates from this
session's agent runs, not measurements: T1 about 1 to 1.5 hours with its gates,
T2 1.5 to 2.5, T3 2 to 2.5 beside them, the documentation tasks 30 to 45 minutes
each beside the code, the step between T1 and T2 about half an hour, the
merges and T5 about 1 hour, T7 2.5 to 3 hours. The longest chain is T1, that
step, T2, the merges, T5, T7: about 6.5 to 8.5 hours, plus any fix
a review demands and the wait for the capture window.
