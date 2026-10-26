# v0.7.0 — GPUI Kit 0.8.1 (G4): Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development
> (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use
> checkbox (`- [ ]`) syntax for tracking.

**Goal:** native-theme-gpui and its showcase on GPUI Kit 0.8.1, gpui-pre 0.3.10 and gpui-fast
0.1.12, with:

- the showcase's keys bound and parsed without a panic path;
- every upstream citation true for the new versions;
- a lint gate for upstream functions that panic on their input;
- the toolbar's unstated height back to the content's;
- the Separator, the issue drafts and the new upstream API recorded;
- v0.7.0's documents stating what it requires.

**Architecture:**

- **What changes:**
  - the gpui connector, its tests and its showcase, plus the documents;
  - in the egui and iced showcases, only their citations of gpui sources (Task 2, Task 3b);
  - in native-theme, nothing in its code, but two doc comments re-cite gpui-pre-linux
    (Task 3b), and its property tests run on proptest 1.12 after the lock update (spec §1).
- **The unit:** Tasks 1–3 — the bump with the migration, the renumbering, and the hand-read.
- **After the unit:** Tasks 3b, 4, 5 and 7, each independent of the others, and Task 6 after
  3b and 5, so that its parity run reads the final layout. Documents and the stamp close
  (Task 8).
- **Order:** 0; 1–3 back to back; then 3b, 4, 5 and 7 in any order, with 6 after 3b and 5;
  then 8. One writer at a time throughout.

**Tech stack:** Rust 2024; the gpui connector's MSRV is 1.95.0 (re-measured in Task 1).
gpui-component, gpui-base and gpui-kit 0.8.1, gpui-pre 0.3.10, gpui-fast 0.1.12.

**Spec:** [`todo_v0.7.0_gpui-kit-0.8.1-spec.md`](todo_v0.7.0_gpui-kit-0.8.1-spec.md).
**Reasons:** [`todo_v0.7.0_gpui-kit-0.8.1-rationale.md`](todo_v0.7.0_gpui-kit-0.8.1-rationale.md).

**When:** on `v0.7.0-rc1`, before the v0.7.0 close-out (rationale P15), and only once the
review session running the C and D rounds in this tree has said that the D round is closed.
- That session asked for this on 2026-10-26. Its writers edit the connector, the showcase, the
  presets, `docs/todo.md`, `CHANGELOG.md` and the ledger, and run `check_release.sh` and cargo
  continuously.
- A dependency bump under a running writer would break its builds and gates, so task
  boundaries are not enough here.
- Until then, G4 commits only its three documents, by explicit path.
- An Opus session orchestrates. Code tasks run in `implement` agents (Opus), with a fresh
  reviewer per task.

**Needs the maintainer for:**

- a veto of any decision other than P1 and P3, which are approved;
- nothing else by default. Branch pushes are granted (2026-10-25), and desktop captures are
  run by the session, announced first.

## Global Constraints

- **No panic paths.** No `unwrap`, `expect`, `panic!`, `unreachable!`, slice indexing or
  unchecked arithmetic outside tests, and no `unsafe`. The strict panic lints run on the
  library and on the showcase example, on both cores. Once Task 4 has landed, §5's gate holds too, whatever the order of the tasks after the
  unit.
- **No invented values.** A number in a claim, comment or document is read from the cited
  source or measured.
- **Both cores.** Every gpui change is tested on both: `--features native,watch` and
  `--features gpui-fast,native,watch`. New code also lints clean with `--no-default-features`,
  which `check_release.sh --commit` and CI run (spec §0.2).
- **The project's panic hook** is not worked around. Test-code edits follow spec §0.2: an
  existing `.unwrap()` or `.expect(` stays outside the edit's text, and a new failure is an
  `assert!`.
- **Commits:**
  - explicit paths (`git commit -- <paths>`, never `-a` or `add -A`), since another session's
    files may be in the tree;
  - author the maintainer, no AI attribution trailer;
  - each message ends "(G4 Tn)".
- **Builds:** in the project's `target/`, with the global build folder. No other target or
  build directory, except `check_release.sh`'s own `target/package-verify`.
- **Shared documents.** `docs/todo.md`, the ledger and the review plan are re-read before
  editing, and only appended to. A closed todo entry moves to `docs/archive/todo_closed.md`
  with its closing commit. Entries are cited by heading and opening words.
- **Gates per task:**
  - `./scripts/check_release.sh --commit` green on the task's last commit;
  - since `--commit` skips the GPUI Fast tests (`check_release.sh`'s *test (native-theme-gpui,
    gpui-fast, native, watch)* leg), every task that
    changes the connector or the showcase — 1, 3, 3b, 4, 5 and 7, and 6 if it changes code —
    also runs `cargo test -p native-theme-gpui --features gpui-fast,native,watch --all-targets`.
  - **One exception, stated in advance:** Tasks 1–3 are one unit. Task 1's commit leaves the
    two showcase citation gates and `the_fallback_stack_copy_is_gpuis` red; Task 2's leaves
    them red only for the set-aside list; Task 3's makes the branch green. Every other commit
    is green.
- **Shared files** (`Cargo.lock`, `CHANGELOG.md`, `docs/todo.md`, the ledger,
  `docs/showcase-exceptions.toml`, the showcase, the connector) are located by content:
  - each task re-reads, at its start, the code the spec describes and checks that the
    description still holds;
  - where it does not, it stops and reports to the orchestrating session, which re-specifies
    the step;
  - a rename or a move that keeps the role and the behaviour is followed and noted.
- **Pushes:** work branches with an explicit refspec (`git push origin v0.7.0-rc1`). No tag
  and no release. No upstream issue is filed by an agent.
- **Outside the project:**
  - reading the cargo home is granted;
  - cargo's own writes there are permitted: the lock update, and `cargo audit`'s advisory
    fetch;
  - nothing else outside the project is written; a missing toolchain is installed only with
    the maintainer's permission.
  - `~/.claude/scripts/sweep-target.sh /home/tibi/Rust/native-theme` runs whenever the build
    folder passes about 200 GB, and at the close.

## Review Focus

1. **No binding can panic** (Task 1).
   - No `KeyBinding::new` remains in the showcase's non-test code.
   - `load`'s error path leaves the binding out, with one `eprintln!` line.
   - The binding table test and `the_macos_menus_keys_are_the_higs` cover all eight bindings.
2. **No vacuous test** (Task 1).
   - `secondary` fails the test on a parse error.
   - `no_inspector_toggle_remains` compares with a real key.
   - `use_preset` and `set_preset` fail the test on a preset that does not install.
3. **The Kbd row and the tooltip are back** (Task 1): four Kbds on the Typography page, held by
   `the_typography_page_draws_its_kbds`, and "Ctrl+S" or "⌘S" in the tooltip, by the test
   platform's OS.
4. **No cited number skipped or mis-moved** (Task 2), as G3 Task 2's focus.
   - gpui-pre's citations are in scope this time.
   - The unit's own lines are left alone and listed.
   - No citation is renumbered by the wrong crate's diff: the three attribution rules of spec
     §4.1 hold on their examples, and every listed chain outside `CN/` or on gpui-pre or a
     sub-crate is read against its claim.
5. **The Dialog says what 0.8.1 does** (Task 3): an entrance and an exit, on the same curve and
   duration, and no `AnimationExt`.
6. **The gate catches** (Task 4).
   - Every entry is shown to resolve, not only one; the planted call fails clippy on both
     cores. "Clean" means no unresolved-path warning in the output, in all three
     configurations, and each `allow-invalid` entry states why.
   - Every listed function's group is argued from its doc line.
   - Every existing call is guarded or allowed with its reason.
7. **The toolbar tests fail before the fix** (Task 5): the single-fixture test is red first,
   and both toolbar unit tests pass after it.
8. **The Separator is closed only on a measurement** (Task 6): the test on both cores, or the
   captures if the test cannot find the label's box.
9. **Both cores lay the showcase out alike** (Task 6): the four dump pairs compared, each
   difference fixed or explained.

---

## Task 0: Register G4

- [ ] **The three documents** are committed at
  `docs/todo_v0.7.0_gpui-kit-0.8.1-{rationale,spec,plan}.md`: the rationale in `8086a20e`, the
  spec and the plan in `02095604`, and the check passes after them (`git log -- <path>`).
  Task 0 checks that they are there.
- [ ] **`docs/todo_code-review-plan.md`** (re-read first):
  - a G4 row in the milestone table;
  - a decision line, D12: GPUI Kit 0.8.1 as milestone G4. It points at these documents,
    records the maintainer's approval of P1 and P3 (2026-10-26), and says that G4 runs after
    the D round's close, as the one writer;
  - the *Resuming* rule names G4.
- [ ] **`docs/todo_code-review-findings.md`** (re-read first): a *G4* status row and section.
- [ ] **The inputs** the spec names are in `.superpowers/sdd/todo_code-review-plan/g4/`:
  `gk081-trial-check.log`, `gk081-trial-tests.log`, `gk081-lockdiff.txt`,
  `gk081-diff-review.md`, `gk081-msrv-1.94.txt`, `trial.sh` and `trial2.sh`. If one is
  missing, the ledger says so.
- **Commit:** `docs(review): GPUI Kit 0.8.1 as milestone G4, after the D round (G4 T0)`.

## Task 1: The bump, the keys and the test migration (the unit opens)

**Files:**

- `CN/Cargo.toml`, `Cargo.lock`, `.cargo/audit.toml` (comments), and `CN/README.md` (the
  Compatibility table and the Quick start);
- `SC/app.rs`, `SC/demo.rs`, `SC/pages/feedback.rs`, and `SC/pages/typography.rs` (doc only);
- `SC/tests.rs`, `CN/src/widgets/tests.rs`.

**Depends on:** Task 0, and the review session's message that the D round is closed, on a
clean tree.

- [ ] **Spec §1:** the four floors and the alias, and the comments re-verified; the scoped
  `cargo update` with the lock-diff gate; `cargo tree` before and after; the MSRV lines
  (1.95.0 passes all three, 1.94.0 fails); `cargo audit` and the comments of
  `.cargo/audit.toml`; the README floors.
- [ ] **Spec §2:**
  - the `load` helper, `init`'s four bindings and `macos_bindings(cx)`'s four, with their
    docs;
  - `demo::kbd` and `save_tooltip(cx)` with `parse_with_os`;
  - the `KBDS` and `SAVE_KEYS` docs;
  - the grep showing no `KeyBinding::new` outside tests.
- [ ] **Spec §3:**
  - the migration of every listed site;
  - `secondary(key, os)` strict;
  - the two plain tests made `#[gpui::test]`;
  - `use_preset` asserting the install, `use_missing_preset` for the two error tests, and any
    call the assertion exposes fixed and listed;
  - `set_preset(cx, showcase, key)` asserting the install for every key but `"default"`, and
    `set_missing_preset` for the two error calls;
  - the new `the_typography_page_draws_its_kbds`.
- [ ] **Every suite on both cores:** `cargo test -p native-theme-gpui --all-targets` with
  `--features native,watch` and with `--features gpui-fast,native,watch`.
  - Expected red: the two showcase citation gates and `the_fallback_stack_copy_is_gpuis`.
  - Anything else is fixed in this task, or, if it is a claim, left to Task 3 and listed.
- [ ] **The start-up path the old bindings panicked on is covered by the suites above.** The
  showcase's test harness runs `app::init` (`SC/tests.rs:125`), as the binary does. No window
  is started for it.
- [ ] **Every upstream line number this task writes** (the docs of §2, `secondary`'s new
  citation, the manifest comments) is read at the new versions by the task's review.
- **Commit:** `build(gpui): GPUI Kit 0.8.1 on gpui-pre 0.3.10 and gpui-fast 0.1.12; the showcase binds its keys through KeyBinding::load and parses its Kbds with gpui's OS, so no binding panics, and the tests follow gpui-pre's new keystroke API (G4 T1)`.
  - Leaves red: the two showcase citation gates and `the_fallback_stack_copy_is_gpuis`
    (Tasks 2–3).

## Task 2: Renumber the moved citations

**Files:** the citations of spec §4.1's scope. **Depends on:** Task 1, immediately.

- [ ] **Spec §4.1's script setup:**
  - G3's script copied to `.superpowers/sdd/todo_code-review-plan/g4/t2/`;
  - the versions changed, gpui-pre and the sub-crates joining `FAMILY`,
    `scripts/connector_parity_exceptions.toml` and the egui and iced showcases added to the
    scope;
  - the three attribution rules in `remap.py` and `gates.py`, each shown on its example in
    `--plan` output;
  - `T1_COMMITS` set to Task 1's commits, and `BASE_REV` to Task 2's starting HEAD.
- [ ] **The run:** `--plan`, then the one `--write`, then the four gates (`gates.py` among
  them).
- [ ] **The review reads** every renumbered chain outside `CN/` or attributed to gpui-pre or a
  sub-crate against its claim (spec §4.1, rule 3), before the `--write`.
- [ ] **The commit body holds:** the command lines, the counts by form and by rule, the list
  of the unit's lines it skipped, and the set-aside list in full (each entry with its reason,
  its cited text and the repo line's text). A fix round that changes the list restates it whole.
- **Commit:** `docs(gpui): the citations that only moved between GPUI Kit 0.8.0 and 0.8.1, gpui-pre 0.3.9 and 0.3.10 and gpui-fast 0.1.10 and 0.1.12 are renumbered (G4 T2)`.

## Task 3: What the gates read, and the behaviour-changed panels (the unit closes)

**Files:** `SC/info/` (as spec §4.2 names), `SC/tests.rs` (the fallback-stack test),
`SC/inspector.rs` (`GPUI_FALLBACK_STACK`'s doc). **Depends on:** Task 2, immediately.

- [ ] **Spec §4.2:**
  - the set-aside citations in `SC/info/`;
  - the fallback-stack test and doc re-cited;
  - the Dialog panel's rewrite (entrance and exit, `motion_transition` and `exit_duration`,
    no `AnimationExt`);
  - the Table, TimeField, Sheet and Input panels re-read.
- [ ] **Gate:** both showcase citation gates and `the_fallback_stack_copy_is_gpuis` pass on
  both cores, and `scripts/check_release.sh --commit` is green. The unit is closed.
- **Commit:** `docs(gpui): the claims GPUI Kit 0.8.1 changed are rewritten to what its code does, the Dialog's exit among them (G4 T3)`.
- [ ] **Push** `v0.7.0-rc1`, then CI green, including *Test the gpui connector on GPUI Fast*.
  A failure is fixed in G4 before Task 3b.
- [ ] **The dependency canary dispatched on the branch** (`gh workflow run
  dependency-canary.yml --ref v0.7.0-rc1`). It is green, or each failure is read and found
  outside the gpui stack (spec §10).

## Task 3b: The citations no gate reads, and the version-stating prose

**Files:** spec §4.3's.

**Depends on:** Task 3. The script's set-aside list is read with `git show` from the last
Task 2 commit that states it.

- [ ] **Spec §4.3:**
  - the set-aside citations outside `SC/info/`;
  - `DrawnFamilies`' docs against the new cache;
  - the connector's dialog docs;
  - the version grep with its present-tense claims and measurements, `native-theme/src`'s two
    gpui-pre-linux citations among them;
  - the counts re-measured.
- [ ] **Gates:** the digits-only check over the re-pointed lines, each new line read by hand;
  the second grep with the kept lines listed; `--commit` green; and the GPUI Fast tests.
- **Commit:** `docs(gpui): the citations no gate reads and the version statements follow GPUI Kit 0.8.1 (G4 T3b)`.

## Task 4: The gate for upstream functions that panic on their input

**Files:** `CN/clippy.toml` (new), `CN/src/lib.rs` (the deny), `SC/main.rs` (the deny),
`SC/inspector.rs`, `SC/demo.rs`, `SC/tests.rs` (the allows), and whatever further call sites
the inventory finds. **Depends on:** Task 3 (after the unit).

- [ ] **Spec §5's inventory:** the 172 candidate lines, each function assigned to a group with
  the reason, and each set-aside line with its reason, in the commit body.
- [ ] **`CN/clippy.toml`:** one entry per listed function, with a reason citing its doc line,
  and the header line about the upgrade sweep.
- [ ] **The two `#![deny(clippy::disallowed_methods)]`.**
- [ ] **Today's calls:** `resolve_font` and `segmented_width`'s `shape_line` allowed, each with
  spec §5's reason. Test code allowed. Any other call decided.
- [ ] **Spec §5's done-criterion, for every entry:**
  - the unresolved-path warning measured (its text, and whether `-D warnings` makes it fatal);
  - the entries that resolve only in some configurations given `allow-invalid = true` with
    their reasons, or left out;
  - the clean runs in all three configurations plus one planted call, or one planted call per
    entry if the wrong entry draws no warning.
- [ ] **Gates:** `--commit` green, and the GPUI Fast tests and clippy.
- **Commit:** `build(gpui): clippy's disallowed-methods gate lists the upstream functions that panic on their input; the inspector's guarded resolve_font, segmented_width's shape_line and the test calls are allowed with their reasons (G4 T4)`.

## Task 5: The toolbar's unstated height

**Files:** `CN/src/geometry.rs`, `CN/README.md` (the `toolbar` row), `SC/info/buttons.rs`,
`SC/tests.rs` (the comment). **Depends on:** Task 3 (after the unit).

- [ ] **Red first:** `toolbar_leaves_min_height_unset_without_a_bar_height`'s unstated
  expectation changed to `Some(gpui::Length::Auto)`, with its new name and doc, and
  `toolbar_carries_the_models_toolbar`'s likewise. Both fail on today's builder; the
  single-fixture test's output is kept for the body.
- [ ] **Spec §6:**
  - `None => r.min_h_auto()`;
  - the doc;
  - both tests pass;
  - the README row, the Widget Info claim and the showcase test's comment.
- [ ] **Gates:** `--commit` green, and the GPUI Fast tests.
- **Commit:** `fix(gpui): geometry::toolbar resets the minimum height to auto where the theme states no bar height, since gpui-component 0.8.1's Toolbar sets min_h_8 where 0.8.0 set h_8, so such a bar sizes to its content again (G4 T5)`.

## Task 6: The Separator, both cores' layouts, parity, and the upstream entries

**Files:**

- `SC/tests.rs` (the Separator test);
- `docs/todo.md`, `docs/archive/todo_closed.md` (for any entry that closes), `ROADMAP.md`
  (its upstream list);
- the exception files and any code a parity or layout difference needs;
- `.superpowers/sdd/todo_code-review-plan/g4/t2/lookup.py` (new, untracked).

**Depends on:** Tasks 3b and 5, so that the comparisons read the final layout.

- [ ] **Spec §7's Separator test**, passing on both cores, or the capture fallback with its
  reason.
- [ ] **The Separator entry** closes with its dated line and moves to the archive, or stays open
  with the measured positions (and the rationale's §1.8 corrected in its own commit).
- [ ] **The two drafts' dated lines** (spec §7). Their texts are not rewritten.
- [ ] **Announce the desktop runs.** Then:
  - spec §7's dumps on both cores, compared pair by pair, each difference fixed or explained in
    the ledger;
  - the Linux parity run against iced and egui: a new difference is fixed or excepted with its
    measured reason, and a stale exception is removed. The three asset directories are restored
    afterwards, and `git status` shows none of them changed.
- [ ] **Spec §7's re-check of the upstream entries:**
  - `lookup.py` written, with its own check against Task 2's moved numbers;
  - each open `[upstream]` gpui entry whose cited file changed in 0.8.1's non-test code
    closed, or given its dated line with its 0.8.1 lines from `lookup.py`;
  - the two gpui-fast entries' dated lines;
  - ROADMAP's upstream list re-cited in place.
  The commit body lists each entry re-read and its outcome.
- [ ] **Gates:** `--commit` green, and the GPUI Fast tests (the Separator test is code).
- **Commits:**
  - `test(gpui showcase): the labelled Separator's label is centred on its line on both cores, since gpui-pre 0.3.10 lays out on taffy 0.15 (G4 T6)`;
  - `docs(todo): GPUI Kit 0.8.1 — the Separator entry <closes | stays open>, the issue drafts and the upstream entries re-checked, ROADMAP's list re-cited (G4 T6)`;
  - a layout or parity change, if any, each in its own commit.

## Task 7: The new upstream API recorded

**Files:** `docs/todo.md`, `SC/info/chrome.rs` (the Dialog's placement row) and
`SC/info/typography.rs` (the inline-code row). **Depends on:** Task 3.

- [ ] **Spec §8:** the three todo entries (the Dialog's placement, Markdown, and the iced and
  egui gate), with their tags and reasons, and the two Widget Info rows.
- [ ] **Gates:** `--commit` green.
- **Commit:** `docs(gpui): Dialog::centered and TextView's text styles are recorded as model questions, and the Dialog and TextView panels say what the theme cannot reach yet (G4 T7)`.

## Task 8: Documents, stamp, gates, close, archive

**Depends on:** Tasks 0–7 and 3b.

- [ ] **Spec §9:**
  - the README's *Backends* section, alias comment and rows;
  - the CHANGELOG (*Changed* rewritten in place, *Added* for the gate);
  - ROADMAP;
  - the full-theme note;
  - the review plan and the ledger.
- [ ] **`scripts/update_compatibility.sh run gpui`, `run iced` and `run egui`**, each after that
  connector's last source change (the iced and egui stamps for their showcases' re-cited
  lines).
  Its own commit.
- [ ] **The full `./scripts/check_release.sh`**, with spec §10's expected items only. Right
  after it, the packaging check with `--features gpui-fast`.
- [ ] **Push**, then CI green, and the dependency canary dispatched on the branch again
  (spec §10).
- [ ] **Close:**
  - the three documents to `docs/archive/`, with an *As built* note at the top of the spec;
  - the archive index row;
  - D12 and the ledger's links pointed at the archive;
  - the ledger's G4 status: done;
  - the memory;
  - `~/.claude/scripts/sweep-target.sh /home/tibi/Rust/native-theme`.
- [ ] **For the close-out:** spec §10's list of screenshots that change goes to the close-out's
  `generate_assets_release.sh` run.
- **Commits:**
  - `docs(gpui): GPUI Kit 0.8.1 in the changelog, README, roadmap and notes (G4 T8)`;
  - `docs(gpui): the compatibility stamp for GPUI Kit 0.8.1 (G4 T8)`;
  - `docs: archive the implemented GPUI Kit 0.8.1 design documents (G4 T8)`.
