# native-theme-build

Build-time code generation for [native-theme](https://crates.io/crates/native-theme)
custom icon roles.

## What it does

Reads a TOML file describing your app's custom icon roles at build time and
generates a Rust enum that implements `native_theme::theme::IconProvider`. The
generated enum maps each role to its platform-specific identifier (SF Symbols,
Segoe Fluent, freedesktop, Material, Lucide) and optionally embeds bundled SVG
data via `include_bytes!`.

Use this when your app needs domain-specific icons (for example `PlayPause`,
`SkipForward`, `GitBranch`) that aren't in `native-theme`'s built-in `IconRole`.
If the bundled `IconRole` variants cover your needs, you don't need this crate.

## How it fits

This crate is a *build-time* dependency. Your final binary links against
`native-theme` only — nothing from `native-theme-build` is at runtime. You add
it to `[build-dependencies]` and call it from `build.rs`.

## Quick start

Add the build dependency:

```toml
[build-dependencies]
native-theme-build = "0.5"
```

Describe your icons in TOML:

```toml
# icons/icons.toml
name = "app-icon"
roles = ["play-pause", "skip-forward", "volume-up"]
bundled-themes = ["material"]
system-themes = ["sf-symbols", "segoe-fluent", "freedesktop"]
```

Generate the enum in `build.rs`:

```rust,ignore
use native_theme_build::UnwrapOrExit;

fn main() {
    native_theme_build::generate_icons("icons/icons.toml")
        .unwrap_or_exit()
        .emit_cargo_directives()
        .expect("failed to write generated code");
}
```

Include and use it in your app:

```rust,ignore
include!(concat!(env!("OUT_DIR"), "/app_icon.rs"));

use native_theme::icons::MaterialLoader;

let icon = MaterialLoader::new(&AppIcon::PlayPause).load();
```

## Core concepts

- **Role** — a semantic name (`PlayPause`). Declared in `roles = [...]`; becomes a PascalCase enum variant.
- **Bundled theme** — SVG assets that ship inside your binary via `include_bytes!`. Declared in `bundled-themes`.
- **System theme** — identifier-only mapping. The actual icon is looked up at runtime by the OS (SF Symbols on macOS, Segoe Fluent on Windows, freedesktop on Linux). Declared in `system-themes`.
- **Generated enum** — produced at build time into `$OUT_DIR`. Implements `native_theme::theme::IconProvider`, so it plugs into every per-set loader (`MaterialLoader::new(&icon)`, `FreedesktopLoader::new(&icon)`, …).

## Common recipes

### Directory layout

```text
icons/
  icons.toml           # master TOML passed to generate_icons()
  material/
    mapping.toml       # role → SVG filename
    play_pause.svg
    skip_next.svg
    volume_up.svg
  sf-symbols/
    mapping.toml       # role → SF Symbol name
  segoe-fluent/
    mapping.toml       # role → Segoe codepoint
  freedesktop/
    mapping.toml       # role → freedesktop icon name
```

### Mapping format

Simple form:

```toml
play-pause = "play_pause"
skip-forward = "skip_next"
volume-up = "volume_up"
```

DE-aware form (freedesktop only — different desktop environments use different names):

```toml
play-pause = { kde = "media-playback-start", default = "media-play" }
```

A `default` key is required for every DE-aware entry.

### Multi-file builder

For projects with several TOML files or a custom enum name:

```rust,ignore
use native_theme_build::UnwrapOrExit;

fn main() {
    native_theme_build::IconGenerator::new()
        .source("icons/media.toml")
        .source("icons/navigation.toml")
        .enum_name("AppIcon")
        .generate()
        .unwrap_or_exit()
        .emit_cargo_directives()
        .expect("failed to write generated code");
}
```

Paths are resolved relative to `CARGO_MANIFEST_DIR`. Both APIs emit
`cargo::rerun-if-changed` directives so edits to any TOML or SVG trigger a
rebuild.

### What compile-time validation catches

- A role is missing from a mapping file (every role must be present in every theme)
- An SVG file is missing for a bundled theme
- A role name appears in a mapping file but is not declared in the master TOML
- A role is declared in two TOML files (multi-source builder)
- A DE-aware entry is missing its `default` key

Each of these emits a build error pointing at the offending file and line.

## Links

- [API reference on docs.rs](https://docs.rs/native-theme-build)
- [`native-theme` crate](../native-theme/) — the runtime side
- [CHANGELOG](../CHANGELOG.md)

## License

Licensed under any of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.
