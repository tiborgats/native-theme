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
/// (non-premultiplied) alpha. `size` is clamped to
/// `1..=`[`MAX_ICON_SIZE`](crate::icons::MAX_ICON_SIZE) first: 0 draws one
/// pixel, and a size above 512 draws at 512.
///
/// # Errors
///
/// Returns [`crate::Error::ReaderFailed`] with reader `"svg-rasterizer"`,
/// whose [`kind`](crate::Error::kind) is
/// [`ErrorKind::Platform`](crate::error::ErrorKind::Platform), if the SVG
/// cannot be parsed or its `size × size` RGBA buffer cannot be allocated.
///
/// An `<image>` is drawn only from an inline `data:` URL; an `href` naming a
/// file is not read.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "svg-rasterize")]
/// # {
/// use native_theme::rasterize::rasterize_svg;
/// use native_theme::icons::IconData;
///
/// let svg = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10'/></svg>";
/// let result = rasterize_svg(svg, 24);
/// assert!(result.is_ok());
/// if let Ok(IconData::Rgba(rgba)) = result {
///     assert_eq!(rgba.width(), 24);
///     assert_eq!(rgba.height(), 24);
///     assert_eq!(rgba.data().len(), 24 * 24 * 4);
/// }
/// # }
/// ```
pub fn rasterize_svg(svg_bytes: &[u8], size: u32) -> crate::Result<IconData> {
    let failed = |source: Box<dyn std::error::Error + Send + Sync>| crate::Error::ReaderFailed {
        reader: crate::error::reader_names::SVG_RASTERIZER,
        source,
    };
    let size = size.clamp(1, crate::icons::MAX_ICON_SIZE);
    let data_len = crate::icons::raster_bytes_in_bound(size, size).ok_or_else(|| {
        failed(
            format!(
                "rasterization size {size}x{size} is over {} bytes",
                crate::icons::MAX_ICON_DECODE_BYTES
            )
            .into(),
        )
    })?;

    // Only inline data: images; a string href would be read from disk
    // relative to the process's working directory.
    let options = usvg::Options {
        image_href_resolver: usvg::ImageHrefResolver {
            resolve_data: usvg::ImageHrefResolver::default_data_resolver(),
            resolve_string: Box::new(|_, _| None),
        },
        ..usvg::Options::default()
    };
    let tree = usvg::Tree::from_data(svg_bytes, &options).map_err(|e| failed(Box::new(e)))?;

    let original_size = tree.size();
    let scale_x = size as f32 / original_size.width();
    let scale_y = size as f32 / original_size.height();
    let scale = scale_x.min(scale_y);

    // Center the icon if aspect ratio doesn't match
    let scaled_w = original_size.width() * scale;
    let scaled_h = original_size.height() * scale;
    let offset_x = (size as f32 - scaled_w) / 2.0;
    let offset_y = (size as f32 - scaled_h) / 2.0;

    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(data_len)
        .map_err(|e| failed(Box::new(e)))?;
    buffer.resize(data_len, 0);
    let mut pixmap = tiny_skia::IntSize::from_wh(size, size)
        .and_then(|int_size| tiny_skia::Pixmap::from_vec(buffer, int_size))
        .ok_or_else(|| failed(format!("invalid rasterization size {size}x{size}").into()))?;
    let transform =
        tiny_skia::Transform::from_scale(scale, scale).post_translate(offset_x, offset_y);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // resvg outputs premultiplied RGBA; convert to straight alpha
    let mut data = pixmap.take();
    crate::color::unpremultiply_alpha(&mut data);

    IconData::rgba(size, size, data)
        .ok_or_else(|| failed(format!("rasterized pixels do not fill {size}x{size}").into()))
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
            IconData::Rgba(rgba) => {
                assert_eq!(rgba.width(), 24);
                assert_eq!(rgba.height(), 24);
                assert_eq!(rgba.data().len(), 24 * 24 * 4);
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
            if let IconData::Rgba(rgba) = icon {
                assert_eq!(
                    rgba.data().len(),
                    (size * size * 4) as usize,
                    "output length mismatch for size {size}"
                );
            }
        }
    }

    #[test]
    fn rasterize_produces_non_empty_pixels() {
        let result = rasterize_svg(VALID_SVG, 24).unwrap();
        if let IconData::Rgba(rgba) = result {
            // At least some pixels should be non-zero (red circle on transparent bg)
            let non_zero = rgba.data().iter().any(|&b| b != 0);
            assert!(non_zero, "rasterized output should contain non-zero pixels");
        }
    }

    #[test]
    fn rasterize_straight_alpha() {
        // A semi-transparent SVG element
        let svg = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 2 2'><rect width='2' height='2' fill='white' fill-opacity='0.5'/></svg>";
        let result = rasterize_svg(svg, 2).unwrap();
        if let IconData::Rgba(rgba) = result {
            // All pixels should be the same semi-transparent white
            for pixel in rgba.data().as_chunks::<4>().0 {
                let &[r, _g, _b, a] = pixel;
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

    /// A size is clamped to `1..=MAX_ICON_SIZE`: 0 draws one pixel, and a
    /// size past the bound draws at the bound rather than allocating
    /// `size² × 4` bytes.
    #[test]
    fn rasterize_clamps_the_size_to_max_icon_size() {
        use crate::icons::MAX_ICON_SIZE;
        for (size, side) in [
            (0, 1),
            (MAX_ICON_SIZE, MAX_ICON_SIZE),
            (MAX_ICON_SIZE + 1, MAX_ICON_SIZE),
            (100_000, MAX_ICON_SIZE),
            (u32::MAX, MAX_ICON_SIZE),
        ] {
            let Ok(IconData::Rgba(rgba)) = rasterize_svg(VALID_SVG, size) else {
                panic!("asked for {size}: no raster");
            };
            assert_eq!(
                (rgba.width(), rgba.height()),
                (side, side),
                "asked for {size}"
            );
        }
    }

    fn alpha_at(icon: &IconData, x: usize, y: usize) -> u8 {
        let IconData::Rgba(rgba) = icon else {
            panic!("expected IconData::Rgba");
        };
        let width = rgba.width() as usize;
        rgba.data()[(y * width + x) * 4 + 3]
    }

    #[test]
    fn rasterize_centres_a_wide_svg() {
        let svg = b"<svg xmlns='http://www.w3.org/2000/svg' width='10' height='5' viewBox='0 0 10 5'><rect width='10' height='5' fill='red'/></svg>";
        let icon = rasterize_svg(svg, 100).unwrap();
        assert_eq!(alpha_at(&icon, 50, 50), 255, "the centre is drawn");
        assert_eq!(
            alpha_at(&icon, 50, 26),
            255,
            "the drawn band starts at row 25"
        );
        assert_eq!(
            alpha_at(&icon, 50, 73),
            255,
            "the drawn band ends at row 75"
        );
        assert_eq!(alpha_at(&icon, 50, 23), 0, "above the band is transparent");
        assert_eq!(alpha_at(&icon, 50, 76), 0, "below the band is transparent");
    }

    #[test]
    fn rasterize_draws_nothing_from_a_file_href() {
        let dir = std::env::temp_dir().join(format!(
            "native-theme-rasterize-href-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let referenced = dir.join("red.svg");
        std::fs::write(&referenced, VALID_SVG).unwrap();
        let svg = format!(
            "<svg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' \
             viewBox='0 0 24 24'><image width='24' height='24' xlink:href='{}'/></svg>",
            referenced.display()
        );
        let icon = rasterize_svg(svg.as_bytes(), 24);
        let _ = std::fs::remove_dir_all(&dir);
        let IconData::Rgba(rgba) = icon.unwrap() else {
            panic!("expected IconData::Rgba");
        };
        assert!(
            rgba.data().iter().all(|&b| b == 0),
            "nothing is drawn from a file on disk"
        );
    }

    #[test]
    fn rasterize_draws_a_data_url_image() {
        let svg = b"<svg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' viewBox='0 0 24 24'><image width='24' height='24' xlink:href='data:image/svg+xml;utf8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 24 24%22%3E%3Crect width=%2224%22 height=%2224%22 fill=%22red%22/%3E%3C/svg%3E'/></svg>";
        let icon = rasterize_svg(svg, 24).unwrap();
        assert_eq!(
            alpha_at(&icon, 12, 12),
            255,
            "an inline data: image is drawn"
        );
    }
}
