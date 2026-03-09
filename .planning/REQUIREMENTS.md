# Requirements: native-theme

**Defined:** 2026-03-08
**Core Value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.

## v0.3 Requirements

Requirements for icon loading milestone. Each maps to roadmap phases.

### Icon Data Model

- [x] **ICON-01**: IconRole enum with 42 semantic icon roles across 7 categories (dialog, window, action, navigation, file, status, system)
- [x] **ICON-02**: IconData enum returning SVG bytes (`Svg(Vec<u8>)`) or rasterized RGBA pixels (`Rgba { width, height, data }`)
- [ ] **ICON-03**: icon_theme field (`Option<String>`) on ThemeVariant with preset-specific default assignments in TOML
- [ ] **ICON-04**: `icon_name()` function to look up the platform-specific identifier string for a given icon set and role
- [ ] **ICON-05**: `system_icon_set()` function to resolve "system" to the OS-native icon set name (macOS->sf-symbols, Windows->segoe-fluent, Linux->freedesktop)

### Platform Loading

- [ ] **PLAT-01**: macOS icon loading via `NSImage(systemSymbolName:)` -> rasterized RGBA pixels (feature "system-icons")
- [ ] **PLAT-02**: Windows stock icon loading via `SHGetStockIconInfo` -> RGBA pixels (feature "system-icons")
- [ ] **PLAT-03**: Windows Segoe Fluent Icons font glyph rendering for action/navigation/window roles (feature "system-icons")
- [ ] **PLAT-04**: Linux freedesktop icon theme lookup following Icon Theme Specification -> SVG file bytes (feature "system-icons")

### Bundled Fallback Icons

- [ ] **BNDL-01**: Material Symbols SVGs (~42 icons covering all IconRole variants) as compile-time fallback (feature "material-icons")
- [ ] **BNDL-02**: Lucide SVGs (~42 icons) as optional alternative icon set (feature "lucide-icons")

### Integration

- [ ] **INTG-01**: `load_icon()` dispatch function selecting the appropriate loader based on icon_theme string
- [ ] **INTG-02**: Optional SVG-to-RGBA rasterization via resvg (feature "svg-rasterize")
- [ ] **INTG-03**: gpui connector: IconData->RenderImage conversion + `icon_name()` Lucide shortcut for gpui-component IconName
- [ ] **INTG-04**: iced connector: IconData conversion helpers
- [ ] **INTG-05**: gpui example updated with icon display and icon set selector dropdown

## v0.2 Requirements (Complete)

All v0.2 requirements completed. See MILESTONES.md for details.

### API Refactors & Structure

- [x] **API-01** through **API-08**: Cargo workspace, flat ThemeColors, NativeTheme methods, ThemeGeometry extensions

### Platform Readers

- [x] **PLAT-01** through **PLAT-13**: macOS reader, Windows enhancements, Linux KDE+portal overlay

### Widget Metrics

- [x] **METRIC-01** through **METRIC-08**: WidgetMetrics struct, platform sources, preset updates

### CI Pipeline

- [x] **CI-01** through **CI-04**: GitHub Actions, feature matrix, semver-checks, clippy/fmt

### Toolkit Connectors

- [x] **CONN-01** through **CONN-09**: gpui + iced connectors with examples

### Publishing Prep

- [x] **PUB-01** through **PUB-06**: Metadata, licenses, changelog, documentation
- [ ] **PUB-07**: Core crate published to crates.io (deferred)
- [ ] **PUB-08**: native-theme-iced published to crates.io (deferred)

## Future Requirements

### Extended Icon Sets

- **XICON-01**: Phosphor Icons as additional bundled set (MIT, 9,000+ icons)
- **XICON-02**: Tabler Icons as additional bundled set (MIT, 5,900+ icons)

### Platform Extensions

- **XPLAT-01**: iOS icon loading via UIImage(systemName:)
- **XPLAT-02**: Android icon loading

### Change Notification

- **NOTIFY-01** through **NOTIFY-04**: Platform-specific change notification streams

### Additional Connectors

- **CONN-10**: egui connector crate

## Out of Scope

| Feature | Reason |
|---------|--------|
| Full icon set bundling (3,800+ Material, 1,700+ Lucide) | Binary size — only ~42 IconRole-mapped icons bundled |
| Icon animation (spinners, loading) | Static icons only — animation is toolkit-specific |
| Icon tinting/coloring in core crate | Connector responsibility — core returns raw icon data |
| Custom user icon sets | Users can implement their own load_icon() equivalent |
| Icon caching | Consumers cache at toolkit level; core crate is stateless |
| iOS/Android runtime readers | Small Rust GUI audience on mobile; deferred |
| Change notification system | Complex async runtime choice; users can poll |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| ICON-01 | Phase 16 | Complete |
| ICON-02 | Phase 16 | Complete |
| ICON-03 | Phase 16 | Pending |
| ICON-04 | Phase 16 | Pending |
| ICON-05 | Phase 16 | Pending |
| PLAT-01 | Phase 19 | Pending |
| PLAT-02 | Phase 20 | Pending |
| PLAT-03 | Phase 20 | Pending |
| PLAT-04 | Phase 18 | Pending |
| BNDL-01 | Phase 17 | Pending |
| BNDL-02 | Phase 17 | Pending |
| INTG-01 | Phase 21 | Pending |
| INTG-02 | Phase 21 | Pending |
| INTG-03 | Phase 21 | Pending |
| INTG-04 | Phase 21 | Pending |
| INTG-05 | Phase 21 | Pending |

**Coverage:**
- v0.3 requirements: 16 total
- Mapped to phases: 16
- Unmapped: 0

---
*Requirements defined: 2026-03-08*
*Last updated: 2026-03-09 after v0.3 roadmap creation*
