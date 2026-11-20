# 0.7.2: issue #4's leftovers (the three looks and the smaller Breeze details) — specification

- Rationale: [`../issues/0004_kde-leftovers-rationale.md`](../issues/0004_kde-leftovers-rationale.md)
  ("the rationale"), decisions E1–E14; the maintainer asked for this specification and its plan on
  2026-11-20, after reading the rationale. It decides nothing the rationale did not decide, except
  the points under *Decisions taken while specifying*; *Corrections to the rationale* lists the
  facts the reading found otherwise. Status: written 2026-11-20; not yet approved.
- **The release (D0 (a) of the open-items rationale).** Part of 0.7.2, as steps 2 to 7, the
  widget icons and the caret's blink are: the version stays 0.7.2, its CHANGELOG names the break
  once, and 0.7.2's close-out is made again after these units, once.
- Read on `v0.7.2-rc1` at `d5df78d9`, worktree `.claude/worktrees/lane2-b` ("the tree"): egui
  0.37.0, gpui-component and gpui-base 0.10.0, gpui-pre 0.3.12, iced 0.14.0 with iced_widget
  0.14.2 and iced_aw 0.14.1. `<record>` is
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.3/`; this work's files are
  in `<record>/leftovers/`. Each part below names its own abbreviations of the toolkits' `src/`
  folders (`GC`, `GB`, `GP`, `IW`, `AW`, `IC`, `EG`, `EP`, `EX`) and of the connectors' folders.
- **Every edit is applied by its anchor** (a type, a function, a key, a test's name, a row's name,
  a quoted phrase). No repository file is cited by line number, except the gpui colour claims'
  lines, which a gate reads at their line (Part C, C.0); a pinned source (a crate at a version,
  Breeze at `f0b1d753`, qtbase v6.12.0) keeps its lines.
- Every value below was read from the file named, recomputed from the rationale's sources, or is
  marked not verified. Nothing was built while specifying; what a build must settle first is the
  trial T-L1 (Part C, C.1.3), the plan's first task.

## How to read it

Five parts by owner, each in the rationale's four units: **U1** the looks (E1, E3, E5), **U2**
menus, the rest (E2, E4), **U3** lists and controls (E6–E10), **U4** fonts and sizes (E11, E12).
Within a unit the parts are built in the order A, B, then C, D, E side by side. The decisions
below are binding on every part, and where a part's text says otherwise, they win.

| Part | Owns |
|---|---|
| A — the model and its gates | `native-theme/src/model/`, `native-theme/src/resolve/`, `native-theme/tests/proptest_roundtrip.rs`, the tests in `native-theme/src/presets.rs`, `docs/property-registry.toml`, `docs/inheritance-rules.toml`; in U4, the gate `native_themes_state_documented_fields` of `native-theme/src/presets/documented_sizes.rs` |
| B — kde-breeze, the KDE reader, the facts, outside the code | `native-theme/src/presets/kde-breeze.toml`, `native-theme/src/kde/`, the rows of `native-theme/src/presets/documented_sizes.rs`, `docs/platform-facts.md`, `CHANGELOG.md`, `ROADMAP.md`, the READMEs' counts, the rationale's status line, `docs/todo.md` |
| C — gpui | `connectors/native-theme-gpui/` and its showcase |
| D — iced | `connectors/native-theme-iced/` and its showcase |
| E — egui | `connectors/native-theme-egui/`, `connectors/native-theme-egui-widgets/`, the egui showcase |
| The orchestrator | the shared files: `docs/showcase-elements.toml`, `scripts/connector_parity_exceptions.toml`, `docs/showcase-exceptions.toml` (S12) |

## Decisions taken while specifying

Each was weighed in its part's draft (the options and their reasons are kept in
`<record>/leftovers/spec/`); the choice and the reason that decides it are given here.

- **S1 — Fallbacks live in the connectors, not the resolver.** Every "unset X is Y" of E2, E3,
  E4, E7, E8 and E10 is applied where a connector or showcase reads the field, as
  `list.header_active_background`'s fallback is today. A resolver fill would destroy the `None`
  that E2 ("where `active_background` is stated, no outline") and E3 ("with both unset, today's
  look") depend on, and none of these fallbacks has a source as an inheritance; so every new field
  goes into `[no_inheritance]`. E2's chain ends at `menu.hover_text_color`, a required field: an
  unset `active_text_color` is `hover_text_color`.
- **S2 — `menu.row_height` wins over `menu.min_row_height`** where a theme states both: written
  in both fields' docs and applied by the connectors; neither validation nor the resolver acts on
  it (an error would reject a valid overlay; clearing a field would change stated data). No bundled
  preset states both.
- **S3 — E11 in the model.** The rounding is applied in validation, where every resolution path
  turns a size into logical pixels (`validate_with_dpi`): one pass over the resolved fonts and the
  text-scale sizes after they are built, before the range checks (Part A, U4); a size it takes to 0
  is a range error. A font field added later cannot miss it: a test walks the serialised
  `ResolvedTheme` of a flagged theme and finds every font size whole. A type carrying the flag
  into each conversion would hold it at compile time, but would change what `native-theme-derive`
  emits for every widget, and the derive's output must not change in 0.7.x: the released
  native-theme 0.7.0 and 0.7.1 require the derive by caret (`version = "0.7.0"`, `"0.7.1"`), so
  they take derive 0.7.2 once it is published, and its expansions name `crate::resolve::*` paths
  of the core they were written for (`native-theme-derive/README.md`). Under the
  flag every size is rounded, a size stated in pixels too: the flag says "the sizes are Qt's", and
  Qt's pixel sizes are whole (the rationale's "after its conversion from points" names the case
  that occurs; no bundled preset states a fractional pixel size). Line heights are not rounded:
  they are not font sizes, and Qt's measured 10 pt line box, 18 px, is nearer the unrounded 18.13
  than 17.68. A connector's line box is its font size × `defaults.line_height`, so under the flag
  the body's is 13 × 1.36 = 17.68 while the display entry, the same 10 pt, keeps 18.13 (a
  text-scale line height is taken from the size before rounding). The flag is carried into the
  resolved theme as `focus_ring_keyboard_only` is (a soft option, one resolved leaf): on the
  `SystemTheme::from_system` path an application re-derives its sizes from `defined_size` at its
  own DPI (`resolve/context.rs`, `font_dpi`'s doc), and needs the flag to round them as Qt does,
  the case E11 chose a flag for.
- **S4 — E1's outline states.** The outline is drawn on a hovered item, on an item whose submenu
  is shown (Breeze keeps that item selected), and on a pressed item where
  `menu.active_background` is unset (E2's fallback is the hover look, outline included). gpui
  draws it as a box over the row shown under group hover and hidden under group active
  (`group_active`), so the row's padding never changes.
- **S5 — E5's outline goes on every row a showcase fills on hover today, in every showcase**:
  gpui's list, tree and files table; iced's list and command-palette rows; egui's list and
  command-palette rows. A row without a hover fill gets none: iced's and egui's Basic tables have
  no hovered row (their rows are containers, or have no hover fill); egui_extras' Data table fills
  a hovered row per cell with no stroke; gpui's command palette is gpui-component's `Command`,
  whose pointer selects the row it is on (`GC/command/state.rs:547-556`), and a selected row takes
  none. Adding a hover to them would change every preset's table, against the rationale's §2. One
  limit: gpui's Tree marks a right-clicked row after the row's closure built it, so that row, which
  has no hover fill, takes the outline under the pointer, as its label takes the hover text
  (`tree_row`'s comment). These differ only under the pointer: no parity entry (S12).
- **S6 — egui draws no pressed menu item.** egui draws a focused widget in `widgets.active` and
  moves the focus through a menu's items with the arrow keys (`EG/widget_style/mod.rs:212-222`,
  `EG/containers/menu.rs:233-288`). Written into `active`, Breeze's pressed look would stay on
  every keyboard-selected item, where Breeze draws the hover look; a press lasts a moment, a
  keyboard selection stays. So egui's menu role keeps `active` the hover look, outline included;
  `menu.active_background` and `menu.active_text_color` are listed unread in egui's mapping with
  that reason, and the README's limits say so. gpui and iced draw the pressed item. A per-call
  pressed look (writing `active` only while the primary button is down) would be showcase code
  an application must copy for a look that lasts a moment, while the role's look is what every
  application gets; not taken.
- **S7 — E8's slider outline, the same in every connector.**
  - Width unset: each connector's own outline as today (gpui's ring at half the fill's alpha,
    egui-widgets' state stroke, iced's none). A stated 0: no outline at all, the connector's own
    ring included. Stated above 0: the outline, in the state's colour.
  - Width stated, rest colour unset: no line at rest, the thumb shown whole at its outer size
    (iced's quad filled, its border 0; egui's stroke none, its disc `knob_radius`; gpui fills the
    ring of the width with the thumb's colour). No bundled preset reaches this case.
  - Hover means the pointer on the handle or the handle dragged, where the toolkit tells it apart
    (egui-widgets; gpui except while dragging, which gpui does not style); iced's `Hovered` and
    `Dragged` cover the whole slider. Hover wins over focus.
  - Focus is shown where the toolkit gives the slider the focus: egui on the keyboard only; gpui's
    case after a click is read in the build (*Not verified*) and stated in the doc; iced has no focused
    status and does not draw it. A range slider shows the focus on its end thumb.
  - Disabled: the rest colour, faded as the connector fades the disabled widget; no hover or focus.
- **S8 — E9 under the pointer and pressed:** `toolbar.button_hover_border_color` where stated,
  else the checked pen (iced, egui). gpui keeps the checked pen in every state: a selected
  gpui-component `Button` registers no hover or press style (`GC/button/button.rs:733`, `:758`),
  and a caller's own hover is refused (`GP/elements/div.rs:866-871`); the limit is in the README.
  The pen's width where no hover outline is stated stays the button's present width (gpui's
  `toolbar_button_rest` unchanged; no preset states the pen without the outline).
- **S9 — E3's titles.** An open title under the pointer takes the open fill. egui takes the bar's
  title style from a new `menu_bar_modifier` beside `role_modifier`, not from a new role (a role
  would move every role table and its tests for one container); egui's `MenuButton` has no open
  state for a bar title, so the open fill shows where the application swaps it in, as the showcase
  does (README limit). iced's page-tab caret menu keeps the window's colour through a small wrapper
  in `page_menu`, so `menu_bar_style` stays the connector's `styles::aw::menu`.
- **S10 — E4's geometry.** egui's menu popup takes its margin from the menu role's style (menus
  and submenus are styled by `MenuConfig::style`, then framed from `spacing.menu_margin`), so the
  menu role writes `menu.panel_padding_*` there; the egui showcase pads with
  `popover.border.padding` only on sides the menu leaves unstated. gpui's popup row gap is 0 where
  `menu.panel_padding_vertical` is stated; iced's Extra page keeps its title offset and puts no gap
  between rows where it is stated. iced's page-tab menu takes the shared row-height helper. In a
  right-to-left layout every connector pads by physical sides, as all their builders do, and says
  so in its doc: no showcase page draws a menu right to left.
- **S11 — Smaller points.** No connector style for iced menu items (only the showcase's
  `menu_row`; the README says so). iced's editable Theme combo box, focused, is drawn as today, not
  open (the model has no editable combo box, E10). kde-breeze's `tab.min_width` 80 is the outer box
  already: Breeze's `tabBarTabSizeFromContents` bounds the whole tab by `TabBar_TabMinWidth` and
  adds nothing after it (`st:3801-3851`); nothing changes. The new shortcut-text backlog entry is
  tagged `[v0.7.x]`. gpui's slider outline data lives in a new `ThumbOutline` beside
  `SliderLook`, which moves no cited line. gpui's `lib.rs`, `geometry.rs` and README citations of
  gpui-component 0.9.1 lines that 0.10.0 holds unchanged are re-pointed to 0.10.0's lines at
  this edit (Part C's U4 and C.5; each diffed in Cargo's registry).
- **S12 — The shared files have one owner, the orchestrator**, who makes each unit's edits of
  them from this list and the parts' texts before the unit's connector builders start, uncommitted, so the three
  start from the same list; a builder stages nothing outside its own paths, and the orchestrator
  commits these files by path, as a commit of the unit's block, after its builders report. They are:
  - `docs/showcase-elements.toml`. A leaf goes on an element only where a showcase draws it
    there (the file's rule: `leaves` is what the element draws, a control the union of its
    parts'); a toolkit that does not draw a listed leaf routes it "not reachable: <why>", the
    file's form. Each part's routes: C.7, D.13, E *All units*. The edits, in the file's order,
    each made in the unit its tag names (a leaf added a unit early fails each connector's route
    test): `chrome.menu_bar` `window.background_color` → `toolbar.background_color` (U1);
    `chrome.menu_bar.{file,view,theme,help}` + `menu.bar_hover_background`, `bar_open_background`,
    `bar_hover_text_color`, `bar_padding_horizontal_px`, `bar_padding_vertical_px`,
    `menu.border.corner_radius_px` (U1); `chrome.menu.theme` + `menu.panel_padding_horizontal_px`,
    `panel_padding_vertical_px` (U2); `chrome.menu.theme.{reload,system,light,dark,preferences}` +
    `menu.hover_border_color`, `border.corner_radius_px`, `defaults.border.line_width_px` (U1),
    `menu.min_row_height_px`, `active_background`, `active_text_color`, `icon_text_gap_px` (U2);
    `basic.list.frame`, `.row_1`, `.row_3`, `basic.table.frame`, `.row_1` (a selected row 2
    never) + `list.hover_border_color`, `defaults.border.line_width_px` (U1); `basic.table.frame`,
    `.header`, `.header.size` (the section line's sample) + `list.header_separator_color` (U3);
    `basic.slider.control`, `.thumb` + the four `slider.thumb_*border*` leaves (U3);
    `basic.icons.edit` + `toolbar.button_checked_border_color` (U3); `basic.drop_down.trigger` +
    `combo_box.open_background`, `open_border_color` (U3, E10). Registry order within an element.
  - `scripts/connector_parity_exceptions.toml` (the public-function gate's file,
    `scripts/check_connector_parity.py`): the function entries Part E *All units* and D.13 give,
    `menu_bar_modifier` (U1) and `toolbar_button_checked_border_color` absent in gpui and in iced
    (U3), each with its reason.
  - `docs/showcase-exceptions.toml` `[parity]`: a visual difference goes there, as a
    `[[parity.group]]` entry in `--merge`'s form written by hand, only where a gate compares it,
    which is what R finds. S5's tables, S6's pressed item and S8's gpui pen differ only under the
    pointer or pressed, and R and the asset run compare the window at rest: they take no entry.
  - In U4, the same file's reasons quote kde-breeze's line box "13.33 x 1.36 = 18.13", which
    becomes "13 x 1.36 = 17.68".

## Corrections to the rationale found while specifying

Applied by this specification; the rationale's text stays as approved (its §5 named these as the
specification's to read).

1. **E1, iced:** iced_aw draws `path` and `path_border` only under `DrawPath::Backdrop`; under its
   default `FakeHovering`, which the showcases use, it draws no highlight of its own. The connector's
   `styles::aw::menu` still takes the colour (for applications that use Backdrop), but every
   highlight the showcases draw is their own `menu_row`'s, and the "open bar title" limit holds only
   under Backdrop.
2. **E1, egui:** the line goes into the hovered, open and active states (S4, S6), not the hovered
   state only.
3. **E2, egui:** not drawn (S6).
4. **E4:** gpui's Overlays page frames its `menu_rows` with no padding today (only
   `MenuBar::popup` takes `popover.border.padding`); where the panel padding is stated it takes it.
   kde-breeze's popover radius is 8 (`corner_radius_lg`), not 5: the drop-down's 5 today is its own
   radius from `defaults.border.corner_radius`, so 4.5 is half a pixel from that.
5. **E5:** not every showcase table has a hovered row (S5).
6. **E8:** "with the width unset, no outline, as today": gpui's and egui-widgets' thumbs are
   outlined today, and keep it (S7). gpui shows no hover outline while the thumb is dragged.
7. **E9, gpui:** the checked pen stays under the pointer and pressed (S8).

## Not verified while specifying (settled in the build)

- T-L1 (C.1.3): the box over a row, shown under group hover and hidden under group active, in
  its hover-lit and its state-lit form, on gpui-pre and GPUI Fast. It runs first in U1; a
  failure stops U1 before any other task.
- Whether a click on gpui's slider thumb focuses the slider (its thumb stops the mouse-down).
- The pixel results in U2's tests (3, 14, 27, 29) as each toolkit lays them out at 13 px.
- Every new 8-bit colour was recomputed by hand from the Breeze schemes; the reader's tests
  compute them in the build.
- Whether `toolbar_button_checked`'s doc stays within its lines in gpui's `geometry.rs`; if not,
  the claims it moves are re-pointed in the same commit (C.0).
- The counts the parts give (egui's leaves, the KDE reader's moved leaves 46 → 51 → 53 → 62, the
  iced contract's leaves) are derived by reading; each unit's tests state the built value.
- The shortcut's colours (E2, §3's new backlog entry): the 2026-11-20 run gives them as the Edit
  row's second ink run (`menus_light.txt`, "ink run 1 x=48..82": the farthest pixel #626567 at
  rest, #515e66 hovered, #c4e7f8 pressed), antialiased ink, not the colour itself; the backlog
  entry cites that run and Breeze's rule, not a value.


# Part A — the model and its gates

Rationale: `docs/issues/0004_kde-leftovers-rationale.md` ("the rationale"), E1–E5, E7–E11, E13
(the model's share). Read on `v0.7.2-rc1` at `d5df78d9`, worktree `.claude/worktrees/lane2-b`.
Edits are placed by anchor (a type, a field, a key, a test's name); no repository line number is
cited. Presets, the KDE reader, `PF` rows, connectors and showcases are other parts'.

## A.0 What every unit follows

- **A colour field** is a soft option declared as `button.hover_border_color` is; **a size field**
  as `switch.thumb_border_line_width` is (`#[serde(rename = "…_px")]`, `soft_option`,
  `check = "non_negative"`). The derive gives the resolved struct an `Option<T>` (`gen_structs.rs`),
  the key to `FIELD_NAMES`, the merge, and a range check of a stated size by its TOML key
  (`gen_ranges.rs`). Resolved structs are `#[non_exhaustive]`, widget structs not: D0 (a)'s break.
  No resolver rule fills any new field.
- **Fallbacks live in the connectors, not the resolver.** Every "unset X is Y" the rationale names
  (E2, E3, E4's panel gap, E7, E8, E10) is applied where the field is read, as
  `list.header_active_background`'s is today (S1).
- **`docs/inheritance-rules.toml`**: every new field joins `[no_inheritance]` in one block appended
  after `"list.header_active_background", …`, the list's last entry, under the comment
  `# issue #4's leftovers (0.7.2): None keeps the connectors' look`; the entries use field names
  without `_px`, as the file's header says; each entry's comment is its doc's `None:` clause,
  shortened, then its section, as `"list.header_active_background", # None: a pressed section keeps
  the hovered fill (§2.15)`. No `[uniform]`, `[per_platform]` or
  `[wrong_safety_nets]` entry; `uniform_rules_all_accounted_for` and `check_wrong_safety_nets.py`
  see no change.
- **`docs/property-registry.toml`**: one key per field, the TOML key (`_px` kept), placed as each
  unit says. `check_platform_facts_rows.py` then needs a `PF` row named after the key without `_px`
  in the table's section (the facts part's rows; `defaults` fields are looked for in all of §2.1).
- **Gates a new field meets in this part**: `property_registry_keys_match_derive_inventory`
  (`model/mod.rs`), `no_inheritance_fields_are_not_inherited` (`resolve/inheritance.rs`),
  `check_section_refs.py` (§2.1.1, §2.6, §2.9, §2.13, §2.15, §2.24 exist as headings: checked),
  `check_platform_facts_rows.py`, `every_bundled_preset_source_lints_clean` (`presets.rs`), the
  proptest round trip, and the tests below.
- **Tests that grow per unit** (created in U1, each later unit adds its rows):
  - `presets.rs`, new `issue_4_leftovers_are_stated_by_kde_breeze_alone`, after
    `toolbar_button_states_are_stated_where_the_facts_give_them`: over `list_presets()`, light and
    dark, each new field of the unit `is_some() == (name == "kde-breeze")`, message
    "preset '{name}' {label}.<path>: stated only by kde-breeze (rationale §2)". U3 adds, for
    kde-breeze alone, `slider.thumb_hover_color.is_none()` (E8's removal).
  - `resolve/tests.rs`, new `issue_4_leftovers_pass_through_resolution`, after
    `an_unstated_list_or_sidebar_hover_is_the_widgets_own_background`: `fully_populated_variant()`
    with the unit's fields unset resolves (`resolve_all`) and validates with each `None`; with each
    set to a distinct test value it validates with the value unchanged; U2 adds
    `menu.min_row_height = Some(-1.0)` failing as `ResolutionInvalid` on `menu.min_row_height_px`.
  - `model/widgets/mod.rs` tests, new `new_sizes_use_px_keys`, after `switch_theme_toml_round_trip`:
    each new size field of the unit set on a default struct serialises under its `_px` key and
    round-trips equal.
- **Struct literals that name every field** (a scan of tracked `*.rs` for the six structs and their
  resolved forms without `..`): only `proptest_roundtrip.rs` `arb_list_theme` (U1, U3) and
  `arb_theme_defaults` (U4), and `model/resolved.rs`' test `sample_defaults` (`ResolvedDefaults`,
  U4); the other four enter `arb_theme_variant` as `Default`, and `serde_roundtrip.rs` names none
  of the new fields.
- **For the connectors' parts** (not edited here): the resolved model gains 21 leaves (the flag
  carried, S3), 14 of them colours; egui's `every_leaf_has_exactly_one_row…` count (534 → 555)
  and iced's `contract-leaves.toml` (its colour leaves) move with them.

## U1 — E1, E3, E5: the looks

Files: `native-theme/src/model/widgets/mod.rs` (`MenuTheme`, `ToolbarTheme`, `ListTheme`, their
tests), `docs/property-registry.toml`, `docs/inheritance-rules.toml`,
`native-theme/tests/proptest_roundtrip.rs`, `native-theme/src/presets.rs`,
`native-theme/src/resolve/tests.rs`.

**`MenuTheme`**, after `disabled_text_color`, before `font`, in this order:

| Field | Type, TOML key | Doc |
|---|---|---|
| `hover_border_color` | colour | "Outline of a hovered item's highlight, drawn at `defaults.border.line_width` inside the highlight, over the item's padding, so nothing moves, and rounded by `border.corner_radius`: Breeze's focus colour mixed 15 % towards `WindowText`. A pressed item draws none where `active_background` is stated. `None`: no outline (docs/platform-facts.md §2.6)." |
| `bar_hover_background` | colour | "Fill of a menu-bar title under the pointer, rounded by `border.corner_radius`: Breeze's opaque hover colour. `None`: the title keeps the connector's own look (docs/platform-facts.md §2.6)." |
| `bar_open_background` | colour | "Fill of a menu-bar title whose menu is open: Breeze's focus colour. `None`: `bar_hover_background`; with both `None`, the connector's own look (docs/platform-facts.md §2.6)." |
| `bar_hover_text_color` | colour | "Text colour of a menu-bar title under the pointer or open: Breeze's `HighlightedText`. `None`: the title keeps its text colour (docs/platform-facts.md §2.6)." |
| `bar_padding_horizontal` | `f32`, `bar_padding_horizontal_px` | "Inset of a menu-bar title's text from its left and right edge, in logical pixels: Breeze's `MenuBarItem_MarginWidth` 10; titles so padded touch. `None`: the connector's own title padding (docs/platform-facts.md §2.6)." |
| `bar_padding_vertical` | `f32`, `bar_padding_vertical_px` | as above, "from its top and bottom edge … `MenuBarItem_MarginHeight` 6" |

`MenuTheme`'s struct doc gains: "A menu bar is filled with `toolbar.background_color`, the fill of
the one tools area Breeze paints (docs/platform-facts.md §2.6, §2.13)." `ToolbarTheme::background_color`'s
doc becomes "Toolbar background color, also a menu bar's fill: Breeze fills the menu bar and the
toolbars as one tools area (docs/platform-facts.md §2.13)." No new field for the bar (E3).

**E3, the bar's fill, confirmed.** `ToolbarTheme::background_color` is
`#[theme(inherit_from = "defaults.background_color")]`, as `WindowTheme::background_color` is. Of
the 17 bundled presets only kde-breeze states `[light.toolbar]`/`[dark.toolbar]`
`background_color` (#dee0e2 / #292c30); none states `window.background_color`. Of the readers only
the KDE reader writes it (`kde/colors.rs`, `variant.toolbar.background_color = header_bg`, the
Header set's `BackgroundNormal`, else the Window set's; test
`without_a_header_group_the_toolbar_is_the_windows`); `windows.rs`, `macos.rs` and `gnome/` write
neither field. So it resolves to `defaults.background_color` on every preset but kde-breeze, and on
every live theme but KDE's.

**`ListTheme`**, after `hover_text_color`, colour `hover_border_color`: "Outline of a hovered,
unselected row, drawn at `defaults.border.line_width` on the shape of the row's hover fill:
Breeze's `Highlight` at alpha 0.8, a translucent layer. `None`: no outline
(docs/platform-facts.md §2.15)."

Registry: `[menu]` after `disabled_text_color`: `hover_border_color`, `bar_hover_background`,
`bar_open_background`, `bar_hover_text_color` = `"color"`, `bar_padding_horizontal_px`,
`bar_padding_vertical_px` = `"f32"`; `[list]` after `hover_text_color`: `hover_border_color = "color"`.
Rules: the seven names, as A.0 says.
Proptest: `arb_list_theme`'s vector `11..=11` becomes `12..=12`, `hover_border_color: colors[11]`.

## U2 — E2, E4: menus, the rest

Files: `model/widgets/mod.rs` (`MenuTheme`, tests), the registry, the rules, `presets.rs`,
`resolve/tests.rs`.

**`MenuTheme`**: `min_row_height` directly after `row_height`; the others after U1's
`bar_padding_vertical`:

| Field | Type, TOML key | Doc |
|---|---|---|
| `min_row_height` | `f32`, `min_row_height_px` | "Least height of a one-line item row in logical pixels; content taller than it grows the row. Breeze's row is 29 wherever the text line and the icon are 20 px or less, and the text + 9 above that, where the model's row grows as the text + `border`'s vertical padding: one pixel short there. Not applied where `row_height` is stated. `None`: the row is its content's height (docs/platform-facts.md §2.6)." |
| `active_background` | colour | "Fill of an item while the mouse button is held on it: Breeze's opaque focus colour. Where it is stated, a pressed item draws no `hover_border_color` outline (Breeze's outline is the fill's own colour). `None`: a pressed item keeps the hover look, outline included (docs/platform-facts.md §2.6)." |
| `active_text_color` | colour | "Text colour of a pressed item: Breeze's `HighlightedText`. `None`: `hover_text_color` (docs/platform-facts.md §2.6)." |
| `panel_padding_horizontal` | `f32`, `panel_padding_horizontal_px` | "Inset of the items from the left and right of the menu panel's frame line, in logical pixels; the frame is `popover.border`'s. Breeze's `PM_MenuHMargin` 4 counts from the panel's outer edge, its 1 px line included, so kde-breeze states 3. Rows so inset touch. `None`: the items are inset by `popover.border`'s padding (docs/platform-facts.md §2.6)." |
| `panel_padding_vertical` | `f32`, `panel_padding_vertical_px` | as above, "top and bottom … `PM_MenuVMargin` 4" |

`row_height`'s doc becomes: "Height of an item row, drawn exactly at a text scale of 1. `None`
where the platform states no fixed row height: KDE's rows size to their content with a floor,
`min_row_height`. Where both are stated, this one is drawn and `min_row_height` is not applied
(docs/platform-facts.md §2.6)."

**E4, the precedence.** The model only documents it (both docs above); no resolver rule clears
either field and validation accepts both. A connector draws `row_height` where it is `Some`, exactly
as today, and reads `min_row_height` only where `row_height` is `None`. No bundled preset states
both: `row_height_px` is stated by adwaita (32), macos-monterey (22) and material-3 (48), and
`min_row_height_px` by kde-breeze alone (the presets part).

Registry `[menu]`: `min_row_height_px = "f32"` after `row_height_px`; after U1's keys
`active_background`, `active_text_color` = `"color"`, `panel_padding_horizontal_px`,
`panel_padding_vertical_px` = `"f32"`. Rules: the five names, as A.0 says.

## U3 — E7, E8, E9, E10: lists and controls

Files: `model/widgets/mod.rs` (`ListTheme`, `SliderTheme`, `ToolbarTheme`, `ComboBoxTheme`,
tests), the registry, the rules, `proptest_roundtrip.rs`, `presets.rs`, `resolve/tests.rs`.

| Struct, after | Field | Type, TOML key | Doc |
|---|---|---|---|
| `ListTheme`, `header_active_background` | `header_separator_color` | colour | "The line on each column-header section's trailing edge (the last and a lone section have none) and along the header's bottom, laid over the header's fill: Breeze's `WindowText` at `frameContrast`, a translucent layer. `None`: `grid_color` (docs/platform-facts.md §2.15)." |
| `SliderTheme`, `disabled_thumb_color` (the struct's end) | `thumb_border_color` | colour | "The handle's outline at rest, `thumb_border_line_width` wide inside the handle: Breeze's `Button` mixed `frameContrast` towards `ButtonText`. `None` with the width stated: no line at rest, the handle keeping its outer size (docs/platform-facts.md §2.9)." |
| | `thumb_border_line_width` | `f32`, `thumb_border_line_width_px` | "Width of the handle's outline in logical pixels; 0 draws no outline in any state, a connector's own included. `None`: the connector draws the handle as it does without this field (its own outline, if any) (docs/platform-facts.md §2.9)." |
| | `thumb_hover_border_color` | colour | "The handle's outline with the pointer on it and while it is dragged, over the focus outline: Breeze's hover colour. `None`: `thumb_border_color` (docs/platform-facts.md §2.9)." |
| | `thumb_focus_border_color` | colour | "The handle's outline while the slider has the focus, from a click or a key: Breeze's focus colour. `None`: `thumb_border_color` (docs/platform-facts.md §2.9)." |
| `ToolbarTheme`, `button_checked_background` | `button_checked_border_color` | colour | "Outline of a checked toolbar button, drawn at `button.border.line_width`: Breeze's ordinary button pen, `Button` mixed `frameContrast` towards `ButtonText`; under the pointer and pressed `button_hover_border_color` applies. `None` where the theme states none: a connector draws it as it draws a selected flat button's (docs/platform-facts.md §2.13)." |
| `ComboBoxTheme`, `disabled_background` | `open_background` | colour | "Fill of a combo box while its popup is open, whatever the pointer does: Breeze draws it as a pressed button, `Button` mixed 0.3 towards `Highlight`. `None`: the fill the connector draws for the pointer's state (docs/platform-facts.md §2.24)." |
| | `open_border_color` | colour | "Border colour of an open combo box, drawn at `border.line_width`: Breeze's `Highlight`. `None`: the border the connector draws for the pointer's state (docs/platform-facts.md §2.24)." |

Registry: `[list]` `header_separator_color` after `header_active_background`; `[slider]` after
`disabled_thumb_color`: `thumb_border_color`, `thumb_border_line_width_px` (`"f32"`),
`thumb_hover_border_color`, `thumb_focus_border_color`; `[toolbar]` `button_checked_border_color`
after `button_checked_background`; `[combo_box]` `open_background`, `open_border_color` after
`disabled_background`. Rules: the eight names, as A.0 says; `slider.thumb_hover_color` keeps its
entry (E8 removes only kde-breeze's value). Proptest: `arb_list_theme` `12..=12` → `13..=13`,
`header_separator_color: colors[12]`.

## U4 — E11: font sizes in whole pixels

Files: `native-theme/src/model/defaults.rs`, `native-theme/src/model/font.rs` (docs),
`native-theme/src/model/resolved.rs`, `native-theme/src/model/mod.rs` (a baseline test),
`native-theme/src/resolve/context.rs` (a doc), `native-theme/src/resolve/validate.rs`,
`native-theme/src/resolve/validate_helpers.rs`, `native-theme/src/resolve/tests.rs`,
`native-theme/src/presets/documented_sizes.rs` (its gate), `proptest_roundtrip.rs`, the
registry, the rules, `presets.rs`. Not `native-theme-derive` (S3).

**The field.** `ThemeDefaults`, after `line_height`: `pub font_whole_pixels: Option<bool>`, doc:
"Whether every font size is rounded to whole logical pixels after its conversion (`true`), as Qt
rounds a font's pixel size: to hundredths, then to a whole number, halves up, before any scale.
`None` or `false`: sizes keep their fraction. Text-scale line heights are not rounded
(docs/platform-facts.md §2.1.1)." Added to `impl_merge!(ThemeDefaults { option { … } })` after
`focus_ring_keyboard_only` (without it the field neither merges nor counts in `is_empty`).
Validation rounds by it and **carries it into `ResolvedDefaults`** (S3), as
`focus_ring_keyboard_only`: `pub font_whole_pixels: Option<bool>` after `line_height`, doc
"Whether the font sizes were rounded to whole logical pixels as Qt rounds (`Some(true)`); an
application that re-derives a size from `defined_size` at its own DPI rounds it so too. `None`
where the theme does not say. See [`ThemeDefaults::font_whole_pixels`]."; `validate_defaults!`'s
`soft { focus_ring_keyboard_only }` → `soft { font_whole_pixels, focus_ring_keyboard_only }`;
`ResolvedDefaults`' struct doc names both soft options; `model/resolved.rs`' test
`sample_defaults` sets it `None`. `resolve/context.rs`' `font_dpi` doc, after "re-derives its
sizes from each resolved font's `defined_size`", adds ", rounding each as Qt does where
`defaults.font_whole_pixels` is `Some(true)`". `model/mod.rs`
`theme_defaults_field_info_matches_baseline`: `"font_whole_pixels"` after `"line_height"`
(declaration order). Registry `[defaults]` after `line_height`: `font_whole_pixels = "bool"`.
Rules: `"defaults.font_whole_pixels", # None: sizes keep their fraction; KDE rounds as Qt
(§2.1.1)`.

**Where the rounding happens.** A size becomes pixels only in `FontSize::to_logical_px(dpi)`, called
from `require_font` (the defaults' fonts; every widget font through `require_font_opt` and
`ValidateNested for FontSpec`) and `require_text_scale_entry`, all inside `validate_with_dpi`, which
every resolution path reaches. None of them changes, nor does any signature the derive emits (S3).
New in `validate_helpers.rs`, after `stated_size`:

```rust
/// Qt's rounding of a font's pixel size: to hundredths, then to a whole number,
/// halves up (`QFontDatabasePrivate::load`, qtbase v6.12.0 qfontdatabase.cpp:2865-2869).
pub(crate) fn qt_whole_px(px: f32) -> f32 {
    ((px * 100.0).round() / 100.0).round()
}
```

`f32::round` rounds halves away from zero, which is halves up for every size `check_font` accepts
(> 0). **The pass.** In `validate_with_dpi`, after the `missing` short-circuit and before
`check_defaults_ranges`, where `self.defaults.font_whole_pixels == Some(true)`, each resolved size
is set to `qt_whole_px` of itself, from one hand list beside the function's two per-widget lists:
`defaults.font`, `defaults.mono_font`, the four `text_scale` entries' `size`, and the nineteen
widget fonts (`window.title_bar_font`, `button.font`, `input.font`, `checkbox.font`, `menu.font`,
`tooltip.font`, `tab.font`, `sidebar.font`, `toolbar.font`, `status_bar.font`, `list.item_font`,
`list.header_font`, `popover.font`, `dialog.title_font`, `dialog.body_font`, `combo_box.font`,
`segmented_control.font`, `expander.font`, `link.font`); the bindings it names become `let mut`.
A text-scale entry's `line_height` was converted from its own value before the pass and stays
unrounded (S3); `defined_size` keeps the source's value. Range checks run after, on the rounded
size. **A size rounded to 0.** `check_font` tests the resolved size, so a font that rounds to 0 (a
stated `Pt(0.3)` at 96 DPI, 0.4 px) fails its range with the stated value shown (`stated_size`).
`check_defaults_ranges` tests a text-scale entry's stated size instead, which the pass does not
touch; its size check takes `check_font`'s form: it tests `entry.size`, the resolved value, and
reports `stated_size(..)` under `size_key(..)` in the same `RangeViolation` (without the flag the
two tests agree: a positive DPI keeps the sign). Docs: `FontSize`'s "converted to logical pixels via
`FontSize::to_logical_px(dpi)`", `FontSpec::size`'s "Converted to `f32` logical pixels during
validation via `FontSize::to_logical_px(dpi)`", and the "converted at the resolution context's
font_dpi" of `ResolvedFontSpec::size` (`model/font.rs`) and `ResolvedTextScaleEntry::size`
(`model/resolved.rs`) each add "and rounded to a whole number where `defaults.font_whole_pixels`
is true"; `ResolvedTextScaleEntry::line_height`'s "Computed as `defaults.line_height * size`"
adds "from the size before that rounding".

**Values** (kde-breeze, 96 DPI): body and mono 10 pt 13.33 → 13; caption 8 pt 10.67 → 11; 12 pt 16
and 13.5 pt 18 unchanged; display, the body's, 13. The connectors pass the `f32` through (rationale §1).

**Tests whose expected KDE sizes change.**
- `kde/mod.rs` `the_reader_reports_a_font_dpi_of_96`: its assert and comment are Part B's (U4).
- `presets/documented_sizes.rs` `native_themes_state_documented_fields`: it would fail on
  kde-breeze (caption `Pt(8.0)` expects 10.67, resolved 11; `BodyTimes(1.20)` takes the rounded
  body, 13 × 1.2 = 15.6, resolved 16; `BodyTimes(1.35)`, 17.55 against 18). Under a variant whose
  `defaults.font_whole_pixels == Some(true)`: `Pt(v)` expects `qt_whole_px(v * dpi / 72.0)`;
  `BodyTimes(f)` expects `qt_whole_px(body_px * f)` with `body_px` the body unrounded,
  `theme.defaults.font.defined_size` converted at the gate's DPI (Kirigami scales the point size,
  then Qt rounds once; the rounded body × f would contradict the preset's 12 and 13.5 pt and the
  reader); elsewhere as today. `Want::Pt`'s doc adds "rounded where the theme says".
- Unchanged (no flag stated): `validate_converts_pt_to_logical_px_at_96_dpi`, `validate_no_dpi_uses_default_96`.
- Comments citing 13.33, for the connectors' parts (assertions computed from the theme; not run):
  egui-widgets `the_header_is_its_stated_height_by_the_row_rule` and the egui showcase's
  `the_expander_header_is_its_stated_height_by_the_row_rule` (at 1.5: 13 × 1.5 × 1.36 = 26.52, 27,
  header 41, still past 40), the gpui showcase's `a_section_heading_takes_the_section_heading_role`
  and `info/mod.rs`, iced `control_line_height_reaches_the_stated_minimum`.

**New tests.** `resolve/tests.rs`, after `validate_no_dpi_uses_default_96`:
`whole_pixel_fonts_round_as_qt` — `fully_populated_variant()` with the flag `Some(true)`, defaults
font `Pt(10.0)`, mono `Pt(8.0)`, a widget font (`button.font`) `Pt(13.5)`, caption `Pt(12.0)`:
resolved 13.0, 11.0, 18.0, 16.0 exactly; `defined_size` stays `Pt(10.0)`; the caption's line
height equals the same variant's with the flag unset; at 120 DPI the body is 17.0 (16.67); with
`Some(false)` the body is 13.333 within 0.01; the resolved `defaults.font_whole_pixels` is the
stated value in each case. `qt_whole_px_rounds_to_hundredths_first`:
12.4999 → 13.0 (one rounding would give 12), 12.5 → 13.0, 13.333334 → 13.0, 10.666667 → 11.0.
`every_resolved_font_size_is_whole_under_the_flag` (S3's guard): `fully_populated_variant()`,
serialised (`serde_json::to_value`), each object's `size_px` or `size_pt` key (only `FontSpec`
and `TextScaleEntry` carry those keys) replaced by `size_pt: 10.0`, deserialised, the flag
`Some(true)`, resolved and validated at 96 DPI; its `ResolvedTheme`, serialised, is walked: every
object with a `family` key and every `text_scale` entry has a whole `size`. The walk counts what
it checked, at least 25 (the pass's list), and the same theme with the flag unset gives a
fractional size at each place it checked, so a font field the pass misses fails the test.
`a_font_size_rounded_to_zero_is_invalid` (the one range test): the flag `Some(true)`, caption
`Pt(0.3)` and `button.font` `Pt(0.3)` (0.4 px each, rounded to 0): `ResolutionInvalid` naming
`text_scale.caption.size_pt` and `button.font.size_pt`, each with the value 0.3.
`defaults.rs`: `default_has_all_none_options` asserts the field `None`; new
`merge_keeps_and_overrides_font_whole_pixels` beside `merge_keeps_and_overrides_focus_ring_keyboard_only`.
Proptest `arb_theme_defaults`: a sixth scalar `proptest::option::of(proptest::bool::ANY), //
font_whole_pixels`, the field set from it, the comment "5 scalar Option fields" → 6.
`presets.rs`' grown test adds `defaults.font_whole_pixels`.


# Part B — kde-breeze, the KDE reader, the facts, and outside the code

Rationale: `docs/issues/0004_kde-leftovers-rationale.md`, E1–E14, not re-decided; read on `v0.7.2-rc1` at `d5df78d9`
(worktree `lane2-b`), 2026-11-20; form of the archived steps 6–7 specification. Edits are placed by anchor (key,
function, test name, row name, quoted phrase), never by a repository line number. Breeze (`st`, `hp`, `hh`, `mt` at
`f0b1d753`), qtbase v6.12.0 and evidence files keep their lines; Breeze's lines are the rationale's and `PF`'s, not
re-read. Field names are the rationale's; Rust and TOML spellings are Part A's (sizes take `_px`). `G-mac`,
`G-win`, `G-gn`, `G-m3`: the parked gap maps `docs/todo_v0.7.1_model-completeness-gap-map-{macos,windows,gnome,material}.md`
on `v0.8.0-rc1` at `78ee0efd` (in `docs/`, not `docs/facts/`).

## B.0 Shared by the four units

**Reader form.** One `const` per new mix or alpha with its citation, as `DEFAULT_BUTTON_MIX`; each write in
`populate_colors`' block for its widget, from a binding the function has (no second read of a key); a missing input
gives `None` and the preset's value stands. `populate_colors`' doc lists each new colour (computed colours; blends on
the accent; the `DecorationFocus` sentence) and adds: "A menu's fills and outlines are Breeze's with
`MenuItemDrawStrongFocus` on, its default (breeze.kcfg:130-132 at breeze f0b1d75); the reader does not read
breezerc." Sizes go into `kde/metrics.rs` `populate_widget_sizing` with their `breezemetrics.h` names.

**Values**, recomputed by hand with the reader's `mix`/`translucent` from the keys of `kde/mod.rs`
`BREEZE_LIGHT_SCHEME` / `BREEZE_DARK_SCHEME`; each equals the rationale's value and kde-breeze's present one where
there is one:

| Field | Light | Dark | Reader | Today |
|---|---|---|---|---|
| `menu.hover_border_color` (E1) | #399acc | #5abaec | `mix(view_focus, window_fg, 0.15)` | new; measured, `2026-11-20/menus_*.txt` |
| `menu.bar_hover_background`, `bar_open_background` (E3) | #3daee9 | #3daee9 | `view_hover`, `view_focus` | new |
| `menu.bar_hover_text_color` (E3), `menu.active_text_color` (E2) | #ffffff | #fcfcfc | `selection_fg` | = `selection_text_color` |
| `menu.active_background` (E2) | #3daee9 | #3daee9 | `view_focus` | new |
| `list.hover_border_color` (E5) | #3daee9cc | #3daee9cc | `translucent(selection_bg, 0.8)` (204) | new |
| `list.hover_text_color` (E6) | #232629 | #fcfcfc | `view_fg` | stated, not written by the reader |
| `button.border.color` (E6), `slider.thumb_border_color` (E8), `toolbar.button_checked_border_color` (E9) | #d1d1d2 | #535659 | `mix(Button, ButtonText, frameContrast)` | button's stated uncited, not written |
| `list.header_separator_color` (E7) | #23262933 | #fcfcfc33 | `translucent(window_fg, frameContrast)` | new; over the header #d1d1d2 / #535659 |
| `slider.thumb_hover_border_color`, `thumb_focus_border_color` (E8) | #3daee9 | #3daee9 | `view_hover`, `view_focus` | new |
| `combo_box.open_background` (E10) | #c3e5f6 | #2f5368 | `mix(button_bg, selection_bg, PRESSED_BUTTON_MIX)` | = `button.active_background`, computed so today |
| `combo_box.open_border_color` (E10) | #3daee9 | #3daee9 | `selection_bg` | new |

Sizes today (`kde/metrics.rs`, kde-breeze): menu padding 4 all round, `menu.icon_text_gap` 8, `button.min_width` 80,
as the rationale says. E3's check: of the 17 presets only kde-breeze states `toolbar.background_color`, none
`window.background_color`, and `docs/inheritance-rules.toml` gives both `= "defaults.background_color"`. E8's:
`slider.thumb_hover_color` is an `Option` resolved and in `[no_inheritance]`, so removing kde-breeze's line is safe.

**Tests every unit extends** (`kde/mod.rs`): `derived_leaves`/`resolved_leaves` gain the unit's leaves at the end,
length and doc count ("The forty-six colour leaves") with them, 46 → 51 → 53 → 62;
`the_derived_colours_are_the_presets_under_breeze` then checks each new leaf against kde-breeze under both schemes,
unedited. `moved_leaves` lists grow as each unit says (appended in leaf order); each test's doc names what it moves.
`kde/colors.rs` `test_full_mapping_completeness` asserts each new colour `is_some()` (its `BREEZE_DARK` holds every key
read). `a_custom_accent_reaches_the_accent_fills`: no edit.

## U1 — The looks (E1, E3, E5)

**kde/colors.rs.** After `MENU_PANEL_MIX`: `const MENU_HOVER_OUTLINE_MIX: f64 = 0.15;`, doc "How far a hovered menu
item's outline moves from the focus colour towards the window text: `focusOutlineColor()`, `KColorUtils::mix(focusColor,
WindowText, 0.15)` (breezehelper.cpp:172-175, breezestyle.cpp:6021-6049 at breeze f0b1d75; docs/platform-facts.md
§2.6)". After `HEADER_SECTION_MIX`: `const LIST_HOVER_OUTLINE_ALPHA: f32 = 0.8;`, doc "The alpha of `Highlight` in a
hovered item-view row's 1 px outline (breezestyle.cpp:4880-4884 at breeze f0b1d75; docs/platform-facts.md §2.15)".
`let selection_fg = …"Colors:Selection", "ForegroundNormal"…` moves from the block "The selection of a view without
the focus" to the function's top after `selection_bg` (the `inactive` closure keeps it). `// Menu` block, after
`variant.menu.background_color`:

```rust
    // A hovered item's highlight is outlined 1 px in `focusOutlineColor()`; a menu bar's
    // title is filled with the opaque hover colour under the pointer and the focus colour
    // while its menu is open, lettered `HighlightedText`; the bar is the toolbar's fill
    // (breezestyle.cpp:5841-5945, :6021-6049, breezehelper.cpp:172-175 at breeze f0b1d75;
    // docs/platform-facts.md §2.6).
    variant.menu.hover_border_color = view_focus
        .zip(window_fg)
        .map(|(focus, text)| mix(focus, text, MENU_HOVER_OUTLINE_MIX));
    variant.menu.bar_hover_background = view_hover;
    variant.menu.bar_open_background = view_focus;
    variant.menu.bar_hover_text_color = selection_fg;
```

List block, after `list.hover_background` (comment gains "outlined 1 px in it at alpha 0.8, :4880-4884"):
`variant.list.hover_border_color = selection_bg.map(|s| translucent(s, LIST_HOVER_OUTLINE_ALPHA));`.
**kde/metrics.rs** `// Menu`, at its end: `variant.menu.bar_padding_horizontal = Some(10.0); // MenuBarItem_MarginWidth`
and `bar_padding_vertical = Some(6.0); // MenuBarItem_MarginHeight`, under "A menu bar's title (docs/platform-facts.md §2.6)".

**kde-breeze.toml** (light; dark the same with B.0's dark values and `BreezeDark.colors` :117 focus, :118 hover,
:96 selection text, :138 window text):

```toml
# [light.menu], after hover_text_color:
# platform-facts.md (§2.6): hover_border_color, focusOutlineColor(), KColorUtils::mix([Colors:View] DecorationFocus
# 61,174,233, [Colors:Window] ForegroundNormal 35,38,41, 0.15) (breezestyle.cpp:6021-6049, breezehelper.cpp:172-175 at
# breeze f0b1d75; measured)
hover_border_color = "#399acc"
```

The other keys take the same form, the comment from their KDE cell below: `[light.menu]` at its end,
`bar_hover_background = "#3daee9"`, `bar_open_background = "#3daee9"`, `bar_hover_text_color = "#ffffff"` (one comment;
View DecorationHover `:117`, DecorationFocus `:116`, Selection ForegroundNormal `:95`; "the bar takes
toolbar.background_color"), `bar_padding_horizontal_px = 10.0`, `bar_padding_vertical_px = 6.0`; `[light.menu.border]`
after `padding_vertical_px`, `corner_radius_px = 4.5` (§2.6 `border.corner_radius`'s KDE cell, hh:345-348, hp:384);
`[light.list]` after `hover_text_color`, `hover_border_color = "#3daee9cc"` (Selection BackgroundNormal `:87`).

**Tests.** Leaves: `menu.hover_border_color`, `menu.bar_hover_background`, `menu.bar_open_background`,
`menu.bar_hover_text_color`, `list.hover_border_color` (51). Moved: accent + `list.hover_border_color`; window set
(both schemes) + `menu.hover_border_color`; hover + `menu.bar_hover_background`; focus + `menu.hover_border_color`,
`menu.bar_open_background`; the others nothing. New after `the_contrast_moves_the_table_grid`:
`the_selection_text_moves_the_menu_bar_titles`, `Scheme { selection: ("61,174,233", "10,20,30"), ..BREEZE_LIGHT_SCHEME }`,
expecting `["menu.bar_hover_text_color"]` (no present leaf reads Selection `ForegroundNormal`; with
`ChangeSelectionColor=true` the inactive text is the window's). `kde/colors.rs`, after
`test_menu_hover_and_separator_are_breezes`: `test_menu_hover_outline_is_breezes`, from `breeze_light_and_dark()`,
#399acc / #5abaec. `kde/metrics.rs`: `breeze_metrics_populates_the_menu_bar_titles` (10, 6).

**PF** (KDE cells; links as §2.6 `hover_background`'s; evidence under `evidence/breeze-offscreen/2026-11-20/`).
§2.6 after `hover_text_color`: `hover_border_color` — **(Breeze src)** `focusOutlineColor()` = `mix(DecorationFocus,
WindowText, 0.15)`, 1 px on the highlight's edge, inside the 4 px inset (st:6021-6049, hp:172-175) — #399acc / #5abaec
**(measured)** (`menus_light.txt`, `menus_dark.txt`). After `disabled_text_color`: `bar_hover_background`,
`bar_open_background`, `bar_hover_text_color` — `drawMenuBarItemControl` fills a hovered title with `hoverColor`, an
open one with `focusColor`, rounded as the menu highlight, lettered `HighlightedText` (st:5841-5945) — #3daee9,
#3daee9, #ffffff / #fcfcfc **(measured)** (`menubar_*.txt`; the hover by an unsynthesized event, the README);
`bar_padding_horizontal`, `bar_padding_vertical` — `MenuBarItem_MarginWidth` 10, `_MarginHeight` 6 (mt:93-94,
st:3598); titles touch, ink 11 in, 30 high for an 18 px line **(measured)**. §2.15 after `hover_text_color`:
`hover_border_color` — `Highlight` at 0.8, 1 px, on a hovered unselected row (st:4880-4884) — #3daee9cc; edge pixel
#5ebbec / #3694c5 **(measured)** (`views_*.txt`). Other platforms: B.F's table.

## U2 — Menus, the rest (E2, E4)

**kde/colors.rs**, after U1's menu lines, comment "A pressed item is the opaque focus colour, outlined in it, lettered
`HighlightedText` (breezestyle.cpp:6021-6029, :6155 at breeze f0b1d75; docs/platform-facts.md §2.6)":
`variant.menu.active_background = view_focus;`, `variant.menu.active_text_color = selection_fg;`.
**kde/metrics.rs** `// Menu`: both "kept" comments go; under "A menu with icons and no checkable items, KDE's default
(docs/platform-facts.md §2.6; breezestyle.cpp:649-651, :3663-3667, :6053-6065, :6091-6097 at breeze f0b1d75)":

```rust
    border.padding_left = Some(14.0); // MenuItem_MarginWidth 4 + MenuItem_TextLeftMargin 8 + (20 - 16) / 2
    border.padding_right = Some(27.0); // MenuItem_MarginWidth 4 + MenuButton_IndicatorWidth 20 + MenuItem_ItemSpacing 2 + 1
    // padding_top, padding_bottom: 4, MenuItem_MarginHeight: the two existing lines stay
    variant.menu.icon_text_gap = Some(4.0); // (20 - 16) / 2 + MenuItem_ItemSpacing 2
    variant.menu.panel_padding_horizontal = Some(3.0); // PM_MenuHMargin = MenuItem_HighlightGap 4, less the 1 px outline
    variant.menu.panel_padding_vertical = Some(3.0); // PM_MenuVMargin, the same
    variant.menu.min_row_height = Some(29.0); // max(text, 20, icon) + 1 + 2 * MenuItem_MarginHeight, text <= 20
```

**kde-breeze.toml** (each comment in U1's form, its content the new KDE cell below; dark at the same anchors with
#fcfcfc, `:117`, `:96`): `[light.menu.border]` `padding_horizontal_px = 4.0` → `padding_left_px = 14.0`,
`padding_right_px = 27.0` (the shorthand beside a side is refused, `model/border.rs`), `padding_vertical_px` stays;
`[light.menu]`: the `# row_height not stated` comment ends "…with a floor, min_row_height", then `min_row_height_px =
29.0`; `icon_text_gap_px` 8.0 → 4.0 (its "kept at 8" comment replaced); `panel_padding_horizontal_px = 3.0`,
`panel_padding_vertical_px = 3.0`; after U1's `hover_border_color`: `active_background = "#3daee9"`,
`active_text_color = "#ffffff"` (DecorationFocus `BreezeLight.colors:116`, Selection ForegroundNormal `:95`).
**`native-theme/src/presets/documented_sizes.rs`**: `row(KdeBreeze, "menu", all(4.0), …)` → `trbl(4.0, 27.0, 4.0,
14.0)`, note "4 + 8 + 2 left, 4 + 20 + 2 + 1 right (a menu with icons); MenuItem_MarginHeight 4" (its static and live
checks read the preset and the reader).

**Tests.** Leaves `menu.active_background`, `menu.active_text_color` (53). Focus + `menu.active_background`; the
selection-text test + `menu.active_text_color`, renamed `the_selection_text_moves_the_pressed_item_and_the_bar_titles`.
`kde/metrics.rs`: `breeze_metrics_populates_the_menu_geometry` (14, 27, 4, 4; gap 4; panel 3, 3; least row 29).

**PF** §2.6: after U1's `hover_border_color`: `active_background`, `active_text_color` — the opaque `focusColor`,
outlined in it, text `HighlightedText` (st:6021-6029, :6108, :6129, :6155) — #3daee9; #ffffff / #fcfcfc **(measured)**
(`menus_*.txt`, `Sunken|Selected`). After `row_height`: `min_row_height`, which takes `row_height`'s KDE text (the
floor, 29, the 2026-11-13 lines) and adds "the text + 9 above a 20 px line, where the model grows as the text + 8";
`row_height` KDE becomes "**(none)** — content-sized; its floor is `min_row_height`". After `border.padding_vertical`:
`panel_padding_horizontal`, `panel_padding_vertical` — `PM_MenuHMargin`/`PM_MenuVMargin` = `MenuItem_HighlightGap` 4
from the outer edge, its 1 px outline included: 3 inside it (st:649-651, mt:55) **(measured)** (2026-11-13).
`border.padding_horizontal` KDE is rewritten to "14 left, 27 right in a menu with icons and no checkable items" with
E4's derivation and measurement (item at x 4, icon 18..33, highlight 4..109, shortcut ink ending at 82), its sentence
on a menu without icons (text 16 in) kept as the deviation. `icon_text_gap` and `border.corner_radius` say 4 and 4.5
already.

## U3 — Lists and controls (E6–E10)

**kde/colors.rs**, `// Button` block, after `default_button_text_color`:

```rust
    // A button's frame, the plain pen: the button colour moved towards its text by the
    // frame contrast (breezehelper.cpp:697-699 at breeze f0b1d75; docs/platform-facts.md
    // §2.3); a slider handle's outline at rest and a checked flat button's pen are the same.
    let button_frame = variant
        .button
        .background_color
        .zip(button_fg)
        .map(|(button, text)| mix(button, text, frame_contrast(ini)));
    if let Some(frame) = button_frame {
        variant.button.border.get_or_insert_default().color = Some(frame);
    }
```

Slider, after `thumb_color` ("outlined in the plain frame at rest, `hoverColor` under the pointer, `focusColor` with
the focus, hover winning; breezestyle.cpp:8122-8128, breezehelper.cpp:248-275 at breeze f0b1d75; §2.9"):
`thumb_border_color = button_frame`, `thumb_hover_border_color = view_hover`, `thumb_focus_border_color = view_focus`.
Toolbar, after `button_checked_background`: `button_checked_border_color = button_frame` (breezehelper.cpp:681-684,
:697-699; §2.13). Combo box, after `hover_border_color` ("open, a pressed button frame whatever the pointer does;
breezestyle.cpp:7823-7853, breezehelper.cpp:693-712 at breeze f0b1d75; §2.24"): `open_background =
button_bg.zip(selection_bg).map(|(button, highlight)| mix(button, highlight, PRESSED_BUTTON_MIX))`,
`open_border_color = selection_bg`. List: after U1's `hover_border_color`, `variant.list.hover_text_color = view_fg;`
(a row is lettered `QPalette::Text`, §2.15); after `header_active_background` ("a section's trailing edge and the
header's bottom, the window text at `frameContrast`; breezestyle.cpp:6803-6893 at breeze f0b1d75; §2.15"):
`variant.list.header_separator_color = window_fg.map(|text| translucent(text, frame_contrast(ini) as f32));` (as
`progress_bar.border.color`). **kde/metrics.rs** Slider: `variant.slider.thumb_border_line_width = Some(1.0); //
PenWidth::Frame 1.001, drawn as 1`.

**kde-breeze.toml** (comments in U1's form from the KDE cells below, with the scheme keys' `BreezeLight.colors`
lines: Button :31, :39; Selection :87; View :116, :117, :123; Window :137; dark: B.0's values, :32, :40, :88, :117,
:118, :124, :138): `[light.button.border] color` gains its comment (§2.3); `[light.slider]` loses `thumb_hover_color
= "#93cee9"`, gains after `thumb_color` `# thumb_hover_color not stated: platform-facts.md (§2.9), the handle keeps its
fill under the pointer` and `thumb_border_color = "#d1d1d2"`, `thumb_border_line_width_px = 1.0`,
`thumb_hover_border_color = "#3daee9"`, `thumb_focus_border_color = "#3daee9"`; `[light.toolbar]` after
`button_checked_background`: `button_checked_border_color = "#d1d1d2"`; `[light.combo_box]` after
`hover_border_color`: `open_background = "#c3e5f6"`, `open_border_color = "#3daee9"`; `[light.list]`:
`hover_text_color` gains its comment (§2.15, `← item_font.color`, View ForegroundNormal), and after
`header_active_background` `header_separator_color = "#23262933"`.

**Tests.** Leaves: `list.hover_text_color`, `button.border.color` (the `border` closure), `list.header_separator_color`,
`slider.thumb_border_color`, `slider.thumb_hover_border_color`, `slider.thumb_focus_border_color`,
`toolbar.button_checked_border_color`, `combo_box.open_background`, `combo_box.open_border_color` (62). Moved: accent +
`combo_box.open_background`, `combo_box.open_border_color`; button set + `button.border.color`,
`slider.thumb_border_color`, `toolbar.button_checked_border_color`, `combo_box.open_background`; window set (both) +
`list.header_separator_color`; view set + `list.hover_text_color`; hover + `slider.thumb_hover_border_color`; focus +
`slider.thumb_focus_border_color`; frame contrast + `button.border.color`, `list.header_separator_color`,
`slider.thumb_border_color`, `toolbar.button_checked_border_color`; header, contrast, selection text nothing.
`kde/colors.rs`: `test_the_plain_frame_and_the_open_combo_box_are_breezes`, from `breeze_light_and_dark()`: the three
frame leaves #d1d1d2 / #535659, the separator #23262933 / #fcfcfc33, the open fill #c3e5f6 / #2f5368 and pen #3daee9.
`breeze_metrics_populates_slider_thumb` asserts the width 1.

**PF.** §2.3 `border.color` KDE: **(Breeze src)** `mix(Button, ButtonText, frameContrast)` (hp:697-699) — #d1d1d2 /
#535659 **(measured)** (`2026-11-20/qt-fonts-widgets/out_widgets.txt`, the checked tool button's pen, the same mix).
§2.9 after `thumb_hover_color`: `thumb_border_color`, `thumb_border_line_width`, `thumb_hover_border_color`,
`thumb_focus_border_color` — E8's rule (st:8122-8128, hp:248-275); rest edge #d2d2d3 / #515457, hover edge #45b1e9 /
#3ca8e0 (an antialiased circle's edge), fill unchanged **(measured)** (`out_widgets.txt`); width `PenWidth::Frame`
1.001. §2.13 after `button_checked_background`: `button_checked_border_color` (hp:681-684, :697-699 — #d1d1d2 / #535659
**(measured)**); that row's KDE "has no field" becomes "is `button_checked_border_color`". §2.15 after
`header_active_background`: `header_separator_color` (st:6803-6893; none after the last or a lone section; #d1d2d2 /
#535558 **(measured)**, `2026-11-13/`). §2.24 after `hover_border_color`: `open_background`, `open_border_color`
(st:7823-7853, hp:693-712 — #c3e5f6 / #2f5368, #3daee9 **(measured)**, `2026-11-18/combo_trigger_*.txt`); the
`hover_border_color` cell's open-combo clause ends "(`open_background`, `open_border_color`)".

## U4 — Fonts and sizes (E11, E12)

**kde-breeze.toml** `[light.defaults]`/`[dark.defaults]`, after `line_height`: `# platform-facts.md (§2.1.1):
font_whole_pixels, Qt rounds a point size's pixel size to hundredths, then to a whole number, halves up, before the
scale (qfontdatabase.cpp:2865-2869 at qtbase v6.12.0; measured)`, `font_whole_pixels = true`. `[light.button]`/dark:
`min_width_px` 80.0 → 82.0, `# platform-facts.md (§2.3): min_width, Button_MinWidth 80 plus the frame's 2 a side less
the 1 px shadow inset a side, the visible box (breezestyle.cpp:3555-3564, breezehelper.cpp:670-671 at breeze
f0b1d75)`. **kde/metrics.rs**: `Some(82.0); // Button_MinWidth 80 + 2 × (2 frame - 1 shadow inset)`. The flag has no
reader write. **Tests**: `breeze_metrics_populates_button_min_width` and `test_from_kde_content_populates_widget_sizing`
assert 82; `the_reader_reports_a_font_dpi_of_96` (pipeline over kde-breeze) asserts `size == 13.0`, comment "Noto Sans
10 pt at 96 DPI, 13.33 px, which kde-breeze's `font_whole_pixels` rounds to 13 as Qt does". The resolver's and the
connectors' 13.33 tests are Part A's and theirs.

**PF.** §2.1.1 after `line_height`: `font_whole_pixels`, KDE **(KF6 src)** true: QtGui rounds before the scale, for Qt
Widgets and Qt Quick alike (qfontdatabase.cpp:2865-2869 at v6.12.0) — 10 pt 13 px **(measured)**, see §1.3.1. §1.3.1, a
paragraph after the `[General]` font table: "**Pixel size.** Qt converts a font's point size to pixels, rounds it to
hundredths and then to a whole number, halves up ([qfontdatabase.cpp:2865-2869](https://github.com/qt/qtbase/blob/v6.12.0/src/gui/text/qfontdatabase.cpp#L2865-L2869)),
in QtGui, which Qt Widgets and Qt Quick share; the scale multiplies afterwards. Measured 2026-11-20 under
`org.kde.desktop` ([out_light.txt](evidence/breeze-offscreen/2026-11-20/qt-fonts-widgets/out_light.txt),
[out2_light.txt](evidence/breeze-offscreen/2026-11-20/qt-fonts-widgets/out2_light.txt)): 10 pt is 13 px for a Qt Quick
`Text`, a QQC2 `Label`, Kirigami's default font and Qt Widgets; 8 pt 11, 12 pt 16, 13.5 pt 18. kde-breeze states
`defaults.font_whole_pixels` (§2.1.1)." §2.3 `min_width` KDE: `Button_MinWidth` 80 + the frame, 2 a side, inside which
a 1 px shadow inset lies (st:3555-3564, hp:670-671): **82**, the outer box without shadows (rule 1).

## Outside the code (E13, E14)

Written once, in the plan's L5, for the units built; a unit cut leaves nothing here, and its text
goes to the backlog (the plan, §2). The unit tags below say which unit each item belongs to.

- **`CHANGELOG.md`** `## [0.7.2]`. *Breaking Changes*, the one bullet (D0 (a)), names the 21 fields by struct:
  `ThemeDefaults` + `font_whole_pixels`; a new `MenuTheme` clause (its eleven); `ListTheme` + `hover_border_color`,
  `header_separator_color`; a new `SliderTheme` clause (its four); `ToolbarTheme` + `button_checked_border_color`;
  `ComboBoxTheme` + `open_background`, `open_border_color`. *Added* (native-theme; connectors' are their parts'), in
  step 6's form with kde-breeze's values: menu item outline, bar and titles, row outline (U1); pressed item and menu
  geometry (U2); header separators, slider outline, checked tool button pen, open combo box (U3); the font flag (U4).
  *Changed*: every KDE font size is a whole pixel (13 for 10 pt); kde-breeze's menu padding 14 / 27, gap 4, radius 4.5.
  *Fixed*: the reader writes `list.hover_text_color` and `button.border.color`; kde-breeze no longer states
  `slider.thumb_hover_color` (#93cee9, no source; the form of "kde-breeze no longer states
  `checkbox.hover_background`…"); the button's minimum width is 82, the visible box.
- **`ROADMAP.md`**: 0.7.2's heading adds "and issue #4's leftovers (the menus' and rows' outlines, the menu bar, the
  open combo box, whole-pixel fonts)", and a paragraph after step 7's says it in sentences. *Also in the 0.7.x
  releases*' "77 open items" is recounted (`grep -c '\[ \] \[v0.7.x\]' docs/todo.md`): −1 closed, +1 new = 77.
- **READMEs**: `README.md` ("47 colours", "five of the 47") and `native-theme/README.md` ("every colour but 47 optional
  ones", "most of the 47", "five of the 47") take 61: the fourteen new colours resolve as `Option`s (A.0). The size
  lists add the menu's panel and title paddings and least row height; beside the switch outlines, the slider handle's
  outline (three of the colours, a width; where the width is `None` a connector draws its own outline, where it is 0
  none); the behaviour list `defaults.font_whole_pixels`, where unstated the sizes keep their fraction.
- **The rationale's status line**: "Not yet approved." → "Approved by the maintainer on <date>; specified in `<spec
  path>`."; at the close-out, "Built in 0.7.2 (<sha>); archived in `docs/archive/`." (as 6.H.4).
- **`docs/todo.md`** (dated lines `*<what> (0.7.2, <date>, <sha>):*`; closed entries move to
  `docs/archive/todo_closed.md` under their heading with the commit):
  1. *KDE values that wait for a connector design*: (5) → "done in 0.7.2 (below)", a dated line (E4's values and
     deviations); all five done, the entry closes (U2).
  2. *Model: the menu bar has no fill of its own*: closed by E3, moved (U1).
  3. *What Breeze draws that the model has no field for* (stays `[v0.7.x]`): its opening is rewritten to §3's
     leftovers — S1–S4 (one unit, with the scroll-bar request upstream), L2 (with the 2026-11-20 values #ecf5fb /
     #16232b, outline pixel #cce5f7 / #1a3a4b), L3 (waits for a clean-room `lighter`), L4 — and a dated line names the
     fields that took the rest (E1–E10, the `list.hover_text_color` gap included).
  4. New after it: "- [ ] [v0.7.x] **A menu's shortcut text.**" — Breeze letters the shortcut in a colour of its own
     at rest, hovered and pressed (`HighlightedText` at 0.7 pressed; measured, `2026-11-20/menus_*.txt`); no field in
     any state; no showcase colours it apart; the gap maps name the part on macOS (MAC-MN04), Windows (W1-048) and
     Material (MAT-M4).
  5. *Presets: values the M8 T15 platform-facts rows disagree with*, kde-breeze: the `slider.thumb_hover_color`
     clause goes, the item's opening parenthesis adds "and its slider's hover fill in 0.7.2 (E8)", a dated line after
     its *Issue #4 (0.7.1)* line.
  6. The entry "kde-breeze states `button.hover_background = "#93cee9"`": a dated line, "the slider's #93cee9 is gone;
     the sidebar's stays".
  7. *Slider: the thumb's outline or shadow* (stays): a dated line — KDE's outline is `slider.thumb_border_*`; other
     platforms' outlines and every shadow stay open (MAC-SL02, W1-069, GN-1.96/1.97, MAT-S9/S10).
  8. *Font sizes and the KDE button and dialog sizes* (stays): (1) and (3)'s width "done in 0.7.2 (below)" with a dated
     line (E11; E12: 82, the padding 6 right, the "7 px" from the line's outer edge); (3)'s `min_height`, (4), (5)
     rewritten as §3's one change (required fields made optional as `toolbar.bar_height` was, unsourced values unset,
     a dialog-icon helper beside `icon_size_dialog`); (2) stays; a new open point: Qt's line box for
     the 13 px body is 18 (`2026-11-20/qt-fonts-widgets/out_light.txt`, `implicitHeight 18.0`), the
     model's 13 × 1.36 = 17.68 (S3: line heights are not rounded).
  9. *GPUI Kit, from issue #4* (`[upstream]`): a dated line (E14), unsent: `PopupMenu` (item outline, pressed item,
     geometry), `AppMenuBar` (bar and title fills, title padding), `ListItem`/`DataTable` (row outline, header
     separators), `Slider` (handle outline), `Select` (open fill).

## B.F Facts rows and preset lines: placement

The archived Part F's rule holds: a new row goes directly after the row named as its anchor, found by its text, an
earlier unit's rows first (U2's `active_*` after U1's `hover_border_color`); rows are inserted only, no blank line
dropped; citations of lines they move are left (the line-reading gates stopped reading positions in v0.7.1). Preset
lines go into the named table at the named anchor, each key under a comment in the batch's form (`# platform-facts.md
(§x.y): <field>, …`; `# <field> not stated: platform-facts.md (§x.y), …`), light first, dark at the same anchors;
`check_platform_facts_rows.py` then finds each key's row (`defaults` keys anywhere in §2.1). The other platforms' cells
are `❓ — not collected; the parked gap map names <source> (<map> <ID> on v0.8.0-rc1 at 78ee0efd)`, or "…names none":

| New rows | macOS | Windows | GNOME | Material |
|---|---|---|---|---|
| menu `hover_border_color` | MAC-MN01, hover capture | W1-045, `MenuFlyoutItemMargin`, item `CornerRadius` | GN-1.80, HC inset ring, `_menus.scss:150-156` | MAT-M2, `_md-comp-menu.scss:93-101` |
| menu `active_*` | none | W1-044, `MenuFlyoutItemBackgroundPressed` | GN-1.66, `_menus.scss:146-148` | MAT-M2 |
| menu `bar_*` | MAC-N20 | W2-22, `MenuBar_themeresources.xaml` | GN-2.30, `_menus.scss:248-282` | none: G-m3 table A finds no menu bar |
| menu `panel_padding_*` | MAC-MN02, menu capture | W1-046, `MenuFlyoutPresenterThemePadding` | GN-1.69, `_menus.scss:58-66` | MAT-M7, `_md-comp-menu.scss:19, :24` |
| menu `min_row_height` | none | none (W1-046's `MenuFlyoutThemeMinHeight` is the presenter's, as `row_height`'s cell says) | none | none |
| list `hover_border_color` | none | W1-106 (fills only) | none | MAT-L4, `_md-comp-list.scss:43-89` |
| list `header_separator_color` | MAC-L05 | none | GN-1.169, `_column-view.scss:95-100` | MAT-L13, `_md-comp-data-table.scss` |
| slider `thumb_border_*` | MAC-SL02, the renders' rim | W1-069, `SliderThumbBorderBrush` | GN-1.96, `_scale.scss:31-35`; focus GN-1.104 | MAT-S10, `_md-comp-slider.scss:98-100`; MAT-S7 |
| toolbar `button_checked_border_color` | MAC-TL11† | W1-100, `AppBarToggleButton_themeresources.xaml` | none | none |
| combo `open_*` | MAC-CB04 | W1-131, `ComboBoxBackgroundPressed` | GN-1.237, `_buttons.scss:44-55` | MAT-CB3 |
| `font_whole_pixels` | none (sizes are points, logical pixels) | W-P3-13†, `UseLayoutRounding` | GN-3.56, `font-hinting`, `font-rendering` (Pango's rounding not read) | none |


# Part C — the gpui connector and its showcase (issue #4's leftovers, 0.7.2)

Owner: `connectors/native-theme-gpui/` (`src/geometry.rs`, `src/contract.rs`, `src/lib.rs`,
`src/widgets/slider.rs`, `src/widgets/mod.rs` (one export line), `src/widgets/tests.rs`,
`README.md`), its showcase `examples/showcase-gpui/` (`demo.rs`, `info/*.rs`, `tests.rs`); its
routes for S12's lines of `docs/showcase-elements.toml` (C.7), which the orchestrator edits.
Decisions: the rationale `docs/issues/0004_kde-leftovers-rationale.md` E1–E14, not
re-decided; field names are its, their registry order part A's. `GC`, `GB`, `GP` are the
`src/` folders of gpui-component 0.10.0, gpui-base 0.10.0 and gpui-pre 0.3.12 in Cargo's
registry, read 2026-11-20. Repository files are cited by anchor; only C.0's colour-claim
lines carry numbers, as the gate reads them.

## C.0 Lines that must not move

`every_colour_claim_is_read_at_the_line_it_cites` (`src/showcase.rs`) opens each claim at
its line. Cited on `d5df78d9` (every `native-theme-gpui/<file>.rs:N` in
`examples/showcase-gpui/` and `src/showcase.rs`): `colors.rs` 281–724 (140 lines);
`variants.rs:62-64, 134-136, 144, 163-165, 254, 255, 258, 259, 329`; `geometry.rs:230, 749,
784, 1463, 1468, 1491, 1492`; `widgets/slider.rs:79, 82`; lines of `widgets/checkbox.rs`,
`switch.rs`, `tab_bar.rs`, `progress.rs`, `separator.rs`, `spinner.rs`, `widgets/mod.rs:349`.
This part's edits move none. Not edited: `colors.rs`, `variants.rs`, every widget file but
`slider.rs` and `widgets/mod.rs` (its `pub use slider::{…}` line, above 349, edited in place);
`lib.rs` is cited by no claim.

- `geometry::menu_item` (above 749): `let r = with_height_rule(StyleRefinement::default(),
  m.row_height, n);` → `let r = row_height_rule(m.row_height, m.min_row_height, n);` and its
  doc's "`menu.row_height` is stated." → "… is stated, else at least `menu.min_row_height`.",
  each in place; its doc gains no line (the RTL sentence is the README's, C.5).
- `geometry::tooltip_content`'s doc (above 749): `src/root.rs:714` → `:725`, in place (Part C's U4).
- `geometry::toolbar_button_checked` (holds 1463, 1468): the arms in place,
  `Some(fill) => checked_pen(r.bg(rgba_to_hsla(fill)), t),`, `None => checked_pen(r, t),`;
  the 13-line doc reworded within 13 lines (*Not verified*).
- New private `geometry::row_height_rule`, `checked_pen`: after `default_button_rest`, before
  `list_item_selected`, below 1492.
- `widgets/slider.rs` (79, 82 in `SliderLook::of`): `SliderLook` untouched; the new
  `ThumbOutline` after `impl SliderLook`; `thumb`, `Slider::render` lie below 82.

New claims cite the new lines of `checked_pen` and `ThumbOutline::of` (pinned from then on);
colours the showcase draws itself cite `"showcase"`, as `info::chrome::app_menus` does.

## U1 — E1, E3, E5

### C.1.1 E1: the hovered menu row's outline

**Today.** `MenuLook::row(lit)` (`demo.rs`) fills a lit row `menu.hover_background` /
`hover_text_color`, rounded `menu.border.corner_radius`, for `MenuBar::popup` (`lit` =
`highlighted`, set by `on_hover` and Up/Down); `demo::menu_rows` (Overlays) takes
`MenuLook::of` but builds its own rows, lit by `.hover(..)`. No outline.

**Change.** `MenuLook` gains `edge: Option<Hsla>` (`menu.hover_border_color`; `None` where
unstated or `defaults.border.line_width` ≤ 0, as `combo_surface` filters its edge) and
`edge_width: Pixels` (`defaults.border.line_width`), and a box laid over the row's padding,
so nothing moves:

`fn edge(&self, lit: Option<bool>) -> Option<Stateful<Div>>`: `None` where `self.edge` is;
else `div().id("menu-row-edge").absolute().inset_0().rounded(self.radius)
.border(self.edge_width)`, its colour `edge` where `lit == Some(true)` and transparent
otherwise; a `.group_hover(MENU_ROW_GROUP, ..)` in every form, `edge` where `lit` is `None`
(hover-lit rows) and the box's own colour where `lit` is `Some` (no change; it gives the box
its hitbox); and `.group_active(MENU_ROW_GROUP, transparent)` where `menu.active_background`
is stated (E2: no outline on a pressed row). The id and the hitbox: gpui applies a
group-active style only to an element with state (`GP/elements/div.rs:3570-3579`), which its
mouse-down listener sets (`:3294-3320`), registered only for an element with a hitbox
(`:2616`, `:2643`); a group-hover style gives one, an active or group-active style does not
(`should_insert_hitbox`, `:2460-2486`; GPUI Fast's the same). `MenuLook::row` adds
`.relative().group(MENU_ROW_GROUP)` only; `menu_rows`' row, which does not call
`MenuLook::row`, takes `.relative().group(MENU_ROW_GROUP)` too, before its children.
`MenuBar::popup` appends `look.edge(Some(lit))` after the row's children, and `menu_rows`
appends `look.edge(None)` after its row's (gpui paints a later child over an earlier one). The form is `combo_surface`'s hover edge (an
absolute child, transparent at rest, a group-hover style, a `Normal` hitbox that leaves the
row's mouse handling alone),
tested by `a_hovered_drop_down_takes_the_hover_border`. `MENU_ROW_GROUP` is a new constant
beside `COMBO_HOVER_GROUP`. Info: `info::chrome::app_menus` and `info::overlays::menu_row`
claim "hovered item edge" `hover_border_color` through `edge_claim` (width
`defaults.border.line_width`).

### C.1.2 E3: the menu bar and its titles

**Today.** `demo::menu_bar` is an `h_flex` with side padding and no fill; in a client-side
title bar the titles lie on `geometry::title_bar`'s fill. `MenuLook::title(title, selected)`
pads with `menu.border.padding` (else `MENU_TITLE_PADDING_X`/`_Y`), fills an open or hovered
title `menu.hover_background`, letters it `hover_text_color`. The titles sit in a gap-less
`h_flex` (they touch), and with one menu open the next title's `on_hover` opens its menu.

**Change.** `menu_bar` takes `.bg(info::stated(toolbar.background_color))` where a native
theme is installed (the title-bar host untouched); `info::chrome::menu_bar` gains `native:
Option<&ResolvedTheme>` and claims "bg" `background_color` (`"showcase"`) in place of "fill:
none of its own". `info::chrome::app_menus`: the `MenuHost::Row` fill note ("none behind the
titles … the model states no menu-bar fill") names `toolbar.background_color` where a native
theme is installed, and the "hovered or open title, hovered item" `hover_background` claim
names the titles only where `bar_hover_background` and `bar_open_background` are unset.
`MenuLook` gains `bar_hover`, `bar_open`, `bar_text`, resolved by E3's fallbacks (`bar_hover_background.unwrap_or(hover_background)`,
`bar_open_background.or(bar_hover_background).unwrap_or(hover_background)`,
`bar_hover_text_color.unwrap_or(hover_text_color)`), and `title_padding`,
`menu.border.padding` with `bar_padding_horizontal` over left/right and
`bar_padding_vertical` over top/bottom where stated. `title` uses them (open `bg(bar_open)`,
else `.hover(bg(bar_hover))`, text `bar_text`, today's rem fallbacks, radius unchanged);
`app_menus`' info claims the three through `stated_claim`.

### C.1.3 E5: the hovered list row's outline

**Today.** List and Tree rows (`ListRow`, `tree_row`) are `ListItem`s in `LIST_ROW_GROUP`,
filled `list_hover` by upstream under the pointer unless active (`GC/list/list_item.rs:229-235`),
the label following the group (`row_label`); `files_table`'s body rows are the showcase's
`h_flex().relative()` with `.hover(..)` unless selected, the last rounded `bl`/`br` by
`list_inner_radius`. No outline. **Change.** The same box, also upstream's form for a
right-clicked row (`GC/list/list_item.rs:268-276`, an `absolute().inset_0()` child repeating
the item's radii):
- `demo::list_hover_edge(cx, shown, row_info) -> Option<Hsla>` beside `list_hover_text`, in
  its form: `list.hover_border_color` where `shown`, stated and `defaults.border.line_width`
  > 0; it records the `edge_claim` on `row_info`.
- `ListRow`, where `!selected && !right_clicked`: `.suffix(..)` returns
  `div().absolute().inset_0().border(w).border_color(transparent)` with the row's corners
  (`tl`/`tr` on the first row, `bl`/`br` on the last, `inner`) and
  `.group_hover(LIST_ROW_GROUP, |s| s.border_color(edge))`; `ListItem` puts the suffix into
  its `relative()` root (`RenderOnce for ListItem`).
- `tree_row`, where `!selected`: the box is a child of its suffix's outer element, after
  `.info(..)` and `.size_full()` (`InfoExt::info` wraps the bare `div()` in a new element, and
  the size is set on the wrapper; a child of the inner `div()` would lie in an element of no
  height). A right-clicked tree row takes it under the pointer too (S5's limit).
- `files_table`: each unselected body row `.group(TABLE_ROW_GROUP)` (new constant) and the
  box as its last child, square but the last row's `bl`/`br`; `info::data::native_table_row`
  claims it.

**Trial T-L1, first in U1** (rationale §4; T-K1's form). Two `#[gpui::test]`s in
`src/widgets/tests.rs` (the scene reader is there), constants only, kde-breeze light, on
gpui-pre and on GPUI Fast; record `<record>/leftovers/trial/t-l1-result.txt`, `t-l1.patch`:
1. `trial_a_list_row_takes_an_edge_under_its_group_hover`: `ListItem::new("row")` in a
   group with a label and the suffix box, edge `#3daee9cc`. At rest no quad bordered in it;
   pointer on the row (`simulate_mouse_move`, as `hover_and_press`): exactly one, the row's
   bounds, border width 1; the row's `list_hover` fill quad still drawn; the row's bounds
   equal in both frames.
2. `trial_a_menu_row_edge_hides_while_pressed`: `div().id("row").relative().group(G)` with
   `.active(bg ACTIVE)` and, in turn, `edge(None)`'s box (hover-lit) and `edge(Some(true))`'s
   (the popup's: a fixed edge colour, the no-op group hover, the group active): hovered, edge
   drawn; mouse down, fill `ACTIVE` and no edge quad; up, edge again.
Pass: all on both backends; a failure stops U1 for re-specification (the looks are not cut).

**U1's tests** (showcase `tests.rs`, kde-breeze light, `a_hovered_drop_down_takes_the_hover_border`'s
form; adwaita: no new look): `a_hovered_menu_row_takes_the_hover_border` (Theme menu opened
by `MenuBar::open_named`, pointer on "System": the last box over the row edged
`hover_border_color`, none at rest; Overlays' first row likewise);
`the_menu_bar_takes_the_toolbars_fill`; `a_menu_title_takes_the_bars_fills_and_padding`
(kde-breeze's two fills are both #3daee9, so `bar_open_background` is set to #f0a014, the reader
tests' hover 240,160,20, on the resolved theme first: open #f0a014, `bar_hover_text_color`, width
the label + 20, height the line + 12; hovered closed #3daee9; the three cleared: today's look);
`a_hovered_list_row_takes_the_hover_border` (rows 1 and 3; the selected row 2 never),
`a_hovered_tree_row_takes_the_hover_border` (an unselected tree row: the box in its suffix
edged `hover_border_color` under the pointer, none at rest),
`a_hovered_table_row_takes_the_hover_border` (row 1; the selected row 2 never).

## U2 — E2, E4

### C.2.1 E2: the pressed row

`MenuLook` gains `active` and `active_text: Option<Hsla>` (`menu.active_background`,
`active_text_color`) and `fn pressed(&self, row: Stateful<Div>) -> Stateful<Div>`: one
`.active(..)` with `bg(active)` and `text_color(active_text)` where each is stated (one
closure: `fn active` replaces an earlier one, `GP/elements/div.rs`). Called on the popup's
row after `.id(("menu-row", ix))`, and on `menu_rows`' row, which takes an id of its own
(`div().id("row")`, unique under the wrapper's) and `look.pressed(..)` before `.info(..)`:
`InfoExt::info` wraps the row in a new element, so a style set after it would land on the
wrapper, under the row's own hover fill and text. The edge hides on press where `active` is
stated (C.1.1); an unset text keeps the hover text, an unset fill
the hover look and its edge. The shortcut is the row's plain `Kbd` text and takes the pressed
text too (no shortcut colour in the model, rationale §3). Info: `stated_claim`s "pressed
item", "pressed item text".

### C.2.2 E4: geometry

- **Panel inset.** `MenuLook::popup_padding` becomes `popover.border.padding` with
  `menu.panel_padding_horizontal` over left/right and `panel_padding_vertical` over
  top/bottom where stated; `MenuBar::popup` pads with it as today. `menu_rows`' frame, which
  pads nothing today (correction 4), takes the two where stated.
- **Row gap.** `MenuBar::popup`'s `.gap(MENU_POPUP_ROW_GAP)` is 0 where
  `panel_padding_vertical` is stated (S10).
- **Radius, padding 14/27, icon gap 4.** Values (part B), arriving by `MenuLook::radius`
  and `menu_item`'s `with_padding` and `gap_x(icon_text_gap)`; the popup replaces the gap
  with `MENU_SHORTCUT_GAP` and puts the shortcut flush right (`justify_between`) inside the
  right padding; only the Overlays rows show an icon.
- **Row height.** New, at C.0's place, `fn row_height_rule(stated: Option<f32>, least:
  Option<f32>, n: Native<'_>) -> StyleRefinement`: `(None, Some(least))` →
  `StyleRefinement::default().min_h(px(least))`; otherwise `with_height_rule(StyleRefinement::default(),
  stated, n)` (a stated `row_height` wins, E4). `MenuLook` gains `min_row_height`;
  `MenuLook::row`'s `MENU_ROW_HEIGHT` fallback applies only where `row_height`,
  `min_row_height` and both vertical sides are unstated. `min_h` is the border box: 29 up to
  a 21 px line, the line + 8 above (E4's one pixel).
- **RTL.** `with_padding` sets physical sides; builders take no `&App`, so cannot read
  `LayoutDirection` (`GB/layout_direction.rs`, `LogicalStyledExt::ps`/`pe`); the showcase is
  LTR (`main.rs`, `LayoutDirection::Ltr.set_global`). The README's `menu_item` row (C.5; not
  its doc, which holds no new line, C.0): in an RTL layout the stated left side stays left; the
  application swaps the sides or pads with `ps`/`pe` itself (S10).

**U2's tests.** `geometry.rs`: `menu_item_takes_the_least_row_height` (kde-breeze:
`min_size.height` 29, no `size.height`; `row_height` 30 set too: `size.height` 30, no min;
adwaita: `size.height` 32, its stated `row_height`, no `min_size.height`). Showcase:
`a_pressed_menu_row_takes_the_pressed_look` (mouse down on "System": fill `active_background`,
no edge; the press only, as a release runs the row's action, closes the menu and switches the
theme, and T-L1 case 2 checks the release; Overlays' first row likewise); `the_theme_menu_has_breezes_geometry`
(kde-breeze: the first row 3 below the frame's inner edge, rows touching, 29 high, the label
14 from the row's left, the shortcut ending 27 from its right).

## U3 — E7, E8, E9, E10

**C.3.1 E7.** `files_table`'s `row` closure draws in every row a
`widgets::Separator::vertical()` in `grid` at the first cell's right edge, and under the
header (`closed`) a horizontal one. `TableLine` gains `line: Hsla`: the header passes
`l.header_separator_color.map(colour).unwrap_or(grid)`, body rows `grid`; both separators
take it, laid over the cell's fill and a section's hover fill. The two columns are
`relative(0.5)` each, so no empty area follows the last section (E7's check). Test
`the_table_headers_separators_take_their_colour` (kde-breeze: the header's vertical and
bottom quads `header_separator_color`, a body row's line `grid_color`; adwaita: `grid_color`).

**C.3.2 E8, `src/widgets/slider.rs`.** Today `thumb()` paints `bg(look.outline)`
(`fill_color` at `THUMB_OUTLINE_ALPHA` 0.5), `p(THUMB_OUTLINE_WIDTH)` 1 px, an inner circle
in `thumb_color`, `thumb_hover_color` over it under `THUMB_GROUP`'s hover. New, after `impl
SliderLook` (C.0):

`pub struct ThumbOutline { pub width: Pixels, pub rest, hover, focus: Option<Hsla> }`
(`Clone, Copy, Debug, PartialEq`, `#[non_exhaustive]`, as `SliderLook`) and
`ThumbOutline::of(resolved, disabled) -> Option<Self>`: `None` where
`thumb_border_line_width` is unset (today's ring); a stated width, 0 included, gives `Some`
(0: no ring and no line, S7); `rest` `thumb_border_color`, `hover`
`thumb_hover_border_color`, `focus` `thumb_focus_border_color`, each unset one `rest`;
disabled, no `hover`/`focus`. `ThumbOutline::faded_over(self, ground: Hsla, look:
&SliderLook) -> Self`, called in `Slider::render` beside `look.faded_over(..)` with the look
before its fade: `rest` faded as `SliderLook::faded_over` fades `outline` (over the fill
composited on the rail, at `look.opacity`). Each colour read on a line of its own (the claims
in `info/inputs.rs` cite them). `thumb()` takes `Option<&ThumbOutline>` and `focused`: with
one, the outer `bg` is `focus` where focused (the end thumb, which the keys move and the ring
is drawn on, S7), else `rest`, else the thumb's colour as the look gives it (S7: the thumb shown
whole), `p(width)`, and `.hover(|s|
s.bg(hover))` on the `SliderThumb` (`StatefulInteractiveElement`, `GB/slider.rs:1036-1042`;
its render sets no hover); without, today's ring. `Slider::render` passes `focused` (any
focus, not the ring's `shows`: Breeze shows it after a click too). Exported beside
`SliderLook` (`widgets/mod.rs`); `contract.rs`' `sinks` inserts `ThumbOutline::of({disabled})`
beside `SliderLook::of`. `Slider`'s doc and the README state the drag limit (correction 6).
Test `the_thumb_outline_is_the_themes` (`widgets/tests.rs`, a rendered `#[gpui::test]` read
from the scene, as the tests that use `hover_and_press` and `focus_next_to`): kde-breeze
light with `thumb_focus_border_color` set to #f0a014 on the resolved theme (its hover and focus
colours are both #3daee9): rest `#d1d1d2`, hovered `#3daee9`, focused (`focus_next_to`) not
hovered `#f0a014`, focused and hovered `#3daee9` (hover wins, S7), width 1; adwaita: today's ring.

**C.3.3 E9.** New private `checked_pen(r, t)` (C.0): `r.border_color(rgba_to_hsla(c))` where
`t.button_checked_border_color` is `Some(c)` (on a line of its own), else `r`. The width is
`toolbar_button_rest`'s (`button.border.line_width`), given where
`button_hover_border_color` is stated; kde-breeze states both (S8). The reworded doc adds
the pen and the limit: a selected Button takes no hover or press style
(`GC/button/button.rs:733`, `:758`), so the pen stays under the pointer and pressed (S8).
Showcase: `info::buttons::toolbar_checked` claims "checked edge"
`button_checked_border_color` through `edge_claim` at `checked_pen`'s line; `icon_button`'s
`(true, Some(true))` edge note names the pen where stated. Tests:
`a_checked_toolbar_button_takes_the_checked_pen` (`widgets/tests.rs`, the build of
`a_checked_toolbar_buttons_icon_takes_the_checked_text`; kde-breeze light and dark: border
`#d1d1d2` / `#535659`, width 1); `geometry.rs`'s `toolbar_button_checked_takes_the_checked_pen`
(stated: `border_color` set; cleared: unset).

**C.3.4 E10.** No gpui code: an open `Select` draws `ring` (`GC/select.rs:559`,
`outline_visible`; `:596-598`) and exposes no open state. The showcase's Theme drop-down is a
`Combobox` (`demo::preset_combobox`), which draws `ring` while open too (`GC/combobox.rs:638`,
`outline_visible`; `:1043-1045`); it tells a custom trigger `ComboboxTriggerContext::is_open`
(`:55`); `preset_combobox` gives it none. Contract and README only.

## U4 — E11, E12

- **E11.** `config.rs`' `to_theme_config` sets `config.font_size =
  Some(prefs.scaled(d.font.size))`: the text scale multiplies the model's rounded size, as in
  Qt. gpui-component sets the window rem to `font_size` (`GC/root.rs:725`), so under kde-breeze (13 px for 10 pt) every rem length is
  2.5 % shorter: upstream's paddings and heights and the showcase's rem fallbacks on
  unstated sides (`MENU_ROW_PADDING_X`, `MENU_POPUP_MIN_WIDTH`, `MENU_SHORTCUT_GAP`). No
  code change; `config.rs`' tests compare with the resolved size and follow it. Docs:
  `lib.rs`' "gpui-component src/root.rs:714" → "gpui-component 0.10.0 src/root.rs:725", and
  `geometry::tooltip_content`'s `src/root.rs:714` → `:725` in place (C.0) (0.9.1's line 714 and
  0.10.0's 725 are the same, `window.set_rem_size(cx.theme().font_size)`); the doc of
  `a_section_heading_takes_the_section_heading_role`, "a 13.33px body" → "a 13px body".
- **E12.** No code change: `geometry::button`'s `min_w(px(button.min_width))` is the border
  box, without shadows. Under kde-breeze the Basic buttons under 82 px grow 2 px; the asset
  run's parity proposal takes the moved keys.

## C.5 E14 and documentation

- `README.md`, after the paragraph "What stays upstream work (…)": "Issue #4's KDE looks
  that gpui-component's own widgets cannot take: `PopupMenu` (a hovered item's outline, the
  pressed item, its inset, row gap, padding and height, which are constants), `AppMenuBar`
  (the bar's fill and the titles' fills, text and padding: its titles are ghost Buttons in
  the shared tokens), `ListItem` and `DataTable` (a hovered row's outline; the header's
  separators, in the shared `border`), `Slider` (the thumb's outline) and `Select` (an open
  combo box's fill and frame: it shows `ring` while open). The showcase draws the first
  four with its own elements; the `widgets` module's `Slider` draws the thumb's outline."
- README geometry table: `menu_item` + "`menu.min_row_height`, a least height where
  `menu.row_height` is unstated" and the RTL sentence; `toolbar_button_checked` +
  "`toolbar.button_checked_border_color` at `toolbar_button_rest`'s width (none of its own
  where no hover outline is stated: upstream's border width then), kept under the pointer".
  *Theme-drawn widgets*: the thumb outline and its drag limit.
- `lib.rs`: module table, `menu` row + the least row height, `slider` row + "the thumb's
  outline via [`widgets::ThumbOutline`]". Its gpui-component 0.9.1 `src/theme/schema.rs`
  citations, each diffed against 0.10.0 in Cargo's registry (S11): both `:1039-1070` (`apply`'s
  step 8; the selection-restore doc, "gpui-component 0.9.1") → "gpui-component 0.10.0
  `src/theme/schema.rs:1043-1074`" (the 32 lines identical); `apply`'s `:1079-1083, :1125` →
  `:1083-1087, :1129`, and the test doc's `:1125` → `:1129` (identical); the blink comment's
  `:1078-1125` → `:1082-1129` (one line differs, `highlight_theme.clone()` becoming
  `highlight_theme`, the same assignment); the private colours' `:657-691` → `:660-694`
  (identical). The README's `src/theme/schema.rs:1039-1070` → `:1043-1074` likewise.
- `examples/showcase-gpui/info/mod.rs`' `GEOMETRY_NOTES`: the `menu_item` and
  `toolbar_button_checked` entries name the new fields (no new `pub fn`).

## C.6 Contract (`src/contract.rs`)

The colour walk needs each leaf to move a sink in one base, or one naming:

| Leaf | gpui |
|---|---|
| `menu.hover_border_color`, `active_background`, `active_text_color` | `UNREAD_COLOURS` `NoReceiver`: "a `PopupMenu` item lights its hover and selection with `accent` and `accent_foreground` only (`menu/menu_item.rs:134-141`); the showcase draws it" |
| `menu.bar_hover_background`, `bar_open_background`, `bar_hover_text_color` | `NoReceiver`: "`AppMenuBar`'s titles are Small ghost Buttons (`menu/app_menu_bar.rs:276-280`), its bar has no fill (`:122-143`); the showcase draws it" |
| `list.hover_border_color` | `NoReceiver`: "a hovered `ListItem` takes a fill only (`list/list_item.rs:229-235`)" |
| `list.header_separator_color` | `NoReceiver`: "a `DataTable` header's line is `border`, every frame's token (`table/state.rs:2191-2192`, `:2295-2296`)" |
| `slider.thumb_border_color`, `thumb_hover_border_color`, `thumb_focus_border_color` | reach the new `ThumbOutline::of(..)` sinks (kde-breeze) |
| `toolbar.button_checked_border_color` | reaches `geometry::toolbar_button_checked`, a sink already |
| `combo_box.open_background`, `open_border_color` | `NoReceiver`: "an open `Select` or `Combobox` draws `ring` and has no open state to style (`select.rs:559`, `:596-598`; `combobox.rs:638`)" |

No gpui walk reads non-colour leaves: `menu.min_row_height` reaches `geometry::menu_item`,
`slider.thumb_border_line_width` `ThumbOutline::of`, the paddings the showcase only;
`defaults.font_whole_pixels`, a resolved leaf (S3), acts before the connector and takes no row,
as `defaults.border.opacity` takes none.

## C.7 Routes for `docs/showcase-elements.toml`

The list is S12's. Each leaf it adds takes an element-prefixed gpui `HOW` line
(`info/leaves.rs`, `every_listed_leaf_has_a_route`) before the generic `""` ones, untrue here
(`defaults.border.line_width_px` "the swatch's frame width", `toolbar.background_color`
"geometry::toolbar's fill"); upstream citations carry their symbol
(`every_prose_citation_still_exists`; e.g. "gpui-component 0.10.0 button/button.rs:758, selected").
Two of them gpui does not draw, routed "not reachable": `menu.icon_text_gap_px` on the Theme
menu's items (they show no icon, and the popup spaces the shortcut by `MENU_SHORTCUT_GAP`,
C.2.2), and the two `combo_box.open_*` on `basic.drop_down.trigger` (C.3.4's reason).

## Order and gates

U1 (T-L1 first), U2, U3 (E10 last, the first cut), U4. Per round: the tests named, and the
guards `every_colour_claim_is_read_at_the_line_it_cites`, `every_colour_leaf_reaches_a_sink_or_is_named`,
`every_listed_leaf_has_a_route`, `every_prose_citation_still_exists`.


# Part D — the iced connector and its showcase

Rationale: `docs/issues/0004_kde-leftovers-rationale.md`, E1–E14 (settled). Form: the archived
`todo_v0.7.2_kde-open-items-steps6-7-spec.md`, 6.E and 6.G. Read on `v0.7.2-rc1` at `d5df78d9`
(worktree `lane2-b`), 2026-11-20. `IW`, `AW`, `IC` = the `src/` of iced_widget 0.14.2, iced_aw
0.14.1, iced_core 0.14.0 in Cargo's registry (pinned lines are theirs); repository files are cited
by anchor. `C` = `connectors/native-theme-iced/`, `SC` = `C/examples/showcase-iced.rs`. Field names
and types are part A's: each new field is a soft option, `Option<Rgba>` / `Option<f32>` resolved.
Files: `C/src/styles.rs`, `C/src/styles/aw.rs`, `C/src/contract/{rows,derived}.rs`,
`C/contract-leaves.toml` (generated), `C/src/lib.rs` (two tests), `C/README.md`, `SC`; its routes
for S12's lines of the shared `docs/showcase-elements.toml`, and its input to
`scripts/connector_parity_exceptions.toml` (the orchestrator edits both, S12).

**D.0 A fact every menu item depends on.** iced_aw paints `path` and `path_border` only under
`DrawPath::Backdrop` (`AW/widget/menu/menu_bar.rs:598-621` under the open title,
`AW/widget/menu/menu_tree.rs:783-801` under the active item). Its default is `FakeHovering`
(`menu_bar.rs:143`): no path, the item handed a pointer at its centre instead
(`menu_bar_overlay.rs:394-401`, `menu_tree.rs:498-511`). No showcase menu sets a draw path, so every
highlight on screen is the item's own button style, `SC::menu_row` or `SC::menu_title`;
`styles::aw::menu`'s `path`/`path_border` reach the screen only in an application that sets Backdrop.

## U1 — the looks (E1, E3, E5)

### D.1 E1, the hovered item's outline

**Today.** `styles::aw::menu` sets `path_border: iced.path_border`, iced_aw's `Style::default()`'s
(`AW/style/menu_bar.rs:49-52`: width 0, radius 6). `SC::menu_row` draws every row of every showcase
menu (window menus, page-tab menu, Extra page drop-downs and context menu), with no border.

**`styles::aw::menu` and its twin `SC::menu_bar_style`** (the same expression in both, as
`the_window_menus_draw_as_the_connectors` compares them over every bundled combination). Where
`menu.hover_border_color` is stated: `path_border: Border { color: composite_over(to_color(c),
highlight), width: resolved.defaults.border.line_width, radius:
Radius::new(m.border.corner_radius) }` (`highlight` = `menu.hover_background`); unset,
`iced.path_border`. The twin's doc sentence that "the highlight's border" has no source is
rewritten with it. Composited: iced paints a quad's border in place of its fill (why
`styles::toggler` composites its track edges, `over_track`), Breeze lays the line over the fill;
kde-breeze's line is opaque, so the composite is itself. Doc: the shadows stay "no native source";
`path_border` as above; the limit: D.0, and under Backdrop iced_aw also outlines an open bar title,
which Breeze does not.

**`SC::menu_row`** (the only new logic of E1 and E2):

```rust
let plain = iced::Border::default().rounded(radius);
let edge = m.hover_border_color.map(|c| iced::Border {
    color: styles::composite_over(to_color(c), hover),
    width: resolved.defaults.border.line_width,
    ..plain
});
let pressed = m.active_background.map(to_color);                                   // E2
let pressed_label = to_color(m.active_text_color.unwrap_or(m.hover_text_color));
// Active (None, label, plain); Disabled (None, disabled_label, plain);
// Hovered (Some(hover), hover_label, edge.unwrap_or(plain));
// Pressed pressed.map_or((Some(hover), pressed_label, edge.unwrap_or(plain)),
//                        |fill| (Some(fill), pressed_label, plain))
```

iced draws a button's border on its bounds, over the padding (`IW/button.rs:379-385`, cited in
`styles::toolbar_button`'s doc): nothing moves. `SC::menu_title` stops delegating to `menu_row`
(D.2), so a bar title never takes the outline. Docs name the fields.

**Contract.** `AW_MENU_BORDER_ROWS` gains `BorderRow { field: "styles::aw::menu.path", statuses:
AW_MENU_STATUSES, native: |r, _| native_aw_path_border(r), .. }`: stated, `NativeBorder { color:
over(colour, hover_background), width: defaults.border.line_width, radius: menu.border.corner_radius }`;
unset, `iced_aw::style::menu_bar::Style::default().path_border`'s colour, width and uniform
`radius.top_left`, as `TOGGLER_SCALAR_ROWS` fall back to `toggler::default`. The six
`styles::aw::menu.path.border.*` `DERIVED` entries go, and "`path_border` has no native source".

### D.2 E3, the menu bar

**Today.** `styles::aw::menu` and its twin `SC::menu_bar_style` (equal by
`the_window_menus_draw_as_the_connectors`): `bar_background` `window.background_color`. `SC::menu_bar`
wraps the bar in an unstyled `container(bar).padding(side)` above the toolbar; the page-tab caret
menu, `SC::page_menu`, is a one-item `MenuBar` in `menu_bar_style`.

**The fill.** `styles::aw::menu` and `menu_bar_style`: `bar = to_color(resolved.toolbar.background_color)`.
Doc: "The bar is the tools area's fill, `toolbar.background_color`: Breeze fills the menu bar and the
toolbars as one area; on every other bundled preset it resolves to `defaults.background_color`, as
the window's does". `SC::menu_bar`'s container takes that colour as its background, so the margins
match the bar and the toolbar. **The caret menu keeps today's fill** through a wrapper in
`page_menu`: `let style = menu_bar_style(resolved);` and `.style(move |t, s| Style { bar_background:
Background::Color(window), ..style(t, s) })`, `window` = `window.background_color`; so
`menu_bar_style` stays the connector's twin and its test is unchanged.

**The titles' fills** (showcase only: iced_aw has one `path` for titles and items, D.0).
`SC::menu_title(resolved, open)` builds its own style: `hover` = `bar_hover_background`, else
`menu.hover_background`; `open_fill` = `bar_open_background`, else `hover`; `lit` =
`bar_hover_text_color`, else `menu.hover_text_color`. `Disabled` as `menu_row`'s; `open` or
`Pressed` → `(open_fill, lit)`; `Hovered` → `(hover, lit)`; `Active` → `(None, label)`; border
`plain`. With the three unset this is today's look in every status. Doc: on the Extra page
(`menu_root` passes `open: false`) an open title is known only by iced_aw's fake pointer and shows
`bar_hover_background` (equal to the open fill in Breeze's schemes).

**The titles' padding.** `fn title_padding(m: &ResolvedMenuTheme, rows: Padding) -> Padding`: left
and right `m.bar_padding_horizontal`, top and bottom `m.bar_padding_vertical`, each pair `rows`'
where unset. Read by `SC::menu_bar`'s `title` closure (`rows` = its `pad`) and the Extra page's
`menu_root` (`item_pad`). Titles already touch in both bars (the Extra page's comment "The titles
side by side with no gap"); `MenuBar::padding`, the bar's inset, stays the showcase's.

**Contract.** `AW_MENU_ROWS`' `bar_background` row reads `r.toolbar.background_color`. The three
title colours into `UNREAD_COLOURS`, `Unread::NoReceiver`, after `menu.disabled_text_color`'s:
"iced_aw paints an open bar title's highlight with the menus' `path`, one `Style` asked for
`Active` alone (`iced_aw-0.14.1/src/widget/menu/menu_bar.rs:587`, `:598-621`,
`src/style/menu_bar.rs:7-26`), and a title is the consumer's own widget".

### D.3 E5, the hovered list row's outline

**Today.** `SC::list_row` (Basic list rows, command palette rows): hover fill and text, no border.
The Basic list's button style is `background: None, ..row(..)`; `list_row_fill`, an underlay,
paints the fill rounded where it meets the frame, with `..style.border`. The Basic table's rows are
containers ("not reachable: the table's rows are containers, not hoverable", `iced_route`).

**Change.** `list_row`: an unselected row's `Hovered | Pressed` take `Border { color:
composite_over(to_color(c), hover), width: resolved.defaults.border.line_width, ..default }` where
`list.hover_border_color` is stated (composited as in D.1: #3daee9cc over a #3daee94d fill); a
selected row none. The Basic list's button style adds `border: Border::default()`, so the outline is
drawn once, by `list_row_fill`, on the fill's rounded shape (E5: "on the shape the showcase gives
the hover fill today"); the palette rows draw it on the button. The table: unchanged (S5).

**Contract.** No iced receiver: `UNREACHABLE` gains `list.hover_border_color` after
`list.grid_color`'s (`cfg(feature = "iced_aw")`): "a `SelectionList` fills a hovered row with a
quad whose border is hardcoded 0 wide (`selection_list/list.rs:226-249`)", `lost: |_, r|
r.list.hover_border_color.and_then(lost_color)`, its exceptions the 32 combinations but
kde-breeze's two, "the preset states no row outline, a soft option, so there is none to lose"
(`dialog.entrance`'s form);
`styles::aw::selection_list`'s doc names it beside `.alternate_row_background`.

## U2 — menus, the rest (E2, E4)

### D.4 E2, the pressed item

**Today.** `menu_row`'s `Pressed` is the hover pair. iced_aw's path has no pressed status
(`Status::Active` alone, `AW/widget/menu/menu_bar.rs:587`); iced's drop-down `overlay::menu::Style`
has a selected look only (`IW/overlay/menu.rs:598-611`).
**Change.** D.1's code: `menu.active_background` under `active_text_color`, else
`hover_text_color`, no outline; with the fill unset, the hover look and its outline under that
label. No iced menu draws an icon, so the `Selected` icon mode has nothing to reach.
**Contract.** `menu.active_background`, `.active_text_color` into `UNREAD_COLOURS` (`NoReceiver`)
after the title colours, with the two citations above and "an `iced_aw` menu's items are the
consumer's own widgets (`menu_tree.rs:830`)".

### D.5 E4, the menu's geometry

**Panel gap.** Today `SC::menu_popup_padding` = `padding_inside_border(&popover.border,
Padding::new(AW_MENU_PADDING))`, read by the window menus (`Menu::padding`, `.offset(popup_pad.top)`),
`HeldMenu`, the page-tab menu and, through `aw_menu_padding`, the Extra page's drop-downs and context
menu. iced_aw pads the panel quad out from the rows and draws the frame inside that quad
(`menu_tree.rs:764-780`; `padding_inside_border`'s doc). New: left/right
`menu.panel_padding_horizontal + popover.border.line_width`, top/bottom
`menu.panel_padding_vertical + popover.border.line_width`, each pair today's where unset;
kde-breeze 3 + 1 = 4, Breeze's highlight at x 4. Rows touching: the window and page-tab menus space
rows 0 already (`Menu::new`, `menu_tree.rs:155`); the Extra page's `aw_menu` (`.spacing(gap)`) and
its context menu's `column!` space them `AW_MENU_GAP`, 0 where `menu.panel_padding_vertical` is
stated, the panel's offset from its title kept (S10).

**Padding 14 / 27 and the gap.** No code: rows are padded by `padding_or(&m.border.padding, ..)`,
physical sides, the shortcut flush right inside the right padding (`space().width(Fill)`);
`menu_width` adds `pad.left + pad.right`, so a KDE menu grows 33 px. No iced menu draws an icon:
`menu.icon_text_gap` reaches nothing (as `styles::aw::menu`'s doc says). **RTL:** iced 0.14 has no
layout direction (`IC`, `IW` searched: the one "right-to-left" is text alignment, `IC/text.rs:87`);
`styles::aw::menu`'s doc: "padding is per physical side; a right-to-left application swaps it".

**Row height.** Today the window menus and the Extra page compute `row_height` (`rows::scaled_height`
of `menu.row_height`); the page-tab menu applies none. One helper replaces both and serves all
three (S10), each row's label centred (`height(Fill)`) where it returns `Some`:

```rust
/// A menu row's height: `menu.row_height`, exact at a text scale of 1 (rows::scaled_height);
/// else at least `menu.min_row_height`, grown to the row's line box and padding; else none.
fn menu_row_height(resolved: &ResolvedTheme, a11y: &AccessibilityPreferences, pad: Padding) -> Option<f32> {
    let m = &resolved.menu;
    let line = resolved.defaults.line_height;
    match (m.row_height, m.min_row_height) {
        (Some(h), _) => Some(rows::scaled_height(h, a11y, m.font.size, line, pad.y(), 0.0)),
        (None, Some(min)) => Some(min.max(scaled_text_size(m.font.size, a11y) * line + pad.y())),
        (None, None) => None,
    }
}
```

The line box is iced's unrounded size × line height, as `Typeset::themed` lays it and
`basic_table`'s `height` closure sums it. **Radius.** No code: `menu_row`, `menu_title`, D.1's
`path_border` and `styles::menu` (the drop-down's panel and highlighted row, `IW/overlay/menu.rs:550`)
read `menu.border.corner_radius`: on kde-breeze 5 today (`defaults.border.corner_radius`), 4.5 after.

## U3 — lists and controls (E7, E8, E9, E10)

### D.6 E7, the table header's separators

**Today.** `SC::basic_table`'s `cell` draws the first column's trailing line and each header cell's
bottom line (`grid_line`) in `TableColours::grid`, `separator.line_width` thick; the last section has
no trailing line, and the two `FillPortion(1)` sections fill the header: no empty area (E7's check).
**Change.** `TableColours` gains `header_line`: `list.header_separator_color`, else `grid`; `cell`
draws a header cell's two lines in it, a body cell's in `grid`. The line is a container fill over the
header's (or a hovered section's) fill, so a translucent colour is laid over it; no compositing.
**Contract.** `UNREACHABLE` + `list.header_separator_color` after `list.header_background`'s,
its evidence in that entry's form, `lost: |_, r| r.list.header_separator_color.and_then(lost_color)`
(the field is an `Option`), its exceptions the 32 combinations but kde-breeze's two, "the preset
states no header separator, a soft option, so there is none to lose" (`dialog.entrance`'s form,
as D.3's).

### D.7 E8, the slider handle's outline

**Today.** `styles::slider`'s handle `border_width`/`border_color` are iced's (0, transparent,
`IW/slider.rs:695-700`), named in `DERIVED`. `Status` is `Active`, `Hovered` (the pointer anywhere on
the slider, `:421-427`), `Dragged`; no focused status (`:578-585`); the handle is one quad, its border
inside it (`:506-522`).
**Change** (new code; uncomposited, as `styles::toggler`'s thumb edge; kde-breeze's colours are opaque):

```rust
let width = s.thumb_border_line_width.filter(|w| w.is_finite() && *w > 0.0);
let edge = |c: Option<Rgba>| width.zip(c).map(|(w, c)| (w, to_color(c)));
let rest = edge(s.thumb_border_color);
let hovered = edge(s.thumb_hover_border_color.or(s.thumb_border_color));
// closure: match status { Active => rest, Hovered | Dragged => hovered }
//          .unwrap_or((iced.handle.border_width, iced.handle.border_color))
```

`thumb_focus_border_color` is not read. A width unset or 0 gives iced's own (0, transparent), no
outline (S7); a stated width with the rest colour unset gives no line at rest. Doc: "`SliderTheme`
carries no border" gives way to the outline, with "no line at rest where `thumb_border_color` is
unset", and "iced's `Hovered` and `Dragged` cover the whole slider, not the handle alone, and it
has no focused status: `thumb_focus_border_color` is not drawn".
**Contract.** `native_slider_handle_edge(r, s) -> Option<(f32, Color)>`, the same rule, in
`native_toggler_thumb_edge`'s form; `SLIDER_ROWS` gains `styles::slider.handle.border_color`
(`map_or(Color::TRANSPARENT, |e| e.1)`), `SLIDER_SCALAR_ROWS` `styles::slider.handle.border_width`
(`|t, r, s| … map_or(slider::default(t, s).handle.border_width, |e| e.0)`); the two `DERIVED`
entries go. `slider.thumb_focus_border_color` into `UNREAD_COLOURS` after
`slider.disabled_track_color`'s: "`slider::Status` is Active, Hovered and Dragged, with no focused
state (`iced_widget-0.14.2/src/slider.rs:578-585`)". kde-breeze's removed `thumb_hover_color`: no
code change (unset, `styles::slider` keeps `thumb_color`).

### D.8 E9, a checked toolbar button's pen

**Today.** `styles::toolbar_button`: `(true, Active)` transparent `outline`; `(true, Hovered | Pressed)`
`hovered_outline` (`toolbar.button_hover_border_color`, else transparent).
**Change.** `(true, Active)`: colour `toolbar.button_checked_border_color`, else transparent;
`(true, Hovered | Pressed)`: `button_hover_border_color.or(button_checked_border_color)`, else
transparent (S8); width `button.border.line_width`, as the hover outline. Doc says so.
**Contract.** `native_toolbar_button`'s third value, the same two arms, and its doc.

### D.9 E10, an open combo box

**Today.** `styles::pick_list`: `Opened { is_hovered }` follows `is_hovered` (its doc: "the model
states no separate open appearance").
**Change.** `Opened { is_hovered }`: fill `composite_over(to_color(open_background), idle)` (layered
as the hover fill is), border colour `open_border_color`, each today's by `is_hovered` where unset;
width and radius `combo_box.border`'s. Doc rewritten. **Contract.** `native_pick_list_fill`'s `Opened`
arm and `PICK_LIST_BORDER_ROWS`' `native`, the same rule.
**The Theme row.** `SC::theme_picker_style` maps an editable `ComboBox`'s `Focused { is_hovered }` to
`Opened { is_hovered }`; unchanged, a focused Theme row on KDE takes the open pressed-button look,
which Breeze gives no editable combo box (E10). It maps `Focused` to `Hovered` / `Active` by
`is_hovered`: today's look on every preset (S11).

## U4 — fonts and sizes (E11, E12); E6

No connector code (iced takes the resolved `f32` through `scaled_text_size`); E6 changes no iced
value. Tests stating KDE values (searched `C/src`, `C/README.md`, `SC` for `13.33`, `18.13`, `80`):
`C/src/lib.rs`'s `control_line_height_reaches_the_stated_minimum`: the input's line box becomes
13 × 1.36 = 17.68, below the 18 a 32 px minimum leaves inside 6 + 1, so its `input_own` assert
becomes `18.0`, its comment ("below the theme's 18.13 line box, which stands") and doc ("13.33px")
rewritten; `button_content_min_size_is_the_minimum_inside_the_padding`: `(66.0, 18.0)` →
`(68.0, 18.0)`, "82 x 32 outer"; `SC`'s `the_buttons_page_takes_the_stated_minimum_size` and
`the_other_pages_push_buttons_take_the_stated_minimum_size` docs: "80 by 32" → "82 by 32".

## E14 — README

`C/README.md`, *What iced cannot take*: the `MenuBar` "lies on `toolbar.background_color`, the tools
area's fill" (was `window.background_color`); a bullet **Menu items** (D.0's Backdrop limit; a
pressed item, the titles' fills and padding and the rows' geometry are the application's items, as
the showcase's `menu_row`, `menu_title`, `menu_row_height`, `menu_popup_padding`; iced's drop-down
outlines no highlighted row, `IW/overlay/menu.rs:598-611`); a bullet **Lists and tables** (D.3, D.6:
no receiver); in the styles table's gloss, "`slider` has no focused status". `### iced_aw`: one
sentence on `path_border`.

## D.13 Contract leaves, tests, routes, elements, parity

**`C/contract-leaves.toml`**: `cargo test -p native-theme-iced --all-features --lib -- --ignored regenerate_contract_leaves`.
Expected, derived, not run (the walk splices a colour into every leaf, a null one too):
`menu.hover_border_color` moves `styles::aw::menu.path.border` (six statuses) in all 34;
`toolbar.background_color` gains `styles::aw::menu.bar_background`, `window.background_color` loses
it; `toolbar.button_checked_border_color` moves `styles::toolbar_button.border` `(true, Active)` in
all 34 and `(true, Hovered)`, `(true, Pressed)` `except` kde-breeze's two; `slider.thumb_border_color`
moves `styles::slider.handle.border_color` `Active`, and `.thumb_hover_border_color` `Hovered`,
`Dragged`, in kde-breeze's two only (the one preset stating the width); `combo_box.open_background`
and `.open_border_color` move `styles::pick_list.background` and `.border` under both `Opened` in all
34, and `.hover_background`, `.hover_border_color`, `.background_color`, `.border.color` gain
kde-breeze's two in the `except` of their `Opened` statuses; six `UNREAD_COLOURS` leaves (three title
colours, two pressed, the slider's focus) and two `UNREACHABLE` ones are named entries. The
walk takes colour leaves only, so the carried `defaults.font_whole_pixels` (S3) takes no row, as
`defaults.border.opacity` takes none. The regenerated file is the authority; a difference is
explained in the commit body.

**Tests** (`SC`; kde-breeze light and dark, adwaita light as the unset case; rationale values):
`menu_rows_outline_and_press_as_stated` (Hovered border #399acc / #5abaec, 1, radius 4.5; Pressed
#3daee9 under #ffffff / #fcfcfc, no border; adwaita `{:?}`-equal to today's body, kept in the test);
`menu_titles_take_the_bar_fills_and_padding` (`bar_open_background` set to #f0a014 on the
resolved theme, as kde-breeze's two fills are both #3daee9: open #f0a014, Hovered #3daee9, no
border, padding 10 / 6);
`menu_rows_are_at_least_the_min_row_height` (29; at text scale 2 the content; material-3's 48 exact);
`menu_panels_take_the_panel_padding` (4 a side); `a_hovered_list_row_is_outlined_once` (`LIST_FILLS`);
`the_menu_bar_row_is_the_toolbars_fill` (and the caret bar the window's). In `C` the contract rows
cover `styles::*`; `drawn_by` needs no new arm (`TableColours`' `Debug` carries `header_line`).

**Routes** (`SC::iced_route`, held by `the_element_list_parses_and_every_leaf_has_a_route` and
`every_hand_colour_route_names_a_function_that_reads_it`), for S12's list: each hand route names
the function this part gives the leaf (`menu_row`, `menu_title`, `title_padding`,
`menu_popup_padding`, `menu_row_height`, `list_row`, `list_row_fill`, `basic_table`,
`styles::slider`). That test takes each `"; "`-separated part's text before `": "` as a function
`drawn_by` must draw, so a colour route puts only `drawn_by`'s functions there (`menu_bar` and
`list_row_fill` have no arm): `toolbar.background_color` on `chrome.menu_bar` replaces the
`window.background_color` arm with "menu_bar_style: bar_background, under the row's container
(menu_bar)"; `list.hover_border_color` on `basic.list.*` is "list_row: the Hovered border";
`list_row_fill` stands only in `defaults.border.line_width_px`'s route on `basic.list.*`, which
that test does not check. The table's `list.hover_border_color` and `defaults.border.line_width_px`
are "not reachable" as its hover fill's; `menu.border.corner_radius_px` routes to `menu_row` on the
items ("the row's border radius") and `menu_title` on the titles; `menu.icon_text_gap_px` on the
items is "not reachable: no iced menu draws an icon" (D.5); `defaults.border.line_width_px` on the
menu items routes to `menu_row`. These `defaults.border.line_width_px` arms stand before the
generic `("defaults", "border.line_width_px")` one ("the swatch container's border.width"),
which would answer otherwise. The slider, toolbar and pick-list colours route from the export;
`unstated_tool_colour` gains a `"toolbar.button_checked_border_color"` arm in
`button_hover_border_color`'s form, and on a checked button the `button_hover_border_color` arm
names `toolbar.button_checked_border_color` in its place where that is stated (S8). Inks:
`aw_menu_inks` + `("path.border", Ink::edge(..))`, `slider_inks` + `("handle.border", ..)`,
`aw_menu_info` + a "path border" line.

**`scripts/connector_parity_exceptions.toml`**: iced adds no public function. For an accessor egui
adds that iced lacks (E9's), an `absent` entry under "absent: iced" in the form of
`toolbar_button_checked_background`'s ("`styles::toolbar_button` draws …; iced has no toolbar, so the
style is the receiver").


# Part E — egui connector, egui widgets, egui showcase

Rationale: `docs/issues/0004_kde-leftovers-rationale.md`, E1–E14 (settled). Form: the archived
`todo_v0.7.2_kde-open-items-steps6-7-spec.md`, 6.F and 6.G. Pinned upstream (Cargo's registry,
lines given): `EG` egui 0.37.0 `src/`, `EP` epaint 0.37.0, `EX` egui_extras 0.37.0. `CE` is
`connectors/native-theme-egui/`, `CW` `connectors/native-theme-egui-widgets/`, `SE`
`CE/examples/showcase-egui/`. Field names are the rationale's; their serde paths (the
`mapping.toml` keys) and registry order are the model part's. Counts are deltas on what the test
holds on its tree, as 6.F.3 gives them.

Files: `CE/src/style/roles.rs`, `atlas.rs`, `ext.rs`, `accessors.rs`, `lib.rs`, `roles.rs` (one
doc), `CE/mapping.toml`, `CE/src/mapping.md` (generated by `mapping_doc::render`: `cargo test -p
native-theme-egui --lib -- --ignored regenerate_mapping_md`, its header), `mapping_tests.rs`,
`contract.rs`, `install_tests.rs`, `CE/README.md`; `CW/src/slider.rs`, `CW/src/tests.rs`,
`CW/README.md`; `SE/chrome.rs`, `demo.rs`, `pages/basic.rs`, `info.rs`, `tests.rs`; egui's
routes for S12's lines of `docs/showcase-elements.toml` and its input to
`scripts/connector_parity_exceptions.toml` (the orchestrator edits both, S12). `convert.rs` is not changed: the radius takes `to_corner_radius` →
`u8_from_f32_saturating` as today.

### What draws each part today

| Part | Today | Where |
|---|---|---|
| Item outline | one stroke, `menu.border` (transparent, 0 on every preset), into `inactive`, `hovered`, `active`, `open` | `style/roles.rs`, `fn menu`, `BorderEntries::Interactive` |
| Pressed item | `active` copies `hovered` | `fn menu`: `FillSource`/`TextSource` `active: None` |
| Bar titles | the `Role::Menu` cell (`role_modifier`) with `item_spacing.x = 0`, `interact_size.y = 0`; an open title: `inactive` swapped for `hovered` | `SE/demo.rs` `menu_bar`; `SE/chrome.rs` `menu_bar` |
| Bar fill | `window.background_color` | `SE/chrome.rs` `menu_panel` |
| Panel margin | egui's `menu_margin`; the showcase writes `popover.border.padding` over it | `SE/demo.rs` `menu_modifier`; `mapping.toml` `[unwritten]` `spacing.menu_margin.*` |
| Padding, gap, radius, row | `to_button_padding` (each pair's mean), `menu.icon_text_gap`, `menu.border.corner_radius`, `row_height` + `fit_row_padding` | `fn menu` |
| List row hover | fill only | `SE/pages/basic.rs` `list_row` |
| Basic table | no row hover; header sections hovered/pressed; grid in `list.grid_color` | `SE/pages/basic.rs` `table` |
| Widgets' slider knob | outline = the state's `fg_stroke` | `CW/src/slider.rs` `slider_ui` |
| Checked tool button | `Button::fill(toolbar_button_checked_background)`, the scope's stroke | `SE/pages/basic.rs` `icon_buttons`, `edit_look` |
| Open combo box | `open` copies `inactive` | `fn combo_box`, `OpenFrom::Inactive`; egui reads `open` under `ComboBox::CLASS_OPEN` (`EG/theme/default_style.rs:41-45`) |

## U1 — E1, E3, E5

**E1 (`fn menu`).** After `write_states`:

```rust
// E1: the base's `window_stroke.width` is `defaults.border.line_width`, sanitised; egui lays
// the stroke inside the padding (egui-0.37.0/src/theme/default_style.rs:68-71): nothing moves
if let Some(c) = m.hover_border_color {
    let line = egui::Stroke::new(own.visuals.window_stroke.width, to_color32(c));
    let w = &mut s.visuals.widgets;
    (w.hovered.bg_stroke, w.open.bg_stroke) = (line, line); // open: a submenu shown, S4
    w.active.bg_stroke = line; // `active` stays the hover look (S6)
}
```

`inactive` keeps `menu.border`'s stroke; unset, nothing changes. The radius (kde-breeze 4.5, the
presets part in U1) is drawn 5. Mapping: new `["menu.hover_border_color"]`, `scoped`, sinks
`visuals.widgets.{hovered,open,active}.bg_stroke.color`, `scope = "menu"`; `["menu.border.color"]`'s and
`["menu.border.line_width"]`'s hovered/active/open sinks gain `when = "menu.hover_border_color is
None"`; `["defaults.border.line_width"]` gains those three widths in `scope = "menu"`, `when =
"menu.hover_border_color is stated"`.
`contract.rs` `expected`: a `(Some("menu"), "normal", None, p)` arm for those colours and widths
(the chain hover border → `menu.border`); T10 names any further arm. Test
`menu_cell_applies_menu_style_then_its_rows` (`style/roles.rs`): kde-breeze light `hovered` and
`open` `Stroke::new(1.0, #399acc)`, corner radius 5 in the four entries, `inactive` unchanged;
an adwaita light case keeps today's four-entry loop.

**E3 — mechanism** (S9): the menu cell plus the bar's writes, beside `role_modifier`.
- `style/roles.rs`, after `fn menu`: `pub(crate) fn menu_bar_titles(t: &ResolvedTheme, s: &mut
  egui::Style)`. `hovered`, `active`, `open` take `inactive`'s `bg_stroke` (Breeze draws no line
  on a title; E1's would show). `hovered.weak_bg_fill` and `active`'s (also the keyboard-focused
  state, which stays on a closed title, S6) = `bar_hover_background`; `open`'s =
  `bar_open_background`, else `bar_hover_background`;
  `bar_hover_text_color` into those three `fg_stroke.color`; `button_padding` = (`bar_padding_
  horizontal`, `bar_padding_vertical`), each finite side through `clamp_length` (a title has no
  line). Each where stated; unset keeps the cell's, today's look.
- `atlas.rs`, after `role_modifier`: `ThemeAtlas::menu_bar_modifier(&self, theme) ->
  StyleModifier`: `role_modifier(theme, Role::Menu, RoleVariant::Normal)`'s writes, then
  `menu_bar_titles(self.resolved_for(theme), style)`; doc: for `MenuBar::style`
  (`EG/containers/menu.rs:345`), `role_modifier` stays `MenuConfig::style`'s.
- `ext.rs`: `pub fn menu_bar_modifier(ui: &egui::Ui) -> Option<StyleModifier>`, in
  `role_modifier`'s form and doc; `lib.rs` re-exports it beside `role_modifier`, and the module
  doc's *Where the theme reaches* names it.
- Limit (README *Limits*, the doc): `MenuButton` draws a title from its pointer state
  (`EG/containers/menu.rs:430`); only `SubMenuButton` swaps `inactive` for `open` while its menu
  shows (`:543-549`). The open fill reaches a title where the application swaps the same way.

**E3 — showcase.** `demo::menu_bar` takes `menu_bar_modifier(ui)` for the bar (was
`role_modifier`), keeping `item_spacing.x = 0.0` and `interact_size.y = 0.0` (titles touch; a
title is 18 + 2 × 6 = 30, as measured). `chrome::menu_bar`: while a title's menu is open,
`inactive`, `hovered` and `active` take `widgets.open` (was: `inactive` takes `hovered`; S9).
`chrome::menu_panel`: fill `t.toolbar.background_color` (was `window.background_color`), doc
likewise; elsewhere than kde-breeze both resolve to `defaults.background_color` (rationale E3).
The Overlays page's `MenuBar` keeps no fill of its own and takes the modifier through
`demo::menu_bar`.

**E3 — mapping, tests.** Five rows, `menu.bar_hover_background`, `bar_open_background`,
`bar_hover_text_color`, `bar_padding_horizontal`, `bar_padding_vertical`: `derived`, `tested_by =
"T18(a)"` (per call, no cell), `note` naming `menu_bar_modifier`; no `probe` (kde-breeze's
`PAIRS` differentiate them). `install_tests.rs`, new
`t18_a_the_menu_bar_modifier_is_the_menu_cell_with_the_bar_fields`, first doc line "T18 (a):
`menu_bar_modifier` is the `Role::Menu` cell with the menu bar's title fields." (the prefix
`SE/info.rs`'s `checked_by` reads): kde-breeze light with `bar_open_background` set to #f0a014
on the resolved theme (its two fills are both #3daee9): `hovered`/`active` `weak_bg_fill` #3daee9,
`open` #f0a014, text #ffffff in the three, `button_padding` (10, 6), `hovered.bg_stroke == inactive.bg_stroke`;
adwaita light equals `role_modifier(Role::Menu)`'s style.

**E5 (`SE/pages/basic.rs`, `list_row`).** In `if shown.is_positive()`, after the fill, where the
row is hovered, not selected and `l.hover_border_color` is stated: `ui.painter().rect_stroke(shown,
<the same inner_corners>, to_stroke(egui::Stroke::NONE, edge, t.defaults.border.line_width),
egui::StrokeKind::Inside)` (`EG/painter.rs:406-414`, `EP/stroke.rs:102-111`): on the fill's
shape, inside it, as Breeze's line. `list`'s doc names it. The command palette's rows
(`SE/chrome.rs`, the "palette rows" `Role::List` scope, a `selectable_label` each), which the
list cell's `hovered` fills today: after `row.add(..)`, where `r.hovered()`, the row is not
`highlighted` and `list.hover_border_color` is stated, the same stroke on the row's frame (`r.rect`
grown by the scope's `widgets.hovered.expansion`, its `corner_radius`;
`EG/theme/default_style.rs:62-72`), `StrokeKind::Inside` (S5). The Basic `table` has no row hover
(nor iced's `table_row_style`): nothing added there (S5, correction 5); the Data page's
`TableBuilder` hover is egui_extras' per-cell fill (`EX/layout.rs:145-151`), no stroke: README
limit. Mapping `["list.hover_border_color"]`, `derived`, `T18(a)`, as `["list.hover_text_color"]`.
`SE/info.rs` `PER_CALL_ROUTES` gains, for S12's list, `("basic.list.row_",
"defaults.border.line_width_px", "egui: per call, the hovered row's outline width
(pages::basic::list_row)")` and the same for `basic.list.frame`; for `basic.table.frame` and
`basic.table.row_`, `list.hover_border_color` and `defaults.border.line_width_px` "not reachable:
the Basic table's rows take no hover (pages::basic::table)"; and `("chrome.menu_bar",
"toolbar.background_color", "egui: per call, the menu bar panel's fill (chrome::menu_panel)")`
(those leaves' rows name other places). Test `SE/tests.rs`,
`a_hovered_list_row_is_outlined_where_the_theme_states_it`, in the shapes form of
`the_basic_list_and_table_fills_keep_inside_the_frames_corners` (`harness.output().shapes`):
pointer on `basic.list.row_1`; kde-breeze light one `Shape::Rect` stroked `(1.0, #3daee9cc)` at
the row's place with the fill's corners; adwaita none; the same on an unhighlighted palette row.

## U2 — E2, E4

**E2 (`fn menu`): not drawn in egui** (decision S6 of the specification's frame). egui draws a
focused widget in `active` (`EG/widget_style/mod.rs:212-222`, `EG/style.rs:1386`) and moves the
focus along a menu's items with the arrow keys (`MenuState::handle_keyboard`,
`EG/containers/menu.rs:233-288`): written into `active`, the pressed look would stay on every
keyboard-selected item, where Breeze draws the hover look, while a press lasts a moment. So `fn
menu` leaves `active` a copy of `hovered` (today's `FillSource`/`TextSource` `active: None`), and
E1's block writes the outline into `active` always.
Mapping: `["menu.active_background"]` and `["menu.active_text_color"]` are `verdict =
"unmappable"`, `sub_tag = "egui-limited"`, `upstream = "egui: a pressed state apart from the
focused one (Widgets::style gives a focused widget active)"` (the header's rule: `sub_tag` and
`upstream` on an unmappable row); README *Limits* and `fn menu`'s comment give the keyboard
reason. The cell test asserts kde-breeze light `active == hovered`, outline included.

**E4 — the panel gap; the rationale's open point settled** (S10): egui's menu popup takes its
margin from the menu role's style. A `MenuButton`'s and a submenu's popups are styled by
`MenuConfig::style` (`EG/containers/menu.rs:457`, `:714`); `Popup::show` applies it, then builds
`Frame::popup(ui.style())` (`EG/containers/popup.rs:650-651`), whose `inner_margin` is
`spacing.menu_margin` (`EG/containers/frame.rs:214-220`). A popup given no modifier starts from
the `Context`'s style (`EG/ui.rs:123`), so tooltips and lists opened inside a menu do not take
it, and the `Dialog`/`Popover`/`Tooltip` surfaces read the base style's (`mapping_tests.rs`
`preset_reads`). So `fn menu` writes `s.spacing.menu_margin = margin([v, h, v, h],
own.spacing.menu_margin, &ResolvedPadding { top: v, right: h, bottom: v, left: h }, notes)` with
`base::margin`, `h`/`v` the two `panel_padding` fields and their paths (`push_note` keeps one
note per leaf); kde-breeze `Margin::same(3)`, the items 3 + 1 (the line) from the panel's edge.
`SE/demo.rs` `menu_modifier` writes `popover.border.padding` only on the sides the menu leaves
unstated; its `item_spacing.y = 0.0` already makes rows touch. `CE/src/roles.rs`, the doc above
`Surface`: its 0.36.2 submenu lines become 0.37.0's (`menu.rs:707-720`; `Frame::menu` at `:598`
only measures). Mapping: `["menu.panel_padding_horizontal"]` → `spacing.menu_margin.left`/`.right`,
`["menu.panel_padding_vertical"]` → `.top`/`.bottom`, `scoped`, `scope = "menu"`; the four
`[unwritten]` `spacing.menu_margin.*` entries go (T10b rejects a path both). `contract.rs`: a
`spacing.menu_margin.<side>` arm in `scope = "menu"`. The cell test
(`menu_cell_applies_menu_style_then_its_rows`): its last assert becomes `Margin::same(3)` on
kde-breeze, the base's on adwaita; its `button_padding` assert takes `button_padding.x` as the
stated left side where stated, else the pair mean (E4's padding, below); its `interact_size.y`
assert, where `m.row_height` is `None`, takes the least row height where stated, else egui's
(E4's row height, below).

**E4 — padding.** `button_padding.x` is `menu.border.padding_left` for both sides where stated,
else today's pair mean (rationale E4); notes stay `button_padding`'s over `&m.border`:

```rust
let mut items = m.border.clone();
items.padding.right = items.padding.left.or(items.padding.right);
s.spacing.button_padding.x = to_button_padding(after_menu_style, &items).x;
```

Only kde-breeze states uneven menu sides (adwaita and macos-monterey 12/12, material-3 16/16,
windows-11 11/11): nothing moves elsewhere. kde-breeze (14, 4). The shortcut, flush right in the
padding (`Button::shortcut_text`), ends 14 from the edge, Breeze's 27; `SubMenuButton`'s arrow
lies there too: README *Limits*. `["menu.border.padding.right"]` gains `when =
"menu.border.padding.left is None"`; `contract.rs` gains a `(Some("menu"), "normal", None,
"spacing.button_padding.x")` arm, the same rule, before the generic `(_, "normal", None,
"spacing.button_padding.x" | …)` one. RTL: egui lays a button out
left to right in every layout and the showcase has no right-to-left mode; one inset for both
sides has nothing to mirror. **Gap, radius**: no code (`icon_spacing` ← `menu.icon_text_gap`, 4;
the radius E1's).

**E4 — row height.** `fn menu`'s `row_height(..)` call becomes:

```rust
match (m.row_height, m.min_row_height) {
    // a least height: egui's Button grows past it (egui-0.37.0/src/theme/default_style.rs:97-101,
    // src/widgets/button.rs:351)
    (None, Some(least)) => s.spacing.interact_size.y = length("menu.min_row_height", least,
        input.scheme.default_style().spacing.interact_size.y, notes),
    _ => row_height(s, "menu.row_height", m.row_height, input, notes), // row_height wins
}
```

`fit_row_padding(s, m.row_height, ..)` is unchanged (a stated `row_height` only); its and
`row_height`'s docs name the least height. `SE/demo.rs` `grow_rows` is unchanged; the bar's
`interact_size.y = 0.0` keeps 29 off the titles. Mapping `["menu.min_row_height"]`, `scoped`,
`spacing.interact_size.y`, `scope = "menu"`, `when = "menu.row_height is None"`; `contract.rs`'s
`(Some(w @ ("menu" | "list")), .., "spacing.interact_size.y")` arm takes row → least → egui's.
Test `the_menu_row_is_at_least_min_row_height_unless_a_row_height_is_stated` (`style/roles.rs`):
kde-breeze light 29 with `button_padding.y` 4; with `row_height = Some(24.0)` too, 24 and the
fitted padding; with neither, egui's own.

## U3 — E7, E8, E9, E10 (E6 reaches no egui code)

**E7 (`SE/pages/basic.rs`, `table`).** Where `l.header_separator_color` is stated, the line under
the header and the column line over `header.top()..=header.bottom() - grid_width` take
`Stroke::new(grid_width, to_color32(c))`; the column line over `header.bottom()..=rect.bottom()`
keeps the grid stroke (today one `vline` over `rect.y_range()`). Split so that no pixel takes the
translucent colour twice; width unchanged (`separator.line_width`). Checked: the two sections
span the header (`mid`, `header.right()`), so no empty area follows the last (rationale E7). The
Data page's `TableBuilder` draws one column line, header and body, in `noninteractive.bg_stroke`
(`EX/table.rs:891-900`): README limit. Mapping `["list.header_separator_color"]`, `derived`,
`T18(a)`, `note` with that line. Test `the_table_headers_lines_take_the_header_separator_colour`
(shapes form): kde-breeze light the header's bottom line `to_color32(#23262933)`, the rows'
column line `grid_color`; adwaita both `grid_color`.

**E8 (`CW/src/slider.rs`).** `Geometry` gains `outline: Option<f32>` from
`slider.thumb_border_line_width` through the Switch's `border` closure (`CW/src/switch.rs`,
`Geometry::of`); `Look` gains the three colours. In `slider_ui`:

```rust
// E8: the knob's own line where the width is stated (0: none); else egui's, as today (S7)
let on_knob = response.dragged()
    || response.hover_pos().is_some_and(|p| p.distance(centre) <= knob_radius);
let mut outline = match g.outline {
    None => ui.style().interact(&response).fg_stroke,
    Some(width) => {
        let rest = sl.thumb_border_color; // a disabled knob too (S7)
        let colour = if enabled && on_knob { sl.thumb_hover_border_color.or(rest) } // hover wins
            else if enabled && response.has_focus() { sl.thumb_focus_border_color.or(rest) }
            else { rest };
        colour.filter(|_| width > 0.0)
            .map_or(egui::Stroke::NONE, |c| egui::Stroke::new(width, to_color32(c)))
    }
};
```

Under a fade a stated outline is faded from its own colour; `None` keeps `Fade::outline`. The
disc stays `knob_radius - outline.width`, so the knob is `thumb_diameter` with or without its
line. `thumb_hover_color`'s fill keeps `response.hovered()`. The type doc's knob bullet says
all this, and that egui focuses by keyboard only (S7). `CW/src/tests.rs`: `WIDGET_READS`'
`"Slider"` row gains the three colours; new `the_slider_outlines_its_knob_in_the_themes_colours`
(the `CircleShape`'s stroke): kde-breeze light with `thumb_focus_border_color` set to #f0a014 on
the resolved theme (its hover and focus colours are both #3daee9): rest `(1.0, #d1d1d2)`, pointer
on the knob and dragging `(1.0, #3daee9)`, pointer on the rail off the knob `(1.0, #d1d1d2)`,
focused `(1.0, #f0a014)`, focused with the pointer on the knob `(1.0, #3daee9)` (hover wins,
S7); adwaita the state's `fg_stroke`. `the_slider_paints_the_themes_knob` is re-read (E8
removes kde-breeze's `thumb_hover_color`). `CW/README.md`'s `slider::Slider` row names the
outline. egui's own `Slider` is left as is (`EG/widgets/slider_core.rs:423-441` strokes the
handle in `fg_stroke`, which letters the value, `EG/style.rs:1434-1436`): README *Limits*.
Mapping: the four leaves as `["slider.disabled_thumb_color"]`: `unmappable`, `sub_tag =
"widgets-crate"`, `upstream = "native-theme-egui-widgets: Slider: the knob's outline; egui's
Slider strokes its handle in fg_stroke, which letters its value"`.

**E9.** `accessors.rs`, after `toolbar_button_checked_background`, in its form:
`toolbar_button_checked_border_color(t) -> Option<egui::Color32>`, doc: "the line of a checked
toolbar button at rest, `button.border.line_width` wide; under the pointer and pressed the
toolbar's hover outline (`toolbar.button_hover_border_color`) applies; `None`: the checked button
keeps its rest stroke". `lib.rs` re-exports it (alphabetical); README's accessor list names it.
Showcase (`icon_buttons`, the edit button's scope, after `demo::tool_button`): where `on` and
`Some(c)`, `widgets.inactive.bg_stroke = Stroke::new(t.button.border.line_width, c)`, and where
`demo::toolbar_edge(t)` is `None`, `hovered` and `active` too (S8); `Button::fill` leaves the
stroke to the scope (only `Button::stroke` overrides it, `EG/widgets/button.rs:161`). Mapping
`["toolbar.button_checked_border_color"]`, `derived`, `T18(a)`. Tests:
`the_atlas_reports_what_the_builder_was_given` (`install_tests.rs`) kde-breeze light #d1d1d2 and
`None` unset, beside the checked fill's lines; `the_basic_edit_button_starts_checked_and_switches`
(`SE/tests.rs`) the rest stroke.

**E10 (`fn combo_box`).** After the `hover_border_color` block (its comment "an open combo box is
egui's `open` state and keeps the border at rest" amended): where stated,
`open.weak_bg_fill = composite_over(cb.open_background, cb.background_color)` (`Layer::Composite`,
as the role's hover; opaque is exact) and `open.bg_stroke.color = cb.open_border_color` at the
width `write_states` put there. egui uses `open` under `CLASS_OPEN` whatever the pointer does,
as Breeze. Mapping `["combo_box.open_background"]` (`open.weak_bg_fill`),
`["combo_box.open_border_color"]` (`open.bg_stroke.color`), `scoped`, `scope = "combo_box"`;
`combo_box.background_color`'s and `combo_box.border.color`'s `open` sinks gain `when`;
`contract.rs`: `layered(open_background, background_color)`, `chain(open_border_color,
border.color)`. Test `combo_box_and_segmented_control_cells_write_their_rows`: kde-breeze light
`open` #c3e5f6 / #3daee9; adwaita `open == inactive`.

## U4 — E11, E12

No egui code: font sizes and `button.min_width` arrive as the resolved `f32`. The carried
`defaults.font_whole_pixels` (S3) takes a row in `["defaults.border.opacity"]`'s form
(`unmappable`, `source-void`, `upstream = "none: the model rounds every font size where it is
true"`); the source-void count 1 → 2 with its message. Two docs computing kde-breeze's 13.33
px expander title become 13 px: `the_header_is_its_stated_height_by_the_row_rule`
(`CW/src/tests.rs`) and `the_expander_header_is_its_stated_height_by_the_row_rule` (`SE/tests.rs`):
at 1.5, 19.5 × 1.36 = 26.52 → 27, + 7 + 7 = 41 (was 27.2, 28, 42); asserts re-read in the build.
`SE/tests.rs` `the_palette_pages_apply_the_per_call_leaves`: its comment "kde-breeze's 80 px is
wider" (the buttons' minimum width) becomes 82.

## All units

- `mapping_tests.rs`: leaves 534 → 555 (21; the message "551 ResolvedTheme leaves + 4 layout
  leaves"); verdict deltas `[direct, scoped, derived, unmappable]`: foundation `+[0,0,0,1]` (the
  flag), surfaces `+[0,4,5,2]`, inputs `+[0,2,2,0]`, indicators `+[0,0,0,4]`, chrome
  `+[0,0,1,0]`; sub-tags `widgets-crate` 22 → 26, `egui-limited` 53 → 55, `source-void` 1 → 2
  (the test's doc counts likewise).
- Parity exceptions (input to the orchestrator, S12): in *egui: frames, focus rings, popups*,
  after `role_modifier`: `connector = ["iced", "gpui"]`, `name = "menu_bar_modifier"`, reason in
  `role_modifier`'s form (`MenuBar::style` takes a `StyleModifier`; iced styles its bar per
  widget, `styles::aw::menu`; gpui-component's `AppMenuBar` has no receiver, E14). In *absent:
  gpui* and *absent: iced*, each after its `toolbar_button_checked_background`:
  `toolbar_button_checked_border_color`, the reason in that entry's form, naming what draws the
  outline (`geometry::toolbar_button_checked`; `styles::toolbar_button`).
- `docs/showcase-elements.toml`: S12's list. egui's routes come from the mapping rows, except where
  `PER_CALL_ROUTES` names them (E5).
- `CE/README.md` *Limits*: E2's keyboard item, E3's open title, E4's right inset and shortcut,
  E5's and E7's egui_extras table, E8's egui `Slider`. Nothing goes upstream for egui (E14). Per
  unit: `cargo test -p native-theme-egui`, `-p native-theme-egui-widgets`, the showcase's tests;
  `mapping.md` regenerated and committed with the rows.

