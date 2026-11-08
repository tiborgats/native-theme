//! The KDE icon theme (the `kde` feature, Linux): `[Icons] Theme` of the
//! kdeglobals files as KConfig cascades them.
//!
//! The files are `$XDG_CONFIG_DIRS/kdeglobals` (`/etc/xdg/kdeglobals`), then
//! `kdedefaults/kdeglobals`, the defaults Plasma keeps beneath the user's
//! file, then the user's `kdeglobals`, each in the user's config directory
//! (`$XDG_CONFIG_HOME`, else `~/.config`).
//!
//! The clean room's Qt font parser, `qfont_from_string` (the 0.8.0 plan's
//! D10), its code placed unchanged in `src/cleanroom_readers/`, is linked
//! here with its vector test; the KDE reader, which reads kdeglobals' font entries
//! with it, returns with the `kde-breeze` preset (M1b).

#[path = "../cleanroom_readers/qfont_from_string.rs"]
#[allow(
    dead_code,
    reason = "the KDE reader's helper, which reads kdeglobals' font entries once the reader returns with kde-breeze (M1b); linked now so its vector test runs"
)]
#[allow(
    clippy::collapsible_if,
    reason = "the clean room's parser, placed unchanged (D10): a style lint only"
)]
mod qfont_from_string;

/// The icon theme `ini` names: `[Icons] Theme`, `None` where it is absent or
/// empty.
fn icon_theme(ini: &configparser::ini::Ini) -> Option<String> {
    ini.get("Icons", "Theme").filter(|s| !s.is_empty())
}

/// The icon theme of the kdeglobals files ([`kdeglobals_files`]), for
/// `detect`'s icon-theme detection; the error says why
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

/// The user's kdeglobals file, in the user's config directory
/// ([`crate::detect::xdg_config_home`]); `None` without one.
pub(crate) fn kdeglobals_path() -> Option<std::path::PathBuf> {
    crate::detect::xdg_config_home().map(|dir| dir.join("kdeglobals"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

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

    /// `content` read as KConfig reads it ([`read_kconfig`]).
    fn kconfig(content: &str) -> configparser::ini::Ini {
        let mut ini = create_kde_parser();
        read_kconfig(&mut ini, content);
        ini
    }

    /// KConfig skips a group header without `]` and keeps the group before
    /// it; the rest of the file is still read.
    #[test]
    fn a_stray_group_line_is_skipped() {
        let ini = kconfig(
            "[Colors:Window\nBackgroundNormal=49,54,59\n\n[General]\nColorScheme=BreezeDark\n",
        );
        assert_eq!(
            ini.get("General", "ColorScheme").as_deref(),
            Some("BreezeDark")
        );
    }

    /// KConfig skips an entry with an empty key.
    #[test]
    fn a_line_with_an_empty_key_is_skipped() {
        let ini = kconfig("[General]\n=stray\nColorScheme=BreezeDark\n");
        assert_eq!(
            ini.get("General", "ColorScheme").as_deref(),
            Some("BreezeDark")
        );
    }

    /// KConfig has no inline comments: a `#` in a value is part of it.
    #[test]
    fn a_hash_in_a_value_is_kept() {
        let ini = kconfig("[General]\nColorScheme=My#Scheme\n");
        assert_eq!(
            ini.get("General", "ColorScheme").as_deref(),
            Some("My#Scheme")
        );
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
            "[General]\nColorScheme=SystemScheme\n\n[Icons]\nTheme=breeze\n",
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
        let ini = read_kconfig_files(&files).expect("the files are readable");
        assert_eq!(
            ini.get("General", "ColorScheme").as_deref(),
            Some("BreezeDark")
        );
        assert_eq!(
            ini.get("Colors:Window", "BackgroundNormal").as_deref(),
            Some("49,54,59")
        );
        assert_eq!(
            kconfig_files_icon_theme(&files),
            Ok("breeze-dark".to_string())
        );
    }

    /// The icon theme `detect` reports is the cascade's: a
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

    /// A byte that is not UTF-8 is replaced; the rest of the file is read.
    #[test]
    fn a_non_utf8_byte_keeps_the_theme() {
        let dir = temp_config_dir("non-utf8");
        std::fs::write(
            dir.join("kdeglobals"),
            b"[General]\nColorScheme=Caf\xe9\n\n[Icons]\nTheme=breeze-dark\n",
        )
        .unwrap();
        let files = [dir.join("kdeglobals")];
        let ini = read_kconfig_files(&files).expect("the file is readable");
        assert_eq!(
            ini.get("General", "ColorScheme").as_deref(),
            Some("Caf\u{fffd}")
        );
        assert_eq!(
            kconfig_files_icon_theme(&files),
            Ok("breeze-dark".to_string())
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
}
