---
phase: 17-bundled-svg-icons
plan: 01
subsystem: icons
tags: [svg, material-symbols, lucide, icon-assets, apache-2.0, isc-license]

# Dependency graph
requires:
  - phase: 16-icon-data-model
    provides: IconRole enum defining all 42 icon variants
provides:
  - 38 Material Symbols SVGs (outlined, weight 400) in native-theme/icons/material/
  - 38 Lucide SVGs in native-theme/icons/lucide/
  - Apache 2.0 license for Material Symbols
  - ISC license for Lucide Icons
affects: [17-02-PLAN (include_bytes! embedding), 21-icon-connectors]

# Tech tracking
tech-stack:
  added: []
  patterns: [raw SVG assets checked into repo for hermetic builds]

key-files:
  created:
    - native-theme/icons/material/ (38 SVGs)
    - native-theme/icons/lucide/ (38 SVGs)
    - native-theme/icons/LICENSE-MATERIAL.txt
    - native-theme/icons/LICENSE-LUCIDE.txt
  modified: []

key-decisions:
  - "38 unique files per icon set (not 32/33 as originally estimated) -- each distinct icon name gets its own file; role-to-file deduplication happens at include_bytes! level"
  - "circle-question-mark.svg exists directly in Lucide repo, no fallback naming needed"
  - "Material Symbols: outlined style, weight 400 from marella/material-symbols"

patterns-established:
  - "Icon assets stored in native-theme/icons/{set-name}/*.svg"
  - "License files stored alongside icons as LICENSE-{SET}.txt"

requirements-completed: [BNDL-01, BNDL-02]

# Metrics
duration: 2min
completed: 2026-03-09
---

# Phase 17 Plan 01: Download SVG Icons Summary

**76 SVG icon files (38 Material Symbols + 38 Lucide) downloaded from upstream repos with Apache 2.0 and ISC licenses**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-09T07:25:11Z
- **Completed:** 2026-03-09T07:27:12Z
- **Tasks:** 2
- **Files modified:** 78

## Accomplishments
- Downloaded 38 Material Symbols SVGs (outlined, weight 400) from marella/material-symbols
- Downloaded 38 Lucide SVGs from lucide-icons/lucide
- Both license files obtained (Apache 2.0 for Material, ISC for Lucide)
- All 76 SVGs verified to contain valid `<svg` tags (no 404 pages)

## Task Commits

Each task was committed atomically:

1. **Task 1: Download Material Symbols SVGs and license** - `bb4e53a` (feat)
2. **Task 2: Download Lucide SVGs and license** - `d9b5628` (feat)

## Files Created/Modified
- `native-theme/icons/material/*.svg` - 38 Material Symbols outlined weight-400 SVGs
- `native-theme/icons/lucide/*.svg` - 38 Lucide SVGs (stroke-based, 24x24 viewBox)
- `native-theme/icons/LICENSE-MATERIAL.txt` - Apache 2.0 license from Google/marella
- `native-theme/icons/LICENSE-LUCIDE.txt` - ISC license from Lucide Contributors

## Decisions Made
- Used 38 unique files per icon set rather than the plan's estimated 32/33. The plan counted unique files after role deduplication but the download list contained 38 distinct icon names per set, which is correct -- multiple IconRole variants can reference the same file via include_bytes! in Plan 02.
- circle-question-mark.svg downloaded directly from Lucide repo without needing any of the planned fallback names (help-circle, circle-help, message-circle-question).
- Material Symbols: outlined style, weight 400 (the default Google Fonts UI selection).

## Deviations from Plan

None - plan executed exactly as written. The only discrepancy is that the plan's verification section stated "32" Material and "33" Lucide unique files, but the plan's own download lists (and frontmatter files_modified) specify 38 distinct filenames for each set. All 38 per set were downloaded successfully.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All SVG files on disk, ready for Plan 02 to use include_bytes! macros
- License files in place for compliance
- File paths match the include_bytes! patterns documented in 17-RESEARCH.md

## Self-Check: PASSED

- [x] native-theme/icons/material/ - 38 SVGs present
- [x] native-theme/icons/lucide/ - 38 SVGs present
- [x] native-theme/icons/LICENSE-MATERIAL.txt exists
- [x] native-theme/icons/LICENSE-LUCIDE.txt exists
- [x] Commit bb4e53a found (Task 1)
- [x] Commit d9b5628 found (Task 2)

---
*Phase: 17-bundled-svg-icons*
*Completed: 2026-03-09*
