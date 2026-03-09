# Roadmap: native-theme

## Overview

native-theme delivers a toolkit-agnostic Rust crate for unified OS theme data. The roadmap builds outward from a stable data model core: first the types and serde layer, then bundled presets for immediate usability, then platform readers (Linux KDE/GNOME, Windows) each as independent feature-gated modules, then cross-platform dispatch tying them together, then extended preset coverage (platform + community themes), and finally documentation with toolkit adapter examples. Each phase produces a verifiable, independently useful increment.

## Milestones

- ✅ **v0.1 MVP** — Phases 1-8 (shipped 2026-03-07)
- ✅ **v0.2 Platform Coverage & Publishing** — Phases 9-15 (shipped 2026-03-09)
- 🚧 **v0.3 Icons** — Phases 16-21 (in progress)

## Phases

<details>
<summary>v0.1 MVP (Phases 1-8) — SHIPPED 2026-03-07</summary>

- [x] Phase 1: Data Model Foundation (3/3 plans) — completed 2026-03-07
- [x] Phase 2: Core Presets (2/2 plans) — completed 2026-03-07
- [x] Phase 3: KDE Reader (2/2 plans) — completed 2026-03-07
- [x] Phase 4: GNOME Portal Reader (2/2 plans) — completed 2026-03-07
- [x] Phase 5: Windows Reader (1/1 plan) — completed 2026-03-07
- [x] Phase 6: Cross-Platform Dispatch (1/1 plan) — completed 2026-03-07
- [x] Phase 7: Extended Presets (2/2 plans) — completed 2026-03-07
- [x] Phase 8: Documentation (1/1 plan) — completed 2026-03-07

</details>

<details>
<summary>v0.2 Platform Coverage & Publishing (Phases 9-15) — SHIPPED 2026-03-09</summary>

- [x] Phase 9: Cargo Workspace (1/1 plan) — completed 2026-03-08
- [x] Phase 10: API Breaking Changes (3/3 plans) — completed 2026-03-08
- [x] Phase 11: Platform Readers (4/4 plans) — completed 2026-03-08
- [x] Phase 12: Widget Metrics (3/3 plans) — completed 2026-03-08
- [x] Phase 13: CI Pipeline (1/1 plan) — completed 2026-03-08
- [x] Phase 14: Toolkit Connectors (5/5 plans) — completed 2026-03-09
- [x] Phase 15: Publishing Prep (3/3 plans) — completed 2026-03-09

</details>

### 🚧 v0.3 Icons (In Progress)

**Milestone Goal:** Platform-native icon loading — semantic icon roles mapped to OS-native icon systems (SF Symbols, Segoe Fluent, freedesktop) with bundled cross-platform fallbacks (Material, Lucide).

- [x] **Phase 16: Icon Data Model** — IconRole enum, IconData type, icon name mapping, ThemeVariant integration
- [x] **Phase 17: Bundled SVG Icons** — Material Symbols and Lucide SVGs as compile-time fallback icon sets (completed 2026-03-09)
- [x] **Phase 18: Linux Icon Loading** — Freedesktop icon theme lookup via freedesktop-icons crate (completed 2026-03-09)
- [x] **Phase 19: macOS Icon Loading** — SF Symbols via NSImage rasterization to RGBA (completed 2026-03-09)
- [x] **Phase 20: Windows Icon Loading** — SHGetStockIconInfo stock icons and Segoe Fluent font glyphs (completed 2026-03-09)
- [ ] **Phase 21: Integration and Connectors** — load_icon() dispatch, SVG rasterization, gpui/iced connector updates

## Phase Details

### Phase 16: Icon Data Model
**Goal**: Developers can define semantic icon roles and look up platform-specific icon identifiers without any platform dependencies
**Depends on**: Phase 15 (v0.2 complete)
**Requirements**: ICON-01, ICON-02, ICON-03, ICON-04, ICON-05
**Success Criteria** (what must be TRUE):
  1. `IconRole::DialogError`, `IconRole::WindowClose`, `IconRole::ActionCopy`, and all 42 variants are accessible and exhaustively matchable
  2. `IconData::Svg(bytes)` and `IconData::Rgba { width, height, data }` can be constructed and pattern-matched by consuming code
  3. `icon_name(IconSet::SfSymbols, IconRole::ActionCopy)` returns `"doc.on.doc"` (and analogous lookups for all 5 icon sets return correct platform identifiers)
  4. `system_icon_set()` returns `IconSet::SfSymbols` on macOS, `IconSet::SegoeIcons` on Windows, `IconSet::Freedesktop` on Linux
  5. Loading a preset TOML with `icon_theme = "material"` populates `theme.light.icon_theme` as `Some("material")`
**Plans:** 2/2 plans executed (phase complete)
Plans:
- [x] 16-01-PLAN.md — IconRole, IconData, IconSet type definitions (TDD)
- [x] 16-02-PLAN.md — icon_name() mapping, system_icon_set(), ThemeVariant integration, preset TOML updates

### Phase 17: Bundled SVG Icons
**Goal**: Any platform can render all 42 icon roles using bundled SVG fallbacks without network access or OS-specific APIs
**Depends on**: Phase 16
**Requirements**: BNDL-01, BNDL-02
**Success Criteria** (what must be TRUE):
  1. With feature `material-icons` enabled, every `IconRole` variant resolves to valid SVG bytes via the bundled Material Symbols set
  2. With feature `lucide-icons` enabled, every `IconRole` variant resolves to valid SVG bytes via the bundled Lucide set
  3. Without any icon feature flags enabled, attempting to load a bundled icon returns `None` (no compile-time bloat when icons not needed)
  4. Total binary size contribution of each bundled set stays under 200KB (Material) and 100KB (Lucide)
**Plans:** 2/2 plans executed (phase complete)
Plans:
- [x] 17-01-PLAN.md — Download Material Symbols and Lucide SVG assets + licenses
- [x] 17-02-PLAN.md — bundled_icon_svg() module, Cargo features, tests

### Phase 18: Linux Icon Loading
**Goal**: Linux users get icons from their active desktop theme (Adwaita, Breeze, Papirus, etc.) following the freedesktop spec
**Depends on**: Phase 17
**Requirements**: PLAT-04
**Success Criteria** (what must be TRUE):
  1. With feature `system-icons` enabled on Linux, `load_icon(IconRole::DialogError, "freedesktop")` returns SVG bytes from the active icon theme
  2. When a role has no matching icon in the current theme, the loader falls back to hicolor, then to the bundled Material SVGs
  3. The loader respects `XDG_DATA_DIRS` and works with Adwaita, Breeze, and hicolor-only environments
**Plans:** 1/1 plans complete
Plans:
- [x] 18-01-PLAN.md — freedesktop.rs module, system-icons feature, two-pass lookup with bundled fallback

### Phase 19: macOS Icon Loading
**Goal**: macOS users get native SF Symbols icons rasterized to RGBA pixels at the requested size
**Depends on**: Phase 17
**Requirements**: PLAT-01
**Success Criteria** (what must be TRUE):
  1. With feature `system-icons` enabled on macOS, `load_icon(IconRole::ActionCopy, "sf-symbols")` returns `IconData::Rgba` with correct pixel dimensions
  2. Rasterized icons have correct alpha (straight, not premultiplied) and produce visually correct output at both 1x and 2x (Retina) scale
  3. When an SF Symbols icon is unavailable (older macOS or missing symbol), the loader falls back to bundled SVGs
**Plans:** 1/1 plans complete
Plans:
- [x] 19-01-PLAN.md — sficons.rs module, objc2-core-graphics dependency, CGBitmapContext rasterization with straight alpha

### Phase 20: Windows Icon Loading
**Goal**: Windows users get native stock icons and Segoe Fluent Icons font glyphs as RGBA pixels
**Depends on**: Phase 17
**Requirements**: PLAT-02, PLAT-03
**Success Criteria** (what must be TRUE):
  1. With feature `system-icons` enabled on Windows, stock icon roles (e.g., `IconRole::FileDocument`, `IconRole::DialogWarning`) return RGBA pixels via `SHGetStockIconInfo`
  2. Action/navigation/window roles (e.g., `IconRole::ActionCopy`, `IconRole::WindowClose`) return RGBA pixels rendered from the Segoe Fluent Icons font
  3. RGBA output has correct byte order (not BGRA) and straight alpha (not premultiplied)
  4. When Segoe Fluent font is not present (some Windows 10 installs), the loader falls back to bundled SVGs
**Plans:** 1/1 plans executed (phase complete)
Plans:
- [x] 20-01-PLAN.md — winicons.rs module, SHGetStockIconInfo + GetGlyphOutlineW pipelines, BGRA-to-RGBA conversion, font fallback chain

### Phase 21: Integration and Connectors
**Goal**: The full icon pipeline works end-to-end: load_icon() dispatches to the right loader, connectors convert IconData to toolkit image types, and the gpui example showcases icons
**Depends on**: Phase 18, Phase 19, Phase 20
**Requirements**: INTG-01, INTG-02, INTG-03, INTG-04, INTG-05
**Success Criteria** (what must be TRUE):
  1. `load_icon(role, icon_theme)` dispatches to the correct platform loader based on the icon_theme string and falls back through the chain (platform -> bundled Material -> None)
  2. With feature `svg-rasterize`, `rasterize_svg(svg_bytes, size)` converts SVG data to `IconData::Rgba` using resvg
  3. `native-theme-gpui` converts `IconData` to a gpui-compatible image and maps `IconRole` to gpui-component `IconName` for Lucide icons (zero I/O shortcut for 27+ roles)
  4. `native-theme-iced` converts `IconData` to `iced::widget::image::Handle`
  5. The gpui example app displays icons with an icon set selector dropdown
**Plans:** 1/3 plans executed
Plans:
- [ ] 21-01-PLAN.md — load_icon() dispatch + rasterize_svg() module in core crate
- [ ] 21-02-PLAN.md — gpui connector icons.rs + showcase example update
- [ ] 21-03-PLAN.md — iced connector icons.rs conversion helpers

## Progress

**Execution Order:**
Phases 16 -> 17 -> 18/19/20 (parallel) -> 21

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Data Model Foundation | v0.1 | 3/3 | Complete | 2026-03-07 |
| 2. Core Presets | v0.1 | 2/2 | Complete | 2026-03-07 |
| 3. KDE Reader | v0.1 | 2/2 | Complete | 2026-03-07 |
| 4. GNOME Portal Reader | v0.1 | 2/2 | Complete | 2026-03-07 |
| 5. Windows Reader | v0.1 | 1/1 | Complete | 2026-03-07 |
| 6. Cross-Platform Dispatch | v0.1 | 1/1 | Complete | 2026-03-07 |
| 7. Extended Presets | v0.1 | 2/2 | Complete | 2026-03-07 |
| 8. Documentation | v0.1 | 1/1 | Complete | 2026-03-07 |
| 9. Cargo Workspace | v0.2 | 1/1 | Complete | 2026-03-08 |
| 10. API Breaking Changes | v0.2 | 3/3 | Complete | 2026-03-08 |
| 11. Platform Readers | v0.2 | 4/4 | Complete | 2026-03-08 |
| 12. Widget Metrics | v0.2 | 3/3 | Complete | 2026-03-08 |
| 13. CI Pipeline | v0.2 | 1/1 | Complete | 2026-03-08 |
| 14. Toolkit Connectors | v0.2 | 5/5 | Complete | 2026-03-09 |
| 15. Publishing Prep | v0.2 | 3/3 | Complete | 2026-03-09 |
| 16. Icon Data Model | v0.3 | 2/2 | Complete | 2026-03-09 |
| 17. Bundled SVG Icons | v0.3 | 2/2 | Complete | 2026-03-09 |
| 18. Linux Icon Loading | v0.3 | 1/1 | Complete | 2026-03-09 |
| 19. macOS Icon Loading | v0.3 | 1/1 | Complete | 2026-03-09 |
| 20. Windows Icon Loading | v0.3 | 1/1 | Complete | 2026-03-09 |
| 21. Integration and Connectors | 1/3 | In Progress|  | - |
