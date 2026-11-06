// sRGB pixel utilities for the icon loaders

/// Convert premultiplied RGBA pixel data to straight (non-premultiplied) alpha.
///
/// For each pixel where `a > 0 && a < 255`:
///   `channel = min(255, channel * 255 / a)`
///
/// Fully opaque pixels (a == 255) and fully transparent pixels (a == 0)
/// are left unchanged.
///
/// Used by `rasterize`, `sficons`, and `winicons` modules (feature/platform gated).
#[allow(dead_code)]
pub(crate) fn unpremultiply_alpha(buffer: &mut [u8]) {
    // `as_chunks_mut::<4>()` splits the buffer into `[u8; 4]` pixels plus a
    // remainder shorter than one pixel; the remainder is dropped, as
    // `chunks_exact_mut(4)` did. Array destructuring binds each byte without
    // panic-prone indexing.
    for pixel in buffer.as_chunks_mut::<4>().0 {
        let [r, g, b, a] = pixel;
        let a_val = u16::from(*a);
        // `a_val in 1..=254` guarantees the divisor is non-zero; `saturating_mul`
        // cannot overflow u16 (max 255 * 255 = 65025 < 65535). The `.min(255)` cap
        // makes the final `as u8` a lossless conversion (no truncation of high bits).
        if (1..255).contains(&a_val) {
            #[allow(clippy::integer_division, clippy::arithmetic_side_effects)]
            {
                *r = (u16::from(*r).saturating_mul(255) / a_val).min(255) as u8;
                *g = (u16::from(*g).saturating_mul(255) / a_val).min(255) as u8;
                *b = (u16::from(*b).saturating_mul(255) / a_val).min(255) as u8;
            }
        }
    }
}

/// Draw a monochrome straight-alpha RGBA raster in `rgb`.
///
/// Every pixel takes `rgb` and keeps its alpha, so the glyph's coverage is
/// unchanged. `None` leaves the buffer as it is.
///
/// Used by `sficons` and `winicons` (platform gated) for their monochrome
/// glyphs; full-colour icons are never passed here.
#[allow(dead_code)]
pub(crate) fn tint_monochrome(buffer: &mut [u8], rgb: Option<[u8; 3]>) {
    let Some([r, g, b]) = rgb else {
        return;
    };
    for pixel in buffer.as_chunks_mut::<4>().0 {
        let [_, _, _, a] = *pixel;
        *pixel = [r, g, b, a];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === tint_monochrome tests ===

    #[test]
    fn tint_monochrome_colours_a_white_glyph_and_keeps_alpha() {
        let mut buf = [255u8, 255, 255, 255, 255, 255, 255, 127];
        tint_monochrome(&mut buf, Some([10, 20, 30]));
        assert_eq!(buf, [10, 20, 30, 255, 10, 20, 30, 127]);
    }

    #[test]
    fn tint_monochrome_keeps_a_transparent_pixel_transparent() {
        let mut buf = [0u8, 0, 0, 0];
        tint_monochrome(&mut buf, Some([10, 20, 30]));
        assert_eq!(buf[3], 0);
    }

    #[test]
    fn tint_monochrome_without_a_colour_leaves_the_bytes() {
        let original = [255u8, 255, 255, 127, 0, 0, 0, 255, 1, 2, 3, 0];
        let mut buf = original;
        tint_monochrome(&mut buf, None);
        assert_eq!(buf, original);
    }
}
