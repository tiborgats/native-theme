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
- ✅ **v0.5.6 Internal Quality & Runtime Watching** — Phases 61-68 (shipped 2026-04-10)
- 🚧 **v0.5.7 API Overhaul** — Phases 69-90 (in progress)

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

<details>
<summary>v0.5.6 Internal Quality & Runtime Watching (Phases 61-68) — SHIPPED 2026-04-10</summary>

- [x] Phase 61: lib.rs Module Split (2/2 plans) — completed 2026-04-09
- [x] Phase 62: Validate Codegen (3/3 plans) — completed 2026-04-09
- [x] Phase 63: KDE Reader Fixture Tests (2/2 plans) — completed 2026-04-09
- [x] Phase 64: Cross-Platform Reader Test Separation (1/1 plan) — completed 2026-04-09
- [x] Phase 65: ThemeWatcher Core API (1/1 plan) — completed 2026-04-09
- [x] Phase 66: Linux Watchers (2/2 plans) — completed 2026-04-09
- [x] Phase 67: macOS and Windows Watchers (2/2 plans) — completed 2026-04-09
- [x] Phase 68: GTK Symbolic Icon Recoloring (1/1 plan) — completed 2026-04-10

</details>

### v0.5.7 API Overhaul (Phases 69-89)

- [x] **Phase 69: Resolver-Level button_order Unlock** — Ship-unit 1: delete macOS/KDE `button_order` hardcodes and move dispatch into the resolver so the `resolve()` docs stop lying (completed 2026-04-12)
- [x] **Phase 70: Drop Error::Clone Bound** — Ship-unit 3: four-item atomic removal of `#[derive(Clone)]`, stale doc comments, and the `error_is_clone` test (completed 2026-04-12)
- [x] **Phase 71: Error Restructure and Validation Split** — Ship-unit 2: partition `validate()` output into `missing` vs `out_of_range` and restructure `Error` per §31.2 Option F (completed 2026-04-12)
- [x] **Phase 72: ENV_MUTEX Test Simplification** — Ship-unit 4 (after 69): drop env-var-mocking serialization now that `resolve()` is pure (completed 2026-04-12)
- [x] **Phase 73: ThemeChangeEvent Cleanup** — Ship-unit 5: delete `Other` variant (zero emitters) and rename `ColorSchemeChanged` to `Changed` (completed 2026-04-12)
- [x] **Phase 74: Rgba Polish and must_use Uniformity** — Ship-unit 6 part A: delete `to_f32_tuple`, add default constants, and enforce bare `#[must_use]` across six sites (completed 2026-04-12)
- [x] **Phase 75: LinuxDesktop non_exhaustive, Compile-Gated Watchers, IconSet::default Removal** — Ship-unit 6 part B: mark `LinuxDesktop` non-exhaustive with new compositor variants, make missing `watch` feature a compile error, delete the misleading `IconSet::default()` (completed 2026-04-12)
- [x] **Phase 76: Type Vocabulary Rename and Crate Root Partition** — Ship-unit 7 part A: atomic rename of `ThemeSpec→Theme`, `ThemeVariant→ThemeMode`, etc. and partition 92-item flat crate root into submodules with a `prelude` (completed 2026-04-12)
- [x] **Phase 77: SystemTheme API and icon_set Relocation** — Ship-unit 7 part B: drop `SystemTheme::active()` in favour of `pick(ColorMode)` + exposed `mode` field, and move `icon_set`/`icon_theme` onto `Theme` (completed 2026-04-13)
- [x] **Phase 78: OverlaySource, AccessibilityPreferences, font_dpi Relocation** — Ship-unit 8 atomic: eliminate `SystemTheme` pre-resolve variant fields via `OverlaySource` replay, move accessibility off `ThemeDefaults` onto `SystemTheme`, move `font_dpi` into `ResolutionContext` (completed 2026-04-13)
- [x] **Phase 79: BorderSpec Split and Platform Reader Visibility Audit** — Ship-unit 9: split `BorderSpec` along defaults-vs-widget, grep-audit connector callers of platform readers, demote `from_kde`/`from_gnome`/`from_windows`/`from_macos` to `pub(crate)` (completed 2026-04-13)
- [x] **Phase 80: native-theme-derive Proc-Macro K Codegen** — Ship-unit 10: new `native-theme-derive` crate generating paired structs, `FIELD_NAMES`, `impl_merge!` bodies, `check_ranges`, `inventory::submit!` registry, and unified border inheritance attribute (completed 2026-04-13)
- [x] **Phase 81: Feature-Matrix Cleanup and Unified from_system** — Ship-unit 11 atomic: unify `from_system`/`from_system_async` via `pollster::block_on`, split aggregators into `linux-kde`/`linux-portal`-style groups, simplify `Cargo.toml` feature graph (completed 2026-04-13)
- [x] **Phase 82: Icon API Rework** — Collapse 13 icon-loading functions into `IconLoader` builder, migrate `IconProvider` and `IconData::Svg` to `Cow<'static, [u8]>`, add `IconRole::name()` / `Display`, add `IconSet` drift-guard test (completed 2026-04-13)
- [x] **Phase 83: Detection Cache Layer** — Replace global `OnceLock` caches with `DetectionContext` backed by `arc_swap::ArcSwapOption`, add no-arg `detect_linux_desktop()` overload (completed 2026-04-13)
- [x] **Phase 84: Reader Output Contract Homogenisation** — Unify single-vs-dual variant semantics across KDE/GNOME/Windows/macOS readers via a `ReaderOutput` type flowing through `run_pipeline` alongside `OverlaySource` (completed 2026-04-13)
- [x] **Phase 85: Data Model Method and Doc Cleanup** — Demote `ThemeVariant::resolve*` intermediates to `#[doc(hidden)]`, `Theme` method grab-bag cleanup, document `ThemeWatcher` internals, rename `FontSize::Px::to_px` to `to_logical_px` (completed 2026-04-13)
- [x] **Phase 86: Validation and Lint Codegen Polish** — Drive `lint_toml` from the `inventory::submit!` widget registry, stop `check_ranges` from eagerly `format!`-ing path strings (completed 2026-04-13)
- [x] **Phase 87: Font Family Arc<str> and AnimatedIcon Invariants** — Migrate `FontSpec::family: String` to `Arc<str>` across widget × connector, wrap `AnimatedIcon` public fields in newtype constructors that enforce invariants (completed 2026-04-13)
- [x] **Phase 88: Diagnostic and Preset-Polish Sweep** — `diagnose_platform_support` returns `Vec<DiagnosticEntry>`, `platform_preset_name` returns structured data, `FontSpec::style` default-consistency documented, `defaults.border.padding` rule corrected, bundled preset `name`/`icon_theme` become `Cow<'static, str>` (completed 2026-04-14)
- [x] **Phase 89: Post-Partition Doctest Path Fixes** — Fix 6 stale doctest API paths in watch/mod.rs and rasterize.rs, remove dangling doc link to deleted load_custom_icon (gap closure from milestone audit) (completed 2026-04-15)

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
**Plans:** 1/1 plans complete
Plans:
- [x] 64-01-PLAN.md — GNOME pure function extraction + inline tests (Windows/macOS already satisfied)

### Phase 65: ThemeWatcher Core API
**Goal**: The public on_theme_change() API exists with ThemeWatcher RAII handle and ThemeChanged event type, feature-gated behind `watch`, with the notify dependency wired up
**Depends on**: Phase 61 (needs module structure for watch/ placement)
**Requirements**: WATCH-01, WATCH-06
**Success Criteria** (what must be TRUE):
  1. on_theme_change() function exists that accepts a callback and returns a ThemeWatcher handle -- dropping the handle stops watching
  2. ThemeChanged enum exists as a signal-only type (no theme data inside -- callers re-run from_system() on signal)
  3. The `watch` feature flag gates the entire on_theme_change() API and the notify dependency
  4. Compiling without the `watch` feature produces no additional dependencies
**Plans:** 1/1 plans complete
Plans:
- [x] 65-01-PLAN.md — ThemeChangeEvent, ThemeWatcher, on_theme_change() stub with watch feature gate

### Phase 66: Linux Watchers
**Goal**: Theme changes on KDE and GNOME desktops trigger ThemeChanged signals through the watcher API
**Depends on**: Phase 65
**Requirements**: WATCH-02, WATCH-03
**Success Criteria** (what must be TRUE):
  1. On KDE, changing the color scheme in System Settings triggers a ThemeChanged signal within 1 second (inotify on kdeglobals with 300ms debounce)
  2. On GNOME, toggling dark mode via Settings triggers a ThemeChanged signal (ashpd portal SettingChanged stream on background thread)
  3. The KDE watcher watches only specific files (kdeglobals, kcmfontsrc) -- not the entire ~/.config/ directory
  4. Multiple rapid KDE config writes (as happens during a theme switch) produce a single debounced signal, not a flood
**Plans:** 2/2 plans complete
Plans:
- [x] 66-01-PLAN.md — KDE inotify watcher and GNOME D-Bus portal watcher backends
- [x] 66-02-PLAN.md — Wire on_theme_change() dispatch with DE detection and test suite

### Phase 67: macOS and Windows Watchers
**Goal**: Theme changes on macOS and Windows trigger ThemeChanged signals through the watcher API
**Depends on**: Phase 65
**Requirements**: WATCH-04, WATCH-05
**Success Criteria** (what must be TRUE):
  1. On macOS, toggling Appearance in System Settings triggers a ThemeChanged signal (NSDistributedNotificationCenter for AppleInterfaceThemeChangedNotification)
  2. On Windows, changing the system theme triggers a ThemeChanged signal (UISettings::ColorValuesChanged with COM STA and message pump on watcher thread)
  3. Both watchers run on dedicated background threads without requiring the caller to provide an event loop
  4. Dropping the ThemeWatcher handle cleanly stops the background thread on both platforms
**Plans:** 2/2 plans complete
Plans:
- [x] 67-01-PLAN.md — macOS watcher backend + ThemeWatcher platform shutdown infrastructure
- [x] 67-02-PLAN.md — Windows watcher backend with COM STA + UISettings::ColorValuesChanged

### Phase 68: GTK Symbolic Icon Recoloring
**Goal**: GTK-convention symbolic SVGs are normalized to use `currentColor` so that dark-on-dark icons on GNOME/GTK desktops are correctly recolored by existing connector colorize logic
**Depends on**: Phase 67
**Plans:** 1/1 plans complete

Plans:
- [x] 68-01-PLAN.md — Normalize GTK symbolic SVGs to currentColor with TDD (find_icon refactor, normalize_gtk_symbolic, 9 tests)

### Phase 69: Resolver-Level button_order Unlock
**Goal**: Callers of `from_kde` / `from_macos` no longer observe a hardcoded `button_order` in the pre-resolve `ThemeMode`, and `resolve()`'s documentation about "no OS detection" becomes literally true
**Depends on**: Phase 68 (last v0.5.6 phase)
**Requirements**: BUG-03, BUG-04, BUG-05
**Success Criteria** (what must be TRUE):
  1. A fixture test asserts that `from_kde_content_pure(breeze_light.ini)` returns a `ThemeMode` whose `defaults.button_order` is `None` (not `Some("kde")`)
  2. A pure test asserts that `from_macos::build_theme(light, sonoma_defaults)` returns a `ThemeMode` whose `defaults.button_order` is `None` (not `Some("apple")`)
  3. After `resolve()` runs the `button_order` field is still `"kde"` on KDE and `"apple"` on macOS — dispatch moves from the readers into `resolve_platform_defaults` (or `resolve` intermediates are demoted)
  4. The `resolve()` rustdoc no longer claims "no OS detection" unless it is literally true for every code path reachable from that function
**Plans**: 2 plans
Plans:
- [x] 69-01-PLAN.md — Remove reader hardcodes, move dispatch to resolve_platform_defaults
- [x] 69-02-PLAN.md — Strip live preset TOMLs, update rustdoc, create presets README

### Phase 70: Drop Error::Clone Bound
**Goal**: `Error` no longer implements `Clone`, and every byproduct of that bond — stale doc comments, stale tests, stale preset comments — is atomically removed in the same commit so the build never enters an intermediate broken state
**Depends on**: Phase 68
**Requirements**: ERR-01, CLEAN-01
**Success Criteria** (what must be TRUE):
  1. `error.rs` does not contain `#[derive(Clone)]` on the `Error` enum; the deleted `error_is_clone` test and grep-based verification (Clone count = 2, both on ThemeResolutionError) guard against reintroduction
  2. The `error_is_clone` test file is deleted and `cargo test` still passes on a fresh clone
  3. The stale `error.rs:73-79` and `presets.rs:85-92` doc comments describing Clone behaviour are gone (grep for "Clone" in those files returns zero matches)
  4. The four-item commit is a single atomic change — bisection cannot land on a revision where Clone is dropped but `error_is_clone` still exists
**Plans:** 1/1 plans complete
Plans:
- [x] 70-01-PLAN.md — Drop Clone from Error enum and remove all stale byproducts

### Phase 71: Error Restructure and Validation Split
**Goal**: `validate()` distinguishes missing fields from out-of-range values in its output, and the `Error` enum is restructured per §31.2 Option F so callers can match on `ReaderFailed` / `FeatureDisabled` / `WatchUnavailable` / `Resolution` with an `Error::kind()` helper for coarse dispatch
**Depends on**: Phase 70
**Requirements**: BUG-01, BUG-02, ERR-02
**Success Criteria** (what must be TRUE):
  1. `ThemeResolutionError` is deleted; its successor Error variants `ResolutionIncomplete { missing }` and `ResolutionInvalid { errors }` carry the two categories separately. `validate()` short-circuits on missing before running `check_ranges`, so range checks only run on fully-populated data (no spurious errors from `T::default()` placeholders).
  2. A unit test constructs a `ThemeMode` with both a missing field AND an out-of-range value: the result is `ResolutionIncomplete` containing only the missing path (no spurious range violation). A second test with NO missing fields but an out-of-range value produces `ResolutionInvalid` with the `RangeViolation` entry.
  3. `Error` variants conform to §31.2 Option F: flat list with explicit boundaries for reader failures, feature-disabled calls, and watcher unavailability; `Error::kind()` returns a coarse `ErrorKind` classifier
  4. `from_system()` returns a `FeatureDisabled` error (not a `Format` error) when called without the right platform feature enabled
**Plans**: 2 plans
Plans:
- [x] 71-01-PLAN.md — Rewrite Error enum per Option F with ErrorKind and RangeViolation
- [x] 71-02-PLAN.md — Two-vec validation split and caller migration

### Phase 72: ENV_MUTEX Test Simplification
**Goal**: The test suite no longer relies on a global `ENV_MUTEX` to serialize env-var-mocking tests, because `resolve()` is now pure and does not read env vars
**Depends on**: Phase 69 (needs BUG-03's "demote resolve intermediates" or "move button_order dispatch" to have landed)
**Requirements**: CLEAN-02
**Success Criteria** (what must be TRUE):
  1. `grep -r ENV_MUTEX tests/` returns zero matches
  2. Tests that previously required the mutex now run with `cargo test -- --test-threads=N` for any N without flakiness
  3. The mutex helper module (`env_mutex.rs` or equivalent) is deleted
  4. A test-suite timing measurement shows no regression — parallel test execution is faster than the mutex-serialized baseline
**Plans**: 2 plans
Plans:
- [x] 72-01-PLAN.md — Rewrite all ENV_MUTEX tests to pure equivalents
- [x] 72-02-PLAN.md — Delete test_util.rs and verify parallel test timing

### Phase 73: ThemeChangeEvent Cleanup
**Goal**: The `ThemeChangeEvent` enum reflects what watchers actually emit — `Other` is gone (it had zero production emitters), and `ColorSchemeChanged` is renamed to `Changed` because KDE/GNOME watchers signal broader changes than just colour-scheme toggles
**Depends on**: Phase 68
**Requirements**: WATCH-01, WATCH-02
**Success Criteria** (what must be TRUE):
  1. `ThemeChangeEvent::Other` no longer exists; `grep` in `src/` and `tests/` returns zero matches
  2. The former `ColorSchemeChanged` variant is named `Changed` and the rename is reflected in doc comments, public API, and the `ThemeWatcher` callback signature
  3. All four watcher backends (KDE inotify, GNOME portal, macOS NSDistributedNotificationCenter, Windows UISettings) emit the renamed `Changed` variant
  4. A doctest on `on_theme_change()` pattern-matches on `ThemeChangeEvent::Changed` and compiles
**Plans**: 2 plans
Plans:
- [ ] 71-01-PLAN.md � Rewrite Error enum per Option F with ErrorKind and RangeViolation
- [ ] 71-02-PLAN.md � Two-vec validation split and caller migration

### Phase 74: Rgba Polish and must_use Uniformity
**Goal**: The `Rgba` type has default colour constants, the confusingly-named `to_f32_tuple` is gone, and every function/type on the short must-use list gets a bare `#[must_use]` attribute — uniform convention, no mixed `#[must_use = "..."]` strings
**Depends on**: Phase 68
**Requirements**: COLOR-01, POLISH-03
**Success Criteria** (what must be TRUE):
  1. `Rgba::TRANSPARENT`, `Rgba::BLACK`, `Rgba::WHITE` constants are defined and documented, with a doctest demonstrating their use
  2. `Rgba::to_f32_tuple` no longer exists — callers use the canonical accessor (documented in the migration note)
  3. The six call sites listed in the design doc (`pipeline.rs:132`, `pipeline.rs:175`, `model/icons.rs:438`, `model/icons.rs:477`, `lib.rs:353`, `model/mod.rs:225`) all carry a bare `#[must_use]` attribute
  4. A clippy lint (or grep audit) confirms no `#[must_use = "..."]` form remains anywhere in the crate
**Plans**: 2 plans
Plans:
- [x] 74-01-PLAN.md — Rgba constants (TRANSPARENT, BLACK, WHITE) and delete to_f32_tuple
- [x] 74-02-PLAN.md — Uniform bare #[must_use] across the core crate (36 sites)

### Phase 75: LinuxDesktop non_exhaustive, Compile-Gated Watchers, IconSet::default Removal
**Goal**: `LinuxDesktop` gains `#[non_exhaustive]` and new Wayland compositor variants, `on_theme_change()` fails at compile time (not runtime) when the `watch` feature is disabled, and the misleading `IconSet::default()` (which was Freedesktop on every platform) is gone
**Depends on**: Phase 68
**Requirements**: LAYOUT-02, WATCH-03, ICON-05
**Success Criteria** (what must be TRUE):
  1. `LinuxDesktop` has `#[non_exhaustive]` and new variants: `Hyprland`, `Sway`, `River`, `Niri`, `CosmicDe` (or the subset confirmed in the design doc)
  2. `cargo check --no-default-features` with a program that calls `on_theme_change()` fails with a compile error mentioning the `watch` feature — not a runtime `FeatureDisabled` error
  3. `IconSet::default()` no longer exists; attempts to call it produce a "no method named default" compile error, and the migration guide documents `system_icon_set()` as the replacement
  4. Matching on `LinuxDesktop` without a wildcard arm produces a "non-exhaustive patterns" compile error, which is the correct forward-compat behaviour
**Plans**: 2 plans
Plans:
- [x] 75-01-PLAN.md — LinuxDesktop #[non_exhaustive] + Wayland compositor variants
- [x] 75-02-PLAN.md — Remove IconSet::default() and verify watch compile-gating
**UI hint**: yes

### Phase 76: Type Vocabulary Rename and Crate Root Partition
**Goal**: The v0.5.7 type vocabulary lands atomically (`ThemeSpec→Theme`, `ThemeVariant→ThemeMode`, `ResolvedThemeVariant→ResolvedTheme`, `ResolvedThemeDefaults→ResolvedDefaults`) alongside a crate-root partition that turns the 92-item flat `lib.rs` surface into `theme::`, `watch::`, `icons::`, `detect::` submodules plus a `prelude` exposing the 6 most-used items
**Depends on**: Phase 68
**Requirements**: NAME-01, LAYOUT-01
**Success Criteria** (what must be TRUE):
  1. `use native_theme::{Theme, ThemeMode, ResolvedTheme, ResolvedDefaults}` compiles; `use native_theme::{ThemeSpec, ThemeVariant, ResolvedThemeVariant, ResolvedThemeDefaults}` does not
  2. Both connector crates (`native-theme-gpui`, `native-theme-iced`) compile against the renamed types without any compatibility shim
  3. `native_theme::prelude::*` re-exports exactly the 6 items listed in design doc §12 Option C+F, and a `tests/prelude_smoke.rs` test asserts the set
  4. `lib.rs` exposes exclusively `pub mod` declarations and the `prelude` — the 92 flat re-exports are gone; the old top-level items are reachable only via `native_theme::theme::`, `native_theme::watch::`, etc.
**Plans**: 2 plans
Plans:
- [x] 76-01-PLAN.md — Atomic type vocabulary rename (ThemeSpec->Theme, ThemeVariant->ThemeMode, etc.)
- [x] 76-02-PLAN.md — Crate root partition into submodules with prelude

### Phase 77: SystemTheme API and icon_set Relocation
**Goal**: `SystemTheme::active()` is gone, replaced by `pick(ColorMode)` plus a public `mode: ColorMode` field, and `icon_set` / `icon_theme` live on `Theme` instead of their former (wrong) host type
**Depends on**: Phase 76
**Requirements**: MODEL-03, MODEL-06
**Success Criteria** (what must be TRUE):
  1. `SystemTheme::active()` no longer exists; callers migrate to `pick(sys.mode)` which returns the light or dark `ResolvedTheme`
  2. `system_theme.mode` is directly readable as a public field, verified by a doctest demonstrating `let dark = sys.pick(ColorMode::Dark);`
  3. `theme.icon_set` and `theme.icon_theme` are accessible from `Theme`, not from the former host type, and all connector call sites use the new path
  4. A doctest shows loading a preset, reading `theme.icon_set`, and passing it to the icon loader — zero reference to the old location
**Plans**: 2 plans
Plans:
- [x] 77-01-PLAN.md � ColorMode enum + drop active() + pick(ColorMode) API
- [x] 77-02-PLAN.md � icon_set/icon_theme relocation from ThemeMode to Theme

### Phase 78: OverlaySource, AccessibilityPreferences, font_dpi Relocation
**Goal**: `SystemTheme` no longer carries pre-resolve variant fields — overlays are re-played from an `OverlaySource` via `ResolutionContext`; accessibility preferences live on `SystemTheme` as a structured `AccessibilityPreferences`; `font_dpi` moves out of `ThemeDefaults` into `ResolutionContext` runtime data
**Depends on**: Phase 77 (builds on the renamed types and the `pick(ColorMode)` API)
**Requirements**: MODEL-02, ACCESS-01, ACCESS-02
**Success Criteria** (what must be TRUE):
  1. `SystemTheme` has no `_variant` / pre-resolve overlay fields; `with_overlay()` rebuilds via `ResolutionContext` from a stored `OverlaySource`, verified by a round-trip test
  2. `AccessibilityPreferences { text_scaling_factor, reduce_motion, high_contrast, reduce_transparency }` lives on `SystemTheme`, NOT in `ResolutionContext`, and all four fields are populated by every OS reader
  3. `ResolutionContext` carries `font_dpi` as runtime data; `ThemeDefaults::font_dpi` is gone and `ResolvedDefaults::font_dpi` is gone too — grep returns zero matches in both types
  4. All 17 presets continue to resolve successfully and produce identical `ResolvedTheme` outputs for tests that existed before the refactor
**Plans**: 4 plans
Plans:
- [x] 78-01-PLAN.md — AccessibilityPreferences extraction + font_dpi removal from ThemeDefaults
- [x] 78-02-PLAN.md — OverlaySource replaces light_variant/dark_variant on SystemTheme
- [x] 78-03-PLAN.md — Fix gpui connector compile errors (gap closure)
- [x] 78-04-PLAN.md — Wire OS accessibility values through pipeline (gap closure)









### Phase 79: BorderSpec Split and Platform Reader Visibility Audit
**Goal**: `BorderSpec` is split along defaults-vs-widget lines (widget-level carries only `color`; defaults-level adds `width`, `corner_radius`, `padding`), and the platform readers (`from_kde`, `from_gnome`, `from_windows`, `from_macos`) are demoted to `pub(crate)` after a grep-based audit of connector consumers
**Depends on**: Phase 78
**Requirements**: BORDER-01, CLEAN-03, READER-02
**Success Criteria** (what must be TRUE):
  1. `BorderSpec` on widget structs exposes only `color`; the defaults-level `BorderDefaultsSpec` (or named equivalent) adds `width`, `corner_radius`, `padding`
  2. A grep of `native-theme-gpui`, `native-theme-iced`, and every `examples/` file finds zero references to `from_kde` / `from_gnome` / `from_windows` / `from_macos` — all consumers went through `from_system`
  3. Attempting `pub use native_theme::from_kde` from an external crate fails with a visibility error
  4. The unified border-resolution pathway correctly populates widget borders from defaults-level values for all 17 presets
**Plans**: 2 plans
Plans:
- [x] 79-01-PLAN.md — Split BorderSpec into DefaultsBorderSpec and WidgetBorderSpec
- [x] 79-02-PLAN.md — L3 visibility audit and platform reader demotion to pub(crate)

### Phase 80: native-theme-derive Proc-Macro K Codegen
**Goal**: The doubled `Option<T>` / `Resolved<T>` struct hierarchy is generated from one source of truth by a new `native-theme-derive` proc-macro crate that also emits `FIELD_NAMES`, `impl_merge!` bodies, `check_ranges` impls from `#[theme(range = "...")]` and `#[theme(check = "non_negative")]` attributes, per-field `#[theme(inherit_from = "...")]` inheritance rules, `inventory::submit!` widget registry entries, and a `#[theme_layer(border_kind = "full" | "partial" | "none")]` unifier for the three parallel border-inheritance validation paths
**Depends on**: Phase 79 (lands after the border split so codegen has a clean target) and Phase 71 (needs the new Error shape)
**Requirements**: MODEL-01, VALID-01, VALID-02, BORDER-02
**Success Criteria** (what must be TRUE):
  1. `native-theme-derive` exists as a separate workspace crate (`proc-macro = true`) and produces the same `ButtonTheme` / `ResolvedButtonTheme` pair the hand-written version did — `cargo expand` confirms equivalence on at least one widget
  2. `validate.rs` shrinks by at least 720 lines of hand-written range-check / construction boilerplate, and `grep -c fn check_ranges src/resolved/*.rs` drops to zero (all generated)
  3. `inheritance.rs` duplication with `inheritance-rules.toml` is gone — ~55 of 82 rules live on per-field `#[theme(inherit_from = "...")]` attributes, and pattern-based rules that stay hand-written are documented in a comment block
  4. The three parallel `require_border` / `border_all_optional` / `require_border_partial` paths collapse into a single generated path selected by `#[theme_layer(border_kind = "...")]`, with no behavioural change for any of the 17 presets
**Plans:** 2/2 plans complete
Plans:
- [x] 80-01-PLAN.md — Create native-theme-derive proc-macro crate and prototype on ButtonTheme
- [x] 80-02-PLAN.md — Migrate all 25 widgets, wire inheritance attributes, and inventory registry

### Phase 81: Feature-Matrix Cleanup and Unified from_system
**Goal**: `from_system_async` and `from_system` collapse to a single code path (`from_system_async` becomes the implementation, `from_system` wraps it with `pollster::block_on`), the `Cargo.toml` feature graph is simplified with clearer `linux-kde` / `linux-portal` aggregators, and these three changes ship atomically so no intermediate revision has a broken feature matrix
**Depends on**: Phase 80 (ships last so it absorbs every other change)
**Requirements**: FEATURE-01, FEATURE-02, FEATURE-03
**Success Criteria** (what must be TRUE):
  1. `src/lib.rs` has exactly one `async fn from_system_inner(...)` and two public wrappers — sync `from_system` (via `pollster::block_on`) and async `from_system_async` — with zero duplicated orchestration logic
  2. `Cargo.toml` features include `linux-kde` and `linux-portal` (or the design-doc names), each aggregating the right target-specific dependencies, and the old opaque aggregators are gone
  3. `cargo hack check --each-feature` (or equivalent) passes for every feature combination the CI matrix enumerates
  4. A sync-only consumer (no tokio) can call `from_system()` without pulling an async runtime into its dependency graph — verified by a test harness built with `--no-default-features` and only sync features enabled
**Plans:** 2/2 plans complete
Plans:
- [x] 81-01-PLAN.md — Unify from_system/from_system_async code paths, restructure Cargo.toml feature graph
- [x] 81-02-PLAN.md — Update CI matrix and full feature-combination verification

### Phase 82: Icon API Rework
**Goal**: The 13 icon-loading functions collapse into a single `IconLoader::new(impl Into<IconId>).set(...).size(...).load()` builder (doc 1 §8 Option C); `IconProvider::icon_svg` and `IconData::Svg` both migrate to `Cow<'static, [u8]>` to eliminate the `Vec<u8>` copy on bundled loads and remove the `&'static [u8]` lifetime lock; `IconRole` gains a kebab-case `name()` method with an `impl Display` delegate; a drift-guard test covers `IconSet::from_name` / `name` round-trip
**Depends on**: Phase 68
**Requirements**: ICON-01, ICON-02, ICON-03, ICON-04, ICON-06, ICON-07
**Success Criteria** (what must be TRUE):
  1. `IconLoader::new(IconRole::ActionSave).set(IconSet::Freedesktop).size(32).load()` returns the expected `IconData` for every platform, and the old 13 standalone functions no longer exist as public API
  2. `IconProvider::icon_svg` returns `Cow<'static, [u8]>`, and a test confirms bundled icon loads produce `Cow::Borrowed` (zero allocation) while runtime icons produce `Cow::Owned`
  3. `IconRole::ActionSave.name()` returns `"action-save"` and `format!("{}", IconRole::ActionSave) == "action-save"` — `Display` delegates to `name()`, not `Debug::fmt`
  4. A drift-guard test asserts `IconSet::from_name(set.name()) == Some(set)` for every variant — if a new variant is added without updating `from_name`, the test fails
  5. The freedesktop size-24 hardcode is gone: `IconLoader::new(IconRole::ActionSave).set(IconSet::Freedesktop).size(48).load()` on Linux requests a 48px icon from the freedesktop spec, not a 24px icon
**Plans**: 2 plans
Plans:
- [ ] 71-01-PLAN.md � Rewrite Error enum per Option F with ErrorKind and RangeViolation
- [ ] 71-02-PLAN.md � Two-vec validation split and caller migration

### Phase 83: Detection Cache Layer
**Goal**: Global `OnceLock` caches in `detect` and `model/icons` are replaced by a `DetectionContext` backed by `arc_swap::ArcSwapOption<T>` — callers get "cache on first read" for convenience and "force re-read on demand" for watchers that need fresh data; `detect_linux_desktop()` gains a no-arg convenience overload that removes the current two-call idiom
**Depends on**: Phase 68
**Requirements**: DETECT-01, DETECT-02
**Success Criteria** (what must be TRUE):
  1. `DetectionContext` struct exists, provides `linux_desktop()`, `is_dark()`, `prefers_reduced_motion()` with transparent first-call caching, and a `invalidate()` method that forces the next call to re-read
  2. A test confirms `ctx.linux_desktop()` reads the environment once and returns the cached result on subsequent calls, and that after `ctx.invalidate_linux_desktop()` the next call re-reads
  3. `detect_linux_desktop()` with no arguments compiles and returns a `LinuxDesktop` — no two-call `let env = ...; detect_linux_desktop(&env)` idiom required
  4. A `grep -c OnceLock src/detect.rs src/model/icons.rs` returns zero — all global caching has moved to `DetectionContext`
**Plans**: 2 plans
Plans:
- [x] 83-01-PLAN.md — Rename detect_linux_de to detect_linux_desktop with no-arg convenience
- [x] 83-02-PLAN.md — DetectionContext struct with ArcSwapOption lock-free caching

### Phase 84: Reader Output Contract Homogenisation
**Goal**: The four platform readers share a unified `ReaderOutput` contract that expresses single-vs-dual variant semantics explicitly — `ReaderOutput::Single(ThemeMode)` for GNOME/KDE/Windows (the OS only reports the active mode), `ReaderOutput::Dual { light, dark }` for macOS (both modes readable), and the type flows through `run_pipeline` alongside the `OverlaySource` added in Phase 78
**Depends on**: Phase 78 (needs `OverlaySource` in place)
**Requirements**: READER-01
**Success Criteria** (what must be TRUE):
  1. `ReaderOutput` enum exists with `Single(ThemeMode)` and `Dual { light: ThemeMode, dark: ThemeMode }` variants; all four platform readers return it
  2. `run_pipeline` accepts `ReaderOutput` and `OverlaySource` and produces a `SystemTheme` without any per-platform branching logic in the pipeline itself
  3. A test confirms that the macOS `Dual` path populates both `sys.light` and `sys.dark` from a single reader call, while the KDE `Single` path populates only the active mode
  4. The previously-scattered "does this reader return both variants?" comments are gone — the type system expresses the contract
**Plans**: 2 plans
Plans:
- [x] 84-01-PLAN.md — Define ReaderOutput enum, rewrite pipeline core (run_pipeline, with_overlay, OverlaySource)
- [x] 84-02-PLAN.md — Migrate all four readers to return ReaderOutput, add contract test, remove variant-ambiguity comments

### Phase 85: Data Model Method and Doc Cleanup
**Goal**: `ThemeMode::resolve*` intermediates are demoted to `#[doc(hidden)] pub` so integration tests still reach them but rustdoc stops advertising them; `Theme`'s method grab-bag is cleaned up (including the coordinated removal of `from_toml_with_base` and the `error.rs:63` hint message); `ThemeWatcher` internals and constructor split are documented with a rename if the old name no longer fits; `FontSize::Px(v).to_px(dpi)` is renamed to `to_logical_px` so the DPI parameter stops being silently ignored in the `Px` branch
**Depends on**: Phase 77 (after the rename lands so docs reference the new names)
**Requirements**: MODEL-04, MODEL-05, NAME-02, NAME-03
**Success Criteria** (what must be TRUE):
  1. `cargo doc --no-deps` for `native_theme::theme::ThemeMode` does not list the `resolve_*` intermediate methods, but `ThemeMode::resolve_intermediate_for_tests()` or equivalent still compiles from an integration test (`#[doc(hidden)] pub` confirmed)
  2. `from_toml_with_base` is gone and the `error.rs:63` hint message no longer references it — the hint points callers to the new intended path
  3. `FontSize::Px(v).to_logical_px(dpi)` exists, and a doctest asserts that the DPI parameter is still respected for the `Pt` branch while the `Px` branch returns `v` unchanged — the rename makes the asymmetry obvious at the call site
  4. `ThemeWatcher`'s internals and constructor split have a module-level doc block explaining the RAII ownership model, the shutdown mechanism, and why the public constructor is the way it is
**Plans**: 2 plans
Plans:
- [ ] 71-01-PLAN.md � Rewrite Error enum per Option F with ErrorKind and RangeViolation
- [ ] 71-02-PLAN.md � Two-vec validation split and caller migration

### Phase 86: Validation and Lint Codegen Polish
**Goal**: `lint_toml` is driven by the `inventory::submit!` widget registry entries from Phase 80 instead of the ~215 hand-maintained string literals; `check_ranges` stops eager `format!` path-string construction so the fast path (everything in range) allocates zero path strings
**Depends on**: Phase 80 (needs the `inventory::submit!` registry)
**Requirements**: VALID-03, VALID-04
**Success Criteria** (what must be TRUE):
  1. `lint_toml` iterates `inventory::iter::<WidgetRegistration>()` to discover valid widget field names; a `grep -c "\"button\"\\|\"slider\"\\|\"textinput\"" src/lint_toml.rs` returns zero
  2. Adding a new widget via the derive macro in Phase 80 automatically teaches `lint_toml` about it — verified by a test that registers a dummy widget and confirms `lint_toml` rejects an unknown field on the new widget
  3. Benchmark (or counted allocation test) confirms that `check_ranges` on a valid `ThemeMode` allocates zero path strings — strings are allocated lazily only when a range error is reported
  4. All existing range-check tests still pass with the lazy allocation path
**Plans**: 2 plans
Plans:
- [x] 86-01-PLAN.md — Rewrite lint_toml to use inventory widget registry
- [x] 86-02-PLAN.md — Lazy path-string allocation in check_ranges

### Phase 87: Font Family Arc<str> and AnimatedIcon Invariants
**Goal**: `FontSpec::family` migrates from `String` to `Arc<str>` across the core widget × connector leak surface (needs `serde rc` feature flag; gpui and iced connector `.family` access updated in lockstep); `AnimatedIcon`'s public fields are replaced with newtype wrappers that enforce construction invariants, so users cannot construct an invalid `AnimatedIcon` by assigning to public fields
**Depends on**: Phase 77 (after the type rename lands so `Theme`/`ThemeMode` paths are stable)
**Requirements**: LAYOUT-03, LAYOUT-04
**Success Criteria** (what must be TRUE):
  1. `FontSpec::family` is `Arc<str>`, serde serialization uses the `rc` feature, and `cargo test --all-features` passes in core, `native-theme-gpui`, and `native-theme-iced`
  2. A benchmark (or allocation-counting test) confirms that cloning a `FontSpec` across widgets no longer clones the underlying family string — the `Arc<str>` is shared
  3. `AnimatedIcon::Frames { frames, duration, .. }` field access is replaced with a constructor (`AnimatedIcon::frames(frames, duration)?`) that returns `Result` on invalid input (e.g., empty frame list, zero duration)
  4. Attempting `AnimatedIcon::Frames { frames: vec![], duration: Duration::ZERO, .. }` fails to compile because the fields are no longer directly accessible — invariants are enforced by construction
**Plans**: 3 plans
Plans:
- [x] 87-01-PLAN.md � AnimatedIcon newtype wrappers and construction invariants
- [x] 87-02-PLAN.md — FontSpec core type definitions: Arc<str> family
- [x] 87-03-PLAN.md — FontSpec connector and platform migration to Arc<str>
**UI hint**: yes

### Phase 88: Diagnostic and Preset-Polish Sweep
**Goal**: `diagnose_platform_support` returns a structured `Vec<DiagnosticEntry>` instead of a `Vec<String>`; `platform_preset_name` returns structured data instead of leaking the internal `-live` naming convention; `FontSpec::style` default-consistency is documented (or corrected); the `defaults.border.padding` derives-from-presence rule is documented or corrected as part of the BORDER-01 follow-up; bundled preset `name` and `icon_theme` are stored as `Cow<'static, str>` to avoid the owned-String allocation on every preset load
**Depends on**: Phase 79 (after the BorderSpec split so POLISH-05 can document the final rule)
**Requirements**: POLISH-01, POLISH-02, POLISH-04, POLISH-05, POLISH-06
**Success Criteria** (what must be TRUE):
  1. `diagnose_platform_support()` returns `Vec<DiagnosticEntry>` where each entry has a `name`, `status`, and optional `detail` field — a doctest shows formatting the output as a table
  2. `platform_preset_name()` returns a struct with a `name: &'static str` and `is_live: bool` field — the `-live` suffix is no longer embedded in a concatenated string
  3. A doctest loads `preset("dracula")` and confirms `preset.name.is_borrowed()` — bundled presets skip the owned-String allocation via `Cow::Borrowed`
  4. The `FontSpec::style` default behaviour is either documented in a rustdoc block (if kept as-is) or corrected to error like its siblings (if changed); likewise `defaults.border.padding` derives-from-presence
  5. `platform_preset_name().name` never contains `-live` — the suffix is internal to `live_name()` and preset entry keys, not exposed in user-facing return values
**Plans:** 2 plans
Plans:
- [x] 88-01-PLAN.md — DiagnosticEntry and PlatformPreset structured return types
- [x] 88-02-PLAN.md — Cow migration for preset name/icon_theme and documentation polish

### Phase 89: Post-Partition Doctest Path Fixes
**Goal**: Fix 6 stale doctest API paths in `watch/mod.rs` and `rasterize.rs` that use pre-Phase-76 flat crate root paths instead of module-qualified paths, and remove 1 dangling doc link to a deleted function in `model/icons.rs`
**Depends on**: Phase 88
**Requirements**: Gap closure (audit tech debt)
**Gap Closure**: Closes stale doctest paths from v0.5.7-MILESTONE-AUDIT.md
**Success Criteria** (what must be TRUE):
  1. `watch/mod.rs` module doctest and `on_theme_change` fn doctest use `native_theme::watch::on_theme_change`, `native_theme::watch::ThemeChangeEvent`, and `native_theme::error::Error` — not flat crate root paths
  2. `rasterize.rs` doctest uses `native_theme::theme::IconData` — not `native_theme::IconData`
  3. `model/icons.rs` has zero references to `crate::load_custom_icon` in doc comments
  4. `cargo doc -p native-theme --no-deps` produces zero warnings about broken intra-doc links
**Plans:** 1/1 plans complete
Plans:
- [x] 89-01-PLAN.md — Fix stale doctest paths and dangling doc link

## Progress

**Execution Order (v0.5.7):**

Phases execute in numeric order 69 → 88 with the following parallelism hints:
- Phases 69, 70, 73, 74, 75, 76, 82, 83 are independent of each other (only depend on Phase 68) and can run in parallel
- Phase 72 must land after Phase 69 (the CLEAN-02 mutex simplification requires BUG-03's "resolve() is pure" guarantee)
- Phase 71 must land after Phase 70 (Error restructure builds on the Clone removal)
- Phase 77 must land after Phase 76 (API changes build on the rename)
- Phase 78 must land after Phase 77 (OverlaySource/font_dpi relocation builds on `pick(ColorMode)` and `Theme.icon_set`)
- Phase 79 must land after Phase 78 (border split lands on the post-accessibility model)
- Phase 80 must land after Phases 79 and 71 (proc-macro needs the clean border target + the new Error shape)
- Phase 81 must land after Phase 80 (feature-matrix cleanup absorbs every other change — ships last)
- Phase 84 must land after Phase 78 (needs `OverlaySource` in place)
- Phase 85, 87 must land after Phase 77 (after the rename)
- Phase 86 must land after Phase 80 (needs the `inventory::submit!` registry)
- Phase 88 must land after Phase 79 (POLISH-05 documents the final BorderSpec rule)

**Ship-unit atomicity constraints honoured:**
- Unit 1 (atomic): BUG-03 + BUG-04 + BUG-05 → Phase 69
- Unit 2 (atomic): BUG-01 + BUG-02 + ERR-02 → Phase 71
- Unit 3 (atomic): ERR-01 + CLEAN-01 → Phase 70
- Unit 4 (after Unit 1): CLEAN-02 → Phase 72
- Unit 5: WATCH-01 + WATCH-02 → Phase 73
- Unit 6 (split): COLOR-01 + POLISH-03 → Phase 74; LAYOUT-02 + WATCH-03 + ICON-05 → Phase 75
- Unit 7 (split): NAME-01 + LAYOUT-01 → Phase 76; MODEL-03 + MODEL-06 → Phase 77
- Unit 8 (atomic): MODEL-02 + ACCESS-01 + ACCESS-02 → Phase 78
- Unit 9: BORDER-01 + CLEAN-03 + READER-02 → Phase 79
- Unit 10: MODEL-01 + VALID-01 + VALID-02 + BORDER-02 → Phase 80
- Unit 11 (atomic): FEATURE-01 + FEATURE-02 + FEATURE-03 → Phase 81

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
| 61-68 | v0.5.6 | 14/14 | Complete | 2026-04-10 |
| 69. Resolver-Level button_order Unlock | v0.5.7 | 2/2 | Complete   | 2026-04-12 |
| 70. Drop Error::Clone Bound | v0.5.7 | 1/1 | Complete   | 2026-04-12 |
| 71. Error Restructure and Validation Split | v0.5.7 | 2/2 | Complete   | 2026-04-12 |
| 72. ENV_MUTEX Test Simplification | v0.5.7 | 2/2 | Complete   | 2026-04-12 |
| 73. ThemeChangeEvent Cleanup | v0.5.7 | 1/1 | Complete   | 2026-04-12 |
| 74. Rgba Polish and must_use Uniformity | v0.5.7 | 2/2 | Complete   | 2026-04-12 |
| 75. LinuxDesktop non_exhaustive, Compile-Gated Watchers, IconSet::default Removal | v0.5.7 | 2/2 | Complete   | 2026-04-12 |
| 76. Type Vocabulary Rename and Crate Root Partition | v0.5.7 | 2/2 | Complete   | 2026-04-12 |
| 77. SystemTheme API and icon_set Relocation | v0.5.7 | 2/2 | Complete   | 2026-04-12 |
| 78. OverlaySource, AccessibilityPreferences, font_dpi Relocation | v0.5.7 | 4/4 | Complete   | 2026-04-13 |
| 79. BorderSpec Split and Platform Reader Visibility Audit | v0.5.7 | 2/2 | Complete   | 2026-04-13 |
| 80. native-theme-derive Proc-Macro K Codegen | v0.5.7 | 2/2 | Complete   | 2026-04-13 |
| 81. Feature-Matrix Cleanup and Unified from_system | v0.5.7 | 2/2 | Complete   | 2026-04-13 |
| 82. Icon API Rework | v0.5.7 | 2/2 | Complete   | 2026-04-13 |
| 83. Detection Cache Layer | v0.5.7 | 2/2 | Complete   | 2026-04-13 |
| 84. Reader Output Contract Homogenisation | v0.5.7 | 2/2 | Complete   | 2026-04-13 |
| 85. Data Model Method and Doc Cleanup | v0.5.7 | 2/2 | Complete   | 2026-04-13 |
| 86. Validation and Lint Codegen Polish | v0.5.7 | 2/2 | Complete   | 2026-04-13 |
| 87. Font Family Arc<str> and AnimatedIcon Invariants | v0.5.7 | 3/3 | Complete   | 2026-04-13 |
| 88. Diagnostic and Preset-Polish Sweep | v0.5.7 | 0/0 | Not started | — |
| 89. Fix Stale Doctest Paths and Dangling Doc Link | v0.5.7 | 1/1 | Complete | 2026-04-15 |
| 90. Remaining v0.5.7 API Overhaul Gaps | v0.5.7 | 6/6 | Complete | 2026-04-15 |
| 91. Resolve Remaining TODO Doc Gaps | v0.5.7 | 3/3 | Complete | 2026-04-15 |
| 92. Icon Theme Selection and Showcase Fixes | v0.5.7 | 4/4 | Complete | 2026-04-16 |
| 93. docs/todo_v0.5.7_gaps.md — P1 Polish Sweep (G1-G5, plus G3 follow-up + G11 deviation) | v0.5.7 | 7/7 | Complete | 2026-04-19 |
| 94. docs/todo_v0.5.7_gaps.md — Close Remaining G6-G8 (Border Inheritance Codegen, ResolutionContext, ThemeReader Trait) | v0.5.7 | 2/3 | In Progress | — |

### Phase 90: resolve remaining v0.5.7 API overhaul gaps

**Goal:** Resolve all remaining v0.5.7 API overhaul gaps from todo docs (12 issues across Rgba polish, ThemeWatcher rename, Theme method cleanup, inheritance drift test, intern_font_family, watch compile gates)
**Depends on:** Phase 89
**Status:** Complete (2026-04-15)
**Plans:** 6/6 plans complete

Plans:
- [x] 90-01-PLAN.md -- Rgba Default removal, rgba()->new() rename, detect.rs doc fix
- [x] 90-02-PLAN.md -- IconSet serde test, icon_theme doc, subprocess timeout const
- [x] 90-03-PLAN.md -- ThemeWatcher -> ThemeSubscription rename + constructor collapse
- [x] 90-04-PLAN.md -- Theme::new() deletion, pick/into_variant Result, preset cache doc
- [x] 90-05-PLAN.md -- intern_font_family helper with global dedup cache
- [x] 90-06-PLAN.md -- Inheritance drift test, kde pub doc, watch compile gates

Accepted deviation: Rgba retains Default via manual impl returning TRANSPARENT (30+ types require the bound). GAP-4a intent achieved.

### Phase 91: Resolve remaining TODO doc gaps: delete with_overlay_toml, add PresetInfo, codegen require() boilerplate, unify border-inheritance paths

**Goal:** Close remaining gaps from TODO doc audit: delete with_overlay_toml convenience wrapper (15b), add PresetInfo struct for richer preset enumeration (15f), replace hand-written defaults extraction boilerplate with declarative macros (B1), unify border-inheritance validation via struct-level border_kind dispatch (B7). Document from_kde_content_pure pub visibility as intentional deviation (C6).
**Depends on:** Phase 90
**Status:** complete (2026-04-15)
**Plans:** 3/3 plans complete

Plans:
- [x] 91-01-PLAN.md -- Delete with_overlay_toml, add PresetInfo struct
- [x] 91-02-PLAN.md -- Unify border validation dispatch via border_kind attribute
- [x] 91-03-PLAN.md -- Replace defaults extraction boilerplate with declarative macros

### Phase 92: implement the chosen solutions described in docs/todo_v0.5.7_icon-theme.md — completed 2026-04-16

**Goal:** Fix "system" icon theme selection reverting and add installed themes to dropdown (both showcases)
**Depends on:** Phase 91
**Plans:** 4/4 plans complete — verified 2026-04-16

Plans:
- [x] 92-01-PLAN.md -- Library: IconSetChoice enum, default_icon_choice(), list_freedesktop_themes()
- [x] 92-02-PLAN.md -- Iced showcase: import library type, guard rebuild_theme(), add installed themes
- [x] 92-03-PLAN.md -- GPUI showcase: replace boolean with library type, guard reapplication, add installed themes
- [x] 92-04-PLAN.md -- Full workspace verification and pre-release check

### Phase 93: docs/todo_v0.5.7_gaps.md — P1 Polish Sweep (G1-G5, plus G3 follow-up + G11 deviation)

**Goal:** Close P1 polish gaps G1-G5 from `docs/todo_v0.5.7_gaps.md` so the v0.5.7 public surface ships with zero half-baked or deferred items. Verifier pass added G3-follow-up (Plan 06), G11 principled deviation (Plan 07), and G11-followup release-gate bootstrap fix (Plan 08). Status: passed 2026-04-19.
**Depends on:** Phase 92
**Plans:** 9 plans (waves: 1=three parallel, 2=one, 3=one, 4=two parallel gap-closures, 5=release-gate bootstrap fix, 6=IconLoader typed-per-set refactor)

Plans:
- [x] 93-01-PLAN.md — G1: Remove `Rgba::Default` and break the `require<T:Default>` bound chain (wave 1)
- [x] 93-02-PLAN.md — G2: Add `LinuxDesktop::Wayfire` variant and route through adwaita+portal fallback (wave 1)
- [x] 93-03-PLAN.md — G3: Demote `bundled_icon_svg`/`bundled_icon_by_name`/`load_freedesktop_icon_by_name` to `pub(crate)` and migrate connector + showcase to `IconLoader` (wave 1)
- [x] 93-04-PLAN.md — G4: `icon_theme` on `Theme` with per-variant override; migrate 15 presets to top-level (wave 2, depends on 93-02 + 93-03)
- [x] 93-05-PLAN.md — G5: `#[derive(ThemeFields)]` for 7 non-widget structs + LayoutTheme; unify `lint_toml` over inventory registries (wave 3, depends on 93-01 + 93-03 + 93-04)
- [x] 93-06-PLAN.md — G3 follow-up: rewrite bundled.rs doctests to use `IconLoader` + add conditional `#[cfg_attr(not(any(material-icons, lucide-icons)), allow(dead_code))]` on `bundled_icon_by_name` (wave 4, depends on 93-01..05)
- [x] 93-07-PLAN.md — G11 principled deviation: document naga/--workspace incompatibility and realign Phase 93 acceptance with `./pre-release-check.sh` per-crate posture (wave 4, docs-only, disjoint files from 93-06)
- [x] 93-08-PLAN.md — G11-followup: pre-release-check.sh bootstrap fix (`--no-verify` on the three `cargo package` invocations) + RELEASING.md documenting ordered-publish workflow + VERIFICATION.md correction (status: passed) (wave 5, depends on 93-06 + 93-07)
- [x] 93-09-PLAN.md — G3 design-level followup: replace `IconLoader` with typed per-set loaders (`FreedesktopLoader`/`SfSymbolsLoader`/`SegoeIconsLoader`/`MaterialLoader`/`LucideLoader`) to eliminate the silent-ignore bug class exposed by Phase 93-03; `load_freedesktop_spinner` accepts `theme: Option<&str>`; `gnome_names_resolve_in_adwaita` regression fixed (152/0 vs prior 151/1) (wave 6, API-breaking, depends on 93-03 + 93-06 + 93-08)

### Phase 94: docs/todo_v0.5.7_gaps.md — Close Remaining G6-G8 (Border Inheritance Codegen, ResolutionContext, ThemeReader Trait)

**Goal:** Close the final three gap-doc items G6 (border/font inheritance codegen via `#[theme_inherit]`), G7 (first-class `ResolutionContext` struct replacing `font_dpi: Option<f32>` parameter threading), and G8 (`ThemeReader` trait for platform reader uniformity). Status: in progress.
**Depends on:** Phase 93
**Plans:** 3 plans (wave 1: 94-01 + 94-02 parallel; wave 2: 94-03 serial on 94-02's pipeline.rs shape)

Plans:
- [x] 94-01-PLAN.md — G6: Border + font inheritance codegen via `#[derive(ThemeWidget)] #[theme_inherit(...)]` with inverted-drift test (wave 1, completed 2026-04-20)
- [x] 94-02-PLAN.md — G7: `ResolutionContext` struct with `from_system()`/`for_tests()` + NO `Default` impl; `ThemeMode::into_resolved(&ctx)` replaces `Option<f32>`; `resolve_system()` zero-arg shortcut; 43 call sites migrated (wave 1, completed 2026-04-20)
- [ ] 94-03-PLAN.md — G8: `ThemeReader` trait unifying platform readers (KDE/GNOME/macOS/Windows) behind common interface (wave 2, depends on 94-02)
