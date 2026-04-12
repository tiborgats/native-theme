---
phase: 77-systemtheme-api-and-icon-set-relocation
plan: 02
subsystem: model
tags: [icon-set, icon-theme, theme-model, systemtheme, toml-presets]

requires:
  - phase: 77-01
    provides: "ColorMode enum, SystemTheme API with pick(ColorMode)"
provides:
  - "icon_set and icon_theme fields on Theme (shared across light/dark)"
  - "icon_set: IconSet and icon_theme: String on SystemTheme (resolved, non-optional)"
  - "ResolvedTheme no longer carries icon_set or icon_theme"
  - "All 16 TOML presets have icon_set/icon_theme at top level"
  - "SC#4 doctest: Theme::preset -> theme.icon_set read"
affects: [phase-78, phase-82, connectors]

tech-stack:
  added: []
  patterns: ["shared-across-variants fields live on Theme, not ThemeMode"]

key-files:
  created: []
  modified:
    - native-theme/src/model/mod.rs
    - native-theme/src/model/resolved.rs
    - native-theme/src/lib.rs
    - native-theme/src/pipeline.rs
    - native-theme/src/resolve/mod.rs
    - native-theme/src/resolve/validate.rs
    - native-theme/src/kde/mod.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/src/macos.rs
    - native-theme/src/windows.rs
    - native-theme/src/error.rs
    - native-theme/src/presets.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs
    - connectors/native-theme-iced/examples/showcase-iced.rs

key-decisions:
  - "icon_theme uses light variant value for kde-breeze preset (KDE reader overrides at runtime)"
  - "Pipeline resolves icon_set/icon_theme from merged Theme with system_icon_set/system_icon_theme fallback"
  - "Connector examples store current_icon_set/current_icon_theme alongside current_resolved"
  - "resolve_icon_choice and load_all_icons take explicit icon_set/icon_theme params instead of reading ResolvedTheme"

patterns-established:
  - "Shared-across-variants fields on Theme: icon_set, icon_theme, layout (not per-variant)"
  - "Pipeline-level resolution: fields shared across variants resolved in run_pipeline, not ThemeMode::resolve"

requirements-completed: [MODEL-06]

duration: 24min
completed: 2026-04-13
---

# Phase 77 Plan 02: Icon Set Relocation Summary

**Relocated icon_set/icon_theme from per-variant ThemeMode to shared Theme, with resolved values on SystemTheme and pipeline-level fallback**

## Performance

- **Duration:** 24 min
- **Started:** 2026-04-12T23:23:12Z
- **Completed:** 2026-04-12T23:47:18Z
- **Tasks:** 2
- **Files modified:** 36

## Accomplishments
- Moved icon_set and icon_theme from ThemeMode (per-variant) to Theme (shared across light/dark)
- Removed icon_set and icon_theme from ResolvedTheme -- now on SystemTheme only
- Added icon_set: IconSet and icon_theme: String to SystemTheme with pipeline-level resolution
- Updated all 16 TOML presets to have icon_set/icon_theme at top level
- Updated KDE, GNOME, macOS, and Windows platform readers to set icon fields on Theme
- Migrated both connector showcases (gpui, iced) to read icon fields from SystemTheme/Theme
- Added SC#4 doctest demonstrating preset load -> theme.icon_set read

## Task Commits

Each task was committed atomically:

1. **Task 1: Relocate icon fields in model, resolve, presets, pipeline, and readers** - `caa178f` (refactor)
2. **Task 2: Migrate connector call sites and add SC#4 doctest** - `9018862` (feat)

## Files Created/Modified
- `native-theme/src/model/mod.rs` - Theme gains icon_set/icon_theme; ThemeMode loses them; lint_toml updated
- `native-theme/src/model/resolved.rs` - ResolvedTheme no longer has icon_set/icon_theme
- `native-theme/src/lib.rs` - SystemTheme gains icon_set/icon_theme; with_overlay propagates them
- `native-theme/src/pipeline.rs` - run_pipeline resolves icon fields from merged Theme with fallback
- `native-theme/src/resolve/mod.rs` - Removed icon_set/icon_theme from resolve/resolve_platform_defaults
- `native-theme/src/resolve/validate.rs` - Removed icon_set/icon_theme extraction and ResolvedTheme construction
- `native-theme/src/error.rs` - Removed icon_set special case in field_category
- `native-theme/src/kde/mod.rs` - Icon theme set on Theme, not variant
- `native-theme/src/gnome/mod.rs` - Icon theme set on Theme in both build_gnome_spec_pure and build_theme
- `native-theme/src/macos.rs` - icon_set set on Theme, removed from variants
- `native-theme/src/windows.rs` - icon_set set on Theme, removed from variants
- `native-theme/src/presets.rs` - Tests read icon_set from Theme, not variant
- `native-theme/src/presets/*.toml` - All 16 presets: icon_set/icon_theme at top level
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - Read icon fields from SystemTheme/Theme
- `connectors/native-theme-iced/examples/showcase-iced.rs` - Added current_icon_set/current_icon_theme state

## Decisions Made
- KDE Breeze preset uses "breeze" (light value) as shared icon_theme; KDE reader overrides at runtime
- Pipeline resolves icon_set/icon_theme after merge chain: merged.icon_set.unwrap_or(system_icon_set())
- Connector examples maintain icon_set/icon_theme state parallel to current_resolved
- resolve_icon_choice/load_all_icons take explicit icon_set/icon_theme params (no longer on ResolvedTheme)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated windows.rs Theme construction**
- **Found during:** Task 1
- **Issue:** Plan did not mention windows.rs reader, but it constructs Theme without icon_set/icon_theme
- **Fix:** Added icon_set: Some(IconSet::SegoeIcons) and icon_theme: None to Windows Theme construction
- **Files modified:** native-theme/src/windows.rs
- **Committed in:** caa178f

**2. [Rule 3 - Blocking] Updated integration test files**
- **Found during:** Task 1
- **Issue:** Integration tests in tests/ directory referenced ThemeMode.icon_set/icon_theme
- **Fix:** Updated proptest_roundtrip.rs, platform_facts_xref.rs, reader_kde.rs to use Theme-level fields
- **Files modified:** native-theme/tests/proptest_roundtrip.rs, platform_facts_xref.rs, reader_kde.rs
- **Committed in:** caa178f

**3. [Rule 1 - Bug] Fixed missing variable in resolve/tests.rs**
- **Found during:** Task 1
- **Issue:** Removing validate_checks_icon_set test accidentally removed let mut v = variant_with_defaults() from the next test
- **Fix:** Restored the variable initialization line
- **Files modified:** native-theme/src/resolve/tests.rs
- **Committed in:** caa178f

---

**Total deviations:** 3 auto-fixed (1 bug, 2 blocking)
**Impact on plan:** All fixes necessary for compilation. No scope creep.

## Issues Encountered
- 3 pre-existing test failures unrelated to this plan (gnome contrast test, kde reader fixture dialog order, rasterize doctest) -- not caused by icon relocation changes

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 77 complete: both MODEL-03 (pick(ColorMode)) and MODEL-06 (icon_set relocation) delivered
- Ready for Phase 78 (OverlaySource + AccessibilityPreferences + font_dpi)

---
*Phase: 77-systemtheme-api-and-icon-set-relocation*
*Completed: 2026-04-13*
