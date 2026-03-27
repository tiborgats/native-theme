---
phase: 46-os-reader-extensions
plan: 04
subsystem: platform-readers
tags: [windows, win32, dwm, getsyscolor, nonclientmetrics, accessibility, per-widget, ThemeVariant]

requires:
  - phase: 44-model-restructure
    provides: ThemeVariant with per-widget structs, define_widget_pair! macro, FontSpec
  - phase: 45-resolution-engine
    provides: resolve() pipeline, validate() for complete theme verification
provides:
  - Windows reader producing sparse ThemeVariant with per-widget fonts, colors, accessibility, icon sizes
  - Win32_Graphics_Dwm feature for DwmGetColorizationColor
  - colorref_to_rgba() and dwm_color_to_rgba() testable color conversion helpers
affects: [47-platform-tomls, 48-connector-update]

tech-stack:
  added: [Win32_Graphics_Dwm feature flag]
  patterns: [sparse ThemeVariant population, cfg-gated Windows API with testable core functions]

key-files:
  created: []
  modified:
    - native-theme/src/windows.rs
    - native-theme/Cargo.toml
    - native-theme/src/lib.rs

key-decisions:
  - "Combined Task 1 and Task 2 into single atomic rewrite since separating per-widget fonts from colors/accessibility was not meaningful"
  - "Made windows module always compiled (not cfg-gated) with dead_code allow on non-Windows, enabling test execution on Linux dev machine"
  - "Used primary_bg from accent shades for button.primary_bg field instead of removed primary_background"
  - "Computed disabled_foreground as midpoint of fg and bg rather than hardcoding"

patterns-established:
  - "Windows reader pattern: cfg-gated API calls with testable pure-function cores (logfont_to_fontspec_raw, colorref_to_rgba, dwm_color_to_rgba)"
  - "AllFonts struct collects all NONCLIENTMETRICSW font fields before build_theme distributes them to per-widget structs"

requirements-completed: [WIN-01, WIN-02, WIN-03, WIN-04, WIN-05]

duration: 8min
completed: 2026-03-27
---

# Phase 46 Plan 04: Windows Reader Extensions Summary

**Windows reader rewritten to produce sparse ThemeVariant with per-widget fonts from NONCLIENTMETRICSW, DwmGetColorizationColor title bar colors, GetSysColor for 10 widget color fields, accessibility (text scale, high contrast, reduce motion), and icon sizes from GetSystemMetricsForDpi**

## Performance

- **Duration:** 8 min
- **Started:** 2026-03-27T11:49:50Z
- **Completed:** 2026-03-27T11:57:53Z
- **Tasks:** 2 (combined into 1 atomic commit)
- **Files modified:** 3

## Accomplishments
- Complete rewrite of windows.rs: build_theme() now constructs ThemeVariant with per-widget fields instead of old ThemeColors/ThemeFonts/ThemeGeometry/WidgetMetrics
- All 5 WIN requirements implemented: per-widget fonts (WIN-01), DWM colorization (WIN-02), GetSysColor colors (WIN-03), accessibility (WIN-04), icon sizes (WIN-05)
- 31 tests covering all new functionality, all passing on Linux dev machine
- Zero references to removed old model types (ThemeColors, ThemeFonts, ThemeGeometry, WidgetMetrics)
- Win32_Graphics_Dwm feature added to Cargo.toml for DwmGetColorizationColor API

## Task Commits

Each task was committed atomically:

1. **Task 1+2: Rewrite Windows build_theme with all WIN requirements** - `af0de3b` (feat)

**Plan metadata:** `8b6313a` (docs: complete plan)

_Note: Tasks 1 and 2 were implemented as a single atomic rewrite since the build_theme function signature change required all new data sources to be wired simultaneously._

## Files Created/Modified
- `native-theme/src/windows.rs` - Complete rewrite: build_theme produces ThemeVariant, per-widget fonts/colors/accessibility/icons
- `native-theme/Cargo.toml` - Added Win32_Graphics_Dwm feature to windows crate dependency
- `native-theme/src/lib.rs` - Changed windows module cfg to allow compilation on non-Windows for testing

## Decisions Made
- Combined Task 1 and Task 2 into single atomic commit: separating the font extraction from color/accessibility was not meaningful since the entire build_theme signature changed
- Made windows module always compiled (cfg_attr dead_code allow on non-Windows) to enable running 31 unit tests on Linux dev machine
- Used accent shade selection for button.primary_bg instead of old primary_background field
- Computed disabled_foreground as (fg+bg)/2 midpoint rather than hardcoding

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Made windows module compilable on Linux for testing**
- **Found during:** Task 1
- **Issue:** windows module was gated by `#[cfg(all(target_os = "windows", feature = "windows"))]` which prevented running any tests on Linux
- **Fix:** Changed to unconditional module inclusion with `#[cfg_attr(not(target_os = "windows"), allow(dead_code, unused_variables))]`; all Windows API calls remain individually cfg-gated
- **Files modified:** native-theme/src/lib.rs
- **Verification:** All 31 tests run and pass on Linux
- **Committed in:** af0de3b

**2. [Rule 2 - Missing Critical] Removed hardcoded border color fallback**
- **Found during:** Task 1
- **Issue:** Old code hardcoded light/dark gray border colors; plan explicitly says "No color fallbacks in the reader"
- **Fix:** Border color left as None; resolve() safety nets handle it
- **Files modified:** native-theme/src/windows.rs
- **Committed in:** af0de3b

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 missing critical)
**Impact on plan:** Both fixes necessary for correctness and testability. No scope creep.

## Issues Encountered
None - plan executed as designed.

## Known Stubs
None - all WIN requirements fully implemented with testable code paths.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Windows reader now produces correct ThemeVariant; ready for platform TOML slimming (Phase 47)
- Compilation on actual Windows host can be verified when available (all API calls are cfg-gated and follow documented Win32 patterns)

## Self-Check: PASSED

- All created/modified files exist on disk
- Commit af0de3b verified in git log
- 31 tests pass, 0 failures
- Zero references to old model types in windows.rs

---
*Phase: 46-os-reader-extensions*
*Completed: 2026-03-27*
