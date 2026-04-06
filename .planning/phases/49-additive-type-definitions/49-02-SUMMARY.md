---
phase: 49-additive-type-definitions
plan: 02
subsystem: model
tags: [define_widget_pair, layout, theme-spec, toml, serde]

# Dependency graph
requires:
  - phase: 49-additive-type-definitions/01
    provides: SurfaceTheme type definition pattern via define_widget_pair!
provides:
  - LayoutTheme/ResolvedLayoutTheme with 4 spacing fields (widget_gap, container_margin, window_margin, section_gap)
  - ThemeSpec.layout top-level field with merge/is_empty/serde support
  - TOML lint_toml validation for [layout] section
affects: [50-atomic-schema-commit, 51-resolution-validation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Top-level ThemeSpec field for variant-independent data (layout is shared, not per light/dark)"

key-files:
  created: []
  modified:
    - native-theme/src/model/widgets/mod.rs
    - native-theme/src/model/mod.rs
    - native-theme/src/lib.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/src/kde/mod.rs
    - native-theme/src/macos.rs
    - native-theme/src/windows.rs

key-decisions:
  - "LayoutTheme is a non-Option field on ThemeSpec (like defaults.font pattern) -- Default is empty, serde(default) on the field handles missing TOML sections"
  - "layout added to lint_toml TOP_KEYS and subfield validation to prevent false warnings"

patterns-established:
  - "Top-level shared section: variant-independent data lives on ThemeSpec directly, not ThemeVariant"

requirements-completed: []

# Metrics
duration: 3min
completed: 2026-04-06
---

# Phase 49 Plan 02: LayoutTheme Summary

**LayoutTheme/ResolvedLayoutTheme via define_widget_pair! with 4 spacing fields, wired to ThemeSpec as top-level shared section**

## Performance

- **Duration:** 3 min 30s
- **Started:** 2026-04-06T22:35:24Z
- **Completed:** 2026-04-06T22:38:54Z
- **Tasks:** 1
- **Files modified:** 7

## Accomplishments
- Defined LayoutTheme/ResolvedLayoutTheme via define_widget_pair! macro with 4 fields (widget_gap, container_margin, window_margin, section_gap)
- Wired LayoutTheme to ThemeSpec as a top-level non-Option field (shared between light/dark variants)
- Updated ThemeSpec::new(), merge(), is_empty() and lint_toml() to include layout
- Added 9 tests: 5 LayoutTheme unit tests + 4 ThemeSpec integration tests including TOML round-trip and top-level serialization verification
- Added LayoutTheme to lib.rs public re-exports

## Task Commits

Each task was committed atomically:

1. **Task 1: Create LayoutTheme module and wire to ThemeSpec** - `33045be` (feat)

**Plan metadata:** (pending)

## Files Created/Modified
- `native-theme/src/model/widgets/mod.rs` - Added LayoutTheme/ResolvedLayoutTheme definition via define_widget_pair! and 5 unit tests
- `native-theme/src/model/mod.rs` - Added layout field to ThemeSpec, updated new/merge/is_empty/lint_toml, added 4 integration tests
- `native-theme/src/lib.rs` - Added LayoutTheme to public re-export list
- `native-theme/src/gnome/mod.rs` - Added layout field to ThemeSpec struct literals
- `native-theme/src/kde/mod.rs` - Added layout field to ThemeSpec struct literals
- `native-theme/src/macos.rs` - Added layout field to ThemeSpec struct literal
- `native-theme/src/windows.rs` - Added layout field to ThemeSpec struct literals

## Decisions Made
- LayoutTheme is a non-Option field on ThemeSpec (like the defaults.font pattern) -- Default is an empty struct where all fields are None, serde(default) on the field handles missing TOML sections
- Added layout to lint_toml TOP_KEYS and subfield validation to prevent false lint warnings on valid [layout] sections

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed ThemeSpec struct literal compilation errors in platform readers**
- **Found during:** Task 1 (compile check)
- **Issue:** macos.rs, windows.rs, gnome/mod.rs, kde/mod.rs construct ThemeSpec with struct literals that became incomplete after adding the layout field
- **Fix:** Added `layout: crate::LayoutTheme::default()` to all 5 struct literals in 4 files
- **Files modified:** native-theme/src/macos.rs, native-theme/src/windows.rs, native-theme/src/gnome/mod.rs, native-theme/src/kde/mod.rs
- **Verification:** `cargo check -p native-theme` passes cleanly
- **Committed in:** 33045be (part of task commit)

**2. [Rule 3 - Blocking] Added layout to lint_toml validation**
- **Found during:** Task 1 (after adding [layout] section support)
- **Issue:** lint_toml TOP_KEYS did not include "layout", causing false warnings on valid TOML with [layout] sections; also needed subfield validation
- **Fix:** Added "layout" to TOP_KEYS const and added LayoutTheme::FIELD_NAMES validation for [layout] subfields
- **Files modified:** native-theme/src/model/mod.rs
- **Verification:** All 14 lint tests pass including lint_toml_all_presets_clean
- **Committed in:** 33045be (part of task commit)

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both fixes necessary for compilation and correctness. No scope creep.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- LayoutTheme and ResolvedLayoutTheme types are defined and accessible via public API
- ThemeSpec supports [layout] section in TOML at top level
- Phase 50 (atomic schema commit) can reference LayoutTheme for migration
- Phase 51 will add resolution/validation that rejects missing layout values

## Self-Check: PASSED

All 7 modified files verified present. Commit 33045be verified in git log.

---
*Phase: 49-additive-type-definitions*
*Completed: 2026-04-06*
