# 0.7.N: the text caret's blink — implementation plan

- Rationale: [`todo_v0.7.1_caret-blink-rationale.md`](todo_v0.7.1_caret-blink-rationale.md)
  (decisions B1 to B6). Specification:
  [`todo_v0.7.x_caret-blink-spec.md`](todo_v0.7.x_caret-blink-spec.md)
  (Parts 0 and A to E, *Order and gates*, *Under the other B6 options*; checked,
  final).
- Written: 2026-11-15, against `v0.7.1-rc2` at `0fa28027`, the specification's
  commit; every `file:line` this plan cites is `0fa28027`'s unless it names
  another commit or version, and the plan adds no line citation of a
  changing repository file but the gate-read lines rule 25 names (S6).
  Status: **not approved**. Nothing below starts before the maintainer
  approves the rationale, the specification and this plan, and answers §7's
  items 1, 2, 5 and 8 (the specification's two open points among them;
  items 3 and 4 were settled on 2026-11-15).
- The release: a 0.7.x release of its own (B6 (a), the specification's basis),
  item 4 of *The releases after v0.7.1* (`docs/issues/0004_kde-open-items-rationale.md`,
  D10). Its number and its place in the order are the maintainer's: `0.7.N`
  below, and `<prev-tag>` is the tag of the release immediately before it in the
  maintainer's order, whichever that is (under D10's proposal with S5's pairs
  `v0.7.5`, the widget icons', which carries the KDE steps 4 and 5 through
  its own merge).
- The lane (S3 as amended by the maintainer on 2026-11-15): this release runs
  in the KDE lane, v0.7.1 → the KDE steps 2 and 3 → steps 4 and 5 → the caret
  → steps 6 and 7, in the worktree `kde-b`, which the steps 2–3 release used
  and which this release reuses once that release has its tag (rule 18). Its
  lane predecessor is the steps 4–5 release, its lane successor the steps
  6–7 release, which reads this release's gated-head line (M, rule 20). The
  second lane holds the widget icons alone; `lane2-b` is not used, and this
  release deletes no lane worktree.
- Under the choices still open (rule 18; §7 item 1): KDE step 1 ships in
  v0.7.1 (D10 (a), the maintainer's answer of 2026-11-15; D10 (b) is not
  taken). Under D10's proposed release order, `<lane-pred-head>` is the
  steps 4–5 release's gated head (0.7.4), read from that plan's record
  (`docs/todo_v0.7.2_kde-hover-outlines-and-toolbar-buttons-plan.md`, its rule 20 line),
  `<prev-tag>` is the icons' `v0.7.5`, and this release is 0.7.6. Under
  D10's other order for its items 2 and 3 (the icons before the KDE steps 2
  to 5), `<prev-tag>` is the steps 4–5 release's own tag (`v0.7.5`), the
  release number unchanged; MG then brings
  the icons and egui 0.37 through that tag, which took them in at the KDE
  releases' own merges, and where that tag exists at P2 the branch is made
  from it and there is no MG. Where the icons' follow-up W7 (D10 item
  3, "the step that serves the two pixel sets") is cut as a second-lane
  release of its own before this one, `<prev-tag>` is W7's where it comes
  immediately before, and every number from W7's on is one higher; the lane
  predecessor, its record and the worktree are unchanged, since W7 is in
  the other lane. Where W7 is cut after this one, nothing here changes.
- Where: branch `v0.7.N-rc1`, made from `<lane-pred-head>`, the gated head of
  the branch of the release before this one in its own lane (here the steps
  4–5 release's: its documents unit, the last unit before its merge, past
  its gate, with its reviews closed, rule 20), not from its tag; and, once it
  exists, merged with `<prev-tag>`, the tag of the release immediately before
  this one in the maintainer's release order, which may be the other lane's:
  `git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>`,
  once, after this release's last unit before the merge has passed its gate
  and every review is closed (S2 as amended on 2026-11-15, approved by the
  maintainer that day, §7 item 8: a merge, not a rebase, so no commit changes its
  sha; task MG). Nothing is pushed and no close-out starts before that merge.
  The two lanes share the machine's limits (§6). Decisions S1 to S6 of
  `docs/todo_v0.7.1_batch-speed-rationale.md` (approved by the maintainer on
  2026-11-15, S3's amendment and S6 included) set the gates' cadence (§2),
  the branch (S2), the lanes (S3), the close-out's order (S4, §2.1) and the
  line citations (S6: the citations this release writes name the document,
  never a line of a tracked file that changes, and no existing citation is
  re-pointed or converted). The batch's working rules are cited as
  "rule N (batch-speed rationale, Appendix)", the numbered rules 1 to 26 of
  that document's *Appendix — the batch's working rules*, authoritative where
  this plan and they differ; below, "rule N" is short for that. The rules,
  and S2's amendment to a merge, were approved by the maintainer on
  2026-11-15 (§7 item 8). This plan
  merges, tags and publishes nothing.
- Placeholders: `0.7.N`, `<prev-tag>` (above, which MG merges),
  `<lane-pred-head>` (the sha on the line "the lane's gated head for the next
  release: …" of the steps 4–5 plan's running record,
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.x_kde-open-items-steps4-5-plan/record.md`:
  its documents unit, the last unit before its merge, past its gate, with its
  reviews closed, rules 18 and 20; the branch is made from it),
  `<merge>` (MG's merge commit), `<core worktree>` (the core worktree,
  `.claude/worktrees/kde-b` under the proposal, §1), `<date>`, `<sha …>`, `<work>` and
  `<locks>` (§5), and every count, time, line and result read at a run. Each is
  filled by the task that reads it, from its run, and written into the running
  record (§9); none is
  estimated.

This plan says who does what, in which order, and how each task is known to be
done. What each task changes is the specification's; the plan repeats none of
it.

## 1. Shape of the work

The units 0 and 2 to 6 are the specification's (*Order and gates*; there is
no unit 1, S6), in its order; unit 6 runs as several tasks (§2). Two lanes in
two worktrees of the same repository, so that no agent's half-done files reach
another's tests or commits. The lanes meet by a merge the orchestrator makes
(`git merge --no-ff`, so every commit keeps its hash), while no agent works in
the worktree merged into. Worktree paths are relative to the main checkout,
`/home/tibi/Rust/native-theme`; a new one is made with `git worktree add` from
the commit named, and every `git worktree add` and `remove` is given absolute
paths (rule 24), so that it does not depend on the directory it runs in (an
agent's own isolation worktree starts from `main`, the wrong base).

| Lane | Worktree and branch | Tasks | Cargo |
|---|---|---|---|
| core | `<core worktree>`: `.claude/worktrees/kde-b`, the KDE lane's second worktree, which the steps 2–3 release used, switched to `v0.7.N-rc1`, or, where it does not exist, made from `<lane-pred-head>` (*The build folder* below) | P2, P0, U2, U3, U4, U5, M, MG, VF, LC, X, V, K1 to K8 | yes: the only one of this work's worktrees that runs it |
| docs | `.claude/worktrees/kde-b-docs`, `v0.7.N-rc1-docs`, from U3's commit | U6 | none but `cargo fmt --all --check`, which builds nothing |

P1, task A and task L run outside the lanes (A reads the core branch with
`git show`, never a working tree; L reads the web and writes only into
`<work>`).

```
core    P1 P2 ─ P0 ─ U2 ─ U3 ─┬─ U4 ─ U5 ─[M]─ MG (VF inside) ─ LC ─ X ─ V ─ K1 K2 K3 ─ K5 ─ K6 ─ K7 ─ K8
        A (beside P0)         │            ↑                           K4 (beside K3's CI)
docs                          └─ U6 ───────┘
beside            L (from U3's commit, beside U4 and U5)
reviews           R3h, then R3 (beside U4)   R4w, R4 (beside U5)   R5 (before [M])
```

Why this order (each from the specification unless named):

- **A before U2.** The specification's lines are `0fa28027`'s. Of the releases
  that land before this one under D10's proposal (egui 0.37, the KDE steps 2–3,
  the KDE steps 4–5, gpui-component's own icons), v0.7.1 and the KDE steps 2
  to 5 are on the tree the work starts from (the branch starts in the KDE
  lane, S2, S3 as amended), and egui 0.37 and the icons arrive at MG. So every
  quoted text and every count's base is read again on the starting tree, and
  MG checks again what the merge brings.
- **U2, then unit 3** (*Order and gates*): Part 0 before unit 3, whose facts
  cells and preset comments cite the evidence it adds. One exception: where no
  branch holds `72fcf6a7` any more, U2 runs first (P2).
- **U4, then U5, in the core lane.** "Units 4 and 5 do not depend on each
  other", and their files are disjoint; both need unit 3 (D.3's exceptions and D.1's
  tests read its presets). Side by side they would need a second worktree with a
  build folder of its own, a whole extra build of the gpui, iced and egui
  stacks; this plan runs them one after the other, the path it takes.
- **U6 in the docs lane beside U4 and U5**: Part E's texts (E.1 to E.4) are
  fixed by the specification and the open points' answers; none needs units 4
  or 5. E.4's dated sentence needs the release's date and is V's (§2).
- **L beside**: the links of A.8 exist from U3 on; opened early, a correction
  lands before the asset run rather than in the close-out, where E.6 lists it.
  Its correction, LC, comes after M, when every copy of a citation it corrects
  is on one tree.
- **MG after the last unit before the merge** (S2, amended; rules 17 and 21):
  the merge of `<prev-tag>`, then its warm-up gate. It runs once, after U5
  and M, when every review is closed with its fixes and the tag exists; VF
  (the version forms after the merge, rule 25) runs inside it, before its
  warm-up gate. LC, X, V and the close-out wait for it, so that X reads the
  merged tree and
  the close-out's compatibility and semver steps compare with the released
  predecessor. No commit changes its sha, so the reviews' ranges, the record's
  shas and the docs branch's merge commit all stand. Where the tag already
  exists at P2, the branch is made from the tag (rule 7), MG and X fall
  (rule 17), and task A reads that tree.
- **X after LC**: on the merged tree it recounts every stated count in every
  file either side of MG changed (rule 17: the union), outside the pinned
  documents, and runs the five
  documentation checkers.
- **V before K1**: the provenance stamp hashes `Cargo.lock` (E.6).
- **K3 onward only after every review is closed** and its fixes committed: a
  run or a measurement of code that a review may still change is not made.
- **Beside the second lane** (S3 as amended): the order in which releases
  are cut stays the maintainer's. Files the icons' release and this one both
  edit (`docs/evidence/README.md`, where both append a section after
  `## breeze-offscreen/`, so the merge conflicts there for certain;
  `connectors/native-theme-gpui/src/lib.rs`, the icons' D.10;
  `connectors/native-theme-gpui/README.md`, the icons' F.1, in a different
  section; and the release documents `CHANGELOG.md`, `ROADMAP.md` and `docs/todo.md`) meet at MG's merge, by quoted text,
  as every edit here is placed; a conflict that holds a citation is
  resolved by both sides' texts like any other text (S6, rule 21 (1)).

**The build folder.** Cargo keeps one per worktree path. Under the batch's
worktree rule (S2, S3 as amended) the KDE lane alternates between
`.claude/worktrees/v071`, v0.7.1's, and `.claude/worktrees/kde-b`: steps 2
and 3 in `kde-b`, steps 4 and 5 in `v071`, the caret in `kde-b`, steps 6 and
7 in `v071`; the widget icons, alone in the second lane, use
`.claude/worktrees/lane2-a`, and `lane2-b` is not used. Each worktree keeps
its own build folder, which its lane's later releases reuse warm; deleting
the lane's extra worktrees and their build folders is the lane's last
release's task, not this one's. So the caret takes `kde-b`, warm from the
steps 2–3 release, and its units can run while the steps 4–5 release's
close-out holds `v071`. It is reused only once the release that last used it
has its tag, checked after `git fetch origin tag <that tag>`, `git rev-parse
-q --verify <that tag>` (rule 18); until then the release waits. P2 makes
the branch there,
`git -C /home/tibi/Rust/native-theme/.claude/worktrees/kde-b switch -c v0.7.N-rc1 <lane-pred-head>`
(`<prev-tag>` in its place on the tag path, after the tag check), where
`git status --porcelain --untracked-files=no` prints nothing in it and every
untracked file is a planning document under `docs/`; `git switch` refuses
where an untracked planning document is a file that tree holds: the two are
compared with `cmp` (against `git show <lane-pred-head>:<path>`, or
`git show <prev-tag>:<path>` on the tag path), the untracked copy removed
where equal, a stop otherwise (§6). Where `kde-b` does not exist (the steps
2–3 release did not make it), P2 makes it,
`git -C /home/tibi/Rust/native-theme worktree add -b v0.7.N-rc1 /home/tibi/Rust/native-theme/.claude/worktrees/kde-b <lane-pred-head>`
(the same substitution on the tag path), which builds the workspace once
from nothing (tens of GB, one long first gate). P2 says whether it switched
or made `kde-b`, and reads its build folder's path from `cargo metadata
--format-version 1 --no-deps` (`build_directory`) into the running record.
When this release closes, `kde-b` and its build folder stay. The docs
worktree is made by the orchestrator after U3's commit, from that commit
(`git -C /home/tibi/Rust/native-theme worktree add -b v0.7.N-rc1-docs /home/tibi/Rust/native-theme/.claude/worktrees/kde-b-docs <U3's commit>`),
and removed at M
(`git -C /home/tibi/Rust/native-theme worktree remove /home/tibi/Rust/native-theme/.claude/worktrees/kde-b-docs`);
it runs no cargo but `cargo fmt --all --check` and has no build folder.

## 2. Tasks

A task that changes files ends in one commit on its lane's branch, made only
when its done-criterion holds, with no attribution line. Every agent stages its
own paths by name; `git commit -a` and `git add -A` are not used. The subject
follows the repository's form (`feat(core): …`, `docs(todo): …`) and the body
says what changed and why, with the outputs the done-criterion names.

**Who runs what.** An agent runs its task's targeted commands (the named
tests, `cargo fmt --all --check`, the checkers, a `-p` clippy or build), each
expected to finish well within ten minutes, in the foreground; it stages its
change by name and reports. It does not run `--quick`, `--commit`, the full
check or an asset run.

**The gate and its cadence** (the batch's S1,
`docs/todo_v0.7.1_batch-speed-rationale.md`, approved 2026-11-15). "The gate
passes" means: the orchestrator runs the task's gate on the staged change in
`<core worktree>` as
`flock <locks>/gate.lock <work>/tools/slot.sh <work>/tools/lowprio.sh scripts/check_release.sh --quick`
(or `--commit`; rule 2: the gate lock keeps two gates from running at once), as a long run (§2.1); it exits 0, its
summary names no failure and no skipped cross-target check
(`target not installed`, `x86_64-w64-mingw32-gcc not found`), and every
warning the summary repeats is named in the running record. The cadence is the
batch's (S1): during a unit, the agent runs only the touched tests (its targeted
commands); at a unit's end, `--quick`, by the orchestrator, as a long run until
its time is measured (the first timed into the record); `--commit` every third
code unit counted from the last `--commit`, on the release's last code unit
before its documents and version units (U5), after a lock or toolchain change
(the warm-up gates, MG's included, and V, whose `Cargo.lock` lists the
workspace crates' new versions) and at the units this plan names structural;
a documents unit between that last code unit and V takes `--quick`, rule
20's gated head (M) included; a documents-only unit runs the five documentation checkers and
`--quick`; the full run in the close-out, unchanged (rule 6).
"Structural" means a unit whose change can reach a crate `--quick` would not
run: one that changes a manifest, `Cargo.lock`, a build script, the toolchain
or the gate scripts (rule 24); new model fields, a new public API or the
derive's new arm are no reason on their own, so of units 2 to 5 only U4 is
structural, and only with C.3, which edits `native-theme/Cargo.toml`'s
comment on the Windows reader (a manifest change); U4 takes `--commit` as
the third code unit in any case. So the task's gate is `scripts/check_release.sh --quick`
(what it runs on 0.7.x is defined by `docs/todo_v0.7.1_gate-speed-spec.md`,
S1; v0.7.1 ports it, before this release), but
`scripts/check_release.sh --commit` at:

- the warm-up gates (§2.1): P0's or P2's, and MG's;
- U4, the third code unit counted from the warm-up (U2, U3, U4);
- U5, the release's last code unit before its documents and version units
  (rule 6);
- V, which changes the manifests and `Cargo.lock`, a lock change (rule 6);
- and any task the count reaches. Any `--commit` (a warm-up gate, MG's
  warm-up, a `--commit`-gated fix) restarts the count, and a `--quick`-gated
  fix does not count toward it (rule 11); the table below shows the marks.

M, the documents unit and the last unit before MG, is the gated head (rule
20): it takes its cadence gate, `--quick`, with no extra `--commit`, since
U5's `--commit` covers the code beneath it, and the running record names its
commit "the lane's gated head for the next release: `<sha>` (M; --quick run
`<folder>`; reviews closed)", which the steps 6–7 release, the next in this
lane, reads as its `<lane-pred-head>`.

| Task | Unit kind | Gate | Count after it |
|---|---|---|---|
| P0 or P2 | warm-up | `--commit` | 0 |
| U2 | code (rule 11: it adds a `.py`, two `.toml` and two `.png` under `docs/evidence/`, none of them comments, rustdoc prose, READMEs or Markdown) | the five checkers and `--quick` | 1 |
| U3 | code (`docs/platform-facts.md`, which tests parse, the fields, the presets and egui) | `--quick` | 2 |
| U4 | code, the third (and structural with C.3, whose `native-theme/Cargo.toml` comment edit is a manifest change, rule 24) | `--commit` | 0 |
| U5 | code, the last before the documents and version units (rule 6) | `--commit` | 0 |
| M | documents (U6's texts), the last unit before MG, the gated head (rule 20) | the five checkers and `--quick` | 0 |
| VF, where a version changed (rule 25; inside MG, before its warm-up) | code and documents | none of its own: covered by MG's warm-up gate (rule 21) | — |
| MG's path commit (rule 22 (b); MG's step (6)), where one is needed | documents | none of its own: covered by MG's warm-up gate (rule 21 (5)) | — |
| MG | warm-up, last, covering every commit of its steps (1), (3), (6), (7) and (4) | `--commit` | 0 |
| LC | code where it corrects a facts cell (`docs/platform-facts.md`), else documents | the five checkers and `--quick` (the count: at most 1 after MG's warm-up) | 0, or 1 |
| X | code where its recount corrects a count in code or a string literal, else documents | the five checkers and `--quick` (the count: at most 2 after MG's warm-up) | 0, 1 or 2 |
| V | manifests and lock (a lock change, rule 6) | `--commit` | 0 |

The marks assume no other `--commit` between the warm-up and U4. Where a
later warm-up gate runs (a lock or toolchain change), the count decides, not
the table. Where P2 moves U2 first (P2's row: no branch holds `72fcf6a7` any
more) and a unit 0 exists, U2 comes before P0's warm-up, U3 and U4 are the
first and second code units after it, and U4 keeps its `--commit`: it then falls one code unit earlier than the count
requires, which rule 6 allows (its count says where a `--commit` must run,
not where one may not), and it restarts the count as the table shows.

What `--quick` re-runs is the gate-speed specification's
(`docs/todo_v0.7.1_gate-speed-spec.md`, A.2 and A.3): a change to a package's
files re-runs the steps of that package and of the packages A.3's table lists
for it; `docs/platform-facts.md` re-runs the steps reaching `native-theme`
(C.4); "any other file outside the packages and not in `QUICK_UNREAD`
(`docs/*.toml`, `scripts/`, the manifests, the lock), a tool, a variable
outside the deny-list, the gate's scripts" re-runs all 29 (and both `+1.88.0` checks, 31 in all, A.2); "a file matching
`QUICK_UNREAD`" re-runs none, `QUICK_UNREAD` being "`*.md` (any depth),
`docs/archive/*`, `docs/issues/*`, `docs/assets/*`, `docs/evidence/*`,
`.github/*`, `LICENSE-*`, `.gitignore`, `.gitattributes`" (A.2). Whether a
unit is a code unit is rule 11's: U3 changes `docs/platform-facts.md` (code,
since tests parse it) with the fields, the presets and egui, and so counts.
A documents unit stays one only when its change is only comments, rustdoc
prose, READMEs and Markdown documents: it runs `--quick`, whose scope then
reaches the crates those files belong to (every quick-keyed step where it
edits a `docs/*.toml` file), and it is not counted toward "every third" or
"the release's last code unit". U2 is not one under rule 11's text, since it
adds a script, two TOML files and two images, so it counts as a code unit;
nothing reads `docs/evidence/` (it is in `QUICK_UNREAD`), so its `--quick`
covers it, re-running none of the 31 quick-keyed steps (the 29 and the
two `+1.88.0` MSRV checks, keyed by O3's option the maintainer approved on
2026-11-15; the gate-speed specification, A.2) while the other `--commit` steps still run. LC is a code unit
where it corrects a facts cell; X is one where its recount corrects a count
in code or a string literal. Each reports which it is, from its diff, so that
the count is right; between MG's warm-up and V both take the five checkers
and `--quick` by the count, MG's warm-up having restarted it (at most 2 by
X). U2, a code unit, is gated by the
five checkers and `--quick`. The documents-only tasks (P0 where it is no
warm-up) run the five checkers and `--quick`; M, which merges U6's texts,
runs them and `--quick` too (rule 20). The first
`--quick` run is timed into the running record. A commit between two
`--commit` runs is proven by `--quick` only (the cost S1 accepts); a failure the
next `--commit` finds is fixed by a new commit on the branch, never by
rewriting one, and the running record names the commit that brought it. The
full run is K4's and K8's, unchanged. The gate runs on the working tree but the commit
takes the index, and `--untracked-files=no` hides a new file an agent forgot
to stage (cargo compiles any `mod` file on disk), so before each gate and
again before the commit the orchestrator requires the staged tree to be the
whole tree: `git diff --quiet` exits 0;
`git status --porcelain --untracked-files=all` shows no `??` line but the
untracked planning documents P2 recorded, named one by one; and `git diff --cached --name-status` matches
the task's list of new and changed files, which its report gives. Only P0 (its
documents), U2 (its ten evidence copies), MG (its merge commit adds every file
`<prev-tag>` brings that `<merge>^1` lacks; its list, before the merge
commit, when `<merge>` does not exist yet, is
`git diff --cached --name-status --no-renames`, checked by MG's staged-path
check, as the MG row has it) and K7 (which also changes the tracked files §2.2's path search finds; the asset files K6's run
wrote, listed from its log, and the archived documents' new paths) add files;
a new file a task's parts do not name is reported, not committed. A red gate goes back to the task's author
(continued) with the log's failing part; the author fixes and stages again,
and the orchestrator runs the gate again. When it passes, the orchestrator
commits the gated tree with the author's message (the row's "Commit:"), so
that the commit is exactly what was gated. Any other command not expected to
finish well within ten minutes (those named here: U4's CI legs, U5's GPUI Fast
legs, K5's release builds) is run by the orchestrator the same way, and a
targeted command the Bash tool stops at ten minutes is reported, and the
orchestrator runs it as a long run. In the docs
lane, which has no gate of its own, U6's worker commits itself after its
checks, and M's gate covers that commit. "Formatted and clean" means §4's rule:
`cargo fmt --all --check` passes before the commit, the gate runs on the staged
change, `git status --porcelain --untracked-files=no` prints the same before
and after the gate, and nothing after the commit. "The header" is the
specification from its top to the end of *Corrections to the rationale*, the
decisions included. "The five checkers" are §4's documentation checkers.

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| P1 | **The tools.** Copy `slot.sh` and `lowprio.sh` from `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/` into `<work>/tools/` and read both; create `<locks>` and every subfolder of `<work>` §5 lists (`mkdir -p`). In the copies: `slot.sh`'s lock folder (`D`, a scratch folder) becomes `<locks>` (§5); `lowprio.sh` keeps its absolute `renice -n 19 -p $$` (a relative `nice -n 19` gives 15 under Konsole's −4) and its `ionice -c 3`, and exports `CARGO_BUILD_JOBS=16` and `RUST_TEST_THREADS=16`, lifted only on the maintainer's word that they are not using the machine. Also the three approved documents (rule 26): P1 runs before the branch exists, so for each it reads the commit the branch will be made from, not a checkout: `git -C /home/tibi/Rust/native-theme cat-file -e <lane-pred-head>:docs/<name>` (`<prev-tag>` in its place where the branch is made from the tag) failing means untracked; succeeding means an earlier commit added it (normally v0.7.1's commit (0)). One still untracked is compared (`cmp`) in `.claude/worktrees/v071/docs/` with its approval copy in `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/caret-blink-docs/` (§7 item 7) and copied into `<work>/docs/`: equal, the v071 copy; gone from v071, the approval copy; different, a stop (§6), the running record naming what changed. One an earlier commit added is not compared: later close-outs rewrite its path mentions (rule 22 (a)) and set its status line; P1 records the commit that added it and copies nothing. P0 takes the copies from there, and an agent that needs the specification before P0 commits it reads that copy. And create the running record, `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.x_caret-blink-plan/record.md` (§9), and read into it, before the documents' check, which needs the base: `<lane-pred-head>`, the sha on the steps 4–5 record's rule-20 line "the lane's gated head for the next release: …" (`~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.x_kde-open-items-steps4-5-plan/record.md`), quoted, and whether `<prev-tag>` exists (`git -C /home/tibi/Rust/native-theme rev-parse -q --verify <prev-tag>`, after `git -C /home/tibi/Rust/native-theme fetch origin tag <prev-tag>` where it is not local); no rule-20 line there is a stop (§6), and the work waits. No repository change. | orchestrator | the maintainer's resource rules | `<work>/tools/slot.sh <work>/tools/lowprio.sh true` exits 0; the rule-20 line, quoted, and whether `<prev-tag>` exists are in the running record; for each of the three documents the running record names either the commit that added it, or that it is untracked, with `cmp` of its copy in `<work>/docs/` against its approval copy in `scratch-keep/caret-blink-docs/` printing nothing and its source named; `<work>/tools/slot.sh <work>/tools/lowprio.sh sh -c 'echo "$CARGO_BUILD_JOBS $RUST_TEST_THREADS"; ps -o ni= -p $$'` prints `16 16` and `19`; `<locks>/slot1.lock` exists (`slot2.lock` appears whenever a command starts while slot 1 is held, `slot.sh`'s loop) |
| P2 | **The branch and the worktree** (*The build folder*, §1), once `<lane-pred-head>` exists: the lane predecessor's (the steps 4–5 release's) gated head, its documents unit, has passed its gate and its reviews are closed (S2; rule 20; rule 18: this release reads those texts), each read from the predecessor's commit messages or record, not from a status line its close-out writes; the branch is made from `<lane-pred-head>`, or from the tag `<prev-tag>` where it exists already (then MG falls). P2's two readings (the gated-head line, whose absence is a stop, §6, and whether `<prev-tag>` exists) are compared with P1's, and where either changed (the tag appearing between P1 and P2, say), P1's rule-26 check of the three documents is run again on the new base before P2 goes on (rule 26). Also, read: the versions `Cargo.lock` holds of `egui`, `iced_widget`, `gpui-component`, `gpui-base` and `windows` (of `windows`, the version `native-theme/Cargo.toml:139` requires; the lock holds several), the toolkits whose versions the specification's texts name (`EG` egui 0.36.2, `IW` iced_widget 0.14.2, `GC` and `GB` 0.9.1, windows 0.62.2, all as the lock holds them at `0fa28027`), into the running record for MG's comparison (rule 19); on both paths (the branch made from `<lane-pred-head>` or from `<prev-tag>`), the tree's `Cargo.lock` is compared with `git show 0fa28027:Cargo.lock` for the same crates (rule 19); a version that differs at P2 from `0fa28027`'s (one v0.7.1 or the KDE steps 2 to 5 brought, or on that path any release in the tag) means that its readings, as VF's part (a) lists them, are made again now, before the units, and task A works from them; P2's accepted set (rule 19) is egui 0.36.2 or 0.37.x and patch releases of the other four's versions above; any other version is a stop (§6); `git cat-file -e` finds `72fcf6a7`, `efbffffe` and the ten blobs of Part 0.1, and `git for-each-ref --contains 72fcf6a7` names a branch (Part 0: the iOS commit is on one local branch only; no remote holds its blobs `3d1d5b2f` and `ed5470ef`, `git rev-list --objects --remotes` read 2026-11-15). Where the objects exist but no branch holds `72fcf6a7` any more, U2 runs at once, before P0, so that the objects are in a commit before anything can prune them; the running record notes the departure from the specification's order. The worktree is `kde-b` (§1), reused only once the release that last used it has its tag (rule 18, checked after `git fetch origin tag <that tag>`), else the release waits; P2 switches it to the new branch, or makes it where it does not exist, as §1 says. Before any code agent is dispatched, the project's hooks are seen to fire in the core worktree (they are configured with absolute paths in the main checkout's `.claude/settings.json`): a Write of a new, untracked file `<core worktree>/native-theme/src/hook_trial.rs` containing `fn f() { x.unwrap() }` (the hook reads a Write's `tool_input.content`) is refused by `no-runtime-panics.sh`, and nothing is written. If the Write applies, the file is removed with `rm` and the plan stops, no code agent dispatched; `rustup target list --installed` lists `x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu` and `x86_64-apple-darwin`, and `x86_64-w64-mingw32-gcc` is on the path (unit 4's Windows half is checked here, not left to CI: *Order and gates*); the installed Pillow's version (Part 0's check was made with 13.0.0). On the tag path, MG's tag check (rule 21 (1)) runs first, before the switch to (or `git worktree add` from) `<prev-tag>`, so that a stop leaves no branch made from a moved tag: the first field of `git ls-remote origin 'refs/tags/<prev-tag>'` equals `git rev-parse <prev-tag>`, else a stop. No commit, except a path re-point before its warm-up (rule 22 (b)). | orchestrator | §1 | in the core worktree, `git rev-parse --abbrev-ref HEAD` prints `v0.7.N-rc1` and `git rev-parse HEAD` equals `<lane-pred-head>`, the sha on the steps 4–5 record's line "the lane's gated head for the next release: …" (rule 20; or `git rev-parse <prev-tag>^{commit}` where the branch was made from the tag, fetched first with `git fetch origin tag <prev-tag>` only where `git rev-parse -q --verify <prev-tag>` fails; on that path `git rev-parse HEAD^` equals it instead where P2 committed a path re-point), and the running record names which (the gated-head line's sha, the predecessor's documents unit's commit, is HEAD, so that unit needs no check of its own); where the branch was made from `<prev-tag>`, `git merge-base --is-ancestor <lane-pred-head> HEAD` exits 0 (the tag contains the lane predecessor's release; on the other path MG checks it before its merge, rule 18); `git status --porcelain --untracked-files=no` prints nothing, and every untracked file is a planning document under `docs/` (§1), each named in the running record (the list §2's staged-tree check reads); whether P2 switched or made `kde-b`, the tag of the release that last used it, the build folder's path (`cargo metadata --format-version 1 --no-deps`, `build_directory`), the targets and Pillow's version are in the running record; the hook's refusal is quoted there, and `<core worktree>/native-theme/src/hook_trial.rs` does not exist. An object `git cat-file -e` cannot find, a target missing, the MinGW linker missing, or the trial Write applied: stop (§6). Where there is no unit 0, P2 ends with the warm-up gate (§2.1), which passes on the clean tree, timed into the running record as the first `--commit`; where there is neither a unit 0 nor an MG (the branch made from the tag), P2 also runs P0's path check (rule 22 (b)) |
| P0 | **Unit 0: the documents** (decision 17; rule 26): those of this work no earlier commit added (the rationale, the specification and the plan too, each where v0.7.1's commit (0) did not add it), from `<work>/docs/` (P1's copy), their status lines set to approved with the date. For a document an earlier commit added (P1 names it), P0 only sets a status line that commit left unset; where nothing is left to add or set, there is no unit 0. **The path check** (rule 22 (b)) runs right after MG; where the branch was made from `<prev-tag>` (no MG) it is P0's, or P2's where there is no unit 0 either. Every token of this specification and this plan that starts with `docs/` (its `:N` suffix and `#…` stripped) is checked with `cd <core worktree> && git ls-files --error-unmatch <path>`, and every relative `](…)` link of the two documents from `docs/` (`cd <core worktree>/docs && git ls-files --error-unmatch <link>`), brace sets expanded; exempt are paths cited "on <branch>" or "at <sha>", the paths a unit of this release creates, placeholders, glob or regex patterns, URLs, `crate::` links and quoted texts of other files. One that moved to `docs/archive/` is re-pointed by a `worker` in a documents-only commit (here P0's own; where P2 runs the check, before its warm-up gate, which covers the re-point); one gone otherwise is a stop (§6). No handoff file is read: an earlier close-out rewrote every path mention of its moved documents in every tracked file (rule 22 (a)). | orchestrator | *Order and gates*, unit 0 | `git show --stat HEAD` lists the documents and nothing else; the five checkers pass; the gate passes. Commit: `docs: the caret blink's rationale, specification and plan, approved <date>` |
| A | **The anchors**, beside P0. At P2's commit (P0 adds only this work's documents, which A does not read), with `git show <sha>:<path>` and the specification's own search methods, scripts kept in `<work>/anchors/`: (a) every present text the specification quotes to replace or to place an edit by, and every test it changes by name, found once in the file it names, with its line, or "not found"; (b) the bases of the specification's counts: the egui leaf count and its message, the inputs' verdict totals and `egui-limited` (D.2), `mapping.md`'s totals, `one_gsettings_read_per_key`'s counts (C.2 test 1), and the struct literals of `InputTheme` (`git grep -n 'InputTheme {' -- '*.rs'` without the struct's definition, A.5's count). No commit, no cargo. | `worker` | the whole specification (parts (a) and (b) need every part, A.5 included) | the two outputs are in `<work>/anchors/`, each table complete; nothing is "not found", or §6's stop for a changed text holds; the orchestrator copies the summary into the running record |
| U2 | **Unit 2: Part 0.** The ten files of 0.1, brought in unchanged with `git restore --source=<commit> --staged --worktree -- <paths>` (`efbffffe`'s three, then `72fcf6a7`'s seven), in place of 0.1's `git show … >` form: the same bytes at the same paths, staged, with no shell write; and 0.2's two README sections. Where P2 moved U2 ahead of P0, 0.2's two links name the specification P0 adds next (no gate reads Markdown links). | `worker` | the header; Part 0 | `git hash-object` of each file equals its blob in 0.1; then 0.1's `python3 -I …/behavior_values.py …` exits 0 and `git diff --exit-code` on `behavior_values.toml` prints nothing (the Pillow version and any warning in the body; the script failing, as it would under a Pillow that has removed `Image.getdata`, is a stop, §6); 0.2's block, extracted from the specification by the worker's own script in `<work>/u2/`, equals the README's new lines; the privacy grep (*U2, the privacy grep*, the block after this table) finds nothing; `git show --stat HEAD` lists the eleven paths; the five checkers pass; the gate passes. Commit: `docs(evidence): the WinUI and iOS host runs that the caret blink cites` |
| U3 | **Unit 3:** A.1 to A.8, A.10, B.1 to B.3 and D.2; A.8's three rows by its placement rule, right after `caret_color`'s row, found by its name, made with the Edit tool. | `implement` | the header; Part A; Part B; D.2; A's outputs (a), (b); §5's *Archived paths* | (0) the three rows equal A.8's blocks (extracted and compared by the agent's script in `<work>/u3/`), and `git diff HEAD -- docs/platform-facts.md` adds exactly those three lines and removes none; (1) the tests of A.1, A.5, B.2 and D.2 are written first and seen to fail (a test that cannot compile before its field exists counts; the report quotes it); (2) they pass: `cargo test -p native-theme-derive --lib min_u32`, `cargo test -p native-theme --lib -- a_blink_stated_whole_or_not_at_all_validates half_a_blink_names_the_key_to_add a_zero_shown_duration_or_time_out_is_out_of_range the_largest_time_out_validates an_overlay_merges_a_blink_field_by_field native_themes_state_the_caret_blink property_registry_keys_match_derive_inventory no_inheritance_fields_are_not_inherited`, `cargo test -p native-theme --test proptest_roundtrip`, `cargo test -p native-theme --test platform_facts_xref`, `cargo test -p native-theme-egui --all-features --lib` (D.2's two `base.rs` tests and the mapping tests among them); (3) `mapping.md` written by D.2's command only, and `mapping_md_is_current` passes; (4) each count equals A's base (its output (b)) plus D.2's delta, in the report (C.2's delta is U4's); (5) `git grep -n 'InputTheme {' -- '*.rs'`, leaving out the struct's definition (`pub struct InputTheme {`, `model/widgets/mod.rs:114` at `0fa28027`), shows as many literals as A's output (b) recorded, and each but `proptest_roundtrip.rs`'s, read at its hit, ends with a `..` base expression (`..Default::default()`), so that `proptest_roundtrip.rs`'s is the one full literal (A.5); (6) the gate passes, formatted and clean. Commit: `feat(core): the caret's blink in the model, the facts, the presets and the egui connector` |
| U4 | **Unit 4: Part C.** C.1 and C.2; C.3, with its edit of `native-theme/Cargo.toml`'s comment on the Windows reader, only with the maintainer's approval of the `unsafe` call, C.5 otherwise (`windows.rs` and the manifest untouched). | `implement` | the header; Part C (C.3 or C.5 by §7 item 2); with C.3, *Open points* item 2 (Microsoft's two pages, their URLs and quotes, which C.3's `SAFETY` comment and constants link); A's outputs (a), (b); §5's *Archived paths* | (1) C.1's two, C.2's five new tests, and with C.3 its four, written first and seen to fail; C.2 test 1's counts are A's base plus 3; (2) `cargo test -p native-theme --features kde,portal,system-icons,material-icons,lucide-icons,watch,svg-rasterize,system-fonts --lib -- kde:: gnome:: watch::` (the list `scripts/check_release.sh:420` sets) and `cargo test -p native-theme --lib windows::` pass (C.3's tests run on every platform), each with a count of tests run above zero, quoted in the report; (3) CI's Linux legs of the edited modules, run on this machine by the orchestrator as long runs beside the gate (§2), each part of that gate and so started under its lock (rule 2): `flock <locks>/gate.lock <work>/tools/slot.sh <work>/tools/lowprio.sh cargo test -p native-theme` with `--no-default-features`, `--features kde`, `--features portal` and `--features linux,watch` (`.github/workflows/ci.yml:211-217`); four legs cover that range's seven, since `linux-kde` and `linux-portal` are aliases of `kde` and `portal` and `linux` is the two of them, within `linux,watch` (`native-theme/Cargo.toml:35-38`); (4) with C.3: C.3's Windows-only test cannot run here and cannot be seen to fail; the gate's cross-target clippy `--all-targets --all-features` for `x86_64-pc-windows-msvc` (`scripts/check_release.sh:488-497`, a step `--quick` keeps too, the gate-speed specification's decision 1 (c)) type-checks it, and CI runs it first (K3); (5) the gate §2's count gives (`--commit` as the table assumes, U4 being the third code unit, and structural with C.3's manifest edit; `--quick` would keep the cross-target steps too, decision 1 (c)) passes, formatted and clean, its cross-target and Windows strict-panic legs run and none skipped. Commit: `feat(core): the KDE, GNOME and Windows readers read the caret's blink` (without the approval: `… the KDE and GNOME readers …`) |
| U5 | **Unit 5: D.1, D.3, D.4**, in the core lane after U4's commit. | `implement` | the header; D.1, D.3, D.4; A's output (a); §5's *Archived paths* | (1) the new tests of D.1 (items 3 to 5), D.3's exceptions, D.4's `a_u32_ms_leaf_prints_whole`, written first and seen to fail; D.1's items 1 and 2 changed with their tests; (2) the two GPUI Fast legs `--commit` skips (`scripts/check_release.sh:710-716`, `:849-855`), run by the orchestrator as long runs beside the gate (§2), each part of that gate and so started under its lock (the batch glossary's GPUI Fast legs, rule 2): `flock <locks>/gate.lock <work>/tools/slot.sh <work>/tools/lowprio.sh cargo test -p native-theme-gpui --features gpui-fast,native,watch --all-targets` and `flock <locks>/gate.lock <work>/tools/slot.sh <work>/tools/lowprio.sh scripts/run_in_runner_env.sh test -p native-theme-gpui --features gpui-fast,native,watch --example showcase-gpui` (the runner environment prepared first with `--prepare`; an exit 75 means U5 is not done: it is reported, not worked around, and the plan stops for the maintainer, §6); (3) `python3 scripts/check_connector_parity.py` passes (decision 9: no new public item); (4) the tests D.1, D.3 and D.4 name pass, each named `ok` in the report: D.1's (`every_theme_field_is_named`, `apply_keeps_the_applications_own_settings`, `caret_blink_maps_each_case`, `to_theme_sets_a_stated_blink_and_leaves_upstreams` and item 5's `#[gpui::test]`s) in the output of item (2)'s `cargo test -p native-theme-gpui --features gpui-fast,native,watch --all-targets`, and the gpui showcase's `every_listed_leaf_has_a_route` in its second command; `cargo test -p native-theme-iced --features iced_aw,watch,native --lib -- every_unreachable_value_is_one_a_preset_loses every_colour_leaf_reaches_a_row_or_is_named` (D.3); `cargo test -p native-theme-showcase-support -- a_u32_ms_leaf_prints_whole a_ms_leaf_reads_its_value` (D.4); and, in the runner environment item (2) prepared, `scripts/run_in_runner_env.sh test -p native-theme-iced --features iced_aw,watch,native --example showcase-iced -- the_element_list_parses_and_every_leaf_has_a_route` and `scripts/run_in_runner_env.sh test -p native-theme-egui --all-features --example showcase-egui -- the_info_is_the_manifest` (D.4); (5) every edit of `connectors/native-theme-gpui/src/` or its examples (D.1's `lib.rs`, D.4's `info/leaves.rs`) keeps the line count above each line a colour claim cites, or re-points those claims in the same commit, so that `connectors/native-theme-gpui/src/showcase.rs`'s `every_colour_claim_is_read_at_the_line_it_cites` (S6's fourth gate) passes, the report naming which; (6) the gate passes, formatted and clean. Commit: `feat(connectors): gpui installs the theme's caret blink; iced and the showcases state it` |
| U6 | **Unit 6's texts:** E.1 (its `### Breaking Changes` entry kept under B6 (a) and (b), dropped under (c); its reader entry by §7 item 2 and C.5), E.2 (by C.5 where the call is not approved), E.3 (the heading with the version B6 sets; its readers' sentence in C.5's wording where the call is not approved), E.4 (its four entries always, as written: they cite no line of a tracked file that changes, naming such a file and quoting its text, and keep the lines of pinned sources, upstream crates at a version, the parked facts at `78ee0efd` and a document at `0fa28027` (S6); without the approval also C.5's entry *Windows reader: the caret's blink time*, after the windows-11 entry, and no dated sentence; with it, the dated sentence is V's), in the docs lane. | `worker` | the header; E.1 to E.4; C.5; *Under the other B6 options*; §7's answers; A's output (a) | each text equals the specification's block for the answers given (extracted and compared by the worker's script in `<work>/u6/`), placed by its anchor; each changelog entry one physical line, under a new heading `## [0.7.N] - <date>` with the proposed number (the date filled at V), never `## [Unreleased]`, its section above every other `## [` heading of the file (rule 14; E.1); the five checkers pass; `cargo fmt --all --check` passes before the commit (E.2 edits `native-theme/src/lib.rs`; it builds nothing), and `git status --porcelain --untracked-files=no` prints nothing after it; the lane's `cargo doc` and doctests are M's gate. Commit: `docs: the caret blink in the changelog, the core README, the roadmap and the backlog` |
| L | **The facts cells' links** (decision 4, E.6's first item), from the time U3's rows are written (U3's commit): each URL of A.8's three rows opened, the lines it shows read against the text the cell quotes or names. For the five LGPL files (`khintssettings.cpp`, `qwidgetlinecontrol.cpp`, `qquicktextinput.cpp`, `gtksettings.c`, `gtktext.c`) L records only line ranges and verdicts, no text of them. | `worker` | A.8; `F/sources.toml`'s pins as A.8 lists them | a table in `<work>/links/`: each URL, the lines shown (for the LGPL files, the range only), "holds" or what differs; for a range off with the text found elsewhere in the same file, the right range. L commits nothing. A text not found, or one that says something else: a stop (§6) |
| M | **The merge**, after U4, U5 and U6 have committed and R3h, R3, R4, R5 and, where C.3 applies, R4w are closed with their fixes: `v0.7.N-rc1-docs` into `v0.7.N-rc1`, `cd <core worktree> && git merge --no-ff -m "Merge branch 'v0.7.N-rc1-docs' into v0.7.N-rc1" v0.7.N-rc1-docs` (the form of the repository's earlier docs merges, `eb31d3a2`). A conflict (none is expected: U4 and U5 on one side and U6 on the other edit different files) is resolved by the quoted texts and reported. A gate that fails only on the merged tree is fixed by the author of the change it points at, on `v0.7.N-rc1`, and reviewed again (§3). Then the docs worktree is removed (§1) and its merged branch deleted after it, `git -C <core worktree> branch -d v0.7.N-rc1-docs` (in the core worktree, whose HEAD `v0.7.N-rc1` holds the merged branch; from the main checkout `-d` would check against its own HEAD and fail). | orchestrator | §1 | the gate passes on the merged tree, formatted and clean; the removals reported |
| MG | **The merge of `<prev-tag>`** (S2 as amended on 2026-11-15: the tag of the release immediately before this one in the maintainer's order), where the branch was made from `<lane-pred-head>`: once, after M (the last unit before the merge, its `--quick` passed: the lane's gated head, rule 20), when every review is closed with its fixes and the tag exists, before LC, X, V and the close-out (VF runs inside it); never while a unit's edits are uncommitted. The steps run in rule 21's order, (1), (3), (6), (7), (4), then (5) (rule 21's (2) is superseded by S6). **(1) The merge**, in rule 21 (1)'s order: the tag fetched with `git fetch origin tag <prev-tag>` only where `git rev-parse -q --verify <prev-tag>` fails, and in any case the first field of `git ls-remote origin 'refs/tags/<prev-tag>'` compared with `git rev-parse <prev-tag>` (the tag ref's own object on both sides, equal for a lightweight tag, as this project's release tags from v0.5.9 to v0.7.0 are, and for an annotated one; tags are shared by every worktree, and a release tag has been moved before): no line printed, or a different sha, is a stop; then rule 18's ancestry check, `git merge-base --is-ancestor <lane-pred-head> <prev-tag>` exits 0 (the tag contains the lane predecessor's release; it catches a squash or rebase merge by the maintainer), a stop where it fails; then each untracked file the merge would overwrite (in the core worktree, the paths both in `git ls-files --others --exclude-standard` and in `git diff --name-only --no-renames --diff-filter=A <lane-pred-head> <prev-tag>`) compared (`cmp`) with the tag's (`git show <prev-tag>:<path>`), removed where equal, a stop otherwise; then, with the HEAD before it recorded, `cd <core worktree> && git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>` (diff3, so that every conflicted hunk carries its base section), which never commits on its own, so that the regeneration and the resolutions below always come before the merge commit (rule 17); then `Cargo.lock` and the version comparison: a conflicted manifest (`Cargo.toml`, `native-theme/Cargo.toml`'s comment among them where C.3 edited it) is resolved first, by the resolution's rules below, before any cargo command; where the merge left `Cargo.lock` conflicted, `<prev-tag>`'s side is taken (`git checkout --theirs Cargo.lock`) and this release's lock changes are re-applied as rule 21 (1) item 5 says (this release changes the lock only at V, after MG, so none is expected), then step (3)'s comparison is read, here so that a stop wastes no build; last, `<work>/tools/slot.sh <work>/tools/lowprio.sh cargo metadata --locked --format-version 1 > /dev/null` passes (it builds nothing, and catches a lock that merged cleanly but is inconsistent). The orchestrator then resolves each conflicted hunk, named in the running record: by both sides' quoted texts (S2), a citation like any other text (S6), a stop where a resolution would change a text's meaning, never running `git checkout --conflict=…` on a file it has touched; `CHANGELOG.md` with this release's section first and `ROADMAP.md` with E.3's section after `<prev-tag>`'s (each in release order); a generated file (`mapping.md`, and every file a generator writes) regenerated by its command on the merged tree through the wrappers under the gate lock (`flock <locks>/gate.lock <work>/tools/slot.sh <work>/tools/lowprio.sh …`, rule 2; for `mapping.md`, D.2's command), never resolved by hand, always as a long run (rule 21 (1) item 8: at every MG `<prev-tag>`'s V has bumped the workspace crates in `Cargo.lock`); where the merged tree does not build before VF (egui 0.37 arriving, say), the generated file is taken from the HEAD recorded before the merge, recorded as pending, and regenerated in VF's code commit, or, where VF makes no commit, by the orchestrator in a commit of its own before the warm-up gate, or, where the tree builds only after a warm-up fix, in that fix's commit before the warm-up runs again; a prose count is resolved to `<prev-tag>`'s side and named (X recounts it); a count or verdict tuple a test or gate checks (under D10's order the egui connector's `mapping_tests.rs` leaf count and verdict tuples, and `one_gsettings_read_per_key`'s counts) is the base plus both sides' deltas; and where both sides re-pointed the same gate-read line (a colour claim `connectors/native-theme-gpui/src/showcase.rs`'s `every_colour_claim_is_read_at_the_line_it_cites` reads, S6's fourth gate, or a rule 25 line), its number is the base plus both sides' deltas, which the warm-up gate confirms. Then, conflicted or clean, `git add` of the resolved and regenerated files; then the marker check (below the table: a `grep` of `git diff --cached -U0` for a staged conflict marker, diff3's base marker of seven bars included, which prints nothing, else MG stops, §6; a marker-like line that `<prev-tag>`'s own file has at that place is not counted); then `cargo fmt --all --check` passes (on a failure, `cargo fmt --all` runs, the files it changed are re-staged and named in the running record, and the check runs again; a file it changes outside the staged set is a stop, §6); before the commit, the staged-path check: a path in `git diff --cached --name-only --no-renames` that is neither in `git diff --name-only --no-renames <lane-pred-head> <prev-tag>` nor a regenerated file named in the running record stops MG (§6), and the record names it; then `git commit --no-edit --cleanup=strip` makes the merge commit. `<merge>` is recorded; the parents check: `git log -1 --format=%P <merge>` prints the HEAD recorded before the merge and `git rev-parse <prev-tag>^{commit}`, in that order; `grep -m1 '^## \[' CHANGELOG.md` prints this release's heading, and `git status --porcelain --untracked-files=no` is clean. **(3) VF** (rules 19 and 25): step (1)'s version comparison, `Cargo.lock`'s versions of the crates P2 listed against P2's, a read that needs no gate. A version outside P2's accepted set (egui 0.36.2 or 0.37.x, patch releases of the others) is P2's stop (§6); a version within it is VF's rework, the task row below, made here, before the warm-up gate; a source the new version needs that Cargo's registry lacks is fetched by the orchestrator with `cargo fetch`, which builds nothing. **(6) The path check** (rule 22 (b)), as P0 describes it, on the merged tree; a moved path is re-pointed by a `worker` in a documents-only commit, covered by the warm-up gate (rule 21 (5)). **(7) The survived-the-merge check**, by a `worker` (no cargo), on the merged tree, for units 0 and 2 to 6, each of this release's blocks compared with its state at `<merge>^1` (not with an earlier unit's script, whose blocks a later unit may have changed; a prose count resolved to `<prev-tag>`'s side in (1) is listed for X's recount, not compared), and the anchors of the later tasks found on the merged tree (V's CHANGELOG heading and compare-link place, every status line the close-out sets, the archive row): the texts this release replaced are absent, the remaining placement anchors and this release's new texts present; A.8's three rows right after `caret_color`; `grep -m1 '^## \[' CHANGELOG.md` prints this release's heading; E.3's ROADMAP section stands after `<prev-tag>`'s; task A's struct-literal and count searches (its part (b)) re-run, each count equal to the base plus both sides' deltas. The checks read HEAD as it stands at step (7), after (3) and (6): the blocks VF rewrote in (3) are expected differences from `<merge>^1`, listed in the running record from VF's commits, each compared with the form the specification gives for the new version; a survived check never undoes VF (rule 21 (7)). A claim that does not hold is a fix in a commit of its own before the warm-up, which covers it, or a stop where its cause is unclear (§6). **(4) R-MG**, where a file was resolved in (1) or written by (7), except Markdown that no test parses (so `.rs` files, manifests, workflows, configuration files, a TOML a test reads, a script, `docs/platform-facts.md`, a README, which is compiled as doctests and parsed by tests, and `connectors/native-theme-egui/src/mapping.md`, which `mapping_doc.rs` reads, are in), where step (1) resolved `Cargo.lock`, or where VF committed anything: a `review` agent (a `review-hard` where they change a hard part of a core: a resolver, validation, a runtime, concurrency, `unsafe`, as `windows.rs`'s call with C.3) reads `git show --remerge-diff <merge>` and every commit of steps (3), (6) and (7), with the running record's list of resolved hunks (VF's review is always within R-MG); a must-fix is a new commit by the author of the commit it fixes. Agents in the steps before (5) run no cargo but `cargo fmt --all --check`, which builds nothing (a regeneration is the orchestrator's long run), but VF's `implement` agent, which may run one `nice 19` `-p` build or clippy of the crate it rewrote, through `slot.sh` (rule 21). **(5) The warm-up gate**, last (`--commit` on the clean tree, §2.1), with U5's item (2) GPUI Fast commands run beside it as long runs (rule 21 (5): always, since the merge brings the other side's code and `<prev-tag>`'s version bump, and `--commit` skips both legs, `scripts/check_release.sh:710-716`, `:849-855`), covering every commit of steps (1), (3), (6), (7) and (4) (VF's, an R-MG must-fix, a path re-point, a fix the survived check needs), which have no gate of their own; each fix after it is a commit with its own gate (rule 21). A red warm-up goes, with the log's failing part, to the author of the unit whose rule the failing test checks (a full struct literal, a count, a mapping), even where the failing file is the other side's, or, where that author is no longer at hand, to a new `implement` agent with the log and that unit's part of its specification (rule 12); it is the orchestrator's where a conflict resolution caused it, and a stop where its cause is unclear; the fix of a red warm-up is gated by the warm-up run again, the GPUI Fast commands beside it again (rule 12). Nothing is pushed before MG. After the merge, "the release's commit range" (the reviews', the record's, the pull request's) is `git log <prev-tag>..HEAD` (rule 21: it keeps the commits this release's docs lane merged in and leaves out what `<prev-tag>` brought). | orchestrator; a `review` agent for R-MG; a `worker` for (6) and (7); VF's agents for (3) | S2; §1; rules 12, 17 to 22, 25 | the ancestry check before the merge passes; `<merge>`'s parents as above (an after-merge tag-ancestry check would hold by construction and is not made); the marker check and the staged-path check found nothing; every conflict with its resolution, R-MG's review where one was needed, the path check, the version comparison and the survived check, every claim holding, are in the running record; the five checkers pass; the warm-up gate passes; `git status --porcelain --untracked-files=no` prints nothing |
| VF | **The version forms after MG** (rule 25), MG's step (3), after the version comparison and before MG's warm-up gate: `<lane-pred-head>` is the steps 4–5 release's pre-merge head, so egui 0.37 and the icons normally reach this branch only at MG. For every crate of P2's list whose version changed within P2's accepted set: (a) a `worker` re-reads, in the new version's sources in Cargo's registry, every upstream line the specification cites for it (`EG`, `IW`, `GC`, `GB` and windows 0.62.2's lines, and E.4's backlog texts that name a version: gpui-base 0.9.1 `src/input/base/blink_cursor.rs:8`, windows 0.62.2 `src/Windows/Win32/UI/WindowsAndMessaging/mod.rs:6108`), and for egui the specification's D.2 paragraph on egui 0.37 (its `TextCursorStyle` fields, defaults and `paint_text_cursor` lines), each result in the running record; (b) an `implement` agent re-writes the forms this release's units took on the older version in code, tests and the texts in code that name a version, each only where its content (behaviour, value, API) changed in the new version, never to update a version-pinned line citation that still holds (rule 25, S6), except a line a gate reads by number (the gpui showcase's `HOW` texts and colour claims, read via `CITED_CRATES`, and the fallback-stack test's gpui-pre lines, rule 25), which is re-pointed where the new version moved it (D.2's `base.rs`, `contract.rs` and `mapping_tests.rs` where they read `TextCursorStyle`; the iced and gpui texts that name `iced_widget 0.14.2`, gpui-component 0.9.1 or gpui-base 0.9.1 lines, D.3, D.4), and regenerates a generated file MG left pending (`mapping.md`, by the orchestrator's long run of D.2's command under the gate lock, rule 2); (c) a `worker` re-writes the documents' forms, editing only repository texts this release wrote (the READMEs' and E.4's texts that name a version or its lines), and only where their content changed in the new version, never to update a version-pinned line citation that still holds (rule 25, S6), never the specification. No agent runs cargo but `cargo fmt --all --check`, which builds nothing, and (b)'s `implement` agent's one `nice 19` `-p` build or clippy of the crate it rewrote, through `slot.sh` (rule 21); a source the new version needs that the registry lacks is fetched first by the orchestrator (`cargo fetch`, MG step (3)). Where nothing changed, VF records "none needed" and commits nothing. | `worker`; `implement`; `worker` | MG's step (3) comparison; the specification's parts whose forms changed; §5's *Archived paths* | each re-read line and each re-written form beside its new reading in the running record; the commits reviewed within R-MG (MG's step (4)); covered by MG's warm-up gate, with no gate of their own (rule 21); `git status --porcelain --untracked-files=no` prints nothing after each. Commits: `fix(connectors): the caret blink's forms on <toolkit> <version>`, `docs: the caret blink's texts on <toolkit> <version>` |
| LC | **L's corrections**, where L found any: each corrected range applied to every copy of that citation on the merged tree, found by the source file's name and the range: the facts cell (A.8, with its sub-ranges of the GNOME schema), B.1's preset comments, C.1's rustdoc, C.2's comment (which cites the schema as one range, `org.gnome.desktop.interface.gschema.xml.in:63-85`) and E.4's windows-11 entry. One commit; no LC where every link held. | `worker` | L's table; A.8, B.1, C.1, C.2, E.4 | `git grep -n` for each corrected file name and old range finds nothing outside dated documents, and the new range is at every place the old one was; the five checkers pass; the gate (`--quick`: the first unit after MG's warm-up restarted the count, §2's table) passes, formatted and clean. Commit: `docs: the caret blink's upstream line citations, as their pages show them` |
| X | **The recount**, on the merged tree (after MG; rule 17): every stated count in every file either side of MG changed (the union of `git diff --name-only <base> <merge>^1` and `git diff --name-only <base> <prev-tag>`, `<base>` being `<lane-pred-head>`; this release's changes also take in `git diff --name-only <merge> HEAD`, the commits MG made after the merge), outside the pinned documents (rule 17: those whose lines are dated to a commit, `docs/archive/`, `docs/evidence/`, `docs/issues/`, the dated `docs/todo_v0.7.1_*` and `docs/todo_v0.7.x_*` documents and `docs/todo.md`'s dated block), recounted, the prose counts MG resolved to `<prev-tag>`'s side among them; then the five documentation checkers. No citation is re-pointed or checked against its line (S6). Where the branch was made from `<prev-tag>` (no MG), there is no X: nothing was merged, and the documents unit already ran the five checkers (rule 17). X reports whether it is a code unit (§2) from its diff; its gate is `--quick` either way, at most the second code unit after MG's warm-up (§2's table). | `worker` | rule 17; MG's `<merge>` and its list of prose counts | each count with its old and new value and where it stands, in the commit body; the five checkers pass; the gate passes on the staged change (§2), formatted and clean. Commit (where anything changed): `docs: the counts the caret blink's merge changed` |
| V | **The version** (E.6's second and third items, the release mechanics of `a3c10475`): the workspace version and the inter-crate requirements become `0.7.N`, the lock follows, the date of U6's `## [0.7.N] - <date>` heading is filled in (its section still above every other `## [` heading, rule 14), the compare link goes at the top of the link list (E.6), and, with the approval of the `unsafe` call, E.4's dated sentence is written with the same date. Before K1 (rule 22 (c)): the stamped-paths grep (*V, the stamped-paths grep*, the block after this table) prints nothing; a hit is re-pointed to the archived path in V's commit. V's `--commit` is a lock change's (rule 6). | orchestrator | E.4, E.6; B6; rule 22 (c) | in `cargo metadata`, the seven crates with `version.workspace = true` (`native-theme`, `native-theme-build`, `native-theme-derive` and the four connectors) report `0.7.N`, and the three `tools/` members keep `0.0.0`; (under B6 (b): and every README's `native-theme*` requirement moves to the new `major.minor`, as `scripts/check_readme_versions.py` requires); `git diff` of `Cargo.lock` changes only the workspace crates' versions; `python3 scripts/check_readme_versions.py` passes; the stamped-paths grep prints nothing; the gate (`--commit`) passes. Commit: `chore: v0.7.N version` |
| K1 to K8 | **The asset run and the close-out**, §2.1. | orchestrator; the maintainer for the capture window | E.6 | §2.1 |

The three commands the table names, which contain pipes and so stand outside it:

*U2, the privacy grep* (its done-criterion), run in `<core worktree>`:

```sh
grep -rnE '/tmp/|/home/|/run/media' docs/evidence/winui-host docs/evidence/ios-host docs/evidence/README.md
```

*V, the stamped-paths grep* (rule 22 (c); before K1), with `<stamped paths>`
every path of `SOURCE_PATHS` in `scripts/update_provenance.sh` and every
connector's source paths in `scripts/update_compatibility.sh` (its
`source_paths`: each connector's `Cargo.toml`, `src`, `examples` and `tests`,
for egui also `mapping.toml` and the egui widgets crate's `Cargo.toml` and
`src`), as the branch holds them. The stem matches the brace form
`todo_v0.7.x_caret-blink-{spec,plan}.md` too, and the filter works per match:

```sh
cd <core worktree> && git grep -noE '(archive/)?todo_v0\.7\.(1|x)_caret-blink-[a-z{},.-]*' -- <stamped paths> | grep -v ':archive/'
```

*MG, the marker check* (rule 21 (1); step (1), after the `git add` and
before the staged-path check), which prints nothing where no conflict marker
is staged; a printed line stops MG:

```sh
cd <core worktree> && git diff --cached -U0 | grep -E '^\+(<{7}|\|{7}|={7}|>{7})( |$)'
```

### 2.1 The asset run and the close-out

In this order; each step starts only when the one before it holds (but K4,
which runs beside K3's CI), K1 starts only after MG's merge (S2), and K3, the
push, starts only when every review of §3 is closed and its fixes committed.

**Where commands run.** Every repository command of an orchestrator task runs
in `<core worktree>`, from P2's switch on (each starts with
`cd <core worktree> &&`): the repository's
scripts, P2's and V's `cargo metadata`, `scripts/check_readme_versions.py` and
the five checkers among them. Some scripts find the repository from their own
path (`scripts/generate_assets_release.sh:21-22`), others from the working
directory (`git rev-parse --show-toplevel`, `scripts/update_compatibility.sh:37`,
`scripts/update_provenance.sh:35`), so a run from the main checkout would read
another branch. The one exception: the `cargo metadata` that finds a build
folder to delete runs in the worktree whose folder it is.

**Long runs.** The Bash tool stops a foreground command at ten minutes.
`--commit` has taken 5 to 12 minutes per commit, and 29 cold, in earlier
measurements before the 16-job cap (the project's memory notes of the code
review, read 2026-11-15); the full check takes about 23 minutes, the asset run
30 to 37 (§8). So the orchestrator runs every gate (`--quick`, `--commit`,
the full check), the asset run, the compatibility run, K5's release builds and
rehearsal, and any other run not expected to finish well within ten minutes
(the first `--commit` among them) as a long run: with `run_in_background`, from
`<core worktree>`, through the wrappers (a gate, with `flock <locks>/gate.lock`
in front of them, rule 2), its output in a log and its exit
status in a `STATUS` file at absolute paths. Every long run, and each repeat of
one (a gate after a red result, K2, K4, K5's rehearsal, K6, K8), writes into a
folder of its own, `<work>/<task>/run-<n>/` (`<n>` counting from 1), made with
`mkdir`, which fails if it exists, and named in the running record, so a
`STATUS` left by an earlier run never ends a wait early. The orchestrator waits
for the exit notice, then reads that run's `STATUS` and the log's tail. A wait
for a condition inside a long run uses the Monitor tool with an until-loop on
that run's files, such as
`until grep -q 'Theme-switching GIFs generated' <work>/K6/run-<n>/LOG || [ -s <work>/K6/run-<n>/STATUS ]; do sleep 10; done`,
never a `pgrep -f` loop, which matches itself.

**The warm-up gate.** An agent's targeted commands build the feature sets the
gate's test legs build (gpui `native,watch`, `scripts/check_release.sh:667`;
iced `iced_aw,watch,native`, `:739`; egui `--all-features`, `:762`), so before
an agent that runs cargo is dispatched the orchestrator runs
`flock <locks>/gate.lock <work>/tools/slot.sh <work>/tools/lowprio.sh scripts/check_release.sh --commit` on the clean tree, as a long run in its own
folder (`<work>/gates/warmup-<n>/`), wherever the worktree is new, `Cargo.lock`
changed since that folder's last gate, or the rustup hook reported a changed
toolchain; and after MG's merge always, whatever the merge changed (S2, MG
step (5)), there with U5's item (2) GPUI Fast commands beside it, and beside
each re-run of a red warm-up (rules 12 and 21 (5)). A red warm-up, wherever
it runs (at P0, at P2 or after MG), goes with the log's failing part to the
author of the unit whose rule the failing test checks, or, where that author
is no longer at hand (a lane predecessor's unit at P2, the other side's at
MG), to a new `implement` agent with the log and that unit's part of its
specification; it is the orchestrator's where a conflict resolution caused
it and a stop where its cause is unclear, and its fix is gated by the
warm-up run again (rule 12). Then those builds are warm and an agent's commands stay within the
Bash tool's limit. Where there is a unit 0, P0's gate serves, as it runs before
any agent that runs cargo (A runs none). Where there is none, which is the
likely case (v0.7.1's commit (0) adds every approved document of this cycle,
the `docs/todo_v0.7.x_*` plans among them, and sets their status lines:
`docs/todo_v0.7.1_gpui-kit-0.9.1-plan.md`, P0 and §7 item 1), P2 runs the
warm-up once its checks hold. The first `--commit` run, P0's gate or P2's
warm-up, is timed and its duration written into the running record, with
the note whether it ran warm (`kde-b` reused, its build folder from the
steps 2–3 release) or cold (`kde-b` made new, its build folder empty).

1. **K1.** `cd <core worktree> && cargo audit` passes with the repository's
   `.cargo/audit.toml`, through the wrappers.
2. **K2.** `cd <core worktree> && flock <locks>/gate.lock <work>/tools/slot.sh <work>/tools/lowprio.sh scripts/update_compatibility.sh run`
   (a heavy run, rule 2), as a long run, passes; it leaves `docs/COMPATIBILITY.toml` and the connector
   READMEs changed and uncommitted until K7. K2 failing (a newer upstream
   release the connectors fail on) is §6's upstream move; a newer release that
   passes is stamped as the script does. An advisory stops the run (§6).
3. **K3.** Once K2 has passed, the branch is pushed
   (`git push origin v0.7.N-rc1:v0.7.N-rc1`), the specification's one push of
   the work (the batch's S4: the push comes before the full check, which runs
   beside CI, K4; `docs/todo_v0.7.1_batch-speed-rationale.md`). Nothing is pushed
   before MG's merge (S2). CI runs on pull requests and on `main`
   only (`.github/workflows/ci.yml:6-10`), so the push checks nothing until a
   pull request from the branch is open: at this first push the orchestrator
   opens a draft pull request from `v0.7.N-rc1` to `main`
   (`cd <core worktree> && gh pr create --draft --base main --head v0.7.N-rc1`,
   with a provisional title naming the branch and the release and no other
   description; the final title and description stay the maintainer's to
   set, drafted and shown first, §2.2; rule 9, approved by the maintainer on
   2026-11-15, §7 item 4). The
   push's notice to the maintainer says, with C.3 done, that this run is the
   first of C.3's Windows-only test (C.3, *Not verified*), since a red run
   emails the maintainer. The run
   is green, and with C.3 the plan reads that the test passed: the logs of the
   jobs *Test Windows (windows)* and *Test Windows (windows, watch,
   system-icons)* (`ci.yml:222-223`, each `cargo test -p native-theme` with the
   `windows` feature) each contain
   `windows::tests::the_windows_reader_maps_the_systems_blink_time ... ok`
   (`gh run view <id> --log`, the lines kept in `<work>/ci/`). A red run is
   §6's.
4. **K4**, while K3's CI runs (S4): the full `scripts/check_release.sh`, run in
   `<core worktree>` as `flock <locks>/gate.lock <work>/tools/slot.sh <work>/tools/lowprio.sh scripts/check_release.sh` (rule 2), as a long run, fails on the
   provenance stamp alone (K2 renewed the other). Its warnings are named in the
   running record; the outdated-dependency warning lists what it lists at the
   run. The semver step (soft, a warning) is expected, by B6's answer: under
   (a), a 0.7.x number, to name `InputTheme`'s three added fields and nothing
   else (E.6; the break E.1 names); under (b), a minor number, which Cargo's
   rules let a 0.x release break, and under (c), where the earlier release made
   `InputTheme` `#[non_exhaustive]` so that a new field breaks nothing, to name
   nothing of this work. Any other failure or warning, or a semver finding
   beyond the expected one, is a stop (§6). The cost S4 accepts: K3's CI run
   can be red, and email the maintainer, for a fault K4 would have shown first.
5. **K5**, after K4 and while K3's CI may still run (it commits nothing; K4
   and K5 both run cargo, and the two slots are shared with the other lane,
   §6), in `<core worktree>`: the
   three showcases built for release with the capture scripts' own commands
   (`scripts/capture_window.sh:440`, with each script's features), as long
   runs, each a heavy run under the gate lock
   (`flock <locks>/gate.lock <work>/tools/slot.sh <work>/tools/lowprio.sh …`, rule 2), so that the desktop is needed for the capture alone; and a rehearsal
   of the parity step's layout half without the desktop, held by the shared
   lock `<locks>/xvfb97.lock`, so that it waits mechanically while the other
   lane's rehearsal runs (one private-X rehearsal at a time on Xvfb display
   `:97`, rule 24): each showcase started
   on a private X server with `--dump-layout`, with the presets the capture
   scripts use, by a copy of
   `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/batch-2026-11-15/gates/rehearsal.sh`
   at `<work>/tools/rehearsal.sh`, its worktree path (`W`, which the original
   hard-codes as `/home/tibi/Rust/native-theme/.claude/worktrees/v071`) set to
   `<core worktree>`'s absolute path (under the proposal
   `/home/tibi/Rust/native-theme/.claude/worktrees/kde-b`, rule 24), the
   value written into the running record. Before it runs, the
   copy's `ENTRIES` are checked against `scripts/capture_window.sh`'s
   `SCREENSHOT_ENTRIES` on the branch (`:352` at `0fa28027`), and brought into
   line where they differ. The two commands, the first a long run into its own
   folder (`<work>/K5/run-<n>/`, §2.1):
   `flock <locks>/xvfb97.lock <work>/tools/rehearsal.sh <work>/K5/run-<n>/dumps`, then
   `cd <core worktree> && python3 scripts/check_showcase_parity.py <work>/K5/run-<n>/dumps > <work>/K5/run-<n>/parity.out`.
   The rehearsal is complete only where its `summary.txt` holds a line for
   every kind and entry, none of them `dump=no`, and ends with `DONE` (rule
   19: the parity checker skips a property fewer than two kinds give,
   `scripts/check_showcase_parity.py:1080-1081`; a kind missing from a group
   shows as an unexcepted "(dump)" row, `:1034-1037`, but an entry with no
   dump of any kind forms no group at all, `:740-757`, and goes unnoticed). Any `dump=no` line leaves
   K5 incomplete, and K6 does not start (rule 19: "K5 requires the
   rehearsal's summary to hold no failed-dump line"). One that this work's
   changes explain is a fix after M (§6), and the rehearsal runs again after
   it; one they do not explain is run once more, into
   `<work>/K5/run-<n+1>/`, and any `dump=no` in that second run is a stop
   (§6), named in the running record.
   It compares geometry only and runs without a window manager, before the
   capture: a difference it shows is reported to the maintainer while the
   capture goes ahead (rule 19). The fault is a difference the asset run's
   comparator shows after the capture (K6).
6. **K6.** When both are green (rule 9: K3's CI green, and K4's full check's
   only failure the provenance stamp, which only the asset run refreshes) and
   K5 is done,
   the asset run (it needs HEAD pushed and the stamped paths clean, `:74-92`)
   starts once, as a long run, in a run folder made first with
   `mkdir <work>/K6/run-<n>` (§2.1). It takes the shared lock
   `<locks>/assets.lock`, the gate lock `<locks>/gate.lock` and a cargo slot
   before the maintainer is asked (one asset run and one capture window at a
   time across both lanes, rule 24; no other heavy run beside it, rule 2),
   marks that it holds them (`HELD`), and waits for `GO`:

   ```sh
   flock <locks>/assets.lock flock <locks>/gate.lock env CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16 <work>/tools/slot.sh sh -c 'touch <work>/K6/run-<n>/HELD; until [ -e <work>/K6/run-<n>/GO ]; do sleep 5; done; kde-inhibit --power --screenSaver --notifications bash -c "cd <core worktree> && bash scripts/generate_assets_release.sh > <work>/K6/run-<n>/LOG 2>&1; echo \$? > <work>/K6/run-<n>/STATUS"'; echo $? > <work>/K6/run-<n>/OUTER.STATUS
   ```

   The wait for `HELD` is a Monitor until-loop that a failure before `HELD`
   also ends (rule 24):

   ```sh
   until [ -e <work>/K6/run-<n>/HELD ] || [ -s <work>/K6/run-<n>/OUTER.STATUS ]; do sleep 10; done
   ```

   Once `HELD` exists, the orchestrator asks the
   maintainer for "capture now": the desktop unlocked and untouched for 4 to 6
   minutes, announced before it starts; on the maintainer's word,
   `touch <work>/K6/run-<n>/GO` starts the capture. `kde-inhibit` takes the
   three options the earlier asset runs used, as the session transcripts show
   them, read 2026-11-15, and the asset-run memory note records. It
   holds a cargo slot through `slot.sh` but does not run under `lowprio.sh`:
   the earlier runs made the captures at normal priority, the desktop is
   reserved for them, and the captures are timing-sensitive (settle times,
   the 20 s window and layout-dump time-outs, `scripts/capture_window.sh:36`,
   `:384-394`); the one exception to §5's *Cargo* rule. The thread caps that
   `lowprio.sh` would set are set before `slot.sh` instead (§5).
   `kde-inhibit` hides the script's exit code, so the orchestrator reads
   `STATUS` and the log's tail, never the wrapper's status. When the log shows
   "Theme-switching GIFs generated" (`scripts/generate_assets_release.sh:172`,
   the last step that captures), which the orchestrator waits for with the
   Monitor until-loop above, it tells the maintainer that the desktop is free
   again for the maintainer; the machine stays busy until the
   script ends, since it then waits for the CI screenshots and its last step
   runs `scripts/update_compatibility.sh run` (`:278`), a set of builds and
   tests of its own. No agent edits the tree until the run has reported. A stop commits nothing. A parity
   difference the asset run's comparator shows after the capture is a fault
   (rule 19): a close-out fix by its author (rule 15) and the run made again,
   or a reviewed exception for a toolkit's limit, the maintainer told either
   way.
   Then the
   orchestrator
   opens the new screenshots beside the previous release's and tells the
   maintainer what changed (the showcases' Widget Info of the focused text input
   is what this work changes, D.4).
7. **K7.** One commit: the assets, both stamps, the READMEs' lines, the
   changelog heading's date (and E.4's dated sentence, where written) set to
   the commit's day where it differs from V's, and the archive (§2.2). Pushed with the same
   refspec: the release needs the stamps on the branch, a push beyond the
   specification's one, made as 0.7.1's close-out made it.
8. **K8.** The full `scripts/check_release.sh` at K7's commit, run in
   `<core worktree>` as K4's is (with `gate.lock`, rule 2), as a long run, reports
   no failure, and K7's CI run is green. Both come after the last commit, so they
   are reported to the maintainer (the summary line, the run's id and
   result), not committed.

### 2.2 Archiving, the last task

In K7's commit: the rationale, the specification and this plan move to
`docs/archive/` with their names; `docs/archive/README.md` gains E.6's row
after the newest release's row; 0.2's two links in `docs/evidence/README.md`
are rewritten for the new place (E.6), and the tracked later documents' links
(below); the three
documents' links to each other name files in the same folder and are checked,
not rewritten; the status lines say implemented, with the commit range; the
running record is copied into §9, the plan's only edit beyond the archiving.
Only where this release is the batch's last in the maintainer's order (under
the proposal it is not: the KDE steps 6 and 7 come after it), K7 also
archives `docs/todo_v0.7.1_batch-speed-rationale.md` by the same rules
(rule 22 (a)). None
of these paths is stamped; K8 confirms it.

K7 also rewrites every path mention of the three moved documents in every
tracked file outside `docs/archive/` (rule 22 (a), which replaces
the later-links handoff: no plan writes or reads a `k7-later-links.md`).
After the move they are found with one pattern for all three:
`cd <core worktree> && git grep -nE 'todo_v0\.7\.(1|x)_caret-blink-(rationale|spec|plan)\.md' -- ':!docs/archive'`;
each hit that names the `docs/` path is rewritten in place to the archived
path in K7's commit and named in the running record; a mention that already
points into `docs/archive/` in any relative form or as an absolute URL is
left, and so are a citation dated "at <sha>" and a path inside another
document's instructions that names its own future target, each named in the
commit body. A document whose header dates its lines is exempt only for line
citations, never for path mentions. The check filters per match, not per
line, and prints nothing but the mentions those exemptions leave, each named
in the commit body:
`cd <core worktree> && git grep -noE '(archive/)?todo_v0\.7\.(1|x)_caret-blink-(rationale|spec|plan)\.md' -- ':!docs/archive' | grep -v ':archive/'`.
The moved documents' own relative links are re-checked after the move, since
they now resolve from `docs/archive/`, with rule 22 (b)'s exemptions
(placeholders, quoted texts of other files, URLs) (rule 22 (a)). An untracked document
that names them is not searched: the plan that commits it runs its own path
check (rule 22 (b)).

After K8 the orchestrator runs
`~/.claude/scripts/sweep-target.sh <core worktree>` and reports the sizes it prints; it runs it also whenever the core worktree's build folder passes about 200 GB during the work. No `cargo clean`. The report and the project status note name that `kde-b` and its build folder stay for the KDE lane's later releases (§1). **Merging the pull
request, tagging, publishing and the GitHub release are the maintainer's** and
are not part of this plan. The pull request's title and description are
drafted by the orchestrator and shown to the maintainer first; setting them is
the maintainer's.

## 3. Reviews

Every commit that changes behaviour is reviewed before M, which waits for
those reviews, and a later fix before K3. The reviewer gets the commit range and the
specification's header and parts, never the author's report; it reads the
commits (`git show`), not a working tree, and runs no cargo. A review runs while
the lane's next task works. A must-fix is a new commit by the task's author (the
same agent, continued), made after the commit of the task then running in the
lane, with the task's done-criterion run again, and reviewed by the same
reviewer; the fix is staged by its author and gated and committed by the
orchestrator (§2). A second must-fix on the same point falls under §6's
take-over rule. R4w's brief, like U4's, carries *Open points* item 2. Every
reviewer, the orchestrator included, also checks that no new code
transcribes an LGPL source (§5, *The licence rule*).

"By size": a commit of up to a few hundred changed lines an Opus agent wrote is
reviewed by the orchestrator, a larger one by a `review` agent; the orchestrator
reads `git diff --stat` and says which it chose.

| Review | Commit | Reviewer | Why |
|---|---|---|---|
| R3h | U3's validation: `native-theme-derive/src/parse.rs`, `gen_ranges.rs`, `native-theme/src/resolve/validate_helpers.rs`, `validate.rs` | `review-hard`, narrow | the maintainer's delegation table names validation: the new rule over three fields (A.4), the derive's new range arm that every generated validation may take (A.1), and their place before the Phase 1 short-circuit. Small, so the Fable review is short; it runs first, so that R4w waits for it and one Fable agent runs at a time |
| R3 | the rest of U3 | `review` | values and texts across many files (the fields' docs, the registry and rules, the facts rows, the presets, B.2's gate, egui's rows and formulas, A.5's tests against A.4's table); too large for the orchestrator |
| R4 | U4, but `windows.rs` | by size | parsing and halving of two readers' settings, each case a test (C.1, C.2) |
| R4w | U4's `windows.rs`, with the approval | `review-hard` | the one new `unsafe` call of the crate, under the maintainer's strict rule for `unsafe`; the hunk is small, so the Fable review is short. It reads the `SAFETY` comment against the specification's quote of Microsoft's page in open point 2 (no argument, no precondition stated; the blink time, `INFINITE` and 0), the `allow` placed per function, and that `read_windows` cannot skip the mapping |
| R5 | U5 | by size (expected `review`) | gpui's write rule across `apply` calls, told apart by value as the tooltip delay is (D.1 item 3), the tests that must fail without it, iced's exceptions |
| — | P0, U2, U6, LC, X, V, K7, M's conflict resolutions (the docs branch's texts, M), MG's resolutions in files that are no code file, MG's path commits, a formatting-only commit (§4) | the checkers, then the orchestrator reads the diff | data and texts the specification fixed; the gates and the extraction scripts compare them |
| R-MG | MG's resolutions in every file but Markdown that no test parses (`.rs` files, manifests, workflows, configuration files, a TOML a test reads, a script, `docs/platform-facts.md`, a README, `connectors/native-theme-egui/src/mapping.md`) and of a conflicted `Cargo.lock`, and every commit of MG's steps (3), (6) and (7), VF's among them (rule 21 (4)) | `review`, or `review-hard` where they change a hard part of a core (`windows.rs`'s `unsafe` call with C.3) | `git show --remerge-diff <merge>` and those commits, with the running record's list of resolved hunks; VF's forms each against its new reading; a must-fix is a new commit made before MG's warm-up gate, which covers it (rule 21) |

At most one Fable agent runs at a time: R3h first, R4w after it, and either
waits while an `implement-hard` runs.

## 4. Gates

- **While working**, an agent runs the tests of the module it changes. **Before
  it stages its change for the gate**, it runs its task's targeted commands
  once, in the foreground, through the wrappers (§5), or as §2.1 says where a
  command is a long run.
- **A gate at every core-lane task's end** (P0, U2, U3, U4, U5, M, MG (whose
  warm-up covers VF's commits and its path commit), LC, X, V), run by the orchestrator as a long run
  on the staged change: `--quick`, or `--commit` where §2's cadence (rule 6)
  names it (P0's or P2's warm-up, U4, U5, MG's warm-up, V;
  §2's table). What `--commit` skips and
  this work needs is run by the orchestrator the same way, as part of that
  gate and so under its lock (`flock <locks>/gate.lock …`, rule 2): the gpui
  connector's tests on GPUI Fast and the gpui showcase's GPUI Fast
  runner-environment leg (`scripts/check_release.sh:710-716`, `:849-855`; U5,
  and MG's warm-up and each re-run of it after a red one, rules 12 and
  21 (5)). The cross-target checks run in `--commit`; a run that skipped one does
  not count (§2).
- **Formatted and clean.** The script's `--commit` runs `cargo fmt --all` in place
  (`scripts/check_release.sh:39-41`, `:518`) and only reports a dirty tree
  (`:345-346`), while CI's `cargo fmt --all --check` fails the push and the
  other jobs wait on it (`needs: fmt`). So every task that edits the tree's sources or comments (U3, U4, U5, U6, M where its conflict resolution edits a `.rs` file, MG step (1)'s merge commit where a resolution edits a `.rs` file, MG's path and survived-check commits and its R-MG must-fix, VF, LC, X)
  runs `cargo fmt --all --check` before its commit; a task's gate runs on the
  staged change; `git status --porcelain --untracked-files=no` prints the same
  before and after the gate, and nothing after the commit. Before each gate
  and again before the commit, the staged tree is the whole tree (§2):
  `git diff --quiet` exits 0,
  `git status --porcelain --untracked-files=all` shows no `??` line but the
  planning documents P2 recorded by name, and `git diff --cached --name-status`
  matches the task's list of new and changed files. The one exception is
  K2's uncommitted `docs/COMPATIBILITY.toml` and connector READMEs, until K7. A
  reformat the gate makes outside a task's own files is not folded into that
  task's commit: it becomes a formatting-only commit of its own, which the
  orchestrator reads (§3).
- **The documentation checkers** (no cargo): `scripts/check_platform_facts_rows.py`,
  `check_section_refs.py`, `check_preset_citations.py`,
  `check_wrong_safety_nets.py`, `check_todo_citations.py`
  (`scripts/check_release.sh:965-983`), from the worktree's root, each with no
  finding.
- **The two release stamps** are stale from U3 on (it edits sources the stamps hash,
  the specification's E.6) and are no task's done-criterion before K8.
- **The minimum supported Rust version**: CI's MSRV checks run in CI and in
  `--commit` and `--quick` of the ported script (the gate-speed
  specification's O3, approved by the maintainer on 2026-11-15: the
  `+1.88.0` checks build in a fresh folder deleted after every run, O2 (e),
  and `--quick` skips them by key); the full run is unchanged (O1a and O1b
  answered no). The briefs say:
  no language or library feature newer than the edited crate's own
  `rust-version`: the workspace's, `1.88.0` at `Cargo.toml:20`, for the core,
  the derive, iced and the showcase support; the gpui and egui connectors' own,
  `connectors/native-theme-gpui/Cargo.toml:11` and
  `connectors/native-theme-egui/Cargo.toml:8`; each as it stands on the branch.
- **The full `scripts/check_release.sh`**: at K4, beside K3's CI (one failure,
  the provenance stamp), and at K8 (none), each a long run by the orchestrator
  (§2.1).
- **CI** runs only once a pull request is open (K3). Pushes, each with an
  explicit refspec and none before MG's merge (S2): K3 and K7. The
  orchestrator watches each run in the background. A red run emails the
  maintainer; no push is made that is expected to be red. If one is red, the
  orchestrator tells the maintainer what failed and why, and it is fixed as
  §6's after-M rule says, its gate, push and full check by rule 15.

## 5. Rules every agent brief carries

- **Inputs.** The brief gives the specification's header and the task's parts;
  the parts are the whole task: nothing beyond them, no refactoring, no
  formatting churn. A needed change the parts do not name is reported, not made.
  Every edit is placed by its anchor (the quoted text, the key, the test's
  name), a count is the tree's value plus the specification's delta (the
  header), and a citation is found by the text it cites, never by the number the
  specification gives; where a quoted text does not stand, the agent stops and
  reports.
- **The worktree.** The brief names the lane's worktree by its absolute path;
  every command starts with `cd` into it or uses `git -C` and
  `--manifest-path`. The main checkout and the other worktrees are other branches
  with the same relative paths: nothing is read, run or written there but
  `<work>` (P1's copy of the documents is there), and, read-only, P1's copy
  from `.claude/worktrees/v071/docs/`, and a check
  that passes there proves nothing. The report quotes
  `git -C <worktree> log -1 --oneline`.
- **The licence rule (the maintainer's D10, not the KDE document's):** no code
  from an LGPL or GPL source. GTK is LGPL: its `gtksettings.c` and `gtktext.c`
  were read for facts only, and no code comes from them; plasma-integration and
  Qt likewise. A function an LGPL source is cited for is written only through
  the clean room; every derivation here is written from the specification's
  description. Every implementer brief (U3, U4, U5) says that the implementer
  opens none of the LGPL sources the specification cites (`khintssettings.cpp`,
  `qwidgetlinecontrol.cpp`, `qquicktextinput.cpp`, `gtksettings.c`,
  `gtktext.c`): the specification's description is the clean room's
  description, and the tests of C.1 and C.2 its test vectors. An Apache-2.0
  port keeps a file-level notice (gpui-component and gpui-base are Apache-2.0;
  none is planned).
- **Hooks.** Source files are edited with the Edit and Write tools only, with
  two exceptions: U2's ten recorded files are brought in by `git restore`;
  D.2's generated `mapping.md` is written by its command (at MG and VF too).
  No agent re-points or converts an existing line citation (S6). The
  project's hooks block invented values and panic-prone code; a
  blocked edit is obeyed, never bypassed, and blocked content is never written
  another way (shell, `sed`, a here-document, a script).
- **Archived paths** (rule 22 (c); in the briefs of U3, U4, U5 and VF). Code,
  tests and rustdoc that name one of this work's three documents (the
  rationale, the specification, this plan) name its archived path,
  `docs/archive/<name>`, since the close-out moves them after the asset run
  and the moved path would otherwise change stamped sources.
- **Code.** No `unwrap`, `expect`, `panic!`, panicking index or unchecked
  arithmetic in non-test code (decisions 5 and 11 give the forms); no `unsafe`
  but C.3's one call, and that only with the maintainer's approval; no public
  item removed. No invented value.
- **Cargo.** Every cargo command, and every script that runs cargo, goes through
  `<work>/tools/slot.sh <work>/tools/lowprio.sh <command>` (P1), in the
  foreground, or as §2.1 says where it is a long run. A command's environment variables go before the wrappers
  (`VAR=… <work>/tools/slot.sh <work>/tools/lowprio.sh cargo …`), since
  `lowprio.sh` ends in `exec "$@"` and would take `VAR=…` for the command.
  `build-dir`, `CARGO_TARGET_DIR` and `--target-dir` are never set;
  `cargo clean` is never run. At most two cargo processes at once on the
  machine, for this lane and the second lane together (S3): one gate, and one
  `nice 19` targeted build in the other lane; the two slots, whose lock folder
  `<locks>` both lanes' wrappers share, enforce it, and every gate run takes
  `<locks>/gate.lock` before its slot (`flock <locks>/gate.lock …`, rule 2),
  which serializes the gates. In this work only the core worktree builds, and the docs lane's
  `cargo fmt --all --check`, which builds nothing, takes a slot through the
  wrappers too, so it counts as one of the two while it runs. The heavy runs (every
  gate, every other command run as part of a gate: the GPUI Fast legs and
  U4's CI legs run beside it, K2's compatibility run, K5's
  release builds and K6's asset script) take `<locks>/gate.lock` too (rule
  2); agents' targeted `-p` commands take only the wrappers. K6's asset run
  holds a slot and the gate lock without `lowprio.sh` (§2.1).
- **Reach.** No `find /` and no other search of the whole file system. Reads and
  writes stay inside the lane's worktree, Cargo's home and build folder (the
  registry for upstream sources), the session's scratch folder, `<work>` and
  `<locks>`. The session's scratch folder does not outlive the session: its
  contents are copied to
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/batch-2026-11-15/`,
  and any input this plan takes from that scratch folder is taken from there.
  No agent writes this release's record (§9). Nothing appears on the maintainer's desktop.
- **Work.** A new test is written first and seen to fail. A changelog entry is
  one physical line. The agent runs its targeted commands in the foreground
  (none that is a long run, §2.1), stages its own paths by name, does not run
  `--commit`, does not commit (but U6's worker in the docs lane, §2) and does
  not push, and reports once: the staged files, the commit message with no
  attribution line, each targeted command with its result line, and everything
  it was unsure of. The orchestrator runs the gate and commits (§2).
- **Something the maintainer must hear before a commit** (a quoted text that
  does not stand, a link that does not hold, the reproduction failing): the agent
  reports it with its work, and the orchestrator commits nothing of it until the
  maintainer has been told.

**`<work>`** is `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.x_caret-blink-plan/`,
in the main checkout and excluded by `.git/info/exclude` (`.superpowers/`):
`tools/`, `docs/`, `anchors/`, `u2/`, `u3/`, `u6/`, `links/`, `ci/`, and
one folder per task that has long runs (`P0/`, `U2/`, `U3/`, `U4/`, `U5/`, `M/`, `MG/`, `VF/`, `LC/`, `X/`, `V/`, `K2/`, `K4/`, `K5/`, `K6/`, `K8/`), in
which each run makes its own `run-<n>/` (§2.1), and `gates/`, which holds each
warm-up gate's `warmup-<n>/` (§2.1).
**`<locks>`** is
`~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/slot-locks/`, a
new folder beside the wrappers' originals, which P1 creates: the one fixed,
durable folder for the two slot locks and the gate lock `gate.lock` (rule 2)
that every plan of this batch uses, so that every copy of the wrappers holds
the same two slots and every gate the same lock. The briefs give the
absolute paths; no other committed file names them.

## 6. Limits and stop rules

- **Shared with the second lane** (S3 as amended): at most four agents on the machine (six
  where two of them run no cargo), both lanes' agents counted together; at
  most two cargo processes (§5, *Cargo*); one asset run, one capture window
  and one private-X rehearsal at a time (K5's rehearsal and K6 wait while the
  other lane's runs, until its `STATUS` is written; rule 24);
  never two Fable agents (`review-hard`,
  `implement-hard`, `design`) at once, both lanes counted. Within that, this
  work has one agent per worktree, task A or L, and one reviewer; U6's
  `cargo fmt --all --check` holds a slot for the moment it runs.
- **A gate failed twice** by the same task is not dispatched a third time: the
  orchestrator takes the task over, re-specifies it, or sends it once as
  `implement-hard`, and says which. For U4 the take-over is always a fresh
  `implement-hard` agent, which has read none of the LGPL sources, never the
  orchestrator (§5, *The licence rule*).
- **The stops**, each with nothing committed by the task that met it, nothing
  pushed after it until the maintainer has been told and, where the rule that
  names the stop asks the maintainer, has answered: a quoted text that does not
  stand (A or any task); an evidence object `git cat-file -e` cannot find, or a cross-target or
  the MinGW linker missing, the hook trial's Write applied, a toolkit version
  outside P2's accepted set, an untracked planning document that blocks the
  switch in `kde-b` and differs from the tree's copy (§1), or, on the tag
  path, the tag check or the ancestry check
  (`git merge-base --is-ancestor <lane-pred-head> HEAD`) failing (P2); a still untracked v071 copy of this work's
  documents that differs from its approval copy, or no rule-20 line in the
  steps 4–5 record (P1, and again at P2); Part 0's reproduction
  failing (U2); the runner environment's exit 75 (U5, and MG's warm-up); a link whose text is not found or says otherwise (L); at MG, the tag
  check (`git ls-remote` printing no line, or another sha than
  `git rev-parse <prev-tag>`), the ancestry check before the merge failing, an
  untracked file the merge would overwrite that differs, a resolution that
  would change a text's meaning, a `Cargo.lock` whose re-applied lock
  changes show more than this release's packages, a staged conflict marker
  (the marker check), a staged path MG's
  staged-path check does not account for, a path
  gone otherwise than
  to `docs/archive/`, a version outside P2's set, a red warm-up of unclear
  cause, or a survived claim that fails for an unclear cause; a `dump=no` in
  the second run of K5's rehearsal after an unexplained one (K5). A parity
  difference the asset run's comparator shows after the capture (K6) is no
  stop but a fault handled by rule 19 (one K5's rehearsal shows before the
  capture is reported while the capture goes ahead): fixed by its author as a close-out
  fix (rule 15) with the run made again, or, where it is a toolkit's limit,
  excepted with its reason and source and the exception reviewed, the
  maintainer told in either case; an element pushed out of view whose
  placement is the maintainer's design choice stays a question. A stop ends when the maintainer has
  answered where the rule asks; otherwise the work continues once the
  maintainer has been told. A quoted text that no longer stands is found in its
  present form by the orchestrator: where the edit's meaning is unchanged, the
  edit is made at the found text and the running record names it; where it is
  not, the maintainer is asked.
- **A task that needs more than its parts name** (a model field, another crate,
  a decision the rationale did not take) stops, and the maintainer is asked. So
  does an upstream move during the work (a new gpui-component, egui or iced
  release, a new advisory): it is reported with what it costs before anything is
  changed for it.
- **A fix after M** that CI, the full check, the rehearsal or the asset run
  demands (a must-fix of R3h to R5 is §3's, by the task's author) is made by
  the failing change's author, continued, or where that agent cannot be
  continued, by a new agent of the author's type (`implement` for code,
  `worker` for texts); it is reviewed as in §3 and passes the done-criterion
  of the unit whose files it touches. A fix the close-out needs follows rule 15 (here: its first step is K1, its
  push step K3, its full check beside CI K4, the asset commit K7 and the final
  full check K8; K2's uncommitted files are `docs/COMPATIBILITY.toml` and the
  connector READMEs; the unstaged exceptions are named in the running record).
  The
  maintainer is told what stopped it, what was changed, and whether
  another capture window is needed.
- **The capture window** is asked for once, when K1 to K5 hold.

## 7. The maintainer's part

**With the approval of this plan, before any task:**

1. **B6: the version and the place** (open point 1). `0.7.N`, and which release
   comes immediately before this one in the order; the branch starts from the
   gated head of the KDE steps 4 and 5, the release before it in the KDE lane
   (S3 as amended), and is merged with the tag of the release immediately
   before it in the order, `<prev-tag>` (the icons' under the proposal) (the
   branch's start is S2's and the merge, task MG, S2's amendment, both
   settled, item 8); task A
   reads P2's tree (v0.7.1 and the KDE steps 2 to 5), and MG what the merge
   brought. `<prev-tag>` and the numbers depend on choices still open (rule
   18; the header's open-choice clause gives each case; KDE step 1's place
   is settled, D10 (a), 2026-11-15): D10's order for its items 2 and 3, and
   whether the icons' W7 step is cut as a release of its own before this one
   (then `<prev-tag>` is W7's and the numbers one up again; the lane
   predecessor stays the steps 4–5 release).
   Under the other answers:
   - **(b), a minor version**: the plan holds with that number in place of
     `0.7.N` (*Under the other B6 options*), but the maintainer names both the
     version and the branch: the next minor would be 0.8.0, the name of the
     parked rebuild, whose branch `v0.8.0-rc1` is on `origin`, and A.8's macOS
     cells and E.4's entries name "the 0.8.0 facts" for that work. V then also
     moves every README's `native-theme*` requirement to the new `major.minor`,
     as `scripts/check_readme_versions.py` requires.
   - **(c), `#[non_exhaustive]` model structs first**: a release specified
     separately, not written; this plan waits for it, U6 drops E.1's *Breaking
     Changes* entry, and the plan is amended and checked again before P0, since
     a `#[non_exhaustive]` `InputTheme` breaks A.5's full struct literal in
     `tests/proptest_roundtrip.rs` and U3's item (5).
   - **Into v0.7.1**: the work runs on `v0.7.1-rc2` after the GPUI Kit 0.9.1 and
     the KDE step 1 work, with *Under the other B6 options*' texts; V falls, the
     close-out is the K5 of `docs/todo_v0.7.1_kde-open-items-step1-plan.md`,
     the close-out of both v0.7.1 plans, which then serves this work too, and
     this plan is amended and checked again before P0.
2. **The `unsafe` call** (open point 2): approve the one `GetCaretBlinkTime`
   call in `windows.rs` (C.3), or leave Windows unread in this release (C.5).
   The answer selects U4's scope, R4w, U6's texts, V's dated sentence and K3's
   Windows reading.
3. **The build folders** (settled: the maintainer approved the KDE build
   lane's extra build folder on 2026-11-15). The caret reuses `kde-b` and its
   build folder from the steps 2–3 release (§1, the batch's worktree rule);
   only where `kde-b` does not exist does P2 make it, and its build folder
   (tens of GB, under `/run/media/tibi/work/cargo-build/`) is named then.
   This release deletes no worktree or build folder: they stay for the KDE
   lane's later releases, and their deletion is the lane's last release's,
   the departure from the rule that "a separate build folder is deleted by
   the agent that made it before its task ends" which that approval covers.
   U5 runs in the core lane after U4; no further build folder is made.
4. **The pull request** (settled: the maintainer approved opening a draft
   pull request at a branch's first push on 2026-11-15, rule 9): the
   orchestrator opens a draft pull request from `v0.7.N-rc1` to `main` at
   K3's push, since CI runs only on pull requests and on `main`.
5. **Keep the branch `facts-ios-m1b3`** until U2 is committed: it is the only ref
   that holds `72fcf6a7` (Part 0), and no remote holds its blobs `3d1d5b2f` and
   `ed5470ef` (P2). Or, the maintainer's choice, push it now with an explicit
   refspec (`git push origin facts-ios-m1b3:facts-ios-m1b3`), so that a remote
   holds the run whatever happens to the local branch.
6. **What this plan adds to the specification:** task A; L before the close-out, with LC after M; X; U6 writing E.1 to E.4 in the
   docs lane beside units 4 and 5, where the specification's order puts unit 6
   after them (*Order and gates*), and E.3's ROADMAP section with them (E.3 says
   "written at the release's close-out"; its text needs only the version, known
   from item 1); U2's `git restore` in place of Part 0.1's `git show … >`
   form; P2's case in which U2 runs first; V's mechanics and the dated
   sentence in it; K5's rehearsal; K7's push; the archive in K7's commit. (The
   gates' cadence, the branch's start, the lanes, the close-out's order and
   the line citations are the batch's S1 to S5, S3's amendment and S6, and
   the merge, the working rules and their re-readings were approved too,
   all settled on 2026-11-15 (item 8); what is put for approval is item 8's
   remaining list.)
7. **At the approval**, the orchestrator copies the plan's three documents (the
   rationale, the specification and this plan, as approved) to
   `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/caret-blink-docs/`,
   a new folder: the record of what was approved, against which P1 compares
   each document still untracked, and its fallback should the untracked
   copies in `.claude/worktrees/v071/docs/` be lost before P0 runs. A
   document an earlier commit has added by then (normally v0.7.1's commit
   (0)) is not compared (rule 26): P1 records that commit, and P0 only sets a
   status line that commit left.
8. **Settled, and still put for approval.** Settled by the maintainer on
   2026-11-15: S1 to S5, S3's amendment and S6; S2's amendment, the merge
   (`git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>`,
   rule 17's command, once, task MG, in place of the rebase); the batch's
   working rules, rules 1 to 26 of the batch-speed rationale's Appendix (13,
   23, 21 (2) and 21 (1) item 7 superseded), which this plan cites
   throughout; the three re-readings the rules name (rule 20's branch point,
   M here as the gated head; rule 7's branching from `<prev-tag>` where that
   tag exists when P2 runs; rule 9's "green"); the KDE build lane's extra
   build folder (item 3); and opening a draft pull request at a branch's
   first push (item 4). Still put for approval with this plan:
   - the Windows `unsafe` call (item 2);
   - this plan's own additions and departures: item 6's list;
   - the version and the place (item 1), and the branch `facts-ios-m1b3`
     (item 5).

   Under a no to any of them, the plan is amended and checked again before
   P0.

**During the work, the maintainer is told or asked:** a stop of §6; a link that does not
hold (L); the notice of K3's push, the first CI run of C.3's Windows-only test;
a red CI run; "capture now" (K6), and what the new screenshots show.

**After K8:** whether to merge, tag and publish, and the pull request's title
and description, which the orchestrator drafts and shows first.

## 8. Time

Measured at the 0.7.1 release on this machine and at GitHub (the archived plan
of issue #4's KDE fixes, §8): the full `check_release.sh` about 23 minutes; the
asset script 30 to 37 minutes, of which 4 to 6 need the desktop; a CI run 21 to
27 minutes. Not measured: a `--commit` run under the 16-job cap, a `--quick`
run (S1; its first run is timed), the first
build in a new worktree, and every task's duration; the first runs give them
(the first `--commit` is P0's gate or P2's warm-up, §2.1), and the running
record (§9) keeps them. The longest chain is the core lane: P2 (with the
warm-up where there is no unit 0), P0, U2, U3, U4, U5, M, MG (and
the wait for `<prev-tag>`, S2; VF inside it), LC, X,
V, K1 to K8, plus any review fix and the waits for the pull request and the
capture window.

## 9. Record

The running record lives outside the repository, in
`~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.x_caret-blink-plan/record.md`
(a new folder named by the plan's stem, which P1 makes; the steps 6–7
release reads its rule-20 line there). Only the
orchestrator writes it, from the agents' reports; no agent writes it, and no
task writes results into this plan during the work, so no tracked file is
edited mid-work and the clean-tree checks hold. It is copied into this section
only in K7's commit (§2.2); what comes after K7 is reported to the maintainer,
not committed. Its items:

- P1: `<locks>` made, the trial's output `<…>`.
- P2: the worktree (`kde-b` under the proposal) `<switched | made>`, the tag
  of the release that last used it `<…>`, and its build folder `<…>`; the targets and
  the linker `<…>`; Pillow `<version>`; `72fcf6a7` held by a branch
  `<yes | no: U2 ran first>`; the hook's refusal of the trial Write, quoted
  `<…>`.
- The first `--commit` run (P0's gate or P2's warm-up) `<which>`, its duration
  `<…>`, cold or warm `<…>` (§2.1); each later warm-up, its folder
  `<work>/gates/warmup-<n>/`, its reason (new worktree, `Cargo.lock` changed,
  toolchain changed) and its result `<…>`.
- The maintainer's answers (§7): version `<0.7.N>` and branch `<…>`, the
  release before it `<<prev-tag>>`, the `unsafe` call
  `<approved | not approved>`, the pull request `<…>`, `facts-ios-m1b3` `<kept | pushed>`, the date
  `<date>`.
- A: `<all found | the texts not found>`; the counts' bases `<…>`.
- L and LC: `<every link holds | the corrections, and every copy LC changed>`.
- X: the counts recounted, each old and new value `<…>`.
- M, rule 20's line: "the lane's gated head for the next release: `<sha>` (M;
  --quick run `<folder>`; reviews closed)", which the steps 6–7 release's
  P2 reads.
- The gates: for each `--quick` or `--commit` run, its task, its mode, its run
  folder, its result and the warnings its summary repeats `<…>`; the first
  `--quick` run's duration `<…>`; a failure a `--commit` found that an earlier
  `--quick` passed, with the commit that brought it and the fixing commit
  `<…>`.
- MG: the tag, `<lane-pred-head>`, the ancestry check before the merge, the
  HEAD before the merge, `<merge>` and its parents, the conflicts with their
  resolutions (prose counts and count tuples among them) and R-MG's review
  where one was needed, `Cargo.lock`'s resolution where it was conflicted,
  the marker and staged-path checks, the path check, the versions of P2's
  crates before and after, the survived check, the warm-up's run folder and
  result `<…>`.
- VF, the version forms after MG:
  `<none needed | each form re-written, with its commit, review and gate>`.
- P2's crate versions (egui, iced_widget, gpui-component, gpui-base, windows)
  `<…>`.
- K1: `cargo audit`'s result `<…>`. K2: its run folder, result and what it
  stamped `<…>`. K5: the release builds' results, the rehearsal's run folder,
  the `ENTRIES` check and `parity.out`'s result `<…>`.
- K6: its run folder, `STATUS` `<…>`, the log's tail `<…>`; a quoted text found
  in a new form and edited there, if any (§6) `<…>`.
- Reviews: who reviewed each commit, and the must-fixes `<…>`.
- K3: its CI run `<id>` and the Windows-only test's two log lines `<…>`. K4:
  the full check's run folder, failure and warnings `<…>`. (K7's CI run and K8's full check
  come after the last commit and are reported to the maintainer, K8.)
- Durations measured `<…>`. Departures from this plan `<…>`.
