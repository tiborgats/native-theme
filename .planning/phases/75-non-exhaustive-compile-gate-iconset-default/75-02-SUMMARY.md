---
phase: 75-non-exhaustive-compile-gate-iconset-default
plan: 02
subsystem: model
tags: [icon-set, default-removal, compile-gate, watch-feature, serde]

# Dependency graph
requires: []
provides:
  - "IconSet enum without Default derive -- IconSet::default() is a compile error"
  - "WATCH-03 verified: on_theme_change/ThemeWatcher/ThemeChangeEvent gated behind watch feature"
affects: [82-icon-api-rework]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Inline validation for types without Default instead of generic require() helper"

key-files:
  created: []
  modified:
    - native-theme/src/model/icons.rs
    - native-theme/src/resolve/validate.rs
    - CHANGELOG.md

key-decisions:
  - "Inlined icon_set validation in validate.rs instead of adding a second require variant, keeping the fix minimal and local"

patterns-established:
  - "Types without Default need inline validation in validate.rs rather than the generic require<T: Default>() helper"

requirements-completed: [WATCH-03, ICON-05]

# Metrics
duration: 3min
completed: 2026-04-12
---

# Phase 75 Plan 02: Remove IconSet::default() and verify watch compile gate Summary

**Removed Default derive from IconSet so calling IconSet::default() is a compile error; verified on_theme_change is compile-gated behind the watch feature**

## Performance

- **Duration:** 3 min
- **Started:** 2026-04-12T15:36:51Z
- **Completed:** 2026-04-12T15:39:46Z
- **Tasks:** 1
- **Files modified:** 3

## Accomplishments
- IconSet no longer implements Default -- calling IconSet::default() produces a compile error, preventing the misleading Freedesktop-on-all-platforms footgun
- validate.rs updated to inline icon_set extraction without requiring Default bound on IconSet
- CHANGELOG.md documents the removal with migration guidance to system_icon_set()
- WATCH-03 verified: lib.rs lines 116 and 185 gate the watch module and re-exports behind #[cfg(feature = "watch")]

## Task Commits

Each task was committed atomically:

1. **Task 1: Remove Default from IconSet and verify watch gating** - `7164a72` (feat)

**Plan metadata:** (pending)

## Files Created/Modified
- `native-theme/src/model/icons.rs` - Removed Default derive and #[default] attribute from IconSet enum; simplified Freedesktop doc comment
- `native-theme/src/resolve/validate.rs` - Inlined icon_set validation to avoid Default trait bound; added IconSet import
- `CHANGELOG.md` - Added migration note for IconSet::default() removal under [0.5.7] Removed section

## Decisions Made
- Inlined icon_set validation in validate.rs with a match expression and Freedesktop placeholder, rather than creating a second require function variant without Default bound. The placeholder is never used (validate short-circuits on missing fields before construction).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed validate.rs Default bound on require() for IconSet**
- **Found during:** Task 1 (Remove Default from IconSet)
- **Issue:** validate.rs used `require(&self.icon_set, ...)` which requires `T: Clone + Default`, but IconSet no longer implements Default
- **Fix:** Inlined the icon_set extraction with a direct match expression and explicit Freedesktop placeholder (never used -- validate returns Err before construction when fields are missing)
- **Files modified:** native-theme/src/resolve/validate.rs
- **Verification:** `cargo check -p native-theme` shows zero IconSet::Default errors
- **Committed in:** 7164a72 (part of task commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary fix for removing Default from IconSet. No scope creep.

## Issues Encountered
- Pre-existing LinuxDesktop non-exhaustive match errors in pipeline.rs and icons.rs prevent full `cargo test` from running. These are from plan 75-01's new LinuxDesktop variants (Hyprland, Sway, River, Niri, CosmicDe) which have not yet had match arms added. Out of scope for this plan.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- IconSet::default() removal is complete and documented
- WATCH-03 compile-gate verification is complete
- Plan 75-01 (LinuxDesktop non_exhaustive + match arm updates) must also complete for clean compilation
- Phase 76 (type rename + crate root partition) can proceed once both 75 plans are done

---
*Phase: 75-non-exhaustive-compile-gate-iconset-default*
*Completed: 2026-04-12*
