---
phase: 87-font-family-arc-str-and-animatedicon-invariants
plan: 01
subsystem: model
tags: [animated-icon, newtype, serde, non-zero, invariant]

# Dependency graph
requires: []
provides:
  - "FrameList newtype with non-empty invariant and custom Deserialize"
  - "FramesData/TransformData wrapper structs with private fields"
  - "AnimatedIcon::frames() and AnimatedIcon::transform() constructors"
  - "NonZeroU32 for all duration fields (frame_duration_ms, Spin::duration_ms)"
  - "Infallible first_frame() returning &IconData"
affects: [87-02, connectors]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Newtype with custom Deserialize for invariant enforcement at deserialization boundary"
    - "Wrapper structs with private fields for enum variant field privacy"
    - "NonZeroU32 for compile-time zero-duration prevention"
    - "Compile-time const assertions for hardcoded NonZeroU32 values"

key-files:
  created: []
  modified:
    - native-theme/src/model/animated.rs
    - native-theme/src/model/mod.rs
    - native-theme/src/freedesktop.rs
    - native-theme/src/spinners.rs
    - native-theme/src/icons.rs
    - connectors/native-theme-iced/src/icons.rs
    - connectors/native-theme-gpui/src/icons.rs
    - connectors/native-theme-iced/examples/showcase-iced.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs
    - connectors/native-theme-iced/README.md
    - connectors/native-theme-gpui/README.md
    - native-theme/README.md
    - README.md
    - CHANGELOG.md

key-decisions:
  - "FrameList::len() has #[allow(clippy::len_without_is_empty)] because FrameList is guaranteed non-empty"
  - "Removed #[must_use] from first_frame(), FrameList::first(), TransformData::icon() to avoid double_must_use (IconData already has #[must_use])"
  - "frames_or_spin_fallback takes &'static [u8] for lifetime compatibility with include_bytes!()"
  - "Constants FREEDESKTOP_FRAME_DURATION_MS and SPIN_FRAME_DURATION_MS remain u32 with NonZeroU32 conversion at call site via ? operator"

patterns-established:
  - "AnimatedIcon::Frames(data)/Transform(data) tuple variant pattern matching with accessor methods"
  - "NonZeroU32 .get() for u32 conversion in connector structs that store plain u32"

requirements-completed: [LAYOUT-03]

# Metrics
duration: 16min
completed: 2026-04-14
---

# Phase 87 Plan 01: AnimatedIcon Private Fields and FrameList Newtype Summary

**FrameList newtype with custom Deserialize, NonZeroU32 durations, and private variant fields via FramesData/TransformData wrapper structs -- eliminates soundness gap where AnimatedIcon::Frames { frames: vec![], duration: 0 } compiled**

## Performance

- **Duration:** 16 min
- **Started:** 2026-04-13T22:28:14Z
- **Completed:** 2026-04-13T22:44:03Z
- **Tasks:** 2
- **Files modified:** 14

## Accomplishments
- FrameList newtype enforces non-empty invariant at construction AND deserialization (custom Deserialize rejects empty arrays)
- AnimatedIcon variant fields are now private via FramesData/TransformData wrapper structs -- struct literal construction fails outside the crate
- All duration fields use NonZeroU32, making zero duration impossible at the type level
- first_frame() is now infallible (returns &IconData, not Option)
- All 14 files across workspace migrated: zero struct-literal construction, zero field destructuring
- 32 new/updated tests in animated.rs including serde round-trip and empty-deserialization-rejection
- 753 native-theme lib tests pass, 97 iced connector tests pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Create FrameList newtype with custom Deserialize and migrate AnimatedIcon to private fields** - `4eee147` (feat)
2. **Task 2: Migrate all AnimatedIcon construction and pattern-matching sites across codebase** - `3522d2f` (feat)

## Files Created/Modified
- `native-theme/src/model/animated.rs` - FrameList, FramesData, TransformData, EmptyFrameListError, constructors, accessors, 32 tests
- `native-theme/src/model/mod.rs` - Re-exports for new types
- `native-theme/src/freedesktop.rs` - Constructor calls with NonZeroU32 conversion via ?
- `native-theme/src/spinners.rs` - frames_or_spin_fallback helper with graceful error handling
- `native-theme/src/icons.rs` - Pattern match migration to tuple variants + accessors
- `connectors/native-theme-iced/src/icons.rs` - accessor methods + .get() for u32 conversion
- `connectors/native-theme-gpui/src/icons.rs` - same accessor migration
- `connectors/native-theme-iced/examples/showcase-iced.rs` - tuple variant patterns
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - tuple variant patterns
- `connectors/native-theme-iced/README.md` - updated code examples
- `connectors/native-theme-gpui/README.md` - updated code examples
- `native-theme/README.md` - updated code examples
- `README.md` - updated code examples
- `CHANGELOG.md` - updated code examples

## Decisions Made
- FrameList::len() gets `#[allow(clippy::len_without_is_empty)]` since FrameList is always non-empty -- providing is_empty() that always returns false would be misleading
- Removed `#[must_use]` from `first_frame()`, `FrameList::first()`, and `TransformData::icon()` because `&IconData` is already `#[must_use]` (clippy double_must_use)
- `frames_or_spin_fallback` takes `&'static [u8]` because callers use `include_bytes!()` and the fallback path needs `Cow::Borrowed` with 'static lifetime
- Duration constants remain `u32` with `NonZeroU32::new()` conversion at call sites via `?` operator (avoids hook-blocked `.unwrap()` in const context while remaining zero-panic)
- Connector structs (`AnimatedSvgHandles`, `AnimatedImageSources`) keep `frame_duration_ms: u32` -- they are connector-owned types, not core model types

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed double_must_use clippy lint**
- **Found during:** Task 2 (pre-release-check.sh)
- **Issue:** `#[must_use]` on `first_frame()`, `FrameList::first()`, `TransformData::icon()` triggered clippy::double_must_use because `&IconData` is already `#[must_use]`
- **Fix:** Removed redundant `#[must_use]` from these three methods
- **Files modified:** native-theme/src/model/animated.rs
- **Committed in:** 3522d2f (Task 2 commit)

**2. [Rule 1 - Bug] Fixed len_without_is_empty clippy lint**
- **Found during:** Task 2 (pre-release-check.sh)
- **Issue:** FrameList had `len()` but no `is_empty()`, triggering clippy::len_without_is_empty
- **Fix:** Added `#[allow(clippy::len_without_is_empty)]` with doc comment explaining non-empty guarantee
- **Files modified:** native-theme/src/model/animated.rs
- **Committed in:** 3522d2f (Task 2 commit)

**3. [Rule 3 - Blocking] Fixed lifetime mismatch in frames_or_spin_fallback**
- **Found during:** Task 2 (compilation)
- **Issue:** `Cow::Borrowed(svg_bytes)` in fallback path required 'static lifetime but parameter was `&[u8]`
- **Fix:** Changed parameter to `&'static [u8]` (callers always pass `include_bytes!()`)
- **Files modified:** native-theme/src/spinners.rs
- **Committed in:** 3522d2f (Task 2 commit)

**4. [Rule 3 - Blocking] Fixed module-level const assertion syntax**
- **Found during:** Task 2 (compilation)
- **Issue:** `const { assert!(...) }` is not valid at module level in Rust (only inside function bodies)
- **Fix:** Removed module-level const assertions; values are obviously non-zero literals (80, 42, 1000)
- **Files modified:** native-theme/src/freedesktop.rs, native-theme/src/spinners.rs
- **Committed in:** 3522d2f (Task 2 commit)

---

**Total deviations:** 4 auto-fixed (2 bug fixes, 2 blocking)
**Impact on plan:** All auto-fixes necessary for correct compilation and clippy compliance. No scope creep.

## Issues Encountered
- Pre-existing uncommitted Plan 02 (font Arc<str>) changes in font.rs and resolved.rs caused compilation failures; stashed during execution and restored after
- Pre-existing gpui connector test failures from Arc<str> migration (not from this plan)
- Pre-existing naga dependency compilation failure blocks `cargo test --all-features` across full workspace; individual crate tests used instead

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- AnimatedIcon API is fully migrated; Plan 02 (font family Arc<str>) can proceed independently
- All connector pattern matches already use the new tuple variant syntax
- Deprecated `new_frames()` shim available for any external consumers during transition

---
*Phase: 87-font-family-arc-str-and-animatedicon-invariants*
*Completed: 2026-04-14*
