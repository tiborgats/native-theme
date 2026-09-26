use std::sync::{Arc, Mutex, PoisonError};

use crate::ThemeAtlas;

/// The hand-off between the watcher thread and the UI thread (§10.2): the pending atlas and
/// the last rebuild error. A poisoned lock is taken as it is — the data is a plain `Option`.
#[derive(Default)]
pub(crate) struct Handoff {
    pending: Mutex<Option<ThemeAtlas>>,
    last_error: Mutex<Option<String>>,
}

impl Handoff {
    /// The step the watcher thread runs on each OS change: `rebuild`, store, wake egui.
    pub(crate) fn on_change(
        &self,
        ctx: &egui::Context,
        rebuild: &dyn Fn() -> native_theme::Result<ThemeAtlas>,
    ) {
        match rebuild() {
            Ok(atlas) => {
                *self.pending.lock().unwrap_or_else(PoisonError::into_inner) = Some(atlas);
                *self
                    .last_error
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner) = None;
                ctx.request_repaint();
            }
            Err(error) => {
                *self
                    .last_error
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner) = Some(error.to_string());
            }
        }
    }

    pub(crate) fn take(&self) -> Option<ThemeAtlas> {
        self.pending
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
    }

    pub(crate) fn last_error(&self) -> Option<String> {
        self.last_error
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

/// Watch for OS theme changes, rebuild the atlas off the UI thread with the application's
/// `rebuild` closure, and wake egui with `ctx.request_repaint()` (§10.2). Installation stays
/// on the UI thread: [`ThemeWatcher::take`] is the hand-off.
#[must_use = "dropping the watcher stops it immediately"]
pub struct ThemeWatcher {
    handoff: Arc<Handoff>,
    _subscription: native_theme::watch::ThemeSubscription,
}

impl ThemeWatcher {
    /// Start watching. `rebuild` runs on the watcher thread after each change event and
    /// returns the atlas to hand over: [`ThemeWatcher::system_rebuild`] for an application on
    /// the OS theme, else a closure that builds exactly what the application built at start-up,
    /// every builder input included, after `native_theme::detect::invalidate_caches()`
    /// (`native-theme/src/detect.rs:155`) when it reads the OS (§10.2).
    ///
    /// Bind the result: dropping it stops the watch and joins the thread
    /// (`native-theme/src/watch/mod.rs:172-187`). `Send` but not `Sync`, like native-theme's
    /// subscription (`native-theme/src/watch/mod.rs:107-109`).
    ///
    /// # Errors
    /// Propagates `native_theme::watch::on_theme_change` (`native-theme/src/watch/mod.rs:217`),
    /// which reports `Error::WatchUnavailable` on desktops and feature sets it cannot watch
    /// (`:234-266`). A `rebuild` error is not returned here; it is kept for
    /// [`ThemeWatcher::last_error`].
    pub fn start(
        ctx: &egui::Context,
        rebuild: impl Fn() -> native_theme::Result<ThemeAtlas> + Send + 'static,
    ) -> native_theme::Result<ThemeWatcher> {
        let handoff = Arc::new(Handoff::default());
        let worker = Arc::clone(&handoff);
        let ctx = ctx.clone();
        let subscription =
            native_theme::watch::on_theme_change(move |_event| worker.on_change(&ctx, &rebuild))?;
        Ok(Self {
            handoff,
            _subscription: subscription,
        })
    }

    /// The OS-theme rebuild: `native_theme::detect::invalidate_caches()`, then
    /// [`from_system`](crate::from_system)'s atlas.
    ///
    /// # Errors
    /// Propagates [`from_system`](crate::from_system).
    pub fn system_rebuild() -> native_theme::Result<ThemeAtlas> {
        native_theme::detect::invalidate_caches();
        crate::from_system().map(|(atlas, _, _)| atlas)
    }

    /// Take the pending atlas, if the OS theme changed since the last call. Non-blocking, and
    /// `None` on the overwhelming majority of frames. Install the result; `install` flushes
    /// the icon cache and requests the repaint (§10.3).
    #[must_use]
    pub fn take(&self) -> Option<ThemeAtlas> {
        self.handoff.take()
    }

    /// The most recent `rebuild`'s error, as text — a `String`, not an [`Error`](crate::Error), because
    /// `native_theme::error::Error` is not `Clone` (`native-theme/src/error.rs:58-60`); `None`
    /// again once a later `rebuild` succeeds.
    #[must_use]
    pub fn last_error(&self) -> Option<String> {
        self.handoff.last_error()
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use native_theme::AccessibilityPreferences;
    use native_theme::error::Error;
    use native_theme::theme::Theme;

    use super::*;

    fn adwaita() -> native_theme::Result<ThemeAtlas> {
        crate::from_preset("adwaita", false, &AccessibilityPreferences::default()).map(|(a, _)| a)
    }

    /// T15 (a): one change → one rebuild, one atlas handed over once, one repaint.
    #[test]
    fn a_rebuilt_atlas_is_handed_over_once_with_one_repaint() {
        let ctx = egui::Context::default();
        let repaints = Arc::new(AtomicUsize::new(0));
        let seen = Arc::clone(&repaints);
        ctx.set_request_repaint_callback(move |_| {
            seen.fetch_add(1, Ordering::SeqCst);
        });
        let runs = Arc::new(AtomicUsize::new(0));
        let counted = Arc::clone(&runs);
        let rebuild = move || {
            counted.fetch_add(1, Ordering::SeqCst);
            adwaita()
        };
        let handoff = Handoff::default();
        handoff.on_change(&ctx, &rebuild);
        let expected = Theme::preset("adwaita").unwrap().name;
        assert_eq!(
            handoff.take().map(|a| a.name().to_owned()),
            Some(expected.to_string())
        );
        assert!(handoff.take().is_none());
        assert_eq!(runs.load(Ordering::SeqCst), 1);
        assert_eq!(repaints.load(Ordering::SeqCst), 1);
        assert!(ctx.has_requested_repaint());
        assert!(handoff.last_error().is_none());
    }

    /// T15 (b): a failed rebuild publishes nothing and keeps its message until a later success.
    #[test]
    fn a_failed_rebuild_hands_over_nothing_and_keeps_its_message() {
        let ctx = egui::Context::default();
        let handoff = Handoff::default();
        let failing = || {
            Err(Error::WatchUnavailable {
                reason: "native-theme-egui test",
            })
        };
        handoff.on_change(&ctx, &failing);
        assert!(handoff.take().is_none());
        assert!(
            handoff
                .last_error()
                .is_some_and(|m| m.contains("native-theme-egui test"))
        );
        handoff.on_change(&ctx, &adwaita);
        assert!(handoff.take().is_some());
        assert!(handoff.last_error().is_none());
    }

    /// T15 (c): `start` never panics, whatever the desktop (`native-theme/src/watch/mod.rs:311-325`):
    /// `Ok` or any error passes, and only a panic fails.
    #[test]
    fn start_never_panics_without_a_desktop() {
        let ctx = egui::Context::default();
        if let Err(e) = ThemeWatcher::start(&ctx, ThemeWatcher::system_rebuild) {
            println!("start returned an error: {e:?}");
        }
    }
}
