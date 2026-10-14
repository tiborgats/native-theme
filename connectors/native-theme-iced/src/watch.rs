//! The OS theme change as an iced subscription (feature `watch`).

use iced_futures::Subscription;
use iced_futures::futures::channel::mpsc;
use iced_futures::futures::{SinkExt, Stream};
use native_theme::watch::{ThemeChangeEvent, on_theme_change};
use std::sync::{Mutex, PoisonError};

/// The OS theme changes, as an iced [`Subscription`]: return it from the
/// application's `subscription()` and map it to a message.
///
/// Each item is `Ok` with a [`ThemeChangeEvent`] when the OS theme changed.
/// On one, call [`native_theme::detect::invalidate_caches()`], then read the
/// theme again with the entry point the application used
/// ([`from_system()`](crate::from_system), or
/// [`SystemTheme::from_system_with_preset`](native_theme::SystemTheme::from_system_with_preset)
/// and [`to_iced_theme()`](crate::SystemThemeExt::to_iced_theme) for a pinned
/// preset), then rebuild what was built from the old theme, such as a
/// `Spinner`: the reader's detection values (the font DPI, dark mode, the
/// icon theme, the accessibility preferences) stay cached until the caches
/// are cleared.
///
/// The watcher starts when iced first runs the subscription and stops when
/// `subscription()` no longer returns it. If it cannot start
/// ([`native_theme::watch::on_theme_change`]'s errors:
/// [`Error::WatchUnavailable`](crate::Error::WatchUnavailable) on a desktop
/// with no watcher or without a reader feature,
/// [`Error::ReaderFailed`](crate::Error::ReaderFailed) where the backend
/// fails, [`Error::PlatformUnsupported`](crate::Error::PlatformUnsupported)
/// on a platform other than Linux, macOS and Windows), the subscription
/// yields that `Err` once and ends. Watching needs this crate's `watch`
/// feature and a reader: the `native` feature, or native-theme's `kde`,
/// `portal`, `macos` or `windows`.
///
/// Dropping the subscription drops the watcher's
/// [`ThemeSubscription`](native_theme::watch::ThemeSubscription) on the
/// executor thread that ran the stream, and that drop joins the watcher's
/// thread (on macOS it removes the observer; see its "Dropping" section).
///
/// The watcher calls back on its own thread, which never waits for the
/// application: the subscription holds two events unread, and drops an event
/// that arrives while they wait. One pending event is enough, since every
/// event means "read the theme again".
///
/// ```rust,ignore
/// fn subscription(_state: &State) -> iced::Subscription<Message> {
///     // A watcher that cannot start yields its error once; this drops it.
///     native_theme_iced::theme_changes()
///         .filter_map(|changed| changed.ok().map(|_| Message::ThemeChanged))
/// }
///
/// // In update():
/// Message::ThemeChanged => {
///     native_theme::detect::invalidate_caches();
///     if let Ok(loaded) = native_theme_iced::from_system() {
///         state.spinner = Spinner::new(&loaded.resolved, loaded.icon_set, loaded.icon_theme.as_deref());
///         state.loaded = loaded;
///     }
/// }
/// ```
#[must_use = "the subscription watches nothing until `subscription()` returns it"]
pub fn theme_changes() -> Subscription<native_theme::Result<ThemeChangeEvent>> {
    Subscription::run(|| change_stream(on_theme_change))
}

/// The stream [`theme_changes()`] runs: `start` is handed the callback and
/// starts the watcher, whose guard the stream holds until it is dropped.
fn change_stream<G: Send + 'static>(
    start: impl FnOnce(Box<dyn Fn(ThemeChangeEvent) + Send>) -> native_theme::Result<G> + Send + 'static,
) -> impl Stream<Item = native_theme::Result<ThemeChangeEvent>> {
    iced_futures::stream::channel(
        1,
        async move |mut sender: mpsc::Sender<native_theme::Result<ThemeChangeEvent>>| {
            // Two senders: the callback's clone is the only one that sends
            // events, so it alone sets the bound (the channel's one slot
            // plus its own: two unread); the original sends nothing but the
            // `Err` below.
            let events = Mutex::new(sender.clone());
            let callback = move |event| {
                let _ = events
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .try_send(Ok(event));
            };
            match start(Box::new(callback)) {
                Ok(_watcher) => std::future::pending::<()>().await,
                Err(error) => {
                    let _ = sender.send(Err(error)).await;
                }
            }
        },
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use iced_futures::futures::executor::block_on;
    use iced_futures::futures::{FutureExt, StreamExt};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn a_started_watcher_yields_each_event() {
        let stream = change_stream(|callback| {
            callback(ThemeChangeEvent::Changed);
            callback(ThemeChangeEvent::Changed);
            Ok(())
        });
        let items: Vec<_> = block_on(stream.take(2).collect());
        assert_eq!(items.len(), 2);
        for item in items {
            assert_eq!(item.unwrap(), ThemeChangeEvent::Changed);
        }
    }

    #[test]
    fn an_event_that_arrives_while_two_wait_is_dropped() {
        let mut stream = Box::pin(change_stream(|callback| {
            for _ in 0..3 {
                callback(ThemeChangeEvent::Changed);
            }
            Ok(())
        }));
        assert!(block_on(stream.next()).is_some());
        assert!(block_on(stream.next()).is_some());
        assert!(stream.next().now_or_never().is_none());
    }

    #[test]
    fn dropping_the_stream_drops_the_watcher() {
        struct Watcher(Arc<AtomicBool>);
        impl Drop for Watcher {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
            }
        }
        let dropped = Arc::new(AtomicBool::new(false));
        let watcher = Watcher(dropped.clone());
        let mut stream = Box::pin(change_stream(move |_callback| Ok(watcher)));
        assert!(stream.next().now_or_never().is_none());
        assert!(
            !dropped.load(Ordering::SeqCst),
            "the running stream dropped its watcher"
        );
        drop(stream);
        assert!(
            dropped.load(Ordering::SeqCst),
            "the dropped stream kept its watcher"
        );
    }

    #[test]
    fn a_watcher_that_cannot_start_yields_its_error_once_and_ends() {
        let stream = change_stream(|_callback| {
            Err::<(), _>(native_theme::Error::WatchUnavailable {
                reason: "the test's watcher",
            })
        });
        let items: Vec<_> = block_on(stream.collect());
        assert_eq!(items.len(), 1);
        assert!(matches!(
            items.first(),
            Some(Err(native_theme::Error::WatchUnavailable {
                reason: "the test's watcher"
            }))
        ));
    }
}
