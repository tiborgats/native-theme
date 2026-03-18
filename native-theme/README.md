# native-theme

Cross-platform native theme detection and loading for Rust GUI applications.

[![License: MIT OR Apache-2.0 OR 0BSD](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0%20%7C%200BSD-blue.svg)](#license)

## Overview

**native-theme** provides a toolkit-agnostic theme data model with 36 semantic
color roles, bundled TOML presets, and optional OS theme reading. It gives your
Rust GUI application access to consistent, structured theme data regardless of
which toolkit you use.

## Quick Start

Add the dependency:

```sh
cargo add native-theme
```

Load a preset and access theme fields:

```rust
let theme = native_theme::NativeTheme::preset("dracula").unwrap();
let dark = theme.dark.as_ref().unwrap();
let accent = dark.colors.accent.unwrap();
let bg = dark.colors.background.unwrap();
// Convert to f32 for toolkits that use normalized colors
let [r, g, b, a] = accent.to_f32_array();
```

## Preset Workflow

Start with a bundled preset, then layer sparse user overrides on top.
The `merge()` method fills in only the fields present in the overlay,
leaving everything else from the base preset intact.

```rust
use native_theme::NativeTheme;
let mut theme = NativeTheme::preset("nord").unwrap();
let user_overrides = NativeTheme::from_toml(r##"
name = "My Custom Nord"
[light.colors]
accent = "#ff6600"
"##).unwrap();
theme.merge(&user_overrides);
```

## Runtime Workflow

Use `from_system()` to read the current OS theme at runtime, with a preset
fallback for unsupported platforms:

```rust
use native_theme::{from_system, NativeTheme};
let theme = from_system().unwrap_or_else(|_| NativeTheme::preset("default").unwrap());
```

**Platform behavior:**

- **Linux (KDE):** Reads live theme from `~/.config/kdeglobals` (requires `kde` feature).
- **Linux (GNOME/other):** Returns the bundled Adwaita preset. For live
  portal data (accent color, dark mode preference, contrast setting), call
  `from_gnome().await` directly -- this requires the `portal-tokio` or
  `portal-async-io` feature.
- **macOS:** Reads system appearance via NSAppearance (requires `macos` feature).
- **Windows:** Reads accent colors and system metrics (requires `windows` feature).
- **Other platforms:** Returns `Error::Unsupported`; use the preset fallback.

## Toolkit Connectors

Official connector crates handle the full mapping from `NativeTheme` to
toolkit-specific theme types:

- [`native-theme-iced`](https://crates.io/crates/native-theme-iced) -- iced
  toolkit connector with Catalog support for all built-in widget styles
- `native-theme-gpui` -- gpui + gpui-component toolkit connector

For other toolkits, map `NativeTheme` fields directly. All color, font,
geometry, and spacing fields are public `Option<T>` values:

```rust,ignore
let variant = theme.dark.as_ref().unwrap();
let bg = variant.colors.background.unwrap();
let accent = variant.colors.accent.unwrap();
let font = variant.fonts.family.as_deref().unwrap_or("sans-serif");
let radius = variant.geometry.radius.unwrap_or(4.0);
```

## Custom Icon Roles

Apps can define domain-specific icons (e.g., play/pause, git-branch, thermometer)
via TOML definitions and the [`native-theme-build`](https://docs.rs/native-theme-build)
crate. Generated icons integrate with the same loading pipeline as built-in
`IconRole` variants, so they work across all icon sets (Material, Lucide,
freedesktop, SF Symbols, Segoe Fluent).

**Workflow:**

1. Define icons in a TOML file with per-set name mappings and bundled SVGs
2. Add `native-theme-build` as a build dependency
3. Call `generate_icons()` in `build.rs` to generate a Rust enum implementing `IconProvider`
4. Include the generated code and use `load_custom_icon()` to load icons at runtime

```rust,ignore
// build.rs
native_theme_build::generate_icons("icons/icons.toml");

// src/lib.rs
include!(concat!(env!("OUT_DIR"), "/app_icon.rs"));

use native_theme::load_custom_icon;
let icon = load_custom_icon(&AppIcon::PlayPause, "material");
```

See the [`native-theme-build` docs](https://docs.rs/native-theme-build) for the
full TOML schema, builder API, and DE-aware mapping support.

## Animated Icons

`loading_indicator()` returns a platform-native loading spinner animation
matching the requested icon set (Material, Lucide, macOS, Windows, Adwaita,
or a freedesktop theme's `process-working` animation):

```rust,ignore
use native_theme::{loading_indicator, prefers_reduced_motion, AnimatedIcon};

if let Some(anim) = loading_indicator("material") {
    if prefers_reduced_motion() {
        // Respect OS accessibility settings with a static fallback
        let static_icon = anim.first_frame();
    } else {
        match &anim {
            AnimatedIcon::Frames { frames, frame_duration_ms, .. } => {
                // Cycle through pre-rendered frames on a timer
            }
            AnimatedIcon::Transform { icon, animation } => {
                // Apply continuous rotation to the icon
            }
        }
    }
}
```

Toolkit connectors provide playback helpers:
[`animated_frames_to_image_sources()`](https://docs.rs/native-theme-gpui) and
[`with_spin_animation()`](https://docs.rs/native-theme-gpui) for gpui,
[`animated_frames_to_svg_handles()`](https://docs.rs/native-theme-iced) and
[`spin_rotation_radians()`](https://docs.rs/native-theme-iced) for iced.

## Feature Flags

| Feature | Enables | Platform |
|---------|---------|----------|
| `kde` | `from_kde()` sync KDE reader | Linux |
| `portal` | Base for GNOME portal reader | Linux |
| `portal-tokio` | `from_gnome()` with tokio runtime | Linux |
| `portal-async-io` | `from_gnome()` with async-io runtime | Linux |
| `windows` | `from_windows()` Windows reader | Windows |
| `macos` | `from_macos()` macOS reader | macOS |
| `system-icons` | Platform icon theme lookup with bundled fallback | All |
| `material-icons` | Bundle Material Symbols SVGs | All |
| `lucide-icons` | Bundle Lucide SVGs | All |
| `svg-rasterize` | SVG-to-RGBA rasterization via resvg | All |

By default, no features are enabled. The preset API (`NativeTheme::preset()`,
`NativeTheme::from_toml()`, `NativeTheme::from_file()`, `NativeTheme::list_presets()`,
`.to_toml()`) works without any features.

## Available Presets

All presets are embedded at compile time via `include_str!()` and available
through `NativeTheme::preset("name")`. Each provides both light and dark variants.

**Core:** `default`, `adwaita`, `kde-breeze`

**Platform:** `windows-11`, `macos-sonoma`, `material`, `ios`

**Community:** `catppuccin-latte`, `catppuccin-frappe`, `catppuccin-macchiato`,
`catppuccin-mocha`, `nord`, `dracula`, `gruvbox`, `solarized`, `tokyo-night`,
`one-dark`

Use `NativeTheme::list_presets()` to get all 17 names programmatically.

## TOML Format

Theme files use TOML. All fields are optional -- omit any you don't need.
See the [`NativeTheme::from_toml()`](https://docs.rs/native-theme/latest/native_theme/struct.NativeTheme.html#method.from_toml)
documentation for the full format reference.

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
