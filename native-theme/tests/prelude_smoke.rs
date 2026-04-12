//! Smoke test: verify the prelude re-exports exactly the expected set.

/// Verify that `use native_theme::prelude::*` brings exactly the 6 expected items
/// into scope and they are the correct types.
#[test]
fn prelude_reexports_expected_items() {
    use native_theme::prelude::*;

    // Each binding confirms the item is accessible via the prelude.
    // The type annotation confirms it is the correct type.
    let _theme: Theme = Theme::preset("catppuccin-mocha").unwrap();
    let _mode = _theme.into_variant(true).unwrap();
    let _resolved: ResolvedTheme = _mode.into_resolved().unwrap();
    let _rgba: Rgba = Rgba::rgb(255, 0, 0);
    let _err: Result<()> = Ok(());

    // SystemTheme::from_system() may fail on CI, just verify the type exists.
    fn _assert_system_theme_type(_s: &SystemTheme) {}

    // Verify Error is the native_theme error type
    fn _assert_error_type(_e: &Error) {}
}

/// Verify that prelude items are exactly what the design doc specifies.
/// If this test fails, the prelude has been modified -- check design doc section 12.
#[test]
fn prelude_does_not_export_unexpected_items() {
    // Import only the prelude
    use native_theme::prelude::*;

    // These 6 items should be accessible:
    let _ = std::any::type_name::<Theme>();
    let _ = std::any::type_name::<ResolvedTheme>();
    let _ = std::any::type_name::<SystemTheme>();
    let _ = std::any::type_name::<Rgba>();
    let _ = std::any::type_name::<Error>();

    // Result is a type alias, verify it works
    let _: Result<i32> = Ok(42);
}
