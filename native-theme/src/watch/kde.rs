//! KDE theme change watcher using inotify on the config directory.

use std::sync::mpsc;
use std::time::{Duration, Instant};

use notify::Watcher;

use super::ThemeChangeEvent;

/// Spawn a background thread that watches for KDE theme config file changes.
///
/// Watches the parent directory of `kdeglobals` (typically `~/.config/`)
/// using `NonRecursive` mode, filtering events to `kdeglobals` and
/// `kcmfontsrc` filenames only. A 300ms debounce window prevents callback
/// floods from KDE's multi-write config pattern (`QSaveFile` atomic writes).
#[allow(dead_code)] // Dispatched from on_theme_change() in Phase 66 Plan 02
pub(crate) fn watch_kde(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeWatcher> {
    let kdeglobals = crate::kde::kdeglobals_path();
    let config_dir = kdeglobals
        .parent()
        .ok_or_else(|| crate::Error::Unavailable("no parent directory for kdeglobals path".into()))?
        .to_path_buf();

    let watched_names: &[&std::ffi::OsStr] = &[
        std::ffi::OsStr::new("kdeglobals"),
        std::ffi::OsStr::new("kcmfontsrc"),
    ];
    // Move owned copies into the thread closure.
    let watched_names: Vec<std::ffi::OsString> =
        watched_names.iter().map(|n| (*n).to_os_string()).collect();

    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();

    let thread = std::thread::spawn(move || {
        let (tx, rx) = mpsc::channel();

        // Create the inotify watcher. The `_watcher` binding keeps it alive
        // for the duration of the thread -- dropping it would close the
        // internal notify channel and stop delivering events.
        let mut _watcher = match notify::recommended_watcher(tx) {
            Ok(w) => w,
            Err(_) => return,
        };

        if _watcher
            .watch(&config_dir, notify::RecursiveMode::NonRecursive)
            .is_err()
        {
            return;
        }

        // Allow the first event to fire immediately by setting last_fire far
        // in the past.
        let mut last_fire = Instant::now() - Duration::from_secs(10);
        let debounce = Duration::from_millis(300);

        loop {
            match rx.recv_timeout(Duration::from_millis(200)) {
                Ok(Ok(event)) => {
                    let is_relevant = event.paths.iter().any(|p| {
                        p.file_name()
                            .is_some_and(|n| watched_names.iter().any(|w| *w == n))
                    });
                    if is_relevant && last_fire.elapsed() >= debounce {
                        last_fire = Instant::now();
                        callback(ThemeChangeEvent::ColorSchemeChanged);
                    }
                }
                Ok(Err(_)) => {
                    // notify error -- ignore
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // Check shutdown channel on timeout.
                    match shutdown_rx.try_recv() {
                        Ok(()) | Err(mpsc::TryRecvError::Disconnected) => break,
                        Err(mpsc::TryRecvError::Empty) => {}
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    // notify watcher was dropped -- exit.
                    break;
                }
            }
        }
    });

    Ok(super::ThemeWatcher::new(shutdown_tx, thread))
}
