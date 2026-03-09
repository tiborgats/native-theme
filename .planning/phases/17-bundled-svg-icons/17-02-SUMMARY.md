---
phase: 17-bundled-svg-icons
plan: 02
subsystem: icons
tags: [svg, include-bytes, feature-gates, material-icons, lucide-icons, bundled-assets]

# Dependency graph
requires:
  - phase: 17-bundled-svg-icons
    provides: 76 SVG icon files (38 Material + 38 Lucide) in native-theme/icons/
  - phase: 16-icon-data-model
    provides: IconRole enum (42 variants), IconSet enum, IconData type
provides:
  - bundled_icon_svg(IconSet, IconRole) -> Option<&'static [u8]> function
  - material-icons Cargo feature flag
  - lucide-icons Cargo feature flag
  - Feature-gated compile-time SVG embedding for all 42 icon roles per set
affects: [21-icon-connectors, crate-root-api]

# Tech tracking
tech-stack:
  added: []
  patterns: [feature-gated include_bytes! for compile-time asset embedding, unreachable_patterns + unused_variables allow for non_exhaustive forward compat]

key-files:
  created:
    - native-theme/src/model/bundled.rs
  modified:
    - native-theme/src/model/mod.rs
    - native-theme/src/lib.rs
    - native-theme/Cargo.toml

key-decisions:
  - "TrashFull and TrashEmpty both map to delete.svg (Material) / trash-2.svg (Lucide) since neither set has a dedicated full-trash icon"
  - "StatusError reuses error.svg / circle-x.svg (same as DialogError) per set"
  - "Help reuses help.svg / circle-question-mark.svg (same as DialogQuestion) per set"
  - "Added unused_variables allow on bundled_icon_svg to suppress warning when no icon features enabled"

patterns-established:
  - "Feature-gated icon embedding: #[cfg(feature = X)] on private helper function, public dispatch function with #[allow(unreachable_patterns, unused_variables)]"
  - "include_bytes! paths relative to source file (../../icons/{set}/{name}.svg)"

requirements-completed: [BNDL-01, BNDL-02]

# Metrics
duration: 2min
completed: 2026-03-09
---

# Phase 17 Plan 02: Bundled SVG Icons Implementation Summary

**Feature-gated bundled_icon_svg() function with include_bytes! embedding for all 42 IconRole variants across Material and Lucide icon sets**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-09T07:29:53Z
- **Completed:** 2026-03-09T07:32:28Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Created bundled.rs module with feature-gated include_bytes! for all 42 IconRole variants per icon set
- Material icons (material-icons feature): all 42 roles resolve to valid SVG bytes, total size well under 200KB
- Lucide icons (lucide-icons feature): all 42 roles resolve to valid SVG bytes, total size well under 100KB
- Without features enabled, bundled_icon_svg returns None for all inputs (zero binary bloat)
- Function re-exported at crate root as native_theme::bundled_icon_svg
- All 176 tests pass with both features; 172 pass with no features; workspace compiles cleanly

## Task Commits

Each task was committed atomically:

1. **Task 1: Create bundled.rs module with feature-gated include_bytes** - `2d4f61c` (feat)
2. **Task 2: Verify full build with all feature combinations** - (verification only, no file changes)

## Files Created/Modified
- `native-theme/src/model/bundled.rs` - Feature-gated bundled_icon_svg() with include_bytes! for 42 Material + 42 Lucide icon roles
- `native-theme/src/model/mod.rs` - Added pub mod bundled and re-export of bundled_icon_svg
- `native-theme/src/lib.rs` - Added bundled_icon_svg to crate root re-exports
- `native-theme/Cargo.toml` - Added material-icons and lucide-icons feature declarations

## Decisions Made
- TrashFull and TrashEmpty both map to the same SVG (delete.svg for Material, trash-2.svg for Lucide) since neither icon set has a dedicated "full trash" icon. This satisfies the requirement that every IconRole variant resolves to valid SVG bytes.
- StatusError reuses the same SVG as DialogError (error.svg / circle-x.svg) per icon set.
- Help reuses the same SVG as DialogQuestion (help.svg / circle-question-mark.svg) per icon set.
- Added `#[allow(unused_variables)]` to bundled_icon_svg to suppress a warning when no icon features are enabled (the `role` parameter becomes unused since the match body is entirely cfg-gated away).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed unused variable warning without features**
- **Found during:** Task 1 (verification step)
- **Issue:** Without any icon features enabled, the `role` parameter in `bundled_icon_svg()` was flagged as unused because all feature-gated match arms are compiled away, leaving only `_ => None`
- **Fix:** Added `#[allow(unused_variables)]` attribute to the function
- **Files modified:** native-theme/src/model/bundled.rs
- **Verification:** `cargo check -p native-theme` produces no warnings
- **Committed in:** 2d4f61c (part of Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Trivial warning fix, no scope creep.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- bundled_icon_svg() is ready for Phase 21 (icon connectors) to use as a fallback icon source
- Callers use `bundled_icon_svg(set, role).map(|b| IconData::Svg(b.to_vec()))` to get IconData
- Both feature flags are opt-in; default compilation has zero icon bloat

---
*Phase: 17-bundled-svg-icons*
*Completed: 2026-03-09*
