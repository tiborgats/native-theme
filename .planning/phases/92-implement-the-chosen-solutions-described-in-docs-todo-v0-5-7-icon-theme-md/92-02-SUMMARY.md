---
phase: 92-icon-theme-dropdown
plan: 02
subsystem: icons
tags: [icon-theme, freedesktop, iced, showcase, dropdown]

requires:
  - phase: 92-01
    provides: "IconSetChoice enum, default_icon_choice(), list_freedesktop_themes()"
provides:
  - "Iced showcase using library IconSetChoice with follows_preset() guard"
  - "Installed freedesktop themes in iced dropdown"
  - "build_icon_choices() helper for dropdown construction"
affects: [92-04, showcase-iced]

tech-stack:
  added: []
  patterns:
    - "follows_preset() guard pattern in rebuild_theme() to preserve user's explicit icon set choice"
    - "build_icon_choices() constructs dropdown from Default + System + installed themes + bundled sets"
    - "effective_icon_set()/freedesktop_theme() methods replace manual match on IconSetChoice variants"

key-files:
  created: []
  modified:
    - connectors/native-theme-iced/examples/showcase-iced.rs

key-decisions:
  - "icon_theme_opt tracked as Option<String> (not bool) to align with library default_icon_choice(Option<&str>) signature"
  - "CLI --icon-set override: unknown names treated as Freedesktop(name) instead of falling back to System"

patterns-established:
  - "Library IconSetChoice replaces local enum in connector showcases"
  - "Conditional icon choice re-derivation via follows_preset() is the canonical guard pattern"

requirements-completed: []

duration: 3min
completed: 2026-04-15
---

# Phase 92 Plan 02: Iced Showcase Icon Theme Selector Fix Summary

**Fixed iced showcase icon theme selector: library IconSetChoice with follows_preset() guard, installed freedesktop themes in dropdown**

## Performance

- **Duration:** 3 min
- **Started:** 2026-04-15T23:30:56Z
- **Completed:** 2026-04-15T23:34:39Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments

- Replaced local `IconSetChoice` enum (4 variants) with library import (5 variants including `Freedesktop(String)`)
- Deleted local `resolve_icon_choice()` function, replaced with `default_icon_choice()` + `build_icon_choices()` helper
- Guarded `rebuild_theme()` icon choice re-derivation with `follows_preset()` -- only `Default` variant is overwritten on theme watcher tick; System/Freedesktop/Material/Lucide choices persist
- Added `installed_themes` field populated from `list_freedesktop_themes()` at init
- Dropdown now includes all installed freedesktop icon themes (e.g. breeze, char-white, Papirus)
- Updated `load_all_icons()` to use `effective_icon_set()` / `freedesktop_theme()` methods
- Updated CLI `--icon-set` override to handle arbitrary freedesktop theme names
- Net -65 lines (68 insertions, 133 deletions) -- local duplication removed

## Task Commits

Each task was committed atomically:

1. **Task 1: Replace local IconSetChoice with library import and add installed themes** - `c1a30f6` (feat)

## Files Created/Modified

- `connectors/native-theme-iced/examples/showcase-iced.rs` - Imports library IconSetChoice, deletes local enum + resolve_icon_choice, adds build_icon_choices helper, installed_themes field, follows_preset guard in rebuild_theme, effective_icon_set/freedesktop_theme usage in load_all_icons and handlers

## Decisions Made

- Tracked `icon_theme_opt` as `Option<String>` instead of the old `has_toml_icon_theme: bool` -- aligns with `default_icon_choice(IconSet, Option<&str>)` signature and eliminates the boolean parameter
- CLI `--icon-set` override maps unknown names to `Freedesktop(name)` instead of falling back to `System` -- allows testing specific installed themes via command line

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed Cow<str> to String conversion for icon_theme_opt**
- **Found during:** Task 1 (cargo check verification)
- **Issue:** `variant.defaults.icon_theme` is `Option<Cow<'_, str>>`, not `Option<String>` -- direct `.clone()` produced type mismatch
- **Fix:** Used `.as_deref().map(|s| s.to_string())` to convert to `Option<String>`
- **Files modified:** connectors/native-theme-iced/examples/showcase-iced.rs
- **Verification:** `cargo check -p native-theme-iced --examples` compiles clean
- **Committed in:** c1a30f6 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 bug fix)
**Impact on plan:** Minor type conversion fix. No scope creep.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Iced showcase fully migrated to library IconSetChoice
- Plan 03 (GPUI showcase) can follow the same pattern: import library type, replace local booleans, add follows_preset() guard
- Plan 04 (verification) can confirm both showcases compile and pass clippy

## Self-Check: PASSED

- All modified files exist on disk
- Commit c1a30f6 found in git log
- No stubs or placeholders in modified code
- No local IconSetChoice enum remains in showcase-iced.rs
- No resolve_icon_choice function remains in showcase-iced.rs

---
*Phase: 92-icon-theme-dropdown*
*Completed: 2026-04-15*
