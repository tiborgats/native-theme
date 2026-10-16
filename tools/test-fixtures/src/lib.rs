//! Freedesktop icon-theme fixtures for the workspace's tests (dev-only, never
//! published): a dev-dependency of native-theme and the connectors, so their
//! tests and the showcases' share one fixture.
//!
//! native-theme reads the icon dirs once per process, so a test that needs
//! an icon theme of its own runs itself again in a child process ([`run`])
//! whose `HOME`, `XDG_DATA_HOME` and `XDG_DATA_DIRS` all lie in a temp dir
//! that holds the theme, so no system dir is an icon base dir.
//! freedesktop-icons still tries its fixed `/usr/share/pixmaps` after a miss
//! in the theme (freedesktop-icons 0.4.0 `src/lib.rs:345-347`): a read-only
//! look whose hit native-theme discards, as it keeps only a file inside the
//! theme's own directory, so it cannot change the result. The child's config
//! dirs lie in the temp dir too, its desktop is the one the test names, and
//! where the test names an icon theme its `kdeglobals` names it, so on KDE
//! the system's icon theme is the fixture's.
//!
//! ```no_run
//! use native_theme_test_fixtures::{Child, in_child, run, write_scalable_theme};
//!
//! const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"/>"#;
//!
//! // A `#[test]` at `module::tests::loads_from_the_fixture`.
//! fn loads_from_the_fixture() -> std::io::Result<()> {
//!     const NAME: &str = "module::tests::loads_from_the_fixture";
//!     if in_child() {
//!         // The test itself, against the fixture's icon dirs.
//!         return Ok(());
//!     }
//!     let child = Child { desktop: "KDE", icon_theme: Some("fixture") };
//!     run(NAME, child, |icons| write_scalable_theme(icons, "fixture", &[("folder", SVG)]))
//! }
//! ```

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

/// Names the fixture's root in the child process [`run`] starts.
const ROOT: &str = "NATIVE_THEME_TEST_FIXTURE";

/// Whether this process is the child [`run`] starts, the one whose icon dirs
/// hold the fixture.
#[must_use]
pub fn in_child() -> bool {
    std::env::var_os(ROOT).is_some()
}

/// The fixture's root, in the child process [`run`] starts.
#[must_use]
pub fn root() -> Option<PathBuf> {
    std::env::var_os(ROOT).map(PathBuf::from)
}

/// The icons dir that holds the fixture, in the child process [`run`]
/// starts: the one `write` was given.
#[must_use]
pub fn icons_dir() -> Option<PathBuf> {
    root().map(|root| root.join("share/icons"))
}

/// The `kdeglobals` that names the system's icon theme, in the child process
/// [`run`] starts (written where [`Child::icon_theme`] is `Some`).
#[must_use]
pub fn kdeglobals() -> Option<PathBuf> {
    root().map(|root| root.join("home/.config/kdeglobals"))
}

/// The child process [`run`] starts.
#[derive(Clone, Copy, Debug)]
pub struct Child<'a> {
    /// Its `XDG_CURRENT_DESKTOP`.
    pub desktop: &'a str,
    /// The icon theme its `kdeglobals` names (`[Icons] Theme`), or `None`
    /// for no `kdeglobals`.
    pub icon_theme: Option<&'a str>,
}

impl<'a> Child<'a> {
    /// A child on KDE whose `kdeglobals` names `icon_theme`, so the system's
    /// icon theme is `icon_theme`.
    #[must_use]
    pub const fn kde(icon_theme: &'a str) -> Self {
        Self {
            desktop: "KDE",
            icon_theme: Some(icon_theme),
        }
    }
}

/// A child process that did not pass its one test: its output.
struct ChildFailed(String);

impl fmt::Debug for ChildFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for ChildFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ChildFailed {}

/// Runs the test `test_path` (its path from the crate root) again in a
/// `child` process whose icon dirs hold only the icon themes `write` puts in
/// the icons dir it is given. The fixture's temp dir is named by the process
/// id, the test path and the desktop, and is removed however the run ends.
///
/// # Errors
///
/// An error from writing the fixture or starting the child, or, where the
/// child did not pass exactly its one test, one whose message is the child's
/// output.
pub fn run(
    test_path: &str,
    child: Child<'_>,
    write: impl FnOnce(&Path) -> io::Result<()>,
) -> io::Result<()> {
    /// Removes the fixture's directory however the test ends.
    struct RemoveOnDrop(PathBuf);
    impl Drop for RemoveOnDrop {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let guard = RemoveOnDrop(std::env::temp_dir().join(format!(
        "native-theme-test-fixture-{}-{}-{}",
        std::process::id(),
        test_path.replace("::", "-"),
        child.desktop
    )));
    let root = &guard.0;
    let _ = std::fs::remove_dir_all(root);
    let icons = root.join("share/icons");
    let config = root.join("home/.config");
    std::fs::create_dir_all(&icons)?;
    std::fs::create_dir_all(&config)?;
    if let Some(theme) = child.icon_theme {
        std::fs::write(
            config.join("kdeglobals"),
            format!("[Icons]\nTheme={theme}\n"),
        )?;
    }
    write(&icons)?;
    let output = std::process::Command::new(std::env::current_exe()?)
        .args(["--exact", test_path, "--nocapture", "--test-threads=1"])
        .env(ROOT, root)
        .env("HOME", root.join("home"))
        .env("XDG_DATA_HOME", root.join("home/.local/share"))
        .env("XDG_DATA_DIRS", root.join("share"))
        .env("XDG_CONFIG_HOME", &config)
        .env("XDG_CONFIG_DIRS", root.join("etc/xdg"))
        .env("XDG_CURRENT_DESKTOP", child.desktop)
        .output()?;
    drop(guard);
    let stdout = String::from_utf8_lossy(&output.stdout);
    if output.status.success() && stdout.contains("1 passed") {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(io::Error::other(ChildFailed(format!(
        "{test_path} did not pass in the fixture's child process ({}):\n{stdout}{stderr}",
        output.status
    ))))
}

/// Writes the theme `name` into `icons`, its one scalable directory
/// (`scalable`, from 8 to 512) holding each `(icon, svg)` as
/// `scalable/<icon>.svg`.
///
/// # Errors
///
/// An error from writing a file.
pub fn write_scalable_theme(icons: &Path, name: &str, files: &[(&str, &str)]) -> io::Result<()> {
    let theme = icons.join(name);
    std::fs::create_dir_all(theme.join("scalable"))?;
    std::fs::write(
        theme.join("index.theme"),
        format!(
            "[Icon Theme]\nName={name}\nDirectories=scalable\n\n\
             [scalable]\nSize=16\nType=Scalable\nMinSize=8\nMaxSize=512\n"
        ),
    )?;
    for (icon, svg) in files {
        std::fs::write(theme.join(format!("scalable/{icon}.svg")), svg)?;
    }
    Ok(())
}

/// Writes the theme `name` into `icons` with one fixed-size directory per
/// size of `files`, each `(size, file, bytes)` as `<size>/<file>` in the
/// directory `<size>` (`Size=<size>`, `Type=Fixed`), so a lookup takes the
/// directory nearest the size it is made at.
///
/// # Errors
///
/// An error from writing a file.
pub fn write_fixed_theme(icons: &Path, name: &str, files: &[(u32, &str, &[u8])]) -> io::Result<()> {
    let theme = icons.join(name);
    let mut sizes: Vec<u32> = files.iter().map(|&(size, ..)| size).collect();
    sizes.sort_unstable();
    sizes.dedup();
    let directories: Vec<String> = sizes.iter().map(u32::to_string).collect();
    let mut index = format!(
        "[Icon Theme]\nName={name}\nDirectories={}\n",
        directories.join(",")
    );
    for size in &sizes {
        index.push_str(&format!("\n[{size}]\nSize={size}\nType=Fixed\n"));
    }
    std::fs::create_dir_all(&theme)?;
    std::fs::write(theme.join("index.theme"), index)?;
    for (size, file, bytes) in files {
        let dir = theme.join(size.to_string());
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join(file), bytes)?;
    }
    Ok(())
}

/// Writes the theme `name` into `icons` with `svg` as its loading
/// indicator, `animations/22/process-working.svg` in a directory declared as
/// Breeze's is (`/usr/share/icons/breeze/index.theme`, `[animations/22]`).
///
/// # Errors
///
/// An error from writing a file.
pub fn write_indicator_theme(icons: &Path, name: &str, svg: &str) -> io::Result<()> {
    let theme = icons.join(name);
    std::fs::create_dir_all(theme.join("animations/22"))?;
    std::fs::write(
        theme.join("index.theme"),
        format!(
            "[Icon Theme]\nName={name}\nDirectories=animations/22\n\n\
             [animations/22]\nSize=22\nContext=Animations\nType=Scalable\nMinSize=22\nMaxSize=256\n"
        ),
    )?;
    std::fs::write(theme.join("animations/22/process-working.svg"), svg)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"/>"#;

    /// The child sees the fixture [`run`] wrote: its root, the theme in the
    /// icons dir, the `kdeglobals` naming the icon theme, the desktop and
    /// the data and config dirs inside the root.
    #[test]
    fn the_child_runs_in_the_fixture() -> Result<(), Box<dyn std::error::Error>> {
        const NAME: &str = "tests::the_child_runs_in_the_fixture";
        if let Some(root) = root() {
            let icons = icons_dir().ok_or("no icons dir")?;
            assert_eq!(
                std::fs::read_to_string(icons.join("fixture/scalable/folder.svg"))?,
                SVG
            );
            assert!(icons.join("fixture/index.theme").is_file());
            let kdeglobals = kdeglobals().ok_or("no kdeglobals")?;
            assert_eq!(
                std::fs::read_to_string(kdeglobals)?,
                "[Icons]\nTheme=fixture\n"
            );
            assert_eq!(std::env::var("XDG_CURRENT_DESKTOP")?, "KDE");
            assert_eq!(
                std::env::var_os("XDG_DATA_DIRS").map(PathBuf::from),
                Some(root.join("share"))
            );
            assert_eq!(
                std::env::var_os("HOME").map(PathBuf::from),
                Some(root.join("home"))
            );
            return Ok(());
        }
        assert!(!in_child());
        let child = Child {
            desktop: "KDE",
            icon_theme: Some("fixture"),
        };
        run(NAME, child, |icons| {
            write_scalable_theme(icons, "fixture", &[("folder", SVG)])?;
            write_indicator_theme(icons, "indicator", SVG)
        })?;
        Ok(())
    }

    /// A fixed theme lists each size once, as a fixed directory holding its
    /// files.
    #[test]
    fn a_fixed_theme_has_one_directory_per_size() -> Result<(), Box<dyn std::error::Error>> {
        let icons = std::env::temp_dir().join(format!(
            "native-theme-test-fixture-fixed-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&icons);
        let written = write_fixed_theme(
            &icons,
            "fixed",
            &[
                (48, "folder.svg", SVG.as_bytes()),
                (16, "folder.svg", SVG.as_bytes()),
                (16, "edit-copy.svg", SVG.as_bytes()),
            ],
        );
        let index = std::fs::read_to_string(icons.join("fixed/index.theme"));
        let files = ["16/folder.svg", "16/edit-copy.svg", "48/folder.svg"]
            .map(|file| icons.join("fixed").join(file).is_file());
        let _ = std::fs::remove_dir_all(&icons);
        written?;
        assert_eq!(
            index?,
            "[Icon Theme]\nName=fixed\nDirectories=16,48\n\n\
             [16]\nSize=16\nType=Fixed\n\n[48]\nSize=48\nType=Fixed\n"
        );
        assert_eq!(files, [true; 3]);
        Ok(())
    }

    /// Without an icon theme the child has no `kdeglobals`.
    #[test]
    fn no_icon_theme_writes_no_kdeglobals() -> Result<(), Box<dyn std::error::Error>> {
        const NAME: &str = "tests::no_icon_theme_writes_no_kdeglobals";
        if in_child() {
            assert!(!kdeglobals().ok_or("no kdeglobals path")?.exists());
            assert_eq!(std::env::var("XDG_CURRENT_DESKTOP")?, "sway");
            return Ok(());
        }
        let child = Child {
            desktop: "sway",
            icon_theme: None,
        };
        run(NAME, child, |_| Ok(()))?;
        Ok(())
    }

    /// A child that passes no test is an error naming the test, and the
    /// fixture's directory is gone afterwards.
    #[test]
    fn a_child_that_passes_no_test_is_an_error() {
        const NAME: &str = "tests::no_such_test";
        let mut written = None;
        let child = Child {
            desktop: "KDE",
            icon_theme: None,
        };
        let ran = run(NAME, child, |icons| {
            written = Some(icons.to_path_buf());
            Ok(())
        });
        let message = ran.map_err(|e| e.to_string());
        assert!(
            matches!(&message, Err(m) if m.starts_with(NAME)),
            "{message:?}"
        );
        assert!(written.is_some_and(|icons| !icons.exists()));
    }
}
