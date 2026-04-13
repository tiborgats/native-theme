---
phase: 81-feature-matrix-cleanup-and-unified-from-system
plan: 01
subsystem: api
tags: [cargo-features, async, pollster, from-system, pipeline]

# Dependency graph
requires:
  - phase: 80-native-theme-derive-proc-macro-k-codegen
    provides: derive macro codegen and widget migration (clean codebase baseline)
provides:
  - Single async from_system_inner() replacing duplicated sync/async inner functions
  - Simplified Cargo.toml feature graph with linux-kde/linux-portal sub-aggregators
  - pollster-based sync wrapper for from_system() on Linux
  - Noop-waker single-poll sync wrapper for from_system() on non-Linux
affects: [81-02 CI feature matrix, connectors, gpui]

# Tech tracking
tech-stack:
  added: [pollster 0.4]
  patterns: [single-async-impl-with-sync-wrapper, noop-waker-poll-for-zero-await-futures, sub-aggregator-features]

key-files:
  created: []
  modified:
    - native-theme/src/pipeline.rs
    - native-theme/src/lib.rs
    - native-theme/Cargo.toml
    - connectors/native-theme-gpui/Cargo.toml

key-decisions:
  - "pollster is non-optional on Linux (not gated behind portal feature) -- from_system() always needs block_on"
  - "Non-Linux from_system() uses Waker::noop() single-poll instead of pollster -- zero-dep on macOS/Windows"
  - "portal feature activates ashpd/async-io directly (no separate runtime-variant features)"
  - "linux-kde and linux-portal sub-aggregators enable fine-grained feature selection"

patterns-established:
  - "Single async inner + sync wrapper: all platform dispatch in one async fn, sync callers use pollster (Linux) or noop-waker (non-Linux)"
  - "Sub-aggregator features: linux-kde/linux-portal compose into linux, providing granular control"

requirements-completed: [FEATURE-01, FEATURE-02]

# Metrics
duration: 7min
completed: 2026-04-13
---

# Phase 81 Plan 01: Feature Matrix Cleanup Summary

**Unified from_system/from_system_async to single async fn with pollster sync wrapper, eliminated 4 redundant features and added linux-kde/linux-portal sub-aggregators**

## Performance

- **Duration:** 7 min
- **Started:** 2026-04-13T12:58:08Z
- **Completed:** 2026-04-13T13:05:17Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Eliminated duplicated orchestration logic between sync and async entry points -- single async `from_system_inner()` handles all platforms
- Simplified feature graph from 15 to 13 features by removing portal-tokio, portal-async-io, linux-async-io, native-async-io
- Added linux-kde and linux-portal sub-aggregator features for fine-grained dependency control
- `from_system()` sync wrapper uses pollster::block_on on Linux (where async D-Bus calls exist) and noop-waker single-poll on non-Linux (zero .await points)

## Task Commits

Each task was committed atomically:

1. **Task 1: Restructure Cargo.toml features and add pollster dependency** - `6657590` (feat)
2. **Task 2: Unify from_system and from_system_async to single async implementation** - `8c944c2` (feat)

## Files Created/Modified
- `native-theme/Cargo.toml` - Restructured features: removed 4 runtime-variant features, added linux-kde/linux-portal sub-aggregators, pollster as non-optional Linux dep
- `connectors/native-theme-gpui/Cargo.toml` - Updated to use `linux` feature (was `linux-async-io`)
- `native-theme/src/pipeline.rs` - Replaced from_linux() + sync from_system_inner() + async from_system_async_inner() with single `async fn from_system_inner()`
- `native-theme/src/lib.rs` - from_system() uses pollster::block_on on Linux / noop-waker on non-Linux; from_system_async() is single unconditional method

## Decisions Made
- pollster is non-optional on Linux because from_system() always calls block_on, even when portal feature is disabled (the async fn may have zero .await points, but pollster is still needed to drive it)
- Non-Linux from_system() uses std::task::Waker::noop() single-poll instead of pulling pollster as a dependency -- correct because macOS/Windows have zero .await points
- portal feature now directly activates ashpd/async-io (not tokio), eliminating the need for separate runtime-variant features
- Removed unused non-Linux import of system_is_dark (was only needed by the old sync from_system_inner)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Feature graph restructured and verified across all feature combinations
- Ready for Plan 02 (CI matrix verification and FEATURE-03 deprecated feature aliases)
- Pre-existing dead_code warning on `build_gnome_spec_pure` in gnome/mod.rs is out of scope (not introduced by this plan)

---
*Phase: 81-feature-matrix-cleanup-and-unified-from-system*
*Completed: 2026-04-13*
