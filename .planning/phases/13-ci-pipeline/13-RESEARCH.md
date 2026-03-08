# Phase 13: CI Pipeline - Research

**Researched:** 2026-03-08
**Domain:** GitHub Actions CI/CD for Rust workspace with platform-specific feature flags
**Confidence:** HIGH

## Summary

This phase creates a GitHub Actions CI workflow for the `native-theme` Rust workspace. The core challenge is that feature flags are platform-bound: `kde` and `portal-tokio` compile only on Linux (ashpd uses D-Bus), `windows` compiles only on Windows (depends on the `windows` crate), and `macos` compiles only on macOS (objc2 dependencies are gated by `cfg(target_os = "macos")` in Cargo.toml). This means the CI matrix cannot simply test all features on all platforms -- each feature must run on its native runner.

The workspace contains three crates: `native-theme` (core, with feature flags), `native-theme-gpui` (connector, no features), and `native-theme-iced` (connector, no features). The connectors currently have minimal code and depend on `native-theme` without features. The crate has not been published to crates.io yet, but a `v0.1` git tag exists, which can serve as the baseline for `cargo semver-checks --baseline-rev v0.1`.

**Primary recommendation:** Create a single `.github/workflows/ci.yml` with four jobs: `fmt` (fast, runs first), `clippy` (lint workspace), `test` (matrix: platform x features), and `semver` (API breakage detection against `v0.1` tag).

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| CI-01 | GitHub Actions workflow testing on Linux + Windows + macOS runners | Matrix strategy with `ubuntu-latest`, `windows-latest`, `macos-latest` runners; `dtolnay/rust-toolchain@stable` for toolchain; `Swatinem/rust-cache@v2` for caching |
| CI-02 | Feature flag matrix: `--no-default-features`, `--features kde`, `--features portal-tokio`, `--features windows`, `--features macos` | Platform-feature mapping: `no-default-features` on all three, `kde` and `portal-tokio` on Linux only, `windows` on Windows only, `macos` on macOS only |
| CI-03 | `cargo semver-checks` integrated for breaking change detection | `obi1kenobi/cargo-semver-checks-action@v2` with `baseline-rev: v0.1` since crate is unpublished; target only `native-theme` package |
| CI-04 | `cargo clippy` + `cargo fmt --check` in CI | Separate jobs: `cargo fmt --all -- --check` as gate job; `cargo clippy --workspace --all-targets` with `RUSTFLAGS: -Dwarnings` |
</phase_requirements>

## Standard Stack

### Core
| Tool | Version/Tag | Purpose | Why Standard |
|------|-------------|---------|--------------|
| `dtolnay/rust-toolchain` | `@stable` | Install Rust toolchain | De facto standard; concise, maintained by dtolnay, outputs cachekey |
| `actions/checkout` | `@v4` | Checkout repository | Official GitHub action, current major version |
| `Swatinem/rust-cache` | `@v2` | Cache cargo registry + target | Smart Cargo.lock-based caching, excludes workspace crates, v2.8.2 latest |
| `obi1kenobi/cargo-semver-checks-action` | `@v2` | Semver violation detection | Official action, v2.8 latest, supports workspace filtering |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| `CARGO_TERM_COLOR: always` | Colored output in CI logs | Set as workflow-level `env` |
| `RUSTFLAGS: -Dwarnings` | Turn warnings into errors | Set on clippy job only |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `dtolnay/rust-toolchain` | `rustup` directly | `dtolnay/rust-toolchain` is more concise and provides cachekey output |
| `obi1kenobi/cargo-semver-checks-action` | `cargo install cargo-semver-checks` + manual run | Action handles caching, toolchain setup; manual install wastes CI minutes |
| `Swatinem/rust-cache` | No caching | Builds take 2-5x longer without caching; cache is free for public repos |

## Architecture Patterns

### Recommended Workflow Structure

```
.github/
  workflows/
    ci.yml          # Single workflow file with multiple jobs
```

### Pattern 1: Job Dependency Graph (fmt -> clippy -> test + semver)

**What:** `fmt` runs first as a fast gate (< 10s). `clippy` and `test` depend on `fmt` passing. `semver` runs independently.
**When to use:** Always -- fail fast on formatting before burning CI minutes on compilation.

```yaml
# Job dependency structure:
# fmt (10s) --> clippy (2min) --> [done]
#          \--> test matrix (3min per entry) --> [done]
# semver (independent, 2min) --> [done]
```

### Pattern 2: Platform-Feature Matrix with `include`

**What:** Instead of a cross-product matrix (3 OS x 5 features = 15 jobs), use `include` to define only valid platform-feature combinations.
**When to use:** When features are platform-bound (this project's exact case).

The valid combinations are:

| Runner | Features | Rationale |
|--------|----------|-----------|
| `ubuntu-latest` | `--no-default-features` | Core-only compilation on Linux |
| `ubuntu-latest` | `--features kde` | KDE reader (configparser, pure Rust) |
| `ubuntu-latest` | `--features portal-tokio` | GNOME portal reader (ashpd, needs D-Bus) |
| `windows-latest` | `--no-default-features` | Core-only compilation on Windows |
| `windows-latest` | `--features windows` | Windows reader (windows crate, Windows-only) |
| `macos-latest` | `--no-default-features` | Core-only compilation on macOS |
| `macos-latest` | `--features macos` | macOS reader (objc2, macOS-only) |

This yields 7 matrix entries instead of 15, each guaranteed to compile.

### Pattern 3: Workspace-Aware Test Commands

**What:** Use `-p native-theme` for feature-specific tests (features are defined on this crate only), and `--workspace` for non-feature tests like fmt and clippy.
**When to use:** Feature flags are on the core crate; connectors don't have features.

### Anti-Patterns to Avoid

- **`--all-features` on a single runner:** Will fail because `windows` + `macos` + `kde` cannot all compile on one platform.
- **Cross-product matrix (os x features):** Testing `--features macos` on `ubuntu-latest` will fail to compile. Use `include` to whitelist valid combos.
- **`cargo test --workspace --features X`:** Feature flags don't propagate to workspace members properly with `--workspace`. Use `-p native-theme --features X` instead.
- **Running `cargo fmt` on multiple platforms:** Formatting is platform-independent. Run once on `ubuntu-latest` only.
- **Skipping `fail-fast: false`:** If one matrix entry fails, you want to see ALL failures, not just the first.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Rust toolchain install | Manual `rustup` scripting | `dtolnay/rust-toolchain@stable` | Handles components, cachekey, profile |
| Cargo caching | Manual cache key management | `Swatinem/rust-cache@v2` | Handles Cargo.lock hashing, target cleanup, registry |
| Semver checking | Manual API diffing | `cargo-semver-checks-action@v2` | 245 lints, baseline caching, workspace support |
| Feature matrix | Shell loops over features | GitHub Actions `matrix.include` | Native parallelism, per-entry logs, fail reporting |

**Key insight:** GitHub Actions' matrix strategy with `include` is purpose-built for this exact problem. Shell-script loops lose parallelism and make failure diagnosis harder.

## Common Pitfalls

### Pitfall 1: Feature Flags on Wrong Platform
**What goes wrong:** `cargo test --features windows` on `ubuntu-latest` fails with compilation errors from the `windows` crate. Similarly `--features macos` on Linux fails because objc2 deps are `cfg(target_os = "macos")` gated in Cargo.toml.
**Why it happens:** Platform-specific optional dependencies don't compile on other platforms.
**How to avoid:** Use `matrix.include` to bind each feature to its native runner. Never use `--all-features`.
**Warning signs:** Compilation errors mentioning `windows_core`, `objc2`, or missing system libraries.

### Pitfall 2: cargo semver-checks Fails on Unpublished Crate
**What goes wrong:** `cargo semver-checks` defaults to comparing against the latest version on crates.io. Since `native-theme` is not yet published, this fails.
**Why it happens:** No published baseline exists on crates.io.
**How to avoid:** Use `baseline-rev: v0.1` to compare against the `v0.1` git tag. The action also supports `baseline-rev` input.
**Warning signs:** Error messages about "no version found" or "could not find crate".

### Pitfall 3: Clippy Warnings Not Failing CI
**What goes wrong:** `cargo clippy` exits 0 even with warnings, so CI passes despite warnings.
**Why it happens:** Clippy's default behavior is to warn, not error.
**How to avoid:** Set `RUSTFLAGS: -Dwarnings` (or use `-- -D warnings` argument) to promote warnings to errors.
**Warning signs:** CI is green but local `cargo clippy` shows warnings (currently 4 warnings exist in the codebase).

### Pitfall 4: Existing Clippy Warnings and Fmt Diffs
**What goes wrong:** CI fails immediately on first run because the codebase currently has clippy warnings and formatting differences.
**Why it happens:** These were not enforced before CI existed.
**How to avoid:** Fix all clippy warnings (`cargo clippy --fix`) and format (`cargo fmt --all`) BEFORE or AS PART of the CI setup task. The current issues: 4 clippy warnings (unused import, needless returns) and formatting diffs in `tests/serde_roundtrip.rs`.
**Warning signs:** First CI run fails on lint/fmt jobs.

### Pitfall 5: Workspace Feature Propagation
**What goes wrong:** `cargo test --workspace --features kde` doesn't work as expected -- Cargo may fail because `kde` isn't a feature of the connector crates.
**Why it happens:** `--features` applies to the root package or requires `-p` to target a specific package.
**How to avoid:** Use `cargo test -p native-theme --features kde` for feature-specific testing. Use `cargo test --workspace` only for non-feature-specific runs.
**Warning signs:** Error messages about unknown features in connector crates.

### Pitfall 6: D-Bus Availability on CI
**What goes wrong:** `portal-tokio` tests might fail on CI if they try to connect to D-Bus, which isn't available on GitHub Actions runners.
**Why it happens:** GitHub Actions Ubuntu runners don't have a D-Bus session bus running.
**How to avoid:** The `portal-tokio` feature's tests that require live D-Bus should use `#[ignore]` or compile-only checks. The current test suite appears to handle this gracefully (tests compile and run without requiring live D-Bus).
**Warning signs:** Test failures mentioning "connection refused" or "D-Bus" errors.

## Code Examples

### Complete CI Workflow Structure

```yaml
# Source: Synthesis from official docs and verified patterns
name: CI

on:
  push:
    branches: [main, master]
  pull_request:

env:
  CARGO_TERM_COLOR: always

jobs:
  fmt:
    name: Rustfmt
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all -- --check

  clippy:
    name: Clippy
    needs: fmt
    runs-on: ubuntu-latest
    env:
      RUSTFLAGS: -Dwarnings
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --workspace --all-targets

  test:
    name: Test ${{ matrix.name }}
    needs: fmt
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        include:
          - { name: "Linux (no features)",    os: ubuntu-latest,  flags: "--no-default-features" }
          - { name: "Linux (kde)",            os: ubuntu-latest,  flags: "--features kde" }
          - { name: "Linux (portal-tokio)",   os: ubuntu-latest,  flags: "--features portal-tokio" }
          - { name: "Windows (no features)",  os: windows-latest, flags: "--no-default-features" }
          - { name: "Windows (windows)",      os: windows-latest, flags: "--features windows" }
          - { name: "macOS (no features)",    os: macos-latest,   flags: "--no-default-features" }
          - { name: "macOS (macos)",          os: macos-latest,   flags: "--features macos" }
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test -p native-theme ${{ matrix.flags }}

  semver:
    name: Semver Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # Need full history for baseline-rev
      - uses: obi1kenobi/cargo-semver-checks-action@v2
        with:
          package: native-theme
          baseline-rev: v0.1
```

### Clippy Fix Command (Pre-CI Cleanup)

```bash
# Fix the 4 existing clippy warnings
cargo clippy --fix --workspace --allow-dirty
# Format all code
cargo fmt --all
```

### Feature-Specific Test Command

```bash
# Correct: target specific package for features
cargo test -p native-theme --features kde

# Wrong: workspace + features (features unknown to connector crates)
cargo test --workspace --features kde  # FAILS
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `actions-rs/toolchain` | `dtolnay/rust-toolchain` | 2023 | actions-rs is unmaintained; dtolnay is actively maintained |
| `actions/cache` manually | `Swatinem/rust-cache@v2` | 2022+ | Purpose-built for Rust; handles cleanup, registry, target |
| `actions/checkout@v3` | `actions/checkout@v4` | 2023 | v4 is current, v3 still works but v4 is recommended |
| `hecrj/setup-rust-action` | `dtolnay/rust-toolchain` | 2023 | hecrj is deprecated, dtolnay is the standard |
| `ubuntu-22.04` | `ubuntu-24.04` (= `ubuntu-latest`) | 2024 | ubuntu-latest now points to 24.04 |

**Deprecated/outdated:**
- `actions-rs/*` (toolchain, cargo, clippy-check, audit-check): Unmaintained since 2022, do not use
- `hecrj/setup-rust-action`: Deprecated in favor of `dtolnay/rust-toolchain`

## Open Questions

1. **Connector crate testing in CI**
   - What we know: `native-theme-gpui` and `native-theme-iced` are in the workspace but have minimal code (just a dependency on `native-theme`). They compile and pass clippy.
   - What's unclear: Should CI explicitly test connectors? Phase 14 will add real code to them.
   - Recommendation: Include connectors in `cargo clippy --workspace` and `cargo fmt --all` but don't add dedicated feature matrix entries for them. They have no features and will be tested by the workspace-level commands.

2. **cargo semver-checks baseline after v0.2 publish**
   - What we know: Currently using `baseline-rev: v0.1` since the crate is unpublished. After Phase 15 publishes to crates.io, the default behavior (compare against crates.io) will work.
   - What's unclear: When to switch from `baseline-rev` to default.
   - Recommendation: Use `baseline-rev: v0.1` now. After publishing v0.2, remove `baseline-rev` to let it auto-detect from crates.io. Add a comment in the workflow noting this.

3. **Existing lint/format issues must be fixed first**
   - What we know: 4 clippy warnings (1 unused import in `kde/metrics.rs`, 3 needless returns in `lib.rs`) and formatting diffs in `tests/serde_roundtrip.rs`.
   - What's unclear: Nothing -- these are straightforward fixes.
   - Recommendation: Fix them as the first task in this phase, before creating the workflow file.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (libtest) |
| Config file | None (standard Cargo test) |
| Quick run command | `cargo test -p native-theme --no-default-features` |
| Full suite command | `cargo test -p native-theme --features kde,portal-tokio` (Linux) |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CI-01 | Workflow runs on 3 OS runners | manual-only | Verify by pushing to GitHub and checking Actions tab | N/A |
| CI-02 | Feature flag matrix tests each feature | manual-only | Verify matrix entries in workflow YAML + green CI runs | N/A |
| CI-03 | semver-checks catches API breakage | manual-only | Verify by checking semver job output in Actions tab | N/A |
| CI-04 | clippy + fmt run in CI | manual-only | Verify by checking lint jobs in Actions tab | N/A |

**Note:** All CI-phase requirements are verified by inspecting the workflow file and observing CI runs. There are no unit tests to write for CI configuration.

### Sampling Rate
- **Per task commit:** Verify workflow YAML syntax with `actionlint` if available, or manual inspection
- **Per wave merge:** Push to GitHub and verify all CI jobs pass
- **Phase gate:** All 4 CI jobs green on a real push

### Wave 0 Gaps
- [ ] `.github/workflows/ci.yml` -- the workflow file itself (does not exist yet)
- [ ] Fix clippy warnings in `native-theme/src/kde/metrics.rs` and `native-theme/src/lib.rs`
- [ ] Fix formatting in `native-theme/tests/serde_roundtrip.rs`

## Sources

### Primary (HIGH confidence)
- [obi1kenobi/cargo-semver-checks-action](https://github.com/obi1kenobi/cargo-semver-checks-action) - v2 action inputs, workspace usage, baseline-rev support
- [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain) - Toolchain action inputs, component support
- [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache) - v2 caching strategy, matrix build handling
- [Cargo CI docs](https://doc.rust-lang.org/cargo/guide/continuous-integration.html) - Official recommended workflow structure
- Project `Cargo.toml` files - Feature definitions, `cfg(target_os)` gating verified locally

### Secondary (MEDIUM confidence)
- [shift.click/blog/github-actions-rust](https://shift.click/blog/github-actions-rust/) - Job structure patterns (clippy with `-Dwarnings`, fmt as separate job)
- [ahmedjama.com/blog/2025/12/cross-platform-rust-pipeline-github-actions](https://ahmedjama.com/blog/2025/12/cross-platform-rust-pipeline-github-actions/) - Matrix strategy patterns
- [GitHub Docs runner images](https://docs.github.com/en/actions/reference/runners/github-hosted-runners) - Current runner labels (ubuntu-latest=24.04, macos-latest=15-arm64, windows-latest=2025)

### Tertiary (LOW confidence)
- None -- all findings verified against official sources or local project state

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All tools verified via official READMEs/docs, versions confirmed
- Architecture: HIGH - Platform-feature constraints verified locally by attempting cross-compilation
- Pitfalls: HIGH - Clippy/fmt issues confirmed by running locally; D-Bus concern documented from ashpd knowledge

**Research date:** 2026-03-08
**Valid until:** 2026-04-08 (GitHub Actions ecosystem is stable; action major versions don't change frequently)
