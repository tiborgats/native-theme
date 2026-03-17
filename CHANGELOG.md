# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.3] - 2026-03-17

### Added

- `IconProvider` trait for defining custom icon types that integrate with native-theme's loading system
- `load_custom_icon()` function dispatching custom icons through the same platform loader chain as built-in icons
- `load_system_icon_by_name()` function for loading platform icons by arbitrary name string
- `native-theme-build` crate: TOML-driven code generation for custom icon roles with `generate_icons()` and `IconGenerator` builder API
- DE-aware code generation: freedesktop mapping TOML entries can specify per-desktop-environment icon names (e.g., `{ kde = "view-visible", default = "view-reveal" }`)
- gpui connector: `custom_icon_to_image_source()` and `custom_icon_to_image_source_colored()` for loading custom icons
- iced connector: `custom_icon_to_image_handle()`, `custom_icon_to_svg_handle()`, and `custom_icon_to_svg_handle_colored()` for loading custom icons
- Icon mapping gap fills: Freedesktop `Notification` -> "notification-active", Material/Lucide `TrashFull` mappings
- Coverage tests: `no_unexpected_icon_gaps` and `all_roles_have_bundled_svg` prevent future mapping regressions

### Changed

- `IconRole` now implements `IconProvider`, delegating to built-in mapping functions
- Platform icon loaders (freedesktop, SF Symbols, Segoe Fluent) return `None` for unmapped roles instead of falling back to Material SVGs

### Removed

- Wildcard Material SVG fallback from `load_icon()` and all platform loaders (icons not found in the requested set now return `None`)

## [0.3.2] - 2026-03-14

### Added

- `NativeTheme::pick_variant()` method for selecting the appropriate theme variant with cross-fallback
- `#[must_use]` annotations on all public API functions and key types (`NativeTheme`, `IconData`)

### Changed

- `system_icon_theme()` and `system_is_dark()` now cache results with `OnceLock` (eliminates redundant subprocess spawns)
- `colorize_svg` renamed to `colorize_monochrome_svg` in iced connector with documentation clarifying monochrome-only contract
- Improved `to_theme` comment in gpui connector explaining the `apply_config`/restore pattern
- `pre-release-check.sh` uses `jq` instead of `python3` for JSON parsing (with bash fallback)

### Deprecated

- `pick_variant()` free functions in gpui and iced connectors (use `NativeTheme::pick_variant()` instead)

### Removed

- Dead `lighten`, `darken`, and `with_alpha` wrapper functions from gpui `derive` module

## [0.3.1] - 2026-03-13

### Added

- Meta-features (`linux-full`, `macos-full`, `windows-full`) for simplified feature gate configuration
- `system_icon_theme()` with DE-aware detection (KDE, GNOME, Xfce, Cinnamon, Mate, LxQt, Budgie)
- `bundled_icon_by_name()` for string-based icon lookup
- `load_freedesktop_icon_by_name()` for arbitrary freedesktop icon lookups
- `LinuxDesktop` enum expanded with Xfce, Cinnamon, Mate, LxQt, Budgie variants
- `LinuxDesktop` and `detect_linux_de()` made public
- Freedesktop icon name mapping for all 86 gpui-component icons
- SVG colorization support in iced connector (`to_svg_handle_colored`)

### Changed

- Target-gated OS dependencies so meta-features compile on all platforms
- Renamed `icon_theme` field to `icon_set` (with serde alias for backward compatibility)
- Updated bundled Material and Lucide SVGs to latest releases (86+ icons each)

### Fixed

- BMP rasterization in gpui connector (red/blue channel swap for colored SVG themes)
- Plasma 6 icon theme detection via `kdedefaults/kdeglobals` fallback
- Symbolic icon preference to avoid animation sprite sheets from freedesktop themes

## [0.3.0] - 2026-03-09

### Added

- Icon system: `IconRole` enum (42 semantic icon roles), `IconSet` enum, `IconData` type
- Bundled SVG icon sets: Material Design and Lucide (86+ icons each, ~300KB total)
- Linux freedesktop icon loading via `freedesktop-icons` crate
- macOS SF Symbols icon loading (compile-time stub with bundled fallback)
- Windows Segoe Fluent Icons loading (compile-time stub with bundled fallback)
- `load_icon()` cross-platform dispatch function
- `rasterize_svg()` for SVG-to-bitmap conversion via `resvg`
- gpui connector: `icon_name()` mapping, `to_image_source()` conversion
- iced connector: `to_svg_handle()` for SVG icon display

## [0.2.0] - 2026-03-09

### Added

- macOS reader (`from_macos()`) with light and dark variant detection
- `WidgetMetrics` with 12 per-widget sub-structs (Button, Checkbox, Input, ListItem, MenuItem, ProgressBar, Scrollbar, Slider, Splitter, Tab, Toolbar, Tooltip)
- `ThemeGeometry::radius_lg` and `shadow` fields for extended geometry support
- Linux D-Bus portal backend detection for improved desktop environment heuristics
- Portal overlay for KDE themes (`from_kde_with_portal()`)
- `native-theme-iced` connector crate for iced toolkit integration
- `native-theme-gpui` connector crate for gpui toolkit integration
- GitHub Actions CI pipeline with cross-platform matrix (Linux, macOS, Windows)
- Windows accent shade colors (AccentDark1-3, AccentLight1-3)
- Windows system font and DPI-aware geometry reading
- GNOME font data population via portal reader
- Async `from_system_async()` with D-Bus portal backend detection

### Changed

- Restructured as Cargo workspace with `native-theme`, `native-theme-iced`, and `native-theme-gpui` crates
- Flattened `ThemeColors` from nested sub-structs to 36 direct `Option<Rgba>` fields
- Moved preset API from free functions to `NativeTheme` associated methods (`preset()`, `from_toml()`, `from_file()`, `list_presets()`, `to_toml()`)
- Renamed primary/secondary color fields with prefix (`primary_background`, `primary_foreground`, `secondary_background`, `secondary_foreground`)

### Removed

- `CoreColors`, `ActionColors`, `StatusColors`, `InteractiveColors`, `PanelColors`, `ComponentColors` nested sub-structs (replaced by flat `ThemeColors`)
- Free-standing `preset()`, `from_toml()`, `from_file()`, `list_presets()`, `to_toml()` functions (now methods on `NativeTheme`)

## [0.1.0] - 2026-03-07

### Added

- `NativeTheme` data model with 36 semantic color roles, fonts, geometry, and spacing
- `Rgba` color type with hex string parsing and serialization
- `ThemeVariant` composing colors, fonts, geometry, and spacing
- TOML serialization and deserialization for all theme types
- 17 bundled presets (platform and community themes)
- KDE reader (`from_kde()`) parsing kdeglobals color scheme
- GNOME portal reader (`from_gnome()`) via D-Bus Settings portal
- Windows reader (`from_windows()`) using Windows registry
- Cross-platform `from_system()` dispatch with automatic desktop detection
- `impl_merge!` macro for recursive Option-based theme merging
- Deep merge support across all theme types

[0.3.3]: https://github.com/tiborgats/native-theme/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/tiborgats/native-theme/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/tiborgats/native-theme/compare/v0.3...v0.3.1
[0.3.0]: https://github.com/tiborgats/native-theme/compare/v0.2.0...v0.3
[0.2.0]: https://github.com/tiborgats/native-theme/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/tiborgats/native-theme/releases/tag/v0.1.0
