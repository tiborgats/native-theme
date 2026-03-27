---
phase: 46-os-reader-extensions
plan: 06
subsystem: testing
tags: [resolve, validate, integration-tests, macos, windows, gnome, pipeline-coverage]

# Dependency graph
requires:
  - phase: 45-resolution-engine
    provides: "resolve() and validate() pipeline"
  - phase: 46-os-reader-extensions
    provides: "macOS/Windows/GNOME reader build_theme() functions and presets"
provides:
  - "Integration tests proving all four OS readers pass resolve/validate"
  - "SC5 full coverage: KDE + macOS + Windows + GNOME"
affects: [47-connector-migration, 48-public-api]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Preset-merge integration test pattern for OS readers without live OS APIs"]

key-files:
  created: []
  modified:
    - native-theme/src/macos.rs
    - native-theme/src/windows.rs
    - native-theme/src/resolve.rs

key-decisions:
  - "GNOME test placed in resolve.rs (not gnome/mod.rs) because portal feature gate has pre-existing zbus compilation failure"
  - "Tests use preset merge pattern (reader output merged on top of OS-specific preset) matching production pipeline"

patterns-established:
  - "OS reader integration test pattern: load preset, build reader output, merge, resolve, validate, spot-check"

requirements-completed: [MACOS-01, MACOS-02, MACOS-03, MACOS-04, MACOS-05, WIN-01, WIN-02, WIN-03, WIN-04, WIN-05, GNOME-01, GNOME-02, GNOME-03, GNOME-04, GNOME-05]

# Metrics
duration: 5min
completed: 2026-03-27
---

# Phase 46 Plan 06: Resolve/Validate Integration Tests Summary

**Three integration tests proving macOS, Windows, and GNOME reader outputs pass the full resolve/validate pipeline, completing SC5 coverage for all four OS readers**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-27T13:02:58Z
- **Completed:** 2026-03-27T13:08:00Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- test_macos_resolve_validate: build_theme() with sample data merged on macos-sonoma preset passes resolve/validate for both light and dark variants
- test_windows_resolve_validate: light_theme() merged on windows-11 preset passes resolve/validate with spot-checked accent, font, dialog button order, and icon set
- test_gnome_resolve_validate: adwaita preset with simulated GNOME reader fields (font, button_order, icon_set) passes resolve/validate pipeline
- SC5 fully satisfied: all four OS readers (KDE, macOS, Windows, GNOME) now have integration tests proving resolve/validate produces Ok(ResolvedTheme)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add macOS and Windows resolve/validate integration tests** - `bd7ee37` (test)
2. **Task 2: Add GNOME resolve/validate integration test in resolve.rs** - `2776a04` (test)

## Files Created/Modified
- `native-theme/src/macos.rs` - Added test_macos_resolve_validate (light + dark variant coverage)
- `native-theme/src/windows.rs` - Added test_windows_resolve_validate (light variant coverage)
- `native-theme/src/resolve.rs` - Added test_gnome_resolve_validate (simulated GNOME reader output)

## Decisions Made
- GNOME test placed in resolve.rs instead of gnome/mod.rs because the portal feature gate has a pre-existing zbus compilation failure that prevents adding tests in the GNOME module on non-GNOME systems
- Tests follow the same preset-merge pattern as the KDE integration test: load OS-specific preset as base, build reader output, merge, resolve, validate, spot-check key fields

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All OS reader integration tests complete, SC5 fully satisfied
- Phase 46 ready for verification: all 6 plans complete
- Ready for Phase 47 (connector migration) or Phase 48 (public API)

## Self-Check: PASSED

- All 3 modified files exist on disk
- Both task commits (bd7ee37, 2776a04) verified in git log
- No stubs or placeholder values found

---
*Phase: 46-os-reader-extensions*
*Completed: 2026-03-27*
