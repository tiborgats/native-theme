---
phase: 64-cross-platform-reader-test-separation
plan: 01
subsystem: testing
tags: [gnome, portal, pure-function, cross-platform, inline-tests]

# Dependency graph
requires:
  - phase: 63-kde-reader-fixture-tests
    provides: pattern for pure function extraction from platform readers
provides:
  - GnomePortalData struct with primitive types for deterministic testing
  - build_gnome_spec_pure() pub fn for GNOME theme testing without D-Bus
  - build_gnome_variant_pure() private zero-I/O variant builder
  - 10 inline tests exercising pure GNOME theme assembly
affects: [65-runtime-theme-change, gnome-reader-tests]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Pure function extraction: separate I/O (gsettings, xrdb) from theme assembly logic"
    - "Primitive-typed data struct as bridge between I/O and pure logic"

key-files:
  created: []
  modified:
    - native-theme/src/gnome/mod.rs

key-decisions:
  - "GnomePortalData.is_dark field unused by build_gnome_variant_pure -- only consumed by build_gnome_spec_pure for variant selection"
  - "build_gnome_variant delegates to build_gnome_variant_pure after converting ashpd types to primitives and reading gsettings"
  - "Windows (TEST-04) and macOS (TEST-05) readers already satisfy cross-platform testability -- no code changes needed"

patterns-established:
  - "Pure extraction pattern: I/O wrapper reads data -> primitive struct -> pure builder -> ThemeVariant"

requirements-completed: [TEST-03, TEST-04, TEST-05]

# Metrics
duration: 5min
completed: 2026-04-09
---

# Phase 64 Plan 01: Cross-Platform Reader Test Separation Summary

**Extracted GnomePortalData struct and build_gnome_spec_pure() for deterministic GNOME theme testing without D-Bus/gsettings**

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-09T17:10:24Z
- **Completed:** 2026-04-09T17:15:31Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Created GnomePortalData struct with primitive types only (no ashpd/D-Bus types)
- Extracted build_gnome_variant_pure() as private zero-I/O variant builder
- Added build_gnome_spec_pure() as pub testable entry point for full GNOME ThemeSpec
- Refactored build_gnome_variant() to delegate to build_gnome_variant_pure()
- Added 10 new inline tests exercising pure function with constructed data
- Confirmed Windows and macOS readers already satisfy TEST-04 and TEST-05 (no changes needed)

## Task Commits

Each task was committed atomically:

1. **Task 1: Create GnomePortalData struct and build_gnome_spec_pure function** - `63ce336` (feat)
2. **Task 2: Add inline tests for build_gnome_spec_pure** - `921507d` (test)

## Files Created/Modified
- `native-theme/src/gnome/mod.rs` - Added GnomePortalData struct, build_gnome_variant_pure(), build_gnome_spec_pure(), refactored build_gnome_variant() delegation, 10 new pure_ tests

## Decisions Made
- GnomePortalData.is_dark is set to false in build_gnome_variant (unused by pure variant builder, only used by build_gnome_spec_pure for light/dark selection)
- gsettings string parsing ("true"/"false") moved from inline match arms to Option<bool> conversion in the I/O wrapper, keeping the pure function free of string parsing

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- Pre-existing test failure: `build_gnome_variant_normal_contrast_no_flag` fails on this machine because gsettings returns "false" for high-contrast, causing the gsettings fallback to set high_contrast=Some(false). Verified this failure exists on the original code (before refactoring) via git stash. This is an environment-dependent test issue, not caused by this plan's changes. Logged to deferred-items.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- GNOME pure function extraction complete, ready for runtime theme change work (Phase 65)
- Pattern established: I/O wrapper -> primitive struct -> pure builder
- All three platform readers (GNOME, Windows, macOS) now have deterministic testable entry points

## Self-Check: PASSED

- All files exist
- All commits verified (63ce336, 921507d)
- GnomePortalData struct, build_gnome_spec_pure, build_gnome_variant_pure all present
- 10 pure_ test functions found

---
*Phase: 64-cross-platform-reader-test-separation*
*Completed: 2026-04-09*
