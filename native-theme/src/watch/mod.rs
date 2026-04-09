//! Runtime theme change watching.
//!
//! This module provides the public API for monitoring OS theme changes at
//! runtime. Call [`on_theme_change()`] with a callback to start watching;
//! the returned [`ThemeWatcher`] keeps the watcher alive via RAII semantics
//! -- dropping it stops the watcher and joins the background thread.
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
//! let _watcher = native_theme::on_theme_change(move |event| {
//!     let _ = tx.send(event);
//! }).expect("theme watching not supported on this platform");
//!
//! // On your UI thread:
//! // if let Ok(event) = rx.try_recv() {
//! //     let theme = native_theme::SystemTheme::from_system().unwrap();
//! //     // re-apply theme ...
//! // }
//! ```

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
///     ThemeChangeEvent::ColorSchemeChanged => { /* ... */ }
///     _ => { /* handle future variants */ }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ThemeChangeEvent {
    /// The OS color scheme (light/dark) changed.
    ColorSchemeChanged,
    /// An unclassified theme change occurred.
    Other,
}

/// RAII guard that keeps a theme watcher alive.
///
/// Holds a background thread and a shutdown channel. When dropped, the
/// channel sender is dropped (signaling shutdown via disconnection) and
/// the background thread is joined.
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
#[derive(Debug)]
#[must_use = "dropping ThemeWatcher stops the watcher immediately"]
pub struct ThemeWatcher {
    shutdown_tx: Option<mpsc::Sender<()>>,
    thread: Option<JoinHandle<()>>,
}

impl ThemeWatcher {
    /// Create a new `ThemeWatcher` from a shutdown channel and thread handle.
    ///
    /// This constructor is `pub(crate)` for use by platform-specific backends
    /// (implemented in later phases).
    #[allow(dead_code)] // Used by platform backends in Phase 66/67
    pub(crate) fn new(shutdown_tx: mpsc::Sender<()>, thread: JoinHandle<()>) -> Self {
        Self {
            shutdown_tx: Some(shutdown_tx),
            thread: Some(thread),
        }
    }
}

impl Drop for ThemeWatcher {
    fn drop(&mut self) {
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
///
/// let (tx, rx) = mpsc::channel();
/// let _watcher = native_theme::on_theme_change(move |event| {
///     let _ = tx.send(event);
/// }).expect("theme watching not supported");
///
/// // On UI thread: poll rx.try_recv()
/// ```
///
/// # Errors
///
/// Returns [`Error::Unsupported`](crate::Error::Unsupported) if no
/// platform-specific backend is available (the current state in Phase 65;
/// backends are added in Phases 66 and 67).
pub fn on_theme_change(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<ThemeWatcher> {
    let _ = callback;
    Err(crate::Error::Unsupported(
        "theme watching requires platform-specific backends (not yet implemented)",
    ))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn theme_change_event_variants_are_distinct() {
        assert_ne!(ThemeChangeEvent::ColorSchemeChanged, ThemeChangeEvent::Other);
    }

    #[test]
    fn theme_change_event_is_debug_clone_eq() {
        let event = ThemeChangeEvent::ColorSchemeChanged;
        let cloned = event.clone();
        assert_eq!(event, cloned);
        // Debug
        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("ColorSchemeChanged"));
    }

    #[test]
    fn on_theme_change_returns_unsupported() {
        let result = on_theme_change(|_| {});
        assert!(result.is_err());
        let err = result.unwrap_err();
        match &err {
            crate::Error::Unsupported(msg) => {
                assert!(msg.contains("platform-specific backends"), "got: {msg}");
            }
            other => panic!("expected Unsupported, got: {other:?}"),
        }
    }

    #[test]
    fn theme_watcher_drop_signals_shutdown() {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();
        let thread_handle = thread::spawn(move || {
            // Block until shutdown signal (channel disconnected)
            let _ = rx.recv();
        });

        let watcher = ThemeWatcher::new(tx, thread_handle);
        // Drop the watcher -- should signal shutdown and join thread
        drop(watcher);
        // If we get here, the thread was joined successfully (did not hang)
    }

    #[test]
    fn theme_watcher_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ThemeWatcher>();
    }
}
