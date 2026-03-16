---
phase: 24-linux-de-audit-and-freedesktop-de-aware-mapping
plan: 01
subsystem: codegen
tags: [linux, desktop-environment, codegen, cfg-gating, freedesktop]

requires:
  - phase: 23-build-crate-and-code-generation
    provides: "generate_icon_name() with MappingValue::DeAware default_name() fallback"
provides:
  - "de_key_to_variant() mapping 7 DE keys to LinuxDesktop variant paths"
  - "cfg-gated DE dispatch in generate_icon_name() for DeAware values"
  - "optimization: DeAware with only default key collapses to simple arm"
affects: [24-02, integration-tests, custom-icon-roles]

tech-stack:
  added: []
  patterns:
    - "cfg-gated code generation: #[cfg(target_os = linux)] / #[cfg(not(target_os = linux))] blocks in generated code"
    - "de_key_to_variant mapping table for TOML-to-LinuxDesktop variant resolution"

key-files:
  created: []
  modified:
    - "native-theme-build/src/codegen.rs"

key-decisions:
  - "DE dispatch inside match arm (lazy detection), not outside (eager)"
  - "Fully-qualified paths in generated code: native_theme::detect_linux_de, native_theme::LinuxDesktop::*"
  - "DeAware with only default key optimizes to simple arm (no unnecessary cfg blocks)"
  - "Unknown DE keys in TOML return None from de_key_to_variant (silent, default covers them)"

patterns-established:
  - "cfg-gated code generation for platform-specific runtime dispatch"
  - "de_key_to_variant as the canonical TOML-to-LinuxDesktop mapping"

requirements-completed: [FDES-01, FDES-02, FDES-03]

duration: 2min
completed: 2026-03-16
---

# Phase 24 Plan 01: DE-Aware Code Generation Summary

**de_key_to_variant() mapping + cfg-gated DE dispatch in generate_icon_name() for runtime desktop environment selection**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-16T00:52:57Z
- **Completed:** 2026-03-16T00:54:57Z
- **Tasks:** 2 (TDD RED + GREEN)
- **Files modified:** 1

## Accomplishments
- Added de_key_to_variant() mapping all 7 recognized DE keys (kde, gnome, xfce, cinnamon, mate, lxqt, budgie) to fully-qualified LinuxDesktop variant paths
- Updated generate_icon_name() to emit cfg-gated DE dispatch blocks for DeAware values with DE-specific overrides
- DeAware values with only a default key (no DE overrides) optimize to simple match arms
- Simple MappingValue arms remain unchanged (no regression)
- Updated 2 Phase 23 tests and added 15 new tests (9 de_key_to_variant + 6 codegen)

## Task Commits

Each task was committed atomically:

1. **RED: Failing tests for DE-aware codegen** - `b6f8337` (test)
2. **GREEN: Implement DE-aware code generation** - `f8109a3` (feat)

_TDD plan: test then feat commits_

## Files Created/Modified
- `native-theme-build/src/codegen.rs` - Added de_key_to_variant(), updated generate_icon_name() with cfg-gated DE dispatch, 17 new/updated tests

## Decisions Made
- DE detection call placed inside match arm body (lazy) rather than outside (eager) -- only called when a DE-aware mapping is actually accessed
- Fully-qualified paths throughout generated code: `native_theme::detect_linux_de()`, `native_theme::LinuxDesktop::Kde`, etc.
- DeAware with only default key optimizes to simple arm -- no unnecessary cfg blocks or detect_linux_de calls
- Unknown DE keys (e.g., "cosmic") silently return None from de_key_to_variant; the mandatory default key covers all DEs

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- DE-aware codegen complete; generated icon_name() now dispatches per-DE on Linux
- Ready for Plan 02 (LNXDE-03 documentation tests for Hyprland/Sway/COSMIC)
- generate_icon_svg() intentionally unchanged -- SVG embedding uses default_name() since SVGs don't vary by DE

---
*Phase: 24-linux-de-audit-and-freedesktop-de-aware-mapping*
*Completed: 2026-03-16*
