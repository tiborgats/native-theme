---
phase: 04-gnome-portal-reader
plan: 01
subsystem: platform-reader
tags: [gnome, ashpd, portal, xdg, freedesktop, feature-flags, tdd]

# Dependency graph
requires:
  - phase: 02-core-presets
    provides: "Adwaita preset used as fallback base in build_theme"
  - phase: 01-data-model
    provides: "Rgba::from_f32, NativeTheme, ThemeVariant, ThemeColors structs"
provides:
  - "portal/portal-tokio/portal-async-io feature flags in Cargo.toml"
  - "ashpd optional dependency with default-features=false"
  - "build_theme() testable core for portal value -> NativeTheme conversion"
  - "portal_color_to_rgba() helper with XDG spec out-of-range validation"
  - "apply_accent() and apply_high_contrast() helpers"
  - "from_gnome() async stub (Plan 02 wires to D-Bus)"
affects: [04-02-PLAN, gnome-portal-reader]

# Tech tracking
tech-stack:
  added: [ashpd 0.13.4]
  patterns: [feature-gated async module, Adwaita base + portal overlay, TDD red-green]

key-files:
  created: [src/gnome/mod.rs]
  modified: [Cargo.toml, src/lib.rs]

key-decisions:
  - "ashpd default-features=false prevents tokio leakage to sync-only consumers"
  - "portal_color_to_rgba returns None for out-of-range (per XDG spec), not clamped"
  - "apply_high_contrast unconditionally sets border_opacity=1.0 and disabled_opacity=0.7"
  - "NoPreference defaults to light variant (matching Adwaita default)"
  - "Only the active variant (light or dark) is populated, matching KDE reader pattern"

patterns-established:
  - "Adwaita base + portal overlay: build_theme() loads preset then patches with portal values"
  - "Feature flag trio: portal (base gate), portal-tokio (runtime), portal-async-io (alt runtime)"

requirements-completed: [PLAT-02]

# Metrics
duration: 3min
completed: 2026-03-07
---

# Phase 4 Plan 1: GNOME Portal Reader - Feature Flags and Core Summary

**Feature flags (portal/portal-tokio/portal-async-io) with ashpd, plus testable build_theme core mapping color scheme, accent, and contrast onto Adwaita base**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-07T17:22:12Z
- **Completed:** 2026-03-07T17:25:27Z
- **Tasks:** 1 (TDD: RED + GREEN)
- **Files modified:** 3

## Accomplishments
- Feature flag structure that prevents tokio leakage: portal, portal-tokio, portal-async-io
- Testable build_theme core converts portal values (ColorScheme, Color, Contrast) to NativeTheme without D-Bus
- 10 unit tests covering all color scheme variants, accent propagation, high contrast, out-of-range validation, and Adwaita fallback
- Both portal-tokio and portal-async-io compile independently (ashpd runtime isolation verified)

## Task Commits

Each task was committed atomically:

1. **Task 1 RED: Failing tests for GNOME portal reader** - `9b74240` (test)
2. **Task 1 GREEN: Implement build_theme core and helpers** - `d13b23c` (feat)

_TDD task: RED commit with todo!() stubs, GREEN commit with implementations._

## Files Created/Modified
- `Cargo.toml` - Added portal/portal-tokio/portal-async-io features and ashpd dependency
- `src/lib.rs` - Feature-gated gnome module and from_gnome re-export
- `src/gnome/mod.rs` - build_theme, portal_color_to_rgba, apply_accent, apply_high_contrast, from_gnome stub, 10 unit tests

## Decisions Made
- ashpd declared with `default-features = false, features = ["settings"]` to prevent tokio from leaking to consumers who only use sync features (kde)
- Out-of-range portal accent color (outside 0.0..=1.0) returns None per XDG spec, rather than clamping -- this signals "accent not set"
- apply_high_contrast unconditionally sets border_opacity and disabled_opacity (simpler than checking if Adwaita base had values)
- ColorScheme::NoPreference defaults to light variant, matching Adwaita's default appearance
- Only the selected variant (light or dark) is populated in the output NativeTheme, matching the KDE reader pattern

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- `cargo test --all-features` fails because ashpd forbids enabling both tokio and async-io simultaneously via `compile_error!`. This is expected behavior -- tests run with `--features portal-tokio,kde` instead.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- build_theme core is ready for Plan 02 to wire to actual D-Bus portal via ashpd Settings API
- from_gnome() stub in place -- Plan 02 replaces the stub body with Settings::new().await + portal reads
- Feature flag structure is finalized and safe to publish

---
*Phase: 04-gnome-portal-reader*
*Completed: 2026-03-07*
