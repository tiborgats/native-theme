# 0.7.x: issue #4's open KDE items, steps 6 and 7 (the default button; the table header's states and the unfocused selection) — specification

- Rationale: [`issues/0004_kde-open-items-rationale.md`](issues/0004_kde-open-items-rationale.md)
  ("the rationale"), decisions D8 (step 6), D7's second half and D9 (step 7), D0 for
  the version and what D10 says a step carries outside the code. Not approved when this
  was written; nothing here is done before the maintainer approves the rationale, this
  specification and its plan. It decides nothing the rationale did not decide, except
  the small points under *Decisions taken while specifying*; what needs a decision the
  rationale does not contain is under *Open points*. What the rationale's §0 leaves
  in the backlog as it is stays there: among it the three more outlines the first
  rationale put into the same step as its items 5 and 6 (a hovered menu item's and a
  hovered list row's 1 px outline, and a menu bar title's fill), none of which these
  steps add (7.I.3 keeps the first rationale's sentence that says so).
- It continues [`issues/0004_kde-hand-overrides-rationale.md`](issues/0004_kde-hand-overrides-rationale.md)
  ("the first rationale"), §5.2 (the default button), §5.3 (the unfocused selection)
  and §4.4 (the header's fills), and the backlog entries `docs/todo.md:1774-1825`
  (*What Breeze draws that the model has no field for*), `:1826-1913` item (2) (*KDE
  values that wait for a connector design*), `:1914-1933` (*The selection of a view
  without the focus*) and `:543-580` (*Selection: the text colour macOS pairs with the
  unemphasised selection*). Form: [`archive/todo_v0.7.1_kde-hand-overrides-spec.md`](archive/todo_v0.7.1_kde-hand-overrides-spec.md)
  and step 1's [`todo_v0.7.1_kde-open-items-step1-spec.md`](todo_v0.7.1_kde-open-items-step1-spec.md).
- Written: 2026-11-14, against `v0.7.1-rc2` at `0fa28027`. Every `file:line` is that
  commit's unless it names another. `P8` is the main checkout on `v0.8.0-rc1` at
  `78ee0efd`, `F` its `docs/facts/`; `GC`, `IW`, `EG`, `EX` are the `src/` folders of
  gpui-component 0.9.1, iced_widget 0.14.2, egui 0.36.2 and egui_extras 0.36.2, as in
  the rationale. D10 proposes the egui 0.37 upgrade before every step, so where a
  sentence or a code comment of these steps rests on an egui line, it is given at
  0.37.0 too (read today in Cargo's registry), and the code cites the version its tree
  builds against.
- **Steps 2 to 5 and the caret land before these two** (D10: 6 and 7 come last; the
  batch's S3 as amended puts the caret in the KDE lane, right before them). They add
  fields to the same structs, rows to the same facts tables, values to the same preset
  sections and rows to the same contracts, so every line number below is
  `0fa28027`'s and moves. **No line numbers in new citations** (the batch's S6,
  decided by the maintainer 2026-11-15): every citation this document has a unit
  write names the document, with a quoted phrase or a row's name where that helps,
  never a line number of a tracked file that changes; existing line-number citations
  are left as they are and may drift, and nothing re-points them; a pinned source (an
  upstream crate at a version, a citation "at <sha>") keeps its lines. The line
  numbers this document gives are places to find, read at `0fa28027`. An edit is applied by its anchor (the quoted text, the key, the test's
  name); a count stated at `0fa28027` (the egui mapping's totals, the derived leaves
  of the KDE tests) is given as "today's, plus what steps 2 to 5 added", and the
  implementer reads the base from the tree. Step 6's gpui part rests on step 3's trial
  (D5, D8) and on 6.D.4's test of the default button: 6.D.2 names the one condition
  every gpui branch of step 6 reads, "gpui has no receiver", and its three cases.
- **The tree U6.1 starts from** is the one the releases before step 6 leave (under S2 as
  amended, the lane predecessor's gated head, `<lane-pred-head>`, into which the tag of
  the release immediately before it in the maintainer's order, `<prev-tag>`, is merged
  after the release's last unit before the merge, U7.4, has passed its gate, U7.3
  before it (rules 17, 20)). The KDE lane's own releases are in it (v0.7.1, steps 2
  and 3, steps 4 and 5, the caret's blink, its lane predecessor); the egui 0.37 upgrade
  (v0.7.2) and the second lane's release, gpui-component's icons (with the step that
  serves the two pixel sets), arrive through the merge of `<prev-tag>`, after this
  release's last unit before the merge, unless the branch is made from `<prev-tag>`
  itself (rule 25, batch-speed rationale, Appendix; D10's proposed order, the
  rationale's *The releases after v0.7.1*, items 1 to 4, as S3 as amended lays the
  lanes out). So the egui lines this document gives at 0.36.2 are the ones the units
  write, and their 0.37.0 forms are the merge's rework (rule 25). The icons release
  edits the same showcases and element list as step 6, so 6.G's measurement and every
  anchor of 6.G are read on the KDE lane's tree, and what that release changes on the
  page meets this release's at the merge (the asset run's parity checks and the
  rehearsal's after-only checks, 6.G). The facts file's lines on the tree U6.1 starts
  from are not `0fa28027`'s: the releases before step 6 insert rows (step 2's 2B.1 one
  row, after `focus_ring_offset`, moving every later line by one; step 3's 3B.1 two
  rows directly before §2.3's `hover_background`, moving every later line by two;
  steps 4 and 5's first facts edit directly after `hover_background`, `PF:1244` in
  their specification; the caret's A.8 after `:1209` at `0fa28027`), and drop no blank
  line, so every facts line this document names is found by its text. Step 6 needs step 3
  (decision 20's field `hover_border_color` and the trial 6.D.2 reads) and step 5
  (B.9's form, which 6.D's builders follow, and B.10's measurement, which 6.G takes
  again) in its tree, both the KDE lane's.
- **What this document takes from the earlier steps' specifications**, read once on
  2026-11-14 while their authors amend them (each dependency is named where it is used):
  step 1's Part A (`docs/todo_v0.7.1_kde-open-items-step1-spec.md`: `shade.rs`, the
  folder `native-theme/tests/vectors/`, the declaration `mod shade;` and its decisions 1
  to 3), for 7.B; steps 2 and 3's Part 3F (`docs/todo_v0.7.x_kde-open-items-steps2-3-spec.md`:
  `variants::bordered_button`, its trial `bordered_button_matches_the_default_button_but_its_hover_border`,
  the pass criterion 3F.3, the two outcomes 3F.4 and 3F.5, the private colour function
  `variants::hover_border` the contract's `sinks` reads), for 6.D and 6.G; their 2B.1 and
  3B.1 (2B.1 a facts row after `focus_ring_offset`; 3B.1 two rows directly before
  §2.3's `hover_background`; rows only, no blank line dropped), for Part F; steps 4 and 5's B.9 (the gpui builder
  `geometry::toolbar_button_rest`, the form of 6.D's `geometry::default_button_rest`, and
  their builders' place after `geometry::button_checked`) and B.10
  (`docs/todo_v0.7.x_kde-open-items-steps4-5-spec.md`: the checked tool button
  `basic.icons.edit` in the Basic page's icon row, and its layout measurement, with the
  `visible_area` stop rule, nine entries, pre-check and pre-build, re-read the same day
  as amended), for 6.D, 6.G and 7.H; their reply
  edits (B.11) and steps 2 and 3's rule for them (2I.3, 3H.3), for 7.I.3. Where those
  parts change, the parts of this document that name them change with them.
- Release: steps 6 and 7 are **one** 0.7.x release (the batch's speed decision S5,
  `docs/todo_v0.7.1_batch-speed-rationale.md`, approved 2026-11-15): one branch, one
  version, one CHANGELOG section, one ROADMAP paragraph, one close-out with one asset
  run (and its capture check, 7.J.4) and one archive at the end; each step keeps its own
  units, measurement (6.G), reviews and record inside the release. Written for D0 (a),
  the rationale's proposal: the version is `0.7.N`, `N` the next patch number when the
  release is cut. Open point 1 says what changes under D0 (b) and (c). The branch is
  made from its lane predecessor's gated head (the caret's) and, after its last
  unit before the merge (U7.4, after U7.3) has passed its gate, merged with the tag of the release immediately before it in the
  maintainer's order (S2 as amended 2026-11-15; 6.I.1), and the units' gates follow
  the batch's cadence (S1), as 6.I.1 says. The batch's working rules are cited as
  "rule N": rule N (batch-speed rationale, Appendix), the consolidated list in
  `docs/todo_v0.7.1_batch-speed-rationale.md`, which governs where this document and
  it differ; the rules and S2's amendment were approved by the maintainer on
  2026-11-15 (the plan's §8 item 8).
- Every value below was read today from the file named, or computed today; the
  computations are named where they are given. What could not be verified is marked
  so, with the reason. No cargo command was run.

## How to read it

Two parts, each complete in itself: **Step 6** (D8) and **Step 7** (D7's header, D9).
Each has the same order of sub-parts, by owner, and ends with its own order, gates and
close-out. Part F, before both, states the rule both parts follow to add rows to
`docs/platform-facts.md` and to insert lines into a preset; the citations those
insertions move are left as they are (S6).

| Part | Rationale | Files it owns |
|---|---|---|
| F — Facts rows and preset lines: placement | D8, D7, D9 | `docs/platform-facts.md` (the rows of 6.B and 7.C) |
| 6.A — The model | D8 | `native-theme/src/model/widgets/mod.rs`, `native-theme/tests/proptest_roundtrip.rs`, `native-theme/src/presets.rs` (one test) |
| 6.B — Registry, rules, facts | D8 | `docs/property-registry.toml`, `docs/inheritance-rules.toml`, `docs/platform-facts.md` §2.3 |
| 6.C — kde-breeze and the KDE reader | D8 | `native-theme/src/presets/kde-breeze.toml`, `native-theme/src/kde/colors.rs`, the tests in `native-theme/src/kde/mod.rs` |
| 6.D — gpui connector | D8 (gpui), step 3's trial (3F) | `connectors/native-theme-gpui/src/variants.rs`, `src/geometry.rs`, `src/contract.rs`, `README.md`, `src/lib.rs` (module doc), the `GEOMETRY_NOTES` entry in `examples/showcase-gpui/info/mod.rs`; where gpui has no receiver because step 3's trial failed (6.D.2, case 1), request 5 in `docs/issues/0004_kde-hand-overrides-upstream-requests.md` |
| 6.E — iced connector | D8 (iced) | `connectors/native-theme-iced/src/styles.rs`, `src/contract.rs`, `src/contract/rows.rs`, `src/contract/derived.rs`, `src/contract/pairs.rs`, `contract-leaves.toml` (generated), `README.md` |
| 6.F — egui connector | D8 (egui) | `connectors/native-theme-egui/src/accessors.rs`, `src/lib.rs`, `mapping.toml`, `src/mapping.md` (generated), `src/mapping_tests.rs`, `src/install_tests.rs`, `README.md`; `scripts/connector_parity_exceptions.toml` |
| 6.G — Showcases | D8 | the three showcases' Basic page, `docs/showcase-elements.toml` |
| 6.H — Outside the code | D10 | `CHANGELOG.md`, `ROADMAP.md`, the first rationale, the rationale's status line, `docs/todo.md` |
| 6.I — Order, gates, close-out | D10; S1, S2, S5, S6 | the units' cadence, the branch and its merge (the release's one close-out and asset run are 7.J's) |
| 7.A — The model | D7, D9 | `native-theme/src/model/widgets/mod.rs`, `native-theme/src/model/defaults.rs`, `native-theme/src/model/resolved.rs`, `native-theme/src/model/mod.rs` (one test), `native-theme/src/resolve/validate.rs`, `native-theme/src/resolve/inheritance.rs`, `native-theme/src/resolve/tests.rs`, `native-theme/tests/proptest_roundtrip.rs`, `native-theme/tests/serde_roundtrip.rs` |
| 7.B — The carried clean-room modules | D9 | new `native-theme/src/kde/kde_color_common.rs`, `native-theme/src/kde/kcu_tint.rs`, `native-theme/tests/vectors/kcu_tint.tsv`, `native-theme/tests/vectors/kde_selection_inactive.tsv`, new `native-theme/src/kde/vector_tests.rs`, their declarations in `native-theme/src/kde/mod.rs` |
| 7.C — Registry, rules, facts | D7, D9 | `docs/property-registry.toml`, `docs/inheritance-rules.toml`, `docs/platform-facts.md` §2.1.3, §2.15 |
| 7.C2 — Evidence carried from the parked branch | D9 (macOS) | new files under `docs/evidence/os-probe/2026-10-31-run-41195846264/` and `docs/evidence/os-probe/2026-11-06-run-42215651196/`, two rows of `docs/evidence/README.md` |
| 7.D — Presets and readers | D7, D9 | `kde-breeze.toml`, `macos-monterey.toml`, `macos-tahoe.toml`, `native-theme/src/kde/colors.rs`, the tests in `native-theme/src/kde/mod.rs`, `native-theme/src/macos.rs` |
| 7.E — gpui connector | D7, D9 | `connectors/native-theme-gpui/src/lib.rs`, `src/contract.rs` |
| 7.F — iced connector | D7, D9 | `connectors/native-theme-iced/src/lib.rs`, `src/contract/rows.rs`, `src/contract/derived.rs`, `contract-leaves.toml` (generated); `scripts/connector_parity_exceptions.toml` |
| 7.G — egui connector | D7, D9 | `connectors/native-theme-egui/src/accessors.rs`, `src/lib.rs`, `mapping.toml`, `src/mapping.md` (generated), `src/mapping_tests.rs`, `src/install_tests.rs`, `README.md`; `scripts/connector_parity_exceptions.toml` |
| 7.H — Showcases | D7, D9 | the three showcases' tables, lists and their tests, `docs/showcase-elements.toml`, the macOS capture step and upload of `.github/workflows/screenshots.yml` |
| 7.I — Outside the code | D10 | as 6.H |
| 7.J — Order, gates, close-out | D10; S1 to S5 | as 6.I; the release's one close-out for both steps (S4's order) with its asset run, the asset run's capture check, and `tools/showcase-support/src/capture.rs` where it fails |

Every edit has one owner within a step. Files edited in both steps (`widgets/mod.rs`,
`kde/colors.rs`, `kde/mod.rs`, `kde-breeze.toml`, the registry, the rules, the facts,
the connectors' contracts and mapping, the parity exceptions, the showcases) are edited
by step 6 first and by step 7 on the tree step 6 left.

**Lines that must not move.** The gpui showcase's test
`every_colour_claim_is_read_at_the_line_it_cites`
(`connectors/native-theme-gpui/src/showcase.rs:2819`) opens each colour claim at the
line it cites (the batch rationale's S6, *The fourth gate, kept*: a unit that moves a
cited line either keeps the line count above it, which every edit below does, or
re-points those claims in the same commit). Listed today from every `<file>.rs:N` in `examples/showcase-gpui/` and
`src/showcase.rs`, for the files this release edits or might: `colors.rs` (139 claims,
lines 281 to 718), `variants.rs:62`, `:63`, `:64` (`ghost_button`'s three setters),
`geometry.rs:230`, `:737`, `:740`, `:772`. The claims also pin lines in
`src/widgets/*.rs` (`checkbox.rs` and `switch.rs` 37 citations each, `tab_bar.rs` 9,
`slider.rs` 5, `spinner.rs` 4, `mod.rs` and `progress.rs` 3, `separator.rs` 1, at
`0fa28027`), files no unit of this release edits. The
scan's two other hits are no colour claims: `lib.rs:94` is gpui-kit's (`main.rs:56`,
"gpui-kit `src/lib.rs:94, 101-103`"), and `lib.rs:834` the connector's `font_style`
cited in a comment (`demo.rs:2183`, "src/lib.rs:834-840"), which does not move: step
6 edits the connector's `lib.rs` in place (6.D.5's `:112` row) and step 7 adds after
`:920` (7.E). Neither step changes a line of `colors.rs` (its comments' old numbers
are left as they are, S6); every addition to `variants.rs`
goes after the last function steps 3 and 5 add there (step 3's `bordered_button` and
`hover_border`, step 5's `toolbar_button` and `toolbar_button_states`, all after
`switch_color`, `:91-93`) and before the test module (`#[cfg(test)]`, `:95` at
`0fa28027`), since steps 3 and 5's colour claims cite lines of theirs (3F.4's claim at
the line of `hover_border` that reads `hover_border_color`); to `geometry.rs` after
`button_checked` and the builders steps 4 and 5 add after it (`:1354-1378`, below the
last pinned line `:772`); to `lib.rs` after
`selection_inactive_background` (`:916-920`), so no pinned line moves. No citation of a
`variants.rs` line after `:93` or of a `geometry.rs` line after `:1378` exists today
(`git grep -noE '(variants|geometry)\.rs:[0-9]+'` outside `docs/archive/` and the dated
documents: the highest are `variants.rs:64` and `geometry.rs:1339`,
`docs/todo.md:4435`). Steps 2 to 5 add their own, below these; this step's go below
theirs, and step 6's own claims (6.G) pin the lines they cite in turn. This release
writes a line number of a changing file in two places only, each read by a gate S6
does not touch (S6 frees three other gates from line positions): the gpui colour
claims at `variants.rs` and `geometry.rs` lines, which
`every_colour_claim_is_read_at_the_line_it_cites` reads at that line, and the `HOW`
text `(native-theme-gpui/geometry.rs:230, border_color)` (6.G), which
`every_prose_citation_still_exists` reads.

## Decisions taken while specifying

Each is the smaller or the safer choice and changes no result of the rationale.

1. **F. Where the new facts rows go.** By the rule all steps share (Part F): beside the
   row each belongs with. D8's three after `primary_text_color` in §2.3, the pair they
   stand beside (step 3's two rows stand before `hover_background`, beside the hover row
   they belong with); D7's two after `header_height` in §2.15, the header's last row;
   D9's one directly after `selection_inactive_background` in §2.1.3. Rows only, no
   blank line dropped (Part F, F.1), so every later line moves; the citations of those
   lines are left as they are (S6).
2. **6.A. The rule "the fill decides" lives in each connector**, as the toggle's
   fallback to the pressed pair does (`geometry::button_checked`,
   `connectors/native-theme-gpui/src/geometry.rs:1369-1378`; `styles::button_toggle`,
   `connectors/native-theme-iced/src/styles.rs:270-306`), stated once on the model's
   fields. A method on `ResolvedButtonTheme` would be new public API in the core crate
   for three lines per connector.
3. **6.A. The default button's states where its fill is stated**, from Breeze's chain
   ("down, checked, default, rest; then the highlighted pen", `F/kde/button.toml:6-7`):
   at rest and hovered the three fields (hovered with the frame of decision 20);
   pressed, the plain button's pressed fill and label with that frame, as Breeze's
   `down` branch comes before the default one; disabled,
   the plain button's disabled look, as the model states no disabled default button
   (Breeze's is the Disabled group's default mix, `F/kde/button.toml:5965-6044`, which
   no field carries). Where the fill is unstated, every state is the primary button's.
4. **6.C. The default button's frame is mixed at Qt's precision.** Breeze mixes the
   highlight halfway towards `mix(Button, ButtonText, frameContrast)` without rounding
   the inner colour to 8 bits. Rounded to 8 bits first, the light frame comes out
   #87c0de where Breeze measured #87c0dd (computed today, below). So `mix_qt` is split:
   a `mix_qt_colors(a: QtColor, b: QtColor, bias)` that the existing `mix_qt` calls.
5. **6.F. egui's accessor returns one value for the default button's look**, a small
   `#[non_exhaustive]` struct `DefaultButton { fill, text, stroke, hover_stroke, focus_stroke }` inside
   an `Option`: `None` where the default button is the primary one. Four one-field
   accessors would each carry the rule, and their names would not be the leaves' (the
   parity script's naming rule), as each returns another leaf where its own is unset
   (`hover_stroke` is `button.hover_border_color`, else the default button's frame:
   decision 20).
6. **6.F. egui's default button: a hover frame is offered, a pressed fill is not.**
   egui's `Button::fill` and `Button::stroke` "override any on-hover effects"
   (`EG/widgets/button.rs:140-155`; egui 0.37.0 `:153-168`, the same words), so one call
   draws one look in every state. The states are therefore the application's, from the
   button's previous response, as egui's per-call overrides make necessary:
   `hover_stroke` is the frame it hands `Button::stroke` while that response was
   hovered or pressed (decision 20), and `focus_stroke` (`button.focus_border_color`,
   else `hover_stroke`'s colour) the one while it had the keyboard focus and was not
   pressed (6.F.2). No pressed fill or label is offered, because the
   pressed default button is the plain button's pressed fill and label (decision 3),
   which the connector's «button» role writes into egui's active widget visuals already
   (`button.active_background`, else `button.hover_background`, and
   `button.active_text_color`: `connectors/native-theme-egui/src/style/roles.rs:200-227`):
   while the previous response was pressed, the application leaves `fill` and the label's
   colour off and egui draws them. The accessor's doc and the README say so; the
   showcase draws all three states this way (6.G).
7. **6.G. The showcases show the default button on the Basic page, in a row of its
   own**, as a new shared element `basic.buttons.dialog_default` reading "OK", the
   Buttons group's fourth row, below the toggle row. The alternative, each showcase's
   dialog, has no confirming button in iced (`showcase-iced.rs:8880-8936` draws a
   close button only), so the three would not show the same element. The first row,
   beside the primary button, is not taken: three buttons need at least three
   `button.min_width` and two `layout.widget_gap`: 3 × 64 + 2 × 6 = 204 px on most
   presets, 3 × 64 + 2 × 8 = 208 on the two macOS presets and ios-13, 3 × 80 + 2 × 6 =
   252 on kde-breeze (every preset's `button.min_width_px` in both modes and its
   `[layout] widget_gap_px`, read today), while a column is sure only of the widest fixed-width
   control the theme states, at least `BASIC_WIDE`, 170 (`showcase-gpui/pages/basic.rs:41`;
   `basic_column_count`, `:263-309`), so the third button would wrap wherever the
   column is narrower than the row; the columns' widths at 1280x720 were not read. A
   row of its own is the same in all three showcases at every width, with no wrap to
   keep in parity; every group below it in column 1 moves down by one row, which 6.G's
   measurement checks. The arrangements in fewer columns were balanced by the groups'
   heights under material-3, the tallest ("fewer columns are balanced by the groups'
   heights under material-3", `docs/showcase-elements.toml:669-670`); the new row adds
   height to column 1 only, so the balance shifts, and the measurement checks only that
   every element stays inside its visible area, not that the columns stay balanced; the
   arrangements are not changed here. Steps 4 and 5 add their checked tool button to another group,
   the icon row (`basic.icons.edit`, their B.10), so the two additions do not meet.
8. **7.B. Module names and places.** `kcu_tint.rs` and `kde_color_common.rs` go to
   `native-theme/src/kde/` under their own names: `kcu_tint.rs` reads
   `use super::kde_color_common::{..}` (P8 `kcu_tint.rs:3`), which must hold as
   carried, and `kcu_` is KColorUtils' prefix, not the `kde_` that step 1 dropped from
   `shade.rs`. Their notice goes into each file's module doc, after its first lines, as
   step 1's decision 2 does; the tables to `native-theme/tests/vectors/`, beside step
   1's.
9. **7.B. The dead-code allow is unconditional on `kde_color_common`.** Its other
   functions (`mix_q`, `shade_q`, the HCY conversions, …) serve P8's `kcu_mix`,
   `kcu_shade` and `kde_effects`, which are not carried; which of them `tint_q`
   reaches was not traced, and a test build uses no more of them than a normal one. An
   `allow` (not an `expect`) does not fail where nothing is dead.
10. **7.B. The vector tests are a new test-only file, `kde/vector_tests.rs`.** P8 tests
    `kcu_tint` and `kde_selection_inactive` from `tests/platform_vectors.rs`
    (`:186-195`, `:288-299`) through helpers this tree has not; `kcu_tint.rs` has no
    test module of its own, and the carried file stays as written. The parsing follows
    step 1's carried `shade.rs` test (`P8 kde_shade.rs:335-434`), with `#rrggbbaa`
    added: both tables hold translucent inputs (read today).
11. **7.D. The KDE reader reads the two booleans as `true` or `false`, trimmed and
    ASCII-case-insensitive; any other value counts as missing**, so the rule falls to
    the next key. Breeze's schemes write `true` and `false` (`F/kde/tokens.toml:11255-11293`).
    The reader has no boolean parsing of its own to reuse: `grep` over
    `native-theme/src/kde/` finds none (its other `[KDE]` keys are numbers,
    `reduce_motion`, `kde/mod.rs:255-261`, and `frame_contrast`, `kde/colors.rs:445-450`);
    the nearest in the crate, `detect.rs:597`, reads GTK's `settings.ini` (`1` or
    `true`). KConfig's whole boolean syntax was not read (clean room; no KDE source
    here), so the tests pin only the forms the function accepts.
12. **7.D. Where `ChangeSelectionColor` resolves off, the reader writes the focused
    pair into both fields**, not `None`: the reader's output is laid over kde-breeze's
    (`kde/colors.rs:456-457`), so a `None` would keep the preset's Breeze tint (the
    rationale's amended D9 states it, `0004_kde-open-items-rationale.md`).
13. **7.E–7.G. The header's two fields and the inactive text colour reach an
    application through accessors named after their leaves** in the three connectors
    (`list_header_hover_background`, `list_header_active_background`,
    `selection_inactive_text_color`), the parity script's naming rule; iced has no
    `selection_inactive_text_color`, as it has no `selection_inactive_background`
    (`scripts/connector_parity_exceptions.toml:202-206`), with the same reason.
14. **7.H. The showcases' header sections take the fills per section**, as Breeze
    fills a section (`breezestyle.cpp:6828-6837`, the facts row): each of the two
    header cells is hovered and pressed on its own.
15. **6.H, 7.I. The backlog gets the entry the rationale's §5 promises** ("a model
    field for the accent a control is filled with"), written by step 6, tagged
    `[1.0]`: no step schedules it.

The next three the coordinating session settled on 2026-11-14, and the rationale's
amended D9 states them.

16. **7.H. Which showcase lists show the unfocused pair.** Options: (a) a list swaps to
    the inactive pair while its window is not active, only where the theme gives an
    inactive fill of its own; (b) every list swaps on every theme; (c) no list swaps,
    the pair is shown in the colour swatches and the Widget Info alone; (d) a list-level
    pair. **(a).** (b) would turn windows-11's faint list selection, #00000009 /
    #ffffff0f (stated in `windows-11.toml`'s `list`), into the accent #0078d4 its
    `defaults.selection_inactive_background` inherits; (c) would leave the pair unseen
    in any widget; (d) is a model change no decision contains. The test, for the
    resolved variant a showcase draws: `defaults.selection_inactive_background !=
    defaults.selection_background`. After step 7 it holds on 6 of the 34 bundled
    combinations (7.H.2).
17. **7.C, 7.D. macOS's text colour on the unfocused selection: `labelColor`.** Options:
    (a) `unemphasizedSelectedTextColor`, #000000 / #ffffff, the rationale's first
    reading; (b) `labelColor`. **(b).** The model's macOS selection pair is the content
    selection's: `selection_background` is `selectedContentBackgroundColor` with
    `alternateSelectedControlTextColor` (`native-theme/src/macos.rs:168-172`,
    `docs/platform-facts.md:1059-1061`), and `selection_inactive_background` is
    `unemphasizedSelectedContentBackgroundColor`. The text drawn on that fill is the
    row's label, which does not adapt to an unemphasized selection: the parked facts'
    note (`F/macos/roles.toml:11958-11963`, the fact `role.selection.foreground_inactive`
    at `:11964-11978`) and the probe's own readings of the label on that row (7.C.3).
    `unemphasizedSelectedTextColor` pairs with the unemphasized *text* selection's
    background, which this field does not hold.
18. **7.D. KColorScheme's other inactive effects: the reader follows D9's rule alone.**
    Options: (a) D9's rule alone, documented; (b) carry `kde_effects.rs` and apply its
    effects to the pair; (c) write neither field where `[ColorEffects:Inactive] Enable`
    is true. **(a).** The inactive group's contrast, intensity and colour effects
    recolour every colour of an inactive window (`P8 native-theme/src/platform/kde_effects.rs:43-62`),
    which the model does not hold at all; applied to the selection pair alone they would
    give a pair that no longer matches the rest of the window the model describes. (c)
    would leave kde-breeze's Breeze values standing under another scheme. The reader's
    documentation and the facts cell say that a scheme with `Enable=true` gets the pair
    before those effects, and a backlog entry records the inactive window's palette as
    one item (7.I.5).

19. **7.A, 7.H. A pressed header section with no pressed fill stated takes the hovered
    fill.** Options: (a) `header_active_background`, else `header_hover_background`,
    else `header_background`; (b) `header_active_background`, else `header_background`.
    **(a)**, D5's reasoning for a button ("A pressed button is under the pointer"): a
    pressed section is a hovered one too, and (b) would drop the hover fill under a
    press. No bundled preset tells the two apart today (kde-breeze states both, the
    others neither).

The next two the session settled on 2026-11-14 as well:

20. **6.A, 6.D-6.G. The default button's hovered and pressed frame**: one rule for the
    three connectors: `button.hover_border_color` where the theme states it, else the
    default button's frame (`default_button_border_color`, else `button.border.color`);
    in code, `b.hover_border_color.unwrap_or(frame)`.
21. **7.H. The window's state in tests and captures.** The showcases follow the real
    window state; nothing fakes it.
    - **gpui tests.** The harness `open_with` (`showcase-gpui/tests.rs:119-139`) stays
      as it is: its window is not active (the test platform's windows start inactive,
      7.H.2). Activating every test's window would change more than the selection: the
      title bar, which the showcase draws from `window.is_window_active()`
      (`showcase-gpui/app.rs:2396`, into `chrome::window_top`); the caret, which
      gpui-base blinks and paints only in an active window (gpui-base 0.9.1
      `input/base/state.rs:957-966`, `:3998-4003`); the selection inside text fields
      (`input/base/element.rs:3927`); and the window border's shadow (gpui-component
      0.9.1 `window_border.rs:200`, `:254`). So each test that reads the Basic table's
      selected row under a preset the rule selects makes its own window active, and each
      new test of the swap makes it inactive, both explicitly, with a comment saying
      why (7.H.2 lists them).
    - **iced and egui.** iced's state starts active (`window_active: true`) and follows
      the window's `Focused` and `Unfocused` events; iced 0.14 offers no query of the
      window's focus (iced_runtime 0.14.0 `window.rs` has `gain_focus`, `:431`, which
      takes the focus, and nothing that reads it). egui's follows `RawInput::focused`,
      whose default is `true` (egui 0.36.2 and 0.37.0 `src/data/input/raw_input.rs:106`),
      which `egui_kittest` keeps (it builds its input with `..Default::default()`,
      egui_kittest 0.36.2 `src/lib.rs:151-154`), so the egui tests see an active window;
      but egui-winit starts a real window at `focused: false` until winit reports focus
      (egui-winit 0.36.2 `src/lib.rs:149`, 0.37.0 `:163`, "winit will tell us when we
      have focus"). So the two showcases differ until the window's first focus event: a
      real iced showcase draws the active pair, a real egui showcase the unfocused one
      (where the rule selects it); after that event both follow the window.
    - **Captures.** During the release's asset run, before its assets are committed,
      the session checks the macOS captures for the unfocused fill in each toolkit's
      own selected row, read from the layout dump the macOS capture step writes beside
      each capture (a workflow edit of U7.3); where it finds it, the capture code makes
      the window active before it captures (7.H.2, 7.J.4), not the showcase.

The next three are small ones of this document's, like 1 to 15:

22. **7.H.2. A selected table or list row's Widget Info does not follow the window's
    state.** It names the unfocused pair beside the active one, as a row's info already
    names its hovered fill beside its resting one ("The info does not follow the
    pointer", `showcase-gpui/info/data.rs:417-421`): where the rule of 7.H.2 selects the
    theme, two more claims, "bg, window not active" `selection_inactive_background` and
    "text, window not active" `selection_inactive_text_color`. A claim that changed with
    the window would make an info opened by the pointer, which activates the window on
    a desktop, show only the active pair.
23. **7.F. iced's two header accessors are rows of `ACCESSOR_ROWS`, not unread
    entries.** `ACCESSOR_ROWS` is "Every colour accessor of the crate root and the native
    leaf it returns" (`connectors/native-theme-iced/src/contract/rows.rs:116-119`), which
    holds today for all nine colour accessors of `src/lib.rs` (`:2005-2077`, read today
    against the rows); a leaf that reaches a row and is also named in `UNREAD_COLOURS`
    fails the walk ("reaches … yet is named unread", `src/contract.rs:5622-5646`). The
    two leaves then count as reached by accessors (`by_accessors`, `:5624-5628`).
24. **6.D. The gpui default button's colours come from two functions**, as steps 4 and
    5's toolbar button's do (their B.9: `toolbar_button_states` and
    `toolbar_button_rest`): the public refinement `geometry::default_button_rest` for the
    rest fill and frame, which the caller applies, and a crate-visible
    `variants::default_button_states` for what the variant carries (the hovered fill,
    the label, the hovered and pressed frame), which `variants::default_button` and the
    contract's `sinks` read, since `sinks` has no `App` to build a variant with (3F.4's
    reason for `hover_border`). Each reads its fields at a line of its own, so the
    showcase's colour claims cite the lines that read them (6.G).

## Corrections to the rationale found while specifying

None remain. The rationale's amended D8 and D9 (re-read 2026-11-14) have taken in every
correction this document found: the reader writing both inactive fields, and why; the
reader writing no inactive fill today; macOS's label colour; the 16-bit frame mix; the
showcases' swap rule; the inactive effects left out; the gpui variant reaching a button
the application builds, a plain `Dialog`'s footer included, while an `AlertDialog`'s own
OK button stays the primary one (D8's *Connectors* bullet, "In gpui the variant reaches
a button the application builds …", `0004_kde-open-items-rationale.md:687-691` on
2026-11-15; 6.D.2 gives the sources); and gpui's `ListItem` sample, which cannot swap
(D9, "One sample cannot swap: the gpui showcase's list built on gpui-component's
`ListItem` …", `:749-751` the same day; 7.H.2 gives the sources).

## Open points for the maintainer

1. **The version (D0).** Written for D0 (a): the release of both steps is `0.7.N`, its
   CHANGELOG section has one *Breaking Changes* entry naming every struct the two steps
   give a field (6.H.1, the break named once, S5). Under (b), the release is a minor
   version: the heading `## [0.N.0]` instead, the same entry. Under (c), the release
   that makes the model's input structs `#[non_exhaustive]` comes first; then the
   *Breaking Changes* entry of 6.H.1 is dropped. The struct
   literals 6.A.2 and 7.A.2 extend then differ: the `widgets/mod.rs` test is inside the
   crate and keeps its struct expression; `native-theme/tests/proptest_roundtrip.rs` is
   outside it (the `tests/` folder builds against the public API), where a struct
   expression of a `#[non_exhaustive]` struct does not compile, with or without
   `..Default::default()`, so under (c) that release rewrites its literals as
   `Default::default()` followed by field assignments, and these steps add assignments.
2. **The asset run** needs the maintainer's desktop unlocked and untouched for 4 to 6
   minutes (the first plan's §8 measurement), once for the release of both steps,
   after its last change. Step 6's layout measurement (6.G) needs the same, twice, for its "before" and
   "after" blocks.
3. **The upstream request where step 3's trial passed and 6.D.4's test fails** (6.D.2,
   case 3). Request 5 does not exist then: 3F.4 adds no request. 3F.5's text cannot be
   added as written: it says that "a theme cannot give a button a hover outline of its
   own" and that a custom variant "cannot stand in", which a passed trial disproves
   (3F.5, *Upstream request 5*, `docs/todo_v0.7.x_kde-open-items-steps2-3-spec.md`).
   Settled by the coordinating session: case 3 stops the work before its push. A
   passed trial with a failed 6.D.4 means the custom variant carries a hover outline
   but not a state this step needs, which no part of this specification foresees; the
   request's text would rest on that state, read only at the run. So the implementer
   reports the test's output (the state and the two colours) and the maintainer
   decides whether a request is written and what it says; the no-receiver branch
   (6.D.2) is applied meanwhile, so the tree stays green.

Settled by the coordinating session, and so not open: the showcase lists (decision 16),
macOS's text colour (decision 17), KColorScheme's other inactive effects (decision 18),
the default button's hover frame and the window's state (decisions 20 and 21); the
reply's edit applies while it is a draft (7.I.3).

## Part F — Facts rows and preset lines: placement

Columns of the facts tables: `Property | macOS | Windows | KDE | GNOME | Material`
(headers `:1048`, `:1175`, `:1419`).

### F.1 The rule, the same for every step

A new row of `docs/platform-facts.md` goes beside the row it belongs with; rows are
inserted only, and no blank line is dropped. The citations of lines the rows move are left as they are
(the batch's S6): nothing re-points them, and the three gates that read the facts file
by line position (`native-theme/tests/platform_facts_xref.rs`,
`native-theme/src/presets/documented_sizes.rs`, `scripts/check_preset_citations.py`)
stop using line positions from v0.7.1 on (its LN unit: the two Rust checks test that
the facts file contains the text, and the preset checker compares nothing with the
facts file any more), so a row added above their lines breaks none of them. A
citation this document has a unit write names the facts file and the row
(`docs/platform-facts.md` §2.3, `button.default_button_background`), never its line
number; a preset comment takes the batch's form, `# platform-facts.md (§2.3): …`
(6.C.3).

**This document's three insertions** (decision 1; numbers at `0fa28027`). The releases
before step 6 insert rows above and below them (the header), so each anchor row is
found by its text on the tree the earlier releases left:

| Step | Rows | After |
|---|---|---|
| 6 | 3 | `primary_text_color` (§2.3), `:1189` at `0fa28027` |
| 7 | 1 | `selection_inactive_background` (§2.1.3), `:1061` |
| 7 | 2 | `header_height` (§2.15's; the name is also a row of the expander's table), `:1432` |

### F.2 Preset lines

6.C.3, 7.D.3 and 7.D.4 insert lines into `kde-breeze.toml`, `macos-monterey.toml` and
`macos-tahoe.toml`. The citations of preset lines they move are left as they are (S6);
no gate reads them outside `docs/todo.md`, where `check_todo_citations.py` fails a
preset line number on a line that names no commit, and the preset line citations there
are each dated (searched on 2026-11-14 with
`git grep -nE '(kde-breeze|macos-monterey|macos-tahoe)\.toml(:|#L)[0-9]'`:
`docs/todo.md:20`, `:433`, `:439`, `:1165` "at 2ca74471" and `:4413` "at de950afd").
The releases before step 6 insert lines into the same presets first; each edit is
placed by its anchor text on the tree U6.1 starts from (and for step 7 on step 6's).

---

# Step 6 — the default button (D8)

## 6.A The model

### 6.A.1 `native-theme/src/model/widgets/mod.rs`, `ButtonTheme`

After the struct's last soft option before `font` (`:98`): `checked_text_color`
(`:94-97`) at `0fa28027`, step 3's `focus_border_color` on its tree (its 3A.1 puts
`hover_border_color` and `focus_border_color` after `checked_text_color`). Three soft
options in the form of the struct's other optional colours
(`#[theme(category = "soft_option")]`, `:81-97`):

```rust
    /// Fill of a dialog's default button, the one Enter activates, where the
    /// platform gives it a look of its own: Breeze's `Button` mixed a fifth
    /// towards `Highlight`. `None` where the default button is the primary
    /// one (macOS, Windows): a connector then draws `primary_background` and
    /// `primary_text_color` in every state. Where it is stated, the default
    /// button is this fill, lettered in `default_button_text_color` (else
    /// `font.color`) and framed in `default_button_border_color` (else
    /// `border.color`): the fill decides (docs/platform-facts.md §2.3).
    /// Breeze tints its default button only in an active window; the
    /// connectors draw this look whatever the window's state.
    #[theme(category = "soft_option")]
    pub default_button_background: Option<Rgba>,
    /// Label colour of a dialog's default button, read only where
    /// `default_button_background` is stated; `None` there is `font.color`,
    /// as Breeze letters it (docs/platform-facts.md §2.3).
    #[theme(category = "soft_option")]
    pub default_button_text_color: Option<Rgba>,
    /// Border colour of a dialog's default button, read only where
    /// `default_button_background` is stated; `None` there is `border.color`
    /// (docs/platform-facts.md §2.3).
    #[theme(category = "soft_option")]
    pub default_button_border_color: Option<Rgba>,
```

The doc's sentence on the window's state rests on Breeze's own condition, "`} else if
(isActiveWindow && defaultButton) {`" (breezehelper.cpp:700-702 at breeze 6.8.2, as the
parked facts quote it, `F/kde/button.toml:1342`, each default-button fact under `when
= { not_conditions = ["window_inactive"], … }`): in an inactive window that branch is
not taken (which later branch draws the button then was not read here). The model has
no field for that state and the connectors
do not read the window's activity for a button, so they draw the default look in either
state; 6.H.5's item 3 records it in the backlog.

The derive gives `ResolvedButtonTheme` three `Option<Rgba>` fields
(`native-theme-derive/src/gen_structs.rs:38`, soft options stay `Option`), registers
them in `FIELD_NAMES` and merges them as `Option`s (`gen_merge.rs:38`, `:68`). The
resolved struct is `#[non_exhaustive]` (the derive's output, as the 0.7.0 CHANGELOG
states, `CHANGELOG.md:67`), so adding to it breaks nothing; `ButtonTheme` is not, which
is D0's break.

**"The fill decides", exactly.** For a resolved theme `r`, with `b = r.button`:

- `b.default_button_background` is `Some(fill)`: the default button at rest is `fill`,
  its label `b.default_button_text_color.unwrap_or(b.font.color)`, its frame's colour
  `b.default_button_border_color.unwrap_or(b.border.color)`, the frame's width, radius
  and padding `b.border`'s. Hovered, the same fill and label, the frame
  `b.hover_border_color.unwrap_or(frame)` (decision 20; step 3's field, where frame is
  the rest frame above). Pressed, the plain button's pressed fill and label, the frame
  as hovered. Disabled, the plain button's disabled look (decision 3).
- `b.default_button_background` is `None`: the default button is the connector's
  primary button in every state. `default_button_text_color` and
  `default_button_border_color` are not read, whatever they hold; a theme that states
  them without the fill is valid (no rule between fields, D8) and draws the primary
  button.

### 6.A.2 Struct literals that name every field

Inside the crate, a struct literal without `..Default::default()` stops compiling:

- `native-theme/src/model/widgets/mod.rs:1833-1868`, `button_validate_widget_extracts_all_fields`:
  add `default_button_background: Some(Rgba::rgb(210, 230, 245))`,
  `default_button_text_color: None`, `default_button_border_color: None` after
  `checked_text_color: None` (`:1848`; after step 3's two fields on its tree), and after
  the existing `assert_eq!(resolved.checked_text_color, None)` (`:1879`; after step 3's
  asserts) the three asserts
  `Some(Rgba::rgb(210, 230, 245))`, `None`, `None`. (The colour is a test literal, not
  a theme value.)
- `native-theme/tests/proptest_roundtrip.rs`, `arb_button_theme` (`:226-259` at
  `0fa28027`): the colour vector grows by three and the three fields take the next three
  indices, on the tree U6.1 starts from, where `arb_button_theme` is as step 3 left it
  (no later release's specification names it) (its 3A.2 makes the vector `13..=13` with
  `colors[11]` and `colors[12]`, so these take `colors[13]`, `colors[14]`, `colors[15]`
  and the vector becomes `16..=16`); the strategy's comment, which step 3 corrects to
  the count it has (its 3A.2), is corrected again. Every other full struct literal of
  `ButtonTheme` (found today with a scan of `git grep -nE '(^|[^A-Za-z])ButtonTheme \{'`
  over `*.rs` for literals without `..`): only `widgets/mod.rs:1833` above and this one.

### 6.A.3 `native-theme/src/presets.rs`, `platform_presets_no_derived_fields`

In the test, the comment above the `button.primary_background` assertion (`:423-425` at
`0fa28027`), found by its words "KDE's default button": its clause from "KDE's default
button is mix(Button, Highlight, 0.2)" to the end of the comment, which ends with a
facts line number ("(docs/platform-facts.md:1188, §2.3)" at `0fa28027`, whatever it
then reads), becomes "KDE's default button has fields of its own, `default_button_*`
(docs/platform-facts.md §2.3)", whatever that number then is. After that
assertion, a new one: `variant.button.default_button_background.is_some() == (*name == "kde-breeze")`,
message "preset '{name}' {label}.button.default_button_background: stated only where the
default button is not the primary one (KDE)".

### 6.A.4 Every gate a new model field passes

| Gate | What it checks | This step |
|---|---|---|
| `property_registry_keys_match_derive_inventory` (`native-theme/src/model/mod.rs:2055`) | the registry's keys equal the derive's `FIELD_NAMES` per struct | 6.B adds the three keys |
| `scripts/check_platform_facts_rows.py` | every registry field has a facts row in its section | 6.B adds the rows to §2.3 |
| `no_inheritance_fields_are_not_inherited` (`resolve/inheritance.rs:816`) | no `[no_inheritance]` field is an inheritance target | the three are listed there and inherit nothing |
| `every_bundled_preset_source_lints_clean` (`presets.rs:211`) | every preset key is a model key | the new kde-breeze keys exist once 6.A lands |
| `scripts/check_preset_citations.py`, `check_section_refs.py`, `check_wrong_safety_nets.py`, `check_todo_citations.py` | the preset comments' citation form (under v0.7.1's LN unit, GPUI Kit specification Part I.3, nothing is compared with the facts rows); section references; the safety-net marks; backlog key paths | 6.B, 6.C, 6.H |
| egui: `every_leaf_has_exactly_one_row_and_every_row_is_well_formed`, `verdict_totals_match_the_specification`, `t2_every_row_s_declared_sinks_are_exactly_what_its_leaf_moves` (`mapping_tests.rs`) | one mapping row per leaf; §5.7's totals; what a leaf moves | 6.F |
| gpui: `every_colour_leaf_reaches_a_sink_or_is_named` (`contract.rs:2933`) | each colour leaf reaches a sink or is named | 6.D |
| iced: `every_colour_leaf_reaches_a_row_or_is_named` (`contract.rs:5578`) against `contract-leaves.toml` | the same, by rows | 6.E |
| `scripts/check_connector_parity.py` | crate-root functions alike in the three | 6.F's accessor, two exceptions |
| `scripts/check_showcase_parity.py --check-list` and the static parity modes | the shared element list | 6.G's element |

## 6.B Registry, rules, facts

### 6.B.1 `docs/property-registry.toml`, `[button]` (`:180-198`)

After `checked_text_color = "color"` (`:196`) at `0fa28027`, after step 3's
`focus_border_color = "color"` on its tree (its 3B.2 puts its two keys after
`checked_text_color`), so that the keys stand in the struct's order:

```toml
default_button_background    = "color"
default_button_text_color    = "color"
default_button_border_color  = "color"
```

### 6.B.2 `docs/inheritance-rules.toml`

In `[no_inheritance]` `fields`, after `"button.checked_text_color", …` (`:412`) at
`0fa28027`; on the tree U6.1 starts from, after step 3's two entries there,
`"button.hover_border_color"` and `"button.focus_border_color"` (its 3B.3 puts them
directly after `"button.checked_text_color"`), so that the button's entries stand in
the struct's order, as 6.B.1's keys do:

```toml
    "button.default_button_background",  # None: the default button is the primary one (§2.3)
    "button.default_button_text_color",  # None where the fill is stated: button.font.color (§2.3)
    "button.default_button_border_color", # None where the fill is stated: button.border.color (§2.3)
```

`[uniform]`'s `button.primary_background` and `primary_text_color` rules (`:164-165`)
stay: the pair keeps meaning the accent-filled button on every platform, KDE included
(D8). Neither gets a `wrong_safety_nets` entry.

### 6.B.3 `docs/platform-facts.md` §2.3

**The two primary rows' KDE cells** (`:1188`, `:1189`, fourth cell), which v0.7.1
pointed at the default button, become (D8: the KDE cell moves to the new rows, and the
pair states what it means):

- `primary_background`: `← \`defaults.accent_color\`: Breeze has no primary-button variant; a default button has a look of its own (\`default_button_background\`), and the field is the accent-filled button`
- `primary_text_color`: `← \`defaults.accent_text_color\`: as \`primary_background\``

**Three rows directly after `primary_text_color`**, by Part F: `:1189` at `0fa28027`,
found by its text on the tree U6.1 starts from (the header: the earlier releases insert
rows above it), before `disabled_opacity`, no blank line dropped (F.1). The two primary
rows' cells above are found the same way (`:1188-1189` at `0fa28027`). The KDE cells
take the text the two primary cells hold today, read today at `:1188` and `:1189` (their
links copied as they are); the macOS and Windows
cells cite the parked facts. The os-probe run they rest on, `2026-10-30-run-41006842699`,
is not in this tree, and not in the parked branch's tree either since `47b3f8c1`
("docs/evidence: removed from this repository … the files live in
native-theme-evidence"); the cells cite the facts file, as 6.B.3's text says.

1. `default_button_background`
   - macOS: `← \`primary_background\`: the default push button is the accent-filled one; on macOS 26 the facts measure its fill as the accent (\`docs/facts/macos/button.toml:13711-13714\` on \`v0.8.0-rc1\` at \`78ee0efd\`, os-probe run 41006842699) · macOS 12–15: not read`
   - Windows: `← \`primary_background\`: a \`ContentDialog\`'s default button takes \`AccentButtonStyle\` (ContentDialog_themeresources.xaml:196-198, :201-203, :206-208 and Button_themeresources.xaml:5, :103 at microsoft-ui-xaml 2.5.1, \`docs/facts/windows/button.toml:10431-10474\` on \`v0.8.0-rc1\` at \`78ee0efd\`)`
   - KDE: `**(Breeze src)** the default button of an active window is filled with \`KColorUtils::mix(Button, Highlight, 0.2)\` in place of \`palette.button()\` ([breezehelper.cpp:700-702](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L700-L702)) — Breeze #d6ecf8, Breeze Dark #2d4655 **(measured)** ([out_light.txt:158, :159](evidence/breeze-offscreen/2026-11-13/out_light.txt#L158), [out_dark.txt:158, :159](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L158))`
   - GNOME: `**(none)** — no default-button state: a suggested action is a variant, \`primary_*\`; the parked facts state none (\`docs/facts/gnome/button.toml\` on \`v0.8.0-rc1\` at \`78ee0efd\`, no fact with \`states = ["default"]\`, searched 2026-11-14)`
   - Material: `**(none)** — as GNOME (\`docs/facts/material/button.toml\`, searched the same)`
2. `default_button_text_color`
   - macOS: `← \`primary_text_color\`: the default push button's label is #ffffff on macOS 26 (\`docs/facts/macos/button.toml:18740-18743\` on \`v0.8.0-rc1\` at \`78ee0efd\`) · macOS 12–15: not read`
   - Windows: `← \`primary_text_color\`: \`AccentButtonStyle\`'s (as \`default_button_background\`)`
   - KDE: `← \`font.color\`: the default button's label is \`ButtonText\`, as every non-flat button's ([breezestyle.cpp:5350-5362](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L5350-L5362)) — Breeze #232629, Breeze Dark #fcfcfc **(measured)** ([out_light.txt:159](evidence/breeze-offscreen/2026-11-13/out_light.txt#L159), [out_dark.txt:159](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L159))`
   - GNOME, Material: `**(none)** — as \`default_button_background\``
3. `default_button_border_color`
   - macOS: `← \`primary_*\`: the default button is the primary one; its frame was not read`
   - Windows: `← \`primary_*\`: \`AccentButtonStyle\`'s (as \`default_button_background\`)`
   - KDE: `**(Breeze src)** \`KColorUtils::mix(Highlight, mix(Button, ButtonText, frameContrast), 0.5)\` ([breezehelper.cpp:700-702](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L700-L702)) — Breeze #87c0dd, Breeze Dark #4882a1 **(measured)** ([out_light.txt:158](evidence/breeze-offscreen/2026-11-13/out_light.txt#L158), [out_dark.txt:158](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L158))`
   - GNOME, Material: `**(none)** — as \`default_button_background\``

The measured lines, read today: `out_light.txt:158` gives the frame #87c0dd and the fill
#d6ecf8 (`row y=17 x=0..7`), `:159` the fill sample #d6ecf8 and the label #232629;
`out_dark.txt:158-159` the same with #4882a1, #2d4655 and #fcfcfc. The macOS facts read
today: `F/macos/button.toml:13711-13714`, `key = "button.body.fill"`,
`when = { conditions = ["preset:macos-tahoe"], states = ["default"] }`,
`value = "= input.accent"`; `:18740-18743`, `button.label.text.fill`, the same `when`,
`value = "#ffffffff"`. No fact for a macos-monterey default fill or for a macOS default
frame was found by key and `when` (searched today).

### 6.B.4 What the checkers require

`check_platform_facts_rows.py`: the three keys under `[button]` (§2.3) find rows of
their name in §2.3. `check_section_refs.py`: the `[no_inheritance]` comments cite §2.3,
which exists. `check_preset_citations.py`: under v0.7.1's LN unit (the GPUI Kit
specification's Part I.3) it reads 6.C.3's comments in the batch's form and compares
nothing with the facts rows. `check_wrong_safety_nets.py`: no entry or mark changes.

## 6.C kde-breeze and the KDE reader

### 6.C.1 The values

| Field | Light | Dark | How computed today |
|---|---|---|---|
| `default_button_background` | #d6ecf8 | #2d4655 | `mix(Button, Highlight, 0.2)`: Button #fcfcfc / #292c30, Highlight #3daee9, by the reader's `mix` (`kde/colors.rs:234-256`) re-done in Python with numpy `float32` (`q_round` as `(x + 0.5) as u16`, `div_257`); equal to the measurement |
| `default_button_text_color` | #232629 | #fcfcfc | `[Colors:Button] ForegroundNormal` |
| `default_button_border_color` | #87c0dd | #4882a1 | `mix(Highlight, mix(Button, ButtonText, 0.2), 0.5)`, the inner mix kept in 16 bits; equal to the measurement. With the inner mix rounded to 8 bits: #87c0de / #4882a1 |

The emulation was checked against values the reader's tests already pin: it gives
`mix(Button, Highlight, 0.3)` #c3e5f6 / #2f5368 and `mix(Button, ButtonText, 0.125)`
#e1e1e2 / #43464a (kde-breeze `[light.button]` `active_background`, `checked_background`
and their dark twins) and step 1's menu panel #f4f5f5 / #1c1f22. Breeze's schemes state
no `frameContrast`, so the reader's default 0.2 applies (`kde/mod.rs:1215-1219`,
`BREEZE_LIGHT_SCHEME`'s doc).

### 6.C.2 `native-theme/src/kde/colors.rs`

**Constants**, after `PRESSED_BUTTON_MIX` (`:369`) and step 1's `MENU_PANEL_MIX`, each
in the file's form of one constant per use with its citation:

```rust
/// How far an active window's default button moves from `Button` towards
/// `Highlight`: `KColorUtils::mix(palette.button().color(), highlightColor,
/// 0.2)` (breezehelper.cpp:700-702 at breeze f0b1d75; docs/platform-facts.md
/// §2.3).
const DEFAULT_BUTTON_MIX: f64 = 0.2;

/// How far the default button's frame moves from `Highlight` towards the
/// plain frame, `mix(Button, ButtonText, frameContrast)`: 0.5
/// (breezehelper.cpp:700-702 at breeze f0b1d75; docs/platform-facts.md §2.3).
const DEFAULT_BUTTON_FRAME_MIX: f64 = 0.5;
```

One name for the plain frame's bias: `frameContrast`, the colour scheme's `[KDE]
frameContrast` that the reader reads as `frame_contrast` (`kde/colors.rs:445-450`) and
that every other place of this document, the facts row and the preset comment use.
Breeze's source spells the call `frameIntensityBias()`, which returns the helper's
`_contrastBias`, set from `KColorScheme::frameContrast()` (as the parked facts quote it:
breezehelper.cpp:700-702 and :104-105 at breeze 6.8.2, `F/kde/button.toml:1342`,
`:1358`); the comments name it once, as here, and say `frameContrast` everywhere else.

**`mix_qt` split (decision 4).** `mix_qt` (`:238-256`) keeps its signature and doc and
becomes `mix_qt_colors(QtColor::from_rgba(a), QtColor::from_rgba(b), bias)`. A new
`fn mix_qt_colors(a: QtColor, b: QtColor, bias: f64) -> QtColor` takes its body from
`if bias.is_nan()` on, unchanged, with the doc "`KColorUtils::mix(a, b, bias)` for
opaque colours already in `QColor`'s storage: [`mix_qt`]'s arithmetic, for a colour
mixed from another mix without rounding it to 8 bits, as Qt keeps it." The comment
"Both alphas are 1" stays true: both arguments come from opaque 8-bit colours.

**`populate_colors`**, after the hovered button's two lines (`:540-541`) and after the
two lines step 3 adds there (its Part 3C: `hover_border_color` and
`focus_border_color`, with their comment), using step 2's `selection_bg` binding (the
steps 2–3 specification, `populate_colors`: `[Colors:Selection] BackgroundNormal`,
bound once beside `view_focus`), so no second read of the key:

```rust
    // The default button of an active window (docs/platform-facts.md §2.3): the
    // button colour moved a fifth towards the highlight, framed in the
    // highlight moved halfway towards the plain frame, `mix(Button,
    // ButtonText, frameContrast)`, kept at Qt's precision, and lettered in the
    // button text (breezehelper.cpp:700-702 and breezestyle.cpp:5350-5362 at
    // breeze f0b1d75).
    variant.button.default_button_background = variant
        .button
        .background_color
        .zip(selection_bg)
        .map(|(button, highlight)| mix(button, highlight, DEFAULT_BUTTON_MIX));
    variant.button.default_button_border_color = variant
        .button
        .background_color
        .zip(button_fg)
        .zip(selection_bg)
        .map(|((button, text), highlight)| {
            let frame = mix_qt(button, text, frame_contrast(ini));
            mix_qt_colors(QtColor::from_rgba(highlight), frame, DEFAULT_BUTTON_FRAME_MIX).to_rgba()
        });
    variant.button.default_button_text_color = button_fg;
```

`populate_colors`' doc (`:459-470`) gains "the default button's fill and frame" in its
list of computed colours. A missing input gives `None` and the preset's value stands,
as every derivation of the reader.

### 6.C.3 `native-theme/src/presets/kde-breeze.toml`

`[light.button]`: the comment `:148-151` ("# primary_background / primary_text_color not
stated: …") is replaced by, in its place, in the batch's preset comment form, today's
with the line number removed (S6: `# platform-facts.md (§2.3): …`, and for a key not
given `# K not stated: platform-facts.md (§2.3), …`; v0.7.1's LN unit makes
`scripts/check_preset_citations.py` read this form, and it compares nothing with the
facts file any more):

```toml
# primary_background / primary_text_color not stated: platform-facts.md (§2.3), Breeze has no primary variant; the
# pair inherits the accent and its text, the accent-filled button
# platform-facts.md (§2.3): default_button_background, KColorUtils::mix(Button #fcfcfc, Highlight #3daee9, 0.2), an
# active window's default button (breezehelper.cpp:700-702 at breeze f0b1d75; measured)
default_button_background = "#d6ecf8"
# platform-facts.md (§2.3): default_button_text_color, [Colors:Button] ForegroundNormal 35,38,41 (BreezeLight.colors:39,
# breeze v6.7.5)
default_button_text_color = "#232629"
# platform-facts.md (§2.3): default_button_border_color, mix(Highlight #3daee9, mix(Button, ButtonText, frameContrast
# 0.2) #d1d1d2, 0.5) (breezehelper.cpp:700-702 at breeze f0b1d75; measured)
default_button_border_color = "#87c0dd"
```

`[dark.button]` (`:733-736`): the same, with `#2d4655`, `(BreezeDark.colors:40, breeze
v6.7.5)` and ForegroundNormal `252,252,252`, `#fcfcfc`, `#292c30`, `#535659` and
`#4882a1`. The line numbers of `BreezeLight.colors:39` and `BreezeDark.colors:40` are the
file's own (`kde-breeze.toml:161`, `:746`), pinned sources: breeze at a version. The blank lines after the block stay. Of the
steps before this one only step 3 adds keys to `[light.button]` (and `[dark.button]`):
its `hover_border_color` and `focus_border_color` with their comment, after
`checked_text_color`, so directly before the comment this block replaces; step 5's keys
go to `[light.toolbar]` and `[dark.toolbar]`. The block stands in the comment's place,
after step 3's keys.

### 6.C.4 Tests, `native-theme/src/kde/mod.rs` and `kde/colors.rs`

| Test | Change |
|---|---|
| `derived_leaves`, `resolved_leaves` (`kde/mod.rs:1246-1372`, today 29 leaves, 31 after step 1, more after steps 2 to 5) | append `("button.default_button_background", …)`, `("button.default_button_text_color", …)`, `("button.default_button_border_color", …)`; the resolved side reads `theme.button.default_button_*` (already `Option`s); the array's length and the doc's count grow by three |
| `the_derived_colours_are_the_presets_under_breeze` (`:1397-1414`) | passes with 6.C.2 and 6.C.3, no edit |
| new, `kde/colors.rs`, after the pressed-button test (`the_pressed_hovered_and_focused_colours_come_from_their_keys`, `:908-930`, whose `:915-918` asserts `active_background`): `test_default_button_is_breezes` | Breeze Light's Button, ButtonText and Selection keys give #d6ecf8, #232629, #87c0dd; Breeze Dark's #2d4655, #fcfcfc, #4882a1 |
| new: `test_default_button_frame_keeps_qts_precision` | the light frame is `Rgba::rgb(0x87, 0xc0, 0xdd)`, and `mix(mix(..))` through 8 bits would be `0xde` in blue: the test computes that and `assert_ne!`s it, so a refactor that rounds early fails |
| `test_full_mapping_completeness` (`kde/colors.rs:1713-1861`) | asserts the three `is_some()` (the `BREEZE_DARK` fixture has Button and Selection, `:770-830`) |

The tests that compare an exact list of moved leaves (`moved_leaves`, `kde/mod.rs:1384-1393`,
in `resolved_leaves`' order, so a new leaf joins each list at the end, after what the
earlier steps appended), found by reading every `assert_eq!(moved_leaves(…)` of
`native-theme/src/kde/mod.rs` (`:1420-1581`; no other file of `native-theme/src/kde/`
or `native-theme/tests/reader_kde.rs` compares an exact list or count of derived or
moved leaves: `grep -n 'moved_leaves\|derived_leaves\|len()'` over them), each with what
step 6 adds, read from what each leaf reads (6.C.2):

| Test | Its scheme | Gains |
|---|---|---|
| `an_accent_moves_the_on_track_the_pressed_button_the_selected_segment_and_the_list_hover` (`:1420-1438`) | `selection: "233,84,32"` | `button.default_button_background`, `button.default_button_border_color` (both read `[Colors:Selection] BackgroundNormal`) |
| `the_button_set_moves_the_button_slider_switch_and_header_leaves` (`:1441-1465`) | `button: ("200,180,150", "60,20,10")` | the three (fill and frame read the Button background, the text and frame its text) |
| `the_header_set_moves_the_toolbar` (`:1470-1480`) | Header set | none |
| `the_window_set_moves_the_popover_edge` (`:1486-1522`), both schemes | Window set | none |
| `the_view_set_moves_the_card_edge` (`:1525-1532`, renamed by step 1) | View set | none |
| `the_view_decorations_move_the_frames_the_tab_hover_and_the_handle` (`:1538-1561`), hover and focus | decorations | none |
| `the_frame_contrast_moves_the_edges` (`:1564-1581`) | `frameContrast` 0.5 | `button.default_button_border_color` (its inner mix) |

`a_custom_accent_reaches_the_accent_fills` (`kde/colors.rs:850-902`) asserts only that
three named fields moved and fails nothing when a leaf is added: no edit.

## 6.D gpui connector

### 6.D.1 What draws a default button in gpui today

gpui-component's dialog builds its OK button itself, `ButtonVariant::Primary` unless the
caller passes another (`GC/dialog/dialog.rs:159`, `ok_variant.unwrap_or(ButtonVariant::Primary)`;
`DialogButtonProps::ok_variant`, `:66-69`). `Primary` fills `button_primary` and letters
`button_primary_foreground` (`GC/button/button.rs:1021`, `:1035-1041`, `text_color`'s `Primary` arm), which the
connector sets from `primary` and `primary_foreground` (`connectors/native-theme-gpui/src/colors.rs:388-391`),
that is `button.primary_background` and `primary_text_color` (`:203-204`); its frame is
`primary` (`button.rs:1088`). So today a gpui dialog's default button is the primary
button on every preset, kde-breeze's #3daee9 under #ffffff / #fcfcfc included. Nothing
in the connector reads the new fields; `primary` keeps its source (D8: "No existing
reader changes").

### 6.D.2 The variant, by step 3's trial

D8: "gpui a variant for a dialog's confirming button, which is a custom variant and so
rests on D5's trial for its border". Step 3's Part 3F builds that trial:
`variants::bordered_button(n, cx) -> Option<ButtonCustomVariant>` (3F.1), checked by
`bordered_button_matches_the_default_button_but_its_hover_border` against
gpui-component's Default button state by state (3F.2), with a strict criterion (3F.3:
every compared item equal but the hover and press border). Its construction: the
variant's `color` is the border of every hovered and pressed state, `hover` and
`active` the fills there (`GC/button/button.rs:1118-1124`, `:1209`, `:1294`), its
`foreground` the label in every state (`:1080`); the rest fill and rest frame the
variant cannot give (its own rest fill is a fifth of `color`, `:1028`) come from the
caller's refinement, `.refine_style(&geometry::button(n))` and `.bg(..)`, which upstream
replays at rest and its hover and press styles replace (`:730-764`).

Because the construction needs the caller's refinement, it reaches a button the
application builds, and not one gpui-component builds itself. In a dialog: a plain
`Dialog` renders no buttons from `button_props`, and its footer is the application's
own (`Dialog::footer`, `GC/dialog/dialog.rs:413-424`, whose doc says so), so a
`variants::default_button` button goes there like any other; an `AlertDialog`'s
default footer builds its OK button from a `ButtonVariant` alone (`render_ok`,
`:149-162`, `ok_variant.unwrap_or(ButtonVariant::Primary)`; `DialogButtonProps::ok_variant`,
`:66-69`), so in every case below that button stays `Primary`, and the README says so.

**One condition for every gpui branch of step 6.** *gpui has no receiver* (for the
default button's look) means: step 3's trial failed (3F.5), or it passed and 6.D.4's
test fails by 3F.3's criterion. *gpui has a receiver* means: the trial passed and
6.D.4's test passes. 6.D.3, 6.D.5, 6.F.5, 6.G, 6.H and 6.I read this condition, never
the trial's outcome alone. Its three cases:

1. **The trial failed.** Nothing of the first bullet below is built; the second bullet
   applies, and request 5, which 3F.5 adds, gains the sentence that bullet gives.
2. **The trial passed and 6.D.4's test passes.** The first bullet applies.
3. **The trial passed and 6.D.4's test fails.** The first bullet's functions are
   written and tested, then removed with their tests, their `GEOMETRY_NOTES` entry,
   their `sinks` entries and their README text, before the unit's commit; the second
   bullet applies. Request 5 does not exist (3F.4 adds no request), so the second
   bullet's request-5 sentence has nothing to join. The work then stops before the
   unit's push and the maintainer is told the state 6.D.4's test found different and
   its two colours; whether a request is written, and its text, is the
   maintainer's (open point 3). The commit message and the plan's status line name that state, as 3F.5 records
   the trial's.

- **Where step 3's trial passed** (built in cases 2 and 3, kept in case 2 alone, where
  gpui has a receiver), in `variants.rs` after the last function steps 3 and 5
  add there and before the test module (`#[cfg(test)]`), so that no line their claims
  cite moves ("Lines that must not move"), in `bordered_button`'s form:

  ```rust
  /// A dialog's default button where the theme gives it a look of its own
  /// (`button.default_button_background`, KDE): `None` where it does not, and the
  /// caller builds the primary button (`.primary()`), as macOS and Windows draw it.
  #[must_use]
  pub fn default_button(n: Native<'_>, cx: &App) -> Option<ButtonCustomVariant>
  ```

  Where the fill is stated: `ButtonCustomVariant::new(cx)` with `.color(s.hover_frame)`
  (the frame of every hovered and pressed state, 6.A.1), `.foreground(s.foreground)`,
  `.hover(s.hover)` (the default fill: Breeze keeps it under the pointer),
  `.active(cx.theme().button_active)` (the plain button's pressed fill, which `colors.rs`
  fills from `button.active_background`) and `.shadow(false)`, `s` being
  `default_button_states(n)`'s value; `None` where that is `None`. Its doc carries a
  `rust,no_run` example in the form of `ghost_button`'s (`CG/variants.rs:40-49`): the
  caller's `.custom(v)`, `.refine_style(&geometry::button(n))` and, for an enabled
  button, `.refine_style(&rest)` with `rest` from `geometry::default_button_rest(n)`; a
  disabled one takes `geometry::button_disabled(n)` in place of `rest`, so that its
  border stays `geometry::button`'s `button.border.color` (decision 3); unlike 3F.1's
  disabled button, which takes its rest fill and then `button_disabled` (whose fill
  replaces it), it must not take `rest`, whose frame `button_disabled` would not
  replace: `button_disabled` sets a fill, a text colour and an opacity and no border
  colour (`CG/geometry.rs:1344-1352`), and upstream replays a refinement in the
  disabled state too (steps 4 and 5's B.9, citing `GC/button/button.rs:821-838`).
  A custom variant has one foreground for every state (`GC/button/button.rs:1080`), so
  gpui's pressed default button keeps `default_button_text_color` (else `font.color`)
  where decision 3 gives the plain button's pressed label, `button.active_text_color`,
  as iced's and egui's do. Under kde-breeze the two are equal: `active_text_color` is
  unstated there and inherits `button.font.color` (`docs/inheritance-rules.toml:170`),
  #232629 / #fcfcfc, and `default_button_text_color` is the same colour (6.C.1). On a
  theme that states the two apart, gpui's pressed label differs; the variant's doc and
  its HOW line say so.
  - **What the variant carries** (decision 24), in `variants.rs` before `default_button`
    (both after the functions steps 3 and 5 add and before the test module), as steps
    4 and 5's B.9 puts `ToolbarButtonStates` and `toolbar_button_states` before
    `toolbar_button`: the claims of 6.G cite the lines of `default_button_states` that
    read the fields, and an edit of `default_button`'s doc would move them if they lay
    below it. In the form of steps 4 and 5's `ToolbarButtonStates`: `pub(crate) struct
    DefaultButtonStates { pub hover: Hsla, pub foreground: Hsla, pub hover_frame: Hsla }`
    (`Debug`, `Clone`, `Copy`, `PartialEq`) and `pub(crate) fn default_button_states(n:
    Native<'_>) -> Option<DefaultButtonStates>`: `None` where
    `button.default_button_background` is `None`; else `hover` that fill,
    `foreground` `default_button_text_color.unwrap_or(font.color)` and `hover_frame`
    `hover_border_color.unwrap_or(frame)` with `frame` =
    `default_button_border_color.unwrap_or(border.color)`, each through `rgba_to_hsla`
    and each field read at a line of its own (6.G's claims cite those lines).
  - **The rest look**, `CG/geometry.rs`, after `button_checked` (`:1354-1378`) and after
    the builders steps 4 and 5 put there (`toolbar_button_checked`, and
    `toolbar_button_rest` where step 3's trial passed), in the form of `toolbar_button_rest`:

    ```rust
    /// The rest look of a dialog's default Button where the theme gives it one
    /// of its own (`button.default_button_background`, KDE): that fill, and a
    /// border in `button.default_button_border_color`, else `button.border.color`
    /// (docs/platform-facts.md §2.3). `None` where the theme states no fill: the
    /// default button is then the primary one, `.primary()`. Apply it after
    /// [`button`], which sets the border's width, over
    /// [`variants::default_button`](crate::variants::default_button): upstream applies the refinement at rest,
    /// under the variant's hover and press styles. An enabled Button only: a
    /// disabled one takes [`button_disabled`] instead, since upstream replays a
    /// refinement in the disabled state too and [`button_disabled`] sets no
    /// border colour, so this frame would stand there.
    #[must_use]
    pub fn default_button_rest(n: Native<'_>) -> Option<StyleRefinement>
    ```

    Body: `let b = &n.resolved.button;`, then `let fill = b.default_button_background?;`
    and `let frame = b.default_button_border_color.unwrap_or(b.border.color);` on lines
    of their own, and `Some(StyleRefinement::default().bg(rgba_to_hsla(fill)).border_color(rgba_to_hsla(frame)))`.
    Its unit test in `geometry.rs`, `default_button_rest_is_the_stated_fill_and_frame`:
    under kde-breeze light the refinement's `background` is #d6ecf8 and its
    `border_color` #87c0dd; with `default_button_border_color` cleared, the frame is
    `button.border.color`; under adwaita light it is `None`. A `GEOMETRY_NOTES` entry
    (`showcase-gpui/info/mod.rs:214`, one per `pub fn` of `geometry.rs` in its order,
    held to it by `every_geometry_builder_has_a_note`, `CG/showcase.rs:572`):
    `("default_button_rest", "button.default_button_background as the fill and
    button.default_button_border_color (button.border.color where unstated) as the
    border colour of a dialog's default Button at rest, over variants::default_button,
    whose rest fill is a fifth of its border colour (button/button.rs, ButtonVariant::bg_color);
    nothing where the theme states no fill, and the Button is .primary()")`. A row in
    the README's geometry table, after `button_checked`'s (`README.md:364`) and the rows
    steps 4 and 5 add after it.
  6.D.4's test must pass by 3F.3's criterion too; where it does not (case 3), this
  bullet's functions go and the next bullet applies.
- **Where gpui has no receiver** (cases 1 and 3; D5's rule applied to D8): gpui offers
  only what it offers today, the primary button, `.primary()`, and no function is kept.
  A rest refinement over a `Default` button is not offered either: it would give
  Breeze's fill and frame at rest, but upstream's hover and press styles replace a
  caller's fill and border (`GC/button/button.rs:730-764`), so the button would take
  the plain button's hover fill under the pointer, where Breeze keeps the default
  fill. The contract names the three leaves (6.D.3); the README's *Limits* gains "A
  dialog's default button: gpui-component draws it as the primary button
  (`.primary()`); `button.default_button_*` have no receiver" followed, in case 1, by
  ", since a `Button`'s hover and press styles cannot take a fill and border of the
  caller's (request 5)." and, in case 3, by ": a custom variant did not draw Breeze's
  default button in every state (<the state 6.D.4's test found different>)." In case 1,
  request 5 of `docs/issues/0004_kde-hand-overrides-upstream-requests.md`, which 3F.5
  adds, gains the sentence "The same holds for a dialog's default button, which KDE
  fills and frames apart from the primary one."; in case 3 there is no request 5 (open
  point 3).

### 6.D.3 The contract, `connectors/native-theme-gpui/src/contract.rs`

- **gpui has a receiver** (6.D.2): `sinks` (`:2736-2889`) gains two entries: beside step 3's
  `variants::hover_border` entry, `out.insert("variants::default_button_states".to_owned(), format!("{:?}", crate::variants::default_button_states(n)))`
  (`n` is the `Native` `sinks` builds at `:2779`); and in the list of value builders
  (`:2825-2836`, whose entries are `show(&gm::…(n))`), `("default_button_rest",
  show(&gm::default_button_rest(n)))`, which `sinks` inserts as
  `geometry::default_button_rest` (`:2837`). The fill and the border colour then reach
  both, the label and `button.hover_border_color` the states
  (`every_colour_leaf_reaches_a_sink_or_is_named` splices each leaf in every base,
  `:2956-2987`; where the fill is stated each leaf moves a sink, and on the other
  presets the fill leaf is `null` and splices to a colour, which makes both `Some`).
- **gpui has no receiver** (6.D.2, cases 1 and 3): `UNREAD_COLOURS` (`:1095-1344`)
  gains three entries after `button.hover_text_color` (`:1186-1194`), each
  `Unread::NoReceiver` with the evidence "a dialog builds its OK button from a
  `ButtonVariant` (`dialog/dialog.rs:159`), and a `ButtonCustomVariant`, whose own rest
  fill is a fifth of its colour and whose border is that colour in every state
  (`button/button.rs:1028`, `:1118-1124`), did not draw Breeze's default button in every
  state (issue #4 D5: step 3's trial, or where it passed the default button's own
  test)". The test requires `.rs:` in the evidence (`:3027-3038`), which it has; it asks
  no symbol (3F.5).
- Either way the `primary` rows (`:159`, `:165`, `:290`, `:296`, `:505`, `:511`) and the
  pairs reading the primary pair (`:1940-1941`, `:2087-2088`) stay.

### 6.D.4 Tests (step 3's trial passed: cases 2 and 3 of 6.D.2)

In `connectors/native-theme-gpui/src/variants.rs`' test module:

- `default_button_is_none_where_no_fill_is_stated`, a `#[gpui::test]` taking
  `cx: &mut TestAppContext` as `ghost_button_follows_the_installed_theme` does (`:161-162`),
  since `default_button` takes `cx: &App`: for every preset but kde-breeze in both
  modes, `default_button_states`, `geometry::default_button_rest` and
  `default_button` are `None`.
- `default_button_follows_the_installed_theme`: as `ghost_button_follows_the_installed_theme`
  (`:161-183`), a mode switch changes the variant.

In `connectors/native-theme-gpui/src/widgets/tests.rs`, beside step 3's trial test
(3F.2 puts `bordered_button_matches_the_default_button_but_its_hover_border` there), not
in `variants.rs`: the scene reader is a module of the widgets' tests (`#[path =
"scene_read.rs"] mod scene_read;`, `widgets/tests.rs:24-26`), `widgets` is compiled only
with its feature (`src/lib.rs:248-249`), and `scripts/check_release.sh:704-705` runs
`cargo test -p native-theme-gpui --lib --no-default-features`, where a `variants.rs`
test that used it would not compile:

- `default_button_matches_breezes_default_button`, a `#[gpui::test]` by 3F.2's method
  (the button's own quad of the last frame, `widgets/scene_read.rs:22`, under kde-breeze
  light and dark, built as 6.D.2 says): rest, fill #d6ecf8 / #2d4655 and border #87c0dd
  / #4882a1; hovered and pressed (reached as 3F.2 reaches them), border step 3's
  `hover_border_color`, #3daee9 in kde-breeze, with the fill #d6ecf8 / #2d4655 hovered
  and the `button_active` token's colour, kde-breeze's `active_background` #c3e5f6 /
  #2f5368, pressed; disabled, equal to a Default button with `geometry::button_disabled`.
  The label by value: the variant's `foreground` equals `default_button_text_color`,
  #232629 / #fcfcfc (`ButtonCustomVariant: PartialEq`), as 3F.2 asserts its text. Every
  item must be equal (3F.3's strictness); a difference is case 3 of 6.D.2: gpui has no
  receiver.

### 6.D.5 Documentation

`README.md`, *Flat buttons*, after step 3's `bordered_button` paragraph (3F.4): "`variants::default_button(n, cx)`
gives a dialog's default button the platform's own look where the theme states one
(KDE's tinted fill and frame), and is `None` where the default button is the primary
one, which the caller then builds with `.primary()`. The variant's own rest fill is a
fifth of its border colour, so the caller gives the rest look after `geometry::button`
with the refinement `geometry::default_button_rest(n)`, the theme's fill and frame, which
is `Some` exactly where `default_button` is; a disabled default button takes
`geometry::button_disabled` instead. A plain `Dialog`'s footer is yours
(`Dialog::footer`), so the button goes there; an `AlertDialog`'s default footer builds
its OK button from a `ButtonVariant` alone, so it keeps the primary button." with a
`rust,no_run` example in the form of the README's other button examples (its code blocks
are compiled as doctests, `src/lib.rs:277-281`) where gpui has a receiver, or the
*Limits* sentence of 6.D.2 where it has none. `src/lib.rs:112`, the module table's
`button` row, after step 3's "; the hover border via [`variants::bordered_button`]": ";
a dialog's default button via [`variants::default_button`] and
[`geometry::default_button_rest`]" (where gpui has a receiver only).

## 6.E iced connector

### 6.E.1 What draws a default button today

iced has no dialog; an application styles its confirming button with
`styles::button_primary` (`styles.rs:392-428`): idle `button.primary_background` under
`primary_text_color`, hovered and pressed iced's derivation of that fill
(`Primary::generate`, `:403-410`), disabled by `class_button` (`:329-372`).

### 6.E.2 `styles::button_default`, `connectors/native-theme-iced/src/styles.rs`

A new public function after `button_primary` (`:392-428`):

```rust
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_default(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>
```

Doc: "The platform's own default button, the one Enter activates in a dialog, for
`button(..).style(..)`. Where the theme states `button.default_button_background`
(KDE's Breeze gives its default button a look of its own), the button is that fill,
lettered in `button.default_button_text_color`, else `button.font.color`, and framed in
`button.default_button_border_color`, else `button.border.color`; under the pointer the
same fill and label, framed in `button.hover_border_color` where the theme states it,
else that frame; pressed, [`button`]'s fill and label in that hovered frame; disabled,
[`button`]'s. Where the theme states none, it is [`button_primary`] in every state: the
default button is the primary one on macOS and Windows (docs/platform-facts.md §2.3)."

Body: `let plain = button(resolved); let primary = button_primary(resolved);`, `let frame =
b.default_button_border_color.unwrap_or(b.border.color);` and `let hover_frame =
b.hover_border_color.unwrap_or(frame);` (decision 20; step 3's field), and, where the
fill is `Some`: `Active`, `background: Some(Background::Color(to_color(fill)))`,
`text_color` the label, `border: Border { color: to_color(frame), ..border_of(&b.border) }`,
`shadow` and `snap` from `plain`'s; `Hovered`, the same with `to_color(hover_frame)`;
`Pressed`, `plain(theme, Pressed)` with its border's colour replaced by
`to_color(hover_frame)`; `Disabled`, `plain(theme, Disabled)`. Where the fill is `None`,
the closure returns `primary(theme, status)`.

**Per state** (light / dark):

| Preset | Active | Hovered | Pressed | Disabled |
|---|---|---|---|---|
| kde-breeze, after | #d6ecf8 / #2d4655 under #232629 / #fcfcfc, frame #87c0dd / #4882a1 | the same fill and label, frame `hover_border_color`, #3daee9 in kde-breeze after step 3 | `styles::button`'s fill and label: #c3e5f6 / #2f5368 composited over #fcfcfc / #292c30 (opaque, so itself) under #232629 / #fcfcfc; frame #3daee9 | `styles::button`'s: #f0f0f0 / #272a2e under #a8a9aa / #6d6f72 |
| every other preset, after | `button_primary`'s | `button_primary`'s | `button_primary`'s | `button_primary`'s |

The primary fills `button_primary` takes per preset, computed today from the presets by
`primary_background ← defaults.accent_color` and `primary_text_color ←
defaults.accent_text_color` (`docs/inheritance-rules.toml:164-165`): adwaita #3584e4 /
#3584e4 under #ffffff; catppuccin-frappe #8caaee under #ffffff / #303446; -latte
#1e66f5 / #8caaee under #ffffff / #303446; -macchiato #8aadf4 under #ffffff / #24273a;
-mocha #89b4fa under #ffffff / #1e1e2e; dracula #644ac9 / #bd93f9 under #ffffff /
#282a36; gruvbox #076678 / #83a598 under #fbf1c7 / #282828; ios-13 #007aff / #0a84ff
under #ffffff; kde-breeze (today, and its primary after) #3daee9 under #ffffff /
#fcfcfc; the two macOS presets #007aff under #ffffff; material-3 #6750a4 / #d0bcff
under #ffffff / #381e72; nord #5e81ac / #88c0d0 under #ffffff / #2e3440; one-dark
#4078f2 / #61aeee under #ffffff / #282c34; solarized #268bd2 under #fdf6e3 / #002b36;
tokyo-night #2e7de9 / #7aa2f7 under #ffffff / #1a1b26; windows-11 #0067c0 / #4cc2ff
(stated) under #ffffff / #000000. These change neither today nor after.

### 6.E.3 The contract

In `styles::button_toggle`'s pattern, at each of its places:

- `src/contract/rows.rs`: `pub(super) fn native_button_default(r: &ResolvedTheme, status: button::Status) -> (Color, Color)`
  beside `native_button_toggle` (`:405-433`): the fill and label of 6.E.2 where the fill
  is stated; where it is not, the primary class's native pair,
  `native_class_button_fill` and `native_class_button_label` with
  `button.primary_background` and `primary_text_color` (`:480-527`). A `native: |r, s|`
  row has no `Theme`, so it cannot compute iced's derivation, which the primary button
  takes in `Hovered` and `Pressed` where the fill is unstated (`primary_vs_iced`,
  `src/contract.rs:2985`, compares it under `primary_slot_theme`, `:2963`, a theme of its
  own). So the rows and the pair cover `CLASS_BUTTON_NATIVE_STATUSES`, `Active` and
  `Disabled` (`rows.rs:463-467`), as the four class buttons' rows and pairs do
  (`class_button_contract!`, `src/contract.rs:427-475`, its rows `:433-446` and pair
  `:461-475`), and `native_button_default`'s two arms no row reaches are, where the
  fill is stated: `Hovered`, the `Active` pair (the stated fill and label, as 6.E.2's
  hovered look); `Pressed`, the plain button's pressed pair,
  `(native_button_fill(r, button::Status::Pressed), native_button_label(r, button::Status::Pressed))`,
  as 6.E.2's `Pressed` is `plain(theme, Pressed)`'s fill and label; and where it is
  not, both arms the idle primary pair (the class pair at `Active`), as
  `native_class_button_fill` does for the statuses no row reaches (its doc,
  `rows.rs:480-487`). `BUTTON_DEFAULT_ROWS` (`background`, `text_color` over
  `CLASS_BUTTON_NATIVE_STATUSES`) and `BUTTON_DEFAULT_BORDER_ROWS` (the frame of 6.E.2,
  over `BUTTON_STATUSES`, `:282`, as the class buttons' border rows, `src/contract.rs:449-454`,
  since the border is native in every status) after `BUTTON_TOGGLE_BORDER_ROWS`
  (`rows.rs:456-461`); their names in the list at `rows.rs:2951` and `:2971`. The
  border row's `native` is a closure `|r, s|` returning a `NativeBorder`
  (`rows.rs:341-352`, `native_button_border`'s type): where the fill is stated, in
  `Active` the frame, `default_button_border_color` else `button.border.color`, in
  `Hovered` and `Pressed` the hover frame, `hover_border_color` else that frame, each
  with `button.border`'s `line_width` and `corner_radius`, as `border_of` gives the
  style (`styles.rs:174-180`), and in `Disabled` `native_button_border(r,
  button::Status::Disabled)`, the plain button's faded border that `plain(theme,
  Disabled)` wears; where it is not, `native_button_border(r, s)` in every status, as
  the class buttons' border rows (`src/contract.rs:449-454`) have it for
  `button_primary`. `Hovered` and `Pressed`
  where the fill is stated are covered by the test below, where it is not by
  `button_default_is_the_primary_where_no_fill_is_stated` with the primary's own test
  (`a_primary_button_takes_its_hovered_and_pressed_states_from_its_own_fill`,
  `src/contract.rs:3055-3075`).
- `src/contract.rs`: `button_default_style_fields` beside `button_toggle_style_fields`
  (`:1278-1286`); its call in the walk after `:1738-1742`; the row checks after
  `:3429-3433`; the pair check after `:3588`; the extractor lines after `:5091` and
  `:5119`.
- `src/contract/derived.rs`: `styles::button_default.shadow` and `.snap` after
  `styles::button_toggle.snap` (`:818-825`), "iced default: button::Style::default().shadow,
  through styles::button or styles::button_primary -- the model has a shadow color but
  no offset or blur" and the `snap` twin.
- `src/contract/pairs.rs`: `BUTTON_DEFAULT_PAIRS` ("default button label", the label on
  the fill over `defaults.background_color`, `statuses: CLASS_BUTTON_NATIVE_STATUSES`,
  `native` from `native_button_default`, in the form of `BUTTON_TOGGLE_PAIRS`) after
  `BUTTON_TOGGLE_PAIRS` (`:171-190`); its name after `:921`. In `Hovered` and `Pressed`
  where the fill is unstated the pair is iced's derivation, which no native pair
  states, as the class pairs' doc says (`src/contract.rs:456-459`).
- `contract-leaves.toml`: regenerated with
  `cargo test -p native-theme-iced --all-features --lib -- --ignored regenerate_contract_leaves`
  (step 1's E.4). Expected difference, by leaf, the rows each gains (three new
  `[rows."styles::button_default.*"]` tables, `.background`, `.text_color` and
  `.border`, beside `styles::button_primary.*`'s), derived here from the rows and the
  presets, not run:
  - `button.default_button_background`: the walk splices a colour into every leaf, a
    `null` one too (`src/contract.rs:5290-5310`), so on the 32 combinations that leave
    the fill unset the splice makes it stated and turns the button from
    `button_primary` into 6.E.2's own look. So: `.background` (`Active`) in all 34;
    `.text_color` (`Active`) wherever `button.font.color`, the label of a stated fill
    with no `default_button_text_color`, differs from `button.primary_text_color`; and
    the `Disabled` rows (`.background`, `.text_color`) wherever the class button's
    disabled pair (`native_class_button_fill` and `native_class_button_label`) and the
    plain button's (`plain(theme, Disabled)`) differ;
  - `button.default_button_text_color`: `.text_color` (`Active`), only under kde-breeze;
  - `button.default_button_border_color`: `.border` (`Active`), only under kde-breeze;
  - `button.hover_border_color`: `.border` (`Hovered`, `Pressed`): under kde-breeze the
    hover frame, elsewhere `button_primary`'s hovered and pressed border, which step
    3's 3D.1 makes `hover_border_color`, else `border.color` (its `class_button` row),
    where the splice of the unset leaf moves it;
  - `button.border.color`: `.border`: under kde-breeze in `Disabled` only (the plain
    button's border, faded; `Active` is the stated frame, `Hovered` and `Pressed` the
    stated hover frame), elsewhere in every status `button_primary`'s border takes it;
  - `button.disabled_background`: `.background` (`Disabled`) everywhere, and
    `.text_color` (`Disabled`) outside kde-breeze (the class label tests it,
    `native_class_button_label`, `rows.rs:509-528`);
  - `button.disabled_text_color`: `.text_color` (`Disabled`): under kde-breeze (the
    plain label, `native_button_label`), elsewhere where `disabled_background` is
    stated;
  - `button.primary_background`: `.background` (`Active`, and `Disabled` where no
    `disabled_background` is stated), `except` kde-breeze;
  - `button.primary_text_color`: `.text_color` (`Active`, and `Disabled` where no
    `disabled_background` is stated), `except` kde-breeze.
  kde-breeze states `disabled_background` in both modes (`#f0f0f0`, `#272a2e`), so
  `button.background_color` moves no `styles::button_default` row there. The regenerated
  file is the authority: a difference from this list is read against the rows before
  the unit lands and explained in its commit body.
- A test in `src/contract.rs`: `button_default_is_the_primary_where_no_fill_is_stated`,
  for every combination whose `default_button_background` is `None` and each status,
  `format!("{:?}")` of the two styles is equal.
- A second, `button_default_hovers_and_presses_as_stated`, for the statuses the rows
  leave out where the fill is stated: for every combination whose
  `default_button_background` is `Some` (kde-breeze, both modes), `Hovered`'s
  background and text colour equal `Active`'s, and `Pressed`'s equal
  `styles::button(r)`'s `Pressed` (6.E.2); the borders are the border rows'.

### 6.E.4 Documentation

`README.md:189`, the styles table's "with a status" row: `button_default` follows step
5's `toolbar_button` (itself after `button_toggle`), and the last cell gains, after step
5's `toolbar_button(&resolved, checked)` gloss, "`button_default` is a dialog's default
button, the primary one where the platform gives it no look of its own;", in the cell's own
form. The crate
doc's coverage table, `src/lib.rs:318`, the `styles` row: "`styles` (33 functions) |
every `Style` field of button (six classes), toggle button, text input, …" counts the
`pub fn`s of `src/styles.rs` but `composite_over`, 33 at `0fa28027` (34 `pub fn` lines,
read today), and names the button styles (the six classes `button`, `button_primary`,
`button_danger`, `button_success`, `button_warning`, `button_link`, then
`button_toggle`). Step 5 adds `styles::toolbar_button` and edits the row in place
(steps 4–5's B.8: "(34 functions)", "toolbar button" after "toggle button"). So this
step's edit, in place: the count becomes theirs plus one, re-read on the tree (the
top-level `^pub fn`s but `composite_over`; 35 where no other work added one), and "a
dialog's default button" goes after step 5's "toolbar button" (the README row's list
likewise: `button_default` after `toolbar_button`). `ROADMAP.md:162`'s "has 34 style
functions" (34 after step 5's B.8) moves to the same count, in 6.H's documents unit.

## 6.F egui connector

### 6.F.1 What draws a default button in egui today

egui has no default button. A selected button in the «button» role fills
`visuals.selection.bg_fill`, lettered `visuals.selection.stroke.color`
(`EG/style.rs:359-368`), which the role sets to the primary pair
(`connectors/native-theme-egui/src/style/roles.rs:228-230`): that is the primary button,
as the showcase draws "button (suggested action)" (`showcase-egui/pages/basic.rs:532`).
The role's writes stay (D8).

### 6.F.2 The accessor, `connectors/native-theme-egui/src/accessors.rs`

After `window_title_bar_text_color` (`:286-297`):

```rust
/// A dialog's default button where the theme gives it a look of its own:
/// the fill, label colour and frame to hand `Button::fill`, `RichText::color`
/// and `Button::stroke` at the call site.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct DefaultButton {
    /// `button.default_button_background`.
    pub fill: egui::Color32,
    /// `button.default_button_text_color`, else `button.font.color`.
    pub text: egui::Color32,
    /// `button.default_button_border_color`, else `button.border.color`, at
    /// `button.border.line_width`.
    pub stroke: egui::Stroke,
    /// The frame under the pointer and while pressed: `button.hover_border_color`,
    /// else `stroke`'s colour, at the same width.
    pub hover_stroke: egui::Stroke,
    /// The frame while it has the keyboard focus: `button.focus_border_color`,
    /// else `hover_stroke`'s colour, at the same width.
    pub focus_stroke: egui::Stroke,
}

#[must_use]
pub fn default_button(t: &ResolvedTheme) -> Option<DefaultButton>
```

Doc of `default_button`: "`Some` where the theme states `button.default_button_background`
(KDE), `None` where the default button is the primary one (macOS, Windows): draw it then
as a selected button in the `Role::Button` scope, which takes `button.primary_background`
and `primary_text_color`. egui's `Button::fill` and `Button::stroke` override every
on-hover effect ("Note that this will override any on-hover effects",
`egui/src/widgets/button.rs:<L>` at egui `<V>`), so one call draws one look in every state, and the
states are the application's, from the button's previous response: at rest `fill`,
`text` and `stroke`; while it was hovered, the same with `hover_stroke`; while it had
the keyboard focus and was not pressed, the same with `focus_stroke`, since egui gives a
focused button its `active` look (`egui/src/style.rs:<F>`), which `Button::stroke`
overrides as well; while it was pressed, `hover_stroke` with neither `fill` nor `text`,
so that egui draws the plain button's pressed fill and label, which the `Role::Button`
scope gives it, as Breeze draws a pressed default button (docs/platform-facts.md
§2.3)." `<F>` is `1273-1285` at egui 0.36.2 (`Widgets::style`, `active` where
`response.has_focus()`, `:1276`) and `1383-1395` at 0.37.0 (the same test at `:1386`).
**The focus stroke** is the egui side of step 3's `focus_border_color`: the «button»
role writes it into the `active` stroke (steps 2 and 3's 3E.1, `focus_border_color`,
else `hover_border_color`, else `border.color`), which a per-call `Button::stroke`
replaces in every state (`EG/widgets/button.rs:148`, "this will override any on-hover
effects"); without the field a focused default button would show its rest frame. The
change stays inside this step: one field, its line in `default_button`, the showcase's
memory keeping `has_focus()` beside the other two (6.G), the test (6.F.4) and the
docs. It adds no mapping row and no sink: `default_button` is a per-call accessor, so
T2's splice of `button.focus_border_color` moves no further `Style` field and its row
keeps the sinks 3E.2 gives it, and T10, which compares sinks with their rows'
formulas, has no new sink to compare; the three `default_button_*` rows stay `tested_by
= "T18(a)"`, as `hover_border_color`, which the accessor reads already, keeps its row. `<L>` is the line range
of the tree the step builds on and `<V>` its egui version: `140-155` at egui 0.36.2,
`153-168` at 0.37.0 (decision 6; the header's rule), the doc naming the version with
the lines (a pinned source). The egui showcase draws these states (6.G), and its Widget Info
says so. `src/lib.rs:199-203`, the `pub use accessors::{..}` list, gains `default_button`
and `DefaultButton` (in its alphabetical place).

### 6.F.3 Mapping, `mapping.toml`

After `["button.checked_text_color"]` (`:996-998`), three rows of the per-call form
(`verdict = "derived"`, `tested_by = "T18(a)"`, as `button.checked_background`'s,
`:992-994`): `["button.default_button_background"]`, `["button.default_button_text_color"]`,
`["button.default_button_border_color"]`. No `probe`: the five platform presets'
pairs differentiate each leaf (kde-breeze states it, the others leave it `null`).

`src/mapping_tests.rs`: `every_leaf_has_exactly_one_row_and_every_row_is_well_formed`
(`:236-241`, today 516) grows by three, and its message, "512 ResolvedTheme leaves + 4
layout leaves (§5.7)" (`:240`), with it (the `ResolvedTheme` count by three);
`verdict_totals_match_the_specification` (`:473-480`): the `buttons` group's derived
count grows by three (`[11, 46, 31, 25]` at `0fa28027`, `[11, 46, 34, 25]` if no
earlier step had touched the group). The work proposed before steps 6 and 7 (D10)
changes the same numbers first: step 2 a `defaults` leaf, step 3 two `button` leaves,
steps 4 and 5 `checkbox`, `combo_box` and `toolbar` leaves, the caret work three
`input` leaves; the implementer adds this step's deltas to the numbers the test holds
on its tree, as every count in this document.
`src/mapping.md` is regenerated
(`cargo test -p native-theme-egui --lib -- --ignored regenerate_mapping_md`, its header,
`:3`).

### 6.F.4 Tests, `src/install_tests.rs`

New, `t18_a_default_button_is_the_themes_or_none`, its doc's first line "T18 (a):
`default_button` is the theme's default button where it states a fill, and `None`
where it does not." (`showcase-egui/info.rs:73-97` finds a clause's tests by the
`t18_a_` prefix or that first line): kde-breeze light gives
`Some(DefaultButton { fill: #d6ecf8, text: #232629, stroke: Stroke::new(1.0, #87c0dd), hover_stroke: Stroke::new(1.0, #3daee9), focus_stroke: Stroke::new(1.0, #3daee9) })`
(the hover and focus frames kde-breeze's `hover_border_color` and `focus_border_color`,
both #3daee9 after step 3's 3B.5)
through `to_color32` (the width `button.border.line_width`, 1 in kde-breeze's
`[light.defaults.border]`, `kde-breeze.toml:77`, which `[light.button.border]` does not
override); adwaita light gives `None`; a theme with only `default_button_text_color`
stated gives `None`; and kde-breeze light with `button.focus_border_color` set to `None`
gives a `focus_stroke` equal to its `hover_stroke` (the fallback), and with
`hover_border_color` `None` as well, one equal to its `stroke`.

### 6.F.5 Parity and README

`scripts/connector_parity_exceptions.toml`: one entry, `connector = ["iced", "gpui"]`,
`name = "default_button"`, `kind = "absent"`, reason "egui has no default button and
its `Button` takes a fill and a stroke per call, which this returns
(connectors/native-theme-egui/src/accessors.rs, `default_button`'s doc); iced's is the
style function `styles::button_default`, gpui's the custom variant `variants::default_button`
for a button the application builds (connectors/native-theme-gpui/src/variants.rs), both
in modules the script lists without comparing". Where gpui has no receiver (6.D.2,
cases 1 and 3), the gpui half of the reason reads instead "gpui has none:
gpui-component draws a dialog's default button as the primary one
(connectors/native-theme-gpui/README.md, *Limits*)".

`README.md:262`, the accessor list ("`window_title_bar_text_color`, `dialog_button_order`,
`list_header_height`…"): `default_button` with the doc's first sentence and its rule for
the four states (at rest `fill`, `text`, `stroke`; hovered `hover_stroke`; focused
`focus_stroke`; pressed `hover_stroke` and no fill, decision 6).

## 6.G Showcases

**A shared element (decision 7).** `docs/showcase-elements.toml`, after the element
`basic.buttons.toggle_on.label` (`:849-856`), under a comment in the form of the third
row's (`:820`), the Buttons group's last elements:

```toml
# The fourth row: a dialog's default button, the one Enter activates.

[[element]]
id = "basic.buttons.dialog_default"
name = "Button"
parent = "basic.buttons.toggle_off"
leaves = ["button.font", "button.font.color", "button.min_width_px", "button.min_height_px", "button.primary_background", "button.primary_text_color", "button.active_background", "button.active_text_color", "button.hover_border_color", "button.focus_border_color", "button.default_button_background", "button.default_button_text_color", "button.default_button_border_color", "button.border.color", "button.border.corner_radius_px", "button.border.line_width_px", "button.border.shadow_enabled", "button.border.padding_top_px", "button.border.padding_right_px", "button.border.padding_bottom_px", "button.border.padding_left_px", "defaults.line_height"]
states = ["Default"]
sample = { fill = [0.5, 0.0, 0.0, 2.5], border = [0.5, 0.0, 0.0, 0.5] }

[[element]]
id = "basic.buttons.dialog_default.label"
name = "Button label"
parent = "basic.buttons.dialog_default"
leaves = ["button.font", "button.font.color", "button.primary_text_color", "button.active_text_color", "button.default_button_text_color", "defaults.line_height"]
states = ["Default"]
part = true
sample = { text = "glyph" }
```

The parent is the first element of the row above, as the third row's first button's,
`basic.buttons.toggle_off`, is the second row's first, `basic.buttons.disabled`
(`:822-826`), by the list's rule "a control against its heading or the control before
it" (`:672-674`). The next group's heading, `basic.checkboxes.heading` (`:860-866`),
whose parent is `basic.buttons.toggle_off` today (`:863`), takes
`parent = "basic.buttons.dialog_default"`, by the same rule's "A heading is measured
against the element above it in its five-column column" (`:672`). The leaves are every leaf the button draws in any
of its states, as `basic.buttons.default` lists its hovered and pressed leaves under
the state `Normal` (`:734-740`): the primary pair where the fill is unstated, the three
fields and their fallbacks `button.font.color` and `button.border.color` where it is
stated, the plain button's pressed pair (decision 3), and step 3's two border leaves,
which its 3B.4 adds immediately before `"button.border.color"` to every Basic button
that wears `button.border` and can be hovered (the hover one is drawn here by decision
20). The control's leaves are the union of its label's (the file's header, `:36`). They
are in the file's order (`:35-36`: the element's own table in registry order, then the
others): `button` in `docs/property-registry.toml`'s `[button]` order, where step 3's
3B.2 puts its two keys after `checked_text_color` and 6.B.1 these three after them, then
`defaults`. No leaf is listed twice, and `states` is non-empty: `check_showcase_parity.py
--check-list` accepts any non-empty state name (`:483-490`, read today), "Default"
included. The sample keys are `basic.buttons.primary`'s (`:751-757`).

**Its row, the same in all three.** The button is the Buttons group's fourth row, alone
on it, below the toggle row, the gap between it and the toggle row each toolkit's gap
between the group's rows already: gpui's group is `with_gap(v_flex(), widget_gap)`
(`showcase-gpui/pages/basic.rs:384-389`), iced's Buttons column `.spacing(gap.widget)`
(`showcase-iced.rs:10497`), egui's group `Ui` keeps the column's `item_spacing`
between its rows (`showcase-egui/pages/basic.rs:296-311`). A row of one button holds
nothing to wrap.

**iced** (`showcase-iced.rs`): `Push` (`:10006-10013`) gains `DialogDefault` after `On`,
and the `push` closure's match (`:10393-10412`, in the closure `:10388-10416`) an arm in its neighbours' form styled
`styles::button_default(resolved)`. The Buttons column (`:10442-10497`) gains a fourth
item after the toggle row (`:10477-10495`): `push("basic.buttons.dialog_default",
"basic.buttons.dialog_default.label", "OK", Push::DialogDefault,
Some(Message::ButtonPressed))`, alone, with no `row!` round it, under a comment in the
form of the toggle row's (`:10477`, "The toggle buttons, off and on (Basic v4's third
row)"). The route functions: `drawing()` (`:7341`) gains `_ if under("basic.buttons.dialog_default")
=> Drawing::enabled(&["styles::button_default."])` before its `_ if under("basic.buttons")`
arm (`:7360`), and before that one an arm for the pressed pair where the fill is
stated, `_ if under("basic.buttons.dialog_default") &&
resolved.button.default_button_background.is_some() && (leaf ==
"button.active_background" || leaf == "button.active_text_color") =>
Drawing::only(BUTTON, &["Pressed"], &[])`: there the pressed button is `styles::button`'s
`Pressed` (6.E.2), whose rows read the two leaves, while `styles::button_default`'s rows
cover `Active` and `Disabled` only (6.E.3), so the info would otherwise say the leaves
are "not drawn here: it moves no row of … in the state shown" (`colour_route`,
`:7595-7610`). `Drawing::only(rows, with, without)` and `BUTTON`, `&["styles::button."]`,
are the showcase's own (`:7245-7255`, `:7267`; the disabled row's arm, `:7358`, is
`Drawing::only(BUTTON, &["Disabled"], &[])`), and `drawing` has the leaf and
`shown.resolved` (`:7341-7342`); `colour_route` reads the drawing per leaf
(`:7580`); `iced_route`'s button class (`let class = if ghost {`, `:7792-7801`) gains `else if
under("basic.buttons.dialog_default") { "styles::button_default" }` before the
`styles::button` fallback; the names table (`:29453-29454`) gains
`("basic.buttons.dialog_default", "Button · Default")`. Every other place of the showcase
that lists `styles::button_primary` among the style functions, found today with
`grep -n '"styles::button_primary"'`, gains `styles::button_default` where its subject
includes it: `:20797`, in `DRESSED`'s `button` row (`:20792-20810`), the closed list of
the styles a `button(` call site may carry, which `styles_cover_every_widget_shown`
holds every site to (`:20949-20965`), so the new button is red without it: it gains
`"styles::button_default"` after `"styles::button_toggle"`; `:21009`
(`themed_buttons_take_the_themes_padding`, `:21005`), the classes whose buttons must
take the theme's padding: it gains `"styles::button_default"`, which the new button
takes through `button_padding(` as its neighbours do; `:7797`, `iced_route`'s button
class, above. No edit at `:27907-27911` (`DRAWN_WITH`, the drawn-with table of
`drawn_with_is_what_the_pages_draw_with`) or `:28217` (`row_ink`'s arm): both serve the
infos of page widgets wrapped in `hoverable` (`DRAWN_WITH` is read by the `hoverable(`
call sites, `:28103`; `row_ink` by `colour_row_faults` over `built_infos`, "The page
widgets the pages of `state` wrap in [`hoverable`]", `:28859-28867`), and the Basic
page's buttons are built by the `push` closure, which wraps none in `hoverable`
(`:10388-10416`); `:12207-12238` is the disabled row's info
(`disabled_buttons_info`), not changed. Each is read before it is edited. The route test
`the_element_list_parses_and_every_leaf_has_a_route` (`:24982`) holds the new element's
leaves to a route: the `drawing()` arm above makes `drawing(..)` `Some` for them, so
their routes are `colour_route`'s from the contract export (`:7579-7580`), naming
`styles::button_default`, which reads them. `every_hand_colour_route_names_a_function_that_reads_it`
(`:29102`) skips every leaf whose `drawing(..)` is `Some` (`:29126-29129`), so it does
not read them.

**egui** (`showcase-egui/pages/basic.rs:519-612`, `buttons`): after the toggle row
(`:590-596`), a fourth `ui.horizontal_wrapped` row holding one "OK" button through the
page's `text_button` helper, as its neighbours (`:531`, `:534`, `:592`), with the kind
`("basic.buttons.dialog_default", "button (dialog default)", normal)`. Where
`default_button(t)` is `Some(d)`, the closure builds it from the button's previous
response, read from the page's memory (`ui.ctx().data(..)`, as `remember_width`,
`:510-513`, keeps a width there), into which the page writes the response's
`hovered()`, `is_pointer_button_down_on()` and `has_focus()` after drawing it: at rest
`Button::new(text.color(d.text)).fill(d.fill).stroke(d.stroke)`, hovered the same with
`d.hover_stroke`, focused and not pressed the same with `d.focus_stroke` (focus before
hover, as egui's `Widgets::style` puts `has_focus()` before `hovered()`), pressed
`Button::new(text).stroke(d.hover_stroke)` (decision 6). Where
it is `None`, `.selected(true)`, as the suggested action is drawn. Either way it takes
`.min_size(min)`, for parity with the neighbouring buttons, which take it (`:525`,
`:531`, `:534`); `the_palette_pages_apply_the_per_call_leaves`
(`showcase-egui/tests.rs:3652`) holds the `Role::Button` kinds of the Buttons and
Overlays pages to `button.min_width` (`:3641-3650`, the pages it opens), not the Basic
page's. The doc of `buttons` (`basic.rs:515-518`): "Buttons (the toggle button their
third row), check boxes, …" becomes "Buttons (the toggle button their third row, a
dialog's default button their fourth), check boxes, …", and "The Buttons group: three
rows of two buttons, each row wrapping …" becomes "The Buttons group: three rows of two
buttons and a fourth of one, each row wrapping …". The test
`the_basic_page_has_every_group_in_its_column` (`showcase-egui/tests.rs:5554`) counts
each group's kinds from `BASIC_CONTROLS` (`:5477-5546`): the `"Buttons"` entry
(`:5478-5488`) gains `("button (dialog default)", 1)` after `("toggle button (on)", 1)`.

**gpui** (`showcase-gpui/`): `elements.rs` gains `("basic-button-dialog-default", "basic.buttons.dialog_default")`
after `("basic-toggle-on", "basic.buttons.toggle_on")` (`:87`), and `LABELS`
(`:269-276`) `("basic-button-dialog-default", "basic.buttons.dialog_default.label")`
after `:275`. The button is built by a new helper, `demo::dialog_default_button(ui, cx,
id, label)`, in the form of `demo::toggle_button` (`demo.rs:9157-9202`), after it, and
is the Buttons group's fourth row: its call follows the toggle row (`pages/basic.rs:450-453`)
in the group's list (`:434-454`), `demo::dialog_default_button(ui, cx,
"basic-button-dialog-default", "OK").into_any_element()`, with no `row()` round it.
`BUTTONS` stays `[BasicButton; 2]` and `ButtonKind` gains no variant: a `BUTTONS` entry
is a button of the first row built by `demo::button` (`demo.rs:3330-…`), which applies
the variant (`ButtonKind::apply`, `:3365`) before `geometry::button` and no refinement
after it but the disabled pair, where `geometry::default_button_rest` must come after
`geometry::button` (6.D.2); and a `ButtonKind` variant would take an arm in every
`match kind` of `info/buttons.rs` (`:18`, `:163`, `:403`, `:450`, `:513`, `:534`,
`:560`), which on the tree U6.1 starts from hold the arms of step 3's `ButtonKind::Bordered` already
(3F.4: `Bordered`, with `ButtonKind::built(self, cx)`, in `at_rest`, `under_the_pointer`,
`edge`, the idle edge note, the shadow note and the variant line). The helper follows
`toggle_button`'s form (`demo.rs:9157-9202`, which step 3 leaves unchanged):
`Button::new(id)` refined through `native_info`; this button has no `ButtonKind` of its
own. Where gpui has a receiver (6.D.2) and `variants::default_button(n, cx)` is
`Some(v)`, the helper builds `Button::new(id).custom(v)`, refined by
`geometry::button` through `native_info` (`info/mod.rs:419-425`, as `toggle_button`
does, `demo.rs:9186-9192`) and then by `rest`, `geometry::default_button_rest(n)`,
through `refined(button, rest.as_ref())` (`support.rs:273`; `native_info` takes a
builder that returns a `StyleRefinement`, not an `Option`), its line recorded with
`button_info.geometry("default_button_rest")` as `demo::button` records
`button_disabled` (`demo.rs:3377-3379`), as 3F.4 has the page's default, tooltip and
toggle-off buttons built through `bordered_button`; where either is `None`, or gpui has no
receiver, `Button::new(id).primary()` refined by `geometry::button` alone. Its info is registered
as `toggle_button` registers its own (`InfoExt::info(labelled(ui, cx, button, id,
label), ui, id, button_info)` and `.debug_selector(..)`, `:9200-9201`): for the
`.primary()` button `info::buttons::button(cx.theme(), ButtonKind::Primary,
ButtonState::Idle, false, None, native_button)`, the primary button's info; for the
variant the info of *gpui's colour claims* below. This use is what
`the_showcase_exercises_every_builder` (`CG/showcase.rs:384-416`) requires of every
`pub fn` of `variants.rs` and `geometry.rs`, so it lands with them (6.I.1, U6.1). Where
gpui has no receiver, `docs/showcase-exceptions.toml` gains a `[parity]` exception for
`basic.buttons.dialog_default` under kde-breeze, in the file's form (its header,
`:110-125`: a key is `[<run>@][<preset>[/<variant>]:]<element id>.<property>`, the
property one of the element's sample names; a group states its `reason` once for its
`keys`), as a group of its own after the last one:

```toml
[[parity.group]]
reason = "gpui-component has no receiver for a default button's look: a ButtonCustomVariant paints its own rest fill as a fifth of its colour and its border in that colour in every state (gpui-component 0.9.1 src/button/button.rs:1028, :1118-1124), so the Button is .primary() and draws the accent pair where iced and egui draw kde-breeze's default button (issue #4: step 3's trial, or where it passed the default button's own test)"
keys = [
    "kde-breeze:basic.buttons.dialog_default.fill",
    "kde-breeze:basic.buttons.dialog_default.border",
    "kde-breeze:basic.buttons.dialog_default.label.text",
]
```

The three keys are the element's two samples (`fill`, `border`) and its label's one
(`text`), from the element list above; unscoped by variant, as kde-breeze's light and
dark both state the fill (6.C.3). The section is the one `check_showcase_parity.py
--merge OUT --write` rewrites, which keeps a group a difference still needs (`:126-131`).

**gpui's routes, in both cases of 6.D.2's condition.** `every_listed_leaf_has_a_route`
(`showcase-gpui/tests.rs:15196-15209`) needs a `HOW` line (`info/leaves.rs:15`, an
`(element-id prefix, leaf, line)` triple; the first whose prefix the element's id starts
with wins, `:2368-2373`) for every leaf of every listed element, and the lookup is by
prefix (`element.starts_with(prefix)`), so `basic.buttons.default`'s lines do not cover
`basic.buttons.dialog_default`. The new element's other leaves have generic lines
already (`""`-prefixed `button.font`, `.font.color`, `.min_width_px`, `.min_height_px`,
`.primary_background`, `.primary_text_color`, `.active_background`,
`.active_text_color`, `.border.*`, `defaults.line_height`; `:281-386` among them, read
today), and `button.focus_border_color` takes the generic line 3F.4 adds and 3F.5 keeps
(its text is 3F.4's, under *Showcase: the routes*, and is not repeated here),
which holds for this button in both cases: gpui-component calls `focus_style` on every
focused Button whatever its variant (`GC/button/button.rs:920-922`, with the line
chosen at `:644-649`), and `focus_style` turns the border to `ring` with the band on
(`GC/styled.rs:257-263`) and, with it off, where the element has a border width
(`:269-271`), and otherwise the line its `FocusLine` gives (`:273-297`); a custom variant and
`.primary()` alike go through it, so neither case of 6.D.2 leaves the line's text. `button.hover_border_color` has no generic line (3F.4 and
3F.5 give it per element), so it gets one here with the three new leaves. Two generic
lines do not hold for this button: `button.font.color`'s ("gpui: ThemeColor
button_foreground (colors.rs): the label's colour", the plain Button's) in either case,
since the variant takes the colour through `default_button_states` and a `.primary()`
Button letters in `button_primary_foreground`; and `button.active_background`'s ("gpui:
ThemeColor button_active, …; the pressed fill"), since a `.primary()` Button is pressed
in `button_primary_active`: where gpui has no receiver on every preset, and where it has
one on every preset but kde-breeze (there the variant's
`.active(cx.theme().button_active)` is the pressed fill, which the line names). So
these get lines here too, in both cases. Each has the
prefix `"basic.buttons.dialog_default"` (which covers the label too), in the button
section after `basic.buttons.toggle_on`'s entries (`:16-36` at `0fa28027`, and step
3's `button.hover_border_color` entry for it, 3F.4):

- Where gpui has a receiver:
  - `button.default_button_background`: "gpui: geometry::default_button_rest's bg at rest, after geometry::button, and variants::default_button's hover fill; where unstated the Button is .primary(), ThemeColor primary"
  - `button.default_button_text_color`: "gpui: variants::default_button's foreground, in every state; where unstated the Primary Button's button_primary_foreground"
  - `button.default_button_border_color`: "gpui: geometry::default_button_rest's border colour at rest, after geometry::button; under the pointer and pressed variants::default_button's color, button.hover_border_color where stated"
  - `button.hover_border_color`: "gpui: variants::default_button's color, the border under the pointer and pressed, where the theme states button.default_button_background; elsewhere the Button is .primary(), whose hover and press border is the primary token (gpui-component/button/button.rs:1088, primary) and whose rest border is geometry::button's button.border.color (native-theme-gpui/geometry.rs:230, border_color)"
  - `button.font.color`: "gpui: variants::default_button_states's label where the theme states button.default_button_background and no button.default_button_text_color, the variant's foreground in every state; elsewhere the Button is .primary(), lettered in ThemeColor button_primary_foreground (gpui-component/button/button.rs:1039, button_primary_foreground)"
  - `button.active_background`: "gpui: variants::default_button's active, ThemeColor button_active (colors.rs), the pressed fill, where the theme states button.default_button_background; elsewhere the Button is .primary(), pressed in ThemeColor button_primary_active (gpui-component/button/button.rs:1255, button_primary_active), which the connector sets to primary_active, a shade of primary (colors.rs)"
- Where gpui has no receiver:
  - `button.font.color`: "not reachable: the Button is .primary(), lettered in ThemeColor button_primary_foreground (gpui-component/button/button.rs:1039, button_primary_foreground)";
  - `button.active_background`: "not reachable: the Button is .primary(), pressed in ThemeColor button_primary_active (gpui-component/button/button.rs:1255, button_primary_active), which the connector sets to primary_active, a shade of primary (colors.rs)";
  - the three `default_button_*` leaves: "not reachable: the Button is .primary(): a ButtonCustomVariant paints its own rest fill as a fifth of its colour and its border in that colour in every state (gpui-component/button/button.rs:1028, Custom; :1118-1124, Custom), and a trial found it short of Breeze's default button (issue #4)";
  - `button.hover_border_color`: "not reachable: a Primary Button's hover and press border is the primary token (gpui-component/button/button.rs:1088, primary); at rest it is geometry::button's button.border.color (native-theme-gpui/geometry.rs:230, border_color)", 3F.4's text for `basic.buttons.primary`, verbatim.

A `.primary()` Button's border is the primary token under the pointer and pressed only:
its press and hover styles set it (`GC/button/button.rs:738-743`, `:751-762`, from
`ButtonVariant::border_color`, `:1088`), while at rest the caller's `geometry::button`
refinement, `button.border.color` (`CG/geometry.rs:230`), is replayed over the Button's
own style (`GC/button/button.rs:764`, `.refine_style(&instance_style)`).

Each range carries the symbol its lines hold, as `every_prose_citation_still_exists`
(`CG/showcase.rs:3762-3769`) requires of a `HOW` line: `line_citations` (`:3273-3387`)
reads `file.rs:N, Symbol` and each further `; :A-B, Symbol` group as ranges of the same
file, and `line_citation_finding` (`:3424-…`) passes a range only where its lines name
the symbol as an identifier (`mentions`, `:1028-1049`). Read today in gpui-component
0.9.1: `:1028` is `Self::Custom(colors) => colors.color.mix_oklab(…)` in `bg_color`, and
`:1118-1124` the `Self::Custom(colors)` arm of `border_color`, so each names `Custom`
and neither names the function it lies in (`bg_color` and `border_color` are named on
`:1014` and `:1084` only); `:1088` is `Self::Primary => cx.theme().primary,`, which
names `primary`.

**gpui's colour claims** (where gpui has a receiver). `info/buttons.rs`, the info of a
button built through `variants::default_button`: a function of its own beside `button`
(`:488`), `dialog_default`, which the helper above calls, since `edge` (`:339-…`)
branches on `ButtonKind` and the dialog default button has no kind of its own (6.G's
gpui paragraph). Its claims are in the form of `bordered_button`'s, which 3F.4 puts in
`at_rest`, `under_the_pointer` and a branch of `edge`: "bg" claims
`default_button_background` cited at the line of `geometry::default_button_rest` that
reads it; "hover" claims `default_button_background` at the line of
`variants::default_button_states` that reads it for `hover`; "active" claims the token
`button_active` at the line of `default_button` that calls `.active(` with
`cx.theme().button_active` (the line names the field); "text" claims
`default_button_text_color` at the line of `default_button_states` that reads it. Both
borders go through `edge_claim` (`info/mod.rs:87`), as `every_edge_colour_is_claimed_through_edge_claim`
(`CG/showcase.rs:2990`) requires of every edge colour and as 3F.4's `edge` arm does,
with `Some(b.border.line_width)`, so that no edge is claimed at width 0: "border"
`default_button_border_color` at the line of `default_button_rest` that reads it, and
"border, hovered or pressed" `hover_border_color` at the line of
`default_button_states` that reads it (decision 24). Its notes follow
`info::buttons::button`'s (`:488`): a variant line in the form of 3F.4's `Bordered` one
(`info.instance("variant", "native_theme_gpui::variants::default_button: a custom
variant with the default button's hover fill, label and hovered and pressed frame and
the plain button's pressed fill (button_active); its rest fill and frame are
geometry::default_button_rest's, refined after geometry::button")`), and a shadow note
in the form of `Bordered`'s, `info.not_themeable("shadow", "none:
variants::default_button builds its custom variant with .shadow(false)
(native-theme-gpui/variants.rs, default_button; button/button.rs,
ButtonVariant::shadow)")`, since the shared one (`:550-554`, "none on a standard
variant") is not true of a custom variant.
`every_colour_claim_is_read_at_the_line_it_cites` (`CG/showcase.rs:2819`) requires the
cited span to name the claimed field (`:2804`), which those lines do and the setter lines
of `default_button` do not. The lines it cites are pinned from then on, as the ones
"Lines that must not move" lists. The `HOW` lines and the claims land with the element
(6.I.1, U6.1).

**egui's routes.** The egui showcase's Widget Info takes each leaf's route from its
`mapping.toml` row (`showcase-egui/info.rs:390-…`, a leaf's key; `checked_by`, `:73-97`,
for a `tested_by` row), which 6.F.3 adds; no egui showcase test was found that
requires a route per element leaf beyond those rows (searched: `grep -n '^fn every_\|leaf'`
over `showcase-egui/tests.rs`, `info.rs`).

**The page with steps 4 and 5.** Their B.10 adds the checked tool button `basic.icons.edit`
to the icon row and its elements after `basic.icons.delete.icon`; this step's element
is the Buttons group's fourth row and goes into the element list after
`basic.buttons.toggle_on.label`. The places that name the Basic buttons (iced's names
table, `:29453-29454`; `drawing()`, `fn` at `:7341`, its `basic.buttons.primary` arm at
`:7357`) gain the element as above; steps 4 and 5's tests of the icon row are not
touched.

**The layout measurement** (decision 7), steps 4 and 5's B.10 measurement taken again,
with nothing new. The new row moves every group below Buttons in column 1 down by the
row's height and the group's row gap: in five columns Checkboxes, Radio buttons and
Drop-down; in four those and Expander, in three List and Expander, in two Switches,
Typography, List, Tabs and Expander besides (`BASIC_FIVE` to `BASIC_TWO`,
`showcase-gpui/pages/basic.rs:186-207`, and the list's header, `docs/showcase-elements.toml:654-668`);
in one every group after Buttons. Whether that pushes an element out of the page's
visible area at 1280x720 under some preset was not read, so it is measured:

- **What is reused.** B.10's nine entries (*ENTRIES*), its pre-check, its pre-build of
  each showcase before its block through the session's build slot, its command
  (`capture_showcases`, one block of nine runs per showcase, a failed block run again
  whole), its comparison script, its stop rule and its duration and announcement, as
  B.10 states them (`docs/todo_v0.7.x_kde-open-items-steps4-5-spec.md`, B.10, *The
  measurement* to *The duration*). Only the folders, the expected ids and the question
  of the comparison's items 5 and 6 differ, below.
- **When.** "Before": on the tree U6.1 starts from (the header; the release's branch
  as S2 makes it, from the predecessor's gated head), in the same worktree,
  before U6.1's first commit (no second build tree, no stash); "after": U6.1's last
  commit, once its gate has passed (6.I.1), before the merge of `<prev-tag>` and the
  release's first push (6.I.1). U6.2 and U6.3 change
  no showcase. One rule for a later change (rule 19, batch-speed rationale, Appendix):
  a fix of this release that touches the measured page's layout before the merge
  re-runs the "after" blocks on the same base (the dumps are kept until the first
  push); after the merge, such a fix is read by the rehearsal's after-only checks and
  named to the maintainer, never compared with the pre-merge "before" dumps. "Before"
  and "after" stand on one base, the branch's `<base>` (rule 17: `<lane-pred-head>`,
  or `<prev-tag>` where the branch was made from it), which the record names: the
  merge of `<prev-tag>` (S2 as amended) comes after this release's last unit before
  the merge, so after both, and what it brings is not re-measured. The "after" dumps
  stay of the measurement's subject, U6.1's last commit, and before they are deleted
  the record keeps the "after" state: per kind and entry, the listed ids present and
  the ids wholly inside their visible area. Instead of a new measurement, the
  release's rehearsal on a private X server, before the capture window, covering every
  entry the measurement used, runs the comparison's after-only checks on its dumps of
  the merged tree, relative to that state, not absolute, since a page that scrolls has
  elements outside its visible area by design: the entry list holds every entry the
  measurement used (exit 2 where one is missing or the list is empty); a dump for
  every entry and kind (exit 2 where one is missing); every id present at the "after"
  run present again and every id wholly inside then wholly inside again, an id that
  was already outside reported, not stopped on; the placement check of items 5 and 6;
  and the measurement's parity check of that placement across the three showcases; an
  entry the measurement did not use has its visible-area findings reported, not
  stopped on, its other checks holding as for the rest, its presence checked against
  the state of the same kind's light entry of that preset (an id a kind never records,
  as gpui's `NOT_RECORDED` in `showcase-gpui/tests.rs`, is not asked of it); and the rehearsal's summary
  holds no failed dump. They are needed since the parity checker counts an element
  that is not visible as ruled out, not as a difference
  (`scripts/check_showcase_parity.py:1077-1079`), and skips a property fewer than two
  kinds give (`:1080-1081`), so a missing dump goes unnoticed (rule 19); the asset
  run's parity checks cover the rest.
  The pre-check runs before each tree's first
  block; step 6 adds colours only, which it does not compare.
- **The folders.** `target/measure-6/before` and `target/measure-6/after`, in B.10's
  command in place of `target/measure-5b/before` and `…/after` (OUTPUT
  `"$PWD/target/measure-6/before/$kind-{theme}-{variant}.capture.png"`, DUMPS
  `"$PWD/target/measure-6/before"`, and the same under `after`). Deleted
  (`rm -rf target/measure-6`) after the release's first push (the batch's rule 19); kept
  while a stop waits for the maintainer.
- **The comparison.** B.10's script, items 1 to 4 and 7 as written, the list the U6.1
  tree's `docs/showcase-elements.toml`. Item 4's expected added ids are
  `basic.buttons.dialog_default` and `basic.buttons.dialog_default.label`. Items 5 and
  6 ask, in place of Edit's wrap: in each "after" dump, does `basic.buttons.dialog_default`
  lie below the toggle row, its rounded top edge not less than `basic.buttons.toggle_off`'s
  rounded bottom edge (`snapped_edges`, `scripts/check_showcase_parity.py:935-943`)? Where
  it does not, or either id is missing, a stop entry; so is `basic.buttons.toggle_off`
  wholly inside its visible area there and `basic.buttons.dialog_default` not (the stop
  rule); where the three showcases do not all answer yes under an entry, a parity entry.
- **What a stop means.** B.10's stop rule unchanged: an element whose id starts
  `basic.`, wholly inside its visible area (`visible_area`, `:776-787`: the window's
  content cut to `chrome.content`) in the "before" dump and not wholly inside it in the
  "after" one, or present before and missing after, stops the work before the push, and
  the maintainer decides. So does the new button, which no "before" dump holds, where in
  an "after" dump `basic.buttons.toggle_off` lies wholly inside its visible area and
  `basic.buttons.dialog_default` does not. A parity entry is no stop and no question
  (the batch's rule 19): a fault, fixed and the "after" blocks run again, or, where it
  is a toolkit's limit, excepted with its reason and source, the maintainer told in
  either case. A move that keeps every element
  inside goes on; the comparison's "moved" lines are the plan's list of what the row
  moves, which the asset run's captures then show.
- **The record.** This step's plan's *Measurement record*, in B.10's form (the folders,
  the pre-check's result, each block's time, the reruns, the entries whose icon set
  fell back, the printout), committed in a commit of the plan's file only on top of
  U6.1's last commit, and pushed with the release's first push, which comes after the
  merge of `<prev-tag>` (S2 as amended); the merge changes no sha, so the record's shas
  stay valid.
- **The parity keys the new row brings.** The asset run's parity step compares the new
  element and its label like every other button, so the differences the page's other
  buttons already have are expected for them too: the label's height (`.label.h`,
  under adwaita and catppuccin-mocha, as every Buttons-row label has it in
  `docs/showcase-exceptions.toml`'s sub-pixel line-box group) and the positions and
  sizes the other buttons have keys for (`.x`, `.y`, `.w`, in the file's existing
  groups for them), and any position key of an element the row moves down. They
  are not written by hand: the rehearsal's or the asset run's `--merge`
  (`check_showcase_parity.py --merge OUT --write`) proposes them into the groups whose
  reasons hold for them, and the proposal is reviewed and the maintainer told
  (rule 19). Only the colour group above, where gpui has no receiver, is written here.

**Widget Info.** The info names the functions each showcase calls and follows the rule
of 6.A.1 through them; no showcase writes a prose of its own for the element:

- **iced:** the title "Button · Default" (the names table above), and per leaf the
  route `colour_route` reads from the contract export (the `drawing()` arm above),
  which names `styles::button_default`'s rows; where the fill is stated,
  `button.active_background` and `button.active_text_color` name `styles::button`'s
  `Pressed` rows instead (the pressed arm above), and where it is not, the rows of
  `styles::button_default` give the primary pair, as 6.E.3's rows do.
- **egui:** the title is the element's kind, "button (dialog default)" (the kind above;
  egui's info title is `shown.info.kind`, `showcase-egui/info.rs:747`), and per leaf
  its `mapping.toml` row (*egui's routes* above): for the three new leaves the rows
  6.F.3 adds (`verdict = "derived"`, `tested_by = "T18(a)"`, shown through
  `checked_by`), for the button's other leaves their rows as they stand.
- **gpui:** the claims above: `variants::default_button` with
  `geometry::default_button_rest` where gpui has a receiver, the primary button's info
  where it has none.

**Screenshots.** The Basic page's Buttons group gains a fourth row, one button, on
every preset, and the groups below it in column 1 move down by that row (the
measurement's "moved" lines); under kde-breeze the button is Breeze's default button
(iced, egui, and gpui where it has a receiver), elsewhere the primary one.

## 6.H Outside the code (D10)

### 6.H.1 `CHANGELOG.md`, the release's one `## [0.7.N]` section (D0 (a); S5)

The release of both steps has one section; step 7's entries (7.I.1) join it. Its
heading is written as `## [0.7.N] - <date>` with the release's proposed number, the date
filled at the version step, never `## [Unreleased]`; the section stands above every
other `## [` heading, and at the merge of `<prev-tag>` a conflict there is resolved by
putting this release's section first (the batch's rule 14).
`### Breaking Changes`, one entry for both steps (S5: the break named once), in 0.7.0's
words (`CHANGELOG.md:66-68`):

```markdown
- **native-theme**: `ButtonTheme` gains three fields, `default_button_background`, `default_button_text_color` and `default_button_border_color`, `ListTheme` gains `header_hover_background` and `header_active_background`, and `ThemeDefaults` gains `selection_inactive_text_color` (see *Added*). None of the three is `#[non_exhaustive]`, so a struct literal of one outside the crate needs the new fields or `..Default::default()`; their resolved forms, `ResolvedButtonTheme`, `ResolvedListTheme` and `ResolvedDefaults`, are `#[non_exhaustive]` already.
```

Step 6 writes the entry when its documents unit runs, with the step 7 clause as given:
the section describes the release, and the release is not cut before step 7 has
landed (7.J.4). `### Added`, step 6's entries (step 7's follow them, 7.I.1):

```markdown
- **native-theme**: a dialog's default button, the one Enter activates, has fields of its own: `button.default_button_background`, `default_button_text_color` and `default_button_border_color`, each optional. Where a theme states the fill, the default button is that fill, lettered in the text colour (else the button's) and framed in the border colour (else the button's); where it states none, the default button is the primary one, as on macOS and Windows. kde-breeze and the KDE reader state Breeze's, `mix(Button, Highlight, 0.2)` framed in `mix(Highlight, mix(Button, ButtonText, frameContrast), 0.5)` under the button text, #d6ecf8 / #2d4655 framed #87c0dd / #4882a1; `button.primary_background` and `primary_text_color` keep the accent pair on every platform.
- **native-theme-iced**: `styles::button_default(&resolved)`, a dialog's default button: Breeze's under kde-breeze, `styles::button_primary` where the theme gives the default button no look of its own.
- **native-theme-egui**: `default_button(&resolved)`, the fill, label colour and stroke to hand a `Button` where the theme gives a dialog's default button a look of its own, and the strokes for a button whose previous response was hovered or pressed (`hover_stroke`) or had the keyboard focus (`focus_stroke`); `None` where it is the primary button (a selected button in `Role::Button`). A pressed default button is the plain pressed button, which the `Role::Button` scope draws for a button given no fill.
- **native-theme-gpui**: `variants::default_button(n, cx)`, a custom variant for a dialog's default button that the application builds, Breeze's under kde-breeze, and `geometry::default_button_rest(n)`, the refinement that gives it its rest fill and frame after `geometry::button` (the README shows how); both `None` where the theme gives the default button no look of its own, and the button is the primary one. It goes into a plain `Dialog`'s own footer; an `AlertDialog`'s default footer builds its OK button from a `ButtonVariant` alone and keeps the primary button.
- **Showcases**: the Basic page shows a dialog's default button, in a row of its own below the other buttons.
```

The gpui line is written only where gpui has a receiver (6.D.2: step 3's trial and
6.D.4's test both passed); otherwise there is no gpui line and *Added*'s first line ends "…; gpui-component has
no receiver for it, so in gpui a dialog's default button stays the primary one." The
egui line holds for egui 0.36 and 0.37 alike (the per-call override, decision 6).

### 6.H.2 `ROADMAP.md`

The release's one paragraph for 0.7.N (S5), in the place the roadmap gives each
release (`## v0.7.1 — …`, `:85`, is the last release heading; the release's own heading
follows it), states both steps: it opens with step 6's sentences, "A dialog's default
button has fields of its own: Breeze's tinted default button under kde-breeze, the
primary button where the platform draws that, as macOS and Windows do. The iced and egui
connectors offer it[, and the gpui connector a variant for a default button the
application builds]." (the bracket where gpui has a receiver, 6.D.2), and goes on with
step 7's sentence (7.I.2) in the same paragraph, which step 7's documents unit appends.
The sentence of the 0.7.1 paragraph "The platform facts for the focus ring and the
default button are corrected too, and their values wait for a connector design" (as
step 1 leaves it) loses "and the default button" (and the focus ring, if step 2 has
removed it already, leaves the sentence to go whole).
`ROADMAP.md:162`'s count of the iced connector's style functions ("has 34 style
functions" after step 5) moves to the count 6.E.4 writes, in place, found by its text,
and the same paragraph's list of the functions by name (`:159-178` at `0fa28027`) gains
"`styles::button_default` a dialog's default button" after step 5's
`styles::toolbar_button` entry, in the list's own form.
`ROADMAP.md:264`'s count of open `[v0.7.x]` entries does not change at step 6: 6.H.5
opens one `[1.0]` entry and closes none (its item 1's `[v0.7.x]` entry stays open), so
U6.3 recounts nothing; where an edit of the unit does open or close a `[v0.7.x]` entry
after all, it recounts as 7.I.2 says.

### 6.H.3 The first rationale, `docs/issues/0004_kde-hand-overrides-rationale.md`

- **§5.2** (`:874`), a line after the heading: "*Settled by `0004_kde-open-items-rationale.md`
  D8 (0.7.N): the default button has fields of its own, which kde-breeze and the KDE
  reader state; the primary pair keeps the accent.*"
- **§9** (`:1164`), the row "Default button, fill / border / label": its third cell
  "the same (§5.2)" stays true; no edit.
- The default button is not one of the reporter's items (§7 has no row for it), and the
  reply (§8) does not mention it: no edit there.

### 6.H.4 The rationale's status line

With both steps in one release (S5), the rationale's status line gains one sentence
for the release, written at its close-out with the archive's path: 7.I.4's, which names
step 6 (D8) beside step 7.

### 6.H.5 `docs/todo.md`

1. **"KDE values that wait for a connector design"** (`:1826-1913`): item (2)
   (`:1856-1868`, "(2) **The default button** …") becomes "(2) **The default button**:
   done in 0.7.N (below)." After the entry's last line, a dated line in the file's
   convention (`*<what> (<date>, <sha>):*`, header `:1-21`): "*Issue #4, open items step
   6 (0.7.N, <date>, <sha>):* (2) done (`docs/issues/0004_kde-open-items-rationale.md`
   D8): `button.default_button_background`, `_text_color`, `_border_color`, stated by
   kde-breeze (`kde-breeze.toml` `[light.button] default_button_background`,
   `[dark.button] default_button_background`) and the KDE reader; iced
   `styles::button_default`, egui `default_button`, gpui `variants::default_button`."
   Where gpui has no receiver (6.D.2), the last clause is instead "gpui: none, the
   primary button (its README's *Limits*)". The entry stays while item (5) is open.
2. **A new entry** (decision 15), `[1.0]`, in the model section beside *Selection: the
   text colour macOS pairs with the unemphasised selection* (`:543`), after it: after
   that entry's last line (`:580`) and its blank line, before `### Presets: colours with
   no reproducible source` (`:582`), each found by its text: "### Model: the
   accent a control is filled with" — "- [ ] [1.0] D8 of `docs/issues/0004_kde-open-items-rationale.md`
   found no field that is, on every platform, the colour a control is filled with when
   it is accented: `defaults.accent_color` is the raw accent on Windows, where controls
   take another shade (windows-11 #0078d4 against #0067c0 / #4cc2ff for the primary
   button and the checked check box), and the check box's checked fill is a tint on KDE
   (#c2e4f6). gpui-component reads its `primary` token as that colour in thirteen widget
   files, and the connector fills it from `button.primary_background`. A field for it
   would let `primary` follow it rather than the primary button." (The values are D8's;
   the check box's #c2e4f6 was not re-read.)
3. **"What Breeze draws that the model has no field for"** (`:1774-1825`, a `[v0.7.x]`
   entry that stays open), a line after its last: "*Issue #4, open items step 6 (0.7.N,
   <date>, <sha>):* Breeze tints a dialog's default button only in an active window
   (breezehelper.cpp:700-702 at breeze f0b1d75, `isActiveWindow && defaultButton`; the
   parked facts' `docs/facts/kde/button.toml:1342` on `v0.8.0-rc1` at `78ee0efd`); the model's
   `button.default_button_*` fields hold the active look, and the connectors draw it
   whatever the window's state." A line in an open entry, not an entry of its own, so
   the roadmap's count does not change (6.H.2).

### 6.H.6 `docs/archive/`

Nothing moves at step 6; the release's close-out (7.J.4, 7.I.6) archives this
specification and its plan once, at the end.

## 6.I Order, gates, close-out

### 6.I.1 What depends on what

- Step 3's trial result (3F.4 or 3F.5) is recorded before 6.D is implemented; where
  it passed, 6.D.4's test decides 6.D.2's condition before anything that reads it
  (6.D.3, 6.D.5, 6.F.5's reason, 6.G's gpui button, `HOW` lines and exception, 6.H) is
  written, and its result is recorded beside the trial's.
- Step 5's measurement recorded before step 6's "before" dumps (6.G, *The layout
  measurement*): steps 4 and 5's measurement-record commit (their 5b unit's record, on
  top of 5b) is an ancestor of the branch's head; its dumps lie in that release's own
  worktree, not in this one.
- The release's branch is made from the gated head of the release before it in its own
  lane (the KDE lane: v0.7.1 → steps 2 and 3 → steps 4 and 5 → the caret → steps 6 and
  7, S3 as amended; here the caret's, `<lane-pred-head>`: its gated head, its
  documents unit's commit past its `--quick` with its reviews closed, the batch's rules 18
  and 20), not from its tag. Once this release's last unit before the merge (U7.4,
  after U7.3) has passed its gate (the five checkers and `--quick`: this is the KDE
  lane's last release, which rule 20 gives no gated head), every review is closed, and the tag
  of the release immediately before it in the maintainer's release order, `<prev-tag>`
  (under the release order the maintainer approved with S3 on 2026-11-15, the caret's, `v0.7.6`, which holds the widget
  icons and egui 0.37 too), exists, that tag is merged into the branch, once:
  `git -c merge.conflictStyle=diff3 merge --no-ff --no-commit <prev-tag>`, the merge
  commit made only after the resolutions and regenerations (S2 as amended 2026-11-15:
  a merge, not a rebase, so every sha stays; the batch's rule 17). Elsewhere in this
  document "the predecessor's tag" is `<prev-tag>`. The merge comes before the
  release's recount (X, the plan's), version and close-out, never while a unit's
  edits are uncommitted and never between 6.G's "before" and "after" runs (both lie
  before it). Since the branch starts on the caret's gated head, the caret's facts
  rows are in it already, and the merge brings the caret's post-gate commits, the
  widget icons and egui 0.37. Conflicts are resolved by the orchestrator by both sides'
  quoted texts, a citation like any other text (S6: an old line number in a conflict
  takes either side's text, both drift alike, and a new citation has none), each
  resolved hunk named in the record; a hunk whose sides differ only in a count stated
  in prose is resolved to `<prev-tag>`'s side and recounted after the merge; a count or
  verdict tuple a test or gate checks (6.F.3's leaf count and group totals, 6.C.4's
  `derived_leaves` count, the verdict tuples) is the base plus both sides' deltas.
  Generated files are regenerated by their commands, every resolved and regenerated
  file is added, no conflict marker may be staged (the plan's marker check), and the
  staged paths are checked against the tag's changes before the merge commit (rule 21
  (1)). No citation is re-pointed at the merge or after it (S6; rule 23 is superseded). A
  resolution in a code file is reviewed; the warm-up gate (`--commit` on the clean
  tree) comes last. After it, every stated count (6.H.2's and 7.I.2's,
  `ROADMAP.md:264`'s) in a file either side changed, outside the pinned documents, is
  recounted on the merged tree, and the five documentation checkers run (rule 17;
  with no merge there is no recount). A toolkit version the merge
  brings (a changed `Cargo.lock` version of egui, iced or gpui-component, say egui
  0.36.2 → 0.37.0) re-reads every upstream line this document cites in that crate and
  every version form the units wrote (this document's egui-version line citations, the
  header) against it (the batch's rule 19); the re-writing, only where the content
  changed, never of a version-pinned citation that still holds but for a line a gate
  reads by number, is rule 25's task VF, inside
  the merge's step (3), after the version comparison and before the warm-up gate,
  reviewed within the merge's review and covered by the warm-up gate (rule 25: egui
  0.37 and the other lane's releases normally reach this branch only at the merge),
  and a version outside the plan's accepted set is a stop. Before the merge, the tag's
  sha is compared with the remote's (the first field of
  `git ls-remote origin 'refs/tags/<prev-tag>'` against `git rev-parse <prev-tag>`, the
  tag ref's own object, equal for the project's lightweight release tags and for an
  annotated one; no line printed is a failure too; rule 21; the same check runs where
  the branch is made from `<prev-tag>`), and `<lane-pred-head>` is checked to be an
  ancestor of `<prev-tag>` (rule 18); either failing is a stop. Nothing
  of this release is pushed before the merge, and its close-out starts only after it.
  Where the branch was made from `<prev-tag>` itself, there is no merge. After the
  merge, a fix of this release that can move a `basic.` element (any change to the
  Basic page's or a showcase's layout; a comment-only edit does not) is not compared
  with the pre-merge "before" dumps: it re-runs the rehearsal and its after-only
  checks (6.G), and is named to the maintainer (rule 19).

**Units.** Each unit is one or more commits; nothing of a unit is left for the next.
Its gate follows the batch's cadence (S1; `--quick` as
`docs/todo_v0.7.1_gate-speed-spec.md` defines it): during a unit, the agent runs only
the touched tests (its targeted commands); at a unit's end, `--quick`, by the
orchestrator, as a long run until its time is measured (the first timed into the
record); `--commit` every third code unit of the release, counted from the last
`--commit` (`docs/platform-facts.md` counts as code: rule 11), on the release's last
code unit before its documents and version units (U7.3, rule 6), after a lock or
toolchain change (the warm-up gate; after the merge too; the version step, whose
`Cargo.lock` changes; any `--commit` restarts the count of three), and
wherever the plan names a unit structural (one that changes a manifest, `Cargo.lock`, a
build script, the toolchain or the gate scripts, rule 24; new model fields are no
reason on their own); a documents-only unit (one that changes nothing but comments,
rustdoc prose, READMEs and Markdown: U6.3, U7.4) runs the five documentation checkers
and `--quick`, U7.4 too: it is the last unit before the merge, but this is the KDE
lane's last release, which rule 20 gives no gated head, so it takes the cadence's gate;
the full run where the plan
has it (the close-out), unchanged. A failure the next `--commit` finds is fixed by a new
commit, never by rewriting a pushed one, and the record names the commit that brought
it. "Why together" below names the tests that fail between a unit's pieces; each is in
the unit's `--quick` (the tests of every crate the unit changes and of their
dependents) as in `--commit`.

- **U6.1 — the fields, everywhere they are counted and shown.** 6.A (the field, the
  struct literals, the `presets.rs` assertion); 6.B (registry, rules, the facts rows,
  placed by Part F); kde-breeze's values (6.C.3); gpui 6.D entire (where gpui has a receiver:
  `default_button`, `default_button_states`, `geometry::default_button_rest` with its
  `GEOMETRY_NOTES` entry, the tests, the two `sinks` entries, 6.D.5's README paragraph
  and row and the `lib.rs:112` row; where it has none: the three `UNREAD_COLOURS`
  entries with 6.D.5's *Limits* line and, in case 1 of 6.D.2, the request-5 sentence:
  the documentation with the code it describes); iced 6.E entire (the style, its rows, pairs, `DERIVED` entries, the
  regenerated `contract-leaves.toml`); egui 6.F entire (the accessor, the three mapping
  rows, the counts, `mapping.md`, the install test, the parity exception); and 6.G
  entire (the two elements of `docs/showcase-elements.toml`, the three showcases'
  buttons, their Widget Info and claims, gpui's `HOW` lines, iced's route arms and the
  tables of 6.G's list, and their tests; where gpui has no receiver, the `[parity]`
  exception), with 6.G's layout measurement: its "before" blocks before the unit's
  first commit, its "after" blocks and comparison after the unit's gate and before the
  merge and the release's first push, and its record committed
  on top of the unit and pushed with that push (S2 as amended: after the merge).
  Why the showcases are in it: iced's `styles_cover_every_widget_shown`
  (`showcase-iced.rs:20919`) fails every public function of `src/styles.rs` with no call
  site in the showcase (its loop `:20969-20981`), so `styles::button_default` is red
  without the iced button; gpui's `the_showcase_exercises_every_builder`
  (`CG/showcase.rs:384-416`) fails every `pub fn` of `variants.rs` and `geometry.rs` the
  showcase does not use, and `every_geometry_builder_has_a_note` (`:572`) every
  `geometry.rs` builder without a `GEOMETRY_NOTES` entry, so `variants::default_button`
  and `geometry::default_button_rest` are red without the gpui button and its note;
  and once the element is listed, every showcase must draw it and route its leaves
  (`check_showcase_parity.py --check-list`; gpui's `every_listed_leaf_has_a_route`,
  `showcase-gpui/tests.rs:15198`; iced's `the_element_list_parses_and_every_leaf_has_a_route`,
  `:24982`, whose routes for this element are the contract export's through
  `drawing()`; egui's `the_layout_dump_names_every_listed_element_the_basic_page_draws`,
  `showcase-egui/tests.rs:8037`), so the egui button comes too. egui's new accessor has
  no such test: no egui test that every accessor is used by the showcase was found
  (searched: `git grep -nE '^\s*fn (every|each|all)_'` over `connectors/native-theme-egui/src/`
  and the showcase's tests, read for what each covers). Why the rest is together, test
  by test: `property_registry_keys_match_derive_inventory`
  fails between the field and its registry keys; `check_platform_facts_rows.py`
  between the keys and the rows; the resolved theme serialises an unset soft option as `null`, so egui's
  `every_leaf_has_exactly_one_row_and_every_row_is_well_formed`
  (`connectors/native-theme-egui/src/mapping_tests.rs:234-262`) counts the three new
  leaves from the field on and fails until their rows land; its T2
  (`t2_every_row_s_declared_sinks_are_exactly_what_its_leaf_moves`) fails a row that no
  preset pair differentiates and that has no `probe`, which holds until kde-breeze
  states the leaves; the egui showcase's `the_theme_map_names_the_tests_that_check_a_per_call_route`
  (`showcase-egui/tests.rs:6872`) fails a `tested_by` row until its install test exists;
  gpui's `every_colour_leaf_reaches_a_sink_or_is_named` and iced's
  `every_colour_leaf_reaches_a_row_or_is_named` (against `contract-leaves.toml`) fail
  each new colour leaf (`null` included, `contract.rs:2692-2706` in gpui) until a sink,
  a row or an entry names it; `check_connector_parity.py` fails egui's new crate-root
  function until its exception lands.
- **U6.2 — the KDE reader.** 6.C.2 and 6.C.4, the moved-leaves tests' new lists among
  them (the reader moves those leaves, so the lists change with it). `the_derived_colours_are_the_presets_under_breeze`
  compares only the leaves `derived_leaves` lists, so U6.1 passes without the reader,
  and U6.2's new leaves equal U6.1's preset values.
- **U6.3 — outside the code.** 6.H.

### 6.I.1b Order inside the units

- 6.B with or before 6.C.3 (a preset comment cites a facts row).
- 6.C.3 before the iced `contract-leaves.toml` is regenerated (the walk reads the
  presets).
- 6.H's backlog lines with or after 6.C.3 (`check_todo_citations.py`); 6.H.1-6.H.4 after
  the code.

### 6.I.2 The citations the edits move

None is re-pointed (the batch's S6): the edits of 6.A, 6.B.2, 6.C.2, 6.E.2, 6.F.2 and
Part F move lines other files cite by number, and those citations are left as they are
and may drift. The lines the gpui colour claims pin do not move (*Lines that must not
move*), and no gate reads the moved citations by position once v0.7.1's LN unit has
landed (Part F, F.1).

### 6.I.3 Gates

The commands of step 1's H.3 (`scripts/check_release.sh` at `0fa28027`), all of them;
what each catches here:

| Gate | Catches |
|---|---|
| `cargo test -p native-theme --features "$NT_FEATURES"` and without | 6.A.4's model gates; 6.C.4; the preset assertion of 6.A.3 |
| the gpui legs | 6.D.3 (either form), 6.D.4, the claims test (no pinned line moved, the new claims at the lines that read their fields), `the_showcase_exercises_every_builder` and `every_geometry_builder_has_a_note` (where gpui has a receiver, 6.D.2) |
| the iced legs, with and without `iced_aw` | 6.E.3's rows, pairs, `DERIVED`, the leaves file; `styles_cover_every_widget_shown` (the showcase's call of `styles::button_default`) |
| the egui legs | 6.F.3, 6.F.4, `the_theme_map_names_the_tests_that_check_a_per_call_route` (`showcase-egui/tests.rs:6872`) |
| the showcase tests | 6.G's routes and element |
| the documentation checkers; `check_connector_parity.py` and its self-test; `check_showcase_parity.py --check-list` | 6.B, 6.H.5, 6.F.5, 6.G |
| clippy, strict panic, `check_panic_patterns.py`, fmt, rustdoc | the new code |
| full run: both stamps | stale from the first commit until the asset run |

**Not covered by any gate:** the wording of the docs, CHANGELOG, roadmap, backlog and
rationale texts; the facts cells against their sources; the citations the edits move,
left as they are (S6); the look of the default button in each showcase (the screenshots, read by the
maintainer before the asset run is committed).

### 6.I.4 The release

Step 6 has no release of its own: the release of both steps (S5) is cut after step 7's
units, and its one close-out is 7.J.4's. Tag, publish and release are the maintainer's.

---

# Step 7 — the table header's states (D7) and the unfocused selection (D9)

## 7.A The model

### 7.A.1 `ListTheme`, `native-theme/src/model/widgets/mod.rs:603-650`

After `header_height` (`:626-631`):

```rust
    /// Fill of a hovered column-header section, in place of
    /// `header_background`: Breeze's `Button` mixed a fifth towards the hover
    /// colour. `None` where the platform states none: a hovered section keeps
    /// `header_background` (docs/platform-facts.md §2.15).
    #[theme(category = "soft_option")]
    pub header_hover_background: Option<Rgba>,
    /// Fill of a pressed column-header section, in place of
    /// `header_background`: Breeze's `Button` mixed a fifth towards the focus
    /// colour. `None` where the platform states none: a pressed section keeps
    /// the hovered fill, else `header_background` (docs/platform-facts.md
    /// §2.15).
    #[theme(category = "soft_option")]
    pub header_active_background: Option<Rgba>,
```

"Unset", exactly: a hovered section is `header_hover_background.unwrap_or(header_background)`;
a pressed one `header_active_background.or(header_hover_background).unwrap_or(header_background)`
(decision 19: a pressed section is under the pointer, as D5 says of a pressed button).

`native-theme/tests/proptest_roundtrip.rs`, `arb_list_theme` (`:403-434` at `0fa28027`):
the colour vector grows by two and the fields take the next two indices (at
`0fa28027` the vector is `9..=9`, so `colors[9]` and `colors[10]`; no earlier step's
specification read today adds a `ListTheme` field). The only full `ListTheme` literals
(scanned as for `ButtonTheme`): this one; `merge_behavior.rs:416` uses `..Default::default()`.

### 7.A.2 `ThemeDefaults`, and how it reaches `ResolvedDefaults`

`ResolvedDefaults` is written by hand (`resolved.rs:74-152`) and built by the
`validate_defaults!` macro (`resolve/validate_helpers.rs:666-740`), which names each
field once in `validate.rs:46-89`; `selection_inactive_background` runs end to end
through: the field (`defaults.rs:71-72`), `impl_merge!` (`:153-168`, `:159`), the inheritance
`selection_inactive_background ← selection_background` (`resolve/inheritance.rs:98-101`;
`docs/inheritance-rules.toml:58`), the macro's `option_color` list (`validate.rs:60`),
the resolved field (`resolved.rs:106-107`), the registry (`docs/property-registry.toml:113`)
and the baselines and tests named below. The new field follows it at each place:

| Where | Edit |
|---|---|
| `native-theme/src/model/defaults.rs:71-72` | after it: `/// Text color over \`selection_inactive_background\`, the selection of a view without the focus; unstated, \`selection_text_color\`.` and `pub selection_inactive_text_color: Option<Rgba>,` |
| `defaults.rs:159` (`impl_merge!`) | `selection_inactive_background,` becomes `selection_inactive_background, selection_inactive_text_color,` |
| `defaults.rs:194` (`default_has_all_none_options`) | after it: `assert!(d.selection_inactive_text_color.is_none());` |
| `native-theme/src/model/resolved.rs:106-107` | after it: `/// Text color over \`selection_inactive_background\`.` and `pub selection_inactive_text_color: Rgba,` |
| `resolved.rs:366` (`sample_defaults`) | after it: `selection_inactive_text_color: c,` |
| `native-theme/src/resolve/validate.rs:60` | after it: `selection_inactive_text_color,` |
| `native-theme/src/resolve/inheritance.rs:98-101` | after the block: `// selection_inactive_text_color <- selection_text_color` and `if d.selection_inactive_text_color.is_none() { d.selection_inactive_text_color = d.selection_text_color; }` |
| `inheritance.rs:381-389` (`implemented_targets`) and `:826-834` (`all_inherited_targets`) | `"defaults.selection_inactive_text_color"` after the inactive background's entry |
| `native-theme/src/model/mod.rs:1988` (`theme_defaults_field_info_matches_baseline`) | after it: `"selection_inactive_text_color",` |
| `native-theme/src/resolve/tests.rs` | after `resolve_phase1_explicit_selection_preserved` (`:117-132`): `resolve_phase1_selection_text_fills_selection_inactive_text` (`selection_text_color` set, the new field `None`: it inherits) and `resolve_phase1_explicit_selection_inactive_text_preserved`; the list of derived fields cleared (`:2343-2345`) gains `v.defaults.selection_inactive_text_color = None;`; and `fully_populated_variant` (`:780-790`), which sets every derived defaults field by hand because its callers run `validate()` without `resolve()` (`:279`, `:285`, `:311`, `:327`, `:339`, `:354`, `:381`, `:401`, `validate_fully_populated_returns_ok` at `:1299-1306`, and the later ones from `:1314`), gains after `:787` (`selection_inactive_background`) `v.defaults.selection_inactive_text_color = Some(Rgba::rgb(255, 255, 255));`, the value its `selection_text_color` line gives (`:786`): without it `validate()` reports the required field missing. The soft options of 6.A.1 and 7.A.1 are `Option`s in the resolved structs, which `validate()` does not require, so the helper needs none of them |
| `native-theme/tests/proptest_roundtrip.rs:153-185` | the `ThemeDefaults` literal: the field takes the next unused index of the colour vector, growing the vector where none is unused (at `0fa28027` the vector is `24..=24`, `:136`, and indices up to 22 are used, so `colors[23]`; step 2's field is a `bool`, its own tuple member, as the steps 2-3 specification's bullet on `arb_theme_defaults` adds it); the only full `ThemeDefaults` literal outside `resolved.rs` (scanned as for `ButtonTheme`) |
| `native-theme/tests/serde_roundtrip.rs:85` | after it: `v.defaults.selection_inactive_text_color = Some(c(35, 38, 41));`. No assertion of its own: the test round-trips the whole theme and compares it whole, so a field that does not survive fails it |

The field resolves on every theme, since `selection_text_color` is a root
(`docs/inheritance-rules.toml:387`): it is `Rgba` in the resolved struct, not an option.
`ResolvedDefaults` is `#[non_exhaustive]` (`resolved.rs:75`); `ThemeDefaults` is not,
which is D0's break. The macOS reader builds `ThemeDefaults` with `..Default::default()`
(`macos.rs:145-177`) and compiles unchanged.

### 7.A.3 Gates

6.A.4's, and `inheritance_rules_toml_covers_defaults_internal` (`inheritance.rs:368-408`),
which fails until the rule is both in the TOML and in `implemented_targets`.

## 7.B The carried clean-room modules

**The rule.** The files are carried as they are, byte for byte below an added notice;
nothing is re-derived from KDE's, Qt's or GTK's sources. `kde_selection_inactive.rs`
(P8, 27 lines) is not carried: it takes P8's `crate::tokens::KdeRoleKind`, which this
tree has not, and D9 carries its rule rewritten for the reader (7.D.2), checked against
its table.

**What step 1 carries already**, and is not carried twice: `kde_shade.rs` as
`native-theme/src/kde/shade.rs`, its table `native-theme/tests/vectors/kde_shade.tsv`
(which creates the folder) and its declaration in `kde/mod.rs` (step 1's A.1-A.3). The
modules below share nothing with it: `kde_shade.rs` uses no other module (step 1's Part
A), and `kcu_tint` uses `kde_color_common` alone. **What this part needs from step 1**:
the folder `native-theme/tests/vectors/`, and the declaration `mod shade;` in
`kde/mod.rs`, after which 7.B.3's go; nothing of `shade.rs` itself (its test helpers are
private to its test module, so 7.B.4 has its own). **The pattern step 1 sets, followed
here**: the file goes under `src/kde/` (its decision 1; the names, decision 8 here); the
notice is a paragraph of the carried file's own module doc, after its first lines (its
decision 2); the dead-code allow goes on the module's declaration, never into the file
(its decision 3; here unconditional, decision 9).

### 7.B.1 Sources (read today)

| P8 file | Lines | sha256 | Its uses |
|---|---|---|---|
| `native-theme/src/platform/kde_color_common.rs` | 306 | `dfd4a24592585b8cf5b6eaa23ac30e4850f4a62414907190163a15630dd70803`; lines 4-306: `49e81317144879dc0806d20d0fd60e48340670f52392e7239801387413897498`; blob `47d7bbe2` | no crate type, no other module; `pub(super)` items: `Rgba`, `QtColor`, `K_R`, `K_G`, `K_B`, `q16`, `round_to_half`, `from_rgbf`, `from_rgbd`, `enter`, `read`, `TRANSPARENT`, `norm`, `maxd`, `mind`, `Hcy`, `luma`, `to_hcy`, `from_hcy`, `mix_q`, `tint_q`, `shade_q` |
| `native-theme/src/platform/kcu_tint.rs` | 10 | `977554bb9d71cd249488323b92ebe7447902f08c4aef76b743231218e7c38677`; lines 2-10: `b9f0259536b86d9b7499abbb319f65822576a985ee651b9345a37b99b0c81719`; blob `300fdec6` | `use super::kde_color_common::{Rgba, enter, read, tint_q};` (`:3`) |
| `native-theme/tests/vectors/kcu_tint.tsv` | 139 (124 rows) | `213587717f31966d7d45150184f5f334d39284f72b7fcf3eb857ef8a36402072` | header `:1-14` names kguiaddons 6.31.0 and qtbase 6.12.0, route `compiled_source`, generator `tools/kde-vectors`; columns `a b amount result` |
| `native-theme/tests/vectors/kde_selection_inactive.tsv` | 84 (70 rows) | `0bbe9d9eb1132d60701ba5fab88c3f0a1a4fa280ae8bcc3963aedc4c48325578` | kcolorscheme, kguiaddons 6.31.0, qtbase 6.12.0; columns `selection window selection_background role_kind inactive_change_selection_color result` |

The hashes were computed today with `sha256sum` over the files in `P8`'s working tree
(`tail -n +4` and `tail -n +2` for the bodies); the blobs are `git rev-parse HEAD:<path>`
at `78ee0efd`. The checks below compare against them. The notice of all of P8's KDE colour functions is in
their parent, `P8 native-theme/src/platform/mod.rs:1-18` ("…the seven KDE colour
functions (`kcu_mix`, `kcu_tint`, …, their shared arithmetic in `kde_color_common`) …
were written in a clean room, from behavioural descriptions and the vector tables alone
(the plan's D10)").

### 7.B.2 The new files

Each written with the Write tool (the project's Write hooks must see it), as step 1's
A.1:

- `native-theme/src/kde/kde_color_common.rs`: P8's lines 1-3 (its module doc), the
  notice below, whose first line is the blank `//!`, then P8's lines 4-306 byte for
  byte.
- `native-theme/src/kde/kcu_tint.rs`: P8's line 1 (its module doc), the notice, whose
  first line is the blank `//!`, then P8's lines 2-10 byte for byte.

The notices, one per file, since `kde_color_common` has no table of its own (P8's
module notice says the shared arithmetic serves the KDE colour functions,
`native-theme/src/platform/mod.rs:1-18` on `v0.8.0-rc1`, and no vector table names
it). `kde_color_common.rs`:

```text
//!
//! Written in a clean room on the parked 0.8.0 branch, from behavioural
//! descriptions and the vector tables alone, and carried here unchanged but for
//! the clippy fixes its commit names, below this paragraph
//! (`native-theme/src/platform/kde_color_common.rs` and its notice in
//! `native-theme/src/platform/mod.rs:1-18` on `v0.8.0-rc1` at 78ee0efd). No line
//! of KDE's or Qt's code is in it. It has no table of its own: its arithmetic is
//! checked through the functions built on it, against `tests/vectors/kcu_tint.tsv`
//! (`kcu_tint`) and `tests/vectors/kde_selection_inactive.tsv` (the reader's
//! inactive selection), which were generated there by `tools/kde-vectors` from
//! kcolorscheme 6.31.0, kguiaddons 6.31.0 and qtbase 6.12.0, as their headers say;
//! the generator is not on this branch.
```

`kcu_tint.rs`:

```text
//!
//! Written in a clean room on the parked 0.8.0 branch, from behavioural
//! descriptions and the vector tables alone, and carried here unchanged but for
//! the clippy fixes its commit names, below this paragraph
//! (`native-theme/src/platform/kcu_tint.rs` and its notice in
//! `native-theme/src/platform/mod.rs:1-18` on `v0.8.0-rc1` at 78ee0efd). No line
//! of KDE's or Qt's code is in it. Its table, `tests/vectors/kcu_tint.tsv`, and the
//! reader's inactive selection's, `tests/vectors/kde_selection_inactive.tsv`, which
//! reaches it through `kcu_tint`, were generated there by `tools/kde-vectors` from
//! kcolorscheme 6.31.0, kguiaddons 6.31.0 and qtbase 6.12.0, as their headers say;
//! the generator is not on this branch.
```

**Checks:** for `kde_color_common.rs`, `tail -n +16` (three doc lines, the blank `//!`
and its notice's eleven further lines) hashes to `49e81317…7498`, P8's lines 4-306, and
`head -n 3` equals P8's first three lines; for `kcu_tint.rs`, `tail -n +13` (one doc
line, the blank `//!` and its notice's ten further lines) hashes to
`b9f02595…1719`, P8's lines 2-10, and `head -n 1` equals P8's first. (The offsets are
those of the notices as printed: twelve lines and eleven, each a blank `//!` and the
rest text; if their wrapping changes, the offsets change with it. The hashes are
taken before any clippy fix, 7.B.5, which the commit then names.)

The two tables, byte for byte, to `native-theme/tests/vectors/`; `sha256sum` as in 7.B.1.

### 7.B.3 Declarations, `native-theme/src/kde/mod.rs`

After step 1's `mod shade;`:

```rust
/// The arithmetic the clean-room KDE colour functions share: Qt's colour
/// storage, KDE's HCY space and `KColorUtils`' mix, tint and shade.
#[allow(
    dead_code,
    reason = "carried as the clean room wrote it, for its functions that are not carried (kcu_mix, kcu_shade, kde_effects); kcu_tint uses part of it"
)]
mod kde_color_common;
/// `KColorUtils::tint`: the clean-room function of `v0.8.0-rc1`, carried with
/// its own notice.
mod kcu_tint;
#[cfg(test)]
mod vector_tests;
```

### 7.B.4 The tests, new `native-theme/src/kde/vector_tests.rs` (decision 10)

Module doc: "The vector tables of the carried clean-room functions, read as step 1's
`shade` test reads its own." **The tolerance, one statement:** every channel of every
row passes where `|got × 255 − expected × 255| ≤ 1e-3`, the check of step 1's carried
test (`P8 kde_shade.rs:417-421`) and of the parked branch's (`P8
tests/platform_vectors.rs:123-131`). Private helpers: `parse_colour(cell) -> Option<[f32; 4]>`
for an input cell, which reads `#rrggbb` and `#rrggbbaa` (straight alpha, each byte
over 255) and, as the parked test's `color` does (`P8 tests/platform_vectors.rs:64-80`),
an `srgb(r, g, b, a)` cell as its four channels: four input cells of `kcu_tint.tsv` are
written so (rows `:120`, `:122`, `:128`, `:139`, read today), none of
`kde_selection_inactive.tsv`'s; `parse_srgb` for an expected `result`; `rows(table)`
skipping comment lines without a tab and the header (step 1's `vectors` logic, `P8
kde_shade.rs:392-406`). The file has no `.unwrap()`, `.expect(` or `panic!`: an
unparsable row is pushed to a failure list that one `assert!` reports, as step 1's
carried test does (`P8 kde_shade.rs:390-433`), so it needs neither the crate's test-module
allow (`#[allow(clippy::unwrap_used, clippy::expect_used)]` on a `#[cfg(test)]` module,
the form of `kde/mod.rs:460`) nor the Write hook's exemption for `#[cfg(test)]`
content. Tests:

- `kcu_tint_vectors`: `include_str!("../../tests/vectors/kcu_tint.tsv")`; each row's
  `super::kcu_tint::kcu_tint(a, b, amount)` against `result` within the tolerance; the
  row count is 124.
- `the_readers_inactive_selection_is_the_tables`: `include_str!("../../tests/vectors/kde_selection_inactive.tsv")`;
  each row through the reader's rule, `super::colors::inactive_selection(Some(selection),
  Some(window), Some(selection_background), role_kind == "background", switch ==
  "true")` (7.D.2), whose `Some` is compared with `result` (a `None` is a failure of the
  row); the row count is 70. `role_kind` `foreground` and `decoration` are both "not
  background" (read today: the table's three kinds).

### 7.B.5 What the project's gates make of the carried code

As step 1's A.4, row by row: `#![deny(clippy::unwrap_used, clippy::expect_used)]` (none
outside tests); `check_strict_panic.sh` (P8's file carries
`#[allow(clippy::modulo_arithmetic, …)]` on `wrap`, `kde_color_common.rs:174-178`, under
the same lint list; whether it passes in this tree is **not verified**, no cargo run; if
not, the work stops and the maintainer is told what failed, as step 1's A.4 rules
(`docs/todo_v0.7.1_kde-open-items-step1-spec.md:413`): a strict-panic finding in code is
fixed or brought to the maintainer, never allowed by the implementer, and the carried
file is not edited);
`check_panic_patterns.py`; clippy `-D warnings` (dead code by 7.B.3; an ordinary
clippy finding in a carried file, one that is no strict-panic finding and no hook
refusal, is no stop: it is fixed in the port, the 7.B.2 hash checks made on the file
before the fix, and the commit body names each fixed line and the lint); the Write hooks
(`.claude/hooks/no-invented-values.sh` exempts `*/src/platform/*`, which `src/kde/` is
not: whether its regexes match the carried file was not run here, unlike step 1's
`shade.rs`; the implementer runs each with `grep -cE` first and stops on a match);
rustdoc (both modules private).

## 7.C Registry, rules, facts

### 7.C.1 `docs/property-registry.toml`

`[defaults]` (`:96-146`), after `selection_inactive_background = "color"` (`:113`):
`selection_inactive_text_color = "color"`. `[list]` (`:377-391`), after
`header_height_px = "f32"` (`:387`): `header_hover_background = "color"` and
`header_active_background = "color"`.

### 7.C.2 `docs/inheritance-rules.toml`

- `[defaults_internal]` (`:55-62`), after `:58`:
  `"defaults.selection_inactive_text_color" = "defaults.selection_text_color"`.
- `[no_inheritance]` `fields`, after `"list.header_height", …` (`:463`):
  `"list.header_hover_background",    # None: a hovered section keeps header_background (§2.15)`
  and `"list.header_active_background",   # None: a pressed section keeps the hovered fill (§2.15)`.

### 7.C.3 `docs/platform-facts.md`

**§2.1.3**, one row after `selection_inactive_background` (`:1061`), Part F:

`selection_inactive_text_color`
- macOS: `\`labelColor\` (\`text_color\`): in a window whose table is not the first responder the selected row is not emphasized and its cells' background style is \`normal\` ([probe 26 :9424-9427](evidence/os-probe/2026-10-31-run-41195846264/os-probe-macos-26/probe.toml#L9424-L9427), :10217-10220), so its label does not adapt ("The system label colors … automatically adapt when displayed with an emphasized backgroundStyle", NSTableCellView.h:35 at Xcode 26.6) and draws as outside the selection — #000000d8 / #ffffffd8, drawn #222222 on #dcdcdc and #e3e3e3 on #464646 **(measured)** on macOS 15 ([probe 15 :9559-9567](evidence/os-probe/2026-11-06-run-42215651196/os-probe-macos-15/probe.toml#L9559-L9567), :10352-10360) and macOS 26 ([probe 26 :9559-9567](evidence/os-probe/2026-10-31-run-41195846264/os-probe-macos-26/probe.toml#L9559-L9567), :10352-10360) · macOS 14: not measured. \`unemphasizedSelectedTextColor\` (#000000 / #ffffff) is the unfocused *text* selection's, not this pair's`
- Windows: `← \`selection_text_color\`: no unfocused selection (\`selection_inactive_background\`)`
- KDE: `**(KF6 src)** \`[Colors:Window] ForegroundNormal\` where the scheme's \`[ColorEffects:Inactive] ChangeSelectionColor\` holds — that key, else the group's \`Enable\`, else true ([kcolorscheme.cpp:304-321](https://invent.kde.org/frameworks/kcolorscheme/-/blob/27066d471c93629efee8459d9f69490caa92b7c4/src/kcolorscheme.cpp#L304-L321)), as both Breeze schemes set it (\`selection_inactive_background\`) — Breeze #232629, Breeze Dark #fcfcfc **(measured)** ([out_light.txt:2, :116, :128](evidence/breeze-offscreen/2026-11-13/out_light.txt#L2), [out_dark.txt:2, :116, :128](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L2)); where it does not hold, \`[Colors:Selection] ForegroundNormal\`, the focused selection's. A scheme with \`[ColorEffects:Inactive] Enable=true\` gets this pair before the inactive group's contrast, intensity and colour effects, which recolour every colour of an inactive window, a palette the model does not hold (Breeze's schemes set \`Enable=false\`)`
- GNOME: `**(none)** — as \`selection_inactive_background\``
- Material: `**(none)** — as \`selection_inactive_background\``

The KDE link is the one `:1061`'s cell gives for the same lines; the `Enable` default is
`F/kde/tokens.toml:11275-11293` (`token.kde.inactive_change_selection_color`, quoting
kcolorscheme.cpp:304-316 at 6.31.0, `readEntry("ChangeSelectionColor",
inactiveEffectGroup.readEntry("Enable", true))`). Read today: `out_light.txt:2` gives
"Inactive Highlight #c2e0f5 Inactive HighlightedText #232629", `:116` the selected item
of an inactive view #c2e0f5 under #232629, `:128` a real list in an inactive window the
same.

`:1061`'s KDE cell ("… under the Window set's `ForegroundNormal`: …") stays.

**§2.15**, two rows after `header_height` (`:1432`), Part F:

1. `header_hover_background`
   - macOS: `❓ — not read: the parked facts state no table-header state (the rationale's §1)`
   - Windows: `**(none)** — WinUI 3 has no column-header control (\`header_height\`)`
   - KDE: `**(Breeze src)** \`KColorUtils::mix(Button, hoverColor, 0.2)\` for a hovered section ([breezestyle.cpp:6828-6837](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6828-L6837)), \`hoverColor\` the View set's \`DecorationHover\` (breezehelper.cpp:89-90 at f0b1d75) — Breeze #d6ecf8, Breeze Dark #2d4655 (computed)`
   - GNOME: `❓ — not read`
   - Material: `❓ — not read`
2. `header_active_background`
   - macOS, Windows, GNOME, Material: as `header_hover_background`
   - KDE: `**(Breeze src)** \`KColorUtils::mix(Button, focusColor, 0.2)\` for a pressed section ([breezestyle.cpp:6828-6837](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6828-L6837)), \`focusColor\` the View set's \`DecorationFocus\` (breezehelper.cpp:89-90 at f0b1d75) — Breeze #d6ecf8, Breeze Dark #2d4655 (computed)`

The breezestyle link is the one `:1429`'s KDE cell gives for the same lines (read
today); the breezehelper lines are those the reader's comment cites for `hoverColor()`
and `focusColor()` (`kde/colors.rs:566-569`), not re-read here (no network). The
`header_background` cell's sentence on the hovered and pressed fills (`:1429`) stays.

**The macOS readings**, read today from the files 7.C2 carries (`git show` of their
blobs): both probes report, for the unemphasized pass (`[macos.selection.<appearance>.unemphasized]`,
`table_is_firstResponder = false`), `row_view` `isSelected = true`,
`isEmphasized = false`, `interiorBackgroundStyle = "normal"` (`:9424-9427` aqua,
`:10217-10220` darkAqua), and the `r1_labelColor_glyph` sample `raw = "#222222ff"` over
`r1_labelColor_background` `raw = "#dcdcdcff"` (aqua, `:9559-9567`) and `#e3e3e3ff`
over `#464646ff` (darkAqua, `:10352-10360`); `#000000d8` over #dcdcdc is 34, #22, and
`#ffffffd8` over #464646 is 227, #e3 (computed), so the drawn label is `labelColor`.
The probes' OS versions are macOS 15.8.2 and 26.6.2 (each `probe.toml`'s `[os]`). The
parked facts' fact `role.selection.foreground_inactive` (`F/macos/roles.toml:11964-11978`
for macos-tahoe, `:14805-14822` for macos-monterey) states the *secondary* label's
colour on that row (`= role.window.foreground_inactive`), from the same note and with
published citations only (NSTableCellView.h:35, NSCell.h:289-290 / :287-288); the
primary label's measurement is the probe samples above, which the cell cites directly.
macos-monterey stands for macOS 12 to 15 (`CHANGELOG.md:38`); only 15 was measured, as
the cell says.

### 7.C.4 Checkers

As 6.B.4; `check_platform_facts_rows.py` looks for a `defaults` field in all of §2.1
(its docstring), and finds the new row in §2.1.3.

## 7.C2 Evidence carried from the parked branch

The macOS cell of 7.C.3 cites two os-probe runs that are not in this tree
(`docs/evidence/os-probe/` holds five runs, the latest `2026-10-26-run-40348854007`,
listed today) and not in the parked branch's tree either: `47b3f8c1` removed
`docs/evidence/` from that branch ("the files live in native-theme-evidence at the
pinned commit"), and the macOS 15 run is only on the branches `checkfacts-perf` and
`facts-macos-m1b4` (commit `71b4c68a`, "os-probe runs 42215651196 (macos-15), … brought
from v0.8.0-tools-probe-m2"; not an ancestor of `v0.8.0-rc1`). Carried, byte for byte,
from the blobs below (`git cat-file blob <id>` in the main checkout's repository):

| Path in this tree | Blob | From |
|---|---|---|
| `docs/evidence/os-probe/2026-10-31-run-41195846264/README.md` | `f134e0593f50379714d9c0b21c8802c357067fd8` | `47b3f8c1^` |
| `docs/evidence/os-probe/2026-10-31-run-41195846264/os-probe-macos-26/probe.toml` | `ef549af05e2581c0776397a049ac4e014810e5e7` | `47b3f8c1^` |
| `docs/evidence/os-probe/2026-10-31-run-41195846264/os-probe-macos-26/renders/selection/aqua-unemphasized.png` | `dba515de5437625e1a6290df88491cdc9ea00dbd` | `47b3f8c1^` |
| `docs/evidence/os-probe/2026-10-31-run-41195846264/os-probe-macos-26/renders/selection/darkAqua-unemphasized.png` | `f01dd3bdffede4a18e6c2e038b80e03f6f68f735` | `47b3f8c1^` |
| `docs/evidence/os-probe/2026-11-06-run-42215651196/README.md` | `5dbbf38ee4fa3fe3b0a4180d866e06854960e404` | `71b4c68a` |
| `docs/evidence/os-probe/2026-11-06-run-42215651196/os-probe-macos-15/probe.toml` | `21847bd148fedb906640033c6cb60932ecb80c39` | `71b4c68a` |
| `docs/evidence/os-probe/2026-11-06-run-42215651196/os-probe-macos-15/renders/selection/aqua-unemphasized.png` | `9dddb498e2924c7367979fa461967ed11d6ccccc` | `71b4c68a` |
| `docs/evidence/os-probe/2026-11-06-run-42215651196/os-probe-macos-15/renders/selection/darkAqua-unemphasized.png` | `e88f2e2817c6de501469a109a3539c9e3860d70e` | `71b4c68a` |

**Check:** `git hash-object <file>` of each equals its blob.

The run READMEs describe the whole runs; only the files above are carried, and
`docs/evidence/README.md`'s os-probe run table (`:22-28`) gains two rows after `:28`, in its form, each
saying so: `2026-10-31-run-41195846264/` (run 41195846264, 2026-10-31, the harness
commit `18d2a5d8` on `v0.8.0-tools-probe-m2`, "the unemphasized table selection on
macOS 26, for the label colour on an unfocused selection (issue #4, step 7); only
`probe.toml`, the two unemphasized renders and the README are kept here") and
`2026-11-06-run-42215651196/` (2026-11-06, `20d56259` on the same branch, the same on
macOS 15). The commits are the READMEs' (`f134e059…`, its lines 5-6; `5dbbf38e…`,
its lines 6-7). The README's rule "a recorded file is not edited after it is added"
(`docs/evidence/README.md:8-9`) holds: the files are added as recorded. No release
check reads `docs/evidence/`: of the gates, only `.github/workflows/screenshots.yml:28`
names it, in a comment (`grep -n evidence` over `scripts/check_release.sh` and
`.github/workflows/`, which finds besides it `check_release.sh:786` and
`screenshots.yml:133`, both the word in another sense).

## 7.D Presets and the readers

### 7.D.1 The values

| Field | Light | Dark | How computed today |
|---|---|---|---|
| `list.header_hover_background` | #d6ecf8 | #2d4655 | `mix(Button #fcfcfc / #292c30, DecorationHover #3daee9, 0.2)`, emulated as 6.C.1 |
| `list.header_active_background` | #d6ecf8 | #2d4655 | `mix(Button, DecorationFocus #3daee9, 0.2)`; Breeze's two decorations are one colour, so the two fields are equal under Breeze |
| `defaults.selection_inactive_background` | #c2e0f5 | #1b4155 | `kcu_tint(Window #eff0f1 / #202326, Selection #3daee9, 0.4)`: `kcu_tint.tsv:58` and `:103`, `srgb(0.759853542, 0.877225935, 0.959700942, 1)` and `srgb(0.105668724, 0.255695432, 0.335179687, 1)`, × 255 rounded; `kde_selection_inactive.tsv:35`, `:59` the same; equal to the measurement |
| `defaults.selection_inactive_text_color` | #232629 | #fcfcfc | `[Colors:Window] ForegroundNormal` (`kde_selection_inactive.tsv:15`, the `foreground`/`true` row, gives the window's colour) |
| macOS `defaults.selection_inactive_text_color` | #000000d8 | #ffffffd8 | `labelColor`, the presets' own `text_color` (`macos-monterey.toml`, `macos-tahoe.toml` `[light.defaults] text_color`, `[dark.defaults] text_color`); drawn #222222 / #e3e3e3 on the unfocused fill (7.C.3, decision 17) |

### 7.D.2 `native-theme/src/kde/colors.rs`

**Import.** Beside step 1's `use super::shade::…`: `use super::kcu_tint::kcu_tint;`.

**Constants**, after 6.C.2's:

```rust
/// How far a hovered or pressed header section moves from `Button` towards
/// the hover or the focus colour: `KColorUtils::mix(palette.button(),
/// hoverColor, 0.2)` and `mix(…, focusColor, 0.2)` (breezestyle.cpp:6828-6837
/// at breeze f0b1d75; docs/platform-facts.md §2.15).
const HEADER_SECTION_MIX: f64 = 0.2;

/// How far the inactive selection's background moves from the window's
/// towards the selection's: `KColorUtils::tint(Window, Selection, 0.4)`
/// (kcolorscheme.cpp:304-321 at 27066d47; docs/platform-facts.md §2.1.3).
const INACTIVE_SELECTION_TINT: f32 = 0.4;
```

**Helpers**, after `frame_contrast` (and step 1's `contrast` and `mid_shade`):

- `fn kconfig_bool(value: Option<String>) -> Option<bool>`: `true` or `false`, trimmed,
  ASCII-case-insensitive; anything else `None` (decision 11). Doc says what it accepts
  and that KConfig's whole syntax was not read.
- `fn inactive_change_selection_color(ini: &configparser::ini::Ini) -> bool`: doc "Whether
  KColorScheme builds an inactive window's selection from the Window set:
  `[ColorEffects:Inactive] ChangeSelectionColor`, else the group's `Enable`, else true
  (kcolorscheme.cpp:304-316 at kcolorscheme v6.31.0)". Body:
  `kconfig_bool(ini.get("ColorEffects:Inactive", "ChangeSelectionColor")).or_else(|| kconfig_bool(ini.get("ColorEffects:Inactive", "Enable"))).unwrap_or(true)`.
- `pub(super) fn inactive_selection(selection: Option<[f32; 4]>, window: Option<[f32; 4]>, selection_background: Option<[f32; 4]>, background: bool, change: bool) -> Option<[f32; 4]>`
  (D9's rule, rewritten), each branch reading only its own inputs: `selection` where
  `!change`; else, for the background, `window.zip(selection_background).map(|(w, sb)|
  kcu_tint(w, sb, INACTIVE_SELECTION_TINT))`, and for the text `window`. Doc: "The
  inactive Selection colour of a role, as KColorScheme builds it before any inactive
  effect: the rule of `kde_selection_inactive` on `v0.8.0-rc1`, rewritten for this
  reader and checked against its table (`vector_tests`). `None` where an input its
  branch reads is missing; an input the branch does not read may be missing. A scheme
  that enables `[ColorEffects:Inactive]` has its effects applied on top, which this
  does not do (docs/platform-facts.md §2.1.3)." `pub(super)` for the test.

**`populate_colors`**, after the list block (`:730-746`), where `window_bg` (`:638`),
`window_fg` (`:477`) and `selection_bg` (`:580`) are bound:

```rust
    // A header section (docs/platform-facts.md §2.15): hovered, the Button
    // set's background moved a fifth towards the hover decoration; pressed,
    // towards the focus decoration (breezestyle.cpp:6828-6837 at breeze f0b1d75).
    variant.list.header_hover_background = button_bg
        .zip(view_hover)
        .map(|(button, hover)| mix(button, hover, HEADER_SECTION_MIX));
    variant.list.header_active_background = button_bg
        .zip(view_focus)
        .map(|(button, focus)| mix(button, focus, HEADER_SECTION_MIX));

    // The selection of a view without the focus (docs/platform-facts.md
    // §2.1.3): where the scheme's `[ColorEffects:Inactive]` changes it, the
    // window tinted towards the selection under the window text; where not,
    // the focused pair, written so that kde-breeze's tint does not stand.
    let change = inactive_change_selection_color(ini);
    let selection_fg = get_color(ini, "Colors:Selection", "ForegroundNormal");
    let inactive = |selection: Option<Rgba>, window: Option<Rgba>, background: bool| {
        inactive_selection(
            selection.map(|c| c.to_f32_array()),
            window.map(|c| c.to_f32_array()),
            selection_bg.map(|c| c.to_f32_array()),
            background,
            change,
        )
        .map(|[r, g, b, a]| Rgba::from_f32(r, g, b, a))
    };
    variant.defaults.selection_inactive_background = inactive(selection_bg, window_bg, true);
    variant.defaults.selection_inactive_text_color = inactive(selection_fg, window_fg, false);
```

`Rgba::from_f32` rounds each channel × 255 to the nearest, as step 1's `mid_shade` does
(its B.2, the rounding claim). `populate_colors`' doc gains "the header's hovered and
pressed fills and the selection of a view without the focus" in its list, and a
sentence: "The inactive selection follows `[ColorEffects:Inactive] ChangeSelectionColor`
and `Enable`; a scheme that sets `[ColorEffects:Inactive] Enable=true` gets the pair
before the inactive group's contrast, intensity and colour effects, which recolour
every colour of an inactive window, a palette the model does not hold (decision 18;
docs/todo.md, *Model: an inactive window's palette*)." Its paragraph on the blends on
the accent (`kde/colors.rs:464-470` at `0fa28027`: "The blends on the accent (the
checked checkbox, the progress fill, the switch's on track and its outline, and the
list's hover) come from kdeglobals' `[Colors:Selection] BackgroundNormal`, so an
application's `with_overlay` accent or the portal's … does not move them") gains "the
inactive selection" in its list, after "the list's hover": the tint reads the same key,
through `selection_bg` (`kde/colors.rs:580`, `[Colors:Selection] BackgroundNormal`), so
a portal accent does not move it either.
A missing input gives `None` for the field whose branch reads it, and the preset's
value stands for that field. Each branch reads its own inputs only (the function's
`Option`s, above): where the tint is on, the fill reads `[Colors:Window]
BackgroundNormal` and `[Colors:Selection] BackgroundNormal`, the text `[Colors:Window]
ForegroundNormal` alone; where it is off, the fill reads `[Colors:Selection]
BackgroundNormal` and the text `[Colors:Selection] ForegroundNormal`, nothing else. So a
scheme without the Selection background, where the tint is on, leaves the fill
unwritten and still writes the window text; a missing Selection text colour matters
only where the tint is off, a missing Window colour only where it is on.
`test_inactive_selection_is_breezes` pins the first case (7.D.6).

**The tint's amount is an `f32` on purpose.** The reader calls the carried
`kcu_tint(a, b, amount: f32)` (`P8 kcu_tint.rs:8`), whose signature is the clean room's,
so `INACTIVE_SELECTION_TINT` is an `f32` and reaches `tint_q` as
`f64::from(0.4f32)`, 0.4000000059604645, where the parked rule passes the double 0.4
to `tint_q` (`P8 kde_selection_inactive.rs:24`). The 6e-9 difference in the amount is
shown by the rule's vector test (7.B.4), which checks the reader's rule against the
parked rule's 70 rows to 1e-3 on the 0-255 scale; Breeze's rows give the same value
either way (`kcu_tint.tsv:58`, `kde_selection_inactive.tsv:35`, read today). Calling
`tint_q` directly with an `f64` would leave `kcu_tint` unused outside its test.

**The keys and where they are read**: `[ColorEffects:Inactive]` is a group of
`kdeglobals`, which the reader reads already (`kde/mod.rs:3-24`) and the watcher
observes (`watch/kde.rs:12`); no source list or watcher change (as step 1's B.7).
Whether Plasma copies `ChangeSelectionColor` from the scheme into `kdeglobals` was not
verified (the facts quote plasma-workspace's applicator for the effect keys,
`F/kde/tokens.toml:11086-11089` for the intensity effect, not for this one); KColorScheme reads the same file
the reader does, so the two agree whichever it is.

**What each branch yields**, read from the tables today:

| `ChangeSelectionColor` / `Enable` | Result | Breeze Light | Breeze Dark |
|---|---|---|---|
| `true` / any (Breeze: `true` / `false`) | tint; window text | #c2e0f5, #232629 | #1b4155, #fcfcfc |
| `false` / any | the focused pair | #3daee9, #ffffff | #3daee9, #fcfcfc |
| missing / `false` | the focused pair | as above | as above |
| missing / `true` or missing | tint; window text | as the first row | as the first row |

### 7.D.3 `kde-breeze.toml`

`[light.defaults]`, after `selection_text_color` (`:49-50`), in the batch's preset
comment form (6.C.3):

```toml
# platform-facts.md (§2.1.3): selection_inactive_background, KColorUtils::tint([Colors:Window] BackgroundNormal
# 239,240,241, [Colors:Selection] BackgroundNormal 61,174,233, 0.4), as [ColorEffects:Inactive] ChangeSelectionColor=true
# (BreezeLight.colors:19 at breeze f0b1d75) has KColorScheme build an inactive window's selection from the Window set
# (kcolorscheme.cpp:304-321 at kcolorscheme 27066d47; measured)
selection_inactive_background = "#c2e0f5"
# platform-facts.md (§2.1.3): selection_inactive_text_color, [Colors:Window] ForegroundNormal 35,38,41
# (BreezeLight.colors:137, breeze v6.7.5)
selection_inactive_text_color = "#232629"
```

`[dark.defaults]`, after `:634-635`: the same with `32,35,38`, `(BreezeDark.colors:20 at
breeze f0b1d75)`, `#1b4155`, `252,252,252`, `(BreezeDark.colors:138, breeze v6.7.5)`,
`#fcfcfc`.

`[light.list]`, after `header_background` (`:347-350`):

```toml
# platform-facts.md (§2.15): header_hover_background, KColorUtils::mix(Button #fcfcfc, [Colors:View] DecorationHover
# 61,174,233 (BreezeLight.colors:117), 0.2), a hovered header section (breezestyle.cpp:6828-6837 at breeze f0b1d75)
header_hover_background = "#d6ecf8"
# platform-facts.md (§2.15): header_active_background, KColorUtils::mix(Button #fcfcfc, [Colors:View] DecorationFocus
# 61,174,233 (BreezeLight.colors:116), 0.2), a pressed header section (breezestyle.cpp:6828-6837 at breeze f0b1d75)
header_active_background = "#d6ecf8"
```

`[dark.list]`, after `:935-938`: the same with `#292c30`, `(BreezeDark.colors:118)`,
`(BreezeDark.colors:117)` and `#2d4655`. The scheme line numbers are the ones the file
already cites (`kde-breeze.toml:17`, `:21`, `:166`, `:602`, `:606`, `:752`), pinned
sources (breeze at a version); the `ChangeSelectionColor` lines are the facts row's
(`selection_inactive_background`), at f0b1d75.

### 7.D.4 macOS presets

**Inside the existing block.** The key goes into the block, one value line, with no
comment of its own; the block's comment is rewritten in place, in the batch's preset
comment form (6.C.3: v0.7.1's LN unit makes `check_preset_citations.py` read it and
compare nothing with the facts file), naming after the colon the rows it covers:

`macos-monterey.toml:45-47` becomes:

```toml
# platform-facts.md (§2.1.3): selection_background, selection_text_color, selection_inactive_background,
# selection_inactive_text_color, text_selection_background and text_selection_color: selectedContentBackgroundColor,
# alternateSelectedControlTextColor, unemphasizedSelectedContentBackgroundColor and labelColor on it,
# selectedTextBackgroundColor and selectedTextColor, probe :324, :317, :325, :331, :330; labelColor's measured on
# macOS 15 (os-probe run 42215651196, :9559-9567)
```

and after `selection_inactive_background = "#dcdcdc"` (`:50`), `selection_inactive_text_color = "#000000d8"`.
`[dark.defaults]` (`:538-540`, after `:543`): the same comment with its own probe lines
(`:398, :391, :399, :405, :404`) and `:10352-10360`, and `"#ffffffd8"`. `macos-tahoe.toml`
(`:49-51` and after `:54`; `:462-464` and after `:467`): the same, "on macOS 26 (os-probe
run 41195846264, …)". The six rows named are §2.1.3's from `selection_background` to
`text_selection_color` once Part F's row stands among them, each named by a covered
value. The probe numbers stay: the probe files
are pinned evidence. The other comments both steps add sit each directly above the one
value they cite, and the comment after each cites again; the
`primary_background / primary_text_color not stated` comment (6.C.3) keeps the
`not stated` form, naming the two primary rows.

### 7.D.5 The macOS reader, `native-theme/src/macos.rs`

The reader writes the label colour it already reads (decision 17): after
`selection_inactive_background: nscolor_to_rgba(&unemph_sel_bg, &srgb),` (`:172`), one
line, `selection_inactive_text_color: label,` (`label` is `labelColor`, `:143`; the
literal already uses it for `text_color` and the status texts, `:149`, `:161-167`). The
comment at `:169-170`, "// platform-facts.md:1060, :1062-1063: alternateSelectedControlTextColor
on a / // selection; selectedTextBackgroundColor and selectedTextColor for selected
text", becomes "// platform-facts.md §2.1.3 (selection_text_color,
selection_inactive_text_color, text_selection_*): alternateSelectedControlTextColor on a
/ // selection, labelColor on an unfocused one, which does not adapt; / //
selectedTextBackgroundColor and selectedTextColor for selected text" (one line more; it
names the rows, never their lines: S6). No new `NSColor` is read.

The reader's test `read_appearance_colors_maps_selection_and_shadow_as_platform_facts_names`
(`macos.rs:1147-1168`, `#[cfg(all(target_os = "macos", feature = "macos"))]`, run on
the macOS CI legs) gains, after its `selection_text_color` assertion (`:1156-1159`),
`assert_eq!(defaults.selection_inactive_text_color, nscolor_to_rgba(&NSColor::labelColor(), &srgb));`.
The comment above it, `// === The NSColor mapping (platform-facts.md:1057, :1060, :1062-1063, :1210) ===`
(`:1145`), stays as it is (S6: an existing citation is left; the new assertion names
the field it tests).

The two added lines move the later lines of `macos.rs` by +2; the citations of them
(`docs/todo.md:562`, `:572`, `:4313` at `0fa28027`) are left as they are (S6), and no
gate reads them.

### 7.D.6 Tests

`native-theme/src/kde/mod.rs`:

| Test or item | Change |
|---|---|
| `struct Scheme` (`:1174-1187`) and `kdeglobals` (`:1189-1213`) | `selection` becomes a pair `(BackgroundNormal, ForegroundNormal)`, `(&'static str, &'static str)` as `button`'s and `window`'s are, written as `[Colors:Selection]\nBackgroundNormal=…\nForegroundNormal=…\n`; two fields `change_selection_color: Option<&'static str>` and `inactive_enable: Option<&'static str>`: where either is `Some`, a `[ColorEffects:Inactive]` group is written, holding `ChangeSelectionColor=…` only where that field is `Some` and `Enable=…` only where that one is |
| The `Scheme` literals that name every field (read today: three of the 17 literals of `kde/mod.rs` have no `..`; the other 14 spread another one, 12 `BREEZE_LIGHT_SCHEME` and 2 their test's `without_header`, and need nothing for the two new fields) | `BREEZE_LIGHT_SCHEME`, `BREEZE_DARK_SCHEME` (row below), and `a_colour_scheme_keeps_the_stated_edge_geometry` (`:1657-1686`, its literal `:1661-1670`): `selection: "233,84,32"` becomes `selection: ("233,84,32", "255,255,255")`, and it gains `change_selection_color: None` and `inactive_enable: None`, so the tint applies, which the test does not read (it compares the edge geometry alone) |
| `an_accent_moves_the_on_track_the_pressed_button_the_selected_segment_and_the_list_hover` (`:1420-1438`; its literal `:1422-1425` spreads Breeze Light's and sets `selection`) | its `selection: "233,84,32"` becomes `selection: ("233,84,32", "255,255,255")` (Breeze Light's text kept); gains `defaults.selection_inactive_background` (tinted towards the selection) |
| `BREEZE_LIGHT_SCHEME`, `BREEZE_DARK_SCHEME` (`:1215-1244`) | `selection: ("61,174,233", "255,255,255")` / `("61,174,233", "252,252,252")`, `change_selection_color: Some("true")`, `inactive_enable: Some("false")`; each doc gains "Selection ForegroundNormal :95 (Dark :96)" among the keys it cites at breeze v6.7.5, as the doc already cites Selection BackgroundNormal :87 (:88) there, and "`[ColorEffects:Inactive]` ChangeSelectionColor :19 and Enable :25 at breeze 6.8.2 (Dark :20, :26)", the version the parked facts cite for these (`F/kde/inputs.toml:1029-1037`, `:3008-3016`, the Selection set's `foreground_normal`; `F/kde/tokens.toml:11255-11293`, at `78ee0efd`); ChangeSelectionColor's `:19` and `:20` are also this tree's facts row's at breeze f0b1d75 (`docs/platform-facts.md:1061`). That `:95` and `:96` are the lines at v6.7.5 rests on the parked facts' numbers at 6.8.2 and the files' layout the doc's other citations show (each set's ForegroundNormal eight lines below its BackgroundNormal: Button :31 and :39, View :115 and :123); the v6.7.5 files were not read here, and whether `:25` and `:26` hold at f0b1d75 was not verified |
| `derived_leaves`, `resolved_leaves` | gain `list.header_hover_background`, `list.header_active_background`, `defaults.selection_inactive_background`, `defaults.selection_inactive_text_color`; on the resolved side the two header fields directly, `theme.list.header_hover_background` and `theme.list.header_active_background`, already `Option<Rgba>`, as the array's other soft options are (`("switch.hover_unchecked_background", sw.hover_unchecked_background)`, `kde/mod.rs:1323-1329`), and the two defaults fields wrapped, `Some(theme.defaults.selection_inactive_background)` and `Some(theme.defaults.selection_inactive_text_color)`, which are `Rgba` (`resolved.rs:107`, 7.A.2); the count grows by four. The four go at the end of both arrays, after every entry the arrays hold on the tree, by the rule every earlier step follows: each appends its own leaves after what the steps before it appended (step 1's B.5, the steps 2–3 specification's 2C.1 and Part 3C, the steps 4–5 specification's A.6 and B.5, and step 6's 6.C.4 here; the caret's specification touches neither array), and none fixes another place for them. The order of the four: `list.header_hover_background`, `list.header_active_background`, `defaults.selection_inactive_background`, `defaults.selection_inactive_text_color`, the two arrays in the same order. `moved_leaves` (`:1384-1393`) keeps `resolved_leaves`' order (it filters the two zipped arrays), so in each asserted array below a new leaf stands after every leaf the array lists on the tree, and where two are added, in that order |
| `the_derived_colours_are_the_presets_under_breeze` | passes with 7.D.2 and 7.D.3, no edit |
| `the_window_set_moves_…` (step 1's name), both schemes | gain `defaults.selection_inactive_background` (the window tinted) and `defaults.selection_inactive_text_color` (the window text) |
| `the_button_set_moves_the_button_slider_switch_and_header_leaves` | gains `list.header_hover_background` and `list.header_active_background` (both mix the Button background) |
| `the_view_decorations_move_the_frames_the_tab_hover_and_the_handle` | the hover sub-case gains `list.header_hover_background` only, the focus sub-case `list.header_active_background` only |
| `the_header_set_moves_the_toolbar`, `the_view_set_moves_…`, `the_frame_contrast_moves_the_edges` | none |
| new: `the_inactive_selection_follows_change_selection_color` | `change_selection_color: Some("false")` gives the focused pair in both fields; `None` with `inactive_enable: Some("false")` the focused pair; `None` with `None` the tint |

`native-theme/src/kde/colors.rs`, new tests after the default button's:
`test_header_sections_are_breezes` (#d6ecf8 / #2d4655 for both fields under Breeze's keys;
a scheme whose `DecorationHover` and `DecorationFocus` differ gives two different
fields; its fixture needs `[Colors:View] DecorationHover` beside `DecorationFocus`,
since the hovered fill reads it (7.D.3), and the file's hand-made groups,
`breeze_menu_list_groups` (`colors.rs:1054-1069`), hold `DecorationFocus` alone: the
test writes `DecorationHover=61,174,233`, the value `kde/mod.rs`' Breeze schemes state
for it at breeze v6.7.5, `BREEZE_LIGHT_SCHEME`'s `hover`, `:1215-1229`) and `test_inactive_selection_is_breezes` (#c2e0f5 / #232629 and #1b4155 /
#fcfcfc; `ChangeSelectionColor= TRUE ` with spaces counts as true, `=False` as false: the
forms decision 11's function accepts, and no others, since KConfig's syntax was not
read; and with `[Colors:Selection] BackgroundNormal` missing under Breeze Light's other
keys, the fill `None` and the text #232629, the window text, which its branch reads
alone).
`test_full_mapping_completeness` asserts the four `is_some()`.

## 7.E gpui connector

**Readers today.** `selection_inactive_background` (`lib.rs:916-920`) and its smoke test
(`:2343-2349`); `UNREAD_COLOURS` names `defaults.selection_inactive_background` as
`NoReceiver` (`contract.rs:1169-1177`: no `ThemeColor` field is an inactive selection,
and a `ListItem` fills `list_active` or `accent` whatever the window's state,
`list/list_item.rs:257-264`). The table header: `table_head` and `list_head`
(`colors.rs`, the header rows `contract.rs:373-411`); gpui-component's header section
has no hover or press style (`GC/table/state.rs:1929-1972`, `render_th`: no `.hover(` or
`.active(`; the sort icon's own hover, `:1912-1913`, is the icon button's, in
`secondary` tokens). Nothing draws text on an inactive selection.

**Accessors**, `src/lib.rs`, after `selection_inactive_background` (`:916-920`), in its
form:

```rust
/// Text color over the selection of a view without the focus.
#[must_use]
pub fn selection_inactive_text_color(resolved: &ResolvedTheme) -> Rgba

/// Fill of a hovered column-header section, where the platform states one
/// (`None`: it keeps `list.header_background`). gpui-component's table has no
/// hover style for a header section (`table/state.rs`, `render_th`); for a
/// header the application draws.
#[must_use]
pub fn list_header_hover_background(resolved: &ResolvedTheme) -> Option<Rgba>

/// Fill of a pressed column-header section, where the platform states one
/// (`None`: the hovered fill, else `list.header_background`). As
/// [`list_header_hover_background`].
#[must_use]
pub fn list_header_active_background(resolved: &ResolvedTheme) -> Option<Rgba>
```

The smoke test `selection_and_disabled_helpers` (`:2343-2349`) calls the three.

**Contract** (`contract.rs`, `UNREAD_COLOURS`), three `NoReceiver` entries after
`defaults.selection_inactive_background`'s:

- `defaults.selection_inactive_text_color`: "a selected `ListItem` keeps the text it set
  before its fill whatever the window's state (`list/list_item.rs:205-264`), and no
  `ThemeColor` field is an inactive selection's text (`theme/theme_color.rs:59-343`)".
- `list.header_hover_background` and `list.header_active_background`: "a `DataTable`'s
  header section is `render_th`'s, with no hover or press style (`table/state.rs:1929-1972`);
  the delegate's `render_th` content lies inside the cell's padding (`:1553-1590`), not
  over the section".

The cited `theme_color.rs:59-343` and `list_item.rs:205-264` are the existing entries'
ranges (`:1172-1175`, `geometry.rs:1384-1392`), not re-read.

**Crate doc**, the coverage table of `src/lib.rs` (`:108-…`), in place, the mapped
counts unchanged since the three leaves reach no `ThemeColor` field:

- `:110`, "| `defaults` colors | 14 of 24 |" becomes "| `defaults` colors | 14 of 25 |".
  The 24 are the 23 `color` keys of the registry's `[defaults]`
  (`docs/property-registry.toml:96-146`, the last at `:127`, counted today) and `border.color`, which the
  row's list names; the fonts' colours it does not count (the count has been 24 since
  `96f8285f`, whose registry held the same 23 keys). Step 7 adds one `color` key; steps
  2 to 5 add none to `[defaults]` (step 2's field is a `bool`).
- `:122`, "| `list` | 7 of 14 + geometry |" becomes "| `list` | 7 of 16 + geometry |".
  The 14 are the fields of the registry's `[list]`, `:377-391`, colours and others
  alike (counted today: 14; the count went from 13 to 14 in `44e1a07a`, which added
  `header_height_px`); step 7 adds two, which the row's list of mapped fields does not
  gain.

## 7.F iced connector

**Readers today.** None: `UNREAD_COLOURS` names `defaults.selection_inactive_background`
(`contract/derived.rs:1390-1397`: iced draws a text field's selection only while it is
focused); the showcase's colour swatches show it (`showcase-iced.rs:16426`); iced's
`table` has no style setter and its header is the application's element (`IW/table.rs:35-44`,
`:149-187`); the showcase's table header is a `container` (`showcase-iced.rs:11697-11712`).

**Accessors**, `src/lib.rs`, after `list_header_height` (its doc and `:1610-1613`), in its form
(`resolved: &native_theme::theme::ResolvedTheme`): `list_header_hover_background` and
`list_header_active_background`, each `-> Option<iced_core::Color>`, with the docs of
7.E's. No `selection_inactive_text_color` (decision 13): the exception
`scripts/connector_parity_exceptions.toml:202-206` gains a second entry beside it, the
same reason with "and so no style takes its text colour either".

**Contract** (decision 23). `src/contract/rows.rs`, `ACCESSOR_ROWS` (`:119-…`), two
rows in the form of `crate::selection_background`'s (`:158-162`), with a comment "iced
0.14's `table` has no style setter, and a header cell is the application's own element
(`iced_widget-0.14.2/src/table.rs:35-44`, `:149-187`): the application draws a
section in this fill": `accessor: "crate::list_header_hover_background"`, `native: |r|
r.list.header_hover_background.map(to_color)`, `get: |r|
crate::list_header_hover_background(r)`, and the same for
`list_header_active_background`. The walk splices each leaf (`null` on every preset but
kde-breeze, a colour there) and the accessor moves with it, so the two leaves reach
their rows and are named nowhere else. `src/contract/derived.rs`, `UNREAD_COLOURS`,
after `:1390-1397`: `defaults.selection_inactive_text_color` alone, with `:1393-1396`'s
evidence. `contract-leaves.toml` regenerated (6.E.3's command): the two header leaves
reach `crate::list_header_*`, the text colour is named.

**Documentation.** `README.md`'s "Full helper list" (`:337-355`): after
`selection_background` (`:351`), "`list_header_hover_background` and
`list_header_active_background` (a table header section's hovered and pressed fills,
for a header you draw: iced's `table` has no style setter)". The crate doc's module
table, its "Color helpers" row (`src/lib.rs:321`): the colour helpers' cell gains
`list_header_hover_background`, `list_header_active_background` after
`selection_background`, and the source cell, `defaults.*`; the widget's own, holds them
(`list.*` is a widget's own).

## 7.G egui connector

**Readers today.** `selection_inactive_background` (`accessors.rs:88-92`), re-exported
(`lib.rs:201`); its mapping row is UNMAPPABLE `source-side gap`, "native-theme: a text
colour for the unfocused selection fill (§14 item 45)" (`mapping.toml:151-154`;
`mapping.md:66`). egui has one selection pair (`EG/style.rs:1190-1200`); egui_extras lays
the header row out without hover (`EX/table.rs:490-506`).

**Accessors**, `accessors.rs`, after `selection_inactive_background`:
`selection_inactive_text_color(t) -> egui::Color32`; and after `list_header_height`
(`:525-532`): `list_header_hover_background(t) -> Option<egui::Color32>`,
`list_header_active_background(t) -> Option<egui::Color32>`, docs as 7.E's with
"egui_extras' header row has no hover (`egui_extras/src/table.rs:490-506`); for a header
the application paints". `lib.rs:199-203` re-exports them.

**Mapping**, `mapping.toml`:

- `["defaults.selection_inactive_background"]` (`:151-154`): the gap is closed, so
  `verdict = "derived"`, `tested_by = "T18(a)"` (its accessor), the `sub_tag` and
  `upstream` keys go.
- `["defaults.selection_inactive_text_color"]`, after it: `verdict = "derived"`,
  `tested_by = "T18(a)"`.
- `["list.header_hover_background"]`, `["list.header_active_background"]`, after
  `["list.header_height"]` (`:1973-1975`): the same.

`mapping_tests.rs`, as deltas on the tree the earlier work left (6.F.3): the leaf count
grows by three; `foundation`'s derived count by two and its unmappable count falls by
one (`[18, 1, 36, 11]` → `[18, 1, 38, 10]` at `0fa28027`), `inputs`' derived count by
two (`[4, 35, 36, 12]` → `[4, 35, 38, 12]`; `list` is in the Inputs group,
`mapping.md:14`, as is the caret work's `input`); `tags.get("source-side gap")`
(`:489`, 16 at `0fa28027`) falls by one, and the test's doc, "sub-tags (16 source-side
gap, …)" (`:440`), with it; the leaf count's message (`:240`) grows as in 6.F.3.
`mapping.md` regenerated.

**Tests**, `install_tests.rs`: `t18_a_the_inactive_selection_and_header_accessors_are_their_leaves`,
doc first line "T18 (a): `selection_inactive_background`, `selection_inactive_text_color`,
`list_header_hover_background` and `list_header_active_background` return their
leaves.", over kde-breeze and adwaita in both modes (adwaita's header fields `None`).

**README** (`:262`, the accessor list): the three, one sentence.

## 7.H Showcases

### 7.H.1 The table header's states (D7)

In each showcase's own table, each of the two header sections is filled, while the
pointer is on it, with the hovered fill of 7.A.1's rule, and while pressed with the
pressed one; with both fields unset nothing changes (every preset but kde-breeze).

**The corners, in all three.** A fill that meets the table's rounded frame is rounded
there by the frame's inner radius, the frame's radius less its line: the header's top
corners, the last row's bottom ones (gpui `demo.rs:9475-9482`, the rule's comment, and
`:9501-9502`, `:9529-9531`; iced `table_row_style`, `showcase-iced.rs:11828-11853`, with
`inner_radius`, `:4823-4830`, and `rounded_at`, `:4834-4842`; egui `inner_corners`,
`showcase-egui/pages/basic.rs:1467`, called at `:1792` and `:1830`). kde-breeze's list
frame is rounded (`list.border` states no radius or width, `kde-breeze.toml:362-365`, so
`defaults.border`'s 5 and 1 apply, `:75`, `:77`: inner radius 4). So a hovered or pressed
section's fill is rounded where it meets the frame: the first section's top-left corner,
the last's top-right, square elsewhere.

- **iced** (`basic_table`, `showcase-iced.rs:11635-11755`): each header `cell(…, header:
  true, …)` (`:11709-11710`) is wrapped in `button(..).padding(0)` with a style function
  of its own, `fn header_section_style(resolved: &ResolvedTheme, first: bool) -> impl
  Fn(&Theme, button::Status) -> button::Style + use<>`, beside `table_row_style`. Per
  status: `Active` no background, so the header row's fill shows (`table_row_style(..,
  TableRow::Header)`, the row's container style, keeps it), the header's text colour
  `list.header_font.color`, no border; `Hovered` the hovered fill of 7.A.1
  (`native_theme_iced::list_header_hover_background`), `Pressed` the pressed one
  (`list_header_active_background`, else the hovered), each where stated, else no
  background; `Disabled` as `Active`; in every status the border's radius
  `rounded_at(inner_radius(&resolved.list), [first, !first, false, false])`.
  `on_press(Message::HeaderSectionPressed)`, a new message the update ignores ("a
  section sorts nothing here"). The two calls: `header_section_style(resolved, true)`
  for "Name", `(resolved, false)` for "Size".
  - **`styles_cover_every_widget_shown`** (`:20919`) holds every `button(` call site to
    `DRESSED`'s `button` row (`:20792-20810`), the closed list of the styles a button
    may carry: `"header_section_style"` joins it after `"list_row"`, and its doc
    sentence (`:20785-20791`, "A button in a role the connector has no class for -- a
    flat button, a menu row, a tab, a list row -- wears the showcase's own style
    function for that role …: `ghost_button`, `menu_row` (…), `tab_style`, `list_row`.")
    gains "a table's header section" in the roles and `header_section_style` in the
    names.
  - **`drawn_by`** (`:29008-29075`) gains the arm `"header_section_style" => [true,
    false].map(|first| buttons(&header_section_style(r, first))).join("|")`, over the
    four `button::Status` values the closure `buttons` draws (`:29010-29022`).
  - **Test**, in `the_basic_list_and_table_fills_keep_inside_the_frames_corners`
    (`:22502`, which walks adwaita, kde-breeze and macos-tahoe, `:22528`): for each
    preset and status, `header_section_style(&r, true)`'s border radius is
    `corners(inner, 0.0, 0.0, 0.0)` and `(&r, false)`'s `corners(0.0, inner, 0.0, 0.0)`,
    in the form of its table rows' check (`:22548-22563`); and under kde-breeze its
    `Hovered` background is `Some` (#d6ecf8 light).
- **egui** (`table`, `showcase-egui/pages/basic.rs:1740-1863`): after the header's fill
  (`:1789-1794`), for each section `i` (the loop `:1796-1800`, whose `id` is the
  element's `&str`), `ui.interact(cell, ui.id().with(("table-head", i)),
  egui::Sense::click())`; where hovered (`response.hovered()`) or pressed
  (`response.is_pointer_button_down_on()`) and the field is stated,
  `painter.rect_filled(cell, inner_corners(l, [i == 0, i == 1, false, false]), fill)`
  before the text. Test: `the_basic_list_and_table_fills_keep_inside_the_frames_corners`
  (`showcase-egui/tests.rs:4731`, its `corners_of`, `:4734`) gains, under kde-breeze with
  the pointer on each section, the section fill's corners: the inner radius on the
  first's top-left and the last's top-right, none elsewhere.
- **gpui** (`files_table`, `showcase-gpui/demo.rs:9402-9555`): the header's two cells
  (the cells of the `row` closure, `:9427-9454`, which the header calls at `:9492` and
  every body row at `:9519`) take `.id(..)`, which `.active(..)` needs on a `div`:
  `hover` is `InteractiveElement`'s, which a plain `Div` has (gpui-pre 0.3.12
  `src/elements/div.rs:866`), while `active` is `StatefulInteractiveElement`'s
  (`:1608`), which only an element with an id is. An id on the header's cells alone would make the closure's cells two
  types (`Div` and `Stateful<Div>`), so every cell takes one: `TableLine` gains
  `cell_id: &'static str`, `"table-head"` for the header and `"table-cell"` for a body
  row, and each cell `.id((spec.cell_id, ix))`; a body cell's id is unique under its
  row's own `.id(("table-row", ix))` (`:9528`), since gpui keys an element by the path
  of its ancestors' ids (`GlobalElementId`, a list of `ElementId`s, gpui-pre 0.3.12
  `src/element.rs:220-223`). The header's cells also take `.debug_selector(..)` (`"{id}-header-{ix}"`,
  for the test) and, where stated, `.hover(|s| s.bg(colour(fill)))` and `.active(|s|
  s.bg(colour(fill)))`, through a `TableLine` field for the header only (`None` for the
  body rows), and `.rounded_tl(inner)` for `ix == 0`, `.rounded_tr(inner)` for `ix == 1`,
  `inner` the `list_inner_radius` the header row already uses (`:9483`). Test: beside
  `the_row_fills_are_rounded_where_they_meet_the_frame` (`showcase-gpui/tests.rs:15253`,
  under adwaita, which states no section fill), a test under kde-breeze light that
  hovers each header section and reads its quad's corner radii by that test's method
  (`:15274-15295`): `[inner, 0, 0, 0]` for the first, `[0, inner, 0, 0]` for the
  last, and the quad's fill #d6ecf8.

**Widget Info.** Each showcase's header info names the two leaves and "hovered" and
"pressed", and says that the toolkit's own table has no such state (gpui-component's
`render_th`, iced's `table`, egui_extras' header row), so the showcase draws it.

**The element list** (`docs/showcase-elements.toml`), in the file's leaf order (own table
in registry order, then the others; 7.C.1 puts the two keys after `header_height_px`):
`list.header_hover_background` and `list.header_active_background` join the leaves of
the two cells that draw the fills, `basic.table.header.name` (`:1891-1897`, after
`"list.header_font.color"`) and `basic.table.header.size` (`:1900-1906`, after
`"list.grid_color"`), and, by the file's rule that a control's leaves are the union of
its parts' (`:36`), of `basic.table.header` (`:1882-1888`) and `basic.table.frame`
(`:1874-1880`), each after `"list.header_height_px"`. The states stay (`["Normal"]`):
the file lists a hover leaf beside a rest state already (`basic.list.row_1`,
`:1704-1705`, `list.hover_background` with `states = ["Normal"]`), since the captures
sample the rest look. No leaf is listed twice.

**The routes.** gpui (`info/leaves.rs`), with the prefix `"basic.table."` (header, cells
and frame), in the table's section beside the `"basic.table.header"` entries
(`:1253-1283`): `list.header_hover_background`, "gpui: demo::files_table's header cells,
.hover(..) bg where stated: gpui-component's own table header has no hover style
(gpui-component/table/state.rs:1929-1972, render_th)"; `list.header_active_background`,
the same with ".active(..), else the hovered fill". The range carries the symbol its
lines hold, as 6.G's routes say `every_prose_citation_still_exists` requires: `:1929`
is `fn render_th(` and `:1962` calls `self.delegate.render_th(..)` in gpui-component
0.9.1 (read today). iced: `iced_route`'s list arms (`:8250-…`, in their form,
`"basic_table: …"`, `"list_row: …"`, `:8251-8311`) gain
`("list", "header_hover_background") => "header_section_style: a header section's
Hovered fill"` and `("list", "header_active_background") => "header_section_style: a
header section's Pressed fill, else its Hovered fill"`.
`every_hand_colour_route_names_a_function_that_reads_it` (`:29102`) takes the name
before ": " (`route_functions`, `:29079-29092`), calls `drawn_by` with it and requires a
splice of the leaf to move what it draws in at least one combination: the new
`drawn_by` arm draws both sections in all four statuses, and splicing either leaf moves
the `Hovered` or `Pressed` background on every preset (on kde-breeze from its value,
elsewhere from `None`). `the_element_list_parses_and_every_leaf_has_a_route`
(`:24982`) passes with the two arms. egui: the `mapping.toml` rows of 7.G.

### 7.H.2 The unfocused selection (D9; decision 16)

Each showcase follows its real window's state (decision 21):

- **iced**: `State` gains `window_active: bool`, `true` where the state is built, and
  `Message` a variant `WindowFocusChanged(bool)`, handled as `state.window_active = active`.
  The subscription's existing `iced::window::events()` filter (`showcase-iced.rs:17509-17515`,
  which maps `Opened` and `Rescaled`) gains `iced::window::Event::Focused =>
  Some(Message::WindowFocusChanged(true))` and `Unfocused => Some(Message::WindowFocusChanged(false))`
  (`iced_core-0.14.0/src/window/event.rs:43`, `:46`). A test that sends no focus event
  keeps the active look.
- **gpui**: `Window::is_window_active()` (gpui-pre 0.3.12 `src/window.rs:3204`) at render;
  an activation change refreshes the window (`:2181-2196`: the active-status callback
  sets `active` and calls `window.refresh()`), so the swap shows at once. In tests the
  test platform's window is not active until it is activated (`TestWindow::is_active`
  returns `false`, `src/platform/test/window.rs:432-434`, read into `Window::active` at
  `src/window.rs:1716`; `activate` sets it through `set_active_window`,
  `src/platform/test/platform.rs:391-411`; neither the test platform's `open_window`,
  `:604-622`, nor `App`'s activates a window it opens). The harness `open_with`
  (`showcase-gpui/tests.rs:119-139`) stays as it is (decision 21). A test that needs the
  active look activates its own window, as gpui-component's own tests do
  (gpui-component 0.9.1 `notification.rs:1182`), with `cx.update(|window, _|
  window.activate_window())` (`src/window.rs:6917`) and `cx.run_until_parked()` (the
  status change is a spawned task, `platform.rs:396-410`) before its next `draw`;
  `VisualTestContext` has no `activate_window` of its own (`src/app/test_context.rs`
  has only `deactivate_window`, `:1074-1080`). A test of the swap deactivates it with
  `VisualTestContext::deactivate_window()` ("Simulates the user blurring the window",
  through `set_active_window(None)`), after activating it first, so that the change is
  one the window sees. The showcase's tests also run on GPUI Fast
  (`scripts/check_release.sh:715`, the connector's `--all-targets`; `:854`, the
  showcase), the locked gpui-fast 0.1.15 (`Cargo.lock:3274-3275`), whose test platform
  reads the same: `set_active_window` `src/platform/test/platform.rs:392-412`,
  `TestWindow::activate` and `is_active` (`false`) `src/platform/test/window.rs:423-434`,
  `deactivate_window` `src/app/test_context.rs:1077-1082`, `Window::activate_window`
  `src/window.rs:6982-6984`, `is_window_active` `:3231`. Each such line carries the comment "// The swap of
  docs/todo_v0.7.x_kde-open-items-steps6-7-spec.md 7.H.2 follows the window's state;
  this test reads the <active | inactive> look" (the path as it is after 7.I.6).
- **egui**: `ctx.input(|i| i.focused)` (`egui/src/input_state/mod.rs:315` at 0.36.2,
  `:316` at 0.37.0), from `RawInput::focused`, `true` by default
  (`src/data/input/raw_input.rs:106` at both), which `egui_kittest` keeps, so the
  showcase's tests keep the active look; egui-winit starts at `false` and sets it from
  the window's focus event, asking for a repaint (egui-winit 0.36.2 `src/lib.rs:149`,
  `:427-448`; 0.37.0 `:163`, `:483-505`). Decision 21 says what the real showcases show
  before the first focus event.

**The gpui tests that read a selected row's colour**, re-listed today: every test
function of `showcase-gpui/tests.rs` whose body names `selection_background`,
`selection_text_color`, `selection_inactive`, `list_active`, `basic-table`,
`basic.table`, `files_table`, `row_2` or a table row, read with a Python scan of the
file's `fn` bodies that printed each with the presets it selects
(`"kde-breeze"`, `"macos-monterey"`, `"macos-tahoe"`) and whether it opens the
showcase; each hit read in place. A test that selects no preset runs under the system
theme (`Showcase::new`, `showcase-gpui/app.rs:1370-1378`: the desktop's preset, else the
platform's), so it was read too. Of gpui's samples only the table swaps (below; iced's
and egui's lists swap as well as their tables), so a gpui test is touched
only if it reads the Basic table's selected row under a theme the rule selects:

| Test | Preset | What it reads | Edit |
|---|---|---|---|
| `the_basic_tables_infos_report_what_it_draws` (`:10993`) | kde-breeze light | the selected row's info claims (`list.selection_background`, `selection_text_color`) and the fill painted on it (`painted_fill(&mut cx, "basic-table-row-1")`, `:11073-11077`) | activates its window after `open` (`:10995`), with the comment: kde-breeze's inactive fill differs from its selection after step 7, so an inactive window would paint #c2e0f5 |
| `the_basic_list_shows_three_rows` (`:6039`), `a_hovered_list_or_tree_row_letters_its_label_in_the_lists_hover_text_colour` (`:7144`), `a_selected_list_or_tree_rows_info_names_the_selection_text_colour` (`:7196`) | kde-breeze | the List's and the tree's rows | none: neither swaps (gpui's List is a `ListItem`, below; the tree is not one of the rule's places) |
| `the_row_fills_are_rounded_where_they_meet_the_frame` (`:15253`) | adwaita | the corner radii of the table's selected row's quad (`:15269-15296`) | none: radii, not colours, under a preset the rule does not select |
| `the_hairlines_are_filled_quads_at_scale_one` (`:18666`) | macos-tahoe | the grid lines' quads inside `basic.table.frame` (`:18698-18703`) | none: `list.grid_color` quads, not the row's fill |
| `a_table_row_reports_what_the_table_paints_on_it` (`:6917`), `the_tables_sort_button_sorts_its_rows` (`:7005`), `a_moved_column_keeps_its_data` (`:7040`) | system | the Data page's `DataTable` rows | none: gpui-component's `DataTable` is not one of the rule's places |
| `the_copy_box_takes_the_buttons_hover_and_pressed_text_colours` (`:4130`) | kde-breeze | `selection_text_color` as a button text colour (`:4136`) | none: no row |
| `the_native_number_field_reports_what_it_draws` (`:10904`) | kde-breeze | the number field | none (the scan matched the next test's doc comment) |

No other test of the file reads the Basic table's rows (`grep -n 'basic-table-row'`:
`:11013`, `:11027`, `:11074`, `:15271`, all in the tests above). The new gpui test of the
swap (below) activates, then deactivates, its window.

**The rule, exactly.** For the resolved variant the showcase draws (`ResolvedTheme`,
the light or dark one in use), let `inactive = r.defaults.selection_inactive_background`
and `active = r.defaults.selection_background`. While the window is not active **and**
`inactive != active` (`Rgba`'s equality, all four channels), a list's or table's
selected row is painted in `inactive` lettered `r.defaults.selection_inactive_text_color`
instead of `r.list.selection_background` and `r.list.selection_text_color`; otherwise
(window active, or the two equal) the list keeps its own pair. The places: iced
`list_row` (`showcase-iced.rs:4772`, its selection at `:4778-4779`) and `TableColours`
(`:11885-11886`); egui the list rows (`basic.rs:1670`) and the table (`:1822`); gpui
`files_table` (`demo.rs:9508-9512`). One function per showcase,
`selection_pair(resolved, window_active) -> (fill, text)`, in each toolkit's colour
type, which every place below calls.

**How the window's state reaches each place, and the signatures that change.**

- **iced.** `state.window_active` (above) is read where `state` is in scope and passed
  down as an argument:
  - `fn list_row(resolved: &ResolvedTheme, selected: bool, window_active: bool)`
    (`:4772-4796`): the selected arm (`:4786`) takes `selection_pair(resolved,
    window_active)` in place of `selection` and `selection_label` (`:4778-4779`).
    Callers: `list_row_fill` (`:4996-4997`), which gains the same parameter, `fn
    list_row_fill(resolved: &ResolvedTheme, selected: bool, window_active: bool, view:
    ListView) -> UnderPaint<'static>`; `view_basic`'s list rows (`:11254`, `:11261`),
    with `state.window_active` (`fn view_basic<'a>(state: &'a State, …)`, `:10371`,
    which reads `state.basic_list_selected` beside them, `:11234`); `command_palette`'s
    rows (`:8992`, `fn command_palette(state: &State)`, `:8956`), with
    `state.window_active` (its rows are never selected, so the value changes nothing
    there).
  - `TableColours::of(resolved: &ResolvedTheme, window_active: bool)` (`:11876-11889`):
    `selected_fill` and `selected_ink` (`:11885-11886`) from `selection_pair`.
    `fn table_row_style(resolved: &ResolvedTheme, row: TableRow, window_active: bool)
    -> container::Style` (`:11833`), which calls `TableColours::of` (`:11834`), passes
    it on. Callers: `basic_table` (`fn basic_table(state: &State)`, `:11635`), with
    `state.window_active` at `:11639`, `:11705` and `:11726`; the corner test
    (`:22556`), with `true` (it reads radii, which the state does not change).
  - `drawn_by` (`:29008`): the `list_row` arm (`:29050-29052`) draws every pair of
    `selected` and `window_active`, `[false, true].map(|selected| [true,
    false].map(|active| buttons(&list_row(r, selected, active))).join("/")).join("|")`;
    the `basic_table` arm (`:29054`) `[true, false].map(|active| format!("{:?}",
    TableColours::of(r, active))).join("|")`. So a splice of either defaults leaf moves
    what each draws on a combination the rule selects, and the hand routes below hold.
  - Every other call of the three functions, found with `grep -n
    'TableColours::of\|table_row_style(\|list_row('`, is one of those above (today:
    `:4997`, `:8992`, `:11254`, `:11639`, `:11705`, `:11726`, `:11834`, `:22556`,
    `:29051`, `:29054`).
- **egui.** The pages read `ui.ctx().input(|i| i.focused)` where they paint the rows
  (`basic.rs:1670`, `:1822`). The inactive pair lives in `t.defaults`, which
  `list_row` (`basic.rs:1655-1663`) cannot reach through its `l: &ResolvedListTheme`,
  so `list_row` takes `t: &native_theme_egui::ResolvedTheme` in place of `l` (as
  `row_metrics` does, `:1495-1496`) and reads `&t.list` inside, still seven
  parameters; its one caller (`:1574-1582`, inside `list`, where `t` is bound at
  `:1527`) passes `t` in place of `l`. The table's painter (`:1822`) sits in a function
  that already binds `t`, so it changes no signature.
- **gpui.** `files_table` takes `cx: &App` and no `&Window` (`demo.rs:9402-9410`), so
  the window's state reaches it as an argument. It has seven parameters today; an
  eighth would fail clippy's `too_many_arguments`, whose threshold is seven (clippy's
  default; `connectors/native-theme-gpui/clippy.toml` sets none, read today; `tab_bar`,
  with eight, carries `#[allow(clippy::too_many_arguments)]`, `demo.rs:1915-1925`),
  which `scripts/check_release.sh:543-550` runs over the showcase with `-D warnings`. So
  `selected` and the window's state are folded into one argument, a small struct in the
  form of the showcase's spec structs (`DemoButton`, `demo.rs:3317-3327`;
  `ExpanderSpec`, `:7140-7152`), beside `TableLine` (`:9360-9372`):

  ```rust
  /// Which row of [`files_table`] is selected, and whether its window is
  /// active, which decides the selected row's pair (`selection_pair`).
  #[derive(Clone, Copy)]
  pub(crate) struct TableSelection {
      pub row: usize,
      pub window_active: bool,
  }
  ```

  and `fn files_table(ui: &Entity<InfoRegistry>, cx: &App, id: &'static str, head:
  [(&'static str, &'static str); 2], rows: &[[&'static str; 2]], selected:
  TableSelection, width: Pixels) -> Stateful<Div>`, seven parameters as today; an
  `allow` is not needed. Its one caller (`git grep -n 'files_table('` over the
  showcase: the definition and `pages/basic.rs:794`), the Basic page
  (`pages/basic.rs:794-802`, in `render_basic_page`, which has `window: &mut Window`,
  `:347-351`), passes `TableSelection { row: TABLE_SELECTED, window_active:
  window.is_window_active() }`. The selected row (`selected.row` where `selected` is
  compared today) and its pair (`:9508-9512`) take `selection_pair(r,
  selected.window_active)`. The row's info (`native_table_row`) takes the theme's rule
  alone, `defaults.selection_inactive_background != defaults.selection_background`
  (decision 16), and not `selected.window_active`: it names both pairs where the rule
  selects the theme, whatever the window's state (decision 22).

**gpui's List sample has no receiver.** `demo::ListRow` (`demo.rs:4973-5180`) is a
gpui-component `ListItem` (`:5138`), which paints a selected row's fill, `list_active`
(or `accent` with `active_highlight` off), after the caller's style
(`GC/list/list_item.rs:209`, `:257-264`), whatever the window's state; its text colour
is the caller's (`geometry::list_item_selected`), but a text colour without its fill is
no pair. So gpui's Basic list keeps the list pair in an inactive window; only gpui's
table swaps. Its Widget Info (`info::data::list_row`) and the element's `HOW` line say
so, and the gpui contract's entry for `defaults.selection_inactive_text_color` (7.E)
already cites `list/list_item.rs`.

**What the rule selects after step 7**, computed today from the presets by
`selection_inactive_background ← selection_background ← accent_color`
(`docs/inheritance-rules.toml:56`, `:58`) and 7.D.3's kde-breeze values: 6 of the 34
bundled combinations.

| Combination | `selection_background` | `selection_inactive_background` | Inactive text |
|---|---|---|---|
| kde-breeze light / dark | #3daee9 / #3daee9 | #c2e0f5 / #1b4155 | #232629 / #fcfcfc |
| macos-monterey light / dark | #0064e1 / #0059d1 | #dcdcdc / #464646 | #000000d8 / #ffffffd8 |
| macos-tahoe light / dark | #0064e1 / #0059d1 | #dcdcdc / #464646 | #000000d8 / #ffffffd8 |

Every other preset states no `selection_inactive_background` (searched today: only the
two macOS presets state it), so it equals `selection_background` and no list changes;
windows-11's lists keep #00000009 / #ffffff0f. The test, in each showcase: on every
bundled combination, `selection_pair(r, true)` is the list pair; `selection_pair(r,
false)` is the inactive pair on exactly the six combinations above and the list pair on
the other 28.

The new tests, one per showcase: iced, the `basic_table` and `list_row` extraction
under kde-breeze light with `window_active` false gives #c2e0f5 under #232629, true the
list pair; egui, a pass with `RawInput::focused = false` paints the table's selected
row #c2e0f5; gpui, a test that opens through `open`, activates its window and reads the
table's selected row quad #3daee9, then deactivates it and reads #c2e0f5, while the
List's selected row keeps `list_active` both times, and the row's info claims what was
painted in each (decision 22's claims).

**Widget Info** (decision 22). The info of a selected table or list row, in each
showcase, names beside the list pair, where the rule selects the theme, "bg, window not
active" `defaults.selection_inactive_background` and "text, window not active"
`defaults.selection_inactive_text_color`, whatever the window's state: gpui's
`native_table_row` (`showcase-gpui/info/data.rs:422-445`, today claiming
`selection_background` and `selection_text_color` for a selected row, `:429-441`) gains
the two claims for the table; the List's row info (`info::data::list_row`) gains a line
saying it keeps the list pair (below); iced's and egui's selected-row infos gain the two
lines for both.

**The element list.** `defaults.selection_inactive_background` and
`defaults.selection_inactive_text_color` join `basic.table.row_2` (`:1926-1932`) and
`basic.table.frame` (`:1874-1880`), and `basic.list.row_2` (`:1719-1725`) and
`basic.list.frame` (`:1693-1699`), each after `"defaults.line_height"` (registry order:
`[defaults]` lists `line_height`, `:99`, before `selection_inactive_background`, `:113`);
`defaults.selection_inactive_text_color` alone joins `basic.list.row_2.text`
(`:1728-1734`). States stay `["Selected"]`: the captures sample the active window's
rows. The list's elements take the leaves in every showcase, gpui's included, whose
route says it does not draw them.

**The routes.** gpui `HOW` lines: prefix `"basic.table."`, the two leaves, "gpui:
demo::files_table's selected row while the window is not active and the theme's
inactive fill differs from its selection (selection_pair)"; prefix `"basic.list."`, the
two leaves, "not reachable: a ListItem paints its selected fill list_active after the
caller's style, whatever the window's state (gpui-component/list/list_item.rs:257-264,
list_active), and a text colour without its fill is no pair". The range carries the
symbol its lines hold (6.G's routes): `:259` is `this.bg(cx.theme().list_active)` in
gpui-component 0.9.1 (read today); the item's `render`, which the lines lie in, is not
named in them. iced: hand routes in `iced_route`'s list arms, in
their convention (`"basic_table: …"`, `"list_row: …"`, `:8251-8311`), each naming a
function `drawn_by` draws (`route_functions` takes the name before ": ", `:29079-29092`):
`("defaults", "selection_inactive_background") if under("basic.table.") =>
"basic_table: the selected row's fill while the window is not active, where it differs
from the selection's (selection_pair)"`, the same leaf otherwise `"list_row: the
selected row's fill while the window is not active, where it differs from the
selection's (selection_pair)"`, and `selection_inactive_text_color` the same with "text
colour" and "text_color". `drawn_by`'s `basic_table` and `list_row` arms draw both
window states (above), so splicing the fill leaf moves what they draw wherever the
spliced fill differs from the selection's, and splicing the text leaf on the six
combinations the rule selects; the test asks for one combination
(`showcase-iced.rs:29094-29100`).
egui: the `mapping.toml` rows of 7.G.

**The captures (decision 21).** Read today: the macOS and Windows captures are taken by
`.github/workflows/screenshots.yml` (`:166-207`), which runs each showcase with
`--tab basic --screenshot <path>` per preset and variant (macos-monterey and macos-tahoe,
light and dark; windows-11); the showcase captures its own window through the shared
crate's `tools/showcase-support/src/capture.rs` (`capture_macos`, `:358`, `screencapture -l`
of the window it finds; `capture_windows`, `:467`); nothing there activates the window.
The Linux captures activate it and check it stays active
(`scripts/capture_window.sh:134-175`). Whether a macOS runner's window is active when
it is captured was not found anywhere (no workflow log was read; no network).

**The capture check** (decision 21; 7.J.4). The session runs it, as it runs and compares
every capture; the maintainer is needed only where its result is undecided. It reads
each toolkit's own row from that toolkit's own layout dump, written beside the capture
on the macOS runner.

- **The workflow edit** (U7.3, so that it is in place before the release's asset run).
  `.github/workflows/screenshots.yml`, the macOS capture step (`:175-191`): the
  showcase's arguments (`:187-190`) gain, beside `--screenshot`, `--dump-layout
  "connectors/native-theme-${{ matrix.toolkit }}/docs/assets/${{ matrix.os_name
  }}-${theme}-${variant}.json"`. All three showcases take the two together and quit
  once the last of them is written, the showcases' one exit rule: gpui
  `showcase-gpui/main.rs:833-835`, iced `showcase-iced.rs:2736-2739` (and its argument
  test, `:29380`, `["--dump-layout", "f", "--screenshot", "p"]`), egui
  `showcase-egui/app.rs:355-359` (`outputs`, "the three showcases' exit rule"). The
  macOS upload (`:214-225`) lists the four JSON files beside the four PNGs, in the same
  folder, so the artifact's layout is unchanged; the comment above it (`:210-213`,
  "Keep these lists in step with the `themes` lists") holds for them too. The Windows
  step (`:193-208`) and its upload are not changed. `scripts/generate_assets_release.sh`
  copies only `*.png` and the OS-version file out of each artifact (`:237-249`), so the
  dumps never reach a committed `docs/assets/`.
- **When.** During the release's asset run, after `generate_assets_release.sh` has
  downloaded the CI artifacts (`gh run download`, `:225`) and before any asset is
  committed: the script commits nothing and ends with "Review the assets, then commit"
  (`:297`). The script removes its download folder after copying (`:253`), so the
  session downloads the run's three macOS artifacts again into its scratch folder
  (`gh run download <run id> --pattern 'screenshots-*-macos' --dir <scratch>`, the run
  id the script prints at `:122`): `screenshots-iced-macos`, `screenshots-gpui-macos`
  and `screenshots-egui-macos`, one per macOS job of the matrix (`:33-55`, three
  `macos-26` entries), each with four PNGs, four JSON files after the edit above, and
  the OS-version file. It reads each PNG with the JSON beside it there.
- **A failed dump.** Where a macOS capture job fails on its dump (a dump not written
  in time, as *What was not verified* says iced's refusal does; gpui's and egui's
  handling of a late dump was not read), or an artifact lacks one of its four JSON
  files, the check has nothing to read for that capture: the work stops, and the
  session reports the run, the job and its log to the maintainer before any asset is
  committed.
- **What is read.** Per toolkit, preset and variant (twelve captures:
  `macos-macos-{monterey,tahoe}-{light,dark}.png` of iced, gpui and egui, the
  workflow's names, `:220-224`), the rectangle of `basic.table.row_2`, the selected row,
  from that toolkit's own dump: logical pixels of the window's content, with the
  dump's `scale`, physical pixels per logical one (`docs/showcase-elements.toml`, "The
  layout dump", `:100-114`). In the capture, each coordinate is multiplied by `scale` and
  moved by the content's place in the capture, (W − 1280·`scale`, H − 720·`scale`): the
  capture is the window with its title bar, its content exactly 1280 × 720 × `scale`
  (`check_frame_capture`, `tools/showcase-support/src/capture.rs:70-95`), and on the
  macOS runners the title bar is on top with no side frame, so the content's top-left
  is at (0, 32) at scale 1 (the committed captures, read below). The region is that
  rectangle shrunk by 2 logical pixels on each side, so that no anti-aliased edge
  counts.
- **What the committed captures show** (read today with Python and PIL, read-only, the
  twelve `connectors/native-theme-{iced,gpui,egui}/docs/assets/macos-macos-{monterey,tahoe}-{light,dark}.png`
  at `0fa28027`, from 0.7.1's asset runs, `0f849012` and, for the files it left
  unchanged, `d8b9ec1c`): every capture is 1280 × 752, the content at scale 1 below a
  32 px title bar with no side frame. The active fills #0064e1 / #0059d1 appear exactly,
  within 2 per channel, 7326 to 7518 pixels below the title bar per capture (the list's
  and the table's selected rows); in gpui's macos-tahoe light capture the table's
  selected row holds 3513 of them in x 1075-1242, y 439-462 (87 % of that box). So the
  captures are not colour-managed away from the presets' values. The window buttons are
  coloured (#ff3337, the red one, at (15, 9) in that capture), so those windows were
  very likely active: that macOS draws a window's buttons grey when it is not key is
  general macOS behaviour, not checked against a source in this repository. The check
  above stays, for later runs.
- **What is counted.** In the region, the pixels within 2 per channel of the active
  fill, #0064e1 light and #0059d1 dark, and of the unfocused fill, #dcdcdc light and
  #464646 dark (the macOS presets' `selection_background` and
  `selection_inactive_background`, 7.H.2's table), each as a share of the region's
  pixels.
- **The decision.** A capture whose active share is at least one half was taken with an
  active window; one whose unfocused share is at least one half with an inactive one.
  Where all twelve are active, the assets are committed. Where any is inactive,
  the capture code (not the showcase) is changed to make the window active before
  `screencapture` runs. `capture_macos` (`capture.rs:358-383`) runs `screencapture`
  inside one `FrameCapture::poll` (`:217-267`, its macOS branch `:241-261`), with no
  frame drawn between the call and the capture, so the activation does not go there:
  it goes in `FrameCapture::start` (`:175-206`, the macOS branch `:191-200`) or in an
  earlier `poll` that returns `None` (as `poll` already returns `None` until
  `next_try`, `:243-246`), so that the showcase draws its active look before the
  capture; the plan picks which, and how a runner's window is activated was not read.
  The fix adds no `unsafe` (the project's rule; no exception covers the capture code):
  its options are the toolkits' own activation calls, which the shared crate's start or
  poll would ask each showcase to make: gpui's `Window::activate_window` (gpui-pre
  0.3.12 `src/window.rs:6917`), egui's `ViewportCommand::Focus` (egui 0.36.2
  `src/viewport.rs:1169`, "puts the window on top of other applications and takes
  input focus away from them"), iced's `window::gain_focus` (iced_runtime 0.14.0
  `src/window.rs:431`); none was run on a macOS runner. A fix that could activate the
  window only with `unsafe` code is a stop. iced's capture cannot show whether its
  window was active: iced's state starts active (`window_active: true`, decision 21)
  and changes only on an `Unfocused` event, so an iced capture shows the active fill
  whether or not the window was key; its active share says only that no `Unfocused`
  event came, and gpui's and egui's captures are the ones that tell.
  The change is reviewed, committed and pushed, and the asset
  run is repeated before any asset is committed. Where a capture reaches neither half,
  the session first checks the region itself in that capture (a misplaced region is
  its own to fix: the dump's rectangle, the scale, the title bar's offset) and counts
  again; only where the result is still undecided does it report the two shares and
  the region to the maintainer, who decides before anything is committed.
- **Windows is left alone.** The rule does not select windows-11 (its two fills are
  equal), so its captures cannot show a swap.

**The colour swatches.** iced's Theme page list of colours (`showcase-iced.rs:16411-16428`)
gains `("sel_inactive_fg", d.selection_inactive_text_color)` after `("sel_inactive", …)`.
gpui's Theme Map lists `ThemeColor` tokens (`showcase-gpui/demo.rs:8897`), none of which
is an inactive selection, and no `selection_inactive` appears anywhere in the gpui
showcase (`git grep -n selection_inactive` over `examples/showcase-gpui/`, empty today):
no edit, and nothing there shows the pair but the table and its info. egui's Theme Map
lists the rows of `mapping.toml` (`showcase-egui/info.rs:760`, a row's four cells), so it
shows 7.G's new rows by itself: no edit.

## 7.I Outside the code (D10)

### 7.I.1 `CHANGELOG.md`, the release's `## [0.7.N]` section (6.H.1)

No section of its own (S5): step 7's entries join the one section 6.H.1 opens. Its break
is named in 6.H.1's one *Breaking Changes* entry, which already names `ListTheme`'s two
fields and `ThemeDefaults`' one; step 7 adds no entry there and checks that one against
the fields 7.A adds. `### Added`, after step 6's entries:

```markdown
- **native-theme**: `list.header_hover_background` and `list.header_active_background`, the fills of a hovered and a pressed column-header section, each optional (unset, the section keeps its fill). kde-breeze and the KDE reader state Breeze's, `mix(Button, hoverColor, 0.2)` and `mix(Button, focusColor, 0.2)`, #d6ecf8 / #2d4655. No toolkit's table has these states; the connectors' `list_header_hover_background` and `list_header_active_background` give them to a header the application draws, as the three showcases' tables do.
- **native-theme**: `defaults.selection_inactive_text_color`, the text colour over `selection_inactive_background`, the selection of a view without the focus; unset, `selection_text_color`. kde-breeze and the KDE reader state Breeze's pair, the window colour tinted towards the selection by 0.4 under the window text, #c2e0f5 / #1b4155 under #232629 / #fcfcfc, where the colour scheme's `[ColorEffects:Inactive] ChangeSelectionColor` (else `Enable`, else on) asks for it, and the focused pair where not, before any `[ColorEffects:Inactive]` effect a scheme enables; the macOS presets and reader state `labelColor`, #000000d8 / #ffffffd8, the label of an unfocused selected row, which does not adapt (measured on macOS 15 and 26). The KDE reader computes the tint with a clean-room port of `KColorUtils::tint` in Qt's arithmetic, its amount the `f32` 0.4 (0.4000000059604645 as a double, 6e-9 from KColorScheme's 0.4), checked against the 70 rows of a table generated from KColorScheme, to 1e-3 on the 0-255 scale. No toolkit's list tells an unfocused window apart; the gpui and egui connectors' `selection_inactive_text_color` gives the colour beside `selection_inactive_background`. The showcases' tables and lists draw the pair while their window is not active, where the theme's inactive fill differs from its selection fill (kde-breeze and the two macOS presets); gpui's List sample, a gpui-component `ListItem`, paints its own selected fill whatever the window's state and keeps the list pair.
```

### 7.I.2 `ROADMAP.md`

No paragraph of its own (S5): in the release's one 0.7.N paragraph that 6.H.2 opens,
after step 6's sentences, the sentence "A table header's hovered and pressed sections
and the selection of a view without the focus take Breeze's colours under KDE: the model
has fields for both, which the connectors offer to an application, since no toolkit's
table or list has these states; the showcases draw them." The 0.7.1 paragraph's clause
"the selection of an unfocused view waits for a text colour the model does not have
yet" (as step 1 leaves it) goes.

`ROADMAP.md:264`'s count ("tags 29 open items `[v0.7.x]`" at `0fa28027`, the number
the releases before it wrote there on their trees): 7.I.5 closes one `[v0.7.x]` entry
(item 2, *The selection of a view without the focus*, `docs/todo.md:1914`) and opens
none (item 3's is `[1.0]`), so U7.4 recounts it on its tree with `grep -c '^- \[ \]
\[v0.7.x\]' docs/todo.md` after 7.I.5's edits, writes that number there and records it
in its commit body (step 1's G.2 form). Today the grep prints 41 at `0fa28027`; the
number on U7.4's tree is the earlier releases', less one.

### 7.I.3 The first rationale

- **§5.3** (`:889`), a line after the heading: "*Settled by `0004_kde-open-items-rationale.md`
  D9 (0.7.N): `defaults.selection_inactive_text_color`; kde-breeze, the KDE reader and
  the macOS presets and reader state the pair.*"
- **§4.4** (`:528-593`), "Not in 0.7.1", the *No field* bullet (`:585-588`): "the
  header's hover and pressed fills (#d6ecf8 / #2d4655) and its separators" becomes "the
  header's separators (its hover and pressed fills are fields since 0.7.N)".
- **§5.4's status line** (the line after the heading `### 5.4`, `:907` at `0fa28027`,
  which step 3 adds and steps 4 and 5 extend), as step 5 leaves it (steps 4 and 5's
  B.11), found by its sentences: after the sentence that starts "Also by D7's toolbar
  part", a sentence is inserted: "Also by D7's table part (<the release that brings
  step 7>): a table header's hovered and pressed fills (`list.header_hover_background`,
  `list.header_active_background`), which kde-breeze and the KDE reader state; no
  toolkit's table draws them, so the three connectors offer them through accessors
  (`list_header_hover_background`, `list_header_active_background`)." (7.A.1, 7.D.3,
  decision 13). Step 5's sentence "The table header is that rationale's D7; the menu
  item's and the list row's outlines and the menu bar title stay in the backlog (its
  §0)." loses its first clause and becomes "The menu item's and the list row's outlines
  and the menu bar title stay in the backlog (its §0)."
- **§7**, the row C "toolbar buttons, sort header" (`:1033`, as step 5 leaves it, found
  by its first cell): the sort header's part, "the sort header waits for its fields"
  in step 5's text (its B.11), becomes "the sort header: **fixed in 0.7.N**, its hovered
  and pressed fills are `list.header_hover_background` and `header_active_background`;
  no toolkit's table draws them, so an application's own header takes them through the
  connectors' accessors".
- **§8, the reply draft**, under 2I.3's rules: all of it applies only while the reply has
  not been posted (once it is, this step edits nothing there, and whether a follow-up is
  written is the maintainer's); every edit is placed by anchor and by content, never by
  line; what this step fixes moves under "Later 0.7.x releases fix more:" with its full
  text, never in both lists; the release is written "<the release that brings step 7>"
  and filled with the version the maintainer gives it; the note keeps every sentence
  still in the draft and its D0/D10 caveat. On the draft as step 5 leaves it (its B.11):
  1. **"The rest of your list"**: the sentence that begins "The pressed sort header:"
     (step 5's, "The pressed sort header: native-theme has no values for its states
     yet; it comes last, and since no GUI library's table can show it, its values will
     reach an application only for a table it draws itself.") is removed, whatever its
     text then is, and the paragraph rewrapped; it then begins "The rest of your list,
     briefly. Dialog text is the general font, …". No item of the reporter's list is
     left there as open.
  2. **The list "Later 0.7.x releases fix more:"**, found by that heading's text, gains
     this bullet as its last, after whatever bullets the earlier steps and other work
     have put there (after step 5's **Toolbar buttons** where nothing else came later):

     ```markdown
     > - **The sort header** (<the release that brings step 7>). Breeze fills a
     >   column header's section while the pointer is on it and while you press
     >   it: the button colour moved a fifth towards the hover colour, and towards
     >   the focus colour when pressed. native-theme now has both colours, read
     >   from your colour scheme, and the colours of a selected row in a window
     >   that is not active, Breeze's window colour tinted towards the selection
     >   under the window text. No table of the GUI libraries native-theme
     >   supports shows these states by itself, gpui-component's included, so
     >   they are for a table you draw yourself: in GPUI,
     >   `native_theme_gpui::list_header_hover_background`,
     >   `list_header_active_background`, `selection_inactive_background` and
     >   `selection_inactive_text_color` give them.
     ```

     It says no "yet": no upstream change is planned for a table header's states or an
     unfocused selection (the upstream requests, `docs/issues/0004_kde-hand-overrides-upstream-requests.md`,
     hold none for them).
  3. **The note after the draft.** "it comes last" leaves the draft with item 1's
     sentence, so it leaves the note; every other sentence the note names stays, with
     the caveat. Where step 3's trial passed: "The sentences "I am reporting it there"
     and "I have asked gpui-component to draw its ring for keyboard focus only" are the
     maintainer's to confirm before posting: they follow
     `0004_kde-open-items-rationale.md`'s proposed order (D10), and "0.7.x" its proposed
     versions (D0 (a); under (b) or (c) some of these releases are minor ones, and the
     wording changes with them)."; where it failed, the same with "so I have asked for
     it" and "it is part of the same request as the button's hover border" after the
     ring's sentence ("The sentences "I am reporting it there", "I have asked
     gpui-component to draw its ring for keyboard focus only", "so I have asked for it"
     and "it is part of the same request as the button's hover border" are …"): the
     second is the end of step 5's **Toolbar buttons** bullet in the trial-failed text,
     which stays in the draft (steps 4 and 5's B.11, its item 1's `<… | …>` text and its
     item 3's note, found there by those words), and it leans on request 5, which the
     maintainer sends or not. The new bullet adds no sentence to confirm: its version is
     the placeholder the maintainer fills.

### 7.I.4 The rationale's status line

`docs/issues/0004_kde-open-items-rationale.md:3-4`, one sentence for the release of
both steps (S5; 6.H.4), at its close-out: "Steps 6 (D8) and 7 (D7's header, D9)
implemented on `<date>` for 0.7.N, by `../archive/todo_v0.7.x_kde-open-items-steps6-7-{spec,plan}.md`."
With step 7 every step is done; the rationale stays in `docs/issues/`, as the first
rationale does, and its status line says so.

### 7.I.5 `docs/todo.md`

1. **"What Breeze draws that the model has no field for"** (`:1774-1825`): in "a header
   section's hover and pressed fills, `mix(Button, hoverColor, 0.2)` and `mix(Button,
   focusColor, 0.2)` (`breezestyle.cpp:6828-6837`), #d6ecf8 / #2d4655 (computed), and
   its separators" (`:1793-1795`), the fills go: "a header section's separators". After
   the entry's last line: "*Issue #4, open items step 7 (0.7.N, <date>, <sha>):* the
   header section's hovered and pressed fills are `list.header_hover_background` and
   `header_active_background` (`kde-breeze.toml` `[light.list] header_hover_background`,
   `[dark.list] header_hover_background`), drawn by the showcases' tables; the parked
   trace's K1.177." The entry stays: its other items are open.
2. **"The selection of a view without the focus, KDE and macOS, and its missing text
   colour"** (`:1914-1933`): closed whole; it moves to `docs/archive/todo_closed.md`
   under its section, with "Done in 0.7.N (`<sha>`): `defaults.selection_inactive_text_color`;
   kde-breeze (`kde-breeze.toml` `[light.defaults] selection_inactive_background`,
   `[light.defaults] selection_inactive_text_color`, and dark), the KDE reader through
   the carried `kcu_tint`, the macOS presets (`labelColor`) and reader; the connectors'
   accessors and the showcases' lists, which swap where the theme gives an inactive fill
   of its own."
3. **A new entry** (decision 18), `[1.0]`, placed by an entry that stays: directly after
   the last line of *Model: the accent a control is filled with*, the entry 6.H.5 adds,
   which item 5 leaves, not beside item 2's or item 4's entries, which this step moves
   to `docs/archive/todo_closed.md`: "### Model:
   an inactive window's palette" — "- [ ] [1.0] KColorScheme gives an inactive window a
   palette of its own where a colour scheme sets `[ColorEffects:Inactive] Enable=true`:
   the group's contrast, intensity and colour effects (`ContrastEffect`, `IntensityEffect`,
   `ColorEffect` and their amounts and colour) recolour every role of the window, not
   the selection alone (Breeze's schemes set `Enable=false`;
   `docs/facts/kde/tokens.toml:11066-11274` on `v0.8.0-rc1` at `78ee0efd` gives Breeze's
   values and KColorScheme's defaults). The model holds
   none of it; since 0.7.N the KDE reader writes the unfocused selection pair before
   those effects (`docs/issues/0004_kde-open-items-rationale.md` D9). The arithmetic
   exists on `v0.8.0-rc1` at `78ee0efd`, written in a clean room and checked against KDE's code:
   `native-theme/src/platform/kde_effects.rs` (87 lines, with `kde_color_common.rs`,
   which 0.7.N carries) and its table `native-theme/tests/vectors/kde_effects.tsv`
   (367 lines, sha256 `653da4a6335be39dee89cd244e79c99c0ff6d0a22f26fc2771e610a3dd6d21fb`).
   Decide whether and how the model carries an inactive window's palette as a whole
   (every colour of an unfocused window, on KDE and on the platforms that dim one), one
   item, not a patch on one field."
4. **"Selection: the text colour macOS pairs with the unemphasised selection"**
   (`:543-580`): answered (decision 17). A dated line: "*Issue #4, open items step 7
   (0.7.N, <date>, <sha>):* the label of an unemphasized selected row does not adapt and
   draws in `labelColor`, #000000d8 / #ffffffd8, measured on macOS 15 and 26 (os-probe
   runs 42215651196 and 41195846264, carried into `docs/evidence/os-probe/` by 0.7.N);
   the macOS presets and reader state it as `defaults.selection_inactive_text_color`." The
   entry closes to `todo_closed.md`. Its suggestion that egui's plugin swap the pair on
   focus loss is recorded as not done: no toolkit's widget tells the state, and D9 offers
   the pair through accessors.
5. **"Model: the accent a control is filled with"** (6.H.5): untouched.

### 7.I.6 `docs/archive/`

At the plan's close-out, this specification and its plan move to `docs/archive/`, and
`docs/archive/README.md` gains a row for 0.7.N (or a line on the row of the release the
steps land in): "`todo_v0.7.x_kde-open-items-steps6-7-{spec,plan}.md`: steps 6 and 7 of
the open KDE items (the default button; the table header's states and the unfocused
selection); their rationale stays with the issue". The links between the documents are
rewritten for the new place.

## 7.J Order, gates, close-out

### 7.J.1 What depends on what

- Step 6's units committed and reviewed (7's units follow them on the same branch, in
  the same release, S5; 7 starts from step 6's tree; D10 orders them).

**Units**, each gated by 6.I.1's cadence (S1):

- **U7.1 — the three fields, everywhere they are counted.** 7.A entire (the fields,
  the hand-written resolved field and its inheritance, every struct literal and test
  that names the fields, `fully_populated_variant` among them: its callers run
  `validate()` without `resolve()`, so without the new line they fail on the missing
  required field); 7.C (registry, rules, the facts rows, placed by Part F); 7.C2 (the
  evidence the macOS cell links); the preset values of 7.D.3 and 7.D.4; 7.D.5 (the
  macOS reader: it builds `ThemeDefaults` with
  `..Default::default()` and compiles without it, but its field is the presets' value
  and belongs with it); gpui 7.E's three `UNREAD_COLOURS` entries and accessors; iced
  7.F's two `ACCESSOR_ROWS` rows and one entry, accessors, parity exception and the
  regenerated `contract-leaves.toml`;
  egui 7.G entire (accessors, the four mapping rows, the counts, `mapping.md`, the
  install test). Why together: as U6.1, test by test (the registry, facts-rows and
  citation checkers; egui's leaf count, `mapping_tests.rs:234-262`, which counts the
  two header leaves from the field on as `null`, and the new defaults leaf, which
  resolves to a colour on every theme; egui's T2, satisfied once kde-breeze and the
  macOS presets state the leaves; the egui showcase's per-call route test; gpui's and
  iced's colour-leaf walks; the parity script); and
  `inheritance_rules_toml_covers_defaults_internal` fails between the
  `[defaults_internal]` rule and its implementation, `theme_defaults_field_info_matches_baseline`
  (`native-theme/src/model/mod.rs:1972-2009`) between the field and its baseline entry.
- **U7.2 — the carried modules and the KDE reader.** 7.B (the two files, the two tables,
  the declarations, `vector_tests.rs`), 7.D.2 and 7.D.6. Together because `kcu_tint` is
  dead code until the reader calls it (clippy `-D warnings` fails a private function no
  build uses; the allow of 7.B.3 covers `kde_color_common` only), and
  `the_readers_inactive_selection_is_the_tables` calls the reader's rule; and the
  `Scheme` pair of 7.D.6 with its three full literals (`BREEZE_LIGHT_SCHEME`,
  `BREEZE_DARK_SCHEME`, `a_colour_scheme_keeps_the_stated_edge_geometry`'s) and the
  accent test's `selection`, without which `kde/mod.rs`' test module does not compile.
- **U7.3 — the showcases.** 7.H entire, in one commit: the element-list edits of 7.H.1
  and 7.H.2, the three showcases' tables, lists, swatches, Widget Info and window-state
  code, gpui's `HOW` lines, iced's routes, the explicit activation in
  `the_basic_tables_infos_report_what_it_draws` (7.H.2's table) and the new tests
  (`check_showcase_parity.py --check-list`, gpui's `every_listed_leaf_has_a_route`,
  iced's route tests read the list and the code together; the activation keeps the one
  existing test that reads the table's selected row under kde-breeze on the active
  look, and no other existing test reads a row that swaps); and the workflow edit of
  7.H.2's capture check (`--dump-layout` on the macOS capture step and the four JSON
  files in its upload), which no gate runs before the release's asset run triggers the
  workflow, and which must be in place by then. In iced, the same commit holds the
  signature changes of 7.H.2 (`list_row`, `list_row_fill`, `TableColours::of`,
  `table_row_style`) with every caller and `drawn_by`'s arms, the new
  `header_section_style` with its `DRESSED` entry and `drawn_by` arm, and the routes:
  `styles_cover_every_widget_shown` and `every_hand_colour_route_names_a_function_that_reads_it`
  read them together. In egui, the same commit holds `list_row`'s change of parameter
  (7.H.2) with its caller.
- **U7.4 — outside the code.** 7.I.

The capture check of 7.H.2 is no part of a unit: the session runs it as a step of the
release's asset run (7.J.4), on the dumps U7.3's workflow edit makes.

Inside the units: 7.C with or before 7.D.3-7.D.4; 7.D.3-7.D.4 before the iced leaves
file is regenerated; 7.I's backlog with or after 7.D.3; 7.I.6 last.

### 7.J.2 The citations the edits move

As 6.I.2: none is re-pointed (S6). The edits of 7.A, 7.B.3, 7.C.2, 7.D.5, 7.E, 7.F, 7.G
and Part F move lines other files cite by number, among them the gpui `colors.rs`
comment at `:445`, which cites `docs/inheritance-rules.toml:245`; those citations are
left as they are and may drift. No pinned line moves (*Lines that must not move*:
`colors.rs` is not edited), and `contract-leaves.toml` is regenerated by its command
for 7.F's own rows, as U7.1 has it.

### 7.J.3 Gates

6.I.3's, and: the carried vector tests and `the_readers_inactive_selection_is_the_tables`
(7.B.4) in `cargo test -p native-theme --features "$NT_FEATURES"`; the macOS reader in
CI's macOS legs; `inheritance_rules_toml_covers_defaults_internal`.

**Not covered by any gate:** as 6.I.3; and the reader's exactness for schemes beyond the
two tables' rows, the swap in each showcase while unfocused (7.H.2), and the macOS
reader's new read outside CI.

### 7.J.4 The release

The one close-out of the release of both steps (S5), after the merge of the
predecessor's tag (S2 as amended), its warm-up gate and the recount on the merged
tree (rule 17):
the version, the compatibility run; then the
branch pushed once the compatibility run has passed, and the full
`scripts/check_release.sh` run while CI runs on it (S4); `scripts/generate_assets_release.sh`
starts only when both are green (open point 2), with one step more in the asset run: after `scripts/generate_assets_release.sh`
has downloaded the CI captures and before its assets are committed (`:297`), the
session runs 7.H.2's capture check on the run's macOS artifacts, the dumps of U7.3's
workflow edit among them; the maintainer decides only an undecided result, and a failed
dump stops the work for the maintainer before any asset is committed (7.H.2, *A failed dump*). Where it
finds an inactive window in a macOS capture, the change to the capture code that makes
the window active before `screencapture` runs (in `FrameCapture::start` or in an
earlier `poll` that returns `None`, not in `capture_macos`, 7.H.2) is made and
reviewed, gated `--commit` (whose `x86_64-apple-darwin` clippy and strict-panic legs
lint the macOS branch from Linux) and committed as a commit of its own, and the
close-out resumes at its first step (its stamp hashes the sources,
`tools/showcase-support/src` among them, `scripts/update_provenance.sh:62-63`), the
change pushed by the close-out's push step, after the compatibility run, before any
asset is committed.
Windows is left alone. Open point 2's time applies to each run.

## What was not verified

- Nothing was compiled or run: the carried modules under this tree's lints (7.B.5), the
  connectors' new code, the contract walks' results with the new leaves, the derive's
  output for the new soft options.
- Step 3's trial (3F) and its outcome (6.D); this specification gives the three cases
  of 6.D.2's condition, and whether 6.D.4's test passes 3F.3's criterion with the
  default button's colours is for the build to show. What step 6 adds to the upstream
  requests in case 3 is open (open point 3).
- The Basic page's column widths at 1280x720 (decision 7): whether the default button's
  own row pushes an element of the page out of its visible area under some preset is
  for 6.G's layout measurement to show.
- Clippy's `too_many_arguments` threshold of seven (7.H.2) is clippy's default as
  documented, not read in clippy's source here; the showcase's `tab_bar`, with eight
  parameters and the lint allowed (`demo.rs:1915-1925`), agrees with it.
- The earlier steps' specifications were read once, on 2026-11-14, while being amended:
  the names and outcomes of 3F, steps 2 and 3's facts rows (2B.1, 3B.1, re-read the same
  day), steps 4 and 5's `basic.icons.edit`, `toolbar_button_rest` and reply edits, and
  steps 2 and 3's reply rule (2I.3, 3H.3, re-read the same day) are as read then; a later
  change to them changes the parts of this document that name them (the header's list).
- The new preset comments (6.C.3, 7.D.3, 7.D.4) take the batch's form,
  `# platform-facts.md (§2.x): …`, which v0.7.1's LN unit (the GPUI Kit specification's
  Part I.3) makes `check_preset_citations.py` read; that unit is not in this tree, so
  the checker's exit 0 on them is the units' to show (the plan's A6 (d), A7 (d)), not
  this document's.
- macOS 14's label colour on the unfocused selection (measured on 15 and 26 only); the
  P8 fact for the primary label on that row does not exist, the cell cites the probe
  samples. (No release check reads `docs/evidence/`: `grep -n evidence
  scripts/check_release.sh` finds one comment, `:786`.)
- The macOS default button's facts for macOS 12-15 and its frame (6.B.3); the GNOME,
  Material and macOS header states (7.C.3); KConfig's boolean syntax (decision 11);
  whether Plasma copies `ChangeSelectionColor` into `kdeglobals` (7.D.2); the
  breezehelper lines for `hoverColor()` and `focusColor()` (taken from the reader's
  comment).
- The colour values of 6.C.1 and 7.D.1 were computed by a Python emulation of the
  reader's arithmetic (checked against five values the reader's tests or the presets
  already pin) and read from the vector tables, not by the crate.
- Whether any test outside `native-theme/src/kde/` and `native-theme/tests/reader_kde.rs`
  compares the KDE reader's whole output under Breeze with the preset (U6.1 and U7.1 put
  the preset values before the reader on the reading that none does; the tests of those
  two files were read).
- The committed macOS captures answer where the content sits, that the pixels are the
  presets' values, and, very likely, that the windows were active (7.H.2, "What the
  committed captures show"; the grey buttons of a non-key window are general macOS
  behaviour, not checked against a source here). Not verified: whether future runs stay
  so, which the asset run's check reads each time; how the capture code
  (`FrameCapture::start` or an earlier `poll`, 7.H.2) would make the window active on
  a runner, if it must; whether writing the layout dump beside the
  screenshot changes when a showcase takes its screenshot (each waits for its own
  output; the exit rule was read, not the timing). The CI run writes no layout dump
  today (read: the workflow); 7.H.2's workflow edit adds one per capture. Its failure
  mode, read: a dump not written within `CAPTURE_WAIT`, 20 s
  (`tools/showcase-support/src/lib.rs:99`), fails the showcase and so the CI capture
  step: iced's `Message::DumpTimedOut` refuses (`showcase-iced.rs:2900-2906`,
  `dump_given_up`, `:857-868`), and `cli::refuse` exits with status 1
  (`tools/showcase-support/src/cli.rs:485-488`); gpui's and egui's handling of a late
  dump was not read. On Linux `--dump-layout` runs only with `--capture`
  (`scripts/capture_window.sh:457-464`), and no script or workflow runs it with
  `--screenshot`, as the workflow edit would (`git grep -n -e '--dump-layout'` over
  `scripts/` and `.github/`, read today: `capture_window.sh:464` alone passes it).
- egui's per-state default button (6.G) rests on the previous frame's response, read
  from the page's memory; that egui keeps no other way to know a button's state before
  it is built was not looked for beyond `Button`'s own API.
- `VisualTestContext::deactivate_window` after `Window::activate_window` in one test
  (7.H.2) was not run; the activation path was read (`window.rs:6917`,
  `platform/test/window.rs:423-430`, `platform.rs:391-411`), and gpui-component's tests
  use the activation call the same way.
- The `check_section_refs.py` and `check_todo_citations.py` results on the new texts
  (not run).
