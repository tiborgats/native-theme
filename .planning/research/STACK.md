# Stack Research

**Domain:** Cross-platform OS theme data crate (Rust)
**Researched:** 2026-03-07
**Confidence:** HIGH (all core dependencies verified against crates.io; platform-specific deps verified against official docs)

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Rust Edition 2024 | MSRV 1.85.0 | Language edition | Stabilized in Rust 1.85.0 (Feb 2025). Enables MSRV-aware resolver by default, unsafe_op_in_unsafe_fn lint, and other 2024 edition improvements. Fresh greenfield crate -- no migration cost. |
| serde | 1.0.228 | Serialization framework | Universal standard for Rust (de)serialization. Derive macros eliminate boilerplate. Required for TOML round-tripping of the theme data model. Pin to `"1"` in Cargo.toml (semver-compatible). |
| toml | 1.0.6 | TOML parsing and serialization | Just stabilized at 1.0 (March 2026) with TOML spec 1.1.0 support. Same API as 0.8.x (`from_str`, `to_string`, `to_string_pretty`). This is the canonical Rust TOML crate. Pin to `"1.0"`. |

**Confidence: HIGH** -- All three verified against crates.io on 2026-03-07. serde and toml are the uncontested standards for their respective roles.

### Linux Platform Dependencies (feature-gated)

| Library | Version | Feature Flag | Purpose | Why Recommended |
|---------|---------|-------------|---------|-----------------|
| configparser | 3.1.0 | `kde` | INI parser for `~/.config/kdeglobals` | Zero external dependencies. Provides `Ini::new_cs()` for case-sensitive parsing (required -- kdeglobals uses CamelCase keys). Fully synchronous. Stable API. Simpler and lighter than rust-ini for read-only INI parsing. |
| dirs | 6.0.0 | `kde` | XDG/platform config directory resolution | Standard crate for locating `~/.config`, `~/.local`, etc. Used to find `kdeglobals` path. Minimal dependency footprint. |
| ashpd | 0.13.4 | `portal` | XDG Desktop Portal D-Bus wrapper | The only ergonomic Rust crate for freedesktop portal access. Provides typed enums for `ColorScheme`, `Contrast`, and typed accent color reading. Built on zbus 5.x. **Async-only** -- see notes below. |

**ashpd async runtime notes:**
- ashpd 0.13 default features include `tokio` (pulls in tokio 1.43+)
- Alternative: `default-features = false, features = ["async-io"]` for smol/glib-compatible runtimes
- There is **no `async-std` feature** in ashpd 0.13. The IMPLEMENTATION.md reference to `async-std` is outdated -- use `async-io` instead
- Recommendation: Keep default (tokio) since most Rust async consumers already use tokio

**Confidence: HIGH** -- configparser and dirs verified on crates.io. ashpd 0.13.4 verified including dependency tree (zbus 5.14.0). async-io vs async-std correction verified against ashpd docs.rs feature list.

### Windows Platform Dependencies (feature-gated)

| Library | Version | Feature Flag | Purpose | Why Recommended |
|---------|---------|-------------|---------|-----------------|
| windows | 0.62.2 | `windows` | Official Microsoft Windows API bindings | The only official, Microsoft-maintained Rust crate for Windows APIs. Provides `UISettings::GetColorValue(UIColorType)` for accent colors (8 color types including 6 accent shades), `GetSystemMetrics` for system metrics, `TextScaleFactor` for DPI scaling, and `ColorValuesChanged` for change notifications. |

**Required Cargo.toml features for the `windows` crate:**
```toml
[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.62", optional = true, features = [
    "UI_ViewManagement",              # UISettings, UIColorType, UIElementType
    "Win32_UI_WindowsAndMessaging",   # GetSystemMetrics, SystemParametersInfoW
] }
```

**Confidence: HIGH** -- Verified against microsoft.github.io/windows-docs-rs and crates.io.

### macOS Platform Dependencies (feature-gated)

| Library | Version | Feature Flag | Purpose | Why Recommended |
|---------|---------|-------------|---------|-----------------|
| objc2-app-kit | 0.3.2 | `macos` | AppKit bindings (NSColor, NSFont, NSAppearance) | Part of the objc2 ecosystem (objc2 0.6.4). Generated from Xcode 16.4 SDKs. Provides access to ~40 NSColor semantic colors, NSFont system font queries, and NSAppearance for dark/light detection. The only actively maintained, safe(r) Objective-C bindings for Rust. |

**Required Cargo.toml features for objc2-app-kit:**
```toml
[target.'cfg(target_os = "macos")'.dependencies]
objc2-app-kit = { version = "0.3", optional = true, features = [
    "NSColor",
    "NSFont",
    "NSAppearance",
    "NSColorSpace",
] }
```

**objc2 ecosystem version compatibility:**
- objc2-app-kit 0.3.2 requires objc2 >=0.6.2, <0.8.0
- objc2-foundation 0.3.2 is pulled in transitively
- objc2 core 0.6.4 is the current latest
- All versions are compatible and co-resolve cleanly

**Confidence: HIGH** -- Dependency chain verified: objc2-app-kit 0.3.2 -> objc2 >=0.6.2 confirmed on crates.io.

### iOS Platform Dependencies (feature-gated)

| Library | Version | Feature Flag | Purpose | Why Recommended |
|---------|---------|-------------|---------|-----------------|
| objc2-ui-kit | 0.3.2 | `ios` | UIKit bindings (UIColor, UIFont, UITraitCollection) | Same objc2 ecosystem as macOS. Provides UIColor semantic colors (~30), UIFont with Dynamic Type support, and UITraitCollection for appearance detection. |

**Required Cargo.toml features for objc2-ui-kit:**
```toml
[target.'cfg(target_os = "ios")'.dependencies]
objc2-ui-kit = { version = "0.3", optional = true, features = [
    "UIColor",
    "UIFont",
    "UITraitCollection",
] }
```

**objc2 ecosystem version compatibility:** Same as macOS -- objc2-ui-kit 0.3.2 requires objc2 >=0.6.2, <0.8.0.

**Confidence: HIGH** -- Verified on crates.io.

### Android Platform Dependencies (feature-gated)

| Library | Version | Feature Flag | Purpose | Why Recommended |
|---------|---------|-------------|---------|-----------------|
| jni | 0.22.3 | `android` | JNI bridge for calling Android Java APIs from Rust | Standard Rust JNI crate. Required to call `Resources.getColor(android.R.color.system_accent1_*)` for Material You dynamic colors (API 31+). Verbose but functional. |
| ndk | 0.9.0 | `android` | Android NDK bindings | Provides `Configuration` access for `uiMode` (dark/light detection) and font scale. |

**Confidence: MEDIUM** -- Crate versions verified on crates.io, but Android theme reading from Rust is immature. No ergonomic wrapper for Material You exists. JNI calls are verbose and error-prone. This is the weakest part of the stack. Deferred to late phase for good reason.

### Optional Utilities

| Library | Version | Feature Flag | Purpose | When to Use |
|---------|---------|-------------|---------|-------------|
| notify | 8.2.0 | `watch` | Filesystem change notifications | Watching `~/.config/kdeglobals` for live theme changes. Use 8.2.0 stable, not 9.0.0-rc.2. |

**Confidence: HIGH** -- Verified on crates.io. notify 8.x is battle-tested.

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| cargo clippy | Lint enforcement | Use `#![warn(clippy::pedantic)]` for library quality. |
| cargo doc | Documentation generation | All public types need doc comments. Use `#![warn(missing_docs)]`. |
| cargo test | Unit and integration tests | Round-trip serde tests, Rgba hex parsing edge cases, preset loading. |
| cargo deny | Dependency auditing | Check for license compatibility and known vulnerabilities. |
| cargo hack | Feature flag testing | Test all feature flag combinations compile: `cargo hack check --feature-powerset`. Essential for a crate with 7+ feature flags. |

## Cargo.toml (Verified Reference)

```toml
[package]
name = "native-theme"
version = "0.1.0"
edition = "2024"
rust-version = "1.85.0"
license = "MIT OR Apache-2.0"
description = "Toolkit-agnostic OS theme data model with presets and runtime readers"
keywords = ["theme", "native", "gui", "colors", "desktop"]
categories = ["gui", "config"]

[dependencies]
serde = { version = "1", features = ["derive"] }
toml = "1.0"

# Linux
ashpd = { version = "0.13", optional = true }
configparser = { version = "3.1", optional = true }
dirs = { version = "6", optional = true }

# Windows
[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.62", optional = true, features = [
    "UI_ViewManagement",
    "Win32_UI_WindowsAndMessaging",
] }

# macOS
[target.'cfg(target_os = "macos")'.dependencies]
objc2-app-kit = { version = "0.3", optional = true, features = [
    "NSColor", "NSFont", "NSAppearance", "NSColorSpace",
] }

# iOS
[target.'cfg(target_os = "ios")'.dependencies]
objc2-ui-kit = { version = "0.3", optional = true, features = [
    "UIColor", "UIFont", "UITraitCollection",
] }

# Android
[target.'cfg(target_os = "android")'.dependencies]
jni = { version = "0.22", optional = true }
ndk = { version = "0.9", optional = true }

# Utilities
notify = { version = "8", optional = true }

[features]
default = []
kde = ["dep:configparser", "dep:dirs"]
portal = ["dep:ashpd"]
windows = ["dep:windows"]
macos = ["dep:objc2-app-kit"]
ios = ["dep:objc2-ui-kit"]
android = ["dep:jni", "dep:ndk"]
watch = ["dep:notify"]

[dev-dependencies]
pretty_assertions = "1"
```

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| TOML parser | toml 1.0 | toml_edit 0.22 | toml_edit preserves formatting (useful for editors), but we only need serialize/deserialize -- not in-place editing. toml 1.0 is simpler. If format-preserving editing is needed later, toml_edit can be added behind a feature flag. |
| INI parser (KDE) | configparser 3.1 | rust-ini 0.21 | configparser has zero dependencies and provides `Ini::new_cs()` for case-sensitive mode out of the box. rust-ini is equally capable but slightly heavier. Either works; configparser is the lighter choice. |
| INI parser (KDE) | configparser 3.1 | light-ini | light-ini is event-based (no HashMap), better for streaming. But we need random access to sections/keys, which configparser provides directly. |
| Error handling | Manual `Error` enum | thiserror 2.0 | For a library crate with only 4-5 error variants (`Unsupported`, `Unavailable`, `Format`, `Platform`), thiserror adds a proc-macro dependency for minimal benefit. Hand-written `Display` + `Error` impl is ~30 lines and zero cost. If variant count grows past 10, reconsider thiserror. |
| Serialization format | TOML | RON | RON is Rust-native but not human-friendly outside Rust. TOML is universally readable/editable and has broader tool support. Theme files are meant to be edited by humans. |
| Serialization format | TOML | JSON | JSON lacks comments, has no native date type, and is less pleasant to edit by hand. TOML is purpose-built for configuration files. |
| macOS bindings | objc2-app-kit 0.3 | cacao 0.3.2 | cacao wraps 39 NSColor variants but is **missing `controlAccentColor`** -- the single most important color for theme reading. objc2-app-kit provides full access. |
| macOS bindings | objc2-app-kit 0.3 | cocoa crate (servo) | Deprecated in favor of objc2 ecosystem. The old `cocoa` crate from Servo uses the legacy `objc` runtime bindings. |
| Windows API | windows 0.62 | windows-sys 0.62 | windows-sys provides raw FFI bindings (faster compile times) but UISettings is a WinRT API that benefits from the high-level wrappers in `windows`. The ergonomic cost of raw WinRT from windows-sys is not worth the marginal compile-time savings. |
| Async portal | ashpd 0.13 | raw zbus 5.14 | ashpd provides typed constants (`APPEARANCE_NAMESPACE`, `COLOR_SCHEME_KEY`, `ColorScheme` enum). Writing raw zbus calls saves one dependency but loses type safety and convenience. The abstraction is worth it. |
| XDG dirs | dirs 6.0 | directories 5.0 | dirs is the lighter cousin of directories. We only need `config_dir()`, not the full `ProjectDirs` API. dirs is sufficient and has fewer dependencies. |
| File watching | notify 8.2 | inotify (direct) | notify abstracts over inotify (Linux), FSEvents (macOS), ReadDirectoryChanges (Windows). Cross-platform consistency matters even though we only use it for kdeglobals today. |
| Dev assertion | pretty_assertions 1.x | none | Provides colored diff output for `assert_eq!` failures in tests. Makes TOML round-trip test failures much easier to debug. Zero-cost (dev-dependency only). |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| `cocoa` crate (servo) | Deprecated. Uses legacy `objc` 0.2 runtime. No longer maintained. The objc2 ecosystem is the successor. | objc2-app-kit 0.3 |
| `objc` crate (0.2.x) | Legacy runtime bindings. Superseded by objc2 0.6. No new development. | objc2 0.6 (pulled in transitively by objc2-app-kit) |
| `cacao` 0.3 | Missing `controlAccentColor` (the most important color). Incomplete NSColor coverage. | objc2-app-kit 0.3 (direct, complete access) |
| `dark-light` 2.0 | Only detects dark/light boolean. No colors, no fonts, no geometry. Solves a much smaller problem than native-theme. | Build runtime readers directly with platform crates |
| `system-theme` 0.3 | Coupled to iced. Only provides dark/light + accent on desktop. Not toolkit-agnostic. | native-theme itself fills this gap |
| `cosmic-theme` | Tightly coupled to COSMIC desktop / iced / libcosmic. Not on crates.io independently. Not toolkit-agnostic. Uses RON format. | native-theme itself fills this gap |
| `winapi` crate | Superseded by the official `windows` crate from Microsoft. No longer the recommended approach. | windows 0.62 |
| `winrt` crate | Superseded by the official `windows` crate. | windows 0.62 |
| `toml` 0.8.x | Superseded by toml 1.0.x (just released March 2026). Same API, but 1.0 signals stability and supports TOML spec 1.1.0. | toml 1.0 |
| `anyhow` | For binary applications, not libraries. Library crates should expose typed errors so consumers can match on variants. | Manual `Error` enum |
| `thiserror` 1.0.x | If you use thiserror at all, use 2.0. But for this crate with 4-5 variants, manual impl is preferred (see Alternatives). | Manual `Error` enum (or thiserror 2.0 if needed) |

## Stack Patterns by Variant

**If building core-only (no platform readers):**
- Dependencies: serde + toml only
- Use case: Loading preset TOML files, theme layering, serialization
- Compile time: Fast (~5s incremental)

**If building with KDE reader only:**
- Add: configparser + dirs (feature `kde`)
- Fully synchronous -- no async runtime needed
- Use case: KDE/Plasma desktop apps that read kdeglobals

**If building with portal reader:**
- Add: ashpd (feature `portal`)
- Pulls in: tokio (default) or async-io runtime
- Use case: GNOME/KDE Plasma 6 apps wanting live accent color and color scheme
- The consumer MUST provide an async runtime (tokio is the default path)

**If building for Windows:**
- Add: windows crate with `UI_ViewManagement` + `Win32_UI_WindowsAndMessaging`
- Fully synchronous (WinRT UISettings works from any thread)
- Use case: Windows desktop apps reading accent colors and system metrics

**If building for macOS:**
- Add: objc2-app-kit with NSColor/NSFont/NSAppearance/NSColorSpace features
- Requires careful thread handling: resolve dynamic colors on main thread, then data is Send+Sync
- Use case: macOS apps reading system colors and fonts

**If building for iOS:**
- Add: objc2-ui-kit with UIColor/UIFont/UITraitCollection features
- Same objc2 ecosystem as macOS -- shared knowledge
- Use case: iOS apps reading semantic colors and Dynamic Type

**If building for Android:**
- Add: jni + ndk
- Requires JNI context (AndroidApp or similar)
- Most verbose and least ergonomic platform -- expect 3-5x more code than other platforms
- Use case: Android apps wanting Material You dynamic colors (API 31+)

## Version Compatibility

| Package | Compatible With | Notes |
|---------|-----------------|-------|
| serde 1.0.228 | toml 1.0.6 | toml depends on serde ^1.0.145. No conflicts. |
| objc2-app-kit 0.3.2 | objc2 >=0.6.2, <0.8.0 | objc2 0.6.4 is current latest -- within range. |
| objc2-ui-kit 0.3.2 | objc2 >=0.6.2, <0.8.0 | Same objc2 range as app-kit. Co-resolves cleanly (not that both would ever be used in same binary). |
| ashpd 0.13.4 | zbus ^5.13 | zbus 5.14.0 is current latest -- compatible. |
| ashpd 0.13.4 | tokio ^1.43 | tokio is at 1.43+ -- compatible. |
| configparser 3.1.0 | (standalone) | Zero external dependencies. No compatibility concerns. |
| dirs 6.0.0 | (standalone) | Minimal deps (dirs-sys). No compatibility concerns. |
| windows 0.62.2 | Rust 1.61+ | Well within our MSRV of 1.85. |
| jni 0.22.3 | ndk 0.9.0 | Both work with standard Android NDK toolchain. |
| notify 8.2.0 | (standalone) | Cross-platform. inotify/FSEvents/ReadDirectoryChanges backends. |

## Key Correction from IMPLEMENTATION.md

The IMPLEMENTATION.md (Section 7.2) states that ashpd supports `async-std` via `default-features = false, features = ["async-std"]`. This is **incorrect for ashpd 0.13.x**. The alternative to tokio is `async-io` (not `async-std`). The correct incantation is:

```toml
# For tokio (default):
ashpd = { version = "0.13", optional = true }

# For async-io (smol/glib-compatible):
ashpd = { version = "0.13", optional = true, default-features = false, features = ["async-io"] }
```

This should be corrected in the implementation spec before coding begins.

## Sources

- [crates.io/serde](https://crates.io/crates/serde) -- version 1.0.228 verified (HIGH confidence)
- [crates.io/toml](https://crates.io/crates/toml) -- version 1.0.6 verified, TOML spec 1.1.0 (HIGH confidence)
- [crates.io/ashpd](https://crates.io/crates/ashpd) -- version 0.13.4 verified, tokio default, async-io alternative (HIGH confidence)
- [docs.rs/ashpd/0.13.4](https://docs.rs/ashpd/latest/ashpd/) -- feature flags verified: tokio (default) and async-io, no async-std (HIGH confidence)
- [crates.io/configparser](https://crates.io/crates/configparser) -- version 3.1.0 verified (HIGH confidence)
- [crates.io/dirs](https://crates.io/crates/dirs) -- version 6.0.0 verified (HIGH confidence)
- [crates.io/windows](https://crates.io/crates/windows) -- version 0.62.2 verified (HIGH confidence)
- [microsoft.github.io/windows-docs-rs UISettings](https://microsoft.github.io/windows-docs-rs/doc/windows/UI/ViewManagement/struct.UISettings.html) -- API surface verified (HIGH confidence)
- [microsoft.github.io/windows-docs-rs GetSystemMetrics](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.GetSystemMetrics.html) -- feature flag verified (HIGH confidence)
- [crates.io/objc2-app-kit](https://crates.io/crates/objc2-app-kit) -- version 0.3.2 verified, requires objc2 >=0.6.2 (HIGH confidence)
- [crates.io/objc2-ui-kit](https://crates.io/crates/objc2-ui-kit) -- version 0.3.2 verified, requires objc2 >=0.6.2 (HIGH confidence)
- [crates.io/objc2](https://crates.io/crates/objc2) -- version 0.6.4 verified (HIGH confidence)
- [crates.io/jni](https://crates.io/crates/jni) -- version 0.22.3 verified (HIGH confidence)
- [crates.io/ndk](https://crates.io/crates/ndk) -- version 0.9.0 verified (HIGH confidence)
- [crates.io/notify](https://crates.io/crates/notify) -- version 8.2.0 stable, 9.0.0-rc.2 pre-release (HIGH confidence)
- [crates.io/thiserror](https://crates.io/crates/thiserror) -- version 2.0.18 verified; not recommended for this crate (HIGH confidence)
- [crates.io/dark-light](https://crates.io/crates/dark-light) -- version 2.0.0 verified; insufficient scope (HIGH confidence)
- [crates.io/system-theme](https://crates.io/crates/system-theme) -- version 0.3.0 verified; iced-coupled (HIGH confidence)
- [Rust 1.85.0 announcement](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/) -- Edition 2024 stabilization confirmed (HIGH confidence)
- [crates.io/material-color-utilities](https://crates.io/crates/material-color-utilities) -- version 1.0.0-dev.18; pre-release, not recommended for production use (MEDIUM confidence)

---
*Stack research for: native-theme (cross-platform OS theme data crate)*
*Researched: 2026-03-07*
