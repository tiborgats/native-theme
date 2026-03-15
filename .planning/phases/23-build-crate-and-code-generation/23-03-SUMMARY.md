---
phase: 23-build-crate-and-code-generation
plan: 03
subsystem: icons
tags: [build-crate, codegen, icon-provider, include-bytes, heck]

# Dependency graph
requires:
  - phase: 23-build-crate-and-code-generation
    plan: 01
    provides: "MasterConfig, MappingValue, ThemeMapping schema types and KNOWN_THEMES constant"
provides:
  - "generate_code() function producing complete Rust enum + IconProvider impl as String"
  - "theme_name_to_icon_set() mapping 5 known theme names to fully-qualified IconSet paths"
  - "include_bytes! generation with CARGO_MANIFEST_DIR prefix for bundled SVGs"
  - "DE-aware mapping support using default value (Phase 24 adds DE dispatch)"
affects: [23-04]

# Tech tracking
tech-stack:
  added: []
  patterns: ["std::fmt::Write codegen with writeln! for generating Rust source", "heck::ToUpperCamelCase for kebab-to-PascalCase conversion in generated code"]

key-files:
  created:
    - "native-theme-build/src/codegen.rs"
  modified:
    - "native-theme-build/src/lib.rs"

key-decisions:
  - "generate_code returns String (not writes to file) for testability; Plan 04 writes to OUT_DIR"
  - "DE-aware values use default_name() in Phase 23; KDE/GNOME-specific arms deferred to Phase 24"
  - "Private helper functions generate_icon_name and generate_icon_svg keep generate_code readable"

patterns-established:
  - "Codegen pattern: build String via writeln! with match arms for (Self::Variant, IconSet) tuples"
  - "SVG path pattern: concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{base_dir}/{theme}/{icon_name}.svg\")"

requirements-completed: [BUILD-05, BUILD-06, BUILD-07, BUILD-08]

# Metrics
duration: 3min
completed: 2026-03-15
---

# Phase 23 Plan 03: Code Generation for Enum and IconProvider Summary

**generate_code() producing Rust enum with derives, const ALL, IconProvider impl with icon_name/icon_svg match arms, and include_bytes! SVG embedding using CARGO_MANIFEST_DIR**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-15T22:02:46Z
- **Completed:** 2026-03-15T22:06:37Z
- **Tasks:** 1 (TDD: RED + GREEN + fix)
- **Files modified:** 2

## Accomplishments
- Implemented generate_code() that produces a complete Rust source file from MasterConfig + theme mappings
- Generated enum has all required derives, #[non_exhaustive], and const ALL array
- IconProvider impl with icon_name() arms for all themes (bundled + system) and icon_svg() arms only for bundled themes
- include_bytes! paths correctly use CARGO_MANIFEST_DIR prefix for OUT_DIR resolution
- DE-aware mapping values correctly use default value (Phase 24 adds full DE dispatch)
- 22 new tests covering all codegen requirements

## Task Commits

Each task was committed atomically:

1. **Task 1 (RED): Failing tests for codegen** - `067fafe` (test)
2. **Task 1 (GREEN): Implement code generation** - `82dd268` (feat)
3. **Task 1 (fix): Correct module declaration** - `fe4b2b5` (fix)

## Files Created/Modified
- `native-theme-build/src/codegen.rs` - Code generation functions: theme_name_to_icon_set, generate_code, generate_icon_name, generate_icon_svg, plus 22 tests
- `native-theme-build/src/lib.rs` - Added mod codegen declaration

## Decisions Made
- generate_code returns a String rather than writing directly to a file, making it easily testable via string assertions; Plan 04 will call it and write the result to OUT_DIR
- For DE-aware mapping values, Phase 23 generates code using only the default value; Phase 24 will add desktop-environment-specific dispatch arms
- Split codegen into private helper functions (generate_icon_name, generate_icon_svg) for readability while keeping generate_code as the single entry point

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed linter interference with mod declarations**
- **Found during:** Task 1 (GREEN commit)
- **Issue:** An external tool kept replacing `mod codegen;` with `mod validate;` in lib.rs between file writes and git staging
- **Fix:** Created additional fix commit to restore correct module declaration
- **Files modified:** native-theme-build/src/lib.rs
- **Committed in:** fe4b2b5

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Minor tooling interference, no scope creep.

## Issues Encountered
- External linter/tool repeatedly modified lib.rs to add `mod validate;` (from Plan 02 which hasn't been executed yet) and remove `mod codegen;`. Resolved with an explicit fix commit.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- codegen module ready for integration into public API in Plan 04
- generate_code() accepts MasterConfig + mappings + base_dir, returns complete source String
- Plan 04 will wire generate_code into generate_icons() and IconGenerator::generate()

## Self-Check: PASSED

All artifacts verified:
- native-theme-build/src/codegen.rs exists on disk
- native-theme-build/src/lib.rs has mod codegen declaration
- All 3 commit hashes (067fafe, 82dd268, fe4b2b5) found in git log
- 38 tests pass via `cargo test -p native-theme-build`
- Workspace compiles via `cargo check --workspace`

---
*Phase: 23-build-crate-and-code-generation*
*Completed: 2026-03-15*
