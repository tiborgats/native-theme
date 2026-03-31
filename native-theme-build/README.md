# native-theme-build

Build-time code generation for [native-theme](https://crates.io/crates/native-theme)
custom icon roles.

This crate reads TOML icon definitions at build time and generates a Rust enum
that implements `native_theme::IconProvider`. The generated enum maps each icon
role to platform-specific identifiers (SF Symbols, Segoe Fluent, freedesktop,
Material, Lucide) and optionally embeds bundled SVG data via `include_bytes!`.

## Quick Start

Add the build dependency:

```toml
[build-dependencies]
native-theme-build = "0.5.2"
```

Create an icon definition TOML:

```toml
# icons/icons.toml
name = "app-icon"
roles = ["play-pause", "skip-forward", "volume-up"]
bundled-themes = ["material"]
system-themes = ["sf-symbols", "segoe-fluent", "freedesktop"]
```

Call `generate_icons()` in your `build.rs`:

```rust,ignore
fn main() {
    native_theme_build::generate_icons("icons/icons.toml");
}
```

Include and use the generated code:

```rust,ignore
include!(concat!(env!("OUT_DIR"), "/app_icon.rs"));

use native_theme::{load_custom_icon, IconSet};
let icon_data = load_custom_icon(&AppIcon::PlayPause, IconSet::Material);
```

## TOML Schema

The master TOML file declares the icon set name, roles, and which themes to support:

- **`name`** -- used to derive the generated enum name (`AppIcon`).
- **`roles`** -- kebab-case role names; each becomes a PascalCase enum variant.
- **`bundled-themes`** -- themes whose SVGs are embedded via `include_bytes!`.
- **`system-themes`** -- themes resolved at runtime by the OS (no embedded SVGs).

## Directory Layout

```text
icons/
  icons.toml           # Master TOML (the file passed to generate_icons)
  material/
    mapping.toml       # Role -> SVG filename mappings
    play_pause.svg
    skip_next.svg
    volume_up.svg
  sf-symbols/
    mapping.toml       # Role -> SF Symbol name mappings
  segoe-fluent/
    mapping.toml       # Role -> Segoe codepoint mappings
  freedesktop/
    mapping.toml       # Role -> freedesktop icon name mappings
```

## Mapping Format

Each theme directory contains a `mapping.toml` that maps roles to
theme-specific identifiers. Simple form:

```toml
play-pause = "play_pause"
skip-forward = "skip_next"
volume-up = "volume_up"
```

DE-aware form (for freedesktop themes that vary by desktop environment):

```toml
play-pause = { kde = "media-playback-start", default = "media-play" }
```

A `default` key is required for every DE-aware entry.

## Builder API

For projects with multiple TOML files or custom enum names:

```rust,ignore
fn main() {
    native_theme_build::IconGenerator::new()
        .source("icons/media.toml")
        .source("icons/navigation.toml")
        .enum_name("AppIcon")
        .generate();
}
```

Both APIs resolve paths relative to `CARGO_MANIFEST_DIR`, emit
`cargo::rerun-if-changed` directives for all referenced files, and write
the generated code to `OUT_DIR`.

## What Gets Generated

The output is a single `.rs` file containing:

- A `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]` enum with one
  variant per role.
- An `IconProvider` implementation with `icon_name()` returning the
  platform-specific identifier and `icon_svg()` returning
  `include_bytes!(...)` data for bundled themes.

## Validation

Build errors are emitted at compile time for:

- Missing roles in mapping files (every role must be present in every theme).
- Missing SVG files for bundled themes.
- Unknown role names in mapping files (not declared in the master TOML).
- Duplicate roles across multiple TOML files (builder API).
- Missing `default` key in DE-aware mapping entries.

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.
