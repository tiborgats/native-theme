---
phase: 10-api-breaking-changes
plan: 01
subsystem: api
tags: [serde, toml, theme-colors, flat-struct, breaking-change]

# Dependency graph
requires:
  - phase: 09-cargo-workspace
    provides: Cargo workspace structure with native-theme crate
provides:
  - Flat ThemeColors with 36 direct Option<Rgba> fields
  - Flat TOML preset format ([light.colors] / [dark.colors])
  - Platform readers using flat field construction
affects: [10-api-breaking-changes remaining plans, connectors]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Flat ThemeColors struct with skip_serializing_none for clean TOML output"
    - "All color roles as direct fields (no nested sub-structs)"

key-files:
  created: []
  modified:
    - native-theme/src/model/colors.rs
    - native-theme/src/model/mod.rs
    - native-theme/src/lib.rs
    - native-theme/src/kde/colors.rs
    - native-theme/src/kde/mod.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/src/windows.rs
    - native-theme/src/presets.rs
    - native-theme/src/presets/*.toml (all 17)
    - native-theme/tests/serde_roundtrip.rs
    - native-theme/tests/merge_behavior.rs
    - native-theme/tests/preset_loading.rs
    - native-theme/README.md

key-decisions:
  - "Flat struct uses option{} macro variant for all 36 fields (no nested{} merge)"
  - "Primary/secondary fields renamed to primary_background, primary_foreground, etc. to avoid collisions"
  - "README updated inline with code changes (deviation Rule 3 - blocking doctest)"

patterns-established:
  - "ThemeColors access pattern: colors.accent instead of colors.core.accent"
  - "TOML preset format: single [light.colors] section instead of sub-tables"

requirements-completed: [API-02, API-03, API-04]

# Metrics
duration: 8min
completed: 2026-03-08
---

# Phase 10 Plan 01: Flatten ThemeColors Summary

**Flat ThemeColors struct with 36 direct Option<Rgba> fields, 17 migrated TOML presets, and all platform readers updated**

## Performance

- **Duration:** 8 min
- **Started:** 2026-03-08T05:23:52Z
- **Completed:** 2026-03-08T05:31:40Z
- **Tasks:** 2
- **Files modified:** 29

## Accomplishments
- Replaced 6 nested sub-structs (CoreColors, ActionColors, StatusColors, InteractiveColors, PanelColors, ComponentColors) with a single flat ThemeColors containing 36 direct Option<Rgba> fields
- Migrated all 17 TOML preset files from nested [light.colors.core] format to flat [light.colors] format
- Updated KDE, GNOME, and Windows platform readers to use flat field construction
- All 175 tests pass (136 unit + 30 integration + 9 doctests)

## Task Commits

Each task was committed atomically:

1. **Task 1: Flatten ThemeColors struct and update impl_merge** - `62eedb0` (feat)
2. **Task 2: Migrate TOML presets, platform readers, and all tests** - `579155f` (feat)

## Files Created/Modified
- `native-theme/src/model/colors.rs` - Flat ThemeColors with 36 Option<Rgba> fields, removed 6 sub-structs
- `native-theme/src/model/mod.rs` - Changed export to ThemeColors only, updated test references
- `native-theme/src/lib.rs` - Removed sub-struct names from pub use statement
- `native-theme/src/kde/colors.rs` - Flat ThemeColors construction from KDE INI
- `native-theme/src/kde/mod.rs` - Fixed test references to flat access
- `native-theme/src/gnome/mod.rs` - Flat field access in apply_accent and tests
- `native-theme/src/windows.rs` - Flat field access in build_theme and tests
- `native-theme/src/presets.rs` - Updated docstrings and test TOML literals
- `native-theme/src/presets/*.toml` (17 files) - Flat [light.colors]/[dark.colors] format
- `native-theme/tests/serde_roundtrip.rs` - All assertions use flat access
- `native-theme/tests/merge_behavior.rs` - Removed sub-struct references, flat access
- `native-theme/tests/preset_loading.rs` - Flat access for all preset checks
- `native-theme/README.md` - Updated API examples and TOML format reference

## Decisions Made
- Used `option{}` macro variant for all 36 fields in impl_merge (since there are no more nested structs in ThemeColors)
- Renamed `primary.background`/`primary.foreground` to `primary_background`/`primary_foreground` (and similarly for secondary) to avoid TOML key collisions with core `background`/`foreground`
- Updated README inline rather than deferring, since failing doctests were blocking

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed README doctest references to nested color paths**
- **Found during:** Task 2 verification (`cargo test`)
- **Issue:** README.md doctests still used `colors.core.accent` and `colors.core.background`
- **Fix:** Updated all README code examples and TOML format reference to flat access
- **Files modified:** native-theme/README.md
- **Verification:** `cargo test --doc` passes
- **Committed in:** 579155f (Task 2 commit)

**2. [Rule 3 - Blocking] Fixed kde/mod.rs test references to nested color paths**
- **Found during:** Task 2 verification (`cargo test --features kde`)
- **Issue:** kde/mod.rs integration tests still used `colors.core.accent` etc.
- **Fix:** Updated 3 assertions to flat field access
- **Files modified:** native-theme/src/kde/mod.rs
- **Verification:** `cargo test --features kde` passes
- **Committed in:** 579155f (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both auto-fixes necessary for test suite to pass. No scope creep.

## Issues Encountered
None beyond the auto-fixed deviations above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Flat ThemeColors is the foundation for all subsequent v0.2 API changes
- No references to old sub-struct types remain in src/ or tests/
- Ready for next plan in phase 10

---
*Phase: 10-api-breaking-changes*
*Completed: 2026-03-08*
