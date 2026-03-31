# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.2] - 2026-03-31

### Added

- `Deserialize` derive on all `Resolved*` types (enables caching, IPC, test fixtures)
- `Serialize` and `Deserialize` on `IconData`, `AnimatedIcon`, and `TransformAnimation`
- `Copy`, `Eq`, `Hash` derives on `DialogButtonOrder`; `Eq`, `Hash` on `LinuxDesktop`
- `#[must_use]` on 20+ public functions across all four crates
- `#[non_exhaustive]` on `BuildError` enum
- `Debug` and `Clone` derives on `IconGenerator`, `GenerateOutput`, `AnimatedImageSources`, `AnimatedSvgHandles`
- `into_image_source()` consuming variant in gpui connector
- KDE reader: `accent_foreground`, `list.background`/`foreground` from live color scheme
- GNOME reader: portal `reduce-motion` and gsettings `high-contrast` detection

### Changed

- `Error::Unsupported` now carries a `&'static str` context payload
- `icon_set` field on `ThemeVariant` changed from `Option<String>` to `Option<IconSet>` (validated at parse time)
- `rasterize_svg()` uses `Error::Format` instead of `Error::Unavailable` for invalid dimensions
- `BuildErrors` inner field is now private; access via `errors()`, `into_errors()`, `len()`, `is_empty()`, and `IntoIterator`
- gpui icon mapping functions (`lucide_name_for_gpui_icon`, `material_name_for_gpui_icon`, `freedesktop_name_for_gpui_icon`) return `&'static str` instead of `Option<&'static str>`
- iced `from_preset()` and `from_system()` return `(Theme, ResolvedThemeVariant)` tuple

### Fixed

- KDE Breeze preset: `radius` 4 -> 5, `focus_ring_width`/`focus_ring_offset` swapped, `line_height` 1.4 -> 1.36, four incorrect `icon_sizes`, `progress_bar.min_width` mismap, `spinner.diameter`, `expander.arrow_size`, `switch` dimensions
- KDE reader: `defaults.border` no longer overwritten with accent color; `forceFontDPI` read from correct file
- Adwaita preset: `radius` 12 -> 9, `radius_lg` 14 -> 15, `line_height` 1.4 -> 1.21, `focus_ring_offset` 1 -> -2, `section_heading` weight 400 -> 700
- macOS Sonoma preset: corrected geometry and metric values across both full and live presets
- Windows 11 preset: corrected geometry and metric values across both full and live presets
- gpui connector: `colorize_svg()` now handles self-closing SVG tags correctly
- Build crate: simplified error handling pipeline, improved codegen

## [0.5.1] - 2026-03-30

### Changed

- Renamed types and tightened visibility across core and build crates
- Build crate Result-based API for validation diagnostics
- Simplified GNOME/KDE readers and polished connector APIs
- Expanded widget resolved types and cleaned up build crate tests

### Fixed

- Windows compilation — swapped `icon_name` args, Rust 2024 unsafe blocks
- macOS compilation errors
- Test compilation — stale call sites after API changes
- iced screenshot delay to avoid blank capture on Windows
- CI: removed tag trigger from docs workflow

## [0.5.0] - 2026-03-28

### Added

- Per-widget data model: 25 `XxxTheme` / `ResolvedXxx` struct pairs (Window, Button, Input, Checkbox, Menu, Tooltip, Scrollbar, Slider, ProgressBar, Tab, Sidebar, Toolbar, StatusBar, List, Popover, Splitter, Separator, Switch, Dialog, Spinner, ComboBox, SegmentedControl, Card, Expander, Link)
- `ThemeDefaults` struct with ~40 global properties (colors, fonts, spacing, icon sizes, accessibility)
- `FontSpec` for per-widget font specification (family, size, weight)
- `TextScale` with 4 typographic roles (caption, section_heading, dialog_title, display)
- `IconSizes` struct (toolbar, small, large, dialog, panel)
- `DialogButtonOrder` enum (TrailingAffirmative / LeadingAffirmative)
- `ThemeSpacing` struct (xxs through xxl)
- `define_widget_pair!` macro generating paired Option/Resolved structs from a single definition
- `ResolvedThemeVariant` type where all fields are guaranteed populated (non-optional)
- `ResolvedThemeDefaults`, `ResolvedFontSpec`, `ResolvedThemeSpacing`, `ResolvedIconSizes`, `ResolvedTextScale`, `ResolvedTextScaleEntry` types
- `ThemeResolutionError` listing missing field paths; `Error::Resolution` variant
- `ThemeVariant::resolve()` with ~90 inheritance rules in 4 phases (defaults-internal, safety-nets, widget-from-defaults, widget-to-widget)
- `ThemeVariant::validate()` producing `ResolvedThemeVariant` or listing all missing fields
- `SystemTheme` type returned by `from_system()` with `active()`, `pick()`, `with_overlay()`, `with_overlay_toml()`
- Live platform presets (geometry-only, internal): `kde-breeze-live`, `adwaita-live`, `macos-sonoma-live`, `windows-11-live`
- `platform_preset_name()` mapping the current OS to its live preset
- `list_presets_for_platform()` filtering presets by current OS
- `system_is_dark()` cross-platform cached dark-mode detection (Linux gsettings/kdeglobals, macOS AppleInterfaceStyle, Windows UISettings)
- KDE reader: per-widget fonts (menuFont, toolBarFont), WM title bar colors, text scale via Kirigami multipliers, icon sizes from index.theme, accessibility (AnimationDurationFactor, forceFontDPI)
- GNOME reader: gsettings fonts (font-name, monospace-font-name, titlebar-font), text scale via CSS percentages, accessibility (text-scaling-factor, enable-animations, overlay-scrolling), icon-theme
- macOS reader: per-widget fonts (+menuFontOfSize:, +toolTipsFontOfSize:, +titleBarFontOfSize:), NSFont.TextStyle text scale, additional NSColor values, scrollbar overlay mode, accessibility (reduce_motion, high_contrast, reduce_transparency, text_scaling_factor)
- Windows reader: NONCLIENTMETRICSW per-widget fonts, DwmGetColorizationColor title bar, GetSysColor widget colors, text scale factor, high contrast, icon sizes via GetSystemMetrics

### Changed

- `ThemeVariant` composes `ThemeDefaults` + 25 per-widget structs instead of flat `ThemeColors`/`ThemeFonts`/`ThemeGeometry`
- `from_system()` and `from_system_async()` return `SystemTheme` instead of `ThemeSpec`
- gpui and iced connector `to_theme()` accept `&ResolvedThemeVariant` instead of `&ThemeVariant`
- All 16 preset TOMLs rewritten for per-widget structure; platform presets slimmed to design constants only
- `impl_merge!` macro extended with `optional_nested` category for per-widget font fields
- Both gpui and iced showcase examples updated for `SystemTheme` / `ResolvedThemeVariant` API

### Removed

- `ThemeColors` flat struct (replaced by `ThemeDefaults` base colors + per-widget color fields)
- `ThemeFonts` struct (replaced by `FontSpec` on `ThemeDefaults` + per-widget font fields)
- `ThemeGeometry` struct (replaced by per-widget geometry fields)
- `WidgetMetrics` and its 12 sub-structs (replaced by per-widget sizing fields on each `XxxTheme`)
- `default` preset (replaced by platform detection via `platform_preset_name()` and live presets)

### Migration from v0.4.x

**Data model:** `variant.colors.accent` -> `variant.defaults.accent`, `variant.fonts.family` -> `variant.defaults.font.family`, `variant.geometry.radius` -> `variant.defaults.radius`. Per-widget fields like `variant.button.min_height` replace `variant.widget_metrics.button.min_height`.

**from_system():**

```rust,ignore
// Before (v0.4.x)
let nt: ThemeSpec = from_system().unwrap_or_else(|_| ThemeSpec::preset("adwaita").unwrap());
let variant = nt.pick_variant(true).unwrap();

// After (v0.5.0)
let system: SystemTheme = from_system().unwrap();
let resolved: &ResolvedThemeVariant = system.active(); // all fields guaranteed
```

**Connectors:**

```rust,ignore
// Before (v0.4.x)
let theme = to_theme(variant, "My App", is_dark);

// After (v0.5.0)
let mut v = variant.clone();
v.resolve();
let resolved = v.validate().unwrap();
let theme = to_theme(&resolved, "My App", is_dark);

// Or from SystemTheme (already resolved):
let theme = to_theme(system.active(), "My App", system.is_dark);
```

## [0.4.1] - 2026-03-20

### Added

- `CONTRIBUTING.md` with development workflow and testing guide
- `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1)
- `SECURITY.md` with responsible disclosure policy
- GitHub issue templates (bug report, feature request) using YAML forms
- Pull request template with CI checklist
- Animated icon sections in gpui and iced connector READMEs
- Animated icon showcase demonstrations in both gpui and iced examples
- CLI argument support (`--tab`, `--preset`) for showcase examples
- GIF generation script for bundled spinner animations
- Screenshot automation (`--screenshot` flag) for iced showcase example
- CI workflow for automated screenshot generation on Linux, macOS, and Windows
- Showcase screenshots embedded in root, iced, and gpui READMEs
- Spinner GIFs embedded in root README
- `#![warn(missing_docs)]` crate-level lint attribute in all workspace crates
- Doc comments for all public API items in native-theme core crate

### Changed

- Root README updated with animated icons section
- Version references updated from 0.3.x to 0.4.x across all documentation

### Fixed

- Broken intra-doc link for `iced::time::every()` in native-theme-iced
- Missing documentation warnings that caused CI failures under `-Dwarnings`
- Formatting violations in gpui showcase example

## [0.4.0] - 2026-03-18

### Added

- `AnimatedIcon` enum with `Frames` and `Transform` variants for animated icon data
- `TransformAnimation` enum with `Spin` variant for continuous rotation
- `Repeat` enum controlling animation looping behavior
- `AnimatedIcon::first_frame()` method returning a static fallback frame
- `loading_indicator(icon_set)` function dispatching to platform-appropriate spinner animations
- `prefers_reduced_motion()` function querying OS accessibility settings (Linux gsettings, macOS NSWorkspace, Windows UISettings)
- Bundled Lucide loader spinner (spin transform) and freedesktop `process-working` sprite sheet loading
- Freedesktop sprite sheet parser for runtime `process-working.svg` animation loading
- gpui connector: `animated_frames_to_image_sources()` and `with_spin_animation()` for animation playback
- iced connector: `animated_frames_to_svg_handles()` and `spin_rotation_radians()` for animation playback

### Changed

- `IconRole::StatusLoading` renamed to `IconRole::StatusBusy` (static icon for busy state)

### Removed

- `IconRole::StatusLoading` variant (use `loading_indicator()` for animated loading indicators, or `IconRole::StatusBusy` for a static busy icon)

### Migration from v0.3.x

**Before (v0.3.x):**

```rust,ignore
use native_theme::{load_icon, IconRole};

// Static loading icon
let icon = load_icon(IconRole::StatusLoading, "material");
```

**After (v0.4.0):**

```rust,ignore
use native_theme::{loading_indicator, prefers_reduced_motion, AnimatedIcon};

// Animated loading indicator with platform-native style
if let Some(anim) = loading_indicator("material") {
    // Check accessibility preference first
    if prefers_reduced_motion() {
        let static_icon = anim.first_frame();
        // Render a single static frame
    } else {
        match &anim {
            AnimatedIcon::Frames { frames, frame_duration_ms, .. } => {
                // Cycle through frames on a timer
            }
            AnimatedIcon::Transform { icon, animation } => {
                // Apply continuous rotation to the icon
            }
        }
    }
}

// If you just need a static busy icon (not animated):
use native_theme::{load_icon, IconRole};
let busy = load_icon(IconRole::StatusBusy, "material");
```

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

- `ThemeSpec::pick_variant()` method for selecting the appropriate theme variant with cross-fallback
- `#[must_use]` annotations on all public API functions and key types (`ThemeSpec`, `IconData`)

### Changed

- `system_icon_theme()` and `system_is_dark()` now cache results with `OnceLock` (eliminates redundant subprocess spawns)
- `colorize_svg` renamed to `colorize_monochrome_svg` in iced connector with documentation clarifying monochrome-only contract
- Improved `to_theme` comment in gpui connector explaining the `apply_config`/restore pattern
- `pre-release-check.sh` uses `jq` instead of `python3` for JSON parsing (with bash fallback)

### Deprecated

- `pick_variant()` free functions in gpui and iced connectors (use `ThemeSpec::pick_variant()` instead)

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
- Moved preset API from free functions to `ThemeSpec` associated methods (`preset()`, `from_toml()`, `from_file()`, `list_presets()`, `to_toml()`)
- Renamed primary/secondary color fields with prefix (`primary_background`, `primary_foreground`, `secondary_background`, `secondary_foreground`)

### Removed

- `CoreColors`, `ActionColors`, `StatusColors`, `InteractiveColors`, `PanelColors`, `ComponentColors` nested sub-structs (replaced by flat `ThemeColors`)
- Free-standing `preset()`, `from_toml()`, `from_file()`, `list_presets()`, `to_toml()` functions (now methods on `ThemeSpec`)

## [0.1.0] - 2026-03-07

### Added

- `ThemeSpec` data model with 36 semantic color roles, fonts, geometry, and spacing
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

[0.5.2]: https://github.com/tiborgats/native-theme/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/tiborgats/native-theme/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/tiborgats/native-theme/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/tiborgats/native-theme/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/tiborgats/native-theme/compare/v0.3.3...v0.4.0
[0.3.3]: https://github.com/tiborgats/native-theme/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/tiborgats/native-theme/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/tiborgats/native-theme/compare/v0.3...v0.3.1
[0.3.0]: https://github.com/tiborgats/native-theme/compare/v0.2.0...v0.3
[0.2.0]: https://github.com/tiborgats/native-theme/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/tiborgats/native-theme/releases/tag/v0.1.0
