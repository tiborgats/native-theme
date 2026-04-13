//! Theme pipeline: reader -> preset merge -> resolve -> validate.

#[cfg(not(target_os = "linux"))]
use crate::detect::system_is_dark;
#[cfg(target_os = "linux")]
use crate::detect::{LinuxDesktop, detect_linux_de, system_is_dark, xdg_current_desktop};

use crate::model::Theme;
use crate::{OverlaySource, SystemTheme};

/// Run the OS-first pipeline: merge reader output onto a platform
/// preset, resolve both light and dark variants, validate.
///
/// For the variant the reader supplied, the merged (reader + live preset)
/// version is used. For the variant the reader did NOT supply, the full
/// platform preset (with colors/fonts) is used as fallback.
pub(crate) fn run_pipeline(
    reader_output: Theme,
    preset_name: &str,
    mode: crate::ColorMode,
    accessibility: crate::AccessibilityPreferences,
    font_dpi: Option<f32>,
) -> crate::Result<SystemTheme> {
    // Clone reader output before it gets consumed by merge -- needed for OverlaySource
    let reader_output_for_overlay = reader_output.clone();

    let live_preset = Theme::preset(preset_name)?;

    // For the inactive variant, load the full preset (with colors).
    // Falls back to original name if not a live preset (e.g. user preset).
    let full_preset_name = preset_name.strip_suffix("-live").unwrap_or(preset_name);
    debug_assert!(
        full_preset_name != preset_name || !preset_name.ends_with("-live"),
        "live preset '{preset_name}' should have -live suffix stripped"
    );
    let full_preset = Theme::preset(full_preset_name)?;

    // Merge: full preset provides color/font defaults, live preset overrides
    // geometry, reader output provides live OS data on top.
    let mut merged = full_preset.clone();
    merged.merge(&live_preset);
    merged.merge(&reader_output);

    // Keep reader name if non-empty, else use preset name
    let name = if reader_output.name.is_empty() {
        merged.name.clone()
    } else {
        reader_output.name.clone()
    };

    // For the variant the reader provided: use merged (live geometry + reader colors)
    // For the variant the reader didn't provide: use FULL preset (has colors).
    // unwrap_or_default() yields an empty ThemeMode -- valid for merge.
    let light_variant = if reader_output.light.is_some() {
        merged.light.unwrap_or_default()
    } else {
        full_preset.light.unwrap_or_default()
    };

    let dark_variant = if reader_output.dark.is_some() {
        merged.dark.unwrap_or_default()
    } else {
        full_preset.dark.unwrap_or_default()
    };

    let light = light_variant.into_resolved(font_dpi)?;
    let dark = dark_variant.into_resolved(font_dpi)?;

    // Resolve icon_set and icon_theme from Theme level (shared across variants)
    let icon_set = merged
        .icon_set
        .unwrap_or_else(crate::model::icons::system_icon_set);
    let icon_theme = merged
        .icon_theme
        .clone()
        .unwrap_or_else(|| crate::model::icons::system_icon_theme().to_string());

    // Build OverlaySource from the original reader data + pipeline parameters
    let overlay_source = OverlaySource {
        reader_output: reader_output_for_overlay,
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
/// by [`from_linux()`], [`from_system_async_inner()`], and
/// [`platform_preset_name()`].
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
        linux_preset_for_de(detect_linux_de(&xdg_current_desktop()))
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
                let de = detect_linux_de(&val);
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

/// Infer dark-mode preference from the reader's output.
///
/// Returns `true` if the reader populated only the dark variant,
/// `false` if it populated only light or both variants.
/// On platforms that produce both variants (macOS), this defaults to
/// `false` (light); callers can use [`SystemTheme::pick()`] for
/// explicit variant selection regardless of this default.
#[allow(dead_code)]
pub(crate) fn reader_is_dark(reader: &Theme) -> bool {
    reader.dark.is_some() && reader.light.is_none()
}

/// Read the current system theme on Linux by detecting the desktop
/// environment and calling the appropriate reader or returning a
/// preset fallback.
///
/// Runs the full OS-first pipeline: reader -> preset merge -> resolve -> validate.
#[cfg(target_os = "linux")]
pub(crate) fn from_linux() -> crate::Result<SystemTheme> {
    let mode = if system_is_dark() {
        crate::ColorMode::Dark
    } else {
        crate::ColorMode::Light
    };
    let de = detect_linux_de(&xdg_current_desktop());
    let preset = linux_preset_for_de(de);
    match de {
        #[cfg(feature = "kde")]
        LinuxDesktop::Kde => {
            let (reader, dpi, acc) = crate::kde::from_kde()?;
            run_pipeline(reader, preset, mode, acc, dpi)
        }
        #[cfg(not(feature = "kde"))]
        LinuxDesktop::Kde => run_pipeline(Theme::preset("adwaita")?, "adwaita-live", mode, crate::AccessibilityPreferences::default(), None),
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
            // GNOME sync path: no portal, just adwaita preset
            run_pipeline(Theme::preset("adwaita")?, preset, mode, crate::AccessibilityPreferences::default(), None)
        }
        LinuxDesktop::Xfce
        | LinuxDesktop::Cinnamon
        | LinuxDesktop::Mate
        | LinuxDesktop::LxQt
        | LinuxDesktop::Hyprland
        | LinuxDesktop::Sway
        | LinuxDesktop::River
        | LinuxDesktop::Niri
        | LinuxDesktop::CosmicDe => run_pipeline(Theme::preset("adwaita")?, preset, mode, crate::AccessibilityPreferences::default(), None),
        LinuxDesktop::Unknown => {
            #[cfg(feature = "kde")]
            {
                let path = crate::kde::kdeglobals_path();
                if path.exists() {
                    let (reader, dpi, acc) = crate::kde::from_kde()?;
                    return run_pipeline(reader, linux_preset_for_de(LinuxDesktop::Kde), mode, acc, dpi);
                }
            }
            run_pipeline(Theme::preset("adwaita")?, preset, mode, crate::AccessibilityPreferences::default(), None)
        }
    }
}

pub(crate) fn from_system_inner() -> crate::Result<SystemTheme> {
    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        {
            let reader = crate::macos::from_macos()?;
            // TODO(78-04-task2): from_macos will return tuple
            let mode = if reader_is_dark(&reader) {
                crate::ColorMode::Dark
            } else {
                crate::ColorMode::Light
            };
            return run_pipeline(reader, "macos-sonoma-live", mode, crate::AccessibilityPreferences::default(), None);
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
            let reader = crate::windows::from_windows()?;
            // TODO(78-04-task2): from_windows will return tuple
            let mode = if reader_is_dark(&reader) {
                crate::ColorMode::Dark
            } else {
                crate::ColorMode::Light
            };
            return run_pipeline(reader, "windows-11-live", mode, crate::AccessibilityPreferences::default(), None);
        }

        #[cfg(not(feature = "windows"))]
        return Err(crate::Error::FeatureDisabled {
            name: "windows",
            needed_for: "Windows theme detection",
        });
    }

    #[cfg(target_os = "linux")]
    {
        from_linux()
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Err(crate::Error::PlatformUnsupported {
            platform: "unsupported",
        })
    }
}

#[cfg(target_os = "linux")]
pub(crate) async fn from_system_async_inner() -> crate::Result<SystemTheme> {
    let mode = if system_is_dark() {
        crate::ColorMode::Dark
    } else {
        crate::ColorMode::Light
    };
    let de = detect_linux_de(&xdg_current_desktop());
    let preset = linux_preset_for_de(de);
    match de {
        #[cfg(feature = "kde")]
        LinuxDesktop::Kde => {
            #[cfg(feature = "portal")]
            {
                let (reader, dpi, acc) = crate::gnome::from_kde_with_portal().await?;
                run_pipeline(reader, preset, mode, acc, dpi)
            }
            #[cfg(not(feature = "portal"))]
            {
                let (reader, dpi, acc) = crate::kde::from_kde()?;
                run_pipeline(reader, preset, mode, acc, dpi)
            }
        }
        #[cfg(not(feature = "kde"))]
        LinuxDesktop::Kde => run_pipeline(Theme::preset("adwaita")?, "adwaita-live", mode, crate::AccessibilityPreferences::default(), None),
        #[cfg(feature = "portal")]
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
            // TODO(78-04-task2): from_gnome will return tuple
            let reader = crate::gnome::from_gnome().await?;
            run_pipeline(reader, preset, mode, crate::AccessibilityPreferences::default(), None)
        }
        #[cfg(not(feature = "portal"))]
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
            run_pipeline(Theme::preset("adwaita")?, preset, mode, crate::AccessibilityPreferences::default(), None)
        }
        LinuxDesktop::Xfce
        | LinuxDesktop::Cinnamon
        | LinuxDesktop::Mate
        | LinuxDesktop::LxQt
        | LinuxDesktop::Hyprland
        | LinuxDesktop::Sway
        | LinuxDesktop::River
        | LinuxDesktop::Niri
        | LinuxDesktop::CosmicDe => run_pipeline(Theme::preset("adwaita")?, preset, mode, crate::AccessibilityPreferences::default(), None),
        LinuxDesktop::Unknown => {
            // Use D-Bus portal backend detection to refine heuristic
            #[cfg(feature = "portal")]
            {
                if let Some(detected) = crate::gnome::detect_portal_backend().await {
                    let detected_preset = linux_preset_for_de(detected);
                    return match detected {
                        #[cfg(feature = "kde")]
                        LinuxDesktop::Kde => {
                            let (reader, dpi, acc) = crate::gnome::from_kde_with_portal().await?;
                            run_pipeline(reader, detected_preset, mode, acc, dpi)
                        }
                        #[cfg(not(feature = "kde"))]
                        LinuxDesktop::Kde => {
                            run_pipeline(Theme::preset("adwaita")?, "adwaita-live", mode, crate::AccessibilityPreferences::default(), None)
                        }
                        LinuxDesktop::Gnome => {
                            // TODO(78-04-task2): from_gnome will return tuple
                            let reader = crate::gnome::from_gnome().await?;
                            run_pipeline(reader, detected_preset, mode, crate::AccessibilityPreferences::default(), None)
                        }
                        _ => {
                            // detect_portal_backend only returns Kde or Gnome;
                            // fall back to Adwaita if the set ever grows.
                            run_pipeline(Theme::preset("adwaita")?, detected_preset, mode, crate::AccessibilityPreferences::default(), None)
                        }
                    };
                }
            }
            // Sync fallback: try kdeglobals, then Adwaita
            #[cfg(feature = "kde")]
            {
                let path = crate::kde::kdeglobals_path();
                if path.exists() {
                    let (reader, dpi, acc) = crate::kde::from_kde()?;
                    return run_pipeline(reader, linux_preset_for_de(LinuxDesktop::Kde), mode, acc, dpi);
                }
            }
            run_pipeline(Theme::preset("adwaita")?, preset, mode, crate::AccessibilityPreferences::default(), None)
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(all(test, target_os = "linux"))]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod dispatch_tests {
    use super::*;

    // -- detect_linux_de() pure function tests --

    #[test]
    fn detect_kde_simple() {
        assert_eq!(detect_linux_de("KDE"), LinuxDesktop::Kde);
    }

    #[test]
    fn detect_kde_colon_separated_after() {
        assert_eq!(detect_linux_de("ubuntu:KDE"), LinuxDesktop::Kde);
    }

    #[test]
    fn detect_kde_colon_separated_before() {
        assert_eq!(detect_linux_de("KDE:plasma"), LinuxDesktop::Kde);
    }

    #[test]
    fn detect_gnome_simple() {
        assert_eq!(detect_linux_de("GNOME"), LinuxDesktop::Gnome);
    }

    #[test]
    fn detect_gnome_ubuntu() {
        assert_eq!(detect_linux_de("ubuntu:GNOME"), LinuxDesktop::Gnome);
    }

    #[test]
    fn detect_xfce() {
        assert_eq!(detect_linux_de("XFCE"), LinuxDesktop::Xfce);
    }

    #[test]
    fn detect_cinnamon() {
        assert_eq!(detect_linux_de("X-Cinnamon"), LinuxDesktop::Cinnamon);
    }

    #[test]
    fn detect_cinnamon_short() {
        assert_eq!(detect_linux_de("Cinnamon"), LinuxDesktop::Cinnamon);
    }

    #[test]
    fn detect_mate() {
        assert_eq!(detect_linux_de("MATE"), LinuxDesktop::Mate);
    }

    #[test]
    fn detect_lxqt() {
        assert_eq!(detect_linux_de("LXQt"), LinuxDesktop::LxQt);
    }

    #[test]
    fn detect_budgie() {
        assert_eq!(detect_linux_de("Budgie:GNOME"), LinuxDesktop::Budgie);
    }

    #[test]
    fn detect_empty_string() {
        assert_eq!(detect_linux_de(""), LinuxDesktop::Unknown);
    }

    // -- Pure pipeline dispatch tests (no env var manipulation) --

    #[test]
    fn from_linux_non_kde_returns_adwaita() {
        // GNOME desktop produces an Adwaita-named theme via the pure pipeline
        let reader = Theme::preset("adwaita").unwrap();
        let theme = run_pipeline(
            reader,
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

        let (reader, dpi, acc) =
            crate::kde::from_kde_content_pure(MINIMAL_KDE_FIXTURE, None).unwrap();
        let theme = run_pipeline(
            reader,
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
        let reader = Theme::preset("adwaita").unwrap();
        let theme = run_pipeline(
            reader,
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
        assert_eq!(detect_linux_de("Hyprland"), LinuxDesktop::Hyprland);
    }

    #[test]
    fn detect_sway() {
        assert_eq!(detect_linux_de("sway"), LinuxDesktop::Sway);
    }

    #[test]
    fn detect_cosmic() {
        assert_eq!(detect_linux_de("COSMIC"), LinuxDesktop::CosmicDe);
    }

    #[test]
    fn detect_river() {
        assert_eq!(detect_linux_de("river"), LinuxDesktop::River);
    }

    #[test]
    fn detect_niri() {
        assert_eq!(detect_linux_de("niri"), LinuxDesktop::Niri);
    }

    #[test]
    fn detect_cosmic_full_desktop() {
        assert_eq!(
            detect_linux_de("COSMIC:Freedesktop"),
            LinuxDesktop::CosmicDe
        );
    }

    // -- Pure pipeline smoke test (replaces from_system env var test) --

    #[test]
    fn from_system_returns_result() {
        // Test the pure pipeline directly instead of mocking env vars for from_system()
        let reader = Theme::preset("adwaita").unwrap();
        let theme = run_pipeline(reader, "adwaita-live", crate::ColorMode::Light, crate::AccessibilityPreferences::default(), None)
            .expect("run_pipeline should succeed with adwaita preset");
        assert_eq!(theme.name, "Adwaita");
    }
}

/// Tests for run_pipeline() and reader_is_dark() -- internal pipeline functions.
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
    use crate::model::{Theme, ThemeMode};

    use super::{reader_is_dark, run_pipeline};

    // --- run_pipeline() tests ---

    #[test]
    fn test_run_pipeline_produces_both_variants() {
        let reader = Theme::preset("catppuccin-mocha").unwrap();
        let result = run_pipeline(reader, "catppuccin-mocha", crate::ColorMode::Light, crate::AccessibilityPreferences::default(), None);
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
        let mut reader = Theme::default();
        reader.name = "CustomTheme".into();
        let mut variant = ThemeMode::default();
        variant.defaults.accent_color = Some(custom_accent);
        reader.light = Some(variant);

        let result = run_pipeline(reader, "catppuccin-mocha", crate::ColorMode::Light, crate::AccessibilityPreferences::default(), None);
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
        let mut reader = Theme::default();
        let mut dark_v = full.dark.clone().unwrap();
        // Override accent to prove reader values win (simulating OS-detected accent)
        dark_v.defaults.accent_color = Some(Rgba::rgb(200, 50, 50));
        reader.dark = Some(dark_v);
        reader.light = None;

        let result = run_pipeline(reader, "kde-breeze-live", crate::ColorMode::Dark, crate::AccessibilityPreferences::default(), None);
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
        let mut reader = Theme::default();
        reader.dark = Some(full.dark.clone().unwrap());
        reader.light = None;

        let st = run_pipeline(reader, "kde-breeze-live", crate::ColorMode::Dark, crate::AccessibilityPreferences::default(), None).unwrap();

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
        let reader = Theme::preset("adwaita").unwrap();
        let result = run_pipeline(reader, "adwaita", crate::ColorMode::Light, crate::AccessibilityPreferences::default(), None);
        assert!(
            result.is_ok(),
            "double-merge with same preset should succeed"
        );
        let st = result.unwrap();
        assert_eq!(st.name, "Adwaita");
    }

    // --- run_pipeline font_dpi propagation ---

    // NOTE: test_run_pipeline_propagates_font_dpi_to_inactive_variant removed.
    // font_dpi is no longer on ThemeDefaults; it will be threaded through
    // run_pipeline as an explicit parameter in Task 2.

    // --- reader_is_dark() tests ---

    #[test]
    fn test_reader_is_dark_only_dark() {
        let mut theme = Theme::default();
        theme.dark = Some(ThemeMode::default());
        theme.light = None;
        assert!(
            reader_is_dark(&theme),
            "should be true when only dark is set"
        );
    }

    #[test]
    fn test_reader_is_dark_only_light() {
        let mut theme = Theme::default();
        theme.light = Some(ThemeMode::default());
        theme.dark = None;
        assert!(
            !reader_is_dark(&theme),
            "should be false when only light is set"
        );
    }

    #[test]
    fn test_reader_is_dark_both() {
        let mut theme = Theme::default();
        theme.light = Some(ThemeMode::default());
        theme.dark = Some(ThemeMode::default());
        assert!(
            !reader_is_dark(&theme),
            "should be false when both are set (macOS case)"
        );
    }

    #[test]
    fn test_reader_is_dark_neither() {
        let theme = Theme::default();
        assert!(
            !reader_is_dark(&theme),
            "should be false when neither is set"
        );
    }
}
