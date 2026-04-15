# native-theme

Cross-platform native theme detection and loading for Rust GUI applications.

[![Crates.io](https://img.shields.io/crates/v/native-theme.svg)](https://crates.io/crates/native-theme)
[![docs.rs](https://img.shields.io/docsrs/native-theme)](https://docs.rs/native-theme)
[![CI](https://github.com/tiborgats/native-theme/actions/workflows/ci.yml/badge.svg)](https://github.com/tiborgats/native-theme/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0 OR 0BSD](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0%20%7C%200BSD-blue.svg)](#license)
[![MSRV: 1.94.0](https://img.shields.io/badge/MSRV-1.94.0-blue.svg)](https://blog.rust-lang.org/2026/03/05/Rust-1.94.0.html)

A toolkit-agnostic theme data model with 24 semantic color roles, 25 per-widget
themes, 16 bundled TOML presets (light + dark), and optional OS theme readers
for Linux, macOS, and Windows.

![gpui theme switching](docs/assets/gpui-theme-switching.gif)

![Iced theme switching](docs/assets/iced-theme-switching.gif)

| Crate | Description |
|-------|-------------|
| [`native-theme`](native-theme/) | Core theme model, presets, and platform readers |
| [`native-theme-gpui`](connectors/native-theme-gpui/) | [gpui](https://gpui.rs) + [gpui-component](https://crates.io/crates/gpui-component) connector |
| [`native-theme-iced`](connectors/native-theme-iced/) | [iced](https://iced.rs) connector |
| [`native-theme-build`](native-theme-build/) | Build-time code generation for custom icon roles |

## Quick Start

```toml
[dependencies]
native-theme = "0.5.7"
```

Load a bundled preset:

```rust
use native_theme::theme::Theme;

let theme = Theme::preset("dracula").unwrap();
let dark = theme.dark.as_ref().unwrap();
let accent = dark.defaults.accent_color.unwrap();
let [r, g, b, a] = accent.to_f32_array();
```

Read the OS theme at runtime (returns a fully resolved `SystemTheme`):

```rust,ignore
use native_theme::SystemTheme;

let system = SystemTheme::from_system()?;
let active = system.pick(system.mode); // &ResolvedTheme for current OS mode
let accent = active.defaults.accent_color;  // Rgba (not Option)
```

Layer user overrides on top of a preset:

```rust
use native_theme::theme::Theme;

let mut theme = Theme::preset("nord").unwrap();
let overrides = Theme::from_toml(r#"
name = "My Nord"
[light.defaults]
accent_color = "#ff6600"
"#).unwrap();
theme.merge(&overrides);
```

## Toolkit Connectors

### gpui

```toml
[dependencies]
native-theme = "0.5.7"
native-theme-gpui = "0.5.7"
```

```rust,ignore
use native_theme::theme::{ColorMode, Theme};
use native_theme_gpui::to_theme;

let nt = Theme::preset("dracula").unwrap();
let variant = nt.into_variant(ColorMode::Dark).unwrap();
let resolved = variant.into_resolved(None).unwrap();
let theme = to_theme(&resolved, "My App", true, false);
// Use as your gpui-component theme
```

Run the gpui showcase (full widget gallery with color map inspector):

```sh
cargo run -p native-theme-gpui --example showcase-gpui
```

### iced

```toml
[dependencies]
native-theme = "0.5.7"
native-theme-iced = "0.5.7"
```

```rust,ignore
use native_theme::theme::{ColorMode, Theme};
use native_theme_iced::to_theme;

let nt = Theme::preset("dracula").unwrap();
let variant = nt.into_variant(ColorMode::Dark).unwrap();
let resolved = variant.into_resolved(None).unwrap();
let theme = to_theme(&resolved, "My App");
// Use as your iced application theme
```

Run the iced showcase (full widget gallery with live theme switching):

```sh
cargo run -p native-theme-iced --example showcase-iced
```

### Other toolkits

Map `ResolvedTheme` fields to your toolkit's types directly. After resolving,
all color, font, geometry, and spacing fields are guaranteed populated. See the
[API docs](https://docs.rs/native-theme) for details.

## Animated Icons

<p align="center">
  <img src="docs/assets/spinner-material.gif" alt="Material spinner" height="80">
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="docs/assets/spinner-lucide.gif" alt="Lucide spinner" height="80">
</p>

Platform-native loading spinners with accessibility support:

```rust,ignore
use native_theme::theme::{AnimatedIcon, IconRole, IconSet};
use native_theme::icons::IconLoader;
use native_theme::detect::prefers_reduced_motion;

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

Bundled spinner: Lucide loader (spin transform). On Linux, freedesktop
`process-working` sprite sheets are loaded at runtime from the active
icon theme (Breeze, Adwaita, etc.).

See the [gpui](connectors/native-theme-gpui/) and
[iced](connectors/native-theme-iced/) connector READMEs for
toolkit-specific playback helpers.

## Platform Support

| Platform | Feature |
|----------|---------|
| Linux (KDE) | `kde` |
| Linux (GNOME/GTK) | `portal` |
| macOS | `macos` |
| Windows | `windows` |

`SystemTheme::from_system()` auto-detects the platform and desktop environment
via `XDG_CURRENT_DESKTOP`, returning a `SystemTheme` with both light and dark
`ResolvedTheme` variants. Falls back to bundled presets when a reader is
unavailable. GTK-based desktops (GNOME, XFCE, Cinnamon, MATE, Budgie, LXQt)
are all handled by the portal reader.

`detect::system_is_dark()` provides a lightweight cached check for the OS dark
mode preference on all platforms (Linux, macOS, and Windows).

## Feature Flags

No features are enabled by default. The preset API works without any features.

**Most apps just need one feature:**

```toml
[dependencies]
native-theme = { version = "0.5.7", features = ["native"] }
```

### Meta-features

| Feature | Enables |
|---------|---------|
| `native` | All platform readers (`kde` + `portal` + `macos` + `windows`) |
| `linux` | `kde` + `portal` |

OS-specific dependencies are target-gated, so `native` on macOS only compiles
macOS deps.

### Individual features

| Feature | Description |
|---------|-------------|
| `kde` | KDE theme reader (`~/.config/kdeglobals`) |
| `portal` | GNOME portal reader (async-io via ashpd) |
| `macos` | macOS reader (NSAppearance) |
| `windows` | Windows reader (UISettings) |
| `watch` | Runtime theme change watching via filesystem/D-Bus |
| `system-icons` | Platform icon theme lookup with bundled fallback |
| `material-icons` | Bundle Material Symbols SVGs |
| `lucide-icons` | Bundle Lucide SVGs |
| `svg-rasterize` | SVG-to-RGBA rasterization via resvg |

## Presets

16 bundled presets, each with light and dark variants:

| Category | Presets |
|----------|--------|
| Platform | `kde-breeze`, `adwaita`, `windows-11`, `macos-sonoma`, `material`, `ios` |
| Community | `catppuccin-latte`, `catppuccin-frappe`, `catppuccin-macchiato`, `catppuccin-mocha`, `nord`, `dracula`, `gruvbox`, `solarized`, `tokyo-night`, `one-dark` |

Use `theme::Theme::list_presets_for_platform()` to get only the presets
appropriate for the current OS.

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be triple licensed as above, without any additional terms or conditions.
