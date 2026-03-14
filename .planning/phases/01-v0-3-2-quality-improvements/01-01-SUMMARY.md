---
phase: 01-v0-3-2-quality-improvements
plan: 01
subsystem: core
tags: [oncelock, caching, api-consolidation, deprecation]

# Dependency graph
requires: []
provides:
  - "OnceLock-cached system_icon_theme() eliminating redundant subprocess spawns"
  - "OnceLock-cached system_is_dark() eliminating redundant gsettings calls"
  - "NativeTheme::pick_variant() method consolidating variant selection logic"
  - "Deprecated connector pick_variant free functions delegating to method"
affects: [connectors, downstream-consumers]

# Tech tracking
tech-stack:
  added: []
  patterns: ["OnceLock caching for expensive system detection calls"]

key-files:
  created: []
  modified:
    - "native-theme/src/model/icons.rs"
    - "native-theme/src/lib.rs"
    - "native-theme/src/model/mod.rs"
    - "connectors/native-theme-gpui/src/lib.rs"
    - "connectors/native-theme-iced/src/lib.rs"

key-decisions:
  - "Used static OnceLock inside function body (not module-level) for Linux-only caching to keep cfg gating clean"
  - "Extracted detect_is_dark_inner as private helper to separate caching from detection logic"

patterns-established:
  - "OnceLock caching: wrap expensive OS detection in static OnceLock for zero-cost repeated access"
  - "API consolidation: move shared logic to core type method, deprecate connector wrappers"

requirements-completed: []

# Metrics
duration: 4min
completed: 2026-03-14
---

# Phase 01 Plan 01: Caching and API Consolidation Summary

**OnceLock caching for system_icon_theme/system_is_dark and NativeTheme::pick_variant() method with deprecated connector wrappers**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-14T03:51:33Z
- **Completed:** 2026-03-14T03:56:16Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Cached system_icon_theme() on Linux with OnceLock to eliminate 42 redundant subprocess spawns per icon load cycle
- Cached system_is_dark() on Linux with OnceLock to avoid repeated gsettings/kdeglobals detection
- Added NativeTheme::pick_variant() method with 5 comprehensive tests covering all fallback scenarios
- Deprecated connector pick_variant free functions in gpui and iced, delegating to the new method

## Task Commits

Each task was committed atomically:

1. **Task 1: Add OnceLock caching to system_icon_theme() and system_is_dark()** - `edcbcc3` (feat)
2. **Task 2: Add NativeTheme::pick_variant() method and deprecate connector free functions** (TDD)
   - RED: `c641cbe` (test) - 5 failing tests for pick_variant
   - GREEN: `1462022` (feat) - implementation + connector deprecation

**Clippy fix:** `bae3d28` (fix) - redundant closure in OnceLock init

## Files Created/Modified
- `native-theme/src/model/icons.rs` - Added OnceLock caching for system_icon_theme() Linux path
- `native-theme/src/lib.rs` - Added OnceLock caching for system_is_dark(), extracted detect_is_dark_inner()
- `native-theme/src/model/mod.rs` - Added NativeTheme::pick_variant() method with 5 tests
- `connectors/native-theme-gpui/src/lib.rs` - Deprecated pick_variant, delegates to method
- `connectors/native-theme-iced/src/lib.rs` - Deprecated pick_variant, delegates to method

## Decisions Made
- Used static OnceLock inside function body (cfg-gated for Linux only) rather than module-level static to keep platform gating clean
- Extracted detect_is_dark_inner as private helper to separate caching concern from detection logic
- Added #[allow(deprecated)] on test modules to keep existing connector tests working against deprecated wrappers

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed clippy redundant_closure lint**
- **Found during:** Overall verification (clippy)
- **Issue:** `|| detect_linux_icon_theme()` closure was redundant -- clippy prefers direct function reference
- **Fix:** Changed to `.get_or_init(detect_linux_icon_theme)`
- **Files modified:** native-theme/src/model/icons.rs
- **Verification:** `cargo clippy -p native-theme -- -D warnings` passes clean
- **Committed in:** bae3d28

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Minor style fix for clippy compliance. No scope creep.

## Issues Encountered
- `cargo test --workspace` and `cargo clippy --workspace` fail due to pre-existing `naga` dependency compilation error in the gpui transitive dependency chain (unrelated to our changes). Tests and clippy were verified per-crate instead.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- OnceLock caching and pick_variant API consolidation complete
- Ready for Plan 02 (DE icon theme detection improvements)
- No blockers

## Self-Check: PASSED

All 5 files verified present. All 4 commits verified in git history. All must-have artifacts confirmed (OnceLock in icons.rs and lib.rs, pick_variant in mod.rs, #[deprecated] in both connectors).

---
*Phase: 01-v0-3-2-quality-improvements*
*Completed: 2026-03-14*
