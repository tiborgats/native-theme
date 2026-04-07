---
phase: 51-resolution-engine-overhaul
plan: 04
subsystem: resolve
tags: [validation, border, font, require, placeholder-removal]

requires:
  - phase: 51-03
    provides: "resolve_border/resolve_font inheritance rules for all widgets"
  - phase: 50-atomic-schema-commit
    provides: "57 placeholder bindings for new fields in validate()"
provides:
  - "require_border() for 13 full-border widgets (4 inherited sub-fields validated)"
  - "border_all_optional() for menu/tab/card (no inheritance, all sub-fields optional)"
  - "require_border_partial() for sidebar/status_bar (color + line_width validated)"
  - "require_font/require_font_opt now validate font.color via require()"
  - "All 57 placeholder bindings replaced with proper require/require_font_opt/require_border calls"
  - "Missing inheritance rules for 13 interactive-state color fields"
  - "Range checks for all new required fields (opacity, font size/weight, geometry)"
affects: [51-05]

tech-stack:
  added: []
  patterns:
    - "require_border() validates only inherited sub-fields; padding uses unwrap_or_default()"
    - "border_all_optional() for widgets excluded from border_inheritance (no validation errors)"
    - "require_border_partial() for widgets with partial inheritance (color + line_width only)"

key-files:
  created: []
  modified:
    - native-theme/src/resolve.rs

key-decisions:
  - "Border padding fields (padding_horizontal/padding_vertical) are sizing fields with no inheritance -- use unwrap_or_default() instead of require()"
  - "Menu/tab/card borders use border_all_optional() since all sub-fields are platform-specific with no inheritance"
  - "Added 13 missing color inheritance rules (button.hover_background, checkbox.background_color, etc.) to resolve_color_inheritance() so require() calls succeed"
  - "Removed unused crate::Rgba import from main code (only needed in test module)"

patterns-established:
  - "Three-tier border validation: require_border (full), require_border_partial (partial), border_all_optional (none)"
  - "Font color validated via require() in both require_font and require_font_opt"

requirements-completed: [RESOLVE-07]

duration: 18min
completed: 2026-04-07
---

# Phase 51 Plan 04: Validate Rewiring Summary

**All 57 placeholder bindings replaced with proper require()/require_border()/require_font_opt() calls; font.color validated; three-tier border validation for full/partial/no-inheritance widgets**

## Performance

- **Duration:** 18 min
- **Started:** 2026-04-07T08:25:51Z
- **Completed:** 2026-04-07T08:43:37Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Fixed require_font and require_font_opt to validate font.color via require() instead of hardcoded Rgba::rgb(0,0,0) fallback
- Created three require_border variants: require_border() for 13 full-border widgets, require_border_partial() for sidebar/status_bar, border_all_optional() for menu/tab/card
- Replaced all 57 placeholder bindings with proper require/require_font_opt/require_border calls, positioned before the missing-field check
- Added range checks for all newly required fields: 6 disabled_opacity (0..=1), 11 widget font size/weight, 4 geometry fields
- Added 13 missing inheritance rules in resolve_color_inheritance() for interactive-state colors that were previously silent defaults

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix require_font/require_font_opt color and create require_border** - `2708fd3` (feat)
2. **Task 2: Replace all 57 placeholder bindings with proper require calls** - `650202b` (feat)

## Files Created/Modified
- `native-theme/src/resolve.rs` - require_border/border_all_optional/require_border_partial functions, 57 placeholder replacements, 13 new inheritance rules, range checks, updated test helper

## Decisions Made
- Border padding fields use unwrap_or_default() (not require) because they are sizing fields with no inheritance per inheritance-rules.toml
- Menu/tab/card borders use border_all_optional() since no border sub-fields have inheritance for these widgets
- Added 13 inheritance rules for interactive-state colors (button.hover_background <- defaults.background_color, checkbox.background_color/indicator_color, menu.icon_size/hover_background/hover_text_color/disabled_text_color, sidebar.hover_background, list.hover_background, combo_box.background_color, segmented_control.background_color/active_background/active_text_color) so validate() succeeds with require() calls
- Removed unused Rgba import from main code after fixing require_font color handling

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added 13 missing inheritance rules for interactive-state colors**
- **Found during:** Task 2
- **Issue:** Plan assumed all 57 fields would be populated after resolve(), but 13 interactive-state color fields had no inheritance rules and no preset data in macOS/Windows builders
- **Fix:** Added inheritance rules in resolve_color_inheritance() for button.hover_background, checkbox.background_color, checkbox.indicator_color, menu.icon_size/hover_background/hover_text_color/disabled_text_color, sidebar.hover_background, list.hover_background, combo_box.background_color, segmented_control.background_color/active_background/active_text_color
- **Files modified:** native-theme/src/resolve.rs
- **Verification:** All 20 presets pass validation; all 429 tests pass
- **Committed in:** 650202b

**2. [Rule 1 - Bug] Fixed border padding requirement in require_border()**
- **Found during:** Task 2
- **Issue:** Plan specified require() for padding_horizontal/padding_vertical in require_border(), but padding is a sizing field with no inheritance per inheritance-rules.toml -- presets that don't provide padding would fail validation
- **Fix:** Changed padding fields to use unwrap_or_default() in require_border(); created border_all_optional() for menu/tab/card (replacing require_border_padding_only from the plan)
- **Files modified:** native-theme/src/resolve.rs
- **Verification:** All presets pass; border padding defaults to 0.0 when not in preset
- **Committed in:** 650202b

**3. [Rule 1 - Bug] Fixed require() calls placed after missing-field check**
- **Found during:** Task 2
- **Issue:** Initial placement of require calls was after the `if !missing.is_empty()` return, meaning they would never catch missing fields
- **Fix:** Moved all require calls to before the range validation section, where the other require calls live
- **Committed in:** 650202b

**4. [Rule 1 - Bug] Updated fully_populated_variant test helper**
- **Found during:** Task 2
- **Issue:** Test helper lacked many of the newly required fields (font colors, border sub-fields, interactive-state colors)
- **Fix:** Added all missing fields to the test helper so validate() succeeds on the "fully populated" variant
- **Committed in:** 650202b

---

**Total deviations:** 4 auto-fixed (1 missing critical, 3 bugs)
**Impact on plan:** All auto-fixes necessary for correctness. No scope creep -- same intent (complete validation coverage) achieved with adjusted border/color handling to match the actual inheritance specification.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- validate() now has zero placeholder bindings; every field uses require()/require_font_opt()/require_border()/border_all_optional()
- All 20 presets pass validation including range checks
- Font color is validated properly (no more Rgba::rgb(0,0,0) fallback)
- Plan 05 (final cleanup/verification) can proceed

---
*Phase: 51-resolution-engine-overhaul*
*Completed: 2026-04-07*
