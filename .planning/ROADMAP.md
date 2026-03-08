# Roadmap: native-theme

## Overview

native-theme delivers a toolkit-agnostic Rust crate for unified OS theme data. The roadmap builds outward from a stable data model core: first the types and serde layer, then bundled presets for immediate usability, then platform readers (Linux KDE/GNOME, Windows) each as independent feature-gated modules, then cross-platform dispatch tying them together, then extended preset coverage (platform + community themes), and finally documentation with toolkit adapter examples. Each phase produces a verifiable, independently useful increment.

## Milestones

- ✅ **v0.1 MVP** — Phases 1-8 (shipped 2026-03-07)
- 🚧 **v0.2 Platform Coverage & Publishing** — Phases 9-15 (in progress)

## Phases

<details>
<summary>✅ v0.1 MVP (Phases 1-8) — SHIPPED 2026-03-07</summary>

- [x] Phase 1: Data Model Foundation (3/3 plans) — completed 2026-03-07
- [x] Phase 2: Core Presets (2/2 plans) — completed 2026-03-07
- [x] Phase 3: KDE Reader (2/2 plans) — completed 2026-03-07
- [x] Phase 4: GNOME Portal Reader (2/2 plans) — completed 2026-03-07
- [x] Phase 5: Windows Reader (1/1 plan) — completed 2026-03-07
- [x] Phase 6: Cross-Platform Dispatch (1/1 plan) — completed 2026-03-07
- [x] Phase 7: Extended Presets (2/2 plans) — completed 2026-03-07
- [x] Phase 8: Documentation (1/1 plan) — completed 2026-03-07

</details>

### 🚧 v0.2 Platform Coverage & Publishing (In Progress)

- [x] **Phase 9: Cargo Workspace** — Restructure repo into a Cargo workspace with core crate in subdirectory — completed 2026-03-08
- [x] **Phase 10: API Breaking Changes** — Flatten ThemeColors, move presets to NativeTheme methods, add geometry fields (completed 2026-03-08)
- [x] **Phase 11: Platform Readers** — macOS reader, Windows enhancements, Linux enhancements (completed 2026-03-08)
- [ ] **Phase 12: Widget Metrics** — Widget metrics data model and platform-specific population
- [ ] **Phase 13: CI Pipeline** — GitHub Actions with cross-platform matrix, semver-checks, linting
- [ ] **Phase 14: Toolkit Connectors** — gpui and iced connector crates with examples
- [ ] **Phase 15: Publishing Prep** — Metadata, licenses, changelog, doc examples, crates.io publish

## Phase Details

### Phase 9: Cargo Workspace
**Goal**: Repo restructured as a Cargo workspace so connector crates can be developed alongside the core crate
**Depends on**: Phase 8 (v0.1 complete)
**Requirements**: API-01
**Success Criteria** (what must be TRUE):
  1. Running `cargo build` from repo root builds the core crate via workspace
  2. Running `cargo test` from repo root runs all existing 140+ tests and they pass
  3. Core crate source lives in a `native-theme/` subdirectory with its own Cargo.toml
  4. A top-level Cargo.toml defines workspace members
**Plans**: 1 plan
Plans:
- [x] 09-01-PLAN.md — Restructure repo as Cargo virtual workspace with connector stubs

### Phase 10: API Breaking Changes
**Goal**: Public API refactored to its final v0.2 shape — flat colors, idiomatic methods, extended geometry — before any new features build on it
**Depends on**: Phase 9
**Requirements**: API-02, API-03, API-04, API-05, API-06, API-07, API-08
**Success Criteria** (what must be TRUE):
  1. `ThemeColors` has 36 direct `Option<Rgba>` fields with no nested sub-structs
  2. All 17 preset TOML files use flat `[light.colors]` / `[dark.colors]` tables and load correctly
  3. `NativeTheme::preset("adwaita")`, `NativeTheme::from_toml()`, `NativeTheme::from_file()`, `NativeTheme::list_presets()`, and `theme.to_toml()` work; old free functions are removed
  4. `ThemeGeometry` has `radius_lg` and `shadow` fields, and presets include values for them
  5. All existing tests pass against the new API (updated as needed)
**Plans**: 3 plans
Plans:
- [x] 10-01-PLAN.md — Flatten ThemeColors to 36 direct fields, migrate TOML presets and platform readers
- [x] 10-02-PLAN.md — Move preset functions to impl NativeTheme, remove old exports, update README
- [x] 10-03-PLAN.md — Add radius_lg and shadow to ThemeGeometry, update preset geometry

### Phase 11: Platform Readers
**Goal**: Full desktop platform coverage — macOS reader completes the 4th platform, Windows and Linux readers enhanced with richer data
**Depends on**: Phase 10
**Requirements**: PLAT-01, PLAT-02, PLAT-03, PLAT-04, PLAT-05, PLAT-06, PLAT-07, PLAT-08, PLAT-09, PLAT-10, PLAT-11, PLAT-12, PLAT-13
**Success Criteria** (what must be TRUE):
  1. `NativeTheme::from_macos()` returns a theme with semantic colors (P3-to-sRGB converted), fonts, and both light/dark variants resolved via NSAppearance
  2. `NativeTheme::from_system()` dispatches to `from_macos()` on macOS
  3. `NativeTheme::from_windows()` returns accent shades (AccentDark1-3, AccentLight1-3), system font, spacing, and DPI-aware geometry; capability checks prevent crashes on older Windows versions
  4. `NativeTheme::from_kde()` with portal overlay merges portal accent onto kdeglobals palette; GNOME reader populates font data; `from_linux()` provides a fallback that tries kdeglobals on non-KDE desktops
  5. D-Bus portal backend detection improves DE heuristic accuracy
**Plans**: 4 plans
Plans:
- [x] 11-01-PLAN.md — macOS reader with NSColor semantic colors, NSAppearance variants, NSFont, and from_system() dispatch
- [x] 11-02-PLAN.md — Windows reader enhancements: accent shades, system font, WinUI3 spacing, DPI-aware geometry
- [x] 11-03-PLAN.md — Linux enhancements: GNOME fonts, KDE+portal overlay, D-Bus backend detection, from_linux() fallback
- [x] 11-04-PLAN.md — Gap closure: wire detect_portal_backend into async dispatch, fix env var test races

### Phase 12: Widget Metrics
**Goal**: Per-widget sizing and spacing data available from all four platforms, enabling toolkit connectors to produce pixel-perfect native layouts
**Depends on**: Phase 11
**Requirements**: METRIC-01, METRIC-02, METRIC-03, METRIC-04, METRIC-05, METRIC-06, METRIC-07, METRIC-08
**Success Criteria** (what must be TRUE):
  1. `WidgetMetrics` struct exists with 12 per-widget sub-structs (Button, Checkbox, Input, Scrollbar, Slider, ProgressBar, Tab, MenuItem, Tooltip, ListItem, Toolbar, Splitter), all using `Option<f32>` fields and `#[non_exhaustive]`
  2. `ThemeVariant` has a `widget_metrics: Option<WidgetMetrics>` field accessible after reading any platform theme
  3. KDE reader populates metrics from breezemetrics.h constants; Windows reader populates via `GetSystemMetricsForDpi`; macOS reader populates from HIG defaults; GNOME reader populates from libadwaita values
  4. Preset TOML files include widget metrics data
**Plans**: 3 plans
Plans:
- [ ] 12-01-PLAN.md — WidgetMetrics data model with 12 sub-structs, ThemeVariant integration
- [ ] 12-02-PLAN.md — Platform reader metrics population (KDE, Windows, macOS, GNOME)
- [ ] 12-03-PLAN.md — Widget metrics added to all 17 preset TOML files

### Phase 13: CI Pipeline
**Goal**: Automated cross-platform testing catches regressions and API breakage on every push
**Depends on**: Phase 12
**Requirements**: CI-01, CI-02, CI-03, CI-04
**Success Criteria** (what must be TRUE):
  1. GitHub Actions workflow runs tests on Linux, Windows, and macOS runners
  2. Feature flag matrix tests `--no-default-features` and each reader feature independently (`kde`, `portal-tokio`, `windows`, `macos`)
  3. `cargo semver-checks` runs in CI and would catch removed or changed public API items
  4. `cargo clippy` and `cargo fmt --check` run in CI and enforce clean code
**Plans**: TBD

### Phase 14: Toolkit Connectors
**Goal**: Developers using gpui or iced can apply native-theme data to their apps with a single connector crate, including working examples
**Depends on**: Phase 12
**Requirements**: CONN-01, CONN-02, CONN-03, CONN-04, CONN-05, CONN-06, CONN-07, CONN-08, CONN-09
**Success Criteria** (what must be TRUE):
  1. `native-theme-gpui` crate maps ThemeColors to gpui-component's 108 ThemeColor fields (direct + derived shade generation), plus fonts, geometry, spacing, and widget metrics
  2. `native-theme-gpui` includes upstream PR proposal documents and an `examples/showcase.rs` widget gallery that renders with a native theme
  3. `native-theme-iced` crate maps ThemeColors to iced Palette + Extended palette, implements per-widget Catalog/Style for 8 core widgets, and maps geometry/spacing/widget metrics
  4. `native-theme-iced` includes an `examples/demo.rs` widget gallery that renders with a native theme
  5. Both connectors include a theme selector dropdown (presets + OS theme)
**Plans**: TBD

### Phase 15: Publishing Prep
**Goal**: Crate metadata, documentation, and licensing complete — `native-theme` and `native-theme-iced` published to crates.io
**Depends on**: Phase 13, Phase 14
**Requirements**: PUB-01, PUB-02, PUB-03, PUB-04, PUB-05, PUB-06, PUB-07, PUB-08
**Success Criteria** (what must be TRUE):
  1. `cargo publish --dry-run` succeeds for `native-theme` and `native-theme-iced` with all required metadata (rust-version, repository, homepage, keywords, categories, readme)
  2. LICENSE-MIT, LICENSE-APACHE, and LICENSE-0BSD files exist at repo root
  3. CHANGELOG.md covers all v0.2 changes in Keep a Changelog format; IMPLEMENTATION.md matches actual implementation; `docs/new-os-version-guide.md` exists
  4. `NativeTheme`, `Rgba`, and `ThemeVariant` have doc examples (`/// # Examples`) that compile
  5. `native-theme` and `native-theme-iced` are published to crates.io
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 9 -> 10 -> 11 -> 12 -> 13 -> 14 -> 15
(Phase 13 and Phase 14 can execute in parallel after Phase 12; Phase 15 depends on both.)

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Data Model Foundation | v0.1 | 3/3 | ✓ Complete | 2026-03-07 |
| 2. Core Presets | v0.1 | 2/2 | ✓ Complete | 2026-03-07 |
| 3. KDE Reader | v0.1 | 2/2 | ✓ Complete | 2026-03-07 |
| 4. GNOME Portal Reader | v0.1 | 2/2 | ✓ Complete | 2026-03-07 |
| 5. Windows Reader | v0.1 | 1/1 | ✓ Complete | 2026-03-07 |
| 6. Cross-Platform Dispatch | v0.1 | 1/1 | ✓ Complete | 2026-03-07 |
| 7. Extended Presets | v0.1 | 2/2 | ✓ Complete | 2026-03-07 |
| 8. Documentation | v0.1 | 1/1 | ✓ Complete | 2026-03-07 |
| 9. Cargo Workspace | v0.2 | 1/1 | ✓ Complete | 2026-03-08 |
| 10. API Breaking Changes | v0.2 | 3/3 | ✓ Complete | 2026-03-08 |
| 11. Platform Readers | 4/4 | Complete   | 2026-03-08 | 2026-03-08 |
| 12. Widget Metrics | 2/3 | In Progress|  | - |
| 13. CI Pipeline | v0.2 | 0/0 | Not started | - |
| 14. Toolkit Connectors | v0.2 | 0/0 | Not started | - |
| 15. Publishing Prep | v0.2 | 0/0 | Not started | - |
