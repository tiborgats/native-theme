//! Icon conversion helpers for iced.
//!
//! Converts [`native_theme::IconData`] variants into iced-compatible handles.
//! Since iced separates raster images ([`iced_core::widget::Image`]) from SVG
//! images ([`iced_core::widget::Svg`]), this module provides separate conversion
//! functions for each variant.

use native_theme::IconData;

/// Converts RGBA [`IconData`] to an iced [`image::Handle`].
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

/// Converts SVG [`IconData`] to an iced [`svg::Handle`].
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
}
