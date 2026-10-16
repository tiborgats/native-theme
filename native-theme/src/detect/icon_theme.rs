// Icon set and icon theme detection: the native icon set per OS, and the
// icon theme each Linux desktop keeps in its settings.

use crate::model::IconSet;

/// Detect the native icon set for the current operating system.
///
/// Returns the platform-appropriate icon set at runtime using `cfg!()` macros:
/// - macOS / iOS: [`IconSet::SfSymbols`]
/// - Windows: [`IconSet::SegoeFluent`]
/// - Linux: [`IconSet::Freedesktop`]
/// - Other: [`IconSet::Material`] (safe cross-platform fallback)
///
/// # Examples
///
/// ```
/// use native_theme::detect::system_icon_set;
///
/// let set = system_icon_set();
/// // On Linux, this returns Freedesktop
/// ```
#[must_use]
pub fn system_icon_set() -> IconSet {
    if cfg!(any(target_os = "macos", target_os = "ios")) {
        IconSet::SfSymbols
    } else if cfg!(target_os = "windows") {
        IconSet::SegoeFluent
    } else if cfg!(target_os = "linux") {
        IconSet::Freedesktop
    } else {
        IconSet::Material
    }
}

/// Detect the icon theme name for the current platform (cached).
///
/// Returns the name of the icon theme that provides the actual icon files:
/// - **macOS / iOS:** `"sf-symbols"` (no user-configurable icon theme)
/// - **Windows:** `"segoe-fluent"` (no user-configurable icon theme)
/// - **Linux:** DE-specific detection (e.g., `"breeze-dark"`, `"Adwaita"`)
/// - **Other:** an error; there is no icon theme to detect
///
/// On Linux, the detection method depends on the desktop environment, the
/// one the reader is chosen by: an unrecognised `XDG_CURRENT_DESKTOP` counts
/// as the desktop of its portal backend (once a system read such as
/// [`SystemTheme::from_system`](crate::SystemTheme::from_system) has asked
/// it, with the `portal` feature), else as KDE where the user's `kdeglobals`
/// exists (with the `kde` feature):
/// - KDE: `[Icons] Theme` of kdeglobals as KConfig cascades it, the KDE
///   reader's read (with the `kde` feature); without the feature, from
///   `kdeglobals`, then from `kdedefaults/kdeglobals`
/// - GNOME/Budgie: `gsettings get org.gnome.desktop.interface icon-theme`
/// - Cinnamon: `gsettings get org.cinnamon.desktop.interface icon-theme`
/// - XFCE: `xfconf-query -c xsettings -p /Net/IconThemeName`
/// - MATE: `gsettings get org.mate.interface icon-theme`
/// - LXQt: reads `icon_theme` from `~/.config/lxqt/lxqt.conf`
/// - Unknown (still): tries KDE, then GNOME gsettings
///
/// No theme stands in for one that cannot be detected: the error says why,
/// and what to do without a theme is the caller's decision.
///
/// Delegates to [`DetectionContext::icon_theme()`](crate::detect::DetectionContext::icon_theme)
/// which caches the outcome, a failure included, and supports per-field
/// invalidation via [`invalidate_caches()`](crate::detect::invalidate_caches).
///
/// # Errors
///
/// - [`Error::ReaderFailed`](crate::Error::ReaderFailed) with `reader`
///   `"icon-theme"` on Linux when no source names a theme. Its message names
///   each source tried and why it gave no name: the config directory could
///   not be resolved (no absolute `$XDG_CONFIG_HOME`, and `$HOME` unset or
///   empty), a file could not be read or lacks the key, or a command was not
///   found, exited unsuccessfully, printed no name or did not finish within
///   2 s. On an unrecognised desktop it names both attempts, KDE's and
///   GNOME's.
/// - [`Error::PlatformUnsupported`](crate::Error::PlatformUnsupported) on a
///   platform other than Linux, macOS, iOS and Windows.
///
/// # Examples
///
/// ```
/// use native_theme::detect::system_icon_theme;
///
/// match system_icon_theme() {
///     // On a KDE system with Breeze Dark: "breeze-dark"; on macOS: "sf-symbols"
///     Ok(theme) => println!("icon theme: {theme}"),
///     Err(e) => println!("no icon theme: {e}"),
/// }
/// ```
pub fn system_icon_theme() -> crate::Result<String> {
    crate::detect::system().icon_theme()
}

/// Detect the icon theme name for the current platform without caching.
///
/// Unlike [`system_icon_theme()`], this function queries the OS every time it
/// is called and never caches the result. Use this when polling for icon theme
/// changes (e.g., the user switches from Breeze to Breeze Dark in system
/// settings).
///
/// See [`system_icon_theme()`] for platform behavior details.
///
/// # Errors
///
/// As [`system_icon_theme()`].
pub fn detect_icon_theme() -> crate::Result<String> {
    detect_icon_theme_outcome().map_err(|failure| failure.to_error())
}

/// [`detect_icon_theme()`]'s outcome in the form the detection cache keeps.
pub(crate) fn detect_icon_theme_outcome() -> Result<String, IconThemeFailure> {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        Ok("sf-symbols".to_string())
    }

    #[cfg(target_os = "windows")]
    {
        Ok("segoe-fluent".to_string())
    }

    #[cfg(target_os = "linux")]
    {
        detect_linux_icon_theme()
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "windows",
        target_os = "macos",
        target_os = "ios"
    )))]
    {
        Err(IconThemeFailure::Unsupported(std::env::consts::OS))
    }
}

/// One source's failure to name an icon theme: the source, and why it gave
/// no name.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub(crate) struct SourceFailure {
    source: String,
    reason: String,
}

impl std::fmt::Display for SourceFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.source, self.reason)
    }
}

/// Why no icon theme was detected: each source tried, in order, with why it
/// gave no name. The source of the
/// [`Error::ReaderFailed`](crate::Error::ReaderFailed) a failed detection
/// reports.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub(crate) struct IconThemeUndetected(Vec<SourceFailure>);

impl std::fmt::Display for IconThemeUndetected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sources = self.0.iter();
        if let Some(first) = sources.next() {
            write!(f, "{first}")?;
        }
        for failure in sources {
            write!(f, "; {failure}")?;
        }
        Ok(())
    }
}

impl std::error::Error for IconThemeUndetected {}

/// A failed icon-theme detection as the detection cache keeps it. Unlike
/// [`crate::Error`] it is `Clone`, so the cache can hand out the failure it
/// holds; [`to_error`](Self::to_error) builds the error from it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IconThemeFailure {
    /// No source named a theme.
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    Undetected(IconThemeUndetected),
    /// The platform has no icon theme to detect.
    #[cfg_attr(
        any(
            target_os = "linux",
            target_os = "windows",
            target_os = "macos",
            target_os = "ios"
        ),
        allow(dead_code)
    )]
    Unsupported(&'static str),
}

impl IconThemeFailure {
    /// The error [`system_icon_theme()`] and [`detect_icon_theme()`] report.
    pub(crate) fn to_error(&self) -> crate::Error {
        match self {
            Self::Undetected(undetected) => crate::Error::ReaderFailed {
                reader: crate::error::reader_names::ICON_THEME,
                source: Box::new(undetected.clone()),
            },
            Self::Unsupported(platform) => crate::Error::PlatformUnsupported { platform },
        }
    }

    /// A failure with `reason`, for tests of the code that keeps and
    /// reports failures.
    #[cfg(test)]
    pub(crate) fn for_tests(reason: &str) -> Self {
        Self::Undetected(IconThemeUndetected(vec![SourceFailure {
            source: "test".to_string(),
            reason: reason.to_string(),
        }]))
    }
}

/// The schema GNOME, and the desktops that follow its settings, keep the
/// icon theme in.
#[cfg(target_os = "linux")]
const GNOME_INTERFACE_SCHEMA: &str = "org.gnome.desktop.interface";

/// Where a Linux desktop keeps its icon theme.
#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IconThemeSource {
    /// KDE's kdeglobals.
    Kde,
    /// `gsettings get <schema> icon-theme`.
    Gsettings(&'static str),
    /// XFCE's xfconf.
    Xfce,
    /// LXQt's `lxqt.conf`.
    LxQt,
    /// KDE's, else GNOME's: a desktop that nothing names.
    KdeThenGnome,
}

/// The icon-theme source of `desktop`, the refined desktop
/// ([`crate::detect::refine_linux_desktop`]) the reader is chosen by.
#[cfg(target_os = "linux")]
fn icon_theme_source(desktop: crate::detect::LinuxDesktop) -> IconThemeSource {
    use crate::detect::LinuxDesktop;

    match desktop {
        LinuxDesktop::Kde => IconThemeSource::Kde,
        LinuxDesktop::Gnome
        | LinuxDesktop::Budgie
        | LinuxDesktop::Hyprland
        | LinuxDesktop::Sway
        | LinuxDesktop::River
        | LinuxDesktop::Niri
        | LinuxDesktop::Wayfire
        | LinuxDesktop::CosmicDe => IconThemeSource::Gsettings(GNOME_INTERFACE_SCHEMA),
        LinuxDesktop::Cinnamon => IconThemeSource::Gsettings("org.cinnamon.desktop.interface"),
        LinuxDesktop::Xfce => IconThemeSource::Xfce,
        LinuxDesktop::Mate => IconThemeSource::Gsettings("org.mate.interface"),
        LinuxDesktop::LxQt => IconThemeSource::LxQt,
        LinuxDesktop::Unknown => IconThemeSource::KdeThenGnome,
    }
}

/// Linux icon theme detection, dispatched by the refined desktop.
#[cfg(target_os = "linux")]
fn detect_linux_icon_theme() -> Result<String, IconThemeFailure> {
    let desktop = crate::detect::system().refined_linux_desktop_sync();
    let found = match icon_theme_source(desktop) {
        IconThemeSource::Kde => detect_kde_icon_theme(),
        IconThemeSource::Gsettings(schema) => gsettings_icon_theme(schema),
        IconThemeSource::Xfce => detect_xfce_icon_theme(),
        IconThemeSource::LxQt => detect_lxqt_icon_theme(),
        IconThemeSource::KdeThenGnome => {
            return unknown_desktop_icon_theme(detect_kde_icon_theme, || {
                gsettings_icon_theme(GNOME_INTERFACE_SCHEMA)
            });
        }
    };
    found.map_err(|failure| IconThemeFailure::Undetected(IconThemeUndetected(vec![failure])))
}

/// The icon theme of a desktop that is not recognised: KDE's, else GNOME's.
/// Where both fail, the failure names both attempts.
#[cfg(target_os = "linux")]
fn unknown_desktop_icon_theme(
    kde: impl FnOnce() -> Result<String, SourceFailure>,
    gnome: impl FnOnce() -> Result<String, SourceFailure>,
) -> Result<String, IconThemeFailure> {
    let kde_failure = match kde() {
        Ok(theme) => return Ok(theme),
        Err(failure) => failure,
    };
    let gnome_failure = match gnome() {
        Ok(theme) => return Ok(theme),
        Err(failure) => failure,
    };
    Err(IconThemeFailure::Undetected(IconThemeUndetected(vec![
        kde_failure,
        gnome_failure,
    ])))
}

/// The failure of a config-file source when there is no config directory.
#[cfg(target_os = "linux")]
fn config_dir_unresolved(source: &str) -> SourceFailure {
    SourceFailure {
        source: source.to_string(),
        reason: crate::detect::CONFIG_HOME_UNRESOLVED.to_string(),
    }
}

/// The source name of KDE's icon theme in a failure.
#[cfg(target_os = "linux")]
const KDE_SOURCE: &str = "KDE kdeglobals";

/// The icon theme of kdeglobals as the KDE reader reads it
/// ([`crate::kde::kdeglobals_icon_theme`]), so the two cannot disagree.
#[cfg(all(target_os = "linux", feature = "kde"))]
fn detect_kde_icon_theme() -> Result<String, SourceFailure> {
    crate::kde::kdeglobals_icon_theme().map_err(|reason| SourceFailure {
        source: KDE_SOURCE.to_string(),
        reason,
    })
}

/// Read the icon theme from KDE's kdeglobals INI file, without the `kde`
/// feature, so without the KDE reader.
#[cfg(all(target_os = "linux", not(feature = "kde")))]
fn detect_kde_icon_theme() -> Result<String, SourceFailure> {
    kde_icon_theme(crate::detect::xdg_config_home().as_deref(), |path| {
        std::fs::read_to_string(path)
    })
}

/// `[Icons] Theme` from `kdeglobals` in `config_dir`, then from
/// `kdedefaults/kdeglobals` (Plasma 6 stores distro defaults there,
/// including the icon theme), each file read by `read`.
///
/// Uses simple line parsing — no `configparser` dependency required — so this
/// works without the `kde` feature enabled.
#[cfg(all(target_os = "linux", not(feature = "kde")))]
fn kde_icon_theme(
    config_dir: Option<&std::path::Path>,
    read: impl Fn(&std::path::Path) -> std::io::Result<String>,
) -> Result<String, SourceFailure> {
    let config_dir = config_dir.ok_or_else(|| config_dir_unresolved(KDE_SOURCE))?;
    let paths = [
        config_dir.join("kdeglobals"),
        config_dir.join("kdedefaults").join("kdeglobals"),
    ];

    let mut reasons = Vec::new();
    for path in &paths {
        match read(path) {
            Ok(content) => match crate::icons::ini_value(&content, "Icons", "Theme") {
                Some(theme) => return Ok(theme.to_string()),
                None => reasons.push(format!(
                    "{}: `[Icons] Theme` is absent or empty",
                    path.display()
                )),
            },
            Err(e) => reasons.push(format!("{}: unreadable ({e})", path.display())),
        }
    }
    Err(SourceFailure {
        source: KDE_SOURCE.to_string(),
        reason: reasons.join(", "),
    })
}

/// Query gsettings for icon-theme with the given schema.
#[cfg(target_os = "linux")]
fn gsettings_icon_theme(schema: &str) -> Result<String, SourceFailure> {
    gsettings_icon_theme_from(
        schema,
        crate::detect::run_with_timeout(
            "gsettings",
            &["get", schema, "icon-theme"],
            crate::detect::SUBPROCESS_TIMEOUT,
        ),
    )
}

/// The icon theme in `gsettings get <schema> icon-theme`'s `output`, which
/// prints it quoted (`'Adwaita'`).
#[cfg(target_os = "linux")]
fn gsettings_icon_theme_from(
    schema: &str,
    output: std::io::Result<std::process::Output>,
) -> Result<String, SourceFailure> {
    command_icon_theme(
        format!("`gsettings get {schema} icon-theme`"),
        output,
        |printed| printed.trim().trim_matches('\''),
    )
}

/// `xfconf-query`'s arguments that print XFCE's icon theme.
#[cfg(target_os = "linux")]
const XFCONF_ICON_THEME_ARGS: [&str; 4] = ["-c", "xsettings", "-p", "/Net/IconThemeName"];

/// Read icon theme from XFCE's xfconf-query.
#[cfg(target_os = "linux")]
fn detect_xfce_icon_theme() -> Result<String, SourceFailure> {
    xfce_icon_theme_from(crate::detect::run_with_timeout(
        "xfconf-query",
        &XFCONF_ICON_THEME_ARGS,
        crate::detect::SUBPROCESS_TIMEOUT,
    ))
}

/// The icon theme in `xfconf-query -c xsettings -p /Net/IconThemeName`'s
/// `output`.
#[cfg(target_os = "linux")]
fn xfce_icon_theme_from(
    output: std::io::Result<std::process::Output>,
) -> Result<String, SourceFailure> {
    command_icon_theme(
        format!("`xfconf-query {}`", XFCONF_ICON_THEME_ARGS.join(" ")),
        output,
        str::trim,
    )
}

/// The icon theme the command `source` names printed, taken out of its
/// `output` by `parse`, or why there is none.
#[cfg(target_os = "linux")]
fn command_icon_theme(
    source: String,
    output: std::io::Result<std::process::Output>,
    parse: fn(&str) -> &str,
) -> Result<String, SourceFailure> {
    let reason = match output {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => "command not found".to_string(),
        Err(e) if e.kind() == std::io::ErrorKind::TimedOut => e.to_string(),
        Err(e) => format!("could not run ({e})"),
        Ok(output) if !output.status.success() => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            match stderr.trim() {
                "" => format!("unsuccessful ({})", output.status),
                stderr => format!("unsuccessful ({}): {stderr}", output.status),
            }
        }
        Ok(output) => match String::from_utf8(output.stdout) {
            Err(_) => "printed output that is not UTF-8".to_string(),
            Ok(printed) => match parse(&printed) {
                "" => "printed no theme name".to_string(),
                theme => return Ok(theme.to_string()),
            },
        },
    };
    Err(SourceFailure { source, reason })
}

/// Read icon theme from LXQt's config file.
#[cfg(target_os = "linux")]
fn detect_lxqt_icon_theme() -> Result<String, SourceFailure> {
    lxqt_icon_theme(crate::detect::xdg_config_home().as_deref(), |path| {
        std::fs::read_to_string(path)
    })
}

/// `icon_theme` from `lxqt/lxqt.conf` in `config_dir`, read by `read`.
///
/// LXQt uses a flat `key=value` format (no section headers for the icon_theme
/// key), so we scan for the bare `icon_theme=` prefix.
#[cfg(target_os = "linux")]
fn lxqt_icon_theme(
    config_dir: Option<&std::path::Path>,
    read: impl Fn(&std::path::Path) -> std::io::Result<String>,
) -> Result<String, SourceFailure> {
    const SOURCE: &str = "LXQt lxqt.conf";
    let config_dir = config_dir.ok_or_else(|| config_dir_unresolved(SOURCE))?;
    let path = config_dir.join("lxqt").join("lxqt.conf");
    let failure = |reason: String| SourceFailure {
        source: SOURCE.to_string(),
        reason: format!("{}: {reason}", path.display()),
    };

    let content = read(&path).map_err(|e| failure(format!("unreadable ({e})")))?;
    content
        .lines()
        .filter_map(|line| line.trim().strip_prefix("icon_theme="))
        .map(str::trim)
        .find(|value| !value.is_empty())
        .map(str::to_string)
        .ok_or_else(|| failure("`icon_theme` is absent or empty".to_string()))
}

#[cfg(test)]
#[cfg(target_os = "linux")]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use std::path::{Path, PathBuf};
    use std::process::{ExitStatus, Output};

    // === system_icon_set() tests ===

    #[test]
    fn system_icon_set_returns_freedesktop_on_linux() {
        assert_eq!(system_icon_set(), IconSet::Freedesktop);
    }

    fn missing(_: &Path) -> std::io::Result<String> {
        Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    }

    fn printed(stdout: &str) -> std::io::Result<Output> {
        Ok(Output {
            status: ExitStatus::from_raw(0),
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
        })
    }

    fn exited_1(stderr: &str) -> std::io::Result<Output> {
        Ok(Output {
            status: ExitStatus::from_raw(1 << 8),
            stdout: Vec::new(),
            stderr: stderr.as_bytes().to_vec(),
        })
    }

    fn not_found() -> std::io::Result<Output> {
        Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    }

    fn timed_out() -> std::io::Result<Output> {
        Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "timed out after 2s",
        ))
    }

    // === KDE: kdeglobals ===

    #[cfg(not(feature = "kde"))]
    #[test]
    fn kde_without_a_config_dir_names_kdeglobals_and_the_config_dir() {
        let failure = kde_icon_theme(None, missing).unwrap_err().to_string();
        assert!(failure.contains("kdeglobals"), "got: {failure}");
        assert!(failure.contains("config directory"), "got: {failure}");
        assert!(failure.contains("XDG_CONFIG_HOME"), "got: {failure}");
    }

    #[cfg(not(feature = "kde"))]
    #[test]
    fn kde_with_no_readable_file_names_both_files() {
        let dir = PathBuf::from("/cfg");
        let failure = kde_icon_theme(Some(&dir), missing).unwrap_err().to_string();
        assert!(failure.contains("/cfg/kdeglobals"), "got: {failure}");
        assert!(
            failure.contains("/cfg/kdedefaults/kdeglobals"),
            "got: {failure}"
        );
        assert!(failure.contains("unreadable"), "got: {failure}");
    }

    #[cfg(not(feature = "kde"))]
    #[test]
    fn kde_with_no_icons_theme_key_says_the_key_is_absent() {
        let dir = PathBuf::from("/cfg");
        let failure = kde_icon_theme(Some(&dir), |_: &Path| {
            Ok("[General]\nTheme=not-this-one\n".to_string())
        })
        .unwrap_err()
        .to_string();
        assert!(failure.contains("kdeglobals"), "got: {failure}");
        assert!(
            failure.contains("`[Icons] Theme` is absent or empty"),
            "got: {failure}"
        );
    }

    #[cfg(not(feature = "kde"))]
    #[test]
    fn kde_takes_the_defaults_file_where_kdeglobals_lacks_the_key() {
        let dir = PathBuf::from("/cfg");
        let theme = kde_icon_theme(Some(&dir), |path: &Path| {
            if path.ends_with("kdedefaults/kdeglobals") {
                Ok("[Icons]\nTheme=breeze-dark\n".to_string())
            } else {
                Ok("[General]\n".to_string())
            }
        });
        assert_eq!(theme, Ok("breeze-dark".to_string()));
    }

    // === gsettings ===

    #[test]
    fn gsettings_not_found_names_the_command() {
        let failure = gsettings_icon_theme_from("org.gnome.desktop.interface", not_found())
            .unwrap_err()
            .to_string();
        assert!(
            failure.contains("gsettings get org.gnome.desktop.interface icon-theme"),
            "got: {failure}"
        );
        assert!(failure.contains("command not found"), "got: {failure}");
    }

    #[test]
    fn gsettings_that_timed_out_says_so() {
        let failure = gsettings_icon_theme_from("org.gnome.desktop.interface", timed_out())
            .unwrap_err()
            .to_string();
        assert!(
            failure.contains("gsettings get org.gnome.desktop.interface icon-theme"),
            "got: {failure}"
        );
        assert!(failure.contains("timed out after 2s"), "got: {failure}");
        assert!(!failure.contains("could not run"), "got: {failure}");
    }

    #[test]
    fn gsettings_nonzero_exit_gives_the_status_and_stderr() {
        let failure = gsettings_icon_theme_from(
            "org.mate.interface",
            exited_1("No such schema “org.mate.interface”\n"),
        )
        .unwrap_err()
        .to_string();
        assert!(
            failure.contains("gsettings get org.mate.interface icon-theme"),
            "got: {failure}"
        );
        assert!(
            failure.contains("unsuccessful (exit status: 1)"),
            "got: {failure}"
        );
        assert!(failure.contains("No such schema"), "got: {failure}");
    }

    #[test]
    fn gsettings_empty_output_says_it_printed_no_name() {
        let failure = gsettings_icon_theme_from("org.cinnamon.desktop.interface", printed("''\n"))
            .unwrap_err()
            .to_string();
        assert!(
            failure.contains("gsettings get org.cinnamon.desktop.interface icon-theme"),
            "got: {failure}"
        );
        assert!(failure.contains("printed no theme name"), "got: {failure}");
    }

    #[test]
    fn gsettings_output_is_unquoted() {
        assert_eq!(
            gsettings_icon_theme_from("org.gnome.desktop.interface", printed("'Adwaita'\n")),
            Ok("Adwaita".to_string())
        );
    }

    // === XFCE: xfconf-query ===

    #[test]
    fn xfconf_not_found_names_the_command() {
        let failure = xfce_icon_theme_from(not_found()).unwrap_err().to_string();
        assert!(
            failure.contains("xfconf-query -c xsettings -p /Net/IconThemeName"),
            "got: {failure}"
        );
        assert!(failure.contains("command not found"), "got: {failure}");
    }

    #[test]
    fn xfconf_nonzero_exit_gives_the_status() {
        let failure = xfce_icon_theme_from(exited_1("")).unwrap_err().to_string();
        assert!(failure.contains("xfconf-query"), "got: {failure}");
        assert!(
            failure.contains("unsuccessful (exit status: 1)"),
            "got: {failure}"
        );
    }

    #[test]
    fn xfconf_empty_output_says_it_printed_no_name() {
        let failure = xfce_icon_theme_from(printed("\n")).unwrap_err().to_string();
        assert!(failure.contains("xfconf-query"), "got: {failure}");
        assert!(failure.contains("printed no theme name"), "got: {failure}");
    }

    #[test]
    fn xfconf_output_is_trimmed() {
        assert_eq!(
            xfce_icon_theme_from(printed("elementary\n")),
            Ok("elementary".to_string())
        );
    }

    // === LXQt: lxqt.conf ===

    #[test]
    fn lxqt_without_a_config_dir_names_lxqt_conf_and_the_config_dir() {
        let failure = lxqt_icon_theme(None, missing).unwrap_err().to_string();
        assert!(failure.contains("lxqt.conf"), "got: {failure}");
        assert!(failure.contains("config directory"), "got: {failure}");
    }

    #[test]
    fn lxqt_unreadable_file_names_the_file() {
        let dir = PathBuf::from("/cfg");
        let failure = lxqt_icon_theme(Some(&dir), missing)
            .unwrap_err()
            .to_string();
        assert!(failure.contains("/cfg/lxqt/lxqt.conf"), "got: {failure}");
        assert!(failure.contains("unreadable"), "got: {failure}");
    }

    #[test]
    fn lxqt_without_the_key_says_the_key_is_absent() {
        let dir = PathBuf::from("/cfg");
        let failure = lxqt_icon_theme(Some(&dir), |_: &Path| Ok("theme=frost\n".to_string()))
            .unwrap_err()
            .to_string();
        assert!(failure.contains("/cfg/lxqt/lxqt.conf"), "got: {failure}");
        assert!(
            failure.contains("`icon_theme` is absent or empty"),
            "got: {failure}"
        );
    }

    #[test]
    fn lxqt_reads_the_icon_theme_key() {
        let dir = PathBuf::from("/cfg");
        assert_eq!(
            lxqt_icon_theme(Some(&dir), |_: &Path| Ok(
                "[General]\nicon_theme=Papirus\n".to_string()
            )),
            Ok("Papirus".to_string())
        );
    }

    // === Unknown desktop: KDE, then GNOME ===

    fn kde_failed() -> Result<String, SourceFailure> {
        Err(SourceFailure {
            source: KDE_SOURCE.to_string(),
            reason: "no file is readable".to_string(),
        })
    }

    #[test]
    fn unknown_desktop_error_names_both_attempts() {
        let error = unknown_desktop_icon_theme(kde_failed, || {
            gsettings_icon_theme_from("org.gnome.desktop.interface", not_found())
        })
        .unwrap_err()
        .to_error();
        let message = error.to_string();
        assert!(message.contains("kdeglobals"), "got: {message}");
        assert!(
            message.contains("gsettings get org.gnome.desktop.interface icon-theme"),
            "got: {message}"
        );
        assert!(
            matches!(
                error,
                crate::Error::ReaderFailed {
                    reader: crate::error::reader_names::ICON_THEME,
                    ..
                }
            ),
            "got: {error:?}"
        );
    }

    #[test]
    fn unknown_desktop_takes_kde_first() {
        let theme = unknown_desktop_icon_theme(
            || Ok("breeze".to_string()),
            || gsettings_icon_theme_from("org.gnome.desktop.interface", printed("'Adwaita'")),
        );
        assert_eq!(theme, Ok("breeze".to_string()));
    }

    #[test]
    fn unknown_desktop_takes_gnome_where_kde_fails() {
        let theme = unknown_desktop_icon_theme(kde_failed, || {
            gsettings_icon_theme_from("org.gnome.desktop.interface", printed("'Adwaita'"))
        });
        assert_eq!(theme, Ok("Adwaita".to_string()));
    }

    // === The refined desktop decides the source, as it decides the reader ===

    fn source_of(
        portal_backend: Option<crate::detect::LinuxDesktop>,
        kdeglobals: bool,
    ) -> IconThemeSource {
        icon_theme_source(crate::detect::refine_linux_desktop(
            crate::detect::LinuxDesktop::Unknown,
            portal_backend,
            || kdeglobals,
        ))
    }

    /// An unrecognised desktop with a KDE portal backend is read as KDE,
    /// as the reader reads it, `kdeglobals` or not.
    #[test]
    fn an_unknown_desktop_with_a_kde_portal_has_kdes_icon_theme() {
        let kde = Some(crate::detect::LinuxDesktop::Kde);
        assert_eq!(source_of(kde, false), IconThemeSource::Kde);
        assert_eq!(source_of(kde, true), IconThemeSource::Kde);
    }

    /// An unrecognised desktop with a GNOME portal backend is read from
    /// GNOME's gsettings, as the GNOME reader reads it, though a
    /// `kdeglobals` exists (M4-49).
    #[test]
    fn an_unknown_desktop_with_a_gnome_portal_has_gnomes_icon_theme() {
        let gnome = Some(crate::detect::LinuxDesktop::Gnome);
        let gsettings = IconThemeSource::Gsettings(GNOME_INTERFACE_SCHEMA);
        assert_eq!(source_of(gnome, true), gsettings);
        assert_eq!(source_of(gnome, false), gsettings);
    }

    /// Without a portal answer, `kdeglobals` makes it KDE; with neither, both
    /// are tried.
    #[test]
    fn an_unknown_desktop_without_a_portal_answer() {
        assert_eq!(source_of(None, true), IconThemeSource::Kde);
        assert_eq!(source_of(None, false), IconThemeSource::KdeThenGnome);
    }

    // === The public result ===

    #[test]
    fn system_icon_theme_is_a_name_or_the_reason_for_none() {
        match system_icon_theme() {
            Ok(theme) => assert!(!theme.is_empty(), "detection named an empty theme"),
            Err(e) => assert!(!e.to_string().is_empty(), "a failure gave no reason"),
        }
    }
}
