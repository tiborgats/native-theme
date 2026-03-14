---
phase: 01-v0-3-2-quality-improvements
plan: 02
subsystem: api
tags: [must-use, api-hygiene, dead-code-removal, documentation]

# Dependency graph
requires:
  - phase: 01-01
    provides: OnceLock caching and pick_variant API consolidation
provides:
  - "#[must_use] annotations on all 16 public functions and 2 key types"
  - "Cleaned derive.rs with dead wrapper removal"
  - "Renamed and documented colorize_monochrome_svg"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "#[must_use] with descriptive messages on all public API return values"
    - "Direct Colorize trait calls instead of wrapper functions"

key-files:
  modified:
    - native-theme/src/lib.rs
    - native-theme/src/model/mod.rs
    - native-theme/src/model/icons.rs
    - native-theme/src/model/bundled.rs
    - connectors/native-theme-gpui/src/derive.rs
    - connectors/native-theme-iced/src/icons.rs
    - native-theme/tests/preset_loading.rs

key-decisions:
  - "Kept Colorize trait import in derive.rs for direct base.darken() call"

patterns-established:
  - "All public functions returning values must have #[must_use] with descriptive messages"

requirements-completed: []

# Metrics
duration: 5min
completed: 2026-03-14
---

# Phase 01 Plan 02: API Hygiene Summary

**#[must_use] annotations on 16 public functions and 2 types, dead derive.rs wrapper removal, and colorize_svg renamed to colorize_monochrome_svg with documentation**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-14T03:59:21Z
- **Completed:** 2026-03-14T04:04:43Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- Added `#[must_use]` with descriptive messages to all 16 public API functions and 2 key types (NativeTheme, IconData)
- Removed 3 dead wrapper functions (lighten, darken, with_alpha) and 5 associated tests from derive.rs
- Replaced `darken(base, factor)` with direct `base.darken(factor)` Colorize trait call in active_color
- Renamed `colorize_svg` to `colorize_monochrome_svg` with doc comments explaining the monochrome-only contract

## Task Commits

Each task was committed atomically:

1. **Task 1: Add #[must_use] annotations** - `1edd021` (feat)
2. **Task 2: Remove dead derive.rs wrappers and rename colorize_svg** - `221a41b` (refactor)
3. **Fix: Silence must_use warning in preset_loading test** - `e05d320` (fix)

## Files Created/Modified

- `native-theme/src/lib.rs` - #[must_use] on from_system, from_system_async, load_icon, system_is_dark
- `native-theme/src/model/mod.rs` - #[must_use] on NativeTheme struct and 6 methods
- `native-theme/src/model/icons.rs` - #[must_use] on IconData enum, system_icon_set, system_icon_theme
- `native-theme/src/model/bundled.rs` - #[must_use] on bundled_icon_svg, bundled_icon_by_name
- `connectors/native-theme-gpui/src/derive.rs` - Removed lighten/darken/with_alpha, inlined darken call
- `connectors/native-theme-iced/src/icons.rs` - Renamed colorize_svg to colorize_monochrome_svg with docs
- `native-theme/tests/preset_loading.rs` - Fixed must_use warning with let binding

## Decisions Made

- Kept `Colorize` trait import in derive.rs since `base.darken(factor)` requires it

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed must_use warning in integration test**
- **Found during:** Task 1 verification (cargo test --workspace)
- **Issue:** `all_presets_parse_without_error` test created NativeTheme without using it, triggering the new #[must_use] warning
- **Fix:** Added `let _theme =` binding to silence the warning
- **Files modified:** native-theme/tests/preset_loading.rs
- **Verification:** cargo test -p native-theme passes with no warnings
- **Committed in:** e05d320

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Expected consequence of adding #[must_use] annotations. No scope creep.

## Issues Encountered

- Pre-existing naga dependency build failure prevents `cargo test --workspace` from completing for gpui examples, but all lib tests pass individually via `-p` flags. This is unrelated to our changes.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- API hygiene complete with must_use annotations and cleaned dead code
- Ready for plan 03 (remaining quality improvements)

---
## Self-Check: PASSED

All 7 modified files exist on disk. All 3 commit hashes (1edd021, 221a41b, e05d320) verified in git log.

---
*Phase: 01-v0-3-2-quality-improvements*
*Completed: 2026-03-14*
