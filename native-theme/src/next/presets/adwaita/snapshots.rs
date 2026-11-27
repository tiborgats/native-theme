//! The golden snapshots of `adwaita` (plan, rule 2): the theme resolved for
//! each scheme, `tests/snapshots/adwaita.<scheme>.json`, and the theme
//! resolved from each case of GNOME's reader parity (the data of 0.7.2's
//! GNOME reader tests) through the GNOME reader,
//! `tests/snapshots/adwaita.reader-<case>.json`, the case's name in lower
//! case with a hyphen for each run of other characters; each written from
//! `ResolvedTheme`'s `Serialize` (feature `serde`; the cases' also need
//! feature `portal` on Linux). A snapshot that differs fails the test; with
//! the environment variable [`UPDATE`] set the test writes the snapshots
//! instead, and a changed value is a snapshot diff justified by its citation
//! in the same commit.

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
        let theme = Theme::preset(Preset::Adwaita).resolve(&ctx);
        let name = format!("{}.{scheme}", Preset::Adwaita.as_str());
        differing.extend(check(&name, &theme)?);
    }
    report(&differing)
}

/// The file name of the reader case `name`: lower case, each run of
/// characters that are not ASCII letters or digits one hyphen.
#[cfg(all(target_os = "linux", feature = "portal"))]
fn slug(name: &str) -> String {
    let mut slug = String::new();
    for word in name
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
    {
        if !slug.is_empty() {
            slug.push('-');
        }
        slug.push_str(&word.to_ascii_lowercase());
    }
    slug
}

#[cfg(all(target_os = "linux", feature = "portal"))]
#[test]
fn each_reader_case_s_theme_is_its_snapshot() -> TestResult {
    use crate::next::parity::gnome_cases;
    use crate::next::reader::portal::snapshot_from_gnome_data;

    let cases = gnome_cases();
    // The cases of `reader_parity_gnome`, from the default data to another icon theme.
    assert_eq!(cases.len(), 24, "GNOME's reader cases");
    let mut names = std::collections::BTreeSet::new();
    let mut differing = Vec::new();
    for (case, data, _) in &cases {
        let slug = slug(case);
        assert!(names.insert(slug.clone()), "two cases named {slug}");
        let snapshot = snapshot_from_gnome_data(data);
        let theme = Theme::preset(snapshot.preset)
            .with_system_overrides(snapshot.overrides)
            .resolve(&snapshot.context);
        let name = format!("{}.reader-{slug}", Preset::Adwaita.as_str());
        differing.extend(check(&name, &theme)?);
    }
    report(&differing)
}
