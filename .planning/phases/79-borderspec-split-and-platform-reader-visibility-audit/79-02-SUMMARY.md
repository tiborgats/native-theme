---
phase: 79-borderspec-split-and-platform-reader-visibility-audit
plan: 02
subsystem: api
tags: [visibility, pub-crate, platform-readers, encapsulation]

# Dependency graph
requires:
  - phase: 78-overlaysource-accessibilitypreferences-font-dpi-relocation
    provides: "Reader functions returning (Theme, Option<f32>, AccessibilityPreferences) tuples"
provides:
  - "Platform reader I/O functions (from_kde, from_gnome, from_macos, from_windows) demoted to pub(crate)"
  - "from_kde_with_portal and build_gnome_spec_pure demoted to pub(crate)"
  - "from_kde_content_pure remains pub for integration test access"
affects: [phase-80, phase-81, phase-84]

# Tech tracking
tech-stack:
  added: []
  patterns: ["pub(crate) demotion after L3 external-consumer audit"]

key-files:
  created: []
  modified:
    - native-theme/src/kde/mod.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/src/macos.rs
    - native-theme/src/windows.rs

key-decisions:
  - "L3 audit confirmed zero external consumers of all 6 demoted functions"
  - "from_kde_content_pure stays pub -- used by native-theme/tests/reader_kde.rs"
  - "Module visibility (pub mod kde, pub mod gnome) unchanged -- required for from_kde_content_pure access"

patterns-established:
  - "Internal-use doc comment pattern: points to SystemTheme::from_system() for external callers"

requirements-completed: [READER-02]

# Metrics
duration: 5min
completed: 2026-04-13
---

# Phase 79 Plan 02: Platform Reader Visibility Audit Summary

**Demoted 6 platform reader I/O functions from pub to pub(crate) after L3 grep audit confirmed zero external consumers**

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-13T08:12:06Z
- **Completed:** 2026-04-13T08:17:15Z
- **Tasks:** 1
- **Files modified:** 4

## Accomplishments
- L3 audit (grep across connectors/, tests/, examples/) confirmed zero external references to any of the 6 I/O reader functions
- Confirmed from_kde_content_pure IS used by native-theme/tests/reader_kde.rs -- correctly kept as pub
- All 6 functions demoted: from_kde, from_gnome, from_macos, from_windows, from_kde_with_portal, build_gnome_spec_pure
- Added internal-use doc comments on each demoted function pointing to SystemTheme::from_system()

## Task Commits

Each task was committed atomically:

1. **Task 1: L3 visibility audit and platform reader demotion** - `8e26f59` (feat)

## Files Created/Modified
- `native-theme/src/kde/mod.rs` - from_kde: pub -> pub(crate), doc comment added
- `native-theme/src/gnome/mod.rs` - from_gnome, from_kde_with_portal, build_gnome_spec_pure: pub -> pub(crate), doc comments added
- `native-theme/src/macos.rs` - from_macos: pub -> pub(crate), doc comment added
- `native-theme/src/windows.rs` - from_windows: pub -> pub(crate), doc comment added

## Decisions Made
- L3 audit confirmed zero external consumers of all 6 demoted functions (only archive docs mention from_macos path -- not source code)
- from_kde_content_pure stays pub -- used by native-theme/tests/reader_kde.rs integration test
- Module visibility (pub mod kde, pub mod gnome) stays unchanged because from_kde_content_pure needs external access via the pub module path

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- Pre-existing uncommitted changes from Phase 79 Plan 01 (BorderSpec split in model/border.rs, model/defaults.rs, model/mod.rs, model/widgets/mod.rs) prevent full `cargo test` suite from compiling. Verified in isolation: stashing those model files and running `cargo check` with only the reader visibility changes passes cleanly. The reader demotion is purely additive-restrictive (pub -> pub(crate)) and cannot break any code that compiled before.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Phase 79 Plan 01 (BorderSpec split) must complete for the full crate to compile again
- Reader visibility is now correct for all downstream phases (80, 81, 84)
- External crates can only access SystemTheme::from_system() -- internal pipeline is properly encapsulated

---
*Phase: 79-borderspec-split-and-platform-reader-visibility-audit*
*Completed: 2026-04-13*
