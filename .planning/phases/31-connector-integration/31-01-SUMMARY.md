---
phase: 31-connector-integration
plan: 01
subsystem: connectors
tags: [gpui, iced, animation, animated-icon, svg, rotation, spin]

# Dependency graph
requires:
  - phase: 27-animated-icon-model
    provides: "AnimatedIcon enum (Frames, Transform variants)"
  - phase: 28-loading-indicator
    provides: "loading_indicator() returns platform-appropriate AnimatedIcon"
  - phase: 30-reduced-motion
    provides: "prefers_reduced_motion() for accessibility gating"
provides:
  - "gpui: animated_frames_to_image_sources() converts AnimatedIcon::Frames to Vec<ImageSource>"
  - "gpui: with_spin_animation() wraps Svg element with continuous rotation"
  - "iced: animated_frames_to_svg_handles() converts AnimatedIcon::Frames to Vec<svg::Handle>"
  - "iced: spin_rotation_radians() computes rotation angle from elapsed time"
affects: [32-documentation]

# Tech tracking
tech-stack:
  added: []
  patterns: ["connector data-conversion helpers for animated icons", "Svg-typed spin wrapper for gpui"]

key-files:
  created: []
  modified:
    - connectors/native-theme-gpui/src/icons.rs
    - connectors/native-theme-iced/src/icons.rs

key-decisions:
  - "with_spin_animation takes Svg instead of generic E: IntoElement because with_transformation() is only available on gpui::Svg"
  - "iced animated_frames_to_svg_handles returns None for empty/RGBA-only frames (filter_map + empty check)"
  - "animation_id parameter uses impl Into<ElementId> for flexible ID types"

patterns-established:
  - "Animated icon connector helpers follow same pattern as static icon helpers (pure data conversion, no state)"
  - "Doc comments note prefers_reduced_motion() and caching guidance"

requirements-completed: [CONN-01, CONN-02, CONN-03, CONN-04]

# Metrics
duration: 10min
completed: 2026-03-18
---

# Phase 31 Plan 01: Connector Integration Summary

**AnimatedIcon playback helpers for gpui (Svg spin + frame sources) and iced (SVG handles + rotation radians) with full TDD coverage**

## Performance

- **Duration:** 10 min
- **Started:** 2026-03-18T11:04:46Z
- **Completed:** 2026-03-18T11:14:18Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Four new public functions across two connector crates for converting AnimatedIcon data model values into toolkit-native animation primitives
- Full rustdoc on all functions with usage examples, caching guidance, and prefers_reduced_motion() notes
- 11 new unit tests (4 gpui + 7 iced) all passing, covering happy path, wrong variant, edge cases (empty frames, RGBA-only, boundary angles)
- with_spin_animation proven to construct AnimationElement without gpui render context (pure data test, no #[ignore])

## Task Commits

Each task was committed atomically:

1. **Task 1: Add AnimatedIcon helpers to gpui connector** (TDD)
   - `ae0ee89` test(31-01): add failing tests for gpui animated icon helpers
   - `8f41bd9` feat(31-01): implement gpui animated icon helpers
   - `78e57fa` refactor(31-01): fix clippy redundant_closure in gpui connector
2. **Task 2: Add AnimatedIcon helpers to iced connector** (TDD)
   - `4055fa1` test(31-01): add failing tests for iced animated icon helpers
   - `72c0f55` feat(31-01): implement iced animated icon helpers

## Files Created/Modified
- `connectors/native-theme-gpui/src/icons.rs` - Added animated_frames_to_image_sources() and with_spin_animation() with 4 unit tests
- `connectors/native-theme-iced/src/icons.rs` - Added animated_frames_to_svg_handles() and spin_rotation_radians() with 7 unit tests

## Decisions Made
- **with_spin_animation takes Svg not generic:** The plan specified `<E: IntoElement + 'static>` but `with_transformation()` is only available on `gpui::Svg`. Using `Svg` directly is correct and type-safe. Test uses `gpui::svg()` instead of `gpui::div()`.
- **animation_id uses impl Into<ElementId>:** More flexible than `&str` or `&'static str`, allows callers to pass any ElementId-compatible type.
- **iced frames returns None for empty:** Rather than returning `Some(vec![])` for empty frame sets, returns `None` to prevent division-by-zero in frame cycling (`% 0`).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] with_spin_animation signature corrected from generic to Svg**
- **Found during:** Task 1 (gpui connector implementation)
- **Issue:** Plan specified generic `<E: IntoElement + 'static>` but `with_transformation()` only exists on `gpui::Svg`, making the generic version uncompilable
- **Fix:** Changed function to take `Svg` directly instead of generic element type. Test uses `gpui::svg()` instead of `gpui::div()`.
- **Files modified:** connectors/native-theme-gpui/src/icons.rs
- **Verification:** Test compiles and passes without #[ignore]
- **Committed in:** 8f41bd9

**2. [Rule 1 - Bug] animation_id lifetime fixed from &str to impl Into<ElementId>**
- **Found during:** Task 1 (gpui connector implementation)
- **Issue:** `with_animation()` requires `impl Into<ElementId>` which only accepts `&'static str`, not arbitrary `&str`. Using `&str` caused "borrowed data escapes outside of function" error.
- **Fix:** Changed parameter type to `impl Into<gpui::ElementId>`
- **Files modified:** connectors/native-theme-gpui/src/icons.rs
- **Verification:** Compiles with &'static str literal in test
- **Committed in:** 8f41bd9

---

**Total deviations:** 2 auto-fixed (2 bugs in plan specification)
**Impact on plan:** Both auto-fixes necessary for compilation. No scope creep. The functions provide identical functionality to what was planned.

## Issues Encountered
- `cargo test --workspace` and `cargo clippy --workspace --all-targets` fail due to pre-existing `naga` crate compilation error (unrelated to our changes). Verified all three affected crates pass individually.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- All four connector animation helpers are implemented and tested
- Ready for Phase 32 (documentation/examples) which can demonstrate these helpers in showcase examples
- No blockers

## Self-Check: PASSED

All files exist. All 5 commits verified in git log.

---
*Phase: 31-connector-integration*
*Completed: 2026-03-18*
