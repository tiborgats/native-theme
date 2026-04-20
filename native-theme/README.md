# native-theme

[![License: MIT OR Apache-2.0 OR 0BSD](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0%20%7C%200BSD-blue.svg)](#license)

## What it does

Cross-platform theme data model with 24 semantic color roles, 25 per-widget
themes, and 16 bundled TOML presets. Reads OS themes from KDE Plasma, GNOME
(via `xdg-desktop-portal`), macOS, and Windows, and produces a fully populated
`ResolvedTheme` any GUI toolkit can consume.

## How it fits

Most apps don't depend on this crate directly — they use a framework connector like
[`native-theme-gpui`](../connectors/native-theme-gpui/) or
[`native-theme-iced`](../connectors/native-theme-iced/), which pull `native-theme` in
transitively. Depend on `native-theme` directly only if you are writing a new connector.

## Quick start

Add the dependency:

```sh
cargo add native-theme
```

Read the live OS theme:

```rust,no_run
use native_theme::SystemTheme;

let sys = SystemTheme::from_system()?;
let active = sys.pick(sys.mode);                 // &ResolvedTheme
let accent = active.defaults.accent_color;       // Rgba (fully populated)
let bg     = active.defaults.background_color;   // Rgba
# Ok::<(), native_theme::error::Error>(())
```

For OS readers, enable the `native` feature (or an individual `kde` / `portal`
/ `macos` / `windows`) — see [Feature flags](#feature-flags) below. No features
are needed to use bundled presets.

## Core concepts

- **`Theme`** — sparse, TOML-shaped definition (fields are `Option<T>`). Load via `Theme::preset(…)`, `Theme::from_toml(…)`, or `Theme::from_file(…)`.
- **`ResolvedTheme`** — fully-populated variant where every field has a value. Safe to hand to UI code.
- **`ColorMode`** — the `Light` / `Dark` choice passed when resolving.
- **Preset** — a named bundled theme. 16 ship today:
  - *Platform:* `kde-breeze`, `adwaita`, `windows-11`, `macos-sonoma`, `material`, `ios`
  - *Community:* `catppuccin-latte`, `catppuccin-frappe`, `catppuccin-macchiato`, `catppuccin-mocha`, `nord`, `dracula`, `gruvbox`, `solarized`, `tokyo-night`, `one-dark`

  Enumerate with `Theme::list_presets()` or `Theme::list_presets_for_platform()`.

## Common recipes

### Load a bundled preset

```rust
use native_theme::theme::{ColorMode, Theme};

let theme = Theme::preset("catppuccin-mocha")?;
let resolved = theme.resolve(ColorMode::Dark)?;
let accent = resolved.variant.defaults.accent_color;  // Rgba
# Ok::<(), native_theme::error::Error>(())
```

`theme.resolve(mode)` returns a `Resolved` wrapper containing `variant`
(the `ResolvedTheme`) plus the effective icon-set and icon-theme metadata.

### Layer user overrides on a preset

```rust
use native_theme::theme::Theme;

let mut theme = Theme::preset("nord")?;
let overrides = Theme::from_toml(r##"
    name = "My Nord"
    [light.defaults]
    accent_color = "#ff6600"
"##)?;
theme.merge(&overrides);
# Ok::<(), native_theme::error::Error>(())
```

`merge` fills in only the fields present in the overlay; everything else stays
from the base preset.

### Apply an overlay to the OS theme at runtime

```rust,no_run
use native_theme::{SystemTheme, theme::Theme};

let sys = SystemTheme::from_system()?;
let overlay = Theme::from_toml(r##"
    [light.defaults]
    accent_color = "#ff6600"
"##)?;
let customised = sys.with_overlay(&overlay)?;
# Ok::<(), native_theme::error::Error>(())
```

### Cheap dark-mode poll

```rust,no_run
use native_theme::detect::system_is_dark;

if system_is_dark() {
    // …
}
```

Cached, does not run the full theme-reader pipeline.

## Icon sets

<p align="center">
  <img src="../docs/assets/spinner-material.gif" alt="Material spinner" height="80">
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="../docs/assets/spinner-lucide.gif" alt="Lucide spinner" height="80">
</p>

Semantic icon roles map to platform-appropriate glyphs via typed per-set loaders:

```rust,ignore
use native_theme::icons::{MaterialLoader, FreedesktopLoader};
use native_theme::theme::IconRole;

// Bundled set (no system dependencies).
let copy = MaterialLoader::new(IconRole::ActionCopy).load();

// System icon theme on Linux (reads the active theme, e.g. breeze-dark).
let sys = FreedesktopLoader::new(IconRole::ActionCopy)
    .theme("breeze-dark")
    .load();
```

Animated spinners respect the OS `prefers-reduced-motion` preference:

```rust,ignore
use native_theme::icons::MaterialLoader;
use native_theme::detect::prefers_reduced_motion;

if let Some(anim) = MaterialLoader::load_indicator() {
    if prefers_reduced_motion() {
        let _static_fallback = anim.first_frame();
    } else {
        // Play the animation — AnimatedIcon has Frames and Transform variants.
    }
}
```

Connector crates provide toolkit playback helpers (see the `native-theme-gpui`
and `native-theme-iced` READMEs).

## Feature flags

```toml
[dependencies]
native-theme = { version = "0.5", features = ["native"] }
```

`native` is a meta-feature enabling every OS reader for the current target
(`kde` + `portal` + `macos` + `windows`). Individual features:

| Feature | Role |
|---|---|
| `kde` / `portal` / `macos` / `windows` | Platform-specific theme readers |
| `linux` | Meta-feature: `kde` + `portal` |
| `watch` | Runtime theme-change notifications |
| `system-icons` | Linux freedesktop icon-theme lookups |
| `material-icons` / `lucide-icons` | Bundle those icon sets |
| `svg-rasterize` | Rasterize SVG icons to RGBA via resvg |

OS-specific dependencies are target-gated — `native` on macOS only pulls in
macOS-related deps.

## Links

- [API reference on docs.rs](https://docs.rs/native-theme)
- Connectors: [`native-theme-gpui`](../connectors/native-theme-gpui/), [`native-theme-iced`](../connectors/native-theme-iced/)
- [Showcase examples](../connectors/)
- [CHANGELOG](../CHANGELOG.md)

## License

Licensed under any of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be triple licensed as above, without any additional terms or conditions.
