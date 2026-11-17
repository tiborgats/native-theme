# 0.7.x: issue #4's open KDE items, steps 2 and 3 (the focus ring; a button's border on hover and focus) — implementation plan

- Rationale: [`issues/0004_kde-open-items-rationale.md`](issues/0004_kde-open-items-rationale.md)
  (D4 for step 2, D5 for step 3, D0 for the version, D10 for what a step carries
  outside the code, the fallback of step 2 and the order of the releases).
  Specification: [`todo_v0.7.x_kde-open-items-steps2-3-spec.md`](todo_v0.7.x_kde-open-items-steps2-3-spec.md)
  (Step 2's parts 2A to 2K, Step 3's parts 3A to 3I; checked, final).
- Written: 2026-11-15, against `v0.7.1-rc2` at `0fa28027`, the specification's
  commit. Every `file:line` here is `0fa28027`'s unless it names another commit or
  version; the
  tasks find each by its text on the tree they edit. Status: **not approved**.
  Nothing below starts before the maintainer approves the rationale, the
  specification and this plan, and answers §7's points.
- The release: **steps 2 and 3 are one 0.7.x release, `0.7.N`** (the maintainer's
  decision of 2026-11-15, `docs/todo_v0.7.1_batch-speed-rationale.md`, S5; D0 (a),
  the specification's basis), in item 2 of *The releases after v0.7.1* (the
  rationale's D10): one branch, one version, one CHANGELOG section, one ROADMAP
  section, one close-out, one asset run and one archive at the end; each step keeps
  its own units, trial, reviews and record. Its number and its place in the order
  are the maintainer's; the release before it in the maintainer's order is tagged
  `<prev-tag>` (under D10's proposal egui 0.37's, `v0.7.2`, which comes after v0.7.1
  and its step 1), and the release before it in its own lane, the KDE lane, is
  v0.7.1. Where this plan
  writes `0.7.N`, the specification's text has `0.7.3`, the proposal (D0 (a), with
  D10's order and S5's pairs) and a placeholder until the maintainer decides D0 and
  the order, and the version the maintainer gives is written.
- Where: branch `v0.7.N-rc1`, made from `<lane-pred-head>`, the gated head of the
  release before this one in its own lane (the KDE lane: v0.7.1 → steps 2 and 3 →
  steps 4 and 5 → the caret → steps 6 and 7, S3 as amended; this release's lane
  successor is steps 4 and 5; here v0.7.1's, KDE step 1 shipping in v0.7.1 (D10 (a),
  the maintainer's answer of 2026-11-15), once that gated head has passed its gate and
  its reviews are closed, rule 20), not from its tag; and merged with `<prev-tag>`, the tag of the release immediately before
  this one in the maintainer's release order, which may be another lane's:
  `git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>`
  once, concluded by `git commit` (rule 17),
  after this release's last unit (U3.3) and its
  reviews are closed and the tag exists, before X, V and the close-out (S2 as amended
  on 2026-11-15: a merge, not a rebase, so every sha stays; task MG). egui 0.37's
  release, `v0.7.2`, stands outside the two lanes (S3's paragraph "v0.7.2 (egui
  0.37)"): its plan, not written yet, names its own worktree and is bound to rules 2,
  8 and 24; this release waits at its MG for the `v0.7.2` tag (rule 18). This plan merges into its own branch only at MG,
  tags and publishes nothing, and opens only the draft pull request the maintainer
  approved at the branch's first push (§7 item 3); merging it is the maintainer's.
- The batch's working rules are cited as "rule N (batch-speed rationale, Appendix)",
  the consolidated list at the end of `docs/todo_v0.7.1_batch-speed-rationale.md`;
  below, "rule N" or "the batch's rule N" is that list's rule N, its authoritative text;
  superseded rules are not cited. No citation is re-pointed (S6): a citation this
  plan's tasks write names the document, not a line of a tracked file that changes,
  and the existing ones are left to drift; this plan's own instructions cite
  `0fa28027`'s lines as pinned.
- The gates follow the batch's cadence (S1, §3): `--quick` at each unit's end,
  `--commit` where §3's table says; `--quick` is defined by
  `docs/todo_v0.7.1_gate-speed-spec.md` (being written when this plan was; what it
  runs is that document's, not this plan's).
- Placeholders: `0.7.N`, `<prev-tag>` (the tag of the release immediately before
  this one in the maintainer's order, which MG merges), `<lane-pred-head>` (the
  gated head of the KDE lane's previous release, which the branch is made from:
  v0.7.1's (MK4), D10 (a) being the maintainer's answer of 2026-11-15; rule 18), `<merge>` (MG's merge commit), `<date>`, `<sha …>`, `<record>`, `<tools>`, `<locks>`,
  `<scratch-keep>`, `<cargo-build>`, `<core worktree>` (§5), and every count,
  line, time and result read at a run.
  Each is filled by the task that reads it, from its run; the orchestrator
  writes it into the running record (§8); none is estimated.

This plan says who does what, in which order, and how each task is known to be
done. What each task changes is the specification's; the plan repeats none of
it.

## 1. Shape of the work

The specification's units are 2.1 to 2.7 for step 2 (2K) and 3.0, 3.1, 3.2, 3.3
for step 3 (3I), in its order. This plan adds the tasks around them (the tools, the
branch, the documents, the anchors, the merge, X, the version, the close-out).

The release has **one lane**, the core worktree, which is the only place this plan
builds; step 3's units follow step 2's on the same branch. Everything else runs
beside it without a worktree of its own: the anchors read the branch with
`git show` and write only into `<record>`; the reviewers read commits or, for the
trial, a saved patch. The batch's second lane (the widget icons alone, S3 as
amended) runs in its own worktree beside this one and shares the machine's limits
(§6).

| Lane | Worktree and branch | Tasks | Cargo |
|---|---|---|---|
| core | `<core worktree>`: `.claude/worktrees/kde-b`, with `v0.7.N-rc1` made in it from `<lane-pred-head>` (*The build folder* below) | P2, P0, U2.1 to U2.7, U3.0, U3.1, U3.2, U3.3, MG (VF inside its step (3)), X, V, K1 to K8 | yes: the only lane of this plan that runs it |

P1 (the tools) runs before P2. A2 (step 2's anchors) and A3 (step 3's) run beside
the lane and commit nothing. P0 runs only where a document is left to add or a
status line to set (§2.1); `[P0]` marks it. MG, the merge of `<prev-tag>` with its
warm-up gate, runs once, after U3.3 and every review of both steps are closed and
the tag exists, before X (S2: nothing is pushed and no close-out starts before it);
where the branch was made from `<prev-tag>` itself, there is no MG.

```
step 2
core     P1 P2 ─[P0]─ U2.1 ─ U2.2 ─ U2.3 ─ U2.4 ─ U2.5 ─ U2.6 ─ U2.7 ─┐
beside      A2 ─┘          R2.1h, R2.1   R2.2   R2.3   R2.4   R2.5   R2.6   (U2.7 after all of them)
step 3                                                              │
core     ┌───────────────────────────────────────────────────────────┘
         A3 ─ U3.0 ─[verdict]─ U3.1 ─ U3.2 ─ U3.3 ─ MG(+VF) ─ X ─ V ─ K1 … K8
beside        R3.0 ─┘            R3.1, R3.1c (beside U3.2), R3.2 (U3.3 after all three)
                                                   R-MG (code resolutions, Cargo.lock, VF's and steps (6), (7)'s commits)
                                                                    K4, K5 beside K3's CI
MG once <prev-tag> exists and U3.3 and every review are closed; the lane waits there for the tag
```

Why this order (each from the specification unless named):

- **One lane for the release.** Every unit of a step needs the one before it or
  shares its files: 2.2's lists hold the reader to 2.1's preset; 2.3 to 2.5 read
  2.1's field, and 2.6 routes it; 2.7's dated lines name the units' last
  commits; step 3 builds on step 2's tree, and its units 3.1 to 3.3 need one
  another likewise (3I). Units 2.3 to 2.5 are independent of one another (2K),
  but side by side they would need two more worktrees, two whole extra builds of
  the gpui, iced and egui stacks, while the machine's two cargo slots are shared
  with the second lane (§6); this plan runs them one after the other.
- **A2 and A3 first** (a plan addition, as the steps 6–7 plan's A6 and A7): the
  specification's lines are `0fa28027`'s, and at least step 1 (v0.7.1) and, under
  D10's proposal, egui 0.37 land before this release. Every quoted text, every
  count's base and the pinned lines are produced again on the tree each step
  starts from: A2 at P2's commit, A3 at U2.7's, since step 3's
  documents unit places its texts by step 2's (3H.1 to 3H.3).
- **MG once, after the last unit** (S2 as amended): the merge of `<prev-tag>` brings
  the lane predecessor's release fixes (and the releases cut between) in without
  changing a sha, so the record's and the texts' shas stay true; X and every gate
  after it run on the merged tree, and X recounts the stated counts in the files both
  sides changed.
- **U2.1 to U2.7 in 2K's order**; U2.7 only once every review of U2.1 to U2.6 is
  closed and its fixes committed, since its dated lines name those units' last
  commits (§2.2).
- **The trial on step 2's last tree** (3I): where it fails, its revert from `HEAD`
  restores U2.7's commit, which unit 3.1 starts from; the two facts rows (3B.1) are
  unit 3.1's, since no sweep orders them before the trial any more (S6).
- **R3.0 before the verdict** (a plan addition): where the trial fails, its test
  is reverted (3I) and no later review would read it; a test that compares the
  wrong quad or skips a state would decide the release's gpui part unseen.
- **X once, after both steps' units and MG, V before the close-out** (2K's *X*,
  rule 17): X recounts the stated counts of the files either side changed and runs
  the five checkers; the provenance stamp hashes `Cargo.lock`, which V changes.
- **One close-out, only after every review of both steps is closed** and its
  fixes committed (S5).

**The build folder** (the batch's worktree rule, S2, S3). Cargo keeps one build
folder per worktree path. The KDE lane (v0.7.1, the three KDE pair releases and
the caret, S3 as amended) alternates between two worktrees: `.claude/worktrees/v071`,
v0.7.1's, and `.claude/worktrees/kde-b`. Steps 2 and 3 run in `kde-b`, since
v0.7.1's close-out holds `v071`; steps 4 and 5 run in `v071`; the caret in `kde-b`;
steps 6 and 7 in `v071`. Each release uses the worktree named here, also where its
predecessor's worktree is free when it starts. The second lane, the widget icons
alone, uses `.claude/worktrees/lane2-a` (`lane2-b` is not used). Each worktree keeps
its own build folder, which its lane's later releases reuse warm; the orchestrator
deletes a lane's extra worktrees and their build folders when the lane's last
release of the batch closes (the KDE lane's, `kde-b`, with steps 6 and 7; the
second lane's, `lane2-a`, with the icons), and `v071`'s folder stays. So this release's core worktree is `kde-b`: where it does not exist yet, P2
makes it with `git worktree add` from `<lane-pred-head>`, which builds the workspace
once from nothing (tens of GB, one long first gate); where it exists, P2 switches
it to the new branch, which is done where
`git status --porcelain --untracked-files=no` in it prints nothing and every
untracked file in it is a planning document under `docs/`. Its build folder (its
path from `cargo metadata --format-version 1 --no-deps`, `build_directory`) stays
after this release's close-out, for the caret; the close-out report and the
project status note name it (§2.6), and §7 item 6 names the departure. P2 says
whether it made or switched `kde-b`. Worktree paths are relative to the main
checkout, `/home/tibi/Rust/native-theme`; a new one is made with
`git worktree add` from the named commit (an agent's own isolation worktree
starts from `main`, the wrong base).

## 2. Tasks

A task that changes files ends in one commit on the lane's branch, unless it
says otherwise, made only when its done-criterion holds, with no attribution
line. **Who runs what** (the batch's rule of 2026-11-15, §3): an agent runs its
task's targeted commands in the foreground (the named tests,
`cargo fmt --all --check`, the checkers, a `-p` build or clippy, each expected to
finish well within 10 minutes), stages its own paths by name (`git commit -a` and
`git add -A` are not used), and reports the staged change with its commit
message; it does not run `--commit`, the full check, an asset run or the GPUI
Fast legs, and it does not commit. The orchestrator then runs the task's gate on
the staged change and, when it passes, commits exactly the gated tree with the
author's message. In the task rows, items an agent cannot run are marked "(in the
gate)". The subject follows the repository's form (`feat(core): …`,
`docs(todo): …`) and the body says what changed and why, with the outputs the
done-criterion names. Shared terms:

- "**The gate passes**": the orchestrator's long run (§3) of the task's gate, as
  §3's cadence table gives it (`scripts/check_release.sh --quick` or `--commit`,
  and for a documents-only task the five checkers with `--quick`), in the core
  worktree under the gate lock and through the wrappers (§3, *Long runs*: every gate
  run starts as `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh
  scripts/check_release.sh …`, rule 2), on the staged change, exits 0 (its `STATUS`
  file reads 0), its summary names no failure and no skipped cross-target check
  (`target not installed`, `x86_64-w64-mingw32-gcc not found`) of those it runs,
  the record names every warning the summary repeats, and the tree is formatted
  and clean; where the row says "with the GPUI Fast legs", those run beside it.
  Where the gate runs the showcase tests in CI's runner environment, a
  runner-environment exit 75 at `--prepare`, which the script reports as a soft
  warning (`scripts/check_release.sh:825-828`: the showcase tests not run in CI's
  runner environment), voids it as a skipped cross-target check does, since nearly
  every unit changes what a showcase test reads: the environment is prepared and
  the gate run again. So does an exit 75 of the GPUI Fast showcase leg. The
  orchestrator prepares the runner environment
  (`scripts/run_in_runner_env.sh --prepare`, through the wrappers) at P2; an exit
  75 there is reported to the maintainer before any agent that runs cargo is
  dispatched. A red gate goes back to the task's author (continued) with the
  log's failing part; the author fixes and stages again, and the orchestrator
  runs the gate again. A `--commit` that finds a failure a unit gated by
  `--quick` alone brought is fixed by a new commit by that unit's author, gated as
  that unit was, and the record names the commit that brought it (S1); a pushed
  commit is never rewritten. "`--commit` passes" in an orchestrator's own task
  means the run on its change.
- "**Formatted and clean**": §3's rule (`cargo fmt --all --check` by the author
  before staging; the gate on the staged change;
  `git status --porcelain --untracked-files=no` the same before and after the
  gate, and empty after the commit, but for §3's exceptions, each named in the
  record).
- "**The staged tree holds**", checked by the orchestrator before each gate and
  again before the commit: `git diff --quiet` exits 0 (nothing unstaged), or,
  for a fix committed between K2 and K7, `git diff --name-only` lists exactly
  that commit's one unstaged exception (rules 11 and 15, §6: K2's uncommitted
  files, with an unfinished asset run's partial outputs where §6 says so), each
  file named in the record;
  `git status --porcelain --untracked-files=all` shows no `??` line but the
  untracked documents P2 recorded (less those P0 commits) and, in that exception,
  the partial asset run's new files, each named one by one
  in the record (never by a pattern), so that a new file of a task is staged, not
  left untracked; and `git diff --cached --name-status` lists exactly the task's
  new files (`A`, each named in its row, "New files") and changed files (`M`, the
  files its parts own in the specification's *How to read it* table, and the
  generated files it regenerates), anything else named
  and explained before the gate runs. A row that adds files names them under "New
  files"; a row that names none adds none. No unit of either step adds a file:
  every new test goes into an existing file. The tasks that add files are P0 (the
  documents it adds), MG, whose merge commit adds every file `<prev-tag>` brings
  (listed by `git diff --cached --name-status` before that commit and by
  `git diff --name-status <merge>^1 <merge>` after it, each named in the record),
  and K7 (the asset files the script wrote).
- "**The GPUI Fast legs**": the two that `--commit` skips
  (`scripts/check_release.sh:710-716`, `:849-855`), run by the orchestrator beside
  the unit's gate, whichever it is, each a long run (a check this plan adds and
  the cadence keeps, S1 *Not changed*):
  `cargo test -p native-theme-gpui --features gpui-fast,native,watch --all-targets`
  and `scripts/run_in_runner_env.sh test -p native-theme-gpui --features gpui-fast,native,watch --example showcase-gpui`.
  They are part of that gate, so each starts under the gate's lock,
  `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh <leg>`
  (rule 2; the batch rationale's glossary); so does the trial's GPUI Fast run (U3.0),
  a long run of the orchestrator's. They run in the gates of U2.3, U2.6, U3.1, U3.2,
  of X where its commit touches a gpui connector or showcase file, of MG's warm-up
  always (rule 21 (5)), and of any fix that
  touches the gpui connector's code or showcase.
- "**The five checkers**": §3's documentation checkers. `$NT_FEATURES` is the list
  `scripts/check_release.sh:420` sets
  (`kde,portal,system-icons,material-icons,lucide-icons,watch,svg-rasterize,system-fonts`
  at `0fa28027`); the orchestrator reads that line on the branch and writes the
  list literally into every brief that uses it. "**The header**" is the
  specification from its top to the end of *Open points for the maintainer*: *How
  to read it* (with *Lines that must not move*), the decisions and the corrections
  included. Every brief also gives *Problems found* and *What could not be
  verified*.
- "**Seen to fail**": a new test, or a changed test's new assertion that tests the
  change, is written first and run before the change it tests; a test that cannot
  compile before its item exists counts, and the report quotes the failure. A
  changed test that only gains a step (a key press, a field set to `None`, a
  renamed preset) is run after the change, its `ok` quoted. A new test whose every
  assertion states what the change must keep (a ring that shows today and still
  shows: 2G.3's `the_ring_after_tab_outlasts_a_pointer_move`,
  `shift_tab_rings_a_pass_late_through_a_pointer_move`,
  `tab_then_a_pointer_move_in_one_pass_rings`), or a changed test's new assertion that
  states what the change keeps (3E.3's `panel_role_cells_write_their_rows`), is run
  before and after the change and passes both times; the report names it "kept" with
  both `ok` lines.
- "**Citations**": no unit re-points a citation of a line its edits move; the
  existing ones are left to drift (S6, spec 2B.6). A citation a unit writes names
  the document (with its section, row or a quoted phrase), never a line of a
  tracked file that changes, except where the gpui showcase's tests read a line
  (its colour claims and `HOW` texts, *Lines that must not move*); pinned sources
  (an upstream crate at a version, a URL at a commit) keep their lines. **S6's
  fourth gate:** `every_colour_claim_is_read_at_the_line_it_cites` reads the colour
  claims' citations of the gpui connector's own files (243 at `0fa28027`, the batch
  rationale's S6), so a unit that edits `connectors/native-theme-gpui/src/` or its
  examples and moves such a cited line keeps the line count above it, or re-points
  those claims in the same commit (U2.3, U2.6, U3.0, U3.1, U3.2 and any fix of
  them say so).

### The showcase tests the agent runs

The gate's log prints a test run's summary and no `ok` lines when it passes
(`scripts/check_release.sh:225-260`, its filter `TEST_OK_FILTER`, `:181`), so a
task that must show a showcase test `ok` runs it itself, by name, in the
foreground, in the runner environment the gate uses
(`scripts/check_release.sh:836-848`):

```bash
cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh scripts/run_in_runner_env.sh test -p native-theme-gpui --features native,watch --example showcase-gpui -- <names>
cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh scripts/run_in_runner_env.sh test -p native-theme-egui --all-features --example showcase-egui -- <names>
cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh scripts/run_in_runner_env.sh test -p native-theme-iced --features iced_aw,watch,native --example showcase-iced -- <names>
```

An exit 75 there (the runner's packages neither fetched nor found) is reported,
and the run is made again once the orchestrator has prepared the environment;
the tests are not run outside it instead. A run the Bash tool kills at 10 minutes
is §3's case.

### 2.1 Before step 2

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| P1 | **The tools**, once. `mkdir -p <locks>` and every subfolder of `<record>` §5 names; copy `slot.sh` and `lowprio.sh` from `<scratch-keep>` into `<tools>` and read both. In the copies: `slot.sh`'s lock folder (`D=`, an old session's scratch folder) becomes `<locks>`, the one lock folder every plan of the batch uses, so the two slots count every cargo process of every plan and session; `lowprio.sh` keeps its absolute `renice -n 19 -p $$` (a relative `nice -n 19` gives 15 under Konsole's −4) and its `ionice -c 3`, and gains `export CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16`, lifted only on the maintainer's word that they are not using the machine. The copies of the approved documents in `<record>/docs/` were made, and compared with their originals by `cmp`, at the approval (§7 item 5), before P1; P1 copies the inputs of §5 into `<record>/inputs/` and creates `<record>/record.md`. P1 runs before the branch exists, so it reads the commit the branch is made from, not a checkout (rule 26): for each of this work's three documents, `git -C /home/tibi/Rust/native-theme cat-file -e <lane-pred-head>:docs/<name>` (`<prev-tag>` in its place where the branch is made from the tag) failing means untracked; for each one an earlier commit added (normally v0.7.1's commit (0)), P1 records that commit (`git -C /home/tibi/Rust/native-theme log --diff-filter=A --format=%H <lane-pred-head> -- docs/<name>`, `<prev-tag>` likewise) and copies nothing; only a document still untracked there is compared with its approval copy, at P0. One input names `.claude/worktrees/v071` as its root, `cite2.py` (`ROOTS`' `native-theme-gpui` entry): P2, once it has chosen the core worktree, makes a copy of it beside the original copy with that path set to the core worktree, as `rehearsal.sh`'s `W=` is set (K5); the other inputs take their root as an argument. No repository change. | orchestrator | the maintainer's resource rules | `<tools>/slot.sh <tools>/lowprio.sh true` exits 0; `<tools>/slot.sh <tools>/lowprio.sh sh -c 'echo "$CARGO_BUILD_JOBS $RUST_TEST_THREADS"; ps -o ni= -p $$'` prints `16 16` and `19`; after the trial `slot1.lock` is in `<locks>` (`ls`; `slot.sh`'s header names `slot1.lock` and `slot2.lock`, and the second appears only when two commands overlap); `cmp` of each copy in `<record>/inputs/` against its original prints nothing (the documents' copies were compared at the approval) |
| P2 | **The branch and the worktree** (§1, *The build folder*; S2). The base is `<lane-pred-head>`, the lane predecessor's (v0.7.1's) gated head once its gate has passed and its reviews are closed (rule 20: its documents unit, so the later release reads those texts; its record says so: v0.7.1's line "the lane's gated head for the next release: <sha> (MK4; …)" in `<scratch-keep>/todo_v0.7.1_gpui-kit-0.9.1-plan/record.md`, the GPUI Kit 0.9.1 plan's, which the step 1 plan's own record repeats (D10 (a), 2026-11-15; rule 18); the orchestrator writes the sha and the record's lines into this record), not its tag; where `<prev-tag>` exists already, the tag itself is the base and MG falls away, and before the branch is made, before `git worktree add` or `git switch`, so that a stop leaves no worktree or branch made from a moved tag, the tag check of MG's step (1) runs (rule 21 (1): fetched only where `git rev-parse -q --verify <prev-tag>` fails; the first field of `git ls-remote origin 'refs/tags/<prev-tag>'` equal to `git rev-parse <prev-tag>`; no line printed, or a different sha, a stop). A step outcome or status this plan reads from the predecessor is read from its commit messages or record, not from a status line its close-out writes. The branch from it, in `kde-b` (§1): where `kde-b` does not exist yet, `git -C /home/tibi/Rust/native-theme/.claude/worktrees/v071 worktree add -b v0.7.N-rc1 /home/tibi/Rust/native-theme/.claude/worktrees/kde-b <lane-pred-head>` (rule 24: worktree commands with absolute paths); where it exists, and only once the release that last used it has its tag (rule 18: checked after `git fetch origin tag <that tag>`; otherwise this release waits for it), `git switch -c v0.7.N-rc1 <lane-pred-head>` in it; on the tag path both commands name `<prev-tag>` in place of `<lane-pred-head>`. `git switch` refuses where an untracked planning document is a file the base's tree holds: the two are compared with `cmp`, the untracked copy removed where equal, a stop otherwise (§6). Also read: the predecessor's record shows `<lane-pred-head>` as rule 20's recorded line ("the lane's gated head for the next release: …"), and HEAD is that commit; v0.7.1's S6 unit is on the tree (the three gates that read `docs/platform-facts.md` by line stop doing so, batch-speed rationale S6; spec 2B.6), read from the tree, not from a record: `cd <core worktree> && git grep -n -e 'lines()\.nth' -e 'facts_line(' -- native-theme/tests/platform_facts_xref.rs native-theme/src/presets/documented_sizes.rs` prints nothing (the two patterns as `-e`, no alternation), since this release's facts rows move lines those gates would otherwise read, a stop otherwise; where the branch was made from `<prev-tag>`, `git merge-base --is-ancestor <lane-pred-head> HEAD` exits 0 (rule 18; otherwise MG checks the tag's ancestry before the merge); on both paths (the branch made from `<lane-pred-head>` or from `<prev-tag>`, rule 19), the tree's `Cargo.lock` versions of the crates below are compared with `git show 0fa28027:Cargo.lock`'s, a changed version's readings made again by A2 before the first unit, or P2's stop where it lies outside the accepted set (rule 19); where there is neither MG nor P0, P2 also runs the path check (P0's row, rule 22 (b)); the versions in `Cargo.lock` against the specification's, exactly (P2's accepted set, which MG's comparison reads too, rule 19): egui and egui-winit 0.36.2 or 0.37.x (the header: the `EG` citations are re-read in the tree's version), iced_widget 0.14.2, iced_core, iced_runtime, iced_winit, iced_test and iced_futures 0.14.0, gpui-component and gpui-base 0.9.1, gpui-pre and gpui-pre-linux 0.3.12 (all as at `0fa28027`); a patch version other than these passes only with A2 reading again, in the version on the tree, every line of that crate the specification cites (A2 (b)), and any other difference is a stop; `rustup target list --installed` lists `x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu` and `x86_64-apple-darwin`, and `x86_64-w64-mingw32-gcc` is on the path; `rustup toolchain list` lists `1.88.0` and `1.95.0` (the MSRV checks the gate runs, §3; O3's keyed option being approved, 2026-11-15); `scripts/run_in_runner_env.sh --prepare` through the wrappers exits 0. Before any code agent is dispatched, the project's hooks are seen to fire in the core worktree (they are configured with absolute paths in the main checkout's `.claude/settings.json`): a Write of a new, untracked file `<core worktree>/native-theme/src/hook_trial.rs` containing `fn f() { x.unwrap() }` is refused by `no-runtime-panics.sh`, and nothing is written; if the Write applies, the file is removed with `rm` and the plan stops, no code agent dispatched. A document of this work that an earlier commit added (P1's `cat-file -e` succeeded) is not compared with its approval copy (rule 26: later close-outs rewrite its path mentions and set its status line); P1 recorded the commit that added it. P2 then runs §3's warm-up gate (`--commit` on the clean tree, `<record>/gates/warmup-<n>/`), before any agent that runs cargo is dispatched, timed into the record as the first `--commit` run, with whether it ran cold or warm. P2 also makes P1's adapted copy in `<record>/inputs/`: `cite2-core.py`, with the core worktree's absolute path in place of `.claude/worktrees/v071`'s (P1). Where a gpui-component patch version other than 0.9.1 is on the tree, `cite2-core.py`'s registry path (`ROOTS`' `gpui-component` entry) names that version, and so does a copy `prose-core.py` of `prose.py` (its `G`), used in its place; MG makes them again where the merge changes a version. No commit. | orchestrator | §1 | `git rev-parse --abbrev-ref HEAD` prints `v0.7.N-rc1` and `git rev-parse HEAD` equals `<lane-pred-head>` (or `git rev-parse <prev-tag>^{commit}` where the tag is the base); `git status --porcelain --untracked-files=no` prints nothing, every untracked file a planning document under `docs/`, the untracked files listed one by one in the record (`git status --porcelain --untracked-files=all`, the list the staged-tree check of §2 reads); on the tag path, the ancestry check exits 0; the S6 grep prints nothing; on the tag path, the tag check found `<prev-tag>`'s sha on `origin` equal to the local one; the versions, the toolchains, the build folder (`cargo metadata --format-version 1 --no-deps`, `build_directory`, through the wrappers, as `<cargo-build>/<hash>`) and the targets are in the record; the hook's refusal is quoted in the record, and `<core worktree>/native-theme/src/hook_trial.rs` does not exist; the warm-up gate passes and its time is in the record; `diff` of each adapted copy against its original shows only the root's line and a registry path's version where one changed. The predecessor's gated head not gated (rule 20) or a review of it open, step 1 not below the branch, a toolkit version that differs from the specification's by more than a patch version, a target, a toolchain or the MinGW linker missing, on the tag path `<prev-tag>` missing on `origin` or with another sha there, an exit 75 at `--prepare`, or the trial Write applied: stop (§6) |
| P0 | **Commit (0): the documents** (2K, *Commit (0)*), only where a document is left to add or a status line to set. v0.7.1's commit (0) adds every document of the cycle the maintainer has approved by then, the `docs/todo_v0.7.x_*` plans written by then among them, and sets their status lines (`docs/todo_v0.7.1_gpui-kit-0.9.1-plan.md`, its P0 and §7 item 1), so on the likely path this work's three documents are on the branch already, and there is no P0 (2K: "Where nothing is left to add, there is no commit (0)"; the caret plan's P0). Otherwise P0 adds those of this work no earlier commit added (each untracked in the commit the branch was made from, as P1's `cat-file -e` found, rule 26), from `.claude/worktrees/v071/docs/` where they lie untracked today, compared first with `<record>/docs/` by `cmp` (rule 26: equal, the copy is added; v071's gone, the record's copy; different, a stop naming what changed, §6); a document an earlier commit added is not compared (rule 26, P1), and sets their status lines to approved with the date; where an earlier commit added all three, P0 only sets the status lines it did not set; where that too is done, there is no P0. Only where P2 made the branch from `<prev-tag>` (no MG), the path check of MG's step (6) runs here instead, on that tree, its re-points P0's own documents-only commit; where there is neither MG nor P0, P2 runs it (rule 22 (b)), its re-points a documents-only commit of their own. New files: the documents it adds, each named in the record. | orchestrator | §7 | `git show --stat HEAD` lists the documents and nothing else; its gate, a documents-only one (§3: the five checkers and `--quick`), passes on the staged change, and where it is the first `--quick` run, its time is in the record. Commit: `docs: issue #4's open KDE items, steps 2 and 3: the documents, approved <date>` (each document named in the body), or, where P0 only sets status lines, `docs: issue #4's open KDE items, steps 2 and 3: the status lines, approved <date>` |
| A2 | **The anchors of step 2**, beside P2's warm-up gate or P0. At P2's commit, with `git show <sha>:<path>` and the specification's own search methods, the scripts kept in `<record>/anchors-2/`: (a) every present text Step 2's parts quote to replace or to place an edit by, and every test they change by name, found in the file named as often as the edit names it, with its line, or "not found"; (b) the earlier releases' parts step 2 builds on, found: step 1's form of `populate_colors`' doc (2C.1's paragraph, "as step 1 leaves it"), step 1's entries of `derived_leaves` and `resolved_leaves` with the doc's count, the reply draft of the first rationale's §8 as step 1 (its G.3) left it (the bullets **The focus ring.** and **Hover borders on buttons.**, the paragraphs that begin "Fixed earlier, in 0.7.0:" and "With 0.7.1 (which is built on GPUI Kit 0.9.1)", the "Still yours to keep:" sentence, the note's sentences that 2I.3 item 5 names), and step 1's backlog texts 2I.5 places by (the dated note under *KDE values that wait for a connector design*, the entry *KDE: a menu's frame is Qt Quick's, its fill Qt Widgets'* and the entry after it, *The selection of a view without the focus*); each egui and egui-winit item the step's texts cite, read again in the version `Cargo.lock` holds (`~/.cargo/registry/src/`) for its content only, whether it still does what the text says (the header: `EG` lines are 0.36.2's; a citation pinned to its version keeps it, S6); where P2 found an iced, gpui-component, gpui-base or gpui-pre patch version other than the specification's, every item of that crate the specification cites, read again in the tree's version for its content only (but for a line a gate reads by number, a `HOW` text's or colour claim's line in a crate of `CITED_CRATES` (`CG/showcase.rs:2049`: gpui-component, gpui-base, gpui-pre, gpui-pre-wgpu, egui) or the fallback-stack test's gpui-pre lines, whose line there is listed too, rule 25); (c) the counts' bases: egui's `mapping.md` totals (Foundation leaves and DERIVED, TOTAL and DERIVED), the leaf count and its message in `mapping_tests.rs`, `verdict_totals_match_the_specification`'s `foundation` tuple, the `derived_leaves` count and its doc's number, `arb_theme_defaults`' tuple and its comment, `ROADMAP.md`'s stated count of open `[v0.7.x]` items (`:264` at `0fa28027`, 29 there) and `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md` on the tree (41 at `0fa28027`), the two recorded apart; (d) the line range of `docs/todo.md`'s dated block, a pinned document X leaves out (from the sentence whose `platform-facts.md` lines are "at caeddd6b", `:2048-2049` at `0fa28027`, found by its text across its line break, to its section's end, the `---` rule before the heading `## Toolkit Connectors`, both found by their text: `:2097` at `0fa28027`, with a blank `:2098` and the heading at `:2099`); (e) every struct literal of `ThemeDefaults` and `ResolvedDefaults` by 2A.4's search, each read to its closing brace: the full ones (three at `0fa28027`) and those that end in `..` (27); (f) the pinned lines of *Lines that must not move*, listed again from every `native-theme-gpui/<file>.rs:N` in `examples/showcase-gpui/info/*.rs`, the last pinned line of each file 2E edits (`colors.rs`, `widgets/mod.rs`, `widgets/checkbox.rs`, `widgets/switch.rs`, `widgets/slider.rs`); (g) kde-breeze's four blocks of 2B.5 and every other preset line 2B.5 inserts after, found by their text. No commit, no cargo. | `worker` | the header; Step 2's parts 2A to 2K | the seven outputs are in `<record>/anchors-2/`, each table complete; nothing is "not found", or each text not found is settled by §6's rule for a quoted text; an egui item whose content changed in the tree's egui version is listed with the change; the orchestrator copies the summary into the record |

### 2.2 Step 2

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| U2.1 | **Unit 2.1, the field and its values**: 2A entire; 2B.1, 2B.2, 2B.3, 2B.5; 2G.2. Not 2B.4 (U2.6). Its changed files include `native-theme/src/resolve/tests.rs` (2A.4's new test). No citation of a line it moves is re-pointed (2B.6). | `implement` | the header; 2A; 2B but 2B.4; 2G.2; 2K's row of unit 2.1; A2 (a), (c), (e), (g) | (1) seen to fail: 2A.1's `merge_keeps_and_overrides_focus_ring_keyboard_only` and `toml_round_trip_focus_ring_keyboard_only`, 2A.4's new test in `resolve/tests.rs` (its name the implementer's, in the report), `default_has_all_none_options`' new assertion and `theme_defaults_field_info_matches_baseline`'s new entry; (2) they pass, each named `ok`: `cargo test -p native-theme --lib -- merge_keeps_and_overrides_focus_ring_keyboard_only toml_round_trip_focus_ring_keyboard_only default_has_all_none_options theme_defaults_field_info_matches_baseline property_registry_keys_match_derive_inventory no_inheritance_fields_are_not_inherited per_platform_rules_match_safety_nets every_bundled_preset_source_lints_clean documented_sizes <the resolve test>`, `cargo test -p native-theme --test proptest_roundtrip`, `cargo test -p native-theme --test platform_facts_xref`, `cargo test -p native-theme-egui --all-features --lib -- every_leaf_has_exactly_one_row_and_every_row_is_well_formed verdict_totals_match_the_specification t2_every_row_s_declared_sinks_are_exactly_what_its_leaf_moves mapping_md_is_current`, `cargo test -p native-theme-iced --all-features --lib -- every_colour_leaf_reaches_a_row_or_is_named`, `cargo test -p native-theme-gpui --features native,watch --lib -- every_colour_leaf_reaches_a_sink_or_is_named`; (3) the generated files by their commands only: `mapping.md` (`cargo test -p native-theme-egui --lib -- --ignored regenerate_mapping_md`), and at the unit's end `cargo test -p native-theme-iced --all-features --lib -- --ignored regenerate_contract_leaves`, whose diff is expected empty (2K) and, where it is not, staged with each difference explained in the commit body; (4) each count equals A2 (c)'s base plus 2G.2's delta (the leaf count and its message +1, Foundation and TOTAL leaves +1, their DERIVED +1, the `foundation` tuple's third number +1), both in the report, and `arb_theme_defaults`' comment states the number of scalar `Option` fields it then lists, counted in the report (2A.4); (5) `git diff HEAD --numstat -- docs/platform-facts.md` prints `1	0`, the row added directly after `focus_ring_offset`'s row (A2 (a)), and the row equals 2B.1's text (extracted from the specification and compared by the implementer's script in `<record>/u2.1/`); the four kde-breeze blocks equal 2B.5's texts (nothing reads either file by line number, spec 2B.6, so neither file's line count is checked); (6) the full literals of A2 (e) all name the field, and `git grep` finds no other literal without `..`; (7) the five checkers pass; `python3 scripts/check_showcase_parity.py --check-list` passes; `cargo fmt --all --check` passes and the change is staged; (8) the gate passes, `--quick` (the first code unit since P2's warm-up gate, §3; not structural, rule 24; no gpui unit by rule 11, since `ResolvedDefaults` is `#[non_exhaustive]` and no gpui code reads the field before U2.3) (in the gate). Commit: `feat(core): defaults.focus_ring_keyboard_only, with its facts row, rules, presets and egui's mapping row` |
| U2.2 | **Unit 2.2, the reader**: 2C entire (the reader, the portal path, their tests, `kde/mod.rs`' two lists and the accent test's list). | `implement` | the header; 2C; 2K's row of unit 2.2; A2 (a) to (c) | (1) seen to fail: `the_focus_ring_is_the_selection_at_blend_value`, `the_kde_portal_accent_rings_at_blend_value`, `reader_kde.rs`'s two changed expectations, and the lists of `derived_leaves`, `resolved_leaves` and the accent test, changed first against U2.1's tree; (2) they pass: `cargo test -p native-theme --features "$NT_FEATURES" --lib -- kde:: gnome:: --nocapture`, its count of tests run quoted, the two new tests, `the_derived_colours_are_the_presets_under_breeze`, `an_accent_moves_the_on_track_the_pressed_button_the_selected_segment_and_the_list_hover` and `the_view_decorations_move_the_frames_the_tab_hover_and_the_handle` named `ok`; `cargo test -p native-theme --features "$NT_FEATURES" --test reader_kde` (warm: the gate's leg `scripts/check_release.sh:683-684` builds that set); (3) CI's Linux legs of the edited modules (`ci.yml:211-216`) pass: `cargo test -p native-theme --no-default-features` by the agent (the gate's own leg builds it, `scripts/check_release.sh:665-677`, so it is warm), and `cargo test -p native-theme` with `--features kde`, `--features portal` and `--features linux` by the orchestrator, each a long run beside the gate under the gate lock (§3, *Long runs*: no gate leg builds those feature sets, so each is a cold build), each leg in a folder of its own, `<record>/gates/U2.2/legs-<feature>-<n>/` (`<feature>` `kde`, `portal` or `linux`); (4) the derived-leaves count and the doc's number equal A2 (c)'s base plus one; (5) `cargo fmt --all --check` passes and the change is staged; the gate passes, `--quick` (§3), with the three legs (in the gate). Commit: `feat(kde): the KDE reader and the KDE portal's accent state Breeze's focus band` |
| U2.3 | **Unit 2.3, gpui**: 2E entire (2E.3's clause in `scripts/connector_parity_exceptions.toml` among it), and 2H.3 but the new leaf's `HOW` entry (U2.6): the edits of the `focus_ring_color` and `focus_ring_width_px` entries, the Theme Map's Ring row, the four branched claims, `info/inputs.rs:169`'s note and `theme_focus_ring`'s clause. Where 2E.2 cannot be built, 2J's rule (*The fallback*, below). S6's fourth gate (§2, *Citations*): a moved line a colour claim cites keeps the line count above it, or the claims are re-pointed in this commit. | `implement` | the header; 2E; 2H.3; 2J; 2K's row of unit 2.3; A2 (a), (f) | (1) seen to fail: `focus_ring_is_the_band_rule` (the renamed test, its new assertions), `ring_is_opaque_where_the_band_is_off`, `a_control_focused_without_a_key_shows_no_ring_where_the_theme_says_keyboard_only`, `the_ring_outlasts_a_mouse_move_and_ends_with_the_focus`; `every_focused_control_draws_the_ring`'s key presses and `a_switch_told_not_to_draws_no_focus_ring`'s doc run after; (2) they pass, each named `ok`: `cargo test -p native-theme-gpui --features native,watch --lib -- focus_ring_is_the_band_rule ring_is_opaque_where_the_band_is_off every_focused_control_draws_the_ring a_control_focused_without_a_key_shows_no_ring_where_the_theme_says_keyboard_only the_ring_outlasts_a_mouse_move_and_ends_with_the_focus a_switch_told_not_to_draws_no_focus_ring the_focus_ring_is_the_themes every_mapped_field_equals_its_native_source every_colour_leaf_reaches_a_sink_or_is_named every_colour_claim_is_read_at_the_line_it_cites every_edge_colour_is_claimed_through_edge_claim every_prose_citation_still_exists`; the showcase tests, by the agent's own runs (*The showcase tests the agent runs*): `a_theme_drawn_controls_info_prints_its_look` (it reads the focus ring's claims through `ring_findings`), `every_listed_leaf_has_a_route`, and every other test of `examples/showcase-gpui/tests.rs` that names a function or info entry the unit edits (found by `grep`, named in the report), each `ok` quoted; (3) no pinned line moved: every hunk of `git diff HEAD -U0 -- <file>` for `colors.rs`, `widgets/mod.rs`, `widgets/checkbox.rs`, `widgets/switch.rs` and `widgets/slider.rs` either replaces one line by one line (`colors.rs`' import and `:223`'s `ring`, 2E.1) or starts after A2 (f)'s last pinned line of that file, and `every_colour_claim_is_read_at_the_line_it_cites` passes; (4) `python3 scripts/check_connector_parity.py` and `python3 scripts/check_showcase_parity.py --check-list` pass; (5) `cargo fmt --all --check` passes and the change is staged; (6) the gate passes, `--commit` (the third code unit since P2's warm-up gate, §3, where no other `--commit` fell between; a `--commit`-gated fix between restarts the count, and the table's marks shift, the record naming the new ones), with the GPUI Fast legs (in the gate). Commit: `feat(gpui): gpui-component's band off where the theme says keyboard only, and the connector's own controls ring a focus that followed a key` |
| U2.4 | **Unit 2.4, iced**: 2F entire, 2F.5 (the showcase's Tab handler and its test), and 2H.2's `the_focused_field_draws_the_themes_ring_round_it`. Where 2F.1 cannot be built, 2J's rule. | `implement` | the header; 2F; 2H.2's test; 2J; 2K's row of unit 2.4; A2 (a) | (1) seen to fail: `where_the_theme_says_keyboard_only_a_click_draws_no_ring`, `a_field_focused_before_any_input_shows_no_ring_where_keyboard_only`, `a_modifier_key_alone_is_no_key`, `tab_moves_the_focus_with_iceds_operations`, and the new split (b) of `the_focused_field_draws_the_themes_ring_round_it`; the four changed tests of 2F.2 run after; (2) they pass: `cargo test -p native-theme-iced --all-features --lib -- focus_ring:: every_colour_leaf_reaches_a_row_or_is_named --nocapture`, its count quoted, each test of 2F.2 named `ok`; the showcase tests by the agent's own runs: `the_focused_field_draws_the_themes_ring_round_it`, `tab_moves_the_focus_with_iceds_operations`, `the_system_menu_bar_takes_the_open_menu_flag_and_the_keys` and `styles_cover_every_widget_shown`, each `ok` quoted, the new ones also before the change; (3) `python3 scripts/check_connector_parity.py` passes; (4) `cargo fmt --all --check` passes and the change is staged; (5) the gate passes, `--quick` (the first code unit since U2.3's `--commit`, §3) (in the gate). Commit: `feat(iced): the focus ring wrapper rings a focus that followed a key where the theme says keyboard only, and the showcase moves the focus by Tab` |
| U2.5 | **Unit 2.5, egui**: 2G.1, 2G.3, 2G.4 (2G.5 changes nothing). Where 2G.1 cannot be built, 2J's rule; then this commit also turns 2G.2's mapping row into 2J's `unmappable` row and regenerates `mapping.md`, with the group totals and sub-tag counts taken from the regenerated file. | `implement` | the header; 2G; 2J; 2K's row of unit 2.5; A2 (a), (b) | (1) seen to fail: the five `FocusOrigin` tests and `focus_origin_a_modifier_alone_is_no_key`, `the_focus_origin_is_kept_per_viewport`, and of 2G.3's eight install tests the five that expect no ring; the three that expect a ring are "kept" (§2); 2G.4's test run after its edit; (2) they pass: `cargo test -p native-theme-egui --all-features --lib -- install_tests:: plugin:: --nocapture`, its count quoted, each new test and `the_ring_marks_keyboard_focus_only_while_the_window_has_it`, `the_ring_takes_the_innermost_scope_s_radius` and `a_clicked_borderless_text_field_shows_the_focus_ring` named `ok`; `cargo test -p native-theme-egui-widgets --all-features -- a_widget_focused_in_the_body_is_ringed_at_the_callers_radius the_switch_rings_its_track`; the egui showcase's `the_theme_map_names_the_tests_that_check_a_per_call_route` by the agent's own run (the new tests' docs begin "T14 (b)", 2G.3), `ok` quoted; (3) in the fallback only: `mapping.md` by its command and the mapping tests of U2.1 (2) passing with the totals read from it; (4) `cargo fmt --all --check` passes and the change is staged; (5) the gate passes, `--quick` (§3) (in the gate). Commit: `feat(egui): the install plugin rings a focus that followed a key where the theme says keyboard only` |
| U2.6 | **Unit 2.6, the showcase leaf**: 2B.4; iced's route (2H.2's four arms and `the_focus_ring_routes_name_the_border_that_draws_it`); gpui's `HOW` entry for the new leaf (2H.3); egui's route, which unit 2.1's row gives (2H.4). Each in 2J's form where its connector fell back (the record). S6's fourth gate (§2, *Citations*): a moved line a colour claim cites keeps the line count above it, or the claims are re-pointed in this commit. | `implement` | the header; 2B.4; 2H.2, 2H.3, 2H.4; 2J; 2K's row of unit 2.6; the record's fallbacks | (1) seen to fail: the element list's new leaf, added first, fails `the_element_list_parses_and_every_leaf_has_a_route` (iced) and `every_listed_leaf_has_a_route` (gpui), each run before the routes; `the_focus_ring_routes_name_the_border_that_draws_it`'s new leaf fails before the arms; (2) they pass, by the agent's own runs: those three, and the egui showcase's tests that read `docs/showcase-elements.toml` (found by `grep` for its name in the egui showcase's tests, named in the report), each `ok` quoted; `cargo test -p native-theme-gpui --features native,watch --lib -- every_how_entry_is_shown every_prose_citation_still_exists` named `ok`; (3) `python3 scripts/check_showcase_parity.py --check-list` passes; (4) `cargo fmt --all --check` passes and the change is staged; (5) the gate passes, `--commit` (the third code unit since U2.3's, §3, where no other `--commit` fell between; a `--commit`-gated fix between restarts the count, the record naming the new marks), with the GPUI Fast legs (in the gate). Commit: `feat(showcases): the focus ring's keyboard rule in the element list and its routes` |
| U2.7 | **Unit 2.7, the documents**, once every review of U2.1 to U2.6 is closed and its fixes committed: 2I entire but 2I.6 (K7's, one sentence with 3H.5, S5): 2I.1, the release's one CHANGELOG section with step 2's entries, under the heading `## [0.7.N] - <date>` in 2I.1's form (its proposed number from the start, never `## [Unreleased]`; its `<date>` left for V to fill), above every other `## [` heading (U3.3 adds step 3's entries and the one *Breaking Changes* entry, 3H.1); 2I.2, the release's one ROADMAP section with its heading and step 2's paragraph (U3.3 adds step 3's, 3H.2), with `ROADMAP.md:264`'s recount; 2I.2a; 2I.3, where the reply is unposted as the orchestrator reads it before dispatching U2.7 (`cd <core worktree> && gh issue view 4 --comments`: posted is a comment by the repository owner carrying the draft's opening sentence, the first rationale's §8; a failed read is reported and U2.7 is not dispatched until a read succeeds, so that the draft U3.3's 3H.3 anchors read is the one step 2 left; where posted, the draft is not edited; under S5 the reply waits for the release of steps 4 and 5, so unposted is expected); 2I.4, 2I.5; each in 2J's form for every connector the record names as fallen back. The release placeholders ("<the release that brings step 2>", in the reply too, 2I.3) are written as `0.7.N`, the version the maintainer gave (§7 item 1), and the `<date>` of 2I.5's dated notes as the commit's day. The dated lines name the last commit, its review fixes included, of the unit that made what they state: item (1)'s note and item (9)'s note the latest of U2.1's to U2.5's; the `[1.0]` entry's closing line U2.3's (it cites `info/data.rs` by the phrase it quotes, no line, 2I.5). | `worker` | the header; 2I; 2J; A2 (a) to (c); the record's shas, fallbacks and the maintainer's answers | each text equals the specification's block for the outcome recorded (extracted and compared by the worker's script in `<record>/u2.7/`), placed by its anchor (A2 (a), (b)); each changelog entry one physical line; the closed entry in `docs/archive/todo_closed.md` verbatim but for the ticked box and the closing line; `ROADMAP.md`'s count equals `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md` at the commit, in the body; the five checkers pass; `python3 scripts/check_readme_versions.py` passes; the change is staged; the gate passes, documents-only (§3: the five checkers and `--quick`; the READMEs are crate docs, `#[doc = include_str!("../README.md")]`, `native-theme/src/lib.rs:179`, which `--quick`'s doctests of the changed crates read) (in the gate). Commit: `docs: the focus ring's keyboard rule in the changelog, the roadmap, the READMEs, the first rationale, the upstream request and the backlog` |

**The fallback (2J).** A mechanism of 2E.2, 2F.1 or 2G.1 falls back only on
evidence: the implementer reports what does not compile against the toolkit, or
which of its tests cannot be made to pass without a change the specification does
not describe, with the compiler's or the test's output, and stops that part. The
orchestrator reads the toolkit source the report cites; where it shows the toolkit
cannot give what the part needs, the fallback is taken; otherwise the part is sent
once to `implement-hard` (§6), and taken only if that fails too. The fallback is the
rationale's decision (D10's last bullet), so no answer is waited for: the
orchestrator writes into the record which mechanism fell back and why, tells the
maintainer, and the unit goes on in 2J's form (its code ignores the field, its tests
of the rule are not added, the `None` edits of the tests it would have broken are
not made, its documentation says why). U2.6 and U2.7 then write 2J's texts for it,
and a backlog entry records what was tried and why it failed (2J), in U2.7.

A **late fallback**, one found after the unit is committed (by a review's must-fix,
by MG's warm-up gate, where the merged toolkit version no longer gives what the
mechanism needs, by CI, or at K6), is §6's fix after the units: a commit by the unit's author
that turns the unit into 2J's form (for egui with 2G.2's row, as U2.5 says), a
`worker`'s commit that rewrites U2.6's and U2.7's texts to match, each reviewed as
in §4 and gated; where the close-out has started, the one close-out fix rule
holds (§6).

### 2.3 Before step 3

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| A3 | **The anchors of step 3**, at U2.7's commit (§1), as A2: (a) every present text Step 3's parts quote, and every test they change by name; (b) step 2's parts step 3 builds on, found: `selection_bg` in `populate_colors`, `focus_ring_of`, `apply_kde_accent`, `window_with_resolved`, `focus_band_on` and `ring_colour`, step 2's entry in `[no_inheritance]`, the accent test's list ending in `"defaults.focus_ring_color"`, step 2's entries of `derived_leaves` and `resolved_leaves`, the reply draft as step 2 left it (3H.3's anchors: the bullets **Hover borders on buttons.** and **The focus ring**, the paragraph "> With " whose text contains "you can drop `focus_ring`", the sentence that begins "Toolbar buttons:", the note), and the egui items step 3's texts cite (3E.1's `Widgets::style`), read again in the egui version on the tree for their content only; where P2 found an iced, gpui-component, gpui-base or gpui-pre patch version other than the specification's, every item of that crate Step 3's parts cite (3F's `GC` lines among them), read again in the tree's version for its content only (but for a line a gate reads by number, a `HOW` text's or colour claim's line in a crate of `CITED_CRATES` (`CG/showcase.rs:2049`: gpui-component, gpui-base, gpui-pre, gpui-pre-wgpu, egui) or the fallback-stack test's gpui-pre lines, whose line there is listed too, rule 25); (c) the counts' bases: egui's `mapping.md` totals (Buttons leaves and DIRECT, TOTAL and DIRECT), the leaf count and its message, the `buttons` tuple, the `derived_leaves` count, `arb_button_theme`'s vector length and comment, `ROADMAP.md:264`'s stated count and the `grep -c` of open `[v0.7.x]` entries, recorded apart, and the 34 in its five places in two READMEs (`README.md` twice, `native-theme/README.md` three times, 3H.2a), each found by its text, with the count of `Option<Rgba>` soft options by `<record>/inputs/soft.py` (3H.2a's read); (d) the dated block of `docs/todo.md`, found as A2 (d) finds it; (e) every struct literal of `ButtonTheme` and `ResolvedButtonTheme` by 3A.2's search (two full, five with `..` at `0fa28027`); (f) the pinned lines as A2 (f), with the last pinned line of `variants.rs` and of `colors.rs`; (g) the facts file's `hover_background` row in §2.3, found by its text; and the key `"hover@basic.buttons.default.border"` in `docs/showcase-exceptions.toml` (3H.2b). No commit, no cargo. | `worker` | the header; Step 3's parts 3A to 3I | as A2; the orchestrator copies the summary into the record |

### 2.4 Step 3

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| U3.0 | **Unit 3.0, the trial** (3F.1 to 3F.3), on U2.7's commit (step 2's last commit, its fixes included), uncommitted: 3A.1 (the two fields), 3A.2's two full literals, 3B.5's kde-breeze lines, 3F.1's `bordered_button` with `hover_border` after it, and 3F.2's test in `CG/widgets/tests.rs`, which prints, for each of the six combinations and each state of 3F.2's table, the compared items of both buttons at 8 bits (fill, border colours, widths, radii), so that the record holds them, and asserts the text colour by value. The decisive test runs on both backends (a plan addition, §7 item 4: the gate runs every widget test on both, and `bordered_button` ships for both), as two commands. The agent's, in the foreground: `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo test -p native-theme-gpui --features native,watch --lib widgets::tests::bordered_button_matches_the_default_button_but_its_hover_border -- --nocapture`. The orchestrator's, a long run in a new `<record>/trial/fast-<n>/` made first, under the gate lock (rule 2): `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh cargo test -p native-theme-gpui --features gpui-fast,native,watch --lib widgets::tests::bordered_button_matches_the_default_button_but_its_hover_border -- --nocapture > <record>/trial/fast-<n>/LOG 2>&1; echo $? > <record>/trial/fast-<n>/STATUS`. The agent reports its result line and printout, and, where it fails, the combination, the state and the item that differed; the tree stays uncommitted. The agent changes no file after its decisive run and before it reports. Each pair of runs (`<n>`; the pair after a fix takes the next) has its patch, `cd <core worktree> && git diff HEAD > <record>/trial/trial-<n>.patch`, saved by the orchestrator when the agent reports and before the GPUI Fast run starts, so that both runs and R3.0 read the same tree. A fix of the trial's code stays within 3F.1's form (one `bordered_button`, built from `hover_border(n)` with 3F.1's four colours and `.shadow(false)`, and the caller's refinements per state); a fix that needs another form needs more than the parts name (§6). S6's fourth gate (§2, *Citations*): a moved line a colour claim cites keeps the line count above it (the trial's lines in `variants.rs` lie after the claimed ones, *Lines that must not move*). | `implement` | the header; 3A.1, 3A.2, 3B.5; 3F.1 to 3F.3; 2E.4's `window_with_resolved` (A3 (b)) | `git diff HEAD --stat` lists exactly `native-theme/src/model/widgets/mod.rs`, `native-theme/tests/proptest_roundtrip.rs`, `native-theme/src/presets/kde-breeze.toml`, `connectors/native-theme-gpui/src/variants.rs` and `connectors/native-theme-gpui/src/widgets/tests.rs`, and `git status --porcelain --untracked-files=all` no new `??`; each run's result lines and printouts are in `<record>/trial/`, beside its `trial-<n>.patch`; R3.0 is closed on the last pair (§4); the verdict is in the record (*The trial's verdict*, below) |
| U3.1 | **Unit 3.1, the fields**, by the same agent continued with the verdict: 3A, 3B.1 (the two facts rows directly before `hover_background`'s row, placed by its text, A3 (g)), 3B.2, 3B.3, 3B.5, 3C, 3D.1 to 3D.3 with `contract-leaves.toml` regenerated, 3E.1 to 3E.4, and gpui's part of 3F.4 or of 3F.5 as 3I's row of unit 3.1 lists it (where it passed: `bordered_button`, `hover_border`, the `sinks` entry, the trial's test, the `UnreadColour` of `button.focus_border_color`, the showcase's `ButtonKind::Bordered` with its claims, `edge` branch and notes, the tooltip's trigger text and the info test's new kind; and the coverage table's sentence in `CG/lib.rs`, which links `variants::bordered_button` and so goes with it, 3F.4 *Contract statement*; where it failed: the two `UnreadColour` entries with the difference the record holds). Where it passed, the trial's edits stay and are part of this commit; where it failed, the agent starts on U2.7's tree after the revert. No citation of a line it moves is re-pointed (spec 2B.6, 3B.6), but for S6's fourth gate (§2, *Citations*): a moved line a colour claim cites keeps the line count above it, or the claims are re-pointed in this commit. | `implement` | the header; 3A to 3E; 3F.4 or 3F.5; 3I's row of unit 3.1; A3; the record's verdict | (1) seen to fail: 3C's `a_buttons_hover_and_focus_border_is_the_selection` and `reader_kde.rs`' new assertions, the lists of `derived_leaves`, `resolved_leaves` and the accent test, 3D.3's `a_kde_breeze_button_hovers_and_presses_in_the_selection_border`, 3E.3's new tests (`flat_and_part_scopes_keep_their_resting_stroke`, seen to fail in this order: 3E.1's base strokes and the `keeps_resting_stroke` predicate written first, the test run red on kde-breeze, where the seven roles then carry the base's new hovered and active strokes, and only then the statement that keeps their resting stroke added; and the «button» cell test, its name the implementer's), 3E.3's changed `hover_and_active_keep_the_resting_stroke_and_radius` and `no_border_colour_carries_the_border_opacity` and `button_cell_writes_its_rows`' new expectation, and, where it passed, `a_refined_buttons_info_claims_an_edge_only_where_one_is_drawn`'s new kind; the trial's test counts as seen to fail (it could not compile before `bordered_button`); "kept" (§2: run before and after the change, both `ok` lines quoted), each passing before the change too: 3D.3's `an_adwaita_button_keeps_its_border_in_every_status`, since iced's `styles::button` hands `Active`, `Hovered` and `Pressed` the same `border` today and `Disabled` its faded form (`faded_border`); and 3E.3's changed `panel_role_cells_write_their_rows` and `switch_cell_writes_its_rows`, since `write_states` copies `inactive`'s stroke into `hovered` and `active` today, which their new assertions compare with; (2) they pass, each named `ok`: `cargo test -p native-theme --lib -- button_validate_widget_extracts_all_fields property_registry_keys_match_derive_inventory no_inheritance_fields_are_not_inherited every_bundled_preset_source_lints_clean documented_sizes`, `cargo test -p native-theme --test proptest_roundtrip`, `cargo test -p native-theme --features "$NT_FEATURES" --lib -- kde:: --nocapture` (its count quoted), `cargo test -p native-theme --features "$NT_FEATURES" --test reader_kde` (warm, as in U2.2), and `cargo test -p native-theme --features kde` (CI's leg) by the orchestrator as a long run beside the gate under the gate lock, as U2.2's legs (`<record>/gates/U3.1/legs-kde-<n>/`); `cargo test -p native-theme-egui --all-features --lib -- style:: every_leaf_has_exactly_one_row_and_every_row_is_well_formed verdict_totals_match_the_specification t2_every_row_s_declared_sinks_are_exactly_what_its_leaf_moves t10_every_sink_holds_the_leafs_value t10b_every_style_path_is_a_sink_or_unwritten t10c_every_undeclared_field_holds_its_inherited_value mapping_md_is_current --nocapture`, its count quoted; `cargo test -p native-theme-egui-widgets --all-features`; `cargo test -p native-theme-iced --all-features --lib -- a_kde_breeze_button_hovers_and_presses_in_the_selection_border an_adwaita_button_keeps_its_border_in_every_status every_colour_leaf_reaches_a_row_or_is_named` (3D.3's two tests live in `CI/contract.rs`; `CI/styles.rs` has no test module), each named `ok`; `cargo test -p native-theme-gpui --features native,watch --lib -- every_colour_leaf_reaches_a_sink_or_is_named every_mapped_field_equals_its_native_source every_colour_claim_is_read_at_the_line_it_cites every_edge_colour_is_claimed_through_edge_claim the_showcase_exercises_every_builder every_prose_citation_still_exists`, with, where it passed, `bordered_button` (its tests by that name and the trial's); where it passed, the showcase tests by the agent's own runs: `a_refined_buttons_info_claims_an_edge_only_where_one_is_drawn`, `a_disabled_buttons_info_prints_its_fade` and every test that names `basic-button`, `basic-button-tooltip`, `basic-toggle-off` or `basic-toggle-on` (found by `grep`, named in the report), each `ok` quoted; (3) the generated files by their commands only (`mapping.md`, `contract-leaves.toml`, as U2.1 (3)), the leaves file's diff read against 3D.2 and the border rows' `except` lists and each difference explained in the commit body; (4) each count equals A3 (c)'s base plus the delta (egui's leaf count and its message +2, Buttons and TOTAL leaves +2, their DIRECT +2, the `buttons` tuple's first number +2; `derived_leaves` +2; `arb_button_theme`'s vector 13); (5) no pinned line moved: `git diff HEAD -U0 -- connectors/native-theme-gpui/src/colors.rs` prints nothing; every hunk of `git diff HEAD -U0 -- connectors/native-theme-gpui/src/variants.rs` starts after A3 (f)'s last pinned line of that file and before its `#[cfg(test)]`, and, where it passed, the four claims cite the lines of `<F>`, `<H>`, `<A>` and `<R>` as they stand (3F.4); `every_colour_claim_is_read_at_the_line_it_cites` passes; (6) `python3 scripts/check_connector_parity.py` passes (a finding for `bordered_button`, which the specification does not expect, is reported, not excepted); (7) the two facts rows equal 3B.1's text (extracted from the specification and compared by the implementer's script in `<record>/u3.1/`) and stand directly before `hover_background`'s row (A3 (g)); the five checkers pass; (8) `cargo fmt --all --check` passes and the change is staged; (9) the gate passes, `--quick` (the first code unit since U2.6's, §3; not structural, rule 24), with the GPUI Fast legs (in the gate). Commit: `feat: a button's border on hover and on keyboard focus: the fields, kde-breeze and the KDE reader, and the three connectors` (the body names the trial's verdict, and where it failed the combination, the state and the item that differed, 3F.5) |
| U3.2 | **Unit 3.2, the showcases**: 3B.4 (the element lists); the gpui showcase's `HOW` entries of 3F.4 or 3F.5; iced's and egui's routes come from unit 3.1 with no code (3D.4, 3G.1); and 3H.2b, the hover run's exception text in `docs/showcase-exceptions.toml`, a TOML string and so code (rule 11), placed here so that U3.3 stays a documents unit (rule 20). S6's fourth gate (§2, *Citations*): a moved line a colour claim cites keeps the line count above it, or the claims are re-pointed in this commit. | `implement` | the header; 3B.4; 3D.4; 3F.4's or 3F.5's *Showcase: the routes*; 3G.1; 3H.2b; 3I's row of unit 3.2; A3 (g)'s key | (1) seen to fail: the element lists' new leaves, added first, fail `the_element_list_parses_and_every_leaf_has_a_route` (iced) and `every_listed_leaf_has_a_route` (gpui), each run before the `HOW` entries; (2) they pass by the agent's own runs, with the egui showcase's tests that read `docs/showcase-elements.toml` (as U2.6), each `ok` quoted; `cargo test -p native-theme-gpui --features native,watch --lib -- every_how_entry_is_shown every_prose_citation_still_exists` named `ok`; each `HOW` text checked first with `<record>/inputs/cite2-core.py` (P2's re-rooted `cite2.py`) and `prose.py` (or `prose-core.py`, P2) (the specification's transcriptions of `line_citations` and the prose parser), their output in the report; (3) `python3 scripts/check_showcase_parity.py --check-list` passes; 3H.2b's text equals the specification's block (extracted and compared by the implementer's script in `<record>/u3.1/`), placed by its key (A3 (g)); (4) `cargo fmt --all --check` passes and the change is staged; (5) the gate passes, `--commit` (rule 6: the release's last code unit before its documents and version units; the second code unit since U2.6's, §3), with the GPUI Fast legs (in the gate). Commit: `feat(showcases): a button's hover and focus border in the element list, its routes and the hover run's exception` |
| U3.3 | **Unit 3.3, the documents**, once R3.1, R3.1c and R3.2 are closed and their fixes committed: 3H.1 into the release's one CHANGELOG section U2.7 began (its step 3 entries after step 2's, and the one *Breaking Changes* entry for both structs, S5; V fills the heading's date), 3H.2 as the second paragraph of the release's one ROADMAP section, with its recount, 3H.2a (each 34 the count `soft.py` gives on the tree), 3H.3 (3H.2b is U3.2's), where the reply is unposted as the orchestrator reads it again before dispatching U3.3 (U2.7's `gh issue view 4 --comments` rule), 3H.4, the gpui README of 3F.4 or 3F.5, and, where the trial failed, request 5; each `<A \| B>` in the verdict's form; the release placeholders ("<the release that brings step 3>", the same release as step 2's, S5) written as `0.7.N`. This plan's status line gains the trial's verdict, and where it failed the state that differed (3I, decision 12): this plan's one edit by a unit before K7 (MG's path re-points, step (6), a merge's rewrites of this plan, and on the no-MG path P0's or P2's path re-points are the others, §5). 3H.5 and 3H.6 are K7's. | `worker` | the header; 3H; 3F.4's or 3F.5's README and request; A3 (a) to (c); the record's verdict and the maintainer's answers | each text equals the specification's block for the verdict (extracted and compared by the worker's script in `<record>/u3.3/`), placed by its anchor (A3 (a), (b)); each changelog entry one physical line; the five 34s equal `soft.py`'s count on the tree, in the body; `ROADMAP.md`'s count equals `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md` at the commit, in the body; the five checkers and `check_readme_versions.py` pass; the change is staged; the gate passes, documents-only, the five checkers and `--quick` (§3: the documents unit and the last unit before MG, so the lane's gated head for steps 4 and 5, rule 20, which takes its cadence's gate with no extra `--commit`, U3.2's `--commit` covering the code beneath it; recorded in the record, §8's line, once it is committed and its diff reading is closed, every review of both steps being closed before it starts; the READMEs are crate docs, and the gpui README's `rust,no_run` example, where it passed, is a doctest, which `--quick`'s doctests of the changed crates run) (in the gate). Commit: `docs: a button's hover and focus border in the changelog, the roadmap, the READMEs, the first rationale and the backlog` |

**The trial's verdict.** The orchestrator writes it into the record from the two
runs and R3.0:

- **Passed** where both result lines show `1 passed`, both printouts show every
  state of 3F.2's table on all six combinations with only the hover and press
  border differing, that border being the variant's `hover_border_color` where the
  `Default` button's is the `input` token (3F.3), and R3.0 is closed with nothing
  serious found.
- **Failed** where both runs show `1 failed`, the failure names a combination, a state
  and an item that differs (3F.3; decision 12), and R3.0 is closed with nothing
  serious found, so the failure is the variant's and not the test's. A failure of
  the text-colour assertion, which 3F.3 counts as a failure, names the
  text colour as its state (the variant's `foreground` against
  `button_foreground`, 3F.2). Where the two backends fail on different
  combinations, states or items, the verdict is still failed: the record holds each
  backend's combination, state and item apart, the maintainer is told both, and
  the texts that name the state that differed (3F.5, request 5) name each.
- Any other result decides nothing and is reported: a panic, a build error, a
  message that names no state, a result line with `0 passed` (the filter found no
  test), `1 passed` with a printout that shows a difference besides the hover
  and press border, shows that border other than as the variant's
  `hover_border_color` against the `Default` button's `input`, or lacks a state or
  a combination, or any result, `1 passed` on both included, in which R3.0 finds
  something serious; the test or the code is
  fixed (within 3F.1's form, U3.0), R3.0 reads the fix, and both runs are made
  again with the next `<n>`.
- **Passed on one backend and failed on the other**: a stop (§6); the
  specification decides neither, and the maintainer is asked.

The maintainer is told the verdict (where it failed, with the state, and that
request 5 is then drafted for the maintainer to send). Where it failed, the trial's
edits are reverted before U3.1, with the record holding every `trial-<n>.patch` and the outputs:
`cd <core worktree> && git restore --staged --worktree --source=HEAD -- native-theme/src/model/widgets/mod.rs native-theme/tests/proptest_roundtrip.rs native-theme/src/presets/kde-breeze.toml connectors/native-theme-gpui/src/variants.rs connectors/native-theme-gpui/src/widgets/tests.rs`,
after which `git status --porcelain --untracked-files=no` prints nothing (3I).

### 2.5 The merge, X and the version

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| MG | **The merge of `<prev-tag>`** (S2 as amended on 2026-11-15: the tag of the release immediately before this one in the maintainer's order, merged, not rebased onto, so no sha changes), once, after U3.3 is committed (its `--quick` passed, the lane's gated head for steps 4 and 5, rule 20, §8), every review of both steps is closed with its fixes committed, and `<prev-tag>` exists; never while a trial's or a unit's edits are uncommitted, with no agent working in the worktree and no gate running. The lane waits here for the tag. Where P2 made the branch from the tag itself, there is no MG. In rule 21's order, (1), (3), (6), (7), (4), then (5) ((2) is superseded by S6; the numbers stay as rule 21 gives them; rule N here is the batch's, its Appendix):<br>(1) **The merge**, in rule 21 (1)'s order. 1. The tag: `cd <core worktree> && git rev-parse -q --verify <prev-tag>`, and only where it fails `git fetch origin tag <prev-tag>`; then in any case the first field of `cd <core worktree> && git ls-remote origin 'refs/tags/<prev-tag>'` is compared with `git rev-parse <prev-tag>` (the tag ref's own object on both sides, equal for a lightweight tag, as this project's release tags from v0.5.9 on are, and for an annotated one, as some older ones are; tags are shared by every worktree, and a release tag has been moved before): no line printed, or a different sha, is a stop. 2. The ancestry check: `cd <core worktree> && git merge-base --is-ancestor <lane-pred-head> <prev-tag>` exits 0 (rule 18: the tag holds the lane predecessor's release), a stop where it fails. 3. The untracked-overwrite check: the paths both `cd <core worktree> && git ls-files --others --exclude-standard` and `git diff --name-only --no-renames --diff-filter=A <lane-pred-head> <prev-tag>` list, each compared with `git show <prev-tag>:<path> \| cmp - <path>`: removed where equal, a stop otherwise. 4. The merge, with the HEAD before it recorded: `cd <core worktree> && git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>` (rule 17: diff3, so that every conflicted hunk carries its base section; it never commits on its own, so the regeneration and the resolutions below always come before the merge commit). 5. `Cargo.lock` and the version comparison: a conflicted manifest (`Cargo.toml`) is resolved first, by item 6's rules, before this item's cargo commands; where the merge left `Cargo.lock` conflicted, `<prev-tag>`'s side is taken (`git checkout --theirs Cargo.lock`) and this release's lock changes are re-applied, one `cargo update -p <name>@<theirs-version> --precise <ours-version>` (which resolves without building) per package this release changed, through the wrappers (§5); a package this release added or removed has no `<theirs-version>`, so `cargo metadata --format-version 1 > /dev/null` first resolves the missing or surplus entry without building or upgrading others, and the diff against `<prev-tag>`'s lock afterwards shows only this release's packages, else a stop; the commands are derived from `git diff <base> <the HEAD recorded at item 4> -- Cargo.lock`. Then the version comparison (rule 19, step (3) below), a read of `Cargo.lock` that needs no gate, here so that a stop wastes no build and its result tells whether the merged tree can build before VF. Last, `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo metadata --locked --format-version 1 > /dev/null` passes (it builds nothing, and catches a lock that merged cleanly but is inconsistent). 6. The resolution of the conflicts, the orchestrator's: it leaves out `Cargo.lock` (item 5) and the generated files (item 8), never runs `git checkout --conflict=…` on a file it has touched, and resolves every conflicted hunk by both sides' quoted texts (both sides' specifications), a citation like any other text (S6: an old line number in a conflict takes either side's text, both drift alike, and a new citation has none; where one side re-pointed a line a gate reads by number, S6's fourth gate or rule 25, that side wins, and where both sides re-pointed the same one, the number is the base plus both sides' deltas, confirmed by the warm-up gate), each hunk named in the record (file, hunk, resolution); a stop where a resolution would change what either side's text means (§6); a prose count takes `<prev-tag>`'s side and is named in the record (X recounts it), while a count or verdict tuple a test or gate checks (egui's leaf count and verdict totals, `derived_leaves`' length, `arb_*`'s vector lengths) is the base plus both sides' deltas; `CHANGELOG.md` puts this release's section first and `ROADMAP.md` puts this release's section after `<prev-tag>`'s (each release's section in release order, 2I.2). 7. (Superseded by S6: there is no second pass.) 8. Generated files (`mapping.md`, `contract-leaves.toml` and every file a generator writes) regenerated by their commands on the merged tree through the wrappers, never resolved by hand, always as a long run (rule 21 (1) item 8: at every MG `<prev-tag>`'s V has bumped the workspace crates in `Cargo.lock`) (where the merged tree does not build before VF, *The pending generated file* below the steps). 9. `cd <core worktree> && git add` of the resolved and regenerated files, each by name, conflicted or clean. 10. The marker check (*MG's marker check*, below the table) prints nothing; a marker-like line that `<prev-tag>`'s own file has at that place is not counted. Then `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo fmt --all --check` passes before the merge commit (it builds nothing); on a failure, `cargo fmt --all` runs through the same wrappers, the files it changed are re-staged and named in the record, and the check runs again; a file it changes outside the staged set is a stop. 11. The staged-path check: every path of `git diff --cached --name-only --no-renames` is in `git diff --name-only --no-renames <lane-pred-head> <prev-tag>` or is a regenerated file the record names; any other path is a stop, named in the record. 12. The commit, `git commit --no-edit --cleanup=strip`; `<merge>` recorded. 13. The parents check: `cd <core worktree> && git log -1 --format=%P <merge>` prints the HEAD recorded at item 4 and `git rev-parse <prev-tag>^{commit}`, in that order; then `grep -m1 '^## \[' CHANGELOG.md` prints this release's heading and `git status --porcelain --untracked-files=no` is clean.<br>(3) **VF** (rules 19 and 25), after item 5's version comparison of the crates P2 lists, before (the HEAD recorded at item 4) and after. A source the new version needs that Cargo's registry lacks is fetched by the orchestrator with `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo fetch`, which builds nothing. A version within P2's accepted set (egui and egui-winit 0.36.2 or 0.37.x, patch releases of the others) is VF's rework (likely here: `<prev-tag>` is, under D10, egui 0.37's `v0.7.2`, which moves egui from 0.36.2 to 0.37.0); any other is a stop. Where one changed, VF (§2.5) runs here, before the warm-up gate, which covers its commits; A2's and A3's version readings are made again by VF's `worker`, and P2's adapted inputs are made again where a path's version moved; "none needed" in the record where no version moved.<br>(6) **The path check** (rule 22 (b), before the warm-up gate): every token that starts with `docs/` in this specification and this plan (its `:N` suffix and `#…` stripped) is checked with `cd <core worktree> && git ls-files --error-unmatch <path>`, and every relative `](…)` link of this specification and this plan, which live in `docs/`, is checked from there, `cd <core worktree>/docs && git ls-files --error-unmatch <link>`, brace sets expanded; exempt are paths cited "on <branch>" or "at <sha>", paths a unit of this release creates, placeholders (`<…>`, `…`), glob or regex patterns, URLs, `crate::` links, quoted texts of other files and the close-out's archive paths; one that moved to `docs/archive/` is re-pointed by a `worker` in a documents-only commit, one that is gone otherwise is a stop.<br>(7) **The survived-the-merge checks**: A2 (e)'s and A3 (e)'s struct-literal searches and every count A2 (c) and A3 (c) based, run again on the merged tree (each count a test or gate checks the base plus both sides' deltas; a prose count resolved to `<prev-tag>`'s side in step (1) is listed for X's recount, not compared); this release's blocks on the merged tree compared with their state at `<merge>^1` (`git show <merge>^1:<path>`), not with U2.7's or U3.3's scripts, whose blocks a later unit may have changed: every text this release wrote still stands at its anchor (but for such a prose count, listed for X) and every text it replaced is absent; the anchors of the later tasks found on the merged tree (V's `## [0.7.N] - <date>` heading, every status line the close-out sets: the rationale's, which K7's sentence of 2I.6 and 3H.5 joins, the specification's and this plan's, and `docs/archive/README.md`'s rows that 3H.6's row follows); the reply's posted state read again (rule 16, U2.7's command): where it was posted since U2.7, a documents-only commit reverts U2.7's and U3.3's draft edits and the maintainer is told; the untracked planning documents listed again (§2); and the list of the files either side changed, `git diff --name-only <base> <merge>^1` and `git diff --name-only <base> <prev-tag>` (the union, `<base>` the branch's base: `<prev-tag>` where P2 made the branch from it, else `<lane-pred-head>`), with the files only the other side changed marked, written into the record for X. These checks read HEAD as it stands at step (7), after steps (3) and (6) (rule 21 (7)): the blocks VF rewrote in step (3) are expected differences from `<merge>^1`, listed in the record from VF's commits, each compared with the form the specification gives for the new version; a survived check never undoes VF.<br>(4) **R-MG** (§4), where a file was resolved in step (1) or written by step (7), except Markdown that no test parses (so `.rs` files, manifests, workflows, configuration files, a TOML a test reads, a script, `docs/platform-facts.md`, a README, which is compiled as doctests and parsed by tests, and `connectors/native-theme-egui/src/mapping.md`, which `mapping_doc.rs` reads, are in), where step (1) resolved `Cargo.lock`, or where VF committed anything: a `review` (a `review-hard` where those commits change a hard part of a core: a resolver, validation, a runtime, concurrency, `unsafe`) of `git show --remerge-diff <merge>` and of every commit of steps (3), (6) and (7), with the record's list of resolved hunks, VF's review always within it; a must-fix is a new commit by the author of the commit it fixes, made before the warm-up gate, which covers it (rule 21: no gate of its own).<br>(5) **The warm-up gate**, last (§3: `--commit` on the clean tree), covering every commit of steps (1), (3), (4), (6) and (7) (VF's, an R-MG must-fix, a path re-point, a fix the survived checks need), which have no gate of their own, and building this release's units against what the merge brought; the GPUI Fast legs (§2) always run beside it, under the gate lock, since the merge brings the other side's code (and `<prev-tag>`'s version bump) (rule 21 (5)). A red warm-up goes, with the log's failing part, to the author of the unit whose rule the failing test checks (a full struct literal, a count, a mapping), even where the failing file is the other side's (rule 12; a fix after the units, §6); it may also be a late fallback (§2.2). One a conflict resolution caused is the orchestrator's to fix; one whose cause is unclear is a stop. The fix of a red warm-up is gated by the warm-up run again (rule 12), with a generated file still pending regenerated in the fix's commit first (*The pending generated file*, below).<br>Agents in the steps before (5) run no cargo but `cargo fmt --all --check`, which builds nothing (a regeneration is the orchestrator's long run); the one exception is VF's `implement` agent, which may run one `nice 19` `-p` build or clippy of the crate it rewrote, through `<tools>/slot.sh <tools>/lowprio.sh` (rule 21). Each step's fixes are made before the next step; each fix after the warm-up is a commit with its own gate, as the task whose files it touches (§3's table: a path re-point is documents-only). **The pending generated file** (rule 21 (1)): generated files the merge touched are regenerated by the orchestrator before the merge commit, always as a long run under the gate lock (§3, *Long runs*); where the merged tree does not build before VF, the generated file is taken from the HEAD recorded at item 4, recorded as pending, and regenerated in VF's code commit; where VF makes no commit, by the orchestrator in a commit of its own before the warm-up gate (`chore: <file> regenerated on the merged tree`), which covers it; and where the tree builds only after a warm-up fix, in that fix's commit, before the warm-up runs again. `mapping.md` is the likely one, where `<prev-tag>` changes egui's `mapping.toml` as this release does (2G.2; U3.1's egui rows). No warm-up gate counts as passed while a generated file is pending. Nothing of this release is pushed, and no close-out step runs, before MG (S2). | orchestrator; a `worker` for step (6)'s re-points; VF's agents in step (3); R-MG's `review` or `review-hard` | S2; §1 | before the merge, the tag check found `<prev-tag>`'s sha on `origin` (the first field of `git ls-remote origin 'refs/tags/<prev-tag>'`) equal to `git rev-parse <prev-tag>`, and `git merge-base --is-ancestor <lane-pred-head> <prev-tag>` exited 0; the marker check printed nothing and the staged-path check found no other path; `git log -1 --format=%P <merge>` prints the HEAD recorded at item 4 and `<prev-tag>`'s commit, in that order; `grep -m1 '^## \[' CHANGELOG.md` prints this release's heading; `git status --porcelain --untracked-files=no` prints nothing; the conflicts and their resolutions, `Cargo.lock`'s re-applied changes, the version comparison, the re-reads, the path check, the survived checks and the either-side list are in the record; R-MG closed where it ran; no generated file pending; the warm-up gate passes (`<record>/gates/warmup-<n>/`) |
| VF | **The version forms after MG** (rule 25; spec 2K's re-read), inside MG's step (3), right after its version comparison and before its warm-up gate: for every crate whose version changed within P2's accepted set (likely egui and egui-winit 0.36.2 to 0.37.0, `<prev-tag>` under D10 being egui 0.37's `v0.7.2`), a `worker` re-reads every upstream line the specification cites in the new version (A2's and A3's version readings made again); an `implement` agent rewrites the forms this release's units took on the older version in code and tests, each only where its content (behaviour, value, API) changed in the new version, never to update a version-pinned citation that still holds (rule 25, S6), with one exception, as A2 (b) and A3 (b) have it: a line a gate reads by number (a `HOW` text's or colour claim's line in a crate of `CITED_CRATES`, `CG/showcase.rs:2049`: gpui-component, gpui-base, gpui-pre, gpui-pre-wgpu, egui; the fallback-stack test's gpui-pre lines) is re-pointed where the new version moved it (rule 25) (the modifier keys of 2G.1 read from egui's `key.rs`; a generated file MG recorded as pending, regenerated by the orchestrator's long run into this commit; where VF makes no commit, MG's *pending generated file* rule says where it is regenerated), each against the new version's source in Cargo's registry; and a `worker` the documents' forms (texts that state what a toolkit does, in the specification's own written texts: the CHANGELOG, the backlog, the READMEs), each found by its text and under the same restriction. Its agents run no cargo but `cargo fmt --all --check`, which builds nothing, except that its `implement` agent may run one `nice 19` `-p` build or clippy of the crate it rewrote, through `<tools>/slot.sh <tools>/lowprio.sh` (rule 21); a source the new version needs that the registry lacks is fetched first by the orchestrator, `cargo fetch` through the wrappers (§5, *Cargo*) (rule 21: the warm-up gate builds and tests them; code that compiled against 0.36.2 and that the warm-up finds red is the warm-up's fix, step (5), not VF's). "None needed" in the record where no version moved or no form changed. | `implement` for code, `worker` for the re-read and the documents | 2K; rule 25; MG's comparison | each form changed listed with its old and new text and the content change that required it in the commit body; the code commit reviewed within R-MG, always (MG's step (4) runs wherever VF committed anything), and the documents commit with it; both commits made before MG's warm-up gate, which covers them, with no gate of their own (rule 21). Commits: `fix: the toolkit forms after egui <version>` and `docs: the toolkit forms after egui <version>` |
| X | **The recount, once for the release** (2K, *X*; rule 17), after MG (VF inside it), on the merged tree, `<base>` the branch's base (`<prev-tag>` where P2 made the branch from it, else `<lane-pred-head>`; with no MG there is no X, rule 17: nothing was merged, and U3.3 ran the five checkers): every stated count in every file either side changed (MG's either-side list: the union of `git diff --name-only <base> <merge>^1` and `git diff --name-only <base> <prev-tag>`), outside the pinned documents (`docs/archive/`, `docs/evidence/`, `docs/issues/`, the dated `docs/todo_v0.7.(1\|x)_*` documents and the files beside them, `docs/todo.md`'s dated block as A2 (d) found it), recounted on the merged tree, the prose counts MG's step (1) resolved to `<prev-tag>`'s side among them; then the five documentation checkers. No citation is re-pointed (S6). A generated file is regenerated by its command, never edited. | `worker` | 2K; 3I; A2 (d) | each count with its old and new value and how it was counted, in the commit body; the five checkers pass; the change is staged; the gate passes, the five checkers and `--quick` (§3: a recount changes prose, or a count inside a message string, which makes it a code unit; rule 6's mark is U3.2, and V's `--commit` covers X, so it takes `--quick` either way), with the GPUI Fast legs where X's commit touches a gpui connector or showcase file (§2) (in the gate). Commit (where any count changed): `docs: the stated counts on the merged tree` |
| V | **The version** (the release mechanics of `a3c10475`), after X: the workspace version and the inter-crate requirements become `0.7.N`, the lock follows, the `<date>` of `## [0.7.N] - <date>`, which U2.7 wrote (2I.1; never `## [Unreleased]`), becomes the day's date, with its compare link at the top of the link list, `[0.7.N]: https://github.com/tiborgats/native-theme/compare/<prev-tag>...v0.7.N` in the form of `[0.7.0]`'s (the release step's own instruction, `scripts/check_release.sh:1417`: "Date the CHANGELOG entry, set its compare link"; at `0fa28027` the list has no `[0.7.1]` link, its first is `[0.7.0]`, `CHANGELOG.md:1802`: that gap is the 0.7.1 release's, not this plan's, and is recorded, not fixed). The rationale's status-line sentence is K7's (2I.6, 3H.5: written with the archive's path). Before the close-out's first step, V checks that no stamped path names this release's planning documents at their present place (rule 22 (c)), with the command of *V's stamped-path check* below the table, which prints nothing; a hit is re-pointed to its `docs/archive/…` path in V's commit. Under D0 (b) every README's `native-theme*` requirement also moves to the new `major.minor`, as `scripts/check_readme_versions.py` requires. Where there is no MG, V reads the reply's posted state again (rule 16, U2.7's command); where it was posted since U2.7, a documents-only commit reverts U2.7's and U3.3's draft edits and the maintainer is told. | orchestrator | §3's version check | §3's version check; `python3 scripts/check_readme_versions.py` passes; the five checkers pass; `--commit` passes (a lock change, §3). Commit: `chore: v0.7.N version` |
| K1 to K8 | **The close-out, once for the release**, §2.6. | orchestrator; the maintainer for the capture window | 2K; 2H.5; 3G.2; 3H.5; 3H.6; 3I | §2.6 |

*V's stamped-path check* (rule 22 (c)), with `<stamped paths>` the paths of
`scripts/update_provenance.sh`'s `SOURCE_PATHS` (`:39` at `0fa28027`) and the source
paths `scripts/update_compatibility.sh`'s `source_paths` gives for every connector
(`:69-78` at `0fa28027`: each connector's `Cargo.toml`, `src`, `examples` and `tests`,
and for egui also its `mapping.toml` and the egui widgets crate's paths), read on the
branch and written out literally; the stem matches the brace form
`…-steps2-3-{spec,plan}.md` too, and the filter works per match:

```bash
cd <core worktree> && git grep -noE '(archive/)?todo_v0\.7\.x_kde-open-items-steps2-3-[a-z{},.-]*' -- <stamped paths> | grep -v ':archive/'
```

*MG's marker check* (rule 21 (1), item 10), after item 9's `git add`; it prints
nothing (diff3's `|||||||` included):

```bash
cd <core worktree> && git diff --cached -U0 | grep -E '^\+(<{7}|\|{7}|={7}|>{7})( |$)'
```

### 2.6 The close-out, once for the release (S5, S4)

In this order; each step starts only when the one before it holds (K4 and K5
beside K3's CI excepted), and K1 only when every review of §4, of both steps, is
closed and its fixes committed, and MG (with VF), X and V have run (S2).

1. **K1.** `cargo audit` passes with the repository's `.cargo/audit.toml`.
2. **K2.** `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/update_compatibility.sh run` passes, a long run under the gate lock (§3; rule 2: a heavy run); it
   leaves `docs/COMPATIBILITY.toml` and the connector READMEs changed and
   uncommitted until K7. An upstream release that moved, or an advisory, stops
   the run (§6).
3. **K3.** Once K2 has passed, `git push origin v0.7.N-rc1:v0.7.N-rc1`, both steps'
   units pushed together (2K; S4: the push comes before the full check, which runs
   beside CI). CI runs on pull requests and `main` only
   (`.github/workflows/ci.yml:6-10`), so at this first push the orchestrator opens a
   draft pull request from `v0.7.N-rc1` to `main` (approved 2026-11-15, §7 item 3;
   rule 9), its title and description shown to the maintainer. The run is green. The cost S4
   accepts: a red run emails the maintainer for a fault K4 would have shown first.
4. **K4**, while K3's CI runs: the full `scripts/check_release.sh`, a long run under
   the gate lock (§3, *Long runs*),
   fails on the provenance stamp alone (K2 renewed the other). Its warnings are
   named in the record. Two are expected: the known `cargo outdated` warning, zbus
   excluded and named as excluded (`0fa28027`), whose list of outdated names and
   versions is whatever it lists at the run and is recorded whole; and the semver
   step (soft), expected under D0 (a) to name `ThemeDefaults`' one new field and
   `ButtonTheme`'s two and nothing else of this work (2K, *Semver*; the release's
   one break, 2I.1, 3H.1); under (b) and (c) nothing of this work. The semver step
   compares each crate with its newest release on crates.io
   (`scripts/check_release.sh:1131-1132`), so where `<prev-tag>`'s release
   is not yet published there, what `<prev-tag>` changed (under D10 egui 0.37's
   changes) is reported too: expected, and named in the record as `<prev-tag>`'s,
   each item read from its report. Any other failure or
   warning is a stop (§6); a fix it needs follows the one close-out fix rule (§6).
5. **K5**, while K3's CI runs (it commits nothing): the three showcases built
   for release with the capture scripts' own commands
   (`scripts/capture_window.sh:440`, with each script's features), under the gate
   lock and through both wrappers (`flock <locks>/gate.lock <tools>/slot.sh
   <tools>/lowprio.sh …`, rule 2: heavy runs), each a long run (§3); and a rehearsal of the parity step's layout
   half on a private X server (Xvfb display `:97`, which the second lane's
   rehearsal uses too: one rehearsal at a time across both lanes, held by the
   shared lock `<locks>/xvfb97.lock`, so a lane waits mechanically; rule 24, §6),
   with no `kde-inhibit` and through `slot.sh` alone:
   each release showcase started with `--dump-layout`, with the presets the
   capture scripts use (P1's copy `<record>/inputs/rehearsal.sh`, whose `W=` names
   `/home/tibi/Rust/native-theme/.claude/worktrees/v071` (its line 6) and is set
   to the core worktree's absolute path,
   `/home/tibi/Rust/native-theme/.claude/worktrees/kde-b`, in a copy of its own,
   `<record>/assets/rehearsal.sh`, the copy's `W=` line and its `diff` against
   P1's copy written into the record before it runs (rule 24), whose `ENTRIES` are first checked against
   `scripts/capture_window.sh`'s `SCREENSHOT_ENTRIES` on the branch, `:352` at
   `0fa28027`, and brought into line where they differ; it runs the built binaries
   and no cargo, and takes its output folder as its one argument, which receives
   the dumps, the logs and `summary.txt`, whose last line is `DONE`), then the
   comparator on the dumps:

   ```bash
   cd <core worktree> && flock <locks>/xvfb97.lock <tools>/slot.sh bash <record>/assets/rehearsal.sh <record>/assets/rehearsal-<n>/dumps/ > <record>/assets/rehearsal-<n>/LOG 2>&1; echo $? > <record>/assets/rehearsal-<n>/STATUS
   cd <core worktree> && python3 scripts/check_showcase_parity.py <record>/assets/rehearsal-<n>/dumps/ > <record>/assets/rehearsal-<n>/parity.out 2>&1; echo $? > <record>/assets/rehearsal-<n>/parity.STATUS
   ```

   The first line is a long run, in a new `rehearsal-<n>/` made first with `mkdir`
   (§5, *Run folders*), its log and `STATUS` inside it and the script's output in
   its `dumps/`; the second reads the dumps once `dumps/summary.txt` ends in `DONE`
   and `STATUS` is written, and the comparator's exit status goes into the record
   with `parity.out`. The orchestrator reads `summary.txt` too: each line
   `<name> dump=yes|no` (`rehearsal.sh`'s own form); any `dump=no` leaves the
   rehearsal incomplete, and it is run once more in the next `rehearsal-<n>/`; a
   `dump=no` in that second run is a stop (§6). It compares geometry only and runs
   without a window manager; a difference it shows, before the capture, is reported
   while the capture goes ahead (rule 19); the fault rule 19 means is the asset run's
   comparator's difference after the capture (K6, §6 *A parity difference*).
6. **K6.** When both are green in rule 9's sense (K3's CI green, and K4's full check's
   only failure the provenance stamp, which only the asset run refreshes; its expected
   warnings named) and K5 is done (S4), the asset run takes the shared lock
   `<locks>/assets.lock`, the gate lock `<locks>/gate.lock` and a cargo slot first,
   and only then is the maintainer
   asked for the capture window (rule 24, §6: one asset run and one capture window
   of the batch at a time). In a new `run-<n>` folder made first, the orchestrator
   writes with the Write tool `<record>/assets/run-<n>/run.sh`, one line, with no
   `slot.sh` (the outer command holds the slot):
   `CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16 bash scripts/generate_assets_release.sh > <record>/assets/run-<n>/LOG 2>&1; echo $? > <record>/assets/run-<n>/STATUS`,
   and starts the long run (rule 24's form):
   `cd <core worktree> && flock <locks>/assets.lock flock <locks>/gate.lock <tools>/slot.sh sh -c 'touch <record>/assets/run-<n>/HELD; until [ -e <record>/assets/run-<n>/GO ]; do sleep 5; done; kde-inhibit --power --screenSaver --notifications bash <record>/assets/run-<n>/run.sh'; echo $? > <record>/assets/run-<n>/OUTER.STATUS`,
   every placeholder written as its literal absolute path (§5, *Cargo*). It waits
   for `HELD` with the Monitor tool's until-loop
   (`until [ -e <record>/assets/run-<n>/HELD ] || [ -s <record>/assets/run-<n>/OUTER.STATUS ]; do sleep 10; done`:
   `HELD` means the two locks and the slot are held; an `OUTER.STATUS` written first is
   a failure before `HELD`, which ends the wait and is reported with the log, the
   maintainer not asked), then asks the maintainer for "capture now": the desktop unlocked and
   untouched for 4 to 6 minutes, announced before it starts; on the answer,
   `touch <record>/assets/run-<n>/GO` starts the capture. The asset script runs once
   (it needs HEAD pushed and the stamped paths clean). It holds a slot but does not run under
   `lowprio.sh`: the desktop is reserved for it and the captures are
   timing-sensitive. `kde-inhibit` hides the script's exit code, so the
   orchestrator reads `STATUS` and the log's tail, never the wrapper's status. The
   orchestrator waits for "Theme-switching GIFs generated"
   (`scripts/generate_assets_release.sh:172`) with the Monitor tool's until-loop on
   the files (never a `pgrep -f` loop):

   ```bash
   until grep -q 'Theme-switching GIFs generated' <record>/assets/run-<n>/LOG || [ -s <record>/assets/run-<n>/STATUS ] || [ -s <record>/assets/run-<n>/OUTER.STATUS ]; do sleep 10; done
   ```

   When the line is there it tells the maintainer that the desktop is free (a
   `STATUS` or `OUTER.STATUS` written first is the run's end, read as such); the script then waits
   for the CI screenshots and ends by running `update_compatibility.sh run`
   (`scripts/generate_assets_release.sh:278`), whose builds keep the machine busy
   until `STATUS` is written, which the maintainer is told with the desktop's
   release. No agent edits the tree until the run has reported. A stop commits
   nothing; a parity difference is handled as rule 19 says (§6, *A parity
   difference*). A fix the
   run or its reading needs follows rule 15 as §6's one close-out fix rule
   applies it, which says when K6 is made again (a new `run-<n>` folder and a
   new capture window) before K7. Then the
   orchestrator reads two things. (a) The parity step's closing list, "Parity
   exceptions that matched no difference in this run:"
   (`scripts/check_showcase_parity.py:2092`), copied from the log into the record
   (2K: an exception written for the old ring can go stale; step 3 changes no
   capture, 3G.2). (b) The new screenshots beside the previous release's, against
   2H.5 and 3G.2: the Linux captures and theme-switching GIF frames of kde-breeze
   and material-3 (light and dark) in the three showcases, and the windows-11
   captures of the CI run, show the start-up field without the ring (step 2); no
   capture shows a button's new outline (step 3); the macOS and catppuccin-mocha
   captures do not change. What it found goes to the maintainer; a difference from
   that list (a capture of the three presets that still shows a ring, a button
   outline, or a change elsewhere) waits for the maintainer's answer before K7
   (2H.5's reading is **not verified** against the files).
7. **K7.** One commit, the asset run's and the archive, the last task (3H.5, 3H.6):
   the assets, both stamps, the READMEs' lines, the changelog heading's date set to
   the commit's day where it differs from V's; the one status-line sentence for
   both steps (2I.6, 3H.5, S5) in the rationale, with the archive's path and that
   date; only where this release is the batch's last in the maintainer's order
   (under D10's proposal it is not: steps 6 and 7 are), the batch rationale,
   `docs/todo_v0.7.1_batch-speed-rationale.md`, moves to `docs/archive/` too, by the
   same rules (rule 22 (a)); this specification and this plan move to `docs/archive/` with their names
   (`git mv`, so that `git diff --cached --name-status` lists them as renames, `R`,
   beside the asset files); `docs/archive/README.md` gains 3H.6's row; every
   path mention of the two documents in every tracked file outside `docs/archive/`
   is rewritten for the new place (the batch's rule 22 (a), v0.7.1's convention:
   a document whose header dates its lines is exempt only for line citations,
   never for a path mention), found after the move with
   `cd <core worktree> && git grep -nE 'kde-open-items-steps2-3' -- ':!docs/archive'`:
   the rationale's status line, the documents' own links (relative to `docs/`
   today: `issues/…` → `../issues/…`,
   `archive/todo_v0.7.1_kde-hand-overrides-spec.md` →
   `todo_v0.7.1_kde-hand-overrides-spec.md`, step 1's specification to where its
   close-out put it), and every other mention the `git grep` finds in the later
   steps' documents and elsewhere (the `git grep` is the rule, not a list, 3H.6).
   Three kinds of mention are left as they are,
   each named in the commit body (rule 22 (a)): a citation dated "at `<sha>`"; a
   path inside another document's instructions that names its own future target
   (at the time of writing, the steps 4–5
   plan's close-out sentence on this specification's own links, which begins "own
   links: `issues/…` → `../issues/…`", and the steps 4–5 specification's B.11
   sentence that says which links "become `todo_v0.7.x_kde-open-items-steps2-3-spec.md`");
   and a path of this plan's record folder, `<scratch-keep>/todo_v0.7.x_kde-open-items-steps2-3-plan/`,
   which does not move (in the steps 4–5 plan, found by its text); each found by its text, not its line. On the likely path v0.7.1's commit (0) added those documents
   (P0 above), so they are tracked and the same `git grep` finds them; a later
   document still untracked at K7 is not this plan's to edit, and its own plan's
   path check after its merge re-points what moved (the batch's rule 22 (b)). The
   status lines of the specification and this plan say implemented, with the
   release's commit range, `cd <core worktree> && git log --oneline <prev-tag>..HEAD`
   (rule 21: it keeps the commits this release made and leaves out what `<prev-tag>`
   brought; with no MG the same range), its first and last commits named;
   §8 holds the record up to this commit. New
   files: the asset files the script wrote that `HEAD` has not (each named from
   `git status --porcelain --untracked-files=all` of the asset folders, in the
   record); the staged-tree check of §2 holds before the commit. **Checks:** a
   mention that already points into `docs/archive/`, in any relative form or as an
   absolute URL, is left (rule 22 (a)); the check filters per match, not per line,
   and matches the documents' `.md` form only, so that the record folder's paths
   (`…-steps2-3-plan/`, rule 22 (a)'s exemption) do not match:
   `cd <core worktree> && git grep -noE '(archive/)?todo_v0\.7\.x_kde-open-items-steps2-3-(spec|plan|\{spec,plan\})\.md' -- ':!docs/archive' | grep -v ':archive/'`
   prints nothing but the mentions left by rule 22 (a)'s exemptions (a mention that
   already points into `docs/archive/` is filtered out; a citation dated "at
   `<sha>`", or a path inside another document's instructions that names its own
   future target, above), each named in the commit body (tried on the files of
   2026-11-15 with `grep -rnoE` over the untracked documents: the record-folder
   paths of the other plans and of this plan no longer match, and every
   mention the earlier `[a-z{},.-]*` form found that is not a record-folder path
   still does); and the moved documents' own relative links are re-checked
   from their new folder (`cd <core worktree>/docs/archive && git ls-files --error-unmatch <link>`
   for each, brace sets expanded), with rule 22 (b)'s exemptions: placeholders
   (`<…>`, `…`), `crate::` links, quoted texts of other files and URLs, each
   exempt link named in the record. The archive index check of
   `scripts/check_release.sh:1033-1056` (every `docs/archive/*.md` named in
   `docs/archive/README.md`, brace sets expanded), its Python body saved
   unchanged from the branch's script as `<record>/archive-index.py`, passes on
   the staged tree: `cd <core worktree> && python3 -I <record>/archive-index.py`.
   A script in the record lists every relative `](…)` target of the moved
   documents, and of every file whose link K7 rewrote, that does not exist, with
   the same exemptions (placeholders, `crate::` links, quoted texts of other files,
   URLs; rule 22 (a), (b)), and lists none. The gate is the five checkers and the orchestrator's reading of the
   diff; K8 checks the rest. Pushed with the same refspec. A fix to K7's own staged
   edits is made within K7, before its commit; a later fix follows the one
   close-out fix rule (§6).
8. **K7x**, where (a) of K6 listed exceptions: each is scoped or removed (2K), in a
   commit of its own by a `worker`, the choice and its reason in the commit body
   (an exception that matched no difference in any compared scope is removed).
   `docs/showcase-exceptions.toml` is no stamped path (`scripts/update_provenance.sh`,
   `SOURCE_PATHS`), so the stamps stay fresh. Done when
   `python3 scripts/check_showcase_parity.py --check-list` and the five checkers
   pass and the gate passes (§3's table: the five checkers and `--quick`; K8's full
   run, after it, runs every step of `--commit`). Commit:
   `docs(showcases): the parity exceptions this release's captures no longer match`.
   Pushed with the same refspec.
9. **K8.** The full `scripts/check_release.sh` at the last commit, a long run under
   the gate lock (§3, *Long runs*),
   reports no failure (K4's two expected warnings still appear, named in the
   report), and the last push's CI run is green; both are reported to the
   maintainer (the summary line, the run's id and result), not written into the
   plan.

Then the orchestrator runs `/home/tibi/.claude/scripts/sweep-target.sh <core worktree>`
and reports the sizes it prints; `kde-b` and its build folder stay for the caret,
and are deleted when steps 6 and 7 close (§1), which that report and the project status note name. **Merging the pull request, tagging,
publishing, the GitHub release, the pull request's title and description, posting
the reply to the reporter (after the release of steps 4 and 5, S5) and sending the
upstream requests (the fourth's new sentences, 2I.4, and the fifth where the trial
failed) are the maintainer's** and are not part of this plan.

## 3. Gates

- **While working**, an agent runs the tests of the module it changes. **Before
  it stages**, it runs its task's targeted done-criterion commands once, in the
  foreground, through the wrappers (§5), each expected to finish well within 10
  minutes; it runs no `--commit`, full check, asset run or GPUI Fast leg (§2,
  *Who runs what*).
- **The gate and its cadence** (S1, `docs/todo_v0.7.1_batch-speed-rationale.md`;
  2K's *Units*): the orchestrator's long run on the staged change of every task
  that changes the tree, then the commit of exactly the gated tree with the
  author's message (the batch's rule: the orchestrator commits). Which gate each
  task takes, by S1's rule (during a unit, the agent runs only the touched tests,
  its targeted commands; at a unit's end, `--quick`, by the orchestrator, as a
  long run until its time is measured, the first timed into the record;
  `--commit` every third code unit of the release, counted from the last
  `--commit` (any `--commit`, a warm-up gate's or a `--commit`-gated fix's
  included, restarts the count, while a `--quick`-gated fix does not count toward
  the next third; the table below shows the marks where none falls between), on the
  release's last code unit before its documents and version units (rule 6: U3.2),
  after a lock or toolchain change (the warm-up gates, P2's and MG's, and V,
  whose `Cargo.lock` lists the workspace crates' new versions; X, between MG's
  warm-up and V, takes `--quick` whatever its kind), and wherever the plan names a unit structural (rule 24: a change to a
  manifest, `Cargo.lock`, a build script, the toolchain or the gate scripts; no unit
  of this release is, new model fields being no reason on their own); a
  documents-only unit runs the five documentation checkers and
  `--quick`, the last unit before MG among them (U3.3, the lane's gated head, rule 20,
  its commit written in the record, §8's line, for the next release, with no extra
  `--commit`); the full run where the plan has it, the close-out, unchanged). A unit
  is a code unit when its change reaches a crate under `--quick`'s scope rule
  (`docs/todo_v0.7.1_gate-speed-spec.md`, A.3: its own package's files, files a
  package includes, the manifests, the lock and the Cargo configuration); the
  facts file counts as code (tests parse it; the `include_str!` of
  `docs/platform-facts.md`), and, by the batch's rule 11, so does a unit that changes any file but
  comments, rustdoc prose, READMEs and Markdown (U3.2's and K7x's TOML string in
  `docs/showcase-exceptions.toml`). A documents unit stays a documents unit when its change is only
  comments, rustdoc prose, READMEs and Markdown documents, even where a crate
  includes them: it runs `--quick`, whose scope (A.3) then
  reaches those crates, so their doctests run, and it is not counted toward "every
  third" or "the release's last code unit":

  | Task | Kind | Gate |
  |---|---|---|
  | P2 | warm-up | `--commit` |
  | P0 | documents only | the five checkers and `--quick` |
  | U2.1, U2.2 | code (first and second since P2's warm-up; no unit of this release is structural, rule 24: none changes a manifest, `Cargo.lock`, a build script, the toolchain or the gate scripts) | `--quick` |
  | U2.3 | code (third) | `--commit` |
  | U2.4, U2.5 | code (first and second since U2.3's) | `--quick` |
  | U2.6 | code (third) | `--commit` |
  | U2.7 | documents only | the five checkers and `--quick` |
  | U3.0 | the trial, uncommitted | no gate (its two runs, §2.4) |
  | U3.1 | code (first since U2.6's) | `--quick` |
  | U3.2 | code (second since U2.6's; 3H.2b's TOML string among it), the release's last code unit before its documents and version units (rule 6) | `--commit` |
  | U3.3 | documents only, the last unit before MG, so the lane's pre-merge gated head (rule 20; §8), with no extra `--commit` | the five checkers and `--quick` |
  | MG | warm-up after the merge (restarts the count); every commit of its steps (1), (3), (4), (6) and (7) is covered by it: VF's, an R-MG must-fix, a path re-point, a fix the survived checks need (rule 21); the fix of a red warm-up is gated by the warm-up run again (rule 12) | `--commit`, with the GPUI Fast legs always (rule 21 (5): the merge brings the other side's code, and `<prev-tag>`'s version bump) |
  | X | the recount of stated counts (rule 17): prose, or a count inside a message string (then a code unit); rule 6's mark is U3.2, and V's `--commit` covers X, so `--quick` either way | the five checkers and `--quick` |
  | V | a lock and manifest change (rule 6: its `Cargo.lock` lists the workspace crates' new versions) | `--commit` |
  | K7x | code (a TOML string, `docs/showcase-exceptions.toml`), in the close-out | the five checkers and `--quick` |
  | a documents-only fix | documents only | the five checkers and `--quick` |
  | a code fix | as the unit whose files it touches; `--commit` where that unit's was | as that unit |

  `--quick` is what `docs/todo_v0.7.1_gate-speed-spec.md` defines (S1: at least
  formatting, the panic scan, clippy with `-D warnings` and the strict panic lints
  over the workspace, the tests and doctests of every crate the unit changes and of
  every crate that depends on one, and the documentation checks; never fewer
  checks than a task's own targeted commands); this plan relies on no more of it.
  Its first run is timed into the record. A commit between two `--commit` runs is
  proven by `--quick` only (the cost the maintainer accepted, S1); a failure the
  next `--commit` finds is fixed by a new commit, the record naming the commit that
  brought it. The documentation units are gated too: their READMEs are crate docs
  and doctests (`native-theme/src/lib.rs:179`, and each connector's `lib.rs`). K7 takes the five checkers and the orchestrator's reading of the diff;
  K8 runs the full check after it. What `--commit` skips and this work needs runs
  beside the gate, whichever it is: the GPUI Fast legs (§2). Its cross-target
  checks run in it where the gate has them; a run that skipped one, or whose
  runner-environment legs did not run (exit 75), does not count (§2).
- **Formatted and clean.** The script runs `cargo fmt --all` in place
  (`scripts/check_release.sh:39-41`, `:518`) and only reports a dirty tree, while
  CI's `cargo fmt --all --check` fails the push. So every task runs
  `cargo fmt --all --check` before it stages; its gate runs on the staged change;
  `git status --porcelain --untracked-files=no` prints the same before and after
  the gate, and nothing after the commit. The exceptions are U3.0's uncommitted
  trial (no gate); K2's uncommitted files until K7, which are the one unstaged
  exception of any fix committed between K2 and K7 (rules 11 and 15); and the
  asset run's outputs after K6, kept into K7, or, where the script did not finish
  and a fix comes first, part of that fix's unstaged exception until K6 is made
  again (§6). A reformat the gate makes outside
  a task's own files is not folded into that task's commit: it becomes a
  formatting-only commit of its own, which the orchestrator reads.
- **After an asset run** (the exception's end). Before a fix to a stamped path made
  after the asset script has run (a stop at K6; a fix to no stamped path keeps the
  outputs, rule 15 as §6 applies it), the orchestrator restores the
  script's outputs to `HEAD`, `git restore --staged --worktree --` on the paths
  the script's closing line names (`scripts/generate_assets_release.sh:297`:
  `docs/assets/PROVENANCE.toml`, `docs/COMPATIBILITY.toml`,
  `native-theme/docs/assets/`, `connectors/native-theme-*/docs/assets/`,
  `connectors/native-theme-*/README.md`), and removes the new untracked asset
  files under those folders (listed by
  `git status --porcelain --untracked-files=all -- <those folders>` first, each
  named in the record); then `git status --porcelain --untracked-files=no` prints
  nothing and the fix is made; where the close-out goes on from there is rule 15's,
  as §6 applies it.
- **One merge.** The release has one lane (§1), and its one merge is MG's (S2);
  every gate from MG's warm-up on runs on the merged tree; a gate that fails on the
  branch after a fix commit is fixed by the author of the change it points at, on
  the branch, and reviewed again (§4).
- **The documentation checkers** (no cargo): `scripts/check_platform_facts_rows.py`,
  `check_section_refs.py`, `check_preset_citations.py`,
  `check_wrong_safety_nets.py`, `check_todo_citations.py`, from the worktree's
  root, each with no finding.
- **The two release stamps** are stale from the release's first code commit on
  and are no task's done-criterion before K8.
- **The minimum supported Rust version**: CI's MSRV checks run in CI and in the
  ported script's `--commit` and `--quick` (`docs/todo_v0.7.1_gate-speed-spec.md`,
  decision 8, Part M; the maintainer's answers of 2026-11-15: O3's keyed option, so
  `--quick` skips them by key; O2 (e), so the two `+1.88.0` checks build in a fresh
  folder deleted after every run; O1a no, so the full run is unchanged). They need
  the `1.88.0` and `1.95.0` toolchains, which P2 checks. The briefs say:
  no language or library feature newer than the edited crate's own
  `rust-version`, as it stands on the branch.
- **The version (V):** in `cargo metadata`, the seven crates with
  `version.workspace = true` (`native-theme`, `native-theme-build`,
  `native-theme-derive` and the four connectors) report the release's version,
  and the three `tools/` members keep `0.0.0`; `git diff` of `Cargo.lock` changes
  only the workspace crates' versions; under D0 (b) every README's `native-theme*`
  requirement names the new `major.minor`.
- **The full `scripts/check_release.sh`**, unchanged by S1: at K4, beside K3's CI
  (S4; one failure, the provenance stamp, and the two expected warnings) and at K8
  (no failure; the expected warnings still appear, and are named in the report).
- **Long runs.** The Bash tool kills a foreground command at 10 minutes. Every run
  not known to finish within that is the orchestrator's, as a long run: the gate
  (`--commit`: 5 to 12 minutes a commit, 29 minutes cold, in the batch's earlier
  measurements; `--quick` until its time is measured, S1), the GPUI Fast legs and
  the trial's GPUI Fast run, the full check (about 23 minutes, §9), the asset
  script (30 to 37 minutes), `update_compatibility.sh run`, the release builds of
  K5, and a first build in a new worktree. A long run is started with `run_in_background`, from
  the core worktree (`cd <core worktree> && …`), through the wrappers (or the
  capture form of §5, *Cargo*), its log and a `STATUS` file at absolute paths in a
  new subfolder of `<record>` for that run (§5, *Run folders*)
  (`… > <log> 2>&1; echo $? > <STATUS>`); the orchestrator waits for the exit
  notice, then reads `STATUS` and the log's tail. Every long run of the orchestrator's
  that runs cargo takes the shared gate lock, so that no two run at once across both
  lanes (rule 2): every `check_release.sh` mode (the units' gates, the warm-up gates,
  K4, K8); every other command run as part of a gate (the GPUI Fast legs, §2, and the
  CI legs beside U2.2's and U3.1's gates); MG's regenerations; the trial's GPUI Fast
  run (U3.0); a killed targeted command's rerun (below); the compatibility run (K2);
  K5's release builds; and the asset script (K6, started as rule 24 says). Each starts
  as `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh
  <command>`. Only an agent's targeted `-p` commands take `slot.sh` and `lowprio.sh`
  alone. A gate run starts as:

  ```bash
  cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh <--quick | --commit | nothing for the full run> > <run folder>/LOG 2>&1; echo $? > <run folder>/STATUS
  ```
 A wait on a condition inside a
  long run (K6's "Theme-switching GIFs generated") uses the Monitor tool with an
  until-loop on a file condition, never a `pgrep -f` loop, which matches itself.
  The first `--commit` run (P2's warm-up gate) and the first `--quick` run are
  timed and their times written into the record, the first with whether it ran
  cold (a new worktree, an empty build folder) or warm (the reused worktree's build
  folder).
- **The warm-up gate.** An agent's targeted commands build the feature sets the
  gate's test legs build (gpui `native,watch`, `scripts/check_release.sh:667`;
  iced `iced_aw,watch,native`, `:739`; egui `--all-features`, `:762`), so before
  an agent that runs cargo is dispatched the orchestrator runs
  `scripts/check_release.sh --commit` on the clean tree, under the gate lock (*Long
  runs*), as a long run in its own
  folder (`<record>/gates/warmup-<n>/`), wherever the worktree is new, `Cargo.lock`
  changed since that folder's last gate, or the rustup hook reported a changed
  toolchain; then those builds are warm and an agent's commands stay within the
  Bash tool's limit. The release starts with one in any case, P2's, before its
  first agent that runs cargo (A2 runs none), and MG runs one after the merge
  (S2); V's `--commit` follows its lock change. Where `--quick` builds fewer
  feature sets than an agent's next targeted commands, the first such command may
  be a cold build: one the Bash tool kills is the case below. A red warm-up gate,
  at P2 or after MG, follows rule 12: it goes, with the log's failing part, to the
  author of the unit whose rule the failing test checks, or, where that author is no
  longer at hand (at P2, where none of this work is yet, a unit of the lane
  predecessor's release), to a new `implement` agent with the log and that unit's
  part of its specification; one a conflict resolution caused is the
  orchestrator's, one whose cause is unclear is a stop (§6); the fix is gated by the
  warm-up run again, and no agent that runs cargo is dispatched before the warm-up
  passes.
- **A targeted command killed at 10 minutes** is reported by the agent with the
  command; the orchestrator runs it as a long run under the gate lock (*Long runs*;
  its own folder under `<record>/gates/<task>/`), then continues the same agent with the `STATUS` and
  the log's tail.
- **CI** runs only once a pull request is open. Pushes, each with an explicit
  refspec: K3, K7, K7x (2K: both steps' units pushed together at the release's
  end, after MG, S2, and after the compatibility run, S4); nothing is pushed
  before them. The orchestrator watches each run in the background. A red run
  emails the maintainer; no push is made that is expected to be red. If one is
  red, the orchestrator tells the maintainer what failed and why, and the failing
  change's author fixes it (reviewed as in §4) as the lane's next commit, under
  rule 15 as §6 applies it.

## 4. Reviews

Every commit that changes behaviour is reviewed before the push that rests on it
(K3) and before the documents unit of its step, whose texts state what it does
(U2.7, U3.3); each step keeps its own reviews (S5). The reviewer gets the commit range and the specification's
header and parts, never the author's report; it reads the commits (`git show`),
not a working tree, and runs no cargo; R3.0 reads `<record>/trial/trial-<n>.patch` and
that pair of runs' outputs, each pair's in turn. A review runs while the lane's next task works, but R3.0,
which the verdict waits for. A must-fix is a new change by the task's author (the
same agent, continued), staged after the commit of the task then running in the
lane and before the lane's next task starts, with the task's done-criterion run
again as it applies to a fix, gated and committed by the orchestrator (§2), and
reviewed by the same reviewer. For a fix, the done-criterion is: the tests the
task names pass (each `ok` quoted), the counts, the pinned-line checks against the
fix's own parent (`git diff HEAD -U0`), the moves the fix makes,
`cargo fmt --all --check`, the staged-tree check and the gate; not "seen to fail",
which a fix of a passing test cannot repeat; a new test the fix adds is seen to
fail as any new test. A second must-fix on the same point falls under §6's
take-over rule. Every reviewer, the orchestrator included, also checks that no new
code transcribes an LGPL or GPL source (§5, *The licence rule*).

"By size": a commit of up to a few hundred changed lines an Opus agent wrote is
reviewed by the orchestrator, a larger one by a `review` agent; the orchestrator
reads `git diff --stat` and says which it chose.

| Review | Commit | Reviewer | Why |
|---|---|---|---|
| R2.1h | U2.1's validation hunks: `native-theme/src/resolve/validate_helpers.rs`, `resolve/validate.rs`, `model/resolved.rs`, `model/defaults.rs` | `review-hard`, narrow | the maintainer's delegation table names validation: a new group of `validate_defaults!` that every resolved theme passes through, a field that may stay `None` where every other is required. Small, so the Fable review is short; it runs first, and R2.1 follows it |
| R2.1 | the rest of U2.1 | by size (expected `review`) | the facts row, rules, registry and presets against 2B and the rationale's values; the moved citations; egui's row and counts; the tests that name every field |
| R2.2 | U2.2 | by size (expected the orchestrator) | one helper, one rebinding, the portal path's function and the lists that hold the reader to the preset |
| R2.3 | U2.3 | `review` | `ring_shows`' state across renders and its borrow order in three widgets, the computed `ring` row, the predicate both switches share, the four claims branched, the pinned lines |
| R2.4 | U2.4 | `review` | the wrapper's event classes before its early return, the state kept from focus to blur, the chained operation in the tests, the showcase's Tab handler and its menus rule |
| R2.5 | U2.5 | `review` | `FocusOrigin`'s rule against decision 14's sequences, the per-viewport map's lifetime against `forget_closed_viewports`, the tests that must fail without the rule |
| R2.6 | U2.6 | by size (expected the orchestrator) | route texts and an element list the showcases' tests hold |
| R3.0 | U3.0's uncommitted patch and the two runs' outputs, before the verdict | `review` | the trial decides what the release ships for gpui, and where it fails its test is reverted unseen: the reviewer checks that the test draws both buttons as 3F.1 and 3F.2 say, reads each button's own quad, reaches every state of 3F.2's table on all six combinations, compares at 8 bits exactly, and allows only the hover and press border to differ |
| R3.1 | U3.1: the model, the facts rows, registry, rules, kde-breeze, the reader and its lists; egui's strokes, roles predicate, rows, T10 formulas and tests | `review` | the base's and «button»'s strokes and the seven kept roles against 3E.1's table, the mapping rows against T2's and T10c's rules, the four changed egui tests; too large for the orchestrator |
| R3.1c | U3.1: iced's styles, row, `UNREAD_COLOURS` entry and leaves file; gpui's variant, contract entries and, where the trial passed, the showcase's `Bordered` kind | `review`, beside R3.1 | each status of 3D.1's table; the leaves file against the walk; `bordered_button`'s three refinements per state; the claims at the lines that read their fields; the tooltip's parameter change and its two callers |
| R3.2 | U3.2 | by size (expected the orchestrator) | route texts and element lists |
| R-MG | MG's step (4), where a file was resolved by the merge or written by step (7), except Markdown that no test parses (a README and `connectors/native-theme-egui/src/mapping.md` are in), where the merge resolved `Cargo.lock`, or where VF committed anything: `git show --remerge-diff <merge>` with the record's list of resolved hunks, and every commit of steps (3), (6) and (7) (VF's review is always within R-MG, rule 21 (4)) | `review`; `review-hard` where those commits change a hard part of a core (a resolver, validation, a runtime, concurrency, `unsafe`) | a resolution joins two releases' code by hand, and VF's commits rewrite toolkit forms on the merged tree; the reviewer checks each resolved hunk against both sides' specifications and each rewritten form against the new toolkit version's source (the cited line holds the item the text names), before MG's warm-up gate runs; a must-fix is a new commit by the author of the commit it fixes (the orchestrator for a resolution or `Cargo.lock`, VF's agent, the `worker` of a path re-point), made before the warm-up gate, whose `--commit` covers it (rule 21), and reviewed by the same reviewer |
| — | P0, U2.7, U3.3's documents, MG's resolutions in documents where R-MG does not run, MG's path re-points, X, V, K7, K7x, a formatting-only commit | the checkers, then the orchestrator reads the diff | data and texts the specification fixed; the extraction scripts compare them |

At most one Fable agent runs at a time, across both lanes of the batch (S3): R2.1h,
a `design` agent and an `implement-hard` agent never overlap with each other or
with the other lane's.

## 5. Rules every agent brief carries

- **Inputs.** The brief gives the header and the task's parts; the parts are the
  whole task: nothing beyond them, no refactoring, no formatting churn. A needed
  change the parts do not name is reported, not made. Every edit is placed by its
  anchor (the quoted text, the key, the test's name), a count is the tree's value
  plus the specification's delta, and a citation is found by the text it cites,
  never by the number the specification gives; where a quoted text does not
  stand, the agent stops that edit and reports it, and §6's rule for a quoted
  text decides. The lines the gpui showcase's colour claims pin do not move
  (*Lines that must not move*).
- **The worktree.** The brief names the core worktree by its absolute path; every
  command starts with `cd` into it or uses `git -C` and `--manifest-path`. The
  main checkout (another branch, `v0.8.0-rc1`'s work) and every other worktree
  have the same relative paths: nothing is read, run or written there, the parked
  branch's files are read with `git show 78ee0efd:<path>` from the core worktree
  where a part needs them, and a check that passes elsewhere proves nothing. The
  report quotes `git -C <worktree> log -1 --oneline`.
- **The licence rule (the maintainer's D10, not the rationale's):** no code from
  an LGPL or GPL source. Breeze (`breezestyle.cpp`, `breezehelper.cpp`,
  `breezemetrics.h`, its colour schemes), KColorScheme, xdg-desktop-portal-kde
  (`settings.cpp`), Qt, GTK (`gtkwindow.c`, `gtkcssstylepropertyimpl.c`) and
  gsettings-desktop-schemas are read for facts only. A function such a source is
  cited for (`focus_ring_of`, `apply_kde_accent`, the reader's two button borders)
  is written only through the clean room: every implementer brief (U2.1 to U2.6,
  U3.0, U3.1, U3.2 and any fix of their code) says that the implementer opens none
  of those sources; the specification's description and its tests are the clean
  room's description and vectors. gpui-component, gpui-base, gpui-pre and GPUI
  Fast are Apache-2.0, egui, egui-winit and iced MIT or Apache-2.0, androidx
  Apache-2.0 and microsoft-ui-xaml MIT: read for facts; a port of their code
  would keep a file-level notice (none is planned).
- **Hooks.** Source files are edited with the Edit and Write tools only, with one
  exception: the generated `mapping.md` and `contract-leaves.toml`, written by
  their commands. The
  project's hooks block invented values and panic-prone code; a blocked edit is
  obeyed, never bypassed, and blocked content is never written another way
  (shell, `sed`, a here-document, a script). A block on content the specification
  requires stops the task (§6).
- **Code.** No `unwrap`, `expect`, `panic!`, panicking index or unchecked
  arithmetic in non-test code; no `unsafe`; no public item removed; no invented
  value. Code, tests and rustdoc that name this release's own planning documents
  (this specification or this plan) name their archived path,
  `docs/archive/todo_v0.7.x_kde-open-items-steps2-3-{spec,plan}.md`, since K7 moves
  them after the asset run (rule 22 (c); every `implement` brief says so).
- **Cargo.** Every cargo command, and every script that runs cargo, goes through
  `<tools>/slot.sh <tools>/lowprio.sh <command>`, an agent's in the foreground
  (its targeted commands, §3), the orchestrator's long runs in the background
  (§3); an environment variable goes before the wrappers
  (`VAR=… <tools>/slot.sh <tools>/lowprio.sh cargo …`), since `lowprio.sh` ends in
  `exec "$@"`. The capture sessions (the asset runs) run under
  `kde-inhibit --power --screenSaver --notifications` and `slot.sh` but not
  `lowprio.sh`, their builds first through both wrappers; the slot is taken by
  the outer command, `flock <locks>/assets.lock flock <locks>/gate.lock <tools>/slot.sh sh -c …` (K6,
  rule 24), and `run.sh` holds no `slot.sh`; since `lowprio.sh` is what sets the
  build and test thread caps, the command line inside `run.sh` begins with
  `CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16`, so that the cargo it starts keeps
  the gate's limit. A run on a private X
  server (the rehearsal) uses `slot.sh` and no `kde-inhibit`. In every command a
  placeholder (`<tools>`, `<record>`, `<core worktree>`) is written as its literal
  absolute path, never with `$HOME` or `~`; no command is wrapped across lines,
  and no command uses `\|` as an alternation (an alternation is written with `-e`
  or `-E`; a literal `|` inside an `-E` pattern is `\|`, as in MG's marker check). A run
  longer than 10 minutes is a long run (§3). `build-dir`, `CARGO_TARGET_DIR` and
  `--target-dir` are never set; `cargo clean` is never run. At most two cargo
  processes at once in total, across both lanes of the batch (S3), which the two
  slots of the shared `<locks>` enforce; in this plan only the core lane builds,
  and a command of this lane waits in `slot.sh` while the other lane holds a slot.
- **Reach.** No `find /` and no other search of the whole file system. Reads and
  writes stay inside the core worktree, Cargo's home and build folder (the
  registry for upstream sources), the session's scratch folder for its own
  temporary files, `<record>`, `<locks>` (written by `slot.sh` alone) and
  `<scratch-keep>/batch-2026-11-15/` (read only). Nothing appears on the
  maintainer's desktop but the orchestrator's capture sessions.
- **Work.** A new test is written first and seen to fail (§2). A changelog entry
  is one physical line. The agent runs its targeted commands in the foreground,
  passing the maximum timeout (600000 ms) for a long one (rule 1), each expected to
  finish well within 10 minutes, and never `--commit`, the full
  check, an asset run or a GPUI Fast leg (§3); it stages its own paths by name,
  does not commit or push, adds no attribution line, and reports once: the staged
  files (`git diff --cached --stat`), its commit message, each done-criterion item
  it ran with its command and result line, and everything it was unsure of
  (U3.0's agent reports the trial, then U3.1 at its end). The orchestrator runs
  the gate and commits with that message; a red gate comes back to the agent,
  continued, with the log's failing part.
- **Something the maintainer must hear before a commit** (a value that differs
  from the specification's, a quoted text whose present form changes the edit's
  meaning, a mechanism that cannot be built, the trial's result): the agent says so
  in its report, and the orchestrator commits nothing of it before §6's stop rule
  lets the work go on.

**`<scratch-keep>`** is `/home/tibi/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/`.
**`<record>`** is `<scratch-keep>/todo_v0.7.x_kde-open-items-steps2-3-plan/`:
`record.md`, `tools/` (`<tools>`), `docs/`, `inputs/`, `anchors-2/`, `u2.1/`,
`u2.7/`, `anchors-3/`, `trial/`, `u3.1/`, `u3.3/`, `assets/`, `ci/`, `gates/`. **`<locks>`** is `<scratch-keep>/slot-locks/`. **`<cargo-build>`**
is Cargo's build folder root. The briefs give absolute paths; no committed file
names them. **The inputs** copied by P1 into `<record>/inputs/` from
`<scratch-keep>/batch-2026-11-15/` (a session's scratchpad does not outlive it):
`steps23fix2/cite2.py` and `steps23fix3/prose.py` (the specification's
transcriptions of the gpui showcase's citation parsers), `steps23fix3/soft.py`
(3H.2a's count) and `gates/rehearsal.sh` (K5). `cite2.py` names
`.claude/worktrees/v071` as its root and `rehearsal.sh` as its `W=`; each is used
only through a copy re-rooted to the core worktree (`cite2-core.py` by P2, the
rehearsal's by K5), so that no input reads another worktree.

**The record.** `<record>/record.md` holds the dated results, shas, counts,
verdicts and answers as the work goes; only the orchestrator writes it, agents
report; it holds each step's part apart (S5: each step keeps its record). It is
copied into this plan's §8 only in the close-out commit (K7), so that no tracked file is edited mid-work and
the clean-tree checks hold; the other edits of this plan before K7 are U3.3's
status line (the trial's verdict, 3I), MG's path re-points (its step (6)) and what
MG's merge itself writes into it (a conflict resolved, or the other side's own
rewrite of this plan's paths), and, on the no-MG path, the path re-points P0 or P2
makes (P0's row).

**Run folders.** Every long run that can be repeated writes into a new subfolder
of its own, never into a folder an earlier run used: the gates into
`<record>/gates/<task>/run-<n>/` (with `LOG` and `STATUS` for `--commit`, and
`FAST-TESTS.log`, `FAST-TESTS.STATUS`, `FAST-SHOWCASE.log` and
`FAST-SHOWCASE.STATUS` for the two GPUI Fast legs where the gate has them, and
`warmup-<n>/` for §3's warm-up gate), the feature legs beside a gate into
`<record>/gates/<task>/legs-<feature>-<n>/` (U2.2's `kde`, `portal` and `linux`,
U3.1's `kde`, each with `LOG` and `STATUS`), a killed targeted command's rerun
into `<record>/gates/<task>/cmd-<n>/`, the trial's GPUI Fast run into
`<record>/trial/fast-<n>/`, and each kind of the close-out's runs into a counter
of its own under `<record>/assets/`: `full-<n>/`, `compat-<n>/`,
`build-<kind>-<n>/`, `rehearsal-<n>/`, the asset runs `run-<n>/`; each `<n>`
counts from 1, the folder is made with `mkdir` (which fails if it exists), and the
record names each folder with its run. So a `STATUS` file left by an earlier run
never ends a Monitor wait early and is never read as the new run's result.

## 6. Limits and stop rules

- **The machine's limits are shared with the batch's second lane** (the widget
  icons alone, in a worktree of their own, S3 as amended): at most two cargo
  processes for both lanes together (one gate, and one `nice 19` targeted build in
  the other lane), the gates serialized by the shared `<locks>/gate.lock` (rule 2),
  at most four agents for both lanes together (six where two run no cargo), one at
  a time across both lanes for each of the asset run, the capture window and the
  private-X rehearsal (`rehearsal.sh` on Xvfb `:97`) (the batch's rule 24: each
  held by a shared lock in `<locks>`, `xvfb97.lock` around K5's rehearsal and
  `assets.lock` around K6's asset run and its capture window, taken with a cargo
  slot before the maintainer is asked, so a lane waits mechanically), and never two
  Fable agents. The orchestrator counts the other lane's agents
  and runs before it dispatches one of this lane's.
- **Agents at once in this lane**, within those shared limits, of which only the
  lane's agent runs cargo. The peaks: beside P2's warm-up gate or P0, A2, a
  `worker` without cargo (the lane's task is the orchestrator's); beside U2.2,
  R2.1h, then R2.1 (one reviewer of a commit at a time); beside U2.3 to U2.6, the
  previous unit's reviewer; beside U3.2, R3.1 and R3.1c together: three. At most
  one Fable agent at a time, across both lanes (§4).
- **A gate failed twice** by the same task is not dispatched a third time: the
  orchestrator takes the task over, re-specifies it, or sends it once as
  `implement-hard`, and says which. For a task whose code a licensed source is
  cited for (U2.2, U3.1's reader part), the take-over is always a fresh
  `implement-hard` agent, which has read none of those sources, never the
  orchestrator (§5, *The licence rule*).
- **The specification's stops**, each with nothing committed by the task that met
  it, nothing pushed after it until the maintainer has been told and, where the
  rule that names the stop asks the maintainer, has answered: a quoted text that
  does not stand (A2, A3, any task; settled as *The batch's stop rule* below
  says); a predecessor's gated head not gated or with a review open (rule 20), step 1
  not below the branch, a toolkit version that differs from the specification's by
  more than a patch version, a missing target, linker or toolchain, v0.7.1's S6 unit
  not below the branch, an exit 75 at `--prepare`, or the hook trial applied (P2); an untracked planning document that
  differs from the base's copy (P2) or, still untracked, from `<record>/docs/`
  (P0, rule 26; a committed one is not compared); a red warm-up gate whose cause is
  unclear, at P2 or after MG (rule 12); at MG (and at P2 on the tag path),
  `<prev-tag>` missing on `origin` (`git ls-remote origin 'refs/tags/<prev-tag>'`
  printing no line) or naming another sha there than `git rev-parse <prev-tag>`;
  at MG, `<lane-pred-head>` not an ancestor of `<prev-tag>`, an untracked file the merge would overwrite that differs
  from the tracked one, a merge conflict whose resolution would change what a text
  means, a `Cargo.lock` re-applied by item 5 whose diff against `<prev-tag>`'s
  lock shows other packages than this release's, a marker the marker check
  prints, a staged path outside
  `git diff --name-only --no-renames <lane-pred-head> <prev-tag>` that is no
  regenerated file the record names (the staged-path check), a toolkit version outside P2's accepted set, a red warm-up after the merge
  whose cause is unclear, or a path of the two documents gone otherwise than to
  `docs/archive/` (MG); the trial passing on one backend and
  failing on the other (§2.4); a `dump=no` in the rehearsal's second run (K5); a failure or
  warning of K4's full check other than its expected ones (K4); a capture that
  differs from 2H.5's or 3G.2's expectation (K6); an element a capture or the
  rehearsal shows pushed out of view, where its placement is a design choice the
  maintainer made (rule 19).
- **A parity difference** the asset run's comparator shows after the capture (K6's
  parity step) is a fault, never a question (rule 19; a difference K5's rehearsal
  shows before the capture is reported while the capture goes ahead): the showcase's or connector's author fixes it, a close-out fix
  under rule 15, and the run is made again; or, where it is a toolkit's limit, it is
  excepted with its reason and source, an exception entry that is reviewed (§4). The
  maintainer is told either way. Only an element pushed out of view, where its
  placement is the maintainer's design choice, is a stop (above).
- **The batch's stop rule.** A stop ends when the maintainer has answered where
  the rule asks; otherwise the work continues once the maintainer has been told.
  A quoted text that no longer stands is found in its present form by the
  orchestrator: where the edit's meaning is unchanged, the edit is made at the
  found text and the record names it; where it is not, the maintainer is asked.
- **The fallback (2J)** is no stop: it is the rationale's decision, taken on
  §2.2's evidence, and the maintainer is told.
- **A task that needs more than its parts name** (a model field, another crate, a
  decision the rationale did not take) stops, and the maintainer is asked. So does
  an upstream move during the work (a new gpui-component, GPUI Fast, egui or iced
  release, a new advisory): it is reported with what it costs before anything is
  changed for it.
- **A fix after the units** that a review, CI, the full check, the rehearsal or
  the asset run demands is made by an `implement` agent (a documentation fix by a
  `worker`), reviewed as in §4, and passes the done-criterion of the task whose
  files it touches; a dated line that names the unit it changes is re-pointed to
  the commit that now holds what it states. **The one close-out fix rule:** a fix
  the close-out needs (K1 to K8: an audit finding, the compatibility run, a red CI
  run, the full check, the rehearsal, the asset run, K7's documents) is gated as the
  task it fixes (§3's table), and goes on as rule 15 says. In this plan's steps:
  the close-out's first step is K1 (`cargo audit`), the compatibility run K2, its
  push K3, the full check beside CI K4, the asset run K6 (finished or not as rule 15
  defines it, by its `STATUS`), the asset commit K7 and the final full check K8; a
  stamped-path fix restores the asset outputs first (§3, *After an asset run*);
  K2's uncommitted files are `docs/COMPATIBILITY.toml` and the connector READMEs; the
  partial outputs of an unfinished asset run are the changed files and new untracked
  files under the asset folders; each unstaged exception is named in the record. A fix the lane predecessor or `<prev-tag>` gets reaches this branch
  through MG (S2), not by a commit of this plan; one it gets after MG is the next
  release's. The maintainer is told what stopped it, what was changed, and whether
  another capture window is needed.
- **The desktop** is asked for when everything before it holds: K6's one capture
  window, and a repeated capture window after a fix.

## 7. The maintainer's part

**With the approval of this plan, before any task:**

1. **D0: the version** (open point 1). `0.7.N`, the one release of both steps
   (S5), and which release comes immediately before it in the order (under D10's
   proposal egui 0.37), whose tag `<prev-tag>` MG merges; P2 starts from the
   gated head of v0.7.1, the release before it in the KDE lane (S2; D10 (a), step 1
   in v0.7.1, the maintainer's answer of 2026-11-15). Under D0 (b), a minor version: the plan
   holds with that number and the branch name the maintainer gives (the next minor
   would be 0.8.0, the name of the parked rebuild, whose branch `v0.8.0-rc1` is on
   `origin`), the changelog heading of 2I.1 as `## [0.N.0]`, and V's README
   requirements. Under D0 (c): the release that makes the model's input structs
   `#[non_exhaustive]` comes first, specified apart; this plan waits for it, the
   *Breaking Changes* entry goes, and 2A.4's and 3A.2's `proptest_roundtrip.rs`
   literals become builds from `Default` (2K, *Under (c)*), which amends the
   specification and this plan before P2.
2. **The desktop**: one capture window for the release (K6, 4 to 6 minutes), and
   one more for each rerun §6 names; asked for when everything before it holds.
3. **The pull request**: settled (below, 2026-11-15): the orchestrator opens a draft
   pull request from `v0.7.N-rc1` to `main` at the branch's first push (K3), its
   title and description shown to the maintainer. Without one, the push checks
   nothing (rule 9), so K3 opens it before CI is read.
4. **What this plan adds to the specification and the batch's rules**, put for
   approval with this plan (the maintainer has not approved these): A2 and A3;
   the trial's GPUI Fast run and the stop where the two backends disagree; R3.0
   before the verdict; the evidence rule for a fallback (§2.2); U2.7's stricter reading of rule 16 (a
   failed `gh issue view` read holds U2.7 back until a read succeeds, where rule 16
   only holds the reply's edits); the CI legs U2.2 and U3.1 run beside their gates
   (`cargo test -p native-theme` with `--features kde`, `portal` and `linux`, and
   `--features kde`, cold builds no gate leg makes); the parity rehearsal (K5);
   the push of K7; the GPUI Fast legs beside `--quick` gates; P2's check that v0.7.1's S6
   unit is on the tree (its `git grep`); K5's rule for a `dump=no` (run once more, then
   a stop); MG step (6)'s exemption of the close-out's archive paths; the reading of
   rule 6 for X (rule 6's mark is U3.2, and V's `--commit` covers X, which takes
   `--quick`); and one departure
   from rule 11's gpui-unit test: X runs the GPUI Fast legs wherever its commit
   touches a gpui connector or showcase file, a broader trigger than the test
   (which counts only code or strings under `connectors/native-theme-gpui/`), and
   so the safe side. The rest is the specification's (2K, 3I: the plan's status
   line in U3.3, the merge and its steps, X, the exceptions K7x reads, the
   rationale's sentence, the cadence) or the batch's rules' (MG's order and checks,
   VF, the path checks, the gate table's marks).
5. **The copies**: at the approval the orchestrator copies the rationale, the
   specification and this plan, as approved, into `<record>/docs/`: P0's source,
   where P0 runs, should the untracked copies in `.claude/worktrees/v071/docs/` be
   lost, and what P2 compares an untracked copy with (§6).
6. **The build folder of `kde-b`** (§1): where P2 makes `kde-b`, its build folder
   (tens of GB, under `/run/media/tibi/work/cargo-build/`) is named when P2 makes
   it; it stays after this release for the caret, the KDE lane's next release in
   `kde-b`, which reuses it (S3 as amended), and the orchestrator deletes it with
   its worktree when the lane's last release of the batch, steps 6 and 7, closes. The
   maintainer approved this extra build folder of the KDE build lane on 2026-11-15
   (settled, below), a departure from the rule that "a separate build folder is
   deleted by the agent that made it before its task ends": the folder keeps the
   lane's builds warm across its releases. The two `+1.88.0` MSRV checks build in a
   fresh folder deleted after every run (O2 (e), 2026-11-15), not in this one.
7. **Approved 2026-11-15** (settled, below): S2's amendment, the batch's rules 1 to
   26 and the three re-readings they name. What remains for approval with this plan
   is item 4 (this plan's own additions and departures), with D0 (item 1).

**Settled by the decisions of 2026-11-15** (`docs/todo_v0.7.1_batch-speed-rationale.md`)
and no longer asked:
- S1 to S5, S3's amendment and S6, the maintainer's decisions, as this plan applies
  them;
- approved by the maintainer on 2026-11-15:
  - S2's amendment: a merge, not a rebase (task MG);
  - the batch's working rules, rules 1 to 26 (the Appendix of
    `docs/todo_v0.7.1_batch-speed-rationale.md`), which this plan applies throughout;
  - the three re-readings the rules name: rule 20's of S2's branch point
    (`<lane-pred-head>` is the predecessor's documents unit, with its `--quick`; here
    U3.3, U3.2 holding the last `--commit`); rule 7's branching from `<prev-tag>` where
    that tag exists when P2 runs (no MG); rule 9's "green" (CI green, and the full
    check's only failure the provenance stamp, K6);
  - the extra build folder of the KDE build lane (`kde-b`, item 6);
  - opening a draft pull request at a branch's first push (item 3);
- D10 (a): KDE step 1 ships in v0.7.1 (the maintainer's answer of 2026-11-15);
- the gate-speed answers: O1a and O1b no (the full run unchanged), O2 (e), O3's keyed
  option, decision 4 (y) (§3, *The minimum supported Rust version*);

and, of S1 to S5 as this plan applies them: one
release for both steps and its single close-out (S5);
the base (S2); the gate cadence (S1); the second lane (S3); the full check
beside CI (S4); the caret in the KDE lane, which reuses `kde-b` (S3's amendment); no
line citation re-pointed, so no sweep unit and MG resolving a citation as text (S6);
the reply, which waits for the release of steps 4 and 5 (S5): this
plan reads whether it was posted (`gh issue view 4 --comments`, before U2.7 and
before U3.3) and does not ask.

**During the work, the maintainer is told or asked:** a stop of §6; a mechanism
that fell back (2J); the trial's verdict, and where it failed the state that
differed and request 5's draft; MG's conflicts and a toolkit version it brings; the reply's posted state where the read fails or finds it posted; a red CI run; "capture now" (K6)
and what the new screenshots show; the parity exceptions K7x removes; an
untracked document that names the moved files (K7).

**After K8:** whether to merge, tag and publish, and the pull request's title and
description; whether the upstream requests are sent; the reply after the release
of steps 4 and 5.

## 8. Record

Copied from `<record>/record.md` at K7, each step's part apart (S5).

- P1: `<locks>` made, the trial's output `<…>`; the commit that added each of this
  work's documents `<sha | still untracked>` (rule 26).
- The release: the base `<lane-pred-head>` and the lane predecessor's record lines that it
  is gated `<…>`; the worktree (reused or new) and its build folder
  `<cargo-build>/<hash>`; the toolkit versions `<…>`; the maintainer's answers
  (§7) `<…>`; the hook's refusal `<…>`; P0 `<none | its commit>`; the first
  `--commit` run (P2's warm-up gate) and its time `<…>` (`<cold | warm>`); the
  first `--quick` run and its time `<…>`; each gate's task, kind, run folder and
  result, and any failure a later `--commit` found with the commit that brought it
  `<…>`; **the lane's gated head for the next release: `<sha>` (U3.3; `--quick`
  run `<folder>`; reviews closed)**, written when U3.3 is committed and its diff
  reading (§4) is closed, naming U3.3's commit (a fix after a red gate included,
  rule 20), for the steps
  4 and 5 plan's `<lane-pred-head>` (that release never waits for this one's MG, X
  or tag); VF `<none needed | each form changed, its commits>`; MG `<the tag check (local and origin shas); the pre-merge ancestry check; the merge commit <merge> and its parents; <prev-tag>; the
  HEAD recorded before the merge; the conflicts and their resolutions; `Cargo.lock`'s re-applied changes; the marker and staged-path checks; a pending generated file and the commit that regenerated it;
  R-MG's result; the versions before and after and the re-reads; the path check
  and its re-points; the survived checks; the either-side list>`; the reply's
  posted state as read before U2.7 and U3.3 `<…>`; K7's path rewrites and the
  remaining mentions `<…>`.
- Step 2: A2 `<all found | the texts not found>`, the counts' bases `<…>`; the
  fallbacks `<none | the mechanism and the reason>`; the units' commits
  `<sha …>`; the reviews and their must-fixes `<…>`.
- Step 3: A3 `<…>`; the trial: both result
  lines `<…>`, R3.0 `<…>`, the verdict
  `<passed | failed: the combination, the state, the item>`; the reviews `<…>`.
- X: each stated count recounted, old and new `<…>`.
- The close-out: K4's failure and warnings `<…>`; K3's CI run `<id>`; K5's
  rehearsal `<…>`; K6's `STATUS`, the log's tail, the unmatched exceptions and what
  the screenshots showed `<…>`; K7's list of remaining mentions and of untracked
  documents `<…>`.
- Durations measured `<…>`. Departures from this plan `<…>`.

## 9. Time

Measured at the 0.7.1 release on this machine and at GitHub (the archived plan of
issue #4's KDE fixes, §8): the full `check_release.sh` about 23 minutes; the asset
script 30 to 37 minutes, of which 4 to 6 need the desktop; a CI run 21 to 27
minutes. A `--commit` run: the batch's earlier figures, 5 to 12 minutes a commit
and 29 minutes cold (§3), measured before the 16-job cap; today's is not measured
until the first timed run (P2's warm-up gate), which the record holds. A `--quick`
run is not measured at all until its first timed run (S1). Not measured: the first
build in a new worktree, the trial's GPUI Fast run, and every task's duration; the
first runs give them, and the record holds them. The longest chain is the release's
lane: P2, P0 where there is one, U2.1 to U2.7, A3, U3.0 with R3.0, U3.1, U3.2,
U3.3, MG with VF inside it (and any wait for `<prev-tag>`), X, V, K1 to K8; plus any review fix, a
fallback's `implement-hard` attempt, waits on the second lane's use of the shared
slots and asset run (S3), and the waits for the maintainer's answers and the
capture window.
