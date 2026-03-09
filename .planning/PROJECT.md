# native-theme

## What This Is

An independent, toolkit-agnostic Rust crate that provides a unified theme data model (36 semantic color roles, fonts, geometry, spacing), 17 TOML-serializable preset theme files for major desktop/mobile platforms and popular community color schemes, and optional runtime OS theme reading behind feature flags. It fills a genuine ecosystem gap — no crate currently unifies OS theme data into a common, toolkit-agnostic format that works across egui, iced, gpui, slint, dioxus, and tauri.

## Core Value

Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.

## Current Milestone: v0.3 Icons

**Goal:** Platform-native icon loading — semantic icon roles mapped to OS-native icon systems (SF Symbols, Segoe Fluent, freedesktop) with bundled cross-platform fallbacks (Material, Lucide).

**Target features:**
- IconRole enum (42 semantic roles: dialog, window, action, navigation, file, status, system)
- IconData type (SVG bytes or rasterized RGBA pixels)
- Platform icon loading: macOS (NSImage/SF Symbols), Windows (SHGetStockIconInfo + Segoe Fluent), Linux (freedesktop icon theme spec)
- Bundled fallback icon sets: Material Symbols (Apache 2.0), Lucide (ISC)
- icon_theme field on ThemeVariant with preset assignments
- Public API: load_icon(), icon_name(), system_icon_set()
- Feature flags: system-icons, material-icons, lucide-icons
- Connector updates: IconData → gpui/iced image conversion

## Requirements

### Validated

- ✓ Data model with 36 semantic color roles, fonts, geometry, and spacing as plain Rust types — v0.1
- ✓ Rgba color type with 8-bit-per-channel sRGB + alpha, custom hex serde (#RRGGBB / #RRGGBBAA) — v0.1
- ✓ All fields Option<T> with #[non_exhaustive] structs for forward compatibility — v0.1
- ✓ TOML serialization/deserialization mapping 1:1 to struct field names — v0.1
- ✓ Theme layering via merge() — load base preset, overlay with user overrides — v0.1
- ✓ Bundled presets: default, kde-breeze, adwaita, windows-11, macos-sonoma, material, ios — v0.1
- ✓ Preset loading API: preset(), list_presets(), from_toml(), from_file(), to_toml() — v0.1
- ✓ Error type with Unsupported/Unavailable/Format/Platform variants — v0.1
- ✓ Linux KDE reader: from_kde() — sync, parses ~/.config/kdeglobals (feature "kde") — v0.1
- ✓ Linux GNOME reader: from_gnome() — async, ashpd portal + Adwaita defaults (feature "portal") — v0.1
- ✓ Cross-platform dispatch: from_system() — auto-detects platform/DE — v0.1
- ✓ Windows reader: from_windows() — UISettings + GetSystemMetrics (feature "windows") — v0.1
- ✓ Community presets: Catppuccin (4 flavors), Nord, Dracula, Gruvbox, Solarized, Tokyo Night, One Dark — v0.1
- ✓ Documentation and README with adapter examples for egui/iced/slint — v0.1
- ✓ Tests: round-trip serde, preset loading, Rgba hex parsing, platform reader unit tests — v0.1
- ✓ Flat ThemeColors with 36 direct Option<Rgba> fields — v0.2
- ✓ NativeTheme associated methods: preset(), from_toml(), from_file(), list_presets(), to_toml() — v0.2
- ✓ ThemeGeometry extensions: radius_lg, shadow — v0.2
- ✓ macOS reader: from_macos() via objc2-app-kit with dual-variant support — v0.2
- ✓ Windows reader: accent shades, system fonts, spacing, DPI-aware geometry — v0.2
- ✓ Linux readers: KDE+portal overlay, D-Bus detection, GNOME fonts, from_linux() — v0.2
- ✓ Widget metrics data model (12 per-widget sub-structs) + platform sources — v0.2
- ✓ CI pipeline: GitHub Actions, feature matrix, semver-checks, clippy/fmt — v0.2
- ✓ Cargo workspace with native-theme-gpui and native-theme-iced connector crates — v0.2
- ✓ gpui connector: 108-field ThemeColor mapping, fonts, geometry, widget metrics — v0.2
- ✓ iced connector: palette/font/style/widget-metrics mapping — v0.2
- ✓ Publishing prep: workspace metadata, licenses, changelog, documentation — v0.2

### Active

- [ ] IconRole enum with 42 semantic icon roles across 7 categories
- [ ] IconData enum (Svg bytes, Rgba pixels) as platform-agnostic icon output
- [ ] icon_theme field on ThemeVariant with preset-specific assignments
- [ ] macOS icon loading: NSImage(systemSymbolName:) → rasterized RGBA (feature "macos")
- [ ] Windows icon loading: SHGetStockIconInfo + Segoe Fluent Icons font glyphs (feature "windows")
- [ ] Linux icon loading: freedesktop icon theme spec with index.theme parsing (feature "kde" or "portal")
- [ ] Bundled Material Symbols SVGs as cross-platform fallback (feature "material-icons")
- [ ] Bundled Lucide SVGs as optional icon set (feature "lucide-icons")
- [ ] Public API: load_icon(), icon_name(), system_icon_set()
- [ ] Connector updates: IconData conversion for gpui and iced

### Out of Scope

- iOS reader (from_ios()) — deferred to post-1.0
- Android reader (from_android()) — deferred to post-1.0
- Change notification system — deferred to post-1.0; users can poll or use toolkit observers
- Named palette colors (platform-specific reds, blues, etc.) — too platform-specific to standardize
- Toolkit adapters inside the core crate — adapters live in connector sub-crates or consuming app code
- Accessibility flags in the data model — environment signals detected by consuming app
- CSS/SCSS export format — trivially implementable by consumers

## Context

Shipped v0.1 with ~7,000 LOC (3,349 Rust + 2,566 TOML presets + 1,100 integration tests).
Tech stack: Rust edition 2024, serde + toml (core), configparser (KDE), ashpd (GNOME portal), windows crate (Windows).
17 bundled presets, 3 platform readers, 140+ tests with zero failures.
Prior art: system-theme 0.3.0, cosmic-theme, dark-light 2.0 — native-theme unifies what these do partially.
v0.2 shipped: API polish (flat ThemeColors, NativeTheme methods), macOS reader, enhanced Windows/Linux readers, widget metrics, CI, toolkit connectors (gpui + iced).
v0.3 focus: Platform-native icon loading with semantic icon roles, OS-native icon systems, and bundled cross-platform fallbacks.

## Constraints

- **Zero GUI toolkit deps**: The crate must not depend on any GUI toolkit (egui, iced, gpui, etc.)
- **Minimal core deps**: With no feature flags, only serde + toml as dependencies
- **Sync by default**: Core crate and most readers are sync; only portal feature requires async runtime
- **sRGB color space**: All Rgba values in sRGB; macOS P3 colors converted on read
- **Logical pixels**: All sizes in logical pixels (CSS-like); consuming toolkit handles DPI scaling
- **Edition 2024**: Rust edition 2024
- **License**: MIT OR Apache-2.0

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Independent crate (not toolkit-specific) | Fills ecosystem gap; benefits all Rust GUI toolkits equally | ✓ Good — clean separation proven across 3 adapter examples |
| Rgba with u8 + alpha from day one | All platform sources are 8-bit; alpha needed for shadow/opacity tokens | ✓ Good — used by all 3 platform readers without conversion |
| 36 semantic color roles | 6 is too few, 100+ too unwieldy; 36 covers what every toolkit can map | ✓ Good — all 17 presets and 3 readers populate these meaningfully |
| Option<T> for all fields | Platforms expose different subsets; enables layering | ✓ Good — KDE maps 35/36, GNOME maps 4+base, Windows maps 6+base |
| TOML format (not JSON/RON) | Human-readable, human-editable | ✓ Good — 17 preset files are readable and hand-editable |
| Separate kde/portal features | KDE sync, portal async; clean separation | ✓ Good — no tokio leakage to sync consumers |
| Named spacing scale (xxs-xxl) | Maps to any toolkit's spacing system | ✓ Good — all platform presets use native-appropriate values |
| impl_merge! macro for theme layering | DRY merge across 10+ structs, declarative field categories | ✓ Good — prevented desynchronization across 36 color fields |
| Single-variant reader output | Readers populate only light or dark based on detection | ✓ Good — consistent pattern across KDE/GNOME/Windows |
| Adwaita as universal fallback | Embedded preset guaranteed available at compile time | ✓ Good — from_system() and from_gnome() both use it reliably |

---
*Last updated: 2026-03-09 after v0.3 milestone start*
