---
phase: 12-widget-metrics
plan: 01
subsystem: model
tags: [serde, toml, widget-metrics, non-exhaustive, merge]

# Dependency graph
requires:
  - phase: 10-api-breaking-changes
    provides: "ThemeVariant with impl_merge! pattern, ThemeGeometry/ThemeSpacing sub-struct conventions"
provides:
  - "WidgetMetrics struct with 12 per-widget sub-structs (Button, Checkbox, Input, Scrollbar, Slider, ProgressBar, Tab, MenuItem, Tooltip, ListItem, Toolbar, Splitter)"
  - "ThemeVariant.widget_metrics: Option<WidgetMetrics> with recursive merge"
  - "Crate-root re-export of WidgetMetrics and all 12 sub-struct types"
affects: [12-02, 12-03, platform-readers, presets]

# Tech tracking
tech-stack:
  added: []
  patterns: ["manual merge/is_empty for Option<NestedStruct> on ThemeVariant"]

key-files:
  created: [native-theme/src/model/widget_metrics.rs]
  modified: [native-theme/src/model/mod.rs, native-theme/src/lib.rs, native-theme/src/macos.rs, native-theme/src/windows.rs, native-theme/src/kde/mod.rs]

key-decisions:
  - "Manual merge/is_empty for ThemeVariant instead of impl_merge! macro, to handle Option<WidgetMetrics> recursive merge semantics"
  - "WidgetMetrics fields are bare sub-structs (not Option) with skip_serializing_if is_empty; ThemeVariant holds Option<WidgetMetrics> for backward compatibility"

patterns-established:
  - "Option<NestedStruct> merge pattern: match arms for (Some,Some)->recurse, (None,Some)->clone, _->noop"
  - "Per-widget sub-struct convention: serde_with::skip_serializing_none, serde(default), non_exhaustive, all Option<f32> fields"

requirements-completed: [METRIC-01, METRIC-02, METRIC-03]

# Metrics
duration: 3min
completed: 2026-03-08
---

# Phase 12 Plan 01: Widget Metrics Data Model Summary

**WidgetMetrics data model with 12 per-widget sub-structs integrated into ThemeVariant with recursive Option merge**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-08T07:52:18Z
- **Completed:** 2026-03-08T07:55:13Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Created widget_metrics.rs with 12 per-widget sub-structs (ButtonMetrics through SplitterMetrics), each with Option<f32> fields, #[non_exhaustive], serde defaults, and impl_merge!
- Integrated WidgetMetrics into ThemeVariant as Option<WidgetMetrics> with manual merge/is_empty that correctly recurses into nested Option
- Re-exported WidgetMetrics and all 12 sub-struct types from crate root
- All 150 existing tests pass unchanged (backward compatible)

## Task Commits

Each task was committed atomically:

1. **Task 1: Create widget_metrics.rs with 12 sub-structs and WidgetMetrics** - `a0e2d64` (feat)
2. **Task 2: Integrate WidgetMetrics into ThemeVariant and re-export** - `7e5031b` (feat)

## Files Created/Modified
- `native-theme/src/model/widget_metrics.rs` - 12 per-widget sub-structs + WidgetMetrics + 12 unit tests
- `native-theme/src/model/mod.rs` - ThemeVariant gains widget_metrics field, manual merge/is_empty, re-exports
- `native-theme/src/lib.rs` - Crate-root re-export of WidgetMetrics
- `native-theme/src/macos.rs` - Added widget_metrics: None to struct literals
- `native-theme/src/windows.rs` - Added widget_metrics: None to struct literal
- `native-theme/src/kde/mod.rs` - Added widget_metrics: None to struct literal

## Decisions Made
- Used manual merge/is_empty implementation on ThemeVariant instead of impl_merge! macro, because the macro's `nested {}` assumes always-present structs while `Option<WidgetMetrics>` requires match-arm recursion (same pattern as NativeTheme::merge for Option<ThemeVariant>)
- WidgetMetrics sub-struct fields are bare (not Option-wrapped) with skip_serializing_if is_empty, while ThemeVariant holds Option<WidgetMetrics> for backward compatibility with existing TOML files

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed ThemeVariant struct literal sites in platform readers**
- **Found during:** Task 2 (ThemeVariant integration)
- **Issue:** Adding widget_metrics field to #[non_exhaustive] ThemeVariant broke struct literal construction in macos.rs, windows.rs, and kde/mod.rs
- **Fix:** Added `widget_metrics: None` to all 4 struct literal sites (2 in macos, 1 in windows, 1 in kde)
- **Files modified:** native-theme/src/macos.rs, native-theme/src/windows.rs, native-theme/src/kde/mod.rs
- **Verification:** cargo build -p native-theme compiles cleanly, all 150 tests pass
- **Committed in:** 7e5031b (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Required for compilation. Plan anticipated this possibility in step 6. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- WidgetMetrics types ready for platform reader population (12-02: KDE, Windows, macOS, GNOME)
- WidgetMetrics types ready for preset TOML updates (12-03)
- All sub-structs follow established patterns for impl_merge!, serde, and is_empty

---
*Phase: 12-widget-metrics*
*Completed: 2026-03-08*
