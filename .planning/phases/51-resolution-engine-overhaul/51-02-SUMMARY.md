---
phase: 51-resolution-engine-overhaul
plan: 02
subsystem: resolve
tags: [inheritance, safety-nets, resolve-engine, bug-fix]

requires:
  - phase: 51-01
    provides: "Explicit text_scale entries in all 20 presets"
  - phase: 50-atomic-schema-commit
    provides: "New schema with FontSpec.color, BorderSpec fields, placeholder bindings in resolve.rs"
provides:
  - "defaults.font.color and mono_font.color inheritance chains in Phase 1"
  - "Removed 4 hardcoded safety nets (line_height, accent_text_color, shadow_color, disabled_text_color)"
  - "dialog.background_color per_platform fallback"
  - "Fixed INH-1: input.selection uses text_selection_background/color"
  - "Fixed INH-3: card.border inheritance removed"
  - "Fixed SPEC-3: switch.unchecked_background inheritance removed"
  - "Replaced scrollbar.thumb_hover_color luminance computation with muted_color inheritance"
affects: [51-03, 51-04, 51-05]

tech-stack:
  added: []
  patterns:
    - "defaults_internal font.color chain precedes font_inheritance phase"
    - "No fabricated safety nets -- all default values must come from presets"

key-files:
  created: []
  modified:
    - native-theme/src/resolve.rs

key-decisions:
  - "card.border and switch.unchecked_background moved from derived to preset-required in test helpers"
  - "dialog.background_color added as per_platform safety net (INH-2 fix)"

patterns-established:
  - "Safety nets only for per_platform fallbacks, never for fabricated values"
  - "Font color inheritance: text_color -> font.color -> mono_font.color chain order"

requirements-completed: [RESOLVE-01, RESOLVE-03, RESOLVE-05]

duration: 7min
completed: 2026-04-07
---

# Phase 51 Plan 02: Defaults/Safety-Net/Bug-Fix Summary

**Removed 4 fabricated safety nets, fixed 4 inheritance bugs (INH-1/INH-3/SPEC-3/thumb_hover), added font.color chain and dialog.background fallback**

## Performance

- **Duration:** 7 min
- **Started:** 2026-04-07T08:06:07Z
- **Completed:** 2026-04-07T08:13:33Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Established defaults.font.color <- defaults.text_color and defaults.mono_font.color <- defaults.font.color chains in Phase 1 (defaults_internal), enabling proper font color inheritance before Phase 3 font_inheritance runs
- Removed 4 hardcoded safety nets that fabricated values (line_height=1.2, accent_text_color=#ffffff, shadow_color=rgba(0,0,0,64), disabled_text_color from muted_color) -- all 20 presets provide these values
- Fixed 4 inheritance bugs: input.selection sources (INH-1), card.border removal (INH-3), switch.unchecked_background removal (SPEC-3), scrollbar.thumb_hover_color luminance computation replaced with simple muted_color inheritance (RESOLVE-03)
- Added dialog.background_color <- defaults.background_color as per_platform fallback (INH-2 fix)

## Task Commits

Each task was committed atomically:

1. **Task 1: defaults_internal additions and safety net removal** - `e692939` (feat)
2. **Task 2: Fix inheritance bugs and replace thumb_hover computation** - `e8bb677` (fix)

## Files Created/Modified
- `native-theme/src/resolve.rs` - Phase 1 font.color chains, removed fabricated safety nets, added dialog.background fallback, fixed 4 inheritance bugs, replaced luminance computation, updated tests

## Decisions Made
- card.border and switch.unchecked_background moved from "derived" (cleared in clear_derived_fields) to "preset-required" (provided in set_widget_geometry) since they no longer have inheritance rules
- dialog.background_color added as per_platform safety net alongside existing popover/list background fallbacks

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Updated test helpers for removed inheritance**
- **Found during:** Task 2 (INH-3 and SPEC-3 removal)
- **Issue:** clear_derived_fields() cleared card.border and switch.unchecked_background, but these fields no longer have inheritance rules, so resolve() cannot reconstruct them
- **Fix:** Stopped clearing these fields in clear_derived_fields(); added them to set_widget_geometry() as preset-required values
- **Files modified:** native-theme/src/resolve.rs (test helpers)
- **Verification:** All 429 tests pass including resolve_completeness_minimal_variant and resolve_completeness_from_preset
- **Committed in:** e8bb677 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Necessary correction for test consistency after inheritance removal. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Font color chain established; Plan 03 (resolve_font/resolve_border expansion) can now rely on defaults.font.color being populated
- Safety nets cleaned; Plan 04 (validate() rewiring) has fewer fabricated values to deal with
- All 4 bug fixes landed; inheritance-rules.toml is now accurately reflected in resolve.rs for the affected fields

---
*Phase: 51-resolution-engine-overhaul*
*Completed: 2026-04-07*
