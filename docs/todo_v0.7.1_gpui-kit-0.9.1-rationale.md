# GPUI Kit 0.9.1 in v0.7.1: what the port left undone — rationale

Status: written 2026-11-14, revised after three reviews. Approved by the maintainer's instruction of
2026-11-15: "When the plan that goes in the 0.7.1 version is ready, without errors, start implementing it."

Abbreviations: `GC`, `GB`, `GP`, `GF` are the `src/` folders of gpui-component
0.9.1, gpui-base 0.9.1, gpui-pre 0.3.12 and gpui-fast 0.1.15 in Cargo's registry;
`CN` is `connectors/native-theme-gpui`; `SC` is
`CN/examples/showcase-gpui`. A line number without a version is read at those
versions and at commit `0fa28027`.

## 0. What this document is for

v0.7.1 moved the gpui connector from GPUI Kit 0.8.1 to 0.9.1 on 2026-11-13
(commits `8eb8f2e5` to `6f85e1d6`). Each earlier upgrade (0.7.3, 0.8.0, 0.8.1) had
a written pass that decided, for everything upstream added or changed, whether it
is shown, mapped or recorded, re-read the backlog's upstream entries, and
re-pointed every cited upstream line. The 0.9.1 port was made without that pass.
On 2026-11-14 the maintainer asked whether v0.7.1 covers everything GPUI Kit
0.8.0, 0.8.1, 0.9.0 and 0.9.1 brought. It does not. This document lists what is
missing and decides what happens to each item.

Three subjects found in the same audit have documents of their own, because each
is a design of its own:

- gpui-component's own icons, through the icon hook 0.9.0 added:
  `docs/todo_v0.7.1_gpui-widget-icons-rationale.md`;
- the caret's blink, which 0.9.0 made themeable and the model cannot state:
  `docs/todo_v0.7.1_caret-blink-rationale.md`;
- the KDE items still open from issue #4:
  `docs/issues/0004_kde-open-items-rationale.md`.

**0.8.0 and 0.8.1 are covered.** Their upgrades decided every new API in writing
(`docs/archive/todo_v0.7.0_gpui-kit-0.8.0-rationale.md` K8 to K12;
`docs/archive/todo_v0.7.0_gpui-kit-0.8.1-rationale.md` §1.10 and P13). On
2026-11-14 the public items each release added were listed again by comparing the
crates' sources (0.7.3 against 0.8.0, 0.8.0 against 0.8.1) and looked up in those
documents: none is missing from them. That comparison finds new names, not changed
behaviour inside an existing function; the two documents' own passes covered that.

## 1. What was read

### 1.1 Sources

- Upstream's release notes for v0.9.0 (2026-11-02) and v0.9.1 (2026-11-09), from
  `github.com/longbridge/gpui-kit/releases`, read on 2026-11-14.
- The sources of gpui-component, gpui-base and gpui-kit 0.8.1 and 0.9.1, gpui-pre
  0.3.10 and 0.3.12, gpui-fast 0.1.12, 0.1.14 and 0.1.15 in Cargo's registry:
  the files and public items 0.9.1 added, and the theme fields each widget file
  reads, compared between the versions.
- Six read-only passes over the repository and those sources on 2026-11-14, one per
  subject (the icon hook, the caret, the new API against the showcase, the changed
  behaviour against the repository's statements, the backlog's upstream entries in
  two halves). Each reported lines it read; what none of them read is in §4.

### 1.2 What the port did

The connector builds on 0.9.1 on both cores, sets the new `Theme::window_radius`
and `ThemeConfig::window_radius`, keeps the application's new `cursor_blink`
across `apply`, and builds the structs 0.9.0 made `#[non_exhaustive]` from their
defaults (`8eb8f2e5`). The showcase states the window frame's theme-drawn colour
and corners (`8eb8f2e5`), shows `SpeechIndicator` (`55e2654f`), and follows the
menu's sizes in rems in its own popup rows (`9066771e`). `clippy.toml`'s list of
upstream functions that panic was redone. The 209 moved line numbers in the
showcase's Widget Info (`SC/info/`) were re-pointed (`391577a0`), and
`geometry::list_item_selected`'s documentation was corrected for the list rows
0.9.1 letters `accent_foreground`.

`ThemeColor` gained no field in 0.9.x, and the only new `Theme` fields are
`window_radius` and `cursor_blink` (`GC/theme/mod.rs:149-220`), so the connector's
mapping of `ThemeColor` is complete at 0.9.1: no widget found below is drawn in
a wrong colour for want of a mapped token. The editor's highlight scheme, which
is not part of `ThemeColor`, is N9's subject.

### 1.3 What 0.9.0 and 0.9.1 added that v0.7.1 does not cover

From the release notes, each checked in the sources.

| # | Addition | What it draws, and from which theme values | In v0.7.1 today |
|---|---|---|---|
| 1 | `GridLayout` (0.9.1): a grid whose items the user drags and resizes (`GC/grid_layout.rs:40`; the struct is gpui-base's, `GB/grid_layout/grid_layout.rs:56`, `grid_drag_handle` `:251`) | gap `spacing_tokens().md` (`GC/grid_layout.rs:65`); motion `duration_normal`, `easing_move` (`:66`); selected ring `primary` (`:51`); focus ring `ring` (`:52-61`); the drop placeholder `radius_lg` and `primary` at alpha 0.08 and 0.5, dashed (`:78-82`); the resize grip `muted_foreground` (`:86`); literal sizes | not shown, no Widget Info |
| 2 | `SpeechWaveform::liquid()` (0.9.0, `GC/speech/waveform.rs:98`): the input as a water surface | `primary` while capturing (`:137`), else `muted_foreground` (`:139`), as the bars; literal alphas and sizes | the waveform is shown as bars only (`SC/demo.rs:4554`) |
| 3 | `Diff::decorations` (0.9.0, `GC/diff/mod.rs:353`, `GC/diff/decoration.rs:14`): a background on marked lines | default `warning` at alpha 0.2 (`GC/diff/mod.rs:1773`) | the `Diff` is shown without one (`SC/demo.rs:6613`) |
| 4 | `Diff::gutter_markers`, `render_gutter_marker` (0.9.0, `GC/diff/mod.rs:362`, `:371`) | only what the caller's renderer returns (`:1775-1790`) | not used |
| 5 | The editor's gutter and overview-ruler decorations (0.9.0, `GB/input/editor/decorations.rs:79`, `:154`, `:359`, `:435`) | each mark's colour is the caller's, else the editor's foreground (`GB/input/base/element.rs:1679-1686`, `:591-599`); literal sizes | not used |
| 6 | The folded line's placeholder and fill (0.9.1, `GB/input/base/state.rs:1148`, `:1159`) | none: both are the caller's, unset by default | not used |
| 7 | Markdown: the hovered table body row (0.9.1) | `table_hover` (`GC/text/mod.rs:60`, drawn at `GB/text/node.rs:4432-4435`) | the sample has a table (`SC/support.rs:174-178`); Widget Info does not state the hover |
| 8 | Markdown: `on_task_toggle` makes task checkboxes clickable (0.9.1, `GC/text/compat.rs:199`) | the checkbox itself is older (gpui-base 0.8.0 draws it already): border the list's text colour, else `muted_foreground`; checked fill and border `link` (`GB/text/node.rs:3904-3912`; `GC/text/mod.rs:52-53`); the check mark is a literal, white in light mode and black in dark, drawn from bytes gpui-base embeds (`GB/text/node.rs:42-43`, `:3882-3887`, `:3913-3917`), so no icon resolver reaches it; a done task's text is `muted_foreground` (`:3941-3944`); literal sizes | the sample has no task list (`SC/support.rs:161-179`), so the checkbox has never been shown |
| 9 | Markdown: `link_tooltip`, block surfaces (0.9.1, `GC/text/compat.rs:190`, `GB/text/markdown_ext.rs:355`) | only what the caller builds | not used |
| 10 | `ColumnFixed::Right` (0.9.1, `GC/table/column.rs:179`, `:252-258`) | as left-pinned columns: divider `border` (`GC/table/state.rs:1840-1846`) | the Data page has no pinned columns |
| 11 | `PieChart::inner_radius_ratio` (0.9.1, `GC/chart/pie_chart.rs:172`) | no theme value | the pie's hole is sized in pixels (`SC/demo.rs:8473-8481`) |
| 12 | `Slider::track_focus`, `Dialog::accessibility_label`, `TimeField::separator`, `Toolbar::axis`, `ToggleGroup::single`, `DataTable::accessibility_label` (0.9.1) | no theme value; the first four are gpui-base only (`GB/slider.rs:585`, `GB/dialog.rs:496`, `GB/time_field.rs:558`, `GB/toolbar.rs:73`), and gpui-component's widgets, which the showcase uses, do not forward them; the last two are gpui-component's (`GC/button/toggle.rs:303`, `GC/table/data_table.rs`) | not used |
| 13 | `TransitionState`, `InputEditsEvent`, `set_native_touch_selection`, `Transition::reduced_motion` (`GB/motion.rs:304`, `GB/input/base/state.rs:151`, `GC/input/input.rs:39`, `GB/motion.rs:145`) | nothing drawn | not used |
| 14 | `set_icon_resolver` (0.9.0, `GC/icon.rs:77-106`) | the path every icon is drawn from | not used; its own document |
| 15 | `Theme::cursor_blink` (0.9.0, `GC/theme/mod.rs:218-220`) | the caret's blink | kept as the application's; its own document |
| 16 | The highlight theme's `created`, `modified` and `deleted` colours (0.9.0, `GC/highlighter/registry.rs:318-464`) | read by no widget of gpui-component or gpui-base; an application reads them for marks of its own | upstream's default scheme, see N9 |

The release notes also name `Button::selected_accent` (0.9.1). No such item is in
the published gpui-component, gpui-base or gpui-kit 0.9.1 sources (searched for
the name and for every public function with `accent` in its name), so there is
nothing to cover. gpui-shell, which many 0.9.1 notes concern, is not a dependency
of this workspace (`Cargo.lock` has no `gpui-shell` package).

### 1.4 Statements the 0.9.x changes made false or incomplete

| # | Our statement | What 0.9.1 does | Verdict |
|---|---|---|---|
| S1 | `SC/info/chrome.rs:395`: "An item is 26px tall at the default Size" | `rems(1.625)` (`GC/menu/popup_menu.rs:1333-1334`); a rem is the theme's font size (`GC/root.rs:714`), so 26 px only at 16 px | false |
| S2 | `SC/support.rs:1272-1278`: the Combobox "tells a changed selection from an unchanged one by the selection's row indices alone", so the showcase's `PresetDelegate::on_will_change` records each choice at its row in the unfiltered list | 0.9.1 compares the items' values (`GC/combobox.rs:184-200`, `:468-483`; 0.8.1 compared indices, `combobox.rs:184-196`) | false; the workaround's reason is gone |
| S3 | `docs/todo.md:3959-3971`: the report of that defect, to be filed upstream | fixed upstream (PR 3449) | false |
| S4 | `docs/todo.md:2449-2456`: `WindowBorder`'s frame colour and shadow are literals | the frame reads `window_border` (`GC/window_border.rs:201`), which the connector writes (`CN/src/colors.rs:243`, `:481`); the shadow is still a literal (`GC/window_border.rs:253-286`) | false for the frame |
| S5 | `CN/src/icons.rs:602`: upstream draws the Square glyph "only as SpeechButton's stop glyph (speech/button.rs:96-100)" | `SpeechIndicator` draws it too (`GC/speech/indicator.rs:130`); the button's are at `GC/speech/button.rs:135`, `:137` | false |
| S6 | `SC/info/mod.rs:444-448`: gpui keeps a drawn icon by its path and offers no call to drop it, "so the showcase cannot hand an icon upstream builds the chosen theme's" | with `set_icon_resolver` an application returns another path per icon set, which is drawn anew (`GC/icon.rs:84-91`) | the conclusion is false |
| S7 | `docs/todo.md:3972-4016` (a hook for the widgets' own icons, to ask upstream for) and `:3812` (its premise that a path cannot vary per theme) | the hook exists | false |
| S8 | `CN/README.md:368` and `SC/info/mod.rs:390`: gpui-component "marks a selected row by its fill alone" and leaves its text the row's | true while `list.active_highlight` is on, the default; off, the row is `accent` under `accent_foreground` (`GC/list/list_item.rs:257-263`) | incomplete |
| S9 | `CN/README.md:610-617`: a selected `DataTable` cell's fill covers its text | still so, and 0.9.1 adds a 1 px border in `table_active_border` (`GC/table/state.rs:2458-2472`, `:2630-2638`); a row is `table_active` only while `active_highlight` is on (`:2537-2540`) | incomplete |
| S10 | `docs/issues/0004_kde-hand-overrides-upstream-requests.md:30-38`, request 2: "a selected row gets the same token" | only while `active_highlight` is on | incomplete |
| S11 | `docs/showcase-exceptions.toml:20-23`: the gpui section's lines are read at 0.8.1 | they moved (the `SpeechButton` icons, for one, to `GC/speech/button.rs:135`, `:137`) | stale |
| S12 | `docs/todo.md:2187-2202`: gpui-fast-web pins `unicode-properties` at exactly 0.1.3, so the lock stays there | gpui-fast-web 0.1.15 asks for `"0.1.3"` without the pin (its `Cargo.toml:53-54`); the lock still holds 0.1.3 (`Cargo.lock:9916-9917`) | resolved upstream |
| S13 | `docs/todo.md:4140-4150`: gpui-fast pins taffy at exactly 0.15.0, so CI cannot test the taffy an application on the default core gets | gpui-fast 0.1.15 asks for `"0.15.0"` without the pin (its `Cargo.toml:347-348`); the lock still holds 0.15.0 (`Cargo.lock:9302-9303`) | resolved upstream |
| S14 | `docs/todo.md:3888`: a request to make the scroll bar's dimensions themeable | gpui-base has had the track's and the thumb's width since before 0.9 (`GB/scrollbar.rs:644-682`), and the connector writes them (`CN/src/base_layer.rs:79-120`); the table's fixed 16 px is the entry at `:2742` | resolved earlier |
| S15 | `CN/src/widgets/checkbox.rs:430-436`: the connector's own check box draws its check mark's path as a raw `svg` | upstream's check box passes that path through the application's icon resolver since 0.9.0 (`GC/checkbox.rs:220-222`) | the connector's check box is the one widget that draws an icon's path without asking the resolver |
| S16 | `SC/info/chrome.rs:921-926`, `:939`: in the dropdown of a `Select` or `Combobox` "a hovered row is accent at 70% and a selected one accent in full", the fills only | 0.9.1 also letters a hovered and a selected row `accent_foreground`, after the caller's own style, and the row's check mark takes the row's text colour where it was `foreground` (`GC/searchable_list/item.rs:111-123`, `:144-149`; 0.8.1 `:114-117`, `:141`) | incomplete |
| S17 | `CN/src/geometry.rs:1434-1436`: `spacing_tokens()` has "one reader, a `Dialog`'s viewport margin" | `GridLayout` reads it too (`GC/grid_layout.rs:65`), and the questionnaire's metrics read it through the semantic tokens, as they did at 0.8.1 (`GC/questionnaire/components.rs:87-88`, `:1219`); the cited `theme/mod.rs:588-590` is now `:596-598` | false |
| S18 | `CN/src/config.rs:106-107` and the crate documentation, `CN/src/lib.rs:116`: of the editor's scheme "the current line, the line numbers … and the syntax colours stay upstream's"; `CN/src/lib.rs:110` says the four status colours are mapped from the platform | the scheme's own status colours stay upstream's too, where it states them: a hint's in both modes, a warning's in light mode, several backgrounds and borders in dark mode, and the three colours 0.9.0 added (N9) | incomplete |

No statement was found for the other changes the notes list (the submenu's side,
column resizing, marked text, the Settings search, the sliders' steps, the
icon-only tab, the editor's fixes). The scroll-chaining entry
(`docs/todo.md:3893-3957`) still holds at 0.9.1 with moved lines; 0.9.1 adds two
cases it does not mention (a horizontal area whose content fits passes a swipe to
its parent, `GB/scrollable_mask.rs:553-561`; sideways packets keep the overscroll
stream alive, `GB/scroll_bounce.rs:575-577`).

### 1.5 The backlog's upstream entries at 0.9.1

`docs/todo.md` has 56 open `[upstream]` entries about GPUI Kit's crates (one
more, on Xwayland, names them in passing). Each was re-read against the 0.9.1
sources on 2026-11-14:

- 46 still hold; most of their cited lines moved. One of them holds by another
  mechanism: `:2433`, where the check box's disabled colour is now a state
  style that re-applies the caller's refinement (`GC/checkbox.rs:262-268`),
  with the same effect.
- 5 are resolved upstream: `:2187` (S12), `:3888` (S14), `:3959` (S3), `:3972`
  (S7), `:4140` (S13).
- 3 changed: `:2449` (S4); `:3812` (S7); `:2424`, whose list rows are now
  lettered `accent_foreground` where `active_highlight` is off.
- 2 state no claim to re-read (`:3878`, `:3958`).

### 1.6 Cited upstream lines outside the Widget Info

The connector's sources, tests, showcase and README cite upstream `file:line`
places in about 685 lines outside `SC/info/` (392 in `CN/src` that name an
upstream file unambiguously); `docs/todo.md` has about 262. Most carry no version
label. Of 20 sampled from `CN/src` (every twentieth), 11 point at the same line in
0.9.1 and 9 at a line that moved, with the cited content unchanged; none changed
content. So close to half of these numbers are off. About 100 lines of the
connector name an older version than the lock's (gpui-component 0.8.1 in 44,
gpui-pre 0.3.10 in 34, gpui-fast 0.1.12 in 10, gpui-base 0.8.1 in 9, and a few
older ones). The port's own commits (`8eb8f2e5`, `55e2654f`, `9066771e`,
`391577a0`, `6f85e1d6`) wrote or re-pointed some lines outside `SC/info/` at
0.9.1 already.

The earlier upgrades re-pointed all of them by one rule (K3 of the 0.8.0
rationale, used unchanged by P9 of the 0.8.1 one): the line's text where it is
unique in both versions of the file; else the position two line maps agree on;
else set aside and read by hand. The scripts of the 0.8.1 run are kept outside
the repository, in the main checkout's
`.superpowers/sdd/todo_code-review-plan/g4/t2/` (`remap.py`, `upstream.py`,
`gates.py`, `lookup.py`, `notes.py`); the port used `remap.py`'s plan for
`SC/info/` only. They are wired to the 0.8.1 run: the version pairs
(`upstream.py:20-28`), the checkout they read (`remap.py:24`), and the commits
whose lines they leave alone (the set at `remap.py:49`, the rule at `:72-80`).
Commits after the port wrote more citations at 0.9.1 in the same files, most
without a version label: the issue #4 fixes among them (`f4d7abef` alone added
24 such lines, for example `CN/README.md:612`).

### 1.7 The coverage gate

`scripts/check_widget_coverage.py` fails when a widget the toolkit offers is
neither shown in its showcase nor listed in `docs/showcase-exceptions.toml`. For
gpui it discovers widgets in gpui-component's source only: public structs that
render themselves (`check_widget_coverage.py:39-47`, `:503-521`, `:923`).
`GridLayout`'s struct is in gpui-base, and gpui-component only re-exports the
module (`GC/grid_layout.rs:14`, `pub use gpui_base::grid_layout::*`), so the gate
could not report it. gpui-component re-exports two gpui-base modules whole from a public module of
its own: `grid_layout` and `plot` (`GC/plot/mod.rs:6`). A third, `actions`, is
re-exported inside a private module (`GC/lib.rs:25-27`) and is not public. The
gate also decides that a showcase shows a widget by the names it constructs
(`check_widget_coverage.py:384-408`); `GridLayout` is built through the function
`grid_layout` (`GC/grid_layout.rs:40`).

## 2. Decisions

### N1 — What belongs to v0.7.1

**Options.**

- (a) v0.7.1 ships as it is; everything here goes to a later release.
- (b) Everything found goes into v0.7.1, the new connector features included.
- (c) By kind. What makes a statement of v0.7.1 false, and what the standing
  upgrade rules make part of an upgrade, goes into v0.7.1. What needs a new model
  field or a new public API of the connector is a step of its own.

**(c).**

- Against (a): v0.7.1's CHANGELOG says the connector is on GPUI Kit 0.9.1. A
  showcase panel that states a size upstream no longer draws (S1), a workaround
  whose stated reason is gone (S2) and backlog entries that ask upstream for what
  it has shipped (S3, S7) are defects of that release, and a defect found before a
  release is fixed before it. The rule set at the 0.7.3 upgrade, "every new
  element a gpui-kit release brings is shown"
  (`docs/archive/todo_v0.7.0_gpui-kit-0.8.0-rationale.md:451`), and the re-reads
  of the backlog and of the citations (K3, P9, P12) were part of every upgrade so
  far; v0.7.1 is the upgrade to 0.9.1.
- Against (b): the icon hook needs a new public API in the connector and the
  caret's blink new model fields with readers on three desktops. 0.7.x grows one
  small step at a time (`ROADMAP.md`), and each of the two has open design
  questions that this port does not. Their documents decide their release.
- One thing in (c) changes what an application sees: the connector's check box
  and an application's icon resolver (S15). Everything else is in the showcase,
  the documentation, the backlog, comments, a script and the lock file.

### N2 — New elements: which are shown

The rule from the earlier upgrades: "an *element* is shown; a behaviour or option
with no theme value is not, and is recorded"
(`docs/archive/todo_v0.7.0_gpui-kit-0.8.0-rationale.md:528-529`). A variant of a
shown element is shown as upstream's variant (K8 there: `TabVariant::Folder`).

| Item of §1.3 | Decision | Reason |
|---|---|---|
| 1 `GridLayout` | **shown**: a section on the Layout page, with Widget Info | a new element that draws with theme values |
| 2 the liquid waveform | **shown**: beside the bars in the *SpeechWaveform & SpeechIndicator* section, over the same synthetic input, on a row of its own (upstream: "use it for a waveform that spans a row", `GC/speech/waveform.rs:96-97`) | a variant of a shown element |
| 3 `Diff::decorations` | **shown**: one decoration on the Typography page's `Diff`, with its default background, and a Widget Info line | it draws a theme value (`warning` at 0.2) |
| 7 the hovered table row | **stated**: a Widget Info line on the Markdown panel; the sample already has a table | it draws a theme value on an element already shown |
| 8 the task checkbox | **shown**: a task list of two items, one checked, in the Markdown sample, with Widget Info lines; the checkboxes stay unclickable, as upstream draws them by default | an element with theme values that has never been shown; `on_task_toggle` itself is behaviour |
| 4, 5, 6, 9 | **recorded here, not shown** | they draw only what the caller supplies |
| 10 `ColumnFixed::Right` | **recorded, not shown** | an option that draws one more divider in `border`, the hairline the table's other dividers already show; the 0.8.0 pass recorded pinned columns the same way (K12) |
| 11, 12, 13 | **recorded, not shown** | no theme value; behaviour, accessibility or layout |

For each shown item the options were to show it, or to record it as an option.
Recording 1, 2, 3 and 8 would leave theme-coloured parts that nobody has seen
under a native theme, which is what the showcase exists to prevent. Showing 4, 5,
6 and 9 would show the showcase's own drawing, not the theme.

For item 8 a third option was to wire `on_task_toggle` so the boxes toggle. It
adds state to the sample for a behaviour that carries no theme value, so it is
left out.

The Widget Info of each new part states every theme value and every literal the
part draws with, cited at 0.9.1, by the rules the existing panels follow (the
claim gates check them). What the theme does not reach is stated as not themeable: item 1's spacing
token, which has no theme field behind it
(`CN/src/geometry.rs:1434`), and its motion tokens, which are the application's
(`CN/src/lib.rs:1337`). Item 8's panel states the check mark's literal colours
and the done task's text colour too.

### N3 — False and incomplete statements: corrected

Every row of §1.4 is corrected where it stands:

- **S1**: the panel states the item's height as upstream computes it, 1.625 rem of
  the theme's font size.
- **S5, S8, S9, S10, S11, S16, S17, S18**: the sentence is completed or
  re-pointed. S18's two sentences name the status colours the scheme states
  and point to N9's backlog entry. S10 is a
  draft that has not been sent, so it is edited in place.
- **S6**: the panel's reason is rewritten. What it says depends on the icons
  document: if the showcase installs the hook in this release, the panel
  describes that; if not, it says that gpui-component 0.9 has the hook and the
  showcase does not use it yet. Either way the false sentence goes.
- **S15**: the connector's check box passes the path through
  `gpui_component::resolve_icon_path`, as upstream's does. The connector's
  radio draws the same mark through the same code where the theme states no
  radio dot (`CN/src/widgets/checkbox.rs:412`), so it asks the resolver with
  it. It is the one change
  to the connector's behaviour in this document, and it shows only in an
  application that set a resolver (the icons document, W11, says why it belongs
  to the port). The Markdown task box's check mark stays outside any resolver
  (§1.3, item 8); that is upstream's and is said in its panel.
- **S2, S3**: N4. **S4, S7, S12, S13, S14**: N5.

There is no option to weigh here beyond wording: a false statement is corrected.

### N4 — The Combobox workaround

**Options.**

- (a) Keep `PresetDelegate::on_will_change` and correct its comment.
- (b) Remove the override, so the showcase uses upstream's default hook, and keep
  the test that covers the case.

**(b)**, gated by the test. The override exists only for the defect 0.9.0 fixed.
Reading the source, the case it was written for (typing "nord" and pressing
Enter while `default` is selected in the same row) now emits the change with the
default hook: the selection's values differ (`GC/combobox.rs:468-483`,
`GC/searchable_list/delegate.rs:174-199`). That reading was not run. The
showcase has a test of exactly this case,
`the_theme_settings_switch_the_preset` (`SC/tests.rs:1640-1675`), so the removal
is proved or refuted mechanically: if the test passes on both cores without the
override, the override and its comment go and the backlog entry closes; if it
fails, (a) applies, the comment says what 0.9.1 still does, and the entry is
rewritten to that. Keeping a workaround for a fixed defect would be code whose
reason nobody can check.

### N5 — The backlog's upstream entries, and the two lifted pins

**Options** (those of P12 of the 0.8.1 rationale): (a) leave the entries, since
each names the version it was read at; (b) note only the entries that waited for
a release; (c) every open `[upstream]` entry about GPUI Kit's crates gets its
0.9.1 state.

**(c)**, as at 0.8.0 and 0.8.1, and for the same reason: the list is what an
upstream request is written from, and §1.5 shows what a list nobody re-read
holds (eight of 56 entries were overtaken).

- An entry that still holds gets one dated line with its 0.9.1 lines.
- A resolved entry closes and moves to the archive (`docs/archive/todo_closed.md`),
  with what resolved it. `:3972` closes as a request and continues as the icons
  document's work; `:3812` keeps its gpui-pre half (an application still cannot
  drop a drawn SVG) and loses the premise the hook removed.
- A changed entry is rewritten to what still stands: `:2449` to the shadow
  alone; `:2424` gains the rows 0.9.1 now letters `accent_foreground` (a list
  row with `active_highlight` off, and the dropdown rows of S16) and keeps the
  rest. `:2433` still holds by another mechanism and gets a dated line saying
  so.
- `ROADMAP.md`'s bullets that cite upstream lines are re-cited in place, since the
  roadmap states the current plan.

**The two pins.** `:2187` says what to do once gpui-fast-web drops its pin: update
`unicode-properties` in the lock and check the result. `:4140` asked for the taffy
pin to be re-checked at the next upgrade; without it the workspace can lock the
taffy an application on the default core resolves.

- (a) Leave the lock; close the entries with a note.
- (b) Update each in the lock, alone, to the newest version the requirements
  accept, and run the release check.

**(b).** The lock decides what CI, the showcases' screenshots and the parity
tests run on, and the point of both entries was that it could not be what an
application gets. The nightly canary resolves the newest versions and passed on
this branch on 2026-11-14 (run 43582296764), and the 0.8.1 work measured that
gpui-pre lays the showcase out on taffy 0.15.1 as on 0.15.0
(`docs/todo.md:4149-4150`). The versions the update lands on are read from the
lock after it, not assumed. A lock change stales both release stamps; N8 already
needs a new asset run. The CHANGELOG does not record it: a library's lock file
does not reach its users, and the 0.7.1 entry records no other lock-only
update.

### N6 — Cited upstream lines: re-pointed by the K3 rule

**Options.**

- (a) Leave them: the numbers were right at the version they were written at.
- (b) Re-point every cited line of the files in P9's scope with the K3 rule, as
  each earlier upgrade did.
- (c) Stop citing line numbers; cite the file and the item's name.

**(b).**

- Against (a): most citations name no version (§1.6), so a reader takes them for
  the locked one, and close to half are then wrong. The connector's README and
  doc comments are the product.
- Against (c): the project's rule is that a claim about upstream names the line
  it was read at, and the showcase's claim gates check claims against those
  lines. A citation without a line cannot be checked by a gate or by a reader in
  one step. The cost of (b) recurs with every upgrade, but it is one script run
  and a bounded hand-read (the 0.8.1 run: about 135 chains read).
- The scope is P9's: `CN/` (sources, tests, examples, README, manifest comments;
  `SC/info/` is done), `docs/showcase-exceptions.toml`,
  `scripts/connector_parity_exceptions.toml`, `tools/showcase-support/src`, the
  iced and egui showcases' citations of GPUI Kit's crates, with P9's three
  attribution rules, and the two present-tense citations in `native-theme/src`
  that P9 names, read by hand. The crates in scope are those whose locked version
  changed since those citations were last re-pointed: gpui-component and gpui-base
  0.8.1 to 0.9.1, gpui-pre and its sub-crates 0.3.10 to 0.3.12, gpui-fast and its
  sub-crates 0.1.12 to 0.1.15. gpui-kit and gpui-kit-assets are not: their
  `src/` folders are the same in 0.8.1 and 0.9.1, so a line that cites them
  keeps its number and only its version label becomes the locked one.
- The scripts are adapted before the one run: the version pairs, the worktree
  as the checkout they read, and the commits whose lines are left alone. A line
  last changed by the port's first commit, `8eb8f2e5`, or by a commit made for
  this release after it, on this branch or on a branch merged into it, was
  written against 0.9.1 and is left alone, by the scripts' existing rule (the
  commit that last touched the line), so nothing is renumbered twice. In git's
  terms: every commit reachable from the branch's head and not from
  `8eb8f2e5^` nor from the `main` side of the merge `0d447c56`, whose commits
  are older documentation work. Those lines are listed and read by hand
  instead, in case one was only moved and not re-read.
- A version label on a line that is re-pointed becomes the locked version; a
  label in a sentence about the past stays.
- `docs/todo.md`'s citations are not rewritten: its entries get N5's dated lines.

Whether to move the scripts into the repository was considered. They are used
once per upgrade and have lived in a working folder through three of them; in the
repository they would have to meet its quality bar and be kept working. They stay
where they are, and the specification names their location.

### N7 — The coverage gate sees what gpui-component re-exports from gpui-base

**Options.**

- (a) List `GridLayout` in the showcase by hand and leave the gate.
- (b) The gate also discovers, by its existing rule, the widgets in the gpui-base
  modules that gpui-component re-exports whole from a public module
  (`pub use gpui_base::<module>::*`).
- (c) The gate discovers widgets in all of gpui-base.
- (d) The gate also discovers the gpui-base widgets that gpui-component makes
  public under its own name in any way: a module re-exported whole, a module
  re-exported by name, or a type re-exported by name.

**(d).** With (a) the next widget upstream adds this way goes unseen again; the
gate exists so that a person does not have to notice. (c) would report the base
layer's building blocks that gpui-component wraps in widgets of its own (a
slider, a dialog, a toolbar), each of which would need an exception that says
nothing. (b) sees `GridLayout` and misses the same case in its other forms:
gpui-component already re-exports gpui-base's `ResizablePanelGroup`,
`ResizablePanel`, `ResizeHandle`, `Scrollbar`, `ScrollableMask` and
`VirtualList` by name (`GC/resizable.rs:18`, `GC/scroll/mod.rs:3-5`,
`GC/virtual_list.rs:2`, `GC/lib.rs:114-117`, `:122`), did so at 0.8.1, and the
gate has never seen them. (d) follows what is public through gpui-component,
which is what an application can build. Run over gpui-base 0.9.1 on
2026-11-14, the gate's own rule finds under (d): `GridLayout`, those six, and
two of the plot module's building blocks, `PlotElement` and `PlotAppearScope`,
which (b) would find as well and which get an exception as parts of the
charts. Each of the others is then shown, or gets an exception with its
reason, as the file's rules require. The gate matches names, and gpui-base
has names that mean two things (two types called `Text`, of which one is a
struct and the other renders; a `Collapsible` widget and a `Collapsible`
trait, of which gpui-component re-exports the trait); the rule must follow the
re-exported path, not the name alone.
The gate's table of constructor functions gains `grid_layout`, so the new
section counts as showing the widget. `docs/showcase-exceptions.toml`'s header
and its gpui lines are re-pointed to 0.9.1 by N6.

### N8 — CHANGELOG, README, version and assets

- The version stays 0.7.1 (the maintainer's decision of 2026-11-14).
- CHANGELOG, in the 0.7.1 entry: the showcase's new parts (N2) under *Added*;
  under *Fixed*, the connector's check box and an application's icon resolver
  (S15); the README's two completed
  sentences (S8, S9) and the crate documentation's (S18) in the entry that
  already describes the README; nothing
  for comments, citations, the backlog and the lock, which no user of the
  crates sees.
- The connector's README changes in its text only where S8 and S9 complete a
  sentence; its cited upstream lines and version labels are re-pointed with
  the rest (N6).
- `docs/todo_gpui-full-theme.md` gets the note every upgrade appends to it
  (the 0.8.0 and 0.8.1 notes at `:724` and `:754`; the reason in
  `docs/archive/todo_v0.7.0_gpui-kit-0.8.1-rationale.md:1177-1178`): the
  upstream version its theme map was checked against, and what §1.2 found
  changed. It is a standing part of an upgrade (N1), and the port left it
  out.
- The CHANGELOG gets one `**Development**:` line for the coverage gate's new
  discovery (N7), as 0.7.0's entry has for the same script
  (`CHANGELOG.md:198`).
- `ROADMAP.md`'s paragraph on v0.7.1 (`:86-110`), which names `SpeechIndicator`
  as what the showcase gained from 0.9.1, names N2's four parts as well.
- The showcase's sources and the lock change, so both release stamps go stale:
  the asset run is repeated once, after the last change of all four documents'
  work that lands in v0.7.1, and the CHANGELOG's date follows it.

### N9 — The highlight theme's status colours: recorded, with what a fix must weigh

The connector installs upstream's default highlight scheme and replaces only the
editor's fill and its gutter's (`CN/src/config.rs:108-122`). That scheme states
some status colours of its own: in light mode `warning`, `hint`, `created`,
`modified` and `deleted`; in dark mode `hint`, `created`, `modified`, `deleted`
and several backgrounds and borders (`GC/theme/default-theme.json:113-128`,
`:315-340`). Where a scheme states none, upstream falls back
(`GC/highlighter/registry.rs:361-462`): `error`, `warning`, `info` and `success`
to the theme's `red`, `yellow`, `blue` and `green`, which the connector maps
from the platform's danger, warning, info and success colours
(`CN/src/colors.rs:691-697`); `hint` to the theme's `cyan`, which the connector
derives (`:711-717`); each background to the window's background blended with
its colour; `created`, `modified` and `deleted` to success, warning and error.
So today an editor's warning diagnostics are upstream's #C99401 in light mode
and the platform's warning colour in dark mode, and the three colours 0.9.0
added are upstream's in both.

Who draws them: the editor's diagnostic underline takes `error`, `warning`,
`info` and `hint` (`GC/input/input.rs:570-575`); the diagnostic popover letters
its text in the severity's colour on that colour's background, framed in its
border (`GC/input/popovers/diagnostic_popover.rs:42-59`,
`GC/highlighter/diagnostic_styles.rs:6-34`); `success`, `created`, `modified`
and `deleted` are read by no widget. The showcase shows no diagnostic.

The status colours are one public field with a default
(`HighlightThemeStyle::status`, `GC/highlighter/registry.rs:485-486`;
`StatusColors` derives `Default`, `:318`).

- (a) Leave the scheme as it is and record the inconsistency.
- (b) Clear all status colours of the installed scheme, one assignment in
  `config::highlight_style`, so every one falls back as above.
- (c) Clear all but `hint`'s three, which needs the style taken apart through
  its serialised form, since the single colours are private.

**(a), recorded as a `[v0.7.x]` backlog entry with the figures below, and the
connector's two sentences on the scheme completed (S18).** The connector's rule
is that a theme's value wins over a toolkit's literal, and neither (b) nor (c)
is much code. What they do to a diagnostic's legibility, computed from the
presets' colours on a white background:

- (b) is ruled out by `hint`: it becomes the connector's derived cyan, and its
  contrast against white falls from 3.84 (upstream's purple) to 1.57 on
  adwaita, 1.50 on kde-breeze and 1.48 on macOS; on its own tint, where the
  popover letters it, to about 1.4. A hint would all but vanish.
- (c) changes one colour a widget draws in light mode, `warning`, lettered on
  its tint: upstream's reaches 2.27; the platform's warning colour gives 1.86
  on adwaita, 1.87 and 1.95 on the two macOS presets and 1.43 on nord, and
  2.30 on kde-breeze, 3.98 on windows-11 and 4.82 on material-3. Better on
  some desktops and worse on others, in a range where none is easy to read.
- In dark mode both are mixed as well: `info` gains (the stated
  `info.background`, #0059D1, lies under the platform's info text today), and
  under (b) `hint` too; `warning` and `error` lose a little.

So (c) would make the scheme consistent, not better, and whether it is right
cannot be judged here: the model's warning colour is made for a filled badge or
an icon, the model has no colour for a diagnostic's text, and the showcase
shows no diagnostic to look at. Deciding it means showing one of each severity
in the showcase's editor and judging both modes, which is a design of its own.
Under N1's rule a false statement of this release is corrected, and with S18
the connector's documentation says what is true: the status colours are partly
upstream's scheme. The inconsistency itself is older than this port (0.8.1's
scheme states the same `warning`); 0.9.0 added three colours of the same kind,
which no upstream widget draws.

## 3. What the maintainer approves

1. N1: this document's items go into v0.7.1, the connector's check box asking
   an application's icon resolver (S15) among them; the icon hook and the
   caret's blink are decided in their own documents.
2. N2: the four new parts of the showcase (`GridLayout`, the liquid waveform, a
   `Diff` decoration, a Markdown task list) and the items recorded instead.
3. N5: the lock moves `unicode-properties` and `taffy` to the newest versions
   their requirements accept.
4. N8: one more asset run before the release, for which the desktop is needed
   for about five minutes.
5. N7: the coverage gate's new rule, and that the widgets it finds are shown or
   excepted.
6. N9: the editor's status colours stay partly upstream's in this release, said
   in the connector's documentation and recorded, although the connector's
   rule is that a theme's value wins.

N3, N4 and N6 follow the standing rules of the earlier upgrades and need no
decision beyond this document's approval.

## 4. What was checked, and what was not

Checked on 2026-11-14 by reading the sources named: every row of §1.3, §1.4 and
§1.5.

Not checked:

- N4's reading of the Combobox was not run; the test named there decides it.
- The citation sample of §1.6 is 20 of 392; the share that moved is an estimate.
  The count of 685 includes lines whose file name exists both upstream and in the
  connector.
- Which versions the two lock updates reach, and whether gpui-fast's feature
  build accepts them: read from the lock and the release check after the update.
- Which of the six widgets N7 names (beside `GridLayout`, which gets its
  section, and the two plot parts, which get exceptions) are already shown in
  the showcase and which need a section or an exception: the specification's.
- The list of the 56 backlog entries is built from the file when the
  specification is written; the count here comes from the two re-reads.
- N9's contrasts were computed on a white background, not on each preset's own
  window colour.
- Changes inside functions that neither the release notes nor a changed theme
  read point to. The repository's gates (the claim gates over the Widget Info,
  the contract tables, the showcase's tests on both cores, CI) passed on 0.9.1
  at `0fa28027`; they cover what the showcase states and draws.
- gpui-fast's sources were read only where a backlog entry cites them.

## 5. Out of scope

- The icon hook, the caret's blink and the KDE items: their own documents.
- egui 0.37 and the move to zbus 6 (`docs/todo.md`, "Dependencies: ...").
- Upstream requests: none is filed by this work. The four drafts of issue #4 and
  the earlier ones wait for the maintainer.
