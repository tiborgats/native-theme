---
phase: 57-verification-and-documentation
plan: 02
subsystem: documentation
tags: [toml, spec-docs, readme, property-registry, inheritance-rules, v0.5.5]

requires:
  - phase: 57-01
    provides: audit findings and acceptance annotations
  - phase: 49-56
    provides: final code implementation (widget structs, resolve.rs, presets)
provides:
  - Synchronized property-registry.toml matching all widget struct definitions
  - Synchronized inheritance-rules.toml matching all resolve.rs inheritance chains
  - All 5 READMEs updated for v0.5.5 API surface and field names
affects: [57-03, release]

tech-stack:
  added: []
  patterns: [spec-code-synchronization]

key-files:
  created: []
  modified:
    - docs/property-registry.toml
    - docs/inheritance-rules.toml
    - README.md
    - native-theme/README.md
    - connectors/native-theme-gpui/README.md
    - connectors/native-theme-iced/README.md
    - native-theme-build/README.md

key-decisions:
  - "Code is source of truth for spec docs -- all discrepancies resolved by updating docs to match code"
  - "13 missing inheritance rules added to inheritance-rules.toml from resolve.rs"

patterns-established:
  - "Spec docs sync: always compare docs against code, not the reverse"

requirements-completed: [VERIFY-03, DOC-03, DOC-04]

duration: 6min
completed: 2026-04-07
---

# Phase 57 Plan 02: Spec-Code Sync and README Updates Summary

**Synchronized property-registry.toml and inheritance-rules.toml with final code, updated all 5 READMEs to v0.5.5 API surface (accent_color, background_color field names, version 0.5.5)**

## Performance

- **Duration:** 5 min 39 sec
- **Started:** 2026-04-07T17:12:47Z
- **Completed:** 2026-04-07T17:18:26Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- property-registry.toml: added missing `line_height` field to TextScaleEntry structure
- inheritance-rules.toml: added 13 missing inheritance rules (defaults.font.color, defaults.border.padding_*, checkbox.background_color/indicator_color, menu.icon_size/hover_background/hover_text_color/disabled_text_color, button.hover_background, sidebar.hover_background, list.hover_background, combo_box.background_color, segmented_control.background_color/active_background/active_text_color)
- All 5 READMEs updated: version 0.5.4 -> 0.5.5 (7 places), stale field names fixed (defaults.accent -> accent_color, defaults.background -> background_color, defaults.radius -> border.corner_radius, accent = -> accent_color = in TOML)

## Task Commits

Each task was committed atomically:

1. **Task 1: Spec-code synchronization** - `e6bd116` (docs)
2. **Task 2: Update all READMEs for v0.5.5 API surface** - `59742e0` (docs)

## Files Created/Modified
- `docs/property-registry.toml` - Added line_height to TextScaleEntry, updated NOTE
- `docs/inheritance-rules.toml` - Added 13 missing inheritance rules from resolve.rs
- `README.md` - Version 0.5.5, accent_color field names
- `native-theme/README.md` - accent_color, background_color, border.corner_radius field names
- `connectors/native-theme-gpui/README.md` - Version 0.5.5
- `connectors/native-theme-iced/README.md` - Version 0.5.5
- `native-theme-build/README.md` - Version 0.5.5

## Decisions Made
- Code is source of truth for spec docs: when resolve.rs has an inheritance rule not in inheritance-rules.toml, the doc is updated (not the code)
- 13 inheritance rules were missing from inheritance-rules.toml but present in resolve.rs -- all were added
- TextScaleEntry.line_height was in code but not in the registry -- added with documentation note

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All spec docs synchronized with code -- ready for Phase 57 Plan 03 (final verification/release prep)
- All READMEs reflect the v0.5.5 API surface
- All cargo tests pass (including doctests and cross-reference tests)

---
*Phase: 57-verification-and-documentation*
*Completed: 2026-04-07*
