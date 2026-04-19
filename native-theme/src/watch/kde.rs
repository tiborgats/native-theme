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
) -> crate::Result<super::ThemeSubscription> {
    let kdeglobals = crate::kde::kdeglobals_path();
    let config_dir = kdeglobals
        .parent()
        .ok_or_else(|| crate::Error::ReaderFailed {
            reader: "kde_watcher",
            source: "no parent directory for kdeglobals path".into(),
        })?
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

        // `None` means "never fired" — the first relevant event always
        // fires immediately, regardless of watcher age.
        let mut last_fire: Option<Instant> = None;
        let debounce = Duration::from_millis(300);

        loop {
            match rx.recv_timeout(Duration::from_millis(200)) {
                Ok(Ok(event)) => {
                    // Only consider mutations: KDE's QSaveFile pattern produces
                    // Modify/Create/Remove events on rename. Access events
                    // (file Open/Read/Close) fire whenever any process — KDE
                    // Plasma, our own freedesktop_icons lookup, the user — just
                    // *reads* the file. Treating reads as "theme changed" causes
                    // a feedback loop: we read kdeglobals, the watcher fires,
                    // the consumer reloads its theme (re-reading kdeglobals),
                    // which fires the watcher again.
                    let is_mutation = matches!(
                        event.kind,
                        notify::EventKind::Modify(_)
                            | notify::EventKind::Create(_)
                            | notify::EventKind::Remove(_)
                    );
                    if !is_mutation {
                        continue;
                    }
                    let is_relevant = event.paths.iter().any(|p| {
                        p.file_name()
                            .is_some_and(|n| watched_names.iter().any(|w| *w == n))
                    });
                    if is_relevant && last_fire.is_none_or(|t| t.elapsed() >= debounce) {
                        last_fire = Some(Instant::now());
                        callback(ThemeChangeEvent::Changed);
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

    Ok(super::ThemeSubscription::new(shutdown_tx, thread, None))
}
