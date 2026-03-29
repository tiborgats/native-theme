# v0.5.2 — Iced Connector: Full Theme Geometry Support

## Problem

The iced connector (`native-theme-iced`) currently maps only **colors** from
`native_theme::ThemeVariant` to iced's `Theme`. Geometry properties (border
radius, border width, disabled opacity, shadow) and font settings are exposed
as standalone helper functions that each application must manually apply to
every widget instance.

This means `to_theme()` produces a Theme that looks correct in color but uses
iced's hardcoded geometry everywhere — for example, every widget gets a 2.0px
border radius regardless of whether the theme specifies 4.0 (KDE Breeze),
8.0 (Catppuccin), or 12.0 (Material).

The fix: the connector should provide **custom style functions** that respect
all theme properties, so applications get full theming without extra effort.

---

## Options Considered

### Option A: Custom Style Functions (chosen)

The connector provides drop-in replacement functions for each of iced's
built-in style functions. Instead of `button::primary`, the user writes
`native_theme_iced::button::primary(&variant)` which returns a closure
with the correct radius, border width, opacity, etc. baked in.

**Pros:**
- Decoupled from iced internals — functions return `Style` structs, which are
  simple data types that rarely change across iced versions
- Users can mix native-theme style functions with their own custom styles
- No wrapper type needed — `iced::Theme` is used directly
- Incremental adoption — users can switch one widget at a time

**Cons:**
- Users must change `.style(button::primary)` to
  `.style(native_theme_iced::button::primary(&variant))` per widget
- Every iced style function needs a corresponding wrapper

### Option B: Wrapper Theme Type (rejected)

Define a `NativeIcedTheme` struct wrapping `iced::Theme` + `ThemeVariant`
that implements all `Catalog` traits. Users get fully transparent theming —
`.style(button::primary)` automatically uses the theme's radius.

**Rejected because:**
- Implements every `Catalog` trait (`button::Catalog`, `text_input::Catalog`,
  `container::Catalog`, `checkbox::Catalog`, etc.) — if iced adds a widget,
  changes a trait signature, or modifies the `Class` associated type, the
  wrapper breaks
- Tight coupling to iced's internal `Class<'a> = StyleFn<'a, Theme>` pattern
- If iced changes how `Theme::custom_with_fn` works, both color AND geometry
  mapping break simultaneously
- Users can't easily mix standard iced styles with native-theme styles

### Option C: Keep Helpers Only (rejected)

Leave the current architecture. Applications manually apply geometry via
`border_radius()`, `scrollbar_width()`, etc.

**Rejected because:**
- Every application using the connector must duplicate the same boilerplate
- The connector's job is to bridge native-theme → iced; exposing raw helpers
  defeats the purpose
- Screenshots/GIFs show iced with wrong geometry, undermining the "look
  native" value proposition

---

## Theme Properties — Complete Inventory

Every property in `ThemeVariant` that affects visual output, grouped by
what the iced connector can and should map.

### 1. Geometry (`ThemeGeometry`)

| Property | Type | Range across presets | iced default | Used by |
|----------|------|---------------------|--------------|---------|
| `radius` | `f32` | 4.0 – 12.0 | 2.0 | button, container, text_input, checkbox, progress_bar, scrollable, pick_list, text_editor, slider rail |
| `radius_lg` | `f32` | 8.0 – 16.0 | 5.0 (bordered_box) | container (panels, cards, dialogs) |
| `frame_width` | `f32` | 0.5 – 1.0 | 0.0 or 1.0 | text_input border, checkbox border, pick_list border, text_editor border, radio border |
| `disabled_opacity` | `f32` | 0.3 – 0.5 | 0.5 (hardcoded) | button, checkbox, toggler, text_input, pick_list disabled states |
| `border_opacity` | `f32` | 0.12 – 0.2 | varies | border color alpha in text_input, checkbox, pick_list |
| `scroll_width` | `f32` | 6.0 – 10.0 | (widget-level) | scrollable rail/scroller width |
| `shadow` | `bool` | true/false | none | button shadow, container shadow |

### 2. Fonts (`ThemeFonts`)

| Property | Type | Example values | iced mechanism |
|----------|------|---------------|----------------|
| `family` | `String` | "Noto Sans", "Segoe UI", "SF Pro Text", "Roboto" | `iced::Font::with_name()` — set on `Settings.default_font` |
| `size` | `f32` (pt) | 10.0, 13.0, 14.0 | `Settings.default_text_size` (after pt→px conversion) |
| `mono_family` | `String` | "Hack", "Menlo", "Consolas", "Roboto Mono" | Set per-widget where monospace is needed |
| `mono_size` | `f32` (pt) | 10.0, 12.0, 13.0 | Set per-widget |

**Note:** Fonts are handled at the application level (`Settings`), not through
style functions. The connector should provide a helper to configure iced
`Settings` from a `ThemeVariant`, but fonts do NOT need custom style functions.

### 3. Widget Metrics (`WidgetMetrics`)

These are per-widget sizing values. iced exposes some as widget builder
methods (e.g., `button.padding()`, `checkbox.size()`), not through the
style system. The connector should provide helpers that apply these.

| Widget | Metrics | iced mechanism |
|--------|---------|----------------|
| **Button** | `min_width`, `min_height`, `padding_horizontal`, `padding_vertical`, `icon_spacing` | `.padding()`, `.width()`, `.height()` |
| **Checkbox** | `indicator_size`, `spacing` | `.size()`, `.spacing()` |
| **Input** | `min_height`, `padding_horizontal`, `padding_vertical` | `.padding()` |
| **Scrollbar** | `width`, `min_thumb_height`, `slider_width` | `.width()`, `.scroller_width()` |
| **Slider** | `track_height`, `thumb_size`, `tick_length` | Custom style only (rail width, handle shape) |
| **ProgressBar** | `height`, `min_width` | `.height()` |
| **Tab** | `min_width`, `min_height`, `padding_horizontal`, `padding_vertical` | No direct iced widget (app-level) |
| **MenuItem** | `height`, `padding_horizontal`, `padding_vertical`, `icon_spacing` | No direct iced widget (app-level) |
| **Tooltip** | `padding`, `max_width` | `.padding()` |
| **ListItem** | `height`, `padding_horizontal`, `padding_vertical` | No direct iced widget (app-level) |
| **Toolbar** | `height`, `item_spacing`, `padding` | No direct iced widget (app-level) |
| **Splitter** | `width` | pane_grid split line width |

### 4. Spacing (`ThemeSpacing`)

| Property | Type | Range | iced mechanism |
|----------|------|-------|----------------|
| `xxs` – `xxl` | `f32` | 2.0 – 36.0 | `.spacing()`, `.padding()` on layouts |

**Note:** Spacing is used at the layout level, not the style level. The
connector should expose these as named constants from the variant, but they
don't need custom style functions.

### 5. Colors (`ThemeColors`)

Already fully mapped by `to_theme()` → `palette::to_palette()` →
`extended::apply_overrides()`. **No changes needed.**

36 color roles: accent, background, foreground, surface, border, muted,
shadow, primary_background, primary_foreground, secondary_background,
secondary_foreground, danger, danger_foreground, warning, warning_foreground,
success, success_foreground, info, info_foreground, selection,
selection_foreground, link, focus_ring, sidebar, sidebar_foreground, tooltip,
tooltip_foreground, popover, popover_foreground, button, button_foreground,
input, input_foreground, disabled, separator, alternate_row.

---

## Custom Style Functions Needed

For each iced built-in style function, the connector needs a corresponding
function that applies theme geometry. The pattern:

```rust
// iced built-in (hardcoded radius 2.0):
button::primary(theme, status) -> Style

// native-theme replacement (uses theme's radius):
native_theme_iced::button::primary(variant) -> impl Fn(&Theme, Status) -> Style
```

### `native_theme_iced::button`

Replaces: `iced::widget::button::{primary, secondary, success, warning, danger, text, background, subtle}`

Properties applied:
- `geometry.radius` → `border.radius`
- `geometry.disabled_opacity` → disabled state alpha
- `geometry.shadow` → `shadow` field (if true, add subtle drop shadow)

| Function | iced original | Geometry applied |
|----------|--------------|-----------------|
| `primary(v)` | `button::primary` | radius, disabled_opacity, shadow |
| `secondary(v)` | `button::secondary` | radius, disabled_opacity, shadow |
| `success(v)` | `button::success` | radius, disabled_opacity, shadow |
| `warning(v)` | `button::warning` | radius, disabled_opacity, shadow |
| `danger(v)` | `button::danger` | radius, disabled_opacity, shadow |
| `text(v)` | `button::text` | radius, disabled_opacity |
| `background(v)` | `button::background` | radius, disabled_opacity, shadow |
| `subtle(v)` | `button::subtle` | radius, disabled_opacity, shadow |

### `native_theme_iced::container`

Replaces: `iced::widget::container::{rounded_box, bordered_box, dark, primary, secondary, success, warning, danger}`

Properties applied:
- `geometry.radius` → `border.radius` (for `rounded_box`, `dark`, `primary`, `secondary`, `success`, `warning`, `danger`)
- `geometry.radius_lg` → `border.radius` (for `bordered_box` — cards/panels)
- `geometry.frame_width` → `border.width` (for `bordered_box`)
- `geometry.shadow` → `shadow` field

| Function | iced original | Geometry applied |
|----------|--------------|-----------------|
| `rounded_box(v)` | `container::rounded_box` | radius, shadow |
| `bordered_box(v)` | `container::bordered_box` | radius_lg, frame_width, shadow |
| `dark(v)` | `container::dark` | radius |
| `primary(v)` | `container::primary` | radius, shadow |
| `secondary(v)` | `container::secondary` | radius, shadow |
| `success(v)` | `container::success` | radius, shadow |
| `warning(v)` | `container::warning` | radius, shadow |
| `danger(v)` | `container::danger` | radius, shadow |

**Not wrapped:** `container::transparent` and `container::background` — these
have no border/radius to customize.

### `native_theme_iced::text_input`

Replaces: `iced::widget::text_input::default`

Properties applied:
- `geometry.radius` → `border.radius`
- `geometry.frame_width` → `border.width`
- `geometry.disabled_opacity` → disabled state alpha
- `geometry.border_opacity` → border color alpha

| Function | iced original | Geometry applied |
|----------|--------------|-----------------|
| `default(v)` | `text_input::default` | radius, frame_width, disabled_opacity, border_opacity |

### `native_theme_iced::text_editor`

Replaces: `iced::widget::text_editor::default`

Properties applied:
- `geometry.radius` → `border.radius`
- `geometry.frame_width` → `border.width`
- `geometry.disabled_opacity` → disabled state alpha
- `geometry.border_opacity` → border color alpha

| Function | iced original | Geometry applied |
|----------|--------------|-----------------|
| `default(v)` | `text_editor::default` | radius, frame_width, disabled_opacity, border_opacity |

### `native_theme_iced::checkbox`

Replaces: `iced::widget::checkbox::{primary, secondary, success, danger}`

Properties applied:
- `geometry.radius` → `border.radius`
- `geometry.frame_width` → `border.width`
- `geometry.disabled_opacity` → disabled state alpha

| Function | iced original | Geometry applied |
|----------|--------------|-----------------|
| `primary(v)` | `checkbox::primary` | radius, frame_width, disabled_opacity |
| `secondary(v)` | `checkbox::secondary` | radius, frame_width, disabled_opacity |
| `success(v)` | `checkbox::success` | radius, frame_width, disabled_opacity |
| `danger(v)` | `checkbox::danger` | radius, frame_width, disabled_opacity |

### `native_theme_iced::progress_bar`

Replaces: `iced::widget::progress_bar::{primary, secondary, success, warning, danger}`

Properties applied:
- `geometry.radius` → `border.radius`

| Function | iced original | Geometry applied |
|----------|--------------|-----------------|
| `primary(v)` | `progress_bar::primary` | radius |
| `secondary(v)` | `progress_bar::secondary` | radius |
| `success(v)` | `progress_bar::success` | radius |
| `warning(v)` | `progress_bar::warning` | radius |
| `danger(v)` | `progress_bar::danger` | radius |

### `native_theme_iced::scrollable`

Replaces: `iced::widget::scrollable::default`

Properties applied:
- `geometry.radius` → rail and scroller `border.radius`
- `geometry.scroll_width` → scroller width (if applied via style)

| Function | iced original | Geometry applied |
|----------|--------------|-----------------|
| `default(v)` | `scrollable::default` | radius |

### `native_theme_iced::pick_list`

Replaces: `iced::widget::pick_list::default`

Properties applied:
- `geometry.radius` → `border.radius`
- `geometry.frame_width` → `border.width`
- `geometry.border_opacity` → border color alpha

| Function | iced original | Geometry applied |
|----------|--------------|-----------------|
| `default(v)` | `pick_list::default` | radius, frame_width, border_opacity |

### `native_theme_iced::toggler`

Replaces: `iced::widget::toggler::default`

Properties applied:
- `geometry.disabled_opacity` → disabled state alpha

| Function | iced original | Geometry applied |
|----------|--------------|-----------------|
| `default(v)` | `toggler::default` | disabled_opacity |

**Note:** Toggler is pill-shaped by design (border_radius = None means fully
round). The theme's `radius` should NOT be applied here — that would make
togglers rectangular, which is wrong on every platform.

### `native_theme_iced::slider`

Replaces: `iced::widget::slider::default`

Properties applied:
- `geometry.radius` → rail `border.radius`
- Widget metrics: `track_height` → rail width, `thumb_size` → handle radius

| Function | iced original | Geometry applied |
|----------|--------------|-----------------|
| `default(v)` | `slider::default` | radius (rail), track_height, thumb_size |

### `native_theme_iced::radio`

Replaces: `iced::widget::radio::default`

Properties applied:
- `geometry.frame_width` → `border.width`

| Function | iced original | Geometry applied |
|----------|--------------|-----------------|
| `default(v)` | `radio::default` | frame_width |

**Note:** Radio buttons are circular by design. The theme's `radius` should
NOT be applied.

### `native_theme_iced::rule`

Replaces: `iced::widget::rule::{default, weak}`

No geometry properties apply (rules are lines with no radius).
**No custom functions needed.**

### `native_theme_iced::qr_code`

Replaces: `iced::widget::qr_code::default`

No geometry properties apply (QR codes are pixel grids).
**No custom functions needed.**

### `native_theme_iced::pane_grid`

Replaces: `iced::widget::pane_grid::default`

Properties applied:
- `geometry.frame_width` → split line width

| Function | iced original | Geometry applied |
|----------|--------------|-----------------|
| `default(v)` | `pane_grid::default` | frame_width |

---

## Summary: Functions to Implement

| Module | Functions | Count |
|--------|-----------|-------|
| `button` | primary, secondary, success, warning, danger, text, background, subtle | 8 |
| `container` | rounded_box, bordered_box, dark, primary, secondary, success, warning, danger | 8 |
| `text_input` | default | 1 |
| `text_editor` | default | 1 |
| `checkbox` | primary, secondary, success, danger | 4 |
| `progress_bar` | primary, secondary, success, warning, danger | 5 |
| `scrollable` | default | 1 |
| `pick_list` | default | 1 |
| `toggler` | default | 1 |
| `slider` | default | 1 |
| `radio` | default | 1 |
| `pane_grid` | default | 1 |
| **Total** | | **33** |

Not wrapped (no geometry applies): `container::transparent`,
`container::background`, `rule::default`, `rule::weak`, `qr_code::default`.

---

## Additional Helpers

Beyond style functions, these helpers bridge other theme properties:

### Font Configuration Helper

```rust
/// Apply theme fonts to iced Settings.
pub fn apply_fonts(settings: &mut iced::Settings, variant: &ThemeVariant) {
    // sets default_font, default_text_size from variant.fonts
}
```

### Widget Metrics Helpers (already partially exist)

Keep existing: `button_padding()`, `input_padding()`, `scrollbar_width()`,
`border_radius()`, `border_radius_lg()`, `font_family()`, `font_size()`,
`mono_font_family()`, `mono_font_size()`.

Add: `checkbox_size()`, `checkbox_spacing()`, `slider_track_height()`,
`slider_thumb_size()`, `progress_bar_height()`, `tooltip_padding()`.

### Spacing Helpers

```rust
/// Get named spacing value from theme.
pub fn spacing(variant: &ThemeVariant, size: SpacingSize) -> f32 { ... }
```
