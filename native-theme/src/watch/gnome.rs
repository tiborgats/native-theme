//! GNOME theme change watcher using XDG Desktop Portal D-Bus signals.

use std::sync::mpsc;

use super::ThemeChangeEvent;

/// Spawn a background thread that watches for GNOME theme changes via D-Bus.
///
/// Subscribes to the `SettingChanged` signal on the
/// `org.freedesktop.portal.Settings` interface, filtered to the
/// `org.freedesktop.appearance` namespace. Uses `zbus::blocking` so no
/// async runtime (tokio) is exposed to the consumer.
///
/// The signal iterator is blocking -- it waits for the next D-Bus signal.
/// Dropping the [`ThemeSubscription`](super::ThemeSubscription) closes the
/// bus connection first (its platform shutdown), which ends the iterator, and
/// then drops the `shutdown_tx` sender, which the loop also checks between
/// signals.
#[allow(dead_code)] // Dispatched from on_theme_change() in Phase 66 Plan 02
pub(crate) fn watch_gnome(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeSubscription> {
    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();

    // The thread hands its bus connection back, as the macOS backend hands
    // its run loop back, so that the platform shutdown can close it.
    let (conn_tx, conn_rx) = mpsc::channel::<ashpd::zbus::blocking::Connection>();

    let thread = std::thread::spawn(move || {
        // Create a blocking D-Bus session connection.
        let conn = match ashpd::zbus::blocking::Connection::session() {
            Ok(c) => c,
            Err(_) => return,
        };
        if conn_tx.send(conn.clone()).is_err() {
            return;
        }

        // Create a proxy to the XDG Desktop Portal Settings interface.
        let proxy = match ashpd::zbus::blocking::Proxy::new(
            &conn,
            "org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
            "org.freedesktop.portal.Settings",
        ) {
            Ok(p) => p,
            Err(_) => return,
        };

        // Subscribe to SettingChanged signals filtered by namespace.
        // Argument 0 is the namespace string; filtering server-side avoids
        // unnecessary traffic from unrelated portal settings.
        let signals = match proxy
            .receive_signal_with_args("SettingChanged", &[(0, "org.freedesktop.appearance")])
        {
            Ok(s) => s,
            Err(_) => return,
        };

        // Ends when the connection is closed from `ThemeSubscription::drop`:
        // the closed socket ends zbus's message stream, and the iterator
        // returns `None`.
        for signal in signals {
            // Check shutdown between signals (best-effort early exit).
            match shutdown_rx.try_recv() {
                Ok(()) | Err(mpsc::TryRecvError::Disconnected) => break,
                Err(mpsc::TryRecvError::Empty) => {}
            }

            // We only need the notification, not the signal payload.
            let _ = signal;
            callback(ThemeChangeEvent::Changed);
        }
    });

    // Receive the connection from the watcher thread; a thread that could
    // not connect has already ended.
    let conn = conn_rx.recv().map_err(|_| crate::Error::ReaderFailed {
        reader: "gnome_watcher",
        source: "GNOME watcher thread could not connect to the session bus".into(),
    })?;

    // Build the platform shutdown closure: closing the connection fails the
    // socket read the watcher thread is blocked in, so its signal iterator
    // returns `None` and the thread exits before `Drop` joins it.
    let platform_shutdown = Box::new(move || {
        let _ = conn.close();
    });

    Ok(super::ThemeSubscription::new(
        shutdown_tx,
        thread,
        Some(platform_shutdown),
    ))
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::time::Duration;

    /// egui spec §10.2: dropping the subscription must not wait for the next
    /// `org.freedesktop.appearance` signal. It discriminates only where a
    /// session bus runs `xdg-desktop-portal` (KDE's portal serves
    /// `org.freedesktop.portal.Settings` too); with no bus the watcher thread
    /// ends at once, or `watch_gnome` errs, and the test passes vacuously.
    #[test]
    fn dropping_a_gnome_subscription_returns_without_a_signal() {
        let subscription = match super::watch_gnome(|_| {}) {
            Ok(subscription) => subscription,
            Err(_) => return, // no session bus: nothing to discriminate
        };
        let (done_tx, done_rx) = mpsc::channel::<()>();
        std::thread::spawn(move || {
            drop(subscription);
            let _ = done_tx.send(());
        });
        assert!(
            done_rx.recv_timeout(Duration::from_secs(5)).is_ok(),
            "dropping the GNOME subscription blocked for 5 s: the watcher thread is still \
             inside the blocking signal iterator"
        );
    }
}
