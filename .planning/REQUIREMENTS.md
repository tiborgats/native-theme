# Requirements: native-theme v0.5.5

**Defined:** 2026-04-06
**Core Value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.

## v0.5.5 Requirements

Align data model, naming, and resolution engine with authoritative spec docs. Eliminate all invented values. Add missing interactive state colors. Fix all audit issues from `docs/todo_v0.5.5.md`.

### Data Model & Naming

- [ ] **SCHEMA-01**: FontSpec has `style` (FontStyle: Normal|Italic|Oblique) and `color` (Rgba) fields, with ResolvedFontSpec mirror
- [ ] **SCHEMA-02**: BorderSpec sub-struct (color, corner_radius, corner_radius_lg, line_width, opacity, shadow_enabled, padding_horizontal, padding_vertical) replaces flat border fields on all widgets, with ResolvedBorderSpec
- [ ] **SCHEMA-03**: LayoutTheme widget (widget_gap, container_margin, window_margin, section_gap) replaces ThemeSpacing
- [x] **SCHEMA-04**: All ~70 field renames applied per property-registry.toml (background→background_color, foreground→text_color, radius→border.corner_radius, etc.)
- [x] **SCHEMA-05**: ~70 interactive state color fields added across 18 widgets (hover_background, hover_text_color, active_background, disabled_background, etc.) per property-registry.toml
- [x] **SCHEMA-06**: Missing non-state fields added (disabled_opacity x7, fonts x11, icon_size x2, separator.line_width, header_font, body_font, text_selection_background/color, etc.)
- [x] **SCHEMA-07**: Extra code-only fields removed (per-widget foreground→font.color, ThemeSpacing removed)
- [ ] **SCHEMA-08**: DialogButtonOrder enum variants aligned with property-registry.toml

### Resolution Engine

- [x] **RESOLVE-01**: All safety-net invented values removed (hardcoded line_height 1.2, accent_foreground #ffffff, shadow rgba(0,0,0,64), etc.)
- [x] **RESOLVE-02**: Text scale computation removed from resolve.rs, windows.rs, kde/mod.rs, macos.rs
- [x] **RESOLVE-03**: scrollbar.thumb_hover computation replaced with explicit field inheritance
- [x] **RESOLVE-04**: resolve_border() and updated resolve_font() implement sub-struct inheritance per inheritance-rules.toml
- [x] **RESOLVE-05**: Inheritance bugs fixed (INH-1: input.selection wrong source, INH-2: dialog.background missing, INH-3: card border removed, SPEC-3: switch.unchecked_background)
- [x] **RESOLVE-06**: Missing inheritance rules added per inheritance-rules.toml (~40 new rules for state colors, fonts, borders)
- [x] **RESOLVE-07**: validate() updated for all new and renamed fields

### Preset Completeness

- [x] **PRESET-01**: All 17 presets rewritten for new naming conventions, BorderSpec sections, and LayoutTheme values
- [ ] **PRESET-02**: All 17 presets have explicit text_scale entries (13 currently missing get real platform values)
- [x] **PRESET-03**: All 17 presets have explicit interactive state color values

### Connector Cleanup

- [ ] **CONNECT-01**: Both connectors updated for new field names, BorderSpec, and font.color access patterns
- [ ] **CONNECT-02**: Connector derive.rs computations replaced with direct theme field reads
- [ ] **CONNECT-03**: Connector inconsistencies fixed (K-1 display name, K-2 from_system return type, K-3 iced contrast, K-4 dead code, K-5 clone)

### Correctness & Safety

- [ ] **CORRECT-01**: detect_is_dark() checks GTK_THEME env var and gtk-3.0/settings.ini for non-GNOME/non-KDE Linux (C-1)
- [ ] **CORRECT-02**: iOS platform detection added to detect_platform() (C-2)
- [ ] **CORRECT-03**: into_resolved() #[must_use] message fixed (C-3)
- [ ] **CORRECT-04**: Spinner/animation safety guards added (S-1 width/height guard, S-3 empty frames, S-4 zero duration, S-5 single-quote viewBox)
- [ ] **CORRECT-05**: gsettings commands get timeout (R-1)

### CI/Publishing

- [ ] **CI-01**: Publish workflow tests gpui connector before publishing (P-1)
- [ ] **CI-02**: Publish steps error handling improved (P-2)
- [ ] **CI-03**: CI tests async-io runtime variants (P-3)
- [ ] **CI-04**: Example names disambiguated (showcase-gpui, showcase-iced) (P-4)
- [ ] **CI-05**: pre-release.sh gets max iteration timeout (P-5)

### Testing

- [ ] **TEST-01**: Property-based tests for TOML round-trips and merge semantics
- [ ] **TEST-02**: Programmatic cross-reference of platform-facts.md against preset values

### Verification

- [ ] **VERIFY-01**: `pre-release-check.sh` passes after each major phase (schema, resolve, presets, connectors)
- [ ] **VERIFY-02**: Final line-by-line audit of `docs/todo_v0.5.5.md` — every item confirmed implemented or improved
- [ ] **VERIFY-03**: No contradictions between `property-registry.toml`, `inheritance-rules.toml`, `platform-facts.md`, and actual code

### Documentation Sync

- [ ] **DOC-01**: Widget struct doc comment inaccuracies fixed (W-2: Checkbox, Switch, Dialog)
- [ ] **DOC-02**: Hardcoded connector opacity values documented (K-6)
- [ ] **DOC-03**: `property-registry.toml`, `inheritance-rules.toml`, `platform-facts.md` updated to match final implementation
- [ ] **DOC-04**: All READMEs updated (core, gpui connector, iced connector, build crate) for new API surface
- [ ] **DOC-05**: CHANGELOG updated with all breaking changes and migration notes

## Future Requirements

Deferred beyond v0.5.5:

- OnceLock cache invalidation mechanism (D-1) — post-1.0
- Error type Clone derive (D-3) — low impact
- TOML fuzzing (T-2) — nice-to-have
- Code coverage reporting (T-3) — infrastructure
- Platform reader CI testing (T-4) — infeasible without desktop environments on CI
- Icon disk I/O caching (R-2) — caller responsibility
- API stability policy (DOC-2) — post-1.0
- MSRV policy review (DOC-1) — not a code change

## Out of Scope

| Feature | Reason |
|---------|--------|
| OnceLock cache invalidation | Post-1.0; documented in todo.md |
| `is_empty()` for optional_nested (D-2) | Low severity; unlikely to surface in practice |
| SVG prefix slice guard (S-2) | Very low severity; private function with controlled inputs |
| Identical muted color audit (PRE-1) | Likely intentional system colors |
| Negative focus_ring_offset (PRE-2) | Already documented as intentional inset styling |
| Unmaintained transitive deps (DEP-1) | Not actionable; monitor for gpui updates |
| Lock file package count (DEP-2) | Not actionable; driven by gpui/iced ecosystems |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SCHEMA-01 | Phase 49 | Pending |
| SCHEMA-02 | Phase 49 | Pending |
| SCHEMA-03 | Phase 49 | Pending |
| SCHEMA-04 | Phase 50 | Complete |
| SCHEMA-05 | Phase 52 | Complete |
| SCHEMA-06 | Phase 50 | Complete |
| SCHEMA-07 | Phase 50 | Complete |
| SCHEMA-08 | Phase 50 | Pending |
| RESOLVE-01 | Phase 51 | Complete |
| RESOLVE-02 | Phase 51 | Complete |
| RESOLVE-03 | Phase 51 | Complete |
| RESOLVE-04 | Phase 51 | Complete |
| RESOLVE-05 | Phase 51 | Complete |
| RESOLVE-06 | Phase 51 | Complete |
| RESOLVE-07 | Phase 51 | Complete |
| PRESET-01 | Phase 50 | Complete |
| PRESET-02 | Phase 53 | Pending |
| PRESET-03 | Phase 53 | Complete |
| CONNECT-01 | Phase 54 | Pending |
| CONNECT-02 | Phase 54 | Pending |
| CONNECT-03 | Phase 54 | Pending |
| CORRECT-01 | Phase 55 | Pending |
| CORRECT-02 | Phase 55 | Pending |
| CORRECT-03 | Phase 55 | Pending |
| CORRECT-04 | Phase 55 | Pending |
| CORRECT-05 | Phase 55 | Pending |
| CI-01 | Phase 55 | Pending |
| CI-02 | Phase 55 | Pending |
| CI-03 | Phase 55 | Pending |
| CI-04 | Phase 55 | Pending |
| CI-05 | Phase 55 | Pending |
| TEST-01 | Phase 56 | Pending |
| TEST-02 | Phase 56 | Pending |
| VERIFY-01 | Phase 57 | Pending |
| VERIFY-02 | Phase 57 | Pending |
| VERIFY-03 | Phase 57 | Pending |
| DOC-01 | Phase 57 | Pending |
| DOC-02 | Phase 57 | Pending |
| DOC-03 | Phase 57 | Pending |
| DOC-04 | Phase 57 | Pending |
| DOC-05 | Phase 57 | Pending |

**Coverage:**
- v0.5.5 requirements: 41 total
- Mapped to phases: 41
- Unmapped: 0

---
*Requirements defined: 2026-04-06*
*Last updated: 2026-04-06 after roadmap creation*
