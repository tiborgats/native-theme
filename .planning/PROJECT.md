# native-theme

## What This Is

An independent, toolkit-agnostic Rust crate that provides a unified theme data model (36 semantic color roles, fonts, geometry, spacing), 17 TOML-serializable preset theme files for major desktop/mobile platforms and popular community color schemes, and optional runtime OS theme reading behind feature flags. It fills a genuine ecosystem gap — no crate currently unifies OS theme data into a common, toolkit-agnostic format that works across egui, iced, gpui, slint, dioxus, and tauri.

## Core Value

Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.

## Current Milestone: v0.2

**Goal:** API polish, full desktop platform coverage, widget metrics, toolkit connectors, CI, and publishing prep — everything needed before broader adoption.

**Target features:**
- API refactors: flatten ThemeColors, move preset API to NativeTheme methods, add missing ThemeGeometry fields
- macOS reader via objc2-app-kit
- Windows reader enhancements (accent shades, fonts, spacing, capability checks)
- Linux reader enhancements (KDE+portal overlay, D-Bus backend detection, GNOME fonts, from_linux() fallback)
- Widget metrics data model + platform sources (KDE, GNOME, Windows, macOS)
- CI pipeline (GitHub Actions, feature flag matrix, semver-checks, clippy/fmt)
- Cargo workspace restructuring for connector sub-crates
- native-theme-gpui connector + upstream PR proposals
- native-theme-iced connector
- Publishing prep (metadata, licenses, changelog, doc examples, crates.io publish)

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

### Active

- [ ] Flatten ThemeColors to 36 direct fields (remove nested sub-structs)
- [ ] Move preset functions to impl NativeTheme associated functions
- [ ] Add ThemeGeometry fields: radius_lg, shadow
- [ ] macOS reader: from_macos() — NSColor + NSFont via objc2-app-kit (feature "macos")
- [ ] Windows reader: capability checks, accent shades, system fonts, spacing, primary_foreground derivation
- [ ] Linux KDE+portal overlay, D-Bus backend detection, GNOME font reading, from_linux() fallback
- [ ] Widget metrics data model (12 per-widget sub-structs) + platform sources
- [ ] CI pipeline: GitHub Actions, feature matrix, semver-checks, clippy/fmt
- [ ] Cargo workspace with native-theme-gpui and native-theme-iced connector crates
- [ ] gpui connector: color/font/geometry/widget-metrics mapping + upstream PR proposals
- [ ] iced connector: palette/font/style/widget-metrics mapping
- [ ] Publishing prep: metadata, licenses, changelog, doc examples, crates.io publish

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
v0.2 focus: API polish (breaking changes while pre-1.0), macOS reader, enhanced existing readers, widget metrics, CI, toolkit connectors (gpui + iced), and crates.io publishing.

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
*Last updated: 2026-03-08 after v0.2 milestone start*
