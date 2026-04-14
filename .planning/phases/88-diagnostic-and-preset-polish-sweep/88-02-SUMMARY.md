---
phase: 88-diagnostic-and-preset-polish-sweep
plan: 02
subsystem: api
tags: [cow, zero-alloc, preset-optimization, rustdoc, border-spec, font-spec]

# Dependency graph
requires:
  - phase: 88-diagnostic-and-preset-polish-sweep
    provides: DiagnosticEntry and PlatformPreset types (plan 01)
  - phase: 79-border-split-and-reader-audit
    provides: clean border split (DefaultsBorderSpec vs WidgetBorderSpec)
provides:
  - Theme.name, ThemeDefaults.icon_theme, SystemTheme.name/icon_theme as Cow<'static, str>
  - Bundled preset names stored as Cow::Borrowed (zero-allocation on preset load)
  - FontSpec::style default asymmetry documented (POLISH-04)
  - DefaultsBorderSpec no-padding design documented (POLISH-05)
affects: [connectors, showcase-gpui, showcase-iced]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Cow<'static, str> for compile-time constant strings in bundled presets", "PRESET_DISPLAY_NAMES lookup table for post-parse Cow::Borrowed replacement"]

key-files:
  created: []
  modified:
    - native-theme/src/model/mod.rs
    - native-theme/src/model/defaults.rs
    - native-theme/src/model/font.rs
    - native-theme/src/model/border.rs
    - native-theme/src/lib.rs
    - native-theme/src/presets.rs
    - native-theme/src/pipeline.rs
    - native-theme/src/kde/mod.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/src/macos.rs
    - native-theme/src/windows.rs
    - native-theme/src/resolve/validate_helpers.rs
    - connectors/native-theme-iced/examples/showcase-iced.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs
    - native-theme/tests/proptest_roundtrip.rs
    - .planning/ROADMAP.md

key-decisions:
  - "Theme.name as Cow<'static, str> with manual Default impl using Cow::Borrowed empty string"
  - "PRESET_DISPLAY_NAMES const table maps preset keys to display names for post-parse Cow::Borrowed replacement"
  - "Connector showcases convert Cow to owned String via .into_owned() for local String fields"
  - "ROADMAP SC-3 updated from preset('default') to preset('dracula') since no 'default' preset exists"

patterns-established:
  - "Cow<'static, str> for bundled compile-time constants: post-parse replacement from owned TOML deserialized values to Cow::Borrowed static strings"
  - "Runtime-detected values (from OS readers) wrapped in Cow::Owned at the point they enter the type system"

requirements-completed: [POLISH-04, POLISH-05, POLISH-06]

# Metrics
duration: 16min
completed: 2026-04-14
---

# Phase 88 Plan 02: Cow Migration and Documentation Polish Summary

**Theme.name and icon_theme migrated to Cow<'static, str> eliminating per-load String allocations for bundled presets, with FontSpec::style asymmetry and border padding rule removal documented**

## Performance

- **Duration:** 16 min
- **Started:** 2026-04-14T00:16:21Z
- **Completed:** 2026-04-14T00:32:21Z
- **Tasks:** 2
- **Files modified:** 16

## Accomplishments
- Theme.name, ThemeDefaults.icon_theme, SystemTheme.name, and SystemTheme.icon_theme are all Cow<'static, str>
- Bundled preset names stored as Cow::Borrowed via PRESET_DISPLAY_NAMES lookup table -- doctest on Theme::preset("dracula") verifies is_borrowed()
- FontSpec::style unwrap_or_default documented in both validate_helpers.rs call sites and FontSpec struct doc
- DefaultsBorderSpec and WidgetBorderSpec doc comments explain the padding split design (Phase 79, BORDER-01)
- ROADMAP SC-3 corrected from preset("default") to preset("dracula")
- All 618 native-theme tests pass, workspace compiles cleanly

## Task Commits

Each task was committed atomically:

1. **Task 1: Migrate Theme.name, ThemeDefaults.icon_theme, and SystemTheme fields to Cow** - `9ef1df6` (feat)
2. **Task 2: Document FontSpec::style default asymmetry and border padding rule removal** - `bf6db90` (feat)

## Files Created/Modified
- `native-theme/src/model/mod.rs` - Theme.name -> Cow<'static, str>, manual Default impl, is_borrowed doctest
- `native-theme/src/model/defaults.rs` - ThemeDefaults.icon_theme -> Option<Cow<'static, str>>
- `native-theme/src/model/font.rs` - FontSpec struct doc documents style default asymmetry
- `native-theme/src/model/border.rs` - DefaultsBorderSpec and WidgetBorderSpec docs updated with padding rationale
- `native-theme/src/lib.rs` - SystemTheme, ReaderResult, OverlaySource fields -> Cow<'static, str>
- `native-theme/src/presets.rs` - PRESET_DISPLAY_NAMES map, cache replaces owned names with Cow::Borrowed
- `native-theme/src/pipeline.rs` - icon_theme resolution wraps runtime values in Cow::Owned, test constructors updated
- `native-theme/src/kde/mod.rs` - Theme name and icon_theme wrapped in Cow::Owned
- `native-theme/src/gnome/mod.rs` - ReaderResult name uses .into(), icon_theme wrapped in Cow::Owned
- `native-theme/src/macos.rs` - Theme name uses .into()
- `native-theme/src/windows.rs` - ReaderResult name uses .into()
- `native-theme/src/resolve/validate_helpers.rs` - style unwrap_or_default documented at both sites
- `connectors/native-theme-iced/examples/showcase-iced.rs` - icon_theme converted via .into_owned()
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - icon_theme converted via .into_owned()
- `native-theme/tests/proptest_roundtrip.rs` - Theme name wrapped in Cow::Owned
- `.planning/ROADMAP.md` - SC-3 updated from preset("default") to preset("dracula")

## Decisions Made
- Theme.name uses manual Default impl with Cow::Borrowed("") instead of derived Default (which would produce Cow::Owned(String::new()))
- PRESET_DISPLAY_NAMES const table maps all 20 preset keys (including live variants) to their display names
- Connector showcase code converts Cow to String via .into_owned() since local state fields remain String
- ROADMAP SC-3 uses "dracula" as test preset since no "default" preset exists in the registry

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed clippy collapsible_if in presets.rs cache initialization**
- **Found during:** Task 1 (pre-release check)
- **Issue:** Nested `if let` in cache initialization triggered `clippy::collapsible_if` lint
- **Fix:** Collapsed into single `if let ... && let ...` expression
- **Files modified:** native-theme/src/presets.rs
- **Verification:** `cargo clippy -p native-theme --all-targets -- -D warnings` passes clean
- **Committed in:** 9ef1df6 (Task 1 commit)

**2. [Rule 3 - Blocking] Fixed proptest_roundtrip.rs Theme construction**
- **Found during:** Task 1 (test compilation)
- **Issue:** proptest strategy produced String for Theme.name but the field is now Cow<'static, str>
- **Fix:** Wrapped proptest-generated name in Cow::Owned()
- **Files modified:** native-theme/tests/proptest_roundtrip.rs
- **Verification:** `cargo test -p native-theme --test proptest_roundtrip` passes
- **Committed in:** 9ef1df6 (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (1 bug fix, 1 blocking)
**Impact on plan:** Both auto-fixes necessary for lint compliance and compilation. No scope creep.

## Issues Encountered
- Pre-existing `build_gnome_spec_pure` dead_code warning in gnome/mod.rs -- out of scope, not caused by this plan's changes (same as 88-01)

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 88 is now complete (both plans 01 and 02 executed)
- All POLISH requirements (POLISH-01 through POLISH-06) are fulfilled
- The v0.5.7 milestone roadmap is complete

---
*Phase: 88-diagnostic-and-preset-polish-sweep*
*Completed: 2026-04-14*
