---
phase: 78-overlaysource-accessibilitypreferences-font-dpi-relocation
plan: 04
subsystem: pipeline
tags: [accessibility, font-dpi, pipeline, kde, gnome, windows, macos]

# Dependency graph
requires:
  - phase: 78-01
    provides: AccessibilityPreferences on SystemTheme, OverlaySource, from_kde_content_pure tuple return
  - phase: 78-02
    provides: OverlaySource replaces pre-resolve variant fields
provides:
  - All four OS readers return (Theme, Option<f32>, AccessibilityPreferences) tuples
  - run_pipeline accepts and threads accessibility + font_dpi to SystemTheme
  - KDE/GNOME/Windows/macOS accessibility values flow end-to-end to SystemTheme.accessibility
  - font_dpi from readers flows to into_resolved() and OverlaySource
affects: [phase-79, phase-84, connectors]

# Tech tracking
tech-stack:
  added: []
  patterns: [accessibility_from_gnome_data helper for DRY accessibility extraction]

key-files:
  created: []
  modified:
    - native-theme/src/pipeline.rs
    - native-theme/src/kde/mod.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/src/windows.rs
    - native-theme/src/macos.rs
    - native-theme/src/lib.rs

key-decisions:
  - "accessibility_from_gnome_data() helper extracts AccessibilityPreferences from GnomePortalData using Default then overriding with real values"
  - "macOS uses 72.0 DPI as font_dpi (Apple points = logical pixels)"
  - "GNOME and Windows set reduce_transparency to default (false) since neither OS exposes this setting"
  - "from_kde_content returns outer font_dpi (I/O-detected) not inner pure parser dpi"

patterns-established:
  - "Reader tuple pattern: all OS readers return (Theme, Option<f32>, AccessibilityPreferences)"
  - "Pipeline destructure pattern: let (reader, dpi, acc) = reader_fn()?; run_pipeline(reader, preset, mode, acc, dpi)"
  - "Fallback path pattern: AccessibilityPreferences::default() and None for font_dpi when no OS reader data"

requirements-completed: [ACCESS-01, ACCESS-02]

# Metrics
duration: 10min
completed: 2026-04-13
---

# Phase 78 Plan 04: Wire OS Accessibility Values Through Pipeline Summary

**All four OS readers return accessibility tuples; run_pipeline threads real accessibility + font_dpi to SystemTheme end-to-end**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-13T01:48:30Z
- **Completed:** 2026-04-13T01:59:05Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- run_pipeline gains accessibility and font_dpi parameters, wired to SystemTheme.accessibility, into_resolved(), and OverlaySource
- KDE from_kde() and from_kde_content() return (Theme, Option<f32>, AccessibilityPreferences) with real OS values
- GNOME build_gnome_variant/build_theme/from_gnome/build_gnome_spec_pure/from_kde_with_portal all return accessibility tuples
- Windows build_theme/from_windows return tuple with AccessibilityData converted to AccessibilityPreferences
- macOS from_macos builds AccessibilityPreferences from read_accessibility() instead of writing removed ThemeDefaults fields
- All 42 native-theme tests pass; workspace compiles clean; pre-release-check passes

## Task Commits

Each task was committed atomically:

1. **Task 1: Change run_pipeline signature, wire KDE reader, update all call sites** - `250901a` (feat)
2. **Task 2: Wire GNOME, Windows, and macOS readers to return accessibility tuples** - `2a6e2a5` (feat)

## Files Created/Modified
- `native-theme/src/pipeline.rs` - run_pipeline gains 2 params; all call sites updated with real or default accessibility/dpi
- `native-theme/src/kde/mod.rs` - from_kde_content and from_kde return (Theme, Option<f32>, AccessibilityPreferences)
- `native-theme/src/gnome/mod.rs` - accessibility_from_gnome_data helper; build_gnome_variant/build_theme/from_gnome/build_gnome_spec_pure/from_kde_with_portal return tuples
- `native-theme/src/windows.rs` - build_theme returns tuple; AccessibilityData converted to AccessibilityPreferences
- `native-theme/src/macos.rs` - from_macos builds AccessibilityPreferences; removed writes to deleted ThemeDefaults fields
- `native-theme/src/lib.rs` - Updated overlay_tests run_pipeline call site

## Decisions Made
- Used `accessibility_from_gnome_data()` helper to DRY the AccessibilityPreferences extraction from GnomePortalData (used in both build_gnome_variant and build_gnome_spec_pure)
- macOS font_dpi set to `Some(72.0_f32)` -- Apple coordinate system uses 72 DPI base where 1pt = 1 logical pixel
- GNOME and Windows `reduce_transparency` defaults to false since neither platform exposes this setting
- `from_kde_content` returns the outer I/O-detected `font_dpi` (from full DPI detection chain), not the pure parser's resolved dpi, ensuring the complete detection chain is used

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated lib.rs overlay_tests call site**
- **Found during:** Task 1
- **Issue:** `pipeline::run_pipeline` call in `lib.rs` overlay_tests module was not listed in the plan but also needed the 2 new args
- **Fix:** Added `AccessibilityPreferences::default(), None` to the test call site
- **Files modified:** native-theme/src/lib.rs
- **Verification:** cargo test -p native-theme passes
- **Committed in:** 250901a (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Minor oversight in plan's call site enumeration. Fix was trivial and necessary for compilation.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 78 is now complete (all 4 plans executed)
- All OS reader accessibility data flows end-to-end through the pipeline to SystemTheme.accessibility
- font_dpi from readers reaches into_resolved() for correct font size resolution
- Ready for Phase 79 (border split + reader visibility audit)

---
*Phase: 78-overlaysource-accessibilitypreferences-font-dpi-relocation*
*Completed: 2026-04-13*
