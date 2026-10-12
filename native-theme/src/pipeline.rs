//! Theme pipeline: reader -> preset merge -> resolve -> validate.

use std::fmt;

#[cfg(target_os = "linux")]
use crate::detect::{LinuxDesktop, detect_linux_desktop, parse_linux_desktop};

use crate::model::Theme;
use crate::{OverlaySource, ReaderOutput, ReaderResult, SystemTheme};

/// Run the OS-first pipeline: merge reader output onto a platform
/// preset, resolve both light and dark variants, validate.
///
/// Accepts a [`ReaderResult`] containing the reader's variant data
/// and metadata (name, icon_set, layout, font_dpi, accessibility).
///
/// `ReaderOutput::Single` readers provide one variant; the pipeline
/// fills the other from the full platform preset. `ReaderOutput::Dual`
/// readers provide both; the pipeline uses both after merging. Both kinds
/// report which variant is OS-active through `is_dark`, which decides `mode`.
pub(crate) fn run_pipeline(
    reader: ReaderResult,
    preset_name: &str,
    mode: crate::ColorMode,
) -> crate::Result<SystemTheme> {
    let ReaderResult {
        output: reader_output,
        name: reader_name,
        icon_set: reader_icon_set,
        layout: reader_layout,
        font_dpi,
        accessibility,
    } = reader;

    // Build ONE resolution-time context for this theme-build. The reader
    // may have supplied an authoritative font_dpi (e.g. KDE forceFontDPI);
    // when it does, the context takes it and no DPI detection runs again.
    // button_order + icon_theme are platform-detected as from_system()'s.
    let context = match font_dpi {
        Some(dpi) => crate::resolve::ResolutionContext::with_font_dpi(
            dpi,
            crate::model::icons::system_icon_theme(),
        ),
        None => crate::resolve::ResolutionContext::from_system(),
    };

    let source = OverlaySource {
        reader_output,
        name: reader_name,
        icon_set: reader_icon_set,
        layout: reader_layout,
        preset_name: preset_name.to_string(),
        context,
        overlay: None,
    };
    replay(source, mode, accessibility)
}

/// Merge and resolve what `source` holds: the platform preset provides the
/// defaults, the reader output live OS data on top, the app's overlays (if
/// any) their changes over both. Shared by [`run_pipeline`] and
/// [`SystemTheme::with_overlay`], which replays its source with one more
/// overlay.
pub(crate) fn replay(
    source: OverlaySource,
    mode: crate::ColorMode,
    accessibility: crate::AccessibilityPreferences,
) -> crate::Result<SystemTheme> {
    // A `Single` reader states no inactive variant, so the merge keeps the
    // preset's own there; a `Dual` reader's two variants go over the preset's.
    let mut merged = Theme::preset(&source.preset_name)?;
    merged.merge(
        &source
            .reader_output
            .to_theme(&source.name, source.icon_set, &source.layout),
    );
    if let Some(overlay) = &source.overlay {
        merged.merge(overlay);
    }

    // Keep reader name if non-empty, else use preset name
    let name = if source.name.is_empty() {
        merged.name.clone()
    } else {
        source.name.clone()
    };

    // Resolve icon_set from Theme level (shared across variants)
    let icon_set = merged
        .icon_set
        .unwrap_or_else(crate::model::icons::system_icon_set);

    // Resolve icon_theme with three-tier precedence (per §G4 / doc 1 §20 Option C):
    //   Tier 1: per-variant override (`ThemeMode::defaults.icon_theme`)
    //   Tier 2: shared Theme-level value (`Theme::icon_theme`)
    //   Tier 3: runtime system detect (from `context.icon_theme`)
    // Must read before variants are consumed by unwrap_or_default().
    // None where no tier names a theme: the TOML states none and detection
    // failed (`context.icon_theme` is then None). No theme stands in for it.
    // Both variants are resolved, kept by variant (`SystemTheme::icon_theme_for`);
    // `icon_theme` is the active mode's copy.
    let icon_theme_of =
        |variant: &Option<crate::model::ThemeMode>| -> Option<std::borrow::Cow<'static, str>> {
            variant
                .as_ref()
                .and_then(|v| v.defaults.icon_theme.clone()) // tier 1: per-variant override
                .or_else(|| merged.icon_theme.clone()) // tier 2: Theme-level shared
                .or_else(|| source.context.icon_theme.clone()) // tier 3: pre-detected system
        };
    let light_icon_theme = icon_theme_of(&merged.light);
    let dark_icon_theme = icon_theme_of(&merged.dark);
    let icon_theme = match mode {
        crate::ColorMode::Light => light_icon_theme.clone(),
        crate::ColorMode::Dark => dark_icon_theme.clone(),
    };

    // Shared across variants.
    crate::resolve::validate_helpers::check_layout(&merged.layout)?;

    let light = merged
        .light
        .unwrap_or_default()
        .into_resolved(&source.context)
        .map_err(|e| e.in_variant(crate::ColorMode::Light))?;
    let dark = merged
        .dark
        .unwrap_or_default()
        .into_resolved(&source.context)
        .map_err(|e| e.in_variant(crate::ColorMode::Dark))?;

    Ok(SystemTheme {
        name,
        mode,
        light,
        dark,
        preset: source.preset_name.clone(),
        overlay_source: source,
        version_match: None,
        icon_set,
        icon_theme,
        light_icon_theme,
        dark_icon_theme,
        layout: merged.layout,
        accessibility,
    })
}

// =============================================================================
// Structured return types
// =============================================================================

/// A single diagnostic observation about platform theme support.
///
/// Returned by [`diagnose_platform_support()`]. Each variant represents
/// a specific category of diagnostic information. Use `Display` to get
/// a human-readable string, or pattern-match for programmatic inspection.
///
/// For simple tabular output, use the [`name()`](Self::name),
/// [`status()`](Self::status), and [`detail()`](Self::detail) accessors:
///
/// ```
/// let diagnostics = native_theme::pipeline::diagnose_platform_support();
/// for entry in &diagnostics {
///     print!("{}: {}", entry.name(), entry.status());
///     if let Some(detail) = entry.detail() {
///         print!(" ({})", detail);
///     }
///     println!();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiagnosticEntry {
    /// Platform identification (e.g. "Linux", "macOS", "Windows").
    Platform(&'static str),
    /// Detected desktop environment (Linux only).
    #[cfg(target_os = "linux")]
    DesktopEnv(crate::detect::LinuxDesktop),
    /// An environment variable was read successfully.
    EnvVar {
        /// Variable name (e.g. `"XDG_CURRENT_DESKTOP"`).
        name: &'static str,
        /// Variable value as read from the environment.
        value: String,
    },
    /// An environment variable was missing or empty.
    EnvVarMissing(&'static str),
    /// An external tool was found and operational.
    ToolAvailable {
        /// Tool binary name (e.g. `"gsettings"`).
        name: &'static str,
        /// Version string reported by the tool.
        version: String,
    },
    /// An external tool was found but returned an error.
    ToolError(&'static str),
    /// An external tool was not found on PATH.
    ToolMissing {
        /// Tool binary name.
        name: &'static str,
        /// Human-readable description of what is lost.
        impact: &'static str,
    },
    /// A config file was found at the given path.
    ConfigFound {
        /// Logical config name (e.g. `"KDE kdeglobals"`).
        name: &'static str,
        /// Filesystem path where the file was found.
        path: std::path::PathBuf,
    },
    /// A config file was not found at the expected path.
    ConfigMissing {
        /// Logical config name.
        name: &'static str,
        /// Filesystem path that was checked.
        path: std::path::PathBuf,
    },
    /// A cargo feature is enabled.
    FeatureEnabled(&'static str),
    /// A cargo feature is disabled.
    FeatureDisabled {
        /// Feature name (e.g. `"KDE"`, `"Portal"`).
        feature: &'static str,
        /// Human-readable description of what is lost.
        impact: &'static str,
    },
    /// The running OS version and the preset of its generation (macOS and
    /// Windows, with their reader feature).
    OsGeneration {
        /// The running OS version: macOS `major.minor.patch`, Windows `10.0.build`.
        version: String,
        /// The preset of that version's generation.
        preset: &'static str,
        /// How the version relates to the preset's generation.
        version_match: VersionMatch,
    },
}

impl DiagnosticEntry {
    /// A short label identifying what is being diagnosed.
    ///
    /// Examples: `"Platform"`, `"XDG_CURRENT_DESKTOP"`, `"gsettings"`,
    /// `"KDE kdeglobals"`, `"Portal support"`.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Platform(_) => "Platform",
            #[cfg(target_os = "linux")]
            Self::DesktopEnv(_) => "Detected DE",
            Self::EnvVar { name, .. } | Self::EnvVarMissing(name) => name,
            Self::ToolAvailable { name, .. }
            | Self::ToolError(name)
            | Self::ToolMissing { name, .. } => name,
            Self::ConfigFound { name, .. } | Self::ConfigMissing { name, .. } => name,
            Self::FeatureEnabled(feature) | Self::FeatureDisabled { feature, .. } => feature,
            Self::OsGeneration { .. } => "OS version",
        }
    }

    /// A short status string: the detected value or a state like
    /// `"not set"`, `"available"`, `"found"`, `"enabled"`, etc.
    #[must_use]
    pub fn status(&self) -> &str {
        match self {
            Self::Platform(p) => p,
            #[cfg(target_os = "linux")]
            Self::DesktopEnv(_) => "detected",
            Self::EnvVar { value, .. } => value.as_str(),
            Self::EnvVarMissing(_) => "not set",
            Self::ToolAvailable { .. } => "available",
            Self::ToolError(_) => "found but returned error",
            Self::ToolMissing { .. } => "not found",
            Self::ConfigFound { .. } => "found",
            Self::ConfigMissing { .. } => "not found",
            Self::FeatureEnabled(_) => "enabled",
            Self::FeatureDisabled { .. } => "disabled",
            Self::OsGeneration { version, .. } => version.as_str(),
        }
    }

    /// Optional extra detail (version string, file path, impact note, DE variant).
    #[must_use]
    pub fn detail(&self) -> Option<String> {
        match self {
            #[cfg(target_os = "linux")]
            Self::DesktopEnv(de) => Some(format!("{de:?}")),
            Self::ToolAvailable { version, .. } => Some(version.clone()),
            Self::ToolMissing { impact, .. } => Some((*impact).to_string()),
            Self::FeatureDisabled { impact, .. } => Some((*impact).to_string()),
            Self::ConfigFound { path, .. } | Self::ConfigMissing { path, .. } => {
                Some(path.display().to_string())
            }
            Self::OsGeneration {
                preset,
                version_match,
                ..
            } => Some(format!("{preset}, {version_match}")),
            _ => None,
        }
    }
}

impl fmt::Display for DiagnosticEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Platform(p) => write!(f, "Platform: {p}"),
            #[cfg(target_os = "linux")]
            Self::DesktopEnv(de) => write!(f, "Detected DE: {de:?}"),
            Self::EnvVar { name, value } => write!(f, "{name}: {value}"),
            Self::EnvVarMissing(name) => write!(f, "{name}: not set"),
            Self::ToolAvailable { name, version } => {
                write!(f, "{name}: available ({version})")
            }
            Self::ToolError(name) => write!(f, "{name}: found but returned error"),
            Self::ToolMissing { name, impact } => write!(f, "{name}: not found ({impact})"),
            Self::ConfigFound { name, path } => {
                write!(f, "{name}: found at {}", path.display())
            }
            Self::ConfigMissing { name, path } => {
                write!(f, "{name}: not found at {}", path.display())
            }
            Self::FeatureEnabled(feature) => write!(f, "{feature} support: enabled"),
            Self::FeatureDisabled { feature, impact } => {
                write!(f, "{feature} support: disabled ({impact})")
            }
            Self::OsGeneration {
                version,
                preset,
                version_match,
            } => write!(f, "OS version: {version} ({preset}, {version_match})"),
        }
    }
}

/// Structured information about the platform's default preset.
///
/// Returned by [`platform_preset_name()`]. The `name` field is the
/// user-facing preset name (e.g. `"macos-monterey"`), the preset the OS-first
/// pipeline merges the platform reader's output over.
///
/// `Display` returns the user-facing name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlatformPreset {
    /// User-facing preset name (e.g. "kde-breeze", "adwaita", "macos-monterey").
    pub name: &'static str,
}

impl fmt::Display for PlatformPreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

// =============================================================================
// OS generations (M1-T1-01)
// =============================================================================

/// How the running OS version relates to the generation of the preset the
/// pipeline merged the platform reader's output over
/// ([`SystemTheme::version_match`](crate::SystemTheme::version_match)).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VersionMatch {
    /// The running version lies between the first version of the preset's
    /// generation and the newest version its values were verified on.
    Exact,
    /// The running OS is newer than any version the preset was verified on:
    /// the preset may carry an older look than the OS draws (macOS 27 under
    /// `macos-tahoe`).
    PresetOlder,
    /// The running OS predates the preset's generation: the preset carries a
    /// newer look than the OS draws (Windows 10 under `windows-11`).
    PresetNewer,
}

impl VersionMatch {
    /// A short label: `"exact"`, `"preset older than the OS"` or
    /// `"preset newer than the OS"`.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::PresetOlder => "preset older than the OS",
            Self::PresetNewer => "preset newer than the OS",
        }
    }
}

impl fmt::Display for VersionMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An OS version as the generation tables compare it: macOS
/// `major.minor.patch`, Windows `10.0.build`.
#[cfg_attr(
    not(any(
        all(target_os = "macos", feature = "macos"),
        all(target_os = "windows", feature = "windows")
    )),
    allow(dead_code)
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct OsVersion {
    pub(crate) major: u32,
    pub(crate) minor: u32,
    pub(crate) build: u32,
}

impl OsVersion {
    #[cfg_attr(
        not(any(
            all(target_os = "macos", feature = "macos"),
            all(target_os = "windows", feature = "windows")
        )),
        allow(dead_code)
    )]
    pub(crate) const fn new(major: u32, minor: u32, build: u32) -> Self {
        Self {
            major,
            minor,
            build,
        }
    }
}

impl fmt::Display for OsVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.build)
    }
}

/// One visual generation of a platform: the preset that carries its look,
/// the first OS version of that look, and the newest version its values were
/// verified on.
#[cfg_attr(
    not(any(
        all(target_os = "macos", feature = "macos"),
        all(target_os = "windows", feature = "windows")
    )),
    allow(dead_code)
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Generation {
    pub(crate) preset: &'static str,
    pub(crate) first: OsVersion,
    pub(crate) last_verified: OsVersion,
}

/// A platform's generations, oldest first; never empty, `oldest` being a field.
#[cfg_attr(
    not(any(
        all(target_os = "macos", feature = "macos"),
        all(target_os = "windows", feature = "windows")
    )),
    allow(dead_code)
)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct GenerationTable {
    pub(crate) oldest: Generation,
    /// The newer generations, in ascending order of `first`.
    pub(crate) newer: &'static [Generation],
    /// Every row's preset, oldest first: the names
    /// [`SystemTheme::from_system_with_preset`](crate::SystemTheme::from_system_with_preset)
    /// accepts on the platform.
    pub(crate) keys: &'static [&'static str],
}

#[cfg_attr(
    not(any(
        all(target_os = "macos", feature = "macos"),
        all(target_os = "windows", feature = "windows")
    )),
    allow(dead_code)
)]
impl GenerationTable {
    /// The rows, oldest first.
    fn rows(&self) -> impl Iterator<Item = Generation> + '_ {
        std::iter::once(self.oldest).chain(self.newer.iter().copied())
    }

    /// The row whose preset is `preset`.
    fn row(&self, preset: &str) -> Option<Generation> {
        self.rows().find(|g| g.preset == preset)
    }
}

/// macOS. `macos-monterey`: the look from macOS 12, whose AppKit release notes
/// end the accent highlight of a clicked button (M2a Step 7, plan D8), verified
/// on 14.8.9 and 15.7.9 by the OS probe; `macos-tahoe`: Liquid Glass from 26
/// (M2a Step 1's version map), verified on 26.6.2 (`docs/evidence/os-probe/
/// 2026-10-11-run-38129819806`). Apple's generations change at a major version,
/// so [`macos_generation`] compares majors. macOS 27 refines Liquid Glass and
/// gets no row until a preset is verified on it (plan D4): it maps to
/// `macos-tahoe`, reported as [`VersionMatch::PresetOlder`].
///
/// Major version 16: early macOS 26 betas reported 16 to a binary built
/// against the macOS 15 SDK ("When built against the 15 SDK or earlier, Tahoe
/// returns 16 for compatibility with previous numbering and all existing
/// apps", <https://eclecticlight.co/2025/06/13/is-tahoe-really-macos-26-or-16/>,
/// 2025-06-13). From beta 3, `NSOperatingSystemVersion` reports 26 even for
/// such a build (that page's comments, 2025-07-28); Apple Developer Forums
/// thread 791906 reports the same for the `kern.osproductversion` sysctl
/// (<https://developer.apple.com/forums/thread/791906>). No
/// source covers what the released OS reports. If a major version 16 is ever
/// read, this table gives `macos-monterey` with [`VersionMatch::PresetOlder`]
/// (no OS probe verified 16), which `generation_tests` pins.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
pub(crate) const MACOS_GENERATIONS: GenerationTable = GenerationTable {
    oldest: Generation {
        preset: "macos-monterey",
        first: OsVersion::new(12, 0, 0),
        last_verified: OsVersion::new(15, 0, 0),
    },
    newer: &[Generation {
        preset: "macos-tahoe",
        first: OsVersion::new(26, 0, 0),
        last_verified: OsVersion::new(26, 0, 0),
    }],
    keys: &["macos-monterey", "macos-tahoe"],
};

/// Windows. `windows-11`: the Fluent / WinUI 3 look from build 22000 (21H2;
/// M2b Step 1's version map), verified on build 26200 (25H2, the OS probe's
/// `windows-11-arm` image). Windows 10 has no preset of its own (M2b Step 7,
/// plan D9: a documented limit): it gets `windows-11`, reported as
/// [`VersionMatch::PresetNewer`].
#[cfg_attr(not(all(target_os = "windows", feature = "windows")), allow(dead_code))]
pub(crate) const WINDOWS_GENERATIONS: GenerationTable = GenerationTable {
    oldest: Generation {
        preset: "windows-11",
        first: OsVersion::new(10, 0, 22000),
        last_verified: OsVersion::new(10, 0, 26200),
    },
    newer: &[],
    keys: &["windows-11"],
};

/// How `running` relates to `row`'s generation.
#[cfg_attr(
    not(any(
        all(target_os = "macos", feature = "macos"),
        all(target_os = "windows", feature = "windows")
    )),
    allow(dead_code)
)]
pub(crate) fn version_match(row: &Generation, running: OsVersion) -> VersionMatch {
    if running < row.first {
        VersionMatch::PresetNewer
    } else if running > row.last_verified {
        VersionMatch::PresetOlder
    } else {
        VersionMatch::Exact
    }
}

/// The generation of `running` in `table`: the newest row whose first version
/// is not newer than `running`, or the oldest row where `running` predates
/// them all; with how `running` relates to it.
#[cfg_attr(
    not(any(
        all(target_os = "macos", feature = "macos"),
        all(target_os = "windows", feature = "windows")
    )),
    allow(dead_code)
)]
pub(crate) fn preset_for_version(
    table: &GenerationTable,
    running: OsVersion,
) -> (PlatformPreset, VersionMatch) {
    let row = table
        .newer
        .iter()
        .rev()
        .find(|g| g.first <= running)
        .copied()
        .unwrap_or(table.oldest);
    (
        PlatformPreset { name: row.preset },
        version_match(&row, running),
    )
}

/// `running` reduced to its major version, the macOS table's granularity.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
pub(crate) fn macos_major(running: OsVersion) -> OsVersion {
    OsVersion::new(running.major, 0, 0)
}

/// The macOS generation of a running macOS version, by its major version.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
pub(crate) fn macos_generation(running: OsVersion) -> (PlatformPreset, VersionMatch) {
    preset_for_version(&MACOS_GENERATIONS, macos_major(running))
}

/// The Windows generation of a running Windows build.
#[cfg_attr(not(all(target_os = "windows", feature = "windows")), allow(dead_code))]
pub(crate) fn windows_generation(running: OsVersion) -> (PlatformPreset, VersionMatch) {
    preset_for_version(&WINDOWS_GENERATIONS, running)
}

/// Map a Linux desktop environment to its matching platform preset.
///
/// [`select_reader`] and [`platform_preset_name()`] both map a desktop
/// through it, after [`refine_linux_desktop`] on `Unknown`. For `Unknown`,
/// [`select_reader`] first asks the portal backend (async D-Bus, with the
/// `portal` feature), which [`platform_preset_name()`] cannot: with the
/// `kde` feature, a KDE portal backend and no `kdeglobals`, `from_system`
/// uses `kde-breeze` where [`platform_preset_name()`] names `adwaita`.
///
/// - KDE -> `PlatformPreset { name: "kde-breeze" }`
/// - All others (GNOME, XFCE, Cinnamon, MATE, LXQt, Budgie, Unknown)
///   -> `PlatformPreset { name: "adwaita" }`
#[cfg(target_os = "linux")]
pub(crate) fn linux_preset_for_de(de: LinuxDesktop) -> PlatformPreset {
    match de {
        LinuxDesktop::Kde => PlatformPreset { name: "kde-breeze" },
        _ => PlatformPreset { name: "adwaita" },
    }
}

/// `de`, with `Unknown` taken as KDE where a `kdeglobals` file exists (with
/// the `kde` feature): the synchronous refinement [`select_reader`] and
/// [`platform_preset_name()`] share.
#[cfg(target_os = "linux")]
fn refine_linux_desktop(de: LinuxDesktop) -> LinuxDesktop {
    #[cfg(feature = "kde")]
    {
        refine_unknown_desktop(de, || crate::kde::kdeglobals_path().exists())
    }
    #[cfg(not(feature = "kde"))]
    {
        de
    }
}

/// [`refine_linux_desktop`], with `kdeglobals_exists` asked only for `Unknown`.
#[cfg(all(target_os = "linux", feature = "kde"))]
fn refine_unknown_desktop(
    de: LinuxDesktop,
    kdeglobals_exists: impl FnOnce() -> bool,
) -> LinuxDesktop {
    match de {
        LinuxDesktop::Unknown if kdeglobals_exists() => LinuxDesktop::Kde,
        other => other,
    }
}

/// Map the current platform to its matching platform preset.
///
/// The preset is the merge base in the OS-first pipeline: the platform
/// reader's output is merged over it.
///
/// - macOS -> the running version's generation: `macos-monterey` on 12 to 15,
///   `macos-tahoe` on 26 and later; `macos-monterey` without the `macos`
///   feature, which reads the version
/// - Windows -> `PlatformPreset { name: "windows-11" }`, the one Windows generation
/// - Linux KDE -> `PlatformPreset { name: "kde-breeze" }`; an unrecognised
///   desktop with a `kdeglobals` file counts as KDE (with the `kde` feature)
/// - Linux other/GNOME -> `PlatformPreset { name: "adwaita" }`
/// - Unknown platform -> `PlatformPreset { name: "adwaita" }`
///
/// On an unrecognised Linux desktop `from_system` also asks the portal
/// backend over D-Bus (with the `portal` feature), which this synchronous
/// function does not: with the `kde` feature, a KDE portal backend and no
/// `kdeglobals`, `from_system` uses `kde-breeze` where this names `adwaita`.
///
/// Returns a [`PlatformPreset`] with the user-facing preset name. Showcase
/// UIs use `preset.name` to build the "default (...)" label.
///
/// # Examples
///
/// ```
/// let preset = native_theme::pipeline::platform_preset_name();
/// println!("Platform preset: {}", preset.name);
/// ```
#[must_use]
pub fn platform_preset_name() -> PlatformPreset {
    #[cfg(all(target_os = "macos", feature = "macos"))]
    {
        macos_generation(crate::macos::os_version()).0
    }
    // Without the reader feature the version cannot be read: the oldest
    // generation's preset.
    #[cfg(all(target_os = "macos", not(feature = "macos")))]
    {
        PlatformPreset {
            name: MACOS_GENERATIONS.oldest.preset,
        }
    }
    #[cfg(all(target_os = "windows", feature = "windows"))]
    {
        windows_generation(crate::windows::os_version()).0
    }
    #[cfg(all(target_os = "windows", not(feature = "windows")))]
    {
        PlatformPreset {
            name: WINDOWS_GENERATIONS.oldest.preset,
        }
    }
    #[cfg(target_os = "linux")]
    {
        linux_preset_for_de(refine_linux_desktop(detect_linux_desktop()))
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        PlatformPreset { name: "adwaita" }
    }
}

/// Check whether OS theme detection is available on this platform.
///
/// Returns a list of [`DiagnosticEntry`] values describing what detection
/// capabilities are available and what might be missing. Useful for
/// debugging theme detection failures in end-user applications.
///
/// Each entry can be printed directly via its `Display` impl, or
/// inspected programmatically via [`DiagnosticEntry::name()`],
/// [`DiagnosticEntry::status()`], and [`DiagnosticEntry::detail()`].
///
/// # Platform Behavior
///
/// - **Linux:** Reports detected desktop environment, `gsettings`
///   availability, `XDG_CURRENT_DESKTOP` value, and KDE config file
///   presence (when the `kde` feature is enabled).
/// - **macOS:** Reports whether the `macos` feature is enabled and, with it,
///   the running version and its generation's preset
///   ([`DiagnosticEntry::OsGeneration`]).
/// - **Windows:** Reports whether the `windows` feature is enabled and, with
///   it, the running version and its generation's preset
///   ([`DiagnosticEntry::OsGeneration`]).
/// - **Other:** Reports that no platform detection is available.
///
/// # Examples
///
/// ```
/// let diagnostics = native_theme::pipeline::diagnose_platform_support();
/// for entry in &diagnostics {
///     println!("{entry}");
/// }
/// ```
#[must_use]
pub fn diagnose_platform_support() -> Vec<DiagnosticEntry> {
    let mut diagnostics = Vec::new();

    #[cfg(target_os = "linux")]
    {
        diagnostics.push(DiagnosticEntry::Platform("Linux"));

        // Check XDG_CURRENT_DESKTOP
        match std::env::var("XDG_CURRENT_DESKTOP") {
            Ok(val) if !val.is_empty() => {
                let de = parse_linux_desktop(&val);
                diagnostics.push(DiagnosticEntry::EnvVar {
                    name: "XDG_CURRENT_DESKTOP",
                    value: val,
                });
                diagnostics.push(DiagnosticEntry::DesktopEnv(de));
            }
            _ => {
                diagnostics.push(DiagnosticEntry::EnvVarMissing("XDG_CURRENT_DESKTOP"));
                diagnostics.push(DiagnosticEntry::DesktopEnv(LinuxDesktop::Unknown));
            }
        }

        // Check gsettings availability
        match std::process::Command::new("gsettings")
            .arg("--version")
            .output()
        {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                diagnostics.push(DiagnosticEntry::ToolAvailable {
                    name: "gsettings",
                    version,
                });
            }
            Ok(_) => {
                diagnostics.push(DiagnosticEntry::ToolError("gsettings"));
            }
            Err(_) => {
                diagnostics.push(DiagnosticEntry::ToolMissing {
                    name: "gsettings",
                    impact: "dark mode and icon theme detection may be limited",
                });
            }
        }

        // Check KDE config files
        #[cfg(feature = "kde")]
        {
            let path = crate::kde::kdeglobals_path();
            if path.exists() {
                diagnostics.push(DiagnosticEntry::ConfigFound {
                    name: "KDE kdeglobals",
                    path,
                });
            } else {
                diagnostics.push(DiagnosticEntry::ConfigMissing {
                    name: "KDE kdeglobals",
                    path,
                });
            }
        }

        #[cfg(not(feature = "kde"))]
        {
            diagnostics.push(DiagnosticEntry::FeatureDisabled {
                feature: "KDE",
                impact: "kde feature not enabled",
            });
        }

        // Report portal feature status
        #[cfg(feature = "portal")]
        diagnostics.push(DiagnosticEntry::FeatureEnabled("Portal"));

        #[cfg(not(feature = "portal"))]
        diagnostics.push(DiagnosticEntry::FeatureDisabled {
            feature: "Portal",
            impact: "portal feature not enabled",
        });
    }

    #[cfg(target_os = "macos")]
    {
        diagnostics.push(DiagnosticEntry::Platform("macOS"));

        #[cfg(feature = "macos")]
        {
            diagnostics.push(DiagnosticEntry::FeatureEnabled("macOS theme detection"));
            let running = crate::macos::os_version();
            let (preset, version_match) = macos_generation(running);
            diagnostics.push(DiagnosticEntry::OsGeneration {
                version: running.to_string(),
                preset: preset.name,
                version_match,
            });
        }

        #[cfg(not(feature = "macos"))]
        diagnostics.push(DiagnosticEntry::FeatureDisabled {
            feature: "macOS theme detection",
            impact: "macos feature not enabled, using subprocess fallback",
        });
    }

    #[cfg(target_os = "windows")]
    {
        diagnostics.push(DiagnosticEntry::Platform("Windows"));

        #[cfg(feature = "windows")]
        {
            diagnostics.push(DiagnosticEntry::FeatureEnabled("Windows theme detection"));
            let running = crate::windows::os_version();
            let (preset, version_match) = windows_generation(running);
            diagnostics.push(DiagnosticEntry::OsGeneration {
                version: running.to_string(),
                preset: preset.name,
                version_match,
            });
        }

        #[cfg(not(feature = "windows"))]
        diagnostics.push(DiagnosticEntry::FeatureDisabled {
            feature: "Windows theme detection",
            impact: "windows feature not enabled",
        });
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        diagnostics.push(DiagnosticEntry::Platform(
            "unsupported (no native theme detection available)",
        ));
    }

    diagnostics
}

/// Build a `ReaderResult` from a preset (for fallback paths where no
/// platform reader is available).
#[cfg(any(target_os = "linux", test))]
fn preset_as_reader(preset_name: &str, mode: crate::ColorMode) -> crate::Result<ReaderResult> {
    let theme = Theme::preset(preset_name)?;
    let is_dark = mode == crate::ColorMode::Dark;
    let output = if is_dark {
        ReaderOutput::Single {
            mode: Box::new(theme.dark.unwrap_or_default()),
            is_dark: true,
        }
    } else {
        ReaderOutput::Single {
            mode: Box::new(theme.light.unwrap_or_default()),
            is_dark: false,
        }
    };
    Ok(ReaderResult {
        output,
        name: theme.name,
        icon_set: theme.icon_set,
        layout: theme.layout,
        font_dpi: None,
        accessibility: crate::AccessibilityPreferences::default(),
    })
}

/// The platform reader [`select_reader`] picked, the preset its output merges
/// over, and how the running OS version relates to that preset's generation.
pub(crate) struct SelectedReader {
    /// The platform reader.
    pub(crate) reader: Box<dyn crate::reader::ThemeReader>,
    /// The platform preset [`run_pipeline`] merges the reader's output over.
    pub(crate) preset: &'static str,
    /// `None` where the platform has no generation table (Linux).
    pub(crate) version_match: Option<VersionMatch>,
}

impl SelectedReader {
    /// A reader of a platform without a generation table (Linux).
    #[cfg_attr(
        not(all(target_os = "linux", any(feature = "kde", feature = "portal"))),
        allow(dead_code)
    )]
    fn unversioned(reader: Box<dyn crate::reader::ThemeReader>, preset: &'static str) -> Self {
        Self {
            reader,
            preset,
            version_match: None,
        }
    }
}

/// Platform + DE + feature cascade that picks the appropriate
/// [`crate::reader::ThemeReader`] impl for the current system.
///
/// Returns `None` when no reader is available and the caller must fall back
/// to a preset-only pipeline (non-KDE/GNOME Linux, unsupported platforms, or
/// platforms whose reader feature is disabled). When `Some`, the
/// [`SelectedReader`] names the platform preset to pass to [`run_pipeline`] —
/// the reader and the preset are tied because the preset is chosen from the
/// detected DE on Linux and from the running version's generation on macOS
/// and Windows ([`preset_for_version`]).
///
/// Per plan 94-03 §G8: the `Box<dyn ThemeReader>` trait object is object-safe
/// only because the trait carries a `#[async_trait::async_trait]` annotation
/// (see [`crate::reader`] module docs for the full rationale).
#[allow(unreachable_code)]
#[allow(clippy::unnecessary_wraps)] // Option is part of the public contract.
#[allow(clippy::needless_return)] // Explicit returns clarify cfg-gated dispatch.
async fn select_reader() -> Option<SelectedReader> {
    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        {
            let (preset, matched) = macos_generation(crate::macos::os_version());
            return Some(SelectedReader {
                reader: Box::new(crate::macos::MacosReader),
                preset: preset.name,
                version_match: Some(matched),
            });
        }
        #[cfg(not(feature = "macos"))]
        return None;
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        {
            let (preset, matched) = windows_generation(crate::windows::os_version());
            return Some(SelectedReader {
                reader: Box::new(crate::windows::WindowsReader),
                preset: preset.name,
                version_match: Some(matched),
            });
        }
        #[cfg(not(feature = "windows"))]
        return None;
    }

    #[cfg(target_os = "linux")]
    {
        let de = detect_linux_desktop();
        match de {
            #[cfg(all(feature = "kde", feature = "portal"))]
            LinuxDesktop::Kde => {
                return Some(SelectedReader::unversioned(
                    Box::new(crate::gnome::GnomePortalKdeReader),
                    linux_preset_for_de(de).name,
                ));
            }
            #[cfg(all(feature = "kde", not(feature = "portal")))]
            LinuxDesktop::Kde => {
                return Some(SelectedReader::unversioned(
                    Box::new(crate::kde::KdeReader),
                    linux_preset_for_de(de).name,
                ));
            }
            #[cfg(not(feature = "kde"))]
            LinuxDesktop::Kde => return None,
            #[cfg(feature = "portal")]
            LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
                return Some(SelectedReader::unversioned(
                    Box::new(crate::gnome::GnomeReader),
                    linux_preset_for_de(de).name,
                ));
            }
            #[cfg(not(feature = "portal"))]
            LinuxDesktop::Gnome | LinuxDesktop::Budgie => return None,
            LinuxDesktop::Xfce
            | LinuxDesktop::Cinnamon
            | LinuxDesktop::Mate
            | LinuxDesktop::LxQt
            | LinuxDesktop::Hyprland
            | LinuxDesktop::Sway
            | LinuxDesktop::River
            | LinuxDesktop::Niri
            | LinuxDesktop::Wayfire
            | LinuxDesktop::CosmicDe => return None,
            LinuxDesktop::Unknown => {
                // Refine heuristic via D-Bus portal backend detection
                #[cfg(feature = "portal")]
                {
                    if let Some(detected) = crate::gnome::detect_portal_backend().await {
                        return match detected {
                            #[cfg(feature = "kde")]
                            LinuxDesktop::Kde => Some(SelectedReader::unversioned(
                                Box::new(crate::gnome::GnomePortalKdeReader),
                                linux_preset_for_de(detected).name,
                            )),
                            #[cfg(not(feature = "kde"))]
                            LinuxDesktop::Kde => None,
                            LinuxDesktop::Gnome => Some(SelectedReader::unversioned(
                                Box::new(crate::gnome::GnomeReader),
                                linux_preset_for_de(detected).name,
                            )),
                            // detect_portal_backend only returns Kde or Gnome;
                            // any future extension falls back to Adwaita preset.
                            _ => None,
                        };
                    }
                }
                // Sync fallback: try kdeglobals, then Adwaita
                #[cfg(feature = "kde")]
                {
                    let refined = refine_linux_desktop(de);
                    if refined == LinuxDesktop::Kde {
                        return Some(SelectedReader::unversioned(
                            Box::new(crate::kde::KdeReader),
                            linux_preset_for_de(refined).name,
                        ));
                    }
                }
                return None;
            }
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        None
    }
}

/// Single async implementation for all platforms. On Linux this may contain
/// `.await` points (portal D-Bus calls); on macOS/Windows the future resolves
/// immediately (no `.await` points).
///
/// Called by:
/// - `from_system()` via `pollster::block_on`.
/// - `from_system_async()` via `.await`.
///
/// Delegates platform detection to [`select_reader`]: when a reader is
/// available, its output feeds [`run_pipeline`]; when no reader is available,
/// falls back to a preset-only pipeline or returns a feature/platform error.
#[allow(unreachable_code)]
#[allow(clippy::needless_return)] // Explicit returns clarify cfg-gated fallback.
pub(crate) async fn from_system_inner() -> crate::Result<SystemTheme> {
    if let Some(selected) = select_reader().await {
        let mut theme = read_through_pipeline(
            selected.reader.as_ref(),
            selected.preset,
            crate::detect::system(),
        )
        .await?;
        theme.version_match = selected.version_match;
        return Ok(theme);
    }

    // Preset-only fallback paths — no reader available for this
    // platform+feature combination.
    #[cfg(target_os = "macos")]
    {
        return Err(crate::Error::FeatureDisabled {
            name: "macos",
            needed_for: "macOS theme detection",
        });
    }
    #[cfg(target_os = "windows")]
    {
        return Err(crate::Error::FeatureDisabled {
            name: "windows",
            needed_for: "Windows theme detection",
        });
    }
    #[cfg(target_os = "linux")]
    {
        return linux_fallback(platform_preset_name(), crate::detect::detect_is_dark());
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Err(crate::Error::PlatformUnsupported {
            platform: std::env::consts::OS,
        })
    }
}

/// The Linux theme where no reader is available: `preset` alone, as both
/// the reader's output and the base it merges over, in the mode `is_dark`
/// names. [`from_system_inner`] passes the uncached
/// [`crate::detect::detect_is_dark`], as the readers read the mode fresh.
#[cfg(target_os = "linux")]
fn linux_fallback(preset: PlatformPreset, is_dark: bool) -> crate::Result<SystemTheme> {
    let mode = if is_dark {
        crate::ColorMode::Dark
    } else {
        crate::ColorMode::Light
    };
    run_pipeline(preset_as_reader(preset.name, mode)?, preset.name, mode)
}

/// Run `reader` and feed its output through [`run_pipeline`] onto `preset`,
/// in the color mode the reader reports as OS-active. The reader's
/// accessibility block is stored in `detection`, so
/// [`crate::detect::accessibility_preferences`] need not run it again.
async fn read_through_pipeline(
    reader: &dyn crate::reader::ThemeReader,
    preset: &str,
    detection: &crate::detect::DetectionContext,
) -> crate::Result<SystemTheme> {
    let result = reader.read().await?;
    detection.store_reader_accessibility(result.accessibility.clone());
    let is_dark = match &result.output {
        ReaderOutput::Single { is_dark, .. } | ReaderOutput::Dual { is_dark, .. } => *is_dark,
    };
    let mode = if is_dark {
        crate::ColorMode::Dark
    } else {
        crate::ColorMode::Light
    };
    run_pipeline(result, preset, mode)
}

/// [`crate::SystemTheme::from_system_with_preset`]'s body: `preset` must be a
/// generation of the running platform's table (M1-T1-04).
#[allow(unreachable_code)]
#[allow(clippy::needless_return)]
// Explicit returns clarify cfg-gated dispatch.
// Without the reader feature the name is never read: the feature error comes first.
#[cfg_attr(
    any(
        all(target_os = "macos", not(feature = "macos")),
        all(target_os = "windows", not(feature = "windows"))
    ),
    allow(unused_variables)
)]
pub(crate) async fn from_system_with_preset_inner(preset: &str) -> crate::Result<SystemTheme> {
    #[cfg(all(target_os = "macos", feature = "macos"))]
    {
        return pinned_pipeline(
            &MACOS_GENERATIONS,
            macos_major(crate::macos::os_version()),
            preset,
            &crate::macos::MacosReader,
            crate::detect::system(),
        )
        .await;
    }
    #[cfg(all(target_os = "windows", feature = "windows"))]
    {
        return pinned_pipeline(
            &WINDOWS_GENERATIONS,
            crate::windows::os_version(),
            preset,
            &crate::windows::WindowsReader,
            crate::detect::system(),
        )
        .await;
    }
    #[cfg(all(target_os = "macos", not(feature = "macos")))]
    {
        return Err(crate::Error::FeatureDisabled {
            name: "macos",
            needed_for: "macOS theme detection",
        });
    }
    #[cfg(all(target_os = "windows", not(feature = "windows")))]
    {
        return Err(crate::Error::FeatureDisabled {
            name: "windows",
            needed_for: "Windows theme detection",
        });
    }
    // No generation table, so no OS generations to choose between: Linux,
    // whose readers read the installed style, BSD and other platforms.
    Err(crate::Error::UnknownPreset {
        name: preset.to_string(),
        known: &[],
    })
}

/// Run `reader` through the pipeline onto `preset`, a row of `table`, and
/// report that row's generation against `running`.
#[cfg_attr(
    not(any(
        all(target_os = "macos", feature = "macos"),
        all(target_os = "windows", feature = "windows")
    )),
    allow(dead_code)
)]
async fn pinned_pipeline(
    table: &GenerationTable,
    running: OsVersion,
    preset: &str,
    reader: &dyn crate::reader::ThemeReader,
    detection: &crate::detect::DetectionContext,
) -> crate::Result<SystemTheme> {
    let Some(row) = table.row(preset) else {
        return Err(crate::Error::UnknownPreset {
            name: preset.to_string(),
            known: table.keys,
        });
    };
    let mut theme = read_through_pipeline(reader, row.preset, detection).await?;
    theme.version_match = Some(version_match(&row, running));
    Ok(theme)
}

/// Reader-only extraction for [`crate::AccessibilityPreferences::from_system`]:
/// [`reader_accessibility`], with `reduce_motion` OR-ed with
/// [`crate::detect::detect_reduced_motion`], a floor for the cases where no
/// reader supplies it and it stays at its default: Linux outside KDE and
/// GNOME, and macOS or Windows builds without their reader feature. A reader's
/// own `true` is never downgraded (spec §11.2).
pub(crate) async fn accessibility_from_system_inner() -> crate::AccessibilityPreferences {
    let mut prefs = reader_accessibility().await;
    if !prefs.reduce_motion {
        prefs.reduce_motion = crate::detect::detect_reduced_motion();
    }
    prefs
}

/// Runs the same platform reader `from_system_inner` would run and returns its
/// accessibility block without merging or resolving a theme. Where no reader
/// is available (or it fails) the defaults are used.
pub(crate) async fn reader_accessibility() -> crate::AccessibilityPreferences {
    match select_reader().await {
        Some(selected) => match selected.reader.read().await {
            Ok(result) => result.accessibility,
            Err(_) => crate::AccessibilityPreferences::default(),
        },
        None => crate::AccessibilityPreferences::default(),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(all(test, target_os = "linux"))]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod dispatch_tests {
    use super::*;

    // -- parse_linux_desktop() pure function tests --

    #[test]
    fn detect_kde_simple() {
        assert_eq!(parse_linux_desktop("KDE"), LinuxDesktop::Kde);
    }

    #[test]
    fn detect_kde_colon_separated_after() {
        assert_eq!(parse_linux_desktop("ubuntu:KDE"), LinuxDesktop::Kde);
    }

    #[test]
    fn detect_kde_colon_separated_before() {
        assert_eq!(parse_linux_desktop("KDE:plasma"), LinuxDesktop::Kde);
    }

    #[test]
    fn detect_gnome_simple() {
        assert_eq!(parse_linux_desktop("GNOME"), LinuxDesktop::Gnome);
    }

    #[test]
    fn detect_gnome_ubuntu() {
        assert_eq!(parse_linux_desktop("ubuntu:GNOME"), LinuxDesktop::Gnome);
    }

    #[test]
    fn detect_xfce() {
        assert_eq!(parse_linux_desktop("XFCE"), LinuxDesktop::Xfce);
    }

    #[test]
    fn detect_cinnamon() {
        assert_eq!(parse_linux_desktop("X-Cinnamon"), LinuxDesktop::Cinnamon);
    }

    #[test]
    fn detect_cinnamon_short() {
        assert_eq!(parse_linux_desktop("Cinnamon"), LinuxDesktop::Cinnamon);
    }

    #[test]
    fn detect_mate() {
        assert_eq!(parse_linux_desktop("MATE"), LinuxDesktop::Mate);
    }

    #[test]
    fn detect_lxqt() {
        assert_eq!(parse_linux_desktop("LXQt"), LinuxDesktop::LxQt);
    }

    #[test]
    fn detect_budgie() {
        assert_eq!(parse_linux_desktop("Budgie:GNOME"), LinuxDesktop::Budgie);
    }

    #[test]
    fn detect_empty_string() {
        assert_eq!(parse_linux_desktop(""), LinuxDesktop::Unknown);
    }

    // -- Pure pipeline dispatch tests (no env var manipulation) --

    #[test]
    fn from_linux_non_kde_returns_adwaita() -> crate::Result<()> {
        // GNOME desktop produces an Adwaita-named theme via the pure pipeline
        let preset = linux_preset_for_de(LinuxDesktop::Gnome);
        let result = preset_as_reader("adwaita", crate::ColorMode::Light)?;
        let theme = run_pipeline(result, preset.name, crate::ColorMode::Light)?;
        assert_eq!(theme.name, "Adwaita");
        Ok(())
    }

    #[test]
    #[cfg(feature = "kde")]
    fn from_linux_unknown_de_with_kdeglobals_fallback() -> crate::Result<()> {
        // Unknown DE with a kdeglobals file uses KDE reader -- test the dispatch
        // branch by calling from_kde_content_pure directly with minimal fixture.
        const MINIMAL_KDE_FIXTURE: &str = "\
[General]
ColorScheme=TestTheme

[Colors:Window]
BackgroundNormal=239,240,241

[Colors:View]
BackgroundNormal=252,252,252
ForegroundNormal=35,38,41
DecorationFocus=61,174,233
BackgroundAlternate=239,240,241
ForegroundLink=41,128,185";

        let (reader_theme, dpi, acc) =
            crate::kde::from_kde_content_pure(MINIMAL_KDE_FIXTURE, None)?;
        let is_dark = reader_theme.dark.is_some() && reader_theme.light.is_none();
        let output = if is_dark {
            ReaderOutput::Single {
                mode: Box::new(reader_theme.dark.unwrap_or_default()),
                is_dark: true,
            }
        } else {
            ReaderOutput::Single {
                mode: Box::new(reader_theme.light.unwrap_or_default()),
                is_dark: false,
            }
        };
        let result = ReaderResult {
            output,
            name: reader_theme.name,
            icon_set: reader_theme.icon_set,
            layout: reader_theme.layout,
            font_dpi: dpi,
            accessibility: acc,
        };
        let preset = linux_preset_for_de(LinuxDesktop::Kde);
        let theme = run_pipeline(result, preset.name, crate::ColorMode::Light)?;
        assert_eq!(
            theme.name, "TestTheme",
            "should use KDE theme name from reader output"
        );
        Ok(())
    }

    #[test]
    fn from_linux_unknown_de_without_kdeglobals_returns_adwaita() -> crate::Result<()> {
        // Unknown DE without kdeglobals falls back to Adwaita preset
        let preset = linux_preset_for_de(LinuxDesktop::Unknown);
        let result = preset_as_reader("adwaita", crate::ColorMode::Light)?;
        let theme = run_pipeline(result, preset.name, crate::ColorMode::Light)?;
        assert_eq!(
            theme.name, "Adwaita",
            "should fall back to Adwaita without kdeglobals"
        );
        Ok(())
    }

    /// The preset's own theme: both variants straight from the preset.
    fn preset_theme(preset: &str) -> SystemTheme {
        let theme = Theme::preset(preset).unwrap();
        let reader = ReaderResult {
            output: ReaderOutput::Dual {
                light: Box::new(theme.light.unwrap_or_default()),
                dark: Box::new(theme.dark.unwrap_or_default()),
                is_dark: false,
            },
            name: theme.name,
            icon_set: theme.icon_set,
            layout: theme.layout,
            font_dpi: None,
            accessibility: crate::AccessibilityPreferences::default(),
        };
        run_pipeline(reader, preset, crate::ColorMode::Light).unwrap()
    }

    #[test]
    fn the_fallback_on_kde_is_the_whole_breeze_preset() {
        let expected = preset_theme("kde-breeze");
        for is_dark in [false, true] {
            let st = linux_fallback(linux_preset_for_de(LinuxDesktop::Kde), is_dark).unwrap();
            assert_eq!(st.preset, "kde-breeze");
            assert_eq!(st.name, expected.name, "is_dark {is_dark}");
            assert_eq!(st.light, expected.light, "is_dark {is_dark}");
            assert_eq!(st.dark, expected.dark, "is_dark {is_dark}");
            assert_eq!(
                st.icon_theme_for(crate::ColorMode::Light),
                expected.icon_theme_for(crate::ColorMode::Light)
            );
            assert_eq!(
                st.icon_theme_for(crate::ColorMode::Dark),
                expected.icon_theme_for(crate::ColorMode::Dark)
            );
        }
    }

    #[test]
    fn the_fallback_on_other_desktops_is_the_whole_adwaita_preset() {
        let expected = preset_theme("adwaita");
        let st = linux_fallback(linux_preset_for_de(LinuxDesktop::Xfce), true).unwrap();
        assert_eq!(st.preset, "adwaita");
        assert_eq!(st.name, expected.name);
        assert_eq!(st.light, expected.light);
        assert_eq!(st.dark, expected.dark);
    }

    #[test]
    fn the_fallback_mode_is_the_dark_reading_it_is_given() {
        let preset = || linux_preset_for_de(LinuxDesktop::Unknown);
        assert_eq!(
            linux_fallback(preset(), true).unwrap().mode,
            crate::ColorMode::Dark
        );
        assert_eq!(
            linux_fallback(preset(), false).unwrap().mode,
            crate::ColorMode::Light
        );
    }

    #[test]
    #[cfg(feature = "kde")]
    fn an_unknown_desktop_with_kdeglobals_maps_to_breeze() {
        let preset = |de, exists| linux_preset_for_de(refine_unknown_desktop(de, || exists)).name;
        assert_eq!(preset(LinuxDesktop::Unknown, true), "kde-breeze");
        assert_eq!(preset(LinuxDesktop::Unknown, false), "adwaita");
        assert_eq!(preset(LinuxDesktop::Kde, false), "kde-breeze");
        assert_eq!(preset(LinuxDesktop::Gnome, true), "adwaita");
        assert_eq!(preset(LinuxDesktop::Xfce, true), "adwaita");
    }

    // -- LNXDE-03: Hyprland, Sway, COSMIC, River, Niri map to their own variants --

    #[test]
    fn detect_hyprland() {
        assert_eq!(parse_linux_desktop("Hyprland"), LinuxDesktop::Hyprland);
    }

    #[test]
    fn detect_sway() {
        assert_eq!(parse_linux_desktop("sway"), LinuxDesktop::Sway);
    }

    #[test]
    fn detect_cosmic() {
        assert_eq!(parse_linux_desktop("COSMIC"), LinuxDesktop::CosmicDe);
    }

    #[test]
    fn detect_river() {
        assert_eq!(parse_linux_desktop("river"), LinuxDesktop::River);
    }

    #[test]
    fn detect_niri() {
        assert_eq!(parse_linux_desktop("niri"), LinuxDesktop::Niri);
    }

    #[test]
    fn parses_xdg_wayfire() {
        assert_eq!(parse_linux_desktop("Wayfire"), LinuxDesktop::Wayfire);
        assert_eq!(parse_linux_desktop("Wayfire:GNOME"), LinuxDesktop::Wayfire);
    }

    #[test]
    fn parses_xdg_empty_is_unknown() {
        // Regression guard -- empty XDG_CURRENT_DESKTOP must keep mapping to Unknown
        // even after the Wayfire variant addition.
        assert_eq!(parse_linux_desktop(""), LinuxDesktop::Unknown);
    }

    #[test]
    fn detect_cosmic_full_desktop() {
        assert_eq!(
            parse_linux_desktop("COSMIC:Freedesktop"),
            LinuxDesktop::CosmicDe
        );
    }

    // -- Pure pipeline smoke test (replaces from_system env var test) --

    #[test]
    fn from_system_returns_result() {
        // Test the pure pipeline directly instead of mocking env vars for from_system()
        let result = preset_as_reader("adwaita", crate::ColorMode::Light).unwrap();
        let theme = run_pipeline(result, "adwaita", crate::ColorMode::Light)
            .expect("run_pipeline should succeed with adwaita preset");
        assert_eq!(theme.name, "Adwaita");
    }
}

/// Tests for run_pipeline() -- internal pipeline functions.
/// These test functions moved from system_theme_tests in lib.rs since they
/// directly test pipeline internals rather than the SystemTheme public API.
#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::field_reassign_with_default
)]
mod pipeline_tests {
    use crate::color::Rgba;
    use crate::model::{LayoutTheme, Theme, ThemeMode};
    use crate::{ReaderOutput, ReaderResult};

    use super::{preset_as_reader, read_through_pipeline, run_pipeline};

    /// Helper: build a ReaderResult from a preset for testing.
    fn reader_from_preset(preset_name: &str) -> ReaderResult {
        let preset = Theme::preset(preset_name).unwrap();
        ReaderResult {
            output: ReaderOutput::Dual {
                light: Box::new(preset.light.clone().unwrap_or_default()),
                dark: Box::new(preset.dark.clone().unwrap_or_default()),
                is_dark: false,
            },
            name: preset.name,
            icon_set: preset.icon_set,
            layout: preset.layout,
            font_dpi: None,
            accessibility: crate::AccessibilityPreferences::default(),
        }
    }

    // --- run_pipeline() tests ---

    #[test]
    fn test_run_pipeline_produces_both_variants() {
        let reader = reader_from_preset("catppuccin-mocha");
        let result = run_pipeline(reader, "catppuccin-mocha", crate::ColorMode::Light);
        assert!(result.is_ok(), "run_pipeline should succeed");
        let st = result.unwrap();
        // Both light and dark exist as ResolvedTheme (non-Option)
        assert!(!st.name.is_empty(), "name should be populated");
        // If we get here, both variants validated successfully
    }

    #[test]
    fn test_run_pipeline_reader_values_win() {
        // Create a reader output where the reader provides a custom accent
        // (simulating a platform reader that detected this accent from the OS)
        let custom_accent = Rgba::rgb(42, 100, 200);
        let mut variant = ThemeMode::default();
        variant.defaults.accent_color = Some(custom_accent);

        let reader = ReaderResult {
            output: ReaderOutput::Single {
                mode: Box::new(variant),
                is_dark: false,
            },
            name: "CustomTheme".into(),
            icon_set: None,
            layout: LayoutTheme::default(),
            font_dpi: None,
            accessibility: crate::AccessibilityPreferences::default(),
        };

        let result = run_pipeline(reader, "catppuccin-mocha", crate::ColorMode::Light);
        assert!(result.is_ok(), "run_pipeline should succeed");
        let st = result.unwrap();
        // The reader's accent should win over the preset's accent
        assert_eq!(
            st.light.defaults.accent_color, custom_accent,
            "reader accent should win over preset accent"
        );
        assert_eq!(st.name, "CustomTheme", "reader name should win");
    }

    #[test]
    fn test_run_pipeline_single_variant() {
        // Simulate a real OS reader that provides a complete dark variant
        // (like KDE's from_kde() would) but no light variant.
        let full = Theme::preset("kde-breeze").unwrap();
        let mut dark_v = full.dark.clone().unwrap();
        // Override accent to prove reader values win (simulating OS-detected accent)
        dark_v.defaults.accent_color = Some(Rgba::rgb(200, 50, 50));

        let reader = ReaderResult {
            output: ReaderOutput::Single {
                mode: Box::new(dark_v),
                is_dark: true,
            },
            name: std::borrow::Cow::Borrowed(""),
            icon_set: None,
            layout: LayoutTheme::default(),
            font_dpi: None,
            accessibility: crate::AccessibilityPreferences::default(),
        };

        let result = run_pipeline(reader, "kde-breeze", crate::ColorMode::Dark);
        assert!(
            result.is_ok(),
            "run_pipeline should succeed with single variant"
        );
        let st = result.unwrap();
        // Dark should have the reader's overridden accent
        assert_eq!(
            st.dark.defaults.accent_color,
            Rgba::rgb(200, 50, 50),
            "dark variant should have reader accent"
        );
        // Light should still exist (from full preset, which has colors)
        // If we get here, both variants validated successfully
        assert_eq!(st.preset, "kde-breeze");
    }

    #[test]
    fn test_run_pipeline_inactive_variant_from_full_preset() {
        // When reader provides only dark, light must come from the full preset.
        let full = Theme::preset("kde-breeze").unwrap();

        let reader = ReaderResult {
            output: ReaderOutput::Single {
                mode: Box::new(full.dark.clone().unwrap_or_default()),
                is_dark: true,
            },
            name: std::borrow::Cow::Borrowed(""),
            icon_set: None,
            layout: LayoutTheme::default(),
            font_dpi: None,
            accessibility: crate::AccessibilityPreferences::default(),
        };

        let st = run_pipeline(reader, "kde-breeze", crate::ColorMode::Dark).unwrap();

        // The light variant should have colors from the full "kde-breeze" preset
        let full_light = full.light.unwrap();
        assert_eq!(
            st.light.defaults.accent_color,
            full_light.defaults.accent_color.unwrap(),
            "inactive light variant should get accent from full preset"
        );
        assert_eq!(
            st.light.defaults.background_color,
            full_light.defaults.background_color.unwrap(),
            "inactive light variant should get background from full preset"
        );
    }

    // --- run_pipeline with preset-as-reader (GNOME double-merge test) ---

    #[test]
    fn test_run_pipeline_with_preset_as_reader() {
        // Simulates GNOME sync fallback: adwaita used as both reader and preset.
        // Double-merge is harmless: merge is idempotent for matching values.
        let reader = reader_from_preset("adwaita");
        let result = run_pipeline(reader, "adwaita", crate::ColorMode::Light);
        assert!(
            result.is_ok(),
            "double-merge with same preset should succeed"
        );
        let st = result.unwrap();
        assert_eq!(st.name, "Adwaita");
    }

    /// Overlays stack: a second `with_overlay` keeps what the first one
    /// stated (M3-10).
    #[test]
    fn with_overlay_keeps_an_earlier_overlay() -> crate::Result<()> {
        let st = run_pipeline(
            reader_from_preset("adwaita"),
            "adwaita",
            crate::ColorMode::Light,
        )?;
        let a = Theme::from_toml("[light.defaults]\naccent_color = \"#ff0000\"")?;
        let b = Theme::from_toml("[light.defaults]\nbackground_color = \"#00ff00\"")?;
        let both = st.with_overlay(&a)?.with_overlay(&b)?;
        assert_eq!(both.light.defaults.accent_color, Rgba::rgb(255, 0, 0));
        assert_eq!(both.light.defaults.background_color, Rgba::rgb(0, 255, 0));
        Ok(())
    }

    // --- Single/Dual contract tests ---

    #[test]
    fn test_single_variant_fills_inactive_from_preset() {
        // Simulate KDE-style single-variant reader: only dark variant provided.
        // The pipeline should fill light from the full preset.
        let full = Theme::preset("kde-breeze").unwrap();
        let dark_v = full.dark.clone().unwrap();
        let reader = ReaderResult {
            output: ReaderOutput::Single {
                mode: Box::new(dark_v),
                is_dark: true,
            },
            name: "Breeze Dark".into(),
            icon_set: full.icon_set,
            layout: full.layout.clone(),
            font_dpi: None,
            accessibility: crate::AccessibilityPreferences::default(),
        };

        let st = run_pipeline(reader, "kde-breeze", crate::ColorMode::Dark).unwrap();

        // Dark variant should have reader's data (merged with preset).
        // Light variant should come from the full preset (kde-breeze).
        let full_light = Theme::preset("kde-breeze").unwrap().light.unwrap();
        assert_eq!(
            st.light.defaults.accent_color,
            full_light.defaults.accent_color.unwrap(),
            "Single: inactive light variant should come from full preset"
        );
        assert_eq!(st.mode, crate::ColorMode::Dark);
    }

    #[test]
    fn test_dual_variant_uses_both_from_reader() {
        // Simulate macOS-style dual-variant reader: both variants provided.
        // The pipeline should use BOTH reader variants (merged with preset).
        let full = Theme::preset("macos-monterey").unwrap();
        let custom_light_accent = Rgba::rgb(42, 100, 200);
        let custom_dark_accent = Rgba::rgb(200, 50, 50);

        let mut light_v = full.light.clone().unwrap();
        light_v.defaults.accent_color = Some(custom_light_accent);
        let mut dark_v = full.dark.clone().unwrap();
        dark_v.defaults.accent_color = Some(custom_dark_accent);

        let reader = ReaderResult {
            output: ReaderOutput::Dual {
                light: Box::new(light_v),
                dark: Box::new(dark_v),
                is_dark: false,
            },
            name: "macOS Sonoma".into(),
            icon_set: full.icon_set,
            layout: full.layout.clone(),
            font_dpi: Some(72.0),
            accessibility: crate::AccessibilityPreferences::default(),
        };

        let st = run_pipeline(reader, "macos-monterey", crate::ColorMode::Light).unwrap();

        // Both variants should reflect the reader's custom accents (merged).
        assert_eq!(
            st.light.defaults.accent_color, custom_light_accent,
            "Dual: light variant should have reader's light accent"
        );
        assert_eq!(
            st.dark.defaults.accent_color, custom_dark_accent,
            "Dual: dark variant should have reader's dark accent"
        );
    }

    /// A dual-variant reader in dark mode, standing in for the macOS reader
    /// on an OS set to dark: both variants from the preset, `is_dark` true.
    struct DarkDualReader;

    #[async_trait::async_trait]
    impl crate::reader::ThemeReader for DarkDualReader {
        async fn read(&self) -> crate::Result<ReaderResult> {
            let mut reader = reader_from_preset("macos-monterey");
            if let ReaderOutput::Dual { is_dark, .. } = &mut reader.output {
                *is_dark = true;
            }
            Ok(reader)
        }
    }

    #[test]
    fn dual_reader_mode_follows_is_dark() {
        let detection = crate::detect::DetectionContext::new();
        let st = pollster::block_on(read_through_pipeline(
            &DarkDualReader,
            "macos-monterey",
            &detection,
        ))
        .unwrap();
        assert_eq!(
            st.mode,
            crate::ColorMode::Dark,
            "a Dual reader reporting is_dark must give SystemTheme::mode Dark"
        );
    }

    #[test]
    fn a_pinned_older_generation_reports_preset_older() {
        let detection = crate::detect::DetectionContext::new();
        let st = pollster::block_on(super::pinned_pipeline(
            &super::MACOS_GENERATIONS,
            super::OsVersion::new(26, 0, 0),
            "macos-monterey",
            &DarkDualReader,
            &detection,
        ))
        .unwrap();
        assert_eq!(st.preset, "macos-monterey");
        assert_eq!(st.version_match, Some(super::VersionMatch::PresetOlder));
        // with_overlay keeps the match (M1-T1-01 change 2)
        let overlaid = st
            .with_overlay(&Theme::from_toml("name = \"overlay\"").unwrap())
            .unwrap();
        assert_eq!(overlaid.version_match, st.version_match);
    }

    #[test]
    fn a_pinned_name_outside_the_table_is_unknown() {
        let detection = crate::detect::DetectionContext::new();
        let result = pollster::block_on(super::pinned_pipeline(
            &super::MACOS_GENERATIONS,
            super::OsVersion::new(26, 0, 0),
            "kde-breeze",
            &DarkDualReader,
            &detection,
        ));
        assert!(matches!(
            result,
            Err(crate::Error::UnknownPreset { known, .. }) if known == super::MACOS_GENERATIONS.keys
        ));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_has_no_generation_to_pin() {
        let result = pollster::block_on(super::from_system_with_preset_inner("adwaita"));
        let err = result.err();
        assert!(matches!(
            err,
            Some(crate::Error::UnknownPreset { known: [], .. })
        ));
        assert!(
            err.is_some_and(|e| e
                .to_string()
                .contains("no OS generations to choose between")),
            "pinning a preset on Linux says the platform has no OS generations to choose between"
        );
    }

    /// A reader reporting a text-scaling factor of 1.5, standing in for any
    /// platform reader.
    struct ScaledTextReader;

    #[async_trait::async_trait]
    impl crate::reader::ThemeReader for ScaledTextReader {
        async fn read(&self) -> crate::Result<ReaderResult> {
            let mut reader = reader_from_preset("adwaita");
            reader.accessibility.text_scaling_factor = 1.5;
            Ok(reader)
        }
    }

    #[test]
    fn the_system_read_fills_the_accessibility_cache() {
        let detection = crate::detect::DetectionContext::new();
        let st = pollster::block_on(read_through_pipeline(
            &ScaledTextReader,
            "adwaita",
            &detection,
        ))
        .unwrap();
        let runs = std::cell::Cell::new(0);
        let cached = detection.reader_accessibility_with(|| {
            runs.set(runs.get() + 1);
            crate::AccessibilityPreferences::default()
        });
        assert_eq!(runs.get(), 0, "the cache was empty after the system read");
        assert_eq!(cached, st.accessibility);
        assert_eq!(cached.text_scaling_factor, 1.5);
    }

    // --- Phase 93-04 G4: Theme.icon_theme + three-tier precedence ---
    //
    // Per docs/archive/v0.5.7_gaps.md §G4 and doc 1 §20 Option C:
    //   Tier 1 (highest): ThemeMode::defaults.icon_theme (per-variant override)
    //   Tier 2:           Theme::icon_theme (shared across variants)
    //   Tier 3 (fallback): system_icon_theme() (runtime detect), none where it fails

    /// Helper: construct a ReaderResult that carries only the active variant
    /// (light) with no per-variant icon_theme. The pipeline must then consult
    /// the preset's Theme-level icon_theme (tier 2) or system detect (tier 3).
    fn reader_with_empty_variant(is_dark: bool) -> ReaderResult {
        ReaderResult {
            output: ReaderOutput::Single {
                mode: Box::new(ThemeMode::default()),
                is_dark,
            },
            name: std::borrow::Cow::Borrowed(""),
            icon_set: None,
            layout: LayoutTheme::default(),
            font_dpi: None,
            accessibility: crate::AccessibilityPreferences::default(),
        }
    }

    #[test]
    fn icon_theme_tier2_theme_level_used_when_per_variant_none() {
        // Adwaita preset (after G4 migration) carries icon_theme = "Adwaita"
        // at the Theme level, and None on both variant defaults.
        // With a reader that provides an empty variant (no per-variant override),
        // the resolver must fall through to tier 2 and produce "Adwaita".
        let reader = reader_with_empty_variant(false);
        let st = run_pipeline(reader, "adwaita", crate::ColorMode::Light).unwrap();
        assert_eq!(
            st.icon_theme.as_deref(),
            Some("Adwaita"),
            "tier 2: Theme.icon_theme should be used when per-variant is None"
        );
    }

    #[test]
    fn icon_theme_tier1_per_variant_wins_over_theme_level() {
        // Construct a reader whose active variant explicitly sets a per-variant
        // icon_theme override. Even with a preset that carries a Theme-level
        // icon_theme, the per-variant override must win.
        let mut variant = ThemeMode::default();
        variant.defaults.icon_theme = Some(std::borrow::Cow::Borrowed("custom-override"));
        let reader = ReaderResult {
            output: ReaderOutput::Single {
                mode: Box::new(variant),
                is_dark: false,
            },
            name: std::borrow::Cow::Borrowed(""),
            icon_set: None,
            layout: LayoutTheme::default(),
            font_dpi: None,
            accessibility: crate::AccessibilityPreferences::default(),
        };
        // Adwaita preset has Theme.icon_theme = "Adwaita" after migration.
        let st = run_pipeline(reader, "adwaita", crate::ColorMode::Light).unwrap();
        assert_eq!(
            st.icon_theme.as_deref(),
            Some("custom-override"),
            "tier 1: per-variant icon_theme override must win over Theme-level"
        );
    }

    /// `with_overlay` replays the overlay through the same tiers and layout
    /// as `run_pipeline`: an icon theme or a layout gap the overlay states
    /// wins, and an overlay that states none keeps the base's.
    #[test]
    fn with_overlay_applies_the_overlays_icon_theme_and_layout() -> crate::Result<()> {
        let st = run_pipeline(
            reader_from_preset("adwaita"),
            "adwaita",
            crate::ColorMode::Light,
        )?;
        let kept = st.with_overlay(&Theme::default())?;
        assert_eq!(kept.icon_theme, st.icon_theme);
        assert_eq!(kept.layout.widget_gap, st.layout.widget_gap);

        let overlay =
            Theme::from_toml("icon_theme = \"custom-override\"\n[layout]\nwidget_gap_px = 9.0")?;
        let changed = st.with_overlay(&overlay)?;
        assert_eq!(changed.icon_theme.as_deref(), Some("custom-override"));
        assert_eq!(
            changed.icon_theme_for(crate::ColorMode::Dark),
            Some("custom-override")
        );
        assert_eq!(changed.layout.widget_gap, Some(9.0));
        Ok(())
    }

    #[test]
    fn icon_theme_kde_per_variant_values_still_win() {
        // Regression guard for Phase 80-fix: KDE Breeze uses per-variant icon_theme
        // values ("breeze" light / "breeze-dark" dark) that MUST continue to win
        // over any Theme-level value. The kde-breeze preset keeps per-variant-only.
        let kde_full = Theme::preset("kde-breeze").unwrap();
        // Simulate KDE reader providing the dark variant with its per-variant value.
        let dark_v = kde_full.dark.clone().unwrap();
        assert_eq!(
            dark_v.defaults.icon_theme.as_deref(),
            Some("breeze-dark"),
            "precondition: kde-breeze dark variant must carry breeze-dark"
        );
        let reader = ReaderResult {
            output: ReaderOutput::Single {
                mode: Box::new(dark_v),
                is_dark: true,
            },
            name: std::borrow::Cow::Borrowed(""),
            icon_set: None,
            layout: LayoutTheme::default(),
            font_dpi: None,
            accessibility: crate::AccessibilityPreferences::default(),
        };
        let st = run_pipeline(reader, "kde-breeze", crate::ColorMode::Dark).unwrap();
        assert_eq!(
            st.icon_theme.as_deref(),
            Some("breeze-dark"),
            "KDE per-variant icon_theme must still win (Phase 80-fix invariant)"
        );
    }

    #[test]
    fn theme_icon_theme_round_trips_when_some() {
        // Serialize a Theme with icon_theme: Some(...) and assert the
        // deserialized round-trip preserves the value.
        let theme = Theme {
            name: std::borrow::Cow::Borrowed("RoundTrip"),
            light: Some(ThemeMode::default()),
            dark: None,
            layout: LayoutTheme::default(),
            icon_set: None,
            icon_theme: Some(std::borrow::Cow::Borrowed("lucide")),
        };
        let toml_str = theme.to_toml().expect("serialize");
        let reparsed = Theme::from_toml(&toml_str).expect("deserialize");
        assert_eq!(
            reparsed.icon_theme.as_deref(),
            Some("lucide"),
            "Theme.icon_theme must round-trip through TOML"
        );
        // Verify the serialized form contains the key at the top level.
        assert!(
            toml_str.contains("icon_theme = \"lucide\""),
            "serialized TOML should contain top-level icon_theme, got:\n{toml_str}"
        );
    }

    #[test]
    fn theme_icon_theme_skipped_when_none() {
        // Serialize a Theme with icon_theme: None and assert the TOML does NOT
        // emit the key (skip_serializing_if = Option::is_none).
        let theme = Theme {
            name: std::borrow::Cow::Borrowed("NoIcon"),
            light: Some(ThemeMode::default()),
            dark: None,
            layout: LayoutTheme::default(),
            icon_set: None,
            icon_theme: None,
        };
        let toml_str = theme.to_toml().expect("serialize");
        assert!(
            !toml_str.contains("icon_theme"),
            "serialized TOML should NOT contain icon_theme when None, got:\n{toml_str}"
        );
    }

    #[test]
    fn lint_toml_accepts_top_level_icon_theme() {
        // lint_toml() must NOT warn about icon_theme at the top level.
        // Raw string uses ##"..."## because the TOML body contains `"#` hex colors.
        let toml_str = r##"
name = "Test"
icon_theme = "lucide"

[light.defaults]
accent_color = "#0066cc"
"##;
        let warnings = Theme::lint_toml(toml_str).expect("lint");
        assert!(
            !warnings.iter().any(|w| w.contains("icon_theme")),
            "lint_toml must not flag top-level icon_theme as unknown; warnings: {warnings:?}"
        );
    }

    /// §11.1: `SystemTheme.layout` is the reader's layout merged field-wise over
    /// the preset's. On the preset-only path the reader *is* the full preset, so
    /// the result equals full ⊕ full.
    #[test]
    fn system_theme_layout_is_the_merged_preset_layout() -> crate::Result<()> {
        let reader = preset_as_reader("adwaita", crate::ColorMode::Light)?;
        let sys = run_pipeline(reader, "adwaita", crate::ColorMode::Light)?;

        let mut expected = Theme::preset("adwaita")?.layout;
        expected.merge(&Theme::preset("adwaita")?.layout);

        assert_eq!(sys.layout, expected);
        assert!(
            sys.layout.widget_gap.is_some(),
            "adwaita defines all four layout keys (spec §1.3)"
        );
        Ok(())
    }

    /// A layout value out of range fails the build, in `run_pipeline` and in
    /// `with_overlay`. Layout is shared by both variants: the path has no
    /// variant table.
    #[test]
    fn system_theme_layout_out_of_range_is_invalid() -> crate::Result<()> {
        let mut reader = preset_as_reader("adwaita", crate::ColorMode::Light)?;
        reader.layout.widget_gap = Some(-8.0);
        let Err(crate::Error::ResolutionInvalid { errors }) =
            run_pipeline(reader, "adwaita", crate::ColorMode::Light)
        else {
            panic!("expected ResolutionInvalid");
        };
        let paths: Vec<&str> = errors.iter().map(|rv| rv.path.as_str()).collect();
        assert_eq!(paths, ["layout.widget_gap_px"]);

        let reader = preset_as_reader("adwaita", crate::ColorMode::Light)?;
        let mut sys = run_pipeline(reader, "adwaita", crate::ColorMode::Light)?;
        sys.overlay_source.layout.section_gap = Some(f32::NAN);
        let Err(crate::Error::ResolutionInvalid { errors }) = sys.with_overlay(&Theme::default())
        else {
            panic!("expected ResolutionInvalid");
        };
        let paths: Vec<&str> = errors.iter().map(|rv| rv.path.as_str()).collect();
        assert_eq!(paths, ["layout.section_gap_px"]);
        Ok(())
    }

    /// Both variants' icon themes are known whichever mode is active: the
    /// egui connector needs the name per `egui::Theme`, because egui draws
    /// either style after a scheme change (egui spec §4.2, §9).
    #[test]
    fn icon_theme_for_gives_each_variants_name() -> crate::Result<()> {
        for mode in [crate::ColorMode::Light, crate::ColorMode::Dark] {
            // A reader that names no icon theme: tier 1 (the reader's own
            // variant) is empty, so the preset's variants decide.
            let reader = ReaderResult {
                output: ReaderOutput::Single {
                    mode: Box::new(crate::model::ThemeMode::default()),
                    is_dark: mode == crate::ColorMode::Dark,
                },
                name: std::borrow::Cow::Borrowed(""),
                icon_set: None,
                layout: crate::theme::LayoutTheme::default(),
                font_dpi: None,
                accessibility: crate::AccessibilityPreferences::default(),
            };
            let theme = run_pipeline(reader, "kde-breeze", mode)?;
            assert_eq!(
                theme.icon_theme_for(crate::ColorMode::Light),
                Some("breeze"),
                "kde-breeze.toml:16 names breeze for light (active mode {mode:?})"
            );
            assert_eq!(
                theme.icon_theme_for(crate::ColorMode::Dark),
                Some("breeze-dark"),
                "kde-breeze.toml:410 names breeze-dark for dark (active mode {mode:?})"
            );
            assert_eq!(
                theme.icon_theme_for(mode),
                theme.icon_theme.as_deref(),
                "the active mode's name is `icon_theme` itself"
            );
            // `mode` is a public field: changing it after the build must not swap the names.
            let mut flipped = theme;
            flipped.mode = match mode {
                crate::ColorMode::Light => crate::ColorMode::Dark,
                crate::ColorMode::Dark => crate::ColorMode::Light,
            };
            assert_eq!(
                flipped.icon_theme_for(crate::ColorMode::Light),
                Some("breeze"),
                "built in {mode:?}, mode flipped"
            );
            assert_eq!(
                flipped.icon_theme_for(crate::ColorMode::Dark),
                Some("breeze-dark"),
                "built in {mode:?}, mode flipped"
            );
        }
        Ok(())
    }
}

// =============================================================================
// Plan 94-03 (G8): ThemeReader trait + select_reader regression tests
// =============================================================================
//
// These tests lock the contract of the G8 ThemeReader refactor:
// - the `ThemeReader` trait exists under `crate::reader` and is object-safe
// - `select_reader()` exists and returns the correct platform-specific impl
//
// Object-safety is the load-bearing property: `select_reader()` is declared
// to return `Option<Box<dyn ThemeReader>>`, which only compiles if the trait
// is object-safe. On current stable Rust (1.95), native `async fn` in traits
// is NOT object-safe for `dyn Trait`; the trait definition must use
// `#[async_trait::async_trait]` to produce a `Pin<Box<dyn Future + Send>>`
// return type that vtables can hold. If an implementer deviates from the
// async-trait strategy (e.g., tries native async-fn-in-trait), the coercion
// below fails to compile with "the trait cannot be made into an object" —
// this is the structural enforcement of Option A (see plan 94-03 objective).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod theme_reader_trait_tests {
    /// Trait object-safety probe. With `#[async_trait::async_trait]` on the
    /// trait definition, this coercion compiles because the macro rewrite
    /// produces `Pin<Box<dyn Future + Send + '_>>` return types
    /// (vtable-compatible).
    #[test]
    fn theme_reader_trait_exists_and_is_object_safe() {
        // Plain function that requires its argument to be ?Sized — compile-time
        // proof that `Box<dyn ThemeReader>` is a valid trait object.
        //
        // `#[allow(dead_code)]` is required because on cfg permutations that
        // exclude every concrete reader (e.g. linux without kde feature) the
        // function is only name-resolved by the compiler but never called.
        #[allow(dead_code)]
        fn assert_object_safe<T: ?Sized>(_: &T) {}

        // Use one of the concrete readers to construct the trait object on
        // the platforms where one exists.
        #[cfg(all(target_os = "linux", feature = "kde"))]
        {
            let r: Box<dyn crate::reader::ThemeReader> = Box::new(crate::kde::KdeReader);
            assert_object_safe(&r);
        }
        #[cfg(all(target_os = "macos", feature = "macos"))]
        {
            let r: Box<dyn crate::reader::ThemeReader> = Box::new(crate::macos::MacosReader);
            assert_object_safe(&r);
        }
        #[cfg(all(target_os = "windows", feature = "windows"))]
        {
            let r: Box<dyn crate::reader::ThemeReader> = Box::new(crate::windows::WindowsReader);
            assert_object_safe(&r);
        }
        // Without any active-platform feature we still want the test body to
        // compile; reference the trait path once so name resolution fires
        // regardless of cfg.
        let _: Option<&dyn crate::reader::ThemeReader> = None;
    }

    /// `select_reader()` exists, is async, and returns an
    /// `Option<SelectedReader>`.
    ///
    /// On each platform with an available backend we expect `Some(selected)`;
    /// on platforms without one (non-Linux/macOS/Windows, or Linux with no
    /// compiled-in backend, or Linux-other-DE without portal) we expect `None`.
    ///
    /// The selected reader carries the platform preset name that `run_pipeline`
    /// should use — kept in the same return type because the preset is chosen
    /// from the DE detected by `select_reader` (KDE -> kde-breeze, GNOME ->
    /// adwaita, macOS -> the running version's generation, Windows -> windows-11).
    #[test]
    fn select_reader_returns_platform_specific_impl() {
        // Call select_reader and observe the Option. The structural invariant
        // is that the function exists, is callable from a sync context via
        // `pollster::block_on`, and returns the selected reader and its preset.
        let picked: Option<super::SelectedReader> = pollster::block_on(super::select_reader());

        // Every cfg branch borrows `picked` (via `&picked`) rather than moving
        // it, so the final `drop(picked)` consumes it uniformly on all
        // permutations -- including linux-without-kde, which has no branch.
        #[cfg(all(target_os = "macos", feature = "macos"))]
        {
            assert!(
                picked.is_some(),
                "select_reader must return Some on macOS+macos"
            );
            if let Some(selected) = &picked {
                let (preset, matched) = super::macos_generation(crate::macos::os_version());
                assert_eq!(selected.preset, preset.name);
                assert_eq!(selected.version_match, Some(matched));
            }
        }
        #[cfg(all(target_os = "windows", feature = "windows"))]
        {
            assert!(
                picked.is_some(),
                "select_reader must return Some on windows+windows"
            );
            if let Some(selected) = &picked {
                let (preset, matched) = super::windows_generation(crate::windows::os_version());
                assert_eq!(selected.preset, preset.name);
                assert_eq!(selected.version_match, Some(matched));
            }
        }
        #[cfg(all(target_os = "linux", feature = "kde", feature = "portal"))]
        {
            // On linux+kde+portal, select_reader returns Some(GnomePortalKdeReader, "kde-breeze")
            // whenever the active DE is KDE; for non-KDE DEs it may legitimately
            // return None (preset fallback). Assert the tuple shape regardless.
            if let Some(selected) = &picked {
                assert!(
                    selected.preset == "kde-breeze" || selected.preset == "adwaita",
                    "select_reader on linux must return a known platform preset, got: {}",
                    selected.preset
                );
                assert_eq!(selected.version_match, None);
            }
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            assert!(
                picked.is_none(),
                "select_reader() must return None on unsupported platforms"
            );
        }
        drop(picked);
    }
}

/// The generation tables (M1-T1-01): pure functions of the version, tested on
/// every platform; CI's macOS and Windows legs run the readers against them.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod generation_tests {
    use super::*;

    /// The macOS preset of a running major version, as `select_reader` picks it.
    fn macos_preset_for_version(major: u32) -> &'static str {
        macos_generation(OsVersion::new(major, 0, 0)).0.name
    }

    #[test]
    fn macos_preset_follows_the_os_generation() {
        assert_eq!(macos_preset_for_version(12), "macos-monterey");
        assert_eq!(macos_preset_for_version(14), "macos-monterey");
        assert_eq!(macos_preset_for_version(15), "macos-monterey");
        assert_eq!(macos_preset_for_version(26), "macos-tahoe");
        // macOS 27 until a macos-golden-gate preset exists (D4)
        assert_eq!(macos_preset_for_version(27), "macos-tahoe");
        assert_eq!(
            macos_generation(OsVersion::new(27, 0, 1)).1,
            VersionMatch::PresetOlder
        );
        // 16: what early macOS 26 betas reported to a macOS 15 SDK build
        assert_eq!(
            macos_generation(OsVersion::new(16, 0, 0)),
            (
                PlatformPreset {
                    name: "macos-monterey"
                },
                VersionMatch::PresetOlder
            )
        );
        // An unreadable version reads as 0.0.0
        assert_eq!(
            macos_generation(OsVersion::new(0, 0, 0)),
            (
                PlatformPreset {
                    name: "macos-monterey"
                },
                VersionMatch::PresetNewer
            )
        );
    }

    #[test]
    fn macos_versions_inside_a_generation_match_exactly() {
        for v in [
            OsVersion::new(12, 0, 0),
            OsVersion::new(14, 8, 9),
            OsVersion::new(15, 7, 9),
            OsVersion::new(26, 6, 2),
        ] {
            assert_eq!(macos_generation(v).1, VersionMatch::Exact, "{v}");
        }
        // Big Sur began the look, before Monterey's change to a clicked button
        assert_eq!(
            macos_generation(OsVersion::new(11, 7, 10)),
            (
                PlatformPreset {
                    name: "macos-monterey"
                },
                VersionMatch::PresetNewer
            )
        );
    }

    #[test]
    fn windows_preset_follows_the_build() {
        // Windows 10 22H2: no preset of its own (M2b Step 7, D9)
        assert_eq!(
            windows_generation(OsVersion::new(10, 0, 19045)),
            (
                PlatformPreset { name: "windows-11" },
                VersionMatch::PresetNewer
            )
        );
        assert_eq!(
            windows_generation(OsVersion::new(10, 0, 22000)).1,
            VersionMatch::Exact
        );
        assert_eq!(
            windows_generation(OsVersion::new(10, 0, 26200)).1,
            VersionMatch::Exact
        );
        // 26H2 (26300): newer than the build the preset was verified on
        assert_eq!(
            windows_generation(OsVersion::new(10, 0, 26300)).1,
            VersionMatch::PresetOlder
        );
        // 26H1 (28000)
        assert_eq!(
            windows_generation(OsVersion::new(10, 0, 28000)),
            (
                PlatformPreset { name: "windows-11" },
                VersionMatch::PresetOlder
            )
        );
        // A failed version read gives 0.0.0
        assert_eq!(
            windows_generation(OsVersion::new(0, 0, 0)),
            (
                PlatformPreset { name: "windows-11" },
                VersionMatch::PresetNewer
            )
        );
    }

    #[test]
    fn every_table_is_ordered_and_lists_its_keys() {
        for table in [MACOS_GENERATIONS, WINDOWS_GENERATIONS] {
            let rows: Vec<Generation> = table.rows().collect();
            let keys: Vec<&str> = rows.iter().map(|g| g.preset).collect();
            assert_eq!(keys, table.keys);
            for g in &rows {
                assert!(g.first <= g.last_verified, "{}", g.preset);
            }
            for (older, newer) in rows.iter().zip(rows.iter().skip(1)) {
                assert!(
                    older.last_verified < newer.first,
                    "{} / {}",
                    older.preset,
                    newer.preset
                );
            }
        }
    }

    #[test]
    fn every_generation_is_a_bundled_preset_of_its_platform() {
        for (table, platform) in [
            (MACOS_GENERATIONS, "macos"),
            (WINDOWS_GENERATIONS, "windows"),
        ] {
            for key in table.keys {
                assert!(crate::Theme::preset(key).is_ok(), "{key}");
                let offered = crate::Theme::list_presets()
                    .iter()
                    .any(|p| p.key == *key && p.platforms.contains(&platform));
                assert!(offered, "{key} is not offered on {platform}");
            }
        }
    }

    #[test]
    fn the_diagnostic_names_the_version_and_its_preset() {
        let entry = DiagnosticEntry::OsGeneration {
            version: OsVersion::new(27, 0, 1).to_string(),
            preset: "macos-tahoe",
            version_match: VersionMatch::PresetOlder,
        };
        assert_eq!(entry.name(), "OS version");
        assert_eq!(entry.status(), "27.0.1");
        assert_eq!(
            entry.to_string(),
            "OS version: 27.0.1 (macos-tahoe, preset older than the OS)"
        );
    }
}
