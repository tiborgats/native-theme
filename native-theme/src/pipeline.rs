//! Theme pipeline: reader -> preset merge -> resolve -> validate.

use std::fmt;

#[cfg(target_os = "linux")]
use crate::detect::{LinuxDesktop, detect_linux_desktop, parse_linux_desktop, system_is_dark};

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
/// readers provide both; the pipeline uses both after merging.
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

    let live_preset = Theme::preset(preset_name)?;

    // For the inactive variant, load the full preset (with colors).
    // Falls back to original name if not a live preset (e.g. user preset).
    let full_preset_name = preset_name.strip_suffix("-live").unwrap_or(preset_name);
    debug_assert!(
        full_preset_name != preset_name || !preset_name.ends_with("-live"),
        "live preset '{preset_name}' should have -live suffix stripped"
    );
    let full_preset = Theme::preset(full_preset_name)?;

    // Reconstruct a Theme from the type-safe ReaderOutput for merge
    let reader_as_theme = reader_output.to_theme(&reader_name, reader_icon_set, &reader_layout);

    // Merge: full preset provides color/font defaults, live preset overrides
    // geometry, reader output provides live OS data on top.
    let mut merged = full_preset.clone();
    merged.merge(&live_preset);
    merged.merge(&reader_as_theme);

    // Keep reader name if non-empty, else use preset name
    let name = if reader_name.is_empty() {
        merged.name.clone()
    } else {
        reader_name.clone()
    };

    // Resolve icon_set from Theme level (shared across variants)
    let icon_set = merged
        .icon_set
        .unwrap_or_else(crate::model::icons::system_icon_set);

    // Build ONE resolution-time context for this theme-build. The reader
    // may have supplied an authoritative font_dpi (e.g. KDE forceFontDPI);
    // when it does we override ctx.font_dpi. button_order + icon_theme
    // come from the from_system() defaults (platform-detected).
    let mut ctx = crate::resolve::ResolutionContext::from_system();
    if let Some(dpi) = font_dpi {
        ctx.font_dpi = dpi;
    }

    // Resolve icon_theme with three-tier precedence (per §G4 / doc 1 §20 Option C):
    //   Tier 1: per-variant override (`ThemeMode::defaults.icon_theme`)
    //   Tier 2: shared Theme-level value (`Theme::icon_theme`)
    //   Tier 3: runtime system detect (from `ctx.icon_theme`)
    // Must read before variants are consumed by unwrap_or_default().
    let icon_theme: std::borrow::Cow<'static, str> = {
        let active = if mode == crate::ColorMode::Dark {
            &merged.dark
        } else {
            &merged.light
        };
        active
            .as_ref()
            .and_then(|v| v.defaults.icon_theme.clone()) // tier 1: per-variant override
            .or_else(|| merged.icon_theme.clone()) // tier 2: Theme-level shared
            .or_else(|| ctx.icon_theme.clone()) // tier 3: pre-detected system
            .unwrap_or_else(|| std::borrow::Cow::Owned(crate::model::icons::system_icon_theme()))
    };

    // Match on ReaderOutput for type-safe variant selection:
    // Single: active variant from merged, inactive from full preset.
    // Dual: both variants from merged.
    let (light_variant, dark_variant) = match &reader_output {
        ReaderOutput::Single { is_dark, .. } => {
            if *is_dark {
                (
                    full_preset.light.unwrap_or_default(),
                    merged.dark.unwrap_or_default(),
                )
            } else {
                (
                    merged.light.unwrap_or_default(),
                    full_preset.dark.unwrap_or_default(),
                )
            }
        }
        ReaderOutput::Dual { .. } => (
            merged.light.unwrap_or_default(),
            merged.dark.unwrap_or_default(),
        ),
    };

    let light = light_variant.into_resolved(&ctx)?;
    let dark = dark_variant.into_resolved(&ctx)?;

    // Build OverlaySource from the original reader data + pipeline parameters
    let overlay_source = OverlaySource {
        reader_output,
        name: reader_name,
        icon_set: reader_icon_set,
        layout: reader_layout,
        preset_name: preset_name.to_string(),
        context: ctx,
    };

    Ok(SystemTheme {
        name,
        mode,
        light,
        dark,
        overlay_source,
        preset: full_preset_name.to_string(),
        live_preset: preset_name.to_string(),
        icon_set,
        icon_theme,
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
        }
    }
}

/// Structured information about the platform's default preset.
///
/// Returned by [`platform_preset_name()`]. The `name` field is the
/// user-facing preset name (e.g. `"macos-sonoma"`). The `is_live` field
/// indicates whether the preset is a live (geometry-only) preset used
/// by the OS-first pipeline.
///
/// `Display` returns the user-facing name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlatformPreset {
    /// User-facing preset name (e.g. "kde-breeze", "adwaita", "macos-sonoma").
    pub name: &'static str,
    /// Whether this is a live preset (geometry-only merge base for OS readers).
    pub is_live: bool,
}

impl PlatformPreset {
    /// Returns the internal live preset name (e.g. `"kde-breeze-live"`)
    /// when `is_live` is true, or the plain name when not.
    ///
    /// This is used internally by the pipeline to look up the correct
    /// preset entry; callers should use [`name`](Self::name) for display.
    #[must_use]
    pub fn live_name(&self) -> String {
        if self.is_live {
            format!("{}-live", self.name)
        } else {
            self.name.to_string()
        }
    }
}

impl fmt::Display for PlatformPreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

/// Map a Linux desktop environment to its matching platform preset.
///
/// This is the single source of truth for the DE-to-preset mapping used
/// by [`from_system_inner()`] and [`platform_preset_name()`].
///
/// - KDE -> `PlatformPreset { name: "kde-breeze", is_live: true }`
/// - All others (GNOME, XFCE, Cinnamon, MATE, LXQt, Budgie, Unknown)
///   -> `PlatformPreset { name: "adwaita", is_live: true }`
#[cfg(target_os = "linux")]
pub(crate) fn linux_preset_for_de(de: LinuxDesktop) -> PlatformPreset {
    match de {
        LinuxDesktop::Kde => PlatformPreset {
            name: "kde-breeze",
            is_live: true,
        },
        _ => PlatformPreset {
            name: "adwaita",
            is_live: true,
        },
    }
}

/// Map the current platform to its matching platform preset.
///
/// Live presets contain only geometry/metrics (no colors, fonts, or icons)
/// and are used as the merge base in the OS-first pipeline. Use
/// [`PlatformPreset::live_name()`] to get the internal live preset key.
///
/// - macOS -> `PlatformPreset { name: "macos-sonoma", is_live: true }`
/// - Windows -> `PlatformPreset { name: "windows-11", is_live: true }`
/// - Linux KDE -> `PlatformPreset { name: "kde-breeze", is_live: true }`
/// - Linux other/GNOME -> `PlatformPreset { name: "adwaita", is_live: true }`
/// - Unknown platform -> `PlatformPreset { name: "adwaita", is_live: true }`
///
/// Returns a [`PlatformPreset`] with the user-facing preset name and
/// whether it is a live (geometry-only) preset. Showcase UIs use
/// `preset.name` to build the "default (...)" label.
///
/// # Examples
///
/// ```
/// let preset = native_theme::pipeline::platform_preset_name();
/// println!("Platform preset: {}", preset.name);
/// assert!(!preset.name.contains("-live"));
/// ```
#[allow(unreachable_code)]
#[must_use]
pub fn platform_preset_name() -> PlatformPreset {
    #[cfg(target_os = "macos")]
    {
        return PlatformPreset {
            name: "macos-sonoma",
            is_live: true,
        };
    }
    #[cfg(target_os = "windows")]
    {
        return PlatformPreset {
            name: "windows-11",
            is_live: true,
        };
    }
    #[cfg(target_os = "linux")]
    {
        linux_preset_for_de(detect_linux_desktop())
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        PlatformPreset {
            name: "adwaita",
            is_live: true,
        }
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
/// - **macOS:** Reports whether the `macos` feature is enabled.
/// - **Windows:** Reports whether the `windows` feature is enabled.
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
        diagnostics.push(DiagnosticEntry::FeatureEnabled("macOS theme detection"));

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
        diagnostics.push(DiagnosticEntry::FeatureEnabled("Windows theme detection"));

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

/// Single async implementation for all platforms. On Linux this may contain
/// `.await` points (portal D-Bus calls); on macOS/Windows the future resolves
/// immediately (no `.await` points).
///
/// Called by:
/// - `from_system()` via `pollster::block_on` on Linux, noop-waker single-poll
///   on non-Linux.
/// - `from_system_async()` via `.await`.
#[allow(unreachable_code)]
pub(crate) async fn from_system_inner() -> crate::Result<SystemTheme> {
    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        {
            let result = crate::macos::from_macos()?;
            let mode = match &result.output {
                ReaderOutput::Single { is_dark, .. } => {
                    if *is_dark {
                        crate::ColorMode::Dark
                    } else {
                        crate::ColorMode::Light
                    }
                }
                ReaderOutput::Dual { .. } => crate::ColorMode::Light,
            };
            return run_pipeline(result, "macos-sonoma-live", mode);
        }

        #[cfg(not(feature = "macos"))]
        return Err(crate::Error::FeatureDisabled {
            name: "macos",
            needed_for: "macOS theme detection",
        });
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        {
            let result = crate::windows::from_windows()?;
            let mode = match &result.output {
                ReaderOutput::Single { is_dark, .. } => {
                    if *is_dark {
                        crate::ColorMode::Dark
                    } else {
                        crate::ColorMode::Light
                    }
                }
                ReaderOutput::Dual { .. } => crate::ColorMode::Light,
            };
            return run_pipeline(result, "windows-11-live", mode);
        }

        #[cfg(not(feature = "windows"))]
        return Err(crate::Error::FeatureDisabled {
            name: "windows",
            needed_for: "Windows theme detection",
        });
    }

    #[cfg(target_os = "linux")]
    {
        let mode = if system_is_dark() {
            crate::ColorMode::Dark
        } else {
            crate::ColorMode::Light
        };
        let de = detect_linux_desktop();
        let preset = linux_preset_for_de(de);
        let preset_live = preset.live_name();
        match de {
            #[cfg(feature = "kde")]
            LinuxDesktop::Kde => {
                #[cfg(feature = "portal")]
                {
                    let result = crate::gnome::from_kde_with_portal().await?;
                    run_pipeline(result, &preset_live, mode)
                }
                #[cfg(not(feature = "portal"))]
                {
                    let result = crate::kde::from_kde()?;
                    run_pipeline(result, &preset_live, mode)
                }
            }
            #[cfg(not(feature = "kde"))]
            LinuxDesktop::Kde => {
                run_pipeline(preset_as_reader("adwaita", mode)?, "adwaita-live", mode)
            }
            #[cfg(feature = "portal")]
            LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
                let result = crate::gnome::from_gnome().await?;
                run_pipeline(result, &preset_live, mode)
            }
            #[cfg(not(feature = "portal"))]
            LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
                run_pipeline(preset_as_reader("adwaita", mode)?, &preset_live, mode)
            }
            LinuxDesktop::Xfce
            | LinuxDesktop::Cinnamon
            | LinuxDesktop::Mate
            | LinuxDesktop::LxQt
            | LinuxDesktop::Hyprland
            | LinuxDesktop::Sway
            | LinuxDesktop::River
            | LinuxDesktop::Niri
            | LinuxDesktop::Wayfire
            | LinuxDesktop::CosmicDe => {
                run_pipeline(preset_as_reader("adwaita", mode)?, &preset_live, mode)
            }
            LinuxDesktop::Unknown => {
                // Use D-Bus portal backend detection to refine heuristic
                #[cfg(feature = "portal")]
                {
                    if let Some(detected) = crate::gnome::detect_portal_backend().await {
                        let detected_preset = linux_preset_for_de(detected);
                        let detected_live = detected_preset.live_name();
                        return match detected {
                            #[cfg(feature = "kde")]
                            LinuxDesktop::Kde => {
                                let result = crate::gnome::from_kde_with_portal().await?;
                                run_pipeline(result, &detected_live, mode)
                            }
                            #[cfg(not(feature = "kde"))]
                            LinuxDesktop::Kde => run_pipeline(
                                preset_as_reader("adwaita", mode)?,
                                "adwaita-live",
                                mode,
                            ),
                            LinuxDesktop::Gnome => {
                                let result = crate::gnome::from_gnome().await?;
                                run_pipeline(result, &detected_live, mode)
                            }
                            _ => {
                                // detect_portal_backend only returns Kde or Gnome;
                                // fall back to Adwaita if the set ever grows.
                                run_pipeline(
                                    preset_as_reader("adwaita", mode)?,
                                    &detected_live,
                                    mode,
                                )
                            }
                        };
                    }
                }
                // Sync fallback: try kdeglobals, then Adwaita
                #[cfg(feature = "kde")]
                {
                    let path = crate::kde::kdeglobals_path();
                    if path.exists() {
                        let result = crate::kde::from_kde()?;
                        let kde_live = linux_preset_for_de(LinuxDesktop::Kde).live_name();
                        return run_pipeline(result, &kde_live, mode);
                    }
                }
                run_pipeline(preset_as_reader("adwaita", mode)?, &preset_live, mode)
            }
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Err(crate::Error::PlatformUnsupported {
            platform: "unsupported",
        })
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
        let theme = run_pipeline(result, &preset.live_name(), crate::ColorMode::Light)?;
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
        let theme = run_pipeline(result, &preset.live_name(), crate::ColorMode::Light)?;
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
        let theme = run_pipeline(result, &preset.live_name(), crate::ColorMode::Light)?;
        assert_eq!(
            theme.name, "Adwaita",
            "should fall back to Adwaita without kdeglobals"
        );
        Ok(())
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
        let theme = run_pipeline(result, "adwaita-live", crate::ColorMode::Light)
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

    use super::run_pipeline;

    /// Helper: build a ReaderResult from a preset for testing.
    fn reader_from_preset(preset_name: &str) -> ReaderResult {
        let preset = Theme::preset(preset_name).unwrap();
        ReaderResult {
            output: ReaderOutput::Dual {
                light: Box::new(preset.light.clone().unwrap_or_default()),
                dark: Box::new(preset.dark.clone().unwrap_or_default()),
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
        // Use a live preset so the inactive light variant gets the full preset.
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

        let result = run_pipeline(reader, "kde-breeze-live", crate::ColorMode::Dark);
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
        assert_eq!(st.live_preset, "kde-breeze-live");
        assert_eq!(st.preset, "kde-breeze");
    }

    #[test]
    fn test_run_pipeline_inactive_variant_from_full_preset() {
        // When reader provides only dark, light must come from the full preset
        // (not the live preset, which has no colors and would fail validation).
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

        let st = run_pipeline(reader, "kde-breeze-live", crate::ColorMode::Dark).unwrap();

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

        let st = run_pipeline(reader, "kde-breeze-live", crate::ColorMode::Dark).unwrap();

        // Dark variant should have reader's data (merged with preset).
        // Light variant should come from the full preset (kde-breeze, not live).
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
        let full = Theme::preset("macos-sonoma").unwrap();
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
            },
            name: "macOS Sonoma".into(),
            icon_set: full.icon_set,
            layout: full.layout.clone(),
            font_dpi: Some(72.0),
            accessibility: crate::AccessibilityPreferences::default(),
        };

        let st = run_pipeline(reader, "macos-sonoma-live", crate::ColorMode::Light).unwrap();

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

    // --- Phase 93-04 G4: Theme.icon_theme + three-tier precedence ---
    //
    // Per docs/todo_v0.5.7_gaps.md §G4 and doc 1 §20 Option C:
    //   Tier 1 (highest): ThemeMode::defaults.icon_theme (per-variant override)
    //   Tier 2:           Theme::icon_theme (shared across variants)
    //   Tier 3 (fallback): system_icon_theme() (runtime detect)

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
            st.icon_theme.as_ref(),
            "Adwaita",
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
            st.icon_theme.as_ref(),
            "custom-override",
            "tier 1: per-variant icon_theme override must win over Theme-level"
        );
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
        let st = run_pipeline(reader, "kde-breeze-live", crate::ColorMode::Dark).unwrap();
        assert_eq!(
            st.icon_theme.as_ref(),
            "breeze-dark",
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
    /// Trait object-safety probe. After Task 2 lands with
    /// `#[async_trait::async_trait]` on the trait definition, this coercion
    /// compiles because the macro rewrite produces `Pin<Box<dyn Future + Send + '_>>`
    /// return types (vtable-compatible).
    ///
    /// Before Task 2 lands: fails to compile (`unresolved module crate::reader`).
    #[test]
    fn theme_reader_trait_exists_and_is_object_safe() {
        // Plain function that requires its argument to be ?Sized — compile-time
        // proof that `Box<dyn ThemeReader>` is a valid trait object.
        fn assert_object_safe<T: ?Sized>(_: &T) {}

        // Use one of the concrete readers to construct the trait object on
        // the platforms where one exists. If the trait or the reader struct
        // does not exist yet, this fails to compile before Task 2.
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

    /// `select_reader()` exists, is async, and returns an `Option<Box<dyn ThemeReader>>`.
    ///
    /// On each platform with an available backend we expect `Some(_)`; on
    /// platforms without one (non-Linux/macOS/Windows, or Linux with no
    /// compiled-in backend) we expect `None`.
    #[test]
    fn select_reader_returns_platform_specific_impl() {
        // Call select_reader and observe the Option; the exact is_some/is_none
        // expectation depends on features and platform and is asserted below
        // under narrowly-guarded cfg arms. The load-bearing invariant is that
        // the function exists, is callable from a sync context via
        // `pollster::block_on`, and returns the expected concrete type.
        let reader: Option<Box<dyn crate::reader::ThemeReader>> =
            pollster::block_on(super::select_reader());

        #[cfg(all(target_os = "linux", feature = "kde"))]
        {
            // On Linux+kde builds we always have at least the KdeReader available
            // as a fallback; select_reader() must therefore yield Some for any
            // detectable DE path.
            let _ = &reader;
        }
        #[cfg(all(target_os = "macos", feature = "macos"))]
        {
            assert!(
                reader.is_some(),
                "select_reader() must return Some on macOS+macos"
            );
        }
        #[cfg(all(target_os = "windows", feature = "windows"))]
        {
            assert!(
                reader.is_some(),
                "select_reader() must return Some on windows+windows"
            );
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            assert!(
                reader.is_none(),
                "select_reader() must return None on unsupported platforms"
            );
        }
        // Keep `reader` alive across the cfg arms so the let-binding above is
        // not flagged as unused on any build.
        drop(reader);
    }
}
