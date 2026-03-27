# Stack Research: Per-Widget Architecture, ResolvedTheme, and Extended OS Readers

**Domain:** Per-widget theme structs, Option-to-concrete resolution, extended platform API readers
**Researched:** 2026-03-27
**Confidence:** HIGH (all findings verified against local cargo registry sources)

## Scope

This research covers ONLY the stack additions needed for the per-widget
ThemeVariant restructuring, ResolvedTheme non-optional output, universal
`resolve()` inheritance, and extended OS readers. It does NOT re-research
the existing validated stack (serde, toml, resvg, freedesktop-icons,
objc2, windows crate, ashpd, configparser, connectors).

Focus areas:
1. `serde_with::skip_serializing_none` with deeply nested Option structs
2. Rust patterns for generating Resolved* (non-Option) structs from Option-based structs
3. macOS reader extensions: NSFont.TextStyle, specialized fonts, NSFontDescriptor weight
4. Windows reader extensions: GetSysColor, DwmGetColorizationColor, additional LOGFONT fields, UISettings extras, SPI_GETHIGHCONTRAST
5. KDE reader extensions: [WM], [Colors:Header], [Colors:Complementary], font keys, icon theme parsing
6. GNOME reader extensions: gsettings for fonts, text-scaling-factor, overlay-scrolling, enable-animations, icon-theme

---

## 1. serde_with::skip_serializing_none with Deeply Nested Option Structs

**Confidence: HIGH** (verified against codebase -- already using this pattern successfully)

### Current State

The crate already uses `#[serde_with::skip_serializing_none]` on 14 structs
(ThemeColors with 36 Option fields, ButtonMetrics with 5, etc.). TOML
round-trip tests pass with sparse data. This is a proven pattern in the
codebase.

### Scaling to 24+ Widget Structs

The new ThemeVariant design adds ~24 per-widget structs, each with 5-15
Option fields. `skip_serializing_none` works identically regardless of
struct count or nesting depth because it is a **per-struct attribute
macro** -- it rewrites each struct's serde annotations independently at
compile time. It does not interact across struct boundaries.

For the new architecture:
- Each widget struct (ButtonTheme, InputTheme, etc.) gets `#[serde_with::skip_serializing_none]`
- ThemeDefaults gets `#[serde_with::skip_serializing_none]`
- The containing ThemeVariant uses `#[serde(default, skip_serializing_if = "...::is_empty")]` on nested struct fields (existing pattern from WidgetMetrics)
- FontSpec, TextScaleEntry also get `#[serde_with::skip_serializing_none]`

### Verified Behavior

- `skip_serializing_none` applies to `Option<T>` fields only. Non-Option fields are untouched.
- Nested structs (e.g., `pub font: FontSpec` inside ButtonTheme) are NOT affected by the parent's `skip_serializing_none`. The nested struct needs its own attribute.
- Empty sub-structs are omitted via the existing `skip_serializing_if = "is_empty"` pattern on the parent.
- The `impl_merge!` macro already handles both `option` (leaf) and `nested` (recursive) field categories.

### What to Watch

- `FontSpec` contains a mix of `Option<String>` and `Option<f32>` fields -- `skip_serializing_none` handles both correctly.
- `DialogButtonOrder` is a non-Option enum inside `Option<DialogButtonOrder>` -- this works fine, the Option wrapping handles it.
- TOML nesting depth: the deepest new path is `[light.button.font]` (3 levels under root). The `toml` crate handles arbitrary nesting; existing `[light.colors]` is 2 levels.

### Recommendation

**Continue using `serde_with::skip_serializing_none` 3.17.0.** No version change needed. No new dependency.

---

## 2. Generating Resolved* Structs from Option-Based Structs

**Confidence: HIGH** (evaluated all options; recommending declarative macros)

### The Problem

The design requires parallel struct hierarchies:
- `ButtonTheme` (all `Option<T>` fields) for merge/overlay
- `ResolvedButton` (all concrete `T` fields) for consumer use
- ~26 struct pairs total (24 widgets + ThemeDefaults + TextScale)
- Plus `try_from()` conversion that reports missing fields

### Options Evaluated

#### Option A: `optfield` / `optional_struct` Crates (REJECTED)

**optfield** generates Option-wrapped structs FROM concrete structs (direction:
concrete -> optional). We need the inverse (optional -> concrete). optfield
does not support this direction. Verified from docs.rs documentation.

**optional_struct** generates optional structs and a merge/fuse function. The
"fuse" can produce the original struct, but it uses `unwrap()` internally
(panics on None) rather than collecting errors. It also requires the
non-optional struct to exist first, then derives the optional version -- the
opposite of our ownership model where Option structs are primary and
ResolvedTheme is derived. Verified from docs.rs documentation.

Neither crate fits the requirement.

#### Option B: Custom Proc Macro (REJECTED)

A derive macro like `#[derive(Resolve)]` on each Option struct could
generate the Resolved* counterpart and a `validate()` implementation.
However:

- Requires a new `native-theme-macros` proc-macro crate in the workspace
- Proc macros need `syn` + `quote` + `proc-macro2` dependencies (~80KB compile overhead)
- The macro must parse field types to distinguish `Option<T>` from non-Option (`FontSpec` nested structs)
- Error messages from generated code are harder to debug
- The conversion logic is not purely mechanical -- some fields have custom inheritance rules (accent -> primary_bg) that a generic derive cannot handle

The complexity-to-benefit ratio is poor for 26 structs.

#### Option C: Declarative `macro_rules!` (RECOMMENDED)

Use `macro_rules!` to reduce boilerplate while keeping logic explicit.
Pattern: define both structs in one macro invocation.

```rust
/// Generate a widget Option struct and its Resolved counterpart.
macro_rules! define_widget_pair {
    (
        $(#[$meta:meta])*
        $vis:vis struct $opt_name:ident / $resolved_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        // The Option struct (for merge/overlay, serde)
        #[serde_with::skip_serializing_none]
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        #[non_exhaustive]
        $(#[$meta])*
        $vis struct $opt_name {
            $(
                $(#[$field_meta])*
                $field_vis $field: Option<$ty>,
            )*
        }

        // The Resolved struct (for consumers, no Option)
        #[derive(Clone, Debug, PartialEq)]
        $vis struct $resolved_name {
            $(
                $field_vis $field: $ty,
            )*
        }

        impl_merge!($opt_name {
            option { $($field),* }
        });
    };
}
```

### Why Declarative Macros

1. **Zero new dependencies.** No proc-macro crate, no syn/quote.
2. **Already proven.** The project uses `impl_merge!` macro_rules successfully.
3. **Explicit.** Both struct shapes are visible in the macro invocation.
4. **Debuggable.** `cargo expand` shows the generated code.
5. **Flexible.** Nested struct fields (FontSpec) can be handled with a separate category (like `impl_merge!`'s `nested {}` syntax).

### Nested Fields (FontSpec)

Widget structs contain nested `FontSpec` fields that should NOT be Option-wrapped
at the field level (they are always present as a struct). Instead, `FontSpec`
itself has `Option` fields. For the Resolved version, `ResolvedFontSpec` has
concrete fields. The macro needs a `nested {}` category:

```rust
define_widget_pair! {
    pub struct ButtonTheme / ResolvedButton {
        option { background: Rgba, foreground: Rgba, ... }
        nested { font: FontSpec / ResolvedFontSpec }
    }
}
```

### validate() Implementation

The `validate()` function that converts `ThemeVariant -> ResolvedTheme` is
hand-written, not macro-generated. It calls per-widget validate functions:

```rust
fn validate_button(opt: &ButtonTheme) -> Result<ResolvedButton, Vec<String>> {
    let mut missing = Vec::new();
    // Each field checked, missing paths collected
    // Font validated recursively
}
```

This is ~10 lines per widget struct. For 26 structs, that is ~260 lines --
manageable and explicit about what each field requires.

### Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| Resolved struct generation | `macro_rules!` pairs | proc macro derive | Proc macro adds build-time dep (syn/quote), harder to debug, overkill for 26 structs |
| resolve() design | Single explicit function | Trait-based per-widget | Inheritance requires cross-widget access (defaults.accent -> button.primary_bg); trait adds indirection without value |
| validate() error collection | `Vec<String>` of paths | thiserror enum per field | 100+ fields would produce 100+ enum variants; path strings are simpler and sufficient |
| FontSpec in widget structs | Direct FontSpec (nested merge) | `Option<FontSpec>` (option merge) | `Option<FontSpec>` replaces entire font on merge, losing partial overrides |

### Recommendation

**Use `macro_rules!` to define struct pairs. Hand-write `validate()` functions.**
No new dependencies. The macro eliminates field-list duplication while keeping
conversion logic transparent.

---

## 3. macOS Reader Extensions (objc2-app-kit)

**Confidence: HIGH** (verified against local objc2-app-kit 0.3.2 source in ~/.cargo/registry)

### NSFont.TextStyle API

All text style constants exist in objc2-app-kit 0.3.2 as `extern "C"` statics
in `NSFontDescriptor.rs`. No additional feature flags needed beyond the existing
`NSFontDescriptor` feature (already enabled in Cargo.toml).

| Constant | Rust Binding | Feature Gate |
|----------|-------------|--------------|
| `NSFontTextStyleLargeTitle` | `pub static NSFontTextStyleLargeTitle: &'static NSFontTextStyle` | `NSFontDescriptor` (already enabled) |
| `NSFontTextStyleTitle1` | `pub static NSFontTextStyleTitle1: &'static NSFontTextStyle` | `NSFontDescriptor` |
| `NSFontTextStyleHeadline` | `pub static NSFontTextStyleHeadline: &'static NSFontTextStyle` | `NSFontDescriptor` |
| `NSFontTextStyleCaption1` | `pub static NSFontTextStyleCaption1: &'static NSFontTextStyle` | `NSFontDescriptor` |
| `NSFontTextStyleBody` | `pub static NSFontTextStyleBody: &'static NSFontTextStyle` | `NSFontDescriptor` |

`NSFontTextStyle` is a type alias for `NSString` (line 112 of NSFontDescriptor.rs).
The constants are linked at runtime from the AppKit framework.

### preferredFontForTextStyle

Two methods are available:

1. **On NSFont** (line 642 of NSFont.rs):
   ```rust
   pub unsafe fn preferredFontForTextStyle_options(
       style: &NSFontTextStyle,
       options: &NSDictionary<NSFontTextStyleOptionKey, AnyObject>,
   ) -> Retained<NSFont>;
   ```
   Requires: `NSFontDescriptor` feature (already enabled).

2. **On NSFontDescriptor** (line 641 of NSFontDescriptor.rs):
   ```rust
   pub unsafe fn preferredFontDescriptorForTextStyle_options(
       style: &NSFontTextStyle,
       options: &NSDictionary<NSFontTextStyleOptionKey, AnyObject>,
   ) -> Retained<NSFontDescriptor>;
   ```
   Returns a descriptor -- useful for extracting weight without instantiating a font.

Usage pattern:
```rust
use objc2_app_kit::{NSFont, NSFontTextStyleCaption1};
use objc2_foundation::NSDictionary;

let empty_opts = NSDictionary::new();
let caption_font = unsafe {
    NSFont::preferredFontForTextStyle_options(NSFontTextStyleCaption1, &empty_opts)
};
let size = unsafe { caption_font.pointSize() } as f32;
let family = caption_font.familyName().map(|n| n.to_string());
```

### Specialized Font Methods

All require `objc2-core-foundation` feature (already enabled):

| Method | Maps to | Feature |
|--------|---------|---------|
| `NSFont::titleBarFontOfSize(0.0)` | `window.title_bar_font` | `objc2-core-foundation` (existing) |
| `NSFont::menuFontOfSize(0.0)` | `menu.font` | `objc2-core-foundation` (existing) |
| `NSFont::toolTipsFontOfSize(0.0)` | `tooltip.font` | `objc2-core-foundation` (existing) |
| `NSFont::messageFontOfSize(0.0)` | (already used for defaults.font) | `objc2-core-foundation` (existing) |
| `NSFont::labelFontOfSize(0.0)` | (informational) | `objc2-core-foundation` (existing) |

Pass `0.0` for size to get the system default size for each role.

### NSFontDescriptor Weight Extraction

The font weight can be extracted from any NSFont via its descriptor.
Constants verified in NSFontDescriptor.rs (no feature gate):

- `NSFontTraitsAttribute` (line 362): `&'static NSFontDescriptorAttributeName`
- `NSFontWeightTrait` (line 382): `&'static NSFontDescriptorTraitKey`
- `NSFontBoldTrait` (line 615): `c_uint` = `1 << 1`

Methods on NSFontDescriptor:
- `fn fontDescriptor(&self) -> Retained<NSFontDescriptor>` (on NSFont, line 262)
- `fn symbolicTraits(&self) -> NSFontDescriptorSymbolicTraits` (line 180)
- `fn objectForKey(&self, attribute: &NSFontDescriptorAttributeName) -> Option<Retained<AnyObject>>` (line 188)

The weight is a `CGFloat` in range -1.0 to 1.0. Approximate CSS mapping:
`css_weight = ((objc_weight + 1.0) / 2.0 * 800.0 + 100.0).round() as u16`

### Accessibility Queries

Already confirmed available. Methods on `NSWorkspace`:
- `accessibilityDisplayShouldReduceMotion` -> `defaults.reduce_motion`
- `accessibilityDisplayShouldIncreaseContrast` -> `defaults.high_contrast`
- `accessibilityDisplayShouldReduceTransparency` -> `defaults.reduce_transparency`

Features `NSAccessibility` + `NSWorkspace` already enabled in Cargo.toml.

### Additional NSColor Values Needed

All available with existing features (NSColor, NSColorSpace):
- `NSColor::placeholderTextColor()` -> `input.placeholder`
- `NSColor::windowFrameTextColor()` -> `window.title_bar_foreground`
- `NSColor::textInsertionPointColor()` -> `input.caret` (macOS 14+)
- `NSColor::unemphasizedSelectedContentBackgroundColor()` -> `defaults.selection_inactive`
- `NSColor::alternatingContentBackgroundColors()` -> `list.alternate_row` (array, take index 1)
- `NSColor::headerTextColor()` -> `list.header_foreground`
- `NSColor::gridColor()` -> `list.grid_color`

### New Feature Flags Needed

**None.** All required features are already enabled.

---

## 4. Windows Reader Extensions

**Confidence: HIGH** (verified against local windows 0.62.2 crate source in ~/.cargo/registry)

### GetSysColor Widget Colors

`GetSysColor(nindex: SYS_COLOR_INDEX) -> u32` is in `Win32::Graphics::Gdi`.
The function and all needed `COLOR_*` constants are verified present under the
existing `Win32_Graphics_Gdi` feature:

| Constant | Value | Maps to |
|----------|-------|---------|
| `COLOR_BTNFACE` | 15 | `button.background` |
| `COLOR_BTNTEXT` | 18 | `button.foreground` |
| `COLOR_MENU` | 4 | `menu.background` |
| `COLOR_MENUTEXT` | 7 | `menu.foreground` |
| `COLOR_INFOBK` | 24 | `tooltip.background` |
| `COLOR_INFOTEXT` | 23 | `tooltip.foreground` |
| `COLOR_WINDOW` | 5 | `input.background` |
| `COLOR_WINDOWTEXT` | 8 | `input.foreground` |
| `COLOR_HIGHLIGHT` | 13 | `defaults.selection` |
| `COLOR_HIGHLIGHTTEXT` | 14 | `defaults.selection_foreground` |
| `COLOR_CAPTIONTEXT` | 9 | `window.title_bar_foreground` |
| `COLOR_INACTIVECAPTION` | 3 | `window.inactive_title_bar_background` |
| `COLOR_INACTIVECAPTIONTEXT` | 19 | `window.inactive_title_bar_foreground` |

Return value is `0x00BBGGRR` (BGR order). Conversion:
```rust
fn sys_color_to_rgba(c: u32) -> Rgba {
    Rgba::rgb((c & 0xFF) as u8, ((c >> 8) & 0xFF) as u8, ((c >> 16) & 0xFF) as u8)
}
```

**No new feature flags needed** -- `Win32_Graphics_Gdi` already enabled.

### DwmGetColorizationColor

Returns DWM title bar colorization in `0xAARRGGBB` format (ARGB order).

**New feature required:** `Win32_Graphics_Dwm`

### Additional LOGFONT Fields

`NONCLIENTMETRICSW` (already read via `SystemParametersInfoW`) contains
additional LOGFONTW fields beyond the currently-used `lfMessageFont`:

| Field | Maps to | Already Accessible |
|-------|---------|-------------------|
| `lfCaptionFont` | `window.title_bar_font` | Yes (same struct) |
| `lfMenuFont` | `menu.font` | Yes (same struct) |
| `lfStatusFont` | `status_bar.font` | Yes (same struct) |
| `lfSmCaptionFont` | (small caption, not mapped) | Yes |

Each `LOGFONTW` provides:
- `lfFaceName` -> family (existing extraction logic)
- `lfHeight` -> size in points (existing conversion: `abs(lfHeight) * 72 / dpi`)
- `lfWeight` -> CSS weight (FW_NORMAL=400, FW_BOLD=700, direct mapping)

**No new feature flags needed** -- all in `Win32_UI_WindowsAndMessaging`.

### UISettings Extras

Methods on the existing `UISettings` struct (feature `UI_ViewManagement`,
already enabled):

| Method | Return | Maps to |
|--------|--------|---------|
| `TextScaleFactor()` | `Result<f64>` | `defaults.text_scaling_factor` (range 1.0-2.25) |
| `AnimationsEnabled()` | `Result<bool>` | `defaults.reduce_motion` (inverted: false = reduce) |
| `AutoHideScrollBars()` | `Result<bool>` | `scrollbar.overlay_mode` |

**No new feature flags needed.**

### SPI_GETHIGHCONTRAST

`SPI_GETHIGHCONTRAST` constant is in `Win32_UI_WindowsAndMessaging` (already
enabled). The `HIGHCONTRASTW` struct and `HCF_HIGHCONTRASTON` flag are in
`Win32_UI_Accessibility`.

**New feature required:** `Win32_UI_Accessibility`

### SPI_GETCLIENTAREAANIMATION

Already available in `Win32_UI_WindowsAndMessaging` (existing feature).
Uses `SystemParametersInfoW` with a `BOOL` output parameter.

**No new feature flags needed.**

### Additional System Metrics

Already available under `Win32_UI_WindowsAndMessaging`:

| Metric | Constant | Maps to |
|--------|----------|---------|
| Focus border width | `SM_CXFOCUSBORDER` (83) | `defaults.focus_ring_width` |
| Small icon size | `SM_CXSMICON` (49) | `defaults.icon_sizes.small` |
| Large icon size | `SM_CXICON` (11) | `defaults.icon_sizes.large` |

**No new feature flags needed.**

### Summary: Windows Feature Changes

```toml
windows = { version = ">=0.59, <=0.62", optional = true, default-features = false, features = [
    "UI_ViewManagement",                  # existing
    "Win32_UI_WindowsAndMessaging",       # existing
    "Win32_UI_HiDpi",                     # existing
    "Win32_Graphics_Gdi",                 # existing
    "Win32_UI_Shell",                     # existing
    "Foundation_Metadata",                # existing
    "Win32_Graphics_Dwm",                # NEW: DwmGetColorizationColor
    "Win32_UI_Accessibility",            # NEW: HIGHCONTRASTW
] }
```

**2 new feature flags. Zero new crate dependencies.**

---

## 5. KDE Reader Extensions

**Confidence: HIGH** (verified against local kdeglobals and configparser behavior)

### [WM] Section

The `[WM]` section in kdeglobals provides window manager decoration colors.
Verified present in local `~/.config/kdeglobals`:

| Key | Format | Maps to |
|-----|--------|---------|
| `activeBackground` | `R,G,B` | `window.title_bar_background` |
| `activeForeground` | `R,G,B` | `window.title_bar_foreground` |
| `inactiveBackground` | `R,G,B` | `window.inactive_title_bar_background` |
| `inactiveForeground` | `R,G,B` | `window.inactive_title_bar_foreground` |
| `activeBlend` | `R,G,B` | (not mapped -- blend hint for kwin) |

The existing `configparser` crate with case-sensitive mode and `=`-only
delimiter handles this section correctly. Verified: `ini.get("WM",
"activeBackground")` returns `"39,44,49"`. The existing `parse_rgb()`
helper parses it.

**No new dependencies.**

### [Colors:Header] Section

Available in KDE Frameworks 5.71+ (released Aug 2020). Verified present
in local kdeglobals:

| Key | Maps to |
|-----|---------|
| `[Colors:Header] BackgroundNormal` | `list.header_background` |
| `[Colors:Header] ForegroundNormal` | `list.header_foreground` |

Also `[Colors:Header][Inactive]` section exists for inactive state headers.

The configparser crate handles `[Colors:Header]` as a section name (colon
in section name is already tested and working -- see `Colors:Window`,
`Colors:Complementary` tests in the existing codebase).

**No new dependencies.**

### Font Keys in [General]

KDE stores fonts as comma-delimited strings in `[General]`:

| Key | Format | Maps to |
|-----|--------|---------|
| `font` | `Family,Size,-1,5,Weight,...` | `defaults.font` (already read) |
| `fixed` | same format | `defaults.mono_font` (already read) |
| `activeFont` | same format | `window.title_bar_font` |
| `menuFont` | same format | `menu.font` |
| `toolBarFont` | same format | `toolbar.font` |
| `smallestReadableFont` | same format | `text_scale.caption` |

The existing `parse_fonts()` reads `font` and `fixed`. The same parsing
logic applies to all font keys.

Qt weight field (index 4 in the comma-separated string) maps to CSS weight:
- Qt 50 (Normal) -> CSS 400
- Qt 63 (DemiBold) -> CSS 600
- Qt 75 (Bold) -> CSS 700

Approximate conversion: `css_weight = (qt_weight - 50) * 12 + 400`

**No new dependencies.**

### AnimationDurationFactor and forceFontDPI

In `[KDE]` section (verified present locally):
- `AnimationDurationFactor=0` means reduced motion
- `AnimationDurationFactor=0.25` (local value) means animations enabled but fast

In `[General]` section:
- `forceFontDPI=120` means custom DPI override (text_scale = 120/96 = 1.25)
- 0 or absent means use system DPI

**No new dependencies.**

### Icon Theme from kdeglobals + index.theme Parsing

Icon theme name from `[Icons]` section:
```
[Icons]
Theme=breeze
```

Icon sizes from the theme's `index.theme` at
`/usr/share/icons/{theme}/index.theme`. Verified locally for Breeze:

```ini
[Icon Theme]
DesktopDefault=48
ToolbarDefault=22
MainToolbarDefault=22
SmallDefault=16
PanelDefault=48
DialogDefault=32
```

| Key | Maps to |
|-----|---------|
| `MainToolbarDefault` | `defaults.icon_sizes.toolbar` |
| `SmallDefault` | `defaults.icon_sizes.small` |
| `DesktopDefault` | `defaults.icon_sizes.large` |
| `DialogDefault` | `defaults.icon_sizes.dialog` |
| `PanelDefault` | `defaults.icon_sizes.panel` |

The `configparser` crate can parse `index.theme` directly (INI format).

**No new dependencies.**

---

## 6. GNOME Reader Extensions

**Confidence: HIGH** (verified against ashpd 0.13.9 source and gsettings behavior)

### Architecture: gsettings Subprocess vs Portal

The XDG Desktop Portal exposes a limited set of appearance settings:
- `color-scheme` (via `ashpd::Settings::color_scheme()`)
- `accent-color` (via `ashpd::Settings::accent_color()`)
- `contrast` (via `ashpd::Settings::contrast()`)
- `reduced-motion` (via `ashpd::Settings::reduced_motion()`) -- ashpd 0.13.7+

Font names, text scaling, overlay scrolling, icon theme, and animations
are NOT available through the portal. They must be read via `gsettings`
subprocess calls, which is the pattern already used by `read_gnome_fonts()`.

### gsettings Keys to Read

`org.gnome.desktop.interface` schema:

| gsettings Key | Maps to | Type |
|---------------|---------|------|
| `font-name` | `defaults.font` | `'Cantarell 11'` (already read) |
| `monospace-font-name` | `defaults.mono_font` | `'Source Code Pro 10'` (already read) |
| `text-scaling-factor` | `defaults.text_scaling_factor` | double (e.g. `1.0`) |
| `enable-animations` | `defaults.reduce_motion` (inverted) | `true`/`false` |
| `icon-theme` | `icon_set` | string (e.g. `'Adwaita'`) |
| `overlay-scrolling` | `scrollbar.overlay_mode` | `true`/`false` |

`org.gnome.desktop.wm.preferences` schema:

| gsettings Key | Maps to |
|---------------|---------|
| `titlebar-font` | `window.title_bar_font` |

### ashpd reduced_motion

Confirmed available in ashpd 0.13.7+ (verified in local 0.13.9 source,
line 313 of settings.rs):

```rust
pub async fn reduced_motion(&self) -> Result<ReducedMotion, Error>
```

Returns `ReducedMotion::NoPreference` or `ReducedMotion::ReducedMotion`.

The portal `reduced-motion` key was added in XDG Desktop Portal 1.21
(Jan 2025). For older portal versions, fall back to `gsettings get
org.gnome.desktop.interface enable-animations`.

The ashpd generic `read()` method (line 280) can read arbitrary portal
namespace/key pairs but GNOME-specific gsettings are NOT proxied through
the portal -- only `org.freedesktop.appearance` keys are.

### GNOME TextScale Computation

GNOME does not expose text scale entries via gsettings. They are computed
from the base font size using libadwaita CSS scale factors:

| Entry | Computation |
|-------|-------------|
| `caption` | `font.size * 0.82`, weight 400 |
| `section_heading` | `font.size * 1.0`, weight 700 (from adwaita.toml) |
| `dialog_title` | `font.size * 1.36`, weight 800 (from adwaita.toml) |
| `display` | `font.size * 1.81`, weight 800 (from adwaita.toml) |

The multipliers are design constants that belong in `adwaita.toml`, not
in the OS reader. The OS reader only provides `font.size`; `resolve()`
applies the multipliers.

### What NOT to Add for GNOME

| Avoid | Why |
|-------|-----|
| `gio` / `glib` crate | gsettings subprocess is simpler, faster to compile, and avoids linking GTK. The crate already uses this pattern successfully. |
| `dconf` crate | Lower-level than gsettings. gsettings provides schema validation and default handling. |
| Portal `read()` for non-appearance keys | The portal only exposes `org.freedesktop.appearance` namespace. Other gsettings schemas are not proxied through the portal. |

**No new dependencies.**

---

## 7. Summary: Full Dependency Delta

### Changes to Existing Dependencies

| Dependency | Change | Reason |
|------------|--------|--------|
| `serde_with` 3.17.0 | No change | Works with deeply nested Option structs as-is |
| `configparser` 3.1.0 | No change | Already handles [WM], [Colors:Header], [Icons], index.theme |
| `ashpd` 0.13.5+ | No version change | 0.13.7+ has `reduced_motion()`; workspace resolves to 0.13.9 |
| `objc2-app-kit` 0.3 | No version change, no new features | `NSFontDescriptor`, `objc2-core-foundation`, `NSAccessibility`, `NSWorkspace` already enabled |
| `windows` >=0.59, <=0.62 | **Add 2 feature flags** | `Win32_Graphics_Dwm` + `Win32_UI_Accessibility` |

### New Feature Flags on `windows` Crate

```toml
windows = { version = ">=0.59, <=0.62", optional = true, default-features = false, features = [
    "UI_ViewManagement",                  # existing
    "Win32_UI_WindowsAndMessaging",       # existing
    "Win32_UI_HiDpi",                     # existing
    "Win32_Graphics_Gdi",                 # existing
    "Win32_UI_Shell",                     # existing
    "Foundation_Metadata",                # existing
    "Win32_Graphics_Dwm",                # NEW: DwmGetColorizationColor for title bar color
    "Win32_UI_Accessibility",            # NEW: HIGHCONTRASTW for SPI_GETHIGHCONTRAST
] }
```

### Total New Crate Dependencies for the Workspace

**Zero.** All capabilities are achieved through existing deps + feature flags.

### New Internal Macros

| Macro | Purpose | Location |
|-------|---------|----------|
| `define_widget_pair!` | Generate Option + Resolved struct pairs with `impl_merge!` | `native-theme/src/model/` |

---

## 8. Version Compatibility

| Package | Version | Feature/API | Status |
|---------|---------|-------------|--------|
| serde_with | 3.17.0 | `skip_serializing_none` nested structs | Works (verified, 14 existing usages) |
| objc2-app-kit | 0.3.2 | `NSFontTextStyle*` constants | Available (verified in local source, line 530-580 of NSFontDescriptor.rs) |
| objc2-app-kit | 0.3.2 | `preferredFontForTextStyle_options` | Available under `NSFontDescriptor` feature (line 642 of NSFont.rs) |
| objc2-app-kit | 0.3.2 | `titleBarFontOfSize`, `menuFontOfSize`, `toolTipsFontOfSize` | Available under `objc2-core-foundation` feature |
| objc2-app-kit | 0.3.2 | `NSFontTraitsAttribute`, `NSFontWeightTrait` | Available, no feature gate (lines 362, 382) |
| windows | 0.62.2 | `GetSysColor` + all `COLOR_*` constants | Available under `Win32_Graphics_Gdi` (existing) |
| windows | 0.62.2 | `DwmGetColorizationColor` | Available under `Win32_Graphics_Dwm` (new feature) |
| windows | 0.62.2 | `HIGHCONTRASTW`, `HCF_HIGHCONTRASTON` | Available under `Win32_UI_Accessibility` (new feature) |
| windows | 0.62.2 | `SPI_GETCLIENTAREAANIMATION` | Available under `Win32_UI_WindowsAndMessaging` (existing) |
| windows | 0.62.2 | `UISettings::TextScaleFactor()` | Available under `UI_ViewManagement` (existing) |
| windows | 0.62.2 | `UISettings::AutoHideScrollBars()` | Available under `UI_ViewManagement` (existing) |
| ashpd | 0.13.9 | `Settings::reduced_motion()` | Available (verified line 313 of settings.rs) |
| configparser | 3.1.0 | [WM], [Colors:Header], index.theme parsing | Works (INI parser, tested with colon sections) |

---

## Sources

### Verified Against Local Source Files (HIGH Confidence)

- objc2-app-kit 0.3.2: `~/.cargo/registry/src/.../objc2-app-kit-0.3.2/src/generated/NSFontDescriptor.rs` lines 112, 362, 382, 530-580, 633-646
- objc2-app-kit 0.3.2: `~/.cargo/registry/src/.../objc2-app-kit-0.3.2/src/generated/NSFont.rs` lines 260-262, 642-643
- windows 0.62.2: `~/.cargo/registry/src/.../windows-0.62.2/src/Windows/Win32/Graphics/Gdi/mod.rs` lines 1197-1204, 2560-2590
- windows 0.62.2: `~/.cargo/registry/src/.../windows-0.62.2/src/Windows/Win32/Graphics/Dwm/mod.rs` -- DwmGetColorizationColor
- windows 0.62.2: `~/.cargo/registry/src/.../windows-0.62.2/src/Windows/Win32/UI/Accessibility/mod.rs` lines 1079-1096
- windows 0.62.2: `~/.cargo/registry/src/.../windows-0.62.2/src/Windows/Win32/UI/WindowsAndMessaging/mod.rs` -- SPI_GETHIGHCONTRAST, SPI_GETCLIENTAREAANIMATION, SM_CX*
- ashpd 0.13.9: `~/.cargo/registry/src/.../ashpd-0.13.9/src/desktop/settings.rs` lines 280, 313, 348
- Local `~/.config/kdeglobals` -- [WM], [Colors:Header], [KDE] AnimationDurationFactor sections
- Local `/usr/share/icons/breeze/index.theme` -- DesktopDefault, ToolbarDefault, SmallDefault, DialogDefault, PanelDefault keys
- Existing codebase: `native-theme/src/model/colors.rs`, `widget_metrics.rs`, `kde/`, `gnome/`, `macos.rs`, `windows.rs`

### Online Sources (MEDIUM Confidence)

- [NSFont objc2-app-kit docs](https://docs.rs/objc2-app-kit/latest/objc2_app_kit/struct.NSFont.html)
- [NSFont.TextStyle Apple docs](https://developer.apple.com/documentation/appkit/nsfont/textstyle)
- [GetSysColor windows-docs-rs](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/fn.GetSysColor.html)
- [DwmGetColorizationColor windows-docs-rs](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Dwm/fn.DwmGetColorizationColor.html)
- [UISettings windows-docs-rs](https://microsoft.github.io/windows-docs-rs/doc/windows/UI/ViewManagement/struct.UISettings.html)
- [UISettings.TextScaleFactor Microsoft](https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.textscalefactor)
- [HIGHCONTRASTW Microsoft](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-highcontrastw)
- [optfield docs](https://docs.rs/optfield) -- confirmed: only wraps fields in Option, not inverse
- [optional_struct docs](https://docs.rs/optional_struct) -- confirmed: generates optional version, not resolved version
- [ashpd GitHub](https://github.com/bilelmoussaoui/ashpd)

---
*Stack research for: native-theme per-widget architecture milestone*
*Researched: 2026-03-27*
