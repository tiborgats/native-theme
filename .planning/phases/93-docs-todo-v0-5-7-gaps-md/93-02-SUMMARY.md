---
phase: 93-docs-todo-v0-5-7-gaps-md
plan: 02
subsystem: linux-desktop-detection
tags: [linux-desktop, wayfire, wlroots, detect, pipeline, v057-polish, xdg-current-desktop]

# Dependency graph
requires:
  - phase: 75-non-exhaustive-compile-gate-iconset-default
    provides: "`#[non_exhaustive]` on `LinuxDesktop` enables additive variant growth without breaking external callers."
  - phase: 83-detection-cache-layer
    provides: "`parse_linux_desktop` pure function returning `LinuxDesktop` from XDG_CURRENT_DESKTOP component."
provides:
  - "`LinuxDesktop::Wayfire` enum variant."
  - "`parse_linux_desktop(\"Wayfire\")` mapping arm."
  - "`pipeline::from_system_inner` wlroots-compositor fallback arm handles Wayfire via adwaita preset + portal."
  - "`model::icons::detect_linux_icon_theme` wlroots-compositor arm handles Wayfire via org.gnome.desktop.interface gsettings."
  - "Parse tests `parses_xdg_wayfire` (`Wayfire`, `Wayfire:GNOME`) and `parses_xdg_empty_is_unknown` (regression guard)."
affects: [phase-93-verification, any-future-plan-matching-on-LinuxDesktop]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Additive enum variant on `#[non_exhaustive]` `LinuxDesktop` — mirrors Phase 73/75 pattern of same-crate exhaustive match updates."
    - "New Wayland compositor recognition routed through existing shared wlroots fallback arm; no new I/O code paths."

key-files:
  created: []
  modified:
    - "native-theme/src/detect.rs — `LinuxDesktop::Wayfire` variant at line 41 and `\"Wayfire\" => return LinuxDesktop::Wayfire` arm at line 103."
    - "native-theme/src/pipeline.rs — `LinuxDesktop::Wayfire` added to the wlroots fallback arm at line 687; tests `parses_xdg_wayfire` + `parses_xdg_empty_is_unknown` added at lines 923-935."
    - "native-theme/src/model/icons.rs — `LinuxDesktop::Wayfire` added to the gsettings-dispatch arm at line 609 (Rule 3 auto-fix for compile-required exhaustiveness)."

key-decisions:
  - "Wayfire's XDG_CURRENT_DESKTOP value is `\"Wayfire\"` (capitalized, matching upstream Wayfire packaging) — no case-insensitive fallback, staying consistent with other exact-match arms (`Hyprland`, `Budgie`, `COSMIC`, `XFCE`, `LXQt`)."
  - "Wayfire routed through the shared wlroots compositor arm alongside Sway/River/Niri/Hyprland rather than creating a dedicated arm — Wayfire has no native theme system and consumes GTK/portal configuration like the other wlroots compositors."
  - "Added exhaustive arm in `model/icons.rs` (Rule 3 — compile-required) because that match has no wildcard; failing to add an arm would break same-crate compilation after the variant lands."

patterns-established:
  - "When adding a new `LinuxDesktop` variant, update three sites in the crate: (1) `detect.rs::LinuxDesktop` enum, (2) `detect.rs::parse_linux_desktop` match arm, (3) any non-wildcard exhaustive match on `LinuxDesktop` (currently `pipeline::from_system_inner` and `model::icons::detect_linux_icon_theme`)."
  - "Wayland compositors (wlroots family) ordering in the enum: Hyprland, Sway, River, Niri, Wayfire — alphabetical within the wlroots group inserted between Niri and CosmicDe."

requirements-completed: [G2]

# Metrics
duration: 20min
completed: 2026-04-19
---

# Phase 93 Plan 02: LinuxDesktop::Wayfire variant and wlroots fallback route Summary

**Recognise Wayfire in `XDG_CURRENT_DESKTOP` and route it through the adwaita + portal wlroots fallback (same path as Sway/River/Niri/Hyprland) instead of the more expensive portal-backend probe fallback.**

## Performance

- **Duration:** 20 min
- **Started:** 2026-04-19T13:57:26Z
- **Completed:** 2026-04-19T14:17:42Z
- **Tasks:** 1
- **Files modified:** 3

## Accomplishments

- Added `LinuxDesktop::Wayfire` enum variant between `Niri` and `CosmicDe` (preserves "wlroots compositors then other desktops" ordering).
- Added `"Wayfire" => return LinuxDesktop::Wayfire` parse arm in `parse_linux_desktop()` (colon-separated XDG parsing already handles `Wayfire:GNOME` via the existing loop).
- Routed `LinuxDesktop::Wayfire` through the shared wlroots fallback arm in `pipeline::from_system_inner` — same preset (`adwaita`) and same portal-aware path as Sway/River/Niri/Hyprland/CosmicDe.
- Routed `LinuxDesktop::Wayfire` through the GNOME-equivalent gsettings arm in `model::icons::detect_linux_icon_theme` (Rule 3 — non-wildcard exhaustive match required the arm to compile).
- Added two tests: `parses_xdg_wayfire` covers `"Wayfire"` and `"Wayfire:GNOME"` parsing, `parses_xdg_empty_is_unknown` is a regression guard on the pre-existing empty-string behaviour.

## Task Commits

1. **Task 1: Add `LinuxDesktop::Wayfire` variant and parse+route it** — `421c3cc` (feat)

The task was executed as a single atomic commit covering the RED test additions, the GREEN variant + parse arm + pipeline arm + icons arm, plus the regression-guard test. No REFACTOR commit needed (mechanical three-file edit, nothing to clean up).

**Plan metadata:** Pending final docs commit after this SUMMARY is written.

## Files Created/Modified

- `native-theme/src/detect.rs` — Added `Wayfire` variant to `LinuxDesktop` enum and parse arm in `parse_linux_desktop()`.
- `native-theme/src/pipeline.rs` — Added `LinuxDesktop::Wayfire` to the wlroots compositor fallback arm in `from_system_inner`; added `parses_xdg_wayfire` and `parses_xdg_empty_is_unknown` tests in `pipeline::dispatch_tests`.
- `native-theme/src/model/icons.rs` — Added `LinuxDesktop::Wayfire` to the gsettings (`org.gnome.desktop.interface`) arm in `detect_linux_icon_theme` (Rule 3 auto-fix — exhaustive match required an arm to compile).

## Decisions Made

- **Wayfire XDG value is exact-case `"Wayfire"`.** Upstream Wayfire packaging capitalizes the token, matching the existing convention (`Hyprland`, `Budgie`, `COSMIC`). No `Wayfire | wayfire` alternative added — stays consistent with the rest of the parse table.
- **No dedicated pipeline arm.** Wayfire is a wlroots compositor without its own theme engine; GTK/portal config carries the theme. Adding Wayfire to the existing shared arm (Sway/River/Niri/Hyprland/CosmicDe) is the minimal correct change.
- **Rule 3 auto-fix in `model/icons.rs`.** The exhaustive match on `LinuxDesktop` in `detect_linux_icon_theme` has no wildcard. Adding the new variant without a corresponding arm would break compilation. Applied Rule 3 inline (blocking the task) and documented under Deviations.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added `LinuxDesktop::Wayfire` arm in `model/icons.rs::detect_linux_icon_theme`**
- **Found during:** Task 1 (while grepping for same-crate exhaustive matches on `LinuxDesktop` per plan action step 5)
- **Issue:** The plan only called out updates to `detect.rs` and `pipeline.rs`. However, `native-theme/src/model/icons.rs:601-629` contains a second non-wildcard exhaustive match on `LinuxDesktop`. Adding the variant without a corresponding arm would produce a compile-error (E0004 "non-exhaustive patterns") and block the task.
- **Fix:** Added `| crate::detect::LinuxDesktop::Wayfire` to the existing Gnome/Budgie/Hyprland/Sway/River/Niri/CosmicDe group — Wayfire consumes the same `gsettings_icon_theme("org.gnome.desktop.interface")` call as the other wlroots compositors.
- **Files modified:** `native-theme/src/model/icons.rs` (one line added at line 609)
- **Verification:** Clean `cargo build -p native-theme --all-features` green; `cargo test -p native-theme --all-features parses_xdg` shows both new tests pass.
- **Committed in:** `421c3cc` (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking, Rule 3)
**Impact on plan:** The deviation is a mechanical extension of the plan's own action-step-5 ("grep `native-theme/src/` for all `match` expressions exhausting `LinuxDesktop`... adding a new variant forces every such match to handle it"). Plan body only listed `pipeline.rs`, but the `model/icons.rs` hit is the same class of change. No scope creep.

## Issues Encountered

- **Parallel-plan execution noise in the working tree.** Plans 93-01 and 93-03 are wave-1 siblings executing concurrently in the same working tree. Multiple WIP files from those plans (`resolve/validate*.rs`, `model/bundled.rs`, `model/mod.rs`, `freedesktop.rs`, `color.rs`, `border.rs`, and connector files under `connectors/native-theme-gpui/`) appeared and changed during 93-02 execution. To verify my changes in isolation, I temporarily stashed the sibling-plan WIP, confirmed clean compile and passing tests, then restored the WIP so the parallel plans could continue.
- **Inability to run full `cargo test --workspace`.** The GPUI transitive dependency `naga v27.0.3` fails to build on the current toolchain (`WriteColor` trait bound issue). This is pre-existing and documented in `deferred-items.md`. Plan 93-02 verification was scoped to `cargo test -p native-theme` which passes cleanly.
- **Pre-release-check clippy failure unrelated to this plan.** `./pre-release-check.sh` fails on `cargo clippy -p native-theme --all-targets -- -D warnings` because commit `7ba2b4c refactor(93-03): demote icon helper fns to pub(crate)` left `bundled_icon_by_name` as `pub(crate)` with no internal callers. Logged to `deferred-items.md` for Plan 93-03 follow-up or a Plan 93-fix; out of scope per SCOPE BOUNDARY.

## User Setup Required

None — no external service configuration required. The Wayfire variant is an additive local code change.

## Next Phase Readiness

- Plan 93-02 deliverable is complete and committed as `421c3cc`.
- Both new tests (`parses_xdg_wayfire`, `parses_xdg_empty_is_unknown`) pass in isolation (2 passed, 0 failed).
- All 20 existing desktop-detection tests continue to pass — no regression.
- Phase 93 verifier should run `./pre-release-check.sh` and `cargo test --workspace` only after plans 93-01 and 93-03 reach green state and after `deferred-items.md` items are resolved.

---
*Phase: 93-docs-todo-v0-5-7-gaps-md*
*Completed: 2026-04-19*

## Self-Check: PASSED

- FOUND: native-theme/src/detect.rs
- FOUND: native-theme/src/pipeline.rs
- FOUND: native-theme/src/model/icons.rs
- FOUND: .planning/phases/93-docs-todo-v0-5-7-gaps-md/93-02-SUMMARY.md
- FOUND: commit 421c3cc (feat(93-02): add LinuxDesktop::Wayfire variant and wlroots fallback route)
