//! Icon conversion functions for the gpui connector.
//!
//! Provides two main functions:
//! - [`icon_name`]: Maps [`IconRole`] to gpui-component's [`IconName`] for the Lucide icon set.
//!   This is a zero-I/O shortcut since gpui-component already bundles Lucide SVGs.
//! - [`to_image_source`]: Converts [`IconData`] to a gpui [`ImageSource`] for rendering.

use gpui::{Image, ImageFormat, ImageSource};
use gpui_component::IconName;
use native_theme::{IconData, IconRole};
use std::sync::Arc;

/// Map an [`IconRole`] to a gpui-component [`IconName`] for the Lucide icon set.
///
/// Returns `Some(IconName)` for roles that have a direct Lucide equivalent in
/// gpui-component's bundled icon set. Returns `None` for roles where
/// gpui-component doesn't ship the corresponding Lucide icon.
///
/// This is a zero-I/O operation -- no icon files are loaded. The returned
/// `IconName` can be rendered directly via gpui-component's `Icon::new()`.
///
/// # Coverage
///
/// Maps 30 of the 42 `IconRole` variants to `IconName`. The 12 unmapped roles
/// (Shield, ActionSave, ActionPaste, ActionCut, ActionEdit, ActionRefresh,
/// ActionPrint, NavHome, TrashFull, DialogQuestion, Help, Lock) have no
/// corresponding Lucide icon in gpui-component 0.5.
///
/// # Examples
///
/// ```ignore
/// use native_theme::IconRole;
/// use native_theme_gpui::icons::icon_name;
///
/// assert_eq!(icon_name(IconRole::DialogWarning), Some(IconName::TriangleAlert));
/// assert_eq!(icon_name(IconRole::Shield), None);
/// ```
pub fn icon_name(role: IconRole) -> Option<IconName> {
    Some(match role {
        // Dialog / Alert
        IconRole::DialogWarning => IconName::TriangleAlert,
        IconRole::DialogError => IconName::CircleX,
        IconRole::DialogInfo => IconName::Info,
        IconRole::DialogSuccess => IconName::CircleCheck,

        // Window Controls
        IconRole::WindowClose => IconName::WindowClose,
        IconRole::WindowMinimize => IconName::WindowMinimize,
        IconRole::WindowMaximize => IconName::WindowMaximize,
        IconRole::WindowRestore => IconName::WindowRestore,

        // Common Actions
        IconRole::ActionDelete => IconName::Delete,
        IconRole::ActionCopy => IconName::Copy,
        IconRole::ActionUndo => IconName::Undo2,
        IconRole::ActionRedo => IconName::Redo2,
        IconRole::ActionSearch => IconName::Search,
        IconRole::ActionSettings => IconName::Settings,
        IconRole::ActionAdd => IconName::Plus,
        IconRole::ActionRemove => IconName::Minus,

        // Navigation
        IconRole::NavBack => IconName::ChevronLeft,
        IconRole::NavForward => IconName::ChevronRight,
        IconRole::NavUp => IconName::ChevronUp,
        IconRole::NavDown => IconName::ChevronDown,
        IconRole::NavMenu => IconName::Menu,

        // Files / Places
        IconRole::FileGeneric => IconName::File,
        IconRole::FolderClosed => IconName::FolderClosed,
        IconRole::FolderOpen => IconName::FolderOpen,
        IconRole::TrashEmpty => IconName::Delete,

        // Status
        IconRole::StatusLoading => IconName::Loader,
        IconRole::StatusCheck => IconName::Check,
        IconRole::StatusError => IconName::CircleX,

        // System
        IconRole::UserAccount => IconName::User,
        IconRole::Notification => IconName::Bell,

        // No Lucide equivalent in gpui-component 0.5
        _ => return None,
    })
}

/// Convert [`IconData`] to a gpui [`ImageSource`] for rendering.
///
/// - `IconData::Svg`: Wraps the SVG bytes in `Image::from_bytes(ImageFormat::Svg, ...)`.
/// - `IconData::Rgba`: Encodes as BMP with a BITMAPV4HEADER and wraps in
///   `Image::from_bytes(ImageFormat::Bmp, ...)`. This avoids needing the `png` crate.
///
/// # Examples
///
/// ```ignore
/// use native_theme::IconData;
/// use native_theme_gpui::icons::to_image_source;
///
/// let svg = IconData::Svg(b"<svg></svg>".to_vec());
/// let source = to_image_source(&svg);
/// ```
pub fn to_image_source(data: &IconData) -> ImageSource {
    match data {
        IconData::Svg(bytes) => {
            let image = Image::from_bytes(ImageFormat::Svg, bytes.clone());
            ImageSource::Image(Arc::new(image))
        }
        IconData::Rgba {
            width,
            height,
            data,
        } => {
            let bmp = encode_rgba_as_bmp(*width, *height, data);
            let image = Image::from_bytes(ImageFormat::Bmp, bmp);
            ImageSource::Image(Arc::new(image))
        }
        // Forward-compatible: treat unknown variants as empty SVG
        _ => {
            let image = Image::from_bytes(ImageFormat::Svg, Vec::new());
            ImageSource::Image(Arc::new(image))
        }
    }
}

/// Encode RGBA pixel data as a BMP with BITMAPV4HEADER.
///
/// BMP with a V4 header supports 32-bit RGBA via channel masks.
/// The pixel data is stored bottom-up (BMP convention) with no compression.
fn encode_rgba_as_bmp(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    let pixel_data_size = (width * height * 4) as usize;
    let header_size: u32 = 14; // BITMAPFILEHEADER
    let dib_header_size: u32 = 108; // BITMAPV4HEADER
    let file_size = header_size + dib_header_size + pixel_data_size as u32;

    let mut buf = Vec::with_capacity(file_size as usize);

    // BITMAPFILEHEADER (14 bytes)
    buf.extend_from_slice(b"BM"); // signature
    buf.extend_from_slice(&file_size.to_le_bytes()); // file size
    buf.extend_from_slice(&0u16.to_le_bytes()); // reserved1
    buf.extend_from_slice(&0u16.to_le_bytes()); // reserved2
    buf.extend_from_slice(&(header_size + dib_header_size).to_le_bytes()); // pixel data offset

    // BITMAPV4HEADER (108 bytes)
    buf.extend_from_slice(&dib_header_size.to_le_bytes()); // header size
    buf.extend_from_slice(&(width as i32).to_le_bytes()); // width
    // Negative height = top-down (avoids flipping rows)
    buf.extend_from_slice(&(-(height as i32)).to_le_bytes()); // height (top-down)
    buf.extend_from_slice(&1u16.to_le_bytes()); // planes
    buf.extend_from_slice(&32u16.to_le_bytes()); // bits per pixel
    buf.extend_from_slice(&3u32.to_le_bytes()); // compression = BI_BITFIELDS
    buf.extend_from_slice(&(pixel_data_size as u32).to_le_bytes()); // image size
    buf.extend_from_slice(&2835u32.to_le_bytes()); // x pixels per meter (~72 DPI)
    buf.extend_from_slice(&2835u32.to_le_bytes()); // y pixels per meter
    buf.extend_from_slice(&0u32.to_le_bytes()); // colors used
    buf.extend_from_slice(&0u32.to_le_bytes()); // important colors

    // Channel masks (RGBA -> BGRA in BMP, but we use BI_BITFIELDS to specify layout)
    buf.extend_from_slice(&0x00FF0000u32.to_le_bytes()); // red mask
    buf.extend_from_slice(&0x0000FF00u32.to_le_bytes()); // green mask
    buf.extend_from_slice(&0x000000FFu32.to_le_bytes()); // blue mask
    buf.extend_from_slice(&0xFF000000u32.to_le_bytes()); // alpha mask

    // Color space type: LCS_sRGB
    buf.extend_from_slice(&0x73524742u32.to_le_bytes()); // 'sRGB'

    // CIEXYZTRIPLE endpoints (36 bytes of zeros)
    buf.extend_from_slice(&[0u8; 36]);

    // Gamma values (red, green, blue) - unused with sRGB
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());

    // Pixel data: RGBA -> BGRA conversion for BMP
    for pixel in rgba.chunks_exact(4) {
        buf.push(pixel[2]); // B
        buf.push(pixel[1]); // G
        buf.push(pixel[0]); // R
        buf.push(pixel[3]); // A
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- icon_name tests ---

    #[test]
    fn icon_name_dialog_warning_maps_to_triangle_alert() {
        assert!(matches!(icon_name(IconRole::DialogWarning), Some(IconName::TriangleAlert)));
    }

    #[test]
    fn icon_name_dialog_error_maps_to_circle_x() {
        assert!(matches!(icon_name(IconRole::DialogError), Some(IconName::CircleX)));
    }

    #[test]
    fn icon_name_dialog_info_maps_to_info() {
        assert!(matches!(icon_name(IconRole::DialogInfo), Some(IconName::Info)));
    }

    #[test]
    fn icon_name_dialog_success_maps_to_circle_check() {
        assert!(matches!(icon_name(IconRole::DialogSuccess), Some(IconName::CircleCheck)));
    }

    #[test]
    fn icon_name_window_close_maps() {
        assert!(matches!(icon_name(IconRole::WindowClose), Some(IconName::WindowClose)));
    }

    #[test]
    fn icon_name_action_copy_maps_to_copy() {
        assert!(matches!(icon_name(IconRole::ActionCopy), Some(IconName::Copy)));
    }

    #[test]
    fn icon_name_nav_back_maps_to_chevron_left() {
        assert!(matches!(icon_name(IconRole::NavBack), Some(IconName::ChevronLeft)));
    }

    #[test]
    fn icon_name_file_generic_maps_to_file() {
        assert!(matches!(icon_name(IconRole::FileGeneric), Some(IconName::File)));
    }

    #[test]
    fn icon_name_status_check_maps_to_check() {
        assert!(matches!(icon_name(IconRole::StatusCheck), Some(IconName::Check)));
    }

    #[test]
    fn icon_name_user_account_maps_to_user() {
        assert!(matches!(icon_name(IconRole::UserAccount), Some(IconName::User)));
    }

    #[test]
    fn icon_name_notification_maps_to_bell() {
        assert!(matches!(icon_name(IconRole::Notification), Some(IconName::Bell)));
    }

    // None cases
    #[test]
    fn icon_name_shield_returns_none() {
        assert!(icon_name(IconRole::Shield).is_none());
    }

    #[test]
    fn icon_name_lock_returns_none() {
        assert!(icon_name(IconRole::Lock).is_none());
    }

    #[test]
    fn icon_name_action_save_returns_none() {
        assert!(icon_name(IconRole::ActionSave).is_none());
    }

    #[test]
    fn icon_name_help_returns_none() {
        assert!(icon_name(IconRole::Help).is_none());
    }

    #[test]
    fn icon_name_dialog_question_returns_none() {
        assert!(icon_name(IconRole::DialogQuestion).is_none());
    }

    // Count test: at least 28 roles map to Some
    #[test]
    fn icon_name_maps_at_least_28_roles() {
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(**r).is_some())
            .count();
        assert!(
            some_count >= 28,
            "Expected at least 28 mappings, got {}",
            some_count
        );
    }

    #[test]
    fn icon_name_maps_exactly_30_roles() {
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(**r).is_some())
            .count();
        assert_eq!(some_count, 30, "Expected exactly 30 mappings, got {some_count}");
    }

    // --- to_image_source tests ---

    #[test]
    fn to_image_source_svg_returns_image_source() {
        let svg = IconData::Svg(b"<svg></svg>".to_vec());
        let source = to_image_source(&svg);
        // Verify it's an ImageSource::Image variant
        match source {
            ImageSource::Image(arc) => {
                assert_eq!(arc.format, ImageFormat::Svg);
                assert_eq!(arc.bytes, b"<svg></svg>");
            }
            _ => panic!("Expected ImageSource::Image for SVG data"),
        }
    }

    #[test]
    fn to_image_source_rgba_returns_bmp_image_source() {
        let rgba = IconData::Rgba {
            width: 2,
            height: 2,
            data: vec![
                255, 0, 0, 255, // red
                0, 255, 0, 255, // green
                0, 0, 255, 255, // blue
                255, 255, 0, 255, // yellow
            ],
        };
        let source = to_image_source(&rgba);
        match source {
            ImageSource::Image(arc) => {
                assert_eq!(arc.format, ImageFormat::Bmp);
                // BMP header starts with "BM"
                assert_eq!(&arc.bytes[0..2], b"BM");
            }
            _ => panic!("Expected ImageSource::Image for RGBA data"),
        }
    }

    // --- BMP encoding tests ---

    #[test]
    fn encode_rgba_as_bmp_correct_file_size() {
        let rgba = vec![0u8; 4 * 4 * 4]; // 4x4 image
        let bmp = encode_rgba_as_bmp(4, 4, &rgba);
        let expected_size = 14 + 108 + (4 * 4 * 4); // header + dib + pixels
        assert_eq!(bmp.len(), expected_size);
    }

    #[test]
    fn encode_rgba_as_bmp_starts_with_bm() {
        let rgba = vec![0u8; 4]; // 1x1 image
        let bmp = encode_rgba_as_bmp(1, 1, &rgba);
        assert_eq!(&bmp[0..2], b"BM");
    }

    #[test]
    fn encode_rgba_as_bmp_pixel_order_is_bgra() {
        // Input RGBA: R=0xAA, G=0xBB, B=0xCC, A=0xDD
        let rgba = vec![0xAA, 0xBB, 0xCC, 0xDD];
        let bmp = encode_rgba_as_bmp(1, 1, &rgba);
        let pixel_offset = (14 + 108) as usize;
        // BMP stores as BGRA
        assert_eq!(bmp[pixel_offset], 0xCC); // B
        assert_eq!(bmp[pixel_offset + 1], 0xBB); // G
        assert_eq!(bmp[pixel_offset + 2], 0xAA); // R
        assert_eq!(bmp[pixel_offset + 3], 0xDD); // A
    }
}
