//! KDE theme change watcher using inotify on the config directory.

use std::sync::mpsc;
use std::time::{Duration, Instant};

use notify::Watcher;

use super::ThemeChangeEvent;

/// Watch the user's config directory for KDE theme config file changes.
///
/// Watches the config directory ([`crate::detect::xdg_config_home`],
/// typically `~/.config/`), where `kdeglobals` lives, in `NonRecursive`
/// mode, filtering events to the `kdeglobals` and `kcmfontsrc` filenames.
pub(crate) fn watch_kde(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeSubscription> {
    let config_dir =
        crate::detect::xdg_config_home().ok_or_else(|| crate::Error::ReaderFailed {
            reader: "kde-watcher",
            source: crate::detect::CONFIG_HOME_UNRESOLVED.into(),
        })?;
    watch_kde_dir(config_dir, callback)
}

/// Watch `config_dir` as [`watch_kde`] watches the config directory.
///
/// The inotify watch is set up on the calling thread, so a failure to set
/// it up is returned. The callback fires once per burst of relevant events:
/// 300ms after the last of them. The debounce window spans KDE's
/// multi-write config pattern (`QSaveFile` atomic writes), so the callback
/// runs after the last write has landed. The background thread checks the
/// shutdown channel on every iteration, at least every 200ms.
fn watch_kde_dir(
    config_dir: std::path::PathBuf,
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeSubscription> {
    let setup_failed = |step: &str, err: notify::Error| crate::Error::ReaderFailed {
        reader: "kde-watcher",
        source: format!("{step} failed for {}: {err}", config_dir.display()).into(),
    };

    let (tx, rx) = mpsc::channel();
    let mut watcher =
        notify::recommended_watcher(tx).map_err(|err| setup_failed("creating the watcher", err))?;
    watcher
        .watch(&config_dir, notify::RecursiveMode::NonRecursive)
        .map_err(|err| setup_failed("watching the config directory", err))?;

    let watched_names: Vec<std::ffi::OsString> = ["kdeglobals", "kcmfontsrc"]
        .iter()
        .map(std::ffi::OsString::from)
        .collect();

    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();

    let thread = std::thread::spawn(move || {
        // The watcher lives as long as the thread: dropping it would close
        // the notify channel and stop delivering events.
        let _watcher = watcher;

        let debounce = Duration::from_millis(300);
        let poll = Duration::from_millis(200);
        // When the last relevant event not yet fired arrived.
        let mut pending: Option<Instant> = None;

        loop {
            match shutdown_rx.try_recv() {
                Ok(()) | Err(mpsc::TryRecvError::Disconnected) => break,
                Err(mpsc::TryRecvError::Empty) => {}
            }

            let wait = pending.map_or(poll, |last| {
                debounce.saturating_sub(last.elapsed()).min(poll)
            });
            match rx.recv_timeout(wait) {
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
                    let is_relevant = event.paths.iter().any(|p| {
                        p.file_name()
                            .is_some_and(|n| watched_names.iter().any(|w| *w == n))
                    });
                    if is_mutation && is_relevant {
                        pending = Some(Instant::now());
                    }
                }
                Ok(Err(_)) => {
                    // notify error -- ignore
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    // notify watcher was dropped -- exit.
                    break;
                }
            }

            if pending.is_some_and(|last| last.elapsed() >= debounce) {
                pending = None;
                super::fire(&callback);
            }
        }
    });

    Ok(super::ThemeSubscription::new(shutdown_tx, thread, None))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, mpsc};
    use std::time::{Duration, Instant};

    /// How long a test waits for an event or a drop before it fails.
    const TIMEOUT: Duration = Duration::from_secs(5);

    struct TempConfigDir(std::path::PathBuf);

    impl Drop for TempConfigDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn temp_config_dir(tag: &str) -> TempConfigDir {
        let dir = std::env::temp_dir().join(format!(
            "native-theme-kde-watcher-{}-{tag}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("kdeglobals"), "[General]\n").unwrap();
        TempConfigDir(dir)
    }

    fn write_kdeglobals(dir: &TempConfigDir, round: usize) {
        std::fs::write(
            dir.0.join("kdeglobals"),
            format!("[General]\nround={round}\n"),
        )
        .unwrap();
    }

    /// Drop `subscription` on a helper thread and wait up to [`TIMEOUT`] for
    /// the drop to return.
    fn drop_returns(subscription: super::super::ThemeSubscription) -> bool {
        let (done_tx, done_rx) = mpsc::channel();
        std::thread::spawn(move || {
            drop(subscription);
            let _ = done_tx.send(());
        });
        done_rx.recv_timeout(TIMEOUT).is_ok()
    }

    /// A directory the watcher cannot watch is an error, not a subscription
    /// that never fires (M4-32).
    #[test]
    fn an_unwatchable_directory_is_an_error() {
        let dir = temp_config_dir("unwatchable");
        let missing = dir.0.join("missing");
        let result = super::watch_kde_dir(missing, |_| {});
        assert!(
            matches!(
                &result,
                Err(crate::Error::ReaderFailed {
                    reader: "kde-watcher",
                    ..
                })
            ),
            "expected ReaderFailed from kde-watcher, got: {result:?}"
        );
    }

    /// Writes 50 ms apart, closer than the debounce window, give one event,
    /// after the last write (`docs/todo.md`, the KDE throttle entry).
    #[test]
    fn a_burst_of_writes_fires_once_after_the_last() {
        let dir = temp_config_dir("burst");
        let (event_tx, event_rx) = mpsc::channel();
        let subscription = super::watch_kde_dir(dir.0.clone(), move |_| {
            let _ = event_tx.send(Instant::now());
        })
        .unwrap();

        let mut last_write = Instant::now();
        for round in 0..6 {
            if round > 0 {
                std::thread::sleep(Duration::from_millis(50));
            }
            write_kdeglobals(&dir, round);
            last_write = Instant::now();
        }

        let fired = event_rx
            .recv_timeout(TIMEOUT)
            .expect("no event after the burst");
        assert!(
            fired >= last_write,
            "the event fired {:?} before the last write",
            last_write.duration_since(fired)
        );
        assert!(
            event_rx.recv_timeout(Duration::from_secs(1)).is_err(),
            "the burst fired more than one event"
        );
        assert!(drop_returns(subscription));
    }

    /// Dropping the subscription while writes keep arriving returns within
    /// the timeout (M4-34).
    #[test]
    fn dropping_during_a_burst_returns() {
        let dir = temp_config_dir("drop-burst");
        let subscription = super::watch_kde_dir(dir.0.clone(), |_| {}).unwrap();

        let stop = Arc::new(AtomicBool::new(false));
        let writer = {
            let stop = Arc::clone(&stop);
            let path = dir.0.clone();
            std::thread::spawn(move || {
                let mut round = 0usize;
                while !stop.load(Ordering::SeqCst) {
                    let _ = std::fs::write(path.join("kdeglobals"), format!("round={round}\n"));
                    round += 1;
                    std::thread::sleep(Duration::from_millis(50));
                }
            })
        };

        std::thread::sleep(Duration::from_millis(500));
        let returned = drop_returns(subscription);
        stop.store(true, Ordering::SeqCst);
        writer.join().unwrap();
        assert!(
            returned,
            "dropping the KDE subscription during a burst did not return within {TIMEOUT:?}"
        );
    }

    /// A callback that panics does not end the watcher: the next change still
    /// fires (M4-33).
    #[test]
    fn a_panicking_callback_does_not_stop_the_watcher() {
        let dir = temp_config_dir("panic");
        let calls = Arc::new(AtomicUsize::new(0));
        let (event_tx, event_rx) = mpsc::channel();
        let subscription = {
            let calls = Arc::clone(&calls);
            super::watch_kde_dir(dir.0.clone(), move |_| {
                let call = calls.fetch_add(1, Ordering::SeqCst);
                let _ = event_tx.send(call);
                assert!(call > 0, "the first callback panics");
            })
            .unwrap()
        };

        write_kdeglobals(&dir, 0);
        assert_eq!(event_rx.recv_timeout(TIMEOUT).unwrap(), 0);
        std::thread::sleep(Duration::from_millis(500));
        write_kdeglobals(&dir, 1);
        assert_eq!(
            event_rx.recv_timeout(TIMEOUT).ok(),
            Some(1),
            "no event after the callback panicked"
        );
        assert!(drop_returns(subscription));
    }
}
