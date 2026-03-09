# Phase 18: Linux Icon Loading - Research

**Researched:** 2026-03-09
**Domain:** freedesktop Icon Theme Specification, Linux icon lookup, Rust
**Confidence:** HIGH

## Summary

Phase 18 implements freedesktop icon theme lookup on Linux -- finding SVG icon files from the user's active desktop icon theme (Adwaita, Breeze, Papirus, etc.) and returning them as `IconData::Svg(Vec<u8>)`. The `freedesktop-icons` crate (v0.4.0, MIT, 3.4M downloads) is the standard Rust solution for this. It implements the full freedesktop Icon Theme Specification: XDG base directory scanning, index.theme parsing, theme inheritance chains, size/scale matching, and hicolor fallback.

The core challenge is bridging our `IconRole` enum (42 roles) through `icon_name(IconSet::Freedesktop, role)` (which returns freedesktop names like `"edit-copy"`) to the `freedesktop_icons::lookup()` API. A critical finding is that Adwaita (GNOME's default theme) stores many action/status icons ONLY as `-symbolic` variants (e.g., `edit-copy-symbolic.svg` in a `symbolic/` directory), while Breeze (KDE's default) has both plain and `-symbolic` variants. The lookup strategy must try the plain name first, then fall back to appending `-symbolic`.

The fallback chain per the success criteria is: active theme -> hicolor -> bundled Material SVGs. The `freedesktop-icons` crate handles theme-to-hicolor fallback natively. Our code adds the final bundled fallback layer using the existing `bundled_icon_svg()` function from Phase 17.

**Primary recommendation:** Use the `freedesktop-icons` crate with `.force_svg()` and a two-pass lookup strategy (plain name, then `-symbolic` suffix) to cover both Breeze-style and Adwaita-style themes, gated behind the `system-icons` feature flag and `#[cfg(target_os = "linux")]`.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PLAT-04 | Linux freedesktop icon theme lookup following Icon Theme Specification -> SVG file bytes (feature "system-icons") | `freedesktop-icons` crate implements the full spec; `.force_svg().find()` returns `Option<PathBuf>` to SVG files; `std::fs::read()` produces the bytes; two-pass lookup handles `-symbolic` suffix for Adwaita compatibility |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| freedesktop-icons | 0.4.0 | Icon theme lookup per freedesktop spec | 3.4M downloads, MIT, pure Rust, handles XDG dirs + theme inheritance + hicolor fallback |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| (std::fs) | stdlib | Read SVG file bytes from resolved path | After lookup returns PathBuf |
| (std::env) | stdlib | Read XDG_DATA_DIRS if needed for theme detection | Already used in codebase |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| freedesktop-icons | freedesktop-icon (v0.0.3) | Much less mature (0.0.x), fewer downloads |
| freedesktop-icons | Hand-rolled spec impl | The spec is deceptively complex: inheritance chains, size matching, directory types (Fixed/Scalable/Threshold), pixmap fallback |
| freedesktop-icons | freedesktop-icon-lookup | Eager scanning approach, less granular control |

**Installation (Cargo.toml):**
```toml
[target.'cfg(target_os = "linux")'.dependencies]
freedesktop-icons = { version = "0.4", optional = true }
```

Feature flag:
```toml
[features]
system-icons = ["dep:freedesktop-icons"]  # on Linux; other platforms add their own deps
```

## Architecture Patterns

### Recommended Module Structure
```
native-theme/src/
  freedesktop.rs        # New: Linux icon loader (cfg(target_os = "linux"))
  lib.rs                # Add: pub mod freedesktop (conditional)
  model/
    icons.rs            # Existing: IconRole, icon_name(), freedesktop_name()
    bundled.rs          # Existing: bundled_icon_svg()
```

### Pattern 1: Two-Pass Lookup with Symbolic Fallback
**What:** Try the plain freedesktop name first (works for Breeze, Papirus, most themes), then try with `-symbolic` suffix appended (needed for Adwaita which stores action icons only as symbolic variants).
**When to use:** Every icon lookup on Linux.
**Example:**
```rust
// Source: verified against local Adwaita/Breeze installations
use freedesktop_icons::lookup;
use std::path::PathBuf;

fn find_freedesktop_icon(name: &str, theme: &str, size: u16) -> Option<PathBuf> {
    // First try: plain name (e.g., "edit-copy")
    if let Some(path) = lookup(name)
        .with_theme(theme)
        .with_size(size)
        .force_svg()
        .find()
    {
        return Some(path);
    }
    // Second try: symbolic variant (e.g., "edit-copy-symbolic")
    // Needed for Adwaita where actions only exist as *-symbolic.svg
    let symbolic = format!("{name}-symbolic");
    lookup(&symbolic)
        .with_theme(theme)
        .with_size(size)
        .force_svg()
        .find()
}
```

### Pattern 2: Theme Detection Chain
**What:** Detect the active icon theme from GTK settings, with fallback to the crate's built-in detection.
**When to use:** When the caller does not specify a theme name.
**Example:**
```rust
// Source: verified against freedesktop-icons 0.4.0 API docs
fn detect_icon_theme() -> String {
    // freedesktop-icons provides GTK theme detection
    if let Some(theme) = freedesktop_icons::default_theme_gtk() {
        return theme;
    }
    // Fallback: always available
    "hicolor".to_string()
}
```

### Pattern 3: Full Fallback Chain (theme -> hicolor -> bundled)
**What:** The complete icon resolution pipeline required by success criteria.
**When to use:** The main public API for Linux icon loading.
**Example:**
```rust
use crate::{IconData, IconRole, IconSet, bundled_icon_svg, icon_name};

pub fn load_freedesktop_icon(role: IconRole) -> Option<IconData> {
    let fd_name = icon_name(IconSet::Freedesktop, role)?;
    let theme = detect_icon_theme();

    // Step 1: Try active theme (with hicolor fallback handled by crate)
    if let Some(path) = find_freedesktop_icon(fd_name, &theme, 24) {
        if let Ok(bytes) = std::fs::read(&path) {
            return Some(IconData::Svg(bytes));
        }
    }

    // Step 2: Bundled Material SVG fallback
    bundled_icon_svg(IconSet::Material, role)
        .map(|bytes| IconData::Svg(bytes.to_vec()))
}
```

### Anti-Patterns to Avoid
- **Hardcoding `/usr/share/icons`:** The freedesktop spec mandates checking `$XDG_DATA_HOME/icons`, `$HOME/.icons`, and all `$XDG_DATA_DIRS/icons`. The crate handles this; do not bypass it.
- **Assuming SVG always exists:** Some themes or hicolor may only have PNG for certain icons. Use `.force_svg()` to prefer SVG, but handle the fallback to bundled icons when no SVG is found.
- **Ignoring the `-symbolic` suffix:** Adwaita (GNOME 42+) moved most action/status/UI icons to symbolic-only variants. A single-pass lookup will miss roughly half of all icons on GNOME desktops.
- **Using `.with_cache()` unconditionally:** The crate's cache uses `once_cell` and is process-global. For a library crate that consumers may use alongside their own `freedesktop-icons` usage, avoid polluting the cache. Only use `.with_cache()` if documented as a performance feature.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| freedesktop icon lookup | Custom index.theme parser + directory walker | `freedesktop-icons` crate | The spec has 3 directory types (Fixed, Scalable, Threshold), scale factors, inheritance chains, pixmap fallback -- 800+ LoC for a reason |
| XDG base directory resolution | Manual env var parsing | `freedesktop-icons` (uses `xdg` + `dirs` internally) | Edge cases: empty XDG_DATA_DIRS, missing HOME, flatpak paths |
| GTK theme detection | Parse settings.ini manually | `freedesktop_icons::default_theme_gtk()` | GTK3 vs GTK4 paths, gsettings vs file, etc. |

**Key insight:** The freedesktop icon theme spec is deceptively complex. A naive implementation that just walks directories will fail on themes with inheritance (most of them), themes using Scalable directory types with MinSize/MaxSize, and themes that rely on hicolor as the universal fallback.

## Common Pitfalls

### Pitfall 1: Adwaita Symbolic-Only Icons
**What goes wrong:** Looking up `"edit-copy"` in Adwaita returns None because the file is named `edit-copy-symbolic.svg` in the `symbolic/actions/` directory.
**Why it happens:** GNOME 42+ moved to symbolic icons as the primary format. The freedesktop spec does NOT mandate that `"edit-copy"` should automatically resolve to `"edit-copy-symbolic"`.
**How to avoid:** Two-pass lookup: try plain name first, then try `"{name}-symbolic"`.
**Warning signs:** Tests pass with Breeze but fail with Adwaita on a GNOME system.

### Pitfall 2: Missing Freedesktop Names for Some IconRoles
**What goes wrong:** `icon_name(IconSet::Freedesktop, IconRole::Notification)` returns `None` -- there is no standard freedesktop name for a notification bell.
**Why it happens:** The freedesktop Icon Naming Specification does not cover every possible UI concept. Our mapping in Phase 16 already returns None for 1 role (Notification).
**How to avoid:** The fallback chain must handle `None` from `icon_name()` by going straight to bundled fallback.
**Warning signs:** Unwrapping `icon_name()` without handling None.

### Pitfall 3: Feature Flag Interaction
**What goes wrong:** Compilation errors on non-Linux platforms, or the `system-icons` feature pulling in Linux-only dependencies everywhere.
**Why it happens:** The `freedesktop-icons` crate only works on Linux (uses XDG paths, `/usr/share/icons`, etc.).
**How to avoid:** Use `#[cfg(target_os = "linux")]` on the module AND `[target.'cfg(target_os = "linux")'.dependencies]` in Cargo.toml. The feature flag `system-icons` should be platform-independent but the actual dependency must be platform-gated.
**Warning signs:** CI failures on macOS or Windows when `system-icons` is enabled.

### Pitfall 4: Theme Not Found Returns None, Not Error
**What goes wrong:** When the detected GTK theme is not installed (e.g., user set Papirus but it's not installed), lookups return None.
**Why it happens:** `freedesktop-icons` silently falls through to hicolor when the specified theme is missing.
**How to avoid:** This is actually correct behavior -- the fallback chain handles it. Document that missing themes degrade gracefully.
**Warning signs:** None -- this is expected behavior.

### Pitfall 5: Adwaita Icons in Unexpected Categories
**What goes wrong:** Looking for `"window-close"` in Adwaita's `actions/` context fails because it is in `ui/`.
**Why it happens:** Adwaita reorganized icons into different Context categories than Breeze.
**How to avoid:** The `freedesktop-icons` crate searches ALL directories listed in the theme's `index.theme`, not just `actions/`. This is handled correctly as long as you use the crate.
**Warning signs:** Only an issue if you hand-roll directory walking.

## Code Examples

Verified patterns from the codebase and official docs:

### Complete Linux Icon Loader Module
```rust
// Source: synthesis of freedesktop-icons 0.4.0 API + project patterns
#[cfg(target_os = "linux")]
pub(crate) mod freedesktop {
    use crate::{IconData, IconRole, IconSet, bundled_icon_svg, icon_name};
    use std::path::PathBuf;

    /// Detect the active freedesktop icon theme.
    fn detect_theme() -> String {
        freedesktop_icons::default_theme_gtk()
            .unwrap_or_else(|| "hicolor".to_string())
    }

    /// Look up an icon by freedesktop name, trying both plain and -symbolic.
    fn find_icon(name: &str, theme: &str, size: u16) -> Option<PathBuf> {
        // Plain name first (Breeze, Papirus, most themes)
        if let Some(path) = freedesktop_icons::lookup(name)
            .with_theme(theme)
            .with_size(size)
            .force_svg()
            .find()
        {
            return Some(path);
        }
        // Symbolic fallback (Adwaita)
        let symbolic = format!("{name}-symbolic");
        freedesktop_icons::lookup(&symbolic)
            .with_theme(theme)
            .with_size(size)
            .force_svg()
            .find()
    }

    /// Load a freedesktop icon for the given role.
    ///
    /// Fallback chain: active theme -> hicolor -> bundled Material SVGs.
    pub fn load_icon(role: IconRole) -> Option<IconData> {
        let fd_name = icon_name(IconSet::Freedesktop, role);
        let theme = detect_theme();

        if let Some(name) = fd_name {
            if let Some(path) = find_icon(name, &theme, 24) {
                if let Ok(bytes) = std::fs::read(&path) {
                    return Some(IconData::Svg(bytes));
                }
            }
        }

        // Bundled fallback
        bundled_icon_svg(IconSet::Material, role)
            .map(|bytes| IconData::Svg(bytes.to_vec()))
    }
}
```

### Cargo.toml Feature Configuration
```toml
# In native-theme/Cargo.toml
[features]
system-icons = ["dep:freedesktop-icons"]

[target.'cfg(target_os = "linux")'.dependencies]
freedesktop-icons = { version = "0.4", optional = true }
```

### Test Pattern: Testable with Temp Theme Directory
```rust
#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use super::*;

    #[test]
    fn load_icon_returns_svg_for_dialog_error() {
        // This test depends on having an icon theme installed
        // (virtually all Linux desktop systems have hicolor at minimum)
        let result = load_icon(IconRole::DialogError);
        // If no theme is installed at all, bundled fallback kicks in
        assert!(result.is_some());
        match result.unwrap() {
            IconData::Svg(bytes) => {
                let content = std::str::from_utf8(&bytes).unwrap();
                assert!(content.contains("<svg"));
            }
            _ => panic!("Expected SVG data"),
        }
    }

    #[test]
    fn load_icon_with_no_freedesktop_name_uses_bundled() {
        // IconRole::Notification has no freedesktop name
        let result = load_icon(IconRole::Notification);
        // Should fall back to bundled Material (if material-icons feature enabled)
        // or None (if no bundled features enabled)
        #[cfg(feature = "material-icons")]
        assert!(result.is_some());
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Scanning /usr/share/icons manually | Using freedesktop-icons crate | 2022+ | Proper spec compliance, inheritance, XDG |
| PNG-first icon lookup | SVG-first with force_svg() | freedesktop-icons 0.2+ | Better for toolkit-agnostic rendering |
| Adwaita with full raster icon set | Adwaita with symbolic-only icons | GNOME 42 (2022) | Must handle -symbolic suffix in lookups |

**Deprecated/outdated:**
- `/usr/share/pixmaps/` is legacy; the crate checks it as a last resort but new icons are never placed there
- XPM format icons (`.xpm`) are still in the spec but functionally extinct

## Open Questions

1. **Should the `system-icons` feature also require `material-icons` for the bundled fallback?**
   - What we know: Success criteria say "falls back to bundled Material SVGs". This requires `material-icons` feature.
   - What's unclear: Whether to make `material-icons` a dependency of `system-icons` or document it as a recommended combination.
   - Recommendation: Make `system-icons` imply `material-icons` via Cargo feature dependency: `system-icons = ["dep:freedesktop-icons", "material-icons"]`. This ensures the fallback chain always works.

2. **What size should we request for SVG icons?**
   - What we know: SVGs are scalable, so size matters less. The crate defaults to 24. Adwaita's symbolic icons declare MinSize=8, MaxSize=512.
   - What's unclear: Whether callers will want to specify size or if a default is fine.
   - Recommendation: Default to 24 (the crate's default and a common UI icon size). The size parameter mainly affects which size-specific directory is searched first. For scalable SVGs, it matters less.

3. **Should `detect_theme()` also check KDE's kdeglobals for icon theme?**
   - What we know: `freedesktop_icons::default_theme_gtk()` reads GTK settings.ini. KDE users typically also have GTK settings configured by KDE's settings module.
   - What's unclear: Whether there are KDE configurations where GTK settings.ini is absent.
   - Recommendation: Start with `default_theme_gtk()` only. KDE auto-generates GTK settings. If a gap is found, add KDE detection in a follow-up.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in) |
| Config file | native-theme/Cargo.toml (features) |
| Quick run command | `cargo test -p native-theme --features system-icons,material-icons --lib` |
| Full suite command | `cargo test -p native-theme --all-features` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PLAT-04-a | load_icon returns SVG bytes from active theme | integration | `cargo test -p native-theme --features system-icons,material-icons freedesktop::tests::load_icon_returns_svg -x` | Wave 0 |
| PLAT-04-b | Missing icon falls back to hicolor then bundled | integration | `cargo test -p native-theme --features system-icons,material-icons freedesktop::tests::fallback_to_bundled -x` | Wave 0 |
| PLAT-04-c | Respects XDG_DATA_DIRS | unit | `cargo test -p native-theme --features system-icons freedesktop::tests::xdg_data_dirs -x` | Wave 0 |
| PLAT-04-d | Works with Adwaita (symbolic suffix) | integration | `cargo test -p native-theme --features system-icons freedesktop::tests::adwaita_symbolic -x` | Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme --features system-icons,material-icons --lib`
- **Per wave merge:** `cargo test -p native-theme --all-features`
- **Phase gate:** Full suite green before verify-work

### Wave 0 Gaps
- [ ] `native-theme/src/freedesktop.rs` -- the main module (does not exist yet)
- [ ] Tests within the module covering PLAT-04 sub-behaviors

## Sources

### Primary (HIGH confidence)
- freedesktop-icons 0.4.0 docs.rs -- API surface, LookupBuilder methods, find() return type
- freedesktop-icons GitHub (oknozor/freedesktop-icons) -- source analysis of theme inheritance, force_svg, symbolic handling
- Local filesystem verification -- confirmed icon locations in /usr/share/icons/Adwaita, /usr/share/icons/breeze, /usr/share/icons/hicolor

### Secondary (MEDIUM confidence)
- freedesktop-icons crates.io API -- version 0.4.0, 28K downloads for latest, MIT license, April 2025 release
- freedesktop Icon Theme Specification (specifications.freedesktop.org) -- referenced but could not fully fetch (redirect); findings verified against local theme structures

### Tertiary (LOW confidence)
- None -- all critical claims verified against installed themes and crate documentation

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - freedesktop-icons is the only serious Rust crate for this (3.4M total downloads), API verified on docs.rs
- Architecture: HIGH - verified icon locations on actual Linux system, tested naming conventions across Adwaita and Breeze
- Pitfalls: HIGH - the -symbolic suffix issue was discovered by actual filesystem inspection of two major themes
- Fallback chain: HIGH - bundled_icon_svg() from Phase 17 is already implemented and tested

**Research date:** 2026-03-09
**Valid until:** 2026-04-09 (stable domain, freedesktop spec rarely changes)
