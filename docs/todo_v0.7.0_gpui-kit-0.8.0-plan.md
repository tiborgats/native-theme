# v0.7.0 — GPUI Kit 0.8.0 (G3): Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development
> (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use
> checkbox (`- [ ]`) syntax for tracking.

**Goal:** native-theme-gpui and its showcase on GPUI Kit 0.8.0 / gpui-fast 0.1.10, every
upstream citation true for 0.8.0, the GPUI Fast test workarounds measured and removed where
unneeded, the new Folder tab bar shown, the slider as accessible as upstream's; M9a's red
CI job green; and, as the maintainer decided D1, `windows-11`'s tabs in WinUI 3's folder
shape, from a new theme value that all three connectors draw.

**Architecture:** gpui connector and its showcase (plus the comment in
`tools/showcase-support/src/capture.rs` and the documents), and for D1 the core crate's
model and data files, the iced and egui connectors and all three showcases (Tasks 11–15).
The bump, the renumbering and the hand-read form one unit (Tasks 1–3); the rest are
independent showcase or connector tasks; documents and the compatibility stamp close.
Order: 0, 1–3, 3b, then 4–8 and 11–15 as their dependencies allow (11 before 12–14, those
before 15; 12 after 6), then 9, then 10.

**Tech stack:** Rust 2024; the gpui connector's MSRV 1.95.0 (re-measured in Task 1);
gpui-component / gpui-base / gpui-kit 0.8.0, gpui-pre 0.3.9, gpui-fast 0.1.10.

**Spec:** [`todo_v0.7.0_gpui-kit-0.8.0-spec.md`](todo_v0.7.0_gpui-kit-0.8.0-spec.md); reasons:
[`todo_v0.7.0_gpui-kit-0.8.0-rationale.md`](todo_v0.7.0_gpui-kit-0.8.0-rationale.md).

**When:** on `v0.7.0-rc1`, as the one writer between M9b's tasks (rationale K13), from the
maintainer's approval of these documents. An Opus session orchestrates; code tasks run in
`implement` agents (Opus); a fresh reviewer per task. Needs the maintainer: the documents,
the push after Task 3 and the one in Task 10, and the go for Task 9's desktop run (D1 is
decided: rationale K14).

## Global Constraints

- No `unwrap` / `expect` / `panic!` / `unreachable!` / slice indexing / unchecked
  arithmetic outside tests; no `unsafe`; the strict panic lints on the library and on the
  showcase example, on both cores.
- NEVER invent a value: a number in a claim, comment or document is read from the cited
  source or measured.
- Every gpui change is tested on both cores (`--features native,watch` and `--features
  gpui-fast,native,watch`).
- Commits: explicit paths (`git commit -- <paths>`, never `-a` or `add -A`); author the
  maintainer; no AI attribution trailer; each message ends "(G3 Tn)". Built in the project's
  `target/`; no other target directory than `check_release.sh`'s own
  `target/package-verify`.
- `docs/todo.md`, the ledger and the review plan: re-read before editing; append, never
  rewrite; a closed todo entry moves to `docs/archive/todo_closed.md` with its closing
  commit; entries are cited by heading and opening words.
- **Gates per task:** `./scripts/check_release.sh --commit` green on the task's last commit,
  and, since `--commit` skips the GPUI Fast tests (`check_release.sh:678-680`), for Tasks 1,
  3b–8, 11, 12 and 15 also `cargo test -p native-theme-gpui --features gpui-fast,native,watch
  --all-targets` (the iced connector's `iced_aw,watch,native` leg, which holds
  `contract-leaves.toml`, is in `--commit`, `check_release.sh:705`).
  One exception, stated in advance: Tasks 1–3 are one unit — Task 1's commit leaves the two
  showcase citation gates red, Task 2's leaves them red only for the hand-read set, Task 3's
  makes the branch green. Every other commit is green. The work no gate reads (spec §3.3)
  is Task 3b, after the unit, so the unit is as short as it can be.
- **Files other milestones write.** A task that edits a file M9b or M9c also writes (the iced
  connector and showcase, the egui connector and showcase, `docs/showcase-elements.toml`,
  `scripts/connector_parity_exceptions.toml`, `contract-leaves.toml`) locates every region by
  content, never by the spec's line numbers; at its start, before writing, it re-reads the code
  the spec describes there and checks that the description still holds (the roles,
  callers, layout paths and tests it names); where it does not, it stops and reports the
  difference to the orchestrating session, which re-specifies the step. The spec names
  such code by its role as well as its present name, so a rename or a move that keeps the
  role and the behaviour is no stop (it is followed, and noted in the task's report); a
  changed behaviour is. (M9b T5 changed how
  the iced Basic tabs are built after these documents were first written, `cffb6355`.)
- Pushes only with the maintainer's approval; no upstream issue is filed by an agent.
- Outside the project: reading the cargo home is granted; cargo's own writes there (the lock
  update, `cargo audit`'s advisory fetch) were permitted by the maintainer on 2026-10-20;
  nothing else outside the project is written (a missing toolchain is installed only with
  the maintainer's permission). `~/.claude/scripts/sweep-target.sh` runs whenever
  `target/` passes about 200 GB, and at the close.

## Interleaving with M9b

- G3 starts a task only on a clean tree at an M9b task boundary (M9b's last task committed
  and reviewed, `git status` clean), and ends each task with the tree clean. The
  orchestrating session decides who writes next; it never runs two writers.
- **Tasks 1–3 run back to back**, with no M9b commit between them: while the unit is open,
  M9b's per-task gate would fail on the gpui citation gates. Tasks 0, 3b–15 are each a
  boundary at which M9b may take a turn.
- **D1's tasks (11–15)** run the same way, each between M9b's tasks on a clean tree (rationale
  K16). Task 13 writes M9b's files (`connectors/native-theme-iced/src/lib.rs`, its
  `src/contract/rows.rs`, `src/styles/aw.rs`, `contract-leaves.toml` and
  `examples/showcase-iced.rs`); it starts only at a boundary where none of M9b's T5 (its
  review and fix rounds), T8 (the iced `basic_column_count`), T10 (the Extra page's
  `TabBar`) and T19 (the page list `page_tabs` draws, the shared list's header) is open:
  each done or not begun, nor an M9b fix round carrying tab items (T5's re-review carried
  two to "T6's fix round if any, else Step 8's final fix"). Task 11 moves one digit run
  in an M9b file, a model citation in a comment of the iced showcase. Task 14 writes the egui showcase, M9c's, which has not
  started; should M9c be running by then, Task 14 takes an M9c task boundary where M9c's
  T7 (the egui `column_count`) is done or not begun (M9c's briefs locate by content).
  Task 15 edits `docs/showcase-elements.toml`, whose header M9b's T19 also edits, and
  starts only where T19 is done or not begun.
- Shared files (both milestones write them): `Cargo.lock` (Task 1 updates it from M9b's last
  lock commit; an M9b lock change after Task 1 is made with `cargo update -p <crate>`, never
  a fresh resolve), `.cargo/audit.toml` (M9b's lock work added entries; G3 Task 1 edits its
  comments), `CHANGELOG.md`, `docs/showcase-exceptions.toml` (Tasks 2 and 3b edit it; 3b
  finds each set-aside entry by its recorded text), `docs/todo.md`, the ledger,
  `.github/workflows/ci.yml`,
  `tools/showcase-support/` (M9b's T2 moved iced's capture there, and its fix round still
  edits the crate; G3 changes only the comment at `capture.rs:39-44`, in Task 3b); for D1
  also the iced connector and showcase (M9b's), the egui showcase (M9c's),
  `docs/showcase-elements.toml`, `scripts/connector_parity_exceptions.toml` and the
  presets. Each writer re-reads them at its start.
- M9a's open item closes here: the GPUI Fast step of CI run 39490500851 failed on the stale
  child-list crash. Task 1 lands the floor that fixes it and the regression test; the push
  after Task 3 is the run that should be green; the ledger's M9a Step 8 then closes. The
  controller's M9a gpui-fast fix brief (`m9a-gpuifast-fix-brief.md`, in the review's SDD
  directory) is superseded (rationale K4): its regression test is adopted, its workaround,
  `[upstream]` entry and CHANGELOG line are not. The SDD progress file records that.

## Review Focus

1. **The crash cannot come back**: no build of the connector can resolve gpui-fast below
   0.1.10 (the floor), and the regression test fails on 0.1.7 and passes on 0.1.10 on both
   cores (Task 1).
2. **No vacuous paint assertion on GPUI Fast**: every test read of painted quads or
   underlines that finds or excludes a primitive sees clip-group content; the mutation
   check shows the descent is needed (Task 1).
3. **Claims say what 0.8.0 paints**: the behaviour-changed panels re-read, Button's hover
   registration among them (Task 3).
4. **An application that turns on `gpui-kit/gpui-fast` alone** still gets a connector that
   builds (the form-A check, Task 1).
5. **Each removed occlusion** has a passing wheel test at its site on both cores; each kept
   one says why in 0.8.0 terms (Task 5).
6. **A themed slider** announces its label and its disabled state as gpui-component's does
   (Task 7).
7. **No cited number skipped**: comma items and continued `:n`s moved with their chain or
   set aside whole; the whole check's wider scan finds none unhandled; every repeated line's
   copy agrees with git's histogram map; no number on Task 1's lines was re-mapped, and each
   was read at 0.8.0 (Task 2).
8. **Folder's panel**: every colour claim at its 0.8.0 line; every literal noted, none
   claimed as theme data (Task 6).
9. **D1 is true and drawn alike**: the facts row's every value cited at the MUX pin, none
   invented; the outline one line from the side edge into the curve, run to the bar's edge
   (our design, spec §12.2); the same §12.2 numbers held by each connector's own test; the flares painted
   after every tab, inside the row's clip (iced: cut by the draw's viewport), on both gpui
   cores and across a second frame drawn without `refresh()` (Tasks 11–14); `windows-11`'s
   selected fill WinUI's own, with its citation (Task 15).
10. **No connector or showcase tab bar ignores the value once a preset states it** (an iced
    application wraps its own bar in `tab_flares`): `windows-11` states it only
    in Task 15, after the three receivers, and the shared list names it only then (Task 15);
    every candidate of Task 11's windows listed and decided, the reviewer's re-run of the
    facts enumeration and of the inheritance and model scans matching the lists (Task 11).

---

## Task 0: Register G3 (at approval)

- [ ] The three documents to `docs/todo_v0.7.0_gpui-kit-0.8.0-{rationale,spec,plan}.md`.
- [ ] `docs/todo_code-review-plan.md` (re-read first): a G3 row in the milestone table; a
  decision line D11 pointing at these documents, stating that G3 runs between M9b's tasks
  as one writer and closes M9a's red CI, and that the maintainer decided D1 "Yes, in G3"
  (Tasks 11–15); the *Resuming* rule names G3.
- [ ] `docs/todo_code-review-findings.md` (re-read first): a *G3* status row and section.
- [ ] The inputs the documents cite by name are in
  `.superpowers/sdd/todo_code-review-plan/g3/` (on the project disk; untracked, as that
  directory is): `gk080-whats-new.md`, `gk080-trial-build.md`,
  `gpui-fast-panic-diagnosis.md`, `g3-check1.md` … `g3-check11.md` (and any later pass),
  the checkers' scan scripts `c10/`, `c11/`, `c12_enum.py` / `.out` and `c13/` (and any
  later pass's), and `g3-d1-revision-notes.md`. The session that drafted the documents copies them there from
  its own scratchpad when it hands the documents over (no later session can reach that
  scratchpad); Task 0 checks they are there and, if one is missing, says so in the ledger.
  Nothing a task needs is only there (the regression test is spec Appendix A).
- **Commit:** `docs(review): GPUI Kit 0.8.0 as milestone G3, between M9b's tasks (G3 T0)`.

## Task 1: Floors, lockfile, MSRV and audit; the GPUI Fast test gates

Files: `CN/Cargo.toml`, `Cargo.lock`, `.cargo/audit.toml` (its comments), `CN/README.md`
(floors, Quick start), `SC/tests.rs`, `CN/src/widgets/tests.rs`, `docs/todo.md`.
Depends on: Task 0; an M9b boundary.

- [ ] Spec §2.1's red run first: the regression test (spec Appendix A) added, run on the current lock
  (gpui-fast 0.1.7); it panics (output kept for the commit body).
- [ ] Spec §1: the four floors and the comments, the scoped `cargo update`, the lock diff
  gate, `cargo tree` before and after, the MSRV lines (1.95.0 passes all three, 1.94.0
  fails), `cargo audit`, the README floors, the compile gate and the form-A check.
- [ ] Spec §2.2: `scene_quads` / `scene_underlines` in both test modules, every read moved
  onto them, the switch's order read hardened; the progress-bar test passes on GPUI Fast;
  the mutation check recorded.
- [ ] Every suite on both cores: `cargo test -p native-theme-gpui --all-targets` with
  `--features native,watch` and with `--features gpui-fast,native,watch` (the trial ran
  `--lib` only; the showcase and seam tests run here for the first time on 0.8.0), and both
  cores' showcase binaries in the stripped environment (spec §2.1). Expected red: the two
  showcase citation gates only. Anything else is fixed in this task (a behaviour change of
  0.8.0 a test exposes, rationale §1.2) or, if it is a claim, left to Task 3 and listed.
- [ ] The `[upstream]` todo entry of spec §2.2.
- [ ] Every upstream line number this task writes (the regression test, `scene_quads`, the
  manifest comments, any fix) is read at 0.8.0 / 0.1.10 by the task's review. Task 2 leaves
  every line of `git diff <base> <T1-last>` alone (its base the M9b boundary commit this
  task started from, fix rounds included) and lists each citation it skips there for its
  own review (spec §3.1, *Not Task 1's lines*).
- **Commit:** `build(gpui): GPUI Kit 0.8.0 on gpui-fast 0.1.10, which fixes the stale child-list crash the regression test now holds; tests read the painted scene's clip groups (G3 T1)`.
  Leaves red: the two showcase citation gates (Tasks 2–3).

## Task 2: Renumber the moved citations

Files: the citations of spec §3.1's scope. Depends on: Task 1, immediately.

- [ ] Spec §3.1: the script — every form (cited `file:n`, the number wrapped to the next
  line, comma lists, bare `:n` continuations, ranges; a chain whole or set aside), files by
  path suffix, crates among every crate with the file (gpui-pre and unchanged files
  included, the nearest named crate winning, foreign crates out of scope, a leading crate
  directory stripped), Task 1's lines (`git diff <base> <T1-last>`) left alone and their
  citations listed, a chain crossing them set aside, the line rule (unique text, else a
  difflib equal block of at least three lines that git's histogram map confirms, else set
  aside; no "stays" shortcut) — its one run, the four gates (the whole check by a
  separate, wider scan among them); counts by form and by rule
  in the body; the review reads every listed skipped citation at 0.8.0 / 0.1.10; the set-aside list, with
  each entry's reason and texts, in full in the commit body, which Tasks 3 and 3b read with
  `git show` (a scratchpad copy a convenience only); a fix round that changes the list
  restates it whole in its body, and the last Task 2 commit that states it is the one read. The script is not run again (spec §3.3).
- **Commit:** `docs(gpui): the citations that only moved between GPUI Kit 0.7.3 and 0.8.0 and gpui-fast 0.1.7 and 0.1.10 are renumbered (G3 T2)`.

## Task 3: Read and rewrite what the gates read; the unit closes

Files: `SC/info/` (as spec §3.2 names). Depends on: Task 2, immediately.

- [ ] Spec §3.2: the hand-read set the gates read (the button citations into the hover
  block `:724-741` among them; the +6 below it is Task 2's), the behaviour-changed panels.
- [ ] Gate: both showcase citation gates pass; `scripts/check_release.sh --commit` green
  (the unit is closed).
- **Commit:** `docs(gpui): the claims and prose GPUI Kit 0.8.0 changed are rewritten to what its code does (G3 T3)`.
- [ ] **Maintainer:** approve the push. Push `v0.7.0-rc1`; CI green, the GPUI Fast step of
  *Test gpui connector* included. A failure is fixed in G3 before Task 3b. Then the ledger's
  M9a Step 8 closing line and the SDD note (Interleaving). M9b may take a turn.

## Task 3b: The citations no gate reads, and the version-stating prose

Files: as spec §3.3 names (`CN/src`, `CN/tests`, `CN/README.md`,
`CN/examples/showcase-gpui` (its ungated comments, and the version prose in `SC/info/`),
`docs/showcase-exceptions.toml`, `tools/showcase-support/src/capture.rs`,
`scripts/connector_parity_exceptions.toml`, `docs/todo.md`). Depends on: Task 3 and its push.

- [ ] Spec §3.3, from Task 2's set-aside list as the last Task 2 commit stating it holds it, by hand (no
  second script run), each entry found by its recorded text: the citations outside `SC/info/` and the ungated comments
  inside it (a repeated reason text in `docs/showcase-exceptions.toml` rewritten alike in
  every group), the gpui-fast citations, the connector's untested citations,
  `scripts/connector_parity_exceptions.toml:461`, the `docs/todo.md` line for M9c naming the
  egui showcase's stale `GC/` citations, the version-stating prose, the re-measured counts;
  its gates.
- **Commit:** `docs(gpui): the connector's own citations and version prose are true for GPUI Kit 0.8.0 (G3 T3b)`.

## Task 4: The `window.refresh()` workaround

Files: `SC/tests.rs`, `CN/src/widgets/tests.rs`, `docs/todo.md`, `docs/archive/todo_closed.md`.
Depends on: Task 3b.

- [ ] Spec §2.3: the five tests and both suites on both cores without `refresh()`; the
  result applied as the spec says; the todo entry closed or dated.
- **Commit:** `test(gpui): the draw helpers no longer force a full frame, since gpui-fast 0.1.10 drops the bounds of elements it no longer draws (G3 T4)`
  (or, if any test needs it, `test(gpui): the draw helpers' full frame is kept for <cause> (G3 T4)`).

## Task 5: Nested scrollers

Files: `SC/tests.rs`, `SC/demo.rs`, `SC/pages/typography.rs`, `SC/pages/layout.rs`,
`docs/todo.md`. Depends on: Task 4 (the draw helper settled first, so a failure has one
cause).

- [ ] Spec §4: the extended test, passing with every occlusion; then each site alone; the
  results in the body; the doc comment; the todo line.
- **Commit:** `fix(showcase-gpui): the scrollers GPUI Kit 0.8.0 keeps the wheel for no longer occlude what lies behind them (G3 T5)`.

## Task 6: `TabVariant::Folder` in the showcase

Files: `SC/demo.rs`, `SC/pages/layout.rs`, `SC/info/layout.rs`, `SC/tests.rs`,
`docs/todo.md`. Depends on: Task 3b (and Task 1's `scene_quads`).

- [ ] Spec §5, the tests first (they fail: no sample).
- [ ] Gate: the colour-claim gate passes with the new panel; coverage `missing 0`.
- [ ] Added at G3 Task 0 (M9-082's gpui half, assigned here as the gpui showcase's writer): `SC/inspector.rs`'s frame row (`:301`, the `Decorations::Server` arm) names the frame per platform, as the iced showcase's `window_frame(std::env::consts::OS)` does since `9a0c356b` (M9b T8), where it prints the KWin sentence on every platform; a test on the row's text per platform, failing first; in its own commit, with the ledger's M9-082 status set to fixed.
- **Commit:** `feat(showcase-gpui): the Layout page shows TabBar's Folder variant, with its theme colours claimed and its literals noted (G3 T6)`.

## Task 7: The slider's accessibility

Files: `CN/src/widgets/slider.rs`, `CN/src/widgets/mod.rs`, `CN/src/widgets/tests.rs`,
`SC/demo.rs`, `SC/pages/basic.rs`, `SC/pages/inputs.rs`, `docs/todo.md`. Depends on:
Task 3b.

- [ ] Spec §6: the `themed_root` seam and `a11y_node_of` first, then the tests (they fail:
  no setter, no disabled flag), then the setter and the wrapper; the fallback case through
  gpui-kit's `TestWindowExt::try_find`.
- **Commit:** `feat(gpui): widgets::Slider announces an accessibility label and its disabled state, as gpui-component 0.8.0's does; the showcase names its sliders and number inputs (G3 T7)`.

## Task 8: The side panel on a window resize

Files: `SC/tests.rs`, `SC/app.rs`. Depends on: Task 3b.

- [ ] Spec §7, the test first; `.fixed()` only if it fails.
- **Commit:** `fix(showcase-gpui): the side panel keeps its width when the window resizes, as the iced and egui showcases' do (G3 T8)` (or `test(showcase-gpui): …` if no code changed).

## Task 11 (D1): the selected tab's outward corners in the model

Files: `native-theme/src/model/widgets/mod.rs`, `docs/property-registry.toml`,
`docs/inheritance-rules.toml`, `docs/platform-facts.md`, the citations Task 11's
renumbering moves (spec Appendix B: the six native presets, `documented_sizes.rs`,
`native-theme/src/kde/metrics.rs`, gpui's `src/geometry.rs`, `docs/inheritance-rules.toml`,
`docs/todo.md`; the model citations in egui's `src/accessors.rs`, `src/atlas.rs`,
`src/contract.rs` and one comment of the iced showcase; `ios-13.toml:19`), the five
name-less comments (`documented_sizes.rs`, `native-theme/tests/platform_facts_xref.rs`),
`connectors/native-theme-egui/mapping.toml`,
`src/mapping.md`, `src/mapping_tests.rs`, `README.md`, `native-theme/README.md`,
`CHANGELOG.md`, the stale model citations' files (egui `src/accessors.rs`, iced
`src/styles.rs`, `docs/todo.md`), `docs/archive/todo_closed.md`. Depends on: Task 3b; an M9b
boundary.

- [ ] Spec §12.1, red first (the registry key and the facts row before the field): the field,
  the registry key, the inheritance entry, the facts row (MUX re-read at the pin), the egui
  mapping row with `mapping.md` regenerated and the count, the two README sentences, the
  CHANGELOG's *Breaking Changes* and *Added* lines, the `[1.0]` `docs/todo.md` entry.
- [ ] The line citations of spec §12.1 (rationale K18 (b)): the row with the blank line
  `:1401` removed, the inheritance entry with the blank `:464` removed; the enumeration
  re-run at the task's base and compared with spec Appendix B (a candidate that differs
  decided by hand); the 123 (or re-derived) facts numbers moved by one, the stale
  `ios-13.toml:19` range re-pointed by content; the model file's citations (found by the
  scan of spec §12.1) moved by the field's length where right, the stale ones re-pointed by
  content, every number of each, those below `:486` too; the entry *Tab: an active-tab
  indicator* closed to the archive (met by `e610750b`); the registry and mapping scans
  showing nothing to move; the five name-less comments rewritten; gates (G1)–(G4); the list with decisions
  in the body.
- **Commit:** `feat(tab): tab.active_outward_corner_radius_px, the selected tab's outward bottom corners: WinUI 3's folder-shaped TabView curves them out by 4 against its 8 top corners, its outline following; no bundled preset states it yet (G3 T11)`.

## Task 12 (D1): gpui — `widgets::TabBar` draws the outward corners

Files: `CN/src/widgets/tab_bar.rs`, `CN/src/widgets/mod.rs` (`part_bounds`' bottom inset),
`CN/src/widgets/tests.rs`, `SC/info/layout.rs`,
`SC/info/leaves.rs`, `SC/pages/basic.rs`, `SC/tests.rs`, `CN/README.md`, `CHANGELOG.md`.
Depends on: Tasks 11 and 6.

- [ ] Spec §12.3, the tests first (they fail: no flare, a bottom border on the selected tab),
  on both cores, with the radius set on a resolved `windows-11`; then `TabLook::outward_radius`,
  `flare_quads`, the bounds-recording canvas and the painting one; the showcase's info, `HOW`
  lines and column rule; `native_tab_row`'s colour claims re-pointed into the edited
  `tab_bar.rs`; the Folder panel's note (spec §5); README and CHANGELOG.
- **Commit:** `feat(gpui): widgets::TabBar draws the selected tab's outward corners where the theme states them, painted after the tabs, its outline following with no bottom edge (G3 T12)`.

## Task 13 (D1): iced — `tab_flares`

Files: `connectors/native-theme-iced/src/lib.rs`, `src/contract/rows.rs`,
`src/styles/aw.rs` (its doc), a new module for the widget's impl, `contract-leaves.toml`,
`examples/showcase-iced.rs` (its tab code, and its two stale `lib.rs:1278-1336`
citations), `README.md`, `connectors/native-theme-egui/src/install_tests.rs`
(one citation), `docs/todo.md` (`:412`'s stale citation into iced `lib.rs`),
`scripts/connector_parity_exceptions.toml`, `CHANGELOG.md`. Depends on: Task 11; an M9b
boundary with none of M9b's T5, T8, T10 and T19 open, nor an M9b fix round carrying tab
items (Interleaving).

- [ ] Spec §12.4, the tests first (the recording renderer's, then the showcase's); then
  `tab_flares` and `tab_flares_look`, the contract rows, `contract-leaves.toml` regenerated
  (no leaf moves the new rows yet), the parity exceptions, the showcase's `tab_row`, the
  default build's Basic tab row (since `99af695b` in `basic_tab_row`) and the iced_aw Basic and
  Extra bars (the room outside the wrapper), column rule and routes, the Basic tabs' pixel
  test in both builds; the egui citation re-pointed by content; README and CHANGELOG.
- **Commit:** `feat(iced): tab_flares draws the selected tab's outward corners over a row of tabs, an iced_aw TabBar or Tabs; the showcase's tab bars use it (G3 T13)`.

## Task 14 (D1): egui — the showcase's tab bar draws the outward corners

Files: `connectors/native-theme-egui/examples/showcase-egui/demo.rs`, `pages/basic.rs`,
`tests.rs`. Depends on: Task 11 (and, should M9c have started, an M9c boundary with its
T7 not open).

- [ ] Spec §12.5, the tests first; then the room at both ends, the strip and the clipped
  circle strokes after the tabs, the reads, the column rule.
- **Commit:** `feat(egui showcase): the tab bars draw the selected tab's outward corners where the theme states them, painted after the tabs (G3 T14)`.

## Task 15 (D1): `windows-11` states it; the shared list names it

Files: `native-theme/src/presets/windows-11.toml`, `documented_sizes.rs`,
`native-theme/src/presets/README.md`, `docs/new-os-version-guide.md`,
`docs/platform-facts.md` (the Windows `active_background` cell), `docs/showcase-elements.toml`,
`connectors/native-theme-iced/contract-leaves.toml`, `connectors/native-theme-iced/src/contract.rs`
(one test's comment), `connectors/native-theme-egui/src/style/variants.rs` (one citation),
`CHANGELOG.md`, `docs/todo.md`, `docs/archive/todo_closed.md`.
Depends on: Tasks 12, 13 and 14; an M9b boundary with M9b's T19, and any M9b task on that
`src/contract.rs` test, not open; should M9c have started, no M9c task open on egui's
`src/style/variants.rs`.

- [ ] Spec §12.6: the radius in both variants; `tab.active_background` corrected to WinUI's
  `#f9f9f9` / `#282828` with its citations, the facts cell with it, the WinUI-brushes todo
  entry closed; the documented-sizes rows and lists and the two gated-size lists; the
  shared list's leaves and its header's column rule; `contract-leaves.toml` regenerated
  (`windows-11` now moves two colour leaves through `tab_flares`' rows); the CHANGELOG
  lines; the three showcases' route tests and the iced colour-route tests pass.
- **Commit:** `feat(windows-11): the selected tab curves out by 4 at its feet and is filled SolidBackgroundFillColorTertiary, WinUI 3's TabView; the three showcases list tab.active_outward_corner_radius_px on their tabs (G3 T15)`.

## Task 9: GPUI Fast at run time

Depends on: Tasks 4–8 and 11–15 (the layout it compares is final). **Maintainer:** the go
for a desktop run.

- [ ] Spec §9: the eight dumps, the four comparisons, the screenshot; a difference fixed (its
  own commit, gated) or explained.
- **Commit:** the ledger's note, `docs(review): the showcase's layout on GPUI Fast 0.1.10 against gpui-pre's (G3 T9)`.

## Task 10: Documents, stamp, gates, close, archive

Depends on: Tasks 0–9, 3b and 11–15.

- [ ] Spec §10: README *Backends*, *What gets mapped*, *Theme-drawn widgets* and
  *Accessibility*, CHANGELOG, ROADMAP, the todo items (the `Window::screenshot()` entry's
  dated line among them, M9b T2's hand-off), the full-theme note; spec §12.7's check that
  D1's documents are all in.
- [ ] The iced and egui compatibility stamps (spec §12.7): `scripts/update_compatibility.sh
  run iced` if M9b has closed, `run egui` if M9c has; otherwise the later close runs them,
  and the ledger's *G3* section says which is left to whom.
- [ ] `scripts/update_compatibility.sh run gpui` (after the connector's last source change;
  it writes `docs/COMPATIBILITY.toml` and the README's Verified line), its own commit.
- [ ] The full `./scripts/check_release.sh` (spec §11's expected items only), and right
  after it the packaging check with `--features gpui-fast` (spec §11).
- [ ] **Maintainer:** approve the push. Push; CI green (spec §11).
- [ ] The three documents to `docs/archive/` with an *As built* note at the top of the spec;
  the archive index row; D11 and the ledger's *G3* links pointed at the archive; the
  ledger's G3 status (done); the memory; `~/.claude/scripts/sweep-target.sh
  /home/tibi/Rust/native-theme`.
- [ ] **Maintainer:** the next `generate_assets_release.sh` run refreshes the screenshots
  (spec §11).
- **Commits:** `docs(gpui): GPUI Kit 0.8.0 in the changelog, README, roadmap and todo list (G3 T10)`;
  `docs(gpui): the compatibility stamp for GPUI Kit 0.8.0 (G3 T10)`;
  `docs: archive the implemented GPUI Kit 0.8.0 design documents (G3 T10)`.
