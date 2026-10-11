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

    let full_preset = Theme::preset(preset_name)?;

    // Reconstruct a Theme from the type-safe ReaderOutput for merge
    let reader_as_theme = reader_output.to_theme(&reader_name, reader_icon_set, &reader_layout);

    // Merge: the platform preset provides the defaults, reader output
    // provides live OS data on top.
    let mut merged = full_preset.clone();
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
    // when it does, the context takes it and no DPI detection runs again.
    // button_order + icon_theme are platform-detected as from_system()'s.
    let ctx = match font_dpi {
        Some(dpi) => crate::resolve::ResolutionContext::with_font_dpi(
            dpi,
            crate::model::icons::system_icon_theme(),
        ),
        None => crate::resolve::ResolutionContext::from_system(),
    };

    // Resolve icon_theme with three-tier precedence (per §G4 / doc 1 §20 Option C):
    //   Tier 1: per-variant override (`ThemeMode::defaults.icon_theme`)
    //   Tier 2: shared Theme-level value (`Theme::icon_theme`)
    //   Tier 3: runtime system detect (from `ctx.icon_theme`)
    // Must read before variants are consumed by unwrap_or_default().
    // None where no tier names a theme: the TOML states none and detection
    // failed (`ctx.icon_theme` is then None). No theme stands in for it.
    // Both variants are resolved, kept by variant (`SystemTheme::icon_theme_for`);
    // `icon_theme` is the active mode's copy.
    let icon_theme_of =
        |variant: &Option<crate::model::ThemeMode>| -> Option<std::borrow::Cow<'static, str>> {
            variant
                .as_ref()
                .and_then(|v| v.defaults.icon_theme.clone()) // tier 1: per-variant override
                .or_else(|| merged.icon_theme.clone()) // tier 2: Theme-level shared
                .or_else(|| ctx.icon_theme.clone()) // tier 3: pre-detected system
        };
    let light_icon_theme = icon_theme_of(&merged.light);
    let dark_icon_theme = icon_theme_of(&merged.dark);
    let icon_theme = match mode {
        crate::ColorMode::Light => light_icon_theme.clone(),
        crate::ColorMode::Dark => dark_icon_theme.clone(),
    };

    // Shared across variants; read before the variants are moved out of `merged`.
    let layout = merged.layout.clone();

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
        preset: preset_name.to_string(),
        icon_set,
        icon_theme,
        light_icon_theme,
        dark_icon_theme,
        layout,
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

/// Map a Linux desktop environment to its matching platform preset.
///
/// This is the single source of truth for the DE-to-preset mapping used
/// by [`from_system_inner()`] and [`platform_preset_name()`].
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

/// Map the current platform to its matching platform preset.
///
/// The preset is the merge base in the OS-first pipeline: the platform
/// reader's output is merged over it.
///
/// - macOS -> `PlatformPreset { name: "macos-monterey" }`
/// - Windows -> `PlatformPreset { name: "windows-11" }`
/// - Linux KDE -> `PlatformPreset { name: "kde-breeze" }`
/// - Linux other/GNOME -> `PlatformPreset { name: "adwaita" }`
/// - Unknown platform -> `PlatformPreset { name: "adwaita" }`
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
    #[cfg(target_os = "macos")]
    {
        PlatformPreset {
            name: "macos-monterey",
        }
    }
    #[cfg(target_os = "windows")]
    {
        PlatformPreset { name: "windows-11" }
    }
    #[cfg(target_os = "linux")]
    {
        linux_preset_for_de(detect_linux_desktop())
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

/// Platform + DE + feature cascade that picks the appropriate
/// [`crate::reader::ThemeReader`] impl for the current system.
///
/// Returns `None` when no reader is available and the caller must fall back
/// to a preset-only pipeline (non-KDE/GNOME Linux, unsupported platforms, or
/// platforms whose reader feature is disabled). When `Some`, the companion
/// `&'static str` is the platform preset name that should be passed to
/// [`run_pipeline`] — the reader and the preset are tied because the preset
/// is chosen from the detected DE.
///
/// Per plan 94-03 §G8: the `Box<dyn ThemeReader>` trait object is object-safe
/// only because the trait carries a `#[async_trait::async_trait]` annotation
/// (see [`crate::reader`] module docs for the full rationale).
#[allow(unreachable_code)]
#[allow(clippy::unnecessary_wraps)] // Option is part of the public contract.
#[allow(clippy::needless_return)] // Explicit returns clarify cfg-gated dispatch.
async fn select_reader() -> Option<(Box<dyn crate::reader::ThemeReader>, &'static str)> {
    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        {
            return Some((Box::new(crate::macos::MacosReader), "macos-monterey"));
        }
        #[cfg(not(feature = "macos"))]
        return None;
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        {
            return Some((Box::new(crate::windows::WindowsReader), "windows-11"));
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
                return Some((Box::new(crate::gnome::GnomePortalKdeReader), "kde-breeze"));
            }
            #[cfg(all(feature = "kde", not(feature = "portal")))]
            LinuxDesktop::Kde => {
                return Some((Box::new(crate::kde::KdeReader), "kde-breeze"));
            }
            #[cfg(not(feature = "kde"))]
            LinuxDesktop::Kde => return None,
            #[cfg(feature = "portal")]
            LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
                return Some((Box::new(crate::gnome::GnomeReader), "adwaita"));
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
                            LinuxDesktop::Kde => {
                                Some((Box::new(crate::gnome::GnomePortalKdeReader), "kde-breeze"))
                            }
                            #[cfg(not(feature = "kde"))]
                            LinuxDesktop::Kde => None,
                            LinuxDesktop::Gnome => {
                                Some((Box::new(crate::gnome::GnomeReader), "adwaita"))
                            }
                            // detect_portal_backend only returns Kde or Gnome;
                            // any future extension falls back to Adwaita preset.
                            _ => None,
                        };
                    }
                }
                // Sync fallback: try kdeglobals, then Adwaita
                #[cfg(feature = "kde")]
                {
                    if crate::kde::kdeglobals_path().exists() {
                        return Some((Box::new(crate::kde::KdeReader), "kde-breeze"));
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
    if let Some((reader, preset)) = select_reader().await {
        return read_through_pipeline(reader.as_ref(), preset, crate::detect::system()).await;
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
        let mode = if system_is_dark() {
            crate::ColorMode::Dark
        } else {
            crate::ColorMode::Light
        };
        let preset = linux_preset_for_de(detect_linux_desktop()).name;
        return run_pipeline(preset_as_reader("adwaita", mode)?, preset, mode);
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Err(crate::Error::PlatformUnsupported {
            platform: "unsupported",
        })
    }
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
        Some((reader, _preset)) => match reader.read().await {
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
                "kde-breeze.toml:9 names breeze for light (active mode {mode:?})"
            );
            assert_eq!(
                theme.icon_theme_for(crate::ColorMode::Dark),
                Some("breeze-dark"),
                "kde-breeze.toml:317 names breeze-dark for dark (active mode {mode:?})"
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
    /// `Option<(Box<dyn ThemeReader>, &'static str)>`.
    ///
    /// On each platform with an available backend we expect `Some((_, preset))`;
    /// on platforms without one (non-Linux/macOS/Windows, or Linux with no
    /// compiled-in backend, or Linux-other-DE without portal) we expect `None`.
    ///
    /// The companion `&'static str` is the platform preset name that `run_pipeline`
    /// should use — kept in the same return type because the preset is chosen
    /// from the DE detected by `select_reader` (KDE -> kde-breeze, GNOME ->
    /// adwaita, macOS -> macos-monterey, Windows -> windows-11).
    #[test]
    fn select_reader_returns_platform_specific_impl() {
        // Call select_reader and observe the Option. The structural invariant
        // is that the function exists, is callable from a sync context via
        // `pollster::block_on`, and returns the (reader, preset) tuple.
        let picked: Option<(Box<dyn crate::reader::ThemeReader>, &'static str)> =
            pollster::block_on(super::select_reader());

        // Every cfg branch borrows `picked` (via `&picked`) rather than moving
        // it, so the final `drop(picked)` consumes it uniformly on all
        // permutations -- including linux-without-kde, which has no branch.
        #[cfg(all(target_os = "macos", feature = "macos"))]
        {
            assert!(
                picked.is_some(),
                "select_reader must return Some on macOS+macos"
            );
            if let Some((_reader, preset)) = &picked {
                assert_eq!(*preset, "macos-monterey");
            }
        }
        #[cfg(all(target_os = "windows", feature = "windows"))]
        {
            assert!(
                picked.is_some(),
                "select_reader must return Some on windows+windows"
            );
            if let Some((_reader, preset)) = &picked {
                assert_eq!(*preset, "windows-11");
            }
        }
        #[cfg(all(target_os = "linux", feature = "kde", feature = "portal"))]
        {
            // On linux+kde+portal, select_reader returns Some(GnomePortalKdeReader, "kde-breeze")
            // whenever the active DE is KDE; for non-KDE DEs it may legitimately
            // return None (preset fallback). Assert the tuple shape regardless.
            if let Some((_reader, preset)) = &picked {
                assert!(
                    *preset == "kde-breeze" || *preset == "adwaita",
                    "select_reader on linux must return a known platform preset, got: {preset}"
                );
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
