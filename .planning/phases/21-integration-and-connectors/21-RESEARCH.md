# Phase 21: Integration and Connectors - Research

**Researched:** 2026-03-09
**Domain:** Icon pipeline dispatch, SVG rasterization, toolkit connector updates (gpui + iced)
**Confidence:** HIGH

## Summary

Phase 21 connects all the icon infrastructure built in Phases 16-20 into an end-to-end pipeline. The work divides into five clear deliverables: (1) a `load_icon()` dispatch function in the core crate that routes to platform loaders and falls back to bundled icons, (2) an optional SVG-to-RGBA rasterization module using `resvg`, (3) gpui connector updates for `IconData` conversion and a zero-I/O Lucide shortcut mapping `IconRole` to gpui-component `IconName`, (4) iced connector updates for `IconData` to `iced_core::image::Handle` conversion, and (5) gpui example updates with icon display and an icon set selector dropdown.

All three platform loaders (`load_freedesktop_icon`, `load_sf_icon`, `load_windows_icon`) already exist with identical signatures (`IconRole -> Option<IconData>`) and internal fallback to bundled Material SVGs. The dispatch function simply resolves the `icon_theme` string to an `IconSet`, calls the appropriate platform loader, and falls through the chain. The gpui connector's Lucide shortcut maps 28 of the 42 `IconRole` variants directly to `gpui_component::IconName` variants (the exact icons that gpui-component ships SVG assets for), allowing those roles to be rendered via gpui's built-in icon system without any I/O or pixel conversion.

**Primary recommendation:** Implement `load_icon()` as a thin dispatcher in the core crate, `rasterize_svg()` behind a feature gate, and connector conversion functions that handle both `IconData::Svg` and `IconData::Rgba` variants.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| INTG-01 | `load_icon()` dispatch function selecting the appropriate loader based on icon_theme string | Existing loader signatures are uniform (`IconRole -> Option<IconData>`), `IconSet::from_name()` parses theme strings, `system_icon_set()` resolves "system" to platform default |
| INTG-02 | Optional SVG-to-RGBA rasterization via resvg (feature "svg-rasterize") | resvg 0.47.0 provides `render(tree, transform, pixmap)`, depends on `usvg` + `tiny_skia`, outputs premultiplied RGBA pixels via `Pixmap` |
| INTG-03 | gpui connector: IconData->RenderImage conversion + `icon_name()` Lucide shortcut for gpui-component IconName | gpui 0.2.2 `Image::from_bytes(ImageFormat::Svg, bytes)` for SVG data; 28 of 42 IconRole variants map to IconName variants; `ImageSource::Image(Arc<Image>)` for rendering |
| INTG-04 | iced connector: IconData conversion helpers | `iced_core::image::Handle::from_rgba(width, height, pixels)` for RGBA, `Handle::from_bytes(bytes)` for SVG (iced decodes internally) |
| INTG-05 | gpui example updated with icon display and icon set selector dropdown | Existing showcase.rs has TAB_ICONS (index 6) showing all IconName variants; needs icon set dropdown selector and IconData display |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| resvg | 0.47.0 | SVG-to-RGBA rasterization | Only production-quality pure-Rust SVG renderer; re-exports usvg + tiny_skia |
| usvg | 0.47.0 | SVG parsing into render tree | Re-exported by resvg; separates parsing from rendering |
| tiny_skia | 0.12.0 | Pixel buffer (Pixmap) for rasterized output | Re-exported by resvg; provides Pixmap with RGBA pixels |

### Supporting (already in project)
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| gpui | 0.2.2 | GPU-accelerated UI framework | gpui connector image conversion |
| gpui-component | 0.5.1 | UI component library with IconName | Lucide shortcut mapping |
| iced_core | 0.14.0 | Iced widget library core | iced connector Handle::from_rgba |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| resvg | svg2png CLI | External process, not embeddable, no library API |
| resvg | librsvg (cairo-based) | C dependency, not pure Rust, system library required |

**Installation (core crate, feature-gated):**
```toml
[dependencies]
resvg = { version = "0.47", optional = true, default-features = false }

[features]
svg-rasterize = ["dep:resvg"]
```

## Architecture Patterns

### Recommended Project Structure
```
native-theme/src/
  lib.rs                # Add load_icon() + re-export
  model/
    icons.rs            # Existing IconRole, IconData, IconSet, icon_name()
    bundled.rs           # Existing bundled_icon_svg()
  freedesktop.rs         # Existing (Linux loader)
  sficons.rs             # Existing (macOS loader)
  winicons.rs            # Existing (Windows loader)
  rasterize.rs           # NEW: svg-rasterize feature module

connectors/native-theme-gpui/src/
  lib.rs                 # Add icon conversion functions
  icons.rs               # NEW: IconData->Image, icon_name() Lucide shortcut

connectors/native-theme-iced/src/
  lib.rs                 # Add icon conversion functions
  icons.rs               # NEW: IconData->Handle conversion

connectors/native-theme-gpui/examples/
  showcase.rs            # Update TAB_ICONS with icon set selector
```

### Pattern 1: Dispatch Function with Fallback Chain
**What:** `load_icon()` resolves icon_theme string to an icon set, dispatches to the appropriate platform loader, falls back to bundled icons.
**When to use:** Every time a consumer wants to load an icon by role.
**Example:**
```rust
/// Load an icon for the given role using the specified icon theme.
///
/// Dispatches to the platform-appropriate loader for system icon sets,
/// or directly to bundled SVGs for Material/Lucide.
///
/// # Fallback chain
/// 1. Platform loader (freedesktop/sf-symbols/segoe-fluent)
/// 2. Bundled Material SVGs (when material-icons feature enabled)
/// 3. None
pub fn load_icon(role: IconRole, icon_theme: &str) -> Option<IconData> {
    let set = IconSet::from_name(icon_theme)
        .unwrap_or_else(|| system_icon_set());

    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => crate::freedesktop::load_freedesktop_icon(role),

        #[cfg(all(target_os = "macos", feature = "system-icons"))]
        IconSet::SfSymbols => crate::sficons::load_sf_icon(role),

        #[cfg(all(target_os = "windows", feature = "system-icons"))]
        IconSet::SegoeIcons => crate::winicons::load_windows_icon(role),

        #[cfg(feature = "material-icons")]
        IconSet::Material => bundled_icon_svg(IconSet::Material, role)
            .map(|b| IconData::Svg(b.to_vec())),

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => bundled_icon_svg(IconSet::Lucide, role)
            .map(|b| IconData::Svg(b.to_vec())),

        // Non-matching platform or unknown set: try bundled fallback
        _ => {
            #[cfg(feature = "material-icons")]
            { return bundled_icon_svg(IconSet::Material, role)
                .map(|b| IconData::Svg(b.to_vec())); }
            #[cfg(not(feature = "material-icons"))]
            { None }
        }
    }
}
```

### Pattern 2: Feature-Gated SVG Rasterization
**What:** `rasterize_svg()` converts SVG bytes to `IconData::Rgba` using resvg.
**When to use:** When consumer needs pixel data from an SVG icon (e.g., for platforms or toolkits that only accept raster images).
**Example:**
```rust
// In src/rasterize.rs, behind #[cfg(feature = "svg-rasterize")]
use crate::IconData;

/// Rasterize SVG bytes to RGBA pixel data at the given size.
///
/// Returns `None` if the SVG cannot be parsed or rendered.
pub fn rasterize_svg(svg_bytes: &[u8], size: u32) -> Option<IconData> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_data(svg_bytes, &options).ok()?;

    let original_size = tree.size();
    let scale_x = size as f32 / original_size.width();
    let scale_y = size as f32 / original_size.height();
    let scale = scale_x.min(scale_y);

    let mut pixmap = tiny_skia::Pixmap::new(size, size)?;
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // resvg outputs premultiplied RGBA; convert to straight alpha
    let mut data = pixmap.take();
    unpremultiply_alpha(&mut data);

    Some(IconData::Rgba {
        width: size,
        height: size,
        data,
    })
}

fn unpremultiply_alpha(buffer: &mut [u8]) {
    for pixel in buffer.chunks_exact_mut(4) {
        let a = pixel[3] as u16;
        if a > 0 && a < 255 {
            pixel[0] = ((pixel[0] as u16 * 255) / a).min(255) as u8;
            pixel[1] = ((pixel[1] as u16 * 255) / a).min(255) as u8;
            pixel[2] = ((pixel[2] as u16 * 255) / a).min(255) as u8;
        }
    }
}
```

### Pattern 3: gpui IconName Shortcut Mapping
**What:** Map `IconRole` to `gpui_component::IconName` for Lucide icons where a direct match exists. This provides a zero-I/O shortcut: the gpui asset system already has these SVGs bundled via gpui-component-assets.
**When to use:** When the consumer uses gpui-component and wants to display Lucide icons matching semantic roles.
**Example:**
```rust
// In connectors/native-theme-gpui/src/icons.rs
use gpui_component::IconName;
use native_theme::IconRole;

/// Map an IconRole to a gpui-component IconName for the Lucide icon set.
///
/// Returns Some(IconName) for the 28 roles that have a direct Lucide
/// equivalent in gpui-component's icon set. Returns None for roles
/// where gpui-component doesn't ship the corresponding Lucide icon
/// (e.g., Shield, Lock, Save, Paste, Cut, Edit, Refresh, Print, Home, Help).
pub fn icon_name(role: IconRole) -> Option<IconName> {
    Some(match role {
        IconRole::DialogWarning => IconName::TriangleAlert,
        IconRole::DialogError => IconName::CircleX,
        IconRole::DialogInfo => IconName::Info,
        IconRole::DialogSuccess => IconName::CircleCheck,
        IconRole::WindowClose => IconName::WindowClose,
        IconRole::WindowMinimize => IconName::WindowMinimize,
        IconRole::WindowMaximize => IconName::WindowMaximize,
        IconRole::WindowRestore => IconName::WindowRestore,
        IconRole::ActionDelete => IconName::Delete,
        IconRole::ActionCopy => IconName::Copy,
        IconRole::ActionUndo => IconName::Undo2,
        IconRole::ActionRedo => IconName::Redo2,
        IconRole::ActionSearch => IconName::Search,
        IconRole::ActionSettings => IconName::Settings,
        IconRole::ActionAdd => IconName::Plus,
        IconRole::ActionRemove => IconName::Minus,
        IconRole::NavBack => IconName::ChevronLeft,
        IconRole::NavForward => IconName::ChevronRight,
        IconRole::NavUp => IconName::ChevronUp,
        IconRole::NavDown => IconName::ChevronDown,
        IconRole::NavMenu => IconName::Menu,
        IconRole::FileGeneric => IconName::File,
        IconRole::FolderClosed => IconName::FolderClosed,
        IconRole::FolderOpen => IconName::FolderOpen,
        IconRole::TrashEmpty => IconName::Delete,
        IconRole::StatusLoading => IconName::Loader,
        IconRole::StatusCheck => IconName::Check,
        IconRole::StatusError => IconName::CircleX,
        IconRole::UserAccount => IconName::User,  // CircleUser also available
        IconRole::Notification => IconName::Bell,
        _ => return None,
    })
}
```

### Pattern 4: gpui IconData to Image Conversion
**What:** Convert `IconData` to a gpui-compatible `ImageSource` for rendering via `img()`.
**When to use:** When displaying platform-loaded or bundled icons in a gpui app.
**Example:**
```rust
use gpui::{Image, ImageFormat, ImageSource};
use native_theme::IconData;
use std::sync::Arc;

/// Convert IconData to a gpui ImageSource for rendering.
///
/// - `IconData::Svg`: wraps bytes in `Image::from_bytes(ImageFormat::Svg, bytes)`
/// - `IconData::Rgba`: encodes as PNG first (gpui requires encoded formats)
pub fn to_image_source(data: &IconData) -> ImageSource {
    match data {
        IconData::Svg(bytes) => {
            let image = Image::from_bytes(ImageFormat::Svg, bytes.clone());
            ImageSource::Image(Arc::new(image))
        }
        IconData::Rgba { width, height, data } => {
            // gpui's Image type requires encoded bytes, not raw RGBA.
            // For RGBA data, encode as minimal BMP or use RenderImage.
            // Simplest approach: encode via a tiny PNG encoder or
            // use gpui's internal rendering pipeline.
            // This is a design decision -- see Open Questions.
            todo!("RGBA to gpui image encoding strategy")
        }
    }
}
```

### Pattern 5: iced IconData to Handle Conversion
**What:** Convert `IconData` to `iced_core::image::Handle`.
**When to use:** When displaying icons in an iced app.
**Example:**
```rust
use iced_core::image::Handle;
use native_theme::IconData;

/// Convert IconData to an iced image Handle.
///
/// - `IconData::Svg`: not directly supported by Handle (iced has a
///   separate `Svg` widget). Use `Handle::from_bytes` only if iced
///   can decode the SVG internally, otherwise the consumer should
///   rasterize first via `rasterize_svg()`.
/// - `IconData::Rgba`: uses `Handle::from_rgba(width, height, pixels)`
pub fn to_image_handle(data: &IconData) -> Option<Handle> {
    match data {
        IconData::Rgba { width, height, data } => {
            Some(Handle::from_rgba(*width, *height, data.clone()))
        }
        IconData::Svg(_) => {
            // SVG icons need the iced Svg widget, not Image.
            // Return None and let the consumer use iced::widget::Svg instead.
            None
        }
    }
}
```

### Anti-Patterns to Avoid
- **Caching in the core crate:** Prior decision (Out of Scope in REQUIREMENTS.md) says "consumers cache at toolkit level; core crate is stateless." Do not add icon caching to `load_icon()`.
- **Encoding RGBA as PNG in the hot path:** If gpui needs encoded images, do the encoding once at load time, not on every render call.
- **Using `image` crate for PNG encoding:** The `image` crate is heavy; for simple RGBA-to-PNG encoding, use a lighter alternative like `png` crate or avoid the encoding entirely.
- **Blocking on resvg in UI thread:** SVG parsing + rasterization can take milliseconds. For real-time UIs, load icons on startup or in a background task.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| SVG rendering | Custom SVG parser | resvg + usvg + tiny_skia | SVG spec is enormously complex; resvg handles gradients, filters, text, clipping |
| Premultiplied alpha conversion | Skip it | `unpremultiply_alpha()` pass | resvg outputs premultiplied; rendering artifacts if not converted |
| Icon name mapping | String-based lookup | Compile-time match expressions | Zero-cost, type-safe, no hash table overhead |
| PNG encoding from RGBA | Custom encoder | `png` crate or avoid entirely | PNG format has deflate compression, CRC checksums, filter rows |

**Key insight:** The dispatch function (`load_icon`) is intentionally thin. All the complexity lives in the platform loaders (already built) and the resvg rendering pipeline. The integration layer just connects the pieces.

## Common Pitfalls

### Pitfall 1: Premultiplied Alpha from resvg
**What goes wrong:** resvg/tiny_skia outputs premultiplied RGBA. If fed directly to a toolkit expecting straight alpha, icons appear too dark or have halos.
**Why it happens:** Compositing engines typically use premultiplied internally, so resvg keeps that format.
**How to avoid:** Apply `unpremultiply_alpha()` to the Pixmap data before returning `IconData::Rgba`. Both sficons.rs and winicons.rs already have this function.
**Warning signs:** Icons with semi-transparent edges appear darker than expected.

### Pitfall 2: gpui Expects Encoded Images, Not Raw RGBA
**What goes wrong:** gpui's `Image::from_bytes()` expects encoded formats (PNG, SVG, etc.), not raw RGBA pixel arrays. Passing raw RGBA bytes with `ImageFormat::Png` causes decoding errors.
**Why it happens:** gpui's image pipeline decodes images through the `image` crate internally.
**How to avoid:** For `IconData::Svg`, use `Image::from_bytes(ImageFormat::Svg, bytes)` directly. For `IconData::Rgba`, either (a) encode as PNG first, or (b) use `RenderImage::new()` with a `Frame` if gpui exposes that API publicly.
**Warning signs:** Runtime panics or blank images when displaying RGBA icon data.

### Pitfall 3: cfg() Gate Mismatches in Dispatch
**What goes wrong:** `load_icon()` compiles on all platforms but calls platform-specific loaders behind `cfg()`. If the gates don't match exactly, some match arms become unreachable or the function returns None when it shouldn't.
**Why it happens:** Three dimensions of gating: target_os, feature flags (system-icons, material-icons), and icon set enum variants.
**How to avoid:** Use `#[allow(unreachable_patterns)]` wildcard arm as established in icons.rs. Test on each platform or review cfg combinations manually.
**Warning signs:** Clippy warnings about unreachable patterns; icons returning None on one platform but not others.

### Pitfall 4: iced SVG vs Image Widget Mismatch
**What goes wrong:** `iced_core::image::Handle` is for raster images. SVG icons need `iced::widget::Svg`, not `iced::widget::Image`. Trying to use `Handle::from_bytes()` with SVG data may fail or produce nothing.
**Why it happens:** iced has separate widget types for SVG and raster images.
**How to avoid:** The connector should provide separate helpers: `to_image_handle()` for `IconData::Rgba`, and `to_svg_handle()` or similar for `IconData::Svg`. Or provide a unified enum that wraps both.
**Warning signs:** SVG icons render as blank in iced.

### Pitfall 5: Icon Set Selector UI Complexity
**What goes wrong:** The gpui example icon set selector needs to trigger icon re-loading when the user changes the set. If `load_icon()` is called synchronously in the render path, it causes frame drops.
**Why it happens:** Freedesktop icon lookup does filesystem I/O; macOS/Windows loaders call system APIs.
**How to avoid:** Pre-load all 42 icons for the selected set on selection change (not per-frame). Store the loaded icons in the view state.
**Warning signs:** Frame rate drops when scrolling through the icons tab after switching icon sets.

### Pitfall 6: Unknown Icon Theme Strings
**What goes wrong:** `IconSet::from_name()` returns `None` for unrecognized theme strings (e.g., typos like "materail" or custom names). If `load_icon()` doesn't handle this gracefully, it returns None for everything.
**Why it happens:** The icon_theme field is `Option<String>`, any arbitrary string can appear.
**How to avoid:** When `IconSet::from_name()` returns None, fall back to `system_icon_set()` (the platform default). Document this behavior.
**Warning signs:** Icons disappear entirely when using custom or misspelled theme names.

## Code Examples

Verified patterns from the existing codebase:

### Platform Loader Signature (established pattern)
```rust
// Source: native-theme/src/freedesktop.rs, sficons.rs, winicons.rs
// All three loaders follow this exact signature:
pub fn load_freedesktop_icon(role: IconRole) -> Option<IconData> { ... }
pub fn load_sf_icon(role: IconRole) -> Option<IconData> { ... }
pub fn load_windows_icon(role: IconRole) -> Option<IconData> { ... }
```

### IconSet Resolution (existing code)
```rust
// Source: native-theme/src/model/icons.rs
pub fn system_icon_set() -> IconSet {
    if cfg!(any(target_os = "macos", target_os = "ios")) {
        IconSet::SfSymbols
    } else if cfg!(target_os = "windows") {
        IconSet::SegoeIcons
    } else if cfg!(target_os = "linux") {
        IconSet::Freedesktop
    } else {
        IconSet::Material
    }
}

impl IconSet {
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
}
```

### gpui Image from SVG Bytes
```rust
// Source: gpui docs.rs - ImageFormat enum + Image struct
use gpui::{Image, ImageFormat, ImageSource};
use std::sync::Arc;

let svg_bytes: Vec<u8> = /* from IconData::Svg */;
let image = Image::from_bytes(ImageFormat::Svg, svg_bytes);
let source = ImageSource::Image(Arc::new(image));
// Use with: img().source(source)
```

### iced Handle from RGBA Pixels
```rust
// Source: iced_core 0.14.0 docs - Handle::from_rgba
use iced_core::image::Handle;

let handle = Handle::from_rgba(width, height, pixels);
// Use with: iced::widget::Image::new(handle)
```

### resvg SVG Rendering Pipeline
```rust
// Source: resvg 0.47.0 docs.rs + GitHub
use resvg;

let options = usvg::Options::default();
let tree = usvg::Tree::from_data(svg_bytes, &options).ok()?;
let size = tree.size();
let mut pixmap = tiny_skia::Pixmap::new(width, height)?;
let transform = tiny_skia::Transform::from_scale(
    width as f32 / size.width(),
    height as f32 / size.height(),
);
resvg::render(&tree, transform, &mut pixmap.as_mut());
let rgba_data = pixmap.take(); // premultiplied RGBA
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| resvg with cairo backend | resvg with tiny-skia (pure Rust) | resvg 0.20+ (2022) | No C dependencies, cross-platform reproducible rendering |
| gpui Image with raw pixels | gpui Image with encoded formats only | gpui 0.2+ | Must provide SVG/PNG bytes, not raw RGBA |
| iced Image + SVG in one widget | iced separates Image and Svg widgets | iced 0.12+ | IconData::Svg needs Svg widget, IconData::Rgba needs Image widget |

**Deprecated/outdated:**
- resvg < 0.20: Used cairo for rendering, required system dependencies
- usvg standalone usage without re-export: resvg now re-exports `usvg` and `tiny_skia`

## Open Questions

1. **RGBA to gpui Image encoding strategy**
   - What we know: gpui's `Image::from_bytes()` accepts encoded formats (PNG, SVG, etc.) but not raw RGBA. `RenderImage` works with `Frame` objects but the `Frame` type is not publicly documented.
   - What's unclear: Whether gpui 0.2.2 exposes a way to create images from raw RGBA without PNG encoding. The `RenderImage::new()` constructor takes `SmallVec<[Frame; 1]>` but Frame's API is undocumented.
   - Recommendation: For `IconData::Svg`, use `Image::from_bytes(ImageFormat::Svg, bytes)` directly (this is the common case for bundled icons). For `IconData::Rgba` from platform loaders (macOS/Windows), either (a) use the `png` crate to encode and wrap in `Image::from_bytes(ImageFormat::Png, encoded)`, or (b) investigate `RenderImage` Frame API at implementation time. Option (a) is safer and guaranteed to work.

2. **iced SVG icon handling strategy**
   - What we know: `iced_core::image::Handle` supports RGBA via `from_rgba()`. But SVG icons need `iced::widget::Svg`, which uses a separate `svg::Handle` type.
   - What's unclear: Whether the iced connector should provide one unified function or two separate helpers.
   - Recommendation: Provide `to_image_handle()` for RGBA icons and `to_svg_handle()` returning `iced_core::svg::Handle` for SVG icons. Also provide a convenience `to_element()` that returns either `Image` or `Svg` widget. The connector depends on `iced_core` (not full `iced`), so exposing the widget wrappers may need the `iced_core::svg` module.

3. **Showcase icon set selector: which sets to include**
   - What we know: There are 5 icon sets (sf-symbols, segoe-fluent, freedesktop, material, lucide). Platform sets only work on their respective OS.
   - What's unclear: Should the dropdown show all 5 and gray out unavailable ones, or only show available ones?
   - Recommendation: Show all sets in the dropdown. For unavailable platform sets, load_icon() falls back to bundled Material SVGs, so icons will still display (just not native ones). Label unavailable sets with "(fallback)" suffix.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in) |
| Config file | Cargo.toml [dev-dependencies] |
| Quick run command | `cargo test -p native-theme --features material-icons,lucide-icons` |
| Full suite command | `cargo test --workspace --features native-theme/material-icons,native-theme/lucide-icons` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| INTG-01 | load_icon dispatches correctly | unit | `cargo test -p native-theme load_icon --features material-icons` | Wave 0 |
| INTG-01 | Unknown theme string falls back | unit | `cargo test -p native-theme load_icon_unknown --features material-icons` | Wave 0 |
| INTG-02 | rasterize_svg produces RGBA | unit | `cargo test -p native-theme rasterize_svg --features svg-rasterize,material-icons` | Wave 0 |
| INTG-02 | rasterize_svg handles invalid SVG | unit | `cargo test -p native-theme rasterize_invalid --features svg-rasterize` | Wave 0 |
| INTG-03 | icon_name maps 28+ roles | unit | `cargo test -p native-theme-gpui icon_name` | Wave 0 |
| INTG-03 | IconData::Svg converts to gpui Image | unit | `cargo test -p native-theme-gpui to_image` | Wave 0 |
| INTG-04 | IconData::Rgba converts to iced Handle | unit | `cargo test -p native-theme-iced to_image_handle` | Wave 0 |
| INTG-05 | Showcase example compiles | smoke | `cargo build -p native-theme-gpui --example showcase` | Existing |

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme --features material-icons,lucide-icons && cargo test -p native-theme-iced`
- **Per wave merge:** `cargo test --workspace --features native-theme/material-icons,native-theme/lucide-icons`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `native-theme/src/rasterize.rs` -- new module for INTG-02
- [ ] Tests for `load_icon()` dispatch in `lib.rs` -- covers INTG-01
- [ ] `connectors/native-theme-gpui/src/icons.rs` -- covers INTG-03
- [ ] `connectors/native-theme-iced/src/icons.rs` -- covers INTG-04

## Sources

### Primary (HIGH confidence)
- gpui 0.2.2 docs.rs -- Image, ImageFormat, ImageSource, RenderImage types
- gpui-component 0.5.1 docs.rs -- IconName enum (86 variants)
- iced_core 0.14.0 docs -- Handle::from_rgba(width, height, pixels)
- resvg 0.47.0 docs.rs -- render(tree, transform, pixmap) API
- Existing codebase -- freedesktop.rs, sficons.rs, winicons.rs loader signatures

### Secondary (MEDIUM confidence)
- [resvg GitHub](https://github.com/linebender/resvg) -- v0.47.0 latest, pure-Rust rendering
- [iced Handle docs](https://docs.iced.rs/iced/advanced/image/enum.Handle.html) -- from_rgba, from_bytes constructors
- [gpui-component assets docs](https://longbridge.github.io/gpui-component/docs/assets) -- IconName maps to Lucide SVG files

### Tertiary (LOW confidence)
- gpui Frame type -- undocumented publicly, may need source inspection at implementation time

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - resvg is the de facto pure-Rust SVG renderer, well-documented
- Architecture: HIGH - all building blocks exist in the codebase, dispatch is straightforward
- Pitfalls: HIGH - based on actual codebase patterns and verified API constraints
- gpui RGBA conversion: MEDIUM - Frame type undocumented, PNG encoding fallback is reliable but adds dependency

**Research date:** 2026-03-09
**Valid until:** 2026-04-09 (30 days -- stable libraries, no fast-moving APIs)
