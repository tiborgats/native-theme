//! OS detection: the Linux desktop environment, and the detection caches,
//! which the icon loaders' icon-theme detection fills too.

use arc_swap::ArcSwapOption;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

mod icon_theme;

pub(crate) use icon_theme::IconThemeFailure;
pub(crate) use icon_theme::{system_icon_set, system_icon_theme};

/// Timeout for subprocess commands ([`run_with_timeout`]: gsettings,
/// xfconf-query, fc-match).
///
/// Prevents D-Bus-dependent tools from blocking indefinitely when the
/// session bus is unresponsive or the display server is unavailable.
#[cfg(unix)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub(crate) const SUBPROCESS_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(2);

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
/// else `Unknown`. The icon-theme detection decides by it
/// ([`DetectionContext::refined_linux_desktop_sync`]). `kdeglobals_exists`
/// is asked only where nothing before it decides.
#[cfg(target_os = "linux")]
pub(crate) fn refine_linux_desktop(
    raw: LinuxDesktop,
    portal_backend: Option<LinuxDesktop>,
    kdeglobals_exists: impl FnOnce() -> bool,
) -> LinuxDesktop {
    match (raw, portal_backend) {
        (LinuxDesktop::Unknown, Some(backend)) => backend,
        (LinuxDesktop::Unknown, None) if kdeglobals_exists() => LinuxDesktop::Kde,
        (other, _) => other,
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

/// The desktop a system read is read by ([`refine_linux_desktop`]): for
/// `Unknown`, the portal backend is asked first (with the `portal`
/// feature), then the user's `kdeglobals`. The raw desktop is the cached
/// one ([`DetectionContext::linux_desktop`]); the portal's answer is not
/// cached, as each read asks it once.
#[cfg(target_os = "linux")]
pub(crate) async fn refined_linux_desktop() -> LinuxDesktop {
    let raw = system().linux_desktop();
    #[cfg(feature = "portal")]
    let portal = match raw {
        LinuxDesktop::Unknown => crate::gnome::detect_portal_backend().await,
        _ => None,
    };
    #[cfg(not(feature = "portal"))]
    let portal = None;
    refine_linux_desktop(raw, portal, kdeglobals_exists)
}

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

/// The session type, `$XDG_SESSION_TYPE` (`x11`, `wayland`, …).
#[cfg(all(target_os = "linux", feature = "portal"))]
pub(crate) fn session_type() -> Option<String> {
    std::env::var("XDG_SESSION_TYPE").ok()
}

/// Reset the process-wide detection caches, so the next detection of the
/// icon theme or the desktop environment re-queries the OS; a cached failure
/// to detect the icon theme is cleared too.
///
/// Call this when you detect that the user has changed system settings
/// (e.g., an icon theme switch). The installed freedesktop icon themes are
/// not re-read: the icon lookup reads them once per process, and so do
/// [`is_freedesktop_theme_available()`](crate::icons::is_freedesktop_theme_available)
/// and [`list_freedesktop_themes()`](crate::icons::list_freedesktop_themes).
pub fn invalidate_caches() {
    system().invalidate_all();
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
#[cfg_attr(
    not(all(
        feature = "system-fonts",
        not(target_os = "macos"),
        not(target_os = "ios"),
        not(target_os = "android")
    )),
    allow(dead_code)
)]
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
struct CacheFill<T> {
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
    #[cfg(test)]
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
/// Provides "cache on first read" semantics for OS detection queries (the
/// icon theme, and the Linux desktop environment) and per-field
/// invalidation for watchers that need fresh data.
///
/// Obtain the process-wide instance via [`system()`].
///
/// # Thread Safety
///
/// All reads and invalidations are lock-free (backed by
/// [`arc_swap::ArcSwapOption`]). A value detected while an invalidation of
/// its field lands is returned to its caller but not cached.
#[non_exhaustive]
pub struct DetectionContext {
    icon_theme: CacheSlot<Result<String, IconThemeFailure>>,
    #[cfg(target_os = "linux")]
    linux_desktop: CacheSlot<LinuxDesktop>,
}

impl DetectionContext {
    /// Create an empty context with no cached values.
    pub(crate) fn new() -> Self {
        Self {
            icon_theme: CacheSlot::empty(),
            #[cfg(target_os = "linux")]
            linux_desktop: CacheSlot::empty(),
        }
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
    pub(crate) fn icon_theme(&self) -> crate::Result<String> {
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

    /// The desktop the icon-theme detection uses, and the icon enums
    /// native-theme-build generates pick their per-desktop names by: the raw
    /// `XDG_CURRENT_DESKTOP` ([`linux_desktop()`](Self::linux_desktop)) where
    /// it names a desktop; for [`LinuxDesktop::Unknown`], KDE where the
    /// user's `kdeglobals` exists (with the `kde` feature), else `Unknown`
    /// (the portal backend is not asked). The raw desktop is cached;
    /// [`invalidate_linux_desktop()`](Self::invalidate_linux_desktop) clears
    /// it.
    #[cfg(target_os = "linux")]
    #[must_use]
    pub fn refined_linux_desktop_sync(&self) -> LinuxDesktop {
        refine_linux_desktop(self.linux_desktop(), None, kdeglobals_exists)
    }

    /// Clear the cached icon theme, or the cached failure to detect one.
    pub fn invalidate_icon_theme(&self) {
        self.icon_theme.invalidate();
    }

    /// Clear the cached Linux desktop environment.
    #[cfg(target_os = "linux")]
    pub fn invalidate_linux_desktop(&self) {
        self.linux_desktop.invalidate();
    }

    /// Clear all cached values.
    pub fn invalidate_all(&self) {
        self.invalidate_icon_theme();
        #[cfg(target_os = "linux")]
        self.invalidate_linux_desktop();
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

#[cfg(test)]
mod cache_slot_tests {
    use super::*;

    /// The slot every cached field uses ([`DetectionContext::icon_theme`],
    /// [`DetectionContext::linux_desktop`]): a value detected across an
    /// invalidation is returned, not cached.
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
        let _ = ctx.icon_theme.get_or_fill(|| Ok("breeze".to_string()));
        #[cfg(target_os = "linux")]
        ctx.linux_desktop.get_or_fill(|| LinuxDesktop::Kde);
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
            icon_theme,
            #[cfg(target_os = "linux")]
            linux_desktop,
        } = &ctx;
        assert!(icon_theme.get().is_none(), "icon_theme");
        #[cfg(target_os = "linux")]
        assert!(linux_desktop.get().is_none(), "linux_desktop");
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

    /// The timeout of a test whose command finishes: a loaded machine never
    /// takes that long, where it could take `SUBPROCESS_TIMEOUT`'s 2 s, the
    /// readers' design value.
    const GENEROUS: Duration = Duration::from_secs(10);

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
        let out = stdout_with_timeout("echo", &["hi"], GENEROUS);
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
        let output = run_with_timeout("sh", &["-c", "echo oops >&2; exit 3"], GENEROUS).unwrap();
        assert_eq!(output.status.code(), Some(3));
        assert_eq!(output.stderr, b"oops\n");
        assert_eq!(stdout_with_timeout("sh", &["-c", "exit 3"], GENEROUS), None);
    }

    #[test]
    fn run_with_timeout_reports_a_missing_program_as_not_found() {
        let error = run_with_timeout("nt-test-no-such-program", &[], GENEROUS).unwrap_err();
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
            stdout_with_timeout("true", &[], GENEROUS).as_deref(),
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
    /// call ends at the timeout, not when the command would have: its sleep
    /// is 30 s, and the 5 s bound leaves a loaded machine room to stall.
    #[test]
    fn run_with_timeout_kills_a_command_that_outlives_the_timeout() {
        if !on_path("sleep") {
            eprintln!("skipped: sleep is not installed");
            return;
        }
        let timeout = Duration::from_millis(100);
        let seconds = format!("30.{}", std::process::id());
        let start = Instant::now();
        let error = run_with_timeout("sleep", &[&seconds], timeout).unwrap_err();
        let elapsed = start.elapsed();
        assert_eq!(error.kind(), std::io::ErrorKind::TimedOut);
        assert_eq!(error.to_string(), "timed out after 100ms");
        assert!(
            elapsed >= timeout && elapsed < Duration::from_secs(5),
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
        let out = stdout_with_timeout("sh", &["-c", "head -c 102400 /dev/zero"], GENEROUS);
        assert_eq!(out.map(|out| out.len()), Some(102_400));
    }

    /// A process the command leaves running with its stdout keeps the pipe
    /// open after the command exits; the call still ends at the deadline,
    /// not when that process exits: it sleeps 10 s, and the 5 s bound leaves
    /// a loaded machine room to stall.
    #[test]
    fn run_with_timeout_ends_at_the_deadline_when_a_grandchild_holds_stdout() {
        if !on_path("sh") || !on_path("sleep") {
            eprintln!("skipped: sh or sleep is not installed");
            return;
        }
        let script = format!("sleep 10.{} & echo hi", std::process::id());
        let start = Instant::now();
        let error =
            run_with_timeout("sh", &["-c", &script], Duration::from_millis(100)).unwrap_err();
        let elapsed = start.elapsed();
        assert_eq!(error.kind(), std::io::ErrorKind::TimedOut);
        assert!(elapsed < Duration::from_secs(5), "took {elapsed:?}");
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
