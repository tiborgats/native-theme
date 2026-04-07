# Roadmap: native-theme

## Overview

native-theme delivers a toolkit-agnostic Rust crate for unified OS theme data. The roadmap builds outward from a stable data model core: first the types and serde layer, then bundled presets for immediate usability, then platform readers (Linux KDE/GNOME, Windows) each as independent feature-gated modules, then cross-platform dispatch tying them together, then extended preset coverage (platform + community themes), and finally documentation with toolkit adapter examples. Each phase produces a verifiable, independently useful increment.

## Milestones

- ✅ **v0.1 MVP** — Phases 1-8 (shipped 2026-03-07)
- ✅ **v0.2 Platform Coverage & Publishing** — Phases 9-15 (shipped 2026-03-09)
- ✅ **v0.3 Icons** — Phases 16-21 (shipped 2026-03-09)
- ✅ **v0.3.3 Custom Icon Roles** — Phases 22-26 (shipped 2026-03-17)
- ✅ **v0.4.0 Animated Icons** — Phases 27-32 (shipped 2026-03-18)
- ✅ **v0.4.1 Release Prep** — Phases 33-43 (shipped 2026-03-21)
- ✅ **v0.5.0 Per-Widget Architecture & Resolution Pipeline** — Phases 44-48 (shipped 2026-03-29)
- 🚧 **v0.5.5 Schema Overhaul & Quality** — Phases 49-57 (in progress)

## Phases

<details>
<summary>v0.1 MVP (Phases 1-8) — SHIPPED 2026-03-07</summary>

- [x] Phase 1: Data Model Foundation (3/3 plans) — completed 2026-03-07
- [x] Phase 2: Core Presets (2/2 plans) — completed 2026-03-07
- [x] Phase 3: KDE Reader (2/2 plans) — completed 2026-03-07
- [x] Phase 4: GNOME Portal Reader (2/2 plans) — completed 2026-03-07
- [x] Phase 5: Windows Reader (1/1 plan) — completed 2026-03-07
- [x] Phase 6: Cross-Platform Dispatch (1/1 plan) — completed 2026-03-07
- [x] Phase 7: Extended Presets (2/2 plans) — completed 2026-03-07
- [x] Phase 8: Documentation (1/1 plan) — completed 2026-03-07

</details>

<details>
<summary>v0.2 Platform Coverage & Publishing (Phases 9-15) — SHIPPED 2026-03-09</summary>

- [x] Phase 9: Cargo Workspace (1/1 plan) — completed 2026-03-08
- [x] Phase 10: API Breaking Changes (3/3 plans) — completed 2026-03-08
- [x] Phase 11: Platform Readers (4/4 plans) — completed 2026-03-08
- [x] Phase 12: Widget Metrics (3/3 plans) — completed 2026-03-08
- [x] Phase 13: CI Pipeline (1/1 plan) — completed 2026-03-08
- [x] Phase 14: Toolkit Connectors (5/5 plans) — completed 2026-03-09
- [x] Phase 15: Publishing Prep (3/3 plans) — completed 2026-03-09

</details>

<details>
<summary>v0.3 Icons (Phases 16-21) — SHIPPED 2026-03-09</summary>

- [x] Phase 16: Icon Data Model (2/2 plans) — completed 2026-03-09
- [x] Phase 17: Bundled SVG Icons (2/2 plans) — completed 2026-03-09
- [x] Phase 18: Linux Icon Loading (1/1 plan) — completed 2026-03-09
- [x] Phase 19: macOS Icon Loading (1/1 plan) — completed 2026-03-09
- [x] Phase 20: Windows Icon Loading (1/1 plan) — completed 2026-03-09
- [x] Phase 21: Integration and Connectors (3/3 plans) — completed 2026-03-09

</details>

<details>
<summary>v0.3.3 Custom Icon Roles (Phases 22-26) — SHIPPED 2026-03-17</summary>

- [x] Phase 22: Core Trait and Loading Functions (2/2 plans) — completed 2026-03-15
- [x] Phase 23: Build Crate and Code Generation (5/5 plans) — completed 2026-03-16
- [x] Phase 24: Linux DE Audit and Freedesktop DE-Aware Mapping (2/2 plans) — completed 2026-03-16
- [x] Phase 25: Connector Integration (1/1 plan) — completed 2026-03-16
- [x] Phase 25.1: Icon Gaps and Fallback Removal (2/2 plans) — completed 2026-03-17
- [x] Phase 26: Documentation and Release (2/2 plans) — completed 2026-03-17

</details>

<details>
<summary>v0.4.0 Animated Icons (Phases 27-32) — SHIPPED 2026-03-18</summary>

- [x] Phase 27: Animation Data Model and Breaking Changes (2/2 plans) — completed 2026-03-18
- [x] Phase 28: Bundled SVG Spinner Frames (2/2 plans) — completed 2026-03-18
- [x] Phase 29: Freedesktop Sprite Sheet Parser (1/1 plan) — completed 2026-03-18
- [x] Phase 30: Reduced Motion Accessibility (1/1 plan) — completed 2026-03-18
- [x] Phase 31: Connector Integration (1/1 plan) — completed 2026-03-18
- [x] Phase 32: Documentation and Release (1/1 plan) — completed 2026-03-18

</details>

<details>
<summary>v0.4.1 Release Prep (Phases 33-43) — SHIPPED 2026-03-21</summary>

- [x] Phase 33: Quick Fixes and Version Consistency (1/1 plan) — completed 2026-03-19
- [x] Phase 34: Animated Icon Documentation (1/1 plan) — completed 2026-03-19
- [x] Phase 35: Animated Icon Showcase Examples (2/2 plans) — completed 2026-03-19
- [x] Phase 36: Screenshot and GIF Generation (3/3 plans) — completed 2026-03-20
- [x] Phase 37: Community Files and GitHub Templates (2/2 plans) — completed 2026-03-20
- [x] Phase 38: CI, Smoke Tests, and Release (3/3 plans) — completed 2026-03-20
- [x] Phase 39: Code Quality and Housekeeping (2/2 plans) — completed 2026-03-20
- [x] Phase 40: Iced Theme Preset Screenshots and CI (2/2 plans) — completed 2026-03-20
- [x] Phase 41: gpui Theme Preset Screenshots (2/2 plans) — completed 2026-03-21
- [x] Phase 42: Theme-Switching GIF and Core README Images (1/1 plan) — completed 2026-03-21
- [x] Phase 43: Self-Capture Screenshots with Window Decorations (3/3 plans) — completed 2026-03-21

</details>

<details>
<summary>v0.5.0 Per-Widget Architecture & Resolution Pipeline (Phases 44-48) — SHIPPED 2026-03-29</summary>

- [x] Phase 44: Per-Widget Data Model and Preset Migration (3/3 plans) — completed 2026-03-27
- [x] Phase 45: Resolution Engine (3/3 plans) — completed 2026-03-27
- [x] Phase 46: OS Reader Extensions (6/6 plans) — completed 2026-03-27
- [x] Phase 47: OS-First Pipeline (2/2 plans) — completed 2026-03-27
- [x] Phase 48: Connector Migration (3/3 plans) — completed 2026-03-28

</details>

### v0.5.5 Schema Overhaul & Quality (Phases 49-57)

- [x] **Phase 49: Additive Type Definitions** - Define BorderSpec, FontStyle, FontSpec extensions, and LayoutTheme as non-breaking additions (completed 2026-04-06)
- [x] **Phase 50: Atomic Schema Commit** - All ~70 renames + BorderSpec integration + font.color + foreground removal + ThemeSpacing removal + all 17 preset rewrites in one commit (completed 2026-04-07)
- [x] **Phase 51: Resolution Engine Overhaul** - resolve_border(), resolve_font() update, safety net removal, text_scale removal, inheritance bug fixes (completed 2026-04-07)
- [x] **Phase 52: Interactive State Colors** - ~70 new hover/active/disabled/focus fields across 18 widgets with preset values (completed 2026-04-07)
- [x] **Phase 53: Preset Completeness** - text_scale for 13 missing presets, interactive state color values for all 17 presets (completed 2026-04-07)
- [x] **Phase 54: Connector Migration** - Both connectors updated for new schema, derive.rs replaced with direct theme reads (completed 2026-04-07)
- [x] **Phase 55: Correctness, Safety, and CI** - Bug fixes, animation safety guards, CI improvements, gsettings timeout (completed 2026-04-07)
- [x] **Phase 56: Testing** - Property-based tests and programmatic platform-facts cross-reference (completed 2026-04-07)
- [ ] **Phase 57: Verification and Documentation** - Full audit, spec-code sync, READMEs, CHANGELOG

## Phase Details

Phase details for milestones v0.1 through v0.5.0 are archived in `.planning/milestones/`.

### Phase 49: Additive Type Definitions
**Goal**: All new types (BorderSpec, FontStyle, FontSpec.style, FontSpec.color, LayoutTheme) exist as compilable, testable Rust structs without breaking any existing code
**Depends on**: Nothing (first phase of v0.5.5)
**Requirements**: SCHEMA-01, SCHEMA-02, SCHEMA-03
**Success Criteria** (what must be TRUE):
  1. `BorderSpec` and `ResolvedBorderSpec` structs exist with all 6 fields (color, corner_radius, line_width, shadow_enabled, padding_horizontal, padding_vertical), and round-trip through TOML serde correctly
  2. `FontStyle` enum (Normal, Italic, Oblique) exists with serde lowercase rename, and `FontSpec`/`ResolvedFontSpec` have `style` and `color` fields
  3. `LayoutTheme` and `ResolvedLayoutTheme` exist via `define_widget_pair!` with 4 fields (widget_gap, container_margin, window_margin, section_gap)
  4. `cargo expand` confirms `define_widget_pair!` correctly handles a widget with both `optional_nested font` and `optional_nested border` (tested on ButtonTheme definition)
  5. `pre-release-check.sh` passes (VERIFY-01 gate)
**Plans**: 3 plans
Plans:
- [x] 49-01-PLAN.md — BorderSpec and FontSpec type definitions
- [x] 49-02-PLAN.md — LayoutTheme and define_widget_pair! macro support
- [x] 49-03-PLAN.md — SC4 dual optional_nested verification

### Phase 50: Atomic Schema Commit
**Goal**: The entire data model matches property-registry.toml naming conventions, with BorderSpec sub-structs replacing flat border fields, font.color replacing foreground fields, ThemeSpacing removed, Layout widget added, and all 17 presets rewritten to the new TOML structure -- in a single atomic commit
**Depends on**: Phase 49
**Requirements**: SCHEMA-04, SCHEMA-06, SCHEMA-07, SCHEMA-08, PRESET-01
**Success Criteria** (what must be TRUE):
  1. Every field name in the Rust widget structs matches its corresponding entry in `property-registry.toml` (no RENAME mismatches remain from the REG-4 audit)
  2. All 17 presets load via `from_toml()` without error and use `[widget.border]` nested table sections instead of flat border fields
  3. No `foreground` fields exist on any widget struct -- text colors live in `font.color` (or named font like `item_font.color`, `header_font.color`)
  4. `ThemeSpacing` struct is removed, `defaults.spacing` field is gone, and `[layout]` section exists in all 17 presets
  5. `pre-release-check.sh` passes (VERIFY-01 gate)
**Plans**: 4 plans
Plans:
- [x] 50-01-PLAN.md — Model layer struct renames, BorderSpec expansion, ThemeSpacing removal
- [x] 50-02-PLAN.md — resolve.rs and validate.rs field reference renames
- [x] 50-03-PLAN.md — OS readers, presets.rs, and all 20 TOML preset rewrites
- [x] 50-04-PLAN.md — Connector field reference updates and final verification

### Phase 51: Resolution Engine Overhaul
**Goal**: resolve.rs correctly implements all inheritance rules from inheritance-rules.toml, with zero invented values -- all safety nets removed, text_scale computation removed, scrollbar.thumb_hover computation replaced, and inheritance bugs fixed
**Depends on**: Phase 50
**Requirements**: RESOLVE-01, RESOLVE-02, RESOLVE-03, RESOLVE-04, RESOLVE-05, RESOLVE-06, RESOLVE-07
**Success Criteria** (what must be TRUE):
  1. `resolve_border()` function exists and implements sub-field inheritance for all 13 widgets listed in `[border_inheritance]`, with corner_radius_lg exceptions for window/popover/dialog
  2. `resolve_font()` handles all 5 sub-fields (family, size, weight, style, color), with the link.font.color exception inheriting from defaults.link_color
  3. Zero hardcoded safety-net values remain in resolve.rs (no 1.2 line_height, no #ffffff accent_foreground, no rgba(0,0,0,64) shadow, no text_scale ratio computation) -- a preset with any missing required field causes a `validate()` error
  4. All 3 inheritance bugs fixed: INH-1 (input.selection uses text_selection source), INH-2 (dialog.background_color has per-platform fallback), INH-3 (card border inheritance removed)
  5. `pre-release-check.sh` passes with all 17 presets resolving successfully through the full pipeline (VERIFY-01 gate)
**Plans**: TBD

### Phase 52: Interactive State Colors
**Goal**: All 18 widgets that need interactive state colors (hover, active, disabled, focus) have the fields defined in their structs, with inheritance rules in resolve.rs
**Depends on**: Phase 51
**Requirements**: SCHEMA-05
**Success Criteria** (what must be TRUE):
  1. All ~70 interactive state color fields from the todo_v0.5.5.md audit exist on their respective widget structs (hover_background, hover_text_color, active_background, disabled_background, etc.)
  2. Inheritance rules for state colors exist in resolve.rs (disabled_opacity inherits from defaults.disabled_opacity, hover_text_color inherits from font.color where documented)
  3. All 17 presets compile and load with the new fields (fields are Option, so presets without values still load)
  4. `pre-release-check.sh` passes (VERIFY-01 gate)
**Plans**: 2 plans
Plans:
- [x] 52-01-PLAN.md — Batch 1: Button, Input, Checkbox, Scrollbar, Slider (18 fields)
- [x] 52-02-PLAN.md — Batch 2: Tab, List, Splitter, Switch, ComboBox, SegCtrl, Expander, Link (19 fields)

### Phase 53: Preset Completeness
**Goal**: All 17 presets have explicit values for text_scale entries and interactive state colors -- no preset relies on runtime computation or safety nets for any field
**Depends on**: Phase 52
**Requirements**: PRESET-02, PRESET-03
**Success Criteria** (what must be TRUE):
  1. All 17 presets have explicit `[light.text_scale]` and `[dark.text_scale]` sections with size and weight for all 4 entries (caption, section_heading, dialog_title, display) -- the 13 presets that were missing them now have real platform-appropriate values
  2. All 17 presets have explicit interactive state color values (hover_background, active_background, disabled states) for every widget that defines those fields
  3. Every preset passes the full resolve() -> validate() pipeline with zero inheritance fallbacks firing for state colors or text_scale
  4. `pre-release-check.sh` passes (VERIFY-01 gate)
**Plans**: 5 plans
Plans:
- [x] 53-01-PLAN.md — Adwaita + KDE Breeze text_scale and interactive state colors
- [x] 53-02-PLAN.md — macOS Sonoma + Windows 11 interactive state colors
- [x] 53-03-PLAN.md — iOS + Material + Catppuccin text_scale and interactive state colors
- [x] 53-04-PLAN.md — Community presets (nord, dracula, gruvbox, solarized, tokyo-night, one-dark) text_scale and interactive state colors
- [x] 53-05-PLAN.md — Comprehensive preset verification

### Phase 54: Connector Migration
**Goal**: Both connectors (gpui and iced) use the new field names, read from BorderSpec and font.color directly, and replace derive.rs color computations with direct theme field reads where the theme now provides explicit values
**Depends on**: Phase 53
**Requirements**: CONNECT-01, CONNECT-02, CONNECT-03
**Success Criteria** (what must be TRUE):
  1. Both connectors compile against the new ResolvedThemeVariant field names (background_color, text_color, border.corner_radius, font.color, etc.)
  2. derive.rs hover/active/disabled computations are replaced with direct theme field reads for all fields the theme now provides (hover_background, active_background, disabled states)
  3. Connector inconsistencies fixed: K-1 (display name uses spec.name), K-2 (gpui from_system returns is_dark), K-3 (iced gets contrast enforcement), K-4 (dead code removed), K-5 (unnecessary clone removed)
  4. `pre-release-check.sh` passes (VERIFY-01 gate)
**Plans**: 3 plans
Plans:
- [x] 54-01-PLAN.md — GPUI derive.rs replacement with direct theme reads + dead code removal (K-4)
- [x] 54-02-PLAN.md — GPUI API fixes: display name (K-1), from_system 3-tuple (K-2), clone removal (K-5)
- [x] 54-03-PLAN.md — Iced WCAG contrast enforcement for status foregrounds (K-3)

### Phase 55: Correctness, Safety, and CI
**Goal**: All correctness bugs, animation safety issues, and CI gaps identified in the audit are fixed
**Depends on**: Phase 50 (uses new field names, but independent of resolve/preset/connector phases)
**Requirements**: CORRECT-01, CORRECT-02, CORRECT-03, CORRECT-04, CORRECT-05, CI-01, CI-02, CI-03, CI-04, CI-05
**Success Criteria** (what must be TRUE):
  1. `detect_is_dark()` checks GTK_THEME env var and gtk-3.0/settings.ini as fallback for non-GNOME/non-KDE Linux desktops (C-1)
  2. `detect_platform()` returns "ios" on `target_os = "ios"` (C-2), and `into_resolved()` has correct `#[must_use]` message (C-3)
  3. Spinner safety guards in place: width/height > 0 check (S-1), empty frames guard (S-3), zero duration guard (S-4), single-quote viewBox handling (S-5)
  4. gsettings commands have a timeout to prevent indefinite blocking (R-1)
  5. CI publish workflow tests gpui connector (P-1), error handling improved (P-2), async-io variants tested (P-3), examples disambiguated (P-4), pre-release.sh has iteration timeout (P-5)
**Plans**: 3 plans
Plans:
- [x] 55-01-PLAN.md — Correctness and reliability fixes (detect_is_dark, detect_platform, #[must_use], gsettings timeout)
- [x] 55-02-PLAN.md — Spinner safety guards (viewBox parsing, dimension validation, empty frames, duration assertion)
- [x] 55-03-PLAN.md — CI/publishing gaps (gpui gate, error handling, async-io variants, example rename, pre-release timeout)

### Phase 56: Testing
**Goal**: Property-based tests verify TOML round-trip and merge correctness, and a programmatic cross-reference catches drift between platform-facts.md and preset values
**Depends on**: Phase 53
**Requirements**: TEST-01, TEST-02
**Success Criteria** (what must be TRUE):
  1. Property-based tests (proptest or quickcheck) exist for TOML round-trip serialization, Rgba hex parsing, and merge() semantics -- random theme values survive serialize -> deserialize -> compare
  2. A test parses platform-facts.md and spot-checks key values against resolved presets for the 4 platform presets (windows-11, macos-sonoma, kde-breeze, adwaita), catching any drift between documentation and actual data
**Plans**: 2 plans
Plans:
- [x] 56-01-PLAN.md — Property-based tests (proptest) for TOML round-trip and merge semantics
- [x] 56-02-PLAN.md — Platform-facts cross-reference tests for 4 platform presets

### Phase 57: Verification and Documentation
**Goal**: Every item in the v0.5.5 audit is confirmed implemented, all spec docs are synchronized with the final code, and all READMEs and CHANGELOG reflect the new API surface
**Depends on**: Phase 54, Phase 55, Phase 56
**Requirements**: VERIFY-01, VERIFY-02, VERIFY-03, DOC-01, DOC-02, DOC-03, DOC-04, DOC-05
**Success Criteria** (what must be TRUE):
  1. Line-by-line review of `docs/todo_v0.5.5.md` confirms every actionable item is implemented, with each item checked off or annotated with the resolution
  2. `property-registry.toml`, `inheritance-rules.toml`, and `platform-facts.md` match the final Rust struct definitions and resolve.rs implementation -- zero contradictions remain (VERIFY-03)
  3. Widget struct doc comments are accurate (W-2 fixes), hardcoded connector opacity values are documented (K-6), all 4 READMEs updated for new API surface
  4. CHANGELOG has complete breaking change list with migration notes for every renamed field, removed struct, and changed TOML format
  5. `pre-release-check.sh` passes as final gate (VERIFY-01)
**Plans**: 3 plans
Plans:
- [x] 57-01-PLAN.md — Verification audit (todo_v0.5.5.md, REQUIREMENTS.md), doc comment fixes (DOC-01), opacity documentation (DOC-02)
- [ ] 57-02-PLAN.md — Spec-code synchronization (VERIFY-03, DOC-03) and README updates (DOC-04)
- [ ] 57-03-PLAN.md — CHANGELOG entry (DOC-05) and final pre-release gate (VERIFY-01)

## Progress

**Execution Order:**
Phases execute in numeric order: 49 -> 50 -> 51 -> 52 -> 53 -> 54 -> 55 -> 56 -> 57
Note: Phase 55 depends only on Phase 50 and can run in parallel with 51-54 if desired.

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1-8 | v0.1 | 14/14 | Complete | 2026-03-07 |
| 9-15 | v0.2 | 20/20 | Complete | 2026-03-09 |
| 16-21 | v0.3 | 10/10 | Complete | 2026-03-09 |
| 22-26 | v0.3.3 | 14/14 | Complete | 2026-03-17 |
| 27-32 | v0.4.0 | 8/8 | Complete | 2026-03-18 |
| 33-43 | v0.4.1 | 22/22 | Complete | 2026-03-21 |
| 44-48 | v0.5.0 | 17/17 | Complete | 2026-03-29 |
| 49. Additive Type Definitions | v0.5.5 | 3/3 | Complete   | 2026-04-06 |
| 50. Atomic Schema Commit | v0.5.5 | 4/4 | Complete   | 2026-04-07 |
| 51. Resolution Engine Overhaul | v0.5.5 | 5/5 | Complete   | 2026-04-07 |
| 52. Interactive State Colors | v0.5.5 | 2/2 | Complete   | 2026-04-07 |
| 53. Preset Completeness | v0.5.5 | 5/5 | Complete   | 2026-04-07 |
| 54. Connector Migration | v0.5.5 | 3/3 | Complete   | 2026-04-07 |
| 55. Correctness, Safety, and CI | v0.5.5 | 3/3 | Complete   | 2026-04-07 |
| 56. Testing | v0.5.5 | 2/2 | Complete   | 2026-04-07 |
| 57. Verification and Documentation | v0.5.5 | 1/3 | In Progress|  |
