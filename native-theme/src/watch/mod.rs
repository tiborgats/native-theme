//! Runtime theme change watching.
//!
//! Requires the `watch` feature.
//!
//! This module provides the public API for monitoring OS theme changes at
//! runtime. Call [`on_theme_change()`](crate::watch::on_theme_change) with a callback to start watching;
//! the returned [`ThemeSubscription`] keeps the watcher alive via RAII semantics
//! -- dropping it stops the watcher and joins the background thread (macOS has
//! no watcher thread: dropping it removes the notification observer).
//!
//! # RAII ownership model
//!
//! [`ThemeSubscription`] is an RAII guard. Dropping it stops the watcher and
//! joins the background thread. You **must** bind it to a variable -- if
//! you discard the return value, the watcher is dropped immediately and
//! no events are ever delivered.
//!
//! # Shutdown mechanism
//!
//! When a `ThemeSubscription` is dropped, shutdown proceeds in three phases
//! (on macOS only the first, which removes the observer):
//!
//! 1. **Platform-specific wakeup** -- where the backend has one, it runs
//!    first (GNOME closes its D-Bus connection, Windows posts `WM_QUIT`).
//!    This wakes the background thread's event loop so it can observe the
//!    disconnect.
//! 2. **Channel disconnect** -- the shutdown channel sender is dropped,
//!    causing the receiver in the background thread to see `Disconnected`
//!    on its next `recv()` or `try_recv()`.
//! 3. **Thread join** -- `JoinHandle::join()` blocks until the background
//!    thread exits, ensuring clean shutdown before the guard is gone.
//!
//! # Signal-only events
//!
//! [`ThemeChangeEvent`] carries no theme data. When you receive an event,
//! call [`detect::invalidate_caches()`](crate::detect::invalidate_caches),
//! then re-run [`SystemTheme::from_system()`](crate::SystemTheme::from_system)
//! to get the updated theme: the detection values the reader uses (the font
//! DPI, dark mode, the icon theme, the accessibility preferences) stay cached
//! until the caches are cleared.
//!
//! # Example
//!
//! ```no_run
//! use std::sync::mpsc;
//!
//! let (tx, rx) = mpsc::channel();
//! let _watcher = native_theme::watch::on_theme_change(move |event| {
//!     let _ = tx.send(event);
//! })?;
//!
//! // On your UI thread:
//! // if let Ok(event) = rx.try_recv() {
//! //     native_theme::detect::invalidate_caches();
//! //     let theme = native_theme::SystemTheme::from_system()?;
//! //     // re-apply theme ...
//! // }
//! # Ok::<(), native_theme::error::Error>(())
//! ```

#[cfg(all(feature = "watch", feature = "kde", target_os = "linux"))]
mod kde;

#[cfg(all(feature = "watch", feature = "portal", target_os = "linux"))]
mod gnome;

#[cfg(all(feature = "watch", feature = "macos", target_os = "macos"))]
mod macos;

#[cfg(all(feature = "watch", feature = "windows", target_os = "windows"))]
mod windows;

use std::sync::mpsc;
use std::thread::JoinHandle;

/// A signal that the OS theme has changed.
///
/// This enum carries no theme data -- it is a notification only.
/// When you receive an event, call
/// [`detect::invalidate_caches()`](crate::detect::invalidate_caches), then
/// [`SystemTheme::from_system()`](crate::SystemTheme::from_system)
/// to read the updated theme; without the first call, the reader's detection
/// values (the font DPI, dark mode, the icon theme) are the cached ones.
///
/// # Non-exhaustive
///
/// Future versions may add new variants. Always include a wildcard arm:
///
/// ```ignore
/// match event {
///     ThemeChangeEvent::Changed => { /* ... */ }
///     _ => { /* handle future variants */ }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ThemeChangeEvent {
    /// The OS theme changed.
    Changed,
}

/// RAII guard that keeps a theme watcher alive.
///
/// Holds a background thread and a shutdown channel. When dropped, the
/// channel sender is dropped (signaling shutdown via disconnection) and
/// the background thread is joined. On macOS it holds the notification
/// observer instead, and dropping it removes the observer.
///
/// Note: `ThemeSubscription` is `Send` but not `Sync`. The background thread
/// handle and shutdown channel are not safe to share across threads, but the
/// guard can be moved to a different thread if needed.
///
/// # Dropping
///
/// `Drop` joins the watcher thread: a callback that is running when the drop
/// starts finishes before the drop returns, and none runs after it. A
/// callback may drop its own subscription: the drop then runs on the watcher
/// thread, so it does not join it but detaches it, and the thread exits once
/// the callback returns. On macOS, where there is no watcher thread,
/// dropping removes the observer and waits for a callback already queued to
/// finish; dropped from inside the callback, it returns without waiting, and
/// a notification already queued can still run the callback once.
///
/// Because the drop waits for the callback, it deadlocks if a callback waits
/// on a lock the dropping thread holds, or, on macOS, dispatches
/// synchronously to the main thread while the main thread drops the
/// subscription.
///
/// # Important
///
/// You **must** bind this to a variable. If you discard it, the watcher
/// stops immediately:
///
/// ```ignore
/// // WRONG -- watcher is dropped immediately:
/// on_theme_change(|e| println!("{e:?}"));
///
/// // RIGHT -- watcher lives as long as `_watcher`:
/// let _watcher = on_theme_change(|e| println!("{e:?}")).unwrap();
/// ```
#[must_use]
pub struct ThemeSubscription {
    shutdown_tx: Option<mpsc::Sender<()>>,
    thread: Option<JoinHandle<()>>,
    platform_shutdown: Option<Box<dyn FnOnce() + Send>>,
}

impl std::fmt::Debug for ThemeSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThemeSubscription")
            .field("shutdown_tx", &self.shutdown_tx)
            .field("thread", &self.thread)
            .field(
                "platform_shutdown",
                &self.platform_shutdown.as_ref().map(|_| "..."),
            )
            .finish()
    }
}

impl ThemeSubscription {
    /// Create a new `ThemeSubscription` from a shutdown channel, thread handle,
    /// and optional platform-specific shutdown action.
    ///
    /// The `platform_shutdown` closure (if `Some`) is called **before** the
    /// channel is dropped during `Drop`, allowing platform backends to wake
    /// their blocked event loops so the thread can observe the disconnect.
    /// Pass `None` on KDE, where the inotify loop polls the channel; GNOME's
    /// backend closes its D-Bus connection here.
    ///
    /// Compiled where a backend exists to call it, and for tests.
    #[cfg(any(
        test,
        all(target_os = "linux", any(feature = "kde", feature = "portal")),
        all(target_os = "windows", feature = "windows"),
    ))]
    pub(crate) fn new(
        shutdown_tx: mpsc::Sender<()>,
        thread: JoinHandle<()>,
        platform_shutdown: Option<Box<dyn FnOnce() + Send>>,
    ) -> Self {
        Self {
            shutdown_tx: Some(shutdown_tx),
            thread: Some(thread),
            platform_shutdown,
        }
    }

    /// Create a `ThemeSubscription` for a watcher without a thread: dropping
    /// it only runs `platform_shutdown`.
    #[cfg(all(target_os = "macos", feature = "macos"))]
    pub(crate) fn from_platform_shutdown(platform_shutdown: Box<dyn FnOnce() + Send>) -> Self {
        Self {
            shutdown_tx: None,
            thread: None,
            platform_shutdown: Some(platform_shutdown),
        }
    }
}

impl Drop for ThemeSubscription {
    fn drop(&mut self) {
        // Run the platform-specific shutdown action first (PostThreadMessageW
        // WM_QUIT on Windows, Connection::close on GNOME) to wake the blocked
        // event loop so it can observe the channel disconnect; on macOS it
        // removes the notification observer.
        if let Some(shutdown_fn) = self.platform_shutdown.take() {
            shutdown_fn();
        }
        // Drop the sender to signal shutdown (receiver sees Disconnected).
        drop(self.shutdown_tx.take());
        // Join the background thread so it finishes cleanly. A callback that
        // drops its own subscription runs on that thread, and a thread that
        // joins itself fails (EDEADLK): dropping the handle detaches the
        // thread instead, which exits once the callback returns.
        if let Some(handle) = self.thread.take()
            && handle.thread().id() != std::thread::current().id()
        {
            let _ = handle.join();
        }
    }
}

/// Call `callback` with [`ThemeChangeEvent::Changed`], containing a panic:
/// a callback that panics loses that one event, and the watcher keeps
/// running. This holds with the default `panic = "unwind"`; under
/// `panic = "abort"` a panicking callback aborts the process.
#[cfg(any(
    test,
    all(target_os = "linux", any(feature = "kde", feature = "portal")),
    all(target_os = "macos", feature = "macos"),
    all(target_os = "windows", feature = "windows"),
))]
fn fire(callback: &impl Fn(ThemeChangeEvent)) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        callback(ThemeChangeEvent::Changed)
    }));
}

/// How long a watcher waits after the last notification of a burst before
/// it fires: one user action can raise several notifications (KDE writes its
/// files in several steps, `QSaveFile`'s atomic writes; GNOME's portal sends
/// a gsettings key and the `org.freedesktop.appearance` setting it maps to),
/// and the callback fires once for all of them. The value is T20's KDE
/// window, reused on every platform to keep one user action one event: a
/// design value, not a platform fact.
#[cfg(any(
    test,
    all(target_os = "linux", any(feature = "kde", feature = "portal")),
    all(target_os = "macos", feature = "macos"),
    all(target_os = "windows", feature = "windows"),
))]
const DEBOUNCE_MS: u32 = 300;

/// [`DEBOUNCE_MS`] as a [`Duration`](std::time::Duration).
#[cfg(any(
    test,
    all(target_os = "linux", any(feature = "kde", feature = "portal")),
    all(target_os = "macos", feature = "macos"),
))]
const DEBOUNCE: std::time::Duration = std::time::Duration::from_millis(DEBOUNCE_MS as u64);

/// Fire `callback` once per burst of relevant messages from `events`,
/// [`DEBOUNCE`] after the last of them; `is_relevant` sorts them. Returns
/// when `events` disconnects or `shutdown` is signalled or disconnected,
/// which is checked on every iteration, at least every 200ms.
#[cfg(any(
    test,
    all(target_os = "linux", any(feature = "kde", feature = "portal"))
))]
fn fire_debounced<T>(
    events: &mpsc::Receiver<T>,
    shutdown: &mpsc::Receiver<()>,
    mut is_relevant: impl FnMut(T) -> bool,
    callback: &impl Fn(ThemeChangeEvent),
) {
    use std::time::{Duration, Instant};

    let poll = Duration::from_millis(200);
    // When the last relevant message not yet fired arrived.
    let mut pending: Option<Instant> = None;

    loop {
        match shutdown.try_recv() {
            Ok(()) | Err(mpsc::TryRecvError::Disconnected) => break,
            Err(mpsc::TryRecvError::Empty) => {}
        }

        let wait = pending.map_or(poll, |last| {
            DEBOUNCE.saturating_sub(last.elapsed()).min(poll)
        });
        match events.recv_timeout(wait) {
            Ok(message) => {
                if is_relevant(message) {
                    pending = Some(Instant::now());
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }

        if pending.is_some_and(|last| last.elapsed() >= DEBOUNCE) {
            pending = None;
            fire(callback);
        }
    }
}

/// Start watching for OS theme changes.
///
/// The `callback` is invoked on a **background thread** whenever the OS
/// theme changes. To marshal events to your UI thread, send them through
/// a channel:
///
/// ```no_run
/// use std::sync::mpsc;
/// use native_theme::watch::ThemeChangeEvent;
///
/// let (tx, rx) = mpsc::channel();
/// let _watcher = native_theme::watch::on_theme_change(move |event| {
///     let _ = tx.send(event);
/// })?;
///
/// // On UI thread:
/// // match rx.try_recv() {
/// //     Ok(ThemeChangeEvent::Changed) => {
/// //         native_theme::detect::invalidate_caches();
/// //         let theme = native_theme::SystemTheme::from_system()?;
/// //         // re-apply theme ...
/// //     }
/// //     _ => {}
/// // }
/// # Ok::<(), native_theme::error::Error>(())
/// ```
///
/// Clear the detection caches before the re-read, as above: the reader's
/// detection values (the font DPI, dark mode, the icon theme, the
/// accessibility preferences) are cached until
/// [`detect::invalidate_caches()`](crate::detect::invalidate_caches).
///
/// On macOS the change arrives as a distributed notification, which is
/// delivered through the process's main run loop, so the application must be
/// running one in a common mode; every GUI toolkit's event loop does
/// (AppKit's `NSApplication::run`). See Apple's Notification Programming
/// Topics, ["Notification Centers"](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Notifications/Articles/NotificationCenters.html).
/// The callback then runs on a private serial queue, never on the main
/// thread and never twice at once.
///
/// The watcher observes what the platform's reader reads (each reader
/// module lists its sources): on KDE `kdeglobals`, `kdedefaults/kdeglobals`
/// and `kcmfonts` in the config directory; on GNOME and Budgie the
/// portal's `SettingChanged` for the `org.freedesktop.appearance` settings
/// and the `org.gnome.desktop.interface`, `org.gnome.desktop.wm.preferences`
/// and `org.gnome.desktop.a11y.interface` keys the reader reads; on macOS
/// the appearance, system-colour, scroller-style and accessibility-display
/// notifications; on Windows `UISettings::ColorValuesChanged`,
/// `TextScaleFactorChanged`, `AnimationsEnabledChanged` and
/// `AdvancedEffectsEnabledChanged` (a change of high contrast is announced to
/// top-level windows only, and the watcher has none). One change can raise several of them, so the
/// callback fires once per burst, 300 ms after the last.
///
/// A callback that panics loses that one event: the panic is contained, and
/// the watcher keeps running (with the default `panic = "unwind"`; under
/// `panic = "abort"` the process aborts). A callback may drop the returned
/// [`ThemeSubscription`]; see its "Dropping" section for what a drop waits
/// for.
///
/// On Linux the backend is the one of the desktop the reader is chosen by:
/// KDE's for KDE (with the `kde` feature), GNOME's for GNOME and Budgie (with
/// `portal`). An unrecognised `XDG_CURRENT_DESKTOP` counts as the desktop of
/// its portal backend once a system read such as
/// [`SystemTheme::from_system`](crate::SystemTheme::from_system) has asked
/// it, else as KDE where the user's `kdeglobals` exists.
///
/// # Errors
///
/// Returns [`Error::WatchUnavailable`](crate::Error::WatchUnavailable) if no
/// platform-specific backend is available for the current desktop
/// environment or platform, and
/// [`Error::ReaderFailed`](crate::Error::ReaderFailed) if the backend could
/// not start: KDE, the config directory is unresolved (`$XDG_CONFIG_HOME` is
/// not an absolute path and `$HOME` is unset or empty) or cannot be watched
/// (it does not exist, or the inotify limits are reached); GNOME and Budgie,
/// the watcher thread could not connect to the session bus or subscribe to
/// the portal's `SettingChanged` signal; Windows, the watcher thread did not
/// start or could not subscribe to `UISettings::ColorValuesChanged` or
/// `TextScaleFactorChanged`. The KDE backend sets up its watch on the
/// calling thread, and the GNOME and Windows backends return only once their
/// thread is subscribed; the macOS backend registers its observer on the
/// calling thread and does not fail.
pub fn on_theme_change(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<ThemeSubscription> {
    #[cfg(target_os = "linux")]
    {
        #[cfg(not(any(feature = "kde", feature = "portal")))]
        let _ = callback;
        match linux_backend(crate::detect::system().refined_linux_desktop_sync()) {
            #[cfg(feature = "kde")]
            Some(LinuxBackend::Kde) => kde::watch_kde(callback),

            #[cfg(feature = "portal")]
            Some(LinuxBackend::Gnome) => gnome::watch_gnome(callback),

            _ => Err(crate::Error::WatchUnavailable {
                reason: "theme watching not supported for this desktop environment",
            }),
        }
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        {
            macos::watch_macos(callback)
        }
        #[cfg(not(feature = "macos"))]
        {
            let _ = callback;
            Err(crate::Error::WatchUnavailable {
                reason: "enable the 'macos' feature for macOS theme watching",
            })
        }
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        {
            windows::watch_windows(callback)
        }
        #[cfg(not(feature = "windows"))]
        {
            let _ = callback;
            Err(crate::Error::WatchUnavailable {
                reason: "enable the 'windows' feature for Windows theme watching",
            })
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        let _ = callback;
        Err(crate::Error::PlatformUnsupported {
            platform: std::env::consts::OS,
        })
    }
}

/// A Linux watcher backend.
#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LinuxBackend {
    /// [`kde::watch_kde`].
    #[cfg(feature = "kde")]
    Kde,
    /// [`gnome::watch_gnome`].
    #[cfg(feature = "portal")]
    Gnome,
}

/// The backend that watches `desktop`, the refined desktop the reader is
/// chosen by ([`crate::detect::refine_linux_desktop`]); `None` where there is
/// none.
#[cfg(target_os = "linux")]
fn linux_backend(desktop: crate::LinuxDesktop) -> Option<LinuxBackend> {
    match desktop {
        #[cfg(feature = "kde")]
        crate::LinuxDesktop::Kde => Some(LinuxBackend::Kde),
        #[cfg(feature = "portal")]
        crate::LinuxDesktop::Gnome | crate::LinuxDesktop::Budgie => Some(LinuxBackend::Gnome),
        _ => None,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// The watcher follows the reader on an unrecognised desktop: a KDE
    /// portal backend gets KDE's watcher (M4-36).
    #[cfg(all(target_os = "linux", feature = "kde"))]
    #[test]
    fn an_unknown_desktop_with_a_kde_portal_is_watched_as_kde() {
        for kdeglobals in [false, true] {
            let desktop = crate::detect::refine_linux_desktop(
                crate::LinuxDesktop::Unknown,
                Some(crate::LinuxDesktop::Kde),
                || kdeglobals,
            );
            assert_eq!(linux_backend(desktop), Some(LinuxBackend::Kde));
        }
    }

    /// A GNOME portal backend gets GNOME's watcher, `kdeglobals` or not.
    #[cfg(all(target_os = "linux", feature = "portal"))]
    #[test]
    fn an_unknown_desktop_with_a_gnome_portal_is_watched_as_gnome() {
        for kdeglobals in [false, true] {
            let desktop = crate::detect::refine_linux_desktop(
                crate::LinuxDesktop::Unknown,
                Some(crate::LinuxDesktop::Gnome),
                || kdeglobals,
            );
            assert_eq!(linux_backend(desktop), Some(LinuxBackend::Gnome));
        }
    }

    /// An unrecognised desktop that nothing names has no watcher.
    #[cfg(target_os = "linux")]
    #[test]
    fn an_unknown_desktop_that_nothing_names_is_not_watched() {
        let desktop =
            crate::detect::refine_linux_desktop(crate::LinuxDesktop::Unknown, None, || false);
        assert_eq!(linux_backend(desktop), None);
    }

    /// The source-to-notification table (M1-T6-08): for each reader, every
    /// source its module lists under `# Sources`, and the notification its
    /// watcher observes for it: a file in the config directory (KDE), a
    /// `namespace key` of the portal's `SettingChanged` (GNOME), a
    /// notification name (macOS) or a `UISettings` event (Windows).
    const SOURCE_NOTIFICATIONS: &[(&str, &str, &str)] = &[
        ("kde", "kdeglobals", "kdeglobals"),
        ("kde", "kdedefaults/kdeglobals", "kdedefaults/kdeglobals"),
        ("kde", "kcmfonts", "kcmfonts"),
        // xdg-desktop-portal-kde's accent is the palette's highlight, which
        // the colour scheme in kdeglobals sets.
        (
            "kde",
            "org.freedesktop.appearance accent-color",
            "kdeglobals",
        ),
        (
            "gnome",
            "org.freedesktop.appearance color-scheme",
            "org.freedesktop.appearance color-scheme",
        ),
        (
            "gnome",
            "org.freedesktop.appearance accent-color",
            "org.freedesktop.appearance accent-color",
        ),
        (
            "gnome",
            "org.freedesktop.appearance contrast",
            "org.freedesktop.appearance contrast",
        ),
        (
            "gnome",
            "org.freedesktop.appearance reduced-motion",
            "org.freedesktop.appearance reduced-motion",
        ),
        (
            "gnome",
            "org.gnome.desktop.interface color-scheme",
            "org.gnome.desktop.interface color-scheme",
        ),
        (
            "gnome",
            "org.gnome.desktop.interface enable-animations",
            "org.gnome.desktop.interface enable-animations",
        ),
        (
            "gnome",
            "org.gnome.desktop.interface font-name",
            "org.gnome.desktop.interface font-name",
        ),
        (
            "gnome",
            "org.gnome.desktop.interface monospace-font-name",
            "org.gnome.desktop.interface monospace-font-name",
        ),
        (
            "gnome",
            "org.gnome.desktop.interface text-scaling-factor",
            "org.gnome.desktop.interface text-scaling-factor",
        ),
        (
            "gnome",
            "org.gnome.desktop.interface overlay-scrolling",
            "org.gnome.desktop.interface overlay-scrolling",
        ),
        (
            "gnome",
            "org.gnome.desktop.interface icon-theme",
            "org.gnome.desktop.interface icon-theme",
        ),
        (
            "gnome",
            "org.gnome.desktop.wm.preferences titlebar-font",
            "org.gnome.desktop.wm.preferences titlebar-font",
        ),
        (
            "gnome",
            "org.gnome.desktop.a11y.interface high-contrast",
            "org.gnome.desktop.a11y.interface high-contrast",
        ),
        (
            "macos",
            "AppleInterfaceStyle",
            "AppleInterfaceThemeChangedNotification",
        ),
        ("macos", "NSColor", "NSSystemColorsDidChangeNotification"),
        (
            "macos",
            "NSScroller.preferredScrollerStyle",
            "NSPreferredScrollerStyleDidChangeNotification",
        ),
        (
            "macos",
            "NSWorkspace.accessibilityDisplayOptions",
            "NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification",
        ),
        ("windows", "UISettings.GetColorValue", "ColorValuesChanged"),
        (
            "windows",
            "UISettings.TextScaleFactor",
            "TextScaleFactorChanged",
        ),
        // `UISettings.AnimationsEnabled` and SPI_GETCLIENTAREAANIMATION read
        // the same on the three Windows probe images; no page says they are
        // one setting.
        (
            "windows",
            "SPI_GETCLIENTAREAANIMATION",
            "AnimationsEnabledChanged",
        ),
        (
            "windows",
            "UISettings.AdvancedEffectsEnabled",
            "AdvancedEffectsEnabledChanged",
        ),
    ];

    /// Each reader's module source, whose docs list its sources.
    const READER_MODULES: &[(&str, &str)] = &[
        ("kde", include_str!("../kde/mod.rs")),
        ("gnome", include_str!("../gnome/mod.rs")),
        ("macos", include_str!("../macos.rs")),
        ("windows", include_str!("../windows.rs")),
    ];

    /// The sources a reader module lists: the first code span of each item
    /// of its `# Sources` doc section.
    fn listed_sources(module: &'static str) -> Vec<&'static str> {
        module
            .lines()
            .skip_while(|line| line.trim_end() != "//! # Sources")
            .skip(1)
            .take_while(|line| line.starts_with("//!") && !line.starts_with("//! #"))
            .filter_map(|line| line.strip_prefix("//! - `"))
            .filter_map(|item| item.split('`').next())
            .collect()
    }

    /// The notifications the table gives for `reader`'s sources.
    #[cfg(any(
        all(target_os = "linux", any(feature = "kde", feature = "portal")),
        all(target_os = "macos", feature = "macos"),
        all(target_os = "windows", feature = "windows"),
    ))]
    fn table_notifications(reader: &str) -> impl Iterator<Item = &'static str> {
        SOURCE_NOTIFICATIONS
            .iter()
            .filter(move |(row_reader, _, _)| *row_reader == reader)
            .map(|(_, _, notification)| *notification)
    }

    /// Every source a reader lists has a row in [`SOURCE_NOTIFICATIONS`],
    /// and every row names a listed source.
    #[test]
    fn every_listed_source_has_a_table_row() {
        for (reader, module) in READER_MODULES {
            let sources = listed_sources(module);
            assert!(!sources.is_empty(), "{reader}: no `# Sources` list");
            for source in &sources {
                assert!(
                    SOURCE_NOTIFICATIONS
                        .iter()
                        .any(|(r, s, _)| r == reader && s == source),
                    "{reader}: the source `{source}` has no row in SOURCE_NOTIFICATIONS"
                );
            }
            for (_, source, _) in SOURCE_NOTIFICATIONS.iter().filter(|(r, _, _)| r == reader) {
                assert!(
                    sources.contains(source),
                    "{reader}: the row for `{source}` names a source the reader does not list"
                );
            }
        }
    }

    /// The KDE watcher observes the file of every KDE row.
    #[cfg(all(target_os = "linux", feature = "kde"))]
    #[test]
    fn the_kde_watcher_observes_every_kde_source() {
        for file in table_notifications("kde") {
            assert!(
                kde::WATCHED_FILES.contains(&file),
                "the KDE watcher does not observe `{file}`"
            );
        }
    }

    /// The GNOME watcher fires on the `SettingChanged` of every GNOME row.
    #[cfg(all(target_os = "linux", feature = "portal"))]
    #[test]
    fn the_gnome_watcher_observes_every_gnome_source() {
        for setting in table_notifications("gnome") {
            let (namespace, key) = setting.split_once(' ').unwrap();
            assert!(
                gnome::watches(namespace, key),
                "the GNOME watcher ignores the SettingChanged of `{setting}`"
            );
        }
    }

    /// Every gsettings key the GNOME reader reads is a listed source.
    #[cfg(all(target_os = "linux", feature = "portal"))]
    #[test]
    fn the_gnome_reader_lists_every_gsetting_it_reads() {
        let sources = listed_sources(include_str!("../gnome/mod.rs"));
        let mut read = Vec::new();
        let _ = crate::gnome::build_theme(
            None,
            None,
            ashpd::desktop::settings::Contrast::default(),
            None,
            |schema, key| {
                read.push(format!("{schema} {key}"));
                None
            },
        );
        assert!(!read.is_empty());
        for setting in &read {
            assert!(
                sources.contains(&setting.as_str()),
                "the GNOME reader reads `{setting}`, which its `# Sources` do not list"
            );
        }
    }

    /// The macOS watcher observes the notification of every macOS row.
    #[cfg(all(target_os = "macos", feature = "macos"))]
    #[test]
    fn the_macos_watcher_observes_every_macos_source() {
        let observed: Vec<&str> = macos::registrations().iter().map(|r| r.name_id).collect();
        for notification in table_notifications("macos") {
            assert!(
                observed.contains(&notification),
                "the macOS watcher does not observe `{notification}`"
            );
        }
    }

    /// The Windows watcher subscribes to the event of every Windows row.
    #[cfg(all(target_os = "windows", feature = "windows"))]
    #[test]
    fn the_windows_watcher_observes_every_windows_source() {
        for event in table_notifications("windows") {
            assert!(
                windows::SUBSCRIBED_EVENTS.contains(&event),
                "the Windows watcher does not subscribe to `{event}`"
            );
        }
    }

    #[test]
    fn theme_change_event_is_debug_clone_eq() {
        let event = ThemeChangeEvent::Changed;
        let cloned = event.clone();
        assert_eq!(event, cloned);
        // Debug
        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("Changed"));
    }

    /// On unsupported platforms, on_theme_change() returns PlatformUnsupported.
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    #[test]
    fn on_theme_change_returns_unsupported() {
        let result = on_theme_change(|_| {});
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(&err, crate::Error::PlatformUnsupported { .. }),
            "expected PlatformUnsupported, got: {err:?}"
        );
    }

    /// On Linux, on_theme_change() dispatches based on the detected DE.
    /// In CI (no DE running), XDG_CURRENT_DESKTOP is usually empty/Unknown,
    /// so we get WatchUnavailable. On a real DE it may succeed or return
    /// ReaderFailed. All outcomes are valid.
    #[cfg(target_os = "linux")]
    #[test]
    fn on_theme_change_dispatches_or_returns_error() {
        let result = on_theme_change(|_| {});
        assert!(
            matches!(
                &result,
                Ok(_)
                    | Err(crate::Error::WatchUnavailable { .. })
                    | Err(crate::Error::PlatformUnsupported { .. })
                    | Err(crate::Error::ReaderFailed { .. })
            ),
            "unexpected result: {result:?}"
        );
    }

    #[test]
    fn theme_subscription_drop_signals_shutdown() {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();
        let thread_handle = thread::spawn(move || {
            // Block until shutdown signal (channel disconnected)
            let _ = rx.recv();
        });

        let watcher = ThemeSubscription::new(tx, thread_handle, None);
        // Drop the watcher -- should signal shutdown and join thread
        drop(watcher);
        // If we get here, the thread was joined successfully (did not hang)
    }

    /// A panicking callback is contained: `fire` returns, and the next call
    /// runs the callback again (M4-33).
    #[test]
    fn fire_contains_a_panicking_callback() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let calls = AtomicUsize::new(0);
        let callback = |_| {
            let call = calls.fetch_add(1, Ordering::SeqCst);
            assert!(call > 0, "the first call panics");
        };
        fire(&callback);
        fire(&callback);
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    /// A burst of relevant messages fires once, after the last of them; an
    /// irrelevant one fires nothing; `fire_debounced` returns when its
    /// channel disconnects.
    #[test]
    fn fire_debounced_fires_once_per_burst() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::time::{Duration, Instant};

        let (events_tx, events_rx) = mpsc::channel::<bool>();
        let (_shutdown_tx, shutdown_rx) = mpsc::channel::<()>();
        let sender = std::thread::spawn(move || {
            for relevant in [true, false, true, true] {
                let _ = events_tx.send(relevant);
                std::thread::sleep(Duration::from_millis(50));
            }
            let last = Instant::now();
            std::thread::sleep(DEBOUNCE * 3);
            let _ = events_tx.send(false);
            std::thread::sleep(DEBOUNCE * 2);
            last
        });

        let fired = std::sync::Mutex::new(Vec::new());
        let calls = AtomicUsize::new(0);
        fire_debounced(&events_rx, &shutdown_rx, |relevant| relevant, &|_| {
            calls.fetch_add(1, Ordering::SeqCst);
            fired.lock().unwrap().push(Instant::now());
        });
        let last = sender.join().unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1, "a burst fires once");
        assert!(fired.lock().unwrap().iter().all(|at| *at >= last));
    }

    #[test]
    fn theme_subscription_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ThemeSubscription>();
    }

    /// Start the platform watcher and drop it at once, `rounds` times. Each
    /// round subscribes and drops on a helper thread, with nothing between
    /// the two, so a drop that hangs fails the test after `per_drop_timeout`
    /// instead of blocking it. A platform or desktop without a watcher
    /// (`WatchUnavailable`) skips the test.
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    fn assert_subscribe_and_drop_returns(rounds: usize, per_drop_timeout: std::time::Duration) {
        for round in 0..rounds {
            let (done_tx, done_rx) = mpsc::channel();
            std::thread::spawn(move || {
                let result = on_theme_change(|_| {}).map(drop);
                let _ = done_tx.send(result);
            });
            match done_rx.recv_timeout(per_drop_timeout) {
                Ok(Ok(())) => {}
                Ok(Err(crate::Error::WatchUnavailable { .. })) => return,
                Ok(Err(err)) => panic!("round {round}: on_theme_change failed: {err:?}"),
                Err(_) => panic!(
                    "round {round}: subscribing and dropping did not return within {per_drop_timeout:?}"
                ),
            }
        }
    }

    #[cfg(all(target_os = "macos", feature = "watch"))]
    #[test]
    fn macos_subscribe_and_drop_returns() {
        assert_subscribe_and_drop_returns(1000, std::time::Duration::from_secs(5));
    }

    #[cfg(all(target_os = "windows", feature = "watch"))]
    #[test]
    fn windows_subscribe_and_drop_returns() {
        assert_subscribe_and_drop_returns(1000, std::time::Duration::from_secs(5));
    }

    /// The KDE watcher polls its shutdown channel every 200 ms, so a drop
    /// takes up to that long: a few rounds keep the test short.
    #[cfg(all(target_os = "linux", feature = "watch"))]
    #[test]
    fn linux_subscribe_and_drop_returns() {
        assert_subscribe_and_drop_returns(10, std::time::Duration::from_secs(5));
    }
}
