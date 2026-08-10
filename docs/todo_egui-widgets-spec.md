# native-theme-egui-widgets — Specification

Target: **egui 0.36.1**. Version milestone: **undecided** — this document does
not claim one. Rationale: [`todo_egui-widgets-rationale.md`](todo_egui-widgets-rationale.md).

Every code citation below resolves against egui 0.36.1 / epaint 0.36.1 as
published. Paths are given the way the sibling documents give them:
`ui.rs:1520` means `egui/src/ui.rs` line 1520; `epaint/src/...` is spelled out.

---

## 0 -- Scope

### 0.1 What this crate is

> A companion widget crate for [`native-theme-egui`](todo_v0.6.0_egui-connector-spec.md).
> It supplies the widgets egui does not have, and correctly-scoped wrappers for
> the ones it does, so that an application gets the platform's appearance
> without threading a `Role` through every call site.

Everything it draws is driven by `ResolvedTheme` values obtained from the
connector's `ThemeAtlas`. It contains no colour, no radius and no metric of its
own beyond the structural constants named in §10.3.

### 0.2 What this crate is not

* **Not a drop-in replacement for egui.** It does not shadow, wrap or replace
  `egui::Ui`, and it never asks an application to change an existing call. See
  §4.2 for why that is a deliberate structural choice and not a limitation.
* **Not a fork.** It depends on published egui; it does not vendor or patch it.
* **Not a theme mapper.** The `ResolvedTheme` → `egui::Style` mapping lives in
  the connector and is not duplicated here.
* **Not a replacement for `TextEdit`, `ScrollArea` or `ComboBox`.** §2.3.

### 0.3 Relationship to `native-theme-egui`

```text
native-theme  ──▶  native-theme-egui  ──▶  native-theme-egui-widgets
  ResolvedTheme      ThemeAtlas               Tier 1 + Tier 2 widgets
                     fonts, icons, install    (this crate)
```

The dependency is one-directional and the connector never learns this crate
exists. Concretely, this crate consumes exactly six items of the connector's
public API (`todo_v0.6.0_egui-connector-spec.md` §4.2):

| item | used for |
|---|---|
| `ThemeAtlas::from_ctx(ctx)` | obtaining the atlas without the app passing it |
| `ThemeAtlas::resolved_for(theme)` | reading leaf values for hand-painting |
| `ThemeAtlas::role_style(theme, role)` | Tier 2 scoping |
| `ThemeAtlas::role_style_variant(..)` | `Selected` / `Disabled` cells |
| `ThemeAtlas::surface_frame(theme, surface)` | container chrome |
| the free accessors (`switch_*`, focus-ring, …) | Tier 1 painting |

**The connector's 62 free accessors are this crate's primary input.** In a
connector-only world they were consolation prizes for values egui cannot
render; here they are the values we paint from.

---

## 1 -- The problem this crate solves

The connector's own honesty ledger states the limit plainly: an application
that calls `install()` and nothing else gets the **31 effective DIRECT** leaves
plus one elected winner per contested field, while the **210 SCOPED** leaves
reach the screen only where the application asks for them
(`todo_v0.6.0_egui-connector-spec.md` §0.1, §1.4 item 2, ledger item 22).

That is not a defect in the mapping. It follows from egui having exactly one
global `Style` with an interaction-state axis and no widget-type axis, which
the connector's rationale establishes at length and which a dedicated sweep
confirmed has no hook in 0.36.1.

The gap is therefore not *knowledge* — the mapping is complete and audited —
but *application*. Somebody has to be at the call site holding the right
`Arc<Style>` at the right moment.

**This crate is that somebody.** The burden moves off the application author
and into a library that can be audited once.

---

## 2 -- The tier model

Every widget in this crate belongs to exactly one tier, and the tier is
recorded in its rustdoc. The tier decides how much this crate owns.

### 2.1 Tier 1 — widgets egui does not have

We allocate, interact and paint ourselves: `Ui::allocate_response`
(`ui.rs:1138`) or `Ui::allocate_exact_size` (`ui.rs:1150`), then
`Ui::painter` (`ui.rs:457`).

We own: geometry, every painted pixel, interaction semantics, the
`WidgetInfo` emission of §8, and any animation.

We do **not** own text layout internals, hit-testing primitives, input
handling or the render backend — those stay egui's.

### 2.2 Tier 2 — styled wrappers over egui's own widgets

We construct egui's widget and run it inside a scope carrying the correct
`Arc<Style>`:

```rust
ui.scope_builder(egui::UiBuilder::new().style(style), |ui| { /* egui widget */ })
```

`Ui::scope_builder` is `ui.rs:2193`; `UiBuilder::style` is
`ui_builder.rs:155` (field `:28`); the child `Ui` takes the `Arc` by clone at
`ui.rs:236`. This reaches every widget egui itself paints **except**
`Area`-based containers, which build their content `Ui` with a bare
`UiBuilder::new()` (`containers/area.rs:611-629`) and fall back to
`ctx.global_style()` (`ui.rs:135`).

We own: choosing the role, and passing container chrome (`Frame`,
`StyleModifier`) where the container accepts one. We own **nothing** of the
widget's behaviour.

### 2.3 Tier 3 — forbidden

Reimplementing the internals of `TextEdit`, `ScrollArea` or `ComboBox` is
**out of scope permanently**, not merely deferred. The measured cost:

| upstream file | lines |
|---|---:|
| `containers/scroll_area.rs` | 1641 |
| `containers/window.rs` | 1482 |
| `widgets/text_edit/builder.rs` | 1440 |
| `widgets/slider.rs` | 1210 |
| all of `widgets/` + `containers/` | **17174** |

`TextEdit` alone carries cursor movement, selection, undo, clipboard, IME and
bidirectional text. Reimplementing it would buy a permanent per-release audit
obligation for behaviour this crate has no opinion about, in exchange for
styling that §2.2 already delivers through a scope.

### 2.4 The admission test

A widget may be added to **Tier 1** only if **both** hold, and the rustdoc
records which:

1. egui 0.36.1 ships no widget with that visual identity, **and**
2. a majority of the corresponding `ResolvedTheme` struct's leaves are
   UNMAPPABLE in the connector's §5 matrices.

This is the connector charter's test (`todo_v0.6.0_egui-connector-spec.md`
§14.3) applied in the crate where the charter says such widgets belong. A
widget that fails the test but would still be *convenient* goes to Tier 2 or
nowhere.

---

## 3 -- Architecture

### 3.1 Obtaining the atlas

Every widget resolves its theme through `ThemeAtlas::from_ctx(ui.ctx())`,
which reads the atlas the connector published into `ctx.data_mut()`. No widget
takes a `&ThemeAtlas` parameter and no widget takes a `&ResolvedTheme`.

**If no atlas is installed, every widget in this crate falls back to egui's own
appearance and renders correctly.** It must never panic, never draw nothing and
never emit a fabricated colour. This is the single most important robustness
property of the crate: a user who adds it before calling `install()` sees
ordinary egui, not a broken screen.

### 3.2 The current colour scheme

egui's active scheme is read once per widget from the `Ui`, not from the OS and
not from the atlas: the atlas holds both schemes and the `Ui` decides which one
applies. Widgets therefore take `egui::Theme` from the context and pass it to
`role_style` / `resolved_for`.

### 3.3 No hardcoded values

Every colour, length, radius, width and gap a widget paints must come from a
`ResolvedTheme` field or a connector accessor. The only exceptions are the
structural constants of §10.3, which are enumerated and justified there.

A numeric literal in painting code that is not in §10.3 is a bug, and the
review checklist of §12 exists to catch it.

---

## 4 -- Public API

### 4.1 Crate root

```rust
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]

pub mod switch;
pub mod tab_bar;
pub mod status_bar;
pub mod toolbar;
pub mod sidebar;
pub mod card;
pub mod expander;
pub mod spinner;
pub mod segmented_control;

pub mod wrap;   // Tier 2

pub use native_theme_egui as connector;
```

The crate re-exports the connector under one name rather than re-exporting its
types individually. Reason: the connector already documents a rule that it
re-exports no name colliding with egui's root, and duplicating that analysis
here would create a second place for it to drift.

### 4.2 Naming: why nothing is shadowed

**Every widget is an `egui::Widget` implementor, used through `ui.add()`.**

```rust
ui.add(native_theme_egui_widgets::switch::Switch::new(&mut wifi_on).label("Wi-Fi"));
```

`egui::Widget` is `widgets/mod.rs:63` (`fn ui(self, ui: &mut Ui) -> Response`)
and `Ui::add` is `ui.rs:1520` (`widget.ui(self)`). This choice is load-bearing
for three reasons:

1. **Shadowing is structurally impossible.** `Ui::button` and its 160 siblings
   are *inherent* methods on `egui::Ui` (`ui.rs:1847` and throughout; 161
   inherent `pub fn` in total). Rust resolves inherent methods before trait
   methods, so an extension trait offering a `button` method would be silently
   ignored at every call site — no error, no warning, wrong pixels. Because we
   never offer a competing method name, that failure cannot occur.
2. **Free integration.** `ui.add_sized` (`ui.rs:1537`) and `ui.add_enabled`
   (`ui.rs:1587`) accept `impl Widget`, so our widgets compose with egui's own
   layout helpers with no extra API.
3. **No false compatibility promise.** A distinct name tells the reader this is
   not egui's widget. §14 item 1 records what that costs.

**No extension trait on `Ui` is provided**, and none may be added later. An
`ui.native_switch(..)` convenience would be additive and harmless *today*, but
it creates a second spelling for every widget and a permanent question at each
call site about which one is current.

#### 4.2.1 The drop-in goal, and why it is not pursued here

"Change one import and existing code looks native" is a legitimate goal. It is
simply not this crate's, and the alternatives are recorded so the question stays
answered rather than re-opened:

| route | reaches existing code | reaches third-party crates | cost |
|---|---|---|---|
| **`Deref` wrapper `Ui`** | partly | **no** | shadow ~30 methods, *plus* every container builder |
| **proc-macro call-site rewrite** | partly | no | cannot reliably know a binding is a `Ui` |
| **`[patch.crates-io]` minimal fork** | **yes** | **yes** | ~120 lines, rebased per egui release |
| this crate | no (§13 item 1) | no (§13 item 2) | per-call-site adoption |

The `Deref` route is real and is often dismissed too fast: Rust resolves
inherent methods on the outer type *before* dereferencing, so an inherent
`button` on a wrapper genuinely does win over `egui::Ui::button` (`ui.rs:1847`),
and `Deref` passes the other ~140 methods through for free. It fails on the
*builder* boundary: `ScrollArea::show`, `Window::show`, `Grid::show` and
`CollapsingHeader::show` hand back a raw `&mut egui::Ui`, and they are methods
on egui's builders rather than on `Ui`, so the wrapper is lost at every
container unless every builder is wrapped too. It is also deref-as-inheritance,
where which method ran is invisible at the call site.

**The route that actually delivers the goal is `[patch.crates-io]` over a
minimal fork**, carrying exactly the `Style::class_overrides` change the
connector already designed and costed (`todo_v0.6.0_egui-connector-spec.md`
§14.2 — two behaviour-neutral commits, roughly 120 lines, in a module that is
byte-identical between egui 0.35.0 and 0.36.1). Applied that way it reaches
*everything* that reads the global `Style`, including third-party crates,
with no source change in the application.

That is deliberately **not** a task in §14, because it is not a task of this
crate: it is the connector's upstream contribution, published as a branch
instead of waiting on a merge. The two do not compete. If it lands upstream,
Tier 2 becomes redundant (§15 Q-4) and Tier 1 is untouched, because Tier 1
exists for widgets egui does not have at any styling fidelity.

### 4.3 The common builder shape

Every Tier 1 widget follows one shape, so learning one teaches all:

```rust
pub struct Switch<'a> { /* … */ }

impl<'a> Switch<'a> {
    pub fn new(on: &'a mut bool) -> Self;
    pub fn label(self, text: impl Into<egui::WidgetText>) -> Self;
    pub fn enabled(self, enabled: bool) -> Self;
    pub fn id_salt(self, salt: impl std::hash::Hash) -> Self;
}

impl egui::Widget for Switch<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response { /* … */ }
}
```

Rules that hold for every widget in the crate:

* `new` takes the state it mutates and nothing else. Never a theme, never a
  `Context`.
* Builder methods take `self` and return `Self`, matching egui.
* `enabled(false)` uses **the theme's disabled colours** (§9.3). This differs
  from `ui.add_enabled(false, w)`, which applies egui's blanket opacity
  multiply (`ui.rs:1587-1589` → `Painter::multiply_opacity`,
  `painter.rs:100-104`). Both are legal; the rustdoc on `enabled` states the
  difference so a caller picks deliberately.
* No widget takes a `Role`. The widget *is* the role.

### 4.4 Tier 2 surface

Tier 2 lives in `mod wrap` and is spelled as functions returning the egui
widget already wrapped, so the call site reads like egui's:

```rust
pub fn button<'a>(ui: &egui::Ui, text: impl Into<egui::WidgetText>) -> impl egui::Widget + 'a;
pub fn text_edit_singleline<'a>(ui: &egui::Ui, text: &'a mut String) -> impl egui::Widget + 'a;
pub fn checkbox<'a>(ui: &egui::Ui, checked: &'a mut bool, text: impl Into<egui::WidgetText>) -> impl egui::Widget + 'a;
pub fn slider<'a>(ui: &egui::Ui, value: &'a mut f64, range: std::ops::RangeInclusive<f64>) -> impl egui::Widget + 'a;
```

Each takes `&egui::Ui` solely to read the atlas and the active scheme; none
mutates it. **OPEN — Q-1 (§15):** whether `impl Widget` here should instead be
a named struct per wrapper, which is uglier but gives callers a nameable type.

---

## 5 -- Tier 1 inventory

Each row states the admission test's two conditions. UNMAPPABLE counts are the
connector's §5 matrices.

| widget | egui counterpart | native struct | UNMAPPABLE | admitted |
|---|---|---|---:|---|
| `Switch` | none | `ResolvedSwitchTheme` (13 leaves) | 8 of 13 | **yes** — both conditions |
| `Spinner` | `egui::Spinner` exists | `ResolvedSpinnerTheme` | 1 | **conditional**, see below |
| `TabBar` | none | `ResolvedTabTheme` (21) | 4 | **yes** on condition 1; condition 2 fails |
| `Toolbar` | none | `ResolvedToolbarTheme` (17) | 4 | **yes** on condition 1; condition 2 fails |
| `StatusBar` | none | `ResolvedStatusBarTheme` (14) | 3 | as above |
| `Sidebar` | none | `ResolvedSidebarTheme` (17) | 3 | as above |
| `Card` | `Frame` only | `ResolvedCardTheme` (9) | 2 | as above |
| `Expander` | `CollapsingHeader` exists | `ResolvedExpanderTheme` (17) | 4 | **conditional** |
| `SegmentedControl` | none | `ResolvedSegmentedControlTheme` (20) | 5 | **yes** on condition 1 |

**This table is the crate's central unresolved design question, and it is
deliberately not resolved here.** Only `Switch` passes both conditions
outright. Six widgets pass condition 1 (egui genuinely has no tab bar, toolbar,
status bar, sidebar, card or segmented control) but fail condition 2, because
the connector can already carry most of their leaves into an `egui::Frame` or a
scoped `Style`.

That is a real tension: their leaves are *mappable* but there is no widget to
map them onto, so an application must hand-assemble the widget from `Frame` and
`Button` and get the composition right itself. Whether "egui has no such
widget" should be sufficient on its own is **Q-2 (§15)**, and it must be
settled before any of the six is written.

`Spinner` and `Expander` are conditional for the opposite reason: egui *does*
ship a counterpart, so condition 1 fails, yet the counterpart hardcodes what
the theme wants to set — `Spinner`'s stroke width and point count
(`widgets/spinner.rs:45`, `:58`), `CollapsingHeader`'s arrow colour, which is
the same field as its label colour (`collapsing_header.rs:353` vs `:598`).
Admitting them requires a third condition, which §15 Q-2 must also decide.

---

## 6 -- Tier 2 inventory

| wrapper | egui widget | role | reaches |
|---|---|---|---|
| `wrap::button` | `Button` | `Role::Button` | full scope |
| `wrap::checkbox` | `Checkbox` | `Role::Checkbox` | full scope |
| `wrap::text_edit_singleline` | `TextEdit` | `Role::Input` | full scope + `TextEdit::frame` (`widgets/text_edit/builder.rs:306`) and `::margin` (`:313`) |
| `wrap::text_edit_multiline` | `TextEdit` | `Role::Input` | as above |
| `wrap::slider` | `Slider` | `Role::Slider` | full scope |
| `wrap::progress_bar` | `ProgressBar` | `Role::ProgressBar` | full scope |
| `wrap::link` | `Link` | `Role::Link` | partial — see §13 item 3 |
| `wrap::separator` | `Separator` | `Role::Separator` | partial — spacing is hardcoded `6.0` (`widget_style.rs:215`), overridable only per instance via `Separator::spacing` (`widgets/separator.rs:45`), which the wrapper sets |
| `wrap::combo_box` | `ComboBox` | `Role::ComboBox` | scope for the closed box + `ComboBox::popup_style` (`containers/combo_box.rs:199`) for the list |
| `wrap::scroll_area` | `ScrollArea` | `Role::Scrollbar` | full scope |

**`Area`-based containers are not Tier 2 wrappers.** `Window`, `Modal`,
`Popup`, `Tooltip` and menus ignore the calling `Ui`'s style entirely
(§2.2). They are served instead by documented helpers that pass what each
container actually accepts — `Window::frame` (`containers/window.rs:265`),
`Window::title_frame` (`:272`), `Modal::frame` (`containers/modal.rs:53`),
`Popup::style` (`containers/popup.rs:417`), `MenuConfig::style`
(`containers/menu.rs:107`), `ComboBox::popup_style` — plus the role scope
applied as the first statement *inside* the closure, which does work because
the closure receives a `Ui` descending from the `Area` content `Ui`
(`containers/window.rs:719-752`, `containers/modal.rs:104-108`, `ui.rs:236`,
`Ui::set_style` `ui.rs:386`). The residue that no such helper reaches is the
connector's ledger item 2b.

---

## 7 -- Fonts: weight and slant

The connector cannot express font weight, because `Style::text_styles` holds
`FontId { size, family }` and nothing else (ledger item 7). **This crate is not
subject to that limit**, because a Tier 1 widget lays out its own text and can
build a `LayoutJob` whose `TextFormat` carries far more:

`font_id`, `extra_letter_spacing`, **`line_height: Option<f32>`**, `color`,
`background`, `expand_bg`, **`coords: VariationCoords`**, **`italics`**,
`underline`, `strikethrough`, `valign`
(`epaint/src/text/text_layout_types.rs`).

### 7.1 The three routes, in order of preference

1. **Variable-font axis (preferred).** `TextFormat::coords` sets variation
   coordinates per text run, and `FontTweak::coords`
   (`epaint/src/text/fonts.rs:255`) sets a registered font's defaults;
   `FontVariationAxis` is `:186`. For a variable family, setting the `wght`
   axis gives the real designed weight at any value with one font file and no
   duplication. This is the only route that is both genuine and free.
2. **A real static face registered as a separate family.** Register
   `FontFamily::Name("Noto Sans Bold")` from the actual Bold file found on the
   system. Genuine, but needs font discovery (§13 item 5) and costs memory per
   face.
3. **Synthetic — prohibited except as a named fallback.** §7.3.

### 7.2 The registration invariant

> **Every `FontFamily::Name` this crate constructs must be proven registered
> before it is used.**

An unregistered name is a **panic**, not a fallback:
`panic!("FontFamily::{family:?} is not bound to any fonts")`
(`epaint/src/text/fonts.rs:1031`) and
`panic!("No font data found for {font_name:?}")` (`:1039`).

Given this project's absolute no-panic rule, the crate must never build a
`FontFamily::Name` from theme data and use it in the same expression. The
required shape is: consult the registry the connector's `mod fonts` populated,
and fall back to `FontFamily::Proportional` when the lookup misses.

**This is the crate's single largest panic risk** and §12 T3 exists to prove it
cannot occur.

### 7.3 Never synthesize

egui already offers synthetic slant: `TextFormat::italics` shears the glyph
quad by a fixed 25% of its height (`epaint/src/text/text_layout.rs:1173-1177`,
`top_offset = rect.height() * 0.25 * Vec2::X`). It is a fake oblique, not an
italic.

Algorithmic emboldening (stroke dilation) is **not** to be implemented. It
distorts stems, destroys hinting at UI sizes, and manufactures a typeface the
designer never drew — which is the same objection this project already records
against fabricated platform assets.

`TextFormat::italics` may be used **only** when routes 1 and 2 have both
failed, and its use must be reported through the connector's `Note` channel so
the degradation is observable rather than silent.

---

## 8 -- Accessibility

A hand-painted widget is invisible to a screen reader unless it says otherwise.
egui's own widget layer touches `WidgetInfo` in **22 files**; a Tier 1 widget
that omits it is not merely imperfect, it is inaccessible.

**Every Tier 1 widget must call `Response::widget_info`** (`response.rs:868`)
with the closest matching constructor: `WidgetInfo::labeled` (`lib.rs:658`),
`::selected` (`:668`), `::slider` (`:686`) or `::text_edit` (`:697`).

`WidgetType` (`lib.rs:623`) has exactly these variants:

```text
Label · Link · TextEdit · Button · Checkbox · RadioButton · RadioGroup
SelectableLabel · ComboBox · Slider · DragValue · ColorButton · Image
CollapsingHeader · Panel · ProgressIndicator · Window · ResizeHandle
ScrollBar · Other
```

**There is no `Switch` or `Toggle` variant.** The mapping this crate uses, and
the reason for each:

| our widget | reported as | why |
|---|---|---|
| `Switch` | `Checkbox` | a switch is semantically a two-state checkbox; `Other` would lose that |
| `SegmentedControl` | `RadioGroup` | exclusive selection among siblings |
| `TabBar` | `RadioGroup` | same semantics; a tab strip is an exclusive choice |
| `Expander` | `CollapsingHeader` | exact match |
| `Spinner` | `ProgressIndicator` | exact match |
| `Card`, `Toolbar`, `StatusBar`, `Sidebar` | `Panel` | containers, not controls |

Reporting `Other` is permitted only where no variant is defensible, and the
rustdoc must say why.

---

## 9 -- Interaction, focus and disabled state

### 9.1 Focus

egui has no focus-ring concept: keyboard focus promotes a widget to the
`active` visuals (connector ledger item 5). A Tier 1 widget escapes this — it
reads `Response::has_focus` (`response.rs:348`) and paints a real ring from the
theme's `focus_ring_color`, `focus_ring_width` and `focus_ring_offset`, which
the connector exposes as accessors precisely because it cannot write them.

**This is the clearest single win of the crate**: three leaves the connector
declares unreachable become reachable, and the result is an accessibility
improvement rather than a cosmetic one.

### 9.2 Interaction

Interaction comes from `Ui::allocate_response` (`ui.rs:1138`) with an explicit
`Sense`, or `Response::interact` (`response.rs:802`) where a sub-region needs
its own. No widget invents hit-testing.

### 9.3 Disabled

egui models disabled as a single opacity multiply — `Visuals::disabled_alpha`
applied via `Ui::disable` → `Painter::multiply_opacity` (`ui.rs:496-501`,
`painter.rs:100-104`) — with no disabled *colour* anywhere (ledger item 6).

A Tier 1 widget with `.enabled(false)` must **not** call `Ui::disable`. It
reads `Ui::is_enabled` (`ui.rs:470`) for the inherited state, combines it with
its own flag, and paints the theme's `disabled_background` and
`disabled_text_color` directly. It must still allocate the same space and still
emit `WidgetInfo` with `enabled: false`.

### 9.4 Animation

Where a widget animates (the switch thumb is the obvious case), it uses
`Context::animate_bool_with_time` (`context.rs:3211`).

**Reduced motion is mandatory**: when the atlas's `AccessibilityPreferences`
reports reduced motion, the animation time passed must be `0.0`. A widget that
animates unconditionally is a bug, not a preference.

---

## 10 -- Painting rules

### 10.1 Order

Background → border → content → focus ring. The ring is painted last and
outside the widget rect by `focus_ring_offset`, so it is never clipped by the
widget's own background.

### 10.2 Conversion

All `f32` → `epaint` integer conversions go through the connector's
`mod convert`, which is already specified as total and saturating
(`todo_v0.6.0_egui-connector-spec.md` §7). This crate adds no conversion
helpers and must not open-code an `as` cast.

### 10.3 The permitted constants

The complete list of numeric literals allowed in painting code. Anything else
is a bug:

| constant | value | why it is structural, not thematic |
|---|---|---|
| half | `0.5` | centring and radius-from-diameter |
| full circle | `std::f32::consts::TAU` | arc sweeps in `Spinner` |
| identity | `0.0`, `1.0` | offsets and opacity identity |

**OPEN — Q-3 (§15):** the switch thumb inset and the tab-bar underline
thickness have no `ResolvedTheme` source today. Either native-theme grows the
fields, or the widgets derive them by a documented formula from a field that
does exist. They must not become a fourth constant.

---

## 11 -- Cargo.toml, features, MSRV

```toml
[package]
name = "native-theme-egui-widgets"
edition = "2024"
# NOT `rust-version.workspace = true` — egui needs more than the 1.88.0
# workspace floor, exactly as the connector does.
rust-version = "1.95"

[dependencies]
egui = { version = "0.36.1", default-features = false }
native-theme-egui = { version = "…", path = "../native-theme-egui" }

[features]
default = []
```

`default = []` matches the connector. This crate adds **no** feature of its
own: features here would multiply against the connector's, and every widget is
useful in every configuration.

**Version policy** is inherited verbatim from the connector's §12.3 — this
crate tracks the same single egui minor and never claims a range. It is
strictly downstream, so it can never be compatible with an egui version the
connector is not.

---

## 12 -- Testing

Headless, no window, no GPU — the same discipline as the connector's §13.

| # | group | proves |
|---|---|---|
| T1 | **No-atlas fallback** | every widget renders with no atlas installed, returns a `Response`, and emits no `Note` |
| T2 | **Determinism** | building the same widget twice against the same atlas produces byte-identical shapes |
| T3 | **Font-name safety** | no code path constructs a `FontFamily::Name` that is not present in the registry — this is what makes §7.2 mechanical rather than a review promise |
| T4 | **Hostile theme** | `NaN`, `±∞`, `-0.0` and extreme magnitudes in every `f32` leaf; every widget still allocates finite space and paints |
| T5 | **Accessibility coverage** | every Tier 1 widget emits a `WidgetInfo`, and its `WidgetType` matches the §8 table |
| T6 | **Reduced motion** | with reduced motion set, every animating widget passes `0.0` animation time |
| T7 | **Disabled colour** | `.enabled(false)` paints the theme's disabled colours and does **not** call `Ui::disable` |
| T8 | **No hardcoded values** | a source scan for numeric literals in painting code, allowing only §10.3 |
| T9 | **Tier 2 scope reach** | a wrapper's widget observes the role style, verified by reading back the style inside the scope |

T8 is a lint-shaped test rather than a behavioural one, and it is the direct
mechanical enforcement of this project's no-hardcoded-values rule.

---

## 13 -- Limits: what this crate does NOT fix

The honesty ledger. Nothing here is softened.

| # | what is lost | evidence |
|---|---|---|
| 1 | **Existing code does not benefit.** Every call site must change to `ui.add(…)`. A user who adds the crate and changes nothing sees no difference | §4.2; inherent-method resolution makes the alternative unsound, not merely ugly |
| 2 | **Third-party egui crates are unaffected.** `egui_plot`, `egui_extras`, `egui_dock` and friends read the global `Style` and will render with the connector's base style only — the 31 DIRECT leaves plus elected winners | they call egui's widgets directly; nothing in this crate is in their path |
| 3 | **Link state colours stay lost in Tier 2.** `Link` reads `visuals.hyperlink_color` unconditionally (`widgets/hyperlink.rs:47`) and egui has no visited-URL set | connector ledger item 14. A Tier 1 `Link` could fix it by tracking visited URLs in `ctx.data_mut()`; it is not admitted because it fails the §2.4 test |
| 4 | **Glyph rasterization still is not the platform's.** epaint computes one coverage value per pixel, so text is grayscale-antialiased; LCD subpixel rendering is not available at any layer | `alpha = coverage^gamma` (`epaint/src/image.rs:378-381`). Hinting and shaping *do* match closely — skrifa and harfrust |
| 5 | **Font discovery is not solved here.** Route 2 of §7.1 needs a real Bold file located on disk, and egui has no font database | connector ledger item 8. This crate consumes whatever the connector's `mod fonts` registered and adds no discovery of its own |
| 6 | **Compositor-level appearance is out of reach** — real window shadows, client-side decorations matching the window manager, corner rounding, blur-behind | eframe presents a single surface |
| 7 | **Platform text interaction is egui's** — IME candidate placement, platform-specific text navigation, native context menus | Tier 3 forbids touching `TextEdit`, so these stay exactly as egui implements them |
| 8 | **Native dialogs are not provided** — file pickers, colour pickers | OS dialogs; an application should use `rfd` or a portal |

---

## 14 -- Implementation task list

Executable in order. Nothing here may start before the connector is
implemented, because every task consumes its API.

**Phase A — foundation**

1. Create the crate with the `Cargo.toml` of §11; add the workspace member.
   Verify `cargo tree -d` shows one `egui` and one `ecolor`.
2. Write `src/lib.rs` with the crate attributes of §4.1.
3. Implement atlas acquisition and the no-atlas fallback path (§3.1). Add T1
   immediately — the fallback is the property everything else rests on.
4. Implement the shared painting helpers: focus ring (§9.1), disabled
   resolution (§9.3), the `WidgetInfo` emission helper (§8). Add T5 and T7.

**Phase B — the first widget, end to end**

5. Implement `Switch` completely: builder, painting, animation with reduced
   motion, `WidgetInfo` as `Checkbox`, focus ring, disabled colours.
   It is the only widget that passes §2.4 outright, so it is the reference
   implementation every later widget is reviewed against. Add T6.
6. Add T2, T4 and T8 against `Switch` alone before writing a second widget.

**Phase C — Tier 2**

7. Implement `mod wrap` for the ten entries of §6. Add T9.
8. Implement the `Area`-container helpers of §6.

**Phase D — remaining Tier 1**

9. **Settle Q-2 (§15) before writing any of these.** Then implement the
   admitted subset in this order: `Spinner`, `Expander`, `SegmentedControl`,
   `TabBar`, `Card`, `Toolbar`, `StatusBar`, `Sidebar`.
10. Implement font weight and slant per §7, and add T3 — the panic guard.

**Phase E — release readiness**

11. Run `./pre-release-check.sh`.
12. Write `README.md` opening with §0.1's sentence verbatim, and stating §13
    items 1 and 2 in the first section — a reader must learn what this crate
    does not change before they adopt it.
13. Write `examples/showcase-egui-widgets.rs` covering every widget in both
    colour schemes.

---

## 15 -- Open questions

**Q-1 — `impl Widget` or named structs for Tier 2?** §4.4 specifies
`impl egui::Widget` returns, which keeps the surface small but gives callers no
nameable type and makes the return opaque in rustdoc.
*Recommendation:* keep `impl Widget` until a concrete need for the name
appears; it is the smaller commitment and can be widened compatibly.

**Q-2 — Is "egui has no such widget" sufficient for Tier 1 admission?**
The §2.4 test requires *both* conditions, and only `Switch` passes. Six
container-shaped widgets pass condition 1 and fail condition 2. A third
condition would also be needed for `Spinner` and `Expander`, where egui ships a
counterpart that hardcodes what the theme wants to set.
*Recommendation:* replace condition 2 with "**either** a majority of the
struct's leaves are UNMAPPABLE, **or** egui ships no widget of that identity
and assembling one correctly from `Frame` and `Button` is more than roughly
forty lines." That admits the six containers on their real merit — composition
burden — rather than by stretching an UNMAPPABLE count that was never about
them. **This blocks Phase D and nothing else.**

**Q-3 — Where do the switch thumb inset and tab underline thickness come
from?** §10.3 forbids inventing them.
*Recommendation:* derive from existing fields by documented formula
(thumb inset from `switch.track_height` and `switch.thumb_diameter`, both of
which exist), and only add native-theme fields if no defensible derivation
exists. Adding fields to `ResolvedTheme` for one toolkit's widget crate is the
wrong direction.

**Q-4 — Does this crate survive the upstream `class_overrides` PR?**
If the connector's §14.2 contribution lands, Tier 2 becomes unnecessary —
`SCOPED` collapses into `DIRECT` and plain egui widgets render natively without
a wrapper.
*Recommendation:* proceed anyway. Tier 1 is unaffected, because it covers
widgets egui does not have at all, and Tier 2 wrappers would then simply become
thin pass-throughs that can be deprecated without breaking callers. The PR is
explicitly **UNVERIFIED** as to upstream appetite, so making this crate wait on
it would be waiting on something nobody has agreed to.
