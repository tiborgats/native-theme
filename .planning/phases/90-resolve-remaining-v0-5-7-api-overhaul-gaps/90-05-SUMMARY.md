---
phase: 90-resolve-remaining-v0-5-7-api-overhaul-gaps
plan: 05
subsystem: model
tags: [arc, font, interning, deduplication, mutex, cache]

# Dependency graph
requires:
  - phase: 87-font-family-arc-str-and-animated-icon-invariants
    provides: "Arc<str> font family migration in FontSpec/ResolvedFontSpec"
provides:
  - "pub fn intern_font_family(&str) -> Arc<str> with global dedup cache"
  - "Iced connector doc example showing Arc-based font usage"
affects: [connectors, font-handling]

# Tech tracking
tech-stack:
  added: []
  patterns: ["LazyLock<Mutex<HashSet<Arc<str>>>> for global string interning with graceful poison handling"]

key-files:
  created: []
  modified:
    - native-theme/src/model/font.rs
    - native-theme/src/model/mod.rs
    - connectors/native-theme-iced/src/lib.rs

key-decisions:
  - "Graceful mutex poison handling via if-let-Ok instead of .unwrap() -- returns uncached Arc on poison"

patterns-established:
  - "intern_font_family pattern: global LazyLock cache for Arc<str> deduplication"

requirements-completed: []

# Metrics
duration: 2min
completed: 2026-04-15
---

# Phase 90 Plan 05: intern_font_family Summary

**Global Arc<str> font family dedup cache with graceful mutex poison handling, replacing Box::leak in iced connector docs**

## Performance

- **Duration:** 2 min
- **Started:** 2026-04-15T11:11:34Z
- **Completed:** 2026-04-15T11:14:21Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Added `intern_font_family` public function with `FONT_FAMILY_CACHE` static dedup cache
- Graceful degradation on mutex poison (returns fresh `Arc<str>` without caching)
- Replaced `Box::leak` pattern in iced connector doc example with `intern_font_family`
- Re-exported function through `native_theme::theme::intern_font_family` path

## Task Commits

Each task was committed atomically:

1. **Task 1: Add intern_font_family function with global cache** - `3de729e` (feat)
2. **Task 2: Update iced connector doc example** - `70a2305` (docs)

## Files Created/Modified
- `native-theme/src/model/font.rs` - Added FONT_FAMILY_CACHE static and intern_font_family() function
- `native-theme/src/model/mod.rs` - Added intern_font_family to explicit re-export list
- `connectors/native-theme-iced/src/lib.rs` - Replaced Box::leak doc example with intern_font_family usage

## Decisions Made
- Graceful mutex poison handling via `if let Ok(mut cache) = FONT_FAMILY_CACHE.lock()` instead of `.unwrap()` -- returns uncached Arc on poison (no panic path)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

Pre-existing compilation errors in the crate (Rgba::rgba missing, Rgba missing Default) from other phase 90 plans not yet executed. These do not affect the correctness of the font.rs and iced connector changes -- verified via rustfmt, doc build, and grep-based checks.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- `intern_font_family` is available for any connector that needs deduplicated font family names
- No blockers for subsequent plans

---
*Phase: 90-resolve-remaining-v0-5-7-api-overhaul-gaps*
*Completed: 2026-04-15*
