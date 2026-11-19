# 0.7.2: gpui-component's own icons follow the theme's icon set — implementation plan

- Rationale: [`todo_v0.7.1_gpui-widget-icons-rationale.md`](todo_v0.7.1_gpui-widget-icons-rationale.md)
  (decisions W1 to W11). Specification:
  [`todo_v0.7.2_gpui-widget-icons-spec.md`](todo_v0.7.2_gpui-widget-icons-spec.md)
  ("the spec"), whose Part G this plan carries out.
- Decided by the maintainer on 2026-11-19: this work is part of v0.7.2, after the KDE steps 2
  to 5 on the same branch, not a release of its own (the spec's head; the rationale's W11).
  The version stays 0.7.2.
- Written 2026-11-19. It replaces the plan of 2026-11-14 for a separate release (git history,
  `0a9141b0`, keeps it). Status: approved by the maintainer with the amended rationale and the spec on
  2026-11-19, with §2 to §5's two gates; the defaults of §8 he left to the session, which
  keeps them. The judged themes' packages, read with `pacman -Q` that day: `breeze-icons`
  6.31.0-1.1 (`breeze`, `breeze-dark`), `adwaita-icon-theme` 51.0-1 (`Adwaita`).
- The record: `<record>` = `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan/`,
  0.7.2's own record, with this work's files in `<record>/icons/`. `<tools>` =
  `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan/tools/`.

## 1. Where and from what

- One worktree, `.claude/worktrees/lane2-b`, branch `v0.7.2-rc1`, from its head `a3fc81f6`
  (the KDE steps 4 and 5 built, gated `--commit` 148/148, pushed). The units are commits on
  that branch: no new branch, no merge, no version step. The branch is pushed by explicit
  refspec at each `--commit` gate that passes, and each push's CI run is read.
- The evidence already taken, in `<record>/icons/`:
  - `upstream-reread.md`: every upstream citation of the spec and the rationale re-read at
    gpui-component 0.10.0 and gpui-fast 0.1.16: no decision W1 to W11 breaks; two statements
    were false, both on the loading button (the rationale's W3, the spec's E.4), three notes are new (`Minus`, the right-to-left chevrons,
    the radio's `Check`).
  - `review-partA.md`, `review-partEG.md`: the first-pass reviews, with every repository
    anchor at `a3fc81f6` (the briefs' lookup).
  - `decisions.md`: the rulings on every finding (amendments T1 to T20, R1 to R8, applied to
    the spec and the rationale), E.4's design for the two loading buttons, this plan's
    outline, the maintainer's points.

## 2. Units

Each unit is one commit; a review fix is a commit of its own, gated by the next gate.

| Unit | The spec's parts | Agent | Gate after it |
|---|---|---|---|
| D0 | The documents (G.1): the amended rationale, the spec amended and renamed, this plan, the two live mentions of the old names re-pointed (`docs/todo_v0.7.x_caret-blink-spec.md`, `docs/todo_v0.7.2_kde-hover-outlines-and-toolbar-buttons-spec.md`), with the approval's date written in the rationale's status and this plan's *Status* | orchestrator (commit) | the five documentation checkers |
| U1 | Part A, *The core's new method*; its tests include the full check's step "test (native-theme, Linux features, stem index)" | one `implement` agent for U1, U2 and U3a, in this order | none of its own |
| U2 | C.1's check of `process-working-symbolic` with B.1 uncommitted, then Part B, *The lookup and the one-colour test* | the same agent | none of its own |
| U3a | C.2 to C.4, *The tool*, *The themes*, *The committed table*: the ignored test | the same agent | `--commit` (1 of 2) over U1 to U3a; the push; CI read |
| U3b | The run and C.5, *The seven stop conditions* (§3 below) | orchestrator | the verdict; a stop only where a condition is met |
| U3c | C.6, *What is recorded*; the tool removed | orchestrator (a `worker` drafts the README) | the five documentation checkers; its code change is proven by U4's gate |
| U4 | Part D, *The handle*; F.2's D items (D.13 only on the maintainer's word, condition 7); its tests include D.12's on both cores and the connector's doctests (G.1) | `implement` | none of its own |
| U5 | Part E, *The showcase* | `implement` | none of its own: built beside U6 |
| U6 | Part F, *Documentation*: F.1, F.3, F.4, F.5 | `worker` | after U5's and U6's reports: the steps below, then `--commit` (2 of 2), the two GPUI Fast legs and the five documentation checkers; the push; CI read |

U1, U2 and U3a are one agent's work because each builds on the one before (B calls A's
method, the tool B's lookup) and they share a build target, so they could not run side by
side; one brief and one reading of the code serve all three. The agent stops where §7 says
(C.1 red, B.3's stops) and leaves the three units' edits uncommitted; their files are
disjoint (U1 `NT/`, U2 `CN/src/` and `scripts/connector_parity_exceptions.toml`, U3a
`SC/tests.rs`), and the orchestrator commits them as three commits by path.

U4 to U6 do not start before U3b's verdict. U6 is committed after U5, since F.5 item 8's
closing line names U4's and U5's commits; F.4's count is taken after F.5's edits.

After U5's and U6's reports, beside U5's review (and U4's, if it still runs), on the tree
that holds both; the review's fixes follow, then the last gate:

- The three release showcases built, then the rehearsal R as in the steps 4–5 plan
  (`<tools>/rehearsal.sh`, read by `scripts/check_showcase_parity.py`). Expected new keys:
  none (E.13); any other is fixed in code or excepted with its reason and source. The
  connector's doctests, the README's quick-start block among them (F.1 item 2), are compiled
  by the last gate's `test (native-theme-gpui, native, watch)` step (G.3), not run apart.
- **R's look**, after R, as its own run on :97 (`rehearsal.sh` stops each showcase once its
  layout is dumped, and opens only the Basic page): Xvfb started as `rehearsal.sh` starts it;
  the gpui showcase launched as `rehearsal.sh` launches it (`--capture`, `--icon-set breeze`
  for kde-breeze light, `material` for material-3 light), once with `--tab basic` and once
  with `--tab icons`; `import -display :97 -window root <file>` once its `--dump-layout` file
  is written; then the showcase and Xvfb stopped. The four captures are read by the session.
  Expected, on the two `--tab basic` captures: the caret of the Basic page's select
  (`basic-select`, gpui-component's `Select`) is drawn and is the set's `ChevronDown` (the
  theme's `go-down` under breeze, Material's `expand_more`), and so are the carets of the
  chrome's icon-theme Select and preset Combobox where the capture shows them; the gpui log
  holds no asset error. Not drawn through the handle, so no part of the look: the Basic page's
  check boxes (the connector's stroked mark under both presets, which state
  `checkbox.check_mark_stroke_width`) and its number field's two steps (the showcase's own
  chevrons under a theme, `demo::number_input`; gpui-component's Minus and Plus are drawn only
  without one). The two `--tab icons` captures are read for the `icons-widget-icons` caption:
  served, and the names it lists as drawn empty. Not verified: that a capture of the private
  display shows the window's content, and that the caption, in the page's fourth section, lies
  inside the window without scrolling (the showcase has no flag that scrolls). Where either
  fails, that part of the look falls to the close-out's captures, and the record says so.

## 3. The measurement and its verdict (U3a to U3c)

- U3a's `--commit` passes, and the run is made at once (C.2's command, outside the runner
  environment), while the review of U1 to U3a runs. The verdict is read when the review closes; a
  must-fix that changes a figure is a fix commit and a second run (the first run's
  directory is not committed).
- The verdict is C.5's: the session reads `listing.md` and the sheets in
  `target/widget-icons-sheets/<run>/` against the seven conditions. A condition met stops
  the work for the maintainer with the table and the sheets; a change he then decides
  (`Check`'s order, the size, an entry, D.13) is made in the spec first. None met: no stop;
  the record takes each verdict and the sheets' paths, he is told them in the next status,
  and U3c follows at once.
- The run reads the installed icon themes through the library only; he is told so with the
  approval and asked then for the package and version of each judged theme (`breeze`,
  `breeze-dark`, `Adwaita`, and the system theme if it is another), which he can give on the
  day (C.4's README).

## 4. Tests and gates

- During a unit: only the tests the spec names for it and the ones its edits reach, through
  `<tools>/slot.sh <tools>/lowprio.sh cargo test -p <crate> … <names>`; the showcase's tests
  through `scripts/run_in_runner_env.sh`. A test the spec says turns red is seen red first.
- The orchestrator runs §2's two gates through `<tools>/gate.sh [--fast] <worktree's full
  path> <name> <args>`: two `--commit` gates, the GPUI Fast legs at the last one. The two legs
  are `cargo test -p native-theme-gpui --features gpui-fast,native,watch --all-targets` and
  `scripts/run_in_runner_env.sh test -p native-theme-gpui --features gpui-fast,native,watch
  --example showcase-gpui`, which `--commit` skips; clippy builds on GPUI Fast in every gate.
  The first gate has no Fast legs: A and B have no code that differs between the cores. U4's
  agent runs D.12's tests on both cores (`--features native,watch` and `--features
  gpui-fast,native,watch`, `--lib`, filtered to the handle's module) and `cargo test -p
  native-theme-gpui --features native,watch --doc`, since D is the part that does.
- Why two gates and not a `--quick` per unit (the script's cadence): each unit's own tests
  run in it, the gate's extra is lint, docs, formatting and the crates' whole suites, and the
  KDE steps 4 and 5 ran the same two gates (`s45-u1-u4`, `s45-u5-u6`) without a fault the
  per-unit gate would have found earlier. The cost: a failure lies in one of three or four
  commits, and the failing step names its crate.
- Before each unit's report: `scripts/check_strict_panic.sh` for the crates it changed.
- The whole suite and the release checks run once, in 0.7.2's close-out (its full check).

## 5. Reviews

U1 to U3a by one `review` agent over their three commits, from the agent's report, beside
the first gate and the run (the orchestrator instead, where the three are a few hundred
lines in all); U4 by `review-hard` (state shared across threads), the one Fable agent,
beside U5; U5 by a `review` agent beside R; U3c and U6 by the orchestrator. A reviewer is given the commit and the spec's parts, not the author's report.
A must-fix is a commit of its own before the next gate.

## 6. Briefs

The form of the steps 4–5 briefs: the spec's parts quoted by name, `decisions.md`'s rulings
that touch them, the two reviews' anchor tables as the lookup (an anchor not found as the
spec quotes it is a stop), the wrappers, the tests, and the rules: no sub-agents, no `find
/`, nothing read outside the worktree, the registry and the record; no `unsafe`, no
panic-prone code, no LGPL code; edits only with the Edit and Write tools, and a hook's
refusal is reported, never written another way; new repository citations name the document
only; no commit; one report at the end. U3a's brief adds unit 3's reach rule: the icon base
directories are read through the library only. U5's and U6's briefs say that the other agent
edits other files of the same tree at the same time; each changes, formats and stages only
its own paths. The old plan's done-criteria for units 1 to 5 (`0a9141b0`) are where the briefs'
done-criteria start: their test commands, their red-first steps and what a report quotes.
This plan and the amended spec overrule them wherever they differ. Known differences: no
clause on lanes, on the old plan's tasks MG, VF, X, V and T0, or on its sections, rules and
record; the gates, the commits and the run's timing are §2 to §4's (no gate after U1, U2
and U4; no GPUI Fast legs at the first gate; the run does not wait for the reviews; U3c is one commit without a gate of its own;
the showcase's tests run through `scripts/run_in_runner_env.sh`); and unit 4's item 2 loses
its line-count clause, since D.11 adds a sentence to `Checkbox`'s doc below every claimed
line: its criterion is that `every_colour_claim_is_read_at_the_line_it_cites` passes and that
the `dot` doc's last line is replaced one for one.

## 7. Stops

A quoted text not found; C.1 red; B.3's `M ≥ F`, its no-opaque-part fixture failing, or a
bundled icon not judged `Some(true)`; a condition of C.5 met; `run_in_runner_env.sh
--prepare` exit 75 at U5; a decision neither the spec nor `decisions.md` makes.

## 8. The maintainer's part

The approval of W1 to W11 as amended, the spec and this plan; with it he is told U3b's
reach (§3) and asked for the judged themes' package versions. His to overrule: the
rationale keeps its name while the spec and the plan are renamed (the spec's decision 18);
the report's variant is `NotServed`, not `NotServedInThisRelease` (it also covers a set the
connector does not know, and would be wrong once the pixel sets are served); the ROADMAP
heading's wording (F.4). Told, not asked: under a served set neither loading button gives
its spinner an icon (E.4); the Widget Info's notes say "no SVG of one colour" and "pixels,
or an SVG in several colours" without telling the two apart (E.3); `Minus` and the
right-to-left chevrons need no design text; the caret's and the steps 6–7 documents still
name the widget icons' own release and lane, and are amended when each is next to be built. At the measurement: nothing where no condition
is met; condition 7 met asks his judgement of two figures, which decides D.13. Then 0.7.2's
capture window, the merge, the tag and the publication.

## 9. Additions to 0.7.2's close-out K

The five of the spec's G.4: the archive move in 0.7.2's one row with F.6's rewrite and its
check; rule 22 (c)'s search over the stamped paths with the stem `gpui-widget-icons`, before
K (expected: nothing); the look at the captures, the screenshots workflow's under
macos-monterey, macos-tahoe and windows-11 included; a parity difference treated as a fault
by rule 19; the sheets under `target/widget-icons-sheets/` deleted on the maintainer's word.

## 10. Cost check

- **What could refute the design, first:** the measurement's conditions 1 and 2 on the
  judged themes, whether the installed `breeze`, `breeze-dark` and `Adwaita` (and the
  session's system theme) each give the nine essentials at 16 px as icons of one colour,
  and more than half of the 106 names. If one does not, the handle declines the very themes
  the work is for, and W4's floor or W9's size is decided again. It is first: it needs U1's
  method and U2's lookup, which it measures, and nothing else; an earlier probe would need
  its own one-colour test, a stand-in. The mechanism itself (the resolver, a path the asset
  source answers, a mask drawn) rests on upstream code read at 0.9.1 and at 0.10.0 and is
  first observed on the real engine by R's look.
- **What waits for it:** U4, U5 and U6; no part of D, E or F is built before the verdict.
- **Built twice:** nothing. The measuring tool is removed after its run; the README's block
  is U4's doc example.
- **In parallel:** U5 beside U6 on separate files; each review beside the next unit or gate;
  the first gate and the run beside the review of U1 to U3a; R's release builds (another
  profile) beside U5's review. At most four agents, one of them Fable (U4's review); U6 and
  the reviewers run no cargo command. Not in parallel, by design: U1 to U3a (each builds on
  the one before, one build target), U4 with U5 (U5 needs the handle; a second worktree is a
  second full build). U6 is not started beside U4: it would finish no sooner, U5 being the
  longer path, and F.1 copies U4's doc example.
- **What each agent reads:** its parts of the spec, `decisions.md`'s rulings on them and the
  reviews' anchor lookup; not the rationale.
- **Tests per round:** the unit's own (U1's with the stem index, U4's on both cores and the
  doctests); `--commit` twice, the GPUI Fast legs once; R and its look once, beside U5's
  review; the full suite once, in K. Measured on 0.7.2's earlier gates, each `--quick` this
  drops took 3 to 13 minutes on the critical path.
- **Compaction:** before K, and earlier where the conversation passes about 300 k tokens
  (it was compacted just before the approval).
