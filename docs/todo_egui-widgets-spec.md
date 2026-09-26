# native-theme-egui-widgets — Specification

Target: **egui 0.36.2** and the native-theme **v0.5.9** model. Version
milestone: **undecided** — this document does not claim one; which release ships
the crate is a scheduling choice with no bearing on the look it draws. Rationale:
[`todo_egui-widgets-rationale.md`](todo_egui-widgets-rationale.md).

**Re-verify against the connector as built when this crate is scheduled.** Every
connector item below is named, never numbered, because the connector's API is
settled only when it is implemented.

Every code citation below resolves against egui 0.36.2 / epaint 0.36.2 as
published. Paths are given the way the sibling documents give them:
`ui.rs:1521` means `egui/src/ui.rs` line 1521; `epaint/src/...` is spelled out.

---

## 0 -- Scope

### 0.1 What this crate is

> A companion widget crate for [`native-theme-egui`](todo_v0.6.0_egui-connector-spec.md).
> It draws what the connector cannot give egui's own widgets: a switch, a
> slider whose knob is not its rail, a spinner at the theme's stroke, and a
> segmented control, which egui does not have. It also wraps egui's links in
> their state and visited colours, the per-call route the connector names for
> them, written once (§4.5).

Everything it draws is driven by `ResolvedTheme` values obtained from the
connector's `ThemeAtlas`. It contains no colour, no radius and no metric of its
own beyond the structural constants of §2.3.

Where the connector already delivers a native look — a role scope around egui's
own widget, a `Surface` frame around a container, an accessor handed to one
builder call — this crate adds nothing: an application writes that one call
itself. §4.6 lists those cases.

### 0.2 What this crate is not

* **Not a drop-in replacement for egui.** It does not shadow, wrap or replace
  `egui::Ui`: existing calls keep working unchanged, and only a call site that
  adopts one of its widgets changes (§3.2, §7 item 1).
* **Not a fork.** It depends on published egui; it does not vendor or patch it.
* **Not a theme mapper.** The `ResolvedTheme` → `egui::Style` mapping lives in
  the connector and is not duplicated here.
* **Not a replacement for `TextEdit`, `ScrollArea` or `ComboBox`** (§1.4).

### 0.3 What it takes from the connector

The dependency is one-directional and the connector never learns this crate
exists. This crate uses these connector items, by name:

| item | used for |
|---|---|
| `ThemeAtlas::from_ctx` | obtaining the atlas without the application passing it; `None` when nothing is installed (§2.1) |
| `ThemeAtlas::resolved_for` | the `ResolvedTheme` of the scheme being drawn, the leaves every widget here paints from |
| `ThemeAtlas::accessibility` | reduced motion for the `Spinner`'s painted arc and its indicator's animation (§4.3) |
| `ThemeAtlas::icon_set`, `ThemeAtlas::icon_theme` | the icon set, and the freedesktop theme of the scheme being drawn, the `Spinner`'s indicator is loaded from (§4.3) |
| `icons::to_image_source`, `icons::animated_frame_index`, `icons::spin_angle`, `IconKey` | drawing and animating that indicator (§4.3) |
| `native_scope` with a `Role` and a `RoleVariant` | the scope every widget here runs in (§2.4) |
| `register_focus_shape` | the focus ring's outline for the `Switch` track (§2.5) |
| `convert::{to_color32, composite_over, finite_or, clamp_length, to_corner_radius, to_stroke, u8_from_f32_saturating}` | every `f32` → epaint conversion (§2.3) |
| `Role`, `RoleVariant` | naming what `native_scope` takes |
| the re-exported `egui` and `native_theme` | this crate's only route to both (§5) |

The focus ring itself is the connector's install plugin's, which rings every
focused widget with the radius of the role scope it sits in; this crate paints
no ring (§2.5).

---

## 1 -- The tier model

Every widget belongs to exactly one tier, recorded in its rustdoc. **The tier
says what this crate owns, and therefore what it must re-audit every egui
release.** The tiers are ordered by that liability, cheapest first.

A widget can be **ours** — our name, our API, our layout and role choices —
while egui still does all the painting, all the interaction and all the
accessibility. "Ours" does not mean "hand-painted".

### 1.1 Tier W — Wrapped

One egui widget, run inside a role scope, with the per-instance builder calls
or text formatting the theme needs.
**We own:** the role and variant choice and those per-instance calls.
**We own none of:** painting, interaction, accessibility, animation.

### 1.2 Tier C — Composed

Several egui widgets arranged by us into one API, with any non-interactive
decoration between them.
**We own:** the composition — which widgets, in what layout, under which roles,
with which per-instance calls — and the group's accessibility role.
**We own none of:** the interaction or accessibility of the composed parts,
because every interactive element is a real egui widget emitting its own
`WidgetInfo`.

The constraint that keeps this tier cheap, and it is absolute: **every
interactive element must be an egui widget.** The moment a composed widget
needs to sense its own clicks, it is Tier P and must be justified as one.

### 1.3 Tier P — Painted

We allocate, sense and paint ourselves: `Ui::allocate_response` (`ui.rs:1139`),
`Ui::allocate_exact_size` (`ui.rs:1151`) or an `AtomLayout`, then
`Ui::painter` (`ui.rs:458`).
**We own:** geometry, every painted pixel, interaction semantics, the
`WidgetInfo` and AccessKit role of §2.7, animation and disabled rendering.
This is the expensive tier, admitted only with the citation §1.5 demands.

### 1.4 Tier F — Forbidden

Reimplementing the internals of `TextEdit`, `ScrollArea` or `ComboBox` is
**out of scope permanently**, not deferred:
`containers/scroll_area.rs` is 1641 lines and `widgets/text_edit/builder.rs`
1457, carrying cursor movement, selection, undo, clipboard, IME and
bidirectional text — behaviour this crate has no opinion about, in exchange
for styling the connector's role scope already delivers.

### 1.5 The admission rule

> **Use the cheapest tier that produces the appearance the theme specifies. A
> widget may be promoted to a more expensive tier only by citing the upstream
> line that makes the cheaper tier insufficient.**

It replaces the connector charter's two-condition test (the connector
specification's *no-widgets charter*); the rationale's §4 says why, and why
replacing it is not charter evasion. It measures cost, it is mechanically
checkable — a promotion without a resolvable `file.rs:N` citation fails review
— and it has no threshold to re-tune.

Every Tier P entry in §4 carries its citation. If a future egui release makes a
cheaper tier sufficient — `Spinner` gaining a stroke-width builder, `Slider` a
handle fill of its own — the citation stops describing a hardcoded value and
the widget must be demoted. **Demotion is not optional**; T7 catches it.

---

## 2 -- Rules every widget follows

### 2.1 The atlas, and no atlas

Every widget resolves its theme through `ThemeAtlas::from_ctx(ui.ctx())` and
`resolved_for(ui.ctx().theme())` (`context.rs:2158`): the atlas holds both
schemes and egui decides which one applies. No widget takes a `&ThemeAtlas` or
a `&ResolvedTheme` parameter.

**If no atlas is installed, every widget renders as egui's own counterpart:**
`Switch` adds `egui::Checkbox`, egui's two-state control; `Slider` adds
`egui::Slider`; `Spinner` adds `egui::Spinner`; `SegmentedControl` adds its
buttons unscoped; the link wrappers add egui's `Link` and `Hyperlink`
unchanged. It never panics, never draws nothing and never emits a fabricated
colour, so a user who adds this crate before calling `install()` sees ordinary
egui. A Tier P widget also takes that fallback when a size its geometry is
built from is not finite: there is no egui sink for a switch track or a
painted knob, so the theme's geometry is either drawn as stated or not at all,
never with a number no source gives.

### 2.2 Sizes the theme leaves unstated

A size the theme does not state keeps the value egui itself uses there — the
scope's `Style` or egui's own widget's builder default — and the rationale's
§3.8 says why. Where a widget below reads such a value, it names the egui line
it takes it from.

### 2.3 No hardcoded values; conversion

Every colour, length, radius, width and gap a widget paints comes from a
`ResolvedTheme` leaf or from egui's own value for an unstated one (§2.2). The
only other numeric literals allowed in painting code:

| constant | value | why it is structural, not thematic |
|---|---|---|
| half | `0.5` | centring and radius-from-diameter |
| full circle | `std::f32::consts::TAU` | the `Spinner`'s rotation |
| identity | `0.0`, `1.0` | offsets, the animation's ends, opacity identity |
| egui's own spinner constants | `240°`, `8`, `128` | cited to `widgets/spinner.rs` at their use (§4.3) |
| egui's own slider key step | `1.0` point per press | cited to `widgets/slider.rs:722` (§4.2) |

A numeric literal in painting code that is not in this table is a bug, and T6
catches it.

All `f32` → epaint conversions go through the connector's `mod convert`, and
this crate open-codes no `as` cast. Every length handed to egui passes
`convert::clamp_length`; a non-finite theme value becomes, through
`convert::finite_or`, egui's own value for the same sink; where egui has no
such sink, §2.1's fallback applies.

### 2.4 Scope, state layers and disabled

Every widget runs inside `native_scope` with its own `Role` and the variant its
state selects: `Disabled` for a widget built with `.enabled(false)`, `Normal`
otherwise. A selected look is not a variant here: the segmented control's
active segment is a `Button::selected(true)` inside the row's `Normal` scope
(§4.4).

**Hover is a layer.** A Tier P widget paints a hover colour composited over its
idle fill with `convert::composite_over`, never in place of it (rule C17,
`archive/todo_v0.5.9_theme-contracts-spec.md` §3.2): the platforms layer these
colours, and an opaque layer composites to itself. A `None` hover or disabled
colour is filled by copying the base colour it covers, never by arithmetic
(C16). A colour is never interpolated between two states: a stated colour is
painted, or the other stated colour is.

**Disabled is the `Disabled` scope plus `ui.disable()`.** `.enabled(false)`
opens the widget's `Disabled` variant scope and calls `Ui::disable` inside it
(`ui.rs:497-502`). `Ui::disable` gives the widget egui's own disabled semantics
— hit testing strips its click and drag sense (`hit_test.rs:131-139`), it
surrenders focus (`context.rs:1256`, `:1274-1276`), and its `WidgetInfo`
reports `enabled: false` from `Ui::is_enabled` (`ui.rs:471`) — and
multiplies the painter's opacity by the scope's `disabled_alpha`, which the
connector's `Disabled` cell sets to `1.0` for every role that has disabled
colours, so the widget's own disabled colours are painted unfaded. A Tier P
widget paints its role's disabled leaves whenever `Ui::is_enabled` is false;
a `None` copies the base colour (C16), and the role's
`disabled_opacity` is not multiplied on top, since the stated disabled colours
already carry the platform's fade.

Disablement *inherited* from an enclosing `Ui` — `ui.add_enabled(false, w)` or
an application's own `ui.disable()` — arrives already faded: that call
multiplied the painter every child clones (`ui.rs:225`) by the enclosing
style's `disabled_alpha` (`ui.rs:1588-1596`), and this crate does not undo an
opacity the application set. The widget then paints its disabled colours under
that fade. `.enabled(false)` is the spelling that shows the platform's disabled
colours unfaded, and each widget's rustdoc says so.

### 2.5 Focus

egui has no focus-ring concept; the connector's install plugin paints one
ring, after every widget of the pass, around the widget holding keyboard focus,
at the corner radius of the role scope the widget sits in. So **no widget here
paints a ring**, and every widget here gets the right radius from its scope.
What an interactive Tier P widget — `Switch`, `Slider` — owes the ring is to
be focusable: it senses with `Sense::click()` or `Sense::drag()`, both
`FOCUSABLE` (`sense.rs:60-61`, `:68-69`), so `Response::has_focus`
(`response.rs:349`) is true for it. The `Spinner` senses `Sense::hover()`, as
egui's does (`widgets/spinner.rs:68`), and takes no focus.

The one outline that is not the response rect at the scope's radius is the
`Switch`'s: its focusable part is the track, so it calls
`register_focus_shape` with the track's rect and `switch.track_radius` in the
pass it paints it.

### 2.6 Animation and reduced motion

The `Switch` thumb moves with `Context::animate_bool_with_time`
(`context.rs:3214`) over the scope's `Style::animation_time`, which the
connector sets to `0.0` in every style under reduced motion. A `0.0` time is
safe: a bool animation's step divides by the time, and a non-finite step snaps
to the target (`animation_manager.rs:56-61`). The `Spinner` honours reduced
motion itself (§4.3).

### 2.7 Accessibility

A hand-painted widget is invisible to a screen reader unless it says
otherwise. **Every Tier P widget calls `Response::widget_info`**
(`response.rs:869`) with the closest constructor —
`WidgetInfo::selected` (`data/output.rs:668`) or `::slider` (`:686`). egui
turns the `WidgetType` into an AccessKit role (`response.rs:935-957`), and
where AccessKit has a better role than `WidgetType` can say, the widget
overwrites it through `Context::accesskit_node_builder` (`context.rs:3684`)
after `widget_info` has filled the node, synchronously (`response.rs:869-895`).
That closure runs with the `Context` lock held (`context.rs:3680`), so it
touches only the node, and it does not run while AccessKit is off (`:3682`).
AccessKit 0.24.1 is a non-optional egui 0.36.2 dependency
(`egui/Cargo.toml:82-83`), re-exported as `egui::accesskit`
(`egui/src/lib.rs:434`).

| widget | `WidgetType` | AccessKit | why |
|---|---|---|---|
| `Switch` | `Checkbox` | `Role::Switch` (accesskit 0.24.1 `src/lib.rs`, line 101), toggled | a switch is a two-state control, which `WidgetType` can say only as a checkbox |
| `Slider` | `Slider` | egui's `Role::Slider` | exact match |
| `Spinner` | `ProgressIndicator` | egui's `Role::ProgressIndicator` | exact match, as egui's own spinner reports (`widgets/spinner.rs:69`) |
| `SegmentedControl` | — (its buttons' own) | `Role::RadioGroup` on the row's content `Ui`; `Role::RadioButton` on each segment, whose toggled state `Button::selected` reports (`response.rs:975-980`) | an exclusive choice among siblings; egui's own radio button reports its state through `toggled` the same way (`widgets/radio_button.rs:71`) |
| `wrap::hyperlink` | `Link` | egui's `Role::Link`, `visited` set once visited | §4.5 |

A row's node is its content `Ui`'s, keyed by `Ui::unique_id` (`ui.rs:357`),
which egui creates as a `GenericContainer` (`ui.rs:314-318`).

---

## 3 -- Public API

### 3.1 Crate root

```rust
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

pub mod switch;
pub mod slider;
pub mod spinner;
pub mod segmented_control;
pub mod wrap;   // Tier W: `link`, `hyperlink`

pub use native_theme_egui as connector;
```

The attributes are the connector's own, so both crates refuse the same panic
paths. The crate re-exports the connector under one name rather than its types
one by one, so the connector's rule about names that collide with egui's root
has one owner.

### 3.2 Widgets are `egui::Widget`, never `Ui` methods

```rust
ui.add(native_theme_egui_widgets::switch::Switch::new(&mut wifi_on).label("Wi-Fi"));
```

`egui::Widget` is `egui/src/widgets/mod.rs:63` and `Ui::add` is `ui.rs:1521`.
`Ui::button` (`ui.rs:1848`) and its siblings are *inherent* methods on
`egui::Ui`, and Rust resolves inherent methods before trait methods, so an
extension trait offering a same-named method would be silently ignored at
every call site — wrong pixels, no diagnostic. Offering no competing name makes
that failure impossible, and `ui.add_sized` (`ui.rs:1538`) and `ui.add_enabled`
(`ui.rs:1588`) accept our widgets for free. **No extension trait on `Ui` is
provided, and none may be added later**: a second spelling for every widget
leaves a permanent question at each call site about which is current.

### 3.3 The builder shape

```rust
pub struct Switch<'a> { /* … */ }

impl<'a> Switch<'a> {
    pub fn new(on: &'a mut bool) -> Self;
    pub fn label(self, text: impl Into<egui::WidgetText>) -> Self;
    pub fn enabled(self, enabled: bool) -> Self;
}

impl egui::Widget for Switch<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response { /* … */ }
}
```

* `new` takes the state it mutates and nothing else — never a theme, never a
  `Context`.
* Builder methods take `self` and return `Self`, matching egui.
* No widget takes a `Role`. The widget *is* the role.

| widget | `new` | builders |
|---|---|---|
| `switch::Switch` | `new(on: &mut bool)` | `label`, `enabled` |
| `slider::Slider` | `new(value: &mut f64, range: RangeInclusive<f64>)` | `label`, `enabled` |
| `spinner::Spinner` | `new()` | — |
| `segmented_control::SegmentedControl` | `new(selected: &mut usize, segments: impl IntoIterator<Item = impl Into<egui::WidgetText>>)` | — |
| `wrap::link`, `wrap::hyperlink` | functions, §4.5 | — |

`SegmentedControl` returns the row's `Response`, marked changed
(`response.rs:625`) when a click moved the selection. It has no `enabled`: its
role has no disabled colour, so `ui.add_enabled` — egui's fade at the calling
`Ui`'s `disabled_alpha` (`ui.rs:1588-1596`) — is its disabled appearance;
`segmented_control.disabled_opacity` inherits `defaults.disabled_opacity`, the
base style's value.

---

## 4 -- The widgets

### 4.1 `Switch` — Tier P

**Why Tier P.** `egui/src/widgets/` contains no switch or toggle module — the
listing is `button`, `checkbox`, `color_picker`, `drag_value`, `hyperlink`,
`image`, `label`, `progress_bar`, `radio_button`, `separator`, `slider`,
`spinner`, `text_edit`. The connector's substitute,
`Button::new(..).selected(checked)` (`widgets/button.rs:270`), shows the track
colours as a button's fill and no thumb.

**Shape.** Built as egui's own `Checkbox` is: an `AtomLayout` whose first atom
is a custom one the size of the track (`widgets/checkbox.rs:92`,
`atomics/atom.rs:97`), the label after it, sensing `Sense::click()`
(`widgets/checkbox.rs:96-100`); the atom's rect comes back from the painted
layout (`:125-127`, `atomics/atom_layout.rs:722`). The gap between track and
label is therefore egui's own `Spacing::icon_spacing`
(`atomics/atom_layout.rs:302`), which the model does not state. A click flips
the value and marks the response changed (`widgets/checkbox.rs:103-104`).

**Geometry**, all from `ResolvedTheme::switch`:

* track: `track_width` × `track_height`, corner radius `track_radius`;
* thumb: a circle of `thumb_diameter`, centred on the track's axis and inset
  `0.5 * (track_height − thumb_diameter)` from the track's ends — the centred
  thumb the two platform numbers describe (rationale §7 Q-3). A thumb larger than its
  track overhangs it, as those numbers state;
* the thumb's position is the animation value of §2.6 between its two ends.

**Colours.** Track: `checked_background` or `unchecked_background` by the
value; while hovered, `hover_checked_background` or
`hover_unchecked_background` composited over it (§2.4). Thumb:
`thumb_background`. Disabled (§2.4): `disabled_checked_background`,
`disabled_unchecked_background`, `disabled_thumb_color`, and the label in
`defaults.disabled_text_color` through `AtomLayout`'s fallback text colour
(`widgets/checkbox.rs:124`), since `SwitchTheme` has no text colour and the
switch role's `Disabled` cell writes none.

**Accessibility and focus:** §2.7, §2.5.

### 4.2 `Slider` — Tier P

**Why Tier P.** egui paints the rail in `widgets.inactive.bg_fill`
(`widgets/slider.rs:774-775`) and the resting knob in the fill of the widget's
interaction state (`widgets/slider.rs:766`, `:813-818`), which at rest is the same
`inactive.bg_fill`. So at rest knob and rail are one colour in any `Style`,
while every platform preset states a knob colour distinct from the rail:
`slider.thumb_color` ← `defaults.surface_color` and `slider.track_color` ←
`defaults.muted_color` (`docs/inheritance-rules.toml:196-197`) resolve to
different colours in both modes of `kde-breeze`, `adwaita`, `macos-sonoma` and
`windows-11` (measured on the resolved presets, 2026-09-25).

**Geometry.** The rail is `track_height` tall across the widget; its corner
radius, which the model does not state, is egui's
`widgets.inactive.corner_radius` (`widgets/slider.rs:772`). The trailing fill
runs from the rail's start to the knob's centre. The knob is a circle of
`thumb_diameter`, its centre travelling the width less one knob radius at each
end, as egui's `position_range` does (`widgets/slider.rs:853-865`). The width is
egui's `Spacing::slider_width` (`style.rs:412`), and the height the larger of
`thumb_diameter` and `track_height`. The knob's outline is egui's, the
interaction state's `fg_stroke` (`widgets/slider.rs:817`): the model states
none. No tick marks are painted and no value field is shown; an application
that wants the number adds a `DragValue`.

**Colours.** Rail `track_color`, trailing fill `fill_color`, knob
`thumb_color`, with `thumb_hover_color` composited over it while hovered.
Disabled: `disabled_track_color`, `disabled_fill_color`,
`disabled_thumb_color` (§2.4).

**Interaction**, egui's own semantics on a linear range: the widget senses
`Sense::drag()` as egui's does (`widgets/slider.rs:655`); a pointer drag sets
the value from the pointer's position (`:666-678`); while focused, the arrow
keys along the rail move it by egui's step of one point per press (`:722`) under
egui's focus-lock filter, so those arrows do not move focus (`:684-698`); and
AccessKit's `Increment`, `Decrement` and `SetValue` requests act as in egui
(`:713-717`, `:753-760`). The value is clamped to the range. `label` is shown
after the rail and names the slider in `WidgetInfo::slider` (`:967`).

### 4.3 `Spinner` — Tier P

**The icon set's indicator first.** Where the application's icon set supplies
an animated indicator, `Spinner` draws that one, as both sibling showcases do
(`connectors/native-theme-gpui/examples/showcase-gpui/app.rs:504-514`,
`connectors/native-theme-iced/examples/showcase-iced.rs:760-763`):
`native_theme::icons::load_icon_indicator(set)` (`native-theme/src/icons.rs:506`),
and for a freedesktop set `FreedesktopLoader::load_indicator(icon_theme)`
(`:246`), with the set and theme from `ThemeAtlas::icon_set` and
`ThemeAtlas::icon_theme(ui.ctx().theme())`, so the spinner comes from the theme
the icons of that colour scheme come from. It is drawn at `spinner.diameter`
through `icons::to_image_source`, its `IconKey` tinted `spinner.fill_color` —
the arc's colour below — for the monochrome bundled sets (Material, Lucide),
whose SVGs otherwise draw black (the connector's `IconKey::tint`); what it
hands `to_image_source` is `frames[i]`, the `IconData` at the index `i` that
`icons::animated_frame_index` returns, under the URI that frame's bytes hash
to; it is animated
by `icons::animated_frame_index` or
`icons::spin_angle` with `reduced_motion` the atlas's
`accessibility().reduce_motion`; under reduced motion they return `None` and
the first frame is drawn still, as both sibling showcases
show it (`connectors/native-theme-gpui/examples/showcase-gpui/app.rs:553-554`,
`connectors/native-theme-iced/examples/showcase-iced.rs:765`).

**Why Tier P.** Where the set supplies none — `SfSymbols` and `SegoeIcons`
(`native-theme/src/icons.rs:511`), or its icon feature is off — it paints an
arc, because `egui::Spinner` exposes only `.size()` (`widgets/spinner.rs:25`)
and `.color()` (`:32`): its stroke is hardcoded `Stroke::new(3.0, color)`
(`:58`) and its radius inset a literal `- 2.0` (`:45`), so
`spinner.stroke_width` cannot reach it at any cheaper tier.

**The painted arc.** Size `spinner.diameter`, colour `spinner.fill_color`,
stroke `spinner.stroke_width`. The model gives the diameter as the ring's outer
size (`native-theme/src/model/widgets/mod.rs:701`), as platform-facts' outer-box
rule measures every element (`docs/platform-facts.md:897-902`), and a `Stroke`
becomes a `PathStroke` centred on its path (`epaint/src/stroke.rs:212-222`,
`StrokeKind::Middle` at `:106-107`), so the path's radius is
`0.5 * (diameter − stroke_width)`. The motion is egui's own, with egui's
constants cited as egui's values: the arc starts at `time · TAU` and sweeps
`240°·sin(time)`, `time` being the input's seconds (`widgets/spinner.rs:47-49`), in
`round(radius)` points clamped to `8..=128` (`:46`, counted here with
`convert::u8_from_f32_saturating` instead of an `as` cast), repainting every
pass it is visible (`:39-40`). Under reduced motion
(`ThemeAtlas::accessibility`) it paints that motion's widest sweep, `240°`,
still, from angle `0` (§2.3's identity), since egui's frame at time `0` sweeps
nothing.

`spinner.min_diameter` is not read: `Spinner` offers no size of its own for it
to bound. On macOS the platform draws radiating fins, not a ring
(`docs/platform-facts.md:1535-1538`), and the model states no fin geometry, so
the arc is what this crate draws there (§7 item 4).

### 4.4 `SegmentedControl` — Tier C

**Composition.** One horizontal row of `Button::new(label).selected(i ==
*selected)` (`widgets/button.rs:270`) in one `Role::SegmentedControl` scope for
the whole row. Its `Normal` cell carries `segmented_control.active_background`
and `active_text_color` in `selection.*`, which egui paints only on the
selected button (`widget_style.rs:150-155`), so no segment opens a scope of its
own; the scope also carries `segment_height` and the segments' padding. Not `Button::selectable`,
whose unselected button paints no frame at rest (`widgets/button.rs:78-83`) and
would lose `segmented_control.background_color`. A click on a segment sets
`*selected` to its index.

**The divider.** `separator_width` is "the width of the divider line between
segments" (`docs/platform-facts.md:987`), so the segments sit
`separator_width` apart. The connector's `Role::SegmentedControl` cell carries
that gap as `spacing.item_spacing.x`, through `finite_or` and `clamp_length`,
so the horizontal row laid out in that scope has it with no work here; what this
widget adds is the composition above and the radio-group semantics (§2.7).

**What is not drawn**, because no source states it:

* **the divider's line.** No leaf states its colour —
  `SegmentedControlTheme` has one line colour, its border's
  (`native-theme/src/model/widgets/mod.rs:773-803`) — so nothing is painted in
  the gap; what lies behind the row shows through it;
* **a joined outline.** Nothing states whether a segment's corners that face a
  divider are rounded, so every segment keeps the scope's radius,
  `segmented_control.border.corner_radius`, on all four corners, and the
  control reads as a row of separate segments rather than one rounded shape.

Both arrive when native-theme states them, researched in
`docs/platform-facts.md` first (`docs/todo.md`, Core API, *Segmented control:
the join and the divider*).

### 4.5 `wrap::link`, `wrap::hyperlink` — Tier W

```rust
pub fn link<'a>(ui: &egui::Ui, text: impl Into<egui::WidgetText>) -> impl egui::Widget + 'a;
pub fn hyperlink<'a>(ui: &egui::Ui, text: impl Into<egui::WidgetText>, url: impl ToString) -> impl egui::Widget + 'a;
```

Each takes `&egui::Ui` solely to read the atlas and the active scheme, and
returns `impl Widget` (rationale §7 Q-1).

The connector grades `link.hover_text_color`, `link.active_text_color`,
`link.hover_background` and `link.visited_text_color` DERIVED, a per-call colour
egui's own `Link` takes (connector spec §5.3), and these wrappers are the
ready-made form of that call.

**`wrap::link`** adds egui's `Link` in the `Role::Link` scope and colours its
text per instance. `Link` paints in `visuals.hyperlink_color`
(`widgets/hyperlink.rs:47`) only as a fallback: `TextShape` replaces nothing
but `Color32::PLACEHOLDER` with it (`epaint/src/shapes/text_shape.rs:25`), which
is what text with no colour of its own carries (`widget_text.rs:423`). So the
wrapper colours the text with `WidgetText::color` (`widget_text.rs:637`):
`link.disabled_text_color` in a disabled `Ui`, else `link.active_text_color`
or `hover_text_color` by this pass's interaction, read before the widget is
added (`Context::read_response`, `context.rs:1350-1355`) with the id
`ui.next_auto_id()` gives inside the wrapper's scope (`ui.rs:889`), the idiom
egui's own `Checkbox` uses (`widgets/checkbox.rs:72-73`) — else
`link.font.color`; and it sets its scope's `hyperlink_color` to the same
colour, so egui's own hover underline matches. A link wrapped over rows
answers on its first row only: a wrapped `Label` gives each further row a
fresh id (`widgets/label.rs:219-227`, `ui.rs:1272-1273`) and `union` keeps the
first (`response.rs:1083`). `link.underline_enabled` goes through `WidgetText::underline`
(`widget_text.rs:673`), drawn in the text's colour (`:422`, `:446-448`);
`link.background_color`, with `hover_background` composited over it while
hovered, through `WidgetText::background_color` (`:709`).

**`wrap::hyperlink`** does the same for the `Link` that `Hyperlink` adds
(`widgets/hyperlink.rs:130`), plus the visited state a bare `Link` cannot have:
`Hyperlink` carries its URL (`:92-96`) and opens it on a click (`:132-142`), so
the wrapper records the URL when the response reports that click, in a set
this crate keeps in `Context` data (`IdTypeMap::get_temp_mut_or_default`,
`util/id_type_map.rs:503`) for the life of the `Context`, and requests a
repaint. A link whose URL is in the set takes `link.visited_text_color` at
rest, and its AccessKit node is marked visited (`set_visited`, accesskit 0.24.1
`src/lib.rs`, line 1804). The set holds what this `Context` saw clicked, and
nothing else: an application's own history is its own.

### 4.6 Delivered by the connector — not a widget here

| wanted | how the application gets it |
|---|---|
| any egui widget in its native role colours — button, checkbox, radio button, text field, combo box, separator, progress bar, tab | the connector's `native_scope` with the widget's `Role`: `RoleVariant::Selected` for a checked checkbox, while an active tab or a primary button is a `Button::selected(true)` in the `Normal` scope, whose cell carries those colours in `selection.*`. A `ScrollArea`'s bars need no scope: the base style carries them (connector spec §5.9) |
| toolbar, status bar, sidebar, card, window, dialog, popover, tooltip and menu chrome | the connector's `Surface` frames and role scopes |
| an expander's arrow colour | the connector's `expander_icon`, passed to `CollapsingHeader::icon` |
| a text field's focused border | the connector's `input_frame`, passed to `TextEdit::frame` |

Each is one call at the call site with no pixel this crate could add, which
§1.5 settles: the cheapest tier is no widget at all.

---

## 5 -- Cargo.toml, features, MSRV

```toml
[package]
name = "native-theme-egui-widgets"
edition = "2024"
# NOT `rust-version.workspace = true` — egui 0.36.2 declares 1.95
# (`egui/Cargo.toml:14`), exactly as the connector does.
rust-version = "1.95"

[dependencies]
# egui and native-theme are reached through the connector's re-exports
# (`native_theme_egui::egui`, `native_theme_egui::native_theme`), so the graph
# holds one copy of each and this crate names no version of either.
native-theme-egui = { version = "…", path = "../native-theme-egui", default-features = false }

[features]
# The connector's default set, forwarded one to one; `svg-rasterize` is
# forwarded but off by default, as it is there.
default = ["material-icons", "lucide-icons", "system-icons", "system-fonts"]
material-icons = ["native-theme-egui/material-icons"]
lucide-icons   = ["native-theme-egui/lucide-icons"]
system-icons   = ["native-theme-egui/system-icons"]
svg-rasterize  = ["native-theme-egui/svg-rasterize"]
system-fonts   = ["native-theme-egui/system-fonts"]
```

The default set is the connector's, forwarded name for name, so an application
using this crate gets the same native look by default as one using the
connector alone, and can still turn a feature off — which it could not were the
connector a dependency with its default features. This crate adds **no**
feature of its own. The version policy is the connector's: one egui minor,
never a range.

---

## 6 -- Testing

Headless: tests drive a bare `egui::Context` with `RawInput`, AccessKit enabled
(`Context::enable_accesskit`, `context.rs:3701`) where a test reads nodes.

| # | group | proves |
|---|---|---|
| T1 | **No-atlas fallback** | every widget renders with no atlas installed, adds the egui counterpart §2.1 names, returns a `Response`, and panics nowhere |
| T2 | **Hostile theme** | `NaN`, `±∞`, `-0.0` and extreme magnitudes in every `f32` leaf a widget reads; every widget allocates finite space, and a Tier P widget whose geometry is not finite takes §2.1's fallback |
| T3 | **Accessibility** | every widget's node carries the role and states of §2.7; a clicked `wrap::hyperlink` is visited on the next pass — its node carries `visited` and its text `link.visited_text_color`; focused with `Key::Tab`, the `Switch` has registered its track as its focus shape and no widget paints a ring |
| T4 | **Reduced motion** | under reduced motion the `Switch` thumb reaches its end in the pass of the click, the painted `Spinner` requests no repaint and paints the same shapes on two passes |
| T5 | **Disabled** | `.enabled(false)` paints the widget's disabled leaves (or the base colour where a leaf is `None`), its painter's opacity is unchanged inside the scope, and a click does not change the value |
| T6 | **No hardcoded values** | a source scan for numeric literals in painting code, allowing only §2.3's table |
| T7 | **Tier P promotions are still justified** | each §4 citation still describes the hardcoded upstream fact: no switch module under `widgets/`; `Stroke::new(3.0` and `- 2.0` in `widgets/spinner.rs`; rail and resting knob both from `inactive.bg_fill` in `widgets/slider.rs`. A failure demotes the widget (§1.5) |
| T8 | **Tier C stays cheap** | `SegmentedControl` calls neither `Ui::allocate_response` nor `Ui::allocate_exact_size` and senses nothing itself; every segment is an egui `Button` |

T6 is the mechanical enforcement of the project's no-hardcoded-values rule.

---

## 7 -- Limits: what this crate does NOT fix

| # | what is lost | evidence |
|---|---|---|
| 1 | **Existing code does not benefit.** Every call site must change to `ui.add(…)`; a user who adds the crate and changes nothing sees no difference | §3.2: inherent-method resolution makes the alternative unsound |
| 2 | **Third-party egui crates are unaffected.** They render with the style of the `Ui` they are given — the connector's base style unless the application scopes them; only the connector's focus ring reaches them | they call egui's widgets directly; nothing here is in their path |
| 3 | **A bare `Link`'s visited colour, and every link's hover underline, stay egui's.** `Link` carries no URL (`widgets/hyperlink.rs:27-29`) and egui records no visited state, so `wrap::link` has nothing to attach `link.visited_text_color` to; and `Link` underlines itself on hover or focus (`:50-54`) whatever `link.underline_enabled` states for the link at rest | native-theme states nothing about a hover underline, so the theme specifies no appearance `:50-54` prevents, and §1.5 admits no promotion on it |
| 4 | **The macOS spinner is an arc, not fins** | §4.3; the model states no fin geometry |
| 5 | **A segmented control's divider line and joined outline are not drawn** | §4.4; no source states them |
| 6 | **No slider tick marks** — `slider.tick_mark_length` | `Slider` takes no tick positions; egui's paints none either |
