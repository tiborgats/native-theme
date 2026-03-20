---
phase: 36-screenshot-and-gif-generation
plan: 01
subsystem: ui
tags: [cli, showcase, gpui, iced, screenshots]

# Dependency graph
requires: []
provides:
  - "CLI argument parsing for gpui showcase (--theme, --variant, --tab, --icon-set)"
  - "CLI argument parsing for iced showcase (--theme, --variant, --tab, --icon-set)"
affects: [36-02, 36-03]

# Tech tracking
tech-stack:
  added: []
  patterns: ["OnceLock for passing CLI args to iced State::default()"]

key-files:
  modified:
    - connectors/native-theme-gpui/examples/showcase.rs
    - connectors/native-theme-iced/examples/showcase.rs

key-decisions:
  - "Used std::env::args() only, no external dependencies (clap/structopt)"
  - "Used OnceLock<CliArgs> for iced showcase since State::default() has no parameter slot"
  - "Applied CLI overrides after default construction to preserve all dependent state initialization"

patterns-established:
  - "CliArgs struct with parse() for showcase CLI argument parsing"
  - "OnceLock pattern for passing init-time config into iced State::default()"

requirements-completed: []

# Metrics
duration: 3min
completed: 2026-03-20
---

# Phase 36 Plan 01: CLI Argument Parsing Summary

**Both showcases accept --theme, --variant, --tab, --icon-set CLI args using std::env::args() with no external dependencies**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-20T08:08:09Z
- **Completed:** 2026-03-20T08:11:19Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Added CLI argument parsing to gpui showcase with CliArgs struct and direct application after Showcase::new()
- Added CLI argument parsing to iced showcase using OnceLock to bridge CLI args into State::default()
- Both showcases can be launched with specific theme/variant/tab/icon-set for screenshot automation
- No behavior change when run without arguments

## Task Commits

Each task was committed atomically:

1. **Task 1: Add CLI argument parsing to gpui showcase** - `57059a0` (feat)
2. **Task 2: Add CLI argument parsing to iced showcase** - `8e7ae32` (feat)

## Files Created/Modified
- `connectors/native-theme-gpui/examples/showcase.rs` - Added CliArgs struct, parse(), tab_index(); modified main() to parse and apply CLI overrides
- `connectors/native-theme-iced/examples/showcase.rs` - Added CliArgs struct with OnceLock; modified State::default() to apply CLI overrides after construction

## Decisions Made
- Used `std::env::args()` only -- no external dependency added, keeping the showcase lightweight
- For gpui: Applied overrides inside `cx.new()` closure after `Showcase::new()` since window/cx are available
- For iced: Used `OnceLock<CliArgs>` since `iced::application()` takes `State::default` as a function pointer and there is no parameter slot to pass CLI args

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Both showcases ready for screenshot automation script (Plan 03)
- Can be launched with: `cargo run -p native-theme-gpui --example showcase -- --theme catppuccin-mocha --variant dark --tab icons`

---
*Phase: 36-screenshot-and-gif-generation*
*Completed: 2026-03-20*
