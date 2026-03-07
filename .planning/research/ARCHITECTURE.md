# Architecture Research

**Domain:** Cross-platform Rust crate with feature-gated platform backends (theme data)
**Researched:** 2026-03-07
**Confidence:** HIGH

## Standard Architecture

### System Overview

```
                         Consumer (egui / iced / gpui / slint / ...)
                                        |
                                   [~50-line adapter]
                                        |
                           +============+============+
                           |       Public API        |
                           |  NativeTheme::preset()  |
                           |  NativeTheme::from_*()  |
                           |  NativeTheme::merge()   |
                           +============+============+
                                        |
             +----------+----------+---------+----------+
             |          |          |         |          |
        +----+----+ +---+---+ +---+---+ +---+---+ +---+---+
        |  model  | |presets| |  kde  | |portal | |windows|  ...
        |         | |       | | (sync)| |(async)| | (sync)|
        +---------+ +-------+ +---+---+ +---+---+ +---+---+
                                   |         |         |
                          +--------+---------+---------+-------+
                          |           Platform APIs            |
                          | kdeglobals | D-Bus | UISettings    |
                          | NSColor    | UIColor | JNI         |
                          +------------------------------------+
```

### Layered Architecture

The crate follows a **data-model-core + optional-readers** layered architecture, the dominant pattern for cross-platform Rust crates. This is the same structural approach used by `notify` (core Watcher trait + platform backends like inotify/fsevent/kqueue), `arboard` (Clipboard + platform modules in src/platform/), and `keyring` (Entry + platform credential stores). The key insight: the core data model and serialization have zero platform dependencies; platform reading is purely additive via feature flags.

**Three layers, strictly one-directional:**

1. **Core Layer** (always compiled): Data model types, TOML serde, error types, merge logic. Dependencies: `serde` + `toml` only.
2. **Preset Layer** (always compiled): Embedded TOML files via `include_str!()`, preset loading API. Dependencies: none beyond core.
3. **Platform Layer** (feature-gated, optional): Each reader is an independent module that populates a `NativeTheme` from OS APIs. Each reader depends on its own platform crate. No reader depends on another reader.

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| `model/` | Define the canonical data types: `NativeTheme`, `ThemeVariant`, `ThemeColors`, `ThemeFonts`, `ThemeGeometry`, `ThemeSpacing`, `Rgba` | Plain `#[derive(Serialize, Deserialize)]` structs with `Option<T>` fields, `#[non_exhaustive]`, `Default` impls, `merge()` methods, custom serde for `Rgba` hex |
| `presets/` | Embed and serve bundled theme files; provide TOML parse/serialize API | `include_str!()` for TOML files, `preset()` does `match name` + `toml::from_str()`, `from_file()` reads disk, `to_toml()` serializes |
| `error.rs` | Unified error type across all layers | `enum Error { Unsupported, Unavailable(String), Format(String), Platform(Box<dyn Error + Send + Sync>) }` with `From` impls for `toml::de::Error`, `io::Error` |
| `platform/kde.rs` | Read `~/.config/kdeglobals` INI file synchronously | `configparser::Ini`, parse `[Colors:*]` sections, Qt font format, hardcoded Breeze geometry |
| `platform/portal.rs` | Read freedesktop portal settings asynchronously | `ashpd::desktop::settings::Settings`, accent/scheme/contrast, overlay on Adwaita defaults |
| `platform/windows.rs` | Read Windows UISettings + system metrics synchronously | `windows` crate, `UISettings::GetColorValue`, `GetSystemMetrics` |
| `platform/macos.rs` | Read macOS NSColor + NSFont synchronously | `objc2-app-kit`, ~20 semantic NSColor properties, appearance resolution, P3-to-sRGB conversion |
| `platform/mod.rs` | Cross-platform dispatch via `from_system()` and DE detection | `#[cfg(target_os)]` dispatch, `XDG_CURRENT_DESKTOP` heuristic on Linux |
| `lib.rs` | Public API surface, re-exports, module declarations with `#[cfg(feature)]` gates | Flat re-exports of model types; conditional `pub mod platform` |

## Recommended Project Structure

```
native-theme/
  Cargo.toml                    # Package metadata, feature flags, platform deps
  src/
    lib.rs                      # Public API re-exports, conditional module declarations
    error.rs                    # Error enum (always compiled)
    model/
      mod.rs                    # NativeTheme, ThemeVariant, merge() impls
      colors.rs                 # ThemeColors (36 fields), Rgba with custom hex serde
      fonts.rs                  # ThemeFonts
      geometry.rs               # ThemeGeometry
      spacing.rs                # ThemeSpacing
    presets/
      mod.rs                    # preset(), list_presets(), from_toml(), from_file(), to_toml()
      default.toml              # Neutral defaults
      kde-breeze.toml           # Plasma 6 Breeze light + dark
      adwaita.toml              # GNOME Adwaita light + dark
      windows-11.toml           # (Phase 5)
      macos-sonoma.toml         # (Phase 5)
      material.toml             # (Phase 5)
      ios.toml                  # (Phase 5)
    platform/                   # Feature-gated (Phase 3+)
      mod.rs                    # from_system(), cfg-dispatched platform detection
      kde.rs                    # feature "kde" -- sync, configparser + dirs
      portal.rs                 # feature "portal" -- async, ashpd
      windows.rs                # feature "windows" -- sync, windows crate
      macos.rs                  # feature "macos" -- sync, objc2-app-kit
      ios.rs                    # feature "ios" -- deferred
      android.rs                # feature "android" -- deferred
  tests/
    serde_roundtrip.rs          # TOML round-trip for all model types
    preset_loading.rs           # Load each preset, validate fields
    rgba_parsing.rs             # Hex parsing edge cases
```

### Structure Rationale

- **`model/` as separate module tree:** The data model is the stable core. Isolating it from presets and platform code makes the dependency boundary visible: model has no deps beyond serde/toml. This follows the `notify` pattern where the event types are separate from backend implementations.
- **`presets/` alongside TOML files:** TOML files live next to their Rust loader because `include_str!()` uses relative paths. Keeping loader + data co-located prevents path breakage.
- **`platform/` as a flat module tree (not nested per-OS):** Each file is a self-contained reader. No shared abstractions between platforms (unlike `notify` which has a `Watcher` trait). Platform readers share a return type (`Result<NativeTheme, Error>`) but not a trait -- the `#[cfg]`-dispatched `from_system()` calls platform functions directly. This avoids a needless trait abstraction: there is exactly one reader per platform, never interchangeable at runtime.
- **No `mod platform_impl` or deeply nested backend:** The `arboard` pattern of `src/platform/{osx,windows,linux}.rs` is clean for this use case. Each file compiles independently under its feature flag. Deeply nested structures add complexity with no benefit for a crate with ~6 backends.

## Architectural Patterns

### Pattern 1: Feature-Gated Conditional Modules

**What:** Each platform reader lives in its own module, gated by both `#[cfg(feature = "...")]` and sometimes `#[cfg(target_os = "...")]`. The module is only compiled when its feature flag is enabled.

**When to use:** Always for platform-specific code that pulls in platform-specific dependencies.

**Trade-offs:** Additive features are simple and safe per Cargo's feature unification rules. The downside: CI must test each feature flag combination (at minimum: no features, each feature individually, all features). With 6 platform features, this is manageable (not 2^N -- many are mutually exclusive by target OS).

**Example:**
```rust
// src/lib.rs
pub mod model;
pub mod presets;
pub mod error;

#[cfg(any(
    feature = "kde",
    feature = "portal",
    feature = "windows",
    feature = "macos",
    feature = "ios",
    feature = "android",
))]
pub mod platform;

// Re-export core types at crate root for ergonomic access
pub use model::{NativeTheme, ThemeVariant, ThemeColors, ThemeFonts,
                ThemeGeometry, ThemeSpacing, Rgba};
pub use error::Error;
```

```rust
// src/platform/mod.rs
#[cfg(feature = "kde")]
mod kde;
#[cfg(feature = "kde")]
pub use kde::from_kde;

#[cfg(feature = "portal")]
mod portal;
#[cfg(feature = "portal")]
pub use portal::from_gnome;

#[cfg(feature = "windows")]
mod windows;
#[cfg(feature = "windows")]
pub use self::windows::from_windows;

#[cfg(feature = "macos")]
mod macos;
#[cfg(feature = "macos")]
pub use macos::from_macos;

// from_system() is always available when any platform feature is enabled
pub fn from_system() -> Result<crate::NativeTheme, crate::Error> {
    #[cfg(target_os = "macos")]
    { return from_macos(); }

    #[cfg(target_os = "windows")]
    { return from_windows(); }

    #[cfg(target_os = "linux")]
    { return from_linux(); }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    Err(crate::Error::Unsupported)
}
```

### Pattern 2: Option-All-Fields Data Model

**What:** Every field in every model struct is `Option<T>`. No mandatory fields except `name: String` on `NativeTheme`. All structs derive `Default` (producing all-`None`). Combined with `#[serde(default)]` and `#[serde(skip_serializing_if = "Option::is_none")]`.

**When to use:** When the data model represents a union of platform capabilities where no single platform fills all fields, and when layering/merging of partial data is a core use case.

**Trade-offs:** Consumers must handle `None` everywhere (unwrap_or pattern). But the alternative -- fabricating values for fields a platform does not expose -- produces worse outcomes (wrong colors, misleading data). This pattern is validated by `cosmic-theme` (RON with optional fields) and is standard for configuration-like data.

**Example:**
```rust
// Merging: overlay.field replaces base.field only when Some
impl ThemeColors {
    pub fn merge(&mut self, overlay: &ThemeColors) {
        if overlay.accent.is_some() { self.accent = overlay.accent; }
        if overlay.background.is_some() { self.background = overlay.background; }
        // ... for all 36 fields
    }
}
```

### Pattern 3: Embedded Static Data (Presets)

**What:** TOML preset files are compiled into the binary via `include_str!()`. Parsed lazily on first access. No file I/O at runtime for bundled presets.

**When to use:** When shipping reference data that is pure configuration (no code), updated infrequently, and must work without filesystem access (e.g., sandboxed apps, WASM).

**Trade-offs:** Binary size grows (~2-4 KB per TOML preset -- negligible). Presets cannot be updated without recompiling. This is acceptable: presets represent specific OS theme snapshots that change only when OS design languages evolve (Breeze, Adwaita updates are yearly at most).

**Example:**
```rust
// src/presets/mod.rs
const DEFAULT_TOML: &str = include_str!("default.toml");
const KDE_BREEZE_TOML: &str = include_str!("kde-breeze.toml");
const ADWAITA_TOML: &str = include_str!("adwaita.toml");

impl NativeTheme {
    pub fn preset(name: &str) -> Option<NativeTheme> {
        let toml_str = match name {
            "default" => DEFAULT_TOML,
            "kde-breeze" => KDE_BREEZE_TOML,
            "adwaita" => ADWAITA_TOML,
            _ => return None,
        };
        toml::from_str(toml_str).ok()
    }

    pub fn list_presets() -> &'static [&'static str] {
        &["default", "kde-breeze", "adwaita"]
    }
}
```

### Pattern 4: Sync/Async Feature Separation

**What:** The `kde` feature is fully synchronous (file parsing). The `portal` feature is async (D-Bus via ashpd/zbus). They are separate features, not bundled. `from_system()` is always sync and falls back to presets when async readers are unavailable.

**When to use:** When some platform APIs are inherently async (D-Bus) but others are sync (file I/O, COM, ObjC), and you do not want to force an async runtime on users who only need sync access.

**Trade-offs:** Users who want both KDE and portal data must enable both features. The `from_system()` sync dispatcher cannot call `from_gnome()` (async). This is by design: GNOME users who want live portal data call `from_gnome().await` directly. The sync fallback loads the bundled Adwaita preset.

## Data Flow

### Preset Loading Flow (Phase 1 -- no platform deps)

```
User code: NativeTheme::preset("kde-breeze")
    |
    v
presets/mod.rs: match "kde-breeze" -> include_str!("kde-breeze.toml")
    |
    v
toml::from_str::<NativeTheme>(toml_str)
    |
    v
serde: deserialize name, light.{colors,fonts,geometry,spacing},
       dark.{colors,fonts,geometry,spacing}
       Rgba fields: custom deserializer parses "#rrggbb" / "#rrggbbaa"
    |
    v
NativeTheme { name: "KDE Breeze", light: Some(...), dark: Some(...) }
    |
    v
User picks variant: let theme = if is_dark { nt.dark } else { nt.light }
    |
    v
Adapter maps ThemeVariant fields -> toolkit types (~50 lines)
```

### Runtime Reader Flow (Phase 3+ -- platform features enabled)

```
User code: native_theme::platform::from_system()
    |
    v
platform/mod.rs: #[cfg(target_os = "linux")] -> from_linux()
    |
    v
from_linux():
  1. Check XDG_CURRENT_DESKTOP for "KDE"
     -> YES: from_kde()
        |
        v
     kde.rs:
       a. dirs::config_dir() -> ~/.config/
       b. Ini::new_cs().load("kdeglobals")
       c. Parse [Colors:Window], [Colors:View], ..., [General] font
       d. Map to ThemeColors, ThemeFonts
       e. Hardcoded Breeze geometry + spacing
       f. Determine light/dark from background luminance
       g. Return NativeTheme { name: "KDE", light/dark: Some(variant) }

     -> NO: Load preset("adwaita") as fallback
    |
    v
NativeTheme (populated from live OS data)
```

### Theme Layering Flow

```
base = NativeTheme::preset("kde-breeze")      // full theme
overlay = NativeTheme::from_file("custom.toml") // partial overrides

base.light.merge(&overlay.light)
// For each field in overlay:
//   Some(value) -> replaces base field
//   None        -> base field unchanged

Result: base with user customizations applied
```

### Key Data Flows

1. **Preset load:** `preset(name)` -> `include_str!()` -> `toml::from_str()` -> `NativeTheme`. Pure, no side effects.
2. **File load:** `from_file(path)` -> `fs::read_to_string()` -> `toml::from_str()` -> `NativeTheme`. I/O, may fail.
3. **Platform read:** `from_kde()` / `from_macos()` / `from_windows()` -> OS APIs -> populate `ThemeVariant` -> `NativeTheme`. Platform-specific, may fail with `Error::Unavailable` or `Error::Platform`.
4. **Merge/overlay:** `variant.merge(&overlay)` -> field-by-field `Option` replacement. In-place mutation. Always succeeds.
5. **Serialize:** `to_toml()` -> `toml::to_string()`. `skip_serializing_if = "Option::is_none"` produces clean output.

## Build Order (Dependency Graph for Phased Implementation)

The layers have strict one-way dependencies. This determines the optimal build order:

```
Phase 1 (foundation -- no platform deps):
  error.rs           <- standalone
  model/colors.rs    <- depends on serde (Rgba custom serde)
  model/fonts.rs     <- depends on serde
  model/geometry.rs  <- depends on serde
  model/spacing.rs   <- depends on serde
  model/mod.rs       <- depends on all model/* (NativeTheme, ThemeVariant, merge)
  presets/mod.rs     <- depends on model + toml (preset loading API)
  presets/*.toml     <- pure data, depends on model schema
  lib.rs             <- re-exports model + presets + error

Phase 3 (Linux readers -- adds platform deps):
  platform/kde.rs    <- depends on model, error, configparser, dirs
  platform/portal.rs <- depends on model, error, ashpd (async)
  platform/mod.rs    <- depends on platform readers, model, error

Phase 4 (desktop readers):
  platform/windows.rs <- depends on model, error, windows crate
  platform/macos.rs   <- depends on model, error, objc2-app-kit

Phase 7 (mobile readers -- deferred):
  platform/ios.rs     <- depends on model, error, objc2-ui-kit
  platform/android.rs <- depends on model, error, jni, ndk
```

**Why this order matters for the roadmap:**
- `error.rs` and `model/` are the foundation. Everything depends on them. Build first.
- `model/colors.rs` (with `Rgba` custom serde) is the hardest part of the model -- it has the most fields (36) and the custom hex serializer. Start here within the model.
- `presets/` depends on the model being stable. Build presets after model is tested.
- Platform readers are independent of each other. They can be built in any order, but each depends on the model being stable.
- `from_system()` dispatch depends on individual readers existing.

## Anti-Patterns

### Anti-Pattern 1: Trait-Based Backend Abstraction

**What people do:** Define a `trait ThemeReader { fn read() -> NativeTheme; }` and implement it for each platform.

**Why it's wrong:** There is exactly one reader per platform, never selected at runtime. The trait adds a vtable indirection and forces a shared method signature when platform readers have legitimately different signatures (`from_gnome()` is async, `from_kde()` is sync). `from_system()` dispatches via `#[cfg]` at compile time, not via dynamic dispatch. The `notify` crate does use a `Watcher` trait, but that is because it supports runtime backend selection (polling vs. native) -- this crate does not.

**Do this instead:** Free functions (`from_kde()`, `from_macos()`, etc.) behind `#[cfg(feature)]` gates. `from_system()` does compile-time dispatch via `#[cfg(target_os)]` blocks.

### Anti-Pattern 2: Feature-Gated Struct Fields

**What people do:** Put `#[cfg(feature = "kde")]` on struct fields or add platform-specific fields to the model.

**Why it's wrong:** The data model must be platform-independent. Adding `kde_complementary_bg: Option<Rgba>` behind a feature flag breaks serde compatibility (TOML files become feature-dependent) and violates Cargo's feature additivity rule (enabling `kde` changes struct layout).

**Do this instead:** The model has a fixed set of 36 semantic color roles that cover the union of all platforms. Platform-specific values that do not map to a semantic role are simply not included. The preset and reader produce the same `NativeTheme` type regardless of which features are enabled.

### Anti-Pattern 3: Forcing Async on Sync Users

**What people do:** Make all platform readers async, pulling in tokio even for synchronous file-parsing operations.

**Why it's wrong:** `system-theme`'s hard `tokio` dependency is its most criticized design choice. Users loading a preset or reading `~/.config/kdeglobals` do not need an async runtime. Forcing one bloats compile times and binary size.

**Do this instead:** Separate `kde` (sync) from `portal` (async). `from_system()` is sync and falls back to presets for GNOME. Users wanting async portal data call `from_gnome().await` explicitly.

### Anti-Pattern 4: Monolithic Platform Crate Dependency

**What people do:** Depend on a large platform crate with all features enabled when only a few APIs are needed.

**Why it's wrong:** The `windows` crate has hundreds of feature flags. Enabling all of them massively increases compile time. Similarly, `objc2-app-kit` has per-class features.

**Do this instead:** Enable only the specific features needed. For `windows`: `UI_ViewManagement` + `Win32_UI_WindowsAndMessaging`. For `objc2-app-kit`: `NSColor`, `NSFont`, `NSAppearance`, `NSColorSpace`. This is already correctly specified in the implementation spec.

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| KDE kdeglobals | File I/O (`configparser` INI parser) | Case-sensitive mode required (`Ini::new_cs()`); file path via `dirs::config_dir()` |
| Freedesktop Portal | D-Bus via `ashpd` (async) | Returns `(f64, f64, f64)` accent; clamp and scale to u8; portal may not be running |
| Windows UISettings | COM API via `windows` crate | Check `ApiInformation::IsMethodPresent` before calling; handles graceful degradation |
| macOS NSColor | ObjC via `objc2-app-kit` | P3-to-sRGB conversion required; appearance resolution before component extraction |
| iOS UIColor | ObjC via `objc2-ui-kit` | Similar to macOS; Dynamic Type for font sizes |
| Android Resources | JNI via `jni` + `ndk` | Requires Android JNI context; Material You API 31+ only; most immature |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| model <-> presets | Direct struct construction via `toml::from_str()` | Presets produce `NativeTheme` directly; no intermediate format |
| model <-> platform readers | Direct struct construction | Readers construct `NativeTheme`/`ThemeVariant` directly, setting `Some(value)` for available data |
| model <-> error | `Result<NativeTheme, Error>` | Platform readers return `Result`; preset loading returns `Option` (not `Result`, since preset names are static) |
| platform/mod.rs <-> individual readers | Direct function calls behind `#[cfg]` | No trait, no dynamic dispatch; compile-time selection |
| crate <-> consumer | `NativeTheme` struct + adapter pattern | Consumer writes ~50-line adapter mapping fields to toolkit types; crate exposes no toolkit types |

## Sources

- [Effective Rust - Item 26: Be wary of feature creep](https://effective-rust.com/features.html) -- guidance on feature flag pitfalls (HIGH confidence)
- [Cargo Book - Features](https://doc.rust-lang.org/cargo/reference/features.html) -- canonical reference for feature flag mechanics (HIGH confidence)
- [Rust Reference - Conditional Compilation](https://doc.rust-lang.org/reference/conditional-compilation.html) -- `#[cfg]` attribute semantics (HIGH confidence)
- [notify-rs/notify](https://github.com/notify-rs/notify) -- cross-platform file watcher; exemplar of platform backend pattern with `Watcher` trait + inotify/fsevent/kqueue/windows backends (HIGH confidence)
- [1Password/arboard](https://github.com/1Password/arboard) -- cross-platform clipboard; `src/platform/{osx,windows,linux}.rs` flat module pattern (HIGH confidence)
- [keyring-rs](https://github.com/open-source-cooperative/keyring-rs) -- cross-platform credential store; feature-gated per-platform keystores (HIGH confidence)
- [system-theme 0.3.0](https://crates.io/crates/system-theme) -- closest prior art; validates error taxonomy, identifies forced-async anti-pattern (HIGH confidence)
- [COSMIC cosmic-theme](https://github.com/pop-os/libcosmic/tree/master/cosmic-theme) -- validates Option-field data model for theme crates (MEDIUM confidence)
- [Sling Academy - Handling Platform-Specific Code](https://www.slingacademy.com/article/handling-platform-specific-code-with-cfg-attributes-in-rust-modules/) -- tutorial on cfg module organization (MEDIUM confidence)
- [docs/IMPLEMENTATION.md](../docs/IMPLEMENTATION.md) -- project implementation specification (HIGH confidence)

---
*Architecture research for: native-theme cross-platform Rust crate*
*Researched: 2026-03-07*
