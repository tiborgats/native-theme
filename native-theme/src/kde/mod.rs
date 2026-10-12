// KDE theme reader -- reads kdeglobals INI file and maps to Theme

/// KDE color group parsing and mapping.
pub(crate) mod colors;
/// Qt font string parsing with weight extraction.
pub(crate) mod fonts;

use crate::Rgba;
use crate::model::IconSizes;

/// Parse KDE kdeglobals content into a Theme without any I/O.
///
/// This function is intentionally `pub` (not `pub(crate)`) because integration tests
/// in `tests/reader_kde.rs` import it from outside the crate. See doc 1 section C6
/// and Phase 79-02 decision for rationale.
///
/// `font_dpi`: if `Some`, used directly for font DPI; if `None`, attempts to
/// extract `forceFontDPI` from the INI content as an X11 session reads it (the
/// session type is not checked), falling back to `None` (no DPI set).
/// Icon sizes are NOT populated (requires filesystem access).
#[doc(hidden)]
pub fn from_kde_content_pure(
    content: &str,
    font_dpi: Option<f32>,
) -> crate::Result<(crate::Theme, Option<f32>, crate::AccessibilityPreferences)> {
    let mut ini = create_kde_parser();
    read_kconfig(&mut ini, content);
    let (mode, is_dark, name, accessibility) = read_kde_ini(&ini);
    let output = crate::ReaderOutput::Single {
        mode: Box::new(mode),
        is_dark,
    };
    let theme = output.to_theme(&name, None, &crate::LayoutTheme::default());
    // Font DPI: use provided value, or try extracting forceFontDPI from INI content
    Ok((
        theme,
        font_dpi.or_else(|| parse_force_font_dpi(&ini)),
        accessibility,
    ))
}

/// The variant, its darkness, the scheme name and the accessibility flags
/// kdeglobals states, without any I/O: per-widget colors, fonts with Qt5/Qt6
/// weight conversion, the icon theme and Breeze widget sizing constants.
fn read_kde_ini(
    ini: &configparser::ini::Ini,
) -> (
    crate::ThemeMode,
    bool,
    String,
    crate::AccessibilityPreferences,
) {
    let mut variant = crate::ThemeMode::default();

    // Populate colors, fonts, and widget sizing on the variant
    colors::populate_colors(ini, &mut variant);
    fonts::populate_fonts(ini, &mut variant);
    crate::kde_metrics::populate_widget_sizing(&mut variant);

    // KDE-05: Icon theme name from [Icons] Theme (per-variant)
    variant.defaults.icon_theme = ini
        .get("Icons", "Theme")
        .filter(|s| !s.is_empty())
        .map(std::borrow::Cow::Owned);

    // KDE-06: Accessibility flags (pure -- no I/O)
    // AnimationDurationFactor from [KDE]
    let accessibility = crate::AccessibilityPreferences {
        reduce_motion: ini
            .get("KDE", "AnimationDurationFactor")
            .and_then(|value| value.trim().parse::<f32>().ok())
            == Some(0.0),
        ..Default::default()
    };

    let name = ini
        .get("General", "ColorScheme")
        .unwrap_or_else(|| "KDE".to_string());

    (variant, is_dark_theme(ini), name, accessibility)
}

/// The KDE reader's result for the kdeglobals in `ini`: [`read_kde_ini`],
/// with the font DPI and the icon theme's sizes read from the system.
fn from_kde_ini(ini: &configparser::ini::Ini) -> crate::ReaderResult {
    let (mut mode, is_dark, name, accessibility) = read_kde_ini(ini);

    // I/O: icon sizes from filesystem (icon_theme is on variant defaults)
    if let Some(ref theme_name) = mode.defaults.icon_theme {
        mode.defaults.icon_sizes = parse_icon_sizes_from_index_theme(theme_name);
    }

    crate::ReaderResult {
        output: crate::ReaderOutput::Single {
            mode: Box::new(mode),
            is_dark,
        },
        name: std::borrow::Cow::Owned(name),
        icon_set: None,
        layout: crate::LayoutTheme::default(),
        // I/O: forceFontDPI (X11), then the cached chain (kcmfontsrc -> xrdb -> xrandr -> 96.0)
        font_dpi: Some(reader_font_dpi(
            ini,
            crate::detect::session_type().as_deref(),
            crate::detect::system_font_dpi,
        )),
        accessibility,
    }
}

/// [`from_kde_ini`] over one kdeglobals file's `content`.
#[cfg(test)]
pub(crate) fn from_kde_content(content: &str) -> crate::Result<crate::ReaderResult> {
    let mut ini = create_kde_parser();
    read_kconfig(&mut ini, content);
    Ok(from_kde_ini(&ini))
}

/// Extract forceFontDPI from the parsed INI content
/// ([`crate::detect::parse_force_font_dpi`]), as an X11 session reads it.
/// This is the pure (no I/O) portion of DPI detection.
fn parse_force_font_dpi(ini: &configparser::ini::Ini) -> Option<f32> {
    crate::detect::parse_force_font_dpi(&ini.get("General", "forceFontDPI")?)
}

/// The KDE reader's font DPI: on a session that reads it
/// ([`crate::detect::reads_force_font_dpi`]) the `forceFontDPI` of the
/// kdeglobals in `ini`, else `chain`, the system's cached DPI chain
/// ([`crate::detect::system_font_dpi`]: `forceFontDPI` on X11 from the
/// kdeglobals and kcmfontsrc files, `Xft.dpi`, xrandr, 96.0).
fn reader_font_dpi(
    ini: &configparser::ini::Ini,
    session_type: Option<&str>,
    chain: impl FnOnce() -> f32,
) -> f32 {
    crate::detect::reads_force_font_dpi(session_type)
        .then(|| parse_force_font_dpi(ini))
        .flatten()
        .unwrap_or_else(chain)
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
        .min_by_key(|sz| sz.abs_diff(22))
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
/// Reads kdeglobals as KConfig cascades it ([`read_kdeglobals`]; no file at
/// all reads as an empty one) and maps KDE color groups and font strings to
/// a [`ReaderResult`](crate::ReaderResult).
///
/// Internal reader dispatched through the [`crate::reader::ThemeReader`] trait
/// by [`pipeline::select_reader`]; external consumers should use
/// [`SystemTheme::from_system()`](crate::SystemTheme::from_system).
pub(crate) struct KdeReader;

#[async_trait::async_trait]
impl crate::reader::ThemeReader for KdeReader {
    async fn read(&self) -> crate::Result<crate::ReaderResult> {
        Ok(from_kde_ini(
            &read_kdeglobals().unwrap_or_else(create_kde_parser),
        ))
    }
}

/// kdeglobals as KConfig reads it: the files of [`kdeglobals_files`] layered
/// in order ([`read_kconfig_files`]); `None` when none of them can be read.
pub(crate) fn read_kdeglobals() -> Option<configparser::ini::Ini> {
    read_kconfig_files(&kdeglobals_files())
}

/// Whether kdeglobals ([`read_kdeglobals`]) states a dark theme
/// ([`is_dark_theme`]); `None` when no kdeglobals file can be read.
pub(crate) fn kdeglobals_is_dark() -> Option<bool> {
    read_kdeglobals().map(|ini| is_dark_theme(&ini))
}

/// The kdeglobals files KConfig reads, lowest priority first
/// ([`kdeglobals_files_pure`] for `$XDG_CONFIG_DIRS` and [`kdeglobals_path`]).
fn kdeglobals_files() -> Vec<std::path::PathBuf> {
    kdeglobals_files_pure(
        std::env::var("XDG_CONFIG_DIRS").ok().as_deref(),
        kdeglobals_path(),
    )
}

/// `kdeglobals` in each of `config_dirs` (`$XDG_CONFIG_DIRS`), last first,
/// then the user's `user_file`.
///
/// KConfig reads every `kdeglobals` `QStandardPaths::locateAll` finds, in
/// reverse, so the user's file comes last and wins (kconfig
/// `src/core/kconfig.cpp:740`, `:761-766` at a4e32e9d). The config dirs are
/// `$XDG_CONFIG_DIRS`'s absolute entries without duplicates, `/etc/xdg` when
/// none is left (qtbase `src/corelib/io/qstandardpaths_unix.cpp:370-389`,
/// `:403-413` at 512d45ad). Plasma puts `kdedefaults` beside the user's file
/// first in `$XDG_CONFIG_DIRS` (plasma-workspace
/// `startkde/startplasma.cpp:419-426` at 8bd4f86a); it is added there when
/// `config_dirs` lacks it, for a process started outside Plasma's session.
fn kdeglobals_files_pure(
    config_dirs: Option<&str>,
    user_file: std::path::PathBuf,
) -> Vec<std::path::PathBuf> {
    let mut dirs: Vec<std::path::PathBuf> = Vec::new();
    for dir in config_dirs
        .unwrap_or_default()
        .split(':')
        .filter(|dir| dir.starts_with('/'))
    {
        let dir = std::path::PathBuf::from(dir);
        if !dirs.contains(&dir) {
            dirs.push(dir);
        }
    }
    if dirs.is_empty() {
        dirs.push(std::path::PathBuf::from("/etc/xdg"));
    }
    if let Some(defaults) = user_file.parent().map(|dir| dir.join("kdedefaults"))
        && !dirs.contains(&defaults)
    {
        dirs.insert(0, defaults);
    }
    let mut files: Vec<std::path::PathBuf> = dirs
        .iter()
        .rev()
        .map(|dir| dir.join("kdeglobals"))
        .collect();
    if !files.contains(&user_file) {
        files.push(user_file);
    }
    files
}

/// The KConfig files `files` layered in order, later files overriding
/// earlier keys ([`read_kconfig`]). A file that cannot be read is skipped,
/// and its bytes are decoded lossily; `None` when no file can be read.
fn read_kconfig_files(files: &[std::path::PathBuf]) -> Option<configparser::ini::Ini> {
    let mut ini = create_kde_parser();
    let mut any = false;
    for bytes in files.iter().filter_map(|path| std::fs::read(path).ok()) {
        read_kconfig(&mut ini, &String::from_utf8_lossy(&bytes));
        any = true;
    }
    any.then_some(ini)
}

/// Parse KConfig `content` into `ini`, over the keys it already holds.
///
/// The lines KConfig skips are skipped (kconfig `src/core/kconfigini.cpp` at
/// a4e32e9d): a group header without `]` (`:100-112`, the group before it
/// stays current) and an entry with an empty key (`:168-171`). They are the
/// only lines configparser rejects, so one stray line no longer loses the
/// whole file.
fn read_kconfig(ini: &mut configparser::ini::Ini, content: &str) {
    let valid: String = content
        .lines()
        .filter(|line| {
            let line = line.trim();
            !(line.starts_with('[') && !line.contains(']')) && !line.starts_with('=')
        })
        .flat_map(|line| [line, "\n"])
        .collect();
    // Cannot fail: the filter above drops every line configparser rejects.
    let _ = ini.read_and_append(valid);
}

/// Create a configparser Ini instance configured for KDE files.
///
/// Uses case-sensitive mode and equals-only delimiter to correctly
/// handle KDE's PascalCase keys and colon-containing section names.
/// Only a line starting with `#` is a comment, as in KConfig (kconfig
/// `src/core/kconfigini.cpp:95-98` at a4e32e9d): a `#` or `;` inside a value,
/// as in `ColorScheme=My#Scheme`, is part of it.
pub(crate) fn create_kde_parser() -> configparser::ini::Ini {
    let tmp = configparser::ini::Ini::new_cs();
    let mut defaults = tmp.defaults();
    defaults.delimiters = vec!['='];
    defaults.comment_symbols = vec!['#'];
    defaults.enable_inline_comments = false;
    configparser::ini::Ini::new_from_defaults(defaults)
}

/// Parse a KDE "R,G,B" color string into an Rgba (opaque).
///
/// Returns None for malformed values (never panics).
/// Exactly 3 comma-separated u8 components required.
pub(crate) fn parse_rgb(value: &str) -> Option<Rgba> {
    let parts: Vec<&str> = value.split(',').collect();
    let [r_s, g_s, b_s] = parts.as_slice() else {
        return None;
    };
    let r = r_s.trim().parse::<u8>().ok()?;
    let g = g_s.trim().parse::<u8>().ok()?;
    let b = b_s.trim().parse::<u8>().ok()?;
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
    if let Some(config_home) = xdg_config_home
        && !config_home.is_empty()
    {
        return std::path::PathBuf::from(config_home).join("kdeglobals");
    }
    if let Some(home) = home {
        return std::path::PathBuf::from(home)
            .join(".config")
            .join("kdeglobals");
    }
    // Last resort fallback
    std::path::PathBuf::from("/etc/xdg/kdeglobals")
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

    /// Helper: extract ThemeMode from a ReaderResult for test assertions.
    fn reader_mode(result: &crate::ReaderResult) -> &crate::ThemeMode {
        match &result.output {
            crate::ReaderOutput::Single { mode, .. } => mode,
            crate::ReaderOutput::Dual { light, .. } => light,
        }
    }

    #[test]
    fn test_dark_theme_detection() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        assert!(
            matches!(
                result.output,
                crate::ReaderOutput::Single { is_dark: true, .. }
            ),
            "dark variant should be populated"
        );
    }

    #[test]
    fn test_light_theme_detection() {
        let result = from_kde_content(BREEZE_LIGHT_FULL).unwrap();
        assert!(
            matches!(
                result.output,
                crate::ReaderOutput::Single { is_dark: false, .. }
            ),
            "light variant should be populated"
        );
    }

    #[test]
    fn test_theme_name_from_colorscheme() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        assert_eq!(result.name, "BreezeDark");
    }

    #[test]
    fn test_theme_name_fallback() {
        let content = "[Colors:Window]\nBackgroundNormal=49,54,59\n";
        let result = from_kde_content(content).unwrap();
        assert_eq!(result.name, "KDE");
    }

    #[test]
    fn test_colors_populated() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let variant = reader_mode(&result);
        assert!(variant.defaults.accent_color.is_some());
        assert!(variant.defaults.background_color.is_some());
        assert!(variant.defaults.text_color.is_some());
    }

    #[test]
    fn test_fonts_populated() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let variant = reader_mode(&result);
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
        let r = result.unwrap();
        assert!(matches!(
            r.output,
            crate::ReaderOutput::Single { is_dark: true, .. }
        ));
    }

    #[test]
    fn test_from_kde_content_populates_widget_sizing() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let variant = reader_mode(&result);
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
        let r = result.unwrap();
        assert_eq!(r.name, "KDE");
    }

    // === KConfig parsing ===

    /// KConfig skips a group header without `]` and keeps the group before
    /// it; the rest of the file is still read.
    #[test]
    fn a_stray_group_line_is_skipped() -> crate::Result<()> {
        let result = from_kde_content(
            "[Colors:Window\nBackgroundNormal=49,54,59\n\n[General]\nColorScheme=BreezeDark\n",
        )?;
        assert_eq!(result.name, "BreezeDark");
        Ok(())
    }

    /// KConfig skips an entry with an empty key.
    #[test]
    fn a_line_with_an_empty_key_is_skipped() -> crate::Result<()> {
        let result = from_kde_content("[General]\n=stray\nColorScheme=BreezeDark\n")?;
        assert_eq!(result.name, "BreezeDark");
        Ok(())
    }

    /// KConfig has no inline comments: a `#` in a value is part of it.
    #[test]
    fn a_hash_in_a_value_is_kept() -> crate::Result<()> {
        let result = from_kde_content("[General]\nColorScheme=My#Scheme\n")?;
        assert_eq!(result.name, "My#Scheme");
        Ok(())
    }

    // === KConfig cascade ===

    /// A fresh directory for one test's kdeglobals files.
    fn temp_config_dir(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "native-theme-kdeglobals-{}-{tag}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("kdedefaults")).unwrap();
        dir
    }

    fn paths(paths: &[&str]) -> Vec<std::path::PathBuf> {
        paths.iter().map(std::path::PathBuf::from).collect()
    }

    #[test]
    fn kdeglobals_files_follow_plasmas_config_dirs() {
        let files = kdeglobals_files_pure(
            Some("/home/u/.config/kdedefaults:/etc/xdg"),
            "/home/u/.config/kdeglobals".into(),
        );
        assert_eq!(
            files,
            paths(&[
                "/etc/xdg/kdeglobals",
                "/home/u/.config/kdedefaults/kdeglobals",
                "/home/u/.config/kdeglobals",
            ])
        );
    }

    /// Without `$XDG_CONFIG_DIRS`, or with only relative entries, the config
    /// dirs are `/etc/xdg`, and `kdedefaults` is read as Plasma adds it.
    #[test]
    fn kdeglobals_files_default_to_etc_xdg_and_kdedefaults() {
        let expected = paths(&[
            "/etc/xdg/kdeglobals",
            "/home/u/.config/kdedefaults/kdeglobals",
            "/home/u/.config/kdeglobals",
        ]);
        for config_dirs in [None, Some(""), Some("relative:")] {
            let files = kdeglobals_files_pure(config_dirs, "/home/u/.config/kdeglobals".into());
            assert_eq!(files, expected, "{config_dirs:?}");
        }
    }

    #[test]
    fn kdeglobals_files_read_the_config_dirs_last_first_once_each() {
        let files = kdeglobals_files_pure(
            Some("/opt/a:/opt/b:/opt/a"),
            "/home/u/.config/kdeglobals".into(),
        );
        assert_eq!(
            files,
            paths(&[
                "/opt/b/kdeglobals",
                "/opt/a/kdeglobals",
                "/home/u/.config/kdedefaults/kdeglobals",
                "/home/u/.config/kdeglobals",
            ])
        );
    }

    /// A ColorScheme set only in `kdedefaults/kdeglobals`, as Plasma 6 writes
    /// it, names the theme; each file overrides the keys of the ones before.
    #[test]
    fn the_scheme_name_and_icon_theme_cascade_from_kdedefaults() {
        let dir = temp_config_dir("cascade");
        std::fs::create_dir_all(dir.join("xdg")).unwrap();
        std::fs::write(
            dir.join("xdg/kdeglobals"),
            "[General]\nColorScheme=SystemScheme\n\n[KDE]\nAnimationDurationFactor=0\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("kdedefaults/kdeglobals"),
            "[General]\nColorScheme=BreezeDark\n\n[Icons]\nTheme=breeze-dark\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("kdeglobals"),
            "[Colors:Window]\nBackgroundNormal=49,54,59\n",
        )
        .unwrap();
        let files = kdeglobals_files_pure(
            Some(dir.join("xdg").to_str().unwrap()),
            dir.join("kdeglobals"),
        );
        let result = from_kde_ini(&read_kconfig_files(&files).expect("the files are readable"));
        assert_eq!(result.name, "BreezeDark");
        assert!(result.accessibility.reduce_motion);
        assert!(matches!(
            result.output,
            crate::ReaderOutput::Single { is_dark: true, .. }
        ));
        assert_eq!(
            reader_mode(&result).defaults.icon_theme.as_deref(),
            Some("breeze-dark")
        );
    }

    /// A missing file, here the user's, reads as empty; with no file at all
    /// there is nothing.
    #[test]
    fn a_missing_kdeglobals_file_reads_as_empty() {
        let dir = temp_config_dir("missing");
        std::fs::write(
            dir.join("kdeglobals"),
            "[General]\nColorScheme=BreezeDark\n",
        )
        .unwrap();
        let ini = read_kconfig_files(&[dir.join("kdeglobals"), dir.join("absent")])
            .expect("one file is readable");
        assert_eq!(
            ini.get("General", "ColorScheme").as_deref(),
            Some("BreezeDark")
        );
        assert!(read_kconfig_files(&[dir.join("absent")]).is_none());
    }

    /// A byte that is not UTF-8 is replaced; the rest of the theme is read.
    #[test]
    fn a_non_utf8_byte_keeps_the_theme() {
        let dir = temp_config_dir("non-utf8");
        std::fs::write(
            dir.join("kdeglobals"),
            b"[General]\nColorScheme=Caf\xe9\n\n[Colors:Window]\nBackgroundNormal=49,54,59\n",
        )
        .unwrap();
        let result = from_kde_ini(
            &read_kconfig_files(&[dir.join("kdeglobals")]).expect("the file is readable"),
        );
        assert_eq!(result.name, "Caf\u{fffd}");
        assert_eq!(
            reader_mode(&result).defaults.background_color,
            Some(Rgba::rgb(49, 54, 59))
        );
    }

    // === Per-widget color tests ===

    #[test]
    fn test_button_colors_populated() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);
        assert_eq!(v.button.background_color, Some(Rgba::rgb(49, 54, 59)));
        assert_eq!(
            v.button.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    #[test]
    fn test_tooltip_colors_populated() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);
        assert_eq!(v.tooltip.background_color, Some(Rgba::rgb(49, 54, 59)));
        assert_eq!(
            v.tooltip.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    #[test]
    fn test_sidebar_colors_populated() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);
        assert_eq!(v.sidebar.background_color, Some(Rgba::rgb(42, 46, 50)));
        assert_eq!(
            v.sidebar.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    #[test]
    fn test_wm_title_bar_colors() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);
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
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);
        assert_eq!(v.list.header_background, Some(Rgba::rgb(35, 38, 41)));
        assert_eq!(
            v.list.header_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    #[test]
    fn test_link_visited() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);
        assert_eq!(v.link.visited_text_color, Some(Rgba::rgb(155, 89, 182)));
    }

    /// Link text takes `ForegroundLink` through `defaults.link_color`, so a
    /// plain read resolves it to that colour, here one `kde-breeze` does not
    /// state.
    #[test]
    fn link_text_resolves_to_foreground_link() -> crate::Result<()> {
        let content = "[Colors:Window]\nBackgroundNormal=49,54,59\n\n\
                       [Colors:View]\nForegroundLink=246,116,0\n";
        let st = crate::pipeline::run_pipeline(
            from_kde_content(content)?,
            "kde-breeze",
            crate::ColorMode::Dark,
        )?;
        assert_eq!(st.dark.link.font.color, Rgba::rgb(246, 116, 0));
        Ok(())
    }

    /// An overlay's `defaults.link_color` recolours link text on top of a
    /// KDE read: the reader states no `link.font.color` of its own.
    #[test]
    fn an_overlay_link_color_recolours_link_text() -> crate::Result<()> {
        let st = crate::pipeline::run_pipeline(
            from_kde_content(BREEZE_DARK_FULL)?,
            "kde-breeze",
            crate::ColorMode::Dark,
        )?;
        let overlay = crate::Theme::from_toml("[dark.defaults]\nlink_color = \"#ff0000\"")?;
        let customised = st.with_overlay(&overlay)?;
        assert_eq!(customised.dark.link.font.color, Rgba::rgb(255, 0, 0));
        Ok(())
    }

    // === KDE-06: Accessibility ===

    #[test]
    fn test_animation_duration_factor_zero_sets_reduce_motion_true() {
        let (_theme, _dpi, accessibility) = from_kde_content_pure(BREEZE_DARK_FULL, None).unwrap();
        assert!(accessibility.reduce_motion);
    }

    #[test]
    fn test_animation_duration_factor_nonzero_sets_reduce_motion_false() {
        let content = "\
[Colors:Window]
BackgroundNormal=49,54,59

[KDE]
AnimationDurationFactor=1.0
";
        let (_theme, _dpi, accessibility) = from_kde_content_pure(content, None).unwrap();
        assert!(!accessibility.reduce_motion);
    }

    #[test]
    fn test_force_font_dpi_sets_font_dpi() {
        let (_theme, dpi, _accessibility) = from_kde_content_pure(BREEZE_DARK_FULL, None).unwrap();
        // forceFontDPI=120 in [General] -> font_dpi=120.0 (raw DPI, not divided by 96)
        assert_eq!(dpi, Some(120.0));
    }

    fn general_ini(content: &str) -> configparser::ini::Ini {
        let mut ini = create_kde_parser();
        read_kconfig(&mut ini, &format!("[General]\n{content}\n"));
        ini
    }

    #[test]
    fn the_reader_takes_kdeglobals_force_font_dpi_on_x11() {
        let ini = general_ini("forceFontDPI=120");
        assert_eq!(reader_font_dpi(&ini, Some("x11"), || 96.0), 120.0);
    }

    #[test]
    fn the_reader_ignores_force_font_dpi_on_wayland() {
        let ini = general_ini("forceFontDPI=120");
        for session in [Some("wayland"), None] {
            assert_eq!(reader_font_dpi(&ini, session, || 96.0), 96.0, "{session:?}");
        }
    }

    #[test]
    fn a_zero_or_infinite_force_font_dpi_is_the_chains() {
        for value in ["0", "inf"] {
            let ini = general_ini(&format!("forceFontDPI={value}"));
            assert_eq!(reader_font_dpi(&ini, Some("x11"), || 96.0), 96.0, "{value}");
            let (_theme, dpi, _accessibility) =
                from_kde_content_pure(&format!("[General]\nforceFontDPI={value}\n"), None).unwrap();
            assert_eq!(dpi, None, "{value}");
        }
    }

    #[test]
    fn test_missing_accessibility_leaves_defaults() {
        let content = "\
[Colors:Window]
BackgroundNormal=49,54,59
";
        let (_theme, dpi, accessibility) = from_kde_content_pure(content, None).unwrap();
        assert!(!accessibility.reduce_motion);
        assert_eq!(accessibility.text_scaling_factor, 1.0);
        // No forceFontDPI in content -> None from pure function
        assert!(dpi.is_none());
    }

    // === KDE-05: Icon set ===

    #[test]
    fn test_icon_theme_from_icons_theme() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        // icon_set is on ReaderResult, icon_theme is on variant defaults
        assert!(result.icon_set.is_none());
        let v = reader_mode(&result);
        assert_eq!(v.defaults.icon_theme.as_deref(), Some("breeze-dark"));
    }

    #[test]
    fn test_icon_theme_none_when_missing() {
        let content = "\
[Colors:Window]
BackgroundNormal=49,54,59
";
        let result = from_kde_content(content).unwrap();
        assert!(result.icon_set.is_none());
        let v = reader_mode(&result);
        assert!(v.defaults.icon_theme.is_none());
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
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);

        if let Some(small) = v.defaults.icon_sizes.small {
            // If populated, they should be reasonable pixel values
            assert!(
                (8.0..=32.0).contains(&small),
                "small icon size should be reasonable: {small}"
            );

            if let Some(large) = v.defaults.icon_sizes.large {
                assert!(
                    (24.0..=128.0).contains(&large),
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
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);
        assert_eq!(
            v.dialog.button_order, None,
            "reader must not hardcode button_order -- resolver handles it"
        );
    }

    // === Widget sizing from Breeze metrics ===

    #[test]
    fn test_widget_sizing_checkbox_indicator() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);
        assert_eq!(v.checkbox.indicator_width, Some(20.0));
    }

    #[test]
    fn test_widget_sizing_splitter() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);
        assert_eq!(v.splitter.divider_width, Some(1.0));
    }

    // === Per-widget fonts (KDE-03) ===

    #[test]
    fn test_menu_font_from_menufont() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);
        let mf = v.menu.font.as_ref().expect("menu.font should be set");
        assert_eq!(mf.family.as_deref(), Some("Noto Sans"));
        assert_eq!(mf.size, Some(FontSize::Pt(9.0)));
    }

    #[test]
    fn test_toolbar_font_from_toolbarfont() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);
        let tf = v.toolbar.font.as_ref().expect("toolbar.font should be set");
        assert_eq!(tf.family.as_deref(), Some("Noto Sans"));
        assert_eq!(tf.size, Some(FontSize::Pt(8.0)));
    }

    #[test]
    fn test_title_bar_font_from_wm_activefont() {
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let v = reader_mode(&result);
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
        let result = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let kde_theme = result
            .output
            .to_theme(&result.name, result.icon_set, &result.layout);

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
            resolved.defaults.font.family.as_ref(),
            "Noto Sans",
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

        // icon_set is now on Theme/SystemTheme, not on ResolvedTheme

        // dialog.button_order should be from KDE reader
        assert_eq!(
            resolved.dialog.button_order,
            crate::DialogButtonOrder::PrimaryLeft,
            "dialog button order should be leading affirmative for KDE"
        );
    }

    // === Plan 94-03 (G8): KdeReader ThemeReader impl ===
    //
    // Locks the G8 contract on the KDE side: a unit struct `KdeReader` exists
    // at the module root and implements `crate::reader::ThemeReader`. Before
    // Task 2 lands, this fails to compile (`no struct KdeReader` / `unresolved
    // module crate::reader`).
    #[test]
    fn kde_reader_exists_and_implements_theme_reader() {
        // Object-safe coercion: proves the trait is consumable via dyn.
        let r: Box<dyn crate::reader::ThemeReader> = Box::new(super::KdeReader);
        // read() is async — drive it to completion on a simple pollster runtime.
        // On systems without a readable kdeglobals the result may legitimately be
        // Err(ReaderFailed); either outcome validates the trait dispatch path,
        // the strictly-tested invariant is that `r.read()` type-checks and can
        // be awaited without runtime panics.
        let _: crate::Result<crate::ReaderResult> = pollster::block_on(r.read());
    }
}
