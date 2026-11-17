# GPUI Kit 0.9.1 in v0.7.1: what the port left undone — implementation plan

- Rationale: [`todo_v0.7.1_gpui-kit-0.9.1-rationale.md`](todo_v0.7.1_gpui-kit-0.9.1-rationale.md)
  (N1–N9). Specification:
  [`todo_v0.7.1_gpui-kit-0.9.1-spec.md`](todo_v0.7.1_gpui-kit-0.9.1-spec.md)
  (Parts A to I, *Order and gates*).
- Sibling: [`todo_v0.7.1_kde-open-items-step1-plan.md`](todo_v0.7.1_kde-open-items-step1-plan.md),
  the plan of issue #4's open KDE items, step 1, which lands after this one. The
  two plans share this plan's P1 (the tools) and P0 (commit (0)), the build lane
  and the documents lane (§1), the documents both edit (`CHANGELOG.md`,
  `ROADMAP.md`, `docs/todo.md`: this plan's commits first, that plan's unit 4
  after them, each by quoted text) and one asset run, the close-out, which that
  plan's K5 makes for both (§2.1). Both plans also share one gate count, the
  batch's cadence (`todo_v0.7.1_batch-speed-rationale.md`, S1; §4 here).
- The batch's working rules are cited as "rule N (batch-speed rationale,
  Appendix)": `todo_v0.7.1_batch-speed-rationale.md`, *Appendix — the batch's
  working rules*, rules 1 to 26, authoritative where this text differs. Rule
  26 does not apply to v0.7.1: its commit (0) (P0) adds the batch's documents
  from `v071`, where they were written; there is no earlier commit and no
  separate approval copy. v0.7.1 has no path check of rule 22 (b) either: it
  has no merge with a previous tag, its branch was not made from a tag, and its
  documents were written on this tree (that rule's "P2 runs it" means a release
  plan's P2, not this plan's P2, the check scripts). Rules 17, 21 and 25 do
  not apply here either: there is no merge with a previous tag, so no MG and no
  X. Of rule 19 only its first part applies, the parity rule (a parity
  difference is a fault, fixed or excepted, the maintainer told), at the KDE
  plan's KR, KD and K5; its MG and measurement parts do not (there is no
  measurement; rule 19's "K5" there is the KDE steps 4 and 5 plan's close-out),
  and its `Cargo.lock` comparison with `0fa28027` is a release plan's P2's, and
  this branch is `0fa28027` itself, whose lock L1 and L2 change by design.
- Before both: v0.7.1's first unit, GS, the gate-speed port
  (`todo_v0.7.1_gate-speed-spec.md`, its plan `todo_v0.7.1_gate-speed-plan.md`),
  after P1 and P0 (its Task 0, which records the maintainer's answers in its
  record and edits no document, before P0), and the first unit that runs a gate; then LN (Part I), the
  three gates that stop reading facts lines by position (§1).
- Written: 2026-11-14, against `v0.7.1-rc2` at `0fa28027` (pull request #10).
  Its `file:line` citations are `0fa28027`'s unless they name another commit or
  version; it adds no line-number citation of a changing repository file (S6).
  Status: **implemented on 2026-11-17** (commits `e866ea6c..v0.7.1`, the
  close-out commit that moved this plan, its rationale and its specification here
  from `docs/` among them); the running record at the end of this plan says how
  it ran. Approved as **to be implemented once free of errors** (the maintainer,
  2026-11-15: "When the plan that goes in the 0.7.1 version is ready, without
  errors, start implementing it."), which approves it as written; §7 lists what
  is settled. The text below is the plan as it was approved: it speaks of the
  work as still to do, and gives the paths of the documents moved here as they
  were in `docs/`.
- Where: branch `v0.7.1-rc2`, worktree
  `/home/tibi/Rust/native-theme/.claude/worktrees/v071`. The pull request is not
  merged by this plan.

This plan says who does what, in which order, and how each task is known to be
done. What each task changes is the specification's; the plan repeats none of
it.

## 1. Shape of the work

Three worktrees of the same repository, so that no agent's half-done files
reach another's tests or commits. The lanes meet by merges the orchestrator
makes (`git merge --no-ff`, so every commit keeps its hash), each made while no
agent works in the worktree it merges into. Worktree paths are relative to the
main checkout.

| Lane | Worktree and branch | Tasks | Cargo |
|---|---|---|---|
| core | `.claude/worktrees/v071`, `v0.7.1-rc2` | P1, GS's Task 0, P0, GS (the gate-speed plan's units), LN, E1 (P2 beside LN and E1), H, O, L1, L2, A, C, BS, then M1, E2, G1, G2, G5 | yes: the gate |
| build | `.claude/worktrees/v071-lane`, `v0.7.1-rc2-gk-connector`, made from O's commit | BT, BX | yes, `-p native-theme-gpui` only, at low priority |
| docs | `.claude/worktrees/v071-docs`, `v0.7.1-rc2-gk-docs`, made from O's commit | D, F, DM | none but `cargo metadata`, which F.6's checks call |

After this plan's last task in a lane, the KDE plan takes the worktree over on a
branch of its own (that plan's §1): the build lane once BT's review and the
orchestrator's reading of BX have passed and every fix is committed (the
condition M1 needs for that branch), the documents lane after M1. So the build lane's build folder (Cargo makes one per worktree path)
is the one extra build of both plans; the documents worktree builds nothing,
though F's `cargo metadata` may leave a folder for it, which the clean-up tests
for. The KDE plan deletes both (its §2.2).

```
core   P1 ─ GS's Task 0 ─ P0 ─ GS ─ LN ─ E1 ─ H ─ O ─┬─ L1 ─ L2 ─ A ─ C ─ BS ──[M1, push]── E2 ─ G1 ─ G2 ─ G5 ─→ KDE plan: MK1 … K5
       P2 (beside LN, E1)                           │                │    │      ↑   ↑
build                                          ├─ BT ─ BX ──────┼────┼──────┘   │      (reviews passed, fixes in: then the KDE plan's K0 … K3)
docs                                           └─ D ──────[merge at A]─ F ─[merge at C]─ DM ─┘  (then the KDE plan's KF, K4)
```

Why this order (each from the specification's *Order and gates* unless named):

- The gate-speed plan's Task 0 before P0 (that plan's *Order*): it records the
  maintainer's answers to the gate-speed specification's O1 to O3 and decision
  4 in that plan's record; the two documents are already written for those
  answers, so it edits nothing and needs no review, and commit (0) commits the
  documents GS follows as they stand.
- P0 then: commit (0) adds the approved documents before any other change,
  the batch-speed rationale and the gate-speed specification and plan among
  them (§7, *Commit (0)*).
- GS second, the gate-speed port (`todo_v0.7.1_gate-speed-plan.md`; rationale
  S1): v0.7.1's first unit, so that every later unit of both plans gates with
  the ported `--quick` and the cadence (§4). It runs on P0's tree and is the
  first unit that runs a gate; P0 runs none of its own (a documents-only
  commit, gated by the five documentation checkers): GS's last gate, `--commit`
  on the ported script, is the branch's first timed `--commit`, the warm-up
  gate, and covers P0's tree, which lies under it. A red warm-up gate follows
  rule 12 (batch-speed rationale, Appendix): it goes to the author of the unit
  whose rule the failing test checks, or, where that author is no longer at
  hand, to a new `implement` agent with the log and the unit's part of its
  specification, and its fix is gated by the warm-up run again.
- LN, Part I, right after GS (the maintainer's S6 in the batch-speed rationale,
  *The one change this needs*): the three gates that compare a facts line's number with its text
  stop doing so before any later unit of both plans adds a facts row or a
  preset citation, so no unit after it re-points a line number to keep a gate
  green.
- E1, commit (i), after LN, before every other part, so that Parts A to D write
  0.9.1 lines directly (decision 3). As E.4 step 1 has it, the run starts on a
  clean tree at LN's last commit, which is `BASE_REV`, so that gate 1 compares
  E's own write alone; GS and LN change no file of E.2's scope (checked before E1,
  §2's E1 row), so the scope's tracked files are commit (0)'s. The
  texts that cite a document "at `<sha>`" (H, D.3) still name commit (0).
- H and O before the lanes fork: point 8 makes Part H's section and D.3's open
  entry come in or before the commits that write B.3's and B.10's texts, which
  are in two different lanes (BS in core, BX in the build lane); point 5 puts H
  before O. Both forks then start from a tree in which both headings exist.
- The lanes on disjoint files: the build lane holds Part B's edits of the
  connector's own source, README and the upstream-request draft (B.2, B.4's
  README cell, B.5, B.6, B.9's comment, B.10, B.13); the core lane Part B's
  showcase texts (B.1, B.3, B.4's `GEOMETRY_NOTES`, B.8, B.9's note, B.15),
  which share `SC/info/` and `SC/tests.rs` with Parts A and C; the documents
  lane every `docs/todo.md`, `docs/archive/todo_closed.md` and `ROADMAP.md`
  edit of Parts D and F, and Part F's script, exceptions and README. H and O
  also edit `docs/todo.md`, in the core lane, before the forks, so both lane
  branches start with them.
- L1 and L2 (D.6) first in the core lane, so that the core lane's parts (A, C
  and Part B's showcase texts) are tested on the lock that ships; each is a
  commit on a clean tree with the gate. The build lane forks at O and works on
  the old lock until M1, whose gate runs its commits on the new one.
- F after A (point 4, A.1); the documents branch takes a merge of
  `v0.7.1-rc2` at A's commit first. DM after C, L1 and L2 (point 2, C's outcome
  decides `:3959`; point 3, `:2187` and `:4140` after D.6), after a second such
  merge. D (D.2, D.4, D.5) needs E1's `lookup.py` only (point 3).
- E2, commit (iii), after all of (ii) is merged (M1): E.5's taffy item needs
  D.6; B.7 lands in it.
- G1, G2, G5, the commits (iv), after (iii) (point 7); G2's count after H and O
  have opened their entries (G.2).
- The KDE plan's merges after G5: this port lands first (both specifications);
  the asset run, G.4, once after both.

## 2. Tasks

A task that changes files ends in one commit on its lane's branch (no
attribution line), made only when its done-criterion holds: in the core lane
by the orchestrator, after the gate, on exactly the gated tree, with the
author's message; in the build and documents lanes by the author (§4, *Who
runs what*). Every agent stages
its own paths by name; `git commit -a` and `git add -A` are not used. The
commit message follows the repository's form (a subject naming the part and
what it does; a body with the outputs the specification asks for: E.4 step 7's
counts and set-aside list, D.2's and D.5's lookup lines, D.3's check, E.5's and
E.7's listings, G.6's outputs). Agent types are the maintainer's delegation
table's. "The work folder" (`<work>`) is the folder the specification names,
`/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/`
(`citations/` E.1's copy, `checks/` G.6's and D.3's scripts and the quote files,
`tools/` the build-slot wrappers both plans use, called `<tools>`:
`/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/tools`);
`.superpowers/` is excluded by
`.git/info/exclude`. "The running record" is
`~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.1_gpui-kit-0.9.1-plan/record.md`:
the dated results, shas, counts and decisions filled in as the work goes. Only
the orchestrator writes it, from the agents' reports; it is copied into this
plan file only in the commit that commits the plan's record, the close-out's
(the KDE plan's K5 step 7), so no tracked file is edited during the work and the
clean-tree checks hold.

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| P1 | **The tools.** First the folders: `mkdir -p <work>/tools` (`<tools>`, as §2 defines it) and `mkdir -p ~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.1_gpui-kit-0.9.1-plan/`, the folder of the running record. Then copy the build-slot wrapper `slot.sh` and the low-priority wrapper `lowprio.sh` from the session's durable scratch folder, `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/` (where the session's gate script calls them today), into `<work>/tools/`, and read both. In the copy of `slot.sh`, the hard-coded lock directory (`D=…`, a session's scratchpad, which does not outlive that session) becomes the one durable lock folder of every plan and session, `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/slot-locks/`, which P1 creates with `mkdir -p`, so that the two slots hold across all plans and sessions: they bind every process started through a copy of `slot.sh` that uses this lock folder (every plan of the batch does), and a copy using another folder is not counted; the copy of `lowprio.sh` keeps its `renice -n 19 -p $$` (an absolute nice of 19, where `nice -n 19` is relative and gives 15 under Konsole's −4) and its `ionice`, and gains the line `export CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16` (the gate's limit while the maintainer works; the original has neither). Both plans' briefs name these paths, and no other copy of `slot.sh` is used during the work. In the same task, the KDE plan's K0 step (1): its inputs, copied from `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/batch-2026-11-15/`, the durable copy of the writing session's scratch folder, since a session's scratchpad does not outlive its session. **The hook trial** for the core worktree, before any code agent works there: a Write of a new untracked file `<core worktree>/native-theme/src/hook_trial.rs` holding `fn f() { x.unwrap() }`, which the project's `no-runtime-panics.sh` must refuse, nothing written; if the file is written, it is removed (`rm`) and P1 stops with no code agent dispatched (§6). **The pull request:** `cd /home/tibi/Rust/native-theme/.claude/worktrees/v071 && gh pr view 10 --json state,headRefName` shows it open with head `v0.7.1-rc2`; anything else, or a failed read, is a stop (§6). No repository change. | orchestrator | the maintainer's resource rules | pull request #10 read open on `v0.7.1-rc2`, the output in the running record; the hook trial was refused and `git status --porcelain` in the core worktree shows no `hook_trial.rs`; the running record lists, one by one by path, the untracked planning documents `git status --porcelain --untracked-files=all` shows in the core worktree (§4, *The staged-tree check*); `grep -n '^D=' <work>/tools/slot.sh` names the lock folder; `slot.sh` runs a command only when one of its two slots is free and holds it until the command ends, tested by running it: two `<tools>/slot.sh sleep 30 &` hold both slots, and a third `<tools>/slot.sh true` prints its slot line only after one of them ends; `<tools>/slot.sh <tools>/lowprio.sh sh -c 'echo "$CARGO_BUILD_JOBS $RUST_TEST_THREADS"; ps -o ni= -p $$'` prints `16 16` and `19`; after it the lock files are in the lock folder (`ls` of it) |
| P2 | **The checks.** `checks/check_text.py` (G.6) and `checks/check_moves.py` (D.3) written verbatim from the specification; one quote file per text of G.6's list, made from the specification's texts and not from any edit, every placeholder not known in advance split with "…". Beside LN and E1, no cargo. No repository change. | `worker` | G.6; D.3; the parts G.6 lists | `diff` of each script against the specification's block (extracted by the worker's own script, saved in the work folder) shows no difference; a listing in the work folder names every G.6 item with its quote file, none missing |
| P0 | **Commit (0)**, made once the two findings of the gate-speed documents' confirming check (the self-test's call form, the include rule inside macros) are fixed and the orchestrator has read the fix (2026-11-16); no further review pass follows, since the port's self-test and the specification's E.1 breaks prove both rules mechanically. The documents of this cycle the maintainer has approved (§7, *Commit (0)*), each committed document's status line set to what the maintainer approved, with the date: the GPUI Kit 0.9.1 rationale's to "Approved by the maintainer's instruction of 2026-11-15: 'When the plan that goes in the 0.7.1 version is ready, without errors, start implementing it.'", and `docs/issues/0004_kde-open-items-rationale.md`'s to "D2, D3 and D10 (a) approved for step 1 (2026-11-15); the later steps not yet approved."; in both, the Status paragraph's second sentence, "No code follows from it before the maintainer approves its specification and plan.", is removed when the status line is set; among them `docs/todo_v0.7.1_batch-speed-rationale.md` and the gate-speed specification and plan (`docs/todo_v0.7.1_gate-speed-spec.md`, `docs/todo_v0.7.1_gate-speed-plan.md`), so that GS follows an approved, committed plan; their commit is the `<sha>` of commit (0) every text that cites a document "at `<sha>`" names (H, D.3). The icons rationale is not in it (§7). New files: the approved documents, each named in the running record; after the commit the record's list of untracked planning documents (P1) keeps only those not committed, one by one. | orchestrator | *Order and gates*, (0) | `git status --porcelain --untracked-files=no` prints nothing; the five documentation checkers pass (§4); no **gate** of its own: GS's last `--commit` covers P0's tree |
| GS | **The gate-speed port**, v0.7.1's first unit, on P0's tree: the units of `docs/todo_v0.7.1_gate-speed-plan.md`, by that plan's owners, done-criteria and reviews. Its Task 0 runs before P0 (that plan's *Order*): it records the maintainer's answers to O1 to O3 and decision 4 in that plan's record, the two gate-speed documents being already written for them, so no document is edited and none reviewed; P0 then commits those two documents as they stand. The rest is one unit of four commits (gate-speed specification, Part F), each committed by the orchestrator after its gate, as §4's core-lane rule says. It changes `scripts/check_release.sh`, `scripts/step_cache.sh` (new), `scripts/step_cache_includes.py` (new; the gate-speed specification's C.4 include-scan helper), `scripts/msrv_checks.sh` (new; Part M), `scripts/README.md`, `CONTRIBUTING.md` and `CHANGELOG.md` (gate-speed specification, *How to read it*), no file in E.2's scope. GS starts only after the maintainer's answers (2026-11-15: O1a and O1b no, O2 (e), O3's keyed option, decision 4 (y)), which Task 0 records; GS is its four commits (i) to (iv). | that plan's | `docs/todo_v0.7.1_gate-speed-spec.md`; rationale S1 | that plan's done-criteria; its last **gate**, `--commit` on the ported script by the orchestrator as a long run (the gate-speed specification's E.4 item 1), passes and changes nothing, on the clean tree at GS's last commit (a last fix is committed first, its content the staged change its own gate passed): the branch's first timed `--commit` (time in the running record), the warm-up gate, covering P0's tree; E.4 item 4 then times `--quick` on the clean tree |
| LN | **Part I**, the three gates that read facts lines by position (I.1 to I.4), one commit, on the tree GS leaves. Each check's temporary edit of I.5 is made, seen red and restored, and the inserted first line seen to pass, then restored, before the commit. | `implement` | header; Part I | I.5 holds: each failing line and the checker's counts in the commit body; `git diff --cached --name-only` lists only the eight files Part I names; `check_todo_citations.py` passes; `cargo fmt --all --check` passes; **gate** (the orchestrator's, §4): `--commit` (structural, rule 24 (batch-speed rationale, Appendix): it changes a comment and a step label of `scripts/check_release.sh`) passes on the staged change and changes nothing; the running record names LN's commit |
| E1 | **Commit (i)**, Part E's digits-only run: E.1 to E.4, on the tree LN leaves; first `git diff --name-only <commit (0)> HEAD -- <E.2's pathspecs>` prints nothing (GS and LN touched no file of the scope; otherwise a stop, §6), and `BASE_REV` is HEAD, LN's last commit (§1). A gate failing in a way E.4 does not name follows *Order and gates* (i): undo, `REVIEW_ASIDE`, repeat; or stop and report. The undo works on a staged change: `git checkout HEAD -- <E.2's pathspecs>` (or `git restore --staged --worktree --source=HEAD -- <E.2's pathspecs>`), and before the run repeats from E.4 step 2, E.4 step 1's clean-tree check runs again (`git status --porcelain --untracked-files=no` prints nothing). | `worker` | header; Part E: E.1–E.4; decision 3 | E.1's check of the copy finds nothing; E.7's first bullet holds (gates 1 and 2 pass, gate 4's failures all in `set-aside-gate4.txt`, E.4 step 6's three tests on both cores); `lookup.py --check` passes; `cargo fmt --all --check` passes (the run writes through `remap.py --write`, which the editor's format hook does not see, and a digit that grows can push a line past 100 columns); **gate** (the orchestrator's, §4): `--quick` (the first code unit after LN's `--commit`; the first `--quick` timed on a change, E.4 item 4 having timed the clean-tree one) passes on the staged change and changes nothing |
| H | Part H's section. | `worker` | Part H; the `<sha>` of P0; after P2 (its quote file) | `check_todo_citations.py` passes; `check_text.py` prints `ok` for every G.6 text this commit writes (Part H's section); `git diff --cached --name-only` lists `docs/todo.md` alone before the commit; no **gate** of its own (O's covers it), the orchestrator commits it with the author's message |
| O | D.3's open entry *gpui: the widgets' own icons follow the theme's icon set*, after Part H's section, its last sentence as §7 (*What lands in v0.7.1*) decides: without the icons sentence, in a commit of its own. | `worker` (the same, after H) | D.3, the open entry's paragraph | as H, for the open entry; **gate** (the orchestrator's, §4): H and O are one documents-only unit in two commits, so the five documentation checkers (run on each) and `--quick` pass on O's staged change, whose tree holds H's commit too, and change nothing |
| — | **The forks**, from O's commit, after it is made: the build and documents worktrees are made on their branches with absolute paths (rule 24 (batch-speed rationale, Appendix)): `git -C /home/tibi/Rust/native-theme/.claude/worktrees/v071 worktree add -b v0.7.1-rc2-gk-connector /home/tibi/Rust/native-theme/.claude/worktrees/v071-lane <O's commit>` and `git -C /home/tibi/Rust/native-theme/.claude/worktrees/v071 worktree add -b v0.7.1-rc2-gk-docs /home/tibi/Rust/native-theme/.claude/worktrees/v071-docs <O's commit>`, and P1's hook trial is made in each before any code agent works there (`<worktree>/native-theme/src/hook_trial.rs`); a worktree where the file is written gets it removed and the work there stops (§6). Then, before BT, the build lane's first builds, long runs (§4): `cargo test -p native-theme-gpui --features native,watch --all-targets --no-run`, the same with `gpui-fast,native,watch`, a clippy warm-up (`cargo clippy -p native-theme-gpui --all-targets` with `native,watch`, with `gpui-fast,native,watch` and with `--no-default-features`), and `scripts/run_in_runner_env.sh --prepare`, each from the build worktree. | orchestrator | §1 | both worktrees exist; `git -C <each> log -1 --oneline` names O's commit; the trial was refused in both, nothing written (`git -C <each> status --porcelain` prints nothing); the build lane's first builds, warm-ups and `--prepare` have finished, each `STATUS` 0 |
| L1 | D.6, `unicode-properties`, on a clean tree. The orchestrator runs D.6's four commands (`cargo update -p unicode-properties`, the `grep` of the lock, `git diff Cargo.lock`, `cargo tree --workspace -i unicode-properties --target all`) and then both cores' builds, `cargo test -p native-theme-gpui --features native,watch --all-targets --no-run` and the same with `gpui-fast,native,watch`, as one long run (§4), its outputs in `<work>/L1/run-<n>/`. It then dispatches the worker with those outputs, for D.6's gates and the commit message. If a test turns red: the lock is restored, staged or not, with `git checkout HEAD -- Cargo.lock`, nothing is committed, and the task reports which test failed on which core at which version; DM writes the entry's dated line in `docs/todo.md` from that report (D.6, D.3). If the lock does not change (D.6's no-change path), nothing is committed either, and the task reports the version the lock keeps and the requirements it read; DM records that in a dated line of its own commit, which the entry's closing line names. | `worker` | D.6; D.3's `:2187` paragraph (the version read back goes into the commit body, the running record and DM's closing line) | D.6's gates: the lock diff as D.6 says (or every other line listed with its reason); the version read back named in the commit body; `cargo audit`; the gpui connector's tests on both cores, all targets; the showcase's suites on both cores in the runner environment; **gate** (the orchestrator's, §4): `--commit` (a lock change, structural) passes on the staged change and changes nothing. Under the gate-speed specification's decision 4 (y), the reviewed registry list: the `--commit` log's refusal lines are read; where one names a package the update moved, that package's build script or proc macro is grepped for program runs, file reads and environment variables, the worker extends the list (`CACHE_REVIEWED_REGISTRY_CODE` in `scripts/check_release.sh`, C.2) and stages it, `scripts/check_release.sh` being then this row's one extra file, which the staged-tree check allows; the orchestrator runs `--commit` again on that staged change (a gate-script change, so structural), whose log must show no refusal naming a moved package, and that tree is committed (rule 2; §2); where there is no such line, the running record says so. No extension is expected: every registry version of taffy (0.15.0, 0.15.1) and unicode-properties (0.1.3 to 0.1.5) has `build = false` and no proc macro (their `Cargo.toml`s in Cargo's registry, read 2026-11-15). On the red path: the lock unchanged (`git status --porcelain --untracked-files=no` prints nothing) and the report |
| L2 | D.6, `taffy`, as L1 (the four commands with `taffy`, the outputs in `<work>/L2/run-<n>/`). | `worker` | D.6; D.3's `:4140`; E.5's taffy item (whose re-reading is E2's) | as L1 |
| A | Part A, its tests first: A.5's tests 2 and 4 are seen to fail before A.3's `.decorations(…)` and A.4's sample lines. | `implement` | header (*Constraints*, *Lines that must not move*); Part A | A.6's commands pass on gpui-pre and GPUI Fast (the eight new tests among them, a `cfg` on GPUI Fast only where A.5 allows it, its reason in the commit body), among them the library tests `cargo test -p native-theme-gpui --lib --features native,watch` and the same with `gpui-fast,native,watch`, which hold the claim, prose, `every_widget_reports_itself` and `the_showcase_hardcodes_no_style_values` tests (`CN/src/lib.rs:243-244`, `#[cfg(test)] mod showcase`); `cargo fmt --all --check` passes before the commit; `check_text.py` prints `ok` for every G.6 text this commit writes (A's doc comments); **gate** (the orchestrator's, §4): `--quick` (the first code unit after L2's `--commit`) passes on the staged change and changes nothing |
| C | Part C: the removal, then its two commands, each run through `scripts/run_in_runner_env.sh` as A.6's suites are. Both pass: the comment of C, committed. Either fails: the override and the import are restored, no commit, and the task stops and reports (§6, Open point 2); where the removal is staged by then, the restore is `git checkout HEAD -- <the file>`. | `implement` | Part C | both commands pass; A.6's suites pass on both cores; `cargo fmt --all --check` passes before the commit; `check_text.py` `ok` for every G.6 text this commit writes (C's comment); **gate** (the orchestrator's, §4): `--quick` (on the normal path, the second code unit since L2's `--commit`; on another path the running record's count decides, rule 11) passes on the staged change and changes nothing |
| BS | Part B's showcase texts: B.1, B.3, B.4's `GEOMETRY_NOTES` text, B.8, B.9's `SC/info/layout.rs` note, B.15. | `implement` | header; B.1, B.3 (it lands, §7), B.4, B.8, B.9, B.15 | A.6's commands pass on both cores, and B.14's: `cargo test -p native-theme-gpui --lib` with `native,watch` and with `gpui-fast,native,watch` (the claim and prose gates among them) and `env RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme-gpui --no-deps` (§5's environment rule); `cargo fmt --all --check` passes before the commit; `check_text.py` `ok` for every G.6 text this commit writes; **gate** (the orchestrator's, §4): `--commit` (on the normal path, the third code unit since L2's; on another path the running record's count decides, rule 11) passes on the staged change and changes nothing |
| BT | B.13: the check box and the radio ask the application's icon resolver. The two tests first, seen to fail. | `implement` | header; B.13 | the two tests fail before the change and pass after; B.13's gates pass, `-p native-theme-gpui`, on both cores and with `--no-default-features`; `cargo fmt --all --check` passes; E.5's pinned-lines check with `F` = `connectors/native-theme-gpui/src/widgets/checkbox.rs` and `L` = 145 prints `ok` for every hunk (its output in the commit body); `check_text.py` `ok` for every G.6 text this commit writes (B.13's comment); no **gate** in the lane (M1's covers it) |
| BX | Part B's connector texts: B.2, B.4's README cell, B.5, B.6, B.9's comment (anchored on its words, B.9), B.10. | `worker` | header; B.2, B.4, B.5, B.6, B.9, B.10; B.14 | B.14's commands pass, `-p native-theme-gpui`, `cargo doc -p native-theme-gpui --no-deps` without warnings among them, run as `<tools>/slot.sh <tools>/lowprio.sh env RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme-gpui --no-deps` (§5's environment rule), and the private-items leg `check_release.sh:1008-1009` runs, the same with `--document-private-items`; `cargo fmt --all --check` passes; E.5's pinned-lines check with `F` = `connectors/native-theme-gpui/src/geometry.rs` and `L` = 772 prints `ok` for every hunk (its output in the commit body); `check_text.py` `ok` for every G.6 text this commit writes; no **gate** in the lane (M1's covers it) |
| D | D.2, D.4, D.5: the dated lines of the still-holding entries, the changed entries, the roadmap's upstream list. One commit. A D.5 bullet that no longer holds is reported, not rewritten: in the commit body, and the orchestrator puts it in the running record (D.5's "the plan's record of the task"). | `worker` | D.1, D.2, D.4, D.5; E1's `lookup.py` | D.7's checkers pass; D.2's and D.5's checks (every `lookup.py` line and each hand-read line's text) are in the commit body; `check_text.py` `ok` for every G.6 text this commit writes (D.4's lines); the commit holds D.1's counts, one D.2 dated line on each of the 45 entries D.1 sends to D.2 alone, and D.4's edits of its four entries, or the body explains each difference; no **gate** in the lane (M1's covers it) |
| — | **The documents branch takes a merge of `v0.7.1-rc2`** at A's commit, while no agent works in the documents worktree. | orchestrator | point 4 | the merge is clean, or its conflict resolved by the quoted texts and reported |
| F | Part F: the discovery rule, its self-tests, the `GPUI_VIA` entries, the exceptions, F.3's dated line, F.7. Each F.5 case is checked by removing the part of the rule it names. | `implement` | header; Part F | F.6 holds: `python3 scripts/check_widget_coverage.py` passes, run through `lowprio.sh` alone (it calls `cargo metadata`, which builds nothing, so it takes no slot), with `discovered` up by nine; `--self-test` passes; each mutation case failed as F.6 asks, reported in the commit body; `check_todo_citations.py` passes; `check_text.py` `ok` for every G.6 text this commit writes: F.3's four `GPUI_VIA` comments and its dated line, F.4's group comment and five reasons, F.7's two texts; no **gate** in the lane (M1's covers it) |
| — | **The documents branch takes a merge of `v0.7.1-rc2`** at the last core commit after C, or after C's stop where it committed nothing; L1 and L2, committed or stopped on their red path, are before it. | orchestrator | points 2, 3 | as above |
| DM | D.3's moves, in a commit of their own; the closing lines from C's, L1's and L2's commits and P0's `<sha>` (where L1 or L2 left the lock unchanged, from the commit of DM's dated line that records it). If C stopped, `:3959` stays and gets Open point 2's dated line: the worker stages it and reports, the maintainer sees the text, and then the worker is continued to commit it (or the orchestrator commits it with the worker's message); if L1 or L2 kept the old version, its entry stays (D.3) and gets D.6's dated line, written from that task's report. Each dated line is a commit of its own before the moves, since D.3's check refuses a line the moves commit adds to `docs/todo.md`. | `worker` | D.3; D.6; L1's and L2's reports | `check_moves.py` prints `ok` on the staged change, its output in the commit body; the five documentation checkers pass after each commit; no **gate** in the lane (M1's covers it) |
| M1 | **After every review it merges has passed and each fix is committed**: BS's, BT's, BX's reading, D's reading, F's, DM's reading, and, of the core commits under M1's gate, A's and C's reviews and the orchestrator's readings of E1, H, O, L1 and L2. Merge each lane branch, `v0.7.1-rc2-gk-connector` and `v0.7.1-rc2-gk-docs`, at its last commit, every review fix of its tasks included, into `v0.7.1-rc2`. A conflict is resolved by the quoted texts and reported. Then the gate, and `git push origin v0.7.1-rc2:v0.7.1-rc2`; CI runs in the background, and its result is read before E2's commit. | orchestrator | *Order and gates*, (ii) | **gate** (the orchestrator's, §4): `--quick` (it brings BT and F, the first and second code units since BS's `--commit`; BX, D and DM are documents units; not structural, rule 24; `--commit` where the count it completes reaches three on another path) passes on the merged tree, with the checks the specification lists under *The checks that prove the whole* that the gate does not run (the GPUI Fast legs, §4), and leaves the tree clean; CI started |
| E2 | **Commit (iii)**, after M1: E.5 with B.7 (its date this commit's day). At its start, before any edit, E.5's version-label grep is taken again (its count at `0fa28027` was 200) and the count goes in the commit body. | `worker` | header; E.3, E.5, E.7; B.7 | E.7's second bullet holds, every listing in the commit body: the word-diff script's output with the rewrite each line belongs to, E.5's pinned-lines check for every pinned file, the second run of E.5's version grep, E.4 step 6's three tests on both cores; `check_text.py` `ok` for every G.6 text this commit writes (B.7's header, E.5's relabelled lines outside E.2's scope); `cargo fmt --all --check` passes; **gate** (the orchestrator's, §4): `--commit` (on the normal path, the third code unit since BS's, after BT and F at M1; on another path the running record's count decides, rule 11) passes on the staged change and changes nothing |
| G1 | G.1: the CHANGELOG's three lines and the README entry's sentence. | `worker` | G.1; decision 11 | `check_text.py` `ok` for every G.6 text this commit writes; `git diff --cached --name-only` lists `CHANGELOG.md` alone before the commit; no **gate** of its own (G5's covers it), the orchestrator commits it with the author's message |
| G2 | G.2: `ROADMAP.md:94`'s sentence and `:264`'s count. | `worker` (the same) | G.2 | `check_text.py` `ok` for every G.6 text this commit writes; G.2's grep, run on the tree it commits, with its output in the commit body; no **gate** of its own (G5's covers it), the orchestrator commits it with the author's message |
| G5 | G.5: the note in `docs/todo_gpui-full-theme.md`. | `worker` (the same) | G.5 | each cited 0.9.1 line opened and listed with its text in the commit body; `check_text.py` `ok` for every G.6 text this commit writes; **gate** (the orchestrator's, §4): G1, G2 and G5 are one documents-only unit in three commits, so the five documentation checkers and `--quick` pass on G5's staged change, whose tree holds G1 and G2, and change nothing |

After G5 and its gate, E2's review passed with every fix committed and gated,
and the orchestrator's reading of G1, G2 and G5, the KDE plan's MK1 to MK4
merge that step's work, and its K5 makes the close-out of both plans.

### 2.1 The close-out

The asset run (G.4) runs once, after the last change of everything that lands in
v0.7.1 (Open point 1): it is the KDE plan's K5 (its §2.1), which runs G.4's eight
steps for both specifications and, in the commit of step 7, archives this
plan's three documents with that plan's and the gate-speed specification and
plan (archiving is the last task of both plans; the batch-speed rationale stays
in `docs/` until the batch's last release archives it), then runs `~/.claude/scripts/sweep-target.sh` on the worktree.
The KDE step lands in v0.7.1 (D10 (a), the maintainer, 2026-11-15), so the
close-out without it is not taken.

v0.7.1's last unit before its close-out is the KDE plan's MK4, the gated head,
which takes its cadence gate, the
five documentation checkers and `--quick`, with no extra `--commit`: the last code
unit's `--commit` (MK3's) covers the code beneath it, and where a fix that KR
or KD demanded landed after MK3, that fix's `--commit` covers it (rule 20
(batch-speed rationale, Appendix)). MK4 is made once every review (each KR or KD
fix's, the reading of K4) is closed with its fixes committed, and once it has
passed, the orchestrator writes into this plan's running record,
`~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.1_gpui-kit-0.9.1-plan/record.md`,
the line "the lane's gated head for the next release: <sha> (MK4; --quick run
<folder>; reviews closed)", naming MK4's commit (a fix after a red gate
included). Both lanes' next releases read
that line, KDE steps 2 and 3 and the widget icons: the next release's branch starts from
that gated head and is merged with its `<prev-tag>` (the tag just before it in
the release order: by the batch's proposed numbers, v0.7.2 for steps 2 and 3,
v0.7.4 for the icons) once (S2 as amended, rules 17 and 21), after its own
last unit before the merge (its documents unit, rule 20); it never waits for v0.7.1's close-out
or tag. Where its `<prev-tag>` already exists when it starts, it branches from
that tag instead and has no merge (rules 17 and 18). The lane predecessor
depends on the release order, still the maintainer's (rule 18). A worktree v0.7.1 used, `v071` among them, is reused only once
v0.7.1's tag exists, checked after `git fetch origin tag v0.7.1` (rule 18). The core worktree `.claude/worktrees/v071` and its build folder
stay after v0.7.1 (the batch's worktree rule, named in each `docs/todo_v0.7.x_*`
plan's §1; S3 as amended): the KDE lane, which holds the caret too, alternates
between `v071` and `.claude/worktrees/kde-b`, steps 2 and 3 running in `kde-b`
while this close-out holds `v071`, steps 4 and 5 in `v071`, the caret in
`kde-b`, steps 6 and 7 in `v071`; the second lane holds the widget icons alone,
in `.claude/worktrees/lane2-a`.

**Merging the pull request, tagging, publishing, the GitHub release, the pull
request's title and description, and sending the upstream requests (B.6's draft
among them) are the maintainer's decisions and are not part of this plan.**

## 3. Reviews

Every code commit is reviewed before its merge into `v0.7.1-rc2`; a core-lane
commit is reviewed while the lane's next task works. The reviewer reads the
commit (`git show`, the commit range), never a working tree, and runs no cargo;
it gets the diff and the specification's parts, never the author's report.
Every reviewer, the orchestrator included, checks that no new code transcribes
an LGPL source (§5, *Clean room*). A
must-fix is a new commit by the task's author in the same lane, reviewed by the
same reviewer, passing the same done-criterion, made as the lane's next commit
after the task then running there has committed. A second must-fix on the same
point falls under §6's take-over rule.

| Commit | Reviewer | Why that one |
|---|---|---|
| LN (three gates by text) | `review` | a weakened test would let a wrong facts row pass unseen; I.5's red runs are its evidence, read against I.1 to I.3 |
| A (four showcase parts, eight tests) | `review` | new code and tests that must prove what A.5 claims; the Widget Info claims against the 0.9.1 sources |
| BS (showcase texts and three new claims) | `review` | statements about upstream's behaviour that the prose gate reads only for their citations, read against the 0.9.1 sources in Cargo's registry |
| BT (the resolver call, two tests) | `review` | a change of what the connector draws, and tests that must fail without it |
| F (the discovery rule) | `review` | a module-path walker whose miss lets a widget pass the coverage gate unseen; the self-tests and the mutation cases are its evidence |
| E2 (the hand-read) | `review` | E.5's *Who checks*: every `MOVES PINNED LINES` line read against the commit body, and each re-pointed number read at its new line |
| C (a removal and a comment) | orchestrator | some forty lines, decided by one test on two cores |
| E1 (digits only) | orchestrator | `gates.py` checks the digits; the orchestrator reads the counts and the set-aside list in the body |
| L1, L2 (the lock) | orchestrator | the lock diff and D.6's gates |
| BX, D, DM, H, O, G1, G2, G5 (texts, backlog, roadmap, changelog) | the checkers, `check_text.py`, `check_moves.py`, then the orchestrator reads the diff | the gates validate the data and the texts; the texts were reviewed in the specification |

No Fable agent is planned for this plan's own tasks (no diff here is in a
resolver, validation, a runtime, concurrency or `unsafe`); GS's review is
`review-hard` (the gate-speed specification, Part F), under its own plan.

## 4. Gates

- **While working**, an agent runs the tests of the module it changes. **Before
  its commit**, it runs its done-criterion's commands once, in the foreground,
  through the tools' wrappers. The two release stamps are stale from E1 (its
  digits are in the connector's and the showcases' sources) until the
  close-out and are no task's done-criterion.
- **Who runs what.** An agent runs only its task's targeted done-criterion
  commands (the named tests, `cargo fmt --all --check`, the checkers, a `-p`
  clippy, build or doc leg), each expected to finish well within the Bash
  tool's 10-minute foreground limit, in the foreground; it stages its change
  and reports, with its commit message. It never runs `--commit`, the full
  check or an asset run. **The gate is the orchestrator's**: it runs it on the
  staged change as a long run (below), then commits exactly the gated tree with
  the author's message, so in the core lane the orchestrator makes every
  commit; a red gate goes back to the task's author (the same agent,
  continued) with the log's failing part, the author fixes and stages again,
  and the orchestrator runs the gate again. In the build and documents lanes,
  which have no gate, the author commits once its done-criterion holds.
- **The gate command.** Every gate run, `--quick`, `--commit` and the full run
  alike (GS's and LN's among them), starts as
  `cd /home/tibi/Rust/native-theme/.claude/worktrees/v071 && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh <mode>`
  (rule 2 (batch-speed rationale, Appendix); the full run with no mode), `<locks>` being the slots' lock folder,
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/slot-locks`: `slot.sh`
  takes either of its two slots, so without the lock two gates could run at once.
  Every gate this plan names, in a row, a brief or a long-run line, runs in this form.
- **The gate**, in the core worktree through the wrappers, each table row
  marking it **gate**, is the batch's cadence
  (`todo_v0.7.1_batch-speed-rationale.md`, S1), on the ported script (GS):
  during a unit, the agent runs only the touched tests (its targeted
  commands); at a unit's end, `scripts/check_release.sh --quick`, by the
  orchestrator, as a long run until its time is measured (the gate-speed
  specification's E.4 item 4 times it on the clean tree; E1's is the first
  timed on a change, both in the running record); `scripts/check_release.sh --commit`
  in its place every third code unit of the release, counted from the last
  `--commit`, on the release's last code unit before its documents and version
  units (rule 6: here the KDE plan's MK3), after a lock or toolchain change (the warm-up gate; a toolchain
  change shows as a `rustc -V` other than the one recorded at the last
  `--commit`; the `--quick` key's base holds the tool versions, gate-speed
  specification A.2, so `--quick` then re-runs every keyed step), and wherever the
  plan names a unit structural, which rule 24 (batch-speed rationale,
  Appendix) defines as a unit that changes a manifest, `Cargo.lock`, a build
  script, the toolchain or the gate scripts, since "these change what the gate or
  its cache keys are computed from": here GS and LN (the gate scripts) and L1 and L2 (`Cargo.lock`); a merge, a
  new module or a new public API is no reason on its own, so M1 and the KDE
  plan's MK1 are not structural; a documents-only unit runs the five documentation
  checkers and `--quick`; the full run where the plan has it (the close-out,
  the KDE plan's K5), unchanged. A unit is a code unit by rule 11
  (batch-speed rationale, Appendix): it changes anything but comments, rustdoc
  prose, READMEs and Markdown (the files `--quick`'s keys read are a package's
  own, the files it includes, and the key base's: the manifests, the lock,
  `.cargo/`, `docs/*.toml`, the scripts; `docs/todo_v0.7.1_gate-speed-spec.md`,
  A.2, C.4). `docs/platform-facts.md` counts as code:
  two tests read it by `include_str!`
  (`native-theme/src/presets/documented_sizes.rs:55`,
  `native-theme/tests/platform_facts_xref.rs:22`), so a facts-rows unit
  counts. A documents unit stays one only when its change is
  comments, rustdoc prose, READMEs and Markdown documents, even where a crate
  includes them: it runs `--quick`,
  whose keys (A.2, C.4) then re-run those crates' steps, their doctests among them, and it is
  not counted toward "every third" or "the release's last code unit". Both plans share one branch and one count, a unit that commits
  nothing (C on its stop path, L1 or L2 on theirs) is not counted, any
  `--commit` restarts the count, a `--quick`-gated fix does not count toward it
  (rule 11), and the lanes' commits are counted at the merge that brings them,
  one per code unit (M1, the KDE plan's MK1 to MK4): a merge's gate is
  `--commit` where the code units it brings complete a count of three, else
  `--quick`. v0.7.1 has no version unit V (the workspace is already at
  `0.7.1`, `Cargo.toml:17`): its last code unit, the KDE plan's MK3, takes
  `--commit` by rule 6, and its documents unit after it, MK4 (unit 4), the gated
  head (rule 20), takes its cadence gate, the checkers and `--quick`. On the normal path
  (every unit commits):

  | Gate point | Unit | Gate |
  |---|---|---|
  | GS's last commit | the port, structural (rule 24: the gate scripts) | `--commit` (its plan's last gate; the first timed; the warm-up gate; covers P0) |
  | LN | code, structural (rule 24: a gate script's comment and step label) | `--commit` |
  | E1 | code, 1st since | `--quick` (the first timed on a change; E.4 item 4 timed the clean-tree one) |
  | O (H and O) | documents | the checkers and `--quick` |
  | L1, L2 | lock changes, structural | `--commit` each |
  | A, C | code, 1st and 2nd | `--quick` |
  | BS | code, 3rd | `--commit` |
  | M1 | brings BT and F, code units, 1st and 2nd (BX, D and DM are documents units) | `--quick` |
  | E2 | code, 3rd | `--commit` |
  | G5 (G1, G2, G5) | documents | the checkers and `--quick` |
  | the KDE plan's MK1 | brings K1 and KF, code units (KF's `docs/platform-facts.md`), 1st and 2nd | `--quick` |
  | MK2 | brings K2, 3rd | `--commit` |
  | MK3 | brings K3, 1st; v0.7.1's last code unit | `--commit` (rule 6) |
  | a fix KR or KD demands, before MK4 (none on the normal path) | code: it becomes v0.7.1's last code unit | `--commit`, with the GPUI Fast legs where it touches gpui code (rule 6, S1); MK4 stays `--quick` |
  | MK4 | brings K4, documents: v0.7.1's last unit before the close-out, the gated head | the checkers and `--quick` (rule 20) |

  Where neither L1 nor L2 commits, C runs `--commit` (the third code unit
  since LN's) and BS `--quick`. On any other path the same rule decides each
  gate, and the running record names each gate point's count. A failure a later `--commit` finds in a commit
  only `--quick` proved is fixed by a new commit on the branch, never by rewriting a pushed one, and the
  running record names the commit that brought it. `--commit` exits 0, its
  summary naming no failed check and only the release-only checks as "skipped
  (--commit mode)"; `--quick` exits 0, its summary naming no failed check. The
  full run is unchanged (the close-out).
  The script runs `cargo fmt --all` in place (`check_release.sh:39-41`, `:518`)
  and only reports a dirty tree (`:345-346`), while CI's
  `cargo fmt --all --check` fails the push, and seven of its jobs wait on that one
  (`ci.yml:16-29` and each job's `needs: fmt`). So:
  - **a task's gate** runs on the change about to be committed, after it is
    staged (and, for a code task, after `cargo fmt --all --check` has passed);
    `git status --porcelain --untracked-files=no` prints the same before and
    after the gate (the gate changed nothing), and nothing after the commit;
  - **a merge's gate** (M1) runs on the merge commit, and
    `git status --porcelain --untracked-files=no` prints nothing after it.
  `--commit` skips the gpui connector's tests on GPUI Fast (and `--quick` runs
  only what the gate-speed specification gives it), which the
  specification asks for on both cores, so where the commits since the last gate
  change the gpui connector or its showcase, the gate adds
  `cargo test -p native-theme-gpui --features gpui-fast,native,watch --all-targets`
  and `scripts/run_in_runner_env.sh test -p native-theme-gpui --features gpui-fast,native,watch --example showcase-gpui`,
  each part of that gate and so started under its lock,
  `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh …` (rule 2; the
  batch-speed rationale's glossary, *The GPUI Fast legs*); a run of them outside
  a gate, a task's done-criterion, takes `slot.sh` and `lowprio.sh` only.
  A task's gate may skip these two where the task's done-criterion already ran
  them on the same staged tree; the task's report then says so. A merge's gate
  (M1) always runs them.
  A lane's commits are covered by the gate at M1, on the merged tree; in the
  lane only `-p` builds run.
- **Documentation checkers** (no cargo): `scripts/check_platform_facts_rows.py`,
  `check_section_refs.py`, `check_preset_citations.py`,
  `check_wrong_safety_nets.py`, `check_todo_citations.py`, from the worktree's
  root, each with no finding.
- **Texts no gate reads:** G.6's `check_text.py` with P2's quote files, on the
  commit that writes each text, its output in that commit's body.
- **The minimum supported Rust version:** under the gate-speed specification's
  O3 as the maintainer answered it (2026-11-15, the keyed option;
  `docs/todo_v0.7.1_gate-speed-spec.md`, Part M), CI's MSRV checks run in
  `--commit` and in `--quick` of the ported script, the two `+1.88.0` checks in
  `--quick` where anything their key follows changed (their crates' reach and
  included files, the files outside the crates but `QUICK_UNREAD`, the tools,
  the environment, their toolchain), and in CI; the full
  run is unchanged (O1a no). The gpui connector's MSRV is 1.95.0 and the
  specification needs no newer feature; the briefs' rule stays: no language or library feature newer than the edited
  crate's `rust-version`.
- **The full `scripts/check_release.sh`** runs only in the close-out (the KDE
  plan's K5, steps 4 and 8, the first while CI runs on the pushed branch,
  `todo_v0.7.1_batch-speed-rationale.md` S4), where the known `cargo outdated` warning (zbus
  excluded) is expected and the list it prints is recorded in the running
  record.
- **Long runs.** Every gate, and every other run not known to finish within the
  10-minute foreground limit, is the orchestrator's, run with
  `run_in_background` from the stated worktree, through the wrappers, with its
  log and a `STATUS` file at absolute paths in the work folder (as the KDE
  plan's K5 step 6 shows); the orchestrator waits for the exit notice, then
  reads `STATUS` and the log's tail. No `pgrep` loop is used (it matches
  itself); a wait on a file condition uses the Monitor tool with an until-loop.
  Long runs here: the gates (`--commit` took 5 to 12 minutes per commit in
  earlier measurements, 29 cold; `--quick` is a long run until its time is
  measured); the build lane's first builds and its runner
  environment's `--prepare` (the forks' row); L1's and L2's runs of D.6's
  commands with their two warm-up builds; and, in the KDE plan, its first
  builds in the build lane, KR's and K5 step 5's release builds, K5 steps 2, 4,
  6 and 8, and KD's captures. Each of these long runs that runs cargo takes the
  gate lock, `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh …` (rule
  2 (batch-speed rationale, Appendix); the asset run and KD's captures as rule 24
  gives them, under `assets.lock` too); an agent's targeted `-p` commands take
  `slot.sh` and `lowprio.sh` only. The first `--commit` run (GS's last gate) and
  the first `--quick` runs (the gate-speed specification's E.4 item 4 on the
  clean tree, then E1's, the first on a change) are timed and their times
  recorded in the running record. **Every long run**, its first included,
  writes its log and `STATUS` into a new subfolder of its own,
  `<work>/<task>/run-<n>/`, `<n>` starting at 1 and named in the running
  record, so that a leftover `STATUS` of an earlier run (a gate run again after
  a red result, the asset run after a fix, KD's captures, KR's rehearsal) never
  ends a Monitor wait early or is read as the new run's result.
  **A foreground command the Bash tool kills at 10 minutes** is reported by the
  agent, and the orchestrator runs it again as a long run, under `gate.lock`
  (rule 2), as every such long run that runs cargo; after such a re-run
  the orchestrator continues the same agent with `STATUS` and the log's tail.
- **The staged-tree check**, before each gate and again before the commit, by
  the orchestrator: `git diff --quiet` exits 0 (nothing left unstaged);
  `git status --porcelain --untracked-files=all` shows no `??` line but the
  untracked planning documents the running record lists for that worktree, one
  by one by path (P1 records them, P0 updates the list after commit (0); a lane
  worktree's list is empty); and `git diff --cached --name-status` shows
  exactly the task's files: those its specification parts name, with `A` only
  for the new files its row names. No task of this plan adds a new tracked file
  but P0 (the approved documents) and GS (its three new scripts,
  `scripts/step_cache.sh`, `scripts/step_cache_includes.py` and
  `scripts/msrv_checks.sh`). Any difference goes back to the author before the gate runs or the
  commit is made. In the build and documents lanes, where the author commits,
  the author runs the same three checks before its commit and puts their
  output in its report.
- **Slots while an agent works.** While an agent runs cargo, the
  orchestrator's long runs hold at most one of the two slots, so the agent's
  targeted commands are never queued behind two of them.
- **CI** (the pull request's checks): after M1's push, and at the close-out's
  pushes. CI checks no release stamp (only `publish.yml` does), so M1's push is
  expected green; it is the early check of the macOS and Windows GPUI Fast
  builds. A red run emails the maintainer: no push is made that is expected to
  be red. If a run is red, the orchestrator tells the maintainer what failed and
  why, and the author of the failing change fixes it on `v0.7.1-rc2` in the core
  worktree, by §6's rule for a gate that fails on a merged tree (the lanes are
  merged by then); the fix is pushed with
  `git push origin v0.7.1-rc2:v0.7.1-rc2` once the gate passes again.

## 5. Rules every agent brief carries

- The brief gives the specification's header and its *Constraints that hold for
  every part* (no panic paths, both cores, shared files located by content, the
  dry-run figures, Part B's present texts, *Lines that must not move*, the
  backlog's conventions, new citations naming their crate) and the task's
  parts. The parts are the whole task: nothing beyond them, no refactoring, no
  formatting churn. A needed change the parts do not name is reported, not made.
- Every edit is applied by its quoted text; where the text no longer stands, the
  agent stops and reports (*Constraints*), and the orchestrator finds it in its
  present form as §6 says. After E1 a quoted present text may hold E's
  digits: the agent anchors on its words as the constraint says.
- **The fourth gate** (S6 in the batch-speed rationale, *The fourth gate,
  kept*): `every_colour_claim_is_read_at_the_line_it_cites` reads the line each
  colour claim cites in the gpui connector's own files. Every unit that edits
  `connectors/native-theme-gpui/src/` or its examples (E1, A, C, BS, BT, BX, E2,
  and any fix there) keeps the line count above each cited line (the
  specification's *Lines that must not move*; E.5's pinned-lines check where its
  row names it), or re-points the claims that cite a moved line, and their copies
  in `SC/tests.rs`, in the same commit; the test, run on both cores, proves it.
- **The worktree.** The brief names the lane's worktree by its absolute path.
  Every command starts with `cd` into it or uses `git -C` and `--manifest-path`;
  every command that runs a repository script states the worktree it runs in
  (`cd <worktree> && scripts/…`, inside a `bash -c` too).
  The main checkout, where the session starts, is another branch (0.8.0) with
  the same relative paths: nothing is read, run or written there but the two
  plans' work folders under its `.superpowers/sdd/` and, for E1, the read-only
  originals in `.superpowers/sdd/todo_code-review-plan/g4/t2/` (E.1). A check
  that passes in the main checkout proves nothing. The report quotes
  `git -C <worktree> log -1 --oneline`.
- **Clean room (D10).** No code from LGPL or GPL sources. An LGPL-referenced
  function enters only through the clean room (none does in this plan: the
  specification cites no LGPL source, checked 2026-11-15 by a search of it for
  KDE's, Qt's and GTK's). Should a task touch a function an LGPL source is
  cited for, its brief says the implementer opens none of those sources, and
  the specification's description and tests are the clean room's description
  and vectors.
  gpui-component, gpui-base, gpui-pre and GPUI Fast are read for citations in
  Cargo's registry; code that follows an Apache-licensed source closely keeps a
  file-level notice (none is planned).
- Code, tests and rustdoc that name one of v0.7.1's own planning documents (the
  seven K5 step 7 moves) name its archived path, `docs/archive/…`, since the
  close-out moves it after the asset run (rule 22 (c) (batch-speed rationale,
  Appendix); K5's step 0 checks the stamped paths for it).
- Source files are edited with the Edit and Write tools only. A hook that blocks
  an edit is obeyed, never bypassed, and its content is never written another
  way (no `sed`, no shell redirection into a source file); `#[gpui::test]` is not
  `#[test]` for the panic hook (*Constraints*). E1's scripts write digits through
  `remap.py --write`, as E.4 specifies; nothing else writes the tree by script.
- No `unwrap`, `expect`, `panic!`, `unreachable!`, slice indexing or unchecked
  arithmetic outside tests; no `unsafe`. No public item is removed. No invented
  value: a date, a sha, a count, a version or a line read at the run is filled
  in from the run.
- An environment setting goes before the first wrapper
  (`VAR=… <tools>/slot.sh <tools>/lowprio.sh …`) or through `env`
  (`… lowprio.sh env RUSTDOCFLAGS="-D warnings" cargo doc …`, as
  `check_release.sh:1002-1010` does), never as a bare `VAR=…` after a wrapper:
  `lowprio.sh` ends in `exec "$@"`.
- Cargo runs through the tools' slot and low-priority wrappers, in the
  foreground (the capture sessions of the shared close-out, the KDE plan's KD
  and K5, take `slot.sh` and `kde-inhibit` without `lowprio.sh`), at most 16 build jobs (`CARGO_BUILD_JOBS=16`) while the maintainer
  works; `build-dir` and the target directory are never set; `cargo clean` is
  never run (`cargo metadata`, which builds nothing, runs through `lowprio.sh`
  alone, F's coverage script among its callers). In the build lane only
  `-p native-theme-gpui`; its first runner-
  environment run prepares the environment
  (`scripts/run_in_runner_env.sh --prepare`) and an exit 75 there is reported,
  not worked around.
- No `find /` or any other search of the whole file system. Reads and writes
  stay inside the lane's worktree, Cargo's home and build folder (the registry
  for upstream sources), the read-only originals in
  `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_code-review-plan/g4/t2/`
  (E.1, read by E1 alone), the running session's scratch folder (for its own
  temporary files only; no input is taken from a scratchpad but through the
  work folder), the durable copy
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/batch-2026-11-15/`
  (read only), the slots' lock folder
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/slot-locks/`
  (written by `slot.sh` alone) and the two work folders. An agent does not
  write the running record or the plan files; it reports.
- Nothing appears on the maintainer's desktop but in KD and K5, which the
  orchestrator runs.
- A new test is written first and seen to fail. A changelog entry is one
  physical line. The agent stages its own paths by name; in the core lane it
  reports its commit message and the orchestrator commits after the gate (§4),
  in a lane it commits once, after §4's staged-tree check passes in its
  worktree; it does not push, adds no attribution line, and
  reports once, at the end: the commit or its message, the files, its
  commands with their result lines, and everything it was unsure of. Its
  commands run in the foreground; the exception is every command not known to
  finish within 10 minutes (`--commit`, the full check, an asset run, a first
  build in a worktree), which is not the agent's but the orchestrator's long
  run (§4).
- **Something the maintainer must hear before a commit** (Part C's test failing
  without the override; a D.5 bullet that no longer holds; a lock update that
  turns a test red): the agent reports; Part C's failure is not committed
  (Open point 2), the other two follow their part's red path and the
  orchestrator tells the maintainer.

## 6. Limits and stop rules

A **stop**, wherever this plan names one: nothing committed by the task that
met it, and nothing pushed after it until the maintainer has been told and,
where the rule that names the stop asks the maintainer, has answered.
A stop ends when the maintainer has answered where the rule asks; otherwise
the work continues once the maintainer has been told. A quoted text that no
longer stands is found in its present form by the orchestrator: where the
edit's meaning is unchanged, the edit is made at the found text and the record
names it; where it is not, the maintainer is asked.

- **Agents at once:** at most one per lane, plus P2 and the reviewers; at most 4
  agents, or 6 when two of them run no cargo. Both plans' agents count together
  (the KDE plan's lanes start in this plan's worktrees). At most two cargo
  processes at once, which the tools' slots enforce across every process
  started through them: the gate, the build lane's builds, the core-lane
  agents' commands and the orchestrator's warm-ups. At most one Fable agent at a time.
  Once the batch's second lane starts (the widget icons,
  `todo_v0.7.1_batch-speed-rationale.md`, S3 as amended), these limits hold for both
  lanes together: two cargo processes, gates serialized by `gate.lock`
  (rule 2), four agents (six where two run no cargo), one asset run at a
  time, never two Fable agents.
- **A gate failed twice** by the same task is not dispatched a third time: the
  orchestrator takes the task over, re-specifies it, or sends it once as
  `implement-hard`, and says which.
- **A task that needs more than its parts name** (a model field, a change in
  another crate, a decision the rationale did not take) stops, and the
  maintainer is asked. **Upstream moving during the work** (a new GPUI Kit or
  GPUI Fast release, a new advisory) is reported with what it costs before
  anything is changed for it; after L2 the lock is not updated again.
- **E1's unexpected gate failure** follows *Order and gates* (i) and stops when
  it is not a renumbered number.
- **A gate that fails only on a merged tree** (M1; the lanes have no gate):
  the author of the change that fails it fixes it on `v0.7.1-rc2` (in
  the core worktree, while no other agent works there), the fix is reviewed
  again as in §3 by the reviewer of that change, and the gate runs again; the
  push waits for it.
- **Part C's failure** (Open point 2) stops C; A, BS and the lanes go on; DM
  keeps `:3959` open with the dated line, whose text the maintainer sees before
  it is committed.

## 7. The maintainer's part

**Settled by the maintainer, 2026-11-15** (`todo_v0.7.1_batch-speed-rationale.md`):

- S1 to S5, S3's amendment and S6;
- S2's amendment (the next release is merged with its `<prev-tag>`, not rebased
  onto it), the rules 1 to 26 of that document's Appendix, which this plan follows
  (rule 26 does not apply here: P0 adds the documents; later refinements of rules
  2 and 12, S3 and S6 reported to the maintainer), and the three re-readings
  those rules name: rule 20's branch point (the next release starts from v0.7.1's
  documents unit, the KDE plan's MK4, gated `--quick`, not from its last code unit,
  MK3), rule 7's branching from `<prev-tag>` where that tag exists when P2 runs,
  and rule 9's "green" (CI green, the full check's only failure the provenance
  stamp, which only the asset run refreshes);
- the gate-speed specification's open points: O1a and O1b no (the full run
  unchanged), O2 (e) and O3's keyed option (the `+1.88.0` checks build in a
  fresh folder deleted after every run, and `--quick` skips them by key), and
  decision 4 (y), the reviewed registry list (it puts a review on L1 and L2); the
  gate-speed documents are written for these answers, and GS's Task 0 records them
  before P0;
- D10 (a): KDE step 1 ships in v0.7.1, so the close-out without it is not taken;
- the build lane's extra build folder (tens of GB, under
  `/run/media/tibi/work/cargo-build/`), shared with the KDE plan and deleted by
  it, and opening a draft pull request at a branch's first push;
- "When the plan that goes in the 0.7.1 version is ready, without errors, start
  implementing it": the 0.7.1 plans are approved as written, their own additions
  and departures included; here LN's form (Part I: the tests read facts rows by
  text, the preset checker compares nothing) and its place after GS, LN's
  `--commit` as structural (rule 24) for its gate-script comment and step label,
  and the shared close-out in the KDE plan's K5. They are implemented once free
  of errors.
- **Commit (0)** takes v0.7.1's documents (this plan, the KDE step 1 plan, their
  rationales and specifications), the batch-speed rationale and the gate-speed
  specification and plan. The other documents of the cycle (the caret blink's and
  the widget icons' rationales, the `docs/todo_v0.7.x_*` specifications and plans,
  and the egui 0.37 rationale, `docs/todo_v0.7.2_egui-0.37-rationale.md`) are not
  approved yet and stay untracked, outside the work.

**What lands in v0.7.1** (Open point 1): the port and KDE step 1 alone. The icons
and caret work are later releases of their own, the icons in the batch's second
lane and the caret in the KDE lane (`todo_v0.7.1_batch-speed-rationale.md`, S3 as
amended), so B.3 lands; the icons rationale is not in commit (0), so the open
entry and `:3972`'s closing line leave out their sentence about it (the specification's D.3: the sentence is
written only where the icons rationale is in commit (0)). Both lanes start from v0.7.1's gated head (§2.1). The core
worktree `v071` keeping its own build folder after v0.7.1 is no departure: that
folder existed before the batch.

**During the work, the maintainer is told:** Part C's outcome (and, if it fails,
the text of `:3959`'s dated line before it is committed); a lock update that
turned a test red and kept the old version; a D.5 bullet that no longer holds; a
red CI run; anything that stops a task under §6.

**At the close-out and after it:** as the KDE plan's §7 says, for both plans.

## 8. Time

Measured at the last release (the first KDE plan's §8): the full
`check_release.sh` about 23 minutes, the asset script 30 to 37 minutes of which 4
to 6 need the desktop, a CI run 21 to 27 minutes. A `--commit` run and this
plan's tasks were not measured on this tree. The longest chain is the core lane: P1, GS's Task 0, P0, GS (its own plan's units), LN, E1, H,
O, L1, L2, A, C, BS, M1, E2, G1, G2, G5, then the KDE plan's merges and K5; the
build and documents lanes, P2 and the reviews run beside it.

## 9. Cost check

Written 2026-11-16, before the work starts, for v0.7.1's three plans (this one, the
gate-speed plan and the KDE step 1 plan). It is read again at each phase's end: after
GS, at M1 and before K5. Nothing in it is measured unless it says so.

- **What could refute the design, and is it first.**
  - The gate-speed port: a test that fails under nextest's one process per test, or a
    `--quick` that is not clearly shorter than `--commit`. The first shows at commit (ii)'s
    gate, the second at E.4 item 4's timing, both inside GS, the first unit. It is not
    moved earlier: 0.8.0's gate ran the same runner (the batch-speed rationale's §1 names
    its commits; how many of today's tests it covered was not counted), and a late find
    costs one fix round inside GS.
  - The port's completion: Part C's trial, and the two lock updates L1 and L2. Each is a
    unit of its own, and each has its red path written (§2).
  - KDE step 1: KR's rehearsal and KD's captures. They need every unit before them, so they
    cannot come first.
- **What waits for it.** Every unit after GS waits for GS's last gate. The lanes fork after
  O, since E1's citation run edits the files the lanes edit.
- **What is built twice.** Nothing new. The gate-speed port is a port of what 0.8.0 built:
  that branch is parked, so the code is carried over once, not written again.
- **What runs in parallel.** After O: the core lane, the connector lane and the documents
  lane, on separate files, one agent each, with one `-p` build beside the gate (rule 8).
  Before O the core lane is alone.
- **What each agent reads.** Its brief quotes or names the parts of the specification its
  task builds (the row's "Input" column) and the files it edits. It does not read the
  rationales, the plans, the batch rationale or this record. A reviewer gets the diff and
  the same parts. A task goes to a new agent; an agent is resumed only for a small fix of
  its own work.
- **Which tests run when.** An agent runs its row's named tests. `--quick` runs at each
  unit's end and `--commit` where §4's cadence says: eight times on the normal path. The
  full run comes once, in K5. A fix round runs the tests its change reaches, then the gate
  it failed.
- **Documents.** No further review pass is made on the v0.7.1 documents. A doubt goes to
  the unit's own tests or to a trial. The `docs/todo_v0.7.x_*` plans get their last check
  when each release is next to be built, not now.
- **Where a new session begins.** After P0 (the build phase starts, the orchestrator on
  Opus); after GS; after M1; before K5. The state is in the two running records and the
  project memory.

## Running record

The orchestrator's running record of this plan, copied at the close-out (K5 step 7) from `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.1_gpui-kit-0.9.1-plan/record.md`; its headings are one level down.

## Running record: v0.7.1, the GPUI Kit 0.9.1 plan (with the KDE step 1 plan's shared steps)

Only the orchestrator writes this file.

### 2026-11-16 — P1, the tools (done)

- Worktree: `/home/tibi/Rust/native-theme/.claude/worktrees/v071`, branch `v0.7.1-rc2`, HEAD
  `0fa28027f7f8ccb376f87e14d85a6e7acce84096`; `git status --porcelain --untracked-files=no` prints nothing.
- `<tools>` = `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/tools/`: `slot.sh`
  and `lowprio.sh` copied from `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/` and read.
- `grep -n '^D=' <tools>/slot.sh` prints
  `7:D=/home/tibi/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/slot-locks` (`<locks>`, created).
- `lowprio.sh` gained `export CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16`; its `renice -n 19 -p $$` and
  `ionice -c 3` are kept.
- Slot test: two `<tools>/slot.sh sleep 8 &` took slots 1 and 2; a third `<tools>/slot.sh true` printed its
  slot line after 20 s (its retry interval), when a slot was free. (The plan says `sleep 30`; 8 s was used,
  the behaviour shown is the same.)
- `<tools>/slot.sh <tools>/lowprio.sh sh -c 'echo "$CARGO_BUILD_JOBS $RUST_TEST_THREADS"; ps -o ni= -p $$'`
  printed `16 16` and `19`. `ls <locks>`: `slot1.lock`, `slot2.lock`.
- KDE plan K0 (1): `gates/rehearsal.sh` copied from `scratch-keep/batch-2026-11-15/gates/` into
  `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_kde-open-items-step1-plan/gates/`. Neither
  specification names another input in a scratch folder (`grep` for `scratch-keep` and `scratchpad`).
- Hook trial: the Write of `<core worktree>/native-theme/src/hook_trial.rs` holding `fn f() { x.unwrap() }`
  was refused ("BLOCKED: Code contains panic-prone or unsafe patterns … `.unwrap()`"); `git status
  --porcelain --untracked-files=all` shows no `hook_trial.rs`.
- Pull request: `gh pr view 10 --json state,headRefName,isDraft,title` printed
  `{"headRefName":"v0.7.1-rc2","isDraft":false,"state":"OPEN","title":"v0.7.1: the gpui connector on GPUI Kit 0.9.1"}`.
- Another project's `cargo test` (cascatura, not this session's) was running on the machine during P1; it
  does not take this batch's slots.

#### Untracked planning documents in the core worktree (22)

- docs/issues/0004_kde-open-items-rationale.md
- docs/todo_v0.7.1_batch-speed-rationale.md
- docs/todo_v0.7.1_caret-blink-rationale.md
- docs/todo_v0.7.1_gate-speed-plan.md
- docs/todo_v0.7.1_gate-speed-spec.md
- docs/todo_v0.7.1_gpui-kit-0.9.1-plan.md
- docs/todo_v0.7.1_gpui-kit-0.9.1-rationale.md
- docs/todo_v0.7.1_gpui-kit-0.9.1-spec.md
- docs/todo_v0.7.1_gpui-widget-icons-rationale.md
- docs/todo_v0.7.1_kde-open-items-step1-plan.md
- docs/todo_v0.7.1_kde-open-items-step1-spec.md
- docs/todo_v0.7.2_egui-0.37-rationale.md
- docs/todo_v0.7.x_caret-blink-plan.md
- docs/todo_v0.7.x_caret-blink-spec.md
- docs/todo_v0.7.x_gpui-widget-icons-plan.md
- docs/todo_v0.7.x_gpui-widget-icons-spec.md
- docs/todo_v0.7.x_kde-open-items-steps2-3-plan.md
- docs/todo_v0.7.x_kde-open-items-steps2-3-spec.md
- docs/todo_v0.7.x_kde-open-items-steps4-5-plan.md
- docs/todo_v0.7.x_kde-open-items-steps4-5-spec.md
- docs/todo_v0.7.x_kde-open-items-steps6-7-plan.md
- docs/todo_v0.7.x_kde-open-items-steps6-7-spec.md

### 2026-11-16 — the last document fixes before P0

- The gate-speed documents' two findings of 2026-11-15 (the self-test's call form; the include rule
  inside macros) and six minor ones were applied by a `worker` and the diff read by the orchestrator. No
  further review pass (the maintainer's document rule of 2026-11-16). A third lock-list point was taken
  with them: E.4 item 3's `cargo test` and nextest runs start under `gate.lock`.
- Small fixes to the two v0.7.1 plans and rule 24 of the batch rationale (the fifth gate script), read by
  the orchestrator.
- §9 *Cost check* added to this plan (the maintainer's rule of 2026-11-16).

### 2026-11-16 — P0, commit (0) (done)

- `e866ea6c28c5ae8af17c58068e902d4923ca2c06` on `v0.7.1-rc2`, not pushed. This is the `<sha>` of commit (0).
- New files (9): docs/issues/0004_kde-open-items-rationale.md, docs/todo_v0.7.1_batch-speed-rationale.md,
  docs/todo_v0.7.1_gate-speed-plan.md, docs/todo_v0.7.1_gate-speed-spec.md,
  docs/todo_v0.7.1_gpui-kit-0.9.1-plan.md, docs/todo_v0.7.1_gpui-kit-0.9.1-rationale.md,
  docs/todo_v0.7.1_gpui-kit-0.9.1-spec.md, docs/todo_v0.7.1_kde-open-items-step1-plan.md,
  docs/todo_v0.7.1_kde-open-items-step1-spec.md.
- Status lines set in the two rationales as P0's row gives them; the second Status sentence removed.
- The five documentation checkers exit 0 on the staged tree (0 findings each).
  `git status --porcelain --untracked-files=no` prints nothing after the commit.
- KDE plan: there is no unit 0 (its three documents are in commit (0)).

#### Untracked planning documents left in the core worktree (13)

- docs/todo_v0.7.1_caret-blink-rationale.md
- docs/todo_v0.7.1_gpui-widget-icons-rationale.md
- docs/todo_v0.7.2_egui-0.37-rationale.md
- docs/todo_v0.7.x_caret-blink-plan.md
- docs/todo_v0.7.x_caret-blink-spec.md
- docs/todo_v0.7.x_gpui-widget-icons-plan.md
- docs/todo_v0.7.x_gpui-widget-icons-spec.md
- docs/todo_v0.7.x_kde-open-items-steps2-3-plan.md
- docs/todo_v0.7.x_kde-open-items-steps2-3-spec.md
- docs/todo_v0.7.x_kde-open-items-steps4-5-plan.md
- docs/todo_v0.7.x_kde-open-items-steps4-5-spec.md
- docs/todo_v0.7.x_kde-open-items-steps6-7-plan.md
- docs/todo_v0.7.x_kde-open-items-steps6-7-spec.md

### 2026-11-16 — the untracked planning documents: a 14th

The planning session wrote `docs/todo_v0.7.2_gpui-kit-0.10.0-rationale.md` into the core worktree during GS
T-ii, a later-release planning document. It is left untracked and unchanged, and the staged-tree check allows
it: the list above holds 14 from here, this one included.

### 2026-11-16 — the maintainer's changes to the order (speed)

Four departures, approved in the session's question dialog ("Split R, start now", "E4 beside R", "LN in own
worktree", "L1+L2 one gate"); the gate-speed record says what each means. For this plan:
- LN is built in its own worktree beside GS and merged after GS, gated by `--commit` on the merged tree.
- L1 and L2 share one `--commit` gate, split on a red result.

### 2026-11-16 — P2, the checks (done, early)

P2 had no dependency on GS, so it ran during GS T-iv. A `worker` built it; no cargo, no repository change.
- `<work>/checks/check_text.py` and `<work>/checks/check_moves.py`: `diff` against the blocks cut from the
  spec by `<work>/checks/p2/extract.py` prints nothing for either; `cmp` says identical; `py_compile`
  passes.
- 54 quote files `<work>/checks/quote-*.txt` for G.6's 51 items (item 35 has three, item 38 two). The listing
  is `<work>/checks/p2/listing.md`.
- `check_text.py` run against the spec itself as a transcription check: 50 `ok`, 4 `MISSING`, each of the
  four made from more than one place in the spec; their pieces each pass.
- The worker's judgments:
  - E.5's two weak fragment quotes (`check_showcase_parity.py`, `styles.rs`);
  - the `proposals/README.md` quote's "…" filled with the present start the spec quotes;
  - the open icons entry ends at "asks first." (the icons rationale is not in commit (0));
  - Part H's `<sha>` kept split.
- Possible gaps in G.6's list, for the tasks that write those texts to settle (not quoted):
  - G.1's README entry ending;
  - A.1's `SC/app.rs` field doc;
  - A.3's `diff` doc addition;
  - G.2's `:264` count line;
  - A's quoted Widget Info notes.

### 2026-11-16 — LN in its own worktree: started

- Worktree `/home/tibi/Rust/native-theme/.claude/worktrees/v071-ln`, branch `v0.7.1-rc2-ln`, made from
  `d733a440` (GS commit (iii)) with an absolute path.
- Hook trial: a Write of `<v071-ln>/native-theme/src/hook_trial.rs` holding `fn f() { x.unwrap() }` was refused
  ("BLOCKED: Code contains panic-prone or unsafe patterns … `.unwrap()`"); `git status --porcelain
  --untracked-files=all` prints nothing.
- Warm-up (the orchestrator's long run, under `gate.lock`):
  `cargo test -p native-theme --all-targets --no-run && cargo clippy -p native-theme --all-targets -- -D warnings`.
  Result: `<work>/LN/warmup/run-1`, STATUS 0, 25 s.
- Its build folder, to be deleted after LN's merge: `/run/media/tibi/work/cargo-build/7d/3eec415c1d1637`.
- Dispatched: one `implement` agent for Part I. It commits in its own worktree after the staged-tree check;
  the orchestrator merges after GS and runs `--commit` on the merged tree.
- Expected merge conflicts with GS commit (iv): in `scripts/check_release.sh`, `scripts/README.md` and
  `CHANGELOG.md`; resolved by the quoted texts.

### 2026-11-16 — LN: committed in its worktree, in review

- `c43a0d2c` on `v0.7.1-rc2-ln` ("gates: the facts-row gates find rows by text (Part I): …"), not pushed; the
  eight files of Part I, all `M`; 225 insertions, 527 deletions (the checker's comparison removed).
- I.5, as the agent reported:
  - the five tests fail on their temporary edits, each failing line in the commit body;
  - `--self-test` failed once on the old `CITATION`: "got {'citations': 4, …} want {'citations': 11, …}";
  - a line inserted at the top of the facts file breaks nothing: `platform_facts_xref` 9 passed,
    `documented_sizes` 13 passed, the checker exits 0;
  - the new form is read: kde-breeze stays at 287, 16, 267, 4;
  - after the restores, `git diff --quiet` over the facts file and the presets exits 0;
  - on the final tree: the checker's counts per preset (adwaita 133, ios-13 15, kde-breeze 287,
    macos-monterey 204, macos-tahoe 237, material-3 240, windows-11 258, the colour schemes 0);
    `self-test ok`; clippy `-D warnings` and strict-panic for native-theme pass; `fmt --check` passes; the
    other four checkers report 0 findings.
- Beyond the spec's letter, as the agent reported:
  - the checker's removed helpers and the `findings` → `count_citations` rename;
  - an extra self-test case;
  - its own output format;
  - the README section in its own wording;
  - the step label change, which makes a recorded cache entry miss once.
- Review: an Opus `review` agent (about 750 changed lines, above the orchestrator's size), given the diff and
  Part I. The verdict was merge, with no must-fix:
  - **should-fix 1**: the checker's self-test no longer exercises the kind rules (`LEADING` with the new form,
    `MARKER`, keys taken from a block's first line). It goes back to the author as a new commit, with each rule
    shown to fail under a deliberate break.
  - **nit 2, not taken**: `facts_row` (documented_sizes) stops at the next `### ` only, as I.2 says, so §2.29's
    range runs into the Appendix until `### macOS`. Those lines hold no row today. Recorded; the spec's wording
    is kept.
  - **nit 3, taken with 1**: `named()` returns a list nobody reads (I.3, "no dead code is left").
  - Checks 1 and 3 to 6 found nothing. The review confirmed the lookups cannot match a partial key or the
    wrong section, the CHANGELOG line is I.4's byte for byte, and the old step label is used nowhere else.
- Fix: `92ec9297` "gates: check_preset_citations.py's self-test covers each citation kind: …", by the same
  agent (about 200 k tokens), as a new commit.
  - The self-test gained cases (a) to (c); `want` is now 16 citations, 4 named, 9 value, 3 rows.
  - Four deliberate breaks each failed the self-test: `LEADING` reverted, `leading` forced `False`, `MARKER`
    not stripped, the keys not taken from the first line.
  - `named()` returns a bool.
  - The per-preset counts are unchanged, and the four other checkers report 0 findings.
  - Read by the orchestrator (30 lines): equivalent where the old code returned a list, possibly empty; no
    finding.
- **LN is ready to merge** (`v0.7.1-rc2-ln` at `92ec9297`) once GS closes, with `--commit` on the merged tree.

### 2026-11-17 — GS closed; LN merged and gated

- GS's last commit: `4e03057bcebb3acaadb3cab7de37b4f5ce58e532` (E.4 item 1 run `<work>/GS/e4/item1/run-2`;
  review-hard closed). LN merged as `925125a7366f519f976f9dc678b2170f742ccb87`. E.4 item 1's `--commit` on
  `925125a7` is GS's last gate and LN's structural gate (the maintainer's speed grant of 2026-11-16).
- The branch's first timed `--commit`: 368 s on `925125a7`, the caches empty, the build folder warm. The
  first, on `30a53e29` after other builds, took 942 s. The clean-tree `--quick` (E.4 item 4): 62 s.
- `rustc -V` at this `--commit`: `rustc 1.100.0 (0ef1bc386 2026-11-09)`, the toolchain recorded for the
  cadence (a change means the warm-up gate again).
- E1's precondition checked (2026-11-17): `git diff --name-only e866ea6c HEAD -- <E.2's pathspecs>` prints
  nothing (0 lines).
- Cadence: the count restarts at LN's `--commit` (item 1). E1 is the first code unit after it, gated by
  `--quick`.
- `BASE_REV` for E1 is LN's last commit on the branch, the merge `925125a7`. E1's row first checks that
  `git diff --name-only e866ea6c HEAD -- <E.2's pathspecs>` prints nothing.

### 2026-11-17 — E1 started (session native-theme-31)

- New orchestrating session `native-theme-31` (Opus); `native-theme-0c` is gone (ListAgents), the planning
  session `native-theme-86` idle.
- E1's precondition checked again: `git status --porcelain --untracked-files=no` 0 lines; the E.2-scope diff
  against `e866ea6c` 0 lines. `BASE_REV` = `925125a7366f519f976f9dc678b2170f742ccb87`.
- Untracked planning documents in `v071`: the 14 listed above plus
  `docs/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan.md` and `docs/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-spec.md`
  (the planning session's, 2026-11-16/17): 16.
- Warm-up of both cores' connector test builds as a long run, `<work>/E1/warmup/run-1/`, under `gate.lock`.
- E1 dispatched to a `worker` agent.

### 2026-11-17 — E1 committed; H and O committed, their unit's gate pending

- Warm-up `<work>/E1/warmup/run-1`: STATUS 0, 32 s (both cores already warm).
- E1 by a `worker`: 559 numbers written in 33 files (E.2's scope only); `counts.txt` rule 1 411, rule 2 55,
  rule 3 81; gates 1 and 2 pass before and after the write; gate 4 1527 checked, 1185 hold, 277 set aside,
  65 fail (the dry run's figures), all 65 in `set-aside-gate4.txt` (61 distinct numbers); `lookup.py --check`
  10 of 10; no `REVIEW_ASIDE` entry (126 candidate chains read for past tense, list
  `<work>/E1/pasttense-candidates.txt`); E.4 step 6's three tests pass on both cores; fmt clean.
  T1_COMMITS/TASK1 computed by `git rev-list` at run time (43 commits at BASE_REV).
- Orchestrator's reading: E.7's word-diff over the staged change shows only numbers and the overlapping-range
  splits (`-N-`/`+-N`), no version label changed; the counts and the full set-aside list are in the body.
- **E1's gate**: `--quick` `<work>/E1/gate/run-1`, STATUS 0, **455 s** (the first `--quick` timed on a change;
  0 unchanged, since E1 touches four crates), `147 checks · 147 ✅`; porcelain the same before and after.
  GPUI Fast legs `<work>/E1/gate/run-1-fast`: STATUS 0, 21 s (lib 399, seams 18, showcase 332 twice).
- **E1: `42b1437b362e1edc5780e00dd2bba6cdbab30e7f`.** Cadence: 1st code unit since LN's `--commit`.
- **H: `30a83933`**, by the orchestrator (a verbatim paste of Part H's text, `<sha>` = `e866ea6c`; cheaper
  than an agent; `check_text.py` ok, five checkers 0). **O: `e22dc00c`**, likewise (D.3's open entry
  without the icons sentence, §7; `check_text.py` ok for both quotes, five checkers 0).
- O's `--quick` (`<work>/O/gate/run-1`) was stopped at its start: the unit takes a third commit first, N5's
  spec-text corrections and the `CARGO_ENCODED_*` backlog entry (gate-speed record, decision 4), by a
  `worker`; one `--quick` on that staged change then covers H, O and N5.

### 2026-11-17 — the documents unit H, O, N5 gated; the forks

- N5 by a `worker` (continued once for the five further contradictions it found, decision 13, M.1, C.1/C.2,
  C.5, D.2's header, and three omissions): the gate-speed specification follows fix-1's code; the backlog entry
  on `CARGO_ENCODED_RUSTFLAGS`/`CARGO_ENCODED_RUSTDOCFLAGS` (Core API). The orchestrator measured Cargo's
  precedence on 2026-11-17 (cargo 1.100.0, throwaway crate in the scratchpad): `RUSTFLAGS=-Dwarnings` fails an
  unused variable, with `CARGO_ENCODED_RUSTFLAGS=` it exits 0; `RUSTDOCFLAGS=-Dwarnings` fails a broken
  intra-doc link (101), with `CARGO_ENCODED_RUSTDOCFLAGS=` it exits 0. Diff read by the orchestrator.
- **The unit's gate**: `--quick` `<work>/O/gate/run-2` on N5's staged change (tree holding H and O), STATUS 0,
  **60 s**, `82 ✅ · 65 unchanged`, porcelain the same. Five checkers 0 on each commit.
- **N5: `15b26f649e44d243500d04497d62bf06541db0eb`.** Documents unit: not counted for the cadence.
- **Forks** from `15b26f64` (the unit's last commit, in place of O's: N5 joined the unit): `v071-lane` on
  `v0.7.1-rc2-gk-connector`, `v071-docs` on `v0.7.1-rc2-gk-docs`; the hook trial refused in both, both clean.
  The build lane's first builds, clippy warm-ups and `--prepare`: long run `<work>/forks/run-1`.
- D dispatched to a `worker` in `v071-docs` (no cargo).

### 2026-11-17 — L1 committed; L2 under the shared `--commit`; BT and D running

- Forks' first builds `<work>/forks/run-1`: STATUS 0, 383 s (`--prepare` included, silent under `set -e`).
- L1 and L2 by the orchestrator (D.6's commands and gates; the worker's part, the message, written from the
  outputs: cheaper than an agent).
- **L1: `cdd0c3c919ab4bf1d5a9e2ee1ec7f54cd4c0f697`**, unicode-properties 0.1.3 -> **0.1.5** (version and
  checksum lines only; requirements "0.1.3" in gpui-fast-web 0.1.15, gpui-pre-web 0.3.12, rustybuzz 0.20.1;
  `build = false`). Checks `<work>/L1/run-1/checks.log`: audit 0 (1083 deps), lib 401 / 399, seams 18,
  showcase 332 on both cores, runner env 332 each; 321 s. Its `--commit` is L2's (maintainer's "L1+L2 one gate").
- L2: taffy 0.15.0 -> **0.15.1** (version and checksum only; gpui-fast 0.1.15 and gpui-pre 0.3.12 require
  "0.15.0", caret; `build = false`); staged; checks then `--commit` as one long run, `<work>/L2/run-1/`.
- Registry note: gpui-component 0.10.0 and gpui-fast 0.1.16 are in Cargo's registry (the v0.7.2 trials'); no
  new upstream for 0.7.1's purposes (v0.7.2 plans 0.10.0).
- BT dispatched to an `implement` agent in `v071-lane`; D running in `v071-docs`.

### 2026-11-17 — BT committed and reviewed; BX running

- **BT: `cd770099`** on `v0.7.1-rc2-gk-connector` (the `implement` agent's commit): both tests red before on
  gpui-pre (`tests.rs:232`, `:277`: "the resolver was asked for icons/check.svg: []"), green after on both
  cores (lib 403 / 401; `--no-default-features` 294); clippy three ways; strict panic `--lib` and showcase,
  both cores; fmt; pinned-lines check `ok` for both hunks; `check_text.py` ok. Code unit (counted at M1).
- BT's `review`: **merge**; no must-fix, no should-fix. One nit, taken: assert `native-checkbox-mark` is laid
  out in the kde-breeze half (proves the stroked arm drew). The reviewer verified per-test `App` isolation of
  the resolver global (gpui-pre 0.3.12 and gpui-fast 0.1.15 `App::new_app` per `#[gpui::test]`).
  The nit goes to BT's agent as the lane's next commit after BX's.
- BX dispatched to a `worker` in `v071-lane` (after BT's commit).

### 2026-11-17 — BX and D committed, read

- **BX: `e49c48f6`** on `v0.7.1-rc2-gk-connector` (worker): B.2, B.4's README cell, B.5, B.6, B.9's comment
  (anchored on its words, digits `596-598` already E's), B.10; lib 403 / 401, clippy three ways, `cargo doc`
  `-D warnings` with and without private items, fmt; pinned-lines `geometry.rs` L=772 ok; seven `check_text.py`
  ok. Orchestrator read the diff: the spec's texts. Documents unit (not counted).
- **D: `43c8a9d4`** on `v0.7.1-rc2-gk-docs` (worker): 45 D.2 dated lines + 4 D.4 entries, as D.1; `lookup.py
  --check` 10/10; all eight ROADMAP bullets still hold (no D.5 report); four `quote-D4-*` ok; five checkers 0.
  Worker's own decisions in the body's Notes: nine entries without line numbers dated from its reading of the
  source; `:2433` also gives `input_style`'s lines; `:2449`'s kept G4 line now points at a `:152-166` the
  rewritten sentence no longer holds (the spec keeps the dated line; left); `:3893`'s editor range ends at the
  listener's `});`. Orchestrator read the diff. Documents unit.

### 2026-11-17 — L2 and the shared `--commit`; A dispatched; the build lane to the KDE plan

- L2's checks `<work>/L2/run-1/checks.log`: audit 0, lib 401 / 399, seams 18, showcase 332 both cores, runner env
  332 each; 301 s. **The shared `--commit` (L1+L2)**: `<work>/L2/run-1/gate.log`, STATUS 0, **666 s**,
  `147 checks · 147 ✅ · 0 unchanged`; porcelain the same; no refusal line in its log (no registry list entry).
- **L2: `30b3e6d548f08925d0a9456b458c621e0162b485`.** Cadence: count restarts here.
- BT's nit: **`1a3e352b`** on `v0.7.1-rc2-gk-connector` (assert `native-checkbox-mark`; lib 403 / 401, fmt,
  clippy). The connector branch is complete for M1: `cd770099`, `e49c48f6`, `1a3e352b`.
- A dispatched to an `implement` agent in `v071`.
- The build lane handed to the KDE plan: `v071-lane` on `v0.7.1-rc2-kde` at `30b3e6d5` (its record has K0).

### 2026-11-17 — A committed; its review, C and F running

- A by an `implement` agent: tests 2 and 4 (and 3) red before (`no quad … painted in its 'decorated line,
  warning at 20%' claim`; `no square quad inside the Markdown …`; `left: (0, 0) right: (1, 1)`), all eight
  green after on both cores, no GPUI Fast `cfg`; showcase 340 both cores, lib 401 / 399, clippy three ways,
  strict panic both, coverage passes, fmt, five `quote-A*` ok. The agent once edited `SC/app.rs` by a shell
  script (the brief forbids it; no hook pattern in it), reverted it and redid it with Edit; nothing staged came
  from the shell. Added helpers `quads_filled_in`, `settled_claim` (sent to review).
- **A's gate**: `--quick` `<work>/A/gate/run-1`, STATUS 0, 279 s in all, `--quick` 4 m 18 s, `95 ✅ · 52
  unchanged`; GPUI Fast legs: 399, 18, 340, 340; porcelain the same.
- **A: `47f28eff6a6543f6702f65d0402f9b4cd8c8d1d8`.** Cadence: 1st code unit since L2's `--commit`.
- A's `review` dispatched (reads through git). C dispatched to an `implement` agent in `v071`.
- The documents branch merged `v0.7.1-rc2` at A: **`a4c87408`** (clean). F dispatched to an `implement` agent in
  `v071-docs` (no cargo but `cargo metadata` through `lowprio.sh`).

### 2026-11-17 — C committed (Part C passes: the workaround is gone); A's must-fix

- **Part C's outcome (for the maintainer, §7): both pass.** `the_theme_settings_switch_the_preset` passes on
  gpui-pre and GPUI Fast without `PresetDelegate::on_will_change`; the removal is committed and the backlog entry
  (`:3959`) closes in DM. C by an `implement` agent; diff read by the orchestrator (C's reviewer): exactly Part C.
- **C's gate**: `--quick` `<work>/C/gate/run-1`, 163 s in all, `87 ✅ · 60 unchanged`, 2 m 26 s; GPUI Fast legs
  399, 18, 340, 340; porcelain the same.
- **C: `64dc16544327254ccbdc7b288e54c195e61c9ec3`** (the author's subject-only message amended, before any push,
  to carry the evidence body; first sha `00ee756f`). Cadence: 2nd code unit since L2's `--commit`.
- A's `review`: merge after one **must-fix**: test 8 measures the info wrapper, which `v_flex` stretches, so it
  passes without `.w_full()`. Taken with nits 1 (shared quad helper), 2 (the duplicated `signal` note) and 5
  (crate name in a citation); nits 3 and 4 left (harmless; test 4 is not vacuous, per the reviewer's trace).
  Sent to A's author as the core lane's next commit after C. Its gate: covered by BS's `--commit` (the fix is
  committed after its targeted tests, then BS is staged on top), as the speed rule "merge gate runs" allows;
  the fix then goes back to the same reviewer.
- K1 (KDE unit 1) committed in `v071-lane` (the KDE record); its `review` and K2 dispatched.

### 2026-11-17 — A's must-fix committed; BS dispatched

- A's author (continued, about 230 k tokens): test 8 reads the waveform's own box through a test-only
  `InfoRegistry::widget_bounds` (`SC/info/registry.rs`, filled by the wrapper's `on_children_prepainted`, which
  test builds now install for every target; release builds unchanged); also asserts the waveform's right edge
  is its row's. Red with `.w_full()` removed, both cores ("does not reach its row's right edge … left: 409px
  right: 1268px"); green after: showcase 340 ×2, lib 401 / 399, clippy three, strict panic two, fmt,
  `quote-A2` ok. Nits 1, 2, 5 taken. Files: `SC/info/inputs.rs`, `SC/info/registry.rs`, `SC/tests.rs`.
- **A's fix: `cf3895f4a9f57ca55670dc4d0341b317f118df8d`**, committed after its targeted tests; its gate is BS's
  `--commit` (on BS's staged tree), and it went back to A's reviewer.
- BS dispatched to an `implement` agent in `v071`.

### 2026-11-17 — F committed; its review running

- **F: `7b8b248e`** on `v0.7.1-rc2-gk-docs` (`implement`): coverage `gpui discovered 159 → 168` (+9), shown
  124 → 128, excepted 35 → 40, missing 0; `--self-test` passes; every F.5 case fails with its part removed
  (applied in memory; listed in the body); five checkers 0; `check_text.py` ok F3-1..5, F4-0..5, F7-1..2.
  Author's choices beyond the spec: a declared module whose file is missing, an unreadable `use` tree, and
  `super` at the root raise `Failure` (exit 2); one docstring sentence on `--self-test`. Noted, left: no F.5
  case covers `\s*\(` in the GridLayout `GPUI_VIA` pattern. The rule adds about 2.7 s. Code unit (counted at M1).
- F's `review` dispatched (may run the script read-only in `v071-docs`).

### 2026-11-17 — A's fix re-reviewed

- A's reviewer on `cf3895f4`: **merge**; the must-fix is resolved (traced: the first prepainted child is the
  waveform; red at 96 vs 955 px; release build unchanged). **Should-fix, taken:** in test builds every info
  wrapper installs `on_children_prepainted`, and on GPUI Fast such a listener calls `note_uncarried`
  (gpui-fast 0.1.15 `elements/div.rs:2112-2113`), which marks the layer content-anchored (`fast/layers/
  invalidate.rs:801-812`, `policy.rs:477`, `:494-511`): no page's scroll layer but Basic's can be composited in
  GPUI Fast tests, unlike the shipped showcase. Fix: the test-only listener only on the targets a test measures
  (the liquid waveform), and the field's doc and test 8's doc (nit 1: the right-edge check) say so. To A's author
  after BS commits (one builder per tree); covered by M1's `--quick`; then back to the same reviewer.

### 2026-11-17 — BS's review; F's review and its fixes

- BS's `review` (on the staged change, while its gate ran): **commit**; no must-fix, no should-fix. Nit, left
  (needs a spec change): the `receivers` note omits `questionnaire/components.rs:1219` (B.9's connector comment
  cites it). The reviewer traced B.8's three claims, B.1's height, B.3's resolver callers, B.4, B.9, B.15.
- F's `review` (ran the gate and self-test read-only; mutations on a copy, `<work>/F-review/`): **merge after
  must-fixes**: (1) no self-test reaches `BaseModules.lookup`'s glob branch (it finds ResizablePanel and
  ResizablePanelGroup; removing it leaves `--self-test` green); (2) F.1 step 2's one-hop publicity misses
  gpui-component 0.9.1's own longer chains (`mod time; pub use time::{calendar, …}`; `styled::*` →
  `component_traits`), dropping 12 names silently (none renders today). Should-fix: (3) an unknown first segment
  in gpui-base taken as external silently; (4) untested `GPUI_VIA` sub-patterns and one `walk` branch. Minors.
- **Orchestrator's decision (for the maintainer's report):** must-fix 2 is a gap in the specification's rule
  F.1 step 2, corrected by its own goal (N7: find what gpui-component makes public; Rust's publicity is
  transitive): reachable modules as a fixpoint, names carried along every chain. Settled by logic, not a
  preference, so the work goes on. Taken: must-fixes 1, 2; should-fixes 3 (external only if a declared
  dependency or std/core/alloc, else `Failure`), 4 (and the author's `grid_layout_sample(` gap); minors 2, 3, 4,
  5. Minor 1 (`.scrollbar(` also matches `MessageScroller::scrollbar(bool)`; the showcase has none) reported
  only: the pattern is the spec's. Sent to F's author as a new commit in `v071-docs`.

### 2026-11-17 — BS committed under `--commit`

- BS by an `implement` agent: B.1, B.3, B.4's `GEOMETRY_NOTES`, B.8 (three new claims), B.9's layout note, B.15;
  lib 401 / 399 with the claim and prose gates, showcase 340 ×2, clippy three, strict panic two, `cargo doc`
  `-D warnings`, fmt, eight quotes ok. Reviewed before the commit (above): commit.
- **BS's gate**: `--commit` `<work>/BS/gate/run-1`, STATUS 0, 534 s in all, `--commit` **8 m 38 s**, `147 checks ·
  113 ✅ · 34 unchanged`; GPUI Fast legs 399, 18, 340, 340; porcelain the same; `rustc 1.100.0 (0ef1bc386
  2026-11-09)`, unchanged. It covers A's fix `cf3895f4` beneath it.
- **BS: `9e02eaa1cf8364f3d6415d7bcef195a1dc2fe590`.** Cadence: 3rd code unit since L2 → `--commit`; the count
  restarts here.
- A's should-fix (listener only on measured targets; test 8's doc) sent to A's author as the next core commit.
  Then M1: BT and F are the code units it brings (1st and 2nd) → `--quick`, which also covers A's fix-2.

### 2026-11-17 — F's review fixes committed; back to its reviewer

- **F's fixes: `ce6b914e`** on `v0.7.1-rc2-gk-docs` (F's author, about 265 k tokens): must-fix 1 (the `rz`/`Panel`
  glob case), must-fix 2 (`component_exports`, publicity as a fixpoint; newly counts 12 names: time::calendar 4,
  time::time_field 5, component_traits 3, none a widget), should-fixes 3 and 4, minors 2–5. Gate unchanged
  `discovered 168 shown 128 excepted 40 missing 0`; `--self-test` passes; 31 mutations all fail the self-test;
  five checkers 0; F's quotes ok. External crates: every dependency kind of gpui-base (dev included; harmless,
  `#[cfg(test)]` modules are stripped). Re-review by F's reviewer running.

### 2026-11-17 — A's fix-2 committed

- **A's fix-2: `b22b431c09bbcd259bdc21bc5d7157460172ad98`** (A's author): a test-only `MEASURED_WIDGETS` limits
  the listener to the liquid waveform; listed elements as before; release builds `listed.is_some()` alone; test
  8's doc names the right-edge check. Red with `.w_full()` removed, both cores; showcase 340 ×2, lib 401 / 399,
  clippy three, strict panic two, fmt. Committed after its targeted tests; gated by M1's `--quick`; re-read by
  A's reviewer (running).
- The documents branch's merge of `v0.7.1-rc2` waits for F's re-review (the reviewer may run the gate read-only
  in `v071-docs`); it will take the core at `b22b431c`.

- A's reviewer on `b22b431c`: **merge**; the compositing concern is resolved for every page but Inputs; the release
  build is as at `47f28eff`; test 8 proves its claim. Nit left (a doc clause: the measured target's own page,
  Inputs, still loses GPUI Fast compositing in test builds). **A's review is closed.**

- F's reviewer on `ce6b914e`: **merge**; every earlier finding resolved (re-ran its mutations; real discovery
  still the nine names; 12 more statement names counted, none dropped). Should-fix taken: three self-test cases
  for branches 0.9.1 never exercises (a named item through private globs; a whole gpui-base module and a named
  item re-exported from an unreachable private module). Nits taken: the redundant `own_module` branch (remove or
  comment), a 104-column docstring line, and the spec's F.1 step 2 / step 3 text brought in line with the code
  (as N5 did for the gate-speed spec). To F's author (about 265 k tokens: a small fix to its own work).

- **F's last round: `43fa2f6f`** (three cases, each red under its mutation; the redundant branch removed; docstring
  rewrapped; the spec's F.1 steps 2 and 3 describe the code). Read by the orchestrator. **F's review is closed.**
- The documents branch merged `v0.7.1-rc2` at `b22b431c`: **`b44cd68f`** (clean). DM next.

### 2026-11-17 — DM committed; M1 merged, its gate running

- **DM: `a887b370`** on `v0.7.1-rc2-gk-docs` (worker): five entries moved (`:2187` in `cdd0c3c9`, `:3888`
  no commit, `:3959` in `64dc1654` citing `e866ea6c`, `:3972` no commit and without the icons sentence,
  `:4140`'s whole section in `30b3e6d5`); `check_moves.py` ok; five checkers 0. Read by the orchestrator. (The
  agent misreported its parent as `43c8a9d4`; it is `b44cd68f`, history intact.)
- **M1 merges**: the connector branch at `1a3e352b` → **`f38d158f`**; the documents branch at `a887b370` →
  **`2bf76d5766fa966ba1edfc8129df69c904eabfe9`**; both clean. Every review M1 needs is closed (BT, BX read, D
  read, F with two fix rounds, DM read; A with two fixes, C read; E1, H, O, L1, L2 read).
- M1's gate: `--quick` + GPUI Fast legs, `<work>/M1/gate/run-1/` (BT and F are its two code units; A's fix-2
  `b22b431c` is under it). Then the push and CI.

- **M1's gate**: `--quick` + GPUI Fast legs `<work>/M1/gate/run-1`, STATUS 0, 414 s in all, `--quick` 6 m 21 s,
  `147 checks · 113 ✅ · 34 unchanged`; legs 401 (GPUI Fast lib: 399 plus BT's two tests), 18, 340, 340;
  tree clean. Cadence: BT and F, 1st and 2nd code units since BS's `--commit`.
- **Pushed**: `git push origin v0.7.1-rc2:v0.7.1-rc2`, `0fa28027..2bf76d57` (fast-forward). CI run
  **44007240992** watched in the background (`<work>/M1/ci-watch.log`); E2's commit waits for its result.
- E2 dispatched to a `worker` in `v071`. The documents lane switched to the KDE plan (`v0.7.1-rc2-kde-docs` at
  `2bf76d57`); KF dispatched there (the KDE record).

- **CI run 44007240992 on `2bf76d57`: success**, every job green (the GPUI Fast builds on macOS and Windows
  included). The first watch ended on a GitHub API HTTP 502, not on the run; a polling watch read the result.
  E2's commit may proceed once its gate passes.

### 2026-11-17 — E2 staged; its gate and review running

- E2 by a `worker` (about 627 k tokens: not to be resumed for new work): 43 files staged; version grep 199 at
  the start, 27 after (history, other crates' or floor versions, one excepted line); set-aside 269 chains (106
  changed, 163 hold), gate-4 65 (none a moved citation; three counts re-measured and hold), E.3 54 (only
  `demo.rs:357`'s known item), review chains 81 (none differs); taffy at 0.15.1; B.7 dated 2026-11-17;
  word-diff 27 lines each a named rewrite; nine pinned files `ok`; check_text ok; the three tests both cores.
  Its calls: `SC/tests.rs:9482` kept "(gpui-component 0.8.1)" over a measured list (sent to review);
  "floor of gpui-fast 0.1.14" (gpui-kit 0.9.1's declared floor; right); `contract.rs` 96 → 97 sites, re-counted.
  Hook: the outside-folder guard refused an inline command that held `…/citations/run/plan.json` (it read
  `/run/plan.json` as a path); the agent moved the same read into `<work>/E2/leftover.py`, reading only that
  file inside the project. A false refusal worked around; reported to the maintainer.
- The orchestrator fixed one citation the author left: `SC/inspector.rs` `resolve_font`'s panic, gpui-pre
  `:621-628` → `:620-627`, gpui-fast `:618-625` → `:617-624` (the `panic!(`…`);` span), noted in the message.
- E2's gate: `--commit` + GPUI Fast legs, `<work>/E2/gate/run-1/`; E2's `review` on the staged change.

### 2026-11-17 (04:50–) — E2 committed; the weekly limit; G1, G2 by the orchestrator

- **E2's gate**: `--commit` + GPUI Fast legs `<work>/E2/gate/run-1`, STATUS 0, 480 s in all, `--commit` 7 m 39 s,
  `147 checks · 123 ✅ · 24 unchanged`; legs 401, 18, 340, 340; porcelain the same.
- **E2: `db717d49f339a99e8c6446803ebb853c93aea067`.** Cadence: E2 is the 3rd code unit since BS's `--commit`
  (BT and F at M1, then E2) → `--commit`; the count restarts here.
- **E2's review was cut off by the weekly limit** (HTTP 429, "resets 8pm (Europe/Madrid)"); its work is saved:
  resume that agent with SendMessage after 20:00. No subagent can run until then. E2's review must close (with
  any fix committed and gated) before the KDE plan's MK1.
- G1 and G2 done by the orchestrator (verbatim spec texts; no agent available): **G1 `844a576d`**
  (CHANGELOG: two Added lines, one Fixed line, the README entry's sentence; quotes G1-1..3 ok), **G2 `6cb5e38b`**
  (ROADMAP's sentence; the count 29 → 44 by the grep on its tree; quote G2 ok). G5 staged (the note, each cited
  line opened; `ThemeColor` 139 fields at 0.8.1 and 0.9.1; no caller of `created`/`modified`/`deleted`), its
  unit's `--quick` running `<work>/G5/gate/run-1`.

- **G5's unit gate**: `--quick` `<work>/G5/gate/run-1`, STATUS 0, 63 s, `147 checks · 82 ✅ · 65 unchanged`;
  porcelain the same; five checkers 0. **G5: `2a885f6f6f8f48162101f7c6390c245a675cdc4e`.** G1, G2, G5 are the
  orchestrator's own (their reading is its).

### Next

State at 2026-11-17 05:00 (session `native-theme-31`): **the GPUI port's commits are complete** on `v0.7.1-rc2`
(HEAD `2a885f6f`; pushed up to M1's `2bf76d57`, CI green). Paused on the weekly limit (subagents refused until
20:00 Europe/Madrid).

1. After 20:00: resume E2's review agent (the "Review E2 staged hand-read" agent; SendMessage to it: the change
   is now committed as `db717d49`, read it with `git show db717d49`; continue). A must-fix goes to a fresh
   `worker` (E2's author is past 600 k tokens), gated by `--quick` (a fix does not count), re-read by the same
   reviewer.
2. Then the KDE plan's MK1 (its record): merge `v0.7.1-rc2-kde` at K1 `2e1c4af8` and `v0.7.1-rc2-kde-docs` at
   KF `76ce9ace`; `cargo package -p native-theme --list` holds `tests/vectors/kde_shade.tsv`; `--quick` + GPUI
   Fast legs (K1 and KF: 1st and 2nd since E2's `--commit`). MK2 (K2 `41c73d48`): `--commit` + legs. MK3 (K3
   `1cc99825` + `14229f12`): `--commit`, legs; close the build lane (KDE plan §2.2). KR (private X server), KD
   (ask the maintainer for the desktop window), K4 (`gh issue view 4 --comments` first; G3-12's condition to
   confirm), MK4, K5 (the close-out of both plans).

### 2026-11-17 05:15 — E2's review closed

- E2's `review` (resumed after the limit reset): **merge**, no must-fix. Should-fix: `SC/tests.rs:9482`'s own-icons
  widget list holds at gpui-component 0.9.1 (re-measured by the reviewer: only `speech/indicator.rs` gained a use,
  `Square`, already listed; `icon.rs`'s `Check` is test-only), so the label becomes 0.9.1 and the separate
  "(gpui-component 0.9.1)" after the SpeechIndicator goes; db717d49's body filed it "excepted, not re-measured".
  Nit taken: `contract.rs:2160-2161` qualified "while `list.active_highlight` is on, the connector's setting"
  (0.9.1's `list_item.rs:260-262` paints accent_foreground when off). Nit left: `SC/inspector.rs:324`'s `:1182`
  is the `Missing` arm 6f85e1d6 chose. The same unqualified claim at `contract.rs:2294` and `geometry.rs:2178`
  is outside B.4's list: left, named here.
- The fix: by the orchestrator (two comment edits, line counts kept), after MK1's gate, covered by MK2/MK3's
  `--commit`.

- MK4 run-2 `--quick`: 147 ✅, 0 ❌, 6m 17s, tree clean (MK4 = merge 29e78d8b + audit fix e417add7).
- the lane's gated head for the next release: e417add7 (MK4; --quick run /home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_kde-open-items-step1-plan/MK4/run-2; reviews closed)
