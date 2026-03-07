# Requirements: native-theme

**Defined:** 2026-03-07
**Core Value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.

## v1 Requirements

Requirements for initial release. Each maps to roadmap phases.

### Core Data Model

- [x] **CORE-01**: Rgba color type with 8-bit-per-channel sRGB + alpha, custom hex serde (#RRGGBB / #RRGGBBAA)
- [ ] **CORE-02**: ThemeColors struct with 36 semantic color roles, all fields Option<Rgba>
- [ ] **CORE-03**: ThemeFonts struct with family, size, monospace family/size, all Option<T>
- [ ] **CORE-04**: ThemeGeometry struct with border radius, border width, disabled/border opacity, all Option<f32>
- [ ] **CORE-05**: ThemeSpacing struct with named spacing scale (xxs through xxl), all Option<f32>
- [ ] **CORE-06**: ThemeVariant composing ThemeColors, ThemeFonts, ThemeGeometry, ThemeSpacing
- [ ] **CORE-07**: NativeTheme with name, light variant, dark variant
- [x] **CORE-08**: merge() method on all structs via declarative macro for theme layering
- [x] **CORE-09**: All public structs #[non_exhaustive] for forward compatibility
- [x] **CORE-10**: All types Send + Sync, Default, Clone, Debug

### Serialization

- [ ] **SERDE-01**: TOML serialization/deserialization mapping 1:1 to struct field names
- [ ] **SERDE-02**: #[serde(default)] + skip_serializing_if = "Option::is_none" on all fields for sparse TOML support

### Presets

- [ ] **PRESET-01**: Bundled core presets embedded via include_str!(): default, kde-breeze, adwaita (light + dark each)
- [ ] **PRESET-02**: Preset loading API: preset(), list_presets(), from_toml(), from_file(), to_toml()
- [ ] **PRESET-03**: Additional platform presets: windows-11, macos-sonoma, material, ios
- [ ] **PRESET-04**: Community presets: Catppuccin (4 flavors), Nord, Dracula, Gruvbox, Solarized, Tokyo Night, One Dark

### Error Handling

- [x] **ERR-01**: Error enum with Unsupported, Unavailable, Format, Platform variants + Display + std::error::Error

### Platform Readers

- [ ] **PLAT-01**: Linux KDE reader: from_kde() -- sync, parses ~/.config/kdeglobals via configparser (feature "kde")
- [ ] **PLAT-02**: Linux GNOME reader: from_gnome() -- async, reads freedesktop portal via ashpd (feature "portal")
- [ ] **PLAT-03**: Cross-platform dispatch: from_system() -- auto-detects platform/DE, calls appropriate reader
- [ ] **PLAT-04**: Windows reader: from_windows() -- UISettings + GetSystemMetrics (feature "windows")

### Documentation

- [ ] **DOC-01**: README with adapter examples for egui, iced, and slint

### Testing

- [ ] **TEST-01**: Round-trip serde tests for all types
- [ ] **TEST-02**: Preset loading tests (all presets parse correctly)
- [x] **TEST-03**: Rgba hex parsing edge cases (3/4/6/8 char, with/without #, invalid)
- [ ] **TEST-04**: Platform reader unit tests with mock data

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Platform Readers

- **PLAT-05**: macOS reader: from_macos() -- NSColor + NSFont via objc2-app-kit (feature "macos")
- **PLAT-06**: iOS reader: from_ios() -- UIColor + UIFont via objc2-ui-kit (feature "ios")
- **PLAT-07**: Android reader: from_android() -- JNI + NDK for Material You colors (feature "android")

### Advanced Features

- **ADV-01**: Widget-level metrics (button height, scrollbar width, checkbox size)
- **ADV-02**: File watching for live theme change detection (notify crate, feature "watch")

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Built-in toolkit adapters (egui, iced, etc.) | Couples crate to toolkit versions; adapters live in consumer code (~50 lines each) |
| Named palette colors (system red, system blue) | Too platform-specific; semantic status colors (error, success, warning) cover practical use cases |
| Accessibility flags in data model | Dark/light, high contrast, reduced motion are environment signals detected by consuming app |
| Reactive change notification system | Complex, opinionated (which async runtime?); consuming toolkit already provides event loops |
| CSS/SCSS export format | Trivially implementable by consumers; one-liner per field |
| W3C design token format (DTCG JSON) | Different schema, partial overlap, low demand from Rust GUI developers |
| Color space conversion utilities | Out of scope for data crate; use palette crate for color math |
| Runtime theme animation/interpolation | Toolkit-specific behavior; data crate cannot own animation timing |
| crates.io publishing | Not in scope for this milestone (code + tests only) |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| CORE-01 | Phase 1 | Complete |
| CORE-02 | Phase 1 | Pending |
| CORE-03 | Phase 1 | Pending |
| CORE-04 | Phase 1 | Pending |
| CORE-05 | Phase 1 | Pending |
| CORE-06 | Phase 1 | Pending |
| CORE-07 | Phase 1 | Pending |
| CORE-08 | Phase 1 | Complete |
| CORE-09 | Phase 1 | Complete |
| CORE-10 | Phase 1 | Complete |
| SERDE-01 | Phase 1 | Pending |
| SERDE-02 | Phase 1 | Pending |
| PRESET-01 | Phase 2 | Pending |
| PRESET-02 | Phase 2 | Pending |
| PRESET-03 | Phase 7 | Pending |
| PRESET-04 | Phase 7 | Pending |
| ERR-01 | Phase 1 | Complete |
| PLAT-01 | Phase 3 | Pending |
| PLAT-02 | Phase 4 | Pending |
| PLAT-03 | Phase 6 | Pending |
| PLAT-04 | Phase 5 | Pending |
| DOC-01 | Phase 8 | Pending |
| TEST-01 | Phase 1 | Pending |
| TEST-02 | Phase 2 | Pending |
| TEST-03 | Phase 1 | Complete |
| TEST-04 | Phase 6 | Pending |

**Coverage:**
- v1 requirements: 26 total
- Mapped to phases: 26
- Unmapped: 0

---
*Requirements defined: 2026-03-07*
*Last updated: 2026-03-07 after roadmap creation*
