# Phase 16: Icon Data Model - Research

**Researched:** 2026-03-09
**Domain:** Rust enum design, static mapping tables, serde/TOML integration
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **IconRole design:** Flat enum with prefixed naming (`IconRole::DialogError`, `IconRole::ActionCopy`, `IconRole::WindowClose`, etc.). No nested sub-enums or category wrapper -- the prefix IS the category. Closed enum with `#[non_exhaustive]` -- exactly 42 predefined variants, no `Custom(String)`. Custom/app-specific icons are out of scope.
- **IconData shape:** Claude's Discretion
- **Name mapping architecture:** Claude's Discretion
- **TOML integration:** Claude's Discretion

### Claude's Discretion
- **IconRole derives:** Choose appropriate derive set (serde, strum, etc.) based on what's actually needed
- **IconRole variant list:** Curate the 42 roles based on cross-platform coverage -- icons that exist across all major sets get priority
- **IconData ownership:** Owned Vec<u8> vs Cow<[u8]> -- decide based on how bundled static SVGs vs loaded icons interact
- **IconData color/tint:** Whether to include tint metadata or keep it the connector's responsibility
- **IconData size hint:** Whether SVG variant carries a nominal size or size is purely a load parameter
- **Mapping strategy:** Static match tables vs phf hash maps -- pick based on performance/simplicity
- **Unmapped roles:** Whether icon_name() returns Option<&str> or always returns something with best-effort approximations
- **IconSet extensibility:** Fixed 5 sets with #[non_exhaustive] vs allowing custom sets
- **system_icon_set() placement:** This phase (simple cfg) vs Phase 21 (with runtime checks)
- **TOML field type:** String vs typed IconSet enum for icon_theme field
- **TOML field placement:** Per-variant (light/dark each have icon_theme) vs top-level theme-wide
- **Preset defaults:** Whether existing presets ship with icon_theme values or leave them None

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| ICON-01 | IconRole enum with 42 semantic icon roles across 7 categories | Variant list curated from docs/native-icons.md availability matrix; derive set analyzed |
| ICON-02 | IconData enum with Svg(Vec<u8>) and Rgba { width, height, data } | Ownership model analyzed (Vec<u8> recommended over Cow); tint/size decisions made |
| ICON-03 | icon_theme field (Option<String>) on ThemeVariant with preset defaults | ThemeVariant struct analyzed; per-variant placement with merge support documented |
| ICON-04 | icon_name() function for platform-specific identifier lookup | Match-based mapping recommended over phf; all 5x42=210 mappings sourced from docs |
| ICON-05 | system_icon_set() function for OS-native icon set resolution | Simple cfg!() approach fits this phase; implementation pattern documented |
</phase_requirements>

## Summary

Phase 16 defines the pure data model layer for icon support in native-theme. It introduces three new types (`IconRole`, `IconData`, `IconSet`), a static mapping function (`icon_name()`), a platform detection function (`system_icon_set()`), and a new `icon_theme` field on `ThemeVariant` with TOML deserialization support. This is a zero-dependency phase -- no I/O, no platform APIs, no rendering.

The existing codebase provides strong patterns to follow: `#[non_exhaustive]` is already used on all model structs and the `Error` enum, serde derives are standard throughout, and the `impl_merge!` macro handles field overlay logic. The `ThemeVariant` struct is the natural home for the `icon_theme` field, placed per-variant (light/dark) since different variants could theoretically use different icon sets.

All 42 icon roles have been pre-researched in `docs/native-icons.md` with complete availability matrices and identifier strings for all 5 icon sets (SF Symbols, Segoe Fluent, freedesktop, Material, Lucide). The mapping data is fully specified -- implementation is mechanical transcription into `match` arms.

**Primary recommendation:** Use simple `match` statements for all mappings (42 variants x 5 icon sets), keep `IconData` with owned `Vec<u8>` (no Cow), add `icon_theme: Option<String>` per-variant on `ThemeVariant`, and place everything in a new `native-theme/src/model/icons.rs` module.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| serde | 1.0.228 (workspace) | Derive Serialize/Deserialize for IconSet | Already used throughout codebase |
| toml | 1.0.6 (workspace) | TOML parsing for icon_theme field | Already used throughout codebase |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| (none needed) | - | - | This phase requires no new dependencies |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| match tables | phf 0.13.1 | phf adds a dependency for constant-time lookup, but for 42 entries match is compiled to a jump table or similar by LLVM and is equally fast with zero deps |
| manual Display | strum 0.28.0 | strum adds enum-string conversion derives, but we only need icon_name() which is a custom mapping per icon set, not a simple Display impl |

**Installation:**
```bash
# No new dependencies needed -- all workspace deps already present
```

## Architecture Patterns

### Recommended Project Structure
```
native-theme/src/
  model/
    mod.rs           # Add: pub mod icons; pub use icons::{IconRole, IconData, IconSet};
    icons.rs         # NEW: IconRole, IconData, IconSet enums + icon_name() + system_icon_set()
    colors.rs        # existing
    fonts.rs         # existing
    ...
  presets/
    *.toml           # Updated: native presets get icon_theme field
  lib.rs             # Updated: re-export icon types
```

### Pattern 1: Non-Exhaustive Enum with Derives
**What:** Define IconRole, IconData, and IconSet as `#[non_exhaustive]` enums matching the project's existing pattern.
**When to use:** For all three new types -- they are public API types that may grow in future versions.
**Example:**
```rust
// Follows existing pattern from model/mod.rs (ThemeVariant) and error.rs (Error)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IconRole {
    // Dialog / Alert
    DialogWarning,
    DialogError,
    DialogInfo,
    DialogQuestion,
    DialogSuccess,
    Shield,
    // ... 36 more variants
}
```

### Pattern 2: Static Match Mapping
**What:** Use `match` statements returning `Option<&'static str>` for icon name lookups. One function dispatches on `IconSet`, calling per-set mapping functions.
**When to use:** For `icon_name()` -- maps (IconSet, IconRole) pairs to platform-specific identifier strings.
**Example:**
```rust
pub fn icon_name(set: IconSet, role: IconRole) -> Option<&'static str> {
    match set {
        IconSet::SfSymbols => sf_symbols_name(role),
        IconSet::SegoeIcons => segoe_name(role),
        IconSet::Freedesktop => freedesktop_name(role),
        IconSet::Material => material_name(role),
        IconSet::Lucide => lucide_name(role),
    }
}

fn sf_symbols_name(role: IconRole) -> Option<&'static str> {
    Some(match role {
        IconRole::DialogWarning => "exclamationmark.triangle.fill",
        IconRole::DialogError => "xmark.circle.fill",
        IconRole::ActionCopy => "doc.on.doc",
        // ... all 42 variants
        _ => return None,  // required by #[non_exhaustive]
    })
}
```

### Pattern 3: ThemeVariant Field Addition
**What:** Add `icon_theme: Option<String>` to `ThemeVariant` following the existing optional field pattern with serde skip_serializing_if.
**When to use:** For ICON-03 -- TOML integration.
**Example:**
```rust
// In model/mod.rs, ThemeVariant struct:
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeVariant {
    // ... existing fields ...

    /// Icon set to use for this variant (e.g., "sf-symbols", "material").
    /// When None, resolved at runtime via system_icon_set().
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_theme: Option<String>,
}
```

### Anti-Patterns to Avoid
- **Nested enums for categories:** Don't create `IconCategory::Dialog(DialogIcon::Error)` -- the user locked the decision on flat prefixed naming.
- **Custom(String) variant:** Don't add catch-all string variants to IconRole -- the user explicitly excluded custom icons.
- **Cow<[u8]> in IconData:** Don't use `Cow<'a, [u8]>` -- it adds a lifetime parameter that infects every struct holding an `IconData`, making the API harder to use. Bundled SVGs (Phase 17) can cheaply `.to_vec()` from static `&[u8]` slices; the allocation is tiny (~2-5KB per icon).
- **Typed IconSet for TOML field:** Don't use `Option<IconSet>` for the TOML field -- user TOML files may contain icon set names that don't map to a known enum variant (e.g., a future icon set or a user-defined one). Keep it `Option<String>` for flexibility.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Enum iteration/count | Custom iterator, `const COUNT` | `IconRole::ALL` const array | One canonical source of truth for the 42 variants; useful in tests |
| String-to-IconSet conversion | Manual `match` on strings everywhere | `IconSet::from_name(&str)` method | Centralizes the mapping of TOML strings like "sf-symbols" to IconSet::SfSymbols |
| TOML field merge logic | Custom merge implementation | Existing `impl_merge!` macro pattern | Just add `icon_theme` to the option fields list in ThemeVariant's merge impl |

**Key insight:** This phase is almost entirely hand-written code (enums and match tables). The "don't hand-roll" concern is more about not duplicating logic: keep one canonical variant list, one canonical mapping function, one canonical string-to-enum conversion.

## Common Pitfalls

### Pitfall 1: Forgetting #[non_exhaustive] Wildcard in Match
**What goes wrong:** Tests compile fine within the crate (non_exhaustive only affects external crates), but downstream consumers get compile errors when a new variant is added.
**Why it happens:** `#[non_exhaustive]` has no effect within the defining crate, so internal match statements can be exhaustive. The pitfall is not testing the external consumer experience.
**How to avoid:** Document that external match on IconRole/IconSet requires `_ =>` arm. Include a doc example showing the wildcard pattern.
**Warning signs:** Match statements without wildcards in doc examples.

### Pitfall 2: ThemeVariant Merge Not Updated
**What goes wrong:** Adding `icon_theme` to `ThemeVariant` but forgetting to update the manual `merge()` and `is_empty()` implementations.
**Why it happens:** `ThemeVariant` has a hand-written `merge()` (not using `impl_merge!`) because of the `Option<WidgetMetrics>` special case.
**How to avoid:** Update `merge()` to include icon_theme in the merge logic and `is_empty()` to check it. Add a test that verifies merge behavior for icon_theme.
**Warning signs:** TOML overlay themes with icon_theme don't override base theme's icon_theme.

### Pitfall 3: Icon Name Mapping Inconsistencies
**What goes wrong:** Mapping the wrong identifier for an icon set (e.g., using Material name for Lucide, or using an SF Symbol that was renamed in a newer version).
**Why it happens:** The availability matrix in docs/native-icons.md is comprehensive but manually curated.
**How to avoid:** Write a test that verifies every (IconSet, IconRole) pair has a consistent return value. Spot-check against official sources for critical icons.
**Warning signs:** icon_name() returns a name that doesn't exist in the target icon set.

### Pitfall 4: Serde Round-Trip Breakage for icon_theme
**What goes wrong:** Adding icon_theme to ThemeVariant breaks the TOML serialization round-trip for existing presets.
**Why it happens:** If icon_theme is not properly defaulted/skipped, existing TOML files that lack the field may fail to parse or emit unexpected output.
**How to avoid:** Use `#[serde(default, skip_serializing_if = "Option::is_none")]` (matching the existing widget_metrics pattern). All existing presets lack icon_theme, so they deserialize to None. Test round-trip for all 17 presets.
**Warning signs:** Existing preset tests fail after adding the field.

### Pitfall 5: Workspace Version Mismatch
**What goes wrong:** The workspace currently has `native-theme = { path = "native-theme", version = "0.2.0" }` but the crate is at 0.3.0. Cargo refuses to resolve.
**Why it happens:** Version was bumped in the crate but not in the workspace dependency specification.
**How to avoid:** Update the workspace Cargo.toml to `version = "0.3.0"` before running tests. This is a pre-existing issue, not caused by Phase 16, but will block testing.
**Warning signs:** `cargo test` fails with version resolution error before any code runs.

## Code Examples

Verified patterns from project source and docs:

### IconRole Enum (all 42 variants)
```rust
// Source: docs/native-icons.md availability matrix
// All 42 variants grouped by category prefix

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IconRole {
    // Dialog / Alert (6)
    DialogWarning,
    DialogError,
    DialogInfo,
    DialogQuestion,
    DialogSuccess,
    Shield,

    // Window Controls (4)
    WindowClose,
    WindowMinimize,
    WindowMaximize,
    WindowRestore,

    // Common Actions (14)
    ActionSave,
    ActionDelete,
    ActionCopy,
    ActionPaste,
    ActionCut,
    ActionUndo,
    ActionRedo,
    ActionSearch,
    ActionSettings,
    ActionEdit,
    ActionAdd,
    ActionRemove,
    ActionRefresh,
    ActionPrint,

    // Navigation (6)
    NavBack,
    NavForward,
    NavUp,
    NavDown,
    NavHome,
    NavMenu,

    // Files / Places (5)
    FileGeneric,
    FolderClosed,
    FolderOpen,
    TrashEmpty,
    TrashFull,

    // Status (3)
    StatusLoading,
    StatusCheck,
    StatusError,

    // System (4)
    UserAccount,
    Notification,
    Help,
    Lock,
}

impl IconRole {
    /// All icon role variants, useful for iteration and exhaustive testing.
    pub const ALL: [IconRole; 42] = [
        Self::DialogWarning, Self::DialogError, Self::DialogInfo,
        Self::DialogQuestion, Self::DialogSuccess, Self::Shield,
        Self::WindowClose, Self::WindowMinimize, Self::WindowMaximize,
        Self::WindowRestore,
        Self::ActionSave, Self::ActionDelete, Self::ActionCopy,
        Self::ActionPaste, Self::ActionCut, Self::ActionUndo,
        Self::ActionRedo, Self::ActionSearch, Self::ActionSettings,
        Self::ActionEdit, Self::ActionAdd, Self::ActionRemove,
        Self::ActionRefresh, Self::ActionPrint,
        Self::NavBack, Self::NavForward, Self::NavUp, Self::NavDown,
        Self::NavHome, Self::NavMenu,
        Self::FileGeneric, Self::FolderClosed, Self::FolderOpen,
        Self::TrashEmpty, Self::TrashFull,
        Self::StatusLoading, Self::StatusCheck, Self::StatusError,
        Self::UserAccount, Self::Notification, Self::Help, Self::Lock,
    ];
}
```

### IconData Enum
```rust
// Source: docs/native-icons.md Types section + ICON-02 requirement

/// Icon data returned by loading functions.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IconData {
    /// SVG content (from freedesktop themes, bundled icon sets).
    Svg(Vec<u8>),
    /// Rasterized RGBA pixels (from macOS/Windows system APIs).
    Rgba {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },
}
```

### IconSet Enum
```rust
// Source: docs/native-icons.md Icon Sets section

/// Known icon sets that icon_name() can map to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum IconSet {
    /// Apple SF Symbols (macOS, iOS)
    SfSymbols,
    /// Microsoft Segoe Fluent Icons (Windows)
    SegoeIcons,
    /// freedesktop Icon Naming Specification (Linux)
    Freedesktop,
    /// Google Material Symbols
    Material,
    /// Lucide Icons (fork of Feather)
    Lucide,
}

impl IconSet {
    /// Parse an icon set from its string identifier.
    ///
    /// Accepts the lowercase kebab-case names used in TOML:
    /// "sf-symbols", "segoe-fluent", "freedesktop", "material", "lucide"
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "sf-symbols" => Some(Self::SfSymbols),
            "segoe-fluent" => Some(Self::SegoeIcons),
            "freedesktop" => Some(Self::Freedesktop),
            "material" => Some(Self::Material),
            "lucide" => Some(Self::Lucide),
            _ => None,
        }
    }

    /// The string identifier for this icon set, as used in TOML.
    pub fn name(&self) -> &'static str {
        match self {
            Self::SfSymbols => "sf-symbols",
            Self::SegoeIcons => "segoe-fluent",
            Self::Freedesktop => "freedesktop",
            Self::Material => "material",
            Self::Lucide => "lucide",
        }
    }
}
```

### system_icon_set() Function
```rust
// Source: ICON-05 requirement + docs/native-icons.md Public API section

/// Resolve "system" to the actual icon set for the current OS.
///
/// - macOS  -> SfSymbols
/// - Windows -> SegoeIcons
/// - Linux  -> Freedesktop
/// - other  -> Material (safe cross-platform fallback)
pub fn system_icon_set() -> IconSet {
    #[cfg(target_os = "macos")]
    { IconSet::SfSymbols }

    #[cfg(target_os = "windows")]
    { IconSet::SegoeIcons }

    #[cfg(target_os = "linux")]
    { IconSet::Freedesktop }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    { IconSet::Material }
}
```

### ThemeVariant icon_theme Addition
```rust
// Source: existing model/mod.rs pattern for optional fields

// In ThemeVariant struct, add:
/// Icon set to use for this variant (e.g., "sf-symbols", "material").
/// When None, resolved at runtime via system_icon_set().
#[serde(default, skip_serializing_if = "Option::is_none")]
pub icon_theme: Option<String>,

// In ThemeVariant::merge(), add:
if overlay.icon_theme.is_some() {
    self.icon_theme = overlay.icon_theme.clone();
}

// In ThemeVariant::is_empty(), add:
&& self.icon_theme.is_none()
```

### Preset TOML Updates (native presets only)
```toml
# windows-11.toml - add at top level, before [light.colors]
name = "Windows 11"

[light]
icon_theme = "segoe-fluent"

[light.colors]
# ... existing colors ...

[dark]
icon_theme = "segoe-fluent"

[dark.colors]
# ... existing colors ...
```

```toml
# macos-sonoma.toml
[light]
icon_theme = "sf-symbols"
# ...
[dark]
icon_theme = "sf-symbols"
```

```toml
# adwaita.toml, kde-breeze.toml
[light]
icon_theme = "freedesktop"
# ...
[dark]
icon_theme = "freedesktop"
```

```toml
# material.toml
[light]
icon_theme = "material"
# ...
[dark]
icon_theme = "material"
```

```toml
# Community themes (catppuccin-*, nord, dracula, etc.) and default.toml:
# NO icon_theme field -- deserializes to None -> resolved at runtime
```

## Discretion Decisions

### IconRole Derives: `Debug, Clone, Copy, PartialEq, Eq, Hash`
**Recommendation:** Do NOT add serde derives to IconRole. IconRole is a runtime enum used for lookups, not serialized to/from TOML. Strum is not needed -- there is no string conversion use case for the role names themselves (the mapping goes through `icon_name()` to platform-specific strings). Adding `Copy` is important since it is a fieldless enum.

### IconData Ownership: Owned `Vec<u8>`
**Recommendation:** Use `Vec<u8>` everywhere, not `Cow<'a, [u8]>`. Rationale:
- Adding `'a` lifetime to `IconData` would infect every function signature and struct that holds it
- Bundled SVGs (Phase 17) are small (~2-5KB each); `.to_vec()` from a `&'static [u8]` is negligible
- Platform-loaded icons (Phases 18-20) produce owned data anyway
- Consistency with the existing codebase pattern (no lifetime parameters on model types)

### IconData Color/Tint: No tint metadata
**Recommendation:** Keep tint/color as the connector's responsibility. The REQUIREMENTS.md explicitly lists "Icon tinting/coloring in core crate" as out of scope. IconData is raw pixel/SVG data only.

### IconData Size Hint: No nominal size on SVG
**Recommendation:** SVG variant carries no size -- SVGs are resolution-independent. Size is a parameter of `load_icon()` (Phase 21) and rasterization (Phase 21 INTG-02). The `Rgba` variant naturally carries width/height.

### Mapping Strategy: Static match tables
**Recommendation:** Use `match` statements, not phf. For 42 entries, LLVM compiles match to efficient code (jump tables or binary search). phf adds a proc-macro dependency for negligible performance difference. Match is also more readable and easier to update.

### Unmapped Roles: Return `Option<&'static str>`
**Recommendation:** `icon_name()` returns `Option<&'static str>`. Return `None` for roles that have no equivalent in the target icon set (documented in the availability matrix with "--"). Callers (Phase 21's `load_icon()`) use this to decide fallback behavior. This is honest about gaps rather than providing confusing "best-effort" approximations.

### IconSet Extensibility: Fixed 5 sets with `#[non_exhaustive]`
**Recommendation:** Five known icon sets as enum variants with `#[non_exhaustive]`. The TOML field remains `Option<String>` (not `Option<IconSet>`) to allow future icon set names to appear in config files without a library update. `IconSet::from_name()` bridges the gap.

### system_icon_set() Placement: This phase (Phase 16)
**Recommendation:** Implement in this phase. It is a pure `cfg!()` function with zero runtime dependencies. No platform API calls needed. Phase 21 can extend it if runtime detection is desired, but the compile-time version is the right default.

### TOML Field Type: `Option<String>`
**Recommendation:** Use `Option<String>`, not `Option<IconSet>`. Reasons:
- TOML files may reference future icon sets not yet in the enum
- The "system" string is meaningful in TOML but is not an IconSet variant
- Consistent with how the API in docs/native-icons.md uses `&str` parameters

### TOML Field Placement: Per-variant (on ThemeVariant)
**Recommendation:** Place `icon_theme` on `ThemeVariant`, not on `NativeTheme`. Each variant (light/dark) gets its own optional `icon_theme`. This follows the existing pattern where all visual properties live on `ThemeVariant`. In practice, light and dark will use the same icon set, but per-variant placement is more consistent and future-proof.

### Preset Defaults: Native presets get values, community presets get None
**Recommendation:** Following the design spec in docs/native-icons.md:
- Platform presets (`windows-11`, `macos-sonoma`, `ios`, `adwaita`, `kde-breeze`, `material`): set `icon_theme` to their native icon set
- Community presets (`catppuccin-*`, `nord`, `dracula`, etc.) and `default`: leave `icon_theme` absent (deserializes to `None`, resolved at runtime via `system_icon_set()`)

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| String-based icon names | Typed enum roles with mapping function | Current design | Type safety, exhaustiveness checking, no typos |
| phf for small mappings | Plain match (LLVM-optimized) | Ongoing consensus | Zero-dep solution with equivalent performance for < 100 entries |
| Lifetime-parameterized data types | Owned Vec<u8> | Rust ecosystem norm | Simpler API at negligible allocation cost for small payloads |

**Deprecated/outdated:**
- phf proc macros (phf_macros): Still maintained at 0.13.1, but unnecessary complexity for 42-entry lookups
- strum for enum string conversion: Useful in general, but icon names are not 1:1 with variant names so custom match is needed anyway

## Open Questions

1. **Workspace version mismatch**
   - What we know: workspace Cargo.toml specifies `native-theme = { version = "0.2.0" }` but crate is at 0.3.0. `cargo test` fails.
   - What's unclear: Whether this should be fixed as part of Phase 16 or is tracked elsewhere.
   - Recommendation: Fix it in the first task of Phase 16 (update workspace dep to 0.3.0) since tests cannot run otherwise.

2. **iOS preset icon_theme**
   - What we know: docs/native-icons.md lists `ios` preset with `icon_theme = "sf-symbols"` since iOS uses the same SF Symbols as macOS.
   - What's unclear: Whether `system_icon_set()` should return `IconSet::SfSymbols` for `target_os = "ios"` too.
   - Recommendation: Yes, add `#[cfg(target_os = "ios")]` returning `SfSymbols`. It costs nothing and is correct.

3. **icon_name() signature: IconSet enum vs &str**
   - What we know: docs/native-icons.md shows `icon_name(icon_theme: &str, role: IconRole)` with string parameter, but the success criteria show `icon_name(IconSet::SfSymbols, IconRole::ActionCopy)` with enum parameter.
   - What's unclear: Which signature to use -- the success criteria are authoritative.
   - Recommendation: Use the `IconSet` enum parameter as shown in success criteria. This is type-safe and matches the enum design. The string-based `load_icon()` comes in Phase 21 where string dispatch makes more sense (since it includes "system" handling).

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test (cargo test) |
| Config file | Cargo.toml (workspace) |
| Quick run command | `cargo test -p native-theme --lib` |
| Full suite command | `cargo test -p native-theme` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| ICON-01 | 42 IconRole variants accessible and matchable | unit | `cargo test -p native-theme icon_role -- --exact` | Wave 0 |
| ICON-02 | IconData::Svg and IconData::Rgba constructable and matchable | unit | `cargo test -p native-theme icon_data -- --exact` | Wave 0 |
| ICON-03 | icon_theme field on ThemeVariant + preset TOML round-trip | unit | `cargo test -p native-theme icon_theme -- --exact` | Wave 0 |
| ICON-04 | icon_name() returns correct identifiers for all 5 sets | unit | `cargo test -p native-theme icon_name -- --exact` | Wave 0 |
| ICON-05 | system_icon_set() returns correct set per OS | unit | `cargo test -p native-theme system_icon_set -- --exact` | Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme --lib`
- **Per wave merge:** `cargo test -p native-theme`
- **Phase gate:** Full suite green before verification

### Wave 0 Gaps
- [ ] `native-theme/src/model/icons.rs` -- module does not exist yet; all icon tests go here
- [ ] Fix workspace version mismatch (Cargo.toml `native-theme = "0.2.0"` -> `"0.3.0"`) -- tests cannot run without this

## Sources

### Primary (HIGH confidence)
- `docs/native-icons.md` -- Complete availability matrix and identifier strings for all 5 icon sets x 42 roles. This is the authoritative source for all mapping data.
- `native-theme/src/model/mod.rs` -- ThemeVariant struct, merge pattern, serde attributes. Verified by reading source.
- `native-theme/src/presets.rs` -- TOML preset loading pattern. Verified by reading source.
- `native-theme/src/error.rs` -- `#[non_exhaustive]` enum pattern. Verified by reading source.
- Rust Reference: [The non_exhaustive attribute](https://doc.rust-lang.org/reference/attributes/type_system.html)
- Cargo SemVer: [SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)

### Secondary (MEDIUM confidence)
- [phf crate 0.13.1](https://crates.io/crates/phf) -- Version confirmed via `cargo search`. Performance comparison with match based on [mega-match-vs-phf benchmark](https://github.com/lmammino/mega-match-vs-phf) and [Rust internals discussion](https://internals.rust-lang.org/t/what-if-match-statetement-could-generate-perfect-hash-function/19222).
- [strum crate 0.28.0](https://crates.io/crates/strum) -- Version confirmed via `cargo search`.

### Tertiary (LOW confidence)
- None -- all findings verified from primary or secondary sources.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- no new dependencies, all patterns verified from existing codebase
- Architecture: HIGH -- follows existing model module patterns exactly, all mapping data pre-researched in docs/native-icons.md
- Pitfalls: HIGH -- identified from direct codebase analysis (merge impl, serde round-trip, workspace version)

**Research date:** 2026-03-09
**Valid until:** 2026-04-08 (stable domain, no external API changes expected)
