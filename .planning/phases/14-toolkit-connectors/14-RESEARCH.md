# Phase 14: Toolkit Connectors - Research

**Researched:** 2026-03-08
**Domain:** gpui-component and iced toolkit integration crates
**Confidence:** MEDIUM (gpui-component internals partially verified; iced API well-documented)

## Summary

Phase 14 creates two connector crates -- `native-theme-gpui` and `native-theme-iced` -- that map the core `NativeTheme` data model (36 semantic colors, fonts, geometry, spacing, 12 widget metric sub-structs) to their respective toolkit theming systems. The workspace skeleton already exists (`connectors/native-theme-gpui/` and `connectors/native-theme-iced/`) with stub `lib.rs` files and workspace-level `native-theme` dependency wired up.

The **gpui connector** maps `ThemeColors` to gpui-component's 108-field `ThemeColor` struct (which uses `Hsla` colors). The mapping is asymmetric: native-theme has 36 semantic colors while gpui-component has 108 fields, so many gpui-component fields must be derived (hover states computed by lightening/darkening base colors, chart colors from accent, table colors from list colors, etc.). The gpui-component theme also has `ThemeConfig` with font family, font size, radius, radius_lg, shadow -- these map almost 1:1 to native-theme's `ThemeFonts` and `ThemeGeometry`. gpui itself is available on crates.io as version 0.2.2, and gpui-component as version 0.5.0 (latest 0.5.1), so this connector CAN depend on published crates.

The **iced connector** maps `ThemeColors` to iced's 6-field `Palette` struct (`background`, `text`, `primary`, `success`, `warning`, `danger`), from which iced auto-generates an `Extended` palette with `Background`, `Primary`, `Secondary`, `Success`, `Warning`, `Danger` sub-palettes. For finer control, `Theme::custom_with_fn()` allows overriding the `Extended` palette generation. Per-widget styling uses the `Catalog` trait, where `Theme` implements `button::Catalog`, `container::Catalog`, etc. The latest iced version is **0.14.0** (not 0.13 as in the architecture doc), which overhauled palette generation to use Oklch color space and added a `warning` color to `Palette`.

**Primary recommendation:** Build the iced connector first (simpler, well-documented API, published on crates.io), then the gpui connector (more complex mapping, less mature documentation). Use iced 0.14 (not 0.13). Both connectors should be thin mapping layers with zero intermediate types.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| CONN-01 | `native-theme-gpui` maps ThemeColors to gpui-component's 108 ThemeColor fields | ThemeColor struct fully documented (108 Hsla fields); mapping table below covers direct + derived fields |
| CONN-02 | `native-theme-gpui` maps fonts, geometry, spacing, and widget metrics | ThemeConfig has font_family, mono_font_family, font_size, mono_font_size, radius, radius_lg, shadow -- near-1:1 with native-theme model |
| CONN-03 | `native-theme-gpui` includes upstream PR proposal documents | gpui-component has ThemeConfig-based theming but may lack hooks for fine-grained widget metric application; proposal docs needed |
| CONN-04 | `native-theme-gpui` includes `examples/showcase.rs` widget gallery | gpui-component provides Button, Input, Checkbox, Slider, etc. components that can be rendered in a gallery |
| CONN-05 | `native-theme-iced` maps ThemeColors to iced Palette + Extended palette | Palette has 6 fields (background, text, primary, success, warning, danger); Extended auto-generated; custom_with_fn for overrides |
| CONN-06 | `native-theme-iced` implements per-widget Catalog/Style for 8 core widgets | Catalog trait documented for Button, Container, TextInput, Scrollable, Checkbox, Slider, ProgressBar, Tooltip; Style structs documented |
| CONN-07 | `native-theme-iced` maps geometry, spacing, and widget metrics to Style fields | Style structs have border (Border type), shadow (Shadow type), background, padding -- maps from ThemeGeometry and WidgetMetrics |
| CONN-08 | `native-theme-iced` includes `examples/demo.rs` widget gallery | Standard iced application with all 8 styled widgets rendered |
| CONN-09 | Both connectors include a theme selector dropdown | iced has pick_list widget; gpui-component has Dropdown; both list NativeTheme::list_presets() + "OS Theme" option |
</phase_requirements>

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| native-theme | workspace (0.2.0) | Core theme data model | The crate being connected |
| iced | 0.14 | iced toolkit dependency | Latest stable; has warning color in Palette, Oklch palette generation |
| gpui | 0.2.2 (crates.io) | GPUI framework | Published on crates.io; required by gpui-component |
| gpui-component | 0.5.x (crates.io) | GPUI UI component library | Provides ThemeColor, ActiveTheme, Theme, widget components |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| serde | workspace | Serialization (shared) | Already in workspace dependencies |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| iced 0.14 | iced 0.13 | 0.14 adds warning to Palette, Oklch generation, is latest stable |
| gpui-component crates.io | gpui-component git | crates.io is stable and publishable; git tracks bleeding edge |

**Installation (iced connector):**
```toml
[dependencies]
native-theme.workspace = true
iced = { version = "0.14", features = ["tokio"] }
```

**Installation (gpui connector):**
```toml
[dependencies]
native-theme.workspace = true
gpui = "0.2"
gpui-component = "0.5"
```

## Architecture Patterns

### Recommended Project Structure

```
connectors/
  native-theme-gpui/
    Cargo.toml
    src/
      lib.rs              # Public API: apply(), to_theme_color(), to_theme_config()
      colors.rs           # ThemeColors -> ThemeColor mapping (108 fields)
      config.rs           # ThemeFonts/ThemeGeometry/WidgetMetrics -> ThemeConfig
      derive.rs           # Shade derivation helpers (lighten/darken for hover/active states)
    examples/
      showcase.rs         # Widget gallery with theme selector dropdown
    proposals/
      README.md           # Upstream PR proposal for missing theming hooks
  native-theme-iced/
    Cargo.toml
    src/
      lib.rs              # Public API: to_theme(), NativeIcedTheme wrapper
      palette.rs          # ThemeColors -> iced::theme::Palette mapping
      catalog.rs          # Catalog trait impls for 8 widgets
      extended.rs         # Custom Extended palette generation from ThemeColors
    examples/
      demo.rs             # Widget gallery with theme selector dropdown
```

### Pattern 1: Thin Mapping Layer (No Intermediate Types)

**What:** Connector crates map directly from `native_theme::ThemeVariant` fields to toolkit types. No intermediate structs, no additional data model layer.
**When to use:** Always. This is the fundamental design principle for connectors.
**Example:**

```rust
// native-theme-iced/src/palette.rs
use native_theme::{Rgba, ThemeVariant};

/// Convert an Option<Rgba> to iced::Color, falling back to a default.
fn to_color(rgba: Option<Rgba>, default: iced::Color) -> iced::Color {
    match rgba {
        Some(c) => {
            let [r, g, b, a] = c.to_f32_array();
            iced::Color { r, g, b, a }
        }
        None => default,
    }
}

/// Build an iced Palette from a ThemeVariant.
pub fn to_palette(variant: &ThemeVariant) -> iced::theme::Palette {
    iced::theme::Palette {
        background: to_color(variant.colors.background, iced::Color::WHITE),
        text: to_color(variant.colors.foreground, iced::Color::BLACK),
        primary: to_color(variant.colors.accent, iced::color!(0x0078d7)),
        success: to_color(variant.colors.success, iced::color!(0x107c10)),
        warning: to_color(variant.colors.warning, iced::color!(0xff8c00)),
        danger: to_color(variant.colors.danger, iced::color!(0xd13438)),
    }
}
```

### Pattern 2: Shade Derivation for gpui-component

**What:** gpui-component's ThemeColor has hover/active states for primary, secondary, danger, etc. Native-theme only has base colors. Derive states by adjusting lightness.
**When to use:** When mapping to gpui-component ThemeColor fields that have no direct native-theme equivalent.
**Example:**

```rust
// native-theme-gpui/src/derive.rs
use gpui::Hsla;

/// Lighten an Hsla color by the given factor (0.0 to 1.0).
fn lighten(color: Hsla, factor: f32) -> Hsla {
    Hsla {
        l: (color.l + (1.0 - color.l) * factor).min(1.0),
        ..color
    }
}

/// Darken an Hsla color by the given factor (0.0 to 1.0).
fn darken(color: Hsla, factor: f32) -> Hsla {
    Hsla {
        l: (color.l * (1.0 - factor)).max(0.0),
        ..color
    }
}

/// Derive a hover state from a base color.
/// Light themes: darken slightly. Dark themes: lighten slightly.
pub fn hover_color(base: Hsla, is_dark: bool) -> Hsla {
    if is_dark { lighten(base, 0.1) } else { darken(base, 0.1) }
}

/// Derive an active/pressed state from a base color.
pub fn active_color(base: Hsla, is_dark: bool) -> Hsla {
    if is_dark { lighten(base, 0.15) } else { darken(base, 0.15) }
}
```

### Pattern 3: iced Custom Theme with Extended Palette Override

**What:** Use `Theme::custom_with_fn()` to override the auto-generated Extended palette with native-theme colors where available.
**When to use:** When the auto-generated Extended palette doesn't match the source theme closely enough.
**Example:**

```rust
// native-theme-iced/src/lib.rs
use iced::theme::{self, Palette};
use native_theme::ThemeVariant;
use std::sync::Arc;

/// Create an iced Theme from a NativeTheme variant.
pub fn to_theme(variant: &ThemeVariant, name: &str) -> iced::Theme {
    let palette = crate::palette::to_palette(variant);

    iced::Theme::custom_with_fn(
        name.to_string(),
        palette,
        |palette| {
            let mut extended = theme::palette::Extended::generate(palette);
            // Override extended palette with native-theme colors where available
            crate::extended::apply_overrides(&mut extended, variant);
            extended
        },
    )
}
```

### Pattern 4: Variant Selection Based on Toolkit Context

**What:** Pick light or dark ThemeVariant based on the toolkit's mode detection.
**When to use:** In both connectors when the consumer doesn't specify which variant to use.
**Example:**

```rust
// Common pattern for both connectors
pub fn pick_variant<'a>(theme: &'a native_theme::NativeTheme, is_dark: bool) -> Option<&'a native_theme::ThemeVariant> {
    if is_dark {
        theme.dark.as_ref().or(theme.light.as_ref())
    } else {
        theme.light.as_ref().or(theme.dark.as_ref())
    }
}
```

### Pattern 5: Theme Selector Dropdown (CONN-09)

**What:** A dropdown widget listing all presets plus "OS Theme" that reloads the theme on selection.
**When to use:** In both example apps.
**Example (iced):**

```rust
// native-theme-iced/examples/demo.rs
use iced::widget::pick_list;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ThemeChoice {
    OsTheme,
    Preset(String),
}

impl std::fmt::Display for ThemeChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeChoice::OsTheme => write!(f, "OS Theme"),
            ThemeChoice::Preset(name) => write!(f, "{name}"),
        }
    }
}

fn theme_choices() -> Vec<ThemeChoice> {
    let mut choices = vec![ThemeChoice::OsTheme];
    for name in native_theme::NativeTheme::list_presets() {
        choices.push(ThemeChoice::Preset(name.to_string()));
    }
    choices
}
```

### Anti-Patterns to Avoid

- **Intermediate data model:** Do NOT create a `ConnectorTheme` struct between NativeTheme and toolkit types. Map directly.
- **Hardcoded fallback colors in mapping functions:** Use the toolkit's defaults as fallback (e.g., `iced::Color::BLACK` for missing text), NOT custom constants scattered through the code.
- **Blocking on OS theme in examples:** Use `NativeTheme::preset("default")` as initial fallback in examples; OS theme reading may fail or be slow.
- **Exhaustive match on all 108 gpui-component fields in one function:** Split into logical groups (interactive states, list/table, sidebar, charts, etc.) for maintainability.
- **Using iced 0.13:** The project should use 0.14 which has warning in Palette and improved color generation.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| sRGB to HSL conversion for gpui | Custom conversion math | gpui's `Hsla::from()` or `gpui::rgb()` / `gpui::hsla()` | gpui has built-in color conversion; avoid floating-point edge cases |
| Extended palette derivation for iced | Custom shade generation for all Extended sub-palettes | `Extended::generate(palette)` + selective overrides | iced's generator handles perceptual uniformity via Oklch |
| Theme persistence/switching | Custom theme state management | Toolkit's built-in theme system (iced `Theme`, gpui `Theme::global_mut`) | Both toolkits have opinionated theme lifecycle management |
| Widget gallery layout | Custom layout system | Toolkit's standard layout widgets (iced Column/Row, gpui v_flex/h_flex) | Standard layout is well-tested and documented |

**Key insight:** The connectors are mapping layers, not theme engines. Both toolkits already have sophisticated theme infrastructure -- connectors just feed native-theme data into it.

## Common Pitfalls

### Pitfall 1: Color Space Mismatch (gpui)

**What goes wrong:** gpui-component uses `Hsla` (hue/saturation/lightness/alpha) while native-theme uses sRGB `Rgba` (u8 per channel). Direct component-wise mapping produces wrong colors.
**Why it happens:** Confusing sRGB with HSL; both have 4 components but represent color differently.
**How to avoid:** Use gpui's `gpui::rgb()` or `gpui::rgba()` constructors which accept sRGB values and handle conversion internally. Or convert via `Rgba::to_f32_array()` and construct `Hsla` from the sRGB float values using gpui's conversion.
**Warning signs:** Colors look desaturated or hue-shifted compared to the source theme.

### Pitfall 2: Missing Hover/Active State Derivation (gpui)

**What goes wrong:** Setting `primary` but leaving `primary_hover` and `primary_active` at defaults, producing jarring visual transitions.
**Why it happens:** native-theme has 36 colors; gpui-component has 108 with hover/active variants for every interactive element.
**How to avoid:** For every base color mapped, also compute and set its hover (slight lighten/darken) and active (more lighten/darken) variants. Use a consistent derivation function.
**Warning signs:** Buttons look correct at rest but flash to wrong color on hover.

### Pitfall 3: iced Extended Palette Not Matching (iced)

**What goes wrong:** The auto-generated `Extended` palette from the base `Palette` produces colors that don't match the source theme's intent (e.g., secondary colors are computed from background, not from native-theme's `secondary_background`).
**Why it happens:** iced's `Extended::generate()` derives secondary, warning sub-palettes algorithmically. It cannot know about native-theme's explicit secondary/info colors.
**How to avoid:** Use `Theme::custom_with_fn()` and override the generated Extended palette's `secondary.base`, `warning.base` etc. with native-theme values where available.
**Warning signs:** Secondary buttons or warning containers show algorithmically-derived colors instead of the theme's actual secondary/warning colors.

### Pitfall 4: Non-Exhaustive Struct Construction Outside Crate

**What goes wrong:** Cannot construct `ThemeColors { accent: Some(...), ..Default::default() }` from outside the native-theme crate because of `#[non_exhaustive]`.
**Why it happens:** All native-theme model structs are `#[non_exhaustive]`. Connector crates are separate crates.
**How to avoid:** Connector crates only READ fields from native-theme types, they never construct them. Pattern: `variant.colors.accent` to read, never `ThemeColors { ... }` to write.
**Warning signs:** Compiler error about non-exhaustive struct construction.

### Pitfall 5: gpui-component API Instability

**What goes wrong:** gpui-component is pre-1.0 (0.5.x), its ThemeColor struct may change between minor versions, breaking the connector.
**Why it happens:** Rapid development of both gpui and gpui-component.
**How to avoid:** Pin to a specific gpui-component version (`= "0.5"` or `">=0.5, <0.6"`). Document the pinned version. Include a version compatibility note.
**Warning signs:** CI failures after dependency update.

### Pitfall 6: iced Platform Feature Flags

**What goes wrong:** iced example doesn't compile because wrong windowing backend is selected.
**Why it happens:** iced 0.14 has multiple backend features (winit, wgpu, tiny-skia).
**How to avoid:** Use default iced features for the example. Document required features in example README.
**Warning signs:** Link errors about missing windowing or rendering symbols.

## Code Examples

### gpui-component: Rgba to Hsla Conversion

```rust
// Source: gpui crate's color constructors
use native_theme::Rgba;

fn rgba_to_hsla(rgba: Option<Rgba>, fallback: gpui::Hsla) -> gpui::Hsla {
    match rgba {
        Some(c) => {
            let [r, g, b, a] = c.to_f32_array();
            // gpui::rgba_value takes a u32 0xRRGGBBAA
            gpui::Rgba { r, g, b, a }.into()
        }
        None => fallback,
    }
}
```

### gpui-component: ThemeColor Mapping (Core Fields)

```rust
// Source: ThemeColor fields from docs.rs/gpui-component/0.5.0
fn build_theme_color(variant: &native_theme::ThemeVariant) -> gpui_component::theme::ThemeColor {
    let colors = &variant.colors;
    let is_dark = /* determine from background luminance */;

    let bg = rgba_to_hsla(colors.background, gpui::hsla(0.0, 0.0, 1.0, 1.0));
    let fg = rgba_to_hsla(colors.foreground, gpui::hsla(0.0, 0.0, 0.0, 1.0));
    let accent = rgba_to_hsla(colors.accent, gpui::hsla(0.6, 0.7, 0.5, 1.0));
    let primary = rgba_to_hsla(colors.primary_background, accent);
    let danger_base = rgba_to_hsla(colors.danger, gpui::hsla(0.0, 0.8, 0.5, 1.0));

    ThemeColor {
        background: bg,
        foreground: fg,
        accent: accent,
        accent_foreground: rgba_to_hsla(colors.foreground, fg),
        muted: rgba_to_hsla(colors.muted, lighten(bg, 0.1)),
        muted_foreground: rgba_to_hsla(colors.muted, darken(fg, 0.3)),
        primary: primary,
        primary_hover: hover_color(primary, is_dark),
        primary_active: active_color(primary, is_dark),
        primary_foreground: rgba_to_hsla(colors.primary_foreground, gpui::hsla(0.0, 0.0, 1.0, 1.0)),
        danger: danger_base,
        danger_hover: hover_color(danger_base, is_dark),
        danger_active: active_color(danger_base, is_dark),
        danger_foreground: rgba_to_hsla(colors.danger_foreground, gpui::hsla(0.0, 0.0, 1.0, 1.0)),
        border: rgba_to_hsla(colors.border, lighten(bg, 0.2)),
        input: rgba_to_hsla(colors.input, bg),
        ring: rgba_to_hsla(colors.focus_ring, accent),
        selection: rgba_to_hsla(colors.selection, lighten(accent, 0.3)),
        link: rgba_to_hsla(colors.link, accent),
        link_hover: hover_color(rgba_to_hsla(colors.link, accent), is_dark),
        link_active: active_color(rgba_to_hsla(colors.link, accent), is_dark),
        // ... 80+ more fields derived or mapped
        ..ThemeColor::default()  // if ThemeColor implements Default
    }
}
```

### iced: Custom Theme with Widget Catalog Implementation

```rust
// Source: iced 0.14 docs - button::Catalog trait
use iced::widget::button;

// The connector's custom theme wraps iced::Theme but adds widget metrics
pub struct NativeIcedTheme {
    inner: iced::Theme,
    metrics: Option<native_theme::WidgetMetrics>,
}

impl button::Catalog for NativeIcedTheme {
    type Class<'a> = button::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme, status| {
            // Delegate to inner theme's default button style
            let inner_style = theme.inner.style(
                &button::Catalog::default::<iced::Theme>(),
                status,
            );
            // Apply widget metrics if available
            if let Some(ref metrics) = theme.metrics {
                if let Some(ref button) = metrics.button {
                    // Metrics can influence padding, min-size via the style
                }
            }
            inner_style
        })
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        class(self, status)
    }
}
```

### iced: Theme Selector Dropdown Example

```rust
// Source: iced 0.14 pick_list widget
use iced::widget::{column, pick_list, text};

#[derive(Debug, Clone)]
enum Message {
    ThemeSelected(ThemeChoice),
}

fn view(state: &AppState) -> iced::Element<Message> {
    let selector = pick_list(
        theme_choices(),
        Some(state.current_choice.clone()),
        Message::ThemeSelected,
    );

    column![
        text("Theme Selector").size(20),
        selector,
        // ... widget gallery below
    ]
    .spacing(12)
    .into()
}
```

## gpui-component ThemeColor Mapping Table

### Direct Mappings (36 native-theme -> gpui-component)

| native-theme field | gpui-component ThemeColor field | Notes |
|--------------------|-------------------------------|-------|
| `background` | `background` | Direct |
| `foreground` | `foreground` | Direct |
| `accent` | `accent` | Direct |
| `foreground` | `accent_foreground` | Reuse foreground |
| `muted` | `muted` | Direct |
| `muted` | `muted_foreground` | Derived (darker muted for light, lighter for dark) |
| `primary_background` | `primary` | Direct |
| `primary_foreground` | `primary_foreground` | Direct |
| `secondary_background` | `secondary` | Direct |
| `secondary_foreground` | `secondary_foreground` | Direct |
| `danger` | `danger` | Direct |
| `danger_foreground` | `danger_foreground` | Direct |
| `success` | `success` | Direct |
| `success_foreground` | `success_foreground` | Direct |
| `warning` | `warning` | Direct |
| `warning_foreground` | `warning_foreground` | Direct |
| `info` | `info` | Direct |
| `info_foreground` | `info_foreground` | Direct |
| `border` | `border` | Direct |
| `input` | `input` | Direct |
| `focus_ring` | `ring` | Direct |
| `selection` | `selection` | Direct |
| `link` | `link` | Direct |
| `sidebar` | `sidebar` | Direct |
| `sidebar_foreground` | `sidebar_foreground` | Direct |
| `popover` | `popover` | Direct |
| `popover_foreground` | `popover_foreground` | Direct |
| `tooltip` | (none -- gpui uses popover) | Map to popover if no tooltip field |
| `button` | (none -- gpui buttons use primary/secondary) | Map to secondary |
| `separator` | `border` | Fallback to border |

### Derived Mappings (remaining ~72 gpui-component fields)

| gpui-component field | Derivation from native-theme | Method |
|---------------------|------------------------------|--------|
| `primary_hover` | lighten/darken primary | `hover_color(primary)` |
| `primary_active` | lighten/darken primary more | `active_color(primary)` |
| `secondary_hover` | lighten/darken secondary | `hover_color(secondary)` |
| `secondary_active` | lighten/darken secondary more | `active_color(secondary)` |
| `danger_hover` | lighten/darken danger | `hover_color(danger)` |
| `danger_active` | lighten/darken danger more | `active_color(danger)` |
| `success_hover` | lighten/darken success | `hover_color(success)` |
| `success_active` | lighten/darken success more | `active_color(success)` |
| `warning_hover` | lighten/darken warning | `hover_color(warning)` |
| `warning_active` | lighten/darken warning more | `active_color(warning)` |
| `info_hover` | lighten/darken info | `hover_color(info)` |
| `info_active` | lighten/darken info more | `active_color(info)` |
| `list` | background | Use background |
| `list_hover` | lighten/darken background | Slight adjust |
| `list_active` | selection or accent lightened | Use selection |
| `list_even` | alternate_row or slight bg adjust | Use alternate_row |
| `list_head` | surface or slight bg adjust | Use surface |
| `list_active_border` | accent | Use accent |
| `table` | background | Same as list |
| `table_hover` through `table_active_border` | Mirror list_* | Same derivation |
| `table_head` | surface | Use surface |
| `table_head_foreground` | foreground | Use foreground |
| `table_row_border` | border with lower opacity | Adjust alpha |
| `tab` | surface | Use surface |
| `tab_active` | background | Use background |
| `tab_foreground` | muted | Use muted |
| `tab_active_foreground` | foreground | Use foreground |
| `tab_bar` | surface | Use surface |
| `tab_bar_segmented` | surface lighter/darker | Adjust surface |
| `sidebar_primary` | primary_background | Use primary |
| `sidebar_primary_foreground` | primary_foreground | Use primary_foreground |
| `sidebar_accent` | accent | Use accent |
| `sidebar_accent_foreground` | foreground | Use foreground |
| `sidebar_border` | border | Use border |
| `title_bar` | surface | Use surface |
| `title_bar_border` | border | Use border |
| `window_border` | border | Use border |
| `accordion` | background | Use background |
| `accordion_hover` | lighten/darken background | Slight adjust |
| `overlay` | shadow or background with alpha | Semi-transparent |
| `caret` | foreground | Use foreground |
| `scrollbar` | transparent or background | Low-opacity background |
| `scrollbar_thumb` | muted or foreground with alpha | 30% foreground |
| `scrollbar_thumb_hover` | foreground with more alpha | 50% foreground |
| `slider_bar` | border or muted | Use muted |
| `slider_thumb` | accent | Use accent |
| `switch` | muted | Use muted |
| `switch_thumb` | background | Use background |
| `progress_bar` | accent | Use accent |
| `skeleton` | muted with alpha | 20% muted |
| `tiles` | surface | Use surface |
| `drag_border` | accent | Use accent |
| `drop_target` | accent with alpha | 20% accent |
| `chart_1` through `chart_5` | Distribute around color wheel from accent | Hue rotation |
| `bullish` | success | Use success |
| `bearish` | danger | Use danger |
| `group_box` | surface | Use surface |
| `group_box_foreground` | foreground | Use foreground |
| `description_list_label` | surface | Use surface |
| `description_list_label_foreground` | muted | Use muted |
| `link_active` | active_color(link) | Darken/lighten link |
| `link_hover` | hover_color(link) | Darken/lighten link |
| Base colors (red, green, blue, etc.) | From status colors or defaults | danger->red, success->green, info->blue, warning->yellow |

## iced Palette + Extended Mapping

### Palette (6 fields)

| iced Palette field | native-theme source | Fallback |
|-------------------|---------------------|----------|
| `background` | `colors.background` | `Color::WHITE` |
| `text` | `colors.foreground` | `Color::BLACK` |
| `primary` | `colors.accent` | iced default blue |
| `success` | `colors.success` | iced default green |
| `warning` | `colors.warning` | iced default orange |
| `danger` | `colors.danger` | iced default red |

### Extended Palette Overrides

After `Extended::generate(palette)`, override these with native-theme data when available:

| Extended field | native-theme source | Notes |
|---------------|---------------------|-------|
| `secondary.base.color` | `colors.secondary_background` | Override auto-derived secondary |
| `secondary.base.text` | `colors.secondary_foreground` | Override auto-derived secondary text |
| `background.weak.color` | `colors.surface` | Surface is semantically "weak background" |
| `background.weak.text` | `colors.foreground` | Maintain readability |

### Per-Widget Catalog Style Mapping

| Widget | Catalog Trait | Style Fields to Map | native-theme Source |
|--------|--------------|--------------------|--------------------|
| Button | `button::Catalog` | background, text_color, border, shadow | colors.button/accent, colors.button_foreground, geometry.radius, geometry.shadow |
| Container | `container::Catalog` | text_color, background, border, shadow | colors.surface, colors.foreground, geometry.radius |
| TextInput | `text_input::Catalog` | background, border, icon, placeholder, value, selection | colors.input, colors.border, colors.muted, colors.input_foreground, colors.selection |
| Scrollable | `scrollable::Catalog` | background, border, scroller color | colors.background, geometry (scrollbar from WidgetMetrics.scrollbar) |
| Checkbox | `checkbox::Catalog` | background, icon_color, border, text_color | colors.accent (checked), colors.input (unchecked), colors.foreground |
| Slider | `slider::Catalog` | rail colors, handle | colors.accent (filled), colors.muted (unfilled), geometry |
| ProgressBar | `progress_bar::Catalog` | background, bar | colors.muted (track), colors.accent (fill) |
| Tooltip | `tooltip::Catalog` | (delegates to container) | colors.tooltip, colors.tooltip_foreground |

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| iced StyleSheet trait | iced Catalog trait | iced 0.12 -> 0.13 | Connectors use Catalog, not StyleSheet |
| iced Palette without warning | Palette with warning field | iced 0.13 -> 0.14 | Can map native-theme warning color directly |
| iced RGB palette generation | Oklch palette generation | iced 0.14 | Better perceptual uniformity in auto-derived colors |
| gpui-component git dependency | crates.io published | 2025 | Can use crates.io version instead of git |

**Deprecated/outdated:**
- `iced::widget::theme::Custom::new()` -- replaced by `Theme::custom()` and `Theme::custom_with_fn()` in 0.14
- `StyleSheet` trait for iced widgets -- replaced by `Catalog` trait

## Open Questions

1. **gpui-component ThemeColor default constructor**
   - What we know: ThemeColor has 108 fields, all `Hsla`. ThemeConfigColors has 97 optional fields.
   - What's unclear: Whether `ThemeColor` implements `Default` or if you must set all 108 fields. The `Theme::from(&ThemeColor)` constructor suggests full initialization is needed.
   - Recommendation: Check at implementation time. If no Default, start from an existing built-in theme's ThemeColor and override fields.

2. **gpui platform limitations**
   - What we know: gpui works on macOS and Linux. Windows support is in progress.
   - What's unclear: Whether the gpui connector example can run on all CI platforms.
   - Recommendation: Make the gpui connector crate compile on all platforms but mark the example as macOS/Linux only. Use `#[cfg]` guards if needed.

3. **iced Catalog: custom theme type vs iced::Theme**
   - What we know: The requirement says "implements per-widget Catalog/Style for 8 core widgets." iced's built-in `Theme` already implements all Catalog traits.
   - What's unclear: Whether we need a wrapper type that implements Catalog, or whether we can use `Theme::custom_with_fn()` which already delegates to the built-in Catalog impls.
   - Recommendation: Start with `Theme::custom_with_fn()` which generates a custom theme that uses iced's built-in Catalog implementations. Only create a wrapper type if the built-in styling is insufficient for mapping widget metrics (padding, border radius) to per-widget styles. The Catalog trait's `style()` method returns `Style` structs which include `border`, `shadow`, etc., so the built-in implementation from the custom palette should handle most cases. Widget metrics (like padding, min-size) are typically set on the widget itself, not through the Catalog.

4. **Upstream PR proposals for gpui-component (CONN-03)**
   - What we know: gpui-component has `ThemeConfig` with radius, shadow, fonts. It does NOT expose per-widget metric configuration (no button min-height, no scrollbar width in ThemeConfig).
   - What's unclear: Whether gpui-component's internal widget rendering already reads from ThemeColor/Theme for sizing, or if sizes are hardcoded.
   - Recommendation: The proposal documents should identify: (1) per-widget metrics that gpui-component currently hardcodes, (2) proposed ThemeConfig extensions for those metrics, (3) how native-theme-gpui would use them. This is a documentation deliverable, not a code deliverable.

5. **Widget metrics application in iced**
   - What we know: iced's `Catalog` trait returns `Style` structs that control visual appearance (colors, borders, shadows). Widget metrics like min-height and padding are typically set on the widget instance, not through the theme.
   - What's unclear: How to apply widget metrics globally through theming vs. per-widget.
   - Recommendation: The iced connector should provide helper functions that set widget properties (e.g., `button("label").padding(metrics.button.padding_horizontal)`) rather than trying to inject them through the Catalog. Document this pattern in the example.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `cargo test` (built-in) |
| Config file | Workspace Cargo.toml |
| Quick run command | `cargo test -p native-theme-iced` / `cargo test -p native-theme-gpui` |
| Full suite command | `cargo test --workspace` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CONN-01 | ThemeColors -> ThemeColor 108 fields | unit | `cargo test -p native-theme-gpui colors` | Wave 0 |
| CONN-02 | Fonts/geometry/spacing -> ThemeConfig | unit | `cargo test -p native-theme-gpui config` | Wave 0 |
| CONN-03 | Upstream PR proposal docs | manual-only | N/A (documentation review) | Wave 0 |
| CONN-04 | showcase.rs compiles and runs | smoke | `cargo build -p native-theme-gpui --example showcase` | Wave 0 |
| CONN-05 | ThemeColors -> iced Palette | unit | `cargo test -p native-theme-iced palette` | Wave 0 |
| CONN-06 | Per-widget Catalog/Style for 8 widgets | unit | `cargo test -p native-theme-iced catalog` | Wave 0 |
| CONN-07 | Geometry/spacing/metrics -> Style fields | unit | `cargo test -p native-theme-iced style_mapping` | Wave 0 |
| CONN-08 | demo.rs compiles and runs | smoke | `cargo build -p native-theme-iced --example demo` | Wave 0 |
| CONN-09 | Theme selector in both examples | manual-only | N/A (visual inspection) | Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme-{iced,gpui}`
- **Per wave merge:** `cargo test --workspace`
- **Phase gate:** Full suite green before verification

### Wave 0 Gaps
- [ ] `connectors/native-theme-iced/src/palette.rs` tests -- covers CONN-05 (all 6 Palette fields mapped correctly)
- [ ] `connectors/native-theme-iced/src/catalog.rs` tests -- covers CONN-06 (8 widget Catalog impls)
- [ ] `connectors/native-theme-iced/src/extended.rs` tests -- covers CONN-05 (Extended overrides)
- [ ] `connectors/native-theme-iced/examples/demo.rs` -- covers CONN-08, CONN-09
- [ ] `connectors/native-theme-gpui/src/colors.rs` tests -- covers CONN-01 (108 field mapping)
- [ ] `connectors/native-theme-gpui/src/config.rs` tests -- covers CONN-02 (ThemeConfig mapping)
- [ ] `connectors/native-theme-gpui/examples/showcase.rs` -- covers CONN-04, CONN-09
- [ ] `connectors/native-theme-gpui/proposals/README.md` -- covers CONN-03

## Sources

### Primary (HIGH confidence)
- [iced Extended palette docs](https://docs.rs/iced/latest/iced/theme/palette/struct.Extended.html) - Extended struct fields, generate() function
- [iced Palette docs](https://docs.rs/iced/latest/iced/theme/palette/struct.Palette.html) - 6-field Palette struct
- [iced Theme enum docs](https://docs.rs/iced/latest/iced/enum.Theme.html) - custom(), custom_with_fn(), 23 variants
- [iced 0.14.0 release notes](https://github.com/iced-rs/iced/releases/tag/0.14.0) - warning added to Palette, Oklch generation
- [iced palette.rs source](https://docs.iced.rs/src/iced_core/theme/palette.rs.html) - Background/Primary/Secondary sub-structs
- [iced button::Style docs](https://docs.rs/iced/latest/iced/widget/button/struct.Style.html) - background, text_color, border, shadow, snap
- [iced container::Style docs](https://docs.rs/iced/0.14.0/iced/widget/container/struct.Style.html) - text_color, background, border, shadow, snap
- [iced text_input::Style docs](https://docs.rs/iced/0.14.0/iced/widget/text_input/struct.Style.html) - background, border, icon, placeholder, value, selection
- [iced checkbox::Style docs](https://docs.rs/iced/0.14.0/iced/widget/checkbox/struct.Style.html) - background, icon_color, border, text_color
- [iced slider::Style docs](https://docs.rs/iced/0.14.0/iced/widget/slider/struct.Style.html) - rail, handle
- Existing codebase: full source analysis of native-theme model types (HIGH confidence)

### Secondary (MEDIUM confidence)
- [gpui-component ThemeColor docs](https://docs.rs/gpui-component/0.5.0/gpui_component/theme/struct.ThemeColor.html) - 108 Hsla fields (verified via docs.rs)
- [gpui-component ThemeConfig docs](https://docs.rs/gpui-component/0.5.0/gpui_component/theme/struct.ThemeConfig.html) - 12 fields including fonts, radius, shadow
- [gpui-component ThemeConfigColors docs](https://docs.rs/gpui-component/0.5.0/gpui_component/theme/struct.ThemeConfigColors.html) - 97 optional color fields
- [gpui-component theme page](https://longbridge.github.io/gpui-component/docs/theme) - ActiveTheme trait, Theme::global_mut, ThemeRegistry
- [gpui-component Theme docs](https://docs.rs/gpui-component/0.5.0/gpui_component/theme/struct.Theme.html) - 17 fields including colors, fonts, radius
- [iced styling reference](https://austinmreppert.github.io/iced-reference/chapter_3.html) - Catalog trait pattern explanation

### Tertiary (LOW confidence)
- gpui-component internal widget rendering and metric usage -- could not verify whether widget sizes come from ThemeColor or are hardcoded
- Exact gpui `Hsla` conversion from sRGB -- assumed via `gpui::Rgba { r, g, b, a }.into()` but not verified in gpui source

## Metadata

**Confidence breakdown:**
- iced connector: HIGH - API well-documented via docs.rs, version 0.14 confirmed, Palette/Extended/Catalog all verified
- gpui connector: MEDIUM - ThemeColor struct verified (108 fields), but internal widget rendering patterns and exact conversion APIs unverified
- Architecture patterns: HIGH - consistent with existing connector architecture from project ARCHITECTURE.md
- Pitfalls: MEDIUM - based on domain knowledge of color space issues and pre-1.0 API instability
- Widget metrics application: LOW - unclear how toolkit-side metrics application works in practice for both toolkits

**Research date:** 2026-03-08
**Valid until:** 2026-03-22 (14 days - gpui-component is fast-moving; iced 0.14 is stable)
