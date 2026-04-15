---
phase: 90-resolve-remaining-v0-5-7-api-overhaul-gaps
plan: 01
subsystem: api
tags: [rgba, color, api-polish, rename, default-removal]

# Dependency graph
requires: []
provides:
  - "Rgba::new() constructor replacing self-named Rgba::rgba()"
  - "Manual Default impl for Rgba returning TRANSPARENT"
  - "Corrected detect.rs doc comment (pub not pub(crate))"
affects: [connectors, color-api-consumers]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Manual Default impl with explicit TRANSPARENT value instead of derive(Default)"

key-files:
  created: []
  modified:
    - native-theme/src/color.rs
    - native-theme/src/detect.rs
    - native-theme/src/windows.rs
    - native-theme/tests/proptest_roundtrip.rs
    - native-theme/tests/serde_roundtrip.rs
    - connectors/native-theme-iced/src/palette.rs
    - connectors/native-theme-gpui/src/lib.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs

key-decisions:
  - "Manual impl Default for Rgba instead of removing Default entirely -- internal types (ResolvedBorderSpec, ResolvedFontSpec, require() helper, ThemeWidget derive) depend on Rgba: Default bound"

patterns-established:
  - "Rgba::new(r, g, b, a) is the explicit-alpha constructor"
  - "Rgba::TRANSPARENT preferred over Rgba::default() for transparent-black"

requirements-completed: []

# Metrics
duration: 3min
completed: 2026-04-15
---

# Phase 90 Plan 01: Rgba API Polish and detect.rs Doc Fix Summary

**Replaced derive(Default) with manual impl, renamed Rgba::rgba() to Rgba::new(), fixed detect.rs pub(crate) doc inaccuracy**

## Performance

- **Duration:** 3 min
- **Started:** 2026-04-15T11:11:27Z
- **Completed:** 2026-04-15T11:15:22Z
- **Tasks:** 1
- **Files modified:** 8

## Accomplishments
- Renamed `Rgba::rgba()` to `Rgba::new()` across all 8 files (native-theme core, windows, tests, iced and gpui connectors)
- Replaced `derive(Default)` with manual `impl Default` returning `TRANSPARENT` -- makes the default value explicit in source
- Removed `#[allow(clippy::self_named_constructors)]` from color.rs
- Replaced `Rgba::default()` with `Rgba::TRANSPARENT` in gpui connector (2 showcase sites, 2 lib.rs comparisons)
- Fixed detect.rs doc comment: `pub(crate)` corrected to `pub` for `parse_linux_desktop()`
- Deleted `rgba_default_is_transparent_black` test (tested derived Default which is now manual)

## Task Commits

Each task was committed atomically:

1. **Task 1: Remove Default from Rgba, rename rgba() to new(), fix detect.rs doc** - `c508ac3` (fix)

## Files Created/Modified
- `native-theme/src/color.rs` - Removed derive(Default), added manual impl Default, renamed rgba() to new(), updated doc comments and doctests
- `native-theme/src/windows.rs` - Updated 4 Rgba::rgba() calls to Rgba::new()
- `native-theme/tests/proptest_roundtrip.rs` - Updated 1 Rgba::rgba() call to Rgba::new()
- `native-theme/tests/serde_roundtrip.rs` - Updated 2 Rgba::rgba() calls to Rgba::new()
- `native-theme/src/detect.rs` - Fixed doc comment: pub(crate) -> pub
- `connectors/native-theme-iced/src/palette.rs` - Updated 1 Rgba::rgba() call to Rgba::new()
- `connectors/native-theme-gpui/src/lib.rs` - Replaced 2 Rgba::default() with Rgba::TRANSPARENT
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - Replaced 2 Rgba::default() with Rgba::TRANSPARENT

## Decisions Made
- **Manual impl Default instead of full removal:** The plan specified removing Default entirely, but `ResolvedBorderSpec`, `ResolvedFontSpec`, the `require()` validation helper, and the `ThemeWidget` derive macro all require `Rgba: Default` as a trait bound. Added manual `impl Default` returning `Self::TRANSPARENT` to preserve compilation while making the default value explicit (no longer hidden in derive). This is a Rule 3 auto-fix (blocking issue).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added manual impl Default for Rgba instead of removing Default entirely**
- **Found during:** Task 1 (compilation after removing derive(Default))
- **Issue:** Removing `Default` from `Rgba` broke 30+ compilation sites where `Rgba: Default` is a trait bound (ResolvedBorderSpec, ResolvedFontSpec, require() helper, ThemeWidget derive macro codegen)
- **Fix:** Added `impl Default for Rgba { fn default() -> Self { Self::TRANSPARENT } }` with doc comment directing users to prefer TRANSPARENT or new()
- **Files modified:** native-theme/src/color.rs
- **Verification:** All 504 unit tests + 47 doctests + proptest + serde roundtrip + iced connector tests pass; clippy clean
- **Committed in:** c508ac3 (part of task commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Manual Default impl preserves the plan's intent (explicit default value, no hidden derive) while maintaining compilation. No scope creep.

## Issues Encountered
None beyond the deviation above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Rgba API is clean: `new()` constructor, explicit Default, no self_named_constructors
- detect.rs documentation is accurate
- Ready for remaining phase 90 plans (02-06)

---
*Phase: 90-resolve-remaining-v0-5-7-api-overhaul-gaps*
*Completed: 2026-04-15*
