# 0.7.x: issue #4's open KDE items, steps 6 and 7 (the default button; the table header's states and the unfocused selection) — implementation plan

- Rationale: [`issues/0004_kde-open-items-rationale.md`](issues/0004_kde-open-items-rationale.md)
  (D8 for step 6, D7's second half and D9 for step 7, D0 for the version, D10 for
  what a step carries outside the code and the order of the releases).
  Specification: [`todo_v0.7.x_kde-open-items-steps6-7-spec.md`](todo_v0.7.x_kde-open-items-steps6-7-spec.md)
  (Part F, Step 6's parts 6.A to 6.I, Step 7's parts 7.A to 7.J; checked, final;
  amended 2026-11-15 for the batch's S3 amendment and S6).
- Written: 2026-11-15, against `v0.7.1-rc2` at `0fa28027`, the specification's
  commit. Status: **not approved**. Nothing below starts before the maintainer
  approves the rationale, the specification and this plan, and answers §8's
  points.
- The release: **one** 0.7.x release, `0.7.N`, for both steps (the batch's speed
  decision S5, `docs/todo_v0.7.1_batch-speed-rationale.md`, approved 2026-11-15; D0
  (a), the specification's basis), item 5 of *The releases after v0.7.1* (the
  rationale's D10): one branch, one version, one CHANGELOG section, one ROADMAP
  paragraph, one close-out with one asset run and its capture check, one archive at the
  end; each step keeps its own units, measurement (step 6), reviews and record. The
  number is the maintainer's; `<prev-tag>` is the tag of the release immediately
  before it in the maintainer's order (under S5 and D10's proposal, the caret's,
  `v0.7.6`), and the release before it in its own lane, the KDE lane, is the caret's
  (S3 as amended 2026-11-15: the caret's blink moved into the KDE lane). It needs
  step 3, step 5 and the caret in its tree, all the KDE lane's, so its base holds
  them; the widget icons, alone in the second lane, and egui 0.37 (v0.7.2) reach its
  tree through MG's merge of `<prev-tag>` (S2, as amended).
- **No line numbers in new citations** (S6, decided by the maintainer 2026-11-15): a
  citation this release writes names the document, with a quoted phrase where that
  helps, never a line number of a tracked file that changes; existing line-number
  citations are left as they are and may drift, and nothing re-points them. This
  plan's own instructions still cite `0fa28027`'s lines as pinned ("at
  `0fa28027`"), and pinned sources (an upstream crate at a version) keep their lines.
  So this release has no sweep unit, no re-pointing at MG, and an X that only
  recounts stated counts. Every `file:line` this plan cites is `0fa28027`'s unless it
  names another commit or version (each such citation pinned, S6).
- Where: branch `v0.7.N-rc1`, made from `<lane-pred-head>`, the gated head of the
  release before this one in its own lane (the KDE lane: v0.7.1 → steps 2 and 3 →
  steps 4 and 5 → the caret → steps 6 and 7; here the caret's, its gated head: its
  documents unit's commit, past its `--quick`, its reviews closed: rules 18, 20), not
  from its tag. `<prev-tag>`, the tag of the release immediately before this one in
  the maintainer's release order, which may be another lane's, is merged into the
  branch once (task MG, §2.4a; S2 as amended 2026-11-15: a merge, not a rebase, so
  that every sha stays). The merge waits for three things: U7.4, this release's last
  unit before the merge (after U7.3), has passed its gate (the five checkers and
  `--quick`: the KDE lane's last release has no gated head, rule 20); every
  review is closed; and the tag exists. Its command is
  `cd <core worktree> && git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>`
  (rule 17), and the merge commit is made only after MG's resolutions and
  regenerations. This
  plan merges no pull request and tags and publishes nothing; it opens a draft pull
  request at the branch's first push (K3), which the maintainer approved on
  2026-11-15 (rule 9; §8 item 3).
- **The choices rule 18 names, settled** (§8): the maintainer chose D10 (a), KDE
  step 1 inside v0.7.1, and approved the release order with S3 on 2026-11-15 (0.7.5
  the widget icons, 0.7.6 the caret, 0.7.7 this release, the version numbers
  themselves still D0's, §8 item 1). So `<lane-pred-head>` is the caret release's
  gated head, read from the rule 20 line of its record,
  `<scratch-keep>/todo_v0.7.x_caret-blink-plan/record.md` (the caret plan's §9), and
  `<prev-tag>` is the caret's `v0.7.6`, which holds the widget icons and egui 0.37
  (v0.7.2, S3's paragraph *v0.7.2 (egui 0.37)*) through the caret's own MG; this
  release uses `.claude/worktrees/v071` (§1). D10 (b), step 1 as v0.7.2, with every
  number one up and this release in `.claude/worktrees/kde-b`: not taken (D10 (a),
  2026-11-15). Where the branch can be made from `<prev-tag>` (P2·6), there is no MG.
- The batch's working rules are cited as "rule N": rule N (batch-speed rationale,
  Appendix), the consolidated list in `docs/todo_v0.7.1_batch-speed-rationale.md`,
  which governs where this plan and it differ. The rules and S2's amendment (a
  merge, not a rebase) were approved by the maintainer on 2026-11-15 (§8 item 8).
- Placeholders: `0.7.N`, `<prev-tag>` (the tag of the release immediately before
  this one in the maintainer's order, which MG merges), `<lane-pred-head>` (the
  gated head of the release before this one in its own lane, which the branch is
  made from), `<base>` (rule 17: `<prev-tag>` where the branch was made from it,
  else `<lane-pred-head>`), `<merge>` (MG's merge commit; with no MG, `<merge>` and
  `<merge>^1` read as HEAD), `<core worktree>` (`/home/tibi/Rust/native-theme/.claude/worktrees/v071`, the KDE lane's
  worktree this release uses, §1), `<date>`, `<sha …>`,
  `<record>`, `<tools>`, `<locks>`, `<scratch-keep>`, `<cargo-build>` (§6), and every
  count, line, time and result read at a run. Each is filled by the task that reads
  it, from its run; the orchestrator writes it into the running record (§6,
  *The record*); none is estimated.

This plan says who does what, in which order, and how each task is known to be
done. What each task changes is the specification's; the plan repeats none of
it.

## 1. Shape of the work

The specification's units are U6.1, U6.2, U6.3 for step 6 (6.I.1) and U7.1 to
U7.4 for step 7 (7.J.1), in its order, each one commit, and adds the tasks around
them: the tools, the branch and its merge of `<prev-tag>`, the documents, the
anchors, the measurement (6.G), the capture check (7.H.2), the recount after the
merge (X), the version and the one close-out.

The release has **one lane**, the core worktree, which is the only place in it that
builds; step 6's tasks run first, then step 7's, on one branch. Everything else
runs beside it without a worktree of its own: the anchors and the measurement's and
capture check's scripts read the branch with `git show` and write only into
`<record>`; the reviewers read commits. The widget icons run in the batch's second
lane, in a worktree of their own (S3 as amended: the caret is in the KDE lane); the
machine's limits hold for both lanes together (§7).

| Lane | Worktree and branch | Tasks | Cargo |
|---|---|---|---|
| core | `<core worktree>`: `.claude/worktrees/v071`, `v0.7.N-rc1` made in it from `<lane-pred-head>` (*The worktrees and their build folders* below) | step 6: P2·6, P0, MB, U6.1, MA, `rec`, U6.2, U6.3; step 7: P·7, U7.1, U7.2, U7.3, U7.4; then MG (once U7.4 has passed its gate, every review is closed and `<prev-tag>` exists; VF inside it), X (only after an MG), V, K1 to K8 | yes: the only place of this release that runs it |

P1 (the tools) runs once, before step 6. A6, M0 (step 6) and A7, C0 (step 7) run
beside the lane and commit nothing.

```
step 6   P1 P2·6 ─ P0* ─ MB ─ U6.1(1) ─[condition]─ U6.1(2) ─────── MA ─ rec ─ U6.2 ─ U6.3
beside        A6 ─┘  M0 ─┘                             R6.1b R6.1c ─┘     R6.2 ┘ (U6.3 after it)
         * P0 where an earlier commit left anything to add or set; else P2·6 runs the warm-up gate in its place
step 7   P·7 ─ U7.1 ─ U7.2 ─ U7.3 ─ U7.4
beside   A7 ─┘  C0 (beside U7.1, by U7.2 at the latest)
reviews         R7.1h, then R7.1 (beside U7.2)   R7.2 (beside U7.3)   R7.3 (U7.4 after all four reviews)
release  MG (U7.4 past its gate, every review closed, <prev-tag> exists; VF inside its step 3) ─ X ─ V ─ K1 … K8 (one close-out, S4's order, C inside it)
```

Why this order (each from the specification unless named):

- **One release, one lane** (S5). Every unit of a step needs the one before it: U6.1's
  fields are what U6.2's reader writes and its moved-leaves lists name (6.I.1);
  U6.3's dated lines name U6.1's and U6.2's last commits (§2.2); U7.2's reader
  writes U7.1's field, and U7.3's showcases call U7.1's accessors (7.J.1). Two
  code units side by side would need a second worktree, a whole extra build of
  the gpui, iced and egui stacks, for no unit that can run alone. A documents lane
  would gain little: every text of 6.H and 7.I that names a commit names a code
  unit's last commit, its review fixes included, and the rest is a short task.
- **A6 and A7 first** (a plan addition, as the caret plan's task A): the
  specification's lines are `0fa28027`'s, and up to seven releases land before
  step 6 (the header). Every quoted text and every count's base the units work from
  is produced again on the tree each step starts from.
- **MB before U6.1**: the "before" dumps are of the tree U6.1 starts from, before
  its first commit, in the same worktree (6.G, *When*). P0 changes no layout, so
  MB may follow it.
- **U6.1 in two phases** (§2.2): 6.D.4's test decides 6.D.2's condition before
  anything that reads it is written (6.I.1), so the implementer reports it once
  and the orchestrator records it before the phase that reads it.
- **MA after R6.1's reviews**: the "after" dumps are of U6.1's last commit, before
  the release's first push (6.G, *When*); a review fix that touches the Basic page or
  a showcase's layout would mean the "after" blocks again, so the reviews close first,
  as the widget icons plan's measurement waits for its reviews.
- **U6.2 after `rec`**: it works in the same worktree, and the "after" blocks build
  the showcases from it; U6.2 changes no showcase (6.G).
- **Step 7 after step 6's units and reviews** (7.J.1; S5: the same branch).
- **MG once, after U7.4's gate and every review, when `<prev-tag>` exists** (S2 as
  amended; rules 17, 20): the branch starts from the lane predecessor's gated head so
  that this release's units run while the lane predecessor's close-out and the
  maintainer's merge, tag and publish are pending; the merge of the tag, then the
  warm-up gate, comes before anything is pushed and before X, V and the close-out, so
  that the compatibility and semver steps compare with the released predecessor; VF,
  the rework of a toolkit version the merge brought, lies inside it, before its
  warm-up gate (rules 21, 25). It
  comes after MB, MA and `rec`, so the measurement stands on one base and no
  re-measurement exists: the merged tree's layout is read by K5's rehearsal and its
  after-only checks (every entry the measurement read, and the rehearsal's own two)
  and by the asset run's parity step (rule 19), and a
  merge changes no sha, so every sha the record, §10 and the dated lines name stays
  valid.
- **X after MG, V before the close-out** (the caret plan's X and V): X recounts the
  stated counts in the files either side changed, on the merged tree, and runs the
  five documentation checkers (rule 17; S6: no citation is re-pointed), only where
  there is an MG (with no MG there is no X); the provenance stamp hashes `Cargo.lock`, which V changes.
- **The close-out only after every review of both steps is closed**, its fixes
  committed, and MG done.

**The worktrees and their build folders** (the batch's worktree rule, S2, S3).
Cargo keeps one build folder per worktree path, and a worktree's build folder is not
shared with another. The KDE lane (v0.7.1, the three KDE pair releases and the
caret, S3 as amended) alternates between two worktrees: `.claude/worktrees/v071`,
v0.7.1's, and `.claude/worktrees/kde-b`: steps 2 and 3 in `kde-b`, steps 4 and 5 in
`v071`, the caret in `kde-b`, steps 6 and 7 in `v071`. Each release uses the
worktree named here, also where its predecessor's worktree is free when it starts,
and reuses it only once the release that last used it has its tag (its close-out
reported and the maintainer's merge done; rule 18, checked after
`git fetch origin tag <tag>`): for this release, steps 4 and 5's tag must exist
before P2·6 switches `v071`; where it does not, the release waits for it (rule 18;
the other worktree, `kde-b`, is held by the caret's close-out). The second lane, the
widget icons alone, uses `.claude/worktrees/lane2-a`; `lane2-b` is not used. Each
worktree keeps its own build folder, which its lane's later releases reuse warm;
the orchestrator deletes a lane's extra worktrees and their build folders when the
lane's last release of the batch closes (the KDE lane's, `kde-b`, with steps 6 and
7; the second lane's, `lane2-a`, with the widget icons), and `v071`'s folder stays.
So this release uses `v071` (S2): P2·6 makes the branch there (`git switch -c` in it,
where `git status --porcelain --untracked-files=no` prints nothing and every
untracked file is a planning document under `docs/`). Its build folder is read from
`cargo metadata --format-version 1 --no-deps` (`build_directory`) and written into
the record as `<cargo-build>/<hash>`. Steps 6 and 7 are the KDE lane's last release
of the batch (S5), so after K8 the orchestrator sweeps `<core worktree>` with
`sweep-target.sh` (§2.5), and when this release closes, at the maintainer's merge
(until then `<core worktree>` holds the branch for the fixes CI or the maintainer
may ask for), it removes `kde-b`, once the caret's tag exists and nothing runs there
(`git -C /home/tibi/Rust/native-theme/.claude/worktrees/v071 worktree remove /home/tibi/Rust/native-theme/.claude/worktrees/kde-b`),
and deletes its build folder; `v071` and its build folder stay. The close-out report and the
project status note name both folders. Worktree paths are written relative to the
main checkout, `/home/tibi/Rust/native-theme`, in prose; every worktree command
runs with absolute paths and `git -C` (rule 24); a new one is made with
`git worktree add` from the named commit (an agent's own isolation worktree starts
from `main`, the wrong base).

## 2. Tasks

A task that changes files ends in one commit on the lane's branch, unless it
says otherwise, made only when its done-criterion holds, with no attribution
line. **Who runs what** (the batch's rule of 2026-11-15, §4): an agent runs its
task's targeted commands in the foreground (the named tests,
`cargo fmt --all --check`, the checkers, a `-p` build or clippy, each expected to finish well
within 10 minutes), stages its own paths by name (`git commit -a` and `git add -A`
are not used), and reports the staged change with its commit message; it does not
run `--quick`, `--commit`, the full check, an asset run or the GPUI Fast legs, and it does not
commit. The orchestrator then runs the task's gate on the staged change and, when
it passes, commits exactly the gated tree with the author's message. In the task
rows, items an agent cannot run are marked "(in the gate)". The subject follows
the repository's form (`feat(core): …`, `docs(todo): …`) and the body says what
changed and why, with the outputs the done-criterion names. Shared terms:

- "**The gate passes**": the orchestrator's long run (§4) of the task's gate by the
  batch's cadence (S1; §4's cadence table names each task's):
  `scripts/check_release.sh --quick` (as `docs/todo_v0.7.1_gate-speed-spec.md` defines it), or
  `scripts/check_release.sh --commit` where the table says so, in the core worktree
  through the wrappers (§6), on the staged change; it exits 0 (its `STATUS` file
  reads 0), its summary names no failure and no skipped cross-target check
  (`target not installed`, `x86_64-w64-mingw32-gcc not found`), the record names every warning
  the summary repeats, and the tree is formatted and clean; where the row says "with
  the GPUI Fast legs", those run beside the gate whatever its mode (`--quick` or
  `--commit`), as every plan of the batch runs them beside each gpui unit's gate:
  U6.1, U7.1, U7.3, MG's warm-up and a fix that touches gpui code. A red gate goes back to the task's author
  (continued) with the log's failing part; the author fixes and stages again, and the
  orchestrator runs the gate again. A failure a later `--commit` finds in a commit
  only `--quick` proved is fixed by a new commit on the branch, never by rewriting a
  pushed one, and the record names the commit that brought it (S1). "`--commit`
  passes" in an orchestrator's own task means that run. For a task whose change a showcase
  test checks (U6.1, U7.3, and U7.1, whose egui mapping rows the egui showcase's
  `the_theme_map_names_the_tests_that_check_a_per_call_route`,
  `showcase-egui/tests.rs:6872`, holds to their tests; and a fix of theirs), a
  runner-environment exit 75 at
  `--prepare`, which the gate reports as a soft warning
  (`scripts/check_release.sh:825-828`: the showcase tests not run in CI's runner
  environment), voids the gate as a skipped cross-target check does: the
  environment is prepared and the gate run again. So does an exit 75 of the GPUI
  Fast showcase leg
  (`scripts/run_in_runner_env.sh test … --example showcase-gpui`, below) in the gates of U6.1, U7.1 and U7.3, and of a fix of theirs. The orchestrator prepares the runner environment (`scripts/run_in_runner_env.sh --prepare`, through the wrappers) before it dispatches U6.1 and U7.3, whose agents run showcase tests in it (§2.2); an exit 75 there is reported to the maintainer, and the dispatch waits for the maintainer's answer.
- "**Formatted and clean**": §4's rule (`cargo fmt --all --check` by the author
  before staging; the gate on the staged change;
  `git status --porcelain --untracked-files=no` the same before and after the gate, and empty after the
  commit).
- "**The staged tree holds**", checked by the orchestrator before each gate and
  again before the commit: `git diff --quiet` exits 0 (nothing unstaged);
  `git status --porcelain --untracked-files=all` shows no `??` line but the
  untracked documents P2 recorded (less those P0 commits), each named one by one
  in the record (never by a pattern), so that a new file of the task (a new `.rs` file, a
  table, an evidence file) is staged, not left untracked; and
  `git diff --cached --name-status` lists exactly the task's new files (`A`, each named in its row,
  "New files") and changed files (`M`, the files its parts own in the
  specification's *How to read it* table and the generated files it regenerates), anything else named and explained before the
  gate runs. A row that adds files names them under "New files"; a row that names
  none adds none. The exception (rules 11, 15): outputs a close-out step keeps
  across a fix stay unstaged through that fix's commit and are named in the record as
  that commit's unstaged exceptions, each by name, `git diff` then listing only them:
  for any fix committed between K2 and the asset commit (K7), K2's
  `docs/COMPATIBILITY.toml` and connector READMEs; for a fix made while the asset
  script had not finished, its partial outputs (with K2's files).
- "**The GPUI Fast legs**": the two that `--commit` skips
  (`scripts/check_release.sh:710-716`, `:849-855`), run by the orchestrator in the
  gate, each a long run, and, as part of that gate, each started under the gate's
  lock (the batch Appendix's glossary; rule 2; §4, *The gate lock*):
  `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh cargo test -p native-theme-gpui --features gpui-fast,native,watch --all-targets`
  and `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/run_in_runner_env.sh test -p native-theme-gpui --features gpui-fast,native,watch --example showcase-gpui`
  (the runner environment prepared first with `--prepare`; an exit 75 is
  reported, not worked around). A GPUI Fast run of the orchestrator's outside a gate
  (a trial's) takes the gate lock too, as every long run of the orchestrator's that
  runs cargo does (rule 2; §4, *The gate lock*).
- "**The five checkers**": §4's documentation checkers. `$NT_FEATURES` is the list
  `scripts/check_release.sh:420` sets (`kde,portal,system-icons,material-icons,lucide-icons,watch,svg-rasterize,system-fonts`
  at `0fa28027`); the orchestrator reads that line on the branch and writes the
  list literally into every brief that uses it. "**The header**" is the specification from
  its top to the end of *Open points for the maintainer*, *How to read it* (with
  *Lines that must not move*) and the decisions included.
- "**Seen to fail**": a new test, or a changed test's new assertion that tests the
  change, is written first and run before the change it tests; a test that cannot
  compile before its item exists counts, and the report quotes the failure. A
  changed test that only gains a step (an activation, a renamed field) is run
  after the change, its `ok` quoted.

### 2.1 Before step 6

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| P1 | **The tools**, once for the release. `mkdir -p <locks>`; copy `slot.sh` and `lowprio.sh` from `<scratch-keep>` into `<tools>` and read both. In the copies: `slot.sh`'s lock folder (`D=`, an old session's scratch folder) becomes `<locks>`, the one lock folder every plan of the batch uses, so the two slots count every cargo process of every plan and session; `lowprio.sh` keeps its absolute `renice -n 19 -p $$` (a relative `nice -n 19` gives 15 under Konsole's −4) and its `ionice -c 3`, and gains `export CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16`, lifted only on the maintainer's word that they are not using the machine. The copies of the approved specification and this plan in `<record>/docs/` were made at the approval (§8 item 6), before P1; P1 copies the inputs of §6 into `<record>/inputs/`, and, for each document of this work, records whether it is still untracked in the commit the branch is made from (rule 26: P1 runs before the branch exists, so it reads that commit, not a checkout; `git -C /home/tibi/Rust/native-theme cat-file -e <lane-pred-head>:docs/<name>`, `<prev-tag>` in place of `<lane-pred-head>` where the branch is made from it, failing means untracked) or which commit added it (`git -C /home/tibi/Rust/native-theme log --diff-filter=A --format=%H <lane-pred-head> -- docs/<name>`, the same commit as the `cat-file` read), copying nothing for an added one. No repository change. | orchestrator | the maintainer's resource rules | `<tools>/slot.sh <tools>/lowprio.sh true` exits 0; `<tools>/slot.sh <tools>/lowprio.sh sh -c 'echo "$CARGO_BUILD_JOBS $RUST_TEST_THREADS"; ps -o ni= -p $$'` prints `16 16` and `19`; after the trial `slot1.lock` is in `<locks>` (`ls`; `slot.sh`'s header names `slot1.lock` and `slot2.lock`, and the second appears only when two commands overlap); `cmp` of each copy in `<record>/inputs/` against its original prints nothing; each document of this work in the record as still untracked or added by `<sha>` |
| P2·6 | **The branch and the worktree** (§1, *The worktrees and their build folders*; S2). The branch is made from the lane predecessor's (the caret's) gated head, `<lane-pred-head>` (its gated head, its documents unit's commit past its `--quick` with its reviews closed, as its record's rule 20 line says, `<scratch-keep>/todo_v0.7.x_caret-blink-plan/record.md`; rules 18, 20; the sha written into this record), or from `<prev-tag>` where that tag already exists (then there is no MG; `git fetch origin tag <prev-tag>` first where it is not local, and in any case, before the branch is made, MG step (0)'s tag check: the first field of `git ls-remote origin 'refs/tags/<prev-tag>'` equals `git rev-parse <prev-tag>`, no line printed or a different sha a stop, rule 21): in `v071` (§1), `git -C /home/tibi/Rust/native-theme/.claude/worktrees/v071 switch -c v0.7.N-rc1 <lane-pred-head>` (absolute paths, rule 24). `git switch` refuses where an untracked planning document is a file the base's tree holds: the two are compared with `cmp`, the untracked copy removed where equal, a stop otherwise (§7). Before `v071` is switched, the worktree reuse check (rule 18; §1): steps 4–5's tag, the release that last used `v071`, exists (`git -C /home/tibi/Rust/native-theme/.claude/worktrees/v071 fetch origin tag <steps 4–5 tag>`, then `git -C /home/tibi/Rust/native-theme/.claude/worktrees/v071 rev-parse -q --verify <steps 4–5 tag>` prints a sha); where it does not, the release waits for it, nothing switched. Also read: step 3's, step 5's and the caret's work is in HEAD (S5 and S3 as amended make them the KDE lane's releases before this one): the caret's record shows `<lane-pred-head>` gated, its rule 20 line quoted into this record ("the lane's gated head for the next release: <sha> (<unit>; --quick run <folder>; reviews closed)"), a stop otherwise; steps 4–5's gated head, the `<lane-pred-head>` the caret's record names, is an ancestor of HEAD (`git merge-base --is-ancestor <that sha> HEAD`); steps 4–5's measurement-record commit (their `rec`, from their record, `<scratch-keep>/todo_v0.7.x_kde-open-items-steps4-5-plan/record.md`) is an ancestor of HEAD (6.I.1); and only where the branch was made from `<prev-tag>`, `<lane-pred-head>` is an ancestor too (on the other path HEAD is that commit). The tag's ancestry is not checked here on the branch-from-lane-head path: MG's step (0) checks `git merge-base --is-ancestor <lane-pred-head> <prev-tag>` before the merge (rule 18); where the branch was made from `<prev-tag>`, P2·6 checks `git merge-base --is-ancestor <lane-pred-head> HEAD` (above); and `Cargo.lock`'s versions of the toolkit crates (`egui`, `egui_extras`, `egui-winit`, `egui_kittest`, `iced`, `iced_widget`, `iced_core`, `iced_runtime`, `gpui-component`, `gpui-pre`, the GPUI Kit crates and `gpui-fast`, each as the lock names it) are listed into the record, for MG's comparison, with the accepted set (rule 19, as the steps 2–3 plan's P2 has it: egui, egui_extras, egui-winit and egui_kittest 0.36.2 or 0.37.x; for the others, patch releases of the specification's versions: gpui-component 0.9.1, the GPUI Kit crates `gpui-kit` and `gpui-kit-assets` 0.9.1, `iced` 0.14.0, iced_widget 0.14.2, iced_core and iced_runtime 0.14.0, gpui-pre 0.3.12, gpui-fast 0.1.15, the versions the header and Step 7's parts cite, as at `0fa28027`; a locked version outside it is a stop here); on both paths (the branch made from `<lane-pred-head>` or from `<prev-tag>`, rule 19), the tree's `Cargo.lock` is compared for the same crates with `git show 0fa28027:Cargo.lock`, and a changed version's readings (A6 (g), A7 (h)) are made before the first unit, or P2·6's stop (rule 19); `rustup target list --installed` lists `x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu` and `x86_64-apple-darwin`, and `x86_64-w64-mingw32-gcc` is on the path. Before any code agent is dispatched, the project's hooks are seen to fire in the core worktree (they are configured with absolute paths in the main checkout's `.claude/settings.json`), in the caret plan's form: a Write of a new, untracked file `<core worktree>/native-theme/src/hook_trial.rs` containing `fn f() { x.unwrap() }` is refused by `no-runtime-panics.sh`, and nothing is written; if the Write applies, the file is removed with `rm` and the plan stops, no code agent dispatched. Where there is no P0 (it has nothing to commit), P2·6 runs §4's warm-up gate before MB's pre-builds, timed into the record as the first `--commit`; and where there is neither a P0 nor an MG (the branch made from `<prev-tag>` and nothing for P0 to commit), P2·6 runs the path check of rule 22 (b) itself, as MG's step 6 describes it, before that warm-up gate: a re-point it needs is a `worker`'s documents-only commit, the one commit P2·6 then has, covered by the warm-up gate that follows (rule 22 (b): a re-point made before a warm-up gate is covered by that gate). No commit otherwise. | orchestrator | §1 | steps 4–5's tag found by `git rev-parse -q --verify` after the fetch, its sha in the record (or the wait recorded, nothing switched); `git rev-parse --abbrev-ref HEAD` prints `v0.7.N-rc1` and `git rev-parse HEAD` equals `<lane-pred-head>` (or `git rev-parse <prev-tag>^{commit}`, or `HEAD^` does where P2·6 committed a re-point); on the tag path, the tag check's two shas in the record, equal; `git status --porcelain --untracked-files=no` prints nothing, every untracked file a planning document under `docs/`, the untracked files listed one by one in the record (`git status --porcelain --untracked-files=all`, the list the staged-tree check of §2 reads); the ancestor checks exit 0; the build folder (`cargo metadata --format-version 1 --no-deps`, `build_directory`, through the wrappers) and the targets are in the record, the folder as `<cargo-build>/<hash>`; the hook's refusal is quoted in the record, and `<core worktree>/native-theme/src/hook_trial.rs` does not exist. `<lane-pred-head>`'s rule 20 line missing from the caret's record, steps 4–5's gated head or steps 4–5's `rec` commit not an ancestor, a target or the MinGW linker missing, or the trial Write applied: stop (§7) |
| P0 | **Unit 0: the documents**, in the caret plan's form (its P0), under rule 26: only a document of this work still untracked on the branch (`git ls-files --error-unmatch <path>` fails there) is compared with its approval copy in `<record>/docs/` by `cmp` and added where it lies untracked today, in `docs/` of `v071`, the core worktree (equal: added as it stands; gone: the record's copy; different: a stop, §7, the record naming what changed since the approval), its status line set to approved with the date. A document an earlier commit added is not compared (later close-outs rewrite its path mentions, rule 22 (a), and set its status line): P1 records the commit that added it, and P0 only sets a status line that commit left. Where an earlier commit added all of them (v0.7.1's commit (0) adds every approved document of the cycle, the `docs/todo_v0.7.x_*` plans written by then among them, and sets their status lines: `docs/todo_v0.7.1_gpui-kit-0.9.1-plan.md`, its P0 row, "**Commit (0).** The documents of this cycle the maintainer has approved", and its §7 item 1, "The documents of commit (0)"), P0 sets only the status lines that commit did not set; where that too is done, there is no P0, and P2·6 runs §4's warm-up gate in its place. **Paths and links** (rule 22 (b), which replaces rule 13: no plan writes or reads a later-links list): the path check runs right after MG (its step (6), which gives the check), at P0 only where P2·6 made the branch from `<prev-tag>` (then there is no MG), and in P2·6 where there is neither (rule 22 (b)); where P0 runs it, P0's commit carries its re-points. On the branch made from `<lane-pred-head>`, P0 makes no path check: that head is the lane predecessor's pre-merge head (rule 20), so the archive moves of the earlier releases' close-outs, both lanes', reach the branch, where that head lacks them, through MG, whose step (6) checks every path on the merged tree; a check at P0 would be made again there and could not see those moves. New files: the documents it adds, each named in the record. | orchestrator | §8 | `git show --stat HEAD` lists the documents (and the edited specification, where link edits were applied) and nothing else; where the branch was made from `<prev-tag>`, the path check's output in the record, each path with its result, none gone; the five checkers pass; `--commit` passes on the staged change, timed into the record as the first `--commit` (§4). Commit: `docs: issue #4's open KDE items, steps 6 and 7: the specification and the plan, approved <date>` (or `…: the status lines, approved <date>`) |
| A6 | **The anchors of step 6**, beside P0 (or the warm-up gate, where there is no P0). At P2·6's commit, with `git show <sha>:<path>` and the specification's own search methods, the scripts kept in `<record>/anchors-6/`: (a) every present text Step 6's parts and Part F quote to replace or to place an edit by (the base holds the facts rows steps 2 to 5 insert and the caret's A.8 after `:1209` at `0fa28027`, so the facts lines stand lower than the specification's numbers say; each is found by its text anyway), and every test they change by name, found in the file named as often as the edit names it, with its line, or "not found"; (b) the earlier releases' parts this step builds on, found: step 3's trial and its outcome as that release recorded it (3F.4 or 3F.5: its commit message, or its plan's record, never a status line its close-out writes, rule 18; `variants::bordered_button`, `variants::hover_border`, `bordered_button_matches_the_default_button_but_its_hover_border`, the `ButtonKind::Bordered` arms), steps 4 and 5's `styles::toolbar_button` with its entries in the iced README's styles row (`README.md:189` at `0fa28027`), the crate doc's `styles` row and `ROADMAP.md`'s paragraph (each found by its text, the place 6.E.4 and 6.H.2 put `button_default` after), `geometry::toolbar_button_rest` or its absence, `ToolbarButtonStates`, `toolbar_button_states`, `toolbar_button_checked`, `basic.icons.edit`, step 3's `hover_border_color` and `focus_border_color` with their registry keys and rules; (c) the counts' bases (6.F.3's leaf count with its message and the `buttons` group's totals, 6.C.4's `derived_leaves` count, 6.E.4's `pub fn` count of `src/styles.rs` and the row's text, and `ROADMAP.md`'s "has <n> style functions" sentence (`:162` at `0fa28027`, "has 33 style functions" there), found by its text, both `ROADMAP.md:264`'s stated count of open `[v0.7.x]` items (29 at `0fa28027`) and `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md` on the tree (41 at `0fa28027`), the two recorded apart, the `colors[..]` indices and vector length of `arb_button_theme`, and every full struct literal of `ButtonTheme` by 6.A.2's scan (its `git grep -nE` over `*.rs` for `ButtonTheme {` at a line's start or after a character that is no letter, as 6.A.2 gives it), the literals without `..` listed against 6.A.2's two); (d) v0.7.1's LN unit in the tree (the GPUI Kit specification's Part I.3: `scripts/check_preset_citations.py` reads the batch's preset comment form, `# platform-facts.md (§2.3): …`, and compares nothing with the facts file), found by its commit or the checker's docstring, so that 6.C.3's new comments are expected to pass it, exit 0 with no finding (U6.1's five checkers show it); (e) the insertion point: the anchor row `primary_text_color` in `docs/platform-facts.md` §2.3 (the rows go after it, F.1), found by its text; (f) the pinned lines of *Lines that must not move*, listed again from every `<file>.rs:N` the gpui showcase's claims cite, the last pinned line of `variants.rs` and of `geometry.rs`; (g) the toolkit versions `Cargo.lock` locks (P2·6's list: egui, egui_extras, egui-winit, egui_kittest, iced, iced_widget, iced_core, iced_runtime, gpui-component, gpui-pre, the GPUI Kit crates, gpui-fast; iced_runtime 0.14.0 and gpui-pre 0.3.12 at `0fa28027`), and every upstream line Step 6's parts and the header cite (`GC/`, `IW/`, `EG/`, `EX/`, gpui-pre, egui-winit, egui_kittest, iced_runtime, iced_core), read in the locked version's source in Cargo's registry (`~/.cargo/registry/src/`), each "holds at `:K`" or its present line there, or "not found" (as the steps 4–5 plan's A4 (b)); where the locked egui is 0.37.0, the specification's 0.37.0 lines are the ones read. No commit, no cargo. | `worker` | the header; Part F; 6.A to 6.I | the seven outputs are in `<record>/anchors-6/`, each table complete; nothing is "not found", or each text not found is settled by §7's rule for a quoted text; the orchestrator copies the summary and step 3's trial outcome into the record |
| M0 | **The measurement's tools** (6.G, *The layout measurement*), beside P0 and A6. In `<record>/measure-6/`: (1) `precheck.py`, B.10's pre-check, copied unchanged from the one steps 4 and 5's run used (its plan's record), else from `<scratch-keep>/batch-2026-11-15/steps45/precheck.py`; (2) `compare6.py`, B.10's comparison from the same place, else `…/steps45b/compare_layout.py`, with items 4 to 6 changed as 6.G's *The comparison* says (the expected added ids; in place of Edit's wrap, `basic.buttons.dialog_default` below the toggle row by its rounded top edge against `basic.buttons.toggle_off`'s rounded bottom edge, a missing id a stop entry, `toggle_off` inside its visible area and `dialog_default` not a stop entry, the three showcases' answers disagreeing a parity entry), and two modes for MA and K5 (rule 19, whose check is relative to the measurement's "after" state, not absolute, since a page that scrolls has elements outside its visible area by design), each taking, as the comparison does, the repository root ROOT as its first positional argument and the element list ELEMENTS (`--elements <file>`, the same option in all three modes, default `<ROOT>/docs/showcase-elements.toml`, as `compare_layout.py`'s fourth argument; MA and K5 use the default, which holds the new ids from U6.1 on; an id the list lacks is exit 2): `--record-state <out> --entries <file>` on one dump folder writes, per kind and entry, the dump's `scale`, the ids of ELEMENTS present in the dump and the ids wholly inside their visible area, as JSON; `--after-only --entries <file> --baseline <state>` on one dump folder checks: the entry list holds every entry of the baseline (exit 2 where one is missing or the list is empty); a dump `<kind>-<theme>-<variant>.json` (the name the capture scripts and the rehearsal both write) for every listed entry and each kind, iced, egui and gpui (exit 2 where one is missing); for an entry of the baseline, every id present then is present now and every id inside then is inside now, each failure a stop entry, each dump's inside test made at its own `scale` (the parity module's `snapped_edges` and `visible_area`, as the comparison makes it) and both scales printed, the state's and the dump's (MA's dumps come from the desktop, the rehearsal's from Xvfb, and whether their scales agree is recorded as not verified); for an entry the baseline lacks (the rehearsal's two own, `material-3` dark and `catppuccin-mocha` dark), presence checked against the baseline's state of the same kind's light entry of that preset (`<kind>-material-3-light`, `<kind>-catppuccin-mocha-light`): every id present there present now (a stop entry where missing), so that an id a kind never records (gpui's `NOT_RECORDED`, `showcase-gpui/tests.rs`: `basic.text_inputs.placeholder.text`, `basic.drop_down.trigger.text` and `basic.drop_down.trigger.arrow` among them) is not asked of it, and the visible-area findings printed as reported findings, never as stop entries; on every dump, items 5 and 6's placement (a stop entry where it fails); across the three kinds of an entry, the measurement's parity check, their answers on that placement agreeing (a parity entry otherwise); each finding printed with its dump, id and class (stop, parity or reported); exit 0 where it found no stop or parity entry, 1 otherwise, 2 for an input fault (also a dump that does not parse, or an id the check needs missing from the element list); and nothing else changed, with the two entry lists beside it, `entries-measure.txt` (B.10's nine) and `entries-rehearsal.txt` (K5's eleven: B.10's nine and the rehearsal's `material-3` and `catppuccin-mocha` dark ones); (3) `block.sh KIND TREE`, B.10's block command with 6.G's folders (`target/measure-6/<TREE>`, OUTPUT `"$PWD/target/measure-6/<TREE>/$kind-{theme}-{variant}.capture.png"`, DUMPS `"$PWD/target/measure-6/<TREE>"`), its nine ENTRIES and its features per kind as B.10 gives them; (4) `window.sh TREE RUN KIND…`, the measurement window of §3.1, run under the asset lock, the gate lock and the cargo slot taken outside it: it writes `RUN/HELD`, waits until `RUN/GO` exists (`until [ -e "$RUN/GO" ]; do sleep 5; done`), then for each KIND in turn makes `RUN/KIND/` and runs `kde-inhibit --power --screenSaver --notifications bash -c 'CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16 bash <record>/measure-6/block.sh "$1" "$2" > "$3/$1/LOG" 2>&1; echo $? > "$3/$1/STATUS"' _ KIND TREE RUN` (no `slot.sh` or lock inside: the window holds the locks and the slot), and exits 0 once every kind has run, whatever their results (each kind's `STATUS` is read, `kde-inhibit` hiding exit codes). No repository change, no cargo. | `worker` | 6.G; steps 4 and 5's B.10 (*The measurement* to *The duration*) | `diff` of `compare6.py` against its source shows only items 4 to 6, the two new modes and the `--elements` option; `block.sh`'s command equals B.10's block (extracted by the worker's own script) but for the folders; `window.sh` run on a trial folder with a stub in place of `block.sh` (a copy that only echoes its arguments and exits 3 for one kind) writes `HELD`, waits until `GO` is touched, then writes each kind's `LOG` and `STATUS` (3 for the stub's failing kind, 0 for the others) and exits 0, its printout in `selftest.txt`; `compare6.py` run on hand-made pairs, its inputs fixed as steps 4 and 5's M5 fixes its own: ROOT is *M0's tree* (below the table: P2·6's commit as files, in `<record>/measure-6/tree/`, which lacks the two new ids), and every run of the self-test, in all three modes, passes `--elements <record>/measure-6/pair/elements.toml` (the default list would exit 2 on every pair); the pairs are written by M0's copy `<record>/measure-6/make_test_pair.py`, which is `…/steps45/make_test_pair.py` (or the copy steps 4 and 5's run used) adapted to the two new ids (its `elements.toml` the tree's list plus the two, their leaves cut to those P2·6's tree's `docs/property-registry.toml` has, as the input cuts its Edit elements' leaves to the registry of its day) and given one flag, `--clean`, that skips the input's two deliberate faults (its macos-tahoe gpui wrap, which pushes one icon, `basic.icons.large`, into the status bar, and its deletion of `basic.icons.toolbar` from egui's ios-13 "after") and nothing else, its `diff` against the input in the record (where that copy has the flag already, it is kept as it is): `python3 -I <record>/measure-6/make_test_pair.py <record>/measure-6/tree <record>/measure-6/pair/` and `python3 -I <record>/measure-6/make_test_pair.py --clean <record>/measure-6/tree <record>/measure-6/pair-clean/`; every exit-0 case below is the clean pair or a state written from it, never the raw pair; `compare6.py` exits 0 on the clean pair (every element inside, the button below the toggle row), 1 with a stop entry for the button above the toggle row's bottom, for the button outside its visible area while `toggle_off` is inside, and for an element missing after, 1 with a parity entry where one showcase answers differently, and 2 for an id the list lacks; with `--after-only --entries <record>/measure-6/pair/entries.txt` (the pair's own entry list, written by M0's `worker` from the `ENTRIES` tuple of its `make_test_pair.py` copy, in `entries-measure.txt`'s form) and `--baseline` a state `--record-state` wrote from the clean pair's "after" folder: on that folder, 0, and 1 on copies where the button is outside its visible area or above the toggle row's bottom; `--record-state` on the clean pair's "after" folder, with `--entries entries-measure.txt`, writes a state whose ids match the hand-made dumps (one id placed outside its visible area there on purpose, so that the state records it present but not inside); then with `--after-only --entries entries-rehearsal.txt --baseline <that state>` on a folder of 33 hand-made dumps named as the rehearsal names them (eleven entries, three showcases), the same id still outside: 0; with one dump removed, 2; with an empty folder, 2; with an entry list lacking one of the baseline's entries, 2; with an empty entry list, 2; with a dump that does not parse, 2; with an id inside at the baseline moved outside in one dump, 1; with an id present at the baseline removed from one dump, 1; with an element placed outside its visible area in a `material-3` dark dump only, 0 with a reported finding; with an id the state holds present for a kind's `catppuccin-mocha` light entry removed from that kind's `catppuccin-mocha` dark dump, 1; with an id the kind never records (`basic.drop_down.trigger.arrow`, absent from the hand-made gpui dumps of both `material-3` entries and so from the state's gpui `material-3` light entry), absent from gpui's `material-3` dark dump, 0; with one kind's button placed above the toggle row's bottom while the other two kinds' stand below, 1 with a parity entry; each run's printout in `<record>/measure-6/selftest.txt` |
*M0's tree*, P2·6's commit as files, from the core worktree (the folder made first
with `mkdir`; the pipe is the command's own; nothing is written into the worktree):

```bash
cd <core worktree> && git archive <P2·6's commit> | tar -x -C <record>/measure-6/tree/
```

### 2.2 Step 6

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| MB | **The "before" dumps** (6.G), on P0's commit (P2·6's, where there is no P0), before U6.1: the pre-check, the three pre-builds, then one measurement window under one asset lock (§3.1): the lock held, the announcement and "measure now", and the three blocks, iced, egui, gpui. No commit. | orchestrator; the maintainer for the desktop | 6.G; §3.1 | the pre-check exits 0; each block's `STATUS` reads 0 and its log's tail shows nine entries done; `target/measure-6/before/` holds 27 dumps (*MB's count* below prints 27); the record holds each block's time, its reruns and the entries whose icon set fell back (the showcase's stderr naming the installed themes) |
| U6.1 | **U6.1**, in two phases and one commit (*U6.1's phases*, below), on the tree MB measured: 6.A, 6.B entire (6.B.3's three §2.3 rows directly after `primary_text_color`, found by its text, A6 (e), with no blank line dropped, F.1, and the two primary rows' KDE cells), 6.C.3, 6.D entire, 6.E entire (6.E.4's README row as the specification has it: `button_default` after step 5's `toolbar_button`, itself after `button_toggle`, and its gloss, ending in ";" as the specification gives it, after `toolbar_button(&resolved, checked)`'s, in the cell's own form; the crate doc's `styles` row with "a dialog's default button" after step 5's "toolbar button"), 6.F entire, and 6.G but the measurement. Every citation it writes names the document, never a line number of a file that changes (S6; the specification's texts give that form). | `implement` | the header; Part F; 6.A to 6.G; 6.I.1, 6.I.1b; A6 (a) to (f); step 3's trial outcome from the record | (0) the three rows' cells and the two changed cells equal 6.B.3's texts (extracted from the specification and compared by the agent's script in `<record>/u6.1/`); the rows stand directly after `primary_text_color` and no blank line was dropped (F.1); (1) the new and changed tests written first and seen to fail: 6.A.2's assertions, 6.A.3's assertion, 6.D.4's three (where the trial passed), `default_button_rest_is_the_stated_fill_and_frame` (where gpui has a receiver), 6.E.3's two, 6.F.4's `t18_a_default_button_is_the_themes_or_none` with each of its cases (the two `focus_stroke` fallbacks among them, each an assertion of its own), 6.G's changed counts (`the_basic_page_has_every_group_in_its_column`, seen to fail by changing its `BASIC_CONTROLS` entry first, `("button (dialog default)", 1)`, before the egui page draws the button, so that the test counts a kind the page lacks); (2) they pass, each named `ok` in the report: `cargo test -p native-theme --lib -- button_validate_widget_extracts_all_fields platform_presets_no_derived_fields property_registry_keys_match_derive_inventory no_inheritance_fields_are_not_inherited every_bundled_preset_source_lints_clean`, `cargo test -p native-theme --test proptest_roundtrip`, `cargo test -p native-theme-egui --all-features --lib -- t18_a_default_button_is_the_themes_or_none every_leaf_has_exactly_one_row_and_every_row_is_well_formed verdict_totals_match_the_specification --nocapture`, `cargo test -p native-theme-iced --all-features --lib -- button_default every_colour_leaf_reaches_a_row_or_is_named`, `cargo test -p native-theme-gpui --features native,watch --lib -- default_button every_colour_leaf_reaches_a_sink_or_is_named every_colour_claim_is_read_at_the_line_it_cites` (`default_button` names tests only in case 2, and is left out in cases 1 and 3; the claim test is `src/showcase.rs:2819`'s); the showcase tests `the_basic_page_has_every_group_in_its_column` of egui (`showcase-egui/tests.rs:5554`) and of iced (`showcase-iced.rs:24446`), which the gate's log does not name when they pass (*The showcase tests the agent runs*, below), each run by the agent before the change and after: egui's seen to fail before (6.G changes its count), iced's, which 6.G does not change, passing after, each `ok` quoted; (3) the generated files written by their commands only: `mapping.md` (`cargo test -p native-theme-egui --lib -- --ignored regenerate_mapping_md`) and `contract-leaves.toml` (`cargo test -p native-theme-iced --all-features --lib -- --ignored regenerate_contract_leaves`), after 6.C.3; the leaves file's diff read against 6.E.3's expected list, each difference explained in the commit body; (4) each count equals A6 (c)'s base plus the specification's delta, both in the report; (5) no pinned line moved (against `HEAD`): `git diff HEAD -U0 -- connectors/native-theme-gpui/src/colors.rs` prints nothing, every hunk of `git diff HEAD -U0 -- <file>` for `variants.rs` and `geometry.rs` starts after A6 (f)'s last pinned line of that file, and `every_colour_claim_is_read_at_the_line_it_cites` passes (S6, *The fourth gate, kept*: a unit that moves a line those claims cite either keeps the line count above it, as this criterion asks, or re-points the claims in the same commit, each named in the commit body); (6) `python3 scripts/check_connector_parity.py` and `python3 scripts/check_showcase_parity.py --check-list` pass; (7) `git diff HEAD` adds no line-number citation of a tracked file that changes (S6; the agent's own check of its added lines, pinned forms such as "at `<sha>`" or an upstream crate's version excepted), with two gated exceptions the specification requires (its *Lines that must not move*): the gpui colour claims at `variants.rs` and `geometry.rs` lines, which `every_colour_claim_is_read_at_the_line_it_cites` reads, and the `HOW` text `(native-theme-gpui/geometry.rs:230, border_color)`, which `every_prose_citation_still_exists` reads; its result in the report; (8) the five checkers pass, `cargo fmt --all --check` passes and the change is staged; (9) the gate passes, with the GPUI Fast legs (in the gate). Commit: `feat: a dialog's default button: the facts rows, the fields, kde-breeze's values, and the three connectors and showcases` (the body names 6.D.2's case, and in case 3 the state 6.D.4's test found different and its two colours) |
| R6.1 | **The reviews of U6.1** (§5), all closed with their fixes committed before MA. | §5 | §5 | §5 |
| MA | **The "after" dumps and the comparison** (6.G), at U6.1's last commit, its review fixes included, once the gate has passed there: the pre-check, the three pre-builds, one measurement window under one asset lock with the announcement and "measure now" once the lock is held and the three blocks (§3.1), then `cd <core worktree> && python3 -I <record>/measure-6/compare6.py <core worktree> target/measure-6/before target/measure-6/after` (the two folders relative to the worktree's root). Exit 0: right after it, the "after" state is recorded for K5 (rule 19), `cd <core worktree> && python3 -I <record>/measure-6/compare6.py --record-state <record>/measure-6/after-state.json --entries <record>/measure-6/entries-measure.txt <core worktree> target/measure-6/after` (per kind and entry, the ids present and the ids wholly inside; written again, over the old file, whenever a rule 19 re-run of MA's blocks makes new "after" dumps), and `rec` follows (no push: the release's first push is K3's). Exit 1: a stop entry is the stop of 6.G, *What a stop means* (§7); a parity entry is a fault, never a question (rule 19; §7, *Parity differences*), fixed and MA's blocks run again, or a toolkit's limit excepted with its reason and source, the maintainer told; the folders are kept. Exit 2: an input fault, fixed in the inputs, the comparison run again. | orchestrator; the maintainer for the desktop | 6.G; §3.1 | the pre-check exits 0; each block's `STATUS` 0; 27 dumps in `target/measure-6/after/`; the comparison's printout and exit status in the record; on exit 0, `<record>/measure-6/after-state.json` written, 27 kind-and-entry records in it |
| rec | **The measurement record** (6.G, *The record*): §10 of this plan filled in B.10's form (the folders, the pre-check's results, each block's time, the reruns, the entries whose icon set fell back, the printout), and, in case 3 of 6.D.2, the plan's status line naming the state 6.D.4's test found different (6.D.2), in one commit of this plan's file only, on top of U6.1's last commit. No push here: the release's first push is K3's, after MG (S2 as amended) and the compatibility run (S4), and carries this commit with every unit; MG changes no sha, so §10's shas stay valid. In case 3, the work goes on with U6.2 while the maintainer's answer is pending, and K3 waits for that answer; where the maintainer decides a request is written, its text is committed before K3 (the maintainer's text into `docs/issues/0004_kde-hand-overrides-upstream-requests.md`, by a `worker`, in a documents-only commit whose gate is the five checkers and `--quick`, §4, the orchestrator reading the diff). The dumps stay in `target/measure-6/` until the release's first push (rule 19), and K3 deletes them. | orchestrator | 6.G; 6.D.2 | `git show --stat <rec's commit>` lists this plan alone, and, in case 3 where a request is written, `git show --stat <request commit>` (its sha in the record) lists `docs/issues/0004_kde-hand-overrides-upstream-requests.md` alone; its gate: the five checkers and `--quick` (a documents-only commit, §4); nothing pushed; `target/measure-6` still holds both trees' dumps |
| U6.2 | **The KDE reader**: 6.C.2 and 6.C.4. | `implement` | the header; 6.C | (1) `test_default_button_is_breezes` and `test_default_button_frame_keeps_qts_precision` written first and seen to fail, and the moved-leaves lists of 6.C.4's table changed first and seen to fail against U6.1's tree; (2) `cargo test -p native-theme --features "$NT_FEATURES" --lib -- kde:: --nocapture` passes, the two new tests, `test_full_mapping_completeness`, `the_derived_colours_are_the_presets_under_breeze` and each test of 6.C.4's second table named `ok`; (3) `cargo test -p native-theme --features kde` passes (CI's Linux leg of the edited module, `ci.yml:212`); (4) the derived-leaves count equals A6 (c)'s base plus three; (5) `git diff HEAD` adds no line-number citation of a tracked file that changes (S6, as U6.1 (7)); (6) `cargo fmt --all --check` passes and the change is staged; the gate passes (in the gate). Commit: `feat(kde): the KDE reader states Breeze's default button` |
| U6.3 | **Outside the code**, once R6.2 is closed and its fixes committed (its dated lines name U6.2's last commit): 6.H.1 under a new `## [0.7.N] - <date>` heading with the release's proposed number (rule 14: never `## [Unreleased]`; the date filled by V), above every other `## [` heading, the release's one section (S5), with its one *Breaking Changes* entry naming both steps' structs as 6.H.1 gives it, 6.H.2 (with its edit of `ROADMAP.md:162`'s count of the iced connector's style functions, found by its text, "has 34 style functions" after step 5, moved in place to the count U6.1 wrote into the iced crate doc's `styles` row, 6.E.4; and the same paragraph's list of the functions by name, `:159-178` at `0fa28027`, gaining "`styles::button_default` a dialog's default button" after step 5's "`styles::toolbar_button` a toolbar's button," entry (itself after "`styles::button_toggle` is a toggle button,", `:171` at `0fa28027`), found by its text, in the list's own form), 6.H.3, 6.H.5, each in its 6.D.2 form from the record. The dated lines of 6.H.5 name the last commit of the unit that made what they state, its review fixes included: item 1 U6.2's, item 3 U6.1's. 6.H.4 has no sentence of its own any more: the release's one status-line sentence is 7.I.4's, written in K7 (§2.5). | `worker` | the header; 6.H; the record's condition and shas | each text equals the specification's block for the condition recorded (extracted and compared by the worker's script in `<record>/u6.3/`), placed by its anchor; each changelog entry one physical line; `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md` equals A6 (c)'s grep count, and `ROADMAP.md:264`'s stated count is unchanged from A6 (c)'s (6.H.2: no change), or the difference recounted and explained; `ROADMAP.md`'s style-function count equals the `(<n> functions)` of the iced `src/lib.rs` `styles` row on the tree, both in the commit body; over the paragraph `:159-178` at `0fa28027` (from "v0.5.9 ships most of it" to "selection list and spinner the same.", found by its text), it names `styles::button_default` once, directly after `styles::toolbar_button`'s entry; each backticked `styles` function name of the paragraph, with or without the `styles::` prefix, exists (its backticked names that are no function are exempt: `native_theme_iced::styles`, a module; `widgets`, a feature; `Style`, a type; `iced_aw`, a crate; at `0fa28027` the fourteen function names all pass the check below) as a top-level `pub fn` of `connectors/native-theme-iced/src/styles.rs` on the tree, each checked with `grep -nE '^pub fn <name>\(' connectors/native-theme-iced/src/styles.rs` (each hit in the commit body), `styles::aw` and `styles::aw::…` exempt as a module path (`pub mod aw;`, `styles.rs:85` at `0fa28027`); and the count it states equals `grep -c '^pub fn' connectors/native-theme-iced/src/styles.rs` less one for `composite_over` (6.E.4's rule; 34 and 33 at `0fa28027`; the indented `pub fn of` at `:1747` is a method in an `impl` and is not counted); `python3 scripts/check_readme_versions.py` passes; `git diff HEAD` adds no line-number citation of a tracked file that changes (S6, as U6.1 (7); a pinned form, "at `<sha>`" or an upstream range at its crate's version, allowed), its result in the commit body; its gate: the five checkers and `--quick` (a documents-only unit, §4). Commit: `docs: the default button in the changelog, the roadmap, the first rationale and the backlog` |

*MB's count* (and MA's, with `after`), from the core worktree:

```bash
cd <core worktree> && ls target/measure-6/before | grep -c '\.json$'
```

**U6.1's phases.** Where A6 recorded that step 3's trial failed (6.D.2, case 1),
there is one phase: the no-receiver branch throughout. Where it passed:

1. **Phase 1**: 6.A, 6.B, 6.C.3, 6.D.2's first bullet
   (`default_button_states`, `default_button`, `geometry::default_button_rest`) and
   6.D.4's three tests; then, through the wrappers, the decisive run (its
   assertion messages name the state and the two colours where it fails):

   ```bash
   cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo test -p native-theme-gpui --features native,watch --lib widgets::tests::default_button_matches_breezes_default_button -- --nocapture
   ```

   The agent reports the result line and, where it fails, the state and the two
   colours, and leaves the tree uncommitted. The orchestrator writes the
   condition into the record: case 2 where the result line shows `1 passed` (gpui
   has a receiver), case 3 where it shows `1 failed` and the failure message names
   a state and two colours, the one 6.D.4's assertion compares (gpui has none). Any
   other failure (a panic, a build error, a failure that names no state and two
   colours) is a fault of the test or the code, fixed and run again, and decides
   nothing; so does a result line with `0 passed` (the filter found no test), the
   run corrected and made again. Nothing that reads the condition (6.D.3, 6.D.5, 6.F.5's
   reason, 6.G's gpui button, `HOW` lines and exception, 6.H) is written before.
2. **Phase 2**, the same agent continued with the condition: in case 3, the
   phase 1 functions, their tests and every part 6.D.2 names are removed first, and
   the no-receiver branch applied; then the rest of the task, and its
   done-criterion. The agent reports once at the end.

*The showcase tests the agent runs.* The gate's log prints a test run's summary
and no `ok` lines when it passes (`scripts/check_release.sh:225-260`, its filter
`TEST_OK_FILTER`, `:181`), so a task that must show a showcase test `ok` (U6.1,
U7.3) runs it itself, by name, in the foreground, in the runner environment the
gate uses (`scripts/check_release.sh:836-848`; prepared first with
`scripts/run_in_runner_env.sh --prepare`):

```bash
cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh scripts/run_in_runner_env.sh test -p native-theme-gpui --features native,watch --example showcase-gpui -- <names>
cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh scripts/run_in_runner_env.sh test -p native-theme-egui --all-features --example showcase-egui -- <names>
cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh scripts/run_in_runner_env.sh test -p native-theme-iced --features iced_aw,watch,native --example showcase-iced -- <names>
```

An exit 75 there (the runner's packages neither fetched nor found) is reported,
and the run is made again once the orchestrator has prepared the environment;
the tests are not run outside it instead. A run the Bash tool kills at 10 minutes
is §4's case.

In case 3 the work goes on to the commit, the reviews, MA and the record commit,
and the release's first push (K3) waits for the maintainer's answer (open point 3;
§7); the later units go on meanwhile, as nothing is pushed before K3 anyway (S2). That U6.1 stays committed is the
case's stated exception to §7's stop rule, by design: the specification applies
the no-receiver branch meanwhile so that the tree stays green (6.D.2, open
point 3); nothing is pushed until the maintainer has answered.

### 2.3 Before step 7

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| P·7 | **Step 7's start**, on the same branch (S5: no branch of its own): U6.3's commit and R6.2's fixes are in HEAD (7.J.1); the hook trial of P2·6 again; §4's warm-up gate where `Cargo.lock` or the toolchain changed since the last `--commit` (S1), else none. Also read: `git cat-file -e` finds `78ee0efd`, `47b3f8c1^`, `71b4c68a` and the eight blobs of 7.C2, and `git rev-list --objects --remotes` holds the four blobs of `71b4c68a`'s files (read 2026-11-15: it does, while `71b4c68a` itself is on the local branches `checkfacts-perf` and `facts-macos-m1b4` only). | orchestrator | §1 | `git rev-parse --abbrev-ref HEAD` prints `v0.7.N-rc1`; `git status --porcelain --untracked-files=no` prints nothing; the hook's refusal quoted in the record; the warm-up gate's result or "none needed" (the lock's and the toolchain's state) in the record; every object found. An object missing: stop (§7) |
| A7 | **The anchors of step 7**, as A6, at P·7's commit: (a) every present text Step 7's parts quote, and every test they change by name; (b) step 1's `mod shade;` and `native-theme/tests/vectors/`, step 6's fields and `DefaultButton`, 7.H.2's call sites (`grep -n -e 'TableColours::of' -e 'table_row_style(' -e 'list_row('` in the iced showcase, `files_table(` in the gpui one) against 7.H.2's lists, and the gpui tests of 7.H.2's table found by name; (c) the counts' bases (7.G's leaf count, the `foundation` and `inputs` totals, `tags.get("source-side gap")` and its doc's number, 7.D.6's `derived_leaves` count and the `Scheme` literals without `..`, 7.E's coverage rows `defaults` and `list`, `ROADMAP.md:264`'s stated count and the `grep -c` of open `[v0.7.x]` entries, recorded apart as A6 (c) has them, the `colors[..]` indices of `arb_list_theme` and `arb_theme_defaults`); (d) as A6 (d): 7.D.3's and 7.D.4's new comments, in the same form, are expected to pass the preset checker, exit 0 with no finding (U7.1's five checkers show it); (e) the hashes and first lines of 7.B.1's four files at `78ee0efd`, by *A7's hash commands* below, against 7.B.1's table; (f) the pinned lines as A6 (f); (g) the two insertion points, as A6 (e): the anchor rows `selection_inactive_background` (§2.1.3) and `header_height` (§2.15's; the name is also a row of the expander's table, `:1675` at `0fa28027`), found by their text; (h) as A6 (g), for every upstream line Step 7's parts cite (gpui-pre's and GPUI Fast's `window.rs` and test platform, gpui-component's `table/state.rs` and `list/list_item.rs`, egui's `raw_input.rs` and `input_state`, egui-winit's `lib.rs`, egui_kittest's `lib.rs`, egui_extras' `table.rs`, iced_core's and iced_runtime's `window` files, iced_widget's `table.rs`), in the locked versions. No commit, no cargo. | `worker` | the header; Part F; 7.A to 7.J | as A6; a hash that differs from 7.B.1's is a stop (§7) |
| C0 | **The capture check's script** (7.H.2, *The capture check*), beside U7.1, and by U7.2 at the latest, so that no more than four agents run at once (§7). `<record>/capture-check/check.py`, a `python3 -I` script: it takes the repository root as its first argument and the download folder as its second (`check.py <repository root> <download folder>`), and loads `scripts/check_showcase_parity.py` by its path under that root for `load_dump`; for each of the twelve captures and its dump in a download folder, it reads `basic.table.row_2`'s rectangle and `scale`, places it in the capture (× `scale`, moved by (W − 1280·`scale`, H − 720·`scale`)), shrinks it by 2 logical pixels a side, and counts the region's pixels within 2 per channel of the active fill (#0064e1 light, #0059d1 dark) and of the unfocused fill (#dcdcdc light, #464646 dark); it reads the folder in `gh run download`'s layout, one sub-folder per artifact (`screenshots-<toolkit>-macos/`, each with its four PNGs and, after U7.3, their four JSON files, the workflow's names); it prints per capture the region and the two shares and its verdict (active at a share of at least one half, inactive likewise, else undecided), and exits 2 where a PNG or a JSON is missing or malformed, else 1 where any capture is inactive (whatever else is undecided: the fix and the repeated run come first, and the undecided captures are read again on the repeated run's), else 3 where any is undecided, else 0, all twelve active. No repository change. | `worker` | 7.H.2, *The capture check* | a twelve-capture fixture in that layout in `<record>/capture-check/fixture/`: each capture a copy of the committed `connectors/native-theme-<toolkit>/docs/assets/macos-macos-<preset>-<variant>.png`, `<preset>` the short name the files carry (`tahoe`, `monterey`: `macos-macos-tahoe-light.png`), each dump hand-made; in gpui's macos-tahoe light one, whose `basic.table.row_2` is x 1075, y 407, 168 × 24 at scale 1 (the box 7.H.2 read, x 1075-1242, y 439-462, less the 32 px title bar), that capture prints "active"; the fixture with every region on its toolkit's selected row exits 0; with one region painted #dcdcdc (light) or #464646 (dark), exit 1; with another region painted over 60 % of it in a colour neither fill is (#ffffff), so that neither share reaches one half: exit 1 while the first stays painted, 3 once only that one is left; with one JSON removed, exit 2, and with one PNG removed, exit 2; each printout in `<record>/capture-check/selftest.txt`. Where a hand-made region of another capture does not lie on its selected row, that dump is corrected from the capture and the correction noted |

*A7's hash commands*, from the core worktree, for each `<path>` of 7.B.1's four
(`native-theme/src/platform/kde_color_common.rs`, `native-theme/src/platform/kcu_tint.rs`,
`native-theme/tests/vectors/kcu_tint.tsv`, `native-theme/tests/vectors/kde_selection_inactive.tsv`),
each output beside 7.B.1's value; the body hashes for the two `.rs` files only:

```bash
git show 78ee0efd:<path> | sha256sum
git show 78ee0efd:<path> | wc -l
git show 78ee0efd:native-theme/src/platform/kde_color_common.rs | tail -n +4 | sha256sum
git show 78ee0efd:native-theme/src/platform/kcu_tint.rs | tail -n +2 | sha256sum
git rev-parse 78ee0efd:<path>
```

### 2.4 Step 7

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| U7.1 | **U7.1**, one commit: 7.A, 7.C entire (7.C.3's three rows, found by their anchors' text: after §2.1.3's `selection_inactive_background` and after §2.15's `header_height`, the name standing in two tables, §2.15's meant; anchors `:1061` and `:1432` at `0fa28027`, each lower on the base by the rows steps 2 to 6 and the caret insert above it, and found by their text anyway), 7.C2's evidence, 7.D.3, 7.D.4, 7.D.5, 7.E, 7.F and 7.G. Every citation it writes names the document, never a line number of a file that changes (S6; the specification's texts give that form, the rewritten macOS preset comments and `macos.rs` comments of 7.D.4 and 7.D.5 among them). The eight evidence files brought in unchanged with `git restore --source=47b3f8c1^ --staged --worktree -- <its four paths>` and `git restore --source=71b4c68a --staged --worktree -- <its four paths>` (the same bytes at the same paths, read 2026-11-15, with no shell write), and `docs/evidence/README.md`'s two rows (7.C2) with the Edit tool. New files: the eight paths of 7.C2's table. | `implement` | the header; Part F; 7.A, 7.C, 7.C2, 7.D.1, 7.D.3 to 7.D.5, 7.E to 7.G; 7.J.1; A7 | (0) the three rows' cells equal 7.C.3's texts (extracted and compared by the agent's script in `<record>/u7.1/`), each directly after its anchor, no blank line dropped (F.1); `git hash-object` of each evidence file equals its blob in 7.C2's table; the two README rows equal 7.C2's text (extracted and compared); `git grep -n -e '[/]tmp[/]' -e '[/]home[/]' -e '[/]run[/]media' -- docs/evidence/os-probe/2026-10-31-run-41195846264 docs/evidence/os-probe/2026-11-06-run-42215651196` on the staged files (the bracketed slashes keep the patterns from being path tokens, so the outside-project guard allows the command; tested 2026-11-15 on `docs/evidence/README.md`, which the guard let run) finds nothing, or what it finds is reported before the commit (the files are added as recorded, `docs/evidence/README.md:8-9` at `0fa28027`); (1) 7.A.2's two new resolve tests, the changed baselines, 7.G's `t18_a_the_inactive_selection_and_header_accessors_are_their_leaves` and the changed counts written first and seen to fail; (2) they pass, each named `ok`: `cargo test -p native-theme --lib -- resolve_phase1_selection_text_fills_selection_inactive_text resolve_phase1_explicit_selection_inactive_text_preserved inheritance_rules_toml_covers_defaults_internal theme_defaults_field_info_matches_baseline default_has_all_none_options validate_fully_populated_returns_ok property_registry_keys_match_derive_inventory no_inheritance_fields_are_not_inherited`, `cargo test -p native-theme --test proptest_roundtrip`, `cargo test -p native-theme --test serde_roundtrip`, `cargo test -p native-theme-egui --all-features --lib -- t18_a_the_inactive_selection every_leaf_has_exactly_one_row verdict_totals_match_the_specification`, `cargo test -p native-theme-iced --all-features --lib -- every_colour_leaf_reaches_a_row_or_is_named`, `cargo test -p native-theme-gpui --features native,watch --lib -- every_colour_leaf_reaches_a_sink_or_is_named selection_and_disabled_helpers every_colour_claim_is_read_at_the_line_it_cites`; (3) the generated files by their commands only, the leaves file's diff read against 7.F's expectation and explained; (4) each count A7 (c)'s base plus the delta; (5) the macOS reader's test, which runs only on CI's macOS legs, is type-checked by the gate's cross-target clippy for `x86_64-apple-darwin` (`--all-targets`, in the gate), read from CI at K3; (6) no pinned line moved (against `HEAD`): `git diff HEAD -U0 -- connectors/native-theme-gpui/src/colors.rs` prints nothing (S6: its comments' old numbers are left as they are), every hunk of `git diff HEAD -U0 -- <file>` for `variants.rs` and `geometry.rs` starts after the last pinned line of that file in A7 (f)'s list (read at P·7's commit), and `every_colour_claim_is_read_at_the_line_it_cites` passes (S6, *The fourth gate, kept*: as U6.1 (5), a moved cited line's claims are re-pointed in the same commit where its line count above is not kept); (7) `check_connector_parity.py` passes; (8) `git diff HEAD` adds no line-number citation of a tracked file that changes (S6, as U6.1 (7)), its result in the report; (9) the five checkers pass, `cargo fmt --all --check` passes and the change is staged; (10) the gate passes, with the GPUI Fast legs, its cross-target legs run, none skipped (in the gate). Commit: `feat: the table header's hovered and pressed fills and the unfocused selection's text colour: the facts rows, the fields, the presets, the macOS reader and the three connectors, with the macOS probe runs the rows cite` |
| U7.2 | **The carried modules and the KDE reader**: 7.B entire, 7.D.2, 7.D.6. The two carried files written with the Write tool from `git show 78ee0efd:<path>` (7.B.2: the project's Write hooks must see them); the two tables brought in byte-exact with `git restore --source=78ee0efd --staged --worktree -- native-theme/tests/vectors/kcu_tint.tsv native-theme/tests/vectors/kde_selection_inactive.tsv` (the same paths in both trees; §6, *Hooks*, the vector tables' exception); before writing a carried file, the implementer runs each pattern of the project's `no-invented-values.sh` and `no-runtime-panics.sh` hooks (`/home/tibi/Rust/native-theme/.claude/hooks/`, the main checkout's, which the worktree has not: read only) over its content with `grep -cE`, as each hook applies it, and stops on a match (7.B.5). New files: `native-theme/src/kde/kde_color_common.rs`, `native-theme/src/kde/kcu_tint.rs`, `native-theme/src/kde/vector_tests.rs`, `native-theme/tests/vectors/kcu_tint.tsv`, `native-theme/tests/vectors/kde_selection_inactive.tsv`. | `implement` | the header; 7.B; 7.D.2; 7.D.6; A7 (a), (c), (e) | (1) 7.B.2's checks print the expected hashes and equal first lines, and the tables' `sha256sum`s equal 7.B.1's, the lines in the commit body; (2) 7.D.6's new tests, the two vector tests and the changed moved-leaves lists written first and seen to fail; (3) they pass: `cargo test -p native-theme --features "$NT_FEATURES" --lib -- kde:: --nocapture`, `kcu_tint_vectors` and `the_readers_inactive_selection_is_the_tables` named `ok` with their row counts, 124 and 70, asserted; (4) `cargo test -p native-theme` with `--features kde`, `--features linux` and `--no-default-features` pass (CI's legs of the module, `ci.yml:211-216`); (5) `bash scripts/check_strict_panic.sh -p native-theme --lib --features "$NT_FEATURES"` passes; a finding in a carried file is a stop (7.B.5; §7), never an allow; (6) `cargo package -p native-theme --list --allow-dirty` on the staged change (it lists files and builds nothing) lists both new tables; (7) `git diff HEAD` adds no line-number citation of a tracked file that changes (S6, as U6.1 (7)); (8) `cargo fmt --all --check` passes and the change is staged; the gate passes (in the gate). Commit: `feat(kde): the KDE reader states Breeze's header sections and the unfocused selection, through the carried KColorUtils::tint` |
| U7.3 | **The showcases and the capture step**, the release's last implementation unit: 7.H entire, in one commit, the workflow edit of 7.H.2 among it. Its change of the Basic table's layout (7.H) comes after MA and is not measured (6.G measures U6.1's last commit); only K5's after-only checks, relative to MA's recorded state, and the asset run's parity step read it. | `implement` | the header; 7.H; 7.J.1's U7.3 | (1) 7.H's new tests written first and seen to fail (the iced header-section and swap tests, the egui table and swap tests, the gpui header-section, swap and activated-window tests); (2) they pass, each named `ok`, by the agent's own runs, the showcase tests among them (*The showcase tests the agent runs*, §2.2: the gate's log names no passing test), each new test run before the change (seen to fail) and after (each `ok` quoted), as are the tests whose new assertions test the change (`the_basic_list_and_table_fills_keep_inside_the_frames_corners` in iced and egui, 7.H.1); a changed test that only gains a step, `the_basic_tables_infos_report_what_it_draws` (its window's activation), is run after the change, its `ok` quoted; (3) 7.H.2's table followed: `the_basic_tables_infos_report_what_it_draws` activates its window, with the comment, and no other test of that table is edited; every activating or deactivating line carries the comment of 7.H.2 with the archived path; (4) `python3 scripts/check_showcase_parity.py --check-list` passes; (5) `.github/workflows/screenshots.yml` parses (`python3 -I -c 'import sys, yaml; yaml.safe_load(open(sys.argv[1]))' .github/workflows/screenshots.yml`, where PyYAML is installed; otherwise reported as not parsed), and its diff touches only the macOS capture step's arguments and the macOS upload's list (7.H.2); (6) no pinned line moved (the claims the gpui showcase's files hold cite the connector's `src/` files, which U7.3 does not edit; were one moved, its claims are re-pointed in the same commit, S6, *The fourth gate, kept*), `every_colour_claim_is_read_at_the_line_it_cites` passing, and `git diff HEAD` adds no line-number citation of a tracked file that changes (S6, as U6.1 (7); a pinned upstream range at its crate's version allowed), its result in the report; (7) `cargo fmt --all --check` passes and the change is staged; (8) the gate passes, with the GPUI Fast legs (in the gate). Commit: `feat(showcases): the table header's states, and lists and tables that follow the window's state` |
| U7.4 | **Outside the code**, once R7.1h, R7.1, R7.2 and R7.3 are all closed and their fixes committed (its dated lines name those units' last commits): 7.I.1 into the release's one `## [0.7.N] - <date>` section U6.3 opened (S5: step 7's *Added* entries after step 6's; no *Breaking Changes* entry of its own, 6.H.1's one entry already naming step 7's structs, checked against 7.A's fields), 7.I.2 (its sentence appended to the release's one paragraph U6.3 opened) with its recount, 7.I.3 (its §8 items only while the reply is unposted, §8 item 4), 7.I.5. 7.I.4 and 7.I.6 are K7's (§2.5). The dated lines name the last commit of the unit that made what they state, review fixes included: 7.I.5 item 1 U7.3's; item 2 (the reader through `kcu_tint` and the showcases' lists among what it states) the later of U7.2's and U7.3's; item 4 U7.1's. | `worker` | the header; 7.I; the record's shas, 6.D.2's case and step 3's trial outcome (7.I.3 item 3) | each text equals the specification's block (extracted and compared in `<record>/u7.4/`), placed by its anchor; the changelog entries one physical line each; the closed entries in `docs/archive/todo_closed.md` verbatim but for the ticked box and the closing line; 7.I.2's count equals `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md` at the commit, in the body; `check_readme_versions.py` passes; `git diff HEAD` adds no line-number citation of a tracked file that changes (S6, as U6.1 (7); a pinned form, "at `<sha>`" or an upstream range at its crate's version, allowed), its result in the commit body; its gate: the five checkers and `--quick` (a documents-only unit, §4; the last unit before MG, but this is the KDE lane's last release, so rule 20 gives it no gated head and no line in the record). Commit: `docs: the header's states and the unfocused selection in the changelog, the roadmap, the first rationale and the backlog` |

### 2.4a The release: the merge, the recount, the version, the close-out

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| MG | **The merge of `<prev-tag>`** (S2 as amended 2026-11-15; S6; rules 16 to 22, 24, 25), once, after U7.4, the release's last unit before the merge, has passed its gate (the five checkers and `--quick`; rule 20 gives the lane's last release no gated head) and every review of both steps is closed with its fixes committed, and once `<prev-tag>` exists; with nothing uncommitted in the worktree and no agent working there; never between a measurement's "before" and "after" runs (MB, MA and `rec` lie before it, so the measurement stands on one base and nothing is measured again: 6.G, *When*; rule 19); nothing of the release is pushed before it. Where P2·6 made the branch from the tag, there is no MG. Here `<base>` is `<lane-pred-head>` (rule 17). **Its order is fixed** (rule 21; this plan's (0) is rule 21 (1)'s items 1 and 2, the tag and ancestry checks, numbered apart; its (1) is rule 21 (1)'s items 3 to 13; its (3) to (7) are rule 21's, run in the order (3), (6), (7), (4), (5); rule 21's (2) is superseded by S6): under D10's order `<prev-tag>`, the caret's tag, already holds this release's base (the caret's gated head) with the caret's facts rows, so the merge brings the caret's post-gate commits, the widget icons and egui 0.37, and no citation is reconciled (S6). **(0)** the tag, fetched with `git fetch origin tag <prev-tag>` only where `git rev-parse -q --verify <prev-tag>` fails, and in any case the first field of `git ls-remote origin 'refs/tags/<prev-tag>'` compared with `git rev-parse <prev-tag>` (the tag ref's own object on both sides, equal for a lightweight tag, as this project's release tags from v0.5.9 on are, and for an annotated one; tags are shared by every worktree, and a release tag has been moved before: v0.7.0's), the two shas in the record, no line printed or a different sha a stop (§7); then, before the merge, the ancestry check (rule 18): `git merge-base --is-ancestor <lane-pred-head> <prev-tag>` exits 0 (the tag holds the lane predecessor's release; it catches a squash or rebase merge by the maintainer), a stop otherwise. **(1) The merge** (rule 21 (1), its items 3 to 13, in this order): item 3, the untracked-overwrite check, in the core worktree: the paths both `git ls-files --others --exclude-standard` and `git diff --name-only --no-renames --diff-filter=A <lane-pred-head> <prev-tag>` list, each listed in the record and compared with the tag's form (*MG's untracked check*, below the table), removed where equal, a stop otherwise; item 4, the merge, with the HEAD before it recorded: `cd <core worktree> && git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>` (rule 17: diff3, so that every conflicted hunk carries its base section; it never commits on its own, so the resolutions and regenerations below always come before the merge commit, conflicted or clean); item 5, `Cargo.lock` and the version comparison: a conflicted manifest (`Cargo.toml`) is resolved first, by item 6's rules, before this item's cargo commands; where the merge left `Cargo.lock` conflicted, `<prev-tag>`'s side is taken (`git checkout --theirs Cargo.lock`) and this release's lock changes are re-applied as rule 21 (1) item 5 gives them, the commands derived from `git diff <base> <the HEAD recorded at item 4> -- Cargo.lock` (no unit of this release changes the lock, so none is expected); then the version comparison of step (3), here so that a stop wastes no build; last, `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo metadata --locked --format-version 1 > /dev/null` passes (it builds nothing, and it catches a lock that merged cleanly but is inconsistent; a failure a stop, §7); item 6, **the resolution**, the orchestrator's: it leaves out `Cargo.lock` and the generated files, never runs `git checkout --conflict=…` on a file it has touched, and resolves every conflicted hunk by both sides' quoted texts (S2), a citation like any other text (S6: an old line number in a conflict takes either side's text, both drift alike, and a new citation has none; where one side re-pointed a line a gate reads by number, S6's fourth gate or rule 25, that side wins, and where both sides re-pointed the same one, the number is the base plus both sides' deltas, confirmed by the warm-up gate; rule 21 (1)), each resolved hunk named in the record, a resolution that changes a text's meaning a stop (§7); `CHANGELOG.md` takes this release's section first and `ROADMAP.md` this release's paragraph after `<prev-tag>`'s (rule 14); a count or verdict tuple a test or gate checks (the leaf counts, the `buttons` and other groups' totals, the verdict tuples, a `derived_leaves` count) is the base plus both sides' deltas, never one side's; a prose count (a count stated in a sentence, as `ROADMAP.md`'s) takes `<prev-tag>`'s side and is named in the record, not a stop (X recounts it); item 8, the generated files (`mapping.md`, `contract-leaves.toml`, every file a generator writes), never resolved by hand, regenerated by their commands on the merged tree under the gate lock and through the wrappers (`flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh …`, rule 2; always a long run, rule 21 (1) item 8, since `<prev-tag>`'s V has bumped the workspace crates in `Cargo.lock`); where the merged tree does not build before VF (a toolkit version the merge brought, step (3)), the generated file is taken from the HEAD recorded at item 4 (`git checkout <that sha> -- <path>`), recorded as pending, and regenerated in VF's code commit, or, where VF makes no commit, by the orchestrator (a long run) in a commit of its own before the warm-up gate, or, where the tree builds only after a warm-up fix, in that fix's commit before the warm-up gate runs again; item 9, `git add` of the resolved and regenerated files, conflicted or clean; item 10, **the marker check** (*MG's marker check*, below the table), which prints nothing but marker-like lines that `<prev-tag>`'s own file has at that place, which are not counted (each named in the record); then `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo fmt --all --check` passes before the merge commit (on a failure, `cargo fmt --all` runs through the same wrappers, the files it changed are re-staged and named in the record, and the check runs again; a file it changes outside the staged set is a stop, §7); item 11, **the staged-path check**: every path of `git diff --cached --name-only --no-renames` is in `git diff --name-only --no-renames <lane-pred-head> <prev-tag>` or is a regenerated file the record names, any other path a stop (§7), named in the record, nothing committed; item 12, `git commit --no-edit --cleanup=strip`, `<merge>` recorded; item 13, the parents check: `git log -1 --format=%P <merge>` prints the HEAD recorded at item 4 and `git rev-parse <prev-tag>^{commit}`, in that order, `grep -m1 '^## \[' CHANGELOG.md` prints this release's heading, and `git status --porcelain --untracked-files=no` prints nothing. Into `<record>/merge/`, for X: the union of `git diff --name-only <base> <merge>^1` and `git diff --name-only <base> <prev-tag>`, the files only the other side changed marked. (Rule 21's step (2) is superseded by S6: no facts citation is re-pointed at MG.) **The steps run in rule 21's order (1), (3), (6), (7), (4), (5)**: R-MG reviews whatever (6) and (7) committed too, and the warm-up gate is the last. **(3) The version comparison and VF** (rules 19, 25): `Cargo.lock`'s versions of the crates P2·6 listed, at `<merge>^1` and on the merged tree (a read of `Cargo.lock`, no gate, made in step (1)'s item 5); a version within P2·6's accepted set (egui 0.36.2 or 0.37.x, patch releases of the others) that changed is reworked here by VF (the row below), before the warm-up gate (a source the new version needs that Cargo's registry lacks is fetched by the orchestrator with `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo fetch`, which builds nothing); any other version is a stop (§7); where nothing changed, the record says "VF: none needed". **(6) The path check** (rule 22 (b)), the one P0 and P2·6 refer to: every token of the specification and this plan that starts with `docs/`, its `:N` suffix and `#…` fragment stripped, checked with `cd <core worktree> && git ls-files --error-unmatch <path>`, and every relative `](…)` link of the two documents, which live in `docs/`, checked from there (`cd <core worktree>/docs && git ls-files --error-unmatch <link>`, its `#…` fragment stripped), brace sets such as `{spec,plan}` expanded into their paths; exempt are paths cited "on <branch>" or "at <sha>", paths a unit of this release creates (its "New files"), placeholders (`<…>`, `…`), glob or regex patterns, URLs, `crate::` links and quoted texts of other files; one that moved to `docs/archive/` is re-pointed by a `worker` in a documents-only commit (made before MG's warm-up gate, which covers it: rule 22 (b)), each edit named in the record and the commit body; one gone otherwise is a stop (§7). Both documents name such paths: the specification its plain paths to the steps 2–3, steps 4–5 and step 1 specifications, the batch's speed rationale and the gate-speed specification, and its link to step 1's; this plan the batch's speed rationale, the gate-speed specification and the GPUI Kit plan. **(7) The survived-the-merge checks** (rule 21 (7)), reading HEAD as it stands at this step, after (3) and (6), and comparing this release's blocks there with their state at `<merge>^1` (the blocks VF rewrote in step (3) are expected differences: the record lists them from VF's commits, and each is compared with the form the specification gives for the new version; a survived check never undoes VF) (`git show <merge>^1:<path>`, not an earlier unit's extraction script, whose blocks a later unit may have changed: U7.4's changelog entries after U6.3's, the one ROADMAP paragraph both write; a prose count resolved to `<prev-tag>`'s side in step (1) is listed for X's recount, not compared), and finding the later tasks' anchors on the merged tree (V's `## [0.7.N] - <date>` heading and the link list's top; every status line K7 sets: the specification's, this plan's and the rationale's; and `docs/archive/README.md`'s table where 7.I.6's row goes): the texts this release replaced are absent, its remaining placement anchors and its new texts present, and the struct-literal and count searches (A6 (c), A7 (c)) run again on the merged tree; and the reply's posted state is read again (rule 16; §8 item 4): where the reply was posted since U7.4's read, a `worker` reverts U7.4's edits of the draft (7.I.3's reply items) in a documents-only commit, and the maintainer is told; where U7.4's read failed and 7.I.3's reply edits waited, this read decides them: unposted, a `worker` makes them now, in a documents-only commit, each text extracted and compared as U7.4's are; posted, they are not made; failed again, they wait on and the maintainer is told; each such commit is made before the warm-up gate, which covers it. **(4) R-MG** (§5), after (6) and (7), where a file was resolved in step (1) or written by step (7), except Markdown that no test parses (so `.rs`, manifests, workflows, configuration files, a TOML a test reads, a script, `docs/platform-facts.md`, a README, which is compiled as doctests and parsed by tests, and `connectors/native-theme-egui/src/mapping.md`, which `mapping_doc.rs` reads, are in), where item 5 resolved `Cargo.lock`, or where VF committed anything (rule 21 (4)): a `review` agent (a `review-hard` agent where those commits change a hard part of a core: a resolver, validation, a runtime, concurrency, `unsafe`; in this tree a hunk in `native-theme/src/resolve/` resolved by hand or changing code) reads `git show --remerge-diff <merge>` with the record's list of resolved hunks, and every commit of steps (3), (6) and (7) (VF's review is always within R-MG); a must-fix is a new commit by the author of the commit it fixes (the orchestrator for a resolution or a regeneration, VF's `implement` or `worker` for theirs, the `worker` of a path re-point or a survived-check fix for theirs). The agents of the steps before (5) run no cargo but `cargo fmt --all --check` (through the wrappers), which builds nothing (a regeneration is the orchestrator's long run), except VF's `implement` agent, which may run one `nice 19` `-p` build or clippy of the crate it rewrote, through `slot.sh` (rule 21). **(5) The warm-up gate** (§4), last, `--commit` with the GPUI Fast legs (rule 21 (5): always, since the merge brings the other side's code, and `<prev-tag>`'s version bump), covering every commit of steps (1), (3), (4), (6) and (7) (the merge, VF's commits, an R-MG must-fix, a path re-point, a fix the survived checks need), none of which has a gate of its own. A red warm-up goes, with the log's failing part, to the author of the unit whose rule the failing test checks (a full struct literal, a count, a mapping), even where the failing file is the other side's (rule 12); one a conflict resolution caused is the orchestrator's; one whose cause is unclear is a stop (§7). The fix of a red warm-up is gated by the warm-up gate run again (rule 12); each commit made after the warm-up gate has passed has a gate of its own (§4's table). The merge changes no sha: every sha the record, §10, the dated lines and the commit messages name stays valid, and no range-diff or re-pointing is made. From here on the release's commit range is `git log <prev-tag>..HEAD` (rule 21: it equals `<base>..HEAD ^<prev-tag>`, and leaves out what `<prev-tag>` brought). | orchestrator; `worker`s for the tracked-file edits; VF's agents; `review` (R-MG) | S2; S6; rules 12, 14, 16 to 22, 24, 25; §1 | step (0)'s `ls-remote` prints a line whose first field equals `git rev-parse <prev-tag>`, and its ancestry check exits 0; `git log -1 --format=%P <merge>` prints the HEAD recorded at item 4 and `<prev-tag>`'s commit; `grep -m1 '^## \[' CHANGELOG.md` prints this release's heading; in the record: the untracked check's paths, the resolved hunks (the prose counts resolved to `<prev-tag>`'s side named apart) and any generated file pending, `Cargo.lock`'s resolution where it conflicted, the version comparison and VF's outcome, the path check (each path with its result, none gone), the survived checks and the reply's state, R-MG's closing, the warm-up gate's result (in run order); the lists of step (1) in `<record>/merge/`; `git status --porcelain --untracked-files=no` prints nothing |
| VF | **The version forms after MG** (rule 25), inside MG's step (3), after the version comparison and before the warm-up gate: for every crate whose version changed within the accepted set (egui 0.36.2 → 0.37.x the expected case, since egui 0.37, v0.7.2, and the other lane's releases normally reach this branch only at MG), a `worker` reads again, in the new version's source in Cargo's registry, every upstream line the specification cites in that crate (A6 (g), A7 (h)), each "holds at `:K`" or its present line, or "not found"; an `implement` agent re-writes the forms the units took on the old version in code, tests and the texts that name a version (the candidates: the egui line citations the specification gives at 0.36.2 and 0.37.0, its code comments and test citations, 6.F.2's `<L>` and `<F>`, 7.H.2's egui and egui-winit lines) from those readings, each only where the content (behaviour, value, API) changed in the new version, never to update a version-pinned line citation that still holds (rule 25; S6), except a line a gate reads by number (the gpui showcase's `HOW` texts and colour claims, read via `CITED_CRATES`: gpui-component, gpui-base, gpui-pre, gpui-pre-wgpu, egui; and the fallback-stack test's gpui-pre lines), which is re-pointed where the new version moved it; it regenerates nothing itself (a generated file step (1) left pending is regenerated by the orchestrator, a long run, into this commit); a `worker` re-writes the documents' forms (`docs/todo.md`, the README texts, the changelog entries), by the same rule. Neither runs cargo but `cargo fmt --all --check`, except that the `implement` agent may run one `nice 19` `-p` build or clippy of the crate it rewrote, through `slot.sh` (rule 21; a source the new version needs that the registry lacks is fetched first by the orchestrator, MG's step (3)); both are reviewed within R-MG (MG's step (4)) and covered by MG's warm-up gate, with no gate of their own, as two commits. Where the comparison shows no change, the record says "none needed" and VF has no commit. | `worker`s; `implement` | MG's comparison; A6 (g), A7 (h) | the readings in `<record>/merge/vf/`; each re-written form listed with its old and new text and the line it cites in the new version, in the commit bodies; R-MG closed over them; or "none needed" in the record. Commits: `fix: the egui <version> forms after the merge`, `docs: the egui <version> forms after the merge` |
| X | **The recount after the merge** (rule 17; S6: no citation is re-pointed), after MG's warm-up gate, and only where there is an MG (rule 17: with no MG, the branch made from `<prev-tag>`, there is no X; nothing was merged, and U7.4 already ran the five checkers): every stated count in every file of MG's union list (`<record>/merge/`: the union of `git diff --name-only <base> <merge>^1` and `git diff --name-only <base> <prev-tag>`) outside the pinned documents (rule 17; the batch Appendix's *The pinned documents*: `docs/archive/`, `docs/evidence/`, `docs/issues/`, the dated `docs/todo_v0.7.1_*` and `docs/todo_v0.7.x_*` documents, the gap maps and spec traces named `todo_v0.7.1_*`, `docs/todo.md`'s dated block) recounted on the merged tree: 6.H.2's style-function count and its named list, 6.E.4's coverage-row count, 7.I.2's `ROADMAP.md:264` count (recounted with `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md`), 7.E's coverage counts (7.G's mapping counts are the tests'), and every prose count MG step (1) resolved to `<prev-tag>`'s side; then the five documentation checkers. A count that differs is corrected in place, in one commit. A documents unit where it changes only prose (rule 11); its gate the five checkers and `--quick` (rule 6). | `worker` | 6.H.2, 6.E.4, 7.I.2, 7.E; A6 (c), A7 (c); MG's lists in `<record>/merge/` | each stated count with its command and figure on the merged tree, a differing one corrected (old and new figure in the commit body); `git diff HEAD` adds no line-number citation of a tracked file that changes (S6, as U6.1 (7); a pinned form allowed), its result in the commit body; the five checkers pass; the gate passes (in the gate; §4's table). Commit (where a count changed): `docs: the stated counts after the merge` |
| V | **The version** (the release mechanics of `a3c10475`), after X: the workspace version and the inter-crate requirements become `0.7.N`, the lock follows, the `<date>` of the one `## [0.7.N] - <date>` heading is filled (and the number, where the maintainer's differs from the proposed one), with its compare link at the top of the link list, in the list's form, from `<prev-tag>`, the release before this one in the maintainer's order (`[0.7.N]: https://github.com/tiborgats/native-theme/compare/<prev-tag>...v0.7.N`, as `CHANGELOG.md:1802`'s `[0.7.0]` line has it at `0fa28027`). Under D0 (b), a minor version, every README's `native-theme*` requirement also moves to the new `major.minor`, as `scripts/check_readme_versions.py` requires (the caret plan's V). Before the close-out's first step, the planning-document check of rule 22 (c), *V's stamped-path check* below the table, prints nothing (code, tests and rustdoc that name this release's planning documents name their archived paths, 7.H.2's comments among them, since K7 moves the documents after the asset run); a hit is re-pointed to its `docs/archive/` path in V's commit and named in its body. | orchestrator | V's rule (§4) | §4's version check (under D0 (b) with the READMEs' requirements); `python3 scripts/check_readme_versions.py` passes; the rule 22 (c) check prints nothing, its command and the paths it read in the record; the five checkers pass; `--commit` passes (rule 6: the release's last code unit's `--commit` is V's, since V changes the manifests and `Cargo.lock`, which reach every crate). Commit: `chore: v0.7.N version` |
| K1 to K8 | **The release's one close-out, with its capture check**, §2.5. | orchestrator; the maintainer for the capture window | 7.J.4; 7.H.2 | §2.5 |

*MG's untracked check* (rule 21 (1) item 3), from the core worktree, for each path
both `git ls-files --others --exclude-standard` and
`git diff --name-only --no-renames --diff-filter=A <lane-pred-head> <prev-tag>` list;
exit 0 (equal) removes the untracked copy, any other exit is a stop:

```sh
cd <core worktree> && git show <prev-tag>:<path> | cmp - <path>
```

*MG's marker check* (rule 21 (1) item 10), from the core worktree, after MG step
(1)'s `git add` and before the staged-path check; it prints nothing (no line of a
conflict marker, `<<<<<<<`, diff3's `|||||||`, `=======` or `>>>>>>>`, alone or
followed by a space, is staged) but marker-like lines that `<prev-tag>`'s own file
has at that place (`git show <prev-tag>:<path>`, read for each line printed), which
are not counted and are named in the record; any other output is a stop, nothing
committed:

```sh
cd <core worktree> && git diff --cached -U0 | grep -E '^\+(<{7}|\|{7}|={7}|>{7})( |$)'
```

*V's stamped-path check* (rule 22 (c)), from the core worktree, with `<stamped paths>`
written out as every path of the `SOURCE_PATHS` array of
`scripts/update_provenance.sh` as it stands on the tree (`:39-71` at `0fa28027`)
together with every path `scripts/update_compatibility.sh`'s `source_paths` prints for
each connector, `gpui`, `iced` and `egui` (`:69-80` at `0fa28027`: each connector's
`Cargo.toml`, `src`, `examples` and `tests`, for egui also `mapping.toml` and the egui
widgets crate's `Cargo.toml` and `src`; the `tests` folders are the ones
`SOURCE_PATHS` lacks), read on the tree, each path once; the stem matches the brace
form `steps6-7-{spec,plan}` too, and the filter works per match:

```bash
cd <core worktree> && git grep -noE '(archive/)?todo_v0\.7\.x_kde-open-items-steps6-7-[a-z{},.-]*' -- <stamped paths> | grep -v ':archive/'
```

### 2.5 The release's one close-out, with the capture check

One close-out for both steps (S5; 7.J.4), after MG, X and V (with no MG, after V alone). In this order; each step
starts only when the one before it holds, but K4 and K5, which run while K3's CI runs
(S4); K1 only when every review of §5 is closed and its fixes committed.

1. **K1.** `cargo audit` passes with the repository's `.cargo/audit.toml`.
2. **K2.** `scripts/update_compatibility.sh run` passes, a long run under the gate
   lock (`cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/update_compatibility.sh run`,
   rule 2; §4); it
   leaves `docs/COMPATIBILITY.toml` and the connector READMEs changed and
   uncommitted until K7. An upstream release that moved, or an advisory, stops
   the run (§7).
3. **K3.** Once K2 has passed (S4): `git push origin v0.7.N-rc1:v0.7.N-rc1`, the
   release's first push (`rec`'s measurement record and every unit with it); then
   `cd <core worktree> && rm -rf target/measure-6` where the folder exists, the dumps
   having stayed until this push (rule 19; the "after" state K5 reads was recorded at
   MA). CI runs
   on pull requests and `main` only (`.github/workflows/ci.yml:6-10`), so at this,
   the release's first push, the orchestrator opens a draft pull request from
   `v0.7.N-rc1` to `main` (approved 2026-11-15, rule 9; §8 item 3), where none is
   open yet, and names it to the maintainer. The run is
   green. It also reads the macOS reader's test from the run: the logs of the jobs
   *Test macOS (macos)* and *Test macOS (macos, watch, system-icons)*
   (`ci.yml:225-226`, each `cargo test -p native-theme` with the `macos` feature)
   each name `read_appearance_colors_maps_selection_and_shadow_as_platform_facts_names`
   with `ok` (`gh run view <id> --log`, the lines kept in `<record>/ci/`).
4. **K4**, while K3's CI runs (S4): the full `scripts/check_release.sh`, a long run
   (§4), fails on the provenance stamp alone (K2 renewed the other). Its warnings are
   named in the record. Expected: the known `cargo outdated` warning, zbus excluded
   and named as excluded (`0fa28027`), whose list of outdated names and versions is
   whatever it lists at the run and is recorded whole (as the caret and step 1 plans
   have it); and, under D0 (a), a 0.7.x number, the semver step (soft), expected to
   name `ButtonTheme`'s three fields, `ListTheme`'s two and `ThemeDefaults`' one and
   nothing else (6.H.1's one break). Under D0 (b), a minor version, which Cargo's
   rules let a 0.x release break, the semver step passes and only the
   `cargo outdated` warning is expected; K8's expected warnings follow the same answer. Any
   other failure or warning is a stop (§7). The cost S4 accepts: a CI run can be red,
   and email the maintainer, for a fault K4 would have shown first.
5. **K5**, while K3's CI runs (it commits nothing), in the cargo slot K4 leaves
   (§7): the three showcases built
   for release with the capture scripts' own commands
   (`scripts/capture_window.sh:440`, with each script's features), under the gate
   lock and through both wrappers (`cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh cargo build …`,
   rule 2), each a long run (§4); and a rehearsal of the parity step's layout
   half on a private X server, with no `kde-inhibit` and through `slot.sh` alone:
   each release showcase started with `--dump-layout`, with the presets the
   capture scripts use (the previous plans' rehearsal, P1's copy
   `<record>/inputs/rehearsal.sh`, in a copy of its own, `<record>/assets/rehearsal.sh`,
   made with the Write tool from it, `diff` against it showing at most two changes: its
   `W=`, which names `/home/tibi/Rust/native-theme/.claude/worktrees/v071`, set to the
   core worktree's absolute path (no change where the core worktree is that path, as
   §1 makes it), and its `ENTRIES`, its six
   (`kde-breeze`, `material-3` and `catppuccin-mocha`, each dark and light) joined by
   those of B.10's nine it lacks, in B.10's form (`adwaita:light:freedesktop:Adwaita`,
   `windows-11:light:default:`, `macos-monterey:light:default:`,
   `macos-tahoe:light:default:`, `ios-13:light:default:`), eleven in all, so that the
   after-only checks read every preset the measurement read; it runs the built
   binaries and no cargo, and takes its output folder as its one argument, which
   receives the dumps (`<kind>-<theme>-<variant>.json`), the logs and `summary.txt`,
   whose last line is `DONE`), held by the shared Xvfb lock (rule 24: one private-X
   rehearsal at a time across both lanes, `flock` waiting for the other lane's), then
   the comparator on the dumps. The run folder `<record>/assets/rehearsal-<n>/` is made
   first (§6, *Run folders*), and its log and `STATUS` lie inside it:

   ```bash
   cd <core worktree> && flock <locks>/xvfb97.lock <tools>/slot.sh bash <record>/assets/rehearsal.sh <record>/assets/rehearsal-<n>/ > <record>/assets/rehearsal-<n>/LOG 2>&1; echo $? > <record>/assets/rehearsal-<n>/STATUS
   cd <core worktree> && python3 scripts/check_showcase_parity.py <record>/assets/rehearsal-<n>/ > <record>/assets/rehearsal-<n>/parity.out 2>&1; echo $? > <record>/assets/rehearsal-<n>/parity.STATUS
   ```

   The first line is a long run (§4); the second reads its dumps once its
   `summary.txt` ends in `DONE`, and the comparator's exit status goes into the record
   with `parity.out`. **The rehearsal is complete** (rule 19) when `summary.txt` holds
   33 `dump=yes` lines (eleven entries, three showcases) and no `dump=no` line: a
   shortfall is run once more, in a new `rehearsal-<n>/`; a second shortfall is a stop
   (§7), with both runs' `summary.txt` and the failed entries' logs to the maintainer.
   Then the measurement's after-only checks on the rehearsal's dumps of the merged tree
   (6.G, *When*; rule 19, since the parity checker counts an element that is not visible
   as ruled out, not as a difference, `scripts/check_showcase_parity.py:1077-1079`, and
   skips a property that fewer than two kinds give, `:1080-1081`, so a missing dump goes unnoticed):
   `cd <core worktree> && python3 -I <record>/measure-6/compare6.py --after-only --entries <record>/measure-6/entries-rehearsal.txt --baseline <record>/measure-6/after-state.json <core worktree> <record>/assets/rehearsal-<n>/ > <record>/assets/rehearsal-<n>/after-only.out 2>&1; echo $? > <record>/assets/rehearsal-<n>/after-only.STATUS`
   (M0's after-only mode, relative to MA's recorded "after" state: the entry list holds
   every entry the measurement used; a dump for every entry and kind; for each measured
   kind and entry, every id present at MA present now and every id wholly inside at MA
   wholly inside now; items 5 and 6's placement of `basic.buttons.dialog_default` below
   the toggle row on every dump; the measurement's after-only parity check, the three
   showcases' answers on that placement agreeing; each dump read at its own `scale`,
   both MA's and the rehearsal's printed, and whether they agree recorded as not
   verified: MA's dumps come from the desktop, the rehearsal's from Xvfb). Exit 0 goes on; exit 2 is an input
   fault (an entry of the measurement missing from the list, an empty list, a dump
   missing for an entry and kind, unreadable or without the ids it needs), fixed in the
   inputs and the check run again; exit 1 with a stop entry, an element pushed out of
   its visible area or missing, is 6.G's stop (§7); a parity entry alone, a
   difference the rehearsal shows before the capture, is reported to the maintainer
   while the capture goes ahead (rule 19), and the asset run's comparator settles it
   after the capture (K6). Two kinds of finding are
   reported to the maintainer and never stopped on: an id that was already outside its
   visible area at MA (the relative check does not test it, and the printout lists it),
   and a visible-area finding on the two entries the measurement did not use
   (`material-3` dark and `catppuccin-mocha` dark, the rehearsal's own), for which no MA
   state exists; every other check holds for those two entries as for the rest (an id
   missing that MA's state holds present for the same kind's light entry of the
   preset, the placement, the parity check). The
   printout, split into stops, parity entries and reported findings, goes into the
   record.

   The comparator compares
   geometry only and runs without a window manager, so a difference it shows before
   the capture is reported while the capture goes ahead (rule 19), and the asset
   run's comparator settles it on the real captures after the capture (K6): a fault
   there, fixed or excepted.
6. **K6.** When K3's CI is green, K4 has shown only its expected failure and
   warnings, and K5 is done (S4: the asset run starts when both are green; rule 9:
   "green" is CI green and the full check's only failure the provenance stamp, which
   only the asset run refreshes), the
   asset script runs once (it needs HEAD pushed and the stamped paths clean), in the
   core worktree, as a long run (§4), in a new `run-<n>` folder made first (`<run>`
   below, `<record>/assets/run-<n>`; §6, *Run folders*), every placeholder written as
   its literal absolute path (§6, *Cargo*). It takes the shared asset lock, the gate lock
   (rules 2, 24) and the cargo slot first, all held for the whole run, its capture
   window included, and waits,
   holding them, for a `GO` file (rule 24). `kde-inhibit` starts only after `GO`,
   inside the slot, and never wraps the waits for the lock, the slot or the
   maintainer. First the orchestrator writes `<run>/run.sh` with the Write tool, one
   line, with no `slot.sh` (the outer command holds the slot):

   ```bash
   CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16 bash scripts/generate_assets_release.sh > <run>/LOG 2>&1; echo $? > <run>/STATUS
   ```

   Then the long run and the Monitor wait:

   ```bash
   cd <core worktree> && flock <locks>/assets.lock flock <locks>/gate.lock <tools>/slot.sh sh -c 'touch <run>/HELD; until [ -e <run>/GO ]; do sleep 5; done; kde-inhibit --power --screenSaver --notifications bash <run>/run.sh'; echo $? > <run>/OUTER.STATUS
   until [ -e <run>/HELD ] || [ -s <run>/OUTER.STATUS ]; do sleep 10; done
   ```

   The first line is the long run; the second is the orchestrator's Monitor wait
   (rule 24): it ends when `<run>/HELD` exists (the two locks and the slot are held) or when
   `OUTER.STATUS` is written first (the run failed before `HELD`: a `flock`, `slot.sh` or
   the shell; read as the run's end, with no desktop asked for). Only once `HELD`
   exists does the orchestrator ask the maintainer for "capture now" (the desktop unlocked and
   untouched for 4 to 6 minutes, announced before it starts), and on the answer runs
   `touch <run>/GO`.
   It holds a slot but does not run under `lowprio.sh`: the desktop is reserved
   for it and the captures are timing-sensitive. `kde-inhibit` hides the
   script's exit code, so the orchestrator reads `STATUS` and the log's tail,
   never the wrapper's status. The orchestrator waits for "Theme-switching GIFs
   generated" (`scripts/generate_assets_release.sh:172`) with the Monitor tool's
   until-loop on the files (never a `pgrep -f` loop):

   ```bash
   until grep -q 'Theme-switching GIFs generated' <record>/assets/run-<n>/LOG || [ -s <record>/assets/run-<n>/STATUS ] || [ -s <record>/assets/run-<n>/OUTER.STATUS ]; do sleep 10; done
   ```

   When the line is there it tells the maintainer that
   the desktop is free (a `STATUS` written first is the run's end, read as such);
   the script then waits for the CI
   screenshots and ends by running `update_compatibility.sh run`
   (`scripts/generate_assets_release.sh:278`), whose builds keep the machine busy
   until `STATUS` is written, which the maintainer is told with the desktop's
   release. No agent edits the tree until the run has reported. A stop
   commits nothing. A parity difference is not a question for the maintainer
   (rule 19; the project's rule: a connector that ignores theme data is a bug to
   fix): one this work's changes explain is fixed (§7, *A fix after the units*) and
   the asset run made again; one that is a toolkit's limit gets a `[[parity.group]]`
   exception with its reason and source (6.G's form; the new button's expected keys,
   6.G, proposed by the run's `--merge`), reviewed (§5); the maintainer is told in
   either case, and K7 waits until it is settled.
   Then the orchestrator opens the new screenshots beside the previous release's
   and tells the maintainer what changed, against 6.G's *Screenshots* (the
   Buttons group's fourth row on every preset, the groups below it moved by
   MA's "moved" lines, Breeze's default button under kde-breeze) and 7.H's (the
   tables' and lists' look, unchanged in an active window). A `STATUS` not 0
   because the CI screenshots run failed (the script's "CI screenshots workflow
   failed", `scripts/generate_assets_release.sh:184`; iced's showcase refuses a dump not
   written in time and fails its job; gpui's and egui's handling was not read, the
   specification's *What was not verified*) is
   7.H.2's failed-dump stop: the run, the job and its log
   (`gh run view <run id> --log-failed`) go to the maintainer before anything is committed. Any other
   `STATUS` not 0 is a stop too, with nothing committed.
7. **C, the capture check** (7.H.2; 7.J.4), after the asset script has reported and
   before any asset is committed, the run id the script printed
   (`scripts/generate_assets_release.sh:122`) naming the folder, so that a
   repeated run's artifacts land in a folder of their own:

   ```bash
   cd <core worktree> && gh run download <run id> --pattern 'screenshots-*-macos' --dir <record>/capture-check/<run id>/
   cd <core worktree> && python3 -I <record>/capture-check/check.py <core worktree> <record>/capture-check/<run id>/
   ```

   Its printout and exit status go into the record. Exit 0: K7 follows. Exit 2 (a PNG or JSON missing or malformed in a
   run whose jobs passed, as when an artifact lacks a file: a failed job ends the
   asset script first, K6): a stop; the run, the artifact and its job's log go to
   the maintainer before any asset is committed. Exit 3: the orchestrator first checks the region in that
   capture (the dump's rectangle, the scale, the title bar's offset) and counts
   again; still undecided, the two shares and the region go to the maintainer, who
   decides. Exit 1 (an inactive window): the capture code makes the window active
   before it captures: in `FrameCapture::poll`'s macOS branch, the first poll asks
   for the activation and returns `None`, so that at least one frame is drawn
   before a later poll captures (the plan's choice between the two places 7.H.2
   names: it adds no timed pause, so no value to invent, and its effect is read by
   the next run's check). How a runner's window is activated was not read (7.H.2):
   the orchestrator settles it from Apple's documentation and the crates' sources,
   with no `unsafe`, through a `design` agent where those do not settle it plainly,
   and tells the maintainer the finding and the change. Where the window can be
   activated only with `unsafe` code, that is a stop for the maintainer (§7): the
   maintainer's earlier approval of `unsafe` (S15, 2026-10-29, the `sficons.rs` form)
   covers AppKit system-control rendering only, not a window's activation. First the asset run's
   outputs are restored (§4, *After an asset run*). An `implement` agent makes
   the change and stages it; its gate is `--commit` (the orchestrator's, §4), formatted
   and clean, with its `x86_64-apple-darwin` legs run (the showcase support's clippy
   and strict-panic, `scripts/check_release.sh:505-506`, `:596-597`, which lint the
   macOS branch from Linux); a `review` agent reviews it; the orchestrator commits it.
   It touches a stamped path (`tools/showcase-support/src`, in
   `scripts/update_provenance.sh`'s `SOURCE_PATHS`), so the close-out resumes at K1
   (rule 15; §4) and K3 pushes it, after the compatibility run, with a new
   capture window. A second inactive result after the fix goes to the
   maintainer, with both runs' shares, and nothing is changed again before the
   answer. Windows is left alone.
8. **K7.** One commit: the assets, both stamps, the READMEs' lines, the changelog
   heading's date (and 7.I.4's date) set to the commit's day where it differs from
   V's, and the archive, the last task (7.I.6): this specification and this plan move
   to `docs/archive/` with their names;
   `docs/archive/README.md` gains 7.I.6's row; 7.I.4's one sentence for the release
   goes into the rationale's status line, with the archive's path; every link between
   the moved documents and to documents that stay is rewritten for the new place; the
   status lines of the specification and this plan say implemented, with the
   release's commit range (`git log <prev-tag>..HEAD`, rule 21, on both paths: after
   MG it leaves out what `<prev-tag>` brought, and where the branch was made from
   `<prev-tag>` that tag is its base);
   §9 holds the whole record (both steps) up to this commit. **The batch's
   rationale** (rule 22 (a): the batch's last release archives it, and every plan
   names that task as conditional on being last): where this release is the batch's
   last in the maintainer's order (as under the release order approved on 2026-11-15, the header), K7 also archives
   `docs/todo_v0.7.1_batch-speed-rationale.md` (`git mv` to `docs/archive/`, its
   status line saying implemented with the batch's releases, its row in
   `docs/archive/README.md`, and its path mentions rewritten by the same rule 22 (a)
   pass and check, its stem `todo_v0\.7\.1_batch-speed-rationale` added to the
   pattern); where it is not the last, it leaves the rationale where it is, and the
   batch's last release archives it.
   New files: the asset files the script wrote that `HEAD` has not (each named from
   `git status --porcelain --untracked-files=all` of the asset folders, in the record).
   **Path mentions** (rule 22 (a), which replaces rule 13: no later-links file is
   written): after the move, every path mention of the moved documents in every
   tracked file outside `docs/archive/` is rewritten for the new place, found with
   `cd <core worktree> && git grep -nE 'todo_v0\.7\.x_kde-open-items-steps6-7-[a-z{},.-]*' -- ':!docs/archive'`;
   documents whose headers date their lines are exempt only for line citations,
   never for path mentions. Left as they are, each named in the commit body: a
   mention that already points into `docs/archive/` in any relative form or as an
   absolute URL (the gpui showcase's test comments of 7.H.2 among them), a citation
   dated "at <sha>", and a path inside another document's instructions that names its
   own future target. **Check**, filtered per match, not per line:

   ```bash
   cd <core worktree> && git grep -noE '(archive/)?todo_v0\.7\.x_kde-open-items-steps6-7-[a-z{},.-]*' -- ':!docs/archive' | grep -v ':archive/'
   ```

   prints nothing but the mentions left by that rule, each named in the commit body;
   and the moved documents' own relative `](…)` links are re-checked after the move,
   from their new folder (`cd <core worktree>/docs/archive && git ls-files --error-unmatch <link>`
   for each, `#…` stripped), every one found, with rule 22 (b)'s exemptions
   (placeholders, quoted texts of other files, URLs; rule 22 (a)). The two documents (three, with the batch's rationale where K7 archives it) move with `git mv`, so that
   `git diff --cached --name-status` lists them as renames (`R`) beside the new and changed files
   (§2, the staged-tree check). Pushed with the same refspec.
9. **K8.** The full `scripts/check_release.sh` at K7's commit, a long run, reports no
   failure (K4's expected warnings still appear, named in the report), and K7's CI
   run is green; both are reported to the maintainer (the summary line, the run's id
   and result), not written into the plan.

Then the orchestrator runs `/home/tibi/.claude/scripts/sweep-target.sh <core worktree>`
and reports the sizes it prints, and names in that report and in the project status
note the pending deletion of `kde-b` and its build folder when this release closes
at the maintainer's merge (§1: this is the KDE lane's last release of the batch),
and that `v071` and its build folder stay. **Merging the pull request,
tagging, publishing, the GitHub releases, the pull requests' titles and
descriptions, posting the reply to the reporter and sending upstream requests are
the maintainer's** and are not part of this plan.

## 3. The measurement (step 6, 6.G)

### 3.1 How a measurement window runs

MB's three blocks run in one measurement window, and MA's in another: one asset lock,
with the gate lock (rule 2) and the cargo slot, is held across the whole window, so that the other lane cannot
take the desktop between blocks (rule 24), and the `HELD` and `GO` pair is the
window's, not each block's.

- **The pre-check** before the window: `python3 -I <record>/measure-6/precheck.py <core worktree>`, exit 0.
- **The pre-builds**, all three before the window's lock is taken, outside it,
  under the gate lock (rule 2) and through both wrappers, each a long run (§4), with B.10's commands: `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh cargo build -p native-theme-iced --example showcase-iced --release --features iced_aw`;
  egui `-p native-theme-egui --example showcase-egui --release --all-features`;
  gpui `-p native-theme-gpui --example showcase-gpui --release`.
- **The window**, a capture session and a long run (§4; the batch's rule 5), in a new
  folder made first (`<run>` below, `<record>/measure-6/<tree>/window-<n>`, with a
  sub-folder per kind; a rerun takes the next `<n>`; §6, *Run folders*), every
  placeholder its literal absolute path. It takes the asset lock, the gate lock and
  the cargo slot first, and waits, holding them, for a `GO` file (rules 2, 24); M0's `window.sh` (§2.1)
  then runs the blocks of the kinds it is given in turn, each under `kde-inhibit`,
  each writing its kind's `LOG` and `STATUS`:

  ```bash
  cd <core worktree> && flock <locks>/assets.lock flock <locks>/gate.lock <tools>/slot.sh bash <record>/measure-6/window.sh <tree> <run> iced egui gpui > <run>/window.log 2>&1; echo $? > <run>/OUTER.STATUS
  until [ -e <run>/HELD ] || [ -s <run>/OUTER.STATUS ]; do sleep 10; done
  ```

  The first line is the long run; the second is the orchestrator's Monitor wait
  (rule 24), which ends when `<run>/HELD` exists (the other lane's asset run or
  capture window may hold the lock until then) or when `OUTER.STATUS` is written first
  (the window failed before `HELD`, read as its end, with no desktop asked for). Only
  once `HELD` exists does the orchestrator make **the announcement** and ask
  the maintainer for "measure now" (the desktop unlocked and untouched: a locked
  screen gives no frames, the runs take the focus; the three blocks, with B.10's
  bounds for one, at least 9 × 6 s and at most 9 × 46 s of sleeps and bounded waits,
  and, for MA's window, MB's measured block times), and on the answer runs
  `touch <run>/GO`. The window does not run under `lowprio.sh`. The orchestrator waits
  for the exit notice, then reads each kind's `STATUS` and its log's tail. A failed
  block (its `STATUS` not 0: the identity check, a window that lost the focus, a
  locked screen) is run again whole, in a new window of its own with that kind alone
  (`window.sh <tree> <run> <kind>`), and the record names the rerun.
- **The folders** `target/measure-6/before` and `…/after` stay until the release's
  first push (K3, rule 19), and while a stop waits for the maintainer.

### 3.2 What it decides

The comparison's exit status (MA): 0 goes on, and its "moved" lines are the list
of what the new row moves, which K6's captures then show; 1 with a stop entry stops
the work before the push (6.G, *What a stop means*), and the maintainer decides; a
parity entry is a fault, fixed or excepted, the maintainer told (rule 19; §7,
*Parity differences*). One rule for
a later change (rule 19; 6.G, *When*), for a fix of this release that can move a
`basic.` element (any change to the Basic page's or a showcase's layout; a
comment-only edit does not): before MG, it re-runs MA's blocks (in new run folders,
on one base) and the comparison against MB's dumps, which stay until K3, the
release's first push, and on exit 0 records the "after" state again (MA's
`--record-state`, over the old file); after MG, it is never compared with the pre-merge "before"
dumps: it re-runs K5's rehearsal and its after-only checks (in a new
`rehearsal-<n>/`) on the fixed tree, and is named to the maintainer. What MG brings
is not re-measured, and K5's after-only checks read the merged tree (§2.5).

## 4. Gates

- **While working**, an agent runs the tests of the module it changes. **Before
  it stages**, it runs its task's targeted done-criterion commands once, in the
  foreground, through the wrappers (§6), each expected to finish well within 10
  minutes; it runs no `--quick`, `--commit`, full check, asset run or GPUI Fast leg (§2,
  *Who runs what*).
- **The gate**, the orchestrator's, on the staged change of every task, by the
  batch's cadence (S1): during a unit, the agent runs only the touched tests (its
  targeted commands); at a unit's end, `scripts/check_release.sh --quick` (as
  `docs/todo_v0.7.1_gate-speed-spec.md` defines it), by the orchestrator, as a long
  run until its time is measured (the first timed into the record); `--commit`
  every third code unit of the release, counted from the last `--commit` (any
  `--commit` restarts the count of three, a `--quick`-gated fix does not count toward
  it: rule 11), on the release's last code unit before its documents and version
  units (U7.3, rule 6), and after a lock or toolchain change (the warm-up gates, MG's
  included, and V, whose `Cargo.lock` lists the workspace crates' new versions); a
  documents unit between U7.3 and V (U7.4, X, a fix's documents) takes `--quick`
  (U7.4 is the last unit before MG, but this is the KDE lane's last release, which
  rule 20 gives no gated head, so U7.4 takes the cadence's gate); a unit the plan
  names structural would take `--commit` too, and no unit before V is (rule 24: no
  unit of this release before V changes a manifest, `Cargo.lock`, a build script,
  the toolchain or the gate scripts; U7.3's `.github/workflows/screenshots.yml` is
  CI's, which `scripts/check_release.sh` does not run; new model fields, modules and
  public API are no reason on their own); a documents-only unit runs the five
  documentation checkers and `--quick`; the full run where the plan has it (the
  close-out), unchanged; then the orchestrator commits exactly the gated tree with
  the author's message (the batch's rule: the orchestrator commits, so that the
  commit is the tree the gate passed). A unit is a code unit when its change reaches
  a crate under `--quick`'s scope rule (`docs/todo_v0.7.1_gate-speed-spec.md`, A.3:
  its own package's files, files a package includes, the manifests, the lock and the
  Cargo configuration; `docs/platform-facts.md` counts as code, rule 11). A documents
  unit stays one when its change is only comments, rustdoc prose, READMEs and
  Markdown documents, even where a crate includes them: it runs `--quick`, whose
  scope (A.3) then reaches those crates, so their doctests run, and it is not counted
  toward "every third". The cadence, task by task (where the branch was made from
  `<prev-tag>`, there is no MG and no X, rule 17):

  | Task | Gate | Why |
  |---|---|---|
  | P0's gate, or P2·6's warm-up | `--commit` | the warm-up gate (and the first timed run); the count starts |
  | U6.1 | `--quick`, with the GPUI Fast legs | the first code unit after the warm-up gate; not structural (rule 24: its new model fields reach no crate `--quick` would not run, its scope rule taking in every dependent) |
  | `rec`, case 3's request commit, U6.3 | the five checkers and `--quick` | documents only |
  | U6.2 | `--quick` | the second code unit |
  | P·7 | `--commit` only where the lock or the toolchain changed (a warm-up gate, which restarts the count) | S1 |
  | U7.1 | `--commit`, with the GPUI Fast legs | the third code unit after the warm-up gate; the count restarts. Where P·7 ran a warm-up gate, U7.1 is the first after it and takes `--quick`, with the GPUI Fast legs |
  | U7.2 | `--quick` | the first code unit after U7.1's `--commit` (the second after P·7's) |
  | U7.3 | `--commit`, with the GPUI Fast legs | the release's last code unit before its documents and version units (rule 6) |
  | U7.4 | the five checkers and `--quick` | documents only; the last unit before MG, but the KDE lane's last release has no gated head (rule 20: no next release reads its line), so it takes the cadence's gate and the record holds no gated-head line |
  | MG's warm-up | `--commit`, with the GPUI Fast legs | a warm-up gate after the merge (rules 17, 21; the legs always, rule 21 (5)), the last of MG's steps; it covers every commit of MG's steps (1), (3), (4), (6) and (7): the merge, VF's commits, the orchestrator's commit of a pending generated file where VF made none, an R-MG must-fix, a path re-point, the reply's revert or late edits, a fix the survived checks need, none gated on its own |
  | VF | none of its own: MG's warm-up gate | rule 21, step (3) |
  | the fix of a red warm-up gate | the warm-up gate run again (`--commit`, with the GPUI Fast legs), never `--quick`; a generated file still pending (MG's step (1)) is regenerated in the fix's commit before the warm-up runs again (rule 21 (1)) | rule 12 |
  | a fix after MG's warm-up gate has passed | as the task it fixes; a documents-only commit the five checkers and `--quick`; a code fix `--quick` (rule 6: before V) | rule 21: each fix after the warm-up gate has a gate of its own |
  | X | the five checkers and `--quick` | a documents unit (rule 17: only the recount and the checkers; rule 6) |
  | V | `--commit` | the lock changes (rule 6) |
  | a review or CI fix before MG | the gate of the task whose files it touches, with the GPUI Fast legs where it is a gpui unit; a `--quick`-gated fix does not count toward the three (rule 11) | — |

  The record names each unit's gate and its place in the count.

  A **gpui unit** is a unit whose diff changes code or strings under
  `connectors/native-theme-gpui/` (the connector or its showcase; rule 11); the GPUI
  Fast legs run beside its gate whatever the gate's mode: U6.1, U7.1, U7.3 and a fix
  of theirs. MG's warm-up runs them too (rule 21 (5): at least wherever the merge
  changed `Cargo.lock` or is a gpui unit; this plan runs them at every MG warm-up).

  Every gate, `--quick` too (gate-speed specification, A.4), runs the cross-target
  checks; a run that skipped one does not count. A failure a `--commit` finds in a commit only `--quick` proved is fixed by
  a new commit, the record naming the commit that brought it (S1).
- **Formatted and clean.** The script runs `cargo fmt --all` in place
  (`scripts/check_release.sh:39-41`, `:518`) and only reports a dirty tree, while
  CI's `cargo fmt --all --check` fails the push. So every task that edits a `.rs`
  file runs `cargo fmt --all --check` before it stages; its gate runs on the
  staged change; `git status --porcelain --untracked-files=no` prints the same
  before and after the gate, and nothing after the commit. The exception is
  K2's uncommitted files, until K7, and the asset run's outputs after K6.
- **After an asset run** (the exception's end). Before a fix to a stamped path made
  after the asset script has run and before the asset commit (C's exit 1, whose fix is
  in `tools/showcase-support/src/`; a stop at K6 whose fix touches a stamped path),
  the orchestrator restores the script's outputs to `HEAD`, `git restore --staged --worktree --`
  on the paths the script's closing line names
  (`scripts/generate_assets_release.sh:297`: `docs/assets/PROVENANCE.toml`,
  `docs/COMPATIBILITY.toml`, `native-theme/docs/assets/`,
  `connectors/native-theme-*/docs/assets/`, `connectors/native-theme-*/README.md`),
  and removes the new untracked asset files under those folders (listed by
  `git status --porcelain --untracked-files=all -- <those folders>` first, each named in the record); then
  `git status --porcelain --untracked-files=no` prints nothing, the fix is made,
  and the close-out is run again from its first step (K1). A fix that touches no
  stamped path follows rule 15, cited, not restated (§4, *A fix the close-out
  needs*; the asset script's `STATUS` tells its two cases apart, and each such
  commit's unstaged exceptions are named in the record). A reformat the gate
  makes outside a task's own files is not folded into that task's commit: it
  becomes a formatting-only commit of its own, which the orchestrator reads.
- **One merge.** The release has one lane (§1) and no lane branches; its one merge
  is MG's of `<prev-tag>` (S2 as amended), gated by MG's warm-up gate; a gate that
  fails on the branch after a fix commit is fixed by the author of the change it
  points at, on the branch, and reviewed again (§5).
- **The documentation checkers** (no cargo): `scripts/check_platform_facts_rows.py`,
  `check_section_refs.py`, `check_preset_citations.py`,
  `check_wrong_safety_nets.py`, `check_todo_citations.py`, from the worktree's
  root, each with no finding.
- **The two release stamps** are stale from the release's first code commit on and
  are no task's done-criterion before K8.
- **The minimum supported Rust version** is checked by CI alone. The briefs say:
  no language or library feature newer than the edited crate's own
  `rust-version`, as it stands on the branch.
- **The version (V):** in `cargo metadata`, the seven crates with
  `version.workspace = true` (`native-theme`, `native-theme-build`,
  `native-theme-derive` and the four connectors) report the release's version,
  and the three `tools/` members keep `0.0.0`; `git diff` of `Cargo.lock` changes
  only the workspace crates' versions.
- **The gate lock** (rules 2, 8, 24): every long run of the orchestrator's that runs
  cargo takes `<locks>/gate.lock`, so that no two heavy runs of the two lanes run at
  once (`slot.sh` takes either of its two slots), wherever a task, a brief or a
  long-run line names one: every `check_release.sh` mode (`--quick`, `--commit`, the
  warm-up gates and MG's included, and the full run, K4 and K8); every other command
  run as part of a gate (the GPUI Fast legs, §2, a CI leg run beside the gate); the
  feature-set builds after a warm-up gate; MG's regenerations (its step (1) item 8);
  a trial's run of the orchestrator's; the measurement's pre-builds and blocks (§3.1,
  under the asset lock as well); the compatibility run (K2); K5's release builds;
  and the asset script (K6, under the asset lock as well). A gate starts as
  `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh …`;
  a run under the asset lock starts `flock <locks>/assets.lock flock <locks>/gate.lock <tools>/slot.sh …`.
  An agent's targeted `-p` commands (§2.2, U6.1's decisive run among them) take only
  `slot.sh` and `lowprio.sh`; so do the commands that build nothing
  (`cargo fetch`, `cargo metadata`, `cargo fmt --all --check`).
- **The full `scripts/check_release.sh`**, unchanged (S1): at K4, while K3's CI runs
  (S4; one failure, the provenance stamp, and the expected warnings) and at K8 (no
  failure; the expected warnings still appear, and are named in the report).
- **Long runs.** The Bash tool kills a foreground command at 10 minutes. Every run
  not known to finish within that is the orchestrator's, as a long run: the gate
  (`--commit`: 5 to 12 minutes a commit, 29 minutes cold, in the batch's earlier
  measurements; `--quick` until its time is measured, the first timed into the
  record, S1), the GPUI Fast legs, the full check (about 23 minutes, §11), the
  asset script (30 to 37 minutes), `update_compatibility.sh run`, the
  measurement's pre-builds and blocks, the release builds of K5, and a
  first build in a new worktree. A long run is started with `run_in_background`,
  from the core worktree (`cd <core worktree> && …`), through the wrappers (or the
  capture form of §6, *Cargo*), its log and a `STATUS` file at absolute paths in
  a new subfolder of `<record>` for that run (§6, *Run folders*)
  (`… > <log> 2>&1; echo $? > <STATUS>`); the orchestrator waits for
  the exit notice, then reads `STATUS` and the log's tail. A wait on a condition
  inside a long run (K6's "Theme-switching GIFs generated") uses the
  Monitor tool with an until-loop on a file condition, never a `pgrep -f` loop,
  which matches itself. The first `--commit` run (P0's gate, or P2·6's warm-up gate where there is no P0) is timed and its time
  written into the record, with whether it ran cold (a new worktree, an empty
  build folder) or warm (the reused worktree's build folder).
- **The warm-up gate.** An agent's targeted commands build the feature sets the
  gate's test legs build (gpui `native,watch`, `scripts/check_release.sh:667`;
  iced `iced_aw,watch,native`, `:739`; egui `--all-features`, `:762`), so before an
  agent that runs cargo is dispatched the orchestrator runs
  `scripts/check_release.sh --commit` on the clean tree, as a long run in its own folder
  (`<record>/gates/warmup-<n>/`), wherever the worktree is new, `Cargo.lock`
  changed since that folder's last gate, or the rustup hook reported a changed
  toolchain, and after MG's merge (S2 as amended); then those builds are warm and an agent's
  commands stay within the Bash tool's limit. At step 6, P0's gate serves where
  there is a P0, as it runs before any agent that runs cargo (A6 and M0 run none),
  and P2·6 runs it where there is none; at step 7, P·7 runs it where the lock or
  the toolchain changed. A red warm-up gate, wherever one runs (P0's, P2·6's,
  P·7's and MG's), follows rule 12: it goes, with the log's failing part, to the
  author of the unit whose rule the failing test checks; where that author is no
  longer at hand (a lane predecessor's unit, at P0 or P2·6), to a new `implement`
  agent with that log and the unit's part of its specification; one a conflict
  resolution caused is the orchestrator's; one whose cause is unclear is a stop (§7);
  the fix is gated by the warm-up gate run again.
- **A targeted command killed at 10 minutes** is reported by the agent with the
  command; the orchestrator runs it as a long run (its own folder under
  `<record>/gates/<task>/`), then continues the same agent with the `STATUS` and
  the log's tail.
- **CI** runs only once a pull request is open: K3 opens a draft one at the release's
  first push (rule 9; §8 item 3). Nothing of the release is pushed
  before MG (S2 as amended). Pushes, each with an explicit refspec: K3 (the release's first
  push, carrying `rec`'s measurement record and every unit, after the compatibility
  run, S4), K7, and a fix that touches no stamped path, by refspec (rule 15, below). The orchestrator watches each run in the background. A
  red run emails the maintainer; no push is made that is expected to be red. If
  one is red, the orchestrator tells the maintainer what failed and why, and the
  failing change's author fixes it (reviewed as in §5) as the lane's next commit.
  **A fix the close-out needs** follows rule 15 (batch-speed rationale, Appendix),
  which this plan cites rather than restates. In this plan's terms: the close-out's
  first step is K1 (`cargo audit`), its push step K3, the asset commit K7, the final full check K8,
  the full check beside CI K4; the stamped paths are `scripts/update_provenance.sh`'s
  `SOURCE_PATHS` and the compatibility stamp's inputs, C's capture-code fix in
  `tools/showcase-support/src/` among them.

## 5. Reviews

Every commit that changes behaviour is reviewed before the next measurement or
push that rests on it: U6.1's before MA, U6.2's before U6.3, step 7's before
U7.4, and every review before V and K1. The reviewer gets the commit range and the specification's header and
parts, never the author's report; it reads the commits (`git show`), not a
working tree, and runs no cargo. A review runs while the lane's next task works,
but R6.1's, which MA waits for, R6.2's, which U6.3 waits for, and R7.1h's, R7.1's,
R7.2's and R7.3's, which U7.4 waits for (their dated lines name those units' last
commits). A must-fix is a new change by the task's author (the same agent,
continued), staged after the commit of the task then running in the lane and
before the lane's next task starts, with the task's done-criterion run again as
it applies to a fix, gated and committed by the orchestrator (§2), and reviewed
by the same reviewer. For a fix, the done-criterion is: the tests the task names
pass (each `ok` quoted), the counts, the pinned-line checks against the fix's own
parent (`git diff HEAD -U0`, a fix that moves no pinned line printing no hunk
there), no line-number citation of a file that changes added (S6; the two gated exceptions of U6.1 (7) excepted, the gpui colour claims and the `HOW` text `(native-theme-gpui/geometry.rs:230, border_color)`), `cargo fmt --all --check`, the staged-tree check
and the gate; not "seen to fail", which a fix of a passing test cannot repeat; a new test the fix adds is seen to fail as any new test. A second must-fix on the same point falls under §7's take-over rule.
Every reviewer, the orchestrator included, also checks that no new code
transcribes an LGPL or GPL source (§6, *The licence rule*), and that the carried
files are the 7.B.2 hashes' bytes with their notice.

"By size": a commit of up to a few hundred changed lines an Opus agent wrote is
reviewed by the orchestrator, a larger one by a `review` agent; the orchestrator
reads `git diff --stat` and says which it chose.

| Review | Commit | Reviewer | Why |
|---|---|---|---|
| R6.1b | U6.1: the model, registry, rules, the facts rows (against 6.B.3's texts), kde-breeze and the presets test; gpui's variant, rest refinement, contract, README and showcase | `review` | the "fill decides" rule in gpui's two functions, 6.D.2's condition applied in every place that reads it, the claims at the lines that read their fields; too large for the orchestrator |
| R6.1c | U6.1: iced's style, rows, pairs and leaves file; egui's accessor, mapping and test; the element list; the iced and egui showcases | `review`, beside R6.1b | each connector's per-state rule against 6.E.2's table and 6.F.2's doc (the focus before the hover in egui's showcase, the pressed button without fill), the regenerated leaves file against 6.E.3's list |
| R6.2 | U6.2 | by size (expected the orchestrator) | two constants, the `mix_qt` split whose body stays, and tests that pin the 16-bit frame; not `review-hard`: the arithmetic is the reader's own, moved, and `test_default_button_frame_keeps_qts_precision` fails on an early rounding |
| R7.1h | U7.1's resolver and validation hunks: `native-theme/src/resolve/inheritance.rs`, `resolve/validate.rs`, `model/resolved.rs`, `model/defaults.rs` | `review-hard`, narrow | the maintainer's delegation table names the resolver and validation: a new inheritance rule and a required resolved field whose order against `selection_text_color`'s resolution decides every theme's value. Small, so the Fable review is short; it runs first, and R7.1 follows it |
| R7.1 | the rest of U7.1 | `review` | the facts rows against 7.C.3's texts and the evidence files' hashes in the commit body, values across the presets, the macOS reader's one line and its comment (document-only citations, S6), three connectors' accessors and contracts, egui's counts |
| R7.2 | U7.2 | `review` | not `review-hard`, judged: the carried files are byte copies checked by hash (the reviewer reads the notice and the declarations), the tint's arithmetic is theirs and the vector test checks the reader's rule against all 70 rows of KColorScheme's table; what is new is a three-branch rule and a boolean parse, each case a test (7.D.6) |
| R7.3 | U7.3 | `review` | the window-state plumbing in three showcases, the signature changes with every caller, the tests that activate or deactivate a window, the workflow edit |
| R-MG | MG's step (4), after (6) and (7): `git show --remerge-diff <merge>` with the record's list of resolved hunks, VF's commits (VF's review is always within R-MG), a pending generated file's commit and the commits of steps (6) and (7), where a file was resolved in MG's step (1) or written by step (7), except Markdown that no test parses (a README and `connectors/native-theme-egui/src/mapping.md` are in), where item 5 resolved `Cargo.lock`, or where VF committed anything (rule 21 (4)); every commit of steps (3), (6) and (7) | `review`, reading commits, running no cargo (rule 21); `review-hard` instead where those commits change a hard part of a core (a resolver, validation, a runtime, concurrency, `unsafe`: here a hunk in `native-theme/src/resolve/`, the maintainer's delegation table, resolved by hand or changing code); one Fable agent at a time | resolutions by both sides' quoted texts and both sides' deltas, `Cargo.lock`'s re-applied changes, VF's re-written forms against the new version's readings; a must-fix is a new commit by the author of the commit it fixes, before the warm-up gate, which covers it |
| — | P0, rec, U6.3, U7.4, X, MG's merge commit and the Markdown-only commits of its steps (6) and (7) (a path re-point, the reply's revert or late edits) where R-MG's trigger is not met (step (4); where it is met, R-MG reads every commit of steps (3), (6) and (7)), V, K7, a formatting-only commit | the checkers, then the orchestrator reads the diff | data and texts the specification fixed; the extraction scripts and the verification compare them |
| — | a capture-code fix (§2.5) | `review` | it changes the shared capture code all three showcases use on macOS |

At most one Fable agent runs at a time, in both lanes together (S3): R7.1h, R-MG's
`review-hard` case, a `design` agent of §2.5 and an `implement-hard` agent never
overlap.

## 6. Rules every agent brief carries

- **Inputs.** The brief gives the header and the task's parts; the parts are the
  whole task: nothing beyond them, no refactoring, no formatting churn. A needed
  change the parts do not name is reported, not made. Every edit is placed by its
  anchor (the quoted text, the key, the test's name), a count is the tree's value
  plus the specification's delta, and a place is found by the text it quotes,
  never by the number the specification gives; a citation the agent writes names
  the document, with a quoted phrase where that helps, never a line number of a
  tracked file that changes (S6; an upstream crate's line at its version stays);
  existing line-number citations are left as they are; where a quoted text does not
  stand, the agent stops that edit and reports it, and §7's rule for a quoted
  text decides. The lines the gpui showcase's colour claims
  pin do not move (*Lines that must not move*).
- **The worktree.** The brief names the core worktree by its absolute path; every
  command starts with `cd` into it or uses `git -C` and `--manifest-path`. The
  main checkout (another branch, `v0.8.0-rc1`'s work) and every other worktree
  have the same relative paths: nothing is read, run or written there (but U7.2's
  reading of the main checkout's hook scripts in `.claude/hooks/`), the parked
  branch's files are read with `git show 78ee0efd:<path>` from the core worktree,
  and a check that passes elsewhere proves nothing. The report quotes
  `git -C <worktree> log -1 --oneline`.
- **The licence rule (the maintainer's D10, not the rationale's):** no code from
  an LGPL or GPL source. Breeze (`breezehelper.cpp`, `breezestyle.cpp`, its colour
  schemes), KColorScheme (`kcolorscheme.cpp`), kguiaddons' `KColorUtils`, kconfig
  and Qt are read for facts only. A function such a source is cited for is written
  only through the clean room: every implementer brief (U6.1, U6.2, U7.1, U7.2,
  U7.3 and any fix of their code) says that the implementer opens none of those
  sources; the specification's description and its tests (6.C.1's values, 7.D.1's,
  the two vector tables) are the clean room's description and vectors. The
  carried `kde_color_common.rs` and `kcu_tint.rs` are the project's own clean-room
  code from the parked branch, written there from behavioural descriptions and the
  vector tables, and are carried byte for byte under 7.B.2's notice, never edited;
  `P8 kde_selection_inactive.rs` is the project's own too, and the reader's rule is
  written from 7.D.2's description and checked by its table. gpui-component,
  gpui-pre and GPUI Fast are Apache-2.0, egui and iced MIT or Apache-2.0: read for
  facts; a port of their code would keep a file-level notice (none is planned).
- **Hooks.** Source files are edited with the Edit and Write tools only, with
  three exceptions: the evidence files of U7.1, brought in by `git restore`; the
  two vector tables of U7.2, brought in byte-exact by
  `git restore --source=78ee0efd` (the carried `.rs` files are written with the Write tool, so
  that the hooks see them); and the generated `mapping.md` and `contract-leaves.toml`,
  written by their commands (at MG and VF too). MG's resolutions are made with the
  Edit tool. The project's
  hooks block invented values and panic-prone code; a blocked edit is obeyed,
  never bypassed, and blocked content is never written another way (shell, `sed`,
  a here-document, a script). A block on content the specification requires stops
  the task (§7).
- **Code.** No `unwrap`, `expect`, `panic!`, panicking index or unchecked
  arithmetic in non-test code; no `unsafe`; no public item removed; no invented
  value. A strict-panic finding in a carried file, or a hook refusing one, stops the
  task; an ordinary clippy finding there is fixed in the port and named in the commit
  body (7.B.5).
- **Cargo.** Every cargo command, and every script that runs cargo, goes through
  `<tools>/slot.sh <tools>/lowprio.sh <command>`, an agent's in the foreground
  (its targeted commands, §4), the orchestrator's long runs in the background
  (§4); an environment variable goes before the wrappers
  (`VAR=… <tools>/slot.sh <tools>/lowprio.sh cargo …`), since `lowprio.sh` ends in
  `exec "$@"`. Every long run of the orchestrator's that runs cargo takes
  `flock <locks>/gate.lock` before the wrappers (§4, *The gate lock*, rule 2); an
  agent's targeted `-p` commands take only the two wrappers. The capture sessions (the measurement windows, the asset runs) run
  under the asset lock, `slot.sh` (taken with the lock, outside the session) and
  `kde-inhibit --power --screenSaver --notifications`, but not `lowprio.sh`, their
  builds first through both wrappers; since `lowprio.sh` is what sets the build and
  test thread caps, a capture session's command inside `kde-inhibit` (K6's
  `run.sh`, `window.sh`'s `bash -c` per block) begins with
  `CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16` (K6, §3.1), so that the
  cargo it starts (the asset script's builds and its closing
  `update_compatibility.sh run`) keeps the gate's limit. A run on a private X
  server (the rehearsal) uses `slot.sh` and no `kde-inhibit`. In every command a
  placeholder (`<tools>`, `<record>`, `<core worktree>`) is written as its literal
  absolute path, never with `$HOME` or `~`. A run longer than 10 minutes is a long
  run (§4). `build-dir`, `CARGO_TARGET_DIR` and
  `--target-dir` are never set; `cargo clean` is never run. At most two cargo
  processes at once in total, which the two slots enforce; in this plan only the
  core lane builds.
- **Reach.** No `find /` and no other search of the whole file system. Reads and
  writes stay inside the core worktree, Cargo's home and build folder (the
  registry for upstream sources), the session's scratch folder for its own
  temporary files, `<record>`, `<locks>` (written by `slot.sh` alone),
  `<scratch-keep>/batch-2026-11-15/` (read only), the record folder of steps 4
  and 5's plan, `<scratch-keep>/todo_v0.7.x_kde-open-items-steps4-5-plan/` (read
  only, M0 and P2·6), the caret's record folder, `<scratch-keep>/todo_v0.7.x_caret-blink-plan/`
  (read only, P2·6's rule 20 line), and the main checkout's
  `/home/tibi/Rust/native-theme/.claude/hooks/` (read only, U7.2's pre-check).
  Nothing appears on the maintainer's desktop but the orchestrator's capture
  sessions.
- **Work.** A new test is written first and seen to fail. A changelog entry is one
  physical line. The agent runs its targeted commands in the foreground, each
  expected to finish well within 10 minutes, and never `--quick`, `--commit`, the
  full check, an asset run or a GPUI Fast leg (§4); it stages its own paths by name,
  does not commit or push, adds no attribution line, and reports once: the staged
  files (`git diff --cached --stat`), its commit message, each done-criterion item it
  ran with its command and result line, and everything it was unsure of (U6.1 reports twice, §2.2). The orchestrator runs the gate and commits with that message; a red
  gate comes back to the agent, continued, with the log's failing part.
- **Something the maintainer must hear before a commit** (a hash that differs, a
  value that differs from the specification's, a quoted text whose present form
  changes the edit's meaning): the agent says so in its report, and the
  orchestrator commits nothing of it before §7's stop rule lets the work go on.

**`<scratch-keep>`** is `/home/tibi/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/`.
**`<record>`** is `<scratch-keep>/todo_v0.7.x_kde-open-items-steps6-7-plan/`:
`record.md`, `tools/` (`<tools>`), `docs/`, `inputs/`, `anchors-6/`,
`measure-6/`, `u6.1/`, `u6.3/`, `anchors-7/`, `u7.1/`, `u7.4/`,
`capture-check/`, `ci/`, `assets/`, `gates/`, `merge/`. **`<locks>`** is `<scratch-keep>/slot-locks/`.
**`<cargo-build>`** is Cargo's build folder root. The briefs give absolute paths;
no committed file names them. **The inputs** copied by P1 into `<record>/inputs/`
from `<scratch-keep>/batch-2026-11-15/` (a session's scratchpad does not outlive
it): `steps45/precheck.py`, `steps45/make_test_pair.py`,
`steps45b/compare_layout.py` (B.10's drafts; M0 takes the versions steps 4 and 5's
run used where its plan's record kept them) and `gates/rehearsal.sh` (K5).

**The record.** `<record>/record.md` holds the dated results, shas, counts and
answers as the work goes; only the orchestrator writes it, agents report. It is
copied into this plan's §9 only in the close-out commit (K7, both steps' parts), so that no tracked file is edited mid-work and the clean-tree
checks hold. The specification asks one thing of the repository mid-work: the
measurement record, a commit of the plan's file of its own (`rec`, §10).

**Run folders.** Every long run that can be repeated writes into a new subfolder
of its own, never into a folder an earlier run used: the asset runs into
`<record>/assets/run-<n>/` (`run.sh`, `LOG`, `STATUS`, `OUTER.STATUS`, and the lock's `HELD` and `GO` files, §2.5),
the measurement windows into `<record>/measure-6/<tree>/window-<n>/` (`HELD`, `GO`, `window.log`, `OUTER.STATUS`, and a sub-folder per kind with its `LOG` and `STATUS`, §3.1) and their
pre-builds into `<record>/measure-6/<tree>-build-<kind>-<n>/`, the gates into
`<record>/gates/<task>/run-<n>/` (with `LOG` and `STATUS` for `--quick` or `--commit`, and
`FAST-TESTS.log`, `FAST-TESTS.STATUS`, `FAST-SHOWCASE.log` and
`FAST-SHOWCASE.STATUS` for the two GPUI Fast legs where the gate has them, and
`warmup-<n>/` for §4's warm-up gate), and each kind of the close-out's runs into a
counter of its own under `<record>/assets/`: `full-<n>/`, `compat-<n>/`,
`build-<kind>-<n>/`, `rehearsal-<n>/`, the asset runs `run-<n>/`; MG's merge log, its conflicts, the intersection list, the
version comparison and the path check into `<record>/merge/`; each `<n>` counts from 1, and the record
names each folder with its run. So a `STATUS` file left by an earlier run never
ends a Monitor wait early and is never read as the new run's result.

## 7. Limits and stop rules

- **Shared with the second lane (S3).** The machine's limits hold for this lane and
  the batch's second lane (the widget icons alone, in their own worktree; S3 as amended)
  together: at most two cargo processes (one heavy run, and one `nice 19` targeted
  build in the other lane), the heavy runs serialized by `<locks>/gate.lock` (rules 2,
  8) and the cargo commands by the two `slot.sh` slots in `<locks>`, at most four agents (six where two run no cargo) across both lanes, and
  never two Fable agents. One at a time across both lanes (rule 24): the asset run,
  the capture window (this release's MB, MA and K6 included) and the private-X
  rehearsal (`rehearsal.sh`, Xvfb display `:97`, this release's K5), each held by a
  shared lock in `<locks>`: `flock <locks>/assets.lock …` around the asset run and
  every capture window (MB's and MA's blocks, K6), `flock <locks>/xvfb97.lock …`
  around the rehearsal (§3.1, §2.5), so that a lane waits mechanically. The asset lock,
  the gate lock and the cargo slot are taken before the maintainer is asked: each measurement window
  (MB's three blocks together, MA's together, their pre-builds done before) and the
  asset run start under all three and wait, holding them, for a `GO` file; the maintainer is
  asked for "measure now" or "capture now" only once the run's `HELD` file exists, and
  `touch <run>/GO` starts the capture (§3.1, K6). The counts below are this lane's share: where the
  other lane's agents would pass the total, this lane's beside tasks wait. Files
  both lanes edit (`CHANGELOG.md`, `ROADMAP.md`, `docs/todo.md`, the showcases)
  meet at MG's merge, by quoted text (in `CHANGELOG.md` this release's section first,
  rule 14).
- **Agents at once:** at most four, within the maintainer's limit (four, or six
  where two run no cargo), both lanes counted, of which only the lane's agent runs
  cargo here. This lane's peaks:
  beside P0, A6 and M0, two `worker`s without cargo (the lane's task is the
  orchestrator's); beside U6.2, none (R6.1 is closed before MA), and R6.2 runs
  while the lane waits for it (U6.3 follows its close); beside U7.1,
  A7's report closed and C0 a `worker` without cargo; beside U7.2, R7.1h, then
  R7.1 (one reviewer at a time, §5), and C0 where it has not run yet, beside the
  lane's `implement` agent: three. R6.1b and R6.1c run together while the lane waits for
  them (MA). At most one Fable agent at a time, both lanes counted (§5).
- **A gate failed twice** by the same task is not dispatched a third time: the
  orchestrator takes the task over, re-specifies it, or sends it once as
  `implement-hard`, and says which.
- **The specification's stops**, each with nothing committed by the task that
  met it, nothing pushed after it until the maintainer has been told and, where
  the rule that names the stop asks the maintainer, has answered: a quoted text
  that does not stand (A6, A7, any task; settled as *The batch's stop rule* below
  says); a planning document that differs from its approval copy (P0); `<lane-pred-head>` not shown gated by the caret's
  record's rule 20 line, or steps 4–5's gated head or steps 4–5's `rec` commit not an
  ancestor of the branch (P2·6), or `<lane-pred-head>` not an ancestor of `<prev-tag>` before MG (rule 18); a toolkit version outside P2·6's accepted set after MG, or at P2·6 against `0fa28027`'s lock on either path (rule 19); a
  path of the two documents gone otherwise than to `docs/archive/` (MG, or P0 or P2·6 on the no-MG path; rule 22 (b)); an untracked file the merge would overwrite that differs from the tag's (MG, rule 21); an object or a hash of 7.B.1 or 7.C2 not as stated (P·7, A7,
  U7.2); the measurement's stop rule (MA, 6.G; its parity entries are faults, not
  stops: rule 19, *Parity differences* below); K4's failure or warning other than the
  expected ones (K4); K5's second shortfall of the rehearsal (K5); K5's after-only
  checks exiting 1 for an element pushed out of its visible area (K5); case 3 of 6.D.2,
  before the release's first push, K3 (open point 3: the orchestrator sends the maintainer 6.D.4's
  state and colours, and the maintainer decides whether a request is written and
  what it says; the case's stated exception, §2.2: U6.1, its no-receiver branch
  applied, stays committed, by design, and only the push waits); a strict-panic finding
  in a carried file, or a hook refusing one (7.B.5; an ordinary clippy finding is fixed in the port); a capture-code fix that could
  activate the window only with `unsafe` code (§2.5, C); a capture check's failed dump
  or undecided result (§2.5); a merge conflict
  whose resolution changes an edit's meaning (MG); a conflict marker staged at the
  merge (MG step (1), item 10); a `Cargo.lock` re-application whose diff against
  `<prev-tag>`'s lock shows more than this release's packages (MG step (1), item 5);
  a tag sha on `origin` that differs from the local one (MG step (0)); a staged path at the merge that
  the tag's side did not change and that is no regenerated file named in the record
  (MG step (1)); a red warm-up gate (P0, P2·6, P·7 or MG) whose
  cause is unclear (rule 12; §4).
- **Parity differences** (rule 19), the measurement's parity entries and, in K5, the
  asset run's comparator's after the capture (K6): never a stop and never a question.
  A difference K5's rehearsal shows before the capture is reported while the capture
  goes ahead (K5). One this work's
  changes explain is fixed (*A fix after the units*, below) and the run made again;
  one that is a toolkit's limit gets a `[[parity.group]]` exception with its reason
  and source, reviewed (§5); the maintainer is told in either case. An element pushed
  out of view whose placement is a design choice the maintainer made stays a
  question.
- **The batch's stop rule.** A stop ends when the maintainer has answered where
  the rule asks; otherwise the work continues once the maintainer has been told.
  A quoted text that no longer stands is found in its present form by the
  orchestrator: where the edit's meaning is unchanged, the edit is made at the
  found text and the record names it; where it is not, the maintainer is asked.
- **A task that needs more than its parts name** (a model field, another crate, a
  decision the rationale did not take) stops, and the maintainer is asked. So does
  an upstream move during the work (a new gpui-component, GPUI Fast, egui or iced
  release, a new advisory): it is reported with what it costs before anything is
  changed for it.
- **A fix after the units** that a review, CI, the full check, the rehearsal, the
  measurement or the asset run demands is made by an `implement` agent (a
  documentation fix by a `worker`), reviewed as in §5, and passes the
  done-criterion of the task whose files it touches, writing no line-number citation
  of a file that changes (S6); a dated line that names the unit it changes is
  re-pointed to the commit that now holds what it states. A fix
  of this release that can move a `basic.` element (any change to the Basic page's or
  a showcase's layout; a comment-only edit does not) follows §3.2's rule (rule 19):
  before MG it re-runs MA's blocks, after MG it re-runs K5's rehearsal and after-only
  checks and is named to the maintainer. A fix the close-out needs follows rule 15
  (§4). The maintainer is told what stopped it, what was changed, and whether
  another capture window is needed.
- **The desktop** is asked for when everything before it holds: MB's and MA's
  measurement windows (open point 2), K6's one capture window, and a repeated
  capture window after a capture-code fix.

## 8. The maintainer's part

**With the approval of this plan, before any task:**

1. **D0: the version** (open point 1). `0.7.N` for the release of both steps (S5
   settles that the two steps are one release, and S2 its base); the commits of the
   step 3 and step 5 work P2·6 checks. Under D0 (b), a minor version: the plan holds
   with that number, the branch name the maintainer gives, and the changelog heading
   of 6.H.1 as `## [0.N.0]`. Under D0 (c): the release that makes the model's input
   structs `#[non_exhaustive]` comes first, specified apart; this plan waits for it,
   the *Breaking Changes* entry goes, and the `proptest_roundtrip.rs` literals become
   assignments (open point 1), which amends the specification and this plan before
   P0.
2. **The desktop**: MB's and MA's measurement blocks (open point 2, as §3.1
   announces them) and K6's one capture window (4 to 6 minutes), and one more for
   each rerun §7 names; each asked for when everything before it holds.
3. **The pull request** (settled 2026-11-15: the maintainer approved opening a draft
   pull request at a branch's first push; rule 9): the orchestrator opens a draft pull
   request from `v0.7.N-rc1` to `main` at the release's first push (K3), and names its
   title and description to the maintainer. CI runs only on it (`ci.yml` runs on
   pushes to `main` and on `pull_request`).
4. **The reply to the reporter**: the maintainer is not asked whether it has been
   posted; U7.4 reads it at its start with
   `cd <core worktree> && gh issue view 4 --comments` (rule 16): posted means a
   comment by the repository owner carrying the draft's opening sentence
   (`docs/issues/0004_kde-hand-overrides-rationale.md` §8); the record holds what it
   found. Where posted, the draft is not edited (7.I.3); where the read fails, the
   failure is reported to the maintainer and 7.I.3's reply edits wait, the rest of
   U7.4 going on (the reply waits for the release of steps 4 and 5, S5). MG reads the
   state again in its survived checks (step (7), rule 16): where the reply was posted
   since U7.4's read, a documents-only commit reverts U7.4's edits of the draft, and
   the maintainer is told.
5. **What this plan adds to the specification:** A6 and A7; M0's adapted scripts
   and their self-test, its after-only mode among them; U6.1's two phases; C0's
   script and its self-test; MG's order with its path check, version comparison, VF
   and survived checks; X after MG, only the recount and the five checkers (rule 17;
   S6); each unit's own check that it adds no line-number citation of a file that
   changes (S6); V with its stamped-path check;
   7.I.4's one sentence in K7; a parity difference at K6 and how it was settled
   (rule 19: fixed and the asset run made again, or a toolkit's limit excepted with
   its reason, reviewed; the maintainer told either way); the parity rehearsal with the after-only checks, relative to MA's recorded "after" state, and their two reported, not stopped, kinds of finding (an id already outside its visible area at MA; a visible-area finding on the rehearsal's own two entries, `material-3` dark and `catppuccin-mocha` dark); K7's
   push; the place of the window's activation, should the capture check need it
   (§2.5); the archive in K7's commit; and the gate of each unit (§4's table: no unit
   before V structural; U7.4, the last unit before MG, takes the cadence's `--quick`,
   since rule 20 gives the lane's last release no gated head).
6. **The copies**: at the approval the orchestrator copies the specification and
   this plan, as approved, into `<record>/docs/` (P1), P0's fallback should the
   untracked copies in `docs/` of `.claude/worktrees/v071` be lost.
7. **The lane's two build folders** (§1; settled 2026-11-15: the maintainer approved
   the KDE build lane's extra build folder): `kde-b`'s (tens of GB, under
   `/run/media/tibi/work/cargo-build/`), made by the steps 2–3 release and kept
   through the caret's, is named in the record, and the orchestrator deletes it with
   `kde-b` when this release, the KDE lane's last of the batch, closes at the
   maintainer's merge; `v071` and its build folder stay.
8. **Put for approval with this plan:** D0, the version number (item 1); and this
   plan's own additions and departures, item 5's list. Under a no to any of them,
   the plan is amended before P0.

   **Settled, the maintainer's decisions, not put again:** S1 to S5 as first proposed
   (rule 6 follows S1 as written, with no re-reading), S3's amendment and S6; and, on
   2026-11-15, S2's amendment, the merge (`git -c merge.conflictStyle=diff3 merge --no-ff --no-commit <prev-tag>`
   once, after the release's last unit before the merge, MG, in place of the first
   form's rebase; the rationale's S2, *Amended*, gives why: every sha stays); the
   batch's working rules, rules 1 to 26 of the batch-speed rationale's Appendix (13
   and 23 superseded), which this plan cites throughout; the three re-readings the
   rules name (rule 20's of S2's branch point, the lane predecessor's documents unit;
   rule 7's branching from `<prev-tag>` where that tag exists when P2·6 runs; rule 9's
   "green"); the KDE build lane's extra build folder (item 7); the draft pull request
   at a branch's first push (item 3); D10 (a), KDE step 1 inside v0.7.1; and the
   release order, approved with S3 (the header). The rules carry them out.

**During the work, the maintainer is told or asked:** "measure now" and its
announcement (MB, MA); the measurement's stop entries or parity entries; 6.D.4's
result in case 3 and the request question (open point 3); a stop of §7; a red CI
run; "capture now" (K6) and what the new screenshots show; a parity difference at
K6 and how it was settled (fixed and run again, or excepted; rule 19); the capture check's
result where it is undecided or a dump failed, and any capture-code fix; MG's
conflicts and a toolkit version it brought.

**After K8:** whether to merge, tag and publish, and the pull request's title and
description.

## 9. Record

Copied from `<record>/record.md`, both steps' parts at K7.

- P1: `<locks>` made, the trial's output `<…>`.
- Step 6: the worktree (reused or new) and its build folder `<cargo-build>/<hash>`;
  the maintainer's answers (§8) `<…>`; A6 `<all found | the texts not found>`, step
  3's trial `<passed | failed>`, the counts' bases `<…>`; 6.D.2's case `<1 | 2 | 3>`,
  with 6.D.4's result line `<…>`; the reviews
  and their must-fixes `<…>`; each unit's gate (`--quick` or `--commit`) and its time
  `<…>`.
- Step 7: P·7's warm-up gate `<run | none needed>`; A7 `<…>`, the hashes `<…>`;
  the reviews `<…>`; each unit's gate and its time `<…>`.
- The release: `<lane-pred-head>` `<sha>`; U7.4's gate `<…>` (no gated-head line: rule 20); MG `<date>`,
  `<prev-tag>` `<…>`, the merge commit `<sha>`, its conflicts, the hunks resolved to
  `<prev-tag>`'s side, a generated file pending, `Cargo.lock`'s resolution, and their
  reviews `<none | …>`, the union list `<…>`, the toolkit versions before and after and VF's
  outcome `<none needed | …>`, R-MG `<…>`, the warm-up gate `<…>`, the path check
  `<…>`, the survived checks and the reply's state `<…>`; X's recounts and gate
  `<…>`; V's stamped-path check `<…>`; a
  failure a `--commit` found in a commit only `--quick` proved, and the commit that
  brought it `<none | …>`; K4's failure and warnings `<…>`; K3's CI run `<id>` and
  the macOS test's two log lines `<…>`; K6's `STATUS` and the log's tail `<…>`; the
  capture check's printout and exit status `<…>`, and a capture-code fix with its
  rerun `<none | …>`.
- Durations measured `<…>`. Departures from this plan `<…>`.

## 10. Measurement record (step 6, 6.G)

Committed by `rec`, on top of U6.1's last commit, in B.10's form.

- The folders: `target/measure-6/before` (at `<sha>`), `target/measure-6/after`
  (at `<sha>`), both on `<base>` (`<sha>`: `<lane-pred-head>`, or `<prev-tag>` where the branch was made from it), before MG; MG changes
  no sha, so these stay valid; deleted after the release's first push (rule 19).
- The pre-check: before `<…>`, after `<…>`.
- Each block's time, before and after, per showcase `<…>`; the reruns `<…>`; the
  entries whose icon set fell back `<…>`.
- The comparison's printout and exit status `<…>`.

## 11. Time

Measured at the 0.7.1 release on this machine and at GitHub (the archived plan of
issue #4's KDE fixes, §8): the full `check_release.sh` about 23 minutes; the asset
script 30 to 37 minutes, of which 4 to 6 need the desktop; a CI run 21 to 27
minutes. From the specification: a measurement block is at least 9 × 6 s and at
most 9 × 46 s of sleeps and bounded waits (B.10, *The duration*). A `--commit`
run: the batch's earlier figures, 5 to 12 minutes a commit and 29 minutes cold
(§4), measured before the 16-job cap; today's is not measured until the first timed run (P0's gate, or P2·6's warm-up gate)
(§4), which the record holds. Not measured: the first build in a new worktree and
every task's duration, and `--quick`'s time (S1: the first timed into the record);
the first runs give them, and the record holds them. The longest chain is the
lane's: P2·6, P0, MB, U6.1, R6.1, MA, `rec`, U6.2, U6.3, P·7, U7.1, U7.2, U7.3,
U7.4, MG and X (when `<prev-tag>` exists; VF inside MG), V, K1 to K8 (K4 and
K5 beside K3's CI, S4); plus any review fix and the
waits for `<prev-tag>` and the desktop.
