# Roadmap: native-theme

## Overview

native-theme delivers a toolkit-agnostic Rust crate for unified OS theme data. The roadmap builds outward from a stable data model core: first the types and serde layer, then bundled presets for immediate usability, then platform readers (Linux KDE/GNOME, Windows) each as independent feature-gated modules, then cross-platform dispatch tying them together, then extended preset coverage (platform + community themes), and finally documentation with toolkit adapter examples. Each phase produces a verifiable, independently useful increment.

## Milestones

- ✅ **v0.1 MVP** — Phases 1-8 (shipped 2026-03-07)
- ✅ **v0.2 Platform Coverage & Publishing** — Phases 9-15 (shipped 2026-03-09)
- ✅ **v0.3 Icons** — Phases 16-21 (shipped 2026-03-09)
- ✅ **v0.3.3 Custom Icon Roles** — Phases 22-26 (shipped 2026-03-17)
- ✅ **v0.4.0 Animated Icons** — Phases 27-32 (shipped 2026-03-18)
- ✅ **v0.4.1 Release Prep** — Phases 33-43 (shipped 2026-03-21)
- **v0.5.0 Per-Widget Architecture & Resolution Pipeline** — Phases 44-48 (in progress)

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

<details>
<summary>v0.3 Icons (Phases 16-21) — SHIPPED 2026-03-09</summary>

- [x] Phase 16: Icon Data Model (2/2 plans) — completed 2026-03-09
- [x] Phase 17: Bundled SVG Icons (2/2 plans) — completed 2026-03-09
- [x] Phase 18: Linux Icon Loading (1/1 plan) — completed 2026-03-09
- [x] Phase 19: macOS Icon Loading (1/1 plan) — completed 2026-03-09
- [x] Phase 20: Windows Icon Loading (1/1 plan) — completed 2026-03-09
- [x] Phase 21: Integration and Connectors (3/3 plans) — completed 2026-03-09

</details>

<details>
<summary>v0.3.3 Custom Icon Roles (Phases 22-26) — SHIPPED 2026-03-17</summary>

- [x] Phase 22: Core Trait and Loading Functions (2/2 plans) — completed 2026-03-15
- [x] Phase 23: Build Crate and Code Generation (5/5 plans) — completed 2026-03-16
- [x] Phase 24: Linux DE Audit and Freedesktop DE-Aware Mapping (2/2 plans) — completed 2026-03-16
- [x] Phase 25: Connector Integration (1/1 plan) — completed 2026-03-16
- [x] Phase 25.1: Icon Gaps and Fallback Removal (2/2 plans) — completed 2026-03-17
- [x] Phase 26: Documentation and Release (2/2 plans) — completed 2026-03-17

</details>

<details>
<summary>v0.4.0 Animated Icons (Phases 27-32) — SHIPPED 2026-03-18</summary>

- [x] Phase 27: Animation Data Model and Breaking Changes (2/2 plans) — completed 2026-03-18
- [x] Phase 28: Bundled SVG Spinner Frames (2/2 plans) — completed 2026-03-18
- [x] Phase 29: Freedesktop Sprite Sheet Parser (1/1 plan) — completed 2026-03-18
- [x] Phase 30: Reduced Motion Accessibility (1/1 plan) — completed 2026-03-18
- [x] Phase 31: Connector Integration (1/1 plan) — completed 2026-03-18
- [x] Phase 32: Documentation and Release (1/1 plan) — completed 2026-03-18

</details>

<details>
<summary>v0.4.1 Release Prep (Phases 33-43) — SHIPPED 2026-03-21</summary>

- [x] Phase 33: Quick Fixes and Version Consistency (1/1 plan) — completed 2026-03-19
- [x] Phase 34: Animated Icon Documentation (1/1 plan) — completed 2026-03-19
- [x] Phase 35: Animated Icon Showcase Examples (2/2 plans) — completed 2026-03-19
- [x] Phase 36: Screenshot and GIF Generation (3/3 plans) — completed 2026-03-20
- [x] Phase 37: Community Files and GitHub Templates (2/2 plans) — completed 2026-03-20
- [x] Phase 38: CI, Smoke Tests, and Release (3/3 plans) — completed 2026-03-20
- [x] Phase 39: Code Quality and Housekeeping (2/2 plans) — completed 2026-03-20
- [x] Phase 40: Iced Theme Preset Screenshots and CI (2/2 plans) — completed 2026-03-20
- [x] Phase 41: gpui Theme Preset Screenshots (2/2 plans) — completed 2026-03-21
- [x] Phase 42: Theme-Switching GIF and Core README Images (1/1 plan) — completed 2026-03-21
- [x] Phase 43: Self-Capture Screenshots with Window Decorations (3/3 plans) — completed 2026-03-21

</details>

### v0.5.0 Per-Widget Architecture & Resolution Pipeline (Phases 44-48)

- [x] **Phase 44: Per-Widget Data Model and Preset Migration** - Restructure ThemeVariant to per-widget structs with ThemeDefaults, FontSpec, TextScale, and atomically rewrite all 17 preset TOMLs (completed 2026-03-27)
- [x] **Phase 45: Resolution Engine** - Implement resolve() inheritance, ResolvedTheme with validate(), and update cross-platform presets for new structure (completed 2026-03-27)
- [x] **Phase 46: OS Reader Extensions** - Extend all four platform readers (macOS, Windows, KDE, GNOME) with per-widget fields, text scale, fonts, and accessibility (completed 2026-03-27)
- [ ] **Phase 47: OS-First Pipeline** - Wire from_system() to run OS reader, platform TOML overlay, resolve(), app TOML overlay, second resolve(), and validate()
- [ ] **Phase 48: Connector Migration** - Update gpui and iced connectors to accept &ResolvedTheme and update showcase examples

## Phase Details

Phase details for completed milestones are archived in `.planning/milestones/`.

### Phase 44: Per-Widget Data Model and Preset Migration
**Goal**: Consumers can define and serialize per-widget theme properties using 24 widget structs, ThemeDefaults, FontSpec, TextScale, IconSizes, and DialogButtonOrder -- and all 17 bundled presets load/save correctly in the new format
**Depends on**: Phase 43
**Requirements**: MODEL-01, MODEL-02, MODEL-03, MODEL-04, MODEL-05, MODEL-06, MODEL-07, MODEL-08, MODEL-09, PRESET-01, PRESET-02
**Success Criteria** (what must be TRUE):
  1. A ThemeVariant can be constructed with per-widget structs (e.g., `theme.light.button.background`, `theme.light.menu.font.family`) and all 24 widget types are addressable
  2. ThemeDefaults holds shared base properties (colors, font, mono_font, spacing, icon_sizes, accessibility flags) and every widget struct can reference them conceptually
  3. All 17 preset TOML files round-trip through serde (deserialize then serialize produces equivalent output) using the new per-widget structure
  4. Platform preset TOMLs contain only design constants (no OS-readable values), while cross-platform presets provide all non-derived color and geometry fields
  5. The define_widget_pair! macro generates both Option and Resolved struct variants from a single definition, and impl_merge! supports nested per-widget struct merging
**Plans:** 3/3 plans complete

Plans:
- [x] 44-01-PLAN.md — Foundation types (FontSpec, TextScale, IconSizes, DialogButtonOrder) + macros (define_widget_pair!, impl_merge! extension)
- [x] 44-02-PLAN.md — ThemeDefaults struct + all 25 per-widget XxxTheme structs via define_widget_pair!
- [x] 44-03-PLAN.md — Atomic ThemeVariant restructure + rewrite all 17 preset TOMLs + test updates

### Phase 45: Resolution Engine
**Goal**: Any sparse ThemeVariant (from an OS reader or partial TOML) can be resolved into a complete ResolvedTheme where every field is guaranteed populated, with clear error reporting for unresolvable gaps
**Depends on**: Phase 44
**Requirements**: RESOLVE-01, RESOLVE-02, RESOLVE-03, RESOLVE-04, RESOLVE-05, RESOLVE-06, PRESET-03
**Success Criteria** (what must be TRUE):
  1. Calling resolve() on a ThemeVariant with only ThemeDefaults.accent set produces a variant where all accent-derived fields (primary_bg, checked_bg, slider.fill, progress_bar.fill, switch.checked_bg) are populated
  2. FontSpec sub-field inheritance works: setting only `menu.font.size` still inherits `family` and `weight` from `defaults.font`
  3. validate() converts a resolved ThemeVariant into a ResolvedTheme with zero Option fields, or returns a ThemeResolutionError listing every missing field path
  4. Every one of the 17 bundled presets (both light and dark variants) passes the full resolve() then validate() pipeline without error
  5. TextScaleEntry inheritance produces correct sizes: caption/section_heading/dialog_title/display entries inherit from defaults.font when their own size/weight are None
**Plans:** 3/3 plans complete

Plans:
- [x] 45-01-PLAN.md — ResolvedDefaults, ResolvedTextScale, ResolvedTheme type system + ThemeResolutionError
- [x] 45-02-PLAN.md — resolve() 4-phase inheritance engine (~90 rules) + validate() function
- [x] 45-03-PLAN.md — Enrich all 17 presets with non-derived fields + integration tests

### Phase 46: OS Reader Extensions
**Goal**: All four platform readers (macOS, Windows, KDE, GNOME) populate per-widget fields, text scale, per-widget fonts, and accessibility flags in the new ThemeVariant structure
**Depends on**: Phase 45
**Requirements**: MACOS-01, MACOS-02, MACOS-03, MACOS-04, MACOS-05, WIN-01, WIN-02, WIN-03, WIN-04, WIN-05, KDE-01, KDE-02, KDE-03, KDE-04, KDE-05, KDE-06, GNOME-01, GNOME-02, GNOME-03, GNOME-04, GNOME-05
**Success Criteria** (what must be TRUE):
  1. On KDE, from_kde() populates title bar colors from [WM], per-widget fonts (menuFont, toolBarFont) with correct Qt5/Qt6 weight conversion, text scale from Kirigami multipliers, icon sizes from index.theme, and reduce_motion from AnimationDurationFactor
  2. On GNOME, from_gnome() populates fonts (font-name, monospace-font-name, titlebar-font), text scale from CSS percentage multipliers, accessibility flags (text-scaling-factor, enable-animations, overlay-scrolling), and icon_set from icon-theme gsetting
  3. On macOS, from_macos() populates text_scale entries from NSFont.TextStyle, per-widget fonts (menu, tooltip, title bar) with weight extraction, additional NSColor values (placeholder, caret, selection_inactive), scrollbar overlay mode, and all accessibility flags
  4. On Windows, from_windows() populates per-widget fonts from NONCLIENTMETRICSW, title bar colors from DwmGetColorizationColor, widget colors from GetSysColor (BTNFACE, MENU, HIGHLIGHT, etc.), icon sizes from SM_CXSMICON/SM_CXICON, and accessibility from UISettings.TextScaleFactor and SPI_GETHIGHCONTRAST
  5. The output of every OS reader, when passed through resolve() then validate(), produces a valid ResolvedTheme without error
**Plans:** 6/6 plans complete

Plans:
- [x] 46-01-PLAN.md — KDE reader rewrite: per-widget ThemeVariant + title bar + fonts + text_scale + accessibility + icons
- [x] 46-02-PLAN.md — GNOME reader rewrite: sparse ThemeVariant + fonts + text_scale + accessibility + icon_set
- [x] 46-03-PLAN.md — macOS reader extension: per-widget fonts + text_scale + NSColor extras + scrollbar + accessibility
- [x] 46-04-PLAN.md — Windows reader rewrite: per-widget ThemeVariant + fonts + DWM + GetSysColor + accessibility + icons
- [x] 46-05-PLAN.md — Gap closure: KDE icon sizes from index.theme parsing
- [x] 46-06-PLAN.md — Gap closure: resolve/validate integration tests for macOS, Windows, GNOME

### Phase 47: OS-First Pipeline
**Goal**: from_system() runs the complete OS-first pipeline producing a guaranteed-complete ResolvedTheme, and app developers can apply TOML overrides that propagate through a second resolve() pass
**Depends on**: Phase 46
**Requirements**: PIPE-01, PIPE-02, PIPE-03
**Success Criteria** (what must be TRUE):
  1. from_system() returns a ResolvedTheme (not a ThemeVariant) after running: OS reader -> platform TOML overlay -> resolve() -> validate()
  2. Platform-to-preset mapping works correctly: macOS uses macos-sonoma, Windows uses windows-11, KDE uses kde-breeze, GNOME uses adwaita as the platform TOML overlay
  3. App TOML overlay with a second resolve() pass works: changing accent in an app override causes all accent-derived fields to re-propagate through resolve()
**Plans**: TBD

### Phase 48: Connector Migration
**Goal**: Both toolkit connectors (gpui and iced) consume &ResolvedTheme with zero Option handling, and showcase examples demonstrate the new API
**Depends on**: Phase 47
**Requirements**: CONN-01, CONN-02, CONN-03
**Success Criteria** (what must be TRUE):
  1. The gpui connector's theme conversion function accepts &ResolvedTheme and contains zero unwrap_or() or .unwrap_or_default() calls for theme fields
  2. The iced connector's theme conversion function accepts &ResolvedTheme and contains zero unwrap_or() or .unwrap_or_default() calls for theme fields
  3. Both showcase examples (gpui and iced) compile and run using the new ResolvedTheme-based API with visible per-widget theming
**Plans**: TBD
**UI hint**: yes

## Progress

**Execution Order:**
Phases execute in numeric order: 44 -> 45 -> 46 -> 47 -> 48

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1-8 | v0.1 | 14/14 | Complete | 2026-03-07 |
| 9-15 | v0.2 | 20/20 | Complete | 2026-03-09 |
| 16-21 | v0.3 | 10/10 | Complete | 2026-03-09 |
| 22-26 | v0.3.3 | 14/14 | Complete | 2026-03-17 |
| 27-32 | v0.4.0 | 8/8 | Complete | 2026-03-18 |
| 33-43 | v0.4.1 | 22/22 | Complete | 2026-03-21 |
| 44. Per-Widget Data Model and Preset Migration | v0.5.0 | 3/3 | Complete   | 2026-03-27 |
| 45. Resolution Engine | v0.5.0 | 3/3 | Complete   | 2026-03-27 |
| 46. OS Reader Extensions | v0.5.0 | 6/6 | Complete   | 2026-03-27 |
| 47. OS-First Pipeline | v0.5.0 | 0/? | Not started | - |
| 48. Connector Migration | v0.5.0 | 0/? | Not started | - |
