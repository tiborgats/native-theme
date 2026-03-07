# native-theme

## What This Is

An independent, toolkit-agnostic Rust crate that provides a unified theme data model (36 semantic color roles, fonts, geometry, spacing), TOML-serializable preset theme files for major desktop and mobile platforms, and optional runtime OS theme reading behind feature flags. It fills a genuine ecosystem gap — no crate currently unifies OS theme data into a common, toolkit-agnostic format that works across egui, iced, gpui, slint, dioxus, and tauri.

## Core Value

Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] Data model with 36 semantic color roles, fonts, geometry, and spacing as plain Rust types
- [ ] Rgba color type with 8-bit-per-channel sRGB + alpha, custom hex serde (#RRGGBB / #RRGGBBAA)
- [ ] All fields Option<T> with #[non_exhaustive] structs for forward compatibility
- [ ] TOML serialization/deserialization mapping 1:1 to struct field names
- [ ] Theme layering via merge() — load base preset, overlay with user overrides
- [ ] Bundled presets embedded via include_str!(): default, kde-breeze, adwaita (Phase 1), windows-11, macos-sonoma, material, ios (Phase 5)
- [ ] Preset loading API: preset(), list_presets(), from_toml(), from_file(), to_toml()
- [ ] Error type with Unsupported/Unavailable/Format/Platform variants
- [ ] Linux KDE reader: from_kde() — sync, parses ~/.config/kdeglobals via configparser (feature "kde")
- [ ] Linux GNOME reader: from_gnome() — async, reads freedesktop portal via ashpd + hardcoded Adwaita defaults (feature "portal")
- [ ] Cross-platform dispatch: from_system() — auto-detects platform/DE, calls appropriate reader
- [ ] Windows reader: from_windows() — UISettings + GetSystemMetrics (feature "windows")
- [ ] macOS reader: from_macos() — NSColor + NSFont via objc2-app-kit (feature "macos")
- [ ] Community presets: Catppuccin (4 flavors), Nord, Dracula, Gruvbox, Solarized, Tokyo Night, One Dark
- [ ] Documentation and README with adapter examples for egui/iced/slint
- [ ] iOS reader: from_ios() — UIColor + UIFont via objc2-ui-kit (feature "ios")
- [ ] Android reader: from_android() — JNI + NDK for Material You colors (feature "android")
- [ ] Tests: round-trip serde, preset loading, Rgba hex parsing edge cases, platform reader unit tests

### Out of Scope

- Widget metrics (Tier 2) — deferred to post-1.0; most toolkits can't consume per-widget metrics today
- Named palette colors (platform-specific reds, blues, etc.) — too platform-specific to standardize; adapters derive from semantic status colors
- Toolkit adapters inside the crate — adapters live in consuming app code (~50 lines each), keeping native-theme fully toolkit-agnostic
- Accessibility flags in the data model — dark/light, high contrast, reduced motion are environment signals detected separately by the consuming app
- Phase 2 (gsr adapter + Settings UI) — app-specific, not part of the crate
- crates.io publishing — not in scope for this milestone (code + tests only)

## Context

- The implementation spec is fully documented in `docs/IMPLEMENTATION.md` — all design decisions are resolved
- Prior art analyzed: system-theme 0.3.0, cosmic-theme, dark-light 2.0
- Platform capabilities exhaustively mapped across KDE, GNOME, Windows 11, macOS, iOS, Android
- Key dependency choices: serde + toml (core), configparser + dirs (KDE), ashpd (portal), windows crate, objc2-app-kit/objc2-ui-kit, jni + ndk (Android)
- Greenfield project — no existing code yet

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
| Independent crate (not toolkit-specific) | Fills ecosystem gap; benefits all Rust GUI toolkits equally; extra cost over gpui-specific is ~1-2 days | — Pending |
| Rgba with u8 + alpha from day one | All platform sources are 8-bit; alpha needed for shadow/opacity tokens; adding alpha later is breaking | — Pending |
| 36 semantic color roles | 6 is too few (still generic), 100+ too unwieldy; 36 covers what every toolkit can map meaningfully | — Pending |
| Option<T> for all fields | Platforms expose different subsets; enables layering; avoids fabricating values | — Pending |
| TOML format (not JSON/RON) | Human-readable, human-editable; can become de facto theme exchange standard | — Pending |
| Separate kde/portal features | KDE reader is sync (no async runtime); portal is async (ashpd/zbus); clean separation | — Pending |
| Named spacing scale (xxs-xxl) | Raw values too platform-specific; single multiplier too coarse; named scale maps to any toolkit's spacing system | — Pending |

---
*Last updated: 2026-03-07 after initialization*
