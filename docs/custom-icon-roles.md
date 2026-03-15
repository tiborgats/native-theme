# Custom Icon Roles: Letting Apps Extend the Icon System

## Problem Statement

The `native-theme` crate defines 42 `IconRole` variants covering common UI
concepts (dialog indicators, window controls, actions, navigation, files,
status, system). These are mapped to every supported icon set: Material, Lucide,
freedesktop, SF Symbols, and Segoe Fluent.

But real-world applications need more icons than these 42. A music player needs
play/pause/skip icons. A code editor needs syntax-highlighting, git-branch,
terminal icons. A home automation app needs thermometer, lightbulb, door-lock
icons. These are **domain-specific** and could be arbitrarily exotic.

The GPUI connector already deals with this on a smaller scale: gpui-component
has 86 `IconName` variants, of which only 30 overlap with `IconRole`. The
remaining 56 are handled with hand-written mapping functions in the connector
(`lucide_name_for_gpui_icon`, `material_name_for_gpui_icon`,
`freedesktop_name_for_gpui_icon`).

This approach doesn't scale to arbitrary applications. Each new app would need
to duplicate this mapping work for every icon set it wants to support.

### The Core Requirement

When the user switches between icon themes in the app (e.g., Material to Lucide
to Breeze), **every** icon in the UI must switch to the corresponding variant
from the new theme. This includes both the built-in 42 roles and any
app-specific extras.

### Hard Constraint: Fully Offline at Runtime

The shipped binary must work without network access. No runtime downloads,
no CDN fetches on first launch, no "downloading icons..." progress bars.

If an icon must be present in a given theme, its data (SVG bytes, name mapping,
or system API identifier) must be available at compile time or loadable from the
local filesystem at runtime (e.g., freedesktop themes installed via the system
package manager).

**Build-time downloads are a different question** — see the dedicated section
below.

## Anatomy of the Problem

An application icon that works across themes has **three layers** of data, each
with different characteristics:

### 1. Name Mappings (Cheap, Static)

A table that says "my `PlayPause` icon is called `play_pause` in Material,
`play` in Lucide, `media-playback-start` in freedesktop, `play.fill` in
SF Symbols, and `Play` in Segoe Fluent."

This is just `(&str, IconSet) -> &str`. It's tiny, compile-time constant, and
the same across all platforms.

### 2. SVG Bytes for Bundled Sets (Medium, Compile-Time)

For Material and Lucide, the crate embeds SVG files via `include_bytes!()`.
Currently, 87 Material SVGs and 103 Lucide SVGs are bundled. Adding an icon
means adding an SVG file to `icons/material/` or `icons/lucide/` and an arm
to the match statement.

Each SVG is typically 200-800 bytes. Even 200 extra icons across two sets would
add roughly 200-400 KB to the binary — well within acceptable limits.

### 3. System Icon Resolution (Free, Runtime)

On Linux, freedesktop icons are loaded from the installed theme (e.g.,
`/usr/share/icons/breeze/`) at runtime. The name mapping is all that's needed;
the SVG files are already on disk.

On macOS, SF Symbols are loaded via `NSImage(systemSymbolName:)`. The name is
all that's needed; Apple ships the icons with the OS.

On Windows, Segoe Fluent glyphs are rendered via `GetGlyphOutlineW`. Again,
only the glyph name/codepoint is needed.

System icons are inherently offline — they're part of the OS.

## Design Space

### What an App Developer Needs to Provide

For each custom icon role, the app developer must provide:

| Icon Set | What's Needed | Who Provides It |
|---|---|---|
| Material | SVG file + name | App developer finds it in the Material Symbols repository and puts it in their build |
| Lucide | SVG file + name | App developer finds it in the Lucide repository and puts it in their build |
| Freedesktop | Name mapping (possibly DE-aware) | App developer looks up the name in Breeze/Adwaita |
| SF Symbols | Symbol name string | App developer looks up the name in SF Symbols app |
| Segoe Fluent | Glyph name or SIID constant | App developer looks up the name in Segoe Fluent docs |

For icons that don't exist in a given set (e.g., no "quantum-computer" in any
standard set), the app developer must supply a custom SVG that serves as the
fallback for that theme.

### Key Tension: Compile-Time vs Runtime Registration

**Compile-time** (proc macros, build scripts, `include_bytes!` in app code):
- SVGs are embedded in the binary — guaranteed available offline
- Errors caught at build time (missing file, wrong path)
- No startup overhead
- Rigid: can't add icons without recompiling

**Runtime** (registration at startup):
- App calls `register_icon("bluetooth", ...)` during initialization
- More flexible: plugins could add icons
- Can read SVGs from app resources directory instead of embedding
- Errors deferred to runtime

Both are valid. The crate should support the compile-time path as the primary
recommendation (it aligns with how the existing `bundled.rs` works) and the
runtime path as an alternative.

## Approaches

### Approach A: Trait-Based Icon Provider

The crate defines a trait that apps implement to extend the icon system:

```rust
/// Trait for providing custom icon data across multiple icon sets.
///
/// Implementors map application-specific icon identifiers to icon data
/// for each supported icon set, enabling theme-switching for custom icons.
pub trait IconProvider {
    /// The type used to identify icons. Typically an enum or &str.
    type IconId: Eq + Hash;

    /// Return the icon name for a given icon set.
    ///
    /// Used for system icon resolution (freedesktop, SF Symbols, Segoe).
    /// Return None if the icon doesn't exist in this set.
    fn icon_name(&self, id: &Self::IconId, set: IconSet) -> Option<&str>;

    /// Return bundled SVG bytes for a given icon set.
    ///
    /// Used for Material and Lucide where SVGs are compiled in.
    /// Return None to fall back to system resolution or another set.
    fn icon_svg(&self, id: &Self::IconId, set: IconSet) -> Option<&[u8]>;

    /// Return the fallback icon set to try when the primary set fails.
    fn fallback_set(&self, id: &Self::IconId) -> Option<IconSet> {
        _ = id;
        Some(IconSet::Material)  // sensible default
    }
}
```

App-side usage:

```rust
enum MyIcon {
    PlayPause,
    SkipForward,
    Bluetooth,
    Thermometer,
}

struct MyIconProvider;

impl IconProvider for MyIconProvider {
    type IconId = MyIcon;

    fn icon_name(&self, id: &MyIcon, set: IconSet) -> Option<&str> {
        match (id, set) {
            (MyIcon::PlayPause, IconSet::Material) => Some("play_pause"),
            (MyIcon::PlayPause, IconSet::Lucide) => Some("play"),
            (MyIcon::PlayPause, IconSet::Freedesktop) => Some("media-playback-start"),
            (MyIcon::PlayPause, IconSet::SfSymbols) => Some("play.fill"),
            // ...
            _ => None,
        }
    }

    fn icon_svg(&self, id: &MyIcon, set: IconSet) -> Option<&[u8]> {
        match (id, set) {
            (MyIcon::PlayPause, IconSet::Material) =>
                Some(include_bytes!("icons/material/play_pause.svg")),
            (MyIcon::PlayPause, IconSet::Lucide) =>
                Some(include_bytes!("icons/lucide/play.svg")),
            // ...
            _ => None,
        }
    }
}
```

Then a loading function in the crate:

```rust
/// Load a custom icon using an IconProvider, with the standard fallback chain.
pub fn load_custom_icon<P: IconProvider>(
    provider: &P,
    id: &P::IconId,
    icon_set: &str,
) -> Option<IconData> {
    let set = IconSet::from_name(icon_set).unwrap_or_else(system_icon_set);

    // 1. Try system loader with the provider's name mapping
    if let Some(name) = provider.icon_name(id, set) {
        if let Some(data) = load_system_icon_by_name(name, set) {
            return Some(data);
        }
    }

    // 2. Try bundled SVG from the provider
    if let Some(svg) = provider.icon_svg(id, set) {
        return Some(IconData::Svg(svg.to_vec()));
    }

    // 3. Try fallback set
    if let Some(fallback) = provider.fallback_set(id) {
        if let Some(svg) = provider.icon_svg(id, fallback) {
            return Some(IconData::Svg(svg.to_vec()));
        }
    }

    None
}
```

#### Pros

- Type-safe: app's icon enum catches typos at compile time
- `include_bytes!` in the app guarantees offline availability
- The crate itself doesn't need to grow to support app-specific icons
- Multiple providers can coexist (core library + app + plugins)
- No macros needed — just a trait implementation
- Works naturally with connectors: `load_custom_icon` returns `IconData`,
  which the GPUI connector's `to_image_source` already handles

#### Cons

- Verbose: each icon needs entries in both `icon_name` and `icon_svg` for
  every set. An icon supporting all 5 sets needs ~10 match arms minimum.
- The app developer must source and bundle the SVG files themselves
- No compile-time validation that all sets are covered (only that the SVG
  files exist). Missing entries silently fall through to fallback.

### Approach B: Declarative Macro for Icon Tables

A macro that generates the provider implementation from a concise table:

```rust
native_theme::define_icons! {
    /// Custom icons for MyApp.
    pub MyIcons {
        PlayPause {
            material: "play_pause" => include_bytes!("icons/material/play_pause.svg"),
            lucide: "play" => include_bytes!("icons/lucide/play.svg"),
            freedesktop: "media-playback-start",
            sf_symbols: "play.fill",
            segoe: "Play",
        },
        SkipForward {
            material: "skip_next" => include_bytes!("icons/material/skip_next.svg"),
            lucide: "skip-forward" => include_bytes!("icons/lucide/skip-forward.svg"),
            freedesktop: "media-skip-forward",
            sf_symbols: "forward.end.fill",
            segoe: "Next",
        },
        Thermometer {
            // Only Material and Lucide — will fall back on other platforms
            material: "thermostat" => include_bytes!("icons/material/thermostat.svg"),
            lucide: "thermometer" => include_bytes!("icons/lucide/thermometer.svg"),
        },
    }
}
```

The macro expands to:

1. An enum `MyIcons { PlayPause, SkipForward, Thermometer }` with derives
2. An `impl IconProvider for MyIcons` (or a standalone provider struct)
3. All the `icon_name` and `icon_svg` match arms

#### Pros

- Compact: one table defines everything
- Hard to make mistakes: the macro enforces the pattern
- Good documentation: the table IS the documentation of what's mapped where
- Could emit compile-time warnings for sets that have a name but no SVG
  (for bundled sets) or SVG but no name

#### Cons

- Proc macros are a heavy dependency; declarative macros are limited in what
  validation they can do
- Custom syntax has a learning curve
- Harder to debug when things go wrong
- The macro itself is complex to implement and maintain
- Opaque to IDE tooling (autocomplete, go-to-definition)

### Approach C: String-Keyed Registry with Builder API

Instead of enums, use string keys. The crate provides a builder that
constructs an icon registry:

```rust
let icons = IconRegistry::builder()
    .icon("play-pause")
        .material("play_pause", include_bytes!("icons/material/play_pause.svg"))
        .lucide("play", include_bytes!("icons/lucide/play.svg"))
        .freedesktop_name("media-playback-start")
        .sf_symbols_name("play.fill")
        .segoe_name("Play")
        .done()
    .icon("skip-forward")
        .material("skip_next", include_bytes!("icons/material/skip_next.svg"))
        .lucide("skip-forward", include_bytes!("icons/lucide/skip-forward.svg"))
        .freedesktop_name("media-skip-forward")
        .done()
    .icon("thermometer")
        .material("thermostat", include_bytes!("icons/material/thermostat.svg"))
        .lucide("thermometer", include_bytes!("icons/lucide/thermometer.svg"))
        .done()
    .build();

// Later:
let data = icons.load("play-pause", "material")?;
let data = icons.load("play-pause", active_theme)?;  // theme-aware
```

#### Pros

- No macros, no traits — just normal Rust code
- Easy to understand and debug
- Flexible: icons can be added from config files, plugins, etc.
- Natural fit for apps that already use string keys for resources
- The builder can validate at `build()` time: warn about icons that don't
  cover all sets, detect duplicate names, etc.

#### Cons

- String keys: typos caught at runtime, not compile time
- Slightly more runtime overhead (HashMap lookups vs enum match)
- The `include_bytes!` calls still happen at compile time, so the SVGs
  are always in the binary even if the builder is never invoked
- No type-level guarantee that an icon exists

### Approach D: Extend the Core `IconRole` Enum via Feature Flags

Add feature-gated variants to `IconRole` for common domain categories:

```toml
# Cargo.toml
[features]
icons-media = []       # PlayPause, SkipForward, SkipBack, Stop, Record, ...
icons-development = [] # GitBranch, GitCommit, Terminal, Debug, ...
icons-communication = [] # Email, Chat, Phone, VideoCall, ...
icons-iot = []         # Thermometer, Lightbulb, Sensor, ...
```

```rust
#[non_exhaustive]
pub enum IconRole {
    // ... existing 42 ...

    #[cfg(feature = "icons-media")]
    MediaPlay,
    #[cfg(feature = "icons-media")]
    MediaPause,
    // ...
}
```

Each feature flag adds the enum variants, the name mappings for all 5 icon
sets, and the bundled SVGs.

#### Pros

- Seamless: custom icons work exactly like built-in ones
- Zero boilerplate for app developers: just enable a feature
- Compile-time checked, fully typed
- Name mappings are curated by the crate maintainer — app developers don't
  need to research icon names across 5 platforms

#### Cons

- Doesn't scale to exotic icons: the crate can't anticipate "quantum-computer"
- Every new domain category grows the crate's maintenance surface
- Feature flags create a combinatorial testing problem
- Release bottleneck: apps are blocked on upstream PRs to add their icons
- `#[non_exhaustive]` plus feature flags interact awkwardly — downstream
  match arms may break when features are toggled
- The bundled SVG count grows without bound, eventually hitting crates.io
  size limits (10 MB per crate)

### Approach E: Hybrid — Domain Feature Flags + Trait Extension Point

Combine D (for common domains) with A (for truly exotic icons):

- The crate ships feature-gated `IconRole` variants for well-known domains
  (media playback, VCS, communication) — these cover 80% of apps
- Apps with exotic needs implement `IconProvider` for the remaining 20%
- Both paths produce `IconData` and flow through the same connector functions

### Approach F: Data-Driven TOML Definitions with Strict Build Validation

The key insight behind approaches A-C is that `icon_svg()` is pure boilerplate:
given a name like `"play_pause"` and a directory like `icons/material/`, the
implementation is always `include_bytes!("icons/material/play_pause.svg")`. The
app developer shouldn't have to write this. They should only define:

1. **What** icon roles exist (master TOML)
2. **How** each role maps to each theme (per-theme TOML)

Everything else — the enum, the trait impl, the `include_bytes!` calls — is
generated. And crucially, the build system **enforces completeness**: every
role must be accounted for in every theme. No silent gaps, no lazy omissions.

#### Separation of concerns

Each theme owns its own mapping file. The master file only lists roles and
declares which themes are in play:

```
icons/
  custom-icons.toml           # role list + theme declarations
  material/                   # bundled theme: mapping + SVGs
    mapping.toml
    play_pause.svg
    skip_next.svg
    bluetooth.svg
    thermostat.svg
  lucide/                     # bundled theme: mapping + SVGs
    mapping.toml
    play.svg
    skip-forward.svg
    bluetooth.svg
    thermometer.svg
  freedesktop/                # system theme: mapping only (no SVGs)
    mapping.toml
  sf-symbols/                 # system theme: mapping only
    mapping.toml
  segoe-fluent/               # system theme: mapping only
    mapping.toml
```

Every declared theme gets a subdirectory with a mandatory `mapping.toml`.
Bundled themes also contain SVG files. System themes contain only the mapping.

#### The master file

The master TOML declares roles and themes. It does **not** contain any
name mappings — those live in the theme-specific files.

```toml
# icons/custom-icons.toml

# Name of the generated enum and provider.
# Produces `enum AppIcon { ... }` and `struct AppIconProvider`.
# kebab-case → PascalCase (e.g., "app-icon" → AppIcon).
name = "app-icon"

# Icon roles for this application.
# Each becomes an enum variant (kebab-case → PascalCase).
# Every role must have an entry in every theme's mapping.toml.
roles = [
  "play-pause",
  "skip-forward",
  "bluetooth",
  "thermometer",
  "eye-toggle",
]

# Themes where SVG files are compiled into the binary.
# Each must have a subdirectory with mapping.toml + one SVG per role.
bundled-themes = ["material", "lucide"]

# Themes resolved by the OS icon loader at runtime (name strings only).
# Each must have a subdirectory with mapping.toml (no SVGs).
system-themes = ["freedesktop", "sf-symbols", "segoe-fluent"]
```

This is the **single source of truth** for what exists. Adding a new role
means adding one line here — and then the build immediately fails until every
theme's mapping.toml accounts for it. Adding a new theme means adding it here
and creating its subdirectory — and the build fails until it covers every role.

#### Per-theme mapping files

Each theme's `mapping.toml` maps every role to that theme's icon name.

**Bundled theme** (SVGs compiled in):

```toml
# icons/material/mapping.toml
#
# Maps role names to Material Symbols icon names.
# Each name must match an SVG file in this directory (without .svg).

play-pause = "play_pause"
skip-forward = "skip_next"
bluetooth = "bluetooth"
thermometer = "thermostat"
eye-toggle = "visibility"
```

```toml
# icons/lucide/mapping.toml

play-pause = "play"
skip-forward = "skip-forward"
bluetooth = "bluetooth"
thermometer = "thermometer"
eye-toggle = "eye"
```

**System theme** (name-only, loaded from OS at runtime):

```toml
# icons/freedesktop/mapping.toml
#
# Maps role names to freedesktop icon names.
# Use an inline table for DE-aware names.
# Every role must have an icon name — no gaps allowed.

play-pause = "media-playback-start"
skip-forward = "media-skip-forward"
bluetooth = "bluetooth"
thermometer = "sensors"
eye-toggle = { kde = "view-visible", default = "view-reveal" }
```

The `default` key is required in DE-aware inline tables. It covers all desktop
environments not explicitly listed (XFCE, Cinnamon, MATE, LXQt, Budgie, and
any future or unknown environments). Recognized DE keys are: `kde`, `gnome`,
`xfce`, `cinnamon`, `mate`, `lxqt`, `budgie`. Any subset can appear alongside
a mandatory `default`.

```toml
# icons/sf-symbols/mapping.toml

play-pause = "play.fill"
skip-forward = "forward.end.fill"
bluetooth = "antenna.radiowaves.left.and.right"
thermometer = "thermometer.medium"
eye-toggle = "eye"
```

```toml
# icons/segoe-fluent/mapping.toml

play-pause = "Play"
skip-forward = "Next"
bluetooth = "Bluetooth"
thermometer = "MapPin"
eye-toggle = "View"
```

**Key rules:**
- Every role from the master list must appear in every mapping file
- A string value = the icon name for that theme
- Inline table = DE-aware name variants (freedesktop only); must include a
  `default` key covering unlisted desktop environments
- Missing entry = **build error** (catches the lazy human mistake)
- There is no escape hatch — every theme maintainer must find a real icon for
  every role. This prevents UX degradation from missing icons.

#### Why per-theme files, not one big file?

1. **Ownership.** A designer managing Material icons edits only
   `material/mapping.toml`. They don't touch the freedesktop mappings. A
   platform specialist adds SF Symbols names without risk of breaking Lucide.

2. **Diffability.** A PR that "adds thermometer to the Lucide theme" touches
   one mapping file and one SVG — clear, reviewable, atomic.

3. **CLI tool integration.** `cargo native-theme-icons fetch bluetooth
   --material --lucide` can write directly into each theme's mapping.toml +
   drop the SVG, without parsing/rewriting a monolithic file.

4. **Strictness.** The build system checks each file independently. A theme
   that's missing a role fails immediately, with a message like:
   `error: role "thermometer" not found in icons/segoe-fluent/mapping.toml`

#### Code generation

The app adds a build dependency:

```toml
# Cargo.toml
[build-dependencies]
native-theme-build = "0.1"
```

```rust
// build.rs
fn main() {
    native_theme_build::generate_icons("icons/custom-icons.toml");
}
```

The generated code in `OUT_DIR`:

```rust
// Generated by native-theme-build — do not edit

// Enum name and provider name derived from `name = "app-icon"` in the master TOML.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppIcon {
    PlayPause,
    SkipForward,
    Bluetooth,
    Thermometer,
    EyeToggle,
}

impl AppIcon {
    pub const ALL: &[Self] = &[
        Self::PlayPause,
        Self::SkipForward,
        Self::Bluetooth,
        Self::Thermometer,
        Self::EyeToggle,
    ];
}

pub struct AppIconProvider;

impl native_theme::IconProvider for AppIconProvider {
    type IconId = AppIcon;

    fn icon_name(&self, id: &AppIcon, set: IconSet) -> Option<&str> {
        use native_theme::IconSet;
        match (id, set) {
            // Material (from icons/material/mapping.toml)
            (AppIcon::PlayPause, IconSet::Material) => Some("play_pause"),
            (AppIcon::SkipForward, IconSet::Material) => Some("skip_next"),
            (AppIcon::Bluetooth, IconSet::Material) => Some("bluetooth"),
            (AppIcon::Thermometer, IconSet::Material) => Some("thermostat"),
            (AppIcon::EyeToggle, IconSet::Material) => Some("visibility"),

            // Lucide (from icons/lucide/mapping.toml)
            (AppIcon::PlayPause, IconSet::Lucide) => Some("play"),
            // ...

            // Freedesktop (from icons/freedesktop/mapping.toml)
            (AppIcon::PlayPause, IconSet::Freedesktop) => {
                Some("media-playback-start")
            }
            (AppIcon::Thermometer, IconSet::Freedesktop) => {
                Some("sensors")
            }
            (AppIcon::EyeToggle, IconSet::Freedesktop) => {
                // DE-aware: inline table in mapping.toml
                match native_theme::detect_linux_de(
                    &std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default(),
                ) {
                    native_theme::LinuxDesktop::Kde => Some("view-visible"),
                    _ => Some("view-reveal"), // default from mapping.toml
                }
            }
            // ...

            _ => None,
        }
    }

    fn icon_svg(&self, id: &AppIcon, set: IconSet) -> Option<&[u8]> {
        use native_theme::IconSet;
        match (id, set) {
            // Material SVGs (auto-generated from directory contents).
            // Paths use CARGO_MANIFEST_DIR because this file lives in OUT_DIR,
            // so relative paths would resolve incorrectly.
            (AppIcon::PlayPause, IconSet::Material) => Some(include_bytes!(
                concat!(env!("CARGO_MANIFEST_DIR"), "/icons/material/play_pause.svg")
            )),
            (AppIcon::SkipForward, IconSet::Material) => Some(include_bytes!(
                concat!(env!("CARGO_MANIFEST_DIR"), "/icons/material/skip_next.svg")
            )),
            // ...

            // Lucide SVGs
            (AppIcon::PlayPause, IconSet::Lucide) => Some(include_bytes!(
                concat!(env!("CARGO_MANIFEST_DIR"), "/icons/lucide/play.svg")
            )),
            // ...

            // System themes: no SVGs, return None
            _ => None,
        }
    }
}
```

The app includes it with:

```rust
// src/icons.rs
include!(concat!(env!("OUT_DIR"), "/app_icons.rs"));
```

#### Build-time validation (strict)

The build system enforces completeness at every level. Nothing is optional,
nothing is silently skipped. The goal: if a designer forgets to add an icon
for one theme, the project does not compile.

| Check | Condition | Result |
|---|---|---|
| Missing theme subdirectory | `bundled-themes` or `system-themes` lists `"phosphor"` but `icons/phosphor/` doesn't exist | **Error** |
| Missing mapping.toml | `icons/material/` exists but has no `mapping.toml` | **Error** |
| Missing role in mapping | Role `"bluetooth"` is in master TOML but not in `icons/sf-symbols/mapping.toml` | **Error** |
| Missing SVG in bundled theme | `icons/material/mapping.toml` has `bluetooth = "bluetooth"` but `icons/material/bluetooth.svg` doesn't exist | **Error** |
| Unknown role in mapping | `icons/material/mapping.toml` has `quantum = "quantum"` but `"quantum"` is not in the master role list | **Error** |
| Invalid TOML syntax | Malformed TOML in any file | **Error** with file path and line number |
| Naming conflict | Two roles map to the same SVG name in the same theme | **Error** |
| Missing `default` in DE-aware table | `eye-toggle = { kde = "view-visible" }` has no `default` key | **Error** |
| Unknown DE key in inline table | `eye-toggle = { kde = "...", windows = "..." }` — `windows` is not a recognized DE | **Error** |
| Orphan SVG files | `icons/material/old_icon.svg` exists but no mapping entry references it | **Warning** |
| SVG in system theme directory | `icons/freedesktop/something.svg` exists (unexpected) | **Warning** |

Every error message includes the file path, the role name, and the theme name.
Example:

```
error[native-theme-build]: role "thermometer" missing from icons/segoe-fluent/mapping.toml
  --> icons/custom-icons.toml
  |
  = note: every role must appear in every theme's mapping.toml
  = help: add `thermometer = "icon-name"` with the closest matching icon
```

#### Adding a new role: the workflow

1. Add `"new-role"` to the `roles` list in `custom-icons.toml`
2. `cargo build` → fails with errors listing every theme that's missing it
3. Add `new-role = "icon_name"` to each theme's `mapping.toml`
4. For bundled themes, add the SVG file
5. `cargo build` → succeeds

The build system acts as a checklist enforcer. You can't ship a half-done icon.

#### Adding a new theme: the workflow

1. Add `"phosphor"` to `bundled-themes` (or `system-themes`) in master TOML
2. Create `icons/phosphor/` directory
3. `cargo build` → fails: missing `mapping.toml`
4. Create `icons/phosphor/mapping.toml` with all role entries
5. For bundled themes: add all SVG files
6. `cargo build` → succeeds

#### Pros

- **Zero Rust boilerplate for icon definitions.** The app developer edits TOML
  and places SVG files — no trait impls, no match arms, no `include_bytes!`
- **Strict completeness enforcement.** Every role must be covered in every
  theme. Missing anything = build error. Catches designer laziness immediately.
- **Clean ownership model.** Each theme's directory is self-contained. A
  designer or platform specialist owns their mapping.toml.
- **The CLI fetch tool can write directly into theme directories** — download
  SVGs + update the mapping.toml in one operation
- **Still produces an `IconProvider` impl** — interoperable with all existing
  loading and connector functions
- **Community icon packs** can ship as just a set of theme directories, with
  the master TOML listing their roles

#### Cons

- **Requires a build dependency** (`native-theme-build`) — adds to compile
  time, though TOML parsing + codegen is fast
- **`include!` is less IDE-friendly** than regular source — autocomplete for
  the generated enum requires IDE support for build script outputs
- **More files to manage** — 1 master TOML + N theme mapping files. For 5
  themes + 1 master = 6 TOML files. This is intentional (ownership), but
  more than the single-file approach.
- **`build.rs` reruns**: needs `cargo:rerun-if-changed` for all TOML files
  and SVG directories

## Analysis

### Dimension 1: Who Provides the Icon Data?

| Approach | SVG source | Name mapping source | Rust code written by app dev? |
|---|---|---|---|
| A (Trait) | App bundles SVGs | App writes Rust match arms | Yes — trait impl |
| B (Macro) | App bundles SVGs | App writes Rust macro invocation | Yes — macro syntax |
| C (Registry) | App bundles SVGs | App writes Rust builder calls | Yes — builder calls |
| D (Features) | Crate bundles SVGs | Crate provides mappings | No |
| E (Hybrid) | Both | Both | Sometimes |
| F (TOML) | App bundles SVGs | App writes TOML | No |

Approaches A/B/C require the app developer to write Rust code for icon
definitions. The amount varies (A is the most verbose, B the most compact),
but all three require understanding the `IconProvider` trait and writing match
arms or equivalent.

Approach F eliminates this entirely. The app developer writes TOML files — a
format they already know from Cargo.toml — and drops SVG files into
directories. Each theme has its own mapping file, owned by the person
responsible for that theme. The build system generates all the Rust code and
enforces that no mapping or SVG is missing. This is the same UX as approach D
(feature flags) in terms of developer effort, but without the scaling problems.

### Dimension 2: Discoverability

How does an app developer find the right icon name for each platform?

This is the same regardless of approach. The app developer needs to:

1. Open the Material Symbols browser, search for "play", download the SVG
2. Open the Lucide icon browser, search for "play", download the SVG
3. Look up the freedesktop Icon Naming Specification for "media-playback-start"
4. Search SF Symbols in Xcode for "play.fill"
5. Search Segoe Fluent Icons documentation for "Play"

This is tedious but unavoidable for approaches A/B/C/F. A companion CLI tool
can automate it — e.g., searching icon repos by keyword, downloading SVGs, and
writing the mapping.toml entries. With approach F, the CLI tool writes directly
into each theme's subdirectory (SVG + mapping entry), which is exactly what the
build system consumes. No Rust code to write at all.

For approach D, the crate maintainer does this work once and all apps benefit.

### Dimension 3: Compile-Time Safety

| Approach | SVG exists? | All sets covered? | Name typos? |
|---|---|---|---|
| A (Trait) | Yes (include_bytes) | No (silent fallback) | Partial (SVG path, not names) |
| B (Macro) | Yes (include_bytes) | Could warn | Yes (if macro validates) |
| C (Registry) | Yes (include_bytes) | Runtime check at build() | Runtime only |
| D (Features) | Yes (crate bundles) | Yes (crate maintains) | Yes (enum match) |
| E (Hybrid) | Both | Partial | Partial |
| F (TOML) | Yes (build.rs checks) | Yes (build.rs **errors**) | Yes (build.rs validates) |

Approach F has the strictest compile-time safety of all approaches. The build
script enforces that every role is present in every theme's mapping.toml, and
that every bundled mapping has a corresponding SVG file. Missing anything is a
build error, not a warning. There is no escape hatch — every role must have a
real icon name in every theme. This eliminates both the "forgot to add it" and
the "lazy designer left it blank" class of bugs entirely.

### Dimension 4: Impact on Binary Size

Material SVGs average ~500 bytes. Lucide SVGs average ~350 bytes.

| Scenario | Approx. size increase |
|---|---|
| 10 extra icons x 2 sets | ~17 KB |
| 50 extra icons x 2 sets | ~85 KB |
| 200 extra icons x 2 sets | ~340 KB |
| All ~4,200 Material icons | ~2.1 MB |
| All ~1,700 Lucide icons | ~600 KB |

All app-driven approaches (A/B/C/F) are equivalent here: the app controls
exactly which icons are bundled. Only the SVG files present in the directory
are compiled in.

For approach D, the crate ships all icons for every enabled feature flag,
regardless of which ones the app actually uses.

### Dimension 5: Handling Missing Icons

What happens when an icon doesn't exist in a particular icon set?

Example: the app needs a "quantum-computer" icon. Material has a "quantum"
symbol in their extended set, but Lucide doesn't. No freedesktop theme has
anything close. SF Symbols has nothing.

**For system sets** (freedesktop, SF Symbols, Segoe): if the name returns
nothing from the system API, the fallback chain kicks in. The app gets the
bundled Material or Lucide version instead. The icon looks slightly out of
place (different style than the surrounding native icons) but is functional.

**For bundled sets** (Material, Lucide): if the app provides an SVG for
Material but not Lucide, and the user selects Lucide as their theme, the
system falls back. The question is: fall back to what?

Fallback chain for a custom icon with active theme Lucide:
1. Try Lucide SVG from the provider → not found
2. Try system icon by Lucide name → N/A (Lucide is bundled, not system)
3. Try fallback set (Material by default) → found, use it
4. If nothing: return None, let the app handle it (placeholder, hide, error)

This is acceptable. The app developer should be encouraged to provide SVGs for
at least their primary bundled set (Material) so there's always a fallback.
Providing both Material and Lucide is better. Providing all 5 sets gives the
best experience.

With approach F, this is handled strictly: every theme's mapping must have an
entry for the role with a real icon name. Even for exotic icons like
"quantum-computer," the theme maintainer must find the closest reasonable
equivalent (e.g., `quantum-computer = "science"` in freedesktop, or
`quantum-computer = "ComputerChip"` in Segoe Fluent). The build system forces
this — you can't skip a theme, and you can't leave a gap. If there's truly
nothing close, the maintainer must make a judgment call on the best available
icon, because a slightly imperfect icon is always better than a missing one.

### Dimension 6: The Freedesktop Complication

Freedesktop is unique among the icon sets because:

1. **Icons are loaded by name from the filesystem**, not bundled
2. **Name mappings may need to be DE-aware** (KDE vs GNOME vs Cinnamon)
3. **The app can't know at compile time which theme is installed**
4. **Theme coverage varies wildly** — Breeze has ~7,100 icons, Adwaita has ~790

This means a custom icon might work perfectly on KDE (Breeze has it) and fall
back to bundled on GNOME (Adwaita doesn't). That's fine — the existing 42
`IconRole` variants already have this characteristic.

Approach F handles this naturally via TOML inline tables:

```toml
eye-toggle = { kde = "view-visible", default = "view-reveal" }
```

The build script generates the appropriate DE-aware branch in the
`icon_name()` impl. Approaches A/B/C leave this to the app developer to
implement manually in their Rust code.

## Build-Time Downloads: Why Not — and When Yes

The original constraint was "fully offline." But that conflates two very
different things:

1. **Runtime downloads** — the shipped binary contacts the network. This is
   ruled out: the app must work in air-gapped environments, on first launch,
   without ever phoning home.

2. **Build-time downloads** — a `build.rs` script or xtask fetches icons from
   the internet during compilation. The result is compiled into the binary,
   which is then fully offline.

These have fundamentally different trust models. Runtime downloads affect end
users. Build-time downloads affect developers.

### Why build-time downloads in `build.rs` are problematic

Even though the binary would be offline, network access in `build.rs` has real
costs:

- **Non-reproducible builds.** The same source tree may produce different
  binaries depending on when it was built, whether the CDN was up, or whether
  the icon repo restructured its URLs. This violates the expectation that
  `cargo build` is deterministic given the same inputs.

- **Air-gapped CI.** Many organizations build in environments without internet
  access. A `build.rs` that requires the network is a hard blocker for those
  teams.

- **crates.io policy.** If the icon-fetching code is in a library crate
  published to crates.io, the crate must be self-contained. `build.rs` cannot
  fetch external resources during `cargo install` or downstream builds.

- **Fragility.** Icon repos move, rename directories, change CDN providers.
  A build that worked last month may break today for reasons entirely outside
  the developer's control.

- **Slow builds.** Network I/O in the critical path of compilation adds
  latency, especially on slow connections or when building frequently during
  development.

### When build-time downloads ARE appropriate

Build-time downloads are fine when they're **a developer action, not a build
action** — i.e., when the developer runs a tool explicitly, inspects the
result, and commits the downloaded files to version control.

This is the `cargo vendor` model: you run a command once, it downloads
dependencies into your tree, and from that point on builds are fully hermetic.

#### The recommended workflow: a CLI tool

A CLI tool (cargo subcommand or standalone binary) that:

1. Takes a list of icon names and target icon sets
2. Downloads the SVG files from each icon set's canonical source
3. Places them in the app's `icons/` directory
4. Prints the license obligations for each downloaded file
5. The developer commits the SVGs and their licenses to version control

```
$ cargo native-theme-icons fetch bluetooth \
    --material --lucide --output icons/

Fetched 2 icons:
  icons/material/bluetooth.svg  (Apache-2.0 — include LICENSE-MATERIAL)
  icons/lucide/bluetooth.svg    (ISC — include LICENSE-LUCIDE)

Remember to commit these files and their license files.
```

After this, the app uses `include_bytes!("icons/material/bluetooth.svg")` in
its `IconProvider` impl. No network access at build time. Fully reproducible.

#### Why this is better than manual download

- **Correct naming.** The tool knows that Material uses `bluetooth.svg` while
  Lucide uses `bluetooth.svg` (same name, different directory) but freedesktop
  uses `bluetooth-symbolic.svg`. It handles the naming conventions.

- **License tracking.** The tool includes the correct license file for each
  icon set and can generate a `NOTICES` file.

- **Batch operations.** `cargo native-theme-icons fetch play pause stop
  skip-forward skip-back --material --lucide` gets 10 SVGs in one command.

- **Validation.** The tool can warn when an icon doesn't exist in a requested
  set, suggest alternatives, or verify that the downloaded SVG is valid.

This tool is a separate concern from the `IconProvider` trait. It assists the
developer workflow but doesn't change the crate's architecture. It could ship
as a companion binary crate (`native-theme-cli`) or even a standalone tool.

## Licensing: What Can Apps Actually Bundle?

The `native-theme` crate is licensed `MIT OR Apache-2.0 OR 0BSD`. It bundles
only permissively-licensed icons (Material under Apache-2.0, Lucide under ISC).

But when an app implements `IconProvider` and uses `include_bytes!` to embed
SVG files, the **app** is the one doing the bundling. The licensing obligations
fall on the app, not on the native-theme crate. This is an important
distinction.

### License Compatibility Matrix for App Bundling

| Icon Set | License | Can App Bundle? | Obligations |
|---|---|---|---|
| Material Symbols | `Apache-2.0` | Yes | Include license file |
| Lucide | `ISC` | Yes | Include copyright notice |
| Phosphor | `MIT` | Yes | Include copyright notice |
| Tabler Icons | `MIT` | Yes | Include copyright notice |
| Bootstrap Icons | `MIT` | Yes | Include copyright notice |
| Heroicons | `MIT` | Yes | Include copyright notice |
| Tango | Public Domain | Yes | None |
| Font Awesome Free | `CC-BY-4.0` | Yes | Attribution required (About dialog, NOTICES file) |
| Breeze (KDE) | `LGPL-3.0` | Complicated | See below |
| Adwaita (GNOME) | `LGPL-3.0` or `CC-BY-SA-3.0` | Complicated | See below |
| Oxygen (KDE) | `LGPL-3.0` | Complicated | See below |
| Papirus | `GPL-3.0` | Only if app is GPL | Full copyleft applies |
| elementary | `GPL-3.0` | Only if app is GPL | Full copyleft applies |
| Numix | `GPL-3.0` | Only if app is GPL | Full copyleft applies |

### The Permissive Tier (Safe to Bundle)

Material, Lucide, Phosphor, Tabler, Bootstrap, Heroicons, and Tango can be
bundled in any application regardless of its license. The only obligation is
including the copyright/license notice, which is standard practice.

Together, these sets cover ~15,000+ unique icon concepts — more than enough for
virtually any application domain. Between Material (~4,200 icons) and Lucide
(~1,700 icons), most common UI concepts are available.

### The LGPL Tier (Breeze, Adwaita, Oxygen)

LGPL-3.0 for artwork is a gray area. The key questions:

**Can you `include_bytes!` an LGPL SVG into a proprietary binary?**

The LGPL was designed for software libraries with a "linking" model. For
artwork, the analogy is imperfect. Two interpretations exist:

1. **Strict reading:** Embedding SVG bytes via `include_bytes!` is analogous
   to static linking. The LGPL requires that the user can replace the linked
   library. With `include_bytes!`, the only way to replace the icon is to
   recompile the application. This may violate LGPL section 4(d)(0), which
   requires providing a mechanism for the user to swap the library.

2. **Pragmatic reading:** The KDE project's position (stated in the Breeze
   icons README and KDE licensing FAQ) is that using icons in your application
   is permitted, analogous to displaying a library's output. The SVG source
   is available in the icon theme's repository, satisfying the source
   availability requirement. The user can modify the installed icon theme
   to change icons system-wide.

**Practical guidance:**

- **If the app reads Breeze/Adwaita from the filesystem at runtime** (via the
  freedesktop loader), there is no licensing issue. The app never distributes
  the icons — they're installed by the user's package manager. This is the
  primary path and needs no special handling.

- **If the app wants to bundle Breeze/Adwaita SVGs** (e.g., for a "Breeze
  theme" option on macOS/Windows where the icons aren't installed), the safest
  approach is:
  - Ship the SVGs as separate resource files (not compiled in), so the user
    can replace them
  - Include the LGPL license text
  - Make the SVG source files available (trivially satisfied if they're shipped
    as SVGs, not rasterized)
  - Or: get explicit permission from the icon set maintainers

- **For the native-theme crate itself:** the crate must NOT bundle LGPL icons.
  The crate's triple license (MIT/Apache-2.0/0BSD) is incompatible with LGPL
  distribution requirements. This is why only Material and Lucide are bundled.

### The GPL Tier (Papirus, elementary, Numix)

GPL-3.0 for artwork means any application that distributes these icons must
itself be GPL-3.0 (or compatible). There is no "linking exception" for artwork
in the GPL.

- **GPL-licensed apps:** can freely bundle Papirus, elementary, or Numix icons
- **Non-GPL apps:** cannot bundle these icons in any form

However, **reading GPL icons from the filesystem at runtime** (e.g., the user
has Papirus installed as their system theme) is not distribution. The app never
copies or redistributes the icons. The freedesktop loader path is safe
regardless of the icon theme's license.

### What This Means for the IconProvider Design

The trait-based approach naturally separates the licensing concerns:

1. **The crate** (MIT/Apache-2.0/0BSD) provides the trait, loading functions,
   and only permissively-licensed bundled icons.

2. **The app** implements `IconProvider` and decides which icons to bundle.
   The app's license determines what it can legally include.

3. **System icon loading** (freedesktop, SF Symbols, Segoe) sidesteps
   licensing entirely — the icons are already on the user's machine.

This means an `IconProvider` impl might look different depending on the
app's license:

```rust
// MIT-licensed app — bundles only permissive icons
impl IconProvider for MyIcons {
    fn icon_svg(&self, id: &MyIcon, set: IconSet) -> Option<&[u8]> {
        match (id, set) {
            // Safe: Material is Apache-2.0
            (MyIcon::Play, IconSet::Material) =>
                Some(include_bytes!("icons/material/play_arrow.svg")),
            // Safe: Lucide is ISC
            (MyIcon::Play, IconSet::Lucide) =>
                Some(include_bytes!("icons/lucide/play.svg")),
            // Freedesktop/SF Symbols/Segoe: no SVG needed, just name mapping
            // The system loader handles it
            _ => None,
        }
    }
}
```

```rust
// GPL-licensed app — can bundle anything
impl IconProvider for MyIcons {
    fn icon_svg(&self, id: &MyIcon, set: IconSet) -> Option<&[u8]> {
        match (id, set) {
            (MyIcon::Play, IconSet::Material) =>
                Some(include_bytes!("icons/material/play_arrow.svg")),
            (MyIcon::Play, IconSet::Lucide) =>
                Some(include_bytes!("icons/lucide/play.svg")),
            // GPL app can bundle Breeze icons for use on non-Linux platforms
            (MyIcon::Play, IconSet::Freedesktop) =>
                Some(include_bytes!("icons/breeze/media-playback-start.svg")),
            _ => None,
        }
    }
}
```

### The Practical Recommendation

For most apps, the path of least resistance is:

1. **Bundle Material + Lucide** SVGs (permissive) for cross-platform fallback
2. **Provide freedesktop name mappings** — the system loader reads from the
   installed theme at runtime, zero licensing concern
3. **Provide SF Symbols / Segoe name mappings** — OS-provided, zero licensing
   concern
4. **Don't bundle freedesktop theme icons** unless the app is GPL

This gives full theme-switching with no legal ambiguity. On Linux, the user
gets native Breeze/Adwaita icons from their installed theme. On macOS/Windows,
they get Material or Lucide.

If an app specifically wants a "Breeze look" on macOS, that's the app
developer's licensing decision — not something the native-theme crate should
facilitate or prevent.

## Recommendation

**Use Approach F (TOML-driven code generation) as the primary developer
experience, with the `IconProvider` trait (Approach A) as the underlying
mechanism.**

The trait is the runtime API. The TOML + build script is the developer-facing
API. The app developer never writes `impl IconProvider` manually — they write
a TOML file, place SVG files in directories, and the build system generates
everything.

For advanced use cases (runtime-registered plugins, dynamic icon sources), the
trait remains directly implementable.

### Rationale

1. **Minimal friction for the common case.** The app developer's workflow is:
   edit a TOML file, drop SVGs in a directory, done. No Rust boilerplate for
   icon definitions. This is comparable to approach D (feature flags) in
   developer effort, without the scaling or maintenance problems.

2. **The TOML file is the single source of truth.** It's human-readable,
   diffable, and editable by non-programmers (designers who curate icon sets).
   It serves as both configuration and documentation.

3. **Strict build-time enforcement eliminates lazy mistakes.** Every role must
   be present in every theme's mapping with a real icon name. Every bundled
   mapping must have its SVG file. There is no way to mark a role as "not
   applicable" — every theme must provide something. The build fails immediately
   when anything is missing, with a message pointing to exactly what's needed.
   This catches both the "designer forgot one theme" and the "designer got lazy
   and skipped it" bugs at compile time, not in production.

4. **Scales to exotic icons without growing the crate.** The native-theme
   crate ships the trait and the build helper. All icon definitions live in the
   app (or in shareable TOML+SVG packages).

5. **Offline-by-construction.** SVGs are committed to version control and
   compiled in via `include_bytes!`. System icon names are strings. No network
   access at build time or runtime.

6. **Naturally integrates with the CLI fetch tool.** The tool's output is a
   TOML file + SVG directory — exactly what `build.rs` consumes. The developer
   runs one command and gets buildable output.

### What the Crate Should Ship

**In `native-theme` (core crate):**

1. **The `IconProvider` trait** in `native-theme/src/model/`, with:
   - `fn icon_name(&self, id, set) -> Option<&str>` — name mappings
   - `fn icon_svg(&self, id, set) -> Option<&[u8]>` — bundled SVG bytes
   - `fn fallback_set(&self, id) -> Option<IconSet>` — default fallback
   (The generated enum provides iteration via `const ALL: &[Self]` — this
   doesn't need to be on the trait, as it's a property of the concrete type.)

2. **A `load_custom_icon` function** in `lib.rs` that takes a provider and
   dispatches through the same fallback chain as `load_icon`.

3. **A `load_system_icon_by_name(name: &str, set: IconSet) -> Option<IconData>`
   function** that wraps the platform-specific loaders for arbitrary names
   (not just `IconRole`). This already partially exists — the freedesktop
   loader, SF Symbols loader, and Segoe loader all take name strings internally.
   This just needs to be exposed publicly.

**In `native-theme-build` (build helper crate):**

4. **`generate_icons(toml_path)` function** for use in app `build.rs`. Reads
   the TOML, validates SVG files, generates the enum + trait impl + include
   directives. Emits `cargo:rerun-if-changed` for the TOML and SVG
   directories.

5. **Build-time validation** — errors for missing SVGs, warnings for
   incomplete theme coverage, errors for naming conflicts.

**In connectors (`native-theme-gpui`, `native-theme-iced`):**

6. **Generic helpers** that work with any `IconProvider`:
   ```rust
   pub fn custom_icon_to_image_source<P: IconProvider>(
       provider: &P,
       id: &P::IconId,
       icon_set: &str,
   ) -> Option<ImageSource>
   ```
   And a colored variant. These are thin wrappers around `load_custom_icon` +
   the existing `to_image_source`.

### What the Crate Should NOT Ship

- **Feature-gated `IconRole` extensions.** This creates a maintenance and
  versioning burden disproportionate to the benefit. Apps that want curated
  icon packs can create shared crates (e.g., `native-theme-media-icons`) that
  ship a TOML + SVG directory.

- **Runtime download infrastructure.** Explicitly ruled out.

- **LGPL/GPL icon SVGs.** The crate's triple license (MIT/Apache-2.0/0BSD)
  is incompatible. Only permissively-licensed icons (Material, Lucide) should
  be bundled in the crate itself.

### What Could Ship Separately

- **`native-theme-cli`** — a companion binary crate that downloads SVGs from
  icon repositories and generates TOML definitions. Outputs are committed to
  version control. See the "Build-Time Downloads" section for the workflow.

- **Community icon packs** — crates or plain repositories that ship a TOML file
  and SVG directories for specific domains. Apps include them by copying the
  files (or depending on the crate) and pointing `build.rs` at the TOML.

### Ecosystem Implications

The TOML + trait approach enables a layered ecosystem:

- `native-theme` — core crate with 42 `IconRole` variants and `IconProvider`
  trait
- `native-theme-build` — build helper for TOML → code generation
- `native-theme-gpui` / `native-theme-iced` — connectors with generic helpers
- `native-theme-cli` — developer tool for fetching SVGs + generating TOML
- `native-theme-media-icons` — community TOML+SVG pack for media playback
- `native-theme-dev-icons` — community pack for git, terminal, debug icons
- App-specific TOML+SVG directories for exotic icons

Community packs don't need to be Rust crates at all — they could be plain Git
repositories with just a TOML file and SVG directories. The app copies them
into its tree and points `build.rs` at the TOML.

### Resolved Design Decisions

1. **`IconProvider` does NOT require `Send + Sync`.**

   Generated providers are unit structs — they are `Send + Sync` automatically.
   Hand-written providers holding only static data are too. Adding the bound to
   the trait would prevent rare but legitimate use cases (e.g., a provider backed
   by a `RefCell` for hot-reload during development). Functions that need thread
   safety can add the bound at the call site:
   `fn load<P: IconProvider + Send + Sync>(...)`. This follows standard Rust
   practice — traits impose the minimum bounds, callers add what they need.

2. **Single fallback set (`Option<IconSet>`), not a chain.**

   The build system guarantees that every role has an SVG in every bundled
   theme. The default fallback is `Some(IconSet::Material)`, and Material is
   always a bundled theme with complete coverage. Therefore
   `icon_svg(id, Material)` always succeeds, and a multi-step chain solves a
   problem that can't arise under the strictness guarantee. If a future use
   case requires it, `fallback_set` can be extended to return
   `&[IconSet]` without breaking existing impls (the single-set return is a
   subset of a slice).

3. **Yes: ship `load_system_icon_by_name` on all platforms.**

   This is not optional — it's required for the design to work. Without it,
   the SF Symbols and Segoe Fluent name mappings in the TOML are dead
   configuration. The `load_custom_icon` function calls
   `load_system_icon_by_name(name, set)` for every icon set, but only the
   freedesktop loader currently accepts arbitrary name strings
   (`load_freedesktop_icon_by_name` already exists). The SF Symbols and Segoe
   loaders must be refactored:

   - **SF Symbols:** Pass the name string directly to
     `NSImage(systemSymbolName:accessibilityDescription:)`. This is a one-line
     change — the existing loader already calls this API, just with a
     role-to-name lookup in front.
   - **Segoe Fluent:** Add a name-to-codepoint lookup. The existing loader
     uses `GetGlyphOutlineW` with codepoints derived from `IconRole`. A
     `HashMap<&str, u32>` mapping glyph names to codepoints is straightforward;
     the Segoe Fluent Icons documentation provides the full mapping.

   Without this refactoring, custom icons on macOS always fall back to bundled
   Material/Lucide (ignoring the SF Symbols mapping), and on Windows always
   fall back (ignoring the Segoe mapping). The user experience would be
   degraded on two of three major platforms.

4. **Yes: support multiple TOML files via a builder API.**

   Apps should be able to compose their own icons with community packs:

   ```rust
   // build.rs
   native_theme_build::IconGenerator::new()
       .add("icons/app-icons.toml")
       .add("third-party/media-icons/icons.toml")
       .generate();
   ```

   The builder merges role lists from all sources into a single enum. Build-time
   checks:
   - **Role name conflict across files** → error (two files both define
     `"play-pause"` with different theme mappings)
   - **Theme mismatch** → error (file A declares `bundled-themes = ["material"]`
     but file B declares `bundled-themes = ["material", "phosphor"]` — the union
     of bundled themes must be consistent, or each file's theme declarations
     must be a subset of a unified set)

   The generated enum name comes from the first TOML's `name` field, or can be
   overridden: `.enum_name("AppIcon")`.

   The simpler single-file API remains as a convenience:
   ```rust
   native_theme_build::generate_icons("icons/custom-icons.toml");
   ```

5. **Document binary size in `native-theme-build` docs; emit a build report.**

   The build script already parses every SVG file. It should print a summary:

   ```
   note[native-theme-build]: 25 roles × 2 bundled themes = 50 SVGs, 21.3 KB total
   ```

   Document expected per-icon sizes in the crate-level docs:

   | Icon set | Average SVG size | 20 icons | 50 icons | 200 icons |
   |----------|-----------------|----------|----------|-----------|
   | Material | ~500 bytes | ~10 KB | ~25 KB | ~100 KB |
   | Lucide | ~350 bytes | ~7 KB | ~18 KB | ~70 KB |

6. **Use `OUT_DIR` via `include!` exclusively.**

   This is the standard Rust convention used by prost, tonic, bindgen, and
   sqlx. Modern rust-analyzer indexes `OUT_DIR` by default, providing
   autocomplete and go-to-definition for generated types. Generating into
   `src/` creates `.gitignore` discipline problems and merge conflicts when
   multiple developers build with different configurations. The `OUT_DIR`
   approach is simpler, has one canonical path, and avoids these issues.
