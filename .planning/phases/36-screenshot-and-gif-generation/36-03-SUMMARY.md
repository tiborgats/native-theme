---
phase: 36-screenshot-and-gif-generation
plan: 03
subsystem: ui
tags: [bash, spectacle, screenshots, kde, automation, gpui, iced]

# Dependency graph
requires:
  - phase: 36-01
    provides: "CLI argument parsing for both showcases (--theme, --variant, --icon-set)"
  - phase: 36-02
    provides: "generate_gifs.py script for spinner GIF generation"
provides:
  - "scripts/generate_screenshots.sh — captures all 68 screenshots (17 themes x 2 variants x 2 toolkits)"
  - "scripts/generate_assets.sh — master orchestration script running GIFs then screenshots"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Pre-build release binaries before the capture loop to avoid compile-time interference"
    - "pkill spectacle before loop to avoid D-Bus singleton issues on KDE Wayland"
    - "spectacle -i -a -b -n -e per-capture to avoid singleton and ensure clean window capture"

key-files:
  created:
    - scripts/generate_screenshots.sh
    - scripts/generate_assets.sh
  modified: []

key-decisions:
  - "Use cargo run --release rather than direct binary invocation so Cargo resolves working directory"
  - "DELAY=3 variable controls window render wait time, adjustable per-machine"
  - "Consistent ICON_SET=material across all captures for visual uniformity"
  - "spectacle -a requires showcase window to have focus; user verified on KDE Wayland"

patterns-established:
  - "Pre-build pattern: compile once before loop, then cargo run reuses artifact without recompile"

requirements-completed: []

# Metrics
duration: ~10min (including human verification checkpoint)
completed: 2026-03-20
---

# Phase 36 Plan 03: Screenshot Automation and Master Orchestration Summary

**Bash automation scripts capturing 68 screenshots (17 themes x 2 variants x 2 toolkits) via spectacle -a on KDE Wayland, plus a single-command master orchestration script**

## Performance

- **Duration:** ~10 min (including human verification checkpoint)
- **Completed:** 2026-03-20
- **Tasks:** 2 (1 auto + 1 human-verify checkpoint)
- **Files created:** 2

## Accomplishments
- Created generate_screenshots.sh iterating all 17 themes x 2 variants x 2 toolkits = 68 captures using spectacle active-window capture
- Created generate_assets.sh as a single-command entry point that runs GIF generation then screenshot capture
- Verified spectacle -a successfully captures showcase window on KDE Wayland (user confirmed at checkpoint)
- Established that showcase window must have focus for spectacle -a to capture it

## Task Commits

Each task was committed atomically:

1. **Task 1: Create screenshot automation and master orchestration scripts** - `aff8d34` (feat)
2. **Task 2: Human verification checkpoint** - Approved by user

## Files Created/Modified
- `scripts/generate_screenshots.sh` - Iterates all 68 theme+variant+toolkit combos, pre-builds release binaries, captures via spectacle -a, outputs to docs/assets/
- `scripts/generate_assets.sh` - Master orchestration: calls generate_gifs.py then generate_screenshots.sh

## Decisions Made
- Used `cargo run --release` in the capture loop; pre-build step ensures no recompilation during captures
- `DELAY=3` seconds between window launch and spectacle capture; exposed as variable for easy tuning
- `ICON_SET="material"` hardcoded for visual consistency across all 68 screenshots
- `pkill spectacle || true` before loop to clear any stale D-Bus singleton from previous runs

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

During Task 2 verification: spectacle -a captures the active window, so the showcase window must have focus at capture time. This is expected behavior on KDE Wayland and the script handles it correctly since the showcase launches and immediately becomes the active/focused window.

Note on fabricated assets: During earlier phase 36 execution, all fabricated spinner animation frames (macOS, Windows, Material arc, Adwaita arc) were removed from the GIF script. Only genuine assets remain: Lucide loader.svg with spin transform, Material progress_activity.svg with spin transform, and freedesktop runtime sprite sheet loading.

## User Setup Required

None - no external service configuration required. spectacle is a standard KDE tool already present on the user's system.

## Next Phase Readiness
- All visual asset generation scripts are in place
- Running `bash scripts/generate_assets.sh` regenerates all GIFs and screenshots in one command
- Phase 36 (screenshot and GIF generation) is complete

---
*Phase: 36-screenshot-and-gif-generation*
*Completed: 2026-03-20*
