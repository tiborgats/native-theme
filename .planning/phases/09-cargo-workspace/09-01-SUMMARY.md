---
phase: 09-cargo-workspace
plan: 01
subsystem: infra
tags: [cargo, workspace, monorepo, virtual-workspace, resolver-v3]

# Dependency graph
requires: []
provides:
  - "Cargo virtual workspace with 3 members (native-theme, native-theme-gpui, native-theme-iced)"
  - "Workspace dependency inheritance for shared deps (serde, serde_with, toml)"
  - "Connector stub crates ready for Phase 14 implementation"
affects: [10-api-refinement, 14-toolkit-connectors]

# Tech tracking
tech-stack:
  added: []
  patterns: [cargo-workspace-inheritance, virtual-workspace-manifest, resolver-v3]

key-files:
  created:
    - Cargo.toml (workspace root)
    - native-theme/Cargo.toml
    - native-theme/README.md
    - connectors/native-theme-gpui/Cargo.toml
    - connectors/native-theme-gpui/src/lib.rs
    - connectors/native-theme-iced/Cargo.toml
    - connectors/native-theme-iced/src/lib.rs
  modified:
    - Cargo.lock

key-decisions:
  - "Single atomic commit for entire restructuring to preserve git blame via git mv"
  - "Virtual workspace with resolver v3 (required explicitly for edition 2024 without package section)"
  - "Workspace dependency inheritance for serde, serde_with, toml shared across members"
  - "Connector stubs in connectors/ subdirectory with workspace dep on core crate"

patterns-established:
  - "Workspace inheritance: shared deps defined in root [workspace.dependencies], members use {dep}.workspace = true"
  - "Connector naming: native-theme-{toolkit} in connectors/ directory"
  - "Crate-local README.md for doc-tests (include_str! resolves relative to src/)"

requirements-completed: [API-01]

# Metrics
duration: 3min
completed: 2026-03-08
---

# Phase 9 Plan 1: Cargo Workspace Summary

**Virtual workspace with 3 members, resolver v3, workspace dep inheritance, all 137 tests passing with git blame preserved via git mv**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-08T04:50:03Z
- **Completed:** 2026-03-08T04:53:58Z
- **Tasks:** 2
- **Files modified:** 42

## Accomplishments
- Restructured single-crate repo into Cargo virtual workspace with 3 members
- Core crate moved to native-theme/ subdirectory with workspace-inherited metadata (edition, license, serde, serde_with, toml)
- Connector stub crates created for gpui and iced with workspace dependency on core
- All 137 tests pass (98 unit + 30 integration + 9 doc-tests, plus 3 ignored doc-tests)
- Git history fully preserved via git mv (100% rename similarity for all 35 source files)

## Task Commits

Both tasks were committed as a single atomic commit per plan instructions:

1. **Task 1: Move core crate into subdirectory and create workspace manifests** - `d3f80fe` (refactor)
2. **Task 2: Create connector stubs and commit atomic restructure** - `d3f80fe` (refactor)

_Note: Plan specified both tasks share a single atomic commit to preserve git mv history._

## Files Created/Modified
- `Cargo.toml` - Virtual workspace manifest with [workspace] members list and shared deps
- `native-theme/Cargo.toml` - Core crate manifest with workspace inheritance (edition.workspace, license.workspace, serde.workspace, etc.)
- `native-theme/README.md` - Crate-local README for include_str! doc-tests
- `native-theme/src/` - All core source files (moved from src/ via git mv)
- `native-theme/tests/` - All integration tests (moved from tests/ via git mv)
- `connectors/native-theme-gpui/Cargo.toml` - gpui connector stub manifest
- `connectors/native-theme-gpui/src/lib.rs` - gpui connector stub entry point
- `connectors/native-theme-iced/Cargo.toml` - iced connector stub manifest
- `connectors/native-theme-iced/src/lib.rs` - iced connector stub entry point
- `Cargo.lock` - Updated for workspace structure

## Decisions Made
- Single atomic commit for the entire restructure (both tasks) to ensure git mv renames are tracked in one changeset
- Used `resolver = "3"` explicitly in workspace root (required for virtual workspaces with edition 2024)
- Workspace dependency inheritance for serde, serde_with, toml (shared across core and future connectors)
- Connector stubs placed in `connectors/` subdirectory following `native-theme-{toolkit}` naming convention
- Crate-local README.md created in native-theme/ for doc-test resolution (include_str!("../README.md") resolves correctly)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- Pre-existing unstaged version bump (v0.1.0 -> v0.2.0) in Cargo.toml and Cargo.lock needed to be preserved through the git mv. Resolved by stashing before git mv, then popping after move.
- Workspace root Cargo.toml references connector stubs that don't exist until Task 2, so `cargo check` in Task 1's verification step was deferred. Connector stubs created in Task 2 Step 1-3 before running verification.
- Stale `native-theme/Cargo.lock` appeared after cargo build (workspace generates Cargo.lock at root). Removed the subcrate copy.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Workspace structure complete, ready for Phase 10 (API Refinement)
- Connector stubs compiled successfully, ready for Phase 14 (Toolkit Connectors) implementation
- All existing tests passing, no regressions introduced

---
*Phase: 09-cargo-workspace*
*Completed: 2026-03-08*

## Self-Check: PASSED

- All 9 key files verified present on disk
- Commit d3f80fe verified in git log
