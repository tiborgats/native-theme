---
phase: 68-implement-docs-todo-v0-5-6-gtk-icon-theme-md
plan: 01
subsystem: icons
tags: [gtk, adwaita, symbolic-icons, svg, currentColor, freedesktop]

# Dependency graph
requires:
  - phase: none
    provides: existing freedesktop.rs icon loading and connector colorize infrastructure
provides:
  - normalize_gtk_symbolic function replacing GTK foreground placeholders with currentColor
  - find_icon returning (PathBuf, bool) to distinguish symbolic from non-symbolic icons
  - conditional normalization in load_freedesktop_icon and load_freedesktop_icon_by_name
  - 9 unit tests covering all normalization patterns
affects: [connector-colorize, icon-loading, dark-theme-support]

# Tech tracking
tech-stack:
  added: []
  patterns: [gtk-symbolic-normalization, two-pass-icon-lookup-with-symbolic-flag]

key-files:
  created: []
  modified: [native-theme/src/freedesktop.rs]

key-decisions:
  - "Raw string literals with br##\"...\"## needed for test SVGs containing hex color # characters"

patterns-established:
  - "GTK symbolic normalization: normalize foreground placeholders to currentColor at load time, let connectors handle final colorization"
  - "find_icon returns (PathBuf, bool) tuple -- bool indicates symbolic, gates normalization"

requirements-completed: []

# Metrics
duration: 8min
completed: 2026-04-09
---

# Phase 68 Plan 01: GTK Symbolic Icon Normalization Summary

**GTK symbolic SVGs normalized to currentColor via String::replace on 4 foreground placeholders, gated by is_symbolic flag from find_icon two-pass lookup**

## Performance

- **Duration:** 8 min
- **Started:** 2026-04-09T23:03:19Z
- **Completed:** 2026-04-09T23:12:06Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Implemented `normalize_gtk_symbolic` function that replaces 4 GTK foreground placeholder colors (#2e3436, #2e3434, #222222, #474747) with `currentColor` in fill/stroke XML attributes and CSS style properties
- Refactored `find_icon` to return `(PathBuf, bool)` where bool indicates symbolic variant, enabling conditional normalization
- Updated both `load_freedesktop_icon` and `load_freedesktop_icon_by_name` to conditionally normalize symbolic SVGs
- 9 unit tests covering: all 4 foreground colors, stroke, CSS style fill, semantic color preservation, Breeze passthrough, non-GTK passthrough

## Task Commits

Each task was committed atomically:

1. **Task 1: Add normalize_gtk_symbolic tests (RED phase)** - `62eced0` (test)
2. **Task 2: Implement normalize_gtk_symbolic, refactor find_icon, update load functions (GREEN phase)** - `832fbf2` (feat)

## Files Created/Modified
- `native-theme/src/freedesktop.rs` - Added GTK_FG_COLORS const, normalize_gtk_symbolic fn, refactored find_icon return type, updated load functions, added 9 tests

## Decisions Made
- Used `br##"..."##` raw string literals for test SVGs containing hex color `#` characters (Rust's `br#"..."#` syntax conflicts with `#` inside the string)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed raw string literal syntax for hex colors in tests**
- **Found during:** Task 1 (RED phase test writing)
- **Issue:** Plan's test code used `br#"..."#` which conflicts with `#` in hex color values like `#2e3436` -- Rust parser interprets `"#` as the raw string terminator
- **Fix:** Changed all test SVG literals from `br#"..."#` to `br##"..."##` (double-hash raw string delimiter)
- **Files modified:** native-theme/src/freedesktop.rs (test section only)
- **Verification:** All 9 tests compile and run correctly
- **Committed in:** 62eced0 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Minimal -- syntax-level fix to make tests compile. No semantic or logic change.

## Issues Encountered
None beyond the raw string syntax fix documented above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- GTK symbolic icon normalization is complete and tested
- Connectors (iced, gpui) already handle `currentColor` replacement -- no changes needed there
- Icons on GNOME/GTK dark themes will now be correctly recolored via existing connector colorize logic

---
*Phase: 68-implement-docs-todo-v0-5-6-gtk-icon-theme-md*
*Completed: 2026-04-09*
