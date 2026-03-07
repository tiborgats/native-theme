---
phase: 01-data-model-foundation
plan: 02
subsystem: core
tags: [rust, serde, toml, theme-model, merge-macro, non-exhaustive]

# Dependency graph
requires:
  - "01-01: Rgba color type, Error enum, impl_merge! macro"
provides:
  - "ThemeColors with 36 semantic color roles across 7 nested sub-structs"
  - "ThemeFonts (4 fields), ThemeGeometry (5 fields), ThemeSpacing (7 fields)"
  - "ThemeVariant composing all 4 sub-structs with recursive merge"
  - "NativeTheme with name + optional light/dark ThemeVariants"
  - "All types re-exported from crate root"
affects: [01-03, 02-presets, 03-kde-reader, 04-portal-reader, 05-windows-reader, 06-gnome-reader]

# Tech tracking
tech-stack:
  added: []
  patterns: [nested-sub-struct-colors, skip-serializing-if-is-empty, manual-merge-for-nativetheme]

key-files:
  created:
    - src/model/colors.rs
    - src/model/fonts.rs
    - src/model/geometry.rs
    - src/model/spacing.rs
    - src/model/mod.rs
  modified:
    - src/lib.rs

key-decisions:
  - "PanelColors name chosen over SurfaceColors to avoid collision with CoreColors.surface field"
  - "ActionColors reused for both primary and secondary to avoid duplicate struct definitions"
  - "ThemeColors uses skip_serializing_if is_empty on nested fields (not skip_serializing_none) since fields are not Option"
  - "NativeTheme merge keeps base name, uses manual impl (not macro) for special variant merging semantics"
  - "No Eq derive on ThemeFonts/ThemeGeometry/ThemeSpacing (f32 prevents it); PartialEq is sufficient"

patterns-established:
  - "Color sub-structs use impl_merge! option category for flat Option<Rgba> fields"
  - "Container structs (ThemeColors, ThemeVariant) use impl_merge! nested category for recursive merge"
  - "skip_serializing_if = is_empty pattern for omitting empty TOML sections"
  - "serde_with::skip_serializing_none on all flat Option-field structs"

requirements-completed: [CORE-02, CORE-03, CORE-04, CORE-05, CORE-06, CORE-07, CORE-08, CORE-09, CORE-10, SERDE-02]

# Metrics
duration: 3min
completed: 2026-03-07
---

# Phase 1 Plan 02: Theme Model Structs Summary

**Complete theme type system: 36-role ThemeColors with 6 nested sub-structs, ThemeFonts/ThemeGeometry/ThemeSpacing, ThemeVariant composition, and NativeTheme with light/dark variant merging**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-07T15:16:27Z
- **Completed:** 2026-03-07T15:20:15Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- 7 color structs (CoreColors, ActionColors, StatusColors, InteractiveColors, PanelColors, ComponentColors, ThemeColors) with 36 total semantic color roles, all using impl_merge! for merge/is_empty
- ThemeFonts (4 fields), ThemeGeometry (5 fields), ThemeSpacing (7 fields) as flat structs with impl_merge! option category
- ThemeVariant composing all 4 sub-structs with recursive merge via impl_merge! nested category
- NativeTheme with name + optional light/dark ThemeVariants, manual merge impl that preserves base name and recursively merges matching variants
- 44 new tests (76 total across crate): is_empty, merge overlay/preserve, serde TOML round-trips, field count validation
- All types re-exported from crate root for ergonomic access

## Task Commits

Each task was committed atomically:

1. **Task 1 (RED): ThemeColors failing tests** - `55718ff` (test)
2. **Task 1 (GREEN): ThemeColors impl_merge! invocations** - `64ba569` (feat)
3. **Task 2: ThemeFonts, ThemeGeometry, ThemeSpacing, ThemeVariant, NativeTheme + lib.rs wiring** - `8ffc050` (feat)

## Files Created/Modified

- `src/model/colors.rs` - 7 color structs with 36 semantic roles, impl_merge!, serde annotations, 15 tests
- `src/model/fonts.rs` - ThemeFonts with 4 Option fields (family, size, mono_family, mono_size), 6 tests
- `src/model/geometry.rs` - ThemeGeometry with 5 Option<f32> fields (radius, frame_width, opacities, scroll_width), 5 tests
- `src/model/spacing.rs` - ThemeSpacing with 7 Option<f32> fields (xxs through xxl), 5 tests
- `src/model/mod.rs` - ThemeVariant (4 nested sub-structs), NativeTheme (name + light/dark), re-exports, 13 tests
- `src/lib.rs` - Uncommented pub mod model, added 12 type re-exports

## Decisions Made

- Used PanelColors (not SurfaceColors) to avoid naming collision with CoreColors.surface field
- ActionColors reused for both primary and secondary fields in ThemeColors (same struct, different instances)
- ThemeColors nested fields use `skip_serializing_if = "SubStruct::is_empty"` instead of skip_serializing_none (fields are not Option<T>)
- NativeTheme.merge() is manually implemented rather than via macro because name handling requires special logic (keep base name, not overlay)
- f32 fields in ThemeFonts/ThemeGeometry/ThemeSpacing derive PartialEq but not Eq (f32 incompatible with Eq)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Complete theme type system ready for Plan 03 (serde round-trip integration tests)
- All 12 public types re-exported from crate root: NativeTheme, ThemeVariant, ThemeColors, CoreColors, ActionColors, StatusColors, InteractiveColors, PanelColors, ComponentColors, ThemeFonts, ThemeGeometry, ThemeSpacing
- impl_merge! macro proven on all struct types (option + nested categories)
- 76 tests passing, cargo doc clean

## Self-Check: PASSED

- All 6 source files exist (colors.rs, fonts.rs, geometry.rs, spacing.rs, mod.rs, lib.rs)
- All 3 commits verified (55718ff, 64ba569, 8ffc050)
- SUMMARY.md created at expected path

---
*Phase: 01-data-model-foundation*
*Completed: 2026-03-07*
