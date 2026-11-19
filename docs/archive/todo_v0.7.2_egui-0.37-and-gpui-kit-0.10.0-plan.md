# v0.7.2 (egui 0.37 and GPUI Kit 0.10.0): implementation plan

Status: written 2026-11-16; reviewed once and checked once that day, revised after each, and
revised on 2026-11-17 when the specification
(`docs/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-spec.md`) was written and when its author read
both again. Approved by the maintainer on 2026-11-17; being built.

It builds the two rationales, `docs/todo_v0.7.2_egui-0.37-rationale.md` (decisions E1 to E20) and
`docs/todo_v0.7.2_gpui-kit-0.10.0-rationale.md` (decisions T1 to T12), under the batch's working
rules (`docs/todo_v0.7.1_batch-speed-rationale.md`, the appendix; cited as "rule N").

Names, as in the specification: `CE`, `CW`, `SE` are the egui connector, the egui widgets crate
and the egui showcase; `CG`, `SG` the gpui connector and its showcase; "the source test" is the
widgets crate's `the_painted_widgets_are_still_needed` (the egui rationale's "T7"), and "T7" is
always the GPUI rationale's decision.

## 0. How this plan saves time, and what it does not save on

Nothing here lowers a gate, a review of code or a fact's standard. The savings:

1. **A specification of one section per unit.** The specification
   (`docs/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-spec.md`) states what the rationales decided,
   unit by unit, with the choices they left to it; a unit's brief and its reviewer read that
   unit's section. The rationales are opened only when a unit stops.
2. **The trials' code is reused, not rewritten.** Z1 has cut the trial patches into one patch per
   unit (§2.1); a unit begins with `git apply`. The patches carry interim code, which a check
   keeps out of every commit (§3).
3. **Two lanes at once.** The moves touch different crates, so they are built in two worktrees by
   two builders, which is what the machine's two cargo slots allow (rule 8). Reviewers and the
   documents' `worker` run beside them without cargo. What this shortens is v0.7.2's date, by the
   shorter lane (about 4.5 to 6.5 days by §4's estimates), not the batch's total work.
4. **Nine code units where the rationales list seventeen**, cut by the files an agent has to
   read; a correction to a file's comments goes with the unit that changes that file. Fewer
   units are fewer agent starts, gates and reviews.
5. **Tests only where the change reaches.** An agent runs the named tests of the crate it changed
   (rule 1). At a unit's end `--quick` runs what `--commit` would for the crates the change
   reaches and skips every test step whose inputs did not change (the gate-speed port, commit
   `30a53e29`), so an egui unit does not re-run the gpui and iced tests, and the reverse. Two
   units are exceptions and say so: F1 changes the core crate, and E-C changes
   `docs/showcase-exceptions.toml`, a file every step's key reads. `--commit` runs five times in
   the release (§5), the full check once, beside CI (rule 9).
6. **The wide gates are placed where they are due anyway.** The one change to the core crate
   (adwaita's check-box radius, E7) is the trunk's first code unit, before the lanes fork, where
   the worktree's build is new.
7. **Each mechanical job once, by script.** The upstream citations are re-pointed once, in the
   GPUI lane's first unit, from Z1's line maps: 385 of the 386 failing citations are pure line
   moves. The documents are drafted beside the lanes (D). One asset run and one close-out serve
   both moves, and a rehearsal without the desktop (R) keeps that asset run from being made
   twice.
8. **No Fable agent is planned.** Neither move touches a resolver, validation, a runtime,
   concurrency or `unsafe`, so reviews are the orchestrator's (small diffs) or a `review` agent's.
   A design question that comes up goes to one `design` agent.

What limits the speed and cannot be planned away: the machine runs one heavy run and one targeted
build at a time, with four agents, six where two run no cargo (rule 8), and those limits count
every lane. Every later tag of the batch waits for v0.7.2's (rule 17), so this plan uses the two
cargo slots in full from F1 to R; the other lanes' work that needs no cargo (their documents'
last checks) can run beside it within the six.

Two more ways to an earlier date were weighed and are not in the plan. Starting a lane before
v0.7.1's gated head exists: only the egui lane could, since v0.7.1's remaining GPUI units edit
the showcase files and citations the GPUI lane re-points; the longer lane sets the date, so the
most it could gain is the difference between the two lanes, about half a day to a day and a half
by §4's estimates; against that stand an amendment to the batch's rule on the branch point, a
merge with everything v0.7.1 still changes, and a builder and reviewers taken from v0.7.1, which
every release waits for. How often v0.7.1's build leaves a cargo slot free was not measured.
Raising the machine's limits (the wrappers hold builds to 16 of the 32
threads and two cargo processes, for the maintainer's work at the same desk): that is the
maintainer's to set, for hours when the desk is free, and nothing here depends on it.

## 1. Places

- `<prev-tag>` is `v0.7.1`. `<lane-pred-head>` is v0.7.1's gated head (rule 20), read from its
  record at P2. The branch `v0.7.2-rc1` is made from the tag where it exists at P2 (no MG), else
  from the gated head (MG later; rule 7, and S3's paragraph on v0.7.2).
- The trunk and the egui lane: worktree `.claude/worktrees/lane2-b`, branch `v0.7.2-rc1`.
- The GPUI lane: worktree `.claude/worktrees/v072-gpui`, branch `v0.7.2-gpui`, made from the
  trunk at F1's commit and merged into it once (M).
- The documents lane: worktree `.claude/worktrees/v072-docs`, branch `v0.7.2-docs`, made at F1's
  commit; no cargo runs there, so it has no build folder.
- The two code worktrees each have a build folder under `/run/media/tibi/work/cargo-build/` (the
  trials measured 16 GB for the egui crates and 32 GB with the gpui tests); the orchestrator
  deletes the GPUI lane's with its worktree after M and the trunk's at the end of the close-out.
- `<work>` is `.superpowers/sdd/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan/` in the main
  checkout (`tools/` with the copies of `slot.sh` and `lowprio.sh`, the gate logs, the copy of
  v0.7.1's `rehearsal.sh`). `<record>` is the plan's folder in the maintainer's `scratch-keep/`,
  with `record.md`, the ledger, and `z1/`. `<locks>` is the batch's shared lock folder.
- The trials' records: `scratch-keep/egui-037-trial/`, `scratch-keep/gpui-kit-0.10.0-trial/`,
  `scratch-keep/v072-open-trial/`.

## 2. The units

### 2.1 Before the code

| Unit | What | Agent | Gate |
|---|---|---|---|
| Z1 | Done on 2026-11-16, without cargo; its files are in `<record>/z1/`. One patch per unit (`E-A1`, `E-A2` with the three follow-up test fixes as excerpts, `G-A`, `G-B1`, `G-C-tooltip-test`), none with `Cargo.lock`, each applying to v0.7.1's head of that day; `sources.txt`, the one source of each file; `stand-ins.txt`, the interim lines the patches carry, and `stand-in-check.sh` (§3); a line map for each of the 53 upstream files the failing citations name, with the count: 385 of 386 moved, one changed (`breadcrumb.rs:144`), and two moved ranges in upstream's `checkbox.rs` whose inside changed; the lint list's and the fallback-stack test's lines at gpui-fast 0.1.16, none changed in text. The re-pointing tools of the earlier upgrades are fixed to their own versions and checkout and are not used (`citation-tool.txt`) | `worker` | none |
| P1 | The tools, the record, the lock folder checked | orchestrator | none |
| P2 | The branch and the trunk's worktree (rule 18's checks; the tag check of rule 21 where the branch is made from the tag). The tree's `Cargo.lock` compared with `e866ea6c`'s, the trials' base, for the crates the two moves name (rule 19): a difference is read before F1. Z1's patches checked again with `git apply --check`. A look at crates.io: an egui 0.37.1 has its changelog read by the orchestrator, and the floor and the lock stay at the measured 0.37.0 unless it fixes something this move meets (the record says which); a newer GPUI Kit is the next upgrade | orchestrator | none |
| P0 | The two rationales, the specification and this plan added with their status lines (rule 26) and the path check (rule 22 (b)) | orchestrator | the five documentation checkers and `--quick`, timed (rule 3): the worktree's first, cold run |
| F1 | adwaita's check box; the GPUI and documents worktrees are made from this commit: spec §F1 | `implement`; the orchestrator reviews | `--quick`; a core change, so every connector's tests run |

### 2.2 The egui lane (trunk worktree)

| Unit | What (decisions) | Agent, review | Gate |
|---|---|---|---|
| E-A | The move to a green tree, one commit: A0 (first, a few hours, reported once; its agent goes on as A1 once the orchestrator has read the report, unless its context has grown past the limit for a resumed agent), A1 (`CE/src`, `CW/src`, the manifests), A2 (`SE`): spec §E-A | `implement`: one agent for A0 and A1, a second for A2; two `review` agents, one on A1's paths and one on A2's | `--commit` (the lock) |
| E-B | The mapping: the «input» cell, the accessors, the records of E5, E6, E7 and E20: spec §E-B | `implement`; review by size (§3) | `--quick` |
| E-C | The rest of the showcase, the parity reasons, the egui READMEs and E11's sentences: spec §E-C | `implement`; review by size (§3) | `--quick`; the exceptions file is in every step's key, so every test step runs |

### 2.3 The GPUI lane (its own worktree)

| Unit | What (decisions) | Agent, review | Gate |
|---|---|---|---|
| G-A | The move to a green tree; first step the GPUI-only lock (GPUI Kit 0.10.0 with egui 0.36.2, not measured by a trial): spec §G-A | `implement`; the orchestrator reviews the code, the gates the citations | `--commit` with the GPUI Fast legs (the lock), timed (rule 3) |
| G-B1 | The tab bar and the check box: spec §G-B1 | `implement`; review by size (§3) | `--quick` with the GPUI Fast legs |
| G-B2 | The slider, switch, progress bar and widgets module, right to left, with the capture: spec §G-B2 | `implement`; review by size (§3) | `--quick` with the GPUI Fast legs |
| G-C | The gpui showcase's new parts and statements, and the gpui README: spec §G-C | `implement`; review by size (§3) | `--commit` with the GPUI Fast legs (the lane's third code unit since G-A, rule 6) |

In every GPUI unit, a change that moves a line of the connector's own files that a claim cites
re-points that claim in the same commit (S6's fourth gate); that is why the comment corrections
ride with the unit that owns the file.

### 2.4 The release

| Unit | What | Agent | Gate |
|---|---|---|---|
| D | The root documents, drafted in the documents worktree while the lanes run and finished when G-C and E-C are committed: spec §D | `worker`; the orchestrator reads the diff | none of its own |
| M | The GPUI lane and the documents lane merged into the trunk, once each, after E-C and G-C and with no review open. `Cargo.lock`: the trunk's side, then `cargo update -p gpui-component --precise 0.10.0`, the joint lock the trials measured; `cargo metadata --locked`. A conflict in Markdown, `docs/showcase-exceptions.toml` or `docs/showcase-elements.toml` is resolved by rule 21's resolution rules; one in a source file is a stop (the lanes share none) | orchestrator | with R |
| R | The parity comparison's layout half without the desktop, as v0.7.1's KR (`docs/archive/todo_v0.7.1_kde-open-items-step1-plan.md`), on the private X server under `xvfb97.lock` (rule 24); its release builds are K's: spec §R | orchestrator; `implement` for a fix | `--commit` with the GPUI Fast legs, once, over M's merges and R's commits: the release's last code commit (rule 6); then the five documentation checkers, and that tree is the gated head's |
| MG, X | Only where the branch was made from the gated head: rules 17 and 21 with v0.7.1's tag; X is the recount and the five documentation checkers. No VF is expected: the tag changes no version of a cited crate (rule 25) | orchestrator | the warm-up `--commit` with the GPUI Fast legs (rule 21 (5)) |
| V | The version 0.7.2 (rule 14), with rule 22 (c)'s search: spec §V and K | `worker` | `--commit` |
| K | The close-out as the batch has it: the compatibility run, the push and the draft pull request, the full check beside CI (rule 9), the asset run for the three showcases with its capture window (rule 24), the four documents archived (rule 22; the batch rationale stays, v0.7.2 not being the batch's last release). The record gets one line for the later plans' last checks: the lock they will meet at their merge holds egui 0.37 and GPUI Kit 0.10: spec §V and K | orchestrator | the full run |

### 2.5 The choices the rationales left open

They are decided in the specification's unit sections.

## 3. How a unit runs

- **The brief** is the unit's section of the specification plus the standing rules (the
  specification's Part A): spawn no sub-agents; long commands in the foreground, one report at
  the end; no search of the whole file system and nothing outside the named paths; no content a
  hook refused written another way; no `unsafe`, no panic-prone code, no code from LGPL sources.
  A test that fails because egui's defaults moved gets its decided form (E5, E6), never an edited
  number. The agent reads that section and the code, not the rationales.
- **A decision the section does not make** is not the agent's: it stops and reports. The
  orchestrator settles it from the rationale, or sends it to one `design` agent.
- **Tests in the unit:** the named tests of the crate the unit changes, through `slot.sh` and
  `lowprio.sh` (rule 1). No whole suite, no gate.
- **Before the gate** the orchestrator runs `<record>/z1/stand-in-check.sh` on the staged change:
  it prints every added line that still holds a trial's interim code (the switched-off
  accessibility check, a leak, `kinetic: _`, `unclamped_offset()`, an allowed missing glyph, the
  doc-hidden `widget_style` call, the word "trial"), and one hit sends the change back.
- **The gate** is the orchestrator's, on the staged change, under `gate.lock` (rule 2); the tree
  is committed with the author's message once it is green. A red gate goes back to the author
  with the failing part; after two failures of the same gate the orchestrator takes the task
  over or re-specifies it.
- **The review** starts when the unit's commit exists and reads while the next unit of the lane
  is built. It is the orchestrator's where the unit's diff is within the few hundred lines the
  delegation rule sets, and a `review` agent's above that. The reviewer gets the commit, the
  unit's section of the specification and the unit's lines of `stand-ins.txt`, reads files as `git show <sha>:<path>`, never the working tree, and runs no
  cargo. A must-fix is made by the unit's author after the running unit's commit and before the
  lane's next unit starts, so the worktree has one builder at a time; the orchestrator reads its
  diff; it is gated with `--quick`, or with `--commit` where the fix itself is structural
  (rule 11). No review is open when M runs.
- **Sessions:** the orchestrator is on Opus. A new session after F1, after E-A and G-A are both
  committed, before M, and before K; the record and the memory are brought up to date first.

## 4. Order, and the units' sizes

```
done:     Z1
v0.7.1's gated head exists:
          P1, P2, P0 -> F1 -> E-A -> E-B -> E-C ---------\
                          \-> G-A -> G-B1 -> G-B2 -> G-C --> M, R -> [MG, X] -> V -> K
                          \-> D (drafted beside both) ---/
```

At most four agents with two builders, and two more without cargo (reviewers, D's `worker`). The
gates of the two lanes queue on `gate.lock`; a builder's targeted tests take the other cargo
slot.

Estimates in working days of agent time, built from the rationales' rows and not measured; a
unit, or a task of E-A, at twice its figure is a stop (§6):

| Unit | Estimate | From the rationales' rows |
|---|---|---|
| F1 | 0.25 | adwaita's radius |
| E-A | 2.75 to 4.75: A0 0.25, A1 1.25 to 2, A2 1.25 to 2.5 | the green build, the widgets crate, the tests that fail, the fonts' code, the coverage items |
| E-B | 1.25 to 1.75 | the mapping |
| E-C | 1 to 1.5 | the rest of the showcase, the parity reasons, the egui READMEs |
| G-A | 1 to 1.5 | the green build, the colour, the citations (below the rationale's row: 385 of 386 are moves) |
| G-B1 | 1.25 to 1.75 | the tab bar, the mixed state, their share of right to left |
| G-B2 | 0.75 to 1.25 | the rest of right to left |
| G-C | 1.5 to 2 | the showcase's new parts and statements, the gpui README |
| D, M, R, V, K | 2 to 3 | the root documents, the rehearsal, the close-out |

The lanes add to about 5 to 8 (egui) and 4.5 to 6.5 (GPUI); side by side, the longer lane sets
the time.

## 5. The gates, counted

- `--quick`: P0, F1, E-B, E-C, G-B1, G-B2: six.
- `--commit`: E-A, G-A, G-C, the one over M and R, V: five, and MG's warm-up where there is an
  MG. E-A and G-A are structural (rule 24); G-C is the GPUI lane's third code unit since G-A; M
  and R end the code; V changes the lock. E-C is the egui lane's second since E-A.
- The GPUI Fast legs: with G-A, G-B1, G-B2, G-C, the gate over M and R, and MG's warm-up
  (rules 11, 21).
- The full run: once, in K.

## 6. Stops

- Each unit's own stops are in its section of the specification.
- A patch of Z1's that no longer applies at P2: the unit starts from the trial's report and the
  patch's text by hand, and that unit's estimate is taken again.
- A unit, or a task of E-A, at twice its estimate (§4).
- Each stop returns to the maintainer with what was found. If one lane stops, the other goes on:
  M then merges only the documents, and the lane that is whole ships alone as v0.7.2.

## 7. What is the maintainer's

- Approving the two rationales, the specification and this plan.
- The change D makes to the batch rationale's rules 19 and 25 (GPUI Kit 0.10.x beside egui
  0.37.x in what a later lane's merge accepts): it follows from GPUI Kit 0.10.0 being in v0.7.2.
- The two capture windows: G-B2's one image, and the asset run.
- Merging the pull request, the tag, the publishing and the GitHub release; sending any upstream
  draft.

Opening the draft pull request at the first push and the extra build folders were approved for
the batch on 2026-11-15; this plan uses two build folders.

## 8. Cost check

- **What could refute the design, and is it first?** The trials answered the large questions: the
  joint lock resolves, and the build errors and the failing tests that ran have known causes.
  What they did not reach is first in its lane. In the egui lane that is A0: what the 13 tests
  behind the glyph panics show, E6's arithmetic, the drop-down's cause, the MSRV checks on the
  new lock. In the GPUI lane: the GPUI-only lock (G-A's first step), the mirrored sites (the
  first steps of G-B1 and G-B2), the tooltip with a bound key (in G-C before the showcase
  changes). For the release: R, before the asset run.
- **What waits for it?** A1 and A2 for A0; the showcase units for their lane's library units; M
  for both lanes; K for R.
- **What is built twice?** The gate-read citations, at 0.9.1 in v0.7.1 and at 0.10.0 here, each
  by script. The egui showcase's `tests.rs` is opened by A2 and by E-C, and the gpui showcase's
  `info/` files by G-A (the citations) and G-C (the new lines); each reads only its part.
  Nothing else: the trials' code is applied, and R's release builds are K's.
- **What runs in parallel?** The two lanes, with two builders; the documents, and the reviews,
  beside them without cargo.
- **What does each agent read?** Its unit's section of the specification, its patch, the files
  that section names.
- **Which tests run when?** Named tests in the unit; `--quick` at six unit ends; `--commit` five
  times; the full run once.
- **Where does a new session begin?** After F1; after E-A and G-A; before M; before K.

## 9. The record

`<record>/record.md` holds, per unit: the commit, the gate's folder and time, the review's
outcome, each stop and its answer. No gated-head line is written: no later release branches from
v0.7.2's head; the lanes read it through its tag (rules 20, 25).
