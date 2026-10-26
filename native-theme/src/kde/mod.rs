//! KDE theme reader: reads the kdeglobals INI files and maps them to a Theme.
//!
//! # Sources
//!
//! The files the reader reads that the user's settings write, each a path in
//! the user's config directory (`$XDG_CONFIG_HOME`, else `~/.config`); the
//! watcher observes each of them:
//!
//! - `kdeglobals`: the colours, fonts, icon theme and accessibility flags.
//! - `kdedefaults/kdeglobals`: the defaults Plasma keeps beneath the user's
//!   file, read under it as KConfig cascades them.
//! - `kcmfonts`: `forceFontDPI`, on X11, the file the fonts KCM writes it
//!   to and krdb reads it from (`docs/platform-facts.md:717`).
//! - `org.freedesktop.appearance accent-color`: the portal's accent colour,
//!   with the `portal` feature (`crate::gnome::GnomePortalKdeReader`):
//!   xdg-desktop-portal-kde gives the application palette's highlight
//!   colour (`src/settings.cpp:279-283` at v6.7.5), which the colour scheme
//!   in kdeglobals sets.
//!
//! # Sources not watched
//!
//! - `$XDG_CONFIG_DIRS/kdeglobals` (`/etc/xdg/kdeglobals`): the system's
//!   files, which the distribution or an administrator writes, not the user's
//!   settings.
//! - `Xft.dpi` (`xrdb -query`) and `xrandr`: the rest of the font DPI
//!   chain, read from the X server, not from a settings file.
//! - The icon theme's `index.theme`: its sizes, which the icon theme's
//!   package writes.

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
/// `font_dpi` is returned as given: kdeglobals holds no font DPI
/// (`forceFontDPI` lives in kcmfonts, `docs/platform-facts.md:717`).
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
    Ok((theme, font_dpi, accessibility))
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
    variant.defaults.icon_theme = icon_theme(ini).map(std::borrow::Cow::Owned);

    // KDE-06: Accessibility flags (pure -- no I/O)
    let accessibility = crate::AccessibilityPreferences {
        reduce_motion: reduce_motion(ini),
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
        // I/O: the cached chain (kcmfonts forceFontDPI on X11 -> xrdb -> xrandr -> 96.0)
        font_dpi: Some(crate::detect::system_font_dpi()),
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

/// Read a single key from `kcmfonts` in the user's config directory
/// ([`crate::detect::xdg_config_home`]).
///
/// Returns `None` if there is no config directory, the file is missing or
/// unreadable, or the key is not found.
pub(crate) fn read_kcmfonts_key(section: &str, key: &str) -> Option<String> {
    kcmfonts_key_in(&crate::detect::xdg_config_home()?, section, key)
}

/// [`read_kcmfonts_key`] in the config directory `config_dir`, the file read
/// as KConfig reads it ([`read_kconfig`]), its bytes decoded lossily.
fn kcmfonts_key_in(config_dir: &std::path::Path, section: &str, key: &str) -> Option<String> {
    let bytes = std::fs::read(config_dir.join("kcmfonts")).ok()?;
    let mut ini = create_kde_parser();
    read_kconfig(&mut ini, &String::from_utf8_lossy(&bytes));
    ini.get(section, key)
}

/// Parse icon sizes from an icon theme's index.theme already loaded into a
/// parser: the `[Icon Theme]` group's `<Group>Default` keys, which KIconTheme
/// reads as each KIconLoader group's default size (kiconthemes
/// `src/kicontheme.cpp:468-473` at v6.31.0), mapped as
/// `docs/platform-facts.md` §2.1.8 maps the groups:
/// - `small`: `SmallDefault`
/// - `toolbar`: `MainToolbarDefault`
/// - `large`: `DesktopDefault`
/// - `dialog`: `DialogDefault`
/// - `panel`: `PanelDefault`
///
/// A key the theme omits, or one that is not a size, has KIconTheme's own
/// fallback for the group ([`KICONTHEME_DEFAULT_SIZES`]), as KConfig returns
/// the default for a missing key or one it cannot convert. That fallback is
/// KDE's, not the preset's, and can differ from it: a theme without
/// `DesktopDefault` is drawn at 32 where Breeze states 48.
pub(crate) fn parse_icon_sizes_from_content(ini: &configparser::ini::Ini) -> IconSizes {
    let [small, toolbar, large, dialog, panel] = KICONTHEME_DEFAULT_SIZES.map(|(key, fallback)| {
        Some(
            ini.get("Icon Theme", key)
                .and_then(|s| s.trim().parse::<u32>().ok())
                .map_or(fallback, |sz| sz as f32),
        )
    });
    IconSizes {
        small,
        toolbar,
        large,
        dialog,
        panel,
    }
}

/// The `[Icon Theme]` key of each size [`parse_icon_sizes_from_content`]
/// reads (`small`, `toolbar`, `large`, `dialog`, `panel`, in that order) and
/// KIconTheme's fallback for its group: `Small` 16, `MainToolbar` 22,
/// `Desktop` 32, `Dialog` 32, `Panel` 48 (kiconthemes `src/kicontheme.cpp:160-167`
/// at v6.31.0), the size it keeps when `readEntry` finds no key
/// (kconfig `src/core/kconfiggroup.cpp:690-693` at v6.31.0) or cannot
/// convert the value (:256-269).
const KICONTHEME_DEFAULT_SIZES: [(&str, f32); 5] = [
    ("SmallDefault", 16.0),
    ("MainToolbarDefault", 22.0),
    ("DesktopDefault", 32.0),
    ("DialogDefault", 32.0),
    ("PanelDefault", 48.0),
];

/// Parse icon sizes from the active icon theme's index.theme file
/// ([`parse_icon_sizes_from_content`]), found where KIconTheme finds it
/// ([`kde_icon_base_dirs`]).
///
/// Returns `IconSizes::default()` (all None) if no index.theme is found or
/// the file cannot be read.
pub(crate) fn parse_icon_sizes_from_index_theme(theme_name: &str) -> IconSizes {
    let bases = kde_icon_base_dirs(
        std::env::home_dir().as_deref(),
        std::env::var_os("XDG_DATA_HOME").as_deref(),
        std::env::var_os("XDG_DATA_DIRS").as_deref(),
    );
    icon_sizes_in_bases(theme_name, &bases)
}

/// The icon base directories KIconTheme searches for a theme's index.theme,
/// in its order: `QStandardPaths::locateAll(GenericDataLocation, "icons")`
/// (kiconthemes `src/kicontheme.cpp:382` at v6.31.0, the first holding the
/// theme's index.theme winning, :395-405), which is the data home
/// (`$XDG_DATA_HOME`, `data_home`), then each data dir (`$XDG_DATA_DIRS`,
/// `data_dirs`; qtbase `src/corelib/io/qstandardpaths_unix.cpp:452` at
/// v6.11.2), each joined with `icons`. `~/.icons`, which the freedesktop icon
/// lookup also searches ([`crate::icons::icon_base_dirs`]) and KIconTheme
/// does not, comes last. Without a home directory the data dirs are still
/// searched.
fn kde_icon_base_dirs(
    home: Option<&std::path::Path>,
    data_home: Option<&std::ffi::OsStr>,
    data_dirs: Option<&std::ffi::OsStr>,
) -> Vec<std::path::PathBuf> {
    crate::icons::xdg_data_home(data_home, home)
        .into_iter()
        .chain(crate::icons::xdg_data_dirs(data_dirs))
        .map(|dir| dir.join("icons"))
        .chain(home.map(|home| home.join(".icons")))
        .collect()
}

/// [`parse_icon_sizes_from_index_theme`] over the icon base directories
/// `bases`: the first `<base>/<theme_name>/index.theme` in their order, for a
/// `theme_name` that names a theme directory ([`crate::icons::is_theme_name`]).
fn icon_sizes_in_bases(theme_name: &str, bases: &[std::path::PathBuf]) -> IconSizes {
    if !crate::icons::is_theme_name(theme_name) {
        return IconSizes::default();
    }
    let Some(bytes) = bases
        .iter()
        .find_map(|base| std::fs::read(base.join(theme_name).join("index.theme")).ok())
    else {
        return IconSizes::default();
    };
    let mut ini = create_kde_parser();
    read_kconfig(&mut ini, &String::from_utf8_lossy(&bytes));
    parse_icon_sizes_from_content(&ini)
}

/// Read the current KDE theme from kdeglobals.
///
/// Reads kdeglobals as KConfig cascades it ([`read_kdeglobals`]; no file at
/// all reads as an empty one) and maps KDE color groups and font strings to
/// a [`ReaderResult`](crate::ReaderResult).
///
/// Internal reader dispatched through the [`crate::reader::ThemeReader`] trait
/// by [`crate::pipeline::select_reader`]; external consumers should use
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

/// Whether `ini` asks for reduced motion: `[KDE] AnimationDurationFactor`
/// is 0.
fn reduce_motion(ini: &configparser::ini::Ini) -> bool {
    ini.get("KDE", "AnimationDurationFactor")
        .and_then(|value| value.trim().parse::<f32>().ok())
        == Some(0.0)
}

/// [`reduce_motion`] of kdeglobals ([`read_kdeglobals`]), as the KDE reader
/// reads it, for [`crate::detect::detect_reduced_motion`].
pub(crate) fn kdeglobals_reduce_motion() -> bool {
    reduce_motion(&read_kdeglobals().unwrap_or_else(create_kde_parser))
}

/// The icon theme `ini` names: `[Icons] Theme`, `None` where it is absent or
/// empty.
fn icon_theme(ini: &configparser::ini::Ini) -> Option<String> {
    ini.get("Icons", "Theme").filter(|s| !s.is_empty())
}

/// The icon theme of kdeglobals ([`read_kdeglobals`]), the one the KDE
/// reader states, for `detect`'s icon-theme detection; the error says why
/// there is none.
pub(crate) fn kdeglobals_icon_theme() -> Result<String, String> {
    kconfig_files_icon_theme(&kdeglobals_files())
}

/// [`icon_theme`] of the KConfig files `files` layered
/// ([`read_kconfig_files`]); the error names the files.
fn kconfig_files_icon_theme(files: &[std::path::PathBuf]) -> Result<String, String> {
    let listed = || {
        files
            .iter()
            .map(|file| file.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let ini =
        read_kconfig_files(files).ok_or_else(|| format!("no file is readable: {}", listed()))?;
    icon_theme(&ini).ok_or_else(|| format!("`[Icons] Theme` is absent or empty in {}", listed()))
}

/// Whether kdeglobals ([`read_kdeglobals`]) states a dark theme
/// ([`window_background_is_dark`]); `None` when no kdeglobals file states a
/// window background.
pub(crate) fn kdeglobals_is_dark() -> Option<bool> {
    kconfig_files_are_dark(&kdeglobals_files())
}

/// [`window_background_is_dark`] over the KConfig files `files` layered
/// ([`read_kconfig_files`]).
fn kconfig_files_are_dark(files: &[std::path::PathBuf]) -> Option<bool> {
    read_kconfig_files(files)
        .as_ref()
        .and_then(window_background_is_dark)
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
/// then the user's `user_file`, which is `None` without a config directory.
///
/// KConfig reads every `kdeglobals` `QStandardPaths::locateAll` finds, in
/// reverse, so the user's file comes last and wins (kconfig
/// `src/core/kconfig.cpp:740`, `:761-766` at a4e32e9d). The config dirs are
/// `$XDG_CONFIG_DIRS`'s absolute entries without duplicates, `/etc/xdg` when
/// none is left (qtbase `src/corelib/io/qstandardpaths_unix.cpp:370-389`,
/// `:403-413` at 512d45ad). Plasma puts `kdedefaults` beside the user's file
/// first in `$XDG_CONFIG_DIRS` (plasma-workspace
/// `startkde/startplasma.cpp:419-426` at 8bd4f86a); it is added there when
/// `config_dirs` lacks it, for a process started outside Plasma's session;
/// without a `user_file` there is no `kdedefaults` either.
fn kdeglobals_files_pure(
    config_dirs: Option<&str>,
    user_file: Option<std::path::PathBuf>,
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
    if let Some(defaults) = user_file
        .as_deref()
        .and_then(std::path::Path::parent)
        .map(|dir| dir.join("kdedefaults"))
        && !dirs.contains(&defaults)
    {
        dirs.insert(0, defaults);
    }
    let mut files: Vec<std::path::PathBuf> = dirs
        .iter()
        .rev()
        .map(|dir| dir.join("kdeglobals"))
        .collect();
    if let Some(user_file) = user_file
        && !files.contains(&user_file)
    {
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

/// The user's kdeglobals file, in the user's config directory
/// ([`crate::detect::xdg_config_home`]); `None` without one.
pub(crate) fn kdeglobals_path() -> Option<std::path::PathBuf> {
    crate::detect::xdg_config_home().map(|dir| dir.join("kdeglobals"))
}

/// Detect whether the active KDE theme is dark based on background luminance.
///
/// [`window_background_is_dark`], false (light) if the section/key is
/// missing or does not parse.
pub(crate) fn is_dark_theme(ini: &configparser::ini::Ini) -> bool {
    window_background_is_dark(ini).unwrap_or(false)
}

/// Whether Colors:Window/BackgroundNormal is dark, by its BT.601 luminance;
/// `None` if the section/key is missing or does not parse.
fn window_background_is_dark(ini: &configparser::ini::Ini) -> Option<bool> {
    let bg = parse_rgb(&ini.get("Colors:Window", "BackgroundNormal")?)?;
    let luma = 0.299 * (bg.r as f32) + 0.587 * (bg.g as f32) + 0.114 * (bg.b as f32);
    Some(luma < 128.0)
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

    /// A fresh directory for one test's kdeglobals files, removed on drop.
    struct TempConfigDir(std::path::PathBuf);

    impl std::ops::Deref for TempConfigDir {
        type Target = std::path::Path;

        fn deref(&self) -> &std::path::Path {
            &self.0
        }
    }

    impl Drop for TempConfigDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn temp_config_dir(tag: &str) -> TempConfigDir {
        let dir = std::env::temp_dir().join(format!(
            "native-theme-kdeglobals-{}-{tag}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("kdedefaults")).unwrap();
        TempConfigDir(dir)
    }

    /// Plasma's fonts KCM and krdb open `forceFontDPI`'s file by the name
    /// `kcmfonts`, and KConfig adds no `rc` to a name it is given
    /// (docs/platform-facts.md:717).
    #[test]
    fn force_font_dpi_is_read_from_kcmfonts() {
        let dir = temp_config_dir("kcmfonts");
        std::fs::write(dir.0.join("kcmfonts"), "[General]\nforceFontDPI=144\n").unwrap();
        assert_eq!(
            kcmfonts_key_in(&dir.0, "General", "forceFontDPI").as_deref(),
            Some("144")
        );
    }

    /// `kcmfonts` is read as KConfig reads it ([`read_kconfig`]): a stray
    /// line, or a byte that is not UTF-8, loses that line, not the file.
    #[test]
    fn kcmfonts_keeps_its_keys_past_a_stray_line() {
        let dir = temp_config_dir("kcmfonts-stray");
        std::fs::write(
            dir.0.join("kcmfonts"),
            "[Stray\n[General]\nforceFontDPI=144\n",
        )
        .unwrap();
        assert_eq!(
            kcmfonts_key_in(&dir.0, "General", "forceFontDPI").as_deref(),
            Some("144")
        );
        std::fs::write(
            dir.0.join("kcmfonts"),
            b"[General]\nName=\xff\nforceFontDPI=120\n",
        )
        .unwrap();
        assert_eq!(
            kcmfonts_key_in(&dir.0, "General", "forceFontDPI").as_deref(),
            Some("120")
        );
    }

    fn paths(paths: &[&str]) -> Vec<std::path::PathBuf> {
        paths.iter().map(std::path::PathBuf::from).collect()
    }

    #[test]
    fn kdeglobals_files_follow_plasmas_config_dirs() {
        let files = kdeglobals_files_pure(
            Some("/home/u/.config/kdedefaults:/etc/xdg"),
            Some("/home/u/.config/kdeglobals".into()),
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
            let files =
                kdeglobals_files_pure(config_dirs, Some("/home/u/.config/kdeglobals".into()));
            assert_eq!(files, expected, "{config_dirs:?}");
        }
    }

    #[test]
    fn kdeglobals_files_read_the_config_dirs_last_first_once_each() {
        let files = kdeglobals_files_pure(
            Some("/opt/a:/opt/b:/opt/a"),
            Some("/home/u/.config/kdeglobals".into()),
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

    /// Without a config directory there is no user file and no
    /// `kdedefaults` beside it: only the config dirs' files.
    #[test]
    fn kdeglobals_files_without_a_config_directory_are_the_config_dirs() {
        assert_eq!(
            kdeglobals_files_pure(None, None),
            paths(&["/etc/xdg/kdeglobals"])
        );
        assert_eq!(
            kdeglobals_files_pure(Some("/opt/a:/opt/b"), None),
            paths(&["/opt/b/kdeglobals", "/opt/a/kdeglobals"])
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
            Some(dir.join("kdeglobals")),
        );
        let result = from_kde_ini(&read_kconfig_files(&files).expect("the files are readable"));
        assert_eq!(result.name, "BreezeDark");
        assert!(result.accessibility.reduce_motion);
        assert!(matches!(
            result.output,
            crate::ReaderOutput::Single { is_dark: true, .. }
        ));
        let mode = reader_mode(&result);
        assert_eq!(mode.defaults.icon_theme.as_deref(), Some("breeze-dark"));
        // The sizes of the theme the cascade found (all None where
        // breeze-dark is not installed).
        assert_eq!(
            mode.defaults.icon_sizes,
            parse_icon_sizes_from_index_theme("breeze-dark")
        );
    }

    /// kdeglobals files that state no window background say nothing about
    /// darkness, so detect's fallback goes on to the next source.
    #[test]
    fn a_kdeglobals_without_a_window_background_is_not_light() {
        let dir = temp_config_dir("colourless");
        std::fs::write(dir.join("kdeglobals"), "[General]\nColorScheme=Custom\n").unwrap();
        std::fs::write(
            dir.join("kdedefaults/kdeglobals"),
            "[Colors:Window]\nBackgroundNormal=not,a,colour\n",
        )
        .unwrap();
        let files = [dir.join("kdedefaults/kdeglobals"), dir.join("kdeglobals")];
        assert_eq!(kconfig_files_are_dark(&files), None);
        std::fs::write(
            dir.join("kdeglobals"),
            "[Colors:Window]\nBackgroundNormal=49,54,59\n",
        )
        .unwrap();
        assert_eq!(kconfig_files_are_dark(&files), Some(true));
    }

    /// The icon theme `detect` reports is the cascade's, as the reader's: a
    /// file without the key leaves the one before it; the failures name the
    /// files.
    #[test]
    fn the_detected_icon_theme_is_the_cascades() {
        let dir = temp_config_dir("icon-theme");
        let files = [dir.join("kdedefaults/kdeglobals"), dir.join("kdeglobals")];
        let failure = kconfig_files_icon_theme(&files).unwrap_err();
        assert!(failure.contains("no file is readable"), "got: {failure}");
        assert!(failure.contains("kdedefaults/kdeglobals"), "got: {failure}");

        std::fs::write(dir.join("kdeglobals"), "[General]\nColorScheme=Custom\n").unwrap();
        let failure = kconfig_files_icon_theme(&files).unwrap_err();
        assert!(
            failure.contains("`[Icons] Theme` is absent or empty"),
            "got: {failure}"
        );

        std::fs::write(
            dir.join("kdedefaults/kdeglobals"),
            "[Icons]\nTheme=breeze-dark\n",
        )
        .unwrap();
        assert_eq!(
            kconfig_files_icon_theme(&files),
            Ok("breeze-dark".to_string())
        );
        let (mode, ..) = read_kde_ini(&read_kconfig_files(&files).unwrap());
        assert_eq!(mode.defaults.icon_theme.as_deref(), Some("breeze-dark"));
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

    /// KConfig has no `;` comments: a `;` in a value is part of it, and a
    /// line starting with `;` is an entry.
    #[test]
    fn a_semicolon_is_not_a_comment() {
        let mut ini = create_kde_parser();
        read_kconfig(
            &mut ini,
            "[General]\n;ColorScheme=Hidden\nColorScheme=My;Scheme\n",
        );
        assert_eq!(
            ini.get("General", "ColorScheme").as_deref(),
            Some("My;Scheme")
        );
        assert_eq!(
            ini.get("General", ";ColorScheme").as_deref(),
            Some("Hidden")
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
        assert_eq!(v.sidebar.background_color, Some(Rgba::rgb(35, 38, 41)));
        assert_eq!(
            v.sidebar.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
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
        let st = crate::pipeline::run_pipeline(from_kde_content(content)?, "kde-breeze")?;
        assert_eq!(st.dark.link.font.color, Rgba::rgb(246, 116, 0));
        Ok(())
    }

    /// A hovered and a pressed link keep `ForegroundLink` too: `kde-breeze`
    /// states neither colour, so both inherit `link.font.color`.
    #[test]
    fn link_hover_and_active_text_follow_foreground_link() -> crate::Result<()> {
        let content = "[Colors:Window]\nBackgroundNormal=49,54,59\n\n\
                       [Colors:View]\nForegroundLink=246,116,0\n";
        let st = crate::pipeline::run_pipeline(from_kde_content(content)?, "kde-breeze")?;
        assert_eq!(st.dark.link.hover_text_color, Rgba::rgb(246, 116, 0));
        assert_eq!(st.dark.link.active_text_color, Rgba::rgb(246, 116, 0));
        Ok(())
    }

    /// An overlay's `defaults.link_color` recolours link text on top of a
    /// KDE read: the reader states no `link.font.color` of its own.
    #[test]
    fn an_overlay_link_color_recolours_link_text() -> crate::Result<()> {
        let st = crate::pipeline::run_pipeline(from_kde_content(BREEZE_DARK_FULL)?, "kde-breeze")?;
        let overlay = crate::Theme::from_toml("[dark.defaults]\nlink_color = \"#ff0000\"")?;
        let customised = st.with_overlay(&overlay)?;
        assert_eq!(customised.dark.link.font.color, Rgba::rgb(255, 0, 0));
        Ok(())
    }

    /// The colour scheme's Button and Window colours reach the combo box, the
    /// menu text and the popover over `kde-breeze`, which states Breeze's
    /// values beneath them (docs/platform-facts.md §2.24, §2.6, §2.16), and
    /// the separator is the border colour (§2.18).
    #[test]
    fn scheme_colours_reach_the_combo_box_menu_and_popover() -> crate::Result<()> {
        let content = "[Colors:Window]\nBackgroundNormal=10,20,30\nForegroundNormal=200,210,220\n\n\
                       [Colors:Button]\nBackgroundNormal=40,50,60\nForegroundNormal=230,240,250\n\n\
                       [Colors:View]\nBackgroundNormal=70,80,90\nForegroundNormal=1,2,3\n";
        let st = crate::pipeline::run_pipeline(from_kde_content(content)?, "kde-breeze")?;
        assert_eq!(st.dark.combo_box.background_color, Rgba::rgb(40, 50, 60));
        assert_eq!(st.dark.combo_box.font.color, Rgba::rgb(230, 240, 250));
        assert_eq!(st.dark.menu.font.color, Rgba::rgb(200, 210, 220));
        assert_eq!(st.dark.popover.background_color, Rgba::rgb(10, 20, 30));
        assert_eq!(st.dark.popover.font.color, Rgba::rgb(200, 210, 220));
        assert_eq!(st.dark.separator.line_color, st.dark.defaults.border.color);
        Ok(())
    }

    /// A colour scheme's background and text keys, `(BackgroundNormal,
    /// ForegroundNormal)`, of the sets the derived leaves read, the View
    /// set's `DecorationFocus` and `DecorationHover`, with an optional
    /// `[Colors:Header]` group and `[KDE] frameContrast`.
    struct Scheme {
        button: (&'static str, &'static str),
        header: Option<(&'static str, &'static str)>,
        selection: &'static str,
        view: (&'static str, &'static str),
        focus: &'static str,
        hover: &'static str,
        window: (&'static str, &'static str),
        frame_contrast: Option<&'static str>,
    }

    impl Scheme {
        fn kdeglobals(&self) -> String {
            let group = |name: &str, (bg, fg): (&str, &str)| {
                format!("[Colors:{name}]\nBackgroundNormal={bg}\nForegroundNormal={fg}\n\n")
            };
            let mut content = group("Button", self.button);
            if let Some(header) = self.header {
                content.push_str(&group("Header", header));
            }
            content.push_str(&format!(
                "[Colors:Selection]\nBackgroundNormal={}\n\n",
                self.selection
            ));
            content.push_str(&format!(
                "[Colors:View]\nBackgroundNormal={}\nForegroundNormal={}\n\
                 DecorationFocus={}\nDecorationHover={}\n\n",
                self.view.0, self.view.1, self.focus, self.hover
            ));
            content.push_str(&group("Window", self.window));
            if let Some(contrast) = self.frame_contrast {
                content.push_str(&format!("[KDE]\nframeContrast={contrast}\n"));
            }
            content
        }
    }

    /// The keys of BreezeLight.colors at breeze v6.7.5 (LGPL-2.0-or-later)
    /// the derived leaves read: Button :31, :39; Header :59, :67; Selection
    /// :87; View :115, :123, DecorationFocus :116 and DecorationHover :117;
    /// Window :129, :137. No `[KDE] frameContrast`
    /// (:193-194), so KColorScheme's 0.2.
    const BREEZE_LIGHT_SCHEME: Scheme = Scheme {
        button: ("252,252,252", "35,38,41"),
        header: Some(("222,224,226", "35,38,41")),
        selection: "61,174,233",
        view: ("255,255,255", "35,38,41"),
        focus: "61,174,233",
        hover: "61,174,233",
        window: ("239,240,241", "35,38,41"),
        frame_contrast: None,
    };

    /// The same keys of BreezeDark.colors at breeze v6.7.5
    /// (LGPL-2.0-or-later): Button :32, :40; Header :60, :68; Selection :88;
    /// View :116, :124, DecorationFocus :117 and DecorationHover :118;
    /// Window :130, :138; no frameContrast (:200-201).
    const BREEZE_DARK_SCHEME: Scheme = Scheme {
        button: ("41,44,48", "252,252,252"),
        header: Some(("41,44,48", "252,252,252")),
        selection: "61,174,233",
        view: ("20,22,24", "252,252,252"),
        focus: "61,174,233",
        hover: "61,174,233",
        window: ("32,35,38", "252,252,252"),
        frame_contrast: None,
    };

    /// The twenty-two colour leaves the reader derives from the colour scheme.
    fn derived_leaves(mode: &crate::ThemeMode) -> [(&'static str, Option<Rgba>); 22] {
        let border = |b: &Option<crate::WidgetBorderSpec>| b.as_ref().and_then(|b| b.color);
        let sw = &mode.switch;
        [
            ("button.hover_background", mode.button.hover_background),
            ("button.hover_text_color", mode.button.hover_text_color),
            ("slider.thumb_color", mode.slider.thumb_color),
            ("switch.unchecked_background", sw.unchecked_background),
            ("switch.checked_background", sw.checked_background),
            (
                "switch.hover_unchecked_background",
                sw.hover_unchecked_background,
            ),
            (
                "switch.hover_checked_background",
                sw.hover_checked_background,
            ),
            ("switch.thumb_background", sw.thumb_background),
            (
                "switch.unchecked_track_border_color",
                sw.unchecked_track_border_color,
            ),
            (
                "switch.checked_track_border_color",
                sw.checked_track_border_color,
            ),
            ("switch.thumb_border_color", sw.thumb_border_color),
            ("toolbar.background_color", mode.toolbar.background_color),
            ("toolbar.border.color", border(&mode.toolbar.border)),
            ("popover.border.color", border(&mode.popover.border)),
            ("card.border.color", border(&mode.card.border)),
            ("button.active_background", mode.button.active_background),
            ("input.hover_border_color", mode.input.hover_border_color),
            ("input.focus_border_color", mode.input.focus_border_color),
            ("tab.hover_background", mode.tab.hover_background),
            (
                "segmented_control.active_background",
                mode.segmented_control.active_background,
            ),
            (
                "scrollbar.thumb_hover_color",
                mode.scrollbar.thumb_hover_color,
            ),
            (
                "scrollbar.thumb_active_color",
                mode.scrollbar.thumb_active_color,
            ),
        ]
    }

    /// The same leaves resolved.
    fn resolved_leaves(theme: &crate::ResolvedTheme) -> [(&'static str, Option<Rgba>); 22] {
        let sw = &theme.switch;
        [
            (
                "button.hover_background",
                Some(theme.button.hover_background),
            ),
            (
                "button.hover_text_color",
                Some(theme.button.hover_text_color),
            ),
            ("slider.thumb_color", Some(theme.slider.thumb_color)),
            ("switch.unchecked_background", Some(sw.unchecked_background)),
            ("switch.checked_background", Some(sw.checked_background)),
            (
                "switch.hover_unchecked_background",
                sw.hover_unchecked_background,
            ),
            (
                "switch.hover_checked_background",
                sw.hover_checked_background,
            ),
            ("switch.thumb_background", Some(sw.thumb_background)),
            (
                "switch.unchecked_track_border_color",
                sw.unchecked_track_border_color,
            ),
            (
                "switch.checked_track_border_color",
                sw.checked_track_border_color,
            ),
            ("switch.thumb_border_color", sw.thumb_border_color),
            (
                "toolbar.background_color",
                Some(theme.toolbar.background_color),
            ),
            ("toolbar.border.color", Some(theme.toolbar.border.color)),
            ("popover.border.color", Some(theme.popover.border.color)),
            ("card.border.color", Some(theme.card.border.color)),
            ("button.active_background", theme.button.active_background),
            ("input.hover_border_color", theme.input.hover_border_color),
            ("input.focus_border_color", theme.input.focus_border_color),
            ("tab.hover_background", theme.tab.hover_background),
            (
                "segmented_control.active_background",
                Some(theme.segmented_control.active_background),
            ),
            (
                "scrollbar.thumb_hover_color",
                Some(theme.scrollbar.thumb_hover_color),
            ),
            (
                "scrollbar.thumb_active_color",
                theme.scrollbar.thumb_active_color,
            ),
        ]
    }

    /// The resolved light variant of `scheme` over `kde-breeze`.
    fn resolved_light(scheme: &Scheme) -> crate::Result<crate::ResolvedTheme> {
        let content = scheme.kdeglobals();
        let st = crate::pipeline::run_pipeline(from_kde_content(&content)?, "kde-breeze")?;
        assert_eq!(st.mode, crate::ColorMode::Light, "{content}");
        Ok(st.light)
    }

    /// The derived leaves whose resolved value differs between `scheme` and
    /// Breeze Light.
    fn moved_leaves(scheme: &Scheme) -> crate::Result<Vec<&'static str>> {
        let breeze = resolved_leaves(&resolved_light(&BREEZE_LIGHT_SCHEME)?);
        let other = resolved_leaves(&resolved_light(scheme)?);
        Ok(breeze
            .iter()
            .zip(other.iter())
            .filter(|(a, b)| a.1 != b.1)
            .map(|(a, _)| a.0)
            .collect())
    }

    /// Under Breeze Light and Breeze Dark the reader derives each leaf as
    /// the `kde-breeze` preset states it.
    #[test]
    fn the_derived_colours_are_the_presets_under_breeze() -> crate::Result<()> {
        let preset = crate::Theme::preset("kde-breeze")?;
        let variants = [
            (&BREEZE_LIGHT_SCHEME, preset.light.as_ref()),
            (&BREEZE_DARK_SCHEME, preset.dark.as_ref()),
        ];
        for (scheme, stated) in variants {
            let stated = derived_leaves(stated.expect("kde-breeze has both variants"));
            let result = from_kde_content(&scheme.kdeglobals())?;
            let read = derived_leaves(reader_mode(&result));
            for ((leaf, read), (_, stated)) in read.iter().zip(stated.iter()) {
                assert!(stated.is_some(), "kde-breeze states {leaf}");
                assert_eq!(read, stated, "{leaf} under {}", scheme.window.0);
            }
        }
        Ok(())
    }

    /// Another accent moves the switch's on track, hovered or not, and its
    /// outline, the pressed button and the selected segment; the checkbox and
    /// the progress bar follow it as before.
    #[test]
    fn an_accent_moves_the_on_track_the_pressed_button_and_the_selected_segment()
    -> crate::Result<()> {
        let scheme = Scheme {
            selection: "233,84,32",
            ..BREEZE_LIGHT_SCHEME
        };
        assert_eq!(
            moved_leaves(&scheme)?,
            [
                "switch.checked_background",
                "switch.hover_checked_background",
                "switch.checked_track_border_color",
                "button.active_background",
                "segmented_control.active_background",
            ]
        );
        Ok(())
    }

    #[test]
    fn the_button_set_moves_the_button_slider_and_switch_leaves() -> crate::Result<()> {
        let scheme = Scheme {
            button: ("200,180,150", "60,20,10"),
            ..BREEZE_LIGHT_SCHEME
        };
        assert_eq!(
            moved_leaves(&scheme)?,
            [
                "button.hover_background",
                "button.hover_text_color",
                "slider.thumb_color",
                "switch.unchecked_background",
                "switch.checked_background",
                "switch.hover_unchecked_background",
                "switch.hover_checked_background",
                "switch.thumb_background",
                "switch.unchecked_track_border_color",
                "switch.thumb_border_color",
                "button.active_background",
            ]
        );
        Ok(())
    }

    #[test]
    fn the_header_set_moves_the_toolbar() -> crate::Result<()> {
        let scheme = Scheme {
            header: Some(("180,200,220", "10,40,80")),
            ..BREEZE_LIGHT_SCHEME
        };
        assert_eq!(
            moved_leaves(&scheme)?,
            ["toolbar.background_color", "toolbar.border.color"]
        );
        Ok(())
    }

    /// The Window set moves the popover's edge, and the toolbar only where
    /// the scheme has no Header group to take it from.
    #[test]
    fn the_window_set_moves_the_popover_edge() -> crate::Result<()> {
        let window = ("230,220,200", "90,40,20");
        let with_header = Scheme {
            window,
            ..BREEZE_LIGHT_SCHEME
        };
        assert_eq!(
            moved_leaves(&with_header)?,
            [
                "popover.border.color",
                "scrollbar.thumb_hover_color",
                "scrollbar.thumb_active_color",
            ]
        );
        let without_header = Scheme {
            window,
            header: None,
            ..BREEZE_LIGHT_SCHEME
        };
        assert_eq!(
            moved_leaves(&without_header)?,
            [
                "toolbar.background_color",
                "toolbar.border.color",
                "popover.border.color",
                "scrollbar.thumb_hover_color",
                "scrollbar.thumb_active_color",
            ]
        );
        Ok(())
    }

    #[test]
    fn the_view_set_moves_the_card_edge() -> crate::Result<()> {
        let scheme = Scheme {
            view: ("250,240,230", "120,30,60"),
            ..BREEZE_LIGHT_SCHEME
        };
        assert_eq!(moved_leaves(&scheme)?, ["card.border.color"]);
        Ok(())
    }

    /// The View set's decorations (D7): DecorationHover moves a hovered
    /// field's frame, the tab hover and the scroll-bar handle, DecorationFocus
    /// a focused field's frame alone.
    #[test]
    fn the_view_decorations_move_the_frames_the_tab_hover_and_the_handle() -> crate::Result<()> {
        let hover = Scheme {
            hover: "240,160,20",
            ..BREEZE_LIGHT_SCHEME
        };
        assert_eq!(
            moved_leaves(&hover)?,
            [
                "input.hover_border_color",
                "tab.hover_background",
                "scrollbar.thumb_hover_color",
                "scrollbar.thumb_active_color",
            ]
        );
        let focus = Scheme {
            focus: "20,200,90",
            ..BREEZE_LIGHT_SCHEME
        };
        assert_eq!(moved_leaves(&focus)?, ["input.focus_border_color"]);
        Ok(())
    }

    #[test]
    fn the_frame_contrast_moves_the_edges() -> crate::Result<()> {
        let scheme = Scheme {
            frame_contrast: Some("0.5"),
            ..BREEZE_LIGHT_SCHEME
        };
        assert_eq!(
            moved_leaves(&scheme)?,
            [
                "switch.unchecked_track_border_color",
                "switch.thumb_border_color",
                "toolbar.border.color",
                "popover.border.color",
                "card.border.color",
            ]
        );
        Ok(())
    }

    /// Without a `[Colors:Header]` group KColorScheme's Header set is the
    /// Window group's (kcolorscheme.cpp:330-332 at 27066d47) and Breeze
    /// draws no tools-area fill (breezestyle.cpp:1322 at breeze f0b1d75), so
    /// the toolbar shows the window and its edge is the Window set's
    /// separator colour.
    #[test]
    fn without_a_header_group_the_toolbar_is_the_windows() -> crate::Result<()> {
        let window = ("230,220,200", "90,40,20");
        let without_header = resolved_light(&Scheme {
            window,
            header: None,
            ..BREEZE_LIGHT_SCHEME
        })?;
        let header_as_window = resolved_light(&Scheme {
            window,
            header: Some(window),
            ..BREEZE_LIGHT_SCHEME
        })?;
        assert_eq!(
            without_header.toolbar.background_color,
            Rgba::rgb(230, 220, 200)
        );
        assert_eq!(
            without_header.toolbar.background_color,
            header_as_window.toolbar.background_color
        );
        assert_eq!(
            without_header.toolbar.border.color,
            header_as_window.toolbar.border.color
        );
        Ok(())
    }

    /// A `[Colors:Header]` group that lacks a key takes that key from
    /// `[Colors:Window]` (kcolorscheme.cpp:330-332 at 27066d47), the other
    /// from the group.
    #[test]
    fn a_header_group_takes_a_missing_key_from_the_window() -> crate::Result<()> {
        let window = ("230,220,200", "90,40,20");
        let without_header = Scheme {
            window,
            header: None,
            ..BREEZE_LIGHT_SCHEME
        };
        let with_one_key = |key: &str| -> crate::Result<crate::ResolvedTheme> {
            let content = format!(
                "{}[Colors:Header]\n{key}=180,200,220\n",
                without_header.kdeglobals()
            );
            let st = crate::pipeline::run_pipeline(from_kde_content(&content)?, "kde-breeze")?;
            Ok(st.light)
        };
        let no_text = with_one_key("BackgroundNormal")?;
        let expected = resolved_light(&Scheme {
            header: Some(("180,200,220", window.1)),
            ..without_header
        })?;
        assert_eq!(no_text.toolbar.background_color, Rgba::rgb(180, 200, 220));
        assert_eq!(no_text.toolbar.border.color, expected.toolbar.border.color);
        let no_fill = with_one_key("ForegroundNormal")?;
        let expected = resolved_light(&Scheme {
            header: Some((window.0, "180,200,220")),
            ..without_header
        })?;
        assert_eq!(no_fill.toolbar.background_color, Rgba::rgb(230, 220, 200));
        assert_eq!(no_fill.toolbar.border.color, expected.toolbar.border.color);
        Ok(())
    }

    /// A colour scheme moves none of the sizes and shadows `kde-breeze`
    /// states beside the derived colours: the toolbar edge's width and
    /// shadow, the popover's padding, the card's width, radius, shadow and
    /// padding, the progress bar's padding: each is the resolved preset's.
    #[test]
    fn a_colour_scheme_keeps_the_stated_edge_geometry() -> crate::Result<()> {
        let preset = crate::Theme::preset("kde-breeze")?
            .resolve(crate::ColorMode::Light)?
            .variant;
        let other = resolved_light(&Scheme {
            button: ("200,180,150", "60,20,10"),
            header: Some(("180,200,220", "10,40,80")),
            selection: "233,84,32",
            view: ("250,240,230", "120,30,60"),
            focus: "20,200,90",
            hover: "240,160,20",
            window: ("230,220,200", "90,40,20"),
            frame_contrast: Some("0.5"),
        })?;
        let geometry = |b: &crate::ResolvedWidgetBorder| {
            (b.corner_radius, b.line_width, b.shadow_enabled, b.padding)
        };
        for (leaf, a, b) in [
            ("toolbar", &preset.toolbar.border, &other.toolbar.border),
            ("popover", &preset.popover.border, &other.popover.border),
            ("card", &preset.card.border, &other.card.border),
            (
                "progress_bar",
                &preset.progress_bar.border,
                &other.progress_bar.border,
            ),
        ] {
            assert_eq!(geometry(a), geometry(b), "{leaf}.border");
        }
        Ok(())
    }

    /// `menuFont` sets the menu's family, size, weight and style and keeps
    /// the colour the colour scheme gave it.
    #[test]
    fn the_menu_font_keeps_the_window_text_colour() -> crate::Result<()> {
        let result = from_kde_content(BREEZE_DARK_FULL)?;
        let font = reader_mode(&result).menu.font.clone().unwrap_or_default();
        assert_eq!(font.family.as_deref(), Some("Noto Sans"));
        assert_eq!(font.color, Some(Rgba::rgb(239, 240, 241)));
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

    /// `forceFontDPI` lives in kcmfonts, which the fonts KCM writes it to
    /// and krdb reads it from (docs/platform-facts.md:717); a kdeglobals key
    /// of that name is no font DPI and does not replace the one given.
    #[test]
    fn a_kdeglobals_force_font_dpi_is_not_the_font_dpi() -> crate::Result<()> {
        let content = "[General]\nforceFontDPI=120\n";
        let (_theme, dpi, _accessibility) = from_kde_content_pure(content, None)?;
        assert_eq!(dpi, None);
        let (_theme, dpi, _accessibility) = from_kde_content_pure(content, Some(144.0))?;
        assert_eq!(dpi, Some(144.0));
        Ok(())
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
        // No font DPI given -> None from pure function
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

    /// Breeze's index.theme from breeze-icons 6.30.0
    /// (`/usr/share/icons/breeze/index.theme`): its `[Icon Theme]` group
    /// (:1-145), `[actions/12]` (:146-150) and `[apps/32]` (:329-332).
    const BREEZE_INDEX_THEME: &str = include_str!("../../tests/fixtures/kde/breeze-index.theme");

    /// The sizes are the `[Icon Theme]` group's `*Default` keys, as
    /// KIconTheme reads them, not the smallest directories the theme lists.
    #[test]
    fn icon_sizes_are_the_icon_themes_default_keys() {
        let mut ini = create_kde_parser();
        read_kconfig(&mut ini, BREEZE_INDEX_THEME);
        assert_eq!(
            parse_icon_sizes_from_content(&ini),
            IconSizes {
                toolbar: Some(22.0),
                small: Some(16.0),
                large: Some(48.0),
                dialog: Some(32.0),
                panel: Some(48.0),
            }
        );
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

    /// A theme that states no default sizes has KIconTheme's fallbacks,
    /// whatever directories it lists.
    #[test]
    fn an_icon_theme_without_default_keys_has_kiconthemes_fallbacks() {
        let content = "\
[Icon Theme]
Name=sizeless
Directories=actions/12

[actions/12]
Size=12
Context=Actions
Type=Fixed
";
        let mut ini = create_kde_parser();
        read_kconfig(&mut ini, content);
        assert_eq!(
            parse_icon_sizes_from_content(&ini),
            IconSizes {
                toolbar: Some(22.0),
                small: Some(16.0),
                large: Some(32.0),
                dialog: Some(32.0),
                panel: Some(48.0),
            }
        );
    }

    /// The index.theme is found in any base directory, `~/.icons` among
    /// them, the first base holding it winning.
    #[test]
    fn the_index_theme_is_found_in_any_icon_base_dir() {
        let dir = temp_config_dir("icon-bases");
        let data_icons = dir.join("share/icons");
        let dot_icons = dir.join("home/.icons");
        std::fs::create_dir_all(dot_icons.join("dotted")).unwrap();
        std::fs::create_dir_all(data_icons.join("both")).unwrap();
        std::fs::create_dir_all(dot_icons.join("both")).unwrap();
        std::fs::write(
            dot_icons.join("dotted/index.theme"),
            "[Icon Theme]\nSmallDefault=24\n",
        )
        .unwrap();
        std::fs::write(
            data_icons.join("both/index.theme"),
            "[Icon Theme]\nDesktopDefault=64\n",
        )
        .unwrap();
        std::fs::write(
            dot_icons.join("both/index.theme"),
            "[Icon Theme]\nDesktopDefault=32\n",
        )
        .unwrap();
        let bases = [data_icons, dot_icons];

        let dotted = icon_sizes_in_bases("dotted", &bases);
        assert_eq!(dotted.small, Some(24.0));
        assert_eq!(icon_sizes_in_bases("both", &bases).large, Some(64.0));
        assert_eq!(icon_sizes_in_bases("absent", &bases), IconSizes::default());
    }

    /// KIconTheme's order: the user's data home, then each data dir, then
    /// `~/.icons`, which KDE does not search, last; with no home directory
    /// the data dirs are still searched.
    #[test]
    fn kde_icon_base_dirs_follow_kiconthemes_order() {
        use std::ffi::OsStr;
        use std::path::Path;

        assert_eq!(
            kde_icon_base_dirs(Some(Path::new("/h")), None, Some(OsStr::new("/a:rel:/b"))),
            paths(&["/h/.local/share/icons", "/a/icons", "/b/icons", "/h/.icons"])
        );
        assert_eq!(
            kde_icon_base_dirs(
                Some(Path::new("/h")),
                Some(OsStr::new("/data")),
                Some(OsStr::new(""))
            ),
            paths(&[
                "/data/icons",
                "/usr/local/share/icons",
                "/usr/share/icons",
                "/h/.icons"
            ])
        );
        assert_eq!(
            kde_icon_base_dirs(None, Some(OsStr::new("rel")), Some(OsStr::new("/a"))),
            paths(&["/a/icons"])
        );
    }

    /// A theme the user installed over a system one, as "Get New Icons"
    /// does into `~/.local/share/icons`, gives the user copy's sizes, the
    /// copy KIconTheme reads.
    #[test]
    fn the_users_copy_of_an_icon_theme_wins() {
        let dir = temp_config_dir("icon-user-copy");
        let home = dir.join("home");
        let system = dir.join("usr/share");
        std::fs::create_dir_all(home.join(".local/share/icons/copied")).unwrap();
        std::fs::create_dir_all(system.join("icons/copied")).unwrap();
        std::fs::write(
            home.join(".local/share/icons/copied/index.theme"),
            "[Icon Theme]\nDesktopDefault=64\n",
        )
        .unwrap();
        std::fs::write(
            system.join("icons/copied/index.theme"),
            "[Icon Theme]\nDesktopDefault=48\n",
        )
        .unwrap();
        let bases = kde_icon_base_dirs(Some(&home), None, Some(system.as_os_str()));
        assert_eq!(icon_sizes_in_bases("copied", &bases).large, Some(64.0));
    }

    /// A key a found theme omits, or one that is no size, has KIconTheme's
    /// own fallback for its group.
    #[test]
    fn a_key_the_icon_theme_omits_has_kiconthemes_fallback() {
        let mut ini = create_kde_parser();
        read_kconfig(
            &mut ini,
            "[Icon Theme]\nName=partial\nSmallDefault=24\nPanelDefault=big\n",
        );
        assert_eq!(
            parse_icon_sizes_from_content(&ini),
            IconSizes {
                toolbar: Some(22.0),
                small: Some(24.0),
                large: Some(32.0),
                dialog: Some(32.0),
                panel: Some(48.0),
            }
        );
    }

    /// A name that is not one directory, as `../dotted`, is no theme: the
    /// search never leaves the base directories.
    #[test]
    fn an_icon_theme_name_with_a_path_is_not_searched() {
        let dir = temp_config_dir("icon-name");
        std::fs::create_dir_all(dir.join("icons")).unwrap();
        std::fs::create_dir_all(dir.join("dotted")).unwrap();
        std::fs::write(
            dir.join("dotted/index.theme"),
            "[Icon Theme]\nSmallDefault=24\n",
        )
        .unwrap();
        let bases = [dir.join("icons")];
        assert_eq!(
            icon_sizes_in_bases("../dotted", &bases),
            IconSizes::default()
        );
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
        assert_eq!(sizes.small, Some(16.0));
        assert_eq!(sizes.toolbar, Some(22.0));
        assert_eq!(sizes.large, Some(32.0));
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
