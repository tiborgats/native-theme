# Feature Research

**Domain:** Toolkit-agnostic cross-platform OS theme data crate (Rust)
**Researched:** 2026-03-07
**Confidence:** HIGH (based on analysis of prior art crates, platform API docs, downstream toolkit APIs, W3C design token spec, and Electron nativeTheme precedent)

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Semantic color data model (36+ roles) | Every theme system defines named color roles (accent, background, foreground, error, warning, success). Electron's `systemPreferences` exposes ~20, iced's `Extended` palette has 7 role groups, egui's `Visuals` has ~15 color fields, COSMIC's theme covers dozens. Without sufficient semantic coverage, adapters produce "tinted but still generic" results (as seen in `system-theme`'s 6-color palette). | MEDIUM | 36 fields is the sweet spot -- covers the union of platform capabilities without over-specializing. All fields `Option<T>` since no platform fills all roles. The macro-generated struct pattern (from PITFALLS.md) keeps merge() synchronized. |
| Light and dark variant support | Every OS supports dark/light mode. Every toolkit has `dark()` / `light()` presets. A theme crate that only stores one variant is incomplete. Electron nativeTheme is built around this duality. COSMIC, catppuccin (4 flavors), iced (Theme enum) all expect this. | LOW | `NativeTheme { light: Option<ThemeVariant>, dark: Option<ThemeVariant> }` -- straightforward. Presets define both; runtime readers populate whichever matches current OS mode. |
| TOML serialization / deserialization | The core value proposition. `system-theme` and catppuccin provide `const` Rust structs -- no serialization, no user-editable themes. `cosmic-theme` uses RON (Rust-specific, not human-friendly outside Rust). TOML is the standard for Rust config and is universally readable/editable. Without serde round-tripping, presets are compile-time-only data and community themes become impossible. | MEDIUM | Custom serde for `Rgba` hex strings (`#rrggbb` / `#rrggbbaa`). All nested structs need `#[serde(default)]` + `skip_serializing_if = "Option::is_none"`. Round-trip testing is critical. |
| Bundled preset themes | Users need a working theme immediately without enabling platform features or having a specific OS. catppuccin bundles 4 flavors; iced has ~27 built-in themes; COSMIC ships complete Adwaita-like defaults. A theme crate with no built-in themes requires users to write TOML from scratch. | MEDIUM | MVP needs at least 3: `default` (neutral fallback), `kde-breeze`, `adwaita`. Each preset is ~2-4KB TOML embedded via `include_str!()`. Values extracted from authoritative design specs (breezemetrics.h, Adwaita CSS variables, HIG). |
| Font data (family, size, monospace) | Every desktop environment exposes UI font settings. egui's `Style` has `text_styles` with `FontId` (family + size). iced has font configuration. KDE exposes 4 font properties dynamically. Ignoring fonts means theme-aware apps still use hardcoded fonts. | LOW | `ThemeFonts { family, size, monospace_family, monospace_size }` -- all `Option<String>` / `Option<f32>`. Simple structure. KDE font string parsing has edge cases (Qt 4 vs Qt 6 format) but the data model itself is trivial. |
| Geometry data (border radius, border width) | Every toolkit has configurable corner radii. egui: `window_corner_radius`, `menu_corner_radius`. iced: corner radius per widget. Slint: border-radius property. Material 3 defines 5 corner radius tiers. Without geometry, theme-aware apps still have hardcoded corners and borders. | LOW | `ThemeGeometry { border_radius, border_radius_large, border_width, ... }` -- all `Option<f32>` in logical pixels. Values are editorial (extracted from design specs), not dynamic from APIs. |
| Spacing data (default spacing, margins) | egui has a full `Spacing` struct. iced has padding/margin. KDE Breeze defines layout spacing constants. Material 3 uses an 8dp grid. Without spacing, visual integration is incomplete -- colors match but layout feels "off". | LOW | `ThemeSpacing { default_spacing, top_level_margin, child_margin, compact_spacing }` -- all `Option<f32>`. Values come from design specs, not APIs. |
| Theme merging / layering | Users need to overlay customizations on base themes (e.g., change accent color but keep everything else from Breeze). Theme UI explicitly documents merging. WordPress theme.json has inheritance. COSMIC supports theme hierarchy. Android uses theme overlays. Without merge, any customization requires a complete TOML file with all 36+ color fields. | MEDIUM | Field-by-field `Option` merge: `overlay.field.is_some()` replaces base. The macro-generated struct approach (from PITFALLS.md) guarantees merge covers all fields. This is a critical quality-of-life feature. |
| Hex color representation with alpha | macOS `NSColor.shadowColor`, GNOME `--border-opacity`, Material 3 disabled states (38% opacity) all use alpha. `system-theme`'s RGB-only `ThemeColor` is a documented limitation. CSS uses `#RRGGBBAA`. Starting without alpha means a painful retrofit later. | LOW | `Rgba { r: u8, g: u8, b: u8, a: u8 }` with `#RRGGBB` (alpha 0xFF implied) and `#RRGGBBAA` serde. `FromStr` + `Display` impls for non-serde contexts. |
| Load theme from file path | Users need to load custom TOML theme files from disk (not just bundled presets). Without `from_file()`, the only way to use custom themes is embedding TOML strings in code. | LOW | `fs::read_to_string()` + `toml::from_str()`. Simple wrapper, but essential for user workflows. |
| Typed error handling | Library crates must expose typed errors consumers can match on. `anyhow` is for binaries. `system-theme` has a good error taxonomy (`Unsupported`, `Unavailable`, `Platform`). Without typed errors, consumers cannot distinguish "platform not supported" from "file not found". | LOW | Manual `enum Error { Unsupported, Unavailable(String), Format(String), Platform(Box<dyn Error + Send + Sync>) }` with `Display` + `std::error::Error` impls. ~30 lines, zero additional deps. |
| `#[non_exhaustive]` on all public structs | Adding fields (new color roles, new geometry properties) must be non-breaking. Downstream consumers already handle `None` for every field. Without `#[non_exhaustive]`, any new field is a semver-major change that forces ecosystem-wide version bumps. | LOW | Applied to all model structs. Prevents exhaustive pattern matches. Constructors via `Default` + serde. |
| `Send + Sync` guarantee on all types | Theme data is loaded once and shared across threads in GUI apps. egui uses `Arc<Style>`. iced clones themes. Multi-threaded renderers need `Send + Sync` data. Without this guarantee, theme data cannot be safely shared. | LOW | All types are plain data (no `Rc`, no raw pointers, no interior mutability). `Send + Sync` holds automatically. Worth adding `static_assertions` or a compile-test to verify. |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valuable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Runtime OS theme reading (feature-gated per platform) | No existing toolkit-agnostic crate reads live OS theme data into a common format. `dark-light` detects only dark/light boolean. `system-theme` reads 6 colors but is iced-coupled. This crate reads 36 semantic colors + fonts + geometry from KDE, GNOME portal, Windows, macOS -- populating the same `NativeTheme` struct. This is the primary differentiator. | HIGH | Each platform reader is an independent feature-gated module: `kde` (sync, configparser+dirs), `portal` (async, ashpd), `windows` (sync, windows crate), `macos` (sync, objc2-app-kit). Each has unique API surface, error handling, and color space conversion challenges. |
| Toolkit-agnostic design (zero GUI deps) | catppuccin depends on bevy/iced/ratatui behind features. `system-theme` depends on iced. `cosmic-theme` depends on iced/libcosmic. This crate has zero toolkit deps -- works equally with egui, iced, gpui, slint, dioxus, tauri, or any future toolkit. The adapter pattern (~50 lines per toolkit) keeps coupling in userland. | LOW | This is an architectural constraint, not a feature to build. Zero additional implementation work -- just discipline about not adding toolkit deps. The value is enormous: one crate serves the entire Rust GUI ecosystem. |
| Cross-platform `from_system()` dispatch | A single function call that auto-detects the platform and desktop environment, then reads the appropriate theme source. No other crate does this with 36 semantic color roles. The consumer writes `native_theme::platform::from_system()` and gets a populated `NativeTheme` regardless of OS. | MEDIUM | `#[cfg(target_os)]` compile-time dispatch + runtime DE detection on Linux (`XDG_CURRENT_DESKTOP`, D-Bus portal backend sniffing). Sync fallback to bundled presets when async portal is unavailable. |
| Community-contributable TOML presets | Anyone can contribute a preset (Catppuccin, Dracula, Nord, Solarized, etc.) without writing Rust code -- just TOML. catppuccin has its own crate; integrating its colors into native-theme is a TOML file. COSMIC uses RON (Rust-only). JSON lacks comments. TOML is the ideal format for human-authored theme data. | LOW | Pure data contribution. CI validates TOML against schema (round-trip parse test). No Rust code review needed for pure preset additions. This enables a theme repository ecosystem. |
| Partial TOML overrides (sparse themes) | A user can write a 3-line TOML file with just `accent = "#ff0000"` and merge it onto any base preset. WordPress theme.json supports this via inheritance. Theme UI supports deep merging. Most theme systems require complete theme definitions -- partial overrides are a significant usability improvement. | LOW | Enabled by `Option<T>` fields + `#[serde(default)]` on all structs + `merge()`. The implementation is already in the data model design. The value comes from documentation and examples making this workflow discoverable. |
| Disabled opacity / border opacity | Material 3 uses 0.38 opacity for disabled states. GNOME Adwaita defines `--disabled-opacity` and `--border-opacity`. egui has `disabled_alpha`. These values are needed for visually correct disabled UI elements but are missing from simpler theme crates. | LOW | `ThemeGeometry { disabled_opacity, border_opacity }` -- `Option<f32>` values. Simple fields, high visual impact. |
| Preset listing API | `NativeTheme::list_presets()` returns available preset names without parsing. Enables theme selector UIs. catppuccin provides `FlavorIterator` for enumeration. Without a listing API, consumers must hardcode preset names. | LOW | `&'static [&'static str]` const array. Zero-cost, no TOML parsing. |
| `to_toml()` serialization | Exporting a runtime-read theme to TOML enables "read from OS, save to file" workflows. Users can snapshot their current OS theme as a portable TOML file, then load it on different machines/platforms. No competing crate supports this export flow. | LOW | `toml::to_string_pretty()` with `skip_serializing_if = "Option::is_none"` for clean output. Important that output is human-readable (pretty-printed, no `None` noise). |
| KDE kdeglobals reader (60+ color roles) | KDE's kdeglobals is the richest freely-accessible theme data source on any platform (60+ color roles in INI format). No other toolkit-agnostic crate reads it. `system-theme` only uses the portal (4 values). This reader populates most of the 36 semantic roles from a single file. | MEDIUM | `configparser::Ini::new_cs()` for case-sensitive parsing. Qt font string parsing (variable field counts). Dark mode detection via background luminance or `[General] ColorScheme` key. Well-documented format but many edge cases. |
| Freedesktop portal reader (live accent, scheme, contrast) | Reads accent color, color scheme, and contrast preference from the XDG Desktop Portal D-Bus interface. Works on both GNOME 47+ and KDE Plasma 6. Provides live change notifications via async stream. The only way to get GNOME's accent color dynamically. | MEDIUM | Async-only (ashpd/zbus). Must be a separate feature flag from `kde` to avoid forcing async runtime on sync-only users. Returns only 3-4 values -- best combined with preset or kdeglobals fallback for full theme. |
| Windows UISettings reader (accent + 8 colors) | Reads accent color + 6 shades + background + foreground from Windows UISettings COM API. This gives Windows apps OS-aware accent colors beyond the hardcoded blue/orange defaults. GetSystemMetrics provides border width and other metrics. | MEDIUM | Sync API via `windows` crate. API presence check (`ApiInformation::IsMethodPresent`) for graceful degradation on older Windows. 8 color types from `UIColorType` enum. |
| macOS NSColor reader (~20 semantic colors) | Reads ~20 semantic NSColor properties (controlAccentColor, windowBackgroundColor, labelColor, separatorColor, etc.) plus NSFont for system fonts and NSAppearance for dark/light detection. macOS has the richest set of dynamic semantic colors of any platform. | HIGH | Requires P3-to-sRGB color space conversion (crash without it -- see PITFALLS.md). Main thread appearance resolution. objc2-app-kit FFI. The most technically demanding reader due to color space and thread safety requirements. |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Built-in toolkit adapters (egui adapter, iced adapter, etc.) | "Just give me one function to apply the theme to my egui app." Reduces boilerplate for consumers. catppuccin provides `catppuccin-egui` with feature-gated iced/bevy/ratatui integration. | Couples the crate to specific toolkit versions. `system-theme` is coupled to iced -- when iced releases a breaking change, system-theme must update or become unusable. With 6+ target toolkits, maintaining adapters becomes a full-time job. egui's `Visuals` struct changes between versions. iced's `Palette` changes. Adapter breakage blocks theme crate releases. | Ship adapters as separate thin crates (`native-theme-egui`, `native-theme-iced`) maintained by the community, or document the ~50-line adapter pattern in examples/docs. The core crate stays toolkit-agnostic forever. |
| Named palette colors (system red, system blue, etc.) | macOS exposes ~13 named system colors. iOS has similar. GNOME Adwaita defines 9 named color scales. Users want a standard `theme.palette.red` field. | Too platform-specific. KDE has none. Windows has none. Android has none. Including them creates a massively asymmetric model where macOS/iOS have rich data and other platforms have all-`None`. Semantic status colors (error=red, success=green, warning=orange) already cover the practical use cases without platform-specific naming. | Map platform named colors to semantic roles in the reader (macOS `systemRedColor` -> `error`). Document that named palette access is available directly through platform APIs for apps that need it. |
| Widget-level metrics (button height, checkbox size, scrollbar width, etc.) | KDE breezemetrics.h defines ~80 widget constants. Windows `GetSystemMetrics` provides ~20. Users building toolkit replacements want pixel-perfect widget sizing. | No toolkit currently consumes per-widget metrics (egui hardcodes `.px_N()`; iced has no per-widget size API). KDE is the only rich source; other platforms provide almost nothing, making the model extremely asymmetric. Including 80+ widget fields in the model inflates the data type with fields that are `None` on 5 of 6 platforms. The ROI is near zero in 2026. | Defer to post-1.0. The model uses `#[non_exhaustive]`, so widget metrics can be added later without breaking changes. Document the deferral and the data sources for future implementors. |
| Color space conversion utilities (sRGB <-> P3, HSL, Oklch, etc.) | W3C design tokens spec supports P3, Oklch. Users want to manipulate theme colors (darken, lighten, generate shades). | Out of scope for a theme data crate. Color math is a separate domain (see `palette` crate, `csscolorparser`). Adding conversion utilities bloats the crate and duplicates well-maintained ecosystem crates. The `Rgba` type is a storage format, not a color manipulation toolkit. | Store everything as sRGB (the universal interchange format). Document that consumers wanting color math should convert `Rgba` to their preferred crate's type (e.g., `palette::Srgba`). Platform readers handle P3-to-sRGB conversion internally. |
| Reactive change notification system (built-in event bus) | `system-theme` provides change callbacks. Electron nativeTheme emits `'updated'` events. Users want automatic theme updates when the OS theme changes. | Building a general-purpose reactive system (channels, callbacks, subscriptions) is complex, opinionated (which async runtime? channels vs callbacks?), and duplicates what every GUI toolkit already provides (egui: `observe_window_appearance`, iced: subscriptions, gpui: observers). The crate becomes a mini-runtime. | Document the platform-specific change notification sources (KDE: file watcher on kdeglobals, portal: `SettingChanged` D-Bus signal, macOS: `NSSystemColorsDidChangeNotification`, Windows: `ColorValuesChanged`). Provide the optional `watch` feature flag for file watching. Let consumers integrate change detection into their existing event loop. |
| CSS/SCSS export format | Some users want to generate CSS custom properties from theme data for web-based UIs (tauri, dioxus). | Supporting CSS output adds a dependency on CSS formatting, creates a secondary serialization path to maintain, and is trivially implementable by consumers (`format!("--accent: {};", theme.colors.accent)` is a one-liner per field). The crate's job is data, not rendering. | Document a 10-line CSS export example in the cookbook. If demand grows, publish as a separate `native-theme-css` crate. |
| Runtime theme interpolation / animation | Animating between theme variants (smooth dark/light transition). Some desktop environments animate theme transitions. | Animation is toolkit-specific behavior. egui, iced, and gpui each have their own animation systems. A data crate cannot own animation timing. Storing interpolation state in the theme struct conflates data with rendering concerns. | Document that consumers can interpolate between two `ThemeVariant` structs using their toolkit's animation system. Provide a `ThemeVariant::lerp()` utility method if demand emerges (it operates on `Option<Rgba>` fields, returning interpolated values). |
| Design token format (W3C DTCG JSON) | W3C design tokens spec reached stable 2025.10. Some teams want token-format import/export. | W3C tokens are JSON-based with a completely different schema (`$value`, `$type`, token groups with `$extensions`). Supporting both TOML (native-theme format) and W3C JSON adds a parallel serialization layer. The W3C spec is designed for web design systems, not OS-level theme data. The overlap is partial at best. | Document the mapping between native-theme TOML fields and W3C token names. Provide a conversion example in docs. If demand grows, publish as a separate `native-theme-tokens` crate. |
| Android runtime reader (Material You) | Android has the richest dynamic accent system (65 tonal palette values on API 31+). Users want Android support. | JNI is verbose, error-prone, and requires an Android JNI context. No ergonomic wrapper exists for Material You in Rust. The Rust Android ecosystem is the least mature. Building this before desktop platforms are solid diverts effort from higher-impact work. | Defer to late phase (Phase 7+). Include a Material 3 preset with static values for immediate usability. The reader can be added when the Android Rust tooling matures. |
| iOS runtime reader | iOS UIColor has ~30 semantic colors. Dynamic Type provides font scaling. | Requires objc2-ui-kit, which is the same ecosystem as macOS (shared objc2 knowledge). But iOS testing requires physical devices or simulators. The audience for Rust iOS GUI apps is small in 2026. | Defer to late phase (Phase 7+). Include an iOS preset with static values. The reader can follow the macOS reader pattern closely since UIKit mirrors AppKit semantics. |

## Feature Dependencies

```
[Rgba type + custom serde]
    └──requires──> [serde + toml deps]

[ThemeColors struct (36 fields)]
    └──requires──> [Rgba type]

[ThemeFonts / ThemeGeometry / ThemeSpacing structs]
    └──requires──> [serde]

[ThemeVariant struct]
    └──requires──> [ThemeColors, ThemeFonts, ThemeGeometry, ThemeSpacing]

[NativeTheme struct]
    └──requires──> [ThemeVariant]

[merge() method]
    └──requires──> [Option-all-fields data model]
    └──requires──> [macro-generated struct pattern]

[Preset loading (preset(), from_file(), to_toml())]
    └──requires──> [NativeTheme + serde round-trip]

[Bundled TOML presets]
    └──requires──> [Preset loading API]
    └──requires──> [Stable data model schema]

[KDE reader (feature "kde")]
    └──requires──> [NativeTheme data model]
    └──requires──> [Error type]

[Portal reader (feature "portal")]
    └──requires──> [NativeTheme data model]
    └──requires──> [Error type]
    └──requires──> [async runtime (tokio or async-io)]

[Windows reader (feature "windows")]
    └──requires──> [NativeTheme data model]
    └──requires──> [Error type]

[macOS reader (feature "macos")]
    └──requires──> [NativeTheme data model]
    └──requires──> [Error type]

[from_system() dispatch]
    └──requires──> [At least one platform reader]
    └──enhances──> [Bundled presets (fallback)]

[Community presets (Catppuccin, Dracula, Nord)]
    └──requires──> [Stable TOML schema]
    └──requires──> [Preset loading API]

[to_toml() export]
    └──enhances──> [Runtime readers (snapshot OS theme to file)]

[Partial TOML overrides]
    └──requires──> [merge() method]
    └──requires──> [#[serde(default)] on all structs]
```

### Dependency Notes

- **Bundled presets require stable data model:** Preset TOML files are authored against a fixed schema. Changing the schema means updating all presets. Stabilize the model before authoring many presets.
- **Platform readers are independent:** No reader depends on another reader. They can be built in any order. Each produces the same `NativeTheme` type.
- **Portal reader requires async runtime:** The `portal` feature pulls in ashpd/zbus (async-only). This must be a separate feature from `kde` (sync) to avoid forcing tokio on sync consumers. Feature flag design must be correct from the start (changing feature structure after publish is breaking).
- **`from_system()` depends on readers but enhances presets:** The sync `from_system()` dispatcher calls platform-specific readers. On platforms with no enabled reader, it falls back to bundled presets. This means presets must exist before `from_system()` is useful as a fallback.
- **`merge()` enables partial overrides:** Without merge, users must write complete TOML files. Merge is what makes the "3-line TOML override" workflow possible. Build merge into the model from day one.

## MVP Definition

### Launch With (v0.1)

Minimum viable product -- what's needed to validate the concept. Delivers the data model + presets + serde. Any Rust GUI app can immediately load a platform-appropriate theme from a TOML file.

- [x] `Rgba` type with custom hex serde (`#rrggbb` / `#rrggbbaa`) -- the foundational color type
- [x] `ThemeColors` struct (36 semantic fields, all `Option<Rgba>`) -- core color data model
- [x] `ThemeFonts`, `ThemeGeometry`, `ThemeSpacing` structs -- complete variant data
- [x] `ThemeVariant` struct composing all sub-structs -- single variant container
- [x] `NativeTheme` struct with `name`, `light`, `dark` -- top-level type
- [x] `merge()` on all sub-structs via declarative macro -- theme layering
- [x] `#[non_exhaustive]` on all public structs -- forward compatibility
- [x] `#[serde(default)]` + `skip_serializing_if` on all fields -- sparse TOML support
- [x] `preset()`, `list_presets()`, `from_file()`, `to_toml()` API -- preset loading/saving
- [x] 3 bundled presets: `default`, `kde-breeze`, `adwaita` (light + dark each) -- immediate usability
- [x] Error enum (`Unsupported`, `Unavailable`, `Format`, `Platform`) -- typed errors
- [x] TOML round-trip tests, minimal TOML tests, Rgba edge case tests -- correctness validation

### Add After Validation (v0.2 - v0.4)

Features to add once core is working and the TOML schema is validated.

- [ ] KDE kdeglobals reader (feature `kde`) -- trigger: core model proven stable via preset usage
- [ ] Freedesktop portal reader (feature `portal`) -- trigger: Linux users want live accent/scheme
- [ ] `from_system()` Linux dispatch (KDE detection + portal/preset fallback) -- trigger: both Linux readers exist
- [ ] Additional presets: `windows-11`, `macos-sonoma`, `material` -- trigger: data model validated against real theme data
- [ ] File watching (feature `watch`, notify crate) for kdeglobals changes -- trigger: users want live updates without portal
- [ ] Windows UISettings reader (feature `windows`) -- trigger: Windows users request it
- [ ] macOS NSColor reader (feature `macos`) -- trigger: macOS users request it

### Future Consideration (v1.0+)

Features to defer until product-market fit is established.

- [ ] iOS runtime reader (feature `ios`) -- why defer: small Rust iOS GUI audience; requires device testing
- [ ] Android runtime reader (feature `android`) -- why defer: immature Rust Android tooling; JNI complexity is 3-5x other platforms
- [ ] Widget-level metrics (button height, scrollbar width, etc.) -- why defer: no toolkit consumes these today; asymmetric platform coverage
- [ ] Community preset registry / repository -- why defer: needs ecosystem adoption first
- [ ] W3C design token format import/export -- why defer: different schema, partial overlap, low demand from Rust GUI developers

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Semantic color model (36 roles) | HIGH | MEDIUM | P1 |
| Light/dark variant support | HIGH | LOW | P1 |
| TOML serde round-trip | HIGH | MEDIUM | P1 |
| Bundled presets (3 initial) | HIGH | MEDIUM | P1 |
| Theme merging / layering | HIGH | LOW | P1 |
| Rgba type with hex serde | HIGH | LOW | P1 |
| Font data model | MEDIUM | LOW | P1 |
| Geometry data model | MEDIUM | LOW | P1 |
| Spacing data model | MEDIUM | LOW | P1 |
| Error enum | MEDIUM | LOW | P1 |
| `#[non_exhaustive]` on all types | MEDIUM | LOW | P1 |
| `from_file()` / `to_toml()` API | MEDIUM | LOW | P1 |
| Preset listing API | LOW | LOW | P1 |
| KDE kdeglobals reader | HIGH | MEDIUM | P2 |
| Portal reader (GNOME accent) | HIGH | MEDIUM | P2 |
| `from_system()` dispatch | HIGH | MEDIUM | P2 |
| Windows UISettings reader | MEDIUM | MEDIUM | P2 |
| macOS NSColor reader | MEDIUM | HIGH | P2 |
| Additional presets (Win11, macOS, Material) | MEDIUM | MEDIUM | P2 |
| File watching (kdeglobals) | LOW | LOW | P2 |
| Partial TOML override docs | MEDIUM | LOW | P2 |
| Disabled/border opacity fields | LOW | LOW | P2 |
| iOS runtime reader | LOW | MEDIUM | P3 |
| Android runtime reader | LOW | HIGH | P3 |
| Widget-level metrics | LOW | HIGH | P3 |
| Community preset registry | MEDIUM | MEDIUM | P3 |

**Priority key:**
- P1: Must have for launch (v0.1)
- P2: Should have, add when possible (v0.2-v0.x)
- P3: Nice to have, future consideration (v1.0+)

## Competitor Feature Analysis

| Feature | dark-light 2.0 | system-theme 0.3 | cosmic-theme | catppuccin 2.x | native-theme (ours) |
|---------|---------------|------------------|-------------|---------------|-------------------|
| Dark/light detection | YES (core purpose) | YES | YES | NO (data-only) | YES (via readers) |
| Accent color | NO | YES (1 color) | YES | NO | YES (+ 6 shades on Windows) |
| Semantic color roles | NO | 6 roles | 30+ roles | 26 named colors (not semantic) | 36 semantic roles |
| Font data | NO | NO | YES | NO | YES |
| Geometry / spacing | NO | NO | YES (corner radii, spacing scale, density) | NO | YES |
| Serialization format | N/A | None (const structs) | RON | None (const structs) | TOML |
| User-editable themes | NO | NO | YES (RON files) | NO | YES (TOML files) |
| Theme merging | NO | NO | Partial | NO | YES (field-by-field Option merge) |
| Toolkit-agnostic | YES | NO (iced dep) | NO (iced/libcosmic dep) | Partial (feature-gated toolkit deps) | YES (zero GUI deps) |
| Community presets | NO | NO | YES (cosmic-themes.org) | YES (4 built-in flavors) | YES (TOML contribution) |
| Platform coverage | Linux, macOS, Windows, BSD, WASM | Linux, macOS, Windows | COSMIC/Linux only | N/A (static data) | Linux, macOS, Windows (+ iOS, Android future) |
| Runtime readers | N/A (mode only) | YES (thin) | N/A (reads own config) | N/A | YES (rich, feature-gated) |
| Alpha channel | NO | NO (RGB only) | YES | YES (hex) | YES (#RRGGBBAA) |
| File I/O (load/save) | NO | NO | YES (RON) | NO | YES (TOML) |
| Change notifications | NO | YES (callbacks) | N/A | NO | Partial (file watch + docs for platform-specific) |

**Key takeaways from competitor analysis:**
1. **No competitor is both toolkit-agnostic AND comprehensive.** dark-light is agnostic but minimal. cosmic-theme is comprehensive but COSMIC-locked. system-theme is in between but iced-coupled. native-theme fills the gap.
2. **Serialization is the biggest gap.** catppuccin and system-theme use const Rust structs. TOML editability is a genuine differentiator.
3. **Alpha channel is a known gap** in system-theme. Starting with Rgba avoids the same mistake.
4. **Theme merging is barely supported** anywhere. This is a high-value, low-cost feature.
5. **Community presets** are proven valuable by catppuccin (widely adopted) and cosmic-themes.org. TOML format lowers the contribution barrier vs RON or Rust code.

## Sources

- [catppuccin Rust crate](https://docs.rs/catppuccin) -- color palette structure, toolkit integrations via feature flags (HIGH confidence)
- [dark-light 2.0](https://docs.rs/dark-light/latest/dark_light/) -- dark/light mode detection API, platform support (HIGH confidence)
- [system-theme 0.3.0](https://crates.io/crates/system-theme) -- prior art for runtime theme reading, 6-color palette limitation (HIGH confidence)
- [cosmic-theme / COSMIC themes](https://cosmic-themes.org/create/) -- comprehensive RON theme system, community theme repository (MEDIUM confidence)
- [egui Visuals struct](https://docs.rs/egui/latest/egui/style/struct.Visuals.html) -- 35 visual fields, color/corner/shadow configuration (HIGH confidence)
- [egui Style struct](https://docs.rs/egui/latest/egui/style/struct.Style.html) -- spacing, text styles, interaction configuration (HIGH confidence)
- [iced Extended palette](https://docs.iced.rs/iced/theme/palette/struct.Extended.html) -- background/primary/secondary/success/danger color groups (HIGH confidence)
- [Electron nativeTheme API](https://www.electronjs.org/docs/latest/api/native-theme) -- shouldUseDarkColors, highContrast, reducedTransparency, updated event (HIGH confidence)
- [Electron systemPreferences API](https://www.electronjs.org/docs/latest/api/system-preferences) -- getAccentColor, getColor, getSystemColor, accent-color-changed event (HIGH confidence)
- [W3C Design Tokens spec 2025.10](https://www.w3.org/community/design-tokens/2025/10/28/design-tokens-specification-reaches-first-stable-version/) -- stable format for design tokens, theming/multi-brand support (MEDIUM confidence)
- [Theme UI merging guide](https://theme-ui.com/guides/merging-themes) -- deep merge patterns for theme composition (MEDIUM confidence)
- [Slint native styling](https://slint.dev/) -- native look via Qt/platform styles, theme integration approach (MEDIUM confidence)
- [Android Material Theme Overlay](https://developer.android.com/reference/com/google/android/material/theme/overlay/MaterialThemeOverlay) -- theme overlay/merge pattern (MEDIUM confidence)
- [IMPLEMENTATION.md](../../docs/IMPLEMENTATION.md) -- project specification, platform capabilities matrix, data model design (HIGH confidence)
- [STACK.md](./STACK.md) -- verified technology stack for all platform readers (HIGH confidence)
- [PITFALLS.md](./PITFALLS.md) -- domain pitfalls informing feature design (macro-generated merge, serde defaults) (HIGH confidence)

---
*Feature research for: native-theme (toolkit-agnostic cross-platform OS theme data crate)*
*Researched: 2026-03-07*
