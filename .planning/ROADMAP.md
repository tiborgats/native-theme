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
- ✅ **v0.5.0 Per-Widget Architecture & Resolution Pipeline** — Phases 44-48 (shipped 2026-03-29)
- ✅ **v0.5.5 Schema Overhaul & Quality** — Phases 49-60 (shipped 2026-04-09)
- 🚧 **v0.5.6 Internal Quality & Runtime Watching** — Phases 61-67 (in progress)

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

<details>
<summary>v0.5.0 Per-Widget Architecture & Resolution Pipeline (Phases 44-48) — SHIPPED 2026-03-29</summary>

- [x] Phase 44: Per-Widget Data Model and Preset Migration (3/3 plans) — completed 2026-03-27
- [x] Phase 45: Resolution Engine (3/3 plans) — completed 2026-03-27
- [x] Phase 46: OS Reader Extensions (6/6 plans) — completed 2026-03-27
- [x] Phase 47: OS-First Pipeline (2/2 plans) — completed 2026-03-27
- [x] Phase 48: Connector Migration (3/3 plans) — completed 2026-03-28

</details>

<details>
<summary>v0.5.5 Schema Overhaul & Quality (Phases 49-60) — SHIPPED 2026-04-09</summary>

- [x] Phase 49: Additive Type Definitions (3/3 plans) — completed 2026-04-06
- [x] Phase 50: Atomic Schema Commit (4/4 plans) — completed 2026-04-07
- [x] Phase 51: Resolution Engine Overhaul (5/5 plans) — completed 2026-04-07
- [x] Phase 52: Interactive State Colors (2/2 plans) — completed 2026-04-07
- [x] Phase 53: Preset Completeness (5/5 plans) — completed 2026-04-07
- [x] Phase 54: Connector Migration (3/3 plans) — completed 2026-04-07
- [x] Phase 55: Correctness, Safety, and CI (3/3 plans) — completed 2026-04-07
- [x] Phase 56: Testing (2/2 plans) — completed 2026-04-07
- [x] Phase 57: Verification and Documentation (3/3 plans) — completed 2026-04-07
- [x] Phase 58: Font pt/px DPI Conversion Fix (3/3 plans) — completed 2026-04-08
- [x] Phase 59: FontSize Compile-Time Unit Safety (3/3 plans) — completed 2026-04-08
- [x] Phase 60: TOML Key Unit Suffixes (5/5 plans) — completed 2026-04-08

</details>

### v0.5.6 Internal Quality & Runtime Watching (Phases 61-67)

- [x] **Phase 61: lib.rs Module Split** - Extract detect, pipeline, icons, and platform modules from the 2,767-line lib.rs monolith (completed 2026-04-09)
- [x] **Phase 62: Validate Codegen** - Extend define_widget_pair! to generate validate extraction, reducing validate.rs from ~2,196 to ~500 lines (completed 2026-04-09)
- [x] **Phase 63: KDE Reader Fixture Tests** - Separate KDE parsing from I/O and add fixture-based tests for all edge cases (completed 2026-04-09)
- [ ] **Phase 64: Cross-Platform Reader Test Separation** - Separate GNOME, Windows, and macOS reader parsing from OS dependencies
- [ ] **Phase 65: ThemeWatcher Core API** - Define ThemeWatcher, ThemeChanged, on_theme_change() public API with watch feature flag
- [ ] **Phase 66: Linux Watchers** - KDE inotify and GNOME portal watchers with debounce
- [ ] **Phase 67: macOS and Windows Watchers** - macOS KVO and Windows UISettings event watchers

## Phase Details

Phase details for milestones v0.1 through v0.5.5 are archived in `.planning/milestones/`.

### Phase 61: lib.rs Module Split
**Goal**: lib.rs is a clean ~250-line root module with impl_merge! macro, module declarations, and re-exports -- all detection, pipeline, icon dispatch, and platform code lives in focused modules
**Depends on**: Nothing (first phase of v0.5.6)
**Requirements**: STRUCT-01
**Success Criteria** (what must be TRUE):
  1. lib.rs is under 300 lines containing only impl_merge! macro, module declarations, re-exports, and the SystemTheme struct
  2. detect.rs exists with system_is_dark(), detect_is_dark(), prefers_reduced_motion(), all OnceLock caches, gsettings helpers, xrandr/DPI detection
  3. pipeline.rs exists with run_pipeline(), from_linux(), from_system_inner(), platform_preset_name() and all from_system() orchestration
  4. icons.rs exists with load_icon(), load_icon_from_theme(), and icon dispatch logic
  5. All existing tests pass unchanged -- zero behavior change, purely mechanical extraction
**Plans:** 2/2 plans complete
Plans:
- [x] 61-01-PLAN.md — Extract detect.rs and test_util.rs, update cross-module callers
- [x] 61-02-PLAN.md — Extract pipeline.rs and icons.rs, trim lib.rs to root module

### Phase 62: Validate Codegen
**Goal**: define_widget_pair! generates per-widget validate extraction methods via ValidateNested trait dispatch, eliminating ~1,600 lines of repetitive boilerplate from validate.rs
**Depends on**: Phase 61
**Requirements**: STRUCT-02, STRUCT-03, STRUCT-04
**Success Criteria** (what must be TRUE):
  1. ValidateNested trait exists with implementations for FontSpec and BorderSpec, enabling uniform type dispatch in macro-generated code
  2. define_widget_pair! emits a validate_widget() method for each widget pair that extracts Option fields into Resolved fields with range checks
  3. ThemeDefaults extraction remains hand-written in validate.rs (special DPI, text_scale, icon_sizes handling not suited to codegen)
  4. validate.rs is under 500 lines total (range checks, construction, defaults extraction, error types)
  5. All existing tests pass -- cargo expand on ButtonTheme confirms correct generated code before applying to all 25 widgets
**Plans:** 3/3 plans complete
Plans:
- [x] 62-01-PLAN.md — ValidateNested trait and macro codegen prototype
- [x] 62-02-PLAN.md — Validate widget extraction rollout for all 25 widgets
- [x] 62-03-PLAN.md — Gap closure: extract helpers and per-widget range checks to hit <500 lines

### Phase 63: KDE Reader Fixture Tests
**Goal**: KDE reader parsing is fully testable without a running KDE desktop, with fixture files covering all known edge cases
**Depends on**: Nothing (independent of Phases 61-62)
**Requirements**: TEST-01, TEST-02
**Success Criteria** (what must be TRUE):
  1. A pure parse_kdeglobals(content: &str) function (or equivalent from_kde_content) exists that accepts raw INI text and returns a ThemeVariant without any filesystem or desktop environment access
  2. Fixture .ini files exist for: Breeze light, Breeze dark, custom accent color, minimal config (only required groups), missing groups, malformed values, and high DPI configuration
  3. Each fixture has at least one test asserting specific field values in the parsed ThemeVariant output
  4. detect_font_dpi logic is separated into a pure INI-parsing function and an I/O function, with the pure part testable via fixture data
**Plans:** 2/2 plans complete
Plans:
- [x] 63-01-PLAN.md — Extract pure from_kde_content_pure function and create 7 fixture .ini files
- [x] 63-02-PLAN.md — Integration tests for all fixture scenarios

### Phase 64: Cross-Platform Reader Test Separation
**Goal**: GNOME, Windows, and macOS reader parsing logic is separated from OS-specific API calls, making parsing testable on any platform
**Depends on**: Nothing (independent of Phases 61-63)
**Requirements**: TEST-03, TEST-04, TEST-05
**Success Criteria** (what must be TRUE):
  1. GNOME reader has a build_gnome_spec() function (or equivalent) that accepts primitive types (not ashpd types) and returns a ThemeVariant, testable without D-Bus or portal access
  2. Windows reader has a build_windows_spec() function (or equivalent) that accepts primitive types (not windows crate types) and returns a ThemeVariant, testable on Linux and macOS
  3. macOS reader has a build_macos_spec() function (or equivalent) that accepts primitive types (not objc2 types) and returns a ThemeVariant, testable on Linux and Windows
  4. At least one test per reader exercises the pure parsing function with representative input values
**Plans:** 1 plan
Plans:
- [ ] 64-01-PLAN.md — GNOME pure function extraction + inline tests (Windows/macOS already satisfied)

### Phase 65: ThemeWatcher Core API
**Goal**: The public on_theme_change() API exists with ThemeWatcher RAII handle and ThemeChanged event type, feature-gated behind `watch`, with the notify dependency wired up
**Depends on**: Phase 61 (needs module structure for watch/ placement)
**Requirements**: WATCH-01, WATCH-06
**Success Criteria** (what must be TRUE):
  1. on_theme_change() function exists that accepts a callback and returns a ThemeWatcher handle -- dropping the handle stops watching
  2. ThemeChanged enum exists as a signal-only type (no theme data inside -- callers re-run from_system() on signal)
  3. The `watch` feature flag gates the entire on_theme_change() API and the notify dependency
  4. Compiling without the `watch` feature produces no additional dependencies
**Plans:** 2 plans
Plans:
- [ ] 63-01-PLAN.md — Extract pure from_kde_content_pure function and create 7 fixture .ini files
- [ ] 63-02-PLAN.md — Integration tests for all fixture scenarios

### Phase 66: Linux Watchers
**Goal**: Theme changes on KDE and GNOME desktops trigger ThemeChanged signals through the watcher API
**Depends on**: Phase 65
**Requirements**: WATCH-02, WATCH-03
**Success Criteria** (what must be TRUE):
  1. On KDE, changing the color scheme in System Settings triggers a ThemeChanged signal within 1 second (inotify on kdeglobals with 300ms debounce)
  2. On GNOME, toggling dark mode via Settings triggers a ThemeChanged signal (ashpd portal SettingChanged stream on background thread)
  3. The KDE watcher watches only specific files (kdeglobals, kcmfontsrc) -- not the entire ~/.config/ directory
  4. Multiple rapid KDE config writes (as happens during a theme switch) produce a single debounced signal, not a flood
**Plans:** 2 plans
Plans:
- [ ] 63-01-PLAN.md — Extract pure from_kde_content_pure function and create 7 fixture .ini files
- [ ] 63-02-PLAN.md — Integration tests for all fixture scenarios

### Phase 67: macOS and Windows Watchers
**Goal**: Theme changes on macOS and Windows trigger ThemeChanged signals through the watcher API
**Depends on**: Phase 65
**Requirements**: WATCH-04, WATCH-05
**Success Criteria** (what must be TRUE):
  1. On macOS, toggling Appearance in System Settings triggers a ThemeChanged signal (NSDistributedNotificationCenter for AppleInterfaceThemeChangedNotification)
  2. On Windows, changing the system theme triggers a ThemeChanged signal (UISettings::ColorValuesChanged with COM STA and message pump on watcher thread)
  3. Both watchers run on dedicated background threads without requiring the caller to provide an event loop
  4. Dropping the ThemeWatcher handle cleanly stops the background thread on both platforms
**Plans:** 2 plans
Plans:
- [ ] 63-01-PLAN.md — Extract pure from_kde_content_pure function and create 7 fixture .ini files
- [ ] 63-02-PLAN.md — Integration tests for all fixture scenarios

## Progress

**Execution Order:**
Phases execute in numeric order: 61 -> 62 -> 63 -> 64 -> 65 -> 66 -> 67
Note: Phases 62, 63, and 64 can run in parallel after Phase 61 completes (62 depends on 61; 63 and 64 are fully independent). Phases 66 and 67 can run in parallel after Phase 65 completes.

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1-8 | v0.1 | 14/14 | Complete | 2026-03-07 |
| 9-15 | v0.2 | 20/20 | Complete | 2026-03-09 |
| 16-21 | v0.3 | 10/10 | Complete | 2026-03-09 |
| 22-26 | v0.3.3 | 14/14 | Complete | 2026-03-17 |
| 27-32 | v0.4.0 | 8/8 | Complete | 2026-03-18 |
| 33-43 | v0.4.1 | 22/22 | Complete | 2026-03-21 |
| 44-48 | v0.5.0 | 17/17 | Complete | 2026-03-29 |
| 49-60 | v0.5.5 | 41/41 | Complete | 2026-04-09 |
| 61. lib.rs Module Split | v0.5.6 | 2/2 | Complete   | 2026-04-09 |
| 62. Validate Codegen | v0.5.6 | 3/3 | Complete   | 2026-04-09 |
| 63. KDE Reader Fixture Tests | v0.5.6 | 2/2 | Complete   | 2026-04-09 |
| 64. Cross-Platform Reader Test Separation | v0.5.6 | 0/0 | Not started | - |
| 65. ThemeWatcher Core API | v0.5.6 | 0/0 | Not started | - |
| 66. Linux Watchers | v0.5.6 | 0/0 | Not started | - |
| 67. macOS and Windows Watchers | v0.5.6 | 0/0 | Not started | - |
