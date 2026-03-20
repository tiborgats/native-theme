# native-theme

Cross-platform native theme detection and loading for Rust GUI applications.

[![Crates.io](https://img.shields.io/crates/v/native-theme.svg)](https://crates.io/crates/native-theme)
[![docs.rs](https://img.shields.io/docsrs/native-theme)](https://docs.rs/native-theme)
[![CI](https://github.com/tiborgats/native-theme/actions/workflows/ci.yml/badge.svg)](https://github.com/tiborgats/native-theme/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0 OR 0BSD](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0%20%7C%200BSD-blue.svg)](#license)
[![MSRV: 1.94.0](https://img.shields.io/badge/MSRV-1.94.0-blue.svg)](https://blog.rust-lang.org/2026/03/05/Rust-1.94.0.html)

A toolkit-agnostic theme data model with 36 semantic color roles, 17 bundled
TOML presets (light + dark), and optional OS theme readers for Linux, macOS,
and Windows.

<table>
<tr><th></th><th>Material Light</th><th>Material Dark</th><th>Lucide Light</th><th>Lucide Dark</th></tr>
<tr>
<td><strong>Linux</strong></td>
<td><img src="docs/assets/iced-linux-material-light.png" width="200" alt="Linux Material Light"></td>
<td><img src="docs/assets/iced-linux-material-dark.png" width="200" alt="Linux Material Dark"></td>
<td><img src="docs/assets/iced-linux-lucide-light.png" width="200" alt="Linux Lucide Light"></td>
<td><img src="docs/assets/iced-linux-lucide-dark.png" width="200" alt="Linux Lucide Dark"></td>
</tr>
<tr>
<td><strong>macOS</strong></td>
<td><img src="docs/assets/iced-macos-material-light.png" width="200" alt="macOS Material Light"></td>
<td><img src="docs/assets/iced-macos-material-dark.png" width="200" alt="macOS Material Dark"></td>
<td><img src="docs/assets/iced-macos-lucide-light.png" width="200" alt="macOS Lucide Light"></td>
<td><img src="docs/assets/iced-macos-lucide-dark.png" width="200" alt="macOS Lucide Dark"></td>
</tr>
<tr>
<td><strong>Windows</strong></td>
<td><img src="docs/assets/iced-windows-material-light.png" width="200" alt="Windows Material Light"></td>
<td><img src="docs/assets/iced-windows-material-dark.png" width="200" alt="Windows Material Dark"></td>
<td><img src="docs/assets/iced-windows-lucide-light.png" width="200" alt="Windows Lucide Light"></td>
<td><img src="docs/assets/iced-windows-lucide-dark.png" width="200" alt="Windows Lucide Dark"></td>
</tr>
</table>
<p align="center"><em>iced showcase — Material and Lucide icon sets across Linux, macOS, and Windows</em></p>

| Crate | Description |
|-------|-------------|
| [`native-theme`](native-theme/) | Core theme model, presets, and platform readers |
| [`native-theme-gpui`](connectors/native-theme-gpui/) | [gpui](https://gpui.rs) + [gpui-component](https://crates.io/crates/gpui-component) connector |
| [`native-theme-iced`](connectors/native-theme-iced/) | [iced](https://iced.rs) connector |
| [`native-theme-build`](native-theme-build/) | Build-time code generation for custom icon roles |

## Quick Start

```toml
[dependencies]
native-theme = "0.4.1"
```

Load a bundled preset:

```rust
use native_theme::NativeTheme;

let theme = NativeTheme::preset("dracula").unwrap();
let dark = theme.dark.as_ref().unwrap();
let accent = dark.colors.accent.unwrap();
let [r, g, b, a] = accent.to_f32_array();
```

Read the OS theme at runtime:

```rust
use native_theme::{from_system, NativeTheme};

let theme = from_system()
    .unwrap_or_else(|_| NativeTheme::preset("default").unwrap());
```

Layer user overrides on top of a preset:

```rust
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

### gpui

```toml
[dependencies]
native-theme = "0.4"
native-theme-gpui = "0.4"
```

```rust
use native_theme::NativeTheme;
use native_theme_gpui::to_theme;

let nt = NativeTheme::preset("dracula").unwrap();
let is_dark = true;
if let Some(variant) = nt.pick_variant(is_dark) {
    let theme = to_theme(variant, "My App", is_dark);
    // Use as your gpui-component theme
}
```

Run the gpui showcase (full widget gallery with color map inspector):

```sh
cargo run -p native-theme-gpui --example showcase
```

### iced

```toml
[dependencies]
native-theme = "0.4"
native-theme-iced = "0.4"
```

```rust
use native_theme::NativeTheme;
use native_theme_iced::to_theme;

let nt = NativeTheme::preset("dracula").unwrap();
if let Some(variant) = nt.pick_variant(true) {
    let theme = to_theme(variant, "My App");
    // Use as your iced application theme
}
```

Run the iced showcase (full widget gallery with live theme switching):

```sh
cargo run -p native-theme-iced --example showcase
```

### Other toolkits

Map `NativeTheme` fields to your toolkit's types directly. All color, font,
geometry, and spacing fields are public `Option<T>` values. See the
[API docs](https://docs.rs/native-theme) for details.

## Animated Icons

<p align="center">
  <img src="docs/assets/spinner-material.gif" alt="Material spinner" height="80">
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="docs/assets/spinner-lucide.gif" alt="Lucide spinner" height="80">
</p>

Platform-native loading spinners with accessibility support:

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

Bundled spinner: Lucide loader (spin transform). On Linux, freedesktop
`process-working` sprite sheets are loaded at runtime from the active
icon theme (Breeze, Adwaita, etc.).

See the [gpui](connectors/native-theme-gpui/) and
[iced](connectors/native-theme-iced/) connector READMEs for
toolkit-specific playback helpers.

## Platform Support

| Platform | Reader | Feature |
|----------|--------|---------|
| Linux (KDE) | `from_kde()` | `kde` |
| Linux (GNOME/GTK) | `from_gnome()` | `portal-tokio` or `portal-async-io` |
| macOS | `from_macos()` | `macos` |
| Windows | `from_windows()` | `windows` |

`from_system()` auto-detects the platform and desktop environment via
`XDG_CURRENT_DESKTOP`, falling back to bundled presets when a reader is
unavailable. GTK-based desktops (GNOME, XFCE, Cinnamon, MATE, Budgie, LXQt)
are all handled by the portal reader.

## Feature Flags

No features are enabled by default. The preset API works without any features.

**Most apps just need one feature:**

```toml
[dependencies]
native-theme = { version = "0.4", features = ["native"] }
```

### Meta-features

| Feature | Enables |
|---------|---------|
| `native` | All platform readers (tokio async runtime) |
| `native-async-io` | All platform readers (async-io runtime) |
| `linux` | KDE + GNOME portal (tokio) |
| `linux-async-io` | KDE + GNOME portal (async-io) |

OS-specific dependencies are target-gated, so `native` on macOS only compiles
macOS deps.

### Individual features

| Feature | Description |
|---------|-------------|
| `kde` | KDE theme reader (`~/.config/kdeglobals`) |
| `portal-tokio` | GNOME portal reader (tokio) |
| `portal-async-io` | GNOME portal reader (async-io) |
| `macos` | macOS reader (NSAppearance) |
| `windows` | Windows reader (UISettings) |
| `system-icons` | Platform icon theme lookup with bundled fallback |
| `material-icons` | Bundle Material Symbols SVGs |
| `lucide-icons` | Bundle Lucide SVGs |
| `svg-rasterize` | SVG-to-RGBA rasterization via resvg |

## Presets

17 bundled presets, each with light and dark variants:

| Category | Presets |
|----------|--------|
| Core | `default`, `adwaita`, `kde-breeze` |
| Platform | `windows-11`, `macos-sonoma`, `material`, `ios` |
| Community | `catppuccin-latte`, `catppuccin-frappe`, `catppuccin-macchiato`, `catppuccin-mocha`, `nord`, `dracula`, `gruvbox`, `solarized`, `tokyo-night`, `one-dark` |

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be triple licensed as above, without any additional terms or conditions.
