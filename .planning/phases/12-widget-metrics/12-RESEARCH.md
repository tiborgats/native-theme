# Phase 12: Widget Metrics - Research

**Researched:** 2026-03-08
**Domain:** Per-widget sizing and spacing data model + platform-specific population
**Confidence:** HIGH

## Summary

Phase 12 adds a `WidgetMetrics` struct with 12 per-widget sub-structs to the data model, integrates it into `ThemeVariant`, and populates it from four platform sources: KDE (breezemetrics.h constants), Windows (GetSystemMetricsForDpi), macOS (HIG defaults), and GNOME (libadwaita hardcoded values). Preset TOML files are also updated to include widget metrics.

The codebase already has a well-established pattern for adding new sub-structs to `ThemeVariant` -- see `ThemeColors`, `ThemeFonts`, `ThemeGeometry`, `ThemeSpacing` -- each using `Option<T>` fields, `#[non_exhaustive]`, `serde(default)`, `skip_serializing_none`, and the `impl_merge!` macro. Widget metrics sub-structs must follow this exact same pattern. The data model is purely additive (no breaking changes), and all four platform readers already exist and produce `ThemeVariant` values that just need a new `widget_metrics` field populated.

**Primary recommendation:** Create a new `model/widget_metrics.rs` module with `WidgetMetrics` (top-level) and 12 per-widget sub-structs, following the identical derive/attribute/macro pattern used by `ThemeGeometry` and `ThemeSpacing`. Add to `ThemeVariant` as `widget_metrics: Option<WidgetMetrics>`. Populate from platform-specific hardcoded constants (KDE, macOS, GNOME) and runtime API calls (Windows).

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| METRIC-01 | `WidgetMetrics` struct with 12 per-widget sub-structs | Data model pattern fully documented; all 12 sub-structs and their fields specified below |
| METRIC-02 | Each sub-struct uses `Option<f32>` fields, `#[non_exhaustive]`, serde defaults | Identical pattern to existing `ThemeGeometry`, `ThemeSpacing`; `impl_merge!` macro handles merge+is_empty |
| METRIC-03 | `widget_metrics: Option<WidgetMetrics>` added to `ThemeVariant` | ThemeVariant is `#[non_exhaustive]`, additive field; requires `impl_merge!` update for nested field |
| METRIC-04 | KDE metrics from breezemetrics.h constants | Full constant list extracted from breeze source; 30+ relevant constants mapped to sub-struct fields |
| METRIC-05 | Windows metrics via `GetSystemMetricsForDpi` | SM_ constants documented; `windows` crate already imports `SM_CXVSCROLL`, `SM_CXBORDER`; need additional SM_ imports |
| METRIC-06 | macOS metrics from HIG defaults | Hardcoded values based on AppKit intrinsic sizes (22pt button, overlay scrollbar defaults) |
| METRIC-07 | GNOME metrics from libadwaita values | Hardcoded values based on libadwaita CSS defaults and Adwaita theme conventions |
| METRIC-08 | Widget metrics in preset TOML files | 17 existing presets need `[light.widget_metrics]` / `[dark.widget_metrics]` sections |
</phase_requirements>

## Standard Stack

### Core

This phase uses only the existing crate dependencies -- no new libraries required.

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| serde | workspace | Serialize/deserialize widget metrics | Already used by all model structs |
| serde_with | workspace | `#[skip_serializing_none]` on sub-structs | Already used by ThemeGeometry, ThemeSpacing |
| toml | workspace | TOML round-trip for preset files | Already used for all preset loading |
| windows | >=0.59, <=0.62 | `GetSystemMetricsForDpi` with additional SM_ constants | Already a dependency; needs additional features |

### Supporting

No new supporting libraries needed. All platform readers already exist.

### Windows Crate Feature Additions

The `windows` crate dependency may need the `SM_CYMENU`, `SM_CXMENUCHECK`, `SM_CYMENUCHECK`, `SM_CXHSCROLL`, `SM_CYHSCROLL`, `SM_CXVSCROLL`, `SM_CYVSCROLL`, `SM_CYVTHUMB`, `SM_CXHTHUMB` constants. These are all in `Win32_UI_WindowsAndMessaging` which is already a feature in Cargo.toml. Verify they are available without additional feature flags.

## Architecture Patterns

### Recommended Project Structure

```
native-theme/src/
  model/
    mod.rs                  # Add widget_metrics module, re-export WidgetMetrics
    widget_metrics.rs       # NEW: WidgetMetrics + 12 sub-structs
    colors.rs               # (existing)
    fonts.rs                # (existing)
    geometry.rs             # (existing)
    spacing.rs              # (existing)
  kde/
    mod.rs                  # Add widget_metrics population to from_kde_content()
    metrics.rs              # NEW: KDE breezemetrics.h constants -> sub-structs
    colors.rs               # (existing)
    fonts.rs                # (existing)
  gnome/
    mod.rs                  # Add widget_metrics population to build_theme()
  windows.rs                # Add widget_metrics population to build_theme()
  macos.rs                  # Add widget_metrics population to build_theme()
  presets/
    *.toml                  # 17 presets updated with widget_metrics sections
```

### Pattern 1: Sub-Struct Convention (follow exactly)

**What:** Every sub-struct in the model uses the same derive/attribute/macro pattern.
**When to use:** For every new widget metrics sub-struct.
**Example:**

```rust
// Source: existing ThemeGeometry pattern in model/geometry.rs
use serde::{Deserialize, Serialize};

/// Button sizing and spacing metrics.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ButtonMetrics {
    /// Minimum button width in logical pixels.
    pub min_width: Option<f32>,
    /// Minimum button height in logical pixels.
    pub min_height: Option<f32>,
    /// Horizontal padding inside the button.
    pub padding_horizontal: Option<f32>,
    /// Vertical padding inside the button.
    pub padding_vertical: Option<f32>,
    /// Spacing between icon and label within the button.
    pub icon_spacing: Option<f32>,
}

impl_merge!(ButtonMetrics {
    option { min_width, min_height, padding_horizontal, padding_vertical, icon_spacing }
});
```

### Pattern 2: Top-Level WidgetMetrics (nested struct)

**What:** `WidgetMetrics` contains 12 per-widget sub-structs as nested fields.
**When to use:** Exactly once, for the `WidgetMetrics` top-level struct.
**Example:**

```rust
/// Per-widget sizing and spacing metrics.
///
/// Contains sub-structs for each widget type with platform-specific
/// dimensions. All sub-structs are nested (not Option) because they
/// default to empty (all fields None).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WidgetMetrics {
    #[serde(default, skip_serializing_if = "ButtonMetrics::is_empty")]
    pub button: ButtonMetrics,

    #[serde(default, skip_serializing_if = "CheckboxMetrics::is_empty")]
    pub checkbox: CheckboxMetrics,

    // ... 10 more sub-structs
}

impl_merge!(WidgetMetrics {
    nested { button, checkbox, input, scrollbar, slider, progress_bar,
             tab, menu_item, tooltip, list_item, toolbar, splitter }
});
```

### Pattern 3: ThemeVariant Integration

**What:** Add `widget_metrics` as an `Option<WidgetMetrics>` field to ThemeVariant.
**When to use:** When integrating into the data model.
**Why Option:** Unlike colors/fonts/geometry/spacing (which are always-present nested structs), widget_metrics is new and should be `Option` so existing code and TOML files without it deserialize correctly. This matches the requirement METRIC-03.
**Example:**

```rust
// In model/mod.rs ThemeVariant
#[serde(default, skip_serializing_if = "Option::is_none")]
pub widget_metrics: Option<WidgetMetrics>,
```

**CRITICAL DESIGN DECISION: `Option<WidgetMetrics>` vs bare `WidgetMetrics`.**

Looking at the requirement text "widget_metrics: Option<WidgetMetrics>", this is explicitly specified. Use `Option<WidgetMetrics>`. The `impl_merge!` macro for ThemeVariant will need to handle this as a special case -- it does not fit cleanly into `option {}` (for leaf `Option<T>` fields) or `nested {}` (for always-present struct fields). Handle with a manual merge method or extend the macro with a new `optional_nested {}` category.

Recommended approach: manually implement merge logic for this field outside the macro, or use a pattern like:

```rust
// Manual merge for Option<WidgetMetrics>
match (&mut self.widget_metrics, &overlay.widget_metrics) {
    (Some(base), Some(over)) => base.merge(over),
    (None, Some(over)) => self.widget_metrics = Some(over.clone()),
    _ => {}
}
```

And for `is_empty`:
```rust
// is_empty check
&& self.widget_metrics.as_ref().map_or(true, |wm| wm.is_empty())
```

### Pattern 4: Platform-Specific Hardcoded Constants (KDE)

**What:** Create a helper function that returns a `WidgetMetrics` populated from breezemetrics.h constants.
**When to use:** For KDE, macOS, and GNOME readers where values are compile-time constants.
**Example:**

```rust
// In kde/metrics.rs
pub(crate) fn breeze_widget_metrics() -> crate::model::widget_metrics::WidgetMetrics {
    crate::model::widget_metrics::WidgetMetrics {
        button: crate::model::widget_metrics::ButtonMetrics {
            min_width: Some(80.0),  // Button_MinWidth
            padding_horizontal: Some(6.0),  // Button_MarginWidth
            icon_spacing: Some(4.0),  // Button_ItemSpacing
            ..Default::default()
        },
        checkbox: crate::model::widget_metrics::CheckboxMetrics {
            indicator_size: Some(20.0),  // CheckBox_Size
            spacing: Some(4.0),  // CheckBox_ItemSpacing
            ..Default::default()
        },
        // ... etc
        ..Default::default()
    }
}
```

### Pattern 5: Runtime API Call (Windows)

**What:** Read widget metrics from `GetSystemMetricsForDpi` at runtime.
**When to use:** Windows reader only.
**Example:**

```rust
// In windows.rs
fn read_widget_metrics(dpi: u32) -> crate::model::widget_metrics::WidgetMetrics {
    use windows::Win32::UI::WindowsAndMessaging::*;

    let scrollbar = unsafe {
        crate::model::widget_metrics::ScrollbarMetrics {
            width: Some(GetSystemMetricsForDpi(SM_CXVSCROLL, dpi) as f32),
            min_thumb_height: Some(GetSystemMetricsForDpi(SM_CYVTHUMB, dpi) as f32),
            ..Default::default()
        }
    };

    crate::model::widget_metrics::WidgetMetrics {
        scrollbar,
        // Mix runtime values with WinUI3 Fluent defaults for widgets
        // that don't have system metrics (buttons, checkboxes, etc.)
        button: winui3_button_metrics(),
        ..Default::default()
    }
}
```

### Anti-Patterns to Avoid

- **Making sub-struct fields non-optional:** Every field MUST be `Option<f32>`. Toolkits need to know "was this specified?" vs "use my default."
- **Putting metrics in ThemeGeometry:** Widget-specific metrics are per-widget, not global geometry. They are a separate concern.
- **Using concrete sub-struct types as fields in ThemeVariant:** The requirement specifies `Option<WidgetMetrics>` as a single field, not 12 separate fields on ThemeVariant.
- **Exhaustive structs:** All sub-structs MUST be `#[non_exhaustive]` for future-proofing.
- **Deep nesting in TOML:** Keep TOML path depth reasonable. `[light.widget_metrics.button]` (3 levels) is the max.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Merge logic for Option fields | Custom merge per struct | `impl_merge!` macro | Already handles option+nested merging correctly across the codebase |
| TOML serialization of nested optional structs | Custom serializer | `serde_with::skip_serializing_none` + `serde(default)` | Handles None fields correctly, proven pattern in ThemeGeometry |
| is_empty() checks | Manual per-struct | `impl_merge!` macro generates is_empty | Consistent with existing codebase |

**Key insight:** The codebase already has all the machinery needed. The `impl_merge!` macro, serde attributes, and derive patterns are battle-tested across 4 existing sub-structs. This phase is a matter of applying the same pattern 12 more times.

## Common Pitfalls

### Pitfall 1: Option<WidgetMetrics> Merge Semantics

**What goes wrong:** Using `impl_merge!` with `option {}` for `widget_metrics` treats it as a leaf Option -- an overlay with `Some(WidgetMetrics)` would replace the entire base, not merge recursively.
**Why it happens:** The macro's `option {}` category does `self.field = overlay.field.clone()` when overlay is Some, which is correct for `Option<f32>` but wrong for `Option<WidgetMetrics>`.
**How to avoid:** Handle `widget_metrics` merge manually in ThemeVariant (not via the macro), using the `(Some(base), Some(over)) => base.merge(over)` pattern shown in `NativeTheme::merge()`.
**Warning signs:** Tests pass but overlay widget_metrics silently replaces base instead of merging.

### Pitfall 2: TOML Nesting Depth

**What goes wrong:** `[light.widget_metrics.button]` works, but `[light.widget_metrics.button.inner]` would break usability.
**Why it happens:** Each level of nesting adds a TOML table header.
**How to avoid:** Keep sub-structs flat (no nesting within widget sub-structs). `ButtonMetrics` has direct `Option<f32>` fields, never nested structs.
**Warning signs:** TOML files become hard to read; 4+ levels of table headers.

### Pitfall 3: Missing skip_serializing_if for Widget Sub-Structs

**What goes wrong:** Empty widget metrics sub-structs serialize as empty TOML tables (`[light.widget_metrics.button]` with no content).
**Why it happens:** Without `skip_serializing_if = "ButtonMetrics::is_empty"`, serde serializes the default empty struct.
**How to avoid:** Every sub-struct field in `WidgetMetrics` must have `#[serde(default, skip_serializing_if = "XxxMetrics::is_empty")]`.
**Warning signs:** TOML output contains dozens of empty tables.

### Pitfall 4: Windows SM_ Constants Need Feature Imports

**What goes wrong:** Compiler error when trying to use SM_CYMENU or other constants not currently imported.
**Why it happens:** The `windows` crate's feature flags control which constants are available.
**How to avoid:** Verify all needed SM_ constants are available under `Win32_UI_WindowsAndMessaging` (already enabled). If any require additional features, add them to Cargo.toml.
**Warning signs:** `cannot find value SM_CYMENU in this scope` errors.

### Pitfall 5: Preset TOML Backward Compatibility

**What goes wrong:** Old TOML files fail to parse because new fields are expected.
**Why it happens:** Forgetting `#[serde(default)]` on WidgetMetrics or ThemeVariant.
**How to avoid:** `#[serde(default)]` is already on ThemeVariant and must be on WidgetMetrics and all sub-structs. Test that existing presets still load after model changes.
**Warning signs:** Existing preset tests fail after adding WidgetMetrics struct.

### Pitfall 6: Non-Exhaustive Struct Initialization

**What goes wrong:** Cannot initialize `#[non_exhaustive]` struct with struct literal outside the defining crate.
**Why it happens:** This is within the same crate, so it is fine. But downstream users will need `..Default::default()`.
**How to avoid:** Always document the `..Default::default()` pattern in doc comments. Within the crate, struct literal initialization works.
**Warning signs:** N/A for this phase, but important for downstream toolkit connectors.

## Code Examples

### Per-Widget Sub-Struct Definitions

Based on research, here are the 12 sub-structs with their recommended fields.

#### ButtonMetrics
```rust
pub struct ButtonMetrics {
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub padding_horizontal: Option<f32>,
    pub padding_vertical: Option<f32>,
    pub icon_spacing: Option<f32>,
}
```

#### CheckboxMetrics
```rust
pub struct CheckboxMetrics {
    pub indicator_size: Option<f32>,
    pub spacing: Option<f32>,  // gap between indicator and label
}
```

#### InputMetrics
```rust
pub struct InputMetrics {
    pub min_height: Option<f32>,
    pub padding_horizontal: Option<f32>,
    pub padding_vertical: Option<f32>,
}
```

#### ScrollbarMetrics
```rust
pub struct ScrollbarMetrics {
    pub width: Option<f32>,          // track width
    pub min_thumb_height: Option<f32>,
    pub slider_width: Option<f32>,   // thumb/slider width (may differ from track)
}
```

#### SliderMetrics
```rust
pub struct SliderMetrics {
    pub track_height: Option<f32>,  // groove thickness
    pub thumb_size: Option<f32>,    // control thumb diameter/width
    pub tick_length: Option<f32>,   // tick mark length
}
```

#### ProgressBarMetrics
```rust
pub struct ProgressBarMetrics {
    pub height: Option<f32>,        // bar thickness
    pub min_width: Option<f32>,     // minimum bar width (for indeterminate)
}
```

#### TabMetrics
```rust
pub struct TabMetrics {
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub padding_horizontal: Option<f32>,
    pub padding_vertical: Option<f32>,
}
```

#### MenuItemMetrics
```rust
pub struct MenuItemMetrics {
    pub height: Option<f32>,        // single menu item height
    pub padding_horizontal: Option<f32>,
    pub padding_vertical: Option<f32>,
    pub icon_spacing: Option<f32>,  // gap between icon and label
}
```

#### TooltipMetrics
```rust
pub struct TooltipMetrics {
    pub padding: Option<f32>,       // inner padding
    pub max_width: Option<f32>,     // max tooltip width
}
```

#### ListItemMetrics
```rust
pub struct ListItemMetrics {
    pub height: Option<f32>,        // row height
    pub padding_horizontal: Option<f32>,
    pub padding_vertical: Option<f32>,
}
```

#### ToolbarMetrics
```rust
pub struct ToolbarMetrics {
    pub height: Option<f32>,        // toolbar height
    pub item_spacing: Option<f32>,  // gap between toolbar items
    pub padding: Option<f32>,       // inner padding
}
```

#### SplitterMetrics
```rust
pub struct SplitterMetrics {
    pub width: Option<f32>,         // splitter handle width/thickness
}
```

### KDE breezemetrics.h Mapping (Verified from Source)

Source: https://invent.kde.org/plasma/breeze (breezemetrics.h)

```rust
// All values are in logical pixels (integers from header, cast to f32)
pub(crate) fn breeze_widget_metrics() -> WidgetMetrics {
    WidgetMetrics {
        button: ButtonMetrics {
            min_width: Some(80.0),      // Button_MinWidth
            padding_horizontal: Some(6.0), // Button_MarginWidth
            icon_spacing: Some(4.0),    // Button_ItemSpacing
            ..Default::default()
        },
        checkbox: CheckboxMetrics {
            indicator_size: Some(20.0), // CheckBox_Size
            spacing: Some(4.0),        // CheckBox_ItemSpacing
            ..Default::default()
        },
        input: InputMetrics {
            padding_horizontal: Some(6.0), // LineEdit_FrameWidth
            ..Default::default()
        },
        scrollbar: ScrollbarMetrics {
            width: Some(21.0),          // ScrollBar_Extend
            min_thumb_height: Some(20.0), // ScrollBar_MinSliderHeight
            slider_width: Some(8.0),    // ScrollBar_SliderWidth
        },
        slider: SliderMetrics {
            track_height: Some(6.0),    // Slider_GrooveThickness
            thumb_size: Some(20.0),     // Slider_ControlThickness
            tick_length: Some(8.0),     // Slider_TickLength
        },
        progress_bar: ProgressBarMetrics {
            height: Some(6.0),          // ProgressBar_Thickness
            min_width: Some(14.0),      // ProgressBar_BusyIndicatorSize
        },
        tab: TabMetrics {
            min_width: Some(80.0),      // TabBar_TabMinWidth
            min_height: Some(30.0),     // TabBar_TabMinHeight
            padding_horizontal: Some(8.0), // TabBar_TabMarginWidth
            padding_vertical: Some(4.0),   // TabBar_TabMarginHeight
        },
        menu_item: MenuItemMetrics {
            padding_horizontal: Some(4.0), // MenuItem_MarginWidth
            padding_vertical: Some(4.0),   // MenuItem_MarginHeight
            icon_spacing: Some(8.0),       // MenuItem_TextLeftMargin
            ..Default::default()
        },
        tooltip: TooltipMetrics {
            padding: Some(3.0),         // ToolTip_FrameWidth
            ..Default::default()
        },
        list_item: ListItemMetrics {
            padding_horizontal: Some(2.0), // ItemView_ItemMarginLeft
            padding_vertical: Some(1.0),   // ItemView_ItemMarginTop
            ..Default::default()
        },
        toolbar: ToolbarMetrics {
            item_spacing: Some(0.0),    // ToolBar_ItemSpacing
            padding: Some(6.0),         // ToolBar_ItemMargin
            ..Default::default()
        },
        splitter: SplitterMetrics {
            width: Some(1.0),           // Splitter_SplitterWidth
        },
    }
}
```

### Windows GetSystemMetricsForDpi Mapping

Source: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics

```rust
fn read_widget_metrics(dpi: u32) -> WidgetMetrics {
    use windows::Win32::UI::WindowsAndMessaging::*;

    let scrollbar = unsafe {
        ScrollbarMetrics {
            width: Some(GetSystemMetricsForDpi(SM_CXVSCROLL, dpi) as f32),
            min_thumb_height: Some(GetSystemMetricsForDpi(SM_CYVTHUMB, dpi) as f32),
            ..Default::default()
        }
    };

    let menu_item = unsafe {
        MenuItemMetrics {
            height: Some(GetSystemMetricsForDpi(SM_CYMENU, dpi) as f32),
            ..Default::default()
        }
    };

    // Button, checkbox, slider, progress bar, tab, tooltip, list_item,
    // toolbar, splitter: use WinUI3 Fluent Design hardcoded defaults
    // since Windows doesn't expose system metrics for these widgets.
    WidgetMetrics {
        button: ButtonMetrics {
            min_height: Some(32.0),     // WinUI3 default
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        checkbox: CheckboxMetrics {
            indicator_size: Some(20.0), // WinUI3 default
            spacing: Some(8.0),
            ..Default::default()
        },
        scrollbar,
        menu_item,
        // ... other Fluent defaults
        ..Default::default()
    }
}
```

### macOS HIG Defaults

Source: Apple Human Interface Guidelines, AppKit intrinsic sizes

```rust
pub(crate) fn macos_widget_metrics() -> WidgetMetrics {
    WidgetMetrics {
        button: ButtonMetrics {
            min_height: Some(22.0),     // NSButton regular control size
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        checkbox: CheckboxMetrics {
            indicator_size: Some(14.0), // NSButton switch type
            spacing: Some(4.0),
            ..Default::default()
        },
        input: InputMetrics {
            min_height: Some(22.0),     // NSTextField regular
            padding_horizontal: Some(4.0),
            ..Default::default()
        },
        scrollbar: ScrollbarMetrics {
            width: Some(15.0),          // NSScroller legacy style
            slider_width: Some(7.0),    // Overlay style
            ..Default::default()
        },
        slider: SliderMetrics {
            track_height: Some(4.0),    // NSSlider circular knob
            thumb_size: Some(21.0),
            ..Default::default()
        },
        progress_bar: ProgressBarMetrics {
            height: Some(6.0),          // NSProgressIndicator regular
            ..Default::default()
        },
        tab: TabMetrics {
            min_height: Some(24.0),     // NSTabView
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        menu_item: MenuItemMetrics {
            height: Some(22.0),         // Standard menu item
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        tooltip: TooltipMetrics {
            padding: Some(4.0),
            ..Default::default()
        },
        list_item: ListItemMetrics {
            height: Some(24.0),         // NSTableView row
            padding_horizontal: Some(4.0),
            ..Default::default()
        },
        toolbar: ToolbarMetrics {
            height: Some(38.0),         // NSToolbar standard
            item_spacing: Some(8.0),
            ..Default::default()
        },
        splitter: SplitterMetrics {
            width: Some(9.0),           // NSSplitView divider
        },
    }
}
```

### GNOME libadwaita Defaults

Source: libadwaita CSS defaults and Adwaita theme conventions

```rust
pub(crate) fn adwaita_widget_metrics() -> WidgetMetrics {
    WidgetMetrics {
        button: ButtonMetrics {
            min_height: Some(34.0),     // libadwaita default button
            padding_horizontal: Some(12.0),
            padding_vertical: Some(8.0),
            ..Default::default()
        },
        checkbox: CheckboxMetrics {
            indicator_size: Some(20.0), // GtkCheckButton indicator
            spacing: Some(8.0),
            ..Default::default()
        },
        input: InputMetrics {
            min_height: Some(34.0),     // GtkEntry
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        scrollbar: ScrollbarMetrics {
            width: Some(12.0),          // Adwaita overlay scrollbar
            slider_width: Some(8.0),
            ..Default::default()
        },
        slider: SliderMetrics {
            track_height: Some(6.0),    // GtkScale trough
            thumb_size: Some(20.0),
            ..Default::default()
        },
        progress_bar: ProgressBarMetrics {
            height: Some(6.0),          // GtkProgressBar
            ..Default::default()
        },
        tab: TabMetrics {
            min_height: Some(34.0),     // AdwTabBar
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        menu_item: MenuItemMetrics {
            height: Some(34.0),         // GtkPopoverMenuBar
            padding_horizontal: Some(8.0),
            padding_vertical: Some(4.0),
            ..Default::default()
        },
        tooltip: TooltipMetrics {
            padding: Some(6.0),         // GtkTooltip
            ..Default::default()
        },
        list_item: ListItemMetrics {
            padding_horizontal: Some(12.0),
            padding_vertical: Some(8.0),
            ..Default::default()
        },
        toolbar: ToolbarMetrics {
            height: Some(46.0),         // AdwHeaderBar default
            item_spacing: Some(6.0),
            ..Default::default()
        },
        splitter: SplitterMetrics {
            width: Some(6.0),           // GtkPaned
        },
    }
}
```

### TOML Preset Format

```toml
# In adwaita.toml [light] section, add:
[light.widget_metrics.button]
min_height = 34.0
padding_horizontal = 12.0
padding_vertical = 8.0

[light.widget_metrics.checkbox]
indicator_size = 20.0
spacing = 8.0

[light.widget_metrics.scrollbar]
width = 12.0
slider_width = 8.0

# ... etc for each widget type
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Single global geometry values | Per-widget metrics sub-structs | This phase | Enables pixel-perfect native rendering per widget |
| Hardcoded toolkit defaults | Platform-sourced widget dimensions | This phase | Toolkit connectors get accurate dimensions |

**Deprecated/outdated:**
- N/A -- this is net-new functionality

## Open Questions

1. **Exact macOS widget dimensions**
   - What we know: NSButton regular height is 22pt, overlay scrollbar is ~7pt, legacy scrollbar is ~15pt
   - What's unclear: Exact values for some widgets (tab, toolbar, splitter) vary by macOS version and are not well-documented in HIG
   - Recommendation: Use best-known values; these are hardcoded defaults that can be refined. Flag with LOW confidence and allow tuning in future patches.

2. **Exact libadwaita widget dimensions**
   - What we know: Button min-height appears to be ~34px based on community reports. General Adwaita sizing follows GTK4 conventions.
   - What's unclear: GitLab SCSS source was not directly accessible. Values are based on community documentation and GTK4 conventions.
   - Recommendation: Use best-known libadwaita values. These are hardcoded and easily updateable. The adwaita.toml preset already exists to verify visual accuracy.

3. **Which Windows SM_ constants need additional feature flags**
   - What we know: `SM_CXVSCROLL` and `SM_CXBORDER` are already used. `SM_CYMENU`, `SM_CYMENUCHECK`, etc. are in `Win32_UI_WindowsAndMessaging`.
   - What's unclear: Whether all needed constants are available with current Cargo.toml features
   - Recommendation: Verify during implementation. If a constant is unavailable, use WinUI3 Fluent defaults as fallback.

4. **ThemeVariant merge handling for Option<WidgetMetrics>**
   - What we know: The `impl_merge!` macro handles `option {}` (leaf Option<T>) and `nested {}` (always-present struct). `Option<WidgetMetrics>` needs recursive merge semantics.
   - What's unclear: Whether to extend the macro or handle manually
   - Recommendation: Handle manually in ThemeVariant's merge implementation (outside the macro), matching the pattern used in `NativeTheme::merge()` for `Option<ThemeVariant>`.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `cargo test` (built-in, no external test framework) |
| Config file | Workspace Cargo.toml |
| Quick run command | `cargo test -p native-theme` |
| Full suite command | `cargo test` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| METRIC-01 | WidgetMetrics struct with 12 sub-structs | unit | `cargo test -p native-theme widget_metrics` | Wave 0 |
| METRIC-02 | Option<f32> fields, non_exhaustive, serde defaults | unit | `cargo test -p native-theme widget_metrics::tests` | Wave 0 |
| METRIC-03 | widget_metrics field on ThemeVariant | unit | `cargo test -p native-theme model::tests::theme_variant` | Wave 0 |
| METRIC-04 | KDE breezemetrics.h population | unit | `cargo test -p native-theme kde::metrics` | Wave 0 |
| METRIC-05 | Windows GetSystemMetricsForDpi | unit | `cargo test -p native-theme windows::tests::widget_metrics` | Wave 0 |
| METRIC-06 | macOS HIG defaults | unit | `cargo test -p native-theme macos::tests::widget_metrics` | Wave 0 |
| METRIC-07 | GNOME libadwaita defaults | unit | `cargo test -p native-theme gnome::tests::widget_metrics` | Wave 0 |
| METRIC-08 | Preset TOML widget metrics | unit | `cargo test -p native-theme presets::tests` | Existing (updated) |

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme`
- **Per wave merge:** `cargo test`
- **Phase gate:** Full suite green before verification

### Wave 0 Gaps
- [ ] `model/widget_metrics.rs` -- covers METRIC-01, METRIC-02 (12 sub-struct unit tests, serde round-trip, merge, is_empty)
- [ ] `model/mod.rs` tests -- covers METRIC-03 (ThemeVariant widget_metrics field integration)
- [ ] `kde/metrics.rs` tests -- covers METRIC-04 (breeze constants populated correctly)
- [ ] `windows.rs` tests -- covers METRIC-05 (build_theme includes widget_metrics)
- [ ] `macos.rs` tests -- covers METRIC-06 (build_theme includes widget_metrics)
- [ ] `gnome/mod.rs` tests -- covers METRIC-07 (build_theme includes widget_metrics)
- [ ] Existing `presets::tests` updated -- covers METRIC-08 (presets still load, widget_metrics accessible)

## Sources

### Primary (HIGH confidence)
- KDE breezemetrics.h: https://invent.kde.org/plasma/breeze (fetched raw content, all constants extracted)
- Microsoft GetSystemMetrics docs: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics (full SM_ constant list)
- Microsoft GetSystemMetricsForDpi docs: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetricsfordpi
- windows-rs crate SM_ constants: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/constant.SM_CXMENUCHECK.html
- Existing codebase model patterns: verified from `model/geometry.rs`, `model/spacing.rs`, `model/fonts.rs`

### Secondary (MEDIUM confidence)
- macOS widget defaults: based on AppKit intrinsicContentSize docs (22pt button height confirmed), NSScroller scrollerWidth API docs
- libadwaita defaults: based on community reports of ~34px button height, style-classes documentation confirming general sizing approach

### Tertiary (LOW confidence)
- Exact macOS tab/toolbar/splitter dimensions: derived from common knowledge of AppKit conventions, not from official Apple documentation specifying exact pixel values
- Exact libadwaita SCSS variable values: could not access the GitLab SCSS source directly; values based on GTK4 defaults and community observations

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - no new dependencies needed, existing patterns fully documented
- Architecture: HIGH - follows established sub-struct convention from 4 existing model types
- KDE metrics: HIGH - breezemetrics.h constants extracted directly from source
- Windows metrics: HIGH - SM_ constants documented in official Microsoft docs
- macOS metrics: MEDIUM - some widget dimensions are LOW confidence (tab, toolbar), core ones (button, scrollbar) are MEDIUM
- GNOME metrics: MEDIUM - could not verify SCSS source directly, but libadwaita values are relatively stable
- Pitfalls: HIGH - all based on direct codebase analysis

**Research date:** 2026-03-08
**Valid until:** 2026-04-08 (30 days - all platforms have stable widget metrics; KDE/GNOME versions change slowly)
