# Phase 26: Documentation and Release - Research

**Researched:** 2026-03-17
**Domain:** Rust crate documentation (rustdoc), release preparation, crates.io publishing
**Confidence:** HIGH

## Summary

Phase 26 covers the final documentation pass and release preparation for v0.3.3 of the native-theme workspace. The codebase is in good shape: cargo doc produces zero warnings across all 4 crates, clippy is clean on native-theme/native-theme-build/native-theme-iced (gpui has an upstream naga compile issue unrelated to our code), all tests pass, and version is already bumped to 0.3.3 across the workspace.

The main work falls into three categories: (1) adding crate-level docs and usage examples to native-theme-build (which currently has no `//!` docs at all), (2) updating READMEs and CHANGELOG to cover the custom icon roles workflow added in phases 22-25, and (3) fixing contradictions between the design document (`docs/custom-icon-roles.md`) and the actual implementation (the design doc describes an API with associated types and fallback_set that was never implemented -- the actual trait is simpler and object-safe).

**Primary recommendation:** Work through DOC requirements first (DOC-01 through DOC-07), then REL requirements. The codebase is already clean enough that REL-03 through REL-06 may pass with minimal changes after the doc work is done.

## Standard Stack

### Core

This phase uses no new libraries. All work is documentation, TOML metadata, and shell scripting.

| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| rustdoc | stable 1.94+ | Documentation generation | Built into toolchain |
| cargo-semver-checks | latest | Semver compatibility checking | NOT installed; pre-release-check.sh does not use it |
| cargo-audit | latest | Security vulnerability scanning | Used by pre-release-check.sh |
| cargo-outdated | latest | Dependency staleness checking | Used by pre-release-check.sh |

### Supporting

| Tool | Purpose | When to Use |
|------|---------|-------------|
| `cargo publish --dry-run` | Validate crates.io packaging | REL-06 metadata check |
| `cargo doc --workspace --no-deps` | Zero-warning doc check | DOC-04 verification |

## Architecture Patterns

### Current Crate Structure

```
native-theme/                   # Workspace root
  Cargo.toml                    # workspace.package with version = "0.3.3"
  README.md                     # Root README (crates: table, Quick Start, Toolkit Connectors)
  CHANGELOG.md                  # Keep a Changelog format, versions 0.1.0-0.3.2
  pre-release-check.sh          # Per-crate check/fmt/clippy/test/doc/publish-dry-run
  native-theme/                 # Core crate (has README.md, has crate-level docs)
    src/lib.rs                  # Has //! docs + #[doc = include_str!("../README.md")] doctest
  native-theme-build/           # Build crate (NO README.md, NO crate-level docs)
    src/lib.rs                  # Missing //! docs entirely
    Cargo.toml                  # Missing: repository, homepage, keywords, categories, readme
  connectors/
    native-theme-gpui/          # Has README.md, has crate-level docs
    native-theme-iced/          # Has README.md, has crate-level docs
```

### Rustdoc Pattern for This Project

The project uses a consistent pattern across crates that have docs:
1. Crate-level `//!` comment block with Overview and Example sections
2. `#[doc = include_str!("../README.md")]` for README doctests (core crate)
3. All public items have `///` doc comments
4. `#[must_use]` annotations on key functions and types
5. Examples in doc comments use `# cfg` guards for feature-gated code

### CHANGELOG Pattern

The project follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format with sections: Added, Changed, Deprecated, Removed, Fixed. Version headers use `[version] - YYYY-MM-DD` format. Comparison links at bottom.

## Current State Assessment

### DOC-01: native-theme-build crate-level docs

**Status: NOT DONE** - `lib.rs` has no `//!` docs at all. Starts with `mod codegen;`.

What needs to be added:
- Crate-level `//!` docs explaining purpose (build-time code generation for custom icon roles)
- Complete example showing: TOML schema, build.rs setup, SVG directory layout, `load_custom_icon()` usage
- Both `generate_icons()` (simple API) and `IconGenerator` (builder API) should be showcased

The public API surface:
- `generate_icons(toml_path)` -- simple one-file API
- `IconGenerator::new().add(path).enum_name(name).generate()` -- builder API for multi-file
- Internal types (`PipelineResult`, `SizeReport`, `run_pipeline`, etc.) are already `#[doc(hidden)]`

### DOC-02: IconProvider trait rustdoc

**Status: PARTIALLY DONE** - The trait in `native-theme/src/model/icons.rs` has basic doc comments:
```rust
pub trait IconProvider: std::fmt::Debug {
    /// Return the platform/theme-specific icon name for this icon in the given set.
    fn icon_name(&self, set: IconSet) -> Option<&str>;
    /// Return bundled SVG bytes for this icon in the given set.
    fn icon_svg(&self, set: IconSet) -> Option<&'static [u8]>;
}
```

Missing: trait-level `///` doc block with usage examples showing manual implementation and generated usage (i.e., what `native-theme-build` produces). Currently the trait has only method-level docs, no trait-level explanation.

### DOC-03: load_custom_icon and load_system_icon_by_name rustdoc

**Status: DONE** - Both functions in `native-theme/src/lib.rs` have comprehensive rustdoc with:
- Dispatch chain explanation
- `# Examples` section with cfg-gated code
- `#[must_use]` annotations
- Fallback chain documentation for `load_custom_icon`

### DOC-04: All public v0.3.3 types/functions have doc comments

**Status: MOSTLY DONE** - `cargo doc --workspace --no-deps` produces zero warnings. Key public items checked:
- `IconProvider` trait: has method docs, needs trait-level docs (see DOC-02)
- `load_custom_icon()`: fully documented
- `load_system_icon_by_name()`: fully documented
- `generate_icons()`, `IconGenerator`: have docs but brief
- Connector `custom_icon_to_*` functions: all have doc comments

### DOC-05: Core crate README updated

**Status: NOT DONE** - `native-theme/README.md` does not mention custom icon roles, `IconProvider`, `load_custom_icon()`, or `native-theme-build` at all. It covers: presets, runtime reading, toolkit connectors, feature flags.

What to add: a section covering the custom icon roles workflow (TOML definitions, build.rs, loading, connector usage).

### DOC-06: No contradictions between inline docs, READMEs, design doc

**Status: CONTRADICTIONS FOUND** - The design document `docs/custom-icon-roles.md` has several discrepancies with the actual implementation:

1. **Associated type vs object-safe trait**: Design doc (lines 129-150) shows `IconProvider` with `type IconId: Eq + Hash` and methods taking `id: &Self::IconId`. Actual implementation uses a simpler object-safe trait where each enum variant IS the provider (no associated type, trait on the icon enum itself).

2. **fallback_set method**: Design doc (lines 146-149) shows a `fallback_set()` method with `Some(IconSet::Material)` default. Actual implementation has NO fallback_set -- explicitly no cross-set fallback per project rules.

3. **Generated code structure**: Design doc (lines 601-691) shows generated code producing both an enum AND a separate `AppIconProvider` struct implementing `IconProvider`. Actual codegen produces just the enum with `impl IconProvider for EnumName` directly (no separate struct).

4. **Version reference**: Design doc (line 589) shows `native-theme-build = "0.1"`. Should be `"0.3.3"` or `"0.3"`.

5. **Unknown DE key**: Design doc (line 716) says unknown DE key is an **Error**. Actual implementation emits a **Warning** (validated in `validate_de_keys` which returns warnings, not errors).

Decision needed: Should the design doc be updated to match the implementation, or is it acceptable as a historical design rationale document? The inline docs and READMEs must match the implementation regardless.

### DOC-07: Connector crate READMEs updated

**Status: NOT DONE** - Neither connector README mentions custom icon usage:
- `connectors/native-theme-gpui/README.md`: mentions `to_image_source()` but not `custom_icon_to_image_source()` or `IconProvider`
- `connectors/native-theme-iced/README.md`: mentions icon module but not `custom_icon_to_svg_handle()` or `IconProvider`

Both connectors have `custom_icon_to_*` functions (added in phase 25) that should be documented in the READMEs.

### REL-01: Version bumped to 0.3.3

**Status: DONE** - `workspace.package.version = "0.3.3"` in root Cargo.toml. All crate Cargo.tomls use `version.workspace = true`. The `native-theme` workspace dep also uses `version = "0.3.3"`.

One issue: Root `README.md` line 25 still says `native-theme = "0.3.2"` in the Quick Start code block.

### REL-02: CHANGELOG.md updated

**Status: NOT DONE** - CHANGELOG.md ends at `[0.3.2]`. No `[0.3.3]` section exists. Need to add v0.3.3 covering:

Key additions since v0.3.2 (50 commits):
- `IconProvider` trait (phase 22)
- `load_custom_icon()` and `load_system_icon_by_name()` functions (phase 22)
- `native-theme-build` crate: TOML-driven codegen with `generate_icons()` and `IconGenerator` (phase 23)
- DE-aware code generation for freedesktop icon names (phase 24)
- `custom_icon_to_image_source()` / `custom_icon_to_image_source_colored()` in gpui connector (phase 25)
- `custom_icon_to_image_handle()` / `custom_icon_to_svg_handle()` / `custom_icon_to_svg_handle_colored()` in iced connector (phase 25)
- Icon mapping gap fills: SF Symbols SystemBluetooth, Segoe Fluent SystemWifi/SystemBluetooth (phase 25.1)
- Removal of wildcard Material fallback from `load_icon` and platform loaders (phase 25.1)

### REL-03: pre-release-check.sh passes

**Status: LIKELY PASSES** - Ran individual checks:
- `cargo doc --workspace --no-deps`: zero warnings
- `cargo clippy -p native-theme`: clean
- `cargo clippy -p native-theme-build`: clean
- `cargo clippy -p native-theme-iced`: clean
- `cargo clippy -p native-theme-gpui`: **FAILS** due to upstream naga compile error (not our code)
- `cargo test -p native-theme -p native-theme-build -p native-theme-iced`: all pass
- `cargo fmt --all`: no issues

The gpui clippy failure is an upstream dependency issue (naga 27.0.3 has a `WriteColor` trait mismatch). This may need a workaround or skip in the pre-release script, or may be resolved by the time the phase executes.

### REL-04: All tests pass

**Status: DONE** (for non-gpui crates). Tests pass including doctests.

### REL-05: Clippy clean

**Status: DONE** (for non-gpui crates). Zero clippy warnings with `-D warnings`.

### REL-06: native-theme-build crates.io metadata

**Status: NOT DONE** - `cargo publish -p native-theme-build --dry-run` reports:
```
warning: manifest has no documentation, homepage or repository
```

Missing from `native-theme-build/Cargo.toml`:
- `repository.workspace = true`
- `homepage.workspace = true`
- `keywords` (workspace uses `["theme", "gui", "native", "colors", "desktop"]`)
- `categories` (workspace uses `["gui", "config", "os"]`)
- `readme = "README.md"` (but README.md doesn't exist yet -- DOC-01 may produce one, or crate-level docs suffice)

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Changelog generation | Script to parse git log | Manual curation | Changelog should tell a story, not list commits |
| Doc comment linting | Custom lint | `cargo doc --workspace --no-deps` + check for warnings | Rustdoc already catches missing docs when `#![warn(missing_docs)]` is enabled |
| Version consistency | grep scripts | Workspace version inheritance | Already in place via `version.workspace = true` |

## Common Pitfalls

### Pitfall 1: Doctest Feature Gates

**What goes wrong:** Doctests that use feature-gated APIs fail when run without those features.
**Why it happens:** `cargo test` runs doctests with default features only.
**How to avoid:** Use `# #[cfg(feature = "...")]` guards in doc examples, or use `ignore`/`no_run` attributes.
**Warning signs:** CI failures on `cargo test` without `--all-features`.

### Pitfall 2: README Code Blocks Becoming Stale

**What goes wrong:** Code examples in README.md use old API or old version numbers.
**How to avoid:** Use `#[doc = include_str!("../README.md")]` with `#[cfg(doctest)]` to compile-test README examples (the core crate already does this). Update version numbers in all code blocks.
**Warning signs:** Version mismatch between Cargo.toml and README examples.

### Pitfall 3: Missing `readme` Field in Cargo.toml

**What goes wrong:** crates.io doesn't display the README on the crate page.
**How to avoid:** Ensure `readme = "README.md"` is in Cargo.toml (or create the file if it should exist).
**Warning signs:** `cargo publish --dry-run` warnings about missing metadata.

### Pitfall 4: CHANGELOG Missing Comparison Links

**What goes wrong:** Version headers without comparison links at the bottom of CHANGELOG.md.
**How to avoid:** Follow the existing pattern -- each `[version]` header gets a `[version]: URL` link at the bottom.
**Warning signs:** Inconsistency with existing entries.

### Pitfall 5: Design Document Contradictions

**What goes wrong:** The design document (`docs/custom-icon-roles.md`) describes an API that differs from implementation. Users reading the design doc form incorrect expectations.
**How to avoid:** Either update the design doc to match implementation or add a prominent note that implementation diverged. At minimum, mark the "Selected Approach" section as superseded.
**Warning signs:** Design doc examples that don't compile against the actual API.

### Pitfall 6: gpui Upstream Compilation Failure

**What goes wrong:** `cargo clippy -p native-theme-gpui` fails due to naga 27.0.3 having an incompatible `WriteColor` trait.
**Why it happens:** Upstream dependency issue, not our code.
**How to avoid:** Either pin naga to a working version, skip gpui in pre-release-check.sh, or wait for upstream fix.
**Warning signs:** `pre-release-check.sh` failing on the gpui crate clippy/test step.

## Code Examples

### Crate-Level Doc Pattern (existing crate: native-theme-iced)

```rust
//! iced toolkit connector for native-theme.
//!
//! Maps [`native_theme::NativeTheme`] data to iced's theming system.
//!
//! # Overview
//!
//! [description]
//!
//! # Example
//!
//! ```rust
//! use native_theme::NativeTheme;
//! // ...
//! ```
```

### native-theme-build Crate-Level Doc (needs to be written)

Should follow the same pattern, showing:
1. TOML schema (`name`, `roles`, `bundled-themes`, `system-themes`)
2. Directory layout (master TOML + theme dirs + mapping.toml + SVGs)
3. build.rs setup (`native_theme_build::generate_icons("path/to/icons.toml")`)
4. Rust include (`include!(concat!(env!("OUT_DIR"), "/app_icon.rs"))`)
5. Usage with `load_custom_icon()`

### CHANGELOG Entry Pattern (existing)

```markdown
## [0.3.3] - 2026-03-17

### Added

- Item with backtick-quoted `types` and `functions`

### Changed

- ...

### Fixed

- ...
```

## Specific Findings

### Public Items Added in v0.3.3

**Core crate (native-theme):**
- `IconProvider` trait (Debug supertrait, object-safe)
- `load_custom_icon(provider, icon_set)` function
- `load_system_icon_by_name(name, set)` function
- `IconProvider` impl for `IconRole`

**Build crate (native-theme-build) -- entirely new:**
- `generate_icons(toml_path)` function
- `IconGenerator` struct with `new()`, `add(path)`, `enum_name(name)`, `generate()` methods

**gpui connector:**
- `custom_icon_to_image_source(provider, icon_set)`
- `custom_icon_to_image_source_colored(provider, icon_set, color)`

**iced connector:**
- `custom_icon_to_image_handle(provider, icon_set)`
- `custom_icon_to_svg_handle(provider, icon_set)`
- `custom_icon_to_svg_handle_colored(provider, icon_set, color)`

### Files That Need Changes

| File | Change | Requirement |
|------|--------|-------------|
| `native-theme-build/src/lib.rs` | Add crate-level `//!` docs with full example | DOC-01 |
| `native-theme/src/model/icons.rs` | Add trait-level docs to `IconProvider` | DOC-02 |
| `native-theme/README.md` | Add custom icon roles section | DOC-05 |
| `connectors/native-theme-gpui/README.md` | Add custom icon usage examples | DOC-07 |
| `connectors/native-theme-iced/README.md` | Add custom icon usage examples | DOC-07 |
| `docs/custom-icon-roles.md` | Fix contradictions with implementation | DOC-06 |
| `CHANGELOG.md` | Add v0.3.3 section | REL-02 |
| `README.md` (root) | Update version from 0.3.2 to 0.3.3 | REL-01 |
| `native-theme-build/Cargo.toml` | Add repository, homepage, keywords, categories | REL-06 |

### TODO/FIXME in Codebase

- **Zero** in `native-theme/src/` and `native-theme-build/src/`
- **One** in `connectors/native-theme-gpui/examples/showcase.rs:642`: `false // TODO: macOS / Windows detection` -- this is in example code, not published source; pre-release-check.sh will flag it with a warning but it's in an example, not src.

## Open Questions

1. **Design doc update scope**
   - What we know: The design doc has 5 concrete contradictions with implementation
   - What's unclear: Should it be updated to match implementation, or marked as historical?
   - Recommendation: Add a "Implementation Notes" section at the top noting that Approach F was simplified during implementation. Update the version reference. Leave the detailed approach analysis as-is (it's useful history).

2. **gpui upstream naga failure**
   - What we know: naga 27.0.3 fails to compile, blocking `cargo clippy -p native-theme-gpui`
   - What's unclear: Whether this will be fixed upstream before release
   - Recommendation: pre-release-check.sh already iterates crates individually. If gpui still fails, skip it in pre-release or note it as a known upstream issue.

3. **native-theme-build README vs crate-level docs**
   - What we know: The crate has no README.md and no crate-level docs
   - What's unclear: Whether to create a README.md (for GitHub) or rely solely on crate-level `//!` docs (for docs.rs)
   - Recommendation: Create crate-level `//!` docs (mandatory for DOC-01). Optionally add a minimal README.md pointing to docs.rs. The `#[doc = include_str!]` pattern used by the core crate would require the README to contain compilable examples, which is harder for a build-dependency crate.

## Sources

### Primary (HIGH confidence)

- Direct codebase inspection of all workspace crates
- `cargo doc --workspace --no-deps` output (zero warnings)
- `cargo clippy` per-crate output
- `cargo test` per-crate output
- `cargo publish --dry-run` output for metadata validation
- `git log v0.3.2..HEAD --oneline` for change enumeration

### Secondary (MEDIUM confidence)

- Keep a Changelog format specification: https://keepachangelog.com/en/1.1.0/
- crates.io package metadata requirements: https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata

## Metadata

**Confidence breakdown:**
- Documentation gaps: HIGH - directly verified by reading all source files
- CHANGELOG content: HIGH - enumerated from git log between v0.3.2 and HEAD
- Release readiness: HIGH - ran all checks locally
- Design doc contradictions: HIGH - compared design doc text against actual trait definition

**Research date:** 2026-03-17
**Valid until:** 2026-04-17 (stable -- no external dependencies changing)
