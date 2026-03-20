---
phase: 39-code-quality-and-housekeeping
plan: 01
subsystem: core
tags: [unsafe-code, lint, deny, reduced-motion, accessibility, testing]

# Dependency graph
requires: []
provides:
  - "deny(unsafe_code) enforcement on core native-theme crate"
  - "Surgical allow(unsafe_code) annotations on FFI modules and test functions"
  - "Reduced motion smoke tests for all platforms"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns: ["deny at crate root, allow at module/function for FFI"]

key-files:
  created: []
  modified:
    - "native-theme/src/lib.rs"
    - "native-theme/src/macos.rs"
    - "native-theme/src/windows.rs"
    - "native-theme/src/sficons.rs"
    - "native-theme/src/winicons.rs"
    - "native-theme/src/kde/mod.rs"

key-decisions:
  - "Used deny(unsafe_code) instead of forbid(unsafe_code) because forbid cannot be overridden by allow in inner scopes, which is required for FFI modules"
  - "Module-level allow for macos.rs, sficons.rs, winicons.rs (nearly 100% FFI); function-level allow for windows.rs (only 3 of 9 functions use unsafe)"

patterns-established:
  - "deny-at-root-allow-at-leaf: Crate root has #![deny(unsafe_code)], FFI modules/functions get #[allow(unsafe_code)]"

requirements-completed: []

# Metrics
duration: 4min
completed: 2026-03-20
---

# Phase 39 Plan 01: Unsafe Code Lint Summary

**deny(unsafe_code) on core crate with surgical FFI allow annotations and cfg-gated reduced motion smoke tests**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-20T21:02:17Z
- **Completed:** 2026-03-20T21:06:39Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Added `#![deny(unsafe_code)]` to the core native-theme crate, preventing accidental unsafe code in pure-Rust modules
- Surgical `#[allow(unsafe_code)]` annotations on 3 FFI modules (macos.rs, sficons.rs, winicons.rs) and 3 functions in windows.rs
- Annotated 7 test functions using `set_var`/`remove_var` (unsafe in Rust 2024 edition) across lib.rs and kde/mod.rs
- Added 4 cfg-gated reduced motion smoke tests (cross-platform + Linux/macOS/Windows specific)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add deny(unsafe_code) lint with surgical allow annotations** - `4d5a4a2` (feat)
2. **Task 2: Write cfg-gated smoke tests for prefers_reduced_motion** - `fefa860` (test)

## Files Created/Modified
- `native-theme/src/lib.rs` - Added `#![deny(unsafe_code)]`, 4 test `#[allow(unsafe_code)]` annotations, and `reduced_motion_tests` module
- `native-theme/src/macos.rs` - Module-level `#![allow(unsafe_code)]` for Objective-C FFI
- `native-theme/src/sficons.rs` - Module-level `#![allow(unsafe_code)]` for CoreGraphics FFI
- `native-theme/src/winicons.rs` - Module-level `#![allow(unsafe_code)]` for Win32 GDI FFI
- `native-theme/src/windows.rs` - Function-level `#[allow(unsafe_code)]` on `read_system_font`, `read_geometry_dpi_aware`, `read_widget_metrics`
- `native-theme/src/kde/mod.rs` - 3 test `#[allow(unsafe_code)]` annotations for `set_var`/`remove_var` usage

## Decisions Made
- Used `deny(unsafe_code)` instead of `forbid(unsafe_code)` because Rust's `forbid` attribute cannot be overridden by inner `allow` annotations -- the compiler rejects it. `deny` provides the same default protection but permits surgical overrides for FFI modules.
- Used module-level `#![allow(unsafe_code)]` for macos.rs, sficons.rs, winicons.rs since these are nearly 100% FFI code. Used function-level `#[allow(unsafe_code)]` for windows.rs where only 3 of 9 functions contain unsafe blocks.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- `cargo check --all-features` fails due to ashpd crate's conflicting async-io/tokio features. This is a pre-existing dependency issue, not caused by this plan's changes. Used platform-appropriate feature set for verification instead.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Core crate now enforces unsafe code lint -- any future additions of unsafe code will require explicit allow annotations
- All existing tests pass with the new lint enforcement
- Reduced motion function has smoke test coverage on all platforms

## Self-Check: PASSED

All 6 modified files verified present. Both task commits (4d5a4a2, fefa860) verified in git log. deny(unsafe_code) and reduced_motion_tests module confirmed in lib.rs.

---
*Phase: 39-code-quality-and-housekeeping*
*Completed: 2026-03-20*
