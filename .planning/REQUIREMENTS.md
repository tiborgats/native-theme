# Requirements: native-theme v0.3.3 Custom Icon Roles

**Defined:** 2026-03-15
**Core Value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.

## v0.3.3 Requirements

Requirements for v0.3.3 release. Each maps to roadmap phases.

### IconProvider Trait

- [x] **PROV-01**: Core crate defines an `IconProvider` trait with `icon_name(self, set: IconSet) -> Option<&'static str>` for name resolution
- [x] **PROV-02**: `IconProvider` trait includes `icon_svg(self, set: IconSet) -> Option<&'static [u8]>` for bundled SVG access
- [x] **PROV-03**: `IconProvider` trait includes `fallback_set() -> IconSet` as an associated function returning the default fallback icon set
- [x] **PROV-04**: Existing `IconRole` enum implements `IconProvider` trait, maintaining backward compatibility
- [x] **PROV-05**: `IconProvider` trait is object-safe, verified by test

### Custom Icon Loading

- [x] **LOAD-01**: Core crate exposes `load_custom_icon<P: IconProvider>(role: P, icon_set: &str) -> Option<IconData>` with the same fallback chain as `load_icon()`
- [x] **LOAD-02**: Core crate exposes `load_system_icon_by_name(name: &str, set: IconSet) -> Option<IconData>` for arbitrary name strings on all platforms
- [x] **LOAD-03**: macOS SF Symbols loader accepts arbitrary name strings (not just `IconRole` mappings)
- [x] **LOAD-04**: Windows Segoe Fluent loader accepts arbitrary name strings or hex codepoints
- [x] **LOAD-05**: `load_custom_icon` dispatches through platform loader -> bundled SVG -> fallback set -> None

### Build-Time Code Generation

- [x] **BUILD-01**: `native-theme-build` crate exists as a workspace member with `generate_icons(toml_path)` public API
- [ ] **BUILD-02**: Builder API exists: `IconGenerator::new().add(path).generate()` for composing multiple TOML files
- [x] **BUILD-03**: Master TOML schema supports role names, bundled-themes list, and system-themes list
- [x] **BUILD-04**: Per-theme mapping TOML maps every role to a theme-specific icon name
- [x] **BUILD-05**: Bundled theme mappings reference SVG files that are embedded via `include_bytes!` in generated code
- [x] **BUILD-06**: Generated enum derives `Debug, Clone, Copy, PartialEq, Eq, Hash`, is `#[non_exhaustive]`, and includes `const ALL: &[Self]`
- [x] **BUILD-07**: Generated `IconProvider` impl produces `icon_name()` and `icon_svg()` match arms for all roles and themes
- [x] **BUILD-08**: Generated `include_bytes!` uses `CARGO_MANIFEST_DIR` for correct path resolution from `OUT_DIR`
- [ ] **BUILD-09**: Build emits `cargo:rerun-if-changed` for every TOML file and SVG file
- [ ] **BUILD-10**: Build emits a size report summarizing role count, theme count, and total SVG bytes

### Build-Time Validation

- [ ] **VAL-01**: Missing role in any theme's mapping.toml produces a build error with role name and file path
- [ ] **VAL-02**: Missing SVG file for a bundled theme mapping produces a build error with file path
- [ ] **VAL-03**: Unknown role in a mapping (not declared in master TOML) produces a build error
- [ ] **VAL-04**: Missing `default` key in DE-aware inline table produces a build error
- [ ] **VAL-05**: Orphan SVG files (present in directory but unreferenced) produce a build warning
- [ ] **VAL-06**: Role name conflicts across multiple TOML files (builder API) produce a build error

### Freedesktop DE-Aware Mapping

- [ ] **FDES-01**: Per-theme freedesktop mapping supports DE-aware inline tables: `{ kde = "view-visible", default = "view-reveal" }`
- [ ] **FDES-02**: Generated code dispatches to correct icon name based on detected Linux desktop environment
- [ ] **FDES-03**: Mandatory `default` key covers all unrecognized DEs (XFCE, Cinnamon, MATE, LXQt, Budgie, and future DEs)

### Linux DE Coverage Audit

- [ ] **LNXDE-01**: Existing `detect_linux_de()` / `LinuxDesktop` enum covers KDE, GNOME, XFCE, Cinnamon, MATE, LXQt, Budgie
- [ ] **LNXDE-02**: Existing code has a `default` / `Unknown` / wildcard fallback for unrecognized desktop environments
- [ ] **LNXDE-03**: Hyprland/Sway (wlroots) and COSMIC detection is considered and handled (either explicitly or via default path)

### Connector Integration

- [ ] **CONN-01**: gpui connector provides generic `custom_icon_to_image_source<P: IconProvider>()` helper
- [ ] **CONN-02**: iced connector provides generic `custom_icon_to_image_handle<P: IconProvider>()` and `custom_icon_to_svg_handle<P: IconProvider>()` helpers
- [ ] **CONN-03**: Connector helpers follow the same pattern as existing `to_image_source()` / `to_image_handle()` for built-in `IconRole`

### Documentation

- [ ] **DOC-01**: `native-theme-build` has complete crate-level docs with usage example (build.rs + TOML + SVG layout)
- [ ] **DOC-02**: `IconProvider` trait has rustdoc with examples showing manual implementation and generated usage
- [ ] **DOC-03**: `load_custom_icon` and `load_system_icon_by_name` have rustdoc with usage examples
- [ ] **DOC-04**: All public types and functions added in v0.3.3 have standard Rust doc comments
- [ ] **DOC-05**: Core crate README updated to cover custom icon roles workflow
- [ ] **DOC-06**: No contradictions between inline docs, READMEs, and design document
- [ ] **DOC-07**: Connector crate READMEs updated with custom icon usage examples

### Release Preparation

- [ ] **REL-01**: Version bumped to 0.3.3 across all workspace crates
- [ ] **REL-02**: CHANGELOG.md updated with v0.3.3 changes
- [ ] **REL-03**: `pre-release-check.sh` passes cleanly
- [ ] **REL-04**: All tests pass (existing + new)
- [ ] **REL-05**: Clippy clean with no warnings
- [ ] **REL-06**: `native-theme-build` has correct crates.io metadata (description, license, repository, categories)

## Future Requirements

Deferred to future releases. Tracked but not in current roadmap.

### CLI Tool

- **CLI-01**: `native-theme-cli` / `cargo native-theme-icons` companion binary for downloading SVGs from icon repos
- **CLI-02**: CLI generates mapping.toml entries and downloads SVGs in one command
- **CLI-03**: CLI tracks license obligations per icon set

### Community Icon Packs

- **PACK-01**: Shareable TOML+SVG packages for common domains (media, development, IoT)
- **PACK-02**: Community packs work with the builder API's `.add()` method

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Runtime icon download/discovery | Hard constraint: fully offline at runtime |
| Proc macro for icon declaration | build.rs codegen is the proven pattern; proc macros can't reliably access filesystem for SVG validation |
| Dynamic plugin loading for providers | Breaks compile-time safety; apps can use generics |
| Merging custom + built-in into one enum | Types live in different crates; IconProvider trait provides the abstraction |
| Runtime TOML parsing for mappings | All TOML parsed at build time; zero runtime overhead |
| Feature-gated IconRole extensions | Doesn't scale; creates maintenance burden per the design doc |
| Stable enum discriminants | No FFI or serialization use case |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| PROV-01 | Phase 22 | Complete |
| PROV-02 | Phase 22 | Complete |
| PROV-03 | Phase 22 | Complete |
| PROV-04 | Phase 22 | Complete |
| PROV-05 | Phase 22 | Complete |
| LOAD-01 | Phase 22 | Complete |
| LOAD-02 | Phase 22 | Complete |
| LOAD-03 | Phase 22 | Complete |
| LOAD-04 | Phase 22 | Complete |
| LOAD-05 | Phase 22 | Complete |
| BUILD-01 | Phase 23 | Complete |
| BUILD-02 | Phase 23 | Pending |
| BUILD-03 | Phase 23 | Complete |
| BUILD-04 | Phase 23 | Complete |
| BUILD-05 | Phase 23 | Complete |
| BUILD-06 | Phase 23 | Complete |
| BUILD-07 | Phase 23 | Complete |
| BUILD-08 | Phase 23 | Complete |
| BUILD-09 | Phase 23 | Pending |
| BUILD-10 | Phase 23 | Pending |
| VAL-01 | Phase 23 | Pending |
| VAL-02 | Phase 23 | Pending |
| VAL-03 | Phase 23 | Pending |
| VAL-04 | Phase 23 | Pending |
| VAL-05 | Phase 23 | Pending |
| VAL-06 | Phase 23 | Pending |
| FDES-01 | Phase 24 | Pending |
| FDES-02 | Phase 24 | Pending |
| FDES-03 | Phase 24 | Pending |
| LNXDE-01 | Phase 24 | Pending |
| LNXDE-02 | Phase 24 | Pending |
| LNXDE-03 | Phase 24 | Pending |
| CONN-01 | Phase 25 | Pending |
| CONN-02 | Phase 25 | Pending |
| CONN-03 | Phase 25 | Pending |
| DOC-01 | Phase 26 | Pending |
| DOC-02 | Phase 26 | Pending |
| DOC-03 | Phase 26 | Pending |
| DOC-04 | Phase 26 | Pending |
| DOC-05 | Phase 26 | Pending |
| DOC-06 | Phase 26 | Pending |
| DOC-07 | Phase 26 | Pending |
| REL-01 | Phase 26 | Pending |
| REL-02 | Phase 26 | Pending |
| REL-03 | Phase 26 | Pending |
| REL-04 | Phase 26 | Pending |
| REL-05 | Phase 26 | Pending |
| REL-06 | Phase 26 | Pending |

**Coverage:**
- v0.3.3 requirements: 48 total
- Mapped to phases: 48
- Unmapped: 0

---
*Requirements defined: 2026-03-15*
*Last updated: 2026-03-15 after roadmap creation*
