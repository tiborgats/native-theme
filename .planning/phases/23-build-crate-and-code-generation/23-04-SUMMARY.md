---
phase: 23-build-crate-and-code-generation
plan: 04
subsystem: icons
tags: [build-crate, codegen, public-api, pipeline, builder-pattern, integration-test]

# Dependency graph
requires:
  - phase: 23-build-crate-and-code-generation
    plan: 02
    provides: "validate_themes, validate_mapping, validate_svgs, check_orphan_svgs, validate_no_duplicate_roles"
  - phase: 23-build-crate-and-code-generation
    plan: 03
    provides: "generate_code() function producing Rust source from MasterConfig + mappings"
provides:
  - "generate_icons() public API for single TOML file pipeline"
  - "IconGenerator builder API for composing multiple TOML files"
  - "run_pipeline() pure core function returning (code, errors, warnings) for testability"
  - "merge_configs() for unioning themes and concatenating roles across files"
  - "emit_result() thin I/O layer for cargo directives and file output"
  - "Full integration test suite with committed fixture files"
affects: [24-de-aware-codegen]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Pure pipeline core pattern: validation+codegen returns data, thin layer does I/O", "Builder pattern with move semantics for composing TOML sources", "doc(hidden) public API for integration test access to internal pipeline"]

key-files:
  created:
    - "native-theme-build/tests/integration.rs"
    - "native-theme-build/tests/fixtures/sample-icons.toml"
    - "native-theme-build/tests/fixtures/material/mapping.toml"
    - "native-theme-build/tests/fixtures/material/play_pause.svg"
    - "native-theme-build/tests/fixtures/material/skip_next.svg"
    - "native-theme-build/tests/fixtures/sf-symbols/mapping.toml"
  modified:
    - "native-theme-build/src/lib.rs"
    - "native-theme-build/src/schema.rs"

key-decisions:
  - "Pure pipeline core: run_pipeline() returns PipelineResult with code/errors/warnings/rerun_paths/size_report, no I/O"
  - "Merge-then-validate: builder API merges configs before validation so shared mappings validate against the full merged role set"
  - "doc(hidden) pub for test access: MasterConfig, PipelineResult, SizeReport, run_pipeline, __run_pipeline_on_files exposed as doc-hidden for integration tests"

patterns-established:
  - "Pipeline core pattern: pure function returns all outputs as data, outer layer handles I/O (println cargo directives, fs::write, process::exit)"
  - "Builder API pattern: IconGenerator::new().add(path).add(path).enum_name(name).generate() with move-self chaining"
  - "Integration test fixtures: committed TOML + SVG stubs in tests/fixtures/ with helper __run_pipeline_on_files()"

requirements-completed: [BUILD-01, BUILD-02, BUILD-09, BUILD-10]

# Metrics
duration: 8min
completed: 2026-03-15
---

# Phase 23 Plan 04: Public API Pipeline and Integration Tests Summary

**generate_icons() and IconGenerator builder wiring validation+codegen into a complete pipeline with 80 tests (67 unit + 13 integration) covering happy paths, error paths, and multi-file composition**

## Performance

- **Duration:** 8 min
- **Started:** 2026-03-15T22:12:02Z
- **Completed:** 2026-03-15T22:20:02Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- Replaced placeholder stubs with full generate_icons() and IconGenerator builder implementations
- Extracted pure run_pipeline() core returning (code, errors, warnings, rerun_paths, size_report) for testability
- IconGenerator builder composes multiple TOML files: union of themes, concatenated roles, duplicate role detection
- Added 12 new unit tests for pipeline logic (happy path, error paths, merge, builder, size report, rerun paths)
- Created 13 integration tests using committed fixture files: enum shape, IconProvider arms, error messages, builder merge/conflict
- Total test count: 80 (67 unit + 13 integration), zero warnings

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement generate_icons and IconGenerator public API** - `2d83882` (feat)
2. **Task 2: Add integration test with fixture TOML and SVG files** - `91b86e7` (feat)

## Files Created/Modified
- `native-theme-build/src/lib.rs` - Full pipeline: run_pipeline(), generate_icons(), IconGenerator, emit_result(), merge_configs(), PipelineResult, SizeReport
- `native-theme-build/src/schema.rs` - MasterConfig visibility changed to doc(hidden) pub for integration test access
- `native-theme-build/tests/integration.rs` - 13 integration tests covering happy path, errors, and builder API
- `native-theme-build/tests/fixtures/sample-icons.toml` - Master TOML fixture with 2 roles, material bundled, sf-symbols system
- `native-theme-build/tests/fixtures/material/mapping.toml` - Material theme mapping fixture
- `native-theme-build/tests/fixtures/material/play_pause.svg` - Minimal SVG stub
- `native-theme-build/tests/fixtures/material/skip_next.svg` - Minimal SVG stub
- `native-theme-build/tests/fixtures/sf-symbols/mapping.toml` - SF Symbols theme mapping fixture

## Decisions Made
- **Pure pipeline core:** run_pipeline() returns all outputs as data (PipelineResult struct). The thin outer layer (emit_result) handles println! cargo directives, fs::write to OUT_DIR, and process::exit(1). This makes the core fully testable without stdout capture.
- **Merge-then-validate for builder API:** When multiple configs share a theme, the pipeline first merges all roles across configs, then validates the mapping against the merged role set. This prevents false "unknown role" errors when a shared mapping file contains entries from multiple config files.
- **doc(hidden) public visibility:** MasterConfig, PipelineResult, SizeReport, and run_pipeline are exposed as `#[doc(hidden)] pub` so integration tests can access the pure pipeline core. The __run_pipeline_on_files() convenience helper loads TOMLs and calls run_pipeline.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed builder validation using per-config roles instead of merged roles**
- **Found during:** Task 1 (pipeline_builder_merges_two_files test)
- **Issue:** When two configs share a theme, validation checked each config's individual roles against the shared mapping, causing false "unknown role" errors for roles declared in the other config
- **Fix:** Restructured run_pipeline() to merge configs first, then validate with the merged role set
- **Files modified:** native-theme-build/src/lib.rs
- **Verification:** pipeline_builder_merges_two_files test passes
- **Committed in:** 2d83882

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Necessary correctness fix for multi-file builder API. No scope creep.

## Issues Encountered

None beyond the auto-fixed builder validation bug.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Phase 23 (Build Crate and Code Generation) is now complete
- generate_icons() and IconGenerator::generate() are ready for use in downstream build.rs files
- Pure pipeline core (run_pipeline) available for testing and future Phase 24 DE-aware dispatch work
- All 80 tests provide regression safety for future codegen changes

## Self-Check: PASSED

All artifacts verified:
- All 8 files exist on disk
- Commit 2d83882 found in git log
- Commit 91b86e7 found in git log
- 80 tests pass (67 unit + 13 integration) via `cargo test -p native-theme-build`
- Zero compiler warnings

---
*Phase: 23-build-crate-and-code-generation*
*Completed: 2026-03-15*
