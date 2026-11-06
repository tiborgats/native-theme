//! Platform support diagnosis: the running OS version's generation, and
//! what detection can see.

use std::fmt;

#[cfg(target_os = "linux")]
use crate::detect::{LinuxDesktop, parse_linux_desktop};

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

// =============================================================================
// OS generations (M1-T1-01)
// =============================================================================

/// How the running OS version relates to the generation of its platform's
/// preset ([`DiagnosticEntry::OsGeneration`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
) -> (&'static str, VersionMatch) {
    let row = table
        .newer
        .iter()
        .rev()
        .find(|g| g.first <= running)
        .copied()
        .unwrap_or(table.oldest);
    (row.preset, version_match(&row, running))
}

/// `running` reduced to its major version, the macOS table's granularity.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
pub(crate) fn macos_major(running: OsVersion) -> OsVersion {
    OsVersion::new(running.major, 0, 0)
}

/// The macOS generation of a running macOS version, by its major version.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
pub(crate) fn macos_generation(running: OsVersion) -> (&'static str, VersionMatch) {
    preset_for_version(&MACOS_GENERATIONS, macos_major(running))
}

/// The Windows generation of a running Windows build.
#[cfg_attr(not(all(target_os = "windows", feature = "windows")), allow(dead_code))]
pub(crate) fn windows_generation(running: OsVersion) -> (&'static str, VersionMatch) {
    preset_for_version(&WINDOWS_GENERATIONS, running)
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
        match crate::detect::run_with_timeout(
            "gsettings",
            &["--version"],
            crate::detect::SUBPROCESS_TIMEOUT,
        ) {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                diagnostics.push(DiagnosticEntry::ToolAvailable {
                    name: "gsettings",
                    version,
                });
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                diagnostics.push(DiagnosticEntry::ToolMissing {
                    name: "gsettings",
                    impact: "dark mode and icon theme detection may be limited",
                });
            }
            Ok(_) | Err(_) => {
                diagnostics.push(DiagnosticEntry::ToolError("gsettings"));
            }
        }

        // Check KDE config files
        #[cfg(feature = "kde")]
        {
            match crate::kde::kdeglobals_path() {
                Some(path) if path.exists() => {
                    diagnostics.push(DiagnosticEntry::ConfigFound {
                        name: "KDE kdeglobals",
                        path,
                    });
                }
                Some(path) => {
                    diagnostics.push(DiagnosticEntry::ConfigMissing {
                        name: "KDE kdeglobals",
                        path,
                    });
                }
                None => diagnostics.push(DiagnosticEntry::EnvVarMissing("HOME")),
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
                preset,
                version_match,
            });
        }

        #[cfg(not(feature = "macos"))]
        diagnostics.push(DiagnosticEntry::FeatureDisabled {
            feature: "macOS theme detection",
            impact: "macos feature not enabled, the OS version is not read",
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
                preset,
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod generation_tests {
    use super::*;

    /// The macOS preset of a running major version.
    fn macos_preset_for_version(major: u32) -> &'static str {
        macos_generation(OsVersion::new(major, 0, 0)).0
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
            ("macos-monterey", VersionMatch::PresetOlder)
        );
        // An unreadable version reads as 0.0.0
        assert_eq!(
            macos_generation(OsVersion::new(0, 0, 0)),
            ("macos-monterey", VersionMatch::PresetNewer)
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
            ("macos-monterey", VersionMatch::PresetNewer)
        );
    }

    #[test]
    fn windows_preset_follows_the_build() {
        // Windows 10 22H2: no preset of its own (M2b Step 7, D9)
        assert_eq!(
            windows_generation(OsVersion::new(10, 0, 19045)),
            ("windows-11", VersionMatch::PresetNewer)
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
            ("windows-11", VersionMatch::PresetOlder)
        );
        // A failed version read gives 0.0.0
        assert_eq!(
            windows_generation(OsVersion::new(0, 0, 0)),
            ("windows-11", VersionMatch::PresetNewer)
        );
    }

    #[test]
    fn every_table_is_ordered() {
        for table in [MACOS_GENERATIONS, WINDOWS_GENERATIONS] {
            let rows: Vec<Generation> = std::iter::once(table.oldest)
                .chain(table.newer.iter().copied())
                .collect();
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
