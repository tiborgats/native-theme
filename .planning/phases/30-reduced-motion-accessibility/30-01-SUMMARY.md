---
phase: 30-reduced-motion-accessibility
plan: 01
subsystem: accessibility
tags: [reduced-motion, gsettings, nsworkspace, uisettings, oncelock, a11y]

# Dependency graph
requires:
  - phase: 27-animated-icon-model
    provides: AnimatedIcon model that needs motion preference awareness
provides:
  - prefers_reduced_motion() public function with OnceLock caching
  - OS-level reduced-motion detection on Linux, macOS, Windows
affects: [animated-icons, loading-indicator]

# Tech tracking
tech-stack:
  added: [objc2-app-kit NSWorkspace/NSAccessibility features]
  patterns: [OnceLock-cached accessibility queries, inverted-semantics OS settings]

key-files:
  created: []
  modified:
    - native-theme/src/lib.rs
    - native-theme/Cargo.toml

key-decisions:
  - "Reused OnceLock caching pattern from system_is_dark() for consistency"
  - "Used #[allow(unreachable_code)] on inner function to suppress cross-platform cfg warnings"

patterns-established:
  - "Accessibility query pattern: OnceLock-cached public fn + inner detection fn with per-platform cfg blocks"

requirements-completed: [A11Y-01, A11Y-02, A11Y-03, A11Y-04, A11Y-05]

# Metrics
duration: 2min
completed: 2026-03-18
---

# Phase 30 Plan 01: Reduced Motion Accessibility Summary

**prefers_reduced_motion() with OnceLock caching querying gsettings/NSWorkspace/UISettings across Linux/macOS/Windows**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-18T08:58:55Z
- **Completed:** 2026-03-18T09:01:40Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Implemented prefers_reduced_motion() public function with OnceLock caching for zero-cost subsequent calls
- Linux path queries gsettings enable-animations with inverted semantics (false = reduce motion)
- macOS path queries NSWorkspace.accessibilityDisplayShouldReduceMotion (feature-gated, direct semantics)
- Windows path queries UISettings.AnimationsEnabled with inverted semantics (feature-gated)
- Unsupported platforms return false (safe default: allow animations)
- Added NSWorkspace and NSAccessibility feature flags to objc2-app-kit in Cargo.toml
- Added doctest demonstrating usage and bool return type

## Task Commits

Each task was committed atomically:

1. **Task 1: Add macOS feature flags and implement prefers_reduced_motion()** - `2acb725` (feat)
2. **Task 2: Add doctest and verify cross-feature compilation** - `e712151` (test)

## Files Created/Modified
- `native-theme/Cargo.toml` - Added NSWorkspace and NSAccessibility features to objc2-app-kit
- `native-theme/src/lib.rs` - Added prefers_reduced_motion() public fn, detect_reduced_motion_inner() with platform cfg blocks, and doctest

## Decisions Made
- Reused OnceLock caching pattern from system_is_dark() for API consistency
- Used #[allow(unreachable_code)] on inner function to suppress cross-platform cfg warnings (matching existing codebase patterns)
- Fixed clippy needless_return in Linux cfg block (changed `return false;` to `false`)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed clippy needless_return warning**
- **Found during:** Task 2 (doctest and clippy verification)
- **Issue:** `return false;` at end of Linux cfg block triggered clippy::needless_return
- **Fix:** Changed to bare `false` expression (early returns within if-let still use `return`)
- **Files modified:** native-theme/src/lib.rs
- **Verification:** cargo clippy passes with -D clippy::needless-return
- **Committed in:** e712151 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Trivial style fix required by clippy. No scope creep.

## Issues Encountered
- `cargo clippy --all-features` fails due to pre-existing ashpd crate conflict (tokio + async-io mutually exclusive) -- not caused by our changes, verified with compatible feature combinations instead

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- prefers_reduced_motion() is ready for use by animation systems
- Applications can check this before starting loading spinners or other animations
- Phase 31 (integration) can wire this into loading_indicator() behavior

---
*Phase: 30-reduced-motion-accessibility*
*Completed: 2026-03-18*
