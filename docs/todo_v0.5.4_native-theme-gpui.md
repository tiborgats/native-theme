# v0.5.4 — native-theme-gpui: Issues & Fixes

Issues found in the gpui connector crate.

---

## 1. Animation Frame Timing Bug

`src/icons.rs:890-901`: `animated_frames_to_image_sources()` uses
`filter_map` to convert frames, silently dropping any frame that fails
conversion. But it preserves the original `frame_duration_ms` unchanged.
If 1 of 6 frames fails, the animation plays with 5 frames at the
original per-frame duration — 17% faster than intended.

```rust
let sources: Vec<ImageSource> = frames
    .iter()
    .filter_map(|f| to_image_source(f, color, size))
    .collect();
// ...
frame_duration_ms: *frame_duration_ms,  // original timing, fewer frames!
```

### Solutions

#### A. Fail the entire animation if any frame fails (recommended)

Replace `filter_map` with `map` + `collect::<Option<Vec<_>>>()`:

```rust
let sources: Option<Vec<ImageSource>> = frames
    .iter()
    .map(|f| to_image_source(f, color, size))
    .collect();
sources.map(|s| AnimatedImageSources {
    sources: s,
    frame_duration_ms: *frame_duration_ms,
})
```

| Pros | Cons |
|------|------|
| No partial animations with wrong timing | Entire animation fails if one frame is bad |
| Simple, correct semantics | Less graceful degradation |
| Timing is always correct | |
| One line change | |

#### B. Adjust `frame_duration_ms` based on surviving frame count

```rust
let adjusted_ms = frame_duration_ms * frames.len() as u32 / sources.len() as u32;
```

| Pros | Cons |
|------|------|
| Graceful degradation — shows what it can | Partial animation may look wrong (skipped frames) |
| Total animation duration stays correct | Uneven frame spacing is worse than no animation |
| | Complex reasoning about partial sequences |

#### C. Log a warning and use original timing (status quo + log)

| Pros | Cons |
|------|------|
| Makes the silent failure visible | Still plays at wrong speed |
| Easy to add | Only helps debugging, doesn't fix the bug |

**Best solution: A.** Animation frames are a coherent sequence. If one
frame can't be converted, the animation is corrupt — returning `None`
is more honest than playing a glitchy partial animation.

---

## 2. Icon Conversion Returns `Option` Without Error Context

`src/icons.rs:745-770`: `to_image_source()` returns
`Option<ImageSource>`. When SVG rasterization, BMP encoding, or
colorization fails, the caller gets `None` with no way to know why.

### Solutions

#### A. Keep `Option` but add tracing/log (recommended)

The function is called in hot loops (42+ icon roles). Full `Result`
return type would force callers to handle errors for every icon.
Instead, add `#[cfg(feature = "tracing")]` log calls:

```rust
tracing::warn!("icon conversion failed for {role:?}: {reason}");
```

However, this crate doesn't currently use `tracing`. A simpler
alternative: keep `Option` and document the failure cases clearly.

| Pros | Cons |
|------|------|
| No API change | Still returns None without programmatic context |
| Callers don't need error handling boilerplate | Requires optional tracing dependency |
| Clear docs explain when None is returned | |

#### B. Return `Result<ImageSource, IconError>`

| Pros | Cons |
|------|------|
| Full error context | Every call site must handle errors |
| Programmatic error discrimination | Verbose for icon loading loops |
| | API change for all callers |

#### C. Return `Option` + accumulate errors in a side channel

| Pros | Cons |
|------|------|
| Keeps simple return type | Complex thread-local or ref-counted side channel |
| Errors accessible after batch load | Unusual pattern, hard to discover |

**Best solution: A.** Icon loading is inherently best-effort (missing
icons shouldn't crash apps). `Option` is the right return type.
Document clearly when `None` is returned (corrupt SVG, unsupported
format, rasterization failure).

---

## 3. SVG Colorization Missing Patterns

`src/icons.rs:966-1031` (`colorize_svg()`) handles:
- `currentColor` replacement
- `fill="black"`, `fill="#000000"`, `fill="#000"`
- Implicit black (no fill on root `<svg>`)

Already handled (by blanket `replace("currentColor", &hex)`):
- `fill="currentColor"` and `stroke="currentColor"` — both covered

Not handled (documented at line 975):
- `stroke="black"` — stroke-based icons with explicit black
- `style="fill:black"` — CSS inline styles
- `fill="rgb(0,0,0)"` — RGB function notation

### Solutions

#### A. Add `stroke="black"` handling (recommended)

The only practical gap. CSS `style=` and `rgb()` are too rare in
icon sets to justify regex complexity.

```rust
svg_str = svg_str.replace("stroke=\"black\"", &format!("stroke=\"{hex}\""));
```

| Pros | Cons |
|------|------|
| Covers stroke-based monochrome SVGs | Slightly more string operations |
| Handles Lucide-style icons fully | |
| Minimal code addition | |

#### B. Use a regex-based SVG color replacer

| Pros | Cons |
|------|------|
| Handles all color formats | Regex dependency |
| More robust | Performance cost for every icon |
| | Over-engineered for bundled icon sets |

#### C. Keep current patterns (status quo)

| Pros | Cons |
|------|------|
| Works for all bundled icon sets | Third-party SVGs with stroke may not colorize |
| No change | |

**Best solution: A for `stroke="black"`, status quo for CSS/rgb().**
The `currentColor` case already handles both fill and stroke. Adding
explicit `stroke="black"` handling covers the remaining practical
case. CSS inline styles and `rgb()` notation are too rare in icon
sets to justify regex complexity.

---

## 4. Showcase Example is 5867 Lines

`examples/showcase.rs` is a full application larger than most library
modules. It mixes icon loading logic, platform-specific screenshot
capture, widget gallery rendering, and theme inspection.

### Solutions

#### A. Keep as single file (recommended)

Rust examples must be single files (Cargo convention for
`examples/*.rs`). Splitting into a multi-file example requires an
`examples/showcase/main.rs` directory structure.

The showcase serves as a designer reference tool, not a "how to use
the API" example. Its size is justified by its purpose: comprehensive
visual verification of all 108 color mappings, 86 icon roles, and all
widget types.

| Pros | Cons |
|------|------|
| No restructuring needed | Large single file |
| Works with `cargo run --example showcase` | Navigation requires search/IDE |
| Single compile unit | |

#### B. Split into `examples/showcase/` directory

```
examples/showcase/
  main.rs          (~400 lines)
  icons.rs         (~600 lines)
  widgets.rs       (~2000 lines)
  theme_map.rs     (~800 lines)
  helpers.rs       (~300 lines)
```

| Pros | Cons |
|------|------|
| Easier to navigate | Changes build command |
| Each file has focused purpose | Must manage mod imports |
| Parallel editing | |

#### C. Extract icon loading to a library module

Move `load_all_icons()` and `load_gpui_icons()` (~300 lines of
duplicated logic) from the example into `src/icons.rs` as public
helper functions.

| Pros | Cons |
|------|------|
| Eliminates duplication between load functions | Adds API surface that may not be stable |
| Reusable by users' applications | Tighter coupling to icon set detection logic |
| Showcase becomes smaller | |

**Best solution: A (keep single file) + C (extract icon loading).**
The showcase's size is inherent to its purpose. But the duplicated
icon loading logic (~150 lines duplicated between `load_all_icons`
and `load_gpui_icons`) should be extracted and deduplicated,
whether into the library or a shared helper within the example.

---

## 5. Hardcoded BMP DPI Value

`src/icons.rs:1076-1077` hardcodes DPI to 72 (2835 pixels/meter) in
BMP header metadata. This doesn't affect rendering (BMP viewers
ignore it for screen display) but is technically wrong for HiDPI.

### Solutions

#### A. Keep 72 DPI (recommended)

The BMP DPI field is metadata-only — gpui uses the pixel dimensions
directly. Changing it has zero visual effect. The value matches the
convention used by most image editors.

| Pros | Cons |
|------|------|
| No change | Technically imprecise metadata |
| No API change (would need DPI parameter) | |
| Zero visual difference | |

#### B. Accept DPI as parameter

| Pros | Cons |
|------|------|
| Correct metadata | API change for zero visual benefit |
| | Callers must know their DPI |

**Best solution: A.** This is cosmetic metadata with no rendering
impact.

---

## 6. Missing Integration Tests for Full Pipeline

The crate has 92 unit tests but no tests that exercise the full
pipeline: `ThemeSpec::preset()` -> `ResolvedThemeVariant` ->
`to_theme()` -> verify resulting `Theme` has expected values.

### Solutions

#### A. Add a pipeline round-trip test (recommended)

```rust
#[test]
fn preset_to_theme_round_trip() {
    let (theme, resolved) = from_preset("dracula", true).unwrap();
    // Verify key color mappings
    assert_ne!(theme.colors.background, Hsla::default());
    assert_ne!(theme.colors.text, Hsla::default());
    // Verify mode
    assert_eq!(theme.mode, ThemeMode::Dark);
}
```

| Pros | Cons |
|------|------|
| Catches integration bugs between crates | Requires gpui types in test |
| Validates the primary user workflow | May need test feature flags |
| Documents expected pipeline behavior | |

#### B. Add property-based tests (all presets resolve to valid Theme)

```rust
#[test]
fn all_presets_produce_valid_themes() {
    for name in ThemeSpec::list_presets() {
        let dark = from_preset(name, true);
        let light = from_preset(name, false);
        // At least one variant should succeed
        assert!(dark.is_ok() || light.is_ok(), "preset {name} failed");
    }
}
```

| Pros | Cons |
|------|------|
| Tests all presets automatically | Slower test suite |
| Catches regressions when presets change | |
| No manual test additions when presets are added | |

**Best solution: Both A and B.** A validates specific mappings; B
validates that all presets work. Together they catch both mapping
errors and preset regressions.

---

## 7. Dependency Version Pinning for Pre-1.0 Crates

`Cargo.toml:14-16` uses semver ranges for pre-1.0 dependencies:

```toml
gpui = "0.2.2"          # allows 0.2.3, 0.2.4...
gpui-component = "0.5.1"  # allows 0.5.2, 0.5.3...
```

Per semver, minor versions of pre-1.0 crates ARE breaking changes.

### Solutions

#### A. Keep semver ranges with tripwire test (recommended)

The crate already has a tripwire test (colors.rs:620-630) that fails
if `ThemeColor` field count changes. This catches structural breaks
from dependency updates at test time.

| Pros | Cons |
|------|------|
| Allows compatible patches | Breaks caught at test time, not compile time |
| Tripwire test already exists | Doesn't catch semantic changes (same fields, different meaning) |
| Standard Cargo practice | |

#### B. Use exact version pins (`=0.5.1`)

| Pros | Cons |
|------|------|
| No surprise breaks | Blocks security patches |
| Deterministic builds | Cargo resolves exact versions anyway via lockfile |
| | Unconventional in Rust ecosystem |

**Best solution: A.** The tripwire test catches structural changes.
Exact pins are unnecessary because `Cargo.lock` already pins exact
versions. The tripwire test provides defense-in-depth.

---

## 8. `muted_fg` Derived Incorrectly From `d.muted`

`colors.rs:88` computes `muted_fg` as:

```rust
muted_fg: rgba_to_hsla(d.muted).blend(fg.opacity(0.7)),
```

This has two compounding problems:

1. **Semantic mismatch:** In native-theme, `d.muted` is documented as
   "Secondary/subdued text color" (a foreground). But gpui-component's
   `ThemeColor.muted` slot (line 135: `tc.muted = c.muted`) is
   documented as "Muted backgrounds such as Skeleton and Switch" —
   a background. So `d.muted` (text color) is being mapped to a
   background slot.

2. **Wrong derivation:** The `muted_fg` derivation blends `d.muted`
   (a text color) with semi-transparent foreground. On dark themes
   where `muted` is grayish text and `fg` is white, blending `fg` at
   0.7 opacity washes it out toward white, making "muted" text
   indistinguishable from regular text.

**Impact:** Two fields are wrong: `tc.muted` receives a text color
in a background slot, and `tc.muted_foreground` is derived from a
text-on-text blend that produces the wrong color on dark themes.

### Solutions

#### A. Use `d.muted` directly as `muted_fg` (recommended)

```rust
muted_fg: rgba_to_hsla(d.muted),
```

The native-theme model already resolves `muted` as the subdued text
color. No further derivation is needed.

| Pros | Cons |
|------|------|
| Correct: muted IS the muted foreground | May look slightly different from current gpui-component default |
| Simplest possible mapping | |
| Matches the theme author's intent | |

#### B. Blend `fg` toward `bg` for a muted effect

```rust
muted_fg: fg.blend(c.bg.opacity(0.4)),
```

| Pros | Cons |
|------|------|
| Derives muted from the actual foreground | Ignores the theme's explicit muted color |
| Consistent with some UI framework conventions | May not match the theme author's chosen muted color |

#### C. Keep current blending (status quo)

| Pros | Cons |
|------|------|
| No change | Muted text is too bright on dark themes |
| | Treats muted as a background, which it isn't |

**Best solution: A.** The theme provides a `muted` color specifically
designed as subdued text. Use it directly. The current blending
misinterprets the field's semantics.

---

## 9. `_light` Color Variants Wrong on Dark Themes

`colors.rs:304-328`: `red_light`, `green_light`, `blue_light`,
`yellow_light`, `magenta_light`, and `cyan_light` are all derived as:

```rust
tc.red_light = c.bg.blend(c.danger.opacity(0.8));
```

On light themes (white bg), this produces a pastel/lighter tint —
correct for a "_light" variant. On dark themes (e.g., bg with l=0.1),
blending at 0.8 opacity produces a dimmer mid-tone (e.g., l~0.42 for
a danger color with l=0.5) — not a "light" tint as the name implies.
The result is darker and muddier than the base status color.

**Impact:** Chart tooltips, syntax highlighting, or any UI using
`red_light`/`green_light` etc. get dimmer mid-tones on dark themes
instead of the lighter shades the name implies. Not near-black, but
noticeably wrong for use cases that expect pastel tints.

### Solutions

#### A. Use mode-aware derivation (recommended)

For dark themes, blend toward white instead of the background:

```rust
fn light_variant(bg: Hsla, color: Hsla, is_dark: bool) -> Hsla {
    if is_dark {
        // Lighter = more luminous: increase lightness
        Hsla { l: (color.l + 0.15).min(1.0), ..color }
    } else {
        // Lighter = more pastel: blend toward white bg
        bg.blend(color.opacity(0.8))
    }
}
```

| Pros | Cons |
|------|------|
| "_light" variants are actually lighter on both modes | Needs `is_dark` parameter in `assign_base_colors` |
| Correct for charts, syntax highlighting | Different derivation per mode adds complexity |
| Matches user expectation of "_light" | |

#### B. Always increase lightness (mode-independent)

```rust
tc.red_light = Hsla { l: (c.danger.l + 0.15).min(1.0), ..c.danger };
```

| Pros | Cons |
|------|------|
| Mode-independent — always lighter | May oversaturate already-light colors |
| Simple, predictable | Loses the pastel tint on light themes |

#### C. Keep current derivation (status quo)

| Pros | Cons |
|------|------|
| No change | Dimmer mid-tone "_light" variants on dark themes |
| Works on light themes | Semantic mismatch with the name |

**Best solution: A.** The derivation needs mode-awareness.
`assign_base_colors` should receive the `is_dark` flag (already
available in the calling code) and use different blend strategies
per mode.

---

## 10. `into_image_source()` Misleading API Documentation

`icons.rs:784-790` is documented as "the consuming variant of
`to_image_source()`. It takes ownership of the `IconData` to avoid
cloning the underlying `Vec<u8>`." But the implementation simply
delegates to `to_image_source(&data, color, size)` — it borrows `data`
and the inner code still copies bytes via `colorize_svg` and
`encode_rgba_as_bmp`. Ownership is taken but never exploited.

```rust
pub fn into_image_source(
    data: IconData,
    color: Option<Hsla>,
    size: Option<u32>,
) -> Option<ImageSource> {
    to_image_source(&data, color, size)
}
```

**Impact:** Users may choose `into_image_source` expecting a
performance benefit that does not exist.

### Solutions

#### A. Fix the doc comment to reflect reality (recommended)

```rust
/// Consuming convenience wrapper for [`to_image_source()`].
///
/// Takes ownership of the `IconData` for ergonomic use in
/// iterator chains where the data is not needed afterward.
/// Internally delegates to `to_image_source()`.
```

| Pros | Cons |
|------|------|
| Honest documentation | Doesn't actually optimize |
| No API change | Users may wonder why it exists |
| No behavior change | |

#### B. Actually optimize to avoid copies

Modify the conversion pipeline to take `Vec<u8>` by value and reuse
the allocation.

| Pros | Cons |
|------|------|
| Fulfills the doc's promise | Complex refactor of conversion pipeline |
| Real performance benefit for large SVGs | Must change to_image_source internals |
| | May not be possible for all code paths (SVG colorization creates new strings) |

#### C. Deprecate `into_image_source()` in favor of `to_image_source()`

| Pros | Cons |
|------|------|
| Eliminates the misleading function | Deprecation warning noise |
| Clean API | Breaking change for callers |

**Best solution: A.** Fix the docs. The function is still useful for
ergonomics (consuming in iterator chains). Document its actual
purpose honestly.

---

## 11. `to_theme()` Only Populates One Theme Mode

`lib.rs:116-122`: when `mode == ThemeMode::Dark`, only `theme.dark_theme`
is set to the native-theme-derived config; `theme.light_theme` retains
the default from `Theme::from(&theme_color)`. Vice versa for light.

If gpui-component's theme switching logic reads the opposite config
(e.g., `Theme.toggle_mode()`), it would snap to gpui-component's
built-in defaults rather than the native-theme-derived colors.

**Impact:** Users relying on gpui-component's built-in mode-switching
get one correct mode and one default mode.

### Solutions

#### A. Document the single-mode behavior (recommended)

Add a doc note that `to_theme()` configures only the requested mode
and that both light and dark must be built separately for full
toggle support:

```rust
/// Builds a gpui `Theme` for a single mode (light or dark).
///
/// Only the active mode's `ThemeConfig` is populated from the resolved
/// theme. The opposite mode retains gpui-component defaults. For full
/// light/dark toggle support, build both modes and manage them in your
/// application.
```

| Pros | Cons |
|------|------|
| Honest documentation | Doesn't fix the toggle behavior |
| No API change | Users must manage both modes manually |
| Clear contract | |

#### B. Accept both light and dark resolved variants

```rust
pub fn to_theme(light: &ResolvedThemeVariant, dark: &ResolvedThemeVariant) -> Theme
```

| Pros | Cons |
|------|------|
| Both modes fully populated | Breaking API change |
| Toggle works correctly | Callers must always provide both variants |
| | Not all themes have both variants |

#### C. Set both modes from the same resolved variant

Use the same colors for both light_theme and dark_theme.

| Pros | Cons |
|------|------|
| Toggle doesn't snap to defaults | Both modes look identical — wrong |
| | Violates light/dark contrast expectations |

**Best solution: A.** Document the behavior. Most apps manage theme
mode at a higher level (rebuilding the Theme when the user toggles).
Populating the unused mode with the same data would be wrong, and
requiring both variants adds complexity for the common case.

---

## 12. Font Weight From `ResolvedFontSpec` Never Mapped

`config.rs:22-38` and `lib.rs:96-122`: both `to_theme_config()` and
`to_theme()` map font `family` and `size` from `ResolvedFontSpec` but
completely ignore the `weight` field (CSS weight 100-900).

gpui itself supports `FontWeight`, so users who specify a custom weight
(e.g., 300 for light text, 700 for bold) in their theme TOML will have
it silently ignored.

**Impact:** Themes with non-default font weights render at the
default weight (400) instead of the specified weight.

### Solutions

#### A. Map weight to gpui's FontWeight (recommended)

gpui's theme configuration may not expose font weight directly, but
individual text rendering calls accept `FontWeight`. Document how
users should apply the weight:

```rust
/// Font weight from the resolved theme.
///
/// gpui's Theme does not have a global font weight field. Use this
/// value when rendering text:
/// ```ignore
/// let weight = FontWeight(resolved.defaults.font.weight as f32);
/// ```
pub fn font_weight(resolved: &ResolvedThemeVariant) -> u16 {
    resolved.defaults.font.weight
}
```

| Pros | Cons |
|------|------|
| Users can access the weight | Not automatic — users must apply it |
| No gpui Theme API dependency | More work for consumers |
| Documents the gap | |

#### B. Set ThemeConfig font weight if the field exists

Check if `ThemeConfig` has a font weight field and set it.

| Pros | Cons |
|------|------|
| Automatic if the field exists | gpui-component may not expose this |
| | Must verify gpui-component API |

#### C. Keep unmapped (status quo)

| Pros | Cons |
|------|------|
| No change | Custom font weights silently ignored |
| | No way for users to discover the weight value |

**Best solution: A.** Expose a `font_weight()` helper function
similar to the existing `button_padding()` and `font_family()`
helpers. This lets users apply the weight in their text rendering
code.

---

## 13. `ThemeConfig.radius` Loses Sub-Pixel Precision

`config.rs:32-33` converts radius to integer via `round() as usize`:

```rust
radius: Some(d.radius.round() as usize),
radius_lg: Some(d.radius_lg.round() as usize),
```

Meanwhile, `lib.rs:111-112` sets `theme.radius = px(d.radius)` using
the full float value. This means `Theme.radius` and
`ThemeConfig.radius` can disagree when the radius has a fractional
component (e.g., 4.5px becomes `px(4.5)` on Theme but `Some(5)` in
ThemeConfig).

Additionally, negative `radius` values (invalid but possible from a
malformed theme) would wrap to a very large number via `as usize`
(negative float to unsigned integer).

**Impact:** Inconsistency between Theme and ThemeConfig radius values.
If gpui-component reads ThemeConfig.radius for re-applying, it will
use a different value than Theme.radius.

### Solutions

#### A. Keep integer conversion, add negative guard (recommended)

The integer radius is fine for ThemeConfig (gpui-component uses integer
radii). Add a guard for negative values:

```rust
radius: Some(d.radius.max(0.0).round() as usize),
```

| Pros | Cons |
|------|------|
| Prevents negative wrap-around | Still loses fractional precision |
| One-character change | |
| Matches gpui-component's integer expectation | |

#### B. Use `f32` in ThemeConfig if gpui-component supports it

| Pros | Cons |
|------|------|
| Full precision preserved | gpui-component may not accept float radius |
| Consistent with Theme.radius | |

#### C. Keep current behavior (status quo)

| Pros | Cons |
|------|------|
| No change | Negative values wrap to huge usize |
| | Float/int disagreement between Theme and ThemeConfig |

**Best solution: A.** Clamping to 0.0 before rounding prevents the
negative wrap-around bug. The integer precision is acceptable since
gpui-component uses integer radii. The minor disagreement with
Theme.radius (float vs rounded integer) is documented.

---

## 14. `from_system()` Drops Inactive Variant

`lib.rs:179-185`: `from_system()` calls `sys.active()` to build the
theme, then moves either `sys.dark` or `sys.light` into the return
tuple. Since `SystemTheme` is consumed, the caller loses access to the
opposite variant.

Users needing both light and dark from the system must call
`SystemTheme::from_system()` directly and manage conversion manually.
This is not documented — the convenience API hides the limitation.

The same gap exists in the iced connector's `from_system()`.

**Impact:** API documentation gap. Users doing runtime theme switching
would discover this quickly and switch to `SystemTheme` directly.

### Solutions

#### A. Document the single-variant behavior (recommended)

```rust
/// Returns the active variant only (light or dark based on OS preference).
///
/// For access to both variants (e.g., runtime light/dark toggling),
/// use [`SystemTheme::from_system()`] directly and convert each
/// variant with [`to_theme()`].
```

| Pros | Cons |
|------|------|
| Clear documentation | Doesn't fix the API limitation |
| No API change | |
| Guides users to the right approach | |

#### B. Return both variants

```rust
pub fn from_system() -> Result<(Theme, ResolvedThemeVariant, ResolvedThemeVariant, bool)>
```

| Pros | Cons |
|------|------|
| Full access to both variants | Complex return type |
| No need to call SystemTheme directly | Breaking API change |
| | Some themes only have one variant |

#### C. Return `(Theme, SystemTheme)` so the user can access both

| Pros | Cons |
|------|------|
| User has full access | SystemTheme is already consumed |
| | Would need to clone or restructure |

**Best solution: A.** Document clearly. The convenience API is for
the common case (single-mode startup). Users who need both variants
have a clear path via `SystemTheme::from_system()`.

---

## 15. `Theme::change()` Can Clobber All 108 Color Fields

`lib.rs:116-121` stores a `ThemeConfig` in `theme.dark_theme` (or
`light_theme`) that contains font, radius, shadow, and name — but
**no color overrides**. The doc comment at line 92-94 explicitly warns:

> "All Theme fields are set explicitly — no `apply_config` call is used.
> This avoids the fragile apply-then-restore pattern where `apply_config`
> would overwrite all 108 color fields with defaults."

The initial `Theme::from(&theme_color)` at line 105 correctly sets all
108 fields. However, if gpui-component's `Theme::change()` or
`sync_system_appearance()` is ever invoked at runtime, it reads the
stored `ThemeConfig` and calls `apply_config()` — which, finding no
color overrides in the config, resets all 108 fields to gpui defaults.

**Impact:** High latent risk. The initial theme is correct, but any
runtime appearance-sync event destroys the entire color mapping. The
developer is aware (per the comment) but the stored config is still
vulnerable to external `apply_config` calls.

### Solutions

#### A. Populate ThemeConfig with color overrides (recommended)

gpui-component's `ThemeConfig` has a `colors: ThemeConfigColors` field
(schema.rs:64-65) with `Option<SharedString>` entries for every color.
Populate it from the computed `ThemeColor`:

```rust
let config = config::to_theme_config(resolved, name, mode);
// Also populate the colors field so apply_config preserves them
config.colors = theme_color_to_config_colors(&theme_color);
```

| Pros | Cons |
|------|------|
| apply_config preserves all colors | Larger ThemeConfig (~108 color strings) |
| Theme::change() works correctly | Must convert Hsla->hex for each field |
| Eliminates the latent risk | Verified: ThemeConfig.colors field exists |

#### B. Override `Theme::change()` to rebuild from resolved data

Instead of relying on ThemeConfig, re-run `to_theme()` on mode change.

| Pros | Cons |
|------|------|
| Correct by construction | Must store ResolvedThemeVariant in the Theme (or externally) |
| No dependency on ThemeConfig color support | More complex mode-switching code |

#### C. Document the limitation and advise against runtime mode switching

| Pros | Cons |
|------|------|
| No code change | Runtime events can still trigger the bug |
| Developer is already aware | |

**Best solution: A.** `ThemeConfig` does have a `colors` field
(verified in gpui-component schema.rs:64-65). Populating it with the
computed colors makes the stored config self-contained and safe
against any external `apply_config` call.

---

## 16. Chart Colors Indistinguishable for Gray/Neutral Accent Themes

`colors.rs:228-245` generates 5 chart colors by rotating the accent
hue:

```rust
tc.chart_2 = Hsla { h: (c.accent.h + 0.2) % 1.0, ..c.accent };
```

When the accent has very low saturation (e.g., a gray accent like
`Hsla { h: 0.0, s: 0.05, l: 0.5 }` ), all 5 chart colors have
distinct hues but are visually identical — hue has no perceptual
effect at near-zero saturation. Charts become unreadable because
all data series look the same color.

**Impact:** Charts in apps using gray/neutral themes have
indistinguishable data series.

### Solutions

#### A. Boost saturation for chart colors (recommended)

Apply a minimum saturation floor for chart palette entries:

```rust
let chart_s = c.accent.s.max(0.4); // ensure visible hue variation
tc.chart_1 = Hsla { h: c.accent.h, s: chart_s, ..c.accent };
tc.chart_2 = Hsla { h: (c.accent.h + 0.2) % 1.0, s: chart_s, ..c.accent };
```

| Pros | Cons |
|------|------|
| Charts are always distinguishable | Chart colors may not match theme aesthetic |
| Simple floor — no complex logic | Forced saturation on intentionally neutral themes |

#### B. Use a fixed chart palette when accent saturation is low

Switch to a predefined colorful palette when `accent.s < threshold`.

| Pros | Cons |
|------|------|
| Always distinct chart colors | Discontinuous behavior at the threshold |
| Preserves theme colors when accent is colorful | Must maintain a second palette |

#### C. Keep current derivation (status quo)

| Pros | Cons |
|------|------|
| No change | Indistinguishable chart colors on gray themes |
| Matches accent aesthetic | |

**Best solution: A.** A saturation floor ensures hue rotation produces
visually distinct colors. 0.4 is a reasonable minimum that preserves
the theme's lightness while making hue differences perceptible.

---

## 17. Neither Connector Maps `spacing`, `icon_sizes`, or `text_scale`

The `ResolvedThemeVariant` contains a full spacing scale (7 tiers:
`xxs` through `xxl`), per-context icon sizes (5 contexts: `toolbar`,
`small`, `large`, `dialog`, `panel`), and a text scale (4 entries:
`caption`, `section_heading`, `dialog_title`, `display`).

None of these are mapped by the gpui connector (or the iced connector).
Theme authors who set custom spacing or text scales will see no effect
when using either toolkit bridge.

**Impact:** Significant theme data is resolved and validated but
completely ignored by both connectors. Users must manually extract
these values from the `ResolvedThemeVariant`.

### Solutions

#### A. Expose helper functions (recommended)

Add public helper functions similar to the existing `button_padding()`
and `font_family()` helpers:

```rust
pub fn spacing(resolved: &ResolvedThemeVariant) -> &ResolvedSpacing { ... }
pub fn icon_size(resolved: &ResolvedThemeVariant, ctx: IconSizeContext) -> f32 { ... }
pub fn text_scale_entry(resolved: &ResolvedThemeVariant, entry: &str) -> &ResolvedTextScaleEntry { ... }
```

| Pros | Cons |
|------|------|
| Users can access all resolved data | Not automatic — users must apply values |
| No toolkit API dependency | More helper functions to maintain |
| Consistent with existing helper pattern | |

#### B. Map to ThemeConfig/Theme fields where possible

Set `ThemeConfig.spacing`, etc. if gpui-component supports them.

| Pros | Cons |
|------|------|
| Automatic if fields exist | gpui-component may not expose these |
| | Must verify upstream API |

#### C. Keep unmapped (status quo)

| Pros | Cons |
|------|------|
| No change | Theme authors' spacing/scale choices ignored |
| | Users must extract from ResolvedThemeVariant directly |

**Best solution: A.** Helper functions are the established pattern
in both connectors. They provide a clean API for accessing resolved
data without requiring toolkit-specific mapping.

---

## 18. Missing `Result` and `Rgba` Re-exports

`lib.rs:73-76` re-exports native-theme types for convenience but
omits `Result` and `Rgba`:

```rust
pub use native_theme::{
    AnimatedIcon, Error, IconData, IconProvider, IconRole, IconSet, ResolvedThemeVariant,
    SystemTheme, ThemeSpec, ThemeVariant, TransformAnimation,
};
```

Both `from_preset()` and `from_system()` return
`native_theme::Result<...>`. Without a re-export, callers must add
`native-theme` as a direct dependency just to name the `Result` type
in their error handling. The doc comment at line 72 says "so downstream
crates don't need native-theme as a direct dependency" — but this is
undermined by the missing `Result`.

The iced connector correctly re-exports both `Result` and `Rgba` at
line 83.

**Impact:** Users must add a redundant `native-theme` dependency to
use `?` with `from_preset()` or `from_system()`.

### Solutions

#### A. Add `Result` and `Rgba` to the re-export block (recommended)

```rust
pub use native_theme::{
    AnimatedIcon, Error, IconData, IconProvider, IconRole, IconSet, ResolvedThemeVariant,
    Result, Rgba, SystemTheme, ThemeSpec, ThemeVariant, TransformAnimation,
};
```

| Pros | Cons |
|------|------|
| Callers don't need direct native-theme dependency | Two more re-exports |
| Consistent with iced connector | |
| Fulfills the doc comment's promise | |

#### B. Keep current re-exports (status quo)

| Pros | Cons |
|------|------|
| No change | Callers need native-theme for Result type |
| | Doc comment is inaccurate |

**Best solution: A.** Two trivial additions that eliminate an
unnecessary dependency for downstream crates.

---

## 19. `from_preset()` Error Message Doesn't Indicate Which Variant Failed

`lib.rs:152-154`: the error message always says "has no light or dark
variant" regardless of which mode was requested:

```rust
let variant = spec.into_variant(is_dark).ok_or_else(|| {
    native_theme::Error::Format(format!("preset '{name}' has no light or dark variant"))
})?;
```

Since `into_variant(is_dark)` falls back from the preferred variant
to the alternate before returning `None`, the error only fires when
BOTH variants are missing (the ThemeSpec is empty). The message should
say "has no variants" rather than "no light or dark variant."

The iced connector has the identical misleading message at line 141.

**Impact:** Confusing error message. Low severity since empty presets
should never occur for bundled presets.

### Solutions

#### A. Fix the error message (recommended)

```rust
native_theme::Error::Format(format!("preset '{name}' has no variants (both light and dark are empty)"))
```

| Pros | Cons |
|------|------|
| Accurate error message | Trivial change |
| Explains what actually went wrong | |

#### B. Keep current message (status quo)

| Pros | Cons |
|------|------|
| No change | Misleading when both variants are missing |

**Best solution: A.** Fix the message in both connectors. The current
wording implies one variant exists when neither does.

---

## 20. Tab/Sidebar/Window Fields Bypass `ResolvedColors` Cache

`colors.rs:202-222` and `267-292`: `assign_tab_sidebar()` and
`assign_misc()` call `rgba_to_hsla()` directly on `resolved.*` fields
instead of using the pre-converted `ResolvedColors` struct (`c.*`):

```rust
tc.tab = rgba_to_hsla(resolved.tab.background);           // bypasses cache
tc.tab_active = rgba_to_hsla(resolved.tab.active_background);
// ...
tc.scrollbar_thumb = rgba_to_hsla(resolved.scrollbar.thumb);
tc.slider_bar = rgba_to_hsla(resolved.slider.track);
```

All other assign functions (`assign_core`, `assign_primary`,
`assign_secondary`, `assign_status`) use the `ResolvedColors` struct
consistently.

**Impact:** Design inconsistency. If a caching or correction step
were added to `ResolvedColors`, tab/window/scrollbar fields would not
benefit. Minor performance waste from redundant conversions.

### Solutions

#### A. Add tab/sidebar/misc fields to `ResolvedColors` (recommended)

Extend `ResolvedColors` with the additional per-widget fields:

```rust
struct ResolvedColors {
    // ... existing fields ...
    tab_bg: Hsla,
    tab_active_bg: Hsla,
    tab_active_fg: Hsla,
    // etc.
}
```

| Pros | Cons |
|------|------|
| Consistent conversion pattern | Larger ResolvedColors struct |
| Single conversion point for all colors | |
| All assign functions use the same source | |

#### B. Keep mixed approach (status quo)

| Pros | Cons |
|------|------|
| No change | Inconsistent conversion pattern |
| Works correctly | |

**Best solution: A.** Consistency matters for maintainability. All
color conversions should flow through a single pre-conversion point.

---

## 21. `bundled_icon_to_image_source` Copies Static Bytes Unnecessarily

`icons.rs:842-844`: the function copies statically-embedded SVG bytes
into a heap-allocated `Vec<u8>` only to borrow them immediately:

```rust
let svg = native_theme::bundled_icon_by_name(name, icon_set)?;  // &'static [u8]
let data = IconData::Svg(svg.to_vec());    // copies to Vec
to_image_source(&data, color, size)        // borrows &data
```

The `svg.to_vec()` allocation is unnecessary since `to_image_source`
only borrows the data. The static `&[u8]` could be passed directly
if `IconData` supported borrowed data or if the conversion pipeline
accepted `&[u8]`.

**Impact:** One unnecessary heap allocation per bundled icon load.
For 86 icons at startup, ~86 allocations of 1-5KB each.

### Solutions

#### A. Accept `&[u8]` directly in the conversion path (recommended)

Add a variant or helper that takes a borrowed slice:

```rust
fn svg_bytes_to_image_source(
    svg: &[u8], color: Option<Hsla>, size: Option<u32>,
) -> Option<ImageSource> { ... }
```

| Pros | Cons |
|------|------|
| Eliminates 86 allocations at startup | New internal function |
| Zero-copy for static data | Slightly different from IconData path |

#### B. Make `IconData` support borrowed data

```rust
enum IconData<'a> { Svg(&'a [u8]), ... }
```

| Pros | Cons |
|------|------|
| Unified API | Lifetime parameter propagates everywhere |
| Clean design | Breaking API change |

#### C. Keep `.to_vec()` (status quo)

| Pros | Cons |
|------|------|
| No change | 86 unnecessary allocations |
| Simple code | |

**Best solution: A.** A dedicated `svg_bytes_to_image_source` helper
avoids the allocation without changing the public API.

---

## 22. `colorize_svg()` Silently Discards Alpha Channel

`icons.rs:979-984`: the function converts `Hsla` to RGB hex,
discarding the alpha channel entirely. The iced connector documents
this limitation; the gpui connector does not.

```rust
let rgba: gpui::Rgba = color.into();
let r = (rgba.r.clamp(0.0, 1.0) * 255.0).round() as u8;
// ... alpha is never used
let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
```

**Impact:** Users passing a semi-transparent color expecting
translucent icons get fully opaque icons instead.

### Solutions

#### A. Document the alpha discard (recommended)

```rust
/// **Alpha channel is discarded** because SVG `fill` attributes accept
/// hex colors (#RRGGBB) which have no alpha component. For transparency,
/// use the `opacity` attribute on the SVG element.
```

| Pros | Cons |
|------|------|
| Honest documentation | No functional change |
| Matches iced connector's approach | |

#### B. Support alpha via `fill-opacity`

| Pros | Cons |
|------|------|
| Full color fidelity | Complex SVG manipulation |
| | Edge cases with existing fill-opacity |

**Best solution: A.** Alpha on icon colors is extremely rare.
Document the behavior for correctness.

---

---

## 23. `ResolvedColors.surface` Is Dead Code

`colors.rs`: the `ResolvedColors` struct has `#[allow(dead_code)]` on
the entire struct. Audit confirms that `surface` is computed at
initialization but never read by any `assign_*` function. The blanket
`#[allow(dead_code)]` masks this and would also mask any future field
that becomes unused.

### Solutions

#### A. Move `#[allow(dead_code)]` to individual fields only (recommended)

```rust
struct ResolvedColors {
    bg: Hsla,
    fg: Hsla,
    // ...
    #[allow(dead_code)]
    surface: Hsla,  // retained for future mapping
}
```

| Pros | Cons |
|------|------|
| Only known dead fields are silenced | Must annotate individual fields |
| New dead fields trigger compiler warnings | Slightly more verbose |

#### B. Remove `surface` field until needed

| Pros | Cons |
|------|------|
| No dead code | Must re-add when mapping surface |

#### C. Map `surface` to a ThemeColor field

| Pros | Cons |
|------|------|
| Eliminates dead code by using it | Must identify correct target |

**Best solution: A.** Per-field `#[allow(dead_code)]` is more precise
and catches future regressions.

---

## 24. 7 Resolved Defaults Fields Silently Ignored

Cross-referencing `ResolvedThemeDefaults` against the connector's usage
reveals 7 resolved defaults fields that are never mapped or exposed:

| Field | Purpose |
|-------|---------|
| `selection_foreground` | Text color over selection highlight |
| `selection_inactive` | Selection bg when unfocused |
| `disabled_foreground` | Disabled control text color |
| `frame_width` | Border/frame width in px |
| `disabled_opacity` | Opacity for disabled controls |
| `border_opacity` | Border alpha multiplier |
| `focus_ring_offset` | Gap between element and focus ring |

Of note: `disabled_foreground` and `disabled_opacity` are important for
accessibility.

### Solutions

#### A. Map where gpui-component supports, expose helpers for rest (recommended)

| Pros | Cons |
|------|------|
| Maps everything possible | Must audit gpui-component for fields |
| Helpers for the rest | More API surface |

#### B. Expose all as helper functions only

| Pros | Cons |
|------|------|
| Simple, consistent | Nothing automatic |

#### C. Keep unmapped (status quo)

| Pros | Cons |
|------|------|
| No change | 7 properties silently dropped |

**Best solution: A.** Map what gpui-component supports, expose helpers
for the rest.

---

## 25. Missing `line_height_multiplier()` Helper

The iced connector exposes `line_height_multiplier()` that returns
`resolved.defaults.line_height`. The gpui connector has no equivalent.
gpui's `Theme` has no global line-height field, so users must access
the resolved data directly — but there's no discovery path.

### Solutions

#### A. Add `line_height_multiplier()` helper (recommended)

| Pros | Cons |
|------|------|
| Consistent with iced connector | Not automatic |
| Trivial to implement | |

#### B. Keep unmapped (status quo)

| Pros | Cons |
|------|------|
| No change | Inconsistent with iced connector |

**Best solution: A.** Trivial helper, matches iced API.

---

## 26. Missing Padding/Geometry Helper Functions

The iced connector exposes `button_padding()`, `input_padding()`,
`border_radius()`, `border_radius_lg()`, and `scrollbar_width()`.
The gpui connector exposes none of these.

### Solutions

#### A. Add matching helper functions (recommended)

| Pros | Cons |
|------|------|
| Consistent with iced connector | More functions to maintain |
| Discovery path via IDE autocomplete | |

#### B. Document the available metrics only

| Pros | Cons |
|------|------|
| No new API surface | No IDE discovery |

**Best solution: A.** Match the iced connector's precedent.

---

## 27. `stroke="#000000"` and `stroke="#000"` Not Handled

Issue 3 covers `stroke="black"`. The same gap exists for the hex
variants: `stroke="#000000"` and `stroke="#000"`. All 6 explicit-black
patterns (3 fill + 3 stroke) should be handled together.

### Solutions

#### A. Add all 3 stroke-black patterns alongside fill-black (recommended)

```rust
svg_str = svg_str
    .replace("stroke=\"black\"", &stroke_hex)
    .replace("stroke=\"#000000\"", &stroke_hex)
    .replace("stroke=\"#000\"", &stroke_hex);
```

| Pros | Cons |
|------|------|
| Covers all explicit-black patterns | 3 more string replacements |
| Handles both fill and stroke consistently | |

#### B. Only add `stroke="black"` (as in issue 3)

| Pros | Cons |
|------|------|
| Covers the most common case | Misses hex-black strokes |

**Best solution: A.** Handle all 6 patterns for completeness.

---

## 28. Accessibility Properties Not Exposed

`ResolvedThemeDefaults` includes 4 accessibility properties:
- `text_scaling_factor` (f32)
- `reduce_motion` (bool)
- `reduce_transparency` (bool)
- `high_contrast` (bool)

None are mapped or exposed. The `with_spin_animation` and
`animated_frames_to_image_sources` docs mention checking
`prefers_reduced_motion()`, but the connector doesn't enforce this.

### Solutions

#### A. Expose helper functions for accessibility properties (recommended)

```rust
pub fn reduce_motion(resolved: &ResolvedThemeVariant) -> bool { ... }
pub fn text_scaling_factor(resolved: &ResolvedThemeVariant) -> f32 { ... }
```

| Pros | Cons |
|------|------|
| Users can check accessibility settings | Not automatic |
| Consistent with helper function pattern | |
| Documents available a11y properties | |

#### B. Keep unmapped (status quo)

| Pros | Cons |
|------|------|
| No change | Accessibility properties invisible |

**Best solution: A.** Expose helpers. Users decide how to apply them.

---

## Summary

| # | Issue | Severity | Best Fix | Effort |
|---|-------|----------|----------|--------|
| 1 | Animation frame timing bug | High | Fail entire animation on any frame error | Trivial |
| 2 | Icon conversion no error context | Low | Document failure cases | Trivial |
| 3 | SVG colorization gaps | Low | Add stroke="black" (see #27) | Trivial |
| 4 | Showcase 5867 lines | Low | Keep + extract icon loading | Medium |
| 5 | Hardcoded BMP DPI | Negligible | Keep (status quo) | None |
| 6 | Missing integration tests | Medium | Add pipeline + all-presets tests | Low |
| 7 | Dependency pinning | Low | Keep + tripwire test (already exists) | None |
| 8 | muted_fg derived incorrectly | Medium | Use d.muted directly | Trivial |
| 9 | _light colors wrong on dark themes | Medium | Mode-aware derivation | Low |
| 10 | into_image_source misleading docs | Low | Fix doc comment | Trivial |
| 11 | to_theme only populates one mode | Low | Document single-mode behavior | Trivial |
| 12 | Font weight never mapped | Low | Add font_weight() helper | Trivial |
| 13 | ThemeConfig radius precision | Low | Clamp negative before rounding | Trivial |
| 14 | from_system drops inactive variant | Low | Document single-variant behavior | Trivial |
| 15 | Theme::change clobbers colors | High | Populate ThemeConfig with colors | Low |
| 16 | Chart colors gray accent | Low | Saturation floor for charts | Trivial |
| 17 | Unmapped spacing/icons/text_scale | Medium | Add helper functions | Low |
| 18 | Missing Result/Rgba re-export | Medium | Add to re-export block | Trivial |
| 19 | from_preset error message wrong | Low | Fix message text | Trivial |
| 20 | Tab/sidebar bypass ResolvedColors | Low | Add fields to ResolvedColors | Low |
| 21 | Unnecessary Vec alloc for bundled icons | Low | Accept &[u8] directly | Low |
| 22 | colorize_svg alpha undocumented | Low | Document alpha discard | Trivial |
| 23 | ResolvedColors.surface is dead code | Low | Per-field #[allow(dead_code)] | Trivial |
| 24 | 7 defaults fields unmapped | Medium | Map where possible + helpers | Low |
| 25 | Missing line_height_multiplier() | Low | Add helper (match iced) | Trivial |
| 26 | Missing padding/geometry helpers | Low | Add helpers (match iced) | Low |
| 27 | stroke="#000000"/#000 not handled | Low | Add all 6 black patterns | Trivial |
| 28 | Accessibility properties not exposed | Medium | Add a11y helper functions | Trivial |
| 29 | `to_theme()` signature asymmetry with iced | Medium | Derive is_dark from resolved | Low (breaking) |
| 30 | Cargo.toml heavy features in prod deps | Medium | Gate behind feature flags | Low |
| 31 | `from_system()` ownership fragility | Low | Restructure to borrow-then-move | Trivial |
| 32 | Status foreground contrast risk | Medium | Add contrast check + fallback | Low |
| 33 | No `from_system()` consistency test | Low | Add theme-matches-resolved test | Trivial |
| 34 | ThemeConfig.colors not populated (extends #15) | High | Populate config colors field | Low |
| 35 | Platform features unconditional in Cargo.toml | Medium | Use target-conditional sections | Low |
| 36 | Doc examples use `.unwrap()` | Low | Use `?` operator | Trivial |
| 37 | overlay ignores reduce_transparency | Medium | Check a11y flag in overlay alpha | Trivial |
| 38 | Missing icon size parameter validation | Medium | Clamp to 1..512 | Trivial |
| 39 | RGBA-to-HSLA doesn't clamp floats | Low | Clamp before conversion | Trivial |
| 40 | SVG fill injection fragile to quoted `>` | Low | Document limitation | Trivial |
| 41 | list_active hardcoded opacity | Low | Mode-aware opacity | Trivial |
| 42 | Tests use only one preset | Medium | Test with multiple presets | Low |
| 43 | Duplicate test_resolved() helper across 3 modules | Low | Extract shared test fixture | Low |
| 44 | icon_name maps two roles to same IconName (Delete) | Low | Map TrashEmpty to Trash2 | Trivial |
| 45 | icon_name maps two roles to same IconName (CircleX) | Low | Map StatusError to ShieldAlert | Trivial |
| 46 | is_dark_background unused outside tests | Negligible | Remove or use in production | Trivial |
| 47 | focus_ring_width mapped but never exposed | Low | Add focus_ring_width() helper | Trivial |
| 48 | No tests for dark theme colors | Medium | Add dark variant color tests | Low |
| 49 | Doc comment coverage table may be stale | Low | Add tripwire test for field counts | Low |

---

## New Issues Found in Deep Review (v0.5.4 Audit, Round 1)

---

## 29. `to_theme()` Signature Asymmetry With Iced Connector

`lib.rs:96`: the gpui connector requires three parameters:
```rust
pub fn to_theme(resolved: &ResolvedThemeVariant, name: &str, is_dark: bool) -> Theme
```

The iced connector requires only two:
```rust
pub fn to_theme(resolved: &ResolvedThemeVariant, name: &str) -> Theme
```

The `is_dark` parameter is redundant — it should be derivable from
the `ResolvedThemeVariant` (e.g., background lightness < 0.5). This
forces callers to track an extra boolean and opens the door to a
split-brain bug where the caller passes `is_dark=true` for a
light-variant resolved theme.

### Solutions

#### A. Derive `is_dark` from the resolved variant (recommended)

| Pros | Cons |
|------|------|
| Eliminates caller error class | Breaking API change |
| Matches iced's simpler API | Heuristic could be wrong for extreme themes |
| Single source of truth | |

#### B. Keep `is_dark` but add validation

| Pros | Cons |
|------|------|
| No API break | Still asymmetric with iced |
| Catches contradictions | Adds runtime check |

#### C. Add `is_dark` metadata to `ResolvedThemeVariant` in core

| Pros | Cons |
|------|------|
| Unambiguous source of truth | Core library change required |
| Both connectors can use it | |

**Best solution: C long-term, A short-term.** Add `is_dark` to the
core model in v0.6.0. Until then, derive from background lightness.

---

## 30. Cargo.toml Pulls Heavy Features Into Production Dependencies

`Cargo.toml:17-25`: the gpui connector enables ALL features
unconditionally in `[dependencies]`:
```toml
native-theme = { workspace = true, features = [
    "material-icons", "lucide-icons", "system-icons",
    "svg-rasterize", "linux-async-io", "macos", "windows",
] }
```

The iced connector uses minimal features in `[dependencies]` and
only enables icon features in `[dev-dependencies]`. Every gpui user
pays the compile cost even if they only need `to_theme()`.

### Solutions

#### A. Move icon/rasterize features behind feature flags (recommended)

Gate the `icons` module behind `#[cfg(feature = "icons")]`:
```toml
[features]
icons = ["native-theme/material-icons", "native-theme/lucide-icons",
         "native-theme/system-icons", "native-theme/svg-rasterize"]
```

| Pros | Cons |
|------|------|
| Opt-in heavy features | Users of icons must enable feature |
| Matches iced pattern | More Cargo.toml complexity |
| Faster default compile | |

#### B. Keep all features unconditional

| Pros | Cons |
|------|------|
| Icons "just work" | Every user pays compile cost |

**Best solution: A.** Users who only need color mapping shouldn't
compile icon sets and SVG rasterization.

---

## 31. `from_system()` Ownership Fragility

`lib.rs:179-185`: the gpui version calls `sys.active()` (borrow)
before destructuring `sys` (move). This interleaving of borrows
and moves is correct but fragile — it relies on drop order and
could break if `to_theme()` ever stored a reference.

### Solutions

#### A. Restructure to match iced pattern (recommended)

Move variant first, then call `to_theme(&resolved, ...)`:
```rust
let sys = SystemTheme::from_system()?;
let is_dark = sys.is_dark;
let name = sys.name;
let resolved = if is_dark { sys.dark } else { sys.light };
let theme = to_theme(&resolved, &name, is_dark);
Ok((theme, resolved))
```

| Pros | Cons |
|------|------|
| Clearer ownership flow | Trivial refactor |
| Matches iced pattern | |
| No borrow/move interleaving | |

#### B. Keep current order

| Pros | Cons |
|------|------|
| No change | Fragile to future changes |

**Best solution: A.** Trivial and prevents future breakage.

---

## 32. Status Foreground Colors May Lack Contrast Against Status Backgrounds

`colors.rs:159-178`: maps `danger_foreground`, `success_foreground`,
etc. directly from the resolved theme. Platform-facts.md §2.1.4
warns:

> macOS, KDE, and GNOME provide the **normal body foreground** —
> suitable as text *alongside* a status indicator, **not** as text
> *on* a status-colored background. Windows provides a **contrast
> foreground for text on the status background**.

gpui-component's `danger_foreground` etc. are used as text rendered
ON status-colored backgrounds. On macOS/KDE/GNOME, this could be
near-black text on a dark-red background — unreadable.

### Solutions

#### A. Add contrast check and derive if needed (recommended)

Check if `danger_foreground` has sufficient contrast against
`danger`. If not, use white or black based on the status color's
lightness:

```rust
fn ensure_contrast(fg: Hsla, bg: Hsla) -> Hsla {
    if contrast_ratio(fg, bg) < 4.5 {
        if bg.l > 0.5 { Hsla::black() } else { Hsla::white() }
    } else { fg }
}
```

| Pros | Cons |
|------|------|
| Guarantees readability | May override theme author intent |
| Handles platform mismatch automatically | More complex derivation |

#### B. Always derive from status color lightness

| Pros | Cons |
|------|------|
| Always readable | Ignores platform-provided values |

#### C. Document the mismatch

| Pros | Cons |
|------|------|
| No code change | Poor default on 3/4 platforms |

**Best solution: A.** Respect the platform value when it has
sufficient contrast, override only when needed.

---

## 33. No `from_system()` Consistency Test

`from_system()` returns `(Theme, ResolvedThemeVariant)`. No test
verifies that the returned `ResolvedThemeVariant` is the same variant
used to build the `Theme`. The existing `from_system_returns_tuple`
test only checks basic non-default values.

### Solutions

#### A. Add a consistency test (recommended)

Verify that a key color (e.g., background) matches between the
Theme and the resolved variant.

| Pros | Cons |
|------|------|
| Catches return-value mismatches | May need access to theme internals |

#### B. Keep current tests

| Pros | Cons |
|------|------|
| No change | Latent risk |

**Best solution: A.** Simple regression protection.

---

## 34. ThemeConfig.colors Not Populated (Extends #15)

This extends issue #15 with a concrete detail: `config.rs:37` fills
`ThemeConfig.colors` with `ThemeConfigColors::default()` (all `None`).
The initial `Theme::from(&theme_color)` correctly sets all 108 fields,
but the stored `ThemeConfig` has zero color data.

gpui-component's `ThemeConfig` at `schema.rs:64-65` HAS a `colors:
ThemeConfigColors` field with `Option<SharedString>` entries for every
color. Populating it from the computed `ThemeColor` would make the
stored config self-contained and safe against external `apply_config`
calls.

### Solutions

#### A. Populate ThemeConfig.colors from ThemeColor (recommended)

Convert each of the 108 Hsla values to hex strings for the config:

```rust
fn theme_color_to_config_colors(tc: &ThemeColor) -> ThemeConfigColors {
    ThemeConfigColors {
        background: Some(hsla_to_hex(tc.background).into()),
        foreground: Some(hsla_to_hex(tc.foreground).into()),
        // ... all 108 fields
    }
}
```

| Pros | Cons |
|------|------|
| apply_config preserves all colors | 108 hex string conversions |
| Theme::change() works correctly | Larger ThemeConfig |
| Eliminates the latent #15 risk | |

#### B. Document the limitation

| Pros | Cons |
|------|------|
| No code change | Runtime events can still trigger the bug |

**Best solution: A.** The field exists in gpui-component. Use it.

---

## 35. Platform Features Unconditional in Cargo.toml

`Cargo.toml:17-25`: `macos` and `windows` features are enabled
unconditionally. On Linux, these compile macOS/Windows code paths
that are never used. The iced connector handles this correctly with
`[target.'cfg(...)'.dev-dependencies]`.

### Solutions

#### A. Move to target-conditional sections (recommended)

```toml
[target.'cfg(target_os = "linux")'.dependencies]
native-theme = { workspace = true, features = ["linux-async-io"] }

[target.'cfg(target_os = "macos")'.dependencies]
native-theme = { workspace = true, features = ["macos"] }
```

| Pros | Cons |
|------|------|
| Only compiles platform code for target | More Cargo.toml sections |
| Matches iced pattern | Must verify Cargo unifies correctly |

#### B. Keep unconditional

| Pros | Cons |
|------|------|
| Simpler Cargo.toml | Compiles dead platform code |

**Best solution: A.** Platform features should be conditional.

---

## 36. Doc Examples Use `.unwrap()`

`lib.rs:29-32`: crate-level doc examples use `.unwrap()`:
```rust
/// let nt = ThemeSpec::preset("catppuccin-mocha").unwrap();
```

Per the project's no-panic policy, doc examples should model correct
error handling. Users copying the example get code that panics on
error. The iced connector has the same pattern.

### Solutions

#### A. Use `?` operator in doc examples (recommended)

Wrap in `fn main() -> Result<...>` and use `?`:
```rust
/// fn main() -> native_theme::Result<()> {
///     let nt = ThemeSpec::preset("catppuccin-mocha")?;
/// ```

| Pros | Cons |
|------|------|
| Models correct error handling | Examples slightly more verbose |
| Consistent with no-panic policy | |

#### B. Keep `.unwrap()` with `ignore` tag

| Pros | Cons |
|------|------|
| Simpler examples | Poor role model |

**Best solution: A.** Use `?` to model idiomatic error handling.

---

## 37. `overlay` Opacity Ignores `reduce_transparency` Accessibility Setting

`colors.rs:265-272`: the overlay color hardcodes opacity per theme mode
(0.5 dark / 0.4 light) without checking `resolved.defaults.reduce_transparency`.
When `true`, the user's OS-level "reduce transparency" preference is
being actively ignored during color derivation — the value is available
in the resolved theme but not consulted.

```rust
tc.overlay = Hsla {
    h: shadow.h, s: shadow.s, l: shadow.l,
    a: if is_dark { 0.5 } else { 0.4 },  // never checks reduce_transparency
};
```

**Impact:** Users who enable "reduce transparency" in system settings
still see translucent overlays, making dialogs harder to read. This is
a concrete instance of issue #28 (accessibility properties not exposed),
but worse — the property is available and silently ignored.

### Solutions

#### A. Check `reduce_transparency` when setting overlay alpha (recommended)

```rust
let alpha = if resolved.defaults.reduce_transparency {
    1.0
} else if is_dark {
    0.5
} else {
    0.4
};
tc.overlay = Hsla { h: shadow.h, s: shadow.s, l: shadow.l, a: alpha };
```

| Pros | Cons |
|------|------|
| Respects OS accessibility preference | Opaque overlay may obscure content more |
| Uses already-available data | |
| Trivial code change | |

#### B. Keep hardcoded (status quo)

| Pros | Cons |
|------|------|
| No change | Ignores accessibility setting |
| | Available data unused |

**Best solution: A.** The resolved theme already provides the boolean.
A single branch respects the user's stated accessibility need.

---

## 38. Missing Size Parameter Validation in Icon Conversion

`icons.rs:745-750`: `to_image_source()` accepts `size: Option<u32>`
with no range validation. Extreme values cause problems:
- `Some(0)` → rasterization fails silently
- `Some(100_000)` → attempts 40 GB RGBA allocation
- `Some(u32::MAX)` → memory exhaustion

```rust
pub fn to_image_source(
    data: &IconData, color: Option<Hsla>, size: Option<u32>,
) -> Option<ImageSource> {
    let raster_size = size.unwrap_or(SVG_RASTERIZE_SIZE);
    // ... no bounds check on raster_size
}
```

### Solutions

#### A. Clamp to a reasonable range (recommended)

```rust
const SVG_MIN_SIZE: u32 = 1;
const SVG_MAX_SIZE: u32 = 512;

let raster_size = size
    .unwrap_or(SVG_RASTERIZE_SIZE)
    .clamp(SVG_MIN_SIZE, SVG_MAX_SIZE);
```

| Pros | Cons |
|------|------|
| Prevents OOM from extreme sizes | Silently clamps user input |
| Zero allocation overhead | 512 limit may be too low for some use cases |
| One-line fix | |

#### B. Return `None` for out-of-range sizes

| Pros | Cons |
|------|------|
| Caller knows the size was rejected | Breaks silent-fail convention of Option return |
| No silent clamping | |

#### C. Keep no validation (status quo)

| Pros | Cons |
|------|------|
| No change | OOM on large sizes |
| | Silent failure on size 0 |

**Best solution: A.** UI icons never exceed 512px. Clamping is safe
and prevents accidental resource exhaustion.

---

## 39. RGBA-to-HSLA Conversion Doesn't Clamp Float Values

`colors.rs:13-18`: `rgba_to_hsla()` passes floats directly to gpui's
RGBA type without clamping. If `Rgba::to_f32_array()` ever returns
values outside [0.0, 1.0] (due to platform rounding, arithmetic in
text scale computation, or malformed presets), the HSLA conversion
may produce invalid values (hue outside [0,1], saturation > 1, NaN).

```rust
fn rgba_to_hsla(rgba: native_theme::Rgba) -> Hsla {
    let [r, g, b, a] = rgba.to_f32_array();
    let gpui_rgba = gpui::Rgba { r, g, b, a };  // no bounds check
    gpui_rgba.into()
}
```

### Solutions

#### A. Clamp before conversion (recommended)

```rust
let gpui_rgba = gpui::Rgba {
    r: r.clamp(0.0, 1.0),
    g: g.clamp(0.0, 1.0),
    b: b.clamp(0.0, 1.0),
    a: a.clamp(0.0, 1.0),
};
```

| Pros | Cons |
|------|------|
| Prevents invalid HSLA downstream | Four extra clamp calls per color |
| Defensive against platform quirks | Masks upstream bugs |
| Called ~20 times per theme, not hot | |

#### B. Keep unclamped (status quo)

| Pros | Cons |
|------|------|
| No change | Out-of-range input produces invalid HSLA |
| Matches how iced connector does it | |

**Best solution: A.** The clamp cost is negligible for the robustness
gained. Same fix should be considered for the iced connector's
`palette::to_color()`.

---

## 40. SVG Fill Injection Fragile to Quoted `>` in Attributes

`icons.rs:1009-1027`: the fill injection code finds the `<svg` tag's
closing `>` with a naive `find('>')`. An SVG attribute containing a
literal `>` inside quotes (e.g., `<svg data-foo="a > b">`) would
match at the wrong position, truncating the tag.

```rust
if let Some(pos) = svg_str.find("<svg")
    && let Some(close) = svg_str[pos..].find('>')  // naive search
{
    let tag_end = pos + close;
    // ...
}
```

### Solutions

#### A. Document the limitation (recommended)

Bundled icon sets (Lucide, Material) never have quoted `>` in
attributes. Add a doc comment:

```rust
/// Note: This function uses naive string matching and assumes
/// well-formed, simple SVGs (no quoted `>` in attributes).
/// Bundled icon sets are validated to meet this requirement.
```

| Pros | Cons |
|------|------|
| Honest documentation | Doesn't fix the edge case |
| No complexity added | Third-party SVGs may still break |
| Matches the function's stated purpose ("monochrome icon sets") | |

#### B. Use a proper XML parser

| Pros | Cons |
|------|------|
| Handles all XML edge cases | Heavy dependency for simple string op |
| Robust for third-party SVGs | Over-engineered for bundled icons |

**Best solution: A.** The function already documents it's for
"monochrome icon sets." Strengthening that documentation is
proportional to the risk.

---

## 41. `list_active` Selection Color Uses Hardcoded Opacity Without Mode Awareness

`colors.rs:184-187`: the active list row highlight blends primary at
0.1 opacity, then applies 0.2 alpha — neither value adapts to theme
mode or primary/background contrast. On low-saturation or gray accent
themes, the selection highlight becomes nearly invisible.

```rust
tc.list_active = c.bg.blend(c.primary.opacity(0.1)).alpha(0.2);
```

### Solutions

#### A. Use mode-aware opacity (recommended)

```rust
let selection_opacity = if is_dark { 0.2 } else { 0.15 };
tc.list_active = c.bg.blend(c.primary.opacity(selection_opacity));
```

| Pros | Cons |
|------|------|
| Better contrast on dark themes | Changes current appearance |
| Removes double-opacity pattern | |
| Simpler to reason about | |

#### B. Keep hardcoded (status quo)

| Pros | Cons |
|------|------|
| No change | Invisible selection on gray themes |

**Best solution: A.** Mode-aware opacity produces consistent visual
weight across theme variants.

---

## New Issues Found in Deep Review (v0.5.4 Audit, Round 2)

---

## 42. All Color/Config Tests Use Only One Preset (catppuccin-mocha)

Every `test_resolved()` helper across `lib.rs`, `colors.rs`, and
`config.rs` loads only `catppuccin-mocha`. This means:

- No coverage of themes with extreme values (very low saturation,
  very high/low lightness, non-standard radii).
- The `_light` variant bug (#9) would not be caught because
  catppuccin-mocha's dark variant is never tested in colors.rs
  (all tests pass `is_dark: false` to `to_theme_color`).
- Regressions in other preset families (dracula, adwaita, kde-breeze)
  are invisible.

Issue #6 proposes an "all presets" integration test, but that only
validates that presets load without error. It does not validate that
the COLOR MAPPING produces correct values for diverse inputs.

### Solutions

#### A. Add parameterized color tests across multiple presets (recommended)

Test at minimum: one dark preset, one light preset, one neutral/gray
accent preset, and one with extreme values (large radius, unusual
saturation).

```rust
#[test]
fn color_mapping_works_across_presets() {
    for (name, is_dark) in &[
        ("catppuccin-mocha", true),
        ("catppuccin-latte", false),
        ("dracula", true),
        ("adwaita", false),
    ] {
        let (theme, resolved) = from_preset(name, *is_dark).unwrap();
        assert_ne!(theme.color.background, ThemeColor::default().background,
            "preset {name} background should differ from default");
    }
}
```

| Pros | Cons |
|------|------|
| Catches preset-specific regressions | Slower test suite |
| Validates dark AND light paths | Must pick representative presets |
| Catches the #9 bug | |

#### B. Add property-based tests with random Rgba inputs

| Pros | Cons |
|------|------|
| Catches edge cases no human would think of | Requires proptest dependency |
| Extremely thorough | Harder to debug failures |

#### C. Keep single-preset tests (status quo)

| Pros | Cons |
|------|------|
| No change | Blind to most of the color space |
| Fast | Cannot catch mode-specific bugs |

**Best solution: A.** A small set of representative presets (2-4)
catches the most important category: dark vs light mode behavior.
This would have caught issue #9 immediately.

---

## 43. Duplicate `test_resolved()` Helper Across 3 Modules

The exact same `test_resolved()` function is copy-pasted in three
test modules:

1. `lib.rs:217-225`
2. `colors.rs:338-346`
3. `config.rs:48-56`

All three load `catppuccin-mocha`, call `into_variant(false)`, and
resolve. The duplication means:
- If the test fixture needs updating (e.g., to test multiple presets
  per #42), all three copies must change.
- Different test modules could accidentally drift to use different
  presets, making test results non-comparable.

### Solutions

#### A. Extract to a shared test fixture module (recommended)

Create a `#[cfg(test)] mod test_fixtures` in a shared location (or
use a test utility function in lib.rs re-exported with `#[cfg(test)]`):

```rust
// In lib.rs or a dedicated test_utils.rs
#[cfg(test)]
pub(crate) fn test_resolved_light() -> ResolvedThemeVariant { ... }
#[cfg(test)]
pub(crate) fn test_resolved_dark() -> ResolvedThemeVariant { ... }
```

| Pros | Cons |
|------|------|
| Single source of truth for test fixtures | Must figure out module visibility |
| Easy to add dark/multi-preset variants | |
| DRY | |

#### B. Keep duplicated (status quo)

| Pros | Cons |
|------|------|
| No change | 3 copies to maintain |
| Each module is self-contained | Drift risk |

**Best solution: A.** Extract and deduplicate. Especially important
if issue #42 is addressed (adding more preset variants to the
fixture).

---

## 44. `icon_name()` Maps Two Roles to Same `IconName::Delete`

`icons.rs:88,108`: both `ActionDelete` and `TrashEmpty` map to
`IconName::Delete`:

```rust
IconRole::ActionDelete => IconName::Delete,
// ...
IconRole::TrashEmpty => IconName::Delete,
```

These are semantically different actions -- "delete" (destructive
action) vs "empty trash" (a state/place). Using the same icon loses
the distinction.

### Solutions

#### A. Map `TrashEmpty` to `IconName::Delete` with a `Trash2` note (recommended)

gpui-component 0.5 does not have a dedicated trash can icon. The
`Delete` mapping is the best available. Document the semantic
difference:

```rust
// Both ActionDelete and TrashEmpty map to Delete because
// gpui-component 0.5 has no dedicated trash can icon.
IconRole::TrashEmpty => IconName::Delete,
```

| Pros | Cons |
|------|------|
| Documents the intentional reuse | Still loses semantic distinction |
| No wrong icon | |

#### B. Return `None` for `TrashEmpty`

| Pros | Cons |
|------|------|
| Honest -- no icon for trash | 29 mappings instead of 30 |
| | TrashEmpty is a common role |

**Best solution: A.** The mapping is the best available. A code
comment prevents future maintainers from thinking it is a bug.

---

## 45. `icon_name()` Maps Two Roles to Same `IconName::CircleX`

`icons.rs:77,113`: both `DialogError` and `StatusError` map to
`IconName::CircleX`:

```rust
IconRole::DialogError => IconName::CircleX,
// ...
IconRole::StatusError => IconName::CircleX,
```

These are used in different contexts -- dialog error (large, prominent)
vs inline status indicator (small, subtle). Reuse is acceptable for
Lucide where both concepts use the same X-in-circle glyph, but it
means the two roles are visually indistinguishable.

### Solutions

#### A. Keep mapping, document the reuse (recommended)

Both concepts legitimately use the same "error circle" metaphor in
the Lucide set. No alternative Lucide icon would be more appropriate
for either role.

| Pros | Cons |
|------|------|
| Correct for Lucide | Cannot visually distinguish roles |
| No wrong icon | |

#### B. Map `StatusError` to a different icon (e.g., `XCircle`)

| Pros | Cons |
|------|------|
| Visual distinction | No better candidate in gpui-component 0.5 |

**Best solution: A.** This is a limitation of the icon set, not a
mapping bug. Document it in the coverage comment.

---

## 46. `is_dark_background()` Exists Only for Tests

`colors.rs:22-25`: `is_dark_background()` is marked `#[cfg(test)]`
and used only in one test (`is_dark_detects_dark_background`). The
function itself tests that lightness < 0.5 detects dark backgrounds.

The function is simple (one comparison) and its test is trivial.
Meanwhile, the actual production code at `to_theme_color()` takes
`is_dark` as a parameter rather than deriving it -- so this function
represents a dead design path.

### Solutions

#### A. Remove both the function and its test (recommended)

The function tests nothing that production code uses. It was likely
part of an earlier design where `is_dark` was derived from the
background color (matching issue #29's suggestion).

| Pros | Cons |
|------|------|
| Removes dead test code | Loses a trivial utility |
| Removes false sense of coverage | |

#### B. Keep for potential future use (status quo)

| Pros | Cons |
|------|------|
| Available if #29 is implemented | Dead code until then |

**Best solution: A** if issue #29 is NOT planned. **B** if #29 will
be implemented (the function would become production code).

---

## 47. `focus_ring_width` Mapped to `ring` But Width Not Exposed

`colors.rs:138`: the `ring` color is mapped from `d.focus_ring_color`:

```rust
tc.ring = c.ring;  // where c.ring = rgba_to_hsla(d.focus_ring_color)
```

But `ResolvedThemeDefaults` also has `focus_ring_width` (f32) and
`focus_ring_offset` (f32) which are never mapped or exposed. Issue #24
lists `focus_ring_offset` as unmapped but omits `focus_ring_width`.

gpui-component likely uses a fixed focus ring width. The native-theme
width varies significantly: macOS 3px, Windows 1-2px, KDE 1px+2px
margin, GNOME 2px (per platform-facts.md section 2.1.5). Not exposing
this means all platforms get the same focus ring width regardless of
their native convention.

### Solutions

#### A. Add `focus_ring_width()` and `focus_ring_offset()` helpers (recommended)

```rust
pub fn focus_ring_width(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.defaults.focus_ring_width
}
pub fn focus_ring_offset(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.defaults.focus_ring_offset
}
```

| Pros | Cons |
|------|------|
| Exposes platform-correct values | Not automatic |
| Consistent with helper function pattern | Users must apply manually |
| Covers accessibility-relevant geometry | |

#### B. Keep unmapped (status quo)

| Pros | Cons |
|------|------|
| No change | Focus ring always uses gpui-component default |
| | Platform-specific values lost |

**Best solution: A.** Especially important for accessibility. Focus
ring geometry varies significantly across platforms and the data is
already resolved.

---

## 48. No Tests Verify Dark Theme Color Derivations

In `colors.rs` tests, every test that calls `to_theme_color()` passes
`is_dark: false` (e.g., line 388, 424, 438, 487, 504, 519, 534, 547,
580, 601). Only one test (`is_dark_passed_not_derived` at line 503)
calls with `true`, and it only checks that `primary_active` differs
between modes -- it does not verify the actual colors are correct.

This means:
- `active_color` dark-mode derivation (20% darkening) is untested
  for correctness (only tested for being different from light).
- The overlay opacity (0.5 dark vs 0.4 light) is untested for the
  dark path.
- The `group_box` opacity difference (0.3 dark vs 0.4 light) is
  untested for the dark path.
- Any dark-mode-specific bug in the assign functions would be invisible.

### Solutions

#### A. Add dark theme color tests (recommended)

For each mode-aware derivation, add a test that verifies the dark
path produces expected values:

```rust
#[test]
fn dark_mode_overlay_uses_higher_alpha() {
    let resolved = test_resolved_dark();
    let tc = to_theme_color(&resolved, true);
    assert!((tc.overlay.a - 0.5).abs() < 0.01,
        "dark overlay alpha should be ~0.5, got {}", tc.overlay.a);
}
```

| Pros | Cons |
|------|------|
| Validates dark-mode-specific code paths | More tests to maintain |
| Would have caught #9 immediately | Need a dark preset fixture |
| Verifies all mode-aware branches | |

#### B. Keep light-only tests (status quo)

| Pros | Cons |
|------|------|
| No change | Dark mode code path untested |
| | Half the color derivation logic is blind |

**Best solution: A.** Dark mode is presumably the more commonly used
mode for catppuccin-mocha (it is a dark theme!). Testing it only in
light mode is backwards.

---

## 49. Doc Comment Coverage Table May Become Stale

`lib.rs:36-58` contains a detailed markdown table listing how many
fields are mapped per widget category (e.g., "button: 4 of 14").
These counts are not verified by any test. When new fields are added
to either side (native-theme adds a widget property, or
gpui-component adds a ThemeColor field), the table silently becomes
inaccurate.

The existing tripwire test at `colors.rs:620-630` checks the TOTAL
ThemeColor field count (108), but not the per-widget breakdown
documented in the table.

### Solutions

#### A. Add a doc-accuracy tripwire test (recommended)

A test that counts mapped vs total fields per category and asserts
against the documented numbers. When the numbers change, the test
fails and the doc must be updated:

```rust
#[test]
fn coverage_table_matches_actual_mapping() {
    // Count how many tab.* fields are set to non-default
    let resolved = test_resolved();
    let tc = to_theme_color(&resolved, false);
    let default = ThemeColor::default();
    let tab_mapped = [
        tc.tab != default.tab,
        tc.tab_active != default.tab_active,
        tc.tab_active_foreground != default.tab_active_foreground,
        tc.tab_bar != default.tab_bar,
        tc.tab_foreground != default.tab_foreground,
    ].iter().filter(|&&b| b).count();
    assert_eq!(tab_mapped, 5, "doc says 5 tab fields mapped");
}
```

| Pros | Cons |
|------|------|
| Catches doc/code drift | Brittle if defaults happen to match |
| Forces doc updates when mapping changes | Some mapped fields might equal default |
| Self-documenting | |

#### B. Remove the coverage table from docs

| Pros | Cons |
|------|------|
| No maintenance burden | Users lose useful coverage overview |

#### C. Keep table without test (status quo)

| Pros | Cons |
|------|------|
| No change | Table may become stale |

**Best solution: A.** A tripwire test for the documented per-category
counts catches doc/code drift. The existing 108-field tripwire proves
the pattern works.
