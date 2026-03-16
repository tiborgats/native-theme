---
phase: 23-build-crate-and-code-generation
plan: 05
subsystem: icons
tags: [build-crate, codegen, include-bytes, path-resolution, gap-closure]

# Dependency graph
requires:
  - phase: 23-build-crate-and-code-generation
    plan: 04
    provides: "run_pipeline(), generate_icons(), IconGenerator::generate() with absolute base_dir bug"
provides:
  - "Fixed include_bytes! path generation: relative base_dir via manifest_dir stripping"
  - "manifest_dir parameter on run_pipeline for portable codegen paths"
affects: [24-de-aware-codegen]

# Tech tracking
tech-stack:
  added: []
  patterns: ["strip_prefix pattern for converting absolute paths to manifest-relative paths in codegen"]

key-files:
  created: []
  modified:
    - "native-theme-build/src/lib.rs"

key-decisions:
  - "Added manifest_dir: Option<&Path> parameter to run_pipeline rather than reading CARGO_MANIFEST_DIR env var inside, preserving the pure pipeline core design"
  - "strip_prefix applied only in codegen path (base_dir_str computation), file I/O continues using absolute base_dir for correctness"

patterns-established:
  - "Manifest-relative codegen: callers pass manifest_dir to run_pipeline for stripping, keeping codegen paths portable across consumer crate locations"

requirements-completed: [BUILD-05, BUILD-08]

# Metrics
duration: 4min
completed: 2026-03-16
---

# Phase 23 Plan 05: Fix include_bytes! Path Generation Summary

**Fixed include_bytes! codegen to emit relative paths via manifest_dir prefix stripping, closing the last Phase 23 gap**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-16T00:21:34Z
- **Completed:** 2026-03-16T00:26:27Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments
- Fixed the include_bytes! path bug: codegen now produces `concat!(env!("CARGO_MANIFEST_DIR"), "/icons/material/play_pause.svg")` instead of `concat!(env!("CARGO_MANIFEST_DIR"), "//absolute/path/icons/material/play_pause.svg")`
- Added `manifest_dir: Option<&Path>` to `run_pipeline()` for prefix stripping while preserving pure pipeline design
- Both `generate_icons()` and `IconGenerator::generate()` now pass their manifest_dir for correct relative path codegen
- Added regression test `pipeline_generates_relative_include_bytes_paths` confirming the fix
- All 81 tests pass (68 unit + 13 integration), zero warnings

## Task Commits

Each task was committed atomically (TDD: RED + GREEN):

1. **Task 1 (RED): Failing test for relative include_bytes! paths** - `126cb68` (test)
2. **Task 1 (GREEN): Fix base_dir to be relative via manifest_dir stripping** - `df8ac3f` (feat)

## Files Created/Modified
- `native-theme-build/src/lib.rs` - Added manifest_dir parameter to run_pipeline, strip_prefix in codegen path, generate_icons/IconGenerator pass manifest_dir, new regression test

## Decisions Made
- **manifest_dir as parameter, not env var:** Added `manifest_dir: Option<&Path>` to `run_pipeline()` instead of reading `CARGO_MANIFEST_DIR` inside the function. This preserves the pure pipeline core design (no side effects, no env var access). The callers (`generate_icons`, `IconGenerator::generate`) already have `manifest_dir` from their own `CARGO_MANIFEST_DIR` reads.
- **strip_prefix only for codegen:** The absolute `base_dir` is still used for file I/O (reading mapping.toml, validating SVGs). Only the `base_dir_str` passed to `generate_code()` is stripped to a relative path. This avoids breaking file resolution while fixing the codegen output.

## Deviations from Plan

None - plan executed with a minor implementation refinement (manifest_dir as parameter to run_pipeline rather than strip_prefix in callers) which achieves the same result more cleanly.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Phase 23 (Build Crate and Code Generation) gap closure complete
- All include_bytes! paths now resolve correctly for any consumer crate directory
- Ready for Phase 24 (DE-Aware Codegen)
- All 81 tests provide full regression safety

## Self-Check: PASSED

All artifacts verified:
- native-theme-build/src/lib.rs exists on disk
- Commit 126cb68 found in git log
- Commit df8ac3f found in git log
- 81 tests pass (68 unit + 13 integration) via `cargo test -p native-theme-build`
- Zero compiler warnings

---
*Phase: 23-build-crate-and-code-generation*
*Completed: 2026-03-16*
