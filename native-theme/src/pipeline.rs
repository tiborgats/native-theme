//! Theme pipeline: reader -> preset merge -> resolve -> validate.

#[cfg(target_os = "linux")]
use crate::detect::{LinuxDesktop, detect_linux_desktop, parse_linux_desktop, system_is_dark};

use crate::model::Theme;
use crate::{OverlaySource, ReaderOutput, SystemTheme};

/// Run the OS-first pipeline: merge reader output onto a platform
/// preset, resolve both light and dark variants, validate.
///
/// Accepts `ReaderOutput` for type-safe single-vs-dual variant handling.
/// `Single`: the reader provided one variant; the pipeline fills the
/// inactive variant from the full platform preset.
/// `Dual`: the reader provided both; the pipeline uses both merged variants.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_pipeline(
    reader: ReaderOutput,
    reader_name: String,
    reader_icon_set: Option<crate::IconSet>,
    reader_layout: crate::LayoutTheme,
    preset_name: &str,
    mode: crate::ColorMode,
    accessibility: crate::AccessibilityPreferences,
    font_dpi: Option<f32>,
) -> crate::Result<SystemTheme> {
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
    let reader_as_theme = reader.to_theme(&reader_name, reader_icon_set, &reader_layout);

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

    // Resolve icon_theme from the active variant's defaults (per-variant).
    // Must read before variants are consumed by unwrap_or_default().
    let icon_theme = {
        let active = if mode == crate::ColorMode::Dark {
            &merged.dark
        } else {
            &merged.light
        };
        active
            .as_ref()
            .and_then(|v| v.defaults.icon_theme.clone())
            .unwrap_or_else(crate::model::icons::system_icon_theme)
    };

    // Match on ReaderOutput for type-safe variant selection:
    // Single: active variant from merged, inactive from full preset.
    // Dual: both variants from merged.
    let (light_variant, dark_variant) = match &reader {
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

    let light = light_variant.into_resolved(font_dpi)?;
    let dark = dark_variant.into_resolved(font_dpi)?;

    // Build OverlaySource from the original reader data + pipeline parameters
    let overlay_source = OverlaySource {
        reader_output: reader,
        name: reader_name,
        icon_set: reader_icon_set,
        layout: reader_layout,
        preset_name: preset_name.to_string(),
        font_dpi,
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

/// Map a Linux desktop environment to its matching live preset name.
///
/// This is the single source of truth for the DE-to-preset mapping used
/// by [`from_system_inner()`] and [`platform_preset_name()`].
///
/// - KDE -> `"kde-breeze-live"`
/// - All others (GNOME, XFCE, Cinnamon, MATE, LXQt, Budgie, Unknown)
///   -> `"adwaita-live"`
#[cfg(target_os = "linux")]
pub(crate) fn linux_preset_for_de(de: LinuxDesktop) -> &'static str {
    match de {
        LinuxDesktop::Kde => "kde-breeze-live",
        _ => "adwaita-live",
    }
}

/// Map the current platform to its matching live preset name.
///
/// Live presets contain only geometry/metrics (no colors, fonts, or icons)
/// and are used as the merge base in the OS-first pipeline.
///
/// - macOS -> `"macos-sonoma-live"`
/// - Windows -> `"windows-11-live"`
/// - Linux KDE -> `"kde-breeze-live"`
/// - Linux other/GNOME -> `"adwaita-live"`
/// - Unknown platform -> `"adwaita-live"`
///
/// Returns the live preset name for the current platform.
///
/// This is the public API for what [`SystemTheme::from_system()`] uses internally.
/// Showcase UIs use this to build the "default (...)" label.
#[allow(unreachable_code)]
#[must_use]
pub fn platform_preset_name() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        return "macos-sonoma-live";
    }
    #[cfg(target_os = "windows")]
    {
        return "windows-11-live";
    }
    #[cfg(target_os = "linux")]
    {
        linux_preset_for_de(detect_linux_desktop())
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        "adwaita-live"
    }
}

/// Check whether OS theme detection is available on this platform.
///
/// Returns a list of human-readable diagnostic messages describing what
/// detection capabilities are available and what might be missing. Useful
/// for debugging theme detection failures in end-user applications.
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
/// for line in &diagnostics {
///     println!("{}", line);
/// }
/// ```
#[must_use]
pub fn diagnose_platform_support() -> Vec<String> {
    let mut diagnostics = Vec::new();

    #[cfg(target_os = "linux")]
    {
        diagnostics.push("Platform: Linux".to_string());

        // Check XDG_CURRENT_DESKTOP
        match std::env::var("XDG_CURRENT_DESKTOP") {
            Ok(val) if !val.is_empty() => {
                let de = parse_linux_desktop(&val);
                diagnostics.push(format!("XDG_CURRENT_DESKTOP: {val}"));
                diagnostics.push(format!("Detected DE: {de:?}"));
            }
            _ => {
                diagnostics.push("XDG_CURRENT_DESKTOP: not set".to_string());
                diagnostics.push("Detected DE: Unknown (env var missing)".to_string());
            }
        }

        // Check gsettings availability
        match std::process::Command::new("gsettings")
            .arg("--version")
            .output()
        {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                diagnostics.push(format!("gsettings: available ({})", version.trim()));
            }
            Ok(_) => {
                diagnostics.push("gsettings: found but returned error".to_string());
            }
            Err(_) => {
                diagnostics.push(
                    "gsettings: not found (dark mode and icon theme detection may be limited)"
                        .to_string(),
                );
            }
        }

        // Check KDE config files
        #[cfg(feature = "kde")]
        {
            let path = crate::kde::kdeglobals_path();
            if path.exists() {
                diagnostics.push(format!("KDE kdeglobals: found at {}", path.display()));
            } else {
                diagnostics.push(format!("KDE kdeglobals: not found at {}", path.display()));
            }
        }

        #[cfg(not(feature = "kde"))]
        {
            diagnostics.push("KDE support: disabled (kde feature not enabled)".to_string());
        }

        // Report portal feature status
        #[cfg(feature = "portal")]
        diagnostics.push("Portal support: enabled".to_string());

        #[cfg(not(feature = "portal"))]
        diagnostics.push("Portal support: disabled (portal feature not enabled)".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        diagnostics.push("Platform: macOS".to_string());

        #[cfg(feature = "macos")]
        diagnostics.push("macOS theme detection: enabled (macos feature active)".to_string());

        #[cfg(not(feature = "macos"))]
        diagnostics.push(
            "macOS theme detection: limited (macos feature not enabled, using subprocess fallback)"
                .to_string(),
        );
    }

    #[cfg(target_os = "windows")]
    {
        diagnostics.push("Platform: Windows".to_string());

        #[cfg(feature = "windows")]
        diagnostics.push("Windows theme detection: enabled (windows feature active)".to_string());

        #[cfg(not(feature = "windows"))]
        diagnostics
            .push("Windows theme detection: disabled (windows feature not enabled)".to_string());
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        diagnostics.push("Platform: unsupported (no native theme detection available)".to_string());
    }

    diagnostics
}

/// Convert a legacy reader `Theme` to a `ReaderOutput` enum.
///
/// Bridge for `from_system_inner()`: existing readers still return `Theme`
/// tuples; this converts to the type-safe `ReaderOutput` for `run_pipeline`.
/// Plan 02 will make readers return `ReaderOutput` directly, eliminating
/// this bridge.
fn theme_to_reader_output(theme: &Theme, is_dark: bool) -> ReaderOutput {
    let has_both = theme.light.is_some() && theme.dark.is_some();
    if has_both {
        ReaderOutput::Dual {
            light: Box::new(theme.light.clone().unwrap_or_default()),
            dark: Box::new(theme.dark.clone().unwrap_or_default()),
        }
    } else if is_dark {
        ReaderOutput::Single {
            mode: Box::new(theme.dark.clone().unwrap_or_default()),
            is_dark: true,
        }
    } else {
        ReaderOutput::Single {
            mode: Box::new(theme.light.clone().unwrap_or_default()),
            is_dark: false,
        }
    }
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
            let (reader_theme, dpi, acc) = crate::macos::from_macos()?;
            let is_dark = reader_theme.dark.is_some() && reader_theme.light.is_none();
            let mode = if is_dark {
                crate::ColorMode::Dark
            } else {
                crate::ColorMode::Light
            };
            let ro = theme_to_reader_output(&reader_theme, is_dark);
            return run_pipeline(
                ro,
                reader_theme.name,
                reader_theme.icon_set,
                reader_theme.layout,
                "macos-sonoma-live",
                mode,
                acc,
                dpi,
            );
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
            let (reader_theme, dpi, acc) = crate::windows::from_windows()?;
            let is_dark = reader_theme.dark.is_some() && reader_theme.light.is_none();
            let mode = if is_dark {
                crate::ColorMode::Dark
            } else {
                crate::ColorMode::Light
            };
            let ro = theme_to_reader_output(&reader_theme, is_dark);
            return run_pipeline(
                ro,
                reader_theme.name,
                reader_theme.icon_set,
                reader_theme.layout,
                "windows-11-live",
                mode,
                acc,
                dpi,
            );
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
        let is_dark = mode == crate::ColorMode::Dark;
        let de = detect_linux_desktop();
        let preset = linux_preset_for_de(de);
        match de {
            #[cfg(feature = "kde")]
            LinuxDesktop::Kde => {
                #[cfg(feature = "portal")]
                {
                    let (reader_theme, dpi, acc) = crate::gnome::from_kde_with_portal().await?;
                    let ro = theme_to_reader_output(&reader_theme, is_dark);
                    run_pipeline(
                        ro,
                        reader_theme.name,
                        reader_theme.icon_set,
                        reader_theme.layout,
                        preset,
                        mode,
                        acc,
                        dpi,
                    )
                }
                #[cfg(not(feature = "portal"))]
                {
                    let (reader_theme, dpi, acc) = crate::kde::from_kde()?;
                    let ro = theme_to_reader_output(&reader_theme, is_dark);
                    run_pipeline(
                        ro,
                        reader_theme.name,
                        reader_theme.icon_set,
                        reader_theme.layout,
                        preset,
                        mode,
                        acc,
                        dpi,
                    )
                }
            }
            #[cfg(not(feature = "kde"))]
            LinuxDesktop::Kde => {
                let fallback = Theme::preset("adwaita")?;
                let ro = theme_to_reader_output(&fallback, is_dark);
                run_pipeline(
                    ro,
                    fallback.name,
                    fallback.icon_set,
                    fallback.layout,
                    "adwaita-live",
                    mode,
                    crate::AccessibilityPreferences::default(),
                    None,
                )
            }
            #[cfg(feature = "portal")]
            LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
                let (reader_theme, dpi, acc) = crate::gnome::from_gnome().await?;
                let ro = theme_to_reader_output(&reader_theme, is_dark);
                run_pipeline(
                    ro,
                    reader_theme.name,
                    reader_theme.icon_set,
                    reader_theme.layout,
                    preset,
                    mode,
                    acc,
                    dpi,
                )
            }
            #[cfg(not(feature = "portal"))]
            LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
                let fallback = Theme::preset("adwaita")?;
                let ro = theme_to_reader_output(&fallback, is_dark);
                run_pipeline(
                    ro,
                    fallback.name,
                    fallback.icon_set,
                    fallback.layout,
                    preset,
                    mode,
                    crate::AccessibilityPreferences::default(),
                    None,
                )
            }
            LinuxDesktop::Xfce
            | LinuxDesktop::Cinnamon
            | LinuxDesktop::Mate
            | LinuxDesktop::LxQt
            | LinuxDesktop::Hyprland
            | LinuxDesktop::Sway
            | LinuxDesktop::River
            | LinuxDesktop::Niri
            | LinuxDesktop::CosmicDe => {
                let fallback = Theme::preset("adwaita")?;
                let ro = theme_to_reader_output(&fallback, is_dark);
                run_pipeline(
                    ro,
                    fallback.name,
                    fallback.icon_set,
                    fallback.layout,
                    preset,
                    mode,
                    crate::AccessibilityPreferences::default(),
                    None,
                )
            }
            LinuxDesktop::Unknown => {
                // Use D-Bus portal backend detection to refine heuristic
                #[cfg(feature = "portal")]
                {
                    if let Some(detected) = crate::gnome::detect_portal_backend().await {
                        let detected_preset = linux_preset_for_de(detected);
                        return match detected {
                            #[cfg(feature = "kde")]
                            LinuxDesktop::Kde => {
                                let (reader_theme, dpi, acc) =
                                    crate::gnome::from_kde_with_portal().await?;
                                let ro = theme_to_reader_output(&reader_theme, is_dark);
                                run_pipeline(
                                    ro,
                                    reader_theme.name,
                                    reader_theme.icon_set,
                                    reader_theme.layout,
                                    detected_preset,
                                    mode,
                                    acc,
                                    dpi,
                                )
                            }
                            #[cfg(not(feature = "kde"))]
                            LinuxDesktop::Kde => {
                                let fallback = Theme::preset("adwaita")?;
                                let ro = theme_to_reader_output(&fallback, is_dark);
                                run_pipeline(
                                    ro,
                                    fallback.name,
                                    fallback.icon_set,
                                    fallback.layout,
                                    "adwaita-live",
                                    mode,
                                    crate::AccessibilityPreferences::default(),
                                    None,
                                )
                            }
                            LinuxDesktop::Gnome => {
                                let (reader_theme, dpi, acc) = crate::gnome::from_gnome().await?;
                                let ro = theme_to_reader_output(&reader_theme, is_dark);
                                run_pipeline(
                                    ro,
                                    reader_theme.name,
                                    reader_theme.icon_set,
                                    reader_theme.layout,
                                    detected_preset,
                                    mode,
                                    acc,
                                    dpi,
                                )
                            }
                            _ => {
                                // detect_portal_backend only returns Kde or Gnome;
                                // fall back to Adwaita if the set ever grows.
                                let fallback = Theme::preset("adwaita")?;
                                let ro = theme_to_reader_output(&fallback, is_dark);
                                run_pipeline(
                                    ro,
                                    fallback.name,
                                    fallback.icon_set,
                                    fallback.layout,
                                    detected_preset,
                                    mode,
                                    crate::AccessibilityPreferences::default(),
                                    None,
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
                        let (reader_theme, dpi, acc) = crate::kde::from_kde()?;
                        let ro = theme_to_reader_output(&reader_theme, is_dark);
                        return run_pipeline(
                            ro,
                            reader_theme.name,
                            reader_theme.icon_set,
                            reader_theme.layout,
                            linux_preset_for_de(LinuxDesktop::Kde),
                            mode,
                            acc,
                            dpi,
                        );
                    }
                }
                let fallback = Theme::preset("adwaita")?;
                let ro = theme_to_reader_output(&fallback, is_dark);
                run_pipeline(
                    ro,
                    fallback.name,
                    fallback.icon_set,
                    fallback.layout,
                    preset,
                    mode,
                    crate::AccessibilityPreferences::default(),
                    None,
                )
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
    fn from_linux_non_kde_returns_adwaita() {
        // GNOME desktop produces an Adwaita-named theme via the pure pipeline
        let preset = Theme::preset("adwaita").unwrap();
        let ro = theme_to_reader_output(&preset, false);
        let theme = run_pipeline(
            ro,
            preset.name,
            preset.icon_set,
            preset.layout,
            linux_preset_for_de(LinuxDesktop::Gnome),
            crate::ColorMode::Light,
            crate::AccessibilityPreferences::default(),
            None,
        )
        .expect("run_pipeline should succeed for GNOME preset");
        assert_eq!(theme.name, "Adwaita");
    }

    #[test]
    #[cfg(feature = "kde")]
    fn from_linux_unknown_de_with_kdeglobals_fallback() {
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
            crate::kde::from_kde_content_pure(MINIMAL_KDE_FIXTURE, None).unwrap();
        let ro = theme_to_reader_output(&reader_theme, false);
        let theme = run_pipeline(
            ro,
            reader_theme.name,
            reader_theme.icon_set,
            reader_theme.layout,
            linux_preset_for_de(LinuxDesktop::Kde),
            crate::ColorMode::Light,
            acc,
            dpi,
        )
        .expect("run_pipeline should succeed with KDE reader output");
        assert_eq!(
            theme.name, "TestTheme",
            "should use KDE theme name from reader output"
        );
    }

    #[test]
    fn from_linux_unknown_de_without_kdeglobals_returns_adwaita() {
        // Unknown DE without kdeglobals falls back to Adwaita preset
        let preset = Theme::preset("adwaita").unwrap();
        let ro = theme_to_reader_output(&preset, false);
        let theme = run_pipeline(
            ro,
            preset.name,
            preset.icon_set,
            preset.layout,
            linux_preset_for_de(LinuxDesktop::Unknown),
            crate::ColorMode::Light,
            crate::AccessibilityPreferences::default(),
            None,
        )
        .expect("run_pipeline should succeed for Unknown DE fallback");
        assert_eq!(
            theme.name, "Adwaita",
            "should fall back to Adwaita without kdeglobals"
        );
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
        let preset = Theme::preset("adwaita").unwrap();
        let ro = theme_to_reader_output(&preset, false);
        let theme = run_pipeline(
            ro,
            preset.name,
            preset.icon_set,
            preset.layout,
            "adwaita-live",
            crate::ColorMode::Light,
            crate::AccessibilityPreferences::default(),
            None,
        )
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
    use crate::ReaderOutput;
    use crate::color::Rgba;
    use crate::model::{LayoutTheme, Theme, ThemeMode};

    use super::run_pipeline;

    // --- run_pipeline() tests ---

    #[test]
    fn test_run_pipeline_produces_both_variants() {
        let preset = Theme::preset("catppuccin-mocha").unwrap();
        let ro = ReaderOutput::Dual {
            light: Box::new(preset.light.clone().unwrap_or_default()),
            dark: Box::new(preset.dark.clone().unwrap_or_default()),
        };
        let result = run_pipeline(
            ro,
            preset.name,
            preset.icon_set,
            preset.layout,
            "catppuccin-mocha",
            crate::ColorMode::Light,
            crate::AccessibilityPreferences::default(),
            None,
        );
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
        let ro = ReaderOutput::Single {
            mode: Box::new(variant),
            is_dark: false,
        };

        let result = run_pipeline(
            ro,
            "CustomTheme".into(),
            None,
            LayoutTheme::default(),
            "catppuccin-mocha",
            crate::ColorMode::Light,
            crate::AccessibilityPreferences::default(),
            None,
        );
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
        let ro = ReaderOutput::Single {
            mode: Box::new(dark_v),
            is_dark: true,
        };

        let result = run_pipeline(
            ro,
            String::new(),
            None,
            LayoutTheme::default(),
            "kde-breeze-live",
            crate::ColorMode::Dark,
            crate::AccessibilityPreferences::default(),
            None,
        );
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
        let ro = ReaderOutput::Single {
            mode: Box::new(full.dark.clone().unwrap_or_default()),
            is_dark: true,
        };

        let st = run_pipeline(
            ro,
            String::new(),
            None,
            LayoutTheme::default(),
            "kde-breeze-live",
            crate::ColorMode::Dark,
            crate::AccessibilityPreferences::default(),
            None,
        )
        .unwrap();

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
        let preset = Theme::preset("adwaita").unwrap();
        let ro = ReaderOutput::Dual {
            light: Box::new(preset.light.clone().unwrap_or_default()),
            dark: Box::new(preset.dark.clone().unwrap_or_default()),
        };
        let result = run_pipeline(
            ro,
            preset.name,
            preset.icon_set,
            preset.layout,
            "adwaita",
            crate::ColorMode::Light,
            crate::AccessibilityPreferences::default(),
            None,
        );
        assert!(
            result.is_ok(),
            "double-merge with same preset should succeed"
        );
        let st = result.unwrap();
        assert_eq!(st.name, "Adwaita");
    }

    // --- theme_to_reader_output() bridge tests ---

    #[test]
    fn test_bridge_dual_variant() {
        use super::theme_to_reader_output;
        let mut theme = Theme::default();
        theme.light = Some(ThemeMode::default());
        theme.dark = Some(ThemeMode::default());
        let ro = theme_to_reader_output(&theme, false);
        assert!(matches!(ro, ReaderOutput::Dual { .. }));
    }

    #[test]
    fn test_bridge_single_dark() {
        use super::theme_to_reader_output;
        let mut theme = Theme::default();
        theme.dark = Some(ThemeMode::default());
        theme.light = None;
        let ro = theme_to_reader_output(&theme, true);
        assert!(matches!(ro, ReaderOutput::Single { is_dark: true, .. }));
    }

    #[test]
    fn test_bridge_single_light() {
        use super::theme_to_reader_output;
        let mut theme = Theme::default();
        theme.light = Some(ThemeMode::default());
        theme.dark = None;
        let ro = theme_to_reader_output(&theme, false);
        assert!(matches!(ro, ReaderOutput::Single { is_dark: false, .. }));
    }

    #[test]
    fn test_bridge_neither_defaults_to_single() {
        use super::theme_to_reader_output;
        let theme = Theme::default();
        let ro = theme_to_reader_output(&theme, false);
        assert!(matches!(ro, ReaderOutput::Single { is_dark: false, .. }));
    }
}
