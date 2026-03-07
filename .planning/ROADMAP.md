# Roadmap: native-theme

## Overview

native-theme delivers a toolkit-agnostic Rust crate for unified OS theme data. The roadmap builds outward from a stable data model core: first the types and serde layer, then bundled presets for immediate usability, then platform readers (Linux KDE/GNOME, Windows) each as independent feature-gated modules, then cross-platform dispatch tying them together, then extended preset coverage (platform + community themes), and finally documentation with toolkit adapter examples. Each phase produces a verifiable, independently useful increment.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Data Model Foundation** - Rgba, all theme structs, merge macro, error type, serde, and validation tests _(completed 2026-03-07)_
- [x] **Phase 2: Core Presets** - Bundled preset TOML files, preset loading API, and preset tests _(completed 2026-03-07)_
- [ ] **Phase 3: KDE Reader** - Sync Linux KDE reader parsing kdeglobals (feature "kde")
- [ ] **Phase 4: GNOME Portal Reader** - Async Linux GNOME reader via freedesktop portal (feature "portal")
- [ ] **Phase 5: Windows Reader** - Sync Windows reader via UISettings and system metrics (feature "windows")
- [ ] **Phase 6: Cross-Platform Dispatch** - from_system() auto-detection and platform reader unit tests
- [ ] **Phase 7: Extended Presets** - Platform presets (windows-11, macos-sonoma, material, ios) and community presets
- [ ] **Phase 8: Documentation** - README with adapter examples for egui, iced, and slint

## Phase Details

### Phase 1: Data Model Foundation
**Goal**: Developers can define, serialize, deserialize, and layer theme data using the complete type system
**Depends on**: Nothing (first phase)
**Requirements**: CORE-01, CORE-02, CORE-03, CORE-04, CORE-05, CORE-06, CORE-07, CORE-08, CORE-09, CORE-10, SERDE-01, SERDE-02, ERR-01, TEST-01, TEST-03
**Success Criteria** (what must be TRUE):
  1. Rgba parses from and serializes to hex strings (#RRGGBB and #RRGGBBAA) correctly, including edge cases (missing #, 3/4 char shorthand, invalid input)
  2. A NativeTheme with light and dark ThemeVariants (each containing ThemeColors with 36 fields, ThemeFonts, ThemeGeometry, ThemeSpacing) round-trips through TOML with no data loss
  3. A sparse TOML file with only a few fields deserializes successfully (all missing fields are None), and serialization skips None fields
  4. merge() on any theme struct overlays non-None fields from the overlay onto the base, leaving base values where overlay is None
  5. All public structs are non_exhaustive, Send + Sync, Default, Clone, Debug; Error enum has correct variants and implements Display + std::error::Error
**Plans:** 3 plans
Plans:
- [x] 01-01-PLAN.md -- Project scaffold, Rgba color type, Error enum, merge macro
- [x] 01-02-PLAN.md -- All theme model structs (ThemeColors, ThemeFonts, ThemeGeometry, ThemeSpacing, ThemeVariant, NativeTheme)
- [x] 01-03-PLAN.md -- Integration tests (TOML round-trip, merge behavior, trait assertions)

### Phase 2: Core Presets
**Goal**: Users can load bundled theme presets and work with TOML theme files without any platform features
**Depends on**: Phase 1
**Requirements**: PRESET-01, PRESET-02, TEST-02
**Success Criteria** (what must be TRUE):
  1. preset("default"), preset("kde-breeze"), and preset("adwaita") each return a valid NativeTheme with both light and dark variants populated
  2. list_presets() returns all available preset names; from_toml() parses a TOML string into NativeTheme; from_file() loads from a path; to_toml() produces valid TOML
  3. All bundled presets parse without error and contain reasonable values (non-empty color sets, valid font sizes)
**Plans:** 2 plans
Plans:
- [x] 02-01-PLAN.md -- TOML preset files (default, kde-breeze, adwaita) + presets.rs API module + lib.rs wiring
- [x] 02-02-PLAN.md -- Integration tests for preset loading (parse, variants, colors, fonts, round-trip)

### Phase 3: KDE Reader
**Goal**: Apps on KDE Linux desktops can read the user's live theme colors and fonts
**Depends on**: Phase 2
**Requirements**: PLAT-01
**Success Criteria** (what must be TRUE):
  1. from_kde() returns a NativeTheme populated from ~/.config/kdeglobals with accent, background, foreground, and selection colors mapped to semantic roles
  2. from_kde() handles missing kdeglobals file, missing color groups, and malformed entries gracefully (returns Error::Unavailable or partial theme, never panics)
  3. KDE font strings from both Qt 4 (10 fields) and Qt 5/6 (16 fields) formats parse correctly into ThemeFonts
**Plans**: TBD

### Phase 4: GNOME Portal Reader
**Goal**: Apps on GNOME Linux desktops can read the user's theme via the freedesktop settings portal
**Depends on**: Phase 2
**Requirements**: PLAT-02
**Success Criteria** (what must be TRUE):
  1. from_gnome() returns a NativeTheme with accent color, color scheme (light/dark), and contrast preference read from the portal
  2. When portal values are unavailable (no D-Bus session, sandboxed environment), from_gnome() falls back to hardcoded Adwaita defaults rather than failing
  3. The "portal" feature compiles without pulling in tokio when the consumer uses async-io (ashpd default features disabled)
**Plans**: TBD

### Phase 5: Windows Reader
**Goal**: Apps on Windows can read the user's live accent colors and system metrics
**Depends on**: Phase 2
**Requirements**: PLAT-04
**Success Criteria** (what must be TRUE):
  1. from_windows() returns a NativeTheme with accent color, foreground/background from UISettings, and geometry values from GetSystemMetrics
  2. from_windows() degrades gracefully on older Windows versions where UISettings APIs are unavailable (returns partial theme or Error::Unavailable)
  3. The "windows" feature only pulls in the minimal windows crate features needed (UI_ViewManagement, Win32_UI_WindowsAndMessaging)
**Plans**: TBD

### Phase 6: Cross-Platform Dispatch
**Goal**: Apps can call one function to get the current OS theme regardless of platform
**Depends on**: Phase 3, Phase 4, Phase 5
**Requirements**: PLAT-03, TEST-04
**Success Criteria** (what must be TRUE):
  1. from_system() on Linux auto-detects KDE vs GNOME and calls the appropriate reader; on Windows calls from_windows(); on unsupported platforms returns Error::Unsupported
  2. from_system() compiles on all platforms regardless of which reader features are enabled (missing features produce Error::Unsupported at runtime)
  3. Platform reader unit tests pass with mock/fixture data for each supported platform (KDE kdeglobals fixture, portal mock, Windows mock)
**Plans**: TBD

### Phase 7: Extended Presets
**Goal**: Users have preset themes covering all major platforms and popular community color schemes
**Depends on**: Phase 1
**Requirements**: PRESET-03, PRESET-04
**Success Criteria** (what must be TRUE):
  1. Platform presets (windows-11, macos-sonoma, material, ios) are available via preset() and contain accurate light and dark variants reflecting each platform's design language
  2. Community presets (Catppuccin Latte/Frappe/Macchiato/Mocha, Nord, Dracula, Gruvbox, Solarized, Tokyo Night, One Dark) are available via preset() with correct color mappings
  3. All extended presets pass round-trip TOML serialization and contain non-empty ThemeColors in both variants
**Plans**: TBD

### Phase 8: Documentation
**Goal**: Developers can integrate native-theme into any Rust GUI app by following documented examples
**Depends on**: Phase 7
**Requirements**: DOC-01
**Success Criteria** (what must be TRUE):
  1. README contains working adapter code examples for egui, iced, and slint that map NativeTheme fields to each toolkit's styling API
  2. README documents the preset workflow (load preset, merge user overrides) and the runtime workflow (from_system() with preset fallback)
  3. README documents feature flags and their platform requirements
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order. Phase 3, 4, 5 can potentially execute in parallel (all depend on Phase 2, not on each other). Phase 7 can execute in parallel with Phases 3-6 (depends only on Phase 1).

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Data Model Foundation | 3/3 | ✓ Complete | 2026-03-07 |
| 2. Core Presets | 2/2 | ✓ Complete | 2026-03-07 |
| 3. KDE Reader | 0/? | Not started | - |
| 4. GNOME Portal Reader | 0/? | Not started | - |
| 5. Windows Reader | 0/? | Not started | - |
| 6. Cross-Platform Dispatch | 0/? | Not started | - |
| 7. Extended Presets | 0/? | Not started | - |
| 8. Documentation | 0/? | Not started | - |
