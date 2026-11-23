//! The spec's ergonomic API smoke target (spec §6, "Ergonomic API Smoke
//! Target"), compiled as written through `native_theme::next::*`.
//! `tests/prelude_smoke.rs` is the 0.7.2 model's until the cut.

use native_theme::next::*;

#[test]
fn ergonomic_api_smoke_target() -> Result<()> {
    // `Theme::from_system_or_default` is M3's (plan, M3);
    // `Theme::for_current_platform` stands in for it until then.
    let theme = Theme::for_current_platform();
    let resolved = theme.resolve_current();
    let _bg = resolved.colors.window.background;
    let _dark = resolved.is_dark();

    // Common 80% query — pass WidgetState directly:
    let _cb = resolved
        .checkbox
        .indicator(WidgetState::CHECKED | WidgetState::HOVER);
    let _btn = resolved.button.body(WidgetState::HOVER);

    // Full orthogonal query when variant/shape/size/surface matters:
    let _pill = resolved.button.body(
        ButtonQuery::primary()
            .pill()
            .size(ControlSize::Large)
            .state(WidgetState::HOVER),
    );

    // Re-resolve for an inactive window preserving all live OS settings:
    let _inactive = theme.resolve(&theme.context().clone().with_window_active(false));

    // Single-scheme palette lock vs adaptive light/dark pair + patch:
    #[cfg(feature = "palettes")]
    {
        let _mocha = Theme::preset(Preset::MacosTahoe)
            .with_palette(Palette::catppuccin_mocha())
            .resolve_current();
        // `Theme::from_system_or_default` is M3's, as above.
        let _adaptive = Theme::for_current_platform()
            .with_palettes(Palette::catppuccin_latte(), Palette::catppuccin_mocha())
            .with_accent(Color::from_hex("#7b2ff7")?)
            .with_patch(ThemePatch::new().corner_scale(1.25))
            .resolve_current();
    }
    Ok(())
}
