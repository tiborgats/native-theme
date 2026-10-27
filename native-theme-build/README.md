# native-theme-build

Build-time code generation for [native-theme](https://crates.io/crates/native-theme)
custom icon roles.

## What it does

Reads a TOML file describing your app's custom icon roles at build time and
generates a Rust enum that implements `native_theme::icons::IconProvider`. The
generated enum maps each role to its platform-specific identifier (SF Symbols,
Segoe Fluent, freedesktop, Material, Lucide) and optionally embeds bundled SVG
data via `include_bytes!`.

Use this when your app needs domain-specific icons (for example `PlayPause`,
`SkipForward`, `GitBranch`) that aren't in `native-theme`'s built-in `IconRole`.
If the bundled `IconRole` variants cover your needs, you don't need this crate.

## How it fits

This crate is a *build-time* dependency. Your final binary links against
`native-theme` only — nothing from `native-theme-build` is linked into it. You add
it to `[build-dependencies]` and call it from `build.rs`. It builds with Rust
1.88 or newer, the workspace's `rust-version`.

## Quick start

Add the build dependency, and native-theme, which the generated code names:

```toml
[build-dependencies]
native-theme-build = "0.7"

[dependencies]
native-theme = { version = "0.7", features = ["material-icons"] }
```

native-theme's loaders give `None` without their set's feature, for a
generated enum too: `material-icons` for a bundled `material` theme,
`lucide-icons` for `lucide`, and `system-icons` for the `sf-symbols`,
`segoe-fluent` and `freedesktop` names.

Describe your icons in TOML:

```toml
# icons/icons.toml
name = "app-icon"
roles = ["play-pause", "skip-forward", "volume-up"]
bundled-themes = ["material"]
system-themes = ["sf-symbols", "segoe-fluent", "freedesktop"]
```

A theme name is one of `material`, `lucide`, `sf-symbols`, `segoe-fluent` and
`freedesktop`. Each listed theme needs its own `mapping.toml` in a folder of
that name next to the TOML file (see [Directory layout](#directory-layout)).

Generate the enum in `build.rs`:

```rust,no_run
use native_theme_build::UnwrapOrExit;

fn main() {
    native_theme_build::generate_icons("icons/icons.toml")
        .unwrap_or_exit()
        .emit_cargo_directives()
        .expect("failed to write generated code");
}
```

The generated file is named after `name` in snake_case (`app_icon.rs`), or
after the `.enum_name()` override when the builder sets one. Include it in your
app:

<!-- `ignore`: `OUT_DIR` exists only in a crate with a build script, and a
doctest has none, so this line cannot compile on its own. -->
```rust,ignore
include!(concat!(env!("OUT_DIR"), "/app_icon.rs"));
```

and use it:

```rust,no_run
use native_theme::icons::MaterialLoader;
# use std::borrow::Cow;
# use native_theme::icons::IconProvider;
# use native_theme::theme::IconSet;
# #[derive(Debug)]
# enum AppIcon { PlayPause }
# impl IconProvider for AppIcon {
#     fn icon_name(&self, _: IconSet) -> Option<&str> { None }
#     fn icon_svg(&self, _: IconSet) -> Option<Cow<'static, [u8]>> { None }
# }

let icon = MaterialLoader::new(&AppIcon::PlayPause).load();
```

An application that depends on a connector alone generates the enum with the
connector's re-export of native-theme as its crate path,
`IconGenerator::new().crate_path("native_theme_iced::native_theme")` (or
`native_theme_gpui::native_theme`, `native_theme_egui::native_theme`), and
names the loaders through it.

## Core concepts

- **Role** — a semantic name (`PlayPause`). Declared in `roles = [...]`; becomes a PascalCase enum variant.
- **Bundled theme** — SVG assets that ship inside your binary via `include_bytes!`. Declared in `bundled-themes`.
- **System theme** — identifier-only mapping. The actual icon is looked up at runtime by the OS (SF Symbols on macOS, Segoe Fluent on Windows, freedesktop on Linux). Declared in `system-themes`.
- **Generated enum** — produced at build time into `$OUT_DIR`. Implements `native_theme::icons::IconProvider`, so it plugs into every per-set loader (`MaterialLoader::new(&icon)`, `FreedesktopLoader::new(&icon)`, …).

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
    mapping.toml       # role → Segoe Fluent glyph (see Mapping format)
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

DE-aware form, for freedesktop, where different desktop environments use
different names:

```toml
play-pause = { kde = "media-playback-start", default = "media-play" }
```

A `default` key is required for every DE-aware entry; it is the name on every
other desktop and on every other platform, as the desktop is read on Linux
only. A bundled theme rejects the form, since it embeds one SVG per role. The
desktop keys are `kde`, `gnome`, `xfce`, `cinnamon`, `mate`, `lxqt`, `budgie`,
`hyprland`, `sway`, `river`, `niri`, `wayfire` and `cosmic`; an unknown key is a
warning.

A `segoe-fluent` value is a codepoint written `0x…` (`0xE8BB`), a Segoe Fluent
glyph name native-theme knows (`ChromeClose`), or a Windows stock icon
(`SIID_*`, `IDI_QUESTION`); native-theme's `SegoeFluentLoader` loads nothing
for any other name.

### Multi-file builder

For projects with several TOML files or a custom enum name:

```rust,no_run
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
- A role is declared in two TOML files (multi-source builder), or twice in one
- A DE-aware entry is missing its `default` key, or sits in a bundled theme
- An unknown theme name, or a theme listed twice or in both lists
- A role or enum name that makes no Rust identifier, or two roles that make the
  same variant
- An icon name that is empty, or holds control or invisible characters, a path
  separator or `..`
- A `.crate_path()` or `.derive()` value that is not a Rust path

With `.unwrap_or_exit()`, each is printed as a `cargo::error` that names the
file involved where there is one. Warnings, such as an SVG no mapping names, an
unknown desktop key or a config with no roles or no themes, go out as
`cargo::warning` lines from `.emit_cargo_directives()`.

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
