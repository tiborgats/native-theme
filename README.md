# native-theme

Cross-platform native theme detection and loading for Rust GUI applications.

[![Crates.io](https://img.shields.io/crates/v/native-theme.svg)](https://crates.io/crates/native-theme)
[![License: MIT OR Apache-2.0 OR 0BSD](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0%20%7C%200BSD-blue.svg)](#license)
[![MSRV: 1.85.0](https://img.shields.io/badge/MSRV-1.85.0-blue.svg)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html)

A toolkit-agnostic theme data model with 36 semantic color roles, 17 bundled
TOML presets (light + dark), and optional OS theme readers for Linux, macOS,
and Windows.

| Crate | Description |
|-------|-------------|
| [`native-theme`](native-theme/) | Core theme model, presets, and platform readers |
| [`native-theme-gpui`](connectors/native-theme-gpui/) | [gpui](https://gpui.rs) + [gpui-component](https://crates.io/crates/gpui-component) connector |
| [`native-theme-iced`](connectors/native-theme-iced/) | [iced](https://iced.rs) connector |

## Quick Start

```toml
[dependencies]
native-theme = "0.3.1"
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
native-theme = "0.3"
native-theme-gpui = "0.3"
```

```rust
use native_theme::NativeTheme;
use native_theme_gpui::{pick_variant, to_theme};

let nt = NativeTheme::preset("dracula").unwrap();
let is_dark = true;
if let Some(variant) = pick_variant(&nt, is_dark) {
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
native-theme = "0.3"
native-theme-iced = "0.3"
```

```rust
use native_theme::NativeTheme;
use native_theme_iced::{pick_variant, to_theme};

let nt = NativeTheme::preset("dracula").unwrap();
if let Some(variant) = pick_variant(&nt, true) {
    let theme = to_theme(variant, "My App");
    // Use as your iced application theme
}
```

Run the iced demo (sidebar widget gallery with live theme switching):

```sh
cargo run -p native-theme-iced --example demo
```

### Other toolkits

Map `NativeTheme` fields to your toolkit's types directly. All color, font,
geometry, and spacing fields are public `Option<T>` values. See the
[API docs](https://docs.rs/native-theme) for details.

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
native-theme = { version = "0.3", features = ["native"] }
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
