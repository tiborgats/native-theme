# Requirements: native-theme v0.4.0 Animated Icons

**Defined:** 2026-03-18
**Core Value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.

## v0.4 Requirements

Requirements for animated loading indicator support. Each maps to roadmap phases.

### Core Animation API

- [x] **ANIM-01**: AnimatedIcon enum with Frames and Transform variants, both #[non_exhaustive]
- [x] **ANIM-02**: Frames variant holds Vec<IconData> frames, frame_duration_ms (u32), and Repeat enum
- [x] **ANIM-03**: Transform variant holds IconData and TransformAnimation enum
- [x] **ANIM-04**: TransformAnimation::Spin variant with duration_ms field for continuous rotation
- [x] **ANIM-05**: loading_indicator(icon_set: &str) dispatches to platform/bundled loaders and returns Option<AnimatedIcon>
- [x] **ANIM-06**: AnimatedIcon provides first_frame() -> Option<&IconData> helper for static fallback

### Bundled Spinner Frames

- [x] **SPIN-01**: Material spinner as ~12 SVG frames of circular stroke arc animation (24x24 viewBox), gated on material-icons feature
- [x] **SPIN-02**: Lucide spinner as Transform::Spin on the loader icon SVG, gated on lucide-icons feature
- [x] **SPIN-03**: macOS-style spinner as 12 SVG frames with radial spokes at sequential opacity steps, gated on system-icons feature
- [x] **SPIN-04**: Windows-style spinner as ~60 SVG frames of arc expansion/contraction over 2-second cycle with easing baked into geometry, gated on system-icons feature
- [x] **SPIN-05**: GNOME Adwaita-style spinner as ~20 SVG frames of overlapping arcs with sinusoidal breathing, gated on system-icons feature
- [x] **SPIN-06**: All bundled SVG frames use 24x24 viewBox coordinate space and are embedded via include_bytes!()
- [x] **SPIN-07**: Each bundled frame set validates through resvg rasterization in tests

### Freedesktop Integration

- [x] **FD-01**: Parse freedesktop vertical SVG sprite sheets (process-working.svg) into individual SVG frames via viewBox string rewriting
- [x] **FD-02**: Detect single-frame process-working-symbolic icons and return Transform::Spin instead of Frames
- [x] **FD-03**: loading_indicator("freedesktop") returns theme-native animation data at runtime, gated on system-icons feature
- [x] **FD-04**: Graceful fallback to bundled frames when no freedesktop sprite sheet is found in the active theme

### Accessibility

- [ ] **A11Y-01**: prefers_reduced_motion() -> bool queries OS accessibility setting, sync with OnceLock caching
- [ ] **A11Y-02**: Linux: query via gsettings org.gnome.desktop.interface enable-animations (sync subprocess fallback)
- [ ] **A11Y-03**: macOS: query NSWorkspace.accessibilityDisplayShouldReduceMotion
- [ ] **A11Y-04**: Windows: query UISettings.AnimationsEnabled()
- [ ] **A11Y-05**: Returns false (allow animations) on unsupported platforms or query failure

### Breaking Changes

- [x] **BREAK-01**: Remove StatusLoading variant from IconRole enum
- [x] **BREAK-02**: Update all internal references, match arms, icon_name() mappings, and bundled icon lookups

### Connector Integration

- [ ] **CONN-01**: gpui connector: AnimatedIcon::Frames → frame cycling with timer-driven index selection
- [ ] **CONN-02**: gpui connector: AnimatedIcon::Transform::Spin → AnimationExt with Transformation::rotate()
- [ ] **CONN-03**: iced connector: AnimatedIcon::Frames → frame cycling with time::every() subscription
- [ ] **CONN-04**: iced connector: AnimatedIcon::Transform::Spin → Svg::rotation() with timer

### Documentation

- [ ] **DOC-01**: API documentation for all new public types and functions
- [ ] **DOC-02**: CHANGELOG entry documenting new features and StatusLoading removal migration
- [ ] **DOC-03**: Migration guide: StatusLoading → loading_indicator() with code examples

## v0.5+ Requirements

Deferred to future releases.

### Extended Animation

- **EANIM-01**: Variable per-frame timing (non-uniform frame durations)
- **EANIM-02**: TransformAnimation::SpinEased with configurable easing curves
- **EANIM-03**: Additional animated icon roles beyond loading (if platforms define them)
- **EANIM-04**: Lottie variant on AnimatedIcon for rich animation data

### Connector Widgets

- **CWID-01**: gpui native_spinner() widget element with toolkit-native drawing
- **CWID-02**: iced native_spinner() widget with Canvas-based rendering

## Out of Scope

| Feature | Reason |
|---------|--------|
| Animated icon roles beyond loading | No platform defines native pulse/bounce/etc. — application-level UX choices |
| Lottie runtime (rlottie/dotlottie-rs) | C++ FFI dependency, overkill for spinner |
| Runtime OS widget capture | Fragile, requires window server, untestable in CI |
| Connector-level native widgets | Per-connector rendering duplication, deferred to v0.5+ |
| APNG/WebP bundled animations | Raster-only, fixed resolution, adds decode dependency |
| Procedural SpinnerParams for connectors | Duplicates rendering logic in every connector |
| Variable frame timing | No current use case; uniform timing sufficient for all platforms |
| Async prefers_reduced_motion | Sync is sufficient; D-Bus portal not needed when gsettings works |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| ANIM-01 | Phase 27 | Complete |
| ANIM-02 | Phase 27 | Complete |
| ANIM-03 | Phase 27 | Complete |
| ANIM-04 | Phase 27 | Complete |
| ANIM-05 | Phase 27 | Complete |
| ANIM-06 | Phase 27 | Complete |
| SPIN-01 | Phase 28 | Complete |
| SPIN-02 | Phase 28 | Complete |
| SPIN-03 | Phase 28 | Complete |
| SPIN-04 | Phase 28 | Complete |
| SPIN-05 | Phase 28 | Complete |
| SPIN-06 | Phase 28 | Complete |
| SPIN-07 | Phase 28 | Complete |
| FD-01 | Phase 29 | Complete |
| FD-02 | Phase 29 | Complete |
| FD-03 | Phase 29 | Complete |
| FD-04 | Phase 29 | Complete |
| A11Y-01 | Phase 30 | Pending |
| A11Y-02 | Phase 30 | Pending |
| A11Y-03 | Phase 30 | Pending |
| A11Y-04 | Phase 30 | Pending |
| A11Y-05 | Phase 30 | Pending |
| BREAK-01 | Phase 27 | Complete |
| BREAK-02 | Phase 27 | Complete |
| CONN-01 | Phase 31 | Pending |
| CONN-02 | Phase 31 | Pending |
| CONN-03 | Phase 31 | Pending |
| CONN-04 | Phase 31 | Pending |
| DOC-01 | Phase 32 | Pending |
| DOC-02 | Phase 32 | Pending |
| DOC-03 | Phase 32 | Pending |

**Coverage:**
- v0.4 requirements: 31 total
- Mapped to phases: 31
- Unmapped: 0

---
*Requirements defined: 2026-03-18*
*Last updated: 2026-03-18 after roadmap creation*
