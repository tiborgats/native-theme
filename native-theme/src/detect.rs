//! OS detection: dark mode, reduced motion, DPI, desktop environment.

/// Desktop environments recognized on Linux.
#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinuxDesktop {
    /// KDE Plasma desktop.
    Kde,
    /// GNOME desktop.
    Gnome,
    /// Xfce desktop.
    Xfce,
    /// Cinnamon desktop (Linux Mint).
    Cinnamon,
    /// MATE desktop.
    Mate,
    /// LXQt desktop.
    LxQt,
    /// Budgie desktop.
    Budgie,
    /// Unrecognized or unset desktop environment.
    Unknown,
}

/// Read the `XDG_CURRENT_DESKTOP` environment variable, returning an
/// empty string if unset or invalid UTF-8.
#[cfg(target_os = "linux")]
pub(crate) fn xdg_current_desktop() -> String {
    std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
}

/// Parse `XDG_CURRENT_DESKTOP` (a colon-separated list) and return
/// the recognized desktop environment.
///
/// Checks components in order; first recognized DE wins. Budgie is checked
/// before GNOME because Budgie sets `Budgie:GNOME`.
#[cfg(target_os = "linux")]
#[must_use]
pub fn detect_linux_de(xdg_current_desktop: &str) -> LinuxDesktop {
    for component in xdg_current_desktop.split(':') {
        match component {
            "KDE" => return LinuxDesktop::Kde,
            "Budgie" => return LinuxDesktop::Budgie,
            "GNOME" => return LinuxDesktop::Gnome,
            "XFCE" => return LinuxDesktop::Xfce,
            "X-Cinnamon" | "Cinnamon" => return LinuxDesktop::Cinnamon,
            "MATE" => return LinuxDesktop::Mate,
            "LXQt" => return LinuxDesktop::LxQt,
            _ => {}
        }
    }
    LinuxDesktop::Unknown
}

static CACHED_IS_DARK: std::sync::RwLock<Option<bool>> = std::sync::RwLock::new(None);

/// Detect whether the system is using a dark color scheme.
///
/// Uses synchronous, platform-specific checks so the result is available
/// immediately at window creation time (before any async portal response).
///
/// # Caching
///
/// The result is cached after the first call and reused on subsequent calls.
/// Call [`invalidate_caches()`] to clear the cached value so the next call
/// re-queries the OS. For a fresh reading without affecting the cache, use
/// [`detect_is_dark()`] instead.
///
/// For live dark-mode tracking, subscribe to OS appearance-change events
/// (D-Bus `SettingChanged` on Linux, `NSAppearance` KVO on macOS,
/// `UISettings.ColorValuesChanged` on Windows) and call [`SystemTheme::from_system()`]
/// to get a fresh [`SystemTheme`] with updated resolved variants.
///
/// # Platform Behavior
///
/// - **Linux:** Checks `GTK_THEME` env var for `:dark` suffix or `-dark`
///   in name; queries `gsettings` for `color-scheme` (with 2-second
///   timeout); falls back to KDE `kdeglobals` background luminance (with
///   `kde` feature); reads `gtk-3.0/settings.ini` for
///   `gtk-application-prefer-dark-theme=1` as final fallback.
/// - **macOS:** Reads `AppleInterfaceStyle` via `NSUserDefaults` (with
///   `macos` feature) or `defaults` subprocess (without).
/// - **Windows:** Checks foreground color luminance from `UISettings` via
///   BT.601 coefficients (requires `windows` feature).
/// - **Other platforms / missing features:** Returns `false` (light).
#[must_use = "this returns whether the system uses dark mode"]
pub fn system_is_dark() -> bool {
    if let Ok(guard) = CACHED_IS_DARK.read()
        && let Some(v) = *guard
    {
        return v;
    }
    let value = detect_is_dark_inner();
    if let Ok(mut guard) = CACHED_IS_DARK.write() {
        *guard = Some(value);
    }
    value
}

/// Reset all process-wide caches so the next call to [`system_is_dark()`],
/// [`prefers_reduced_motion()`], or [`system_icon_theme()`] re-queries the OS.
///
/// Call this when you detect that the user has changed system settings (e.g.,
/// dark mode toggle, icon theme switch, accessibility preferences).
///
/// The `detect_*()` family of functions are unaffected — they always query
/// the OS directly.
pub fn invalidate_caches() {
    if let Ok(mut g) = CACHED_IS_DARK.write() {
        *g = None;
    }
    if let Ok(mut g) = CACHED_REDUCED_MOTION.write() {
        *g = None;
    }
    crate::model::icons::invalidate_icon_theme_cache();
}

/// Detect whether the system is using a dark color scheme without caching.
///
/// Unlike [`system_is_dark()`], this function queries the OS every time it is
/// called and never caches the result. Use this when polling for theme changes
/// or implementing live dark-mode tracking.
///
/// See [`system_is_dark()`] for platform behavior details.
#[must_use = "this returns whether the system uses dark mode"]
pub fn detect_is_dark() -> bool {
    detect_is_dark_inner()
}

/// Run a gsettings command with a 2-second timeout.
///
/// Spawns `gsettings` with the given arguments, waits up to 2 seconds
/// for completion, and returns the trimmed stdout on success.  Returns
/// `None` if the command fails, times out, or produces empty output.
///
/// Used by [`detect_is_dark_inner()`] and [`crate::gnome::read_gsetting()`] to
/// prevent gsettings from blocking indefinitely when D-Bus is unresponsive.
#[cfg(target_os = "linux")]
fn run_gsettings_with_timeout(args: &[&str]) -> Option<String> {
    use std::io::Read;
    use std::time::{Duration, Instant};

    let deadline = Instant::now() + Duration::from_secs(2);
    let mut child = std::process::Command::new("gsettings")
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()?;

    loop {
        match child.try_wait() {
            Ok(Some(status)) if status.success() => {
                let mut buf = String::new();
                if let Some(mut stdout) = child.stdout.take() {
                    let _ = stdout.read_to_string(&mut buf);
                }
                let trimmed = buf.trim().to_string();
                return if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                };
            }
            Ok(Some(_)) => return None,
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    return None;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(_) => return None,
        }
    }
}

/// Read `Xft.dpi` from X resources via `xrdb -query`.
///
/// Returns `None` if xrdb is not installed, times out (2 seconds),
/// or the output does not contain a valid positive `Xft.dpi` value.
#[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]
fn read_xft_dpi() -> Option<f32> {
    use std::io::Read;
    use std::time::{Duration, Instant};

    let deadline = Instant::now() + Duration::from_secs(2);
    let mut child = std::process::Command::new("xrdb")
        .arg("-query")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()?;

    loop {
        match child.try_wait() {
            Ok(Some(status)) if status.success() => {
                let mut buf = String::new();
                if let Some(mut stdout) = child.stdout.take() {
                    let _ = stdout.read_to_string(&mut buf);
                }
                // Parse "Xft.dpi:\t96" from multi-line output
                for line in buf.lines() {
                    if let Some(rest) = line.strip_prefix("Xft.dpi:")
                        && let Ok(dpi) = rest.trim().parse::<f32>()
                        && dpi > 0.0
                    {
                        return Some(dpi);
                    }
                }
                return None;
            }
            Ok(Some(_)) => return None,
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    return None;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(_) => return None,
        }
    }
}

/// Detect physical DPI from display hardware via `xrandr`.
///
/// Parses the primary connected output's resolution and physical dimensions
/// to compute DPI. Falls back to the first connected output if no primary
/// is found. Returns `None` if `xrandr` is unavailable, times out (2 seconds),
/// or the output cannot be parsed.
///
/// This is a last-resort fallback: prefer `forceFontDPI` (KDE), `Xft.dpi`
/// (X resources), or `GetDpiForSystem` (Windows) before calling this.
#[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]
fn detect_physical_dpi() -> Option<f32> {
    use std::io::Read;
    use std::time::{Duration, Instant};

    let deadline = Instant::now() + Duration::from_secs(2);
    let mut child = std::process::Command::new("xrandr")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()?;

    loop {
        match child.try_wait() {
            Ok(Some(status)) if status.success() => {
                let mut buf = String::new();
                if let Some(mut stdout) = child.stdout.take() {
                    let _ = stdout.read_to_string(&mut buf);
                }
                return parse_xrandr_dpi(&buf);
            }
            Ok(Some(_)) => return None,
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    return None;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(_) => return None,
        }
    }
}

/// Parse DPI from xrandr output.
///
/// Looks for lines like:
/// ```text
/// DP-1 connected primary 3840x2160+0+0 (...) 700mm x 390mm
/// ```
/// Extracts the current resolution from the mode string and the physical
/// dimensions from the trailing `NNNmm x NNNmm`, then computes average DPI.
#[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]
fn parse_xrandr_dpi(output: &str) -> Option<f32> {
    // Prefer the primary output; fall back to the first connected output.
    let line = output
        .lines()
        .find(|l| l.contains(" connected") && l.contains("primary"))
        .or_else(|| {
            output
                .lines()
                .find(|l| l.contains(" connected") && !l.contains("disconnected"))
        })?;

    // Resolution: "3840x2160+0+0" (digits x digits + offset)
    let res_token = line
        .split_whitespace()
        .find(|s| s.contains('x') && s.contains('+'))?;
    let (w_str, rest) = res_token.split_once('x')?;
    let h_str = rest.split('+').next()?;
    let w_px: f32 = w_str.parse().ok()?;
    let h_px: f32 = h_str.parse().ok()?;

    // Physical size: "700mm x 390mm" at the end of the line
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut w_mm = None;
    let mut h_mm = None;
    for i in 1..words.len().saturating_sub(1) {
        if words[i] == "x" {
            w_mm = words[i - 1]
                .strip_suffix("mm")
                .and_then(|n| n.parse::<f32>().ok());
            h_mm = words[i + 1]
                .strip_suffix("mm")
                .and_then(|n| n.parse::<f32>().ok());
        }
    }
    let w_mm = w_mm.filter(|&v| v > 0.0)?;
    let h_mm = h_mm.filter(|&v| v > 0.0)?;

    let h_dpi = w_px / (w_mm / 25.4);
    let v_dpi = h_px / (h_mm / 25.4);
    let avg = (h_dpi + v_dpi) / 2.0;

    if avg > 0.0 { Some(avg) } else { None }
}

#[cfg(all(test, target_os = "linux", any(feature = "kde", feature = "portal")))]
#[allow(clippy::unwrap_used)]
mod xrandr_dpi_tests {
    use super::parse_xrandr_dpi;

    #[test]
    fn primary_4k_display() {
        // Real xrandr output: 4K display at 700mm wide
        let output = "Screen 0: minimum 16 x 16, current 3840 x 2160, maximum 32767 x 32767\n\
                       DP-1 connected primary 3840x2160+0+0 (normal left inverted right x axis y axis) 700mm x 390mm\n\
                          3840x2160     60.00*+\n";
        let dpi = parse_xrandr_dpi(output).unwrap();
        // 3840/(700/25.4) = 139.3, 2160/(390/25.4) = 140.7, avg ~140
        assert!((dpi - 140.0).abs() < 1.0, "expected ~140 DPI, got {dpi}");
    }

    #[test]
    fn standard_1080p_display() {
        let output = "DP-2 connected primary 1920x1080+0+0 (normal) 530mm x 300mm\n";
        let dpi = parse_xrandr_dpi(output).unwrap();
        // 1920/(530/25.4) = 92.0, 1080/(300/25.4) = 91.4, avg ~91.7
        assert!((dpi - 92.0).abs() < 1.0, "expected ~92 DPI, got {dpi}");
    }

    #[test]
    fn no_primary_falls_back_to_first_connected() {
        let output = "HDMI-1 connected 1920x1080+0+0 (normal) 480mm x 270mm\n\
                       DP-1 disconnected\n";
        let dpi = parse_xrandr_dpi(output).unwrap();
        assert!(dpi > 90.0 && dpi < 110.0, "expected ~100 DPI, got {dpi}");
    }

    #[test]
    fn disconnected_only_returns_none() {
        let output = "DP-1 disconnected\nHDMI-1 disconnected\n";
        assert!(parse_xrandr_dpi(output).is_none());
    }

    #[test]
    fn missing_physical_dimensions_returns_none() {
        // No "NNNmm x NNNmm" in the line
        let output = "DP-1 connected primary 1920x1080+0+0 (normal)\n";
        assert!(parse_xrandr_dpi(output).is_none());
    }

    #[test]
    fn zero_mm_returns_none() {
        let output = "DP-1 connected primary 1920x1080+0+0 (normal) 0mm x 0mm\n";
        assert!(parse_xrandr_dpi(output).is_none());
    }

    #[test]
    fn empty_output_returns_none() {
        assert!(parse_xrandr_dpi("").is_none());
    }
}

/// Detect the font DPI for the current system.
///
/// Used by [`ThemeVariant::into_resolved()`] as a fallback when no OS reader
/// has provided `font_dpi`. Returns the platform-appropriate DPI for
/// converting typographic points to logical pixels.
///
/// - **Linux (KDE)**: `forceFontDPI` from kdeglobals/kcmfontsrc → `Xft.dpi` → xrandr → 96.0
/// - **Linux (other)**: `Xft.dpi` → xrandr → 96.0
/// - **macOS**: 72.0 (Apple coordinate system: 1pt = 1px)
/// - **Windows**: `GetDpiForSystem()` → 96.0
/// - **Other**: 96.0
#[allow(unreachable_code)]
fn detect_system_font_dpi() -> f32 {
    #[cfg(target_os = "macos")]
    {
        return 72.0;
    }

    #[cfg(all(target_os = "windows", feature = "windows"))]
    {
        return crate::windows::read_dpi() as f32;
    }

    // KDE: check forceFontDPI first (same chain as the KDE reader)
    #[cfg(all(target_os = "linux", feature = "kde"))]
    {
        if let Some(dpi) = read_kde_force_font_dpi() {
            return dpi;
        }
    }

    #[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]
    {
        if let Some(dpi) = read_xft_dpi() {
            return dpi;
        }
        if let Some(dpi) = detect_physical_dpi() {
            return dpi;
        }
    }

    96.0
}

/// Read KDE's `forceFontDPI` from kdeglobals or kcmfontsrc.
///
/// This mirrors the first step of [`crate::kde::detect_font_dpi()`] so that
/// standalone preset loading (via [`ThemeVariant::into_resolved()`]) uses the
/// same DPI as the full KDE reader pipeline.
#[cfg(all(target_os = "linux", feature = "kde"))]
fn read_kde_force_font_dpi() -> Option<f32> {
    // Try kdeglobals [General] forceFontDPI
    let path = crate::kde::kdeglobals_path();
    if let Ok(content) = std::fs::read_to_string(&path) {
        let mut ini = crate::kde::create_kde_parser();
        if ini.read(content).is_ok()
            && let Some(dpi_str) = ini.get("General", "forceFontDPI")
            && let Ok(dpi) = dpi_str.trim().parse::<f32>()
            && dpi > 0.0
        {
            return Some(dpi);
        }
    }
    // Try kcmfontsrc [General] forceFontDPI
    if let Some(dpi_str) = crate::kde::read_kcmfontsrc_key("General", "forceFontDPI")
        && let Ok(dpi) = dpi_str.trim().parse::<f32>()
        && dpi > 0.0
    {
        return Some(dpi);
    }
    None
}

/// Inner detection logic for [`system_is_dark()`].
///
/// Separated from the public function to allow caching via `OnceLock`.
#[allow(unreachable_code)]
fn detect_is_dark_inner() -> bool {
    #[cfg(target_os = "linux")]
    {
        // Check GTK_THEME env var (works across all GTK-based DEs)
        if let Ok(gtk_theme) = std::env::var("GTK_THEME") {
            let lower = gtk_theme.to_lowercase();
            if lower.ends_with(":dark") || lower.contains("-dark") {
                return true;
            }
        }

        // gsettings color-scheme (with 2-second timeout)
        if let Some(val) =
            run_gsettings_with_timeout(&["get", "org.gnome.desktop.interface", "color-scheme"])
        {
            if val.contains("prefer-dark") {
                return true;
            }
            if val.contains("prefer-light") || val.contains("default") {
                return false;
            }
        }

        // Fallback: read KDE's kdeglobals background luminance
        #[cfg(feature = "kde")]
        {
            let path = crate::kde::kdeglobals_path();
            if let Ok(content) = std::fs::read_to_string(&path) {
                let mut ini = crate::kde::create_kde_parser();
                if ini.read(content).is_ok() {
                    return crate::kde::is_dark_theme(&ini);
                }
            }
        }

        // Fallback: gtk-3.0/settings.ini for DEs that set the GTK dark preference
        let config_home = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_default();
            format!("{home}/.config")
        });
        let ini_path = format!("{config_home}/gtk-3.0/settings.ini");
        if let Ok(content) = std::fs::read_to_string(&ini_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("gtk-application-prefer-dark-theme")
                    && let Some(val) = trimmed.split('=').nth(1)
                    && (val.trim() == "1" || val.trim().eq_ignore_ascii_case("true"))
                {
                    return true;
                }
            }
        }

        false
    }

    #[cfg(target_os = "macos")]
    {
        // AppleInterfaceStyle is "Dark" when dark mode is active.
        // The key is absent in light mode, so any failure means light.
        #[cfg(feature = "macos")]
        {
            use objc2_foundation::NSUserDefaults;
            let defaults = NSUserDefaults::standardUserDefaults();
            let key = objc2_foundation::ns_string!("AppleInterfaceStyle");
            if let Some(value) = defaults.stringForKey(key) {
                return value.to_string().eq_ignore_ascii_case("dark");
            }
            return false;
        }
        #[cfg(not(feature = "macos"))]
        {
            if let Ok(output) = std::process::Command::new("defaults")
                .args(["read", "-g", "AppleInterfaceStyle"])
                .output()
                && output.status.success()
            {
                let val = String::from_utf8_lossy(&output.stdout);
                return val.trim().eq_ignore_ascii_case("dark");
            }
            return false;
        }
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        {
            // BT.601 luminance: light foreground indicates dark background.
            let Ok(settings) = ::windows::UI::ViewManagement::UISettings::new() else {
                return false;
            };
            let Ok(fg) =
                settings.GetColorValue(::windows::UI::ViewManagement::UIColorType::Foreground)
            else {
                return false;
            };
            let luma = 0.299 * (fg.R as f32) + 0.587 * (fg.G as f32) + 0.114 * (fg.B as f32);
            return luma > 128.0;
        }
        #[cfg(not(feature = "windows"))]
        return false;
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        false
    }
}

static CACHED_REDUCED_MOTION: std::sync::RwLock<Option<bool>> = std::sync::RwLock::new(None);

/// Query whether the user prefers reduced motion.
///
/// Returns `true` when the OS accessibility setting indicates animations
/// should be reduced or disabled. Returns `false` (allow animations) on
/// unsupported platforms or when the query fails.
///
/// # Caching
///
/// The result is cached after the first call and reused on subsequent calls.
/// Call [`invalidate_caches()`] to clear the cached value so the next call
/// re-queries the OS. For live accessibility-change tracking, subscribe to
/// OS accessibility events and call `invalidate_caches()` when notified.
///
/// # Platform Behavior
///
/// - **Linux:** Queries `gsettings get org.gnome.desktop.interface enable-animations`.
///   Returns `true` when animations are disabled (`enable-animations` is `false`).
/// - **macOS:** Queries `NSWorkspace.accessibilityDisplayShouldReduceMotion`
///   (requires `macos` feature).
/// - **Windows:** Queries `UISettings.AnimationsEnabled()` (requires `windows` feature).
/// - **Other platforms:** Returns `false`.
///
/// # Examples
///
/// ```
/// let reduced = native_theme::prefers_reduced_motion();
/// // On this platform, the result depends on OS accessibility settings.
/// // The function always returns a bool (false on unsupported platforms).
/// assert!(reduced == true || reduced == false);
/// ```
#[must_use = "this returns whether reduced motion is preferred"]
pub fn prefers_reduced_motion() -> bool {
    if let Ok(guard) = CACHED_REDUCED_MOTION.read()
        && let Some(v) = *guard
    {
        return v;
    }
    let value = detect_reduced_motion_inner();
    if let Ok(mut guard) = CACHED_REDUCED_MOTION.write() {
        *guard = Some(value);
    }
    value
}

/// Detect whether the user prefers reduced motion without caching.
///
/// Unlike [`prefers_reduced_motion()`], this function queries the OS every time
/// it is called and never caches the result. Use this when polling for
/// accessibility preference changes.
///
/// See [`prefers_reduced_motion()`] for platform behavior details.
#[must_use = "this returns whether reduced motion is preferred"]
pub fn detect_reduced_motion() -> bool {
    detect_reduced_motion_inner()
}

/// Inner detection logic for [`prefers_reduced_motion()`].
///
/// Separated from the public function to allow caching via `OnceLock`.
#[allow(unreachable_code)]
fn detect_reduced_motion_inner() -> bool {
    #[cfg(target_os = "linux")]
    {
        // gsettings boolean output is bare "true\n" or "false\n" (no quotes)
        // enable-animations has INVERTED semantics: false => reduced motion preferred
        if let Some(val) =
            run_gsettings_with_timeout(&["get", "org.gnome.desktop.interface", "enable-animations"])
        {
            return val.trim() == "false";
        }
        false
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        {
            let workspace = objc2_app_kit::NSWorkspace::sharedWorkspace();
            // Direct semantics: true = reduce motion preferred (no inversion needed)
            return workspace.accessibilityDisplayShouldReduceMotion();
        }
        #[cfg(not(feature = "macos"))]
        return false;
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        {
            let Ok(settings) = ::windows::UI::ViewManagement::UISettings::new() else {
                return false;
            };
            // AnimationsEnabled has INVERTED semantics: false => reduced motion preferred
            return match settings.AnimationsEnabled() {
                Ok(enabled) => !enabled,
                Err(_) => false,
            };
        }
        #[cfg(not(feature = "windows"))]
        return false;
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        false
    }
}

// === Crate-internal accessors ===

/// Run `gsettings get <schema> <key>` with timeout.
#[cfg(target_os = "linux")]
pub(crate) fn gsettings_get(schema: &str, key: &str) -> Option<String> {
    run_gsettings_with_timeout(&["get", schema, key])
}

/// Read Xft.dpi from X resources.
#[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]
pub(crate) fn xft_dpi() -> Option<f32> {
    read_xft_dpi()
}

/// Detect physical DPI from xrandr.
#[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]
pub(crate) fn physical_dpi() -> Option<f32> {
    detect_physical_dpi()
}

/// Detect the system font DPI (combining multiple sources).
pub(crate) fn system_font_dpi() -> f32 {
    detect_system_font_dpi()
}

#[cfg(all(test, target_os = "linux", any(feature = "kde", feature = "portal")))]
#[allow(clippy::unwrap_used)]
mod xrandr_dpi_tests {
    use super::parse_xrandr_dpi;

    #[test]
    fn primary_4k_display() {
        // Real xrandr output: 4K display at 700mm wide
        let output = "Screen 0: minimum 16 x 16, current 3840 x 2160, maximum 32767 x 32767\n\
                       DP-1 connected primary 3840x2160+0+0 (normal left inverted right x axis y axis) 700mm x 390mm\n\
                          3840x2160     60.00*+\n";
        let dpi = parse_xrandr_dpi(output).unwrap();
        // 3840/(700/25.4) = 139.3, 2160/(390/25.4) = 140.7, avg ~140
        assert!((dpi - 140.0).abs() < 1.0, "expected ~140 DPI, got {dpi}");
    }

    #[test]
    fn standard_1080p_display() {
        let output = "DP-2 connected primary 1920x1080+0+0 (normal) 530mm x 300mm\n";
        let dpi = parse_xrandr_dpi(output).unwrap();
        // 1920/(530/25.4) = 92.0, 1080/(300/25.4) = 91.4, avg ~91.7
        assert!((dpi - 92.0).abs() < 1.0, "expected ~92 DPI, got {dpi}");
    }

    #[test]
    fn no_primary_falls_back_to_first_connected() {
        let output = "HDMI-1 connected 1920x1080+0+0 (normal) 480mm x 270mm\n\
                       DP-1 disconnected\n";
        let dpi = parse_xrandr_dpi(output).unwrap();
        assert!(dpi > 90.0 && dpi < 110.0, "expected ~100 DPI, got {dpi}");
    }

    #[test]
    fn disconnected_only_returns_none() {
        let output = "DP-1 disconnected\nHDMI-1 disconnected\n";
        assert!(parse_xrandr_dpi(output).is_none());
    }

    #[test]
    fn missing_physical_dimensions_returns_none() {
        // No "NNNmm x NNNmm" in the line
        let output = "DP-1 connected primary 1920x1080+0+0 (normal)\n";
        assert!(parse_xrandr_dpi(output).is_none());
    }

    #[test]
    fn zero_mm_returns_none() {
        let output = "DP-1 connected primary 1920x1080+0+0 (normal) 0mm x 0mm\n";
        assert!(parse_xrandr_dpi(output).is_none());
    }

    #[test]
    fn empty_output_returns_none() {
        assert!(parse_xrandr_dpi("").is_none());
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod reduced_motion_tests {
    use super::*;

    #[test]
    fn prefers_reduced_motion_smoke_test() {
        // Smoke test: function should not panic on any platform.
        // Cannot assert a specific value because OnceLock caches the first call
        // and CI environments have varying accessibility settings.
        let _result = prefers_reduced_motion();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn detect_reduced_motion_inner_linux() {
        // Bypass OnceLock to test actual detection logic.
        // On CI without gsettings, returns false (animations enabled).
        // On developer machines, depends on accessibility settings.
        let result = detect_reduced_motion_inner();
        // Just verify it returns a bool without panicking.
        let _ = result;
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn detect_reduced_motion_inner_macos() {
        let result = detect_reduced_motion_inner();
        let _ = result;
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn detect_reduced_motion_inner_windows() {
        let result = detect_reduced_motion_inner();
        let _ = result;
    }
}
