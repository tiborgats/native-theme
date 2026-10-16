//! A freedesktop icon theme in a temp dir, for a test that loads from one.
//!
//! Test-only. The icon dirs are read once per process, so a test that needs a
//! theme of its own runs itself again in a child process whose `HOME`,
//! `XDG_DATA_HOME` and `XDG_DATA_DIRS` all lie in a temp dir that holds the
//! theme, so no system dir is an icon base dir. freedesktop-icons still tries
//! its fixed `/usr/share/pixmaps` after a miss in the theme (freedesktop-icons
//! 0.4.0 `src/lib.rs:345-347`): a read-only look whose hit core discards, as
//! it keeps only a file inside the theme's own directory, so it cannot change
//! the result. The child's desktop is KDE, its config dirs lie in the temp dir
//! too, and its `kdeglobals` names the fixture's theme, so the system's icon
//! theme is the fixture's. The iced and gpui connectors' tests have the same
//! module.

#![cfg(all(target_os = "linux", feature = "system-icons"))]

use std::path::{Path, PathBuf};

/// Names the fixture's root in the child process [`run`] starts.
const ROOT: &str = "NATIVE_THEME_EGUI_ICON_FIXTURE";

/// Whether this process is the child [`run`] starts, the one whose icon dirs
/// hold the fixture.
pub(crate) fn in_child() -> bool {
    std::env::var_os(ROOT).is_some()
}

/// Runs the test `name` (its path from the crate root) again in a child
/// process whose system icon theme is `theme`, an icon theme `write` puts in
/// the icons dir it is given, and asserts that the test passed there.
pub(crate) fn run(
    name: &str,
    theme: &str,
    write: impl FnOnce(&Path) -> std::io::Result<()>,
) -> std::io::Result<()> {
    /// Removes the fixture's directory however the test ends.
    struct RemoveOnDrop(PathBuf);
    impl Drop for RemoveOnDrop {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let guard = RemoveOnDrop(std::env::temp_dir().join(format!(
        "native-theme-egui-icon-fixture-{}-{}",
        std::process::id(),
        name.replace("::", "-")
    )));
    let root = &guard.0;
    let _ = std::fs::remove_dir_all(root);
    let icons = root.join("share/icons");
    let config = root.join("home/.config");
    std::fs::create_dir_all(&icons)?;
    std::fs::create_dir_all(&config)?;
    std::fs::write(
        config.join("kdeglobals"),
        format!("[Icons]\nTheme={theme}\n"),
    )?;
    write(&icons)?;
    let output = std::process::Command::new(std::env::current_exe()?)
        .args(["--exact", name, "--nocapture", "--test-threads=1"])
        .env(ROOT, root)
        .env("HOME", root.join("home"))
        .env("XDG_DATA_HOME", root.join("home/.local/share"))
        .env("XDG_DATA_DIRS", root.join("share"))
        .env("XDG_CONFIG_HOME", &config)
        .env("XDG_CONFIG_DIRS", root.join("etc/xdg"))
        .env("XDG_CURRENT_DESKTOP", "KDE")
        .output()?;
    drop(guard);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success() && stdout.contains("1 passed"),
        "{stdout}{stderr}"
    );
    Ok(())
}
