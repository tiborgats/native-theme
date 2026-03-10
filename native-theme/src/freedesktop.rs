// Linux freedesktop icon theme lookup
//
// Resolves IconRole variants to SVG bytes from the user's active desktop
// icon theme (Adwaita, Breeze, Papirus, etc.) using the freedesktop-icons
// crate, with a full fallback chain: active theme -> hicolor -> bundled
// Material SVGs.

use crate::{bundled_icon_svg, icon_name, IconData, IconRole, IconSet};
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
/// First tries the plain name (works for Breeze, Papirus, most themes),
/// then tries with `-symbolic` suffix appended (needed for Adwaita which
/// stores action icons only as symbolic variants).
fn find_icon(name: &str, theme: &str, size: u16) -> Option<PathBuf> {
    // First try: plain name (e.g., "edit-copy")
    if let Some(path) = freedesktop_icons::lookup(name)
        .with_theme(theme)
        .with_size(size)
        .force_svg()
        .find()
    {
        return Some(path);
    }
    // Second try: symbolic variant (e.g., "edit-copy-symbolic")
    // Needed for Adwaita where actions only exist as *-symbolic.svg
    let symbolic = format!("{name}-symbolic");
    freedesktop_icons::lookup(&symbolic)
        .with_theme(theme)
        .with_size(size)
        .force_svg()
        .find()
}

/// Load a freedesktop icon for the given role.
///
/// Resolves the role to a freedesktop icon name, looks it up in the
/// user's active icon theme, and returns the SVG bytes as `IconData::Svg`.
///
/// # Fallback chain
///
/// 1. Active icon theme (with hicolor fallback handled by the crate)
/// 2. Bundled Material SVGs (requires `material-icons` feature, which
///    `system-icons` implies)
///
/// Returns `None` only if no icon is found at any level of the chain.
pub fn load_freedesktop_icon(role: IconRole) -> Option<IconData> {
    let theme = detect_theme();

    if let Some(name) = icon_name(IconSet::Freedesktop, role) {
        if let Some(path) = find_icon(name, &theme, 24) {
            if let Ok(bytes) = std::fs::read(&path) {
                return Some(IconData::Svg(bytes));
            }
        }
    }

    // Bundled Material SVG fallback
    bundled_icon_svg(IconSet::Material, role).map(|bytes| IconData::Svg(bytes.to_vec()))
}

#[cfg(test)]
#[cfg(feature = "system-icons")]
mod tests {
    use super::*;

    #[test]
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
    fn load_icon_notification_uses_bundled_fallback() {
        // Notification has no freedesktop name, so goes straight to bundled Material
        let result = load_freedesktop_icon(IconRole::Notification);
        assert!(
            result.is_some(),
            "Notification should fall back to bundled Material icon"
        );
        match result.unwrap() {
            IconData::Svg(bytes) => {
                let content = std::str::from_utf8(&bytes).expect("SVG should be valid UTF-8");
                assert!(
                    content.contains("<svg"),
                    "Bundled fallback should contain <svg tag"
                );
            }
            _ => panic!("Expected SVG data"),
        }
    }

    #[test]
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
}
