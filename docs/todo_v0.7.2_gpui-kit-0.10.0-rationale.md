# GPUI Kit 0.10.0 in v0.7.2: rationale

Status: written 2026-11-16; reviewed, checked with the egui document, and re-read by its author
that day and the next. Approved by the maintainer on 2026-11-17, with the specification,
`docs/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-spec.md`, and the plan,
`docs/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan.md`; being built.

Names used below. `GC` and `GB` are the `src/` folders of gpui-component and gpui-base in Cargo's
registry, at 0.10.0 unless a line says 0.9.1; upstream lines carry their version, so they do not
drift. `CN` is `connectors/native-theme-gpui`, `SC` is `CN/examples/showcase-gpui`. Repository
files are cited by path and an item or a quoted phrase, never by line. `REC` is the record folder
of this document's measurements,
`~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/gpui-kit-0.10.0-trial/`.

## 0. What this document is for

GPUI Kit 0.10.0 was released on 2026-11-16. The maintainer put it into v0.7.2 the same day, beside
the egui 0.37 move (`docs/todo_v0.7.2_egui-0.37-rationale.md`), and said what the word means:
"GPUI Kit 0.10.0 support is not only being able to use it without errors. It means, any new
widgets, graphical elements are themed with our crate, and the showcase includes the new
elements." v0.7.1 stays on GPUI Kit 0.9.1.

This document lists what 0.10.0 adds and changes in what it draws, what a build and a test run on
it showed, and decides what v0.7.2 does about each item. It follows the form and the standing
rules of the 0.9.1 document (`docs/todo_v0.7.1_gpui-kit-0.9.1-rationale.md`, N1 to N6): every new
element a release brings is shown; a statement the release makes false is corrected in the release
that moves to it; the backlog's upstream entries, the lint list and the gate-read citations are
re-read at each upgrade.

The release's lock is the two moves together. The egui document decides the egui move only; this
one decides the GPUI Kit move only. The version bump, the asset run and the close-out are shared
and belong to the release's plan.

## 1. What was read and measured

### 1.1 Sources

- Upstream's release notes for v0.10.0 (`github.com/longbridge/gpui-kit/releases/tag/v0.10.0`,
  published 2026-11-16, read that day; kept at `REC/release-notes-v0.10.0.md`).
- The published sources of gpui-component, gpui-base and gpui-kit at 0.9.1 and 0.10.0, compared
  file by file in two read-only passes on 2026-11-16: what is new or changed in what is drawn
  (`REC/inventory-A.md`), and what the changes do to the repository's statements, backlog, lint
  list and pins (`REC/inventory-B.md`). Both carry the upstream line of every statement; the
  tables below give the lines a decision rests on.
- A measuring build on a throwaway branch (`REC/trial/report.md`, with its logs, lock moves and
  patches): the gpui connector moved to 0.10.0 on top of the egui 0.37 trial's lock, checked in
  the gate's three feature forms, linted, and its tests run once. Nothing of it merges.
- The passes and the trial read the repository at `e866ea6c`, before v0.7.1's own plan for 0.9.1
  is built (its new showcase parts, corrected statements and re-pointed citations). The
  specification takes the counts below again on the tree v0.7.2 starts from. Nothing 0.10.0
  changes touches what that plan adds (`GridLayout`, the Diff's decorations), by the review of
  this document.
- gpui-kit 0.10.0 itself changes a doc example and its tests only; everything below is
  gpui-component's or gpui-base's. gpui-pre stays at 0.3.12; GPUI Fast goes from 0.1.15 to 0.1.16.

### 1.2 What the trial measured

- **Lock.** One `cargo update -p gpui-component --precise 0.10.0` moved 14 packages and added
  none: the five GPUI Kit crates to 0.10.0, gpui-fast and its nine sub-crates to 0.1.16. All three
  0.10.0 crates pin `gpui-pre = "=0.3.12"`, so the 0.3.13 that crates.io also has cannot resolve.
  gpui-base gains a dependency on sys-locale (already in the lock). One move nobody asked for:
  swash 0.2.10 now takes skrifa 0.37.0 where it took 0.44.0 (it accepts both; skrifa 0.44.0 stays
  for the egui crates).
- **The two moves together.** The egui trial's lock (rust-i18n-support 4.2.4, serde-saphyr 1.3.0,
  smallvec 1.16.5) holds with GPUI Kit 0.10.0, and with 0.9.1. serde-saphyr 1.3.0 declares Rust
  1.89; by the lock it is in the gpui connector's graph only, and that crate declares 1.95.0, so
  no floor moves. swash is reached only from the gpui and iced connectors, so the gate's check
  that the egui connector's graph holds one skrifa is untouched.
- **Compile errors: one site**, under the `gpui-fast` feature only (T7). The forms on gpui-pre
  compiled unchanged.
- **Lints.** Both clippy forms and the strict-panic check pass with no finding.
- **Tests.** 9 library tests and 1 showcase test fail (the library's other 390 pass in the gate's
  GPUI Fast form and 285 with no features; the showcase's other 331 pass). By cause:
  - moved citations: 363 of the 826 colour claims and 23 of the 1,020 prose citations are no
    longer read at the upstream line they cite (two tests);
  - the new colour (§1.3 row 1): five tests that hold the connector to upstream's field list;
  - version text: the README's Compatibility table and Quick start (two tests), and the
    showcase's fallback-stack test, whose citation names gpui-fast 0.1.15.
  No test failed for a change in drawing or behaviour, and none for a cause not determined.
- **Coverage gate.** For gpui it finds 159 widgets at 0.10.0 as at 0.9.1, 124 shown and 35
  excepted, none missing. 0.10.0 adds no public widget struct: what it adds are options and drawn
  parts of existing widgets, which that gate cannot see. §1.3 is the list it cannot give.
- **Time.** About 12 minutes of builds and tests on a warm build folder.

### 1.3 What 0.10.0 adds that draws

From the release notes, each checked in the sources, and from the diff for what the notes do not
name.

| # | Addition | What it draws, and from which theme values | In v0.7.1 today |
|---|---|---|---|
| 1 | The colour `ThemeColor::list_highlight_foreground`, config key `list.highlight.foreground` (`GC/theme/theme_color.rs:196-197`, `GC/theme/schema.rs:472-474`) | the matched part of a row's text under `highlight_matches` (row 2), and nothing else (`GC/label.rs:264-275`). A theme that does not set it gets `blue` (`schema.rs:991`); upstream's own palettes set blue-600 and blue-400 | not set. On the connector's direct path (`to_theme`) it stays at `ThemeColor::default()`, transparent black. After a `Theme::change` it is upstream's own blue, not the theme's: `apply_config` starts from upstream's palette (`GC/theme/schema.rs:1097-1101`), the connector's config cannot carry `base.blue`, and the fallback is taken (`:991`) before the connector restores its base colours |
| 2 | `highlight_matches(bool)` on `Select`, `Combobox` and `Command` (`GC/select.rs:764`, `GC/combobox.rs:848`, `GC/command/command.rs:135`), off by default | the matched substrings of a row's title in the colour of row 1 | the three widgets are shown without it |
| 3 | `Checkbox::indeterminate` (`GC/checkbox.rs:109`) | the box as when checked (border `primary`, fill `tokens.primary`), with `IconName::Minus` for the mark, through the icon resolver, in `primary_foreground` (`:230-250`, `:284-290`, `:354-370`) | the check box is shown checked and unchecked only. The connector's own `widgets::Checkbox` has no mixed state |
| 4 | Keyboard focus on tabs (`GB/tabs.rs:160-173`, `:352`, `:385`): a tab list is one tab stop, the arrows, Home and End move the focus | gpui-component's `Tab` draws a focus ring on keyboard focus: `ring`, an inset shadow with a 2px spread, a literal (`GC/tab/tab.rs:933`, `:962-973`). The connector's own `widgets::TabBar` is built on the same gpui-base `Tabs` and `Tab`, so its tabs take the focus too, and it draws no focus state | tabs are shown; no Widget Info states a focus ring |
| 5 | The Slider is a tab stop (`GC/slider.rs:147-153`, `:299-311`) and answers the arrows, Page Up and Page Down (ten steps), Home, End and AccessKit's numeric `SetValue` (`GB/slider.rs:430-463`, `:501-525`, `:726-735`) | the focused end thumb shows the thumb's ring in `ring` (`GC/slider.rs:63`, `:228`, `:233-238`) | shown; the Widget Info states no ring |
| 6 | A loading `Button` without an icon shows a spinner (`GC/button/button.rs:777-786`) | the `Spinner` a loading button with an icon shows, with the Spinner's own `IconName::Loader` where no `loading_icon` is set | the Widget Info says "only a button with one shows it" |
| 7 | `Clipboard::label` (`GC/clipboard.rs:52`) | a text label beside the copy icon, in the ghost button's colours | shown without a label |
| 8 | The tooltip changes its width and height when the pointer goes from one tooltip to the next (`GC/tooltip.rs:219-243`) | 200ms, a literal (`:164`); no theme value | tooltips are shown; the Widget Info states delays only |
| 9 | Right-to-left layout: `LayoutDirection`, a global that `gpui_base::init` sets from the system's language unless the application set one, and `LogicalStyledExt` (`GB/layout_direction.rs:24-104`, `:121-234`) | no theme value. In a right-to-left layout upstream's widgets mirror: rows, paddings, chevrons, popups' anchors, docks and sheets, and the direction the slider, progress bar, switch and tab strip run in. Left to right they draw what 0.9.1 drew (checked line by line for 13 widgets) | not used. The connector's own widgets and the showcase's layout name physical edges |
| 10 | Options that draw only what the caller supplies or remove a part: `SidebarGroup::suffix`, `TableRow::hover`, `TextareaState` decorations, `TextView::link`, `NumberInput::stepper(false)`, `Diff::split_ratio`, `DataTable::scrollbar_mode`, the dock's hidden collapsed tab bar | no theme value of their own | not used |
| 11 | gpui-base only, with no route through gpui-component's widgets: the tooltip request's offset, `SelectableText::highlights`, raw HTML kept as text in Markdown | caller's values | not used |
| 12 | A Japanese locale for upstream's own labels | no theme value | not used |

Eleven more additions draw nothing (getters, events, identities, batch calls); they are listed in
`REC/inventory-A.md`, "Additions that draw nothing". The reduced-motion preference is one of them
and is T8's subject.

The release notes name `EditorState::text_gutter_margin()` as replaced. No such item is in the
published sources of either version, so there is nothing to follow.

### 1.4 Statements the changes make false or incomplete

| # | The statement | What 0.10.0 does | |
|---|---|---|---|
| S1 | `SC/info/feedback.rs`, `TAG_HOVER_OPACITY` and the Tag's hover note; `SC/tests.rs`, `a_tags_painted_fill_is_its_bg_claim` and `a_hovered_tags_info_names_its_painted_fill`: a hovered Tag paints at 90% | the hover is gone (0.9.1 `GC/tag.rs:269`; 0.10.0 `GC/tag.rs:265-269`) | false |
| S2 | `SC/info/buttons.rs`, the Loading arms, and `SC/pages/buttons.rs`'s comment: only a button with an icon shows the spinner, so the demo gives it one | §1.3 row 6 | false; the demo icon's reason is gone |
| S3 | `CN/src/geometry.rs`, `tooltip` and `tooltip_content`; `SC/demo.rs`, `action_tooltip`; `SC/info/feedback.rs`, the tooltip's "why built" note: the bubble cannot wrap its text, so the width goes on a content element | the content box is now `min_w_0().overflow_hidden()` (`GC/tooltip.rs:134-144`), and upstream's own test wraps `Tooltip::new(text).max_w(…)` (`:1017-1028`). Not run here | false; the conversion's reason is gone, by upstream's test |
| S4 | `CN/src/widgets/slider.rs`, `Slider`'s doc: "Unlike gpui-component's, it takes keyboard focus"; `CN/README.md`, *Assistive technology*: the theme-drawn `Slider` takes an `accessibility_label` "as gpui-component's `Slider` does" | §1.3 row 5: upstream's takes the focus too, and answers Page Up, Page Down and `SetValue`, which the connector's does not; upstream emits `Release` after a key's `Change` | the doc is false; the README's sentence stays true and says nothing of what upstream's now does more |
| S5 | `CN/README.md`, *Keyboard focus*; `CN/src/widgets/mod.rs`, `FocusRing`'s doc; `SC/info/leaves.rs`: gpui-component's own widgets draw a 3 px ring at half the colour's alpha | true of `focus_ring_style`, unchanged; the `Tab` now draws another, 2 px inset at the full colour, on keyboard focus only | incomplete |
| S6 | `CN/src/widgets/tab_bar.rs`, `TabBar`'s doc: built on gpui-base's `Tabs` and `Tab`, "activation by click" | §1.3 row 4: it now takes keyboard focus and shows none | incomplete |
| S7 | `CN/src/lib.rs`, `to_theme` ("Scrolling … when the platform draws overlay scrollbars"); `CN/README.md`: `init` "also sets `scrollbar_mode` from the platform" | `Theme::sync_scrollbar_appearance` now maps Windows by its own rule: auto-hide to `Hover`, otherwise `Always` (`GC/theme/mod.rs:145-154`, `:386-392`; 0.9.1: auto-hide to `Scrolling`, otherwise `Hover`, on every platform) | incomplete |
| S8 | `CN/src/lib.rs`, `apply`'s and `forward_reduce_motion`'s docs: how an application keeps its own reduce-motion flag | a new `ReduceMotionPreference` and `set_reduce_motion_preference` (`GB/reduce_motion.rs:85-127`) protect an explicit choice from gpui-base's readings; the connector's request still switches the flag on over an explicit `Allow` (read, not run) | incomplete |
| S9 | `CN/src/lib.rs`, the crate doc: "all 139 colour fields" | 140 | false |
| S10 | `CN/src/widgets/mod.rs`, `A11yDisabled`: gpui-base's wrapper "of this shape" | gpui-base's copy now gates its `IntoElement` on the core (`GB/a11y_disabled.rs:63-70`) | incomplete; T7 |
| S11 | `CN/README.md`, *Backends*: "GPUI Fast 0.1.14 or later (gpui-kit 0.9.1's `gpui-fast = "0.1.14"`)"; the Compatibility table; the Quick start | 0.10.0 asks for GPUI Fast 0.1.16 | false at 0.10.0 |
| S12 | The theme-drawn `Checkbox`, `Radio` and `Switch`: "disabled: inert, unfocusable" | a control that is focused when it becomes disabled keeps the focus so that Tab can leave it (gpui-base's checkbox, radio, switch, toggle) | incomplete, marginally |

Two statements stay true with moved lines (the stacked dialog's offset, the tooltip's fade). For
the other changes that were searched, no statement was found in the repository;
`REC/inventory-B.md` lists each with its search terms.

### 1.5 The backlog, the lint list and the pins

- **Backlog.** Of `docs/todo.md`'s open upstream entries about GPUI Kit's crates (57 by the
  pass's count, not reproduced), 0.10.0 answers none. One changes in part: issue #4's fourth
  request (a focus ring on keyboard focus only), since the `Tab` now does that while `focus_ring_style` does not. The entry on the icons
  gpui-component builds for itself gains three: the Spinner's `Loader` on a loading button, the
  mixed check box's `Minus`, and `ChevronLeft` in a right-to-left layout. The entries about GPUI
  Fast (taffy, unicode-properties, the paint-dump defects) were not re-read: 0.1.16 was not on disk
  for the passes.
- **Lint list** (`CN/clippy.toml`). No entry is gone; clippy resolves all of them at 0.10.0. The
  two `RopeExt` reason lines moved by one, and every gpui-fast reason line cites 0.1.15. One new
  hazard is not a candidate for the list: a `Label` is now an accessibility node whose id defaults
  to its call site, so sibling labels built by one `Label::new` in a loop share an id, and GPUI's
  debug builds panic on the duplicate (`GC/label.rs:75-96`; gpui-pre 0.3.12 `window/a11y.rs:414-418`).
  `CN/src` and `CN/examples` have 36 lines with `Label::new`; which build siblings in a loop was
  not determined. The panic needs a connected accessibility tree, so the passing tests say nothing
  about it.
- **Pins.** The three floors become 0.10.0. The exact gpui-pre pin, the features' wiring, the
  docs.rs feature list and the `cold_path` reason of the 1.95.0 floor hold; the gpui-fast halves of
  the MSRV and `image` comments name 0.1.14 and 0.1.15 and were not re-read at 0.1.16.

### 1.6 Cited upstream lines

The showcase's claim gates read each cited upstream line at the version the lock holds. At 0.10.0
363 colour claims and 23 prose citations fail (§1.2), against 209 at the move to 0.9.1. Of the 416
distinct gpui-component and gpui-base citations in `SC/info/`, 336 point into files the release
changed. GPUI Kit has released once a week since 0.7.0 (eight releases from 2026-09-28 to
2026-11-16, by upstream's release list).

## 2. Decisions

### T1 — What belongs to v0.7.2

- (a) The build only: the floors, the lock, the one compile error, the version texts.
- (b) Everything of §1.3 to §1.6, whatever it needs.
- (c) By kind, as N1 of the 0.9.1 document: what the upgrade makes false, what it adds that draws,
  and what the standing upgrade rules name go into the release; what needs a new model field with
  platform facts behind it, or a new public API with open design questions, is a step of its own,
  named in §4 so that the maintainer sees every place where the release stops short.

**(c).** (a) is what the maintainer's sentence of 2026-11-16 rules out. (b) would put things into
a patch release that have no facts yet, such as a desktop's own emphasis for a search match (T2);
a value without a source is not written (the project's rule), and gathering the facts is a task
of its own size. Under (c) every new drawn part of 0.10.0 is themed from theme data and shown.
§4 names the places where that falls short of the full sentence: the match highlight, which no
one colour can theme on every preset (T2), and the showcase's layout direction (T6).

### T2 — The new colour

**What was measured** (2026-11-16; `REC/t2-contrast.tsv`, the program beside it). The rows that
draw the highlight are the Select's and Combobox's (`GC/searchable_list/item.rs`) and the Command
palette's (`GC/command/state.rs`). At rest a row is the popup's `popover` under `foreground` text;
hovered it is `accent` at 0.7 under `accent_foreground` (`item.rs:106-118`); selected, or under the
keyboard cursor, `accent` under `accent_foreground` (`item.rs:120-123`; `command/state.rs:747-751`).
The connector fills `accent` from the menu's hover background and `accent_foreground` from its
hover text. On kde-breeze, adwaita, windows-11 and material-3 the menu's hover is a faint tint and
the text keeps its colour when a row is selected. On the others (the macOS and iOS presets and the
ten community ones) the selected row is a full colour, on most of them under a text colour of its
own. The highlight is one colour for all three states.

For five theme colours the program computed the WCAG contrast of the matched text against the
three painted row backgrounds, on the 17 presets in both modes (34 cases):

| Candidate | Below 3.0 at rest / hovered / selected | Below 1.5 at rest / hovered / selected |
|---|---|---|
| `info` | 6 / 27 / 27 | 3 / 11 / 20 |
| the accent (`primary`: 5 / 27 / 27 and the same below 1.5) | 5 / 28 / 28 | 0 / 13 / 24 |
| `link` | 4 / 25 / 28 | 0 / 10 / 23 |
| `foreground`, the resting text | 0 / 13 / 16 | 0 / 0 / 7 |
| `accent_foreground`, the selected row's text | 23 / 9 / 3 | 21 / 0 / 0 |

With `info`, in 10 of the 20 cases the selected row's contrast is between 1.00 and 1.04: the
matched letters cannot be seen at all (ios-13, gruvbox, solarized and one-dark in both modes, nord
dark, dracula light). `info` stays at 3.0 or above in all three states in 7 cases (kde-breeze
dark, adwaita, windows-11, material-3). No candidate reaches the row's own text contrast in all
three states on every preset, and none can: where the selected row changes its text colour, a
colour that reads at rest does not read on the selected row. The program assumed that the popup
lies on the window background and that GPUI blends in sRGB, and it read the connector's mapping at
0.7.1.

**Options.**

- (a) Leave it unset. Refused: the highlighted text would be transparent on the direct path, and
  five tests fail.
- (b) `info`: what upstream's own fallback rule gives for the connector's palette (`blue`, which
  the connector sets from the theme's `info`). The match is marked at rest (3.0 or above in 28
  cases). On the hovered row and on the selected row, which is also the keyboard cursor's row
  while the user types, the matched letters are hard to read on most presets and invisible on the
  10 cases above.
- (b2) `foreground`, the resting text colour. At rest the matched part is drawn as the rest of
  the title, so nothing is marked. It never falls below 1.5 at rest or on the hovered row; on the
  selected row it does in 7 cases (catppuccin-mocha dark, nord dark, solarized and tokyo-night in
  both modes, one-dark dark), and it is below 3.0 there in 16, where the row's own text reads
  better than its matched part.
- (c) The accent, or (d) the link colour: as (b) with slightly different counts; a link colour
  also says "clickable" on text that is not.
- (e) An emphasis that keeps the row's own text colour (a weight or an underline), or a colour per
  row state. Upstream's `highlight_matches` offers neither; an application can draw its own through
  `SearchableListItem::render_highlighted`. Which emphasis a desktop uses is a platform fact
  nobody has gathered: macOS has `NSColor.findHighlightColor`, a background
  (`docs/platform-facts.md`, the system colours table), and the other desktops have no row.
- (f) A choice per theme. A rule needs a bar. WCAG's for text of this size is 4.5, which `info`
  reaches in all three states in 3 cases, and the project's contracts assert nothing against that
  bar, because the desktops' own pairs break it. The one test without a number, "`info` where the
  selected row keeps the resting text colour", picks `info` on ios-13 dark, where it is invisible
  on the selected row. And the candidate with the best worst case is the resting text almost
  everywhere, which is (b2).

**(b2), with its limit stated, and (e) as an upstream request and a backlog step.** No value of
this one field themes the element on every preset; the cause is upstream's one colour for three row states, and it is fixed there. Of the
single colours, the resting text is the one that never hides text at rest or under the pointer and
hides it least on the selected row: 7 cases below 1.5 and none invisible, against 20 and 10. A
theme connector must not make text unreadable on a row whose own text reads, and that weighs more
than marking the match. Its price is the feature: with a native theme, turning `highlight_matches`
on marks nothing at rest, and on a theme whose selected row has a text colour of its own it draws
the matched part of that row in the resting colour. (b) is refused: it keeps the feature as
upstream means it and pays with the letters the user has just typed, on the row the cursor is on,
invisible in 10 cases. When upstream's highlight keeps the row's text colour, the field takes
whatever that form needs and this decision is made again. `highlight_matches` is off unless an
application turns it on.

- The key is exported in the connector's config, so that the direct path and the path through
  `Theme::change` give the same colour (today the second gives upstream's own blue, §1.3).
- The Widget Info of the Select states the colour, its source and the limit, with the counts
  above; `CN/README.md` says the same to an application's author, and names `render_highlighted`
  as the way to draw a match that keeps the row's text colour.
- A draft for upstream: a highlight that keeps the row's text colour, or a colour for the selected
  row. Not sent without the maintainer's approval.
- The backlog: the platform facts for a search match's emphasis and a model field for it.

Changes: `CN/src/colors.rs` (the field, the tripwire's 140), `CN/src/config.rs` (the key),
`CN/src/contract.rs` (declared source: derived, the resting text colour), the Theme Map's token
row, the crate doc's mapping table and its "139".

### T3 — New drawn parts: shown, stated or recorded

The rule (N2 of the 0.9.1 document): an element or drawn part with a theme value is shown and its
Widget Info states every value and literal it draws with; an option that draws only what the
caller supplies, or nothing new, is recorded.

| Row of §1.3 | Decision | Reason |
|---|---|---|
| 2 `highlight_matches` | **shown** once: a searchable `Select` on its page with the option on, labelled as such; `Select` has no public query setter, so the match is drawn when the user types, and a test types a query and asserts the matched run's colour. Its Widget Info states the colour, its source and the limit (T2). The option is not turned on for the preset Combobox and the Command palette, which are the showcase's own chrome (`SC/chrome.rs`): there it would change how the showcase's own rows read on the themes T2 names. Their Widget Info names the option and says so | the only reader of the new colour; the three widgets draw the match through the same function (`GC/label.rs:264-275`) |
| 3 the mixed check box | **shown** twice, each with Widget Info lines for the fill, the mark and its colours: upstream's check box in its mixed state, and the connector's own `widgets::Checkbox`, which gains `indeterminate(bool)`. The theme-drawn box draws its check as upstream's `Check` icon, stroked at the theme's width where the theme states one (`CN/src/widgets/checkbox.rs`, `CheckboxLook`'s `mark_stroke`); it draws the mixed mark as upstream's `Minus` icon in the same way and colours, in the box as when checked, which is how upstream draws the state, on gpui-base's mixed state (in gpui-base since 0.9.1) | the theme-drawn check box stands in for upstream's and must not lack a state upstream's has; it uses no fact it does not already use for the check. Whether a desktop draws another mark for the mixed state is a facts task in the backlog. A new public method, named in §4 |
| 4 the tab's focus ring | **stated** on upstream's `Tab` (Widget Info: `ring`, 2px inset, a literal the theme does not reach), and **drawn** by the connector's `TabBar` (T4) | a new drawn part on a shown element |
| 5 the slider's focus ring | **stated** in the Slider's Widget Info | as row 4 |
| 6 the loading button | **shown** twice: with an icon and without, the Widget Info corrected (S2). The button without an icon gets `loading_icon` set to the chosen icon theme's loading icon, as the first does; left unset, the Spinner draws its own `IconName::Loader` (`GC/spinner.rs:25`) through the icon resolver (`GC/icon.rs:208`), which without the icon hook of a later release is upstream's bundled icon beside a native set. Where the chosen theme has no loading icon the demo does what the button with an icon does today; the unit reads that and the Widget Info names the icon drawn | the 0.9.1 demo could not show the second; icon sets are never mixed |
| 7 `Clipboard::label` | **shown**: one Clipboard with a label, a Widget Info line | text in theme colours on a shown element |
| 8 the tooltip's size change | **stated** as a literal the theme does not reach; the unit reads whether it honours the reduce-motion flag and states that too | motion with no theme value |
| 9 right to left | T6 | |
| 10, 11, 12 | **recorded here, not shown** | caller's values, a removed part, or no route through the widgets the showcase uses |

### T4 — Keyboard focus on the connector's tab bar

At 0.10.0 each tab inside gpui-base's `Tabs` has a focus handle, the list is one tab stop, and the
arrows, Home and End move the focus (§1.3 row 4). The connector's `TabBar` builds bare gpui-base
tabs in a scrolling row (`CN/src/widgets/tab_bar.rs`), so its tabs take the keyboard focus and
show nothing.

- (a) Leave it.
- (b) The theme's focus ring whenever a tab has the focus, as the theme-drawn `Checkbox`, `Radio`,
  `Slider` and `Switch` draw theirs (`CN/src/widgets/mod.rs`, `FocusRing`).
- (b′) The theme's focus ring on keyboard focus only, as upstream's `Tab` draws its own
  (`focus_visible`, `GC/tab/tab.rs:962-973`).
- (c) Take the tabs out of the tab order again.

**(b′), and the strip follows the focus.** (a) is an accessibility defect the upgrade brings: a
keyboard user cannot see where the focus is. (c) removes a keyboard route that upstream's tabs
now have. (b) would show a ring after any focus by pointer, where the selected state already says
which tab is open; upstream chose the keyboard-only form for the same element, and issue #4's
fourth request asks for it.

A follow-up trial on 2026-11-16 settled how (its report is `report.md` in the folder
`v072-open-trial` beside `REC`, with its tests in the patch there; the same on both cores):

- A click on a tab activates it and never moves the focus: gpui-base's `Tab` stops the press
  (`GB/tabs.rs:194`) before GPUI's focus move runs (gpui-pre 0.3.12 `window.rs:6238`,
  `elements/div.rs:2828`). So the connector's doc, the README and the CHANGELOG say that the tabs
  take the focus from the keyboard only.
- gpui-base's `Tab` accepts a keyboard-focus style, `.focus_visible(…)`. In the theme's ring
  colour and width it is drawn inside the tab, on keyboard focus and not after a click; the
  scrolling row cuts it only where it cuts the tab itself. The ring is an inset shadow whose
  spread is the theme's ring width, upstream's own form for its `Tab`: it moves nothing in the
  tab. A border would either shift the tab's content when it appears or take that width from
  every tab all the time. Tests read a painted shadow on GPUI Fast only (gpui-pre gives tests no
  access to shadows), so the ring's test runs in the gate's GPUI Fast form, on code both cores
  share. `FocusRing`, which draws the theme's offset band outside a control, is not used: the
  scrolling row would cut a band outside the tab above and below. The theme's ring offset is
  stated in the Widget Info as not reached, with that reason.
- gpui-base gives no handle, callback or event for the focused tab, but GPUI's public API does
  the work: a one-pixel child of each tab with a focus handle of its own answers whether the focus
  is within that tab, and on a change the bar scrolls to it. The trial built this and tested it
  with a tab outside the strip. The backlog gets a request to upstream for a focus callback on
  `Tabs`, which would replace the probe; the draft is not sent without the maintainer's approval.

### T5 — The connector's slider against upstream's

- (a) Correct the statements (S4) and leave the behaviour: the doc no longer says "Unlike
  gpui-component's", and the README says which keys and actions upstream's slider has from 0.10.0
  that the theme-drawn one has not (Page Up and Page Down, a numeric `SetValue`, `Release` after a
  key's `Change`).
- (b) Correct them and bring the theme-drawn `Slider` to upstream's keys and actions.

**(a), with (b) as a backlog entry.** The upgrade breaks nothing in the theme-drawn slider: its
behaviour is v0.7.1's, and no theme value and no drawn part is involved. (b) is new behaviour in a
release that is already large, and one of its values, the page step of ten, would be a toolkit's
literal with no desktop's fact behind it (T9 refuses that where the claim is about a platform). It
is about half a day by estimate and a step of its own. The connector's slider is built from
gpui-base's parts (`SliderTrack`, `SliderIndicator`, `SliderThumb`) with a key handler of its own,
and upstream's new key and `SetValue` handlers are on `gpui_base::Slider` alone
(`GB/slider.rs:726-735`), so they do not run beside it.

### T6 — Right to left

From 0.10.0 `gpui_base::init` mirrors upstream's widgets on a system whose language is written
right to left, with no action of the application.

- (a) Nothing. A window that holds upstream's widgets and the connector's then runs in two
  directions.
- (b) The connector's own widgets follow `LayoutDirection::global`: the direction the slider, the
  progress bar and the switch run in, the tab bar's order and edges, and whatever else in them
  names a side. Counted by pattern in six files of `CN/src/widgets/`: 27 calls that name a
  physical edge (some are getters such as `tab.left()` and symmetric pairs, which do not change)
  and 5 row layouts, plus the geometry computed in pixels; which of them a mirrored layout changes
  is read in the unit; what this document does not name (the arrow keys, the scroll's origin)
  is as upstream's counterpart widget does at 0.10.0. Tested under `set_global`, and looked at
  once: during the unit the
  theme-drawn widgets are captured in a right-to-left window and the capture is read, because
  tests alone have missed layout defects in a showcase before. The showcase sets left to right at
  start and says so, because its own layout is written for that direction; a direction switch
  and a mirrored showcase are a step of their own in the backlog.
- (c) (b), and the showcase gets the direction switch now. Counted by the same patterns outside
  its tests: 46 calls that name a physical edge (`demo.rs` 31, `app.rs` 7, `inspector.rs` 4,
  `chrome.rs` 2, `elements.rs` 1, `info/registry.rs` 1) and 51 explicit row layouts; rows built
  with a bare `flex()` were not counted. Its tests hold 128 more lines that read a left or right
  edge, of which those that assert a side would need a second direction.
- (d) The connector forces left to right for everyone. Refused: the direction is the system's and
  the application's, not the theme's.

**(b); (c) is a backlog step.** (a) leaves a mixed window on every right-to-left system, a defect
the upgrade brings to the published crate, so the connector's widgets follow the direction in this
release. The showcase is another matter. A layout direction is not a new element, so the
maintainer's sentence does not ask for (c); pinned to left to right the showcase has no defect and
draws the same on every machine, where without the pin it would mirror upstream's widgets inside
an unmirrored page on a right-to-left system. (c) is a feature of the showcase: about a hundred
counted sites and an unknown number of uncounted rows, a direction switch in the chrome, and a
second direction for the geometry tests that assert a side, an estimate of one and a half to two
and a half days (not measured). One small step at a time puts it in the backlog, with these
counts. Until it is built the mirrored widgets are seen in the unit's capture and held by its
tests.

### T7 — `A11yDisabled` and the choice of core

GPUI Fast 0.1.16 implements `IntoElement` for every `Element` (`gpui-fast-0.1.16/src/element.rs:146`),
and gpui-pre 0.3.12 requires each `Element` to bring its own (`element.rs:52`, `Element: 'static +
IntoElement`). The connector's `A11yDisabled` is an `Element`, so its `IntoElement` must exist on
one core and must not on the other.

- (a) Gate the impl on the connector's own `gpui-fast` feature, as gpui-base gates its copy on its
  own. An application on GPUI Fast must then turn on `native-theme-gpui/gpui-fast`; one that turns
  on only `gpui-kit/gpui-fast` gets error E0119 inside the connector.
- (b) A wrapper that is not an `Element`. Not available: marking an accessibility node disabled
  has no public route in GPUI or gpui-base (the backlog's entry, unchanged at 0.10.0), which is
  why the copy exists.
- (c) Detect the core without a feature of the connector's own. A crate cannot read another
  crate's features in a `cfg`. gpui-kit has a constant for it, `gpui_kit::GPUI_FAST`
  (`gpui-kit-0.10.0/src/lib.rs:122`, `#[doc(hidden)]`, in 0.9.1 too), but a constant cannot gate an
  impl.
- (a+) (a), and a compile-time assertion that `gpui_kit::GPUI_FAST` equals the connector's
  feature, so that the application that turns on only `gpui-kit/gpui-fast` reads a message naming
  the fix. Its cost: the published crate then depends on a `#[doc(hidden)]` item, which upstream
  may remove in any release that the caret requirement admits; a removed item would break the
  released connector for every user, on both cores (upstream removed a public field in a patch
  release once, and it broke the released 0.5.8).

**(a).** It is the only form that compiles on both cores. Upstream's comment on the constant says
what it is for ("Crates layered on the Kit that keep a backend feature of their own check it at
compile time to stay in step"), but the item is hidden from its documentation. (a+) cannot
prevent E0119, and whether its message would appear beside that error was not tried; it trades a
possibly clearer message in one misconfiguration for a way the released crate can stop compiling
for everyone, so it is refused. The backlog gets an entry asking upstream to document the
constant, after which (a+) is free.

(a)'s cost is a claim narrowed: the manifest says the connector "follows an application that
turns on `gpui-kit/gpui-fast`". That sentence and the README's *Backends* say from v0.7.2 that an
application on GPUI Fast turns on the connector's `gpui-fast` feature, which turns on the other
two, and the CHANGELOG says that the configuration v0.7.1 documented now fails to compile with
E0119. Whether that configuration built at 0.9.1 was never tested by a gate. The backlog's entry
asking upstream for a public way to mark a node disabled gains this as a second reason.

### T8 — Reduced motion

- (a) Correct the two docs (S8): what the new preference protects, and that the connector's
  request still overrides an explicit `Allow`; an application that keeps motion regardless sets
  the flag after `apply`, as today.
- (b) The connector forwards the theme's request only while the application has made no explicit
  choice. Not possible at 0.10.0: gpui-base publishes the setter and no getter
  (`GB/reduce_motion.rs`: `apply_system_reduce_motion`, `ReduceMotionPreference`,
  `set_reduce_motion_preference`).
- (c) A connector option that leaves the flag to the application.

**(a), with an upstream request drafted for a getter.** The behaviour is v0.7.1's; 0.10.0 only
adds a function that does not reach it. (c) is a new public API for a case no user has raised;
with the getter, (b) needs none. The draft is not sent without the maintainer's approval.

### T9 — Windows scroll bars

Upstream's `init` now gives auto-hidden scroll bars on Windows the mode `Hover` (S7); the connector
maps the model's `overlay_mode` to `Scrolling` on every platform.

- (a) Keep the mapping; correct S7's statements; a facts task.
- (b) Follow upstream on Windows.

**(a).** `docs/platform-facts.md` says what the Windows setting is and marks which scroll bars
follow it as unknown; it has nothing on what reveals a hidden one. Upstream's change is a
statement about Windows without a source of its own, and a platform value is not copied from a
toolkit. The facts task reads Microsoft's documentation of the behaviour; the mapping follows what
it finds.

### T10 — False statements and workarounds

Every row of §1.4 is corrected in v0.7.2, in the unit that touches its file. Three have more to
them:

- S1: the constant `TAG_HOVER_OPACITY`, the hover note and the two tests' reasons go; the tests
  keep asserting the fill.
- S2: the demo's icon stays on one button and a second shows the spinner without one (T3).
- S3: measured on 2026-11-16 (the same trial): a long text in `Tooltip::new(text)` with the
  theme's maximum width on the bubble gives exactly the bubble of today's `tooltip_content` way,
  on kde-breeze, windows-11 and adwaita and on both cores. So the docs say the conversion is no
  longer forced, and the showcase's `action_tooltip` takes upstream's route once the unit has
  measured the one case the trial did not, a tooltip with a bound key. `geometry::tooltip_content`
  stays public: it is still right for a content element the caller builds, and removing a public
  function is not part of an upgrade.

### T11 — Cited upstream lines

- (a) Re-point what the gates read, by script: the 363 colour claims, the 23 prose citations,
  the fallback-stack test's gpui-fast lines, the lint list's reason lines. A line map of each
  cited upstream file between the two versions was computed on 2026-11-16 (in the plan's record
  folder): 385 of the 386 failing citations are pure line moves, one line changed
  (`breadcrumb.rs:144`, re-indented), and two moved ranges in upstream's `checkbox.rs` have a
  changed line inside. The tools the earlier upgrades kept
  (`.superpowers/sdd/todo_code-review-plan/g4/t2/`) are fixed to their own versions and checkout
  and are not used.
- (b) Re-point every cited upstream line in the repository.
- (c) Change the claim gates to find a claim's line by its content instead of its number.

**(a) now; (c) recorded.** A citation no gate reads is left as it is unless its statement changes:
the maintainer's rule of 2026-11-15, "The old ones, you can leave as they are, don't waste any time
on them. Yes the line numbers will shift, but who cares? If we need it later, we can search it."
That rules out (b), and it narrows what N6 of the 0.9.1 document did, which re-pointed every
cited line of the connector, its doc comments included. An unversioned upstream line in a doc
comment that no gate reads then keeps its 0.9.1 number. (c) would end the weekly renumbering:
upstream releases every week and each release has moved hundreds of cited lines. It is a redesign
of two gates and the claims' format, so it is a backlog entry with that reason, not a part of this
upgrade.

### T12 — Backlog, lint list, pins, README, CHANGELOG

- `docs/todo.md`: the entries §1.5 names are updated; new entries for T2 (e), the desktops' own
  mixed mark (T3), T4's request for a focus callback, T5 (b), T6's showcase, T7's request to
  document the constant, T8's getter, T9's facts task and T11 (c); the GPUI Fast entries are
  re-read at 0.1.16, which Cargo's registry holds since the trial.
- `CN/clippy.toml`: the header and every reason line re-read at gpui-base 0.10.0 and gpui-fast
  0.1.16. `Label::new` is not put on the list, which would forbid every label; the unit reads the
  36 sites, and a site that builds sibling labels from one call gives each an id.
- The manifest: the three floors at 0.10.0, the comments that name 0.9.1, 0.1.14 and 0.1.15
  re-read, T7's sentence. The lock: the precise update of §1.2. swash's move to skrifa 0.37.0 is
  left as cargo made it: no precise update moves one edge, and with it the iced graph holds one
  skrifa where it held two.
- `CN/README.md`: the Compatibility table and "Verified against" (from
  `scripts/update_compatibility.sh`), the Quick start, *Backends* (T7: its second case, the
  application that "needs nothing more", goes), the keyboard-focus and assistive-technology
  paragraphs (S4, S5, S6), the match highlight (T2).
- CHANGELOG, in the 0.7.2 entry beside egui's. *Changed*: native-theme-gpui requires
  gpui-component, gpui-base and gpui-kit 0.10.0, with the sentence that an application on GPUI Kit
  0.9.1 stays on `native-theme-gpui = "=0.7.1"`; GPUI Fast needs the connector's `gpui-fast`
  feature, and the configuration v0.7.1 documented fails with E0119 (T7); the tab bar's focus
  ring on keyboard focus and its strip following the focus (T4); the connector's widgets follow
  the layout direction (T6). *Added*: the mapped colour, with its limit (T2); `Checkbox::indeterminate` on
  the theme-drawn check box (T3); the showcase's new parts (T3). No migration section.

## 3. Cost and risk

**Estimate (not measured): about five to seven working days of agent time**, plus reviews, in
eight units: the floors, lock, T7 and version texts to a green build (0.25 day); the new colour
(0.5); the citations (0.5 to 1); the showcase's new parts and Widget Info (1 to 1.5); the tab
bar's focus ring and its scroll to the focused tab (0.5 to 1); the theme-drawn check box's mixed
state (0.5); the connector's widgets right to left (1 to 1.5); statements, workarounds, backlog, lint list, README
and CHANGELOG (0.5 to 1). The version bump, the asset run and the close-out are the release's,
shared with the egui move.

What could make it larger:

- T6: the slider's and switch's geometry is computed in pixels, not by edge helpers; mirroring it
  may reach their tests' assertions (16 physical-edge calls in `CN/src/widgets/tests.rs`).
- GPUI Fast 0.1.16: its sources were not read; the gate's Fast legs decide.
- A GPUI Kit 0.11 before v0.7.2 ships. The floors are caret requirements at 0.10.0, so v0.7.2
  stays what it is; the next release is the next upgrade.

**Stop and re-scope** if a unit takes twice its estimate.

**The release as a whole.** This move and the egui move touch different crates, and nothing forces
one to be built after the other: two worktrees in parallel, each merged whole (the coverage gate
is red for egui until its showcase unit lands). Run once, after both: the full gate, CI with the
MSRV job, macOS and Windows, the asset run, the version and the CHANGELOG's date. Together the two
estimates are about 11 to 16 working days of agent time, which is not a small release. The plan
carries the cut: if one move meets its stop rule, the other ships alone as v0.7.2. The egui-only
lock and the joint lock were measured; the GPUI-Kit-only lock was not, and needs its one
`cargo update`.

Built twice, knowingly: v0.7.1 re-points the gate-read citations at 0.9.1 and v0.7.2 re-points
them at 0.10.0. v0.7.1 is released on 0.9.1, so its gates need the first; here it is a script's
work (T11).

Consequences for the later releases' documents, each a line to change when that release is next to
be built: the rule that rewrites version forms when a release merges `v0.7.2` and the later plans'
accepted lock sets name GPUI Kit 0.9.1; the widget icons plan reads gpui-component's own icons,
which gain three (§1.5).

## 4. What is decided, and what is the maintainer's

Every decision above is made from the sources, the measurements and the project's rules; none is
left as a question. What changes for an application, or falls short of the maintainer's sentence
of 2026-11-16, is listed here so that it is approved knowingly:

1. T2: the match highlight is drawn in the resting text colour, so with a native theme
   `highlight_matches` marks nothing at rest; no one colour reads in all three row states on every
   preset, and this one never hides text at rest and hides it least on the selected row (below 1.5
   in 7 of 34 cases, against 20 for the theme's `info`, 10 of them invisible). The real fix is
   upstream's.
2. T6: the connector's own widgets follow the layout direction. The showcase stays left to right;
   a mirrored showcase with a direction switch is a backlog step.
3. T3: the theme-drawn check box gains a mixed state, a new public method, drawn as upstream
   draws it.
4. T7: an application on GPUI Fast must turn on the connector's `gpui-fast` feature; the
   configuration v0.7.1's README called sufficient no longer compiles.
5. T4: the tab bar shows the theme's focus ring on keyboard focus, drawn inside the tab, and its
   strip scrolls to the focused tab; a click on a tab does not move the focus.
6. T5, T8, T9: the theme-drawn slider, reduced motion and Windows scroll bars keep their
   behaviour; each has a backlog entry.
7. T12: v0.7.2 requires GPUI Kit 0.10.0, as the maintainer set on 2026-11-16; an application on
   GPUI Kit 0.9.1 stays on `=0.7.1`, and the CHANGELOG says so.

The maintainer's: the approval of this document, its specification and its plan; sending any
of the upstream drafts (T2, T4, T7, T8); everything that publishes.

## 5. What was checked, and what was not

Checked on 2026-11-16: the highlight's contrast on every preset (T2); the tab bar's focus on a
click and on the keyboard, what a keyboard-focus style draws there and how the strip can follow
the focus (T4), and the tooltip's two layouts (T10), by a follow-up trial; the lock, the build in
three feature forms, both lint forms, the strict-panic check, the library's and the showcase's
tests and the coverage gate, by the first trial; every row of §1.3 to §1.5 in the sources, by the
two passes.

Not checked:

- macOS and Windows paths, the MSRV job, the full gate, the showcase's screenshots.
- GPUI Fast 0.1.16's sources, beyond the one line T7 cites.
- T2: the measurement's assumptions (the popup on the window background, blending in sRGB, the
  connector's reads as at 0.7.1); rows an application draws itself and the generic `ListItem` were
  not measured.
- T3 row 3: how the theme-drawn check box's two mark paths (the icon, and the stroked polyline)
  take a second mark (read at the level of `CheckboxLook` only). Row 6: what the Loading demo
  does where the chosen icon theme has no loading icon. Row 8: whether the tooltip's size change
  honours the reduce-motion flag.
- The lock with GPUI Kit 0.10.0 and egui 0.36.2.
- T10 S3: a tooltip with a bound key, built both ways.
- S8: the connector overriding an explicit `Allow` (read, not run).
- Which `Label::new` sites build siblings in a loop.
- Whether the Markdown sample draws differently under 0.10.0's white-space folding, and the
  drawing changes `REC/inventory-A.md` marks "not determined" (the Select's popup while searching,
  the textarea's rounding to a device pixel, the plot labels' weight). The unit that owns the
  showcase's new parts checks each against the Widget Info of its widget before the release; the
  asset run's screenshots show the rest.
- The estimate.

## 6. Out of scope

- The egui 0.37 move, the version bump, the asset run and the close-out of v0.7.2.
- A model field for the search-match highlight, the desktops' own mixed mark, the slider's parity
  with upstream's, the showcase's direction switch, the claim gates' line-free form: backlog
  entries (T2, T3, T5, T6, T11).
- The icon hook and the caret's blink, which have documents and releases of their own.
- GPUI Kit features with no theme value that the connector could use (the Japanese locale, the
  dock's renderer hook).
