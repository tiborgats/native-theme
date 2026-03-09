# Pitfalls Research

**Domain:** Adding cross-platform native icon loading to an existing Rust theme crate
**Researched:** 2026-03-09
**Confidence:** HIGH (verified via official docs, platform specifications, Rust crate analysis, and community post-mortems)

This document covers pitfalls specific to v0.3 icon loading features: SF Symbols loading on macOS, SHGetStockIconInfo / Segoe Fluent Icons on Windows, freedesktop icon theme lookup on Linux, SVG bundling for Material/Lucide fallback sets, cross-platform API design, and feature flag architecture.

---

## Critical Pitfalls

### Pitfall 1: SF Symbols Name Strings Are Not Redistributable as Font Data

**What goes wrong:**
Developers assume that because SF Symbol *name strings* (like `"exclamationmark.triangle.fill"`) are just text, they can be freely redistributed. The name strings themselves are fine to include in source code -- they are effectively API identifiers, similar to Win32 constant names. However, the SF Symbols *font files*, *glyph outlines*, and *rendered images* are Apple proprietary and cannot be redistributed. The danger is scope creep: starting with name-string lookup (safe) and gradually adding bundled SVG copies of SF Symbol glyphs for offline use or cross-platform fallback (violation).

A secondary risk: some SF Symbols are marked with restriction annotations (visible in the SF Symbols app with a small "i" icon). These restricted symbols may only be used to represent specific Apple services (e.g., AirDrop, FaceTime, iCloud). Using restricted symbol names to load icons for generic purposes could violate Apple's terms even on macOS.

**Why it happens:**
The Apple EULA states: "All SF Symbols shall be considered to be system-provided images as defined in the Xcode and Apple SDKs license agreements." The key constraint is: you may not redistribute the symbols themselves, create lookalikes, or use them in trademarks. But storing a mapping table of `IconRole -> &str` name strings is analogous to storing Win32 API constant names -- it is referencing an API, not redistributing the asset. Developers conflate "name string" with "symbol" and either over-restrict (avoiding name strings unnecessarily) or under-restrict (bundling rasterized glyphs).

**How to avoid:**
1. The `icon_name()` function that returns `&'static str` SF Symbol names is safe to ship. These are API identifiers.
2. Never bundle rasterized SF Symbol images or SVG outlines derived from them.
3. The `load_icon()` function on macOS must call `NSImage(systemSymbolName:)` at runtime -- the symbol is loaded from the OS, not from the crate.
4. For cross-platform fallback when SF Symbols are unavailable (Linux, Windows), fall back to Material or Lucide -- never to copies of SF Symbol glyphs.
5. Audit any community-maintained "SF Symbol name lists" used as reference (e.g., `sam4096/apple-sf-symbols-list` on GitHub) -- the names are fine, but never copy glyph data from these repos.
6. Check for restriction annotations on each symbol used in the IconRole mapping. The symbols in the spec (`exclamationmark.triangle.fill`, `xmark.circle.fill`, etc.) are all general-purpose and unrestricted.

**Warning signs:**
- The crate's published package contains SVG or PNG files that visually resemble SF Symbols
- Tests embed rasterized SF Symbol data for comparison
- CI attempts to load SF Symbols on Linux (they are macOS/iOS only)

**Phase to address:**
Phase 1 (API design and mapping tables). Establish the legal boundary as a project rule before any implementation begins.

---

### Pitfall 2: Segoe Fluent Icons Font Cannot Be Redistributed or Loaded on Non-Windows

**What goes wrong:**
The spec calls for rendering Segoe Fluent Icons glyphs for UI action icons (save, copy, paste, etc.) on Windows. The EULA explicitly states: "you may use the font solely to design, develop and test your programs that run on a Microsoft Platform" and "this license does not grant you the right to distribute or sublicense all or part of the Software to any third party." Developers may attempt to: (a) bundle the font file with the crate for use on Linux/macOS, or (b) reference PUA codepoints on non-Windows where the font is absent, producing invisible rectangles.

A subtler issue: the codepoints are in the Unicode Private Use Area (PUA, U+E000-U+F8FF). Referencing PUA codepoints is fine (they are just numbers). But *rendering* them requires the font to be installed. On Windows 11 the font is always present. On Windows 10 it may not be. On Linux and macOS it is never present unless manually installed.

**Why it happens:**
The Segoe Fluent Icons font is proprietary Microsoft IP. Unlike Fluent UI System Icons (MIT licensed, available on GitHub), the Segoe Fluent Icons *font file* has restrictive licensing. Developers confuse "Fluent UI System Icons" (open source, redistributable) with "Segoe Fluent Icons" (proprietary, Windows-only).

**How to avoid:**
1. On Windows: load glyphs from the system-installed Segoe Fluent Icons font. Use `CreateFontW("Segoe Fluent Icons")` or DirectWrite to load the font, render the glyph at the requested size, and extract RGBA pixels.
2. On Windows 10 where the font may be absent: detect its absence gracefully and fall back to SHSTOCKICONID stock icons or bundled Material/Lucide SVGs.
3. Never bundle the Segoe Fluent Icons `.ttf` file in the crate.
4. For cross-platform use, the `icon_name()` function can return the glyph name string (e.g., "Save") and the codepoint (e.g., U+E74E) -- these are just identifiers. But `load_icon()` must only attempt font rendering on Windows.
5. Consider using Microsoft's open-source Fluent UI System Icons (MIT license, `microsoft/fluentui-system-icons` on GitHub) as a redistributable alternative for non-stock icons, instead of Segoe Fluent Icons glyph rendering.

**Warning signs:**
- `cargo package --list` shows a `.ttf` file
- Tests fail on Linux CI with "font not found" errors
- Icons render as empty rectangles on Windows 10

**Phase to address:**
Phase 1 (API design). Define the rendering strategy for Windows early: SHSTOCKICONID for stock icons, Segoe Fluent font for UI glyphs, with explicit fallback chain.

---

### Pitfall 3: freedesktop Icon Theme Lookup Has Multiple Subtle Failure Modes

**What goes wrong:**
The freedesktop Icon Theme Specification defines a multi-phase lookup algorithm with inheritance chains, three directory types (Fixed, Scalable, Threshold), and fallback to hicolor. Implementing it incorrectly causes icons to silently load from the wrong theme, at the wrong size, or not at all. The most common failures:

1. **Inheritance chain not followed:** A theme like Papirus inherits from hicolor. If the lookup stops at the user theme without traversing `Inherits`, icons present in hicolor but not Papirus will return None.
2. **hicolor fallback skipped:** The spec mandates that hicolor is always the last fallback, even if the theme's `Inherits` key does not mention it. Implementations that only traverse the declared inheritance chain miss this.
3. **Scalable directory size matching wrong:** Scalable directories have `MinSize` and `MaxSize`. A scalable directory with `Size=48, MinSize=1, MaxSize=256` matches ANY requested size. Implementations that only check exact size matches miss scalable icons entirely.
4. **Threshold directory matching wrong:** Threshold directories default to `Threshold=2`, meaning a size-48 directory matches requests for 46-50. Implementations that ignore this parameter fail to find icons.
5. **`-symbolic` suffix not stripped on fallback:** When looking up `dialog-warning-symbolic`, if not found, the spec says to strip `-symbolic` and retry. Implementations that skip this miss full-color icons that could serve as fallback.
6. **XDG_DATA_DIRS not respected:** The default is `/usr/local/share:/usr/share`. Some distros set this differently. Hardcoding `/usr/share/icons` misses icons installed to other prefixes (e.g., Flatpak, Nix).
7. **$HOME/.icons not checked:** User-installed themes go in `~/.icons` or `$XDG_DATA_HOME/icons`. Skipping this means custom themes are invisible.
8. **index.theme not parsed:** Each theme directory has an `index.theme` file defining subdirectories, their types, and size parameters. Loading icons by guessing directory structure (e.g., always using `{size}x{size}/`) instead of parsing index.theme produces wrong results for themes with non-standard layouts.

**Why it happens:**
The spec is deceptively simple in prose but has many edge cases. Most implementations start with "just look in `{size}x{size}/{context}/`" which works for Adwaita and Breeze but fails for themes with different directory structures. The three-phase lookup (exact match, closest match, unthemed fallback) is easy to short-circuit incorrectly.

**How to avoid:**
1. Use the `freedesktop-icons` crate (crates.io) instead of implementing from scratch. It handles inheritance, hicolor fallback, and caching correctly. Version 0.2+ supports the builder pattern with theme, size, and scale.
2. If implementing from scratch (to avoid the dependency), follow the spec's pseudocode exactly. The critical functions are `FindIcon`, `FindIconHelper`, `LookupIcon`, `DirectoryMatchesSize`, and `DirectorySizeDistance`.
3. Parse `index.theme` for every theme in the inheritance chain. Do not guess directory structure.
4. Always append hicolor to the inheritance chain even if not declared in `Inherits`.
5. Cache the parsed directory listings -- the spec explicitly recommends this and allows 5-second mtime staleness.
6. Test with at least three icon themes: Adwaita (GNOME), Breeze (KDE), and hicolor (minimal fallback).

**Warning signs:**
- Icons load from Adwaita but not from Papirus or Arc
- Scalable SVG icons never found even though they exist on disk
- Works on Ubuntu but fails on Fedora or Arch (different `XDG_DATA_DIRS`)
- `load_icon("freedesktop", DialogWarning, 24)` returns None even though `/usr/share/icons/Adwaita/scalable/status/dialog-warning-symbolic.svg` exists

**Phase to address:**
Phase 2 (Linux platform implementation). This is the most complex platform to implement correctly. Budget extra time and testing for it.

---

### Pitfall 4: SVG Bundling Balloons Crate Size and Compile Time

**What goes wrong:**
Material Symbols has 3,800+ icons. Even with only the ~42 icons needed for IconRole, bundling entire icon sets "for future extensibility" causes problems. The full Material Symbols SVG set is approximately 50-100 MB uncompressed. Even a curated subset of 42 icons at ~1-2 KB each is only ~84 KB -- manageable. But feature creep ("let's include all icons so users can use any Material icon") turns the crate into a multi-megabyte download that inflates compile time.

Specific Rust issues:
- `include_bytes!` on large blobs has a known compiler performance regression (rust-lang/rust#65818). The compiler iterates per-byte for metadata emission, causing non-trivial compile times for multi-MB includes.
- The `icondata` crate (which bundles 20,000+ icons across many libraries) is known to heavily slow down rust-analyzer when using wildcard imports.
- crates.io has a 10 MB package size limit. Bundling thousands of SVGs can hit this.

**Why it happens:**
The temptation to "include everything" is strong. Material Symbols has 3,800 icons -- why not ship them all? The answer: crate size, compile time, and the fact that 99% of users need fewer than 50 icons.

**How to avoid:**
1. Bundle ONLY the ~42 SVGs needed for the IconRole enum. These are the semantic roles the crate defines. Store them in a `icons/material/` or `icons/lucide/` directory.
2. Use `include_bytes!` for each individual SVG file, not `include_dir!` on a directory of thousands. This keeps the include list explicit and small.
3. For users who want more icons, provide a `load_icon_by_name()` function that loads from the freedesktop theme directories at runtime (Linux) or accepts a custom SVG directory path.
4. If compressed bundling is needed later, use `include_flate` or `include-bytes-zstd` to compress SVGs at compile time and decompress at runtime. SVGs compress very well (70-90% reduction).
5. Set a hard budget: Material icons feature adds no more than 200 KB to the binary. Lucide icons feature adds no more than 100 KB.
6. Measure with `cargo bloat --crates` after adding bundled icons to verify actual binary impact.

**Warning signs:**
- `cargo package --list` shows hundreds of SVG files
- Compile time increases by more than 2 seconds after adding icon features
- Binary size increases by more than 500 KB
- rust-analyzer becomes sluggish in the icon module

**Phase to address:**
Phase 3 (bundled icon sets). Establish the size budget before selecting which icons to bundle.

---

### Pitfall 5: HICON to RGBA Conversion Drops Alpha Channel or Produces Wrong Colors

**What goes wrong:**
Windows stock icons retrieved via `SHGetStockIconInfo(SHGSI_ICON)` return an `HICON` handle. Converting this to RGBA pixel data requires careful handling of several Windows GDI quirks:

1. **Premultiplied alpha vs straight alpha:** Windows GDI internally uses premultiplied alpha (R,G,B values are pre-multiplied by A). The crate's `IconData::Rgba` presumably expects straight alpha. Converting without un-premultiplying produces washed-out colors.
2. **32bpp icons without alpha:** Some stock icons are 32bpp but have no alpha channel (all alpha bytes are zero). `GetIconInfo` returns both `hbmColor` (color bitmap) and `hbmMask` (AND mask). If the alpha channel is all zeros, you must use the AND mask to determine transparency. If you treat zero-alpha as fully transparent, the icon is invisible.
3. **BGR to RGB byte order:** Windows bitmaps use BGRA byte order, not RGBA. Forgetting to swap B and R produces blue-tinted icons.
4. **GetDIBits alignment:** `GetDIBits` may produce misaligned data depending on the bitmap info header used. Using `BITMAPINFOHEADER` vs `BITMAPV5HEADER` changes alpha handling behavior.
5. **HICON handle leak:** `SHGetStockIconInfo` docs explicitly state: "you are responsible for freeing the icon with DestroyIcon." Forgetting to call `DestroyIcon` leaks GDI handles. Windows has a per-process limit of ~10,000 GDI objects.
6. **DPI mismatch:** `SHGetStockIconInfo` with `SHGSI_ICON | SHGSI_LARGEICON` returns icons at system DPI. On a 200% display, this may return 64x64 when you expected 32x32. The size is not controllable -- you get whatever Windows decides.

**Why it happens:**
Windows icon handling predates modern alpha-composited rendering. The API surface accumulated decades of backward compatibility layers. The "right" way to extract RGBA data from an HICON involves 6-8 API calls with careful parameter choices, and the official docs do not provide a complete code sample.

**How to avoid:**
1. Use `GetIconInfo` to get `hbmColor` and `hbmMask`.
2. Use `GetDIBits` with a `BITMAPINFOHEADER` set to `biBitCount = 32` and `biCompression = BI_RGB` to extract the color bitmap into a buffer.
3. Scan the alpha channel: if any byte is non-zero, the icon has per-pixel alpha. If all alpha bytes are zero, apply the AND mask to determine transparency.
4. Convert BGRA to RGBA (swap bytes 0 and 2 in each 4-byte pixel).
5. Un-premultiply alpha: for each pixel where A > 0, set `R = R * 255 / A`, `G = G * 255 / A`, `B = B * 255 / A`.
6. Always call `DestroyIcon` on the HICON after extraction. Use a wrapper type with `Drop` to guarantee cleanup.
7. Always call `DeleteObject` on `hbmColor` and `hbmMask` -- these are also caller-owned.
8. For Segoe Fluent Icons glyph rendering, use DirectWrite or GDI to render the glyph to a bitmap, then extract the same way.

**Warning signs:**
- Icons appear as solid black rectangles (alpha not handled)
- Icons appear with a blue tint (BGR not swapped to RGB)
- Icons appear washed out (premultiplied alpha not converted)
- GDI handle count grows over time (HICON leak)
- Icon size varies unpredictably across displays (DPI not accounted for)

**Phase to address:**
Phase 2 (Windows platform implementation). Write a dedicated `hicon_to_rgba()` utility function with comprehensive tests. Consider adapting the approach from the `muda` crate's `icon.rs` which handles this correctly.

---

### Pitfall 6: NSImage Rasterization Produces Wrong Size on Retina/HiDPI Without Explicit Scale Handling

**What goes wrong:**
On macOS, `NSImage(systemSymbolName:)` returns a resolution-independent image. When rasterizing to a bitmap for `IconData::Rgba`, the output size depends on the backing scale factor. On a Retina display (2x scale), requesting a 24-point icon produces a 48-pixel bitmap. On a non-Retina display, it produces 24 pixels. If the caller asks for `load_icon("sf-symbols", DialogWarning, 24)` expecting a 24x24 pixel image, they get 48x48 on Retina.

Additionally:
1. **P3 to sRGB color space:** SF Symbols rendered into a bitmap may be in Display P3 color space. The existing macOS reader already handles P3-to-sRGB conversion for colors, but the same conversion must be applied to icon rasterization.
2. **NSBitmapImageRep is scale-aware, CGImage is not:** If you extract pixels via `CGImage`, you lose scale factor metadata. The resulting `IconData::Rgba` must explicitly report the pixel dimensions, not point dimensions.
3. **Thread safety:** NSImage is not thread-safe for mutation (including first-time rasterization). The `bitmapData` property triggers lazy loading with no internal synchronization. Multiple threads calling `load_icon` simultaneously can cause data races in the NSImage cache.

**Why it happens:**
macOS's coordinate system uses points (logical pixels), not physical pixels. An `NSImage` with a size of 24x24 points contains 48x48 pixels on a 2x display. This abstraction is invisible within AppKit but becomes a problem when extracting raw pixel data to pass to a non-AppKit rendering pipeline.

**How to avoid:**
1. The spec's API contract says: "`size` is in pixels. On HiDPI displays, callers should multiply the desired point size by the scale factor." This is correct -- the caller passes pixel size, and the crate returns pixel-sized data.
2. When rasterizing, create an `NSBitmapImageRep` at the exact requested pixel dimensions: `initWithBitmapDataPlanes:nil pixelsWide:size pixelsHigh:size ...`. Lock focus on it, draw the NSImage, unlock focus.
3. Convert the resulting bitmap to sRGB color space before extracting pixel data.
4. For thread safety: perform all NSImage rasterization on the main thread, or use a serial dispatch queue. Do not call `load_icon` from multiple threads simultaneously on macOS.
5. Document that on macOS, `load_icon` may need to run on the main thread (or at minimum, not concurrently with other `load_icon` calls).

**Warning signs:**
- Icons are 2x larger than expected on Retina Macs
- Colors look subtly different between macOS and other platforms (P3 vs sRGB)
- Intermittent crashes or corrupted icons when loading from background threads
- `load_icon` works in single-threaded tests but crashes in production

**Phase to address:**
Phase 2 (macOS platform implementation). The rasterization logic must be correct from the start -- getting it wrong silently produces wrong-sized images.

---

### Pitfall 7: Cross-Compilation Builds Code That Cannot Work on Target

**What goes wrong:**
The crate uses `#[cfg(target_os = "...")]` to gate platform-specific code. This means the macOS icon loader is only compiled when targeting macOS, the Windows loader only for Windows, etc. Cross-compilation works correctly for *compilation*, but runtime failures can hide behind the cfg gates:

1. **Testing blind spots:** When building on Linux (CI), the macOS and Windows icon loading paths are not compiled at all. Syntax errors, type mismatches, and logic bugs in those paths are invisible until someone builds on macOS/Windows.
2. **freedesktop crate as Linux-only dependency:** If the `freedesktop-icons` crate is used for Linux icon lookup, it should be gated behind `cfg(target_os = "linux")` in Cargo.toml. If not gated, it becomes a mandatory dependency on all platforms, pulling in Linux-specific code on macOS/Windows.
3. **Bundled SVG fallback must work everywhere:** The Material/Lucide SVG fallback path is the only path that works on ALL platforms. If it is accidentally gated behind a platform cfg, some platforms lose their fallback.
4. **Font rendering on Windows requires `windows` crate features:** Adding Segoe Fluent Icons glyph rendering requires new `windows` crate features (e.g., `Win32_Graphics_DirectWrite` or `Win32_Graphics_Gdi`). These must be added to the `[target.'cfg(target_os = "windows")'.dependencies]` section, not to the main `[dependencies]` section.

**Why it happens:**
`#[cfg]` is a compile-time mechanism. Code behind a false cfg is not checked by the compiler. The existing crate already has this pattern (the `macos` and `windows` modules), but adding icon loading significantly increases the amount of platform-specific code, increasing the surface area for hidden bugs.

**How to avoid:**
1. Set up CI with separate build jobs for each target: `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `x86_64-pc-windows-msvc`. At minimum, run `cargo check` on each target even if tests cannot run.
2. Gate platform-specific dependencies correctly in Cargo.toml using `[target.'cfg(...)'.dependencies]`.
3. Keep platform-specific code as thin as possible -- extract the raw icon data and immediately convert to `IconData`. The conversion logic (BGRA->RGBA, P3->sRGB) should be pure Rust functions testable on any platform.
4. The bundled SVG fallback path must have NO platform cfg gates. It is the universal fallback.
5. Use the existing pattern from the crate: `pub mod macos;` is always compiled (for `build_theme` testability), while the actual ObjC FFI calls are behind `#[cfg(feature = "macos")]`.

**Warning signs:**
- CI only runs on Linux and all tests pass, but the macOS build is broken
- A dependency intended for one platform is pulled in on all platforms
- The bundled SVG fallback silently disabled on one platform

**Phase to address:**
Phase 1 (CI and build infrastructure). Ensure multi-target CI before writing platform-specific code.

---

### Pitfall 8: Returning Option Instead of Result Hides Actionable Errors

**What goes wrong:**
The spec defines `load_icon()` as returning `Option<IconData>`. This conflates several failure modes into a single `None`:
- Icon role not supported by the icon set (expected, not an error)
- Icon set not found or not installed (configuration error)
- OS API call failed (runtime error)
- Font not available for glyph rendering (platform issue)
- SVG file corrupt or unreadable (data error)
- Permission denied reading icon directory (environment error)

Callers cannot distinguish "this icon set doesn't have a warning icon" (show nothing or use fallback) from "the freedesktop icon theme directory is unreadable" (log an error, alert the user).

**Why it happens:**
`Option` is simpler for the common case where the caller just wants to show an icon if available and skip it otherwise. Many icon loading APIs in other ecosystems use this pattern. But native-theme is a library, not an application -- library consumers need enough information to debug failures.

**How to avoid:**
1. Keep `load_icon()` returning `Option<IconData>` for the public API. This is the right ergonomic choice for the "show icon if available" use case.
2. Add a companion function `try_load_icon()` returning `Result<IconData, IconError>` for consumers who need diagnostic information.
3. Define `IconError` with variants: `UnsupportedRole`, `ThemeNotFound`, `PlatformError(String)`, `IoError(std::io::Error)`.
4. Implement `load_icon` as `try_load_icon(...).ok()`.
5. Log warnings (via the `log` crate) for unexpected failures inside `load_icon`. An `UnsupportedRole` is silent; an `IoError` gets `warn!()`.

Alternatively: keep only `Option<IconData>` but use `tracing` or `log` internally so failures are observable without changing the API signature. This is simpler and may be sufficient.

**Warning signs:**
- Users report `load_icon` returning None and have no way to debug why
- Bug reports that say "icons don't work" without any error message to diagnose

**Phase to address:**
Phase 1 (API design). Decide the error reporting strategy before implementing any platform code.

---

### Pitfall 9: Feature Flag Proliferation Creates Untestable Combinations

**What goes wrong:**
The spec defines three icon-related feature flags: `system-icons`, `material-icons`, `lucide-icons`. Combined with existing flags (`kde`, `portal`, `portal-tokio`, `portal-async-io`, `windows`, `macos`), the crate now has 9+ features. With N features, there are 2^N possible combinations (512). Some combinations are invalid (e.g., `portal-tokio` + `portal-async-io`), some are unusual but valid (e.g., `material-icons` without `system-icons`). Testing all combinations is infeasible.

Specific risks:
- `system-icons` without `macos`/`windows` features: on macOS/Windows, the system icon loading path is compiled but the platform reader dependencies are not. Does this compile? Does it return None gracefully?
- `material-icons` without `system-icons`: the crate only has bundled SVGs, no platform icon loading. Is the fallback chain correct?
- Default features include `system-icons` and `material-icons` but not `lucide-icons`. A user who adds `default-features = false, features = ["lucide-icons"]` gets Lucide but no system icons and no Material fallback. Is this a usable configuration?

**Why it happens:**
Feature flags are additive in Cargo. Each flag adds capability but also adds a dimension to the test matrix. The desire for fine-grained control ("I only want Lucide, not Material") conflicts with the desire for simplicity ("just give me icons").

**How to avoid:**
1. Keep the feature set minimal. The proposed three flags (`system-icons`, `material-icons`, `lucide-icons`) are reasonable. Do NOT add per-platform icon flags (e.g., `sf-symbols`, `segoe-fluent`, `freedesktop-icons`) -- this is too granular.
2. `system-icons` should be a meta-feature that enables platform-appropriate icon loading. On macOS it uses the `macos` feature's dependencies (objc2). On Windows it uses the `windows` feature's dependencies. On Linux it adds `freedesktop-icons` as a dependency. The user enables `system-icons` and the crate handles the rest via cfg.
3. Test the "important" combinations in CI: default, `system-icons` only, `material-icons` only, `lucide-icons` only, all three, none (just color/font theme data).
4. Document which features are needed for which use cases in the crate-level docs.
5. Ensure that disabling all icon features produces a crate that compiles and works -- it just has no icon loading capability. `load_icon` returns None for everything.

**Warning signs:**
- CI passes with default features but fails with a specific feature combination
- A feature flag silently enables a heavy dependency the user didn't expect
- Users file issues saying "icons don't work" because they disabled default features

**Phase to address:**
Phase 1 (feature flag design). Design the feature matrix and test it BEFORE implementing icon loading.

---

### Pitfall 10: SHGetStockIconInfo Requires COM Initialization and Has HICON Leak Potential

**What goes wrong:**
`SHGetStockIconInfo` is a Shell API function. Shell API functions generally require COM to be initialized on the calling thread. Calling `SHGetStockIconInfo` without `CoInitializeEx` may succeed on some Windows versions but fail or return incorrect results on others. Additionally, the returned HICON must be freed with `DestroyIcon` -- the caller owns it. This is easy to forget or get wrong in Rust's ownership model, especially if an early return or panic occurs between the `SHGetStockIconInfo` call and the `DestroyIcon` call.

**Why it happens:**
The `SHGetStockIconInfo` documentation does not explicitly list COM initialization as a prerequisite, but related Shell functions (`SHGetFileInfo`) do. The inconsistency in Microsoft's documentation leads developers to skip COM initialization. As for HICON leaks, Rust's borrow checker does not track Win32 handle ownership.

**How to avoid:**
1. Call `CoInitializeEx(None, COINIT_APARTMENTTHREADED)` at the start of the Windows icon loading path. Check the return value -- `S_OK` or `S_FALSE` (already initialized) are both fine. Call `CoUninitialize` when done, respecting the COM initialization count.
2. Create a RAII wrapper for HICON:
   ```rust
   struct OwnedIcon(HICON);
   impl Drop for OwnedIcon {
       fn drop(&mut self) {
           unsafe { DestroyIcon(self.0); }
       }
   }
   ```
3. Similarly, create RAII wrappers for HBITMAP (from `GetIconInfo`), which must be freed with `DeleteObject`.
4. Use the RAII wrappers consistently so that panics and early returns cannot leak handles.

**Warning signs:**
- `SHGetStockIconInfo` returns `E_FAIL` intermittently
- GDI object count in Task Manager grows over time
- The function works in debug mode but fails in release mode (timing-dependent COM issues)

**Phase to address:**
Phase 2 (Windows platform implementation). The RAII wrappers should be the first thing implemented.

---

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Bundling all 3,800 Material SVGs "for completeness" | Users can load any Material icon | 50+ MB crate, 5+ second compile time, hits crates.io 10MB limit | Never -- bundle only the ~42 IconRole icons |
| Using `include_dir!` for SVG directory | One-line include of all icons | Compile time regression, binary bloat, cannot subset | Never -- use explicit `include_bytes!` per file |
| Skipping hicolor fallback in freedesktop lookup | Simpler lookup code | Missing icons on themes that rely on hicolor inheritance | Never -- the spec mandates hicolor as ultimate fallback |
| Hardcoding `/usr/share/icons` instead of respecting XDG_DATA_DIRS | Works on most distros | Fails on Nix, Flatpak, custom prefixes, some Fedora configs | Never -- XDG compliance is table stakes |
| Making `load_icon` panic on errors instead of returning None | Easier debugging during development | Crashes production apps | Never |
| Not freeing HICON handles ("they're small") | Less code | GDI handle exhaustion after loading ~10,000 icons | Never -- use RAII wrappers |
| Loading all icons eagerly at startup | Simple initialization | Slow startup, wasted memory for unused icons | Never -- load on demand |
| Skipping premultiply-to-straight alpha conversion on Windows | Simpler extraction code | Icons look washed out in all renderers that expect straight alpha | Never |

## Integration Gotchas

Common mistakes when connecting to platform icon APIs.

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| `NSImage(systemSymbolName:)` | Not checking for nil return (symbol name typo or unavailable on older macOS) | Check for nil; fall back to bundled SVG |
| `NSImage` rasterization | Drawing into a context without specifying size in pixels | Create `NSBitmapImageRep` at exact pixel dimensions before drawing |
| `NSImage` color space | Extracting pixels in Display P3 on wide-gamut displays | Convert to sRGB before extracting bitmap data |
| `SHGetStockIconInfo` | Not initializing `cbSize` field of `SHSTOCKICONINFO` | Set `sii.cbSize = std::mem::size_of::<SHSTOCKICONINFO>() as u32` before calling |
| `SHGetStockIconInfo` | Using `SHGSI_LARGEICON` expecting a specific size | The size depends on system DPI and icon set; do not assume 32x32 or 48x48 |
| `GetDIBits` for icon extraction | Using `BI_BITFIELDS` compression which changes pixel layout | Use `BI_RGB` for straightforward BGRA extraction |
| freedesktop `index.theme` | Assuming directory names follow `{size}x{size}` pattern | Parse `index.theme` to get actual directory names and their type/size parameters |
| freedesktop inheritance | Stopping after the first theme in the chain | Traverse ALL parents recursively, then always check hicolor last |
| `include_bytes!` for SVGs | Including SVGs with XML declarations and comments | Strip XML declarations and minify before bundling to save space |
| Material Symbols SVGs | Assuming all SVGs use the same viewBox | Verify viewBox is consistent (typically `0 0 24 24` for 24dp) or normalize at load time |
| Lucide SVGs | Assuming stroke-based SVGs render at any size | Lucide uses `stroke-width="2"` at 24px; scaling down to 16px makes strokes disproportionately thick |

## Performance Traps

Patterns that work at small scale but fail as usage grows.

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Parsing freedesktop `index.theme` on every `load_icon` call | 5-10ms per icon load (file I/O + INI parsing) | Parse once, cache in a `LazyLock<HashMap>` | First production use with icon-heavy UI (50+ icons) |
| Re-rasterizing the same SF Symbol at the same size | 1-2ms per rasterization (ObjC message dispatch overhead) | Cache `IconData` by (role, size) tuple | Any UI that redraws frequently |
| Calling `SHGetStockIconInfo` + full HICON extraction per frame | Multiple GDI API calls per icon per frame | Cache extracted RGBA data, not HICON handles | Any real-time rendering loop |
| Loading all 42 bundled SVGs at startup via `include_bytes!` | ~84 KB decoded into 42 allocations at startup | SVGs are already in static memory via `include_bytes!`; parse on demand, not eagerly | Only matters if SVG parsing is eager |
| Scanning filesystem for freedesktop icon on every load | Repeated `stat()` calls on icon directories | Build a lookup table from `index.theme` and cache file paths | More than 10 icon loads per second |

## "Looks Done But Isn't" Checklist

Things that appear complete but are missing critical pieces.

- [ ] **SF Symbols loading:** `NSImage(systemSymbolName:)` works -- but verify it returns nil for names not available on the running macOS version (e.g., symbols added in macOS 15 tested on macOS 14)
- [ ] **SF Symbols rasterization:** Bitmap extracted -- but verify the pixel dimensions match the requested size, not 2x on Retina
- [ ] **SF Symbols colors:** Icon rasterized -- but verify it renders in a usable color (not all-black template images). SF Symbols are template images by default; they need to be tinted or rendered with `withSymbolConfiguration` to get a visible color
- [ ] **Windows HICON extraction:** RGBA pixels extracted -- but verify alpha channel is correct by testing with the Shield icon (which has complex alpha) not just the Warning icon (which is mostly opaque)
- [ ] **Windows Segoe Fluent glyph rendering:** Glyph rendered -- but verify the font was actually found. Silently falling through to a default font produces wrong glyphs
- [ ] **freedesktop lookup:** Icons load from Adwaita -- but test with a minimal hicolor-only installation (no Adwaita, no Breeze). The fallback to hicolor must work standalone
- [ ] **freedesktop lookup:** Works with `-symbolic` icons -- but verify full-color icons also load (remove `-symbolic` suffix fallback is working)
- [ ] **Bundled SVGs:** 42 icons compile -- but verify they render correctly at 16px, 24px, and 48px. SVGs with hardcoded stroke widths look wrong at small sizes
- [ ] **Feature flags:** `default-features = false` compiles -- but verify that `load_icon` returns None gracefully (not a compile error) when no icon features are enabled
- [ ] **load_icon with "system":** Resolves to correct platform set -- but verify the resolution does not panic on unsupported platforms (e.g., FreeBSD, WASM)
- [ ] **icon_name() for all sets:** Returns correct identifiers -- but verify the names match current versions (SF Symbols renames happen across major versions; freedesktop spec adds names over time)
- [ ] **HICON cleanup:** `DestroyIcon` called -- but verify `hbmColor` and `hbmMask` from `GetIconInfo` are also freed with `DeleteObject`
- [ ] **Thread safety:** Single-threaded tests pass -- but verify `load_icon` does not crash when called from multiple threads on macOS (NSImage cache race)

## Recovery Strategies

When pitfalls occur despite prevention, how to recover.

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| SF Symbols rasterization at wrong scale (Pitfall 6) | LOW | Fix the bitmap rep creation to use pixel dimensions; change is localized to one function |
| HICON alpha channel wrong (Pitfall 5) | MEDIUM | Add premultiplied-to-straight conversion and AND-mask fallback; requires testing with many icon types |
| freedesktop lookup missing inheritance (Pitfall 3) | MEDIUM | Refactor lookup to follow spec exactly; may require rewriting the lookup loop |
| SVG bundle too large (Pitfall 4) | LOW | Remove extra SVGs, keep only IconRole set; no API change |
| GDI handle leak (Pitfall 10) | LOW | Add RAII wrappers; no API change, just internal cleanup |
| Feature flag combination doesn't compile (Pitfall 9) | LOW | Add missing cfg gates; fix is mechanical |
| SF Symbols names violate license (Pitfall 1) | LOW | Names are fine; if glyph data was accidentally bundled, remove it from the package |
| Segoe Fluent font bundled (Pitfall 2) | LOW | Remove from package; update rendering to load from system only |
| NSImage thread safety crash (Pitfall 6) | HIGH | Must serialize all macOS icon loading to a single thread; may require architectural change to add a dispatch queue or main-thread requirement |
| Option API hides errors (Pitfall 8) | MEDIUM | Adding `try_load_icon()` is backward-compatible but requires propagating errors through all internal code |

## Pitfall-to-Phase Mapping

How roadmap phases should address these pitfalls.

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| SF Symbols licensing (Pitfall 1) | Phase 1: API design | Audit: `cargo package --list` contains no font/image files from Apple |
| Segoe Fluent licensing (Pitfall 2) | Phase 1: API design | Audit: no `.ttf` files in published package; rendering loads from system font only |
| freedesktop lookup (Pitfall 3) | Phase 2: Linux implementation | Test with Adwaita, Breeze, Papirus, and hicolor-only; test with custom XDG_DATA_DIRS |
| SVG bundle size (Pitfall 4) | Phase 3: Bundled icons | `cargo bloat --crates` shows < 200 KB for material-icons feature |
| HICON conversion (Pitfall 5) | Phase 2: Windows implementation | Visual comparison of extracted RGBA vs native Windows icon rendering |
| NSImage rasterization (Pitfall 6) | Phase 2: macOS implementation | Test on both Retina and non-Retina; verify pixel dimensions match requested size |
| Cross-compilation (Pitfall 7) | Phase 1: CI setup | CI runs `cargo check` for all three target triples |
| Option vs Result API (Pitfall 8) | Phase 1: API design | API review before implementation begins |
| Feature flag combinations (Pitfall 9) | Phase 1: Feature design | CI tests: default, each flag alone, all flags, no flags |
| COM init / HICON leak (Pitfall 10) | Phase 2: Windows implementation | Run icon loading in a loop 10,000 times; verify GDI handle count does not grow |

## Sources

- [SF Symbols licensing](https://developer.apple.com/sf-symbols/) -- "All SF Symbols shall be considered to be system-provided images" (Apple Developer)
- [SF Symbol License discussion](https://developer.apple.com/forums/thread/757407) -- Apple Developer Forums, community discussion on redistribution constraints
- [Segoe Fluent Icons font](https://learn.microsoft.com/en-us/windows/apps/design/style/segoe-fluent-icons-font) -- Microsoft Learn, codepoint reference and EULA restrictions
- [Segoe Fluent Icons redistribution issue](https://github.com/microsoft/fluentui-system-icons/issues/202) -- GitHub issue discussing the distinction between Segoe Fluent Icons (proprietary) and Fluent UI System Icons (MIT)
- [freedesktop Icon Theme Specification](https://specifications.freedesktop.org/icon-theme/latest/) -- icon lookup algorithm, inheritance, hicolor fallback, directory types, caching
- [freedesktop-icons crate](https://docs.rs/freedesktop-icons) -- Rust implementation of icon theme lookup
- [NSImage is dangerous](https://wadetregaskis.com/nsimage-is-dangerous/) -- thread safety analysis of NSImage, cache races, bitmapData hazards
- [Apple Thread Safety Summary](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html) -- NSImage threading constraints
- [NSImage HiDPI APIs](https://developer.apple.com/library/archive/documentation/GraphicsAnimation/Conceptual/HighResolutionOSX/APIs/APIs.html) -- NSBitmapImageRep vs CGImage, backing scale factor
- [HICON to PNG in Rust](https://users.rust-lang.org/t/how-to-convert-hicon-to-png/90975) -- community discussion of BGRA byte order, GetDIBits alignment, alpha channel detection
- [Windows Premultiplied Alpha](https://learn.microsoft.com/en-us/windows/apps/develop/win2d/premultiplied-alpha) -- Microsoft Win2D docs on straight vs premultiplied alpha
- [SHGetStockIconInfo](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetstockiconinfo) -- handle ownership, DestroyIcon requirement
- [include_bytes! performance](https://github.com/rust-lang/rust/issues/65818) -- compiler regression with large binary blobs
- [include_flate crate](https://docs.rs/include-flate) -- compile-time compression for embedded resources
- [Rust feature flags best practices](https://effective-rust.com/features.html) -- "Item 26: Be wary of feature creep"
- [Cargo features documentation](https://doc.rust-lang.org/cargo/reference/features.html) -- 300 feature limit, additive semantics, resolver v2

---
*Pitfalls research for: v0.3 native icon loading addition to native-theme crate*
*Researched: 2026-03-09*
