//! The OS theme change on gpui's foreground (feature `watch`).

use crate::{ThemeChangeEvent, ThemeSubscription};
use futures::StreamExt;
use futures::channel::mpsc;
use gpui::{App, Window};

/// Call `f` on gpui's foreground, with `window` and the [`App`], each time
/// the OS theme changes, for as long as the returned [`ThemeSubscription`]
/// is held and `window` is open.
///
/// native-theme's watcher calls back on a thread of its own; this sends each
/// [`ThemeChangeEvent`] through a channel to a task spawned on the
/// foreground for `window` (`Window::spawn`), which runs `f` with
/// `&mut Window` and `&mut App` there, where the theme can be applied. On an
/// event, call [`native_theme::detect::invalidate_caches()`], then read the
/// theme again with the entry point the application used
/// ([`SystemTheme::from_system()`](crate::SystemTheme::from_system), or
/// [`SystemTheme::from_system_with_preset`](crate::SystemTheme::from_system_with_preset)
/// for a pinned preset), then [`apply_system_theme`](crate::apply_system_theme):
/// the reader's detection values (the font DPI, dark mode, the icon theme,
/// the accessibility preferences) stay cached until the caches are cleared.
///
/// The subscription lives as long as the guard and the window it was
/// registered on. Register it on the application's longest-lived window,
/// the main window: once that window closes (or the application quits),
/// events stop reaching `f`, since nothing is left to theme through it, and
/// the task ends at the next event and drops `f`.
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
/// let watcher = native_theme_gpui::on_theme_change(window, cx, |_event, _window, cx| {
///     native_theme::detect::invalidate_caches();
///     if let Ok(sys) = native_theme_gpui::SystemTheme::from_system() {
///         native_theme_gpui::apply_system_theme(&sys, cx);
///     }
/// });
/// // Keep `watcher` alive (in the main window's view) while the application
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
    window: &mut Window,
    cx: &mut App,
    f: impl FnMut(ThemeChangeEvent, &mut Window, &mut App) + 'static,
) -> native_theme::Result<ThemeSubscription> {
    forward(window, cx, native_theme::watch::on_theme_change, f)
}

/// [`on_theme_change`] with the watcher's start as `start`, which is handed
/// the callback and returns the guard that holds it.
fn forward<G>(
    window: &mut Window,
    cx: &mut App,
    start: impl FnOnce(Box<dyn Fn(ThemeChangeEvent) + Send>) -> native_theme::Result<G>,
    mut f: impl FnMut(ThemeChangeEvent, &mut Window, &mut App) + 'static,
) -> native_theme::Result<G> {
    let (sender, mut events) = mpsc::unbounded();
    let watcher = start(Box::new(move |event| {
        // Fails only once the task has ended, when nobody is listening.
        let _ = sender.unbounded_send(event);
    }))?;
    window
        .spawn(cx, async move |cx| {
            while let Some(event) = events.next().await {
                // `Err` once the window is closed or the application gone
                // (gpui's `AsyncWindowContext::update`): nobody is left to
                // theme.
                if cx.update(|window, cx| f(event, window, cx)).is_err() {
                    break;
                }
            }
        })
        .detach();
    Ok(watcher)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use gpui::{Context, Empty, IntoElement, Render, TestAppContext, WindowHandle};
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

    /// The window's view; it draws nothing.
    struct Blank;

    impl Render for Blank {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            Empty
        }
    }

    fn start_fake(
        slot: &Arc<Mutex<Option<Callback>>>,
    ) -> impl FnOnce(Callback) -> native_theme::Result<FakeWatcher> {
        let slot = slot.clone();
        move |callback| {
            *slot.lock().unwrap() = Some(callback);
            Ok(FakeWatcher(slot))
        }
    }

    /// Send one event from another thread, as the watcher does. Only before
    /// the task first waits: gpui's test scheduler fails a test whose task is
    /// woken from a thread other than the test's, as nondeterministic
    /// (gpui-pre-scheduler 0.3.9 `test_scheduler.rs:194`); a real dispatcher
    /// takes the wake-up from any thread.
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

    /// Send one event from the test's own thread, once the task waits.
    fn send(slot: &Arc<Mutex<Option<Callback>>>) {
        if let Some(callback) = slot.lock().unwrap().as_ref() {
            callback(ThemeChangeEvent::Changed);
        }
    }

    /// A window, and a watcher registered on it whose `f` counts its calls
    /// in `calls`.
    fn watched_window(
        cx: &mut TestAppContext,
        slot: &Arc<Mutex<Option<Callback>>>,
        calls: &Rc<RefCell<u32>>,
    ) -> (WindowHandle<Blank>, FakeWatcher) {
        let window = cx.add_window(|_, _| Blank);
        let calls = calls.clone();
        let watcher = window
            .update(cx, |_, window, cx| {
                forward(window, cx, start_fake(slot), move |_, _, _| {
                    *calls.borrow_mut() += 1;
                })
            })
            .unwrap()
            .unwrap();
        (window, watcher)
    }

    #[gpui::test]
    fn an_event_sent_on_another_thread_runs_f_on_the_foreground_with_the_window(
        cx: &mut TestAppContext,
    ) {
        let slot = Arc::new(Mutex::new(None));
        let foreground = std::thread::current().id();
        let seen: Rc<RefCell<Vec<(ThemeChangeEvent, ThreadId, bool)>>> = Rc::default();
        let window = cx.add_window(|_, _| Blank);
        let handle = window.into();
        let watcher = {
            let seen = seen.clone();
            window
                .update(cx, |_, window, cx| {
                    forward(window, cx, start_fake(&slot), move |event, window, cx| {
                        // The window `f` is handed is the one it was
                        // registered on, and the App reaches it.
                        let ours =
                            window.window_handle() == handle && cx.windows().contains(&handle);
                        seen.borrow_mut()
                            .push((event, std::thread::current().id(), ours));
                    })
                })
                .unwrap()
                .unwrap()
        };

        send_from_another_thread(&slot);
        cx.run_until_parked();

        assert_eq!(
            *seen.borrow(),
            vec![(ThemeChangeEvent::Changed, foreground, true)]
        );
        drop(watcher);
    }

    #[gpui::test]
    fn dropping_the_subscription_ends_the_task(cx: &mut TestAppContext) {
        let slot = Arc::new(Mutex::new(None));
        let calls = Rc::new(RefCell::new(0));
        let (_window, watcher) = watched_window(cx, &slot, &calls);
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
        send(&slot);
        cx.run_until_parked();
        assert_eq!(*calls.borrow(), 1);
    }

    #[gpui::test]
    fn once_the_window_is_closed_an_event_does_not_call_f(cx: &mut TestAppContext) {
        let slot = Arc::new(Mutex::new(None));
        let calls = Rc::new(RefCell::new(0));
        let (window, watcher) = watched_window(cx, &slot, &calls);
        send_from_another_thread(&slot);
        cx.run_until_parked();
        assert_eq!(*calls.borrow(), 1);

        window
            .update(cx, |_, window, _| window.remove_window())
            .unwrap();
        cx.run_until_parked();
        assert!(
            !cx.windows().contains(&window.into()),
            "the window did not close"
        );

        send(&slot);
        cx.run_until_parked();
        assert_eq!(*calls.borrow(), 1, "`f` ran for a closed window");
        // The event found no window, so the task ended and dropped `f`.
        assert_eq!(Rc::strong_count(&calls), 1, "the task still holds `f`");
        drop(watcher);
    }

    #[gpui::test]
    fn a_watcher_that_cannot_start_returns_its_error_and_spawns_nothing(cx: &mut TestAppContext) {
        let calls = Rc::new(RefCell::new(0));
        let window = cx.add_window(|_, _| Blank);
        let result = {
            let calls = calls.clone();
            window
                .update(cx, |_, window, cx| {
                    forward(
                        window,
                        cx,
                        |_callback| -> native_theme::Result<()> {
                            Err(crate::Error::WatchUnavailable { reason: "test" })
                        },
                        move |_, _, _| *calls.borrow_mut() += 1,
                    )
                })
                .unwrap()
        };
        assert!(matches!(
            result,
            Err(crate::Error::WatchUnavailable { reason: "test" })
        ));
        cx.run_until_parked();
        assert_eq!(Rc::strong_count(&calls), 1, "a task holds `f`");
    }
}
