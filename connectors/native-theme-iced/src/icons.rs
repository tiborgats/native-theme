//! Icon conversion helpers for iced.
//!
//! Converts [`native_theme::IconData`] variants into iced-compatible handles.
//! Since iced separates raster images (`iced::widget::Image`) from SVG
//! images (`iced::widget::Svg`), this module provides separate conversion
//! functions for each variant.

use native_theme::IconData;

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

/// Converts SVG [`IconData`] to an iced [`iced_core::svg::Handle`], colorized
/// with the given color.
///
/// For monochrome icon sets (Material, Lucide) whose SVGs use `currentColor`
/// or implicit black, this replaces colors with the specified theme color.
/// Returns `None` for [`IconData::Rgba`] data.
///
/// This is the iced equivalent of the gpui connector's `to_image_source_colored`.
pub fn to_svg_handle_colored(
    data: &IconData,
    color: iced_core::Color,
) -> Option<iced_core::svg::Handle> {
    match data {
        IconData::Svg(bytes) => {
            let colored = colorize_svg(bytes, color);
            Some(iced_core::svg::Handle::from_memory(colored))
        }
        _ => None,
    }
}

/// Rewrite SVG bytes to use the given color for strokes and fills.
///
/// - Replaces all occurrences of `currentColor` with the hex color.
/// - If the SVG has no `fill=` attribute in its root `<svg>` tag and didn't
///   contain `currentColor`, injects `fill="<hex>"` so that paths with
///   implicit black fill use the theme color instead.
fn colorize_svg(svg_bytes: &[u8], color: iced_core::Color) -> Vec<u8> {
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
        let colored = colorize_svg(
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

        let colored = colorize_svg(&svg, color);
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

    #[test]
    fn colorize_svg_preserves_existing_fill() {
        let svg = b"<svg fill=\"red\"><path d=\"M0 0\"/></svg>";
        let color = iced_core::Color::from_rgb(0.0, 1.0, 0.0);

        let colored = colorize_svg(svg, color);
        let result = String::from_utf8(colored).unwrap();
        // Should not inject a second fill since one already exists
        assert!(result.contains("fill=\"red\""));
        assert!(!result.contains("#00ff00"));
    }
}
