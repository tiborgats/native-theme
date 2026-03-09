# Architecture Patterns: Native Icon Loading Integration

**Domain:** Toolkit-agnostic native icon loading for the native-theme Rust crate (v0.3)
**Researched:** 2026-03-09
**Confidence:** HIGH -- based on full source code analysis of v0.2 codebase plus ecosystem research

## Recommended Architecture

The icon loading system integrates as a new domain within the existing native-theme crate, following the same patterns as colors/fonts/geometry: data model types in the core crate, platform implementations behind `#[cfg]`, and bundled fallback assets embedded at compile time.

### Architecture Overview

```
native-theme crate (existing, modified)
  src/
    model/
      mod.rs          -- ThemeVariant gains icon_theme: Option<String>
      colors.rs       -- (unchanged)
      fonts.rs        -- (unchanged)
      ...
    icons/            -- NEW module
      mod.rs          -- IconRole, IconData, load_icon(), icon_name(), system_icon_set()
      names.rs        -- Static mapping tables: IconRole -> &str per icon set
      bundled.rs      -- include_bytes! for Material/Lucide SVGs (feature-gated)
      freedesktop.rs  -- Linux icon lookup via freedesktop spec
      macos.rs        -- NSImage systemSymbolName loading
      windows.rs      -- SHGetStockIconInfo + Segoe Fluent font rendering
    icons/            -- SVG asset directories
      material/       -- 42 optimized SVGs
      lucide/         -- 42 optimized SVGs
    presets/          -- TOML files gain icon_theme field
    lib.rs            -- re-exports IconRole, IconData, load_icon, icon_name, system_icon_set
```

### Data Flow

```
User calls:
  load_icon(icon_theme, role, size) -> Option<IconData>
    |
    +-- resolve "system" -> system_icon_set() (one level)
    |
    +-- match icon_theme:
    |     "sf-symbols"   -> icons::macos::load(role, size)       [cfg(target_os="macos")]
    |     "segoe-fluent" -> icons::windows::load(role, size)     [cfg(target_os="windows")]
    |     "freedesktop"  -> icons::freedesktop::load(role, size) [cfg(target_os="linux")]
    |     "material"     -> icons::bundled::load_material(role)  [cfg(feature="material-icons")]
    |     "lucide"       -> icons::bundled::load_lucide(role)    [cfg(feature="lucide-icons")]
    |     _              -> None
    |
    +-- If None, try fallback: material > lucide > None
    |
    +-- Returns: IconData::Svg(Vec<u8>) or IconData::Rgba { width, height, data }
```

---

## Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| `icons/mod.rs` | Public API: `load_icon()`, `icon_name()`, `system_icon_set()`, type definitions | Dispatches to `names`, `bundled`, `freedesktop`, `macos`, `windows` |
| `icons/names.rs` | Static `IconRole -> &str` mapping tables per icon set (5 functions) | Called by `icon_name()` and by each platform loader |
| `icons/bundled.rs` | Compile-time SVG embedding via `include_bytes!`, lookup by `IconRole` | Called by `load_icon()` for "material"/"lucide" themes |
| `icons/freedesktop.rs` | Linux icon lookup via `freedesktop-icons` crate | Called by `load_icon()` for "freedesktop", uses `names::freedesktop_name()` |
| `icons/macos.rs` | NSImage(systemSymbolName:) rasterization to RGBA | Called by `load_icon()` for "sf-symbols", uses `names::sf_symbols_name()` |
| `icons/windows.rs` | SHGetStockIconInfo + Segoe Fluent font glyph rendering | Called by `load_icon()` for "segoe-fluent", uses `names::segoe_fluent_*()` |
| `model/mod.rs` | ThemeVariant gains `icon_theme: Option<String>` | Serialized in preset TOMLs, read by connectors |
| Connectors | Convert `IconData` to toolkit image types, OR use `icon_name()` to leverage own icons | Call `load_icon()` / `icon_name()`, never contain platform logic |

---

## Design Decisions

### Decision 1: IconRole lives in the core crate, not a separate crate

IconRole, IconData, and the public API functions (`load_icon`, `icon_name`, `system_icon_set`) live in the native-theme core crate inside a new `icons` module.

**Rationale:**

- **Consistency.** Colors, fonts, geometry, spacing, widget_metrics all live in the core crate. Icons are the same category of theme data.
- **Avoids circular dependency.** The core crate needs `icon_theme: Option<String>` on `ThemeVariant`. If icons were in a separate crate, the core would need icon types for `ThemeVariant` and the icon crate would need `ThemeVariant` -- circular.
- **Reuses existing cfg patterns.** The core crate already has `macos.rs`, `windows.rs`, `kde/`, `gnome/` behind `#[cfg]`. Adding `icons/macos.rs` follows the same pattern.
- **Feature flags integrate naturally.** `system-icons`, `material-icons`, `lucide-icons` sit alongside existing `kde`, `portal`, `macos`, `windows` flags in the same Cargo.toml.

The `icons/` module is a sibling to `model/`, not nested inside it, because it contains runtime loading logic (file I/O, FFI), not just data types.

### Decision 2: load_icon() as a free function, not a method on ThemeVariant

`load_icon()` is a **free function** in the `icons` module, re-exported from `lib.rs`.

**Rationale:**

- **ThemeVariant is a pure data struct.** It derives Serialize/Deserialize, has `merge()` and `is_empty()`, and holds `Option<T>` fields. Adding platform FFI methods breaks this clean separation.
- **load_icon() takes parameters not on ThemeVariant.** It needs `size: u32` (a rendering concern) and the `icon_theme` string. These are external to the data model.
- **Matches existing API patterns.** `from_system()`, `from_kde()`, `from_macos()` are all free functions.
- **Easier testing.** Free functions can be tested without constructing ThemeVariant instances.

**Connector convenience pattern (2 lines):**

```rust
let icon_theme = variant.icon_theme.as_deref()
    .unwrap_or_else(|| native_theme::system_icon_set());
let icon = native_theme::load_icon(icon_theme, role, size);
```

### Decision 3: Feature flag structure -- new flags, not reusing existing

Three new feature flags, separate from the existing theme-reading flags:

```toml
[features]
default = ["system-icons", "material-icons"]
system-icons = []          # Platform-native icon loading (macOS/Windows/Linux APIs)
material-icons = []        # Bundle 42 Material Symbols SVGs as cross-platform fallback
lucide-icons = []          # Bundle 42 Lucide SVGs as optional icon set
```

**Why separate from `macos`/`windows` features:**

- A user might want `macos` (color reading) without icon loading, or vice versa.
- `system-icons` adds new dependency features (`NSImage`, `Win32_UI_Shell`, `freedesktop-icons`) that increase compile time.
- On Linux, `system-icons` adds the `freedesktop-icons` crate which `kde`/`portal` features do not need.

**Dependency mapping:**

| Feature | Platform | Dependency change |
|---------|----------|-------------------|
| `system-icons` on macOS | macOS | `objc2-app-kit` -- add `NSImage` feature (already optional dep) |
| `system-icons` on Windows | Windows | `windows` -- add `Win32_UI_Shell` feature (already optional dep) |
| `system-icons` on Linux | Linux | New optional dep: `freedesktop-icons = "0.4"` |
| `material-icons` | All | None (SVGs embedded via `include_bytes!`) |
| `lucide-icons` | All | None (SVGs embedded via `include_bytes!`) |

**Cargo.toml changes:**

```toml
[target.'cfg(target_os = "macos")'.dependencies]
objc2-app-kit = { version = "0.3", optional = true, features = [
    "NSColor", "NSColorSpace", "NSAppearance", "NSFont", "NSFontDescriptor",
    "NSImage",  # NEW: for systemSymbolName icon loading
    "objc2-core-foundation",
] }

[target.'cfg(target_os = "linux")'.dependencies]
freedesktop-icons = { version = "0.4", optional = true }

[dependencies]
windows = { version = ">=0.59, <=0.62", optional = true, default-features = false, features = [
    "UI_ViewManagement",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_HiDpi",
    "Win32_Graphics_Gdi",
    "Win32_UI_Shell",      # NEW: for SHGetStockIconInfo
    "Foundation_Metadata",
] }
```

**system-icons feature gate pattern:**

The `system-icons` feature does not directly imply `macos`, `windows`, or `kde`. Instead, each platform loader uses compound cfg:

```rust
#[cfg(all(feature = "system-icons", target_os = "macos", feature = "macos"))]
// or just:
#[cfg(all(feature = "system-icons", target_os = "macos"))]
```

The simpler approach: `system-icons` on macOS implies using the `objc2-app-kit` dep that is already gated behind `cfg(target_os = "macos")`. The NSImage feature is added to the existing objc2-app-kit dep unconditionally (it only compiles on macOS anyway). This avoids complex feature interaction.

### Decision 4: Bundled SVGs via include_bytes!, not include_dir! or build.rs

**One `include_bytes!` per SVG file, in a dedicated `bundled.rs` module.** Only 42 SVGs per icon set (one per IconRole).

**Why not `include_dir!` or `rust-embed`:**

- Those crates are designed for hundreds/thousands of files. For 42 known, fixed files, explicit `include_bytes!` is simpler.
- No proc-macro dependency required.
- The mapping from IconRole to SVG bytes is a direct match expression -- no directory traversal at runtime.

**Why not a build script:**

- SVGs are optimized at development time (run SVGO once, commit results). No transformation needed at build time.
- build.rs adds complexity and makes `cargo publish` harder (asset files must be in the published package).

**Binary size impact:** At ~200-500 bytes per optimized SVG, 42 icons = ~10-20KB per set. With both Material and Lucide: ~20-40KB. Negligible.

**Compile time impact:** The [rust-lang/rust#65818](https://github.com/rust-lang/rust/issues/65818) issue affects large binary blobs (MB-scale). At 42 small SVGs totaling <20KB, there is no measurable compile time impact.

**File structure:**

```
native-theme/src/
  icons/
    material/          # 42 SVG files, pre-optimized with SVGO
      warning.svg
      error.svg
      info.svg
      ...
    lucide/            # 42 SVG files, pre-optimized
      triangle-alert.svg
      circle-x.svg
      ...
    bundled.rs         # include_bytes! + match dispatch
```

**Implementation pattern:**

```rust
#[cfg(feature = "material-icons")]
mod material_svgs {
    pub const WARNING: &[u8] = include_bytes!("material/warning.svg");
    pub const ERROR: &[u8] = include_bytes!("material/error.svg");
    // ... 42 total, one per IconRole
}

#[cfg(feature = "material-icons")]
pub fn load_material(role: IconRole) -> Option<IconData> {
    let svg = match role {
        IconRole::DialogWarning => material_svgs::WARNING,
        IconRole::DialogError => material_svgs::ERROR,
        // ... exhaustive match
    };
    Some(IconData::Svg(svg.to_vec()))
}
```

### Decision 5: Match statements for icon name mapping, not phf

**Use `match` statements.** 42 arms per icon set.

**Why not phf:**

- At 42 entries, match compiles to a jump table or binary search -- O(1) or O(log n), effectively instant.
- No extra dependency (`phf` requires `phf_macros` proc-macro).
- Better IDE support: exhaustiveness checking, jump-to-definition, readable at a glance.
- The [mega-match-vs-phf benchmark](https://github.com/lmammino/mega-match-vs-phf) shows match is competitive with phf up to hundreds of entries.

**Implementation -- one function per icon set:**

```rust
// icons/names.rs

pub fn sf_symbols_name(role: IconRole) -> Option<&'static str> {
    match role {
        IconRole::DialogWarning => Some("exclamationmark.triangle.fill"),
        IconRole::DialogError => Some("xmark.circle.fill"),
        IconRole::DialogInfo => Some("info.circle.fill"),
        // ... 42 arms total, None for roles without SF Symbol equivalent
    }
}

pub fn freedesktop_name(role: IconRole) -> Option<&'static str> {
    match role {
        IconRole::DialogWarning => Some("dialog-warning"),
        // ...
    }
}

pub fn material_name(role: IconRole) -> Option<&'static str> { ... }
pub fn lucide_name(role: IconRole) -> Option<&'static str> { ... }
pub fn segoe_fluent_name(role: IconRole) -> Option<&'static str> { ... }

/// Segoe Fluent uses codepoints, not names, for glyph rendering.
pub fn segoe_fluent_codepoint(role: IconRole) -> Option<char> {
    match role {
        IconRole::ActionSave => Some('\u{E74E}'),
        IconRole::WindowClose => Some('\u{E8BB}'),
        // ...
    }
}
```

**Public `icon_name()` dispatches:**

```rust
pub fn icon_name(icon_theme: &str, role: IconRole) -> Option<&'static str> {
    match icon_theme {
        "sf-symbols" => names::sf_symbols_name(role),
        "segoe-fluent" => names::segoe_fluent_name(role),
        "freedesktop" => names::freedesktop_name(role),
        "material" => names::material_name(role),
        "lucide" => names::lucide_name(role),
        "system" => icon_name(system_icon_set(), role),
        _ => None,
    }
}
```

### Decision 6: icon_theme on ThemeVariant -- merge and serialization

Add `icon_theme: Option<String>` to `ThemeVariant`:

```rust
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeVariant {
    #[serde(default, skip_serializing_if = "ThemeColors::is_empty")]
    pub colors: ThemeColors,
    #[serde(default, skip_serializing_if = "ThemeFonts::is_empty")]
    pub fonts: ThemeFonts,
    #[serde(default, skip_serializing_if = "ThemeGeometry::is_empty")]
    pub geometry: ThemeGeometry,
    #[serde(default, skip_serializing_if = "ThemeSpacing::is_empty")]
    pub spacing: ThemeSpacing,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_metrics: Option<WidgetMetrics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_theme: Option<String>,  // NEW
}
```

**TOML serialization:**

```toml
# Native preset (e.g., macos-sonoma.toml)
[light]
icon_theme = "sf-symbols"

[light.colors]
accent = "#007aff"
# ...

# Community preset (e.g., catppuccin-mocha.toml)
# icon_theme omitted -> None -> resolved at runtime via system_icon_set()
[light.colors]
accent = "#89b4fa"
```

**merge() behavior:**

ThemeVariant has a manual `merge()` implementation (not `impl_merge!`) because of the `widget_metrics: Option<WidgetMetrics>` special case. The `icon_theme` field uses simple Option replacement:

```rust
impl ThemeVariant {
    pub fn merge(&mut self, overlay: &Self) {
        self.colors.merge(&overlay.colors);
        self.fonts.merge(&overlay.fonts);
        self.geometry.merge(&overlay.geometry);
        self.spacing.merge(&overlay.spacing);

        match (&mut self.widget_metrics, &overlay.widget_metrics) {
            (Some(base), Some(over)) => base.merge(over),
            (None, Some(over)) => self.widget_metrics = Some(over.clone()),
            _ => {}
        }

        // NEW: icon_theme uses simple Option replacement
        if overlay.icon_theme.is_some() {
            self.icon_theme = overlay.icon_theme.clone();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
            && self.fonts.is_empty()
            && self.geometry.is_empty()
            && self.spacing.is_empty()
            && self.widget_metrics.as_ref().is_none_or(|wm| wm.is_empty())
            && self.icon_theme.is_none()  // NEW
    }
}
```

**Non-breaking:** ThemeVariant is `#[non_exhaustive]`, so adding a field does not break downstream construction (callers must already use `..Default::default()`). Adding `icon_theme` to preset TOML files is a data change, not an API break. Old TOML files without `icon_theme` deserialize correctly (serde default = None).

### Decision 7: Connector integration -- dual strategy

Connectors have two paths for icon usage:

**Path A: Use `load_icon()` and convert `IconData`** -- for connectors without their own icon library.

**Path B: Use `icon_name()` and leverage the toolkit's own icons** -- for connectors where the toolkit already bundles icons (e.g., gpui-component has 87 Lucide icons).

**gpui connector -- Lucide shortcut + IconData fallback:**

```rust
// native-theme-gpui/src/icons.rs

/// Try to map IconRole to gpui-component's built-in IconName.
/// Only works for the 27/42 roles that gpui-component covers.
pub fn icon_name_for_role(role: IconRole) -> Option<IconName> {
    match role {
        IconRole::DialogWarning => Some(IconName::TriangleAlert),
        IconRole::DialogError => Some(IconName::CircleX),
        IconRole::ActionSearch => Some(IconName::Search),
        // ... 27 mappings total
        _ => None,
    }
}

/// Load an icon, using gpui-component's built-in Lucide icons when possible,
/// falling back to native-theme's load_icon() for other icon themes.
pub fn load_icon(
    variant: &native_theme::ThemeVariant,
    role: IconRole,
    size: u32,
) -> Option<GpuiIcon> {
    let icon_theme = variant.icon_theme.as_deref()
        .unwrap_or_else(|| native_theme::system_icon_set());

    // Fast path: Lucide + gpui-component has the icon -> use directly
    if icon_theme == "lucide" {
        if let Some(name) = icon_name_for_role(role) {
            return Some(GpuiIcon::Named(name));
        }
    }

    // Standard path: load via native-theme and convert
    let icon_data = native_theme::load_icon(icon_theme, role, size)?;
    Some(match icon_data {
        IconData::Rgba { width, height, data } => {
            GpuiIcon::Raster(/* convert to gpui RenderImage */)
        }
        IconData::Svg(bytes) => {
            GpuiIcon::Svg(/* render SVG to gpui image */)
        }
    })
}
```

**iced connector -- always uses IconData:**

iced does not bundle icon sets, so it always goes through `load_icon()` and converts `IconData` to iced's image type.

**Key principle:** Connectors contain NO platform logic. All platform-specific icon resolution happens in the core crate. Connectors are pure format converters.

---

## Build Order -- Dependency Graph

```
Phase 1: Data model + name mapping (no FFI, no bundled assets)
  |
  +-- IconRole enum (42 variants)
  +-- IconData enum (Svg/Rgba)
  +-- icon_theme: Option<String> on ThemeVariant (merge + is_empty + serde)
  +-- names.rs: 5 mapping functions (sf_symbols, segoe_fluent, freedesktop, material, lucide)
  +-- icon_name() public function (dispatches to names.rs)
  +-- system_icon_set() public function
  +-- Update 6 native preset TOMLs with icon_theme values
  +-- lib.rs re-exports
  +-- Tests for all of the above
  |
  v
Phase 2: Bundled icon sets (no FFI, compile-time only)
  |
  +-- Download + SVGO-optimize 42 Material Symbols SVGs
  +-- Download + SVGO-optimize 42 Lucide SVGs
  +-- bundled.rs with include_bytes! + load_material/load_lucide functions
  +-- material-icons and lucide-icons feature flags in Cargo.toml
  +-- load_icon() dispatch to bundled loaders + fallback chain
  +-- Tests: verify all 42 roles return Some(IconData::Svg) for each set
  |
  v
Phase 3: Platform-native loading (FFI, platform-specific)
  |  (sub-phases can be parallelized)
  |
  +-- 3a: Linux/freedesktop
  |     icons/freedesktop.rs using freedesktop-icons 0.4 crate
  |     system-icons feature + freedesktop-icons optional dep
  |     CI: test on Ubuntu runner
  |
  +-- 3b: macOS
  |     icons/macos.rs using NSImage(systemSymbolName:) + rasterization
  |     system-icons + NSImage feature on objc2-app-kit
  |     CI: test on macOS runner
  |
  +-- 3c: Windows
  |     icons/windows.rs using SHGetStockIconInfo + Segoe Fluent font
  |     system-icons + Win32_UI_Shell on windows crate
  |     CI: test on Windows runner
  |
  v
Phase 4: Connector integration
  |
  +-- native-theme-gpui: icons module (Lucide shortcut + IconData conversion)
  +-- native-theme-iced: icons module (IconData conversion)
  +-- Update connector examples with icon loading demos
```

**Rationale for this order:**

1. **Phase 1 first** because everything else depends on `IconRole`, `IconData`, and the name mapping tables. This phase has zero new dependencies and can be fully tested on any platform.

2. **Phase 2 before Phase 3** because bundled SVGs provide the fallback that platform loaders need when a native icon is unavailable (e.g., `trash-full` on macOS, `status-loading` on Windows). Having fallbacks working first means Phase 3 can focus purely on platform FFI.

3. **Phase 3 platforms are independent.** Each adds its own cfg-gated code and dependencies. Can be developed and CI-tested in parallel.

4. **Phase 4 last** because connectors consume the core crate's icon API. They should only be updated after the API is stable and tested.

---

## New vs Modified Components

### New files (to create from scratch)

| File | Purpose | Dependencies |
|------|---------|-------------|
| `src/icons/mod.rs` | Module root, IconRole/IconData types, public API, dispatch | None new |
| `src/icons/names.rs` | 5+ mapping functions, 42 match arms each | None |
| `src/icons/bundled.rs` | `include_bytes!` for Material + Lucide, load functions | None |
| `src/icons/freedesktop.rs` | Linux icon file lookup | `freedesktop-icons` 0.4 (new optional dep) |
| `src/icons/macos.rs` | NSImage systemSymbolName loading + RGBA rasterization | `objc2-app-kit` (existing dep, add NSImage feature) |
| `src/icons/windows.rs` | SHGetStockIconInfo + Segoe Fluent glyph rendering | `windows` (existing dep, add Win32_UI_Shell feature) |
| `src/icons/material/*.svg` | 42 optimized Material Symbol SVGs | N/A (asset files) |
| `src/icons/lucide/*.svg` | 42 optimized Lucide SVGs | N/A (asset files) |

### Modified files (existing, to update)

| File | Change | Phase |
|------|--------|-------|
| `src/model/mod.rs` | Add `icon_theme: Option<String>` to ThemeVariant, update `merge()` and `is_empty()` | 1 |
| `src/lib.rs` | Add `pub mod icons;`, re-export IconRole, IconData, load_icon, icon_name, system_icon_set | 1 |
| `Cargo.toml` | Add 3 feature flags; `freedesktop-icons` optional dep; extend `objc2-app-kit` and `windows` features | 1-3 |
| `src/presets/windows-11.toml` | Add `icon_theme = "segoe-fluent"` to both variants | 1 |
| `src/presets/macos-sonoma.toml` | Add `icon_theme = "sf-symbols"` to both variants | 1 |
| `src/presets/ios.toml` | Add `icon_theme = "sf-symbols"` to both variants | 1 |
| `src/presets/adwaita.toml` | Add `icon_theme = "freedesktop"` to both variants | 1 |
| `src/presets/kde-breeze.toml` | Add `icon_theme = "freedesktop"` to both variants | 1 |
| `src/presets/material.toml` | Add `icon_theme = "material"` to both variants | 1 |
| Community preset TOMLs (11 files) | No change (icon_theme stays None = system default) | -- |
| `connectors/native-theme-gpui/src/` | Add icons module with Lucide shortcut + load_icon wrapper | 4 |
| `connectors/native-theme-iced/src/` | Add icons module with load_icon wrapper | 4 |

---

## Platform-Specific Implementation Details

### macOS: NSImage(systemSymbolName:) via objc2-app-kit

The existing `from_macos()` already uses `objc2-app-kit` with `NSColor`, `NSFont`. Icon loading adds `NSImage`:

```rust
use objc2_app_kit::NSImage;
use objc2_foundation::NSString;

pub fn load(role: IconRole, size: u32) -> Option<IconData> {
    let name = crate::icons::names::sf_symbols_name(role)?;
    let ns_name = NSString::from_str(name);
    let image = unsafe {
        NSImage::imageWithSystemSymbolName_accessibilityDescription(&ns_name, None)
    }?;
    // Rasterize: create NSBitmapImageRep at requested size, extract RGBA pixels
    // The image needs to be drawn into a bitmap context at the target size
    let rgba_data = rasterize_nsimage(&image, size)?;
    Some(IconData::Rgba { width: size, height: size, data: rgba_data })
}
```

**Rasterization complexity:** SF Symbols are vector images. Converting to RGBA pixels requires:
1. Creating an `NSBitmapImageRep` of the target size
2. Locking focus on it
3. Drawing the NSImage into the rep
4. Extracting the pixel data

This is the most complex platform implementation (~30-50 lines of unsafe objc2 code). The `NSImage` feature on `objc2-app-kit` (v0.3) provides the necessary bindings.

### Windows: SHGetStockIconInfo + Segoe Fluent Icons

Two loading paths depending on the icon role:

**Stock icons** (dialog, file, system roles): Use `SHGetStockIconInfo` from the `windows` crate with the `Win32_UI_Shell` feature. Returns an HICON handle, which is converted to RGBA pixels via `GetIconInfo` + `GetDIBits`.

```rust
use windows::Win32::UI::Shell::{SHGetStockIconInfo, SHSTOCKICONINFO, SHGSI_ICON};

pub fn load_stock_icon(siid: u32, size: u32) -> Option<IconData> {
    let mut sii = SHSTOCKICONINFO::default();
    sii.cbSize = std::mem::size_of::<SHSTOCKICONINFO>() as u32;
    unsafe { SHGetStockIconInfo(siid, SHGSI_ICON, &mut sii) }.ok()?;
    let hicon = sii.hIcon;
    let rgba = hicon_to_rgba(hicon, size)?;
    // DestroyIcon(hicon) after extraction
    Some(IconData::Rgba { width: size, height: size, data: rgba })
}
```

**UI action icons** (save, copy, navigation): Load the "Segoe Fluent Icons" system font, render the glyph at the codepoint using GDI or DirectWrite. This requires either `Win32_Graphics_Gdi` (already in deps) or `Win32_Graphics_DirectWrite`.

### Linux: freedesktop-icons crate (v0.4)

The `freedesktop-icons` crate implements the full freedesktop Icon Theme Specification:

```rust
use freedesktop_icons::lookup;

pub fn load(role: IconRole, size: u32) -> Option<IconData> {
    let name = crate::icons::names::freedesktop_name(role)?;
    let path = lookup(name)
        .with_size(size as u16)
        .force_svg()
        .find()?;
    let svg_bytes = std::fs::read(&path).ok()?;
    Some(IconData::Svg(svg_bytes))
}
```

The crate handles:
- Theme inheritance chain traversal
- Fallback to `hicolor` theme
- Fallback to `/usr/share/pixmaps`
- Size matching with closest-size fallback
- SVG vs PNG preference via `force_svg()`

At 0.4, the crate provides a `with_cache()` option for repeated lookups, and supports `with_theme()` to specify the active icon theme. The default behavior uses the system GTK theme setting.

---

## Patterns to Follow

### Pattern 1: Feature-gated platform code with stub fallback

Every platform-specific icon loader uses a dual `#[cfg]` pattern -- real implementation when the feature + platform match, stub returning `None` otherwise:

```rust
// icons/macos.rs

#[cfg(all(feature = "system-icons", target_os = "macos"))]
pub fn load(role: IconRole, size: u32) -> Option<IconData> {
    let name = crate::icons::names::sf_symbols_name(role)?;
    // ... NSImage loading + rasterization ...
    Some(IconData::Rgba { width: size, height: size, data })
}

#[cfg(not(all(feature = "system-icons", target_os = "macos")))]
pub fn load(_role: IconRole, _size: u32) -> Option<IconData> {
    None  // Not available on this platform/config
}
```

This matches the existing pattern in `src/macos.rs` where the real `from_macos()` is behind `#[cfg(feature = "macos")]` and the module itself is behind `#[cfg(target_os = "macos")]`.

### Pattern 2: Dispatch-then-fallback in load_icon()

`load_icon()` tries the requested icon theme first. If it returns `None` (icon not available in that theme), it falls back to bundled icon sets:

```rust
pub fn load_icon(icon_theme: &str, role: IconRole, size: u32) -> Option<IconData> {
    let theme = if icon_theme == "system" { system_icon_set() } else { icon_theme };

    let result = match theme {
        "sf-symbols" => macos::load(role, size),
        "segoe-fluent" => windows::load(role, size),
        "freedesktop" => freedesktop::load(role, size),
        "material" => {
            #[cfg(feature = "material-icons")]
            { bundled::load_material(role) }
            #[cfg(not(feature = "material-icons"))]
            { None }
        }
        "lucide" => {
            #[cfg(feature = "lucide-icons")]
            { bundled::load_lucide(role) }
            #[cfg(not(feature = "lucide-icons"))]
            { None }
        }
        _ => None,
    };

    if result.is_some() { return result; }

    // Fallback chain: material > lucide > None
    #[cfg(feature = "material-icons")]
    if let Some(icon) = bundled::load_material(role) { return Some(icon); }
    #[cfg(feature = "lucide-icons")]
    if let Some(icon) = bundled::load_lucide(role) { return Some(icon); }

    None
}
```

### Pattern 3: One mapping function per icon set

Each icon set gets its own pure function in `names.rs`. No shared trait, no generic table struct. Each function is self-contained with an exhaustive match:

```rust
pub fn sf_symbols_name(role: IconRole) -> Option<&'static str> { ... }
pub fn freedesktop_name(role: IconRole) -> Option<&'static str> { ... }
pub fn material_name(role: IconRole) -> Option<&'static str> { ... }
pub fn lucide_name(role: IconRole) -> Option<&'static str> { ... }
pub fn segoe_fluent_name(role: IconRole) -> Option<&'static str> { ... }
```

**Why no trait:** Each icon set has different naming conventions and coverage gaps. A trait like `IconSet::name(&self, role) -> Option<&str>` adds vtable indirection and trait object boxing for no benefit. The dispatch in `icon_name()` is already a 6-arm match.

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Trait-based icon loader abstraction

**What:** `trait IconLoader { fn load(&self, role: IconRole, size: u32) -> Option<IconData>; }` implemented per platform.

**Why bad:** Each platform has fundamentally different initialization needs. NSImage needs no state. Freedesktop needs a theme name. Windows needs HICON handles. A trait forces a common interface that does not fit. The dispatch in `load_icon()` is 10 lines of match -- a trait adds abstraction for no reduction in complexity.

**Instead:** Free functions per platform, dispatched by match.

### Anti-Pattern 2: Embedding full icon libraries

**What:** Bundling all 3,800 Material Symbols or 1,700 Lucide icons.

**Why bad:** Adds 2-10MB to binary for icons that will never be used. Only 42 roles are defined. Compile time increases dramatically with thousands of `include_bytes!` calls.

**Instead:** Bundle exactly 42 SVGs per icon set. Users needing the full library can use dedicated crates (`md-icons`, `icondata`).

### Anti-Pattern 3: Platform logic in connectors

**What:** Putting NSImage or SHGetStockIconInfo calls in `native-theme-gpui` or `native-theme-iced`.

**Why bad:** Duplicates platform logic. Every new connector must reimplement. Connectors become platform-specific.

**Instead:** All platform logic in the core crate. Connectors are pure format converters.

### Anti-Pattern 4: Lazy-static HashMap for bundled SVGs

**What:** `lazy_static!` or `OnceLock` initializing a `HashMap<IconRole, &[u8]>`.

**Why bad:** `include_bytes!` produces `&'static [u8]` in the binary's data segment. A HashMap adds heap allocation and hashing for 42 entries that a match statement handles in constant time with zero allocation.

**Instead:** Direct match on IconRole returning the `include_bytes!` reference.

### Anti-Pattern 5: IconRole as string-based keys

**What:** `load_icon("sf-symbols", "dialog-warning", 24)` instead of `load_icon("sf-symbols", IconRole::DialogWarning, 24)`.

**Why bad:** No compile-time exhaustiveness checking. Typos become runtime `None`. Cannot derive Hash, Eq, Copy for collections.

**Instead:** `IconRole` enum with `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`.

---

## Scalability Considerations

| Concern | 42 roles (v0.3) | 100+ roles (future) | 500+ roles (unlikely) |
|---------|----------------|---------------------|----------------------|
| IconRole enum | 1 byte discriminant | Still 1 byte (up to 255) | 2 bytes, still trivial |
| Name mapping | 42-arm match per set | Match still fine | Consider phf at this scale |
| Bundled SVG binary size | ~10-20KB per set | ~50-100KB per set | ~250-500KB, may want lazy loading |
| Compile time | No measurable impact | Negligible | Benchmark include_bytes! count |
| load_icon() dispatch | 6-way match on theme | Same 6-way match | Same -- theme count does not scale with roles |

The architecture handles up to ~200 IconRole variants without changes. Beyond that, bundled SVGs may need separate asset crates, but the API surface (`load_icon`, `icon_name`, `IconRole`, `IconData`) remains stable.

---

## Sources

- [freedesktop-icons crate v0.4](https://docs.rs/freedesktop-icons/latest/freedesktop_icons/) - Rust freedesktop icon theme lookup (HIGH confidence)
- [NSImage initWithSystemSymbolName Apple Docs](https://developer.apple.com/documentation/appkit/nsimage/init(systemsymbolname:accessibilitydescription:)) - macOS SF Symbol loading (HIGH confidence)
- [objc2-app-kit NSImage struct](https://docs.rs/objc2-app-kit/latest/objc2_app_kit/struct.NSImage.html) - Rust bindings for NSImage (HIGH confidence)
- [SHGetStockIconInfo Win32 Docs](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetstockiconinfo) - Windows stock icon loading (HIGH confidence)
- [SHSTOCKICONID enumeration](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ne-shellapi-shstockiconid) - Windows stock icon identifiers (HIGH confidence)
- [freedesktop Icon Theme Specification](https://specifications.freedesktop.org/icon-theme-spec/) - Linux icon lookup spec (HIGH confidence)
- [mega-match-vs-phf benchmark](https://github.com/lmammino/mega-match-vs-phf) - Match vs PHF performance (MEDIUM confidence)
- [rust-lang/rust#65818](https://github.com/rust-lang/rust/issues/65818) - include_bytes! compile time with large blobs (MEDIUM confidence)
- [include_dir crate](https://docs.rs/include_dir) - Directory embedding alternative, evaluated and rejected (MEDIUM confidence)
- [md-icons Rust crate](https://github.com/codefionn/md-icons-rs) - Material Design SVGs for Rust reference (LOW confidence)
- Full source analysis of native-theme v0.2 codebase: lib.rs, model/mod.rs, model/colors.rs, model/geometry.rs, presets.rs, macos.rs, windows.rs, kde/mod.rs, gnome/mod.rs, connector crates (HIGH confidence)
- docs/native-icons.md specification document (HIGH confidence)
