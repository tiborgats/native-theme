---
phase: 46-os-reader-extensions
plan: 01
subsystem: kde-reader
tags: [kde, kdeglobals, qt-font, configparser, theme-variant, breeze, accessibility]

requires:
  - phase: 44-per-widget-model
    provides: "ThemeVariant with 25 per-widget structs, define_widget_pair! macro, impl_merge!"
  - phase: 45-resolution-engine
    provides: "resolve() 4-phase inheritance + validate() -> ResolvedTheme pipeline"
provides:
  - "KDE reader producing sparse ThemeVariant with per-widget colors, fonts, text_scale, accessibility, icon_set"
  - "Qt5/Qt6 font weight conversion via qt5_to_css_weight()"
  - "Title bar colors from [WM] section (KDE-01)"
  - "Per-widget fonts: menu, toolbar, title bar (KDE-03)"
  - "Text scale from Kirigami multipliers (KDE-04)"
  - "Accessibility flags from AnimationDurationFactor and forceFontDPI (KDE-06)"
affects: [gnome-reader, connector-mapping, integration-tests]

tech-stack:
  added: []
  patterns:
    - "populate_xxx(ini, &mut variant) pattern for OS readers targeting ThemeVariant directly"
    - "Qt5/Qt6 font weight detection by field count (>=16 = Qt6 CSS scale, <16 = Qt5 0-100 scale)"
    - "Kirigami text scale multipliers: section_heading=1.20x, dialog_title=1.35x"

key-files:
  created: []
  modified:
    - native-theme/src/kde/colors.rs
    - native-theme/src/kde/fonts.rs
    - native-theme/src/kde/metrics.rs
    - native-theme/src/kde/mod.rs

key-decisions:
  - "KDE reader merges with default preset before resolve/validate (sparse reader + preset = complete theme)"
  - "Icon sizes from index.theme deferred -- parsing freedesktop icon theme directories adds complexity for marginal value; icon_set name is sufficient for runtime lookup"

patterns-established:
  - "populate_colors/fonts/sizing pattern: each submodule takes (ini, &mut variant) and sets fields directly"
  - "Integration test pattern: load preset, merge reader output, resolve, validate, spot-check"

requirements-completed: [KDE-01, KDE-02, KDE-03, KDE-04, KDE-05, KDE-06]

duration: 14min
completed: 2026-03-27
---

# Phase 46 Plan 01: KDE Reader Rewrite Summary

**KDE reader rewritten to produce sparse ThemeVariant with per-widget colors from 8 INI groups, Qt5/Qt6 font weight conversion, Kirigami text scale, accessibility flags, and breeze-dark icon set**

## Performance

- **Duration:** 14 min
- **Started:** 2026-03-27T11:48:47Z
- **Completed:** 2026-03-27T12:02:58Z
- **Tasks:** 3
- **Files modified:** 4

## Accomplishments

- Complete rewrite of KDE reader from old ThemeColors/ThemeFonts/WidgetMetrics types to per-widget ThemeVariant model
- Qt5/Qt6 font weight conversion with qt5_to_css_weight() mapping (0-100 scale -> CSS 100-900)
- Text scale computation from smallestReadableFont (caption) and Kirigami multipliers (heading 1.20x, dialog title 1.35x)
- Accessibility: AnimationDurationFactor=0 -> reduce_motion, forceFontDPI -> text_scaling_factor
- Integration test proving KDE output passes resolve() -> validate() -> Ok(ResolvedTheme)
- 112 KDE tests passing, zero old model type references

## Task Commits

Each task was committed atomically:

1. **Task 1: Rewrite kde/colors.rs and kde/fonts.rs** - `8af64b6` (feat)
2. **Task 2: Rewrite kde/mod.rs with text_scale, accessibility, icons** - `2671421` (feat)
3. **Task 3: Integration test resolve/validate** - `8b6313a` (test, included in parallel agent commit)

## Files Created/Modified

- `native-theme/src/kde/colors.rs` - populate_colors() targeting ThemeVariant with defaults + 8 per-widget color groups + WM title bar + Header
- `native-theme/src/kde/fonts.rs` - parse_qt_font_with_weight() with Qt5/Qt6 detection, populate_fonts() for defaults + menu + toolbar + title bar fonts
- `native-theme/src/kde/metrics.rs` - populate_widget_sizing() setting breeze constants directly on per-widget structs
- `native-theme/src/kde/mod.rs` - from_kde_content() orchestrating all modules + text_scale + accessibility + icon_set + dialog button_order

## Decisions Made

- KDE reader integration test merges output with default preset before resolve/validate, matching real-world usage pattern (sparse reader + preset = complete theme)
- Icon sizes from index.theme parsing deferred -- the icon_set name (breeze-dark) is sufficient for runtime icon lookup; parsing freedesktop icon directories adds filesystem complexity for marginal value
- dialog.button_order set to LeadingAffirmative per existing project decision (KDE/macOS style)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Known Stubs

None - all planned fields are wired to real data sources.

## Next Phase Readiness

- KDE reader complete and tested; ready for GNOME reader (plan 46-02), macOS (46-03), and Windows (46-04)
- Integration test pattern established for other OS readers to follow

---
*Phase: 46-os-reader-extensions*
*Plan: 01*
*Completed: 2026-03-27*
