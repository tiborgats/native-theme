# Proposal: Per-Widget Metric Configuration in ThemeConfig

**Target:** gpui-component (https://github.com/longbridge/gpui-component)
**Author:** native-theme project
**Status:** Draft proposal

## Background

gpui-component's `ThemeConfig` currently supports the following theming controls:

- `font_family` / `mono_font_family` -- UI and monospace font families
- `font_size` / `mono_font_size` -- UI and monospace font sizes
- `radius` / `radius_lg` -- border radius for general and large elements
- `shadow` -- enable/disable shadows on interactive elements

These fields allow theme authors to control typography, corner rounding, and shadow
behavior. However, per-widget sizing metrics -- button height, checkbox indicator
size, input padding, scrollbar width, slider track dimensions, etc. -- appear to be
hardcoded within individual widget implementations.

## Problem

Theme systems like [native-theme](https://github.com/nickmass/native-theme) extract
per-widget metrics from operating system theme data:

- **Windows:** `GetSystemMetricsForDpi()` returns exact pixel values for scrollbar
  width, checkbox size, button minimum dimensions, etc.
- **Linux/KDE:** Breeze's `breezemetrics.h` defines constants like `CheckBox_Size`,
  `ScrollBar_Width`, `MenuItem_MarginWidth`, `TabBar_TabMinHeight`.
- **Linux/GNOME:** Adwaita's CSS defines `min-height`, `min-width`, `padding` for
  every widget class.
- **macOS:** Apple's Human Interface Guidelines specify control sizes (Regular, Small,
  Mini) with exact dimensions.

The `native-theme` crate captures these values in a `WidgetMetrics` struct containing
12 sub-structs:

| Sub-struct | Fields | Example values |
|------------|--------|----------------|
| `ButtonMetrics` | min_width, min_height, padding_horizontal, padding_vertical, icon_spacing | 80px, 30px, 6px, 4px, 4px |
| `CheckboxMetrics` | indicator_size, spacing | 20px, 4px |
| `InputMetrics` | min_height, padding_horizontal, padding_vertical | 30px, 6px, 4px |
| `ScrollbarMetrics` | width, min_thumb_height, slider_width | 21px, 20px, 8px |
| `SliderMetrics` | track_height, thumb_size, tick_length | 6px, 20px, 8px |
| `ProgressBarMetrics` | height, min_width | 6px, 14px |
| `TabMetrics` | min_width, min_height, padding_horizontal, padding_vertical | 80px, 30px, 8px, 4px |
| `MenuItemMetrics` | height, padding_horizontal, padding_vertical, icon_spacing | 22px, 4px, 4px, 8px |
| `TooltipMetrics` | padding, max_width | 3px, 300px |
| `ListItemMetrics` | height, padding_horizontal, padding_vertical | 24px, 2px, 1px |
| `ToolbarMetrics` | height, item_spacing, padding | 38px, 0px, 6px |
| `SplitterMetrics` | width | 1px |

Without `ThemeConfig` hooks for these values, connector crates like `native-theme-gpui`
cannot apply native widget sizing -- they can only map colors, fonts, and border radius.

## Proposed Changes

Add optional per-widget metric fields to `ThemeConfig`. All new fields are `Option<f32>`
so existing themes are unaffected (None falls back to current hardcoded defaults).

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ThemeConfig {
    // ... existing fields ...

    // Button metrics
    #[serde(rename = "button.height")]
    pub button_height: Option<f32>,
    #[serde(rename = "button.padding_horizontal")]
    pub button_padding_horizontal: Option<f32>,

    // Checkbox metrics
    #[serde(rename = "checkbox.size")]
    pub checkbox_size: Option<f32>,
    #[serde(rename = "checkbox.indicator_size")]
    pub checkbox_indicator_size: Option<f32>,

    // Input metrics
    #[serde(rename = "input.height")]
    pub input_height: Option<f32>,
    #[serde(rename = "input.padding_horizontal")]
    pub input_padding_horizontal: Option<f32>,

    // Scrollbar metrics
    #[serde(rename = "scrollbar.width")]
    pub scrollbar_width: Option<f32>,

    // Slider metrics
    #[serde(rename = "slider.track_height")]
    pub slider_track_height: Option<f32>,
    #[serde(rename = "slider.thumb_size")]
    pub slider_thumb_size: Option<f32>,

    // Tab metrics
    #[serde(rename = "tab.height")]
    pub tab_height: Option<f32>,

    // Tooltip metrics
    #[serde(rename = "tooltip.padding")]
    pub tooltip_padding: Option<f32>,
}
```

## How native-theme-gpui Would Use This

With these fields available, the connector's `to_theme_config()` function would map
directly from `WidgetMetrics` sub-structs:

```rust
pub fn to_theme_config(variant: &ThemeVariant, name: &str) -> ThemeConfig {
    let wm = variant.widget_metrics.as_ref();

    ThemeConfig {
        // ... existing font/radius/shadow mappings ...

        button_height: wm.and_then(|m| m.button.min_height),
        button_padding_horizontal: wm.and_then(|m| m.button.padding_horizontal),
        checkbox_size: wm.and_then(|m| m.checkbox.indicator_size),
        scrollbar_width: wm.and_then(|m| m.scrollbar.width),
        slider_track_height: wm.and_then(|m| m.slider.track_height),
        slider_thumb_size: wm.and_then(|m| m.slider.thumb_size),
        tab_height: wm.and_then(|m| m.tab.min_height),
        tooltip_padding: wm.and_then(|m| m.tooltip.padding),
        // ...
    }
}
```

This would allow gpui-component applications using native-theme-gpui to render
widgets with OS-native sizing automatically, without manual per-widget configuration.

## Widget Implementation Changes

Each widget that currently uses hardcoded sizes would need to read from the theme:

```rust
// Before (hardcoded):
impl Button {
    fn height(&self, _cx: &App) -> Pixels {
        px(32.)
    }
}

// After (theme-aware):
impl Button {
    fn height(&self, cx: &App) -> Pixels {
        cx.theme().button_height.unwrap_or(px(32.))
    }
}
```

This pattern preserves backward compatibility: existing themes without metric
overrides continue to use the current defaults.

## Compatibility

- **Fully backward-compatible:** All new fields are `Option<f32>` with `#[serde(default)]`.
  Existing JSON theme files parse without changes.
- **No breaking API changes:** The `ThemeConfig` struct gains new fields, but all are
  optional with `Default` providing `None`.
- **Incremental adoption:** Widget implementations can adopt metric reading one widget
  at a time. Widgets that have not been updated continue using hardcoded values.

## Alternatives Considered

1. **CSS-like approach:** Allow arbitrary key-value metric overrides in a HashMap.
   Rejected because it loses type safety and discoverability.

2. **Separate WidgetConfig struct:** Create a parallel `WidgetConfig` alongside
   `ThemeConfig`. Rejected because it fragments the theme API and complicates
   `Theme::apply_config`.

3. **Per-widget style overrides only:** Let connector crates set widget sizes via
   style/rendering overrides rather than theme config. This works for some toolkits
   but fights against gpui-component's centralized theme model.

## References

- native-theme WidgetMetrics: 12 sub-structs covering Button, Checkbox, Input,
  Scrollbar, Slider, ProgressBar, Tab, MenuItem, Tooltip, ListItem, Toolbar, Splitter
- gpui-component ThemeConfig: https://docs.rs/gpui-component/0.5.1/gpui_component/theme/struct.ThemeConfig.html
- Windows GetSystemMetricsForDpi: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetricsfordpi
- KDE Breeze metrics: breezemetrics.h in kde/breeze repository
