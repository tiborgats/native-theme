---
phase: 55-correctness-safety-ci
plan: 01
subsystem: platform-detection
tags: [gsettings, dark-mode, gtk, ios, must_use, timeout]

# Dependency graph
requires:
  - phase: 54-connector-migration
    provides: existing lib.rs platform detection, gnome/mod.rs reader
provides:
  - GTK_THEME env var dark-mode detection for non-GNOME/KDE Linux DEs
  - gtk-3.0/settings.ini fallback for XFCE, MATE, etc.
  - 2-second timeout on all gsettings subprocess calls
  - iOS platform detection in detect_platform()
  - Accurate must_use messages on into_resolved() and validate()
affects: [55-02, 55-03]

# Tech tracking
tech-stack:
  added: []
  patterns: [spawn-try_wait timeout pattern for subprocess calls]

key-files:
  created: []
  modified:
    - native-theme/src/lib.rs
    - native-theme/src/presets.rs
    - native-theme/src/resolve.rs
    - native-theme/src/gnome/mod.rs

key-decisions:
  - "Shared run_gsettings_with_timeout() helper in lib.rs as pub(crate) for reuse by gnome/mod.rs"
  - "detect_reduced_motion_inner() gsettings call also switched to timeout helper for consistency"
  - "validate() must_use says 'handle the Result or propagate with ?' to avoid .unwrap() in message text"

patterns-established:
  - "gsettings timeout: all gsettings subprocess calls use run_gsettings_with_timeout() with 2-second deadline"
  - "dark-mode detection order: GTK_THEME env > gsettings color-scheme > KDE kdeglobals > gtk-3.0/settings.ini > false"

requirements-completed: [CORRECT-01, CORRECT-02, CORRECT-03, CORRECT-05]

# Metrics
duration: 7min
completed: 2026-04-07
---

# Phase 55 Plan 01: Correctness & Reliability Fixes Summary

**GTK_THEME env + settings.ini dark-mode fallbacks, gsettings 2-second timeout, iOS platform detection, accurate must_use docs**

## Performance

- **Duration:** 7 min
- **Started:** 2026-04-07T15:32:00Z
- **Completed:** 2026-04-07T15:39:30Z
- **Tasks:** 1
- **Files modified:** 5

## Accomplishments
- detect_is_dark_inner() now checks GTK_THEME env var first (works on XFCE, MATE, Cinnamon, etc.), then gsettings with timeout, then KDE kdeglobals, then gtk-3.0/settings.ini as final fallback
- All gsettings subprocess calls (dark-mode, reduce-motion, gnome reader) use shared run_gsettings_with_timeout() with 2-second deadline -- prevents indefinite blocking when D-Bus is unresponsive
- detect_platform() returns "ios" on target_os = ios, fixing preset filtering for iOS builds
- into_resolved() #[must_use] now says "consumes self" (accurate for a fn taking self by value)
- validate() #[must_use] now describes the returned Result (was copy-pasted inaccurately)

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix detect_is_dark, detect_platform, #[must_use], and gsettings timeout** - `8247414` (fix)

**Plan metadata:** (pending)

## Files Created/Modified
- `native-theme/src/lib.rs` - Added run_gsettings_with_timeout() helper, GTK_THEME env check, gtk-3.0/settings.ini fallback, switched all gsettings calls to timeout helper
- `native-theme/src/presets.rs` - Added #[cfg(target_os = "ios")] block to detect_platform(), updated catch-all cfg
- `native-theme/src/resolve.rs` - Fixed #[must_use] messages on into_resolved() and validate()
- `native-theme/src/gnome/mod.rs` - read_gsetting() now delegates to crate::run_gsettings_with_timeout()
- `native-theme/src/spinners.rs` - Auto-formatted by cargo fmt (no functional change)

## Decisions Made
- Shared run_gsettings_with_timeout() placed in lib.rs as pub(crate) so gnome/mod.rs can call it via crate:: path
- detect_reduced_motion_inner() gsettings call also switched to timeout helper for consistency (Rule 2 deviation -- same blocking risk)
- validate() must_use message avoids ".unwrap()" text to pass the project's panic-pattern lint

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added timeout to detect_reduced_motion_inner() gsettings call**
- **Found during:** Task 1
- **Issue:** The plan only mentioned timeout for detect_is_dark_inner() and read_gsetting(), but detect_reduced_motion_inner() has the same blocking risk with its gsettings call
- **Fix:** Switched it to use run_gsettings_with_timeout() as well
- **Files modified:** native-theme/src/lib.rs
- **Verification:** cargo test + clippy clean
- **Committed in:** 8247414 (part of task commit)

**2. [Rule 1 - Bug] Collapsed nested if statements for clippy compliance**
- **Found during:** Task 1
- **Issue:** Plan's settings.ini code had nested if/if-let/if that triggered clippy::collapsible_if
- **Fix:** Combined into single if with && let chains
- **Files modified:** native-theme/src/lib.rs
- **Verification:** clippy -Dwarnings clean
- **Committed in:** 8247414 (part of task commit)

---

**Total deviations:** 2 auto-fixed (1 missing critical, 1 bug)
**Impact on plan:** Both fixes necessary for correctness and CI compliance. No scope creep.

## Issues Encountered
- cargo doc has a pre-existing unresolved link error in widgets/mod.rs (ThemeVariant reference in macro-generated doc) -- not introduced by this plan, not fixed (out of scope)

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All correctness fixes landed, ready for Plan 02 (safety lints) and Plan 03 (CI)
- Pre-release check passes

---
*Phase: 55-correctness-safety-ci*
*Completed: 2026-04-07*
