# 0.7.2: issue #4's open KDE items, steps 2 and 3 (the focus ring; a button's border on hover and focus) — implementation plan

- Rationale: [`issues/0004_kde-open-items-rationale.md`](issues/0004_kde-open-items-rationale.md),
  D4 (step 2), D5 (step 3), D0 (the version). Specification:
  [`todo_v0.7.2_kde-focus-ring-and-button-outlines-spec.md`](todo_v0.7.2_kde-focus-ring-and-button-outlines-spec.md)
  ("the spec").
- Decided by the maintainer on 2026-11-18: steps 2 and 3 are part of v0.7.2, not a release
  of their own; D0 (a), the release names its break; the version stays 0.7.2 (Cargo's rule that
  a break takes a new minor version was put to him; 0.7.2 already breaks it with egui 0.37,
  which the egui connector re-exports).
- Written 2026-11-18. It replaces the plan of 2026-11-15 for a separate release (git history
  keeps it). Status: approved by the maintainer on 2026-11-18, with the rationale's D4
  and D5 and the spec.
- The record: `<record>` = `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan/`,
  0.7.2's own record, with this work's files in `<record>/steps23/`. `<tools>` =
  `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan/tools/`.

## 1. Where and from what

- One worktree, `.claude/worktrees/lane2-b`, branch `v0.7.2-rc1`, from its head `b434b9bc`
  (0.7.2 code-complete, version bumped, gated `--commit` 148/148). The units are commits on
  that branch. No new branch, no merge, no version step, no recount after a merge: the
  spec's 2K and 3I say what of theirs that removes.
- The branch was pushed at `b434b9bc` on 2026-11-18 with a draft pull request, #11 (the
  maintainer's decision, ahead of the compatibility run that 0.7.2's plan puts before the
  first push), so that CI checks the egui 0.37 and GPUI Kit 0.10.0 work on macOS and
  Windows while these units are built. After that it is pushed, by explicit refspec, at
  each `--commit` gate that passes (after U4, and after U6 with U7), and each push's CI
  run is read; a red run emails the maintainer.
- The evidence already taken (in `<record>/steps23/`): the upstream citations re-read at
  gpui-component 0.10.0, gpui-pre 0.3.12 and egui 0.37.0 (`upstream-reread.md`); every
  repository anchor at `b434b9bc` (`repo-anchors.md`); the gpui trial of 3F.1 to 3F.3 run on
  `b434b9bc` (`trial/`: it passes, both backends alike, the focused row with the band off
  included: the second run); the two reviews and the decisions taken on them
  (`decisions.md`), the second trial run with the band off and the tab bar measurement
  (`trial/`), and the band-switch measurements of the check after the push
  (`trial/band-switch*.patch` and `band-switch*-output.txt`: gpui-component's band follows
  keyboard focus through a plugin of gpui-base's `Root`, with nothing called by the
  application; a call in the root view's `render` works too; the observers-only way is a
  frame late on a click). The spec and the rationale's D4 are amended by them.

## 2. Units

The spec's units are merged where they share files and reviewers and none needs a gate of
its own between them. Each unit is one commit (a review fix is a commit of its own, gated by
the next gate).

| Unit | The spec's parts | Agent | Gate after it |
|---|---|---|---|
| D0 | The documents: the amended spec, this plan, the links to the old names (`todo_v0.7.x_kde-open-items-steps2-3-*`) in other documents re-pointed (none inside `docs/archive/`, rule 22 (a)) | orchestrator (commit) | the five documentation checkers |
| U1 | Step 2's core: units 2.1 and 2.2 (2A, 2B but 2B.4, 2C, 2G.2) | `implement` | `--quick` |
| U2 | Step 2, gpui: unit 2.3 (2E, 2H.3 but the new leaf's `HOW` entry), with the `Root` plugin for gpui-component's band (decision 18; the plugin and its tests start from `steps23/trial/band-switch-plugin.patch`), the tab bar's ring (decision 16) and the 0.7.2 tab bar's two defects the check found and measured (2E.2: the scroll to the first tab when an element round the bar takes the focus; the right and middle press that focused a tab) | `implement` | none of its own: built beside U3 and U4 |
| U3 | Step 2, iced: unit 2.4 (2F, 2H.2's showcase test) | `implement` | none of its own: built beside U2 and U4 |
| U4 | Step 2, egui: unit 2.5 (2G.1, 2G.3, 2G.4) | `implement` | one `--commit` + GPUI Fast legs over U2, U3 and U4, when the three have reported and are committed; then the push |
| U5 | Step 3's code: unit 3.1 (3A, 3B but 3B.4, 3C, 3D.1 to 3D.3, 3E, 3F.4 with its README paragraph), from `steps23/trial/trial2.patch`, with the five differences 3I lists, applied on U4's tree; the trial's test run again there, on both backends, its focused row decisive | `implement` | `--quick` + GPUI Fast legs |
| U6 | Both steps' showcases: units 2.6 and 3.2 (2B.4, 2H.2 to 2H.4's routes, 3B.4, 3D.4, 3F.4's routes, 3G.1, 3H.2b) | `implement` | none of its own: built beside U7 |
| U7 | Both steps' documents: units 2.7 and 3.3 (2I but 2I.3's reply and 2I.6, 3H.1 to 3H.4 but 3H.2b and the reply), into 0.7.2's CHANGELOG section (a *Breaking Changes* heading first, as 0.7.0's) and a new ROADMAP section `## v0.7.2` for the whole release | `worker` | the rehearsal, then one `--commit` + GPUI Fast legs + the five documentation checkers over U6 and U7 (U6 is the last code unit); then the push |

After U6's and U7's reports and before their `--commit`, on the tree that holds U6's change, the
parity rehearsal runs again (0.7.2's unit R, its spec §R: `<tools>/rehearsal.sh`, read by
`scripts/check_showcase_parity.py`, on the private X server under `xvfb97.lock`), since
U2 to U5 change the three showcases after R ran. A difference it finds that no exception
explains is fixed in code or made an exception with its reason and source, before that
`--commit`, which then covers it. Before the close-out, V's rule 22 (c) search over the stamped paths (its
command in 0.7.2's record) runs again with the two new stems,
`todo_v0.7.2_kde-focus-ring-and-button-outlines-spec` and
`todo_v0.7.2_kde-focus-ring-and-button-outlines-plan` (expected: nothing).

Then 0.7.2's close-out, unchanged but for two lines (the record's *K* steps): the archive
step moves the spec and this plan with 0.7.2's four documents, and writes the rationale's
status line (2I.6 and 3H.5 in one sentence); the asset run's parity list is read for stale
exceptions, since step 2 changes the start-up focus on kde-breeze, windows-11 and material-3
(2H.5).

Why merged: units 2.1 and 2.2 are both the core crate and small; 2.6 and 3.2 are the same
kind of edit in the same files (element lists, routes); 2.7 and 3.3 write the same sections.
Merging saves three gates and three review rounds, and a test still turns red in the unit
that makes it so (the spec's tables say which).

Why one worktree and three builders in it: U2, U3 and U4 each change one connector's
folder and nothing else (the spec's table of files: `connectors/native-theme-gpui/` with
the gpui entry of `scripts/connector_parity_exceptions.toml`; `connectors/native-theme-iced/`;
`connectors/native-theme-egui/` with `connectors/native-theme-egui-widgets/src/tests.rs`),
no connector depends on another, and all three build on U1's gated tree. So three agents
build them at once, each compiling and testing its own package (`-p`), through `slot.sh`,
which lets two cargo commands run at a time; no gate runs while they build. One builder's
half-made edit is in no other builder's build. The orchestrator commits each unit by its
paths, three commits, and one `--commit` then closes over them; a red gate there is
attributed by crate. A second worktree would give nothing more and cost a cold build (the
gpui worktree's was 65 GB, deleted after 0.7.2's merge; its build time was not measured)
and a merge back. U5 changes the core and all three connectors, so it follows the block
alone. U6 (the showcases' lists and routes) and U7 (the CHANGELOG, the ROADMAP, the
READMEs, the backlog and the rationales) share no file either and are built at once; U7
runs no cargo command but the documentation checkers. The reviews read commits, not the
tree, and run beside the next unit (§4).

## 3. Tests and gates

- During a unit: only the tests the spec names for it and the ones its edits reach, through
  `<tools>/slot.sh <tools>/lowprio.sh cargo test -p <crate> … <names>`; the showcase tests of
  the showcase it touches through `scripts/run_in_runner_env.sh`; tests that read painted
  primitives in the GPUI Fast form. A test the spec says turns red is seen red first.
- At a unit's end the orchestrator runs the gate of §2 through `<tools>/gate.sh [--fast]
  lane2-b <name> <args>`. `--commit` falls every third code unit counted from the last
  `--commit` (V's), and on the last code unit (S1): after the block of U2, U3 and U4 (the
  second to fourth code units, which end together), and after U6, the last code unit, with
  U7. So four gates: `--quick` after U1 and after U5, `--commit` twice. Measured in 0.7.2:
  `--quick` 1 to 12 minutes, `--commit` 7 to 21.
- Before each unit's report: `scripts/check_strict_panic.sh` for the crates it changed, and
  `<record>/z1/stand-in-check.sh` on a temporary index that holds the unit's own paths (as
  in 0.7.2's briefs; in the block of three, a builder adds only its own connector's
  paths). For U5 it also has to catch what 3I removes from the trial's patch: the word
  "trial", which the script catches in any case of its letters (read 2026-11-18), and the
  `band_off` edit, which it does not: U5's brief checks it with `git grep band_off`
  (expected: nothing).
- The whole suite and the release checks run once, in 0.7.2's close-out (its full check).

## 4. Reviews

Every code unit is reviewed beside the unit that follows it (U2, U3 and U4 by three
reviewers at once, beside U5; U5 beside U6 and U7; U6 and U7 before their `--commit`), by
size, as 0.7.2's plan does: by a `review` agent where its diff is over a few hundred
lines, and for the focus-state machinery (U2, U3, U4) and U5's three connectors; by the
orchestrator for the rest, U1 (data the checkers validate, round a macro group and the
reader), U6 and U7 (routes, lists and prose that the checkers and the showcase tests
hold). A reviewer is given the commit and the spec's parts, not the author's report. A
must-fix is a commit of its own before the next gate; the last `--commit` closes over
every one of them.

## 5. Briefs

The form of 0.7.2's unit briefs: the spec's parts quoted by name, the decisions that touch
them, the anchors' lines at HEAD from `repo-anchors.md` as a lookup (an anchor not found as
the spec quotes it is a stop), the wrappers, the tests, and the rules: no sub-agents, no
`find /`, nothing read outside the worktree, the registry and the record; no `unsafe`, no
panic-prone code outside tests (the project's hook skips `#[test]` but not `#[gpui::test]`:
test code there uses `let … else`), no LGPL code; a hook's refusal is reported, never written
another way; new repository citations name the document only; no commit (the orchestrator
commits after reading the report); one report at the end. In the block of three and in
U6 beside U7, the brief also says: two other agents (one other) edit other folders of the
same tree at the same time; change, format (`cargo fmt -p <crate>`) and stage nothing
outside your own paths, which the brief lists; `git status` shows their files too.

## 6. Stops

A decision neither the spec nor `decisions.md` makes is a stop. The spec's fallback (2J)
is not a stop: where a connector's keyboard-only rule cannot be built as written, the unit
builds the fallback and the record says why. A failure of the `Root` plugin's tests (the
spec's 2E.4) is a stop, not a fallback (2E.1, 2J): it was measured. The trial's fail branch (3F.5) applies where U5's
run of the test fails.

## 7. The maintainer's part

The approval of D4, D5, the spec and this plan; the capture window of 0.7.2's close-out
(once, 4 to 6 minutes); the pull request's merge, the tag, the publication and the GitHub
release.

## 8. Cost check

- **What could refute the design, first:** whether gpui can draw a button's hover border
  (the trial: done, passes) and whether the upstream lines still say what the spec builds on
  (re-read: two claims false, both the new `Tab` ring, decided in `decisions.md`); and
  whether gpui-component's band can follow keyboard focus (measured: yes, by a plugin of
  `Root`). Left open: nothing of the design; U5 runs the kept test on step 2's tree first, as a
  regression check (3F.5). What waits: U5's showcase parts wait for that run; U1 to U4
  wait for nothing.
- **Built twice:** nothing. The trial's patch is the start of U5.
- **In parallel:** the document checks ran as five agents at once. In the build: U2, U3
  and U4 by three builders at once in the one worktree (§2); U6 beside U7; the reviews
  beside the next unit, three at once for the block. At most four agents and two cargo
  commands at a time.
- **What each agent reads:** its parts of the spec, `decisions.md`'s sections on them, the
  anchors' lookup; not the rationale.
- **Tests per round:** the unit's own; `--quick` twice, `--commit` twice (§3; the plan of
  2026-11-18 morning had six gates); R once more, after U6; the full suite once, in the
  close-out.
- **Time (estimate, not measured):** U1, the block of three (as long as its longest unit,
  gpui's), U5, and U6 with U7, each about 30 to 75 minutes, and four gates, so 3 to 5
  hours, where one unit after the other was 4 to 7; then 0.7.2's close-out (its full check
  about 23 minutes, the asset run 30 to 37, CI 21 to 27, measured at 0.7.1).
- **Compaction:** done on 2026-11-18 after the check; again before the close-out, and earlier
  where the conversation passes about 300 k tokens.
