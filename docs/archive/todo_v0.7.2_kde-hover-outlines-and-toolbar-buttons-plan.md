# 0.7.2: issue #4's open KDE items, steps 4 and 5 (the hover outlines of the check box, the radio and the combo box; a toolbar button's states) — implementation plan

- Rationale: [`../issues/0004_kde-open-items-rationale.md`](../issues/0004_kde-open-items-rationale.md),
  D6 (step 4), D7's toolbar part (step 5). Specification:
  [`todo_v0.7.2_kde-hover-outlines-and-toolbar-buttons-spec.md`](todo_v0.7.2_kde-hover-outlines-and-toolbar-buttons-spec.md)
  ("the spec"), whose Part C this plan carries out.
- Decided by the maintainer on 2026-11-18: steps 4 and 5 are part of v0.7.2, as steps 2 and 3
  are ([`todo_v0.7.2_kde-focus-ring-and-button-outlines-plan.md`](todo_v0.7.2_kde-focus-ring-and-button-outlines-plan.md)),
  not a release of their own; the version stays 0.7.2, whose *Breaking Changes* bullet names the
  new fields.
- Written 2026-11-18. It replaces the plan of 2026-11-15 for a separate release (git history
  keeps it). Status: approved by the maintainer with D6, D7 and the spec on 2026-11-18.
- The record: `<record>` = `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan/`,
  0.7.2's own record, with this work's files in `<record>/steps45/`. `<tools>` =
  `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan/tools/`.

## 1. Where and from what

- One worktree, `.claude/worktrees/lane2-b`, branch `v0.7.2-rc1`, from its head `f02c3ae9`
  (steps 2 and 3 built and gated `--commit` 148/148; pushed; draft pull request #11). The units
  are commits on that branch: no new branch, no merge, no version step. The branch is pushed by
  explicit refspec at each `--commit` gate that passes, and each push's CI run is read (the
  Windows reader's tests run only there; a red run emails the maintainer).
- The evidence already taken, in `<record>/steps45/`, read by the spec's amendment:
  - `upstream-reread.md`: every upstream citation re-read at egui 0.37.0 and gpui-component
    0.10.0: nothing the design rests on changed; lines and one stale doc (`demo::ghost`) did.
  - `review-partA.md`, `review-partBC.md`: the first-pass reviews, with every repository anchor at
    `f02c3ae9` (the briefs' lookup).
  - `trial/RESULTS.md`: T1, a fourth tool button (Edit) in the Basic page's icon row, headless
    layout dumps before and after: no element leaves its view; Edit starts a second line under
    adwaita and catppuccin-mocha, the groups below moving 32 to 40 px, alike in the three
    showcases. T2, the icon of a selected gpui `Button` takes the refined text colour (decision
    14): passes on both backends. T3, the gpui showcase's hover overlay over a combo trigger
    (decision 7): passes on both backends. The patches are the units' start where the spec says.
  - `decisions.md`: the rulings on every finding and the open points; the spec applies them.
  - E4 (below): what Breeze draws on an open combo box, in
    `docs/evidence/breeze-offscreen/2026-11-18/`.

## 2. Units

Each unit is one commit; a review fix is a commit of its own, gated by the next gate.

| Unit | The spec's parts (Part C) | Agent | Gate after it |
|---|---|---|---|
| D0 | The documents: the amended spec, this plan, the links to the old names (`todo_v0.7.x_kde-open-items-steps4-5-*`) in other documents re-pointed (none inside `docs/archive/`), E4's evidence folder `docs/evidence/breeze-offscreen/2026-11-18/` and its row in `docs/evidence/README.md` | orchestrator (commit) | the five documentation checkers |
| E4 | Decision 10's measurement (done before D0; its folder and README row are committed with D0) | `worker` | its own check (the closed, hovered trigger's pen is `#3daee9`) |
| U1 | Both cores: A.1 to A.7 (A.3's KDE cell written whole, as its table gives it), B.1 to B.5, B.5a, and the backlog edits the spec puts in U1 | `implement` | `cargo test -p native-theme` with the gate's features; the five documentation checkers; and, of the *system-icons (cross-target)* section of `scripts/check_release.sh`, the `all features, -D warnings` check for `x86_64-pc-windows-gnu` and `clippy, all features` for `x86_64-pc-windows-msvc`, which compiles the reader's tests; not `--quick`: the three connectors stay red until each reads the new leaves |
| U2 | gpui, both steps without Edit (A.10 but its two `HOW` lines; B.9) | `implement` | none of its own: built beside U3 and U4 |
| U3 | iced, both steps without Edit (A.8 (decision 10); B.8) | `implement` | none of its own: built beside U2 and U4 |
| U4 | egui, both steps (A.9; B.7; B.10's `tool_button` edge) | `implement` | one `--commit` + GPUI Fast legs over U1 to U4; then the push |
| U5 | The element lists and Edit in the three showcases (A.11, A.10's two `HOW` lines, B.10) | `implement` | none of its own: built beside U6 |
| U6 | Both steps' documents into 0.7.2's sections (A.12, B.11): CHANGELOG, ROADMAP, the READMEs' 42, the backlog, the first rationale | `worker` | the check on the built tree and the rehearsal R (below), then one `--commit` + GPUI Fast legs + the five documentation checkers over U5 and U6; then the push |

After U5's and U6's reports and before their `--commit`, on the tree that holds U5's change:

- First the three release showcases are built on that tree, through `slot.sh`, with T1's
  commands (`<record>/steps45/trial/t1/build-after.log`: `cargo build -p native-theme-iced
  --example showcase-iced --release --features iced_aw`, the egui one with
  `--all-features`, the gpui one as that log has it); the check and R read those binaries.
- **The check on the built tree** (the spec's B.10): the three release showcases under kde-breeze
  light and dark, adwaita light, material-3 light and catppuccin-mocha light, by a copy of
  `<record>/steps45/trial/rehearsal-t1.sh` cut to those five entries, under `xvfb97.lock`, then
  `<record>/steps45/trial/compare-t1.py` against T1's "after" dumps. Expected: no stop entry, no
  parity entry, no moved line. A stop entry is a stop; a parity entry or a moved line is fixed or
  explained in the record before the gate.
- **The rehearsal R** again (0.7.2's unit R: `<tools>/rehearsal.sh`, read by
  `scripts/check_showcase_parity.py`). Expected new keys: `catppuccin-mocha:basic.icons.edit.*`,
  which join the existing exception group of Copy, Paste and Delete; any other new key is fixed in
  code or excepted with its reason and source before the gate.

Before the close-out, the search over the stamped paths (0.7.2's rule 22 (c), its command in
the record) with this spec's and this plan's stems (expected: nothing), as the steps 2–3 plan
has it for its own.

Then 0.7.2's close-out, as the steps 2–3 plan leaves it, with these additions: the archive moves
this spec and plan with 0.7.2's other documents, and the rationale's status paragraph takes one
sentence for steps 2 to 5 (spec A.12, in place of the steps 2–3 plan's); the asset run's parity
list is read for Edit's keys; the captures are looked at for Edit, the screenshots workflow's
under macos-monterey, macos-tahoe and windows-11 included.

Why this cut: U1's two cores share the core crate and its checkers; U2, U3 and U4 each change one
connector's folder (U3 also `ROADMAP.md`'s count of iced style functions; U4 also
`scripts/connector_parity_exceptions.toml` and egui-widgets' doc line), no connector depends on
another, and all build on U1's tree: three builders at once in the one worktree, each compiling
and testing its own package (`-p`) through `slot.sh`, which lets two cargo commands run at a time,
as in the steps 2–3 block. U5 touches the three showcases and `docs/showcase-elements.toml`, so it
runs alone, beside the documents-only U6. The orchestrator commits each unit by its paths. U1
cannot pass a workspace gate alone, so the first `--commit` closes over four code units, one more
than the batch's cadence of three: no earlier gate can pass.

## 3. Tests and gates

- During a unit: only the tests the spec names for it and the ones its edits reach, through
  `<tools>/slot.sh <tools>/lowprio.sh cargo test -p <crate> … <names>`; the showcase tests of the
  showcase it touches through `scripts/run_in_runner_env.sh`; tests that read painted primitives
  in the GPUI Fast form too. A test the spec says turns red is seen red first.
- At a unit's end the orchestrator runs §2's gate through `<tools>/gate.sh [--fast] lane2-b <name>
  <args>`: two `--commit` gates in all (measured in 0.7.2: 7 to 21 minutes each).
- Before each unit's report: `scripts/check_strict_panic.sh` for the crates it changed, and
  `<record>/z1/stand-in-check.sh` on a temporary index that holds the unit's own paths (a builder
  of the block adds only its own connector's paths). U2's and U5's briefs also say `git grep -n
  trial_` over their paths finds nothing (the T2 and T3 patches' test names).
- The whole suite and the release checks run once, in 0.7.2's close-out (its full check).

## 4. Reviews

Every code unit is reviewed beside the unit that follows it, by size: U2, U3 and U4 by three
`review` agents at once, beside U5; U5 by a `review` agent before its gate; U1 (data the checkers
validate, the two readers' few assignments with their tests; B.5a adds no `unsafe`) and U6 (prose
the checkers hold) by the orchestrator. A reviewer is given the commit and the spec's parts, not
the author's report. A must-fix is a commit of its own before the next gate.

## 5. Briefs

The form of the steps 2–3 briefs: the spec's parts quoted by name, the decisions that touch them,
the anchors' lines at `f02c3ae9` from the two reviews' tables as a lookup (an anchor not found as
the spec quotes it is a stop), the trial patch a unit starts from, the wrappers, the tests, and the
rules: no sub-agents, no `find /`, nothing read outside the worktree, the registry and the record;
no `unsafe`, no panic-prone code outside tests (in `#[gpui::test]` functions test code uses `let …
else`), no LGPL code; edits only with the Edit and Write tools, and a hook's refusal is reported,
never written another way; new repository citations name the document only; no commit (the
orchestrator commits after reading the report); one report at the end. In the block of three and
in U5 beside U6, the brief also says: other agents edit other folders of the same tree at the same
time; change, format (`cargo fmt -p <crate>`) and stage nothing outside your own paths, which the
brief lists.

## 6. Stops

A decision neither the spec nor `decisions.md` makes; a stop entry of the check on the built tree;
the failure of the icon-colour test that T2 measured (the spec's B.9).

## 7. The maintainer's part

The approval of D6, D7, the spec and this plan; with them he is told, not asked: Edit's second line
under adwaita and the colour-scheme presets (T1), the two new `[1.0]` backlog items (a toolbar
button's Tab focus; an open combo box's frame (E4)), three differences from the platforms the
connectors and showcases keep, each visible on its platform (a checked toolbar button under the
pointer and while it is held, which WinUI, libadwaita and Breeze draw in fills of their own, the
spec's decision 5 and its new `[1.0]` entry; no hover outline over a checked toolbar button in
gpui, where iced and egui draw it, B.13.6; the toolbar look the showcases' tool buttons keep
where an icon set lacks the icon and the label shows, which on GNOME a text-only button would
not have, B.13.9), and the ROADMAP heading's new wording, which is his to change. Then 0.7.2's capture window (once, 4 to 6 minutes), the pull
request's merge, the tag, the publication and the GitHub release; and whether a follow-up comment
goes on the issue.

## 8. Cost check

- **What could refute the design, first:** whether the fourth tool button breaks the Basic page's
  layout (T1: no), whether gpui can colour a checked toolbar button's icon and draw the combo
  box's hover outline without moving it (T2, T3: yes, both backends), whether the upstream lines
  still say what the spec builds on (re-read: yes), and what Breeze draws on an open combo box (E4:
  the `Highlight` pen and the pressed fill, pointer or not). Left open: nothing of the design.
  What waits: nothing.
- **Built twice:** nothing. The trial patches are the units' start; the desktop measurement of the
  old plan is replaced by T1 and the check on the built tree, headless.
- **In parallel:** the re-read, two reviews and the trials ran as four agents at once. In the
  build: U2, U3 and U4 by three builders at once in the one worktree; U5 beside U6; the reviews
  beside the next unit, three at once for the block. At most five agents, of which only U5 runs
  cargo commands (the three reviewers and U6 run none); two cargo commands at a time.
- **What each agent reads:** its parts of the spec, `decisions.md`'s sections on them, the anchors'
  lookup, its trial patch; not the rationale.
- **Tests per round:** the unit's own; `--commit` twice; the check on the built tree and R once,
  after U5; the full suite once, in the close-out.
- **Time (estimate, not measured):** U1, the block of three (as long as gpui's unit), and U5 with
  U6, each about 45 to 90 minutes, plus two gates and the check with R: about 4 to 5 hours, the
  steps 2–3 build having taken about 2.5 hours against an estimate of 3 to 5. Then 0.7.2's
  close-out, unchanged in length.
- **Compaction:** after the documents' approval, before U1; again before the close-out, and
  earlier where the conversation passes about 300 k tokens.
