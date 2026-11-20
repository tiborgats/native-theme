# 0.7.2: issue #4's leftovers (the three looks and the smaller Breeze details) — implementation plan

- Rationale: [`../issues/0004_kde-leftovers-rationale.md`](../issues/0004_kde-leftovers-rationale.md),
  E1–E14. Specification: [`todo_v0.7.2_kde-leftovers-spec.md`](todo_v0.7.2_kde-leftovers-spec.md)
  ("the spec"): its decisions S1–S12 and Parts A–E, each in the units U1–U4.
- Part of 0.7.2 (D0 (a)): the version stays 0.7.2, and 0.7.2's close-out is made again after these
  units, once. Written 2026-11-20; not yet approved.
- `<record>` = `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.3/`, this
  work's files in `<record>/leftovers/` (the trial's in `<record>/leftovers/trial/`). `<tools>` =
  `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan/tools/`,
  0.7.2's wrappers, used as they are.

## 1. Where and from what

- One worktree, `.claude/worktrees/lane2-b`, branch `v0.7.2-rc1`, from the head that holds the
  approved rationale, spec and plan. The units are commits on that branch: no new branch, no merge,
  no version step. The branch is pushed by explicit refspec at each `--commit` gate that passes, and
  each push's CI run is read (a red run emails the maintainer).
- Nothing is built yet. What the spec could not settle by reading is listed in its *Not verified
  while specifying*; the trial T-L1 is the one result that could refute a design (S4, S5 in gpui)
  and runs first.
- The orchestrating session runs the build on Opus 5.5 at high effort. Fable runs only in the
  agents this plan names: §4's `review-hard`, and at a stop §6's `design` or `implement-hard`.

## 2. Units

The spec's four units are built in two passes (the spec's correction 8): **L1**, U1 (the looks,
behind the trial, never cut), and **L2**, U2, U3 and U4 (the smaller details). Each pass costs a
block of three connector builders, their three reviews, a rehearsal (three release builds) and a
`--commit`; built as four, the details would pay that three times for 14 fields. A connector
builder's share of the spec in L2 is its part's U2–U4 sections, about one and a half times its U1
sections. L1 keeps its own gate and push: the push puts the looks under CI's Windows and macOS
legs before L2 builds on them, and L1's must-fixes reach L2's builders.

Each row is one commit per agent's task; a review fix is a commit of its own, covered by the next
gate.

| Unit | The spec's parts | Agent | Gate after it |
|---|---|---|---|
| T-L1 | C.1.3's trial: two `#[gpui::test]`s, a list row's box under group hover and a menu row's box hidden under group active (in its hover-lit and its state-lit form), kde-breeze light, gpui-pre and GPUI Fast | `implement` | its own two tests on both backends; result and patch into `<record>/leftovers/trial/`, nothing committed, and the tree left clean (the patch is the record's; L1a builds in the same tree next). A failure stops L1 and goes back to the spec (S4, S5) |
| L1a | U1 of Parts A and B (the looks' seven fields, kde-breeze, the reader, the facts rows) | `implement` | `cargo test -p native-theme` with the gate's features; the five documentation checkers. Not `--quick`: the connectors are red until L1b–L1d |
| L1b, L1c, L1d | U1 of Parts C (from T-L1's patch), D, E | three `implement` at once | none of its own |
| — | after L1b–L1d | orchestrator | R beside one `--commit` with the GPUI Fast legs and the five documentation checkers; push; CI read |
| L2a | U2 and U3 of A and B (E10 last, unless cut) | `implement` | as L1a |
| L2f | U4 of A and B (the flag, the rounding pass, kde-breeze's sizes) | `implement`, a fresh agent after L2a: U4 is other work (validation, its narrow `review-hard`) and nearly as long in Parts A and B as U2 and U3 together | as L1a |
| L2b, L2c, L2d | U2, U3 and U4 of C, D, E (E10 last in each, unless cut); first, as a commit of its own, L1's must-fixes for that crate | three `implement` at once | none of its own |
| L5 | Part B's *Outside the code*, written once for the units built (CHANGELOG, ROADMAP, READMEs' counts, the rationale's status line, `docs/todo.md` with E14's additions) | `worker`, beside L2's connector block (it runs no cargo) | the five documentation checkers; goes into L2's `--commit` |
| — | after L2b–L2d | orchestrator | as after L1b–L1d |
| K | 0.7.2's close-out again: the archive (rationale, spec, plan into `docs/archive/`), the compatibility run, the push, the full check beside CI, the asset run with its capture window, the capture check, the assets' commit, PR #11's description; at its end `~/.claude/scripts/sweep-target.sh` on the worktree (the global rule for a multi-task build) | orchestrator | the full run twice: the first before the asset run, so that nothing found later voids the capture window; the second on the assets' commit, the release verdict (0.7.2's close-out: the first fails only on the assets' stamp) |

Before each block of three, the orchestrator makes the pass's edits of the shared files from S12's
list and the parts' texts, uncommitted, so that the three builders start from the same lists. A
builder stages nothing outside its own paths; the orchestrator commits the shared files by path, as
a commit of the block, after the three have reported. A builder of L2 goes straight to the figures
the spec gives for the last unit it builds; a test states the built value.

**The rehearsal R** (0.7.2's `<tools>/rehearsal.sh`, read by `scripts/check_showcase_parity.py`)
runs once per pass after its connector block, on release builds of the three showcases (it runs
`target/release/examples/showcase-*`; the three builds go through `slot.sh`). R and the `--commit`
run side by side, the two cargo processes the project allows, with no edits in the tree. Expected
new keys: the leaves the pass adds to the listed elements. A difference R finds is fixed after the
gate: an exception with its reason in a `[[parity.group]]` of `docs/showcase-exceptions.toml` (S12)
by the orchestrator; a code fix in L1 through that crate's L2 brief, in L2 as a must-fix (§4). The
next gate covers it: L2's, or K's full run.

**On an overrun** (the rationale's §4; the project's rule: stop and re-scope at the first large
one): E10 is cut first, then whole units from the end, U4 first; U1 is never cut. A unit not begun
leaves nothing behind; the spec's and the docs' text for a cut unit or for E10 move to the backlog
in L5.
- **The cut is decided once, at L2a's start**, before any of L2's fields exist, against L1's time.
  The budget is the rationale's §4: one to two days for the smaller details (14 fields, about one
  and a half days at its rate of nine a day). At L1's measured rate (its seven fields, from T-L1's
  start to its gate) the 14 fields fit the two days where L1 took at most one day. Where they do
  not, E10 is cut; where the rest still does not fit at that rate, U4, then U3.
- **Cut**, U3 is built without E10 in every part: no `combo_box.open_*` fields, reader lines,
  preset lines, `PF` rows, connector code, contract rows or element-list line (B.0's two rows,
  C.3.4, D.9, Part E's E10, S12's `basic.drop_down.trigger` line), and the figures the spec gives
  with E10 become: 19 fields (U3 6), 12 colours, the resolved model +19 leaves, the KDE reader's
  colour leaves 46 → 51 → 53 → 60, egui's leaves 534 → 553 with inputs `+[0,0,2,0]`, the READMEs'
  59 colours. E10's text goes to the backlog in L5.

## 3. Tests and gates

- During a task: the tests the spec names for it and the ones its edits reach, through
  `<tools>/slot.sh <tools>/lowprio.sh cargo test -p <crate> … <names>`; a showcase's tests through
  `scripts/run_in_runner_env.sh`; tests that read painted primitives in the GPUI Fast form too. A
  test the spec says fails without the change is seen failing first.
- Before each task's report: `scripts/check_strict_panic.sh` for the crates it changed; from L1b
  on, `git grep -n trial_` over its paths finds nothing.
- At each gate `<tools>/gate.sh [--fast] <the worktree's path> <name> <args>` (its first argument
  is a path, `cd "$wt"`): two `--commit` gates, one per pass, each beside its R
  (7 to 21 minutes each, measured in 0.7.2), and the full run twice in K (about 23 minutes each).
  L1's is a `--commit`, not a `--quick`: every crate builds on `native-theme`, which L1 changes, so
  `--quick`'s step cache would skip nothing.
- The whole suite and the release checks run in K only. Before K's compatibility run and before its
  asset run the session's own rust-analyzer is stopped.

## 4. Reviews

A reviewer gets the commit and the spec's parts (with S1–S12), never the author's report, and reads
files as they stand at the commits under review (`git show <commit>:<path>`), not in the working
tree, where the next task may already be editing. At most one Fable agent at a time. The passes run
in this order, with these agents at once:

1. L1a: one agent.
2. L1b–d and beside them L1a's review: four agents (the builders share the two cargo slots; the
   reviewer runs no cargo). A must-fix from L1a's review edits `native-theme` while the connectors
   compile against it, so it lands after the three have reported and before R and the gate, which
   re-run what it reaches.
3. R beside the `--commit`, the orchestrator's; then the push. Beside them the block's three
   reviews: none runs cargo.
4. L2a starts once R and the gate have passed, not waiting for the three reviews: their must-fixes
   go into that crate's L2 brief as its first commit (a must-fix in `native-theme`, if one comes,
   into L2f's brief), so no agent is started for them and nothing waits on them.
5. L2a, then L2f; L2a's review runs beside L2f.
6. L2b–d, and beside them L5, L2f's `review-hard` and, if still running, L2a's review: at most six
   agents, three without cargo, within the project's limit (four, six with two that run no cargo).
   L2a's and L2f's must-fixes land as in step 2.
7. As step 3. The block's must-fixes then go in as commits, each with the tests it reaches: one
   `implement` agent per connector that has one (at most three, on disjoint crates, sharing the two
   cargo slots), all before K's compatibility run, whose stamp hashes the connectors' sources at
   `HEAD`. K's full run covers them.

| Commit | Reviewer | Why |
|---|---|---|
| L1a, L2a | `review` | new soft fields, registry and rules rows, the reader's mixes against Part B's value table, the facts rows |
| L2f's validation hunk (the rounding pass in `validate_with_dpi`, `qt_whole_px`, the text-scale size check) and its walk test (`every_resolved_font_size_is_whole_under_the_flag`) | `review-hard`, narrow | every font size of every theme passes through the pass, and the walk is what holds a later font field to it (S3) |
| The rest of L2f | the orchestrator | expected values and two tests, a few dozen lines |
| L1b–L1d, L2b–L2d | three `review` agents at once per block | each connector's per-state rules against S4–S10 and its part, gpui's claims at their lines, the regenerated leaves files, the fallbacks; in L2 also U4's expected sizes and doc texts |
| L5, K's commits | the checkers, then the orchestrator reads the diff | texts the spec fixes |

Every reviewer also checks that no new code transcribes an LGPL or GPL source.

## 5. Briefs

0.7.2's form: the brief quotes the unit's sections of its part and the S-decisions that bind them,
names the files, and says: no `find /`, no sub-agents, never bypass a hook, long commands in the
foreground, one report at the end; the readers are written from the spec's text alone (Breeze's and
Qt's sources are not opened; D10's clean room). In a block of three: other agents edit other folders
of the same tree at the same time; change, format (`cargo fmt -p <crate>`) and stage nothing outside
your own paths, which the brief lists; the shared files are edited already.

## 6. Stops

- T-L1 fails: L1 stops; the orchestrator re-specifies gpui's receiver (S4, S5) and asks a `design`
  agent only if the options are not settled by a second trial.
- An anchor not found as the spec quotes it: the orchestrator finds its present form; where the
  edit's meaning is unchanged it is made there and the record names it, else the work stops.
- A decision neither the spec nor the record makes; a task needing more than its parts name; a new
  upstream release or advisory, reported with its cost before anything changes for it.
- A unit that fails the same gate twice is not sent a third time: re-specified, or sent once as
  `implement-hard`.

## 7. The maintainer's part

- The approval of the spec and this plan.
- Told, not asked: S6 (egui draws no pressed menu item) and the corrections the spec lists. L5
  writes 0.7.2's ROADMAP heading as Part B gives it, which the approval of the spec covers; the
  issue #4 follow-up comment and the CHANGELOG's date at tagging are the maintainer's.
- In K: the capture window, once, 4 to 6 minutes, asked for when everything before it holds.
- After K: the merge of PR #11, the tag `v0.7.2`, the publication and the GitHub release.

## 8. Cost check

- **What could refute the design, and is it first.** gpui's receiver for the row and menu outlines
  (S4, S5): T-L1, first, before any model change. Everything else has a receiver read in the
  toolkits' sources at their versions, and the values are Breeze's measured or recomputed ones. What
  waits for T-L1: everything. L1a does not run beside it: T-L1 compiles `native-theme` in the same
  tree that L1a edits (one builder per tree), and a second worktree would be a whole extra build.
- **Built twice:** nothing. T-L1's test is where L1b's starts; the rationale's 21 fields are added
  once, each in its unit.
- **Paid per pass, not per unit:** the connector block, its reviews, R and the `--commit`: two
  passes instead of four (§2): 9 builder agents instead of 16 and 9 reviewers instead of 13, two R
  and two gates fewer, and one must-fix round instead of up to four. L1's must-fixes ride in L2's
  briefs, so no agent is started for them and L2 does not wait on L1's reviews.
- **In parallel:** each block of three connector builders (two cargo commands at a time through
  `slot.sh`) beside the core reviews and, in L2, L5; R beside each gate; the block's three
  reviewers, who run no cargo, beside its R and gate and on into the next core build (§4: at most
  six agents, three without cargo).
- **What each agent reads:** its pass's sections of its part, S1–S12, the files named; not the
  rationale, not this plan, not the other parts.
- **Tests per round:** the task's own; one `--commit` and one R per pass; the full suite twice, in
  K, each for a reason of its own (§2, K).
- **Time (an estimate, not measured):** 0.7.2's earlier folds took 1 h 40 to 1 h 55 each for three
  to six fields with one connector block. Here L1, with the trial, about two and a half to three
  hours; L2, with its two core builders in a row and a block about one and a half times L1's,
  three and a half to five; six to eight hours, then K, about two and a half hours.
- **Compaction:** after the approval, before T-L1; after L1's gate; before K; and wherever the
  conversation passes about 300 k tokens.
