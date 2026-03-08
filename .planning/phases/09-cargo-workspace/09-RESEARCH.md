# Phase 9: Cargo Workspace - Research

**Researched:** 2026-03-08
**Domain:** Cargo workspace restructuring / Rust project organization
**Confidence:** HIGH

## Summary

This phase converts the existing single-crate `native-theme` repository into a Cargo virtual workspace. The core crate moves into a `native-theme/` subdirectory with its own `Cargo.toml`, while a new root `Cargo.toml` defines the workspace. Two connector stub crates (`native-theme-gpui` and `native-theme-iced`) are created as workspace members with path dependencies on the core crate. No functionality changes -- purely structural.

The key complexity is ensuring all existing tests (137 passing + 3 ignored doc-tests) continue to pass after the restructure. Critical areas: the `include_str!("../README.md")` in `lib.rs` for doc-tests (path changes when source moves), the `Cargo.lock` staying at workspace root (automatic with virtual workspaces), and deciding which root-level files/directories stay at repo root vs. move into the core crate subdirectory.

Cargo workspaces are a well-established, stable Rust feature (since Rust 1.12, workspace dependency inheritance since 1.64). The edition 2024 the project uses implies resolver version 3, which must be explicitly set in virtual workspace manifests since there is no root `package.edition` to infer from.

**Primary recommendation:** Use a virtual workspace (no root package), `resolver = "3"`, workspace dependency inheritance for shared deps, and `git mv` for the atomic restructure commit.

<user_constraints>

## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Git history
- Use `git mv` to move files into the core crate subdirectory (preserve blame/log tracking)
- Single atomic commit for the entire restructuring (move + workspace Cargo.toml + adjustments)
- Keep at repo root: .git, .gitignore, LICENSE files, README, .planning/, Cargo.lock
- Everything else moves into the core crate subdirectory

#### Connector crate stubs
- Create placeholder crates for gpui and iced connectors in this phase (not deferred to Phase 14)
- Stubs include Cargo.toml with a workspace dependency on the core crate, plus an empty lib.rs
- Use workspace dependency inheritance -- define shared deps at workspace level, connectors use `{ workspace = true }`

### Claude's Discretion
- Cargo.lock placement (workspace convention)
- Crate naming for connectors
- Workspace-level metadata (edition, resolver)
- Directory naming for the core crate subdirectory

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.

</user_constraints>

<phase_requirements>

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| API-01 | Repo converted to Cargo workspace with core crate in `native-theme/` subdirectory | All research findings below support this -- workspace manifest format, directory structure, dependency inheritance, git mv strategy, path fixups |

</phase_requirements>

## Standard Stack

### Core
| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| Cargo workspaces | Stable since Rust 1.12 | Multi-crate project organization | Built into Cargo, no external tooling needed |
| Workspace dep inheritance | Stable since Rust 1.64 | DRY dependency declarations | Avoids version drift between workspace members |
| Resolver v3 | Rust 1.84+ (edition 2024) | MSRV-aware dependency resolution | Required for edition 2024; must be explicit in virtual workspaces |

### Supporting
| Tool | Version | Purpose | When to Use |
|------|---------|---------|-------------|
| `git mv` | Any | Move files preserving rename tracking | When restructuring directories in the atomic commit |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Virtual workspace | Root package workspace | Virtual is cleaner -- no package in root means `cargo build` builds all members; root package requires `--workspace` flag for multi-member builds |

**No installation needed** -- all tooling is built into Cargo 1.94.0 (installed on system).

## Architecture Patterns

### Recommended Project Structure (Post-Restructure)

```
native-theme/                    # repo root
+-- Cargo.toml                   # [workspace] manifest (virtual -- no [package])
+-- Cargo.lock                   # shared lockfile (auto-managed by Cargo for workspaces)
+-- README.md                    # repo-level README (stays at root)
+-- .gitignore                   # stays at root
+-- .planning/                   # stays at root
+-- .claude/                     # stays at root (AI tooling config)
+-- .codex/                      # stays at root (AI tooling config)
+-- .gemini/                     # stays at root (AI tooling config)
+-- .opencode/                   # stays at root (AI tooling config)
+-- docs/                        # stays at root (project-level docs)
+-- native-theme/                # core crate subdirectory
|   +-- Cargo.toml               # [package] name = "native-theme"
|   +-- src/
|   |   +-- lib.rs
|   |   +-- color.rs
|   |   +-- error.rs
|   |   +-- presets.rs
|   |   +-- presets/             # .toml preset files
|   |   +-- model/
|   |   +-- kde/
|   |   +-- gnome/
|   |   +-- windows.rs
|   +-- tests/
|       +-- merge_behavior.rs
|       +-- preset_loading.rs
|       +-- serde_roundtrip.rs
+-- connectors/
    +-- native-theme-gpui/
    |   +-- Cargo.toml           # depends on native-theme = { workspace = true }
    |   +-- src/
    |       +-- lib.rs           # empty stub
    +-- native-theme-iced/
        +-- Cargo.toml           # depends on native-theme = { workspace = true }
        +-- src/
            +-- lib.rs           # empty stub
```

### Pattern 1: Virtual Workspace Manifest
**What:** A root `Cargo.toml` that defines only `[workspace]`, no `[package]`
**When to use:** When the workspace itself is not a crate -- it only organizes members
**Why:** Virtual workspaces treat all members equally. `cargo build` and `cargo test` from root operate on all members by default (no need for `--workspace`).

```toml
# Source: https://doc.rust-lang.org/cargo/reference/workspaces.html
[workspace]
members = [
    "native-theme",
    "connectors/native-theme-gpui",
    "connectors/native-theme-iced",
]
resolver = "3"

[workspace.package]
edition = "2024"
license = "MIT OR Apache-2.0 OR 0BSD"

[workspace.dependencies]
native-theme = { path = "native-theme" }
serde = { version = "1.0.228", features = ["derive"] }
serde_with = "3.17.0"
toml = "1.0.6"
```

### Pattern 2: Member Crate Inheriting Workspace Config
**What:** A member `Cargo.toml` that references workspace-level settings
**When to use:** Every workspace member crate

```toml
# native-theme/Cargo.toml (core crate)
# Source: https://doc.rust-lang.org/cargo/reference/workspaces.html
[package]
name = "native-theme"
version = "0.2.0"
edition.workspace = true
license.workspace = true
description = "Cross-platform native theme detection and loading for Rust GUI applications"

[features]
kde = ["dep:configparser"]
portal = ["dep:ashpd"]
portal-tokio = ["portal", "ashpd/tokio"]
portal-async-io = ["portal", "ashpd/async-io"]
windows = ["dep:windows"]

[dependencies]
ashpd = { version = "0.13.4", optional = true, default-features = false, features = ["settings"] }
configparser = { version = "3.1.0", optional = true }
windows = { version = ">=0.59, <=0.62", optional = true, default-features = false, features = [
    "UI_ViewManagement",
    "Win32_UI_WindowsAndMessaging",
] }
serde.workspace = true
serde_with.workspace = true
toml.workspace = true

[dev-dependencies]
serde_json = "1.0.149"
```

### Pattern 3: Connector Stub Crate
**What:** Minimal crate that declares a workspace dependency on the core crate
**When to use:** Placeholder crates for future development

```toml
# connectors/native-theme-iced/Cargo.toml
[package]
name = "native-theme-iced"
version = "0.1.0"
edition.workspace = true
license.workspace = true
description = "iced toolkit connector for native-theme"

[dependencies]
native-theme.workspace = true
```

```rust
// connectors/native-theme-iced/src/lib.rs
//! iced toolkit connector for native-theme.
//!
//! Maps [`native_theme::NativeTheme`] data to iced's theming system.
```

### Anti-Patterns to Avoid
- **Root package workspace instead of virtual:** Putting `[package]` in the root Cargo.toml makes `cargo test` only run root package tests by default. Virtual workspaces run all members.
- **Not setting `resolver = "3"` in virtual workspace:** Virtual workspaces have no `package.edition` to infer resolver from. Without explicit setting, Cargo defaults to resolver v1, which has worse feature unification behavior.
- **Hardcoding dependency versions in each member:** Use `[workspace.dependencies]` to define shared deps once. Members reference them with `{ workspace = true }`.
- **Flat connector directories at root:** Putting `native-theme-gpui/` at repo root creates clutter. Use a `connectors/` grouping directory.

## Discretion Recommendations

These are areas marked as Claude's Discretion in CONTEXT.md.

### Cargo.lock Placement
**Recommendation:** Keep at workspace root (automatic behavior).

Cargo automatically places `Cargo.lock` at the workspace root for virtual workspaces. No action needed -- `Cargo.lock` already exists at repo root and will continue to be managed there. The user's locked decision to keep it at root aligns with Cargo's default.

### Crate Naming for Connectors
**Recommendation:** `native-theme-gpui` and `native-theme-iced`

This follows the Rust ecosystem convention of `{core-crate}-{toolkit}` naming (e.g., `serde_json`, `tokio-util`, `wgpu-hal`). Hyphenated names are idiomatic for multi-word crate names. The crate names map to module names `native_theme_gpui` and `native_theme_iced` (Cargo auto-converts hyphens to underscores).

### Workspace-Level Metadata
**Recommendation:**
- `edition = "2024"` -- matches current crate edition
- `resolver = "3"` -- required explicitly for virtual workspaces with edition 2024
- `license = "MIT OR Apache-2.0 OR 0BSD"` -- shared across all crates

These go in `[workspace.package]` for inheritance by members via `edition.workspace = true`.

### Directory Naming for Core Crate
**Recommendation:** `native-theme/` (matches the crate/package name)

This is the standard Rust convention: directory name matches `package.name`. The ROADMAP already specifies `native-theme/` as the subdirectory name in the success criteria.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Multi-crate dependency management | Custom build scripts coordinating crates | Cargo workspace with `[workspace.dependencies]` | Cargo handles version resolution, feature unification, and build ordering |
| Workspace-wide commands | Shell scripts wrapping per-crate cargo commands | `cargo test` / `cargo build` from workspace root | Virtual workspaces automatically include all members |
| Shared metadata across crates | Copy-pasting edition/license in each Cargo.toml | `[workspace.package]` inheritance | Single source of truth, no drift |

**Key insight:** Cargo workspaces handle all the orchestration. The only manual work is the file restructuring and Cargo.toml authoring.

## Common Pitfalls

### Pitfall 1: `include_str!("../README.md")` Path Breaks After Move
**What goes wrong:** `src/lib.rs` line 8 has `#[doc = include_str!("../README.md")]` which resolves relative to the source file. After moving source to `native-theme/src/lib.rs`, the README at repo root would be at `../../README.md`, not `../README.md`.
**Why it happens:** `include_str!` paths are relative to the file containing the macro invocation.
**How to avoid:** Two options:
1. **Symlink** (recommended): Create `native-theme/README.md` as a symlink to `../README.md`. The `include_str!("../README.md")` path stays the same. Symlinks are resolved by Cargo during publish.
2. **Change path**: Update to `include_str!("../../README.md")`. Works locally but may fail on `cargo publish` (files outside package root are not included in the published crate).
3. **Crate-local README**: Create a separate `native-theme/README.md` for the crate (different from repo README). Most practical for publishing.

**Recommendation:** Option 3 -- create a crate-specific README inside `native-theme/` (can be shorter, focused on API usage). Update `include_str!` to point to `"../README.md"` (relative to `src/lib.rs`, pointing to `native-theme/README.md`). This avoids cross-package-boundary path issues and works cleanly with `cargo publish`.
**Warning signs:** Doc-tests fail after the move with "file not found" errors.

### Pitfall 2: Forgetting `resolver = "3"` in Virtual Workspace
**What goes wrong:** Virtual workspaces have no `package.edition` to infer the resolver version. Without explicit `resolver = "3"`, Cargo falls back to resolver v1.
**Why it happens:** Only root-package workspaces auto-infer resolver from edition. Virtual workspaces require explicit configuration.
**How to avoid:** Always set `resolver = "3"` in `[workspace]` for edition 2024 projects.
**Warning signs:** Unexpected feature unification behavior; deps compiled with wrong feature sets.

### Pitfall 3: Root-Level Dotfiles/Directories Moving Into Core Crate
**What goes wrong:** The locked decision says "Everything else moves into the core crate subdirectory." Taken literally, this would move `.claude/`, `.codex/`, `.gemini/`, `.opencode/`, and `docs/` into `native-theme/`. These are repo-level tooling configs and project documentation, NOT crate source.
**Why it happens:** Overly broad interpretation of "everything else."
**How to avoid:** Only move files that belong to the crate: `src/`, `tests/`, and the current `Cargo.toml` (which becomes the crate's manifest). Keep all dotfile directories and `docs/` at repo root alongside `.planning/`, `.gitignore`, `README.md`, etc.
**Warning signs:** AI tool configs stop working; `docs/IMPLEMENTATION.md` becomes invisible at the project level.

### Pitfall 4: Feature Unification Across Workspace Members
**What goes wrong:** If the connector stub crates enable features on shared dependencies that differ from the core crate's usage, Cargo unifies features during workspace-wide builds, potentially changing behavior.
**Why it happens:** Cargo builds workspace members with the union of all features requested across the workspace for each dependency.
**How to avoid:** For this phase (stubs only), connector crates should have minimal dependencies. The core crate's optional features (`kde`, `portal`, `windows`) remain isolated since connectors don't enable them. Monitor this in Phase 14 when connectors gain real dependencies.
**Warning signs:** Unexpected compilation of platform-specific code when building from workspace root.

### Pitfall 5: `git mv` Ordering and Cargo.toml Conflict
**What goes wrong:** If you `git mv Cargo.toml native-theme/Cargo.toml` before creating the workspace root `Cargo.toml`, git sees the root as having no manifest. The move and new file creation must happen in the right order within the commit.
**Why it happens:** `git mv` is a rename -- it removes the source. You cannot have two `Cargo.toml` files at root.
**How to avoid:** Sequence within the atomic commit:
1. `git mv` everything into `native-theme/` (including `Cargo.toml` -> `native-theme/Cargo.toml`)
2. Create new workspace root `Cargo.toml`
3. Create connector stub directories and files
4. Edit `native-theme/Cargo.toml` to add workspace inheritance
5. Stage all changes, commit atomically
**Warning signs:** `cargo build` fails because root Cargo.toml is missing or malformed.

### Pitfall 6: Integration Tests Referencing Crate Incorrectly
**What goes wrong:** Files in `tests/` use `use native_theme::...`. After the move, these files are at `native-theme/tests/` and still reference the same crate. This should work fine since the crate name hasn't changed.
**Why it happens:** Non-issue in this case, but worth verifying.
**How to avoid:** Run `cargo test` after restructure and confirm all 137+ tests pass.
**Warning signs:** Compilation errors in integration tests.

## Code Examples

Verified patterns from official documentation.

### Workspace Root Cargo.toml (Virtual Manifest)

```toml
# Source: https://doc.rust-lang.org/cargo/reference/workspaces.html
[workspace]
members = [
    "native-theme",
    "connectors/native-theme-gpui",
    "connectors/native-theme-iced",
]
resolver = "3"

[workspace.package]
edition = "2024"
license = "MIT OR Apache-2.0 OR 0BSD"

[workspace.dependencies]
native-theme = { path = "native-theme" }
serde = { version = "1.0.228", features = ["derive"] }
serde_with = "3.17.0"
toml = "1.0.6"
```

### Core Crate Cargo.toml (After Move)

```toml
# native-theme/Cargo.toml
[package]
name = "native-theme"
version = "0.2.0"
edition.workspace = true
license.workspace = true
description = "Cross-platform native theme detection and loading for Rust GUI applications"

[features]
kde = ["dep:configparser"]
portal = ["dep:ashpd"]
portal-tokio = ["portal", "ashpd/tokio"]
portal-async-io = ["portal", "ashpd/async-io"]
windows = ["dep:windows"]

[dependencies]
ashpd = { version = "0.13.4", optional = true, default-features = false, features = ["settings"] }
configparser = { version = "3.1.0", optional = true }
windows = { version = ">=0.59, <=0.62", optional = true, default-features = false, features = [
    "UI_ViewManagement",
    "Win32_UI_WindowsAndMessaging",
] }
serde.workspace = true
serde_with.workspace = true
toml.workspace = true

[dev-dependencies]
serde_json = "1.0.149"
```

### Connector Stub Cargo.toml

```toml
# connectors/native-theme-gpui/Cargo.toml
[package]
name = "native-theme-gpui"
version = "0.1.0"
edition.workspace = true
license.workspace = true
description = "gpui toolkit connector for native-theme"

[dependencies]
native-theme.workspace = true
```

### Connector Stub lib.rs

```rust
// connectors/native-theme-gpui/src/lib.rs
//! gpui toolkit connector for native-theme.
//!
//! Maps [`native_theme::NativeTheme`] data to gpui-component's theming system.
```

### Git Mv Sequence (Atomic Commit)

```bash
# 1. Move crate files into subdirectory
mkdir -p native-theme
git mv src native-theme/src
git mv tests native-theme/tests
git mv Cargo.toml native-theme/Cargo.toml

# 2. Create workspace root Cargo.toml (new file, git add)
# 3. Edit native-theme/Cargo.toml (add workspace inheritance)
# 4. Create connector stubs
mkdir -p connectors/native-theme-gpui/src
mkdir -p connectors/native-theme-iced/src
# 5. Create connector Cargo.toml and lib.rs files
# 6. Handle README.md for doc-tests
# 7. Stage everything and commit
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `resolver = "1"` default | `resolver = "3"` with edition 2024 | Rust 1.84 (late 2024) | Must set explicitly in virtual workspaces |
| Copy dep versions in each member | `[workspace.dependencies]` inheritance | Rust 1.64 (Sep 2022) | DRY dependency management across workspace |
| Copy package metadata in each member | `[workspace.package]` inheritance | Rust 1.64 (Sep 2022) | Share edition, license, etc. |
| `resolver = "2"` for feature unification fix | `resolver = "3"` adds MSRV-awareness | Rust 1.84 | v3 includes v2 feature behavior plus MSRV-aware resolution |

**Deprecated/outdated:**
- `[replace]` in workspace Cargo.toml: deprecated, use `[patch]` instead
- Resolver v1: lacks feature deduplication for build/dev/target deps

## Open Questions

1. **Which root-level directories stay vs. move?**
   - What we know: User locked decision says keep `.git`, `.gitignore`, LICENSE files, README, `.planning/`, `Cargo.lock` at root. "Everything else moves."
   - What's unclear: `.claude/`, `.codex/`, `.gemini/`, `.opencode/`, `docs/` are repo-level configs/docs, not crate source. Moving them into the core crate subdirectory would be incorrect.
   - Recommendation: Keep all dotfile directories (`.claude/`, `.codex/`, `.gemini/`, `.opencode/`) and `docs/` at repo root. Only move `src/`, `tests/`, and `Cargo.toml` into `native-theme/`. Interpret "everything else" as "all crate-related files" rather than literally every file. Flag to user during planning if needed.

2. **README.md for doc-tests after move**
   - What we know: `lib.rs` line 8 uses `include_str!("../README.md")` for doc-tests. After move to `native-theme/src/lib.rs`, path would need to be `../../README.md` to reach repo root.
   - What's unclear: Whether `../../README.md` works reliably with `cargo publish` (files outside package root may not be included).
   - Recommendation: Create a crate-specific `native-theme/README.md` (can be a brief version or symlink). Update `include_str!` path accordingly. This is cleanest for future publishing (Phase 15).

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Cargo built-in test framework (libtest) |
| Config file | None (standard Cargo test runner) |
| Quick run command | `cargo test` |
| Full suite command | `cargo test --workspace` (post-restructure) |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| API-01 | Workspace builds from root | smoke | `cargo build --workspace` | N/A (build check) |
| API-01 | All existing tests pass | integration | `cargo test --workspace` | Yes (137+ tests in native-theme/tests/) |
| API-01 | Core crate in native-theme/ subdirectory | smoke | `test -f native-theme/Cargo.toml && test -f native-theme/src/lib.rs` | N/A (file check) |
| API-01 | Top-level workspace Cargo.toml exists | smoke | `cargo metadata --format-version 1 \| grep '"workspace_root"'` | N/A (metadata check) |
| API-01 | Connector stubs compile | smoke | `cargo check -p native-theme-gpui -p native-theme-iced` | Wave 0 (new crates) |

### Sampling Rate
- **Per task commit:** `cargo test --workspace`
- **Per wave merge:** `cargo test --workspace` + `cargo check -p native-theme-gpui -p native-theme-iced`
- **Phase gate:** Full suite green (`cargo test --workspace`) before verification

### Wave 0 Gaps
- [ ] `connectors/native-theme-gpui/Cargo.toml` + `src/lib.rs` -- new stub crate
- [ ] `connectors/native-theme-iced/Cargo.toml` + `src/lib.rs` -- new stub crate
- [ ] `native-theme/README.md` -- needed for doc-test `include_str!` path resolution

## Sources

### Primary (HIGH confidence)
- [Cargo Workspaces Reference](https://doc.rust-lang.org/cargo/reference/workspaces.html) -- workspace manifest format, members, virtual workspaces, package/dependency inheritance, resolver
- [Cargo Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) -- path dependencies, `{ workspace = true }` syntax, publishing constraints
- [Rust Edition 2024 Resolver](https://doc.rust-lang.org/edition-guide/rust-2024/cargo-resolver.html) -- resolver v3 behavior, virtual workspace requirements

### Secondary (MEDIUM confidence)
- [Workspace Feature Unification Pitfall](https://nickb.dev/blog/cargo-workspace-and-the-feature-unification-pitfall/) -- feature unification behavior across workspace members
- [Including README in workspace crates](https://users.rust-lang.org/t/how-to-include-readme-as-crate-doc-in-a-cargo-workspace/138148) -- `include_str!` path issues with workspace restructuring
- [Naming conventions for workspaces](https://users.rust-lang.org/t/naming-conventions-for-cargo-workspaces/65369) -- directory and crate naming patterns

### Tertiary (LOW confidence)
- None -- all findings verified against official documentation

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- Cargo workspaces are stable, well-documented, used by this project's Rust 1.94
- Architecture: HIGH -- virtual workspace pattern is well-established; directory structure follows conventions
- Pitfalls: HIGH -- `include_str!` path issue verified against official docs and community threads; resolver requirement verified against edition guide
- Discretion items: HIGH -- naming/directory conventions well-established in Rust ecosystem

**Research date:** 2026-03-08
**Valid until:** 2026-04-08 (Cargo workspace features are stable; no breaking changes expected)
