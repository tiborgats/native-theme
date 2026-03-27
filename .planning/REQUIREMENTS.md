# Requirements: native-theme

**Defined:** 2026-03-27
**Core Value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.

## v0.5.0 Requirements

Requirements for per-widget architecture and resolution pipeline. Each maps to roadmap phases.

### Data Model

- [x] **MODEL-01**: ThemeVariant has per-widget structs (24 widgets) with colors, font, sizing, and geometry fields per widget
- [x] **MODEL-02**: ThemeDefaults struct provides shared base properties (colors, font, mono_font, spacing, icon_sizes, accessibility)
- [x] **MODEL-03**: FontSpec struct with family, size, weight fields for per-widget font specification
- [x] **MODEL-04**: TextScale with 4 entries (caption, section_heading, dialog_title, display) using TextScaleEntry (size, weight, line_height)
- [x] **MODEL-05**: DialogButtonOrder enum (TrailingAffirmative / LeadingAffirmative) on DialogTheme
- [x] **MODEL-06**: IconSizes struct with toolbar, small, large, dialog, panel fields on ThemeDefaults
- [x] **MODEL-07**: WindowTheme with title bar colors, inactive states, title bar font, radius, shadow
- [x] **MODEL-08**: define_widget_pair! macro generates paired Option and Resolved structs from single definition
- [x] **MODEL-09**: impl_merge! supports nested per-widget struct merge on ThemeVariant

### Resolution

- [x] **RESOLVE-01**: resolve() fills ~90 inheritance rules (accent->primary_bg, font->menu.font, radius->button.radius, etc.)
- [x] **RESOLVE-02**: ResolvedTheme with non-optional fields mirrors ThemeVariant per-widget structure
- [x] **RESOLVE-03**: validate() converts ThemeVariant->ResolvedTheme, returns ThemeResolutionError listing all missing fields
- [x] **RESOLVE-04**: FontSpec sub-field inheritance (None family/size/weight individually inherit from defaults.font)
- [x] **RESOLVE-05**: TextScaleEntry inheritance (size<-font.size, weight<-font.weight, line_height<-line_height multiplier x resolved size)
- [x] **RESOLVE-06**: Accent-derived propagation (accent->primary_bg, checked_bg, slider.fill, progress_bar.fill, switch.checked_bg)

### macOS Reader

- [ ] **MACOS-01**: NSFont.TextStyle entries populate text_scale (caption, section_heading, dialog_title, display)
- [ ] **MACOS-02**: Per-widget fonts from +menuFontOfSize:, +toolTipsFontOfSize:, +titleBarFontOfSize: with weight extraction
- [ ] **MACOS-03**: Additional NSColor values (placeholder, caret, selection_inactive, alternate_row, header_foreground, grid_color)
- [ ] **MACOS-04**: NSScroller.preferredScrollerStyle -> scrollbar.overlay_mode
- [ ] **MACOS-05**: Accessibility queries (reduce_motion, high_contrast, reduce_transparency, text_scaling_factor)

### Windows Reader

- [ ] **WIN-01**: NONCLIENTMETRICSW fonts (lfCaptionFont, lfMenuFont, lfStatusFont) -> per-widget FontSpec
- [ ] **WIN-02**: DwmGetColorizationColor -> window.title_bar_background; COLOR_CAPTION/INACTIVECAPTION colors
- [ ] **WIN-03**: GetSysColor widget colors (BTNFACE, BTNTEXT, MENU, MENUTEXT, INFOBK, INFOTEXT, WINDOW, WINDOWTEXT, HIGHLIGHT, HIGHLIGHTTEXT)
- [ ] **WIN-04**: UISettings.TextScaleFactor, SPI_GETHIGHCONTRAST, SPI_GETCLIENTAREAANIMATION -> accessibility fields
- [ ] **WIN-05**: SM_CXSMICON, SM_CXICON -> defaults.icon_sizes.small, defaults.icon_sizes.large

### KDE Reader

- [ ] **KDE-01**: [WM] section -> title bar colors (active/inactive bg/fg) + activeFont -> title bar font
- [ ] **KDE-02**: [Colors:Header], [Colors:Complementary], [Colors:View] extras -> list header, sidebar, placeholder, alternate_row, visited
- [ ] **KDE-03**: Per-widget fonts (menuFont, toolBarFont) with Qt5/Qt6 weight scale detection
- [ ] **KDE-04**: Text scale computation from smallestReadableFont + font.size x Kirigami multipliers
- [ ] **KDE-05**: Icon sizes from icon theme index.theme; [Icons] Theme -> icon_set
- [ ] **KDE-06**: AnimationDurationFactor -> reduce_motion; forceFontDPI -> text_scaling_factor

### GNOME Reader

- [ ] **GNOME-01**: font-name, monospace-font-name, titlebar-font gsettings -> FontSpec fields
- [ ] **GNOME-02**: Text scale computation from font.size x CSS percentages (caption 82%, title-2 136%, title-1 181%)
- [ ] **GNOME-03**: text-scaling-factor, enable-animations, overlay-scrolling gsettings -> accessibility + scrollbar fields
- [ ] **GNOME-04**: icon-theme gsetting -> icon_set; portal accent already handled
- [ ] **GNOME-05**: Portal reduced_motion + contrast with gsettings fallback

### Pipeline

- [ ] **PIPE-01**: from_system() runs full pipeline: OS reader -> platform TOML overlay -> resolve() -> ResolvedTheme
- [ ] **PIPE-02**: Platform-to-preset mapping (macOS->macos-sonoma, Windows->windows-11, KDE->kde-breeze, GNOME->adwaita)
- [ ] **PIPE-03**: App TOML overlay support with second resolve() pass propagating changed source fields

### Presets

- [x] **PRESET-01**: All 17 preset TOMLs rewritten for new per-widget structure with serde round-trip tests
- [x] **PRESET-02**: Platform preset TOMLs slimmed (OS-readable values removed, only design constants remain)
- [x] **PRESET-03**: Cross-platform presets (catppuccin, nord, etc.) provide all non-derived fields for new structure

### Connectors

- [ ] **CONN-01**: gpui connector accepts &ResolvedTheme, removes all Option handling
- [ ] **CONN-02**: iced connector accepts &ResolvedTheme, removes all Option handling
- [ ] **CONN-03**: Showcase examples updated for new API (both gpui and iced)

## Future Requirements

Deferred to post-v0.5.0. Tracked but not in current roadmap.

### Iced Geometry

- **ICED-01**: Custom style functions for all iced widgets using theme geometry (radius, frame_width, shadow)
- **ICED-02**: Widget metrics helpers for iced (checkbox_size, slider_track_height, progress_bar_height, etc.)

### Additional Platforms

- **PLAT-01**: iOS reader (from_ios())
- **PLAT-02**: Android reader (from_android())

### Runtime

- **RT-01**: Change notification system for live theme updates

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Per-widget style class system (.primary, .flat variants) | Multiplicative struct explosion; use button.primary_bg instead |
| CSS cascade with selectors | Runtime evaluation engine, not a data structure library |
| Runtime theme change watchers | Couples to async runtimes; document re-calling from_system() |
| Computed color algebra (darken/lighten/mix) | Rendering concern, not theme data |
| Per-widget interaction state colors (hover/pressed/focus) | Multiplies fields 4-5x; use disabled_opacity + toolkit color ops |
| Animation/transition tokens (duration, easing) | Not standardized across platforms; reduce_motion bool is sufficient |
| Font fallback chains | Text shaping concern handled by layout engine |
| crates.io publishing | Publish on GitHub only for this milestone |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| MODEL-01 | Phase 44 | Complete |
| MODEL-02 | Phase 44 | Complete |
| MODEL-03 | Phase 44 | Complete |
| MODEL-04 | Phase 44 | Complete |
| MODEL-05 | Phase 44 | Complete |
| MODEL-06 | Phase 44 | Complete |
| MODEL-07 | Phase 44 | Complete |
| MODEL-08 | Phase 44 | Complete |
| MODEL-09 | Phase 44 | Complete |
| RESOLVE-01 | Phase 45 | Complete |
| RESOLVE-02 | Phase 45 | Complete |
| RESOLVE-03 | Phase 45 | Complete |
| RESOLVE-04 | Phase 45 | Complete |
| RESOLVE-05 | Phase 45 | Complete |
| RESOLVE-06 | Phase 45 | Complete |
| MACOS-01 | Phase 46 | Pending |
| MACOS-02 | Phase 46 | Pending |
| MACOS-03 | Phase 46 | Pending |
| MACOS-04 | Phase 46 | Pending |
| MACOS-05 | Phase 46 | Pending |
| WIN-01 | Phase 46 | Pending |
| WIN-02 | Phase 46 | Pending |
| WIN-03 | Phase 46 | Pending |
| WIN-04 | Phase 46 | Pending |
| WIN-05 | Phase 46 | Pending |
| KDE-01 | Phase 46 | Pending |
| KDE-02 | Phase 46 | Pending |
| KDE-03 | Phase 46 | Pending |
| KDE-04 | Phase 46 | Pending |
| KDE-05 | Phase 46 | Pending |
| KDE-06 | Phase 46 | Pending |
| GNOME-01 | Phase 46 | Pending |
| GNOME-02 | Phase 46 | Pending |
| GNOME-03 | Phase 46 | Pending |
| GNOME-04 | Phase 46 | Pending |
| GNOME-05 | Phase 46 | Pending |
| PIPE-01 | Phase 47 | Pending |
| PIPE-02 | Phase 47 | Pending |
| PIPE-03 | Phase 47 | Pending |
| PRESET-01 | Phase 44 | Complete |
| PRESET-02 | Phase 44 | Complete |
| PRESET-03 | Phase 45 | Complete |
| CONN-01 | Phase 48 | Pending |
| CONN-02 | Phase 48 | Pending |
| CONN-03 | Phase 48 | Pending |

**Coverage:**
- v0.5.0 requirements: 45 total
- Mapped to phases: 45
- Unmapped: 0

---
*Requirements defined: 2026-03-27*
*Last updated: 2026-03-27 after roadmap creation*
