---
phase: 01-data-model-foundation
plan: 01
subsystem: core
tags: [rust, serde, toml, color, error-handling, macro]

# Dependency graph
requires: []
provides:
  - "Rgba color type with hex serde (#RGB, #RGBA, #RRGGBB, #RRGGBBAA)"
  - "Error enum with Unsupported, Unavailable, Format, Platform variants"
  - "impl_merge! declarative macro for theme struct merge/is_empty generation"
  - "Crate scaffold with serde, serde_with, toml dependencies"
  - "Result<T> type alias"
affects: [01-02, 01-03, 02-presets, 03-kde-reader, 04-portal-reader, 05-windows-reader, 06-gnome-reader]

# Tech tracking
tech-stack:
  added: [serde 1, serde_with 3, toml 1, serde_json 1 (dev)]
  patterns: [custom-serde-hex, declarative-merge-macro, non-exhaustive-enums]

key-files:
  created:
    - Cargo.toml
    - src/lib.rs
    - src/color.rs
    - src/error.rs
  modified: []

key-decisions:
  - "u8 internal representation for Rgba (matches platform sources, enables Copy/Eq/Hash)"
  - "Alpha defaults to 255 (opaque) when parsing 6-digit hex, following CSS convention"
  - "impl_merge! macro handles two field categories: option (leaf) and nested (recursive)"
  - "Error uses #[non_exhaustive] for forward-compatible variant addition"
  - "serde_json added as dev-dependency for Rgba serde round-trip tests"

patterns-established:
  - "Custom Serialize/Deserialize for Rgba via Display/FromStr"
  - "impl_merge! macro pattern: option { fields } and nested { fields } categories"
  - "Error From conversions for toml::de::Error, toml::ser::Error, std::io::Error"

# Metrics
duration: 3min
completed: 2026-03-07
---

# Phase 1 Plan 01: Project Scaffold Summary

**Rgba color type with full hex serde, Error enum with 4 variants, and impl_merge! declarative macro for theme struct generation**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-07T15:10:48Z
- **Completed:** 2026-03-07T15:13:59Z
- **Tasks:** 3
- **Files modified:** 4

## Accomplishments

- Rgba type with const constructors, FromStr for 4 hex formats, Display with alpha elision, custom Serialize/Deserialize, f32 interop methods
- Error enum with Unsupported, Unavailable, Format, Platform variants plus Display, source(), and From conversions for toml and io errors
- impl_merge! macro supporting option (leaf) and nested (recursive) field categories, generating both merge() and is_empty() methods
- 32 tests total: 23 for Rgba (parsing, display, serde round-trips, traits), 9 for Error (display, source, From, Send+Sync)

## Task Commits

Each task was committed atomically:

1. **Task 1: Create project scaffold with Cargo.toml, lib.rs, and merge macro** - `4efb382` (feat)
2. **Task 2: Implement Rgba color type (TDD RED)** - `dd59871` (test)
3. **Task 2: Implement Rgba color type (TDD GREEN)** - `0b345d1` (feat)
4. **Task 3: Implement Error enum** - `8eaaa4d` (feat)

## Files Created/Modified

- `Cargo.toml` - Project manifest with serde, serde_with, toml dependencies; edition 2024
- `src/lib.rs` - Crate root with impl_merge! macro, module declarations, re-exports, Result alias
- `src/color.rs` - Rgba struct with rgb/rgba/from_f32 constructors, FromStr, Display, custom Serialize/Deserialize, to_f32_array/tuple, 23 tests
- `src/error.rs` - Error enum with 4 variants, Display, source(), From<toml::de::Error/toml::ser::Error/io::Error>, 9 tests

## Decisions Made

- Used u8 internal representation for Rgba (matches all platform sources, enables Copy/Eq/Hash derive)
- Alpha defaults to 255 when parsing 6-digit hex (#RRGGBB), following CSS convention
- Macro uses two field categories (option/nested) rather than a single field list, enabling deep recursive merge
- Added serde_json as dev-dependency for Rgba serde round-trip testing (not a runtime dep)
- Error uses `#[non_exhaustive]` for forward-compatible variant addition in future phases

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Rgba, Error, and impl_merge! are ready for Plan 02 (model structs)
- Plan 02 will uncomment `pub mod model;` in lib.rs and use impl_merge! on all theme structs
- All 32 tests passing, cargo check clean

## Self-Check: PASSED

- All 4 source files exist (Cargo.toml, src/lib.rs, src/color.rs, src/error.rs)
- All 4 commits verified (4efb382, dd59871, 0b345d1, 8eaaa4d)
- SUMMARY.md created at expected path

---
*Phase: 01-data-model-foundation*
*Completed: 2026-03-07*
