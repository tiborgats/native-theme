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
/// Shutdown is primarily triggered by dropping the [`ThemeWatcher`], which
/// drops the `shutdown_tx` sender. Between signals, we also check the
/// shutdown channel via `try_recv()` as a best-effort early exit.
#[allow(dead_code)] // Dispatched from on_theme_change() in Phase 66 Plan 02
pub(crate) fn watch_gnome(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeWatcher> {
    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();

    let thread = std::thread::spawn(move || {
        // Create a blocking D-Bus session connection.
        let conn = match ashpd::zbus::blocking::Connection::session() {
            Ok(c) => c,
            Err(_) => return,
        };

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

    Ok(super::ThemeWatcher::new(shutdown_tx, thread))
}
