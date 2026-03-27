---
phase: 47-os-first-pipeline
plan: 01
subsystem: api
tags: [system-theme, pipeline, resolve, validate, preset-merge]

# Dependency graph
requires:
  - phase: 45-resolution-engine
    provides: ThemeVariant::resolve() and validate() producing ResolvedTheme
  - phase: 46-os-reader-extensions
    provides: Platform readers (KDE, GNOME, macOS, Windows) returning NativeTheme
  - phase: 44-data-model-widget-structs
    provides: NativeTheme, ThemeVariant, merge(), preset()
provides:
  - SystemTheme struct with both light and dark ResolvedTheme variants
  - run_pipeline() orchestrating reader -> preset merge -> resolve -> validate
  - platform_preset_name() mapping OS/DE to preset name
  - reader_is_dark() inferring dark mode from reader output
  - from_system() and from_system_async() returning SystemTheme
affects: [47-02, 48-connectors]

# Tech tracking
tech-stack:
  added: []
  patterns: [os-first-pipeline, preset-merge-then-resolve, single-variant-to-dual-variant]

key-files:
  created: []
  modified:
    - native-theme/src/lib.rs

key-decisions:
  - "SystemTheme defined in lib.rs (not separate module) -- tightly coupled to from_system entry points"
  - "Pre-resolve ThemeVariant retained as pub(crate) fields for Plan 02 overlay support"
  - "reader_is_dark() used for macOS/Windows is_dark inference; Linux uses existing system_is_dark()"
  - "GNOME double-merge (reader already merged with adwaita + pipeline merges again) validated harmless via test"

patterns-established:
  - "OS-first pipeline: reader output + preset merge + resolve + validate = SystemTheme"
  - "Single-variant platforms fill inactive variant from preset-only"

requirements-completed: [PIPE-01, PIPE-02]

# Metrics
duration: 5min
completed: 2026-03-27
---

# Phase 47 Plan 01: SystemTheme Pipeline Summary

**SystemTheme struct with OS-first pipeline wiring: from_system() returns both resolved light/dark variants via reader -> preset merge -> resolve -> validate**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-27T13:58:45Z
- **Completed:** 2026-03-27T14:04:18Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- SystemTheme struct with active()/pick() convenience methods for resolved variant access
- run_pipeline() merges reader output onto platform preset, resolves and validates both variants
- from_system(), from_system_async(), from_linux() all return SystemTheme instead of raw NativeTheme
- Platform preset mapping: macOS->macos-sonoma, Windows->windows-11, KDE->kde-breeze, GNOME/other->adwaita
- 13 new tests covering SystemTheme, pipeline, preset mapping, reader_is_dark, and GNOME double-merge

## Task Commits

Each task was committed atomically:

1. **Task 1: Define SystemTheme + pipeline internals + platform preset mapping** (TDD)
   - `b6f7505` (test: failing tests for SystemTheme, run_pipeline, platform_preset_name)
   - `8297753` (feat: implement SystemTheme, run_pipeline, platform_preset_name)
2. **Task 2: Rewire from_system/from_system_async/from_linux to return SystemTheme**
   - `04da152` (feat: rewire entry points to return SystemTheme)

## Files Created/Modified
- `native-theme/src/lib.rs` - Added SystemTheme struct, run_pipeline(), platform_preset_name(), resolve_variant(), reader_is_dark(); rewired from_system/from_system_async/from_linux to return SystemTheme; added 13 tests

## Decisions Made
- SystemTheme defined in lib.rs rather than a separate module since it is tightly coupled to the from_system entry points
- Pre-resolve ThemeVariant stored as pub(crate) fields (light_variant, dark_variant) for Plan 02's with_overlay() support
- reader_is_dark() returns true only when dark is Some and light is None; defaults to false (light) when both variants present (macOS case)
- GNOME double-merge confirmed harmless via dedicated test: adwaita used as both reader and preset produces identical results

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- SystemTheme is ready for Plan 02 (app TOML overlay with with_overlay()) which will use the retained pre-resolve ThemeVariant fields
- Phase 48 (connectors) can consume &ResolvedTheme from SystemTheme.active() or SystemTheme.pick()
- All 368 tests pass (355 pre-existing + 13 new)

## Self-Check: PASSED

- All created/modified files exist
- All commit hashes verified: b6f7505, 8297753, 04da152

---
*Phase: 47-os-first-pipeline*
*Completed: 2026-03-27*
