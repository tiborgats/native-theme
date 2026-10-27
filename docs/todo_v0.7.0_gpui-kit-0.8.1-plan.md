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

**When:** on `v0.7.0-rc1`, before the v0.7.0 release (rationale P15), after the code review's
close.
- The review session asked on 2026-10-26 that G4 wait until it said the D round was closed:
  its writers edited the connector, the showcase, the presets, `docs/todo.md` and
  `CHANGELOG.md`, and ran `check_release.sh` and cargo continuously.
- It said so the same day: the review's plan and ledger archived at `60fa77fd`, the branch
  pushed at `7f011f00` (after D15's last fix), its writers stopped.
- The start is the maintainer's. Until then, G4 commits only its three documents, by explicit
  path.
- An Opus session orchestrates. Code tasks run in `implement` agents (Opus), with a fresh
  reviewer per task.

**Needs the maintainer for:**

- the start;
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
- **Shared documents.** `docs/todo.md` is re-read before editing, and only appended to. A closed todo entry moves to `docs/archive/todo_closed.md`
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
- **Shared files** (`Cargo.lock`, `CHANGELOG.md`, `ROADMAP.md`, `docs/todo.md`,
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
  - cargo's own writes there are permitted: the downloads of the lock update, `cargo fetch`
    and `cargo package`, `cargo audit`'s advisory fetch, and the full `check_release.sh`'s
    removal of its own unpacked `native-theme*` copies from the registry
    (`scripts/check_release.sh:1105-1107`);
  - `~/.claude` is granted (Task 8's memory);
  - nothing else outside the project is written; a missing toolchain is installed only with
    the maintainer's permission.
  - `~/.claude/scripts/sweep-target.sh /home/tibi/Rust/native-theme` runs whenever the build
    folder passes about 200 GB, and at the close.

## Review Focus

1. **No binding can panic** (Task 1).
   - No `KeyBinding::new` remains in the showcase's non-test code.
   - `load`'s error path leaves the binding out, with one `eprintln!` line.
   - The binding table test, its Mac run (`the_bindings_take_cmd_under_macos`) and
     `the_macos_menus_keys_are_the_higs` cover all eight bindings.
2. **No vacuous test** (Task 1).
   - `secondary` fails the test on a parse error.
   - `no_inspector_toggle_remains` compares with a real key.
   - `use_preset` and `set_preset` fail the test on a preset that does not install.
3. **The Kbd row and the tooltip are back** (Task 1): four Kbds on the Typography page, held by
   `the_typography_page_draws_its_kbds`, and "Ctrl+S" off macOS or "⌘S" on it in the tooltip
   (the modifier by gpui's runtime OS, drawn by the compile target, spec §2.2).
4. **No cited number skipped or mis-moved** (Task 2), as G3 Task 2's focus.
   - gpui-pre's citations are in scope this time.
   - The unit's own lines are left alone and listed.
   - No citation is renumbered by the wrong crate's diff: the three attribution rules of spec
     §4.1 hold on their examples, and every listed chain outside `CN/` or on gpui-pre or a
     sub-crate is read against its claim.
5. **The Dialog says what 0.8.1 does** (Task 3): an entrance and an exit, on the same curve and
   duration, and no `AnimationExt`.
6. **The gate catches** (Task 4).
   - Every entry is shown to flag a call, not only one: the planted function draws exactly one
     `disallowed_methods` error at each planted line that a configuration builds, counted per
     line (spec §5), in each of the three configurations, GPUI Fast included, and each
     `allow-invalid` entry states why.
   - Every listed function's group is argued from its doc line.
   - Every existing call is allowed, with its reason; a guard comes in addition where needed.
7. **The toolbar tests fail before the fix** (Task 5): the single-fixture test is red first,
   and both toolbar unit tests pass after it.
8. **The Separator is closed only on gpui-pre's measurement** (Task 6): the test, or a capture
   if the test cannot find the label's box there; not centred, the test is ignored with the open
   entry as its reason. GPUI Fast asserts where it finds the box.
9. **Both cores lay the showcase out alike** (Task 6): the four dump pairs compared, each
   difference fixed or explained. The same for gpui-pre on taffy 0.15.1 against 0.15.0, and
   the temporary manifest and lock restored.

---

## Task 0: Open G4's Record

- [ ] **The three documents** are committed at
  `docs/todo_v0.7.0_gpui-kit-0.8.1-{rationale,spec,plan}.md`: the rationale in `8086a20e`, the
  spec and the plan in `02095604`, and the check passes after them (`git log -- <path>`).
  Task 0 checks that they are there.
- [ ] **The *Record*** (rationale P15): a `## Record` section at the end of this plan, opened
  with the review's close (`60fa77fd`, the branch at `7f011f00`), the maintainer's approvals
  (rationale §3) and the date of the maintainer's start. Each task adds, in its own last commit,
  each failure or difference it explains rather than fixes; it names no commit hash
  (`git log --grep 'G4 T'` lists them). The archived review plan and ledger are not edited.
- [ ] **The inputs** the spec names are in `.superpowers/sdd/todo_code-review-plan/g4/`:
  `gk081-trial-check.log`, `gk081-trial-tests.log`, `gk081-lockdiff.txt`,
  `gk081-diff-review.md`, `gk081-msrv-1.94.txt`, `trial.sh` and `trial2.sh`. If one is
  missing, the *Record* says so.
- **Commit:** `docs(gpui): the GPUI Kit 0.8.1 plan's Record, after the code review's close (G4 T0)`.

## Task 1: The bump, the keys and the test migration (the unit opens)

**Files:**

- `CN/Cargo.toml`, `Cargo.lock`, `.cargo/audit.toml` (comments), and `CN/README.md` (the
  Compatibility table and the Quick start);
- `SC/app.rs`, `SC/demo.rs`, `SC/pages/feedback.rs`, and `SC/pages/typography.rs` (doc only);
- `SC/tests.rs`, `CN/src/widgets/tests.rs`.

**Depends on:** Task 0, on a clean tree.

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
  - the new `the_typography_page_draws_its_kbds` and `the_bindings_take_cmd_under_macos`.
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
  dependency-canary.yml --ref v0.7.0-rc1`). It is green, or each failure is read as spec §10
  says: every gpui failure is G4's, and gpui lines the step never reached are run by hand.

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

- [ ] **Spec §5's inventory:** the 279 candidate lines (172 doc-comment, 107 plain), each function assigned to a group with
  the reason, and each set-aside line with its reason, in the commit body.
- [ ] **`CN/clippy.toml`:** one entry per listed function, with a reason citing its doc line,
  and the header line about the upgrade sweep.
- [ ] **The two `#![deny(clippy::disallowed_methods)]`.**
- [ ] **Today's calls:** `resolve_font` and `segmented_width`'s `shape_line` allowed, each with
  spec §5's reason. Test code allowed. Any other call decided.
- [ ] **Spec §5's done-criterion, for every entry:**
  - the planted function calling every entry, and its three clippy runs, each with exactly one
    `disallowed_methods` error per entry built there, in the commit body;
  - `allow-invalid = true` only on entries absent from some Linux configuration, each with
    its reason;
  - the planted function removed, and the three runs passing with no unresolved-path
    warning.
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
- `CN/Cargo.toml` and `Cargo.lock`, changed for the taffy 0.15.1 measurement only and restored, never committed;
- `.superpowers/sdd/todo_code-review-plan/g4/t2/lookup.py` (new, untracked).

**Depends on:** Tasks 3b and 5, so that the comparisons read the final layout.

- [ ] **Spec §7's Separator test**, passing on both cores; or with GPUI Fast's half under a
  `cfg` with its reason; or, if gpui-pre cannot find the box, the capture there; or, if gpui-pre
  does not centre the label, committed `#[ignore]`d with the entry as its reason.
- [ ] **The Separator entry** closes with its dated line and moves to the archive, or stays open
  with the measured positions (and the rationale's §1.8 corrected in its own commit).
- [ ] **The two drafts' dated lines** (spec §7). Their texts are not rewritten.
- [ ] **Announce the desktop runs.** Then:
  - spec §7's dumps on both cores, compared pair by pair, each difference fixed or explained in
    the *Record*;
  - spec §7's taffy 0.15.1 measurement on gpui-pre (the suite and the four dumps on a lock
    without gpui-fast, `the_fallback_stack_copy_is_gpuis` skipped, spec §7), then `CN/Cargo.toml` and `Cargo.lock` restored and `git status`
    showing neither changed;
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
  - `test(gpui showcase): the labelled Separator's label is centred on its line on both cores, since gpui-pre 0.3.10 lays out on taffy 0.15 (G4 T6)`, or, not centred, `test(gpui showcase): the labelled Separator's centring test, ignored while gpui-pre 0.3.10 draws the label at the line's end (G4 T6)`;
  - `docs(todo): GPUI Kit 0.8.1 — the Separator entry <closes | stays open>, the issue drafts and the upstream entries re-checked, ROADMAP's list re-cited (G4 T6)`;
  - a layout or parity change, if any, each in its own commit.

## Task 7: The new upstream API recorded

**Files:** `docs/todo.md`, `SC/info/chrome.rs` (the Dialog's placement row) and
`SC/info/typography.rs` (the inline-code row). **Depends on:** Task 3.

- [ ] **Spec §8:** the four todo entries (the Dialog's placement, Markdown, the iced and egui
  gate, and gpui-fast's taffy pin), with their tags and reasons, and the two Widget Info rows.
- [ ] **Gates:** `--commit` green.
- **Commit:** `docs(gpui): Dialog::centered and TextView's text styles are recorded as model questions, with the iced and egui panic gate and gpui-fast's taffy pin, and the Dialog and TextView panels say what the theme cannot reach yet (G4 T7)`.

## Task 8: Documents, stamp, gates, close, archive

**Depends on:** Tasks 0–7 and 3b.

- [ ] **Spec §9:**
  - the README's *Backends* section, alias comment and rows;
  - the CHANGELOG (the floors' *Breaking Changes* entry rewritten in place, *Added* for the
    gate);
  - ROADMAP;
  - the full-theme note;
  - the dated line on `docs/todo.md`'s gpui screenshot entry, with spec §10's list.
- [ ] **`scripts/update_compatibility.sh run gpui`, `run iced` and `run egui`**, each after that
  connector's last source change (the iced and egui stamps for their showcases' re-cited
  lines).
  Its own commit.
- [ ] **The full `./scripts/check_release.sh`**, with spec §10's expected items only. Right
  after it, the packaging check with `--features gpui-fast`.
- [ ] **Push**, then CI green, and the dependency canary dispatched on the branch again
  (spec §10). The Close's commits are pushed after it, with CI green; being documents only,
  they need no new dispatch.
- [ ] **Close:**
  - the three documents to `docs/archive/`, with an *As built* note at the top of the spec;
  - the archive index row;
  - the *Record*'s last line: G4 closed on its date, by the archive commit that carries it;
  - the memory;
  - `~/.claude/scripts/sweep-target.sh /home/tibi/Rust/native-theme`.
- **Commits:**
  - `docs(gpui): GPUI Kit 0.8.1 in the changelog, README, roadmap and notes (G4 T8)`;
  - `docs(gpui): the compatibility stamp for GPUI Kit 0.8.1 (G4 T8)`;
  - `docs: archive the implemented GPUI Kit 0.8.1 design documents (G4 T8)`.

---

## Record

- **2026-10-26, the start.** The code review closed: its plan and ledger archived at
  `60fa77fd`, the branch pushed at `7f011f00`, the review session's writers stopped. The
  maintainer approved P1 and P3 (rationale §3) and started G4 on 2026-10-26.
- **The inputs** (Task 0): all seven are in `.superpowers/sdd/todo_code-review-plan/g4/`.
- **Task 1, the colour-claim gate's list.** On the bumped lock it lists 127 of 826 claims,
  where the trial listed 125 of 821. The two more are the code Editor's `[fill]` and `[gutter]`
  claims (`SC/info/typography.rs:313`, `:322`, citing gpui-component `input/input.rs:722` and
  `:578`), which the D round's D4 added after the trial. They moved like the others, so Task 2's
  run takes them. The prose-citation gate's 14 are the trial's 14.
- **Task 1, two ranges wider than the spec's.** The Windows mapper's round trip is cited as
  gpui-pre-windows `src/keyboard.rs:39-86` (the whole `map_key_equivalent`, the key written
  back at `:66-78`), where spec §2.1 wrote `:39-60`. `Kbd::format` is cited as gpui-component
  `kbd.rs:105-223` (the whole function, which draws by `cfg(target_os)` throughout), where
  §2.2 wrote `:105-151`, which ends inside the function's key `match`.
- **Task 1, the `KeyBinding::new` grep.** In the showcase's non-test code it finds two lines,
  both doc comments (`init`'s and `macos_bindings`'), which name the constructor that panics,
  as spec §2.1 asks the docs to. It finds no call.
- **Task 2, four numbers already wrong at the old version.** The chain review (rule 3) read
  the 112 renumbered chains outside `CN/` or of a crate new to the scope against their
  claims; 108 held. Four cited the wrong lines before this bump, and an explicit list in the
  script sets them aside unchanged for Task 3's hand-read:
  `docs/showcase-exceptions.toml`'s three placeholder entries cite gpui-base
  `src/input/base/element.rs:1967-1971`, where the placeholder draw is 0.8.0 `:2634-2638`,
  0.8.1 `:2718-2722`. Its `--open-menu` entry cites gpui-pre `src/window.rs:6022-6023`, where
  the focus path is 0.3.9 `:6014-6015`, 0.3.10 `:6058-6059`.
- **Task 2, rule 2 read over the whole note.** A family crate a note names anywhere, before or
  after the citation, in its prose or as a path's leading crate directory, counts, so 73 chains
  are set aside, among them citations a leading directory names (`SC/info/feedback.rs`'s
  `gpui-base/tooltip.rs`). Reading only the names before the citation, with a leading
  directory exempt, would have set aside 46.
- **Task 2, gate (4)'s 76 failures.** The wider scan's numbers that are no citation (a 0, a
  count, a pixel value, a CI run id) or whose file the scan, attributing each number by
  itself, finds in two crates. None moved, and all 76 are on the set-aside list as its G
  entries. Among them are citations in files the script judged unchanged that the scan cannot
  attribute, such as `CN/src/geometry.rs`'s `select.rs:591-633` and `link.rs` `:90`.
- **Task 2, old version words beside new numbers.** A renumbered citation that names the old
  version keeps the version: the egui showcase's "gpui-component 0.8.0
  src/button/button.rs:1022, :1112" and the `gpui-pre-0.3.9/` paths
  (`SC/tests.rs`, the egui showcase's `main.rs`) now hold 0.8.1 / 0.3.10 numbers. The
  version grep corrects their words.
- **Task 2, the tests.** On both cores the two showcase citation gates pass:
  127 of 826 colour claims and 14 of 976 prose citations failed before the run, none after.
  `tests::the_fallback_stack_copy_is_gpuis` stays red ("gpui-pre 0.3.9 is not in the cargo
  metadata"), Task 3's.
- **Task 3, set-aside numbers that hold.** Of the 35 set-aside chains and five gate (4) entries
  in `SC/info/`, 21 chains cite lines that did not move, in files unchanged between the
  versions (gpui-component `styled.rs`, `radio.rs`, `accordion.rs`, `link.rs`, `label.rs`,
  gpui-base `tooltip.rs`, gpui-fast `src/fast/retained.rs`, `src/fast/splice.rs`) or in the
  unchanged parts of changed ones (both `switch.rs`, whose only change is in their tests,
  gpui-kit `src/test.rs`, whose changes are `ScrollEvent` and the keystroke parsing, gpui-fast
  `src/window.rs` and `src/fast/dependencies.rs`). Task 3b moved gpui-kit `src/test.rs` here
  from the unchanged files, where Task 3 had listed it. They keep their
  numbers and, in the comments, their unqualified file names, so a later run of the script
  sets them aside again. The five gate (4) entries are no citation (a 0, a 75%) or a citation
  of an unchanged gpui-component file, and stay as they are.
- **Task 3, a version word left to Task 3b.** `SC/info/registry.rs`'s module doc names
  "gpui-fast 0.1.10" before `src/fast/retained.rs:738-825`, which Task 2 judged unchanged.
  The version grep corrects it. The re-pointed lines that named a version name the new one
  (`SC/info/registry.rs`'s "gpui-kit 0.8.1's `click`", the fallback-stack doc and test).
- **Task 3b, numbers already wrong at the old version.** Besides the chain review's four,
  now set right (gpui-base `element.rs:2718-2722`, gpui-pre `window.rs:6058-6059`), the
  hand-read found `docs/showcase-exceptions.toml`'s `window.rs:4907-4924` (three copies),
  which at 0.3.9 began 8 lines inside `should_use_subpixel_rendering` (`:4899-4916`), and
  its `:4768-4771 paint_path`, which at 0.3.9 was `paint_underline`'s doc. They now cite
  0.3.10's function (`:4917-4934`) and `paint_path`'s opacity (`:4778-4781`). Two more
  ranges began or ended on a blank line and now hold their code alone: `separator.rs`'s
  `taffy.rs` range (`:534-538`, as the egui and iced showcases cite it) and `SC/main.rs`'s
  `test/platform.rs` range (`:131-137`).
- **Task 3b, a citation of gpui-pre 0.3.6.** The sub-pixel line box reason (five copies in
  `docs/showcase-exceptions.toml`) cited "gpui-pre 0.3.6 src/taffy.rs:412-414" for a
  present-tense claim; it now cites 0.3.10 `:423-425`, where the measured size is still
  rounded up. Every copy took the same text, so the parity check's merge groups keep their
  members. The iced showcase's `PANEL_MIN_SIZE` (gpui-base 0.7.0 `:14`) now cites 0.8.1 `:16`.
- **Task 3b, left to Task 5.** `CN/src/geometry.rs`'s `toolbar` doc still says the builder
  replaces "the `h_8`" a Small `Toolbar` sets (`toolbar.rs:252-256`); gpui-component 0.8.1
  sets `min_h_8` there (`:254`). The numbers hold, and spec §6 rewrites that doc with the fix.
- **Task 3b, what 0.8.1's `Scrollable` adds.** It lays its root out as a flex column
  (`scroll/scrollable.rs:203-207`) and takes the caller's maximum size off the content
  (`:162-170`); upstream's comments say that without these an area held to a maximum height
  did not scroll (on GPUI Fast, in a content-high column inside a row). `SC/support.rs`'s
  `gutter_scroll` doc now says it leaves the flex column out, as it leaves out `ScrollBounce`;
  whether the showcase's scroller needs either was not examined.
- **Task 3b, line counts kept.** Every rewritten doc keeps its file's line count, so the
  citations of the showcases' own lines (the egui showcase's of `SC/inspector.rs` and
  `SC/support.rs`, Widget Info's of `CN/src/geometry.rs`) still hold.
- **Task 3b, set-aside citations that hold.** 146 of the 259 set-aside chains outside
  `SC/info/` cite lines that did not move. As in Task 3, they keep their numbers and their
  unqualified file names, so a later run of the script sets them aside again.
- **Task 3b, the second version grep.** It finds 11 lines, none present-tense of this
  task's: `CN/README.md:29`, `:57`, `:95`, `:144`, `:832`, `:835`, `:847` (Task 8's) and
  `:366` (Task 5's); history at `CN/src/widgets/scene_read.rs:9` ("since 0.1.10"),
  `SC/tests.rs:7644` since 26186e33 (the Folder TabBar 0.8.0 added) and `docs/showcase-exceptions.toml:22`
  (the re-read dates).
- **Task 7, line numbers read again.** Spec §8's repo lines are of `8086a20e`: the mono-font
  line is now `CN/src/config.rs:62` (§8 wrote `:65`) and `DialogTheme`'s attributes and
  header `native-theme/src/model/widgets/mod.rs:811-817` (§8 wrote `:810-815`); the todo
  entries cite these, dated. Upstream's default placement is `dialog/dialog.rs:669-674`, the
  `if`/`else` that picks the anchor, which the todo entry cites; the Dialog's Widget Info row
  cites `:662-675`, wider than §8's `:667-673`, because it also states upstream's comment on
  an overflowing dialog (`:662-666`) and the 16px offset per stacked dialog (`:668`, `:675`).
  Inline code's fallback to the mono font, which §8 states, holds through gpui-component's
  projection of `mono_font_family` into gpui-base's tokens (`theme/mod.rs:595`) and
  gpui-base `text/node.rs:2760-2766`; the entry cites both.
- **Task 7, the entries' places and two tenses.** The Dialog entry is under Core API (a model
  change), the iced and egui gate and gpui-fast's pin under Toolkit Connectors, the Markdown
  entry under Post-1.0, each at its section's end. Task 4 has not landed, so the iced and
  egui entry names the gpui connector's `clippy.toml` as G4's plan's Task 4, not as a file;
  the taffy entry says G4 measures 0.15.1 once in Task 6 (the controller's ruling), for
  Task 6 to append its result as a dated line.
- **Task 4, the inventory's count.** The grep finds the spec's 279 lines (172 doc comments,
  107 plain). gpui-pre-shared-string 0.3.10 has no `src/`: its one file,
  `gpui_shared_string.rs`, sits at the crate root and holds no panic line.
- **Task 4, an entry the grep alone does not find.** `WindowTextSystem::shape_line_by_hash`'s
  doc has no "panic": it says "Like [`Self::shape_line`], this must be used only for
  single-line text" (gpui-pre `src/text_system.rs:721`, gpui-fast `:718`), and its body
  holds `shape_line`'s `debug_assert!` on a newline (gpui-pre `:760-763`). It is listed
  under `shape_line`'s doc line (`:668-670`), which it refers to, and the planted count
  covers it: 12 entries, 19 after the fix rounds below.
- **Task 4, a panic no gpui function documents.** gpui-base and gpui-component comment that
  the platform text system panics on a run that splits a multi-byte character (gpui-base
  `src/input/base/element.rs:373`, `:3666`; gpui-component
  `src/highlighter/highlighter.rs:158`, `:1205`, `:1434`). Those lines document private
  helpers that snap ranges to char boundaries; no doc line of gpui-pre or gpui-fast states
  the panic. The review traced it: `WindowTextSystem::shape_text` and `layout_line` (gpui-pre
  `src/text_system.rs:783`, `:919`; gpui-fast `:780`, `:916`), like `shape_line` and
  `shape_line_by_hash`, hand their runs' lengths to the platform text system's
  `layout_line`, and on Linux `compute_run_spans` slices the text by them
  (`&text[run_offset..run_end]`, gpui-pre-wgpu 0.3.10 and gpui-fast-wgpu 0.1.12
  `src/cosmic_text_system.rs:1074`), in the branch for a run whose font has fallbacks. A run
  that splits a character panics there, and so does one past the text, which `shape_text`
  clips to each line and the other three do not. The `&text[range.clone()]` at `:666`
  slices at paragraph separators found by `char_indices`, so runs do not reach it. The
  fix round lists `shape_text` and `layout_line` (14 entries), and the reasons of
  `shape_line` and `shape_line_by_hash` add this path. A second ruling adds
  `layout_line_by_hash` (gpui-pre `src/text_system.rs:1061`, gpui-fast `:1058`), which on a
  cache miss reaches the same slice (gpui-pre `text_system/line_layout.rs:851-853`;
  gpui-fast `:848`, then `src/fast/text.rs:798`): 15 entries. `try_layout_line_by_hash`
  (gpui-pre `src/text_system.rs:1000`, gpui-fast `:997`) only probes the cache
  (`text_system/line_layout.rs:730-777`, gpui-fast `:727-774`), returning a cached layout or
  `None`, and never reaches the text system, so it is not listed. A third ruling adds four
  and stops the search there: `PlatformTextSystem::layout_line` (the trait method, gpui-pre
  `src/platform.rs:1552`, gpui-fast `:1570`; the Linux implementation is the slicing one,
  `cosmic_text_system.rs:273`), and `StyledText::with_highlights` (its char-boundary checks
  are `debug_assert!`s, gpui-pre `src/elements/text.rs:445-446`, gpui-fast `:446-447`),
  `with_runs` (`panic!` and `assert!` on runs that do not fit the text, `:526-536`,
  gpui-fast `:527-537`) and `with_default_highlights` (through `with_runs`, `:418-429`,
  gpui-fast `:419-430`): 19 entries. Reading function bodies is a wider inventory than P6's;
  `CN/clippy.toml`'s header and a `docs/todo.md` entry, *gpui: the panic gate's sweep reads
  function bodies too*, give it to the next upgrade's sweep. The macOS and Windows text
  systems were not examined.
- **Task 4, no `allow-invalid`.** Every entry drew its error at its planted line on gpui-pre,
  GPUI Fast and without default features; none is in a crate that a Linux configuration
  lacks.
- **Task 4, citations of the moved lines.** The allows add lines to `SC/demo.rs` (8 since 9c325090, before
  `segmented_width`'s closure), `SC/inspector.rs` (5, in `DrawnFamilies::of`) and
  `SC/main.rs` (2 after the module doc, 1 before `mod tests`). The egui and iced
  showcases' citations of the lines after them are renumbered (34 numbers in 18
  citations, each checked to name the line it named before). `CN/src/lib.rs` keeps its
  line count: the deny takes the blank line after `expect_used`. `docs/todo.md`'s
  citations of `SC/inspector.rs:562-565` and `SC/demo.rs:6822`, `:6892` already named
  other lines before this task and are left as they are (the file is append-only).
- **Task 4 fix round, the showcases' citations of the gpui showcase.** The review found that
  most of the egui and iced showcases' citations of the gpui showcase's lines were already
  wrong before Task 4 renumbered them: re-pointed by content on 2026-10-23 and 2026-10-24,
  they drifted with the M9x fixes. At b847149a, 26 of the 50 named other lines (24 by 83 to 476 lines). The
  fix round re-points all 50 by content: each cited block, read at the commit that last wrote
  the citation, is found at HEAD; four blocks that occur twice at HEAD (an icon-or-label
  arm and three `.tooltip` lines) and the two functions that changed since (`panel_toggle`,
  `icon_size_cell`) were placed by their enclosing function. A `docs/todo.md` entry, *Showcases: a gate for one showcase's citations
  of another's lines*, asks for a gate.
- **Task 6, the Separator.** On both cores the test finds one quad inside the labelled
  Separator filled with `tokens.background` and narrower than it, the label's box, centred on
  the Separator: in the test window, at scale 2, the Separator spans x 311–1270 and the label
  745–836, both centred at 790.5. The entry closed and moved to the archive; rationale §1.8
  stands as written.
- **Task 6, both cores' layouts.** `--dump-layout` from release builds of the default core
  and of `--features gpui-fast`, for kde-breeze and adwaita, light and dark, on the Basic page
  (spec §7's four) and, as G3 did, on the Layout page: all eight pairs are byte-identical
  (Basic 175 elements, Layout 39, the Layout page's dump the chrome alone). G3's one
  difference between the cores, the labelled Separator's label, was seen in its captures, not
  in its dumps, which record the Separator's wrapper (rationale P11 (c)); the Separator test
  shows it gone.
- **Task 6, taffy 0.15.1, measured once (fix round 1).** The first attempt, spec §7's
  step 1 (`gpui-fast = []` in `CN/Cargo.toml`), left gpui-fast in Cargo.lock: `cargo
  tree -i gpui-fast` printed nothing, but the lock keeps an optional dependency that
  an enabled feature names weakly, and `gpui-fast?/test-support` sits in the
  `test-support` features of gpui-kit 0.8.1 (`Cargo.toml:81-82`, beside
  `gpui-fast-platform?/test-support`), gpui-base 0.8.1 (`:48`) and gpui-component
  0.8.1 (`:75`), all turned on through the connector's dev-dependency; `cargo update
  -p taffy` locked nothing and `--precise 0.15.1` failed on gpui-fast 0.1.12's `taffy
  = "=0.15.0"`. By the controller's ruling the measurement then deviated from spec
  §7's steps 1 and 4: it left `CN/Cargo.toml` alone and patched the root Cargo.toml
  instead, with `[patch.crates-io] gpui-fast` pointing at a copy of gpui-fast 0.1.12
  in the session's scratchpad whose taffy requirement read `"0.15.0"`, and it restored
  the root Cargo.toml and Cargo.lock afterwards (`git checkout --`) and deleted the
  copy. `cargo update -p taffy --precise 0.15.1` then took 0.15.1, and `cargo tree -p
  native-theme-gpui -i taffy` showed gpui-pre 0.3.10 on it. The gpui-pre suite passed
  with nothing skipped (387; 331 with one ignored; 18; three ignored),
  `the_fallback_stack_copy_is_gpuis` included, since the metadata still holds
  gpui-fast 0.1.12. The eight gpui-pre dumps (kde-breeze and adwaita, light and dark,
  Basic and Layout) from the 0.15.1 build are byte-identical to the 0.15.0 build's: no
  difference.
- **Task 6, parity.** The Linux parity run (the three screenshot scripts with a dump
  directory, then `check_showcase_parity.py`): 887 differences, 887 excepted, 0 not. Six keys
  matched no difference, and the three kinds' dumps agree there, so they were dropped in their
  own commit: material-3's status bar y and height and its toggle's x and y, and
  catppuccin-mocha's toggle x and y. M9x's run found no rest-run key matching nothing; which
  change since made these six agree was not traced. The other 34 keys the check lists as
  matching nothing are of elements drawn only in the hover and menu runs, which the release
  run does not make. The three asset directories were restored.
- **Task 6, 0.8.1's `Scrollable`.** Nothing the dumps or the parity run read moved. The
  showcase's page scroller is its own (`support::gutter_scroll`, without the flex column);
  gpui-component's `Scrollable` draws the Layout page's Scrollable Area (`demo::scroll_area`),
  which neither the Layout page's dump nor the parity run reaches; the showcase tests that
  scroll it pass on both cores.
- **Task 6, the upstream entries.** `lookup.py` (beside the copied script) reproduces ten
  numbers Task 2's run moved, both bounds of each range (14 lines, none failed). Besides the
  Separator entry, 25 open `[upstream]` gpui entries cite a file 0.8.1, gpui-pre 0.3.10 or
  gpui-fast 0.1.12 changed outside its tests (test-only changes read from the diffs: hunks
  inside the file's test module); none is resolved, so each got a dated line, as did the
  `unicode-properties` entry and gpui-fast's taffy pin, which wait for a release. The
  entries citing only unchanged files (gpui-component's `combobox.rs`, `color_picker.rs`,
  `menu/popup_menu.rs`, `status_bar.rs`, `tab/tab.rs`, `tree.rs`, gpui-base's
  `a11y_disabled.rs`, `questionnaire/state.rs`, among others) got none. Some cited numbers already named other
  lines at the old version, and their dated lines say so: `window_border.rs:152-166`, the
  wheel entry's `popover.rs:282` and `dialog/dialog.rs:572`, and the Textarea entry's "a
  `Textarea` offers no size" (it has had a `Size` since at least 0.8.0). One lookup result
  was not taken: gpui-fast 0.1.10's `src/fast/text.rs:1029-1058`, a function 0.1.12 split in
  two, maps by rule (ii) to `:1125-1190`, where the function now ends at `:1155`; the dated
  line cites what 0.1.12 holds, read from the diff.
- **Task 6, the parity script's R-snap rule.** Its docstring cited gpui-pre 0.3.6; each line
  was found at 0.3.10 by its content: the snapping design `src/taffy.rs:281-356` (0.3.6
  `:270-345`), the edge rounding `:382-385` (`:371-374`), the measured text size rounded up
  `:423-425` (`:412-414`, where Task 3b found it), the glyph quantising
  `src/window.rs:4853-4858` (`:4661-4666`); `src/util.rs:128-132` and
  `src/text_system.rs:52` did not move.
- **Task 6 fix round 1, two more parity-file corrections.** `check_showcase_parity.py`'s
  R-glyph note cited `glyph_dilation_for_color` at gpui-pre 0.3.7 `src/window.rs:4665`; it is
  0.3.10 `:4865`, the same line, and no other gpui citation in the script names an older
  version. The flat-button group's reason in `docs/showcase-exceptions.toml` named material-3
  beside catppuccin-mocha after the group lost its material-3 keys; it names catppuccin-mocha
  alone now. The text occurs in that group only, so no merge group splits.
- **Task 8, the documents.** The README's icon audit sentence names 0.8.1's list beside
  0.8.0's on Task 3b's measurement (`default-icons.txt` 106 lines, identical to 0.8.0's); its
  `h_resizable` row names 0.8.1 on Task 6's (`resizable.rs` unchanged). The CHANGELOG's
  other `[Unreleased]` lines that name gpui-component or GPUI Kit 0.8.0 (the Slider's
  `accessibility_label`, the Folder tab bar, the two charts, the nested scroll areas, the
  base-palette tokens, `geometry::list`) say what 0.8.0 added or did and are left as they
  are: spec §9 rewrites only the floors' entry. The screenshot entry's dated line notes that
  every capture path passes `--tab basic`, so neither the Layout page's Separator nor the
  Inputs page's ColorPicker is in a screenshot.
- **Task 8, the stamps and the full run.** `scripts/update_compatibility.sh run` locked no
  newer release for any family: gpui at GPUI Kit 0.8.1, gpui-pre 0.3.10 and the gpui-fast
  crates 0.1.12, iced and egui at their previous sets, all dated 2026-10-27. The full
  `./scripts/check_release.sh` after them: 170 checks, 161 passed, 8 warnings (the `v0.6.1`
  tag; the six crates' semver-checks, the approved 0.7.0 breaks, the gpui crate's none from
  G4; `cargo outdated` failing on zbus, as recorded since M8), 1 failure (the visual assets,
  regenerated at the release). The packaging check with `--features gpui-fast` passed.
- **Task 8, the final review's citation fixes.** Task 3b re-cited `native-theme/src`'s two
  gpui-pre-linux citations at 0.3.10 (spec §4.3), but `src/lib.rs`'s `text_scaling_factor`
  doc kept 0.3.9's `client.rs:2620-2666` for the `GPUI_X11_SCALE_FACTOR` block under the
  0.3.10 citation; it now cites 0.3.10's `:2623-2669` (from `let env_dpi` to the match's
  `DpiMode::NotSet` arm), as `src/gnome/mod.rs` already did. The text-metrics reason (five
  copies in `docs/showcase-exceptions.toml`) cited gpui-pre `src/elements/text.rs:765` for
  the width rounded up; the `.ceil()` is at 0.3.10 `:772`, which `SC/demo.rs` cites. Every
  copy took the same text, and the parity check's merged groups (84 over 582 keys) and their
  reasons' order are unchanged.
