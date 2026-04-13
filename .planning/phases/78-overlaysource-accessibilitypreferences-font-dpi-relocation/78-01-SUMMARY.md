---
phase: 78-overlaysource-accessibilitypreferences-font-dpi-relocation
plan: 01
subsystem: model, resolve, pipeline, connectors
tags: [accessibility, font-dpi, api-restructure, breaking-change]
dependency_graph:
  requires: [77-02]
  provides: [AccessibilityPreferences, font_dpi-parameter, clean-ThemeDefaults]
  affects: [pipeline, kde-reader, gnome-reader, macos-reader, windows-reader, gpui-connector, iced-connector, presets]
tech_stack:
  added: []
  patterns: [tuple-return-for-reader-output, explicit-dpi-parameter]
key_files:
  created: []
  modified:
    - native-theme/src/lib.rs
    - native-theme/src/prelude.rs
    - native-theme/src/model/defaults.rs
    - native-theme/src/model/mod.rs
    - native-theme/src/model/resolved.rs
    - native-theme/src/resolve/mod.rs
    - native-theme/src/resolve/validate.rs
    - native-theme/src/resolve/validate_helpers.rs
    - native-theme/src/resolve/tests.rs
    - native-theme/src/pipeline.rs
    - native-theme/src/kde/mod.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/src/macos.rs
    - native-theme/src/windows.rs
    - native-theme/src/presets.rs
    - connectors/native-theme-gpui/src/lib.rs
    - connectors/native-theme-gpui/src/colors.rs
    - connectors/native-theme-gpui/src/config.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs
    - connectors/native-theme-iced/src/lib.rs
    - connectors/native-theme-iced/src/extended.rs
    - connectors/native-theme-iced/src/palette.rs
    - connectors/native-theme-iced/tests/integration.rs
    - connectors/native-theme-iced/examples/showcase-iced.rs
    - native-theme/tests/prelude_smoke.rs
    - native-theme/tests/reader_kde.rs
    - native-theme/tests/proptest_roundtrip.rs
    - native-theme/src/presets/*.toml (all 17)
decisions:
  - "validate() convenience method retained (delegates to validate_with_dpi(DEFAULT_FONT_DPI=96.0)) to avoid updating 40+ internal call sites"
  - "from_kde_content_pure returns (Theme, Option<f32>, AccessibilityPreferences) tuple; from_kde_content discards dpi/accessibility (pipeline wires them in Plan 02)"
  - "GPUI to_theme() and to_theme_color() accept reduce_transparency: bool parameter; from_preset passes false, from_system passes real value"
  - "GPUI accessibility helpers (is_reduced_motion, is_high_contrast, is_reduced_transparency, text_scaling_factor) changed from &ResolvedTheme to &SystemTheme"
  - "Pipeline run_pipeline uses AccessibilityPreferences::default() temporarily; Plan 02 will wire real OS values through"
metrics:
  duration_seconds: 1158
  completed: "2026-04-13"
---

# Phase 78 Plan 01: AccessibilityPreferences and font_dpi Relocation Summary

AccessibilityPreferences struct on SystemTheme, font_dpi as explicit into_resolved() parameter, all 17 presets cleaned, 4 OS readers updated, both connector crates migrated.

## What Was Done

### Task 1: Define AccessibilityPreferences, restructure ThemeDefaults/ResolvedDefaults, thread font_dpi
- Created `AccessibilityPreferences` struct with 4 fields (text_scaling_factor, reduce_motion, high_contrast, reduce_transparency) and Default impl
- Added `pub accessibility: AccessibilityPreferences` field to `SystemTheme`
- Re-exported `AccessibilityPreferences` from prelude (7 items now)
- Removed 5 fields from `ThemeDefaults`: text_scaling_factor, reduce_motion, high_contrast, reduce_transparency, font_dpi
- Removed 5 fields from `ResolvedDefaults`: font_dpi, text_scaling_factor, reduce_motion, high_contrast, reduce_transparency
- Changed `into_resolved()` to `into_resolved(font_dpi: Option<f32>)` with auto-detect on None
- Added `validate_with_dpi(f32)` alongside `validate()` convenience method
- Removed accessibility fields from FIELD_NAMES const and impl_merge! macro call
- Updated all resolve/tests.rs font_dpi tests to use validate_with_dpi()

### Task 2: Update pipeline, OS readers, presets, connectors, all call sites
- Removed accessibility fields from all 17 TOML presets (34 sections total: light + dark)
- KDE reader: `from_kde_content_pure` returns `(Theme, Option<f32>, AccessibilityPreferences)` tuple
- GNOME reader: removed 8 lines setting accessibility on ThemeDefaults
- macOS reader: removed test assertions on removed ThemeDefaults fields
- Windows reader: removed accessibility field assignments and test assertions
- Pipeline: removed font_dpi propagation block, passes `into_resolved(None)`, constructs default accessibility
- GPUI connector: `to_theme()` and `to_theme_color()` accept `reduce_transparency: bool` parameter
- GPUI accessibility helpers changed signatures from `&ResolvedTheme` to `&SystemTheme`
- Updated all 13 `into_resolved()` call sites across connectors and tests to `into_resolved(None)`
- Updated reader_kde.rs integration test assertions to check tuple return values
- Updated proptest_roundtrip.rs to remove accessibility fields from strategy
- Fixed clippy redundant_closure warnings

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added validate() convenience wrapper**
- **Found during:** Task 1
- **Issue:** 40+ internal call sites use `.validate()` directly; changing all to `.validate_with_dpi(dpi)` would bloat the diff
- **Fix:** Added `validate()` that delegates to `validate_with_dpi(DEFAULT_FONT_DPI)` alongside the new method
- **Files modified:** native-theme/src/resolve/validate.rs

**2. [Rule 3 - Blocking] Removed pipeline font_dpi propagation test**
- **Found during:** Task 1
- **Issue:** `test_run_pipeline_propagates_font_dpi_to_inactive_variant` set font_dpi on ThemeDefaults which no longer exists
- **Fix:** Removed the test; font_dpi propagation will be re-tested after Plan 02 wires it through run_pipeline parameters
- **Files modified:** native-theme/src/pipeline.rs

**3. [Rule 1 - Bug] Fixed clippy redundant_closure warnings**
- **Found during:** Task 2 verification
- **Issue:** `unwrap_or_else(|| fn())` flagged as redundant closure by clippy -D warnings
- **Fix:** Changed to `unwrap_or_else(fn)` in resolve/mod.rs and pipeline.rs
- **Files modified:** native-theme/src/resolve/mod.rs, native-theme/src/pipeline.rs

**4. [Rule 3 - Blocking] Updated proptest_roundtrip.rs**
- **Found during:** Task 2 verification
- **Issue:** proptest strategy constructed ThemeDefaults with removed fields
- **Fix:** Removed font_dpi, text_scaling_factor, reduce_motion, high_contrast, reduce_transparency from the strategy
- **Files modified:** native-theme/tests/proptest_roundtrip.rs

## Verification

- SC#1: ThemeDefaults has zero accessibility/font_dpi fields (grep confirms)
- SC#2: ResolvedDefaults has zero accessibility/font_dpi fields (only doc comments mention font_dpi)
- SC#3: AccessibilityPreferences field exists on SystemTheme
- SC#3b: into_resolved takes font_dpi: Option<f32> parameter
- SC#5: No stale .into_resolved() calls in Rust source (only in README.md documentation)
- SC#6: pre-release-check.sh passes clean (fmt, clippy, all tests across workspace)

## Self-Check: PASSED

All files exist, both commits verified:
- Task 1: 77d47e1
- Task 2: c1ab251
