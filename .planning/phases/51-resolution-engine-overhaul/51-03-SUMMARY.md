---
phase: 51-resolution-engine-overhaul
plan: 03
subsystem: resolve
tags: [inheritance, border, font, resolve-engine, disabled-opacity]

requires:
  - phase: 51-02
    provides: "defaults.font.color chain, safety net cleanup, bug fixes"
  - phase: 50-atomic-schema-commit
    provides: "BorderSpec with corner_radius_lg, FontSpec with style/color, placeholder bindings"
provides:
  - "resolve_border() free function handling 4 sub-fields with lg_radius support"
  - "resolve_border_inheritance() for 13 full-border widgets + 2 partial (sidebar, status_bar)"
  - "resolve_font() expanded to 5 sub-fields (family, size, weight, style, color)"
  - "resolve_font_inheritance() expanded from 8 to 19 widget fonts"
  - "link.font.color override to defaults.link_color after generic font inheritance"
  - "All disabled_opacity inheritance rules (input, checkbox, slider, switch, combo_box, segmented_control)"
  - "button.hover_text_color widget-to-widget inheritance from button.font.color"
  - "sidebar.selection, toolbar.background/icon_size, status_bar.background, splitter.divider_color, separator.line_width rules"
affects: [51-04, 51-05]

tech-stack:
  added: []
  patterns:
    - "resolve_border() as free function paralleling resolve_font() for sub-struct inheritance"
    - "Partial border inheritance (color + line_width only) for sidebar and status_bar"
    - "Widget-to-widget rules in Phase 4 resolve_widget_to_widget for cross-widget dependencies"

key-files:
  created: []
  modified:
    - native-theme/src/resolve.rs

key-decisions:
  - "button.hover_text_color placed in resolve_widget_to_widget (Phase 4) since it depends on button.font.color from Phase 3"
  - "Ad-hoc border code replaced with centralized resolve_border() calls -- net reduction of 65 lines"
  - "Font color assignments removed from resolve_color_inheritance since resolve_font now handles color sub-field"

patterns-established:
  - "resolve_border(widget_border, defaults_border, use_lg_radius) pattern for all border inheritance"
  - "Phase ordering: color -> border -> font -> text_scale -> widget-to-widget"

requirements-completed: [RESOLVE-04, RESOLVE-06]

duration: 6min
completed: 2026-04-07
---

# Phase 51 Plan 03: Border/Font/Missing Rules Summary

**resolve_border() for 13 widgets with lg-radius support, resolve_font() expanded to 19 widgets with 5 sub-fields, all missing disabled_opacity and color rules implemented**

## Performance

- **Duration:** 6 min
- **Started:** 2026-04-07T08:16:41Z
- **Completed:** 2026-04-07T08:22:54Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Created resolve_border() free function and resolve_border_inheritance() method covering 13 full-border widgets (with corner_radius_lg for window/popover/dialog) plus 2 partial-border widgets (sidebar, status_bar with color+line_width only)
- Expanded resolve_font() from 3 sub-fields to 5 (added style, color) and resolve_font_inheritance() from 8 to 19 widget fonts, with link.font.color override to defaults.link_color
- Added all missing inheritance rules: 7 disabled_opacity rules, sidebar.selection, toolbar.background/icon_size, status_bar.background, splitter.divider_color, separator.line_width, button.hover_text_color
- Removed 185 lines of ad-hoc border and redundant font.color code from resolve_color_inheritance, replaced with 120 lines of centralized border/font handling (net -65 lines)

## Task Commits

Each task was committed atomically:

1. **Task 1: Create resolve_border() and update resolve_font()** - `90e8eca` (feat)
2. **Task 2: Add missing color, opacity, and sizing inheritance rules** - `eb350e7` (feat)

## Files Created/Modified
- `native-theme/src/resolve.rs` - resolve_border() function, resolve_border_inheritance() method, expanded resolve_font() and resolve_font_inheritance(), missing color/opacity/sizing rules, button.hover_text_color in widget-to-widget

## Decisions Made
- button.hover_text_color placed in resolve_widget_to_widget (Phase 4) since it depends on button.font.color being resolved in Phase 3 font inheritance
- Ad-hoc border code in resolve_color_inheritance replaced with centralized resolve_border() calls, netting a 65-line reduction
- Font color assignments removed from resolve_color_inheritance since resolve_font now handles the color sub-field uniformly

## Deviations from Plan

None - plan executed exactly as written. Several rules listed in Task 2 as "verify before adding" were already added in Task 1 (sidebar.selection, toolbar.background/icon_size, status_bar.background, splitter.divider_color, separator.line_width) as part of the resolve_color_inheritance cleanup.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All inheritance rules from inheritance-rules.toml are now implemented in resolve.rs
- resolve_border_inheritance() provides fully resolved border specs for all 15 widgets with borders
- resolve_font_inheritance() provides fully resolved font specs for all 19 widgets with fonts
- Plan 04 (validate() rewiring) can now replace all 57 placeholder bindings with proper require() calls that read from the now-resolved widget fields

---
*Phase: 51-resolution-engine-overhaul*
*Completed: 2026-04-07*
