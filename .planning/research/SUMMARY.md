# Project Research Summary

**Project:** native-theme v0.3
**Domain:** Cross-platform native icon loading for a toolkit-agnostic Rust theme crate
**Researched:** 2026-03-09
**Confidence:** HIGH

## Executive Summary

native-theme v0.3 adds platform-native icon loading to a shipping theme crate (~7,000 LOC, 17 presets, 4 platform readers, 2 connector crates). The icon system introduces an `IconRole` enum with 42 semantic roles, an `IconData` return type (SVG bytes or RGBA pixels), and three loading pipelines: macOS SF Symbols via `NSImage(systemSymbolName:)` rasterized to RGBA through CGBitmapContext, Windows stock icons via `SHGetStockIconInfo` with HICON-to-RGBA conversion plus Segoe Fluent Icons font glyph rendering, and Linux freedesktop icon theme lookup via the `freedesktop-icons 0.4` crate. Two bundled SVG icon sets (Material Symbols and Lucide, ~42 SVGs each at ~20-40KB total) provide cross-platform fallback when native APIs are unavailable. The only new runtime dependencies are `freedesktop-icons 0.4` (Linux, optional) and `resvg 0.47` (optional SVG rasterization utility); macOS and Windows extend existing `objc2-app-kit` and `windows` crate dependencies with additional feature flags.

The recommended approach follows a four-phase build order driven by the dependency graph: (1) data model and name mapping tables first, since every downstream component depends on `IconRole`, `IconData`, and the icon theme integration with `ThemeVariant`; (2) bundled SVG fallback icons next, providing the universal fallback chain that platform loaders need when a native icon is unavailable; (3) platform-native loading pipelines in parallel (macOS, Windows, Linux are independent); (4) connector integration last, once the core icon API is stable. This ordering mirrors v0.2's proven pattern of "data model before consumers, core before connectors."

The key risks are: (1) macOS NSImage rasterization producing wrong-sized images on Retina displays without explicit pixel-dimension control -- silent correctness bug; (2) Windows HICON-to-RGBA conversion dropping alpha or producing wrong colors due to premultiplied alpha and BGRA byte order; (3) freedesktop icon lookup silently failing on non-Adwaita themes due to incomplete inheritance chain traversal; (4) SF Symbols and Segoe Fluent Icons font files being proprietary and non-redistributable -- the crate must load them at runtime from the OS, never bundle them; (5) NSImage thread safety issues causing crashes under concurrent access. All five have concrete prevention strategies: the `freedesktop-icons` crate handles (3), RAII wrappers and conversion functions handle (2), explicit pixel-dimension bitmap creation handles (1), and project rules established during API design handle (4).

## Key Findings

### Recommended Stack

v0.3 adds minimal new dependencies. The core insight is that macOS and Windows icon loading extend *existing* crate dependencies with new feature flags rather than adding new crates. Only Linux needs a genuinely new dependency (`freedesktop-icons`). An optional SVG rasterization utility adds `resvg` behind a feature flag.

**Core technologies:**
- **objc2-app-kit 0.3 (existing dep, new features):** Add `NSImage`, `NSImageRep`, `NSBitmapImageRep`, `NSGraphicsContext`, `objc2-core-graphics` features for SF Symbols loading and CGBitmapContext-based RGBA extraction
- **windows crate (existing dep, new feature):** Add `Win32_UI_Shell` for `SHGetStockIconInfo`; existing `Win32_Graphics_Gdi` already covers HICON-to-RGBA conversion
- **freedesktop-icons 0.4.0 (new, optional):** Freedesktop Icon Theme Specification lookup with inheritance, hicolor fallback, size matching, and caching -- small dependency tree (dirs, ini_core, xdg)
- **resvg 0.47 (new, optional):** Pure Rust SVG-to-RGBA rasterization for consumers without their own SVG renderer -- behind `svg-rasterize` feature, not default
- **Material Symbols + Lucide SVGs (compile-time, no runtime dep):** ~42 SVGs each bundled via `include_bytes!()`, ~20-40KB per set

**Critical version requirements:**
- freedesktop-icons must be 0.4+ (builder API with `.with_cache()`, `.force_svg()`)
- resvg 0.47 uses usvg 0.47 + tiny-skia 0.12 (latest as of January 2026)
- objc2-app-kit 0.3.2 unchanged from v0.2; `objc2-core-graphics` is a transitive feature, not a new crate

### Expected Features

**Must have (table stakes):**
- **IconRole enum + icon_name() lookup** -- the foundational abstraction: 42 variants, 5 icon set mapping functions, zero dependencies
- **IconData return type** -- `Svg(Vec<u8>)` for Linux/bundled icons, `Rgba { width, height, data }` for macOS/Windows rasterized icons
- **icon_theme field on ThemeVariant** -- connects the icon system to the preset/theme system; `Option<String>` with `skip_serializing_if`
- **Bundled Material SVGs** -- universal fallback ensuring `load_icon()` never returns `None` for any role on any platform
- **Freedesktop icon lookup (Linux)** -- easiest platform to implement (file I/O, no FFI), largest Rust GUI audience segment
- **macOS SF Symbols loading** -- completes the macOS platform story with real Apple icons
- **Windows icon loading** -- two pipelines: SHGetStockIconInfo for stock icons (~18 roles), Segoe Fluent font for UI actions (~24 roles)

**Should have (differentiators):**
- **Bundled Lucide SVGs** -- gpui-component already bundles 87 Lucide icons; the connector can short-circuit to `IconName` for 27 of 42 roles with zero I/O
- **Connector icon helpers** -- `icon_name_for_role()` (gpui Lucide shortcut) and `load_icon()` wrappers converting `IconData` to toolkit image types
- **SVG rasterization utility** -- `svg-rasterize` feature wrapping resvg for consumers without SVG support

**Defer (v0.4+):**
- Animated icon support (spinners, loading indicators) -- each toolkit handles animation differently
- Multi-color / layered SF Symbol rendering -- requires platform-specific drawing contexts
- Variable font axis control for Material Symbols -- over-engineering for v0.3
- Custom icon registration / user-defined roles -- 42 roles cover standard UI needs
- DirectWrite-based Segoe Fluent rendering -- GDI is adequate for monochrome font glyphs; upgrade if quality complaints arise
- Icon caching in the core crate -- caching policy is application-specific

### Architecture Approach

The icon system integrates as a new `icons/` module in the core crate, following the established pattern of data model types + platform implementations behind `#[cfg]` + fallback assets. `load_icon()` is a free function (not a method on `ThemeVariant`) that dispatches to platform loaders by icon theme string, with a fallback chain to bundled SVGs. Each platform loader is a thin wrapper: call the OS API, convert the result to `IconData`, return. Name mapping uses exhaustive `match` statements (42 arms per icon set) -- no traits, no HashMaps, no phf.

**Major components:**
1. **`icons/mod.rs`** -- Public API: `load_icon()`, `icon_name()`, `system_icon_set()`, `IconRole`, `IconData` types
2. **`icons/names.rs`** -- Five pure functions mapping `IconRole` to platform-specific identifier strings (`sf_symbols_name`, `freedesktop_name`, `segoe_fluent_name`, `material_name`, `lucide_name`)
3. **`icons/bundled.rs`** -- `include_bytes!()` for 42 Material + 42 Lucide SVGs with `match`-based lookup
4. **`icons/freedesktop.rs`** -- Linux: `freedesktop-icons` crate lookup, reads SVG bytes from disk
5. **`icons/macos.rs`** -- macOS: `NSImage(systemSymbolName:)` + CGBitmapContext rasterization to RGBA
6. **`icons/windows.rs`** -- Windows: `SHGetStockIconInfo` HICON extraction + Segoe Fluent font glyph GDI rendering
7. **`model/mod.rs`** (modified) -- `icon_theme: Option<String>` added to `ThemeVariant`

### Critical Pitfalls

1. **SF Symbols / Segoe Fluent licensing** -- Name strings are API identifiers and safe to ship. Font files, glyph outlines, and rendered images are proprietary and must never be bundled. Load at runtime from the OS only. Establish this as a project rule before any implementation.
2. **HICON-to-RGBA conversion** -- Windows GDI returns BGRA with premultiplied alpha. Must swap B/R channels, un-premultiply alpha, and handle the edge case where 32bpp icons have all-zero alpha (use the AND mask instead). Create RAII wrappers for HICON/HBITMAP to prevent GDI handle leaks.
3. **NSImage rasterization on Retina** -- SF Symbols are resolution-independent. Without explicit pixel-dimension control, a 24-point request produces 48 pixels on 2x displays. Create `NSBitmapImageRep` at exact pixel dimensions. Additionally, NSImage is not thread-safe for mutation -- serialize all icon loading on macOS.
4. **freedesktop lookup edge cases** -- Theme inheritance chains, hicolor ultimate fallback, three directory types (Fixed/Scalable/Threshold), and XDG_DATA_DIRS compliance. Use the `freedesktop-icons` crate rather than implementing from scratch. Test with Adwaita, Breeze, Papirus, and hicolor-only.
5. **Feature flag combinations** -- 9+ features create 512 theoretical combinations. Test the important subset: default, each icon flag alone, all flags, no flags. Ensure `load_icon` returns `None` gracefully when no icon features are enabled.

## Implications for Roadmap

Based on the dependency graph across all four research files, the icon system has a clear critical path: `IconRole + IconData` (blocks everything) -> bundled SVGs (provides fallback chain) -> platform loaders (parallel, independent) -> connectors (consumes stable API). This mirrors v0.2's proven "data model before consumers" principle.

### Phase 1: Data Model and Icon Name Mapping
**Rationale:** Everything else depends on `IconRole`, `IconData`, and the name mapping tables. This phase has zero new dependencies, zero FFI, and can be fully tested on any platform. It also establishes the licensing boundary (project rule: never bundle proprietary icon assets).
**Delivers:** `IconRole` enum (42 variants), `IconData` enum, `icon_name()` with 5 mapping functions (210 match arms total), `system_icon_set()`, `icon_theme: Option<String>` on `ThemeVariant` with merge/is_empty/serde support, preset TOML updates for 6 native presets, lib.rs re-exports.
**Addresses:** Features 1-3, 7 from FEATURES.md (IconRole, IconData, icon_theme, API functions)
**Avoids:** Pitfall 1 (SF Symbols licensing -- establish project rule), Pitfall 2 (Segoe Fluent licensing), Pitfall 8 (Option vs Result -- decide error strategy), Pitfall 9 (feature flag design)
**Estimated LOC:** ~400

### Phase 2: Bundled SVG Icon Sets
**Rationale:** Bundled icons provide the universal fallback chain. Platform loaders in Phase 3 need this fallback when a native icon is unavailable (e.g., `trash-full` on macOS). Having fallbacks working first means Phase 3 can focus purely on platform FFI. This phase also has no FFI and no platform-specific code.
**Delivers:** ~42 SVGO-optimized Material Symbols SVGs, ~42 Lucide SVGs, `bundled.rs` with `include_bytes!` + match-based lookup, `material-icons` and `lucide-icons` feature flags, `load_icon()` dispatch function with fallback chain.
**Addresses:** Features 3, 8 from FEATURES.md (bundled Material, bundled Lucide)
**Avoids:** Pitfall 4 (SVG bundle size -- budget 200KB max for Material, 100KB for Lucide, verify with `cargo bloat`)
**Estimated LOC:** ~200 (plus ~84 SVG asset files)

### Phase 3: Platform-Native Icon Loading
**Rationale:** All three platform loaders are independent and can be developed in parallel once the data model and fallback chain are stable. Each adds cfg-gated code and extends existing dependencies. This is the highest-risk phase due to unsafe FFI code and platform-specific pixel format handling.
**Delivers:** Three platform loaders producing `IconData` from native APIs.

**Sub-phase 3a: Linux/freedesktop**
- Uses: `freedesktop-icons 0.4` crate (new optional dependency)
- Implements: `icons/freedesktop.rs` -- lookup by icon name, read SVG file, return `IconData::Svg`
- Risk: LOW -- the crate handles spec compliance; wrapper is ~50 lines

**Sub-phase 3b: macOS SF Symbols**
- Uses: `objc2-app-kit` with new `NSImage`, `objc2-core-graphics` features
- Implements: `icons/macos.rs` -- NSImage(systemSymbolName:) + CGBitmapContext rasterization
- Risk: MEDIUM -- unsafe ObjC FFI, pixel format control, Retina size handling, thread safety

**Sub-phase 3c: Windows stock icons + Segoe Fluent glyphs**
- Uses: `windows` crate with new `Win32_UI_Shell` feature
- Implements: `icons/windows.rs` -- SHGetStockIconInfo HICON extraction + GDI font glyph rendering
- Risk: MEDIUM -- BGRA-to-RGBA conversion, premultiplied alpha, GDI handle cleanup, COM initialization

**Addresses:** Features 4-6 from FEATURES.md (freedesktop, SF Symbols, Windows icons)
**Avoids:** Pitfall 3 (freedesktop lookup -- use crate), Pitfall 5 (HICON conversion -- RAII wrappers, correct alpha), Pitfall 6 (NSImage Retina -- explicit pixel dimensions), Pitfall 7 (cross-compilation -- multi-target CI), Pitfall 10 (COM init + handle leaks)
**Estimated LOC:** ~600-900 total across three sub-phases

### Phase 4: Integration and Connectors
**Rationale:** Connectors consume the stable core API. The gpui connector has a valuable fast path: for `icon_theme = "lucide"`, it maps `IconRole` directly to gpui-component's built-in `IconName` enum (27 of 42 roles) with zero I/O. The iced connector converts `IconData` to `iced::widget::image::Handle`. Optional SVG rasterization utility (`svg-rasterize` feature with resvg) also lands here.
**Delivers:** `native-theme-gpui` icons module (Lucide shortcut + IconData conversion), `native-theme-iced` icons module, optional `rasterize_svg()` utility, documentation and examples.
**Addresses:** Features 9-11 from FEATURES.md (connector helpers, SVG rasterization, documentation)
**Avoids:** Anti-pattern of putting platform logic in connectors -- connectors are pure format converters
**Estimated LOC:** ~200-300

### Phase Ordering Rationale

- **Phase 1 before Phase 2:** Bundled SVGs need `IconRole` for match dispatch and `IconData` for return type. The `load_icon()` function needs both plus the fallback chain.
- **Phase 2 before Phase 3:** Platform loaders fall back to bundled SVGs when a native icon is unavailable. Without the fallback chain, platform loaders would need placeholder logic that gets replaced later.
- **Phase 3 platforms are independent:** macOS, Windows, and Linux share no code paths. Each adds its own cfg-gated module. Can be developed and CI-tested in any order.
- **Phase 4 last:** Connectors depend on the complete and stable core icon API. Building them before the API is finalized means rework.
- **Total estimated LOC:** 1,200-1,700 lines of Rust (excluding SVG asset files), consistent with FEATURES.md complexity assessment.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 3b (macOS SF Symbols):** NSImageSymbolConfiguration feature flag availability in objc2-app-kit needs verification. CGBitmapContext approach for controlled RGBA output is documented but not yet prototyped in this codebase. Thread safety constraints may require architectural decisions (serial dispatch queue or main-thread requirement).
- **Phase 3c (Windows Segoe Fluent):** GDI-based font glyph rendering (CreateFont + TextOut into DIB section) has not been verified with the `windows` crate bindings for this specific use case. If quality is insufficient, DirectWrite fallback adds significant complexity (3 more `windows` crate features). Segoe Fluent font presence on Windows 10 needs graceful detection.

Phases with standard patterns (skip deeper research):
- **Phase 1 (Data model):** Pure Rust enums and match tables. Follows exact pattern of existing `ThemeVariant` fields. Mechanical work.
- **Phase 2 (Bundled SVGs):** Standard `include_bytes!` pattern. Download SVGs, optimize with SVGO, add to source tree. No unknowns.
- **Phase 3a (Linux/freedesktop):** The `freedesktop-icons` crate provides a clean builder API. The wrapper is ~50 lines. Well-documented spec.
- **Phase 4 (Connectors):** Thin mapping layers following the established connector pattern from v0.2.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All crate versions verified on docs.rs/lib.rs with publication dates. Dependency chains confirmed. freedesktop-icons 0.4.0 (April 2025), resvg 0.47.0 (January 2026). Existing deps need only feature flag additions. |
| Features | HIGH | 42 IconRole variants mapped against 5 icon sets with availability matrix verified. Feature dependency graph has clear critical path. Complexity estimates grounded in existing codebase patterns. |
| Architecture | HIGH | Based on full source analysis of v0.2 codebase. Build order follows proven v0.2 pattern. Component boundaries map cleanly to the existing module structure. No architectural novelty -- same patterns at work. |
| Pitfalls | HIGH | 10 pitfalls identified with verified sources (Apple docs, Microsoft Learn, freedesktop spec, Rust community discussions, compiler issue tracker). All have concrete prevention strategies and phase assignments. Recovery costs assessed for each. |

**Overall confidence:** HIGH

### Gaps to Address

- **NSImageSymbolConfiguration feature gate:** Whether objc2-app-kit gates this class under the `NSImage` feature or a separate feature flag needs verification during Phase 3b implementation. If separate, one additional feature flag is needed in Cargo.toml.
- **Windows DirectWrite fallback complexity:** If GDI-based Segoe Fluent glyph rendering produces insufficient quality (fuzzy icons, missing hinting), the DirectWrite path requires 3 additional `windows` crate features (`Win32_Graphics_DirectWrite`, `Win32_Graphics_Direct2D`, `Win32_Graphics_Imaging`) and a significantly more complex COM-based rendering pipeline. Decision can be deferred to after initial GDI prototype.
- **Segoe Fluent on Windows 10:** The font may not be present on all Windows 10 installations. The fallback chain (stock icons -> bundled Material SVGs) handles this gracefully, but the detection logic needs implementation.
- **NSImage thread safety:** If `load_icon` must be called from multiple threads on macOS, a serialization mechanism (serial dispatch queue or Mutex around NSImage operations) is needed. This may affect the public API contract (document thread safety requirements).
- **Premultiplied-to-straight alpha conversion correctness:** The RGBA output from both macOS (CGBitmapContext with premultiplied alpha) and Windows (GDI premultiplied BGRA) must be un-premultiplied. Division-by-alpha with rounding edge cases at very low alpha values needs careful implementation to avoid artifacts.
- **PNG icons from freedesktop themes:** The research recommends `force_svg()` on freedesktop lookups, but some themes only provide PNG icons for certain roles. The current design returns `None` for PNG-only icons. Consider whether to add a PNG reading path (would require the `image` crate or `png` crate) or document this limitation.

## Sources

### Primary (HIGH confidence)
- [docs.rs/objc2-app-kit NSImage](https://docs.rs/objc2-app-kit/latest/objc2_app_kit/struct.NSImage.html) -- imageWithSystemSymbolName, CGImageForProposedRect methods
- [lib.rs/objc2-app-kit features](https://lib.rs/crates/objc2-app-kit/features) -- NSImage, NSImageRep, NSBitmapImageRep, NSGraphicsContext, objc2-core-graphics feature flags
- [docs.rs/objc2-core-graphics](https://docs.rs/objc2-core-graphics/latest/objc2_core_graphics/) -- CGImage, CGBitmapContext functions
- [microsoft.github.io/windows-docs-rs SHGetStockIconInfo](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/fn.SHGetStockIconInfo.html) -- Win32::UI::Shell function
- [microsoft.github.io/windows-docs-rs GetDIBits](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/fn.GetDIBits.html) -- Win32::Graphics::Gdi pixel extraction
- [lib.rs/freedesktop-icons 0.4.0](https://lib.rs/crates/freedesktop-icons) -- builder API with theme/size/scale/cache
- [docs.rs/freedesktop-icons](https://docs.rs/freedesktop-icons/latest/freedesktop_icons/) -- API: lookup().with_theme().with_size().find()
- [docs.rs/resvg 0.47](https://docs.rs/resvg/latest/resvg/) -- render() + tiny-skia Pixmap for RGBA output
- [freedesktop Icon Theme Specification](https://specifications.freedesktop.org/icon-theme/latest/) -- lookup algorithm, inheritance, directory types
- [Apple SF Symbols](https://developer.apple.com/sf-symbols/) -- licensing, availability (macOS 11+)
- [Apple NSImage.SymbolConfiguration](https://developer.apple.com/documentation/appkit/nsimage/symbolconfiguration) -- pointSize, weight, scale parameters
- [SHGetStockIconInfo (Win32)](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetstockiconinfo) -- handle ownership, DestroyIcon requirement
- [Segoe Fluent Icons font](https://learn.microsoft.com/en-us/windows/apps/design/style/segoe-fluent-icons-font) -- codepoints, EULA restrictions
- [Material Design Icons (GitHub)](https://github.com/google/material-design-icons) -- Apache 2.0 license, SVG source
- [Lucide Icons](https://lucide.dev/) -- ISC license, 1,700+ icons

### Secondary (MEDIUM confidence)
- [docs.rs/swash 0.2.6](https://docs.rs/swash/latest/swash/) -- Content::Mask/Color for glyph images (backup for font rendering)
- [HICON to PNG in Rust](https://users.rust-lang.org/t/how-to-convert-hicon-to-png/90975) -- BGRA byte order, GetDIBits, alpha handling
- [NSImage to RGBA pixels (gist)](https://gist.github.com/figgleforth/b5b193c3379b3f048210) -- CGBitmapContext approach
- [NSImage is dangerous](https://wadetregaskis.com/nsimage-is-dangerous/) -- thread safety analysis
- [include_bytes! performance (rust-lang/rust#65818)](https://github.com/rust-lang/rust/issues/65818) -- compiler regression with large blobs
- [Windows Premultiplied Alpha (Win2D)](https://learn.microsoft.com/en-us/windows/apps/develop/win2d/premultiplied-alpha) -- straight vs premultiplied conversion

### Tertiary (LOW confidence)
- GDI-based Segoe Fluent font glyph rendering quality -- not prototyped; may need DirectWrite upgrade
- swash 0.2.6 as fallback font renderer -- evaluated but deferred; needs validation if DirectWrite/GDI paths prove inadequate

---
*Research completed: 2026-03-09*
*Ready for roadmap: yes*
