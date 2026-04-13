//! Icon conversion helpers for iced.
//!
//! Converts [`native_theme::IconData`] variants into iced-compatible handles.
//! Since iced separates raster images (`iced::widget::Image`) from SVG
//! images (`iced::widget::Svg`), this module provides separate conversion
//! functions for each variant.

use native_theme::icons::IconLoader;
use native_theme::theme::{AnimatedIcon, IconData, IconProvider};

/// Converted animation frames with timing metadata.
///
/// Returned by [`animated_frames_to_svg_handles`]. Contains the
/// SVG handles and the per-frame duration needed to drive playback.
#[derive(Debug, Clone)]
pub struct AnimatedSvgHandles {
    /// SVG handles ready for iced rendering.
    pub handles: Vec<iced_core::svg::Handle>,
    /// Duration of each frame in milliseconds.
    pub frame_duration_ms: u32,
}

/// Converts RGBA [`IconData`] to an iced [`iced_core::image::Handle`].
///
/// Returns `Some(Handle)` for [`IconData::Rgba`] data, or `None` for
/// [`IconData::Svg`]. SVG icons should use [`to_svg_handle()`] and
/// `iced::widget::Svg` instead.
#[must_use]
pub fn to_image_handle(data: &IconData) -> Option<iced_core::image::Handle> {
    match data {
        IconData::Rgba {
            width,
            height,
            data,
        } => Some(iced_core::image::Handle::from_rgba(
            *width,
            *height,
            data.clone(),
        )),
        _ => None,
    }
}

/// Converts SVG [`IconData`] to an iced [`iced_core::svg::Handle`].
///
/// Returns `Some(Handle)` for [`IconData::Svg`] data, or `None` for
/// [`IconData::Rgba`]. When `color` is `Some`, colorizes the SVG for
/// monochrome icon sets (Material, Lucide). Pass `None` for multi-color
/// system icons to preserve their native palette.
#[must_use]
pub fn to_svg_handle(
    data: &IconData,
    color: Option<iced_core::Color>,
) -> Option<iced_core::svg::Handle> {
    match data {
        IconData::Svg(cow) => {
            let final_bytes: Vec<u8> = if let Some(c) = color {
                colorize_monochrome_svg(cow, c)
            } else {
                cow.to_vec()
            };
            Some(iced_core::svg::Handle::from_memory(final_bytes))
        }
        _ => None,
    }
}

/// Load a custom RGBA icon from an [`IconProvider`] and convert to an iced image handle.
///
/// Returns `None` if the provider has no icon for the given set, or if the loaded
/// icon is SVG (use [`custom_icon_to_svg_handle()`] for SVG icons).
#[must_use]
pub fn custom_icon_to_image_handle(
    provider: &(impl IconProvider + ?Sized),
    icon_set: native_theme::theme::IconSet,
) -> Option<iced_core::image::Handle> {
    let data = load_custom_via_builder(provider, icon_set)?;
    to_image_handle(&data)
}

/// Load a custom SVG icon from an [`IconProvider`] and convert to an iced SVG handle.
///
/// Returns `None` if the provider has no icon for the given set, or if the loaded
/// icon is RGBA. When `color` is `Some`, colorizes monochrome SVGs.
#[must_use]
pub fn custom_icon_to_svg_handle(
    provider: &(impl IconProvider + ?Sized),
    icon_set: native_theme::theme::IconSet,
    color: Option<iced_core::Color>,
) -> Option<iced_core::svg::Handle> {
    let data = load_custom_via_builder(provider, icon_set)?;
    to_svg_handle(&data, color)
}

/// Internal helper: load an icon from a provider using [`IconLoader`].
///
/// Uses the provider's `icon_name` and `icon_svg` methods directly, then
/// dispatches through `IconLoader` for system lookups. This preserves
/// the `?Sized` bound on the public API.
fn load_custom_via_builder(
    provider: &(impl IconProvider + ?Sized),
    icon_set: native_theme::theme::IconSet,
) -> Option<IconData> {
    // Step 1: Try system loader with provider's name mapping
    if let Some(name) = provider.icon_name(icon_set) {
        if let Some(data) = IconLoader::new(name).set(icon_set).load() {
            return Some(data);
        }
    }
    // Step 2: Try bundled SVG from provider
    if let Some(svg) = provider.icon_svg(icon_set) {
        return Some(IconData::Svg(svg));
    }
    None
}

/// Convert all frames of an [`AnimatedIcon::Frames`] to iced SVG handles.
///
/// Returns `Some(AnimatedSvgHandles)` when the icon is the `Frames` variant
/// and at least one frame is SVG. Returns `None` for `Transform` variants,
/// empty frame sets, or if all frames are RGBA.
///
/// Only SVG frames are included. RGBA frames are silently excluded because
/// iced's `Svg` widget cannot render raster data. The returned animation may
/// have fewer frames than the input, causing it to play faster. If all frames
/// are non-SVG, returns `None`.
///
/// When `color` is `Some`, colorizes monochrome SVG frames (Material, Lucide)
/// to match the given color. Pass `None` for multi-color system icons.
///
/// **Call this once and cache the result.** Do not call on every frame tick.
/// Index into the cached `handles` using an `iced::time::every()` subscription
/// that increments a frame counter.
///
/// Callers should check [`native_theme::prefers_reduced_motion()`] and fall
/// back to [`AnimatedIcon::first_frame()`] for a static display when the user
/// has requested reduced motion.
///
/// # Examples
///
/// ```no_run
/// use native_theme_iced::icons::animated_frames_to_svg_handles;
///
/// if let Some(anim) = native_theme::icons::IconLoader::new(native_theme::theme::IconRole::StatusBusy).set(native_theme::theme::IconSet::Material).load_indicator() {
///     if let Some(anim_handles) = animated_frames_to_svg_handles(&anim, None) {
///         // Cache `anim_handles`, then in subscription():
///         // iced::time::every(Duration::from_millis(anim_handles.frame_duration_ms as u64))
///         //     .map(|_| Message::AnimationTick)
///         // In update(): frame_index = (frame_index + 1) % anim_handles.handles.len();
///         // In view(): Svg::new(anim_handles.handles[frame_index].clone())
///     }
/// }
/// ```
#[must_use]
pub fn animated_frames_to_svg_handles(
    anim: &AnimatedIcon,
    color: Option<iced_core::Color>,
) -> Option<AnimatedSvgHandles> {
    match anim {
        AnimatedIcon::Frames(data) => {
            let handles: Vec<_> = data
                .frames()
                .iter()
                .filter_map(|f| to_svg_handle(f, color))
                .collect();
            if handles.is_empty() {
                None
            } else {
                Some(AnimatedSvgHandles {
                    handles,
                    frame_duration_ms: data.frame_duration_ms().get(),
                })
            }
        }
        _ => None,
    }
}

/// Compute the current rotation angle for a spin animation.
///
/// Returns a [`Radians`](iced_core::Radians) value representing the current
/// rotation based on `elapsed` time and `duration_ms` (the full rotation
/// period from [`native_theme::theme::TransformAnimation::Spin`]).
///
/// The angle wraps around via modulo, so values of `elapsed` greater than
/// `duration_ms` cycle correctly.
///
/// Use the result with `Svg::rotation(Rotation::Floating(angle))` -- use
/// `Rotation::Floating` (not `Rotation::Solid`) to avoid layout jitter
/// during rotation.
///
/// Callers should check [`native_theme::prefers_reduced_motion()`] and
/// skip animation when the user has requested reduced motion.
///
/// # Examples
///
/// ```no_run
/// use native_theme_iced::icons::spin_rotation_radians;
///
/// let elapsed = std::time::Duration::from_millis(500);
/// let angle = spin_rotation_radians(elapsed, 1000);
/// // Use with: Svg::new(handle).rotation(Rotation::Floating(angle))
/// ```
#[must_use]
pub fn spin_rotation_radians(elapsed: std::time::Duration, duration_ms: u32) -> iced_core::Radians {
    if duration_ms == 0 {
        return iced_core::Radians(0.0);
    }
    let progress = (elapsed.as_millis() as f32 % duration_ms as f32) / duration_ms as f32;
    iced_core::Radians(progress * std::f32::consts::TAU)
}

/// Consuming version of [`to_image_handle`] — moves the [`IconData`] instead of borrowing.
#[must_use]
pub fn into_image_handle(data: IconData) -> Option<iced_core::image::Handle> {
    match data {
        IconData::Rgba {
            width,
            height,
            data,
        } => Some(iced_core::image::Handle::from_rgba(width, height, data)),
        _ => None,
    }
}

/// Consuming version of [`to_svg_handle`] — moves the [`IconData`] instead of borrowing.
#[must_use]
pub fn into_svg_handle(
    data: IconData,
    color: Option<iced_core::Color>,
) -> Option<iced_core::svg::Handle> {
    match data {
        IconData::Svg(cow) => {
            let final_bytes: Vec<u8> = if let Some(c) = color {
                colorize_monochrome_svg(&cow, c)
            } else {
                cow.into_owned()
            };
            Some(iced_core::svg::Handle::from_memory(final_bytes))
        }
        _ => None,
    }
}

/// Colorize a **monochrome** SVG icon with the given color.
///
/// Works correctly for bundled icon sets (Material, Lucide) which use
/// `currentColor` or implicit black fills. For multi-color SVGs from
/// freedesktop system themes, use [`to_svg_handle()`] instead to
/// preserve the original icon colors.
///
/// Note: Only RGB channels are used; the alpha channel is discarded during
/// hex conversion. For transparency, use `Svg::opacity()` on the rendered element.
///
/// ## Replacement strategy
///
/// 1. If `currentColor` appears anywhere in the SVG, it is replaced globally
///    (not scoped to individual attributes). This correctly handles `fill`,
///    `stroke`, `stop-color`, etc. For monochrome icons this is the desired
///    behavior. Multi-color SVGs should not use this function.
///
/// 2. Explicit black fills and strokes (`"black"`, `"#000000"`, `"#000"`)
///    are replaced in both `fill=` and `stroke=` attributes.
///
/// 3. If no replacements are found and the root `<svg>` tag lacks a `fill=`
///    attribute, a `fill` is injected. Note: if the root tag has `fill="none"`
///    (common in stroke-based SVGs), no injection occurs -- the explicit black
///    stroke replacements from step 2 handle colorization instead.
///
/// ## Limitations
///
/// - Colors in CSS `style` attributes (e.g., `style="fill:black"`) are not
///   replaced (except `currentColor` which is caught by step 1).
/// - Only the first `<svg` in the document is considered for fill injection.
///   SVGs with `<svg` inside comments could cause incorrect injection, though
///   this is extremely unlikely with real icon files.
fn colorize_monochrome_svg(svg_bytes: &[u8], color: iced_core::Color) -> Vec<u8> {
    let r = (color.r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (color.g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (color.b.clamp(0.0, 1.0) * 255.0).round() as u8;
    let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);

    // Validate UTF-8 before attempting string operations. Non-UTF-8 SVGs
    // (e.g., legacy Latin-1 encoded system icons) pass through unmodified
    // rather than being corrupted by lossy replacement.
    let Ok(svg_str) = std::str::from_utf8(svg_bytes) else {
        return svg_bytes.to_vec();
    };

    // Replace currentColor (handles Lucide-style SVGs).
    // This is a global replacement that covers fill, stroke, stop-color, etc.
    if svg_str.contains("currentColor") {
        return svg_str.replace("currentColor", &hex).into_bytes();
    }

    // Replace explicit black fills and strokes (handles third-party SVGs).
    // Stroke replacements handle outline-style icons that use stroke="black".
    let fill_hex = format!("fill=\"{}\"", hex);
    let stroke_hex = format!("stroke=\"{}\"", hex);
    let replaced = svg_str
        .replace("fill=\"black\"", &fill_hex)
        .replace("fill=\"#000000\"", &fill_hex)
        .replace("fill=\"#000\"", &fill_hex)
        .replace("stroke=\"black\"", &stroke_hex)
        .replace("stroke=\"#000000\"", &stroke_hex)
        .replace("stroke=\"#000\"", &stroke_hex);
    if replaced != svg_str {
        return replaced.into_bytes();
    }

    // No currentColor or explicit black found -- inject fill into the root
    // <svg> tag (handles Material-style SVGs with implicit black fill).
    if let Some(pos) = svg_str.find("<svg")
        && let Some(close) = svg_str[pos..].find('>')
    {
        let tag_end = pos + close;
        let tag = &svg_str[pos..tag_end];
        if !tag.contains("fill=") {
            // Handle self-closing tags: inject before '/' in '<svg .../>'
            let inject_pos = if tag_end > 0 && svg_str.as_bytes()[tag_end - 1] == b'/' {
                tag_end - 1
            } else {
                tag_end
            };
            let mut result = String::with_capacity(svg_str.len() + 20);
            result.push_str(&svg_str[..inject_pos]);
            result.push_str(&format!(" fill=\"{}\"", hex));
            result.push_str(&svg_str[inject_pos..]);
            return result.into_bytes();
        }
    }

    svg_bytes.to_vec()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use native_theme::theme::IconData;

    #[test]
    fn to_image_handle_with_rgba_returns_some() {
        let data = IconData::Rgba {
            width: 24,
            height: 24,
            data: vec![0u8; 24 * 24 * 4],
        };
        assert!(to_image_handle(&data).is_some());
    }

    #[test]
    fn to_image_handle_with_svg_returns_none() {
        let data = IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>"));
        assert!(to_image_handle(&data).is_none());
    }

    #[test]
    fn to_svg_handle_with_svg_returns_some() {
        let data = IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>"));
        assert!(to_svg_handle(&data, None).is_some());
    }

    #[test]
    fn to_svg_handle_with_rgba_returns_none() {
        let data = IconData::Rgba {
            width: 16,
            height: 16,
            data: vec![255u8; 16 * 16 * 4],
        };
        assert!(to_svg_handle(&data, None).is_none());
    }

    #[test]
    fn to_svg_handle_colored_replaces_current_color() {
        let data = IconData::Svg(std::borrow::Cow::Borrowed(
            b"<svg><path stroke=\"currentColor\" fill=\"currentColor\"/></svg>",
        ));
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);

        let handle = to_svg_handle(&data, Some(color));
        assert!(handle.is_some());

        // Verify the colorization happened by checking the internal SVG
        let colored = colorize_monochrome_svg(
            b"<svg><path stroke=\"currentColor\" fill=\"currentColor\"/></svg>",
            color,
        );
        let result = String::from_utf8(colored).unwrap();
        assert!(result.contains("#ff0000"));
        assert!(!result.contains("currentColor"));
    }

    #[test]
    fn to_svg_handle_colored_injects_fill_for_material_style() {
        let svg = b"<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\"/></svg>".to_vec();
        let color = iced_core::Color::from_rgb(0.0, 0.5, 1.0);

        let colored = colorize_monochrome_svg(&svg, color);
        let result = String::from_utf8(colored).unwrap();
        assert!(result.contains("fill=\"#0080ff\""));
    }

    #[test]
    fn to_svg_handle_colored_with_rgba_returns_none() {
        let data = IconData::Rgba {
            width: 16,
            height: 16,
            data: vec![0u8; 16 * 16 * 4],
        };
        let color = iced_core::Color::WHITE;
        assert!(to_svg_handle(&data, Some(color)).is_none());
    }

    // --- custom_icon tests ---

    #[derive(Debug)]
    struct TestSvgProvider;

    impl native_theme::theme::IconProvider for TestSvgProvider {
        fn icon_name(&self, _set: native_theme::theme::IconSet) -> Option<&str> {
            None
        }
        fn icon_svg(
            &self,
            _set: native_theme::theme::IconSet,
        ) -> Option<std::borrow::Cow<'static, [u8]>> {
            Some(std::borrow::Cow::Borrowed(
                b"<svg xmlns='http://www.w3.org/2000/svg'><circle cx='12' cy='12' r='10'/></svg>",
            ))
        }
    }

    #[derive(Debug)]
    struct EmptyProvider;

    impl native_theme::theme::IconProvider for EmptyProvider {
        fn icon_name(&self, _set: native_theme::theme::IconSet) -> Option<&str> {
            None
        }
        fn icon_svg(
            &self,
            _set: native_theme::theme::IconSet,
        ) -> Option<std::borrow::Cow<'static, [u8]>> {
            None
        }
    }

    #[test]
    fn custom_icon_to_image_handle_with_svg_provider_returns_none() {
        // SVG data is not RGBA, so to_image_handle returns None
        let result =
            custom_icon_to_image_handle(&TestSvgProvider, native_theme::theme::IconSet::Material);
        assert!(result.is_none());
    }

    #[test]
    fn custom_icon_to_svg_handle_with_svg_provider_returns_some() {
        let result = custom_icon_to_svg_handle(
            &TestSvgProvider,
            native_theme::theme::IconSet::Material,
            None,
        );
        assert!(result.is_some());
    }

    #[test]
    fn custom_icon_to_svg_handle_with_color_returns_some() {
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);
        let result = custom_icon_to_svg_handle(
            &TestSvgProvider,
            native_theme::theme::IconSet::Material,
            Some(color),
        );
        assert!(result.is_some());
    }

    #[test]
    fn custom_icon_to_image_handle_with_empty_provider_returns_none() {
        let result =
            custom_icon_to_image_handle(&EmptyProvider, native_theme::theme::IconSet::Material);
        assert!(result.is_none());
    }

    #[test]
    fn custom_icon_to_svg_handle_with_empty_provider_returns_none() {
        let result =
            custom_icon_to_svg_handle(&EmptyProvider, native_theme::theme::IconSet::Material, None);
        assert!(result.is_none());
    }

    #[test]
    fn custom_icon_helpers_accept_dyn_provider() {
        let boxed: Box<dyn native_theme::theme::IconProvider> = Box::new(TestSvgProvider);
        let result =
            custom_icon_to_svg_handle(&*boxed, native_theme::theme::IconSet::Material, None);
        assert!(result.is_some());
    }

    #[test]
    fn colorize_svg_preserves_existing_fill() {
        let svg = b"<svg fill=\"red\"><path d=\"M0 0\"/></svg>";
        let color = iced_core::Color::from_rgb(0.0, 1.0, 0.0);

        let colored = colorize_monochrome_svg(svg, color);
        let result = String::from_utf8(colored).unwrap();
        // Should not inject a second fill since one already exists
        assert!(result.contains("fill=\"red\""));
        assert!(!result.contains("#00ff00"));
    }

    // --- animated icon tests ---

    use std::time::Duration;

    #[test]
    fn animated_frames_returns_handles() {
        let anim = AnimatedIcon::frames(
            vec![
                IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>")),
                IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>")),
            ],
            std::num::NonZeroU32::new(80).expect("test constant"),
        )
        .expect("non-empty frames");
        let result = animated_frames_to_svg_handles(&anim, None);
        assert!(result.is_some());
        let anim_handles = result.unwrap();
        assert_eq!(anim_handles.handles.len(), 2);
        assert_eq!(anim_handles.frame_duration_ms, 80);
    }

    #[test]
    fn animated_frames_transform_returns_none() {
        let anim = AnimatedIcon::transform(
            IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>")),
            native_theme::theme::TransformAnimation::Spin {
                duration_ms: std::num::NonZeroU32::new(1000).expect("test constant"),
            },
        );
        let result = animated_frames_to_svg_handles(&anim, None);
        assert!(result.is_none());
    }

    #[test]
    fn animated_frames_empty_returns_none() {
        // Empty FrameList is rejected at construction, so this test verifies that.
        let result = AnimatedIcon::frames(
            vec![],
            std::num::NonZeroU32::new(80).expect("test constant"),
        );
        assert!(result.is_err());
    }

    #[test]
    fn animated_frames_rgba_only_returns_none() {
        let anim = AnimatedIcon::frames(
            vec![IconData::Rgba {
                width: 16,
                height: 16,
                data: vec![0u8; 16 * 16 * 4],
            }],
            std::num::NonZeroU32::new(80).expect("test constant"),
        )
        .expect("non-empty frames");
        let result = animated_frames_to_svg_handles(&anim, None);
        assert!(result.is_none());
    }

    #[test]
    fn spin_rotation_zero_elapsed() {
        let radians = spin_rotation_radians(Duration::ZERO, 1000);
        assert_eq!(radians, iced_core::Radians(0.0));
    }

    #[test]
    fn spin_rotation_half() {
        let radians = spin_rotation_radians(Duration::from_millis(500), 1000);
        let expected = std::f32::consts::PI;
        assert!(
            (radians.0 - expected).abs() < 0.001,
            "Expected ~{}, got {}",
            expected,
            radians.0
        );
    }

    #[test]
    fn spin_rotation_full_wraps() {
        let radians = spin_rotation_radians(Duration::from_millis(1000), 1000);
        assert!(
            radians.0.abs() < 0.001,
            "Expected ~0.0 (wrapped), got {}",
            radians.0
        );
    }

    #[test]
    fn spin_rotation_zero_duration_returns_zero() {
        let radians = spin_rotation_radians(Duration::from_millis(500), 0);
        assert_eq!(radians.0, 0.0, "zero duration should return 0.0, not NaN");
        assert!(!radians.0.is_nan(), "must not be NaN");
    }

    #[test]
    fn colorize_replaces_explicit_black_fill() {
        let svg = b"<svg><path fill=\"black\" d=\"M0 0\"/></svg>";
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);
        let result = colorize_monochrome_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            !result_str.contains("fill=\"black\""),
            "fill=\"black\" should be replaced, got: {}",
            result_str
        );
    }

    #[test]
    fn into_image_handle_with_rgba() {
        let data = IconData::Rgba {
            width: 24,
            height: 24,
            data: vec![0u8; 24 * 24 * 4],
        };
        assert!(into_image_handle(data).is_some());
    }

    #[test]
    fn into_image_handle_with_svg_returns_none() {
        let data = IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>"));
        assert!(into_image_handle(data).is_none());
    }

    #[test]
    fn into_svg_handle_with_svg() {
        let data = IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>"));
        assert!(into_svg_handle(data, None).is_some());
    }

    #[test]
    fn into_svg_handle_with_rgba_returns_none() {
        let data = IconData::Rgba {
            width: 16,
            height: 16,
            data: vec![0u8; 16 * 16 * 4],
        };
        assert!(into_svg_handle(data, None).is_none());
    }

    #[test]
    fn animated_frames_mixed_svg_rgba_filters_rgba() {
        let anim = AnimatedIcon::frames(
            vec![
                IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>")),
                IconData::Rgba {
                    width: 16,
                    height: 16,
                    data: vec![0u8; 16 * 16 * 4],
                },
                IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>")),
            ],
            std::num::NonZeroU32::new(80).expect("test constant"),
        )
        .expect("non-empty frames");
        let result = animated_frames_to_svg_handles(&anim, None);
        assert!(result.is_some());
        let handles = result.unwrap();
        // RGBA frame should be filtered out, leaving 2 SVG frames
        assert_eq!(handles.handles.len(), 2);
    }

    #[test]
    fn colorize_self_closing_svg_produces_valid_xml() {
        let svg = b"<svg xmlns='http://www.w3.org/2000/svg'/>";
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);
        let result = colorize_monochrome_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        // Should inject fill before '/', not between '/' and '>'
        assert!(
            result_str.contains("fill=\"#") && result_str.ends_with("/>"),
            "self-closing SVG should remain valid XML, got: {}",
            result_str
        );
        assert!(
            !result_str.contains("/ fill="),
            "fill should not be between / and >, got: {}",
            result_str
        );
    }

    #[test]
    fn colorize_replaces_fill_hex_000000() {
        let svg = b"<svg><path fill=\"#000000\" d=\"M0 0\"/></svg>";
        let color = iced_core::Color::from_rgb(0.0, 1.0, 0.0);
        let result = colorize_monochrome_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            result_str.contains("fill=\"#00ff00\""),
            "fill=\"#000000\" should be replaced, got: {}",
            result_str
        );
    }

    #[test]
    fn colorize_replaces_fill_hex_short() {
        let svg = b"<svg><path fill=\"#000\" d=\"M0 0\"/></svg>";
        let color = iced_core::Color::from_rgb(0.0, 0.0, 1.0);
        let result = colorize_monochrome_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            result_str.contains("fill=\"#0000ff\""),
            "fill=\"#000\" should be replaced, got: {}",
            result_str
        );
    }

    #[test]
    fn colorize_replaces_stroke_black() {
        let svg = b"<svg><path stroke=\"black\" d=\"M0 0\"/></svg>";
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);
        let result = colorize_monochrome_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            result_str.contains("stroke=\"#ff0000\""),
            "stroke=\"black\" should be replaced, got: {}",
            result_str
        );
    }

    #[test]
    fn colorize_replaces_stroke_hex_000000() {
        let svg = b"<svg><path stroke=\"#000000\" d=\"M0 0\"/></svg>";
        let color = iced_core::Color::from_rgb(0.0, 1.0, 0.0);
        let result = colorize_monochrome_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            result_str.contains("stroke=\"#00ff00\""),
            "stroke=\"#000000\" should be replaced, got: {}",
            result_str
        );
    }

    #[test]
    fn colorize_replaces_stroke_hex_short() {
        let svg = b"<svg><path stroke=\"#000\" d=\"M0 0\"/></svg>";
        let color = iced_core::Color::from_rgb(0.0, 0.0, 1.0);
        let result = colorize_monochrome_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            result_str.contains("stroke=\"#0000ff\""),
            "stroke=\"#000\" should be replaced, got: {}",
            result_str
        );
    }

    #[test]
    fn colorize_non_utf8_returns_original_bytes() {
        // SVG bytes with an invalid UTF-8 sequence (0xFF is never valid UTF-8)
        let svg: Vec<u8> = b"<svg>\xff<path d=\"M0 0\"/></svg>".to_vec();
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);
        let result = colorize_monochrome_svg(&svg, color);
        // Should return the original bytes unchanged, not corrupt them
        assert_eq!(result, svg, "non-UTF-8 SVG should pass through unmodified");
    }

    #[test]
    fn animated_frames_with_color_colorizes_frames() {
        let anim = AnimatedIcon::frames(
            vec![IconData::Svg(std::borrow::Cow::Borrowed(
                b"<svg><path fill=\"currentColor\"/></svg>",
            ))],
            std::num::NonZeroU32::new(80).expect("test constant"),
        )
        .expect("non-empty frames");
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);
        let result = animated_frames_to_svg_handles(&anim, Some(color));
        assert!(result.is_some(), "should produce handles with color");
    }

    // Issue 14.3: colorize with mixed fill attributes (both black and non-black)
    #[test]
    fn colorize_mixed_fill_attributes() {
        // SVG with both fill="black" (should be replaced) and fill="red" (should be preserved)
        let svg = b"<svg><rect fill=\"black\" width=\"10\" height=\"10\"/><rect fill=\"red\" width=\"10\" height=\"10\"/></svg>";
        let color = iced_core::Color::from_rgb(0.0, 1.0, 0.0);
        let result = colorize_monochrome_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            !result_str.contains("fill=\"black\""),
            "fill=\"black\" should be replaced, got: {}",
            result_str
        );
        assert!(
            result_str.contains("fill=\"red\""),
            "fill=\"red\" should be preserved, got: {}",
            result_str
        );
    }

    // Issue 14.3: colorize with non-black explicit fills (fill="white", fill="#FFF")
    #[test]
    fn colorize_non_black_fills_preserved() {
        let svg = b"<svg fill=\"white\"><path d=\"M0 0\"/></svg>";
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);
        let result = colorize_monochrome_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        // fill="white" is not black, should not be replaced by phases 1-2.
        // Phase 3 (root fill injection) should be skipped since fill= already exists.
        assert!(
            result_str.contains("fill=\"white\""),
            "fill=\"white\" should be preserved, got: {}",
            result_str
        );
    }

    // Issue 14.3: spin_rotation_radians with very large elapsed (wraps correctly)
    #[test]
    fn spin_rotation_large_elapsed_wraps() {
        let duration_ms = 1000;
        // 1_000_000 seconds = 1M full rotations. Result should be near 0.
        let elapsed = std::time::Duration::from_secs(1_000_000);
        let result = spin_rotation_radians(elapsed, duration_ms);
        // The progress should be (elapsed_ms % duration_ms) / duration_ms.
        // 1_000_000_000ms % 1000 = 0, so rotation should be ~0.
        assert!(
            result.0.abs() < 0.01,
            "very large elapsed should wrap to near-zero, got {}",
            result.0
        );
    }

    // Issue 14.3: from_preset with a single-variant preset
    #[test]
    fn from_preset_single_variant_fallback() {
        // catppuccin-mocha is dark-only; requesting light should still succeed
        // via fallback (into_variant tries the other mode when primary is empty)
        let result = crate::from_preset("catppuccin-mocha", false);
        assert!(
            result.is_ok(),
            "single-variant preset should fallback to available variant: {:?}",
            result.err()
        );
    }
}
