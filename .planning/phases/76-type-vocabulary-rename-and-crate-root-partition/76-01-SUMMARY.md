---
phase: 76-type-vocabulary-rename-and-crate-root-partition
plan: 01
subsystem: api
tags: [rename, type-vocabulary, breaking-change, NAME-01]

# Dependency graph
requires:
  - phase: 75-non-exhaustive-compile-gate-iconset-default
    provides: "stable crate with non_exhaustive enums and IconSet::default removal"
provides:
  - "Theme (was ThemeSpec) -- top-level theme specification struct"
  - "ThemeMode (was ThemeVariant) -- single light/dark mode struct"
  - "ResolvedTheme (was ResolvedThemeVariant) -- fully resolved output struct"
  - "ResolvedDefaults (was ResolvedThemeDefaults) -- resolved defaults struct"
  - "GpuiTheme/GpuiThemeMode aliases in gpui connector for collision avoidance"
affects: [76-02, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88]

# Tech tracking
tech-stack:
  added: []
  patterns: ["GpuiTheme/GpuiThemeMode aliasing pattern to avoid name collisions between native-theme and gpui-component re-exports"]

key-files:
  created: []
  modified:
    - native-theme/src/model/mod.rs
    - native-theme/src/model/resolved.rs
    - native-theme/src/lib.rs
    - connectors/native-theme-gpui/src/lib.rs
    - connectors/native-theme-gpui/src/config.rs
    - connectors/native-theme-iced/src/lib.rs

key-decisions:
  - "gpui connector aliases gpui_component::{Theme,ThemeMode} as GpuiTheme/GpuiThemeMode to avoid collision with native_theme re-exports"
  - "Showcase examples use fully qualified native_theme::Theme where ambiguous with framework Theme types"

patterns-established:
  - "GpuiTheme/GpuiThemeMode alias pattern: when re-exporting native_theme types that collide with gpui_component names, alias the gpui_component imports"

requirements-completed: [NAME-01]

# Metrics
duration: 12min
completed: 2026-04-12
---

# Phase 76 Plan 01: Type Vocabulary Rename Summary

**Atomic rename of four core types (ThemeSpec->Theme, ThemeVariant->ThemeMode, ResolvedThemeVariant->ResolvedTheme, ResolvedThemeDefaults->ResolvedDefaults) across 40 files in core crate and both connectors, with gpui name collision resolution via aliasing**

## Performance

- **Duration:** 12 min
- **Started:** 2026-04-12T17:02:21Z
- **Completed:** 2026-04-12T17:14:39Z
- **Tasks:** 2/2
- **Files modified:** 40

## Accomplishments
- Renamed all four core types per NAME-01 specification across the entire workspace (484 source-level replacements in .rs files, plus README updates)
- Resolved the gpui connector's Theme/ThemeMode name collision by aliasing gpui_component imports as GpuiTheme/GpuiThemeMode
- Resolved showcase example ambiguities in both gpui and iced examples by qualifying native_theme::Theme where iced::Theme or gpui_component::theme::Theme was also in scope
- All 39 core crate tests, all gpui connector tests, and all iced connector tests pass with the new names

## Task Commits

Each task was committed atomically:

1. **Task 1: Rename types in core crate** - `5c279aa` (refactor)
2. **Task 2: Rename types in connector crates** - `8126ea1` (refactor)

## Files Created/Modified

### Core crate (29 files)
- `native-theme/src/model/mod.rs` - ThemeSpec->Theme, ThemeVariant->ThemeMode struct defs and impls
- `native-theme/src/model/resolved.rs` - ResolvedThemeVariant->ResolvedTheme, ResolvedThemeDefaults->ResolvedDefaults
- `native-theme/src/model/defaults.rs` - Doc comment reference update
- `native-theme/src/model/widgets/mod.rs` - ThemeVariant reference in comment
- `native-theme/src/lib.rs` - Re-exports, SystemTheme field types, test modules
- `native-theme/src/pipeline.rs` - Function signatures and imports
- `native-theme/src/resolve/{mod,inheritance,validate,validate_helpers,tests}.rs` - All resolve engine references
- `native-theme/src/detect.rs` - Doc comment updates
- `native-theme/src/error.rs` - Error variant reference
- `native-theme/src/icons.rs` - No changes needed (no references)
- `native-theme/src/{gnome/mod,kde/mod,kde/colors,kde/fonts,kde/metrics,macos,windows}.rs` - Platform reader function signatures
- `native-theme/src/presets.rs` - Preset loading references
- `native-theme/tests/{merge_behavior,platform_facts_xref,preset_loading,serde_roundtrip,proptest_roundtrip,resolve_and_validate}.rs` - Integration test updates
- `native-theme/README.md` - Code examples and prose

### Connector crates (11 files)
- `connectors/native-theme-gpui/src/lib.rs` - Re-exports, GpuiTheme/GpuiThemeMode aliases, function signatures
- `connectors/native-theme-gpui/src/config.rs` - GpuiThemeMode alias, function signatures, tests
- `connectors/native-theme-gpui/src/colors.rs` - Import and doc comment updates
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - Qualified native_theme::Theme for ambiguity
- `connectors/native-theme-gpui/README.md` - Type name updates
- `connectors/native-theme-iced/src/lib.rs` - Re-exports, function signatures
- `connectors/native-theme-iced/src/extended.rs` - Import and test updates
- `connectors/native-theme-iced/src/palette.rs` - Import and test updates
- `connectors/native-theme-iced/tests/integration.rs` - Import and test updates
- `connectors/native-theme-iced/examples/showcase-iced.rs` - Qualified native_theme::Theme for ambiguity
- `connectors/native-theme-iced/README.md` - Type name updates

## Decisions Made
- **gpui aliasing strategy:** Aliased `gpui_component::theme::{Theme, ThemeMode}` as `GpuiTheme`/`GpuiThemeMode` rather than qualifying native_theme types, since the connector's public API re-exports `native_theme::Theme` and `native_theme::ThemeMode` as the canonical names for downstream crates
- **Example disambiguation:** Used fully qualified `native_theme::Theme::preset(...)` in showcase examples where both framework Theme and native_theme Theme were in scope, rather than introducing additional aliases

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Resolved Theme name collision in showcase examples**
- **Found during:** Task 2 (connector compilation)
- **Issue:** Both showcase-gpui.rs and showcase-iced.rs import their framework's Theme type AND native_theme::Theme, creating ambiguity after the rename
- **Fix:** Removed bare Theme from native_theme imports in examples, qualified native_theme::Theme:: calls explicitly
- **Files modified:** connectors/native-theme-gpui/examples/showcase-gpui.rs, connectors/native-theme-iced/examples/showcase-iced.rs
- **Verification:** Both examples compile successfully
- **Committed in:** 8126ea1 (Task 2 commit)

**2. [Rule 3 - Blocking] Fixed presets/README.md missed by initial rename**
- **Found during:** Task 1 (post-rename grep verification)
- **Issue:** native-theme/src/presets/README.md contained a ThemeSpec reference not caught by the initial sed
- **Fix:** Applied the same sed replacement to presets/README.md
- **Files modified:** native-theme/src/presets/README.md
- **Verification:** grep confirms no old names remain
- **Committed in:** 5c279aa (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both auto-fixes necessary for compilation. No scope creep.

## Issues Encountered
- Pre-existing test failure `breeze_dark_fixture_colors_and_fonts` in reader_kde.rs (PrimaryLeft assertion) -- confirmed to exist before rename, not introduced by this plan
- `cargo test --workspace --all-features` fails due to upstream ashpd async-io/tokio feature conflict and naga crate build issue -- tested each crate individually instead

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Type vocabulary is now clean: Theme, ThemeMode, ResolvedTheme, ResolvedDefaults
- Ready for Plan 76-02 (crate root partition / lib.rs restructuring)
- All downstream phases can reference the new type names

---
*Phase: 76-type-vocabulary-rename-and-crate-root-partition*
*Completed: 2026-04-12*
