//! The resolution pipeline's layer order (spec §6, "Resolution Algorithm";
//! plan, M1 item 6), through the public API on `kde-breeze`: the live
//! system over the preset, the palette over the system, the patch over
//! both, the transforms last.

use std::sync::Arc;

use native_theme::next::{
    Color, ColorPatch, Contrast, OsColors, OsFont, OsSurfaceOverrides, Palette, Preset,
    ResolvedTheme, Scheme, SurfaceColors, SystemOverrides, Theme, ThemeContext, ThemePatch,
};

/// Test inputs, not a theme's colours.
const RED: Color = Color::rgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);

/// A palette named `name` for `scheme`, its every colour `c`.
fn palette(name: &str, scheme: Scheme, c: Color) -> Palette {
    Palette::new(
        name, scheme, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c,
    )
}

/// The system as read in `scheme`: its theme name, its UI font's family,
/// and its window background `background` for `scheme` at the normal
/// contrast.
fn system(scheme: Scheme, background: Color) -> SystemOverrides {
    let mut window = SurfaceColors::default();
    window.background = Some(background);
    let mut surfaces = OsSurfaceOverrides::default();
    surfaces.window = Some(window);
    let mut colors = OsColors::default();
    colors.surfaces = Some(surfaces);
    let mut font = OsFont::default();
    font.family = Some(Arc::from("System Sans"));
    let mut system = SystemOverrides::default();
    system.color_schemes = vec![((scheme, Contrast::Normal), colors)];
    system.scheme = Some(scheme);
    system.theme_name = Some(Arc::from("System Theme"));
    system.ui_font = Some(font);
    system
}

fn breeze() -> Theme {
    Theme::preset(Preset::KdeBreeze)
}

/// The patch that sets the window's background to `background` and the UI
/// font family.
fn patch(background: Color) -> ThemePatch {
    let mut colors = ColorPatch::new();
    colors.window.background = Some(background);
    ThemePatch::new()
        .color_overrides(colors)
        .font_family("Patch Sans")
}

#[test]
fn the_system_lays_over_the_preset() {
    let ctx = ThemeContext::light();
    let preset = breeze().resolve(&ctx);
    let theme = breeze()
        .with_system_overrides(system(Scheme::Light, GREEN))
        .resolve(&ctx);
    assert_ne!(preset.colors.window.background, Some(GREEN));
    assert_eq!(theme.colors.window.background, Some(GREEN));
    assert_ne!(&*preset.typography.body.family, "System Sans");
    assert_eq!(&*theme.typography.body.family, "System Sans");
    assert_eq!(&*theme.name, "System Theme");
}

#[test]
fn a_palette_lays_over_the_system() {
    // The system was read dark; the palette locks the light scheme.
    let theme = breeze()
        .with_system_overrides(system(Scheme::Dark, GREEN))
        .with_palette(palette("Light Palette", Scheme::Light, BLUE));
    let resolved = theme.resolve(&ThemeContext::dark());
    assert_eq!(resolved.context.scheme, Scheme::Light);
    assert_eq!(&*resolved.name, "Light Palette");
    assert_eq!(resolved.colors.window.background, Some(BLUE));
    // Over the system's colours of the scheme resolved too.
    let light = breeze()
        .with_system_overrides(system(Scheme::Light, GREEN))
        .with_palette(palette("Light Palette", Scheme::Light, BLUE))
        .resolve(&ThemeContext::light());
    assert_eq!(light.colors.window.background, Some(BLUE));
}

#[test]
fn an_explicit_accent_lays_over_the_palette() {
    let theme = breeze().with_palette(palette("Light Palette", Scheme::Light, BLUE));
    let ctx = ThemeContext::light();
    assert_eq!(theme.resolve(&ctx).colors.accent.fill, BLUE);
    let accented = theme.with_accent(RED).resolve(&ctx);
    assert_eq!(accented.colors.accent.fill, RED);
    assert_eq!(accented.colors.window.background, Some(BLUE));
}

#[test]
fn a_patch_lays_over_the_palette_and_the_system() {
    let layered = breeze()
        .with_system_overrides(system(Scheme::Light, GREEN))
        .with_palette(palette("Light Palette", Scheme::Light, BLUE));
    let ctx = ThemeContext::light();
    let unpatched = layered.resolve(&ctx);
    assert_eq!(unpatched.colors.window.background, Some(BLUE));
    assert_eq!(&*unpatched.typography.body.family, "System Sans");
    let patched = layered.with_patch(patch(RED)).resolve(&ctx);
    assert_eq!(patched.colors.window.background, Some(RED));
    assert_eq!(&*patched.typography.body.family, "Patch Sans");
    assert_eq!(&*patched.name, "Light Palette");
    assert_eq!(patched.context.scheme, Scheme::Light);
}

#[test]
fn the_transforms_run_last() {
    let theme = breeze()
        .with_system_overrides(system(Scheme::Light, GREEN))
        .with_palette(palette("Light Palette", Scheme::Light, BLUE))
        .with_transform(|theme: &mut ResolvedTheme| {
            // It sees every layer below it.
            if theme.colors.window.background == Some(RED) && &*theme.name == "Light Palette" {
                theme.colors.window.background = Some(BLUE);
            }
        })
        .with_patch(patch(RED));
    let resolved = theme.resolve(&ThemeContext::light());
    assert_eq!(resolved.colors.window.background, Some(BLUE));
    assert_eq!(&*resolved.typography.body.family, "Patch Sans");
}

#[test]
fn a_lock_holds_its_scheme_a_pair_follows_the_context() {
    let lock = breeze().with_palette(palette("Dark Palette", Scheme::Dark, BLUE));
    let pair = breeze().with_palettes(
        palette("Light Palette", Scheme::Light, GREEN),
        palette("Dark Palette", Scheme::Dark, BLUE),
    );
    for (ctx, scheme, name) in [
        (ThemeContext::light(), Scheme::Light, "Light Palette"),
        (ThemeContext::dark(), Scheme::Dark, "Dark Palette"),
    ] {
        let locked = lock.resolve(&ctx);
        assert_eq!(locked.context.scheme, Scheme::Dark);
        assert_eq!(&*locked.name, "Dark Palette");
        let adaptive = pair.resolve(&ctx);
        assert_eq!(adaptive.context.scheme, scheme);
        assert_eq!(&*adaptive.name, name);
        assert_eq!(adaptive.is_dark(), scheme == Scheme::Dark);
    }
}
