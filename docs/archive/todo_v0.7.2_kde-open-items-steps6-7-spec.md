# 0.7.2: issue #4's open KDE items, steps 6 and 7 (the default button; the table header's states and the unfocused selection) — specification

- Rationale: [`../issues/0004_kde-open-items-rationale.md`](../issues/0004_kde-open-items-rationale.md)
  ("the rationale"), decisions D8 (step 6), D7's second half and D9 (step 7), D0 for
  the version and what D10 says a step carries outside the code. Approved by the maintainer on
  2026-11-19: the rationale's D8, D7's second half and D9, this specification and its
  plan. It decides nothing the
  rationale did not decide, except the small points under *Decisions taken while
  specifying*; what needs the maintainer is under *Open points*. What the rationale's §0
  leaves in the backlog as it is stays there: among it the three more outlines the first
  rationale put into the same step as its items 5 and 6 (a hovered menu item's and a
  hovered list row's 1 px outline, and a menu bar title's fill), none of which these
  steps add (7.I.3 keeps the first rationale's sentence that says so).
- It continues [`../issues/0004_kde-hand-overrides-rationale.md`](../issues/0004_kde-hand-overrides-rationale.md)
  ("the first rationale"), §5.2 (the default button), §5.3 (the unfocused selection)
  and §4.4 (the header's fills), and the backlog entries `docs/todo.md:1774-1825`
  (*What Breeze draws that the model has no field for*), `:1826-1913` item (2) (*KDE
  values that wait for a connector design*), `:1914-1933` (*The selection of a view
  without the focus*) and `:543-580` (*Selection: the text colour macOS pairs with the
  unemphasised selection*). Form: [`todo_v0.7.1_kde-hand-overrides-spec.md`](todo_v0.7.1_kde-hand-overrides-spec.md)
  and step 1's [`todo_v0.7.1_kde-open-items-step1-spec.md`](todo_v0.7.1_kde-open-items-step1-spec.md).
- **The release (D0).** Decided by the maintainer on 2026-11-19: steps 6 and 7 are part
  of **0.7.2**, as steps 2 to 5 are, together with the text caret's blink
  ([`todo_v0.7.2_caret-blink-spec.md`](todo_v0.7.2_caret-blink-spec.md), "the caret
  document"); the version stays 0.7.2. (That morning the two works were set for a
  release of their own, 0.7.3; he folded them into 0.7.2, which is not released yet, so
  that no release changes the API a few days after another did.) That is D0 (a): a
  0.7.x release whose CHANGELOG names the break once. The works add to 0.7.2's branch,
  its CHANGELOG section and its ROADMAP section, and 0.7.2's close-out is made again
  after them, once, with one asset run and one archive. The caret's units are built
  first, then step 6's, then step 7's; each step keeps its own units, reviews and
  record; the close-out is this document's (7.J.4) and its plan's. Below, "the
  close-out" and "the release's asset run" are that second pass.
- Written 2026-11-14 against `v0.7.1-rc2` at `0fa28027`, for a release of its own;
  amended 2026-11-19 for 0.7.2. The work is built on the branch `v0.7.2-rc1`, from its
  head `da89b4ba` (the caret document's *Order and gates*, *The branch*), in the worktree
  `.claude/worktrees/lane2-b`. "0.7.2's tree" below is that tree, 0.7.2 as it stood
  before the caret's work and this one: egui 0.37.0, gpui-component and gpui-base 0.10.0, gpui-pre 0.3.12, steps
  2 to 5 of this issue and gpui-component's own icons; iced is 0.14.0 with iced_widget
  0.14.2. `P8` is the main checkout on `v0.8.0-rc1` at `78ee0efd`, `F` its `docs/facts/`;
  `GC`, `IW`, `EG`, `EX` are the `src/` folders of gpui-component 0.10.0, iced_widget
  0.14.2, egui 0.37.0 and egui_extras 0.37.0 in Cargo's registry. `<record>` is
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.3/`, the
  record of both works (the folder keeps the name it got while they were set for 0.7.3).
- **Every edit is applied by its anchor** (the quoted text, the key, the test's name).
  A `file:line` of a repository file is `0fa28027`'s and serves to find the place; none
  is re-pointed, and a new citation of a repository document names the document, with a
  quoted phrase or a row's name where that helps, never a line number (the maintainer,
  2026-11-15; the batch's S6); a pinned source (an upstream crate at a version, a
  citation "at <sha>") keeps its lines. The anchors, the counts and the upstream lines
  were read again on 0.7.2's tree on 2026-11-19 (`<record>/respec/findings-k02-6ABC.md`
  to `-k09-7H.md`): each part states that tree's facts, and a count is given as that
  tree's value and what the units before it and its own unit make of it.
- **What is in the tree a unit starts from.** 0.7.2's tree; the caret's blink
  (three `u32` fields of `InputTheme`, three rows in §2.4 of the facts, lines in the
  presets' input sections, three leaves in egui's mapping; no colour leaf, no derived
  leaf of the KDE reader); and for step 7, step 6. Step 6 builds on what steps 3 and 5
  left: `button.hover_border_color` (decision 20), `variants::bordered_button` with
  `variants::hover_border` and its kept trial test
  `bordered_button_matches_the_default_button_but_its_hover_border`
  (`docs/archive/todo_v0.7.2_kde-focus-ring-and-button-outlines-spec.md`, 3F), the gpui
  builders `variants::toolbar_button_states`, `variants::toolbar_button`,
  `geometry::toolbar_button_rest` and `geometry::toolbar_button_checked`, whose form
  6.D's follow, and the checked tool button `basic.icons.edit` in the Basic page's icon
  row (`docs/archive/todo_v0.7.2_kde-hover-outlines-and-toolbar-buttons-spec.md`, B.9,
  B.10). Step 7 builds on step 1's carried module `native-theme/src/kde/shade.rs` and
  its folder `native-theme/tests/vectors/`
  (`docs/archive/todo_v0.7.1_kde-open-items-step1-spec.md`, Part A).
- **Taken before the build** (2026-11-19, in `<record>/respec/` and `<record>/trial/`):
  the parts' re-read; and the trial T-K1: a custom variant built as 6.D.2 says draws
  Breeze's default button at rest, hovered, pressed and disabled, under kde-breeze light
  and dark, on gpui-pre and on GPUI Fast (`trial/t-k1-result.txt`; its test,
  `trial/t-k1.patch`, is where 6.D.4's starts). So gpui has a receiver for the default
  button, and 6.D holds one case. The trial T-K2: where the default button stands on the
  Basic page, measured headless in the three showcases (decision 7; `trial/t-k2/`). The
  trial T-K3: in one gpui test a window is made active, inactive and active again, the
  window's state and an activation observer following each change, on both backends; a
  newly opened test window is not active (`trial/t-k3-result.txt`; 7.H.2's tests start
  from `trial/t-k3.patch`). The trial T-K4: the two carried files pass this tree's
  strict-panic lints as they are, and clippy's one finding is `kcu_tint` unused before
  the reader calls it (`trial/t-k4-result.txt`; 7.B.5).
- The batch's working rules are cited as "rule N": rule N (batch-speed rationale,
  Appendix), the consolidated list in `docs/todo_v0.7.1_batch-speed-rationale.md`. Its
  rules about lanes, a branch's start and the merge of another release's tag (rules 7,
  17 to 21 and 25) have nothing to apply to here: the release is built on one branch,
  in one worktree.
- Every value below was read from the file named, or computed; the computations are
  named where they are given. What could not be verified is marked so, with the reason.

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
| 6.D — gpui connector | D8 (gpui); the trial T-K1 | `connectors/native-theme-gpui/src/variants.rs`, `src/geometry.rs`, `src/contract.rs`, `README.md`, `src/lib.rs` (module doc), the `GEOMETRY_NOTES` entry in `examples/showcase-gpui/info/mod.rs` |
| 6.E — iced connector | D8 (iced) | `connectors/native-theme-iced/src/styles.rs`, `src/contract.rs`, `src/contract/rows.rs`, `src/contract/derived.rs`, `src/contract/pairs.rs`, `contract-leaves.toml` (generated), `README.md` |
| 6.F — egui connector | D8 (egui) | `connectors/native-theme-egui/src/accessors.rs`, `src/lib.rs`, `mapping.toml`, `src/mapping.md` (generated), `src/mapping_tests.rs`, `src/install_tests.rs`, `README.md`; `scripts/connector_parity_exceptions.toml` |
| 6.G — Showcases | D8 | the three showcases' Basic page, `docs/showcase-elements.toml` |
| 6.H — Outside the code | D10 | `CHANGELOG.md`, `ROADMAP.md`, `README.md` and `native-theme/README.md` (6.H.2a), the first rationale, the rationale's status line, `docs/todo.md` |
| 6.I — Order and gates | D10 | step 6's units and their gates (the close-out and its asset run are 7.J's) |
| 7.A — The model | D7, D9 | `native-theme/src/model/widgets/mod.rs`, `native-theme/src/model/defaults.rs`, `native-theme/src/model/resolved.rs`, `native-theme/src/model/mod.rs` (one test), `native-theme/src/resolve/validate.rs`, `native-theme/src/resolve/inheritance.rs`, `native-theme/src/resolve/tests.rs`, `native-theme/tests/proptest_roundtrip.rs`, `native-theme/tests/serde_roundtrip.rs` |
| 7.B — The carried clean-room modules | D9 | new `native-theme/src/kde/kde_color_common.rs`, `native-theme/src/kde/kcu_tint.rs`, `native-theme/tests/vectors/kcu_tint.tsv`, `native-theme/tests/vectors/kde_selection_inactive.tsv`, new `native-theme/src/kde/vector_tests.rs`, their declarations in `native-theme/src/kde/mod.rs` |
| 7.C — Registry, rules, facts | D7, D9 | `docs/property-registry.toml`, `docs/inheritance-rules.toml`, `docs/platform-facts.md` §2.1.3, §2.15 |
| 7.C2 — Evidence carried from the parked branch | D9 (macOS) | new files under `docs/evidence/os-probe/2026-10-31-run-41195846264/` and `docs/evidence/os-probe/2026-11-06-run-42215651196/`, two rows of `docs/evidence/README.md` |
| 7.D — Presets and readers | D7, D9 | `kde-breeze.toml`, `macos-monterey.toml`, `macos-tahoe.toml`, `native-theme/src/kde/colors.rs`, the tests in `native-theme/src/kde/mod.rs`, `native-theme/src/macos.rs` |
| 7.E — gpui connector | D7, D9 | `connectors/native-theme-gpui/src/lib.rs`, `src/contract.rs` |
| 7.F — iced connector | D7, D9 | `connectors/native-theme-iced/src/lib.rs`, `src/contract/rows.rs`, `src/contract/derived.rs`, `contract-leaves.toml` (generated); `scripts/connector_parity_exceptions.toml` |
| 7.G — egui connector | D7, D9 | `connectors/native-theme-egui/src/accessors.rs`, `src/lib.rs`, `mapping.toml`, `src/mapping.md` (generated), `src/mapping_tests.rs`, `src/install_tests.rs`, `README.md` |
| 7.H — Showcases | D7, D9 | the three showcases' tables, lists and their tests, `docs/showcase-elements.toml` |
| 7.I — Outside the code | D10 | as 6.H |
| 7.J — Order, gates, close-out | D10 | step 7's units and their gates; the close-out (7.J.4) with its asset run, the capture check (after step 7's push and again in the asset run), and `tools/showcase-support/src/capture.rs` where that check finds an inactive window |

Every edit has one owner within a step. Files edited in both steps (`widgets/mod.rs`,
`kde/colors.rs`, `kde/mod.rs`, `kde-breeze.toml`, the registry, the rules, the facts,
the connectors' contracts and mapping, the parity exceptions, the showcases) are edited
by step 6 first and by step 7 on the tree step 6 left.

**Lines that must not move.** The gpui showcase's test
`every_colour_claim_is_read_at_the_line_it_cites`
(`connectors/native-theme-gpui/src/showcase.rs`) opens each colour claim at the line it
cites: a unit that moves a cited line either keeps the line count above it, which
every edit below does, or re-points those claims in the same commit. Listed on 0.7.2's
tree from every `<file>.rs:N` in `examples/showcase-gpui/` and `src/showcase.rs`, for
the connector's files this release edits or might: `colors.rs` (140 citations, lines
281 to 724); `variants.rs:62-64` (`ghost_button`'s three setters), `:134-136`
(`bordered_button`), `:144` (`hover_border`) and `:163-165` (`toolbar_button_states`);
`geometry.rs:230`, `:740`, `:749`, `:784`, and `:1463` and `:1468` in
`toolbar_button_checked`. The claims also pin lines in the widgets' files
(`src/widgets/checkbox.rs`, `switch.rs`, `tab_bar.rs`, `slider.rs`, `spinner.rs`,
`progress.rs`, `separator.rs`, `mod.rs`), which no unit of this release edits: 6.D.4's
test goes into `src/widgets/tests.rs`, which no claim cites. Neither step changes a
line of `colors.rs`. Every addition to `variants.rs` goes after `toolbar_button`, its
last function before the test module (`#[cfg(test)]`), below the last cited line; to
`geometry.rs` after `toolbar_button_checked` and before `list_item_selected`, below
the last cited line; to the connector's `lib.rs` after `selection_inactive_background`
(7.E), a file no colour claim cites. This release's own claims (6.G) pin the lines
they cite in turn. It writes a line number of a changing file in two places only, each
read by a gate: the gpui colour claims at `variants.rs` and `geometry.rs` lines, which
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
   (`EG/widgets/button.rs:153-167`), so one call
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
7. **6.G. The showcases show the default button on the Basic page, after the two
   toggles in the Buttons group's third row**, as a new shared element
   `basic.buttons.dialog_default` reading "OK". That row wraps in all three showcases
   already, so the button stands on the toggles' line where the column has room and on
   a line of its own below them where it has none. Options: (a) a row of its own, the
   group's fourth; (b) the first row, beside the primary button; (c) the third row,
   after the toggles; (d) each showcase's dialog. **(c)**, by measurement (0.7.2's
   layout dumps, and the trial T-K2 of 2026-11-19, `<record>/trial/t-k2/RESULTS.md`;
   6.G has its table):
   - (a) adds a line's height, 38 to 46 px, to column 1 under every preset. Under
     material-3, whose page has four columns with column 1 full, the Expander group's
     heading lies 2 to 12 px inside the page's visible area in the three showcases
     today and would leave it; the cross-platform dumps gave margins below the line's
     height under windows-11, the macOS presets and ios-13 too.
   - (b) The first row has 3 to 56 px of room under kde-breeze, adwaita,
     catppuccin-mocha, material-3 and the macOS presets, against a button of at least
     64 px and its gap, so the button would wrap there and cost what (a) costs.
   - (c) The toggles are the narrowest pair, so their row has room wherever the page
     has four or three columns: under material-3 the button stays on their line with
     9 to 12 px to spare and nothing moves. Where the page has five columns
     (kde-breeze, adwaita, catppuccin-mocha) it takes a line of its own, the groups
     below it in column 1 move down by 38 to 40 px, and column 1 still ends 86 to
     176 px above the end of the page's visible area. No element leaves its visible
     area under any entry, and the three showcases agree on every one. Under
     kde-breeze, the one preset whose default button has a look of its own, the button
     so stands on a line of its own, as (a) would have put it.
   - (d) has no confirming button in iced (`showcase-iced.rs:8880-8936` draws a close
     button only), so the three would not show the same element, and no capture shows
     a dialog.

   Steps 4 and 5 added their checked tool button to another group, the icon row
   (`basic.icons.edit`, their B.10), so the two additions do not meet. What the
   platforms' own fonts make of the row on macOS and Windows is seen in the asset
   run's captures (7.J.4); the Linux cross-platform pass kept the button on the
   toggles' line under windows-11, macos-tahoe and ios-13, and under macos-monterey
   met a difference that is older than the button (egui's five columns against iced's
   and gpui's four, 6.G).
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
    - **gpui tests.** The harnesses `open_with` (`showcase-gpui/tests.rs:119-139`) and
      `open_set_up`, which opens the window for it and for `open_with_widget_icons`,
      stay as they are: their window is not active (the test platform's windows start inactive,
      7.H.2). Activating every test's window would change more than the selection: the
      title bar, which the showcase draws from `window.is_window_active()`
      (`showcase-gpui/app.rs:2396`, into `chrome::window_top`); the caret, which
      gpui-base blinks and paints only in an active window (gpui-base 0.10.0
      `input/base/state.rs:959-971`, `:4071-4076`); the selection inside text fields
      (`input/base/element.rs:3968`); and the window border's shadow (gpui-component
      0.10.0 `window_border.rs:200`, `:259`). So each test that reads the Basic table's
      selected row under a preset the rule selects makes its own window active, and each
      new test of the swap makes it inactive, both explicitly, with a comment saying
      why (7.H.2 lists them).
    - **iced and egui.** iced's state starts active (`window_active: true`) and follows
      the window's `Focused` and `Unfocused` events (starting it `false`, as egui-winit
      starts, would make the two agree in a window that never gets the focus, but it
      would rest on iced delivering a `Focused` event at start, which was not read, and
      would turn every existing iced test's window inactive; `true` is the state a
      missed first event leaves right); iced 0.14 offers no query of the
      window's focus (iced_runtime 0.14.0 `window.rs` has `gain_focus`, `:431`, which
      takes the focus, and nothing that reads it). egui's follows `RawInput::focused`,
      whose default is `true` (egui 0.37.0 `src/data/input/raw_input.rs:106`),
      which `egui_kittest` keeps (it builds its input with `..Default::default()`,
      egui_kittest 0.37.0 `src/lib.rs:238-241`), so the egui tests see an active window;
      but egui-winit starts a real window at `focused: false` until winit reports focus
      (egui-winit 0.37.0 `src/lib.rs:163`, "winit will tell us when we have focus"). So the two showcases differ until the window's first focus event: a
      real iced showcase draws the active pair, a real egui showcase the unfocused one
      (where the rule selects it); after that event both follow the window.
    - **Captures.** After step 7's push, on captures of the screenshots workflow, and
      again during the release's asset run, before its assets are committed, the
      session checks the twelve macOS captures for an inactive window: it counts
      the active selection fill's pixels in each and holds the count against the
      captures of 0.7.2's first asset run (`b494d6ed`), in which the windows were very likely active (7.H.2, *The capture
      check*; no workflow edit and no layout dump is needed for it). Where it finds an
      inactive window, the capture code makes the window active before it captures
      (7.H.2, 7.J.4), not the showcase.

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

1. **The version (D0).** Settled by the maintainer on 2026-11-19: 0.7.2, with the
   caret's blink (the head). D0 (a): the release's CHANGELOG section has one *Breaking
   Changes* bullet naming every struct the release gives a field (6.H.1).
2. **The asset run** needs the maintainer's desktop unlocked and untouched for 4 to 6
   minutes, once for the release, in its close-out (7.J.4). Step 6's layout check
   (6.G) runs headless and needs no desktop.

Nothing else is open. Settled before: the showcase lists (decision 16), macOS's text
colour (decision 17), KColorScheme's other inactive effects (decision 18), the default
button's hover frame and the window's state (decisions 20 and 21). Settled by
measurement on 2026-11-19: that gpui has a receiver for the default button (the trial
T-K1; 6.D.2), and where the default button stands on the Basic page (decision 7; the
trial T-K2). The reply on the issue was posted on 2026-11-17, so no step edits it
(7.I.3).

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

**This document's three insertions** (decision 1; numbers at `0fa28027`). 0.7.2's steps
inserted rows above and below them (one of step 2's above `primary_text_color`, step
3's two below it, before `hover_background`), and the caret inserts its three in §2.4,
so each anchor row is found by its text:

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
0.7.2's steps and the caret insert lines into the same presets first; each edit is
placed by its anchor text on the tree its unit starts from.

---

# Step 6 — the default button (D8)

## 6.A The model

### 6.A.1 `native-theme/src/model/widgets/mod.rs`, `ButtonTheme`

After the struct's last soft option before `font` (`:98` at `0fa28027`): step 3's
`focus_border_color`, which today's struct has after `hover_border_color`, both after
`checked_text_color` (`:94-97` at `0fa28027`). Three soft options in the form of the
struct's other optional colours (`#[theme(category = "soft_option")]`, `:81-97`):

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
  `default_button_text_color: None`, `default_button_border_color: None` after step 3's
  `hover_border_color: Some(Rgba::rgb(0, 120, 215))` and `focus_border_color: None`,
  which follow `checked_text_color: None` (`:1848` at `0fa28027`), and after step 3's
  two asserts, the last `assert_eq!(resolved.focus_border_color, None)`, which follow
  `assert_eq!(resolved.checked_text_color, None)` (`:1879` at `0fa28027`), the three
  asserts `Some(Rgba::rgb(210, 230, 245))`, `None`, `None`. (The colour is a test
  literal, not a theme value.)
- `native-theme/tests/proptest_roundtrip.rs`, `arb_button_theme` (`:226-259` at
  `0fa28027`): the colour vector grows by three and the three fields take the next three
  indices. Today the vector is `13..=13`, with `hover_border_color: colors[11]` and
  `focus_border_color: colors[12]` (step 3's), and the caret's units do not touch the
  function (they extend `arb_input_theme`), so these take `colors[13]`, `colors[14]`,
  `colors[15]` and the vector becomes `16..=16`; the strategy's comment, today
  "Generate all 13 colours, 4 scalars, the font and the border", says 16. Every other
  full struct literal of `ButtonTheme` (found with a scan of
  `git grep -nE '(^|[^A-Za-z])ButtonTheme \{'` over `*.rs` for literals without `..`,
  the same result in today's tree): only `widgets/mod.rs:1833` above and this one.

### 6.A.3 `native-theme/src/presets.rs`, `platform_presets_no_derived_fields`

In the test, the comment above the `button.primary_background` assertion (`:423-425` at
`0fa28027`), found by its words "KDE's default button": its clause from "KDE's default
button is mix(Button, Highlight, 0.2)" to the end of the comment, which ends with a
facts line number ("and waits for the connectors (docs/platform-facts.md:1188, §2.3)",
unchanged in today's tree), becomes "KDE's default button has fields of its own,
`default_button_*` (docs/platform-facts.md §2.3)". After that
assertion, a new one: `variant.button.default_button_background.is_some() == (*name == "kde-breeze")`,
message "preset '{name}' {label}.button.default_button_background: stated only where the
default button is not the primary one (KDE)".

### 6.A.4 The gates a new model field passes first

| Gate | What it checks | This step |
|---|---|---|
| `property_registry_keys_match_derive_inventory` (`native-theme/src/model/mod.rs:2055`) | the registry's keys equal the derive's `FIELD_NAMES` per struct | 6.B adds the three keys |
| `scripts/check_platform_facts_rows.py` | every registry field has a facts row in its section | 6.B adds the rows to §2.3 |
| `no_inheritance_fields_are_not_inherited` (`resolve/inheritance.rs:816`) | no `[no_inheritance]` field is an inheritance target | the three are listed there and inherit nothing |
| `every_bundled_preset_source_lints_clean` (`presets.rs:211`) | every preset key is a model key | the new kde-breeze keys exist once 6.A lands |
| `scripts/check_preset_citations.py`, `check_section_refs.py`, `check_wrong_safety_nets.py`, `check_todo_citations.py` | the preset comments' citation form (nothing is compared with the facts rows: the script's docstring, "this script compares no citation with the facts file's rows"; v0.7.1's LN unit, the GPUI Kit specification's Part I.3, `docs/archive/todo_v0.7.1_gpui-kit-0.9.1-spec.md`); section references; the safety-net marks; backlog key paths | 6.B, 6.C, 6.H |
| egui: `every_leaf_has_exactly_one_row_and_every_row_is_well_formed`, `verdict_totals_match_the_specification`, `t2_every_row_s_declared_sinks_are_exactly_what_its_leaf_moves` (`mapping_tests.rs`) | one mapping row per leaf; §5.7's totals; what a leaf moves | 6.F |
| gpui: `every_colour_leaf_reaches_a_sink_or_is_named` (`contract.rs:2933`) | each colour leaf reaches a sink or is named | 6.D |
| iced: `every_colour_leaf_reaches_a_row_or_is_named` (`contract.rs:5578`) against `contract-leaves.toml` | the same, by rows | 6.E |
| `scripts/check_connector_parity.py` | crate-root functions alike in the three | 6.F's accessor, two exceptions |
| `scripts/check_showcase_parity.py --check-list` and the static parity modes | the shared element list | 6.G's element |

The connectors' and showcases' gates a new leaf also meets are in 6.D to 6.G.

## 6.B Registry, rules, facts

### 6.B.1 `docs/property-registry.toml`, `[button]` (`:180-198`)

After step 3's `focus_border_color = "color"`, which today's table has after
`hover_border_color = "color"`, both after `checked_text_color = "color"` (`:196` at
`0fa28027`), so that the keys stand in the struct's order:

```toml
default_button_background    = "color"
default_button_text_color    = "color"
default_button_border_color  = "color"
```

### 6.B.2 `docs/inheritance-rules.toml`

In `[no_inheritance]` `fields`, after step 3's two entries, `"button.hover_border_color"`
and `"button.focus_border_color"`, which today's list has directly after
`"button.checked_text_color", …` (`:412` at `0fa28027`), so that the button's entries
stand in the struct's order, as 6.B.1's keys do:

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
found by its text (step 2's row after `focus_ring_offset` stands above it and moves it;
the caret's rows go to §2.4, below it), before `disabled_opacity`, which follows it
directly in today's tree, no blank line dropped (F.1). The two primary rows' cells above
are found the same way (`:1188-1189` at `0fa28027`). The KDE cells take the text the two
primary cells hold, read at `:1188` and `:1189` at `0fa28027` and unchanged in today's
tree (their links copied as they are); the macOS and Windows
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
`value = "#ffffffff"`. For macos-monterey the parked facts give the default push
button's fill in an active window as pending (`button.body.fill`, `states =
["default"]`, U141, and U133 for `not_main`) and state it only for an inactive window;
no plain push button's default frame (`button.body.stroke`, `states = ["default"]`) has
a value on either macOS preset (macos-tahoe's two are pending, U95, U97) (searched
2026-11-19 by key and `when` in `F/macos/button.toml`).

### 6.B.4 What the checkers require

`check_platform_facts_rows.py`: the three keys under `[button]` (§2.3) find rows of
their name in §2.3. `check_section_refs.py`: the `[no_inheritance]` comments cite §2.3,
which exists. `check_preset_citations.py` (as v0.7.1's LN unit, the GPUI Kit
specification's Part I.3, left it) reads 6.C.3's comments in the batch's form and
compares nothing with the facts rows. `check_wrong_safety_nets.py`: no entry or mark changes.

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

**Constants**, after `PRESSED_BUTTON_MIX` (`:369` at `0fa28027`) and step 1's
`MENU_PANEL_MIX`, which follows it directly in today's file (before `TAB_HOVER_ALPHA`),
each in the file's form of one constant per use with its citation:

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

**`populate_colors`**, after the hovered button's two lines (`:540-541` at `0fa28027`)
and step 3's two lines after them, `variant.button.hover_border_color = selection_bg;`
and `variant.button.focus_border_color = selection_bg;` with their comment ("The pen of
a non-flat button turns the active group's `Highlight` …"), the last lines of the
`// Button` block before `// Tooltip`, using the function's `selection_bg` binding
(`let selection_bg = get_color(ini, "Colors:Selection", "BackgroundNormal");`, bound
once beside `view_focus` at the function's top) and its `button_fg` binding (the
checked button's), so no second read of either key:

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
list of computed colours and in its parenthesis that lists the blends on the accent,
today "(the checked checkbox, the progress fill, the switch's on track and its outline,
the list's hover, and the toolbar button's pressed fill)". A missing input gives `None` and the preset's value stands,
as every derivation of the reader.

### 6.C.3 `native-theme/src/presets/kde-breeze.toml`

`[light.button]`: the comment `:148-151` at `0fa28027` ("# primary_background /
primary_text_color not stated: platform-facts.md:1188-1189 (§2.3), Breeze's default
button is …", four lines, unchanged in today's tree) is replaced by, in its place, in
the batch's preset comment form, today's with the line number removed (S6:
`# platform-facts.md (§2.3): …`, and for a key not given
`# K not stated: platform-facts.md (§2.3), …`, the form step 3's comment above it
takes; `scripts/check_preset_citations.py` reads this form and compares nothing with
the facts file, v0.7.1's LN unit):

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

`[dark.button]` (`:733-736` at `0fa28027`, the same comment with #2d4655 and #fcfcfc):
the same, with `#2d4655`, `(BreezeDark.colors:40, breeze
v6.7.5)` and ForegroundNormal `252,252,252`, `#fcfcfc`, `#292c30`, `#535659` and
`#4882a1`. The line numbers of `BreezeLight.colors:39` and `BreezeDark.colors:40` are the
file's own (`kde-breeze.toml:161`, `:746`), pinned sources: breeze at a version. The blank lines after the block stay. In today's
`[light.button]` and `[dark.button]` the keys before the comment are step 3's
`hover_border_color` and `focus_border_color` with their comment, after
`checked_text_color`, so directly before the comment this block replaces; no other step
of 0.7.2 added a key to either table (step 5's went to `[light.toolbar]` and
`[dark.toolbar]`), and the caret's units add theirs to `[light.input]` and
`[dark.input]`. The block stands in the comment's place, after step 3's keys.

### 6.C.4 Tests, `native-theme/src/kde/mod.rs` and `kde/colors.rs`

| Test | Change |
|---|---|
| `derived_leaves`, `resolved_leaves` (`kde/mod.rs:1246-1372` at `0fa28027`; 39 leaves in today's tree, `[(&'static str, Option<Rgba>); 39]`, the doc "The thirty-nine colour leaves", the last `("toolbar.button_checked_background", …)`; the caret's units add none) | append `("button.default_button_background", …)`, `("button.default_button_text_color", …)`, `("button.default_button_border_color", …)`; the resolved side reads `theme.button.default_button_*` (already `Option`s); the array's length and the doc's count grow by three, to 42 ("forty-two") |
| `the_derived_colours_are_the_presets_under_breeze` (`:1397-1414`) | passes with 6.C.2 and 6.C.3, no edit |
| new, `kde/colors.rs`, after the pressed-button test (`the_pressed_hovered_and_focused_colours_come_from_their_keys`, `:908-930`, whose `:915-918` asserts `active_background`): `test_default_button_is_breezes` | built on the tests' helper `breeze_light_and_dark()`, whose groups hold Breeze's Button, ButtonText and Selection keys: Breeze Light's give #d6ecf8, #232629, #87c0dd; Breeze Dark's #2d4655, #fcfcfc, #4882a1 |
| new: `test_default_button_frame_keeps_qts_precision` | the light frame is `Rgba::rgb(0x87, 0xc0, 0xdd)`, and `mix(mix(..))` through 8 bits would be `0xde` in blue: the test computes that and `assert_ne!`s it, so a refactor that rounds early fails |
| `test_full_mapping_completeness` (`kde/colors.rs:1713-1861`) | asserts the three `is_some()` (the `BREEZE_DARK` fixture has Button and Selection, `:770-830`) |

The tests that compare an exact list of moved leaves (`moved_leaves`, `kde/mod.rs:1384-1393`,
in `resolved_leaves`' order, so a new leaf joins each list at the end, after what the
earlier steps appended), found by reading every `moved_leaves(…)` call of
`native-theme/src/kde/mod.rs` (`:1420-1581` at `0fa28027`; today eight tests, read with
`grep -n 'moved_leaves(' native-theme/src/kde/mod.rs`; no other file of
`native-theme/src/kde/` or `native-theme/tests/reader_kde.rs` compares an exact list or
count of derived or moved leaves: `grep -n 'moved_leaves\|derived_leaves\|len()'` over
them, run again today), each with what step 6 adds, read from what each leaf reads
(6.C.2):

| Test | Its scheme | Gains |
|---|---|---|
| `an_accent_moves_the_on_track_the_pressed_button_the_selected_segment_and_the_list_hover` (`:1420-1438`) | `selection: "233,84,32"` | `button.default_button_background`, `button.default_button_border_color` (both read `[Colors:Selection] BackgroundNormal`); the test's doc comment names the default button's fill and frame among what moves |
| `the_button_set_moves_the_button_slider_switch_and_header_leaves` (`:1441-1465`) | `button: ("200,180,150", "60,20,10")` | the three (fill and frame read the Button background, the text and frame its text) |
| `the_header_set_moves_the_toolbar` (`:1470-1480`) | Header set | none |
| `the_window_set_moves_the_popover_edge` (`:1486-1522`), both schemes | Window set | none |
| `the_view_set_moves_the_card_edge_and_the_menu_panel` (`:1525-1532` at `0fa28027`, under step 1's name) | View set | none |
| `the_view_decorations_move_the_frames_the_tab_hover_and_the_handle` (`:1538-1561`), hover and focus | decorations | none |
| `the_frame_contrast_moves_the_edges` (`:1564-1581`) | `frameContrast` 0.5 | `button.default_button_border_color` (its inner mix) |
| `the_contrast_moves_the_table_grid` (step 1's), both schemes | `[KDE] contrast` 0 and unset | none (no default-button leaf reads `contrast`) |

`a_custom_accent_reaches_the_accent_fills` (`kde/colors.rs:850-902`) asserts only that
three named fields moved and fails nothing when a leaf is added: no edit.

## 6.D gpui connector

### 6.D.1 What draws a default button in gpui today

gpui-component's dialog builds its OK button itself, `ButtonVariant::Primary` unless the
caller passes another (`GC/dialog/dialog.rs:159`, `ok_variant.unwrap_or(ButtonVariant::Primary)`;
`DialogButtonProps::ok_variant`, `:66-69`). `Primary` fills `button_primary` and letters
`button_primary_foreground` (`GC/button/button.rs:1037`, `:1051-1057`, `text_color`'s `Primary` arm), which the
connector sets from `primary` and `primary_foreground` (`connectors/native-theme-gpui/src/colors.rs:388-391`),
that is `button.primary_background` and `primary_text_color` (`:203-204`); its frame is
`primary` (`button.rs:1104`). So today a gpui dialog's default button is the primary
button on every preset, kde-breeze's #3daee9 under #ffffff / #fcfcfc included. Nothing
in the connector reads the new fields; `primary` keeps its source (D8: "No existing
reader changes").

### 6.D.2 The variant

D8: "gpui a variant for a dialog's confirming button, which is a custom variant and so
rests on D5's trial for its border". Step 3's Part 3F built that trial, and it passed
(3F.3, *The outcome*): `variants::bordered_button(n, cx) -> Option<ButtonCustomVariant>`
(3F.1) and the crate-visible `variants::hover_border` it reads (3F.4) are in
`connectors/native-theme-gpui/src/variants.rs`, held to gpui-component's Default button
state by state by `bordered_button_matches_the_default_button_but_its_hover_border` in
`connectors/native-theme-gpui/src/widgets/tests.rs` (3F.2), with a strict criterion (3F.3:
every compared item equal but the hover and press border; the test asserts that no other
difference is found), and `button.hover_border_color` reaches the contract's `sinks`
through the entry `variants::hover_border` (`connectors/native-theme-gpui/src/contract.rs`,
`sinks`), not an `UNREAD_COLOURS` entry. Its construction: the
variant's `color` is the border of every hovered and pressed state, `hover` and
`active` the fills there (`GC/button/button.rs:1134-1140`, `:1225`, `:1310`), its
`foreground` the label in every state (`:1096`); the rest fill and rest frame the
variant cannot give (its own rest fill is a fifth of `color`, `:1044`) come from the
caller's refinement, `.refine_style(&geometry::button(n))` and `.bg(..)`, which upstream
replays at rest and its hover and press styles replace (`:733-767`).

Because the construction needs the caller's refinement, it reaches a button the
application builds, and not one gpui-component builds itself. In a dialog: a plain
`Dialog` renders no buttons from `button_props`, and its footer is the application's
own (`Dialog::footer`, `GC/dialog/dialog.rs:416-427`, whose doc says so), so a
`variants::default_button` button goes there like any other; an `AlertDialog`'s
default footer builds its OK button from a `ButtonVariant` alone (`render_ok`,
`:149-162`, `ok_variant.unwrap_or(ButtonVariant::Primary)`; `DialogButtonProps::ok_variant`,
`:66-69`), so that button stays `Primary`, and the README says so.

**gpui has a receiver for the default button.** The trial T-K1 ran on 2026-11-19
(record: `<record>/trial/t-k1-result.txt` and `<record>/trial/t-k1.patch`): a custom
variant built as this section says draws Breeze's default button at rest, hovered,
pressed and disabled, under kde-breeze light and dark, on gpui-pre and on GPUI Fast,
with no differing state. 6.D.4's widgets test starts from that trial's test,
`trial_default_button_draws_breezes_default_button_in_every_state` in `t-k1.patch`,
which builds the variant from constants; the unit's test takes it from
`variants::default_button` and `geometry::default_button_rest` and reads kde-breeze's
values from the preset.

- **The functions**, in `variants.rs` after `toolbar_button`, the last function
  before the test module (`#[cfg(test)]`), so that no line the showcase's claims
  cite moves ("Lines that must not move": the last cited line of `variants.rs` is in
  `toolbar_button_states`, above `toolbar_button`), in `bordered_button`'s form:

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
  `rust,no_run` example in the form of `bordered_button`'s, which shows the caller's
  `.custom(v)`, `geometry::button` and the rest and disabled refinements; here the
  caller's `.custom(v)`, `.refine_style(&geometry::button(n))` and, for an enabled
  button, `.refine_style(&rest)` with `rest` from `geometry::default_button_rest(n)`; a
  disabled one takes `geometry::button_disabled(n)` in place of `rest`, so that its
  border stays `geometry::button`'s `button.border.color` (decision 3); unlike 3F.1's
  disabled button, which takes its rest fill and then `button_disabled` (whose fill
  replaces it), it must not take `rest`, whose frame `button_disabled` would not
  replace: `button_disabled` sets a fill, a text colour and an opacity and no border
  colour (`CG/geometry.rs:1344-1352`), and upstream replays a refinement in the
  disabled state too (steps 4 and 5's B.9; `GC/button/button.rs:834-851`).
  A custom variant has one foreground for every state (`GC/button/button.rs:1096`), so
  gpui's pressed default button keeps `default_button_text_color` (else `font.color`)
  where decision 3 gives the plain button's pressed label, `button.active_text_color`,
  as iced's and egui's do. Under kde-breeze the two are equal: `active_text_color` is
  unstated there and inherits `button.font.color` (`docs/inheritance-rules.toml:170`),
  #232629 / #fcfcfc, and `default_button_text_color` is the same colour (6.C.1). On a
  theme that states the two apart, gpui's pressed label differs; the variant's doc and
  its HOW line say so.
  - **What the variant carries** (decision 24), in `variants.rs` before `default_button`
    (both after `toolbar_button` and before the test module), as steps
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
  - **The rest look**, `CG/geometry.rs`, after `button_checked` (`:1354-1378`) and the
    two builders steps 4 and 5 put after it, `toolbar_button_rest` and
    `toolbar_button_checked`: directly after `toolbar_button_checked` and before
    `list_item_selected`, below the last line of `geometry.rs` the showcase's claims cite
    (in `toolbar_button_checked`), in the form of `toolbar_button_rest`:

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
    (`showcase-gpui/info/mod.rs:214`, one per `pub fn` of `geometry.rs`;
    `every_geometry_builder_has_a_note`, `CG/showcase.rs:572`, checks that every
    builder has an entry, that every entry names a builder and that no entry is
    doubled, not their order), after `toolbar_button_checked`'s, which keeps the list
    in the builders' order: `("default_button_rest", "button.default_button_background as the fill and
    button.default_button_border_color (button.border.color where unstated) as the
    border colour of a dialog's default Button at rest, over variants::default_button,
    whose rest fill is a fifth of its border colour (button/button.rs, ButtonVariant::bg_color);
    nothing where the theme states no fill, and the Button is .primary()")`. A row in
    the README's geometry table, after `button_checked`'s (`README.md:364`) and the rows
    steps 4 and 5 put after it, `toolbar_button_rest`'s and `toolbar_button_checked`'s:
    directly after `toolbar_button_checked`'s.

  An upstream line the new docs cite names gpui-component 0.10.0, as `bordered_button`'s
  doc does: `variants.rs`'s module doc says its upstream citations are 0.9.1's unless a
  version is named.

### 6.D.3 The contract, `connectors/native-theme-gpui/src/contract.rs`

- `sinks` (`:2736-2889`) gains two entries: after step 3's
  `variants::hover_border` entry and steps 4 and 5's `variants::toolbar_button_states`
  entry, which follow the loop over the value builders, `out.insert("variants::default_button_states".to_owned(), format!("{:?}", crate::variants::default_button_states(n)))`
  (`n` is the `Native` `sinks` builds at `:2779`); and in the list of value builders
  (`:2825-2836`, whose entries are `show(&gm::…(n))`), `("default_button_rest",
  show(&gm::default_button_rest(n)))`, which `sinks` inserts as
  `geometry::default_button_rest` (`:2837`). The fill and the border colour then reach
  both, the label and `button.hover_border_color` the states
  (`every_colour_leaf_reaches_a_sink_or_is_named` splices each leaf in every base,
  `:2956-2987`; where the fill is stated each leaf moves a sink, and on the other
  presets the fill leaf is `null` and splices to a colour, which makes both `Some`).
- The `primary` rows (`:159`, `:165`, `:290`, `:296`, `:505`, `:511`) and the
  pairs reading the primary pair (`:1940-1941`, `:2087-2088`) stay.

### 6.D.4 Tests

In `connectors/native-theme-gpui/src/variants.rs`' test module:

- `default_button_is_none_where_no_fill_is_stated`, a `#[gpui::test]` taking
  `cx: &mut TestAppContext` as `ghost_button_follows_the_installed_theme` does (`:161-162`),
  since `default_button` takes `cx: &App`: for every preset but kde-breeze in both
  modes, `default_button_states`, `geometry::default_button_rest` and
  `default_button` are `None`.
- `default_button_follows_the_installed_theme`: as `ghost_button_follows_the_installed_theme`
  (`:161-183`), a mode switch changes the variant.

In `connectors/native-theme-gpui/src/widgets/tests.rs`, beside step 3's trial test
`bordered_button_matches_the_default_button_but_its_hover_border`, not
in `variants.rs`: the scene reader is a module of the widgets' tests (`#[path =
"scene_read.rs"] mod scene_read;`, `widgets/tests.rs:24-26`), `widgets` is compiled only
with its feature (`src/lib.rs:248-249`), and `scripts/check_release.sh:704-705` runs
the crate's lib tests without it (`cargo_tests -p native-theme-gpui --lib --no-default-features`),
where a `variants.rs` test that used it would not compile:

- `default_button_matches_breezes_default_button`, a `#[gpui::test]` by 3F.2's method
  (the button's own quad of the last frame, `widgets/scene_read.rs:22`, under kde-breeze
  light and dark, built as 6.D.2 says): rest, fill #d6ecf8 / #2d4655 and border #87c0dd
  / #4882a1; hovered and pressed (reached as 3F.2 reaches them, the trial test's
  `hover_and_press` in `widgets/tests.rs`), border step 3's
  `hover_border_color`, #3daee9 in kde-breeze, with the fill #d6ecf8 / #2d4655 hovered
  and the `button_active` token's colour, kde-breeze's `active_background` #c3e5f6 /
  #2f5368, pressed; disabled, equal to a Default button with `geometry::button_disabled`.
  The label by value: the variant's `foreground` equals `default_button_text_color`,
  #232629 / #fcfcfc (`ButtonCustomVariant: PartialEq`), as 3F.2 asserts its text. Every
  item must be equal (3F.3's strictness).

### 6.D.5 Documentation

`README.md`, *Flat buttons*, after step 3's `bordered_button` paragraph and its example
(3F.4), before the `variants::toolbar_button(cx)` paragraph of steps 4 and 5: "`variants::default_button(n, cx)`
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
are compiled as doctests, `src/lib.rs:277-281`). `src/lib.rs:112`, the module table's
`button` row, after step 3's "; the hover border via [`variants::bordered_button`]": ";
a dialog's default button via [`variants::default_button`] and
[`geometry::default_button_rest`]".

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
| kde-breeze, after | #d6ecf8 / #2d4655 under #232629 / #fcfcfc, frame #87c0dd / #4882a1 | the same fill and label, frame `hover_border_color`, #3daee9 (`kde-breeze.toml`, `[light.button]` and `[dark.button]`) | `styles::button`'s fill and label: #c3e5f6 / #2f5368 composited over #fcfcfc / #292c30 (opaque, so itself) under #232629 / #fcfcfc; frame #3daee9 | `styles::button`'s: #f0f0f0 / #272a2e under #a8a9aa / #6d6f72 |
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

In `styles::button_toggle`'s pattern, at each of its places; in every list the new
entries go after `styles::toolbar_button`'s, the last of the button styles:

- `src/contract/rows.rs`: `pub(super) fn native_button_default(r: &ResolvedTheme, status: button::Status) -> (Color, Color)`
  after `TOOLBAR_BUTTON_BORDER_ROWS`, the last of `styles::toolbar_button`'s items: the fill and label of 6.E.2 where the fill
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
  since the border is native in every status) after `native_button_default`; their names
  in `style_row_fields`, after the `TOOLBAR_BUTTON_ROWS` line and the
  `border_row_fields(TOOLBAR_BUTTON_BORDER_ROWS)` line. The
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
- `src/contract.rs`: `button_default_style_fields` after `toolbar_button_style_fields`;
  its call in the walk after `toolbar_button_style_fields`'s;
  the row checks after `check_style_rows(TOOLBAR_BUTTON_ROWS, ..)` and
  `check_border_rows(TOOLBAR_BUTTON_BORDER_ROWS, ..)`; the pair check after
  `check_style_pairs(TOOLBAR_BUTTON_PAIRS, ..)`; the extractor lines after
  `x.style(TOOLBAR_BUTTON_ROWS, t, r)` and `x.border(TOOLBAR_BUTTON_BORDER_ROWS, t, r)`;
  in `every_style_closure_is_clone`, `assert_clone(&styles::button_default(r));` after
  its `assert_clone(&styles::toolbar_button(r, true));` line.
- `src/contract/derived.rs`: `styles::button_default.shadow` and `.snap` after
  `styles::toolbar_button.snap`, "iced default: button::Style::default().shadow,
  through styles::button or styles::button_primary -- the model has a shadow color but
  no offset or blur" and the `snap` twin.
- `src/contract/pairs.rs`: `BUTTON_DEFAULT_PAIRS` ("default button label", the label on
  the fill over `defaults.background_color`, `statuses: CLASS_BUTTON_NATIVE_STATUSES`,
  `native` from `native_button_default`, in the form of `BUTTON_TOGGLE_PAIRS`) after
  `TOOLBAR_BUTTON_PAIRS`; its name in `style_pair_names`, after the
  `TOOLBAR_BUTTON_PAIRS` line. In `Hovered` and `Pressed`
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
    plain button's (`plain(theme, Disabled)`) differ: only where `disabled_background` is
    unset, adwaita in both modes (every other preset states it, today's
    `[leaves."button.primary_background"]` moving `styles::button_primary.background`
    (`Disabled`) in adwaita's two combinations alone, and where it is stated both pairs
    are `disabled_background` under `disabled_text_color`), so `.background` there in
    both modes and `.text_color` under adwaita/light only, adwaita/dark's
    `disabled_text_color` (`defaults.disabled_text_color`) and `primary_text_color`
    (`defaults.accent_text_color`) being both #ffffff (`adwaita.toml`, `[dark.defaults]`);
  - `button.default_button_text_color`: `.text_color` (`Active`), only under kde-breeze;
  - `button.default_button_border_color`: `.border` (`Active`), only under kde-breeze;
  - `button.hover_border_color`: `.border` (`Hovered`, `Pressed`): under kde-breeze the
    hover frame, elsewhere `button_primary`'s hovered and pressed border,
    `hover_border_color`, else `border.color` (`class_button`; `native_button_border`),
    so in all 34, as today's `[leaves."button.hover_border_color"]` moves
    `styles::button_primary.border` (`Hovered`, `Pressed`) with no `except`;
  - `button.border.color`: `.border`: under kde-breeze in `Disabled` only (the plain
    button's border, faded; `Active` is the stated frame, `Hovered` and `Pressed` the
    stated hover frame), elsewhere in every status `button_primary`'s border takes it;
  - `button.disabled_background`: `.background` (`Disabled`) everywhere, and
    `.text_color` (`Disabled`) under adwaita/light only, as today's
    `styles::button_primary.text_color` entry of `[leaves."button.disabled_background"]`:
    the class label tests it (`native_class_button_label`, `rows.rs:509-528`), but where
    `disabled_background` is stated the splice changes the fill alone, adwaita/dark's two
    labels are the same colour, and under kde-breeze the plain label does not read it;
  - `button.disabled_text_color`: `.text_color` (`Disabled`): under kde-breeze (the
    plain label, `native_button_label`), elsewhere where `disabled_background` is
    stated, so `except` adwaita's two combinations (today's
    `styles::button_primary.text_color` entry of `[leaves."button.disabled_text_color"]`);
  - `button.primary_background`: `.background` (`Active`, and `Disabled` where no
    `disabled_background` is stated, adwaita's two combinations), `except` kde-breeze;
  - `button.primary_text_color`: `.text_color` (`Active`, and `Disabled` where no
    `disabled_background` is stated, adwaita's two combinations), `except` kde-breeze.
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

`README.md:189`, the styles table's "with a status" row: `button_default` follows
`toolbar_button` (itself after `button_toggle`), and the last cell gains, after the
`toolbar_button(&resolved, checked)` gloss, "`button_default` is a dialog's default
button, the primary one where the platform gives it no look of its own;", in the cell's own
form. The crate
doc's coverage table, `src/lib.rs:318`, the `styles` row: "`styles` (34 functions) |
every `Style` field of button (six classes), toggle button, toolbar button, text input, …"
counts the top-level `pub fn`s of `src/styles.rs` but `composite_over` (35 `^pub fn`
lines, so 34), and names the button styles (the six classes `button`, `button_primary`,
`button_danger`, `button_success`, `button_warning`, `button_link`, then
`button_toggle` and `toolbar_button`). So this step's edit, in place: the count becomes 35,
re-read on the tree (the top-level `^pub fn`s but `composite_over`; the caret's units add
none to `src/styles.rs`), and "a dialog's default button" goes after "toolbar button" (the
README row's list likewise: `button_default` after `toolbar_button`). `ROADMAP.md:162`'s
"has 34 style functions" moves to the same count, in 6.H's documents unit.

## 6.F egui connector

### 6.F.1 What draws a default button in egui today

egui has no default button. A selected button in the «button» role fills
`visuals.selection.bg_fill`, lettered `visuals.selection.stroke.color`
(`EG/theme/default_style.rs:47-52`, `StyleProvider<ButtonStyle> for DefaultStyle` under
`Button::CLASS_SELECTED`), which the role sets to the primary pair
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
`egui/src/widgets/button.rs:153-167` at egui 0.37.0), so one call draws one look in every state, and the
states are the application's, from the button's previous response: at rest `fill`,
`text` and `stroke`; while it was hovered, the same with `hover_stroke`; while it had
the keyboard focus and was not pressed, the same with `focus_stroke`, since egui gives a
focused button its `active` look (`egui/src/widget_style/mod.rs:212-222` at egui 0.37.0),
which `Button::stroke` overrides as well; while it was pressed, `hover_stroke` with neither `fill` nor `text`,
so that egui draws the plain button's pressed fill and label, which the `Role::Button`
scope gives it, as Breeze draws a pressed default button (docs/platform-facts.md
§2.3)." At egui 0.37.0 a `Button` takes its look from `Ui::widget_style`
(`EG/theme/mod.rs:26-47`), whose state is the previous pass's `Response::widget_state`,
`Active` where `self.has_focus()` (`EG/widget_style/mod.rs:215`), and the default
`ButtonStyle` takes `visuals.widgets.state(state)` (`EG/theme/default_style.rs:44`);
`Widgets::style` (`EG/style.rs:1383-1395`) makes the same test (`:1386`), but `Button`
does not call it.
**The focus stroke** is the egui side of step 3's `focus_border_color`: the «button»
role writes it into the `active` stroke (`connectors/native-theme-egui/src/style/roles.rs`,
`fn button`: `focus_border_color`, else `hover_border_color`, else `border.color`), which a per-call `Button::stroke`
replaces in every state (`EG/widgets/button.rs:161`, "this will override any on-hover
effects"); without the field a focused default button would show its rest frame. The
change stays inside this step: one field, its line in `default_button`, the showcase's
memory keeping `has_focus()` beside the other two (6.G), the test (6.F.4) and the
docs. It adds no mapping row and no sink: `default_button` is a per-call accessor, so
T2's splice of `button.focus_border_color` moves no further `Style` field and its row
keeps its two sinks, `visuals.widgets.active.bg_stroke.color` in the base style and in
the «button» scope (`mapping.toml`, `["button.focus_border_color"]`), and T10, which compares sinks with their rows'
formulas, has no new sink to compare; the three `default_button_*` rows stay `tested_by
= "T18(a)"`, as `hover_border_color`, which the accessor reads already, keeps its row. The
doc names egui's version with the lines (a pinned source; decision 6). The egui showcase draws these states (6.G), and its Widget Info
says so. `src/lib.rs:199-203`, the `pub use accessors::{..}` list, gains `default_button`
and `DefaultButton` (in its alphabetical place).

### 6.F.3 Mapping, `mapping.toml`

After `["button.checked_text_color"]` (`:996-998`), three rows of the per-call form
(`verdict = "derived"`, `tested_by = "T18(a)"`, as `button.checked_background`'s,
`:992-994`): `["button.default_button_background"]`, `["button.default_button_text_color"]`,
`["button.default_button_border_color"]`. No `probe`: the five platform presets'
pairs differentiate each leaf (kde-breeze states it, the others leave it `null`).

`src/mapping_tests.rs`: `every_leaf_has_exactly_one_row_and_every_row_is_well_formed`
(`:236-241`, today 525) grows by three, and its message, "521 ResolvedTheme leaves + 4
layout leaves (§5.7)" (`:240`), with it (the `ResolvedTheme` count by three);
`verdict_totals_match_the_specification` (`:473-480`): the `buttons` group's derived
count grows by three (today `("buttons", [13, 47, 31, 25])`, so `[13, 47, 34, 25]`).
The caret's units, built before this step, change the leaf count first (three `input`
leaves: 528, "524 ResolvedTheme leaves + 4 layout leaves (§5.7)") and leave the
`buttons` group as it is, so after this step 531, "527 ResolvedTheme leaves + 4 layout
leaves (§5.7)"; the implementer adds this step's deltas to the numbers the test holds
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
both #3daee9, `kde-breeze.toml`, `[light.button]`)
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
in modules the script lists without comparing". It goes after `role_modifier`'s entry,
the last of the section *egui: frames, focus rings, popups*.

`README.md:262`, the accessor list ("`window_title_bar_text_color`, `dialog_button_order`,
`list_header_height`…"): `default_button` with the doc's first sentence and its rule for
the four states (at rest `fill`, `text`, `stroke`; hovered `hover_stroke`; focused
`focus_stroke`; pressed `hover_stroke` and no fill, decision 6).

## 6.G Showcases

**A shared element (decision 7).** `docs/showcase-elements.toml`, after the element
`basic.buttons.toggle_on.label` (`:849-856`), under a comment of their own, the
Buttons group's last elements:

```toml
# After the toggles, in the third row: a dialog's default button, the one Enter activates.

[[element]]
id = "basic.buttons.dialog_default"
name = "Button"
parent = "basic.buttons.toggle_on"
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

The parent is the control before it in its row, `basic.buttons.toggle_on`, as that
one's is `basic.buttons.toggle_off` and, in the icon row, `basic.icons.edit`'s is
`basic.icons.delete`, by the list's rule "a control against its heading or the control
before it" (`:672-674`). The next group's heading, `basic.checkboxes.heading` (`:860-866`),
keeps `parent = "basic.buttons.toggle_off"`, the first element of the row above it
whether the button wraps or not. Measured against the button it would bring a parity
difference of its own where the button stays on the toggles' line: in T-K2's material-3
dumps the heading stands 145, 145 and 146 px left of the button in the three
showcases, and at the same x as `toggle_off` in all three. The leaves are every leaf the button draws in any
of its states, as `basic.buttons.default` lists its hovered and pressed leaves under
the state `Normal` (`:734-740`): the primary pair where the fill is unstated, the three
fields and their fallbacks `button.font.color` and `button.border.color` where it is
stated, the plain button's pressed pair (decision 3), and step 3's two border leaves,
which every Basic button that wears `button.border` and can be hovered lists
immediately before `"button.border.color"` (step 3's 3B.4; `basic.buttons.default`'s
`leaves`) (the hover one is drawn here by decision
20). The control's leaves are the union of its label's (the file's header, `:36`). They
are in the file's order (`:35-36`: the element's own table in registry order, then the
others): `button` in `docs/property-registry.toml`'s `[button]` order, where
`hover_border_color` and `focus_border_color` stand after `checked_text_color` (step
3's 3B.2) and 6.B.1 puts these three after them, then
`defaults`. No leaf is listed twice, and `states` is non-empty: `check_showcase_parity.py
--check-list` accepts any non-empty state name (`:483-490`, read today), "Default"
included. The sample keys are `basic.buttons.primary`'s (`:751-757`).

**Its place, the same in all three (decision 7).** The button follows the two toggles
in the Buttons group's third row. That row wraps in all three showcases today, as their
icon rows do: gpui's is the page's `row()` (`showcase-gpui/pages/basic.rs`, the toggle
row), iced's `row![…].spacing(gap.widget).wrap()` (`showcase-iced.rs`, `view_basic`),
egui's a `ui.horizontal_wrapped` (`showcase-egui/pages/basic.rs`, `buttons`). So no
layout code changes: where the column is wide enough the button stands on the toggles'
line and the group keeps its height; where it is not, it takes a line of its own below
them, at the row's own gap. The trial T-K2 (2026-11-19, `<record>/trial/t-k2/`)
measured where: a line of its own under kde-breeze light and dark, adwaita and
catppuccin-mocha (five columns), the toggles' line under material-3 (four columns),
the three showcases alike. `t-k2.patch` shows the three places with a stand-in button:
it gives the place only. The button, its element, its kinds and its helper are this
part's, and the patch's own hunk of `docs/showcase-elements.toml` is not applied.

**iced** (`showcase-iced.rs`): `Push` (`:10006-10013`) gains `DialogDefault` after `On`,
and the `push` closure's match (`:10393-10412`, in the closure `:10388-10416`) an arm in its neighbours' form styled
`styles::button_default(resolved)`. The toggle row (`:10477-10495`, a `row![…]` that ends `.spacing(gap.widget).wrap()`)
gains a third item after the `Push::On` button: `push("basic.buttons.dialog_default",
"basic.buttons.dialog_default.label", "OK", Push::DialogDefault,
Some(Message::ButtonPressed))`, and the row's comment (`:10477`, "The toggle buttons, off
and on (Basic v4's third row)") names it too. The route functions: `drawing()` (`:7341`) gains `_ if under("basic.buttons.dialog_default")
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
(`:7580`); `iced_route`'s button class (`let class = if tool {`, its arms
`styles::toolbar_button`, `ghost_button`, `styles::button_toggle`,
`styles::button_primary`, `:7792-7801`) gains `else if
under("basic.buttons.dialog_default") { "styles::button_default" }` before the
`styles::button` fallback; the cases of the test `widget_info_names_the_hovered_element`
(`:29453-29454`), which hovers each element and asserts its Widget Info title, gain
`("basic.buttons.dialog_default", "Button · Default")`. The title itself is not
written by the showcase: it is the element's `name` and first state, "Button" and
"Default" from the element list above (`ShowcaseElement::title`,
`tools/showcase-support/src/elements.rs`). Every other place of the showcase
that lists `styles::button_primary` among the style functions, found today with
`grep -n '"styles::button_primary"'`, gains `styles::button_default` where its subject
includes it: `:20797`, in `DRESSED`'s `button` row (`:20792-20810`), the closed list of
the styles a `button(` call site may carry, which `styles_cover_every_widget_shown`
holds every site to (`:20949-20965`), so the new button is red without it: it gains
`"styles::button_default"` after `"styles::button_toggle"` (before
`"styles::toolbar_button"`, which follows it today); `:21009`
(`themed_buttons_take_the_themes_padding`, `:21005`), the classes whose buttons must
take the theme's padding: it gains `"styles::button_default"`, which the new button
takes through `button_padding(` as its neighbours do; `:7797`, `iced_route`'s button
class, above. No edit at `:27907-27911` (`DRAWN_WITH`, the drawn-with table of
`drawn_with_is_what_the_pages_draw_with`) or `:28217` (`row_ink`'s arm): both serve the
infos of page widgets wrapped in `hoverable` (`DRAWN_WITH` is read by the `hoverable(`
call sites, `:28103`; `row_ink` by `colour_row_faults` over `built_infos`, "The page
widgets the pages of `state` wrap in [`hoverable`]", `:28859-28867`), and the Basic
page's buttons are built by the `push` closure, which wraps none in `hoverable`
(`:10388-10416`); `:12207-12238` are the Buttons page's infos of its primary row
(`button_info`, two sites) and of its disabled row (`disabled_buttons_info`, one), not
changed. Each is read before it is edited. The route test
`the_element_list_parses_and_every_leaf_has_a_route` (`:24982`) holds the new element's
leaves to a route: the `drawing()` arm above makes `drawing(..)` `Some` for them, so
their routes are `colour_route`'s from the contract export (`:7579-7580`), naming
`styles::button_default`, which reads them. `every_hand_colour_route_names_a_function_that_reads_it`
(`:29102`) skips every leaf whose `drawing(..)` is `Some` (`:29126-29129`), so it does
not read them.

**egui** (`showcase-egui/pages/basic.rs:519-612`, `buttons`): in the toggle row's
`ui.horizontal_wrapped` closure (`:590-596`), after the On button, one "OK" button
through the page's `text_button` helper, as its neighbours (`:531`, `:534`, `:592`), with the kind
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
`:531`, `:534`; `text_button` applies it to the `Button` its `make` returns, so the
closure does not); `the_palette_pages_apply_the_per_call_leaves`
(`showcase-egui/tests.rs:3652`) holds the `Role::Button` kinds of the Buttons and
Overlays pages to `button.min_width` (`:3641-3650`, the pages it opens), not the Basic
page's. The doc of `buttons` (`basic.rs:515-518`): "Buttons (the toggle button their
third row), check boxes, …" becomes "Buttons (the toggle buttons and a dialog's
default button their third row), check boxes, …", and "The Buttons group: three
rows of two buttons, each row wrapping …" becomes "The Buttons group: two rows of two
buttons and a third of three, each row wrapping …". The test
`the_basic_page_has_every_group_in_its_column` (`showcase-egui/tests.rs:5554`) counts
each group's kinds from `BASIC_CONTROLS` (`:5477-5546`): the `"Buttons"` entry
(`:5478-5488`) gains `("button (dialog default)", 1)` after `("toggle button (on)", 1)`.

**gpui** (`showcase-gpui/`): `elements.rs`'s `LISTED` gains `("basic-button-dialog-default", "basic.buttons.dialog_default")`
after `("basic-toggle-on", "basic.buttons.toggle_on")` (`:87`), and `LABELS`
(`:269-276`) `("basic-button-dialog-default", "basic.buttons.dialog_default.label")`
after `:275`. The test `the_basic_page_holds_every_group_in_its_column`
(`examples/showcase-gpui/tests.rs`), which lists every Basic control id, gains
`"basic-button-dialog-default"` after `"basic-toggle-on"`. The button is built by a new helper, `demo::dialog_default_button(ui, cx,
id, label)`, in the form of `demo::toggle_button` (`demo.rs:9157-9202`), after it, and
is the third child of the toggle row's `row()` (`pages/basic.rs:450-453`), after
`demo::toggle_button(ui, cx, "basic-toggle-on", "On")`:
`.child(demo::dialog_default_button(ui, cx, "basic-button-dialog-default", "OK"))`.
`BUTTONS` stays `[BasicButton; 2]` and `ButtonKind` gains no variant: a `BUTTONS` entry
is a button of the first row built by `demo::button` (`demo.rs:3330-…`, through
`button_turning`), which applies
the variant (`ButtonKind::apply`, `:3365`) before `geometry::button` and no refinement
after it but the disabled pair, where `geometry::default_button_rest` must come after
`geometry::button` (6.D.2); and a `ButtonKind` variant would take an arm in every
`match kind` of `info/buttons.rs` (`:18`, `:163`, `:403`, `:450`, `:513`, `:534`,
`:560`, and the shadow note's, which is a `match kind` today), which hold the arms of step 3's
`ButtonKind::Bordered` (3F.4, with `ButtonKind::built(self, cx)`): in `at_rest`,
`under_the_pointer`, `edge`, the idle edge note, the shadow note and the variant line.
The helper follows `toggle_button`'s form (`demo.rs:9157-9202`):
`Button::new(id)` refined through `native_info`; this button has no `ButtonKind` of its
own. Where `variants::default_button(n, cx)` is
`Some(v)`, the helper builds `Button::new(id).custom(v)`, refined by
`geometry::button` through `native_info` (`info/mod.rs:419-425`, as `toggle_button`
does, `demo.rs:9186-9192`) and then by `rest`, `geometry::default_button_rest(n)`,
through `refined(button, rest.as_ref())` (`support.rs:273`; `native_info` takes a
builder that returns a `StyleRefinement`, not an `Option`), its line recorded with
`button_info.geometry("default_button_rest")` as `demo::button` records
`button_disabled` (in `button_turning`, `demo.rs:3377-3379`), as the page's default,
tooltip and toggle-off buttons are built through `bordered_button` (`ButtonKind::Bordered`
in `pages/basic.rs`'s `BUTTONS`, `TOGGLE_OFF` and the tooltip button's call); where either is `None`,
`Button::new(id).primary()` refined by `geometry::button` alone. Its info is registered
as `toggle_button` registers its own (`InfoExt::info(labelled(ui, cx, button, id,
label), ui, id, button_info)` and `.debug_selector(..)`, `:9200-9201`): for the
`.primary()` button `info::buttons::button(cx.theme(), ButtonKind::Primary,
ButtonState::Idle, false, false, None, native_button)` (its arguments `t`, `kind`,
`state`, `icon`, `default_spinner`, `size`, `native`), the primary button's info; for the
variant the info of *gpui's colour claims* below. This use is what
`the_showcase_exercises_every_builder` (`CG/showcase.rs:384-416`) requires of every
`pub fn` of `variants.rs` and `geometry.rs`, so it lands with them (6.I.1, K6b).

**gpui's routes.** `every_listed_leaf_has_a_route`
(`showcase-gpui/tests.rs:15196-15209`) needs a `HOW` line (`info/leaves.rs:15`, an
`(element-id prefix, leaf, line)` triple; the first whose prefix the element's id starts
with wins, `:2368-2373`) for every leaf of every listed element, and the lookup is by
prefix (`element.starts_with(prefix)`), so `basic.buttons.default`'s lines do not cover
`basic.buttons.dialog_default`. The new element's other leaves have generic lines
already (`""`-prefixed `button.font`, `.font.color`, `.min_width_px`, `.min_height_px`,
`.primary_background`, `.primary_text_color`, `.active_background`,
`.active_text_color`, `.border.*`; `defaults.line_height` the `"basic.buttons"`-prefixed
one, "gpui: geometry::button's line height (control-height rule)"; `:281-386` among them, read
at `0fa28027`), and `button.focus_border_color` takes its generic line (`""`, "not
reachable: with gpui-component's band off, a focused Button's border is ThemeColor ring
where it has a border width, …", step 3's 3F.4),
which holds for this button whether it is the variant or `.primary()`: gpui-component calls `focus_style` on every
focused Button whatever its variant (`GC/button/button.rs:936-938`, with the line
chosen at `:653-658`), and `focus_style` turns the border to `ring` with the band on
(`GC/styled.rs:257-263`) and, with it off, where the element has a border width
(`:269-271`), and otherwise the line its `FocusLine` gives (`:273-297`); a custom variant and
`.primary()` alike go through it, so neither leaves the line's text. `button.hover_border_color` has no generic line (its lines are per
element: `basic.buttons.toggle_on`, `.default`, `.tooltip`, `.toggle_off`, `.primary`), so it gets one here with the three new leaves. Two generic
lines do not hold for this button: `button.font.color`'s ("gpui: ThemeColor
button_foreground (colors.rs): the label's colour", the plain Button's), for the variant
and the `.primary()` Button alike,
since the variant takes the colour through `default_button_states` and a `.primary()`
Button letters in `button_primary_foreground`; and `button.active_background`'s ("gpui:
ThemeColor button_active, …; the pressed fill"), since a `.primary()` Button is pressed
in `button_primary_active`, on every preset but kde-breeze (there the variant's
`.active(cx.theme().button_active)` is the pressed fill, which the line names). So
these get lines here too. Each has the
prefix `"basic.buttons.dialog_default"` (which covers the label too), in the button
section after `basic.buttons.toggle_on`'s entries (`:16-36` at `0fa28027`; their last
is its `button.hover_border_color` entry, step 3's 3F.4):

- `button.default_button_background`: "gpui: geometry::default_button_rest's bg at rest, after geometry::button, and variants::default_button's hover fill; where unstated the Button is .primary(), ThemeColor primary"
- `button.default_button_text_color`: "gpui: variants::default_button's foreground, in every state; where unstated the Primary Button's button_primary_foreground"
- `button.default_button_border_color`: "gpui: geometry::default_button_rest's border colour at rest, after geometry::button; under the pointer and pressed variants::default_button's color, button.hover_border_color where stated"
- `button.hover_border_color`: "gpui: variants::default_button's color, the border under the pointer and pressed, where the theme states button.default_button_background; elsewhere the Button is .primary(), whose hover and press border is the primary token (gpui-component 0.10.0 button/button.rs:1104, primary) and whose rest border is geometry::button's button.border.color (native-theme-gpui/geometry.rs:230, border_color)"
- `button.font.color`: "gpui: variants::default_button_states's label where the theme states button.default_button_background and no button.default_button_text_color, the variant's foreground in every state; elsewhere the Button is .primary(), lettered in ThemeColor button_primary_foreground (gpui-component 0.10.0 button/button.rs:1055, button_primary_foreground)"
- `button.active_background`: "gpui: variants::default_button's active, ThemeColor button_active (colors.rs), the pressed fill, where the theme states button.default_button_background; elsewhere the Button is .primary(), pressed in ThemeColor button_primary_active (gpui-component 0.10.0 button/button.rs:1271, button_primary_active), which the connector sets to primary_active, a shade of primary (colors.rs)"

A `.primary()` Button's border is the primary token under the pointer and pressed only:
its press and hover styles set it (`GC/button/button.rs:741-746`, `:754-765`, from
`ButtonVariant::border_color`, `:1104`), while at rest the caller's `geometry::button`
refinement, `button.border.color` (`CG/geometry.rs:230`), is replayed over the Button's
own style (`GC/button/button.rs:767`, `.refine_style(&instance_style)`).

Each range carries the symbol its lines hold, as `every_prose_citation_still_exists`
(`CG/showcase.rs:3762-3769`) requires of a `HOW` line: `line_citations` (`:3273-3387`)
reads `file.rs:N, Symbol` and each further `; :A-B, Symbol` group as ranges of the same
file, and `line_citation_finding` (`:3424-…`) passes a range only where its lines name
the symbol as an identifier (`mentions`, `:1028-1049`). Read in gpui-component
0.10.0: `:1104` is `Self::Primary => cx.theme().primary,`, which names `primary`;
`:1055` names `button_primary_foreground` and `:1271` `button_primary_active`.

**gpui's colour claims.** `info/buttons.rs`, the info of a
button built through `variants::default_button`: a function of its own beside `button`
(`:488`), `dialog_default`, which the helper above calls, since `edge` (`:339-…`)
branches on `ButtonKind` and the dialog default button has no kind of its own (6.G's
gpui paragraph). Its claims are in the form of `bordered_button`'s, which
`at_rest`, `under_the_pointer` and a branch of `edge` hold (step 3's 3F.4): "bg" claims
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
(6.I.1, K6b).

**egui's routes.** The egui showcase's Widget Info takes each leaf's route from its
`mapping.toml` row (`showcase-egui/info.rs:390-…`, a leaf's key; `checked_by`, `:73-97`,
for a `tested_by` row), which 6.F.3 adds; no egui showcase test was found that
requires a route per element leaf beyond those rows (searched: `grep -n '^fn every_\|leaf'`
over `showcase-egui/tests.rs`, `info.rs`).

**The page with steps 4 and 5.** The icon row holds their checked tool button
`basic.icons.edit` (their B.10), its elements after `basic.icons.delete.icon` in the
list; this step's element
follows the toggles in the Buttons group's third row and goes into the element list
after `basic.buttons.toggle_on.label`. The places that name the Basic buttons (iced's
`widget_info_names_the_hovered_element` cases, `:29453-29454`; `drawing()`, `fn` at `:7341`, its `basic.buttons.primary` arm at
`:7357`) gain the element as above; steps 4 and 5's tests of the icon row are not
touched.

**The layout, measured (decision 7).** The trial T-K2 took the Basic page's layout
dumps of the three release showcases before and after the button, headless (0.7.2's
`rehearsal-t1.sh`, nine entries on the private X server), with a stand-in button built
as `basic.buttons.default` is, and compared them with `compare-tk2.py`, 0.7.2's
comparison with this element's ids (`<record>/trial/t-k2/RESULTS.md`, `compare.txt`,
`compare-xplat.txt`):

| Entry | Columns, column 1 | The button | The group gains | Moved |
|---|---|---|---|---|
| kde-breeze light and dark | 5, 177 px | a line of its own | 38 px | 23 to 26 elements, the rest of column 1 |
| adwaita light | 5, 177 px | a line of its own | 40 px | the same |
| catppuccin-mocha light | 5, 177 px | a line of its own | 38 px | the same |
| material-3 light | 4, 222 to 223 px | on the toggles' line, 9 to 12 px to spare | nothing | nothing |

No element that lay inside its visible area lies outside it afterwards, under any
entry; where the button wraps, column 1 still ends 86 to 176 px above the end of the
page's visible area; the three showcases agree on every entry. The four presets of
other platforms cannot be shown on Linux and repeat kde-breeze there; a second pass
with 0.7.2's patch that lets the Linux build run them (`t1-xplat-cli.patch`, fallback
fonts) kept the button on the toggles' line under windows-11 (18 px to spare),
macos-tahoe (25) and ios-13 (12 to 92), with nothing moved, and found one difference:
under macos-monterey egui lays the page out in five columns of 170 px and iced and gpui
in four of 218 px, as they did before the button (0.7.2's trial T1 met the same split),
so egui wraps the button there and the other two do not. That pass is not the
platforms' own layout: the macOS and Windows captures of the asset run are, and the
session looks at them there (7.J.4).

**The check on the built tree.** After K6b to K6d and before the step's gate, the three
release showcases are built on the tree that holds them (the commands of
`<record>/trial/t-k2/build-after.log`, through `slot.sh`), and the same nine entries
are dumped headless (`flock ~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/slot-locks/xvfb97.lock
<record>/trial/t-k2/rehearsal-t1.sh <record>/steps67/dumps-6`) and compared with T-K2's
"after" dumps: `python3 -I <record>/trial/t-k2/compare-built.py <record>/trial/t-k2/after
<record>/steps67/dumps-6`, `compare-tk2.py` with both sides read against the list that
holds the button (tried on T-K2's "after" dumps against themselves: exit 0, nothing
moved). Expected: exit 0, with no stop entry (an element that lay inside its visible
area and no longer does), no parity entry (the showcases disagreeing on the button's
line) and no moved line: the real button has the stand-in's geometry,
`geometry::button`'s. A stop entry or an input fault (exit 2) stops the work; a parity
entry or a moved line is fixed, or explained in the record, before the gate. Then the
parity rehearsal runs on the same binaries (0.7.2's `<tools>/rehearsal.sh`, read by
`scripts/check_showcase_parity.py`). Step 7 runs both again after its own block
(7.J.1), into `<record>/steps67/dumps-7`, against `dumps-6`.

- **The parity keys the button brings.** The asset run's parity step compares the new
  element and its label like every other button, so the differences the page's other
  buttons already have are expected for them too: the label's height (`.label.h`,
  under adwaita and catppuccin-mocha, as every Buttons-row label has it in
  `docs/showcase-exceptions.toml`'s sub-pixel line-box group) and the positions and
  sizes the other buttons have keys for (`.x`, `.y`, `.w`, in the file's existing
  groups for them), and any position key of an element the button's line moves down.
  They are not written by hand: the rehearsal's or the asset run's `--merge`
  (`check_showcase_parity.py --merge OUT --write --proposal FILE… [RUN=]DUMP_DIR…`)
  proposes them into the groups whose reasons hold for them, and the proposal is
  reviewed and the maintainer told.

**Widget Info.** The info names the functions each showcase calls and follows the rule
of 6.A.1 through them; no showcase writes a prose of its own for the element:

- **iced:** the title "Button · Default" (the element's `name` and state, through
  `ShowcaseElement::title`; the test case above), and per leaf the
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
  `geometry::default_button_rest` where the theme states the fill, the primary
  button's info where it does not (the helper above).

**Screenshots.** The Basic page's Buttons group gains the button after the toggles: on
a line of its own, with the groups below it in column 1 moved down by that line, under
kde-breeze, adwaita and catppuccin-mocha; on the toggles' line, with nothing moved,
under material-3 (the table above); under kde-breeze the button is Breeze's default
button (iced, egui and gpui), elsewhere the primary one.

## 6.H Outside the code (D10)

### 6.H.1 `CHANGELOG.md`, 0.7.2's section (D0 (a))

Step 6's entries join 0.7.2's section, `## [0.7.2] - <date>`, each at the end of its
subsection, after the caret's (the caret document's E.1); step 7's follow (7.I.1).
`### Breaking Changes` keeps one bullet for the release, the break named once: step
6's documents unit rewrites it to name every struct the release gives a field, with
step 7's clause in it already, since the section describes the release and the
release is not cut before step 7 has landed (7.J.4):

```markdown
- **native-theme**: `ThemeDefaults` gains `focus_ring_keyboard_only` and `selection_inactive_text_color`, `ButtonTheme` gains `hover_border_color`, `focus_border_color`, `default_button_background`, `default_button_text_color` and `default_button_border_color`, `CheckboxTheme` and `ComboBoxTheme` gain `hover_border_color`, `ToolbarTheme` gains `button_hover_background`, `button_hover_border_color`, `button_active_background` and `button_checked_background`, `InputTheme` gains `caret_blink_on`, `caret_blink_off` and `caret_blink_timeout`, and `ListTheme` gains `header_hover_background` and `header_active_background` (see *Added*). None of them is `#[non_exhaustive]`, so a struct literal of any of them outside the crate needs the new fields or `..Default::default()`; their resolved forms are `#[non_exhaustive]` already.
```

`### Added`, step 6's entries (step 7's follow them, 7.I.1):

```markdown
- **native-theme**: a dialog's default button, the one Enter activates, has fields of its own: `button.default_button_background`, `default_button_text_color` and `default_button_border_color`, each optional. Where a theme states the fill, the default button is that fill, lettered in the text colour (else the button's) and framed in the border colour (else the button's); where it states none, the default button is the primary one, as on macOS and Windows. kde-breeze and the KDE reader state Breeze's, `mix(Button, Highlight, 0.2)` framed in `mix(Highlight, mix(Button, ButtonText, frameContrast), 0.5)` under the button text, #d6ecf8 / #2d4655 framed #87c0dd / #4882a1; `button.primary_background` and `primary_text_color` keep the accent pair on every platform.
- **native-theme-iced**: `styles::button_default(&resolved)`, a dialog's default button: Breeze's under kde-breeze, `styles::button_primary` where the theme gives the default button no look of its own.
- **native-theme-egui**: `default_button(&resolved)`, the fill, label colour and stroke to hand a `Button` where the theme gives a dialog's default button a look of its own, and the strokes for a button whose previous response was hovered or pressed (`hover_stroke`) or had the keyboard focus (`focus_stroke`); `None` where it is the primary button (a selected button in `Role::Button`). A pressed default button is the plain pressed button, which the `Role::Button` scope draws for a button given no fill.
- **native-theme-gpui**: `variants::default_button(n, cx)`, a custom variant for a dialog's default button that the application builds, Breeze's under kde-breeze, and `geometry::default_button_rest(n)`, the refinement that gives it its rest fill and frame after `geometry::button` (the README shows how); both `None` where the theme gives the default button no look of its own, and the button is the primary one. It goes into a plain `Dialog`'s own footer; an `AlertDialog`'s default footer builds its OK button from a `ButtonVariant` alone and keeps the primary button.
- **Showcases**: the Basic page shows a dialog's default button after the two toggle buttons, on their line where the column has room and on a line of its own below them where it has none.
```

### 6.H.2 `ROADMAP.md`

In 0.7.2's section (the caret document's E.3 rewrites its heading and appends the
caret's paragraph), step 6's documents unit adds, after the caret's paragraph, a paragraph
that opens with step 6's sentences: "A dialog's default button has fields of its own:
Breeze's tinted default button under kde-breeze, the primary button where the platform
draws that, as macOS and Windows do. The iced and egui connectors offer it, and the
gpui connector a variant for a default button the application builds." Step 7's
documents unit appends its sentence to the same paragraph (7.I.2). The 0.7.1 section's
sentence "The platform facts for the focus ring and the default button are corrected
too, and their values wait for a connector design" stays: that section describes
v0.7.1, as it still does for the focus ring, which 0.7.2 delivered.

The count of the iced connector's style functions ("has 34 style functions" on 0.7.2's
tree) moves to the count 6.E.4 writes, in place, found by its text, and the same
paragraph's list of the functions by name gains "`styles::button_default` a dialog's
default button" after `styles::toolbar_button`'s entry, in the list's own form.

The count of open `[v0.7.x]` items in *Also in the 0.7.x releases* does not change at
step 6: 6.H.5 opens one `[1.0]` entry and closes none (its item 1's `[v0.7.x]` entry
stays open).

### 6.H.2a The READMEs' count of optional colours

The two READMEs count the colours the resolved theme holds as `Option`s, 42 on 0.7.2's
tree, in five places found by their quoted text: `README.md`, "42 colours" and "five of
the 42"; `native-theme/README.md`, "every colour but 42 optional ones", "most of the
42" and "five of the 42". Step 6's three fields are optional colours, so step 6's
documents unit writes 45 in the five places; step 7 adds its two header fills and
writes 47 (7.I.2; `defaults.selection_inactive_text_color` resolves on every theme and
is not counted). "five" stays: it counts the switch's outline colours. No gate reads
the counts.

### 6.H.3 The first rationale, `docs/issues/0004_kde-hand-overrides-rationale.md`

- **§5.2** (`:874`), a line after the heading: "*Settled by `0004_kde-open-items-rationale.md`
  D8 (0.7.2): the default button has fields of its own, which kde-breeze and the KDE
  reader state; the primary pair keeps the accent.*"
- **§9** (`:1164`), the row "Default button, fill / border / label": its third cell
  "the same (§5.2)" stays true; no edit.
- The default button is not one of the reporter's items (§7 has no row for it), and the
  reply (§8) does not mention it: no edit there.

### 6.H.4 The rationale's status line

The rationale's status paragraph gains one sentence for both steps, written at the
release's close-out with the archive's path: 7.I.4's, which names step 6 (D8) beside
step 7.

### 6.H.5 `docs/todo.md`

1. **"KDE values that wait for a connector design"** (`:1826-1913`): item (2)
   (`:1856-1868`, "(2) **The default button** …") becomes "(2) **The default button**:
   done in 0.7.2 (below)." After the entry's last line, a dated line in the file's
   convention (`*<what> (<date>, <sha>):*`, header `:1-21`): "*Issue #4, open items step
   6 (0.7.2, <date>, <sha>):* (2) done (`docs/issues/0004_kde-open-items-rationale.md`
   D8): `button.default_button_background`, `_text_color`, `_border_color`, stated by
   kde-breeze (`kde-breeze.toml` `[light.button] default_button_background`,
   `[dark.button] default_button_background`) and the KDE reader; iced
   `styles::button_default`, egui `default_button`, gpui `variants::default_button`."
   The entry stays while item (5) is open.
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
   entry that stays open), a line after its last: "*Issue #4, open items step 6 (0.7.2,
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

### 6.I.1 The units

Step 6's units follow the caret's on the release's branch (`v0.7.2-rc1`, the head) and
start from its tree. The fields cannot pass a workspace gate before every connector
and showcase names them (below), so the step is built as a core unit, a block of
three connector units and a documents unit under one gate.

| Unit | Content | Gate after it |
|---|---|---|
| K6a — the core | 6.A (the fields, the struct literals, the `presets.rs` assertion); 6.B (registry, rules, the facts rows, placed by Part F); 6.C entire (kde-breeze's values, the KDE reader, its tests and the moved-leaves tests' new lists) | native-theme's tests with the gate's features and the five documentation checkers; the connectors are red until K6b to K6d |
| K6b — gpui | 6.D entire (`default_button_states`, `default_button`, `geometry::default_button_rest` with its `GEOMETRY_NOTES` entry, the tests, the two `sinks` entries, 6.D.5's README paragraph and row and the `lib.rs` row); 6.G's gpui showcase (the button, its Widget Info and claims, the `HOW` lines, the tests) | none of its own: built beside K6c and K6d |
| K6c — iced | 6.E entire (the style, its rows, pairs, `DERIVED` entries, the regenerated `contract-leaves.toml`, the README and `lib.rs` lists); 6.G's iced showcase (the button, its route arms and tables, the tests) | none of its own |
| K6d — egui | 6.F entire (the accessor, the three mapping rows, the counts, `mapping.md`, the install test, the parity exception); 6.G's egui showcase (the button and its tests) | none of its own |
| K6e — outside the code | 6.H | after K6b to K6e: `check_showcase_parity.py --check-list`, 6.G's check on the built tree and the parity rehearsal; then one `scripts/check_release.sh --commit` with the GPUI Fast legs and the five documentation checkers over K6a to K6e; push; CI read |

The element-list edit of 6.G (`docs/showcase-elements.toml`) is made by the
orchestrator before K6b to K6d start, from 6.G's text, and committed with them: once
the element is listed, every showcase must draw it and route its leaves
(`check_showcase_parity.py --check-list`; gpui's `every_listed_leaf_has_a_route`; iced's
`the_element_list_parses_and_every_leaf_has_a_route`, whose routes for this element are
the contract export's through `drawing()`; egui's
`the_layout_dump_names_every_listed_element_the_basic_page_draws`), so each builder's
showcase tests are red until its own showcase does.

Why K6a is one unit and the connectors wait for it: `property_registry_keys_match_derive_inventory`
fails between the field and its registry keys; `check_platform_facts_rows.py` between
the keys and the rows; the resolved theme serialises an unset soft option as `null`,
so egui's `every_leaf_has_exactly_one_row_and_every_row_is_well_formed` counts the
three new leaves from the field on and fails until their rows land; its T2
(`t2_every_row_s_declared_sinks_are_exactly_what_its_leaf_moves`) fails a row that no
preset pair differentiates and that has no `probe`, which holds until kde-breeze states
the leaves; gpui's `every_colour_leaf_reaches_a_sink_or_is_named` and iced's
`every_colour_leaf_reaches_a_row_or_is_named` (against `contract-leaves.toml`) fail
each new colour leaf (`null` included) until a sink or a row names it;
`check_connector_parity.py` fails egui's new crate-root function until its exception
lands. `the_derived_colours_are_the_presets_under_breeze` compares only the leaves
`derived_leaves` lists, and the reader's new leaves equal the preset's values, both in
K6a.

Why each showcase is with its connector: iced's `styles_cover_every_widget_shown`
fails every public function of `src/styles.rs` with no call site in the showcase, so
`styles::button_default` is red without the iced button; gpui's
`the_showcase_exercises_every_builder` fails every `pub fn` of `variants.rs` and
`geometry.rs` the showcase does not use, and `every_geometry_builder_has_a_note` every
`geometry.rs` builder without a `GEOMETRY_NOTES` entry, so `variants::default_button`
and `geometry::default_button_rest` are red without the gpui button and its note; the
egui showcase's `the_theme_map_names_the_tests_that_check_a_per_call_route` fails a
`tested_by` row until its install test exists. egui's new accessor has no test that
the showcase uses it (searched: `git grep -nE '^\s*fn (every|each|all)_'` over
`connectors/native-theme-egui/src/` and the showcase's tests, read for what each
covers); the element list brings the egui button.

**Gates.** During a unit the agent runs the tests its edits reach and
`scripts/check_strict_panic.sh` for the crates it changed. The `--commit` over the
step is the first workspace gate after the caret's; a failure a later gate finds is
fixed by a new commit, never by rewriting a pushed one, and the record names the
commit that brought it. The branch is pushed by explicit refspec at each `--commit`
gate that passes, and each push's CI run is read. After step 6's push the session also
dispatches the screenshots workflow on that head (`gh workflow run screenshots.yml --ref
v0.7.2-rc1`, no desktop) and looks at the Basic page in its macOS and Windows captures:
the default button's line on the platforms' own fonts, and that the three showcases
agree on it (decision 7). Nothing of that run is committed; a fault it shows is fixed
before step 7.

### 6.I.1b Order inside the units

- 6.B with or before 6.C.3 (a preset comment cites a facts row).
- 6.C.3 before the iced `contract-leaves.toml` is regenerated (the walk reads the
  presets): K6a before K6c.
- 6.H's backlog lines with or after 6.C.3 (`check_todo_citations.py`): K6e after K6a.
  K6e's texts are this document's, so it is written beside the block; it is committed
  after it, the orchestrator holding its counts against the block's tree (6.H.2's count
  of style functions is the one 6.E.4 writes).

### 6.I.2 The citations the edits move

None is re-pointed (the batch's S6): the edits of 6.A, 6.B.2, 6.C.2, 6.E.2, 6.F.2 and
Part F move lines other files cite by number, and those citations are left as they are
and may drift. The lines the gpui colour claims pin do not move (*Lines that must not
move*), and no gate reads the moved citations by position (Part F, F.1).

### 6.I.3 Gates

The commands of `scripts/check_release.sh --commit`, all of them; what each catches
here:

| Gate | Catches |
|---|---|
| `cargo test -p native-theme --features "$NT_FEATURES"` and without | 6.A.4's model gates; 6.C.4; the preset assertion of 6.A.3 |
| the gpui legs | 6.D.3, 6.D.4, the claims test (no pinned line moved, the new claims at the lines that read their fields), `the_showcase_exercises_every_builder` and `every_geometry_builder_has_a_note` |
| the iced legs, with and without `iced_aw` | 6.E.3's rows, pairs, `DERIVED`, the leaves file; `styles_cover_every_widget_shown` (the showcase's call of `styles::button_default`) |
| the egui legs | 6.F.3, 6.F.4, `the_theme_map_names_the_tests_that_check_a_per_call_route` (`showcase-egui/tests.rs:6872`) |
| the showcase tests | 6.G's routes and element |
| the documentation checkers; `check_connector_parity.py` and its self-test; `check_showcase_parity.py --check-list` | 6.B, 6.H.5, 6.F.5, 6.G |
| clippy, strict panic, `check_panic_patterns.py`, fmt, rustdoc | the new code |
| full run: the two stamps | the assets' stale since `da89b4ba` until the asset run; the compatibility stamp stale from K6a until the close-out's compatibility run |

**Not covered by any gate:** the wording of the docs, CHANGELOG, roadmap, backlog and
rationale texts; the facts cells against their sources; the citations the edits move,
left as they are (S6); the look of the default button in each showcase (the captures, which the session
looks at before the asset run's assets are committed, 7.J.4).

### 6.I.4 The release

Step 6 has no close-out of its own: 0.7.2 is cut after step 7's units, and the
close-out is 7.J.4's. Tag, publish and release are the maintainer's.

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
the colour vector grows by two and the fields take the next two indices (today's
vector is `9..=9`, as at `0fa28027`: 0.7.2 added no `ListTheme` field and step 6 adds
none, so `colors[9]` and `colors[10]`). The only full `ListTheme` literals
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
| `defaults.rs:194` (`default_has_all_none_options`) | after `assert!(d.selection_inactive_background.is_none());`: `assert!(d.selection_inactive_text_color.is_none());` |
| `native-theme/src/model/resolved.rs:106-107` (`ResolvedDefaults`' `pub selection_inactive_background: Rgba,` and its doc) | after it: `/// Text color over \`selection_inactive_background\`.` and `pub selection_inactive_text_color: Rgba,` |
| `resolved.rs:366` (`sample_defaults`) | after `selection_inactive_background: c,`: `selection_inactive_text_color: c,` |
| `native-theme/src/resolve/validate.rs:60` | after `selection_inactive_background,` in `validate_defaults!`' `option_color` list: `selection_inactive_text_color,` |
| `native-theme/src/resolve/inheritance.rs:98-101` | after the block: `// selection_inactive_text_color <- selection_text_color` and `if d.selection_inactive_text_color.is_none() { d.selection_inactive_text_color = d.selection_text_color; }` |
| `inheritance.rs:381-389` (`implemented_targets`) and `:826-834` (`all_inherited_targets`) | `"defaults.selection_inactive_text_color"` after the inactive background's entry |
| `native-theme/src/model/mod.rs:1988` (`theme_defaults_field_info_matches_baseline`) | after `"selection_inactive_background",`: `"selection_inactive_text_color",` |
| `native-theme/src/resolve/tests.rs` | after `resolve_phase1_explicit_selection_preserved` (`:117-132`): `resolve_phase1_selection_text_fills_selection_inactive_text` (`selection_text_color` set, the new field `None`: it inherits) and `resolve_phase1_explicit_selection_inactive_text_preserved`; the list of derived fields `clear_derived_fields` clears (`:2343-2345`, its last lines, after "Defaults internal chains") gains `v.defaults.selection_inactive_text_color = None;`; and `fully_populated_variant` (`:780-790`), which sets every derived defaults field by hand because its callers run `validate()` without `resolve()` (`:279`, `:285`, `:311`, `:327`, `:339`, `:354`, `:381`, `:401`, `validate_fully_populated_returns_ok` at `:1299-1306`, and the later ones from `:1314`), gains after `:787` (`selection_inactive_background`) `v.defaults.selection_inactive_text_color = Some(Rgba::rgb(255, 255, 255));`, the value its `selection_text_color` line gives (`:786`): without it `validate()` reports the required field missing. The soft options of 6.A.1 and 7.A.1 are `Option`s in the resolved structs, which `validate()` does not require, so the helper needs none of them |
| `native-theme/tests/proptest_roundtrip.rs:153-185` | the `ThemeDefaults` literal of `arb_theme_defaults`: the field takes `colors[23]`, the colour vector's one unused index (today's vector is `24..=24`, as at `0fa28027`, `:136`, and indices up to 22 are used; step 2's `focus_ring_keyboard_only` is a `bool`, its own tuple member, and step 6 adds no `ThemeDefaults` field); the only full `ThemeDefaults` literal outside `resolved.rs` (scanned as for `ButtonTheme`) |
| `native-theme/tests/serde_roundtrip.rs:85` (`fully_populated_variant`) | after `v.defaults.selection_inactive_background = Some(c(180, 210, 233));`: `v.defaults.selection_inactive_text_color = Some(c(35, 38, 41));`. No assertion of its own: the test round-trips the whole theme and compares it whole, so a field that does not survive fails it |

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

**What step 1 carried**, in today's tree since 0.7.1 and not carried twice: `kde_shade.rs`
as `native-theme/src/kde/shade.rs`, its table `native-theme/tests/vectors/kde_shade.tsv`
(the folder's one file) and its declaration `mod shade;` in `kde/mod.rs`, after `pub(crate)
mod fonts;` (step 1's A.1-A.3, `docs/archive/todo_v0.7.1_kde-open-items-step1-spec.md`;
read today: `tail -n +13` of `shade.rs` and the table hash to A.1's and A.2's sums). The
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
the form of `kde/mod.rs:460`, on its `mod tests`) nor the Write hook's exemption for `#[cfg(test)]`
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

As step 1's A.4, row by row: `#![deny(clippy::unwrap_used)]` and
`#![deny(clippy::expect_used)]` (`native-theme/src/lib.rs`; none outside tests);
`check_strict_panic.sh` (P8's file carries
`#[allow(clippy::modulo_arithmetic, …)]` on `wrap`, `kde_color_common.rs:174-178`, under
the same lint list: the `STRICT_PANIC_LINTS` arrays of this tree and of P8 diffed equal
today, and it passes in this tree: the trial T-K4 (2026-11-19,
`<record>/trial/t-k4-result.txt`) copied the two files in as they are, declared as 7.B.3
has them, and ran it with the gate's features, exit 0; should a later tree find
otherwise, the work stops and the maintainer is told what failed, as step 1's A.4 rules
(`docs/archive/todo_v0.7.1_kde-open-items-step1-spec.md`, A.4's `check_strict_panic.sh`
row): a strict-panic finding in code is
fixed or brought to the maintainer, never allowed by the implementer, and the carried
file is not edited);
`check_panic_patterns.py`; clippy `-D warnings` (dead code by 7.B.3; T-K4's run found one finding only, `kcu_tint`
never used, which the reader's call in the same unit removes, 7.J.1; an ordinary
clippy finding in a carried file, one that is no strict-panic finding and no hook
refusal, is no stop: it is fixed in the port, the 7.B.2 hash checks made on the file
before the fix, and the commit body names each fixed line and the lint); the Write hooks
(the main checkout's `.claude/hooks/no-invented-values.sh` exempts `*/src/platform/*`,
which `src/kde/` is not: each of its regexes, and those of `no-runtime-panics.sh`, run
with `grep -cE` over P8's two files and over the two notices of 7.B.2 today gives 0 matches;
the implementer runs each over the written files first and stops on a match);
rustdoc (both modules private; the carried docs hold no intra-doc link, read today).

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

The breezestyle link is the one `:1429`'s KDE cell (`header_background`) gives for the same lines (read
today); the breezehelper lines are those the reader's comment cites for `hoverColor()`
and `focusColor()` (`kde/colors.rs:566-569`, the comment that begins "A field's frame
(docs/platform-facts.md §2.4)"), not re-read here (no network). The
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
macos-monterey stands for macOS 12 to 15 (`CHANGELOG.md:38`, `[0.7.0]`'s entry on the
preset's name, "it covers macOS 12 to 15"); only 15 was measured, as
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
`.github/workflows/`, which finds besides it `check_release.sh:786`, "its header gives
the evidence", and `screenshots.yml:133`, both the word in another sense); the step
cache's `QUICK_UNREAD` (`scripts/step_cache.sh`) lists `docs/evidence/*` among the paths
no test or build script reads.

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

**Import.** Beside `use super::shade::{KdeShadeRole, kde_shade};`, the file's first `use`:
`use super::kcu_tint::kcu_tint;`.

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

**Helpers**, after `frame_contrast`, `contrast` and `mid_shade`, which follow each other
directly before `populate_colors`' doc:

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

**`populate_colors`**, after the list block (`:730-746`; the block under "// KDE-02: list
fields", whose last lines set `variant.list.header_background = button_bg;` and the
header font's colour, before "// KDE-02: link.visited"), where `window_bg` (`:638`; bound
in the progress bar's block), `window_fg` (`:477`) and `selection_bg` (`:580`; both bound
at the function's top, beside `view_focus`), `button_bg` (the checkbox's block) and
`view_hover` (the input's block) are bound:

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

`Rgba::from_f32` rounds each channel × 255 to the nearest, as `mid_shade` does (its doc:
"[`Rgba::from_f32`] rounds each times 255 to the nearest"). `populate_colors`' doc gains "the header's hovered and
pressed fills and the selection of a view without the focus" in its list of computed
colours (the paragraph "Some colours Plasma computes from the scheme, …", which step 6's
6.C.2 extends by "the default button's fill and frame"), and a
sentence: "The inactive selection follows `[ColorEffects:Inactive] ChangeSelectionColor`
and `Enable`; a scheme that sets `[ColorEffects:Inactive] Enable=true` gets the pair
before the inactive group's contrast, intensity and colour effects, which recolour
every colour of an inactive window, a palette the model does not hold (decision 18;
docs/todo.md, *Model: an inactive window's palette*)." Its paragraph on the blends on
the accent (`kde/colors.rs:464-470` at `0fa28027`; today's text: "The blends on the
accent (the checked checkbox, the progress fill, the switch's on track and its outline,
the list's hover, and the toolbar button's pressed fill) come from kdeglobals'
`[Colors:Selection] BackgroundNormal`, so an application's `with_overlay` accent or the
portal's … does not move them") gains "the inactive selection" in its list, after "the
list's hover", so that it reads "…, the list's hover, the inactive selection, and the
toolbar button's pressed fill": the tint reads the same key,
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
either way (`kcu_tint.tsv:58`, `kde_selection_inactive.tsv:35`, read in `P8` at
`78ee0efd`, where the table holds 70 data rows). Calling
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

`[dark.defaults]`, after `:634-635` (`selection_text_color = "#fcfcfc"` and its comment): the same with `32,35,38`, `(BreezeDark.colors:20 at
breeze f0b1d75)`, `#1b4155`, `252,252,252`, `(BreezeDark.colors:138, breeze v6.7.5)`,
`#fcfcfc`.

`[light.list]`, after `header_background` (`:347-350`, its three comment lines and
`header_background = "#fcfcfc"`), before `alternate_row_background`'s comment:

```toml
# platform-facts.md (§2.15): header_hover_background, KColorUtils::mix(Button #fcfcfc, [Colors:View] DecorationHover
# 61,174,233 (BreezeLight.colors:117), 0.2), a hovered header section (breezestyle.cpp:6828-6837 at breeze f0b1d75)
header_hover_background = "#d6ecf8"
# platform-facts.md (§2.15): header_active_background, KColorUtils::mix(Button #fcfcfc, [Colors:View] DecorationFocus
# 61,174,233 (BreezeLight.colors:116), 0.2), a pressed header section (breezestyle.cpp:6828-6837 at breeze f0b1d75)
header_active_background = "#d6ecf8"
```

`[dark.list]`, after `:935-938` (`header_background` and its comment): the same with `#292c30`, `(BreezeDark.colors:118)`,
`(BreezeDark.colors:117)` and `#2d4655`. The scheme line numbers are the ones the file
already cites (`kde-breeze.toml:17`, `:21`, `:166`, `:602`, `:606`, `:752`), pinned
sources (breeze at a version); the `ChangeSelectionColor` lines are the facts row's
(`selection_inactive_background`), at f0b1d75.

### 7.D.4 macOS presets

**Inside the existing block.** The key goes into the block, one value line, with no
comment of its own; the block's comment is rewritten in place, in the batch's preset
comment form (6.C.3: v0.7.1's LN unit makes `check_preset_citations.py` read it and
compare nothing with the facts file), naming after the colon the rows it covers:

`macos-monterey.toml:45-47` (the three lines from "# platform-facts.md:1059-1063:
selectedContentBackgroundColor, alternateSelectedControlTextColor," to "# probe :324,
:317, :325, :331, :330"; neither macOS preset has changed since `0fa28027`, so these
lines are today's as well) becomes:

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
are pinned evidence. `check_preset_citations.py` reads the section form
`platform-facts.md (§2.1.3)` as it reads the line form, and "compares no citation with
the facts file's rows" (its docstring, read today). The other comments both steps add sit each directly above the one
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
names the rows, never their lines: S6). No new `NSColor` is read. `macos.rs` has not
changed since `0fa28027`, so the lines given here are today's as well.

**No new `unsafe`.** The edit adds no `unsafe` block and no FFI call: it is one field of
the `crate::ThemeDefaults { … }` literal in `read_appearance_colors`, a safe `fn` with no
`unsafe` block in its body, taking the `Option<Rgba>` the function already holds. It is
modelled on that literal's neighbouring reads of the same value, `text_color: label,`
and the four status texts (`danger_text_color: label,` and the warning, success and info
ones), where `label` is `let label = nscolor_to_rgba(&label_c, &srgb);` and `label_c`
is `let label_c = NSColor::labelColor();`, objc2's safe binding, bound at the function's
head. The module-level `#![allow(unsafe_code)]` ("Objective-C FFI via objc2 -- no safe
alternative") covers the `unsafe` blocks of the font reads (among them
`NSFontTraitsAttribute`, `NSFont::monospacedSystemFontOfSize_weight`,
`CTFont::new_ui_font_for_language` and `font.attribute`), the file's only ones; this edit
needs none of it. The test's new assertion calls
`NSColor::labelColor()`, a safe call, as its other assertions call
`NSColor::alternateSelectedControlTextColor()`.

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
| The `Scheme` literals that name every field (read today with `grep -n 'Scheme {' native-theme/src/kde/mod.rs` and the `..` lines below each: three of the 19 literals of `kde/mod.rs` have no `..`; the other 16 spread another one, 14 `..BREEZE_LIGHT_SCHEME` and 2 `..without_header` in `a_header_group_takes_a_missing_key_from_the_window`, and need nothing for the two new fields; step 6 and the caret's units add no `Scheme` literal) | `BREEZE_LIGHT_SCHEME`, `BREEZE_DARK_SCHEME` (row below), and `a_colour_scheme_keeps_the_stated_edge_geometry` (`:1657-1686`, its literal `:1661-1670`): `selection: "233,84,32"` becomes `selection: ("233,84,32", "255,255,255")`, and it gains `change_selection_color: None` and `inactive_enable: None`, so the tint applies, which the test does not read (it compares the edge geometry alone) |
| `an_accent_moves_the_on_track_the_pressed_button_the_selected_segment_and_the_list_hover` (`:1420-1438`; its literal `:1422-1425` spreads Breeze Light's and sets `selection`) | its `selection: "233,84,32"` becomes `selection: ("233,84,32", "255,255,255")` (Breeze Light's text kept); gains `defaults.selection_inactive_background` (tinted towards the selection) |
| `BREEZE_LIGHT_SCHEME`, `BREEZE_DARK_SCHEME` (`:1215-1244`) | `selection: ("61,174,233", "255,255,255")` / `("61,174,233", "252,252,252")`, `change_selection_color: Some("true")`, `inactive_enable: Some("false")`; each doc gains "Selection ForegroundNormal :95 (Dark :96)" among the keys it cites at breeze v6.7.5, as the doc already cites Selection BackgroundNormal :87 (:88) there, and "`[ColorEffects:Inactive]` ChangeSelectionColor :19 and Enable :25 at breeze 6.8.2 (Dark :20, :26)", the version the parked facts cite for these (`F/kde/inputs.toml:1029-1037`, `:3008-3016`, the Selection set's `foreground_normal`; `F/kde/tokens.toml:11255-11293`, at `78ee0efd`); ChangeSelectionColor's `:19` and `:20` are also this tree's facts row's at breeze f0b1d75 (`docs/platform-facts.md:1061`). That `:95` and `:96` are the lines at v6.7.5 rests on the parked facts' numbers at 6.8.2 and the files' layout the doc's other citations show (each set's ForegroundNormal eight lines below its BackgroundNormal: Button :31 and :39, View :115 and :123); `kde-breeze.toml`'s `selection_text_color` comments cite the same lines at that version ("[Colors:Selection] ForegroundNormal 255,255,255 (BreezeLight.colors:95, breeze v6.7.5)", and 252,252,252 at "BreezeDark.colors:96, breeze v6.7.5", read today); the v6.7.5 files were not read here, and whether `:25` and `:26` hold at f0b1d75 was not verified |
| `derived_leaves`, `resolved_leaves` | gain `list.header_hover_background`, `list.header_active_background`, `defaults.selection_inactive_background`, `defaults.selection_inactive_text_color`; on the resolved side the two header fields directly, `theme.list.header_hover_background` and `theme.list.header_active_background`, already `Option<Rgba>`, as the array's other soft options are (`("switch.hover_unchecked_background", sw.hover_unchecked_background)`, `kde/mod.rs:1323-1329`), and the two defaults fields wrapped, `Some(theme.defaults.selection_inactive_background)` and `Some(theme.defaults.selection_inactive_text_color)`, which are `Rgba` (`resolved.rs:107`, 7.A.2); the count grows by four: today both arrays are `[(&'static str, Option<Rgba>); 39]`, `derived_leaves`' doc "The thirty-nine colour leaves", step 6's 6.C.4 makes them 42 ("forty-two"), so after this step 46 ("forty-six"), the array type and the doc's word both. The four go at the end of both arrays, after every entry the arrays hold on the tree, by the rule every earlier step follows: each appends its own leaves after what the steps before it appended (step 1's B.5, the steps 2–3 specification's 2C.1 and Part 3C, the steps 4–5 specification's A.6 and B.5, and step 6's 6.C.4 here; the caret's specification touches neither array), and none fixes another place for them. Today both arrays end with
`("toolbar.button_checked_background", …)`, and step 6 appends its three
`button.default_button_*` leaves after it, so the four follow
`button.default_button_border_color`. The order of the four: `list.header_hover_background`, `list.header_active_background`, `defaults.selection_inactive_background`, `defaults.selection_inactive_text_color`, the two arrays in the same order. `moved_leaves` (`:1384-1393`) keeps `resolved_leaves`' order (it filters the two zipped arrays), so in each asserted array below a new leaf stands after every leaf the array lists on the tree, and where two are added, in that order |
| `the_derived_colours_are_the_presets_under_breeze` | passes with 7.D.2 and 7.D.3, no edit |
| `the_window_set_moves_the_popover_edge`, both schemes (`with_header` and `without_header`) | gain `defaults.selection_inactive_background` (the window tinted) and `defaults.selection_inactive_text_color` (the window text) |
| `the_button_set_moves_the_button_slider_switch_and_header_leaves` | gains `list.header_hover_background` and `list.header_active_background` (both mix the Button background) |
| `the_view_decorations_move_the_frames_the_tab_hover_and_the_handle` | the hover sub-case gains `list.header_hover_background` only, the focus sub-case `list.header_active_background` only |
| `the_header_set_moves_the_toolbar`, `the_view_set_moves_the_card_edge_and_the_menu_panel`, `the_frame_contrast_moves_the_edges`, `the_contrast_moves_the_table_grid` (both schemes, `contrast` `"0"` and unset) | none (no new leaf reads the Header set, the View set's two normal colours, `frameContrast` or `contrast`); these four and the accent's, the window set's, the button set's and the view decorations' tests above are the eight tests of `kde/mod.rs` that compare an exact list of moved leaves (`grep -n 'moved_leaves(' native-theme/src/kde/mod.rs`, read today) |
| new: `the_inactive_selection_follows_change_selection_color` | `change_selection_color: Some("false")` gives the focused pair in both fields; `None` with `inactive_enable: Some("false")` the focused pair; `None` with `None` the tint; `Some("yes")`, which is no boolean form the reader accepts (decision 11), with `inactive_enable: Some("false")` the focused pair, so the value counts as missing and the rule falls to the next key |

`native-theme/src/kde/colors.rs`, new tests after the default button's (step 6's
`test_default_button_is_breezes` and `test_default_button_frame_keeps_qts_precision`, 6.C.4):
`test_header_sections_are_breezes` (#d6ecf8 / #2d4655 for both fields under Breeze's keys;
a scheme whose `DecorationHover` and `DecorationFocus` differ gives two different
fields; its fixture needs `[Colors:View] DecorationHover` beside `DecorationFocus`,
since the hovered fill reads it (7.D.2), and the file's hand-made groups,
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
`NoReceiver` (`contract.rs:1169-1177`: none of the 140 `ThemeColor` fields is an
inactive selection, `GC/theme/theme_color.rs:61-347`, and a `ListItem` fills
`list_active` or `accent` whatever the window's state, `GC/list/list_item.rs:260-267`).
The table header: `table_head` and `list_head` (`colors.rs`, the header rows
`contract.rs:373-411`); gpui-component's header section has no hover or press style
(`GC/table/state.rs:1932-2023`, `render_th`: no `.hover(` or `.active(`; the sort icon's
own hover and press, `:1915-1916` in `render_sort_icon`, are the icon button's, in the
`secondary` and `secondary_active` tokens). Nothing draws text on an inactive selection.

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
  before its fill whatever the window's state (`list/list_item.rs:207-267`), and no
  `ThemeColor` field is an inactive selection's text (`theme/theme_color.rs:61-347`)".
- `list.header_hover_background` and `list.header_active_background`: "a `DataTable`'s
  header section is `render_th`'s, with no hover or press style (`table/state.rs:1932-2023`);
  the delegate's `render_th` content lies inside the cell's padding (`:1553-1592`), not
  over the section".

The cited lines are gpui-component 0.10.0's, read today: `ThemeColor` is
`theme/theme_color.rs:61-347`, the range the existing entry (`:1172-1175` at `0fa28027`)
cites; a `ListItem` sets its text colour at `list/list_item.rs:207` and fills a selected
row at `:260-267`, under `list.active_highlight` (the connector's setting) with no text
colour of its own; `render_th` is `table/state.rs:1932-2023` and `render_cell`, whose
padding holds the delegate's content, `:1553-1592`. Where a header row height is set and
the column has no paddings of its own, `render_th` takes the cell's vertical padding off
(`:1948-1951`) and its horizontal padding stays, so the content still does not cover the
section.

**Crate doc**, the coverage table of `src/lib.rs` (`:108-…`), in place, the mapped
counts unchanged since the three leaves reach no `ThemeColor` field:

- `:110`, "| `defaults` colors | 14 of 24 |" becomes "| `defaults` colors | 14 of 25 |".
  The 24 are the 23 `color` keys of the registry's `[defaults]`
  (`docs/property-registry.toml:96-146`, the last at `:127`, counted today) and `border.color`, which the
  row's list names; the fonts' colours it does not count (the count has been 24 since
  `96f8285f`, whose registry held the same 23 keys; today's `[defaults]` holds the same
  23 `color` keys and the row still reads "14 of 24"). Step 7 adds one `color` key; the
  one `[defaults]` key 0.7.2 added is step 2's `focus_ring_keyboard_only = "bool"`, and
  step 6 and the caret's units add none there (6.B.1 edits `[button]`, the caret's A.6
  `[input]`).
- `:122`, "| `list` | 7 of 14 + geometry |" becomes "| `list` | 7 of 16 + geometry |".
  The 14 are the fields of the registry's `[list]`, `:377-391`, colours and others
  alike (counted today: 14, and the row still reads "7 of 14 + geometry"; the count went
  from 13 to 14 in `44e1a07a`, which added `header_height_px`); neither step 6 nor the
  caret's units add to `[list]`; step 7 adds two, which the row's list of mapped fields
  does not gain.

## 7.F iced connector

**Readers today.** None: `UNREAD_COLOURS` names `defaults.selection_inactive_background`
(`contract/derived.rs:1390-1397`: iced draws a text field's selection only while it is
focused); the showcase's colour swatches show it (`showcase-iced.rs:16426`,
`("sel_inactive", d.selection_inactive_background)`); iced's
`table` has no style setter and its header is the application's element (`IW/table.rs:35-44`,
`:149-187`); the showcase's table header is a `container` (`showcase-iced.rs:11697-11712`,
in `basic_table`, the `container(row![…])` tagged `"basic.table.header"`).

**Accessors**, `src/lib.rs`, after `list_header_height` (its doc and `:1610-1613`), in its form
(`resolved: &native_theme::theme::ResolvedTheme`): `list_header_hover_background` and
`list_header_active_background`, each `-> Option<iced_core::Color>`, with the docs of
7.E's. No `selection_inactive_text_color` (decision 13): the exception
`scripts/connector_parity_exceptions.toml:202-206` (`connector = "iced"`,
`name = "selection_inactive_background"`, `kind = "absent"`, its reason ending "so no
style takes an inactive selection") gains a second entry beside it, the
same reason with "and so no style takes its text colour either".

**Contract** (decision 23). `src/contract/rows.rs`, `ACCESSOR_ROWS` (`:119-…`; 14 rows
today, the nine crate-root colour accessors', three under `#[cfg(feature =
"widgets")]` and two under `#[cfg(feature = "spinner")]`; `AccessorRow`'s `native` and `get` are `fn(&ResolvedTheme) ->
Option<Color>`, `src/contract.rs`), two rows in the form of `crate::selection_background`'s (`:158-162`), with a comment "iced
0.14's `table` has no style setter, and a header cell is the application's own element
(`iced_widget-0.14.2/src/table.rs:35-44`, `:149-187`): the application draws a
section in this fill": `accessor: "crate::list_header_hover_background"`, `native: |r|
r.list.header_hover_background.map(to_color)`, `get: |r|
crate::list_header_hover_background(r)`, and the same for
`list_header_active_background`. The walk splices each leaf (`null` on every preset but
kde-breeze, a colour there) and the accessor moves with it, so the two leaves reach
their rows and are named nowhere else. `src/contract/derived.rs`, `UNREAD_COLOURS`,
after `:1390-1397` (the `leaf: "defaults.selection_inactive_background"` entry):
`defaults.selection_inactive_text_color` alone, with `:1393-1396`'s
evidence ("iced draws a text field's selection only while the field is focused
(`iced_widget-0.14.2/src/text_input.rs:500-503`, `text_editor.rs:1021`), so an
unfocused selection has no look to give"). `contract-leaves.toml` regenerated (6.E.3's command): the two header leaves
reach `crate::list_header_*`, the text colour is named.

**Documentation.** `README.md`'s "Full helper list" (`:337-355`): after
`selection_background` (`:351`, the line "`selection_background`, `info_color`,
`info_text_color`,"), "`list_header_hover_background` and
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
`mapping.md:66`). egui has one selection pair (`EG/style.rs:1300-1310`, `Selection`'s
`bg_fill` and `stroke`); egui_extras lays the header row out without hover
(`EX/table.rs:490-506`, `TableBuilder::header`'s `TableRow { …, hovered: false, … }`).

**Accessors**, `accessors.rs`, after `selection_inactive_background`:
`selection_inactive_text_color(t) -> egui::Color32`; and after `list_header_height`
(`:525-532`): `list_header_hover_background(t) -> Option<egui::Color32>`,
`list_header_active_background(t) -> Option<egui::Color32>`, docs as 7.E's with
"egui_extras' header row has no hover (`egui_extras/src/table.rs:490-506`); for a header
the application paints". `lib.rs:199-203` (the `pub use accessors::{…}` list, in its
alphabetical order) re-exports them.

**Mapping**, `mapping.toml`:

- `["defaults.selection_inactive_background"]` (`:151-154`): the gap is closed, so
  `verdict = "derived"`, `tested_by = "T18(a)"` (its accessor), the `sub_tag` and
  `upstream` keys go.
- `["defaults.selection_inactive_text_color"]`, after it: `verdict = "derived"`,
  `tested_by = "T18(a)"`.
- `["list.header_hover_background"]`, `["list.header_active_background"]`, after
  `["list.header_height"]` (`:1973-1975`): the same, `verdict = "derived"`,
  `tested_by = "T18(a)"` (`["list.header_height"]` itself is `T18(e)`'s).

`mapping_tests.rs`, as deltas on the tree the caret's units and step 6 leave (6.F.3),
each with today's value (read in `src/mapping_tests.rs` today): the leaf count of
`every_leaf_has_exactly_one_row_and_every_row_is_well_formed` grows by three (today 525;
the caret's units make it 528 and step 6 531, so 534), and its message (`:240`, today
"521 ResolvedTheme leaves + 4 layout leaves (§5.7)") grows as in 6.F.3, to "530
ResolvedTheme leaves + 4 layout leaves (§5.7)"; in `verdict_totals_match_the_specification`,
`foundation`'s derived count grows by two and its unmappable count falls by one (today
`("foundation", [18, 1, 37, 11])`, which neither the caret's units nor step 6 change, so
`[18, 1, 39, 10]`), `inputs`' derived count by two (today `[4, 40, 32, 12]`, the caret's
units make it `[4, 40, 34, 13]` and step 6 leaves it, so `[4, 40, 36, 13]`; `list` is
in the Inputs group, the test's `group_of` and `mapping.md`'s Totals, "Inputs (input,
text area, combo box, list)", as is the caret work's `input`);
`tags.get("source-side gap")` (`:489`, 16 today, which neither the caret's units nor
step 6 change) falls by one, to 15, and the test's doc, "sub-tags (16 source-side gap,
…)" (`:440`), with it. `mapping.md` regenerated
(`cargo test -p native-theme-egui --lib -- --ignored regenerate_mapping_md`, its header);
its Totals (today Foundation 67 leaves, 18 / 1 / 37 / 11, and `source-side gap` 16) then
show Foundation 68 leaves, 18 / 1 / 39 / 10, two more Inputs leaves and DERIVED, and
`source-side gap` 15.

**Tests**, `install_tests.rs`: `t18_a_the_inactive_selection_and_header_accessors_are_their_leaves`,
doc first line "T18 (a): `selection_inactive_background`, `selection_inactive_text_color`,
`list_header_hover_background` and `list_header_active_background` return their
leaves.", over kde-breeze and adwaita in both modes (adwaita's header fields `None`).

**README** (`:262`, the accessor list, "`window_title_bar_text_color`,
`dialog_button_order`, `list_header_height`…", the list step 6's 6.F.5 extends by
`default_button`): the three, one sentence.

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
(gpui-component/table/state.rs:1932-1975, render_th)"; `list.header_active_background`,
the same with ".active(..), else the hovered fill". The range carries the symbol its
lines hold, as 6.G's routes say `every_prose_citation_still_exists` requires: `:1932`
is `fn render_th(` and `:1965` calls `self.delegate.render_th(..)` in gpui-component
0.10.0, and no line of the range sets a hover or active style. iced: `iced_route`'s list arms (`:8250-…`, in their form,
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
  `:604-622`, nor `App`'s, nor GPUI Kit 0.10.0's `open_window`, which the harness calls
  (`src/lib.rs:181-196`: `cx.open_window` and a `Root`, no activation), activates a
  window it opens). The harness `open_with` (`showcase-gpui/tests.rs:119-139`), which
  opens its window through `open_set_up`, as `open_with_widget_icons` does, stays as it
  is, `open_set_up` with it (decision 21). A test that needs the
  active look activates its own window, as gpui-component's own tests do
  (gpui-component 0.10.0 `notification.rs:1196`) and as the connector's own
  `src/widgets/tests.rs` does (`fn activate`), with `cx.update(|window, _|
  window.activate_window())` (`src/window.rs:6917`) and `cx.run_until_parked()` (the
  status change is a spawned task, `platform.rs:396-410`) before its next `draw`;
  `VisualTestContext` has no `activate_window` of its own (`src/app/test_context.rs`
  has only `deactivate_window`, `:1074-1080`). A test of the swap deactivates it with
  `VisualTestContext::deactivate_window()` ("Simulates the user blurring the window",
  through `set_active_window(None)`), after activating it first, so that the change is
  one the window sees. The showcase's tests also run on GPUI Fast
  (`scripts/check_release.sh:715`, the connector's `--all-targets`, the check "test
  (native-theme-gpui, gpui-fast, native, watch)"; `:854`, the showcase, "test (gpui
  showcase, gpui-fast, native, watch, runner environment)"), the locked gpui-fast 0.1.16
  (`Cargo.lock:3274-3275`, its `name = "gpui-fast"` entry), whose test platform
  reads the same: `set_active_window` `src/platform/test/platform.rs:392-412` (a spawned
  task, as gpui-pre's), `TestWindow::activate` and `is_active` (`false`)
  `src/platform/test/window.rs:436-447`, `deactivate_window`
  `src/app/test_context.rs:1087-1093`, `Window::activate_window`
  `src/window.rs:7082-7084`, `is_window_active` `:3321`, `Window::active` read from
  `is_active` at `:1758`. Each such line carries the comment "// The swap of
  docs/archive/todo_v0.7.2_kde-open-items-steps6-7-spec.md 7.H.2 follows the window's
  state; this test reads the <active | inactive> look" (the path as it is after 7.I.6:
  code names a planning document by its archived path, the batch's rule 22 (c)).
- **egui**: `ctx.input(|i| i.focused)` (egui 0.37.0 `src/input_state/mod.rs:316`), from
  `RawInput::focused`, `true` by default (`src/data/input/raw_input.rs:106`), which
  `egui_kittest` keeps (egui_kittest 0.37.0 builds its input with
  `..Default::default()`, `src/lib.rs:238-241`), so the showcase's tests keep the active
  look; egui-winit starts at `false` and sets it from the window's focus event, asking
  for a repaint (egui-winit 0.37.0 `src/lib.rs:163`, `:483-505`; on macOS it takes
  `window.has_focus()` at that event in place of the event's value, `:484-491`).
  Decision 21 says what the real showcases show before the first focus event.

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
| `the_tables_headers_take_the_themes_header_height`, `the_basic_page_holds_every_group_in_its_column` | adwaita, kde-breeze; material-3, kde-breeze | the height of `basic-table-header`; the bounds of `basic-table` in its column | none: sizes, not the row's fill |
| `a_table_row_reports_what_the_table_paints_on_it` (`:6917`), `the_tables_sort_button_sorts_its_rows` (`:7005`), `a_moved_column_keeps_its_data` (`:7040`) | system | the Data page's `DataTable` rows | none: gpui-component's `DataTable` is not one of the rule's places |
| `the_copy_box_takes_the_buttons_hover_and_pressed_text_colours` (`:4130`) | kde-breeze | `selection_text_color` as a button text colour (`:4136`) | none: no row |
| `the_native_number_field_reports_what_it_draws` (`:10904`) | kde-breeze | the number field | none (the scan matched the next test's doc comment) |

No other test of the file reads the Basic table's rows (`grep -n 'basic-table-row'`:
four hits, in the same two tests today as at `0fa28027`, where they were `:11013`, `:11027`, `:11074`, `:15271`, all in the tests above). The new gpui test of the
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
    'TableColours::of\|table_row_style(\|list_row('`, is one of those above (ten, in
    the same functions today as at `0fa28027`; at `0fa28027`: `:4997`, `:8992`, `:11254`, `:11639`, `:11705`, `:11726`, `:11834`, `:22556`,
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
  which `scripts/check_release.sh:543-550` runs over the showcase with `-D warnings`
  (the checks "clippy (native-theme-gpui, native, watch)", "clippy (native-theme-gpui,
  gpui-fast, native, watch)" and "clippy (native-theme-gpui, no features)", each
  `--all-targets`). So
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
(`GC/list/list_item.rs:211`, `:260-267`), whatever the window's state; its text colour
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
caller's style, whatever the window's state (gpui-component/list/list_item.rs:260-267,
list_active), and a text colour without its fill is no pair". The range carries the
symbol its lines hold (6.G's routes): `:262` is `this.bg(cx.theme().list_active)` in
gpui-component 0.10.0; the item's `render`, which the lines lie in, is not
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
Of the showcases, gpui's `main` alone asks for activation: after its window opens and
before `--screenshot`'s capture it calls `cx.activate(true)`, which on macOS is
`NSApplication`'s `activateIgnoringOtherApps` (gpui-pre-macos 0.3.12
`src/platform.rs:647-650`), the application's, not a window's; the iced and egui
showcases call no activation (no `gain_focus` or `ViewportCommand::Focus` under their
`examples/`). The Linux captures activate it and check it stays active
(`scripts/capture_window.sh:134-175`). Whether a macOS runner's window is active when
it is captured was not found anywhere (no workflow log was read; no network).

**The capture check** (decision 21; 7.J.4). The session runs it, as it runs and compares
every capture; the maintainer is needed only where its result is undecided. It needs no
workflow edit and no layout dump from the macOS runners: the captures 0.7.2's first asset run
committed are its baseline.

- **When.** Twice. First after step 7's gate and push (7.J.1): the session dispatches
  the screenshots workflow on the pushed head (`gh workflow run screenshots.yml --ref
  v0.7.2-rc1`; the workflow runs on `workflow_dispatch`, and the asset run dispatches
  the same one), downloads its three macOS artifacts (`gh run download <run id>
  --pattern 'screenshots-*-macos'`) into the record and counts their twelve captures;
  nothing of that run is committed, and a capture-code fix it asks for lands before the
  close-out. Then during the release's asset run, after
  `scripts/generate_assets_release.sh` has copied the CI captures into the connectors'
  `docs/assets/` and before any asset is committed (the script commits nothing and ends
  with "Review the assets, then commit"), on the captures that are about to be
  committed.
- **What is read.** The twelve macOS captures,
  `connectors/native-theme-{iced,gpui,egui}/docs/assets/macos-macos-{monterey,tahoe}-{light,dark}.png`.
  In each, below the 32 px title bar, the pixels within 2 per channel of the preset's
  `selection_background` are counted: #0064e1 in a light capture, #0059d1 in a dark one.
  On the Basic page that colour is the selected rows' alone, the list's and the
  table's (the macOS presets' accent is another blue, #007aff).
- **The baseline**, 0.7.2's captures as its first asset run committed them (`b494d6ed`; counted
  on 2026-11-19 with Python and PIL, read-only). Every capture is 1280 × 752: the content
  at scale 1 below a 32 px title bar with no side frame. The window buttons are coloured
  (the red one at (15, 9) is #ff3337 in the six light captures and #ed2125 in the six
  dark ones), so those windows were very likely active: that macOS draws a window's buttons grey when it is not key
  is general macOS behaviour, not checked against a source in this repository.

  | Capture | gpui | iced | egui |
  |---|---|---|---|
  | macos-monterey light | 7330 | 7518 | 7500 |
  | macos-monterey dark | 7330 | 7518 | 7498 |
  | macos-tahoe light | 7326 | 7514 | 7492 |
  | macos-tahoe dark | 7326 | 7514 | 7490 |

  In gpui's macos-tahoe light capture the table's selected row holds 3513 of the 7326
  (x 1075-1242, y 479-502), the list's row the rest.
- **What an inactive window would give.** With this step the showcases draw a selected
  row in the unfocused fill while their window is not active (above). In an egui capture
  both rows then leave the count, which falls to about nothing. In a gpui capture the
  table's row leaves it and the List sample's stays (a gpui-component `ListItem`, which
  paints its own selected fill, above), so the count falls to about half. An iced
  capture cannot tell: iced's state starts active (`window_active: true`, decision 21)
  and changes only on an `Unfocused` event, so its count says only that no such event
  came; for an iced capture the session also reads the window's red button at (15, 9),
  which is grey in a window that is not key. The unfocused fill itself is not counted: #dcdcdc and #464646 are also the
  colours of other surfaces of the page (699 to 25802 such pixels in the twelve baseline
  captures).
- **The decision.** A capture whose count is at least 6000, about four fifths of its
  baseline, was taken with an active window (for a capture wider than 1280 px, should a
  runner capture at another scale, the count is first divided by the square of its
  width over 1280). Where all twelve are, the assets are committed. A capture below
  6000 stops the commit of the assets. The session looks at it: where the selected rows
  are drawn in the unfocused fill, the window was inactive, and the capture code (not
  the showcase) is changed to make the window active before `screencapture` runs;
  where the rows are in the active fill but moved or changed size, that is a layout
  difference of its own, settled as any other the asset run shows. Only where the
  capture shows neither does the session report it to the maintainer, who decides
  before anything is committed.
- **The capture-code fix, should it be needed.** `capture_macos` (`capture.rs:358-383`)
  runs `screencapture` inside one `FrameCapture::poll` (`:217-267`, its macOS branch
  `:241-261`), with no frame drawn between the call and the capture, so the activation
  does not go there: it goes in `FrameCapture::start` (`:175-206`, the macOS branch
  `:191-200`) or in an earlier `poll` that returns `None` (as `poll` already returns
  `None` until `next_try`, `:243-246`), so that the showcase draws its active look
  before the capture; how a runner's window is activated was not read. The fix adds no
  `unsafe` (the project's rule; no exception covers the capture code): its options are
  the toolkits' own activation calls, which the shared crate's start or poll would ask
  each showcase to make: gpui's `Window::activate_window` (gpui-pre 0.3.12
  `src/window.rs:6917`; gpui's `main` already activates its application before a
  capture, above), egui's `ViewportCommand::Focus` (egui 0.37.0 `src/viewport.rs:1175`,
  "puts the window on top of other applications and takes input focus away from
  them"), iced's `window::gain_focus` (iced_runtime 0.14.0 `src/window.rs:431`); none
  was run on a macOS runner. A fix that could activate the window only with `unsafe`
  code is a stop. The change is reviewed, gated, committed and pushed, and the
  check is run again on new captures: the workflow's where the first check asked for
  the fix, the asset run's, repeated with a second capture window, where only the
  second did (7.J.4).
- **Windows is left alone.** The rule does not select windows-11 (its two fills are
  equal), so its captures cannot show a swap.

**The colour swatches.** iced's Theme page list of colours (`showcase-iced.rs:16411-16428`,
the `pairs` of `view_theme_map`)
gains `("sel_inactive_fg", d.selection_inactive_text_color)` after `("sel_inactive", …)`.
gpui's Theme Map lists `ThemeColor` tokens (`showcase-gpui/demo.rs:8897`, `ThemeToken`), none of which
is an inactive selection, and no `selection_inactive` appears anywhere in the gpui
showcase (`git grep -n selection_inactive` over `examples/showcase-gpui/`, empty today):
no edit, and nothing there shows the pair but the table and its info. egui's Theme Map
lists the rows of `mapping.toml` (`showcase-egui/info.rs:760`, `row_cells`, a row's four cells), so it
shows 7.G's new rows by itself: no edit.

## 7.I Outside the code (D10)

### 7.I.1 `CHANGELOG.md`, 0.7.2's section (6.H.1)

No section of its own: step 7's entries join 0.7.2's section. Its break is
named in the section's one *Breaking Changes* bullet, which 6.H.1 extends with
`ListTheme`'s two fields and `ThemeDefaults`' one; step 7 adds no entry there and checks
that bullet against the fields 7.A adds. `### Added`, after step 6's entries:

```markdown
- **native-theme**: `list.header_hover_background` and `list.header_active_background`, the fills of a hovered and a pressed column-header section, each optional (unset, the section keeps its fill). kde-breeze and the KDE reader state Breeze's, `mix(Button, hoverColor, 0.2)` and `mix(Button, focusColor, 0.2)`, #d6ecf8 / #2d4655. No toolkit's table has these states; the connectors' `list_header_hover_background` and `list_header_active_background` give them to a header the application draws, as the three showcases' tables do.
- **native-theme**: `defaults.selection_inactive_text_color`, the text colour over `selection_inactive_background`, the selection of a view without the focus; unset, `selection_text_color`. kde-breeze and the KDE reader state Breeze's pair, the window colour tinted towards the selection by 0.4 under the window text, #c2e0f5 / #1b4155 under #232629 / #fcfcfc, where the colour scheme's `[ColorEffects:Inactive] ChangeSelectionColor` (else `Enable`, else on) asks for it, and the focused pair where not, before any `[ColorEffects:Inactive]` effect a scheme enables; the macOS presets and reader state `labelColor`, #000000d8 / #ffffffd8, the label of an unfocused selected row, which does not adapt (measured on macOS 15 and 26). The KDE reader computes the tint with a clean-room port of `KColorUtils::tint` in Qt's arithmetic, its amount the `f32` 0.4 (0.4000000059604645 as a double, 6e-9 from KColorScheme's 0.4), checked against the 70 rows of a table generated from KColorScheme, to 1e-3 on the 0-255 scale. No toolkit's list tells an unfocused window apart; the gpui and egui connectors' `selection_inactive_text_color` gives the colour beside `selection_inactive_background`. The showcases' tables and lists draw the pair while their window is not active, where the theme's inactive fill differs from its selection fill (kde-breeze and the two macOS presets); gpui's List sample, a gpui-component `ListItem`, paints its own selected fill whatever the window's state and keeps the list pair.
```

### 7.I.2 `ROADMAP.md`

No paragraph of its own: in 0.7.2's section (the caret document's E.3 rewrites its
heading and appends the caret's paragraph, 6.H.2 adds step 6's sentences), after step 6's
sentences, the sentence "A table header's hovered and pressed sections and the
selection of a view without the focus take Breeze's colours under KDE: the model has
fields for both, which the connectors offer to an application, since no toolkit's
table or list has these states; the showcases draw them." The 0.7.1 section's clause
"the selection of an unfocused view waits for a text colour the model does not have
yet" stays: that section describes v0.7.1, as its sentences on the focus ring do after
0.7.2 delivered it.

The count of open `[v0.7.x]` items in *Also in the 0.7.x releases* ("tags 74 open
items `[v0.7.x]`" on 0.7.2's tree): 7.I.5 closes one `[v0.7.x]` entry (item 2; item
4's is `[1.0]`) and opens none (item 3's is `[1.0]`), so the documents unit recounts it on its tree with `grep -c
'^- \[ \] \[v0.7.x\]' docs/todo.md` after 7.I.5's edits, writes that number there and
records it in its commit body. On 0.7.2's tree the grep prints 74; the caret's
documents unit has written its own count before (the caret document's E.3).

The READMEs' count of optional colours (6.H.2a) moves from 45 to 47 in its five places:
`list.header_hover_background` and `list.header_active_background` are optional colours.

### 7.I.3 The first rationale

- **§5.3** (`:889`), a line after the heading: "*Settled by `0004_kde-open-items-rationale.md`
  D9 (0.7.2): `defaults.selection_inactive_text_color`; kde-breeze, the KDE reader and
  the macOS presets and reader state the pair.*"
- **§4.4** (`:528-593`), "Not in 0.7.1", the *No field* bullet (`:585-588`): "the
  header's hover and pressed fills (#d6ecf8 / #2d4655) and its separators" becomes "the
  header's separators (its hover and pressed fills are fields since 0.7.2)".
- **§5.4's status line** (the line after the heading `### 5.4`, `:907` at `0fa28027`,
  which step 3 adds and steps 4 and 5 extend), as step 5 leaves it (steps 4 and 5's
  B.11), found by its sentences: after the sentence that starts "Also by D7's toolbar
  part", a sentence is inserted: "Also by D7's table part (0.7.2): a table header's hovered and pressed fills (`list.header_hover_background`,
  `list.header_active_background`), which kde-breeze and the KDE reader state; no
  toolkit's table draws them, so the three connectors offer them through accessors
  (`list_header_hover_background`, `list_header_active_background`)." (7.A.1, 7.D.3,
  decision 13). Step 5's sentence "The table header is that rationale's D7; the menu
  item's and the list row's outlines and the menu bar title stay in the backlog (its
  §0)." loses its first clause and becomes "The menu item's and the list row's outlines
  and the menu bar title stay in the backlog (its §0)."
- **§7**, the row C "toolbar buttons, sort header" (`:1033`, as step 5 leaves it, found
  by its first cell): the sort header's part, "the sort header waits for its fields"
  in step 5's text (its B.11), becomes "the sort header: **fixed in 0.7.2**, its hovered
  and pressed fills are `list.header_hover_background` and `header_active_background`;
  no toolkit's table draws them, so an application's own header takes them through the
  connectors' accessors".
- **§8, the reply.** It was posted on 2026-11-17 (the first rationale's §8, "Reply
  (posted 2026-11-17)"; the issue's third comment), so nothing is edited there.
  Whether a follow-up comment goes on the issue once 0.7.2 is released is the
  maintainer's; this text may serve it:

  ```markdown
  > **The sort header** (0.7.2). Breeze fills a column header's section while
  > the pointer is on it and while you press it: the button colour moved a fifth
  > towards the hover colour, and towards the focus colour when pressed.
  > native-theme now has both colours, read from your colour scheme, and the
  > colours of a selected row in a window that is not active, Breeze's window
  > colour tinted towards the selection under the window text. No table of the
  > GUI libraries native-theme supports shows these states by itself,
  > gpui-component's included, so they are for a table you draw yourself: in
  > GPUI, `native_theme_gpui::list_header_hover_background`,
  > `list_header_active_background`, `selection_inactive_background` and
  > `selection_inactive_text_color` give them.
  ```

  It says no "yet": no upstream change is planned for a table header's states or an
  unfocused selection (the upstream requests,
  `docs/issues/0004_kde-hand-overrides-upstream-requests.md`, hold none for them).

### 7.I.4 The rationale's status line

`docs/issues/0004_kde-open-items-rationale.md`'s status paragraph, one sentence for
both steps, at the release's close-out: "Steps 6 (D8) and 7 (D7's header, D9)
implemented on `<date>` for 0.7.2, by `../archive/todo_v0.7.2_kde-open-items-steps6-7-{spec,plan}.md`."
With step 7 every step is done; the rationale stays in `docs/issues/`, as the first
rationale does, and its status line says so. (The sentence on the approval of D8, D7's
second half and D9 is D0's, written when the maintainer approves them.)

### 7.I.5 `docs/todo.md`

1. **"What Breeze draws that the model has no field for"** (`:1774-1825`): in "a header
   section's hover and pressed fills, `mix(Button, hoverColor, 0.2)` and `mix(Button,
   focusColor, 0.2)` (`breezestyle.cpp:6828-6837`), #d6ecf8 / #2d4655 (computed), and
   its separators" (`:1793-1795`), the fills go: "a header section's separators". After
   the entry's last line: "*Issue #4, open items step 7 (0.7.2, <date>, <sha>):* the
   header section's hovered and pressed fills are `list.header_hover_background` and
   `header_active_background` (`kde-breeze.toml` `[light.list] header_hover_background`,
   `[dark.list] header_hover_background`), drawn by the showcases' tables; the parked
   trace's K1.177." The entry stays: its other items are open.
2. **"The selection of a view without the focus, KDE and macOS, and its missing text
   colour"** (`:1914-1933`): closed whole; it moves to `docs/archive/todo_closed.md`
   under its section, with "Done in 0.7.2 (`<sha>`): `defaults.selection_inactive_text_color`;
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
   none of it; since 0.7.2 the KDE reader writes the unfocused selection pair before
   those effects (`docs/issues/0004_kde-open-items-rationale.md` D9). The arithmetic
   exists on `v0.8.0-rc1` at `78ee0efd`, written in a clean room and checked against KDE's code:
   `native-theme/src/platform/kde_effects.rs` (87 lines, with `kde_color_common.rs`,
   which 0.7.2 carries) and its table `native-theme/tests/vectors/kde_effects.tsv`
   (367 lines, sha256 `653da4a6335be39dee89cd244e79c99c0ff6d0a22f26fc2771e610a3dd6d21fb`).
   Decide whether and how the model carries an inactive window's palette as a whole
   (every colour of an unfocused window, on KDE and on the platforms that dim one), one
   item, not a patch on one field."
4. **"Selection: the text colour macOS pairs with the unemphasised selection"**
   (`:543-580`): answered (decision 17). A dated line: "*Issue #4, open items step 7
   (0.7.2, <date>, <sha>):* the label of an unemphasized selected row does not adapt and
   draws in `labelColor`, #000000d8 / #ffffffd8, measured on macOS 15 and 26 (os-probe
   runs 42215651196 and 41195846264, carried into `docs/evidence/os-probe/` by 0.7.2);
   the macOS presets and reader state it as `defaults.selection_inactive_text_color`." The
   entry closes to `todo_closed.md`. Its suggestion that egui's plugin swap the pair on
   focus loss is recorded as not done: no toolkit's widget tells the state, and D9 offers
   the pair through accessors.
5. **"Model: the accent a control is filled with"** (6.H.5): untouched.

### 7.I.6 `docs/archive/`

At the close-out, this specification and its plan move to `docs/archive/` with the
caret's documents, and the v0.7.2 row of `docs/archive/README.md` gains them, inserted
at the end of the row's second cell, before its closing ` |` (the caret document's E.6
names the same text for its files; it is written once):

```
; `todo_v0.7.1_caret-blink-rationale.md`, `todo_v0.7.2_caret-blink-{spec,plan}.md`: the text caret's blink; `todo_v0.7.2_kde-open-items-steps6-7-{spec,plan}.md`: steps 6 and 7 of the open KDE items (the default button; the table header's states and the unfocused selection); their rationale stays with the issue
```

The links between the documents are rewritten for the new place.

0.7.2 is the last release of the batch the working rules were written for (0.7.1; the
KDE steps 2 to 7, the icons and the caret, all in 0.7.2), so the same commit archives
the batch's rationale too, `docs/todo_v0.7.1_batch-speed-rationale.md` (the batch's
rule 22 (a)): moved with its status line set, named in the v0.7.2 row after the text
above ("; `todo_v0.7.1_batch-speed-rationale.md`: the working rules of the batch of
releases 0.7.1 and 0.7.2"), and every path mention of it outside `docs/archive/`
rewritten by that rule (one is in `docs/issues/0004_kde-open-items-rationale.md`). The
same rule rewrites the path mentions of the other documents the commit moves; one of
the caret rationale's is in that file too (its list item "The caret's blink").

## 7.J Order, gates, close-out

### 7.J.1 The units

Step 7's units follow step 6's on the same branch and start from its tree. As in step
6 (6.I.1), the fields cannot pass a workspace gate before every connector names them,
and the showcases' pieces hold each other, so the units are built as blocks under one
gate each.

| Unit | Content | Gate after it |
|---|---|---|
| K7a — the cores | 7.A entire (the fields, the hand-written resolved field and its inheritance, every struct literal and test that names the fields, `fully_populated_variant` among them: its callers run `validate()` without `resolve()`, so without the new line they fail on the missing required field); 7.C (registry, rules, the facts rows, placed by Part F); 7.C2 (the evidence the macOS cell links); the preset values of 7.D.3 and 7.D.4; 7.D.5 (the macOS reader's one field); 7.B (the two carried files, the two tables, the declarations, `vector_tests.rs`), 7.D.2 and 7.D.6 (the KDE reader and its tests) | native-theme's tests with the gate's features, the five documentation checkers, and the macOS reader type-checked for its target (the `x86_64-apple-darwin` legs of `scripts/check_release.sh`); the connectors are red until K7b to K7d |
| K7b — gpui | 7.E (the three `UNREAD_COLOURS` entries, the accessors); 7.H's gpui showcase (the table's header sections, the swap, the Widget Info, the `HOW` lines, the tests and the explicit activation in `the_basic_tables_infos_report_what_it_draws`) | none of its own: built beside K7c and K7d |
| K7c — iced | 7.F (the two `ACCESSOR_ROWS` rows and one entry, the accessors, the parity exception, the regenerated `contract-leaves.toml`); 7.H's iced showcase (the signature changes of `list_row`, `list_row_fill`, `TableColours::of` and `table_row_style` with every caller and `drawn_by`'s arms, `header_section_style` with its `DRESSED` entry and `drawn_by` arm, the routes, the swatch, the window's state) | none of its own |
| K7d — egui | 7.G entire (the accessors, the four mapping rows, the counts, `mapping.md`, the install test); 7.H's egui showcase (`list_row`'s change of parameter with its caller, the header sections, the swap) | none of its own |
| K7e — outside the code | 7.I.1 to 7.I.5 (7.I.4's sentence and 7.I.6's archive are the close-out's) | after K7b to K7e: `check_showcase_parity.py --check-list`, the check on the built tree and the parity rehearsal (6.G); then one `scripts/check_release.sh --commit` with the GPUI Fast legs and the five documentation checkers over K7a to K7e; push; CI read (the macOS reader's field is compiled and tested in CI's macOS legs) |

The element-list edits of 7.H.1 and 7.H.2 (`docs/showcase-elements.toml`) are made by
the orchestrator before K7b to K7d start, from 7.H's text, and committed with them:
once an element is listed, every showcase must draw it and route its leaves, so each
builder's showcase tests are red until its own showcase does.

Why K7a is one unit: the registry, facts-rows and citation checkers hold the fields,
the rows and the presets together; `inheritance_rules_toml_covers_defaults_internal`
fails between the `[defaults_internal]` rule and its implementation, and
`theme_defaults_field_info_matches_baseline` between the field and its baseline entry;
`kcu_tint` is dead code until the reader calls it (clippy `-D warnings` fails a private
function no build uses; the allow of 7.B.3 covers `kde_color_common` only), and
`the_readers_inactive_selection_is_the_tables` calls the reader's rule; the `Scheme`
pair of 7.D.6 with its three full literals and the accent test's `selection` are needed
for `kde/mod.rs`' test module to compile. Why the connectors wait for it and for each
other's gate: egui's leaf count counts the two header leaves from the field on as
`null`, and the new defaults leaf, which resolves to a colour on every theme; egui's
T2 is satisfied once kde-breeze and the macOS presets state the leaves; gpui's and
iced's colour-leaf walks and the parity script fail until each connector names the
leaves. Why each showcase is with its connector: iced's
`styles_cover_every_widget_shown` and
`every_hand_colour_route_names_a_function_that_reads_it` read the showcase and the
styles together, and the gpui and egui showcases' route tests read the list and the
code together.

Inside the units: 7.C with or before 7.D.3-7.D.4; 7.D.3-7.D.4 before the iced leaves
file is regenerated; 7.I's backlog with or after 7.D.3; 7.I.6 at the close-out.

The capture check of 7.H.2 is no part of a unit. The session runs it first after
step 7's push, on captures of the screenshots workflow dispatched on that head, which
needs no desktop (7.H.2, *When*): a capture-code fix then lands before the close-out.
It runs again in the release's asset run, on the captures that are committed (7.J.4).

Inside the units: K7e's texts are this document's, so it is written beside the block;
it is committed after it, the orchestrator holding its counts against the block's tree.

### 7.J.2 The citations the edits move

As 6.I.2: none is re-pointed (S6). The edits of 7.A, 7.B.3, 7.C.2, 7.D.5, 7.E, 7.F, 7.G
and Part F move lines other files cite by number, among them the gpui `colors.rs`
comment at `:445`, which cites `docs/inheritance-rules.toml:245`; those citations are
left as they are and may drift. No pinned line moves (*Lines that must not move*:
`colors.rs` is not edited), and `contract-leaves.toml` is regenerated by its command
for 7.F's own rows.

### 7.J.3 Gates

6.I.3's, and: the carried vector tests and `the_readers_inactive_selection_is_the_tables`
(7.B.4) in `cargo test -p native-theme --features "$NT_FEATURES"`; the macOS reader in
CI's macOS legs; `inheritance_rules_toml_covers_defaults_internal`.

**Not covered by any gate:** as 6.I.3; and the reader's exactness for schemes beyond the
two tables' rows, the swap in each showcase while unfocused (7.H.2), and the macOS
reader's new read outside CI.

### 7.J.4 The close-out

0.7.2's close-out, made again once after K7e, for the caret's work and both steps. The
branch went through it once (`478cdab7` archived 0.7.2's earlier documents, `b494d6ed`
holds the captures and both stamps), and there is no version step: the workspace is at
0.7.2 since `b434b9bc`, and the CHANGELOG's link `[0.7.2]` is in its list (`526cce51`).
In order:

1. **Before it.** Every review is closed. The new facts cells' links of the caret
   document's A.8 are opened (its E.6). The stamped paths are searched for the stems of
   the documents this close-out moves and the batch rationale's (the batch's rule 22
   (c), its command in 0.7.2's record): expected, nothing.
2. **The archive** (7.I.6, which holds the caret documents' row text too, with 7.I.4's
   sentence, the links of the caret document's E.6 and the batch's rationale), with the path rewrites and their check of the batch's rule 22
   (a); the five documentation checkers.
3. **The compatibility run** (`scripts/update_compatibility.sh run`, which writes
   `docs/COMPATIBILITY.toml` and the connector READMEs' *Verified against* lines; they
   stay uncommitted until item 6), then the push.
4. **The full `scripts/check_release.sh`**, while CI runs on that push. The asset run
   starts when both are green, the full check's only failure the visual assets' stamp,
   which the asset run refreshes (the batch's rule 9; so it was on `da89b4ba`, 165 of
   169 steps passed, three known warnings). Its semver step reports the release's
   break, a warning the CHANGELOG names.
5. **The asset run** (`scripts/generate_assets_release.sh`), which needs the
   maintainer's desktop unlocked and untouched for 4 to 6 minutes, once. Before its
   assets are committed, the session reads the Linux captures' parity list (for the
   Basic page's new button and for a `fill` exception; none is planned), looks at the
   captures (the default button under kde-breeze; a focused input's caret; the
   screenshots workflow's captures under macos-monterey, macos-tahoe and windows-11),
   and runs 7.H.2's capture check on the twelve macOS captures. Where that check finds
   an inactive window, the capture-code change of 7.H.2 is made and reviewed, gated
   `--commit` (whose `x86_64-apple-darwin` clippy and strict-panic legs lint the macOS
   branch from Linux) and committed as a commit of its own, and the close-out resumes
   as the batch's rule 15 says for a fix that touches a stamped path: the stamps hash
   `tools/showcase-support/src`.
6. **The assets' commit**: the captures, both stamps, the compatibility run's files and
   the CHANGELOG heading's date; then the full check again on that commit, and the
   push with its CI run read. It is the release's last commit.

The merge of the pull request (#11), the tag `v0.7.2`, the publication and the GitHub
release are the maintainer's.

## What was not verified

- Compiled or run before the build: the gpui default button's variant (T-K1), the Basic
  page's layout (T-K2), gpui's test windows made active and inactive (T-K3) and the
  carried modules under this tree's lints (T-K4; the head). Not compiled or run: the
  connectors' other new code, the contract walks' results with the new leaves, the
  derive's output for the new soft options.
- The Basic page's layout with the default button on the platforms' own fonts: the
  trial T-K2 measured it headless on Linux (decision 7); the macOS and Windows captures
  of the asset run show it there.
- Clippy's `too_many_arguments` threshold of seven (7.H.2) is clippy's default as
  documented, not read in clippy's source here; the showcase's `tab_bar`, with eight
  parameters and the lint allowed (`demo.rs:1915-1925`), agrees with it.
- The new preset comments (6.C.3, 7.D.3, 7.D.4) take the batch's form,
  `# platform-facts.md (§2.x): …`, which `check_preset_citations.py` reads (run on a
  copy of 0.7.2's presets with 6.C.3's blocks on 2026-11-19: exit 0); the units run it
  on their trees.
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
- The committed macOS captures say, very likely, that 0.7.2's windows were active
  (7.H.2, *The baseline*; the grey buttons of a non-key window are general macOS
  behaviour, not checked against a source here). Not verified: whether later runs stay
  so, which the capture check reads after step 7's push and again in the asset run; how the capture code
  (`FrameCapture::start` or an earlier `poll`, 7.H.2) would make the window active on
  a runner, if it must.
- egui's per-state default button (6.G) rests on the previous frame's response, read
  from the page's memory; that egui keeps no other way to know a button's state before
  it is built was not looked for beyond `Button`'s own API.
- `VisualTestContext::deactivate_window` after `Window::activate_window` in one test
  (7.H.2) was run by the trial T-K3 on both backends; the showcase's own harness
  (`open_with`) was not, and K7b's tests show it.
- The `check_section_refs.py` and `check_todo_citations.py` results on the new texts
  (not run).
