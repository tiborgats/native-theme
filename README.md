# native-theme

Cross-platform native theme detection and loading for Rust GUI applications.

[![License: MIT OR Apache-2.0 OR 0BSD](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0%20%7C%200BSD-blue.svg)](#license)

## Overview

**native-theme** provides a toolkit-agnostic theme data model with 36 semantic
color roles, bundled TOML presets, and optional OS theme reading.

| Crate | Description |
|-------|-------------|
| [`native-theme`](native-theme/) | Core theme model, presets, and platform readers |
| [`native-theme-iced`](connectors/native-theme-iced/) | [iced](https://iced.rs) toolkit connector |
| [`native-theme-gpui`](connectors/native-theme-gpui/) | [gpui](https://gpui.rs) + gpui-component toolkit connector |

## Quick Start

```sh
cargo add native-theme
```

Load a bundled preset:

```rust,ignore
use native_theme::NativeTheme;

let theme = NativeTheme::preset("dracula").unwrap();
let dark = theme.dark.as_ref().unwrap();
let accent = dark.colors.accent.unwrap();
let [r, g, b, a] = accent.to_f32_array();
```

Read the current OS theme at runtime:

```rust,ignore
use native_theme::{from_system, NativeTheme};

let theme = from_system().unwrap_or_else(|_| NativeTheme::preset("default").unwrap());
```

Layer user overrides on top of a preset:

```rust,ignore
use native_theme::NativeTheme;

let mut theme = NativeTheme::preset("nord").unwrap();
let overrides = NativeTheme::from_toml(r#"
name = "My Nord"
[light.colors]
accent = "#ff6600"
"#).unwrap();
theme.merge(&overrides);
```

## Toolkit Connectors

### iced

```sh
cargo add native-theme-iced
```

```rust,ignore
use native_theme::NativeTheme;
use native_theme_iced::{pick_variant, to_theme};

let nt = NativeTheme::preset("dracula").unwrap();
if let Some(variant) = pick_variant(&nt, true) {
    let theme = to_theme(variant, "My App");
    // Use as your iced application theme
}
```

### gpui

The `native-theme-gpui` connector maps to gpui-component's `Theme` type.
See [connectors/native-theme-gpui](connectors/native-theme-gpui/) for details.

### Other toolkits

Map `NativeTheme` fields to your toolkit's types directly. All color, font,
geometry, and spacing fields are public `Option<T>` values. See the
[crate documentation](https://docs.rs/native-theme) for the full API.

## Platform Support

| Platform | Reader | Feature |
|----------|--------|---------|
| Linux (KDE) | `from_kde()` | `kde` |
| Linux (GNOME) | `from_gnome()` | `portal-tokio` or `portal-async-io` |
| macOS | `from_macos()` | `macos` |
| Windows | `from_windows()` | `windows` |

`from_system()` auto-detects the platform and returns the appropriate theme,
falling back to bundled presets when a platform reader is unavailable.

## Feature Flags

| Feature | Description | Platform |
|---------|-------------|----------|
| `kde` | Sync KDE theme reader (`~/.config/kdeglobals`) | Linux |
| `portal` | Base for GNOME portal reader | Linux |
| `portal-tokio` | `from_gnome()` with tokio backend | Linux |
| `portal-async-io` | `from_gnome()` with async-io backend | Linux |
| `windows` | Windows theme reader (UISettings + system metrics) | Windows |
| `macos` | macOS theme reader (NSAppearance) | macOS |
| `system-icons` | Platform icon theme lookup with bundled fallback | All |
| `material-icons` | Bundle Material Symbols SVGs | All |
| `lucide-icons` | Bundle Lucide SVGs | All |
| `svg-rasterize` | SVG-to-RGBA rasterization via resvg | All |

No features are enabled by default. The preset API works without any features.

## Available Presets

17 bundled presets, each with light and dark variants:

**Core:** `default`, `adwaita`, `kde-breeze`

**Platform:** `windows-11`, `macos-sonoma`, `material`, `ios`

**Community:** `catppuccin-latte`, `catppuccin-frappe`, `catppuccin-macchiato`,
`catppuccin-mocha`, `nord`, `dracula`, `gruvbox`, `solarized`, `tokyo-night`,
`one-dark`

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
