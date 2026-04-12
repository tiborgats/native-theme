// KDE theme reader -- reads kdeglobals INI file and maps to Theme

/// KDE color group parsing and mapping.
pub mod colors;
/// Qt font string parsing with weight extraction.
pub mod fonts;
/// Breeze widget sizing constants.
pub mod metrics;

use crate::Rgba;
use crate::model::IconSizes;

/// Parse KDE kdeglobals content into a Theme without any I/O.
///
/// `font_dpi`: if `Some`, used directly for font DPI; if `None`, attempts to
/// extract `forceFontDPI` from the INI content, falling back to `None` (no DPI set).
/// Icon sizes are NOT populated (requires filesystem access) -- the caller
/// (`from_kde_content` / `from_kde`) handles that after this returns.
pub fn from_kde_content_pure(
    content: &str,
    font_dpi: Option<f32>,
) -> crate::Result<crate::Theme> {
    let mut ini = create_kde_parser();
    ini.read(content.to_string())
        .map_err(|e| crate::Error::ReaderFailed {
            reader: "kde",
            source: e.into(),
        })?;

    let mut variant = crate::ThemeMode::default();

    // Populate colors, fonts, and widget sizing on the variant
    colors::populate_colors(&ini, &mut variant);
    fonts::populate_fonts(&ini, &mut variant);
    metrics::populate_widget_sizing(&mut variant);

    // KDE-06: Accessibility flags (pure -- no I/O)
    // AnimationDurationFactor from [KDE]
    if let Some(anim_str) = ini.get("KDE", "AnimationDurationFactor")
        && let Ok(value) = anim_str.trim().parse::<f32>()
    {
        variant.defaults.reduce_motion = Some(value == 0.0);
    }

    // Font DPI: use provided value, or try extracting forceFontDPI from INI content
    variant.defaults.font_dpi = font_dpi.or_else(|| parse_force_font_dpi(&ini));

    // KDE-05: Icon theme name from [Icons] Theme (no filesystem access for sizes)
    if let Some(theme_name) = ini.get("Icons", "Theme")
        && !theme_name.is_empty()
    {
        variant.icon_theme = Some(theme_name);
    }

    let dark = is_dark_theme(&ini);

    let name = ini
        .get("General", "ColorScheme")
        .unwrap_or_else(|| "KDE".to_string());

    let theme = if dark {
        crate::Theme {
            name,
            light: None,
            dark: Some(variant),
            layout: crate::LayoutTheme::default(),
        }
    } else {
        crate::Theme {
            name,
            light: Some(variant),
            dark: None,
            layout: crate::LayoutTheme::default(),
        }
    };

    Ok(theme)
}

/// Parse a KDE kdeglobals content string into a Theme.
///
/// Builds a sparse ThemeMode with per-widget colors, fonts with Qt5/Qt6
/// weight conversion, text scale from Kirigami multipliers, accessibility
/// flags, icon set, and Breeze widget sizing constants.
///
/// Delegates to [`from_kde_content_pure`] for parsing, then performs I/O
/// for full DPI detection and icon size lookup.
pub(crate) fn from_kde_content(content: &str) -> crate::Result<crate::Theme> {
    let mut ini = create_kde_parser();
    ini.read(content.to_string())
        .map_err(|e| crate::Error::ReaderFailed {
            reader: "kde",
            source: e.into(),
        })?;

    // I/O: full DPI detection chain (forceFontDPI -> kcmfontsrc -> xrdb -> xrandr -> 96.0)
    let font_dpi = detect_font_dpi(&ini);

    let mut theme = from_kde_content_pure(content, Some(font_dpi))?;

    // I/O: icon sizes from filesystem
    let variant = if theme.dark.is_some() {
        theme.dark.as_mut()
    } else {
        theme.light.as_mut()
    };
    if let Some(variant) = variant
        && let Some(ref theme_name) = variant.icon_theme
    {
        variant.defaults.icon_sizes = parse_icon_sizes_from_index_theme(theme_name);
    }

    Ok(theme)
}

/// Extract forceFontDPI from the parsed INI content.
/// Returns `Some(dpi)` if `[General]` `forceFontDPI` exists and is a valid positive f32.
/// This is the pure (no I/O) portion of DPI detection.
fn parse_force_font_dpi(ini: &configparser::ini::Ini) -> Option<f32> {
    let dpi_str = ini.get("General", "forceFontDPI")?;
    let dpi = dpi_str.trim().parse::<f32>().ok()?;
    if dpi > 0.0 { Some(dpi) } else { None }
}

/// Detect font DPI from KDE settings.
///
/// Detection chain (first positive value wins):
/// 1. `forceFontDPI` from kdeglobals `[General]` or kcmfontsrc
/// 2. `Xft.dpi` from X resources (via `xrdb -query`)
/// 3. Physical DPI from display hardware (via `xrandr`)
/// 4. Fallback: 96.0
fn detect_font_dpi(ini: &configparser::ini::Ini) -> f32 {
    // Check forceFontDPI from kdeglobals [General], then kcmfontsrc
    let dpi_str = ini
        .get("General", "forceFontDPI")
        .or_else(|| read_kcmfontsrc_key("General", "forceFontDPI"));
    if let Some(dpi_str) = dpi_str
        && let Ok(dpi) = dpi_str.trim().parse::<f32>()
        && dpi > 0.0
    {
        return dpi;
    }

    // Fallback to Xft.dpi from X resources
    if let Some(dpi) = crate::detect::xft_dpi() {
        return dpi;
    }

    // Physical DPI from display hardware (xrandr)
    if let Some(dpi) = crate::detect::physical_dpi() {
        return dpi;
    }

    // Default: standard 96 DPI
    96.0
}

/// Read a single key from `$XDG_CONFIG_HOME/kcmfontsrc` (or `~/.config/kcmfontsrc`).
///
/// Returns `None` if the file is missing, unreadable, or the key is not found.
pub(crate) fn read_kcmfontsrc_key(section: &str, key: &str) -> Option<String> {
    let path = if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        if config_home.is_empty() {
            None
        } else {
            Some(std::path::PathBuf::from(config_home).join("kcmfontsrc"))
        }
    } else {
        None
    }
    .or_else(|| {
        std::env::var("HOME").ok().map(|h| {
            std::path::PathBuf::from(h)
                .join(".config")
                .join("kcmfontsrc")
        })
    })?;

    let content = std::fs::read_to_string(path).ok()?;
    let mut ini = create_kde_parser();
    ini.read(content).ok()?;
    ini.get(section, key)
}

/// Parse icon sizes from a freedesktop index.theme INI already loaded into a parser.
///
/// Reads the `[Icon Theme]` section's `Directories` key, then for each listed
/// directory reads `Size` and `Context`. Derives:
/// - `small`: smallest Size where Context is "Actions" or "Status"
/// - `toolbar`: Size closest to 22 from "Actions" entries
/// - `large`: smallest Size >= 32 from "Applications" entries
///
/// Fields without matching entries remain None.
pub(crate) fn parse_icon_sizes_from_content(ini: &configparser::ini::Ini) -> IconSizes {
    let dirs_str = match ini.get("Icon Theme", "Directories") {
        Some(s) if !s.trim().is_empty() => s,
        _ => return IconSizes::default(),
    };

    // Collect (size, context) pairs from each directory section
    let mut entries: Vec<(u32, String)> = Vec::new();
    for dir_name in dirs_str.split(',') {
        let dir_name = dir_name.trim();
        if dir_name.is_empty() {
            continue;
        }
        let size = ini
            .get(dir_name, "Size")
            .and_then(|s| s.trim().parse::<u32>().ok());
        let context = ini.get(dir_name, "Context");
        if let (Some(sz), Some(ctx)) = (size, context) {
            entries.push((sz, ctx));
        }
    }

    if entries.is_empty() {
        return IconSizes::default();
    }

    // small: smallest Size from Actions or Status context
    let small = entries
        .iter()
        .filter(|(_, ctx)| ctx == "Actions" || ctx == "Status")
        .map(|(sz, _)| *sz)
        .min()
        .map(|sz| sz as f32);

    // toolbar: Actions entry closest to 22
    let toolbar = entries
        .iter()
        .filter(|(_, ctx)| ctx == "Actions")
        .map(|(sz, _)| *sz)
        .min_by_key(|sz| (*sz as i32 - 22).unsigned_abs())
        .map(|sz| sz as f32);

    // large: smallest Applications entry >= 32 (or largest if none >= 32)
    let large = {
        let app_sizes: Vec<u32> = entries
            .iter()
            .filter(|(_, ctx)| ctx == "Applications")
            .map(|(sz, _)| *sz)
            .collect();
        if app_sizes.is_empty() {
            None
        } else {
            let ge32: Vec<u32> = app_sizes.iter().copied().filter(|&s| s >= 32).collect();
            if ge32.is_empty() {
                app_sizes.iter().copied().max().map(|sz| sz as f32)
            } else {
                ge32.iter().copied().min().map(|sz| sz as f32)
            }
        }
    };

    // dialog: DialogDefault from [Icon Theme] (e.g., 22)
    let dialog = ini
        .get("Icon Theme", "DialogDefault")
        .and_then(|s| s.trim().parse::<u32>().ok())
        .map(|sz| sz as f32);

    // panel: PanelDefault from [Icon Theme] (e.g., 48)
    let panel = ini
        .get("Icon Theme", "PanelDefault")
        .and_then(|s| s.trim().parse::<u32>().ok())
        .map(|sz| sz as f32);

    IconSizes {
        small,
        toolbar,
        large,
        dialog,
        panel,
    }
}

/// Parse icon sizes from the active icon theme's index.theme file.
///
/// Searches XDG icon theme directories for `{theme_name}/index.theme`:
/// 1. `$HOME/.local/share/icons/{theme_name}/index.theme`
/// 2. Each directory in `$XDG_DATA_DIRS` (default `/usr/local/share:/usr/share`)
///    appended with `/icons/{theme_name}/index.theme`
///
/// Returns `IconSizes::default()` (all None) if no index.theme is found or
/// the file cannot be parsed.
pub(crate) fn parse_icon_sizes_from_index_theme(theme_name: &str) -> IconSizes {
    let path = match find_index_theme_path(theme_name) {
        Some(p) => p,
        None => return IconSizes::default(),
    };

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return IconSizes::default(),
    };

    let mut ini = create_kde_parser();
    if ini.read(content).is_err() {
        return IconSizes::default();
    }

    parse_icon_sizes_from_content(&ini)
}

/// Locate the index.theme file for a given icon theme name.
///
/// Searches in XDG standard order: user local first, then system dirs.
fn find_index_theme_path(theme_name: &str) -> Option<std::path::PathBuf> {
    // 1. $HOME/.local/share/icons/{theme_name}/index.theme
    if let Ok(home) = std::env::var("HOME") {
        let p = std::path::PathBuf::from(home)
            .join(".local/share/icons")
            .join(theme_name)
            .join("index.theme");
        if p.exists() {
            return Some(p);
        }
    }

    // 2. XDG_DATA_DIRS entries + /icons/{theme_name}/index.theme
    let data_dirs = std::env::var("XDG_DATA_DIRS")
        .unwrap_or_else(|_| "/usr/local/share:/usr/share".to_string());
    for dir in data_dirs.split(':') {
        let dir = dir.trim();
        if dir.is_empty() {
            continue;
        }
        let p = std::path::PathBuf::from(dir)
            .join("icons")
            .join(theme_name)
            .join("index.theme");
        if p.exists() {
            return Some(p);
        }
    }

    None
}

/// Read the current KDE theme from kdeglobals.
///
/// Parses `~/.config/kdeglobals` (respecting `XDG_CONFIG_HOME`) and maps
/// KDE color groups and font strings to a `Theme`.
pub fn from_kde() -> crate::Result<crate::Theme> {
    let path = kdeglobals_path();
    let content = std::fs::read_to_string(&path).map_err(|e| crate::Error::ReaderFailed {
        reader: "kde",
        source: format!("cannot read {}: {e}", path.display()).into(),
    })?;
    from_kde_content(&content)
}

/// Create a configparser Ini instance configured for KDE files.
///
/// Uses case-sensitive mode and equals-only delimiter to correctly
/// handle KDE's PascalCase keys and colon-containing section names.
pub(crate) fn create_kde_parser() -> configparser::ini::Ini {
    let tmp = configparser::ini::Ini::new_cs();
    let mut defaults = tmp.defaults();
    defaults.delimiters = vec!['='];
    configparser::ini::Ini::new_from_defaults(defaults)
}

/// Parse a KDE "R,G,B" color string into an Rgba (opaque).
///
/// Returns None for malformed values (never panics).
/// Exactly 3 comma-separated u8 components required.
pub(crate) fn parse_rgb(value: &str) -> Option<Rgba> {
    let parts: Vec<&str> = value.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let r = parts[0].trim().parse::<u8>().ok()?;
    let g = parts[1].trim().parse::<u8>().ok()?;
    let b = parts[2].trim().parse::<u8>().ok()?;
    Some(Rgba::rgb(r, g, b))
}

/// Resolve the path to the kdeglobals file.
///
/// Checks XDG_CONFIG_HOME (non-empty), then $HOME/.config/kdeglobals,
/// then /etc/xdg/kdeglobals as last resort.
pub(crate) fn kdeglobals_path() -> std::path::PathBuf {
    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME")
        && !config_home.is_empty()
    {
        return std::path::PathBuf::from(config_home).join("kdeglobals");
    }
    if let Ok(home) = std::env::var("HOME") {
        return std::path::PathBuf::from(home)
            .join(".config")
            .join("kdeglobals");
    }
    // Last resort fallback
    std::path::PathBuf::from("/etc/xdg/kdeglobals")
}

/// Pure version of [`kdeglobals_path`] for testing.
///
/// Accepts XDG_CONFIG_HOME and HOME values as parameters instead of
/// reading environment variables, following the `_pure` suffix convention
/// (established in Phase 63).
#[cfg(test)]
fn kdeglobals_path_pure(xdg_config_home: Option<&str>, home: Option<&str>) -> std::path::PathBuf {
    if let Some(config_home) = xdg_config_home {
        if !config_home.is_empty() {
            return std::path::PathBuf::from(config_home).join("kdeglobals");
        }
    }
    if let Some(home) = home {
        return std::path::PathBuf::from(home)
            .join(".config")
            .join("kdeglobals");
    }
    // Last resort fallback
    std::path::PathBuf::from("/etc/xdg/kdeglobals")
}

/// Read KDE theme from a specific kdeglobals file path (for testing).
#[cfg(test)]
pub(crate) fn from_kde_at(path: &std::path::Path) -> crate::Result<crate::Theme> {
    let content = std::fs::read_to_string(path).map_err(|e| crate::Error::ReaderFailed {
        reader: "kde",
        source: Box::new(e),
    })?;
    from_kde_content(&content)
}

/// Detect whether the active KDE theme is dark based on background luminance.
///
/// Uses BT.601 luminance coefficients on Colors:Window/BackgroundNormal.
/// Defaults to false (light) if the section/key is missing.
pub(crate) fn is_dark_theme(ini: &configparser::ini::Ini) -> bool {
    if let Some(bg_str) = ini.get("Colors:Window", "BackgroundNormal")
        && let Some(bg) = parse_rgb(&bg_str)
    {
        let luma = 0.299 * (bg.r as f32) + 0.587 * (bg.g as f32) + 0.114 * (bg.b as f32);
        return luma < 128.0;
    }
    false
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::Rgba;
    use crate::model::font::FontSize;

    // === parse_rgb tests ===

    #[test]
    fn parse_rgb_valid_breeze_accent() {
        assert_eq!(parse_rgb("61,174,233"), Some(Rgba::rgb(61, 174, 233)));
    }

    #[test]
    fn parse_rgb_handles_whitespace() {
        assert_eq!(parse_rgb("0, 0, 0"), Some(Rgba::rgb(0, 0, 0)));
    }

    #[test]
    fn parse_rgb_white() {
        assert_eq!(parse_rgb("255,255,255"), Some(Rgba::rgb(255, 255, 255)));
    }

    #[test]
    fn parse_rgb_invalid_text() {
        assert_eq!(parse_rgb("invalid"), None);
    }

    #[test]
    fn parse_rgb_too_few_components() {
        assert_eq!(parse_rgb("1,2"), None);
    }

    #[test]
    fn parse_rgb_too_many_components() {
        assert_eq!(parse_rgb("1,2,3,4"), None);
    }

    #[test]
    fn parse_rgb_out_of_u8_range() {
        assert_eq!(parse_rgb("256,0,0"), None);
    }

    #[test]
    fn parse_rgb_empty_string() {
        assert_eq!(parse_rgb(""), None);
    }

    // === kdeglobals_path_pure tests ===

    #[test]
    fn kdeglobals_path_respects_xdg_config_home() {
        let path = kdeglobals_path_pure(Some("/tmp/test"), Some("/home/user"));
        assert_eq!(path, std::path::PathBuf::from("/tmp/test/kdeglobals"));
    }

    #[test]
    fn kdeglobals_path_falls_back_to_home_config() {
        let path = kdeglobals_path_pure(None, Some("/home/testuser"));
        assert_eq!(
            path,
            std::path::PathBuf::from("/home/testuser")
                .join(".config")
                .join("kdeglobals")
        );
    }

    #[test]
    fn kdeglobals_path_fallback_to_etc_xdg() {
        let path = kdeglobals_path_pure(None, None);
        assert_eq!(path, std::path::PathBuf::from("/etc/xdg/kdeglobals"));
    }

    // === is_dark_theme tests ===

    #[test]
    fn is_dark_theme_detects_breeze_dark() {
        let mut ini = create_kde_parser();
        ini.read("[Colors:Window]\nBackgroundNormal=20,22,24\n".to_string())
            .unwrap();
        assert!(is_dark_theme(&ini));
    }

    #[test]
    fn is_dark_theme_detects_breeze_light() {
        let mut ini = create_kde_parser();
        ini.read("[Colors:Window]\nBackgroundNormal=239,240,241\n".to_string())
            .unwrap();
        assert!(!is_dark_theme(&ini));
    }

    #[test]
    fn is_dark_theme_defaults_false_when_missing() {
        let ini = create_kde_parser();
        assert!(!is_dark_theme(&ini));
    }

    // === create_kde_parser tests ===

    #[test]
    fn create_kde_parser_is_case_sensitive() {
        let mut ini = create_kde_parser();
        ini.read("[Colors:View]\nBackgroundNormal=255,255,255\n".to_string())
            .unwrap();
        assert!(ini.get("Colors:View", "BackgroundNormal").is_some());
    }

    #[test]
    fn create_kde_parser_preserves_section_colons() {
        let mut ini = create_kde_parser();
        ini.read("[Colors:Window]\nForegroundNormal=0,0,0\n".to_string())
            .unwrap();
        assert!(ini.get("Colors:Window", "ForegroundNormal").is_some());
    }

    #[test]
    fn create_kde_parser_equals_only_delimiter() {
        let mut ini = create_kde_parser();
        ini.read("[Test]\nsome:value=actual\n".to_string()).unwrap();
        assert_eq!(ini.get("Test", "some:value"), Some("actual".to_string()));
    }

    // === from_kde_content / from_kde integration tests ===

    /// Full Breeze Dark kdeglobals fixture with all sections including
    /// KDE-01 through KDE-06 fields.
    const BREEZE_DARK_FULL: &str = "\
[General]
ColorScheme=BreezeDark
font=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
fixed=Hack,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
menuFont=Noto Sans,9,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
toolBarFont=Noto Sans,8,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
smallestReadableFont=Noto Sans,7,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
forceFontDPI=120

[KDE]
AnimationDurationFactor=0

[Icons]
Theme=breeze-dark

[Colors:View]
BackgroundNormal=35,38,41
BackgroundAlternate=30,33,36
ForegroundNormal=252,252,252
ForegroundInactive=161,169,177
ForegroundActive=61,174,233
ForegroundLink=29,153,243
ForegroundNegative=218,68,83
ForegroundNeutral=246,116,0
ForegroundPositive=39,174,96
ForegroundVisited=155,89,182
DecorationFocus=61,174,233
DecorationHover=29,153,243

[Colors:Window]
BackgroundNormal=49,54,59
BackgroundAlternate=44,49,54
ForegroundNormal=239,240,241
ForegroundInactive=161,169,177
ForegroundActive=61,174,233
ForegroundLink=29,153,243
ForegroundNegative=218,68,83
ForegroundNeutral=246,116,0
ForegroundPositive=39,174,96
DecorationFocus=61,174,233
DecorationHover=29,153,243

[Colors:Button]
BackgroundNormal=49,54,59
BackgroundAlternate=44,49,54
ForegroundNormal=239,240,241
ForegroundInactive=161,169,177

[Colors:Selection]
BackgroundNormal=61,174,233
BackgroundAlternate=29,153,243
ForegroundNormal=252,252,252
ForegroundInactive=161,169,177

[Colors:Tooltip]
BackgroundNormal=49,54,59
ForegroundNormal=252,252,252

[Colors:Complementary]
BackgroundNormal=42,46,50
ForegroundNormal=239,240,241

[Colors:Header]
BackgroundNormal=35,38,41
ForegroundNormal=252,252,252

[WM]
activeBackground=49,54,59
activeForeground=239,240,241
inactiveBackground=42,46,50
inactiveForeground=161,169,177
activeFont=Noto Sans,10,-1,5,75,0,0,0,0,0
";

    /// Light theme fixture with light background colors.
    const BREEZE_LIGHT_FULL: &str = "\
[General]
ColorScheme=BreezeLight

[Colors:View]
BackgroundNormal=255,255,255
ForegroundNormal=35,38,41
DecorationFocus=61,174,233

[Colors:Window]
BackgroundNormal=239,240,241
ForegroundNormal=35,38,41
ForegroundInactive=127,140,141
DecorationFocus=61,174,233

[Colors:Button]
BackgroundNormal=239,240,241
ForegroundNormal=35,38,41

[Colors:Selection]
BackgroundNormal=61,174,233
ForegroundNormal=255,255,255

[Colors:Tooltip]
BackgroundNormal=247,247,247
ForegroundNormal=35,38,41
";

    /// Minimal fixture -- only Colors:Window section.
    const MINIMAL_FIXTURE: &str = "\
[Colors:Window]
BackgroundNormal=49,54,59
";

    #[test]
    fn test_dark_theme_detection() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        assert!(theme.dark.is_some(), "dark variant should be populated");
        assert!(
            theme.light.is_none(),
            "light variant should be None for dark theme"
        );
    }

    #[test]
    fn test_light_theme_detection() {
        let theme = from_kde_content(BREEZE_LIGHT_FULL).unwrap();
        assert!(theme.light.is_some(), "light variant should be populated");
        assert!(
            theme.dark.is_none(),
            "dark variant should be None for light theme"
        );
    }

    #[test]
    fn test_theme_name_from_colorscheme() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        assert_eq!(theme.name, "BreezeDark");
    }

    #[test]
    fn test_theme_name_fallback() {
        let content = "[Colors:Window]\nBackgroundNormal=49,54,59\n";
        let theme = from_kde_content(content).unwrap();
        assert_eq!(theme.name, "KDE");
    }

    #[test]
    fn test_colors_populated() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let variant = theme.dark.as_ref().unwrap();
        assert!(variant.defaults.accent_color.is_some());
        assert!(variant.defaults.background_color.is_some());
        assert!(variant.defaults.text_color.is_some());
    }

    #[test]
    fn test_fonts_populated() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let variant = theme.dark.as_ref().unwrap();
        assert_eq!(variant.defaults.font.family.as_deref(), Some("Noto Sans"));
        assert_eq!(variant.defaults.font.size, Some(FontSize::Pt(10.0)));
        assert_eq!(variant.defaults.mono_font.family.as_deref(), Some("Hack"));
        assert_eq!(variant.defaults.mono_font.size, Some(FontSize::Pt(10.0)));
    }

    #[test]
    fn test_minimal_fixture_no_panic() {
        let result = from_kde_content(MINIMAL_FIXTURE);
        assert!(
            result.is_ok(),
            "minimal fixture should not panic: {result:?}"
        );
        let theme = result.unwrap();
        assert!(theme.dark.is_some());
    }

    #[test]
    fn test_from_kde_content_populates_widget_sizing() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let variant = theme.dark.as_ref().unwrap();
        assert_eq!(variant.button.min_width, Some(80.0), "Button_MinWidth");
        assert_eq!(
            variant.scrollbar.groove_width,
            Some(21.0),
            "ScrollBar_Extend"
        );
    }

    #[test]
    fn test_empty_content() {
        let result = from_kde_content("");
        assert!(
            result.is_ok(),
            "empty content should produce Ok: {result:?}"
        );
        let theme = result.unwrap();
        assert_eq!(theme.name, "KDE");
    }

    #[test]
    fn test_missing_file() {
        // Test that from_kde_at returns ReaderFailed for a nonexistent path
        let path = std::path::PathBuf::from("/tmp/nonexistent_kde_test_dir_12345/kdeglobals");
        let result = from_kde_at(&path);
        assert!(result.is_err());
        let err = result.unwrap_err();
        let crate::Error::ReaderFailed { source, .. } = &err else {
            panic!("expected ReaderFailed, got: {err:?}");
        };
        let msg = source.to_string();
        assert!(
            msg.contains("kdeglobals") || msg.contains("No such file"),
            "unexpected error message: {msg}"
        );
    }

    // === Per-widget color tests ===

    #[test]
    fn test_button_colors_populated() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert_eq!(v.button.background_color, Some(Rgba::rgb(49, 54, 59)));
        assert_eq!(
            v.button.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    #[test]
    fn test_tooltip_colors_populated() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert_eq!(v.tooltip.background_color, Some(Rgba::rgb(49, 54, 59)));
        assert_eq!(
            v.tooltip.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    #[test]
    fn test_sidebar_colors_populated() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert_eq!(v.sidebar.background_color, Some(Rgba::rgb(42, 46, 50)));
        assert_eq!(
            v.sidebar.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    #[test]
    fn test_wm_title_bar_colors() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert_eq!(v.window.title_bar_background, Some(Rgba::rgb(49, 54, 59)));
        assert_eq!(
            v.window.title_bar_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
        assert_eq!(
            v.window.inactive_title_bar_background,
            Some(Rgba::rgb(42, 46, 50))
        );
        assert_eq!(
            v.window.inactive_title_bar_text_color,
            Some(Rgba::rgb(161, 169, 177))
        );
    }

    #[test]
    fn test_list_header_colors() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert_eq!(v.list.header_background, Some(Rgba::rgb(35, 38, 41)));
        assert_eq!(
            v.list.header_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    #[test]
    fn test_link_visited() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert_eq!(v.link.visited_text_color, Some(Rgba::rgb(155, 89, 182)));
    }

    // === KDE-06: Accessibility ===

    #[test]
    fn test_animation_duration_factor_zero_sets_reduce_motion_true() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert_eq!(v.defaults.reduce_motion, Some(true));
    }

    #[test]
    fn test_animation_duration_factor_nonzero_sets_reduce_motion_false() {
        let content = "\
[Colors:Window]
BackgroundNormal=49,54,59

[KDE]
AnimationDurationFactor=1.0
";
        let theme = from_kde_content(content).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert_eq!(v.defaults.reduce_motion, Some(false));
    }

    #[test]
    fn test_force_font_dpi_sets_font_dpi() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        // forceFontDPI=120 in [General] -> font_dpi=120.0 (raw DPI, not divided by 96)
        assert_eq!(v.defaults.font_dpi, Some(120.0));
        // forceFontDPI must NOT set text_scaling_factor (Fix 5)
        assert!(v.defaults.text_scaling_factor.is_none());
    }

    #[test]
    fn test_missing_accessibility_leaves_none() {
        let content = "\
[Colors:Window]
BackgroundNormal=49,54,59
";
        let theme = from_kde_content(content).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert!(v.defaults.reduce_motion.is_none());
        assert!(v.defaults.text_scaling_factor.is_none());
        // Exact DPI value is environment-dependent (xrdb availability);
        // we only verify the field is populated since detect_font_dpi always
        // returns a value (forceFontDPI -> Xft.dpi -> 96.0 fallback).
        assert!(v.defaults.font_dpi.is_some());
    }

    // === KDE-05: Icon set ===

    #[test]
    fn test_icon_set_from_icons_theme() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert!(v.icon_set.is_none());
        assert_eq!(v.icon_theme.as_deref(), Some("breeze-dark"));
    }

    #[test]
    fn test_icon_set_none_when_missing() {
        let content = "\
[Colors:Window]
BackgroundNormal=49,54,59
";
        let theme = from_kde_content(content).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert!(v.icon_set.is_none());
    }

    // === KDE-05: Icon sizes from index.theme ===

    /// Minimal index.theme content for testing icon size parsing.
    const BREEZE_INDEX_THEME: &str = "\
[Icon Theme]
Name=breeze-dark
Comment=Breeze Dark
Directories=actions/16,actions/22,actions/32,apps/16,apps/32,apps/48,status/16,status/22

[actions/16]
Size=16
Context=Actions
Type=Fixed

[actions/22]
Size=22
Context=Actions
Type=Fixed

[actions/32]
Size=32
Context=Actions
Type=Fixed

[apps/16]
Size=16
Context=Applications
Type=Fixed

[apps/32]
Size=32
Context=Applications
Type=Fixed

[apps/48]
Size=48
Context=Applications
Type=Fixed

[status/16]
Size=16
Context=Status
Type=Fixed

[status/22]
Size=22
Context=Status
Type=Fixed
";

    #[test]
    fn test_parse_icon_sizes_from_content() {
        let mut ini = create_kde_parser();
        ini.read(BREEZE_INDEX_THEME.to_string()).unwrap();
        let sizes = parse_icon_sizes_from_content(&ini);

        // small: smallest Size from Actions/Status context = 16
        assert_eq!(
            sizes.small,
            Some(16.0),
            "small should be 16 from actions/16 and status/16"
        );

        // toolbar: closest to 22 from Actions context = 22
        assert_eq!(
            sizes.toolbar,
            Some(22.0),
            "toolbar should be 22 from actions/22"
        );

        // large: smallest Applications size >= 32 = 32
        assert_eq!(sizes.large, Some(32.0), "large should be 32 from apps/32");

        // dialog and panel: not extracted from index.theme
        assert!(sizes.dialog.is_none(), "dialog should remain None");
        assert!(sizes.panel.is_none(), "panel should remain None");
    }

    #[test]
    fn test_parse_icon_sizes_missing_theme() {
        let sizes =
            parse_icon_sizes_from_index_theme("nonexistent_theme_that_does_not_exist_12345");
        assert!(
            sizes.small.is_none(),
            "small should be None for missing theme"
        );
        assert!(
            sizes.toolbar.is_none(),
            "toolbar should be None for missing theme"
        );
        assert!(
            sizes.large.is_none(),
            "large should be None for missing theme"
        );
        assert!(
            sizes.dialog.is_none(),
            "dialog should be None for missing theme"
        );
        assert!(
            sizes.panel.is_none(),
            "panel should be None for missing theme"
        );
    }

    #[test]
    fn test_parse_icon_sizes_empty_directories() {
        let content = "\
[Icon Theme]
Name=empty
Directories=
";
        let mut ini = create_kde_parser();
        ini.read(content.to_string()).unwrap();
        let sizes = parse_icon_sizes_from_content(&ini);
        assert!(sizes.small.is_none());
        assert!(sizes.toolbar.is_none());
        assert!(sizes.large.is_none());
    }

    #[test]
    fn test_parse_icon_sizes_no_icon_theme_section() {
        let content = "\
[General]
Name=whatever
";
        let mut ini = create_kde_parser();
        ini.read(content.to_string()).unwrap();
        let sizes = parse_icon_sizes_from_content(&ini);
        assert!(sizes.small.is_none());
        assert!(sizes.toolbar.is_none());
        assert!(sizes.large.is_none());
    }

    #[test]
    fn test_icon_sizes_populated_in_full_parse() {
        // This test checks that from_kde_content wires icon sizes from index.theme.
        // On systems with breeze-dark installed, icon_sizes will be populated.
        // On systems without it, icon_sizes will be None. Both are valid.
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();

        if v.defaults.icon_sizes.small.is_some() {
            // If populated, they should be reasonable pixel values
            let small = v.defaults.icon_sizes.small.unwrap();
            assert!(
                small >= 8.0 && small <= 32.0,
                "small icon size should be reasonable: {small}"
            );

            if let Some(large) = v.defaults.icon_sizes.large {
                assert!(
                    large >= 24.0 && large <= 128.0,
                    "large icon size should be reasonable: {large}"
                );
                assert!(large > small, "large should be bigger than small");
            }
        }
        // else: theme not installed on this system, icon_sizes remain None -- acceptable
    }

    // === Dialog button order ===

    #[test]
    fn test_dialog_button_order_not_set_by_reader() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert_eq!(
            v.dialog.button_order, None,
            "reader must not hardcode button_order -- resolver handles it"
        );
    }

    // === Widget sizing from Breeze metrics ===

    #[test]
    fn test_widget_sizing_checkbox_indicator() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert_eq!(v.checkbox.indicator_width, Some(20.0));
    }

    #[test]
    fn test_widget_sizing_splitter() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        assert_eq!(v.splitter.divider_width, Some(1.0));
    }

    // === Per-widget fonts (KDE-03) ===

    #[test]
    fn test_menu_font_from_menufont() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        let mf = v.menu.font.as_ref().expect("menu.font should be set");
        assert_eq!(mf.family.as_deref(), Some("Noto Sans"));
        assert_eq!(mf.size, Some(FontSize::Pt(9.0)));
    }

    #[test]
    fn test_toolbar_font_from_toolbarfont() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        let tf = v.toolbar.font.as_ref().expect("toolbar.font should be set");
        assert_eq!(tf.family.as_deref(), Some("Noto Sans"));
        assert_eq!(tf.size, Some(FontSize::Pt(8.0)));
    }

    #[test]
    fn test_title_bar_font_from_wm_activefont() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = theme.dark.as_ref().unwrap();
        let tbf = v
            .window
            .title_bar_font
            .as_ref()
            .expect("title_bar_font should be set");
        assert_eq!(tbf.family.as_deref(), Some("Noto Sans"));
        assert_eq!(tbf.size, Some(FontSize::Pt(10.0)));
        assert_eq!(tbf.weight, Some(700)); // Qt5 75 -> CSS 700
    }

    // === Integration test: resolve() + validate() pipeline ===

    #[test]
    fn test_kde_resolve_validate() {
        // Load the KDE Breeze preset as a base (provides geometry, spacing, icon sizes,
        // and other fields that KDE's kdeglobals doesn't carry).
        let mut base = crate::Theme::preset("kde-breeze").unwrap();
        let kde_theme = from_kde_content(BREEZE_DARK_FULL).unwrap();

        // Merge KDE reader output on top of the base preset.
        // The KDE variant is dark-only; merge will clone it into the base.
        base.merge(&kde_theme);

        // Extract the dark variant (KDE's output merged on top of default dark).
        let mut dark = base
            .dark
            .clone()
            .expect("dark variant should exist after merge");

        // Run the resolution pipeline.
        dark.resolve_all();

        // Validate should produce Ok(ResolvedTheme) with all fields filled.
        let resolved = dark.validate().unwrap_or_else(|e| {
            panic!("KDE resolve/validate pipeline failed: {e}");
        });

        // Spot-check: KDE-sourced fields should be present.
        // accent from Colors:View/DecorationFocus = 61,174,233
        assert_eq!(
            resolved.defaults.accent_color,
            crate::Rgba::rgb(61, 174, 233),
            "accent should be from KDE reader"
        );

        // font.family from [General] font
        assert_eq!(
            resolved.defaults.font.family, "Noto Sans",
            "font family should be from KDE reader"
        );

        // button.background from Colors:Button/BackgroundNormal = 49,54,59
        assert_eq!(
            resolved.button.background_color,
            crate::Rgba::rgb(49, 54, 59),
            "button bg should be from KDE reader"
        );

        // window.title_bar_background from WM/activeBackground = 49,54,59
        assert_eq!(
            resolved.window.title_bar_background,
            crate::Rgba::rgb(49, 54, 59),
            "title bar bg should be from KDE reader"
        );

        // input.caret from Colors:View/DecorationFocus = 61,174,233
        assert_eq!(
            resolved.input.caret_color,
            crate::Rgba::rgb(61, 174, 233),
            "input.caret should be from KDE reader (DecorationFocus)"
        );

        // icon_set should be freedesktop (from kde-breeze preset base)
        assert_eq!(
            resolved.icon_set,
            crate::IconSet::Freedesktop,
            "icon_set should be freedesktop for KDE"
        );

        // dialog.button_order should be from KDE reader
        assert_eq!(
            resolved.dialog.button_order,
            crate::DialogButtonOrder::PrimaryLeft,
            "dialog button order should be leading affirmative for KDE"
        );
    }
}
