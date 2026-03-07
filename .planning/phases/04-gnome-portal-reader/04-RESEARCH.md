# Phase 4: GNOME Portal Reader - Research

**Researched:** 2026-03-07
**Domain:** Linux freedesktop settings portal (D-Bus, ashpd, async runtime design, GNOME appearance settings)
**Confidence:** HIGH

## Summary

This phase implements `from_gnome()`, an async function that reads the user's accent color, color scheme (light/dark preference), and contrast preference from the XDG Desktop Portal Settings interface via ashpd. The portal exposes exactly three appearance-related values under the `org.freedesktop.appearance` namespace: `accent-color` (a tuple of three f64 RGB components in [0,1] sRGB range), `color-scheme` (uint: 0=no preference, 1=prefer dark, 2=prefer light), and `contrast` (uint: 0=normal, 1=high). These values are sparse -- far fewer than KDE's 60+ color roles -- so the function must overlay portal data onto hardcoded Adwaita defaults (already available as a bundled preset) to produce a usable NativeTheme with all 36 semantic color roles populated.

The critical design decision for this phase is the async runtime feature flag structure. ashpd 0.13.4 defaults to tokio but supports async-io as an alternative. Since Cargo's feature unification means all crates in a dependency graph see the union of enabled features, the `portal` feature in native-theme must NOT include ashpd's tokio by default. Instead, native-theme should disable ashpd's default features and expose two convenience features: `portal-tokio` and `portal-async-io`, letting the consumer choose. A bare `portal` feature that requires the consumer to enable one runtime themselves is also acceptable but less ergonomic. This design is breaking to change post-publish, so it must be correct from day one.

The function falls back to hardcoded Adwaita defaults rather than failing when portal values are unavailable (no D-Bus session, sandboxed environment without portal access, old portal version). This means `from_gnome()` should return `Ok(NativeTheme)` with Adwaita defaults even when all three portal reads fail -- only returning `Err` if D-Bus connection itself catastrophically fails and even the Adwaita fallback cannot be constructed (which in practice means never, since Adwaita values are hardcoded constants).

**Primary recommendation:** Use ashpd 0.13.4 with `default-features = false`, expose `portal-tokio` and `portal-async-io` features that pass through the runtime choice, load the bundled Adwaita preset as base, then overlay accent color from portal (mapping to accent + selection + focus_ring + primary.background), color scheme to pick light/dark variant, and contrast to adjust border/separator colors.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PLAT-02 | Linux GNOME reader: from_gnome() -- async, reads freedesktop portal via ashpd (feature "portal") | ashpd 0.13.4 API verified (Settings::new(), accent_color(), color_scheme(), contrast()), XDG portal spec verified (org.freedesktop.appearance namespace), Adwaita preset exists as fallback base, feature flag design researched |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| ashpd | 0.13.4 | XDG Desktop Portal D-Bus wrapper | The only ergonomic Rust crate for freedesktop portal access. Provides typed `Settings` proxy with `accent_color()`, `color_scheme()`, `contrast()` convenience methods. Built on zbus 5.x. Verified on docs.rs. |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| zbus | 5.13+ (transitive via ashpd) | D-Bus protocol implementation | Not a direct dependency -- pulled in by ashpd. Provides the async D-Bus connection. |
| tokio | 1.41+ (transitive via ashpd/tokio) | Async runtime (one option) | When consumer already uses tokio (most common case). Enabled via `portal-tokio` feature. |
| async-io | (transitive via ashpd/async-io) | Async runtime (alternative) | When consumer uses smol, glib, or iced's default runtime. Enabled via `portal-async-io` feature. |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| ashpd | Raw zbus 5.x | ashpd provides typed `ColorScheme`, `Contrast` enums and `Color` struct with `accent_color()` convenience. Raw zbus requires manual D-Bus interface/method calls, string-based namespace/key lookups, and manual `OwnedValue` deserialization. The abstraction saves ~100 lines and prevents typo bugs. |
| ashpd | dconf/GSettings directly | Portal is the correct abstraction for sandboxed apps (Flatpak, Snap). dconf requires direct filesystem/D-Bus access to GSettings backend. Portal works universally. |

**Installation (Cargo.toml changes):**
```toml
[features]
portal = ["dep:ashpd"]
portal-tokio = ["portal", "ashpd/tokio"]
portal-async-io = ["portal", "ashpd/async-io"]

[dependencies]
ashpd = { version = "0.13", optional = true, default-features = false, features = ["settings"] }
```

## Architecture Patterns

### Recommended Project Structure
```
src/
  gnome/
    mod.rs          # pub async fn from_gnome() -> Result<NativeTheme>, internal helpers
  kde/              # (existing) sync KDE reader
  model/            # (existing) data model
  presets/           # (existing) Adwaita preset used as fallback base
  lib.rs            # Add: #[cfg(feature = "portal")] pub mod gnome; + re-export
```

### Pattern 1: Adwaita Base + Portal Overlay
**What:** Load the bundled Adwaita preset as a complete NativeTheme (all 36 color roles filled), then overlay the 3-4 values from the portal (accent color, scheme, contrast). This produces a fully populated theme rather than a sparse one with only 3 fields.
**When to use:** Always -- the portal provides too few values to build a complete theme from scratch.
**Example:**
```rust
pub async fn from_gnome() -> crate::Result<crate::NativeTheme> {
    // 1. Start with full Adwaita as base
    let base = crate::preset("adwaita")
        .expect("adwaita preset must be bundled");

    // 2. Try to connect to portal and read settings
    let settings = match ashpd::desktop::settings::Settings::new().await {
        Ok(s) => s,
        Err(_) => return Ok(base), // No portal -> return Adwaita defaults
    };

    // 3. Read color scheme to determine light/dark
    let scheme = settings.color_scheme().await.unwrap_or_default();

    // 4. Read accent color (if available)
    let accent = settings.accent_color().await.ok();

    // 5. Read contrast preference
    let contrast = settings.contrast().await.unwrap_or_default();

    // 6. Build NativeTheme from base + overlay
    build_theme(base, scheme, accent, contrast)
}
```

### Pattern 2: Feature-Gated Async Module
**What:** The `gnome` module is behind `#[cfg(feature = "portal")]` and only compiled when the portal feature is enabled. The function is `async fn`, not sync.
**When to use:** Always for portal reader.
**Example:**
```rust
// src/lib.rs
#[cfg(feature = "portal")]
pub mod gnome;

#[cfg(feature = "portal")]
pub use gnome::from_gnome;
```

### Pattern 3: Internal Content Helper for Testing
**What:** Following the KDE pattern of `from_kde_content()`, create an internal helper that takes pre-read portal values as parameters, enabling unit testing without D-Bus.
**When to use:** Always -- D-Bus is not available in CI/test environments.
**Example:**
```rust
/// Internal helper for building theme from portal values.
/// Enables testing without D-Bus connection.
fn build_theme(
    base: crate::NativeTheme,
    scheme: ashpd::desktop::settings::ColorScheme,
    accent: Option<ashpd::desktop::Color>,
    contrast: ashpd::desktop::settings::Contrast,
) -> crate::Result<crate::NativeTheme> {
    // Pick variant based on scheme
    // Overlay accent color
    // Adjust contrast if high
    // Return modified base
}
```

### Pattern 4: Accent Color Mapping
**What:** Map the single portal accent color to multiple ThemeColors fields for visual consistency.
**When to use:** Whenever accent color is available from the portal.
**Example:**
```rust
fn apply_accent(variant: &mut crate::ThemeVariant, accent: &crate::Rgba) {
    variant.colors.core.accent = Some(*accent);
    variant.colors.interactive.selection = Some(*accent);
    variant.colors.interactive.focus_ring = Some(*accent);
    variant.colors.primary.background = Some(*accent);
    // primary.foreground stays white (from Adwaita base)
}
```

### Anti-Patterns to Avoid
- **Returning Error when portal is unavailable:** The spec says fall back to Adwaita defaults, not fail. Only catastrophic errors (cannot construct Adwaita base) should produce Err.
- **Enabling ashpd default features:** This forces tokio on all consumers via feature unification. Always use `default-features = false`.
- **Clamping out-of-range accent color values:** Per the XDG portal spec, out-of-range RGB values (outside [0,1]) mean "no accent color set." Treat as None, not as a clamped color.
- **Providing a sync wrapper with block_on:** This creates problems when called from within an existing async runtime. Keep `from_gnome()` as `async fn` and let consumers handle the sync/async boundary.
- **Populating both light AND dark variants from portal:** Color scheme tells us which variant the user prefers. Populate only the selected variant (like KDE reader), use the corresponding Adwaita base variant.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| D-Bus portal communication | Raw zbus interface calls | ashpd `Settings` proxy | ashpd handles connection management, type deserialization, namespace/key constants, typed enums (ColorScheme, Contrast, Color) |
| Adwaita default colors | Hardcoded color constants in gnome module | `crate::preset("adwaita")` | Adwaita preset already exists with all 36 color roles, fonts, geometry, spacing. Loading it is one function call. Keeping two copies of Adwaita values in sync would be a maintenance burden. |
| Accent-to-Rgba conversion | Manual f64-to-u8 arithmetic | `Rgba::from_f32()` | Already exists on Rgba type, handles clamping and rounding correctly |
| Async runtime selection | Custom runtime detection | Feature flags passing through to ashpd | The consumer knows their runtime. Feature flags are the standard Rust mechanism. |

**Key insight:** The GNOME portal reader is fundamentally a thin adapter: read 3 values from D-Bus, convert types, overlay onto Adwaita base. The complexity is in the feature flag design and error handling, not in the reading logic.

## Common Pitfalls

### Pitfall 1: ashpd Default Features Leak Tokio
**What goes wrong:** Enabling `portal` feature pulls in ashpd with default features, which includes tokio. Due to Cargo feature unification, ALL consumers get tokio even if they only enabled the `kde` (sync) feature in another crate in the dependency graph.
**Why it happens:** ashpd's default features include `tokio`. Feature unification means the union of all requested features is activated.
**How to avoid:** In Cargo.toml: `ashpd = { version = "0.13", optional = true, default-features = false, features = ["settings"] }`. Expose `portal-tokio` and `portal-async-io` features that add the runtime.
**Warning signs:** `cargo tree` shows tokio appearing when only `kde` feature is enabled in another crate. Compile times increase unexpectedly for sync-only consumers.

### Pitfall 2: Portal Unavailable Causes Hard Failure
**What goes wrong:** `from_gnome()` returns `Err` when D-Bus session is not available (headless servers, minimal containers, CI environments).
**Why it happens:** `Settings::new().await` fails if no D-Bus session bus exists. Propagating this error violates the requirement to fall back to Adwaita.
**How to avoid:** Catch `Settings::new()` errors and return the Adwaita preset as fallback. Only return `Err` for truly unrecoverable situations (which in practice should not occur since Adwaita values are hardcoded).
**Warning signs:** Tests fail in CI; `from_gnome()` panics or errors on headless Linux.

### Pitfall 3: Out-of-Range Accent Color Treated as Valid
**What goes wrong:** Portal returns accent-color with values outside [0,1] (e.g., `(2.0, -1.0, 0.5)`). Code clamps these to [0,1] and uses the result, producing a wrong color.
**Why it happens:** Per the XDG portal spec, out-of-range values mean "accent color is not set." Clamping misinterprets the signal.
**How to avoid:** Check that all three components are in [0.0, 1.0] range. If any is out of range, treat accent as None (use Adwaita default accent).
**Warning signs:** Accent color appears as pure red (#ff0000) or black (#000000) on systems without accent color support.

### Pitfall 4: ashpd "settings" Feature Not Enabled
**What goes wrong:** Compilation fails with missing `Settings`, `ColorScheme`, `Contrast` types.
**Why it happens:** ashpd uses per-portal feature flags. The `settings` feature must be explicitly enabled when `default-features = false`.
**How to avoid:** Include `features = ["settings"]` in the ashpd dependency declaration.
**Warning signs:** Compilation errors about missing types in `ashpd::desktop::settings`.

### Pitfall 5: Color Scheme NoPreference Handled Incorrectly
**What goes wrong:** When portal returns `ColorScheme::NoPreference`, the code does not know which variant to populate and either panics, produces an empty theme, or always defaults to light.
**Why it happens:** `NoPreference` is the default when the portal does not report a color scheme. It means the app should use its own default, not that there is no theme.
**How to avoid:** Treat `NoPreference` as light (Adwaita's default is light). Populate the light variant from the Adwaita base. Document this behavior.
**Warning signs:** Theme is empty or always dark on systems that do not report color scheme.

### Pitfall 6: Changing Feature Structure After Publish
**What goes wrong:** After publishing with `portal = ["dep:ashpd"]` (ashpd default features = tokio), trying to change to `default-features = false` is a breaking change for consumers who relied on tokio being transitively available.
**Why it happens:** Cargo feature removal/change is breaking per semver. Consumers may depend on the transitive tokio availability.
**How to avoid:** Get the feature flag design RIGHT in this phase, before any publish. Use `default-features = false` from the start. This is explicitly called out in prior research as a critical blocker.
**Warning signs:** N/A -- this is a design-time decision, not a runtime error.

## Code Examples

### Cargo.toml Feature Configuration
```toml
# Source: Prior research STACK.md + ashpd docs.rs feature analysis
[features]
portal = ["dep:ashpd"]
portal-tokio = ["portal", "ashpd/tokio"]
portal-async-io = ["portal", "ashpd/async-io"]

[dependencies]
ashpd = { version = "0.13", optional = true, default-features = false, features = ["settings"] }
```

### Reading Portal Settings
```rust
// Source: ashpd docs.rs Settings struct API
use ashpd::desktop::settings::{ColorScheme, Contrast, Settings};
use ashpd::desktop::Color;

async fn read_portal() -> (ColorScheme, Option<Color>, Contrast) {
    let settings = match Settings::new().await {
        Ok(s) => s,
        Err(_) => return (ColorScheme::default(), None, Contrast::default()),
    };

    let scheme = settings.color_scheme().await.unwrap_or_default();
    let accent = settings.accent_color().await.ok();
    let contrast = settings.contrast().await.unwrap_or_default();

    (scheme, accent, contrast)
}
```

### Converting ashpd Color to Rgba
```rust
// Source: ashpd docs.rs Color struct (red/green/blue return f64 in [0,1])
fn portal_color_to_rgba(color: &ashpd::desktop::Color) -> Option<crate::Rgba> {
    let r = color.red();
    let g = color.green();
    let b = color.blue();

    // Per XDG spec: out-of-range means "unset"
    if r < 0.0 || r > 1.0 || g < 0.0 || g > 1.0 || b < 0.0 || b > 1.0 {
        return None;
    }

    // Rgba::from_f32 handles clamping and rounding
    Some(crate::Rgba::from_f32(r as f32, g as f32, b as f32, 1.0))
}
```

### Building Theme from Portal Values
```rust
fn build_theme(
    base: crate::NativeTheme,
    scheme: ColorScheme,
    accent: Option<Color>,
    contrast: Contrast,
) -> crate::Result<crate::NativeTheme> {
    // Determine if dark based on portal color scheme
    let is_dark = matches!(scheme, ColorScheme::PreferDark);

    // Pick the appropriate variant from the Adwaita base
    let mut variant = if is_dark {
        base.dark.unwrap_or_default()
    } else {
        base.light.unwrap_or_default()
    };

    // Apply accent color if available
    if let Some(color) = accent {
        if let Some(rgba) = portal_color_to_rgba(&color) {
            apply_accent(&mut variant, &rgba);
        }
    }

    // Apply high contrast adjustments
    if matches!(contrast, Contrast::High) {
        apply_high_contrast(&mut variant);
    }

    // Build NativeTheme with single variant
    let theme = if is_dark {
        crate::NativeTheme {
            name: "GNOME".to_string(),
            light: None,
            dark: Some(variant),
        }
    } else {
        crate::NativeTheme {
            name: "GNOME".to_string(),
            light: Some(variant),
            dark: None,
        }
    };

    Ok(theme)
}
```

### Accent Color Propagation
```rust
/// Apply a portal accent color across multiple semantic roles.
fn apply_accent(variant: &mut crate::ThemeVariant, accent: &crate::Rgba) {
    // Core accent
    variant.colors.core.accent = Some(*accent);

    // Interactive elements use accent
    variant.colors.interactive.selection = Some(*accent);
    variant.colors.interactive.focus_ring = Some(*accent);

    // Primary action uses accent as background
    variant.colors.primary.background = Some(*accent);

    // Link color slightly different shade -- keep Adwaita default
    // (portal does not provide link color separately)
}
```

### High Contrast Adjustments
```rust
/// Adjust theme for high contrast preference.
/// Makes borders more visible and increases separation.
fn apply_high_contrast(variant: &mut crate::ThemeVariant) {
    // Increase border opacity to full
    if variant.geometry.border_opacity.is_some() {
        variant.geometry.border_opacity = Some(1.0);
    }

    // Increase disabled opacity for better visibility
    if variant.geometry.disabled_opacity.is_some() {
        variant.geometry.disabled_opacity = Some(0.7);
    }

    // Could also darken borders / increase contrast between bg and fg
    // but the base Adwaita values are already good for accessibility
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| GSettings/dconf direct reads | XDG Desktop Portal Settings | Portal spec v2 | Portal works in sandboxed environments (Flatpak); dconf requires direct access |
| No accent color in portal | `accent-color` key added | XDG portal spec, ~2023 | Apps can now read GNOME 47+ user-chosen accent color via portal |
| ashpd tokio-only | ashpd 0.13 supports tokio OR async-io | ashpd 0.13+ | Consumers can choose runtime; iced (async-io) users no longer forced into tokio |
| Manual D-Bus calls for settings | ashpd typed convenience methods | ashpd 0.10+ | `settings.accent_color()` replaces manual `read("org.freedesktop.appearance", "accent-color")` with type conversion |

**Deprecated/outdated:**
- `ashpd/async-std` feature: Does NOT exist in ashpd 0.13.x. The alternative to tokio is `async-io`, not `async-std`. Prior documentation referencing `async-std` is incorrect.
- Direct GSettings reads for theme data: Use the portal instead -- it works in sandboxed environments and is the official API for app-facing settings.

## Open Questions

1. **Should `from_gnome()` populate BOTH light and dark variants?**
   - What we know: Portal tells us the user's preference (dark/light). Adwaita base has both variants.
   - What's unclear: Whether to return both Adwaita variants (with accent applied to each) or only the active one.
   - Recommendation: Populate only the active variant (matching KDE reader pattern). Users who want both can merge with `preset("adwaita")`. This is consistent with the KDE reader behavior.

2. **Should the bare `portal` feature work without specifying a runtime?**
   - What we know: ashpd with `default-features = false` and `features = ["settings"]` but no runtime feature will fail to compile (zbus needs a runtime).
   - What's unclear: Whether `portal` alone (without `portal-tokio` or `portal-async-io`) should be a valid feature.
   - Recommendation: Make `portal` alone NOT compilable -- it requires one of `portal-tokio` or `portal-async-io`. Add a compile_error! message if `portal` is enabled without a runtime. Alternatively, the simplest approach: `portal` always implies tokio (via ashpd default features) and `portal-async-io` disables that. But this goes against the "no tokio leak" requirement. The cleanest design: `portal` is a base feature that gates the code, `portal-tokio` and `portal-async-io` add the runtime. Document that one must be chosen.

3. **How should high contrast be applied beyond border/opacity changes?**
   - What we know: Portal reports `Contrast::High`. GNOME's high-contrast mode in libadwaita changes many CSS variables beyond just borders.
   - What's unclear: Exact libadwaita high-contrast color modifications.
   - Recommendation: For v1, adjust `border_opacity` and `disabled_opacity`. A future version could load a separate `adwaita-hc` preset if demand exists. Keep it simple.

4. **Should from_gnome() return the accent-modified Adwaita or a NativeTheme named "GNOME"?**
   - What we know: KDE reader uses `ColorScheme` key from kdeglobals or falls back to "KDE".
   - What's unclear: Best name for the theme.
   - Recommendation: Use "GNOME" as the theme name. It tells the consumer this data came from a GNOME environment.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (cargo test) |
| Config file | none (built-in) |
| Quick run command | `cargo test --features portal-tokio` |
| Full suite command | `cargo test --all-features` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PLAT-02a | from_gnome() returns NativeTheme with accent, scheme, contrast | unit | `cargo test --features portal-tokio gnome::tests -- -x` | Wave 0 |
| PLAT-02b | build_theme with PreferDark produces dark variant only | unit | `cargo test --features portal-tokio gnome::tests::dark_scheme -- -x` | Wave 0 |
| PLAT-02c | build_theme with PreferLight produces light variant only | unit | `cargo test --features portal-tokio gnome::tests::light_scheme -- -x` | Wave 0 |
| PLAT-02d | build_theme with NoPreference defaults to light | unit | `cargo test --features portal-tokio gnome::tests::no_preference -- -x` | Wave 0 |
| PLAT-02e | Out-of-range accent color treated as None (Adwaita default) | unit | `cargo test --features portal-tokio gnome::tests::accent_out_of_range -- -x` | Wave 0 |
| PLAT-02f | Valid accent color propagates to accent, selection, focus_ring, primary | unit | `cargo test --features portal-tokio gnome::tests::accent_propagation -- -x` | Wave 0 |
| PLAT-02g | High contrast adjusts border_opacity and disabled_opacity | unit | `cargo test --features portal-tokio gnome::tests::high_contrast -- -x` | Wave 0 |
| PLAT-02h | portal feature compiles without tokio when using portal-async-io | integration | `cargo check --features portal-async-io --no-default-features` | Wave 0 |
| PLAT-02i | All portal values unavailable -> returns Adwaita defaults | unit | `cargo test --features portal-tokio gnome::tests::all_unavailable -- -x` | Wave 0 |
| PLAT-02j | Feature flag wiring: portal-tokio enables portal + ashpd/tokio | integration | `cargo check --features portal-tokio` | Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test --features portal-tokio`
- **Per wave merge:** `cargo test --all-features`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `src/gnome/mod.rs` -- module with `from_gnome()`, `build_theme()`, helpers
- [ ] Cargo.toml feature flag changes: `portal`, `portal-tokio`, `portal-async-io`, ashpd dependency
- [ ] lib.rs additions: `#[cfg(feature = "portal")] pub mod gnome;` and re-export
- [ ] Test fixtures: mock ColorScheme/Contrast/Color values (no D-Bus needed for unit tests via `build_theme()` helper)

## Sources

### Primary (HIGH confidence)
- [docs.rs/ashpd/latest - Settings struct](https://docs.rs/ashpd/latest/ashpd/desktop/settings/struct.Settings.html) - accent_color(), color_scheme(), contrast() convenience methods, Settings::new() constructor
- [docs.rs/ashpd/latest - ColorScheme enum](https://docs.rs/ashpd/latest/ashpd/desktop/settings/enum.ColorScheme.html) - NoPreference, PreferDark, PreferLight variants
- [docs.rs/ashpd/latest - Contrast enum](https://docs.rs/ashpd/latest/ashpd/desktop/settings/enum.Contrast.html) - NoPreference, High variants
- [docs.rs/ashpd/latest - Color struct](https://docs.rs/ashpd/latest/ashpd/desktop/struct.Color.html) - red(), green(), blue() returning f64 in [0,1], new(f64, f64, f64)
- [docs.rs/crate/ashpd/latest/features](https://docs.rs/crate/ashpd/latest/features) - 48 feature flags, tokio default, async-io alternative, settings per-portal feature
- [XDG Desktop Portal Settings spec](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html) - org.freedesktop.appearance namespace, accent-color (ddd) tuple, color-scheme (u), contrast (u), out-of-range values, version 2
- [ashpd source: settings.rs](https://bilelmoussaoui.github.io/ashpd/src/ashpd/desktop/settings.rs.html) - APPEARANCE_NAMESPACE, COLOR_SCHEME_KEY, ACCENT_COLOR_SCHEME_KEY, CONTRAST_KEY constants
- Existing codebase: `src/kde/mod.rs` (from_kde_content pattern), `src/presets/adwaita.toml` (fallback base), `src/color.rs` (Rgba::from_f32)

### Secondary (MEDIUM confidence)
- [XDG Desktop Portal accent-color PR #815](https://github.com/flatpak/xdg-desktop-portal/pull/815) - accent-color key addition to portal spec
- [ashpd GitHub repository](https://github.com/bilelmoussaoui/ashpd) - default features include tokio, zbus dependency chain
- Prior project research: `.planning/research/STACK.md`, `.planning/research/PITFALLS.md`, `.planning/research/ARCHITECTURE.md` - ashpd version, feature flag design guidance, anti-patterns

### Tertiary (LOW confidence)
- None -- all findings verified against primary sources.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - ashpd 0.13.4 verified on docs.rs, API surface confirmed, feature flags documented
- Architecture: HIGH - Pattern follows established KDE reader structure, Adwaita preset exists as fallback base, feature flag design researched with multiple sources
- Pitfalls: HIGH - tokio leak identified in prior research and verified against ashpd feature structure; out-of-range accent behavior verified against XDG spec; portal unavailability fallback pattern well-understood

**Research date:** 2026-03-07
**Valid until:** 2026-04-07 (ashpd 0.13.x is stable; XDG portal spec evolves slowly)
