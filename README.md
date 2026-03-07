# native-theme

Cross-platform native theme detection and loading for Rust GUI applications.

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

## Overview

**native-theme** provides a toolkit-agnostic theme data model with 36 semantic
color roles, bundled TOML presets, and optional OS theme reading. It gives your
Rust GUI application access to consistent, structured theme data regardless of
which toolkit you use.

What native-theme **is:**

- A data model for theme colors, fonts, geometry, and spacing
- A library of 17 bundled presets (platform and community themes)
- Optional readers for live OS theme data (KDE, GNOME, Windows)
- A TOML-based format for user-customizable themes with deep merge support

What native-theme **is not:**

- Not a GUI toolkit -- it produces data, not widgets
- Not a widget styling library -- it does not render anything
- Toolkit adapters live in **your** application code, not in this crate

Works with **egui**, **iced**, **slint**, and any Rust GUI toolkit that accepts
color, font, or spacing values.

## Quick Start

Add the dependency:

```sh
cargo add native-theme
```

Load a preset and access theme fields:

```rust
let theme = native_theme::preset("dracula").unwrap();
let dark = theme.dark.as_ref().unwrap();
let accent = dark.colors.core.accent.unwrap();
let bg = dark.colors.core.background.unwrap();
// Convert to f32 for toolkits that use normalized colors
let [r, g, b, a] = accent.to_f32_array();
```

## Preset Workflow

Start with a bundled preset, then layer sparse user overrides on top.
The `merge()` method fills in only the fields present in the overlay,
leaving everything else from the base preset intact.

```rust
use native_theme::{NativeTheme, Rgba, preset, from_toml};
let mut theme = preset("nord").unwrap();
let user_overrides = from_toml(r#"
name = "My Custom Nord"
[light.colors.core]
accent = "#ff6600"
"#).unwrap();
theme.merge(&user_overrides);
```

## Runtime Workflow

Use `from_system()` to read the current OS theme at runtime, with a preset
fallback for unsupported platforms:

```rust
use native_theme::{from_system, preset};
let theme = from_system().unwrap_or_else(|_| preset("default").unwrap());
```

**Platform behavior:**

- **Linux (KDE):** Reads live theme from `~/.config/kdeglobals` (requires `kde` feature).
- **Linux (GNOME/other):** Returns the bundled Adwaita preset. For live
  portal data (accent color, dark mode preference, contrast setting), call
  `from_gnome().await` directly -- this requires the `portal-tokio` or
  `portal-async-io` feature.
- **Windows:** Reads accent colors and system metrics (requires `windows` feature).
- **Other platforms:** Returns `Error::Unsupported`; use the preset fallback.

## Toolkit Adapter Examples

Each example below shows a standalone mapping function that converts
`NativeTheme` fields into the toolkit's styling types. This code lives in
**your** application, not in native-theme.

### egui

```rust,ignore
use egui::{Color32, style::Visuals};
use native_theme::Rgba;

fn rgba_to_color32(c: &Rgba) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
}

fn apply_theme(ctx: &egui::Context, theme: &native_theme::NativeTheme) {
    let variant = theme.dark.as_ref().unwrap();
    let c = &variant.colors;

    let mut visuals = Visuals::dark();
    visuals.window_fill = rgba_to_color32(&c.core.background.unwrap());
    visuals.panel_fill = rgba_to_color32(&c.panel.sidebar.unwrap());
    visuals.hyperlink_color = rgba_to_color32(&c.interactive.link.unwrap());
    visuals.error_fg_color = rgba_to_color32(&c.status.danger.unwrap());
    visuals.warn_fg_color = rgba_to_color32(&c.status.warning.unwrap());
    visuals.selection.bg_fill = rgba_to_color32(&c.interactive.selection.unwrap());
    visuals.extreme_bg_color = rgba_to_color32(&c.core.surface.unwrap());
    visuals.faint_bg_color = rgba_to_color32(&c.component.alternate_row.unwrap());

    ctx.set_visuals(visuals);
}
```

### iced

```rust,ignore
use iced::{Color, Theme};
use iced::theme::Palette;

fn rgba_to_iced(c: &native_theme::Rgba) -> Color {
    Color::from_rgb8(c.r, c.g, c.b)
}

fn to_iced_theme(theme: &native_theme::NativeTheme) -> Theme {
    let v = theme.dark.as_ref().unwrap();
    let c = &v.colors;

    let palette = Palette {
        background: rgba_to_iced(&c.core.background.unwrap()),
        text: rgba_to_iced(&c.core.foreground.unwrap()),
        primary: rgba_to_iced(&c.core.accent.unwrap()),
        success: rgba_to_iced(&c.status.success.unwrap()),
        warning: rgba_to_iced(&c.status.warning.unwrap()),
        danger: rgba_to_iced(&c.status.danger.unwrap()),
    };
    Theme::custom("Native".into(), palette)
}
```

### slint

Define a global singleton in your `.slint` file:

```text
export global ThemeBridge {
    in-out property <color> background;
    in-out property <color> foreground;
    in-out property <color> accent;
    in-out property <color> surface;
    in-out property <color> danger;
    in-out property <color> success;
}
```

Then set it from Rust:

```rust,ignore
fn apply_theme(app: &App, theme: &native_theme::NativeTheme) {
    let v = theme.light.as_ref().unwrap();
    let c = &v.colors;

    let bridge = app.global::<ThemeBridge>();
    bridge.set_background(to_slint(&c.core.background.unwrap()));
    bridge.set_foreground(to_slint(&c.core.foreground.unwrap()));
    bridge.set_accent(to_slint(&c.core.accent.unwrap()));
    bridge.set_surface(to_slint(&c.core.surface.unwrap()));
    bridge.set_danger(to_slint(&c.status.danger.unwrap()));
    bridge.set_success(to_slint(&c.status.success.unwrap()));
}

fn to_slint(c: &native_theme::Rgba) -> slint::Color {
    slint::Color::from_argb_u8(c.a, c.r, c.g, c.b)
}
```

> **Production note:** The adapter examples above use `unwrap()` for brevity.
> Production code should handle `None` fields with fallback defaults rather
> than `unwrap()`.

## Feature Flags

| Feature | Enables | Platform | Notes |
|---------|---------|----------|-------|
| `kde` | `from_kde()` sync KDE reader | Linux | Parses `~/.config/kdeglobals` via configparser |
| `portal` | Base for GNOME portal reader | Linux | Not useful alone -- must also enable `portal-tokio` or `portal-async-io` |
| `portal-tokio` | `from_gnome()` with tokio runtime | Linux | Implies `portal`; uses ashpd with tokio backend |
| `portal-async-io` | `from_gnome()` with async-io runtime | Linux | Implies `portal`; uses ashpd with async-io backend |
| `windows` | `from_windows()` Windows reader | Windows | Reads UISettings accent + GetSystemMetrics geometry |

By default, no features are enabled. The preset API (`preset()`, `from_toml()`,
`from_file()`, `list_presets()`, `to_toml()`) works without any features.

## Available Presets

All presets are embedded at compile time via `include_str!()` and available
through `preset("name")`. Each provides both light and dark variants.

### Core

| Name | Description |
|------|-------------|
| `default` | Neutral toolkit-agnostic theme with balanced colors |
| `kde-breeze` | KDE Breeze theme colors and spacing |
| `adwaita` | GNOME Adwaita theme colors |

### Platform

| Name | Description |
|------|-------------|
| `windows-11` | Windows 11 design language with Segoe UI |
| `macos-sonoma` | macOS Sonoma system appearance |
| `material` | Material Design 3 baseline colors |
| `ios` | iOS system appearance with SF Pro |

### Community

| Name | Description |
|------|-------------|
| `catppuccin-latte` | Catppuccin Latte (light pastel) |
| `catppuccin-frappe` | Catppuccin Frappe (mid-tone pastel) |
| `catppuccin-macchiato` | Catppuccin Macchiato (dark pastel) |
| `catppuccin-mocha` | Catppuccin Mocha (deep pastel) |
| `nord` | Nord arctic color palette |
| `dracula` | Dracula dark theme |
| `gruvbox` | Gruvbox retro groove colors |
| `solarized` | Solarized precision colors |
| `tokyo-night` | Tokyo Night editor theme |
| `one-dark` | Atom One Dark colors |

Use `list_presets()` to get all 17 names programmatically.

## TOML Format Reference

Theme files use TOML with the following structure. All fields are
`Option<T>` -- omit any field you do not need. Unknown fields are ignored.
Hex colors accept `#RRGGBB` or `#RRGGBBAA` format.

```toml
name = "My Theme"

[light.colors.core]
accent = "#4a90d9"
background = "#fafafa"
foreground = "#2e3436"
surface = "#ffffff"
border = "#c0c0c0"
muted = "#929292"
shadow = "#00000018"

[light.colors.primary]
background = "#4a90d9"
foreground = "#ffffff"

[light.colors.secondary]
background = "#6c757d"
foreground = "#ffffff"

[light.colors.status]
danger = "#dc3545"
warning = "#f0ad4e"
success = "#28a745"
info = "#4a90d9"

[light.colors.interactive]
selection = "#4a90d9"
link = "#2a6cb6"
focus_ring = "#4a90d9"

[light.colors.panel]
sidebar = "#f0f0f0"
tooltip = "#2e3436"
popover = "#ffffff"

[light.colors.component]
button = "#e8e8e8"
input = "#ffffff"
disabled = "#c0c0c0"
separator = "#d0d0d0"
alternate_row = "#f5f5f5"

[light.fonts]
family = "sans-serif"
size = 10.0
mono_family = "monospace"
mono_size = 10.0

[light.geometry]
radius = 6.0
frame_width = 1.0
disabled_opacity = 0.5
border_opacity = 0.15
scroll_width = 8.0

[light.spacing]
xxs = 2.0
xs = 4.0
s = 6.0
m = 12.0
l = 18.0
xl = 24.0
xxl = 36.0

# [dark.*] mirrors the same structure as [light.*]
```

Each `[light.*]` section has a corresponding `[dark.*]` section with the
same field names. Status, panel, and component color groups also support
`_foreground` suffixed fields (e.g., `danger_foreground`, `sidebar_foreground`).

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
