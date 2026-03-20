//! Icon conversion helpers for iced.
//!
//! Converts [`native_theme::IconData`] variants into iced-compatible handles.
//! Since iced separates raster images (`iced::widget::Image`) from SVG
//! images (`iced::widget::Svg`), this module provides separate conversion
//! functions for each variant.

use native_theme::{AnimatedIcon, IconData, IconProvider, load_custom_icon};

/// Converts RGBA [`IconData`] to an iced [`iced_core::image::Handle`].
///
/// Returns `Some(Handle)` for [`IconData::Rgba`] data, or `None` for
/// [`IconData::Svg`]. SVG icons should use [`to_svg_handle()`] and
/// `iced::widget::Svg` instead.
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
/// [`IconData::Rgba`]. RGBA icons should use [`to_image_handle()`] and
/// `iced::widget::Image` instead.
pub fn to_svg_handle(data: &IconData) -> Option<iced_core::svg::Handle> {
    match data {
        IconData::Svg(bytes) => Some(iced_core::svg::Handle::from_memory(bytes.clone())),
        _ => None,
    }
}

/// Convert icon SVG data to an iced SVG handle, colorized with the given color.
///
/// Best suited for monochrome icons (bundled Material/Lucide sets).
/// For multi-color freedesktop theme icons, prefer [`to_svg_handle()`].
pub fn to_svg_handle_colored(
    data: &IconData,
    color: iced_core::Color,
) -> Option<iced_core::svg::Handle> {
    match data {
        IconData::Svg(bytes) => {
            let colored = colorize_monochrome_svg(bytes, color);
            Some(iced_core::svg::Handle::from_memory(colored))
        }
        _ => None,
    }
}

/// Load a custom RGBA icon from an [`IconProvider`] and convert to an iced image handle.
///
/// Returns `None` if the provider has no icon for the given set, or if the loaded
/// icon is SVG (use [`custom_icon_to_svg_handle()`] for SVG icons).
pub fn custom_icon_to_image_handle(
    provider: &(impl IconProvider + ?Sized),
    icon_set: &str,
) -> Option<iced_core::image::Handle> {
    let data = load_custom_icon(provider, icon_set)?;
    to_image_handle(&data)
}

/// Load a custom SVG icon from an [`IconProvider`] and convert to an iced SVG handle.
///
/// Returns `None` if the provider has no icon for the given set, or if the loaded
/// icon is RGBA (use [`custom_icon_to_image_handle()`] for RGBA icons).
pub fn custom_icon_to_svg_handle(
    provider: &(impl IconProvider + ?Sized),
    icon_set: &str,
) -> Option<iced_core::svg::Handle> {
    let data = load_custom_icon(provider, icon_set)?;
    to_svg_handle(&data)
}

/// Load a custom SVG icon from an [`IconProvider`] and convert to a colorized iced SVG handle.
///
/// Like [`custom_icon_to_svg_handle()`] but colorizes monochrome SVG icons with the
/// given color. Best for bundled icon sets. For multi-color system icons, prefer
/// [`custom_icon_to_svg_handle()`].
pub fn custom_icon_to_svg_handle_colored(
    provider: &(impl IconProvider + ?Sized),
    icon_set: &str,
    color: iced_core::Color,
) -> Option<iced_core::svg::Handle> {
    let data = load_custom_icon(provider, icon_set)?;
    to_svg_handle_colored(&data, color)
}

/// Convert all frames of an [`AnimatedIcon::Frames`] to iced SVG handles.
///
/// Returns `Some(Vec<svg::Handle>)` when the icon is the `Frames` variant and
/// at least one frame is SVG. Non-SVG (RGBA) frames are filtered out. Returns
/// `None` for `Transform` variants, empty frame sets, or if all frames are RGBA.
///
/// **Call this once and cache the result.** Do not call on every frame tick.
/// Index into the cached `Vec` using an `iced::time::every()` subscription
/// that increments a frame counter.
///
/// Callers should check [`native_theme::prefers_reduced_motion()`] and fall
/// back to [`AnimatedIcon::first_frame()`] for a static display when the user
/// has requested reduced motion.
///
/// # Examples
///
/// ```ignore
/// use native_theme_iced::icons::animated_frames_to_svg_handles;
///
/// let anim = native_theme::loading_indicator();
/// if let Some(handles) = animated_frames_to_svg_handles(&anim) {
///     // Cache `handles`, then in subscription():
///     // iced::time::every(Duration::from_millis(frame_duration_ms as u64))
///     //     .map(|_| Message::AnimationTick)
///     // In update(): frame_index = (frame_index + 1) % handles.len();
///     // In view(): Svg::new(handles[frame_index].clone())
/// }
/// ```
pub fn animated_frames_to_svg_handles(anim: &AnimatedIcon) -> Option<Vec<iced_core::svg::Handle>> {
    match anim {
        AnimatedIcon::Frames { frames, .. } => {
            let handles: Vec<_> = frames.iter().filter_map(to_svg_handle).collect();
            if handles.is_empty() {
                None
            } else {
                Some(handles)
            }
        }
        _ => None,
    }
}

/// Compute the current rotation angle for a spin animation.
///
/// Returns a [`Radians`](iced_core::Radians) value representing the current
/// rotation based on `elapsed` time and `duration_ms` (the full rotation
/// period from [`native_theme::TransformAnimation::Spin`]).
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
/// ```ignore
/// use native_theme_iced::icons::spin_rotation_radians;
/// use iced_core::Rotation;
///
/// let angle = spin_rotation_radians(self.elapsed, 1000);
/// Svg::new(handle).rotation(Rotation::Floating(angle))
/// ```
pub fn spin_rotation_radians(elapsed: std::time::Duration, duration_ms: u32) -> iced_core::Radians {
    let progress = (elapsed.as_millis() as f32 % duration_ms as f32) / duration_ms as f32;
    iced_core::Radians(progress * std::f32::consts::TAU)
}

/// Colorize a **monochrome** SVG icon with the given color.
///
/// Works correctly for bundled icon sets (Material, Lucide) which use
/// `currentColor` or implicit black fills. For multi-color SVGs from
/// freedesktop system themes, use [`to_svg_handle()`] instead to
/// preserve the original icon colors.
fn colorize_monochrome_svg(svg_bytes: &[u8], color: iced_core::Color) -> Vec<u8> {
    let r = (color.r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (color.g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (color.b.clamp(0.0, 1.0) * 255.0).round() as u8;
    let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);

    let svg_str = String::from_utf8_lossy(svg_bytes);

    // Replace currentColor (handles Lucide-style SVGs)
    if svg_str.contains("currentColor") {
        return svg_str.replace("currentColor", &hex).into_bytes();
    }

    // No currentColor found — inject fill into the root <svg> tag
    // (handles Material-style SVGs with implicit black fill)
    if let Some(pos) = svg_str.find("<svg")
        && let Some(close) = svg_str[pos..].find('>')
    {
        let tag_end = pos + close;
        let tag = &svg_str[pos..tag_end];
        if !tag.contains("fill=") {
            let mut result = String::with_capacity(svg_str.len() + 20);
            result.push_str(&svg_str[..tag_end]);
            result.push_str(&format!(" fill=\"{}\"", hex));
            result.push_str(&svg_str[tag_end..]);
            return result.into_bytes();
        }
    }

    svg_bytes.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use native_theme::IconData;

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
        let data = IconData::Svg(b"<svg></svg>".to_vec());
        assert!(to_image_handle(&data).is_none());
    }

    #[test]
    fn to_svg_handle_with_svg_returns_some() {
        let data = IconData::Svg(b"<svg></svg>".to_vec());
        assert!(to_svg_handle(&data).is_some());
    }

    #[test]
    fn to_svg_handle_with_rgba_returns_none() {
        let data = IconData::Rgba {
            width: 16,
            height: 16,
            data: vec![255u8; 16 * 16 * 4],
        };
        assert!(to_svg_handle(&data).is_none());
    }

    #[test]
    fn to_svg_handle_colored_replaces_current_color() {
        let svg = b"<svg><path stroke=\"currentColor\" fill=\"currentColor\"/></svg>".to_vec();
        let data = IconData::Svg(svg);
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);

        let handle = to_svg_handle_colored(&data, color);
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
        assert!(to_svg_handle_colored(&data, color).is_none());
    }

    // --- custom_icon tests ---

    #[derive(Debug)]
    struct TestSvgProvider;

    impl native_theme::IconProvider for TestSvgProvider {
        fn icon_name(&self, _set: native_theme::IconSet) -> Option<&str> {
            None
        }
        fn icon_svg(&self, _set: native_theme::IconSet) -> Option<&'static [u8]> {
            Some(b"<svg xmlns='http://www.w3.org/2000/svg'><circle cx='12' cy='12' r='10'/></svg>")
        }
    }

    #[derive(Debug)]
    struct EmptyProvider;

    impl native_theme::IconProvider for EmptyProvider {
        fn icon_name(&self, _set: native_theme::IconSet) -> Option<&str> {
            None
        }
        fn icon_svg(&self, _set: native_theme::IconSet) -> Option<&'static [u8]> {
            None
        }
    }

    #[test]
    fn custom_icon_to_image_handle_with_svg_provider_returns_none() {
        // SVG data is not RGBA, so to_image_handle returns None
        let result = custom_icon_to_image_handle(&TestSvgProvider, "material");
        assert!(result.is_none());
    }

    #[test]
    fn custom_icon_to_svg_handle_with_svg_provider_returns_some() {
        let result = custom_icon_to_svg_handle(&TestSvgProvider, "material");
        assert!(result.is_some());
    }

    #[test]
    fn custom_icon_to_svg_handle_colored_with_svg_provider_returns_some() {
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);
        let result = custom_icon_to_svg_handle_colored(&TestSvgProvider, "material", color);
        assert!(result.is_some());
    }

    #[test]
    fn custom_icon_to_image_handle_with_empty_provider_returns_none() {
        let result = custom_icon_to_image_handle(&EmptyProvider, "material");
        assert!(result.is_none());
    }

    #[test]
    fn custom_icon_to_svg_handle_with_empty_provider_returns_none() {
        let result = custom_icon_to_svg_handle(&EmptyProvider, "material");
        assert!(result.is_none());
    }

    #[test]
    fn custom_icon_helpers_accept_dyn_provider() {
        let boxed: Box<dyn native_theme::IconProvider> = Box::new(TestSvgProvider);
        let result = custom_icon_to_svg_handle(&*boxed, "material");
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
        let anim = AnimatedIcon::Frames {
            frames: vec![
                IconData::Svg(b"<svg></svg>".to_vec()),
                IconData::Svg(b"<svg></svg>".to_vec()),
            ],
            frame_duration_ms: 80,
            repeat: native_theme::Repeat::Infinite,
        };
        let result = animated_frames_to_svg_handles(&anim);
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn animated_frames_transform_returns_none() {
        let anim = AnimatedIcon::Transform {
            icon: IconData::Svg(b"<svg></svg>".to_vec()),
            animation: native_theme::TransformAnimation::Spin { duration_ms: 1000 },
        };
        let result = animated_frames_to_svg_handles(&anim);
        assert!(result.is_none());
    }

    #[test]
    fn animated_frames_empty_returns_none() {
        let anim = AnimatedIcon::Frames {
            frames: vec![],
            frame_duration_ms: 80,
            repeat: native_theme::Repeat::Infinite,
        };
        let result = animated_frames_to_svg_handles(&anim);
        assert!(result.is_none());
    }

    #[test]
    fn animated_frames_rgba_only_returns_none() {
        let anim = AnimatedIcon::Frames {
            frames: vec![IconData::Rgba {
                width: 16,
                height: 16,
                data: vec![0u8; 16 * 16 * 4],
            }],
            frame_duration_ms: 80,
            repeat: native_theme::Repeat::Infinite,
        };
        let result = animated_frames_to_svg_handles(&anim);
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
}
