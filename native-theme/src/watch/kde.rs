//! KDE theme change watcher using inotify on the config directory.

use std::sync::mpsc;

use notify::Watcher;

use super::ThemeChangeEvent;

/// The files the watcher observes, each a path in the config directory: the
/// KDE reader's sources that the user's settings write (`kde/mod.rs`,
/// "Sources").
pub(super) const WATCHED_FILES: &[&str] = &["kdeglobals", "kcmfonts", "kdedefaults/kdeglobals"];

/// The subdirectory of the config directory that holds Plasma's defaults.
const KDEDEFAULTS: &str = "kdedefaults";

/// Watch the user's config directory for KDE theme config file changes.
///
/// Watches the config directory ([`crate::detect::xdg_config_home`],
/// typically `~/.config/`), where `kdeglobals` lives, and its `kdedefaults`
/// subdirectory, each in `NonRecursive` mode, filtering events to the paths
/// of [`WATCHED_FILES`].
pub(crate) fn watch_kde(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeSubscription> {
    let config_dir =
        crate::detect::xdg_config_home().ok_or_else(|| crate::Error::ReaderFailed {
            reader: crate::error::reader_names::KDE_WATCHER,
            source: crate::detect::CONFIG_HOME_UNRESOLVED.into(),
        })?;
    watch_kde_dir(config_dir, callback)
}

/// Watch `config_dir` as [`watch_kde`] watches the config directory.
///
/// The inotify watch of `config_dir` is set up on the calling thread, so a
/// failure to set it up is returned. `kdedefaults` is watched where it
/// exists, and from its creation where it does not; its creation counts as
/// a change. The callback fires once per burst of relevant events, 300ms
/// after the last of them ([`super::fire_debounced`]). The debounce window
/// spans KDE's multi-write config pattern (`QSaveFile` atomic writes), so
/// the callback runs after the last write has landed.
fn watch_kde_dir(
    config_dir: std::path::PathBuf,
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeSubscription> {
    let setup_failed = |step: &str, err: notify::Error| crate::Error::ReaderFailed {
        reader: crate::error::reader_names::KDE_WATCHER,
        source: format!("{step} failed for {}: {err}", config_dir.display()).into(),
    };

    let (tx, rx) = mpsc::channel();
    let mut watcher =
        notify::recommended_watcher(tx).map_err(|err| setup_failed("creating the watcher", err))?;
    watcher
        .watch(&config_dir, notify::RecursiveMode::NonRecursive)
        .map_err(|err| setup_failed("watching the config directory", err))?;
    let defaults_dir = config_dir.join(KDEDEFAULTS);
    // Absent until Plasma creates it: then its creation event adds the watch.
    let _ = watcher.watch(&defaults_dir, notify::RecursiveMode::NonRecursive);

    let watched_paths: Vec<std::path::PathBuf> = WATCHED_FILES
        .iter()
        .map(|file| config_dir.join(file))
        .collect();

    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();

    let thread = std::thread::spawn(move || {
        let is_relevant = |event: notify::Result<notify::Event>| {
            // A notify error is ignored.
            let Ok(event) = event else {
                return false;
            };
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
                return false;
            }
            let defaults_created = matches!(event.kind, notify::EventKind::Create(_))
                && event.paths.contains(&defaults_dir);
            if defaults_created {
                let _ = watcher.watch(&defaults_dir, notify::RecursiveMode::NonRecursive);
            }
            defaults_created || event.paths.iter().any(|path| watched_paths.contains(path))
        };
        // The thread owns the watcher while the loop runs: dropping it
        // would close the notify channel and stop delivering events.
        super::fire_debounced(&rx, &shutdown_rx, is_relevant, &callback);
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
                    reader: crate::error::reader_names::KDE_WATCHER,
                    ..
                })
            ),
            "expected ReaderFailed from kde-watcher, got: {result:?}"
        );
    }

    /// Writes 50 ms apart, closer than the debounce window, fire the
    /// debounce window after the last write, not only after the first
    /// (`docs/archive/todo_closed.md`, *Closed items from Theme watcher:
    /// OS changes it does not report*, "KDE: the 300 ms throttle keeps the first event of a
    /// burst"). A loaded machine can stall a write past the window and split
    /// the burst, so that a burst fires exactly once is checked on made-up
    /// instants (`watch::tests::a_burst_is_due_once_after_its_last_message`).
    #[test]
    fn a_burst_of_writes_fires_after_the_last() {
        let dir = temp_config_dir("burst");
        let (event_tx, event_rx) = mpsc::channel();
        let subscription = super::watch_kde_dir(dir.0.clone(), move |_| {
            let _ = event_tx.send(Instant::now());
        })
        .unwrap();

        let mut before_last_write = Instant::now();
        for round in 0..6 {
            if round > 0 {
                std::thread::sleep(Duration::from_millis(50));
            }
            before_last_write = Instant::now();
            write_kdeglobals(&dir, round);
        }

        // The last write's events arrive after `before_last_write`, and
        // their burst fires the debounce window after that.
        let due = before_last_write + super::super::DEBOUNCE;
        let deadline = Instant::now() + TIMEOUT;
        loop {
            let fired = event_rx
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .expect("no event the debounce window after the last write");
            if fired >= due {
                break;
            }
        }
        assert!(drop_returns(subscription));
    }

    /// A write to `kdedefaults/kdeglobals`, which the KDE reader reads
    /// beneath the user's file, fires (`docs/archive/todo_closed.md`, *Closed items from Theme
    /// watcher: OS changes it does not report*, "`watch::on_theme_change()` fires on fewer
    /// changes").
    #[test]
    fn a_write_to_kdedefaults_fires() {
        let dir = temp_config_dir("kdedefaults");
        std::fs::create_dir_all(dir.0.join("kdedefaults")).unwrap();
        let (event_tx, event_rx) = mpsc::channel();
        let subscription = super::watch_kde_dir(dir.0.clone(), move |_| {
            let _ = event_tx.send(());
        })
        .unwrap();

        std::fs::write(
            dir.0.join("kdedefaults/kdeglobals"),
            "[Icons]\nTheme=breeze\n",
        )
        .unwrap();
        assert!(
            event_rx.recv_timeout(TIMEOUT).is_ok(),
            "no event after kdedefaults/kdeglobals changed"
        );
        assert!(drop_returns(subscription));
    }

    /// A `kdedefaults` directory created after the watcher started is
    /// watched too: its creation fires, and so does a later write to its
    /// `kdeglobals`.
    #[test]
    fn a_kdedefaults_created_later_is_watched() {
        let dir = temp_config_dir("kdedefaults-later");
        let (event_tx, event_rx) = mpsc::channel();
        let subscription = super::watch_kde_dir(dir.0.clone(), move |_| {
            let _ = event_tx.send(());
        })
        .unwrap();

        std::fs::create_dir_all(dir.0.join("kdedefaults")).unwrap();
        assert!(
            event_rx.recv_timeout(TIMEOUT).is_ok(),
            "no event after kdedefaults was created"
        );
        std::fs::write(
            dir.0.join("kdedefaults/kdeglobals"),
            "[Icons]\nTheme=breeze\n",
        )
        .unwrap();
        assert!(
            event_rx.recv_timeout(TIMEOUT).is_ok(),
            "no event after kdedefaults/kdeglobals changed"
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

    /// A callback that drops its own subscription: the drop returns instead
    /// of joining the watcher thread from itself, and the thread exits once
    /// the callback returns (M4-89).
    #[test]
    fn a_callback_that_drops_its_subscription_returns_and_the_thread_exits() {
        /// Sends on `0` when dropped: the watcher thread owns the callback,
        /// so this drops when the thread exits.
        struct OnDrop(mpsc::Sender<()>);

        impl Drop for OnDrop {
            fn drop(&mut self) {
                let _ = self.0.send(());
            }
        }

        let dir = temp_config_dir("drop-in-callback");
        let slot: Arc<std::sync::Mutex<Option<super::super::ThemeSubscription>>> =
            Arc::new(std::sync::Mutex::new(None));
        let (dropped_tx, dropped_rx) = mpsc::channel();
        let (exited_tx, exited_rx) = mpsc::channel();
        let subscription = {
            let slot = Arc::clone(&slot);
            let exited = OnDrop(exited_tx);
            super::watch_kde_dir(dir.0.clone(), move |_| {
                let _ = &exited;
                let subscription = slot.lock().unwrap().take();
                drop(subscription);
                let _ = dropped_tx.send(());
            })
            .unwrap()
        };
        *slot.lock().unwrap() = Some(subscription);

        write_kdeglobals(&dir, 0);
        assert!(
            dropped_rx.recv_timeout(TIMEOUT).is_ok(),
            "dropping the subscription from its own callback did not return"
        );
        assert!(
            exited_rx.recv_timeout(TIMEOUT).is_ok(),
            "the watcher thread did not exit after its callback dropped the subscription"
        );
    }
}
