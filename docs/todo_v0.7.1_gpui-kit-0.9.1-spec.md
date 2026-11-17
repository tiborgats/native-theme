# GPUI Kit 0.9.1 in v0.7.1: what the port left undone — specification

- Rationale: [`todo_v0.7.1_gpui-kit-0.9.1-rationale.md`](todo_v0.7.1_gpui-kit-0.9.1-rationale.md)
  (N1–N9). This document says what changes, where, to which text or value, with
  which tests and gates. It decides nothing the rationale did not decide, except
  the small points under *Decisions taken while specifying*; what needs a
  decision of the maintainer is under *Open points*.
- Plan: `todo_v0.7.1_gpui-kit-0.9.1-plan.md`, written from this specification.
- Written: 2026-11-14, against `v0.7.1-rc2` at `0fa28027`. Every repository
  `file:line` is that commit's unless it names another; every upstream line is
  read in Cargo's registry at the versions below.
- Status: **to be implemented once free of errors** (the maintainer, 2026-11-15:
  "When the plan that goes in the 0.7.1 version is ready, without errors, start
  implementing it."), with its plan.
- The batch's working rules are cited as "rule N (batch-speed rationale,
  Appendix)": `docs/todo_v0.7.1_batch-speed-rationale.md`, *Appendix — the
  batch's working rules*, authoritative where this text differs.

**Abbreviations.** As in the rationale: `GC`, `GB`, `GP`, `GF` are the `src/`
folders of gpui-component 0.9.1, gpui-base 0.9.1, gpui-pre 0.3.12 and gpui-fast
0.1.15 in `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`; `CN` is
`connectors/native-theme-gpui`; `SC` is `CN/examples/showcase-gpui`. Commands
are run from the worktree's root, `/home/tibi/Rust/native-theme/.claude/worktrees/v071`.

## How to read it

| Part | Rationale | What it changes |
|---|---|---|
| A — The showcase's new parts | N2 | `SC/pages/layout.rs`, `SC/pages/inputs.rs`, `SC/demo.rs`, `SC/support.rs`, `SC/app.rs`, `SC/info/layout.rs`, `SC/info/inputs.rs`, `SC/info/typography.rs`, `SC/tests.rs` |
| B — Statements | N3 | `SC/info/chrome.rs`, `SC/info/mod.rs`, `SC/info/layout.rs`, `SC/info/icons.rs`, `CN/src/icons.rs`, `CN/src/widgets/checkbox.rs` and `tests.rs`, `CN/src/geometry.rs`, `CN/src/config.rs`, `CN/src/lib.rs`, `CN/README.md`, `docs/showcase-exceptions.toml` (header), `docs/issues/0004_kde-hand-overrides-upstream-requests.md` |
| C — The Combobox workaround | N4 | `SC/support.rs`, `SC/tests.rs` (a comment) |
| D — The backlog and the two pins | N5 | `docs/todo.md`, `docs/archive/todo_closed.md`, `ROADMAP.md` (upstream list), `Cargo.lock` |
| E — Citations | N6 | the files of E.2's scope, digits first, then by hand; a copy of the 0.8.1 scripts outside the repository; by hand also `scripts/check_showcase_parity.py`, `connectors/native-theme-iced/src/styles.rs`, `CN/proposals/README.md`, and the taffy citations after D.6 |
| F — The coverage gate | N7 | `scripts/check_widget_coverage.py`, `docs/showcase-exceptions.toml` (`[gpui]`), `scripts/README.md` (F.7), `docs/todo.md` (F.3's dated line) |
| G — CHANGELOG, ROADMAP, notes, assets | N8 | `CHANGELOG.md`, `ROADMAP.md` (the v0.7.1 paragraph and `:264`'s count), `docs/todo_gpui-full-theme.md` (a new note), the release assets and both stamps |
| H — The editor's status colours | N9 | `docs/todo.md` (one new section) |
| I — Facts rows found by text | batch-speed rationale, S6 | `native-theme/tests/platform_facts_xref.rs`, `native-theme/src/presets/documented_sizes.rs`, `scripts/check_preset_citations.py`, a comment and a step label in `scripts/check_release.sh`, a comment in `.github/workflows/ci.yml`, `scripts/README.md`, a clause of `docs/todo.md`, `CHANGELOG.md` (one line) |

**Constraints that hold for every part** (as in the 0.8.1 specification, §0.2):

- **No panic paths and no `unsafe`.** No `unwrap`, `expect`, `panic!`,
  `unreachable!`, slice indexing or unchecked arithmetic outside tests. The
  project's hook (`.claude/hooks/no-runtime-panics.sh`) refuses an edit whose new
  text holds `.unwrap()`, `.expect(`, `panic!(`, `unreachable!(`,
  `unimplemented!(`, `todo!(` or `unsafe` unless the text holds `#[test]`,
  `#[cfg(test)]` or `#[allow(clippy::unwrap_used`; `#[gpui::test]` is not
  `#[test]`. A failure in test code is an `assert!`. The hook is never worked
  around.
- **Both cores.** Every gpui change is tested on gpui-pre
  (`--features native,watch`) and on GPUI Fast (`--features gpui-fast,native,watch`),
  and clippy also runs with `--no-default-features`.
- **Shared files are located by content.** Every line number below is
  `0fa28027`'s. A part that edits a file another part edited first finds its
  place by the quoted text; where the text no longer stands, it stops and
  reports.
- **The dry-run figures** (51, 64, 65, 117, 4 chains, and the set-asides named in
  E.5) come from the reviewers' dry runs of 2026-11-14 on copies of the scripts
  with E.1's constants, over the tree at `0fa28027`. The implementer's run at E.4
  produces its own figures; a difference is expected where the tree has changed
  since.
- **Part B's "Present" texts are `0fa28027`'s.** Part E's run comes first
  (decision 3), so an upstream line number inside a quoted present text may
  already be E's when Part B runs. Where that is so, the edit is anchored on the
  quoted text's first and last words and replaces the whole quoted span whatever
  its digits (B.9), or the line is a set-aside of E's run that Part B's edit
  resolves (B.2). Checked against the dry run of E.1's copy (its `plan.json`,
  read 2026-11-14): of the present texts quoted in Parts A to D and G, only B.2's
  (`CN/src/icons.rs:602`, set aside) and B.9's (`CN/src/geometry.rs:1435`,
  `588-590` renumbered to `596-598`; `:1436`'s `667` kept) hold digits the run
  touches; B.5's README lines are E.3's (left alone); the rest are in files out of
  E's scope (`SC/info/`, `docs/`, `CHANGELOG.md`, `ROADMAP.md`) or hold no line
  number.
- **Lines that must not move.** The showcase's colour claims cite lines of the
  connector, and `every_colour_claim_is_read_at_the_line_it_cites`
  (`CN/src/showcase.rs:2819`) fails when a cited line moves. At `0fa28027` they
  pin, in `CN/src/` (the range, and the file's **last pinned line**):
  `variants.rs` 62–64 (64), `geometry.rs` 230–772 (772), `colors.rs` 281–718
  (718), `widgets/checkbox.rs` 123–145 (145), `widgets/mod.rs` 344 (344),
  `widgets/progress.rs` 77–79 (79), `widgets/separator.rs` 30 (30),
  `widgets/slider.rs` 77–80 (80), `widgets/spinner.rs` 93–466 (466),
  `widgets/switch.rs` 94–144 (144) and `widgets/tab_bar.rs` 217–239 (239). No
  edit of Parts A to D is above or inside one of these ranges; B.13's edit of
  `widgets/checkbox.rs` is at its line 430 and must not change the file's `use`
  items (rustfmt would reflow them above line 123). Part E's hand-read can be:
  the dry run sets aside or leaves to the hand-read chains above the last pinned
  line in `geometry.rs` (51 set-aside chains, the first at `:67`), `colors.rs` (`:220`, `:359`),
  `widgets/mod.rs` (`:7-9`, `:304`) and `widgets/tab_bar.rs` (`:123`). E.5's rule
  for those files keeps their line counts.
- **`docs/todo.md` conventions** (its header, lines 1-21; the header of
  `docs/archive/todo_closed.md`, lines 1-13). A closed item moves to
  `todo_closed.md` verbatim, its box ticked, with a dated line
  `  *Closed <date> in <sha>:* …` (two-space indent); a `###` section whose every
  item is done moves whole under its `##` heading; an item from a section that
  keeps open items goes under `### Closed items from <heading>`, at the end of
  that group if it exists. `<date>` and `<sha>` are filled in at commit time. An
  entry is cited by its heading and opening words.
- **New Widget Info citations name their crate** (`gpui-component/…`,
  `gpui-base/…`): `every_prose_citation_still_exists` (`CN/src/showcase.rs:3691`)
  reports a bare file name that two cited crates hold with the symbol as
  ambiguous (`symbol_citation_finding`, `:3392-3417`). Every upstream citation
  this specification adds to a source file, a doc comment, a script's comment or
  a Widget Info text names its crate, even where its file is unique among the
  cited crates; a citation a rewritten text keeps from its present text (B.1's
  `menu/popup_menu.rs, RenderOptions`, A.3's three in the `alphas` note, B.4's and
  B.8's) stays as it is. The backlog's dated lines and closing lines name the
  crate once per line, as their entries do; the `[gpui]` reasons of
  `docs/showcase-exceptions.toml` follow that file's header (a bare path is
  gpui-component's, a gpui-base path is named). A
  `file.rs:N-M, Symbol` pair is read as "the range holds the symbol"
  (`line_citations`, `:3320-3346`; `line_citation_finding`, `:3482-3487`), so
  every such pair of the new texts was checked against the 0.9.1 source on
  2026-11-14: `gpui-component/grid_layout.rs:67, dragging_item`,
  `gpui-base/grid_layout/grid_layout.rs:22, RESIZE_HANDLE_SIZE`,
  `gpui-base/text/node.rs:42-43, CHECK_SVG_LIGHT`, and A.2's
  `speech/waveform.rs:192-204, liquid; :26, CREST_RATIO; :30, CREST_SOFTNESS;
  :33, LINE_TAPER; :41, WATER_OPACITY` each hold their symbol.

## Decisions taken while specifying

Each changes no result of the rationale; each is the smaller or the safer choice.

1. **E.** The copy of the scripts goes to
   `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/citations/`,
   the folder convention of the earlier plans there
   (`todo_v0.6.0-rc1_gpui-kit-0.7-plan/`, `todo_code-review-plan/g4/t2/`).
2. **E.** `SC/info/` ("done", rationale N6) is kept out of the scripts' scope by
   an exclude pathspec, so the run cannot touch the lines the port re-pointed.
3. **E before A–D.** The renumbering runs first, on the tree the plan starts
   from, so every later part writes 0.9.1 lines directly and none of its lines
   joins the hand-read.
4. **F.** Three widgets the rule finds are drawn by the showcase today through a
   free function or an extension method, which the gate does not count:
   `ResizablePanelGroup` (`h_resizable`, `SC/app.rs:2350`), `ResizablePanel`
   (`resizable_panel`, `SC/app.rs:2315`, `:2343`) and `Scrollbar`
   (`vertical_scrollbar`, `SC/support.rs:358`; `overflow_y_scrollbar`,
   `SC/demo.rs:7004`). They get `GPUI_VIA` entries beside `grid_layout`'s. An
   exception would state that the showcase does not render them, or that each
   is an internal part the showcase draws only as part of another widget, as
   F.4's `ResizeHandle` is (which has a Widget Info of its own,
   `SC/info/chrome.rs:1213`); neither holds for these three, each of which a
   call of the showcase's own builds.
5. **F.** The re-export rule pairs a `pub struct` with a render impl or derive
   **in the same gpui-base file**. That is how it follows the path rather than
   the name (rationale N7); all nine widgets it finds today meet it (read with
   the script's own regexes, 2026-11-14), and it tells apart gpui-base's two
   `Text`s and its two `Collapsible`s.
6. **F.** `ScrollableMask`, `VirtualList` and `PlotAppearScope` are excepted as
   wrappers with no visual surface; `ResizeHandle` and `PlotElement` as parts of a
   compound widget. `PlotAppearScope`'s reason says both that it belongs to the
   charts' appear animation (rationale N7, "as parts of the charts") and that it
   paints only its child, which is what its source shows.
7. **A.** The liquid waveform goes through the existing helper with a
   `WaveformKind` argument, and the bars' Widget Info gains the variant "bars",
   so the two infos differ by variant, as the showcase's other pairs do.
8. **A.** The `Diff` decoration is on the modified side's line 7, `let system =
   SystemTheme::from_system()?;`: a line both sides keep (so the tint lies over no
   change fill), the third code row. Line 13, `let resolved = system.pick(mode);`,
   is the twelfth code row, which by a count of rows (not measured) lies below the
   Diff's 240 px at rest; A.5's second test fails if line 7 is not painted.
9. **A.** Sample content (the grid's three cards, the task list's two items) is
   the showcase's own, like `CAROUSEL_SLIDES` and `STEPPER_STEPS`.
10. **D.** A still-holding entry's dated line opens `*GPUI Kit 0.9.1 (<date>):*`,
    the form `docs/todo.md:4014` already uses. `:2742`, written at 0.9.1 on
    2026-11-13, gets one too, saying so, as the rule "an entry that still holds
    gets one dated line" asks.
11. **G.** The coverage gate's new discovery gets one `**Development**:` line in
    the 0.7.1 entry's *Added* (G.1), by the precedent of 0.7.0's lines for the
    same script (`CHANGELOG.md:198`, under *Added*; `:487`, under *Fixed*).
12. **E.** gpui-kit and gpui-kit-assets stay in the scripts' `FAMILY` though N6
    leaves them out of scope. It is harmless: their `src/` folders are identical
    in 0.8.1 and 0.9.1 (`diff -rq`, 2026-11-14), so a chain the scripts give them
    is decided "unchanged file" and keeps its number (a reviewer's dry run with
    E.1's copy: 4 chains); and it keeps the attribution of an unnamed path as the
    0.8.1 run made it, where a gpui-kit file competes with the others.
13. **E, D.** taffy's citations follow the lock: if D.6 moves taffy, the three
    citations of 0.15.0 lines are re-read at the version reached (E.5); no
    citation of `unicode-properties`' source exists outside `docs/todo.md`
    (`git grep -n unicode-properties`, 2026-11-14).

## Corrections to the rationale found while specifying

None that changes a decision. One reading to add: besides the files the
rationale names, the egui and iced showcases cite 43 lines of the gpui
showcase's own files (`grep -rnE 'showcase-gpui/[a-z_/]+\.rs:[0-9]+'` over
`connectors/native-theme-egui`, `connectors/native-theme-iced`, `tools`,
`scripts`, `docs/showcase-exceptions.toml`), which Parts A to C move. The 0.8.1
work re-pointed such citations; under the maintainer's S6
(`docs/todo_v0.7.1_batch-speed-rationale.md`) they are left as they are and may
drift, and Part I makes the three gates that read facts lines by position stop
doing so.

## Open points for the maintainer

1. **What else lands in v0.7.1.** Two things here wait for it. (a) When the
   asset run happens (G.4): N8 puts it after the last change of all four
   documents' work that lands in v0.7.1, and which of the icons, caret and KDE
   open-items documents land in v0.7.1 is decided in those documents. (b) Settled
   by the session: B.3 lands unless the icons work lands in v0.7.1 — so under the
   icons rationale's W11 (b) (its section *W11 — Where it lands*), when the icons
   rationale is not approved when the work starts, and when W11 (a) is chosen but
   its work does not land in v0.7.1 (rationale N3: "Either way the false sentence
   goes"). When the icons work lands in v0.7.1 it writes those texts itself and
   B.3 does not land. B.3's texts name no document of this cycle (the doc comment
   names the backlog entry D.3 writes, by its heading), so they hold in every case
   where they land. Settled by the session: this port's work
   lands before the KDE step 1 specification's
   (`docs/todo_v0.7.1_kde-open-items-step1-spec.md`), and one asset run follows
   both (*Order and gates*).
2. **Part C's second outcome.** If `the_theme_settings_switch_the_preset` fails
   without the override, the comment's and the backlog entry's new text depend
   on what the failure shows; they cannot be written in advance. The task then
   stops after restoring the override, reports the failure and the reading of
   `GC/combobox.rs` it leads to, and the text is reviewed before it is
   committed.

---

## Part A — The showcase's new parts (N2)

Four parts and one stated hover. All are on gpui-only pages, so nothing is
mirrored elsewhere: `docs/showcase-elements.toml` lists only the chrome and the
Basic page the three showcases share (its header, lines 80-122, "Shared elements
only"), `scripts/check_showcase_parity.py` compares only those, and
`scripts/check_connector_parity.py` compares the connectors' `pub fn`s, of which
this work adds none.

### A.1 `GridLayout` on the Layout page

**Upstream.** `grid_layout(id, &Entity<GridLayoutState>, &App) -> GridLayout`
(`GC/grid_layout.rs:40-105`), gpui-component's skin over gpui-base's `GridLayout`
(`GB/grid_layout/grid_layout.rs:56`), re-exported whole (`GC/grid_layout.rs:14`).
`GridLayoutState::new(cx)` is 12 columns, compacting vertically, draggable and
resizable (`GB/grid_layout/state.rs:176-190`); `.default_areas(…)` takes one
`GridArea::new(id).col(c).row(r).col_span(w).row_span(h)` per item
(`GB/grid_layout/layout.rs:60-98`; `state.rs:229-233`). `GridItem::new(id)` is
`Styled` and `ParentElement` (`GB/grid_layout/grid_layout.rs:195-240`). The row
height is 2.5 rem unless set (`:80`, `:109-114`).

**`SC/support.rs`**, after `STEPPER_STEPS` (`:99-104`):

```rust
/// One card of the Layout page's `GridLayout`: its id, which is the item's,
/// its area's and its card's selector, its label, and its area in grid cells.
#[derive(Clone, Copy)]
pub(crate) struct GridCard {
    pub(crate) id: &'static str,
    pub(crate) label: &'static str,
    pub(crate) col: usize,
    pub(crate) row: usize,
    pub(crate) col_span: usize,
    pub(crate) row_span: usize,
}

/// The cards of the Layout page's `GridLayout` in its 12 columns: two side by
/// side over one as wide as both.
pub(crate) const GRID_LAYOUT_ITEMS: [GridCard; 3] = [
    GridCard { id: "layout-grid-item-one", label: "One", col: 0, row: 0, col_span: 6, row_span: 2 },
    GridCard { id: "layout-grid-item-two", label: "Two", col: 6, row: 0, col_span: 6, row_span: 2 },
    GridCard { id: "layout-grid-item-three", label: "Three", col: 0, row: 2, col_span: 12, row_span: 2 },
];
```

A named struct, not a tuple: the tuple type `[(&str, &str, (usize, usize,
usize, usize)); 3]` would draw clippy's `type_complexity` (on by default; nothing
in the workspace raises its threshold, `CN/clippy.toml` has none, and
`SC/tests.rs:14069` already needs an allow for it), which the release check's
clippy legs deny (`scripts/check_release.sh:529-550`, `-D warnings`). The score
of the tuple type was a reviewer's, not computed here. Every other new item of
Parts A and B was re-read for the same lint and for the strict panic lints
(`scripts/check_strict_panic.sh:77-91`: no indexing, slicing, arithmetic or
`unwrap` in non-test code): `WaveformKind` (an enum), `grid_layout_sample` and
`speech_waveform` (five parameters each, of named types),
`DIFF_DECORATED_PATH` and `DIFF_DECORATED_LINE` (`&str`, `usize`), the Widget
Info functions (`&Theme` and an enum) and B.13's call hold none; the arithmetic of
A.5's first test is test code, which those lints do not build.

**`SC/app.rs`:**

- a field after `folder_tab` (its doc and field, `:522-523`), under `// Layout page`:
  `/// The Layout page's GridLayout: GRID_LAYOUT_ITEMS's areas, which a drag or a
  resize changes.` `pub(crate) grid_layout_state: Entity<GridLayoutState>,`
- built after `carousel_state` (`:1553`):
  `let grid_layout_state = cx.new(|cx| GridLayoutState::new(cx).default_areas(GRID_LAYOUT_ITEMS.map(|card| GridArea::new(card.id).col(card.col).row(card.row).col_span(card.col_span).row_span(card.row_span))));`
- `grid_layout_state,` in the struct literal after `folder_tab: 0,` (`:1708`);
- `use gpui_component::grid_layout::{GridArea, GridLayoutState};` and
  `GRID_LAYOUT_ITEMS` added to the `crate::support` import.

**`SC/demo.rs`**, a new helper after `settings` (`:7713-7784`), before the
Overlays banner (`:7786`):

```rust
/// The Layout page's `GridLayout` over `state`, `width` wide: gpui-component's
/// `grid_layout`, its gap, motion, rings, placeholder and grip upstream's
/// (gpui-component grid_layout.rs:40-105), holding GRID_LAYOUT_ITEMS as cards
/// framed as the showcase frames every demonstration.
pub(crate) fn grid_layout_sample(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<GridLayoutState>,
    width: Pixels,
) -> Stateful<Div>
```

Its body: `grid_layout(id, state, cx).w(width).items(GRID_LAYOUT_ITEMS.map(…))`,
each `card` becoming `GridItem::new(card.id).rounded(cx.theme().radius).child(div()
.size_full().demo_frame(cx).bg(cx.theme().background).flex().items_center()
.justify_center().text_sm().child(card.label).debug_selector(move || card.id.into()))`
— rounded by the
card's radius, so the selection ring follows the card's corners as upstream's
example asks (`GC/grid_layout.rs:36-39`) — then
`.info(ui, id, info::layout::grid_layout(cx.theme()))`, `.self_start()` and
`.debug_selector(move || id.into())`, as `demo::carousel` ends (`:7543-7554`).
`use gpui_component::grid_layout::{GridItem, GridLayoutState, grid_layout};`, and
`GRID_LAYOUT_ITEMS`, which the body's `.items(…)` reads, added to `SC/demo.rs`'s
`crate::support` import (`:112-116`); A.3's `DIFF_DECORATED_PATH` and
`DIFF_DECORATED_LINE`, which `diff`'s `.decorations(…)` reads, join the same
import. The name `grid_layout_sample` keeps the free function `grid_layout(` for upstream's
call, which F.3's pattern counts.

**`SC/pages/layout.rs`**, after the Settings section (`:501-515`), the page's
last:

```rust
.child(demo::heading(ui, cx, "layout-heading-grid-layout",
    "GridLayout (drag a card, or resize it by its corner)"))
.child(demo::grid_layout_sample(ui, cx, "layout-grid-layout",
    &self.grid_layout_state, px(480.0)))
```

(480 px as the Stepper's column, `:450`.)

**Widget Info, `SC/info/layout.rs`**, a new `pub fn grid_layout(t: &Theme) ->
WidgetInfo` after `settings` (`:1499-1513`), built from `WidgetInfo::new("GridLayout")`
with no variant (the kind A.5's fifth test expects):

| Kind | Role / what | Value | Cited at |
|---|---|---|---|
| claim | `selected card's ring, 2px` | `primary`, `t.primary` | `gpui-component/grid_layout.rs:51` |
| claim, `t.focus_ring` true | `keyboard's card, ring at 50%, 3px` | `ring`, `t.ring.alpha(0.5)` | `gpui-component/grid_layout.rs:58` |
| claim, `t.focus_ring` false | `keyboard's card, ring, 1px` | `ring`, `t.ring` | `gpui-component/grid_layout.rs:61` |
| claim | `drop placeholder, primary at 8%` | `primary`, `t.primary.opacity(0.08)` | `gpui-component/grid_layout.rs:79` |
| claim | `drop placeholder's dashed edge, primary at 50%` | `primary`, `t.primary.opacity(0.5)` | `gpui-component/grid_layout.rs:82` |
| claim | `resize grip, at 35%` | `muted_foreground`, `t.muted_foreground.opacity(0.35)` | `gpui-component/grid_layout.rs:86` |
| claim | `resize grip under the pointer, at 90%` | `muted_foreground`, `t.muted_foreground.opacity(0.9)` | `gpui-component/grid_layout.rs:86` |
| config `border-radius` | `radius_lg: {px}px, the drop placeholder's corners` | `px_text(t.radius_lg.as_f32())` | — |
| config `focus_ring` | `{t.focus_ring}` | — | — |
| not_themeable `gap` | `{px}px, spacing_tokens().md: Theme::spacing_tokens returns SpacingTokens::default() with no field behind it, so no theme reaches it (gpui-component/grid_layout.rs:65; gpui-component/theme/mod.rs:596-598; gpui-base/theme_tokens.rs:146-158)` | `px_text(t.spacing_tokens().md.as_f32())` | — |
| not_themeable `motion` | `cards settle on Theme::motion's duration_normal and easing_move (gpui-component/grid_layout.rs:66); ResolvedTheme has no motion field, and apply keeps the application's` | — | — |
| not_themeable `dragged card` | `shadow_lg, two black layers at 10%, gpui's literal (gpui-component/grid_layout.rs:67, dragging_item)` | — | — |
| not_themeable `rings` | `both are the card's shadow, so the focus ring replaces the selection ring while the keyboard drives the grid, and a read-only grid shows no selection (gpui-component/grid_layout.rs:28-31, :68-73)` | — | — |
| not_themeable `placeholder` | `a 1px dashed edge round the cells the dragged card will take (gpui-component/grid_layout.rs:74-84)` | — | — |
| not_themeable `resize grip` | `a 6px corner mark 3px in from the card's corner, two 2px edges (gpui-component/grid_layout.rs:94-99), in a 16px hit corner (gpui-base/grid_layout/grid_layout.rs:22, RESIZE_HANDLE_SIZE): literals` | — | — |
| not_themeable `row height` | `2.5 rem unless the caller sets one, a literal (gpui-base/grid_layout/grid_layout.rs, GridLayout::row_height)` | — | — |
| instance `cards` | `{n} of the showcase's own cards in 12 columns (GRID_LAYOUT_ITEMS), each framed and rounded as the showcase frames a demonstration (demo_frame)` | `GRID_LAYOUT_ITEMS.len()` | — |
| instance `editing` | `drag a card to move it and its bottom-right corner to resize it; a click selects one; with the grid focused the arrow keys move it, Shift with them resizes it, Tab moves the selection (gpui-base/grid_layout/grid_layout.rs, GridLayout)` | — | — |

Every claim's cited line names its field (checked by reading the lines above); the
three opacity claims follow the Diff panel's practice of citing the line that
reads the token (`SC/info/typography.rs:479-507`).

### A.2 The liquid `SpeechWaveform`, on the Inputs page

**Upstream.** `SpeechWaveform::liquid()` (`GC/speech/waveform.rs:85-101`): the
same colour as the bars, `primary` while capturing and `muted_foreground`
otherwise (`:136-140`); a ribbon a tenth of the height thick, at least 2 px
(`:204`), thinning to 0.25 of that at its ends (`:33`, `LINE_TAPER`); crests
reaching 0.42 of the height (`:26`, `CREST_RATIO`), flattened at 1.4 (`:30`,
`CREST_SOFTNESS`); water under it in the line's colour at 16 % at the crest,
fading to clear (`:41`, `WATER_OPACITY`; `:228-235`); advanced every frame while
capturing (`:132-135`, `:209-217`), a still ripple under reduced motion
(`:199-201`). Height by `Size`, 20 px at the default (`:103-111`); 96 px wide
unless sized (`:19`, `:148`).

**`SC/demo.rs`**, before `speech_waveform` (`:4545`):

```rust
/// How a `SpeechWaveform` draws the input: upstream's bars, or the water
/// surface `SpeechWaveform::liquid` draws across a row (gpui-component
/// speech/waveform.rs:85-101).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum WaveformKind {
    Bars,
    Liquid,
}
```

`speech_waveform` (`:4548-4555`) gains `kind: WaveformKind` after `state`; the
liquid one is `SpeechWaveform::new(state).liquid().w_full()`; both then take
`.info(ui, id, info::inputs::speech_waveform(cx.theme(), kind))` and
`.debug_selector(move || id.into())`. Its doc says the liquid one spans its row.

**`SC/pages/inputs.rs`**, the *SpeechWaveform & SpeechIndicator* section
(`:344-376`): the `h_flex` row (`:350-376`) becomes the first child of
`with_gap(v_flex(), widget_gap)`, whose second child is
`demo::speech_waveform(ui, cx, "inputs-speech-waveform-liquid", &self.speech_state, WaveformKind::Liquid)`;
the row's own call passes `WaveformKind::Bars`. The heading is unchanged.

**Widget Info, `SC/info/inputs.rs`, `speech_waveform`** (`:1774-1792`) takes
`kind: WaveformKind` and sets `.variant("bars")` or `.variant("liquid")`. The
bars keep their two claims, `bars, capturing` and `bars, idle` (`:1777-1788`), and
their notes (`:1789-1791`). The liquid one draws no bars but a ribbon and the
water under it in the same colour (`GC/speech/waveform.rs:141-144`, `:192-258`),
so it has claims of its own and, in place of the bars' `size` and `motion` notes,
notes of its own:

- claim `surface, capturing`, `primary`, `t.primary`,
  `gpui-component/speech/waveform.rs:137`;
- claim `surface, idle`, `muted_foreground`, `t.muted_foreground`,
  `gpui-component/speech/waveform.rs:139`;
- claim `water at the crest, capturing, at 16%`, `primary`,
  `t.primary.opacity(0.16)`, `gpui-component/speech/waveform.rs:137`;
- claim `water at the crest, idle, at 16%`, `muted_foreground`,
  `t.muted_foreground.opacity(0.16)`, `gpui-component/speech/waveform.rs:139`;
- not_themeable `size`: "20px tall at the default Size, a literal, and as wide as
  its row, the showcase's w_full in place of upstream's 96px (gpui-component/speech/waveform.rs, SpeechWaveform::height and DEFAULT_WIDTH)";
- not_themeable `surface`: "a ribbon a tenth of the height thick in the middle, at least 2px, thinning to a quarter of that at its ends; crests reach 0.42 of the height, flattened by tanh at 1.4; the water fades from 16% at the crest to clear at the bottom (gpui-component/speech/waveform.rs:192-204, liquid; :26, CREST_RATIO; :30, CREST_SOFTNESS; :33, LINE_TAPER; :41, WATER_OPACITY): literals";
- not_themeable `motion`: "while capturing the surface is advanced every frame and the window redraws; under reduced motion it holds a ripple that grows with the level (gpui-component/speech/waveform.rs:132-135, :199-201, :209-217)";
- instance `signal`: the bars' text (`:1789`).

### A.3 A `DiffDecoration` on the Typography page's `Diff`

**Upstream.** `Diff::decorations(impl IntoIterator<Item = DiffDecoration>)`
(`GC/diff/mod.rs:351-356`); `DiffDecoration::new(DiffLineRange)`, its background
`warning` at 0.2 unless set (`GC/diff/decoration.rs:19-34`; drawn at
`GC/diff/mod.rs:1766-1774`, an overlay over the change fill, `:1813-1816`; a
selected line shows the selection instead, `:1769`, `decoration.rs:7-12`).
`DiffLineRange::new(path, side, start, end)`, one-based lines
(`GC/diff/document.rs:58-85`), all re-exported from `gpui_component::diff`
(`GC/diff/mod.rs:15-16`).

**`SC/support.rs`**, after `DIFF_SAMPLE` (`:136-158`):

```rust
/// The file of DIFF_SAMPLE as `DiffFile::path` gives it, and the line of its
/// modified side the Typography page's `Diff` decorates: `let system =
/// SystemTheme::from_system()?;`, which both sides keep, so the decoration lies
/// over no change fill, and which the Diff shows at rest.
pub(crate) const DIFF_DECORATED_PATH: &str = "src/install.rs";
pub(crate) const DIFF_DECORATED_LINE: usize = 7;
```

`DiffFile::path` of `diff --git a/src/install.rs b/src/install.rs` is
`src/install.rs`, read in the parser: the Git header's paths keep their `a/` and
`b/` prefixes (`GC/diff/parser.rs:194-205`, prefixes `:62-65`), which are stripped
at `:233-240`, and the file's path is the modified side's
(`GC/diff/document.rs:445`). A.5's first test pins it.

**`SC/demo.rs`, `diff`** (`:6603-6618`): after `.hover_highlight(…)`,
`.decorations([DiffDecoration::new(DiffLineRange::new(DIFF_DECORATED_PATH, DiffSide::Modified, DIFF_DECORATED_LINE, DIFF_DECORATED_LINE))])`;
its doc adds "and one line decorated in upstream's default background". The
`gpui_component::diff` import (`:48`) gains `DiffDecoration, DiffLineRange,
DiffSide`.

**Widget Info, `SC/info/typography.rs`, `diff`** (`:408-548`):

- after the "selected text" claim (`:522-527`): claim `decorated line, warning at
  20%`, `warning`, `t.warning.opacity(0.2)`, `gpui-component/diff/mod.rs:1773`;
- the `alphas` note (`:541`) becomes, in full: "the 12% of a changed line's fill
  (diff/mod.rs, code_line: lines 1810-1812), the 30% of its changed words
  (diff/mod.rs, line_highlights: line 1570), the 35% of the hunk separator
  (diff/mod.rs, render_hunk_separator: line 1020) and the 20% of a decorated line's
  default background (gpui-component/diff/mod.rs, code_line: line 1773) are
  upstream literals over the theme's colours" (the first three citations kept as
  they are);
- a new instance `decoration`: "the showcase decorates the modified side's line
  {DIFF_DECORATED_LINE}, a line both sides keep, with a DiffDecoration of
  upstream's default background; it is painted under the text, over a changed
  line's 12% fill where a line has one, which this unchanged line has not, and a
  selected line shows the selection instead (gpui-component/diff/decoration.rs,
  DiffDecoration; gpui-component/diff/mod.rs, code_line)". Read:
  `GC/diff/mod.rs:1810-1817`.

### A.4 The Markdown task list, and the hovered table row

**Upstream.** gpui-base draws a task item's box (`GB/text/node.rs:3881-3933`):
`rems(0.9375)` square, rounded 4 px, a 1.5 px edge in the list style's text colour
or else `muted_foreground` (`:3899-3910`); checked, filled and edged in `link`
with a check mark drawn from bytes gpui-base embeds, white in a light theme and
black in a dark one (`:42-43`, `:3883-3887`, `:3911-3918`, `rems(0.75)` at
`:3917`); a done task's text in `muted_foreground` (`:3941-3944`); no list marker
beside a task (`:3857`). The box is clickable only with `on_task_toggle`
(`GC/text/compat.rs:197-205`; `GB/text/node.rs:3921-3931`). gpui-component's
style gives `muted_foreground`, `link`, `table_hover` and dark (`GC/text/mod.rs:52`,
`:53`, `:60`, `:65`) and no list style (`:50-65`). A body row of a table under
the pointer is `table_hover` (`GB/text/node.rs:4431-4435`).

**`SC/support.rs`, `MARKDOWN_SAMPLE`** (`:160-179`): before the closing `"#;`, a
blank line and

```markdown
- [x] Resolve the desktop's theme
- [ ] Apply it with `apply_system_theme`
```

No `on_task_toggle` is set (rationale N2).

**Widget Info, `SC/info/typography.rs`, `markdown`** (`:349-404`), after the
"table head text" claim (`:388-393`):

- claim `hovered table row`, `table_hover`, `t.table_hover`, `gpui-component/text/mod.rs:60`;
- claim `task box edge`, `muted_foreground`, `t.muted_foreground`, `gpui-component/text/mod.rs:52`;
- claim `checked task box, fill and edge`, `link`, `t.link`, `gpui-component/text/mod.rs:53`;
- claim `done task's text`, `muted_foreground`, `t.muted_foreground`, `gpui-component/text/mod.rs:52`;
- not_themeable `task check mark`: "white in a light theme and black in a dark one, an SVG gpui-base embeds and draws as an image, so neither a theme colour nor an icon set or resolver reaches it (gpui-base/text/node.rs:42-43, CHECK_SVG_LIGHT and CHECK_SVG_DARK; :3883-3887, :3913-3917)";
- not_themeable `task box`: "a 0.9375 rem square, rounded 4px, with a 1.5px edge and a 0.75 rem mark (gpui-base/text/node.rs:3899-3903, :3917): literals; the edge is the list's text colour where a list style sets one, and gpui-component's sets none (gpui-base/text/node.rs:3904-3910; gpui-component/text/mod.rs, base_text_view_style)";
- instance `task boxes`: "not clickable: the showcase sets no on_task_toggle, so they are drawn as upstream draws them by default (gpui-component/text/compat.rs, TextView::on_task_toggle)";
- instance `table hover`: "a body row under the pointer is filled table_hover; the header keeps table_head (gpui-base/text/node.rs:4431-4435)".

### A.5 Tests (`SC/tests.rs`)

Each new test is placed beside the existing ones of its page. Selectors are the
ids above.

1. `the_diff_samples_file_is_the_decorated_one` (`#[test]`, beside
   `the_diff_sample_parses_to_one_file`, `:7518-7535`): `DiffFile::parse(DIFF_SAMPLE)`
   holds one file whose `path()` is `DIFF_DECORATED_PATH` (`DiffFile::path` is
   public, `GC/diff/document.rs:494`; its per-side line accessors are `pub(crate)`,
   `:555-614`). The line is proved from `DIFF_SAMPLE`'s own text: its one hunk
   header is `@@ -5,12 +5,12 @@` (`SC/support.rs:141`), so the modified side starts
   at 5; counting from 5 the body lines after the header that do not start with
   `-`, the one counted `DIFF_DECORATED_LINE` starts with a space (a line both sides
   keep) and, with that space removed, is
   `    let system = SystemTheme::from_system()?;` (`SC/support.rs:144`).
2. `the_diffs_decorated_line_is_painted_in_its_claim` (`#[gpui::test]`):
   Typography page in `TALL_WINDOW`; the `typography-diff` info's `decorated line,
   warning at 20%` claim exists; among `scene_quads` inside `typography-diff`'s
   bounds one is painted exactly in that claim's colour (`painted_fill`'s reading,
   `:9415-9433`, filtered by colour instead of by area). Red without A.3's
   `.decorations(…)`.
3. `the_markdown_sample_holds_a_task_list` (`#[test]`): `MARKDOWN_SAMPLE` has
   exactly one line beginning `- [x] ` and one beginning `- [ ] `.
4. `the_checked_task_box_is_painted_in_its_claim` (`#[gpui::test]`): Typography
   page in `TALL_WINDOW` (the Markdown sits low on the page, and gpui-pre drops a
   primitive wholly outside its content mask, `gpui-pre 0.3.12 src/scene.rs:87-95`); the `typography-markdown` info's `checked task box, fill and edge` claim
   exists; a quad inside the Markdown's bounds is filled exactly in it and is
   square within 1 device pixel: gpui-pre rounds each edge of a painted quad to
   the device-pixel grid on its own (`gpui-pre 0.3.12 src/window.rs:4938`,
   `snap_bounds` at `:3525-3535`), so a square's width and height may differ by
   one. Red without A.4's sample lines.
5. `the_layout_page_shows_a_grid_layout` (`#[gpui::test]`): Layout page in
   `TALL_WINDOW`; `layout-grid-layout` is laid out and `settle_on` gives an info
   of kind `GridLayout`; `layout-grid-item-one` and `-two` share a top, and the
   second's left edge is the first's right edge plus `cx.theme().spacing_tokens().md`
   (read from the theme, not written out) within 1 device pixel, since gpui-pre
   rounds each element's outer edges to device pixels after layout
   (`gpui-pre 0.3.12 src/taffy.rs:381-387`); `layout-grid-item-three` lies below
   both and spans from the first's left to the second's right.
6. `the_grid_layouts_info_says_its_gap_and_motion_are_upstreams` (`#[test]`):
   `info::layout::grid_layout(&Theme::default())`'s `gap` note holds
   `spacing_tokens().md` and `no field behind it`, and its `motion` note
   `ResolvedTheme has no motion field` (the toolbar test's shape, `:1600-1637`).
7. `a_bars_waveform_and_a_liquid_one_show_different_infos` (`#[gpui::test]`):
   Inputs page; `inputs-speech-waveform` and `inputs-speech-waveform-liquid` settle
   on infos of kind `SpeechWaveform` with the variants `bars` and `liquid`.
8. `the_liquid_waveform_spans_its_row` (`#[gpui::test]`): Inputs page; the
   liquid waveform (`inputs-speech-waveform-liquid`) starts at the section
   heading's left edge (`inputs-heading-speech-waveform`, whose id is its debug
   selector, `SC/demo.rs:3155-3160`) and is wider than the bars
   (`inputs-speech-waveform`), which keep upstream's 96 px.

**On GPUI Fast**, tests 2 and 4 read the scene, whose reader skips composited
scroll layers (`CN/src/widgets/scene_read.rs:15-18`). If only GPUI Fast cannot
find the quad, that half takes a `cfg` with this reason, as the 0.8.1 Separator
test did (0.8.1 spec §7); on gpui-pre the test must pass.

### A.6 Gates of Part A

On both cores: the showcase's suites,
`cargo test -p native-theme-gpui --features native,watch --example showcase-gpui`
and the same with `gpui-fast,native,watch` (through `scripts/run_in_runner_env.sh`
as `scripts/check_release.sh:845-855` runs them); and the connector's library
tests, `cargo test -p native-theme-gpui --lib --features native,watch` and the
same with `gpui-fast,native,watch`, which hold the claim and prose gates
(`CN/src/showcase.rs:2819`, `:3691`), `every_widget_reports_itself` (`:1558`)
and `the_showcase_hardcodes_no_style_values` (`:4719`): `CN/src/showcase.rs`
is a library test module (`CN/src/lib.rs:243-244`, `#[cfg(test)] mod showcase`),
which an `--example` run does not build; clippy
`-D warnings` on both cores and with `--no-default-features`; the strict panic
lints on the showcase
(`scripts/check_strict_panic.sh -p native-theme-gpui --example showcase-gpui`,
both feature sets). `python3 scripts/check_widget_coverage.py`
passes before Part F as after it, but before it proves nothing about the new
section: today's rule does not discover `GridLayout` at all. Part F's rule and its
`grid_layout` entry are what make the gate count the section (F.6).

---

## Part B — Statements (N3)

### B.1 S1 — the menu item's height

`SC/info/chrome.rs:393-396`, in `popup_menu(info, t)` (`t: &Theme` in scope,
`:339`). Present:

> "PopupMenu builds its own; geometry::menu_item has no receiver here (geometry.rs, menu/menu_item.rs: MenuItemElement is pub(crate)). An item is 26px tall at the default Size, rounded with the theme radius up to 8px (menu/popup_menu.rs, RenderOptions)"

New, a `format!` with `super::rems_text(crate::demo::MENU_ROW_HEIGHT, t)` (the
showcase's copy of upstream's `rems(1.625)`, `SC/demo.rs:351-360`, as
`SC/info/chrome.rs:286` already states it). `rems_text` prints the size in
pixels, the rem multiple and the font size it is taken at
(`SC/info/mod.rs:487-497`), so the sentence does not repeat them:

> "PopupMenu builds its own; geometry::menu_item has no receiver here (geometry.rs, menu/menu_item.rs: MenuItemElement is pub(crate)). An item is {rems_text} tall at the default Size, a rem being the theme's font_size, which gpui-component makes the window's rem (gpui-component/menu/popup_menu.rs, PopupMenu::render_item; gpui-component/root.rs, WindowState::prepare), rounded with the theme radius up to 8px (menu/popup_menu.rs, RenderOptions)"

Read: `GC/menu/popup_menu.rs:1332-1335`, `:1591-1595`; `GC/root.rs:711-714`.

### B.2 S5 — the Square glyph's users

`CN/src/icons.rs:602`, the comment after `IconName::Square => &["media-playback-stop"],`.
Present: `// close: upstream draws it only as SpeechButton's stop glyph (speech/button.rs:96-100); Breeze and Adwaita fill the square`.
New: `// close: upstream draws it as SpeechButton's stop glyph (gpui-component speech/button.rs:135) and as SpeechIndicator's under the pointer (gpui-component speech/indicator.rs:130); Breeze and Adwaita fill the square`.
Part E's run sets this line aside ("line rule: 96-100: vanished", in the dry run's
`plan.json`), so it is on E.5's hand-read list; this edit is that item's
resolution, and E.5 leaves it to B.2 (E.5 names it).

### B.3 S6 — why the widgets' own icons stay upstream's

`SC/info/mod.rs:433-448`, `own_icons`, and the doc comment of
`a_widgets_own_icons_are_named_gpui_components` (`SC/tests.rs:9262-9266`).
**Lands unless the icons work lands in v0.7.1** (rationale N3: the text depends
on the icons document; settled by the session). Under W11 (b), "proposed" and
the maintainer's (`docs/todo_v0.7.1_gpui-widget-icons-rationale.md`, *W11 —
Where it lands*), when the icons rationale is not approved when the work starts,
and when W11 (a) is chosen but its work does not land in v0.7.1, the showcase
installs no resolver in v0.7.1 and the texts below land: the hook exists, the
showcase does not use it yet. When the icons work lands in v0.7.1 it writes these
texts itself, describing the resolver the showcase then installs, and B.3 does
not land (Open point 1).

The doc (`:433-443`, 11 lines) becomes these 10 (no citation outside this
specification reads `SC/info/mod.rs` below `:443`: `git grep` over the worktree
less `docs/archive/`, 2026-11-14):

```rust
/// What a Widget Info says of the icons gpui-component builds and draws
/// inside a widget on its own, `icons` naming them and where upstream draws
/// them. An icon the showcase builds itself is the chosen icon theme's
/// (`SampleIcon`), so this is for upstream's alone. They stay
/// gpui-component's whichever icon theme is chosen: upstream draws each by
/// its asset path, which gpui-component 0.9 first hands to the resolver an
/// application may set (gpui-component icon.rs, `set_icon_resolver` and
/// `resolve_icon_path`), and the showcase sets none yet, so every path
/// draws upstream's own icon. `docs/todo.md` records the work that sets one
/// (*gpui: the widgets' own icons follow the theme's icon set*).
```

The comment names the backlog entry by its heading, as the present one ends
"`docs/todo.md` records it.": D.3 writes that open entry, and the heading stays
when the icons rationale is archived and moves. Where the icons work closes the
entry, it rewrites this comment too.

`SC/tests.rs:9262-9266` keeps the false premise S6 corrects:

```rust
/// Every sample whose gpui-component widget draws icons of its own says
/// they are gpui-component's whichever icon theme is chosen (decision D1):
/// upstream loads them by asset path, and gpui keeps the first SVG it drew
/// for a path (Task 6's spike, `docs/todo.md`), so they cannot follow the
/// chosen theme. The widgets are those whose shown configuration draws one
```

These five lines become the five below; the rest of the paragraph, from `:9267`
("(gpui-component 0.8.1): a DropdownButton's caret, …"), stays as it is (the
comment block's width, 76 columns, is kept):

```rust
/// Every sample whose gpui-component widget draws icons of its own says
/// they are gpui-component's whichever icon theme is chosen (decision D1):
/// gpui-component 0.9 has an icon resolver hook (`set_icon_resolver`) and
/// the showcase does not set it yet, so they stay upstream's. The widgets
/// are those whose shown configuration draws one
```

The text (`:446`) becomes:

> "{icons}: gpui-component's own icons whichever icon theme is chosen, so under any other they are not that theme's. Upstream builds them inside the widget and draws each by its asset path, which it first hands to the application's icon resolver (gpui-component/icon.rs, resolve_icon_path), a hook gpui-component 0.9 added (gpui-component/icon.rs, set_icon_resolver); the showcase sets no resolver yet, so each path draws upstream's icon"

It keeps the phrase `a_widgets_own_icons_are_named_gpui_components` (`SC/tests.rs:9280`)
looks for. Read: `GC/icon.rs:77-106`, `:206-211`, `:260-265`.

### B.4 S8 — a selected list row with `active_highlight` off

`list.active_highlight`'s effect: `GC/list/list_item.rs:257-263`; the connector
maps `accent` and `accent_foreground` from `menu.hover_background` and
`menu.hover_text_color` (`CN/src/colors.rs:289-290`; the builder's doc says so
at `CN/src/geometry.rs:1393-1399`).

- **`CN/README.md:368`**, the `list_item_selected` row's last cell. Present: "a
  selected `ListItem` (`selected(true)`), after `list_item`. A colour, where no
  `ThemeColor` field maps: upstream marks a selected row by its fill alone
  (`list_active`, `list.selection_background`) and leaves its text the row's".
  New: "a selected `ListItem` (`selected(true)`), after `list_item`. A colour,
  where no `ThemeColor` field maps: while `list.active_highlight` is on,
  gpui-component's default, upstream marks a selected row by its fill alone
  (`list_active`, `list.selection_background`) and leaves its text the row's;
  with it off, upstream paints the row `accent` (`menu.hover_background`) under
  `accent_foreground` (`menu.hover_text_color`) after the refinement
  (`src/list/list_item.rs:257-263`), so this colour does not reach it".
- **`SC/info/mod.rs:390`**, `GEOMETRY_NOTES`' `list_item_selected`. Present:
  "list.selection_text_color on a selected ListItem's text, over the foreground
  upstream sets on every row before the refinement -- or geometry::list_item's
  list.item_font colour: upstream marks a selected row by its fill alone, painted
  after the refinement, and mutes a row that is not selectable after it too
  (list/list_item.rs, ListItem::render)". New: "list.selection_text_color on a
  selected ListItem's text, over the foreground upstream sets on every row before
  the refinement -- or geometry::list_item's list.item_font colour: while
  list.active_highlight is on, gpui-component's default, upstream marks a selected
  row by its fill alone, painted after the refinement; with it off, it letters the
  row accent_foreground on accent after the refinement, so this colour does not
  reach it; and it mutes a row that is not selectable after the refinement too
  (list/list_item.rs, ListItem::render)".

### B.5 S9 — a selected `DataTable` cell

`CN/README.md:610-613`. Present: "…gpui-component paints `table_active` over a
selected cell (`src/table/state.rs:2458-2472`), where it paints it behind a
selected row (`:2536-2541`). The connector gives…". New: "…gpui-component paints
`table_active` over a selected cell, inside a 1 px edge of `table_active_border`
(`src/table/state.rs:2458-2472`, `:2630-2638`), where it paints it behind a
selected row as long as `list.active_highlight` is on, its default
(`:2536-2541`; with it off, a selected row is `accent`). The connector gives…".
The rest of the paragraph (`:613-629`) stays. The connector gives
`table_active_border` `list_active_border` (`CN/src/colors.rs:419`, `:431`); the
README does not need that. Read: `GC/table/state.rs:2458-2472`, `:2536-2543`,
`:2630-2638`.

### B.6 S10 — upstream request 2, a draft

`docs/issues/0004_kde-hand-overrides-upstream-requests.md:30-34`. Present: "In
cell-selection mode (`cell_selectable`), gpui-component 0.9.1 paints
`table_active`, edged with `table_active_border`, as an absolutely positioned
child laid over a selected cell's content (`src/table/state.rs:2458-2472`), while
a selected row gets the same token as its background, behind its content
(`:2536-2541`)." New: "In cell-selection mode (`cell_selectable`), gpui-component
0.9.1 paints `table_active`, edged with `table_active_border`, as an absolutely
positioned child laid over a selected cell's content (`src/table/state.rs:2458-2472`,
`:2630-2638`), while a selected row gets the same token as its background, behind
its content, as long as `list.active_highlight` is on, its default (`:2536-2541`;
with it off, a selected row is `accent`)." The draft is unsent and edited in
place (rationale N3).

### B.7 S11 — the exceptions' header

`docs/showcase-exceptions.toml:20-24`. Present: "…cites the file and line it was
read at: gpui-component 0.8.1 (first read at 0.6.4; the gpui section re-read at
0.7.0 on 2026-10-01, re-pointed to 0.7.1 on 2026-10-06, to 0.7.3 on 2026-10-14,
to 0.8.0 on 2026-10-21 and to 0.8.1 on 2026-10-26), iced_widget 0.14.2, iced_aw
0.14.1." New: "…cites the file and line it was read at: gpui-component 0.9.1,
and gpui-base 0.9.1 for the widgets gpui-component re-exports from it (first read
at 0.6.4; the gpui section re-read at 0.7.0 on 2026-10-01, re-pointed to 0.7.1 on
2026-10-06, to 0.7.3 on 2026-10-14, to 0.8.0 on 2026-10-21, to 0.8.1 on
2026-10-26 and to 0.9.1 on <date of Part E's hand-read>), iced_widget 0.14.2,
iced_aw 0.14.1." The `[gpui]` entries' lines and their "read in gpui-component
0.8.1" labels (`:52-53`, `:56`) are Part E's (E.5 names the known ones). **B.7
lands in Part E's hand-read commit**, not with Parts A to D: its date is that
commit's, and the header can say the section is read at 0.9.1 only once the
entries are.

### B.8 S16 — a dropdown row's text

`SC/info/chrome.rs`, `preset_combobox` (`:903-950`). Read: `GC/searchable_list/item.rs:111-123`,
`:143-150`.

- After the claim `row hover, accent at 70%` (`:921-926`), three claims:
  `row hover text`, `accent_foreground`, `t.accent_foreground`,
  `gpui-component/searchable_list/item.rs:116`; `selected row`, `accent`,
  `t.accent`, `gpui-component/searchable_list/item.rs:121`; `selected row text`,
  `accent_foreground`, `t.accent_foreground`, `gpui-component/searchable_list/item.rs:122`.
  `row hover text` stays after `row hover, accent at 70%`: the test at
  `SC/tests.rs:6840-6847` takes the first claim whose role starts with
  "row hover" and expects accent at 70%.
- The `rows` note (`:937-940`). Present: "SearchableList's own, not ListItems: a
  hovered row is accent at 70% and a selected one accent in full
  (searchable_list/item.rs, SearchableListItemElement)". New: "SearchableList's
  own, not ListItems: a hovered row is accent at 70% and a selected one accent in
  full, each lettered accent_foreground after the caller's style, and the check
  beside the chosen row takes the row's text colour (searchable_list/item.rs,
  SearchableListItemElement)".

### B.9 S17 — the readers of `spacing_tokens()`

`CN/src/geometry.rs:1433-1436` (below the claimed range). Present at `0fa28027`:
"There is no receiver to map these into: gpui-component's `Theme::spacing_tokens()`
returns `SpacingTokens::default()` with no field behind it (`theme/mod.rs:588-590`),
so its one reader, a `Dialog`'s viewport margin (`dialog/dialog.rs:667`), gets that
default whatever a theme says." Part E's run renumbers `588-590` to `596-598`
first (the line was last changed by `fef436aa`, outside E.3's set), so the edit is
anchored on the span from "There is no receiver" to "whatever a theme says." and
replaces all of it, whatever its digits are then: the four comment lines
`:1433-1436` (last changed by `56cb658a`, `6faef5b2` and `fef436aa`) become:

```rust
// the system path. There is no receiver to map these into: gpui-component's
// `Theme::spacing_tokens()` returns `SpacingTokens::default()` with no field
// behind it (gpui-component `theme/mod.rs:596-598`), so its readers get that
// default whatever a theme says: a `Dialog`'s viewport margin (gpui-component
// `dialog/dialog.rs:667`), a `GridLayout`'s gap (gpui-component
// `grid_layout.rs:65`) and, through `Theme::semantic_tokens`, a
// `Questionnaire`'s metrics (gpui-component `questionnaire/components.rs:87-88`,
// `:1219`).
```

Read: `GC/theme/mod.rs:513-517`, `:596-598`; `GC/dialog/dialog.rs:667`;
`GC/grid_layout.rs:65`; `GC/questionnaire/components.rs:87-88`, `:1219`.

**The same statement in the showcase**, `SC/info/layout.rs:46`, the `receivers`
note of the *Layout spacing* panel (`spacing_box`, `:27-62`). Present: "none a
theme can write: Theme::spacing_tokens() returns SpacingTokens::default() with no
field behind it (theme/mod.rs, spacing_tokens), and its one reader, a Dialog's
viewport margin, gets that default (dialog/dialog.rs, Dialog). So these four are
the application's to apply (geometry.rs §9.5)". New, naming the readers of the
`geometry.rs` comment above:

> "none a theme can write: Theme::spacing_tokens() returns SpacingTokens::default() with no field behind it (gpui-component/theme/mod.rs, spacing_tokens), and its readers get that default: a Dialog's viewport margin (gpui-component/dialog/dialog.rs, Dialog), a GridLayout's gap (gpui-component/grid_layout.rs, grid_layout) and, through Theme::semantic_tokens, a Questionnaire's metrics (gpui-component/questionnaire/components.rs, QuestionnaireMetrics). So these four are the application's to apply (geometry.rs §9.5)"

It agrees with A.1's `gap` note on the same page, which says the same of the
grid's gap. `grid_layout.rs` names its crate: gpui-base's
`grid_layout/grid_layout.rs` also holds the word `grid_layout`, which the prose
gate would report as ambiguous (`CN/src/showcase.rs:3392-3417`).

### B.10 S18 — the editor scheme's status colours

Read: `GC/theme/default-theme.json:113-128` (light: `warning` #C99401, `hint`
#9e5dff, `created`, `modified`, `deleted`), `:315-340` (dark: `hint`, `created`,
`modified`, `deleted`, the backgrounds of `error`, `hint`, `info`, `success`
and `warning` and the borders of `error`, `hint`, `info` and `warning`); the
fields and fallbacks, `GC/highlighter/registry.rs:318-463`.

- **`CN/src/config.rs:106-107`.** Present: "The current line, the line numbers,
  the invisibles and the syntax colours / stay upstream's scheme: the model states
  none of them." New:

```rust
/// The current line, the line numbers, the invisibles and the syntax colours
/// stay upstream's scheme: the model states none of them. So do the status
/// colours the scheme states (gpui-component `src/theme/default-theme.json:113-128`,
/// `:315-340`): a hint's in both modes, a warning's in light mode, several
/// backgrounds and borders in dark mode, and the `created`, `modified` and
/// `deleted` colours gpui-component 0.9 added. A status colour the scheme leaves
/// unstated falls back to the `Theme`'s (gpui-component
/// `src/highlighter/registry.rs:358-463`), which the connector maps from the
/// platform's status colours. `docs/todo.md`
/// records it, *gpui: the editor's status colours are partly upstream's scheme*.
```

The block's words are the specification; its lines are rewrapped to the width of
the comment block it joins, as G.6's rule asks. The same holds for
B.9's block above, whose `Questionnaire` line is 81 columns.

- **`CN/src/lib.rs:116`**, the `input` row. Present: "…through the highlight
  theme (its current line, line numbers and syntax colours stay upstream's
  scheme), and its line height via `geometry::editor` |". New: "…through the
  highlight theme (its current line, line numbers and syntax colours stay
  upstream's scheme, and so do the status colours the scheme states: a hint's in
  both modes, a warning's in light mode, several backgrounds and borders in dark
  mode, and 0.9's `created`, `modified` and `deleted`; see
  [the backlog](https://github.com/tiborgats/native-theme/blob/main/docs/todo.md),
  *gpui: the editor's status colours are partly upstream's scheme*), and its line
  height via `geometry::editor` |": a Markdown link to the repository's document,
  as `CN/src/lib.rs:154` points to one by URL (an autolink there).
- `CN/src/lib.rs:110` stays: it states the `Theme`'s status colours, which the
  connector does map (`CN/src/colors.rs:691-697`).

### B.11–B.12

S2 and S3 are Part C's; S4, S7, S12, S13 and S14 are Part D's.

### B.13 S15 — the connector's check box asks the resolver

**`CN/src/widgets/checkbox.rs:430-437`**, the `None =>` arm of `look.mark_stroke`
(the theme states no check-mark stroke). Line 435,
`mark.path(IconName::Check.path()).opacity(opacity)`, becomes
`mark.path(gpui_component::resolve_icon_path(IconName::Check.path(), cx)).opacity(opacity)`,
inside the same `.when(opacity > 0., …)`, as upstream's check box resolves only
a visible mark (`GC/checkbox.rs:220-223`). The function is called by path: the
file's `use gpui_component::{…}` (`:14-16`) is not touched (*Lines that must not
move*). A comment above the arm: "gpui-component's own glyph by its asset path,
which the application's icon resolver is asked for first, as gpui-component's
check box asks it (gpui-component checkbox.rs:220-222)" — the crate named, since
the comment sits in the connector's own `checkbox.rs`. The `Some(stroke)` arm, the mark the
connector draws itself, is unchanged (icons rationale W8).

**The same arm draws a `Radio`'s mark** where the theme states neither a radio
dot nor a check-mark stroke: the match at `:412` takes the dot only for a round
control with `look.dot` set, a radio without it falls to the `look.mark_stroke`
match, and
`look.dot` is `checkbox.radio_dot_diameter` (`:163`), which inherits nothing
(`docs/inheritance-rules.toml:446`). The connector's `Radio` draws through the
same `indicator_and_label` (`Radio::row`, `:795-799`, with `CheckboxLook::radio`). Eleven
presets state neither `radio_dot_diameter` nor `check_mark_stroke_width`
(`grep -c` of both keys over `native-theme/src/presets/*.toml`, 2026-11-14): the
four catppuccin presets, dracula, gruvbox, ios-13, nord, one-dark, solarized and
tokyo-night. Under them the connector's `Radio` draws the check glyph by path and,
after this change, asks the application's resolver too: the same gap, closed for
the same reason, so it stays in (the rationale's S15 names the radio too).

**Test, `CN/src/widgets/tests.rs`**, `#[gpui::test]`
`the_check_mark_asks_the_applications_icon_resolver`, beside
`the_check_mark_fills_the_indicator_inside_its_padding` (`:175-189`):

1. a preset whose `checkbox.check_mark_stroke_width` is unstated —
   `macos-monterey`; only `adwaita`, `kde-breeze` and `material-3` state one
   (`grep -l check_mark_stroke_width native-theme/src/presets/*.toml`) — and an
   assertion that `CheckboxLook::of(&resolved("macos-monterey", ColorMode::Light), true, false)`
   is `Some(look)` with `look.mark_stroke == None`. Both halves matter: where
   `CheckboxLook::of` gives `None`, the connector draws gpui-component's own check
   box instead (`Checkbox::fallback`, `CN/src/widgets/checkbox.rs:609-624`, through
   `row`, `:635-638`), which already asks the resolver (`GC/checkbox.rs:221`), and
   step 3 would pass without the change;
2. the test helper `window_with(cx, Some(("macos-monterey", ColorMode::Light)), …)`
   (`CN/src/widgets/tests.rs:1678-1695`: it installs the preset, then opens and
   draws the window), drawing `Checkbox::new("c").label("C").checked(true)`; an
   assertion that `native-checkbox-indicator` is laid out (the connector's box,
   not the fallback); then `gpui_component::set_icon_resolver` with a resolver
   that records each path it is asked for in an `Rc<RefCell<Vec<String>>>` and
   returns `None`; `redraw`;
3. assert that the record holds `IconName::Check.path()`; a spring's first value
   is adopted at once (`GB/motion.rs:712-717`), so a box drawn checked is opaque
   from its first frame;
4. the record is cleared; then the same in the same `App`, where the resolver is
   still installed, under `kde-breeze` (stroke stated) through a second
   `window_with`: the record does not hold it;
**The radio half is a test of its own**, `#[gpui::test]`
`the_radio_mark_asks_the_applications_icon_resolver`, so it runs in an `App` of
its own: in the first test's `App`, the earlier windows could draw again under
nord's theme and ask the resolver for the check mark themselves, and the record
would not tell which control asked; the test file's helpers open windows
(`window_with`) and offer no call that closes one. Its steps: an assertion that
`CheckboxLook::radio(&resolved("nord", ColorMode::Light), true, false)` is
`Some(look)` with `look.dot == None` and `look.mark_stroke == None`;
`window_with(cx, Some(("nord", ColorMode::Light)), …)` drawing
`Radio::new("r").label("R").checked(true)`; `native-checkbox-indicator` is laid
out and `native-radio-dot` is not (`CN/src/widgets/checkbox.rs:420`, `:501`);
the resolver installed as in step 2, `redraw`; the record holds
`IconName::Check.path()`.

Red before the change (step 3 and the radio test fail), green after. Gates:
`cargo test -p native-theme-gpui --lib` on both cores and with `--no-default-features` (the
test takes the `widgets` feature's `cfg` where the module needs it), clippy as in
A.6, and the claim gate (its pinned lines in this file are unmoved).

### B.14 Gates of Part B

A.6's commands, `cargo test -p native-theme-gpui --lib` on both cores, and
`cargo doc -p native-theme-gpui --no-deps` without warnings (B.10's link). For
the texts no test reads, G.6's check, on the texts of G.6's list that Part B's
commit writes (B.7's in Part E's hand-read commit).

### B.15 A self-citation in `SC/info/` that no gate reads

`SC/info/icons.rs:15-17`, the doc of `opaque`, cites "native-theme-gpui
icons.rs:1326-1330", which today holds test code (inside `mod tests`, which opens
at `CN/src/icons.rs:1192-1194`). What the doc states — the connector writes the
colour into the SVG as `#rrggbb`, with no alpha — is at `CN/src/icons.rs:698-700`
(`to_image_source`'s parameter doc) and `:1181-1190` (`to_rgba`, which builds an
opaque `Rgba`). The citation becomes "(native-theme-gpui icons.rs,
`to_image_source` and `to_rgba`)": a new citation names the functions, not their
lines (the maintainer's S6, `docs/todo_v0.7.1_batch-speed-rationale.md`).

How the rest was checked (2026-11-14): `grep -nE 'native-theme-gpui[^"]{0,40}:[0-9]|connector[^"]{0,60}\.rs:[0-9]|widgets/[a-z_]+\.rs:[0-9]' SC/info/*.rs`,
leaving out the colour claims' quoted `"native-theme-gpui/…:N"` citations, which
`every_colour_claim_is_read_at_the_line_it_cites` reads, finds this line and one
other, `SC/info/feedback.rs:295`, which cites egui's `spinner.rs`, not the
connector. The notes that name a connector function without a line
(`native-theme-gpui geometry.rs, select`) cite no line to drift.

---

## Part C — The Combobox workaround (N4)

**Removed:** `SC/support.rs:1270-1305`, `PresetDelegate::on_will_change` with its
doc, and `SearchableListChange` from the import (`:15`), which only it uses
(`grep -n SearchableListChange SC/*.rs`: `:15`, `:1283`, `:1287`, `:1300`).
`PresetDelegate::position` (`:1249-1258`) and the rest stay.

**The test that decides:** `the_theme_settings_switch_the_preset`
(`SC/tests.rs:1639-1674`), which types "nord" while `default` is the selection
in row 0 and presses Enter, run alone on both cores:

```
cargo test -p native-theme-gpui --features native,watch --example showcase-gpui the_theme_settings_switch_the_preset
cargo test -p native-theme-gpui --features gpui-fast,native,watch --example showcase-gpui the_theme_settings_switch_the_preset
```

What the reading expects (not run): 0.9.1 compares the selection's values before
and after `on_will_change` (`GC/combobox.rs:184-200`, `:467-483`), and the default
hook records the item at its filtered row (`GC/searchable_list/delegate.rs:167-201`),
so choosing Nord changes the values and emits the change.

- **Both pass.** The removal is committed. The test's comment (`:1656-1659`)
  becomes: "Typing filters the rows and puts the cursor on the first match
  (list/list.rs, ListState::start_search), which Enter takes. Nord lands in row 0,
  where `default` already is: the case gpui-component before 0.9.0 missed by
  telling a selection apart by its filtered row indices alone; 0.9.0 compares the
  selection's values (gpui-component combobox.rs, ComboboxState::new)." It names
  no backlog entry, so it is true at its own commit, before D.3 moves the entry.
  The backlog entry closes (D.3).
- **Either fails.** The override and the import are restored; Open point 2
  applies.

Gate: the two commands above, then A.6's suites.

---

## Part D — The backlog's upstream entries and the two pins (N5)

### D.1 The entries

`grep -n '^\s*- \[ \] \[upstream\]' docs/todo.md` lists 65 open `[upstream]`
entries at `0fa28027`. Nine are not about GPUI Kit's crates: `:1755` (KDE's Union
styles), `:2119` and `:2134` (egui), `:2802`, `:2818` and `:2831` (iced and
iced_aw), `:4307` (iced's cosmic-text), `:4397` (epaint) and `:4297` (Xwayland,
which names gpui-pre in passing). The other 56, with the rationale's verdict
(§1.4, §1.5; not re-read for this specification) and what happens:

| Line | Heading | Opening words | 0.9.1 | What happens |
|---|---|---|---|---|
| 727 | *The tab gap, the check mark's line, …* | gpui Textarea padding below upstream's editor padding | still so | D.2 |
| 2187 | *native-theme-gpui connector* | `unicode-properties` back to 0.1.4 when gpui-fast-web unpins it | resolved (S12) | D.6, then D.3 |
| 2204 | *native-theme-gpui connector* | File the gpui-fast issue: `Window::painted_quads()` omits | still so | D.2 |
| 2286 | *native-theme-gpui connector* | File the gpui-kit issue: gpui-pre clips a rounded box's children | still so | D.2 |
| 2336 | *native-theme-gpui connector* | File the gpui-kit issue: a non-panicking `TextSystem::try_resolve_font` | still so | D.2 |
| 2390 | *Upstream PR candidates from v0.5.8 (Tier U)* | a `ghost_hover` / `ghost_hover_foreground` token pair | still so | D.2 |
| 2403 | *Tier U* | a menu-surface token | still so | D.2 |
| 2414 | *Tier U* | a foreground token for the status bar | still so | D.2 |
| 2424 | *Tier U* | foreground tokens for a selected list row and for selected text | changed | D.4 |
| 2433 | *Tier U* | `Checkbox::label` sets `text_color(foreground)` | still so, by another mechanism | D.4 |
| 2449 | *Tier U* | `WindowBorder`'s frame colour is a literal | changed (S4) | D.4 |
| 2457 | *Tier U* | a placeholder colour for `Input` | still so | D.2 |
| 2472 | *Tier U* | the focus ring's width | still so | D.2 |
| 2482 | *Tier U* | gpui-component reads `tokens.tab`, `tokens.list` | still so | D.2 |
| 2489 | *Tier U* | the segmented `TabBar` has receivers for the track only | still so | D.2 |
| 2494 | *Tier U* | `Button::icon` discards an `Icon`'s own size | still so | D.2 |
| 2499 | *Tier U* | `Checkbox` and `Radio` draw their unchecked border with `theme.input` | still so | D.2 |
| 2517 | *Tier U* | a control's radius should not shrink with its size | still so | D.2 |
| 2528 | *Tier U* | `Toggle` needs a token of its own for the pressed state | still so | D.2 |
| 2538 | *Tier U* | `TabVariant::Outline` hovers with `tokens.secondary_hover` | still so | D.2 |
| 2549 | *Tier U* | `InputGroupButton` should hover with the platform's button hover | still so | D.2 |
| 2558 | *Tier U* | a `Dialog::max_h` prop | still so | D.2 |
| 2566 | *Tier U* | a styled `Theme` scrollbar-style override honoured by `base_theme()` | still so | D.2 |
| 2568 | *Tier U* | a scrollbar thumb inset per axis | still so | D.2 |
| 2583 | *Tier U* | `Tab` keeping the caller's height, radius and text size | still so | D.2 |
| 2586 | *Tier U* | `Theme.shadow` honoured beyond `Button` | still so | D.2 |
| 2600 | *Tier U* | `Size::Size` honoured by `Checkbox` and `Switch` | still so | D.2 |
| 2601 | *Tier U* | a disabled `Button` should read the platform's disabled colours | still so | D.2 |
| 2621 | *Tier U* | the `Link` widget should read `link_hover` and `link_active` | still so | D.2 |
| 2646 | *Tier U* | a `PopupMenu` item style hook; a `Button::tooltip` style hook | still so | D.2 |
| 2647 | *Tier U* | button label text size independent of rem | still so | D.2 |
| 2651 | *Tier U* | public base-palette fields on `ThemeConfigColors` | still so | D.2 |
| 2652 | *Tier U* | internal scrollbars reserve no gutter | still so | D.2 |
| 2688 | *Tier U* | a right-clicked `Tree` row shows the hover text colour | still so | D.2 |
| 2715 | *Tier U* | element hooks on the surfaces upstream builds itself | still so | D.2 |
| 2742 | *Tier U* | **GPUI Kit, from issue #4** | still so (written at 0.9.1) | D.2 |
| 3793 | *Upstream PR to gpui* | PR: add `Window::screenshot()` API to gpui | still so | D.2 |
| 3812 | *Upstream PR to gpui* | PR: let an application drop the SVGs a window has drawn | changed (S7) | D.4 |
| 3878 | *Upstream PRs to gpui-component* | Audit gpui-component widgets for hardcoded values | no claim | nothing |
| 3880 | *Upstream PRs to gpui-component* | PR: expose per-widget padding/margin as theme-configurable | still so | D.2 |
| 3881 | *Upstream PRs to gpui-component* | PR: expose checkbox/radio indicator size as theme token | still so | D.2 |
| 3888 | *Upstream PRs to gpui-component* | PR: expose scrollbar dimensions as theme-configurable | resolved earlier (S14) | D.3 |
| 3889 | *Upstream PRs to gpui-component* | PR: expose button min-height and icon spacing as theme tokens | still so | D.2 |
| 3893 | *Upstream PRs to gpui-component* | PR: let a scroll container keep the wheel to itself | still so | D.2, with two cases |
| 3958 | *Upstream PRs to gpui-component* | Additional PRs as gaps are discovered | no claim | nothing |
| 3959 | *Upstream PRs to gpui-component* | Report: a single-select `Combobox` misses a change after a search | resolved (S3), Part C decides | D.3 |
| 3972 | *Upstream PRs to gpui-component* | PR: let a consumer supply the icons gpui-component's widgets build | resolved (S7) | D.3 |
| 4043 | *Upstream PRs to gpui-component* | PR: Button: let the caller's text size reach the label | still so | D.2 |
| 4051 | *Upstream PRs to gpui-component* | PR: ColorSelect: apply the caller's refinement to the framed field | still so | D.2 |
| 4054 | *Upstream PRs to gpui-component* | PR: DataTable: `Styled`, or a text-size setter | still so | D.2 |
| 4056 | *Upstream PRs to gpui-component* | a public way to mark an element's accessibility node disabled | still so | D.2 |
| 4063 | *Upstream PRs to gpui-component* | PR: an `Icon` that draws an image, or an icon slot | still so | D.2 |
| 4086 | *Upstream PRs to gpui-component* | PR: let an application set the window's appearance on Windows | still so | D.2 |
| 4140 | *gpui-fast pins `taffy = "=0.15.0"`* | gpui-fast 0.1.12 requires taffy at exactly 0.15.0 | resolved (S13) | D.6, then D.3 |
| 4316 | *gpui: Settings and dock splitters draw upstream's renderer* | gpui-component 0.7.0's `Settings` divider and the dock's edges | still so | D.2 |
| 4365 | *gpui: Questionnaire needs an infallible state constructor* | `QuestionnaireState::new` returns a `Result` | still so | D.2 |

Counts: 45 "still so" with D.2 alone, `:2433` "still so" through D.4 (46), 5
resolved, 3 changed, 2 with no claim: 56.

### D.2 A still-holding entry's dated line

Appended after the entry's last line, at its indentation:
`*GPUI Kit 0.9.1 (<date>):* still so; <each upstream item the entry names, with its 0.9.1 line>.`
Each 0.9.1 line comes from the copy's `lookup.py` (E.1) given the crate, file and
line of the entry's newest dated citation (G4's 0.8.1 lines where it has them),
never by eye; a number it sets aside, or one cited at a version older than
0.8.1, is read in the 0.9.1 source and found by its text. Where no line moved, the
line says "still so, at the same lines". The text of the entry is not rewritten.
`:2742` gets "*GPUI Kit 0.9.1 (<date>):* still so; written at 0.9.1 (2026-11-13),
its lines unchanged." `:3893` also names 0.9.1's two cases it does not mention: a
horizontal area whose content fits passes a swipe that moves vertically to its
parent (gpui-base 0.9.1 `src/scrollable_mask.rs:553-561`), and sideways packets
keep the overscroll stream alive (`src/scroll_bounce.rs:575-577`).

**Check:** the `lookup.py` command lines and their output, one per cited number,
and for each number read by hand the 0.9.1 file and line with its text, go into
the commit body.

### D.3 Entries that close

Each moves to `docs/archive/todo_closed.md` verbatim, ticked, with a closing line
in one of two forms (settled by the session):

- `  *Closed <date> in <sha>:* …` where a commit of this work resolves the entry:
  `:2187` and `:4140` (D.6's lock commits), `:3959` (Part C's removal commit);
- `  *Closed <date>:* …`, with no commit, where none does, the entry having been
  resolved upstream: `:3888` and `:3972` — the form of `todo_closed.md:2619`
  ("*Closed 2026-10-26:* …").

The moves are a commit of their own, without
D.2's or D.4's edits, so this **check** holds, run on the staged change before it
is committed: `git diff --cached --name-only` lists only `docs/todo.md` and
`docs/archive/todo_closed.md`; `git diff -U0 --cached -- docs/todo.md` adds no
line, and `git diff -U0 --cached -- docs/archive/todo_closed.md` removes none;
and the lines `docs/todo.md` loses, blank lines dropped and `- [ ]` read as
`- [x]`, are, in the same order, the lines `todo_closed.md` gains once these are
dropped from them: blank lines, the `### Closed items from …` headings, and each
closing paragraph — a line that starts with two spaces and `*Closed `, with the
lines that follow it and start with exactly two spaces and then a character
other than a space, not `- [` (a closing paragraph wraps so, as
`todo_closed.md:921-922` does). The order holds because each entry's place in
`todo_closed.md` keeps `docs/todo.md`'s order: `:2187` goes into the group at
`todo_closed.md:1062-1382`, `:3888`, `:3959` and `:3972` into the new group after
`:1383-1442`, and `:4140`'s section before `:1942`. The moved taffy section's
heading is removed and added, so the comparison accounts for it. The script,
kept with G.6's in
`/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/checks/`
(a new folder beside E.1's `citations/`; `.superpowers/` is excluded from the
repository by `.git/info/exclude:7`) and run from the worktree's root as
`python3 -I /home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/checks/check_moves.py`,
prints `ok` (exit status 0) or each line that breaks the check (exit status 1):

```python
import difflib
import re
import subprocess

TODO, CLOSED = "docs/todo.md", "docs/archive/todo_closed.md"

def diff(*args):
    return subprocess.run(["git", "diff", "--cached", "--no-color", *args],
                          capture_output=True, text=True, check=True).stdout

def body(path, sign):
    lines, hunk = [], False
    for line in diff("-U0", "--", path).splitlines():
        if line.startswith("@@"):
            hunk = True
        elif line.startswith("diff "):
            hunk = False
        elif hunk and line.startswith(sign):
            lines.append(line[1:])
    return lines

def moved(added):
    out, closing = [], False
    for line in added:
        if line.startswith("  *Closed "):
            closing = True
        elif closing and re.match(r"  [^ ]", line) and not line.startswith("  - ["):
            pass
        else:
            closing = False
            if line.strip() and not line.startswith("### Closed items from "):
                out.append(line)
    return out

bad = []
names = sorted(diff("--name-only").split())
if names != sorted([CLOSED, TODO]):
    bad.append(f"staged files: {names}")
for line in body(TODO, "+"):
    bad.append(f"{TODO} adds: {line}")
for line in body(CLOSED, "-"):
    bad.append(f"{CLOSED} removes: {line}")
removed = [line.replace("- [ ]", "- [x]", 1)
           for line in body(TODO, "-") if line.strip()]
added = moved(body(CLOSED, "+"))
match = difflib.SequenceMatcher(None, removed, added, autojunk=False)
for tag, i1, i2, j1, j2 in match.get_opcodes():
    if tag != "equal":
        bad += [f"{TODO} removes, not moved (ticked): {line}" for line in removed[i1:i2]]
        bad += [f"{CLOSED} adds, not moved: {line}" for line in added[j1:j2]]
print("\n".join(bad) or "ok")
raise SystemExit(1 if bad else 0)
```

Its output goes into the commit's body. The check does not read what a closing
paragraph says (any line of two spaces and a non-space under a `*Closed` line
passes as its continuation); that text is the reviewer's reading of the diff, as
G.6 leaves D.3's lines.

- **`:2187`** → the end of *Closed items from native-theme-gpui connector*
  (`todo_closed.md:1062-1382`). Its line: gpui-fast-web 0.1.15 asks for
  `unicode-properties = "0.1.3"` without the exact pin (its `Cargo.toml:53-54`;
  0.1.12 `"=0.1.3"`); `cargo update -p unicode-properties` moved the lock from
  0.1.3 to <the version D.6 read back>, for <the dependents D.6's `cargo tree`
  lists>; the checks D.6 names passed.
- **`:4140`**, its whole section *gpui-fast pins `taffy = "=0.15.0"`*
  (`:4138-4167`) → under `## Toolkit Connectors` of `todo_closed.md`, after its
  last section and before `## Publishing Prep` (`:1942`). Its line: gpui-fast
  0.1.15 asks for taffy `"0.15.0"` without the exact pin (its `Cargo.toml:347-348`;
  0.1.12 `"=0.15.0"`, `:344-345`), as gpui-pre 0.3.12 does (`Cargo.toml:527-528`);
  `cargo update -p taffy` moved the lock to <the version D.6 read back>, which both
  cores now build on; the checks D.6 names passed.
- **`:3888`, `:3959`** (Part C passed) **and `:3972`** → a new group
  `### Closed items from Upstream PRs to gpui-component` after *Closed items from
  Upstream PR candidates from v0.5.8 (Tier U)* (`todo_closed.md:1383-1442`), in
  their `docs/todo.md` order.
  - `:3888`: "resolved before GPUI Kit 0.9: gpui-base styles the track's and the
    thumb's width (gpui-base 0.9.1 `src/scrollbar.rs:644-682`), which the connector
    writes (`connectors/native-theme-gpui/src/base_layer.rs`, `scrollbar_geometry`
    and `scrollbar_styles`); the table's
    fixed 16 px is *GPUI Kit, from issue #4*, request (1)."
  - `:3959`: "gpui-component 0.9.0 compares the selection's values before and
    after `on_will_change` (0.9.1 `src/combobox.rs:184-200`, `:467-483`), so the
    default hook emits the change; the showcase's `PresetDelegate::on_will_change`
    is removed in <sha>, and `the_theme_settings_switch_the_preset` passes on both
    cores without it. Upstream's v0.9.0 release notes name PR 3449
    (`docs/todo_v0.7.1_gpui-kit-0.9.1-rationale.md` at <sha of commit (0)>, §1.4,
    S3; the notes were not re-read for this line)." A closing line is a record,
    so it cites a document of this cycle at a commit: commit (0) of *Order and
    gates*, which adds this cycle's documents.
  - `:3972`: "closed as a request: gpui-component 0.9.0 has the hook,
    `set_icon_resolver`, which every `Icon` drawn from an asset path asks for the
    path, the icons its widgets build included (gpui-component 0.9.1
    `src/icon.rs:77-106`). The work that uses it is the open entry *gpui: the
    widgets' own icons follow the theme's icon set*." Where the icons rationale
    is in commit (0), the line adds: "It is designed in
    `docs/todo_v0.7.1_gpui-widget-icons-rationale.md` at <sha of commit (0)>."
    **The open entry it names** is written first, in a commit of its own before
    D.3's moves (D.3's check refuses any line the moves commit adds to
    `docs/todo.md`), whether or not the icons rationale is in commit (0): the
    entry tracks the icons work wherever that lands — a later release, or this one
    — and stays open until that work is done. The icons work closes it: the
    session tells the icons specification's author to close it in that
    specification's F.5 (`docs/todo_v0.7.x_gpui-widget-icons-spec.md`, *F.5
    `docs/todo.md`*). It is placed at the end of `## Toolkit Connectors`, after
    Part H's section and before the `---` that closes the part (`:4248` at
    `0fa28027`); Part H's commit comes first (*Order and gates*, point 5), so the
    entry follows Part H's section in the commit that writes it and in the final
    file. Its form is that of the backlog's other entries:

    ```markdown
    ### gpui: the widgets' own icons follow the theme's icon set

    - [ ] [v0.7.x] gpui-component's widgets draw their own Lucide glyphs — a
          `Select`'s or `Combobox`'s caret, the check mark beside a chosen row, an
          empty list's Inbox and the like — whatever the theme's icon set, until
          the connector or the gpui showcase sets the application's icon resolver
          (`set_icon_resolver`, gpui-component 0.9.1 `src/icon.rs:77-106`), which
          every `Icon` drawn from an asset path asks first. The work is designed in
          *gpui: the icons gpui-component's widgets draw follow the theme's icon set
          — rationale*.
    ```

    The last sentence ("The work is designed in *… — rationale*.") is written only
    where the icons rationale is in commit (0), as the closing line's added
    sentence is; otherwise the entry ends at "asks first.". The examples are the
    ones the showcase's Widget Info names today: the preset Combobox's own icons
    (`SC/info/chrome.rs:906`) and the icon-theme Select's "upstream's icons"
    (`:994-995`). The rationale is named by its title, which does not move when it
    is archived. G.6's check covers the entry.
  - If Part C fails, `:3959` stays open and gets the dated line Open point 2
    describes.
  - If D.6's update of a package turns a test red, its entry (`:2187` or
    `:4140`) stays open with the dated line D.6 describes, and does not move.

### D.4 Entries changed or holding by another mechanism

- **`:2449`** (*Tier U*, "`WindowBorder`'s frame colour is a literal"). Its first
  sentence (`:2449-2451`) is rewritten to the shadow alone, after the entry's
  `- [ ] [upstream] ` prefix, which stays: "`WindowBorder`'s
  shadow is a literal: no theme value reaches the two-layer black shadow it draws
  below the frame in its `render`." The G4 line stays (it is dated). Appended:
  "*GPUI Kit 0.9.1 (<date>):* the frame reads `window_border`
  (gpui-component 0.9.1 `src/window_border.rs:201`), which the connector writes
  from `window.border.color` (`connectors/native-theme-gpui/src/colors.rs`,
  `to_theme_color` and `assign_tab_sidebar`); the shadow is still the literal two black layers
  (`src/window_border.rs:253-286`)."
- **`:2424`** (*Tier U*, "foreground tokens for a selected list row and for
  selected text"). Kept; appended: "*GPUI Kit 0.9.1 (<date>):* still so for a
  selected list row while `list.active_highlight` is on, gpui-component's default
  (gpui-component 0.9.1 `src/list/list_item.rs:257-259`), and for selected text. With it off, the
  row is `accent` under `accent_foreground` (`:260-263`); a `Select`'s or
  `Combobox`'s dropdown row is lettered `accent_foreground` when hovered or
  selected, after the caller's style (`src/searchable_list/item.rs:112-123`)."
- **`:2433`** (*Tier U*, "`Checkbox::label` sets `text_color(foreground)`").
  Appended: "*GPUI Kit 0.9.1 (<date>):* still so by another mechanism: the
  disabled colour is now a state style that re-applies the caller's refinement
  (gpui-component 0.9.1 `src/checkbox.rs:262-268`), so a carried colour would still displace it;
  <the label wrapper's and `combobox.rs`'s 0.9.1 lines, by D.2's rule>."
- **`:3812`** (*Upstream PR to gpui*, "PR: let an application drop the SVGs").
  Its opening purpose clause (`:3812-3814`), "so the icons gpui-component's
  widgets build for themselves can follow a theme switch", is no longer what the
  request is for: with 0.9.0's resolver an application can make them follow a
  switch by answering each icon set with paths of its own. The clause becomes "so
  a path a window has drawn can show what the application's asset source answers
  for it now; gpui-component 0.9's icon resolver lets the widgets' own icons follow
  a theme switch without it, through paths of their own per icon set". Its premise
  that a path cannot vary per theme — "The path cannot vary per theme
  either: each call site builds a fixed `IconName`." (`:3831-3833`) — and the
  pointer "the icon-provider hook in "Upstream PRs to gpui-component" below is the
  other way in" (`:3846-3847`) are removed. Appended: "*GPUI Kit 0.9.1 (<date>):*
  gpui-component 0.9.0's `set_icon_resolver` lets an application answer each icon
  path with another, so a path can vary per icon set (0.9.1 `src/icon.rs:77-106`);
  what stays is gpui-pre's half: a drawn SVG cannot be dropped, so a set reused
  under the same paths would keep the old drawing; <gpui-pre 0.3.12's lines for
  `Window::paint_svg`, the atlas and `Window::drop_image`, by D.2's rule>."

### D.5 `ROADMAP.md`'s upstream list

The bullets of *GPUI Kit's PRs* (`ROADMAP.md:222-257`) that cite upstream lines
are re-cited in place at 0.9.1, by D.2's lookup, their version labels with them:
`:224` (`tab/tab.rs:958-966`, gpui-component 0.8.1), `:225-228`
(`dialog/dialog.rs:343-360`), `:239-241` (`schema.rs:643-677`), `:242-244`
(`diff/mod.rs:472`), `:245-246` (`text/node.rs:4418`, gpui-base 0.8.1), `:247-250`
(`dialog/entrance.rs:14-27`), `:251-253` (`src/elements/div.rs:53`, gpui-pre
0.3.10) and `:254-257` (`tooltip.rs:226-275`, gpui-base 0.8.1). A bullet whose
item no longer holds at 0.9.1 is reported, not rewritten: in the commit body and
in the plan's record of the task. **Check:** as D.2's,
the `lookup.py` output and each hand-read line's text go into the commit body.
For D.4's edits, the same, with each quoted new text present in the file (G.6's
check).

### D.6 The two pins

One commit per package, in this order, each on a clean tree:

```
cargo update -p unicode-properties
grep -A1 '^name = "unicode-properties"$' Cargo.lock
git diff Cargo.lock
cargo tree --workspace -i unicode-properties --target all
```

then the same with `taffy`. **Gates of each:** the diff changes the package's
`version` and `checksum` lines only, or the commit body lists every other changed
line with its reason; the version read back is the one the commit body and the
closing line name (it is read, not assumed — the registry holds
unicode-properties 0.1.4 and 0.1.5 and taffy 0.15.1 today, which says nothing of
the index); `cargo audit`;
`cargo test -p native-theme-gpui --features native,watch --all-targets` and
`--features gpui-fast,native,watch --all-targets`;
the gpui showcase's suites on both cores through `scripts/run_in_runner_env.sh`;
`scripts/check_release.sh --commit`. If the lock does not change, the entry closes
with the version the lock keeps and the requirements read above. **If the update
turns a test red on either core**, the lock keeps the old version
(`git checkout HEAD -- Cargo.lock`, staged or not), the entry (`:4140` for taffy, `:2187` for `unicode-properties`)
stays open with a dated line saying which test failed on which core at which
version, and the commit body (of the dated line's commit) reports it. If taffy moves,
the three taffy citations E.5 names are re-cited at the version reached in Part
E's hand-read commit, which follows D.6 (*Order and gates*). No CHANGELOG line
(rationale N5).

### D.7 Gates of Part D

The five documentation checkers: `python3 scripts/check_platform_facts_rows.py`,
`check_section_refs.py`, `check_preset_citations.py`,
`check_wrong_safety_nets.py`, `check_todo_citations.py`, each with no finding;
D.2's, D.3's and D.5's checks, their output in the commit bodies.

---

## Part E — Cited upstream lines (N6)

### E.1 The copy of the scripts

Copied from `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_code-review-plan/g4/t2/`
(`remap.py`, `upstream.py`, `notes.py`, `gates.py`, `gate4_additions.py`,
`lookup.py`, `chains_md.py`) to
`/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/citations/`.
The originals are not changed. In the copy, every G4 constant changes; these are
all of them:

- **`upstream.py`**: `FAMILY` (`:19-28`): gpui-component and gpui-base
  `("0.8.1", "0.9.1")`; gpui-kit and gpui-kit-assets `("0.8.1", "0.9.1")`, whose
  `src/` folders are identical (`diff -rq`, 2026-11-14), so only a manifest
  citation can move (kept although N6 leaves them out of scope: decision 12);
  gpui-pre and the 19 `PRE_SUBS` `("0.3.10", "0.3.12")`;
  gpui-fast and the 8 `FAST_SUBS` `("0.1.12", "0.1.15")`. All 66 directories
  are in the registry (checked 2026-11-14). gpui-pre-bench-metrics is new at
  0.3.12 and has no 0.3.10, so it stays out. `NEW_TO_SCOPE` (`:30`) becomes the
  empty set: no crate is new to the scope. The docstring (`:1-2`) names this
  document.
- **`remap.py`**: `REPO` (`:24`) the worktree's root; `SCOPE` (`:25-36`) as E.2;
  `REVIEW_ASIDE` (`:42-47`) empty and `_WRONG` (`:41`) gone (G4's review rulings);
  `T1_COMMITS` (`:48-49`) becomes the set of E.3, computed at `HEAD`; the docstring
  (`:2-12`) and the message at `:506` ("the unit's: a 0.8.1 / 0.3.10 / 0.1.12
  number") name 0.9.1, 0.3.12 and 0.1.15. The decision label "Task 1's lines" is
  kept as a label and means E.3's lines.
- **`gates.py`**: `REPO` (`:19`); `BASE_REV` (`:20`) the full SHA of the commit the
  run starts from; `VERSIONS` (`:22-56`) as `FAMILY`, kept separately on purpose
  (0.8.1 spec §4.1); `TASK1` (`:57`) E.3's set computed at `BASE_REV`; `SCOPE`
  (`:58-69`) as E.2; `REVIEW_ASIDE` (`:75-80`) empty; `NEW_CRATES` (`:82`) empty;
  the docstring (`:2-10`).
- **`lookup.py`**: the docstring (`:2-14`) names the new pairs; `PLAN` (`:25`)
  reads the copy's `run/plan.json`.
- **`chains_md.py`**: `REPO` (`:15`); its heading texts (`:41-46`) name this run.
- `notes.py` and `gate4_additions.py` hold no constant to change.

**The check of the copy**, before any run:
`grep -nE '0\.8\.0|"0\.3\.9"|0\.1\.10|010a99b8|native-theme"$' <copy>/*.py`
finds nothing.

### E.2 Scope

The 0.8.1 run's scope (P9), with `SC/info/` excluded:

```
connectors/native-theme-gpui/src
connectors/native-theme-gpui/tests
connectors/native-theme-gpui/examples
:(exclude)connectors/native-theme-gpui/examples/showcase-gpui/info
connectors/native-theme-gpui/README.md
connectors/native-theme-gpui/Cargo.toml
docs/showcase-exceptions.toml
tools/showcase-support/src
scripts/connector_parity_exceptions.toml
connectors/native-theme-egui/examples
connectors/native-theme-iced/examples
```

Both `remap.py` and `gates.py` pass these to `git ls-files`, which takes the
exclude pathspec. P9's three attribution rules stay as the copy has them
(`remap.py:37-38`, `:117-157`; `gates.py:70-71`, `:267-313`). `docs/todo.md` is not
in scope (rationale N6; its entries get D.2's lines).

### E.3 The lines left alone

Every line `git blame` attributes to a commit of
`git rev-list HEAD ^8eb8f2e54c6c4f08c73953ffbebdb4a1999e21c3^ ^fc6109d1b31167645ac00593647bbcb3b252515a`:
reachable from the branch's head, not from the port's parent (`8eb8f2e5^`,
`caeddd6b`) nor from `fc6109d1`, the `main` side of the merge `0d447c56` (its
second parent, an ancestor of `origin/main`). At `0fa28027` the range holds 34
commits; at the run it holds those and the commits made since. The script's rule
(`remap.py:72-80`, `gates.py:346-352`) is unchanged; only the set is. Every such
line that cites upstream is in `t1-skipped.txt` and is read by hand (E.5).

### E.4 The run, in order

1. The copy and its check (E.1), on a clean tree at the last commit of unit LN
   (Part I), which follows commit (0) and the gate-speed unit (GS,
   `docs/todo_v0.7.1_gate-speed-plan.md`) of *Order and gates*; `BASE_REV` is
   LN's last commit. First
   `git diff --name-only <commit (0)> HEAD -- <E.2's pathspecs>` prints nothing:
   GS changes `scripts/check_release.sh`, `scripts/step_cache.sh` (new),
   `scripts/step_cache_includes.py` (new), `scripts/msrv_checks.sh` (new),
   `scripts/README.md`, `CONTRIBUTING.md` and `CHANGELOG.md` only
   (`docs/todo_v0.7.1_gate-speed-spec.md`, *How to read it*), and LN the files
   Part I names, none of them in E.2's scope,
   so the scope's tracked files are commit (0)'s; where it prints a path, the
   work stops.
2. `python3 <copy>/remap.py --plan <copy>/run`: nothing is written in the tree;
   `counts.txt` shows rule 1 (out of scope) and rule 2 (set aside) at least once
   each, so both rules still act. A past-tense citation (E.5) whose chain the plan
   would renumber is put into `remap.py`'s `REVIEW_ASIDE`, keyed as G4's entries
   were (file, line, the number's text), so the run leaves its number as written;
   then `remap.py --plan` runs again, since steps 3 and 4 read `plan.json`. The
   entries go into `remap.py` only: `gates.py` takes the set-aside list from
   `plan.json` (`gates.py:355-362`), and an entry of its own `REVIEW_ASIDE`
   (`:75-80`) that the scan does not meet makes gate 4 fail (`:405-424`), so
   `gates.py`'s stays empty (E.1).
3. `python3 <copy>/gates.py <copy>/run <copy>/run/tree` on the tree the edits
   would leave. **What passes means here and in step 5:** gates 1 (digits only)
   and 2 (line text) pass; gate 4 is not expected to pass. `gates.py` reads the
   set-aside list only from `plan.json` (`gates.py:355-362`), so a number its own,
   independent attribution places differently is reported as failing and the
   script exits 1. G4's run had 76 such failures before and after its write
   (`todo_code-review-plan/g4/t2/run/gates-plan-tree.txt`, `gates.txt`, read
   2026-11-14), and a reviewer's dry run of this copy 65 (its output, read
   2026-11-14: "1527 numbers checked, 1185 hold their old-version text, 277 on the
   set-aside list, 65 fail"). Each is listed by step 5's `gate4_additions.py` and read
   by hand (E.5); gate 4 passes when nothing it reports is missing from that
   list.
4. `python3 <copy>/lookup.py --check` passes (ten moved numbers in ten files, the
   lookup agreeing with the run).
5. `python3 <copy>/remap.py --write <copy>/run`, the one run; then
   `python3 <copy>/gates.py <copy>/run` on the working tree, gates 1 and 2 passing
   as in step 3, and
   `python3 <copy>/gate4_additions.py <copy>/run > <copy>/run/set-aside-gate4.txt`,
   which lists every number gate 4 fails with its whole chain; that file joins the
   set-aside list.
6. `every_colour_claim_is_read_at_the_line_it_cites` and
   `every_prose_citation_still_exists` (`CN/src/showcase.rs:2819`, `:3691`) and
   `the_fallback_stack_copy_is_gpuis` (`SC/tests.rs:10730`) pass on both cores
   (they read `SC/info/`, which the run does not touch, and `inspector.rs`'s stack
   lines, written at 0.3.12 and 0.1.15 by the port).
7. One commit, digits only. Its body holds the counts by form and rule and the
   set-aside list in full, each entry with its reason, its cited old text and the
   repository line's text, as the 0.8.1 run's did. The script is not run again: a
   second run would read moved numbers as old ones.

### E.5 Read by hand

- **The set-aside list** (`set-aside.txt`, `set-aside-gate4.txt`): each citation
  read in the 0.9.1 source and re-pointed or rewritten.
- **E.3's lines** (`t1-skipped.txt`): each read at 0.9.1, in case it was moved
  and not re-read when written.
- **Chains renumbered outside `CN/`** (`review-chains.txt`, rule 3): each one's old
  text read against its claim.
- **Version labels.** The command, run from the worktree's root on 2026-11-14:

  ```
  git grep -I -nE '\b(0\.8\.[01]|0\.7\.[0-9]|0\.3\.(9|10)|0\.1\.(1[0-4]|[7-9]))\b' -- \
    connectors/native-theme-gpui/src connectors/native-theme-gpui/tests \
    connectors/native-theme-gpui/examples \
    ':(exclude)connectors/native-theme-gpui/examples/showcase-gpui/info' \
    connectors/native-theme-gpui/README.md connectors/native-theme-gpui/Cargo.toml \
    docs/showcase-exceptions.toml tools/showcase-support/src \
    scripts/connector_parity_exceptions.toml connectors/native-theme-egui/examples \
    connectors/native-theme-iced/examples scripts/check_widget_coverage.py \
    native-theme/src | wc -l
  ```

  gives 200 at `0fa28027` (201 without the exclude; the same without `-I`, since
  no binary file is in these paths), native-theme's own versions and other
  crates' among them. The count is taken again when the work starts and goes in
  the hand-read commit's body. **The rule** (G4's `7b05831c`, rationale N6): every
  present-tense version label of an in-scope crate becomes the locked version
  (gpui-component, gpui-base, gpui-kit and gpui-kit-assets 0.9.1, gpui-pre and
  its sub-crates 0.3.12, gpui-fast and its sub-crates 0.1.15; one gpui-kit-assets
  label is in scope, `CN/src/widgets/checkbox.rs:304`, "gpui-kit-assets 0.8.1
  `assets/icons/check.svg`"), whether
  the run moved its number, kept it, or found its file unchanged — the dry run of
  E.1's copy has 117 labelled chains that keep their number (80 in unchanged
  files, 37 renumbered to the same number), and they are relabelled too. A
  present-tense measurement is re-measured, not relabelled (0.8.1 spec §4.3). A
  sentence about the past keeps its label and its number: `remap.py` renumbers a
  chain labelled with the old version (`remap.py:407-416`), so a past-tense
  citation is set aside before the write (E.4 step 2) or, where one is found only
  afterwards, its number is restored to the one it had.
- **Files with pinned lines.** In a file the colour claims pin (*Lines that must
  not move*, with each file's last pinned line), a rewrite that starts at or above
  the file's last pinned line keeps its own line count: **each paragraph is
  rewrapped within its own lines, and blank comment lines (`///`, `//!`, `//`)
  stay where they are**. The check below reads `git diff`'s hunks, and a diff
  splits a rewrite that moves a blank line across unchanged lines into hunks of
  unequal size even where the paragraph's total count is kept, so a rewrite that
  keeps each paragraph in place is the one that passes. Below the last pinned line a rewrite may change the count
  (the dry run of E.1's copy has 64 set-aside chains and 3 of E.3's lines in
  `geometry.rs` after line 772). A file's total line count proves nothing here —
  a line added above a pinned line and one removed below it cancel out — and the
  claims test does not catch every move (`geometry.rs:772` claims `border`, and
  `:771` names `border` too; hand-read chains sit at `:763-765`; the same for
  `:229` and `:230`). **The check**, for each pinned file `F` with last pinned
  line `L`, on the hand-read commit's staged change, before it is committed (its
  output goes into that commit's body). `F` is the file's path from the
  repository's root, `connectors/native-theme-gpui/src/<file>`, where *Lines that
  must not move* names it relative to `CN/src/`; the `echo` heads each file's
  listing with its path:

  ```
  echo "== F"; git diff --no-color -U0 --cached -- F | awk -v last=L '
  BEGIN { last += 0 }
  /^@@/ {
      split($2, o, ","); split($3, n, ",")
      start = substr(o[1], 2) + 0
      old = (o[2] == "" ? 1 : o[2] + 0); new = (n[2] == "" ? 1 : n[2] + 0)
      moves = (old == 0 ? start < last : start <= last && old != new)
      print $0, (moves ? "MOVES PINNED LINES" : "ok")
      if (moves) bad = 1
  }
  END { exit bad }'
  ```

  Each hunk header is `@@ -a,n +b,m @@`; an omitted count is 1, and every field is
  read as a number. A hunk that replaces or deletes old lines (`n > 0`) starting at
  or above `L` (`a <= L`) must keep its count (`n == m`). A pure insertion
  (`-a,0`) puts its lines after old line `a`, so it moves a pinned line only when
  `a < L`. A hunk below `L` never fails. Every line the script prints goes into
  the commit body. A line marked `MOVES PINNED LINES` (and the exit status 1 that
  comes with it) fails the check, with one exception: where a rewrite cannot keep
  its count, the same commit re-points the showcase's colour claims that cite the
  moved lines, in `SC/info/`, and their expected copies in `SC/tests.rs` (which
  holds 31 `"native-theme-gpui/…"` citations as expected values, among them
  `"native-theme-gpui/widgets/mod.rs:344"` at `:16109`, the 13 `checkbox.rs` ones
  in `checkbox_leaves`, `:15853-15922`, and 17 `switch.rs` ones after it,
  `:15947-16035`) — a named exception to
  decision 2 — and that hunk passes only where the commit body names, under the
  hunk's listing line, each claim and copy re-pointed for it, and
  `every_colour_claim_is_read_at_the_line_it_cites` is green on both cores. The
  exception is not for a rewrap that keeps a paragraph's count but shifts its
  lines: `git diff` shows a paragraph whose first line was dropped and a line added
  at its end as two hunks (`@@ -67 +66,0 @@`, `@@ -69,0 +69 @@`), both marked; it is
  fixed by reflowing the whole paragraph so that each new line replaces an old
  one, which gives one hunk of equal counts. **Who checks:** the reviewer of the
  commit (the session's diff review before merge) reads every `MOVES PINNED LINES`
  line against the commit body. `every_colour_claim_is_read_at_the_line_it_cites`
  stays the backstop for every commit. **Where it runs:** on commit (iii), and on
  each commit of (ii) that edits a pinned file: B.13's (`connectors/native-theme-gpui/src/widgets/checkbox.rs`,
  `L` = 145; its edit is at `:429-437`) and B.9's
  (`connectors/native-theme-gpui/src/geometry.rs`, `L` = 772; its edit at
  `:1433-1436`); no other part of (ii) edits a pinned file.
- **`native-theme/src`'s two gpui citations** P9 names, now `src/lib.rs:425` and
  `src/gnome/mod.rs:572` (with `src/detect.rs:456-459`): they cite gpui-pre-linux
  0.3.12 `src/linux/x11/client.rs:2674-2681`, which reads `Xft.dpi` (read
  2026-11-14); they stay.
- **Known items**, which the script may set aside:
  - `docs/showcase-exceptions.toml:53`, `SpeechButton`: `speech/button.rs:96-100`
    → `:135`, `:137` (the `Square` and `Mic` arms, in that order) and `:78-79` → `:98-99`, "read
    in gpui-component 0.9.1";
  - `:52`, `SidebarToggleButton`: `sidebar/mod.rs:358-369`, `:378`, re-read;
  - every other `[gpui]` reason, re-read, and the header rewritten as B.7 says,
    in this commit and with this commit's date;
  - `CN/src/icons.rs:602`, `speech/button.rs:96-100`: set aside by the run and
    resolved by B.2's edit, which E.5 leaves as B.2 writes it;
  - `SC/demo.rs:354-356`, the doc of `MENU_ROW_PADDING_X` and `MENU_ROW_HEIGHT`:
    written by `9066771e` (in E.3's range, so the run leaves it) and already wrong
    at 0.9.1, "menu/popup_menu.rs:1327, `INNER_PADDING`; :1333, the item height"
    → `:1328`, `:1334` (`GC/menu/popup_menu.rs:1328`, `INNER_PADDING`; `:1334`,
    the default Size's `rems(1.625)`).
- **taffy, after D.6.** Three citations read taffy 0.15.0 lines:
  `SC/demo.rs:2536` (`style/mod.rs:882`), `:2538` (`compute/oof.rs:511-512`) and
  `SC/tests.rs:7600` (`src/compute/flexbox.rs:2274-2275`). If D.6 leaves taffy at
  0.15.0 they stay. If it reaches 0.15.1, read 2026-11-14: `style/mod.rs` is the
  same file (`:882`, `box_sizing: BoxSizing::BorderBox`), `oof.rs:511-512` is
  `:515-516`, `flexbox.rs:2274-2275` is `:2320-2321`, and the labels become 0.15.1;
  another version is read there. `unicode-properties` has no such citation
  (decision 13).
- **Labels outside E.2's scope**, found with
  `git grep -nE '\b(0\.8\.1|0\.3\.10|0\.1\.12)\b'` over the worktree less
  `docs/archive/`, `Cargo.lock`, `CHANGELOG.md`, `docs/todo.md`, this release's
  documents and E.2's scope (2026-11-14):
  - `scripts/check_showcase_parity.py:122-128`: "gpui-pre 0.3.10" becomes 0.3.12;
    `src/taffy.rs` and `src/util.rs` are identical in 0.3.10 and 0.3.12 (`cmp`), so
    `:281-356`, `:382-385`, `util.rs:128-132` and `:423-425` stay;
    `src/text_system.rs:52` becomes `:53` (`SUBPIXEL_VARIANTS_Y`) and
    `src/window.rs:4853-4858` becomes `:5085-5090` (the quantised glyph origin);
  - `scripts/check_showcase_parity.py:191-192`: "gpui-pre 0.3.10 src/window.rs:4865"
    becomes "gpui-pre 0.3.12 src/window.rs:5097" (`glyph_dilation_for_color`);
  - `connectors/native-theme-iced/src/styles.rs:262-263`: "gpui-component 0.8.1"
    becomes 0.9.1; `src/button/button.rs:730` is the same line there
    (`.when(!self.disabled && !selected, …)`). This is the iced connector's
    source, so its compatibility stamp goes stale until G.4;
  - `connectors/native-theme-gpui/proposals/README.md:67-77`: the status note
    becomes "(v0.7.1, gpui-component 0.9.1)", and "`ThemeConfig` still holds only
    the fields listed under *Background*, with `colors` and `highlight`" becomes
    "… listed under *Background*, `window_radius` (0.9.0's corner radius of a
    client-decorated window), `colors` and `highlight`" (`ThemeConfig`'s fields,
    `GC/theme/schema.rs`, against 0.8.1's: `window_radius` is the one added);
  - they stay: `ROADMAP.md:82-83` (the v0.7.0 section, history), `ROADMAP.md:224-257`
    (D.5's), `connectors/native-theme-gpui/clippy.toml:8` (a document's path),
    `docs/todo_gpui-full-theme.md:754-782` (the 0.8.1 note, history; G.5 adds
    0.9.1's), `scripts/check_widget_coverage.py:44` (F.1's).
- A reason text in `docs/showcase-exceptions.toml` that occurs verbatim in
  several `[[parity.group]]`s gets the same rewrite in each
  (`scripts/check_showcase_parity.py` keys groups by reason text).

### E.6 Citations of the connector's own lines from outside it

None is re-pointed (the maintainer's S6, `docs/todo_v0.7.1_batch-speed-rationale.md`):
the citations of the gpui showcase's and the connector's lines from the egui and
iced connectors, `tools`, `scripts` and the documents are left as they are and may
drift as Parts A to D move lines. The colour claims' citations, which
`every_colour_claim_is_read_at_the_line_it_cites` reads, are E.5's (*Files with
pinned lines*).

### E.7 Gates of Part E

Each gate applies to one commit:

- **The digits-only commit (E.4 step 7):** gates 1 and 2 of the copy's
  `gates.py` pass, and gate 4's failures are all in `set-aside-gate4.txt` (E.4
  step 5); the three tests of E.4 step 6 pass on both cores. Gate 1 compares with
  `BASE_REV`, so it is not run on a later commit.
- **The hand-read commit (E.5 with B.7, after Parts A to D):** the commit
  is not digits-only (it rewrites set-aside citations, relabels versions and
  rewrites B.7's header), so its changes are sorted, on the whole staged change, by

  ```
  test -n "$(git diff --cached --name-only)" || { echo "nothing staged"; exit 1; }
  git diff --cached --no-color -U0 --word-diff=porcelain \
      --word-diff-regex='[0-9]+(\.[0-9]+)*|[^0-9[:space:]]+' \
    | awk '/^@@/{h=1;next} /^diff /{h=0} h && /^[-+]/' \
    | grep -vE '^[-+][0-9]+(\.[0-9]+)*$'
  ```

  which first refuses an empty index ("nothing staged") and then prints every
  changed word run other than a number or a version. It is saved as a script
  and run from the worktree's root (`sh <script>`), not pasted into an
  interactive shell, where its `exit` would close the shell. Its exit status: 1
  with "nothing staged" for an empty index; otherwise the last `grep`'s, 0 when
  it prints a line and 1 when it prints none (only numbers and versions
  changed). The `awk` keeps the hunk
  bodies' `-` and `+` lines and drops the file headers (`--- a/…`, `+++ b/…`),
  so a run that itself starts with `-` or `+` still shows, as `---…`, `--D` or
  `+-W`. A range shift whose two ranges overlap (`:1435-1436` to `:1436-1437`)
  is split by porcelain into the parts that differ and shows as `-N-` / `+-N`
  (here `-1435-` and `+-1437`); such a line is checked at the citation it belongs
  to, read at its new line. Each line it prints must belong to a rewrite E.5
  or B.7 names, and the output, with the rewrite each belongs to, goes into
  the commit body. Each re-pointed number is read at its new line as well. Then: E.5's check for files with pinned lines (the
  hunk listing of each such file, in the body); G.6's check for B.7's
  header; a second run of E.5's version grep that finds only history, other
  crates' versions or excepted lines, listed in the commit body; the three tests
  of E.4 step 6 again.
- **Both:** the gate the batch's cadence gives each unit
  (`docs/todo_v0.7.1_batch-speed-rationale.md`, S1: during a unit, the agent
  runs only the touched tests, its targeted commands; at a unit's end,
  `--quick`, by the orchestrator, as a long run until its time is measured, the
  first timed into the record; `--commit` every third code unit of the
  release, counted from the last `--commit`, on the release's last code unit,
  before its documents and version units, after a lock or toolchain
  change, the warm-up gate, and wherever the plan names a unit structural; a
  documents-only unit runs the five documentation checkers and `--quick`; the
  full run where the plan has it, the close-out, unchanged), which replaces a
  `scripts/check_release.sh --commit` at every commit; the plan says which each
  commit gets.

---

## Part F — The coverage gate (N7)

### F.1 The discovery rule, `scripts/check_widget_coverage.py`

A new function, `gpui_base_reexports(component_src, base_src)`, returns the
public names of the gpui-base widgets gpui-component makes public. `main`
(`:923`) adds them to the gpui section, as discovered and as the universe, each
as `{name: [name]}`; a name gpui-component's own rule already found stays one
entry (none does today). gpui-base's source comes from the same `cargo metadata`,
`source_dir(meta, "gpui-base")` (`:489-493`).

1. **The statements.** Every `.rs` file under gpui-component's `src/`, comments
   and `#[cfg(test)]` modules removed (`strip_comments`, `strip_test_modules`).
   A statement counts when it is `pub use gpui_base::<tree>;` with the visibility
   exactly `pub` (`pub(crate)`, `pub(super)` and `pub(in …)` do not count, so
   `GC/virtual_list.rs:1` does not). The tree is expanded into (path, name) pairs:
   braces nest; `X as Y` is the item `X` under the public name `Y`; `*` is a glob;
   `self` inside braces is the module itself.
2. **Which statements are public.** A file's module path follows from its place
   (`lib.rs` the root, `a.rs` or `a/mod.rs` → `a`, `a/b.rs` → `a::b`); an inline
   `mod name { … }` block is a module of its own. A statement counts when every
   module from the root down to its own is declared `pub mod` (`GC/lib.rs:29-97`).
   In a module that is not, it counts only for the names a public module
   re-exports from that module by name or by glob (`GC/lib.rs:122`
   `pub use virtual_list::{VirtualList, …}` for the private `mod virtual_list`,
   `:21`), along any chain, as Rust's visibility is transitive: publicity is a
   fixpoint, a module being public when a public module declares it `pub mod`
   under a public name or re-exports it by name or by glob, and each name a
   public module re-exports is carried through every private module it is
   re-exported from. So `pub(crate) mod actions { pub use gpui_base::actions::*; }`
   (`GC/lib.rs:25-27`) does not count: the module is not public and nothing
   re-exports from it.
3. **Following the path in gpui-base, as Rust resolves it.** A path from
   gpui-component starts at gpui-base's `lib.rs`; a path inside gpui-base starts at
   the module its `pub use` is in.
   - `crate::` starts again at `lib.rs`, `self::` stays in the current module and
     `super::` goes to its parent (gpui-base has both: `text/editor_host.rs:44`
     `pub use super::node::{…}`, `input/editor/display_map/mod.rs:19`
     `pub use self::display_map::{…}`, `input/mod.rs:86`
     `pub use crate::number_input::{…}`).
   - A segment is a module when the current module declares it with `mod` or
     `pub mod`. Its file is `<segment>.rs` or `<segment>/mod.rs` in the current
     module's directory, which for `lib.rs` and a `mod.rs` is its own folder and for
     any other file `x.rs` is the folder `x/` beside it (gpui-base: `motion.rs` with
     `motion/`, `plot/scale.rs` with `plot/scale/`, `plot/shape.rs`,
     `text/inline.rs`). A `#[path = "…"]` on the declaration names the file,
     relative to the declaring file's folder (`input/mod.rs:13-31`, nine of them);
     rustc reads that file as a `mod.rs`, so its own modules are beside it.
     A segment the current module brings in by a `pub use` of a module is followed
     through that `pub use`. An inline `mod name { … }` (gpui-base has one,
     `input/base/native.rs:1-2`, `#[cfg(target_os = "macos")] mod macos { … }`,
     with `pub use macos::set_text_content_type;` at `:107-108`) is a module whose
     items are those inside its braces, and is followed as any other module; no
     path the rule follows today crosses one. Only `pub use` brings a name in: a
     `pub(crate) use`, `pub(super) use` or plain `use` brings in nothing.
   - A path whose **first** segment is none of these — not `crate`, `self` or
     `super`, not a module of the current module — names an external crate and
     yields nothing when it is one of gpui-base's dependencies in the
     `cargo metadata` (the name Rust knows it by, a rename honoured, `-` read as
     `_`) or `std`, `core` or `alloc`: `input/mod.rs:129` `pub use ropey::Rope;`
     (which `GC/input/mod.rs:26-44` re-exports), `text/markdown_ext.rs:21`
     `pub use markdown::mdast as markdown_ast;`, and `gpui::…`. Any other such
     first segment is a `Failure`.
   - A `Failure` (exit 2: "the re-export rule no longer matches gpui-base's
     layout") stays only for a **module segment** — any segment before the
     path's last — that names nothing the current module declares or brings in,
     the first segment of a path from gpui-component included. The last segment,
     the item's name, never fails: a name the module does not offer (not declared,
     or brought in only by a `pub(crate) use`) yields nothing.
   - At the last module, the name is (a) a `pub struct` that module's file
     declares, or (b) brought in by that module's own `pub use` items, by name, by
     rename or by glob, which are followed the same way to the file that declares
     it; anything else — a trait, an enum, a function, a constant — is not a
     widget and yields nothing. A glob yields every `pub struct` the module
     declares and every name its own `pub use` items bring in. A `pub mod`
     re-exported by name (`pub use gpui_base::animation;`, `GC/lib.rs:103`) is
     followed as a glob of that module.
4. **Which items are widgets.** A followed `(file, name)` is a widget when that
   file declares `pub struct name` and, in the same file, either implements
   `RenderOnce` or `IntoElement` for it (`RENDER_IMPL`, `:199-201`) or derives
   `IntoElement` or `IntoPlot` on it (`DERIVE`, `RENDER_DERIVES`, `:202-203`):
   the existing rule, applied per file. This is what keeps same-named types
   apart: gpui-base's `Text` is a non-rendering `pub struct` in `plot/label.rs:107`
   and a rendering `pub enum` in `text/mod.rs:54` (`:110`), and its `Collapsible`
   a trait in `component_traits.rs:53` (which `GC/component_traits.rs:1`
   re-exports) and a widget in `collapsible.rs:14`.
5. **The name** the gate looks for in the showcase is gpui-component's public
   one (the `as` name where renamed).

Run over gpui-component and gpui-base 0.9.1 on 2026-11-14 as a simulation in
Python with the script's own regexes and steps 1 to 5 as now written (the external
crates, `crate::`/`self::`/`super::`, `#[path]` and the `x.rs` folders included;
not the final code), the rule raises no `Failure` and finds exactly:
`GridLayout` (`GC/grid_layout.rs:14`), `ResizablePanel`, `ResizablePanelGroup`,
`ResizeHandle` (`GC/resizable.rs:18-21`), `VirtualList` (`GC/virtual_list.rs:2`,
public through `GC/lib.rs:122`), `PlotElement`, `PlotAppearScope`
(`GC/plot/mod.rs:6`), `ScrollableMask` and `Scrollbar` (`GC/scroll/mod.rs:4-8`);
gpui-component's own rule finds none of these names.

**The docstring** (`:40-47`) gains a paragraph stating the rule above, and its
version label (`:44`, "gpui-component-macros 0.8.1, derive_into_plot.rs:20")
becomes 0.9.1 (the line is unchanged there).

### F.2 Each widget found

`shows_gpui` over the gpui showcase with literals stripped (the script's own
reading, 2026-11-14) constructs none of them. Read by hand:

| Widget | In the showcase | What happens |
|---|---|---|
| `GridLayout` | not yet | Part A.1's section; F.3's `grid_layout` entry |
| `ResizablePanelGroup` | the window body, `h_resizable("body")` (`SC/app.rs:2350`); `GC/resizable.rs:29` returns one | F.3 entry |
| `ResizablePanel` | the side panel and the content, `resizable_panel()` (`SC/app.rs:2315`, `:2343`); `GB/resizable/mod.rs:29` | F.3 entry |
| `Scrollbar` | `.vertical_scrollbar(handle)` (`SC/support.rs:358`) and `.overflow_y_scrollbar()` (`SC/demo.rs:7004`), both drawing one through `render_scrollbar` (`GC/scroll/scrollable.rs:221`, `:251`, `:327`) | F.3 entry |
| `ResizeHandle` | drawn by the body group between its panels, not constructed by the showcase | exception, kind "an internal sub-part of a compound widget" |
| `PlotElement` | every chart is drawn through one, none constructed by the showcase | exception, kind "an internal sub-part of a compound widget" |
| `PlotAppearScope` | not shown | exception, kind "a layout wrapper with no visual surface", of the charts' machinery |
| `ScrollableMask` | built inside the `Diff`, `DataTable` and Markdown code blocks, not by the showcase | exception, kind "a layout wrapper with no visual surface" |
| `VirtualList` | built inside the `List` and the command palette, not by the showcase | exception, kind "a layout wrapper with no visual surface" |

### F.3 `GPUI_VIA` entries (`:339-359`)

Each with a comment citing what it reads, as the existing ones:

```python
    # `grid_layout` returns gpui-component's `GridLayout`
    # (gpui-component grid_layout.rs:40-64); not a path segment of another
    # call, not a `fn` of the showcase's own.
    "GridLayout": r"(?<!fn )(?<![\w:.])grid_layout\s*\(",
    # `h_resizable` and `v_resizable` return a `ResizablePanelGroup`
    # (gpui-component resizable.rs:29, :34).
    "ResizablePanelGroup": r"(?<![\w:.])[hv]_resizable\s*\(",
    # gpui-base's `resizable_panel` returns a `ResizablePanel`
    # (gpui-base resizable/mod.rs:29; re-exported at gpui-component
    # resizable.rs:18-21).
    "ResizablePanel": r"(?<![\w:.])resizable_panel\s*\(",
    # `ScrollableElement::scrollbar`, `vertical_scrollbar`, `horizontal_scrollbar`
    # and `Scrollable`, which `overflow_*_scrollbar` returns, each draw a gpui-base
    # `Scrollbar` (gpui-component scroll/scrollable.rs:19-41, :221, :251, :327).
    "Scrollbar": r"\.(?:overflow(?:_[xy])?_|vertical_|horizontal_)?scrollbar\s*\(",
```

The existing comments' citations moved at 0.9.1 and are re-pointed in the same
edit (read 2026-11-14): `menu/context_menu.rs:35` → `:39` (`ContextMenu::new(id,
self).menu(f)`; also the docstring, `:117`); `window_ext.rs:30-32` → `:75-77`
(`open_dialog`), `:51-53` → `:101-103` (`open_alert_dialog`), `:14-16` → `:20-22`
(`open_sheet`), `:19-20` → `:30-32` (`open_sheet_at`). `scroll/scrollable.rs:46,
53, 60` and `accordion.rs:63-70` are unchanged. The docstring's *Matching* section
(`:109-119`) adds that a free function that returns a widget counts through
`GPUI_VIA` too.

**The backlog.** `docs/todo.md`, *What the audit turned up that is ours to
fix*, "`scripts/check_widget_coverage.py` still accepts weak evidence"
(`:3560-3568`), says `shows_gpui` "requires a constructor, a call, a struct
literal or a named extension method"; with these entries a free function counts
too. Part F's commit appends, after the entry's last line and at its
indentation (six spaces, wrapped at the entry's 77 columns): "*GPUI Kit 0.9.1
(<date>):* the gpui side also counts a free function that returns a widget,
each one a pattern in `GPUI_VIA` as an extension method is (`grid_layout`,
`h_resizable`, `resizable_panel`); the iced side is unchanged." The entry
stays open. Gate: `python3 scripts/check_todo_citations.py` passes; G.6's
check covers the line.

### F.4 The exceptions (`docs/showcase-exceptions.toml`, `[gpui]`)

A new group after line 67, the last of the Questionnaire group (`:58-67`), and its
blank line; lines 69-71 are `[iced_widget]`'s comment and stay above
`[iced_widget]` (`:72`):

```toml
# gpui-base widgets that gpui-component makes public (check_widget_coverage.py's
# re-export rule): parts of a widget the showcase shows, and wrappers with no
# visual surface of their own. "Paints nothing" was read as the absence of any
# paint_quad or bg call, except a debug outline. Read in gpui-component and
# gpui-base 0.9.1.
PlotAppearScope = "a wrapper of the charts' appear animation, never needed to show a chart: it remembers which plots inside it have finished appearing, so a plot painted again after a gap is shown whole; it takes no part in layout and paints only its child (gpui-base plot/appear.rs:92-113, :196-207). No chart builds one; an application wraps a region of plots in it"
PlotElement = "a part of every chart: `#[derive(IntoPlot)]` writes `type Element = PlotElement<Self>` (gpui-component-macros derive_into_plot.rs:21, :24; gpui-base plot/element.rs:13-30), so each chart of the Charts page is drawn through one, and none is built by hand"
ResizeHandle = "a part of `ResizablePanelGroup`: each `ResizablePanel` after a group's first adds one as a child after its own children, on its leading edge (gpui-base resizable/panel.rs:395-401, through resize_handle.rs:43-48). The window body's group draws one between the side panel and the content, with the showcase's appearance and its own Widget Info (`demo::resize_handles`). gpui-component also offers a standalone `resize_handle`, for an edge no group owns (resizable.rs:42-47, re-exported at lib.rs:116), which the showcase does not use"
ScrollableMask = "a wheel-event mask over a scroll area with no visual surface: it paints nothing but a yellow outline its `debug` switches on (gpui-base scrollable_mask.rs:398-404, :491-500). gpui-component's `Diff` and `DataTable` and gpui-base's Markdown code blocks build one (diff/mod.rs:643; table/state.rs:2949, :2958; gpui-base text/node.rs:2719), and the showcase shows all three"
VirtualList = "a layout wrapper with no visual surface: it lays out the items its caller renders and paints only them (gpui-base virtual_list.rs:214-215, :745-768). gpui-component's `List` and command palette build one (list/list.rs:564; command/state.rs:1021), and the showcase shows both"
```

`ResizeHandle`'s place is read at `GB/resizable/panel.rs:395-401` (the handle is
the child after `.children(self.children)`) and agrees with the showcase's own
comment, `SC/app.rs:2330-2331` ("carries the handle on its left edge"). It is an
exception as an internal part, by the precedent of the section's
`DialogHeader` and `SearchableListItemElement` (`docs/showcase-exceptions.toml:39`,
`:49`); the Widget Info the showcase gives it (`SC/info/chrome.rs:1213`) does
not change that (decision 4).

### F.5 Self-tests (`self_test`, `:806-879`)

Synthetic gpui-component and gpui-base roots in the temporary directory. The
fixtures below are written inline for brevity; in the files each item stands on
its own line — a `#[derive(…)]` on the line above its `pub struct`, and each
`impl … for …` starting a line — since `RENDER_IMPL` and the derive scan read
line by line from the line's start (`scripts/check_widget_coverage.py:199-201`,
`:512-520`).

- gpui-base: `lib.rs` declaring `pub mod grid; mod vl; mod fold; pub mod traits;
  pub mod plot; mod text; pub mod acts;` and `pub use vl::VList; pub use
  fold::Fold; pub use text::Txt;`; `grid/mod.rs` `mod g; pub use g::Grid;` and
  `grid/g.rs` `#[derive(IntoElement)] pub struct Grid;`; `vl.rs` `pub struct VList;
  impl IntoElement for VList {}`; `fold.rs` `pub struct Fold; impl RenderOnce for
  Fold {}`; `traits.rs` `pub trait Fold {}`; `plot/mod.rs` `mod label; pub use
  label::*;` and `plot/label.rs` `pub struct Txt;`; `text.rs` `pub enum Txt {}
  impl RenderOnce for Txt {}`; `acts.rs` `pub struct Act; impl RenderOnce for Act {}`.
- gpui-component: `lib.rs` declaring `pub mod grid; mod vl; pub mod traits; pub
  mod plot; pub mod hidden; pub(crate) mod acts { pub use gpui_base::acts::*; }`,
  `pub use vl::{VList};` and `pub use gpui_base::{Fold as Folder};`; `grid.rs`
  `pub use gpui_base::grid::*;`; `vl.rs` `pub use gpui_base::VList;`; `traits.rs`
  `pub use gpui_base::traits::Fold;`; `plot.rs` `pub use gpui_base::plot::*;`;
  `hidden.rs` `pub(crate) use gpui_base::Fold;`.
- Expected: `gpui_base_reexports` gives `["Folder", "Grid", "VList"]` — a module
  re-exported whole, a name through a private module re-exported at the root, a
  rename — and not `Fold` (the trait the path leads to), `Txt` (the struct does
  not render; the renderer is an enum elsewhere), `Act` (inside a `pub(crate)`
  module) or anything from `hidden.rs` (not `pub`).
- The path rule's other parts, each a case of its own on top of the roots above:
  - **a `pub mod` re-exported by name:** gpui-base `pub mod anim;` with `anim.rs`
    `pub struct Fade; impl RenderOnce for Fade {}`, gpui-component `pub use
    gpui_base::anim;` → `Fade` is found;
  - **an external crate:** gpui-base `vl.rs` also `pub use ropey::Rope;`,
    gpui-component `pub use gpui_base::Rope;` (with gpui-base's `lib.rs`
    `pub use vl::Rope;`)
    → nothing, and no `Failure`;
  - **`crate::`, `self::`, `super::`:** gpui-base `pub mod nest;` with `nest/mod.rs`
    `mod inner; pub use self::inner::Deep; pub use super::vl::VList as Again;` and
    `nest/inner.rs` `pub struct Deep; impl RenderOnce for Deep {}`, plus `lib.rs`
    `pub use crate::nest::Deep as Root2;`; gpui-component `pub use
    gpui_base::nest::{Deep, Again}; pub use gpui_base::Root2;` → `Deep`, `Again`
    and `Root2` are found;
  - **`#[path]`:** gpui-base `pub mod pathed;` with `pathed.rs` holding
    `#[path = "other/place.rs"] mod placed; pub use placed::Placed;` and
    `other/place.rs` `#[derive(IntoElement)] pub struct Placed;`; gpui-component
    `pub use gpui_base::pathed::Placed;` → `Placed` is found;
  - **a submodule of `x.rs`:** gpui-base `pub mod outer;` with `outer.rs`
    `mod sub; pub use sub::Sub;` and `outer/sub.rs` `pub struct Sub; impl
    IntoElement for Sub {}`; gpui-component `pub use gpui_base::outer::Sub;` →
    `Sub` is found;
  - **an inline module:** gpui-base `pub mod host;` with `host.rs` holding
    `mod inner { pub struct Nested; impl RenderOnce for Nested {} } pub use
    inner::Nested;`; gpui-component `pub use gpui_base::host::Nested;` → `Nested`
    is found;
  - **a `pub(crate) use`:** gpui-base `pub mod hid;` with `hid.rs` holding
    `pub(crate) use crate::fold::Fold;`; gpui-component `pub use
    gpui_base::hid::Fold;` → nothing, and no `Failure` (the name is not brought in,
    so it is an item the module does not offer; F.1 step 3 yields nothing for it);
  - **the `Failure`:** a gpui-component root with `pub use gpui_base::gone::X;`
    and no `gone` declared or brought in at gpui-base's root raises `Failure`; so
    does `pub use gpui_base::grid::nowhere::X;`.
- `shows_gpui` with `GPUI_VIA`: `grid_layout("g", &state, cx)` and
  `.child(grid_layout(` show `GridLayout`; `info::layout::grid_layout(t)`,
  `fn grid_layout(` and `demo::grid_layout_sample(` do not; `h_resizable(` shows
  `ResizablePanelGroup`; `[resizable_panel()` shows `ResizablePanel`;
  `.vertical_scrollbar(h)` and `.overflow_y_scrollbar()` show `Scrollbar` and
  `.scrollbar_show(` does not.

### F.6 Gates of Part F

`python3 scripts/check_widget_coverage.py` passes ("Every widget is shown or
excepted.", no stale exception) once Part A.1 is in, and its gpui line counts the
nine names (the report's `discovered` rises by nine from today's); `--self-test`
passes; each F.5 case fails when the part of the rule it names is removed
(checked once by hand, case by case, and reported in the commit body).

### F.7 `scripts/README.md`, the script's section (`:442-458`)

The section states the discovery rule and the exit-2 cases, which F.1 changes, so
Part F's commit updates it, as `89b9564b` and `2f0066a8` did when the exit
conditions changed before. Two edits, by quoted text:

- "deriving `IntoPlot`), then requires" becomes "deriving `IntoPlot`, and the
  gpui-base widgets gpui-component makes public by a `pub use gpui_base::…`,
  followed through gpui-base's own modules), then requires".
- "or a file the rules read has moved upstream." becomes "a file the rules read
  has moved upstream, or the re-export rule no longer matches gpui-base's layout."

Both new texts go into G.6's list; G.6 runs on Part F's commit for them. The rest
of the section stays.

---

## Part G — CHANGELOG, ROADMAP and the assets (N8)

### G.1 `CHANGELOG.md`, 0.7.1

- ***Added***, after the `SpeechIndicator` line (`:14`):

  > - **Showcases (gpui)**: the Layout page shows gpui-component 0.9.1's `GridLayout`, three cards that can be dragged and resized, whose Widget Info states the theme colours of its selection ring, focus ring, drop placeholder and resize grip and that its gap and motion are upstream's tokens, which no theme reaches; the Inputs page shows 0.9.0's liquid `SpeechWaveform` on a row of its own under the bars, over the same synthetic input; the Typography page's `Diff` marks a line with a `DiffDecoration` (0.9.0) in its default background, `warning` at 20 %, and its Markdown sample has a task list, whose boxes gpui-base edges in `muted_foreground` and, when checked, fills and edges in the theme's `link` colour with a check mark of its own in white or black, and whose Widget Info also states the hovered table row's `table_hover`.

  and, after it:

  > - **Development**: `scripts/check_widget_coverage.py` also discovers the gpui-base widgets gpui-component makes public, a module re-exported whole or an item re-exported by name, which it missed: `GridLayout`, now shown on the Layout page; `ResizablePanelGroup`, `ResizablePanel` and `Scrollbar`, which the gpui showcase already drew and which it now counts through the functions that build them (`grid_layout`, `h_resizable`, `resizable_panel`, the scroll bar extensions); and five parts and wrappers with no surface of their own, excepted with their reasons.

- ***Fixed***, after the last entry (`:30`):

  > - **native-theme-gpui**: the connector's `widgets::Checkbox` asks the application's icon resolver (gpui-component 0.9's `set_icon_resolver`) for its check mark's path, as gpui-component's own check box does, where the theme states no check-mark stroke, and so does a `widgets::Radio` where the theme states neither a radio dot nor a check-mark stroke; they drew gpui-component's path without asking. Where the theme states a stroke, the connector draws the mark itself, as before.

- **The README's entry** (`:16`) ends with: "The `list_item_selected` row and
  *Limits* say what gpui-component draws with `list.active_highlight` off — a
  selected row `accent` under `accent_foreground` — and that a selected cell's
  overlay is edged 1 px in `table_active_border`; the crate documentation names
  the editor's status colours that stay upstream's highlight scheme (a hint's in
  both modes, a warning's in light mode, backgrounds and borders in dark mode,
  and 0.9's `created`, `modified` and `deleted`)."
- Nothing for comments, citations, the backlog or the lock (rationale N8). The
  coverage gate's line is decision 11's.

### G.2 `ROADMAP.md`

`:94`, "The showcase shows 0.9.1's `SpeechIndicator`." becomes "The showcase shows
0.9.1's `SpeechIndicator` and `GridLayout`, the liquid `SpeechWaveform`, a `Diff`
decoration and a Markdown task list." The upstream list is D.5's.

`:264`, "[`docs/todo.md`](docs/todo.md) tags 29 open items `[v0.7.x]`", is already
not the count: `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md` gives 41 at
`0fa28027` (read 2026-11-14). This work opens two such entries (Part H's and
D.3's *gpui: the widgets' own icons follow the theme's icon set*) and closes
none (D.3's closed entries are all `[upstream]`). G.2's commit comes after both
(*Order and gates*, (iv)) and replaces 29 with the number that grep gives on the
tree it commits, run then, with its output in the commit body; the number is
not computed here from a delta, since other work of this cycle may change the
base first. The sentence's other words stay.

### G.3 The connector's README

Only B.4's and B.5's sentences change in substance (rationale N8). Part E also
renumbers the README's upstream citations and relabels their versions (E.2's
scope holds `connectors/native-theme-gpui/README.md`).

### G.4 The asset run

The sources of the showcases (Parts A–C, E) and the lock (D.6) change, so both
release stamps are stale until the run. It runs once, after the last change of all
v0.7.1 work (Open point 1), as the KDE specification runs it
(`docs/archive/todo_v0.7.1_kde-hand-overrides-spec.md`, §E.3):

1. `cargo audit` passes.
2. `scripts/update_compatibility.sh run` passes; it leaves `docs/COMPATIBILITY.toml`
   and the three connector READMEs changed and uncommitted until step 7.
3. Once step 2 has passed, the branch is pushed, and CI runs on it
   (`docs/todo_v0.7.1_batch-speed-rationale.md`, S4).
4. While CI runs, the full `scripts/check_release.sh` shows only the provenance
   stamp failing and the known `cargo outdated` warning (zbus excluded,
   `0fa28027`). Step 6 starts only when both CI and this check are green.
5. The three showcases are built in advance with the capture scripts' commands
   (`--release` for gpui, `--release --features iced_aw` for iced,
   `--release --all-features` for egui).
6. The maintainer says when the desktop can be left unlocked and untouched (4 to 6
   minutes). `scripts/generate_assets_release.sh` runs once; it requires the
   branch pushed and the stamped paths clean, triggers the macOS and Windows
   screenshot workflow, captures the Linux screenshots and GIFs, checks parity,
   writes both stamps and the READMEs' "Verified" lines. If it stops, rule 15
   (batch-speed rationale, Appendix) says what follows.
7. One commit holds the assets, both stamps, the READMEs' lines and the 0.7.1
   heading's date (that commit's day). It is pushed.
8. The full `scripts/check_release.sh` passes at that commit, and CI is green.

The screenshots that change: the gpui Layout page's (the grid, at the page's end,
shows only where a capture scrolls to it), the Inputs and Typography pages'
where they show the new rows, and whatever the lock's taffy moves (D.6).

### G.5 The per-upgrade note in `docs/todo_gpui-full-theme.md`

The 0.8.0 and 0.8.1 upgrades each appended a note to that file, which records
which upstream version its theme map was checked against
(`docs/todo_gpui-full-theme.md:724`, `:754`; the reason,
`docs/archive/todo_v0.7.0_gpui-kit-0.8.1-rationale.md:1177-1178`; added by
`edd27b0b`), and the file still says "the citations below are at 0.8.1"
(`:756-758`). Appended at its end (after `:782`), in their form; every statement
was read in the 0.9.1 sources on 2026-11-14:

```markdown
## Note: gpui-component 0.9.1

Appended <date>; the text above is left as written. In v0.7.1 the connector
moves on to gpui-component, gpui-base and gpui-kit 0.9.1, on gpui-pre 0.3.12 or
GPUI Fast 0.1.14 or later, and the citations below are at 0.9.1.

- **`ThemeColor` is unchanged.** It keeps the same 139 fields as 0.8.1, and the
  base-palette fields of `ThemeConfigColors` are still private
  (`theme/schema.rs:657-691`). The rows above stand.
- **`Theme` gains two fields.** `window_radius`, the corner radius of a
  client-decorated window (`theme/mod.rs:192-193`; in `ThemeConfig` too,
  `theme/schema.rs:76`, applied at `:1117-1118`), which `WindowBorder` rounds
  the window with (`window_border.rs:20-29`) and which the connector sets from
  `window.border.corner_radius` in `to_theme` and `to_theme_config`; and
  `cursor_blink`, the caret's blink (`theme/mod.rs:218-220`), the application's
  setting, which `apply` keeps as it keeps `motion`.
- **`WindowBorder` draws its frame in the theme's colour.** It reads
  `window_border` (`window_border.rs:201`), which the connector writes from
  `window.border.color`, where 0.8.1 picked one of two literal greys; its shadow
  is still two literal black layers (`:253-286`).
- **The editor's highlight scheme gains `created`, `modified` and `deleted`**
  status colours (`highlighter/registry.rs:350-355`), which no widget of
  gpui-component or gpui-base reads; with the scheme's other stated status
  colours they stay upstream's (`docs/todo.md`, *gpui: the editor's status
  colours are partly upstream's scheme*).
```

The file is not in Part E's scope: its earlier notes keep the versions they
name, and only this note speaks of 0.9.1. Check: each cited line is opened in
the 0.9.1 source as the note is written, and the commit body lists them with each
line's text.

### G.6 A check for the texts no gate reads

It covers the new texts this specification quotes and no test reads for their
wording:

- G.1's three lines, G.2's sentence, G.5's note, Part H's section;
- B.2's comment, B.4's README cell and its `GEOMETRY_NOTES` text, B.5, B.6, B.7,
  B.8's `rows` note, B.9's comment and its `SC/info/layout.rs` note, B.10's two
  texts, B.13's comment, B.15's citation, B.1's note (the prose gate reads only the
  citations of B.1's, B.4's, B.8's and B.9's notes, not their wording);
- where B.3 lands, its three texts: the `own_icons` doc comment, the
  `SC/tests.rs:9262-9266` doc comment, and the `own_icons` text at
  `SC/info/mod.rs:446`, its quote split at "…" in place of the `{icons}`
  placeholder (the prose gate's `note_calls`, `CN/src/showcase.rs:2430`, reads a
  note call's arguments, not the text `own_icons` formats);
- A's new doc comments, their comment lines only (rustfmt may lay out the code
  under them otherwise): `GridCard` and `GRID_LAYOUT_ITEMS`, `grid_layout_sample`,
  `WaveformKind`, `DIFF_DECORATED_PATH` and `DIFF_DECORATED_LINE`; Part C's test
  comment;
- D.3's open icons entry, D.4's lines and E.5's relabelled lines outside E.2's
  scope;
- F.7's two `scripts/README.md` texts;
- F.3's four new `GPUI_VIA` comments and its dated line in `docs/todo.md`, and
  F.4's group comment and five reasons (the coverage gate checks only that a
  reason is non-empty, `scripts/check_widget_coverage.py:763-767`), checked on
  Part F's commit.

Each text is checked on the commit that writes it; B.14 and the other parts'
gates run this check for the texts of their own commit, by this list.

A text the specification gives only in part — A's Widget Info rows other than
quoted notes, D.2's and D.3's lines with their placeholders read from a run — is
left to the reviewer's reading of the diff. The quoted
texts are wrapped differently from their files, some are elided with "…", and
some sit in comment lines, so the check compares normalised text: in both the
file and the quote, a line's leading `//! `, `/// `, `// `, `# ` or `> ` is
stripped, the lines are joined and runs of whitespace collapsed; the quote is
split at each "…", and its parts must occur in the file in that order. The quote
is the new text as specified, without the specification's outer quotation marks,
every placeholder in angle brackets (`<date>`, `<sha>`, `<the version D.6 read
back>`, `<… lines, by D.2's rule>` and the like) filled in with what the task
wrote there, or, where a placeholder stands for text not known in advance, the
quote split there with "…". The quotes and the script are kept in
`/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/checks/`
(D.3's `check_moves.py` beside them), never in the repository. The script, run
from the worktree's root as
`python3 -I /home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/checks/check_text.py <file> <quote-file>`:

```python
import sys

PREFIXES = ("//! ", "/// ", "// ", "# ", "> ")

def norm(text):
    out = []
    for line in text.splitlines():
        line = line.strip()
        for prefix in PREFIXES:
            if line.startswith(prefix) or line == prefix.strip():
                line = line[len(prefix):]
                break
        out.append(line)
    return " ".join(" ".join(out).split())

with open(sys.argv[1], encoding="utf-8") as f:
    text = norm(f.read())
with open(sys.argv[2], encoding="utf-8") as f:
    quote = norm(f.read())
pos = 0
for part in (p.strip() for p in quote.split("…")):
    if not part:
        continue
    at = text.find(part, pos)
    if at < 0:
        print("MISSING:", part[:100])
        sys.exit(1)
    pos = at + len(part)
print("ok")
```

Its output for each text goes into the commit body. **Rewrapping:** an edited
paragraph is rewrapped to the width of its file's paragraph as it was, its longest
line (measured 2026-11-14): `CN/README.md:610-629` 80 columns, `ROADMAP.md:87-110`
78, `docs/issues/0004_kde-hand-overrides-upstream-requests.md:28-38` 78,
`docs/showcase-exceptions.toml:1-24` 85, the `docs/todo.md` entries of D.4
(`:2424-2456`) 88, `docs/todo_gpui-full-theme.md`'s notes (`:754-782`) 79,
`CN/proposals/README.md:67-77` 78, `scripts/README.md:444-458` 78; a Rust comment keeps the width of the comment
block it is in, as rustfmt leaves comments unwrapped. A table cell (`CN/README.md:368`,
`CN/src/lib.rs:116`) and a CHANGELOG entry stay on one line, as they are.

---

## Part H — The editor's status colours (N9)

`docs/todo.md`, a new section after *gpui showcase: look at the overlays that
open on input*, whose item ends at `:4246`: the new section follows its blank
line, before the `---` at `:4248` that closes `## Toolkit Connectors`:

```markdown
### gpui: the editor's status colours are partly upstream's scheme

- [ ] [v0.7.x] The connector installs gpui-component's default highlight scheme
      and replaces only the editor's fill and its gutter's
      (`connectors/native-theme-gpui/src/config.rs`, `highlight_style`). The scheme
      states status colours of its own (gpui-component 0.9.1
      `src/theme/default-theme.json:113-128`, `:315-340`): in light mode `warning`
      (#C99401), `hint` (#9e5dff), `created`, `modified` and `deleted`; in dark mode
      `hint`, `created`, `modified`, `deleted`, the backgrounds of `error`, `hint`,
      `info`, `success` and `warning`, and the borders of `error`, `hint`, `info`
      and `warning`. A colour the scheme leaves
      unstated falls back (`src/highlighter/registry.rs:358-463`): `error`,
      `warning`, `info` and `success` to the theme's `red`, `yellow`, `blue` and
      `green`, which the connector maps from the platform's danger, warning, info
      and success colours (`connectors/native-theme-gpui/src/colors.rs`,
      `assign_base_colors`); `hint` to the theme's `cyan`, which the same
      function derives; a
      background to the window's background blended with its colour at 0.2; a
      border to its colour; `created`, `modified` and `deleted` to `success`,
      `warning` and `error`. So an editor's warning diagnostics are upstream's
      #C99401 in light mode and the platform's warning colour in dark mode, and
      the three colours 0.9.0 added are upstream's in both. The diagnostic
      underline takes `error`, `warning`, `info` and `hint`
      (`src/input/input.rs:570-575`), and the diagnostic popover letters its text in
      the severity's colour on that colour's background, framed in its border
      (`src/input/popovers/diagnostic_popover.rs:42-59`,
      `src/highlighter/diagnostic_styles.rs:6-34`); `success`, `created`,
      `modified` and `deleted` are read by no widget. The showcase shows no
      diagnostic.
      The status colours are one public field with a default
      (`HighlightThemeStyle::status`, `registry.rs:485-486`; `StatusColors` derives
      `Default`, `:318`). Computed from the presets' colours on a white background
      (`docs/todo_v0.7.1_gpui-kit-0.9.1-rationale.md` at <sha>, N9; not on each preset's own
      window colour): clearing them all makes `hint` the derived cyan, its contrast
      against white falling from 3.84 (upstream's purple) to 1.57 on adwaita, 1.50
      on kde-breeze and 1.48 on macOS, and to about 1.4 on its own tint, where the
      popover letters it. Clearing all but `hint`'s three (through the style's
      serialised form, since the single colours are private) changes one colour a
      widget draws in light mode, `warning` lettered on its tint: upstream's reaches
      2.27; the platform's warning colour gives 1.86 on adwaita, 1.87 and 1.95 on
      the two macOS presets, 1.43 on nord, 2.30 on kde-breeze, 3.98 on windows-11
      and 4.82 on material-3. In dark mode both are mixed: `info` gains (the stated
      `info.background`, #0059D1, lies under the platform's info text today), and
      so does `hint` when all are cleared; `warning` and `error` lose a little. The model's warning
      colour is made for a filled badge or an icon and has no counterpart for a
      diagnostic's text. To do: show one diagnostic of each severity in the gpui
      showcase's editor, judge both modes, and then decide between keeping the
      scheme's colours, clearing them all or all but `hint`'s, or a model colour for
      diagnostic text. Until then the connector's documentation says the status
      colours are partly upstream's (`config.rs`, `highlight_style`; the crate
      documentation's `input` row).
```

The figures are the rationale's (N9, computed for it; not recomputed here). The
entry cites the rationale at `<sha>`, commit (0) of *Order and gates*, which adds it to the repository,
since the rationale moves to `docs/archive/` when this work is archived; no text
the showcase shows and no source comment of this specification names a document
of this cycle by path (B.3, B.10).
Gates: `python3 scripts/check_todo_citations.py` passes; the check of G.6
(the text in the file is the text above) passes.

---

## Part I — Three gates find facts rows by text (S6)

The maintainer's S6 (`docs/todo_v0.7.1_batch-speed-rationale.md`): a citation the
batch writes names no line of a file that changes, the old ones are left to drift,
and three gates that compare a line number with the line it names stop doing so,
since a row added above the cited one turns them red. Unit LN, after GS and before
E1 (*Order and gates*). Its files: `native-theme/tests/platform_facts_xref.rs`,
`native-theme/src/presets/documented_sizes.rs`, `scripts/check_preset_citations.py`,
the checker's comment and step label in `scripts/check_release.sh`, CI's comment on
it in `.github/workflows/ci.yml`, its section of `scripts/README.md`, one clause of
`docs/todo.md` and one line of `CHANGELOG.md`: eight files, none in E.2's scope. No
new text names a document of this cycle.

### I.1 `native-theme/tests/platform_facts_xref.rs`

In `macos_tahoe_probe_sizes_match_platform_facts` and
`macos_probe_sizes_match_platform_facts` the closure `line` (`PLATFORM_FACTS.lines().nth(n - 1)`)
goes. One helper beside `PLATFORM_FACTS`, `fn facts_row_has(parts: &[&str]) -> bool`,
true where one line of the facts file holds every part
(`PLATFORM_FACTS.lines().any(|l| parts.iter().all(|p| l.contains(p)))`), so a row is
named by its own text and the value it must give, not by its position; each assert
says in its message which row it reads (section and property). The asserts:

| Today | Becomes `facts_row_has(&[…])` with |
|---|---|
| `line(1183)` "macOS 26: 24 **(API)**" | "NSButton intrinsic: 20 **(API)**", "macOS 26: 24 **(API)**" |
| `line(1212)` "macOS 26: 24 **(API)**" | "NSTextField intrinsic: 21 **(API)**", "macOS 26: 24 **(API)**" |
| `line(1616)` "macOS 26: 24 **(API)**" | "NSPopUpButton: 20 **(API)**", "macOS 26: 24 **(API)**" |
| `line(1468)` "macOS 26: same" | "NSSplitView divider: 9 **(API)**", "macOS 26: same" |
| `line(1469)` "macOS 26: same" | "clear for the `.thick` style", "macOS 26: same" |
| `line(1468)` "NSSplitView divider: 9 **(API)**" | "NSSplitView divider: 9 **(API)**" |
| `line(1469)` "clear for the `.thick` style" | "clear for the `.thick` style" |
| `line(1183)` "macOS 14–15: NSButton intrinsic: 20 **(API)**" | "macOS 14–15: NSButton intrinsic: 20 **(API)**" |
| `line(1212)` "macOS 14–15: NSTextField intrinsic: 21 **(API)**" | "macOS 14–15: NSTextField intrinsic: 21 **(API)**" |

Read 2026-11-15 at `0fa28027`: each pair of the first five stands together on one
line only (an `awk` over the file), while "macOS 26: 24 **(API)**" stands on 6 lines
and "macOS 26: same" on 194, so the pairs keep the tests as strict as the line
numbers made them. The two comments of the second test that cite lines
("§2.17 (platform-facts.md:1468-1469)", "§2.3 (platform-facts.md:1183) and §2.4
(:1212)") name the sections and rows without the numbers.

### I.2 `native-theme/src/presets/documented_sizes.rs`

`facts_line` and `facts_heading` go. In their place one helper,
`fn facts_row(section: &str, key: &str) -> Option<&'static str>`: the first line that
starts with `` | `<key>` `` inside the section whose heading starts with
`### <section> `, up to the next line starting with `### ` (a `####` heading does not
end it), and `None` where there is no such section or row. The checks:

- `every_citation_names_its_platform_facts_row` becomes
  `every_row_names_rows_of_its_section`: for each row, `section(widget)`'s
  `border.padding_horizontal` and `border.padding_vertical` rows exist, and each
  `extra` field's `facts_row(section, field)` exists. (Read 2026-11-15: every section
  the rows name has both padding rows, and every extra field and every
  `field_cell` key below has exactly one row in its section.)
- `every_field_citation_names_its_platform_facts_row` becomes
  `every_field_row_names_a_row_of_its_section`: `facts_row(section, key)` exists for
  each `field_cell` pair.
- The four fixed asserts: `facts_line(1683)` and `facts_line(1686)` become
  `facts_row("2.27", "arrow_side")` and `facts_row("2.27", "frame_enabled")`;
  `facts_line(1290)` and `facts_heading(1277)` `facts_row("2.7", "show_delay")`;
  `facts_line(1569)` and `facts_heading(1556)` `facts_row("2.22", "entrance")`.

`soft_field_failures` keeps its five parameters (`field`, `line`, `want`, `resolved`,
`stated` at `0fa28027`), its second, `line: usize`, becoming
`row: &str`, so that its message reads `docs/platform-facts.md {row}` in place of
`docs/platform-facts.md:{line}`; its two callers pass `"§2.7 show_delay"` and
`"§2.22 entrance"` in place of `1277` and `1556` (both rows read 2026-11-15 at
`0fa28027`: `` | `show_delay` `` once inside §2.7 and `` | `entrance` `` once inside
§2.22). Its doc comment's "against platform-facts' row at `line`" becomes "against
the platform-facts row `row` names". The caret specification's B.2, which adds a
sixth parameter, calls it the same way.

The other line numbers in the tables (`Row::lines`, the `extra` triples,
`FieldRow::line`) and in the other failure messages stay as written:
they are old citations (S6), now read only by the messages, and no check compares
them. The doc comments that say a line is checked against its row are reworded to
say it is a citation; the module doc's "Each row cites the platform-facts lines it
reads" stays.

### I.3 `scripts/check_preset_citations.py`

**The new form.** A new preset comment cites the facts file as today's form does
with the line number removed: `# platform-facts.md (§2.3): …` above the value it
cites (today `# platform-facts.md:1192 (§2.3): …`), and, for a key the preset does
not give, `# K not stated: platform-facts.md (§2.3), …`. The plans of KDE steps 2
and 3, steps 4 and 5, the caret and steps 6 and 7 write their prescribed preset
comments in this form, as this release's KDE step 1 does (its B.4).

The checker accepts both forms and compares nothing with the facts file's rows
any more, old citation or new: the smallest change that keeps the gate green
while old numbers drift (the maintainer's S6). What it keeps is what reads no line
position:

- **Read.** `CITATION` also reads a citation with no line,
  `platform-facts.md (§2.6)` or `platform-facts.md (§2.6, §2.15)`; a comment line
  that starts with it is a value citation as one that starts with
  `platform-facts.md:` is (rule "Subjects"), and the `K not stated: …` form is a
  named key as today (`NAMED`). Neither form is reported for its number or its
  lack of one.
- **Kept** (read in the script 2026-11-15): the input errors, exit 2 (a preset
  that is not UTF-8 or not TOML, a facts file with no Chapter 2 rows, and a preset
  that cites the facts file where the rule reads no citation, now in either form:
  `findings`' `platform-facts\.md:\d` test widened to the new form); the reading
  of each citation and its kind (`named`, the value below with `MARKER`'s marks,
  or rows), counted per preset on stderr; the self-test.
- **Gone:** every comparison, since each takes its rows from the cited lines: the
  findings "the cited line is not a row", "`row` is in §x, not §y", "`row`, not
  the value", "`row`, not the key the comment names" and "`row` names no value the
  citation covers". What served only them goes with them, so that no dead code is
  left: `Facts`' row and section maps (it keeps what its "no Chapter 2 rows" error
  needs), `cited_lines` and `PART`, `names`,
  `subject_names`, `present` and `covered`, `BARE_PARTS`, `SIDE_TO_AXIS` and
  `STATES` (the name and cross-row rules), and `EXCEPTIONS` with
  `SEVERITY_TEXT_RANGE` and the unused-exception finding, since an exception
  excuses a comparison. `covered` also decides a citation's kind today
  (`check_preset_citations.py:362-364`: a value citation with no covered value
  becomes rows); it is replaced there by its guard, so that a citation counts as
  a value citation where `VALUE.match(below) and not HEADER.fullmatch(below)`
  holds, which is where `covered` returns a value. The kinds stay today's, so
  I.5's count comparison holds. An implementer who finds another part used only by them
  removes it too and names it in the commit body. With no finding left, the
  checker prints no finding line: it exits 0 on presets it can read and 2 on one
  it cannot; the per-preset counts are its citations by kind.
- **Docstring and self-test** say and test this: a value citation and a named one
  in each form, read and counted; a `K not stated:` citation of the new form; the
  input errors, the new form among them. The docstring names the third form, which
  it does not read (below).

`scripts/check_release.sh`'s step label "data documents: preset comments cite their
platform-facts rows" (`:980-981` at `0fa28027`) becomes "data documents: preset
comments' platform-facts citations read". Its comment ("on a preset comment whose
`platform-facts.md:N` row is not the value below it") and `scripts/README.md`'s
section *check_preset_citations.py* state the new rule: the checker reads the
presets' facts citations of the two forms above, `platform-facts.md:N (§x.y)` and
`platform-facts.md (§x.y)`, and fails on a preset it cannot read; it no longer
compares a citation with the facts file. The docstring and the README also say
that a third form, `# platform-facts.md §x.y <key>: …` with neither a number nor
parentheses, is read by neither this checker nor another; they name it without a
count. (58 preset comments cite `platform-facts.md §`, counted 2026-11-15 at
`0fa28027`; the count is this specification's alone, since no gate reads it and it
would drift.) `CITATION` is not widened to it. The README's account of how 52 of material-3's comments came to
cite a neighbouring row goes with the comparison it explained.

`.github/workflows/ci.yml`'s comment on the data documents' checks ends "a preset
comment whose platform-facts row is not the value below it" (`:415-416` at
`0fa28027`); that clause becomes "a preset whose platform-facts citations cannot be
read".

`docs/todo.md`'s entry *A machine-readable platform-facts* says the documented-sizes
gate "cites the line; a companion test checks the citation lands on the right row,
but not that the number is the one written there"; the words "a companion test
checks the citation lands on the right row" become "a companion test checks that
each row's section has the rows it names", and ", but not that the number is the
one written there" and the rest of the entry stay as they stand.

### I.4 `CHANGELOG.md`

The 0.7.1 section's `### Changed` gains, after its last entry:

```
- **Development**: no check reads `docs/platform-facts.md` by line number any more, so a row added above another no longer turns one red: native-theme's documented-size gate and macOS probe tests look a row up by its section, name and text, and `scripts/check_preset_citations.py` reads the presets' facts citations of the forms `platform-facts.md:N (§x.y)` and `platform-facts.md (§x.y)`, the second naming no line, without comparing them with the facts file's rows.
```

### I.5 Gates of Part I

- **Each check still fails on a missing text**, each by one temporary edit of
  `docs/platform-facts.md` or a preset, seen red and then restored:
  `macos_probe_sizes_match_platform_facts` with "NSButton intrinsic: 20" changed in
  the facts file; `macos_tahoe_probe_sizes_match_platform_facts` with the
  NSPopUpButton row's "macOS 26: 24" changed; `every_row_names_rows_of_its_section`
  with §2.13's `` `bar_height` `` row renamed;
  `every_field_row_names_a_row_of_its_section` with §2.9's `` `track_height` `` row
  renamed; `native_themes_state_the_dialog_entrance` with §2.22's `` `entrance` `` row
  renamed. Each run's failing line goes into the commit body.
  `check_preset_citations.py` compares no facts text; its `--self-test` is seen
  to fail once with `CITATION` reading only today's form (a temporary edit of the
  script), and its output goes into the commit body.
- **A row added above no longer turns any red:** one line inserted as the first
  line of `docs/platform-facts.md` (temporary):
  `cargo test -p native-theme --test platform_facts_xref`,
  `cargo test -p native-theme --lib documented_sizes` and
  `python3 scripts/check_preset_citations.py` pass; restored after.
- **The new form passes:** two comments of `native-theme/src/presets/kde-breeze.toml`
  rewritten in the new form (temporary), one value citation
  (`# platform-facts.md (§2.x): …`) and one `K not stated: platform-facts.md (§2.x), …`
  of the same rows: `python3 scripts/check_preset_citations.py` exits 0, and
  kde-breeze's counts show both read, its citations count and each kind's count
  unchanged from the run before the edit; restored after.
- After the restores `git diff --quiet -- docs/platform-facts.md native-theme/src/presets/*.toml`
  exits 0.
- On the unit's tree: the two test runs above, `python3 scripts/check_preset_citations.py`
  (no finding, its counts in the commit body) and `--self-test` (`self-test ok`),
  clippy `-D warnings` and `scripts/check_strict_panic.sh` for `native-theme`,
  `cargo fmt --all --check`, and the other four documentation checkers, among them
  `python3 scripts/check_todo_citations.py` for the `docs/todo.md` clause.
- The gate the plan gives the unit: it changes a gate script's comment and step
  label (`scripts/check_release.sh`), so it is structural (rule 24 (batch-speed rationale,
  Appendix)) and takes `--commit`.

---

## Order and gates

**Order.**

**The commits, in order:**

- **Within v0.7.1** (settled by the session): this port's work lands before the
  KDE step 1 specification's (`docs/todo_v0.7.1_kde-open-items-step1-spec.md`),
  so that Part E's renumbering applies to a tree in which no line that step writes
  at 0.9.1 exists yet. The documents both edit — the CHANGELOG's 0.7.1 entry,
  `ROADMAP.md`, `docs/todo.md` — are edited by each by quoted text. Where both
  land in v0.7.1, one asset run (G.4) follows both.
- (0) The work's first commit adds the documents of this cycle the maintainer has
  approved, under `docs/` — of the five rationales (the batch-speed rationale
  among them), the specifications and the
  plans — before any other change. An approved icons rationale is committed
  with them, whichever release its work lands in. A document not approved when the work
  starts stays untracked and outside the work. A text that cites one of them
  "at `<sha>`" (D.3's `:3959` and `:3972`, Part H) cites this commit. "A clean
  tree" (E.4 step 1) means no change to tracked files
  (`git status --porcelain --untracked-files=no` prints nothing); an untracked
  document cannot reach the run, which lists its files with `git ls-files`
  (`remap.py:68-69`, `gates.py:431`), and E.2's pathspecs hold no document but
  `docs/showcase-exceptions.toml`.
- (0a) The gate-speed unit (GS, `docs/todo_v0.7.1_gate-speed-plan.md`), then
  unit LN (Part I), each on the tree before it.
- (i) Part E's digits-only commit (E.1–E.4), on the tree LN leaves after
  commit (0) and GS, whose files in E.2's scope are commit (0)'s (E.4
  step 1; decision 3). If a gate fails there that E.4 does not name as expected, the
  work does not go on: where the failing test compares a renumbered number, the
  write is undone first (`git checkout HEAD -- <E.2's pathspecs>`, which
  restores the scope's tracked files to `BASE_REV`'s, commit (0)'s, staged or not), that number is added to `remap.py`'s
  `REVIEW_ASIDE`, the run repeated from E.4 step 2, and the case written into the
  commit body; where the failure is not a renumbered
  number, the work stops and reports.
- (ii) Parts A to D, each in its own commits, in the order of the points below;
  the open entry *gpui: the widgets' own icons follow the theme's icon set*
  (D.3, `:3972`) in a commit of its own before D.3's moves; D.3's moves in a
  commit of their own (D.3's check). E.5's pinned-lines check runs on B.13's and
  B.9's commits.
- (iii) Part E's hand-read commit, E.5 with B.7, after all of (ii): E.5's taffy
  item needs D.6. E.5's rule for files with pinned lines applies to this commit.
- (iv) Part G's commits (G.1, G.2, G.5; G.3 makes no edit of its own), after
  (iii), each checked by G.6's check (a check, not a commit); then G.4's asset
  commit.
- Part F's and Part H's commits fall in (ii): Part F's after A.1, Part H's
  anywhere before the open icons entry's commit (point 5).

**The points that fix the order inside (ii):**

1. Part E's hand-read is commit (iii), not done beside Parts A to D; a part that
   edits a file of E.2's scope after (i) locates its text by content.
2. Parts A, B, C and D.6 are independent of each other. Within A, A.1 before
   Part F (F's gate needs the `GridLayout` section). B.13 is independent. C's
   outcome decides D.3's `:3959`.
3. Part D.2–D.5 after E.4 (they need the copy's `lookup.py`), and D.3's `:2187`
   and `:4140` after D.6.
4. Part F after A.1.
5. Part H any time before the open entry *gpui: the widgets' own icons follow
   the theme's icon set* (D.3), which goes after Part H's section; B.10's
   sentences name Part H's heading and B.3's doc comment names the open entry,
   so point 8 orders them.
6. E.5's taffy item in commit (iii), as above.
7. Part G last, as (iv): G.1–G.3 and G.5, each with G.6's check, once the parts they describe are
   in (Parts A to D and F, and B.13 for G.1's *Fixed* line); G.4 after everything
   that lands in v0.7.1. B.3 lands unless the icons work lands in v0.7.1 (Open
   point 1).
8. Each commit is true when it lands. B.3's doc comment names D.3's open entry
   *gpui: the widgets' own icons follow the theme's icon set*, and B.10's two
   texts name Part H's heading, *gpui: the editor's status colours are partly
   upstream's scheme*; so Part H's section and D.3's open entry are written in
   or before the commit that writes B.3's and B.10's texts (Part H's first, by
   point 5).

**The checks that prove the whole.**

- On both cores: `cargo test -p native-theme-gpui --all-targets` (gpui-pre with
  `native,watch`, GPUI Fast with `gpui-fast,native,watch`) and the doctests on
  gpui-pre; the showcase suites through `scripts/run_in_runner_env.sh`
  (`scripts/check_release.sh:845-855`); clippy `-D warnings` on both cores and
  with `--no-default-features`; the strict panic lints on the library and the
  showcase (`scripts/check_strict_panic.sh`, `:624-632`).
- `python3 scripts/check_widget_coverage.py` and `--self-test`;
  `scripts/check_connector_parity.py`, `scripts/check_readme_versions.py` and
  `scripts/check_showcase_parity.py --check-list` (each with `--self-test`), as
  `scripts/check_release.sh:913-943` runs them.
- The five documentation checkers (D.7).
- The batch's cadence (`docs/todo_v0.7.1_batch-speed-rationale.md`, S1) in
  place of `scripts/check_release.sh --commit` at each part's last commit:
  during a unit, the agent runs only the touched tests (its targeted commands);
  at a unit's end, `--quick`, by the orchestrator, as a long run until its time
  is measured (the first timed into the record); `--commit` every third code
  unit of the release, counted from the last `--commit`, on the release's last
  code unit (before its documents and version units; here v0.7.1's last),
  after a lock or toolchain change (the warm-up gate; D.6's commits), and
  wherever the plan names a unit structural; a documents-only unit runs the five
  documentation checkers and `--quick`; the full run where the plan has it (the
  close-out), unchanged; the plan names each unit's gate. The full
  `scripts/check_release.sh` at G.4's last commit with no failure. Under the
  gate-speed specification's O3 as the maintainer answered it (2026-11-15, the
  keyed option; `docs/todo_v0.7.1_gate-speed-spec.md`, Part M), CI's MSRV checks
  run in `--commit` and in `--quick` of the ported script, the two `+1.88.0`
  checks in `--quick` where anything their key follows changed (their crates'
  reach and included files, the files outside the crates but `QUICK_UNREAD`, the
  tools, the environment, their toolchain), and in CI;
  the full run is unchanged (O1a no). The gpui connector's MSRV stays 1.95.0;
  nothing here needs a newer language feature. The briefs' rule stays.
- CI on the branch green, *Test the gpui connector on GPUI Fast* and the macOS
  and Windows GPUI Fast builds included.

Nothing is merged, tagged, published or released by this specification; those
are the maintainer's, after G.4.
