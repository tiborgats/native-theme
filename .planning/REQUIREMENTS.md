# Requirements: native-theme

**Defined:** 2026-03-08
**Core Value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.

## v0.2 Requirements

Requirements for v0.2 release. Each maps to roadmap phases.

### API Refactors & Structure

- [x] **API-01**: Repo converted to Cargo workspace with core crate in `native-theme/` subdirectory
- [x] **API-02**: ThemeColors flattened to 36 direct `Option<Rgba>` fields (no nested sub-structs)
- [x] **API-03**: All presets updated to flat `[light.colors]` / `[dark.colors]` TOML format
- [x] **API-04**: Platform readers updated for flat ThemeColors field access
- [x] **API-05**: Preset functions moved to `impl NativeTheme` associated functions (`NativeTheme::preset()`, `::from_toml()`, `::from_file()`, `::list_presets()`, `theme.to_toml()`)
- [x] **API-06**: Old free functions removed (no deprecation period, pre-1.0)
- [x] **API-07**: `ThemeGeometry` gains `radius_lg: Option<f32>` and `shadow: Option<bool>` fields
- [x] **API-08**: Presets updated with radius_lg and shadow data where applicable

### Platform Readers

- [x] **PLAT-01**: macOS reader `from_macos()` reads ~20 NSColor semantic colors with P3-to-sRGB conversion
- [x] **PLAT-02**: macOS reader resolves both light and dark variants via NSAppearance
- [x] **PLAT-03**: macOS reader reads NSFont system and monospace fonts
- [x] **PLAT-04**: macOS reader wired into `from_system()` dispatch
- [x] **PLAT-05**: Windows reader adds `ApiInformation::IsMethodPresent` capability checks
- [x] **PLAT-06**: Windows reader reads AccentDark1-3 and AccentLight1-3 accent shades
- [x] **PLAT-07**: Windows reader reads system font via `SystemParametersInfo(SPI_GETNONCLIENTMETRICS)`
- [x] **PLAT-08**: Windows reader populates spacing from WinUI3 defaults and derives `primary_foreground`
- [x] **PLAT-09**: Windows reader uses DPI-aware `GetSystemMetricsForDpi` for geometry
- [x] **PLAT-10**: Linux `from_kde_with_portal()` async overlay of portal accent on kdeglobals palette
- [x] **PLAT-11**: Linux D-Bus portal backend detection for DE heuristic
- [x] **PLAT-12**: GNOME font reading from gsettings/dconf (`org.gnome.desktop.interface font-name`)
- [x] **PLAT-13**: `from_linux()` fallback: try kdeglobals if file exists on non-KDE desktops

### Widget Metrics

- [x] **METRIC-01**: `WidgetMetrics` struct with 12 per-widget sub-structs (Button, Checkbox, Input, Scrollbar, Slider, ProgressBar, Tab, MenuItem, Tooltip, ListItem, Toolbar, Splitter)
- [x] **METRIC-02**: Each sub-struct uses `Option<f32>` fields, `#[non_exhaustive]`, serde defaults
- [x] **METRIC-03**: `widget_metrics: Option<WidgetMetrics>` added to `ThemeVariant`
- [ ] **METRIC-04**: KDE metrics populated from breezemetrics.h constants (versioned per Plasma release)
- [ ] **METRIC-05**: Windows metrics populated via `GetSystemMetricsForDpi` at runtime
- [ ] **METRIC-06**: macOS metrics populated with hardcoded HIG defaults
- [ ] **METRIC-07**: GNOME metrics populated from hardcoded libadwaita values
- [x] **METRIC-08**: Widget metrics added to preset TOML files

### CI Pipeline

- [ ] **CI-01**: GitHub Actions workflow testing on Linux + Windows + macOS runners
- [ ] **CI-02**: Feature flag matrix: `--no-default-features`, `--features kde`, `--features portal-tokio`, `--features windows`, `--features macos`
- [ ] **CI-03**: `cargo semver-checks` integrated for breaking change detection
- [ ] **CI-04**: `cargo clippy` + `cargo fmt --check` in CI

### Toolkit Connectors

- [ ] **CONN-01**: `native-theme-gpui` crate maps ThemeColors to gpui-component's 108 ThemeColor fields (direct + derived)
- [ ] **CONN-02**: `native-theme-gpui` maps fonts, geometry, spacing, and widget metrics
- [ ] **CONN-03**: `native-theme-gpui` includes upstream PR proposal documents for missing gpui-component theming hooks
- [ ] **CONN-04**: `native-theme-gpui` includes `examples/showcase.rs` widget gallery
- [ ] **CONN-05**: `native-theme-iced` crate maps ThemeColors to iced Palette + Extended palette
- [ ] **CONN-06**: `native-theme-iced` implements per-widget Catalog/Style for core widgets (Button, Container, TextInput, Scrollable, Checkbox, Slider, ProgressBar, Tooltip)
- [ ] **CONN-07**: `native-theme-iced` maps geometry, spacing, and widget metrics to Style fields
- [ ] **CONN-08**: `native-theme-iced` includes `examples/demo.rs` widget gallery
- [ ] **CONN-09**: Both connectors include a theme selector (dropdown of presets + OS theme)

### Publishing Prep

- [ ] **PUB-01**: Cargo.toml metadata: `rust-version`, `repository`, `homepage`, `keywords`, `categories`, `readme`
- [ ] **PUB-02**: LICENSE-MIT, LICENSE-APACHE, LICENSE-0BSD files at repo root
- [ ] **PUB-03**: CHANGELOG.md following Keep a Changelog format
- [ ] **PUB-04**: Doc examples (`/// # Examples`) on `NativeTheme`, `Rgba`, `ThemeVariant`
- [ ] **PUB-05**: IMPLEMENTATION.md spec updated to match actual implementation
- [ ] **PUB-06**: `docs/new-os-version-guide.md` for maintaining platform constants
- [ ] **PUB-07**: Core crate published to crates.io
- [ ] **PUB-08**: `native-theme-iced` published to crates.io

## Future Requirements

Deferred to post-v0.2. Tracked but not in current roadmap.

### Mobile Readers

- **MOBILE-01**: iOS reader `from_ios()` via objc2-ui-kit
- **MOBILE-02**: Android reader `from_android()` via JNI + NDK for Material You colors

### Change Notification

- **NOTIFY-01**: Linux portal `SettingChanged` D-Bus signal via ashpd stream
- **NOTIFY-02**: Linux KDE file watching via `notify` crate
- **NOTIFY-03**: macOS ObjC notification observers
- **NOTIFY-04**: Windows `UISettings.ColorValuesChanged` event

### Additional Connectors

- **CONN-10**: egui connector crate

## Out of Scope

| Feature | Reason |
|---------|--------|
| iOS/Android runtime readers | Small Rust GUI audience on mobile; ship preset TOML files for static theming |
| Change notification system | Complex, opinionated async runtime choice; users can poll or use toolkit observers |
| Color manipulation utilities (darken, lighten, contrast) | Out of scope for data crate; use the `palette` crate |
| egui connector in v0.2 | Least structured theming API; defer to v0.3 or community contribution |
| Widget-level animation/transitions | Animation is rendering concern; each toolkit has its own animation system |
| Exhaustive widget metrics (every KDE constant) | Diminishing returns past core measurements; model only what connectors consume |
| Named palette colors (platform-specific reds, blues) | Too platform-specific to standardize |
| Accessibility flags in the data model | Environment signals detected by consuming app |
| CSS/SCSS export format | Trivially implementable by consumers |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| API-01 | Phase 9 | Complete |
| API-02 | Phase 10 | Complete |
| API-03 | Phase 10 | Complete |
| API-04 | Phase 10 | Complete |
| API-05 | Phase 10 | Complete |
| API-06 | Phase 10 | Complete |
| API-07 | Phase 10 | Complete |
| API-08 | Phase 10 | Complete |
| PLAT-01 | Phase 11 | Complete |
| PLAT-02 | Phase 11 | Complete |
| PLAT-03 | Phase 11 | Complete |
| PLAT-04 | Phase 11 | Complete |
| PLAT-05 | Phase 11 | Complete |
| PLAT-06 | Phase 11 | Complete |
| PLAT-07 | Phase 11 | Complete |
| PLAT-08 | Phase 11 | Complete |
| PLAT-09 | Phase 11 | Complete |
| PLAT-10 | Phase 11 | Complete |
| PLAT-11 | Phase 11 | Complete |
| PLAT-12 | Phase 11 | Complete |
| PLAT-13 | Phase 11 | Complete |
| METRIC-01 | Phase 12 | Complete |
| METRIC-02 | Phase 12 | Complete |
| METRIC-03 | Phase 12 | Complete |
| METRIC-04 | Phase 12 | Pending |
| METRIC-05 | Phase 12 | Pending |
| METRIC-06 | Phase 12 | Pending |
| METRIC-07 | Phase 12 | Pending |
| METRIC-08 | Phase 12 | Complete |
| CI-01 | Phase 13 | Pending |
| CI-02 | Phase 13 | Pending |
| CI-03 | Phase 13 | Pending |
| CI-04 | Phase 13 | Pending |
| CONN-01 | Phase 14 | Pending |
| CONN-02 | Phase 14 | Pending |
| CONN-03 | Phase 14 | Pending |
| CONN-04 | Phase 14 | Pending |
| CONN-05 | Phase 14 | Pending |
| CONN-06 | Phase 14 | Pending |
| CONN-07 | Phase 14 | Pending |
| CONN-08 | Phase 14 | Pending |
| CONN-09 | Phase 14 | Pending |
| PUB-01 | Phase 15 | Pending |
| PUB-02 | Phase 15 | Pending |
| PUB-03 | Phase 15 | Pending |
| PUB-04 | Phase 15 | Pending |
| PUB-05 | Phase 15 | Pending |
| PUB-06 | Phase 15 | Pending |
| PUB-07 | Phase 15 | Pending |
| PUB-08 | Phase 15 | Pending |

**Coverage:**
- v0.2 requirements: 47 total
- Mapped to phases: 47/47 (100%)
- Unmapped: 0

---
*Requirements defined: 2026-03-08*
*Last updated: 2026-03-08 after roadmap creation*
