# v0.5.9 — The gpui showcase as an application: Rationale

Status: Implemented (2026-09-23) and archived; see *As built* at the end.
Companion specification:
[`todo_v0.5.9_showcase-app-spec.md`](todo_v0.5.9_showcase-app-spec.md)
Companion plan:
[`todo_v0.5.9_showcase-app-plan.md`](todo_v0.5.9_showcase-app-plan.md)
Predecessor, already implemented and archived:
[`todo_v0.5.9_widget-info-rationale.md`](todo_v0.5.9_widget-info-rationale.md)
— the citation gates this work keeps, and the per-block panel it replaces.

---

## 0 -- What this document is for

The gpui showcase is the one place a reader sees native-theme's gpui
connector do its work. Two defects, both found by the maintainer, show that
its structure is wrong rather than its details:

1. **Widget Info describes a demo block, not a widget.** Hovering any of the
   eight Tags on the Feedback page shows one panel titled "Tag (per variant)"
   with twenty colour claims for all eight. A widget nested inside another
   (a Button inside a Dialog) can never show its own info.
2. **The window chrome is demonstrated inside a page.** The Toolbar and the
   StatusBar are drawn in the middle of the Layout page instead of at the top
   and bottom of the window; the Layout page even contains a *second*
   TitleBar nested in the content, and the Overlays page an AppMenuBar with
   no menus. The line between the left column and the content cannot be
   dragged, although gpui-component ships a Resizable panel group and the
   showcase demonstrates it on the Layout page.

This document records why both happened, which alternatives were weighed,
and the decisions (D1–D13) the specification implements. It is written for
the reviewer who has to say yes before implementation starts.

---

## 1 -- The evidence

### 1.1 The panel is keyed on the demo block

`hover_info` (showcase-gpui.rs:2781) formats a fixed text from three
hand-written arrays; `set_info` (:2769) writes that text into the panel when
its `tt-` block is hovered and never clears it. The gates in
`src/showcase.rs` define a demo as "a `div().id("tt-…")` with one
`.on_hover(self.hover_info(`", and require every such block to have one
panel. Nothing ties a panel to a widget: the unit is the block.

58 blocks hold more than one widget. Most are compound widgets whose parts
belong together (a Dialog, a Carousel, a Settings page), and one panel is
right for them. **Thirteen are galleries of independent instances** — Tags
(8), Bubble (7), Marker (5), Notification (4), Badge, GroupBox, Separator,
Checkbox, Toggle, Attachment, Label, Disabled Buttons, Buttons with Icons,
Avatar/AvatarGroup — where the panel must describe every instance at once.

### 1.2 The audit's failures are the same failure

The eight-pass audit of the panels (docs/todo.md, "Finish auditing the
Widget Info…") found 64 wrong "Not themeable" entries out of 236, 30 wrong
colour claims out of 374, 25 claims citing another widget's line, and 36
wrong config lines. Read by cause, a large share are **drift between the
panel and the instance it sits beside**:

- panels describing a variant the demo does not build: an icon Badge, a
  Label's highlights, an outline Kbd, an empty ColorPicker swatch, an
  anonymous Avatar, a masked OtpInput, a Settings group that is Normal;
- panels naming upstream's token where a `geometry::` builder the demo
  applies replaces it: eighteen "border-radius" lines, the Radio and Select
  label colours, the button edges;
- panels whose widget changed while the panel did not (the AppMenuBar that
  has had no menus since gpui-component 0.6.0).

A panel written next to a block, separately from the code that builds the
widgets in it, drifts. The audit corrected the text; it could not remove the
cause.

### 1.3 The chrome is decoration

`Showcase::render` (showcase-gpui.rs:7278) builds a hand-drawn `v_flex`
column 220px wide, the TabBar and a scroll area. The window uses the
platform's decorations; there is no menu bar, toolbar or status bar. The
widgets that *are* an application's chrome are shown as samples inside
pages: TitleBar (Layout, nested, its close button deliberately inert),
Toolbar and StatusBar (Layout), Sidebar and SidebarToggleButton (Layout),
Resizable (Layout, four sample groups), AppMenuBar and Command (Overlays).
A sample of a title bar inside a page demonstrates the widget's colours and
nothing of what it is for.

The chrome also hardcodes what the platform states: `text_size(px(13.0))`
for every section heading (:714), `px(12.0)` for the error banner, `px(11.0)`
for the Widget Info text, a 220px column. The "no hardcoded style values"
gate (src/showcase.rs) covers radii and colours, not text sizes.

### 1.4 What already works and is kept

- The citation gates: every colour claim is read at the line it cites; every
  prose citation names a symbol that exists (archived widget-info spec §4).
- The coverage script `scripts/check-widget-coverage.py` and
  `docs/showcase-exceptions.toml`.
- The showcase's windowed tests (`open`, `click`, `bounds_of`,
  `every_tab_lays_out`, showcase-gpui.rs:8118 onward), which already drive
  the real window headlessly — the base the new behaviour is tested on.
- `Root`'s dialog, sheet and notification layers, and `Root`'s window
  border, which are real already.

---

## 2 -- Why it went this way

The showcase grew as a gallery: a page per category, a block per widget, a
panel per block. Each step was reasonable for a gallery. The panels were
then made *true* (the citation work) without being made *specific*, because
the unit stayed the block. The chrome widgets were added the same way as
every other widget — as a block on a page — because the showcase had no
notion of being an application with chrome of its own.

---

## 3 -- Options considered

### 3.1 Split the gallery blocks into one block per item

Thirteen blocks become about fifty, each with a hand-written panel. It fixes
the Tags symptom and keeps every cause in §1.2: the panels are still written
beside the widgets rather than from them, nested widgets still have no info,
and the chrome is untouched. Rejected as a patch.

### 3.2 gpui's element inspector

gpui-pre 0.3.6 ships an inspector (`src/inspector.rs`): in *picking mode*
hovering an element makes it active and exposes its construction site and
its base style (`DivInspectorState`, elements/div.rs:1853). It is compiled
only with `debug_assertions` or the `inspector` feature, and in picking mode
a mouse-down selects the element instead of reaching the widget
(window.rs:6996-7019). A showcase whose buttons stop working while the info
panel is live is not a showcase. It also reports *elements* (the label div
inside a Button) rather than widgets, and style values rather than the
tokens and native fields that produced them. Rejected as the mechanism;
kept in mind as a research aid (§3.6).

### 3.3 Per-instance info, derived from what the instance is (chosen)

Each widget instance on screen carries its own info. The info is produced
by one function per widget kind from the instance's configuration — the
same values the widget is built from — so a danger-outline Tag reports the
danger-outline colours and nothing else, and cannot report a variant it is
not. The innermost hovered widget wins, so a Button inside a Dialog reports
the Button and the Dialog surface reports the Dialog. This removes the cause
in §1.2 structurally: a panel can no longer be written for a block, and a
builder applied by the helper that builds the widget is also recorded by it.

### 3.4 Keep the chrome as page samples, add a real one beside it

Two copies of every chrome widget, one real and one sample, both needing
info and both needing to stay in sync. Rejected: the real one *is* the
demonstration, and it demonstrates more (a title bar that moves the window,
a menu that runs actions, a splitter that resizes panels).

### 3.5 One document per concern

The per-instance info and the chrome could be two plans. They are not
independent: the inspector panel is chrome, the chrome widgets must report
themselves, and moving the chrome removes eight blocks the info migration
would otherwise have to convert and then delete. One specification with
phased, independently green milestones is the smaller total.

### 3.6 A paint-level gate

A windowed test that reads the colour actually painted at an instance and
compares it with the claimed token would catch the "borrowed line" class
mechanically — the class the citation gate cannot see (docs/todo.md, "The
colour gate cannot tell whose line it is"). Whether gpui's test platform
exposes the painted scene is not established. Scheduled as a time-boxed
spike (plan Task 22), not as a requirement.

---

## 4 -- Decisions

**D1 — The showcase is an application.** Its window has real chrome: title
bar, menu bar, toolbar, navigation, draggable splitters, an inspector and a
status bar, all built from gpui-component widgets and all themed by the
connector like any page content. A chrome widget is demonstrated by being
the chrome; its sample leaves the page.

**D2 — `TitleBar` is the window's title bar.** The window opens with
`TitleBar::window_options()` (title_bar.rs:81) and client-side decorations,
so the TitleBar moves, maximises and closes the real window. On Linux and
Windows the AppMenuBar sits inside it with real menus; on macOS the menus go
to the system menu bar (`cx.set_menus`) and the TitleBar leaves room for the
traffic lights, which upstream already does. Client-side decorations also
make `Root`'s WindowBorder visible, which is itself a demonstration.
*Trade-off accepted:* on KDE the window no longer uses KWin's own frame.
A showcase of the connector's TitleBar needs the TitleBar to be the frame.

**D3 — Navigation is a Sidebar.** Ten pages are a sidebar's worth, not a tab
strip's. The Sidebar collapses to icons through a SidebarToggleButton in
the toolbar. The TabBar gets a real job instead: the inspector's two tabs,
Widget and Theme.

**D4 — The body is a Resizable group.** Navigation, content and inspector
are three panels of one `h_resizable` group, so the lines between them drag.
Their initial sizes are layout defaults, not theme values; the model states
no sidebar or inspector width (docs/todo.md, "A sidebar width and a tooltip
delay are missing from the model").

**D5 — The toolbar is sized by the model's toolbar.** The model states
`toolbar.bar_height`, `item_gap`, `icon_size`, `font`, `border` and
`background_color`, every preset states the first two, and nothing reads
any of them today (docs/todo.md, "The model's toolbar is read by nothing").
A new public `geometry::toolbar` builder carries them, and
`geometry::icon_size_toolbar` reads `toolbar.icon_size`. The toolbar holds
the controls that change the theme — preset (searchable Combobox), colour
mode (ToggleGroup), icon set (Select) — and the actions: sidebar toggle,
command palette, reload the system theme, inspector toggle.

**D6 — The status bar reports the environment and the hover.** Detected
desktop and preset, colour mode, the platform font in the unit it was
defined in, the text-scale factor and the accessibility flags, and the name
of the widget under the pointer. Styled by `geometry::status_bar`.

**D7 — Overlays do real work.** A command palette (the Command widget, in a
dialog, bound to a key) jumps to pages and switches presets and modes. A
Preferences sheet (the Settings widget) edits the accessibility preferences
`apply_accessibility` consumes. Help › About opens a Dialog. A theme that
fails to load is reported by an Alert at the top of the content instead of
a hand-coloured banner. The Overlays page keeps its samples of the overlays
that are not chrome (Dialog, AlertDialog, Sheet, Popover, HoverCard,
ContextMenu, PopupMenu, Notification).

**D8 — Widget Info describes the instance under the pointer.** Every
gpui-component widget on screen — page content and chrome — is built by a
helper that also attaches its info. The info comes from one function per
widget kind, called with the instance's configuration. The innermost
hovered instance wins; leaving every instance keeps the last info shown, so
moving the pointer into the inspector to read it does not erase it; an
instance no longer on screen never wins.

**D9 — Four sections.** *Theme colors* (claims, as now), *Theme config*
(the geometry lines are generated from the builders the helper applied,
plus live values), *Not themeable* (resolvability, as audited), and a new
*This instance* for API and demo facts that no theme could set — the
decision parked in docs/todo.md ("'Not themeable' has become a bucket").

**D10 — The inspector renders structure, not a text box.** Sections with a
swatch per colour, the token, its hex value and its citation; a Copy button
puts the plain-text form on the clipboard. The Theme tab holds what the
Theme Config Inspector shows today, plus the window-level facts no hover
target can carry (the WindowBorder, the fonts).

**D11 — The example becomes a module tree.** 8,684 lines in one file cannot
absorb per-kind info functions, demo helpers and chrome. The example moves
to `examples/showcase-gpui/` with one module per responsibility; the gates
read every file of it and fail if a file is added without being read.

**D12 — The gates follow the unit.** The block gates go; in their place:
every widget constructed in the showcase is constructed by an info-attaching
helper (lexical), every widget kind the toolkit offers has at least one
instance (the coverage script, unchanged in intent), and the behaviour —
innermost wins, stale never wins, chrome reports itself — is tested in the
windowed test harness. The citation gates move to the info module unchanged
in what they check. Every new gate ships a discrimination proof.

**D13 — Out of scope.** The iced showcase (it has no chrome widgets to
promote; it should get per-instance info later, filed); pinning an info
panel; gpui's inspector as a user-facing feature; screenshot diffing.

---

## As built

The decisions held. Three outcomes differ from the text above.

- **§3.6:** the spike succeeded. gpui-pre's test-only
  `Window::painted_quads` (window.rs:2718-2725) returns the last frame's
  quads, and three windowed tests compare a painted fill with its claim: a
  Tag at rest and hovered, and a Theme Map swatch. Every other claim is
  still checked only against the line it cites.
- **D7:** the command palette offers the toolbar's presets — `default` and
  the presets for this platform — not every bundled preset, so a showcase
  on Linux offers no macOS or Windows preset.
- **D12:** one gate was added. The registry keys an instance by its local
  id, so an id drawn twice in one frame is caught at run time, by the
  registry in test builds and asserted by the windowed test harness; a
  lexical version of that gate was written first and replaced, because it
  could not follow every way an id is built.
