---
phase: 16-icon-data-model
plan: 01
subsystem: model
tags: [enums, icons, icon-role, icon-data, icon-set, non-exhaustive]

# Dependency graph
requires: []
provides:
  - "IconRole enum with 42 semantic variants and ALL const array"
  - "IconData enum with Svg and Rgba variants"
  - "IconSet enum with 5 variants and from_name/name conversion"
  - "Crate-root re-exports for all three icon types"
affects: [17-bundled-svgs, 18-platform-loaders, 19-icon-mappings, 20-connectors, 21-icon-integration]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Flat prefixed enum naming for IconRole (DialogWarning, ActionCopy, etc.)"
    - "Owned Vec<u8> for binary icon data (no lifetime infection)"
    - "Kebab-case from_name/name conversion pattern for TOML strings"

key-files:
  created:
    - "native-theme/src/model/icons.rs"
  modified:
    - "Cargo.toml"
    - "native-theme/src/model/mod.rs"
    - "native-theme/src/lib.rs"

key-decisions:
  - "No serde on IconRole -- it is a runtime enum, not serialized to TOML"
  - "Owned Vec<u8> in IconData -- avoids lifetime parameter infection"
  - "IconSet uses serde derives for TOML serialization support"
  - "Fixed workspace version mismatch (0.2.0 -> 0.3.0) as prerequisite"

patterns-established:
  - "Flat prefixed enum: category prefix IS the grouping (no nested sub-enums)"
  - "ALL const array: canonical list of all enum variants for iteration"
  - "from_name/name: kebab-case string conversion for TOML-facing enums"

requirements-completed: [ICON-01, ICON-02]

# Metrics
duration: 3min
completed: 2026-03-09
---

# Phase 16 Plan 01: Icon Type Definitions Summary

**IconRole (42 variants), IconData (Svg/Rgba), and IconSet (5 icon sets) enums with TDD, const ALL array, and kebab-case from_name/name conversion**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-09T06:36:55Z
- **Completed:** 2026-03-09T06:39:55Z
- **Tasks:** 4
- **Files modified:** 4

## Accomplishments
- Defined IconRole enum with exactly 42 variants across 7 categories, all accessible via const ALL array
- Defined IconData enum with Svg(Vec<u8>) and Rgba { width, height, data } variants for cross-platform icon data
- Defined IconSet enum with 5 platform icon sets and bidirectional from_name/name kebab-case conversion
- All types re-exported at crate root (native_theme::IconRole, etc.)
- Fixed workspace Cargo.toml version mismatch blocking test execution

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix workspace version mismatch** - `b7aa114` (fix)
2. **Task 2: TDD RED -- failing tests** - `ed2c16a` (test)
3. **Task 3: TDD GREEN -- implement enums** - `bb430a9` (feat)
4. **Task 4: Wire up re-exports** - `ac81768` (feat)

## Files Created/Modified
- `native-theme/src/model/icons.rs` - New module with IconRole (42 variants), IconData, IconSet enums + 27 unit tests
- `native-theme/src/model/mod.rs` - Added pub mod icons and pub use re-exports
- `native-theme/src/lib.rs` - Added IconData, IconRole, IconSet to crate root re-exports
- `Cargo.toml` - Fixed workspace dependency version from 0.2.0 to 0.3.0

## Decisions Made
- No serde derives on IconRole -- it is used for runtime lookups, not serialized to/from TOML
- Owned Vec<u8> for IconData (not Cow) -- avoids lifetime parameter that would infect all consuming types
- IconSet gets Serialize/Deserialize since it appears in TOML configuration
- Fixed workspace version mismatch as prerequisite since tests could not run otherwise

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed workspace version mismatch**
- **Found during:** Task 1 (prerequisite check)
- **Issue:** Root Cargo.toml had `native-theme = { version = "0.2.0" }` but crate was at 0.3.0; cargo resolve failed
- **Fix:** Updated workspace dependency to version = "0.3.0"
- **Files modified:** Cargo.toml
- **Verification:** `cargo check -p native-theme` succeeds
- **Committed in:** b7aa114

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Fix was explicitly called out in the plan's implementation section as a prerequisite. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All three icon types are defined and re-exported at crate root
- Ready for Plan 02 (icon_name mapping, system_icon_set, ThemeVariant icon_theme field)
- IconRole::ALL provides the canonical variant list for exhaustive mapping tables

## Self-Check: PASSED

All files exist, all commits verified.

---
*Phase: 16-icon-data-model*
*Completed: 2026-03-09*
