// macOS SF Symbols icon loader
//
// Resolves IconRole variants to RGBA pixel data by loading SF Symbols
// via NSImage and rasterizing through CGBitmapContext, with fallback
// to bundled Material SVGs when symbols are unavailable.

use crate::{bundled_icon_svg, icon_name, IconData, IconRole, IconSet};
use objc2::rc::Retained;
use objc2_app_kit::{NSFontWeight, NSImage, NSImageSymbolConfiguration, NSImageSymbolScale};
use objc2_core_graphics::{
    CGBitmapContextCreate, CGColorSpace, CGContext, CGImage, CGImageAlphaInfo, CGPoint, CGRect,
    CGSize,
};
use objc2_foundation::NSString;
use std::ffi::c_void;
use std::ptr;

/// Default icon size in pixels (suitable for toolbar/menu icons).
const DEFAULT_ICON_SIZE: u32 = 24;

/// Load an SF Symbol image by name with the given point size.
///
/// Creates an NSImage from the system symbol name, then applies
/// NSImageSymbolConfiguration with the given point size, regular weight,
/// and medium scale.
fn load_symbol(name: &str, point_size: f64) -> Option<Retained<NSImage>> {
    let ns_name = NSString::from_str(name);
    let image =
        NSImage::imageWithSystemSymbolName_accessibilityDescription(&ns_name, None)?;
    let config = NSImageSymbolConfiguration::configurationWithPointSize_weight_scale(
        point_size,
        NSFontWeight::Regular,
        NSImageSymbolScale::Medium,
    );
    image.imageWithSymbolConfiguration(&config)
}

/// Extract a CGImage from an NSImage at its natural pixel size.
///
/// Uses a null rect to let AppKit choose the best representation,
/// which on Retina displays will be at the full pixel resolution.
fn extract_cgimage(image: &NSImage) -> Option<Retained<CGImage>> {
    unsafe { image.CGImageForProposedRect_context_hints(ptr::null_mut(), None, None) }
}

/// Rasterize a CGImage to an RGBA pixel buffer using CGBitmapContext.
///
/// Creates a bitmap context with PremultipliedLast alpha format,
/// draws the image into it, and returns the raw RGBA pixel buffer.
fn rasterize(cg_image: &CGImage, width: u32, height: u32) -> Option<Vec<u8>> {
    let color_space = CGColorSpace::new_device_rgb()?;
    let bytes_per_row = (width as usize) * 4;
    let buf_size = bytes_per_row * (height as usize);
    let mut buffer = vec![0u8; buf_size];

    let bitmap_info = CGImageAlphaInfo::PremultipliedLast.0;

    let context = unsafe {
        CGBitmapContextCreate(
            buffer.as_mut_ptr() as *mut c_void,
            width as usize,
            height as usize,
            8,
            bytes_per_row,
            Some(&color_space),
            bitmap_info,
        )
    }?;

    let rect = CGRect {
        origin: CGPoint { x: 0.0, y: 0.0 },
        size: CGSize {
            width: width as f64,
            height: height as f64,
        },
    };
    CGContext::draw_image(Some(&context), rect, Some(cg_image));

    Some(buffer)
}

/// Convert premultiplied RGBA to straight (non-premultiplied) alpha.
///
/// For each pixel where `a > 0 && a < 255`:
///   `channel = min(255, channel * 255 / a)`
///
/// Fully opaque pixels (a == 255) are left unchanged.
/// Fully transparent pixels (a == 0) are left unchanged (RGB should
/// already be zero for premultiplied data).
fn unpremultiply_alpha(buffer: &mut [u8]) {
    for pixel in buffer.chunks_exact_mut(4) {
        let a = pixel[3] as u16;
        if a > 0 && a < 255 {
            pixel[0] = ((pixel[0] as u16 * 255) / a).min(255) as u8;
            pixel[1] = ((pixel[1] as u16 * 255) / a).min(255) as u8;
            pixel[2] = ((pixel[2] as u16 * 255) / a).min(255) as u8;
        }
    }
}

/// Load an SF Symbols icon for the given role as RGBA pixel data.
///
/// # Fallback chain
///
/// 1. SF Symbols via NSImage (macOS 11+, 38 of 42 roles have mappings)
/// 2. Bundled Material SVGs (requires `material-icons` feature, which
///    `system-icons` implies)
///
/// Returns `None` only if no icon is found at any level of the chain.
pub fn load_sf_icon(role: IconRole) -> Option<IconData> {
    if let Some(name) = icon_name(IconSet::SfSymbols, role) {
        let size = DEFAULT_ICON_SIZE;
        if let Some(image) = load_symbol(name, size as f64) {
            if let Some(cg_image) = extract_cgimage(&image) {
                // Read actual pixel dimensions from CGImage (not NSImage size)
                // to get correct Retina pixel counts
                let w = CGImage::width(&cg_image) as u32;
                let h = CGImage::height(&cg_image) as u32;
                if let Some(mut data) = rasterize(&cg_image, w, h) {
                    unpremultiply_alpha(&mut data);
                    return Some(IconData::Rgba {
                        width: w,
                        height: h,
                        data,
                    });
                }
            }
        }
    }

    // Bundled Material SVG fallback
    bundled_icon_svg(IconSet::Material, role).map(|bytes| IconData::Svg(bytes.to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpremultiply_correctness() {
        // Premultiplied [128, 0, 0, 128] -> straight [255, 0, 0, 128]
        let mut buf = [128u8, 0, 0, 128];
        unpremultiply_alpha(&mut buf);
        assert_eq!(buf, [255, 0, 0, 128]);

        // Fully opaque pixels are unchanged
        let mut buf = [100u8, 200, 50, 255];
        unpremultiply_alpha(&mut buf);
        assert_eq!(buf, [100, 200, 50, 255]);

        // Fully transparent pixels are unchanged
        let mut buf = [0u8, 0, 0, 0];
        unpremultiply_alpha(&mut buf);
        assert_eq!(buf, [0, 0, 0, 0]);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn load_icon_returns_some() {
        let result = load_sf_icon(IconRole::ActionCopy);
        assert!(result.is_some(), "ActionCopy should resolve to an icon");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn fallback_for_unmapped_role() {
        // WindowRestore has no SF Symbols name, should fall back to bundled
        let result = load_sf_icon(IconRole::WindowRestore);
        assert!(
            result.is_some(),
            "WindowRestore should fall back to bundled Material icon"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn rgba_dimensions_correct() {
        if let Some(IconData::Rgba {
            width,
            height,
            data,
        }) = load_sf_icon(IconRole::ActionCopy)
        {
            assert_eq!(
                (width * height * 4) as usize,
                data.len(),
                "RGBA buffer size must equal width * height * 4"
            );
        }
        // If it returns SVG fallback, that's also acceptable
    }
}
