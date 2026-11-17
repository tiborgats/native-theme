# 0.7.x: issue #4's open KDE items, steps 4 and 5 (the hover outlines of the check box, the radio and the combo box; a toolbar button's states) — implementation plan

- Rationale: [`issues/0004_kde-open-items-rationale.md`](issues/0004_kde-open-items-rationale.md)
  (D6 for step 4, D7's first half for step 5, D0 for the version, D10 for what a
  step carries outside the code and the order of the releases).
  Specification: [`todo_v0.7.x_kde-open-items-steps4-5-spec.md`](todo_v0.7.x_kde-open-items-steps4-5-spec.md)
  (Part A for step 4, Part B for step 5, Part C for the units, their order, gates
  and version; checked, final).
- Written: 2026-11-15, against `v0.7.1-rc2` at `0fa28027`, the specification's
  commit. Every `file:line` here is `0fa28027`'s unless it names another version
  or tree (these date their own lines, and no citation is re-pointed but one a
  gate reads: the batch's S6). Status: **not approved**. Nothing below starts before the maintainer
  approves the rationale, the specification and this plan, and answers §8's
  points.
- The release: **one 0.7.x release, `0.7.N`, for both steps** (the batch's speed
  decision S5, `docs/todo_v0.7.1_batch-speed-rationale.md`, approved by the
  maintainer on 2026-11-15; the rationale's D10: "The maintainer may merge or
  reorder steps"), in item 2 of *The releases after v0.7.1*: one branch, one
  version, one CHANGELOG section, one ROADMAP section, one close-out, one asset
  run and one archive at the end. Each step keeps its own units, its trial (step
  3's outcome read; step 5's icon trial), its measurement (step 5's), its reviews
  and its part of the record. Under D0 (a) the release names its break once; the
  reply to the reporter waits for this release (S5). The place in the order is
  settled (approved with S3, 2026-11-15) and the number is the maintainer's (D0):
  `0.7.N` follows the release tagged `<prev-tag>`, steps 2 and 3's, which is also
  the release before it in its own lane, the KDE lane. Where this plan writes `0.7.N`, the
  specification's texts have `<version>`, "<the release that brings steps 4 and
  5>" and the headings of D0 (a)'s
  proposal (C.3), and the version the maintainer
  gives is written (C.3 proposes 0.7.4 under D10's order with S5's pairs, a
  placeholder until the maintainer decides D0); A.12
  and B.11 say how the two steps' texts become one section (S5).
  Steps 2 and 3 come before this release in the KDE lane; the caret's blink comes
  after it (its lane successor, which branches from this release's gated head and
  reads this plan's record, S3 as amended on 2026-11-15), and steps 6 and 7 after
  the caret.
- Where: branch `v0.7.N-rc1`, made from `<lane-pred-head>`, the gated head of the
  release before this one in its own lane (the KDE lane: v0.7.1 → steps 2 and 3 →
  steps 4 and 5 → the caret → steps 6 and 7; here steps 2 and 3's, its gated head,
  its documents unit, past its gate (`--quick`), its reviews closed: the batch's
  rules 18 and 20), not from its tag; and, once after this release's last unit
  before the merge (U5c, rule 17), merged
  with `<prev-tag>`, the tag of the release immediately before this one in the
  maintainer's release order, which may be another lane's:
  rule 17's command, `git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>`, committed
  by MG once its step (1) is done (S2 as amended on 2026-11-15, rule 17; MG, §2.4).
  A merge keeps every sha, so no record, measurement or commit message needs
  re-pointing. This plan merges no pull request and tags and publishes nothing;
  it opens a draft pull request at the branch's first push, as the maintainer
  approved on 2026-11-15 (§8 item 3).
- **The open choices** (rule 18; §8 item 8). D10 (a) is decided: KDE step 1 ships
  in v0.7.1 (the maintainer, 2026-11-15), and the release order was approved with
  S3 the same day. For both: `<lane-pred-head>` is the gated head of
  steps 2 and 3's release, read from the record of the steps 2–3 plan
  (`docs/todo_v0.7.x_kde-open-items-steps2-3-plan.md`, its rule 20 line),
  `<prev-tag>` is `v0.7.3`, this release is 0.7.4 (its number D0's, §8 item 1),
  and its worktree is `v071` (§1). Not taken (D10 (a), 2026-11-15), kept for the record: under D10 (b), step 1 as v0.7.2, every number from 0.7.2 on moves one up
  (steps 2 and 3 0.7.4, so `<prev-tag>` is `v0.7.4` and this release 0.7.5; the
  rationale's D10), the lane predecessor and its record stay steps 2 and 3's, and
  the KDE lane gains one release, so its worktree alternation (§1) moves by one:
  the orchestrator names this release's worktree before P2, the one the steps 2–3
  release does not hold, reused only once the release that last used it has its
  tag (rule 18); §1, P2 and §8 item 7 then read that name in place of `v071` and
  `kde-b`. Not taken (the release order approved with S3, 2026-11-15), kept for
  the record: under another release order, `<prev-tag>` would be whichever tag stands
  immediately before this release in the maintainer's order, this release's
  version would follow that order, and `<lane-pred-head>`, its record and the
  worktree would stay as proposed. Where the branch can be made from `<prev-tag>`
  (P2: `v0.7.3` already exists), there is no MG. In every case the caret follows this release in the KDE lane (S3 as
  amended), so its work never reaches this branch.
- The gates follow the batch's cadence (S1; §4): the targeted tests during a
  unit, `scripts/check_release.sh --quick` at each unit's end
  (`docs/todo_v0.7.1_gate-speed-spec.md` says what it runs, cited by name since
  it is being reviewed), `--commit` on the warm-up gates (P0's or P2's, MG's), on
  every third code unit and the release's last code unit before its documents and
  version units (U5b), and on V, whose lock changes (rule 6); U5c, the pre-merge
  gated head, takes `--quick` (rule 20); the full run in the close-out. Every gate
  run starts as `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh …`
  (rule 2; §6, *Cargo*). The batch's working rules
  hold throughout; this plan cites each as "rule N", meaning the Appendix of
  `docs/todo_v0.7.1_batch-speed-rationale.md`, which is authoritative; the rules
  it marks superseded (13, 23) are not cited. S2's amendment (a merge, not a rebase)
  and the Appendix's rules, with the re-readings they name, were approved by the
  maintainer on 2026-11-15, as were S1 to S5, and S3's amendment and S6 were
  decided by the maintainer the same day (§8, item 0, settled).
- Placeholders: `0.7.N`, `<prev-tag>` (the tag of the release immediately before
  this one in the maintainer's order, which MG merges), `<lane-pred-head>` (the
  gated head of the release before this one in its own lane, which the branch is
  made from), `<merge>` (MG's merge commit), `<date>`, `<sha …>`, `<A5 base>` (the commit step 5's units start from, U4b's),
  `<wip sha>` and `<unit base>` (a checkpoint and its unit's base, §2), `<record>`, `<tools>`, `<locks>`,
  `<scratch-keep>`, `<cargo-build>`, `<core worktree>` (§6), and every count,
  line, time and result read at a run. Each is filled by the task that reads it,
  from its run; the orchestrator writes it into the running record (§6, *The
  record*); none is estimated.

This plan says who does what, in which order, and how each task is known to be
done. What each task changes is the specification's; the plan repeats none of
it.

## 1. Shape of the work

The specification's units are 0, 4a (tasks 1 to 5) and 4b for step 4, and 5a
(tasks 1 to 5), 5b (tasks 1 and 2) and 5c for step 5 (C.1), in its order, in one
release (S5). C.1 lets the plan make each unit one commit or a stack whose inner
commits pass only the gates of the crates they touch. **This plan makes each code
unit one commit** (its choice, with its reason under *Why this order*), built by
its tasks one after the other, each task's state a local work-in-progress commit
that is squashed before the unit's gate and never pushed (§2, *A checkpoint*); and
it adds the tasks around them: the tools, the branch and its merge with
`<prev-tag>` (MG), the documents, the anchors, the measurement's tools (M5), the
icon's colour trial in 5b, the recount after the merge (X), the version and the one
close-out. No task re-points a line citation but one a gate reads (S6; §2, *The citations*).

The release has **one KDE lane**, the core worktree, which is the only place this
plan builds; the widget icons run in a lane of their own beside it (S3 as amended:
the caret's blink is in the KDE lane, after this release), with which this plan
shares the machine's limits (§7).
Everything else runs beside the lane without a worktree of its own: the anchors
and the measurement's scripts read the branch with `git show` and write only
into `<record>`; the reviewers read commits, a unit's local work-in-progress
commits among them (§2, *A checkpoint*).

| Lane | Worktree and branch | Tasks | Cargo |
|---|---|---|---|
| KDE, steps 4 and 5 | `<core worktree>`: `.claude/worktrees/v071`, switched to `v0.7.N-rc1` (*The build folder* below) | P2, P0, T4.1 to T4.5 and U4a, U4b, T5.1 to T5.4 and U5a, MB, U5b, MA, `rec`, U5c, MG (once the tag exists), X, V, K1 to K8 | yes: the only place this plan runs it |

P1 (the tools) runs once, first. A4 (before step 4's units), A5 and M5 (before
step 5's) run beside the lane and commit nothing.

```
step 4
core     P1 P2 ─ P0 ─ T4.1 ─ T4.2 ─ T4.3 ─ T4.4 ─ T4.5 ─ U4a ─ U4b ─┐
beside      A4 ─┘   R4.T1   R4.T2   R4.T3   R4.T45 ─┘ (all closed before U4a's commit)

step 5
core  ┌─ T5.1 ─ T5.2 ─ T5.3 ─ T5.4 ─ U5a ─ MB ─ U5b(1) ─[verdict]─ U5b(2) ─ U5b ─ MA ─ rec ─ U5c ─ MG[(1) ─ (3) VF ─ (6) ─ (7) ─ (4) R-MG ─ (5) warm-up] ─ X ─ V ─ K1 … K8
beside A5, M5   R5.T1  R5.T2  R5.T3  R5.T4 ─┘ (closed before MB)   R5.trial ─┘       R5b.1, R5b.2 ─┘ (closed before U5b's commit)   (VF's review within R-MG)

MG (S2, rules 17, 21): once, after U5b, its reviews, MA, `rec` and U5c (the pre-merge gated head, rule 20), when `<prev-tag>` exists (else the lane waits there); VF runs inside it, at step (3), before the warm-up; nothing is pushed before it
```

Where there is no P0 (§2.1), P2 runs the warm-up gate in its place, and the chain
starts `P1 P2 ─ T4.1`, A4 beside P2's gate. Each `T…` ends in a local
work-in-progress commit (§2, *A checkpoint*); each `U4a`, `U5a`, `U5b` squashes
its unit's into one commit before its gate.

Why this order (each from the specification unless named):

- **One KDE lane.** Every unit needs the one before it: 4a's tasks are ordered so
  that each reads what the one before it wrote (the core's fields, the iced export
  the iced showcase's routes come from, A.11), 4b's texts state what 4a does; 5a's
  tasks likewise, 5b builds on 5a's variant, style and role and is measured against
  5a's tree (B.10), and 5c states both. Two units side by side would need a second
  KDE worktree, a whole extra build of the gpui, iced and egui stacks, for no unit
  that can run alone.
- **Each code unit one commit** (the plan's choice under C.1, which leaves it to
  the plan). C.1's stack leaves its inner commits red by design: after 4a's task 1 the iced leaf
  walk, egui's leaf count and gpui's leaf walk fail until each connector's task
  (C.1, *What stays red inside a unit*), and likewise in 5a; a commit whose own
  workspace tests are red is a bisect stop and cannot pass a gate of the whole
  workspace (`--quick` and `--commit` both test every crate a unit changes and
  every crate that depends on one, S1). One commit per unit has every commit pass
  its gate, keeps C.1's task order
  and its crates per task (each task by its own agent, its change a local
  work-in-progress commit the reviewer reads, squashed before the gate), and
  needs no per-crate gate the batch does not
  have. That no inner commit of the stack leaves its own crates red was read in
  the specification, not run (*What could not be verified*); in one commit the
  question does not arise. The cost: four or five agents' work lands in one
  commit, whose body names each task's part.
- **A4 and A5 first** (a plan addition): the specification's lines are
  `0fa28027`'s, and steps 1 to 3 and egui 0.37 land before step 4. Every quoted
  text, every count's base and the pinned lines are produced again on the tree each
  step starts from: A4 at P2's commit, A5 at the commit step 5's units start from
  (4b's); what MG brings later is read again by MG and X.
- **The facts rows in the core's task** (C.1): A.3's rows go in with T4.1, B.3's
  with T5.1, as the specification's 4a and 5a task 1 have them; no citation of the
  lines they move is re-pointed (A.3, C.4; S6).
- **U4a's commit only once its reviews are closed** and their fixes are in the
  squash; U4b after that, since its texts state what 4a does.
- **MB after U5a's commit** (5a task 5, B.10: the "before" dumps of 5a's last
  commit, before 5b is written): 5a's reviews are closed and their fixes are in
  its commit first, since a later 5a fix that moved the Basic page would be read
  by the comparison as Edit's doing.
- **U5b in two phases**: B.9's and B.13.8's open question, whether a gpui
  Button's icon takes the checked refinement's text colour, is decided by a trial
  run before anything that depends on it is written (§2.4, *The icon's colour*).
- **MA after U5b's reviews** (5b task 2: the "after" dumps are of its subject
  unit's last commit, U5b's, B.10 and rule 19; a fix of this release that touches
  the Basic page or a showcase's layout before MG means the "after" blocks again,
  and after MG K5's after-only checks, §7), then the record commit.
- **MG after the last unit before it, U5c** (S2 as amended, rule 17): the branch starts from
  the lane predecessor's gated head so that its units run while that release's
  close-out and the maintainer's merge, tag and publish are pending; the merge
  with `<prev-tag>` comes once, after U5b, its reviews, MA and `rec` (so MB and MA
  stand on one base; what the merge brings is not measured again, but K5 runs the
  measurement's after-only checks on the rehearsal's dumps of the merged tree,
  rule 19), and after U5c, the pre-merge gated head (rule 20), whose
  texts the merge's conflicts in `CHANGELOG.md`, `ROADMAP.md` and `docs/todo.md`
  then meet once (rules 14, 21); then MG's own steps in rule 21's order and VF; nothing is pushed and no recount, version
  or close-out starts before it, so that the compatibility and semver steps
  compare with the released predecessor. A merge keeps every sha.
- **X after MG and VF, V before the close-out**: X recounts, on the merged tree,
  every stated count in every file either side changed, outside the pinned
  documents, and runs the five documentation checkers (rule 17); with no MG there
  is no X (rule 17); the provenance stamp hashes `Cargo.lock`, which
  V changes.
- **One close-out after both steps** (S5), only after every review is closed and
  its fixes committed. The archive is its last commit (K7), as the steps 2–3
  plan's K7 has it.

**The build folder** (the batch's worktree rule, S2, S3 as amended). Cargo keeps one
build folder per worktree path. The KDE lane (v0.7.1, the three KDE pair releases
and the caret) alternates between two worktrees: `.claude/worktrees/v071`, v0.7.1's,
and `.claude/worktrees/kde-b`. Under D10 (a), decided 2026-11-15 (the header's
*The open choices*), steps 2 and 3 run in `kde-b`, since v0.7.1's close-out holds `v071`;
steps 4 and 5 run in `v071`; the caret in `kde-b`; steps 6 and 7 in `v071`. (Not
taken, D10 (a), 2026-11-15: under D10 (b) the KDE lane would gain one release and the
alternation move by one, the orchestrator naming this release's worktree before P2,
the one the steps 2–3 release does not hold, every `v071` and `kde-b` below, and in
P2 and §8 item 7, then meaning the worktree so named and the other one.) Each release uses the worktree
named here, also where its predecessor's worktree is free when it starts. The
second lane holds the widget icons alone, in `.claude/worktrees/lane2-a`. Each
worktree keeps its own build folder, which its lane's later releases reuse warm; the
orchestrator deletes a lane's extra worktrees and their build folders when the
lane's last release of the batch closes (the KDE lane's, `kde-b`, with steps 6 and
7; the second lane's, `lane2-a`, with the widget icons), and `v071`'s folder stays. So this release's core worktree is `v071`, switched to the new branch while
the steps 2–3 release's close-out runs in `kde-b` (S2), and only once the release
that last used `v071` has its tag (rule 18: checked after
`git fetch origin tag <that tag>`, with `git rev-parse -q --verify <that tag>`;
until then the release waits for it). That release is the one recorded when
`v071` was last switched, whatever it is: v0.7.1 as far as this batch's plans
know; the egui 0.37 release (v0.7.2) is outside the two lanes, and its plan, not
written yet, names its own worktree (the batch's S3, *v0.7.2 (egui 0.37)*, with
`lane2-b` free for it), so P2 still reads the record rather than assuming
v0.7.1: P2 switches it where
`git status --porcelain --untracked-files=no` in it prints nothing and every
untracked file in it is a planning document under `docs/`, and otherwise stops
(§7). Its build folder (its path from
`cargo metadata --format-version 1 --no-deps`, `build_directory`) stays warm and stays after this release; `kde-b`'s
stays for the caret, which uses it next, and until steps 6 and 7 close, which the
close-out's report and the project status note name (§2.5); §8 item 7 names the
departure, which the maintainer approved on 2026-11-15. Worktree paths are relative to the
main checkout, `/home/tibi/Rust/native-theme`; a new one is made with
`git worktree add` from the named commit (an agent's own isolation worktree
starts from `main`, the wrong base).

## 2. Tasks

A task that changes files ends in one commit on the lane's branch, unless it
says otherwise; the tasks of a unit (T4.1 to T4.5, T5.1 to T5.4, and U5b's
phases) end in a checkpoint, a local work-in-progress commit, and their unit in
one commit. A commit is made only when its
done-criterion holds, with no attribution line. **Who runs what** (the batch's
rule of 2026-11-15, §4): an agent runs its task's targeted commands in the
foreground (the named tests, its crate's test legs, `cargo fmt --all --check`,
the checkers, a `-p` build or clippy, each expected to finish well within 10
minutes), stages its own paths by name (`git commit -a` and `git add -A` are not
used), and reports the staged change with its commit message (for a unit's
task, its part of the unit's message); it does not run `--commit`, the full
check, an asset run or the GPUI Fast legs, and it does not commit. The
orchestrator then runs the gate on the staged change and, when it passes,
commits exactly the gated tree with the author's message. In the task rows,
items an agent cannot run are marked "(in the gate)". The subject follows the
repository's form (`feat(core): …`, `docs(todo): …`) and the body says what
changed and why, with the outputs the done-criterion names. Shared terms:

- "**A checkpoint**": inside a unit, once a task (or a fix of it) has staged its
  change, its checks are reported and the staged tree holds (below), the
  orchestrator commits it locally as a work-in-progress commit,
  `cd <core worktree> && git commit -m 'wip(<unit>): <task>'` (for example
  `wip(4a): T4.2` or `wip(4a): T4.2 fix 1`), and writes its sha into the record
  with the unit's base (`<unit base>`: the commit the unit's first task starts
  from: P0's commit, or P2's tree where there is no P0, for 4a; U4b's, `<A5 base>`,
  for 5a; U5a's for U5b). So `HEAD` is the
  previous checkpoint for every check a task states against `HEAD` (the pinned
  lines, a file's line count, `git diff HEAD -U0 -- <file>`), and the next task
  starts from it. The task's reviewer reads `git show <wip sha>`, the brief
  naming the unit's base and that commit. An undo to a checkpoint (before the
  unit's squash) first saves what it drops,
  `cd <core worktree> && git diff HEAD --binary > <record>/undo/dropped-<n>.patch`
  (a new `<n>` each time), and copies every untracked file that is not on P2's
  untracked list (`git status --porcelain --untracked-files=all`, its `??` lines
  compared with that list) into `<record>/undo/dropped-<n>/` under its path, then
  removes it from the tree (a patch leaves untracked files out); then
  `cd <core worktree> && git reset --hard <wip sha>`, never `git stash` or
  `git reset --keep` (which keeps uncommitted changes and so undoes nothing of
  them); done when `git rev-parse HEAD` equals `<wip sha>` and
  `git status --porcelain --untracked-files=all` prints exactly P2's untracked
  list, less those P0 commits; §7's
  take-over starts its new agent from the last reviewed work-in-progress commit.
  Before the unit's gate the orchestrator squashes the unit's work-in-progress
  commits into the index, `cd <core worktree> && git reset --soft <unit base>`,
  then runs the staged-tree check, the gate and the unit's commit with the
  authors' message. **No work-in-progress commit is ever pushed**: before every
  push, `cd <core worktree> && git log --format=%s <remote branch>..HEAD` (the
  branch's `origin/` ref, or `<lane-pred-head>` before the branch's first push,
  which comes after MG) lists no line that starts `wip(`; a line that does is a
  stop. MG runs only after every unit's squash and never while a trial's or a
  unit's edits are uncommitted (§2.4), so no work-in-progress commit reaches the
  merge. A test of the unit that
  a later task of the same unit settles (C.1's list: the leaf walks and counts
  after T4.1 and T5.1, a showcase's route test before T4.5 or T5.4) may be red in
  a task's report, named with the task that settles it; any other red is the
  task's to fix before its checkpoint. A work-in-progress commit is gated by its
  task's own checks only; the unit's squashed commit takes the gate.
- "**The gate passes**": the orchestrator's long run (§4) of the commit's gate in
  the core worktree through the wrappers and `gate.lock` (§6, *Cargo*; rule 2), on
  the staged change: by the batch's cadence (S1; §4, *The cadence*),
  `scripts/check_release.sh --quick` at a unit's end (U5c, the pre-merge gated
  head, among them, rule 20), and `scripts/check_release.sh --commit` where §4's
  count names it (the warm-up gates, MG's among them, U5b as the third code unit
  and the last before the documents and the version, and V); "`--quick` passes" and "`--commit` passes" name the
  mode. The run exits 0 (its `STATUS` file reads 0), its summary names no failure
  and, for `--commit`, no skipped cross-target check (`target not installed`,
  `x86_64-w64-mingw32-gcc not found`), the record names every warning the summary
  repeats, and the tree is formatted and clean; where the row says "with the GPUI
  Fast legs", those run in the same gate. A red gate goes back to the author of
  the change it points at (continued) with the log's failing part; the author
  fixes and stages again, and the orchestrator runs the gate again. A failure a
  `--commit` finds in a commit an earlier `--quick` passed is fixed by a new
  commit, never by rewriting a pushed one, and the record names the commit that
  brought it (S1). For the units whose change a showcase test checks (U4a, U5a,
  U5b, and a fix of theirs), a runner-environment exit 75 at `--prepare`, which
  the gate reports as a soft warning where it runs the showcase legs (in
  `scripts/check_release.sh`, the branch found by its text
  `elif [ "$runner_env_status" -eq 75 ]; then`, `:825` at `0fa28027`: the showcase
  tests not run in CI's runner environment), voids the gate as a skipped cross-target check does:
  the environment is prepared and the gate run again. So does an exit 75 of the
  GPUI Fast showcase leg in the same gates. The orchestrator prepares the runner
  environment (`scripts/run_in_runner_env.sh --prepare`, through the wrappers)
  before it dispatches T4.2, T5.2 and U5b, the first task of each unit whose agent
  runs showcase tests in it; an exit 75 there is reported to the maintainer, and
  the dispatch waits for the maintainer's answer.
- "**Formatted and clean**": §4's rule (`cargo fmt --all --check` by the author
  before staging; the gate on the staged change;
  `git status --porcelain --untracked-files=no` the same before and after the
  gate, and empty after the commit).
- "**The staged tree holds**", checked by the orchestrator at each checkpoint,
  before each gate and again before the commit: `git diff --quiet` exits 0
  (nothing unstaged), except for a close-out fix's unstaged exceptions as rule 15
  names them, where `git diff --name-only` lists exactly those files, named in
  the record (rule 15);
  `git status --porcelain --untracked-files=all` shows no `??`
  line but the untracked documents P2 recorded (less those P0 commits), each named
  one by one in the record (never by a pattern); and the name-status of the
  change lists exactly the task's changed files (`M`: the files its parts own in
  the specification, C.1's row and the *How to read it* table; those whose gated
  claims it re-points, A.10; the generated files it regenerates), anything else named and
  explained before the gate runs: `git diff --cached --name-status`, against the
  previous checkpoint for a task and against `<unit base>` after the squash, where
  it is the union of the unit's tasks' lists. **No task of either step adds a file**: every new test goes into an
  existing file, and the specification names no new source file; an `A` line is a
  finding. P0 (the documents it adds) and K7 (the asset files, the archive's
  renames) are the exceptions, each naming its files.
- "**The GPUI Fast legs**": the two that `--commit` skips
  (`scripts/check_release.sh:710-716`, `:849-855`; `--quick` too, unless the
  gate-speed specification puts them in it), run by the orchestrator in the gate,
  whichever its mode, each a long run, started under the gate's lock as part of
  that gate (rule 2; the Appendix's glossary):
  `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh cargo test -p native-theme-gpui --features gpui-fast,native,watch --all-targets`
  and `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/run_in_runner_env.sh test -p native-theme-gpui --features gpui-fast,native,watch --example showcase-gpui`
  (the trial's GPUI Fast run, §2.4, an orchestrator's long run outside a gate,
  takes the lock too, rule 2; the runner environment prepared first with `--prepare`; an exit 75 is
  reported, not worked around). They run in the gates of U4a, U5a, U5b and of any
  fix that touches the gpui connector's code or showcase, and always in MG's
  warm-up gate, since the merge brings the other side's code (and `<prev-tag>`'s
  version bump) (rule 21 (5)).
- "**The five checkers**": §4's documentation checkers. `$NT_FEATURES` is the list
  `scripts/check_release.sh:420` sets
  (`kde,portal,system-icons,material-icons,lucide-icons,watch,svg-rasterize,system-fonts`
  at `0fa28027`); the orchestrator reads that line on the branch and writes the
  list literally into every brief that uses it. "**The header**" is the
  specification from its top to the end of *Open points*: *How to read it* (with
  *Lines that must not move*), the decisions and the corrections included. Every
  brief also gives the part's *Problems found* (A.14, B.13) and *What could not be
  verified*.
- "**Seen to fail**": a new test, or a changed test's new assertion that tests the
  change, is written first and run before the change it tests; a test that cannot
  compile before its item exists counts, and the report quotes the failure. A
  changed test that only gains a step (a field set, a vector one longer) is run
  after the change, its `ok` quoted. A **guard**, a new test whose every assertion
  states what the change must keep or what holds as soon as the data is in
  (`every_preset_with_a_toolbar_outline_states_its_checked_fill`,
  `a_tool_buttons_box_keeps_its_size_with_the_outline`), is run when written and
  again after the change and passes both times; the report names it "kept" with
  both `ok` lines.
- "**The citations**" (S6; C.4): no task re-points a line citation no gate reads;
  the existing ones are left as they are. Every new citation a task writes of a
  changing repository file names the document (with a quoted phrase or a section
  where useful), never a line; a pinned source (an upstream crate at a version, a
  URL at a commit, a citation "at <sha>") keeps its lines. The gated citations stay
  the task's (S6, *The fourth gate, kept*: `every_colour_claim_is_read_at_the_line_it_cites`
  reads the colour claims' citations of the gpui connector's own files): every
  task that edits `connectors/native-theme-gpui/src/` or its examples (T4.4, T4.5,
  T5.4, U5b) keeps the line count above each cited line, or re-points the claims
  whose lines it moves in the same change (A.10, *Lines that must not move*), and
  its done-criterion runs that test.

### The showcase tests the agent runs

The gate's log prints a test run's summary and no `ok` lines when it passes
(`scripts/check_release.sh:225-260`, its filter `TEST_OK_FILTER`, `:181`), so a
task that must show a showcase test `ok` runs it itself, by name or as the whole
leg, in the foreground, in the runner environment the gate uses
(`scripts/check_release.sh:836-848`):

```bash
cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh scripts/run_in_runner_env.sh test -p native-theme-gpui --features native,watch --example showcase-gpui -- <names>
cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh scripts/run_in_runner_env.sh test -p native-theme-egui --all-features --example showcase-egui -- <names>
cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh scripts/run_in_runner_env.sh test -p native-theme-iced --features iced_aw,watch,native --example showcase-iced -- <names>
```

An exit 75 there (the runner's packages neither fetched nor found) is reported,
and the run is made again once the orchestrator has prepared the environment;
the tests are not run outside it instead. A run the Bash tool kills at 10 minutes
is §4's case.

### 2.1 Before step 4

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| P1 | **The tools**, once. `mkdir -p <locks>` and every subfolder of `<record>` §6 names; copy `slot.sh` and `lowprio.sh` from `<scratch-keep>` into `<tools>` and read both. In the copies: `slot.sh`'s lock folder (`D=`, an old session's scratch folder) becomes `<locks>`, the one lock folder every plan of the batch uses, so the two slots count every cargo process of every plan and session; `lowprio.sh` keeps its absolute `renice -n 19 -p $$` (a relative `nice -n 19` gives 15 under Konsole's −4) and its `ionice -c 3`, and gains `export CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16`, lifted only on the maintainer's word that they are not using the machine. The copies of the approved documents in `<record>/docs/` were made at the approval (§8 item 6), before P1; P1 copies the inputs of §6 into `<record>/inputs/` and creates `<record>/record.md`. Of this work's own documents (rule 26), only one still untracked in the commit the branch is made from is compared with its approval copy. P1 runs before the branch exists, so it first reads the two inputs P2 reads again: `<lane-pred-head>` from the predecessor's record line "the lane's gated head for the next release: <sha> (…)" (rule 20), and whether `<prev-tag>` exists (`git -C /home/tibi/Rust/native-theme rev-parse -q --verify <prev-tag>`, after `git -C /home/tibi/Rust/native-theme fetch origin tag <prev-tag>`), both into the record; then it reads that commit, not a checkout: `git -C /home/tibi/Rust/native-theme cat-file -e <lane-pred-head>:docs/<name>` (`<prev-tag>` in place of `<lane-pred-head>` where the branch is made from it) failing means untracked. One that commit holds (normally added by v0.7.1's commit (0)) is not compared, since later close-outs rewrite its path mentions and set its status line: P1 records the commit that added it (`git -C /home/tibi/Rust/native-theme log --diff-filter=A --format=%H <lane-pred-head> -- docs/<name>`) and copies nothing. No repository change. | orchestrator | the maintainer's resource rules | `<tools>/slot.sh <tools>/lowprio.sh true` exits 0; `<tools>/slot.sh <tools>/lowprio.sh sh -c 'echo "$CARGO_BUILD_JOBS $RUST_TEST_THREADS"; ps -o ni= -p $$'` prints `16 16` and `19`; after the trial `slot1.lock` is in `<locks>` (`ls`; `slot.sh`'s header names `slot1.lock` and `slot2.lock`, and the second appears only when two commands overlap); `cmp` of each copy in `<record>/inputs/`, and of each copy in `<record>/docs/` whose document the `cat-file -e` reading found untracked, against its original prints nothing; each document that commit holds has its adding commit in the record |
| P2 | **The branch and the worktree** (§1, *The build folder*; S2). The branch is made from `<lane-pred-head>`, the head of the lane predecessor's branch (steps 2 and 3's, the release before this one in the KDE lane) once its gated head, its documents unit, the last unit before its merge, has passed its gate (`--quick`, rule 20) and its reviews are closed (rule 18: this release reads those texts), both read from the predecessor's plan's record by its line "the lane's gated head for the next release: <sha> (…)" (rule 20), which is copied into this one; no such line is a stop; P2's two readings (that line and whether `<prev-tag>` exists) are compared with P1's, and where either changed, P1's rule-26 check is run again on the new base before P2 goes on (rule 26); where `<prev-tag>` exists already (`cd <core worktree> && git fetch origin tag <prev-tag>` where it is not local), from that tag instead, and there is no MG (rule 17; this row's checks then read that tree). In `v071` (§1; D10 (b)'s other worktree is not taken, D10 (a), 2026-11-15, the header's *The open choices*): `git -C /home/tibi/Rust/native-theme/.claude/worktrees/v071 switch -c v0.7.N-rc1 <lane-pred-head>`; every other command of this row runs from the core worktree, written `cd <core worktree> && …`. `git switch` refuses where an untracked planning document is a file that tree holds: the two are compared with `cmp`, the untracked copy removed where equal, a stop otherwise (§7). Also read (rule 18): step 1's last commit, as the record of the plan that implemented it names it, is an ancestor of HEAD (`cd <core worktree> && git merge-base --is-ancestor <sha> HEAD`); the tag's relation is checked by MG before the merge (`<lane-pred-head>` an ancestor of `<prev-tag>`, rule 18), or here, where the branch was made from `<prev-tag>`, as `cd <core worktree> && git merge-base --is-ancestor <lane-pred-head> HEAD`, after rule 21 (1)'s tag check, which then runs here: the first field of `cd <core worktree> && git ls-remote origin 'refs/tags/<prev-tag>'` equals `cd <core worktree> && git rev-parse <prev-tag>`, no line printed or a different sha a stop; `v071` is reused only where the release that last used it, as recorded when `v071` was last switched (v0.7.1 today; whatever release that is by then), has its tag (`cd <core worktree> && git fetch origin tag <that tag>`, then `cd <core worktree> && git rev-parse -q --verify <that tag>`; rule 18), and until then the release waits for it (§1); where neither MG nor P0 will run it (the branch made from `<prev-tag>` and nothing left for P0), P2 runs rule 22 (b)'s path check (MG's step (6)) on this tree, a path that moved to `docs/archive/` re-pointed by a `worker` in a documents-only commit made before P2's warm-up gate and covered by it (no gate of its own); the exact version in `Cargo.lock` of every crate the specification cites by version (the header's abbreviations and its other version citations: egui 0.36.2 or 0.37.0, iced_widget 0.14.2, gpui-component 0.9.1, gpui-pre 0.3.12, num-traits 0.2.19 of B.13.3; egui-winit, egui_extras and egui_kittest are cited nowhere in the specification, searched 2026-11-15, so they are in no list); on both paths (the branch made from `<lane-pred-head>` or from `<prev-tag>`), those versions are also compared with `cd <core worktree> && git show 0fa28027:Cargo.lock`, and a changed version's readings are made again before the first unit, or P2's stop (rule 19), each against the specification's; a patch release other than the cited one passes only once A4 (b) has read every line the specification cites in that crate again in the locked version (`~/.cargo/registry/src/`), each with its line there, and a cited text that no longer stands is §7's quoted-text rule; `cd <core worktree> && rustup target list --installed` lists `x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu` and `x86_64-apple-darwin`, and `x86_64-w64-mingw32-gcc` is on the path. The two parser copies that run (§6, *The inputs*): `<record>/inputs/cite2-core.py`, `cite2.py` with its `ROOTS` set to the core worktree's `connectors/native-theme-gpui/src/` and to the registry folder of the gpui-component version `Cargo.lock` holds, and `<record>/inputs/prose-core.py`, `prose.py` with any root it names set the same way, each `diff` against its original in the record. Before any code agent is dispatched, the project's hooks are seen to fire in the core worktree (they are configured with absolute paths in the main checkout's `.claude/settings.json`): a Write of a new, untracked file `<core worktree>/native-theme/src/hook_trial.rs` containing `fn f() { x.unwrap() }` is refused by `no-runtime-panics.sh`, and nothing is written; if the Write applies, the file is removed with `rm` and the plan stops, no code agent dispatched. Where P0 finds nothing left to do (P0's row), P2 then runs §4's warm-up gate, a long run in `<record>/gates/warmup-<n>/`, timed into the record as the release's first `--commit` with whether it ran cold or warm, and after it §4's builds of the feature sets the gate does not build. No commit, but the one documents-only commit of the path check's re-points where the branch was made from `<prev-tag>` and there is no P0 (above). | orchestrator | §1 | `cd <core worktree> && git rev-parse --abbrev-ref HEAD` prints `v0.7.N-rc1` and `cd <core worktree> && git rev-parse HEAD` equals `<lane-pred-head>` (or `cd <core worktree> && git rev-parse <prev-tag>^{commit}` where the branch was made from the tag, after the tag check of rule 21 (1) passed; where P2 made the re-point commit, `cd <core worktree> && git rev-parse HEAD^` equals it instead and `git show --stat HEAD` lists the re-pointed documents alone); `<lane-pred-head>` and the predecessor's record lines that make it the gated head are in the record; `cd <core worktree> && git status --porcelain --untracked-files=no` prints nothing, every untracked file a planning document under `docs/`, the untracked files listed one by one in the record (`cd <core worktree> && git status --porcelain --untracked-files=all`, the list the staged-tree check of §2 reads); the ancestor checks exit 0; the versions, the build folder (`cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo metadata --format-version 1 --no-deps`, `build_directory`, as `<cargo-build>/<hash>`) and the targets are in the record; the hook's refusal is quoted in the record, and `<core worktree>/native-theme/src/hook_trial.rs` does not exist; the two parser copies' diffs are in the record; where there is no P0, the warm-up gate passes and its time is in the record. `v071` not free as §1 says, step 1's last commit not below the branch, `<lane-pred-head>` not shown gated (its documents unit's gate, its reviews) by the predecessor's record, a toolkit version other than the specification's beyond a patch release whose cited lines were read again (egui other than 0.36.2 or 0.37.x), a target or the MinGW linker missing, or the trial Write applied: stop (§7) |
| P0 | **Unit 0: the documents** (C.1, *Commit (0)*; the caret plan's P0 form). v0.7.1's commit (0) adds every document of this cycle the maintainer has approved by then, the `docs/todo_v0.7.x_*` plans among them, and sets their status lines (`docs/todo_v0.7.1_gpui-kit-0.9.1-plan.md`, its P0 row, "**Commit (0).** The documents of this cycle the maintainer has approved", and its §7 item 1, "The documents of commit (0)"), so P0 is likely empty. Where a document of this work is not on the branch (this plan, the specification, the rationale), P0 adds it from `.claude/worktrees/v071/docs/` where it lies untracked today, compared first with `<record>/docs/` by `cmp` (equal, or v071's gone: the record's copy; different: a stop naming what changed, §7); only a document still untracked in the commit the branch was made from is so compared (P1's `cat-file -e` reading, rule 26): one an earlier commit added (normally v0.7.1's commit (0)) is not compared, P1 having recorded the commit that added it, and P0 only sets a status line that commit left. No later-links file is read (rule 22). Where P2 made the branch from `<prev-tag>`, P0 runs rule 22 (b)'s path check, as MG's step (6) describes it, on that tree (else that check runs only after MG), and a path that moved to `docs/archive/` is re-pointed by a `worker` within P0's own commit, covered by P0's `--commit` (rule 22 (b)); every document P0 adds gets its status line set to approved with the date, and where a committed document of this work still has an unapproved status line, P0 sets it the same way; where nothing is left to do, there is no unit 0 (C.1), and P2 runs §4's warm-up gate instead (P2's row). New files: the documents it adds, each named in the record. | orchestrator | §8 | `git show --stat HEAD` lists the documents and the path check's re-points, each named in the record, and nothing else; the five checkers pass; `--commit` passes on the staged change, this first run timed into the record with whether it ran cold or warm (§4); it is step 4's warm-up gate. Commit: `docs: issue #4's open KDE items, steps 4 and 5: the specification and the plan, approved <date>` (the documents it adds or sets named) |
| A4 | **The anchors of step 4**, beside P0. At P2's commit, with `git show <sha>:<path>` and the specification's own search methods, the scripts kept in `<record>/anchors-4/`: (a) every present text Part A quotes to replace or to place an edit by, and every test it changes by name, found in the file named as often as the edit names it, with its line, or "not found"; (b) the earlier releases' parts step 4 builds on, found: step 3's trial and its outcome as that work recorded it (rule 18: U3.1's commit message on the branch, subject `feat: a button's border on hover and on keyboard focus: …`, whose body names the trial's verdict, and the steps 2–3 plan's record, not a status line its close-out writes: passed or failed, and where it failed the state that differed; the two disagreeing is a stop); the predecessor's documents unit's texts as committed in `<lane-pred-head>` (rule 18), which the reading below takes; step 2's `selection_bg` binding beside `view_focus` in `populate_colors` (A.6); the entries steps 1 to 3 appended to `derived_leaves` and `resolved_leaves` with the doc's count (A.6 item 5: 34 after step 3); the backlog entry *What Breeze draws that the model has no field for* as step 3's 3H.4 left it, with its sentences A.12 rewrites; the first rationale's §5.4 status line as step 3 left it; the reply draft as steps 1 to 3 left it (the list "Later 0.7.x releases fix more:" with its bullets **The focus ring** and **Hover borders on buttons**, "Some things are not fixed yet" and its bullets, the note); step 2's item in `README.md`'s list after its `:73`; the steps 2–3 work's `## [0.7.N] - <date>` heading in `CHANGELOG.md` (rule 14), above which U4b puts this release's; (the other lane's releases are never on P2's tree, except where the branch is made from `<prev-tag>`, when A4 reads them here, else at MG, rule 25; the caret comes after this release in the KDE lane, S3 as amended); the egui version in `connectors/native-theme-egui/Cargo.toml` and `Cargo.lock`, and A.9's egui facts read again in that version in `~/.cargo/registry/src/` (`Widgets::style`'s `active` state, `EG/style.rs:1273-1285` at 0.36.2; `style.rs:1383-1395` at 0.37.0, its `active` arm `:1386-1388`, read 2026-11-15), each with its line there; and, for every crate P2 found at a patch version other than the specification's (egui, iced_widget, gpui-component, gpui-pre), every line the specification cites in that crate read again in the locked version, each with its line there or "not found" (§7's quoted-text rule); (c) the counts' bases: egui's `mapping.md` totals (*Buttons* and *Inputs* leaves and SCOPED, TOTAL), the leaf count and its message in `src/mapping_tests.rs`, `verdict_totals_match_the_specification`'s `buttons` and `inputs` tuples, the `derived_leaves` and `resolved_leaves` lengths and the doc's count, `arb_checkbox_theme`'s vector length, the five READMEs' optional-colour counts (A.12) with the count of `Option<Rgba>` soft options by `<record>/inputs/soft.py` on `native-theme/src/model/widgets/mod.rs`, the count in the comment of iced's `a_cleared_widget_soft_option_copies_the_base_state_value` (twelve after step 3, A.8), `ROADMAP.md:264`'s stated count of open `[v0.7.x]` items (29 at `0fa28027`) and `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md` on the tree (41 at `0fa28027`), the two recorded apart; (d) none: no citation list is made, since no citation is re-pointed (S6; C.4); (e) the facts file's lines, each found by its text: §2.5's `hover_background` and `border.color` rows, and §2.24's `hover_background` and `border.color` rows; (f) the pinned lines of *Lines that must not move*, listed again from every `native-theme-gpui/<file>.rs:N` the gpui showcase's claims cite (`examples/showcase-gpui/info/*.rs`, `tests.rs`), with the last pinned line of each file Part A edits; (g) every struct literal of `CheckboxTheme` and `ComboBoxTheme` by `git grep` (A.1: `arb_checkbox_theme` builds every field, `ComboBoxTheme` is built with `Default`), each read to its closing brace: the full ones and those that end in `..`. No commit, no cargo. | `worker` | the header; Part A; C.1 | the six outputs are in `<record>/anchors-4/`, each table complete; nothing is "not found", or each text not found is settled by §7's rule for a quoted text; an egui fact whose line moved in the tree's egui version is listed with its new line; step 3's outcome is found and is "passed" or "failed" (anything else: a stop, §7); the orchestrator copies the summary and step 3's outcome into the record |

### 2.2 Step 4

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| T4.1 | **Unit 4a, task 1: the core**: A.1, A.2, A.3 (the row inserted directly after `hover_background`'s in §2.5 and in §2.24, and no other line changed; the two `hover_background` KDE cells corrected as A.3 says; each placed by its text, A4 (e)), A.5, A.6, A.7, and A.12's two backlog sub-bullets that cite kde-breeze's removed keys by key path, each entire but its dated note (*What Breeze draws …*: its first two sentences as A.12 gives them; *Presets: values the M8 T15 …*: the two clauses removed, the file name moved, the *Issue #4* note), so that each of those texts has one writer; U4b writes the dated note of *What Breeze draws …* (the *Presets* sub-bullet gets none) and the rest of A.12. No citation of the lines A.3 moves is re-pointed, and the preset comments cite the facts file by its section, never a line (A.3, A.5; S6). | `implement` | the header; A.1 to A.3, A.5 to A.7, A.12's two key-path edits; C.1's row; A4 (a) to (c), (e), (g) | (1) seen to fail: A.6's `test_hover_borders_are_breezes` and `a_custom_scheme_moves_the_hover_borders`, the new assertions of `test_missing_groups_leave_the_new_fields_to_the_preset` and `test_full_mapping_completeness`, the lists of `derived_leaves`, `resolved_leaves`, `the_view_decorations_move_…` and `an_accent_moves_…`, changed first against the unit base's tree, and A.7's `hover_outlines_are_stated_where_the_facts_give_them`; `arb_checkbox_theme`'s longer vector run after; (2) they pass, each named `ok`: `cargo test -p native-theme --lib -- property_registry_keys_match_derive_inventory no_inheritance_fields_are_not_inherited every_bundled_preset_source_lints_clean platform_presets_no_derived_fields hover_outlines_are_stated_where_the_facts_give_them documented_sizes`, `cargo test -p native-theme --test proptest_roundtrip`, `cargo test -p native-theme --features "$NT_FEATURES" --lib -- kde:: --nocapture` (its count of tests run quoted; the two new tests, `test_missing_groups_leave_the_new_fields_to_the_preset`, `test_full_mapping_completeness`, `the_derived_colours_are_the_presets_under_breeze`, `the_view_decorations_move_the_frames_the_tab_hover_and_the_handle` and the accent test named `ok`), `cargo test -p native-theme --features kde` (CI's Linux leg of the module, `ci.yml:212`), and the crate's whole leg `cargo test -p native-theme --features "$NT_FEATURES"` (`scripts/check_release.sh:684`); `cargo clippy -p native-theme --all-targets --features "$NT_FEATURES" -- -D warnings` (`:538`) passes; (3) each count equals A4 (c)'s base plus A.6's delta (the two lists and the doc's count +2), both in the report; (4) the rows and the two corrected cells equal A.3's texts, the four kde-breeze blocks and material-3's two equal A.5's texts, and the two sentences of A.12's *What Breeze draws* block (the first placeholder the version step 3's sentence names, the second `0.7.N`) and the *Issue #4* note naming `slider.thumb_hover_color` alone equal A.12's texts (each extracted from the specification and compared by the implementer's script); the rows stand directly after their sibling rows, and no other line of the file changed (`git diff HEAD -U0 -- docs/platform-facts.md` shows the two added rows and the two corrected cells only); the full literals A4 (g) found all name the new field, and `git grep` finds no other literal without `..`; (5) no line citation re-pointed (S6), `cargo test -p native-theme --lib documented_sizes` and `cargo test -p native-theme --test platform_facts_xref` passing as v0.7.1's unit LN left them (`docs/todo_v0.7.1_gpui-kit-0.9.1-spec.md`, Part I), and `check_preset_citations.py` reading the new preset comments' form (Part I.3); (6) the five checkers pass (`check_todo_citations.py` among them: the two key paths removed with their keys, and the preset's file name moved to the first citation left, ``… `slider.thumb_hover_color` `#93cee9` (`kde-breeze.toml` `[light.slider] thumb_hover_color`, …``, A.12); (7) `cargo fmt --all --check` passes, the change is staged and its checkpoint recorded |
| T4.2 | **Unit 4a, task 2: egui**: A.9 entire (the two roles, `selected_cell`, the mapping rows and `when`s, the counts, the contract arms, the tests, egui-widgets' `combo_box.rs` doc line), `mapping.md` regenerated. Each egui text in the tree's version's form (A4 (b)). | `implement` | the header; A.9; C.1's row; A4 (a) to (c) | (1) seen to fail: the new assertions of `a_none_soft_option_copies_its_fallback_leaf`, `checkbox_cell_writes_its_rows` and `combo_box_and_segmented_control_cells_write_their_rows` for `hovered` and `active`, and `the_checkbox_selected_cell_fills_the_box_and_outlines_it`'s sentinel; (2) they pass: `cargo test -p native-theme-egui --all-features --lib -- style:: every_leaf_has_exactly_one_row_and_every_row_is_well_formed verdict_totals_match_the_specification t2_every_row_s_declared_sinks_are_exactly_what_its_leaf_moves t10_every_sink_holds_the_leafs_value t10c_every_undeclared_field_holds_its_inherited_value mapping_md_is_current --nocapture`, its count quoted, each test above named `ok`; the crate's legs `cargo test -p native-theme-egui --all-features` and `--no-default-features` (`scripts/check_release.sh:760-762`), `cargo test -p native-theme-egui-widgets --all-features` and `--no-default-features` (`:770-772`); `cargo clippy -p native-theme-egui --all-targets --all-features -- -D warnings` and the same for `native-theme-egui-widgets` (`:764`, `:774`) pass; the egui showcase's leg in the runner environment by the agent's own run (*The showcase tests the agent runs*, the whole `--all-features` leg), its result line quoted and `the_theme_map_names_the_tests_that_check_a_per_call_route` named `ok`; (3) `mapping.md` written by its command only (`cargo test -p native-theme-egui --lib -- --ignored regenerate_mapping_md`); (4) each count equals A4 (c)'s base plus A.9's delta (the leaf count and its message's `ResolvedTheme` figure +2; *Buttons* and *Inputs* leaves +1 and SCOPED +1 each; the `buttons` and `inputs` tuples' scoped +1), both in the report; (5) no line citation re-pointed but a gated claim (S6); (6) `cargo fmt --all --check` passes, the change is staged and its checkpoint recorded |
| T4.3 | **Unit 4a, task 3: iced**: A.8 entire (`CheckFills`, its two comments and `at`'s doc, the `checkbox` and `radio` docs, `pick_list` and its doc, the contract rows, `a_cleared_hover_border_copies_the_border_at_rest`, the windows-11 test's comment), `contract-leaves.toml` regenerated by its command. | `implement` | the header; A.8; A.11's iced bullet; C.1's row; A4 (a) to (c) | (1) seen to fail: `a_cleared_hover_border_copies_the_border_at_rest` and the changed expectations of the contract rows' tests; (2) they pass: `cargo test -p native-theme-iced --all-features --lib -- styles:: contract:: every_colour_leaf_reaches_a_row_or_is_named --nocapture`, its count quoted, the new test named `ok`; the crate's legs `cargo test -p native-theme-iced --no-default-features` and `--features iced_aw,watch,native` (`scripts/check_release.sh:737-739`); its two clippy legs (`:741`, `:746`) pass; the iced showcase's leg in the runner environment by the agent's own run, its result line quoted, `the_element_list_parses_and_every_leaf_has_a_route` and `styles_cover_every_widget_shown` named `ok`; (3) `contract-leaves.toml` written by its command only (`cargo test -p native-theme-iced --all-features --lib -- --ignored regenerate_contract_leaves`), its diff read against A.8's expected differences, each other difference a finding in the report; (4) the windows-11 test's comment states the count A4 (c) found (twelve, A.8), or the difference is explained; (5) no line citation re-pointed but a gated claim (S6); (6) `cargo fmt --all --check` passes, the change is staged and its checkpoint recorded |
| T4.4 | **Unit 4a, task 4: the gpui connector**: A.10's `CheckboxLook` (the field, `hover_fill`'s doc line, `of`'s doc), `sized`, the render's one group-hover style, `UNREAD_COLOURS`' entry and the unit tests (`a_checkbox_paints_checkbox_theme`, `an_unset_hover_border_keeps_the_border`); the showcase claims that cite `widgets/checkbox.rs` (`examples/showcase-gpui/info/inputs.rs`, `tests.rs`), which the claim gate reads, re-pointed by the lines the edit adds above them; `checkbox_leaves`' doc keeps its citations (no gate reads them, S6; its sixth leaf's clause is T4.5's). | `implement` | the header; A.10 to its *gpui-component's `Select` and `Combobox`* paragraph; C.1's row; A4 (a), (f) | (1) seen to fail: `a_checkbox_paints_checkbox_theme`'s new assertions and `an_unset_hover_border_keeps_the_border`; (2) they pass: `cargo test -p native-theme-gpui --features native,watch --lib -- widgets:: every_colour_leaf_reaches_a_sink_or_is_named every_colour_claim_is_read_at_the_line_it_cites every_edge_colour_is_claimed_through_edge_claim every_prose_citation_still_exists --nocapture`, its count quoted, each named test `ok`; the crate's legs `cargo test -p native-theme-gpui --features native,watch` (`scripts/check_release.sh:667`) and `--lib --no-default-features` (`:705`); `cargo clippy -p native-theme-gpui --all-targets --features native,watch -- -D warnings` and `--no-default-features` (`:530`, `:550`) pass; the gpui showcase's tests that name the check box or the radio (found by `grep` for `checkbox`, `radio`, `basic-check` and `basic-radio` in `examples/showcase-gpui/tests.rs`, named in the report) by the agent's own runs, each `ok` quoted or, where it waits for T4.5's part, named as such (§2, *A checkpoint*); (3) no pinned line moved but those A.10 moves: every claim citing `widgets/checkbox.rs` cites the line that reads its field (the claim gate above), each re-pointed claim old and new line in the report; `git diff HEAD -U0 -- connectors/native-theme-gpui/src/colors.rs connectors/native-theme-gpui/src/variants.rs connectors/native-theme-gpui/src/geometry.rs` prints nothing (`HEAD` is T4.3's work-in-progress commit); (4) `python3 scripts/check_connector_parity.py` passes; (5) no line citation re-pointed but a gated claim (S6); (6) `cargo fmt --all --check` passes, the change is staged and its checkpoint recorded |
| T4.5 | **Unit 4a, task 5: the showcases**: A.10's showcase part (`combo_surface` with its group `const`, overlay and doc; the Widget Info `edge_claim`s of the drop-down, the check box and the radio, the `fill` text; `info/leaves.rs`' two `HOW` lines; the tests: `a_theme_drawn_controls_info_prints_its_look`'s entry, `checkbox_leaves`' sixth leaf and its doc's clause, `painted_edges`, `a_hovered_checkbox_takes_the_hover_border`, `a_hovered_drop_down_takes_the_hover_border`) and A.11's element edits, in this task (A.10: the `HOW` lines land with the elements). | `implement` | the header; A.10's showcase part; A.11; A.14; C.1's row; A4 (a), (f) | (1) seen to fail: the two new tests and the info test's new entry; the element list's new leaves, added first, fail the gpui showcase's `every_listed_leaf_has_a_route`, run before the `HOW` lines; (2) they pass, by the agent's own runs (*The showcase tests the agent runs*): the gpui showcase's whole `native,watch` leg in the runner environment, its result line quoted, the two new tests, `a_theme_drawn_controls_info_prints_its_look`, `every_listed_leaf_has_a_route` and `the_preset_combobox_paints_its_fill_swatch_and_names_its_row_hover` named `ok`; every existing gpui showcase test that clicks or opens one of the four drop-downs (the side panel's theme, mode and icon theme pickers and `basic.drop_down`, found by `grep` for their selectors in `tests.rs`, named in the report) passes, the run that reads A.14.1's unbuilt click-through (decision 7); the iced showcase's and the egui showcase's whole legs in the runner environment, their result lines quoted, iced's `the_element_list_parses_and_every_leaf_has_a_route` named `ok`; `cargo test -p native-theme-gpui --features native,watch --lib -- every_how_entry_is_shown every_edge_colour_is_claimed_through_edge_claim every_colour_claim_is_read_at_the_line_it_cites every_prose_citation_still_exists` named `ok`; each `HOW` text checked first with `<record>/inputs/cite2-core.py` and `prose-core.py` (the transcriptions of the showcase's citation parsers), their output in the report; (3) `python3 scripts/check_showcase_parity.py --check-list` passes; (4) `git diff HEAD -U0 -- connectors/native-theme-gpui/src/` prints nothing (`HEAD` is T4.4's work-in-progress commit; the showcase part touches no connector source file); (5) no line citation re-pointed but a gated claim (S6); (6) `cargo fmt --all --check` passes, the change is staged and its checkpoint recorded |
| U4a | **Unit 4a's commit**: the orchestrator's, once T4.5's work-in-progress commit is made and every must-fix of R4.T1 to R4.T45 so far has its own (§5): the squash, `cd <core worktree> && git reset --soft <unit base>` (P0's commit, or the branch's head where there is no P0), then the gate and the commit (§2, *A checkpoint*). | orchestrator | §2; §5 | `git log --format=%s <unit base>..HEAD` printed nothing after the squash; the staged tree holds, its name-status the union of T4.1 to T4.5's lists; the gate passes (`--quick`, §4's cadence: 4a is the first code unit after the warm-up gate), with the GPUI Fast legs (in the gate); the reviews of §5 for 4a are closed, their fixes in the commit (a fix after the gate runs the gate again). Commit: `feat: the hover outlines of the check box, the radio and the combo box: the fields, kde-breeze and the KDE reader, material-3's combo box, and the three connectors and showcases` (the body one paragraph per task, from each task's report, with the counts and the regenerated files' differences). Not pushed before MG (S2; §4, *CI*) |
| U4b | **Unit 4b, the documents**, after U4a's commit, once every review of step 4 (§5) is closed and its fixes are committed: A.12 but its two key-path sub-bullets (T4.1's, the first entire but its dated note, which is U4b's) and the rationale's status line (K7's, S5): the dated note of *What Breeze draws …* (the *Presets* sub-bullet gets none), the READMEs' counts (each the count `soft.py` gives on the tree, with "five" kept), step 4's CHANGELOG entries under a new heading `## [0.7.N] - <date>` (rule 14: the proposed number, never `## [Unreleased]`, `<date>` left as written until V fills it), its section above every other `## [` heading of `CHANGELOG.md`, the release's one section that U5c extends and V dates (S5, C.3; its *Breaking Changes* bullet A.12's, which U5c makes the one bullet of both steps, B.11), and the release's one ROADMAP section with its heading and first paragraph as A.12 gives them, after steps 2 and 3's, the backlog's notes and new `[1.0]` entry, `ROADMAP.md:264`'s recount, the first rationale's §5.4 line, and the reply draft's bullet (only while the reply is unposted, as the orchestrator reads it before U4b, rule 16: `cd <core worktree> && gh issue view 4 --comments`, posted being a comment by the repository owner that carries the draft's opening sentence, `docs/issues/0004_kde-hand-overrides-rationale.md` §8; a failed read is reported and the reply edits wait; posted, the draft is not edited). The release placeholders are written as `0.7.N`, the release that brought step 3 as A4 (b) recorded it, and each `<… step 3's trial …>` text in the outcome's form. | `worker` | the header; A.12; the record's step 3 outcome and the maintainer's answers; A4 (a) to (c) | each text equals the specification's block (extracted and compared by the worker's script in `<record>/u4b/`), placed by its anchor (A4 (a), (b)); each changelog entry one physical line; the five READMEs' counts equal `soft.py`'s count on the tree, in the body; `ROADMAP.md`'s count equals `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md` at the commit, in the body, any difference from A4 (c)'s base explained; the five checkers and `python3 scripts/check_readme_versions.py` pass; the change is staged; the gate passes (`--quick`, a documents-only unit's, S1; the READMEs are crate docs, `#[doc = include_str!("../README.md")]`); `CHANGELOG.md`'s `## [0.7.N] - <date>` heading is the first `## [` heading of the file. Commit: `docs: the hover outlines in the READMEs, the changelog, the roadmap, the first rationale and the backlog`. Not pushed before MG |

### 2.3 Before step 5

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| A5 | **The anchors of step 5**, after U4b's commit and before T5.1, at `<A5 base>`, the commit step 5's units start from, as A4: (a) every present text Part B quotes, and every test it changes by name; (b) the earlier releases' parts step 5 builds on, found: step 3's trial outcome as A4 (b) recorded it, read again on the branch; step 3's `keeps_resting_stroke` and its one call in `role_cell`, `flat_and_part_scopes_keep_their_resting_stroke`, `panel_role_cells_write_their_rows`' «toolbar» comparison and `["button.border.color"]`'s fourteen scoped sinks (3E.1 to 3E.3), and, where the trial passed, `variants::bordered_button` and `hover_border` with their claimed lines; step 4's `populate_colors` doc sentence on the combo box's pen (A.6), step 4's backlog note at the end of *What Breeze draws …* and its list "Breeze also draws, with no field yet:", step 4's §5.4 sentence "Also by D6 …", step 4's reply bullet **Hover outlines on check boxes, radio buttons and combo boxes**, the reply's sentence that begins "Toolbar buttons:" and the note as step 3 left them (B.11); the egui version on the tree and B.7's and B.10's egui facts read again in it (`Button::frame_when_inactive` and its class, `egui-0.37.0/src/widgets/button.rs:191-205` read 2026-11-15; `WidgetInfo::selected`'s first parameter, an accesskit `Role` at `data/output.rs:704` in 0.37.0, `WidgetType` in 0.36.2), which pick each `<egui 0.37 \| egui 0.36.2>` text of B.7, B.10 and B.11; B.9's and decision 14's gpui-component and gpui-pre lines (among them `GC/button/button.rs:1028`, `:1080`, `:1118-1124`, `:1337`, `:724-729`, `:821-838`, `GC/toolbar.rs:17-22`, `:71-74`, `GC/icon.rs:213-214`, `:247`, `:260-266`, `GP/window.rs:3112-3120`) read again in the versions `Cargo.lock` holds at `<A5 base>` where they are newer than the specification's (P2's reading; MG's version check reads them again, rule 19), and every cited line of any other crate locked at a patch version, each with its line there or "not found"; (what landed first in another lane is MG's reading, as A4 (b) says); (c) the counts' bases: egui's leaf count and its message, the *Chrome* totals and the `chrome` tuple (`[0, 54, 27, 19]` at `0fa28027`), the `derived_leaves` and `resolved_leaves` lengths and the doc's count word (36 after step 4), the iced `pub fn` count of `src/styles.rs` less `composite_over` with the `styles` row of `src/lib.rs` (33 functions at `0fa28027`) and `ROADMAP.md`'s "has <n> style functions" sentence found by its text (`:162` at `0fa28027`), `accent_fills`' length (6), the five READMEs' counts with `soft.py`'s, `ROADMAP.md:264`'s stated count and the `grep -c` of open `[v0.7.x]` entries, recorded apart; (d) none, as A4 (d); (e) the facts file's lines, each found by its text: §2.13's `border.shadow_enabled` row; (f) the pinned lines as A4 (f), with the last pinned line of `variants.rs` and `geometry.rs`, the line of `use crate::Native;` in `variants.rs`, the end of `hover_border` (where step 3's trial passed) or of `switch_color`, the line of `variants.rs`' `#[cfg(test)]`, and `button_checked`'s end in `geometry.rs`; (g) every struct literal of `ToolbarTheme`, as A4 (g). No commit, no cargo. | `worker` | the header; Part B; C.1 | as A4; the egui version and each picked form in the record |
| M5 | **The measurement's tools** (B.10, *The measurement* to *The duration*; the specification's *What could not be verified*: "the plan writes them again from B.10's logic"), beside T5.1 to T5.4. In `<record>/measure-5b/`: (1) `precheck.py`, B.10's pre-check, from `<record>/inputs/precheck.py`, read item by item against *The pre-check* (the map of dropped entries to kept ones, the layout inputs, colours by the registry's types with `[_structures.*]`, `defaults.border.opacity` folded, a key the registry gives no type failing, a key on one side only a difference); (2) `compare_layout.py`, B.10's comparison, from `<record>/inputs/compare_layout.py`, read against *The comparison*'s items 1 to 7 (the comparator's own `load_elements`, `load_dump`, `visible_area` and `snapped_edges`; exit 2 for a missing or malformed file or an unknown id; the stop entries, the moved lines, the added ids, item 5's new-line test with its stop entries, item 6's parity entries; exit 0 or 1); each departure from B.10 corrected in the copy and named; (3) `block.sh KIND TREE`, B.10's block command (*The command*) with its nine ENTRIES and its features per kind, OUTPUT `"$PWD/target/measure-5b/$TREE/$kind-{theme}-{variant}.capture.png"` and DUMPS `"$PWD/target/measure-5b/$TREE"`, `KIND` setting `kind` and its features as B.10's three blocks do; (4) `after_state.py ROOT DUMPS OUT [ELEMENTS]`, ELEMENTS the element list read in place of the tree's, default `<ROOT>/docs/showcase-elements.toml`, as `compare_layout.py`'s fourth argument (`steps45b/compare_layout.py:22-23`; an id the list lacks is exit 2, `:39-45`), run at MA on `target/measure-5b/after` with the default (rule 19): for each kind × entry of the measurement's nine, the listed `basic.` ids present in the dump and the ids wholly inside their visible area (item 3's test), with the dump's `scale`, written as JSON to OUT, `<record>/measure-5b/after-state.json`, kept after K3 deletes `target/measure-5b`; exit 0, or 2 for a missing or malformed dump. (5) `after_only.py ROOT DUMPS ENTRIES_FILE STATE [ELEMENTS]`, ELEMENTS as in (4) (K5 uses the default), the comparison's after-side checks, relative to MA's state (rule 19), for K5's run on the rehearsal's merged-tree dumps: it reads the `ENTRIES=(` array of ENTRIES_FILE (the rehearsal copy, `<record>/assets/rehearsal.sh`) and STATE (`after-state.json`); exit 2 where the array is empty or lacks any of the nine entries STATE holds (matched by preset and variant: B.10's block `ENTRIES` and the rehearsal's share the form `theme:variant:icon_set:icon_theme`, which `capture_window.sh` splits with `IFS=':'` at `:450` and names `$kind-$theme-$variant.json` at `:461-462`, as `rehearsal.sh` does at `:24-26`, so an entry's dump has the same name in both folders; the icon fields do not enter the name), and exit 2 where DUMPS lacks `<kind>-<preset>-<variant>.json` for a kind (`egui`, `iced`, `gpui`, the rehearsal's loop) × entry; for each kind × entry of the nine, a failure where an id STATE lists as present is missing from the merged-tree dump, or one it lists as wholly inside is not wholly inside there (an element outside in both is no failure: the page scrolls, so some elements are outside by design); for every entry, item 5's Edit inside wherever Copy is and, per entry, item 6's wrap parity across the three kinds (one showcase wrapping Edit while another does not) as a failure; an entry outside the nine (one the rehearsal adds from the capture scripts) has no MA state, so only its visible-area findings are reports: each listed `basic.` id missing or not wholly inside is reported, never a failure, while item 5's Edit-versus-Copy check and item 6's wrap parity apply to it as to every entry. Rectangles are compared as logical edges, each dump's snapped at its own `scale` by the comparator's `snapped_edges`, and both scales printed: MA's dumps come from the desktop, the rehearsal's from Xvfb, and whether their scales agree is recorded as not verified. Exit 0 a pass, 1 a failure, 2 an input fault (as above, a malformed dump, an id the list lacks, an unreadable ENTRIES array or STATE), on the comparator's own functions as `compare_layout.py` loads them. Its self-tests run on a fixed tree, not the moving core worktree: `<A5 base>` extracted once into `<record>/measure-5b/tree-p2/` by *M5's tree* below (the folder made first with `mkdir`), and `precheck.py`, `make_test_pair.py` and `compare_layout.py` given that folder as their repository root. No repository change, no cargo. | `worker` | B.10; `<record>/inputs/` | `block.sh`'s command equals B.10's block (extracted from the specification by the worker's own script) but for `$TREE`, `kind` and the features, and `bash -n <record>/measure-5b/block.sh` exits 0; `python3 -I <record>/measure-5b/precheck.py <record>/measure-5b/tree-p2` exits 0, its printout in `<record>/measure-5b/precheck-p2.txt` (the specification's draft found the 25 dropped entries equal at `0fa28027`; a difference is a stop, §7: the nine entries would no longer stand for the 34); `compare_layout.py`, run on hand-made pairs made by `python3 -I <record>/inputs/make_test_pair.py <record>/measure-5b/tree-p2 <record>/measure-5b/pair/` (which writes `before/`, `after/` and the list with the two Edit elements, `elements.toml`, the list the comparison reads in place of the tree's, as its fourth argument: `python3 -I <record>/measure-5b/compare_layout.py <record>/measure-5b/tree-p2 <before> <after> <record>/measure-5b/pair/elements.toml`; the draft takes it so, `steps45b/compare_layout.py:22-23`) and changed by `move_edit.py` or by hand per case, exits 0 on a pair that keeps every element inside with Edit beside Copy (the clean pair, `pair-clean/`, below), and 0 on a pair where Edit wraps in all three showcases, its moved lines printed (the clean pair serves as that case too: under material-3, `make_test_pair.py` wraps Edit in all three showcases, `:81-82`); 1 with a stop entry for an element pushed out of its visible area, for an element missing after, for Edit missing from an "after" dump, and for Edit outside its visible area while Copy is inside; 1 with a parity entry where one showcase wraps Edit and the others do not; 2 for an id the list lacks, and 2 for a dump file missing and for one malformed; `after_state.py` and `after_only.py` are self-tested with ROOT `<record>/measure-5b/tree-p2` and ELEMENTS `<record>/measure-5b/pair/elements.toml` (the pair's Edit elements are in that list only; `docs/showcase-elements.toml` lacks them until U5b, so the default would exit 2 on every pair): `python3 -I <record>/measure-5b/after_state.py <record>/measure-5b/tree-p2 <pair folder> <state file> <record>/measure-5b/pair/elements.toml` and `python3 -I <record>/measure-5b/after_only.py <record>/measure-5b/tree-p2 <pair folder> <record>/measure-5b/pair/entries.sh <state file> <record>/measure-5b/pair/elements.toml`; `after_state.py` on a pair's "after" folder writes a STATE that lists each dump's present and inside ids (checked against the pair's dumps by the worker's own reading) and exits 0, and 2 for a missing and for a malformed dump; `after_only.py` on the pairs' folders, with an ENTRIES file in the rehearsal's form that names the pairs' entries (`<record>/measure-5b/pair/entries.sh`, written by the worker), and with a STATE taken for the exit-0 cases from a **clean pair**, never from the raw one (the raw pair deliberately makes Edit wrap in gpui alone under macos-tahoe, `make_test_pair.py:83-84`, and deletes `basic.icons.toolbar` from egui's ios-13 "after", `:94-95`, so item 6's parity fails on it even against itself): the clean pair is a second output folder, `<record>/measure-5b/pair-clean/`, written by M5's copy `<record>/measure-5b/make_test_pair.py`, which is `<record>/inputs/make_test_pair.py` with one added flag, `--clean`, that skips those two deliberate faults and nothing else (its `diff` against the input in the record): `python3 -I <record>/measure-5b/make_test_pair.py --clean <record>/measure-5b/tree-p2 <record>/measure-5b/pair-clean/`; compare_layout's own exit-0 pair is that folder (`compare_layout.py` on it exits 0, the run in `selftest.txt`); the raw pair stays the input of the exit-1 cases. `after_only.py` then exits 0 where the clean pair's dumps equal the clean state's, 0 where an element lies outside its visible area in both the state and the dump, and 0 with an extra entry outside the nine in ENTRIES whose element lies outside its visible area, that finding reported, and 1 where that extra entry's Edit is outside while Copy is inside (in all three kinds' dumps, as below); 1 for an element inside in the state and pushed out in the dump, for an id present in the state and missing from the dump (a dump with no Basic page among them), for Edit outside while Copy is inside (Edit moved out in all three kinds' dumps of the entry, so that wrap parity does not mask item 5's result), and for a wrap-parity case (one kind's Edit wrapped, the others' not, under one entry); 2 for an ENTRIES array that lacks one of the state's nine, for an empty ENTRIES array, for a dump missing for one kind × entry, for a malformed dump and for an id the list lacks; each run's command and printout in `<record>/measure-5b/selftest.txt` |

*M5's tree*, `<A5 base>` as files, from the core worktree (the pipe is the
command's own; nothing is written into the worktree):

```bash
cd <core worktree> && git archive <A5 base> | tar -x -C <record>/measure-5b/tree-p2/
```

### 2.4 Step 5

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| T5.1 | **Unit 5a, task 1: the core**: B.1, B.2, B.3 (the four rows inserted directly after §2.13's `border.shadow_enabled` row, and no other line changed, each placed by its text, A5 (e)), B.4 (the presets and `toolbar_button_states_are_stated_where_the_facts_give_them`), B.5, B.5a. No citation of the lines B.3 moves is re-pointed, and the preset comments cite the facts file by its section (B.3, B.4; S6). | `implement` | the header; B.1 to B.5, B.5a; C.1's row; A5 (a) to (c), (e), (g) | (1) seen to fail: B.5's `test_toolbar_button_states_are_breezes`, the new assertions of the custom-scheme test, `test_missing_groups_leave_the_new_fields_to_the_preset` and `test_full_mapping_completeness`, the lists of `derived_leaves`, `resolved_leaves`, `an_accent_moves_…` and `the_button_set_moves_…` changed first, B.4's new test, and B.5a's `accent_fills` entry (its three tests run on Windows only: type-checked here by the cross-target commands below and run by CI's Windows legs, read at K3, whose CI run is the release's first); (2) they pass, each named `ok`: `cargo test -p native-theme --lib -- property_registry_keys_match_derive_inventory no_inheritance_fields_are_not_inherited every_bundled_preset_source_lints_clean toolbar_button_states_are_stated_where_the_facts_give_them hover_outlines_are_stated_where_the_facts_give_them documented_sizes`, `cargo test -p native-theme --test proptest_roundtrip`, `cargo test -p native-theme --test merge_behavior`, `cargo test -p native-theme --features "$NT_FEATURES" --lib -- kde:: --nocapture` (its count quoted, the tests above named `ok`), `cargo test -p native-theme --features kde`, and the crate's whole leg `cargo test -p native-theme --features "$NT_FEATURES"` (`scripts/check_release.sh:684`); `cargo clippy -p native-theme --all-targets --features "$NT_FEATURES" -- -D warnings` (`:538`); the Windows reader type-checked from Linux by the gate's own commands, `cargo clippy --target x86_64-pc-windows-msvc -p native-theme --all-targets --all-features -- -D warnings` (`:494-495`) and `env RUSTFLAGS="-D warnings" cargo check --target x86_64-pc-windows-gnu -p native-theme --all-features` (`:476-477`); (3) each count equals A5 (c)'s base plus B.5's delta (the lists and the doc's count +3; `accent_fills` 6 → 7), both in the report; (4) the four rows equal B.3's texts; the rows stand directly after their sibling row, and no other line of the file changed (`git diff HEAD -U0 -- docs/platform-facts.md` shows the four added rows only); and the twenty preset values and their comments, and the four `not stated` lines, equal B.4's texts (extracted and compared); (5) no line citation re-pointed (S6), `cargo test -p native-theme --test platform_facts_xref` passing as v0.7.1's unit LN left it (Part I), and `check_preset_citations.py` reading the new preset comments' form (Part I.3); (6) the five checkers pass; (7) `cargo fmt --all --check` passes, the change is staged and its checkpoint recorded |
| T5.2 | **Unit 5a, task 2: egui**: B.7 entire, in the tree's egui version's form (A5 (b)): the role's writes and their comment, the predicate's new form and its call with the import, the four rows, `["button.border.color"]`'s two «toolbar» `when`s, the counts and totals, the contract arm, the accessor with its re-export and the README paragraph, `Role::Toolbar`'s doc, the tests, and the two `kind = "absent"` parity exceptions; and B.10's egui bullet for the six tool buttons (`demo::tool_button`'s `edge` and its doc, its four callers, the six `PER_CALL_ROUTES` entries and their doc; Edit's entry is U5b's); `mapping.md` regenerated. | `implement` | the header; B.6, B.7; B.10's egui bullets; C.1's row; A5 (a) to (c) | (1) seen to fail: `sidebar_and_toolbar_write_their_own_rows`' new «toolbar» assertions, the changed and new assertions of `flat_and_part_scopes_keep_their_resting_stroke` and `panel_role_cells_write_their_rows`, `a_none_soft_option_copies_its_fallback_leaf`'s new part, and `the_atlas_reports_what_the_builder_was_given`'s new expectation; (2) they pass: the egui `--lib` command of T4.2 (2) with these tests, its count quoted, each named `ok`; the crate's legs and egui-widgets' as T4.2; the two clippy legs; `python3 scripts/check_connector_parity.py` passes; the egui showcase's whole `--all-features` leg by the agent's own run, `the_theme_map_names_the_tests_that_check_a_per_call_route` named `ok`; (3) `mapping.md` by its command only; (4) each count equals A5 (c)'s base plus B.7's delta (the leaf count and its message's figure +4; *Chrome* leaves +4, SCOPED +3, DERIVED +1; the `chrome` tuple's scoped +3 and derived +1), both in the report; (5) no line citation re-pointed but a gated claim (S6); (6) `cargo fmt --all --check` passes, the change is staged and its checkpoint recorded |
| T5.3 | **Unit 5a, task 3: iced**: B.8 but its Edit parts (U5b's): `styles::toolbar_button` and its doc, the contract (statuses, `native_toolbar_button`, rows, field lists, `toolbar_button_style_fields` and its call, `DERIVED` entries, `assert_clone`), `TOOLBAR_BUTTON_PAIRS`, `a_checked_toolbar_button_takes_the_checked_text`, the README's row (`README.md:189`, in place), the crate doc's `styles` row (`src/lib.rs:318`, in place), `ROADMAP.md`'s count of style functions and its list of them (found by their text, in place), and the showcase's switch of the six tool buttons with `IconButton::toolbar`, `DRESSED`'s entry, `ghost_button`'s doc, the routes (`Shown`, `tool_button`, `TOOLBAR_BUTTON`, `drawing()`'s arms, `iced_route`'s, `ghost_padding`'s guard, text and doc), `icon_colour`'s parameter with its callers, `drawn_by`'s arms and the oracle (`toolbar_drawn`, `route_faults`' use of it); `contract-leaves.toml` regenerated. | `implement` | the header; B.6, B.8; C.1's row; A5 (a) to (c) | (1) seen to fail: `a_checked_toolbar_button_takes_the_checked_text`; the showcase's `styles_cover_every_widget_shown` with the new `pub fn` before the switch; (2) they pass: `cargo test -p native-theme-iced --all-features --lib -- styles:: contract:: every_colour_leaf_reaches_a_row_or_is_named style_contrast_never_degrades_the_native_pair --nocapture`, its count quoted, the new test named `ok`; the crate's legs and two clippy legs as T4.3; the iced showcase's whole leg by the agent's own run, `styles_cover_every_widget_shown`, `every_hand_colour_route_names_a_function_that_reads_it`, `a_route_names_a_widget_only_where_the_state_draws_it` and `the_element_list_parses_and_every_leaf_has_a_route` named `ok`; (3) `contract-leaves.toml` by its command only, its diff read against B.8's *Contract* paragraph (the four leaves on `styles::toolbar_button` rows only; the six `button.*` colour leaves gaining its rows, the two fills with `except` lists; `button.checked_text_color` and `button.active_text_color` under the checked statuses as B.8 says), each other difference a finding in the report; (4) the `styles` row's count and `ROADMAP.md`'s each equal `grep -c '^pub fn' connectors/native-theme-iced/src/styles.rs` less one for `composite_over` (A5 (c)'s base plus one), both in the report, and the paragraph names `styles::toolbar_button` once, after `styles::button_toggle`'s entry; (5) no line citation re-pointed but a gated claim (S6); (6) `cargo fmt --all --check` passes, the change is staged and its checkpoint recorded |
| T5.4 | **Unit 5a, task 4: gpui and the elements**: B.9's unit-5a parts in step 3's outcome's form (A5 (b)): `ToolbarButtonStates`, `toolbar_button_states`, `toolbar_button` with its doc (without the 5b sentence), the import line, `toolbar_button_rest` with its doc, unit test, `GEOMETRY_NOTES` entry, its row in the README's geometry table and its entry in `CG/contract.rs`'s refinements where step 3's trial passed (B.9), the contract's sink and the interim `UNREAD_COLOURS` entry for `toolbar.button_checked_background` (and, where the trial failed, the `NoReceiver` entry for the outline), the README paragraph (naming `toolbar_button_rest` alone where the trial passed), `toolbar_button_takes_the_toolbars_states`; the showcase's switch of the six tool buttons to `variants::toolbar_button` (with `toolbar_button_rest` after `tool_button_box`'s refinement where the trial passed), `info::buttons::native_toolbar` and its three callers with their `Option<&ResolvedTheme>` parameter and the tests that call them, `icon_button`'s edge note and the reworded doc of the showcase test `the_basic_pages_icon_buttons_claim_no_edge` ("no edge claimed at rest; the hover edge goes through `edge_claim`", B.9) where the trial passed, the `HOW` lines for `"chrome.toolbar."` and `"basic.icons."` (no `"basic.icons.edit"` line yet), `a_tool_buttons_box_keeps_its_size_with_the_outline` where the trial passed; B.10's first bullet (the six elements' new leaves). | `implement` | the header; B.6, B.9; B.10's first bullet; C.1's row; A5 (a), (b), (f) | (1) seen to fail: `toolbar_button_takes_the_toolbars_states` and, where the trial passed, `toolbar_button_rest_keeps_the_padding_box`; where the trial passed, `a_tool_buttons_box_keeps_its_size_with_the_outline` is a guard ("kept", §2), run before the switch and after it, both `ok` quoted; the elements' new leaves, added first, fail the gpui showcase's `every_listed_leaf_has_a_route` and the iced showcase's `the_element_list_parses_and_every_leaf_has_a_route` where their routes are missing; (2) they pass: `cargo test -p native-theme-gpui --features native,watch --lib -- variants:: geometry:: the_showcase_exercises_every_builder every_geometry_builder_has_a_note every_how_entry_is_shown every_colour_leaf_reaches_a_sink_or_is_named every_colour_claim_is_read_at_the_line_it_cites every_edge_colour_is_claimed_through_edge_claim every_prose_citation_still_exists --nocapture`, its count quoted, each named `ok`; the crate's legs and two clippy legs as T4.4; the three showcases' whole legs in the runner environment by the agent's own runs, their result lines quoted, `every_tool_buttons_size_note_follows_the_stated_padding`, `every_listed_leaf_has_a_route` and the gpui test above named `ok`, and iced's route tests (`the_element_list_parses_and_every_leaf_has_a_route`, `a_route_names_a_widget_only_where_the_state_draws_it`) named `ok` (no egui showcase test requires a route per listed leaf, B.10); each `HOW` text checked with `cite2-core.py` and `prose-core.py`, their output in the report; (3) no pinned line moved (`HEAD` is T5.3's work-in-progress commit): in `git diff HEAD -U0 -- connectors/native-theme-gpui/src/variants.rs`, one hunk replaces the line A5 (f) found for `use crate::Native;` by one line and every other hunk's old-side start is at or after the line that ends `hover_border` (or `switch_color`, where the trial failed): the new items before `#[cfg(test)]`, and `toolbar_button_takes_the_toolbars_states` inside the test module after it; every hunk of `git diff HEAD -U0 -- connectors/native-theme-gpui/src/geometry.rs` starts after `button_checked`'s end (A5 (f)); `git diff HEAD -U0 -- connectors/native-theme-gpui/src/colors.rs` prints nothing; `every_colour_claim_is_read_at_the_line_it_cites` passes; (4) `python3 scripts/check_connector_parity.py` and `python3 scripts/check_showcase_parity.py --check-list` pass; (5) no line citation re-pointed but a gated claim (S6); (6) `cargo fmt --all --check` passes, the change is staged and its checkpoint recorded |
| U5a | **Unit 5a's commit**, as U4a's, once T5.4's work-in-progress commit is made: the squash to `<unit base>` (`<A5 base>`, U4b's commit), the gate, the commit. | orchestrator | §2; §5 | as U4a, with the union of T5.1 to T5.4's lists, the gate in the mode §4's count gives (`--quick`, the second code unit after the warm-up gate), the GPUI Fast legs, and every review of 5a closed with its fixes in the commit. Commit: `feat: a toolbar button's states: the fields, kde-breeze, adwaita and windows-11, the KDE and Windows readers, and the three connectors and showcases` (the body as U4a's, with step 3's outcome named). Not pushed before MG (§4, *CI*), so K3's CI run is the first to read the Windows reader's tests (B.5a's three, in *Test Windows (windows)* and *Test Windows (windows, watch, system-icons)*, `ci.yml:222-223`). A fix that only Windows compiles has as its done-criterion T5.1's two cross-target commands and the next CI run's Windows legs |
| MB | **The "before" dumps** (5a task 5, B.10), on U5a's commit, before U5b is written, in §3.1's order: the pre-check, the three pre-builds, the measurement's long run started (it takes the asset lock and the cargo slot, then writes `HELD`), the announcement and "measure now" once `HELD` exists, then `GO` on the maintainer's answer: one measurement of three blocks, iced, egui, gpui, under one lock. The orchestrator writes the "before" part of §10 into the record. No commit. | orchestrator; the maintainer for the desktop | B.10; §3.1 | the pre-check exits 0; each block's `<kind>.STATUS` reads 0 and its log's tail shows nine entries done; `target/measure-5b/before/` holds 27 dumps (*MB's count* below prints 27); the record holds each block's time, its reruns and the entries whose icon set fell back (the showcase's stderr naming the installed themes) |
| U5b | **Unit 5b, task 1: the Edit button**, in one commit, in two phases (*The icon's colour*, below), by one agent, from `<unit base>`, U5a's commit (MB's tree; MG comes only after MA and `rec`, §2.4, so both dumps stand on one base, rule 17); each phase ends in a work-in-progress commit (`wip(5b): phase 1`, made after the verdict and, where the icon does not follow, after the revert; `wip(5b): phase 2`), and the unit's commit is their squash (§2, *A checkpoint*): phase 1 is B.9's `geometry::toolbar_button_checked` with its doc, its `GEOMETRY_NOTES` entry and its unit tests (`toolbar_button_checked_letters_in_the_checked_text`, and the guard `every_preset_with_a_toolbar_outline_states_its_checked_fill` where step 3's trial passed), and the trial's test (decision 14); phase 2 the rest of C.1's 5b task 1: the two elements; the README row and the *Flat buttons* sentences of 5b; `toolbar_button`'s doc sentence and link; `toolbar_button_rest`'s "Apply it before `toolbar_button_checked`" where the trial passed; the contract's refinement sink in place of the interim entry; the three showcases' button, state and click (B.10's table, gpui's `IconButton` spec struct with its callers among them); Widget Info (B.9's checked claims, B.10's *Widget Info*); gpui's `HOW` lines under `"basic.icons.edit"`; iced's Edit arm, `Shown::basic_edit`, the `.edit` id, `ghost_icon`'s entry and the `ICON_BUTTONS` entry; egui's `PER_CALL_ROUTES` entry for Edit; B.10's three showcase tests; and, where the trial's verdict says the icon does not follow, decision 14's and B.9's branch: the caller colours the checked button's icon with `Icon::text_color` from `button.checked_text_color`, and the gpui README's *Flat buttons* paragraph gains B.9's sentence ("A checked toolbar button's icon does not take the refinement's text colour: …"). | `implement` | the header; B.8 to B.10's Edit parts; B.13.6, B.13.8; C.1's row; A5 (a), (b), (f); the record's step 3 outcome and, for phase 2, the trial's verdict | phase 1 as *The icon's colour* says; then (1) seen to fail: B.10's three showcase tests, `toolbar_button_checked_letters_in_the_checked_text` (seen in phase 1), and the elements, added first, failing `every_listed_leaf_has_a_route` and iced's route test before the routes; (2) they pass: the gpui `--lib` command of T5.4 (2) with `toolbar_button_checked` named, and, where the verdict kept it, the trial's test; where step 3's trial passed, the guard `every_preset_with_a_toolbar_outline_states_its_checked_fill`, run when written and again at the end, both `ok` quoted; `cargo test -p native-theme-iced --all-features --lib -- contract:: every_colour_leaf_reaches_a_row_or_is_named`; `cargo test -p native-theme-egui --all-features --lib -- install_tests:: every_leaf_has_exactly_one_row_and_every_row_is_well_formed`; the three crates' legs as T4.2 to T4.4; the three showcases' whole legs in the runner environment by the agent's own runs, their result lines quoted, `the_basic_edit_button_is_checked_and_switches` (gpui), `the_basic_edit_button_starts_checked_and_switches` (iced and egui), `every_listed_leaf_has_a_route`, `the_element_list_parses_and_every_leaf_has_a_route` and `a_route_names_a_widget_only_where_the_state_draws_it` named `ok`; each `HOW` text checked with `cite2-core.py` and `prose-core.py`; (3) no pinned line moved: the hunks of `git diff <unit base> -U0 -- connectors/native-theme-gpui/src/variants.rs` and of `connectors/native-theme-gpui/src/geometry.rs` as T5.4 (3) says, with no hunk at the import line (the doc sentence lies in `toolbar_button`'s doc, after the claimed lines; the builder after `button_checked`), `git diff <unit base> -U0 -- connectors/native-theme-gpui/src/colors.rs` prints nothing, the claim gate passes; (4) `python3 scripts/check_connector_parity.py` and `python3 scripts/check_showcase_parity.py --check-list` pass; (5) no line citation re-pointed but a gated claim (S6); (6) `cargo fmt --all --check` passes, the change is staged and phase 2's work-in-progress commit made; (7) R5b.1 and R5b.2 have read the two work-in-progress commits and every must-fix is in a work-in-progress commit of its own; then the squash, `cd <core worktree> && git reset --soft <unit base>`, the staged-tree check, and the gate passes in the mode §4's count gives (`--commit`: the third code unit after the warm-up gate and the release's last code unit before its documents and version units, rule 6), with the GPUI Fast legs (in the gate). Commit: `feat(showcases): a checked toolbar button, Edit, on the Basic page, with gpui's geometry::toolbar_button_checked` (the body names the trial's verdict) |
| R5b | **The reviews of U5b** (§5), on its work-in-progress commits before the squash and the gate, all closed with their fixes folded into the unit's commit before MA. | §5 | §5 | §5 |
| MA | **The "after" dumps and the comparison** (5b task 2, B.10), at U5b's last commit, its review fixes included, once the gate has passed there, in §3.1's order: the pre-check, the three pre-builds, the measurement's long run started (lock and slot, then `HELD`), the announcement and "measure now" once `HELD` exists, `GO` on the maintainer's answer, one measurement of three blocks under one lock; then `cd <core worktree> && python3 -I <record>/measure-5b/compare_layout.py <core worktree> target/measure-5b/before target/measure-5b/after` (the two folders relative to the worktree's root), and MA's state for K5 (rule 19): `cd <core worktree> && python3 -I <record>/measure-5b/after_state.py <core worktree> target/measure-5b/after <record>/measure-5b/after-state.json`, before K3's `rm -r target/measure-5b` (a re-taken "after" measurement writes it again). Exit 0: the record commit follows. Exit 1: a stop entry is B.10's stop (*The stop rule*; §7), the folders kept; a parity entry (*The wrap's parity*) is fixed by the showcase's author (`implement`, reviewed, §7's *A fix after the units*) and MA run again, or, where it is a toolkit's limit, excepted with its reason as §3.2 defines an exception, reviewed (R-exc, §5); the maintainer is told, not asked (rule 19). Exit 2: an input fault, fixed in the inputs, the comparison run again. | orchestrator; the maintainer for the desktop | B.10; §3.1 | the pre-check exits 0; each block's `<kind>.STATUS` 0; 27 dumps in `target/measure-5b/after/`; the comparison's printout and exit status in the record; `after_state.py` exits 0 and `<record>/measure-5b/after-state.json` holds 27 kind × entry records |
| rec | **The measurement record** (B.10, *The record*): §10 of this plan filled in B.10's form (the folders, the pre-check's results, each block's time, the reruns, the entries whose icon set fell back, the printout, with each wrap and what it moves), in one commit of this plan's file only, on top of U5b's last commit, its gate the five checkers and `--quick` (S1, a documents-only commit). Not pushed before MG; it goes with the release's first push (K3), after which `cd <core worktree> && rm -r target/measure-5b` (rule 19: the dumps stay until the release's first push; §10's shas stay valid, MG changing none). | orchestrator | B.10 | `git show --stat HEAD` lists this plan alone; the five checkers and `--quick` pass; `target/measure-5b` kept, its removal named in K3's record. Commit: `docs: the Edit button's layout measurement` |
| U5c | **Unit 5c, the documents**, once every review of step 5 (§5: R5.T1 to R5.T4, R5.trial, R5b) is closed with its fixes committed and `rec` made: B.11 but the rationale's status line and `docs/archive/` (K7's): the READMEs' counts (`soft.py`'s on the tree), step 5's CHANGELOG bullets in the `## [0.7.N] - <date>` section U4b opened (rule 14), the release's one section (V dates it): its *Added* bullets after A.12's, the egui bullet in the tree's egui form, the gpui bullet in step 3's outcome's form, and A.12's *Breaking Changes* bullet replaced by B.11's one bullet for both steps (S5, C.3), the ROADMAP section's second paragraph, added to the section U4b wrote (B.11: no section of its own), the backlog's edits, notes and four new `[1.0]` entries, `ROADMAP.md:264`'s recount, the first rationale's §7 row and §5.4 sentence, and the reply draft's edits (only while the reply is unposted, read again before U5c as U4b's row says, rule 16), each `<A \| B>` in step 3's outcome's form; the release placeholders, step 4's and step 5's alike, written as `0.7.N`. | `worker` | the header; B.11; the record's step 3 outcome, egui version and the maintainer's answers; A5 (a) to (c) | as U4b, with B.11's blocks; the closed or rewritten backlog texts as B.11 gives them; the release's section holds step 4's and step 5's entries under one set of headings and one *Breaking Changes* bullet, and the ROADMAP one section with both paragraphs, as A.12 and B.11 give them; the five checkers and `check_readme_versions.py` pass; the gate passes: `--quick`, a documents-only unit's, as the last unit before MG (rule 20: the pre-merge gated head, with no extra `--commit`, since U5b's covers the code beneath it). Commit: `docs: a toolbar button's states in the READMEs, the changelog, the roadmap, the first rationale and the backlog`. The record then holds the line "the lane's gated head for the next release: `<sha>` (U5c; `--quick` run `<record>/gates/U5c/run-<n>/`; reviews closed)", naming U5c's commit (or the commit of a fix after a red gate), the next release's `<lane-pred-head>`, which never waits for this release's MG, X or tag. Not pushed before MG |
| MG | **The merge with `<prev-tag>`** (rules 7, 17 to 22, 24, 25), once, after U5b and its reviews, MA, `rec` and U5c (the pre-merge gated head, rule 20), when `<prev-tag>` exists (until then the lane waits here), never while a trial's or a unit's edits are uncommitted. Its steps run in rule 21's order, **(1), (3), (6), (7), (4), then (5)** ((2) is superseded by S6: no citation is re-pointed at MG but a gate-read one in a conflict, (vi) below), with **no agent running cargo before step (5) but `cargo fmt --all --check`, which builds nothing** (a regeneration is the orchestrator's long run; VF's `implement` agent alone may run one `nice 19` `-p` build or clippy of the crate it rewrote, through `slot.sh`, rule 21). **(1) The merge**, in this order: (i) the tag, fetched only where `cd <core worktree> && git rev-parse -q --verify <prev-tag>` fails (`git fetch origin tag <prev-tag>`), and in any case the first field of `cd <core worktree> && git ls-remote origin 'refs/tags/<prev-tag>'` compared with `git rev-parse <prev-tag>` (the tag ref's own object on both sides, equal for a lightweight tag, as this project's release tags from v0.5.9 on are, and for an annotated one, as some older ones are; tags are shared by every worktree, and a release tag has been moved before); no line printed, or a different sha, is a stop; (ii) rule 18's ancestry check, `git merge-base --is-ancestor <lane-pred-head> <prev-tag>` exiting 0, else a stop; (iii) every untracked file the merge would overwrite (the paths both `cd <core worktree> && git ls-files --others --exclude-standard` and `cd <core worktree> && git diff --name-only --no-renames --diff-filter=A <lane-pred-head> <prev-tag>` list), each compared by `cd <core worktree> && git show <prev-tag>:<path>` piped to `cmp - <path>` (*MG's commands*, below the table), removed where equal, a stop otherwise; (iv) the merge, with the HEAD before it recorded (`git rev-parse HEAD`): `cd <core worktree> && git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>` (diff3, so that every conflicted hunk carries its base section; it never commits on its own, so the resolutions and the regeneration below always come before the merge commit, whether or not a conflict arose); (v) `Cargo.lock` and the version comparison: a conflicted manifest (`Cargo.toml`) is resolved first, by (vi)'s rules, before this item's cargo commands; where the merge left `Cargo.lock` conflicted, `<prev-tag>`'s side is taken (`git checkout --theirs Cargo.lock`) and this release's lock changes are re-applied, one `cargo update -p <name>@<theirs-version> --precise <ours-version>` (which resolves without building) per package this release changed, the commands derived from `git diff <base> <the HEAD recorded at (iv)> -- Cargo.lock` (a package this release added or removed has no `<theirs-version>`: `cargo metadata --format-version 1 > /dev/null` first resolves the missing or surplus entry without building or upgrading others, and the diff against `<prev-tag>`'s lock afterwards must show only this release's packages, else a stop), each through `<tools>/slot.sh <tools>/lowprio.sh`; then the version comparison (rule 19's read of `Cargo.lock`, which needs no gate, step (3)), here so that a stop wastes no build and its result tells whether the merged tree can build before VF; last, `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo metadata --locked --format-version 1 > /dev/null` passes (it builds nothing, and it catches a lock that merged cleanly but is inconsistent; rule 21 (1) item 5). (vi) **The resolution** (rule 21 (1)), the orchestrator's: it leaves out `Cargo.lock` ((v)) and the generated files ((vii)), never runs `git checkout --conflict=…` on a file it has touched (that would put the markers back over its resolution), and resolves every conflicted hunk by both sides' quoted texts, a citation like any other text (S6: an old line number in a conflict takes either side's text, both drift alike, and a new citation has none; where one side re-pointed a line a gate reads by number, S6's fourth gate or rule 25, that side wins; where both sides re-pointed the same one, the number is the base plus both sides' deltas, confirmed by the warm-up gate), each resolved hunk listed in the record (file, hunk, side or text), a resolution that changes a text's meaning a stop (§7). `CHANGELOG.md` puts this release's `## [0.7.N] - <date>` section first and `ROADMAP.md` this release's section after `<prev-tag>`'s (rule 14). A count or verdict tuple a test or gate checks (a leaf count, a verdict tuple) is the base plus both sides' deltas, never one side's; a prose count (a count stated in a sentence) takes `<prev-tag>`'s side and is named in the record (X recounts it). Step (1) then always ends the same way, conflicts or none: (vii) every generated file (`mapping.md`, `contract-leaves.toml`, every file a generator writes) regenerated by its command on the merged tree through the wrappers under the gate lock (`flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh …`, rule 2; always a long run, since at every MG `<prev-tag>`'s V has bumped the workspace crates in `Cargo.lock`, rule 21 (1) item 8; where the merged tree does not build before VF, the file is taken from the HEAD recorded at (iv), `git checkout HEAD -- <path>` during the merge, recorded as pending and regenerated in VF's code commit, or, where VF makes no commit, by the orchestrator in a commit of its own before the warm-up gate, or, where the tree builds only after a warm-up fix, in that fix's commit before the warm-up runs again, rule 12); (viii) `git add` of the resolved and regenerated files; (ix) the marker check (*MG's commands*, below the table, diff3's base marker included) printing nothing (no conflict marker staged, rule 21 (1)), a marker-like line that `<prev-tag>`'s own file has at that place not counted; (x) rule 21 (1)'s staged-path check: every path `cd <core worktree> && git diff --cached --name-only --no-renames` lists is in `cd <core worktree> && git diff --name-only --no-renames <lane-pred-head> <prev-tag>` or is a regenerated file named in the record, any other a stop (§7); then `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo fmt --all --check` passes (§4, *Formatted and clean*: the resolutions can edit `.rs` files; on a failure the orchestrator runs `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo fmt --all`, re-stages the files it changed, names the change in the record, and runs the check again, a file it changes outside the staged set being a stop (§7); R-MG reads that change); (xi) `git commit --no-edit --cleanup=strip`, `<merge>` recorded; (xii) the parents check: `git log -1 --format=%P <merge>` prints the HEAD recorded at (iv) (U5c's commit or a later fix) and `git rev-parse <prev-tag>^{commit}`, in that order; `grep -m1 '^## \[' CHANGELOG.md` prints this release's heading; `git status --porcelain --untracked-files=no` is clean. **(3) The version comparison and VF** (rules 19, 25): the comparison made at (1) (v) (`Cargo.lock`'s versions of the crates P2 lists at the HEAD recorded at (iv) and at `<merge>`, a read that needs no gate); a version outside P2's accepted set (egui 0.36.2 or 0.37.x, a patch release of the others) is P2's stop (§7); within it, VF (its row), its commits made here, reviewed within R-MG and covered by step (5); a source the new version needs that `~/.cargo/registry/src/` lacks is fetched by the orchestrator with `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo fetch`, which builds nothing. **(6) The path check** (rule 22 (b)): every token of this specification and this plan starting `docs/`, its `:N` suffix and `#…` fragment stripped, checked with `cd <core worktree> && git ls-files --error-unmatch <path>`, and every relative `](…)` link of the two documents checked from `docs/` (`cd <core worktree>/docs && git ls-files --error-unmatch <link>`), brace sets expanded; exempt: paths cited "on <branch>" or "at <sha>", paths a unit of this release creates, placeholders (`<…>`, `…`), glob or regex patterns, URLs, `crate::` links and quoted texts of other files; one that moved to `docs/archive/` is re-pointed by a `worker` in a documents-only commit, made before the warm-up gate and covered by it (rule 22 (b)); one gone otherwise is a stop. **(7) The survived checks** (rules 17, 21, 16), reading HEAD as it stands at (7), after (3) and (6); the blocks VF rewrote in (3) are expected differences from `<merge>^1`, listed in the record from VF's commits and each compared with the specification's form for the new version, and a survived check never undoes VF (rule 21 (7)): this release's blocks on the merged tree compared with their state at `<merge>^1` (a script in `<record>/mg-survived/` that reads each block of the release's units, T4.1 to U5c, by its anchor in `git show <merge>^1:<path>` and on the merged tree; not U4b's or U5c's own comparison scripts, whose blocks a later unit may have changed; a prose count resolved to `<prev-tag>`'s side in step (1) is listed for X's recount, not compared): the texts this release replaced absent, the remaining anchors and this release's new texts present; the anchors of the later tasks found on the merged tree (V's `## [0.7.N] - <date>` heading in `CHANGELOG.md`, every status line K7 sets: the rationale's, the specification's and this plan's, and the archive row's place in `docs/archive/README.md`); A4 (c) and (g)'s and A5 (c) and (g)'s searches (counts' bases, struct literals) run again on the merged tree, each count the base plus both sides' deltas; the reply's posted state read again (`cd <core worktree> && gh issue view 4 --comments`, rule 16): posted since U4b or U5c, a documents-only commit reverts those units' draft edits, made before the warm-up gate and covered by it, and the maintainer is told; a fix the survived checks need is a commit made here, covered by the warm-up gate. **(4) R-MG** (§5): a `review` of `git show --remerge-diff <merge>` with the record's list of resolved hunks, and of every commit of steps (3), (6) and (7), wherever a file was resolved or reformatted in (1) or written by (7), except Markdown that no test parses (so `.rs`, manifests, workflows, configuration files, a TOML a test reads, a script, `docs/platform-facts.md`, a README, which is compiled as doctests and parsed by tests, and `connectors/native-theme-egui/src/mapping.md`, which `mapping_doc.rs` reads, are in), where (1) (v) resolved `Cargo.lock`, or where VF committed anything (VF's review is always within R-MG; rule 21 (4)); the reviewer is `review`, or `review-hard` where those commits change a hard part of a core (a resolver, validation, a runtime, concurrency, `unsafe`); a must-fix is a new commit by the author of the commit it fixes, covered by step (5). **(5) The warm-up gate** (§4), last, `--commit`, always with the GPUI Fast legs, since the merge brings the other side's code (and `<prev-tag>`'s version bump) (rule 21 (5)), covering every commit of steps (1), (3), (4), (6) and (7), which have no gate of their own; a red one goes as rule 12 says (§7), and its fix is gated by the warm-up run again. Each fix after the warm-up is a commit with its own gate. Where P2 made the branch from `<prev-tag>`, there is no MG. | orchestrator; a `worker` for the path check's re-points; `review` or `review-hard` (R-MG, VF's review within it); VF's agents | rules 7, 12, 16 to 22, 24, 25; §2 | the remote tag ref's object (`git ls-remote`'s first field) equals `git rev-parse <prev-tag>`; the ancestry check exits 0; the parents, the `CHANGELOG.md` heading and the clean status as above; `git log --format=%s <lane-pred-head>..HEAD` shows no `wip(` subject; the resolutions, the `Cargo.lock` resolution where there was one, the version comparison and VF's outcome, R-MG's verdict (VF's commits within it), the path check's list and the survived checks are in the record; the warm-up gate passes (`<record>/gates/warmup-<n>/`), its time in the record |
| VF | **The version forms after MG** (rule 25), MG's step (3), right after the version comparison and before the warm-up gate. For every crate whose locked version changed, normally egui 0.36.2 → 0.37 (v0.7.2, released before this release but on this branch only from MG): a `worker` re-reads every upstream line the specification cites in that crate in the new version (`~/.cargo/registry/src/`), each with its line there or "not found" (§7's quoted-text rule); an `implement` agent rewrites the forms the units took on the older version, in code, tests and the code texts that name a version: for egui, T4.2's (A.9's egui facts and texts), T5.2's (B.7: `Role::Toolbar`'s doc, the README's accessor paragraph, the role's comment), and U5b's egui Edit (B.10: `demo::tool_button`'s doc's `<egui 0.37 \| egui 0.36.2>` part, `WidgetInfo::selected`'s first argument), and for any other changed crate the units' texts that cite it, as the `worker`'s reading lists them; and a `worker` the documents' forms, U5c's egui CHANGELOG bullet (B.11's two forms) and any other; each form is rewritten only where the content (behaviour, value, API) changed in the new version, never to update a version-pinned line citation that still holds (S6), except a line a gate reads by number (the gpui showcase's `HOW` texts and colour claims, read via `CITED_CRATES`, and the fallback-stack test's gpui-pre lines), which is re-pointed where the new version moved it (rule 25). The agents run no cargo but `cargo fmt --all --check`, which builds nothing, except that VF's `implement` agent may run one `nice 19` `-p` build or clippy of the crate it rewrote, through `<tools>/slot.sh` (rule 21); a source the new version needs that the registry lacks is fetched by the orchestrator (MG's step (3)); each commit is reviewed within R-MG (§5; rules 21, 25), a must-fix made by the author of the commit it fixes, and covered by MG's warm-up gate, with no gate of its own; a generated file MG's step (1) left pending is regenerated in VF's code commit by the orchestrator's long run; where VF makes no commit, by the orchestrator in a commit of its own before the warm-up gate; and where the tree builds only after a warm-up fix, in that fix's commit before the warm-up runs again (rules 12, 21 (1)). Where nothing changed, the record says "none needed" and VF makes no commit. | `worker`, `implement`, `worker` | the header; A.9, B.7, B.10, B.11's egui texts; MG's version comparison | the upstream reading in `<record>/vf/`; each form equals the specification's text for the new version (extracted and compared, no cargo); R-MG closed for VF's commits; then MG's warm-up gate, which runs the egui tests of T4.2 (2), T5.2 (2) and U5b (2) and the egui showcase's legs, passes. Commits: `fix(egui): the egui 0.37 forms of the toolbar role, the check box and combo box roles and the Edit button` and `docs: the egui 0.37 form of the changelog's egui bullet` |
| X | **The recount** (rule 17), after MG and VF, on the merged tree, with `<base>` the branch's base (`<lane-pred-head>`); with no MG there is no X (rule 17: nothing was merged, and U5c already ran the five checkers): every stated count in every file either side changed (the union of `git diff --name-only <base> <merge>^1` and `git diff --name-only <base> <prev-tag>`), outside the pinned documents (as the batch's Appendix lists them: `docs/archive/`, `docs/evidence/`, `docs/issues/`, the dated `docs/todo_v0.7.1_*` and `docs/todo_v0.7.x_*` documents, `docs/todo.md`'s dated block), recounted on the merged tree, the prose counts MG's step (1) took from `<prev-tag>`'s side among them; then the five documentation checkers. No citation is read or re-pointed (S6). | `worker` | rule 17; MG's record | each recounted count with its old and new value and its file in the record, and in the commit body where one changed; formatted and clean; the five checkers pass; the gate passes (in the gate): the five checkers and `--quick` (rule 6: a unit between the last code unit and V). Commit (where a count changed): `docs: the counts the merge with <prev-tag> changed` |
| V | **The version** (the release mechanics of `a3c10475`): the workspace version and the inter-crate requirements become `0.7.N`, the lock follows, the `<date>` of `## [0.7.N] - <date>` (rule 14: U4b wrote the heading) is filled, and the compare link goes at the top of the link list. No status-line sentence (A.12's and B.11's are K7's, with the archive's path). Under D0 (b), a minor version, every README's `native-theme*` requirement also moves to the new `major.minor`, as `scripts/check_readme_versions.py` requires. Before the close-out's first step (rule 22 (c)): *V's grep* (below the table) prints nothing (code, tests and rustdoc that name this release's own planning documents name their archived path, since K7 moves them after the asset run); a hit is re-pointed to `docs/archive/…` in V's commit. | orchestrator | §4's version check; rule 22 (c) | §4's version check; `python3 scripts/check_readme_versions.py` passes; the stamped-path grep prints nothing; the five checkers pass; `--commit` passes (rule 6: a lock change, its `Cargo.lock` listing the new versions). Commit: `chore: v0.7.N version` |
| K1 to K8 | **The asset run and the close-out, with the archive**, §2.5, after MG. | orchestrator; the maintainer for the capture window | C.2; B.10; A.12; B.11 | §2.5 |

*MB's count* (and MA's, with `after`), from the core worktree:

```bash
cd <core worktree> && ls target/measure-5b/before | grep -c '\.json$'
```

*V's grep* (rule 22 (c)), from the core worktree, over the stamped paths, read
on the branch and written literally in place of `<stamped paths>`: those listed in
`scripts/update_provenance.sh`'s `SOURCE_PATHS=(` array (`:39` at `0fa28027`) and
every connector's source paths in `scripts/update_compatibility.sh`'s
`source_paths()` (`:69-80` at `0fa28027`: each connector's `Cargo.toml`, `src`,
`examples` and `tests`, and for egui also `mapping.toml` and the egui widgets
crate's `Cargo.toml` and `src`):

```bash
cd <core worktree> && git grep -noE '(archive/)?todo_v0\.7\.x_kde-open-items-steps4-5-[a-z{},.-]*' -- <stamped paths> | grep -v ':archive/'
```

The stem matches the brace form `steps4-5-{spec,plan}` too, and the filter works
per match, not per line (rule 22 (c)): a line that names one document by its
archived path and the other by its present one still prints.

*MG's commands* (rule 21 (1)), from the core worktree. The untracked-overwrite
check of MG step (1) (iii), for each path both lists name (it prints nothing and
exits 0 where the files are equal):

```bash
cd <core worktree> && git show <prev-tag>:<path> | cmp - <path>
```

The marker check of (ix), after `git add` and before the staged-path check, diff3's
base marker included; it prints nothing (no conflict marker staged), a printed
line that `<prev-tag>`'s own file has at that place not counted (rule 21 (1)):

```bash
cd <core worktree> && git diff --cached -U0 | grep -E '^\+(<{7}|\|{7}|={7}|>{7})( |$)'
```

**The icon's colour** (decision 14, B.9, B.13.8). Whether a gpui Button's icon
takes the selected state's text colour that `geometry::toolbar_button_checked`
sets was not read, and the specification decides how a test reads it (decision 14;
B.9, from "Whether a Button's icon takes the refinement's `text_color`"; B.13.8;
cited by section, as those texts were amended on 2026-11-15): not from the painted
frame, since gpui-pre 0.3.12's test API gives painted quads and underlines only
(`GP/window.rs:3112-3120`), but by a probe child that records
`window.text_style().color` during paint, the colour gpui-component 0.9.1 paints
an `Icon` in where the icon states none (`GC/icon.rs:260-266`, `:213-214`). Where
this section and the specification differ, the specification holds; U5b's brief
gives decision 14's, B.9's and B.13.8's text. The plan's part is the run, the
review and the verdict:

- **The test** is the specification's: `a_checked_toolbar_buttons_icon_takes_the_checked_text`,
  a lib test in `connectors/native-theme-gpui/src/widgets/tests.rs` (no runner
  environment), on windows-11 light and dark (where `button.checked_text_color`,
  #ffffff / #000000, differs from the variant's one foreground), a selected
  Button built with `variants::toolbar_button(cx)` and
  `geometry::toolbar_button_checked(n)` as the Edit button is built; it records and
  prints every case first, one line per case (preset, mode, recorded colour, checked
  text), then asserts once that each recorded colour is `button.checked_text_color`,
  the message naming each case that differs (B.9).
- **The run**: the agent runs it in the foreground,
  `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo test -p native-theme-gpui --features native,watch --lib widgets::tests::a_checked_toolbar_buttons_icon_takes_the_checked_text -- --nocapture`,
  and reports the result line and the printout, leaving phase 1 unstaged; the
  orchestrator saves the phase's diff, numbered with the run,
  `cd <core worktree> && git diff HEAD > <record>/trial/trial-<n>.patch`, the
  agent's result line and printout going to `<record>/trial/run-<n>.txt`, and runs
  the same test on GPUI Fast as a long run, in a new `<record>/trial/fast-<n>/`
  made first:
  `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh cargo test -p native-theme-gpui --features gpui-fast,native,watch --lib widgets::tests::a_checked_toolbar_buttons_icon_takes_the_checked_text -- --nocapture > <record>/trial/fast-<n>/LOG 2>&1; echo $? > <record>/trial/fast-<n>/STATUS`;
  each repeated run takes the next `<n>`.
- **R5.trial** (§5) reads the run's patch and both outputs before the verdict.
- **The verdict**, written into the record by the orchestrator: **the icon
  follows** where both result lines show `1 passed`, both printouts show every
  case the test names (windows-11 light and dark, each with the recorded colour
  and the checked text), each recorded colour equal to the checked text, and
  R5.trial is closed with nothing serious; **it does not follow** where both runs
  show `1 failed`, both printouts show every case, the one assertion's message
  names each case that differs with its preset, mode and two colours, and R5.trial
  is closed with nothing serious (one run passing and the other failing is the stop
  below, not this verdict). Any other result decides nothing: a panic outside that
  assertion, a build error,
  a printout that lacks a case, a message that names no mode and two colours, or
  a result line with `0 passed; 0 failed` (the filter found no test); the test or the code
  is fixed within the form above (a probe child of the Button so built, the same
  presets and refinements, nothing else changed), R5.trial reads the fix's
  patch, and both runs are made again. **Passed on one backend and failed on
  the other**: a stop (§7); the specification decides neither.
- **Where the icon follows**, the test stays as it is and phase 2 gives the icon no
  colour of its own. **Where it does not**, the trial's test is reverted before
  phase 2, as decision 14 says, with the record holding the patch and the
  outputs:
  `cd <core worktree> && git restore --staged --worktree --source=HEAD -- connectors/native-theme-gpui/src/widgets/tests.rs`
  (`HEAD` is `<unit base>`; no other part of U5b edits that file), after which
  `git diff HEAD --stat` lists that file no more; phase 2 then takes the
  specification's branch: the caller colours the checked button's icon with
  `Icon::text_color` from `button.checked_text_color` (an OS icon theme's raster
  or SVG keeps its own colours, B.9), no test reads that colour, and the gpui
  README's *Flat buttons* paragraph says so in B.9's sentence. The maintainer is
  told the verdict.
- **Between the phases**: after the verdict (and, where the icon does not follow,
  the revert), the agent stages phase 1's paths by name (`git add -- <path> …`,
  each path of phase 1 named, never `git add -A`) and reports; the orchestrator
  makes `wip(5b): phase 1` from the staged tree (§2, *A checkpoint*); the agent
  then continues with phase 2, the same agent with the verdict, and reports once
  at its end.

### 2.5 The release's close-out and the archive

One close-out for both steps (S5), after MG (S2), X (where there is an MG, rule 17) and V, and only when every
review of §5 is closed and its fixes committed. In this order; each step starts
only when the one before it holds, but K4 and K5 run beside K3's CI (S4).

1. **K1.** `cargo audit` passes with the repository's `.cargo/audit.toml`.
2. **K2.** `scripts/update_compatibility.sh run` passes, a long run (§4) started as
   `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/update_compatibility.sh run > <record>/assets/compat-<n>/LOG 2>&1; echo $? > <record>/assets/compat-<n>/STATUS`
   (rule 2); it
   leaves `docs/COMPATIBILITY.toml` and the connector READMEs changed and
   uncommitted until K7. An upstream release that moved, or an advisory, stops
   the run (§7).
3. **K3.** The push, once K2 has passed (S4: before the full check, which runs
   beside CI): the push check of §2, then
   `git push origin v0.7.N-rc1:v0.7.N-rc1`, the release's first push (every unit's
   commit, `rec`, MG's merge, X and V with it); after it,
   `cd <core worktree> && rm -r target/measure-5b` (rule 19). CI runs on pull requests and `main`
   only (`.github/workflows/ci.yml:6-10`), so at this first push the orchestrator
   opens the draft pull request to `main` (approved 2026-11-15, §8 item 3; rule 9),
   its title and description shown to the maintainer first. The run is green, and its
   Windows legs show the Windows reader's tests: the logs of the jobs *Test
   Windows (windows)* and *Test Windows (windows, watch, system-icons)*
   (`ci.yml:222-223`) each name `build_theme_accent_shades_light_mode`,
   `build_theme_accent_shades_dark_mode` and
   `under_high_contrast_the_reader_maps_the_system_colours` with `ok`
   (`cd <core worktree> && gh run view <id> --log > <record>/ci/run-<id>.log`, the
   lines found there by `grep` and copied into the record; a name missing is a
   stop, §7). The cost S4 accepts: this run can be red, and email the maintainer,
   for a fault K4 would have shown first; a red run is §4's *CI* case.
4. **K4**, while K3's CI runs (S4): the full `scripts/check_release.sh`, a long
   run started as
   `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh > <record>/assets/full-<n>/LOG 2>&1; echo $? > <record>/assets/full-<n>/STATUS`
   (rule 2), fails on the provenance stamp alone (K2 renewed the other). Its warnings
   are named in the record. Two are expected: the known `cargo outdated` warning,
   zbus excluded and named as excluded (`0fa28027`), whose list of outdated names
   and versions is whatever it lists at the run and is recorded whole; and, under
   D0 (a), a 0.7.x number, the semver step (soft), expected to name
   `CheckboxTheme`'s and `ComboBoxTheme`'s new field and `ToolbarTheme`'s four and
   nothing else (the release's one break, A.12 and B.11). Under D0 (b) the semver
   step passes and only the `cargo outdated` warning is expected; K8's expected
   warnings follow the same answer. Any other failure or warning is a stop (§7).
5. **K5**, while K3's CI runs (it commits nothing): the three showcases built
   for release with the capture scripts' own commands
   (`scripts/capture_window.sh:440`, with each script's features), through both
   wrappers under `gate.lock` (`flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh cargo build …`,
   rule 2), each a long run (§4) in a new `<record>/assets/build-<kind>-<n>/`; and a rehearsal of the parity step's layout
   half on a private X server, with no `kde-inhibit` and through `slot.sh` alone:
   each release showcase started with `--dump-layout`, with the presets the
   capture scripts use (P1's copy `<record>/inputs/rehearsal.sh`, whose `W=` names
   `.claude/worktrees/v071` and is set to the core worktree in a copy of its own,
   `<record>/assets/rehearsal.sh`, whose `ENTRIES` are first checked against
   `scripts/capture_window.sh`'s `SCREENSHOT_ENTRIES` on the branch, `:352` at
   `0fa28027`, and brought into line where they differ, and then extended by B.10's
   nine layout entries (*ENTRIES*: the union of its six, kde-breeze, material-3 and
   catppuccin-mocha in both modes, and B.10's nine, so that adwaita, windows-11,
   macos-monterey, macos-tahoe and ios-13 light join them, each with B.10's icon
   set), the union written into that copy and the difference in the record; it
   dumps the Basic page (`--tab basic`), runs the built binaries and no cargo, and
   takes its output folder as its one argument, which receives the dumps, the logs
   and `summary.txt`, whose last line is `DONE`), then the comparator on the dumps.
   The rehearsal holds the shared Xvfb lock (rule 24):

   ```bash
   cd <core worktree> && flock <locks>/xvfb97.lock <tools>/slot.sh bash <record>/assets/rehearsal.sh <record>/assets/rehearsal-<n>/ > <record>/assets/rehearsal-<n>.log 2>&1; echo $? > <record>/assets/rehearsal-<n>.STATUS
   cd <core worktree> && python3 scripts/check_showcase_parity.py <record>/assets/rehearsal-<n>/ > <record>/assets/rehearsal-<n>/parity.out 2>&1; echo $? > <record>/assets/rehearsal-<n>/parity.STATUS
   ```

   The first line is a long run, into a new `rehearsal-<n>/` (§6, *Run folders*);
   the second reads its dumps once its `summary.txt` ends in `DONE` and holds no
   `dump=no` line (`grep -c 'dump=no' <record>/assets/rehearsal-<n>/summary.txt`
   prints 0; a `dump=no` line fails the rehearsal, its log read and the run
   repeated or the fault fixed), and the
   comparator's exit status goes into the record with `parity.out`. It compares
   geometry only and runs without a window manager, so a difference it shows,
   before the capture, is reported while the capture goes ahead (rule 19: a
   parity fault is a difference the asset run's comparator shows after the
   capture, K6). The rehearsal is one at a time across both lanes
   (rule 24: its private X display `:97`, the `xvfb97.lock` above, so a lane waits
   mechanically). **The measurement's after-only checks on the merged tree** (rule
   19; B.10): what MG brought is not measured again, and the parity comparator
   counts an element that is not visible as ruled out, not as a difference
   (`scripts/check_showcase_parity.py:1077-1079`) and skips a property fewer than
   two kinds give (`:1080-1081`), so K5 also runs
   `cd <core worktree> && python3 -I <record>/measure-5b/after_only.py <core worktree> <record>/assets/rehearsal-<n>/ <record>/assets/rehearsal.sh <record>/measure-5b/after-state.json`
   (M5's (5)) on the rehearsal's dumps, relative to MA's state, not absolute (the
   page scrolls, so some elements lie outside its visible area by design): the
   rehearsal copy's `ENTRIES` must hold all nine of the measurement's entries and
   not be empty, and a dump must exist for each of the three kinds × each entry
   (exit 2 otherwise); for each kind × entry of the nine, an id MA saw present
   and now missing, or MA saw wholly inside and now not wholly inside, fails; for
   every entry, the nine and any other, `basic.icons.edit` outside wherever
   `basic.icons.copy` is inside fails (item 5), and so do the three kinds' Edit
   not wrapped alike (item 6's parity); of an entry outside the nine, only the
   visible-area findings are reported, not stopped on (rule 19). The edges are compared as
   logical edges, each dump snapped at its own `scale`; MA's dumps are the
   desktop's and the rehearsal's are Xvfb's, so the two scales are printed and
   the record notes that their agreement is not verified. Exit 0 is a pass; 1 with
   an element pushed out of view (an id MA saw present or inside now missing or not
   inside, or Edit outside where Copy is inside) is a stop (§7), the maintainer
   asked, as at MA (rule 19); 1 with a wrap-parity failure only is not a question
   and not a stop: it is reported, named to the maintainer, and the capture goes
   ahead (rule 19: a difference K5's rehearsal shows before the capture is
   reported; the fault is decided on the asset run's comparator after the capture,
   K6; a failure of an entry MA already excepted is reported as excepted, §3.2);
   2 an input fault (a missing or malformed dump or state, an id the
   list lacks, an incomplete or empty entry list), fixed in the inputs and run
   again. Its printout and exit status go into the record. A layout fix after MG (§7) is read here and named to the maintainer.
6. **K6.** When K3's CI is green, K4 is done with only its expected failure and
   warnings (rule 9's "green": CI green, and the full check's only failure the
   provenance stamp, which only the asset run refreshes), and K5 is done, and no other release's asset run or capture window is
   running (one at a time across both lanes, rule 24: the asset run and its
   capture window hold the shared `assets.lock`, so a lane waits mechanically), the
   orchestrator takes the lock (below) and then asks the maintainer for "capture
   now": the desktop unlocked and untouched for 4 to 6 minutes, announced before
   it starts. The
   asset script runs once (it needs HEAD pushed and the stamped paths clean), in
   the core worktree, as a long run:
   `cd <core worktree> && flock <locks>/assets.lock flock <locks>/gate.lock <tools>/slot.sh sh -c 'touch <record>/assets/run-<n>/HELD; until [ -e <record>/assets/run-<n>/GO ]; do sleep 5; done; kde-inhibit --power --screenSaver --notifications bash -c "CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16 bash scripts/generate_assets_release.sh > <record>/assets/run-<n>/LOG 2>&1; echo \$? > <record>/assets/run-<n>/STATUS"'; echo $? > <record>/assets/run-<n>/OUTER.STATUS`,
   in a new `run-<n>` folder made first (§6, *Run folders*), every placeholder
   written as its literal absolute path (§6, *Cargo*). The asset lock, the gate
   lock (rule 2) and the cargo slot are all taken before the maintainer is asked
   (rule 24), and
   `kde-inhibit` starts only after `GO`, inside the slot, never wrapping the waits
   for the locks, the slot or the maintainer (as the steps 2–3 plan's K6 has it): the orchestrator starts this long run, waits
   for `HELD` with the Monitor tool's until-loop
   `until [ -e <record>/assets/run-<n>/HELD ] || [ -s <record>/assets/run-<n>/OUTER.STATUS ]; do sleep 10; done`
   (an `OUTER.STATUS` written first is a failure before `HELD`, read from it and
   the run folder, and nobody is asked), asks for "capture now" once `HELD`
   exists, and starts the
   capture with `touch <record>/assets/run-<n>/GO` on the maintainer's answer. It holds a slot but does
   not run under `lowprio.sh`: the desktop is reserved for it and the captures are
   timing-sensitive. `kde-inhibit` hides the script's exit code, so the
   orchestrator reads `STATUS` and the log's tail, never the wrapper's status. The
   orchestrator waits for "Theme-switching GIFs generated"
   (`scripts/generate_assets_release.sh:172`) with the Monitor tool's until-loop on
   the files (never a `pgrep -f` loop):

   ```bash
   until grep -q 'Theme-switching GIFs generated' <record>/assets/run-<n>/LOG || [ -s <record>/assets/run-<n>/STATUS ]; do sleep 10; done
   ```

   When the line is there it tells the maintainer that the desktop is free (a
   `STATUS` written first is the run's end, read as such); the script then waits
   for the CI screenshots and ends by running `update_compatibility.sh run`
   (`scripts/generate_assets_release.sh:278`), whose builds keep the machine busy
   until `STATUS` is written, which the maintainer is told with the desktop's
   release. No agent edits the tree until the run has reported. A stop commits
   nothing. Then the orchestrator opens the new screenshots beside the previous
   release's. The expectation is C.2's for both steps: step 4 changes nothing at
   rest (gpui paints no border quad whose colour is transparent, A.10), and by
   B.10's *Screenshots* every capture and theme-switching frame of the Basic page
   in the three showcases changes (the Edit button, checked; under kde-breeze its
   checked fill; the groups below the icon row moved where MA's printout shows a
   wrap), and no other page's capture changes (the toolbar's buttons at rest:
   iced's transparent quad paints no pixel, C.2). A capture that differs otherwise
   goes to the maintainer, and K7 waits. A parity difference is not a question for
   the maintainer (rule 19; the project's rule: a connector that ignores theme data is a bug
   to fix): one this work's changes explain is fixed (§7, *A fix after the units*)
   and the asset run made again; one that is a toolkit's limit gets B.10's
   `[[parity.group]]` exception "with its reason, only for a difference the
   comparison then shows", reviewed (§5); the maintainer is told in either case,
   and K7 waits until it is settled.
7. **K7.** One commit: the assets, both stamps, the READMEs' lines, the
   changelog heading's date set to the commit's day where it differs from V's,
   this plan's §9 copied from the record, and the archive, the last task (B.11):
   A.12's and B.11's status-line sentences go into the rationale's status line,
   both with the archive's path and the commit's date (step 4's link written as
   B.11 says it is re-pointed: `../archive/todo_v0.7.x_kde-open-items-steps4-5-{spec,plan}.md`), and both naming
   `0.7.N`; this
   specification and this plan move to `docs/archive/` with their names (`git mv`,
   so that `git diff --cached --name-status` lists them as renames, `R`, beside the
   asset files); `docs/archive/README.md` gains B.11's row; every path mention and
   link of the two documents in every tracked file outside `docs/archive/` is
   rewritten for the new place (rule 22 (a)), found with
   `cd <core worktree> && git grep -n 'kde-open-items-steps4-5' -- ':!docs/archive'`
   over every tracked file (the pattern matches the brace form
   `kde-open-items-steps4-5-{spec,plan}` too; B.11's list: the
   rationale's status lines, the later documents' mentions (the caret's, this
   release's lane successor, which reads this plan's gated head and record; steps 6
   and 7's; the widget icons'; the gate-speed specification's), as their texts
   stand at the close-out, since those documents were still being edited on
   2026-11-15, where they are tracked outside `docs/archive/` by then; this
   specification's and this plan's
   own links: `issues/…` → `../issues/…`, `archive/todo_v0.7.x_kde-open-items-steps2-3-spec.md`
   → `todo_v0.7.x_kde-open-items-steps2-3-spec.md`; the steps 2–3 specification's
   3H.6 stays as written); the status lines of the specification and this plan say
   implemented, with the release's commit range; §9 holds the record up to this
   commit. **The batch's rationale** (rule 22 (a): the batch's last release
   archives it, and every plan names that task as conditional on being last): where
   this release is the batch's last in the maintainer's order (not so under D10's
   order, *The open choices*, where the widget icons, the caret and steps 6 and 7
   come later), K7 also archives
   `docs/todo_v0.7.1_batch-speed-rationale.md` (`git mv` to `docs/archive/`, its
   status line saying implemented with the batch's releases, its row in
   `docs/archive/README.md`, and its path mentions rewritten by the same rule 22 (a)
   pass and check, its stem `todo_v0\.7\.1_batch-speed-rationale` added to the
   pattern); where it is not the last, it leaves the rationale where it is, and the
   batch's last release archives it. New files: the asset files the script wrote that `HEAD` has not (each
   named from `git status --porcelain --untracked-files=all` of the asset folders,
   in the record); the staged-tree check of §2 holds before the commit. The gate is
   the five checkers, the checks below and the orchestrator's reading of the diff;
   K8 checks the rest. **Check:**
   `cd <core worktree> && git grep -noE '(archive/)?todo_v0\.7\.x_kde-open-items-steps4-5-[a-z{},]+\.md' -- ':!docs/archive' | grep -v ':archive/'`
   prints only the exempt mentions (a citation dated "at <sha>", a path inside
   another document's instructions that names its own future target), each named
   in the commit body. The pattern matches the document form only (ending in
   `.md`, the brace form `{spec,plan}.md` included), so this plan's record folder,
   `<scratch-keep>/todo_v0.7.x_kde-open-items-steps4-5-plan/`, which later plans read
   (the caret its rule-20 line; steps 6 and 7 its `<record>/measure-5b/…`), is not
   matched: a path of a plan's
   record or work folder does not move and is left as written, named in the commit
   body (rule 22 (a)). Tested on 2026-11-15 with `grep -rnoE` over `docs/` (the
   documents being untracked then): against the earlier form `[a-z{},.-]*`, the
   matches this form leaves out are all record-folder paths. Rule 22
   (a): the check filters per match, not per line; every
   path mention of the two documents in every tracked file outside `docs/archive/`
   is rewritten after the move, other plans' instructions among them; left are a
   mention already pointing into `docs/archive/` in any relative form or as an
   absolute URL, a citation dated "at <sha>", a path inside another document's
   instructions that names its own future target, and a record or work folder's
   path; a document whose
   header dates its lines is exempt only for its line citations, never for a path
   mention. The rewritten mentions go in the commit body;
   `cd <core worktree> && git grep -n 'kde-open-items-steps4-5' -- docs/archive`
   lists the archived documents' mentions, each left as written (an archived
   document is not rewritten, B.11) and listed in the commit body; a script in
   the record lists every relative `](…)` target that does not exist, in the moved
   documents (their links now resolving from `docs/archive/`, rule 22 (a)) and in
   every file whose links the commit rewrote, with rule 22 (b)'s exemptions (paths
   cited "on <branch>" or "at <sha>", paths a unit of this release creates,
   placeholders, glob or regex patterns, URLs, `crate::` links and quoted texts
   of other files), and lists none;
   the archive index's check passes: the inline script `scripts/check_release.sh`
   runs after `print_section "Archive index"` (found by that text; `:1033-1058` at
   `0fa28027`), its Python body saved as `<record>/archive-index.py` and run from
   the worktree's root, `cd <core worktree> && python3 -I <record>/archive-index.py`,
   exit 0 (every `docs/archive/*.md` named in `docs/archive/README.md`, brace sets
   expanded). No later-links file is written (rule 22): an
   untracked later document is checked by its own plan's path check after its MG
   (rule 22 (b)). The release's commit range written into the status lines is
   `git log <prev-tag>..HEAD` (rule 21: it keeps this release's commits and leaves
   out what `<prev-tag>` brought). Pushed with the same refspec,
   the push check of §2 passing.
8. **K8.** The full `scripts/check_release.sh` at K7's commit, a long run in a
   new `<record>/assets/full-<n>/`, started as K4's (rule 2's `gate.lock`),
   reports no failure (K4's expected warnings still appear, named in the
   report), and K7's CI run is green; both are reported to the maintainer (the
   summary line, the run's id and result), not written into the plan.

Then the orchestrator runs `/home/tibi/.claude/scripts/sweep-target.sh <core worktree>`
and reports the sizes it prints; `v071` and its build folder stay (§1), and so do
`kde-b`'s for the caret, the lane's next release, which that report and the project status note name. **Merging the pull request,
tagging, publishing, the GitHub release, the pull request's title and
descriptions and posting the reply to the reporter are the maintainer's** and are
not part of this plan.

## 3. The measurement (step 5, B.10)

### 3.1 How a block runs

- **The announcement**, before MB's measurement and again before MA's: the
  maintainer is asked for "measure now", the desktop unlocked and untouched (a
  locked screen gives no frames; the runs take the focus), for one window that
  holds the measurement's three blocks, with B.10's bounds for one block (at least
  9 × 6 s, at most 9 × 46 s of sleeps and bounded waits) and, from MB's measured
  block times, the expected time of MA's window.
- **The pre-check** before each tree's measurement:
  `python3 -I <record>/measure-5b/precheck.py <core worktree>`, exit 0.
- **The pre-builds**, all three before the measurement's lock is taken (so the
  window holds no build), through both wrappers, each a long run (§4), with
  B.10's commands, each under the gate lock (rule 2): `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh cargo build -p native-theme-iced --example showcase-iced --release --features iced_aw`;
  egui `-p native-theme-egui --example showcase-egui --release --all-features`;
  gpui `-p native-theme-gpui --example showcase-gpui --release`. Nothing changes
  the tree between them and the blocks, so `cargo run`'s freshness check in each
  block finds them current.
- **The measurement**, one capture session and one long run per tree (§4; the
  batch's rule 5), its three blocks in sequence under one asset lock, the gate
  lock (rule 2) and one cargo slot, so that the other lane cannot take the desktop
  between blocks (rule 24; the plan's choice of the same lock for the measurement):

  ```bash
  cd <core worktree> && flock <locks>/assets.lock flock <locks>/gate.lock <tools>/slot.sh sh -c 'touch <record>/measure-5b/<tree>/run-<n>/HELD; until [ -e <record>/measure-5b/<tree>/run-<n>/GO ]; do sleep 5; done; kde-inhibit --power --screenSaver --notifications bash -c "for kind in <kinds>; do CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16 bash <record>/measure-5b/block.sh \$kind <tree> > <record>/measure-5b/<tree>/run-<n>/\$kind.LOG 2>&1; s=\$?; echo \$s > <record>/measure-5b/<tree>/run-<n>/\$kind.STATUS; [ \$s -eq 0 ] || break; done; echo end > <record>/measure-5b/<tree>/run-<n>/STATUS"'; echo $? > <record>/measure-5b/<tree>/run-<n>/OUTER.STATUS
  ```

  The wait for `HELD` (the Monitor tool's until-loop; rule 24), which a failure
  before `HELD` (`OUTER.STATUS` written first) ends:

  ```bash
  until [ -e <record>/measure-5b/<tree>/run-<n>/HELD ] || [ -s <record>/measure-5b/<tree>/run-<n>/OUTER.STATUS ]; do sleep 10; done
  ```

  in a new `run-<n>` folder made first (a rerun takes the next `<n>`; §6, *Run
  folders*), every placeholder its literal absolute path, `<kinds>` being
  `iced egui gpui` (MB's order) for a first run. The HELD/GO pair is per
  measurement, not per block: the orchestrator starts the long run, asks for
  "measure now" once `HELD` exists, and on the maintainer's answer starts all three
  blocks with `touch <record>/measure-5b/<tree>/run-<n>/GO`. It does not run under
  `lowprio.sh`. The orchestrator waits for the exit notice, then reads each
  `<kind>.STATUS` and its log's tail. A failed block (`<kind>.STATUS` not 0: the
  identity check, a window that lost the focus, a locked screen) ends the run
  there; the failed block and the blocks after it are run again whole in a new
  run with those kinds as `<kinds>` (its own HELD/GO, the maintainer asked again),
  and the record names the rerun.
- **The folders** `target/measure-5b/before` and `…/after` stay until the
  release's first push (K3, after MG; rule 19), and while a stop waits for the
  maintainer.

### 3.2 What it decides

The comparison's exit status (MA): 0 goes on, and its "moved" lines are the list
of each wrap and what it moves, which K6's captures then show; 1 with a stop entry
stops the work before `rec` and MG (B.10's stop rule: an element pushed out of view,
Edit's place being a design choice), and the maintainer decides; 1 with a parity
entry only is not a question (rule 19): the showcase's author fixes the difference and MA runs
again, or a toolkit's limit is excepted with its reason, reviewed, and the
maintainer is told (B.10, *The wrap's parity*). **An exception** is a record entry
naming the entry, the kinds that differ and the toolkit's limit with its source; the
orchestrator reviews it by size (R-exc, §5), `rec`'s §10 names it, and K5 reports
the same entry's parity failure as excepted, with no new exception; a fix after it that touches the Basic page or a showcase's layout means
MA's blocks again before MG, with §10 updated in a new documents-only commit, and
K5's after-only check after MG (§7).

## 4. Gates

- **While working**, an agent runs the tests of the module it changes. **Before
  it stages**, it runs its task's targeted done-criterion commands once, in the
  foreground, through the wrappers (§6), each expected to finish well within 10
  minutes; it runs no `--commit`, full check, asset run or GPUI Fast leg (§2,
  *Who runs what*).
- **The cadence** (the batch's speed decision S1,
  `docs/todo_v0.7.1_batch-speed-rationale.md`; C.1, *The cadence*), on the staged
  change of every commit, as a long run by the orchestrator, who then commits
  exactly the gated tree with the author's message (the batch's rule: the
  orchestrator commits, so that the commit is the tree the gate passed):
  - during a unit, the agent's targeted commands only; a unit's tasks are not
    gated one by one: each ends in a local work-in-progress commit (§2, *A
    checkpoint*), never pushed, and the unit's squashed commit takes the gate;
  - at a unit's end, `scripts/check_release.sh --quick`
    (`docs/todo_v0.7.1_gate-speed-spec.md` says what it runs on 0.7.x, never fewer
    checks than a task's own targeted commands), a long run until its time is
    measured, its first run timed into the record: U4a and U5a (as the count below
    gives them), the documents-only units and commits (U4b, `rec`, U5c), and X (rule 6),
    the documents-only ones and X with the five checkers too;
  - `scripts/check_release.sh --commit` on every warm-up gate (P0's or P2's, MG's),
    after a lock or toolchain change (V, whose `Cargo.lock` lists the workspace
    crates' new versions), on every third code unit counted from the last
    `--commit`, and on the release's last code unit before its documents and
    version units (rule 6); a documents unit between that last code unit and V
    takes `--quick`, rule 20's gated head included (rule 6).
    A unit is a code unit when its change
    reaches a crate under `--quick`'s scope rule
    (`docs/todo_v0.7.1_gate-speed-spec.md`, A.3: its own package's files, files a
    package includes, the manifests, the lock and the Cargo configuration, and any
    file outside the packages not in `QUICK_UNREAD`, `docs/*.toml` and `scripts/`
    among them, which reaches every crate, that spec's A.3 table);
    `docs/platform-facts.md` counts as code (rule 11). A documents unit stays one
    when its change is only comments, rustdoc prose, READMEs and Markdown
    documents, even where a crate includes them: it runs `--quick`, whose scope
    (A.3) then reaches those crates, so their doctests run, and it is not counted
    toward "every third" or "the release's last code unit". With the warm-up gate
    as the first `--commit`, U4a and U5a are the first and second code units and
    U5b the third and the last before the documents and the version, so U5b runs
    `--commit`, which covers the code beneath U5c, the gated head, whose gate is
    `--quick` (rule 20). Any `--commit`
    restarts the count (rule 11): a `--commit`-gated fix between units counts as
    the last `--commit`, a `--quick`-gated fix does not count toward the next
    third, and the orchestrator writes each unit's mode into the record before its
    gate, from the count as it then stands. After MG's warm-up gate, VF's commits
    are covered by it (rule 21), X takes `--quick` (rule 6), and V runs `--commit`
    for its lock change. No unit is named structural (rule 24: none changes a manifest,
    `Cargo.lock`, a build script, the toolchain or the gate scripts before V; new
    model fields are no reason on their own); 4a, 5a and 5b are gated with the GPUI
    Fast legs whatever their mode;
  - U5c, the last unit before MG, is the lane's pre-merge gated head (rule 20)
    and takes a documents-only unit's gate, the five checkers and `--quick`, with
    no extra `--commit`;
  - every gate run, whichever its mode, starts as
    `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh …`
    (rule 2), so that no two gates of any lane run at once;
  - the full run in the close-out (K4, K8), unchanged.

  The marks, where no `--commit`-gated fix falls between (one restarts the count as
  above, and the orchestrator records each unit's mode before its gate):

  | Task | Kind | Gate |
  |---|---|---|
  | P2 (or P0) | warm-up | `--commit` |
  | U4a | code (first) | `--quick`, with the GPUI Fast legs |
  | U4b | documents only | the five checkers and `--quick` |
  | U5a | code (second) | `--quick`, with the GPUI Fast legs |
  | U5b | code (third, and the last code unit before the documents and the version) | `--commit`, with the GPUI Fast legs |
  | `rec` | documents only | the five checkers and `--quick` |
  | U5c | documents only, the last unit before MG: the pre-merge gated head (rule 20) | the five checkers and `--quick` |
  | MG's steps (1), (3), (6), (7) and (4): the merge, VF's commits, the orchestrator's regeneration of a pending generated file where VF makes no commit, the path check's re-points, a reply revert or fix the survived checks need, an R-MG must-fix | covered by the warm-up gate, no gate of their own (rule 21) | — |
  | MG's warm-up gate, step (5), last | warm-up after the merge (restarts the count); every commit of MG's other steps is covered by it | `--commit`, always with the GPUI Fast legs (rule 21 (5)) |
  | a fix of a red warm-up gate, with a pending generated file regenerated in it where the tree builds only after that fix (rules 12, 21 (1)) | as rule 12 says | the warm-up gate run again, `--commit`, with the GPUI Fast legs as above |
  | X | the recount, between MG's warm-up and V (rule 6) | the five checkers and `--quick` |
  | V | the version: the manifests and the lock (a lock change, rule 6) | `--commit` |
  | a documents-only fix | documents only | the five checkers and `--quick` |
  | a code fix | as the unit whose files it touches; `--commit` where that unit's was | as that unit |

  Where no fix falls between, that is four `--commit` gates (P2's or P0's warm-up,
  U5b, MG's warm-up and V) and six `--quick` ones (U4a, U4b, U5a, `rec`, U5c and
  X), beside the two full runs (K4, K8); with no MG, three `--commit` and five
  `--quick`, since there is neither MG's warm-up nor X (rule 17).

  The cost the maintainer accepted (S1): a commit between two `--commit` runs is
  proven by `--quick` only; a failure the next `--commit` finds is fixed by a new
  commit, never by rewriting a pushed one, and the record names the commit that
  brought it. K7 takes the five checkers, its own checks and the
  orchestrator's reading of the diff; K8 runs the full check after it. What
  `--commit` and `--quick` skip and this work needs runs in the same gate: the GPUI
  Fast legs (§2), in U4a's, U5a's and U5b's gates, a gpui fix's and MG's warm-up
  where §2 says. `--commit`'s
  cross-target checks run in it; a run that skipped one, or whose
  runner-environment legs did not run where §2 says they must (exit 75), does not
  count.
- **Formatted and clean.** The script runs `cargo fmt --all` in place
  (`scripts/check_release.sh:39-41`, `:518`) and only reports a dirty tree, while
  CI's `cargo fmt --all --check` fails the push. So every task that edits a `.rs`
  file runs `cargo fmt --all --check` before it stages, and so does MG step (1)
  before its merge commit, since its resolutions can edit `.rs` files (`cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo fmt --all --check`,
  which builds nothing, rule 21; on a failure the orchestrator runs
  `cargo fmt --all`, re-stages the files it changed, names the change in the
  record, and runs the check again, a change outside the staged set being a stop,
  and R-MG reads that change; rule 21 (1) item 10); its
  gate runs on the
  staged change; `git status --porcelain --untracked-files=no` prints the same
  before and after the gate, and nothing after the commit (a work-in-progress
  commit included). The exceptions are U5b's unstaged phase 1 until the verdict,
  K2's uncommitted files until K7, and the asset run's outputs after K6 until the
  asset commit (a fix there and its unstaged exceptions go as rule 15 says, the
  exceptions named in the record). A reformat the gate makes outside a task's
  own files is not folded into that task's commit: it becomes a formatting-only
  commit of its own, which the orchestrator reads.
- **After an asset run** (the exception's end). A fix the asset run's step needs
  goes as rule 15 (batch-speed rationale, Appendix) says, its unstaged exceptions
  named in the record. For a fix that touches a stamped path this plan adds its
  own step, the restore, since rule 15 keeps no outputs there: before such a fix
  made after the asset script has run (a stop at K6), the orchestrator restores the
  script's outputs to `HEAD`, `git restore --staged --worktree --` on the paths
  the script's closing line names (`scripts/generate_assets_release.sh:297`:
  `docs/assets/PROVENANCE.toml`, `docs/COMPATIBILITY.toml`,
  `native-theme/docs/assets/`, `connectors/native-theme-*/docs/assets/`,
  `connectors/native-theme-*/README.md`), and removes the new untracked asset
  files under those folders (listed by
  `git status --porcelain --untracked-files=all -- <those folders>` first, each
  named in the record); then `git status --porcelain --untracked-files=no` prints
  nothing, the fix is made, and the close-out goes on as rule 15 (batch-speed
  rationale, Appendix) says.
- **One merge.** The release has one KDE lane (§1); its one merge is MG's with
  `<prev-tag>`, gated by its warm-up gate; a gate that fails on the branch after a
  fix commit is fixed by the author of the change it points at, on the branch, and
  reviewed again (§5).
- **The documentation checkers** (no cargo): `scripts/check_platform_facts_rows.py`,
  `check_section_refs.py`, `check_preset_citations.py`,
  `check_wrong_safety_nets.py`, `check_todo_citations.py`, from the worktree's
  root, each with no finding.
- **The two release stamps** are stale from the release's first code commit on
  and are no task's done-criterion before K8.
- **The minimum supported Rust version** is checked by CI alone. The briefs say:
  no language or library feature newer than the edited crate's own
  `rust-version`, as it stands on the branch.
- **The version (V):** in `cargo metadata`, the seven crates with
  `version.workspace = true` (`native-theme`, `native-theme-build`,
  `native-theme-derive` and the four connectors) report the release's version,
  and the three `tools/` members keep `0.0.0`; `git diff` of `Cargo.lock` changes
  only the workspace crates' versions; under D0 (b) every README's `native-theme*`
  requirement names the new `major.minor`.
- **The full `scripts/check_release.sh`**: at K4, beside K3's CI (S4; one
  failure, the provenance stamp, and the expected warnings), and at K8 (no
  failure; the expected warnings still appear, and are named in the report).
- **Long runs.** The Bash tool kills a foreground command at 10 minutes. Every run
  not known to finish within that is the orchestrator's, as a long run: the gate
  (`--commit`: 5 to 12 minutes a commit, 29 minutes cold, in the batch's earlier
  measurements; `--quick`: not measured until its first run here), the GPUI Fast legs and the trial's GPUI Fast run, the full check
  (about 23 minutes, §11), the asset script (30 to 37 minutes),
  `update_compatibility.sh run`, the measurement's pre-builds and blocks, the
  release builds of K5, and a first build in a new worktree. A long run
  is started with `run_in_background`, from the core worktree
  (`cd <core worktree> && …`), through the wrappers, a gate run under
  `gate.lock` too (rule 2; §6, *Cargo*), or in the capture form of §6, *Cargo*, its log and a `STATUS` file at absolute paths in a new subfolder of
  `<record>` for that run (§6, *Run folders*) (`… > <log> 2>&1; echo $? > <STATUS>`);
  the orchestrator waits for the exit notice, then reads `STATUS` and the log's
  tail. A wait on a condition inside a long run (K6's
  "Theme-switching GIFs generated") uses the Monitor tool with an until-loop on a
  file condition, never a `pgrep -f` loop, which matches itself. The first
  `--commit` run (P0's or P2's) and the first `--quick` run are timed and their
  times written into the record, the first with
  whether it ran cold (a new worktree, an empty build folder) or warm (the reused
  worktree's build folder).
- **The warm-up gate.** An agent's targeted commands build the feature sets the
  gate's test legs build (gpui `native,watch`, `scripts/check_release.sh:667`;
  iced `iced_aw,watch,native`, `:739`; egui `--all-features`, `:762`), so before an
  agent that runs cargo is dispatched the orchestrator runs
  `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh --commit`
  on the clean tree (rule 2), as a long run in its own
  folder (`<record>/gates/warmup-<n>/`), wherever the worktree is new, `Cargo.lock`
  changed since that folder's last gate, or the rustup hook reported a changed
  toolchain; then those builds are warm and an agent's commands stay within the
  Bash tool's limit. P0's or P2's gate serves (P2's where there is no P0), as it
  runs before any agent that runs cargo (A4 runs none); MG runs one after the
  merge. **The feature sets the gate does not build**, which a task's
  targeted commands name, would be cold builds: `cargo test -p native-theme --features kde`
  (CI's leg, `ci.yml:212`; T4.1, T5.1), the iced connector with `--all-features`
  (A.8's regeneration command; T4.3, T5.3, U5b) and the egui connector's
  `--all-features --lib` test binary where the gate's `--all-features` leg has not
  built it. So after each warm-up gate the orchestrator builds them once as long
  runs in `<record>/gates/warmup-<n>/` (`cargo test -p native-theme --features kde --no-run`,
  `cargo test -p native-theme-iced --all-features --no-run`,
  `cargo test -p native-theme-egui --all-features --no-run`, each through the
  wrappers under the gate lock, `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh cargo …`,
  rule 2), and any other cold build an agent would start (a feature set no
  earlier run built) is run the same way first.
- **A targeted command killed at 10 minutes** is reported by the agent with the
  command; the orchestrator runs it as a long run under the gate lock (rule 2;
  its own folder under `<record>/gates/<task>/`), then continues the same agent with the `STATUS` and
  the log's tail.
- **CI** runs only once a pull request is open. **Nothing is pushed before MG**
  (S2, rule 17), so that no commit of this release reaches `origin` before it
  holds the released predecessor; and since MG comes after the last code unit, the
  release's first push is K3's (S4), after the compatibility run, carrying every
  commit before it; K7 pushes again. Each push with an explicit refspec,
  `git push origin v0.7.N-rc1:v0.7.N-rc1`, and only after §2's push check (no
  `wip(` subject in `git log --format=%s <remote branch>..HEAD`). The orchestrator watches each
  run in the background. A red run emails the maintainer; no push is made that is expected
  to be red. If one is red, the orchestrator tells the maintainer what failed and
  why, and the failing change's author fixes it (reviewed as in §5) as the lane's
  next commit, gated as the task it fixes; in the close-out, rule 15 (batch-speed
  rationale, Appendix) decides how it goes on.

## 5. Reviews

Every change to behaviour is reviewed before the push that rests on it: 4a's
tasks before U4a's commit, 5a's before U5a's commit (and so before MB), U5b's
before its commit (and so before MA), and every one before its step's documents
unit, whose texts state what it
does. The reviewer gets the commit (a unit task's work-in-progress commit, with
the unit's base named, or a commit range), the specification's header and parts,
never the author's report; it reads `git show <sha>` from the core worktree's
repository, not a working tree, and runs no cargo; R5.trial reads
`<record>/trial/trial-<n>.patch` and the two outputs of run `<n>` (phase 1 is
uncommitted until the verdict). A review runs while the lane's next task works,
but R5.trial, which the verdict waits for, and R5b's, which U5b's squash and so
MA wait for. A must-fix is a new change by the task's author (the same agent,
continued), staged after the task then running in the lane has made its
work-in-progress commit and before the lane's next task starts (inside a unit: a
work-in-progress commit of its own, `wip(<unit>): <task> fix <k>`, folded into the
unit's commit by the squash; after a commit: a commit of its own, gated as in
§2), with the task's done-criterion run again as it applies to a fix, and
reviewed by the same reviewer. For a fix, the done-criterion is: the tests the
task names pass (each `ok` quoted), the counts, the pinned-line checks against the
fix's own base (`git diff HEAD -U0`, `HEAD` being the previous work-in-progress
commit or the last commit), the gated claims the fix re-points (none other, S6),
`cargo fmt --all --check`, the staged-tree check and, after a commit, the gate;
not "seen to fail", which a fix of a passing test cannot repeat; a new test the
fix adds is seen to fail as any new test. A second must-fix on the same point
falls under §7's take-over rule. Every reviewer, the orchestrator included, also
checks that no new code transcribes an LGPL or GPL source (§6, *The licence
rule*).

"By size": a change of up to a few hundred changed lines an Opus agent wrote is
reviewed by the orchestrator, a larger one by a `review` agent; the orchestrator
reads `git show --stat <sha>` (or `git diff --stat` of a range) and says which it chose.

| Review | Change | Reviewer | Why |
|---|---|---|---|
| R4.T1 | `wip(4a): T4.1` and its fixes | by size (expected the orchestrator) | two fields with their registry keys and rules, preset lines extracted and compared, two reader writes, tests that name every new leaf, two backlog clauses |
| R4.T2 | `wip(4a): T4.2` and its fixes | `review` | the strokes of the check box's and combo box's roles and the checked cell against A.4 and A.9, the mapping rows' `when`s against T2's and T10c's rules, the contract arms, the four changed tests and the sentinel |
| R4.T3 | `wip(4a): T4.3` and its fixes | `review` | `CheckFills` per state of A.8's table, `pick_list`'s statuses, the contract rows, the leaves file against A.8's expected differences |
| R4.T45 | `wip(4a): T4.4` and `wip(4a): T4.5` with their fixes, together | `review` | the one group-hover style that carries both the fill and the edge, the claims re-pointed with the moved lines, the overlay and its group, the `edge_claim` widths, the sentinel tests, `painted_edges`; the two patches meet at the claims and the `HOW` lines |
| R5.T1 | `wip(5a): T5.1` and its fixes | by size (expected `review`) | the twenty preset values against B.3's table and arithmetic, two readers' writes (the KDE reader's alpha constant, the Windows reader's two writes in an `unsafe` exception file, no `unsafe` added), the presets test |
| R5.T2 | `wip(5a): T5.2` and its fixes | `review` | the predicate's change and its one call, the rows without `when` against T2's rule, the «toolbar» contract arm's fallback, the tests over `PRESETS`, the tool button's per-state stroke width, the parity exceptions' reasons |
| R5.T3 | `wip(5a): T5.3` and its fixes | `review` | `styles::toolbar_button` per status of B.8's table (the checked states of decision 5 and 11), the contract and pairs, the leaves file, the showcase's routes, `drawn_by` and the oracle |
| R5.T4 | `wip(5a): T5.4` and its fixes | `review` | the variant's states and its fallbacks, `toolbar_button_rest`'s padding less the width floored at 0, its README geometry row and its `CG/contract.rs` refinements entry (where the trial passed), the interim `UNREAD_COLOURS` entry's cited line, the info functions' new parameter with every caller, the `HOW` lines, the pinned lines of `variants.rs` and `geometry.rs` |
| R5.trial | U5b's phase 1 patch of each run and its two outputs, before the verdict | `review` | the verdict decides whether the showcase colours its icon; where the icon does not follow the test is reverted unseen: the reviewer checks that the probe is a child of the Button, rendered in its selected state with the refinements in the showcase's order, reads what `Icon::render` reads (`GC/icon.rs:260-266`, decision 14), and runs on presets whose two colours differ |
| R5b.1 | U5b: gpui's builder, contract sink, README and doc sentences, the gpui showcase's Edit, `IconButton` and its callers, Widget Info, the `HOW` lines, and the verdict's branch | `review` | the checked refinement after the rest one, the selected state's claims, the spec struct's callers, the hover outline that gpui cannot draw over a selected button (B.13.6) said where B.10 says it |
| R5b.2 | U5b: iced's and egui's Edit, routes and tests, the elements | `review`, beside R5b.1 | iced's Edit arm and `Shown`, the checked label's lines; egui's scope text colour set before the image is built (decision 12), the fill per call, `WidgetInfo`'s form for the tree's egui |
| R-exc | a wrap-parity exception (§3.2) at MA, before `rec` goes on | by size (expected the orchestrator) | the entry and the kinds that differ against the comparison's printout, and the toolkit's limit against its source (an upstream line at a version or a URL at a commit): an exception is for a toolkit's limit only, never for a showcase's fault, which is fixed |
| R-MG | MG's step (4), after steps (3), (6) and (7), wherever a file was resolved or reformatted in the merge (step (1)) or written by step (7), except Markdown that no test parses (`.rs`, manifests, workflows, configuration files, a TOML a test reads, a script, `docs/platform-facts.md`, a README, which is compiled as doctests and parsed by tests, and `connectors/native-theme-egui/src/mapping.md`, which `mapping_doc.rs` reads, are in), where step (1) resolved `Cargo.lock`, or where VF committed anything (VF's review is always within R-MG, rules 21, 25) | `review`, or `review-hard` where those commits change a hard part of a core (a resolver, validation, a runtime, concurrency, `unsafe`; rule 21 (4)), given `git show --remerge-diff <merge>` with the record's list of resolved hunks, and every commit of steps (3), (6) and (7) (rule 21) | a resolution in code decides which side's behaviour ships; each of VF's rewritten forms against the specification's text for the new version and the egui tests that check it; before MG's warm-up gate, running no cargo but `cargo fmt --all --check`. A must-fix is a new commit by the author of the commit it fixes, made before the warm-up gate and covered by it, with no gate of its own (rule 21) |
| — | P0, U4b, `rec`, U5c, X (its recount; where it edits a crate's file, by size, expected the orchestrator), V, K7, MG's resolutions in documents and its path-check re-points, a formatting-only commit | the checkers, then the orchestrator reads the diff | data and texts the specification fixed; the extraction scripts and the recount compare them |

No review of this release's own tasks is a `review-hard` one: neither step adds an
inheritance rule, a validation, a resolver change, concurrency or `unsafe` (the new
fields are soft options in `[no_inheritance]`, A.2, B.2). R-MG is `review-hard`
only where MG's commits change such a part (rule 21 (4)). At most one Fable agent
runs at a time (an `implement-hard` agent of §7's take-over, or a `review-hard`
R-MG).

## 6. Rules every agent brief carries

- **Inputs.** The brief gives the header and the task's parts; the parts are the
  whole task: nothing beyond them, no refactoring, no formatting churn. A needed
  change the parts do not name is reported, not made. Every edit is placed by its
  anchor (the quoted text, the key, the test's name), a count is the tree's value
  plus the specification's delta, and a citation is found by the text it cites,
  never by the number the specification gives; where a quoted text does not
  stand, the agent stops that edit and reports it, and §7's rule for a quoted
  text decides. The lines the gpui showcase's colour claims pin do not move but as
  A.10 moves and re-points them (*Lines that must not move*).
- **The worktree.** The brief names the core worktree by its absolute path; every
  command starts with `cd` into it or uses `git -C` and `--manifest-path`. The
  main checkout (another branch, `v0.8.0-rc1`'s work) and every other worktree
  have the same relative paths: nothing is read, run or written there, the parked
  branch's facts are read with `git show 78ee0efd:<path>` from the core worktree
  where a part needs them, and a check that passes elsewhere proves nothing. The
  report quotes `git -C <worktree> log -1 --oneline`.
- **The licence rule (the maintainer's D10, not the rationale's):** no code from
  an LGPL or GPL source. Breeze (`breezehelper.cpp`, `breezestyle.cpp`,
  `breezemetrics.h`, its colour schemes), KColorScheme, Qt (`qtoolbarlayout.cpp`)
  and libadwaita's stylesheets are read for facts only. A function such a source
  is cited for (the KDE reader's writes of A.6 and B.5, `FLAT_CHECKED_ALPHA`) is
  written only through the clean room: every implementer brief (T4.1 to T4.5,
  T5.1 to T5.4, U5b and any fix of their code) says that the implementer opens
  none of those sources; the specification's description and its tests (A.6's and
  B.5's values) are the clean room's description and vectors. gpui-component,
  gpui-base, gpui-pre and GPUI Fast are Apache-2.0, egui and iced MIT or
  Apache-2.0, microsoft-ui-xaml MIT and material-web Apache-2.0: read for facts; a
  port of their code would keep a file-level notice (none is planned).
- **Hooks.** Source files are edited with the Edit and Write tools only, with one
  exception: the generated `mapping.md` and `contract-leaves.toml`, written by
  their commands (at MG and VF too), and the lock's re-application at MG step (1)
  (v), written by `cargo update` (rule 21 (1)). The project's hooks block invented values
  and panic-prone code; a blocked edit is obeyed, never bypassed, and blocked
  content is never written another way (shell, `sed`, a here-document, a script).
  A block on content the specification requires stops the task (§7).
- **Code.** No `unwrap`, `expect`, `panic!`, panicking index or unchecked
  arithmetic in non-test code; no `unsafe`; no public item removed; no invented
  value. Code, tests and rustdoc that name one of this release's own planning
  documents that K7 archives (the specification, this plan) name its archived path,
  `docs/archive/<file name>`, since K7 moves them after the asset run (rule 22
  (c)); V's grep checks it. A new citation of a changing repository file, in code,
  a comment, a test's doc or a document, names the file (with a quoted phrase or a
  section where useful), never a line; an existing line citation is left as it is,
  and only a gated claim is re-pointed (S6; §2, *The citations*).
- **Cargo.** Every cargo command, and every script that runs cargo, goes through
  `<tools>/slot.sh <tools>/lowprio.sh <command>`, an agent's in the foreground
  (its targeted commands, §4), the orchestrator's long runs in the background
  (§4); an environment variable goes before the wrappers
  (`VAR=… <tools>/slot.sh <tools>/lowprio.sh cargo …`), since `lowprio.sh` ends in
  `exec "$@"`. Every gate run (`--quick`, `--commit`, the full run) starts as
  `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh …`
  (rule 2): `slot.sh` takes either of its two slots, so without the lock two
  gates could run at once. Every long run of the orchestrator's that runs cargo
  takes `<locks>/gate.lock` the same way (rule 2): every `check_release.sh` mode,
  every other command run as part of a gate (the GPUI Fast legs, §2), the
  feature-set builds after a warm-up gate (§4), a targeted command the
  orchestrator runs again as a long run (§4), MG's regenerations, the trial's
  GPUI Fast run (§2.4), the measurement's pre-builds and blocks (§3.1, with
  `assets.lock` as well), the compatibility run (K2), K5's release builds and the
  asset script (K6); an agent's targeted `-p` commands take only `slot.sh` and
  `lowprio.sh`. The capture sessions (the measurement's blocks, the asset runs) run
  under `kde-inhibit --power --screenSaver --notifications` and `slot.sh` but not
  `lowprio.sh`, their builds first through both wrappers; the cargo slot is taken
  with the asset lock, outside the session's `sh -c`, before `HELD` (rule 24: the
  asset run as `flock <locks>/assets.lock flock <locks>/gate.lock <tools>/slot.sh sh -c '…'`,
  the measurement the same way, `flock <locks>/assets.lock flock <locks>/gate.lock <tools>/slot.sh sh -c '…'`, §3.1); since `lowprio.sh` is
  what sets the build and test thread caps, the session's inner `bash -c` sets
  `CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16` before the script it runs (K6,
  §3.1), so that the cargo it starts keeps the gate's limit. A run on a
  private X server (the rehearsal) uses `slot.sh` and no `kde-inhibit`. In every
  command a placeholder (`<tools>`, `<record>`, `<core worktree>`) is written as
  its literal absolute path, never with `$HOME` or `~`; no command is wrapped
  across lines, and no command uses `\|` (an alternation is written with `-e` or
  `-E`). A run longer than 10 minutes is a long run (§4). `build-dir`,
  `CARGO_TARGET_DIR` and `--target-dir` are never set; `cargo clean` is never run.
  At most two cargo processes at once in total, which the two slots enforce; in
  this plan only the core lane builds.
- **Reach.** No `find /` and no other search of the whole file system. Reads and
  writes stay inside the core worktree, Cargo's home and build folder (the
  registry for upstream sources), the session's scratch folder for its own
  temporary files, `<record>`, `<locks>` (written by `slot.sh` alone),
  `<scratch-keep>/batch-2026-11-15/` (read only), and the lane predecessor's
  record folder, read only: steps 2–3's
  `<scratch-keep>/todo_v0.7.x_kde-open-items-steps2-3-plan/` for P1, P2, A4 and A5
  (the rule-20 line and step 3's outcome), read where its own plan's text names it
  (nothing in the main checkout). Nothing appears on the maintainer's desktop but the orchestrator's
  capture sessions.
- **Work.** A new test is written first and seen to fail (§2). A changelog entry
  is one physical line. The agent runs its targeted commands in the foreground,
  each expected to finish well within 10 minutes, and never `--commit`, the full
  check, an asset run or a GPUI Fast leg (§4); it stages its own paths by name,
  does not commit or push, adds no attribution line, and reports once: the staged
  files (`git diff --cached --stat`), its commit message or its part of the
  unit's, each done-criterion item it ran with its command and result line, every
  test it left red for a later task of the unit with that task's name, and
  everything it was unsure of. U5b's agent reports three times (§2.4, *The
  icon's colour*): the trial's run, phase 1 staged after the verdict (and any
  revert), and phase 2 at the unit's end. The orchestrator makes the task's work-in-progress commit (§2, *A
  checkpoint*) or runs the gate and commits with that message; the agent makes
  neither, and never resets, squashes or stashes; a red gate comes back to the
  agent, continued, with the log's failing part.
- **Something the maintainer must hear before a commit** (a value that differs
  from the specification's, a quoted text whose present form changes the edit's
  meaning, the trial's result): the agent says so in its report, and the
  orchestrator commits nothing of it before §7's stop rule lets the work go on.

**`<scratch-keep>`** is `/home/tibi/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/`.
**`<record>`** is `<scratch-keep>/todo_v0.7.x_kde-open-items-steps4-5-plan/`:
`record.md`, `tools/` (`<tools>`), `docs/`, `inputs/`, `anchors-4/`,
`u4b/`, `anchors-5/`, `measure-5b/` (with `tree-p2/`, M5's
extracted tree), `trial/`, `u5c/`, `assets/` (the close-out's runs), `ci/`,
`gates/`, `x/` (X's recount), `vf/` (VF's upstream reading),
`mg-survived/` (MG's survived-the-merge script), `undo/` (§2's dropped-work
patches, `dropped-<n>.patch`, and the untracked files in `dropped-<n>/`), and
K7's `archive-index.py`. A later plan of the batch may read
`measure-5b/precheck.py` and `measure-5b/compare_layout.py` from here (the steps
6–7 plan's measurement tools take them), so they stay after the work; the caret,
this release's lane successor, reads the record's rule-20 line. **`<locks>`** is `<scratch-keep>/slot-locks/`.
**`<cargo-build>`** is Cargo's build folder root. The briefs give absolute paths;
no committed file names them. **The inputs** copied by P1 into `<record>/inputs/`
from `<scratch-keep>/batch-2026-11-15/` (a session's scratchpad does not outlive
it): `steps45b/soft.py` (A.12's count), `steps45/precheck.py`,
`steps45b/compare_layout.py`, `steps45/make_test_pair.py` and
`steps45b/move_edit.py` (B.10's drafts), `steps23fix2/cite2.py` and
`steps23fix3/prose.py` (the transcriptions of the gpui showcase's citation
parsers) and `gates/rehearsal.sh` (K5). Three of them name a fixed root
in their text, read 2026-11-15: `cite2.py`'s `ROOTS` (`:65`: the v071 worktree and
gpui-component 0.9.1's registry folder), `prose.py`'s `G` (`:2`, gpui-component
0.9.1's registry folder) and `rehearsal.sh`'s `W=` (`:6`, the v071 worktree); the
others take the root as an argument or run in the current folder. Each is copied
unchanged into `<record>/inputs/`, the `cmp` of P1 holds for that copy, and the
copy that runs is a second one beside it with those roots set to the core
worktree and to the locked gpui-component version's folder (`cite2-core.py` and
`prose-core.py`, made by P2; `rehearsal.sh` as K5 says), the difference shown
by `diff` in the record; they are made again where MG's version check finds the
locked gpui-component version changed (rule 19).

**The record.** `<record>/record.md` holds the dated results, shas (the units'
bases and work-in-progress commits among them), counts, verdicts and answers as the work goes; only the orchestrator
writes it, agents report. It is copied into this plan's §9 only in the close-out
commit (K7, step 4's part and step 5's), so that no tracked file is
edited mid-work and the clean-tree checks hold. The specification asks one thing
of the repository mid-work: the measurement record, a commit of the plan's file
of its own (`rec`, §10).

**Run folders.** Every long run that can be repeated writes into a new subfolder
of its own, never into a folder an earlier run used: the gates into
`<record>/gates/<task>/run-<n>/` (with `LOG` and `STATUS` for `--quick` or `--commit`, the mode in the record, and
`FAST-TESTS.log`, `FAST-TESTS.STATUS`, `FAST-SHOWCASE.log` and
`FAST-SHOWCASE.STATUS` for the two GPUI Fast legs where the gate has them, and
`warmup-<n>/` for §4's warm-up gate), the trial's GPUI Fast run into
`<record>/trial/fast-<n>/`, the measurement blocks into
`<record>/measure-5b/<tree>/run-<n>/` (one per measurement run, with a
`<kind>.LOG` and `<kind>.STATUS` per block) and their pre-builds into
`<record>/measure-5b/<tree>-build-<kind>-<n>/`, and each kind of the close-out's
runs into a counter of its own under `<record>/assets/`: `full-<n>/`,
`compat-<n>/`, `build-<kind>-<n>/`, `rehearsal-<n>/`, the asset runs `run-<n>/`;
each `<n>` counts from 1, the folder is made with `mkdir` (which fails if it
exists), and the record names each folder with its run. So a `STATUS` file left
by an earlier run never ends a Monitor wait early and is never read as the new
run's result.

## 7. Limits and stop rules

- **The machine's limits are shared** (S3 as amended): the widget icons run in a
  lane of their own beside this one, the predecessor's close-out may still run
  (S2), and the caret, the KDE lane's next release, may start from this release's
  gated head while this release's merge and close-out run (rule 20), so the limits hold for all of them together: at
  most two cargo processes (one gate, and one `nice 19` targeted build in another
  lane), the heavy runs serialized by the shared `<locks>/gate.lock` (rule 2; §6, *Cargo*) and the
  cargo processes counted by `slot.sh`'s slots in the same folder; at most four
  agents (six where two run no cargo) across the lanes; one asset run and one
  measurement block at a time on the desktop; never two Fable agents. The
  orchestrator counts the other lanes' running agents and long runs, from their
  records, before it dispatches one here, and a dispatch that would pass a limit
  waits. Files both lanes edit (`CHANGELOG.md`, `ROADMAP.md`, `docs/todo.md`, the
  showcases) meet at the merge, by quoted text (S3; MG).
- **Agents at once in this lane:** within those limits, of which only this lane's
  task agent runs cargo here. The peaks: beside P0, A4, a `worker` without cargo
  (the lane's task is the orchestrator's); beside T4.2 to T4.5, the previous
  task's reviewer; before T5.1, A5 and M5, two `worker`s without cargo; beside
  T5.1 to T5.4, M5 where it has not finished (on its extracted tree) and, from
  T5.2 on, the previous task's reviewer: three; beside the trial's GPUI Fast run,
  R5.trial; R5b.1 and R5b.2 run together while the lane waits for them. At most
  one Fable agent at a time (§5), counted across the lanes.
- **A gate failed twice** by the same task is not dispatched a third time: the
  orchestrator takes the task over, re-specifies it, or sends it once as
  `implement-hard`, and says which. Inside a unit the new agent starts from the
  last reviewed work-in-progress commit, by §2's undo (the dropped work saved to
  `<record>/undo/dropped-<n>.patch` and the untracked files P2's list lacks to
  `<record>/undo/dropped-<n>/` and removed, then `git reset --hard <wip sha>`;
  `HEAD` equal to `<wip sha>` and `git status --porcelain --untracked-files=all`
  printing exactly P2's untracked list, less those P0 commits, after it). For a task whose code a licensed source is cited for
  (T4.1 and T5.1, the reader parts), the take-over is always a fresh
  `implement-hard` agent, which has read none of those sources, never the
  orchestrator (§6, *The licence rule*).
- **The specification's stops**, each with nothing committed by the task that met
  it, nothing pushed after it until the maintainer has been told and, where the
  rule that names the stop asks the maintainer, has answered: a quoted text that
  does not stand (A4, A5, any task; settled as *The batch's stop rule* below
  says); step 1's last commit not below the branch, `<lane-pred-head>` not gated
  (its documents unit's gate, its reviews) by the predecessor's
  record, a toolkit version other than the specification's, a missing target or
  linker, or the hook trial applied (P2); `<lane-pred-head>` not an ancestor of
  `<prev-tag>` before the merge
  (`git merge-base --is-ancestor <lane-pred-head> <prev-tag>` failing, rule 18),
  the remote tag ref missing or at a different sha (rule 21 (1)), an untracked
  file the merge would overwrite that differs from the tag's, a merge conflict
  whose resolution changes what a part's quoted text means (the batch's stop
  rule), a `Cargo.lock` re-application whose diff against `<prev-tag>`'s lock shows
  more than this release's packages (rule 21 (1)), a staged conflict marker (the marker check), a staged path the merge should not bring
  (rule 21 (1)'s staged-path check, in MG step (1)), a file `cargo fmt --all`
  changes outside the staged set (MG step (1)), or a red
  warm-up gate after MG whose cause is unclear (MG); step 3's trial outcome not found, neither passed nor failed, or
  told differently by U3.1's commit message (`feat: a button's border on hover and on keyboard focus: …`) and the predecessor's record (A4,
  A5);
  the pre-check failing (M5, MB, MA); K5's after-only check exiting 1 for an
  element pushed out of view (an id MA saw present or inside now missing or not
  inside, or Edit outside where Copy is inside, rule 19; a wrap-parity failure
  there is not a stop: before the capture it is reported and the capture goes
  ahead, rule 19, K5); the measurement's stop rule (MA, B.10: an
  element pushed out of view; a wrap's parity entry is not a stop, but fixed or
  excepted with its reason and told, B.10, §3.2, rule 19); the icon trial passing on one backend and failing on
  the other (§2.4); a Windows reader test name missing from a CI run's Windows
  logs (K3); a `wip(` subject in a push's range (§2); a capture that differs from C.2's expectation (K6). A parity
  difference at a capture is not a stop (rule 19): K6 fixes it or, for a toolkit's limit,
  excepts it with its reason, reviewed, and tells the maintainer.
- **The batch's stop rule.** A stop ends when the maintainer has answered where
  the rule asks; otherwise the work continues once the maintainer has been told.
  A quoted text that no longer stands is found in its present form by the
  orchestrator: where the edit's meaning is unchanged, the edit is made at the
  found text and the record names it; where it is not, the maintainer is asked.
- **A task that needs more than its parts name** (a model field, another crate, a
  decision the rationale did not take) stops, and the maintainer is asked. So does
  an upstream move during the work (a new gpui-component, GPUI Fast, egui or iced
  release, a new advisory) outside what MG brings: it is reported with what it
  costs before anything is changed for it. A version MG brings (rule 19) within
  P2's accepted set (egui 0.36.2 or 0.37.x, a patch release of the others) is VF's
  expected rework, no stop; any other version is P2's stop.
- **A fix after the units** that a review, CI, the full check, the rehearsal, the
  measurement or the asset run demands is made by an `implement` agent (a
  documentation fix by a `worker`), reviewed as in §5, and passes the
  done-criterion of the task whose files it touches. **A fix that touches 5a's or 5b's
  showcases or the Basic page's layout** (rule 19), one rule by when it comes:
  before MB, nothing is measured yet; between MB and MA, MA's blocks take it (a
  difference it makes is then read as the fix's, not Edit's); after MA and before
  MG, MA's blocks are taken again (one base), the comparison and `after_state.py`
  run again, and §10 is updated with the new run in a new documents-only commit
  (the five checkers and `--quick`; `rec`'s commit is never rewritten); after MG,
  it is never compared with the pre-merge "before" dumps: K5's after-only checks
  and the rehearsal read it, and it is named to the maintainer. A red warm-up gate,
  wherever one runs (P0's, P2's or MG's), goes as rule 12 says: to the author of the unit whose rule the failing
  test checks (a full struct literal, a count, a mapping), even where the failing
  file is the other side's, and where that author is no longer at hand (a lane
  predecessor's unit, at P2's warm-up), to a new
  `implement` agent with the log's failing part and that unit's part of its
  specification; one caused by a conflict resolution is the
  orchestrator's; one whose cause is unclear is a stop; its fix is gated by the
  warm-up run again (`--commit`, rule 12). A fix the close-out needs
  goes as rule 15 (batch-speed rationale, Appendix) says, its unstaged exceptions
  named in the record. The maintainer is told
  what stopped it, what was changed, and whether another capture window is needed.
- **The desktop** is asked for when everything before it holds and no other
  lane's capture or measurement holds it (S3): MB's and MA's measurement windows
  and K6's capture window, and a repeated window after a fix.

## 8. The maintainer's part

**With the approval of this plan, before any task:**

0. **Settled, not put again** (the maintainer, 2026-11-15):
   - S1 to S5, S3's amendment (the caret in the KDE lane) and S6 (no line numbers
     in new citations);
   - S2's amendment, the merge (the branch from the lane predecessor's gated head,
     merged once with `<prev-tag>` by rule 17's command, after the release's last
     unit before the merge, not rebased);
   - the batch's working rules 1 to 26
     (`docs/todo_v0.7.1_batch-speed-rationale.md`, Appendix), which MG, VF, X, V,
     the cadence (§4) and the close-out (§2.5) follow;
   - the three re-readings the rules name: rule 20's re-reading of S2's branch
     point (`<lane-pred-head>` is the documents unit, the last unit before MG, here
     U5c with its `--quick`, not the last code unit), rule 7's branching from
     `<prev-tag>` where that tag exists when P2 runs (no MG), and rule 9's "green"
     (CI green, and the full check's only failure the provenance stamp, which only
     the asset run refreshes);
   - D10 (a): KDE step 1 ships in v0.7.1;
   - the release order, approved with S3 (0.7.1, 0.7.2 egui 0.37, 0.7.3 KDE steps
     2 and 3, 0.7.4 KDE steps 4 and 5, 0.7.5 the widget icons, 0.7.6 the caret,
     0.7.7 KDE steps 6 and 7; S3's amendment), so `<prev-tag>` is `v0.7.3`;
   - the KDE lane's extra build folder (item 7) and a draft pull request at the
     branch's first push (item 3).

   **Put for approval with this plan:** the plan's own additions and departures
   (item 5), and the point the maintainer has not answered: D0, the version
   number (item 1). Should the maintainer
   change a settled point, the plan is amended before P1.
1. **D0: the version** (open point 1, C.3), for approval. `0.7.N`, the one
   version of steps 4 and 5 (S5); the release immediately before it in the order,
   settled with S3 on 2026-11-15, is steps 2 and 3's (`<prev-tag>`, which MG
   merges; P2 starts from the gated head of steps 2 and 3, the release before it
   in the KDE lane). P2 checks step 1's last commit by its record, and
   MG checks before the merge that `<lane-pred-head>` is an ancestor of
   `<prev-tag>`, and after it the merge commit's parents (rule 18). Under D0 (b),
   a minor version: the plan holds
   with that number and the branch name the maintainer gives, the changelog
   heading as `## [0.N.0]` and the ROADMAP heading with the same number, and V's
   README requirements. Under D0 (c): the release that makes the model's input
   structs `#[non_exhaustive]` comes first, specified apart; this plan waits for
   it, the *Breaking Changes* bullet goes (C.3), and A.1's `proptest_roundtrip.rs`
   edit is read again, which amends the specification and this plan before P0. The
   caret work now follows this release in the KDE lane (S3 as amended), so it
   never lands before it.
2. **The desktop**: two measurement windows (MB, MA; three blocks each, run
   together under one lock, §3.1) and one capture window (K6, 4 to 6 minutes); one
   more for each rerun §3.1 or §7 names; each
   asked for when everything before it holds.
3. **The pull request** (approved 2026-11-15, settled): the orchestrator opens a
   draft pull request from `v0.7.N-rc1` to `main` at its first push (K3, after MG),
   its title and description shown to the maintainer first (rule 9: CI runs on
   pull requests).
4. **The reply to the reporter**: it waits for this release (S5). The maintainer is
   not asked whether it has been posted: U4b and U5c each read it at their start
   with `cd <core worktree> && gh issue view 4 --comments`, and the record holds
   what they found (A.12, B.11: a posted reply is not edited, and whether a
   follow-up is written is the maintainer's).
5. **What this plan adds to the specification:** A4, A5 and M5 (its self-test);
   each code unit one commit with its tasks' work-in-progress commits and their
   reviews per task (the plan's choice under C.1, §1); the drop-down click tests as the check of A.14.1's click-through (T4.5); R5.trial
   before the verdict, the trial's GPUI Fast run and the stop where the two
   backends disagree; MG's place (after U5b, MA, `rec` and U5c) and its steps in
   rule 21's order, VF inside its step (3) (rule 25), K5's after-only measurement
   checks on the merged tree, and X's recount on the merged tree (the batch's
   rules 17 to 22, 24, 25); X and V (V's stamped-path grep, rule 22 (c)); the parity
   rehearsal; the reading of CI's Windows legs at K3; a parity difference at K6 fixed where this
   work's changes explain it, or, for a toolkit's limit, given B.10's exception with
   its reason, reviewed, the maintainer told in either case (K6).
   Two of these go beyond the batch's rules and are approved with this plan: the
   restore of the asset script's outputs to `HEAD` before a fix to a stamped path
   (§4, *After an asset run*; rule 15 keeps no outputs there but names no restore);
   and the layout measurement held under the shared `assets.lock` with the cargo
   slot, one lock across a measurement's three blocks (§3.1; rule 24 names that
   lock for the asset run and its capture window only). (`cargo fmt --all --check`
   before MG step (1)'s merge commit, with its `cargo fmt --all` fallback, is now
   rule 21 (1) item 10's, no addition.)
6. **The copies**: at the approval the orchestrator copies the rationale, the
   specification and this plan, as approved, into `<record>/docs/`, P0's fallback
   should the untracked copies in `.claude/worktrees/v071/docs/` be lost.
7. **The lane's build folders** (§1): this release reuses `v071`'s, and the
   steps 2–3 release's `kde-b` and its build folder (tens of GB, under
   `/run/media/tibi/work/cargo-build/`) stay through this release for the caret,
   the lane's next release, which uses `kde-b` (S3 as amended); the orchestrator
   deletes `kde-b` with its build folder when the KDE lane's last release of the
   batch, steps 6 and 7, closes. That departs from the
   maintainer's rule that "a separate build folder is deleted by the agent that
   made it before its task ends"; the maintainer approved it on 2026-11-15
   (settled): the folder keeps the lane's builds warm across its releases and
   serves the fixes CI or the maintainer may still ask for until each merge.
8. **The choices this plan depends on** (rule 18), both settled on 2026-11-15:
   D10 (a) (KDE step 1 in v0.7.1; D10 (b) not taken) and the release order
   (approved with S3), which give steps 2 and 3's gated head as
   `<lane-pred-head>`, `v0.7.3` as `<prev-tag>`, 0.7.4 as the proposed number
   (D0, item 1, decides the number) and `v071`, as the header's *The open
   choices* gives them. This release's own gated head and record are its lane
   successor's, the caret's (S3 as amended).

**During the work, the maintainer is told or asked:** step 3's outcome as A4
found it; a stop of §7; "measure now" and its announcement (MB, MA); the
measurement's stop entries (asked) and its parity entries (told, with how each was
fixed or excepted); the icon trial's verdict; a red CI
run; MG's conflicts and any toolkit version it brought; "capture now" (K6) and what the new screenshots show; a parity difference at K6
and how it was settled (fixed, or excepted with its reason), told, not asked; K7's
mentions left by rule 22 (a)'s exemptions.

**After K8:** whether to merge, tag and publish, and the pull request's title and
description. Posting the reply to the reporter is then the maintainer's own action,
after this release (S5); the plan neither asks about it nor waits for it.

## 9. Record

Copied from `<record>/record.md` at K7.

- P1: `<locks>` made, the trial's output `<…>`.
- The branch: `<lane-pred-head>` and the predecessor's record line "the lane's
  gated head for the next release" that names it `<…>`; this release's line of the
  same name, U5c's commit (rule 20) `<sha; --quick run folder; reviews closed>`;
  MG's merge, with which tag, the remote tag check, the ancestry check, the
  conflicts and their resolutions, the `Cargo.lock` re-application where there
  was one, the version comparison, the path check, the survived checks and R-MG
  `<…>`; VF `<none needed | the commits>`; the
  warm-up gates `<…>`; the first
  `--quick` run's time `<…>`; each unit's gate mode and result `<…>`, and any
  failure a `--commit` found with the commit that brought it `<none | …>`.
- Step 4: the worktree (reused or new) and its build folder `<cargo-build>/<hash>`;
  the toolkit versions `<…>`; the maintainer's answers (§8) `<…>`; the hook's
  refusal `<…>`; P0's or P2's gate time `<…>` (`<cold | warm>`); A4
  `<all found | the texts not found>`, step 3's outcome `<passed | failed>`, the
  counts' bases `<…>`; the units' bases and
  work-in-progress commits `<sha …>`; the reviews and their must-fixes `<…>`; the units' commits `<sha …>`.
- Step 5: A5 `<…>` at `<A5 base>`,
  the egui version and its forms `<…>`; M5's self-test `<…>`; the units' bases and work-in-progress commits `<sha …>`; the reviews `<…>`; the icon
  trial: both result lines `<…>`, R5.trial `<…>`, the verdict
  `<follows | does not follow: the mode and the two colours>`.
- The close-out: K3's CI run `<id>` and the Windows tests' log lines `<…>`; K4's
  failure and warnings `<…>`; K5's rehearsal and its after-only check, with the two
  scales (their agreement not verified) `<…>`; K6's `STATUS`, the log's tail and
  what the screenshots showed `<…>`; K7's rewritten mentions and the exempt ones
  left (rule 22 (a)) `<…>`.
- Durations measured `<…>`. Departures from this plan `<…>`.

## 10. Measurement record (step 5, B.10)

Committed by `rec`, on top of U5b's last commit, in B.10's form.

- The folders: `target/measure-5b/before` (at `<sha>`), `target/measure-5b/after`
  (at `<sha>`, its subject unit's last commit, U5b's, rule 19), deleted after the
  release's first push (K3), MA's state kept in `<record>/measure-5b/after-state.json`
  for K5 (rule 19); the shas stay valid, as MG is a merge.
- The pre-check: before `<…>`, after `<…>`.
- Each block's time, before and after, per showcase `<…>`; the reruns `<…>`; the
  entries whose icon set fell back `<…>`.
- The comparison's printout and exit status `<…>`; per entry, whether Edit starts
  a new line, and each wrap with what it moves `<…>`.
- The wrap-parity exceptions (§3.2): per exception, the entry, the kinds that
  differ, the toolkit's limit with its source, and R-exc's verdict
  `<none | …>`; the parity fixes made instead, with their commits `<none | …>`.

## 11. Time

Measured at the 0.7.1 release on this machine and at GitHub (the archived plan of
issue #4's KDE fixes, §8): the full `check_release.sh` about 23 minutes; the asset
script 30 to 37 minutes, of which 4 to 6 need the desktop; a CI run 21 to 27
minutes. From the specification: a measurement block is at least 9 × 6 s and at
most 9 × 46 s of sleeps and bounded waits (B.10, *The duration*). A `--commit`
run: the batch's earlier figures, 5 to 12 minutes a commit and 29 minutes cold
(§4), measured before the 16-job cap; today's is not measured until P0's or
P2's timed run (§4), which the record holds. Not measured: `--quick` (its first
run is timed, S1), the first build in a new worktree, the trial's GPUI Fast run,
the agents' whole showcase legs, and every task's duration; the first runs give
them, and the record holds them. The longest chain is the lane: P2, P0,
T4.1 to T4.5, U4a, U4b, T5.1 to T5.4, U5a, MB, U5b with the trial and
R5.trial, R5b, MA, `rec`, U5c, MG (and the wait for the tag) with VF, its path
check, its survived checks and R-MG before its warm-up `--commit`, X, V, K1 to
K8 (K4
and K5 beside K3's CI, S4); plus any review fix, and the waits for
`<prev-tag>`, the pull request, the maintainer's answers, the measurement
windows and the capture window.
