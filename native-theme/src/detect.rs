//! OS detection: dark mode, reduced motion, accessibility preferences, icon
//! theme and set, desktop environment.

use arc_swap::ArcSwapOption;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

mod icon_theme;

pub(crate) use icon_theme::IconThemeFailure;
pub use icon_theme::{detect_icon_theme, system_icon_set, system_icon_theme};

/// Timeout for subprocess commands ([`run_with_timeout`]: gsettings, xrdb,
/// xrandr, xfconf-query, fc-match, macOS `defaults`).
///
/// Prevents D-Bus-dependent tools from blocking indefinitely when the
/// session bus is unresponsive or the display server is unavailable.
#[cfg(unix)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub(crate) const SUBPROCESS_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(2);

/// Desktop environments recognized on Linux.
#[cfg(target_os = "linux")]
#[non_exhaustive]
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
    /// Hyprland Wayland compositor.
    Hyprland,
    /// Sway Wayland compositor (i3-compatible).
    Sway,
    /// River Wayland compositor.
    River,
    /// Niri scrollable Wayland compositor.
    Niri,
    /// Wayfire scene-graph Wayland compositor (wlroots-based).
    Wayfire,
    /// COSMIC desktop environment (System76).
    CosmicDe,
    /// Unrecognized or unset desktop environment.
    Unknown,
}

/// Read the `XDG_CURRENT_DESKTOP` environment variable, returning an
/// empty string if unset or invalid UTF-8.
#[cfg(target_os = "linux")]
pub(crate) fn xdg_current_desktop() -> String {
    std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
}

/// Detect the current Linux desktop environment.
///
/// Reads `XDG_CURRENT_DESKTOP` and returns the recognized desktop.
/// Returns [`LinuxDesktop::Unknown`] if the variable is unset or
/// contains no recognized value.
///
/// For testable parsing without environment access, use the
/// `pub` [`parse_linux_desktop()`] function instead.
///
/// # Examples
///
/// ```no_run
/// use native_theme::detect::{detect_linux_desktop, LinuxDesktop};
///
/// let de = detect_linux_desktop();
/// match de {
///     LinuxDesktop::Kde => println!("KDE Plasma"),
///     LinuxDesktop::Gnome => println!("GNOME"),
///     _ => println!("Other: {de:?}"),
/// }
/// ```
#[cfg(target_os = "linux")]
#[must_use]
pub fn detect_linux_desktop() -> LinuxDesktop {
    parse_linux_desktop(&xdg_current_desktop())
}

/// Parse `XDG_CURRENT_DESKTOP` (a colon-separated list) and return
/// the recognized desktop environment.
///
/// Checks components in order; first recognized DE wins. Budgie is checked
/// before GNOME because Budgie sets `Budgie:GNOME`.
#[cfg(target_os = "linux")]
#[must_use]
pub fn parse_linux_desktop(xdg_current_desktop: &str) -> LinuxDesktop {
    for component in xdg_current_desktop.split(':') {
        match component {
            "KDE" => return LinuxDesktop::Kde,
            "Budgie" => return LinuxDesktop::Budgie,
            "GNOME" => return LinuxDesktop::Gnome,
            "XFCE" => return LinuxDesktop::Xfce,
            "X-Cinnamon" | "Cinnamon" => return LinuxDesktop::Cinnamon,
            "MATE" => return LinuxDesktop::Mate,
            "LXQt" => return LinuxDesktop::LxQt,
            "Hyprland" => return LinuxDesktop::Hyprland,
            "sway" => return LinuxDesktop::Sway,
            "river" => return LinuxDesktop::River,
            "niri" => return LinuxDesktop::Niri,
            "Wayfire" => return LinuxDesktop::Wayfire,
            "COSMIC" => return LinuxDesktop::CosmicDe,
            _ => {}
        }
    }
    LinuxDesktop::Unknown
}

/// The desktop a theme is read by: `raw` ([`parse_linux_desktop`] of
/// `XDG_CURRENT_DESKTOP`) where it names one; for `Unknown`, the desktop of
/// the portal backend (`portal_backend`), else KDE where `kdeglobals_exists`,
/// else `Unknown`. The reader, the theme watcher, the icon-theme detection
/// and the font DPI all decide by it ([`DetectionContext::refined_linux_desktop`]),
/// so they agree. `kdeglobals_exists` is asked only where nothing before it
/// decides.
#[cfg(target_os = "linux")]
pub(crate) fn refine_linux_desktop(
    raw: LinuxDesktop,
    portal_backend: Option<LinuxDesktop>,
    kdeglobals_exists: impl FnOnce() -> bool,
) -> LinuxDesktop {
    refine_desktop(raw, portal_backend, kdeglobals_exists).desktop
}

/// A refined desktop ([`refine_linux_desktop`]), and whether only
/// `kdeglobals` named it.
#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RefinedDesktop {
    /// The desktop.
    pub(crate) desktop: LinuxDesktop,
    /// KDE named by the user's `kdeglobals` alone: `XDG_CURRENT_DESKTOP`
    /// names no desktop and no portal backend answered.
    pub(crate) by_kdeglobals: bool,
}

/// [`refine_linux_desktop`], with what named the desktop.
#[cfg(target_os = "linux")]
pub(crate) fn refine_desktop(
    raw: LinuxDesktop,
    portal_backend: Option<LinuxDesktop>,
    kdeglobals_exists: impl FnOnce() -> bool,
) -> RefinedDesktop {
    let (desktop, by_kdeglobals) = match (raw, portal_backend) {
        (LinuxDesktop::Unknown, Some(backend)) => (backend, false),
        (LinuxDesktop::Unknown, None) if kdeglobals_exists() => (LinuxDesktop::Kde, true),
        (other, _) => (other, false),
    };
    RefinedDesktop {
        desktop,
        by_kdeglobals,
    }
}

/// Whether the user's `kdeglobals` exists, with the `kde` feature; `false`
/// without it, as no KDE file is read then.
#[cfg(target_os = "linux")]
fn kdeglobals_exists() -> bool {
    #[cfg(feature = "kde")]
    {
        crate::kde::kdeglobals_path().is_some_and(|path| path.exists())
    }
    #[cfg(not(feature = "kde"))]
    {
        false
    }
}

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
/// For live dark-mode tracking, use `watch::on_theme_change` (feature
/// `watch`): on each event call [`invalidate_caches()`] and re-read
/// [`crate::SystemTheme::from_system()`].
///
/// # Platform Behavior
///
/// - **Linux:** Checks `GTK_THEME` for `:dark` or `-dark`; on KDE (with the
///   `kde` feature; the desktop the KDE reader is chosen by, so an
///   unrecognised one with a KDE portal backend or a `kdeglobals` counts)
///   reads `kdeglobals`' background luminance; then
///   `gsettings` `color-scheme` (2-second timeout); then `kdeglobals` on
///   other desktops (with `kde`); finally `gtk-3.0/settings.ini`
///   `gtk-application-prefer-dark-theme`.
/// - **macOS:** Reads `AppleInterfaceStyle` via `NSUserDefaults` (with
///   `macos` feature) or `defaults` subprocess (without).
/// - **Windows:** Checks foreground color luminance from `UISettings` via
///   BT.601 coefficients (requires `windows` feature).
/// - **Other platforms / missing features:** Returns `false` (light).
#[must_use]
pub fn system_is_dark() -> bool {
    system().is_dark()
}

/// Reset all process-wide caches so the next call to [`system_is_dark()`],
/// [`prefers_reduced_motion()`], [`accessibility_preferences()`], or
/// [`system_icon_theme()`] re-queries the OS; a cached failure to detect the
/// icon theme is cleared too, and so is the font DPI that
/// [`ResolutionContext::from_system()`](crate::ResolutionContext::from_system)
/// and the KDE reader use.
///
/// Call this when you detect that the user has changed system settings (e.g.,
/// dark mode toggle, icon theme switch, accessibility preferences).
///
/// The `detect_*()` family of functions are unaffected — they always query
/// the OS directly. The installed freedesktop icon themes are not re-read:
/// the icon lookup reads them once per process, and so do
/// [`is_freedesktop_theme_available()`](crate::icons::is_freedesktop_theme_available)
/// and [`list_freedesktop_themes()`](crate::icons::list_freedesktop_themes).
pub fn invalidate_caches() {
    system().invalidate_all();
}

/// Detect whether the system is using a dark color scheme without caching.
///
/// Unlike [`system_is_dark()`], this function queries the OS every time it is
/// called and never caches the result. Use this when polling for theme changes
/// or implementing live dark-mode tracking.
///
/// See [`system_is_dark()`] for platform behavior details.
#[must_use]
pub fn detect_is_dark() -> bool {
    detect_is_dark_inner()
}

/// The longest pause between two polls of a running command.
#[cfg(unix)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
const POLL_INTERVAL_CAP: std::time::Duration = std::time::Duration::from_millis(50);

/// The pauses between polls of a running command: 1 ms, doubling up to
/// [`POLL_INTERVAL_CAP`], so a command that exits in a few milliseconds is
/// seen to exit within a few milliseconds, and a slow one costs a poll per
/// 50 ms.
#[cfg(unix)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn poll_intervals() -> impl Iterator<Item = std::time::Duration> {
    std::iter::successors(Some(std::time::Duration::from_millis(1)), |pause| {
        Some(pause.saturating_mul(2).min(POLL_INTERVAL_CAP))
    })
}

/// Run `program` with `args` and its stdin closed, waiting up to `timeout`
/// for it to exit and for its stdout and stderr to close.
///
/// Its output is what [`std::process::Command::output`] gives, an
/// unsuccessful exit included. Both pipes are read while the command runs,
/// each on a thread of its own, so a command that prints more than a pipe
/// holds (65,536 bytes on Linux, pipe(7)) is not stalled until it is
/// killed. The error is the spawn's, a reader thread's that cannot be
/// started, or one of kind [`std::io::ErrorKind::TimedOut`] when the command
/// outlives `timeout` or a process it started still holds its output open
/// at `timeout`. A command that has not exited is killed and reaped on
/// every error; one the kill fails on is left unreaped rather than waited
/// for, and a reader still blocked at `timeout` is left to end when the pipe
/// closes, so `timeout` stays a bound on the call. The command is polled at
/// [`poll_intervals`].
#[cfg(unix)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub(crate) fn run_with_timeout(
    program: &str,
    args: &[&str],
    timeout: std::time::Duration,
) -> std::io::Result<std::process::Output> {
    use std::process::{Child, Command, Stdio};
    use std::sync::mpsc::{Receiver, RecvTimeoutError};
    use std::time::Instant;

    fn kill_and_reap(child: &mut Child) {
        if child.kill().is_ok() {
            let _ = child.wait();
        }
    }

    let timed_out = || {
        std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            format!("timed out after {timeout:?}"),
        )
    };

    let start = Instant::now();
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let readers = read_on_a_thread(child.stdout.take())
        .and_then(|stdout| Ok((stdout, read_on_a_thread(child.stderr.take())?)));
    let (stdout, stderr) = match readers {
        Ok(readers) => readers,
        Err(e) => {
            kill_and_reap(&mut child);
            return Err(e);
        }
    };

    let mut intervals = poll_intervals();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {
                let elapsed = start.elapsed();
                if elapsed >= timeout {
                    kill_and_reap(&mut child);
                    return Err(timed_out());
                }
                let pause = intervals.next().unwrap_or(POLL_INTERVAL_CAP);
                std::thread::sleep(pause.min(timeout.saturating_sub(elapsed)));
            }
            Err(e) => {
                kill_and_reap(&mut child);
                return Err(e);
            }
        }
    };

    let received = |reader: Receiver<Vec<u8>>| {
        reader
            .recv_timeout(timeout.saturating_sub(start.elapsed()))
            .map_err(|e| match e {
                RecvTimeoutError::Timeout => timed_out(),
                RecvTimeoutError::Disconnected => {
                    std::io::Error::other("the output reader stopped without a result")
                }
            })
    };
    Ok(std::process::Output {
        status,
        stdout: received(stdout)?,
        stderr: received(stderr)?,
    })
}

/// Read `pipe` to its end on a new thread, which sends what it read; an
/// absent pipe is read as empty. The error is the thread's spawn error.
#[cfg(unix)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn read_on_a_thread(
    pipe: Option<impl std::io::Read + Send + 'static>,
) -> std::io::Result<std::sync::mpsc::Receiver<Vec<u8>>> {
    let (sender, receiver) = std::sync::mpsc::channel();
    match pipe {
        Some(mut pipe) => {
            std::thread::Builder::new()
                .name("native-theme-subprocess-output".to_string())
                .spawn(move || {
                    let mut read = Vec::new();
                    let _ = pipe.read_to_end(&mut read);
                    let _ = sender.send(read);
                })?;
        }
        None => {
            let _ = sender.send(Vec::new());
        }
    }
    Ok(receiver)
}

/// The stdout of `program` run with `args` ([`run_with_timeout`]) when it
/// exits successfully within `timeout` and prints UTF-8; `None` otherwise.
#[cfg(unix)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub(crate) fn stdout_with_timeout(
    program: &str,
    args: &[&str],
    timeout: std::time::Duration,
) -> Option<String> {
    let output = run_with_timeout(program, args, timeout).ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout).ok()
}

/// The user's config directory: `$XDG_CONFIG_HOME`, else `$HOME/.config`;
/// `None` without either ([`xdg_config_home_from`] of the environment).
#[cfg(target_os = "linux")]
pub(crate) fn xdg_config_home() -> Option<std::path::PathBuf> {
    xdg_config_home_from(
        std::env::var_os("XDG_CONFIG_HOME").as_deref(),
        std::env::var_os("HOME").as_deref(),
    )
}

/// The config directory `$XDG_CONFIG_HOME` (`var`) names: itself when it is
/// absolute, else `<home>/.config` when `home` is non-empty, else `None`.
///
/// The XDG Base Directory Specification 0.8: "If `$XDG_CONFIG_HOME` is
/// either not set or empty, a default equal to `$HOME/.config` should be
/// used", and "All paths set in these environment variables must be
/// absolute. If an implementation encounters a relative path in any of
/// these variables it should consider the path invalid and ignore it"
/// (<https://specifications.freedesktop.org/basedir-spec/0.8/>).
#[cfg(target_os = "linux")]
fn xdg_config_home_from(
    var: Option<&std::ffi::OsStr>,
    home: Option<&std::ffi::OsStr>,
) -> Option<std::path::PathBuf> {
    var.map(std::path::PathBuf::from)
        .filter(|dir| dir.is_absolute())
        .or_else(|| {
            home.filter(|home| !home.is_empty())
                .map(|home| std::path::Path::new(home).join(".config"))
        })
}

/// Why [`xdg_config_home`] is `None`.
#[cfg(target_os = "linux")]
pub(crate) const CONFIG_HOME_UNRESOLVED: &str = "the config directory is unresolved: \
    $XDG_CONFIG_HOME is not an absolute path and $HOME is unset or empty";

/// Run `gsettings get <schema> <key>`, waiting up to `timeout`.
///
/// Returns the trimmed stdout on success; `None` if the command fails, times
/// out, or produces empty output. The timeout keeps gsettings from blocking
/// indefinitely when D-Bus is unresponsive.
#[cfg(target_os = "linux")]
pub(crate) fn gsettings_get(
    schema: &str,
    key: &str,
    timeout: std::time::Duration,
) -> Option<String> {
    let out = stdout_with_timeout("gsettings", &["get", schema, key], timeout)?;
    let trimmed = out.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Read `Xft.dpi` from X resources via `xrdb -query`.
///
/// Returns `None` if xrdb is not installed, times out (2 seconds),
/// or the output does not contain a valid positive `Xft.dpi` value.
#[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]
fn read_xft_dpi() -> Option<f32> {
    let out = stdout_with_timeout("xrdb", &["-query"], SUBPROCESS_TIMEOUT)?;
    // Parse "Xft.dpi:\t96" from multi-line output
    for line in out.lines() {
        if let Some(rest) = line.strip_prefix("Xft.dpi:")
            && let Ok(dpi) = rest.trim().parse::<f32>()
            && dpi > 0.0
        {
            return Some(dpi);
        }
    }
    None
}

/// Detect physical DPI from display hardware via `xrandr`.
///
/// Parses the primary connected output's resolution and physical dimensions
/// to compute DPI. Falls back to the first connected output if no primary
/// is found. Returns `None` if `xrandr` is unavailable, times out (2 seconds),
/// or the output cannot be parsed.
///
/// This is a last-resort fallback: prefer `forceFontDPI` (KDE), `Xft.dpi`
/// (X resources) before calling this.
#[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]
fn detect_physical_dpi() -> Option<f32> {
    let out = stdout_with_timeout("xrandr", &[], SUBPROCESS_TIMEOUT)?;
    parse_xrandr_dpi(&out)
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
    for window in words.windows(3) {
        let [prev, cur, next] = window else { continue };
        if *cur == "x" {
            w_mm = prev.strip_suffix("mm").and_then(|n| n.parse::<f32>().ok());
            h_mm = next.strip_suffix("mm").and_then(|n| n.parse::<f32>().ok());
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

/// GNOME's font DPI: 96. gnome-settings-daemon lays fonts out at a fixed 96
/// (`DPI_FALLBACK`) times `text-scaling-factor`, and GTK on Wayland at the same
/// 96 × factor (gnome-settings-daemon 51.0
/// `plugins/xsettings/gsd-xsettings-manager.c:243`, `:583-595`; GTK 4.24.1
/// `gdk/wayland/gdksettings-wayland.c:166-168`). The factor reaches
/// [`AccessibilityPreferences::text_scaling_factor`](crate::AccessibilityPreferences::text_scaling_factor),
/// which the connectors apply, on a Wayland session, and the toolkit's
/// `Xft.dpi` scale factor on an X11 one, so the font DPI leaves it out.
/// `Xft.dpi` is not read on GNOME: gnome-settings-daemon sets it to
/// 96 × factor × the X11 UI scale (`:665-668`, `:797-798`).
#[cfg(all(target_os = "linux", feature = "portal"))]
pub(crate) const GNOME_FONT_DPI: f32 = 96.0;

/// The font DPI a desktop fixes: [`GNOME_FONT_DPI`] on GNOME and Budgie, the
/// desktops the GNOME reader reads (`pipeline::select_reader`, both deciding
/// by the refined desktop, [`refine_linux_desktop`]); `None` elsewhere, where
/// the detection chain decides.
#[cfg(all(target_os = "linux", feature = "portal"))]
fn desktop_font_dpi(desktop: LinuxDesktop) -> Option<f32> {
    matches!(desktop, LinuxDesktop::Gnome | LinuxDesktop::Budgie).then_some(GNOME_FONT_DPI)
}

/// Detect the font DPI for the current system.
///
/// Used, through [`system_font_dpi`], by
/// [`ResolutionContext::from_system()`](crate::ResolutionContext::from_system)
/// where no OS reader has provided `font_dpi`, and by the KDE reader as its
/// own. Returns the platform-appropriate DPI for converting typographic
/// points to logical pixels.
///
/// - **Linux, GNOME or Budgie, `portal` feature**: 96.0 (`GNOME_FONT_DPI`);
///   the desktop is [`DetectionContext::refined_linux_desktop_sync`]'s, so an
///   unrecognised one counts as the reader's
/// - **Linux, `kde` feature** (any other desktop): on an X11 session
///   `kcmfonts`' `forceFontDPI` (`kde_force_font_dpi`), then `Xft.dpi` →
///   xrandr → 96.0
/// - **Linux, `portal` feature without `kde`** (any other desktop): `Xft.dpi` → xrandr → 96.0;
///   neither feature: 96.0
/// - **macOS**: 72.0 (Apple coordinate system: 1pt = 1px)
/// - **Windows**: 96.0, the Windows reader's `font_dpi`: a point is 96/72
///   logical (effective) pixels at any display scale
/// - **Other**: 96.0
#[allow(unreachable_code)]
fn detect_system_font_dpi() -> f32 {
    #[cfg(target_os = "macos")]
    {
        return 72.0;
    }

    #[cfg(all(target_os = "windows", feature = "windows"))]
    {
        return crate::windows::LOGICAL_DPI as f32;
    }

    // GNOME: a fixed 96, the text scale carried apart (GNOME_FONT_DPI)
    #[cfg(all(target_os = "linux", feature = "portal"))]
    {
        if let Some(dpi) = desktop_font_dpi(system().refined_linux_desktop_sync()) {
            return dpi;
        }
    }

    #[cfg(all(target_os = "linux", feature = "kde"))]
    {
        if let Some(dpi) = kde_force_font_dpi(session_type().as_deref(), || {
            crate::kde::read_kcmfonts_key("General", "forceFontDPI")
        }) {
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

/// The session type, `$XDG_SESSION_TYPE` (`x11`, `wayland`, …).
#[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]
pub(crate) fn session_type() -> Option<String> {
    std::env::var("XDG_SESSION_TYPE").ok()
}

/// Whether a session of `session_type` reads KDE's `forceFontDPI`: an X11
/// session only. Plasma 6.7 reads the key on X11 alone (its fonts KCM shows
/// it only for `xcb`, krdb hands it to X11 as `Xft.dpi` only off Wayland,
/// `docs/platform-facts.md:713-714`), and Plasma 6.8 drops it; a Wayland
/// session that kept the key from an X11 one would otherwise scale fonts
/// Plasma does not.
#[cfg(all(target_os = "linux", feature = "kde"))]
pub(crate) fn reads_force_font_dpi(session_type: Option<&str>) -> bool {
    session_type == Some("x11")
}

/// A `forceFontDPI` value as a font DPI: Plasma declares the key a UInt
/// whose default 0 means unset (`docs/platform-facts.md:713`), so it is
/// parsed as a `u32` and 0 is `None`, as is anything that is not a `u32`.
#[cfg(all(target_os = "linux", feature = "kde"))]
pub(crate) fn parse_force_font_dpi(value: &str) -> Option<f32> {
    let dpi = value.trim().parse::<u32>().ok()?;
    (dpi > 0).then_some(dpi as f32)
}

/// KDE's `forceFontDPI` on a session that reads it
/// ([`reads_force_font_dpi`]), from kcmfonts, where [`parse_force_font_dpi`]
/// accepts it. Plasma keeps the key in that file alone: the fonts KCM writes
/// it there and krdb reads it there (`docs/platform-facts.md:713`). The file
/// is not read on any other session.
#[cfg(all(target_os = "linux", feature = "kde"))]
fn kde_force_font_dpi(
    session_type: Option<&str>,
    kcmfonts: impl FnOnce() -> Option<String>,
) -> Option<f32> {
    if !reads_force_font_dpi(session_type) {
        return None;
    }
    kcmfonts().as_deref().and_then(parse_force_font_dpi)
}

/// Whether the colour scheme is dark: the portal's answer when it gave one,
/// else gsettings' `org.gnome.desktop.interface` `color-scheme` read through
/// `gsetting` (`prefer-dark` dark; `prefer-light` and `default` light);
/// `None` when neither answers. [`detect_is_dark`] (no portal) and the GNOME
/// reader both decide through it, so the two cannot disagree.
#[cfg(target_os = "linux")]
pub(crate) fn color_scheme_is_dark(
    portal: Option<bool>,
    gsetting: impl FnOnce(&str, &str) -> Option<String>,
) -> Option<bool> {
    if portal.is_some() {
        return portal;
    }
    let val = gsetting("org.gnome.desktop.interface", "color-scheme")?;
    if val.contains("prefer-dark") {
        Some(true)
    } else if val.contains("prefer-light") || val.contains("default") {
        Some(false)
    } else {
        None
    }
}

/// Dark or light from the desktop's settings: on KDE (`detection`'s refined
/// desktop, the one the KDE reader is chosen by) `kdeglobals` first, as
/// gsettings `color-scheme` is synced by xdg-desktop-portal-kde and can be
/// stale or inverted; elsewhere gsettings (reliable on GNOME and GTK-based
/// desktops), then `kdeglobals`' background luminance.
#[cfg(target_os = "linux")]
fn desktop_is_dark(
    detection: &DetectionContext,
    kdeglobals: impl FnOnce() -> Option<bool>,
    gsettings: impl FnOnce() -> Option<bool>,
) -> Option<bool> {
    if detection.refined_linux_desktop_sync() == LinuxDesktop::Kde {
        kdeglobals().or_else(gsettings)
    } else {
        gsettings().or_else(kdeglobals)
    }
}

/// Inner detection logic for [`system_is_dark()`].
///
/// Separated from the public function to allow caching.
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

        if let Some(dark) = desktop_is_dark(
            system(),
            || {
                #[cfg(feature = "kde")]
                {
                    crate::kde::kdeglobals_is_dark()
                }
                #[cfg(not(feature = "kde"))]
                {
                    None
                }
            },
            || {
                color_scheme_is_dark(None, |schema, key| {
                    gsettings_get(schema, key, SUBPROCESS_TIMEOUT)
                })
            },
        ) {
            return dark;
        }

        // Fallback: gtk-3.0/settings.ini for DEs that set the GTK dark preference
        if let Some(content) = xdg_config_home()
            .and_then(|dir| std::fs::read_to_string(dir.join("gtk-3.0/settings.ini")).ok())
        {
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
            defaults
                .stringForKey(key)
                .is_some_and(|value| value.to_string().eq_ignore_ascii_case("dark"))
        }
        #[cfg(not(feature = "macos"))]
        {
            stdout_with_timeout(
                "defaults",
                &["read", "-g", "AppleInterfaceStyle"],
                SUBPROCESS_TIMEOUT,
            )
            .is_some_and(|value| value.trim().eq_ignore_ascii_case("dark"))
        }
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        {
            let Ok(settings) = ::windows::UI::ViewManagement::UISettings::new() else {
                return false;
            };
            let Ok(fg) =
                settings.GetColorValue(::windows::UI::ViewManagement::UIColorType::Foreground)
            else {
                return false;
            };
            crate::windows::is_dark_mode(&crate::windows::win_color_to_rgba(fg))
        }
        #[cfg(not(feature = "windows"))]
        {
            false
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        false
    }
}

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
/// Each platform's read is the one its theme reader makes, so the result is
/// the `reduce_motion` of
/// [`AccessibilityPreferences::from_system`](crate::AccessibilityPreferences::from_system)
/// and of [`SystemTheme::from_system`](crate::SystemTheme::from_system)'s
/// accessibility block.
///
/// - **Linux:** by the desktop the reader is chosen by. KDE (with the `kde`
///   feature): `kdeglobals`' `[KDE] AnimationDurationFactor` is 0. GNOME and
///   Budgie: the portal's `reduced-motion` (with the `portal` feature, 2-second
///   deadline), else `gsettings get org.gnome.desktop.interface enable-animations`
///   is `false` (2-second timeout). Other desktops, and KDE without the `kde`
///   feature: that gsettings key alone.
/// - **macOS:** Queries `NSWorkspace.accessibilityDisplayShouldReduceMotion`
///   (requires `macos` feature).
/// - **Windows:** Queries `SystemParametersInfoW(SPI_GETCLIENTAREAANIMATION)`;
///   `true` when client-area animations are off (requires `windows` feature).
/// - **Other platforms:** Returns `false`.
///
/// # Examples
///
/// ```
/// let reduced = native_theme::detect::prefers_reduced_motion();
/// // On this platform, the result depends on OS accessibility settings.
/// // The function always returns a bool (false on unsupported platforms).
/// assert!(reduced == true || reduced == false);
/// ```
#[must_use]
pub fn prefers_reduced_motion() -> bool {
    system().prefers_reduced_motion()
}

/// Detect whether the user prefers reduced motion without caching.
///
/// Unlike [`prefers_reduced_motion()`], this function queries the OS every time
/// it is called and never caches the result. Use this when polling for
/// accessibility preference changes.
///
/// See [`prefers_reduced_motion()`] for platform behavior details.
#[must_use]
pub fn detect_reduced_motion() -> bool {
    detect_reduced_motion_inner()
}

/// The OS accessibility preferences (cached): what
/// [`AccessibilityPreferences::from_system`](crate::AccessibilityPreferences::from_system)
/// reads, without running the platform reader again.
///
/// The cache holds the accessibility block of the last platform reader run:
/// [`SystemTheme::from_system`](crate::SystemTheme::from_system) and
/// `from_system_async` fill it, and where neither has run since the last
/// [`invalidate_caches()`], the first call runs the reader itself.
/// [`invalidate_caches()`] clears it. A reader that fails is not reported:
/// the preferences used without a reader (the defaults, with the platform's
/// [`detect_reduced_motion()`]) stay cached until the next invalidation, as a
/// failure to detect the icon theme does.
///
/// For a preset switch, where the OS preferences apply under the preset and
/// the reader has usually just run. The theme watcher
/// (`watch::on_theme_change`) reports a change of every accessibility setting
/// the reader reads but one, Windows' high contrast, which Windows announces
/// only to top-level windows, as it does the non-client fonts; nothing clears
/// the cache when it changes. Where the current setting matters more than the
/// cost, read [`AccessibilityPreferences::from_system`](crate::AccessibilityPreferences::from_system),
/// which runs the reader every time.
#[must_use]
pub fn accessibility_preferences() -> crate::AccessibilityPreferences {
    system().accessibility_preferences()
}

/// Whether GNOME asks for reduced motion: the portal's answer when it gave
/// one, else gsettings' `org.gnome.desktop.interface` `enable-animations`
/// read through `gsetting` (`false` is reduced motion); `None` when neither
/// answers. [`detect_reduced_motion`] and the GNOME reader both decide
/// through it.
#[cfg(target_os = "linux")]
pub(crate) fn gnome_reduce_motion(
    portal: Option<bool>,
    gsetting: impl FnOnce(&str, &str) -> Option<String>,
) -> Option<bool> {
    if portal.is_some() {
        return portal;
    }
    let enabled = gsetting("org.gnome.desktop.interface", "enable-animations")?;
    enabled.trim().parse::<bool>().ok().map(|enabled| !enabled)
}

/// Reduced motion as the reader of `desktop` (the refined desktop) reads it:
/// on KDE `kdeglobals` (`None` without the `kde` feature, which reads no KDE
/// file), as the KDE reader does; on GNOME and Budgie the `portal`, then
/// gsettings ([`gnome_reduce_motion`]), as the GNOME reader does; elsewhere,
/// and on KDE where `kdeglobals` gives `None`, gsettings alone. `false` when
/// nothing answers.
#[cfg(target_os = "linux")]
fn desktop_reduce_motion(
    desktop: LinuxDesktop,
    kdeglobals: impl FnOnce() -> Option<bool>,
    portal: impl FnOnce() -> Option<bool>,
    gsetting: impl FnOnce(&str, &str) -> Option<String>,
) -> bool {
    match desktop {
        LinuxDesktop::Kde => kdeglobals().or_else(|| gnome_reduce_motion(None, gsetting)),
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => gnome_reduce_motion(portal(), gsetting),
        _ => gnome_reduce_motion(None, gsetting),
    }
    .unwrap_or(false)
}

/// Inner detection logic for [`prefers_reduced_motion()`].
///
/// Separated from the public function to allow caching.
fn detect_reduced_motion_inner() -> bool {
    #[cfg(target_os = "linux")]
    {
        desktop_reduce_motion(
            system().refined_linux_desktop_sync(),
            || {
                #[cfg(feature = "kde")]
                {
                    Some(crate::kde::kdeglobals_reduce_motion())
                }
                #[cfg(not(feature = "kde"))]
                {
                    None
                }
            },
            || {
                #[cfg(feature = "portal")]
                {
                    pollster::block_on(crate::gnome::portal_reduce_motion())
                }
                #[cfg(not(feature = "portal"))]
                {
                    None
                }
            },
            |schema, key| gsettings_get(schema, key, SUBPROCESS_TIMEOUT),
        )
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        {
            crate::macos::reduce_motion()
        }
        #[cfg(not(feature = "macos"))]
        {
            false
        }
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        {
            crate::windows::read_reduce_motion().unwrap_or(false)
        }
        #[cfg(not(feature = "windows"))]
        {
            false
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        false
    }
}

// === DetectionContext ===

/// A value cached with the generation of its slot when its detection began.
struct Stamped<T> {
    generation: u64,
    value: T,
}

/// One cached detection: the value, and a generation every invalidation
/// bumps. A value is cached only if no invalidation landed while it was
/// detected, so a fill that began before an OS change and ends after a
/// watcher's invalidation does not cache what it read before the change.
struct CacheSlot<T> {
    value: ArcSwapOption<Stamped<T>>,
    generation: AtomicU64,
}

/// A fill in progress: the slot's generation and entry when it began.
pub(crate) struct CacheFill<T> {
    generation: u64,
    seen: Option<Arc<Stamped<T>>>,
}

impl<T> CacheFill<T> {
    /// The value the slot held for this fill's generation, if any.
    fn cached(&self) -> Option<&T> {
        self.seen
            .as_deref()
            .filter(|entry| entry.generation == self.generation)
            .map(|entry| &entry.value)
    }
}

impl<T> CacheSlot<T> {
    fn empty() -> Self {
        Self {
            value: ArcSwapOption::empty(),
            generation: AtomicU64::new(0),
        }
    }

    // The generation's Acquire loads pair with the Release bump in
    // `invalidate`, which follows its clearing store: a fill that reads the
    // bumped generation also sees the slot cleared, and an entry stamped with
    // an older generation than the one loaded is never returned.
    fn begin_fill(&self) -> CacheFill<T> {
        let generation = self.generation.load(Ordering::Acquire);
        CacheFill {
            generation,
            seen: self.value.load_full(),
        }
    }

    /// Cache `value` unless an invalidation landed after `fill` began, or
    /// another fill stored a value since.
    fn finish_fill(&self, fill: CacheFill<T>, value: T) {
        if self.generation.load(Ordering::Acquire) != fill.generation {
            return;
        }
        let stamped = Arc::new(Stamped {
            generation: fill.generation,
            value,
        });
        let _ = self.value.compare_and_swap(&fill.seen, Some(stamped));
    }

    /// The cached value, or `detect`'s, cached as [`finish_fill`](Self::finish_fill)
    /// caches it.
    fn get_or_fill(&self, detect: impl FnOnce() -> T) -> T
    where
        T: Clone,
    {
        let fill = self.begin_fill();
        if let Some(value) = fill.cached() {
            return value.clone();
        }
        let value = detect();
        self.finish_fill(fill, value.clone());
        value
    }

    /// The cached value, if any.
    #[cfg(any(test, target_os = "linux"))]
    fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.begin_fill().cached().cloned()
    }

    fn invalidate(&self) {
        self.value.store(None);
        self.generation.fetch_add(1, Ordering::Release);
    }
}

/// Process-wide detection cache.
///
/// Provides "cache on first read" semantics for OS detection queries
/// (`is_dark`, `reduced_motion`, `accessibility_preferences`, `icon_theme`,
/// `linux_desktop`, the font DPI) and
/// per-field invalidation for watchers that need fresh data.
///
/// Obtain the process-wide instance via [`system()`].
///
/// # Thread Safety
///
/// All reads and invalidations are lock-free (backed by
/// [`arc_swap::ArcSwapOption`]). A value detected while an invalidation of
/// its field lands is returned to its caller but not cached.
pub struct DetectionContext {
    is_dark: CacheSlot<bool>,
    reduced_motion: CacheSlot<bool>,
    reader_accessibility: CacheSlot<crate::AccessibilityPreferences>,
    icon_theme: CacheSlot<Result<String, IconThemeFailure>>,
    #[cfg(target_os = "linux")]
    linux_desktop: CacheSlot<LinuxDesktop>,
    #[cfg(target_os = "linux")]
    refined_linux_desktop: CacheSlot<RefinedDesktop>,
    font_dpi: CacheSlot<f32>,
}

impl DetectionContext {
    /// Create an empty context with no cached values.
    pub(crate) fn new() -> Self {
        Self {
            is_dark: CacheSlot::empty(),
            reduced_motion: CacheSlot::empty(),
            reader_accessibility: CacheSlot::empty(),
            icon_theme: CacheSlot::empty(),
            #[cfg(target_os = "linux")]
            linux_desktop: CacheSlot::empty(),
            #[cfg(target_os = "linux")]
            refined_linux_desktop: CacheSlot::empty(),
            font_dpi: CacheSlot::empty(),
        }
    }

    /// Whether the system is using a dark color scheme (cached).
    ///
    /// The first call queries the OS and caches the result.
    /// Subsequent calls return the cached value.
    /// Call [`invalidate_is_dark()`](Self::invalidate_is_dark) to
    /// force a re-read on the next call.
    #[must_use]
    pub fn is_dark(&self) -> bool {
        self.is_dark.get_or_fill(detect_is_dark_inner)
    }

    /// Whether the user prefers reduced motion (cached).
    ///
    /// The first call queries the OS and caches the result.
    /// Call [`invalidate_reduced_motion()`](Self::invalidate_reduced_motion)
    /// to force a re-read.
    #[must_use]
    pub fn prefers_reduced_motion(&self) -> bool {
        self.reduced_motion.get_or_fill(detect_reduced_motion_inner)
    }

    /// The OS accessibility preferences (cached), as
    /// [`accessibility_preferences()`](crate::detect::accessibility_preferences)
    /// describes: the platform reader's accessibility block, run on the
    /// first call unless a system read stored it.
    /// Call
    /// [`invalidate_accessibility_preferences()`](Self::invalidate_accessibility_preferences)
    /// to force a re-read.
    #[must_use]
    pub fn accessibility_preferences(&self) -> crate::AccessibilityPreferences {
        self.reader_accessibility_with(|| {
            pollster::block_on(crate::pipeline::reader_accessibility())
        })
    }

    /// The cached reader accessibility block, with `read` run where
    /// nothing is cached.
    pub(crate) fn reader_accessibility_with(
        &self,
        read: impl FnOnce() -> crate::AccessibilityPreferences,
    ) -> crate::AccessibilityPreferences {
        self.reader_accessibility.get_or_fill(read)
    }

    /// Begin a platform read whose accessibility block
    /// [`store_reader_accessibility`](Self::store_reader_accessibility)
    /// stores.
    pub(crate) fn begin_reader_accessibility(&self) -> CacheFill<crate::AccessibilityPreferences> {
        self.reader_accessibility.begin_fill()
    }

    /// Store the accessibility block a platform reader just returned, unless
    /// the cache was invalidated or filled since the read began (`fill`).
    pub(crate) fn store_reader_accessibility(
        &self,
        fill: CacheFill<crate::AccessibilityPreferences>,
        prefs: crate::AccessibilityPreferences,
    ) {
        self.reader_accessibility.finish_fill(fill, prefs);
    }

    /// The current icon theme name (cached), or why none was detected.
    ///
    /// The first call detects the theme from the OS and caches the
    /// outcome, a failure included, so a failed detection is not
    /// repeated on every call; subsequent calls return the cached
    /// outcome. Call
    /// [`invalidate_icon_theme()`](Self::invalidate_icon_theme) to
    /// force a re-read.
    ///
    /// # Errors
    ///
    /// As [`system_icon_theme()`]: no theme stands in for one that could
    /// not be detected.
    pub fn icon_theme(&self) -> crate::Result<String> {
        self.icon_theme_with(detect_icon_theme_inner)
    }

    /// [`icon_theme()`](Self::icon_theme), with `detect` run where
    /// nothing is cached.
    fn icon_theme_with(
        &self,
        detect: impl FnOnce() -> Result<String, IconThemeFailure>,
    ) -> crate::Result<String> {
        self.icon_theme
            .get_or_fill(detect)
            .map_err(|failure| failure.to_error())
    }

    /// The current Linux desktop environment (cached).
    ///
    /// The first call reads `XDG_CURRENT_DESKTOP` and caches the
    /// result. Call
    /// [`invalidate_linux_desktop()`](Self::invalidate_linux_desktop)
    /// to force a re-read.
    #[cfg(target_os = "linux")]
    #[must_use]
    pub fn linux_desktop(&self) -> LinuxDesktop {
        self.linux_desktop.get_or_fill(detect_linux_desktop)
    }

    /// The desktop the reader, the theme watcher, the icon-theme detection
    /// and the font DPI decide by ([`refine_linux_desktop`]), cached: on an
    /// unrecognised desktop the portal backend is asked over D-Bus (with the
    /// `portal` feature), then `kdeglobals` is looked for. The system reads
    /// (`SystemTheme::from_system`, `from_system_async`,
    /// `AccessibilityPreferences::from_system`) fill the cache.
    /// [`invalidate_linux_desktop()`](Self::invalidate_linux_desktop) clears it.
    #[cfg(target_os = "linux")]
    pub(crate) async fn refined_linux_desktop(&self) -> RefinedDesktop {
        let fill = self.refined_linux_desktop.begin_fill();
        if let Some(v) = fill.cached() {
            return *v;
        }
        let raw = self.linux_desktop();
        #[cfg(feature = "portal")]
        let portal_backend = if raw == LinuxDesktop::Unknown {
            crate::gnome::detect_portal_backend().await
        } else {
            None
        };
        #[cfg(not(feature = "portal"))]
        let portal_backend = None;
        let refined = refine_desktop(raw, portal_backend, kdeglobals_exists);
        let without_portal = refine_linux_desktop(raw, None, kdeglobals_exists);
        self.finish_refined_desktop(fill, refined, without_portal);
        refined
    }

    /// Cache `refined`, the desktop a system read refined. Where it is not
    /// `without_portal`, the desktop a synchronous caller refines while
    /// nothing is cached ([`refined_linux_desktop_sync`](Self::refined_linux_desktop_sync)),
    /// the values detected from that desktop are cleared: the font DPI, the
    /// icon theme, dark mode and reduced motion.
    #[cfg(target_os = "linux")]
    fn finish_refined_desktop(
        &self,
        fill: CacheFill<RefinedDesktop>,
        refined: RefinedDesktop,
        without_portal: LinuxDesktop,
    ) {
        self.refined_linux_desktop.finish_fill(fill, refined);
        if refined.desktop != without_portal {
            self.invalidate_font_dpi();
            self.invalidate_icon_theme();
            self.invalidate_is_dark();
            self.invalidate_reduced_motion();
        }
    }

    /// The desktop the platform readers use: the raw `XDG_CURRENT_DESKTOP`
    /// ([`linux_desktop()`](Self::linux_desktop)) where it names a desktop;
    /// for [`LinuxDesktop::Unknown`], the desktop the portal backend named at
    /// the last system read (`SystemTheme::from_system`, `from_system_async`,
    /// `AccessibilityPreferences::from_system`, with the `portal` feature),
    /// else KDE where the user's `kdeglobals` exists (with the `kde`
    /// feature), else `Unknown`. The reader, the theme watcher, the icon-theme
    /// detection and the font DPI decide by it, and the icon enums
    /// native-theme-build generates pick their per-desktop names by it.
    ///
    /// A system read caches the desktop it refined; until one has, this call
    /// does not ask the portal and caches nothing, so on an unrecognised
    /// desktop whose portal backend names one it can differ from what the
    /// next system read caches (a KDE backend without `kdeglobals` is
    /// `Unknown` here, a GNOME backend with `kdeglobals` is KDE here); the
    /// system read then clears the font DPI, icon theme, dark mode and
    /// reduced motion detected from this desktop.
    /// [`invalidate_linux_desktop()`](Self::invalidate_linux_desktop) clears
    /// the cached desktop.
    #[cfg(target_os = "linux")]
    #[must_use]
    pub fn refined_linux_desktop_sync(&self) -> LinuxDesktop {
        if let Some(v) = self.refined_linux_desktop.get() {
            return v.desktop;
        }
        refine_linux_desktop(self.linux_desktop(), None, kdeglobals_exists)
    }

    /// The system font DPI (cached), from the detection chain of
    /// `detect_system_font_dpi`.
    ///
    /// The first call runs the chain (on Linux `xrdb -query`, maybe
    /// `xrandr`) and caches the result. Call
    /// [`invalidate_font_dpi()`](Self::invalidate_font_dpi) to force a re-read.
    pub(crate) fn font_dpi(&self) -> f32 {
        self.font_dpi_with(detect_system_font_dpi)
    }

    /// [`font_dpi()`](Self::font_dpi), with `detect` run where nothing is
    /// cached.
    fn font_dpi_with(&self, detect: impl FnOnce() -> f32) -> f32 {
        self.font_dpi.get_or_fill(detect)
    }

    /// Clear the cached dark-mode value.
    pub fn invalidate_is_dark(&self) {
        self.is_dark.invalidate();
    }

    /// Clear the cached reduced-motion value.
    pub fn invalidate_reduced_motion(&self) {
        self.reduced_motion.invalidate();
    }

    /// Clear the cached accessibility preferences.
    pub fn invalidate_accessibility_preferences(&self) {
        self.reader_accessibility.invalidate();
    }

    /// Clear the cached icon theme, or the cached failure to detect one.
    pub fn invalidate_icon_theme(&self) {
        self.icon_theme.invalidate();
    }

    /// Clear the cached font DPI.
    pub fn invalidate_font_dpi(&self) {
        self.font_dpi.invalidate();
    }

    /// Clear the cached Linux desktop environment, and the desktop refined
    /// from it.
    #[cfg(target_os = "linux")]
    pub fn invalidate_linux_desktop(&self) {
        self.linux_desktop.invalidate();
        self.refined_linux_desktop.invalidate();
    }

    /// Clear all cached values.
    pub fn invalidate_all(&self) {
        self.invalidate_is_dark();
        self.invalidate_reduced_motion();
        self.invalidate_accessibility_preferences();
        self.invalidate_icon_theme();
        #[cfg(target_os = "linux")]
        self.invalidate_linux_desktop();
        self.invalidate_font_dpi();
    }
}

/// Return the process-wide default [`DetectionContext`].
///
/// This is a lazily-initialized singleton. The first call creates it;
/// all subsequent calls return the same instance.
pub fn system() -> &'static DetectionContext {
    static INSTANCE: std::sync::OnceLock<DetectionContext> = std::sync::OnceLock::new();
    INSTANCE.get_or_init(DetectionContext::new)
}

/// Inner icon theme detection, delegating to platform-specific logic.
///
/// On Linux, dispatches by desktop environment. On macOS/iOS/Windows,
/// returns the compile-time constant; elsewhere, the platform's failure.
fn detect_icon_theme_inner() -> Result<String, IconThemeFailure> {
    icon_theme::detect_icon_theme_outcome()
}

// === Crate-internal accessors ===

/// The system font DPI, cached ([`DetectionContext::font_dpi`]).
pub(crate) fn system_font_dpi() -> f32 {
    system().font_dpi()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod reduced_motion_tests {
    use super::*;

    #[test]
    fn prefers_reduced_motion_smoke_test() {
        // Smoke test: function should not panic on any platform.
        // Cannot assert a specific value because caching preserves the first call
        // and CI environments have varying accessibility settings.
        let _result = prefers_reduced_motion();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn detect_reduced_motion_inner_linux() {
        // Bypass caching to test actual detection logic.
        // On CI without gsettings, returns false (animations enabled).
        // On developer machines, depends on accessibility settings.
        let result = detect_reduced_motion_inner();
        // Just verify it returns a bool without panicking.
        let _ = result;
    }

    /// `gsetting` for [`desktop_reduce_motion`]: `enable-animations` is
    /// `enabled`; `asked` counts the reads.
    #[cfg(target_os = "linux")]
    fn animations<'a>(
        enabled: &'static str,
        asked: &'a std::cell::Cell<u32>,
    ) -> impl FnOnce(&str, &str) -> Option<String> + 'a {
        move |schema, key| {
            asked.set(asked.get() + 1);
            ((schema, key) == ("org.gnome.desktop.interface", "enable-animations"))
                .then(|| enabled.to_string())
        }
    }

    /// On KDE reduced motion is kdeglobals' `AnimationDurationFactor`, as the
    /// KDE reader reads it, not GNOME's `enable-animations`, which may be
    /// absent or stale there (M1-T6-04); gsettings answers only where
    /// kdeglobals is not read (no `kde` feature).
    #[cfg(target_os = "linux")]
    #[test]
    fn kde_reduced_motion_is_kdeglobals_not_gsettings() {
        let asked = std::cell::Cell::new(0);
        let portal = || -> Option<bool> { panic!("KDE does not ask the portal") };
        assert!(desktop_reduce_motion(
            LinuxDesktop::Kde,
            || Some(true),
            portal,
            animations("true", &asked)
        ));
        assert!(!desktop_reduce_motion(
            LinuxDesktop::Kde,
            || Some(false),
            portal,
            animations("false", &asked)
        ));
        assert_eq!(asked.get(), 0);
        assert!(desktop_reduce_motion(
            LinuxDesktop::Kde,
            || None,
            portal,
            animations("false", &asked)
        ));
        assert_eq!(asked.get(), 1);
    }

    /// On GNOME and Budgie the portal's answer decides, as in the GNOME
    /// reader; without one, gsettings' `enable-animations`.
    #[cfg(target_os = "linux")]
    #[test]
    fn gnome_reduced_motion_is_the_portal_then_gsettings() {
        let asked = std::cell::Cell::new(0);
        let kdeglobals = || -> Option<bool> { panic!("GNOME does not read kdeglobals") };
        for desktop in [LinuxDesktop::Gnome, LinuxDesktop::Budgie] {
            assert!(desktop_reduce_motion(
                desktop,
                kdeglobals,
                || Some(true),
                animations("true", &asked)
            ));
            assert!(!desktop_reduce_motion(
                desktop,
                kdeglobals,
                || Some(false),
                animations("false", &asked)
            ));
            assert_eq!(asked.get(), 0);
            assert!(desktop_reduce_motion(
                desktop,
                kdeglobals,
                || None,
                animations("false", &asked)
            ));
            assert_eq!(asked.get(), 1);
            asked.set(0);
        }
    }

    /// A desktop without a reader answers from gsettings alone.
    #[cfg(target_os = "linux")]
    #[test]
    fn other_desktops_reduced_motion_is_gsettings() {
        let asked = std::cell::Cell::new(0);
        assert!(desktop_reduce_motion(
            LinuxDesktop::Xfce,
            || -> Option<bool> { panic!("XFCE does not read kdeglobals") },
            || -> Option<bool> { panic!("XFCE does not ask the portal") },
            animations("false", &asked)
        ));
        assert!(!desktop_reduce_motion(
            LinuxDesktop::Unknown,
            || None,
            || None,
            |_, _| None
        ));
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

#[cfg(all(test, target_os = "linux", feature = "kde"))]
mod force_font_dpi_tests {
    use super::*;

    #[test]
    fn force_font_dpi_is_a_positive_u32() {
        assert_eq!(parse_force_font_dpi("120"), Some(120.0));
        assert_eq!(parse_force_font_dpi(" 144 "), Some(144.0));
        for value in ["0", "inf", "-inf", "NaN", "-96", "96.5", "1e3", ""] {
            assert_eq!(parse_force_font_dpi(value), None, "forceFontDPI={value}");
        }
    }

    #[test]
    fn only_an_x11_session_reads_force_font_dpi() {
        assert!(reads_force_font_dpi(Some("x11")));
        for session in [Some("wayland"), Some("tty"), Some(""), None] {
            assert!(!reads_force_font_dpi(session), "{session:?}");
        }
    }

    #[test]
    fn a_wayland_session_ignores_force_font_dpi() {
        let reads = std::cell::Cell::new(0);
        let file = || {
            reads.set(reads.get() + 1);
            Some("120".to_string())
        };
        for session in [Some("wayland"), None] {
            assert_eq!(kde_force_font_dpi(session, file), None, "{session:?}");
        }
        assert_eq!(reads.get(), 0, "a file was read off X11");
    }

    #[test]
    fn an_x11_session_takes_a_valid_kcmfonts_key() {
        assert_eq!(
            kde_force_font_dpi(Some("x11"), || Some("144".into())),
            Some(144.0)
        );
        for kcmfonts in [None, Some("0"), Some("inf")] {
            assert_eq!(
                kde_force_font_dpi(Some("x11"), || kcmfonts.map(String::from)),
                None,
                "kcmfonts {kcmfonts:?}"
            );
        }
    }
}

#[cfg(test)]
mod cache_slot_tests {
    use super::*;

    /// The slot every cached field uses ([`DetectionContext::is_dark`],
    /// [`DetectionContext::linux_desktop`] and the rest): a value detected
    /// across an invalidation is returned, not cached.
    #[test]
    fn a_value_detected_across_an_invalidation_is_not_cached() {
        let slot = CacheSlot::empty();
        assert!(slot.get_or_fill(|| {
            slot.invalidate();
            true
        }));
        assert!(slot.get().is_none());
        assert!(!slot.get_or_fill(|| false));
        assert_eq!(slot.get(), Some(false));
    }

    /// A fill whose generation check passed before an invalidation and whose
    /// swap landed after it leaves an entry of the old generation, which no
    /// read returns.
    #[test]
    fn an_entry_stamped_before_an_invalidation_is_not_returned() {
        let slot = CacheSlot::empty();
        let fill = slot.begin_fill();
        slot.invalidate();
        let stale = Arc::new(Stamped {
            generation: fill.generation,
            value: true,
        });
        let _ = slot.value.compare_and_swap(&fill.seen, Some(stale));
        assert!(slot.get().is_none());
        assert!(!slot.get_or_fill(|| false));
        assert_eq!(slot.get(), Some(false));
    }

    #[test]
    fn a_fill_does_not_replace_a_value_stored_since_it_began() {
        let slot = CacheSlot::empty();
        let slow = slot.begin_fill();
        let _ = slot.get_or_fill(|| 2);
        slot.finish_fill(slow, 1);
        assert_eq!(slot.get(), Some(2));
    }
}

#[cfg(test)]
mod detection_context_tests {
    use super::*;

    #[test]
    fn system_returns_same_instance() {
        let a = system() as *const DetectionContext;
        let b = system() as *const DetectionContext;
        assert_eq!(a, b);
    }

    /// A context with every slot filled.
    fn filled() -> DetectionContext {
        let ctx = DetectionContext::new();
        ctx.is_dark.get_or_fill(|| true);
        ctx.reduced_motion.get_or_fill(|| true);
        ctx.reader_accessibility
            .get_or_fill(crate::AccessibilityPreferences::default);
        let _ = ctx.icon_theme.get_or_fill(|| Ok("breeze".to_string()));
        #[cfg(target_os = "linux")]
        ctx.linux_desktop.get_or_fill(|| LinuxDesktop::Kde);
        #[cfg(target_os = "linux")]
        ctx.refined_linux_desktop.get_or_fill(|| RefinedDesktop {
            desktop: LinuxDesktop::Kde,
            by_kdeglobals: false,
        });
        ctx.font_dpi.get_or_fill(|| 120.0);
        ctx
    }

    /// `invalidate_all` empties every slot. The pattern names each field, so
    /// a slot added to [`DetectionContext`] does not compile until it is
    /// checked here.
    #[test]
    fn invalidate_all_empties_every_slot() {
        let ctx = filled();
        ctx.invalidate_all();
        let DetectionContext {
            is_dark,
            reduced_motion,
            reader_accessibility,
            icon_theme,
            #[cfg(target_os = "linux")]
            linux_desktop,
            #[cfg(target_os = "linux")]
            refined_linux_desktop,
            font_dpi,
        } = &ctx;
        assert!(is_dark.get().is_none(), "is_dark");
        assert!(reduced_motion.get().is_none(), "reduced_motion");
        assert!(reader_accessibility.get().is_none(), "reader_accessibility");
        assert!(icon_theme.get().is_none(), "icon_theme");
        #[cfg(target_os = "linux")]
        assert!(linux_desktop.get().is_none(), "linux_desktop");
        #[cfg(target_os = "linux")]
        assert!(
            refined_linux_desktop.get().is_none(),
            "refined_linux_desktop"
        );
        assert!(font_dpi.get().is_none(), "font_dpi");
    }

    #[test]
    fn invalidate_is_dark_empties_only_is_dark() {
        let ctx = filled();
        ctx.invalidate_is_dark();
        assert_eq!(ctx.is_dark.get(), None);
        assert_eq!(ctx.font_dpi.get(), Some(120.0));
    }

    #[test]
    fn font_dpi_is_cached_until_invalidate_all() {
        let ctx = DetectionContext::new();
        let runs = std::cell::Cell::new(0);
        let detect = |dpi| {
            runs.set(runs.get() + 1);
            dpi
        };
        assert_eq!(ctx.font_dpi_with(|| detect(120.0)), 120.0);
        assert_eq!(ctx.font_dpi_with(|| detect(96.0)), 120.0);
        assert_eq!(runs.get(), 1, "a cached font DPI was detected again");
        ctx.invalidate_all();
        assert_eq!(ctx.font_dpi_with(|| detect(96.0)), 96.0);
        assert_eq!(runs.get(), 2);
    }

    fn undetected() -> Result<String, IconThemeFailure> {
        Err(IconThemeFailure::for_tests("the test's failing detection"))
    }

    #[test]
    fn a_failed_icon_theme_detection_is_cached_and_reported() {
        let ctx = DetectionContext::new();
        let runs = std::cell::Cell::new(0);
        let detect = || {
            runs.set(runs.get() + 1);
            undetected()
        };
        let first = ctx.icon_theme_with(detect);
        let second = ctx.icon_theme_with(detect);
        assert_eq!(runs.get(), 1, "a failed detection ran again");
        for outcome in [first, second] {
            match outcome {
                Ok(name) => panic!("a failed detection gave the theme {name}"),
                Err(e) => assert!(
                    e.to_string().contains("the test's failing detection"),
                    "got: {e}"
                ),
            }
        }
    }

    #[test]
    fn invalidate_icon_theme_clears_a_cached_failure() {
        let ctx = DetectionContext::new();
        assert!(ctx.icon_theme_with(undetected).is_err());
        ctx.invalidate_icon_theme();
        let theme = ctx.icon_theme_with(|| Ok("breeze".to_string()));
        assert_eq!(theme.ok().as_deref(), Some("breeze"));
    }

    #[test]
    fn invalidate_all_clears_a_cached_failure() {
        let ctx = DetectionContext::new();
        assert!(ctx.icon_theme_with(undetected).is_err());
        ctx.invalidate_all();
        let theme = ctx.icon_theme_with(|| Ok("breeze".to_string()));
        assert_eq!(theme.ok().as_deref(), Some("breeze"));
    }

    fn scaled_by(text_scaling_factor: f32) -> crate::AccessibilityPreferences {
        crate::AccessibilityPreferences {
            text_scaling_factor,
            ..crate::AccessibilityPreferences::default()
        }
    }

    #[test]
    fn a_second_accessibility_read_does_not_run_the_reader() {
        let ctx = DetectionContext::new();
        let runs = std::cell::Cell::new(0);
        let read = || {
            runs.set(runs.get() + 1);
            scaled_by(1.5)
        };
        assert_eq!(ctx.reader_accessibility_with(read), scaled_by(1.5));
        assert_eq!(ctx.reader_accessibility_with(read), scaled_by(1.5));
        assert_eq!(runs.get(), 1, "the second call ran the reader");
    }

    #[test]
    fn a_stored_accessibility_block_is_returned_until_invalidated() {
        let ctx = DetectionContext::new();
        let runs = std::cell::Cell::new(0);
        let read = || {
            runs.set(runs.get() + 1);
            scaled_by(1.25)
        };
        ctx.store_reader_accessibility(ctx.begin_reader_accessibility(), scaled_by(2.0));
        assert_eq!(ctx.reader_accessibility_with(read), scaled_by(2.0));
        assert_eq!(runs.get(), 0, "a stored block ran the reader");
        ctx.invalidate_all();
        assert_eq!(ctx.reader_accessibility_with(read), scaled_by(1.25));
        assert_eq!(runs.get(), 1);
        ctx.invalidate_accessibility_preferences();
        assert_eq!(ctx.reader_accessibility_with(read), scaled_by(1.25));
        assert_eq!(runs.get(), 2);
    }

    /// A font DPI detected before an invalidation that lands mid-detection
    /// is not cached: the next read detects again.
    #[test]
    fn a_font_dpi_detected_across_an_invalidation_is_not_cached() {
        let ctx = DetectionContext::new();
        let stale = ctx.font_dpi_with(|| {
            ctx.invalidate_font_dpi();
            120.0
        });
        assert_eq!(stale, 120.0);
        assert_eq!(ctx.font_dpi_with(|| 96.0), 96.0);
    }

    #[test]
    fn an_icon_theme_detected_across_an_invalidation_is_not_cached() {
        let ctx = DetectionContext::new();
        let stale = ctx.icon_theme_with(|| {
            ctx.invalidate_icon_theme();
            Ok("breeze".to_string())
        });
        assert_eq!(stale.ok().as_deref(), Some("breeze"));
        let theme = ctx.icon_theme_with(|| Ok("Adwaita".to_string()));
        assert_eq!(theme.ok().as_deref(), Some("Adwaita"));
    }

    #[test]
    fn an_accessibility_block_read_across_an_invalidation_is_not_cached() {
        let ctx = DetectionContext::new();
        let stale = ctx.reader_accessibility_with(|| {
            ctx.invalidate_all();
            scaled_by(1.5)
        });
        assert_eq!(stale, scaled_by(1.5));
        assert_eq!(
            ctx.reader_accessibility_with(|| scaled_by(2.0)),
            scaled_by(2.0)
        );
    }

    /// A synchronous caller gets the desktop a system read refined, portal
    /// answer included, until it is invalidated.
    #[cfg(target_os = "linux")]
    #[test]
    fn the_sync_refined_desktop_is_the_cached_one() {
        let ctx = DetectionContext::new();
        let fill = ctx.refined_linux_desktop.begin_fill();
        let gnome = refine_desktop(LinuxDesktop::Gnome, None, || false);
        ctx.refined_linux_desktop.finish_fill(fill, gnome);
        assert_eq!(ctx.refined_linux_desktop_sync(), LinuxDesktop::Gnome);
        assert_eq!(pollster::block_on(ctx.refined_linux_desktop()), gnome);
        ctx.invalidate_linux_desktop();
        assert!(ctx.refined_linux_desktop.get().is_none());
    }

    /// A context whose raw desktop is `Unknown` and whose system read refined
    /// it with `portal_backend`, `kdeglobals` absent.
    #[cfg(target_os = "linux")]
    fn refined_unknown(portal_backend: Option<LinuxDesktop>) -> DetectionContext {
        let ctx = DetectionContext::new();
        ctx.linux_desktop
            .finish_fill(ctx.linux_desktop.begin_fill(), LinuxDesktop::Unknown);
        ctx.finish_refined_desktop(
            ctx.refined_linux_desktop.begin_fill(),
            refine_desktop(LinuxDesktop::Unknown, portal_backend, || false),
            LinuxDesktop::Unknown,
        );
        ctx
    }

    /// A font DPI, icon theme, dark mode or reduced motion read before the
    /// first system read came from the desktop refined without the portal; a
    /// portal answer that names another desktop clears them, so they are read
    /// again from the reader's desktop (docs/todo.md:2628, M4-49).
    #[cfg(target_os = "linux")]
    #[test]
    fn a_portal_answer_that_changes_the_desktop_clears_what_was_read_from_it() {
        let ctx = DetectionContext::new();
        assert_eq!(ctx.font_dpi_with(|| 120.0), 120.0);
        assert_eq!(
            ctx.icon_theme_with(|| Ok("breeze".to_string()))
                .ok()
                .as_deref(),
            Some("breeze")
        );
        assert!(ctx.is_dark.get_or_fill(|| true));
        assert!(ctx.reduced_motion.get_or_fill(|| true));

        let refined = refine_desktop(LinuxDesktop::Unknown, Some(LinuxDesktop::Gnome), || true);
        ctx.finish_refined_desktop(
            ctx.refined_linux_desktop.begin_fill(),
            refined,
            LinuxDesktop::Kde,
        );
        assert_eq!(ctx.refined_linux_desktop_sync(), LinuxDesktop::Gnome);
        assert_eq!(ctx.font_dpi_with(|| 96.0), 96.0);
        assert_eq!(
            ctx.icon_theme_with(|| Ok("Adwaita".to_string()))
                .ok()
                .as_deref(),
            Some("Adwaita")
        );
        assert_eq!(ctx.is_dark.get(), None);
        assert_eq!(ctx.reduced_motion.get(), None);
    }

    /// A portal answer that names the desktop the synchronous refinement
    /// already gave keeps what was read from it.
    #[cfg(target_os = "linux")]
    #[test]
    fn a_portal_answer_that_keeps_the_desktop_keeps_what_was_read() {
        let ctx = DetectionContext::new();
        assert_eq!(ctx.font_dpi_with(|| 120.0), 120.0);
        let refined = refine_desktop(LinuxDesktop::Unknown, Some(LinuxDesktop::Kde), || true);
        ctx.finish_refined_desktop(
            ctx.refined_linux_desktop.begin_fill(),
            refined,
            LinuxDesktop::Kde,
        );
        assert_eq!(ctx.font_dpi_with(|| 96.0), 120.0);
    }

    /// On an unrecognised desktop a system read refined to KDE, dark mode
    /// comes from `kdeglobals` first, as the KDE reader takes it; on one
    /// refined to GNOME, from gsettings first.
    #[cfg(target_os = "linux")]
    #[test]
    fn dark_mode_follows_the_refined_desktop() {
        let kde = refined_unknown(Some(LinuxDesktop::Kde));
        assert_eq!(
            desktop_is_dark(&kde, || Some(true), || Some(false)),
            Some(true)
        );
        let gnome = refined_unknown(Some(LinuxDesktop::Gnome));
        assert_eq!(
            desktop_is_dark(&gnome, || Some(true), || Some(false)),
            Some(false)
        );
        assert_eq!(desktop_is_dark(&gnome, || Some(true), || None), Some(true));
    }
}

#[cfg(all(test, target_os = "linux"))]
mod refine_linux_desktop_tests {
    use super::*;

    fn asked() -> bool {
        panic!("kdeglobals asked where the desktop is already decided")
    }

    /// `XDG_CURRENT_DESKTOP` decides where it names a desktop; the portal
    /// backend, then `kdeglobals`, only where it names none.
    #[test]
    fn the_raw_desktop_then_the_portal_backend_then_kdeglobals() {
        let refine = |raw, portal_backend, kdeglobals_exists: fn() -> bool| {
            refine_linux_desktop(raw, portal_backend, kdeglobals_exists)
        };
        assert_eq!(
            refine(LinuxDesktop::Gnome, Some(LinuxDesktop::Kde), asked),
            LinuxDesktop::Gnome
        );
        assert_eq!(refine(LinuxDesktop::Xfce, None, asked), LinuxDesktop::Xfce);
        assert_eq!(
            refine(LinuxDesktop::Unknown, Some(LinuxDesktop::Kde), asked),
            LinuxDesktop::Kde
        );
        assert_eq!(
            refine(LinuxDesktop::Unknown, Some(LinuxDesktop::Gnome), asked),
            LinuxDesktop::Gnome
        );
        assert_eq!(
            refine(LinuxDesktop::Unknown, None, || true),
            LinuxDesktop::Kde
        );
        assert_eq!(
            refine(LinuxDesktop::Unknown, None, || false),
            LinuxDesktop::Unknown
        );
    }
}

#[cfg(all(test, target_os = "linux"))]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod subprocess_tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{Duration, Instant};

    /// Whether `program` is an executable file in a `$PATH` directory.
    fn on_path(program: &str) -> bool {
        std::env::var_os("PATH")
            .is_some_and(|path| std::env::split_paths(&path).any(|dir| dir.join(program).is_file()))
    }

    #[test]
    fn poll_intervals_double_from_1_ms_up_to_50_ms() {
        let intervals: Vec<Duration> = poll_intervals().take(10).collect();
        let expected: Vec<Duration> = [1, 2, 4, 8, 16, 32, 50, 50, 50, 50]
            .into_iter()
            .map(Duration::from_millis)
            .collect();
        assert_eq!(intervals, expected);
    }

    #[test]
    fn run_with_timeout_returns_stdout_of_a_successful_command() {
        if !on_path("echo") {
            eprintln!("skipped: echo is not installed");
            return;
        }
        let out = stdout_with_timeout("echo", &["hi"], SUBPROCESS_TIMEOUT);
        assert_eq!(out.as_deref().map(str::trim), Some("hi"));
    }

    /// An unsuccessful exit is an output, with its status and its stderr;
    /// [`stdout_with_timeout`] gives `None` for it.
    #[test]
    fn run_with_timeout_returns_the_status_and_stderr_of_a_failing_command() {
        if !on_path("sh") {
            eprintln!("skipped: sh is not installed");
            return;
        }
        let output =
            run_with_timeout("sh", &["-c", "echo oops >&2; exit 3"], SUBPROCESS_TIMEOUT).unwrap();
        assert_eq!(output.status.code(), Some(3));
        assert_eq!(output.stderr, b"oops\n");
        assert_eq!(
            stdout_with_timeout("sh", &["-c", "exit 3"], SUBPROCESS_TIMEOUT),
            None
        );
    }

    #[test]
    fn run_with_timeout_reports_a_missing_program_as_not_found() {
        let error =
            run_with_timeout("nt-test-no-such-program", &[], SUBPROCESS_TIMEOUT).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    }

    /// A command that exits at once with no output gives an empty stdout;
    /// the time it took is printed (the schedule test is the timing proof).
    #[test]
    fn run_with_timeout_returns_the_empty_stdout_of_a_quiet_command() {
        if !on_path("true") {
            eprintln!("skipped: true is not installed");
            return;
        }
        let start = Instant::now();
        assert_eq!(
            stdout_with_timeout("true", &[], SUBPROCESS_TIMEOUT).as_deref(),
            Some("")
        );
        eprintln!("`true` ran in {:?}", start.elapsed());
    }

    /// Whether a process runs with exactly the command line `cmdline`.
    fn running(cmdline: &str) -> Vec<PathBuf> {
        std::fs::read_dir("/proc")
            .unwrap()
            .flatten()
            .map(|entry| entry.path().join("cmdline"))
            .filter(|path| std::fs::read(path).is_ok_and(|bytes| bytes == cmdline.as_bytes()))
            .collect()
    }

    /// A command that outlives the timeout is killed and reaped, and the
    /// call ends about the timeout after it began.
    #[test]
    fn run_with_timeout_kills_a_command_that_outlives_the_timeout() {
        if !on_path("sleep") {
            eprintln!("skipped: sleep is not installed");
            return;
        }
        let timeout = Duration::from_millis(100);
        let seconds = format!("5.{}", std::process::id());
        let start = Instant::now();
        let error = run_with_timeout("sleep", &[&seconds], timeout).unwrap_err();
        let elapsed = start.elapsed();
        assert_eq!(error.kind(), std::io::ErrorKind::TimedOut);
        assert_eq!(error.to_string(), "timed out after 100ms");
        assert!(
            elapsed >= timeout && elapsed < Duration::from_secs(1),
            "took {elapsed:?}"
        );
        let still_running = running(&format!("sleep\0{seconds}\0"));
        assert!(still_running.is_empty(), "not killed: {still_running:?}");
    }

    /// 100 KiB is more than a Linux pipe holds (65,536 bytes, pipe(7)), so
    /// the command finishes only if its stdout is read while it runs.
    #[test]
    fn run_with_timeout_returns_all_of_an_output_larger_than_a_pipe() {
        if !on_path("sh") || !on_path("head") {
            eprintln!("skipped: sh or head is not installed");
            return;
        }
        let out = stdout_with_timeout(
            "sh",
            &["-c", "head -c 102400 /dev/zero"],
            SUBPROCESS_TIMEOUT,
        );
        assert_eq!(out.map(|out| out.len()), Some(102_400));
    }

    /// A process the command leaves running with its stdout keeps the pipe
    /// open after the command exits; the call still ends at the deadline.
    #[test]
    fn run_with_timeout_ends_at_the_deadline_when_a_grandchild_holds_stdout() {
        if !on_path("sh") || !on_path("sleep") {
            eprintln!("skipped: sh or sleep is not installed");
            return;
        }
        let script = format!("sleep 3.{} & echo hi", std::process::id());
        let start = Instant::now();
        let error =
            run_with_timeout("sh", &["-c", &script], Duration::from_millis(100)).unwrap_err();
        let elapsed = start.elapsed();
        assert_eq!(error.kind(), std::io::ErrorKind::TimedOut);
        assert!(elapsed < Duration::from_secs(1), "took {elapsed:?}");
    }
}

#[cfg(all(test, target_os = "linux"))]
mod config_home_tests {
    use super::*;
    use std::ffi::OsStr;
    use std::path::PathBuf;

    fn config_home(var: Option<&str>, home: Option<&str>) -> Option<PathBuf> {
        xdg_config_home_from(var.map(OsStr::new), home.map(OsStr::new))
    }

    #[test]
    fn an_absolute_xdg_config_home_is_the_config_directory() {
        assert_eq!(
            config_home(Some("/tmp/cfg"), Some("/home/u")),
            Some(PathBuf::from("/tmp/cfg"))
        );
        assert_eq!(
            config_home(Some("/tmp/cfg"), None),
            Some(PathBuf::from("/tmp/cfg"))
        );
    }

    /// Unset, empty and relative are each "not set" (the XDG Base Directory
    /// Specification ignores a relative path).
    #[test]
    fn an_unset_empty_or_relative_xdg_config_home_falls_back_to_home() {
        for var in [None, Some(""), Some("relative/cfg"), Some(".config")] {
            assert_eq!(
                config_home(var, Some("/home/u")),
                Some(PathBuf::from("/home/u/.config")),
                "{var:?}"
            );
        }
    }

    #[test]
    fn without_xdg_config_home_and_home_there_is_no_config_directory() {
        for home in [None, Some("")] {
            for var in [None, Some(""), Some("relative")] {
                assert_eq!(config_home(var, home), None, "{var:?} {home:?}");
            }
        }
    }
}

#[cfg(all(test, target_os = "linux", feature = "portal"))]
mod gnome_font_dpi_tests {
    use super::*;

    /// GNOME and Budgie, the desktops the GNOME reader reads, lay fonts out at
    /// GNOME's 96 whatever `Xft.dpi` says; other desktops keep the chain.
    #[test]
    fn gnome_desktops_fix_the_font_dpi() {
        assert_eq!(desktop_font_dpi(LinuxDesktop::Gnome), Some(96.0));
        assert_eq!(desktop_font_dpi(LinuxDesktop::Budgie), Some(96.0));
        assert_eq!(desktop_font_dpi(LinuxDesktop::Kde), None);
        assert_eq!(desktop_font_dpi(LinuxDesktop::Unknown), None);
    }

    /// An unrecognised desktop with a GNOME portal backend gets the GNOME
    /// reader, so the preset path takes the reader's 96, `kdeglobals` or not.
    #[test]
    fn an_unknown_desktop_with_a_gnome_portal_has_gnomes_font_dpi() {
        for kdeglobals in [false, true] {
            let desktop =
                refine_linux_desktop(LinuxDesktop::Unknown, Some(LinuxDesktop::Gnome), || {
                    kdeglobals
                });
            assert_eq!(desktop_font_dpi(desktop), Some(GNOME_FONT_DPI));
            assert_eq!(GNOME_FONT_DPI, 96.0);
        }
    }
}
