# Phase 38: CI, Smoke Tests, and Release - Research

**Researched:** 2026-03-20
**Domain:** CI/CD pipeline, pre-release validation, crate publishing, git tagging
**Confidence:** HIGH

## Summary

Phase 38 is the final phase of the v0.4.1 Release Prep milestone. It covers three distinct work streams: (1) verifying and fixing CI so it passes on the current codebase (78 unpushed commits since v0.3.3), (2) running comprehensive pre-release smoke tests, and (3) executing the release sequence (version bump 0.4.0 to 0.4.1, CHANGELOG update, git tag, push, crates.io publish).

The current codebase has several CI-blocking issues that must be resolved BEFORE pushing to main or tagging. The `#![warn(missing_docs)]` attribute added in Phase 33 generates 63 warnings that become errors under CI's `RUSTFLAGS=-Dwarnings`. There is also a cargo fmt violation in the gpui showcase example, a broken intra-doc link in native-theme-iced (`iced::time::every`), and an uncommitted Cargo.lock change. The existing `pre-release-check.sh` script covers most smoke test needs but only validates `cargo publish --dry-run` for native-theme, not for native-theme-build or native-theme-iced. The publish workflow already handles dependency-ordered publishing with a 30-second index wait between core and connectors.

The workspace version must be bumped from 0.4.0 to 0.4.1 in `Cargo.toml` (workspace.package.version and workspace.dependencies native-theme version), the CHANGELOG needs a v0.4.1 entry, and README Quick Start must update from `"0.4.0"` to `"0.4.1"`. The `native-theme-gpui` connector should have `publish = false` added to its Cargo.toml since gpui is not on crates.io (currently it has `continue-on-error: true` in the publish workflow, which is fragile). Publishing order: native-theme, native-theme-build, (wait 30s), native-theme-iced. The CARGO_REGISTRY_TOKEN secret is already configured.

**Primary recommendation:** Fix the three CI-blocking issues first (missing docs, formatting, broken doc link), then bump version to 0.4.1, update CHANGELOG, run pre-release-check.sh, push to main, verify CI passes, tag v0.4.1, and publish.

## Standard Stack

### Core
| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| cargo test | Rust 1.94.0 | Run unit and doc tests | Built-in Cargo command |
| cargo clippy | Rust 1.94.0 | Lint with `-Dwarnings` | CI gate for code quality |
| cargo fmt | Rust 1.94.0 | Format checking | CI gate for consistency |
| cargo doc | Rust 1.94.0 | Documentation build with `-Dwarnings` | CI gate for doc quality |
| cargo publish | Rust 1.94.0 | Publish to crates.io | Built-in with `--workspace` support |
| cargo audit | latest | Security vulnerability scanning | Used by pre-release-check.sh |
| cargo outdated | latest | Dependency freshness check | Used by pre-release-check.sh |
| gh | latest | GitHub CLI for releases | Create GitHub releases with notes |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| pre-release-check.sh | Comprehensive pre-release validation | Run before tagging; covers fmt, clippy, test, doc, audit, outdated, dry-run |
| publish.yml workflow | Automated crates.io publishing on tag push | Triggered by `v*` tags; handles dependency ordering |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Manual sequential publish | `cargo publish --workspace` (stable in 1.94) | Workspace publish handles ordering automatically but the existing publish.yml already works well with sequential approach and 30s wait |
| Manual git tag + push | cargo-release / release-plz | Overkill for a one-time release; manual is clearer |

## Architecture Patterns

### Release Sequence (Dependency-Compatible Order)

```
1. Fix CI blockers (missing docs, fmt, doc link)
2. Bump version: 0.4.0 -> 0.4.1 in workspace Cargo.toml
3. Update CHANGELOG.md with v0.4.1 entry
4. Update README version ref ("0.4.0" -> "0.4.1")
5. Commit all changes
6. Run pre-release-check.sh
7. Push to main (triggers CI)
8. Verify CI passes
9. Tag v0.4.1, push tag
10. Publish workflow runs automatically (or manual trigger)
11. Create GitHub release with notes
```

### Publishing Order

```
1. native-theme        (core crate, no workspace deps)
2. native-theme-build  (no workspace deps)
   -- wait 30s for crates.io index --
3. native-theme-iced   (depends on native-theme from crates.io)
4. native-theme-gpui   (SKIP: publish = false, gpui not on crates.io)
```

### CI Workflow Structure (Existing)

```
.github/workflows/
  ci.yml       -> fmt, clippy (per-crate), test (cross-platform matrix), doc, audit
  publish.yml  -> CI gate + sequential publish on v* tags
  docs.yml     -> Build & deploy to GitHub Pages on push to main or v* tags
```

### Version Bump Pattern

Only TWO lines need changing in root `Cargo.toml`:
```toml
[workspace.package]
version = "0.4.1"   # was "0.4.0"

[workspace.dependencies]
native-theme = { path = "native-theme", version = "0.4.1" }  # was "0.4.0"
```

All member crates use `version.workspace = true`, so they inherit automatically.

### Anti-Patterns to Avoid
- **Pushing before fixing CI blockers:** The 63 missing-doc warnings will fail CI immediately under -Dwarnings. Fix them first.
- **Tagging before verifying CI:** Always push to main first, wait for CI green, then tag.
- **Publishing gpui connector:** gpui/gpui-component are not on crates.io. The connector publish will always fail. Add `publish = false`.
- **Forgetting Cargo.lock:** The uncommitted Cargo.lock change (tokio added to iced dependency tree) must be committed.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Pre-release validation | Custom check script | Existing `pre-release-check.sh` | Already covers fmt, clippy, test, doc, audit, outdated, dry-run |
| CI pipeline | New workflow | Existing `ci.yml` | Already has cross-platform matrix, per-crate clippy, security audit |
| Publish automation | Manual cargo publish | Existing `publish.yml` | Handles dependency ordering, index wait, CI gate |
| Missing doc resolution | Individual doc comments | `#![allow(missing_docs)]` on specific modules | 63 warnings are too many to fix in a release phase; suppress with targeted allows |

**Key insight:** The release infrastructure (ci.yml, publish.yml, pre-release-check.sh) already exists from Phase 13 and 15. This phase is about fixing regressions, running validation, and executing the release.

## Common Pitfalls

### Pitfall 1: Missing Docs Warnings Become CI Errors
**What goes wrong:** CI sets `RUSTFLAGS=-Dwarnings` for clippy, turning the 63 `#![warn(missing_docs)]` warnings into errors.
**Why it happens:** Phase 33 added `#![warn(missing_docs)]` to lib.rs but many public items (8 modules, 8 variants, 44 struct fields) lack doc comments. No CI run has happened since.
**How to avoid:** Either add doc comments to all 63 items, or change to `#[allow(missing_docs)]` on specific undocumented items/modules, or move `#![warn(missing_docs)]` out of CI scope by adjusting the clippy invocation. The cleanest approach: add targeted `#[allow(missing_docs)]` on the modules that have undocumented internals (color.rs fields, LinuxDesktop variants, etc.), since documenting 63 items is out of scope for a release phase.
**Warning signs:** `RUSTFLAGS=-Dwarnings cargo clippy -p native-theme --all-targets` fails with 63 errors.

### Pitfall 2: Formatting Violation in gpui Showcase
**What goes wrong:** `cargo fmt --all --check` fails due to import ordering and expression formatting in `connectors/native-theme-gpui/examples/showcase.rs`.
**Why it happens:** Recent Phase 35 changes were not formatted before commit.
**How to avoid:** Run `cargo fmt --all` before committing.
**Warning signs:** `cargo fmt --all --check` output shows diffs.

### Pitfall 3: Broken Intra-Doc Link in Iced Connector
**What goes wrong:** `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps` fails on native-theme-iced because `iced::time::every()` is an unresolved link.
**Why it happens:** The doc comment references `iced::time::every()` but `iced` is only a dev-dependency, not in scope during doc builds.
**How to avoid:** Use backtick-only formatting: `` `iced::time::every()` `` instead of doc link syntax `[`iced::time::every()`]`.
**Warning signs:** `cargo doc` error about unresolved link in `connectors/native-theme-iced/src/icons.rs:104`.

### Pitfall 4: crates.io Index Propagation Delay
**What goes wrong:** Publishing native-theme-iced immediately after native-theme fails because crates.io hasn't indexed the new version.
**Why it happens:** crates.io takes seconds to minutes to update its index.
**How to avoid:** The publish.yml already has a 30-second sleep. If manual publishing, wait at least 30 seconds between core and connectors.
**Warning signs:** "failed to select a version" during connector publish.

### Pitfall 5: Version Mismatch in README Quick Start
**What goes wrong:** Users copy `native-theme = "0.4.0"` from README after v0.4.1 is published. Works but confusing.
**Why it happens:** Root README line 26 has the exact version `"0.4.0"` (other refs use `"0.4"` which is fine).
**How to avoid:** Update README line 26 to `"0.4.1"` (or use `"0.4"` for semver compatibility).
**Warning signs:** Grep for exact version strings in markdown files.

### Pitfall 6: gpui Connector Publish Failure
**What goes wrong:** `cargo publish -p native-theme-gpui` fails because its dependency `gpui = "0.2.2"` is on crates.io but `native-theme = "0.4.1"` hasn't been published yet (or other issues).
**Why it happens:** publish.yml has it as a step with `continue-on-error: true` but it still produces noisy failures.
**How to avoid:** Add `publish = false` to `connectors/native-theme-gpui/Cargo.toml`.
**Warning signs:** Publish workflow shows red step for gpui despite overall success.

### Pitfall 7: Uncommitted Cargo.lock
**What goes wrong:** `cargo fmt --all --check` or other CI checks fail because Cargo.lock has uncommitted changes (tokio added to iced dependency tree).
**Why it happens:** A dependency update occurred locally but Cargo.lock wasn't committed.
**How to avoid:** Include Cargo.lock in the commit with CI fixes.
**Warning signs:** `git status` shows `M Cargo.lock`.

## Code Examples

### Fix Missing Docs: Targeted Allow on Undocumented Modules

```rust
// In native-theme/src/lib.rs, add #[allow(missing_docs)] to modules
// whose internals are not fully documented:

#[allow(missing_docs)]
pub mod color;
#[allow(missing_docs)]
pub mod error;
```

Or alternatively, add doc comments to the specific undocumented items. For struct fields in color.rs:

```rust
/// sRGB color with alpha channel.
pub struct Rgba {
    /// Red component (0-255).
    pub r: u8,
    /// Green component (0-255).
    pub g: u8,
    /// Blue component (0-255).
    pub b: u8,
    /// Alpha component (0-255, where 255 is fully opaque).
    pub a: u8,
}
```

### Fix Broken Doc Link in Iced Connector

```rust
// connectors/native-theme-iced/src/icons.rs:104
// Before (broken):
/// Index into the cached `Vec` using an [`iced::time::every()`] subscription

// After (fixed — backtick-only, no link resolution):
/// Index into the cached `Vec` using an `iced::time::every()` subscription
```

### Version Bump in Cargo.toml

```toml
# Root Cargo.toml — only two changes needed:
[workspace.package]
version = "0.4.1"

[workspace.dependencies]
native-theme = { path = "native-theme", version = "0.4.1" }
```

### CHANGELOG v0.4.1 Entry

```markdown
## [0.4.1] - 2026-03-20

### Added

- `CONTRIBUTING.md` with development workflow and testing guide
- `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1)
- `SECURITY.md` with responsible disclosure policy
- GitHub issue templates (bug report, feature request) using YAML forms
- Pull request template with CI checklist
- Animated icon sections in gpui and iced connector READMEs
- Animated icon showcase demonstrations in both gpui and iced examples
- CLI argument support (`--tab`, `--preset`) for showcase examples
- GIF generation script for bundled spinner animations
- Screenshot automation scripts for showcase examples
- `#![warn(missing_docs)]` crate-level lint attribute

### Changed

- Root README updated with animated icons section
- Version references updated from 0.3.x to 0.4.x across all documentation

[0.4.1]: https://github.com/tiborgats/native-theme/compare/v0.4.0...v0.4.1
```

### Git Tag and Push

```bash
# After all fixes committed and CI green on main:
git tag -a v0.4.1 -m "v0.4.1 Release Prep

Documentation, examples, visual assets, community files, and CI coverage
for the v0.4.0 Animated Icons feature."

git push origin v0.4.1
# This triggers publish.yml automatically
```

### GitHub Release Creation

```bash
gh release create v0.4.1 \
  --title "v0.4.1" \
  --notes-file - <<'EOF'
## What's Changed

This release adds documentation, showcase examples, community files, and CI coverage for the v0.4.0 Animated Icons feature.

### Highlights

- **Animated icon demonstrations** in both gpui and iced showcase examples
- **Animated icon documentation** in connector READMEs and root README
- **Community files**: CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md
- **GitHub templates**: Bug report, feature request (YAML forms), PR template
- **Visual assets**: GIF generation and screenshot automation scripts
- **CLI support**: `--tab` and `--preset` arguments for showcase examples

### Full Changelog
See [CHANGELOG.md](https://github.com/tiborgats/native-theme/blob/main/CHANGELOG.md) for details.

**Full Changelog**: https://github.com/tiborgats/native-theme/compare/v0.4.0...v0.4.1
EOF
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Sequential per-crate publish | `cargo publish --workspace` (stable) | Rust 1.90+ (mid-2025) | Handles dependency ordering automatically; but existing sequential workflow works fine |
| `continue-on-error: true` for unpublishable crates | `publish = false` in Cargo.toml | Always available | Cleaner: Cargo skips the crate entirely instead of failing and continuing |

**Current status:**
- Rust 1.94.0 with edition 2024, resolver = "3"
- `cargo publish --workspace` is stable but the existing sequential publish.yml is already proven
- CARGO_REGISTRY_TOKEN secret is configured
- Last published version on crates.io: 0.3.3 (all 4 crates)
- v0.4.0 was tagged but NOT published to crates.io (only exists as git tag)
- 78 unpushed commits on main since v0.3.3

## Open Questions

1. **v0.4.0 vs v0.4.1 on crates.io**
   - What we know: v0.4.0 exists as a git tag but was never published to crates.io. The latest on crates.io is 0.3.3.
   - What's unclear: Should v0.4.0 be published first, then v0.4.1? Or skip straight to v0.4.1?
   - Recommendation: Skip v0.4.0 on crates.io, publish directly as v0.4.1. The git tag v0.4.0 documents the API snapshot. Semver allows skipping patch versions. Publishing v0.4.0 then immediately v0.4.1 would be confusing for users.

2. **Missing Docs Resolution Strategy**
   - What we know: 63 items lack doc comments (8 modules, 8 variants, 44 struct fields, 3 other). Adding docs to all 63 is substantial work.
   - What's unclear: Does the user want full documentation or targeted `#[allow(missing_docs)]`?
   - Recommendation: For the release phase, use targeted `#[allow(missing_docs)]` on specific modules (color, error, model submodules) to unblock CI. Full documentation can be a future phase.

3. **gpui Connector crates.io Status**
   - What we know: `native-theme-gpui` 0.3.3 IS published on crates.io (via `continue-on-error`). But gpui 0.2.2 is on crates.io, so it works.
   - What's unclear: Should we continue publishing it or add `publish = false`?
   - Recommendation: Keep publishing it since it works (gpui IS on crates.io). The dry-run failure is because native-theme 0.4.0 isn't on crates.io yet, but after publishing native-theme 0.4.1, the gpui connector publish should succeed. Keep `continue-on-error: true` as a safety net.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in, Rust 1.94.0) |
| Config file | none (standard cargo test) |
| Quick run command | `cargo test -p native-theme` |
| Full suite command | `cargo test --workspace` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CI-01 | CI passes on current codebase (fmt, clippy, test, doc) | smoke | `cargo fmt --all --check && RUSTFLAGS=-Dwarnings cargo clippy -p native-theme --all-targets` | N/A (CI workflow) |
| CI-02 | Animated icon tests covered in CI matrix | verification | `cargo test -p native-theme --features material-icons,lucide-icons,system-icons,svg-rasterize` | Existing tests in lib.rs |
| CI-03 | Pre-release smoke tests pass | smoke | `./pre-release-check.sh` | Existing script |
| CI-04 | All crate dry-runs succeed | smoke | `cargo publish --dry-run -p native-theme --allow-dirty && cargo publish --dry-run -p native-theme-build --allow-dirty` | N/A |
| CI-05 | Version bumped to 0.4.1 | verification | `grep 'version = "0.4.1"' Cargo.toml` | N/A |
| CI-06 | CHANGELOG has v0.4.1 entry | manual | Visual inspection | N/A |
| CI-07 | Git tag v0.4.1 created and pushed | manual | `git tag -l v0.4.1` | N/A |
| CI-08 | Crates published to crates.io | manual | `cargo search native-theme` | N/A |

### Sampling Rate
- **Per task commit:** `cargo fmt --all --check && cargo test -p native-theme`
- **Per wave merge:** `./pre-release-check.sh`
- **Phase gate:** CI green on main, all dry-runs pass, tag pushed, publish successful

### Wave 0 Gaps
None -- existing test infrastructure and pre-release-check.sh cover all automatable phase requirements.

## Sources

### Primary (HIGH confidence)
- Local project verification: `cargo clippy`, `cargo fmt --check`, `cargo doc`, `cargo publish --dry-run` output analyzed directly
- Existing CI workflow: `.github/workflows/ci.yml` (cross-platform test matrix, per-crate clippy)
- Existing publish workflow: `.github/workflows/publish.yml` (dependency-ordered publish with index wait)
- Existing pre-release script: `pre-release-check.sh` (comprehensive validation)
- GitHub Actions run history: `gh run list` showing last CI run on v0.3.3 tag
- crates.io: `cargo search native-theme` showing latest published version is 0.3.3

### Secondary (MEDIUM confidence)
- [Cargo publish documentation](https://doc.rust-lang.org/cargo/commands/cargo-publish.html) - `--workspace` flag is stable in Rust 1.94
- [Tweag: Cargo Workspace Publishing](https://www.tweag.io/blog/2025-07-10-cargo-package-workspace/) - workspace-level publish behavior

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - all tools are built-in Cargo commands already in use by this project
- Architecture: HIGH - release sequence verified against existing workflows and crates.io state
- Pitfalls: HIGH - all pitfalls verified empirically (CI failures reproduced locally, doc link error confirmed)

**Research date:** 2026-03-20
**Valid until:** 2026-04-20 (stable domain, 30-day validity)
