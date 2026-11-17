# 0.7.1: issue #4's open KDE items, step 1 (the table's grid and the menu panel) — implementation plan

- Rationale: [`../issues/0004_kde-open-items-rationale.md`](../issues/0004_kde-open-items-rationale.md)
  (D2, D3, and what D10 gives a step outside the code). Specification:
  [`todo_v0.7.1_kde-open-items-step1-spec.md`](todo_v0.7.1_kde-open-items-step1-spec.md)
  (Parts A to H).
- Sibling: [`todo_v0.7.1_gpui-kit-0.9.1-plan.md`](todo_v0.7.1_gpui-kit-0.9.1-plan.md),
  the plan of the GPUI Kit port's completion, which lands first. The two plans
  share the tools and commit (0) (that plan's P1 and P0), the build lane and the
  documents lane (§1), the documents both edit (`CHANGELOG.md`, `ROADMAP.md`,
  `docs/todo.md`: that plan's commits first, this plan's unit 4 after them, each
  by quoted text) and one asset run, which this plan's K5 makes for both. They
  also share one gate count, the batch's cadence
  (`todo_v0.7.1_batch-speed-rationale.md`, S1), on the script v0.7.1's first
  unit ports: GS, the gate-speed port (`todo_v0.7.1_gate-speed-spec.md`, its
  plan `todo_v0.7.1_gate-speed-plan.md`), after the GPUI plan's P1 and P0, and
  the first unit that runs a gate.
- The batch's working rules are cited as "rule N (batch-speed rationale,
  Appendix)": `todo_v0.7.1_batch-speed-rationale.md`, *Appendix — the batch's
  working rules*, rules 1 to 26, authoritative where this text differs. Rule
  26 does not apply to v0.7.1: its commit (0) (the GPUI plan's P0) adds the
  batch's documents from `v071`, where they were written; there is no earlier
  commit and no separate approval copy. v0.7.1 has no path check of rule 22 (b)
  either: it has no merge with a previous tag, its branch was not made from a
  tag, and its documents were written on this tree (that rule's "P2 runs it"
  means a release plan's P2, not the GPUI plan's P2, the check scripts). Rules
  17, 21 and 25 do not apply here either: there is no merge with a previous
  tag, so no MG and no X. Of rule 19 only its first part applies, the parity
  rule, at KR, KD and K5; its MG and measurement parts do not (there is no
  measurement; rule 19's "K5" there is the KDE steps 4 and 5 plan's close-out,
  not this plan's K5), and its `Cargo.lock` comparison with `0fa28027` is a
  release plan's P2's, and this branch is `0fa28027` itself, whose lock the GPUI
  plan's L1 and L2 change by design.
- Written: 2026-11-14, against `v0.7.1-rc2` at `0fa28027` (pull request #10).
  Its `file:line` citations are `0fa28027`'s unless they name another commit or
  version; it adds no line-number citation of a changing repository file (S6).
  Status: **implemented on 2026-11-17** (commits `e866ea6c..v0.7.1`, the
  close-out commit that moved this plan and its specification here from `docs/`
  among them); the running record at the end of this plan says how it ran.
  Approved as **to be implemented once free of errors** (the maintainer, 2026-11-15:
  "When the plan that goes in the 0.7.1 version is ready, without errors, start
  implementing it."); written for the specification's D10 (a), step 1 in
  v0.7.1, which the maintainer chose the same day; the instruction approves the
  plan as written; §7 lists what is settled. The text below is the plan as it
  was approved: it speaks of the work as still to do, and gives the paths of the
  documents moved here as they were in `docs/`.
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

| Lane | Worktree and branch | This plan's tasks | Cargo |
|---|---|---|---|
| core | `.claude/worktrees/v071`, `v0.7.1-rc2` | the merges MK1 to MK4 and their gates, KR, KD, K5 | yes: the gate |
| build | `.claude/worktrees/v071-lane`, `v0.7.1-rc2-kde` | K0's first builds, K1, K2, K3 | yes, `-p` only, at low priority |
| docs | `.claude/worktrees/v071-docs`, `v0.7.1-rc2-kde-docs` | KF, K4 | none |

The build and docs worktrees are the GPUI plan's lanes, reused: its connector
branch and its documents branch are worked in them first. The orchestrator
switches each to this plan's branch
(`git -C /home/tibi/Rust/native-theme/.claude/worktrees/<v071-lane or v071-docs> switch -c <branch> v0.7.1-rc2`,
while no agent works there) once the GPUI plan is done in that
lane: in the build lane once that plan's BT review and the orchestrator's
reading of its BX have passed and every fix is committed (the condition its M1
needs for that branch), in the documents lane after its M1 merge. A worktree keeps its path, so the build lane keeps the one
extra build folder Cargo made for it (one per worktree path), and no third one
is made. The project hooks were tried in all three worktrees by the GPUI plan
(its P1 and the forks' row: a Write of `native-theme/src/hook_trial.rs` holding
`fn f() { x.unwrap() }` refused by `no-runtime-panics.sh`); a worktree made
anew for this plan gets the same trial before any code agent works there, and
where the file is written it is removed and the work there stops (§6). **Landing order:** this plan's branches are developed beside the GPUI
plan's later work but merged into `v0.7.1-rc2` only after that plan's last
commit before the asset run (its G5, gated), as both specifications order it
(this one's *Cost and order*; the port's *Order and gates*). Until then nothing
of this step is on `v0.7.1-rc2`, so the port's re-pointing run and its
hand-read (its commits (i) and (iii)) see none of this step's lines.

```
core   GPUI plan: P1 GS's Task 0 P0 GS LN E1 H O L1 L2 A C BS [M1] E2 G1 G2 G5 ─┬─[MK1]─[MK2]─[MK3]─ KR KD ──[MK4]── K5
build  GPUI plan: BT BX (reviewed, fixes in) │ K0 → K1 → K2 → K3 ────────────────┘   (merged at unit ends)
docs   GPUI plan: D F DM [M1] │ KF ─────────────── (into MK1)                           K4 ──┘
                              (KD's fixes, if any, before MK4)
```

Why this order (each from the specification):

- After the GPUI plan's G5: the port's work, its Part G included, lands first
  (*Cost and order*), so that its re-pointing run applies to a tree without
  this step's lines and unit 4 edits the shared documents after the port's Part G.
- K1 before K2 and K3 (H.1, *Order*: unit 1 first; units 2 and 3 after it in
  either order). K2 before K3 here only because one build lane runs them.
- KF beside K1: Part C's two files are edited in place (C.1, C.2: each edit replaces a cell's or a string's text and nothing else)
  and no Rust gate reads those rows (C.4); KF and K1 land together as unit 1 at
  MK1. For the cadence, each lane task counts as one code unit: K1 and KF are
  counted separately at MK1, though this section and the specification's H.1
  call them together "unit 1" (§4). KF is a `worker`'s and not K1's implementer's because C.1 makes its
  author open the kcolorscheme source, and the clean-room rule keeps an author
  of reader code away from KDE's and Qt's sources (§5).
- KR and KD after MK3: H.1's dry parity run needs units 1 and 3 landed; it runs
  on the merged tree, so it previews the asset run's comparison of the three
  showcases with the GPUI port's changes in.
- K4 after MK3 (H.1: unit 4 after units 1 to 3, whose merges its dated lines
  name) and beside KR and KD; a fix KD demands is merged before MK4.
- K5 last: H.4, one asset run after everything that lands in v0.7.1.

## 2. Tasks

A task that changes files ends in one commit on its lane's branch (no
attribution line), made only when its done-criterion holds: in the build and
documents lanes, which have no gate, by the author; in the core worktree (a
fix, §6) by the orchestrator, after the gate, on exactly the gated tree, with
the author's message (§4). Every agent stages
its own paths by name; `git commit -a` and `git add -A` are not used. The
commit message follows the repository's form (a subject naming the unit and
what it does; a body with the outputs the specification and this table ask
for). Agent types are the maintainer's delegation table's. "The work folder"
(`<work>`) is this plan's folder for scripts, quote files and outputs, outside
the repository,
`/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_kde-open-items-step1-plan/`
(`.superpowers/` is excluded by `.git/info/exclude`). "The running record" is
`~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.1_kde-open-items-step1-plan/record.md`:
the dated results, shas, counts and decisions filled in as the work goes. Only
the orchestrator writes it, from the agents' reports; it is copied into this
plan file only in K5 step 7's commit, which commits the plan's record, so no
tracked file is edited during the work and the clean-tree checks hold; "the tools" (`<tools>`) are the GPUI
plan's `tools/` folder,
`/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/tools`
(that plan's P1), which both plans use; their slots lock
in the one durable lock folder of every plan and session,
`~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/slot-locks/`, so
the two slots count every cargo process of both plans and of any other session:
they bind every process started through a copy of `slot.sh` that uses this lock
folder (every plan of the batch does), and a copy using another folder is not
counted.

| # | Task | By | Input | Done when |
|---|---|---|---|---|
| K0 | (1) **At the GPUI plan's P1, before anything else:** the work folder, with the previous plan's layout rehearsal script `gates/rehearsal.sh`, copied from the durable copy of the writing session's scratch folder, `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/batch-2026-11-15/` (`gates/rehearsal.sh` there), since a session's scratchpad does not outlive its session; every other input the specification names in a scratch folder is copied from there too. **The rest before K1, in the build lane once it is switched to `v0.7.1-rc2-kde`:** (1a) the lane's first builds of the crates this plan's units build there, long runs (§4) from the build worktree: `cargo test -p native-theme --features "<NT_FEATURES>" --no-run`, `cargo test -p native-theme-iced --features iced_aw,watch,native --all-targets --no-run` and `cargo test -p native-theme-iced --no-default-features --all-targets --no-run` and, whenever `Cargo.lock` differs from the one the GPUI plan's BT and BX built on (`git diff --quiet <BX's commit> HEAD -- Cargo.lock` exits 1, as after L1 or L2 moved it), the gpui connector's again: `cargo test -p native-theme-gpui --features native,watch --all-targets --no-run` and the same with `gpui-fast,native,watch`; and `cargo clippy` warm-ups of the crates and feature sets K1 and K3 lint (H.3's clippy and strict-panic legs): `-p native-theme --features "<NT_FEATURES>"`, `-p native-theme-iced --all-targets` with `--no-default-features` and with `--features iced_aw,watch,native`, and, only when the lock check above exits 1, `-p native-theme-gpui --all-targets` with `native,watch`, with `gpui-fast,native,watch` and with `--no-default-features`; (2) Unit 0 (H.1): if the GPUI plan's commit (0) did not add this plan's rationale, specification and plan (approved), their status lines are set first in the core worktree's untracked copies (as P0 sets them: what the maintainer approved, with the date), those files are then copied into the build lane, and they are committed there as the first commit of `v0.7.1-rc2-kde`, landing with MK1 before unit 1, so that MK1's `cmp` of the untracked copies holds (new files: those three documents; after MK1 the running record's list of the core worktree's untracked planning documents drops them); else there is no unit 0. (3) The quote files for unit 4's texts (§4, *Texts no gate reads*): each new text of G.1, G.2, G.3 and G.5 as the specification gives it, `<date>`, `<sha>` and the ROADMAP count split with "…", one file each in `<work>/quotes/`, made from the specification and not from any edit. No repository change but unit 0's. | orchestrator | H.1; G.1–G.5 | the work folder holds the rehearsal script and the quote files |
| K1 | Unit 1's code: Part A; Part B (B.2, B.4, B.5, B.6; B.7 has no edit); D.2's `MENU_PANEL_SURFACE` and the two exceptions of "hovered menu row". The carried file and the table are written with the Write tool; the test changes and the new tests first (B.5, B.6), seen to fail, then B.2 and B.4 and the exceptions. New files: `native-theme/src/kde/shade.rs` and `native-theme/tests/vectors/kde_shade.tsv` (in a new folder); every other file it stages is an edit (§4, *The staged-tree check*). | `implement` | header; Parts A, B; D.2's second and third bullets; the rationale's D2 and D3 | H.1's three hash commands print the expected values, the three lines in the commit body under H.1's heading; the tests B.5 and B.6 add or change fail before B.2 and B.4 and pass after (names in the report); H.3's commands for `native-theme` and `native-theme-gpui`, each `-p`, pass: the core's two test runs, the gpui connector's three, clippy `-D warnings` and `check_strict_panic.sh` for both crates as H.3 names them, the private-items rustdoc of the core with `$NT_FEATURES` (read from `scripts/check_release.sh`), as `<tools>/slot.sh <tools>/lowprio.sh env RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme --no-deps --document-private-items --features "<NT_FEATURES>"` (§5's environment rule), `check_panic_patterns.py`; `cargo fmt --all --check` passes; A.4's strict-panic case, if it fails, stops the task (§6); no **gate** in the lane (MK1's covers it) |
| KF | Unit 1's facts: Part C (C.1, C.2; C.3 has no edit). Opens C.1's kcolorscheme link at `27066d47` (the one network read of this plan, and the one upstream source page this plan's agents open) and cites the lines that hold `contrastF`. | `worker` | header; Part C | the five documentation checkers pass with no finding (§4); each edit replaces the cell's or the string's text and nothing else (C.1, C.2), read in `git diff --cached` before the commit (or `git diff HEAD~1` after it); the commit body says what the link holds at the lines it cites; no **gate** in the lane (MK1's covers it) |
| K2 | Unit 2: D.1, D.2's first bullet (the `chart_grid` row), D.3. The renamed test's assertion is changed first and seen to fail against today's `chart_grid` source. | `implement` | header; Part D | the changed test fails before D.1's source edit and passes after; H.3's gpui commands pass, `-p native-theme-gpui`, on gpui-pre and on GPUI Fast (the showcase suites through `scripts/run_in_runner_env.sh` as `check_release.sh` runs them; the claim test, `every_colour_claim_is_read_at_the_line_it_cites`, is a test of the connector's library, `#[cfg(test)] mod showcase`, which the plain `cargo test -p native-theme-gpui` legs run); `cargo fmt --all --check` passes; the pinned-lines check of the GPUI port's E.5, with `F` = `connectors/native-theme-gpui/src/colors.rs` and `L` = 718, prints `ok` for every hunk (its output in the commit body); no **gate** in the lane (MK2's covers it) |
| K3 | Unit 3: E.2 to E.6, with the regeneration of `contract-leaves.toml` (E.4). Decision 12's test is written first and seen to fail; decision 13's test is written after E.2 and E.3 and before E.6's edits 0 and 3, and seen to fail. | `implement` | header; Part E; decisions 7, 8, 9, 12, 13 | the two new tests fail as above and pass after; `every_colour_leaf_reaches_a_row_or_is_named` passes against the regenerated file, whose diff shows E.4's expected differences and no other (the comparison in the commit body); H.3's iced commands pass, `-p native-theme-iced`: the two test runs, the showcase suites in the runner environment with and without `iced_aw`, clippy `-D warnings`, `check_strict_panic.sh` for the connector and the showcase legs, the all-features rustdoc, as `<tools>/slot.sh <tools>/lowprio.sh env RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme-iced --no-deps --all-features`, and the private-items one, `… env RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme-iced --no-deps --document-private-items` (both `check_release.sh:1000-1015`; §5's environment rule); `cargo fmt --all --check` passes; no **gate** in the lane (MK3's covers it) |
| MK1 | **After the GPUI plan's G5 and its gate, its E2's review passed with every fix committed and gated, and the orchestrator's reading of its G1, G2 and G5.** Merge `v0.7.1-rc2-kde` at unit 1's last commit (its review fix included; unit 0 before it, where there is one) into `v0.7.1-rc2`, then `v0.7.1-rc2-kde-docs` at KF's commit. Where there is a unit 0, the untracked copies of its documents in the core worktree are removed first, once `cmp` shows each equal to unit 0's committed file; where one differs, the merge stops and the orchestrator reports it. A conflict is resolved by the specifications' quoted texts and reported. Then A.2's packaging, unrun in the specification: `cargo package -p native-theme --list` (it lists the files, builds nothing), through the wrappers. | orchestrator | §1; A.2 | **gate** (the orchestrator's, §4): `--quick` (it brings K1 and KF, the first and second code units since the GPUI plan's E2 `--commit`; not structural, rule 24; this is the normal path; on another path the running record's count decides, rule 11) passes on the merged tree, with the GPUI Fast legs, and leaves the tree clean; the package list holds `tests/vectors/kde_shade.tsv` |
| MK2 | Merge `v0.7.1-rc2-kde` at unit 2's last commit. A conflict is resolved by the specifications' quoted texts and reported. | orchestrator | — | **gate** (the orchestrator's, §4): `--commit` (K2, the third code unit since E2's) passes, with the GPUI Fast legs, and leaves the tree clean |
| MK3 | Merge `v0.7.1-rc2-kde` at unit 3's last commit. A conflict is resolved by the specifications' quoted texts and reported. Then the build lane is closed (§2.2). | orchestrator | — | **gate** (the orchestrator's, §4): `--commit` (K3, v0.7.1's last code unit, rule 6 (batch-speed rationale, Appendix)) passes, with the GPUI Fast legs where the merge brings gpui changes, and leaves the tree clean; `git -C <core worktree> log --oneline` shows the three merges in unit order |
| KR | The layout half of the parity comparison, rehearsed without the desktop, as the previous plan's T7 did: the three showcases built for release with the capture scripts' commands, each started as `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh …` (rule 2 (batch-speed rationale, Appendix)), as long runs (§4), each started on a private X server with `--dump-layout` by the rehearsal script, which runs without `kde-inhibit` (it does not touch the desktop) and under `flock <locks>/xvfb97.lock` (its Xvfb display `:97` is shared across both lanes; the capture window of KD holds `flock <locks>/assets.lock` and `flock <locks>/gate.lock`, rules 2 and 24 (batch-speed rationale, Appendix); `<locks>` is the slots' lock folder), then `scripts/check_showcase_parity.py` on the dumps. A difference the rehearsal shows, before any capture, is reported, and KD's capture goes ahead (rule 19 (batch-speed rationale, Appendix)); a difference a comparator shows after a capture, KD's or the asset run's, is the fault rule 19 names. Commits nothing. | orchestrator | H.1, *Unit 3 ends with a dry parity run*; H.4 | the comparator's output is in the work folder, each difference reported (the running record lists them) |
| KD | H.1's dry parity run, on the desktop, in the maintainer's first capture window (§7), its release builds made first, each started as `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh …` (rule 2); the three screenshot scripts, a capture session, run in K5 step 6's form as one long run that takes the asset lock, the gate lock and the cargo slot before the window is asked for (rules 2 and 24 (batch-speed rationale, Appendix)), the literal paths written in, one log and one status per kit, `<run>` being `<work>/KD/run-<n>`: `mkdir -p <run> && CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16 flock <locks>/assets.lock flock <locks>/gate.lock <tools>/slot.sh sh -c 'touch <run>/HELD; until [ -e <run>/GO ]; do sleep 5; done; kde-inhibit --power --screenSaver --notifications bash -c "cd <core worktree> && for kit in iced gpui egui; do bash scripts/generate_screenshots_\$kit.sh <run>/dry-parity > <run>/dry-\$kit.log 2>&1; echo \$? > <run>/dry-\$kit.status; done"'; echo $? > <run>/OUTER.STATUS`; the orchestrator waits with `until [ -e <run>/HELD ] || [ -s <run>/OUTER.STATUS ]; do sleep 10; done` (an `OUTER.STATUS` first is a failure before `HELD`: the maintainer is not asked, and the failure is reported); once `<run>/HELD` exists the maintainer is asked for the window, and `touch <run>/GO` starts the captures (`slot.sh` holding a cargo slot, not `lowprio.sh`), at normal priority, since the desktop is reserved for them and the captures are timing-sensitive (settle times, 20 s dump timeouts); then, in the core worktree, `cd <core worktree> && python3 scripts/check_showcase_parity.py <work>/KD/run-<n>/dry-parity > <work>/KD/run-<n>/dry-parity.out`. The folder `<work>/KD/run-<n>/dry-parity/` stands in for the specification's `mktemp -d`; nothing later reads it, so it is removed (`rm -rf <work>/KD/run-<n>/dry-parity`, the specification's `rm -rf "$DRY"`) once the orchestrator has read `dry-parity.out`, which stays in the run's folder, `<work>/KD/run-<n>/`, with the logs and status files. Then H.3's one manual check, the orchestrator's own, in the same capture window (captures are the orchestrator's to run): the gpui showcase's Theme Map, the `chart_grid` swatch's Info, read on screen; if it cannot be done there, it is reported as not checked. The screenshot folders the scripts wrote into (their `OUTPUT_DIR`s) are restored with `cd <core worktree> && git restore connectors/native-theme-iced/docs/assets/ connectors/native-theme-gpui/docs/assets/ connectors/native-theme-egui/docs/assets/`. | orchestrator; the maintainer for the window | H.1; H.3 | each `<work>/KD/run-<n>/dry-<kit>.status` holds 0 and its log's last lines were read; the orchestrator has read `<work>/KD/run-<n>/dry-parity.out`; `git status --porcelain --untracked-files=no` prints nothing; a difference the comparator reports is fixed (§6) or excepted, as rule 19 says (KR's row), and the maintainer told; the manual check's result is in the running record |
| K4 | Unit 4, after a merge of `v0.7.1-rc2` (MK3) into the documents branch: G.1, G.2, G.3, G.5. G.3's reply edits only if the reply has not been posted: before K4's brief the orchestrator reads it with `cd <core worktree> && gh issue view 4 --comments` and records what it found in the running record (the reply waits for the release of steps 4 and 5, S5; no one asks the maintainer). "Posted" means a comment by the repository's owner that carries the draft's opening sentence ("Thank you, this was very useful, and I am glad you wrote it down even though your application has moved to Slint.", `docs/issues/0004_kde-hand-overrides-rationale.md` §8). A read that fails is reported, and K4's reply edits wait for a read that succeeds; its other edits go on. G.2's count is the grep G.2 names, run on the tree it commits. The `<sha>` of G.5's dated lines is the merge commit that brought the unit's last commit, its fixes included, into `v0.7.1-rc2`: item 1 covers units 1 to 3, so MK3's; item 2 (kde-breeze's panel and the gpui exceptions) MK1's; item 3 (the iced menu bar) MK3's. | `worker` | header; Part G but G.4 and G.6; K0's quote files in the work folder; the running record's lines, which the orchestrator gives in K4's brief | the five documentation checkers pass with no finding; `check_text.py` (§4) prints `ok` for the quote file of every text K4 writes (none for G.3's reply texts where the reply was posted, since a posted reply is not edited), its outputs in the commit body; G.2's grep output in the commit body (46 expected, another figure explained there); `git diff --cached --name-only` before the commit (or `git diff --name-only HEAD~1` after it) lists only files Part G names; no **gate** in the lane (MK4's covers it) |
| MK4 | Merge `v0.7.1-rc2-kde-docs` at K4's commit, once every review (each KR or KD fix's, the reading of K4) is closed with its fixes committed (rule 20 (batch-speed rationale, Appendix)). Then the documents lane is closed (§2.2). | orchestrator | — | **gate** (the orchestrator's, §4): the five documentation checkers and `--quick`: unit 4 is a documents unit, v0.7.1's last unit before the close-out, its gated head, which takes its cadence gate with no extra `--commit` (rule 20; MK3's `--commit` covers the code beneath it, and where a fix that KR or KD demanded landed after MK3, that fix's `--commit` covers it); it passes and leaves the tree clean (no GPUI Fast legs: unit 4 changes no gpui code); then, every review closed, the GPUI plan's running record holds rule 20's line "the lane's gated head for the next release: <sha> (MK4; --quick run <folder>; reviews closed)" (§2.1) |
| K5 | The shared close-out (§2.1). | orchestrator; the maintainer for the second capture window | GPUI specification G.4; this specification's H.4, G.4, G.6 | the full `scripts/check_release.sh` at the close-out commit passes with no failure, and CI is green |

### 2.1 K5, the close-out of both plans

One asset run for the two specifications (the port's G.4, this one's H.4), in the
port's G.4 steps, after MK4 and the gate. Every command of K5 runs in the core
worktree (`cd /home/tibi/Rust/native-theme/.claude/worktrees/v071 && …`), the
long ones (steps 2, 4, 5, 6 and 8) in the background as §4's *Long runs* says.
A fix K5 needs follows rule 15 (batch-speed rationale, Appendix; §6). v0.7.1
has no version unit V (the workspace is already at `0.7.1`), so the check rule
22 (c) gives V runs here, as step 0:

0. Before step 1, the stamped paths name no v0.7.1 planning document outside
   `docs/archive/` (rule 22 (c)'s per-match form, the stems matching the brace
   form `{spec,plan}` too). The pathspecs are `scripts/update_provenance.sh`'s
   `SOURCE_PATHS` (`:39-71`) and `scripts/update_compatibility.sh`'s source paths
   of every connector (`source_paths`, `:69-80`: each connector's `Cargo.toml`,
   `src`, `examples` and `tests`, egui's `mapping.toml`, the egui widgets
   crate's `Cargo.toml` and `src`), read again from the two scripts at the run:

   ```sh
   cd /home/tibi/Rust/native-theme/.claude/worktrees/v071 && git grep -noE \
     '(archive/)?todo_v0\.7\.1_(gpui-kit-0\.9\.1|kde-open-items-step1|gate-speed)-[a-z{},.-]*' -- \
     Cargo.toml Cargo.lock native-theme/Cargo.toml native-theme/build.rs \
     native-theme/src native-theme/icons native-theme-derive/Cargo.toml \
     native-theme-derive/src native-theme-build/Cargo.toml native-theme-build/src \
     connectors/native-theme-gpui/Cargo.toml connectors/native-theme-gpui/src \
     connectors/native-theme-gpui/examples connectors/native-theme-gpui/tests \
     connectors/native-theme-iced/Cargo.toml connectors/native-theme-iced/src \
     connectors/native-theme-iced/examples connectors/native-theme-iced/tests \
     connectors/native-theme-egui/Cargo.toml connectors/native-theme-egui/src \
     connectors/native-theme-egui/examples connectors/native-theme-egui/tests \
     connectors/native-theme-egui/mapping.toml \
     connectors/native-theme-egui-widgets/Cargo.toml connectors/native-theme-egui-widgets/src \
     tools/showcase-support/Cargo.toml tools/showcase-support/src \
     scripts/generate_screenshots_iced.sh scripts/generate_screenshots_gpui.sh \
     scripts/generate_screenshots_egui.sh scripts/generate_gifs_theme_switching.sh \
     scripts/capture_window.sh scripts/generate_gifs_spinners.py \
     .github/workflows/screenshots.yml \
     | grep -v ':archive/'
   ```

   It prints nothing; a hit is re-pointed to its archived path then, as a fix
   under rule 15 (it touches a stamped path).
1. `cargo audit` passes (through the wrappers).
2. `scripts/update_compatibility.sh run` passes, started as
   `cd <core worktree> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/update_compatibility.sh run`
   (a heavy run, rule 2 (batch-speed rationale, Appendix)), as a long run; its
   changes stay uncommitted until step 7.
3. Once it has passed, `git push origin v0.7.1-rc2:v0.7.1-rc2` (units 1 to 4
   together, H.1, with the GPUI plan's commits since its own push); CI runs on
   it (`todo_v0.7.1_batch-speed-rationale.md`, S4).
4. While CI runs, the full `scripts/check_release.sh` (§4's gate command, under
   `gate.lock`) shows only the provenance
   stamp failing, and the known `cargo outdated` warning (zbus excluded), its
   list recorded in the running record. A red CI run here can email the
   maintainer for a fault this check would have shown first (S4's cost).
5. The three showcases are built for release with the capture scripts' commands
   (the port's G.4 step 5), each started as
   `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh …` (rule 2), while
   CI runs, as long runs.
   Step 6 starts only when both CI and step 4 are green, as rule 9 (batch-speed
   rationale, Appendix) reads it: CI green, and step 4's only failure the
   provenance stamp, which only the asset run refreshes.
6. The asset run, under rule 24 (batch-speed rationale, Appendix): the asset
   lock, the gate lock (rule 2) and the cargo slot are all taken before the
   maintainer is asked for the capture window. The orchestrator starts, as a
   long run, with `<run>` a new run folder `<work>/K5/run-<n>/`:

   ```sh
   mkdir -p <run> && CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16 flock <locks>/assets.lock flock <locks>/gate.lock <tools>/slot.sh sh -c 'touch <run>/HELD; until [ -e <run>/GO ]; do sleep 5; done; kde-inhibit --power --screenSaver --notifications bash -c "cd <core worktree> && bash scripts/generate_assets_release.sh > <run>/log 2>&1; echo \$? > <run>/STATUS"'; echo $? > <run>/OUTER.STATUS
   ```

   with `<core worktree>`, `<tools>` and `<run>` written as literal paths, and
   `<locks>` the slots' lock folder,
   `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/slot-locks`:
   the asset run and its capture window hold the shared `assets.lock` and
   `gate.lock`, so a lane's run waits for another's, and for any gate,
   mechanically, and the slot is held from the start. The orchestrator waits with the Monitor tool,
   `until [ -e <run>/HELD ] || [ -s <run>/OUTER.STATUS ]; do sleep 10; done`
   (rule 24): where `<run>/OUTER.STATUS` comes first, the run failed before
   `HELD` (the lock, the slot or the shell), the maintainer is not asked, and the
   failure is read from that file and reported. Once `<run>/HELD` exists, the
   maintainer is asked once for "capture now" (§7); on the maintainer's word the orchestrator runs
   `touch <run>/GO`, which starts the capture. The two variables stand before
   the first wrapper (§5's environment rule): the script's last step, after the
   GIFs, is `update_compatibility.sh run` (`scripts/generate_assets_release.sh:278`),
   all three connectors' gates. `kde-inhibit` with
   exactly those options, as the earlier runs used them, and `slot.sh` (holding
   a cargo slot) but not `lowprio.sh`, since earlier runs ran the captures at
   normal priority, the desktop is reserved for them, and the captures are
   timing-sensitive (settle times, 20 s dump timeouts). `kde-inhibit` returns 0
   whatever its child returns, so the orchestrator reads the script's own exit
   status from `<run>/STATUS`, and the log's last lines. The desktop is free for
   the maintainer from the log's "Theme-switching GIFs generated"
   (`scripts/generate_assets_release.sh:172`): the orchestrator waits for it
   with the Monitor tool,
   `until grep -q 'Theme-switching GIFs generated' <run>/log || [ -s <run>/STATUS ]; do sleep 10; done`
   (the literal paths written in), and then tells the maintainer; the machine
   stays busy until the script ends. A difference its parity step shows after the
   capture is a fault (rule 19): fixed as a close-out fix, or excepted with its
   reason and source and reviewed, and the maintainer told. If the script stops, §6 (rule 15) says
   what follows.
7. The reply's state is read again (rule 16 (batch-speed rationale,
   Appendix): `cd /home/tibi/Rust/native-theme/.claude/worktrees/v071 && gh issue view 4 --comments`).
   A failed read is reported, the asset commit goes on, and the read is
   repeated before step 8. Where it reads "posted" (since K4), K4's draft edits
   are reverted in a documents-only commit made right after the asset commit
   below (the five checkers and `--quick` on the staged change), pushed with
   this step's refspec and covered by step 8, and the maintainer is told
   (rule 16). Before the asset commit, the orchestrator opens the new KDE screenshots beside the
   previous ones and tells the maintainer what changed, with H.4's list and the
   port's G.4 list of what should. Then one commit holds what the port's G.4
   step 7 names (the assets, both stamps, the READMEs' lines, the 0.7.1 date set
   to that commit's day), and the archiving, which is the last task of both
   plans: seven documents move to `docs/archive/`, this specification and this
   plan, the GPUI Kit rationale, its specification and its plan, and the
   gate-speed specification and plan (`docs/todo_v0.7.1_gate-speed-spec.md`,
   `docs/todo_v0.7.1_gate-speed-plan.md`); `docs/archive/README.md`'s
   v0.7.1 row gains G.6's text and, for the port's documents, ";
   `todo_v0.7.1_gpui-kit-0.9.1-{rationale,spec,plan}.md`: what the GPUI Kit 0.9.1
   port left undone (the showcase's new parts, statements, the citations at
   0.9.1, the coverage gate, the backlog's upstream entries and the two pins)",
   and, for the gate-speed documents, "; `todo_v0.7.1_gate-speed-{spec,plan}.md`:
   `--quick`, nextest and the step cache ported from 0.8.0, and CI's MSRV
   checks in `--commit` and `--quick` of `scripts/check_release.sh`" (as the
   maintainer answered the gate-speed specification's O1a, O2 and O3 on
   2026-11-15: O1a no, so no "and the full run"; O2 (e) and O3's keyed option,
   so the clause stands as written). The batch-speed
   rationale (`docs/todo_v0.7.1_batch-speed-rationale.md`) stays in `docs/`: it
   applies to the whole batch, and the close-out of the batch's last release in
   the maintainer's order (proposed: 0.7.7) archives it (rule 22 (a)); were
   v0.7.1 that release, this step would move it too, by the same rules;
   every mention of a moved document, a link or a plain path, is rewritten for
   the new place in every document committed in commit (0) or since, the
   rationale in `docs/issues/` among them (links and plain paths both; they are
   found by the grep below at the run, since these documents' lines still
   move); the cycle's documents still untracked (the caret's, the icons', the
   `docs/todo_v0.7.x_*` ones and the egui 0.37 rationale, §7) are outside the
   work and not edited here, and a mention in them of a moved document is left
   for the plan that commits them; and the
   links from the seven moved documents to documents that stay; the status
   lines of the seven say implemented, with the date and the commit range,
   stated as `<first commit of the work>..v0.7.1` (the release tag, which the
   maintainer makes after this step; not the asset commit's own sha, so that a
   later commit, the rule-16 revert or a rule 15 fix, stays inside it); and
   G.4's sentence goes into the rationale's status line (the rationale itself
   stays in `docs/issues/`, G.4). The rewrite follows rule 22 (a) (batch-speed
   rationale, Appendix): every path mention in every tracked file outside
   `docs/archive/`, found after the move with
   `cd /home/tibi/Rust/native-theme/.claude/worktrees/v071 && git grep -nE '<pattern>' -- ':!docs/archive'`,
   `<pattern>` being
   `todo_v0\.7\.1_(gpui-kit-0\.9\.1-(rationale|spec|plan)|kde-open-items-step1-(spec|plan)|gate-speed-(spec|plan))\.md`;
   a document that dates its lines is exempt only for line citations; left as
   they are, each named in the commit body: a mention already pointing into
   `docs/archive/` in any relative form or as an absolute URL, a citation dated
   "at `<sha>`", and a path inside another document's instructions that names
   its own future target. **Check**, per match, not per line:
   `cd /home/tibi/Rust/native-theme/.claude/worktrees/v071 && git grep -noE '(archive/)?<pattern>' -- ':!docs/archive' | grep -v ':archive/'`
   prints nothing but the mentions left by those exemptions, each named in the
   commit body with the rewritten list. The seven moved documents' own relative
   links are re-checked after the move: each resolves from `docs/archive/` (a
   script in the work folder lists every `](…)` target that does not exist), with
   rule 22 (b)'s exemptions (placeholders, quoted texts of other files, URLs); it
   lists none but those. The same commit commits each plan's
   record: the three running records are copied into their plan files, this
   plan's and the GPUI plan's, and the gate-speed plan's, which that plan is
   required to keep at
   `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.1_gate-speed-plan/record.md`,
   written by the orchestrator alone (where that plan names another path, its
   path holds and is used here), as the archived plans' status lines
   and departures are, and the seven moved documents' status lines are written
   from them; no tracked file was edited for the record before it. Before the
   commit, §4's staged-tree check: its new files are the asset files the asset
   script added (each named in the running record from the script's log), its
   moves the seven moved documents, read with
   `git diff --cached --find-renames=20% --name-status` (appending a running
   record in the same commit can push a moved plan under git's default 50 %
   similarity): each moved document shows as an `R` pair, or, where it still
   falls under 20 %, as a `D` at its old path with an `A` at its archive path,
   which is accepted for these seven and named; every other path an edit the step names. Pushed with the same
   refspec.
8. The full `scripts/check_release.sh` passes at that commit with no failure,
   the known `cargo outdated` warning expected and its list recorded, beside CI
   on it; CI is green.

From step 2 on, `Cargo.lock` does not change (no `cargo update`, no new
dependency): the provenance stamp hashes it.

Then the orchestrator runs
`~/.claude/scripts/sweep-target.sh /home/tibi/Rust/native-theme/.claude/worktrees/v071`
and reports the sizes it
prints. Then the 1.88.0 MSRV checks' build folder: under O2 (e), the
maintainer's answer of 2026-11-15, the gate deletes the folder itself at every
run's end, so the close-out only checks that it is gone
(`test ! -e <build_directory>/msrv-1.88`, `<build_directory>` from the stable
toolchain's `cargo metadata --format-version 1 --no-deps`, run through
`lowprio.sh` alone). v0.7.1's last unit before this close-out, which both lanes branch
from, is MK4, the gated head, gated by the five documentation checkers and
`--quick` with no extra `--commit` (rule 20 (batch-speed rationale, Appendix):
MK3's `--commit` covers the code beneath it, and where a fix that KR or KD
demanded landed after MK3, that fix's `--commit` covers it), made once every review (each KR or
KD fix's, the reading of K4) is closed with its fixes committed: once it has
passed, the orchestrator writes into the GPUI plan's running record,
`~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.1_gpui-kit-0.9.1-plan/record.md`,
the line "the lane's gated head for the next release: <sha> (MK4; --quick run
<folder>; reviews closed)", naming MK4's commit (a fix after a red gate
included), and the same line into this plan's own running
record, so that a reader of either finds it. Both lanes' next releases, KDE steps 2 and 3
(the KDE lane, which holds the caret too) and the widget icons (the second lane), read that line: the next release's branch starts from
that gated head and is merged with its `<prev-tag>` (the tag just before it in
the release order: by the batch's proposed numbers, v0.7.2 for steps 2 and 3,
v0.7.4 for the icons) once (S2 as amended, rules 17 and 21), after its own
last unit before the merge (its documents unit, rule 20); it never waits for this close-out or
the tag. Where its `<prev-tag>` already exists when it starts, it branches from
that tag instead and has no merge (rules 17 and 18). The lane predecessor
depends on the release order, still the maintainer's (rule 18). A worktree v0.7.1 used, `v071` among them, is reused only once
v0.7.1's tag exists, checked after `git fetch origin tag v0.7.1` (rule 18). The core worktree `.claude/worktrees/v071` and its build folder
stay after v0.7.1 (the batch's worktree rule, named in each `docs/todo_v0.7.x_*`
plan's §1; S3 as amended): the KDE lane alternates between `v071` and
`.claude/worktrees/kde-b`, steps 2 and 3 running in `kde-b` while this close-out
holds `v071`, steps 4 and 5 in `v071`, the caret in `kde-b`, steps 6 and 7 in
`v071`; the second lane holds the widget icons alone, in
`.claude/worktrees/lane2-a`. **Merging the pull request, tagging, publishing, the GitHub release,
the pull request's title and description, posting the reply to the reporter
(after the release of steps 4 and 5, `todo_v0.7.1_batch-speed-rationale.md`, S5)
and sending the upstream requests are the maintainer's decisions and are not part
of this plan.**

### 2.2 Clean-up

When MK3 is made, the orchestrator reads the build lane's build folder from
`cargo metadata --format-version 1 --no-deps` (`build_directory`), run
through `lowprio.sh` alone (no slot: it builds nothing) in that
worktree, checks that no build runs there, deletes the folder, removes the
worktree
(`git -C /home/tibi/Rust/native-theme/.claude/worktrees/v071 worktree remove /home/tibi/Rust/native-theme/.claude/worktrees/v071-lane`,
absolute paths, rule 24 (batch-speed rationale, Appendix)) and reports it. After MK4 it reads the
documents worktree's `build_directory` the same way, tests whether that folder
exists (no build is expected there, but F's `cargo metadata` ran in it),
deletes it if so, removes the worktree
(`git -C /home/tibi/Rust/native-theme/.claude/worktrees/v071 worktree remove /home/tibi/Rust/native-theme/.claude/worktrees/v071-docs`)
and reports it. The four merged
lane branches, the GPUI plan's `v0.7.1-rc2-gk-connector` and
`v0.7.1-rc2-gk-docs` and this plan's `v0.7.1-rc2-kde` and
`v0.7.1-rc2-kde-docs`, are deleted with `git branch -d` (each after its last
merge; `-d` refuses a branch not merged).

## 3. Reviews

Every code commit is reviewed before its merge into `v0.7.1-rc2`. The review
runs while the lane's next task works: the reviewer reads the commit
(`git show`, the commit range), never a working tree, and runs no cargo; it gets
the diff and the specification's parts and decisions, not the author's report.
Every reviewer, the orchestrator included, checks that no new code transcribes
an LGPL source (§5, *Clean room*).
A must-fix is a new commit by the task's author, in the same lane, reviewed by
the same reviewer and passing the same done-criterion; it is made as the lane's
next commit after the task then running there has committed. A unit's merge
point (MK1 to MK3) is the commit that holds its fix; where a fix lands after a
later unit's commit, that merge takes both units and its gate covers both, and
the running record says so. A second must-fix on the same point falls under §6's
take-over rule.

| Commit | Reviewer | Why that one |
|---|---|---|
| K1 (carried module, reader, preset, tests, gpui exceptions) | `review` | the derivations and the tests against Parts A and B and the rationale's tables; the carried file is read for its notice and declaration only, its bytes are H.1's hash check |
| K2 (the chart's grid) | orchestrator | some thirty lines with a gate that pins them (the claim test, the pinned-lines check) |
| K3 (iced styles, contract, showcase, generated export) | `review` | two styles and a contract export that decide what every iced menu and drop-down draws, and two new tests that must prove what decisions 12 and 13 claim |
| KF, K4 (facts, documents, backlog) | the checkers, `check_text.py`, then the orchestrator reads the diff | the gates and the quote check validate the data and the texts; the texts were reviewed in the specification |
| a fix KR, KD or K5 demands | `review` | it changes code after units 1 to 3 are merged (§6) |

No Fable agent is planned for this plan's own tasks. `review-hard` is for the
hard parts of a core (resolver, validation, runtime, concurrency, `unsafe`);
this step's reader changes are two derivations through a carried function whose
table checks it. GS's review, before this plan's work, is `review-hard` (the
gate-speed specification, Part F), under its own plan.

## 4. Gates

- **While working**, an agent runs the tests of the module it changes. **Before
  its commit**, it runs its done-criterion's commands once, in the foreground,
  through the tools' slot and low-priority wrappers. The release stamps are
  stale from this plan's first commit until K5 (H.1) and are no task's
  done-criterion.
- **The gate command.** Every gate run of this plan, `--quick`, `--commit` and K5's
  two full runs alike, starts as
  `cd /home/tibi/Rust/native-theme/.claude/worktrees/v071 && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh <mode>`
  (rule 2 (batch-speed rationale, Appendix); `<locks>` the slots' lock folder, §2;
  the full run with no mode), as the GPUI plan's §4 gives it.
- **The gate** (MK1 to MK4, and a fix's) is the batch's cadence
  (`todo_v0.7.1_batch-speed-rationale.md`, S1: during a unit, the agent runs
  only the touched tests, its targeted commands; at a unit's end, `--quick`, by
  the orchestrator, as a long run until its time is measured, the first timed
  into the record; `--commit` every third code unit of the release, counted from
  the last `--commit`, on the release's last code unit before its documents and
  version units (rule 6: here MK3, below), after a lock or toolchain change, the warm-up gate,
  and wherever the plan names a unit structural; a documents-only unit runs the
  five documentation checkers and `--quick`; the full run where the plan has it,
  the close-out, unchanged; a unit is a code unit by rule 11 (batch-speed
  rationale, Appendix): only comments, rustdoc prose, READMEs and Markdown keep
  it a documents unit, and `docs/platform-facts.md` counts as code (two tests read it by
  `include_str!`, `native-theme/src/presets/documented_sizes.rs:55` and
  `native-theme/tests/platform_facts_xref.rs:22`); a documents unit runs
  `--quick`, whose keys (gate-speed specification A.2, C.4) re-run the steps of
  the crates whose files it changed, and is not counted toward "every third" or
  "the release's last code unit"), on the script the GPUI plan's
  GS ported, counted on the one branch and the one count both plans share (the
  GPUI plan's §4 and its table: one count per code unit a merge brings; any
  `--commit` restarts it; a `--quick`-gated fix does not count, rule 11). On the
  normal path, after the GPUI plan's E2 `--commit`: MK1 `--quick` (it brings
  K1 and KF, both code: KF's facts rows; not structural, since rule 24 makes a
  unit structural only where it changes a manifest, `Cargo.lock`, a build
  script, the toolchain or the gate scripts, and a new module is no reason on
  its own); MK2 `--commit` (K2, the third); MK3 `--commit` (K3, the first since
  MK2's, and v0.7.1's last code unit, rule 6; v0.7.1 has no version unit V, the
  workspace being already at `0.7.1`, `Cargo.toml:17`); MK4, unit 4, a
  documents unit and v0.7.1's gated head, the five documentation checkers and
  `--quick`, with no extra `--commit` (rule 20). On any
  other path the same rule decides, and the running record names each gate
  point's count. A fix KR or KD demands lands before K4's merge, between MK3 and
  MK4, gated `--commit`, with the GPUI Fast legs where it touches gpui code: it
  becomes v0.7.1's last code unit (rule 6, S1), and MK4 stays `--quick`.
  A fix K5 needs follows rule 15 (§6). A failure a
  later `--commit` finds in a commit only `--quick` proved is fixed by a new
  commit, the running record naming the commit that brought it. Each runs in
  the core worktree through the wrappers; `--commit` exits 0, its summary
  naming no failed check, only the release-only checks as "skipped (--commit
  mode)", and `--quick` exits 0 with no failed check; and after
  it `git status --porcelain --untracked-files=no` prints nothing, since the
  script formats in place and only reports a dirty tree (the GPUI plan's §4
  gives the lines), while CI's `cargo fmt --all --check` fails the push. Every
  code task runs `cargo fmt --all --check` before its commit. The gate is the
  orchestrator's, a long run (below); agents run only their targeted
  commands, as the GPUI plan's §4 *Who runs what* says. A fix commit on
  `v0.7.1-rc2` (§6) is gated as a task is there: the author stages it and
  reports its message; the orchestrator runs the gate on the staged change,
  after `cargo fmt --all --check`,
  `git status --porcelain --untracked-files=no` printing the same before and
  after the gate, and commits exactly the gated tree with the author's
  message, after which it prints nothing; a red gate goes back to the author
  (the same agent, continued) with the log's failing part. `--commit`
  skips the gpui connector's tests on GPUI Fast (and `--quick` runs only what
  the gate-speed specification gives it), so where the merge brings gpui
  changes (MK1, MK2) the gate adds them:
  `cargo test -p native-theme-gpui --features gpui-fast,native,watch --all-targets`
  and `scripts/run_in_runner_env.sh test -p native-theme-gpui --features gpui-fast,native,watch --example showcase-gpui`,
  each started under the gate's lock,
  `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh …` (rule 2; the
  batch-speed rationale's glossary, *The GPUI Fast legs*).
  A lane commit's own done-criterion runs `-p` builds only; the gate on the
  merged tree is what covers the workspace.
- **Documentation checkers** (no cargo): `scripts/check_platform_facts_rows.py`,
  `check_section_refs.py`, `check_preset_citations.py`,
  `check_wrong_safety_nets.py`, `check_todo_citations.py`, from the worktree's
  root, each with no finding.
- **Texts no gate reads** (H.3, *Not covered by any gate*): unit 4's new texts are
  checked by the GPUI specification's G.6 script, `check_text.py`, kept in the
  GPUI plan's work folder,
  `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/checks/`,
  run as `python3 -I /home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/checks/check_text.py <file> <quote file>`
  with K0's quote files. It adds a check the specification does not ask for; it decides nothing
  about the texts. The code-side texts (the doc comments, the `UNREAD_COLOURS`
  reasons, the showcase's comments of E.6 items 1, 2 and 4) are read in the diff
  by the reviewer, as H.3's last table says.
- **The minimum supported Rust version:** under the gate-speed specification's
  O3 as the maintainer answered it (2026-11-15, the keyed option;
  `docs/todo_v0.7.1_gate-speed-spec.md`, Part M), CI's MSRV checks run in
  `--commit` and in `--quick` of the ported script, the two `+1.88.0` checks in
  `--quick` where anything their key follows changed (their crates' reach and
  included files, the files outside the crates but `QUICK_UNREAD`, the tools,
  the environment, their toolchain), and in CI; the full
  run is unchanged (O1a no). The briefs' rule stays: no language or library feature newer than the
  edited crate's own `rust-version`
  (the core and the iced connector 1.88.0, the gpui connector 1.95.0, the egui
  connector 1.95).
- **Full `scripts/check_release.sh`:** twice, both in K5: step 4, while CI runs (the provenance
  stamp failing) and step 8 (no failure); in both, the known `cargo outdated`
  warning (zbus excluded) is expected, and the list it prints is recorded in
  the running record.
- **Long runs.** Every gate, and every other run not known to finish within the
  Bash tool's 10-minute foreground limit, is the orchestrator's, run with
  `run_in_background` from the stated worktree, through the wrappers, its log
  and a `STATUS` file at absolute paths in the work folder, as K5 step 6's
  command does; the orchestrator waits for the exit notice, then reads `STATUS`
  and the log's tail. No `pgrep` loop is used (it matches itself); a wait on a
  file condition uses the Monitor tool with an until-loop (K5 step 6). Long
  runs here: MK1 to MK4's gates and a fix's gate; K0 (1a)'s first builds; KR's
  and K5 step 5's release builds; KD's captures; K5 steps 2, 4, 6 and 8 (the
  full check about 23 minutes, the asset run 30 to 37). Each of these that runs
  cargo takes the gate lock, `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh …`
  (rule 2 (batch-speed rationale, Appendix); KD's captures and the asset run
  under `assets.lock` too, rule 24); an agent's targeted `-p` commands take
  `slot.sh` and `lowprio.sh` only. The first `--commit`
  run of both plans (the GPUI plan's GS, its last gate) and the first `--quick`
  runs (the gate-speed specification's E.4 item 4 on the clean tree, then the
  GPUI plan's E1, the first on a change) are timed into that plan's running
  record. Every long run, its first included, writes its log and `STATUS` into
  a new subfolder of its own, `<work>/<task>/run-<n>/`, `<n>` starting at 1
  and named in the running record, so that a leftover `STATUS` of an earlier
  run (a gate run again after a red result, the asset run after a fix, KD's
  captures, KR's rehearsal) never ends a Monitor wait early or is read as the
  new run's result; KD's and K5 step 6's paths above are those of a run's
  subfolder. A foreground command the Bash tool kills at 10 minutes is
  reported by the agent and run again by the orchestrator as a long run, under
  `gate.lock` (rule 2), as every such long run that runs cargo; after
  such a re-run the orchestrator continues the same agent with `STATUS` and
  the log's tail. **The staged-tree check**, before each gate and again before
  the commit, by the orchestrator: `git diff --quiet` exits 0;
  `git status --porcelain --untracked-files=all` shows no `??` line but the
  untracked planning documents the running record lists for that worktree,
  one by one by path (the GPUI plan's P1 and P0 record the core worktree's; K0
  updates the list after unit 0; a lane worktree's list is empty); and
  `git diff --cached --name-status` shows exactly the task's files: those its
  specification parts name (*How to read it*'s table), with `A` only for the
  new files its row names (K1: `native-theme/src/kde/shade.rs` and
  `native-theme/tests/vectors/kde_shade.tsv`; unit 0 in K0: its documents; K5
  step 7: the asset files the asset script adds and the seven moved documents,
  as `R` pairs under `--find-renames=20%`, or as accepted `D`/`A` pairs, K5
  step 7 says); a row that names no new file adds none. Any difference goes back to
  the author before the gate runs or the commit is made. In the build and
  documents lanes, where the author commits, the author runs the same three
  checks before its commit and puts their output in its report. While an
  agent runs cargo, the orchestrator's long
  runs hold at most one of the two slots.
- **CI** (the pull request's checks): the GPUI plan pushes once after its M1;
  this plan pushes at K5 step 3 and step 7. CI checks no release stamp (only
  `publish.yml` does), so those pushes are expected green. A red run emails the
  maintainer: no push is made that is expected to be red. If a run is red, the
  orchestrator tells the maintainer what failed and why, and the author of the
  failing change fixes it (reviewed as in §3); the fix is pushed with
  `git push origin v0.7.1-rc2:v0.7.1-rc2` once the gate passes again.
- **Parallel work before a capture window** commits nothing: KR's and step 5's
  builds run while K4 works or CI runs.

## 5. Rules every agent brief carries

- The brief gives the specification's header, its *How to read it* (with
  *Lines that must not move*) and the task's parts. The parts are the whole
  task: nothing beyond them, no refactoring, no formatting churn. A needed
  change the parts do not name is reported, not made.
- Every edit is applied by its quoted text or its test's name; the
  specification's line numbers are `0fa28027`'s and serve to find the place.
  Where a quoted text no longer stands, the agent stops and reports, and the
  orchestrator finds it in its present form as §6 says.
  Where an upstream line number inside a quoted "today" text differs on the tree
  because the GPUI port's re-pointing run moved it, the tree's number is kept
  and the case reported. (Checked for D.1's comment: the three chart lines it
  cites are the same at gpui-component 0.8.1 and 0.9.1.)
- **The fourth gate** (S6 in the batch-speed rationale, *The fourth gate,
  kept*): `every_colour_claim_is_read_at_the_line_it_cites` reads the line each
  colour claim cites in the gpui connector's own files. Every unit that edits
  `connectors/native-theme-gpui/src/` or its examples (K1, K2, and a fix KR, KD
  or K5 demands there) keeps the line count above each cited line (the
  specification's *Lines that must not move*: Part D keeps `colors.rs`'s lines 1
  to 718; K2's pinned-lines check), or re-points the claims that cite a moved
  line, and their copies in the showcase's `tests.rs`, in the same commit; the
  test proves it.
- **The worktree.** The brief names the lane's worktree by its absolute path.
  Every command starts with `cd` into it or uses `git -C` and `--manifest-path`;
  every command that runs a repository script states the worktree it runs in
  (`cd <worktree> && scripts/…`, inside a `bash -c` too, as KD's and K5 step
  6's commands do).
  The main checkout, where the session starts, is another branch (0.8.0) with
  the same relative paths: nothing is read, run or written there, but for the
  two plans' work folders under its `.superpowers/sdd/` and `git show v0.8.0-rc1:…`
  from the lane's own worktree, for K1's two carried files only
  (`native-theme/src/platform/kde_shade.rs`, `native-theme/tests/vectors/kde_shade.tsv`):
  nothing else of `v0.8.0-rc1` is read by K1, since its
  `docs/facts/kde/tokens.toml`, for one, quotes kcolorscheme's code. A check that passes in the main
  checkout proves nothing. The report quotes `git -C <worktree> log -1 --oneline`.
- **Clean room (D10).** No code from LGPL or GPL sources, KDE's and Qt's
  included. An LGPL-referenced function enters only through the clean room: K1
  carries `kde_shade` byte for byte with its notice (A.1) and writes the reader's
  calls from the specification's text. Every implementer brief whose task
  touches a function a KDE or Qt source is cited for (the sources the
  specification cites: kcolorscheme, kconfig, kguiaddons, Breeze's
  `breezestyle.cpp`, `breezehelper.cpp` and `breezemetrics.h`, qtbase's
  `qcommonstyle.cpp`, plasma-workspace's `kcms/colors/colorsapplicator.cpp`
  (B.1), qqc2-desktop-style's `Popup.qml` and Kirigami's `platformtheme.cpp`
  (C.1)) says the implementer opens none of those sources: the
  specification's description and its tests (B.3's values, B.6's cases, the
  carried table's 260 rows) are the clean room's description and vectors.
  That is K1, and any fix of K1's code that KR, KD or K5 demands. KF's worker
  reads the one kcolorscheme link for a citation and writes no code. An
  Apache-licensed port keeps a file-level notice (none in this step).
- Code, tests and rustdoc that name one of v0.7.1's own planning documents (the
  seven K5 step 7 moves) name its archived path, `docs/archive/…`, since the
  close-out moves it after the asset run (rule 22 (c) (batch-speed rationale,
  Appendix); K5's step 0 checks the stamped paths for it).
- Source files are edited with the Edit and Write tools only, the carried file
  and the table included. A hook that blocks an edit is obeyed, never bypassed,
  and its content is never written another way (no `git show … >`, no `sed`, no
  shell redirection); a block on content the specification requires stops the
  task (§6). B.6's note on the `no-invented-values.sh` hook is followed.
- No `unwrap`, `expect`, `panic!` or panicking index in non-test code; no
  `unsafe`. No public item is removed. No invented value: a date, a sha, a count
  or a line read at the run is filled in from the run.
- An environment setting goes before the first wrapper
  (`VAR=… <tools>/slot.sh <tools>/lowprio.sh …`) or through `env`
  (`… lowprio.sh env RUSTDOCFLAGS="-D warnings" cargo doc …`, as
  `check_release.sh:1002-1010` does), never as a bare `VAR=…` after a wrapper:
  `lowprio.sh` ends in `exec "$@"`.
- Cargo runs through the tools' slot and low-priority wrappers, in the
  foreground (the capture sessions, KD's screenshot scripts and K5's asset
  script, take `slot.sh` and `kde-inhibit` without `lowprio.sh`), at most 16 build jobs (`CARGO_BUILD_JOBS=16`) while the maintainer
  works; `build-dir` and the target directory are never set; `cargo clean` is
  never run (`cargo metadata`, which builds nothing, runs through `lowprio.sh`
  alone). In the build lane only `-p` builds. A run through
  `scripts/run_in_runner_env.sh` that exits 75 (a package it cannot fetch) is
  reported, not worked around.
- No `find /` or any other search of the whole file system. Reads and writes
  stay inside the lane's worktree, Cargo's home and build folder, the running
  session's scratch folder (for its own temporary files only; no input is
  taken from a scratchpad but through the work folder), the durable copy
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/batch-2026-11-15/`
  (read only), the slots' lock folder
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/slot-locks/`
  (written by `slot.sh` alone) and the two work folders. An agent does not
  write the running record or the plan files; it reports.
- Nothing appears on the maintainer's desktop, but in KD and K5, which the
  orchestrator runs.
- A new test is written first and seen to fail. A changelog entry is one
  physical line. The agent stages its own paths by name; in a lane it commits
  once, after §4's staged-tree check passes in its worktree, and in the core
  worktree (a fix, §6) it reports its commit message and
  the orchestrator commits after the gate (§4); it does not push, adds no
  attribution line, and reports once, at the end: the commit or its message,
  the files, its commands with their result lines, and everything it was
  unsure of. Its commands run in the foreground; the exception is every
  command not known to finish within 10 minutes (`--commit`, the full check,
  an asset run, a first build in a worktree), which is not the agent's but the
  orchestrator's long run (§4).
- **Something the maintainer must hear before a commit** (C.1's link does not
  hold `contrastF` at any cited line; a value differs from the specification's):
  the agent leaves the work uncommitted and reports. The orchestrator tells the
  maintainer and commits without waiting, unless the difference contradicts a
  decision of the rationale; then §6's stop rule holds.

## 6. Limits and stop rules

A **stop**, wherever this plan names one: nothing committed by the task that
met it, and nothing pushed after it until the maintainer has been told and,
where the rule that names the stop asks the maintainer, has answered.
A stop ends when the maintainer has answered where the rule asks; otherwise
the work continues once the maintainer has been told. A quoted text that no
longer stands is found in its present form by the orchestrator: where the
edit's meaning is unchanged, the edit is made at the found text and the record
names it; where it is not, the maintainer is asked.

- **Agents at once:** at most one per lane, plus reviewers; at most 4 agents, or
  6 when two of them run no cargo (the documents lane and the reviewers run
  none). Both plans' agents count together. At most two cargo processes at once,
  which the tools' slots enforce across every process started through them:
  the gate, the build lane's builds, a fix author's commands and the
  orchestrator's warm-ups and long runs. Once the batch's second lane starts
  (the widget icons, `todo_v0.7.1_batch-speed-rationale.md`, S3 as amended), these
  limits hold for both lanes together: two cargo processes, gates serialized by
  `gate.lock` (rule 2), four agents (six where two run no cargo), one
  asset run at a time, never two Fable agents.
- **A gate failed twice** by the same task is not dispatched a third time: the
  orchestrator takes the task over, re-specifies it, or sends it once as
  `implement-hard` (Fable; the only Fable agent then running), and says which.
- **A task that needs more than its parts name** (a model field, a change in
  another crate, a decision the rationale did not take) stops, and the
  maintainer is asked. In particular: the carried file failing a strict-panic or
  clippy leg (A.4: it is not edited, nothing is allowed by the implementer), and
  a hook refusing the carried file or the table.
- **A gate that fails only on a merged tree** (MK1 to MK4): the author of the
  change that fails it fixes it on `v0.7.1-rc2`, in the core worktree, while no
  other agent works there; the fix is reviewed again as in §3 by the reviewer
  of that change, passes the done-criterion of the unit whose files it
  touches, and the gate runs again before the next merge.
- **Upstream moving during the work** (a new GPUI Kit, iced, egui or GPUI Fast
  release, a new advisory) is reported with what it costs before anything is
  changed for it.
- **A fix KR, KD or K5 demands** is a code change after units 1 to 3: an
  `implement` agent makes it in the core worktree, a `review` agent reviews it,
  it passes the done-criterion of the unit whose files it touches and the gate.
  **A fix K5 needs** follows rule 15 (batch-speed rationale,
  Appendix), cited here and not restated; where the rule resumes the close-out
  at its first step, `cargo audit`, that step is K5 step 1; step 0, which stands
  for V's check (rule 22 (c)), runs again before it, since the fix touched a
  stamped path its grep reads. The stamped paths it names are
  `scripts/update_provenance.sh`'s `SOURCE_PATHS` (`:39-71`) and the
  compatibility stamp's inputs (`scripts/update_compatibility.sh`'s
  `source_paths`, `:69-80`); the compatibility run's uncommitted files, which
  the rule leaves unstaged in any fix between K5 step 2 and the asset commit
  (with an unfinished asset run's partial outputs where there are any), are
  `docs/COMPATIBILITY.toml` and the three connectors' READMEs
  (`scripts/update_compatibility.sh:316-318` writes them as each connector
  passes), each named in the running record (§4's staged-tree check). The
  maintainer is told what stopped it, what was changed, and whether another
  capture window is needed.
- **The capture windows** are asked for when everything before them is green:
  KD's once KR is done, K5's once steps 1 to 5 are (CI and the full check of
  step 4 both green, rule 9's reading: step 4 failing on the provenance stamp
  alone).

## 7. The maintainer's part

**Settled by the maintainer, 2026-11-15** (`todo_v0.7.1_batch-speed-rationale.md`):

- D10 (a): step 1 ships in v0.7.1 (the specification's open point 1); the KDE
  lane's next release starts from v0.7.1's gated head (§2.1);
- S1 to S5, S3's amendment and S6;
- S2's amendment (the next release is merged with its `<prev-tag>`, not rebased
  onto it), the rules 1 to 26 of that document's Appendix, which this plan follows
  (rule 26 does not apply here: P0 adds the documents; later refinements of rules
  2 and 12, S3 and S6 reported to the maintainer), and the three re-readings
  those rules name: rule 20's branch point (the next release starts from v0.7.1's
  documents unit, MK4, gated `--quick`, not from its last code unit, MK3), rule
  7's branching from `<prev-tag>` where that tag exists when P2 runs, and rule 9's
  "green" (CI green, the full check's only failure the provenance stamp, which
  only the asset run refreshes);
- the gate-speed specification's open points: O1a and O1b no, O2 (e), O3's keyed
  option, and decision 4 (y), the reviewed registry list (it puts a review on the
  GPUI plan's L1 and L2); the gate-speed documents are written for these answers,
  and the gate-speed plan's Task 0 records them before P0;
- the build lane's extra build folder (tens of GB, under
  `/run/media/tibi/work/cargo-build/`), shared with the GPUI plan and deleted at
  MK3 (the core worktree `v071` keeping its own build folder after v0.7.1 is no
  departure: that folder existed before the batch), and opening a draft pull
  request at a branch's first push;
- "When the plan that goes in the 0.7.1 version is ready, without errors, start
  implementing it": the 0.7.1 plans are approved as written, their own additions
  and departures included; here the shared close-out (§2.1, K5) for both plans,
  KR's rehearsal without the desktop and KD's dry parity run before it, and
  `check_text.py` on unit 4's texts (§4, *Texts no gate reads*). They are
  implemented once free of errors.
- **Commit (0)** (the GPUI plan's P0) takes v0.7.1's documents, this plan's
  among them, the batch-speed rationale and the gate-speed specification and
  plan, so there is no unit 0 (K0 (2)); the cycle's other documents (the caret's,
  the icons', the `docs/todo_v0.7.x_*` ones and the egui 0.37 rationale,
  `docs/todo_v0.7.2_egui-0.37-rationale.md`) are not approved yet and stay
  untracked (the GPUI plan's §7).

**During the work, the maintainer is told:**

- C.1's link not holding `contrastF` at the cited lines (KF);
- the carried file failing a lint leg, or a hook refusing it (K1);
- what KR and KD's comparator report, and any fix it leads to;
- a red CI run, and anything that stops a task under §6.

**The maintainer is asked:** for the two capture windows, each
announced with its purpose and the desktop time (4 to 6 minutes at the last
release, the first plan's §8): KD's dry run, and K5 step 6.

**Near the end:** what changed in the KDE screenshots (K5 step 7).

**After K5:** whether to merge, tag and publish; whether the pull request's
title and description are updated; whether the upstream requests are sent. The
reply to the reporter waits for the release of steps 4 and 5
(`docs/todo_v0.7.1_batch-speed-rationale.md`, S5).

## 8. Time

Measured at the last release (the first KDE plan's §8): the full
`check_release.sh` about 23 minutes, the asset script 30 to 37 minutes of which 4
to 6 need the desktop, a CI run 21 to 27 minutes. A `--commit` run and this
plan's tasks were not measured on this tree. The longest chain runs through the GPUI plan's
core lane to its G5, then MK1, MK2, MK3, KD, MK4 and K5, with K1 to K3 and KF
beside the GPUI plan's E2 and Part G, and K4 beside KR and KD.

## Running record

The orchestrator's running record of this plan, copied at the close-out (K5 step 7) from `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.1_kde-open-items-step1-plan/record.md`; its headings are one level down.

## Running record — 0.7.1, issue #4's open KDE items, step 1 (plan `docs/todo_v0.7.1_kde-open-items-step1-plan.md`)

Only the orchestrator writes this file.

### 2026-11-16 — K0 (1) at the GPUI plan's P1

The work folder `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_kde-open-items-step1-plan/` holds
`gates/rehearsal.sh` (from P1; the GPUI plan's record). No unit 0: this plan's rationale, specification and plan
are in the GPUI plan's commit (0), `e866ea6c`.

### 2026-11-17 — the build lane switched; K0 (1a)

- Session `native-theme-31`. This record created now (it did not exist).
- `v071-lane` switched to `v0.7.1-rc2-kde` at `30b3e6d5` (L2), after the GPUI plan's BT (`cd770099`, review
  passed, nit `1a3e352b`) and BX (`e49c48f6`, read) on `v0.7.1-rc2-gk-connector`.
- `git diff --quiet e49c48f6 HEAD -- Cargo.lock` exits 1 (L1, L2): the gpui legs are part of K0 (1a).

### 2026-11-17 — K0 (3), the quote files (done)

- A `worker`: 26 quote files and `INDEX.txt` in `<work>/quotes/` (G.1 4, G.2 2, G.3 12 of which 7 only where
  the reply is not posted, G.5 8); scripts in `<work>/K0/` (`make_quotes.py`, an independent
  `check_quotes.py`: `RESULT: all ok`, 26 byte-equal, 16 markdown blocks covered, `check_text.py` ok 26/26).
- Open for K4's brief: G3-12 (the note after the reply) is filed as "only if not posted"; to be confirmed
  against G.3 §8 when K4's brief is written. G2-2 is a weak quote (the count is G.2's grep's).

### 2026-11-17 — K0 (1a) done; K1 dispatched

- K0 (1a), first builds and clippy warm-ups in `v071-lane` (native-theme with NT_FEATURES; iced with and
  without features; gpui on both cores, the lock having moved): `<kwork>/K0/run-1`, STATUS 0, 476 s.
- K1 dispatched to an `implement` agent in `v071-lane` (clean-room rules in the brief: no KDE/Qt source opened;
  only the two carried files read from `v0.8.0-rc1`).

### 2026-11-17 — K1 committed; its review and K2 running

- **K1: `2e1c4af8`** on `v0.7.1-rc2-kde` (`implement`). H.1's three hashes match (b1b8fa52…, 92068628…,
  8328e762…). Red before: 9 failed of 188, plus `a_custom_scheme_moves_the_menu_panel_and_the_grid` not
  compiling; after: `kde::` 191 passed. native-theme lib 1228 (NT_FEATURES) / 796; gpui 401/332/18 and
  399/332/18, `--no-default-features` 294; claim test both cores; clippy five; strict panic four (A.4's case
  passes); private-items rustdoc; `check_panic_patterns.py`; five checkers; fmt. Departures: `cargo fmt --check`
  and `cargo metadata` run outside the wrappers (compile nothing); rustfmt put the `shade` import above
  `crate::Rgba` (B.2 says after). Part C is KF's.
- K1's `review` and K2 (`implement`) dispatched.
- K1's `review`: **merge**, no must-fix, no should-fix. The reviewer recomputed the three hashes from `2e1c4af8`
  and from `78ee0efd`; checked B.3's vector rows by hand, the contrast fallback (7, decision 14), missing groups,
  the exceptions in both builds; no transcribed KDE/Qt structure. Nits, not taken (no code change): one test's
  first half guards a regression and is not red alone; the body does not name rustfmt's import order (recorded
  here). Unit 1's code part is closed; KF (Part C) follows in the documents lane after M1.

### 2026-11-17 — K2 committed and read; K3 dispatched

- **K2: `41c73d48`** (`implement`): red before (`colors.rs:846`, kde-breeze Light chart_grid not
  separator.line_color), green after; gpui both cores all targets (401/18/332, 399/18/332), `--no-default-features`
  294, runner env 332 ×3, clippy three, strict panic four, `check_panic_patterns.py` on the gpui crate,
  private-items rustdoc, fmt; pinned-lines `colors.rs` L=718: six hunks, all `ok` (the agent added
  `END { exit bad }` to its copy of the awk check).
- Orchestrator's reading (K2's reviewer): exactly D.1, D.2's first bullet, D.3. The agent's remark that D.4's
  kde-breeze row is stale does not hold: D.4 gives kde-breeze's chart grid as #bcc0bf / #4d545b before and after,
  which is `separator.line_color`; its red run compared with unit 1's new `list.grid_color`. H.3's manual Theme
  Map check stays for KD.
- K3 dispatched to an `implement` agent in `v071-lane`.

### 2026-11-17 — K3 committed; its review running

- **K3: `1cc99825`** (`implement`): decision 12's test red first (`showcase-iced.rs:29099`, kde-breeze/light
  Active: menu_bar_style and styles::aw::menu differ), decision 13's red after E.2/E.3 (`:29183`, both Infos name
  menu.border.line_width while the styles draw 0 at 3); green after. `contract-leaves.toml` regenerated by the
  ignored `regenerate_contract_leaves` test; its diff is E.4's expected one (in the body). iced lib 193, showcase
  208 / 194 in the runner env, `--no-default-features` 114; clippy both; strict panic three; rustdoc both; fmt.
  Left: `aw_menu_inks` still reports `bar.border` (no Info row asks; E.6 does not name it) — sent to review.
- K3's `review` dispatched. The build lane's three units are committed: K1 `2e1c4af8`, K2 `41c73d48`, K3
  `1cc99825`; MK1–MK3 wait for the GPUI plan's G5.
- K3's `review`: **merge**, no must-fix. Should-fix taken: decision 12's test gets a shadow-off case
  (kde-breeze/light with `popover.border.shadow_enabled = false`), since no bundled combination turns it off and
  the `Shadow::default()` branch was untested (a gap in the spec's decision 12 too). Nit taken: `AwStatus` alias.
  Nit left: `aw_menu_inks`' unused `bar.border` lookup (outside E.6, not a defect). Sent to K3's author as the
  lane's next commit.
- **K3's should-fix: `14229f12`** (the author): a "kde-breeze/light, no shadow" case in decision 12's test and the
  `AwStatus` alias; red with the `if` removed from either function (`showcase-iced.rs:29100`); showcase 208,
  iced lib 193, clippy, strict panic, fmt. Read by the orchestrator (test-only, 30 lines): correct.
- The build lane is done: K1 `2e1c4af8`, K2 `41c73d48`, K3 `1cc99825` + `14229f12`, every review closed.
  MK1 takes up to K1 (+KF), MK2 K2, MK3 K3 with its fix, after the GPUI plan's G5.

### 2026-11-17 — the documents lane switched; KF dispatched

- After the GPUI plan's M1 (`2bf76d57`, pushed): `v071-docs` switched to `v0.7.1-rc2-kde-docs` at `2bf76d57`.
- KF dispatched to a `worker` (C.1, C.2; the one kcolorscheme link opened for its citation, no code).
- **KF: `76ce9ace`** on `v0.7.1-rc2-kde-docs` (worker): the kcolorscheme link at `27066d47`, read raw with curl
  (697 lines), holds `KColorScheme::contrastF` at 519–527 (526 reads the `KDE` group's `contrast`, default 7):
  the citation stands. C.1's three cells and C.2's two `why_wrong` strings, each replacing only its text; five
  checkers 0. Read by the orchestrator (word diff). Unit 1 complete: K1 `2e1c4af8` + KF `76ce9ace`, for MK1.

### 2026-11-17 05:00 — waiting for MK1

The GPUI plan's G5 is committed and gated (`2a885f6f`); MK1 waits for that plan's E2 review, which the weekly
limit stopped (subagents refused until 20:00 Europe/Madrid). Ready for MK1: K1 `2e1c4af8` + KF `76ce9ace`.

### 2026-11-17 05:05 — MK1 merged before E2's review closed (a departure)

- The maintainer objected to the pause. MK1's row waits for the GPUI plan's E2 review; that review is the
  citations' hand-read (comments and documents only), and no KDE unit touches its lines, so MK1 goes ahead and
  any E2 fix lands after it, gated by its own `--quick`. Departure reported to the maintainer.
- E2's review agent resumed by SendMessage (result pending).
- MK1: `096079e2` (K1 `2e1c4af8`, clean, git's auto-merge in gpui `contract.rs`) and `9b1e64d9` (KF `76ce9ace`).
  No unit 0. `cargo package -p native-theme --list` holds `tests/vectors/kde_shade.tsv`.
  Gate: `--quick` + GPUI Fast legs, run folder `<kwork>/MK1/run-1/`.
- K4's issue read (2026-11-17 05:08, `gh issue view 4 --json comments`): two comments, tiborgats 2026-10-22
  ("Hi @l-raider, thanks for asking…") and l-raider 2026-10-30; no owner comment carries the draft's opening
  sentence: **not posted**, so G.3's reply edits go ahead (and G3-12's quote file is used).
- MK1's gate: `--quick` 134 ✅ / 13 unchanged, 13m 25s, GPUI Fast legs 0, tree clean (`<kwork>/MK1/run-1/`).
- E2's review fix `1a7d31a8` (GPUI plan's record), then MK2 `9c4d4591` (K2 `41c73d48`) and MK3 `cdd2f191`
  (K3 `1cc99825` + `14229f12`), git's auto-merge in gpui colors.rs/contract.rs and iced styles.rs/showcase-iced.rs,
  no conflict. **One `--commit` + GPUI Fast legs for MK2 and MK3 together** (a departure: the second `--commit`
  would check the same tree class; saves one gate), run `<kwork>/MK3/run-1/`.
- Documents lane: `c256e422` merges `v0.7.1-rc2` at MK3 into `v0.7.1-rc2-kde-docs`. K4 dispatched to a `worker`
  (shas: item 1 and 3 `cdd2f191`, item 2 `096079e2`; reply not posted).
- KR queued behind the gate (`<kwork>/KR/run-1/`): the three release builds (they serve KD too), the rehearsal on
  :97 under xvfb97.lock, `check_showcase_parity.py` on the dumps.
- **K4: `580ac4dc`** on `v0.7.1-rc2-kde-docs` (worker): G.1, G.2, G.3 (reply edits, not posted), G.5; 26 quote
  files `ok`; five checkers 0; four files only. G.2's count **47**, not the spec's 46: the port added three
  `[v0.7.x]` entries (15b26f64, 30a83933, e22dc00c; 44 at both 2bf76d57 and c256e422, recounted by the
  orchestrator), K4 three. Read by the orchestrator (word diff of all four files): each text where Part G puts it.
  MK4 waits for KR and KD.
- MK2+MK3's gate: `--commit` 113 ✅ / 34 unchanged, 6m 50s, GPUI Fast legs 0, tree clean (`<kwork>/MK3/run-1/`).
  Core log: 096079e2, 9b1e64d9 (MK1), 1a7d31a8 (E2 fix), 9c4d4591 (MK2), cdd2f191 (MK3): the three merges in
  unit order.
- §2.2: the build lane's build folder `/run/media/tibi/work/cargo-build/e0/f9ec5b1bea498e` (48 GB) deleted,
  `v071-lane` removed, `v0.7.1-rc2-kde` and `v0.7.1-rc2-gk-connector` deleted (`-d`).
- Pushed `2bf76d57..cdd2f191` (MK3); CI run 44016819955.
- KR run-1 **void**: the orchestrator's command ran in zsh, which does not split `$T`, so the three builds failed
  (flock 69: no such file) and the rehearsal ran the 2026-11-14 binaries. Run-2 under `bash -c`
  (`<kwork>/KR/run-2/`), the builds' exit statuses checked before the rehearsal.
- **KR run-2**: builds 0 (binaries 2026-11-17 05:23-05:24), 18/18 dumps, `check_showcase_parity.py`: 6 groups,
  836 differences, 836 excepted, 0 not excepted (R-shape 210, R-snap 750); the same totals as void run-1 on the
  11-14 binaries. No difference to report. `<kwork>/KR/run-2/parity.out`.
- KD run-1 started (`<kwork>/KD/run-1/`): HELD with assets.lock, gate.lock and the slot; waits for the
  maintainer's window.
- **KD run-1** (window given by the maintainer 12:08, "capture now"; done 12:11): iced, gpui, egui statuses 0,
  each "Generated 6 screenshots", identity checks 1410x878 holding 1280x720 at +65+81. `check_showcase_parity.py`:
  6 groups, 887 differences, 887 excepted, 0 not excepted (R-glyph 51+21, R-shape 210, R-snap 750, not visible 54);
  no fix. `dry-parity.out` kept; the dump folder removed; asset folders restored; tree clean.
- H.3's manual check (the gpui Theme Map's `chart_grid` Info read on screen): **not checked**. The showcase's
  `--press` holds a press without a release, so it cannot open an Info, and the session has no desktop control.
  What it would show is the contract row and claim line that `every_colour_claim_is_read_at_the_line_it_cites` and
  the contract tests check (MK3's gate). A stray `showcase-gpui --help` window (unknown flags are ignored) was
  opened and closed during the attempt.
- CI 44016819955 on `cdd2f191`: success.
- MK4: `29e78d8b` merges K4 `580ac4dc`; the five checkers 0; `--quick` running (`<kwork>/MK4/run-1/`).
- K5 step 0: the stamped-paths grep prints nothing. Step 1, `cargo audit`: **fails** on RUSTSEC-2026-0543
  (chrono 0.4.45 unmaintained, all versions, `patched = []`), reached only through GPUI Kit (gpui-base,
  gpui-component, gpui-fast, gpui-pre, gpui-pre-scheduler 0.9.1/0.1.15/0.3.12); no workspace crate depends on it.
  Decided by `.cargo/audit.toml`'s own rule: an ignore entry with the reason (a rule 15 fix, after MK4's gate).
- KD's manual check stays "not checked" (above). Branches `v0.7.1-rc2-kde-docs` and `v0.7.1-rc2-gk-docs` deleted (`-d`).
- A departure for speed: `v071-docs` is kept past MK4 as the archive lane: branch `v0.7.1-rc2-archive` at
  `29e78d8b`; a `worker` prepares step 7's archiving (seven moves, README row, rewrites, status lines; no records,
  no assets) as one commit, which the orchestrator folds into the asset commit (`cherry-pick -n`), so step 7 stays
  one commit. The worktree is removed after the fold (§2.2).
- MK4 run-1 `--quick`: one failure, `cargo audit` (RUSTSEC-2026-0543). Fix: `.cargo/audit.toml` entry with the
  reason (commit after `29e78d8b`; `cargo audit` 0). MK4 run-2 `--quick` on it (`<kwork>/MK4/run-2/`).
- Archive prepared: **`8c0fa634`** on `v0.7.1-rc2-archive` (worker): seven R pairs (R098/R099), README v0.7.1 row
  (G.6 + the two quoted clauses), G.4's sentence, two path rewrites (0004 open-items rationale, batch-speed
  rationale), moved documents' links re-pointed; grep check: only `docs/todo.md:4442` (dated "at e866ea6c",
  exempt); link script `<kwork>/K5/check_archive_links.py`: 8 rule 22 (b) exemptions; five checkers 0. Reading
  taken: plain paths inside the seven moved documents are not rewritten (rule 22 (a) covers files outside
  docs/archive/). Read by the orchestrator (word diff of the three edited files).

- MK4 run-2 `--quick`: 147 ✅, 0 ❌, 6m 17s, tree clean (MK4 = merge 29e78d8b + audit fix e417add7).
- the lane's gated head for the next release: e417add7 (MK4; --quick run /home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_kde-open-items-step1-plan/MK4/run-2; reviews closed)
- K5 step 3 moved ahead of step 2 (speed): pushed `cdd2f191..e417add7` right after MK4, so CI runs beside the
  compatibility run (the run's changes are uncommitted and do not enter the push). Steps 2 then 4 chained under
  gate.lock: `<kwork>/K5/run-1/` (compat.log, full.log, STATUS).

### 2026-11-17 12:40 — K5 step 2 red: gpui-fast 0.1.17 released

- `update_compatibility.sh run` (`<kwork>/K5/run-1/compat.log`): the lock update took gpui-fast 0.1.15 -> 0.1.17
  (nine gpui-fast-* crates); native-theme-gpui NOT verified: `tests::the_fallback_stack_copy_is_gpuis` panics by
  design ("gpui-fast 0.1.15 is not in the cargo metadata: re-cite ..."). Tree restored, clean.
- Full check (`full.log`): 166 ✅, 2 ❌ (the asset and compatibility stamps, the asset run's), 1 ⚠️: `cargo outdated`
  exit 1 — it raises native-theme-gpui's `gpui` to gpui-pre 0.3.13, which gpui-kit, gpui-component and gpui-base
  0.9.1 pin out (`=0.3.12`); no user can get 0.3.13, so this is the tool, not a dependency to move. No list.
- Decided by logic (rule 15, fix before the release): unit **L3**, the lock moves to gpui-fast 0.1.17 (what every
  fresh resolution gets) and its citations are re-read at 0.1.17; 47 files differ 0.1.15 -> 0.1.17. Dispatched
  to `implement` in `v071`; then its review, `--commit` + GPUI Fast legs, the compatibility run again.
- CI 44073677773 on `e417add7`: running.
- L3 stopped by its agent (nothing committed, lock restored): with gpui-fast 0.1.17 gpui-base 0.9.1 does not
  compile on the GPUI Fast core (24 E0119: gpui-fast 0.1.16's `impl<T: Element> IntoElement for T`,
  `src/element.rs:146`, against gpui-base's own impls, e.g. `src/a11y_disabled.rs:63`); gpui-base 0.9.1 asks
  `gpui-fast = "0.1.14"`, so every fresh build of native-theme-gpui's `gpui-fast` feature fails today. gpui-base
  0.10.0 gates those impls (`:63-64`). `text_system.rs:255-280` identical 0.1.15/0.1.17.
- **Re-scoped, decided by logic:** native-theme-gpui gets an optional direct `gpui-fast = ">=0.1.14, <0.1.16"`
  under its `gpui-fast` feature (no version >= 0.1.16 builds with GPUI Kit 0.9.1, so the bound loses nothing;
  the taffy pin of G4 is the precedent); CHANGELOG Fixed line, README sentence, todo entry to lift it with GPUI
  Kit 0.10.0. Sent to the same agent (a small fix to its own work, ~105k). An upstream report is the maintainer's.
- **L3: `7f71f637`** (agent's `d9f26086`, amended by the orchestrator before any push): optional
  `gpui-fast-bound = { package = "gpui-fast", version = ">=0.1.14, <0.1.16", … }` under the `gpui-fast` feature
  (renamed so its lib `gpui` is not a second `--extern gpui`), Cargo.lock +1 line, CHANGELOG Fixed, README two
  sentences (the bound; the gpui-kit-only case needs this crate's feature or its own hold), todo entry. Proof (agent):
  dry run moves only gpui-fast-macros (0.1.17, allowed by gpui-fast 0.1.15's `^0.1.15`, tests pass with it); fresh
  resolution 0.1.15. Tests both cores, clippy x3, strict panic, fmt, todo citations: pass. Read by the orchestrator.
- Chain `<kwork>/L3/run-1/`: `--commit` + GPUI Fast legs, then `update_compatibility.sh run`, then the full check.
- L3 chain: `--commit` 147 ✅ (17m 30s), GPUI Fast legs 0; **K5 step 2** `update_compatibility.sh run` 0: all three
  verified on 2026-11-17 (gpui: gpui-fast 0.1.15 family, gpui-fast-macros 0.1.17; iced 0.14.0/iced_aw 0.14.1/
  iced_widget 0.14.2; egui 0.36.2), its changes uncommitted (three READMEs, COMPATIBILITY.toml); **step 4** full
  check: 167 ✅, 1 ❌ the asset stamp only, 1 ⚠️ `cargo outdated` (the gpui-pre 0.3.13 artifact). The list,
  `cargo outdated --workspace --depth 1 --exclude zbus,gpui`: `<kwork>/L3/run-1/outdated.txt`.
- Pushed `e417add7..7f71f637`. Step 5's release builds and the CI wait: `<kwork>/K5/run-2/`.
- Step 5 builds 0; CI 44083520418 on `7f71f637`: success (13:49). Step 6: asset run HELD (`<kwork>/K5/run-3/`),
  waiting for the maintainer's "capture now".

### 2026-11-17 — K5 step 6, the asset run (`<kwork>/K5/run-3/`, then `run-4/`)

The maintainer said "capture now"; GO at 16:04:58, the desktop free at 16:08:50 (the three kits' screenshots and
GIFs; parity at rest passed, "The three showcases agree at rest"). The script then stopped (STATUS 1) at CI's
screenshots run 44110255347 on `7f71f637`: its two macOS jobs (egui, iced) were cancelled, not failed, by GitHub:
"The job was not acquired by Runner of type hosted even after multiple attempts" (macOS arm64 capacity); the four
other jobs passed. **Departure:** instead of a second capture window, the two jobs were re-run in the same run
(`gh run rerun 44110255347 --failed`, so its headSha stays HEAD), and the script's own remaining steps (its
lines from "Verify the completed run matches our commit" to the end: the SHA check, the download, the provenance
stamp, `update_compatibility.sh run`) run unchanged from `<kwork>/K5/tail.sh` (the script's preamble, its
`SCRIPT_DIR` pinned to the worktree's `scripts/`, `RUN_ID`, `EXPECTED_SHA` and a HEAD check), under the gate lock.

The KDE screenshots against the previous ones (`0f849012`, 2026-11-14), all three kits, light and dark: the one
change is the tables' grid line, the same in the three kits (light `#BCC0BF` → `#C4C8CC`, dark `#4D545B` →
`#1C1F21`), as H.4 says (tables in Qt's grid, `list.grid_color`, the three moving together); else the spinner's
phase (animated) and the translucent window corners. Issue #4 read again: no comment since 2026-10-30, the reply
not posted, so K4's draft edits stand.

### 2026-11-17 — K5 step 7, the asset commit

CI's re-run passed (the two macOS jobs; run 44110255347 green, its six jobs), and `<kwork>/K5/tail.sh` passed
(STATUS 0): 18 screenshots downloaded (macOS 26.7.2, Windows 10.0.26100.0), `docs/assets/PROVENANCE.toml` written
for `7f71f637` (sources `d53b549bd61c`), `update_compatibility.sh run` passed (`docs/COMPATIBILITY.toml` and the
three connector READMEs' Verified lines from it). No new asset file: every asset path the run wrote was already
tracked (`git status`: modified only). One commit: those assets, both stamps, the READMEs' lines, the CHANGELOG's
0.7.1 date set to 2026-11-17, the archiving (the documents lane's `8c0fa634`, applied with `git cherry-pick -n`),
and the three running records appended to their archived plans under "Running record", headings one level down.
