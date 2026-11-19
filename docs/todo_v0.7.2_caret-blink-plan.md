# 0.7.2: the text caret's blink — implementation plan

- Rationale: [`todo_v0.7.1_caret-blink-rationale.md`](todo_v0.7.1_caret-blink-rationale.md),
  B1 to B6. Specification: [`todo_v0.7.2_caret-blink-spec.md`](todo_v0.7.2_caret-blink-spec.md)
  ("the spec"), whose *Order and gates* this plan carries out.
- Decided by the maintainer on 2026-11-19: the caret's blink is part of 0.7.2, together with
  issue #4's KDE steps 6 and 7
  ([`todo_v0.7.2_kde-open-items-steps6-7-plan.md`](todo_v0.7.2_kde-open-items-steps6-7-plan.md),
  "the KDE plan"); the version stays 0.7.2, which is not released yet. (That morning the two
  works were set for a release of their own, 0.7.3; he folded them into 0.7.2 so that no release
  changes the API a few days after another did.) This plan's units are built first; the KDE
  plan's follow on the same branch and end with 0.7.2's close-out, made again once.
- Written 2026-11-19. It replaces the plan of 2026-11-14 for a release of its own in a lane
  beside another release (git history keeps it). Status: approved by the maintainer on 2026-11-19, with the grant of the Windows reader's one `unsafe` call (the spec's open point 2).
- `<record>` = `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.3/`, the
  record of both works (the folder keeps the name it got while they were set for 0.7.3), with this work's files in `<record>/caret/` and the amendment's in
  `<record>/respec/`. `<tools>` =
  `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan/tools/`,
  0.7.2's wrappers (`slot.sh`, `lowprio.sh`, `gate.sh`, `rehearsal.sh`), used as they are; a
  gate's log is in that folder's sibling `gates/<name>/`.

## 1. Where and from what

- One worktree, `.claude/worktrees/lane2-b`, branch `v0.7.2-rc1`, from its head `da89b4ba`:
  0.7.2 through its close-out once (`b494d6ed`), then the lock's heapless 0.9.4 against nine
  advisories of 2026-11-18, which the audit of every gate and of CI denies (gated `--commit`
  with the Fast legs, pushed, CI green; draft pull request #11). The units are commits on that
  branch: no new branch, no merge, no version step. The branch is pushed by explicit refspec at
  each `--commit` gate that passes, and each push's CI run is read (a red run emails the
  maintainer). The local branch `v0.7.3-rc1`, which held the drafts and was never pushed, is
  deleted at D0.
- The assets' stamp is stale from `da89b4ba` on (it hashes the lock) and stays so until the
  close-out's asset run (the KDE plan); the unit gates do not read it.
- The evidence already taken, in `<record>/respec/` (2026-11-19, four `worker` agents, no cargo):
  - `findings-c2-0A.md`, `findings-c3-BC.md`, `findings-c4-D.md`: every anchor, count and
    upstream citation of the spec's Parts 0 to D read again on 0.7.2's tree. Every anchor
    stands; nothing of this work is built; the two evidence commits and their blobs are in the
    object store (`72fcf6a7` only on the local branch `facts-ios-m1b3`); gpui-base 0.10.0's
    `blink_cursor.rs` is byte-identical to 0.9.1's and gpui-component 0.10.0 has no other way
    to set the blink; egui 0.37.0's `TextCursorStyle`, its defaults and the caret's painting
    are 0.36.2's. The spec holds their corrections.
  - The counts the units start from: egui's 525 leaves, the inputs' `[4, 40, 32, 12]`,
    `egui-limited` 52; `one_gsettings_read_per_key` 9 and 8.

## 2. Units

Each unit is one commit; a review fix is a commit of its own, covered by the next gate.

| Unit | The spec's parts | Agent | Gate after it |
|---|---|---|---|
| D0 | Both works' documents under their `todo_v0.7.2_` names (the four `todo_v0.7.x_` files go by the rename): this plan and the spec, the KDE plan and its specification; the two rationales' status lines | orchestrator | none: no gate reads them |
| C2 | Part 0: the ten files of 0.1 (`efbffffe`'s three, `72fcf6a7`'s seven) with their blob ids and the reproduction check; 0.2's two sections of `docs/evidence/README.md` | `worker` | its own checks (the blob ids, the reproduction check); built beside C3 |
| C3 | Part A, Part B, D.2 (the spec's unit 3). First, once A.1 and A.2 compile: the leaf walks of the other crates (the spec's *Order and gates*, its second paragraph), a failure outside egui's count being a stop | `implement` | `--quick` over C2 and C3, with the five documentation checkers |
| C4 | Part C: C.1 (KDE), C.2 (GNOME, Budgie, the watcher), C.3 (Windows) with the grant, C.5's form without it; E.2's edits of `native-theme/README.md` and `lib.rs`, the crate's documentation | `implement` | `cargo test -p native-theme` with the gate's features, before C5 starts |
| C5 | D.1 (gpui), D.3 (iced), D.4 (showcases, `tools/showcase-support`) | `implement` | none of its own: built after C4 |
| C6 | Part E: E.1, E.3, E.4 and the root README's line of E.2, into 0.7.2's sections; beside C4 or C5 | `worker` | after C4, C5 and C6: one `--commit` with the GPUI Fast legs and the five documentation checkers; push; CI read |

Why this cut. C3 is one unit because the model, the presets and egui cannot pass a gate apart
(the spec's *Order and gates*: an unset leaf still serialises, egui counts it, and its tests
need two presets' values). C2 writes only under `docs/evidence/`, so it runs beside C3. C4 and
C5 touch disjoint paths and do not depend on each other, but they are built one after the other:
C4 edits the core crate, and C5's connectors compile it (`native-theme-gpui`'s `native` feature
turns on `native-theme/native`), so a C5 beside C4 would build C4's half-made edits. C6 runs no
cargo and writes root documents only (`CHANGELOG.md`, `ROADMAP.md`, `README.md`,
`docs/todo.md`, `docs/todo_gpui-full-theme.md`), which no crate compiles: `native-theme/README.md` is the core crate's
documentation and goes with C4. So C6 runs beside C4 or C5. The orchestrator commits each unit
by its paths.

## 3. Tests and gates

- During a unit: the tests the spec names for it and the ones its edits reach, through
  `<tools>/slot.sh <tools>/lowprio.sh cargo test -p <crate> … <names>`; a showcase's tests through
  `scripts/run_in_runner_env.sh`; C5's gpui tests in the GPUI Fast form too. A test the spec says
  fails without the change is seen failing first.
- Before each unit's report: `scripts/check_strict_panic.sh` for the crates it changed.
- At a gate the orchestrator runs `<tools>/gate.sh lane2-b v072-caret-c3 --quick` and
  `<tools>/gate.sh --fast lane2-b v072-caret-c456 --commit` (measured in 0.7.2: a `--commit` with the
  Fast legs takes 7 to 21 minutes). The second must name no skipped cross-target check (the
  spec's *What the gates cover*; the targets are installed: `rustup target list --installed`,
  read 2026-11-19).
- The whole suite and the release checks run in the close-out (the KDE plan).

## 4. Reviews

A reviewer is given the commit and the spec's parts, never the author's report; it runs beside
the next unit, and the reviews of C5 and C6 during the `--commit` gate. Every review is closed
before the KDE plan's first unit; a must-fix after the gate is covered by a `--quick` and pushed. A must-fix is a commit of its own before the next gate. At most one Fable agent
runs at a time, so the two `review-hard` reviews run one after the other.

| Commit | Reviewer | Why |
|---|---|---|
| C3's validation: `native-theme-derive/src/parse.rs`, `gen_ranges.rs`, `native-theme/src/resolve/validate_helpers.rs`, `validate.rs` | `review-hard`, narrow | a new rule over three fields (A.4) and a new range arm every generated validation may take (A.1), in the core's validation |
| The rest of C3 | `review` | values and texts across many files (the fields' docs, the registry and rules, the facts rows, the presets, B.2's gate, egui's rows and formulas, A.5's tests against A.4's table) |
| C4, but `windows.rs` | `review` (never the orchestrator, which has read LGPL sources for the facts and must not shape the clean-room readers) | the parsing and halving of two readers' settings, each case a test |
| C4's `windows.rs`, with the grant | `review-hard`, narrow | the crate's one new `unsafe` call: the `SAFETY` comment against the spec's quote of Microsoft's page (open point 2), the `allow` on its function alone, and that `read_windows` cannot skip the mapping |
| C5 | `review` | gpui's write rule across `apply` calls, the tests that must fail without it, iced's exceptions |
| D0, C2, C6 | the checkers, then the orchestrator reads the diff | data and texts the spec fixes |

Every reviewer also checks that no new code transcribes an LGPL source.

## 5. Briefs

Each brief quotes the spec's parts it builds by name, the head's decisions that touch them, and
the findings file of its part as a lookup of where each anchor is today (an anchor not found as
the spec quotes it is a stop); it names the files, the wrappers and the tests. The rules in every
brief: no sub-agents; no `find /`; nothing read outside the worktree, Cargo's registry and the
record; no `unsafe` but C.3's one call under the grant; no panic-prone code (the hook also
refuses `.expect(` and `.unwrap(` in tests: `let … else`); no code from an LGPL source: the
readers of C.1 and C.2 are written from the spec's text, and Qt's, KDE's and GTK's sources are
not opened; edits only with the Edit and Write tools, and a hook's refusal is reported, never
written another way; a new citation of a repository file names the document only; no commit (the
orchestrator commits after reading the report); long commands in the foreground; one report at
the end. Where two agents work side by side (C2 with C3; C6 with C4 or C5) each brief also says:
another agent edits other paths of the same tree at the same time; change, format (`cargo fmt -p
<crate>`) and stage nothing outside your own paths, which the brief lists. A reviewer's brief
says it runs no cargo.

## 6. Stops

- An anchor that is not found as the spec quotes it: the orchestrator finds its present form;
  where the edit's meaning is unchanged it is made there and the record names it, else the work
  stops.
- C2: an evidence object `git cat-file -e` does not find, or 0.1's reproduction check failing.
- A decision neither the spec nor the record makes; a task that needs more than its parts name
  (a model field, another crate); a new upstream release or advisory during the work, which is
  reported with its cost before anything is changed for it.
- A unit that fails the same gate twice is not sent a third time: the orchestrator takes it
  over, re-specifies it, or sends it once as `implement-hard`. C4 is never taken over by the
  orchestrator, which has read LGPL sources for the facts: always a fresh `implement-hard`.
- Without the grant of the `unsafe` call, C4 builds C.5's form; nothing waits.

## 7. The maintainer's part

- The approval of the rationale (B1 to B5; B6 is settled), the spec and this plan.
- One grant: the Windows reader's `unsafe` call of `GetCaretBlinkTime` (the spec's open point 2).
  Granted on 2026-11-19 (proposed was: grant it; without it Windows keeps the preset's 530 / 530 in this release).
- Told, not asked: 0.7.2's new ROADMAP heading (E.3) is his to change; the local
  branch `facts-ios-m1b3` is not touched before C2 is committed, since it alone holds the iOS
  evidence's commit; the push after C4 to C6 runs C.3's Windows-only test in CI for the first
  time, and its notice says so.
- The close-out's capture window, the pull request's merge, the tag, the publication and the
  GitHub release are in the KDE plan.

## 8. Cost check

- **What could refute the design, and is it first.** (1) That no walk over the leaves but
  egui's counts the three new ones (the spec's *Order and gates*): C3's first step, as soon as
  the fields compile, long before its gate. (2)
  That 0.7.2's toolkits still take the blink as designed: read on 2026-11-19, before the build
  (§1); D.1's and D.2's tests are seen failing first in their units. (3) Part 0's sources: read
  on 2026-11-19; C2's checks run first in C2. (4) That `read_windows()` returns `Ok` on CI's
  Windows image: only CI shows it, at this work's first push, after the `--commit` over C4 to C6. What waits for these: nothing.
- **Built twice:** nothing.
- **In parallel:** C2 beside C3 (two agents, one of them running cargo); C6 beside C4 or C5;
  each review beside the next unit. C4 and C5 are not built side by side (§2). At most four
  agents at once, one of them running cargo.
- **What each agent reads:** its parts of the spec, the decisions named in its brief, its part's
  findings file; not the rationale, not this plan.
- **Tests per round:** the unit's own; one `--quick`, one `--commit`; the full suite once, in the
  close-out.
- **Time (an estimate):** 0.7.2's three earlier folds each took 1 h 40 to 1 h 55 from the
  documents' commit to the last unit's (measured from the commits' times; their specifications
  2,800 to 3,400 lines). This work has three readers beside the three connectors and two gates:
  three to five hours with the reviews' fixes.
- **Compaction:** after the documents' approval, before C3; after this plan's `--commit`, before
  the KDE plan's first unit; earlier where the conversation passes about 300 k tokens.
