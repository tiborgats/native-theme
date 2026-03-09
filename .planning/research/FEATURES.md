# Feature Landscape: Native Icon Loading (v0.3)

**Domain:** Cross-platform native icon loading for a toolkit-agnostic theme crate (Rust)
**Researched:** 2026-03-09
**Overall confidence:** HIGH

This document covers ONLY the native icon loading features planned as a subsequent milestone. Existing v0.2 features (36-color model, 17 presets, 4 platform readers, widget metrics, connectors) are already shipped. The spec in `docs/native-icons.md` defines the target API: `IconRole` enum (42 roles), `IconData` (SVG or RGBA), `load_icon()`, `icon_name()`, and `system_icon_set()`.

---

## Table Stakes

Features users expect when icon loading is advertised. Missing any of these makes the feature feel incomplete.

### 1. IconRole Enum and icon_name() Lookup

| Aspect | Detail |
|--------|--------|
| Why expected | The entire value proposition of native-theme icons is "one enum, every platform." Without the static mapping from semantic roles to platform-specific identifier strings, there is no abstraction at all. Connectors (especially gpui, which already has Lucide icons loaded) need string lookups without loading pixels. |
| Complexity | LOW |
| Dependencies | None -- pure Rust data, no FFI, no I/O |

**What this involves:**

- 42-variant `IconRole` enum (Dialog, Window, Action, Navigation, Files, Status, System groups)
- `icon_name(icon_theme: &str, role: IconRole) -> Option<&'static str>` returning the platform-specific identifier string for each (icon_theme, role) pair
- Five icon theme namespaces: `"sf-symbols"`, `"segoe-fluent"`, `"freedesktop"`, `"material"`, `"lucide"`
- Static `match` arms -- no allocations, no runtime cost
- Full coverage verified: the spec's availability matrix shows 100% coverage for Material and Lucide, 95%+ for SF Symbols and freedesktop, ~80% for Segoe Fluent (stock icons + font glyphs combined)

**Gaps that return None:**

| Role | Missing from | Reason |
|------|-------------|--------|
| `dialog-success` | Windows (Segoe Fluent) | No standard Windows success icon |
| `folder-open` | SF Symbols | Apple has no open-folder symbol |
| `trash-full` | SF Symbols, Material, Lucide | Only freedesktop and Windows distinguish empty vs full trash |
| `window-restore` | SF Symbols | macOS uses colored traffic lights, no restore concept |
| `status-loading` | SF Symbols, Windows | Loading is animated on these platforms, not a static icon |
| `notification` | freedesktop | No standard notification bell in freedesktop spec |

These gaps are acceptable -- `Option<&str>` communicates availability clearly. Callers can fall back to Material/Lucide which have near-complete coverage.

### 2. icon_theme Field on ThemeVariant

| Aspect | Detail |
|--------|--------|
| Why expected | Without per-variant icon theme selection, there is no way to connect the icon system to the preset/theme system. The spec defines `icon_theme: Option<String>` on `ThemeVariant`, with `None` meaning "use system_icon_set() at runtime." |
| Complexity | LOW |
| Dependencies | ThemeVariant struct (already `#[non_exhaustive]`, so adding a field is non-breaking) |

**What this involves:**

- Add `icon_theme: Option<String>` to `ThemeVariant`
- Update 17 preset TOML files: native presets get explicit values (`"sf-symbols"`, `"segoe-fluent"`, `"freedesktop"`), community presets get `None` (resolved at runtime)
- `system_icon_set() -> &'static str`: returns `"sf-symbols"` on macOS/iOS, `"segoe-fluent"` on Windows, `"freedesktop"` on Linux, `"material"` on other platforms
- Serde support: `skip_serializing_if = "Option::is_none"` so community theme TOML files stay clean

### 3. Bundled SVG Fallback Icons (Material Symbols)

| Aspect | Detail |
|--------|--------|
| Why expected | On Linux (non-KDE/GNOME), on any platform without native icon APIs compiled in, or when a user explicitly wants consistent cross-platform icons, there must be a working fallback. Without bundled icons, `load_icon()` returns `None` on any platform that lacks native API access. This defeats the purpose. |
| Complexity | MEDIUM |
| Dependencies | Feature flag `material-icons`, SVG files at build time |

**What this involves:**

- Bundle ~42 Material Symbols Outlined SVGs (one per IconRole) as `include_bytes!()` in a module gated by `material-icons` feature
- Total binary size: ~42 SVGs x ~500 bytes average = ~21KB compressed. Material Symbols individual SVGs are small (simple paths, no embedded fonts)
- SVGs sourced from [google/material-design-icons](https://github.com/google/material-design-icons) repository (Apache 2.0 license, redistributable)
- `load_icon("material", role, _size) -> Some(IconData::Svg(bytes))` -- size parameter is informational for SVG (SVGs are resolution-independent); callers render at desired size
- Default feature: `default = ["system-icons", "material-icons"]` ensures there is always a fallback

**Why Material Symbols over Lucide as default:**

- Material has 100% coverage of all 42 IconRole variants (no gaps)
- Material is visually closer to platform-native icons (filled style options, optical sizes)
- Lucide's stroke-based design is distinctive -- good as an option, but noticeably non-native as a default
- Apache 2.0 license is permissive for bundling

### 4. Freedesktop Icon Theme Lookup (Linux)

| Aspect | Detail |
|--------|--------|
| Why expected | Linux is a primary platform for native-theme. Linux desktop icons live in theme directories following the freedesktop Icon Theme Specification. Without directory lookup, Linux icons don't work at all. |
| Complexity | MEDIUM |
| Dependencies | Feature flag `system-icons`, filesystem access |

**What this involves:**

- Implement or depend on freedesktop icon lookup following the [Icon Theme Specification](https://specifications.freedesktop.org/icon-theme/latest/)
- Search order: `$HOME/.icons`, `$XDG_DATA_DIRS/icons`, `/usr/share/pixmaps`
- Parse `index.theme` files to find the right subdirectory for requested size
- Fallback chain: requested theme -> inherited themes -> `hicolor`
- Prefer `-symbolic` suffix variants (monochrome, recolorable SVGs) for action/navigation/system icons; use full-color variants for dialog icons
- Return `IconData::Svg(bytes)` by reading the SVG file from disk

**Build vs buy decision for freedesktop lookup:**

| Option | Pros | Cons | Recommendation |
|--------|------|------|----------------|
| `freedesktop-icons` crate (v0.4, 176K downloads/month) | Battle-tested, handles index.theme parsing, theme inheritance, caching, scale support | External dependency, may pull in more than needed | USE THIS |
| `icon-loader` crate (v0.4.1) | 100% safe Rust, KDE/GTK theme detection built in | Linux-only, no macOS/Windows, smaller community | Skip -- overlaps with what native-theme already does for theme detection |
| Custom implementation | No external dependencies, minimal code for 42 icons | Re-implementing spec parsing is error-prone, edge cases in index.theme format | Skip -- not worth the effort for an icon spec with inheritance chains |

**Recommendation:** Depend on `freedesktop-icons` behind the `system-icons` feature flag on `cfg(target_os = "linux")`. Its API is clean: `lookup("dialog-warning").with_size(24).with_theme("Adwaita").find()` returns `Option<PathBuf>`. Read the SVG file at that path and return `IconData::Svg`.

### 5. macOS SF Symbols Loading

| Aspect | Detail |
|--------|--------|
| Why expected | macOS is a primary platform. SF Symbols are the native icon system (6,900+ symbols). Users choosing the macOS preset or `"sf-symbols"` icon theme expect real Apple icons, not Material fallbacks. |
| Complexity | HIGH |
| Dependencies | Feature flag `system-icons`, `objc2-app-kit` with `NSImage` feature, `objc2-core-graphics` |

**What this involves:**

The loading pipeline is: symbol name string -> NSImage -> CGImage -> RGBA pixel buffer.

1. **Create NSImage from symbol name:**
   `NSImage::imageWithSystemSymbolName_accessibilityDescription(name, None)` (available macOS 11+). The `objc2-app-kit` crate provides this method when the `NSImage` feature is enabled.

2. **Configure symbol appearance (optional but important):**
   `NSImageSymbolConfiguration::configurationWithPointSize_weight_scale(size, weight, scale)` controls rendering size, weight (ultraLight through black, 9 levels), and scale (small/medium/large). Apply via `imageWithSymbolConfiguration:`. For native-theme's use case, use medium weight and large scale as defaults.

3. **Rasterize to RGBA:**
   - Get CGImage: `NSImage::CGImageForProposedRect_context_hints(nil, nil, nil)` (requires `objc2-core-graphics` feature)
   - Create NSBitmapImageRep from CGImage
   - Extract pixel data via `bitmapData` property
   - Copy into owned `Vec<u8>` for `IconData::Rgba { width, height, data }`

4. **Template mode / tinting:**
   SF Symbols are "template images" by default -- they render in the current drawing context's foreground color. When rasterizing outside a drawing context, they render as black. This is fine for `IconData::Rgba` because:
   - Connectors can apply tinting using the theme's foreground color
   - The alternative (capturing a tinted render) requires setting up an NSGraphicsContext, which is complex and couples the icon to a specific color at load time

**Key dependency requirements:**
   - `objc2-app-kit` features: `NSImage`, `NSBitmapImageRep`, `NSGraphicsContext`
   - `objc2-core-graphics` (for CGImage conversion)
   - Existing `objc2-foundation` dependency already in the project

**Risk:** The rasterization pipeline (NSImage -> CGImage -> bitmap) involves several `unsafe` blocks and memory management. The existing macOS reader already demonstrates this pattern for NSColor, so the team has precedent for objc2 FFI. But icon rasterization has more moving parts (image representations, bitmap formats, premultiplied alpha).

### 6. Windows Icon Loading (SHGetStockIconInfo + Segoe Fluent)

| Aspect | Detail |
|--------|--------|
| Why expected | Windows is a primary platform. The spec defines two icon sources: SHSTOCKICONID stock icons (dialog, file, system icons) and Segoe Fluent Icons font glyphs (action, navigation, window control icons). Both must work for full coverage. |
| Complexity | HIGH |
| Dependencies | Feature flag `system-icons`, `windows` crate with Shell and GDI features |

**What this involves -- two separate pipelines:**

**Pipeline A: Stock icons via SHGetStockIconInfo (18 roles)**

1. Call `SHGetStockIconInfo(SIID_*, SHGSI_ICON | SHGSI_LARGEICON, &mut info)` to get an HICON handle
2. Convert HICON to RGBA pixels:
   - `GetIconInfo(hicon, &mut iconinfo)` to get HBITMAP handles (color + mask bitmaps)
   - `CreateCompatibleDC(null)` to create a device context
   - `GetDIBits(hdc, hbm_color, ...)` with a BITMAPINFO header requesting 32-bit BGRA
   - Swap B and R channels to produce RGBA
   - Compose with mask bitmap for proper alpha
3. Clean up: `DestroyIcon(hicon)`, `DeleteObject(hbm_color)`, `DeleteObject(hbm_mask)`, `DeleteDC(hdc)`
4. Return `IconData::Rgba { width, height, data }`

**Pixel format note:** Windows GDI returns pixels in BGRA order with premultiplied alpha. Must convert to straight RGBA for `IconData`.

**Pipeline B: Segoe Fluent Icons font glyphs (24 roles)**

1. Load the Segoe Fluent Icons font (present on all Windows 10/11 installs)
2. Render the Unicode codepoint (e.g., U+E74E for Save) to a bitmap
3. Two approaches:
   - **DirectWrite + Direct2D:** Most correct. Create `IDWriteTextFormat` with "Segoe Fluent Icons", render glyph to `ID2D1Bitmap`, read pixels. Heavy dependency chain.
   - **GDI fallback:** `CreateFont` + `SelectObject` + `TextOut` into a DIB section. Simpler but less precise rendering (no subpixel, no ClearType).

**Recommended approach:** Start with the GDI fallback path. It requires only `windows` crate features already in the project (`Win32_Graphics_Gdi`). The font glyphs are simple monochrome shapes -- GDI renders them adequately. DirectWrite can be added later if quality is insufficient.

**Optimal icon sizes:** Segoe Fluent Icons are designed for 16, 20, 24, 32, 40, 48, and 64 pixels. Requesting other sizes may produce fuzzy results.

**DPI consideration:** The existing Windows reader already calls `GetDpiForSystem()`. Stock icons from `SHGetStockIconInfo` with `SHGSI_LARGEICON` return 32x32 at 96 DPI, scaled at higher DPI. Font glyph rendering should use the requested pixel size directly.

### 7. IconData Return Type

| Aspect | Detail |
|--------|--------|
| Why expected | The dual-format return type (`Svg` vs `Rgba`) is the core abstraction that makes cross-platform icon loading work. SVG for Linux/bundled icons, RGBA for macOS/Windows rasterized icons. Without this, connectors cannot handle the output generically. |
| Complexity | LOW |
| Dependencies | None -- simple enum |

**What this involves:**

```rust
pub enum IconData {
    Svg(Vec<u8>),           // SVG content (freedesktop, Material, Lucide)
    Rgba { width: u32, height: u32, data: Vec<u8> },  // RGBA pixels (macOS, Windows)
}
```

**Design decisions baked into this type:**

- **No `Png` variant:** PNG is an intermediate format. Connectors that need PNG can use the `image` crate to encode from RGBA. Adding PNG would force a dependency on a PNG decoder in the core crate.
- **No `Path` variant:** Returning a file path instead of bytes would avoid I/O in the core crate but leak filesystem details to connectors. The spec correctly loads the bytes in the core crate.
- **`Vec<u8>` not `&[u8]`:** Owned data avoids lifetime complexity. The SVG bytes from `include_bytes!` could theoretically be `&'static [u8]`, but RGBA data is always dynamically allocated. Unified `Vec<u8>` keeps the API simple. A `Cow<'static, [u8]>` alternative could avoid one copy for bundled SVGs but adds API noise for marginal benefit.
- **Size parameter is caller's responsibility for DPI:** The spec says "callers should multiply the desired point size by the scale factor (e.g. 24pt x 2 = 48px)." This is correct -- the core crate should not query DPI itself, as the correct DPI depends on which monitor the window is on, which only the GUI toolkit knows.

---

## Differentiators

Features that set native-theme's icon support apart. Not strictly expected but create significant value.

### 8. Bundled Lucide SVG Icons (Optional)

| Aspect | Detail |
|--------|--------|
| Value proposition | gpui-component already bundles 87 Lucide icons. When the gpui connector sees `icon_theme = "lucide"`, it can map `IconRole` directly to gpui-component's `IconName` enum without any I/O or SVG parsing -- zero-cost native integration. For non-gpui users, bundled Lucide SVGs provide a lightweight alternative to Material with a distinct visual style. |
| Complexity | LOW |
| Dependencies | Feature flag `lucide-icons` |

**What this involves:**

- Bundle ~42 Lucide SVGs (one per IconRole) as `include_bytes!()` behind `lucide-icons` feature
- Lucide SVGs are stroke-based (unlike Material's filled style), averaging ~300-400 bytes each. Total: ~15KB
- ISC license (very permissive, compatible with any project)
- `icon_name("lucide", role)` returns Lucide kebab-case names (`"triangle-alert"`, `"circle-x"`, etc.)
- gpui connector shortcut: `icon_name_for_role(role) -> Option<IconName>` maps to gpui-component's existing `IconName` enum for 27 of 42 roles. The remaining 15 roles need the full SVG from `load_icon()`

**Not default:** Unlike `material-icons`, `lucide-icons` is opt-in. Material has better coverage and is closer to native icon aesthetics.

### 9. Symbolic Icon Tinting Metadata

| Aspect | Detail |
|--------|--------|
| Value proposition | Freedesktop `-symbolic` icons and SF Symbols are designed to be recolored with the current theme foreground color. Material and Lucide SVGs use `currentColor` or fixed black fills that should be tinted. Providing tinting metadata with `IconData` lets connectors render icons in the correct theme color without guessing. |
| Complexity | LOW |
| Dependencies | Adds one field to load_icon() output or a wrapper struct |

**What this involves:**

Two approaches:

**Approach A (recommended): Convention-based tinting**

Document that `IconData::Svg` bytes should be rendered with the theme's `foreground` color applied as fill/stroke, and `IconData::Rgba` pixels should be treated as alpha masks (multiply each pixel by the foreground color). This is how GTK, AppKit, and WinUI all handle monochrome icons.

No API change needed -- just documentation. Connectors already know the theme foreground color from `ThemeVariant.colors.foreground`.

**Approach B: Explicit metadata**

```rust
pub struct LoadedIcon {
    pub data: IconData,
    pub is_template: bool,  // true = monochrome, should be tinted
}
```

This adds clarity but increases API surface. Not worth it for v0.3 when the convention is universal: all platform icon systems treat symbolic/template icons as tintable.

**Recommendation:** Use Approach A (convention + documentation). Every icon returned by `load_icon()` is a monochrome template icon that should be tinted with the foreground color. This matches how SF Symbols (template mode), freedesktop `-symbolic` icons, and Material/Lucide SVGs all work. Full-color icons (freedesktop non-symbolic) are an edge case that can be addressed later.

### 10. Connector Icon Integration Helpers

| Aspect | Detail |
|--------|--------|
| Value proposition | The connectors (native-theme-gpui, native-theme-iced) need to convert `IconData` to toolkit-specific image types. Providing helper methods on the connector crates makes icon usage a one-liner for application code. |
| Complexity | LOW per connector |
| Dependencies | Existing connector crates, `IconData` type |

**gpui connector:**

```rust
// For icon_theme = "lucide" with gpui-component's built-in icons:
pub fn icon_name_for_role(role: IconRole) -> Option<IconName>

// For all other icon themes:
pub fn load_icon(variant: &ThemeVariant, role: IconRole, size: u32) -> Option<RenderImage>
```

The Lucide shortcut avoids SVG parsing/rendering entirely since gpui-component already has those icons loaded. This is a meaningful performance win in hot paths (toolbar rendering, list views with icons).

**iced connector:**

```rust
pub fn load_icon_handle(variant: &ThemeVariant, role: IconRole, size: u32) -> Option<iced::widget::image::Handle>
```

iced's `image::Handle` can be created from RGBA bytes or SVG bytes. The connector dispatches based on `IconData` variant.

### 11. SVG Rendering to RGBA (Optional Utility)

| Aspect | Detail |
|--------|--------|
| Value proposition | When a connector receives `IconData::Svg` but the toolkit only accepts raster images, the SVG must be rendered to pixels. Providing an optional `svg-render` feature that uses `resvg` + `tiny-skia` saves every connector from independently solving this. |
| Complexity | MEDIUM |
| Dependencies | Optional feature flag `svg-render`, `resvg` + `tiny-skia` dependencies |

**What this involves:**

```rust
/// Render SVG bytes to RGBA pixels at the specified size.
/// Requires the `svg-render` feature.
#[cfg(feature = "svg-render")]
pub fn render_svg(svg_bytes: &[u8], size: u32) -> Option<IconData> {
    // Returns IconData::Rgba { width: size, height: size, data }
}
```

- `resvg` (latest: 0.45.x) renders static SVG subset -- exactly what icon SVGs use (no animation, no scripts)
- `resvg` uses `tiny-skia` internally, which renders to RGBA8888 pixel buffers
- Combined dependency adds ~500KB to binary (acceptable for an opt-in feature)
- Both crates are actively maintained by the linebender project

**When needed:**
- Toolkits that only accept raster images
- When `IconData::Svg` comes from freedesktop lookup but the consumer needs pixels
- Unit tests that want to verify SVG icons render without errors

**When NOT needed:**
- gpui has its own SVG renderer
- iced has SVG widget support
- Web targets (WASM) can render SVG natively in the browser

**Recommendation:** Include as optional feature, do not make it default. Most GUI toolkits can handle SVGs directly.

---

## Anti-Features

Features to explicitly NOT build in the icon loading milestone.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Icon caching / memoization | Caching policy is application-specific (LRU? TTL? per-window?). Adding a cache in the core crate couples it to a particular usage pattern. Freedesktop icon lookup already has its own caching. | Document that `load_icon()` performs I/O on each call. Consumers should cache `IconData` at the application level. The `icon_name()` function is free (static lookup) and can be used for cache keys. |
| Animated icon support | Loading spinners and progress indicators are animated on every platform. Animation is a rendering concern -- each toolkit handles it differently (CSS animation, render loop, frame callbacks). A static `IconData` cannot represent animation. | Return `None` for `StatusLoading` from SF Symbols and Windows. Document that loading indicators should use the toolkit's native spinner widget. Bundled Material/Lucide provide a static spinner icon as a fallback. |
| Multi-color / layered icon rendering | SF Symbols multicolor mode and Segoe Fluent Icons color layers provide full-color icon rendering. Supporting this requires platform-specific rendering contexts and does not map to a simple RGBA buffer. | Render all icons as monochrome/template images. Document that tinting with foreground color produces correct results. Multi-color support can be explored in a future version. |
| Variable font axis control for Material Symbols | Material Symbols variable font supports fill, weight, grade, and optical size axes. Exposing these parameters adds complexity to the `load_icon()` API for a feature that only applies to one icon set. | Bundle static SVGs (Outlined style, default weight). Users who need variable axis control can use the Material Symbols font directly. |
| Custom icon registration / user-defined roles | Allowing users to register custom `IconRole` values or override icon mappings adds complexity without clear demand. The 42 roles cover standard UI needs. | The `icon_name()` function returns the platform-specific string, which users can use to build their own lookup for additional icons outside the `IconRole` enum. |
| High-resolution icon atlas / spritesheet | Some games and performance-critical apps batch icons into texture atlases. This is an optimization specific to GPU-accelerated rendering pipelines. | Return individual `IconData` per role. Connectors can batch into atlases if their toolkit supports it. |
| Windows font glyph rendering via DirectWrite | DirectWrite provides higher quality text rendering than GDI, but adds a significant dependency chain (DirectWrite + Direct2D COM interfaces). The icon font glyphs are simple shapes where GDI quality is adequate. | Start with GDI-based font rendering. If quality complaints arise, add DirectWrite as a future enhancement. |

---

## Feature Dependencies

```
[IconRole enum + icon_name() lookup]
    independent -- pure data, no I/O, implement first
    blocks -> [every load_icon implementation]
    blocks -> [connector icon helpers]

[icon_theme field on ThemeVariant]
    independent (non-breaking, #[non_exhaustive])
    blocks -> [preset TOML updates]
    blocks -> [system_icon_set() function]

[IconData return type]
    independent -- simple enum definition
    blocks -> [every load_icon implementation]
    blocks -> [connector icon helpers]
    blocks -> [SVG rendering utility]

[Bundled Material SVGs]
    requires -> [IconRole enum]
    requires -> [IconData type]
    blocks -> [fallback path in load_icon()]
    ~21KB binary impact (feature-gated)

[Bundled Lucide SVGs]
    requires -> [IconRole enum]
    requires -> [IconData type]
    ~15KB binary impact (feature-gated, not default)

[Freedesktop icon lookup]
    requires -> [IconRole enum]
    requires -> [IconData type]
    platform: Linux only (cfg(target_os = "linux"))
    external dep: freedesktop-icons crate (0.4.0)

[macOS SF Symbols loading]
    requires -> [IconRole enum]
    requires -> [IconData type]
    platform: macOS only (cfg(target_os = "macos"))
    extends existing objc2-app-kit dependency
    needs new features: NSImage, NSBitmapImageRep, objc2-core-graphics

[Windows icon loading]
    requires -> [IconRole enum]
    requires -> [IconData type]
    platform: Windows only (cfg(target_os = "windows"))
    extends existing windows crate dependency
    needs new features: Win32_UI_Shell, Win32_Graphics_Gdi

[SVG rendering utility]
    requires -> [IconData type]
    optional feature (svg-render)
    external deps: resvg, tiny-skia

[Connector icon helpers]
    requires -> [IconData type]
    requires -> [icon_theme field]
    depends on connector crates already existing

[load_icon() dispatch function]
    requires -> ALL platform loaders
    requires -> [Bundled Material SVGs] (fallback path)
    THIS IS THE INTEGRATION POINT
```

**Critical path:** IconRole + IconData -> Bundled Material SVGs -> Platform loaders (parallel) -> load_icon() dispatch -> Connector helpers

**Parallelizable work:** All three platform loaders (macOS, Windows, Linux) can be developed in parallel once IconRole and IconData are defined. Bundled SVGs can be prepared in parallel with platform loaders.

---

## Sizing and DPI Behavior

Each platform icon system handles sizing differently. The `load_icon()` `size` parameter means "desired pixel size" and each platform backend interprets it appropriately:

| Platform | Behavior | Available sizes | DPI handling |
|----------|----------|----------------|--------------|
| **SF Symbols** | Renders at requested point size via `NSImageSymbolConfiguration`. Size is precise. | Any size (vector-based) | Caller multiplies by scale factor. SF Symbols auto-adapt to weight/scale. |
| **Windows stock** | `SHGetStockIconInfo` returns fixed sizes: `SHGSI_SMALLICON` (16x16) or `SHGSI_LARGEICON` (32x32), scaled by system DPI. | 16, 32 (at 96 DPI); scaled at higher DPI | System DPI scaling is automatic. May not match exact requested size. |
| **Segoe Fluent** | Font rendering at requested pixel size. Optimal at 16, 20, 24, 32, 40, 48, 64. | Any size (font-based) | Caller provides pixel size. |
| **freedesktop** | `index.theme` defines available directories (e.g., `16x16`, `24x24`, `scalable`). Lookup finds best match. SVGs scale to any size. | Theme-dependent. Typical: 16, 22, 24, 32, 48, scalable | Scale parameter supported by `freedesktop-icons` crate. |
| **Material SVGs** | Bundled SVGs are resolution-independent. Render at any size. | Any size (SVG) | N/A -- SVG |
| **Lucide SVGs** | Bundled SVGs are resolution-independent. Stroke-width scales with viewport. | Any size (SVG) | N/A -- SVG |

**Design decision:** The `size` parameter in `load_icon()` is best-effort. The returned `IconData::Rgba` may not exactly match the requested size (Windows stock icons). Callers should check `width`/`height` fields and scale if needed. For `IconData::Svg`, size is informational only (used by freedesktop lookup to pick the best directory).

---

## Icon Coloring Model

All platform icon systems converge on the same coloring model: monochrome template icons tinted with the current foreground color.

| Platform | Coloring mechanism | native-theme approach |
|----------|-------------------|----------------------|
| **SF Symbols** | Template mode by default. `NSImage.isTemplate = true`. AppKit tints with foreground color during rendering. | Return black RGBA pixels. Connector tints with `colors.foreground`. |
| **Segoe Fluent** | Font glyphs rendered in specified text color. Typically matches foreground. | Render glyphs in black. Connector tints with `colors.foreground`. |
| **freedesktop -symbolic** | SVGs use CSS classes (`foreground-fill`, `foreground-stroke`). GTK replaces at load time. | Return raw SVG bytes. Connector applies `fill` attribute or CSS. |
| **Material SVGs** | Static SVGs with `fill="black"` or `fill="currentColor"`. | Return raw SVG bytes. Connector sets `currentColor` or applies fill. |
| **Lucide SVGs** | Stroke-based SVGs with `stroke="currentColor"`. | Return raw SVG bytes. Connector sets `currentColor` or applies stroke color. |

**Unified approach:** All `IconData` should be treated as monochrome templates. For RGBA, multiply each pixel's RGB by the theme foreground color (preserving alpha). For SVG, replace fill/stroke with the foreground color or set `currentColor`. This convention is documented, not enforced in the API.

---

## MVP Recommendation

### Must ship (core value of icon loading):

1. **IconRole enum + icon_name()** -- the foundational abstraction. Zero dependencies, zero risk.
2. **IconData return type** -- needed by everything downstream.
3. **icon_theme field on ThemeVariant** -- connects icons to the preset system.
4. **Bundled Material SVGs** -- universal fallback ensuring `load_icon()` never returns `None` for any role on any platform.
5. **Freedesktop icon lookup** -- Linux icons are the easiest platform to implement (file I/O, no FFI) and serve the largest segment of the Rust GUI community.

### Should ship (platform coverage):

6. **macOS SF Symbols loading** -- completes the macOS story. Complex but high value.
7. **Windows icon loading** -- completes the Windows story. Two separate pipelines (stock + font) add complexity.
8. **Bundled Lucide SVGs** -- low effort, high value for gpui connector.

### Stretch (ship if time permits):

9. **Connector icon helpers** -- low complexity per connector, but depends on all platform loaders being done.
10. **SVG rendering utility** -- optional, most toolkits handle SVGs natively.

---

## Complexity Assessment Summary

| Feature | Complexity | LOC Estimate | Risk |
|---------|-----------|-------------|------|
| IconRole enum + icon_name() | LOW | ~300 (enum + match tables) | None |
| icon_theme on ThemeVariant | LOW | ~30 (field) + ~50 (preset updates) | None |
| IconData type | LOW | ~15 | None |
| Bundled Material SVGs | MEDIUM | ~100 (module) + SVG files | Low -- file selection and licensing |
| Bundled Lucide SVGs | LOW | ~80 (module) + SVG files | None |
| Freedesktop lookup | MEDIUM | ~150 (wrapper around freedesktop-icons crate) | Low -- well-tested dependency |
| macOS SF Symbols | HIGH | ~250-350 (FFI, rasterization pipeline) | Medium -- complex unsafe code, multiple image conversions |
| Windows stock icons | HIGH | ~200-300 (FFI, HICON -> RGBA conversion, GDI cleanup) | Medium -- GDI pixel format conversion, resource management |
| Windows font glyphs | MEDIUM | ~150-200 (font loading, glyph rendering) | Medium -- font rendering quality |
| SVG rendering utility | MEDIUM | ~50 (thin wrapper around resvg) | Low -- resvg is mature |
| Connector helpers (gpui) | LOW | ~80 | None |
| Connector helpers (iced) | LOW | ~50 | None |
| load_icon() dispatch | LOW | ~100 (cfg-gated match) | None |

**Total new code estimate:** ~1,200-1,700 lines (excluding bundled SVG files)

---

## Sources

- [freedesktop Icon Theme Specification](https://specifications.freedesktop.org/icon-theme/latest/) -- directory layout, index.theme format, lookup algorithm (HIGH confidence)
- [freedesktop Icon Naming Specification](https://specifications.freedesktop.org/icon-naming-spec/latest/) -- standard icon names (HIGH confidence)
- [freedesktop-icons crate (v0.4.0)](https://lib.rs/crates/freedesktop-icons) -- Rust implementation, 176K downloads/month (HIGH confidence)
- [icon-loader crate (v0.4.1)](https://lib.rs/crates/icon-loader) -- alternative Rust icon loader, Linux focus (MEDIUM confidence)
- [GTK 4 Symbolic Icons](https://docs.gtk.org/gtk4/icon-format.html) -- SVG recoloring mechanism, style classes (HIGH confidence)
- [Apple SF Symbols](https://developer.apple.com/sf-symbols/) -- 6,900+ symbols, macOS 11+ (HIGH confidence)
- [NSImage.SymbolConfiguration](https://developer.apple.com/documentation/appkit/nsimage/symbolconfiguration) -- pointSize, weight, scale parameters (HIGH confidence)
- [NSImage imageWithSystemSymbolName](https://developer.apple.com/documentation/appkit/nsimage/3622472-imagewithsystemsymbolname) -- SF Symbol initialization (HIGH confidence)
- [objc2-app-kit NSImage docs](https://docs.rs/objc2-app-kit/latest/objc2_app_kit/struct.NSImage.html) -- Rust bindings, feature flags for NSImage methods (HIGH confidence)
- [SHGetStockIconInfo (Win32)](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetstockiconinfo) -- stock icon retrieval (HIGH confidence)
- [Segoe Fluent Icons font](https://learn.microsoft.com/en-us/windows/apps/design/style/segoe-fluent-icons-font) -- codepoints, optimal sizes (HIGH confidence)
- [HICON to pixels Rust discussion](https://users.rust-lang.org/t/how-to-convert-hicon-to-png/90975) -- GetIconInfo + GetDIBits approach (MEDIUM confidence)
- [GetDIBits (Win32)](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/fn.GetDIBits.html) -- pixel extraction from HBITMAP (HIGH confidence)
- [Material Design Icons GitHub](https://github.com/google/material-design-icons) -- SVG source, Apache 2.0 license (HIGH confidence)
- [Lucide Icons](https://lucide.dev/) -- ISC license, 1,700+ icons (HIGH confidence)
- [resvg SVG renderer](https://github.com/linebender/resvg) -- static SVG rendering, tiny-skia backend (HIGH confidence)
- [Extract RGBA from NSImage (Swift)](https://gist.github.com/figgleforth/b5b193c3379b3f048210) -- CGImage -> bitmap pipeline reference (MEDIUM confidence)
- [NSBitmapImageRep docs](https://developer.apple.com/documentation/appkit/nsbitmapimagerep) -- bitmapData pixel access (HIGH confidence)

---
*Feature research for: native-theme icon loading milestone (v0.3)*
*Researched: 2026-03-09*
