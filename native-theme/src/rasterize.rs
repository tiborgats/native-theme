// SVG-to-RGBA rasterization using resvg.
//
// Converts SVG byte data to IconData::Rgba pixel data at a specified size.
// Enabled behind the `svg-rasterize` feature gate.

use crate::IconData;
use resvg::tiny_skia;
use resvg::usvg;

/// Rasterize SVG bytes to RGBA pixel data at the given size.
///
/// Parses the SVG, scales it uniformly to fit within `size x size` pixels
/// (preserving aspect ratio), and renders to a pixel buffer with straight
/// (non-premultiplied) alpha.
///
/// # Errors
///
/// Returns [`crate::Error::Format`] if the SVG cannot be parsed or
/// the size is zero / pixmap allocation fails.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "svg-rasterize")]
/// # {
/// use native_theme::rasterize::rasterize_svg;
/// use native_theme::IconData;
///
/// let svg = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10'/></svg>";
/// let result = rasterize_svg(svg, 24);
/// assert!(result.is_ok());
/// if let Ok(IconData::Rgba { width, height, data }) = result {
///     assert_eq!(width, 24);
///     assert_eq!(height, 24);
///     assert_eq!(data.len(), 24 * 24 * 4);
/// }
/// # }
/// ```
#[must_use = "this returns the rasterized icon data"]
pub fn rasterize_svg(svg_bytes: &[u8], size: u32) -> crate::Result<IconData> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_data(svg_bytes, &options)
        .map_err(|e| crate::Error::Format(format!("SVG parse error: {e}")))?;

    let original_size = tree.size();
    let scale_x = size as f32 / original_size.width();
    let scale_y = size as f32 / original_size.height();
    let scale = scale_x.min(scale_y);

    // Center the icon if aspect ratio doesn't match
    let scaled_w = original_size.width() * scale;
    let scaled_h = original_size.height() * scale;
    let offset_x = (size as f32 - scaled_w) / 2.0;
    let offset_y = (size as f32 - scaled_h) / 2.0;

    let mut pixmap = tiny_skia::Pixmap::new(size, size)
        .ok_or_else(|| crate::Error::Format(format!("invalid rasterization size {size}x{size}")))?;
    let transform =
        tiny_skia::Transform::from_translate(offset_x, offset_y).post_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // resvg outputs premultiplied RGBA; convert to straight alpha
    let mut data = pixmap.take();
    crate::color::unpremultiply_alpha(&mut data);

    Ok(IconData::Rgba {
        width: size,
        height: size,
        data,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    const VALID_SVG: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10' fill='red'/></svg>";

    #[test]
    fn rasterize_valid_svg_returns_rgba() {
        let result = rasterize_svg(VALID_SVG, 24);
        assert!(result.is_ok(), "valid SVG should produce Ok");
        match result.unwrap() {
            IconData::Rgba {
                width,
                height,
                data,
            } => {
                assert_eq!(width, 24);
                assert_eq!(height, 24);
                assert_eq!(data.len(), 24 * 24 * 4);
            }
            _ => panic!("expected IconData::Rgba"),
        }
    }

    #[test]
    fn rasterize_invalid_svg_returns_err() {
        let result = rasterize_svg(b"not svg", 24);
        assert!(result.is_err(), "invalid SVG should return Err");
    }

    #[test]
    fn rasterize_output_length_matches_size() {
        for size in [16, 24, 32, 48, 64] {
            let result = rasterize_svg(VALID_SVG, size);
            let icon = result.unwrap_or_else(|_| panic!("should produce output for size {size}"));
            if let IconData::Rgba { data, .. } = icon {
                assert_eq!(
                    data.len(),
                    (size * size * 4) as usize,
                    "output length mismatch for size {size}"
                );
            }
        }
    }

    #[test]
    fn rasterize_produces_non_empty_pixels() {
        let result = rasterize_svg(VALID_SVG, 24).unwrap();
        if let IconData::Rgba { data, .. } = result {
            // At least some pixels should be non-zero (red circle on transparent bg)
            let non_zero = data.iter().any(|&b| b != 0);
            assert!(non_zero, "rasterized output should contain non-zero pixels");
        }
    }

    #[test]
    fn rasterize_straight_alpha() {
        // A semi-transparent SVG element
        let svg = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 2 2'><rect width='2' height='2' fill='white' fill-opacity='0.5'/></svg>";
        let result = rasterize_svg(svg, 2).unwrap();
        if let IconData::Rgba { data, .. } = result {
            // All pixels should be the same semi-transparent white
            for pixel in data.chunks_exact(4) {
                let (r, _g, _b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);
                if a > 0 {
                    // Straight alpha: R should be close to 255, not ~128
                    // (premultiplied would have R ~= 128 for alpha 128)
                    assert!(
                        r > 200,
                        "expected straight alpha (R near 255), got R={r} A={a}"
                    );
                }
            }
        }
    }

    #[test]
    fn rasterize_zero_size_returns_err() {
        let result = rasterize_svg(VALID_SVG, 0);
        assert!(result.is_err(), "zero size should return Err");
    }
}
