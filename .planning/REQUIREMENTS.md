# Requirements: native-theme v0.5.6

**Defined:** 2026-04-09
**Core Value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.

## v0.5.6 Requirements

Improve internal maintainability (module split, validate codegen, reader testing) and ship the first new public API: runtime theme change watching.

### Internal Structure

- [x] **STRUCT-01**: lib.rs broken into detect.rs, system_theme.rs, icon_loader.rs, macros.rs with lib.rs as pure root (~200 lines)
- [ ] **STRUCT-02**: define_widget_pair! generates validate extraction methods for all 25 widget pairs via ValidateNested trait dispatch
- [ ] **STRUCT-03**: ThemeDefaults extraction stays hand-written (special DPI, text_scale, icon_sizes handling)
- [x] **STRUCT-04**: validate.rs reduced to <500 lines (range checks + construction + defaults extraction)

### Platform Reader Testing

- [x] **TEST-01**: KDE reader separated into parse_kdeglobals(content: &str) pure function testable with fixture data
- [x] **TEST-02**: KDE fixture tests cover: Breeze light/dark, custom accent, minimal config, missing groups, malformed values, high DPI
- [ ] **TEST-03**: GNOME reader separated into build_gnome_spec(PortalData) with primitive types (no ashpd dependency in tests)
- [ ] **TEST-04**: Windows reader separated into build_windows_spec(WindowsData) testable on any platform
- [ ] **TEST-05**: macOS reader separated into build_macos_spec(MacOSData) testable on any platform

### Runtime Theme Watching

- [ ] **WATCH-01**: on_theme_change() callback API with ThemeWatcher RAII handle — signal-only (ThemeChangeEvent), not theme-rebuilding
- [ ] **WATCH-02**: Linux GNOME watcher via ashpd portal SettingChanged stream on background thread (no async runtime exposed to consumer)
- [ ] **WATCH-03**: Linux KDE watcher via inotify (notify crate) on kdeglobals with 300ms debounce
- [ ] **WATCH-04**: macOS watcher via NSDistributedNotificationCenter for AppleInterfaceThemeChangedNotification with CFRunLoop on watcher thread
- [ ] **WATCH-05**: Windows watcher via UISettings::ColorValuesChanged with COM STA initialization and message pump on watcher thread
- [ ] **WATCH-06**: `watch` feature flag gates on_theme_change() API and the `notify` dependency

## Future Requirements

### Connector Updates

- **CONN-01**: gpui connector helper for ThemeWatcher integration (subscribe + re-read pattern)
- **CONN-02**: iced connector Subscription adapter for ThemeChangeEvent

### Extended Watching

- **EXTWATCH-01**: Watch accent color changes (GNOME portal accent-color, Windows accent)
- **EXTWATCH-02**: Watch font changes
- **EXTWATCH-03**: Watch icon theme changes
- **EXTWATCH-04**: Watch high contrast toggle

### egui Connector

- **EGUI-01**: native-theme-egui connector crate

## Out of Scope

| Feature | Reason |
|---------|--------|
| Async Stream API for watching | Callback API is universal; async consumers wrap trivially with mpsc channel |
| Polling-based watching | Wasteful; all target platforms have native signals |
| Theme rebuilding inside watcher | Too complex (overlay-aware pipeline); callers re-run from_system() on signal |
| Proc-macro for validation | define_widget_pair! extension + ValidateNested trait achieves the same without new crate |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| STRUCT-01 | Phase 61 | Complete |
| STRUCT-02 | Phase 62 | Pending |
| STRUCT-03 | Phase 62 | Pending |
| STRUCT-04 | Phase 62 | Complete |
| TEST-01 | Phase 63 | Complete |
| TEST-02 | Phase 63 | Complete |
| TEST-03 | Phase 64 | Pending |
| TEST-04 | Phase 64 | Pending |
| TEST-05 | Phase 64 | Pending |
| WATCH-01 | Phase 65 | Pending |
| WATCH-02 | Phase 66 | Pending |
| WATCH-03 | Phase 66 | Pending |
| WATCH-04 | Phase 67 | Pending |
| WATCH-05 | Phase 67 | Pending |
| WATCH-06 | Phase 65 | Pending |

**Coverage:**
- v0.5.6 requirements: 15 total
- Mapped to phases: 15
- Unmapped: 0

---
*Requirements defined: 2026-04-09*
*Last updated: 2026-04-09 after roadmap creation*
