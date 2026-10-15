//! The OS theme change on gpui's foreground (feature `watch`).

use crate::{ThemeChangeEvent, ThemeSubscription};
use futures::StreamExt;
use futures::channel::mpsc;
use gpui::App;

/// Call `f` on gpui's foreground, with the [`App`], each time the OS theme
/// changes, for as long as the returned [`ThemeSubscription`] is held.
///
/// native-theme's watcher calls back on a thread of its own; this sends each
/// [`ThemeChangeEvent`] through a channel to a task spawned on the
/// foreground (`App::spawn`), which runs `f` with `&mut App` there, where
/// the theme can be applied. On an event, call
/// [`native_theme::detect::invalidate_caches()`], then read the theme again
/// with the entry point the application used
/// ([`SystemTheme::from_system()`](crate::SystemTheme::from_system), or
/// [`SystemTheme::from_system_with_preset`](crate::SystemTheme::from_system_with_preset)
/// for a pinned preset), then [`apply_system_theme`](crate::apply_system_theme):
/// the reader's detection values (the font DPI, dark mode, the icon theme,
/// the accessibility preferences) stay cached until the caches are cleared.
///
/// Dropping the subscription stops the watcher (see its "Dropping" section:
/// the drop joins the watcher's thread, and on macOS removes the observer);
/// once the watcher has let go of its callback the task ends and drops `f`.
/// An event sent before the drop can still reach `f` after it. Every event
/// means "read the theme again", and the watcher sends one per burst of OS
/// notifications, so the channel is unbounded: the watcher's thread never
/// waits for the foreground.
///
/// ```rust,ignore
/// let watcher = native_theme_gpui::on_theme_change(cx, |_event, cx| {
///     native_theme::detect::invalidate_caches();
///     if let Ok(sys) = native_theme_gpui::SystemTheme::from_system() {
///         native_theme_gpui::apply_system_theme(&sys, cx);
///     }
/// });
/// // Keep `watcher` alive (in the view, or a global) while the application
/// // follows the OS theme.
/// ```
///
/// # Errors
///
/// [`native_theme::watch::on_theme_change`]'s, at once, and no task is
/// spawned: [`Error::WatchUnavailable`](crate::Error::WatchUnavailable) on a
/// desktop with no watcher or without a reader feature (the `native`
/// feature, or native-theme's `kde`, `portal`, `macos` or `windows`),
/// [`Error::ReaderFailed`](crate::Error::ReaderFailed) where the backend
/// cannot start, [`Error::PlatformUnsupported`](crate::Error::PlatformUnsupported)
/// on a platform other than Linux, macOS and Windows.
pub fn on_theme_change(
    cx: &mut App,
    f: impl FnMut(ThemeChangeEvent, &mut App) + 'static,
) -> native_theme::Result<ThemeSubscription> {
    forward(cx, native_theme::watch::on_theme_change, f)
}

/// [`on_theme_change`] with the watcher's start as `start`, which is handed
/// the callback and returns the guard that holds it.
fn forward<G>(
    cx: &mut App,
    start: impl FnOnce(Box<dyn Fn(ThemeChangeEvent) + Send>) -> native_theme::Result<G>,
    mut f: impl FnMut(ThemeChangeEvent, &mut App) + 'static,
) -> native_theme::Result<G> {
    let (sender, mut events) = mpsc::unbounded();
    let watcher = start(Box::new(move |event| {
        // Fails only once the task has ended, when nobody is listening.
        let _ = sender.unbounded_send(event);
    }))?;
    cx.spawn(async move |cx| {
        while let Some(event) = events.next().await {
            cx.update(|cx| f(event, cx));
        }
    })
    .detach();
    Ok(watcher)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use gpui::{Global, TestAppContext};
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    use std::thread::ThreadId;

    type Callback = Box<dyn Fn(ThemeChangeEvent) + Send>;

    /// A stand-in for the OS watcher: it holds the callback until it is
    /// dropped, as the watcher's thread does until the drop joins it.
    struct FakeWatcher(Arc<Mutex<Option<Callback>>>);

    impl Drop for FakeWatcher {
        fn drop(&mut self) {
            self.0.lock().unwrap().take();
        }
    }

    /// What `f` saw, kept on the App, so the test reads it back through the
    /// App that `f` was handed.
    #[derive(Default)]
    struct Seen(Vec<(ThemeChangeEvent, ThreadId)>);

    impl Global for Seen {}

    fn start_fake(
        slot: &Arc<Mutex<Option<Callback>>>,
    ) -> impl FnOnce(Callback) -> native_theme::Result<FakeWatcher> {
        let slot = slot.clone();
        move |callback| {
            *slot.lock().unwrap() = Some(callback);
            Ok(FakeWatcher(slot))
        }
    }

    /// Send one event from another thread, as the watcher does.
    fn send_from_another_thread(slot: &Arc<Mutex<Option<Callback>>>) {
        let slot = slot.clone();
        std::thread::spawn(move || {
            if let Some(callback) = slot.lock().unwrap().as_ref() {
                callback(ThemeChangeEvent::Changed);
            }
        })
        .join()
        .unwrap();
    }

    #[gpui::test]
    fn an_event_sent_on_another_thread_runs_f_on_the_foreground_with_the_app(
        cx: &mut TestAppContext,
    ) {
        let slot = Arc::new(Mutex::new(None));
        let foreground = std::thread::current().id();
        let watcher = cx
            .update(|cx| {
                cx.set_global(Seen::default());
                forward(cx, start_fake(&slot), |event, cx| {
                    cx.global_mut::<Seen>()
                        .0
                        .push((event, std::thread::current().id()));
                })
            })
            .unwrap();

        send_from_another_thread(&slot);
        cx.run_until_parked();

        let seen = cx.update(|cx| cx.global::<Seen>().0.clone());
        assert_eq!(seen, vec![(ThemeChangeEvent::Changed, foreground)]);
        drop(watcher);
    }

    #[gpui::test]
    fn dropping_the_subscription_ends_the_task(cx: &mut TestAppContext) {
        let slot = Arc::new(Mutex::new(None));
        let calls = Rc::new(RefCell::new(0));
        let watcher = {
            let calls = calls.clone();
            cx.update(|cx| {
                forward(cx, start_fake(&slot), move |_, _| {
                    *calls.borrow_mut() += 1;
                })
            })
            .unwrap()
        };
        send_from_another_thread(&slot);
        cx.run_until_parked();
        assert_eq!(*calls.borrow(), 1);

        drop(watcher);
        cx.run_until_parked();
        assert!(
            slot.lock().unwrap().is_none(),
            "the watcher kept its callback"
        );
        // The task held the last other reference, through `f`: it has ended.
        assert_eq!(Rc::strong_count(&calls), 1, "the task still holds `f`");
        send_from_another_thread(&slot);
        cx.run_until_parked();
        assert_eq!(*calls.borrow(), 1);
    }

    #[gpui::test]
    fn a_watcher_that_cannot_start_returns_its_error_and_spawns_nothing(cx: &mut TestAppContext) {
        let calls = Rc::new(RefCell::new(0));
        let result = {
            let calls = calls.clone();
            cx.update(|cx| {
                forward(
                    cx,
                    |_callback| -> native_theme::Result<()> {
                        Err(crate::Error::WatchUnavailable { reason: "test" })
                    },
                    move |_, _| *calls.borrow_mut() += 1,
                )
            })
        };
        assert!(matches!(
            result,
            Err(crate::Error::WatchUnavailable { reason: "test" })
        ));
        cx.run_until_parked();
        assert_eq!(Rc::strong_count(&calls), 1, "a task holds `f`");
    }
}
