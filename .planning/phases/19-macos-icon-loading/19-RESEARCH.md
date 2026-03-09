# Phase 19: macOS Icon Loading - Research

**Researched:** 2026-03-09
**Domain:** macOS AppKit (NSImage, SF Symbols), Core Graphics rasterization, objc2 Rust bindings
**Confidence:** HIGH

## Summary

Phase 19 implements macOS SF Symbols icon loading -- resolving `IconRole` variants to SF Symbols names via the existing `icon_name(IconSet::SfSymbols, role)` mapping, creating an `NSImage` from the system symbol, rasterizing it to RGBA pixels via Core Graphics, and returning `IconData::Rgba`. The fallback chain mirrors Phase 18's pattern: try SF Symbols first, then fall back to bundled Material SVGs.

The rasterization approach uses `NSImage::imageWithSystemSymbolName_accessibilityDescription` (macOS 11+) to obtain the symbol image, `NSImageSymbolConfiguration::configurationWithPointSize_weight_scale` to configure the desired pixel size, `NSImage::CGImageForProposedRect_context_hints` to extract a CGImage, then `CGBitmapContextCreate` + `CGContext::draw_image` to rasterize into a caller-owned RGBA buffer with known pixel format. This CGBitmapContext approach guarantees consistent RGBA output regardless of the internal image format.

The critical alpha handling issue: `CGBitmapContextCreate` only supports premultiplied alpha. The success criteria require straight (non-premultiplied) alpha. The module must perform a post-processing unpremultiply pass: for each pixel where `a > 0`, compute `r = min(255, r * 255 / a)`, `g = min(255, g * 255 / a)`, `b = min(255, b * 255 / a)`. This is a well-known conversion and is straightforward to implement.

**Primary recommendation:** Create `sficons.rs` module (cfg-gated to `target_os = "macos"` + `system-icons` feature) following the exact same pattern as `freedesktop.rs`. Use the objc2 ecosystem crates already in the project (`objc2`, `objc2-app-kit`, `objc2-foundation`) plus `objc2-core-graphics` (new dependency) for CGBitmapContext rasterization. Add `objc2-core-graphics` as a macOS-gated optional dependency activated by `system-icons`.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PLAT-01 | macOS icon loading via `NSImage(systemSymbolName:)` -> rasterized RGBA pixels (feature "system-icons") | `NSImage::imageWithSystemSymbolName_accessibilityDescription` creates symbol image (macOS 11+); `CGImageForProposedRect` extracts CGImage; CGBitmapContext rasterizes to RGBA buffer; unpremultiply pass converts to straight alpha; 38 of 42 roles have SF Symbols names, 4 fall back to bundled Material SVGs |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| objc2 | 0.6 | Objective-C runtime bindings | Already in project, required for AppKit interop |
| objc2-app-kit | 0.3 | NSImage, NSImageSymbolConfiguration, NSBitmapImageRep | Already in project for macos feature; needs additional feature flags |
| objc2-foundation | 0.3 | NSString for symbol names | Already in project |
| objc2-core-graphics | 0.3 | CGBitmapContext, CGImage, CGColorSpace for rasterization | New dependency; same objc2 ecosystem, needed for pixel buffer extraction |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| objc2-core-foundation | (transitive) | CFData for CGDataProvider data access | Already pulled in by objc2-app-kit features |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| CGBitmapContext rasterization | NSBitmapImageRep.bitmapData | NSBitmapImageRep format varies (could be ARGB, float, planar); CGBitmapContext guarantees exact format |
| CGBitmapContext rasterization | CGImage.dataProvider.data | Raw data format depends on how CGImage was created; no format normalization |
| objc2-core-graphics | core-graphics crate (servo) | Different ecosystem from objc2; would introduce dependency conflict with existing objc2 usage |

**Dependency additions to Cargo.toml:**
```toml
# Under [target.'cfg(target_os = "macos")'.dependencies], add:
objc2-core-graphics = { version = "0.3", optional = true, features = [
    "CGBitmapContext", "CGColorSpace", "CGContext", "CGImage",
] }

# Under [features], update system-icons:
system-icons = [
    "dep:freedesktop-icons",
    "dep:objc2-core-graphics",
    "material-icons",
]

# Under objc2-app-kit features list, add:
# "NSImage", "NSImageRep", "NSGraphicsContext"
```

Note: The `system-icons` feature gains `dep:objc2-core-graphics` but `objc2-core-graphics` is gated with `cfg(target_os = "macos")` in the dependency section, so it only activates on macOS. The `freedesktop-icons` dep is similarly gated to Linux. Cargo handles this correctly -- optional deps under target-specific sections only activate on matching targets.

## Architecture Patterns

### Recommended Module Structure
```
native-theme/src/
  sficons.rs             # New: macOS SF Symbols icon loader (cfg macos + system-icons)
  freedesktop.rs         # Existing: Linux freedesktop icon loader
  lib.rs                 # Add: pub mod sficons (conditional) + re-export
  model/
    icons.rs             # Existing: icon_name(IconSet::SfSymbols, role) already maps 38/42 roles
    bundled.rs           # Existing: bundled_icon_svg() for fallback
```

### Pattern 1: CGBitmapContext Rasterization Pipeline
**What:** Create a bitmap graphics context with known RGBA format, draw the CGImage into it, extract the raw pixel buffer.
**When to use:** Any time you need guaranteed RGBA pixel data from an NSImage/CGImage.
**Example:**
```rust
// Source: Apple CoreGraphics docs + objc2-core-graphics 0.3 API
use objc2_core_graphics::{
    CGBitmapContextCreate, CGBitmapContextGetData,
    CGColorSpace, CGContext, CGImage, CGImageAlphaInfo, CGRect, CGPoint, CGSize,
};
use std::ffi::c_void;
use std::ptr;

fn rasterize_cgimage(cg_image: &CGImage, width: u32, height: u32) -> Option<Vec<u8>> {
    let color_space = CGColorSpace::new_device_rgb()?;
    let bytes_per_row = (width as usize) * 4;
    let buf_size = bytes_per_row * (height as usize);

    // Allocate buffer for RGBA pixels
    let mut buffer: Vec<u8> = vec![0u8; buf_size];

    // kCGImageAlphaPremultipliedLast = RGBA with premultiplied alpha
    let bitmap_info = CGImageAlphaInfo::PremultipliedLast.0;

    let context = unsafe {
        CGBitmapContextCreate(
            buffer.as_mut_ptr() as *mut c_void,
            width as usize,
            height as usize,
            8,                    // bits per component
            bytes_per_row,
            Some(&color_space),
            bitmap_info,
        )
    }?;

    // Draw the image into our context -- fills the buffer
    let rect = CGRect {
        origin: CGPoint { x: 0.0, y: 0.0 },
        size: CGSize { width: width as f64, height: height as f64 },
    };
    CGContext::draw_image(Some(&context), rect, Some(cg_image));

    Some(buffer)
}
```

### Pattern 2: Premultiplied-to-Straight Alpha Conversion
**What:** Convert premultiplied RGBA pixels to straight alpha, as required by the success criteria.
**When to use:** After extracting pixels from CGBitmapContext (which only supports premultiplied alpha).
**Example:**
```rust
fn unpremultiply_alpha(buffer: &mut [u8]) {
    for pixel in buffer.chunks_exact_mut(4) {
        let a = pixel[3] as u16;
        if a > 0 && a < 255 {
            pixel[0] = ((pixel[0] as u16 * 255) / a).min(255) as u8;
            pixel[1] = ((pixel[1] as u16 * 255) / a).min(255) as u8;
            pixel[2] = ((pixel[2] as u16 * 255) / a).min(255) as u8;
        }
        // If a == 0, RGB should be 0 (already true for premultiplied)
        // If a == 255, no conversion needed
    }
}
```

### Pattern 3: SF Symbol Configuration for Specific Pixel Size
**What:** Configure the symbol to render at a specific point size, which controls the resulting pixel dimensions.
**When to use:** When creating the NSImage for a specific icon size.
**Example:**
```rust
use objc2_app_kit::{NSImage, NSImageSymbolConfiguration, NSImageSymbolScale, NSFontWeight};
use objc2_foundation::NSString;

fn load_sf_symbol(name: &str, point_size: f64) -> Option<Retained<NSImage>> {
    let ns_name = NSString::from_str(name);

    // Create the base symbol image
    let image = NSImage::imageWithSystemSymbolName_accessibilityDescription(
        &ns_name,
        None,
    )?;

    // Configure for specific point size with regular weight at medium scale
    let config = NSImageSymbolConfiguration::configurationWithPointSize_weight_scale(
        point_size,
        NSFontWeight::Regular,
        NSImageSymbolScale::Medium,
    );

    // Apply configuration (returns a new configured image)
    image.imageWithSymbolConfiguration(&config)
}
```

### Pattern 4: NSImage to CGImage Extraction
**What:** Extract a CGImage from an NSImage at a specific pixel size.
**When to use:** Before rasterization.
**Example:**
```rust
use objc2_app_kit::NSImage;
use objc2_core_graphics::CGImage;
use objc2_foundation::NSRect;
use std::ptr;

fn extract_cgimage(image: &NSImage) -> Option<Retained<CGImage>> {
    // Pass null rect to use the image's natural size
    unsafe {
        image.CGImageForProposedRect_context_hints(
            ptr::null_mut(),   // use natural size
            None,              // no specific graphics context
            None,              // no hints
        )
    }
}
```

### Anti-Patterns to Avoid
- **Using lockFocus/unlockFocus:** These are deprecated since macOS 10.14. Use CGBitmapContext instead.
- **Assuming CGImage pixel format:** CGImages from `CGImageForProposedRect` may have various internal formats (BGRA, ARGB, premultiplied, etc.). Always normalize through CGBitmapContext.
- **Skipping unpremultiply:** CGBitmapContext only supports premultiplied alpha. The project's `IconData::Rgba` contract specifies straight alpha. Consumers (gpui, iced) expect straight alpha.
- **Hardcoding pixel dimensions:** SF Symbols are vector-based and render at the configured point size. The actual pixel dimensions come from the configured `NSImageSymbolConfiguration`, not a hardcoded constant.
- **Forgetting Retina scale:** On Retina displays, a 24pt symbol produces a 48px image. The function should accept a `size` parameter in pixels and configure the point size accordingly (or let the caller specify scale).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| SF Symbol loading | Manual font glyph rendering | `NSImage::imageWithSystemSymbolName_accessibilityDescription` | Apple's API handles symbol versioning, fallback, multicolor |
| Image format normalization | Manual pixel format detection + conversion | CGBitmapContext with known format | CGBitmapContext handles all source formats correctly |
| Symbol size configuration | Manual point-to-pixel math | `NSImageSymbolConfiguration::configurationWithPointSize_weight_scale` | Handles scale factors, baseline alignment, weight matching |
| sRGB color space | Manual ICC profile loading | `CGColorSpace::new_device_rgb()` | Device RGB is correct for screen rendering of monochrome symbols |

**Key insight:** The CGBitmapContext approach may seem like extra work compared to reading CGImage data directly, but it is the ONLY approach that guarantees a consistent pixel format. CGImages from `CGImageForProposedRect` can have varying internal formats depending on the image source, display configuration, and macOS version.

## Common Pitfalls

### Pitfall 1: Premultiplied Alpha Not Converted
**What goes wrong:** Icons appear correct at full opacity but semi-transparent edges are darker than expected or show color fringing.
**Why it happens:** CGBitmapContext only outputs premultiplied alpha. If passed directly as `IconData::Rgba`, consumers that expect straight alpha will composite incorrectly.
**How to avoid:** Always run the unpremultiply pass on the buffer before returning `IconData::Rgba`. The conversion is `r = r * 255 / a` for each channel where `a > 0`.
**Warning signs:** Semi-transparent pixels around icon edges appear darker or tinted compared to the bundled SVG fallback rendered by resvg.

### Pitfall 2: SF Symbols Unavailable on Older macOS
**What goes wrong:** `NSImage::imageWithSystemSymbolName_accessibilityDescription` returns `None` on macOS 10.15 and earlier, or returns `None` for symbols introduced in newer SF Symbols versions.
**Why it happens:** SF Symbols via NSImage requires macOS 11.0+. Individual symbols may require newer versions (e.g., some symbols added in SF Symbols 3.0 require macOS 13.0).
**How to avoid:** The function already handles `None` by falling through to the bundled Material SVG fallback. The fallback chain design from Phase 18 handles this naturally.
**Warning signs:** None -- this is expected behavior. The fallback chain handles it.

### Pitfall 3: Feature Flag Dependency Graph
**What goes wrong:** Adding `objc2-core-graphics` to the `system-icons` feature without proper platform gating causes compilation failures on Linux (tries to link CoreGraphics).
**Why it happens:** The `system-icons` feature activates BOTH `dep:freedesktop-icons` (Linux-only) and `dep:objc2-core-graphics` (macOS-only). Both must be under platform-gated `[target.'cfg(...)'.dependencies]` sections.
**How to avoid:** Place `objc2-core-graphics` under `[target.'cfg(target_os = "macos")'.dependencies]` with `optional = true`. Cargo resolves optional deps under target-gated sections correctly -- they only activate on the matching platform even when the feature is enabled globally.
**Warning signs:** CI failures on Linux or Windows when `--features system-icons` is used.

### Pitfall 4: Icon Dimensions Mismatch
**What goes wrong:** The returned `IconData::Rgba { width, height, data }` has `width * height * 4 != data.len()`, causing panics in consumers.
**Why it happens:** The CGImage from `CGImageForProposedRect` may have different dimensions than the NSImage's logical size, especially on Retina displays where pixel dimensions are 2x the point dimensions.
**How to avoid:** Read `width` and `height` from the CGImage (via `CGImage::width()` and `CGImage::height()`), not from the NSImage's `size()` property. Use those exact dimensions for the CGBitmapContext AND the returned `IconData::Rgba`.
**Warning signs:** `data.len()` assertion failures, or icons appearing at half or double the expected size.

### Pitfall 5: objc2-app-kit Feature Flags Missing
**What goes wrong:** Compilation errors like "method not found" for `CGImageForProposedRect_context_hints`, `imageWithSymbolConfiguration`, etc.
**Why it happens:** objc2-app-kit gates methods behind cargo features. `CGImageForProposedRect_context_hints` requires `NSGraphicsContext`, `NSImageRep`, and `objc2-core-graphics` features. `configurationWithPointSize_weight_scale` requires `NSFontDescriptor` and `objc2-core-foundation`.
**How to avoid:** Add all required features to the objc2-app-kit dependency: `"NSImage"`, `"NSImageRep"`, `"NSGraphicsContext"`, `"NSFontDescriptor"`. Since these are only needed when `system-icons` is active, they should be added to the main objc2-app-kit dependency (which is already macOS-gated).
**Warning signs:** Cryptic "method not found" errors from objc2-app-kit.

## Code Examples

Verified patterns from official docs and existing codebase:

### Complete macOS SF Symbols Loader Module Structure
```rust
// native-theme/src/sficons.rs
// macOS SF Symbols icon loader
//
// Resolves IconRole variants to RGBA pixel data by loading SF Symbols
// via NSImage and rasterizing through CGBitmapContext, with fallback
// to bundled Material SVGs.

use crate::{bundled_icon_svg, icon_name, IconData, IconRole, IconSet};
use objc2::rc::Retained;
use objc2_app_kit::{
    NSFontWeight, NSImage, NSImageSymbolConfiguration, NSImageSymbolScale,
};
use objc2_core_graphics::{
    CGBitmapContextCreate, CGColorSpace, CGContext, CGImage,
    CGImageAlphaInfo, CGPoint, CGRect, CGSize,
};
use objc2_foundation::NSString;
use std::ffi::c_void;
use std::ptr;

/// Default icon size in pixels (suitable for toolbar/menu icons).
const DEFAULT_ICON_SIZE: u32 = 24;

/// Load an SF Symbol image by name with the given point size.
fn load_symbol(name: &str, point_size: f64) -> Option<Retained<NSImage>> {
    let ns_name = NSString::from_str(name);
    let image = NSImage::imageWithSystemSymbolName_accessibilityDescription(
        &ns_name, None,
    )?;
    let config = NSImageSymbolConfiguration::configurationWithPointSize_weight_scale(
        point_size,
        NSFontWeight::Regular,
        NSImageSymbolScale::Medium,
    );
    image.imageWithSymbolConfiguration(&config)
}

/// Extract a CGImage from an NSImage.
fn extract_cgimage(image: &NSImage) -> Option<Retained<CGImage>> {
    unsafe {
        image.CGImageForProposedRect_context_hints(
            ptr::null_mut(), None, None,
        )
    }
}

/// Rasterize a CGImage to an RGBA pixel buffer.
fn rasterize(cg_image: &CGImage, width: u32, height: u32) -> Option<Vec<u8>> {
    let color_space = CGColorSpace::new_device_rgb()?;
    let bytes_per_row = (width as usize) * 4;
    let buf_size = bytes_per_row * (height as usize);
    let mut buffer = vec![0u8; buf_size];

    let context = unsafe {
        CGBitmapContextCreate(
            buffer.as_mut_ptr() as *mut c_void,
            width as usize,
            height as usize,
            8,
            bytes_per_row,
            Some(&color_space),
            CGImageAlphaInfo::PremultipliedLast.0,
        )
    }?;

    let rect = CGRect {
        origin: CGPoint { x: 0.0, y: 0.0 },
        size: CGSize { width: width as f64, height: height as f64 },
    };
    CGContext::draw_image(Some(&context), rect, Some(cg_image));

    Some(buffer)
}

/// Convert premultiplied RGBA to straight (non-premultiplied) alpha.
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

/// Load an SF Symbols icon for the given role as RGBA pixel data.
///
/// # Fallback chain
///
/// 1. SF Symbols via NSImage (macOS 11+, 38 of 42 roles have mappings)
/// 2. Bundled Material SVGs (requires `material-icons` feature, which
///    `system-icons` implies)
///
/// Returns `None` only if no icon is found at any level.
pub fn load_sf_icon(role: IconRole) -> Option<IconData> {
    if let Some(name) = icon_name(IconSet::SfSymbols, role) {
        let size = DEFAULT_ICON_SIZE;
        if let Some(image) = load_symbol(name, size as f64) {
            if let Some(cg_image) = extract_cgimage(&image) {
                let w = CGImage::width(&cg_image) as u32;
                let h = CGImage::height(&cg_image) as u32;
                if let Some(mut data) = rasterize(&cg_image, w, h) {
                    unpremultiply_alpha(&mut data);
                    return Some(IconData::Rgba {
                        width: w,
                        height: h,
                        data,
                    });
                }
            }
        }
    }

    // Bundled Material SVG fallback
    bundled_icon_svg(IconSet::Material, role).map(|bytes| IconData::Svg(bytes.to_vec()))
}
```

### Cargo.toml Changes
```toml
# Feature update (system-icons gains macOS deps):
[features]
system-icons = ["dep:freedesktop-icons", "dep:objc2-core-graphics", "material-icons"]

# New macOS-gated dependency:
[target.'cfg(target_os = "macos")'.dependencies]
# ... existing objc2, objc2-foundation, objc2-app-kit entries ...
objc2-core-graphics = { version = "0.3", optional = true, features = [
    "CGBitmapContext", "CGColorSpace", "CGContext", "CGImage",
] }

# Update objc2-app-kit to add needed features:
objc2-app-kit = { version = "0.3", optional = true, features = [
    "NSColor", "NSColorSpace", "NSAppearance", "NSFont", "NSFontDescriptor",
    "objc2-core-foundation",
    # New features needed for icon loading:
    "NSImage", "NSImageRep", "NSGraphicsContext",
    "objc2-core-graphics",  # enables CGImageForProposedRect
] }
```

### lib.rs Wiring
```rust
// In lib.rs, alongside the existing freedesktop module:
#[cfg(all(target_os = "macos", feature = "system-icons"))]
pub mod sficons;

#[cfg(all(target_os = "macos", feature = "system-icons"))]
pub use sficons::load_sf_icon;
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| No SF Symbols (pre-macOS 11) | `NSImage(systemSymbolName:)` | macOS 11 (2020) | Platform-native monochrome icons available |
| lockFocus/unlockFocus for rasterization | CGBitmapContext | macOS 10.14 deprecated lockFocus | Must use CGBitmapContext approach |
| core-graphics crate (servo) | objc2-core-graphics | 2024-2025 | objc2 ecosystem provides unified bindings, no dependency conflict |
| objc2-app-kit without feature gates | Feature-gated method availability | objc2-app-kit 0.3 | Must enable specific features for each API |

**Deprecated/outdated:**
- `lockFocus`/`unlockFocus` on NSImage: deprecated since macOS 10.14, removed in practice for new code
- `CGColorSpaceCreateDeviceRGB()` free function: renamed to `CGColorSpace::new_device_rgb()` method in objc2-core-graphics
- `CGContextDrawImage()` free function: renamed to `CGContext::draw_image()` method in objc2-core-graphics

## Open Questions

1. **What point size should be used for the default icon rendering?**
   - What we know: SF Symbols are vector-based. A 24pt symbol at 1x scale produces ~24px output; at 2x (Retina) it produces ~48px. The existing `DEFAULT_ICON_SIZE` constant of 24 matches the freedesktop default.
   - What's unclear: Whether the caller should specify size or if a fixed default is sufficient for this phase.
   - Recommendation: Default to 24 pixels. The `load_sf_icon` function takes `IconRole` only (matching `load_freedesktop_icon` signature). Size customization can be added to `load_icon()` in Phase 21 if needed.

2. **Should the function accept a scale factor for Retina displays?**
   - What we know: `CGImageForProposedRect` with `nil` rect uses the image's natural size at the configured point size. On Retina displays, `CGImage::width()` may return 2x the point size.
   - What's unclear: Whether the Phase 19 function should explicitly handle scale or let the caller (Phase 21's `load_icon()`) handle it.
   - Recommendation: For Phase 19, accept the natural CGImage dimensions from `CGImageForProposedRect`. The width/height stored in `IconData::Rgba` will reflect actual pixel dimensions. Phase 21 can add size/scale parameters to the unified `load_icon()` API.

3. **Are the required objc2-app-kit features already satisfied by existing Cargo.toml features?**
   - What we know: The current Cargo.toml has `"NSColor", "NSColorSpace", "NSAppearance", "NSFont", "NSFontDescriptor", "objc2-core-foundation"`. We need to add `"NSImage", "NSImageRep", "NSGraphicsContext", "objc2-core-graphics"`.
   - What's unclear: Whether `"NSImage"` is already implicitly enabled by another feature. The docs suggest it is NOT -- each feature must be explicitly enabled.
   - Recommendation: Explicitly add all 4 new features. Cargo deduplicates features, so adding already-enabled features is harmless.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in) |
| Config file | native-theme/Cargo.toml (features) |
| Quick run command | `cargo test -p native-theme --features system-icons --lib` |
| Full suite command | `cargo test -p native-theme --all-features` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PLAT-01-a | load_sf_icon returns Rgba for roles with SF Symbols names | integration | `cargo test -p native-theme --features system-icons sficons::tests::load_icon_returns_rgba -- -x` | Wave 0 |
| PLAT-01-b | Returned RGBA has correct dimensions (width * height * 4 == data.len()) | unit | `cargo test -p native-theme --features system-icons sficons::tests::rgba_dimensions_correct -- -x` | Wave 0 |
| PLAT-01-c | Straight alpha (not premultiplied) in output | unit | `cargo test -p native-theme --features system-icons sficons::tests::unpremultiply_correctness -- -x` | Wave 0 |
| PLAT-01-d | Missing SF Symbol falls back to bundled SVG | integration | `cargo test -p native-theme --features system-icons sficons::tests::fallback_to_bundled -- -x` | Wave 0 |
| PLAT-01-e | Roles without SF Symbols name (4 roles) fall back to bundled | integration | `cargo test -p native-theme --features system-icons sficons::tests::no_symbol_name_uses_fallback -- -x` | Wave 0 |

Note: Integration tests for PLAT-01-a, PLAT-01-b, PLAT-01-d, PLAT-01-e require running on macOS. On Linux CI, these tests should be gated with `#[cfg(target_os = "macos")]`. The `unpremultiply_correctness` test is a pure unit test that can run on any platform.

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme --features system-icons --lib`
- **Per wave merge:** `cargo test -p native-theme --all-features`
- **Phase gate:** Full suite green before verify-work

### Wave 0 Gaps
- [ ] `native-theme/src/sficons.rs` -- the main module (does not exist yet)
- [ ] Tests within the module covering PLAT-01 sub-behaviors
- [ ] `objc2-core-graphics` dependency in Cargo.toml
- [ ] Additional objc2-app-kit features in Cargo.toml (`NSImage`, `NSImageRep`, `NSGraphicsContext`, `objc2-core-graphics`)

## Sources

### Primary (HIGH confidence)
- [objc2-app-kit docs.rs](https://docs.rs/objc2-app-kit/latest/objc2_app_kit/) - NSImage methods, NSImageSymbolConfiguration, feature flags
- [objc2-core-graphics docs.rs](https://docs.rs/objc2-core-graphics/latest/objc2_core_graphics/) - CGBitmapContextCreate, CGContext, CGImage, CGColorSpace, CGImageAlphaInfo
- [Apple Developer Docs: NSImage init(systemSymbolName:)](https://developer.apple.com/documentation/appkit/nsimage/init(systemsymbolname:accessibilitydescription:)) - macOS 11+ availability
- [Apple Developer Docs: CGImageAlphaInfo](https://developer.apple.com/documentation/coregraphics/cgimagealphainfo) - Premultiplied vs straight alpha constants
- Existing codebase: `native-theme/src/macos.rs` (objc2 usage patterns), `native-theme/src/freedesktop.rs` (icon loader pattern)

### Secondary (MEDIUM confidence)
- [NSImage to pixels gist](https://gist.github.com/figgleforth/b5b193c3379b3f048210) - RGBA extraction approach via NSBitmapImageRep (alternative approach, verified principle)
- [mikeash.com pixel data article](https://www.mikeash.com/pyblog/friday-qa-2012-08-31-obtaining-and-interpreting-image-data.html) - CGBitmapContext as canonical approach for format normalization
- [Apple CGBitmapContext docs](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaDrawingGuide/Images/Images.html) - CGBitmapContext usage patterns

### Tertiary (LOW confidence)
- None -- all critical claims verified against official docs and existing codebase patterns

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - objc2 ecosystem already used in project, objc2-core-graphics is the natural addition for CG operations
- Architecture: HIGH - CGBitmapContext rasterization is Apple's canonical approach, verified via multiple official sources; module pattern follows existing freedesktop.rs exactly
- Pitfalls: HIGH - premultiplied alpha issue is well-documented; feature flag issues verified against actual objc2-app-kit docs.rs
- API availability: HIGH - NSImage(systemSymbolName:) is macOS 11+ (2020), all target users will have this

**Research date:** 2026-03-09
**Valid until:** 2026-04-09 (stable APIs, objc2 ecosystem is mature)
