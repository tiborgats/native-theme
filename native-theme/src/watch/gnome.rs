//! GNOME theme change watcher using XDG Desktop Portal D-Bus signals.

use std::sync::mpsc;

use super::ThemeChangeEvent;

/// The `SettingChanged` namespaces and keys the watcher fires on: the GNOME
/// reader's sources (`gnome/mod.rs`, "Sources"). The portal's GNOME and GTK
/// backends forward a change of a key in `org.gnome.desktop.interface`,
/// `org.gnome.desktop.wm.preferences` or `org.gnome.desktop.a11y.interface`
/// under its schema's name, and the `org.freedesktop.appearance` settings
/// with them (xdg-desktop-portal-gnome 51.0 `src/settings.c:389-438`,
/// `:444-455`; xdg-desktop-portal-gtk 1.15.3 `src/settings.c:266-297`,
/// `:303-314`), and the portal passes each on (xdg-desktop-portal 1.22.1
/// `src/settings.c:240-250`).
const WATCHED: &[(&str, &str)] = &[
    ("org.freedesktop.appearance", "color-scheme"),
    ("org.freedesktop.appearance", "accent-color"),
    ("org.freedesktop.appearance", "contrast"),
    ("org.freedesktop.appearance", "reduced-motion"),
    ("org.gnome.desktop.interface", "color-scheme"),
    ("org.gnome.desktop.interface", "enable-animations"),
    ("org.gnome.desktop.interface", "font-name"),
    ("org.gnome.desktop.interface", "monospace-font-name"),
    ("org.gnome.desktop.interface", "text-scaling-factor"),
    ("org.gnome.desktop.interface", "overlay-scrolling"),
    ("org.gnome.desktop.interface", "icon-theme"),
    ("org.gnome.desktop.wm.preferences", "titlebar-font"),
    ("org.gnome.desktop.a11y.interface", "high-contrast"),
];

/// Whether the watcher fires on the `SettingChanged` of `namespace` `key`.
pub(super) fn watches(namespace: &str, key: &str) -> bool {
    WATCHED.contains(&(namespace, key))
}

/// Whether `signal`, a `SettingChanged`, is of a setting the watcher fires
/// on ([`watches`]); a body that does not decode is not.
fn is_watched(signal: &ashpd::zbus::Message) -> bool {
    signal
        .body()
        .deserialize::<(&str, &str, ashpd::zbus::zvariant::Value<'_>)>()
        .is_ok_and(|(namespace, key, _)| watches(namespace, key))
}

/// Spawn a background thread that watches for GNOME theme changes via D-Bus.
///
/// Subscribes to the `SettingChanged` signal on the
/// `org.freedesktop.portal.Settings` interface, and fires on the settings of
/// [`WATCHED`], once per burst, 300ms after the last of them
/// ([`super::fire_debounced`]): one change raises a gsettings key's signal
/// and its `org.freedesktop.appearance` setting's. Uses `zbus::blocking` so
/// no async runtime (tokio) is exposed to the consumer.
///
/// The signal iterator is blocking -- it waits for the next D-Bus signal --
/// so it runs on a thread of its own, which passes the watched signals to
/// the watcher thread's debounce. Dropping the
/// [`ThemeSubscription`](super::ThemeSubscription) closes the bus connection
/// first (its platform shutdown), which ends the iterator and its thread,
/// and then drops the `shutdown_tx` sender, which the debounce also checks.
///
/// Returns once the thread is subscribed, or with `Error::ReaderFailed` when
/// a setup step failed.
pub(crate) fn watch_gnome(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeSubscription> {
    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();

    // The thread hands its bus connection back once it is subscribed, so
    // that the platform shutdown can close it, or the setup step that failed.
    let (ready_tx, ready_rx) = mpsc::channel::<crate::Result<ashpd::zbus::blocking::Connection>>();

    let thread = std::thread::spawn(move || {
        let setup_failed = |step: &str, err: ashpd::zbus::Error| crate::Error::ReaderFailed {
            reader: "gnome-watcher",
            source: format!("{step} failed: {err}").into(),
        };

        // Create a blocking D-Bus session connection.
        let conn = match ashpd::zbus::blocking::Connection::session() {
            Ok(c) => c,
            Err(err) => {
                let _ = ready_tx.send(Err(setup_failed("connecting to the session bus", err)));
                return;
            }
        };

        // Create a proxy to the XDG Desktop Portal Settings interface.
        let proxy = match ashpd::zbus::blocking::Proxy::new(
            &conn,
            "org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
            "org.freedesktop.portal.Settings",
        ) {
            Ok(p) => p,
            Err(err) => {
                let _ = ready_tx.send(Err(setup_failed("creating the portal Settings proxy", err)));
                return;
            }
        };

        // Subscribe to every SettingChanged: the watched settings span four
        // namespaces, and `is_watched` picks them.
        let signals = match proxy.receive_signal("SettingChanged") {
            Ok(s) => s,
            Err(err) => {
                let _ = ready_tx.send(Err(setup_failed("subscribing to SettingChanged", err)));
                return;
            }
        };

        if ready_tx.send(Ok(conn.clone())).is_err() {
            return;
        }

        let (changed_tx, changed_rx) = mpsc::channel::<()>();
        // Ends when the connection is closed from `ThemeSubscription::drop`:
        // the closed socket ends zbus's message stream, and the iterator
        // returns `None`.
        let signal_thread = std::thread::spawn(move || {
            for signal in signals {
                if is_watched(&signal) && changed_tx.send(()).is_err() {
                    break;
                }
            }
        });
        // Returns when the signal thread ends or the subscription is dropped.
        super::fire_debounced(&changed_rx, &shutdown_rx, |()| true, &callback);
        drop(changed_rx);
        let _ = signal_thread.join();
    });

    // Receive the connection, or the setup error, from the watcher thread.
    let conn = match ready_rx.recv() {
        Ok(Ok(conn)) => conn,
        Ok(Err(err)) => {
            let _ = thread.join();
            return Err(err);
        }
        Err(_) => {
            let _ = thread.join();
            return Err(crate::Error::ReaderFailed {
                reader: "gnome-watcher",
                source: "failed to start GNOME theme watcher thread".into(),
            });
        }
    };

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
#[allow(clippy::unwrap_used)]
mod tests {
    use std::sync::mpsc;
    use std::time::Duration;

    use ashpd::zbus::zvariant::Value;

    /// A `SettingChanged` signal for `namespace` `key`, as the portal sends it.
    fn setting_changed(namespace: &str, key: &str, value: Value<'_>) -> ashpd::zbus::Message {
        ashpd::zbus::Message::signal(
            "/org/freedesktop/portal/desktop",
            "org.freedesktop.portal.Settings",
            "SettingChanged",
        )
        .unwrap()
        .build(&(namespace, key, value))
        .unwrap()
    }

    /// The watcher picks a watched setting out of every `SettingChanged`,
    /// in a gsettings namespace as in `org.freedesktop.appearance`, and
    /// ignores the rest.
    #[test]
    fn a_watched_setting_is_picked_and_the_rest_ignored() {
        let watched = [
            (
                "org.gnome.desktop.interface",
                "font-name",
                Value::from("Cantarell 11"),
            ),
            (
                "org.gnome.desktop.a11y.interface",
                "high-contrast",
                Value::from(true),
            ),
            (
                "org.freedesktop.appearance",
                "color-scheme",
                Value::from(1u32),
            ),
        ];
        for (namespace, key, value) in watched {
            assert!(
                super::is_watched(&setting_changed(namespace, key, value)),
                "{namespace} {key} is not picked"
            );
        }
        let ignored = [
            (
                "org.gnome.desktop.interface",
                "clock-format",
                Value::from("24h"),
            ),
            (
                "org.gnome.desktop.sound",
                "theme-name",
                Value::from("freedesktop"),
            ),
        ];
        for (namespace, key, value) in ignored {
            assert!(
                !super::is_watched(&setting_changed(namespace, key, value)),
                "{namespace} {key} is picked"
            );
        }
    }

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
