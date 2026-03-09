# Milestones

## v0.1 MVP (Shipped: 2026-03-07)

**Phases completed:** 8 phases, 14 plans, 0 tasks

**Key accomplishments:**
- Toolkit-agnostic theme data model with 36 semantic color roles, fonts, geometry, spacing — all types Send+Sync with recursive merge()
- 17 bundled TOML presets: core (default, KDE Breeze, Adwaita), platform (Windows 11, macOS Sonoma, Material, iOS), and 10 community themes
- Three platform readers: sync KDE (configparser), async GNOME (ashpd portal), sync Windows (UISettings + GetSystemMetrics) — all feature-gated
- Cross-platform dispatch via from_system() with auto-detection and preset fallback
- Complete documentation with compile-tested README, adapter examples for egui/iced/slint
- 140+ tests (unit, integration, doctests) with zero failures

---

## v0.2 Platform Coverage & Publishing (Shipped: 2026-03-09)

**Phases completed:** 7 phases (9–15), 20 plans

**Key accomplishments:**
- Cargo workspace restructuring with native-theme-gpui and native-theme-iced connector sub-crates
- API breaking changes: flat ThemeColors (36 direct fields), NativeTheme associated methods, ThemeGeometry extensions (radius_lg, shadow)
- macOS reader via objc2-app-kit with dual-variant (light+dark) support
- Enhanced Windows reader: accent shades, system fonts, spacing, DPI-aware geometry, primary_foreground derivation
- Enhanced Linux readers: KDE+portal overlay, D-Bus backend detection, GNOME font reading, from_linux() fallback, from_system_async()
- Widget metrics data model (12 per-widget sub-structs) with platform-specific sources (Breeze, Adwaita, WinUI3, macOS HIG)
- CI pipeline: GitHub Actions with feature flag matrix, semver-checks, clippy/fmt
- native-theme-gpui connector: 108-field ThemeColor mapping, font/geometry/widget-metrics, example app with theme switcher
- native-theme-iced connector: palette/font/style/widget-metrics mapping
- Publishing prep: workspace metadata, licenses, changelog, documentation updates

**Note:** crates.io publishing (15-04) deferred — will publish when ready for broader adoption.

---

