//! The golden snapshots of `kde-breeze` (plan, rule 2): the theme resolved
//! for each scheme, `tests/snapshots/kde-breeze.<scheme>.json`, and the theme
//! resolved from each `tests/fixtures/kde/*.ini` through the KDE reader,
//! `tests/snapshots/kde-breeze.<fixture>.json`, each written from
//! `ResolvedTheme`'s `Serialize` (feature `serde`; the fixtures' also need
//! feature `kde` on Linux). A snapshot that differs fails the test; with the
//! environment variable [`UPDATE`] set the test writes the snapshots instead,
//! and a changed value is a snapshot diff justified by its citation in the
//! same commit.

use std::path::{Path, PathBuf};

use crate::next::preset::{Preset, Theme};
use crate::next::query::ThemeContext;
use crate::next::widgets::ResolvedTheme;

/// A test's result.
type TestResult = Result<(), Box<dyn std::error::Error>>;

/// The environment variable that makes the test write the snapshots.
const UPDATE: &str = "NATIVE_THEME_UPDATE_SNAPSHOTS";

/// The folder of the snapshots.
fn snapshots() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/snapshots")
}

/// `theme` in its snapshot form: pretty JSON, a newline at the end.
fn snapshot_form(theme: &ResolvedTheme) -> Result<String, serde_json::Error> {
    let mut text = serde_json::to_string_pretty(theme)?;
    text.push('\n');
    Ok(text)
}

/// `theme` compared with the snapshot `name`, or written to it where
/// [`UPDATE`] is set; the name of a snapshot that differs is returned.
fn check(name: &str, theme: &ResolvedTheme) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let path = snapshots().join(format!("{name}.json"));
    let text = snapshot_form(theme)?;
    if std::env::var_os(UPDATE).is_some() {
        std::fs::create_dir_all(snapshots())?;
        std::fs::write(&path, text)?;
        return Ok(None);
    }
    let committed = std::fs::read_to_string(&path)
        .map_err(|e| format!("{}: {e} (set {UPDATE} to write it)", path.display()))?;
    Ok((committed != text).then(|| name.to_owned()))
}

/// Fails naming every snapshot in `differing`.
fn report(differing: &[String]) -> TestResult {
    if differing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "these snapshots differ from the resolved themes: {differing:?} (set {UPDATE} to \
             write them, and justify each changed value by its citation)"
        )
        .into())
    }
}

#[test]
fn each_scheme_s_theme_is_its_snapshot() -> TestResult {
    let mut differing = Vec::new();
    for (scheme, ctx) in [
        ("light", ThemeContext::light()),
        ("dark", ThemeContext::dark()),
    ] {
        let theme = Theme::preset(Preset::KdeBreeze).resolve(&ctx);
        let name = format!("{}.{scheme}", Preset::KdeBreeze.as_str());
        differing.extend(check(&name, &theme)?);
    }
    report(&differing)
}

#[cfg(all(target_os = "linux", feature = "kde"))]
#[test]
fn each_reader_fixture_s_theme_is_its_snapshot() -> TestResult {
    use crate::kde::{create_kde_parser, read_kconfig};
    use crate::next::reader::kde::snapshot_from_kdeglobals;

    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/kde");
    let mut fixtures = Vec::new();
    for entry in std::fs::read_dir(&dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e == "ini") {
            fixtures.push(path);
        }
    }
    fixtures.sort();
    // breeze-dark, breeze-light, custom-accent, high-dpi, malformed-values,
    // minimal, missing-groups.
    assert_eq!(
        fixtures.len(),
        7,
        "the .ini fixtures of tests/fixtures/kde/"
    );
    let mut differing = Vec::new();
    for path in &fixtures {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("a fixture's name")?;
        let mut ini = create_kde_parser();
        read_kconfig(&mut ini, &std::fs::read_to_string(path)?);
        let snapshot = snapshot_from_kdeglobals(&ini, None, None);
        let theme = Theme::preset(snapshot.preset)
            .with_system_overrides(snapshot.overrides)
            .resolve(&snapshot.context);
        let name = format!("{}.{stem}", Preset::KdeBreeze.as_str());
        differing.extend(check(&name, &theme)?);
    }
    report(&differing)
}
