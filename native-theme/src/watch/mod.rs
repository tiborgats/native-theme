//! Runtime theme change watching.
//!
//! This module provides the public API for monitoring OS theme changes at
//! runtime. Call [`on_theme_change()`] with a callback to start watching;
//! the returned [`ThemeSubscription`] keeps the watcher alive via RAII semantics
//! -- dropping it stops the watcher and joins the background thread.
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
//! When a `ThemeSubscription` is dropped, shutdown proceeds in three phases:
//!
//! 1. **Platform-specific wakeup** -- if a platform shutdown closure was
//!    registered (see constructor below), it runs first. This wakes
//!    the background thread's event loop so it can observe the disconnect.
//! 2. **Channel disconnect** -- the shutdown channel sender is dropped,
//!    causing the receiver in the background thread to see `Disconnected`
//!    on its next `recv()` or `try_recv()`.
//! 3. **Thread join** -- `JoinHandle::join()` blocks until the background
//!    thread exits, ensuring clean shutdown before the guard is gone.
//!
//! # Constructor
//!
//! There is a single `pub(crate)` constructor:
//!
//! - [`ThemeSubscription::new(tx, handle, platform_shutdown)`] -- the optional
//!   `platform_shutdown` closure wakes the background thread's event loop on
//!   platforms where dropping the channel sender alone is not sufficient
//!   (`CFRunLoop::stop` on macOS, `PostThreadMessageW(WM_QUIT)` on Windows).
//!   Pass `None` on Linux where inotify/D-Bus poll the channel directly.
//!
//! # Signal-only events
//!
//! [`ThemeChangeEvent`] carries no theme data. When you receive an event,
//! re-run [`SystemTheme::from_system()`](crate::SystemTheme::from_system)
//! to get the updated theme.
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
/// [`SystemTheme::from_system()`](crate::SystemTheme::from_system)
/// to read the updated theme.
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
/// the background thread is joined.
///
/// Note: `ThemeSubscription` is `Send` but not `Sync`. The background thread
/// handle and shutdown channel are not safe to share across threads, but the
/// guard can be moved to a different thread if needed.
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
    /// Pass `None` on Linux where the channel disconnect alone suffices.
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
}

impl Drop for ThemeSubscription {
    fn drop(&mut self) {
        // Run the platform-specific shutdown action first (e.g. CFRunLoop::stop
        // on macOS, PostThreadMessageW WM_QUIT on Windows) to wake the blocked
        // event loop so it can observe the channel disconnect.
        if let Some(shutdown_fn) = self.platform_shutdown.take() {
            shutdown_fn();
        }
        // Drop the sender to signal shutdown (receiver sees Disconnected).
        drop(self.shutdown_tx.take());
        // Join the background thread so it finishes cleanly.
        if let Some(handle) = self.thread.take() {
            let _ = handle.join();
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
/// //     Ok(ThemeChangeEvent::Changed) => { /* re-read theme */ }
/// //     _ => {}
/// // }
/// # Ok::<(), native_theme::error::Error>(())
/// ```
///
/// # Errors
///
/// Returns [`Error::WatchUnavailable`](crate::Error::WatchUnavailable) if no
/// platform-specific backend is available for the current desktop
/// environment or platform.
pub fn on_theme_change(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<ThemeSubscription> {
    #[cfg(target_os = "linux")]
    {
        let de = crate::detect_linux_desktop();
        match de {
            #[cfg(feature = "kde")]
            crate::LinuxDesktop::Kde => kde::watch_kde(callback),

            #[cfg(feature = "portal")]
            crate::LinuxDesktop::Gnome | crate::LinuxDesktop::Budgie => {
                gnome::watch_gnome(callback)
            }

            _ => Err(crate::Error::WatchUnavailable {
                reason: "theme watching not supported for this desktop environment",
            }),
        }
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        {
            return macos::watch_macos(callback);
        }
        #[cfg(not(feature = "macos"))]
        {
            let _ = callback;
            return Err(crate::Error::WatchUnavailable {
                reason: "enable the 'macos' feature for macOS theme watching",
            });
        }
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        {
            return windows::watch_windows(callback);
        }
        #[cfg(not(feature = "windows"))]
        {
            let _ = callback;
            return Err(crate::Error::WatchUnavailable {
                reason: "enable the 'windows' feature for Windows theme watching",
            });
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        let _ = callback;
        Err(crate::Error::PlatformUnsupported {
            platform: "unsupported",
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

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

    #[test]
    fn theme_subscription_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ThemeSubscription>();
    }
}
