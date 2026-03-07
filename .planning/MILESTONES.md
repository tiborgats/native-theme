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

