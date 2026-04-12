---
phase: 74-rgba-polish-and-must-use-uniformity
plan: 02
subsystem: api
tags: [must_use, clippy, rust-attributes, polish]

# Dependency graph
requires:
  - phase: 73-themechangeevent-cleanup
    provides: clean crate state after ThemeChangeEvent refactor
provides:
  - Uniform bare #[must_use] convention across all 13 core crate source files
  - ThemeSpec struct freed from #[must_use] (no longer fires on partial construction)
  - Result-returning functions freed from double_must_use lint
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns: ["bare #[must_use] only on non-Result returns; Result types carry their own must_use"]

key-files:
  created: []
  modified:
    - native-theme/src/model/mod.rs
    - native-theme/src/model/icons.rs
    - native-theme/src/model/bundled.rs
    - native-theme/src/lib.rs
    - native-theme/src/icons.rs
    - native-theme/src/detect.rs
    - native-theme/src/resolve/mod.rs
    - native-theme/src/resolve/validate.rs
    - native-theme/src/watch/mod.rs
    - native-theme/src/kde/mod.rs
    - native-theme/src/macos.rs
    - native-theme/src/windows.rs
    - native-theme/src/rasterize.rs

key-decisions:
  - "Remove #[must_use] entirely from Result-returning fns to avoid double_must_use clippy lint"
  - "Remove #[must_use] from ThemeSpec struct per plan (moralising on construction)"
  - "Keep bare #[must_use] only on functions returning Option, bool, &[T], Vec, or struct types"

patterns-established:
  - "must_use convention: bare #[must_use] on non-Result returns only; Result carries its own must_use"

requirements-completed: [POLISH-03]

# Metrics
duration: 7min
completed: 2026-04-12
---

# Phase 74 Plan 02: must_use Uniformity Summary

**Uniform bare #[must_use] convention: 24 sites converted to bare, 12 removed (ThemeSpec struct + 11 Result-returning fns), zero custom messages remain**

## Performance

- **Duration:** 7 min
- **Started:** 2026-04-12T14:47:13Z
- **Completed:** 2026-04-12T14:54:16Z
- **Tasks:** 1
- **Files modified:** 13

## Accomplishments
- Eliminated all 36 `#[must_use = "..."]` custom-message annotations from native-theme/src/
- Converted 24 sites to bare `#[must_use]` (functions returning Option, bool, Vec, &[T], or struct types)
- Removed `#[must_use]` entirely from ThemeSpec struct (fired on legitimate partial construction)
- Removed `#[must_use]` from 11 Result-returning functions (clippy double_must_use lint -- Result already carries must_use)
- Clippy passes clean, all tests pass, pre-release-check.sh green

## Task Commits

Each task was committed atomically:

1. **Task 1: Convert all must_use annotations to bare form** - `864d70f` (feat)

## Files Created/Modified
- `native-theme/src/model/mod.rs` - ThemeSpec struct #[must_use] removed; 4 methods converted to bare; 4 Result-returning methods stripped
- `native-theme/src/model/icons.rs` - 4 sites converted to bare #[must_use]
- `native-theme/src/model/bundled.rs` - 2 sites converted to bare #[must_use]
- `native-theme/src/lib.rs` - 5 sites: all removed (Result-returning fns)
- `native-theme/src/icons.rs` - 5 sites converted to bare #[must_use]
- `native-theme/src/detect.rs` - 4 sites converted to bare #[must_use]
- `native-theme/src/resolve/mod.rs` - 1 site removed (Result-returning fn)
- `native-theme/src/resolve/validate.rs` - 1 site removed (Result-returning fn)
- `native-theme/src/watch/mod.rs` - 1 site converted to bare #[must_use]
- `native-theme/src/kde/mod.rs` - 1 site removed (Result-returning fn)
- `native-theme/src/macos.rs` - 1 site removed (Result-returning fn)
- `native-theme/src/windows.rs` - 1 site removed (Result-returning fn)
- `native-theme/src/rasterize.rs` - 1 site removed (Result-returning fn)

## Decisions Made
- **Remove #[must_use] from Result-returning functions:** Clippy's `double_must_use` lint fires when a bare `#[must_use]` is placed on a function returning `Result<T>` because `Result` itself is already `#[must_use]`. Removing the attribute entirely is the correct fix -- callers still get the must_use warning from the Result type. This affects 11 functions (preset, from_toml, from_file, to_toml, into_resolved, validate, with_overlay, with_overlay_toml, from_system, from_system_async, rasterize_svg, from_kde, from_macos, from_windows).
- **ThemeSpec struct #[must_use] removed per plan:** The attribute was moralising on construction and fired on legitimate partial construction patterns.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Removed #[must_use] from Result-returning functions instead of converting to bare**
- **Found during:** Task 1 (clippy verification)
- **Issue:** Plan specified converting all 35 non-ThemeSpec sites to bare #[must_use], but 11 of those functions return `crate::Result<T>`. Clippy's `double_must_use` lint fires on bare `#[must_use]` when the return type already carries `#[must_use]`.
- **Fix:** Removed `#[must_use]` entirely from 11 Result-returning functions instead of converting to bare. Result type already provides must_use semantics.
- **Files modified:** lib.rs, model/mod.rs, resolve/mod.rs, resolve/validate.rs, kde/mod.rs, macos.rs, windows.rs, rasterize.rs
- **Verification:** `cargo clippy -p native-theme -- -D warnings` passes clean
- **Committed in:** 864d70f (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 bug fix)
**Impact on plan:** Auto-fix necessary for clippy compliance. The plan's goal (uniform convention, no custom messages) is fully achieved. The deviation refines the approach: bare #[must_use] for non-Result returns, no attribute for Result returns.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 74 complete (both plans executed)
- POLISH-03 requirement satisfied
- Ready for Phase 75 (non_exhaustive + compile-gate + IconSet::default removal)

---
*Phase: 74-rgba-polish-and-must-use-uniformity*
*Completed: 2026-04-12*
