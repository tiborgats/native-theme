//! Smoke test: verify the prelude re-exports exactly the expected set.

/// Verify that `use native_theme::prelude::*` brings the 10 expected items
/// into scope and they are the correct types.
#[test]
fn prelude_reexports_expected_items() {
    use native_theme::prelude::*;

    // Each binding confirms the item is accessible via the prelude.
    // The type annotation confirms it is the correct type.
    let _theme: Theme = Theme::preset("catppuccin-mocha").unwrap();
    let _picked: Resolved = _theme
        .resolve_with(ColorMode::Light, &ResolutionContext::for_tests())
        .unwrap();
    let _mode = _theme.into_variant(ColorMode::Dark).unwrap();
    let _resolved: ResolvedTheme = _mode
        .into_resolved(&ResolutionContext::for_tests())
        .unwrap();
    let _rgba: Rgba = Rgba::rgb(255, 0, 0);
    let _err: Result<()> = Ok(());
    let _accessibility: AccessibilityPreferences = AccessibilityPreferences::default();

    // SystemTheme::from_system() may fail on CI, just verify the type exists.
    fn _assert_system_theme_type(_s: &SystemTheme) {}

    // Verify Error is the native_theme error type
    fn _assert_error_type(_e: &Error) {}
}

/// `statement` without the outer attributes (`#[..]`) in front of it.
fn without_attributes(mut statement: &str) -> &str {
    while let Some(attribute) = statement.strip_prefix("#[") {
        let mut depth = 1;
        let end = attribute.find(|c| {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                _ => {}
            }
            depth == 0
        });
        let Some(end) = end else {
            return statement;
        };
        statement = attribute[end + 1..].trim_start();
    }
    statement
}

/// The names `src/prelude.rs` exports: the last path segment (or `as` name)
/// of each `pub use`, attributed or not, each name of a `{..}` group, and any
/// other `pub` item whole, so it never matches a name. A `//` comment is
/// dropped to the end of its line.
fn exported_names(source: &str) -> Vec<String> {
    let code: String = source
        .lines()
        .map(|line| line.split("//").next().unwrap_or(line))
        .collect::<Vec<_>>()
        .join(" ");
    let mut names = Vec::new();
    for statement in code.split(';').map(str::trim) {
        let statement = without_attributes(statement);
        let Some(rest) = statement.strip_prefix("pub ") else {
            continue;
        };
        let Some(path) = rest.trim_start().strip_prefix("use ") else {
            names.push(statement.to_string());
            continue;
        };
        let leaves = match path.split_once('{') {
            Some((_, group)) => group.trim_end_matches('}').split(',').collect(),
            None => vec![path],
        };
        for leaf in leaves.into_iter().map(str::trim).filter(|l| !l.is_empty()) {
            let name = match leaf.rsplit_once(" as ") {
                Some((_, alias)) => alias,
                None => leaf.rsplit("::").next().unwrap_or(leaf),
            };
            names.push(name.trim().to_string());
        }
    }
    names.sort();
    names
}

/// Verify that prelude items are exactly what the design doc specifies.
/// If this test fails, the prelude has been modified -- check design doc section 12.
#[test]
fn prelude_does_not_export_unexpected_items() {
    let mut expected = [
        "AccessibilityPreferences",
        "ColorMode",
        "Error",
        "Resolved",
        "ResolvedTheme",
        "ResolutionContext",
        "Result",
        "Rgba",
        "SystemTheme",
        "Theme",
    ];
    expected.sort_unstable();
    assert_eq!(
        exported_names(include_str!("../src/prelude.rs")),
        expected,
        "src/prelude.rs exports another set"
    );
}

/// A `pub use` behind an attribute is an export.
#[test]
fn exported_names_reads_a_pub_use_behind_an_attribute() {
    let source = "pub use crate::A;\n\
                  #[doc(no_inline)]\n\
                  pub use crate::B;\n\
                  #[cfg(feature = \"kde\")] pub use crate::c::{C, D as E};\n";
    assert_eq!(exported_names(source), ["A", "B", "C", "E"]);
}

/// A comment after a `pub use` hides neither it nor the next one.
#[test]
fn exported_names_reads_past_a_trailing_comment() {
    let source = "pub use crate::A; // the first\n\
                  pub use crate::B;\n\
                  pub use crate::{C, D}; // two; with a semicolon\n\
                  pub use crate::F;\n";
    assert_eq!(exported_names(source), ["A", "B", "C", "D", "F"]);
}
