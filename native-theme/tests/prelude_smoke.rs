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

/// The names `src/prelude.rs` exports: the last path segment (or `as` name)
/// of each `pub use`, each name of a `{..}` group, and any other `pub` item
/// whole, so it never matches a name.
fn exported_names(source: &str) -> Vec<String> {
    let code: String = source
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join(" ");
    let mut names = Vec::new();
    for statement in code.split(';').map(str::trim) {
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
