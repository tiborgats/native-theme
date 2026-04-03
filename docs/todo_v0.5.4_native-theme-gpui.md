# v0.5.4 -- native-theme-gpui: Deep Audit

Independent critical analysis of the gpui connector crate.

**Crate stats:** 5 source files (3071 lines), 1 example (5867 lines).
92 unit tests total: lib.rs (11), colors.rs (15), config.rs (3), derive.rs (3), icons.rs (60).
All production code is `#![deny(clippy::unwrap_used)]` and `#![deny(clippy::expect_used)]`.
Test code uses `#[allow(clippy::unwrap_used, clippy::expect_used)]`.

### Test Inventory

**lib.rs -- 11 tests:**
1. `to_theme_produces_valid_theme` -- light mode theme has correct `is_dark()` flag
2. `to_theme_dark_mode` -- dark mode theme has correct `is_dark()` flag
3. `to_theme_applies_font_and_geometry` -- font family/size, mono font, radius, shadow mapped
4. `from_preset_valid_light` -- light preset loads without error
5. `from_preset_valid_dark` -- dark preset loads without error
6. `from_preset_returns_resolved` -- returned ResolvedThemeVariant has populated font size
7. `from_preset_invalid_name` -- nonexistent preset returns Err
8. `system_theme_ext_to_gpui_theme` -- SystemThemeExt trait produces matching is_dark
9. `from_system_does_not_panic` -- from_system() doesn't panic (result may be Err)
10. `from_system_returns_tuple` -- returned tuple has valid font size and is_dark
11. `from_system_matches_manual_path` -- convenience and manual paths agree on is_dark

**colors.rs -- 15 tests:**
1. `rgba_to_hsla_converts_red` -- red RGB has hue near 0
2. `rgba_to_hsla_converts_green` -- green RGB has hue near 0.333
3. `rgba_to_hsla_converts_blue` -- blue RGB has hue near 0.667
4. `to_theme_color_produces_nondefault` -- bg, fg, primary, danger, border differ from default
5. `is_dark_detects_dark_background` -- test-only function correctly classifies lightness
6. `hover_states_differ_from_base` -- primary_hover and danger_hover differ from base
7. `per_widget_fields_used` -- scrollbar, slider, progress, title bar, switch, caret match resolved
8. `accent_foreground_uses_theme_value` -- accent_fg comes from d.accent_foreground not d.fg
9. `is_dark_passed_not_derived` -- primary_active differs between is_dark=false and is_dark=true
10. `link_hover_differs_from_link` -- link_hover and link_active differ from link
11. `selection_not_clamped` -- selection uses theme value without alpha clamping
12. `chart_colors_have_hue_separation` -- all 5 chart hues are distinct, chart_1 matches accent
13. `magenta_uses_theme_saturation` -- magenta.s = min(accent.s, 0.85), magenta.l = accent.l
14. `overlay_uses_shadow_color` -- overlay hue and saturation match shadow color
15. `theme_color_field_count_tripwire` -- ThemeColor has exactly 108 Hsla fields

**config.rs -- 3 tests:**
1. `to_theme_config_from_resolved` -- all config fields populated from resolved defaults
2. `to_theme_config_dark_mode` -- dark mode sets ThemeMode::Dark
3. `font_size_is_not_converted_from_points` -- font size equals resolved value (no pt-to-px)

**derive.rs -- 3 tests:**
1. `hover_color_differs_from_base` -- hover result differs from input base color
2. `active_color_light_theme_darkens` -- light active has lower lightness than base
3. `active_color_dark_theme_darkens_more` -- dark active darkens more than light active

**icons.rs -- 60 tests (53 main + 7 linux-only freedesktop):**

Main module (53):
1. `all_icons_have_lucide_mapping` -- all 86 IconName variants have non-empty Lucide name
2. `all_icons_have_material_mapping` -- all 86 variants have non-empty Material name
3. `icon_name_dialog_warning_maps_to_triangle_alert` -- DialogWarning -> TriangleAlert
4. `icon_name_dialog_error_maps_to_circle_x` -- DialogError -> CircleX
5. `icon_name_dialog_info_maps_to_info` -- DialogInfo -> Info
6. `icon_name_dialog_success_maps_to_circle_check` -- DialogSuccess -> CircleCheck
7. `icon_name_window_close_maps` -- WindowClose -> WindowClose
8. `icon_name_action_copy_maps_to_copy` -- ActionCopy -> Copy
9. `icon_name_nav_back_maps_to_chevron_left` -- NavBack -> ChevronLeft
10. `icon_name_file_generic_maps_to_file` -- FileGeneric -> File
11. `icon_name_status_check_maps_to_check` -- StatusCheck -> Check
12. `icon_name_user_account_maps_to_user` -- UserAccount -> User
13. `icon_name_notification_maps_to_bell` -- Notification -> Bell
14. `icon_name_shield_returns_none` -- Shield has no Lucide mapping
15. `icon_name_lock_returns_none` -- Lock has no Lucide mapping
16. `icon_name_action_save_returns_none` -- ActionSave has no Lucide mapping
17. `icon_name_help_returns_none` -- Help has no Lucide mapping
18. `icon_name_dialog_question_returns_none` -- DialogQuestion has no Lucide mapping
19. `icon_name_maps_at_least_28_roles` -- at least 28 IconRole variants map to Some
20. `icon_name_maps_exactly_30_roles` -- exactly 30 of 42 IconRole variants map to Some
21. `to_image_source_svg_returns_bmp_rasterized` -- SVG produces BMP ImageSource
22. `to_image_source_rgba_returns_bmp_image_source` -- RGBA produces BMP ImageSource
23. `to_image_source_with_color` -- colorized SVG converts successfully
24. `to_image_source_with_custom_size` -- custom raster size converts successfully
25. `encode_rgba_as_bmp_correct_file_size` -- 4x4 BMP has correct byte count
26. `encode_rgba_as_bmp_starts_with_bm` -- BMP starts with "BM" magic bytes
27. `encode_rgba_as_bmp_pixel_order_is_bgra` -- RGBA input stored as BGRA in BMP
28. `encode_rgba_as_bmp_zero_width_returns_none` -- zero width rejected
29. `encode_rgba_as_bmp_zero_height_returns_none` -- zero height rejected
30. `encode_rgba_as_bmp_mismatched_length_returns_none` -- too few bytes rejected
31. `encode_rgba_as_bmp_oversized_length_returns_none` -- too many bytes rejected
32. `colorize_svg_replaces_fill_black` -- fill="black" replaced with hex
33. `colorize_svg_replaces_fill_hex_black` -- fill="#000000" replaced with hex
34. `colorize_svg_replaces_fill_short_hex_black` -- fill="#000" replaced with hex
35. `colorize_svg_current_color_still_works` -- currentColor replaced with hex
36. `colorize_svg_implicit_black_still_works` -- fill injected into root svg tag
37. `colorize_svg_non_utf8_returns_original` -- non-UTF-8 input returned unchanged
38. `colorize_self_closing_svg_produces_valid_xml` -- fill injected before / in self-closing tag
39. `into_image_source_svg_returns_some` -- consuming SVG variant returns Some
40. `into_image_source_rgba_returns_some` -- consuming RGBA variant returns Some
41. `into_image_source_with_color` -- consuming colorized SVG returns Some
42. `custom_icon_to_image_source_with_svg_provider_returns_some` -- custom SVG provider works
43. `custom_icon_to_image_source_with_empty_provider_returns_none` -- empty provider returns None
44. `custom_icon_to_image_source_with_color` -- custom provider with color works
45. `custom_icon_to_image_source_accepts_dyn_provider` -- Box<dyn IconProvider> accepted
46. `bundled_icon_lucide_returns_some` -- Lucide Search icon converts
47. `bundled_icon_material_returns_some` -- Material Search icon converts
48. `bundled_icon_freedesktop_returns_none` -- Freedesktop set returns None (not bundled)
49. `bundled_icon_with_color` -- bundled Lucide Check icon with color converts
50. `animated_frames_returns_sources` -- 3-frame animation produces 3 sources with correct timing
51. `animated_frames_transform_returns_none` -- Transform variant returns None
52. `animated_frames_empty_returns_none` -- empty frames returns None
53. `spin_animation_constructs_without_context` -- with_spin_animation needs no render context

Linux-only freedesktop_mapping_tests (7):
54. `all_86_gpui_icons_have_mapping_on_kde` -- all variants have KDE freedesktop name
55. `eye_differs_by_de` -- Eye maps differently for KDE vs GNOME
56. `freedesktop_standard_ignores_de` -- edit-copy is same for all DEs
57. `all_86_gpui_icons_have_mapping_on_gnome` -- all variants have GNOME freedesktop name
58. `xfce_uses_gnome_names` -- XFCE follows GNOME naming convention
59. `all_kde_names_resolve_in_breeze` -- all KDE names find icons in Breeze theme
60. `gnome_names_resolve_in_adwaita` -- all GNOME names find icons in Adwaita theme

---

## 1. All Tests Use a Single Preset, and Only Test Light Mode Colors

**Files:** `lib.rs:217-225`, `colors.rs:338-346`, `config.rs:48-56`

Every `test_resolved()` helper across all three test modules loads exactly one
preset: `catppuccin-mocha`. Furthermore, the colors.rs tests exclusively call
`to_theme_color(&resolved, false)` -- every single color-mapping test runs
with `is_dark: false`.

This is doubly wrong. Catppuccin Mocha IS a dark theme. The test helper calls
`into_variant(false)` which loads the light fallback variant. So the most
important code path -- dark theme color derivation for the primary design-target
preset -- is never tested.

**Exception:** `lib.rs:237-248` has a `to_theme_dark_mode` test that loads
catppuccin-mocha with `into_variant(true)` and passes `is_dark: true` to
`to_theme()`. But this only checks `theme.is_dark()` -- it does not test any
color derivation.

**Concrete consequences:**
- `active_color()` dark branch (20% darkening) is never tested for correctness
- `overlay` alpha 0.5 (dark path) is never tested
- `group_box` opacity 0.3 (dark path) is never tested
- `_light` color variants (issue #3) produce wrong results on dark themes but
  this was invisible because dark mode was never tested
- No coverage of themes with low saturation, unusual radii, or extreme lightness
- No coverage of any preset other than catppuccin-mocha (dracula, adwaita,
  kde-breeze, nord, etc.)

The existing `is_dark_passed_not_derived` test (colors.rs:503) only verifies
that `primary_active` DIFFERS between modes -- not that either value is correct.

### Solutions

#### A. Add multi-preset, dual-mode test coverage (recommended)

Create a shared fixture module and test representative presets in both modes:

```rust
#[cfg(test)]
pub(crate) fn test_resolved(name: &str, is_dark: bool) -> ResolvedThemeVariant {
    let spec = ThemeSpec::preset(name).expect("preset must exist");
    let variant = spec.into_variant(is_dark).expect("variant exists");
    variant.into_resolved().expect("resolves")
}

#[test]
fn dark_mode_color_derivations_are_correct() {
    for (preset, is_dark) in &[
        ("catppuccin-mocha", true),
        ("catppuccin-latte", false),
        ("dracula", true),
        ("adwaita", false),
    ] {
        let resolved = test_resolved(preset, *is_dark);
        let tc = to_theme_color(&resolved, *is_dark);
        let default = ThemeColor::default();
        assert_ne!(tc.background, default.background,
            "{preset} bg should differ from default");
        // Verify dark-mode-specific paths
        if *is_dark {
            assert!((tc.overlay.a - 0.5).abs() < 0.01,
                "{preset} dark overlay alpha should be ~0.5");
        }
    }
}
```

| Pros | Cons |
|------|------|
| Catches dark-mode bugs immediately | Slightly slower test suite |
| Tests the actual design-target mode for each preset | Must pick representative presets |
| Eliminates the duplicate test_resolved() helper | |
| Would have caught issue #3 | |

#### B. Add property-based fuzz tests with proptest

| Pros | Cons |
|------|------|
| Exercises the full color space | Requires proptest dependency |
| Finds edge cases humans miss | Harder to debug failures |
| Very thorough | Slow in CI |

#### C. Keep single-preset light-only tests

| Pros | Cons |
|------|------|
| No change | Dark mode entirely untested |
| | Most common preset's design-intent mode untested |
| | Future mode-dependent bugs invisible |

**Best solution: A.** This is the most critical test gap. At minimum, test
catppuccin-mocha in dark mode (its design-intent mode) and one light theme.
Extract the shared fixture to eliminate the triplicated `test_resolved()`.

---

## 2. `muted_fg` Semantic Mismatch and Wrong Derivation

**File:** `colors.rs:88`

```rust
muted_fg: rgba_to_hsla(d.muted).blend(fg.opacity(0.7)),
```

Two compounding problems:

1. **Semantic mismatch:** In native-theme, `d.muted` is documented as
   "Secondary/subdued text color" (resolved.rs:106) -- a FOREGROUND color.
   But gpui-component's `ThemeColor.muted` slot (theme_color.rs:86-87) is
   documented as "Muted backgrounds such as Skeleton and Switch" -- a BACKGROUND
   slot. So a text color is being written to a background slot at colors.rs:135
   (`tc.muted = c.muted`).

2. **Wrong derivation:** `muted_fg` blends `d.muted` (already a subdued text
   color) with 0.7-opacity foreground. On dark themes where `muted` is grayish
   and `fg` is white, this pushes the result TOWARD white, making "muted"
   foreground indistinguishable from regular foreground. The derivation actively
   defeats its own purpose.

**Impact:** `tc.muted` (background slot) receives a text color. `tc.muted_foreground`
is a blend of two foreground colors that produces too-bright text on dark themes.

### Solutions

#### A. Use `d.muted` directly as `muted_fg`, derive a background for `tc.muted` (recommended)

```rust
muted: rgba_to_hsla(d.surface), // or bg.blend(fg.opacity(0.1)) for a muted background
muted_fg: rgba_to_hsla(d.muted),
```

The native-theme `d.muted` IS the muted foreground. No further derivation is
needed. The `tc.muted` slot needs a background color, not `d.muted`.

| Pros | Cons |
|------|------|
| Correct semantic mapping for both fields | Changes both muted and muted_fg appearance |
| Uses theme author's intent for muted text | Must identify the right background source |
| Fixes dark theme readability | |

#### B. Use `d.muted` directly as `muted_fg`, keep `muted` as-is

```rust
muted_fg: rgba_to_hsla(d.muted),
// tc.muted = c.muted stays as rgba_to_hsla(d.muted) -- still a text color in a bg slot
```

| Pros | Cons |
|------|------|
| Fixes muted_fg derivation | tc.muted remains semantically wrong |
| Minimal change | Skeleton/Switch backgrounds use a text color |

#### C. Keep current blending

| Pros | Cons |
|------|------|
| No change | Muted text too bright on dark themes |
| | Text color in background slot |

**Best solution: A.** Fix both fields. `d.muted` maps to `muted_fg` (it IS
muted text). `tc.muted` needs a proper background -- `d.surface` or a derived
value from `d.background`.

---

## 3. `_light` Color Variants Wrong on Dark Themes

**File:** `colors.rs:302-329` (`assign_base_colors`)

```rust
tc.red_light = c.bg.blend(c.danger.opacity(0.8));
```

On light themes (bg lightness ~0.95), blending danger (l~0.5) at 0.8 opacity
produces a pastel tint (l~0.59) -- correct for a "_light" name.

On dark themes (bg lightness ~0.1), the same blend produces l~0.42 -- DIMMER
than the base danger color. The "_light" variant is darker than the original.
This affects all 6 `_light` colors: red, green, blue, yellow, magenta, cyan.

**Impact:** Chart tooltips, syntax highlighting backgrounds, and any UI using
`*_light` colors get dimmer mid-tones on dark themes instead of lighter tints.

Note: `assign_base_colors` does NOT receive the `is_dark` parameter, even
though the calling code has it available (colors.rs:122). This is the root
cause -- the function has no way to adapt its derivation.

### Solutions

#### A. Mode-aware derivation with `is_dark` parameter (recommended)

```rust
fn assign_base_colors(tc: &mut ThemeColor, c: &ResolvedColors, is_dark: bool) {
    fn light_variant(bg: Hsla, color: Hsla, is_dark: bool) -> Hsla {
        if is_dark {
            Hsla { l: (color.l + 0.15).min(0.95), ..color }
        } else {
            bg.blend(color.opacity(0.8))
        }
    }
    tc.red_light = light_variant(c.bg, c.danger, is_dark);
    // ...
}
```

| Pros | Cons |
|------|------|
| "_light" variants are actually lighter on both modes | Adds is_dark parameter to assign_base_colors |
| Correct for charts and syntax highlighting | Different derivation per mode |
| The calling code already has is_dark | |

#### B. Always increase lightness (mode-independent)

```rust
tc.red_light = Hsla { l: (c.danger.l + 0.15).min(0.95), ..c.danger };
```

| Pros | Cons |
|------|------|
| Simple, mode-independent | Loses the pastel tint on light themes |
| Always lighter than base | May oversaturate already-light colors |

#### C. Blend toward white instead of background

```rust
let white = Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 };
tc.red_light = white.blend(c.danger.opacity(0.3));
```

| Pros | Cons |
|------|------|
| Always produces a pastel tint | May not match theme aesthetic |
| Mode-independent | Fixed blend with white |
| Predictable result | |

**Best solution: A.** The function already sits in a call chain where `is_dark`
is available. Thread it through and use mode-appropriate derivation.

---

## 4. Animation Frame Timing Bug

**File:** `icons.rs:890-901`

`animated_frames_to_image_sources()` uses `filter_map` to convert frames,
silently dropping frames that fail conversion. But it preserves the original
`frame_duration_ms` unchanged. If 1 of 6 frames fails, the animation plays
5 frames at the original per-frame duration -- 17% faster than intended.

The doc comment at line 853-854 acknowledges this: "Frames that cannot be
converted ... are silently excluded. The returned animation may have fewer
frames than the input, causing it to play faster." But acknowledging a bug
in documentation does not fix it.

### Solutions

#### A. Fail the entire animation if any frame fails (recommended)

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
| Timing is always correct | Entire animation fails if one frame is bad |
| Simple, honest semantics | Less graceful degradation |
| Prevents glitchy partial animations | |

#### B. Adjust frame_duration_ms to compensate

```rust
let adjusted = frame_duration_ms * frames.len() as u32 / sources.len() as u32;
```

| Pros | Cons |
|------|------|
| Total duration preserved | Partial animation with skipped frames looks wrong |
| Graceful degradation | Uneven frame spacing |
| | Division by zero if all frames fail (guarded by existing empty check) |

#### C. Keep current behavior + log warning

| Pros | Cons |
|------|------|
| Makes failure visible | Still plays at wrong speed |
| Easy to add | Only helps debugging |

**Best solution: A.** Animation frames are a coherent sequence. Partial
playback at wrong speed is worse than no animation.

---

## 5. ThemeConfig.colors Not Populated -- Latent apply_config Bomb

**Files:** `lib.rs:116-121`, `config.rs:22-38`

The code stores a `ThemeConfig` in `theme.dark_theme`/`light_theme` that
contains font, radius, shadow, and name -- but NO color overrides. The
`..ThemeConfig::default()` at config.rs:37 fills `colors` with all `None`.

The initial `Theme::from(&theme_color)` at lib.rs:105 correctly sets all 108
color fields. But if gpui-component's `Theme::change()` or
`sync_system_appearance()` is invoked at runtime, it reads the stored
`ThemeConfig` and calls `apply_config()` internally -- which, finding no color
overrides in the config, RESETS all 108 fields to gpui-component defaults.

The developer is aware (per comment at lib.rs:92-94) but the stored config is
still vulnerable to external events.

gpui-component's `ThemeConfig` at schema.rs:64-65 HAS a `colors:
ThemeConfigColors` field with `Option<SharedString>` entries for every color.
Verified in gpui-component 0.5.1 at schema.rs:72-73.

### Solutions

#### A. Populate ThemeConfig.colors from the computed ThemeColor (recommended)

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
| apply_config preserves all colors | 108 hex string conversions (one-time at theme build) |
| Theme::change() works correctly | Larger ThemeConfig in memory |
| Eliminates the latent bomb | Must implement hsla_to_hex for config |
| ThemeConfig becomes self-contained | |

#### B. Override Theme::change() to rebuild from resolved data

| Pros | Cons |
|------|------|
| Correct by construction | Must store ResolvedThemeVariant externally |
| No dependency on ThemeConfig colors | Complex mode-switching architecture |

#### C. Document the limitation

| Pros | Cons |
|------|------|
| No code change | Runtime events can still trigger the bug |
| | Developer cannot prevent external apply_config calls |

**Best solution: A.** The field exists in gpui-component. Populate it. One-time
cost at theme construction prevents a correctness landmine at runtime.

---

## 6. `overlay` Opacity Ignores `reduce_transparency` Accessibility Setting

**File:** `colors.rs:267-273`

```rust
tc.overlay = Hsla {
    h: shadow.h, s: shadow.s, l: shadow.l,
    a: if is_dark { 0.5 } else { 0.4 },
};
```

The resolved theme carries `resolved.defaults.reduce_transparency` (bool) from
the OS accessibility settings (resolved.rs:176). This property is available but
actively ignored. Users who enable "reduce transparency" in system settings
still see translucent overlays.

Note: `assign_misc()` does receive `resolved` as a parameter (colors.rs:248),
so `resolved.defaults.reduce_transparency` is already accessible at this call
site.

### Solutions

#### A. Check reduce_transparency when setting overlay alpha (recommended)

```rust
let alpha = if resolved.defaults.reduce_transparency {
    1.0
} else if is_dark {
    0.5
} else {
    0.4
};
```

| Pros | Cons |
|------|------|
| Respects OS accessibility preference | Opaque overlay may obscure more content |
| Uses already-resolved data | Slight behavior change |
| Trivial one-branch addition | |

#### B. Keep hardcoded

| Pros | Cons |
|------|------|
| No change | Ignores user's stated accessibility need |
| | Property resolved but unused |

**Best solution: A.** The data is already there. A single branch respects
the user's OS-level setting.

---

## 7. `list_active` Selection Highlight Uses Hardcoded Double-Opacity Pattern

**File:** `colors.rs:187`

```rust
tc.list_active = c.bg.blend(c.primary.opacity(0.1)).alpha(0.2);
```

This applies opacity TWICE: first blending primary at 0.1 opacity onto bg,
then setting the result's alpha to 0.2. The double-opacity pattern produces
a nearly invisible selection highlight, especially on dark themes or with
gray/neutral accent colors.

Neither opacity adapts to theme mode or primary/background contrast.

### Solutions

#### A. Use mode-aware single-stage opacity (recommended)

```rust
fn assign_list_table(tc: &mut ThemeColor, c: &ResolvedColors, is_dark: bool) {
    let selection_opacity = if is_dark { 0.2 } else { 0.15 };
    tc.list_active = c.bg.blend(c.primary.opacity(selection_opacity));
    // ...
}
```

| Pros | Cons |
|------|------|
| Removes confusing double-opacity | Changes current appearance |
| Better contrast on dark themes | Needs is_dark parameter |
| Simpler to reason about | |

#### B. Keep double-opacity but adjust values

| Pros | Cons |
|------|------|
| Same pattern | Still confusing double application |
| Minor visual change | |

#### C. Keep current

| Pros | Cons |
|------|------|
| No change | Nearly invisible selection on gray themes |
| | Double-opacity is hard to reason about |

**Best solution: A.** Also requires adding `is_dark` to `assign_list_table`
signature (same pattern as issue #3).

---

## 8. Missing `Result` and `Rgba` Re-exports

**File:** `lib.rs:73-76`

The re-export block says "so downstream crates don't need native-theme as a
direct dependency" but omits `Result` and `Rgba`. Both `from_preset()` and
`from_system()` return `native_theme::Result<...>`. Without the re-export,
callers must add `native-theme` as a direct dependency just to name the Result
type.

The iced connector correctly re-exports both at its lib.rs:82-85.

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
| Fulfills the doc comment's promise | Two more items in the use statement |
| Consistent with iced connector | |
| Eliminates unnecessary downstream dependency | |

#### B. Keep current

| Pros | Cons |
|------|------|
| No change | Doc comment is inaccurate |
| | Callers need native-theme for Result |

**Best solution: A.** Trivial fix.

---

## 9. Status Foreground Colors May Lack Contrast Against Status Backgrounds

**Files:** `colors.rs:159-178`, `platform-facts.md:894-903`

The connector maps `danger_foreground`, `success_foreground`, etc. directly
from the resolved theme. platform-facts.md section 2.1.4 explicitly warns:

> macOS, KDE, and GNOME provide the **normal body foreground** -- suitable as
> text *alongside* a status indicator, **not** as text *on* a status-colored
> background. Windows provides a **contrast foreground for text on the status
> background**.

gpui-component's `danger_foreground` etc. are used as text rendered ON
status-colored backgrounds. On macOS/KDE/GNOME, this could be near-black text
on a dark-red background -- unreadable.

### Solutions

#### A. Add contrast check and fallback (recommended)

```rust
fn ensure_contrast(fg: Hsla, bg: Hsla) -> Hsla {
    // Simplified relative luminance contrast check
    let contrast = if bg.l > fg.l { bg.l - fg.l } else { fg.l - bg.l };
    if contrast < 0.3 {
        if bg.l > 0.5 {
            Hsla { h: 0.0, s: 0.0, l: 0.0, a: 1.0 }
        } else {
            Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 }
        }
    } else {
        fg
    }
}
```

| Pros | Cons |
|------|------|
| Guarantees readability | May override theme author's intent |
| Handles the 3/4 platform mismatch automatically | Simplified contrast check (not WCAG) |
| Respects platform value when it has sufficient contrast | More code in status assignment |

#### B. Always derive from status color lightness

```rust
let fg = if status.l > 0.5 { black } else { white };
```

| Pros | Cons |
|------|------|
| Always readable | Ignores all platform-provided values |
| Simple | Overrides even Windows's correct values |

#### C. Document the mismatch and leave to callers

| Pros | Cons |
|------|------|
| No code change | Poor default on 3 of 4 platforms |
| Matches what native-theme provides | |

**Best solution: A.** Respect the platform value when it works, override when
contrast is insufficient. This handles Windows correctly (its values already
have good contrast) while fixing macOS/KDE/GNOME.

**Limitation note:** The proposed `ensure_contrast` heuristic uses HSL
lightness difference, which is NOT perceptually uniform. Two colors with
0.31 lightness difference can still be illegible (e.g., saturated blue fg
on dark blue bg). A more robust approach would use WCAG relative luminance:
`L = 0.2126*R + 0.7152*G + 0.0722*B` (on linearized sRGB). At minimum,
document that the heuristic is approximate.

---

## 10. SVG Colorization Missing `stroke="black"` Patterns

**File:** `icons.rs:966-1031` (`colorize_svg()`)

The function handles `currentColor`, `fill="black"`, `fill="#000000"`,
`fill="#000"`, and implicit black (no fill on root `<svg>`). The
`currentColor` case handles both fill and stroke via blanket replacement.

Missing: `stroke="black"`, `stroke="#000000"`, `stroke="#000"`. These are
documented as not handled at line 975. Lucide uses `currentColor` for strokes
(covered), but third-party SVGs with explicit black strokes will not colorize.

### Solutions

#### A. Add all 6 explicit-black stroke patterns alongside fill (recommended)

```rust
// After the fill replacement block:
let stroke_hex = format!("stroke=\"{}\"", hex);
let replaced = replaced
    .replace("stroke=\"black\"", &stroke_hex)
    .replace("stroke=\"#000000\"", &stroke_hex)
    .replace("stroke=\"#000\"", &stroke_hex);
```

| Pros | Cons |
|------|------|
| Covers all practical explicit-black patterns | 3 more string operations |
| Handles both fill and stroke consistently | Still no CSS/rgb() (documented limitation) |
| Third-party SVGs work better | |

#### B. Only add `stroke="black"`

| Pros | Cons |
|------|------|
| Covers the most likely case | Misses hex-black strokes |
| Minimal change | Inconsistent with fill handling |

#### C. Keep current

| Pros | Cons |
|------|------|
| No change | Third-party stroke-based SVGs don't colorize |
| Works for bundled icon sets | |

**Best solution: A.** Handle all 6 patterns for consistency with the fill
patterns.

---

## 11. `into_image_source()` Documentation Promises Optimization That Doesn't Exist

**File:** `icons.rs:773-790`

The doc says "takes ownership of the `IconData` to avoid cloning the underlying
`Vec<u8>`" but the implementation just borrows and delegates:

```rust
pub fn into_image_source(data: IconData, ...) -> Option<ImageSource> {
    to_image_source(&data, color, size)
}
```

The inner code still copies bytes via `colorize_svg` and `encode_rgba_as_bmp`.
Ownership is taken but never exploited.

### Solutions

#### A. Fix the documentation to reflect reality (recommended)

```rust
/// Consuming convenience wrapper for [`to_image_source()`].
///
/// Takes ownership of the `IconData` for ergonomic use in
/// iterator chains where the data is not needed afterward.
/// Internally delegates to `to_image_source()` -- there is
/// no performance difference.
```

| Pros | Cons |
|------|------|
| Honest documentation | Doesn't add optimization |
| No API change | |

#### B. Actually optimize to avoid copies

| Pros | Cons |
|------|------|
| Fulfills the doc's promise | Complex refactor |
| Real performance benefit for large SVGs | May not be possible for all paths |

#### C. Deprecate the function

| Pros | Cons |
|------|------|
| Eliminates misleading API | Breaking change |
| Cleaner API surface | |

**Best solution: A.** Fix the docs. The function is still useful for
ergonomics in consuming contexts.

---

## 12. `to_theme()` Only Populates One Mode's ThemeConfig

**File:** `lib.rs:116-122`

When `mode == ThemeMode::Dark`, only `theme.dark_theme` is set to the
native-theme-derived config; `theme.light_theme` retains the default from
`Theme::from(&theme_color)`. Vice versa for light.

If gpui-component's `Theme::toggle_mode()` reads the opposite config, it snaps
to gpui-component's built-in defaults.

### Solutions

#### A. Document the single-mode behavior (recommended)

```rust
/// Builds a gpui `Theme` for a single mode (light or dark).
///
/// Only the requested mode's `ThemeConfig` is populated. For full
/// light/dark toggle, build both modes with separate `to_theme()`
/// calls and manage them in your application.
```

| Pros | Cons |
|------|------|
| Clear contract | Doesn't fix toggle behavior |
| No API change | |

#### B. Accept both resolved variants

| Pros | Cons |
|------|------|
| Both modes work | Breaking API change |
| Toggle works | Not all themes have both |

**Best solution: A.** Most apps rebuild the theme on mode change. Document
the expectation.

---

## 13. Font Weight Silently Ignored

**Files:** `config.rs:22-38`, `lib.rs:96-122`

Both `to_theme_config()` and `to_theme()` map font `family` and `size` from
`ResolvedFontSpec` but completely ignore the `weight` field (CSS weight
100-900). gpui supports `FontWeight` in rendering calls, but neither the Theme
nor ThemeConfig surface it.

### Solutions

#### A. Add `font_weight()` helper function (recommended)

```rust
/// Returns the resolved UI font weight (CSS 100-900 scale).
///
/// gpui's `Theme` has no global font weight field. Apply this when
/// rendering text: `FontWeight(resolved.defaults.font.weight as f32)`.
#[must_use]
pub fn font_weight(resolved: &ResolvedThemeVariant) -> u16 {
    resolved.defaults.font.weight
}
```

| Pros | Cons |
|------|------|
| Discoverable via IDE autocomplete | Not automatic |
| Documents the gap | |
| Consistent with helper pattern | |

#### B. Set ThemeConfig font weight if gpui-component supports it

| Pros | Cons |
|------|------|
| Automatic if field exists | Must verify gpui-component API |

#### C. Keep unmapped

| Pros | Cons |
|------|------|
| No change | Custom font weights silently lost |

**Best solution: A.** Helper function for discoverability.

---

## 14. `ThemeConfig.radius` Negative Value Wrap-Around

**File:** `config.rs:33-34`

```rust
radius: Some(d.radius.round() as usize),
```

If `d.radius` is negative (invalid but possible from a malformed theme), the
`as usize` cast wraps to a very large number. Also, `Theme.radius = px(d.radius)`
uses the full float while `ThemeConfig.radius` rounds to integer, creating
a minor disagreement.

### Solutions

#### A. Clamp before rounding (recommended)

```rust
radius: Some(d.radius.max(0.0).round() as usize),
radius_lg: Some(d.radius_lg.max(0.0).round() as usize),
```

| Pros | Cons |
|------|------|
| Prevents negative wrap-around | Still loses fractional precision |
| One-character addition per field | |
| Matches gpui-component integer expectation | |

#### B. Use saturating conversion

```rust
radius: Some((d.radius.round() as i64).max(0) as usize),
```

| Pros | Cons |
|------|------|
| Also prevents wrap | Slightly more verbose |

**Best solution: A.** Minimal and sufficient.

---

## 15. 8 Resolved Defaults Fields Silently Ignored

**File:** `colors.rs` (all assign_* functions)

Cross-referencing `ResolvedThemeDefaults` (resolved.rs:83-177) against the
connector's usage reveals these fields are never mapped or exposed:

| Field | Purpose | Available in gpui-component? |
|-------|---------|------------------------------|
| `selection_foreground` | Text color over selection highlight | No direct field |
| `selection_inactive` | Selection bg when unfocused | No direct field |
| `disabled_foreground` | Disabled control text color | No direct field |
| `frame_width` | Border width in px | No direct field |
| `disabled_opacity` | Opacity for disabled controls | No direct field |
| `border_opacity` | Border alpha multiplier | No direct field |
| `focus_ring_width` | Focus ring outline width | No direct field |
| `focus_ring_offset` | Gap between element and focus ring | No direct field |

Of note: `disabled_foreground` and `disabled_opacity` are accessibility-relevant.
`focus_ring_width` varies significantly across platforms (macOS 3px, Windows
1-2px, KDE 1px+2px, GNOME 2px per platform-facts.md section 2.1.5).

### Solutions

#### A. Add helper functions for all unmapped fields (recommended)

```rust
pub fn disabled_foreground(resolved: &ResolvedThemeVariant) -> Rgba { ... }
pub fn disabled_opacity(resolved: &ResolvedThemeVariant) -> f32 { ... }
pub fn focus_ring_width(resolved: &ResolvedThemeVariant) -> f32 { ... }
pub fn focus_ring_offset(resolved: &ResolvedThemeVariant) -> f32 { ... }
pub fn frame_width(resolved: &ResolvedThemeVariant) -> f32 { ... }
pub fn border_opacity(resolved: &ResolvedThemeVariant) -> f32 { ... }
pub fn selection_foreground(resolved: &ResolvedThemeVariant) -> Rgba { ... }
pub fn selection_inactive(resolved: &ResolvedThemeVariant) -> Rgba { ... }
```

| Pros | Cons |
|------|------|
| Discoverable via IDE | Not automatic |
| Consistent with iced connector's helper pattern | More functions to maintain |
| Documents what is available | |

#### B. Keep unmapped

| Pros | Cons |
|------|------|
| No change | 8 properties silently dropped |
| | Accessibility properties invisible |

**Best solution: A.** Especially important for accessibility-relevant fields.

---

## 16. Chart Colors Indistinguishable for Gray/Neutral Accent Themes

**File:** `colors.rs:225-245`

```rust
tc.chart_2 = Hsla { h: (c.accent.h + 0.2) % 1.0, ..c.accent };
```

When the accent has very low saturation (e.g., s=0.05), hue rotation has no
perceptual effect. All 5 chart colors look identical -- gray. Charts become
unreadable.

### Solutions

#### A. Enforce minimum saturation for chart colors (recommended)

```rust
let chart_s = c.accent.s.max(0.4);
tc.chart_1 = Hsla { s: chart_s, ..c.accent };
tc.chart_2 = Hsla { h: (c.accent.h + 0.2) % 1.0, s: chart_s, ..c.accent };
```

| Pros | Cons |
|------|------|
| Charts always distinguishable | Chart colors may not match neutral theme aesthetic |
| Simple floor, no complex logic | |
| Preserves theme lightness | |

#### B. Switch to fixed palette when saturation is low

| Pros | Cons |
|------|------|
| Always distinct | Discontinuous behavior at threshold |
| | Must maintain second palette |

**Best solution: A.** A saturation floor of 0.4 ensures hue differences are
visible while preserving the theme's lightness.

---

## 17. Spacing, Icon Sizes, and Text Scale Not Mapped or Exposed

The `ResolvedThemeVariant` carries:
- `defaults.spacing` (7-tier scale: xxs through xxl) -- resolved.rs:162
- `defaults.icon_sizes` (5 contexts: toolbar, small, large, dialog, panel) -- resolved.rs:166
- `text_scale` (4 entries: caption, section_heading, dialog_title, display) -- resolved.rs:191

None of these are mapped by the connector. Theme authors who set custom spacing
or text scales see no effect.

### Solutions

#### A. Add helper functions (recommended)

```rust
pub fn spacing(resolved: &ResolvedThemeVariant) -> &ResolvedThemeSpacing { ... }
pub fn icon_size_toolbar(resolved: &ResolvedThemeVariant) -> f32 { ... }
pub fn text_scale(resolved: &ResolvedThemeVariant) -> &ResolvedTextScale { ... }
```

| Pros | Cons |
|------|------|
| Users can access all resolved data | Not automatic |
| Consistent with helper pattern | More functions |
| IDE discoverable | |

#### B. Keep unmapped

| Pros | Cons |
|------|------|
| No change | Theme data resolved but invisible |

**Best solution: A.** Consistent with the helper pattern used for other
unmapped fields.

---

## 18. Cargo.toml Pulls Heavy Features Unconditionally

**File:** `Cargo.toml:17-25`

```toml
native-theme = { workspace = true, features = [
    "material-icons", "lucide-icons", "system-icons",
    "svg-rasterize", "linux-async-io", "macos", "windows",
] }
```

Every gpui user pays compile cost for ALL icon sets, SVG rasterization, and
all 3 platform backends -- even if they only need `to_theme()`.

The iced connector uses minimal features in `[dependencies]` and only enables
icon features in `[dev-dependencies]`.

Additionally, `macos` and `windows` features are enabled unconditionally. On
Linux, these compile code paths that are never used.

### Solutions

#### A. Gate behind feature flags (recommended)

```toml
[features]
default = ["icons"]
icons = ["native-theme/material-icons", "native-theme/lucide-icons",
         "native-theme/system-icons", "native-theme/svg-rasterize"]

[dependencies]
native-theme = { workspace = true }

[target.'cfg(target_os = "linux")'.dependencies]
native-theme = { workspace = true, features = ["linux-async-io"] }

[target.'cfg(target_os = "macos")'.dependencies]
native-theme = { workspace = true, features = ["macos"] }

[target.'cfg(target_os = "windows")'.dependencies]
native-theme = { workspace = true, features = ["windows"] }
```

| Pros | Cons |
|------|------|
| Users who only need colors don't compile icon sets | More Cargo.toml complexity |
| Platform features conditional | Must verify Cargo feature unification |
| Matches iced pattern | Users of icons must opt in (or default is fine) |

#### B. Keep all features unconditional

| Pros | Cons |
|------|------|
| Icons "just work" | Every user pays full compile cost |
| Simpler Cargo.toml | Compiles dead platform code on every platform |

**Best solution: A.** Default the icons feature on for backward compatibility
but allow opt-out.

---

## 19. `from_system()` Ownership Fragility

**File:** `lib.rs:179-185`

```rust
let sys = SystemTheme::from_system()?;
let is_dark = sys.is_dark;
let theme = to_theme(sys.active(), &sys.name, sys.is_dark);  // borrows sys
let resolved = if is_dark { sys.dark } else { sys.light };     // moves from sys
```

The code borrows `sys` (via `sys.active()`, `&sys.name`) and then moves from
it on the next line. This works in current Rust because the temporary borrows
end before the move, but it is fragile -- if `to_theme()` ever stored a
reference, it would break.

### Solutions

#### A. Move variant first, then call to_theme (recommended)

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
| Clear ownership flow | Trivial refactor |
| No borrow/move interleaving | |
| Matches iced pattern | |

#### B. Keep current order

| Pros | Cons |
|------|------|
| No change | Fragile to future changes |

**Best solution: A.** Trivial and prevents future breakage.

---

## 20. Tab/Sidebar/Window Fields Bypass `ResolvedColors` Cache

**Files:** `colors.rs:202-222`, `colors.rs:247-292`

`assign_tab_sidebar()` and `assign_misc()` call `rgba_to_hsla()` directly on
`resolved.*` fields instead of using the pre-converted `ResolvedColors` struct:

```rust
tc.tab = rgba_to_hsla(resolved.tab.background);           // bypasses cache
tc.scrollbar_thumb = rgba_to_hsla(resolved.scrollbar.thumb);
tc.slider_bar = rgba_to_hsla(resolved.slider.fill);
```

All other assign functions use `ResolvedColors` consistently. This is a design
inconsistency -- if a correction step were added to `ResolvedColors`, these
fields would not benefit.

### Solutions

#### A. Add per-widget fields to ResolvedColors (recommended)

| Pros | Cons |
|------|------|
| Consistent conversion pattern | Larger ResolvedColors struct (~15 more fields) |
| Single conversion point for all colors | |

#### B. Keep mixed approach

| Pros | Cons |
|------|------|
| No change | Inconsistent pattern |
| Works correctly | |

**Best solution: A.** Consistency matters for maintainability.

---

## 21. `colorize_svg()` Silently Discards Alpha Channel

**File:** `icons.rs:979-984`

```rust
let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
// alpha is never used
```

SVG `fill` attributes accept `#RRGGBB` which has no alpha. Users passing a
semi-transparent Hsla expecting translucent icons get fully opaque icons.
The iced connector documents this; the gpui connector does not.

### Solutions

#### A. Document the alpha discard (recommended)

Add to the `color` parameter doc:

```rust
/// **Note:** The alpha channel is discarded because SVG `fill` attributes
/// accept `#RRGGBB` (no alpha). For transparency, use the `opacity`
/// attribute on the rendered element.
```

| Pros | Cons |
|------|------|
| Honest documentation | No functional change |
| Matches iced connector approach | |

#### B. Support alpha via `fill-opacity`

| Pros | Cons |
|------|------|
| Full color fidelity | Complex SVG manipulation |

**Best solution: A.** Alpha on icon colors is extremely rare. Document it.

---

## 22. `ResolvedColors.surface` Is Dead Code Behind Blanket Allow

**File:** `colors.rs:34-65`

The `ResolvedColors` struct has `#[allow(dead_code)]` on the ENTIRE struct
(line 34). `surface` is computed at initialization (line 85) but never consumed
by any `assign_*` function. The blanket allow also masks any future field that
becomes unused.

### Solutions

#### A. Move `#[allow(dead_code)]` to individual fields only (recommended)

```rust
struct ResolvedColors {
    bg: Hsla,
    fg: Hsla,
    // ...
    #[allow(dead_code)]
    surface: Hsla,  // retained for future ThemeColor.surface mapping
}
```

| Pros | Cons |
|------|------|
| Only known dead fields are silenced | Per-field annotations |
| New dead fields trigger warnings | |

#### B. Remove `surface` until needed

| Pros | Cons |
|------|------|
| No dead code | Must re-add later |

#### C. Map `surface` to a ThemeColor field

| Pros | Cons |
|------|------|
| Eliminates dead code by using it | Must identify correct target |

**Best solution: A.** Per-field annotation is more precise. Note: if issue #2
is fixed with solution A (use `d.surface` for `tc.muted`), `surface` would no
longer be dead code and the allow attribute could be removed entirely.

---

## 23. `from_preset()` Error Message Inaccurate

**File:** `lib.rs:152-154`

```rust
native_theme::Error::Format(format!("preset '{name}' has no light or dark variant"))
```

Since `into_variant(is_dark)` falls back from preferred to alternate variant
before returning `None`, this error only fires when BOTH variants are missing.
The message should say "has no variants" not "no light or dark variant."

The iced connector has the identical message.

### Solutions

#### A. Fix the error message (recommended)

```rust
"preset '{name}' has no variants (both light and dark are empty)"
```

| Pros | Cons |
|------|------|
| Accurate error message | Trivial change |

#### B. Keep current

| Pros | Cons |
|------|------|
| No change | Misleading when both variants missing |

**Best solution: A.** Fix in both connectors.

---

## 24. `is_dark_background()` Is Dead Test Code

**File:** `colors.rs:22-25`

```rust
#[cfg(test)]
fn is_dark_background(bg: Hsla) -> bool {
    bg.l < 0.5
}
```

This function exists only in test code and is used by one test
(`is_dark_detects_dark_background` at colors.rs:406). Production code takes
`is_dark` as a parameter -- it never derives it from the background. This test
gives false coverage confidence by testing a function that production code
doesn't use.

### Solutions

#### A. Remove both the function and its test (recommended)

| Pros | Cons |
|------|------|
| Removes false coverage | Loses trivial utility |
| No dead test code | |

#### B. Keep for potential future use (if issue #25's `to_theme()` redesign
is planned)

| Pros | Cons |
|------|------|
| Available if is_dark derivation is added | Dead until then |

**Best solution: A** unless `to_theme()` will be redesigned to derive
`is_dark` from the resolved variant.

---

## 25. `to_theme()` Signature Asymmetry With Iced Connector

**File:** `lib.rs:96`

```rust
pub fn to_theme(resolved: &ResolvedThemeVariant, name: &str, is_dark: bool) -> Theme
```

The iced connector takes only `(resolved, name)`. The `is_dark` parameter is
redundant -- it should be derivable from the resolved variant's background
lightness. This forces callers to track an extra boolean and opens the door to
a split-brain bug where `is_dark=true` is passed for a light-variant resolved
theme.

### Solutions

#### A. Derive is_dark from the resolved variant (recommended)

```rust
pub fn to_theme(resolved: &ResolvedThemeVariant, name: &str) -> Theme {
    let bg = rgba_to_hsla(resolved.defaults.background);
    let is_dark = bg.l < 0.5;
    // ...
}
```

| Pros | Cons |
|------|------|
| Eliminates caller error class | Breaking API change |
| Matches iced connector | Heuristic may be wrong for extreme themes |
| Single source of truth | |

#### B. Add is_dark metadata to ResolvedThemeVariant in the core crate

| Pros | Cons |
|------|------|
| Unambiguous source of truth | Core library change |
| Both connectors benefit | Requires v0.6.0 |

#### C. Keep is_dark parameter, add validation

```rust
let bg_dark = rgba_to_hsla(resolved.defaults.background).l < 0.5;
debug_assert_eq!(is_dark, bg_dark, "is_dark contradicts background lightness");
```

| Pros | Cons |
|------|------|
| No API break | Still asymmetric |
| Catches contradictions in debug builds | |

**Best solution: B long-term (add metadata to core), A short-term.**

---

## 26. Missing Integration Tests for Full Pipeline

The crate has 92 unit tests but none that exercise the full end-to-end pipeline:
`ThemeSpec::preset()` -> `into_variant()` -> `into_resolved()` -> `to_theme()`
-> verify resulting Theme has expected non-default values.

The existing `from_preset_valid_light` test (lib.rs:270-273) only checks
`is_dark()` -- it does not verify that ANY color field was actually mapped.

### Solutions

#### A. Add pipeline round-trip test + all-presets smoke test (recommended)

```rust
#[test]
fn preset_to_theme_round_trip() {
    let (theme, resolved) = from_preset("dracula", true).unwrap();
    assert!(theme.is_dark());
    // Verify at least one color was mapped
    let bg = rgba_to_hsla(resolved.defaults.background);
    // Background should be populated and different from default
    assert!(bg.l < 0.3, "dracula bg should be dark");
}

#[test]
fn all_presets_produce_valid_themes() {
    for name in ThemeSpec::list_presets_for_platform() {
        for is_dark in [true, false] {
            if let Ok((theme, _)) = from_preset(name, is_dark) {
                let _ = theme.is_dark(); // should not panic
            }
        }
    }
}
```

| Pros | Cons |
|------|------|
| Catches integration bugs | Slower test suite |
| Validates primary user workflow | |
| all_presets catches regressions automatically | |

#### B. Keep current tests

| Pros | Cons |
|------|------|
| No change | No integration coverage |

**Best solution: A.** Both tests together catch mapping errors and preset
regressions.

---

## 27. `bundled_icon_to_image_source` Copies Static Bytes Unnecessarily

**File:** `icons.rs:842-844`

```rust
let svg = native_theme::bundled_icon_by_name(name, icon_set)?;  // &'static [u8]
let data = IconData::Svg(svg.to_vec());    // heap copy
to_image_source(&data, color, size)        // borrows &data
```

The `to_vec()` is unnecessary -- `to_image_source` only borrows the data.
For 86 icons at startup, this is 86 unnecessary heap allocations.

### Solutions

#### A. Add internal `svg_bytes_to_image_source` helper (recommended)

```rust
fn svg_bytes_to_image_source(
    svg: &[u8], color: Option<Hsla>, size: Option<u32>,
) -> Option<ImageSource> {
    let raster_size = size.unwrap_or(SVG_RASTERIZE_SIZE);
    if let Some(c) = color {
        let colored = colorize_svg(svg, c);
        svg_to_bmp_source(&colored, raster_size)
    } else {
        svg_to_bmp_source(svg, raster_size)
    }
}
```

| Pros | Cons |
|------|------|
| Zero-copy for static data | New internal function |
| Eliminates 86 allocations | |

#### B. Keep `.to_vec()`

| Pros | Cons |
|------|------|
| No change | Unnecessary allocations |
| Simple | |

**Best solution: A.** Clean optimization with no API change.

---

## 28. Missing Size Parameter Validation in Icon Conversion

**File:** `icons.rs:750`

```rust
let raster_size = size.unwrap_or(SVG_RASTERIZE_SIZE);
// no bounds check
```

Extreme values cause problems:
- `Some(0)` -> rasterization fails silently
- `Some(100_000)` -> attempts ~40 GB RGBA allocation
- `Some(u32::MAX)` -> memory exhaustion

### Solutions

#### A. Clamp to reasonable range (recommended)

```rust
const SVG_MIN_SIZE: u32 = 1;
const SVG_MAX_SIZE: u32 = 512;

let raster_size = size.unwrap_or(SVG_RASTERIZE_SIZE).clamp(SVG_MIN_SIZE, SVG_MAX_SIZE);
```

| Pros | Cons |
|------|------|
| Prevents OOM | Silently clamps |
| One-line fix | 512 may be too low for rare use cases |

#### B. Return None for out-of-range

| Pros | Cons |
|------|------|
| Caller knows rejection | Breaks Option convention |

**Best solution: A.** UI icons never exceed 512px.

---

## 29. RGBA-to-HSLA Conversion Doesn't Clamp Float Values

**File:** `colors.rs:15-19`

```rust
fn rgba_to_hsla(rgba: native_theme::Rgba) -> Hsla {
    let [r, g, b, a] = rgba.to_f32_array();
    let gpui_rgba = gpui::Rgba { r, g, b, a };  // no bounds check
    gpui_rgba.into()
}
```

If `to_f32_array()` returns values outside [0.0, 1.0], HSLA conversion may
produce invalid values (NaN, hue > 1.0, etc.).

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
| Prevents invalid HSLA | Four clamp calls per color (~30 colors) |
| Defensive against platform quirks | Masks upstream bugs |

#### B. Keep unclamped

| Pros | Cons |
|------|------|
| No change | Out-of-range input produces invalid HSLA |

**Best solution: A.** Negligible cost for robustness gained.

**Severity note:** `native_theme::Rgba` stores `u8` values (0-255) and
`to_f32_array()` at `color.rs:82-88` divides by 255.0, guaranteeing the
result is always in [0.0, 1.0]. The concern is theoretical only -- no
malformed input can reach the connector through the `ResolvedThemeVariant`
pipeline. Severity downgraded from **Low** to **Negligible**. The clamp
is still good defensive practice.

---

## 30. Doc Examples Use `.unwrap()`

**File:** `lib.rs:29-31`

```rust
/// let nt = ThemeSpec::preset("catppuccin-mocha").unwrap();
/// let variant = nt.into_variant(false).unwrap();
/// let resolved = variant.into_resolved().unwrap();
```

Per the project's `#![deny(clippy::unwrap_used)]` policy, doc examples should
model correct error handling. Users copying the example get code that panics.

Note: these are inside `/// ```ignore` blocks so they do not compile, but they
still serve as documentation that users will copy.

### Solutions

#### A. Use `?` operator in doc examples (recommended)

| Pros | Cons |
|------|------|
| Models correct error handling | Slightly more verbose |
| Consistent with no-panic policy | |

#### B. Keep `.unwrap()` with `ignore` tag

| Pros | Cons |
|------|------|
| Simpler examples | Poor role model |

**Best solution: A.**

---

## 31. `from_system()` Drops Inactive Variant Without Documentation

**File:** `lib.rs:179-185`

`from_system()` returns only the active variant. The caller loses access to the
opposite mode. Users needing both for runtime theme switching must call
`SystemTheme::from_system()` directly.

### Solutions

#### A. Document the limitation (recommended)

```rust
/// Returns the active variant only (light or dark based on OS preference).
///
/// For both variants (runtime toggling), use [`SystemTheme::from_system()`]
/// directly and convert each variant with [`to_theme()`].
```

| Pros | Cons |
|------|------|
| Clear guidance | No API change |

**Best solution: A.**

---

## 32. Accessibility Properties Not Exposed

`ResolvedThemeDefaults` includes 4 accessibility properties (resolved.rs:169-176):
- `text_scaling_factor` (f32)
- `reduce_motion` (bool)
- `reduce_transparency` (bool) -- partially addressed by issue #6
- `high_contrast` (bool)

The `with_spin_animation` and `animated_frames_to_image_sources` docs mention
checking `prefers_reduced_motion()`, but the connector doesn't enforce this.

### Solutions

#### A. Add helper functions (recommended)

```rust
pub fn reduce_motion(resolved: &ResolvedThemeVariant) -> bool { ... }
pub fn text_scaling_factor(resolved: &ResolvedThemeVariant) -> f32 { ... }
pub fn high_contrast(resolved: &ResolvedThemeVariant) -> bool { ... }
pub fn reduce_transparency(resolved: &ResolvedThemeVariant) -> bool { ... }
```

| Pros | Cons |
|------|------|
| Discoverable | Not enforced automatically |
| Documents available a11y data | |

**Best solution: A.**

---

## 33. `icon_name()` Maps Two Distinct Roles to `IconName::Delete`

**File:** `icons.rs:88,108`

Both `ActionDelete` and `TrashEmpty` map to `IconName::Delete`. These are
semantically different -- "delete action" vs "empty trash state/place."

### Solutions

#### A. Keep mapping with explanatory comment (recommended)

gpui-component 0.5 has no dedicated trash icon. `Delete` is the best available.

```rust
// ActionDelete and TrashEmpty both map to Delete because
// gpui-component 0.5 has no dedicated trash can icon.
IconRole::TrashEmpty => IconName::Delete,
```

| Pros | Cons |
|------|------|
| Documents intentional reuse | Loses semantic distinction |
| No wrong icon | |

**Best solution: A.**

---

## 34. SVG Fill Injection Fragile to Quoted `>` in Attributes

**File:** `icons.rs:1009-1011`

```rust
if let Some(pos) = svg_str.find("<svg")
    && let Some(close) = svg_str[pos..].find('>')  // naive search
```

An SVG attribute containing `>` inside quotes would match at the wrong position.

### Solutions

#### A. Document the limitation (recommended)

Bundled icon sets never have quoted `>` in attributes. Add a doc note.

| Pros | Cons |
|------|------|
| Honest documentation | Doesn't fix edge case |
| No complexity added | |

**Best solution: A.** The function is for monochrome icon sets, not arbitrary XML.

---

## 35. Doc Comment Coverage Table May Become Stale

**File:** `lib.rs:36-58`

The per-widget coverage table (e.g., "button: 4 of 14") is not verified by any
test. The 108-field tripwire at colors.rs:619-630 checks total count but not
per-widget breakdown.

### Solutions

#### A. Add per-category tripwire test (recommended)

```rust
#[test]
fn coverage_table_tab_count() {
    let resolved = test_resolved("catppuccin-mocha", true);
    let tc = to_theme_color(&resolved, true);
    let d = ThemeColor::default();
    let mapped = [
        tc.tab != d.tab, tc.tab_active != d.tab_active,
        tc.tab_active_foreground != d.tab_active_foreground,
        tc.tab_bar != d.tab_bar, tc.tab_foreground != d.tab_foreground,
    ].iter().filter(|&&b| b).count();
    assert_eq!(mapped, 5, "doc says 5 tab fields mapped");
}
```

| Pros | Cons |
|------|------|
| Catches doc/code drift | Brittle if mapped value equals default |
| Forces doc updates | |

**Best solution: A.**

---

## 36. Missing `line_height_multiplier()` Helper

The iced connector exposes `line_height_multiplier()`. The gpui connector has
no equivalent. gpui's Theme has no global line-height field.

### Solutions

#### A. Add helper (recommended)

```rust
pub fn line_height_multiplier(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.defaults.line_height
}
```

| Pros | Cons |
|------|------|
| Consistent with iced | Trivial function |

**Best solution: A.**

---

## 37. Missing Padding/Geometry Helper Functions

The iced connector exposes `button_padding()`, `input_padding()`,
`border_radius()`, `scrollbar_width()`. The gpui connector has none.

### Solutions

#### A. Add matching helpers (recommended)

| Pros | Cons |
|------|------|
| Consistent with iced | More API surface |
| IDE discoverable | |

**Best solution: A.**

---

## 38. Hardcoded BMP DPI Value

**File:** `icons.rs:1076-1077`

Hardcodes DPI to 72 (2835 pixels/meter) in BMP header metadata. This is
metadata-only -- gpui uses pixel dimensions directly.

### Solutions

#### A. Keep 72 DPI (recommended)

Zero visual impact. Not worth changing.

| Pros | Cons |
|------|------|
| No change needed | Technically imprecise metadata |

**Best solution: A (status quo).**

---

## 39. `from_system()` Consistency Test Missing

`from_system()` returns `(Theme, ResolvedThemeVariant)`. No test verifies
the returned resolved variant is the one used to build the Theme.

### Solutions

#### A. Add consistency test (recommended)

| Pros | Cons |
|------|------|
| Catches return-value mismatches | May need theme internals access |

**Best solution: A.**

---

## 40. Showcase Example is 5867 Lines

`examples/showcase.rs` is a comprehensive designer reference tool. Its size
is inherent to its purpose (all 108 color fields, 86 icons, all widgets).

### Solutions

#### A. Keep as single file (recommended)

Rust examples must be single files by Cargo convention (unless restructured
as `examples/showcase/main.rs`). The showcase is a reference tool, not a
"how to use" example.

| Pros | Cons |
|------|------|
| No restructuring needed | Large single file |
| Works with `cargo run --example showcase` | |

**Best solution: A.** The size is justified by the purpose.

---

## 41. `ALL_ICON_NAMES` Has No Compile-Time Exhaustiveness Check

**File:** `icons.rs:1114-1201`

The test constant `ALL_ICON_NAMES` lists 86 `IconName` variants manually.
If `gpui_component` adds a new variant, the `lucide_name_for_gpui_icon`
and `material_name_for_gpui_icon` match arms get a compile error
(non-exhaustive match) -- that is good. However, `ALL_ICON_NAMES` would
silently remain at 86, so the "all_icons_have_lucide_mapping" test would
appear to pass while not covering the new variant.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add count tripwire test: `assert_eq!(ALL_ICON_NAMES.len(), 86)` | Alerts when list goes stale; trivial | Must manually update count on change |
| B | Use `strum` to derive variant iteration | Automatic | Requires `gpui_component` to derive `EnumIter`, which it doesn't |

**Recommended:** A. A simple count assertion catches the most likely failure
mode and complements the existing match exhaustiveness.

---

## 42. `tab_bar_segmented` Uses `c.secondary` Without Explanation

**File:** `colors.rs:208`

```rust
tc.tab_bar_segmented = c.secondary;
```

All other tab fields are mapped from `resolved.tab.*` per-widget values.
`tab_bar_segmented` uses the button secondary color instead. There is no
`segmented` field in `ResolvedTabTheme`, so this is the only available
fallback. However, there is no comment explaining why.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add explanatory comment | Documents the design decision | No functional change |

**Recommended:** A. One-line comment.

---

## 43. `scrollbar_show` and `highlight_theme` Not Set in `to_theme()`

**Files:** `lib.rs:96-123`, `colors.rs:228-244`

`Theme` has 5 fields beyond colors/font/radius that `to_theme()` never sets:

| Field | Default | Resolved source |
|---|---|---|
| `scrollbar_show` | `Scrolling` | `resolved.scrollbar.overlay_mode` |
| `highlight_theme` | `default_light()` | Should use `is_dark` |
| `tile_grid_size` | `px(8.)` | No direct field |
| `tile_shadow` | `true` | No direct field |
| `tile_radius` | `px(0.)` | No direct field |

**Critical:** `highlight_theme` is always `default_light()` even for dark
themes. Code syntax highlighting shows light-mode colors on dark backgrounds
-- unreadable. And `scrollbar_show = Scrolling` on all platforms ignores
the per-platform `overlay_mode` boolean.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Map `scrollbar_show` from `overlay_mode`; set `highlight_theme` from `is_dark` | Correct behavior per platform; readable syntax highlighting | Must decide overlay -> ScrollbarShow mapping |
| B | Add helper functions only | Discoverable | Not automatic |

**Recommended:** A. Set `highlight_theme` based on `is_dark` and
`scrollbar_show` based on `overlay_mode`.

---

## 44. `ThemeConfig.highlight` Not Populated (Compounds Issue #5)

**File:** `config.rs:23-38`

`to_theme_config()` leaves `highlight: None`. When `apply_config()` runs
(issue #5 scenario), it does NOT update `highlight_theme`, so the wrong
light-mode default from issue 43 persists even through config application.

**Recommended:** Set `highlight` based on `is_dark` alongside fix for #43.

---

## 45. 19 of 30 Icon Role Mappings Lack Individual Regression Tests

**File:** `icons.rs:1203-1365`

The count test (`icon_name_maps_exactly_30_roles`) passes if any 30 roles
map to `Some(anything)`. Only 11 of 30 have spot-check tests verifying
the specific `IconName` target. Missing tests for: `WindowMinimize`,
`WindowMaximize`, `WindowRestore`, `ActionDelete`, `ActionUndo`,
`ActionRedo`, `ActionSearch`, `ActionSettings`, `ActionAdd`,
`ActionRemove`, `NavForward`, `NavUp`, `NavDown`, `NavMenu`,
`FolderClosed`, `FolderOpen`, `TrashEmpty`, `StatusBusy`, `StatusError`.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add data-driven test with `[(IconRole, IconName)]` table | Catches any mapping regression; single test | Must list all 30 pairs |
| B | Add 19 individual tests | Matches existing style | 19 functions |

**Recommended:** A. One data-driven test covering all 30 pairs.

---

## 46. `StatusError`/`DialogError` Both Map to `CircleX` (Undocumented)

**File:** `icons.rs:77,113`

Same pattern as issue #33 (`ActionDelete`/`TrashEmpty` -> `Delete`). These
are semantically different but gpui-component has no separate icon.

**Recommended:** Add explanatory comment, same approach as issue #33.

---

## 47. `mono_font_weight()` Helper Missing (Iced Has It)

**File:** `lib.rs` (absent)

The iced connector exposes both `font_weight()` and `mono_font_weight()`
at `lib.rs:248,254`. The gpui connector's issue #13 mentions only the UI
font weight.

**Recommended:** Add both `font_weight()` and `mono_font_weight()` helpers.

---

## 48. `DialogButtonOrder` Not Re-exported

**File:** `lib.rs:73-79`

`dialog.button_order` is critical for correct dialog layout but
`DialogButtonOrder` is not in the re-export block. Users must depend on
`native-theme` directly.

**Recommended:** Add to re-exports + add `dialog_button_order()` helper.

---

## 49. `from_system()` Resolves Both Variants, Discards One

**File:** `lib.rs:179-185`

`SystemTheme::from_system()` resolves both light and dark. `from_system()`
discards the unused one. Wasted resolution cost on every call.

**Recommended:** Document cost for v0.5.4; consider `from_system_active_only()`
in core for v0.6.0.

---

## 50. Issue #1 Refinement: `test_resolved()` Tests Wrong Preset's Colors

`into_variant(false)` on catppuccin-mocha loads the LIGHT fallback variant,
which is catppuccin-latte. Every test assertion about "catppuccin-mocha
colors" is actually testing catppuccin-latte values. This is worse than
just "testing light mode" -- it is testing a different preset entirely.

---

## Priority Summary

| # | Issue | Severity | Effort | Best Fix |
|---|-------|----------|--------|----------|
| 1 | All tests single-preset light-only (+ wrong preset colors) | **High** | Low | Multi-preset dual-mode tests + shared fixture |
| 2 | muted_fg semantic mismatch + wrong derivation | **High** | Low | Use d.muted as muted_fg, derive bg for tc.muted |
| 3 | _light colors wrong on dark themes | **High** | Low | Mode-aware derivation with is_dark parameter |
| 4 | Animation frame timing bug | **High** | Trivial | Fail entire animation on frame error |
| 5 | ThemeConfig.colors not populated (apply_config bomb) | **High** | Medium | Populate config colors from ThemeColor |
| 6 | overlay ignores reduce_transparency | **Medium** | Trivial | Check a11y flag in overlay alpha |
| 7 | list_active hardcoded double-opacity | **Medium** | Trivial | Mode-aware single-stage opacity |
| 8 | Missing Result/Rgba re-export | **Medium** | Trivial | Add to re-export block |
| 9 | Status foreground contrast risk | **Medium** | Low | Contrast check + fallback |
| 10 | SVG colorization missing stroke patterns | **Low** | Trivial | Add 6 stroke-black patterns |
| 11 | into_image_source misleading docs | **Low** | Trivial | Fix doc comment |
| 12 | to_theme only populates one mode | **Low** | Trivial | Document single-mode behavior |
| 13 | Font weight never mapped | **Low** | Trivial | Add font_weight() helper |
| 14 | ThemeConfig radius negative wrap | **Low** | Trivial | Clamp to 0.0 before rounding |
| 15 | 8 defaults fields unmapped | **Medium** | Low | Add helper functions |
| 16 | Chart colors gray accent | **Low** | Trivial | Saturation floor for charts |
| 17 | Spacing/icons/text_scale not exposed | **Medium** | Low | Add helper functions |
| 18 | Cargo.toml heavy features unconditional | **Medium** | Low | Feature-gate icon/platform features |
| 19 | from_system ownership fragility | **Low** | Trivial | Move variant before calling to_theme |
| 20 | Tab/sidebar bypass ResolvedColors | **Low** | Low | Add per-widget fields to ResolvedColors |
| 21 | colorize_svg alpha undocumented | **Low** | Trivial | Document alpha discard |
| 22 | ResolvedColors.surface dead code | **Low** | Trivial | Per-field #[allow(dead_code)] |
| 23 | from_preset error message wrong | **Low** | Trivial | Fix message text |
| 24 | is_dark_background dead test code | **Negligible** | Trivial | Remove function and test |
| 25 | to_theme() signature asymmetry with iced | **Medium** | Low (breaking) | Derive is_dark from resolved |
| 26 | Missing integration tests | **Medium** | Low | Pipeline + all-presets tests |
| 27 | bundled_icon Vec alloc unnecessary | **Low** | Low | Accept &[u8] directly |
| 28 | Missing icon size validation | **Medium** | Trivial | Clamp to 1..512 |
| 29 | RGBA-to-HSLA no float clamp | **Negligible** | Trivial | Clamp before conversion |
| 30 | Doc examples use .unwrap() | **Low** | Trivial | Use ? operator |
| 31 | from_system drops inactive variant | **Low** | Trivial | Document behavior |
| 32 | Accessibility properties not exposed | **Medium** | Trivial | Add helper functions |
| 33 | icon_name two roles to same Delete | **Low** | Trivial | Add comment |
| 34 | SVG fill injection fragile to quoted > | **Low** | Trivial | Document limitation |
| 35 | Coverage table may become stale | **Low** | Low | Per-category tripwire test |
| 36 | Missing line_height_multiplier() | **Low** | Trivial | Add helper |
| 37 | Missing padding/geometry helpers | **Low** | Low | Add helpers matching iced |
| 38 | Hardcoded BMP DPI | **Negligible** | None | Keep (status quo) |
| 39 | from_system consistency test missing | **Low** | Trivial | Add test |
| 40 | Showcase 5867 lines | **Negligible** | None | Keep (justified by purpose) |
| 41 | `ALL_ICON_NAMES` no exhaustiveness check | **Low** | Trivial | Add count tripwire test |
| 42 | `tab_bar_segmented = c.secondary` unexplained | **Negligible** | Trivial | Add comment |
| 43 | `highlight_theme` wrong on dark + `scrollbar_show` ignored | **High** | Low | Set from `is_dark` and `overlay_mode` |
| 44 | `ThemeConfig.highlight` not populated | **Medium** | Low | Set highlight in config |
| 45 | 19 of 30 icon mappings lack regression tests | **Medium** | Low | Data-driven mapping test |
| 46 | `StatusError`/`DialogError` -> same CircleX | **Negligible** | Trivial | Add comment |
| 47 | `mono_font_weight()` helper missing | **Low** | Trivial | Add helper (+ expand #13) |
| 48 | `DialogButtonOrder` not re-exported | **Low** | Trivial | Add re-export + helper |
| 49 | `from_system()` resolves both variants, wastes one | **Low** | Trivial | Document cost |
| 50 | `test_resolved()` tests wrong preset's colors | **High** | Trivial | Fix `into_variant(true)` |
| 51 | Switch ON uses `tc.primary` not `switch.checked_background` | **Medium** | N/A | gpui-component limitation; document |
| 52 | `active_color()` indistinguishable on near-black themes | **Medium** | Trivial | Add minimum lightness delta + clamp |
| 53 | `Theme.transparent` never explicitly set | **Negligible** | Trivial | Add comment |
| 54 | BMP encoder comment misleading about BGRA swizzle | **Negligible** | Trivial | Fix comment |
| 55 | Showcase `role_for_gpui_icon()` ambiguous reverse mapping | **Negligible** | Trivial | Add comment |
| 56 | Showcase redundant Material loads in `load_gpui_icons()` | **Low** | Low | Pre-load material cache |
| 57 | Showcase NaN propagation from NumberInput | **Low** | Trivial | Reject non-finite values |
| 58 | Dark theme 20% active darkening counterproductive | **Low** | Trivial | Document rationale |
| 59 | Showcase uses `resolve()` not `resolve_all()` (CC-10) | **Medium** | Trivial | Replace at showcase.rs:1636 |

---

## 51. Switch Checked State Ignores `switch.checked_background`

**File:** `colors.rs:285-286`

```rust
tc.switch = rgba_to_hsla(resolved.switch.unchecked_background);
tc.switch_thumb = rgba_to_hsla(resolved.switch.thumb_background);
```

gpui-component's switch rendering (switch.rs:97-98) uses:
- **Checked (ON):** `cx.theme().primary` as the track background
- **Unchecked (OFF):** `cx.theme().switch` as the track background

The connector correctly maps `tc.switch` from `resolved.switch.unchecked_background`.
But the checked/ON state uses `tc.primary`, which is mapped from
`resolved.button.primary_background` (colors.rs:89) -- NOT from
`resolved.switch.checked_background`.

`ResolvedSwitchTheme` (widgets/mod.rs:477-484) has an explicit
`checked_background: Rgba` field that theme authors set to control the
switch ON color. This field is resolved, validated, and present in every
preset's TOML -- but the connector never reads it.

**Impact:** Theme authors who set `switch.checked_background` to a color
different from `button.primary_background` (e.g., a green "active" color
vs a blue "primary" button color) see the wrong ON color. The switch always
uses the button primary color instead of its own checked background.

### Solutions

#### A. Set `tc.primary` from `switch.checked_background` when they differ? -- NOT viable

This would break all primary button colors just to fix the switch. The
underlying problem is gpui-component hardcodes `primary` as the checked
track color -- there is no separate `switch_checked` ThemeColor field.

#### B. Document the limitation (recommended for v0.5.4)

Add a comment in `assign_misc()` explaining that gpui-component's Switch
uses `tc.primary` for the checked state, so `switch.checked_background`
cannot be independently mapped. Theme authors should set
`button.primary_background` to their desired switch-ON color, or accept
that the switch ON color matches the primary button.

```rust
// gpui-component Switch uses tc.primary for the checked (ON) track,
// so resolved.switch.checked_background cannot be mapped independently.
// The ON color is controlled by button.primary_background / d.accent.
tc.switch = rgba_to_hsla(resolved.switch.unchecked_background);
tc.switch_thumb = rgba_to_hsla(resolved.switch.thumb_background);
```

| Pros | Cons |
|------|------|
| Honest documentation | No functional change |
| No risk of breaking primary | Theme authors lose independent switch color |

#### C. Request `switch_checked` field in gpui-component upstream

| Pros | Cons |
|------|------|
| Fixes the root cause | Upstream dependency, unknown timeline |

**Best solution: B now, C long-term.** Document the limitation so theme
authors understand the constraint, and file an upstream issue for a
dedicated `switch_checked` ThemeColor field.

---

## 52. `active_color()` Darkens Already-Black Colors to Black (No Visual Feedback)

**File:** `derive.rs:27-29`

```rust
pub fn active_color(base: Hsla, is_dark: bool) -> Hsla {
    let factor = if is_dark { 0.2 } else { 0.1 };
    base.darken(factor)
}
```

`Colorize::darken` (gpui-component color.rs:94-98) computes `l = self.l * (1.0 - factor)`.
When the base color's lightness is already very low (e.g., `l = 0.05` for a near-black
button on a dark theme), darkening by 20% yields `l = 0.04` -- visually
indistinguishable. The active state provides zero visual feedback.

Similarly, `hover_color(base, bg)` blends the base at 0.9 opacity onto bg.
On a dark theme where both base and bg have low lightness, the hover state
is also indistinguishable from the base.

This affects all 8 `_active` fields (primary, secondary, danger, success,
warning, info, link, accordion) and all corresponding hover fields when
the theme uses very dark base colors.

**Impact:** Dark themes with near-black button/accent colors (e.g., some
industrial/minimal themes) get no visual feedback on hover or click.

### Solutions

#### A. Lighten instead of darken when base is already dark (recommended)

```rust
pub fn active_color(base: Hsla, is_dark: bool) -> Hsla {
    if base.l < 0.15 {
        // Already very dark: lighten instead of darken for visible feedback
        Hsla { l: base.l + 0.08, ..base }
    } else {
        let factor = if is_dark { 0.2 } else { 0.1 };
        base.darken(factor)
    }
}
```

| Pros | Cons |
|------|------|
| Visible active state on all themes | Additional branch |
| Preserves normal behavior for typical colors | Threshold is somewhat arbitrary |
| Same pattern as "_light" fix (issue #3) | |

#### B. Use mix toward mid-gray instead of darken

```rust
base.mix(Hsla { h: base.h, s: base.s, l: 0.3, a: base.a }, 0.3)
```

| Pros | Cons |
|------|------|
| Always moves toward a visible tone | May look odd for very dark themes |

#### C. Keep current behavior

| Pros | Cons |
|------|------|
| No change | No visual feedback on near-black themes |
| Simple | |

**Best solution: A.** A lightness threshold with inverse direction ensures
visual feedback without changing behavior for the vast majority of themes
where base lightness is in the normal range.

---

## 53. `Theme.transparent` Field Never Set -- Gets `Hsla::default()` (Transparent Black)

**File:** `lib.rs:96-123`, gpui-component `theme/mod.rs:67`

gpui-component's `Theme` struct has a `pub transparent: Hsla` field (mod.rs:67).
The connector's `to_theme()` builds the theme via `Theme::from(&theme_color)` and
then explicitly sets `mode`, `font_family`, `font_size`, `mono_font_family`,
`mono_font_size`, `radius`, `radius_lg`, and `shadow` -- but never touches
`transparent`.

`Theme::from(&ThemeColor)` sets `transparent` from `ThemeColor::default()`, which
is `Hsla::default()` -- transparent black `(h:0, s:0, l:0, a:0)`. This is actually
correct for its purpose (a fully transparent color), but it's the only `Theme`-level
field that is never explicitly set by the connector. If a future gpui-component
version changes the default, the connector would silently pick up the wrong value.

**Severity:** Negligible -- the default is correct. But this is an inconsistency in
the "all Theme fields are set explicitly" claim in the doc comment at lib.rs:92-94.

### Solutions

#### A. Explicitly set `theme.transparent = gpui::transparent_black()` (recommended)

```rust
theme.transparent = gpui::transparent_black();
```

| Pros | Cons |
|------|------|
| Matches the doc comment's claim | One line |
| Immune to upstream default changes | |

#### B. Fix the doc comment to exclude `transparent`

| Pros | Cons |
|------|------|
| Accurate documentation | Leaves implicit dependency |

**Best solution: A.** One line, future-proof.

---

## 54. BMP Encoder Has Redundant BGRA Conversion Comment

**File:** `icons.rs:1081-1084`

```rust
// Channel masks (RGBA -> BGRA in BMP, but we use BI_BITFIELDS to specify layout)
buf.extend_from_slice(&0x00FF0000u32.to_le_bytes()); // red mask
buf.extend_from_slice(&0x0000FF00u32.to_le_bytes()); // green mask
buf.extend_from_slice(&0x000000FFu32.to_le_bytes()); // blue mask
buf.extend_from_slice(&0xFF000000u32.to_le_bytes()); // alpha mask
```

The comment says "RGBA -> BGRA in BMP, but we use BI_BITFIELDS" suggesting the
masks handle the swizzle. But the code ALSO does manual BGRA conversion in the
pixel loop at lines 1098-1104:

```rust
buf.push(pixel[2]); // B
buf.push(pixel[1]); // G
buf.push(pixel[0]); // R
buf.push(pixel[3]); // A
```

The channel masks say: red is at byte offset 2 (`0x00FF0000`), green at 1
(`0x0000FF00`), blue at 0 (`0x000000FF`). But the manual BGRA conversion
already writes blue to offset 0, green to 1, red to 2. So the masks describe
the already-swizzled data correctly.

However, if someone reads the comment "but we use BI_BITFIELDS to specify layout"
they might think the masks handle the swizzle and remove the manual conversion
in the pixel loop -- which would break the output. The comment is misleading.

### Solutions

#### A. Fix the comment to clarify both steps are needed (recommended)

```rust
// BI_BITFIELDS channel masks: describe the byte layout of BGRA pixel data.
// The pixel loop below manually swizzles RGBA to BGRA; these masks tell
// the BMP reader where each channel lives in the swizzled output.
```

| Pros | Cons |
|------|------|
| Prevents future maintainer from removing pixel swizzle | Trivial change |

**Best solution: A.**

---

## 55. `role_for_gpui_icon()` in Showcase Missing `StatusError` Reverse Mapping

**File:** `showcase.rs:323-356` (`role_for_gpui_icon()`)

The reverse-lookup table maps gpui-component icon names back to `IconRole`.
`"CircleX"` maps to `Some(IconRole::DialogError)` (line 328). But the
forward mapping in `icon_name()` (icons.rs:113) also maps
`IconRole::StatusError => IconName::CircleX`.

The reverse lookup is used to decide which icons get loaded from
native-theme. Since `CircleX` -> `DialogError` only, when the showcase
renders the `CircleX` row in the gpui icon gallery, it loads the
`DialogError` role icon. But the user might expect to see the
`StatusError` role's icon for that same `CircleX` slot.

This is arguably not a bug (existing issue #46 documents the
forward-mapping overlap), but the reverse lookup silently picks one role
over another without documentation. The showcase displays "DialogError"
as the role for `CircleX` even though `StatusError` would also be valid.

**Impact:** Showcase-only (no production impact). Minor UX confusion.

### Solutions

#### A. Add comment to `role_for_gpui_icon()` explaining the ambiguity (recommended)

```rust
// CircleX maps to both DialogError and StatusError in the forward mapping.
// We prefer DialogError as the reverse lookup result since it's the
// more common use case for the gpui-component CircleX icon.
"CircleX" => Some(IconRole::DialogError),
```

| Pros | Cons |
|------|------|
| Documents the choice | No functional change |

**Best solution: A.** Same treatment as issues #33 and #46.

---

## 56. Showcase `load_gpui_icons()` Calls `load_icon()` Redundantly for Material Fallback Detection

**File:** `showcase.rs:538-549`

```rust
Some(IconData::Svg(loaded)) => {
    let mat = load_icon(r, IconSet::Material);
    if let Some(IconData::Svg(mat_bytes)) = &mat {
        if loaded == mat_bytes {
            IconSource::Fallback
        } else {
            IconSource::System
        }
    } else {
        IconSource::System
    }
}
```

For EVERY icon in the system set, this calls `load_icon(r, IconSet::Material)` to
compare bytes and detect whether the result is a bundled fallback. This is 42
Material icon loads per call to `load_gpui_icons()`, happening every time the icon
set changes. Compare with `load_all_icons()` (showcase.rs:244-255) which pre-loads
all Material icons once in a batch.

**Impact:** Showcase-only performance. Each icon set switch triggers 42+86=128
unnecessary Material icon loads in `load_gpui_icons` (42 with roles + 86 total,
some overlap). On fast systems this is <100ms, but on slow I/O it can stutter.

### Solutions

#### A. Pre-load Material icons once and pass as parameter (recommended)

```rust
fn load_gpui_icons(
    icon_set: Option<IconSet>,
    default_theme: Option<&str>,
    cli_override: Option<&str>,
    material_cache: &[Option<IconData>],  // pre-loaded material icons
) -> Vec<IconEntry> {
```

| Pros | Cons |
|------|------|
| Eliminates 42 redundant loads per call | Requires passing pre-loaded data |
| Consistent with `load_all_icons` pattern | |

#### B. Keep current (acceptable for showcase)

| Pros | Cons |
|------|------|
| Showcase-only, not production | Stutters on slow I/O |

**Best solution: A for polish, B is acceptable.**

---

## 57. Showcase `number_input_state` Handler Uses `f64::parse` Without NaN/Inf Guard

**File:** `showcase.rs:1203-1204`

```rust
let value = input.value();
let num: f64 = value.parse().unwrap_or(0.0);
```

If a user types "NaN" or "Infinity" into the number input, `f64::parse`
succeeds (Rust's `f64::from_str` accepts these). Then:
```rust
let new_value = if *action == StepAction::Increment {
    num + 1.0  // NaN + 1.0 = NaN
} else {
    num - 1.0  // NaN - 1.0 = NaN
};
input.set_value(SharedString::from(new_value.to_string()), window, cx);
```

The input permanently displays "NaN" -- subsequent increment/decrement
operations are no-ops because NaN propagates. The only escape is to
manually clear and retype a number.

**Impact:** Showcase-only, but demonstrates poor NumberInput integration
to users copying the example pattern.

### Solutions

#### A. Guard against non-finite values (recommended)

```rust
let num: f64 = value.parse().unwrap_or(0.0);
let num = if num.is_finite() { num } else { 0.0 };
```

| Pros | Cons |
|------|------|
| NaN/Inf resets to 0 | Silently discards edge values |
| One-line fix | |

**Best solution: A.**

---

## 58. `Colorize::darken` Doc Mismatch -- Active Color Direction Inverted for Dark Themes

**File:** `derive.rs:22-24`

```rust
/// For light themes (is_dark=false): darkens by 10%.
/// For dark themes (is_dark=true): darkens by 20%.
```

The doc says dark themes darken MORE (20% vs 10%). But on dark themes,
darkening the already-dark active state makes it LESS visible (closer to
the dark background), not more. This is the opposite of the expected UX
where active states should be MORE distinct.

gpui-component's own `apply_config` uses the same darkening approach, so
the connector matches upstream. But the design choice itself is questionable:
on a dark theme with `primary.l = 0.4`, active becomes `0.4 * 0.8 = 0.32`,
moving it TOWARD the dark background (l~0.1) and reducing contrast. On a
light theme with the same primary, active becomes `0.4 * 0.9 = 0.36`,
also moving toward the background (l~0.95) -- but the light background is
so far away that contrast is still fine.

The asymmetry (20% dark vs 10% light) amplifies this problem on dark themes.

This compounds with issue #52 (already-near-black colors get invisible).

### Solutions

#### A. Document the upstream rationale (recommended for v0.5.4)

```rust
/// For dark themes (is_dark=true): darkens by 20%.
///
/// NOTE: This matches gpui-component's apply_config derivation.
/// On dark themes, 20% darkening reduces contrast with the dark background
/// more than 10% on light themes. For near-black base colors (l < 0.15),
/// consider issue #52's lightening approach.
```

| Pros | Cons |
|------|------|
| Honest documentation | No behavioral fix |
| Explains the upstream constraint | |

#### B. Invert direction on dark themes (lighten instead of darken)

| Pros | Cons |
|------|------|
| Better contrast on dark themes | Diverges from gpui-component behavior |
| More accessible | May look wrong to users expecting upstream behavior |

**Best solution: A for v0.5.4** (matches upstream). Consider B for v0.6.0
alongside a discussion with the gpui-component maintainers.

---

## New Findings: Second-Pass Deep Audit

The following issues are genuinely new -- not covered by existing issues 1--59.

### 60. `accordion_hover` Derived From Accent Instead of Accordion Base Color

**Category:** derive-bug
**Severity:** medium
**File(s):** `colors.rs:257`

**Problem:** `tc.accordion_hover` is derived as `hover_color(c.accent, c.bg)` -- a
hover variant of the ACCENT color blended onto background. But `tc.accordion` is set
to `c.bg` at line 256. The hover state of a bg-colored element should be derived from
bg, not from accent. This means hovering over an accordion header produces an
accent-tinted highlight (e.g. blue tint) rather than a subtle lightening/darkening of
the background. Compare with `tc.list_hover = hover_color(c.secondary, c.bg)` which
correctly derives hover from the list's own base color (secondary).

gpui-component's own `apply_config` derives `accordion_hover` as
`accent.opacity(0.08)` blended on bg -- accent-tinted but very subtle. The connector's
version uses `accent.opacity(0.9)` (via `hover_color`), which is far more opaque and
produces a much stronger accent tint than upstream intends. The 0.9 opacity was designed
for hover on buttons (where the base IS the accent); applied to accordion headers
(where the base is bg), it creates an unexpectedly vivid hover effect.

**Solution Options:**

A. Match gpui-component's own accordion_hover derivation:
```rust
tc.accordion_hover = c.bg.blend(c.accent.opacity(0.08));
```

B. Derive from the accordion base color:
```rust
tc.accordion_hover = hover_color(c.bg, c.bg); // subtle bg shift
```

C. Keep current (accent-tinted hover).

**Best Solution:** A. Matches upstream intent: subtle accent tint on bg, not a
near-opaque accent blend.

---

### 61. `description_list_label_foreground` Reads From `tc.muted_foreground` After Assignment

**Category:** mapping-bug
**Severity:** low
**File(s):** `colors.rs:264`

**Problem:**
```rust
tc.description_list_label_foreground = tc.muted_foreground;
```

This reads from `tc.muted_foreground`, which was set by `assign_core` (called before
`assign_misc`). This works correctly TODAY because `assign_core` runs first in
`to_theme_color()` (line 114). But it creates an implicit ordering dependency between
`assign_core` and `assign_misc` -- if someone reorders the assign calls (e.g. for
readability), the description list label foreground silently picks up the wrong value
(ThemeColor::default muted_foreground).

All other assign functions read from `&ResolvedColors` which is computed once upfront.
This is the only place in the codebase where one assign function reads a ThemeColor
field set by another assign function.

**Solution Options:**

A. Read from `c.muted_fg` instead (recommended):
```rust
tc.description_list_label_foreground = c.muted_fg;
```

B. Add comment documenting the ordering dependency.

**Best Solution:** A. Uses the same source, removes the fragile cross-assignment read.

---

### 62. `sidebar_border` Uses Global `c.border` Instead of `resolved.window.border`

**Category:** mapping-bug
**Severity:** low
**File(s):** `colors.rs:215`

**Problem:**
```rust
tc.sidebar_border = c.border;
```

The `sidebar_border` field uses the global `defaults.border` color. But two lines
below, `tc.title_bar_border` and `tc.window_border` both use the per-widget
`resolved.window.border` (lines 221-222). On themes where `window.border` differs
from `defaults.border` (e.g., window has a darker frame border while the global
border is a lighter separator), the sidebar border will not match the window border,
creating a visible discontinuity where the sidebar meets the title bar.

platform-facts.md section 2.2 shows `window.border` inherits from `defaults.border`
on most platforms (`<- defaults.border`), so in practice these values are usually
equal. However, theme authors can override `window.border` independently (e.g., KDE's
`[WM]` decoration colors can differ from `[Colors:Window]`), and the connector should
respect that override for sidebar border too.

**Solution Options:**

A. Use `resolved.window.border` for sidebar border:
```rust
tc.sidebar_border = rgba_to_hsla(resolved.window.border);
```

B. Keep using global border (documented as intentional).

**Best Solution:** A. The sidebar is part of the window chrome, so its border should
match the window border, not the global separator.

---

### 63. No Test Verifies `assign_tab_sidebar` Maps All 12 Fields

**Category:** test-gap
**Severity:** low
**File(s):** `colors.rs` tests (absent)

**Problem:** `assign_tab_sidebar` sets 12 ThemeColor fields: `tab`, `tab_active`,
`tab_active_foreground`, `tab_bar`, `tab_bar_segmented`, `tab_foreground`, `sidebar`,
`sidebar_foreground`, `sidebar_accent`, `sidebar_accent_foreground`, `sidebar_border`,
`sidebar_primary`, `sidebar_primary_foreground`, `title_bar`, `title_bar_border`,
`window_border`. The only sidebar/tab test is the `per_widget_fields_used` test
(colors.rs:437) which checks `scrollbar_thumb`, `slider_bar`, `progress_bar`,
`title_bar`, `switch`, and `caret` -- but NOT `tab`, `tab_active`, `sidebar`, or
`sidebar_foreground`. Similarly, the `accent_foreground_uses_theme_value` test
(colors.rs:485) checks `sidebar_accent_foreground` but none of the other 11 fields.

This means 10 of the 15 fields set by `assign_tab_sidebar` have no regression test
verifying they receive the correct source value. A mapping regression (e.g.,
accidentally swapping `tab_active` and `tab_foreground`) would be invisible to the
test suite.

Note: Issue #35 covers the doc-comment coverage TABLE becoming stale, and issue #45
covers icon MAPPING regression tests. This issue is specifically about the
tab/sidebar/window COLOR mapping lacking individual field-level assertions.

**Solution Options:**

A. Add test that verifies all 15 fields against their expected sources:
```rust
#[test]
fn tab_sidebar_window_fields_match_resolved() {
    let resolved = test_resolved();
    let tc = to_theme_color(&resolved, false);
    assert_eq!(tc.tab, rgba_to_hsla(resolved.tab.background));
    assert_eq!(tc.tab_active, rgba_to_hsla(resolved.tab.active_background));
    assert_eq!(tc.sidebar, rgba_to_hsla(resolved.sidebar.background));
    // ... all 15 fields
}
```

B. Keep relying on the "produces nondefault" test.

**Best Solution:** A. Direct source-to-field assertions catch mapping regressions.

---

### 64. `encode_rgba_as_bmp` Height Cast Can Overflow for Heights > i32::MAX

**Category:** api-misuse
**Severity:** low
**File(s):** `icons.rs:1071`

**Problem:**
```rust
buf.extend_from_slice(&(-(height as i32)).to_le_bytes()); // height (top-down)
```

If `height` exceeds `i32::MAX` (2,147,483,647), the `as i32` cast wraps, and
negation can overflow. For example, `height = u32::MAX` casts to `-1i32`, then
negation yields `1` -- a valid but incorrect height. The function validates that
`width * height * 4` fits in `usize` (line 1046-1047), but on 64-bit systems
this allows values well above `i32::MAX`.

The existing validation at line 1053 checks `u32::try_from(file_size).ok()?` which
limits total file size to 4 GB (about 32k x 32k at 4 bytes/pixel), so in practice
height cannot exceed ~32768 before the file-size check fails. However, for
non-square images (e.g., width=1, height=3_000_000_000), the pixel data is only 12 GB
which overflows usize on 32-bit but could pass on 64-bit -- though the u32 file_size
check at line 1053 would catch it.

While the existing checks make exploitation difficult, the `as i32` cast is
technically unsound. Issue #28 covers the SIZE parameter validation (clamping to
1..512), which would also prevent this. But `encode_rgba_as_bmp` is a separate
internal function that could be called with arbitrary dimensions.

**Solution Options:**

A. Add explicit i32 range check (recommended):
```rust
if width == 0 || height == 0 || height > i32::MAX as u32 {
    return None;
}
```

B. Rely on the u32 file-size overflow check (current behavior).

**Best Solution:** A. Trivial guard, prevents the silent i32 wrap for defense in depth.

---

### 65. `colorize_svg` Short-Circuits on `currentColor`, Misses Co-occurring Explicit Black Fills

**Category:** mapping-bug
**Severity:** low
**File(s):** `icons.rs:992-995`

**Problem:**
```rust
if svg_str.contains("currentColor") {
    return svg_str.replace("currentColor", &hex).into_bytes();
}
```

If an SVG contains BOTH `currentColor` AND `fill="black"` on different elements
(e.g., `<path stroke="currentColor"/><rect fill="black"/>`), the function replaces
only `currentColor` and returns immediately -- the `fill="black"` on the rect is left
unchanged, rendering that element in black regardless of the requested color.

This is unlikely with bundled Lucide/Material icons (which use one convention
consistently), but third-party SVGs that mix conventions would be partially colorized.
Issue #10 covers missing `stroke="black"` patterns, and issue #34 covers quoted `>`
fragility -- but neither covers the early-return short-circuit that skips subsequent
replacement phases.

**Solution Options:**

A. Remove early return; apply all three phases sequentially:
```rust
let mut result = svg_str.to_string();
// Phase 1: currentColor
result = result.replace("currentColor", &hex);
// Phase 2: explicit black fills
let fill_hex = format!("fill=\"{}\"", hex);
result = result.replace("fill=\"black\"", &fill_hex)
    .replace("fill=\"#000000\"", &fill_hex)
    .replace("fill=\"#000\"", &fill_hex);
// Phase 3: implicit black -- skip if fill was already replaced/injected
```

B. Keep early return with documentation (recommended for v0.5.4):
The function doc at line 967-971 says "Handles three SVG color patterns (in order)"
implying they are alternatives. Add explicit note that `currentColor` SVGs with
co-occurring explicit black fills are only partially colorized.

**Best Solution:** B for v0.5.4, A for v0.6.0. The early return is a deliberate
design choice (currentColor SVGs from Lucide should not have explicit black fills
rewritten), and removing it risks double-colorizing elements. But documenting the
limitation prevents user confusion.

---

### 66. `colorize_svg` Phase 3 Injection Skips SVGs Where Root Has Non-Black `fill`

**Category:** test-gap
**Severity:** low
**File(s):** `icons.rs:1014`

**Problem:**
```rust
if !tag.contains("fill=") {
    // inject fill
}
```

This check prevents injection when the root `<svg>` tag has ANY `fill=` attribute --
including `fill="none"` or `fill="white"`. If a third-party SVG has
`<svg fill="none">` with children that rely on inherited fill, the function correctly
skips injection. But if the SVG has `<svg fill="white">` and the user wants to
colorize it, the function returns the original unchanged because the root already has
a fill.

The existing test suite has no test for this case. There is `colorize_svg_implicit_
black_still_works` (no fill at all) and `colorize_svg_replaces_fill_black` (explicit
black), but no test for `fill="white"` or `fill="none"` on the root tag.

**Solution Options:**

A. Add tests for non-black root fills (recommended):
```rust
#[test]
fn colorize_svg_root_fill_white_unchanged() {
    let svg = b"<svg fill=\"white\"><path d=\"M0 0\"/></svg>";
    let color = gpui::hsla(0.0, 1.0, 0.5, 1.0);
    let result = colorize_svg(svg, color);
    assert_eq!(result, svg.to_vec(), "non-black root fill should be unchanged");
}

#[test]
fn colorize_svg_root_fill_none_unchanged() {
    let svg = b"<svg fill=\"none\"><path d=\"M0 0\"/></svg>";
    let color = gpui::hsla(0.0, 1.0, 0.5, 1.0);
    let result = colorize_svg(svg, color);
    assert_eq!(result, svg.to_vec(), "fill=none root should be unchanged");
}
```

B. Keep untested.

**Best Solution:** A. Tests document the intended behavior for these edge cases.

---

### 67. `hover_color` and `active_color` Not Tested With Fully Transparent or Zero-Alpha Colors

**Category:** test-gap
**Severity:** low
**File(s):** `derive.rs` tests (lines 39-70)

**Problem:** The three derive.rs tests use a single base color `hsla(0.6, 0.7, 0.5, 1.0)` -- fully opaque, mid-lightness. No test exercises edge cases:
- `alpha = 0.0` (fully transparent base) -- `hover_color` blends at 0.9 opacity, but
  the base itself has 0 alpha. The blend math `bg.blend(base.opacity(0.9))` where
  `base.a = 0.0` means `base.opacity(0.9)` produces `a = 0.0 * 0.9 = 0.0` -- the
  hover color equals `bg`, making hover indistinguishable from the background.
- `saturation = 0.0` (achromatic) -- hue is meaningless, verify no NaN.
- `lightness = 0.0` (pure black) -- `active_color` darkens by 10-20%, which is a
  no-op on black. This overlaps with issue #52 but that issue has no TEST for it.
- `lightness = 1.0` (pure white) -- `active_color` darkens to 0.8/0.9, which works
  but `hover_color` blend on a white bg produces white -- indistinguishable.

Issue #52 describes the near-black active problem but proposes a CODE fix, not a TEST.
The test gap remains: the derive module has zero edge-case tests.

**Solution Options:**

A. Add edge-case tests (recommended):
```rust
#[test]
fn hover_color_transparent_base_equals_bg() {
    let base = hsla(0.0, 0.0, 0.5, 0.0);
    let bg = hsla(0.0, 0.0, 1.0, 1.0);
    let result = hover_color(base, bg);
    // With base alpha=0, result should essentially be bg
    assert!((result.l - bg.l).abs() < 0.01);
}

#[test]
fn active_color_black_stays_black() {
    let base = hsla(0.0, 0.0, 0.0, 1.0);
    let result = active_color(base, true);
    assert!(result.l < 0.01, "darkening black should stay at ~0");
}
```

B. Keep testing only the happy path.

**Best Solution:** A. Edge-case tests catch regressions if the derivation logic changes
and document the known limitations for future maintainers.

---

## New Findings: Priority Summary

| # | Issue | Severity | Effort | Best Fix |
|---|-------|----------|--------|----------|
| 60 | `accordion_hover` derived from accent at 0.9 opacity, not bg | **Medium** | Trivial | Match upstream `accent.opacity(0.08)` blend |
| 61 | `description_list_label_foreground` reads tc field, not ResolvedColors | **Low** | Trivial | Read from `c.muted_fg` |
| 62 | `sidebar_border` uses global border, not window border | **Low** | Trivial | Use `resolved.window.border` |
| 63 | No test for 10 of 15 tab/sidebar/window field mappings | **Low** | Low | Add per-field assertions |
| 64 | `encode_rgba_as_bmp` height i32 cast can wrap | **Low** | Trivial | Add `height > i32::MAX` guard |
| 65 | `colorize_svg` early-returns on currentColor, skips explicit black | **Low** | Trivial | Document limitation |
| 66 | `colorize_svg` non-black root fill untested | **Low** | Trivial | Add edge-case tests |
| 67 | `hover_color`/`active_color` zero-alpha/black edge cases untested | **Low** | Trivial | Add edge-case tests |

---

## New Findings: Third-Pass Deep Audit

The following issues are genuinely new -- not covered by existing issues 1--67.

### 68. `tc.scrollbar` Uses `c.bg` Instead of `resolved.scrollbar.track`

**Category:** mapping-bug
**Severity:** medium
**File(s):** `colors.rs:276`

**Problem:**
```rust
tc.scrollbar = c.bg;
```

The `tc.scrollbar` field is documented in gpui-component as "Scrollbar background color"
(theme_color.rs:107). The connector maps it to `c.bg` (`defaults.background`). However,
`ResolvedScrollbarTheme` has an explicit `track: Rgba` field (widgets/mod.rs:266) that
represents the scrollbar track/gutter color.

platform-facts.md section 2.8 shows:
- macOS: track is **transparent** (overlay mode)
- Windows: track is **transparent**
- KDE: track is `defaults.background`
- GNOME: track is from Adwaita CSS scrollbar styling

On macOS and Windows, `resolved.scrollbar.track` will be a transparent or near-transparent
color. By using `c.bg` (which is always opaque), the connector produces an opaque scrollbar
gutter on platforms where native scrollbars have transparent backgrounds. This is visually
wrong for overlay-style scrollbars -- the gutter should be transparent so content shows
through behind it.

On KDE, `track` inherits from `defaults.background`, so the current mapping accidentally
produces the correct result.

Two lines below, the connector correctly uses `resolved.scrollbar.thumb` and
`resolved.scrollbar.thumb_hover` for the thumb colors -- making the track the only
scrollbar field that bypasses the resolved per-widget value.

**Solution Options:**

A. Use `resolved.scrollbar.track`:
```rust
tc.scrollbar = rgba_to_hsla(resolved.scrollbar.track);
```

B. Keep `c.bg` (correct for KDE, wrong for macOS/Windows).

**Best Solution:** A. Consistent with all other per-widget scrollbar field mappings in the
same function and respects the platform-specific track color.

---

### 69. `to_theme_config` Does Not Set `is_default` Field

**Category:** config-gap
**Severity:** low
**File(s):** `config.rs:23-38`

**Problem:** `ThemeConfig` has an `is_default: bool` field (gpui-component schema.rs:32).
The connector leaves it at `false` via `..ThemeConfig::default()`. gpui-component uses
`is_default` to decide whether to treat a theme as the built-in default when
`Theme::default()` is invoked.

For native-theme-derived configs, `is_default = false` is arguably correct -- these are
NOT gpui-component's built-in defaults. However, the field is never mentioned in
documentation, and the connector's `to_theme_config` has no comment acknowledging it.
If a downstream consumer checks `is_default` to decide whether to allow theme customization,
native-theme themes would be incorrectly treated as customized even when they are the
application's primary theme.

**Solution Options:**

A. Add a comment documenting the intentional `false`:
```rust
// is_default: false -- native-theme configs are not gpui-component built-in defaults.
// Consumers who want to mark the system theme as "default" should set this on the
// returned ThemeConfig.
```

B. Keep as-is.

**Best Solution:** A. Documentation for future maintainers.

---

### 70. `config.rs` Tests Assert Tautology: `radius` Compared to Same Expression

**Category:** test-weakness
**Severity:** low
**File(s):** `config.rs:81-88`

**Problem:**
```rust
assert_eq!(
    config.radius,
    Some(resolved.defaults.radius.round() as usize)
);
assert_eq!(
    config.radius_lg,
    Some(resolved.defaults.radius_lg.round() as usize)
);
```

These assertions compare the output of `to_theme_config` against the SAME expression used
in the production code (config.rs:33-34). If the production code has a bug (e.g., rounding
wrong, wrong source field), the test replicates the same bug in its expectation. The test
would pass with `radius: Some(d.radius_lg.round() as usize)` (swapped fields) because
catppuccin-mocha happens to have radius == radius_lg (both 8.0 in the light fallback).

This is a special case of the broader issue #1 (single-preset testing), but specifically
for config.rs: the test asserts structural identity ("the code does what the code says")
rather than semantic correctness ("the theme config has a radius of 8 for catppuccin-mocha
light").

**Solution Options:**

A. Assert against a known concrete value from the preset:
```rust
// catppuccin-mocha light (actually catppuccin-latte) has radius 8.0
assert_eq!(config.radius, Some(8));
```

B. Use a preset where radius != radius_lg to catch field swaps.

**Best Solution:** A combined with B. A hardcoded known-good value catches regressions the
tautological assertion misses, and a preset with differing radii catches field swaps.

---

### 71. Material Icon Mapping: `SortAscending` and `ArrowUp` Both Map to `arrow_upward`

**Category:** icon-collision
**Severity:** low
**File(s):** `icons.rs:301,236`

**Problem:**
```rust
IconName::ArrowUp => "arrow_upward",       // line 236
IconName::SortAscending => "arrow_upward", // line 301
```

Two distinct gpui-component `IconName` variants map to the same Material icon name
`arrow_upward`. Similarly:
```rust
IconName::ArrowDown => "arrow_downward",     // line 232
IconName::SortDescending => "arrow_downward", // line 302
```

And:
```rust
IconName::Dash => "remove",   // line 255
IconName::Minus => "remove",  // line 283
```

And:
```rust
IconName::Redo => "redo",     // line 294
IconName::Redo2 => "redo",    // line 295
IconName::Undo => "undo",     // line 310
IconName::Undo2 => "undo",    // line 311
```

And:
```rust
IconName::Folder => "folder",        // line 263
IconName::FolderClosed => "folder",  // line 264
```

And:
```rust
IconName::PanelRight => "right_panel_close",       // line 291
IconName::PanelRightClose => "right_panel_close",  // line 292
```

This is 7 collision pairs in the Material mapping alone. The Lucide mapping has fewer because
Lucide has more specific icons. The issues are:
1. `SortAscending` should ideally use a sort-specific icon (`sort` in Material) rather than a
   directional arrow.
2. `PanelRight` and `PanelRightClose` are semantically different (panel visible vs closing the
   panel) but use the same Material icon.

Issues #33 and #46 cover the forward `IconRole -> IconName` collisions (2 roles mapping to the
same IconName). This issue covers a different problem: **different IconNames producing the same
Material icon file**, which means the Material icon set provides no visual distinction for
these pairs.

**Solution Options:**

A. Document the collisions in `material_name_for_gpui_icon`:
```rust
// NOTE: SortAscending and ArrowUp both map to "arrow_upward" because Material
// has no dedicated sort-direction icon distinct from a directional arrow.
```

B. Use `sort` for `SortAscending` / `SortDescending` if Material has it.

**Best Solution:** A for v0.5.4, investigate B for v0.6.0. Material Symbols does have `sort`
and `sort_by_alpha` icons, but they may not be bundled. Documenting the known collisions
prevents confusion when the same icon appears in multiple gallery rows.

---

### 72. `list_active` Result Has `a=0.2` Making the Selection Nearly Invisible

**Category:** mapping-bug
**Severity:** medium
**File(s):** `colors.rs:187`

**Problem:** Issue #7 identifies the double-opacity pattern but does not fully analyze the
resulting alpha value. Let me trace the exact computation:

```rust
tc.list_active = c.bg.blend(c.primary.opacity(0.1)).alpha(0.2);
```

Step by step:
1. `c.primary.opacity(0.1)` -- produces an Hsla with `a=0.1`
2. `c.bg.blend(...)` -- blends primary at 10% onto bg. Result has `a = bg.a` (bg alpha,
   typically 1.0). The blend itself is barely visible (only 10% primary mixed in).
3. `.alpha(0.2)` -- SETS the result's alpha to 0.2, regardless of what came before.

The final `list_active` color has `a=0.2`. When gpui renders a `list_active` background,
the background is 80% transparent. On a dark theme where `bg` is near-black and `primary`
is a saturated color, the 10%-blended-then-80%-transparent result is nearly invisible.

Issue #7 proposes removing the double-opacity but does not note that the `.alpha(0.2)` call
produces a SEMI-TRANSPARENT fill that is also used for:
- `tc.table_active = tc.list_active` (line 194)

Both the table and list active selection states are 80% transparent. This means the
selection highlight bleeds through to whatever is beneath it, which may be acceptable for
overlaid scroll containers but is incorrect for opaque list backgrounds.

By contrast, `tc.list_active_border` at line 188 uses `.opacity(0.6)` for blending but does
NOT call `.alpha()`, so its result is fully opaque (`a = bg.a = 1.0`). The selection has a
visible border but a nearly invisible fill.

**NOTE:** This deepens issue #7's analysis rather than replacing it. Issue #7's recommended
fix (mode-aware single-stage opacity) would also fix this. But the key new observation is
that the final alpha value is 0.2, making the fill semi-transparent -- which is a distinct
problem from the double-opacity confusion issue #7 describes.

**Solution Options:**

Same as issue #7's solution A, with the additional note that the resulting color should
have `a=1.0` (fully opaque) for proper list selection rendering:
```rust
tc.list_active = c.bg.blend(c.primary.opacity(selection_opacity));
// Result has a=1.0 from bg, no .alpha() override
```

**Best Solution:** Fix alongside issue #7. The `.alpha(0.2)` call should be removed entirely.

---

### 73. `to_theme()` Never Sets `theme.colors` Field -- Relies on `Theme::from` Deref

**Category:** correctness-risk
**Severity:** low
**File(s):** `lib.rs:97-122`

**Problem:**
```rust
let theme_color = colors::to_theme_color(resolved, is_dark);
let mut theme = Theme::from(&theme_color);
theme.mode = mode;
theme.font_family = ...;
// Never: theme.colors = theme_color;
```

`Theme::from(&ThemeColor)` at gpui-component mod.rs:183-210 sets `colors: *colors`
(copies the ThemeColor). Then the connector sets individual fields on `theme.*` which,
via `DerefMut` to `ThemeColor`, write directly into `theme.colors.*`.

Wait -- looking more carefully at the code, the connector does NOT write to any ThemeColor
fields after `Theme::from()`. It only writes to Theme-level fields (`mode`, `font_family`,
etc.). The colors are all set correctly via `Theme::from(&theme_color)`.

This is NOT an issue. Removing from findings.

---

### 73. `assign_list_table` Does Not Receive `is_dark` But Contains Mode-Dependent Logic

**Category:** design-inconsistency
**Severity:** low
**File(s):** `colors.rs:184,118`

**Problem:** `assign_list_table` is the only assign function (other than `assign_charts` and
`assign_base_colors`) that does NOT receive `is_dark` as a parameter:

```rust
assign_core(&mut tc, &c, is_dark);        // receives is_dark
assign_primary(&mut tc, &c, is_dark);     // receives is_dark
assign_secondary(&mut tc, &c, is_dark);   // receives is_dark
assign_status(&mut tc, &c, is_dark);      // receives is_dark
assign_list_table(&mut tc, &c);           // NO is_dark
assign_tab_sidebar(&mut tc, &c, resolved); // no is_dark but has resolved
assign_charts(&mut tc, &c);              // no is_dark (stateless hue rotation)
assign_misc(&mut tc, &c, resolved, is_dark); // receives is_dark
assign_base_colors(&mut tc, &c);          // NO is_dark (issue #3 already covers this)
```

`assign_list_table` derives `list_active` and `list_hover` via `hover_color` and opacity
blending that would benefit from mode-awareness (as issue #7 proposes). The function
CANNOT currently implement issue #7's fix because it does not have access to `is_dark`.

Issue #3 covers `assign_base_colors` missing `is_dark`. Issue #7 covers `list_active`'s
double-opacity. But neither notes that `assign_list_table`'s signature actively prevents
mode-aware fixes -- it is a prerequisite blocker for issue #7's recommended solution.

**Solution Options:**

A. Add `is_dark` parameter when implementing issue #7's fix:
```rust
fn assign_list_table(tc: &mut ThemeColor, c: &ResolvedColors, is_dark: bool) {
```

B. Leave as-is until issue #7 is addressed.

**Best Solution:** A. This is the enabling change for issue #7 and should be done at the
same time.

---

### 74. `from_system()` Uses `sys.active()` Which Borrows, Then `sys.dark`/`sys.light` Moves

**Category:** already-covered (withdraw)

This is existing issue #19. Skipping.

---

### 74. `ThemeConfigColors` Has `group_box_title_foreground` With No `ThemeColor` Counterpart

**Category:** upstream-mismatch
**Severity:** negligible
**File(s):** gpui-component `schema.rs:98-100`

**Problem:** `ThemeConfigColors` (schema.rs:98-100) has a field:
```rust
#[serde(rename = "group_box.title.foreground")]
pub group_box_title_foreground: Option<SharedString>,
```

But `ThemeColor` (theme_color.rs) has only `group_box` and `group_box_foreground` -- no
`group_box_title_foreground`. This means `ThemeConfigColors` has a color field that has no
destination in `ThemeColor` when `apply_config` runs. If issue #5's fix populates
`ThemeConfigColors` from the computed `ThemeColor`, this field should be left as `None`
(it has no source in `ThemeColor` to read from).

This is a gpui-component inconsistency, not a connector bug. But it affects the
implementation of issue #5's recommended fix.

**Solution Options:**

A. When implementing issue #5, skip `group_box_title_foreground` (it has no `ThemeColor`
source) and add a comment:
```rust
// group_box_title_foreground: None -- ThemeConfigColors has this field but
// ThemeColor does not, so there is no source value to populate it from.
```

B. File upstream issue to either add `group_box_title_foreground` to `ThemeColor` or
remove it from `ThemeConfigColors`.

**Best Solution:** A now, B if gpui-component has an issue tracker.

---

### 75. No Test Verifies `list_active_border` / `table_active_border` Opacity Values

**Category:** test-gap
**Severity:** low
**File(s):** `colors.rs` tests (absent)

**Problem:** `tc.list_active_border` is set at colors.rs:188:
```rust
tc.list_active_border = c.bg.blend(c.primary.opacity(0.6));
```

And `tc.table_active_border = tc.list_active_border` at line 195.

No test verifies this value. The `to_theme_color_produces_nondefault` test (line 386)
checks `background`, `foreground`, `primary`, `danger`, `border` -- but not
`list_active_border`. The `per_widget_fields_used` test (line 437) checks `scrollbar_thumb`,
`slider_bar`, `progress_bar`, `title_bar`, `switch`, `caret` -- but not list/table borders.

If the `0.6` opacity was changed to `0.06` (typo), or if `c.primary` was accidentally
replaced with `c.fg`, no test would catch it.

**Solution Options:**

A. Add test verifying list_active_border is a blend of primary onto bg:
```rust
#[test]
fn list_active_border_blends_primary_on_bg() {
    let resolved = test_resolved();
    let tc = to_theme_color(&resolved, false);
    let c_bg = rgba_to_hsla(resolved.defaults.background);
    let c_primary = rgba_to_hsla(resolved.button.primary_background);
    let expected = c_bg.blend(c_primary.opacity(0.6));
    assert_eq!(tc.list_active_border, expected);
}
```

B. Keep untested.

**Best Solution:** A. This is one of the most complex derivations and deserves a direct test.

---

### 76. `table_head_foreground` Uses `c.muted_fg` But Has Same Semantic Issue as `tc.muted`

**Category:** mapping-concern
**Severity:** low
**File(s):** `colors.rs:198`

**Problem:**
```rust
tc.table_head_foreground = c.muted_fg;
```

Issue #2 identifies that `c.muted_fg` is derived as `rgba_to_hsla(d.muted).blend(fg.opacity(0.7))`
(colors.rs:88), which on dark themes pushes the result toward white, making it
indistinguishable from regular foreground.

If issue #2 is fixed (using `d.muted` directly as `muted_fg`), `table_head_foreground`
would also be fixed because it reads from `c.muted_fg`. However, the semantic question
is: should table column headers use the muted foreground at all?

gpui-component's `apply_config` derivation for `table_head_foreground` uses
`muted_foreground` (the same field). So the mapping choice is correct -- the issue is
upstream in the derivation of `muted_fg` itself (issue #2).

This is NOT a new issue -- it's a downstream consequence of issue #2. Documented here
for completeness during the color mapping audit to confirm that no independent mapping
error exists for this field.

---

### 77. `drag_border` and `drop_target` Use Hardcoded Opacity Values

**Category:** hardcoded-value
**Severity:** negligible
**File(s):** `colors.rs:298-299`

**Problem:**
```rust
tc.drag_border = c.primary.opacity(0.65);
tc.drop_target = c.primary.opacity(0.2);
```

These use hardcoded opacity values (0.65 and 0.2) that do not adapt to theme mode or
primary color lightness. On a dark theme with a high-lightness primary (e.g., l=0.9),
`drag_border` at 65% opacity is very prominent; on a dark primary (l=0.2), it nearly
vanishes.

gpui-component's own `apply_config` uses the same fixed opacities (0.65 and 0.2), so
the connector matches upstream behavior. The values are not derived from any resolved
property.

**Solution Options:**

A. Keep current -- matches upstream, and drag/drop styling is rarely visible.

**Best Solution:** A. The values match gpui-component exactly. This is a gpui-component
design choice, not a connector mapping error. Logged for audit completeness only.

---

### 78. `config.rs` Test `to_theme_config_from_resolved` Does Not Verify `mono_font_size`

**Category:** test-weakness
**Severity:** low
**File(s):** `config.rs:74-77`

**Problem:** The test verifies `font_size` matches resolved:
```rust
assert_eq!(config.font_size, Some(resolved.defaults.font.size));
```

And verifies `mono_font_family` is `Some`:
```rust
assert!(config.mono_font_family.is_some(), "mono_font_family should be set");
```

But it never checks that `mono_font_size` has the correct VALUE. The assertion at line
74 checks `font_size` against the expected resolved value, but the corresponding check
for `mono_font_size` is missing entirely. Line 76-77 only checks `mono_font_size`
indirectly via the `font_size_is_not_converted_from_points` test, which also only checks
the UI font, not the mono font.

If `mono_font_size` were accidentally set to `Some(d.font.size)` instead of
`Some(d.mono_font.size)`, no test would catch it (both would be present, both non-None).

**Solution Options:**

A. Add explicit value check:
```rust
assert_eq!(config.mono_font_size, Some(resolved.defaults.mono_font.size),
    "mono_font_size should match resolved mono font");
```

B. Keep relying on the indirect `is_some()` check.

**Best Solution:** A. Trivial addition that catches a likely copy-paste error class.

---

### 79. `with_spin_animation` Duration Cast: `u32` to `u64` Is Always Safe But Undocumented

**Category:** documentation-gap
**Severity:** negligible
**File(s):** `icons.rs:940`

**Problem:**
```rust
Animation::new(Duration::from_millis(duration_ms as u64)).repeat()
```

The `as u64` cast from `u32` is always widening and lossless. However, a `duration_ms`
of 0 produces a zero-duration animation that repeats infinitely -- effectively a no-op
that might spin-lock the animation system on some gpui backends (though gpui likely
guards against this internally).

`TransformAnimation::Spin { duration_ms }` in native-theme has no documented minimum.
A theme author could set `duration_ms: 0` in TOML, which would propagate here.

**Solution Options:**

A. Guard zero duration:
```rust
let ms = (duration_ms as u64).max(1);
Animation::new(Duration::from_millis(ms)).repeat()
```

B. Keep as-is (gpui likely handles zero duration gracefully).

**Best Solution:** B for v0.5.4 -- this is defensive paranoia. If gpui does not handle
zero-duration animations, the fix is one line.

---

## New Findings: Third-Pass Priority Summary

| # | Issue | Severity | Effort | Best Fix |
|---|-------|----------|--------|----------|
| 68 | `tc.scrollbar` uses `c.bg` instead of `resolved.scrollbar.track` | **Medium** | Trivial | Use `resolved.scrollbar.track` |
| 69 | `to_theme_config` does not document `is_default = false` | **Low** | Trivial | Add comment |
| 70 | `config.rs` test asserts tautology for radius values | **Low** | Trivial | Assert against known concrete value |
| 71 | 7 Material icon collision pairs undocumented | **Low** | Trivial | Add comments to colliding pairs |
| 72 | `list_active` has `a=0.2` making selection 80% transparent (deepens #7) | **Medium** | Trivial | Remove `.alpha(0.2)` alongside #7 fix |
| 73 | `assign_list_table` missing `is_dark` blocks issue #7 fix | **Low** | Trivial | Add `is_dark` param when fixing #7 |
| 74 | `ThemeConfigColors.group_box_title_foreground` has no ThemeColor source | **Negligible** | Trivial | Document in #5 implementation |
| 75 | No test for `list_active_border` / `table_active_border` | **Low** | Trivial | Add blend-verification test |
| 78 | `config.rs` test missing `mono_font_size` value assertion | **Low** | Trivial | Add `assert_eq!` for mono_font_size |
| 79 | `with_spin_animation` zero-duration undocumented | **Negligible** | Trivial | Keep as-is or guard with `.max(1)` |
