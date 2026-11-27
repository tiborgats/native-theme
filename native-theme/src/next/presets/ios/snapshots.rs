//! The golden snapshots of `ios-13` (plan, rule 2): the theme resolved for
//! each scheme, `tests/snapshots/ios-13.<scheme>.json`, written from
//! `ResolvedTheme`'s `Serialize` (feature `serde`). The preset has no
//! reader, so no reader case has a snapshot. A snapshot that differs fails
//! the test; with the environment variable [`UPDATE`] set the test writes
//! the snapshots instead, and a changed value is a snapshot diff justified
//! by its citation in the same commit.

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

#[test]
fn each_scheme_s_theme_is_its_snapshot() -> TestResult {
    let mut differing = Vec::new();
    for (scheme, ctx) in [
        ("light", ThemeContext::light()),
        ("dark", ThemeContext::dark()),
    ] {
        let theme = Theme::preset(Preset::Ios13).resolve(&ctx);
        let name = format!("{}.{scheme}", Preset::Ios13.as_str());
        differing.extend(check(&name, &theme)?);
    }
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
