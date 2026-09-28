# native-theme-gpui `widgets` — Specification

Target: **gpui-component 0.6.6, gpui-base 0.6.6, gpui-pre 0.3.6** and the
native-theme model as resolved on branch `v0.6.0-rc1`. The sibling design for
egui is [`todo_egui-widgets-spec.md`](todo_egui-widgets-spec.md); this document
follows its rules wherever the two toolkits allow, and says where they differ.

Citations: `GC` = `gpui-component-0.6.6/src`, `GB` = `gpui-base-0.6.6/src`,
`GP` = `gpui-pre-0.3.6/src`, all as published on crates.io.

---

## 0 — Scope

### 0.1 What the module is

A module of the gpui connector, `native_theme_gpui::widgets`, behind the
feature `widgets` (on by default, as `native-theme-iced`'s `widgets` feature
is). It draws the six controls whose look gpui-component builds from its own
literals, so that the connector's theme cannot reach them (the round-2 audit's
CANNOT rows):

| widget | what gpui-component hard-codes |
|---|---|
| `Checkbox` | indicator size in rems per `Size` (GC `checkbox.rs:219-224`), radius `radius.min(4)` (`:245`), unchecked fill `input_background()` (`:299`), disabled at half alpha (`:240-244`), no hover (`:288-322`) |
| `Radio`, `RadioGroup` | the same indicator (GC `radio.rs:168-173`), fills and half-alpha disabled (`:186-192`), no hover |
| `Switch` | track 36×20 / thumb 16 per `Size` (GC `switch.rs:150-157`), disabled = half alpha of the track (`:145`), no hover, thumb never fades |
| `Slider` | rail = fill at 0.2 (GC `slider.rs:290`), rail height `h_1p5` (`:288`), thumb `size_4` (`:218`), hover only grows a ring (`:15-17`); no keyboard |
| `ProgressBar` | track = fill at 0.2 (GC `progress/progress.rs:135`), no border |
| `Spinner` | a turning `IconName::Loader` icon (GC `spinner.rs:25`, `:70`): no stroke width |

Everything else in the connector stays as it is: a control gpui-component
draws from the connector's `ThemeColor` and `geometry` builders needs no
widget here.

### 0.2 What it is not

* Not a replacement for gpui-component: it depends on it and falls back to it.
* Not a theme mapper: colours come straight from `ResolvedTheme` leaves, not
  through `ThemeColor`.
* Not a fork of gpui-base: every behaviour (activation, focus, keyboard,
  AccessKit role and state, dragging) is gpui-base's headless primitive, the
  one gpui-component builds its own control on.

---

## 1 — Rules every widget follows

### 1.1 The theme, and no theme

Every widget reads the variant installed for the current mode,
`cx.native_theme()?.native(cx)` (the connector's `NativeTheme`, `lib.rs`),
at render. **With no native theme installed, it renders gpui-component's own
control** with the same builder values, so an application that adds a widget
before `apply` sees ordinary gpui-component, never a fabricated colour. It
takes the same fallback when a length it lays out is not finite.

### 1.2 Sources of every value

Every colour, length, radius, width and gap is a `ResolvedTheme` leaf, or —
where the model states none — gpui-component's own value for the same part,
named as a constant with its upstream citation. No other literal is painted.
Text sizes carry the text-scaling factor (`scaled_text_size`), lengths do not
(connector spec §3.4).

### 1.3 States

* **Hover is a layer** composited over the idle fill (`Hsla::blend`, as
  `colors.rs` composites the button states, C17). A `None` soft option copies
  the base colour it would cover (C16). A state the model states nothing for
  has no look of its own; no colour is derived by alpha or interpolation.
* **Disabled** paints the widget's stated disabled leaves; `disabled_opacity`
  is not multiplied on top (the stated colours already carry the fade), as the
  iced connector does (`styles.rs`, `checkbox`).
* **Checked / unchecked, hovered** follow the iced connector's reading of
  `CheckboxTheme` so the two draw the same control: `hover_background` is the
  hover of the *unchecked* box; a checked box's border is `border.color`, an
  unchecked one's `unchecked_border_color`; a disabled box's mark is
  `disabled_text_color`, because `disabled_background` replaces the accent the
  on-accent `indicator_color` was stated for (iced `styles.rs:470-500`).

### 1.4 Kept from gpui-component (behaviour)

Activation by click, Enter and Space; focus and tab order; a pointer press not
moving focus (checkbox, radio); the focus ring (`ThemeStyled::focus_ring_style`,
GC `styled.rs:175`) where gpui-component draws one; the AccessKit role and
toggled/value state; a disabled control inert and unfocusable; the switch
thumb's spring and the check mark's spring fade (`gpui_base::spring`, the
theme's `motion_tokens`), which gpui holds still under reduced motion
(GP `elements/animation.rs:298-307`).

---

## 2 — The widgets

### 2.1 `Checkbox` — on `gpui_base::Checkbox`

| part | leaf | how |
|---|---|---|
| indicator | `checkbox.indicator_width` | square side (PF `platform-facts.md:980`) |
| indicator | `checkbox.border.corner_radius`, `.line_width` | radius, border width |
| fill, unchecked | `unchecked_background` (`None` → `background_color`) | |
| fill, checked | `checked_background` | |
| fill, hovered unchecked | `hover_background` (`None` → `background_color`) over the unchecked fill | `group_hover` on the indicator |
| fill, disabled | `disabled_background` (`None` → `background_color`), checked or not | |
| border | checked: `border.color`; unchecked: `unchecked_border_color` (`None` → `border.color`) | disabled keeps the state's colour |
| mark | `indicator_color`; disabled: `disabled_text_color` | gpui-component's `IconName::Check` path, filling the box inside its border: PF §2.5 "checkmark fills indicator" (`platform-facts.md:1216`) |
| label | `checkbox.font` size, weight, `font.color`; disabled `disabled_text_color` | |
| label gap | `checkbox.label_gap` | |

Not drawn: `border.shadow_enabled` (the model states no shadow geometry),
`disabled_opacity` (§1.3). Kept: the ring on the whole row while focused, the
row's `radius * 0.5` rounding that ring follows (GC `checkbox.rs:280-283`).

### 2.2 `Radio`, `RadioGroup` — on `gpui_base::Radio` / `RadioGroup`

As §2.1, from the same `CheckboxTheme` (PF §2.5: "Radio buttons use the same
colors but with circular `border.corner_radius`", `platform-facts.md:1222`):
the indicator is a circle of `indicator_width`. **The mark is a dot**
`radio_dot_diameter` across in `indicator_color`, centred in the circle
(`platform-facts.md:1220`: KDE 6, GNOME 8, Windows 12), faded in and out on the
check mark's spring. Where the theme states no dot size — macOS publishes none —
the mark is gpui-component's own check glyph (GC `radio.rs:242`), since gpui has
no dot of its own.
`RadioGroup` is gpui-base's (role `RadioGroup`, orientation) with each radio's
position in the set; it lays its radios out with the caller's style.

### 2.3 `Switch` — on `gpui_base::Switch`, `SwitchTrack`, `SwitchThumb`

| part | leaf |
|---|---|
| track | `track_width` × `track_height`, radius `track_radius` |
| thumb | circle of `thumb_diameter`, centred on the track's axis, inset `0.5 × (track_height − thumb_diameter)` from each end (a larger thumb overhangs, as the numbers state) |
| track colour | `checked_background` / `unchecked_background` |
| hovered | `hover_checked_background` / `hover_unchecked_background` over it (`None` → the track itself) |
| disabled | `disabled_checked_background` / `disabled_unchecked_background`, thumb `disabled_thumb_color` (each `None` → the enabled colour, C16) |
| thumb colour | `thumb_background` |
| label | `SwitchTheme` states no font: the window's `defaults.font`, colour `defaults.text_color`, disabled `defaults.disabled_text_color` |

Kept from gpui-component (unstated): the track↔label gap `gap_2`
(GC `switch.rs:197`), the label's line box = the track height (`:237`), the
thumb's spring. gpui-component's switch draws no focus ring, and neither does
this one.

### 2.4 `Slider` — on `gpui_base::slider::{SliderState, SliderTrack, SliderIndicator, SliderThumb}`

| part | leaf |
|---|---|
| rail | `track_height` tall, `track_color`; disabled `disabled_track_color` |
| filled part | `fill_color` from the rail's start to the thumb's centre; disabled `disabled_fill_color` |
| thumb | circle of `thumb_diameter`, `thumb_color`; hovered: `thumb_hover_color` over it; disabled `disabled_thumb_color` |
| height | the larger of `thumb_diameter` and `track_height` |

The thumb's centre travels the width less one thumb radius at each end (the
geometry egui's slider uses, `todo_egui-widgets-spec.md` §4.2), so the thumb
stays inside the widget's box; the pointer maps onto the same travel
(`SliderIndicator` records it, GB `slider.rs:686-693`). Unstated and kept from
gpui-component: the rail's pill rounding (`radius_full`, GC `slider.rs:174`),
the thumb's 1-px outline in the fill colour at half alpha (`:216-219`); not
kept: the pressed rail at 0.4 and the hover ring (colours the model does not
state).

Interaction: gpui-base's pointer press and drag on the rail and the thumb, and
its AccessKit `Increment`/`Decrement` (GB `slider.rs:469-490`). **Added**,
because gpui-component's slider cannot be focused: a focus handle, the focus
ring on the thumb while focused, and the keys Left/Down (−step), Right/Up
(+step), Home (min), End (max), which update the state and emit
`SliderEvent::Change`.

### 2.5 `ProgressBar` — on `gpui_base::Progress`, `ProgressTrack`, `ProgressIndicator`

`track_height` tall, at least `min_width` wide, radius `border.corner_radius`,
track `track_color`, fill `fill_color` for the value, framed by `border.color`
at `border.line_width` inside that height. (Whether a platform draws the frame
is open, O4; the resolved theme states it and the iced connector draws it.)
Kept: gpui-component's eased value transition (GC `progress/progress.rs:111`).

### 2.6 `Spinner` — on `gpui_base::Progress` (indeterminate)

An arc: outer size `spinner.diameter` (at least `min_diameter`), stroke
`stroke_width`, colour `fill_color`; the stroke's centre line has radius
`0.5 × (diameter − stroke_width)`. The motion is gpui-component's own
indeterminate arc (GC `progress/progress_circle.rs:199-208`: over one second,
the head eased from 0 to 100 %, the tail following over the second half),
painted with gpui-component's `plot::shape::Arc`. No track ring: the model
states no track colour. Under reduced motion the arc is painted still at that
motion's half-way frame (gpui would render a repeating animation's first
frame, an empty arc, GP `elements/animation.rs:407-414`).

---

## 3 — Public API

```rust
use native_theme_gpui::widgets::{Checkbox, Radio, RadioGroup, Switch, Slider, ProgressBar, Spinner};

Checkbox::new("id").label("Wi-Fi").checked(on).disabled(false).on_change(|checked, window, cx| {});
Radio::new("id").label("A").checked(true).on_change(|_, window, cx| {});
RadioGroup::new("id").children([radio_a, radio_b]);         // Styled, ParentElement-free
Switch::new("id").label("On").checked(on).on_change(|checked, window, cx| {});
Slider::new(&slider_state).disabled(false).w(px(140.));     // Styled
ProgressBar::new("id").value(40.);                            // Styled
Spinner::new("id");
```

Every widget is a `RenderOnce` element (`#[derive(IntoElement)]`). The
callbacks are controlled, as gpui-component's: the owner writes the requested
value and notifies.

---

## 4 — Tests (`src/widgets/tests.rs`, headless `TestAppContext`)

| group | proves |
|---|---|
| look | per widget and state, each colour and length the widget paints equals the leaf §2 names, under kde-breeze light and material dark (the `*Look` functions the render uses) |
| geometry | laid out under a native theme, the indicator, track, thumb, rail, bar and arc boxes measure the stated sizes (`debug_bounds`) |
| behaviour | click, Enter and Space toggle; a disabled control is inert and unfocusable; a radio reports its index; a slider click sets the value, the arrow keys, Home and End step it |
| fallback | with no native theme installed every widget lays out (gpui-component's control) |
| accessibility | role and toggled / numeric value on the nodes gpui-base writes |

## 5 — Limits

| # | what is not drawn | why |
|---|---|---|
| 1 | a radio dot on macOS | `checkbox.radio_dot_diameter` is unstated there; gpui-component's mark is kept (§2.2) |
| 2 | slider tick marks | the widget takes no tick positions, as gpui-component's |
| 3 | the macOS spinner's fins | the model states no fin geometry; the arc is drawn |
| 4 | shadows (`checkbox.border.shadow_enabled`, `progress_bar.border.shadow_enabled`) | no shadow geometry in the model |
