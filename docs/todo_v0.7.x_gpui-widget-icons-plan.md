# 0.7.N: gpui-component's own icons follow the theme's icon set — implementation plan

- Rationale: [`todo_v0.7.1_gpui-widget-icons-rationale.md`](todo_v0.7.1_gpui-widget-icons-rationale.md)
  (decisions W1 to W11). Specification:
  [`todo_v0.7.x_gpui-widget-icons-spec.md`](todo_v0.7.x_gpui-widget-icons-spec.md)
  (Parts A to G and two appendices; checked).
- Written: 2026-11-14, against `v0.7.1-rc2` at `0fa28027`, the specification's
  commit. Status: **not approved**. Nothing below starts before the maintainer
  approves the rationale, the specification and this plan (§10).
- The release: a 0.7.x release of its own (W11 (b);
  `docs/issues/0004_kde-open-items-rationale.md`, D10, *The releases after
  v0.7.1*, item 3). Its number is the
  maintainer's to approve (D0): `0.7.N` below, 0.7.5 in the settled order.
  `<prev-tag>` is the tag of the release immediately before it in that
  order, `v0.7.4`, the KDE steps 4 and 5's, the other lane's.
- Settled on 2026-11-15: KDE step 1 ships in v0.7.1 (D10, *Where step 1
  goes*, (a); (b) not taken), and the release order is D10's proposal
  (approved with S3; rule 18; §10 item 1): 0.7.1, 0.7.2 egui 0.37, 0.7.3 the
  KDE steps 2 and 3, 0.7.4 the KDE steps 4 and 5, 0.7.5 the widget icons,
  0.7.6 the caret's blink, 0.7.7 the KDE steps 6 and 7.
  `<lane-pred-head>` is v0.7.1's gated head, its record the GPUI Kit 0.9.1
  plan's (T0.2), since this lane runs v0.7.1 → the widget icons. Should the
  order change later, every text that says what `<prev-tag>` brings is
  amended then (§10 item 1 lists them).
- Where: branch `v0.7.N-rc1`, alone in the batch's second lane, in its own
  worktree beside the KDE lane (S3 of
  `docs/todo_v0.7.1_batch-speed-rationale.md`, as the maintainer amended it
  on 2026-11-15: the caret's blink moved into the KDE lane). The branch is
  made from `<lane-pred-head>`, the head of the release before this one in
  its own lane (the second lane: v0.7.1 → the widget icons; here v0.7.1's)
  once that gated head, its documents unit, has passed its gate with every
  review closed (batch rules 18, 20), not from its tag. Once `<prev-tag>`
  exists, the tag of the release immediately before
  this one in the maintainer's release order, which may be another lane's, it
  is merged in, once, after this release's last unit before the merge (the
  merge of unit 6) has passed its gate and every review is closed:
  `git -c merge.conflictStyle=diff3 merge --no-ff --no-commit <prev-tag>` (the
  full command: task MG; S2 as amended on 2026-11-15, a merge and not a
  rebase, so that no sha changes; batch rule 17). The lane
  predecessor is v0.7.1 (the GPUI Kit 0.9.1 port, whose D.3 writes the backlog
  entry this work closes). This plan merges no pull request, and tags and
  publishes nothing.
- The batch's working rules are cited as "rule N (batch-speed rationale,
  Appendix)", here once and "rule N" or "batch rule N" below
  (`docs/todo_v0.7.1_batch-speed-rationale.md`, *Appendix — the batch's
  working rules*, rules 1 to 26, the superseded ones not cited;
  authoritative where this plan words a rule differently). They and S2's
  amendment are proposals, approved with this plan (§10 item 7). S6 of the
  same rationale (the maintainer's decision of 2026-11-15) applies
  throughout: a citation this release writes of a tracked file that changes
  names the file only, never a line; existing line citations are left as
  they are, and nothing re-points them. This plan's own `file:line`
  citations of the repository are `0fa28027`'s unless they name another
  commit or version (pinned, S6), as the specification's header says of its
  own. Line numbers a gate reads (the colour claims, the `HOW` texts) are
  the exception: they stay true (unit 4 item 2 keeps the colour claims'
  lines in place; VF re-points a gate-read upstream line a new version
  moved, rule 25).
- Placeholders: `0.7.N`, `<prev-tag>` (above, which MG merges in),
  `<lane-pred-head>` (the head of the release before this one in its own
  lane, which the branch is made from), `<date>`, `<sha …>`, `<scratch-keep>`
  (§8), `<logs>`, the folder `<scratch-keep>/widget-icons-0.7.N/logs/`;
  `<run>`, a long run's own new folder `<logs>/<task>/run-<n>/` (§4); and
  `<log>` and `<STATUS>`, that run's `<run>/log` and `<run>/status`, each
  given by its absolute path; and every count, time
  and result read at a run. Each is filled by the task that reads it, from its
  run, and written by the orchestrator into the running record outside the
  repository, `<scratch-keep>/widget-icons-0.7.N/record.md`, which 3c and 7f
  copy into §12; none is estimated.

This plan says who does what, in which order, and how each task is known to be
done. What each task changes is the specification's; the plan repeats none of
it.

## 1. Shape of the work

Units 0 to 7 are the specification's (G.1), in its order. The whole release
runs in the batch's second lane, beside the KDE lane (S3): the machine's limits
hold for both lanes together (§9), the gates of both are serialized by the
shared `gate.lock` in the `slot.sh` locks folder (rule 2, §2), one asset run
runs at a time, and the files both
lanes edit (`CHANGELOG.md`, `ROADMAP.md`, `docs/todo.md`, the showcases) meet at
MG's merge, by the texts the specification quotes. Inside this release, two
sub-lanes, each in a worktree of its own, so that no agent's half-done files
reach another's tests or commits. The lanes meet by merges the orchestrator makes
(`git merge --no-ff`, which forces a merge commit even where a fast-forward
would do, so each merge is one commit to gate and name; any merge keeps the
merged commits' hashes), each while no agent works in the worktree merged
into. A conflict in either direction (the core branch into the
docs worktree before 6b, the docs branch into the core one after it) is resolved
by the texts the specification quotes, and reported.

| Lane | Worktree and branch | Tasks | Cargo |
|---|---|---|---|
| core | `.claude/worktrees/lane2-a`, the second lane's first worktree (*The worktrees and their build folders* below); the branch made from the lane predecessor's gated head, `<lane-pred-head>`, merged with `<prev-tag>` (MG) | T0.2, T0.3, units 1 to 5, the merge of the docs lane, MG, unit 7 | yes: the only sub-lane that runs it; beside the KDE lane's cargo, within §9's limits |
| docs | `.claude/worktrees/lane2-a-docs`, `v0.7.N-rc1-docs`, made from unit 3's last commit | 6a, 6b | none |

T0.1 runs in `<scratch-keep>` (§8), and T0.4 reads the core branch's commits with
`git show`, so neither needs a worktree. Worktree paths are relative to the main
checkout. A new worktree is made with `git worktree add`
from the named commit (an agent's own isolation worktree starts from `main`,
which is the wrong base).

```
core     T0.1 → T0.2 → U0 → U1 → U2 → U3a → R3 → U3b → verdict → U3c → U3d → U4 → U5 → R5────────→ [merge docs] → MG → X → V → 7b → 7c → 7e → 7f
reviews                          R1   R2                                          R4                                            7d
beside                      T0.4
docs                                                                         6a ·········· 6b ─┘
```

`U0` to `U5` are the specification's units (T0.3 is unit 0, likely empty
because v0.7.1's commit (0) holds the documents; then `U0` falls away and
T0.2 runs the warm-up gate, §4). A name on a lower
line runs beside the core-lane task above it: the reviews `R1`, `R2` and `R4`
(§3) while the next unit works, T0.4 beside unit 1, and 7d (the showcases'
release builds and the parity rehearsal) while 7c's full check and CI run on
7b's push (S4, §6). `R5` is
drawn on the core line after unit 5, and 6b after it, since 6b needs R5's
fixes merged.
3b starts only when R1, R2 and R3 are closed and every fix they asked for is
committed and gated again (§5). The docs worktree is made from 3d's commit,
with absolute paths
(`git worktree add -b v0.7.N-rc1-docs /home/tibi/Rust/native-theme/.claude/worktrees/lane2-a-docs <3d's commit>`,
rule 24); 6a runs
beside units 4 and 5, 6b after the orchestrator has merged the core branch into
it, and the docs branch then goes into `[merge docs]`. Units 1 onward run on
the branch T0.2 made from `<lane-pred-head>` and never wait for `<prev-tag>`
(S2's gain). MG, the merge of `<prev-tag>`, runs once, after unit 5 and its
review R5 are closed and the tag exists: after `[merge docs]` and before V
(the plan's choice; the docs branch is merged as before, untouched by MG), so
that 6b never waits for the tag. It never runs while a unit's edits are
uncommitted, nor between 3b and 3d. Its check that this release's results
survived the merge and its warm-up gate, then X, the recount (rule 17),
follow before V. No push comes before it (S2). Where T0.2 made the branch
from `<prev-tag>` itself, there is no MG and no X, and the branch may be
pushed after any gated unit (rule 17).

Why this order (each from the specification):

- **One code lane.** The units are a chain: unit 2's freedesktop lookup calls
  unit 1's method (B.2); unit 3 measures unit 2's lookup (C.1); units 4 to 7
  wait for the stop's verdict (G.1); unit 5 needs unit 4's handle (E.2). No two
  code units can run side by side, and a second code worktree would build the
  gpui stack a second time for nothing.
- **The docs lane beside units 4 and 5.** Part F's files (`CN/README.md`,
  `CHANGELOG.md`, `ROADMAP.md`, `docs/todo.md`, `docs/archive/todo_closed.md`)
  are Part F's alone (S6: no unit re-points a citation in them). Part F's texts depend
  on the stop's verdict (D.13 in the README; `Check`'s entry), and F.5 items 2
  and 4 cite the evidence of unit 3; so 6a starts from unit 3's last commit.
  6b waits for units 4 and 5 and their reviews: F.1 item 2's README block
  names unit 4's `WidgetIcons`,
  F.5 item 1 lands in the commit that moves item 8's entry out (F.5), item 8's
  closing line names units 4's and 5's commits, and F.4's count is taken after
  F.5 (G.1, unit 6).
- **T0.4 beside unit 1.** It reads only, and every quoted text it checks
  belongs to a later unit; unit 2 starts only on its report "all found".
- **Unit 7 last**: the asset run follows the last change of the release (G.4,
  open point 2).

**The worktrees and their build folders** (the batch's worktree rule, S2, S3).
Cargo keeps one build folder per worktree path. The core sub-lane works in the
second lane's worktree (S3), never in the KDE lane's. The KDE lane (v0.7.1, then
the KDE steps 2 and 3, the steps 4 and 5, the caret's blink and the steps 6
and 7) alternates between `.claude/worktrees/v071`, v0.7.1's, and
`.claude/worktrees/kde-b` (steps 2 and 3 in `kde-b`, steps 4 and 5 in `v071`,
the caret in `kde-b`, steps 6 and 7 in `v071`; S3 as amended); the second lane
holds this release alone, in `.claude/worktrees/lane2-a` (`lane2-b` is not
used). So this release is the second lane's last, and its close-out removes
`lane2-a`'s worktree and build folder (§7). This release takes
`lane2-a`, made with `git worktree add` where it does not exist yet, or, where it
exists, switched to `v0.7.N-rc1`, which is done only where
`git status --porcelain --untracked-files=no` in it prints nothing and every
untracked file is a planning document under `docs/`: the documentation
checkers read fixed files, the archive gate reads only `docs/archive/*.md`, and
every agent stages its own paths by name, so such files reach no gate and no
commit. A new worktree builds the workspace once from nothing (tens of GB, one
long first gate), and its build folder is named to the maintainer (§10). The
orchestrator says whether it made or switched `lane2-a`. The docs worktree
runs no cargo and has no build folder.

## 2. Tasks

**Who runs what** (rules 1 and 2). An agent
runs its task's targeted done-criterion commands in the foreground (the named
tests, `cargo fmt --all --check`, the checkers, a `-p` clippy or build), each
expected to finish well within the Bash tool's 10-minute limit; it stages its
own paths by name and reports, with the commit message it proposes. It does
not run `--quick`, `--commit`, the full check or the asset run, and it does
not commit.
The orchestrator then runs the task's gate on the staged change as a long run
(§4) and, when it is green, commits exactly the gated tree with the author's
message. A red gate goes back to the author (continued) with the log's failing
part; the author fixes and stages again, and the orchestrator runs the gate
again. A task that changes files ends in one such commit on its lane's branch,
unless it says otherwise, with no attribution line; `git commit -a` and
`git add -A` are not used. The subject follows the repository's style
(`feat(core): …`, `docs(todo): …`) and the message says what changed and why.

"**`--commit` passes**" means: the orchestrator's long run of
`scripts/check_release.sh --commit` on the staged change, from the lane's
worktree, started as every gate run is (`--quick`, `--commit`, the full
run; rule 2):
`flock <scratch-keep>/slot-locks/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh …`,
so that no two gates of the two lanes run at once, exits 0 (its `STATUS` file holds `0`), its
summary counts no failure, and the orchestrator names every warning the
summary repeats in the record; `git status --porcelain --untracked-files=no`
prints the same before and after it (§8). "**`--quick` passes**" means the
same of `scripts/check_release.sh --quick`, the light gate S1 ports to 0.7.x
in v0.7.1's first unit (`docs/todo_v0.7.1_batch-speed-rationale.md`, S1;
what it runs is `docs/todo_v0.7.1_gate-speed-spec.md`'s, cited by name, that
specification being written as this plan is: at least formatting, the panic
scan, clippy with `-D warnings` and the strict panic lints over the workspace,
the tests and doctests of the crates a unit changes and of those that depend
on them, and the documentation checks). It is a long run until its time is
measured; the first is timed into the record. Every code task's done-criterion
also holds §8's rule on formatting and the staged change
(`cargo fmt --all --check` first).

**The cadence** (S1): during a unit, the agent runs only the touched tests (its
targeted commands); at a unit's end, `--quick`, by the orchestrator, as a long run
until its time is measured (the first timed into the record); `--commit` every
third code unit of the release, counted from the last `--commit`, on the
release's last code unit before its documents and version units (unit 5,
rule 6), after a lock or toolchain change (the warm-up gates, §4, MG's
included, and V, whose lock lists the new versions), and wherever the plan
names a unit structural (none here, below); a documents-only unit
runs the five documentation checkers and `--quick`; the full run where the plan
has it (the close-out, 7c and 7f), unchanged. A unit is a code unit when its
change reaches a crate under `--quick`'s scope rule
(`docs/todo_v0.7.1_gate-speed-spec.md`, A.3: its own package's files, files a
package includes, the manifests, the lock and the Cargo configuration, and any
file outside the packages not matching `QUICK_UNREAD` (`docs/*.toml`,
`scripts/`), which re-runs every quick-keyed step: 31, the two `+1.88.0` MSRV
checks among them, keyed under the gate-speed specification's O3, which the
maintainer approved on 2026-11-15, A.2). A documents unit stays one when its
change is only comments,
rustdoc prose, READMEs and Markdown documents, even where a crate includes
them: it runs `--quick`, whose scope (A.3) then reaches those crates, so their
doctests run, and it is not counted toward "every third" or "the release's
last code unit" (rule 11).

**Structural** (rule 24) means a unit whose change can reach a crate
`--quick` would not run: one that changes a manifest, `Cargo.lock`, a build
script, the toolchain or the gate scripts. No unit of this release does, V
aside (a warm-up gate by its lock change): unit 4's new module, public API and
shared state, and the merge of unit 6's first compile of F.1's README block
are no reason on their own, since `--quick` runs every step of `--commit` for
the crates a change reaches, their dependents included (the gate-speed
specification, decision 1 (c), A.4). So no unit is named structural. The
merge of unit 6 is a documents unit and takes the cadence's gate, the five
checkers and `--quick` (rule 6): this release is the second lane's last, so
it has no gated head (rule 20). V runs `--commit` for its lock change (rule
6); a documents fix between unit 5 and V takes `--quick`, except where a
warm-up gate gives it `--commit` (rule 6). So, with the branch's first
warm-up gate at T0.2 or T0.3:

| Task | Gate |
|---|---|
| unit 1, unit 2 | `--quick` (the first and second code units since the warm-up) |
| 3a | `--commit` (the third) |
| 3c (documents only) | the five documentation checkers and `--quick` |
| 3d, unit 4 | `--quick` (the first and second since 3a's `--commit`) |
| unit 5 | `--commit` (the third, and the last code unit before the documents and version units, rule 6) |
| 6a, 6b (documents only, no cargo in the docs worktree) | the five documentation checkers; `--quick` at the merge |
| the merge of unit 6 | the five documentation checkers and `--quick` (rule 6; no gated head, rule 20: the lane's last release) |
| MG | the warm-up gate, `--commit` (its item 5), last; VF's commits, the path check's re-points, item 7's fixes and an R-MG must-fix made before it are covered by it (rule 21 (5)) |
| MG's later commits, if any (MG, *Commits*) | after the warm-up gate, each its own gate: a warm-up fix: its unit's done-criterion, gated by the warm-up run again (rule 12; the record names every later warm-up run); a re-measured tolerance: `--commit` |
| X, after MG (§2, *X*; none with no MG) | the five documentation checkers and `--quick` (a documents unit) |
| V | `--commit`, the warm-up gate of its lock change (rule 6) |

The count of three restarts at every `--commit` (rule 11); a `--quick`-gated
fix does not count toward the three. With MG after unit 5 it restarts for
nothing but MG's own fixes and V. Beside the gate in the table, the gpui
units, 2, 3a, 3d, 4 and 5, run both GPUI Fast legs (rule 11; §4), whichever
of `--quick` and `--commit` they take.

A fix commit (a review's, CI's) takes the gate its unit had. A commit between
two `--commit` runs is proven by `--quick` only (the cost the maintainer
accepted on 2026-10-15, S1); a failure the next `--commit` finds is fixed by a
new commit, never by rewriting a pushed one, and the record names the commit
that brought it. Where a later `--quick` run shows that it covers less than
this plan assumes, the orchestrator says so and adds `--commit` runs rather
than fewer.

**The colour claims' cited lines** (S6's fourth gate):
`every_colour_claim_is_read_at_the_line_it_cites` reads each colour claim's
cited line, 243 of them in the gpui connector's own files (S6, counted at
`0fa28027`). Every unit that edits `connectors/native-theme-gpui/src/` or its
examples (units 2, 3a, 3d, 4 and 5) keeps the line count above each cited
line, or re-points those claims in the same commit; that test, which its
gate runs, shows which. The specification finds `widgets/checkbox.rs` the
only cited file edited here, in place with the same line count (G.2, D.11;
unit 4 item 2).

**The staged tree is the gated tree.** The gate runs on the working tree but
the commit takes the index, and `--untracked-files=no` hides a new file an
agent forgot to stage, which cargo still compiles where a `mod` names it; so a
commit could lack a file the gate saw, and the first push would go red.
Before each gate, and again before the commit, the orchestrator requires all
three:

```sh
git diff --quiet
git status --porcelain --untracked-files=all
git diff --cached --name-status
```

the first exits 0 (nothing changed that is not staged); the second shows no
`??` line but the untracked planning documents T0.2 recorded, named one by
one; the third lists exactly the task's new and changed files. Each task's
list is the files its specification parts own (the specification's *How to
read it* table), with its new files named: unit
4's `connectors/native-theme-gpui/src/icons/widget.rs` (D.1); 3c's
`docs/evidence/widget-icons/<date>/listing.md` and `README.md` (C.6); T0.3's
three documents (those of them unit 0 adds or sets a status line of) and the
path check's re-points, each named in the record, where unit 0 exists; and
7f's new asset files, if the asset
run adds any, besides its moves into `docs/archive/`, which show as renames.
MG's merge commit adds files too: every file `<prev-tag>` brings that
`<merge>^1` lacks, and v0.7.1's archive moves, which show as renames. Its
list is not a task's: before its `git commit`, every path of
`git diff --cached --name-only --no-renames` (so that both paths of a rename
are listed) is in
`git diff --name-only --no-renames <lane-pred-head> <prev-tag>` or is a
regenerated file the record names (MG item 1; rule 21 (1), item 11); a path
that is neither stops MG, and the record names it, since the merge commit
may bring only what the tag brings and the regenerations, and anything else
is a file nobody reviewed; after MG, the record's list
of untracked planning documents drops those MG item 1 removed (equal under
`cmp`). No other task adds a file: every other file the parts edit exists
today (checked on 2026-11-15 at `0fa28027`). The two docs merges (the core
branch into the docs branch before 6b, and `[merge docs]`, unit 6's merge)
commit themselves, so for them only
`git status --porcelain --untracked-files=all` runs, before the gate. A file
outside the list, or a list's file missing, goes back to the author before
the gate. The exceptions are rule 15's (and rule 11's): the outputs a
close-out step keeps across a fix stay unstaged across that fix's commit as
rule 15 says, and the record names each, so `git diff --quiet` does not hold
there and is replaced by a check that only those paths differ.

Every command below runs in the lane's worktree, entered first with
`cd <worktree> &&` (§8), and every cargo command through the wrappers (T0.1);
where a block leaves them out, they stand in front of it.
**`$NT_FEATURES`**: the orchestrator reads the `NT_FEATURES=` line of
`scripts/check_release.sh` on the branch, found by its text (`:420` at
`0fa28027`), and writes the list literally into every brief that uses it, never
the variable (below it stands as `<NT_FEATURES>`); at `0fa28027` it is
`kde,portal,system-icons,material-icons,lucide-icons,watch,svg-rasterize,system-fonts`.
A done-criterion that runs a named test requires that test's `… ok` line and
the `test result:` count in the report, so that a filter that matches nothing
cannot pass. The inputs every brief gives are in §8; "the header" is the
specification from its top to *Open points for the maintainer*, the decisions
and corrections included.

### T0.1 — The build-slot tools

- **By:** orchestrator. No commit: the tools live outside the repository.
- **What:** `mkdir -p <scratch-keep>/slot-locks/`, the one durable lock folder
  every plan's tools share; the existing `slot.sh` and `lowprio.sh` at the top
  of `<scratch-keep>` (§8) are copied into `<scratch-keep>/widget-icons-0.7.N/tools/`
  (`<tools>` below), and the orchestrator reads both before it changes them.
  `slot.sh <command…>` holds one of the machine's two build slots (an
  exclusive `flock` on `slot1.lock` or `slot2.lock`) while the command runs and
  exits with its status; its copy takes its lock folder (`D`) as
  `<scratch-keep>/slot-locks/`, in place of the old session scratch folder the
  existing script names, so every lane, agent, plan and later session shares
  the same two slots. `lowprio.sh <command…>` keeps its `renice -n 19 -p $$` (an
  absolute nice of 19; a relative `nice -n 19` would give 15 under Konsole's −4)
  and its `ionice -c 3`, and its copy also exports `CARGO_BUILD_JOBS=16` and
  `RUST_TEST_THREADS=16`. Every cargo command, and every script that runs
  cargo, is run as `<tools>/slot.sh <tools>/lowprio.sh <command>`. The
  orchestrator lifts the thread cap only on the maintainer's word that they are
  not using the machine.
- **Done when:** `<tools>/slot.sh <tools>/lowprio.sh true` exits 0;
  `<tools>/slot.sh <tools>/lowprio.sh sh -c 'echo "$CARGO_BUILD_JOBS $RUST_TEST_THREADS"; ps -o ni= -p $$'`
  prints `16 16` and `19`; `slot1.lock` exists in `<scratch-keep>/slot-locks/`
  (the copy's `slot.sh` creates it there on its first run; the form of the
  caret-blink plan's P1).

### T0.2 — The branch and the worktree

- **By:** orchestrator. No commit, except the path check's re-point on the
  no-MG path (below).
- **When:** as soon as the lane predecessor (the release before this one in
  its own lane, v0.7.1) has its gated head, its documents unit, the last
  unit before its merge, past its gate (`--quick`), with every review closed,
  which v0.7.1's record shows in its line "the lane's gated head for the next
  release" (rules 18, 20); not waiting for its close-out, merge or tag (S2).
- **What:** the branch is made from `<lane-pred-head>`, the commit that line
  names (the documents unit's commit, a fix after a red gate included;
  written into the record), in the reused worktree with
  `git switch -c v0.7.N-rc1 <base>` or in a new one with `git worktree add` run with absolute
  paths (§1, rule 24). A worktree is reused only once the release that last
  used it has its tag, checked after `git fetch origin tag <tag>`; otherwise
  the release waits for it (rule 18). Where `<prev-tag>` already exists, the
  branch is made from that tag instead (rule 7; fetched first if it is not
  local, `git fetch origin tag <prev-tag>`), and MG falls away; before the branch is
  made, MG item 1's tag check runs (rule 21 (1), "the same check runs at P2
  where the branch is made from `<prev-tag>`"): the first field of
  `git ls-remote origin 'refs/tags/<prev-tag>'` is compared with
  `git rev-parse <prev-tag>`, and no line printed, or a different sha, is a
  stop. A step outcome or
  status of v0.7.1's that this release reads is read from v0.7.1's commit
  messages or record, never from a status line its close-out writes (rule 18).
  The commands that run before a new worktree exists name the main
  checkout with `git -C` and are exempt from §2's `cd <worktree> &&` rule
  (`<base>` is `<lane-pred-head>`, or `<prev-tag>` on the no-MG path; the
  first three commands run only on the no-MG path, and the first of them
  only where `git -C /home/tibi/Rust/native-theme rev-parse -q --verify <prev-tag>`
  fails):

  ```sh
  git -C /home/tibi/Rust/native-theme fetch origin tag <prev-tag>
  git -C /home/tibi/Rust/native-theme ls-remote origin 'refs/tags/<prev-tag>'
  git -C /home/tibi/Rust/native-theme rev-parse <prev-tag>
  git -C /home/tibi/Rust/native-theme worktree add -b v0.7.N-rc1 /home/tibi/Rust/native-theme/.claude/worktrees/lane2-a <base>
  ```

  `git switch` refuses where an untracked file of the worktree is a file the
  base commit holds (a planning document of an earlier release, now in its
  tree): the orchestrator compares the two with `cmp`, removes the untracked
  copy where they are equal, and otherwise stops and reports, since the
  untracked copy holds edits the release does not.
- **Done when:** in the core worktree, `git rev-parse --abbrev-ref HEAD` prints
  `v0.7.N-rc1` and `git rev-parse HEAD` equals the base's commit (`<lane-pred-head>`, or
  `git rev-parse <prev-tag>^{commit}`, where the tag check above printed
  one sha equal on both sides, both written into the record); v0.7.1's
  record shows "the lane's
  gated head for the next release: `<sha>` (<unit>; `--quick` run …; reviews
  closed)", and its `<sha>` is `<lane-pred-head>`, that line being v0.7.1's
  gated head (rules 18, 20). That record is the GPUI Kit 0.9.1 plan's running
  record,
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.1_gpui-kit-0.9.1-plan/record.md`
  (that plan's §2). Where the branch was made from `<prev-tag>`,
  `git merge-base --is-ancestor <lane-pred-head> HEAD` exits 0 (rule 18). On
  both paths, T0.2 compares the tree's `Cargo.lock` with
  `git show 0fa28027:Cargo.lock` for the crates MG item 1 lists (its
  sub-item 5; rule 19, which names both paths): a changed version's readings
  are made again before unit 1, or it is a stop as that sub-item says.
  `git status --porcelain --untracked-files=no` prints
  nothing, and every untracked file is a planning document under `docs/` (§1),
  each one named in the record (the list §2's staged-tree check allows);
  the build folder's path is read from
  `cargo metadata --format-version 1 --no-deps` (`build_directory`) in the
  worktree, and the record (§12) holds it in the placeholder form §8 uses,
  `<cargo-build>/<hash>`, never as an absolute path; and the project's hooks fire in the core worktree: a Write of a new
  untracked file `<core worktree>/native-theme/src/hook_trial.rs` holding
  `fn f() { x.unwrap() }` is refused and writes nothing. If it applies, the
  orchestrator removes the file (`rm`) and stops, with no code agent
  dispatched, and tells the maintainer. Where there is no unit 0 (T0.3), T0.2
  ends with the warm-up gate (§4) on the clean tree, timed into the record
  as the branch's first `--commit`; a red one follows rule 12 (it goes, with
  the log's failing part, to the author of the unit whose rule the failing
  test checks, and where that author is no longer at hand, as a v0.7.1
  unit's is here, to a new `implement` agent with the log and that unit's
  part of its specification; one whose cause is unclear is a stop), the
  maintainer is told,
  and unit 1 does not start on it. Where there is neither MG nor
  T0.3, T0.2 also runs the path check (rule 22 (b), as MG's item 6 says),
  before its warm-up gate; a path that moved to `docs/archive/` is re-pointed
  by a `worker` in a documents-only commit made before the warm-up gate and
  covered by it, with no gate of its own (rule 22 (b)).

### T0.3 — Unit 0: the documents

- **By:** orchestrator. **Implements:** G.1 unit 0, decision 23.
- **Likely empty.** v0.7.1's commit (0) adds every approved document of the
  cycle, the `docs/todo_v0.7.x_*` plans among them, and sets their status
  lines (`docs/todo_v0.7.1_gpui-kit-0.9.1-plan.md`, P0 and §7 item 1). Each
  of the three documents is taken on its own (rule 26):
  `cd <core worktree> && git ls-files --error-unmatch <path>` for each. One
  an earlier commit added is not compared with its approval copy (later
  close-outs rewrite its path mentions, rule 22 (a), and set its status
  line): the record names the commit that added it,
  `git log --diff-filter=A --format=%H -- <path>`, and nothing is copied.
  Where all three were added earlier and their status lines set, there is
  no unit 0: nothing is committed here, the record says so with those
  commits, and the warm-up gate (§4) runs at T0.2's end instead. T0.4 then
  reads the branch's base commit.
- **What, where it is not empty:** each document still untracked (its
  `git ls-files --error-unmatch` fails) into the core worktree's `docs/`,
  from the copies made at the approval (§10, item 5) in
  `<scratch-keep>/widget-icons-0.7.N/docs/`, compared first with
  `.claude/worktrees/v071/docs/`'s with `cmp`: equal, or v071's gone, the
  approval copy is used; different, the orchestrator stops and reports what
  changed since the approval. For a document an earlier commit added, unit
  0 only sets a status line that commit left unset. The status lines unit 0
  writes are set to approved, with the date of the approval.
  **Paths:** no later-links list exists (a close-out rewrites the path
  mentions itself, rule 22 (a)). Where T0.2 made the branch
  from `<prev-tag>` (no MG), T0.3, or T0.2's end where there is no unit 0,
  runs rule 22 (b)'s path check on that tree (as MG's item 6 describes it;
  where there is neither MG nor T0.3, T0.2 runs it at its end);
  otherwise only MG does. A re-point T0.3's path check finds goes into unit
  0's own commit (rule 22 (b)).
- **Done when:** the five documentation checkers (§4) pass;
  `git show --stat HEAD` lists the three documents (those of them unit 0
  adds or sets a status line of) and the path check's re-points, each named
  in the record, and nothing else. Then the
  orchestrator runs the warm-up gate (§4) on that commit, the first
  `--commit` of the branch, as a long run, before any agent that runs cargo,
  and writes its time into the record: it says whether `--commit` fits in 10
  minutes at all on this machine and the new branch, which is what §4's
  long-run form assumes it may not (§11). A red first `--commit` follows rule
  12, as at T0.2's end, the maintainer is told, and unit 1 does not start on
  it.
- **Review:** none (checked as documents). **Commit:**
  `docs: the widget icons' rationale, specification and plan, approved <date>`.

### T0.4 — The texts the specification quotes still stand

- **By:** `worker`, no cargo, beside unit 1. It reads unit 0's commit, or,
  where there is no unit 0, the commit T0.2 made the branch from
  (`<lane-pred-head>`, or `<prev-tag>`'s commit), with
  `git show <sha>:<path>`, never a working tree.
- **What:** (a) the earlier releases' parts this work builds on landed: the
  0.9.1 specification's B.3, B.13, B.15, D.3, D.4 and E (the specification's
  header). What `<prev-tag>` brings (v0.7.1's close-out, v0.7.2 with egui
  0.37, the KDE steps 2 and 3, the KDE steps 4 and 5, B.9 and B.10 among
  them, E.14) is not in this tree: the units start on a tree without it (the
  lane rule, S2), and it comes in at MG, after which E.14's claims are
  checked as claims that this release's results survived the merge (MG item
  7). Where T0.2 made the branch from `<prev-tag>` itself (no MG), T0.4 checks
  only that B.9's and B.10's texts are present on that tree; the result
  claims are then 3a's (C.4's 23 names) and unit 5's (Edit drawn through
  `role_chrome_icon` with E.2's judgement) own done-criteria; (b) every present
  text the specification quotes to replace or to place an edit by is found in
  the file it names as often as the edit names it (once, as a rule; E.12's
  texts stand in both reasons by design): B.1, B.4, D.10, D.11, E.1 to E.8,
  E.10, E.12, the placement texts of B.2 and B.3 (after `to_rgba`, before
  `CN/src/icons.rs`'s `#[cfg(test)] mod tests`) and of E.9 (`button`'s doc and
  E.4's loading arms in `SC/info/buttons.rs`), F.1 items 1 to 8, F.4, F.5
  items 1 to 8, C.6's placement (the heading
  `` ## `breeze-offscreen/` `` in `docs/evidence/README.md`), and the function
  names of C.4 item 8. Part A's places are unit 1's own check.
- **Done when:** a table, each quoted text against `file:line` at that commit
  or "not found", is in the report, and the orchestrator writes its result
  into the record (§12). All found: the plan goes on, and unit 2 may start
  (it waits for this report). One not found: §9's stop for a quoted text that
  no longer stands, which the orchestrator settles as §9 says (the text found
  in its present form, and the edit made there where its meaning is
  unchanged, or the maintainer asked) before the unit that edits it starts.
- **Commit:** none.

### Unit 1 — The core's new method (Part A)

- **By:** `implement`, core lane. **Implements:** A.1 to A.4.
- **Done when:**
  1. A.3's tests are written first and seen to fail (they cannot compile before
     A.1 and A.2); the report quotes the failure.
  2. They pass:

     ```sh
     cargo test -p native-theme --features "<NT_FEATURES>" --lib first_accepted_of_name
     cargo test -p native-theme --features "<NT_FEATURES>" --lib load_where
     cargo test -p native-theme --lib load_where_is_none_without_the_freedesktop_loader
     ```

     (the last without features). The report quotes an `… ok` line for each
     of A.3's seven tests: `first_accepted_of_names_goes_on_in_its_order` and
     `first_accepted_of_name_goes_on_in_its_order` from the first command;
     `load_where_goes_on_by_names`,
     `load_where_goes_on_by_a_name_through_the_chain`, `load_where_by_a_role`
     and `load_where_by_a_provider` from the second;
     `load_where_is_none_without_the_freedesktop_loader` from the third; and
     each command's `test result:` line, whose passed count is not 0.
  3. The ignored stem-index test, which `--commit` skips and which compares
     the lookup A.1 rewrites with the unfiltered one, passes:

     ```sh
     cargo test -p native-theme --features "<NT_FEATURES>" --lib -- --ignored freedesktop::tests::indexed_lookup_matches_the_unfiltered_lookup_on_installed_themes --nocapture
     ```

     (`scripts/check_release.sh:697-699`; it reads the installed
     themes through the library, as the full gate does; ~30-50 s by the
     script's comment, `:686-687`). It passes without checking anything on a
     host with none of breeze, breeze-dark and Adwaita
     (`scripts/check_release.sh:688-689`), so the report quotes its `… ok`
     line, its `test result: ok. 1 passed` line, and every `skipped` line it
     prints, or states that none was printed.
  4. The agent stages its change and reports; then `--quick` passes, the
     orchestrator's long run on the staged change (§2, *The cadence*), and the
     orchestrator commits.
- **Review:** R1. **Commit:** `feat(core): FreedesktopLoader::load_where …`.

### Unit 2 — The lookup and the one-colour test (Part B)

- **By:** `implement`, core lane, once T0.4 has reported "all found".
  **Implements:** C.1's check, then B.1 to B.4. It keeps the colour claims'
  cited lines (§2, *The colour claims' cited lines*).
  The implementer's brief and R2's brief both carry §10 item 3's two unit-2
  additions: the `eprintln!` of `M` and `F` in the tolerance test, and the
  resvg version in `ONE_COLOUR_TOLERANCE`'s doc read from `Cargo.lock`
  (item 3).
- **Done when:**
  1. **First, C.1's check**, with B.1's two entries changed in the working tree
     only, outside the runner environment:

     ```sh
     cargo test -p native-theme-gpui --features native,watch --lib all_kde_names_resolve_in_breeze -- --nocapture
     ```

     The test passes when it skips, printing a line that begins
     `Skipping:` (`CN/src/icons.rs:2745`, `:2751`), so the report quotes its
     result line and the `Skipping:` line, or states that none was printed.
     Red: the agent stops with nothing committed (§9). Skipped: the
     orchestrator writes it into the record (§12); condition 4 then falls to
     unit 3 (C.4 item 5).
  2. **Red first**, each seen and quoted: with B.1's entries and without the
     fixture rows, `the_theme_s_own_name_wins_in_each_convention_s_theme` fails
     on `breeze-only` and `both-names`; with the lookup and without B.4,
     `python3 scripts/check_connector_parity.py` names `icons::load_gpui_icon`
     and `icons::mask_drawable`; B.3's and the lookup's tests fail before
     their items exist.
  3. **The tolerance**, by B.3's procedure, each run as

     ```sh
     cargo test -p native-theme-gpui --features native,watch --lib the_one_colour_tolerance_separates_the_bundled_sets_from_two_colours -- --nocapture
     ```

     the test holding one `eprintln!` of `M` and `F` before its
     assertions (a plan addition, §10 item 3, so that `F` is read on a passing
     run too): the first run, with `0`, prints `M`; the constant is written as
     `M` with the run's date
     in its doc, and the resvg version its doc names is native-theme's resvg
     in `Cargo.lock` at the run (the lock holds several resvg versions; B.3
     writes `0.48.1`, §10 item 3); the second run passes. `M`, `F`, that resvg
     version and the deviations `an_icon_with_no_opaque_part_is_one_colour`
     prints go into the report, each quoted from the printed line, and the
     orchestrator writes them into the record (§12); that test runs as

     ```sh
     cargo test -p native-theme-gpui --features native,watch --lib an_icon_with_no_opaque_part_is_one_colour -- --nocapture
     ```

     A stop (B.3; §9):
     `M ≥ F`; that test failing; or the tolerance test's further assertion
     naming a bundled icon
     that `mask_drawable` does not judge `Some(true)` (the bundled sets are one
     colour throughout, W6).
  4. B.3's and the lookup's tests pass:

     ```sh
     cargo test -p native-theme-gpui --features native,watch --lib icons::
     cargo test -p native-theme-gpui --lib --no-default-features
     cargo test -p native-theme-gpui --lib --no-default-features --features lucide-icons without_the_rasteriser_no_svg_is_judged
     ```

     The last runs the test's Lucide half, which needs `lucide-icons`
     without `svg-rasterize` (B.3) and which neither of the other two runs;
     the report quotes its `… ok` line.

  5. The GPUI Fast legs, which `--commit` skips (G.1), pass: this is a gpui
     unit by rule 11's test (§4). They build a second GPUI core and are not
     known to finish within 10 minutes, so they are the orchestrator's long
     runs on the staged change (§4), after the gate and before the commit
     (all use the worktree's one build folder); a red run goes back to the
     author as a red gate does (§2), and the orchestrator commits once all
     have passed. They are part of the unit's gate, so each command below
     starts under the gate's lock, with
     `flock <scratch-keep>/slot-locks/gate.lock <tools>/slot.sh <tools>/lowprio.sh`
     in front of it (rule 2; §4's gate `<wrappers>`). The connector's leg
     (`scripts/check_release.sh:710-716`) and the showcase's in the runner
     environment (`:849-855`; where `--prepare` exits 75, unit 5 item 7
     holds):

     ```sh
     cargo test -p native-theme-gpui --features gpui-fast,native,watch --all-targets
     scripts/run_in_runner_env.sh --prepare
     scripts/run_in_runner_env.sh test -p native-theme-gpui --features gpui-fast,native,watch --example showcase-gpui
     ```

  6. The agent stages its change and reports; then `--quick` passes (§2,
     *The cadence*), the GPUI Fast runs of item 5 pass, and the orchestrator
     commits.
- **After a fix from R2:** the unit's tests (item 4), `--quick` and the GPUI
  Fast runs (item 5) run again.
  Where the fix touches `one_colour_deviation`, `embeds_image` or
  `ONE_COLOUR_TOLERANCE`, 3a's author updates the measuring test's copies in
  the same series of fixes (reviewed under R3); where it touches the
  deviation, `M` is measured again by B.3's procedure (item 3). Items 1 and 2
  (C.1's check before B.1 lands, and the red-first runs) cannot be repeated
  once the unit is committed; their first results stand.
- **Review:** R2. **Commit:**
  `feat(gpui): icons::load_gpui_icon and icons::mask_drawable …`.

### Unit 3 — The measurement, the first stop (Part C)

Four tasks; §5 says how the run is made and read.

- **3a — the tool.** `implement`, core lane. Implements C.2 to C.4, and C.5's
  verdicts where the table decides them (C.4 item 9); it keeps the colour
  claims' cited lines (§2, *The colour claims' cited lines*). Done when:
  1. the test is in `SC/tests.rs` and is listed by
     `cargo test -p native-theme-gpui --example showcase-gpui -- --ignored --list`;
  2. run in the core worktree, first with `NT_WIDGET_ICONS_LISTING` unset,
     then with it set to a path outside `docs/evidence/widget-icons/` (the
     variable before the wrappers):

     ```sh
     <tools>/slot.sh <tools>/lowprio.sh cargo test -p native-theme-gpui --example showcase-gpui -- --ignored widget_icons_listing --nocapture
     NT_WIDGET_ICONS_LISTING=target/x <tools>/slot.sh <tools>/lowprio.sh cargo test -p native-theme-gpui --example showcase-gpui -- --ignored widget_icons_listing --nocapture
     ```

     The first prints that the variable is unset and passes, and the report
     quotes that line; the second refuses and writes nothing, and the report
     quotes the refusal;
  3. it holds three named copies of the connector's private items (B.3),
     `one_colour_deviation`, `ONE_COLOUR_TOLERANCE`'s value (`T_copy`) and
     `embeds_image` (`embeds_copy`), each named in a comment as a copy of the
     connector's, and asserts for every SVG row that rasterises:
     `mask_drawable == Some(!embeds_copy && d <= T_copy)`, `d` the copied
     deviation (C.4 item 1);
  4. its trial runs, made only after §5's announcement, write into
     `docs/evidence/widget-icons/trial-<n>/` and
     `target/widget-icons-sheets/trial-<n>/`, a new `<n>` for each.
     A trial run reads every installed theme and times five runs of every
     step, so it is not known to finish within 10 minutes: the agent asks for
     it, the orchestrator runs it as a long run (§4) under
     `flock <scratch-keep>/slot-locks/gate.lock` before the wrappers (a
     trial's run is a heavy run, rule 2), its `<run>` a new folder
     `<logs>/3a-trial/run-<n>/`, with
     `NT_WIDGET_ICONS_LISTING=docs/evidence/widget-icons/trial-<n>` before the
     wrappers (the same `<n>`, so each trial writes its own listing, and its
     sheets go to `target/widget-icons-sheets/trial-<n>/`, C.2), and hands the
     log and the outputs back to the agent (continued). Every trial's folders
     are deleted before the agent stages
     (`git status --porcelain -- docs/evidence` prints nothing, and no
     `trial-*` folder is left under `target/widget-icons-sheets/`, which git
     ignores);
  5. the 23 names of C.4 item 8 are confirmed at their call sites on the
     branch, each function and line in the report; where the branch was made
     from `<prev-tag>` (no MG), this includes E.14's claim that the KDE
     steps 4 and 5's Edit tool button adds none of the 23 (T0.4);
  6. the agent stages its change and reports; then `--commit` passes (§2,
     *The cadence*: the third code unit since the warm-up; the test is ignored
     there), the GPUI Fast legs pass as in unit 2, item 5 (a gpui unit by
     rule 11's test: it adds code under `connectors/native-theme-gpui/`),
     and the orchestrator commits.
  Review R3, closed before the run. Commit:
  `test(gpui showcase): the widget icons' measurement, an ignored test …`.
- **3b — the run.** The run itself is the orchestrator's long run (§4); a
  `worker` then writes the README and drafts the verdicts. It starts only
  when R1, R2 and R3 are closed and every fix they asked for is committed and
  its unit's done-criterion holds again; the run is made at the lane's head
  then, which is the commit C.6's `<sha>` names (§5). It starts from a clean
  tree (`git status --porcelain --untracked-files=no` prints nothing). The run
  is made in the core worktree with `run_in_background`, its log and `STATUS`
  in its own `<run>` = `<logs>/3b/run-<n>/` (§4), and holds the gate lock (a
  measurement's run is a heavy run, rule 2) and both build slots, the
  variable before the wrappers (the outer `slot.sh` takes one slot, the inner
  one the other):

  ```sh
  cd <core worktree> && NT_WIDGET_ICONS_LISTING=docs/evidence/widget-icons/<date> flock <scratch-keep>/slot-locks/gate.lock <tools>/slot.sh <tools>/slot.sh <tools>/lowprio.sh cargo test -p native-theme-gpui --example showcase-gpui -- --ignored widget_icons_listing --nocapture > <run>/log 2>&1; echo $? > <run>/status
  ```

  (the example's name, `connectors/native-theme-gpui/Cargo.toml:119`; the
  test's, C.2). Just before it, the orchestrator checks once that
  `pgrep -x cargo` (the exact process name, so it cannot match itself) prints
  no PID: no other cargo runs, and none wrapped can start until the run ends.
  Where it prints one, the orchestrator waits, and asks the maintainer if the
  process persists; the record names every cargo process seen. After the exit
  notice, the `worker` (given the log, `STATUS` and the outputs) writes C.4's
  README, but for its table of the packages' versions, which the orchestrator
  writes in 3c (§5). Done when: `STATUS` holds `0`; `listing.md` holds C.4's
  items 1 to 9 and `README.md` everything else C.4 asks of it;
  `git status --porcelain --untracked-files=all -- docs/evidence` shows these
  two files and nothing else;
  `grep -rnE '/tmp/|/home/|/run/media' docs/evidence/widget-icons` finds
  nothing; the worker's report gives the run's result line (from the log) and
  duration, and a draft verdict for each of C.5's seven conditions with the
  rows (and for condition 3 the sheets) that decide it. No commit.
- **The verdict.** Orchestrator, as §5 says. A condition met stops the plan
  (§5, §9); after a stop at condition 7, once the maintainer has answered, 3c
  (with the maintainer's word) and 3d follow in the same way.
- **3c — the record.** Orchestrator, once the maintainer has been sent the
  verdicts and the sheets' paths (§5). Implements C.6. One commit: 3b's two
  files, the README with the table of the packages' versions written from the
  maintainer's answer (§5), `docs/evidence/README.md`'s section, whose `<sha>`
  is the lane's head the run was made at (3b), and this plan's §12, into which
  the orchestrator copies the record's entries up to the run: the run, the
  seven verdicts, the timings' medians and the maintainer's word, where a
  condition stopped the plan.
  Done when: the five documentation checkers pass and `--quick` passes on the
  staged change (a documents-only unit, §2, *The cadence*); the commit holds
  those four files and nothing else; the privacy grep of 3b finds nothing.
  Commit:
  `docs(evidence): the widget icons' measurement of <date> …`.
- **3d — the tool removed.** Orchestrator. Implements G.1 unit 3, its last
  commit; it keeps the colour claims' cited lines (§2, *The colour claims'
  cited lines*). Done when, in the core worktree, with the removal made in the
  working tree and staged,

  ```sh
  git diff <3a>^ -- connectors/native-theme-gpui/examples/showcase-gpui/tests.rs
  git grep -n widget_icons_listing -- connectors
  ```

  print nothing (`<3a>` the commit that added the test; the first compares the
  working tree with the commit before it), and `--quick` passes on the staged
  change (§2, *The cadence*), and the GPUI Fast legs pass as in unit 2, item
  5 (a gpui unit by rule 11's test: it removes code under
  `connectors/native-theme-gpui/`), before the orchestrator commits. The sheets stay
  in `target/widget-icons-sheets/` until §7 deletes them. Review: as §3's
  table says, the checkers, then the orchestrator reads
  the diff (the file returns to a reviewed text). Commit:
  `test(gpui showcase): the widget icons' measurement removed, its run recorded (<sha of 3c>)`.

### Unit 4 — The handle (Part D)

- **By:** `implement`, core lane, after the verdict. **Implements:** D.1 to
  D.12, D.13 only where the record (§12) holds the maintainer's choice of it (condition
  7), F.2's D items. It keeps the colour claims' cited lines (§2, *The
  colour claims' cited lines*; item 2).
- **Done when:**
  1. D.12's tests (and D.13's, where built) are written first and seen to
     fail; they pass:

     ```sh
     cargo test -p native-theme-gpui --features native,watch --lib icons::widget
     cargo test -p native-theme-gpui --features native,watch --lib apply_system_theme_tells_the_installed_handle
     ```
  2. `widgets/checkbox.rs` keeps its line count (`git diff --numstat` shows as
     many lines added as removed for it), so the lines the colour claims pin
     stay; `every_colour_claim_is_read_at_the_line_it_cites` passes.
  3. The GPUI Fast legs pass, as in unit 2, item 5 (a gpui unit by rule 11's
     test): the orchestrator's long runs after the gate and before the
     commit, a red run going back to the author.
  4. The agent stages its change and reports; then `--quick` passes (§2,
     *The cadence*: the second code unit since 3a's `--commit`; `--quick`
     runs every step of `--commit` for the crates the change reaches, the
     connector's build without the feature, its lints and its docs among
     them), the GPUI Fast runs of item 3 pass, and the orchestrator commits.
- **Review:** R4. **Commit:**
  `feat(gpui): icons::WidgetIcons serves gpui-component's widgets the theme's icon set …`.

### Unit 5 — The showcase (Part E)

- **By:** `implement`, core lane. **Implements:** E.1 to E.12, as E.14 places
  them, keeping the colour claims' cited lines (§2, *The colour claims'
  cited lines*); where the record (§12) holds D.13 as built, D.13's E.5 caption for `Loading`
  too. Not verified (D.13 only): whether E.11's tests that
  call `reload_icons` or `select_icon_set` directly need
  `cx.run_until_parked()` before they assert, as D.13 asks of D.12's tests;
  the implementer finds out under the gate and reports.
- **Done when:**
  0. The agent's targeted commands, outside the runner environment (the
     runner-environment legs are the gate's and the orchestrator's):

     ```sh
     cargo test -p native-theme-gpui --features native,watch --example showcase-gpui <filter>
     ```

     with `<filter>` each test name item 1 or 2 names, and once without a
     filter for the whole suite.
  1. With E.2's role-first `chrome_icon` and before E.10's edits, the tests of
     E.10's table fail, each seen and named in the report; with the edits they
     pass.
  2. E.11's tests are written first and seen to fail; they pass.
  3. E.8's grep is run again and its result reported. Where the branch was
     made from `<prev-tag>` (no MG), the report also shows E.14's claim on
     this tree: the Basic page's Edit tool button (B.10) is drawn through
     `role_chrome_icon` with E.2's judgement, and its two elements keep
     B.10's routes (T0.4).
  4. Every other test that E.2 or E.3 turns red is named with its edit in the
     report, by the agent's runs of E.11's and the suite's tests, or from the
     gate's log the orchestrator hands back; the orchestrator writes them into
     the record (§12; E.10's last sentence).
  5. The GPUI Fast legs pass, as in unit 2, item 5 (a gpui unit by rule 11's
     test; the orchestrator's long runs after the gate and before the commit;
     `--commit` skips both, the connector's `--all-targets` building the
     showcase on GPUI Fast and the runner-environment leg,
     `scripts/check_release.sh:849-855`, running its tests there; §10 item
     3). A red run of either goes back to the author as a red gate does (§2).
  6. The agent stages its change and reports; then `--commit` passes (§2,
     *The cadence*: the third code unit since 3a's `--commit` and the last
     before the documents and version units; the showcase's tests
     inside the runner environment and outside it,
     `scripts/check_release.sh:667`), the runs of item 5 pass, and the
     orchestrator commits.
  7. Where `scripts/run_in_runner_env.sh --prepare` exits 75 (the runner's
     packages could be neither fetched nor found; the gate then only warns,
     `scripts/check_release.sh:825-828`), unit 5's runner-environment check
     did not happen: the unit is not done, and it stops for the maintainer.
- **Review:** R5. **Commit:**
  `feat(gpui showcase): gpui-component's widgets draw the chosen icon set …`.

### Unit 6 — Outside the code (Part F), in the docs lane

The docs worktree runs no cargo, so the gate of 6a and 6b is the five
documentation checkers their worker runs (documents-only units, §2, *The
cadence*; the `--quick` such a unit also needs runs at the merge, on the core
worktree). The worker stages and reports with its message, and the
orchestrator commits (§2); the cargo gate of these texts is the merge's
`--quick` below.

- **6a.** `worker`, docs worktree, from unit 3's last commit, beside units 4
  and 5. Implements F.1 items 1 and 3 to 8 (with D.13's sentence where it is
  built), F.3, F.4's section, and F.5 items 3 and 4 and the removal of item 6.
  F.3's entries go under a `## [0.7.N] - <date>` heading with the proposed
  number, above every other `## [` heading, never `## [Unreleased]` (batch
  rule 14); task V fills the date (§6), and at MG a conflict there puts this
  release's section first. Done
  when: the five documentation checkers pass;
  `python3 scripts/check_readme_versions.py` passes; every in-page link F.1 adds names a
  heading the README has (`#the-widgets-own-icons`,
  `#what-the-widget-icons-leave-as-they-are`); each new changelog entry is one
  physical line; 6a writes no Rust block (the README's one Rust block is F.1
  item 2's, verbatim, in 6b, compiled by the merge gate). Commit:
  `docs: the widget icons in the gpui connector's README, the changelog, the roadmap and the backlog`.
- **6b.** `worker`, docs worktree, after the orchestrator has merged
  `v0.7.N-rc1` into it (units 4 and 5 and every review fix of theirs). Implements
  F.1 item 2 (its block names unit 4's `WidgetIcons`, so it is written on a tree
  that has it), F.5 items 1 and 2 (item 1 lands in the commit that moves item
  8's entry out, F.5; item 2 is placed after item 1), items 5, 6 (its dated
  line) and 7, item 8 and F.4's count. The dated
  lines name the last commit of the unit that made what they state: items 5
  and 6 unit 4's, item 7 unit 5's; item 8's closing line names both as F.5
  says. Done when: the five checkers and
  `python3 scripts/check_readme_versions.py` pass; the number F.4 writes equals
  `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md` at the commit; the commit's
  diff shows item 8's section removed from `docs/todo.md` and the same lines
  in `docs/archive/todo_closed.md`, but for the ticked box and the closing
  line. Commit:
  `docs: the gpui connector's quick start with the widget icons, and their backlog: … closed by <sha of unit 4> and <sha of unit 5> …`.
- **The merge.** Orchestrator: `v0.7.N-rc1-docs` into `v0.7.N-rc1`, conflicts by
  §1's rule. Done when: the five documentation checkers and `--quick` pass
  on the merge commit (the gate of a merge runs on the merge commit, not on a
  staged change; a documents unit's gate, rule 6), `--quick` compiling F.1's
  README block as a doctest, since the README is a file the connector
  includes (the gate of D.11; gate-speed specification, A.3). A failure
  there is fixed in the block by an `implement` agent on the core branch,
  reviewed by the orchestrator. The merge is the last unit before MG, after
  every review is closed; this release is the second lane's last, so it has
  no gated head and writes no gated-head line (rule 20). The branch is pushed
  only after MG (§4).

### MG — The merge of `<prev-tag>` (S2 as amended; batch rule 17)

- **By:** orchestrator; R-MG, a `review` agent, for any conflict resolution
  in a code file; a `worker` (no cargo) for the path check and the
  survived-the-merge checks, and their documents fixes.
- **When:** once, after unit 5 and its review R5 are closed and every fix
  committed, after the merge of unit 6 (the plan's choice, so that 6b never
  waits for the tag; §10 item 3 names it as a departure), once `<prev-tag>`
  exists, and before V. Never while a unit's edits are
  uncommitted, and never between 3b and 3d (both lie before it). Until the
  tag exists, V, unit 7 and every push wait. Where T0.2 made the branch from
  `<prev-tag>` itself, there is no MG (T0.3 and T0.4 make its checks).
- **What it brings:** `<prev-tag>` holds, besides v0.7.1's commits up to
  `<lane-pred-head>`: v0.7.1's close-out (version, assets, archiving), egui
  0.37's release (v0.7.2, outside the two lanes: S3's paragraph on it), the
  KDE steps 2 and 3,
  and the KDE steps 4 and 5 (E.14).
- **What**, in rule 21's order, (1), (3), (6), (7), (4), then (5) ((2) is
  superseded by S6); the items keep the rule's numbers:
  1. **The merge**, in the core worktree, on a clean tree
     (`git status --porcelain --untracked-files=no` prints nothing), in rule
     21 (1)'s order:
     1. The tag: fetched only where `git rev-parse -q --verify <prev-tag>`
        fails, with the command T0.2 uses (`git fetch origin tag <prev-tag>`),
        and in any case the first field of
        `git ls-remote origin 'refs/tags/<prev-tag>'` compared with
        `git rev-parse <prev-tag>` (the tag ref's own object on both sides,
        equal for a lightweight tag, as this project's release tags from
        v0.5.9 on are, and for an annotated one, as some older ones are,
        `git for-each-ref refs/tags`, read 2026-11-15; the `^{}` form prints
        nothing for a lightweight tag; tags are shared by every worktree, and
        a release tag has been moved before): no line printed, or a
        different sha, is a stop.
     2. `git merge-base --is-ancestor <lane-pred-head> <prev-tag>` exits 0
        (the tag holds the lane predecessor's release, rule 18), a stop
        otherwise.
     3. The untracked-overwrite check: every path both
        `git ls-files --others --exclude-standard` and
        `git diff --name-only --no-renames --diff-filter=A <lane-pred-head> <prev-tag>`
        list is compared with `git show <prev-tag>:<path> | cmp - <path>`:
        removed where equal, a stop otherwise; the record's list of
        untracked planning documents drops each one removed (§2).
     4. The merge, with the HEAD before it recorded; it never commits on its
        own, and diff3 gives every conflicted hunk its base section (rule
        17):

        ```sh
        cd <core worktree> && git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>
        ```

     5. `Cargo.lock` and the version comparison, as rule 21 (1) item 5 says
        (a conflicted manifest, `Cargo.toml`, is resolved first, by
        sub-item 6's rules, before this sub-item's cargo commands; a
        conflicted lock takes `<prev-tag>`'s side and this release's lock
        changes are re-applied with `cargo update -p … --precise …`, which
        builds nothing; this release changes no lock entry, so normally none
        is re-applied). Then the version comparison (rule 19): a read of
        `Cargo.lock`, which needs no gate, before and after the merge, of
        these crates: gpui-component, gpui-base, gpui-kit, gpui-kit-assets,
        gpui-pre, gpui-fast, egui and iced; native-theme's resvg (0.48.1 at
        `0fa28027`) with its usvg and tiny-skia; and the resvg 0.46.x and
        usvg 0.46.x the two GPUI cores build (F.5 item 1's backlog entry
        cites resvg 0.46.0 and usvg 0.46.0). T0.2 refers to this list. A
        version within the accepted set (egui 0.36.2 or 0.37.x, egui 0.37
        being the likely change; patch releases of the others) is VF's
        rework (item 3); any other change is a stop. Its result says whether
        the merged tree can build before VF. Last,
        `cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo metadata --locked --format-version 1 > /dev/null`
        passes: it builds nothing, and it catches a lock that merged cleanly
        but is inconsistent.
     6. The resolution, the orchestrator's: every conflicted hunk by both
        sides' quoted texts (the specification's for this release's side, the
        other release's specification for its side), a citation like any
        other text (S6; where one side re-pointed a line a gate reads by
        number, S6's fourth gate or rule 25, that side wins; where both
        sides re-pointed the same one, the number is the base plus both
        sides' deltas, confirmed by the warm-up gate), each resolved hunk named in the
        record; where a
        resolution would change what either side's text means, it is a stop.
        A count or verdict tuple a test or gate checks is the base plus both
        sides' deltas; a prose count takes `<prev-tag>`'s side, named in the
        record, and X's recount sets its true value. `CHANGELOG.md` puts
        this release's section first; `ROADMAP.md` puts it after
        `<prev-tag>`'s. The resolution leaves `Cargo.lock` and the generated
        files out and never runs `git checkout --conflict=…` on a file it has
        touched.
     7. A file a generator writes is regenerated by its command on the merged
        tree, as a long run under the gate's wrappers (`gate.lock`, rule 2;
        always a long run, since `<prev-tag>`'s V has bumped the workspace
        crates in `Cargo.lock`), never resolved by hand; where the merged tree does not
        build before VF, it is taken from the HEAD recorded at sub-item 4,
        recorded as pending, and regenerated in VF's code commit, or, where VF
        makes no commit, by the orchestrator in a commit of its own before the
        warm-up gate, or, where the tree builds only after a warm-up fix (item
        5), in that fix's commit before the warm-up runs again.
     8. `git add` of the resolved and regenerated files, conflicted or clean;
        then the marker check,
        `git diff --cached -U0 | grep -E '^\+(<{7}|\|{7}|={7}|>{7})( |$)'`,
        prints nothing (diff3's `|||||||` included; a marker-like line that
        `<prev-tag>`'s own file has at that place is not counted). Then
        `cargo fmt --all --check` passes before the merge commit: on a
        failure, `cargo fmt --all` runs, the files it changed are re-staged
        and named in the record, and the check runs again; a file it changes
        outside the staged set is a stop.
     9. The staged-path check of §2: every path of
        `git diff --cached --name-only --no-renames` is in
        `git diff --name-only --no-renames <lane-pred-head> <prev-tag>` or is
        a regenerated file the record names; a path that is neither is a
        stop, named in the record, since anything beyond what the tag brings
        and the regenerations is a file nobody reviewed.
     10. `git commit --no-edit --cleanup=strip`, so the resolution is the
         merge commit itself; `<merge>` is recorded. Then
         `git log -1 --format=%P <merge>` prints the HEAD recorded at
         sub-item 4 and `git rev-parse <prev-tag>^{commit}`, in that order;
         `grep -m1 '^## \[' CHANGELOG.md` prints this release's heading; the
         status is clean. No sha changes: unit 3's evidence measurement keeps
         its commit, reachable through the merge, and the record's shas and
         commit subjects that name shas stay true.
  2. Superseded by S6.
  3. **VF, the version forms after MG** (rule 25), for every crate whose
     version sub-item 5 found changed. A source the new version needs that
     the registry lacks is fetched by the orchestrator with `cargo fetch`,
     which builds nothing. For each: a `worker` re-reads every upstream
     line the specification cites in the new version; an `implement` agent
     rewrites the forms this release's units took on the older version in
     code, tests and texts naming a version; a `worker` the documents' forms
     (a version named in a README or the CHANGELOG). Each rewrite is made
     only where the cited content (a behaviour, a value, an API) changed in
     the new version, never to update a version-pinned line citation that
     still holds (rule 25, S6), except a line a gate reads by number (the
     gpui showcase's `HOW` texts and colour claims, read via `CITED_CRATES`,
     and the fallback-stack test's gpui-pre lines), which is re-pointed where
     the new version moved it; "none needed" means no cited content changed
     and no gate-read line moved.
     VF leaves
     `ONE_COLOUR_TOLERANCE` and its doc alone: where native-theme's resvg,
     usvg or tiny-skia changed, the tolerance's re-measurement by B.3's
     procedure (it runs cargo, which agents before the warm-up do not) is
     made after the warm-up gate, as a fix with its own gate, `--commit`,
     whose commit sets the constant's value, its doc's date and its doc's
     resvg version together. VF's commits are reviewed within R-MG (item 4)
     and covered by the warm-up gate, with no gate of their own (rule 21);
     agents in the steps before item 5 run no cargo but
     `cargo fmt --all --check`, which builds nothing, except VF's `implement`
     agent, which may run one `nice 19` `-p` build or clippy of the crate it
     rewrote, through `slot.sh`. "None needed" is written into the
     record. A generated file item 1 recorded as pending is regenerated in
     VF's code commit, or, where VF makes none, by the orchestrator in a
     commit of its own before the warm-up gate, or, where the tree builds only
     after a warm-up fix (item 5), in that fix's commit before the warm-up
     runs again (rule 21 (1)).
  6. **The path check** (rule 22 (b)): every token starting `docs/` in this
     specification and this plan (its `:N` suffix and `#…` stripped), checked
     with `cd <core worktree> && git ls-files --error-unmatch <path>`, and
     every relative `](…)` link of the two documents, checked from `docs/`
     (`cd <core worktree>/docs && git ls-files --error-unmatch <link>`),
     brace sets expanded. Exempt: paths cited "on `<branch>`" or "at
     `<sha>`", paths a unit of this release creates, placeholders, glob or
     regex patterns, URLs, `crate::` links and texts quoted from other files.
     One that moved to `docs/archive/` is re-pointed by the `worker` in a
     documents-only commit, made before the warm-up gate and covered by it;
     one gone otherwise is a stop.
  7. **This release's results survived the merge** (rule 21 (7)), checked by
     the `worker` on the merged tree as it stands at this step, after items 3
     and 6 (`git show HEAD:<path>`, since both may commit after `<merge>`),
     comparing this release's blocks there with their state at `<merge>^1`
     (not with an earlier unit's text, which a later unit may have changed; a
     prose count item 1 resolved to `<prev-tag>`'s side is left for X's
     recount, not compared). The blocks VF rewrote in item 3 are expected
     differences from `<merge>^1`: the record lists them from VF's commits,
     and each is compared with the form the specification gives for the new
     version; a survived check never undoes VF. The table in the record:
     - the texts this release replaced are absent, and the remaining
       placement anchors of Parts A to F and this release's new texts, unit
       5's among them, are present, each with its line or "not found"; the
       anchors of the later tasks are found on the merged tree too (6a's
       `## [0.7.N] - <date>` heading, which V dates; every status line the
       close-out sets, the three documents', and `docs/archive/README.md`'s
       last 0.7.x row, which 7f edits);
     - E.14's claims on the merged tree, which now holds the KDE steps 4 and
       5: Edit drawn through `role_chrome_icon` with E.2's judgement; C.4's 23
       names unchanged; B.10's two Edit elements with their routes;
     - F.3's section placed above every other `## [` heading; F.4's section
       placed after `<prev-tag>`'s section.
     A failed claim that is a placement is fixed by the `worker` in a
     documents commit; one that is a code text goes to its unit's author, an
     `implement` fix (no cargo before the warm-up, rule 21), reviewed within
     R-MG. Both come before the warm-up gate and are covered by it (rule 21
     (5)). Only one whose cause is unclear is a stop (§9).
  4. **R-MG** (rule 21 (4)): where a file was resolved in item 1 or written
     by item 7, except Markdown that no test parses (so `.rs` files,
     manifests, workflows, configuration files, a TOML a test reads,
     `docs/showcase-exceptions.toml` and
     `scripts/connector_parity_exceptions.toml` among them, a script,
     `docs/platform-facts.md`, a README, which is compiled as doctests and
     parsed by tests, and `connectors/native-theme-egui/src/mapping.md`,
     which `mapping_doc.rs` reads, are in), where item 1 resolved
     `Cargo.lock`, or where VF committed anything: a `review` agent (a
     `review-hard` where those commits change a hard part of a core: a
     resolver, validation, a runtime, concurrency, `unsafe`; the handle's
     shared state, D.7 and decision 8, is one, as R4 says) reviews
     `git show --remerge-diff <merge>` and every commit of items 3, 6 and 7,
     with the record's list of resolved hunks (VF's review is always within
     R-MG). An R-MG must-fix is a new commit by the author of
     the commit it fixes (the orchestrator for a resolution, VF's agent for
     VF's), made before the warm-up gate and covered by it, with no gate of
     its own (rule 21).
  5. **The warm-up gate** (§4), last, on the clean tree, MG's `--commit`,
     covering every commit of items 1, 3, 4, 6 and 7; both GPUI Fast legs
     always run with it, since the merge brings the other side's code (and
     `<prev-tag>`'s version bump), and with each re-run of a red warm-up
     (rule 21 (5), rule 12), under the gate's lock as at unit 2: the connector's
     (`scripts/check_release.sh:710-716`, as at unit 2) and the showcase's in
     the runner environment (`:849-855`, as at unit 5 item 5). A red one
     goes, with the log's failing part, to the author of the unit whose rule
     the failing test checks, even where the failing file is the other
     side's (an author no longer at hand, such as the other side's, is
     replaced by a new `implement` agent given the log and that unit's part
     of its specification, rule 12): a fix after the units by an `implement` agent, reviewed (§3),
     under its unit's done-criterion, and gated by the warm-up run again
     (MG's `--commit` once more); one caused by a conflict resolution is the
     orchestrator's, and one whose cause is unclear is a stop (rule 12). The
     record names every later warm-up run, with the fix it gates.
  No re-measurement of unit 3 after MG (rule 19): its run measured this
  release's own code before MG, and item 7's E.14 checks show on the merged
  tree that MG left what it measured in place (C.4's 23 names unchanged, Edit
  drawn through `role_chrome_icon` with E.2's judgement, B.10's two Edit
  elements with their routes), a failure there being fixed or a stop as item
  7 says;
  rule 19's layout "after" blocks do not apply to unit 3's evidence, which is
  not a layout dump. The sheets stay until the
  release's first push at the earliest.
- **Done when:** item 1's tag and ancestor checks passed before the merge;
  the marker check printed nothing; the merge commit's parents are the two
  named in item 1; `git status --porcelain --untracked-files=no` prints
  nothing; every resolution is named in the record; VF's commits and R-MG,
  where they ran, are done and closed; the path check and the
  survived-the-merge table hold, each with its fixes committed; the warm-up
  gate passes.
- **Commits:** before the warm-up gate, the merge commit with its
  resolutions, VF's commits, the path check's re-points, item 7's fixes and
  an R-MG must-fix, all covered by that gate with no gate of their own (rule
  21 (5)). After the warm-up gate, each fix is a commit of its own with its
  own gate: a warm-up fix (its unit's done-criterion, gated by the warm-up
  run again, rule 12; the record names every later warm-up run); a
  re-measured tolerance (`--commit`). Then the push (§4).

### X — The recount after MG (batch rule 17)

- **By:** `worker`, no cargo, in the core worktree, once MG's warm-up gate
  has passed (and any re-measured tolerance is committed), before V. Where
  there is no MG there is no X (rule 17).
- **What:** every stated count in every file of the union of
  `git diff --name-only <lane-pred-head> <merge>^1` and
  `git diff --name-only <lane-pred-head> <prev-tag>`, outside the pinned
  documents (`docs/archive/`, `docs/evidence/`, `docs/issues/`, the dated
  `docs/todo_v0.7.1_*` and `docs/todo_v0.7.x_*` documents and the files
  beside them, `docs/todo.md`'s dated block), recounted on the merged tree,
  F.4's count among them and the prose counts MG item 1 resolved to
  `<prev-tag>`'s side; then the five documentation checkers. No citation is
  re-pointed (S6).
- **Done when:** each changed count is in the commit body with its old and
  new value and how it was counted; the five checkers pass; the change is
  staged; `--quick` passes on it (a documents unit, §2, *The cadence*), with
  the GPUI Fast legs where X's commit changes code or strings under
  `connectors/native-theme-gpui/` (rule 11). Commit, where any count
  changed: `docs: the stated counts on the merged tree`.

### Unit 7 — The asset run and the close-out

Tasks V and 7b to 7f, in §6 and §7.

## 3. Reviews

Every commit that changes behaviour is reviewed before the release branch is
offered for merging. The reviewer gets the commit range and the specification's
header and part, never the author's report; it reads the commits (`git show`),
not a working tree, and runs no cargo. Every review brief carries the licence
rule of §8 as a check of its own: no new code transcribes an LGPL source, and
every Apache-2.0 port has a file-level notice. A review runs while the lane's next
task works, but 3b, the run, waits until R1, R2 and R3 are closed and their
fixes committed and gated again, since a fix made after it would leave the
table describing code that no longer exists (§5). A
must-fix is a new change by the task's author (the same agent, continued),
staged after the commit of the task then running in the lane, with the
author's unit's done-criterion run again (its targeted commands by the author,
its gate by the orchestrator, who then commits, §2), and reviewed by the same
reviewer. A second must-fix
on the same point falls under §9's take-over rule.

The maintainer's rule decides who reviews by size: a commit of up to a few
hundred changed lines that an Opus agent wrote is reviewed by the orchestrator
itself, a larger one by a `review` agent, and one in the hard parts of a core by
a `review-hard` agent whatever its size. The orchestrator reads
`git diff --stat` and says which it chose.

| Review | Commit | Reviewer | Why |
|---|---|---|---|
| R1 | unit 1 | by size | a public method of the core and the extraction of the lookup every freedesktop icon takes; its order is pinned by A.3's and A.4's tests. Not `review-hard`: no concurrency, no `unsafe`, no theme resolution or validation; a loader's search order, every case of which a test states |
| R2 | unit 2 | by size (expected `review`) | values and arithmetic under the strict-panic lints, the tolerance's derivation, the lookup per set; its brief carries the plan's two unit-2 additions (§10 item 3) |
| R3 | 3a | by size, closed before 3b | the instrument decides seven stop conditions, and no gate reads its rows |
| R4 | unit 4 | `review-hard` | shared state read by the asset source from any thread (decision 8: a lock and atomics), a state machine whose every transition must fall under D.7's one rule, a cache keyed once per choice (decision 24); with D.13, tasks in flight that a later choice must win over. A miss there serves another set's icons, or none, with no gate red |
| R5 | unit 5 | by size (expected `review`) | the showcase's handling of the handle's absence, the reloads' telling (E.2), the tests E.10 rewrites |
| R-MG | MG's merge commit, where a code file was resolved | `review` | `git show --remerge-diff <merge>` with the record's list of resolved hunks (rule 21); a must-fix is a new commit made before MG's warm-up gate and covered by it, with no gate of its own |
| R-VF | VF's commits (MG item 3) | within R-MG (`review`) | covered by MG's warm-up gate (rule 25) |
| — | MG's commits after the warm-up gate (warm-up fixes, a re-measured tolerance) | as their kind: a code fix by size or `review`, its own gate | each comes after MG's warm-up gate, so each has its own gate, a warm-up fix's being the warm-up run again (MG, *Commits*; rule 12); the path check's and item 7's fixes come before it, within R-MG where they touch code (MG item 4) |
| — | 3c, 3d, 6a, 6b, the merge, V, 7f | the checkers, then the orchestrator reads the diff | data and texts the specification reviewed; 3d restores a reviewed text |

At most one Fable agent runs at a time: R4 waits while an `implement-hard` runs,
and a `design` check of an amended specification or plan (§5, *After a stop*)
waits for R4.

## 4. Gates

- **While working**, an agent runs the tests of the module it changes. **Before
  it stages**, it runs its task's targeted done-criterion commands once, in the
  foreground, through the wrappers (§8); the gate is the orchestrator's (§2).
- **A gate at every unit's end**, on the staged change (on the merge commit
  for a merge), by the orchestrator: `--quick` or `--commit` by §2's cadence
  (S1), `--commit` at 3a, unit 5 and every warm-up gate (T0.2 or T0.3, MG,
  V); the merge of unit 6, a documents unit with no gated head (rule 20),
  takes the five checkers and `--quick`. **The GPUI Fast legs**, the connector's and the
  showcase's in the runner environment (unit 2, item 5), run as long runs at
  every gpui unit, one whose diff changes code or strings under
  `connectors/native-theme-gpui/` (rule 11): units 2, 3a, 3d, 4 and 5 (G.1
  names the connector's leg at 2 and 4; §10 item 3). Units 1, 3c, 6a, 6b,
  their merge and V are not gpui units as planned (6a and 6b change the
  connector's README, Markdown, and no code or string); the orchestrator
  checks each unit's staged diff against the test before its gate
  (`git diff --cached -- connectors/native-theme-gpui/`), and a unit the test
  catches runs both legs too. Both legs also always run with MG's warm-up and
  each re-run of it (rule 21 (5)); the ignored
  stem-index test (about 30 to 50
  seconds) is unit 1's agent's.
- **The documentation checkers** (no cargo): `scripts/check_platform_facts_rows.py`,
  `check_section_refs.py`, `check_preset_citations.py`,
  `check_wrong_safety_nets.py`, `check_todo_citations.py`
  (`scripts/check_release.sh:964-983`; G.3).
- **The two release stamps** are stale from unit 1 on (G.3, its last row; G.4)
  and are no task's done-criterion before 7b.
- **The full `scripts/check_release.sh`**: at 7c (one failure, the provenance
  stamp, G.4) and at 7f (none). In both, the `cargo outdated` warning is
  expected (`scripts/check_release.sh:1280`, a warning and no failure, since
  newer releases exist that the workspace does not take); its list is written
  into the record (§12).
- **Long runs.** The Bash tool kills a foreground command at 10 minutes. These
  are the orchestrator's long runs (§2; rules 2 and 5): every
  `--commit`, the full check (about 23 minutes), the asset script (30 to 37),
  the compatibility run (7b), the release builds of 7d, the GPUI Fast legs,
  3a's trial runs and 3b's run, and any other run not known to finish within
  10 minutes. Each runs with `run_in_background`, from the stated worktree,
  through the wrappers (§6 says which for unit 7), and writes into a new
  folder of its own, `<run>` = `<logs>/<task>/run-<n>/`, `<task>` the task's
  name (`u1-commit`, `3b`, `7e`, …) and `<n>` one more than that task's last
  run, made with `mkdir -p <logs>/<task> && mkdir <logs>/<task>/run-<n>` (the
  second `mkdir` fails where the folder exists, so no run reuses one). Its
  form:
  `cd <worktree> && <wrappers> <command> > <run>/log 2>&1; echo $? > <run>/status`,
  where a heavy run's `<wrappers>` (every long run of the orchestrator's
  that runs cargo: a gate, the commands run as part of it, MG's
  regenerations, a trial, a measurement, the compatibility run, the release
  builds; rule 2) are
  `flock <scratch-keep>/slot-locks/gate.lock <tools>/slot.sh <tools>/lowprio.sh`,
  and another long run's (the rehearsal, which runs no cargo) are
  `<tools>/slot.sh` alone or as its task says.
  A repeated run (a gate after a red result, the asset run after a fix, the
  rehearsal, 3a's trials) thus never meets an earlier run's `status`, which a
  Monitor's `[ -s <STATUS> ]` would take as its own end, and the earlier
  run's log stays readable. The record names each run's folder. The
  orchestrator waits for the exit notice, with no polling loop, then reads
  `STATUS` and the log's tail; a wait on a condition inside a run (7e's) uses
  the Monitor tool with an until-loop on a file, never a `pgrep -f` loop. The
  first `--commit` is the first warm-up gate (below), run and timed at T0.3,
  or at T0.2's end where there is no unit 0, and its time is written into the
  record with whether it ran cold (a new worktree, an empty build folder) or
  warm (the reused worktree's build folder). Agents run only commands
  expected to finish well within 10 minutes.
- **The warm-up gate.** An agent's targeted commands build the feature sets
  the gate's test legs build (the gpui connector's `native,watch`,
  `scripts/check_release.sh:667`), so before an agent that runs cargo is
  dispatched the orchestrator runs `scripts/check_release.sh --commit` on the
  clean tree, as a long run in its own folder (`<logs>/warmup/run-<n>/`),
  wherever the worktree is new, `Cargo.lock` changed since that folder's last
  gate, or the rustup hook reported a changed toolchain; then those builds
  are warm and an agent's commands stay within the Bash tool's limit. T0.3's
  gate serves where T0.3 exists, as it runs before any agent that runs cargo
  (T0.4's runs none); where there is no T0.3, T0.2 runs it at its end. Later,
  a unit's own gate on the previous unit leaves the tree warm, so a warm-up
  is needed again only where one of the three conditions arises (a lock
  change, such as V's or one MG's merge brings, or a new toolchain); MG
  always runs one (its item 5; rules 17, 21). A warm-up gate counts as a
  `--commit` for §2's cadence.
- **The minimum supported Rust version** is checked by CI's `msrv` job and
  by `--commit` and `--quick` too (`docs/todo_v0.7.1_gate-speed-spec.md`,
  decision 8, Part M; O3's keyed option and O2 (e), approved 2026-11-15: the
  1.88 checks build in a fresh folder deleted after every run, and `--quick`
  skips them by key where nothing they read changed). The briefs say:
  no language or library feature newer than the edited crate's own
  `rust-version` (the workspace's for the core, the connector's own
  `connectors/native-theme-gpui/Cargo.toml:11`).
- **CI** runs on pull requests and on `main` only (`.github/workflows/ci.yml:6-10`),
  so a push checks anything only once a pull request is open: the
  orchestrator opens the draft pull request at the branch's first push
  (rule 9; approved by the maintainer on 2026-11-15, §10 item 2). Pushes,
  each with an explicit refspec
  (`git push origin v0.7.N-rc1`). Nothing of this release is pushed before
  MG (S2, batch rule 17), so the first push follows MG's warm-up gate; then
  at 7b (after the compatibility run, S4) and at 7f. Where T0.2 made the
  branch from `<prev-tag>` (no MG), the pushes also go after unit 2 (the
  first check of the connector's macOS and Windows builds, B.2's
  `cfg(not(target_os = "linux"))` statement), after unit 4 and after the merge
  of unit 6. With MG, those macOS and Windows builds are first checked at
  MG's push. The orchestrator watches each run in the background.
  A red run emails the maintainer; no push is made that is expected to be red.
  If one is red, the orchestrator tells the maintainer what failed and why, and
  the failing change's author fixes it (reviewed as in §3) as the lane's next
  commit; the fix is pushed once that unit's done-criterion holds again, and
  in the close-out as §6's rule says (batch rule 15).

## 5. The measurement (unit 3)

- **The announcement.** Before 3a's first trial run the orchestrator tells the
  maintainer that unit 3's test reads the maintainer's installed icon themes
  through the library, as the connector's installed-theme tests do when the
  gates run them (C.2), first in 3a's trial runs and then in 3b's run; that
  nothing of it appears on the desktop; and that the packages' versions will be
  asked for at the run (open point 3). No agent reads an icon base directory
  itself (§8).
- **The run.** 3b runs C.2's command once, in the core worktree at the lane's
  head once R1, R2 and R3 are closed and every fix they asked for is committed
  and gated again (that head is C.6's `<sha>`), outside the runner
  environment, through the wrappers holding both slots (3b), while no other
  cargo process runs (the docs lane does not exist yet, and reviewers run
  none), so that the timings
  of C.4 item 7 share one quiet machine. The record (§12) says that they ran
  at low priority. The run's "showcase's reload of today" (C.4 item 7)
  is the showcase's code on the branch at 3a: v0.7.1's showcase with B.1's
  two table entries (unit 2), v0.7.1 being the lane predecessor (the KDE
  lane's releases come in only at MG); C.4 names `0fa28027` because it was written there. The
  one-colour judgements C.4 item 7 adds to the handle's side of a step are of
  the icons the showcase's side of the same step loads, since Part E's lists do
  not exist at unit 3. Settled by the orchestrator (2026-11-15).
- **The versions.** At the run, before 3b's run starts, the orchestrator asks
  the maintainer to read, with the package manager, the package and version of
  each installed icon theme (the owner of its directory, or "not from a
  package"), and not to update packages until the run ends, so that the
  versions are those the run read. The orchestrator writes the README's table
  from that answer in 3c, for every theme `listing.md` names, as C.4 says ("not
  given" for a theme the answer leaves out, and for every theme where the
  maintainer has not answered by 3c: 3c does not wait for it). 3b's worker
  reports without it.
- **The verdict.** The orchestrator reads `listing.md`, the sheets C.5 lists
  (in the core worktree's `target/widget-icons-sheets/<date>/`) and 3b's drafts,
  and settles each of C.5's seven conditions, condition 3 by eye from the
  sheets. None met: before 3c the maintainer is sent the seven verdicts with
  the sheets' paths, so that C.4's README sentence (the sheets were "shown to
  the maintainer") is true; then 3c, 3d and unit 4. One met: the plan stops
  (§9) and the maintainer gets the table, the sheets' paths and the condition;
  at condition 7 the maintainer judges both medians of every step of the run
  (open point 1), which decides D.13. The maintainer's word goes into the
  record (§12) with the verdicts. The sheets are deleted by §7, not here.
- **After a stop.** D.13, if chosen, is already specified and goes into unit 4.
  Any other change the maintainer decides (`Check`'s order, the size, an entry)
  is made in the specification first, and this plan gains its task before unit
  4, with whether the run is repeated. The amended specification and the
  amended plan each get the checks the maintainer's rules give such documents
  (`doc-review` passes, then one confirming `design` check) before the
  maintainer approves them.

## 6. The asset run (unit 7)

In G.4's order, with the batch's S4: the branch is pushed once the
compatibility run (7b) has passed, and the full check (7c) runs while CI runs
on that push; the asset run (7e) starts only when both are green (rule 9:
CI green, and the full check's only failure the provenance stamp, which only
the asset run refreshes). The cost S4
accepts: a CI run can be red, and email the maintainer, for a fault the full
check would have shown first; the gates have by then checked formatting,
clippy and the tests. One deliberate departure beyond S4: 7d runs while CI
runs, not after it. Its builds and its rehearsal commit nothing and need
neither result, and 7e still waits for both 7c and CI. 7d's release builds
are heavy runs and take the gate lock (rule 2), so they wait while 7c's full
check holds it and run once it has ended; its rehearsal runs no cargo and
takes only its Xvfb lock. Unit 7 starts only after MG and X (S2). A fix the
close-out needs follows rule 15, which this plan cites rather than
restates; its terms here: the close-out's first step is 7b's `cargo audit`,
its push step 7b's push, its asset commit 7f's, and its full check
beside CI 7c's.

Every command of unit 7 runs in the core worktree and says so: it starts with
`cd <core worktree> &&`, and 7e's `run.sh` inherits that directory. A repository script
finds the repository's root from its own path (`scripts/check_release.sh:73`,
`scripts/generate_assets_release.sh:21-22`) or from the current directory
(`scripts/update_compatibility.sh:37`, `git rev-parse --show-toplevel`), so a
script run from the main checkout would be another branch's script, or would
work on another branch's tree.

- **V — the version.** Orchestrator, after MG and X (with no MG, after the
  merge of unit 6; rule 17: with no MG there is no X). Not in the specification: the
  release mechanics the 0.7.1 release used (`a3c10475`). The workspace's
  version and the inter-crate requirements become `0.7.N`, the lock follows,
  and the `## [0.7.N] - <date>` heading 6a wrote gets V's day as its date
  (batch rule 14).
  Done when: the
  seven crates whose manifests say `version.workspace = true` (native-theme,
  native-theme-derive, native-theme-build and the four connector crates)
  report `0.7.N` in `cargo metadata`, and the three tools (`tools/os-probe`,
  `tools/showcase-support`, `tools/test-fixtures`) keep their own `0.0.0`, as
  at `a3c10475`; `git diff` of
  `Cargo.lock` changes only the workspace crates' versions;
  `python3 scripts/check_readme_versions.py` passes; no stamped path names one
  of this release's planning documents at a live path (rule 22 (c); the
  documents move only at 7f, after the asset run). Over the stamped paths,
  `scripts/update_provenance.sh`'s `SOURCE_PATHS` (`:39-70` at `0fa28027`)
  and `scripts/update_compatibility.sh`'s source paths of every connector
  (`:20-23`: each connector's `Cargo.toml`, `src`, `examples` and `tests`,
  egui's `mapping.toml` and the egui widgets crate's `Cargo.toml` and
  `src`), both read again at V, this prints nothing (the stem matching the
  brace form `{spec,plan}` too, filtered per match):

  ```sh
  cd <core worktree> && git grep -noE '(archive/)?todo_v0\.7\.[1x]_gpui-widget-icons[a-z{},.-]*' -- Cargo.toml Cargo.lock native-theme/Cargo.toml native-theme/build.rs native-theme/src native-theme/icons native-theme-derive/Cargo.toml native-theme-derive/src native-theme-build/Cargo.toml native-theme-build/src connectors/native-theme-gpui/Cargo.toml connectors/native-theme-gpui/src connectors/native-theme-gpui/examples connectors/native-theme-gpui/tests connectors/native-theme-iced/Cargo.toml connectors/native-theme-iced/src connectors/native-theme-iced/examples connectors/native-theme-iced/tests connectors/native-theme-egui/Cargo.toml connectors/native-theme-egui/src connectors/native-theme-egui/examples connectors/native-theme-egui/tests connectors/native-theme-egui/mapping.toml connectors/native-theme-egui-widgets/Cargo.toml connectors/native-theme-egui-widgets/src tools/showcase-support/Cargo.toml tools/showcase-support/src scripts/generate_screenshots_iced.sh scripts/generate_screenshots_gpui.sh scripts/generate_screenshots_egui.sh scripts/generate_gifs_theme_switching.sh scripts/capture_window.sh scripts/generate_gifs_spinners.py .github/workflows/screenshots.yml | grep -v ':archive/'
  ```

  (the paths as `0fa28027` has them; a path either script adds or drops by
  V is added or dropped here); a hit is re-pointed to its archived path in
  V's commit;
  `--commit` passes, the warm-up gate of its lock change (rule 6). It comes
  before the asset run because the provenance stamp hashes `Cargo.lock`. Commit:
  `chore: v0.7.N version …`.
- **7b.** In the core worktree, these pass; the second resolves the newest
  upstream releases and runs the three connectors' gates on them, so it is a
  long run (§4), its `<run>` a new folder under `<logs>/7b/`:

  ```sh
  cd <core worktree> && <tools>/slot.sh <tools>/lowprio.sh cargo audit
  cd <core worktree> && flock <scratch-keep>/slot-locks/gate.lock <tools>/slot.sh <tools>/lowprio.sh bash scripts/update_compatibility.sh run > <run>/log 2>&1; echo $? > <run>/status
  ```

  The second leaves `docs/COMPATIBILITY.toml` and the connector READMEs changed and
  uncommitted until 7f, as at 0.7.1. An advisory or an upstream release that
  moves stops the run (§9). Once both have passed, the branch is pushed
  (`git push origin v0.7.N-rc1`), and CI starts on it (S4).
- **7c.** While CI runs on 7b's push, the full check, run in the background
  (§4), its `<run>` a new folder under `<logs>/7c/`, as

  ```sh
  cd <core worktree> && flock <scratch-keep>/slot-locks/gate.lock <tools>/slot.sh <tools>/lowprio.sh bash scripts/check_release.sh > <run>/log 2>&1; echo $? > <run>/status
  ```

  fails on the provenance stamp alone; the expected `cargo outdated` warning's
  list and every other warning are named in the record (§12). 7c holds when
  the full check shows that and CI on the draft pull request, open since the
  first push (§10, item 2), is green (rule 9). A fault CI finds that the
  full check also shows is fixed
  once (§9).
- **7d** (while CI runs, its builds after 7c's full check under the gate
  lock; it commits nothing). The
  three showcases are built for release with the capture scripts' own command
  (`scripts/capture_window.sh:440`, with each script's features:
  `scripts/generate_screenshots_gpui.sh:37` none, `_iced.sh:35`
  `--features iced_aw`, `_egui.sh:39` `--all-features`), so that the
  desktop is needed for the capture alone; the builds are heavy long runs
  (§4), under the gate lock and both wrappers (rule 2):

  ```sh
  cd <core worktree> && flock <scratch-keep>/slot-locks/gate.lock <tools>/slot.sh <tools>/lowprio.sh cargo build -p native-theme-gpui --example showcase-gpui --release
  cd <core worktree> && flock <scratch-keep>/slot-locks/gate.lock <tools>/slot.sh <tools>/lowprio.sh cargo build -p native-theme-iced --example showcase-iced --release --features iced_aw
  cd <core worktree> && flock <scratch-keep>/slot-locks/gate.lock <tools>/slot.sh <tools>/lowprio.sh cargo build -p native-theme-egui --example showcase-egui --release --all-features
  ```

  Then a rehearsal of the parity step's layout half,
  without the desktop. Its tool is
  `<scratch-keep>/batch-2026-11-15/gates/rehearsal.sh <out-dir>`, the one the
  0.7.1 release ran (read 2026-11-15): it starts a private Xvfb on `:97`, runs
  each release showcase (`target/release/examples/showcase-{egui,iced,gpui}`)
  there with `--tab basic --capture --dump-layout` and the presets and icon
  sets of the capture scripts (kde-breeze, material-3, catppuccin-mocha, each
  mode), waits up to 60 s for each dump, and writes `summary.txt`. The other
  script read, `<scratch-keep>/issue4-0.7.1/issue4/x11/shot.sh`, is not it: it
  takes one screenshot of one binary at a given `Xft.dpi` for the 0.7.1 font
  measurement and dumps no layout. `rehearsal.sh` hard-codes the 0.7.1
  worktree (`W=…/worktrees/v071`), so the orchestrator copies it to
  `<scratch-keep>/widget-icons-0.7.N/tools/` with `W` set to the core
  worktree, and reads the copy before running it. Then
  `cd <core worktree> && python3 scripts/check_showcase_parity.py <out-dir>`
  compares the dumps, as the asset script does
  (`scripts/generate_assets_release.sh:164`). It compares geometry only and
  runs without a window manager and in software (Xvfb has no GPU), as the
  0.7.1 plan's rehearsal did, so a difference it shows before the capture is
  reported while the capture goes ahead (rule 19); a fault is what the asset
  run's comparator shows after the capture (7e). The rehearsal runs as
  `cd <core worktree> && flock <scratch-keep>/slot-locks/xvfb97.lock <tools>/slot.sh bash <tools>/rehearsal.sh <out-dir> > <run>/log 2>&1; echo $? > <run>/status`,
  a long run (§4): 18 showcase starts at up to 60 s each, 18 minutes at
  worst; its `<run>` is a new folder under `<logs>/7d/`, and `<out-dir>` is
  `<scratch-keep>/widget-icons-0.7.N/rehearsal/run-<n>/`, with the same `<n>`,
  so a repeated rehearsal leaves no earlier dump or `summary.txt` to read. Every
  `dump=no` line of its `summary.txt` is reported to the maintainer. It holds a
  slot, does not run under `lowprio.sh`, and runs without `kde-inhibit`,
  which reserves nothing on a private X server. It keeps normal priority
  because each dump is waited for within a time limit (60 s in
  `rehearsal.sh`; the captures' own limit is 20 s, `SHOWCASE_WINDOW_TIMEOUT=20`,
  `scripts/capture_window.sh:36`, `:394`), which software rendering at nice 19
  could miss. The rehearsal uses Xvfb display `:97`, which the other lane's
  rehearsal uses too, so it runs one at a time across both lanes (rule 24),
  held by the shared lock `<scratch-keep>/slot-locks/xvfb97.lock` (the
  `flock` in the command above): a lane's rehearsal waits mechanically while
  the other's holds it.
- **7e — the capture.** When 7c's full check and CI are both green (rule 9)
  and 7d is
  done, the orchestrator starts the run below, which takes the shared lock
  `<scratch-keep>/slot-locks/assets.lock` first (rule 24: one asset run and
  capture window at a time across both lanes, a lane waiting mechanically
  while the other's run holds it), then the gate lock (rule 2: the asset
  script is a heavy run) and a cargo slot inside it: once it holds all three
  it writes `<run>/HELD` and waits for `<run>/GO`. A Monitor
  until-loop,
  `until [ -e <run>/HELD ] || [ -s <run>/OUTER.STATUS ]; do sleep 10; done`,
  tells the orchestrator the locks and the slot are held, or that the line
  ended before `HELD` (`OUTER.STATUS`, the exit code of the `flock` part,
  written when that part ends; rule 24). An `OUTER.STATUS` first is a
  failure before `HELD`: the maintainer is not asked, and the orchestrator
  reads the run's output (the line's background output; `<log>` starts only
  after `GO`). Only once `HELD` exists does it ask the
  maintainer for "capture now": the desktop unlocked and untouched for 4 to 6
  minutes, announced before it starts. On the maintainer's word,
  `touch <run>/GO` starts the capture, and
  `scripts/generate_assets_release.sh` runs once. `kde-inhibit` wraps the
  asset script alone, after `GO`, so that screen locking, power management
  and notifications stay on while the run waits for the locks, the slot and
  the maintainer's answer (rule 24; the KDE steps 2 and 3 plan's form, its
  task K6). The orchestrator first writes `<run>/run.sh` with the Write tool,
  holding this one line:

  ```sh
  CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16 bash scripts/generate_assets_release.sh > <log> 2>&1; echo $? > <STATUS>
  ```

  and then starts the long run:

  ```sh
  cd <core worktree> && flock <scratch-keep>/slot-locks/assets.lock flock <scratch-keep>/slot-locks/gate.lock <tools>/slot.sh sh -c 'touch <run>/HELD; until [ -e <run>/GO ]; do sleep 5; done; kde-inhibit --power --screenSaver --notifications bash <run>/run.sh'; echo $? > <run>/OUTER.STATUS
  ```

  (the `kde-inhibit` options earlier runs used; the job
  caps reach the script's last step, `update_compatibility.sh run`, which runs
  the three connectors' gates after the captures, so the machine stays busy
  until the script ends): it holds a cargo slot, and it does not run
  under `lowprio.sh`, since earlier runs captured at normal priority, the
  desktop is reserved for the capture and the captures are timing-sensitive.
  `kde-inhibit` hides the script's exit code, so the orchestrator reads the
  `STATUS` file and the log's tail, never the wrapper's status. The line runs
  with `run_in_background`, its `<log>` and `<STATUS>` the `log` and `status`
  of a new folder under `<logs>/7e/` (§4), so that the Monitor below never
  ends on an earlier attempt's `status`, and the
  orchestrator waits for its exit notice (§4). To free the maintainer's desktop
  as soon as the captures end, the orchestrator starts a Monitor beside it, an
  until-loop on the log:

  ```sh
  until grep -q 'Theme-switching GIFs generated' <log> || [ -s <STATUS> ] || [ -s <run>/OUTER.STATUS ]; do sleep 10; done
  ```

  When it ends on "Theme-switching GIFs generated"
  (`scripts/generate_assets_release.sh:172`, the last step that captures), the
  orchestrator tells the maintainer that the desktop is free again; the script
  then waits for the CI screenshots. The run itself commits nothing; a fix
  found from the asset run on follows rule 15. A parity difference is a
  fault, not a question for the maintainer (G.4, rule 19): one this work's
  changes explain is fixed (§9, *A fix after unit 5*); one that is a
  toolkit's limit gets a `[[parity.group]]` exception in
  `docs/showcase-exceptions.toml` with its reason and source, only for the
  difference the comparison shows, reviewed (§3); the maintainer is told in
  either case. Either way the asset run is made again: the difference fails
  the asset script at its step 6 (`scripts/generate_assets_release.sh`, "Step
  6/7: Showcase parity at rest"), before the GIFs, so the run did not finish
  and the fix or exception follows rule 15's case of a script that had not
  finished; 7f waits until it is settled.
  Then the orchestrator
  opens the new gpui screenshots beside the previous release's and
  tells the maintainer what changed, against G.4's list of what this work shows.
- **7f — the commit.** One commit: the assets, both stamps, the READMEs' lines,
  the changelog heading's date changed to the commit's day only where that
  day differs from V's, and F.6 (§7). Push. Done
  when: the full `scripts/check_release.sh` at that commit, run as in 7c from
  the core worktree (a long run, §4, its `<run>` a new folder under
  `<logs>/7f/`), reports
  no failure, with the expected `cargo outdated` warning, and CI is green.

## 7. Close-out

- **Archiving is in 7f, the last commit**: the rationale, the specification and
  this plan move to `docs/archive/` with their names (decision 18), the archive
  README gains F.6's row, every path mention of the three documents in every
  tracked file outside `docs/archive/` is rewritten for the new place (F.6;
  batch rule 22 (a); a document whose header dates its lines keeps its line
  citations, never a stale path, so `docs/issues/` and the batch's other
  `docs/todo_v0.7.*` documents are included), the three status lines say
  implemented with the commit range, read as
  `git log --first-parent <base>..HEAD` (rule 21; `<base>` is
  `<lane-pred-head>`, or `<prev-tag>` on the no-MG path), and the
  orchestrator copies into §12 the record's entries after the run, up to that
  commit. The mentions are found with F.6's `git grep -nE` after the move. A
  mention that already points into `docs/archive/` (in any relative form, or
  as the absolute URL) is left, and so are a citation dated "at `<sha>`" and
  a path inside another document's instructions that names its own future
  target, each named in the commit body (rule 22 (a)).
  The check filters per match, not per line (rule 22 (a)), and prints
  nothing but the mentions the exemptions above leave, each named in the
  commit body:

  ```sh
  cd <core worktree> && git grep -noE '(archive/)?todo_v0\.7\.(1_gpui-widget-icons-rationale|x_gpui-widget-icons-(spec|plan)|x_gpui-widget-icons-\{spec,plan\})' -- ':!docs/archive' | grep -v ':archive/'
  ```

  Then the three moved documents' own relative links are checked again from
  `docs/archive/`
  (`cd <core worktree>/docs/archive && git ls-files --error-unmatch <link>`
  for each), with rule 22 (b)'s exemptions (placeholders, texts quoted from
  other files, URLs), and each that no longer resolves is rewritten in the
  same commit (rule 22 (a)). No later-links list is written (rule 22).
  None of these paths is stamped, so the stamps stay valid; the full check at
  7f confirms it. 7f's own results, its full
  check and its CI run, cannot be in the commit they check: the orchestrator
  reports them to the maintainer.
- After 7f the orchestrator removes the docs worktree with an absolute path
  (`git worktree remove /home/tibi/Rust/native-theme/.claude/worktrees/lane2-a-docs`,
  rule 24) and its merged branch
  (`git -C <core worktree> branch -d v0.7.N-rc1-docs`). No `cargo clean`.
  This release is the second lane's last (S3 as amended, §1), so the
  close-out then removes the core worktree, `lane2-a`, and its build folder:
  once the sheets are deleted or kept elsewhere (below; they live in that
  worktree's `target/`), the orchestrator first clears the untracked files
  `git worktree remove` would refuse on: each file
  `git -C <core worktree> status --porcelain --untracked-files=all` lists as
  `??` (the planning copies T0.2 recorded) is compared with `cmp` with its
  original in `.claude/worktrees/v071/docs/` and removed where equal; one
  that differs, or whose original is gone, is copied first to
  `<scratch-keep>/widget-icons-0.7.N/untracked/` and named in the record,
  then removed (never `--force`). Then it runs
  `git -C /home/tibi/Rust/native-theme worktree remove /home/tibi/Rust/native-theme/.claude/worktrees/lane2-a`
  (the branch `v0.7.N-rc1` stays, for the pull request) and deletes the build
  folder T0.2 recorded (`<cargo-build>/<hash>`), and reports what it freed.
  Where the sheets' answer has not come by 7f, the removal waits for it and
  is named, with both folders, in 7f's report and the project's status note.
  A fix CI or the maintainer asks for after the removal is made in a new
  worktree of `v0.7.N-rc1` (`git worktree add` with absolute paths, a cold
  first build), removed again the same way once the fix is pushed.
- **The batch's own rationale** (rule 22 (a)): where this release is the
  last of the batch in the maintainer's order (not so under D10's proposal,
  where the KDE steps 6 and 7 come last), 7f also moves
  `docs/todo_v0.7.1_batch-speed-rationale.md` to `docs/archive/` by the same
  rules as the three documents (the per-match check with its pattern, the
  path mentions rewritten, its archive row); otherwise nothing of it moves
  here.
- **The sheets.** `target/widget-icons-sheets/` in the core worktree is deleted
  once the maintainer says the sheets were seen or are not needed, and not
  before; C.4's README says they were shown to the maintainer and are not kept,
  and the deletion makes its second half true. Where the answer has not come
  by 7f, the pending deletion is named, with the folder, in 7f's report and
  the project's status note.
- **Merging the pull request, tagging, publishing, the GitHub release and the
  pull request's title and description are the maintainer's** and are not part
  of this plan.

## 8. Rules every agent brief carries

- **Inputs.** The brief gives the specification's header and the task's parts;
  the parts are the whole task: nothing beyond them, no refactoring, no
  formatting churn. A needed change the parts do not name is reported, not made.
  Every edit is placed by the text it quotes, the item it names or the test's
  name; where the quoted text no longer stands, the agent stops and reports
  (the header), and the orchestrator settles it as §9 says: it finds the text
  in its present form and has the edit made there where the edit's meaning
  is unchanged, or asks the maintainer. The lines the colour claims pin do
  not move (*Lines that must not move*).
- **The worktree.** The brief names the lane's worktree by its absolute path;
  every command starts with `cd` into it or uses `git -C` and `--manifest-path`.
  The main checkout and the KDE lane's worktrees (`.claude/worktrees/v071`,
  `.claude/worktrees/kde-b`, §1) are other branches with the same relative
  paths: nothing is read, run or written there (but T0.3's copy),
  and a check that passes there proves nothing. A new worktree may be made
  with `git worktree add` run from the main checkout (§1); that command
  changes nothing in the main checkout's own tree. The report quotes
  `git -C <worktree> log -1 --oneline`.
- **Archived paths in code** (rule 22 (c)), in every `implement` brief: code,
  tests and rustdoc that name one of this release's own planning documents
  name its archived path, `docs/archive/…`, since the close-out moves the
  documents after the asset run (decision 23 already names backlog entries by
  title instead, so such a mention should not arise); V checks it (§6).
- **The licence rule (D10 of the maintainer's rules, not the KDE document's
  D10):** no code from an LGPL source (Breeze, KIconLoader, GTK, Adwaita's
  tooling among them); a function an LGPL source is cited for is written only
  through the clean room; an Apache-2.0 port (gpui-component, gpui-pre and
  gpui-fast are Apache-2.0) keeps a file-level notice. Every derivation is
  written from the specification's description. A brief whose task touches a
  function an LGPL source is cited for says the implementer opens none of
  those sources: the specification's description and its tests are the clean
  room's. Read on 2026-11-15: the specification cites no LGPL source for a
  function these tasks touch; the rationale cites KDE's busy indicators (§1.3,
  `Loader`) only for the icon name they draw, which B.1's entries record, a
  fact and no code to port. Every implementer brief still carries the rule.
- **Hooks.** Source files are edited with the Edit and Write tools only. The
  project's hooks block invented values and panic-prone code; a blocked edit is
  obeyed, never bypassed, and blocked content is never written another way
  (shell, `sed`, a here-document, a script).
- **Code.** No `unwrap`, `expect`, `panic!`, panicking index or unchecked
  arithmetic in non-test code; no `unsafe`; no public item removed; no feature
  newer than the crate's `rust-version`. No invented value: what the
  specification measures (the tolerance, the listing) comes from the run.
- **Cargo.** Every cargo command, and every script that runs cargo, goes through
  `<scratch-keep>/widget-icons-0.7.N/tools/slot.sh` and `lowprio.sh` (T0.1): an
  agent's in the foreground, the orchestrator's long runs in the background
  (§4), every gate run under `flock <scratch-keep>/slot-locks/gate.lock` too
  (rule 2); the parity rehearsal (7d) goes through `slot.sh` alone,
  without `kde-inhibit`, and the capture (7e) through `slot.sh` alone, under
  `kde-inhibit` (§6). `build-dir` and
  `CARGO_TARGET_DIR` are never set, and no agent's own command passes
  `--target-dir` or runs `cargo clean` (the full
  check's own `cargo clean --target-dir target/package-verify`,
  `scripts/check_release.sh:1113`, is the script's). At most two cargo
  processes run at once on the machine, both lanes counted (§9: one gate and
  one `nice 19` targeted build in the other lane); in this release only the
  core sub-lane runs cargo, and the gate runs there after each merge. A
  worktree that did build (none is planned to) would build with `-p` only.
- **Reach.** No `find /` and no other search of the whole file system. Reads and
  writes stay inside the lane's worktree, Cargo's home and build folder, the
  session's scratch folder and `<scratch-keep>`; the orchestrator's other reach
  is the one *The worktree* names (T0.3's copy from `v071`, and
  `git worktree add` run from the main checkout). The installed icon themes are
  read only through the library, by the gates, unit 3's test, unit 1's
  item 3 (the stem-index test) and unit 2's item 1 (C.1's check), never by an
  agent's own tools (no `ls`, `cat` or Read of `/usr/share/icons` or another
  icon base directory). Nothing appears on the maintainer's desktop.
- **Formatting and the staged change.** Every code task's agent runs
  `cargo fmt --all --check` before it stages. The orchestrator's gate runs on
  the staged change, and `git status --porcelain --untracked-files=no` prints
  the same before and after the gate (`--commit` formats the tree in place,
  `scripts/check_release.sh:39-41`) and nothing after the commit. A reformat
  outside the task's files becomes a formatting-only commit of its own, which
  the orchestrator reads. The agent stages every file it created as well as
  every file it changed, and its report lists both; before each gate and
  before the commit the orchestrator checks §2's three commands
  (*The staged tree is the gated tree*).
- **Work.** A new test is written first and seen to fail. A changelog entry is
  one physical line. The agent runs its targeted commands in the foreground,
  each expected to finish well within 10 minutes, asks the orchestrator for
  any longer run (§4), stages its own paths by name, does not commit or push,
  and reports once: the staged files, the commit message it proposes, each
  done-criterion with its command and result line, and everything it was
  unsure of. The orchestrator gates, commits with that message, and continues
  the agent with the log's failing part if the gate is red (§2).
- **Something the maintainer must hear before a commit** (a quoted text that
  no longer stands, a measured value that stops the plan): the agent leaves
  the work unstaged and reports, and the orchestrator neither gates nor
  commits it until the stop has ended as §9 says (a quoted text whose edit
  keeps its meaning at the found text needs no answer, only the record's
  note).

**`<scratch-keep>`** is the project's durable scratch folder, which survives a
restart (the 0.7.1 plan's),
`~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/`; its
`slot-locks/` is the one lock folder every plan's tools use.
**`<cargo-build>`** is Cargo's build folder root; the briefs give its absolute
path, and no committed file names it. `<scratch-keep>` holds what this plan
keeps outside the repository: the tools of T0.1 and their `slot-locks/`, the
copies of the three documents made at the approval (§10, item 5), and the
running record, `widget-icons-0.7.N/record.md` (§12). This plan has no other
input outside the repository.
A session's scratchpad does not outlive the session: the scratchpad of the
session that wrote these documents is copied to
`<scratch-keep>/batch-2026-11-15/`, and any input a task would take from a
scratchpad is taken from there.

## 9. Limits and stop rules

- **The machine's limits, shared with the KDE lane** (S3): at most two cargo
  processes on the machine, one gate and one `nice 19` targeted build in the
  other lane, the gates of both lanes serialized by the shared
  `<scratch-keep>/slot-locks/gate.lock` every heavy run takes (rule 2: every
  gate, the GPUI Fast legs, MG's regenerations, 3a's trials, 3b's run, the
  compatibility run, 7d's release builds and the asset script; an agent's
  targeted commands take only `slot.sh` and `lowprio.sh`; §2);
  at most four agents in both lanes together, six where two of them
  run no cargo; one at a time across both lanes, each held by a shared lock
  in the `slot.sh` locks folder so that a lane waits mechanically (rule 24):
  the asset run and its capture window (`<scratch-keep>/slot-locks/assets.lock`,
  taken before the maintainer is asked for the window, 7e) and the private-X
  rehearsal (`rehearsal.sh` on Xvfb display `:97`,
  `<scratch-keep>/slot-locks/xvfb97.lock`, 7d); never two Fable agents
  (`review-hard`, `implement-hard`, `design`) at once, in either lane. The
  orchestrators of the two lanes, or the one orchestrator running both, count
  both lanes' agents and runs before dispatching.
- **Agents at once in this release:** at most one in each sub-lane and up to
  two reviewers (R4's re-review of a fix can overlap R5); T0.4's worker runs
  only beside unit 1, before the docs worktree exists. Its peak is four, of
  which only the core sub-lane's runs cargo; where the KDE lane's agents
  would take the total above the shared limit, this release's next agent
  waits.
- **A gate failed twice** by the same task is not dispatched a third time: the
  orchestrator takes the task over, re-specifies it, or sends it once as
  `implement-hard`, and says which.
- **The specification's stops**, each with nothing committed by the task that
  met it, nothing pushed after it until the maintainer has been told and,
  where the rule that names the stop asks the maintainer, has answered: a
  quoted text that no longer stands (T0.4, or any task, the header); C.1's
  check red (unit 2); `M ≥ F`, the fixture with no opaque part failing, or a
  bundled icon that `mask_drawable` does not judge `Some(true)` (unit 2, B.3);
  a condition of C.5 met (unit 3, §5). A parity difference at the capture
  is not a stop but a fault (rule 19): 7e fixes it or, for a toolkit's
  limit, excepts it with its reason and source, reviewed, and tells the
  maintainer (G.4). A stop ends when the
  maintainer has answered where the rule asks;
  otherwise the work continues once the maintainer has been told. A quoted
  text that no longer stands is found in its present form by the
  orchestrator: where the edit's meaning is unchanged, the edit is made at
  the found text and the record names it; where it is not, the maintainer is
  asked.
- **MG's stops** (rules 18, 19, 21, 22 (b)), each with nothing pushed after
  it until the maintainer has been told: the tag check printing no line or a
  different sha (also at T0.2 where the branch is made from `<prev-tag>`);
  the ancestry check failing; an untracked file the merge would overwrite
  that differs; a resolution that would change what a side's text means; a
  conflicted `Cargo.lock` whose diff against `<prev-tag>`'s lock shows more
  than this release's packages; the staged-path check finding a path the
  tag does not bring; a version outside the accepted set (item 1); a red
  warm-up whose cause is unclear (item 5); a
  path gone other than into `docs/archive/` (item 6); a failed survived
  claim whose cause is unclear (item 7).
- **A task that needs more than its parts name** (a model field, another crate,
  a decision the rationale did not take) stops, and the maintainer is asked. So
  does an upstream move during the work (a new gpui-component, GPUI Kit or GPUI
  Fast release, resvg or tiny-skia among the lock's updates, a new advisory): it
  is reported with what it costs before anything is changed for it (a version
  MG's merge brings is MG item 1's: within the accepted set VF's rework,
  outside it a stop).
- **A fix after unit 5** that a review, CI, the full check, the rehearsal or
  the asset run demands is made by an `implement` agent, reviewed as in §3, and
  passes the done-criterion of the unit whose files it touches. A fix to unit 4's or unit
  5's files after 6b updates 6b's dated lines and item 8's closing line in the
  same series of fixes, so that they name the commit that now holds what they
  state. A fix during unit 7 follows rule 15 (§6; its unstaged exception,
  §2). The maintainer is told what stopped it, what was changed, and whether
  another capture window is needed.
- **The capture window** is asked for once, when 7c to 7d hold.

## 10. The maintainer's part

**With the approval of this plan, before any task** (each item says whether
it is settled or put for approval):

1. **The number and the place.** For approval: the version number, `0.7.N`
   (the KDE document's D0; 0.7.5 in the settled order). Settled on
   2026-11-15: the release order (D10, *The releases after v0.7.1*, approved
   with S3), so `<prev-tag>` is `v0.7.4`, the KDE steps 4 and 5's; D10's
   place of KDE step 1 (in v0.7.1); and how the branch starts and where it
   runs (S2, S3; T0.2, MG). `<lane-pred-head>` is v0.7.1's gated head, since
   the second lane holds this release alone (S3 as amended; rule 18).
   The units start on a tree without what `<prev-tag>` brings (the
   branch starts from v0.7.1's gated head, the release before this one in its
   lane): v0.7.1's close-out, v0.7.2 (egui 0.37), the KDE steps 2 and 3 and
   the KDE steps 4 and 5. All of it comes in with MG's merge of `<prev-tag>`,
   and E.14's claims are checked right after it as claims that this
   release's results survived the merge (MG, item 7): a failed claim that
   is a placement or a count is fixed by a `worker`, one that is a code text
   by its unit's author (an `implement` fix, reviewed within R-MG, covered by
   the warm-up gate), and only one whose cause is unclear is a stop. S2's
   merge in place of its rebase is settled (item 7). Should the maintainer
   change the settled order later, every text that says what `<prev-tag>`
   brings is amended then: this item's own list above; T0.4 (a); MG's *What
   it brings*, its items 1 and 7 and the paragraph after item 7; 3a's item 5
   and unit 5's item 3; and in the specification its header, E.14 and C.4
   item 8's last sentence ("The Basic page's Edit tool button that the KDE
   steps 4 and 5 add …").
2. **The pull request** (settled 2026-11-15: the maintainer approved
   opening a draft pull request at a branch's first push, rule 9). The
   orchestrator opens it from `v0.7.N-rc1` to `main` at the first push. Its
   title and description are drafted by the orchestrator and shown to the
   maintainer before it is opened (changing them later stays the
   maintainer's, §7). CI then checks every push of §4.
3. **What this plan adds to the specification:** T0.4; MG and its
   survived-the-merge check; task V; the parity rehearsal of 7d; three
   departures from the batch's order, for approval: 7d beside CI (§6; its
   builds wait for 7c's full check under the gate lock);
   6a's and 6b's `--quick` made at the merge, on the core worktree, rather
   than in the docs worktree, which runs no cargo (§2, *The cadence*); and MG
   after 6b and the docs merge, so that 6b never waits for the tag; F.6 in the asset
   commit; the deletion of the sheets at the close-out (§7), on the
   maintainer's word; the `eprintln!` of `M` and `F` in the tolerance test
   (unit 2, item 3); the split of unit 6 into 6a and 6b
   (F.1 item 2 and F.5 items 1 and 2 in 6b); both GPUI Fast legs by hand at
   every gpui unit by rule 11's test, units 2, 3a, 3d, 4 and 5 (§4; G.1 names
   the connector's leg at units 2 and 4); the ignored stem-index test run by hand at
   unit 1 (item 3); and the resvg version in
   `ONE_COLOUR_TOLERANCE`'s doc read from `Cargo.lock` at the run, where B.3
   writes `0.48.1`.
4. **Unit 3's reads.** Approving this plan permits unit 3's reads of the
   maintainer's installed icon themes through the library, in 3a's trial runs
   and 3b's run (C.2; §5 announces them before they start).
5. **The copies.** At the approval the orchestrator copies the three approved
   documents into `<scratch-keep>/widget-icons-0.7.N/docs/`, the fallback T0.3
   takes them from if `.claude/worktrees/v071` no longer holds them.
6. **The build folder.** Where `lane2-a` is new (§1), its build folder
   (`<cargo-build>/<hash>`, tens of GB) is named to the maintainer when T0.2
   makes it. This release is the second lane's last (S3 as amended), so its
   close-out removes `lane2-a` and that build folder (§7), as the
   maintainer's build-folder rule asks (a separate build folder is deleted by
   the agent that made it before its task ends); a fix asked for after that
   builds in a new worktree from nothing (§7).
7. **The batch's decisions, settled.** Settled by the maintainer: S1 to S5,
   S3's amendment and S6 of `docs/todo_v0.7.1_batch-speed-rationale.md`;
   and, on 2026-11-15:
   - S2's amendment, the orchestrator's of 2026-11-15: a merge, not a
     rebase (task MG);
   - the batch's working rules, rules 1 to 26 of that rationale's Appendix,
     which this plan applies throughout;
   - the three re-readings those rules name: rule 20's of S2's branch point
     (the gated head is the documents unit, the last unit before MG, gated
     by its `--quick`), which reaches this release through v0.7.1's gated
     head, which T0.2 reads, while this release, the second lane's last,
     writes none (rule 20); rule 7's branching from `<prev-tag>` where that
     tag exists when T0.2 runs (no MG); and rule 9's "green" before the
     asset run (CI green, and the full check's only failure the provenance
     stamp);
   - opening a draft pull request at a branch's first push (item 2);
   - the gate-speed specification's answers (O1a and O1b no, O2 (e), O3's
     keyed option, decision 4 (y)), D10 (a), and the release order (approved
     with S3; item 1).

   What remains for approval with this plan: the version number (D0, item
   1), its own additions and departures (item 3), and unit 3's reads of the
   installed icon themes (item 4).

**During the work, the maintainer is told or asked:**

- before 3a's trial runs, the measurement's announcement (§5); at the run,
  before 3b's run starts, the packages' versions, which the orchestrator writes
  in 3c;
- a stop of §9, condition 7's two figures above all (open point 1);
- the tolerance measured (`M`, `F`), and the verdicts when none stops;
- after the verdicts, whether the sheets were seen or are not needed, which
  lets §7 delete them;
- a red CI run, and an upstream move;
- a parity difference at the capture and how it was settled by rule 19
  (7e).

**Near the end:** what the new screenshots show, and one capture window (open
point 2).

**After 7f:** whether to merge, tag and publish, and the pull request's title
and description. The next step, serving SF Symbols and Segoe Fluent, is the
backlog entry F.5 item 1 opens.

## 11. Time

Measured at the 0.7.1 release on this machine and at GitHub (the archived
0.7.1 plan, §8): the full `check_release.sh` about 23 minutes; the asset script
30 to 37 minutes, of which 4 to 6 need the desktop; a CI run 21 to 27 minutes.
`--commit` took 5 to 12 minutes per commit, and 29 minutes cold, in earlier
measurements
made before the 16-job cap (the batch-speed rationale's Appendix, its
opening; not measured again here), which is why
every `--commit` is a long run (§4) and the first warm-up gate (T0.3's, or
T0.2's where there is no unit 0) times the first. Under S1 most units run
`--quick` instead (§2, *The cadence*): three `--commit` runs at the units'
ends (3a, unit 5 and V's warm-up) and one or two other warm-ups (the first,
at T0.2 or T0.3; MG's, where there is an MG; X after MG takes `--quick`),
against
eight in the cadence this plan had before, plus any `--commit`-gated fix
after MG. Beside them, both GPUI Fast legs run as long runs at five units
(2, 3a, 3d, 4 and 5, §4), and always with MG's warm-up (rule 21 (5)).
Not measured: the GPUI Fast legs' run time, `--quick`'s
run time (the first is timed into
the record), `--commit`'s run time under the cap on this branch, the first build in a new
worktree, unit 3's run (it reads every installed theme and times five runs of
every step), and the tasks' own durations; the first runs give them, and the
record (§12) holds them. The longest chain is the core lane's: units 1 to 5,
the merge, V, 7b to 7f, plus the stop's verdict, any review fix, and the wait
for the capture window.

## 12. Record

The running record lives outside the repository, in
`<scratch-keep>/widget-icons-0.7.N/record.md`: only the orchestrator writes it,
and agents report to the orchestrator. 3c copies into this section the entries
up to the run, and 7f the rest. 7f's own full check and CI run are reported to
the maintainer, not held here (§7). No absolute path is written here: the
placeholders of §8 stand for them.

- T0.2: the worktree chosen (reused or new) and its build folder,
  `<cargo-build>/<hash>`; the hook trial refused `<yes | no>`;
  `<lane-pred-head>` `<sha>`, read from v0.7.1's gated head (its record's
  line "the lane's gated head for the next release"), or "made from
  `<prev-tag>`, no MG", with the tag check's two shas (the first field of
  `git ls-remote origin 'refs/tags/<prev-tag>'` and `git rev-parse <prev-tag>`),
  the `Cargo.lock` comparison against `0fa28027` and the path check's result
  and re-point, if any.
- T0.4: `<all found | the texts not found>`.
- Unit 2: C.1's check `<passed | red | skipped>`; `M = <…>`, `F = <…>`, resvg
  `<version>`, `<date>`; the deviations of the fixture with no opaque part
  `<…>`.
- Unit 3: the run's commit `<sha>`, date `<date>`, duration `<…>`, at low
  priority with no other cargo process; conditions 1 to 7 `<met | not met>`,
  each with the rows that decide it; condition 7's medians per step, both sides
  `<table>`; the maintainer's word `<…>`; D.13 `<built | not built>`.
- Unit 5: the tests E.2 or E.3 turned red beyond E.10's table, each with its
  edit: `<…>`.
- Reviews: who reviewed each commit, and the must-fixes `<…>`.
- 7c: the full check's failures and warnings `<…>`, the expected
  `cargo outdated` list among them; the CI runs up to 7f's push `<ids>`.
- Times (§11): the first `--commit` (the first warm-up gate, at T0.3 or at
  T0.2's end) `<…>`, cold or warm `<…>`; every later warm-up gate and why it
  ran `<…>`; the first `--quick` `<…>`; a new worktree's first build `<…>`
  (where one was made); and each task's duration `<…>`.
- MG: the tag check's two shas `<…>`; the ancestor check before the merge
  `<…>`; the untracked files the overwrite check removed `<…>`; the HEAD
  before the merge `<sha>`; the merge commit `<sha>` and its two parents;
  `Cargo.lock`'s resolution, if it was conflicted `<…>`; each conflict and
  its resolution, by both sides' quoted texts; the prose-count hunks resolved
  to `<prev-tag>`'s side `<…>`; the marker check and the staged-path check
  `<…>`; any generated file taken as pending `<…>`; the toolkit
  versions before and after, VF's re-readings and rewritten forms ("none
  needed" where nothing changed) `<…>`; R-MG's review of the resolutions and
  VF's commits `<…>`; the warm-up gate's result `<…>`, and every later
  warm-up run with the fix it gates `<…>`; a re-measured
  tolerance `<…>`; the path check, each path with "tracked", "re-pointed" or
  "stop" `<…>`; VF's rewrites, the expected differences from `<merge>^1`
  `<…>`; the survived-the-merge table (replaced texts absent,
  anchors and new texts present, each E.14 claim, each count recounted in
  the files either side changed, the five checkers, F.3's and F.4's
  placement), each with its line or "not found" and each fix commit `<…>`.
- Cadence (S1): any failure a `--commit` found that a `--quick` before it had
  not, with the commit that brought it and the fixing commit `<…>`.
- T0.3: for each of the three documents, the commit that added it
  (`git log --diff-filter=A --format=%H -- <path>`, rule 26) `<sha>`, or
  "untracked: copied in unit 0" with its `cmp` result; unit 0's commit
  `<sha>`, or "no unit 0"; the path check's re-points, if any `<…>`.
- Long runs: each run's folder, `<logs>/<task>/run-<n>/`, with its command and
  its `status` `<…>`.
- Departures from this plan: `<…>`.
