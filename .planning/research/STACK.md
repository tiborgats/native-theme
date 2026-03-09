# Technology Stack: v0.3 Icon Loading Additions

**Project:** native-theme v0.3
**Researched:** 2026-03-09
**Scope:** New dependencies for icon loading only. Does NOT repeat v0.1/v0.2 validated stack (serde, toml, configparser, ashpd, windows crate, objc2-app-kit existing features, connectors).

---

## 1. macOS: SF Symbols via objc2-app-kit (existing dep, new features)

### No New Crate -- Additional Feature Flags Only

The project already depends on `objc2-app-kit = "0.3"` and `objc2-core-foundation` (transitive). Icon loading requires enabling additional feature flags on the same crate to access `NSImage`, `NSBitmapImageRep`, `NSGraphicsContext`, and the `objc2-core-graphics` bridge for CGImage pixel extraction.

**Confidence: HIGH** -- objc2-app-kit 0.3.2 already verified in v0.2 research. Feature flags confirmed via lib.rs/crates/objc2-app-kit/features.

### SF Symbol Loading Pipeline

```
NSImage::imageWithSystemSymbolName_accessibilityDescription("exclamationmark.triangle.fill", None)
  -> NSImage::withSymbolConfiguration(NSImageSymbolConfiguration::configurationWithPointSize_weight_scale(size, ...))
  -> NSImage::CGImageForProposedRect_context_hints(NULL, NULL, NULL)
  -> CGImageGetWidth / CGImageGetHeight
  -> CGBitmapContextCreate (RGBA8, premultiplied alpha)
  -> CGContextDrawImage (draw CGImage into context)
  -> CGBitmapContextGetData -> copy to Vec<u8>
  -> IconData::Rgba { width, height, data }
```

The CGBitmapContext approach is preferred over NSBitmapImageRep::bitmapData because:
1. **Guaranteed format**: you specify RGBA8 premultiplied layout when creating the context
2. **No format guessing**: NSBitmapImageRep can return ARGB, BGRA, different bit depths, or even CMYK
3. **Standard pattern**: this is Apple's recommended approach for getting controlled pixel output

### Required Cargo.toml Changes

```toml
[target.'cfg(target_os = "macos")'.dependencies]
objc2 = { version = "0.6", optional = true }
objc2-foundation = { version = "0.3", optional = true, features = ["NSString"] }
objc2-app-kit = { version = "0.3", optional = true, features = [
    # Existing (colors/fonts)
    "NSColor", "NSColorSpace", "NSAppearance", "NSFont", "NSFontDescriptor",
    "objc2-core-foundation",
    # NEW for icon loading
    "NSImage",                    # NSImage class, imageWithSystemSymbolName
    "NSImageRep",                 # NSImage::representations(), CGImageForProposedRect
    "NSBitmapImageRep",           # Fallback pixel extraction path
    "NSGraphicsContext",          # Required by CGImageForProposedRect
    "objc2-core-graphics",        # CGImage, CGBitmapContext, CGDataProvider
] }
block2 = { version = ">=0.6.1, <0.8.0", optional = true }
```

**New features explained:**
- `NSImage` -- `imageWithSystemSymbolName_accessibilityDescription()`, `withSymbolConfiguration()`, `size()`, `CGImageForProposedRect_context_hints()`
- `NSImageRep` -- required by `CGImageForProposedRect` method signature (returns via `NSImageRep` protocol)
- `NSBitmapImageRep` -- fallback path: `TIFFRepresentationUsingCompression_factor` requires this
- `NSGraphicsContext` -- parameter type in `CGImageForProposedRect_context_hints()`
- `objc2-core-graphics` -- `CGImage`, `CGBitmapContextCreate`, `CGContextDrawImage`, `CGImageGetWidth/Height`, `CGBitmapContextGetData` for controlled RGBA pixel extraction

### NSImageSymbolConfiguration

To control SF Symbol rendering size, the approach is:
1. Create `NSImageSymbolConfiguration` with `configurationWithPointSize_weight_scale(size, .regular, .large)`
2. Call `image.withSymbolConfiguration(config)` to get sized variant
3. Rasterize via CGBitmapContext at the configured size

This may require the `NSImageSymbolConfiguration` feature flag on `objc2-app-kit` if it exists as a separate gate. If it is bundled under `NSImage`, no additional flag is needed.

**Confidence: MEDIUM** -- The `NSImageSymbolConfiguration` feature flag presence needs verification during implementation. The class exists in AppKit (macOS 11+); the objc2-app-kit binding likely gates it under the `NSImage` feature, but this should be confirmed.

### Alternatives Rejected

| Alternative | Why Not |
|-------------|---------|
| core-graphics crate (servo) | Deprecated in favor of objc2-core-graphics. Uses legacy objc 0.2 runtime. |
| NSBitmapImageRep::bitmapData only | Pixel format varies (ARGB, BGRA, different bit depths). CGBitmapContext gives controlled RGBA8 output. |
| TIFFRepresentation + image crate decode | Over-engineering. Encodes to TIFF then decodes -- wasteful when direct pixel access is available. |

---

## 2. Windows: SHGetStockIconInfo + Segoe Fluent Icons Font

### 2a. SHGetStockIconInfo: New Feature Flag on Existing Dep

The existing `windows` crate dependency needs one additional feature flag: `Win32_UI_Shell`.

**Confidence: HIGH** -- `SHGetStockIconInfo` is under `windows::Win32::UI::Shell`. Feature flag naming convention confirmed: namespace path segments joined by underscores. `SHSTOCKICONINFO` struct requires `Win32_UI_WindowsAndMessaging` (already enabled) for its `Default` impl.

### HICON to RGBA Conversion Pipeline

```
SHGetStockIconInfo(SIID_WARNING, SHGSI_ICON | SHGSI_LARGEICON, &mut info)
  -> info.hIcon: HICON
  -> GetIconInfo(hIcon, &mut icon_info)
  -> icon_info.hbmColor: HBITMAP (color bitmap)
  -> CreateCompatibleDC(NULL) -> hdc
  -> GetObject(hbmColor, ...) -> BITMAP (get dimensions)
  -> Allocate Vec<u8> of width * height * 4
  -> Setup BITMAPINFOHEADER { biBitCount: 32, biCompression: BI_RGB, biHeight: -height (top-down) }
  -> GetDIBits(hdc, hbmColor, 0, height, buffer.as_mut_ptr(), &bmi, DIB_RGB_COLORS)
  -> Result: BGRA bytes in buffer
  -> Convert BGRA to RGBA (swap R and B channels)
  -> Apply alpha from icon_info.hbmMask if needed
  -> DestroyIcon(hIcon), DeleteDC(hdc), DeleteObject(hbmColor), DeleteObject(hbmMask)
  -> IconData::Rgba { width, height, data }
```

### Required Cargo.toml Changes

```toml
[dependencies]
windows = { version = ">=0.59, <=0.62", optional = true, default-features = false, features = [
    # Existing
    "UI_ViewManagement",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_HiDpi",
    "Win32_Graphics_Gdi",
    "Foundation_Metadata",
    # NEW for icon loading
    "Win32_UI_Shell",             # SHGetStockIconInfo, SHSTOCKICONID, SHGSI_FLAGS
] }
```

**New feature explained:**
- `Win32_UI_Shell` -- `SHGetStockIconInfo()`, `SHSTOCKICONID` enum, `SHGSI_FLAGS` constants

**Already available (no new features needed):**
- `Win32_UI_WindowsAndMessaging` -- `HICON`, `SHSTOCKICONINFO` struct, `GetIconInfo`, `ICONINFO`, `DestroyIcon`
- `Win32_Graphics_Gdi` -- `CreateCompatibleDC`, `GetDIBits`, `GetObject`, `BITMAPINFOHEADER`, `BITMAP`, `DeleteDC`, `DeleteObject`, `HBITMAP`, `HDC`

### 2b. Segoe Fluent Icons Font Glyph Rendering

For icons only available as Segoe Fluent Icons font glyphs (save, copy, paste, undo, redo, nav arrows, window controls), two approaches exist:

**Option A: DirectWrite (recommended) -- No new Rust crate dependency**

Use the `windows` crate's DirectWrite bindings to render individual glyphs:

```
CreateDWriteFactory() -> IDWriteFactory
  -> factory.CreateTextFormat("Segoe Fluent Icons", NULL, ..., size, "en-us")
  -> factory.CreateTextLayout(&[codepoint_char], format, size, size)
  -> Create ID2D1RenderTarget (WIC bitmap render target)
  -> layout.Draw(NULL, renderer, 0, 0)
  -> Extract pixel data from WIC bitmap
  -> IconData::Rgba { ... }
```

This requires additional `windows` crate features:

```toml
# Additional features for Segoe Fluent Icons glyph rendering
"Win32_Graphics_DirectWrite",     # IDWriteFactory, IDWriteTextFormat, IDWriteTextLayout
"Win32_Graphics_Direct2D",        # ID2D1Factory, render targets
"Win32_Graphics_Imaging",         # WIC bitmap for render target backing
```

**Option B: swash crate -- Pure Rust, cross-platform**

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| swash | 0.2.6 | Font introspection and glyph rendering | Pure Rust. Renders font glyphs to alpha masks or color bitmaps. Can load .ttf/.otf from bytes and render individual glyphs by codepoint. |

```
Load Segoe Fluent Icons font from C:\Windows\Fonts\SegoeIcons.ttf
  -> FontRef::from_index(font_data, 0)
  -> context.builder(font).size(size as f32).build() -> Scaler
  -> font.charmap().map(codepoint) -> GlyphId
  -> Render::new(&[Source::ColorOutline(0), Source::Outline]).render(&mut scaler, glyph_id)
  -> Image { data: Vec<u8>, content: Content::Mask (alpha) or Content::Color (RGBA), placement }
  -> Convert alpha mask to RGBA (apply foreground color) or use RGBA directly
  -> IconData::Rgba { ... }
```

**swash Image.content variants:**
- `Content::Mask` -- 8-bit alpha mask (1 byte per pixel). Caller applies foreground color.
- `Content::SubpixelMask` -- 32-bit subpixel mask (not useful for icons).
- `Content::Color` -- 32-bit RGBA bitmap (for color emoji / layered color outlines).

For Segoe Fluent Icons (monochrome outlines), swash produces `Content::Mask`. The caller must composite: for each pixel, `rgba = [fg_r, fg_g, fg_b, alpha]`.

**Recommendation: Option A (DirectWrite) for stock icons, defer swash**

Use DirectWrite for Segoe Fluent Icons rendering because:
1. The font is system-installed -- no need to bundle or find font files
2. DirectWrite handles font fallback, hinting, and ClearType natively
3. No additional crate dependency (just more `windows` features on the existing dep)
4. swash would require reading the font file from disk (path varies by Windows version)

However, if the DirectWrite pipeline proves too complex, swash 0.2.6 is a clean fallback with a simpler API.

**Confidence: MEDIUM** -- DirectWrite glyph rendering is well-documented in Win32 but the Rust bindings (`windows` crate) for DirectWrite/Direct2D/WIC have not been verified for this specific use case. The general approach is sound. swash 0.2.6 API verified via docs.rs.

### Alternatives Rejected

| Alternative | Why Not |
|-------------|---------|
| ab_glyph | Lower-level than swash, no built-in rasterizer output format. Would need manual scanline rendering. |
| fontdue | Fastest rasterizer but simpler API. Does not support color fonts or layered outlines. Sufficient for Segoe Fluent Icons but swash is more general. |
| cosmic-text | Full text layout engine -- massive overkill for rendering single glyphs. Depends on swash internally. |
| rusttype | Deprecated in favor of ab_glyph. |

---

## 3. Linux: freedesktop Icon Theme Lookup

### New Dependency: freedesktop-icons

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| freedesktop-icons | 0.4.0 | Freedesktop icon theme spec lookup | Only maintained Rust implementation of the Icon Theme Specification. Handles index.theme parsing, directory traversal, size matching, scale factor support, theme inheritance (e.g., Adwaita inherits hicolor). Builder API with caching. |

**Confidence: HIGH** -- Version 0.4.0 verified on lib.rs (published April 16, 2025). API confirmed: `lookup("dialog-warning").with_theme("Adwaita").with_size(24).with_scale(1).find()` returns `Option<PathBuf>`. The crate handles the full lookup algorithm internally.

### API Usage

```rust
use freedesktop_icons::lookup;

// Look up an icon in the current theme
let path: Option<PathBuf> = lookup("dialog-warning")
    .with_theme("Adwaita")      // or "breeze", "hicolor"
    .with_size(24)
    .with_scale(1)
    .with_cache()               // internal LRU cache
    .find();

// path is e.g. /usr/share/icons/Adwaita/24x24/status/dialog-warning-symbolic.svg
```

The returned `PathBuf` points to the actual icon file (SVG or PNG). The native-theme crate then:
1. Reads the file bytes (`std::fs::read(path)`)
2. Checks extension: `.svg` -> `IconData::Svg(bytes)`, `.png` -> needs rasterization or pass-through

### Detecting the Active Icon Theme

On GNOME: `gsettings get org.gnome.desktop.interface icon-theme` (or via the existing `ashpd` portal Settings reader which already reads `color-scheme`).

On KDE: read from `~/.config/kdeglobals` under `[Icons]` -> `Theme=breeze` (the existing `configparser` dep handles this).

This does NOT require a new dependency -- the existing platform reader code already accesses these config sources.

### Required Cargo.toml Changes

```toml
[dependencies]
freedesktop-icons = { version = "0.4", optional = true }
```

New feature flag:

```toml
[features]
freedesktop-icons = ["dep:freedesktop-icons"]
# Or bundle under a broader "icons" feature, see section 7
```

### Dependencies of freedesktop-icons 0.4.0

| Dependency | Version | Notes |
|------------|---------|-------|
| dirs | 5.0 | XDG directory resolution. Already in the crate ecosystem (configparser uses similar paths). |
| ini_core | 0.2.0 | Lightweight INI parser for index.theme files. Minimal. |
| once_cell | 1.19.0 | Lazy initialization for internal caches. Standard. |
| thiserror | 1.0 | Error derive macro. Widely used. |
| tracing | 0.1.41 | Structured logging. Does nothing unless a subscriber is installed. Zero overhead. |
| xdg | 2.5.2 | XDG Base Directory resolution. Standard for Linux desktop apps. |

No heavy or controversial dependencies. Total added dep tree is small.

### Alternatives Rejected

| Alternative | Why Not |
|-------------|---------|
| freedesktop-icon 0.0.3 | Different crate, pre-1.0, less maintained (no updates in 2+ years). |
| freedesktop-icons-greedy 0.2.6 | Fork with greedy matching. Less strict spec compliance. |
| linicon | Less popular, fewer downloads, less documented. freedesktop-icons is the established choice. |
| Manual index.theme parsing | The Icon Theme Spec has non-trivial lookup rules (inheritance, directory type matching, closest size). freedesktop-icons handles all of this correctly. Reimplementing would be error-prone and wasteful. |

---

## 4. SVG Handling: resvg for Optional Rasterization

### Recommendation: Return Raw SVG, Offer Optional Rasterization

The core `load_icon` function returns `IconData::Svg(Vec<u8>)` for Linux freedesktop icons and bundled icon sets. This is correct because:
1. SVG data is small (typically 1-5 KB per icon)
2. Most GUI toolkits (gpui, iced) have their own SVG renderers
3. Rasterization parameters (size, color, DPI) are best decided by the consumer

However, provide a **utility function** for consumers that cannot handle SVG:

```rust
/// Rasterize SVG bytes to RGBA pixels at the given size.
/// Requires the `svg-rasterize` feature.
#[cfg(feature = "svg-rasterize")]
pub fn rasterize_svg(svg_bytes: &[u8], width: u32, height: u32) -> Option<IconData> {
    let tree = usvg::Tree::from_data(svg_bytes, &usvg::Options::default()).ok()?;
    let mut pixmap = tiny_skia::Pixmap::new(width, height)?;
    resvg::render(&tree, /* fit transform */, &mut pixmap.as_mut());
    // resvg output is premultiplied RGBA -- unpremultiply for IconData
    Some(IconData::Rgba {
        width,
        height,
        data: pixmap.take(),
    })
}
```

### New Optional Dependency: resvg

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| resvg | 0.47 | SVG-to-RGBA rasterization | Pure Rust. Best SVG rendering library in the ecosystem. Uses tiny-skia for rendering. Fast, small binary (~3MB standalone). Supports most of SVG spec. Output is premultiplied RGBA pixels via tiny-skia Pixmap. |

**Confidence: HIGH** -- resvg 0.47.0 verified on docs.rs (released January 2026). API confirmed: `usvg::Tree::from_data()` + `resvg::render()` + `tiny_skia::Pixmap`. Dependencies: usvg 0.47, tiny-skia 0.12.

### Required Cargo.toml Changes

```toml
[dependencies]
resvg = { version = "0.47", optional = true, default-features = false }
```

Feature flag:

```toml
[features]
svg-rasterize = ["dep:resvg"]
```

**This is NOT in default features.** Most consumers will handle SVG themselves. The rasterizer is opt-in.

### Dependencies of resvg 0.47

| Dependency | Version | Notes |
|------------|---------|-------|
| usvg | 0.47 | SVG parser + tree builder. Pulled automatically by resvg. |
| tiny-skia | 0.12 | 2D rendering engine. Pure Rust Skia subset. |
| svgtypes | 0.16 | SVG type definitions. |
| rgb | 0.8 | Pixel format types. |

Moderate dep tree size. Default features include PNG support; can be trimmed with `default-features = false` if only SVG rendering is needed.

### Alternatives Rejected

| Alternative | Why Not |
|-------------|---------|
| Return SVG always, let consumer rasterize | Good default, but some consumers need RGBA and don't have an SVG renderer. Providing opt-in rasterization covers that gap. |
| image crate for SVG | The image crate does not render SVG. It handles raster formats only. |
| cairo-rs | C dependency (libcairo). Not pure Rust. Requires system library installation. |
| rsvg / librsvg binding | C dependency (librsvg + glib + cairo). Heavy. Not appropriate for a toolkit-agnostic crate. |

---

## 5. Bundled Icon Sets: Material Symbols + Lucide

### Strategy: include_bytes! with Build Script

Bundle SVG icons as compile-time embedded byte slices using `include_bytes!()`. A build script generates a Rust source file mapping icon names to their SVG data.

**Why this approach:**
- Zero runtime filesystem access
- Deterministic -- icons are part of the binary
- No external data files to distribute
- Compile-time verified (missing icons = build error)

### Build Script Pipeline

```
build.rs:
  1. Read icons from icons/material/*.svg and icons/lucide/*.svg
  2. For each icon role in the mapping table:
     - Find the corresponding SVG file
     - Generate: pub const DIALOG_WARNING: &[u8] = include_bytes!("../../icons/material/warning.svg");
  3. Write generated source to OUT_DIR/material_icons.rs and OUT_DIR/lucide_icons.rs

src/icons/material.rs:
  include!(concat!(env!("OUT_DIR"), "/material_icons.rs"));

  pub fn lookup(role: IconRole) -> Option<&'static [u8]> {
      match role {
          IconRole::DialogWarning => Some(DIALOG_WARNING),
          ...
      }
  }
```

### Icon File Sources

**Material Symbols (Outlined, 24px SVGs):**
- Source: https://github.com/google/material-design-icons
- License: Apache 2.0 (compatible with MIT/Apache-2.0/0BSD)
- Need only ~35 SVGs (one per IconRole)
- Files are typically 24x24 viewport, optimized SVGs (~1-3 KB each)
- Total bundled size: ~50-100 KB for 35 icons

**Lucide (24px SVGs):**
- Source: https://github.com/lucide-icons/lucide
- License: ISC (compatible with MIT/Apache-2.0/0BSD)
- Need only ~35 SVGs (one per IconRole)
- Files are 24x24 viewport, well-optimized (~500 bytes - 2 KB each)
- Total bundled size: ~30-60 KB for 35 icons

### Required Feature Flags

```toml
[features]
default = ["system-icons", "material-icons"]
system-icons = []           # Platform-native icon loading (macOS/Windows/Linux APIs)
material-icons = []         # Bundle Material Symbols SVGs as cross-platform fallback
lucide-icons = []           # Bundle Lucide SVGs as optional icon set
svg-rasterize = ["dep:resvg"]  # Optional: rasterize SVG to RGBA
```

### No New Runtime Dependencies

The bundled icons use `include_bytes!()` -- no runtime file I/O, no parsing libraries, no image decoders. The build script uses only `std::fs` and `std::io` (always available).

### Alternatives Rejected

| Alternative | Why Not |
|-------------|---------|
| Separate data crate (e.g., `native-theme-icons`) | Over-engineering for 35 SVGs (~100 KB). A separate crate adds publishing complexity. Feature flags on the main crate are simpler. |
| Runtime SVG loading from bundled directory | Requires filesystem access. Does not work in all environments (WASM, embedded). compile-time embedding is more robust. |
| Icon font (TTF) bundling + swash rendering | Converts SVG to font, then renders font to pixels. Two conversion steps when the SVG is already the desired format. Over-engineering. |
| Embedding as string literals | Binary `include_bytes!` is cleaner, avoids escaping issues, and is what SVG renderers expect (byte slices). |

---

## 6. Windows Feature Flag Summary

The existing `windows` crate dependency needs these additional features for icon loading:

```toml
windows = { version = ">=0.59, <=0.62", optional = true, default-features = false, features = [
    # Existing (theme colors, fonts, geometry)
    "UI_ViewManagement",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_HiDpi",
    "Win32_Graphics_Gdi",
    "Foundation_Metadata",
    # NEW for icon loading (stock icons)
    "Win32_UI_Shell",
    # NEW for icon loading (Segoe Fluent Icons glyph rendering) -- optional, see section 2b
    # "Win32_Graphics_DirectWrite",
    # "Win32_Graphics_Direct2D",
    # "Win32_Graphics_Imaging",
] }
```

**Phase approach:** Start with `Win32_UI_Shell` for stock icons (covers ~18 of 42 icon roles). Add DirectWrite features later for Segoe Fluent Icons glyph rendering (covers remaining ~24 roles). This reduces initial scope and risk.

---

## 7. Updated Feature Flags (v0.3)

```toml
[features]
default = ["system-icons", "material-icons"]

# Platform readers (unchanged from v0.2)
kde = ["dep:configparser"]
portal = ["dep:ashpd"]
portal-tokio = ["portal", "ashpd/tokio"]
portal-async-io = ["portal", "ashpd/async-io"]
windows = ["dep:windows"]
macos = ["dep:objc2", "dep:objc2-foundation", "dep:objc2-app-kit", "dep:block2"]

# NEW in v0.3: Icon loading
system-icons = []           # Enable platform-native icon loading APIs
material-icons = []         # Bundle Material Symbols SVGs (~100 KB)
lucide-icons = []           # Bundle Lucide SVGs (~60 KB)
svg-rasterize = ["dep:resvg"]  # Optional SVG-to-RGBA rasterization
freedesktop-icons = ["dep:freedesktop-icons"]  # Linux icon theme lookup
```

**Feature interaction:**
- `system-icons` + `macos` = SF Symbols via NSImage APIs
- `system-icons` + `windows` = SHGetStockIconInfo + Segoe Fluent Icons
- `system-icons` + `freedesktop-icons` = freedesktop icon theme lookup on Linux
- `material-icons` = bundled fallback, works on all platforms, no OS deps
- `lucide-icons` = bundled alternative, works on all platforms
- `svg-rasterize` = utility function for consumers without SVG rendering

---

## 8. Version Compatibility Matrix (v0.3 additions)

| Package | Version | Compatible With | Notes |
|---------|---------|-----------------|-------|
| freedesktop-icons | 0.4.0 | dirs 5.0, xdg 2.5, thiserror 1.0 | Latest release, April 2025. Stable API. |
| resvg | 0.47.0 | usvg 0.47, tiny-skia 0.12 | Latest release, January 2026. Active development (linebender org). |
| objc2-app-kit | 0.3.2 | objc2 >=0.6.2 <0.8.0 | Unchanged from v0.2. Additional features only. |
| objc2-core-graphics | 0.3.2 | objc2 >=0.6.2 <0.8.0 | Transitive via objc2-app-kit's `objc2-core-graphics` feature. |
| windows | >=0.59, <=0.62 | Rust 1.61+ | Unchanged from v0.2. Additional features only. |
| swash | 0.2.6 | (standalone, minimal deps) | Backup for font glyph rendering. Not in initial scope. |

---

## 9. What NOT to Add in v0.3

| Avoid | Why | Notes |
|-------|-----|-------|
| image crate | Only needed if we decoded PNGs from freedesktop themes. SVG pass-through avoids this. If PNG icons are found, either skip them or defer to consumer. | Massive dependency tree. |
| svg crate | SVG construction/manipulation library. We only read existing SVGs, not create them. | Wrong tool. |
| icondata crate | Pre-bundled icon data for web frameworks (Yew, Leptos). Not useful for native desktop icons. | Wrong audience. |
| embed-bytes | Over-abstraction over `include_bytes!`. The standard macro is sufficient. | Unnecessary dependency. |
| verglas | Converts SVG icons to icon fonts. We want SVGs, not fonts. | Wrong direction. |
| Cairo/librsvg bindings | C dependencies. Against project philosophy (pure Rust, toolkit-agnostic). | resvg is the pure Rust alternative. |
| Full icon font bundling | Bundling Material Symbols as a variable font (.ttf) + using swash to render would give font variation axis control (fill, weight, grade) but adds ~2 MB binary size and rendering complexity. SVG files are simpler and sufficient. | Future consideration for v0.4+. |
| thiserror (in main crate) | The existing manual Error enum pattern works. freedesktop-icons pulls thiserror transitively but the main crate should not depend on it directly. | Keep error handling consistent with v0.1/v0.2. |

---

## 10. Dependency Delta Summary (v0.2 to v0.3)

### New Runtime Dependencies

| Dependency | Feature Gate | Added For |
|------------|-------------|-----------|
| freedesktop-icons 0.4 | `freedesktop-icons` | Linux icon theme spec lookup |
| resvg 0.47 | `svg-rasterize` | Optional SVG-to-RGBA rasterization |

### Extended Feature Flags on Existing Dependencies

| Dependency | New Features | Added For |
|------------|-------------|-----------|
| objc2-app-kit 0.3 | `NSImage`, `NSImageRep`, `NSBitmapImageRep`, `NSGraphicsContext`, `objc2-core-graphics` | macOS SF Symbol loading + pixel extraction |
| windows >=0.59, <=0.62 | `Win32_UI_Shell` | SHGetStockIconInfo for stock icons |

### No New Dependencies (compile-time only)

| Mechanism | Purpose |
|-----------|---------|
| build.rs + include_bytes! | Embed Material/Lucide SVG icons at compile time |

### Deferred (not in v0.3)

| Dependency | Why Deferred |
|------------|-------------|
| swash 0.2.6 | Backup for font glyph rendering. DirectWrite preferred on Windows. Reconsider if DirectWrite pipeline proves too complex. |
| windows DirectWrite/Direct2D/WIC features | Segoe Fluent Icons glyph rendering. Can be added incrementally after stock icon pipeline works. |

---

## Sources

- [docs.rs/objc2-app-kit NSImage](https://docs.rs/objc2-app-kit/latest/objc2_app_kit/struct.NSImage.html) -- imageWithSystemSymbolName, CGImageForProposedRect methods verified (HIGH confidence)
- [lib.rs/objc2-app-kit features](https://lib.rs/crates/objc2-app-kit/features) -- NSImage, NSImageRep, NSBitmapImageRep, NSGraphicsContext, objc2-core-graphics feature flags confirmed (HIGH confidence)
- [docs.rs/objc2-core-graphics](https://docs.rs/objc2-core-graphics/latest/objc2_core_graphics/) -- v0.3.2, CGImage/CGBitmapContext/CGDataProvider functions available (HIGH confidence)
- [Apple NSImage.SymbolConfiguration docs](https://developer.apple.com/documentation/appkit/nsimage/symbolconfiguration) -- pointSize:weight:scale: initializer confirmed (HIGH confidence)
- [Apple NSImage init(systemSymbolName:) docs](https://developer.apple.com/documentation/appkit/nsimage/3622472-init) -- macOS 11+ availability confirmed (HIGH confidence)
- [microsoft.github.io/windows-docs-rs SHGetStockIconInfo](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/fn.SHGetStockIconInfo.html) -- function in Win32::UI::Shell confirmed (HIGH confidence)
- [microsoft.github.io/windows-docs-rs SHSTOCKICONINFO](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/struct.SHSTOCKICONINFO.html) -- requires Win32_UI_WindowsAndMessaging for Default (HIGH confidence)
- [microsoft.github.io/windows-docs-rs GetDIBits](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/fn.GetDIBits.html) -- in Win32::Graphics::Gdi, already enabled (HIGH confidence)
- [lib.rs/freedesktop-icons](https://lib.rs/crates/freedesktop-icons) -- v0.4.0, April 2025, builder API with theme/size/scale/cache (HIGH confidence)
- [docs.rs/freedesktop-icons](https://docs.rs/freedesktop-icons/latest/freedesktop_icons/) -- API: lookup().with_theme().with_size().find() -> Option<PathBuf> (HIGH confidence)
- [docs.rs/resvg](https://docs.rs/resvg/latest/resvg/) -- v0.47.0, render() + tiny-skia Pixmap for RGBA output (HIGH confidence)
- [github.com/linebender/resvg](https://github.com/linebender/resvg) -- moved to linebender org, actively maintained (HIGH confidence)
- [docs.rs/swash](https://docs.rs/swash/latest/swash/) -- v0.2.6, Content::Mask/Color/SubpixelMask for glyph images (MEDIUM confidence)
- [pop-os.github.io/cosmic-text/swash Content enum](https://pop-os.github.io/cosmic-text/swash/scale/image/enum.Content.html) -- Mask (alpha), SubpixelMask, Color (RGBA) variants (MEDIUM confidence)
- [github.com/google/material-design-icons](https://github.com/google/material-design-icons) -- Apache 2.0, individual SVG files available (HIGH confidence)
- [github.com/lucide-icons/lucide](https://github.com/lucide-icons/lucide) -- ISC license, individual SVG files in repo (HIGH confidence)
- [users.rust-lang.org HICON to PNG thread](https://users.rust-lang.org/t/how-to-convert-hicon-to-png/90975) -- HICON->RGBA via GetIconInfo+GetDIBits pattern (MEDIUM confidence)
- [GitHub gist: NSImage to RGBA pixels](https://gist.github.com/figgleforth/b5b193c3379b3f048210) -- CGBitmapContext recommended over NSBitmapImageRep for controlled format (MEDIUM confidence)

---
*Stack research for: native-theme v0.3 icon loading*
*Researched: 2026-03-09*
