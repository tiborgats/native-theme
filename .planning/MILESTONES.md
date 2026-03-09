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


## v0.3 Icons (Shipped: 2026-03-09)

**Phases completed:** 6 phases (16–21), 10 plans
**Timeline:** 3 days (2026-03-07 → 2026-03-09)
**Stats:** 139 files changed, +11,810 lines

**Key accomplishments:**
- IconRole enum (42 semantic variants across 7 categories), IconData (SVG/RGBA), IconSet (5 platform sets) with icon_name() mapping (210 match arms)
- 76 bundled SVG icons (38 Material Symbols + 38 Lucide) as compile-time cross-platform fallbacks, feature-gated
- Linux freedesktop icon theme loading with two-pass Adwaita-compatible lookup and hicolor→Material fallback
- macOS SF Symbols rasterization via CGBitmapContext with straight alpha unpremultiply pass
- Windows icon loading: SHGetStockIconInfo stock icons + Segoe Fluent/MDL2 font glyph rendering with BGRA-to-RGBA conversion
- load_icon() dispatch API with platform→bundled→None fallback chain, rasterize_svg() via resvg
- gpui connector: 30-role IconName shortcut, inline BMP V4 encoder for RGBA, showcase icon set selector
- iced connector: to_image_handle() / to_svg_handle() type-routing helpers

---

