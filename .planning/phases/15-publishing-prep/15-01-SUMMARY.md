---
phase: 15-publishing-prep
plan: 01
subsystem: infra
tags: [cargo, crates-io, publishing, license, metadata]

# Dependency graph
requires:
  - phase: 09-cargo-workspace
    provides: workspace structure with member crates
provides:
  - crates.io metadata on all publishable crates
  - versioned native-theme workspace dependency for iced connector
  - MIT, Apache-2.0, and 0BSD license files at repo root
  - iced connector README for crates.io display
  - gpui connector publish exclusion
affects: [15-02, 15-03, 15-04]

# Tech tracking
tech-stack:
  added: []
  patterns: [workspace metadata inheritance, versioned workspace dependencies]

key-files:
  created:
    - LICENSE-MIT
    - LICENSE-APACHE
    - LICENSE-0BSD
    - connectors/native-theme-iced/README.md
  modified:
    - Cargo.toml
    - native-theme/Cargo.toml
    - connectors/native-theme-iced/Cargo.toml
    - connectors/native-theme-gpui/Cargo.toml

key-decisions:
  - "Repository URL uses tiborgats org placeholder (no git remote configured); user should update before publishing"
  - "native-theme-iced dry-run fails because native-theme is not on crates.io yet; expected, resolves after native-theme is published first"

patterns-established:
  - "Workspace metadata inheritance: rust-version, repository, homepage inherited via .workspace = true"
  - "Crate-specific keywords/categories: iced connector uses its own keywords rather than workspace defaults"

requirements-completed: [PUB-01, PUB-02]

# Metrics
duration: 3min
completed: 2026-03-09
---

# Phase 15 Plan 01: Publishing Prep Metadata Summary

**Cargo.toml metadata for crates.io with workspace inheritance, versioned dependency, triple license files, and iced README**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-09T02:39:04Z
- **Completed:** 2026-03-09T02:42:13Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- Both native-theme and native-theme-iced have complete crates.io metadata (rust-version, repository, homepage, keywords, categories, readme)
- Versioned workspace dependency (`version = "0.2.0"`) enables crates.io resolution for the iced connector
- Three license files (MIT, Apache-2.0, 0BSD) created at repo root matching the SPDX expression
- gpui connector marked `publish = false` to prevent accidental publishing

## Task Commits

Each task was committed atomically:

1. **Task 1: Add workspace metadata and fix dependency versioning** - `08ef1a6` (feat)
2. **Task 2: Create license files** - `c3a138f` (chore)

## Files Created/Modified
- `Cargo.toml` - Added workspace metadata fields and versioned native-theme dependency
- `native-theme/Cargo.toml` - Inherited workspace metadata fields, added readme
- `connectors/native-theme-iced/Cargo.toml` - Inherited workspace fields, added crate-specific keywords/categories, readme
- `connectors/native-theme-gpui/Cargo.toml` - Added publish = false
- `connectors/native-theme-iced/README.md` - Crate README with usage example and widget metrics docs
- `LICENSE-MIT` - MIT license text
- `LICENSE-APACHE` - Apache License 2.0 full text
- `LICENSE-0BSD` - Zero-Clause BSD license text

## Decisions Made
- Repository URL uses "tiborgats" as a placeholder organization (no git remote configured). User should update `repository` and `homepage` in workspace Cargo.toml before actual publishing.
- native-theme-iced `cargo publish --dry-run` fails because native-theme is not yet on crates.io. This is expected behavior for workspace dependencies -- it will resolve once native-theme is published first. The versioned dependency (`^0.2.0`) is correctly configured.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- native-theme passes `cargo publish --dry-run` cleanly
- native-theme-iced metadata is complete; will pass dry-run after native-theme is published
- License files in place for all three licenses in the SPDX expression
- Ready for remaining publishing prep plans (CHANGELOG, CI publish workflow, etc.)

## Self-Check: PASSED

- All 4 created files verified on disk
- All 2 task commits verified in git log (08ef1a6, c3a138f)

---
*Phase: 15-publishing-prep*
*Completed: 2026-03-09*
