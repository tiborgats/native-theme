# native-theme

Cross-platform native theme detection and loading for Rust GUI applications.

[![License: MIT OR Apache-2.0 OR 0BSD](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0%20%7C%200BSD-blue.svg)](#license)

## Overview

**native-theme** provides a toolkit-agnostic theme data model with 24 semantic
color roles, 25 per-widget resolved themes, bundled TOML presets, and optional
OS theme reading. It gives your Rust GUI application access to consistent,
structured theme data regardless of which toolkit you use.

## Quick Start

Add the dependency:

```sh
cargo add native-theme
```

Load a preset and access theme fields:

```rust
let theme = native_theme::theme::Theme::preset("dracula").unwrap();
let dark = theme.dark.as_ref().unwrap();
let accent = dark.defaults.accent_color.unwrap();   // Option<Rgba>
let bg = dark.defaults.background_color.unwrap();   // Option<Rgba>
// Convert to f32 for toolkits that use normalized colors
let [r, g, b, a] = accent.to_f32_array();
```

For fully resolved themes (all fields guaranteed populated), use the
resolve + validate pipeline:

```rust,ignore
use native_theme::theme::ColorMode;
let variant = theme.into_variant(ColorMode::Dark)?;
let resolved = variant.into_resolved(None)?; // -> ResolvedTheme
let accent = resolved.defaults.accent_color; // Rgba (not Option)
```

## Preset Workflow

Start with a bundled preset, then layer sparse user overrides on top.
The `merge()` method fills in only the fields present in the overlay,
leaving everything else from the base preset intact.

```rust
use native_theme::theme::Theme;
let mut theme = Theme::preset("nord").unwrap();
let user_overrides = Theme::from_toml(r##"
name = "My Custom Nord"
[light.defaults]
accent_color = "#ff6600"
"##).unwrap();
theme.merge(&user_overrides);
```

## Runtime Workflow

Use `from_system()` to read the current OS theme at runtime. It returns a
`SystemTheme` with both light and dark `ResolvedTheme` variants:

```rust,ignore
use native_theme::SystemTheme;
use native_theme::theme::ColorMode;
let system = SystemTheme::from_system()?;
let active = system.pick(system.mode);   // &ResolvedTheme for current OS mode
let light = system.pick(ColorMode::Light); // Explicit variant selection
let is_dark = system.mode.is_dark();     // OS dark mode state
```

Apply user overrides on top of the OS theme:

```ignore
let overlay = Theme::from_toml(r#"
    [light.defaults]
    accent_color = "#ff6600"
"#)?;
let customized = system.with_overlay(&overlay)?;
```

**Platform behavior:**

- **Linux (KDE):** Reads live theme from `~/.config/kdeglobals` (requires `kde` feature).
- **Linux (GNOME/other):** Returns the bundled Adwaita preset. For live
  portal data (accent color, dark mode preference, contrast setting), enable
  the `portal` feature.
- **macOS:** Reads system appearance via NSAppearance (requires `macos` feature).
- **Windows:** Reads accent colors and system metrics (requires `windows` feature).
- **Other platforms:** Returns `Error::PlatformUnsupported`.

`detect::system_is_dark()` provides a lightweight cached check for the OS dark mode
preference on all platforms (Linux, macOS, and Windows), without running the
full theme reader pipeline.

## Toolkit Connectors

Official connector crates handle the full mapping from `Theme` to
toolkit-specific theme types:

- [`native-theme-iced`](https://crates.io/crates/native-theme-iced) -- iced
  toolkit connector with Catalog support for all built-in widget styles
- `native-theme-gpui` -- gpui + gpui-component toolkit connector

For other toolkits, map `ResolvedTheme` fields directly. After
resolution, all fields are guaranteed populated:

```rust,ignore
let resolved = variant.into_resolved(None)?; // ResolvedTheme
let bg = resolved.defaults.background_color;     // Rgba
let accent = resolved.defaults.accent_color;     // Rgba
let font = &resolved.defaults.font.family;       // &Arc<str>
let radius = resolved.defaults.border.corner_radius; // f32
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
4. Include the generated code and use `IconLoader` to load icons at runtime

```rust,ignore
// build.rs
use native_theme_build::UnwrapOrExit;
native_theme_build::generate_icons("icons/icons.toml")
    .unwrap_or_exit()
    .emit_cargo_directives()
    .expect("failed to write generated code");

// src/lib.rs
include!(concat!(env!("OUT_DIR"), "/app_icon.rs"));

use native_theme::icons::IconLoader;
use native_theme::theme::IconSet;
let icon = IconLoader::new(&AppIcon::PlayPause).set(IconSet::Material).load();
```

See the [`native-theme-build` docs](https://docs.rs/native-theme-build) for the
full TOML schema, builder API, and DE-aware mapping support.

## Animated Icons

<p align="center">
  <img src="https://raw.githubusercontent.com/tiborgats/native-theme/main/docs/assets/spinner-material.gif" alt="Material spinner" height="80">
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="https://raw.githubusercontent.com/tiborgats/native-theme/main/docs/assets/spinner-lucide.gif" alt="Lucide spinner" height="80">
</p>

`IconLoader::load_indicator()` returns a platform-native loading spinner
animation matching the requested icon set (Material, Lucide, or a freedesktop
theme's `process-working` animation):

```rust,ignore
use native_theme::icons::IconLoader;
use native_theme::detect::prefers_reduced_motion;
use native_theme::theme::{AnimatedIcon, IconRole, IconSet};

if let Some(anim) = IconLoader::new(IconRole::StatusBusy).set(IconSet::Material).load_indicator() {
    if prefers_reduced_motion() {
        // Respect OS accessibility settings with a static fallback
        let static_icon = anim.first_frame();
    } else {
        match &anim {
            AnimatedIcon::Frames(data) => {
                // Cycle through data.frames() on a timer (data.frame_duration_ms().get() ms)
            }
            AnimatedIcon::Transform(data) => {
                // Apply continuous rotation to data.icon() via data.animation()
            }
            _ => {}
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
| `kde` | KDE theme reader (`~/.config/kdeglobals`) | Linux |
| `portal` | GNOME portal reader (async-io via ashpd) | Linux |
| `windows` | Windows reader (UISettings) | Windows |
| `macos` | macOS reader (NSAppearance) | macOS |
| `system-icons` | Platform icon theme lookup with bundled fallback | All |
| `material-icons` | Bundle Material Symbols SVGs | All |
| `lucide-icons` | Bundle Lucide SVGs | All |
| `watch` | Runtime theme change watching via filesystem/D-Bus | Linux |
| `svg-rasterize` | SVG-to-RGBA rasterization via resvg | All |

By default, no features are enabled. The preset API (`Theme::preset()`,
`Theme::from_toml()`, `Theme::from_file()`, `Theme::list_presets()`,
`.to_toml()`) works without any features.

## Available Presets

All presets are embedded at compile time via `include_str!()` and available
through `Theme::preset("name")`. Each provides both light and dark variants.

**Platform:** `kde-breeze`, `adwaita`, `windows-11`, `macos-sonoma`, `material`, `ios`

**Community:** `catppuccin-latte`, `catppuccin-frappe`, `catppuccin-macchiato`,
`catppuccin-mocha`, `nord`, `dracula`, `gruvbox`, `solarized`, `tokyo-night`,
`one-dark`

Use `Theme::list_presets()` to get all 16 names programmatically, or
`Theme::list_presets_for_platform()` for only those appropriate on the current OS.

## TOML Format

Theme files use TOML. All fields are optional -- omit any you don't need.
See the [`Theme::from_toml()`](https://docs.rs/native-theme/latest/native_theme/struct.Theme.html#method.from_toml)
documentation for the full format reference.

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be triple licensed as above, without any additional terms or conditions.
