# v0.5.9 — The gpui showcase as an application: Specification

Status: Design (2026-09-22); nothing implemented
Companion rationale:
[`todo_v0.5.9_showcase-app-rationale.md`](todo_v0.5.9_showcase-app-rationale.md)
(decisions D1–D13)
Companion plan:
[`todo_v0.5.9_showcase-app-plan.md`](todo_v0.5.9_showcase-app-plan.md)
Builds on the archived widget-info work
([`archive/todo_v0.5.9_widget-info-spec.md`](archive/todo_v0.5.9_widget-info-spec.md)):
its citation gates are kept and move; its block convention is replaced.

---

## 0 -- Scope

### 0.1 What this delivers

1. **Real window chrome** (§1, §2): the window's own title bar with menus,
   a toolbar, a navigation sidebar, draggable splitters, an inspector and a
   status bar — each a gpui-component widget themed by the connector.
2. **Per-instance Widget Info** (§3–§6): every widget on screen reports
   itself; the innermost hovered one wins; its info is derived from its
   kind and configuration.
3. **Pages without chrome samples** (§7): the chrome widgets' samples leave
   the Layout and Overlays pages.
4. **Scrollbars never cover content** (§8), where a receiver exists.
5. **One new public builder**, `geometry::toolbar`, and
   `geometry::icon_size_toolbar` reading `toolbar.icon_size` (§9).
6. **Gates for the new unit** (§10) with discrimination proofs, and the
   documentation that goes with a changed showcase (§11).

### 0.2 Constraints

- **NEVER LIE, NEVER INVENT.** Every colour claim is read at the line it
  cites; every note about upstream cites `file.rs, Symbol`; a claim that
  cannot be verified is a finding, never a sentence.
- No runtime panics, no `unsafe`, no hardcoded theme values. Layout
  defaults the model does not state (a panel's initial width) may be
  literals and are named constants with a comment saying the model states
  none.
- `native-theme-gpui` gains exactly the public items of §9. Nothing else in
  `src/` becomes public.
- Every new gate ships a discrimination proof: a seeded defect makes it
  fail, naming the file and line.
- Every phase of the plan ends with `./pre-release-check.sh` green and a
  commit; no phase leaves the showcase broken.
- Verified against gpui-component 0.6.6, gpui-base 0.6.6, gpui-pre 0.3.6
  (the `Cargo.lock` pins). Line numbers below are those versions'.

### 0.3 Out of scope

The iced showcase (it gets per-instance info in a later release; filed in
docs/todo.md); pinning an info panel; gpui's inspector as a user-facing
feature; screenshot diffing; any change to what `apply` writes other than
§9.

---

## 1 -- The window

### 1.1 Layout

```
┌ TitleBar ─────────────────────────────────────────────────── ─ □ × ┐
│ native-theme showcase — <preset> (<mode>)   File  View  Theme  Help │  menus: Linux and Windows
├ Toolbar ────────────────────────────────────────────────────────────┤
│ [sidebar] [preset ▾] [System|Light|Dark] [icons ▾] │ [⌘K] [↻] [ⓘ] │
├───────────┬╫┬───────────────────────────────────────┬╫┬────────────┤
│ Sidebar   │║│ page content (scrolls)                │║│ Inspector  │
│ • Buttons │║│                                       │║│ [Widget|Theme]
│ • Inputs  │║│                                       │║│            │
│   …       │║│                                       │║│            │
├───────────┴╨┴───────────────────────────────────────┴╨┴────────────┤
│ StatusBar: <desktop> · <preset> <mode> · <font> · text ×<s> · … │ <hovered widget> │
└─────────────────────────────────────────────────────────────────────┘
```

`║` are the handles of one `h_resizable` group (gpui-component re-exports
`h_resizable`, `resizable_panel`, lib.rs:107-110); dragging them resizes the
neighbouring panels.

### 1.2 Window options

The window opens with `WindowOptions { window_bounds, window_decorations:
Some(WindowDecorations::Client), ..TitleBar::window_options() }`
(gpui-component title_bar.rs:81-92; gpui-pre platform.rs:576-582). On
macOS the TitleBar leaves room for the traffic lights, which upstream
already does (title_bar.rs:16-17). The screenshot and `--screenshot`
capture paths keep working (plan Task 8 verifies).

### 1.3 Named layout defaults

`NAV_WIDTH`, `INSPECTOR_WIDTH` and `WINDOW_SIZE` are the panels' and the
window's initial sizes. They are named constants, each with a comment
saying the model states no such value (`SidebarTheme` has no width; there
is no inspector in the model). Nothing else in the chrome is a literal
size: text uses the platform's text scale or rems, spacing uses
`geometry::widget_gap`, `container_margin`, `window_margin`, `section_gap`.

---

## 2 -- The chrome

### 2.1 Title bar (D2)

`TitleBar::new()` refined by `geometry::title_bar`, as the first child of
the window. Its label reads `native-theme showcase — <preset> (<mode>)`.
On Linux and Windows the AppMenuBar is its next child; on macOS the menus
go through `cx.set_menus` only. `on_close_window` quits the application.

### 2.2 Menus and actions

Actions are declared with `gpui::actions!` and bound with `KeyBinding`;
the same action backs a menu item, a toolbar button and a command-palette
entry. The menus are built once and given to both `cx.set_menus` and
`GlobalState::global_mut(cx).set_app_menus` (the latter guarded by
`cx.has_global::<GlobalState>()`), as the showcase does since 8e22f94.

| Menu | Items (action) |
|---|---|
| File | Quit (`Quit`, `ctrl-q`) |
| View | one item per page (`ShowPage(n)`), Toggle Sidebar (`ToggleSidebar`, `ctrl-b`), Toggle Inspector (`ToggleInspector`, `ctrl-i`), Command Palette (`OpenCommandPalette`, `ctrl-k`) |
| Theme | Reload System Theme (`ReloadTheme`), System / Light / Dark (`SetColorMode`), Preferences… (`OpenPreferences`, `ctrl-,`) |
| Help | About (`OpenAbout`) |

### 2.3 Toolbar (D5)

An application-drawn row styled by `geometry::toolbar` (§9). Children, in
order: the SidebarToggleButton; the preset Combobox (the searchable one the
Inputs page demonstrates today, now the real preset switch); a ToggleGroup
System / Light / Dark; the icon-set Select; a vertical Separator; icon
Buttons with Tooltips for `OpenCommandPalette`, `ReloadTheme`,
`ToggleInspector`, their icons at `geometry::icon_size_toolbar`, their
gap `toolbar.item_gap`. The three Selects that live in the left column
today move here.

### 2.4 Navigation (D3)

A `Sidebar` in the first resizable panel, one `SidebarMenuItem` per page
with the page's icon, `active` on the current page, `on_click` dispatching
`ShowPage`. The SidebarToggleButton collapses it to icons. The TabBar no
longer navigates.

### 2.5 Content

The second panel: an Alert at the top when a theme failed to load (D7,
replacing the hand-coloured banner), then the page, in a scroll area whose
scrolled element carries `geometry::scrollbar_gutter` (§8).

### 2.6 Inspector (D10)

The third panel: a TabBar with two tabs, **Widget** and **Theme**.

- *Widget* renders the current `WidgetInfo` (§3) as sections: the title;
  *Theme colors* with a swatch, the role, the token, the hex value and the
  citation per claim; *Theme config*; *Not themeable*; *This instance*. A
  Copy button puts `WidgetInfo::to_text()` on the clipboard. Before any
  hover it shows one line: "Hover any widget to see what the theme sets on
  it."
- *Theme* renders what the Theme Config Inspector renders today (radius,
  radius_lg, fonts in their defined unit, shadow, scrollbar mode) plus the
  window-level facts no hover target carries: the WindowBorder (drawn by
  `Root`, root.rs:605) and the platform fonts.

The inspector's content carries no info; its TabBar does (§4.4).

### 2.7 Status bar (D6)

A `StatusBar` styled by `geometry::status_bar`. Left: detected desktop,
preset and mode, the platform font in its defined unit
(`defined_size`, showcase-gpui.rs:674), the text-scale factor, and the
accessibility flags that are set. Right: the title of the `WidgetInfo`
currently shown, then the crate version.

### 2.8 Overlays (D7)

- **Command palette**: the Command widget in a Dialog, opened by
  `OpenCommandPalette`; entries: every page, every bundled preset, the
  three colour modes.
- **Preferences**: a Sheet holding a Settings page with the four
  accessibility preferences (`AccessibilityPreferences`); a change calls
  `native_theme_gpui::apply_accessibility`.
- **About**: a Dialog with the crate name and version and the upstream
  versions from `Cargo.lock` known at build time
  (`env!("CARGO_PKG_VERSION")` for ours; upstream's are stated in the
  README's compatibility table and are shown as a link, not duplicated).
- **Theme errors**: an Alert (§2.5).

---

## 3 -- Widget Info: the model

### 3.1 Types

A module `info` in the example holds:

```rust
pub struct ColorClaim {
    pub role: &'static str,     // "bg", "text", "edge"…
    pub field: &'static str,    // the ThemeColor field name
    pub value: Hsla,            // the installed theme's value
    pub cited_at: &'static str, // "<crate>/<path>.rs:<line>" or "showcase"
}

pub struct Note { pub what: &'static str, pub text: String }

pub struct WidgetInfo {
    pub kind: &'static str,       // "Tag"
    pub variant: Option<String>,  // "Danger, outline"
    pub colors: Vec<ColorClaim>,
    pub config: Vec<Note>,
    pub not_themeable: Vec<Note>,
    pub instance: Vec<Note>,
}
```

`WidgetInfo::title()` is `kind`, or `kind · variant`. `to_text()` renders
the four sections in the order and format `widget_tooltip`
(showcase-gpui.rs:617) uses today, with *This instance* last.

### 3.2 How claims are written

Claims are written only as `claim("role", "field", t.field,
"crate/path.rs:N")` calls, and notes only as `.config("what", …)`,
`.not_themeable("what", "…")`, `.instance("what", "…")` calls with string
literal first arguments. That is the shape the gates of §10 parse. A
variant-dependent claim is a `match` whose arms each contain a complete
`claim(…)` call.

### 3.3 Geometry lines are generated

`WidgetInfo::geometry(builder: Builder)` appends the config line for a
`geometry::` builder from one table, `GEOMETRY_NOTES: &[(Builder, &str)]`,
which states what that builder carries (the texts the panels' "geometry"
lines state today, audited in the eighth pass). A demo helper that applies
a builder calls `.geometry()` for it in the same expression (§5.2). Hand-
written "geometry" config lines are not allowed.

### 3.4 Per-kind functions

One function per widget kind, `info::<kind>(t: &Theme, …config) ->
WidgetInfo`, where `…config` is exactly what distinguishes instances of
that kind in the showcase (variant, outline, size, state, whether a
builder was applied). Kinds are grouped by page into files
`info/<page>.rs`. The content of every function is the audited content of
today's panel for that kind, split by the configuration: a gallery panel's
per-variant claims go to the arm of that variant; its variant-independent
claims and notes go to every instance; its API facts go to *This instance*.

---

## 4 -- Widget Info: resolution

### 4.1 Registration

`InfoExt::info(self, ui: &InfoUi, id: impl Into<ElementId>, info:
WidgetInfo) -> Stateful<Div>` wraps any element in `div().id(id)` with:

- an `on_hover` handler reporting `(id, hovered)` to the registry;
- a `canvas` child, absolutely positioned and full size, whose prepaint
  reports `(id, bounds, epoch)` to the registry.

The wrapper sets no layout of its own; a page that sized the widget
(`.w_full()`, `.flex_1()`) sizes the wrapper instead.

### 4.2 Choosing what to show

The registry keeps, per id, the latest bounds and the epoch they were
recorded in, and the set of ids currently hovered with their `WidgetInfo`.
The window's root element bumps the epoch in a prepaint that runs before
any other (its first child). After every hover change the registry picks,
among hovered ids whose bounds carry the current epoch, the one with the
**smallest area**; ties go to the most recently hovered. A choice that
differs from what is shown replaces it only after it has stayed the choice
for `INFO_SETTLE` (a named constant, 250 ms; the model states no hover
delay — docs/todo.md), so passing over a resize handle or a neighbouring
widget on the way to the inspector does not replace what the reader went
there to read. Then the inspector and the status bar are updated.

### 4.3 Guarantees (tested, §10.3)

1. Innermost wins: pointer over a Button inside a Dialog shows the Button;
   over the Dialog's surface outside the Button, the Dialog.
2. Instances are distinct: two Tags of different variants show different
   infos.
3. Leaving every target keeps what is shown.
4. Stale never wins: after switching pages, no instance of the previous
   page is shown, even where its bounds would contain the pointer.
5. Chrome reports itself: the TitleBar, Toolbar, Sidebar, StatusBar,
   the inspector's TabBar and the resize handles each show their info.
6. Crossing is not hovering: a target hovered for less than
   `INFO_SETTLE` never replaces what is shown.

### 4.4 The inspector's content does not register

The widgets that render the info — the sections, swatches and the Copy
button below the inspector's TabBar — are built without `.info()`, so
moving the pointer into them to read or copy leaves the info in place
(§4.3.3). The TabBar above them is chrome and registers. The content is
the one exemption from §10.1, stated as a named module in the gate.

---

## 5 -- Demo helpers

### 5.1 One helper builds a widget and its info

A module `demo` holds one function per widget kind shown, returning the
wrapped element. It builds the widget and its info from the same
arguments:

```rust
pub fn tag(ui: &InfoUi, cx: &App, id: &'static str, kind: TagVariant,
           outline: bool, label: &'static str) -> Stateful<Div> {
    let tag = Tag::new().with_variant(kind)
        .when(outline, |t| t.outline()).child(label);
    tag.info(ui, id, info::tag(cx.theme(), kind, outline))
}
```

### 5.2 Builders go through the helper

A helper that applies a `geometry::` builder does it with
`native_info(cx, geometry::X, Builder::X, &mut info)`, which applies the
refinement when a native theme is installed and records the builder in
`info` — so a builder is never applied without the info saying so.

### 5.3 Pages and chrome call helpers only

Page and chrome modules construct no gpui-component widget directly; they
call `demo::` (or `chrome::`) helpers and gpui layout primitives (`div`,
`h_flex`, `v_flex`, `canvas`). Page text (section headings, captions) is a
Label and is built by `demo::heading` / `demo::caption`, which report
themselves as Labels.

---

## 6 -- Sections

**Theme colors** — the claims. **Theme config** — live values from the
installed theme and the generated geometry lines. **Not themeable** —
resolvability only: Tier U, our model's gap, or upstream literal with its
citation. **This instance** — what this instance is and does that no theme
could set (a Tag's label, an AvatarGroup's limit, a DatePicker's format),
moved out of *Not themeable* (docs/todo.md, "'Not themeable' has become a
bucket").

---

## 7 -- Pages

### 7.1 Removed samples

| Page | Sample removed | Demonstrated instead by |
|---|---|---|
| Layout | TitleBar | the window's title bar (§2.1) |
| Layout | Toolbar (application-drawn) | the toolbar (§2.3) |
| Layout | StatusBar | the status bar (§2.7) |
| Layout | Sidebar, SidebarToggleButton | navigation (§2.4) |
| Layout | Resizable (four groups) | the body's panel group (§1.1) |
| Layout | WindowBorder | the window (client-side, §1.2); facts in the Theme tab |
| Overlays | AppMenuBar | the title bar's menus (§2.1) |
| Overlays | Command | the command palette (§2.8) |
| Inputs | Combobox over the presets | the toolbar's preset switch (§2.3) |

The TabBar demo on no page: the inspector's tabs are its demonstration.

### 7.2 Kept samples

Everything else stays on its page, rebuilt with `demo::` helpers so every
instance reports itself. Gallery blocks become rows of instances; nothing
else about a page's content changes.

### 7.3 Coverage

`scripts/check-widget-coverage.py` reads every file of the example; a
widget shown only as chrome is shown. `docs/showcase-exceptions.toml` loses
the `WindowBorder` entry only if the window's own border is recognised as
shown; otherwise the entry stays with its reason updated.

---

## 8 -- Scrollbars never cover content

1. Every scroll area the showcase owns — content, navigation, inspector —
   applies `geometry::scrollbar_gutter` to the element it scrolls
   (geometry.rs:543-563).
2. Where an upstream widget scrolls internally and takes a refinement on
   what it scrolls, the showcase applies the gutter there: each
   `SettingGroup` of the Settings sample and of the Preferences sheet
   (setting/group.rs:112 refines the group's own root last; the page body
   at setting/page.rs:222-248 lays a `ScrollbarLayer` over its right edge,
   scroll/scrollable.rs:19-29).
3. Where it does not — Dialog, Sheet, PopupMenu, MessageScroller, Sidebar,
   Tree, the Settings page body — the widget's info says so (Tier U) and
   docs/todo.md records the upstream PR candidate.

---

## 9 -- New public API

```rust
/// An application-drawn toolbar row: `toolbar.bar_height` as its minimum
/// height (KDE's toolbar sizes to its content, platform-facts §2.13),
/// `toolbar.item_gap` between items, `toolbar.border` padding,
/// `toolbar.background_color`, and `toolbar.font` size and weight. No edge:
/// §2.13 states none.
pub fn toolbar(n: Native<'_>) -> StyleRefinement;
```

`geometry::icon_size_toolbar` reads `n.resolved.toolbar.icon_size` (which
inherits `defaults.icon_sizes.toolbar`, model/widgets/mod.rs:453-456)
instead of `defaults.icon_sizes.toolbar`. Both are covered by unit tests
that read the expected values from the resolved theme of every bundled
preset, and by `the_showcase_exercises_every_builder`.

---

## 10 -- Gates and tests

### 10.1 Every widget reports itself (replaces the block gates)

Lexical, over the page and chrome modules: no gpui-component widget
constructor (`Name::new(`, or a named constructor such as `Tag::primary(`,
for every `Name` imported from `gpui_component`) outside `demo.rs` and
`chrome.rs`; and every `pub fn` in `demo.rs` and `chrome.rs` whose body
constructs a widget also calls `.info(`. The inspector module is the named
exemption (§4.4). Discrimination proof: a seeded `Tag::primary()` in a page
fails naming the file and line; a seeded helper without `.info(` fails
naming the helper.

### 10.2 The citation gates move

`every_colour_claim_is_read_at_the_line_it_cites` and
`every_prose_citation_still_exists` read `claim(` calls and note calls from
the `info/` files instead of `hover_info(` arrays. Their checks are
unchanged. `the_omission_report` reads the same claims.
`every_builder_has_a_geometry_note` (new) requires one `GEOMETRY_NOTES`
entry per `pub fn` of geometry.rs that returns a `StyleRefinement`.

### 10.3 Behaviour (windowed tests)

One test per guarantee of §4.3, plus: the splitter handle moves the
panel boundary by the dragged distance; clicking a navigation item shows
its page; the toolbar is at least `toolbar.bar_height` tall for a
preset that states it; no showcase-owned scroll area's content extends under an
always-visible scrollbar (the existing
`a_non_overlay_scrollbar_keeps_off_the_content`, extended to the
inspector and to a Settings group).

### 10.4 Kept

`the_showcase_exercises_every_builder`, `the_showcase_hardcodes_no_style_values`
(extended to `text_size(px(`), `every_tab_lays_out` (renamed
`every_page_lays_out`), the coverage script.

### 10.5 Removed

`every_demo_id_is_a_tt_id`, `every_demo_block_has_a_widget_info_panel`,
`every_demo_names_the_builders_it_applies` (superseded by §3.3/§5.2), the
`tt-` id convention and `hover_info`.

---

## 11 -- Documentation

CHANGELOG `[Unreleased]` entries for the chrome, the per-instance info and
`geometry::toolbar`; docs/todo.md items closed or updated (the toolbar
model, the bucket, the gate that cannot tell whose line it is — if the
spike of plan Task 22 succeeds); the connector README's screenshots are
regenerated by `./scripts/pre-release.sh` as for every release; these three
documents move to `docs/archive/` as the last task.
