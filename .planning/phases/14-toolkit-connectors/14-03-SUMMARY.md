---
phase: 14-toolkit-connectors
plan: 03
subsystem: connectors
tags: [gpui, gpui-component, hsla, theme-mapping, color-conversion]

# Dependency graph
requires:
  - phase: 10-api-breaking-changes
    provides: Flat ThemeColors (36 fields), ThemeVariant, NativeTheme API
  - phase: 12-widget-metrics
    provides: WidgetMetrics (12 sub-structs) for upstream proposal context
provides:
  - native-theme-gpui crate with 108-field ThemeColor mapping
  - derive.rs shade derivation helpers (lighten, darken, hover_color, active_color)
  - config.rs ThemeFonts/ThemeGeometry to ThemeConfig mapping
  - to_theme() and pick_variant() public API
  - Upstream PR proposal for gpui-component widget metric hooks
affects: [14-toolkit-connectors]

# Tech tracking
tech-stack:
  added: [gpui 0.2.2, gpui-component 0.5.1]
  patterns: [thin-mapping-layer, shade-derivation, colorize-trait-reuse]

key-files:
  created:
    - connectors/native-theme-gpui/src/derive.rs
    - connectors/native-theme-gpui/src/colors.rs
    - connectors/native-theme-gpui/src/config.rs
    - connectors/native-theme-gpui/proposals/README.md
  modified:
    - connectors/native-theme-gpui/Cargo.toml
    - connectors/native-theme-gpui/src/lib.rs

key-decisions:
  - "Reused gpui-component's Colorize trait for lighten/darken instead of reimplementing"
  - "Matched gpui-component's internal apply_config fallback logic for hover/active derivation"
  - "Used ThemeColor::default() as base and override all 108 fields explicitly"

patterns-established:
  - "Shade derivation via gpui-component Colorize trait (multiplicative lightness)"
  - "Thin mapping with helper functions per logical group (core, primary, status, list, tab, misc)"

requirements-completed: [CONN-01, CONN-02, CONN-03]

# Metrics
duration: 9min
completed: 2026-03-08
---

# Phase 14 Plan 03: gpui Connector Core Summary

**108-field ThemeColor mapping with shade derivation, ThemeConfig font/geometry mapping, and upstream PR proposal for widget metric hooks**

## Performance

- **Duration:** 9 min
- **Started:** 2026-03-08T09:07:20Z
- **Completed:** 2026-03-08T09:16:53Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Complete 108-field ThemeColor mapping from 36 native-theme semantic colors, with derived hover/active/chart/list/table/sidebar states
- Shade derivation helpers reusing gpui-component's Colorize trait for consistent lighten/darken behavior
- ThemeConfig mapping from ThemeFonts and ThemeGeometry (font_family, mono_font_family, font_size, mono_font_size, radius, radius_lg, shadow)
- Public API: to_theme() builds complete gpui-component Theme, pick_variant() selects light/dark with fallback
- 183-line upstream PR proposal document for per-widget metric configuration in gpui-component ThemeConfig
- 24 unit tests covering color conversion, shade derivation, config mapping, variant selection, and theme construction

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement shade derivation and 108-field ThemeColor mapping** - `435b379` (feat)
2. **Task 2: Implement ThemeConfig mapping, public API, and upstream proposal** - `8db9df4` (feat)

## Files Created/Modified
- `connectors/native-theme-gpui/Cargo.toml` - Added gpui 0.2 and gpui-component 0.5 dependencies
- `connectors/native-theme-gpui/src/derive.rs` - Shade derivation: lighten, darken, hover_color, active_color, with_alpha
- `connectors/native-theme-gpui/src/colors.rs` - Full 108-field ThemeColor mapping with rgba_to_hsla conversion
- `connectors/native-theme-gpui/src/config.rs` - ThemeFonts/ThemeGeometry to ThemeConfig mapping
- `connectors/native-theme-gpui/src/lib.rs` - Public API: to_theme(), pick_variant(), module declarations
- `connectors/native-theme-gpui/proposals/README.md` - Upstream PR proposal for widget metric hooks

## Decisions Made
- Reused gpui-component's Colorize trait (lighten/darken) instead of custom shade math, ensuring consistent behavior with the upstream library
- Matched gpui-component's internal apply_config fallback patterns (e.g., hover = bg.blend(base.opacity(0.9)), active = base.darken(0.1/0.2))
- Started from ThemeColor::default() and overrode all 108 fields via helper functions grouped by category (core, primary, secondary, status, list/table, tab/sidebar, charts, misc, base colors)
- Used SharedString::from(s.clone()) for ThemeConfig string fields to satisfy 'static lifetime requirements

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- SharedString (ArcCow<str>) in gpui's ThemeConfig requires 'static lifetimes -- resolved by cloning strings into SharedString::from(s.clone()) rather than borrowing
- SharedString comparison in tests required .to_string() conversion since ArcCow doesn't implement PartialEq<str> directly

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- native-theme-gpui crate compiles and all 24 tests pass
- Ready for plan 04 (showcase example) which will depend on to_theme() and pick_variant()
- Upstream proposal document ready for review before submission

## Self-Check: PASSED

All 6 created/modified files verified present. Both task commits (435b379, 8db9df4) verified in git log.

---
*Phase: 14-toolkit-connectors*
*Completed: 2026-03-08*
