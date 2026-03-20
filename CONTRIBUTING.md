# Contributing to native-theme

Thank you for your interest in contributing! This guide covers everything you
need to get started.

## Quick Start

1. Fork and clone the repository
2. Install Rust stable (MSRV: **1.94.0**, edition **2024**)
3. Run the test suite:

   ```bash
   cargo test --workspace
   ```

## Development Workflow

Before submitting a PR, run these checks locally. They match the CI pipeline
that runs on every pull request.

```bash
# Format
cargo fmt --all --check

# Lint each crate (CI runs clippy per-crate, not workspace-wide)
cargo clippy -p native-theme --all-targets
cargo clippy -p native-theme-build --all-targets
cargo clippy -p native-theme-gpui --all-targets
cargo clippy -p native-theme-iced --all-targets

# Tests
cargo test --workspace

# Documentation
RUSTDOCFLAGS="-Dwarnings" cargo doc --workspace --no-deps
```

> **Note:** CI sets `RUSTFLAGS=-Dwarnings`, so any clippy warning is treated as
> an error. Make sure your code is warning-free before pushing.

## Project Structure

| Crate | Path | Description |
|-------|------|-------------|
| `native-theme` | `native-theme/` | Core theme model, presets, platform readers, icons, animations |
| `native-theme-build` | `native-theme-build/` | Build-time TOML code generation for custom icon roles |
| `native-theme-gpui` | `connectors/native-theme-gpui/` | gpui toolkit connector |
| `native-theme-iced` | `connectors/native-theme-iced/` | iced toolkit connector |

## Feature Flags

The core `native-theme` crate has many feature flags:

- **Platform readers:** `kde`, `portal-tokio`, `portal-async-io`, `windows`,
  `macos`, `linux`, `native`
- **Icons:** `system-icons`, `material-icons`, `lucide-icons`
- **Rendering:** `svg-rasterize`

Platform-specific features only compile on their target OS. See
[`native-theme/Cargo.toml`](native-theme/Cargo.toml) for the full list.

## Submitting Changes

1. Fork the repository and create a feature branch
2. Make your changes
3. Run the CI commands listed above
4. Push your branch and open a pull request
5. All PRs run CI automatically -- you will see the results on the PR page

## License

By contributing, you agree that your contributions will be licensed under the
project's triple license: **MIT OR Apache-2.0 OR 0BSD**.
