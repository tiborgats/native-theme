//! Theme pipeline: reader -> preset merge -> resolve -> validate.

#[cfg(not(target_os = "linux"))]
use crate::detect::system_is_dark;
#[cfg(target_os = "linux")]
use crate::detect::{LinuxDesktop, detect_linux_de, system_is_dark, xdg_current_desktop};

use crate::SystemTheme;
use crate::model::ThemeSpec;

/// Run the OS-first pipeline: merge reader output onto a platform
/// preset, resolve both light and dark variants, validate.
///
/// For the variant the reader supplied, the merged (reader + live preset)
/// version is used. For the variant the reader did NOT supply, the full
/// platform preset (with colors/fonts) is used as fallback.
pub(crate) fn run_pipeline(
    reader_output: ThemeSpec,
    preset_name: &str,
    is_dark: bool,
) -> crate::Result<SystemTheme> {
    let live_preset = ThemeSpec::preset(preset_name)?;

    // For the inactive variant, load the full preset (with colors).
    // Falls back to original name if not a live preset (e.g. user preset).
    let full_preset_name = preset_name.strip_suffix("-live").unwrap_or(preset_name);
    debug_assert!(
        full_preset_name != preset_name || !preset_name.ends_with("-live"),
        "live preset '{preset_name}' should have -live suffix stripped"
    );
    let full_preset = ThemeSpec::preset(full_preset_name)?;

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
    // unwrap_or_default() yields an empty ThemeVariant -- valid for merge.
    let mut light_variant = if reader_output.light.is_some() {
        merged.light.unwrap_or_default()
    } else {
        full_preset.light.unwrap_or_default()
    };

    let mut dark_variant = if reader_output.dark.is_some() {
        merged.dark.unwrap_or_default()
    } else {
        full_preset.dark.unwrap_or_default()
    };

    // Propagate font_dpi from the reader to both variants so the
    // pt->px conversion uses the system-detected DPI for both.
    // The active variant already has font_dpi via merge; the inactive
    // variant comes from the full preset (no reader data) and needs it.
    let reader_dpi = reader_output
        .light
        .as_ref()
        .and_then(|v| v.defaults.font_dpi)
        .or_else(|| {
            reader_output
                .dark
                .as_ref()
                .and_then(|v| v.defaults.font_dpi)
        });
    if let Some(dpi) = reader_dpi {
        light_variant.defaults.font_dpi = light_variant.defaults.font_dpi.or(Some(dpi));
        dark_variant.defaults.font_dpi = dark_variant.defaults.font_dpi.or(Some(dpi));
    }

    // Clone pre-resolve variants for overlay support (Plan 02)
    let light_variant_pre = light_variant.clone();
    let dark_variant_pre = dark_variant.clone();

    let light = light_variant.into_resolved()?;
    let dark = dark_variant.into_resolved()?;

    Ok(SystemTheme {
        name,
        is_dark,
        light,
        dark,
        light_variant: light_variant_pre,
        dark_variant: dark_variant_pre,
        preset: full_preset_name.to_string(),
        live_preset: preset_name.to_string(),
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
/// let diagnostics = native_theme::diagnose_platform_support();
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
pub(crate) fn reader_is_dark(reader: &ThemeSpec) -> bool {
    reader.dark.is_some() && reader.light.is_none()
}

/// Read the current system theme on Linux by detecting the desktop
/// environment and calling the appropriate reader or returning a
/// preset fallback.
///
/// Runs the full OS-first pipeline: reader -> preset merge -> resolve -> validate.
#[cfg(target_os = "linux")]
pub(crate) fn from_linux() -> crate::Result<SystemTheme> {
    let is_dark = system_is_dark();
    let de = detect_linux_de(&xdg_current_desktop());
    let preset = linux_preset_for_de(de);
    match de {
        #[cfg(feature = "kde")]
        LinuxDesktop::Kde => {
            let reader = crate::kde::from_kde()?;
            run_pipeline(reader, preset, is_dark)
        }
        #[cfg(not(feature = "kde"))]
        LinuxDesktop::Kde => run_pipeline(ThemeSpec::preset("adwaita")?, "adwaita-live", is_dark),
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
            // GNOME sync path: no portal, just adwaita preset
            run_pipeline(ThemeSpec::preset("adwaita")?, preset, is_dark)
        }
        LinuxDesktop::Xfce
        | LinuxDesktop::Cinnamon
        | LinuxDesktop::Mate
        | LinuxDesktop::LxQt
        | LinuxDesktop::Hyprland
        | LinuxDesktop::Sway
        | LinuxDesktop::River
        | LinuxDesktop::Niri
        | LinuxDesktop::CosmicDe => run_pipeline(ThemeSpec::preset("adwaita")?, preset, is_dark),
        LinuxDesktop::Unknown => {
            #[cfg(feature = "kde")]
            {
                let path = crate::kde::kdeglobals_path();
                if path.exists() {
                    let reader = crate::kde::from_kde()?;
                    return run_pipeline(reader, linux_preset_for_de(LinuxDesktop::Kde), is_dark);
                }
            }
            run_pipeline(ThemeSpec::preset("adwaita")?, preset, is_dark)
        }
    }
}

pub(crate) fn from_system_inner() -> crate::Result<SystemTheme> {
    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        {
            let reader = crate::macos::from_macos()?;
            let is_dark = reader_is_dark(&reader);
            return run_pipeline(reader, "macos-sonoma-live", is_dark);
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
            let is_dark = reader_is_dark(&reader);
            return run_pipeline(reader, "windows-11-live", is_dark);
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
    let is_dark = system_is_dark();
    let de = detect_linux_de(&xdg_current_desktop());
    let preset = linux_preset_for_de(de);
    match de {
        #[cfg(feature = "kde")]
        LinuxDesktop::Kde => {
            #[cfg(feature = "portal")]
            {
                let reader = crate::gnome::from_kde_with_portal().await?;
                run_pipeline(reader, preset, is_dark)
            }
            #[cfg(not(feature = "portal"))]
            {
                let reader = crate::kde::from_kde()?;
                run_pipeline(reader, preset, is_dark)
            }
        }
        #[cfg(not(feature = "kde"))]
        LinuxDesktop::Kde => run_pipeline(ThemeSpec::preset("adwaita")?, "adwaita-live", is_dark),
        #[cfg(feature = "portal")]
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
            let reader = crate::gnome::from_gnome().await?;
            run_pipeline(reader, preset, is_dark)
        }
        #[cfg(not(feature = "portal"))]
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
            run_pipeline(ThemeSpec::preset("adwaita")?, preset, is_dark)
        }
        LinuxDesktop::Xfce
        | LinuxDesktop::Cinnamon
        | LinuxDesktop::Mate
        | LinuxDesktop::LxQt
        | LinuxDesktop::Hyprland
        | LinuxDesktop::Sway
        | LinuxDesktop::River
        | LinuxDesktop::Niri
        | LinuxDesktop::CosmicDe => run_pipeline(ThemeSpec::preset("adwaita")?, preset, is_dark),
        LinuxDesktop::Unknown => {
            // Use D-Bus portal backend detection to refine heuristic
            #[cfg(feature = "portal")]
            {
                if let Some(detected) = crate::gnome::detect_portal_backend().await {
                    let detected_preset = linux_preset_for_de(detected);
                    return match detected {
                        #[cfg(feature = "kde")]
                        LinuxDesktop::Kde => {
                            let reader = crate::gnome::from_kde_with_portal().await?;
                            run_pipeline(reader, detected_preset, is_dark)
                        }
                        #[cfg(not(feature = "kde"))]
                        LinuxDesktop::Kde => {
                            run_pipeline(ThemeSpec::preset("adwaita")?, "adwaita-live", is_dark)
                        }
                        LinuxDesktop::Gnome => {
                            let reader = crate::gnome::from_gnome().await?;
                            run_pipeline(reader, detected_preset, is_dark)
                        }
                        _ => {
                            // detect_portal_backend only returns Kde or Gnome;
                            // fall back to Adwaita if the set ever grows.
                            run_pipeline(ThemeSpec::preset("adwaita")?, detected_preset, is_dark)
                        }
                    };
                }
            }
            // Sync fallback: try kdeglobals, then Adwaita
            #[cfg(feature = "kde")]
            {
                let path = crate::kde::kdeglobals_path();
                if path.exists() {
                    let reader = crate::kde::from_kde()?;
                    return run_pipeline(reader, linux_preset_for_de(LinuxDesktop::Kde), is_dark);
                }
            }
            run_pipeline(ThemeSpec::preset("adwaita")?, preset, is_dark)
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
        let reader = ThemeSpec::preset("adwaita").unwrap();
        let theme = run_pipeline(reader, linux_preset_for_de(LinuxDesktop::Gnome), false)
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

        let reader = crate::kde::from_kde_content_pure(MINIMAL_KDE_FIXTURE, None).unwrap();
        let theme = run_pipeline(reader, linux_preset_for_de(LinuxDesktop::Kde), false)
            .expect("run_pipeline should succeed with KDE reader output");
        assert_eq!(
            theme.name, "TestTheme",
            "should use KDE theme name from reader output"
        );
    }

    #[test]
    fn from_linux_unknown_de_without_kdeglobals_returns_adwaita() {
        // Unknown DE without kdeglobals falls back to Adwaita preset
        let reader = ThemeSpec::preset("adwaita").unwrap();
        let theme = run_pipeline(reader, linux_preset_for_de(LinuxDesktop::Unknown), false)
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
        let reader = ThemeSpec::preset("adwaita").unwrap();
        let theme = run_pipeline(reader, "adwaita-live", false)
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
    use crate::model::{ThemeDefaults, ThemeSpec, ThemeVariant};

    use super::{reader_is_dark, run_pipeline};

    // --- run_pipeline() tests ---

    #[test]
    fn test_run_pipeline_produces_both_variants() {
        let reader = ThemeSpec::preset("catppuccin-mocha").unwrap();
        let result = run_pipeline(reader, "catppuccin-mocha", false);
        assert!(result.is_ok(), "run_pipeline should succeed");
        let st = result.unwrap();
        // Both light and dark exist as ResolvedThemeVariant (non-Option)
        assert!(!st.name.is_empty(), "name should be populated");
        // If we get here, both variants validated successfully
    }

    #[test]
    fn test_run_pipeline_reader_values_win() {
        // Create a reader output where the reader provides a custom accent
        // (simulating a platform reader that detected this accent from the OS)
        let custom_accent = Rgba::rgb(42, 100, 200);
        let mut reader = ThemeSpec::default();
        reader.name = "CustomTheme".into();
        let mut variant = ThemeVariant::default();
        variant.defaults.accent_color = Some(custom_accent);
        reader.light = Some(variant);

        let result = run_pipeline(reader, "catppuccin-mocha", false);
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
        let full = ThemeSpec::preset("kde-breeze").unwrap();
        let mut reader = ThemeSpec::default();
        let mut dark_v = full.dark.clone().unwrap();
        // Override accent to prove reader values win (simulating OS-detected accent)
        dark_v.defaults.accent_color = Some(Rgba::rgb(200, 50, 50));
        reader.dark = Some(dark_v);
        reader.light = None;

        let result = run_pipeline(reader, "kde-breeze-live", true);
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
        let full = ThemeSpec::preset("kde-breeze").unwrap();
        let mut reader = ThemeSpec::default();
        reader.dark = Some(full.dark.clone().unwrap());
        reader.light = None;

        let st = run_pipeline(reader, "kde-breeze-live", true).unwrap();

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
        let reader = ThemeSpec::preset("adwaita").unwrap();
        let result = run_pipeline(reader, "adwaita", false);
        assert!(
            result.is_ok(),
            "double-merge with same preset should succeed"
        );
        let st = result.unwrap();
        assert_eq!(st.name, "Adwaita");
    }

    // --- run_pipeline font_dpi propagation ---

    #[test]
    fn test_run_pipeline_propagates_font_dpi_to_inactive_variant() {
        // Create a reader that provides only dark variant with font_dpi=120
        // (simulating a platform reader that detected this DPI from the OS)
        let mut reader = ThemeSpec::default();
        reader.dark = Some(ThemeVariant {
            defaults: ThemeDefaults {
                font_dpi: Some(120.0),
                ..Default::default()
            },
            ..Default::default()
        });

        let st = run_pipeline(reader, "kde-breeze-live", true).unwrap();
        // The light variant (inactive, from full preset) should have gotten
        // font_dpi propagated and used for conversion.
        //
        // After resolution, font_dpi is consumed (cleared during conversion,
        // then filled with DEFAULT_FONT_DPI in validate.rs), so we cannot
        // check font_dpi directly. Instead, verify the conversion effect:
        // the preset's default font size is in points. With font_dpi=120:
        //   px = pt * 120 / 72 = pt * 1.6667
        //
        // The Breeze preset default font size is 10.0 pt.
        // With DPI 120: 10.0 * 120/72 = 16.667 px
        // Without propagation (no conversion): 10.0 px
        let resolved_size = st.light.defaults.font.size;
        assert!(
            resolved_size > 10.0,
            "inactive variant font size should be DPI-converted (got {resolved_size}, expected > 10.0)"
        );
        // Check it matches the expected conversion
        let expected = 10.0 * 120.0 / 72.0; // ~16.667
        assert!(
            (resolved_size - expected).abs() < 0.1,
            "font size should be 10pt * 120/72 = {expected:.1}px, got {resolved_size}"
        );
    }

    // --- reader_is_dark() tests ---

    #[test]
    fn test_reader_is_dark_only_dark() {
        let mut theme = ThemeSpec::default();
        theme.dark = Some(ThemeVariant::default());
        theme.light = None;
        assert!(
            reader_is_dark(&theme),
            "should be true when only dark is set"
        );
    }

    #[test]
    fn test_reader_is_dark_only_light() {
        let mut theme = ThemeSpec::default();
        theme.light = Some(ThemeVariant::default());
        theme.dark = None;
        assert!(
            !reader_is_dark(&theme),
            "should be false when only light is set"
        );
    }

    #[test]
    fn test_reader_is_dark_both() {
        let mut theme = ThemeSpec::default();
        theme.light = Some(ThemeVariant::default());
        theme.dark = Some(ThemeVariant::default());
        assert!(
            !reader_is_dark(&theme),
            "should be false when both are set (macOS case)"
        );
    }

    #[test]
    fn test_reader_is_dark_neither() {
        let theme = ThemeSpec::default();
        assert!(
            !reader_is_dark(&theme),
            "should be false when neither is set"
        );
    }
}
