---
phase: 55-correctness-safety-ci
plan: 03
subsystem: ci
tags: [github-actions, publish, async-io, examples, pre-release]

requires:
  - phase: 54-connector-migration
    provides: gpui and iced connector examples
provides:
  - gpui connector in publish CI gate (clippy + test)
  - publish error handling (already-published detection, no silent failures)
  - async-io runtime variant CI coverage (portal-async-io, linux-async-io)
  - disambiguated example names (showcase-gpui, showcase-iced)
  - pre-release CI wait timeout guard (30 minutes)
affects: [publish workflow, CI pipeline, pre-release automation]

tech-stack:
  added: []
  patterns: ["already-published detection pattern for cargo publish re-runs"]

key-files:
  created: []
  modified:
    - .github/workflows/publish.yml
    - .github/workflows/ci.yml
    - .github/workflows/screenshots.yml
    - connectors/native-theme-gpui/Cargo.toml
    - connectors/native-theme-gpui/examples/showcase-gpui.rs
    - connectors/native-theme-gpui/README.md
    - connectors/native-theme-iced/Cargo.toml
    - connectors/native-theme-iced/examples/showcase-iced.rs
    - connectors/native-theme-iced/README.md
    - scripts/pre-release.sh
    - scripts/generate_screenshots.sh
    - scripts/generate_gpui_screenshots.sh
    - scripts/generate_theme_switching_gif.sh
    - README.md

key-decisions:
  - "Publish error handling uses grep -qi for case-insensitive matching of 'already uploaded' or 'already published' from cargo stderr"
  - "Pre-release timeout set to 180 iterations * 10s = 30 minutes, using fail() for consistent error reporting"
  - "Screenshots workflow uses matrix.toolkit variable in example name (showcase-${{ matrix.toolkit }}) for clean dynamic references"

patterns-established:
  - "Cargo publish error handling: capture stderr, check for already-published, fail on real errors"
  - "Example naming convention: showcase-{toolkit} to avoid binary name collisions across connectors"

requirements-completed: [CI-01, CI-02, CI-03, CI-04, CI-05]

duration: 5min
completed: 2026-04-07
---

# Phase 55 Plan 03: CI/Publishing Gaps Summary

**Publish CI gate with gpui connector, already-published error handling, async-io test variants, disambiguated example names, and pre-release timeout guard**

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-07T15:32:14Z
- **Completed:** 2026-04-07T15:37:30Z
- **Tasks:** 2
- **Files modified:** 14

## Accomplishments
- Publish workflow CI gate now tests gpui connector (clippy + test) with system deps before publishing
- All four publish steps detect "already published" errors and succeed gracefully, but fail on real errors (no more silent continue-on-error)
- CI matrix expanded with portal-async-io and linux-async-io feature test variants
- Example names disambiguated: showcase-gpui and showcase-iced (no binary name collision)
- Pre-release CI wait loop has 30-minute timeout guard preventing infinite hangs

## Task Commits

Each task was committed atomically:

1. **Task 1: Publish workflow CI gate and error handling** - `df2e6ad` (feat)
2. **Task 2: CI async-io variants, example rename, pre-release timeout** - `bf00490` (feat)

## Files Created/Modified
- `.github/workflows/publish.yml` - gpui CI gate, already-published detection for all 4 publish steps
- `.github/workflows/ci.yml` - portal-async-io and linux-async-io test matrix entries
- `.github/workflows/screenshots.yml` - Updated example name references to use matrix.toolkit
- `connectors/native-theme-gpui/Cargo.toml` - Example renamed to showcase-gpui
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - Renamed from showcase.rs
- `connectors/native-theme-gpui/README.md` - Updated example run command
- `connectors/native-theme-iced/Cargo.toml` - Example renamed to showcase-iced
- `connectors/native-theme-iced/examples/showcase-iced.rs` - Renamed from showcase.rs
- `connectors/native-theme-iced/README.md` - Updated example run command
- `scripts/pre-release.sh` - 30-minute timeout guard on CI wait loop
- `scripts/generate_screenshots.sh` - Updated example name to showcase-iced
- `scripts/generate_gpui_screenshots.sh` - Updated example name to showcase-gpui
- `scripts/generate_theme_switching_gif.sh` - Updated both iced and gpui example names
- `README.md` - Updated both example run commands in root README

## Decisions Made
- Publish error detection uses `grep -qi "already.*uploaded\|already.*published"` for case-insensitive matching of both crates.io error message variants
- Pre-release timeout: 180 iterations at 10s sleep = 30 minutes, using the existing `fail()` function for consistent error reporting
- Screenshots workflow uses `showcase-${{ matrix.toolkit }}` pattern for dynamic example name resolution

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Task 2 changes already committed by prior plan executor**
- **Found during:** Task 2 commit
- **Issue:** The 55-02 plan executor had already committed Task 2 changes (ci.yml, example renames, scripts, pre-release.sh) in its summary commit (bf00490)
- **Fix:** Verified all changes are correct and present in git history; no additional commit needed
- **Files modified:** None (already committed)

---

**Total deviations:** 1 (commit attribution overlap with prior plan)
**Impact on plan:** No functional impact. All five CI requirements (CI-01 through CI-05) are correctly implemented and verified.

## Issues Encountered
None beyond the commit overlap noted above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All CI/publishing gaps closed
- Phase 55 plan 03 requirements complete
- Workflows syntactically validated (YAML parsing, bash -n)
- Clippy passes for both connectors with renamed examples

---
*Phase: 55-correctness-safety-ci*
*Completed: 2026-04-07*

## Self-Check: PASSED

- All 13 modified files verified present
- Old showcase.rs files verified removed
- Both task commits verified in git history (df2e6ad, bf00490)
