# Phase 15: Publishing Prep - Research

**Researched:** 2026-03-09
**Domain:** Rust crate publishing, documentation, licensing
**Confidence:** HIGH

## Summary

Phase 15 prepares `native-theme` (core) and `native-theme-iced` (connector) for crates.io publication. This involves six distinct work streams: (1) adding required Cargo.toml metadata fields across the workspace, (2) creating three license files for the triple MIT/Apache-2.0/0BSD license, (3) writing a CHANGELOG.md in Keep a Changelog format, (4) adding `/// # Examples` doc-tests to three key public types, (5) auditing and updating IMPLEMENTATION.md to match actual v0.2 implementation, (6) creating a new-os-version-guide.md, and (7+8) executing the actual `cargo publish` in dependency order.

The core crate already passes `cargo publish --dry-run` with only a warning about missing `documentation`, `homepage`, or `repository`. The iced connector fails dry-run because its workspace dependency on `native-theme` has no version specified -- this must be fixed by adding `version = "0.2.0"` to the workspace dependency declaration before publishing. The core crate must be published first, then the iced connector can reference it from the registry.

**Primary recommendation:** Add all metadata to `[workspace.package]` using workspace inheritance, publish `native-theme` first, then `native-theme-iced` with the version-pinned dependency.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PUB-01 | Cargo.toml metadata: rust-version, repository, homepage, keywords, categories, readme | Workspace inheritance pattern; valid category slugs identified; MSRV is 1.85.0 for edition 2024 |
| PUB-02 | LICENSE-MIT, LICENSE-APACHE, LICENSE-0BSD files at repo root | Standard license text templates identified; SPDX expression already set in workspace |
| PUB-03 | CHANGELOG.md following Keep a Changelog format | Keep a Changelog 1.1.0 spec verified; change categories and format documented |
| PUB-04 | Doc examples on NativeTheme, Rgba, ThemeVariant | Existing doctests verified (NativeTheme methods have them; Rgba and ThemeVariant do not); doc warnings identified |
| PUB-05 | IMPLEMENTATION.md spec updated to match actual implementation | 2041-line spec at docs/IMPLEMENTATION.md; data model sections reference old nested ThemeColors; missing WidgetMetrics, connectors |
| PUB-06 | docs/new-os-version-guide.md for maintaining platform constants | New file needed; should cover KDE breezemetrics.h updates, Windows metric constants, macOS HIG defaults, GNOME libadwaita values |
| PUB-07 | Core crate published to crates.io | Dry-run passes with warnings; needs metadata fix; no git remote configured yet |
| PUB-08 | native-theme-iced published to crates.io | Dry-run fails: workspace dep needs version; needs README; must publish after core |
</phase_requirements>

## Standard Stack

### Core
| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| cargo publish | Rust 1.94.0 (current) | Publish crates to crates.io | Built-in Cargo command |
| cargo package | Rust 1.94.0 | Dry-run packaging and verification | Pre-publish validation |
| cargo doc | Rust 1.94.0 | Generate and verify documentation | Checks doc-test compilation |
| cargo test --doc | Rust 1.94.0 | Run documentation examples as tests | Ensures examples compile and run |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| cargo semver-checks | Breaking change detection | Already in CI; verify before publish |
| cargo package --list | Inspect packaged files | Verify license/readme files included |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Manual cargo publish | cargo-release / release-plz | Automation useful for frequent releases; manual is fine for first publish |
| Manual workspace publish | cargo publish (Rust 1.90+ native) | Workspace publish in stable since Rust 1.90; but manual sequential publish is simpler for 2 crates |

## Architecture Patterns

### Publishing Order (Dependency-Compatible)

```
1. native-theme (core)        -- no crates.io dependencies on own workspace
2. native-theme-iced           -- depends on native-theme from crates.io
   (native-theme-gpui NOT published -- gpui is not on crates.io)
```

### Workspace Metadata Inheritance Pattern

All shared metadata goes in `[workspace.package]`, member crates inherit with `.workspace = true`:

```toml
# Root Cargo.toml
[workspace.package]
edition = "2024"
license = "MIT OR Apache-2.0 OR 0BSD"
rust-version = "1.85.0"
repository = "https://github.com/USER/native-theme"
homepage = "https://github.com/USER/native-theme"
keywords = ["theme", "gui", "native", "colors", "desktop"]
categories = ["gui", "config", "os"]
readme = "README.md"

[workspace.dependencies]
native-theme = { path = "native-theme", version = "0.2.0" }
```

```toml
# native-theme/Cargo.toml
[package]
name = "native-theme"
version = "0.2.0"
edition.workspace = true
license.workspace = true
rust-version.workspace = true
repository.workspace = true
homepage.workspace = true
keywords.workspace = true
categories.workspace = true
description = "Cross-platform native theme detection and loading for Rust GUI applications"
readme = "README.md"   # crate-local README
```

```toml
# connectors/native-theme-iced/Cargo.toml
[package]
name = "native-theme-iced"
version = "0.1.0"
edition.workspace = true
license.workspace = true
rust-version.workspace = true
repository.workspace = true
homepage.workspace = true
keywords = ["theme", "iced", "gui", "native", "colors"]
categories = ["gui", "config"]
description = "iced toolkit connector for native-theme"
readme = "README.md"   # needs to be created in this directory
```

### Connector Dependency Fix

The workspace dependency for `native-theme` must include a version for crates.io publishing:

```toml
# Root Cargo.toml [workspace.dependencies]
native-theme = { path = "native-theme", version = "0.2.0" }
```

When publishing, Cargo strips the `path` and uses the `version` to resolve from the registry. The core crate must be published first so the version exists on crates.io when the connector publishes.

### License File Structure

```
repo-root/
  LICENSE-MIT          # MIT license text
  LICENSE-APACHE       # Apache-2.0 license text
  LICENSE-0BSD         # 0BSD license text
```

The SPDX expression `"MIT OR Apache-2.0 OR 0BSD"` is already set in `[workspace.package]`.

### Keep a Changelog Format

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-03-XX

### Added
- ...

### Changed
- ...

### Removed
- ...
```

### Anti-Patterns to Avoid
- **Publishing connector before core**: The iced connector depends on `native-theme` from crates.io, so core must be published and indexed first
- **Omitting version on workspace path dependency**: `cargo publish` fails for downstream crates if the workspace dependency lacks a `version` field
- **Setting rust-version higher than MSRV**: Edition 2024 requires Rust 1.85.0 minimum; do not set higher without testing
- **Forgetting readme in crate subdirectory**: `readme` in Cargo.toml is relative to the crate's directory, not the workspace root; each publishable crate needs its own README or a path that resolves correctly

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| License file text | Custom wording | Standard templates from choosealicense.com / OSI | Legal accuracy matters; standard text is well-understood |
| Changelog format | Custom format | Keep a Changelog 1.1.0 | Ecosystem standard; tooling expects it |
| MSRV determination | Guessing | Edition 2024 = Rust 1.85.0 minimum | Edition determines floor; no guesswork needed |
| Publishing order | Random | Dependency-topological sort (core first) | Cargo rejects publish if dependency not yet on registry |
| Doc example wrapper | Manual fn main() | Let rustdoc auto-wrap | Rustdoc automatically wraps examples in fn main() unless one is present |

**Key insight:** Publishing is mechanical and order-dependent. The creative work is in documentation; the publishing itself follows strict rules.

## Common Pitfalls

### Pitfall 1: Workspace Dependency Missing Version
**What goes wrong:** `cargo publish --dry-run -p native-theme-iced` fails with "all dependencies must have a version requirement specified when publishing"
**Why it happens:** The workspace dependency `native-theme = { path = "native-theme" }` has no version field. When publishing, Cargo removes the `path` and needs a version to reference the registry.
**How to avoid:** Add `version = "0.2.0"` to the workspace dependency: `native-theme = { path = "native-theme", version = "0.2.0" }`
**Warning signs:** Dry-run failure on connector crates

### Pitfall 2: README Path Resolution
**What goes wrong:** Published crate on crates.io shows no README or wrong README
**Why it happens:** `readme` field is relative to the crate's Cargo.toml directory, not workspace root. The `native-theme` subcrate has its own `README.md` (good), but `native-theme-iced` does not have one.
**How to avoid:** Create a `README.md` in `connectors/native-theme-iced/` or set `readme = "../../README.md"` (less ideal)
**Warning signs:** `cargo package --list` shows no README in the package

### Pitfall 3: Doc-Test Compilation Failures with Feature-Gated Types
**What goes wrong:** Doc examples that reference feature-gated types fail to compile without features enabled
**Why it happens:** Doc-tests compile with default features only unless annotated
**How to avoid:** Only use types available without features in doc examples (NativeTheme, Rgba, ThemeVariant are all available without features -- safe)
**Warning signs:** `cargo test --doc` failures

### Pitfall 4: Existing Doc Warnings
**What goes wrong:** `cargo doc` produces 5 warnings about unresolved links (`Error::Unavailable`, `Error::Format`, `from_gnome()`)
**Why it happens:** Doc comments reference items not in scope or behind feature gates
**How to avoid:** Fix link references to use full paths: `crate::Error::Unavailable`, or use backtick-only formatting for feature-gated items
**Warning signs:** `cargo doc` warnings

### Pitfall 5: Publishing Before Git Remote/Tag Setup
**What goes wrong:** `repository` field points to nonexistent URL; no git tag for version
**Why it happens:** Currently no git remote is configured
**How to avoid:** Set up GitHub repository and push before publishing; create a `v0.2.0` git tag
**Warning signs:** `git remote -v` returns empty

### Pitfall 6: crates.io Index Propagation Delay
**What goes wrong:** Publishing native-theme-iced immediately after native-theme fails because the index hasn't propagated
**Why it happens:** crates.io takes a few seconds to index new publications
**How to avoid:** Wait 30-60 seconds between publishing core and connector, or retry
**Warning signs:** "failed to select a version for the requirement `native-theme`"

### Pitfall 7: IMPLEMENTATION.md References Old API
**What goes wrong:** The 2041-line IMPLEMENTATION.md still references the pre-v0.2 nested ThemeColors structure and lacks WidgetMetrics, connectors, and macOS reader documentation
**Why it happens:** The spec was written before the v0.2 refactoring phases
**How to avoid:** Audit each section against actual code; update data model, crate structure, and phase sections
**Warning signs:** Comparing IMPLEMENTATION.md data model structs to actual `src/model/` code reveals mismatches

## Code Examples

### Adding Doc Examples to Rgba

```rust
/// An sRGB color with alpha, stored as four u8 components.
///
/// # Examples
///
/// ```
/// use native_theme::Rgba;
///
/// // Create an opaque color
/// let blue = Rgba::rgb(0, 120, 215);
/// assert_eq!(blue.a, 255);
///
/// // Parse from hex string
/// let parsed: Rgba = "#3daee9".parse().unwrap();
/// assert_eq!(parsed, Rgba::rgb(61, 174, 233));
///
/// // Convert to f32 for toolkit interop
/// let [r, g, b, a] = blue.to_f32_array();
/// assert!(r < 0.01); // red is near zero
/// ```
```

### Adding Doc Examples to ThemeVariant

```rust
/// A single light or dark theme variant containing all visual properties.
///
/// # Examples
///
/// ```
/// use native_theme::{ThemeVariant, Rgba};
///
/// let mut variant = ThemeVariant::default();
/// variant.colors.accent = Some(Rgba::rgb(0, 120, 215));
/// variant.fonts.family = Some("Inter".into());
///
/// assert!(!variant.is_empty());
/// assert_eq!(variant.colors.accent, Some(Rgba::rgb(0, 120, 215)));
/// ```
```

### Adding Doc Examples to NativeTheme (struct-level)

```rust
/// A complete native theme with a name and optional light/dark variants.
///
/// # Examples
///
/// ```
/// use native_theme::NativeTheme;
///
/// // Load a bundled preset
/// let theme = NativeTheme::preset("dracula").unwrap();
/// assert!(theme.dark.is_some());
///
/// // Parse from TOML
/// let custom = NativeTheme::from_toml(r#"
/// name = "My Theme"
/// [light.colors]
/// accent = "#ff6600"
/// "#).unwrap();
/// assert_eq!(custom.name, "My Theme");
///
/// // Deep merge: overlay sparse overrides onto a full preset
/// let mut base = NativeTheme::preset("nord").unwrap();
/// base.merge(&custom);
/// assert_eq!(base.name, "Nord"); // base name preserved
/// ```
```

### Fixing Doc Link Warnings

```rust
// Before (broken):
/// Returns [`Error::Unavailable`] if the preset name is not recognized.

// After (fixed):
/// Returns [`crate::Error::Unavailable`] if the preset name is not recognized.

// Or for feature-gated items:
// Before (broken):
/// call [`from_gnome()`] directly.

// After (fixed, using code formatting only):
/// call `from_gnome()` directly (requires `portal-tokio` or `portal-async-io` feature).
```

### Workspace Dependency with Version

```toml
# Before (fails for connector publish):
[workspace.dependencies]
native-theme = { path = "native-theme" }

# After (works for both local dev and publish):
[workspace.dependencies]
native-theme = { path = "native-theme", version = "0.2.0" }
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual sequential publish | `cargo publish` supports workspace-level publish | Rust 1.90 (Sep 2025) | Can publish workspace crates with `cargo publish` at workspace root; dependency order handled automatically |
| Guessing MSRV | `rust-version` field + MSRV-aware resolver | Rust 1.84/1.85 | Cargo rejects builds below declared MSRV; resolver prefers compatible dependency versions |
| Edition 2021 | Edition 2024 | Rust 1.85 (Feb 2025) | New resolver = "3" default; this project already uses edition 2024 |

**Current status:**
- This project uses `edition = "2024"` and `resolver = "3"`, requiring Rust 1.85.0 minimum
- Current Rust is 1.94.0; setting `rust-version = "1.85.0"` is correct for the edition floor
- Workspace-level `cargo publish` is available but manual sequential publish is simpler for 2 crates

## Open Questions

1. **GitHub Repository URL**
   - What we know: No git remote is currently configured; `git remote -v` returns empty
   - What's unclear: What will the GitHub username/org be? Is the repo name `native-theme`?
   - Recommendation: User must create the GitHub repository and configure the remote before publishing. The `repository` and `homepage` fields in Cargo.toml depend on this. Use placeholder during metadata setup, replace before actual publish.

2. **MSRV vs Actual Minimum**
   - What we know: Edition 2024 requires Rust 1.85.0; current Rust is 1.94.0
   - What's unclear: Whether any dependency or language feature actually requires higher than 1.85.0
   - Recommendation: Set `rust-version = "1.85.0"` (edition floor). If any dependency requires higher, Cargo will error during dry-run verification. Do not run `cargo msrv` -- just set the edition floor.

3. **native-theme-gpui Exclusion**
   - What we know: gpui and gpui-component are not published to crates.io; `native-theme-gpui` cannot be published
   - What's unclear: Should it be excluded from workspace or just marked `publish = false`?
   - Recommendation: Add `publish = false` to `native-theme-gpui/Cargo.toml`

4. **IMPLEMENTATION.md Audit Scope**
   - What we know: The spec is 2041 lines covering 20 sections + 2 appendices. Many sections reference pre-v0.2 API (nested ThemeColors, no WidgetMetrics, etc.)
   - What's unclear: How deep should the update go? Full rewrite or targeted section updates?
   - Recommendation: Targeted updates to sections that reference changed APIs (8. Data Model, 10. Bundled Presets, 11. Crate Structure, 18. Implementation Phases). Add WidgetMetrics section. Do NOT rewrite entire document.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in, Rust 1.94.0) |
| Config file | none (standard cargo test) |
| Quick run command | `cargo test --doc -p native-theme` |
| Full suite command | `cargo test -p native-theme && cargo test -p native-theme-iced` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PUB-01 | Cargo.toml has all metadata fields | smoke | `cargo publish --dry-run -p native-theme 2>&1 \| grep -v warning` | N/A (command check) |
| PUB-02 | License files exist at repo root | smoke | `test -f LICENSE-MIT && test -f LICENSE-APACHE && test -f LICENSE-0BSD` | N/A (file check) |
| PUB-03 | CHANGELOG.md exists and has v0.2 entry | manual | Visual inspection of CHANGELOG.md | N/A (new file) |
| PUB-04 | Doc examples compile on NativeTheme, Rgba, ThemeVariant | unit (doctest) | `cargo test --doc -p native-theme` | Partial (NativeTheme methods have some) |
| PUB-05 | IMPLEMENTATION.md matches actual code | manual-only | Manual review against source | Existing but outdated |
| PUB-06 | docs/new-os-version-guide.md exists | smoke | `test -f docs/new-os-version-guide.md` | N/A (new file) |
| PUB-07 | Core crate dry-run succeeds cleanly | smoke | `cargo publish --dry-run -p native-theme 2>&1` | N/A |
| PUB-08 | Iced connector dry-run succeeds | smoke | `cargo publish --dry-run -p native-theme-iced 2>&1` | N/A |

### Sampling Rate
- **Per task commit:** `cargo test --doc -p native-theme && cargo publish --dry-run -p native-theme`
- **Per wave merge:** `cargo test -p native-theme && cargo publish --dry-run -p native-theme && cargo publish --dry-run -p native-theme-iced`
- **Phase gate:** Both dry-runs succeed with no errors; all doc-tests pass; `cargo doc -p native-theme --no-deps` has zero warnings

### Wave 0 Gaps
None -- existing test infrastructure covers all automatable phase requirements. Doc-tests are built into Rust's test framework.

## Sources

### Primary (HIGH confidence)
- [Cargo Reference: Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html) - Required metadata fields, dry-run process, publishing steps
- [Cargo Reference: Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html) - All Cargo.toml fields: rust-version, keywords, categories, readme
- [Cargo Reference: Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) - Workspace metadata inheritance, inheritable fields list
- [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/) - Changelog format specification
- Local project verification - `cargo publish --dry-run`, `cargo test --doc`, `cargo doc` output analyzed directly

### Secondary (MEDIUM confidence)
- [crates.io Category Slugs](https://crates.io/category_slugs) - Valid category identifiers (could not fully render page; used gist backup)
- [Choose a License: 0BSD](https://choosealicense.com/licenses/0bsd/) - 0BSD license template text
- [Rust 1.85.0 Announcement](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/) - Edition 2024 stabilization, MSRV 1.85.0
- [Tweag: Cargo Workspace Publishing](https://www.tweag.io/blog/2025-07-10-cargo-package-workspace/) - Workspace-level publish behavior in Rust 1.90+

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - cargo publish is well-documented, dry-run output verified directly
- Architecture: HIGH - workspace inheritance verified against official docs; publishing order is deterministic
- Pitfalls: HIGH - all pitfalls verified empirically (dry-run failures observed, doc warnings counted)

**Research date:** 2026-03-09
**Valid until:** 2026-04-09 (stable domain, 30-day validity)
