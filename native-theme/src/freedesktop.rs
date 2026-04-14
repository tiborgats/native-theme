// Linux freedesktop icon theme lookup
//
// Resolves IconRole variants to SVG bytes from the user's active desktop
// icon theme (Adwaita, Breeze, Papirus, etc.) using the freedesktop-icons
// crate. Returns None when the role has no freedesktop mapping or the
// icon is not found in the active theme.

use std::borrow::Cow;
use std::num::NonZeroU32;

use crate::model::animated::{AnimatedIcon, TransformAnimation};
use crate::{IconData, IconRole, IconSet, icon_name};
use std::path::PathBuf;

/// Frame duration for freedesktop sprite sheet animations (80ms per frame).
const FREEDESKTOP_FRAME_DURATION_MS: u32 = 80;

/// Spin duration for single-frame icons animated with rotation (1 second).
const FREEDESKTOP_SPIN_DURATION_MS: u32 = 1000;

/// Detect the current freedesktop icon theme name.
///
/// Delegates to [`crate::system_icon_theme()`] which handles DE-specific
/// detection (KDE reads kdeglobals, GNOME uses gsettings, etc.). The result
/// is cached by [`DetectionContext`](crate::detect::DetectionContext).
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
fn find_icon(name: &str, theme: &str, size: u16) -> Option<(PathBuf, bool)> {
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
        return Some((path, true));
    }
    // Second try: plain name (e.g., "edit-copy")
    // If the name itself already ends with "-symbolic" (caller passed it
    // explicitly via load_freedesktop_icon_by_name), mark as symbolic.
    freedesktop_icons::lookup(name)
        .with_theme(theme)
        .with_size(size)
        .force_svg()
        .find()
        .map(|path| (path, name.ends_with("-symbolic")))
}

/// Load a freedesktop icon for the given role.
///
/// Resolves the role to a freedesktop icon name, looks it up in the
/// user's active icon theme (with `-symbolic` suffix fallback), and
/// returns the SVG bytes as `IconData::Svg`.
///
/// For GTK-convention symbolic icons (Adwaita, Yaru, elementary), the
/// hardcoded foreground placeholders are replaced with `fg_color` so
/// the SVG renders correctly on both light and dark themes. Pass
/// `None` to fall back to `currentColor` (requires connector
/// colorization).
///
/// Returns `None` if the role has no freedesktop mapping or the icon
/// is not found in the active theme.
///
/// **Performance note:** Each call reads the icon file from disk. Callers
/// that load the same icon repeatedly (e.g. per-frame in a GUI) should
/// cache the returned `IconData` themselves.
#[must_use]
pub(crate) fn load_freedesktop_icon(
    role: IconRole,
    size: u16,
    fg_color: Option<[u8; 3]>,
) -> Option<IconData> {
    let theme = detect_theme();
    let name = icon_name(role, IconSet::Freedesktop)?;
    let (path, is_symbolic) = find_icon(name, &theme, size)?;
    let bytes = std::fs::read(&path).ok()?;
    let bytes = if is_symbolic {
        let replacement = fg_to_replacement(fg_color);
        normalize_gtk_symbolic(bytes, &replacement)
    } else {
        bytes
    };
    Some(IconData::Svg(Cow::Owned(bytes)))
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
/// For GTK-convention symbolic icons, pass `fg_color` to bake the
/// correct foreground into the SVG (see [`load_freedesktop_icon`]).
///
/// Returns `None` if the icon is not found in the theme.
///
/// **Performance note:** Each call reads the icon file from disk. Callers
/// that load the same icon repeatedly should cache the returned `IconData`.
#[must_use]
pub fn load_freedesktop_icon_by_name(
    name: &str,
    theme: &str,
    size: u16,
    fg_color: Option<[u8; 3]>,
) -> Option<IconData> {
    let (path, is_symbolic) = find_icon(name, theme, size)?;
    let bytes = std::fs::read(&path).ok()?;
    let bytes = if is_symbolic {
        let replacement = fg_to_replacement(fg_color);
        normalize_gtk_symbolic(bytes, &replacement)
    } else {
        bytes
    };
    Some(IconData::Svg(Cow::Owned(bytes)))
}

/// Convert an optional RGB foreground color to a replacement string
/// for GTK symbolic icon normalization.
fn fg_to_replacement(fg_color: Option<[u8; 3]>) -> String {
    match fg_color {
        Some([r, g, b]) => format!("#{r:02x}{g:02x}{b:02x}"),
        None => "currentColor".to_string(),
    }
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
    let (vb_attr_start, vb_val_start, quote) = if let Some(i) = svg_str.find("viewBox=\"") {
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
        let frame_dur = NonZeroU32::new(FREEDESKTOP_FRAME_DURATION_MS)?;
        let spin_dur = NonZeroU32::new(FREEDESKTOP_SPIN_DURATION_MS)?;
        if let Some(frames) = parse_sprite_sheet(&bytes) {
            let frame_icons: Vec<IconData> = frames
                .into_iter()
                .map(|b| IconData::Svg(Cow::Owned(b)))
                .collect();
            return AnimatedIcon::frames(frame_icons, frame_dur).ok();
        }
        // Not a sprite sheet -- treat as single frame with spin
        return Some(AnimatedIcon::transform(
            IconData::Svg(Cow::Owned(bytes)),
            TransformAnimation::Spin {
                duration_ms: spin_dur,
            },
        ));
    }

    // Second pass: symbolic name (always single frame)
    if let Some(path) = freedesktop_icons::lookup("process-working-symbolic")
        .with_theme(&theme)
        .with_size(22)
        .force_svg()
        .find()
    {
        let bytes = std::fs::read(&path).ok()?;
        let spin_dur = NonZeroU32::new(FREEDESKTOP_SPIN_DURATION_MS)?;
        return Some(AnimatedIcon::transform(
            IconData::Svg(Cow::Owned(bytes)),
            TransformAnimation::Spin {
                duration_ms: spin_dur,
            },
        ));
    }

    None
}

/// The GTK symbolic icon foreground placeholder colors.
///
/// GTK's icon rendering pipeline replaces these at paint time with the
/// widget's CSS `color` property (the text/foreground color). We replace
/// them with the caller-provided foreground color so the SVG is
/// self-contained and renders correctly without connector colorization.
///
/// Measured from `/usr/share/icons/Adwaita/symbolic/`:
/// - `#2e3436`: 483 fill attrs + 8 CSS style fills + 1 stroke (Tango Aluminium 6)
/// - `#2e3434`: 118 files (68 primary, 50 with fill-opacity)
/// - `#222222`: 27 occurrences (primary + dimmed)
/// - `#474747`: 50 emote/legacy icons (monochrome, never mixed with above)
const GTK_FG_COLORS: &[&str] = &["#2e3436", "#2e3434", "#222222", "#474747"];

/// Recolor a GTK-convention symbolic SVG by replacing foreground placeholders.
///
/// GTK symbolic icons use hardcoded dark fill colors (e.g., `#2e3436`)
/// that GTK replaces at render time with the widget's CSS text color.
/// This function does the same: it replaces those placeholders with
/// `replacement`, which should be either a hex color (e.g., `#ffffff`)
/// or `currentColor` as a fallback.
///
/// Handles three placement patterns found in Adwaita:
/// - XML attributes: `fill="#2e3436"`, `stroke="#2e3436"`
/// - CSS style attributes: `style="fill:#2e3436;..."`
///
/// Only foreground placeholder colors are replaced. Semantic colors
/// (success green `#33d17a`, warning orange `#ff7800`, error red
/// `#e01b24`/`#ed333b`) are preserved.
///
/// Returns the original bytes unchanged if the SVG already uses
/// `currentColor` (Breeze-style) or is not valid UTF-8.
fn normalize_gtk_symbolic(svg_bytes: Vec<u8>, replacement: &str) -> Vec<u8> {
    let Ok(svg_str) = std::str::from_utf8(&svg_bytes) else {
        return svg_bytes;
    };

    // Already uses currentColor (Breeze convention) -- no normalization needed
    if svg_str.contains("currentColor") {
        return svg_bytes;
    }

    // Check if any GTK foreground colors are present
    if !GTK_FG_COLORS.iter().any(|c| svg_str.contains(c)) {
        return svg_bytes;
    }

    let mut result = svg_str.to_string();
    for color in GTK_FG_COLORS {
        // XML attributes: fill="..." and stroke="..."
        result = result.replace(
            &format!("fill=\"{color}\""),
            &format!("fill=\"{replacement}\""),
        );
        result = result.replace(
            &format!("stroke=\"{color}\""),
            &format!("stroke=\"{replacement}\""),
        );
        // CSS style attributes: fill:#2e3436 (8 icons use this form)
        result = result.replace(&format!("fill:{color}"), &format!("fill:{replacement}"));
        result = result.replace(&format!("stroke:{color}"), &format!("stroke:{replacement}"));
    }
    result.into_bytes()
}

#[cfg(test)]
#[cfg(feature = "system-icons")]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires a freedesktop icon theme installed (not available on CI)"]
    fn load_icon_returns_some_for_dialog_error() {
        let result = load_freedesktop_icon(IconRole::DialogError, 24, None);
        assert!(result.is_some(), "DialogError should resolve to an icon");
        match result.unwrap() {
            IconData::Svg(ref cow) => {
                let s = String::from_utf8_lossy(cow);
                assert!(s.contains("<svg"), "Icon data should contain <svg tag");
            }
            other => assert!(false, "Expected SVG data, got {other:?}"),
        }
    }

    #[test]
    fn load_icon_notification_attempts_native_lookup() {
        // Notification is mapped to "notification-active" (KDE convention).
        // Result depends on whether the active theme ships this icon.
        // This test verifies the loader does not panic and does not fall back to Material.
        let _result = load_freedesktop_icon(IconRole::Notification, 24, None);
        // No assertion on Some/None -- theme-dependent
    }

    #[test]
    #[ignore = "requires a freedesktop icon theme installed (not available on CI)"]
    fn load_icon_returns_svg_variant() {
        let result = load_freedesktop_icon(IconRole::ActionCopy, 24, None);
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
        let result = load_freedesktop_icon_by_name("edit-copy", &theme, 24, None);
        assert!(
            result.is_some(),
            "edit-copy should be found in system theme"
        );
        assert!(matches!(result.unwrap(), IconData::Svg(_)));
    }

    #[test]
    fn load_icon_by_name_returns_none_for_nonexistent() {
        let result = load_freedesktop_icon_by_name("zzz-nonexistent-icon", "hicolor", 24, None);
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
        assert!(
            frame0.contains(r#"viewBox="0 0 10 10""#),
            "frame 0 viewBox: {frame0}"
        );

        let frame1 = std::str::from_utf8(&frames[1]).unwrap();
        assert!(
            frame1.contains(r#"viewBox="0 10 10 10""#),
            "frame 1 viewBox: {frame1}"
        );
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
            assert!(
                s.contains("unique-marker"),
                "SVG content should be preserved in all frames"
            );
            assert!(s.contains("<rect"), "rect elements should be preserved");
            assert!(s.contains("xmlns="), "namespace should be preserved");
        }
    }

    #[test]
    fn test_load_freedesktop_spinner_no_panic() {
        // Just verify the function doesn't panic -- result is theme-dependent
        let _result = load_freedesktop_spinner();
    }

    // === GTK symbolic icon normalization tests ===

    #[test]
    fn normalize_gtk_symbolic_replaces_2e3436() {
        let svg = br##"<svg><path fill="#2e3436" d="M0 0"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains(r##"fill="#ffffff""##));
        assert!(!s.contains("#2e3436"));
    }

    #[test]
    fn normalize_gtk_symbolic_replaces_2e3434_preserves_opacity() {
        let svg = br##"<svg><path fill="#2e3434" fill-opacity="0.35" d="M0 0"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains(r##"fill="#ffffff""##));
        assert!(s.contains(r#"fill-opacity="0.35""#));
    }

    #[test]
    fn normalize_gtk_symbolic_replaces_222222() {
        let svg = br##"<svg><path fill="#222222" d="M0 0"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains(r##"fill="#ffffff""##));
        assert!(!s.contains("#222222"));
    }

    #[test]
    fn normalize_gtk_symbolic_replaces_474747() {
        let svg = br##"<svg><path fill="#474747" d="M0 0"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains(r##"fill="#ffffff""##));
        assert!(!s.contains("#474747"));
    }

    #[test]
    fn normalize_gtk_symbolic_replaces_stroke() {
        let svg = br##"<svg><path stroke="#2e3436" fill="none" d="M1 1l14 14"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains(r##"stroke="#ffffff""##));
        assert!(!s.contains("#2e3436"));
    }

    #[test]
    fn normalize_gtk_symbolic_replaces_css_style_fill() {
        let svg = br##"<svg><path style="fill:#2e3436;fill-opacity:1" d="M0 0"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains("fill:#ffffff"));
        assert!(!s.contains("#2e3436"));
    }

    #[test]
    fn normalize_gtk_symbolic_preserves_semantic_colors() {
        let svg = br##"<svg><path fill="#2e3436"/><path fill="#ff7800"/><path fill="#33d17a"/><path fill="#e01b24"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains("#ffffff"));
        assert!(s.contains("#ff7800"), "warning color must be preserved");
        assert!(s.contains("#33d17a"), "success color must be preserved");
        assert!(s.contains("#e01b24"), "error color must be preserved");
    }

    #[test]
    fn normalize_gtk_symbolic_skips_currentcolor_svgs() {
        let svg = br##"<svg><defs><style>.ColorScheme-Text{color:#232629}</style></defs><path fill="currentColor"/></svg>"##.to_vec();
        let original = svg.clone();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        assert_eq!(
            result, original,
            "Breeze-style SVGs should pass through unchanged"
        );
    }

    #[test]
    fn normalize_gtk_symbolic_skips_non_gtk_svgs() {
        let svg = br#"<svg><path fill="red"/></svg>"#.to_vec();
        let original = svg.clone();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        assert_eq!(
            result, original,
            "non-GTK SVGs should pass through unchanged"
        );
    }
}
