# v0.5.3 Review -- native-theme-iced (iced connector)

Verified against source code on 2026-03-31. Each chapter covers one
problem, lists all resolution options with full pro/contra, and
recommends the best solution. Pre-1.0 -- backward compatibility is
not a constraint.

**Companion documents:**
- [todo_v0.5.3_native-theme.md](todo_v0.5.3_native-theme.md)
- [todo_v0.5.3_native-theme-build.md](todo_v0.5.3_native-theme-build.md)
- [todo_v0.5.3_native-theme-gpui.md](todo_v0.5.3_native-theme-gpui.md)

---

## Summary

| # | Issue | Severity | Fix complexity |
|---|-------|----------|----------------|
| 1 | `to_theme()` clones entire `ResolvedThemeVariant` for 4 fields | Medium | Trivial |
| 2 | `from_preset()` clones variant unnecessarily | Low | Trivial |
| 3 | `from_preset()` error message is misleading | Low | Trivial |
| 4 | `animated_frames_to_svg_handles()` silently drops non-SVG frames | Low | Low |

---

## 1. to_theme() clones entire ResolvedThemeVariant for 4 fields

**File:** `connectors/native-theme-iced/src/lib.rs:83-97`

**What:** The theme creation function clones the entire
`ResolvedThemeVariant` into a closure that uses only 4 fields:

```rust
pub fn to_theme(
    resolved: &native_theme::ResolvedThemeVariant,
    name: &str,
) -> iced_core::theme::Theme {
    let pal = palette::to_palette(resolved);
    let resolved_clone = resolved.clone(); // deep clone

    iced_core::theme::Theme::custom_with_fn(name.to_string(), pal, move |p| {
        let mut ext = iced_core::theme::palette::Extended::generate(p);
        extended::apply_overrides(&mut ext, &resolved_clone);
        ext
    })
}
```

`apply_overrides()` (`extended.rs:17-25`) reads exactly 4 fields:
- `resolved.button.background`
- `resolved.button.foreground`
- `resolved.defaults.surface`
- `resolved.defaults.foreground`

The `ResolvedThemeVariant` contains 25 resolved widget structs, each
with 2-14 fields (colors, floats, bools, font specs with heap-
allocated `String` family names). The clone copies all of this to
capture 4 `Rgba` values (each 4 bytes, `Copy`).

### Options

**A. Clone only the 4 needed values (recommended)**

Extract the 4 `Rgba` values before the closure and move those
instead:

```rust
pub fn to_theme(
    resolved: &native_theme::ResolvedThemeVariant,
    name: &str,
) -> iced_core::theme::Theme {
    let pal = palette::to_palette(resolved);

    let btn_bg = resolved.button.background;
    let btn_fg = resolved.button.foreground;
    let surface = resolved.defaults.surface;
    let foreground = resolved.defaults.foreground;

    iced_core::theme::Theme::custom_with_fn(name.to_string(), pal, move |p| {
        let mut ext = iced_core::theme::palette::Extended::generate(p);
        ext.secondary.base.color = crate::palette::to_color(btn_bg);
        ext.secondary.base.text = crate::palette::to_color(btn_fg);
        ext.background.weak.color = crate::palette::to_color(surface);
        ext.background.weak.text = crate::palette::to_color(foreground);
        ext
    })
}
```

- Pro: Zero heap allocation (Rgba is Copy, 4 bytes each).
- Pro: Makes the data dependency explicit -- the closure captures
  exactly what it needs.
- Pro: Trivial change, 4 lines replace 1.
- Con: Inlines the `apply_overrides` logic into `to_theme()`. The
  `extended::apply_overrides()` function becomes unused unless
  called elsewhere.
- Con: If `apply_overrides` is extended in the future to use more
  fields, the closure must be updated to capture them too.

**B. Clone only the 4 values, keep apply_overrides**

Pass the 4 values to `apply_overrides` instead of the full resolved:

```rust
// extended.rs
pub fn apply_overrides(
    extended: &mut Extended,
    btn_bg: Rgba, btn_fg: Rgba, surface: Rgba, foreground: Rgba,
) { ... }
```

```rust
// lib.rs
let btn_bg = resolved.button.background;
...
Theme::custom_with_fn(name.to_string(), pal, move |p| {
    let mut ext = Extended::generate(p);
    extended::apply_overrides(&mut ext, btn_bg, btn_fg, surface, foreground);
    ext
})
```

- Pro: Same allocation benefit as Option A.
- Pro: Keeps `apply_overrides` as a reusable function.
- Con: Changes the `apply_overrides` signature (internal, non-public).
- Con: If more fields are added later, the parameter list grows.

**C. Use Arc<ResolvedThemeVariant> to share ownership**

```rust
let resolved = Arc::new(resolved.clone());
let resolved_ref = Arc::clone(&resolved);
Theme::custom_with_fn(name.to_string(), pal, move |p| { ... })
```

- Pro: Single heap allocation (the Arc) instead of a deep clone.
- Con: Still clones into the Arc on the first call.
- Con: Arc overhead (atomic refcount) for a closure that's created
  once and held for the theme's lifetime.
- Con: Does not make the 4-field dependency explicit.

**D. Keep status quo**

- Pro: No change; clone cost is modest for a one-time operation.
- Con: Unclear data dependency; future readers must trace through
  `apply_overrides` to understand what the clone is for.

### Recommendation

**Option A.** Inlining the 4 assignments is the cleanest solution.
The closure captures exactly what it needs (16 bytes of Copy data
instead of ~2KB of mixed heap/stack data). The intent is immediately
clear.

If `apply_overrides` is extended in a future version to use more
fields, use **Option B** instead to keep the override logic
centralized. However, the v0.6.0 geometry plan
([todo_v0.6.0_iced-full-theme-geometry.md](todo_v0.6.0_iced-full-theme-geometry.md))
explicitly states "Colors: No changes needed", so `apply_overrides`
is not expected to grow.

### Compatibility with v0.6.0 geometry plan

The v0.6.0 plan adds 33 custom style functions (button::primary,
container::bordered_box, etc.) that capture geometry fields (radius,
disabled_opacity, shadow, frame_width) from `&ResolvedThemeVariant`
into closures. These functions operate on top of iced's built-in
styles, which derive their colors from the Theme produced by
`to_theme()`. The two layers are independent:

- v0.5.3 optimizes the **color** layer (this issue)
- v0.6.0 adds the **geometry** layer (new style function modules)

Option A is fully compatible with v0.6.0 and establishes the right
pattern: v0.6.0 style functions should similarly capture only their
2-4 needed fields (e.g., `radius: f32`, `disabled_opacity: f32`)
rather than cloning the entire `ResolvedThemeVariant` into each
closure.

Note: `extended::apply_overrides()` remains as a public utility
after this change (callable by users who build custom iced Themes
with `custom_with_fn`), even though `to_theme()` no longer calls it
internally. The v0.6.0 doc's reference to the
`to_theme() -> apply_overrides()` pipeline should be updated to
reflect the inlined approach.

---

## 2. from_preset() clones variant unnecessarily

**File:** `connectors/native-theme-iced/src/lib.rs:108-119`

**What:** The `from_preset()` function uses `pick_variant()` (which
returns `&ThemeVariant`) and then clones the reference to call
`into_resolved()` (which takes `self` by value):

```rust
pub fn from_preset(
    name: &str,
    is_dark: bool,
) -> native_theme::Result<(iced_core::theme::Theme, native_theme::ResolvedThemeVariant)> {
    let spec = native_theme::ThemeSpec::preset(name)?;
    let variant = spec
        .pick_variant(is_dark)
        .ok_or_else(|| ...)?;
    let resolved = variant.clone().into_resolved()?;
    //             ^^^^^^^^^^^^^^^ unnecessary clone
    let theme = to_theme(&resolved, name);
    Ok((theme, resolved))
}
```

The `ThemeSpec::preset()` call already clones the cached preset, so
`spec` is owned. The crate provides `ThemeSpec::into_variant()` which
consumes the owned `ThemeSpec` and returns `Option<ThemeVariant>` by
value, avoiding the second clone:

```rust
let variant = spec
    .into_variant(is_dark)
    .ok_or_else(|| ...)?;
let resolved = variant.into_resolved()?;  // no clone needed
```

### Options

**A. Use into_variant() instead of pick_variant() + clone()
   (recommended)**

- Pro: Eliminates unnecessary clone of the entire ThemeVariant.
- Pro: Trivial change.
- Pro: Uses the API as designed (`into_variant()` doc: "Use this when
  you own the `ThemeSpec` and don't need it afterward").
- Con: None identified.

**B. Keep status quo**

- Pro: No change.
- Con: Unnecessary heap allocation for font family strings.

### Recommendation

**Option A.** The fix is trivial and uses the intended API.

---

## 3. from_preset() error message is misleading

**File:** `connectors/native-theme-iced/src/lib.rs:115`

**What:** The error message says "has no variants" but the actual
condition is "has no matching variant for the requested mode":

```rust
.ok_or_else(|| native_theme::Error::Format(
    format!("preset '{name}' has no variants")
))?;
```

`pick_variant()` / `into_variant()` returns `None` only when both
`light` and `dark` variants are `None`. Since both functions include
cross-fallback (dark falls back to light and vice versa), this truly
means "no variants at all" -- so the message is technically accurate.

However, a user seeing this error may think they selected the wrong
mode, when the real problem is that the ThemeSpec is empty. A more
precise message would help:

```rust
format!("preset '{name}' has no light or dark variant")
```

The gpui connector at `lib.rs:141` has the identical message.

### Options

**A. Improve the error message (recommended)**

```rust
.ok_or_else(|| native_theme::Error::Format(
    format!("preset '{name}' has no light or dark variant")
))?;
```

- Pro: Clearer about what's missing.
- Pro: One-line change.
- Con: Cosmetic improvement.

**B. Keep status quo**

- Pro: No change; the current message is technically correct.
- Con: Potentially confusing for users debugging empty preset files.

### Recommendation

**Option A.** Apply to both the iced connector (lib.rs:115) and the
gpui connector (lib.rs:141) for consistency.

---

## 4. animated_frames_to_svg_handles() silently drops non-SVG frames

**File:** `connectors/native-theme-iced/src/icons.rs:124-145`

**What:** The frame-based animation converter uses `filter_map` to
silently skip frames that cannot be converted to SVG handles:

```rust
pub fn animated_frames_to_svg_handles(anim: &AnimatedIcon) -> Option<AnimatedSvgHandles> {
    match anim {
        AnimatedIcon::Frames { frames, frame_duration_ms } => {
            let handles: Vec<_> = frames
                .iter()
                .filter_map(|f| to_svg_handle(f, None))
                .collect();
            if handles.is_empty() {
                None
            } else {
                Some(AnimatedSvgHandles {
                    handles,
                    frame_duration_ms: *frame_duration_ms,
                })
            }
        }
        _ => None,
    }
}
```

If a frame is `IconData::Rgba` (not SVG), `to_svg_handle()` returns
`None` and the frame is silently removed. The `frame_duration_ms`
remains unchanged, so an animation with mixed frame types plays
faster than intended (fewer frames times the same per-frame duration).

**Practical impact:** Low. Bundled loading indicators use
`AnimatedIcon::Transform` (spin), not `Frames`. The `Frames` variant
is for user-defined sprite-sheet animations, where all frames are
typically homogeneous SVG. Mixed or unconvertible frames are unlikely.

The gpui connector's `animated_frames_to_image_sources()` has the
same pattern (see gpui review issue #7).

### Options

**A. Document the filtering behavior (recommended)**

Add a doc comment noting that non-SVG frames are silently excluded:

```rust
/// Note: Only SVG frames are included. RGBA frames are silently
/// excluded because iced's `Svg` widget cannot render raster data.
/// The returned animation may have fewer frames than the input,
/// causing it to play faster. If all frames are non-SVG, returns
/// `None`.
```

- Pro: Users know what to expect.
- Pro: No behavioral change.
- Con: Doesn't fix the root cause.

**B. Return None if any frame is non-SVG**

- Pro: Explicit failure.
- Con: Too aggressive -- mixed animations are rare, and partial
  rendering is better than no rendering.

**C. Keep status quo**

- Pro: No change.
- Con: Undocumented behavior.

### Recommendation

**Option A.** Document the behavior. SVG-only filtering is the
correct design for a function named `animated_frames_to_svg_handles`.
The function name already implies SVG output; a doc comment
confirming that non-SVG frames are excluded completes the contract.
