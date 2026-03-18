// Linux freedesktop icon theme lookup
//
// Resolves IconRole variants to SVG bytes from the user's active desktop
// icon theme (Adwaita, Breeze, Papirus, etc.) using the freedesktop-icons
// crate. Returns None when the role has no freedesktop mapping or the
// icon is not found in the active theme.

use crate::model::animated::{AnimatedIcon, Repeat, TransformAnimation};
use crate::{IconData, IconRole, IconSet, icon_name};
use std::path::PathBuf;

/// Detect the active freedesktop icon theme.
///
/// Delegates to `system_icon_theme()` which handles DE-specific detection
/// (KDE reads kdeglobals, GNOME uses gsettings, etc.).
fn detect_theme() -> String {
    crate::system_icon_theme()
}

/// Look up an icon by freedesktop name using a two-pass strategy.
///
/// First tries the `-symbolic` suffix (single-frame static icons), then
/// falls back to the plain name. This order avoids animation sprite
/// sheets (e.g. Breeze's `animations/process-working.svg` is a 15-frame
/// vertical strip) which render incorrectly as static images.
///
/// The symbolic-first order also naturally handles Adwaita, which stores
/// most action icons only as `*-symbolic.svg`.
fn find_icon(name: &str, theme: &str, size: u16) -> Option<PathBuf> {
    // First try: symbolic variant (e.g., "edit-copy-symbolic")
    // Symbolic icons are always single-frame, avoiding sprite sheets
    // in themes like Breeze that put animation strips under plain names.
    let symbolic = format!("{name}-symbolic");
    if let Some(path) = freedesktop_icons::lookup(&symbolic)
        .with_theme(theme)
        .with_size(size)
        .force_svg()
        .find()
    {
        return Some(path);
    }
    // Second try: plain name (e.g., "edit-copy")
    freedesktop_icons::lookup(name)
        .with_theme(theme)
        .with_size(size)
        .force_svg()
        .find()
}

/// Load a freedesktop icon for the given role.
///
/// Resolves the role to a freedesktop icon name, looks it up in the
/// user's active icon theme (with `-symbolic` suffix fallback), and
/// returns the SVG bytes as `IconData::Svg`.
///
/// Returns `None` if the role has no freedesktop mapping or the icon
/// is not found in the active theme.
pub fn load_freedesktop_icon(role: IconRole) -> Option<IconData> {
    let theme = detect_theme();
    let name = icon_name(IconSet::Freedesktop, role)?;
    let path = find_icon(name, &theme, 24)?;
    let bytes = std::fs::read(&path).ok()?;
    Some(IconData::Svg(bytes))
}

/// Load a freedesktop icon by name from the given theme.
///
/// Looks up the name in the specified theme directory (with `-symbolic`
/// suffix fallback for Adwaita-style themes), reads the SVG file, and
/// returns it as `IconData::Svg`.
///
/// Unlike [`load_freedesktop_icon`] which takes an `IconRole`, this
/// function takes an arbitrary freedesktop icon name string. This is
/// used by connectors to load toolkit-specific icons beyond the 42
/// `IconRole` variants.
///
/// Returns `None` if the icon is not found in the theme.
pub fn load_freedesktop_icon_by_name(name: &str, theme: &str) -> Option<IconData> {
    let path = find_icon(name, theme, 24)?;
    let bytes = std::fs::read(&path).ok()?;
    Some(IconData::Svg(bytes))
}

/// Parse a vertical SVG sprite sheet into individual frame SVGs.
///
/// Detection: if the viewBox height > width and is an exact multiple,
/// the SVG is treated as a sprite sheet with `height/width` frames.
/// Each frame's SVG is the original with viewBox rewritten to window
/// into the correct vertical slice.
///
/// Returns `None` if the SVG is not a sprite sheet (single-frame,
/// non-multiple dimensions, or parse error).
fn parse_sprite_sheet(svg_bytes: &[u8]) -> Option<Vec<Vec<u8>>> {
    let svg_str = std::str::from_utf8(svg_bytes).ok()?;

    // Find viewBox attribute (handle both double and single quotes)
    let (vb_attr_start, vb_val_start, quote) =
        if let Some(i) = svg_str.find("viewBox=\"") {
            (i, i + 9, '"')
        } else if let Some(i) = svg_str.find("viewBox='") {
            (i, i + 9, '\'')
        } else {
            return None;
        };

    let vb_val_end = svg_str[vb_val_start..].find(quote)? + vb_val_start;
    let vb_value = &svg_str[vb_val_start..vb_val_end];

    // Split on whitespace or commas
    let parts: Vec<f64> = vb_value
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect();

    if parts.len() != 4 {
        return None;
    }

    let (width, height) = (parts[2], parts[3]);
    if height <= width {
        return None; // Single-frame, not a sprite sheet
    }

    let frame_count = (height / width).round() as usize;
    if frame_count < 2 {
        return None;
    }

    // Verify exact division (within floating-point tolerance)
    if (height - width * frame_count as f64).abs() > 0.01 {
        return None;
    }

    // Build the full original viewBox attribute string for replacement
    let original_vb_attr = &svg_str[vb_attr_start..vb_val_end + 1]; // includes closing quote

    let frames = (0..frame_count)
        .map(|i| {
            let y_offset = width * i as f64;
            let new_vb = format!("viewBox={quote}0 {y_offset} {width} {width}{quote}");
            svg_str.replacen(original_vb_attr, &new_vb, 1).into_bytes()
        })
        .collect();

    Some(frames)
}

/// Load the freedesktop loading spinner from the active icon theme.
///
/// Strategy:
/// 1. Try "process-working" (plain) at size 22 -- may be a sprite sheet -> Frames
/// 2. If found but single-frame (parse_sprite_sheet returns None) -> Transform::Spin
/// 3. Try "process-working-symbolic" at size 22 -- single frame -> Transform::Spin
/// 4. Return None if neither found (caller falls back to bundled Adwaita)
pub(crate) fn load_freedesktop_spinner() -> Option<AnimatedIcon> {
    let theme = detect_theme();

    // First pass: plain name (finds sprite sheets in animations/ dirs)
    if let Some(path) = freedesktop_icons::lookup("process-working")
        .with_theme(&theme)
        .with_size(22)
        .force_svg()
        .find()
    {
        let bytes = std::fs::read(&path).ok()?;
        if let Some(frames) = parse_sprite_sheet(&bytes) {
            return Some(AnimatedIcon::Frames {
                frames: frames.into_iter().map(IconData::Svg).collect(),
                frame_duration_ms: 80,
                repeat: Repeat::Infinite,
            });
        }
        // Not a sprite sheet -- treat as single frame with spin
        return Some(AnimatedIcon::Transform {
            icon: IconData::Svg(bytes),
            animation: TransformAnimation::Spin { duration_ms: 1000 },
        });
    }

    // Second pass: symbolic name (always single frame)
    if let Some(path) = freedesktop_icons::lookup("process-working-symbolic")
        .with_theme(&theme)
        .with_size(22)
        .force_svg()
        .find()
    {
        let bytes = std::fs::read(&path).ok()?;
        return Some(AnimatedIcon::Transform {
            icon: IconData::Svg(bytes),
            animation: TransformAnimation::Spin { duration_ms: 1000 },
        });
    }

    None
}

#[cfg(test)]
#[cfg(feature = "system-icons")]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires a freedesktop icon theme installed (not available on CI)"]
    fn load_icon_returns_some_for_dialog_error() {
        let result = load_freedesktop_icon(IconRole::DialogError);
        assert!(result.is_some(), "DialogError should resolve to an icon");
        match result.unwrap() {
            IconData::Svg(bytes) => {
                let content = std::str::from_utf8(&bytes).expect("SVG should be valid UTF-8");
                assert!(
                    content.contains("<svg"),
                    "Icon data should contain <svg tag"
                );
            }
            _ => panic!("Expected SVG data"),
        }
    }

    #[test]
    fn load_icon_notification_attempts_native_lookup() {
        // Notification is mapped to "notification-active" (KDE convention).
        // Result depends on whether the active theme ships this icon.
        // This test verifies the loader does not panic and does not fall back to Material.
        let _result = load_freedesktop_icon(IconRole::Notification);
        // No assertion on Some/None -- theme-dependent
    }

    #[test]
    #[ignore = "requires a freedesktop icon theme installed (not available on CI)"]
    fn load_icon_returns_svg_variant() {
        let result = load_freedesktop_icon(IconRole::ActionCopy);
        assert!(result.is_some(), "ActionCopy should resolve to an icon");
        assert!(
            matches!(result.unwrap(), IconData::Svg(_)),
            "Expected Svg variant"
        );
    }

    #[test]
    fn detect_theme_returns_non_empty() {
        let theme = detect_theme();
        assert!(!theme.is_empty(), "Theme name should not be empty");
    }

    #[test]
    fn find_icon_nonexistent_returns_none() {
        let result = find_icon("totally-nonexistent-icon-xyz", "hicolor", 24);
        assert!(result.is_none(), "Nonexistent icon should return None");
    }

    #[test]
    #[ignore = "requires a freedesktop icon theme installed (not available on CI)"]
    fn load_icon_by_name_finds_edit_copy() {
        let theme = detect_theme();
        let result = load_freedesktop_icon_by_name("edit-copy", &theme);
        assert!(
            result.is_some(),
            "edit-copy should be found in system theme"
        );
        assert!(matches!(result.unwrap(), IconData::Svg(_)));
    }

    #[test]
    fn load_icon_by_name_returns_none_for_nonexistent() {
        let result = load_freedesktop_icon_by_name("zzz-nonexistent-icon", "hicolor");
        assert!(result.is_none());
    }

    // === Sprite sheet parser tests ===

    #[test]
    fn test_parse_sprite_sheet_two_frames() {
        // 10x20 viewBox = 2 frames of 10x10
        let svg = br#"<svg viewBox="0 0 10 20" xmlns="http://www.w3.org/2000/svg">
            <rect x="0" y="0" width="10" height="10" fill="red"/>
            <rect x="0" y="10" width="10" height="10" fill="blue"/>
        </svg>"#;

        let frames = parse_sprite_sheet(svg).expect("should parse 2-frame sprite sheet");
        assert_eq!(frames.len(), 2);

        let frame0 = std::str::from_utf8(&frames[0]).unwrap();
        assert!(frame0.contains(r#"viewBox="0 0 10 10""#), "frame 0 viewBox: {frame0}");

        let frame1 = std::str::from_utf8(&frames[1]).unwrap();
        assert!(frame1.contains(r#"viewBox="0 10 10 10""#), "frame 1 viewBox: {frame1}");
    }

    #[test]
    fn test_parse_sprite_sheet_fifteen_frames() {
        // 22x330 viewBox = 15 frames (Breeze-like)
        let svg = br#"<svg viewBox="0 0 22 330" xmlns="http://www.w3.org/2000/svg">
            <path d="M0 0"/>
        </svg>"#;

        let frames = parse_sprite_sheet(svg).expect("should parse 15-frame sprite sheet");
        assert_eq!(frames.len(), 15);

        // Verify first and last frame viewBox values
        let first = std::str::from_utf8(&frames[0]).unwrap();
        assert!(first.contains(r#"viewBox="0 0 22 22""#));

        let last = std::str::from_utf8(&frames[14]).unwrap();
        assert!(last.contains(r#"viewBox="0 308 22 22""#));
    }

    #[test]
    fn test_parse_sprite_sheet_single_frame_returns_none() {
        // 22x22 = single frame, not a sprite sheet
        let svg = br#"<svg viewBox="0 0 22 22" xmlns="http://www.w3.org/2000/svg">
            <circle cx="11" cy="11" r="10"/>
        </svg>"#;
        assert!(parse_sprite_sheet(svg).is_none());
    }

    #[test]
    fn test_parse_sprite_sheet_non_multiple_returns_none() {
        // 22x33: height is not an exact multiple of width
        let svg = br#"<svg viewBox="0 0 22 33" xmlns="http://www.w3.org/2000/svg">
            <path d="M0 0"/>
        </svg>"#;
        assert!(parse_sprite_sheet(svg).is_none());
    }

    #[test]
    fn test_parse_sprite_sheet_invalid_svg_returns_none() {
        assert!(parse_sprite_sheet(b"not svg at all").is_none());
    }

    #[test]
    fn test_parse_sprite_sheet_comma_separated_viewbox() {
        // viewBox with commas instead of spaces
        let svg = br#"<svg viewBox="0,0,10,20" xmlns="http://www.w3.org/2000/svg">
            <rect x="0" y="0" width="10" height="10" fill="red"/>
        </svg>"#;

        let frames = parse_sprite_sheet(svg).expect("should parse comma-separated viewBox");
        assert_eq!(frames.len(), 2);

        let frame0 = std::str::from_utf8(&frames[0]).unwrap();
        assert!(frame0.contains(r#"viewBox="0 0 10 10""#));
    }

    #[test]
    fn test_parse_sprite_sheet_preserves_svg_content() {
        let svg = br#"<svg viewBox="0 0 10 20" xmlns="http://www.w3.org/2000/svg">
            <rect x="0" y="0" width="10" height="10" fill="red" id="unique-marker"/>
            <rect x="0" y="10" width="10" height="10" fill="blue"/>
        </svg>"#;

        let frames = parse_sprite_sheet(svg).unwrap();
        // Both frames should preserve the full SVG content
        for frame in &frames {
            let s = std::str::from_utf8(frame).unwrap();
            assert!(s.contains("unique-marker"), "SVG content should be preserved in all frames");
            assert!(s.contains("<rect"), "rect elements should be preserved");
            assert!(s.contains("xmlns="), "namespace should be preserved");
        }
    }

    #[test]
    fn test_load_freedesktop_spinner_no_panic() {
        // Just verify the function doesn't panic -- result is theme-dependent
        let _result = load_freedesktop_spinner();
    }
}
