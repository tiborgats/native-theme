#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
//! The showcase's self-tests (spec §13 T11): the real `eframe::App`, driven headlessly.

use egui_kittest::Harness;
use native_theme::icons::{IconSetChoice, default_icon_choice};
use native_theme::{AccessibilityPreferences, theme::Theme};
use native_theme_egui::from_preset;

use crate::{
    CliArgs,
    app::{App, ModeChoice, Page, Settings, ThemeChoice},
    apply_cli_args,
};

/// A community preset every platform offers (`native-theme/src/presets.rs:196-199`).
pub(crate) const TEST_PRESET: &str = "catppuccin-mocha";

/// The showcase under a named preset, never the desktop's theme (§13 T11).
pub(crate) fn open(theme: egui::Theme, cli: CliArgs) -> Harness<'static, App> {
    Harness::builder()
        .with_theme(theme)
        .build_eframe(move |cc| {
            App::new(cc, &cli).expect("the showcase starts under a bundled preset")
        })
}

pub(crate) fn cli(pairs: &[(&str, &str)]) -> CliArgs {
    let mut cli = CliArgs::default();
    for (flag, value) in pairs {
        let value = Some((*value).to_string());
        match *flag {
            "--theme" => cli.theme = value,
            "--variant" => cli.variant = value,
            "--tab" => cli.tab = value,
            "--icon-set" => cli.icon_set = value,
            "--screenshot" => cli.screenshot = value,
            other => panic!("the tests do not pass {other}"),
        }
    }
    cli
}

/// T11 (a): every page in both schemes lays out, paints shapes and tessellates.
/// `run_steps`, not `run`: the Range and Icons pages animate and would exceed
/// `run`'s step limit (`egui_kittest/src/lib.rs:356`, `:451`).
#[test]
fn every_page_renders() {
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let mut harness = open(theme, cli(&[("--theme", TEST_PRESET)]));
        for page in Page::ALL {
            harness.state_mut().settings.page = page;
            harness.run_steps(4);
            let shapes = harness.output().shapes.clone();
            assert!(
                !shapes.is_empty(),
                "{page:?} under {theme:?} painted nothing"
            );
            let ppp = harness.ctx.pixels_per_point();
            let primitives = harness.ctx.tessellate(shapes, ppp);
            assert!(
                !primitives.is_empty(),
                "{page:?} under {theme:?} tessellated to nothing"
            );
        }
    }
}

/// T11 (c), first rule of §10.4: a flag value the pickers do not offer is
/// reported and ignored (the iced showcase's `the_command_line_rejects_what_it_cannot_honour`,
/// `connectors/native-theme-iced/examples/showcase-iced.rs:7086`).
#[test]
fn the_command_line_rejects_what_it_cannot_honour() {
    let mut settings = Settings::for_tests();
    let before = settings.clone();
    apply_cli_args(&mut settings, &cli(&[("--variant", "sideways")]));
    assert_eq!(settings, before, "--variant sideways changed the settings");
    apply_cli_args(&mut settings, &cli(&[("--variant", "dark")]));
    assert_eq!(settings.mode, ModeChoice::Dark);
    apply_cli_args(&mut settings, &cli(&[("--variant", "system")]));
    assert_eq!(settings.mode, ModeChoice::System);

    let offered = Theme::list_presets_for_platform();
    let foreign = Theme::list_presets()
        .iter()
        .find(|info| !offered.iter().any(|o| o.key == info.key))
        .map(|info| info.key)
        .expect("every bundled preset is offered on this platform");
    let before = settings.clone();
    apply_cli_args(&mut settings, &cli(&[("--theme", foreign)]));
    assert_eq!(
        settings, before,
        "--theme {foreign}, another platform's, changed the settings"
    );
    apply_cli_args(&mut settings, &cli(&[("--theme", "no-such-preset")]));
    assert_eq!(settings, before);
    apply_cli_args(&mut settings, &cli(&[("--theme", TEST_PRESET)]));
    assert_eq!(settings.theme, ThemeChoice::Preset(TEST_PRESET.to_string()));

    let before = settings.clone();
    apply_cli_args(&mut settings, &cli(&[("--icon-set", "no-such-icon-theme")]));
    assert_eq!(
        settings, before,
        "an icon theme that is not installed was accepted"
    );
    apply_cli_args(&mut settings, &cli(&[("--icon-set", "material")]));
    assert_eq!(settings.icon, IconSetChoice::Material);
    assert!(
        !settings.icon_follows_theme,
        "a command-line icon set is a pick"
    );
    apply_cli_args(&mut settings, &cli(&[("--icon-set", "default")]));
    assert!(
        settings.icon_follows_theme,
        "`--icon-set default` follows the theme again"
    );

    let before = settings.clone();
    apply_cli_args(&mut settings, &cli(&[("--tab", "no-such-page")]));
    assert_eq!(settings, before);
    apply_cli_args(&mut settings, &cli(&[("--tab", "icons")]));
    assert_eq!(settings.page, Page::Icons);
}

/// T11 (c), second rule of §10.4: the icon choice follows the theme until the
/// user picks one, and again after a pick of `default` (the iced showcase's
/// `an_icon_choice_that_followed_the_preset_keeps_following_it`,
/// `connectors/native-theme-iced/examples/showcase-iced.rs:7040`). Following is
/// `default_icon_choice` of what the atlas reports per scheme — `breeze`, then
/// `breeze-dark`, where that theme is installed, else `system` (`native-theme/src/icons.rs:741`),
/// which then keeps following.
#[test]
fn an_icon_choice_that_followed_the_preset_keeps_following_it() {
    let prefs = AccessibilityPreferences::default();
    let (breeze, _) = from_preset("kde-breeze", false, &prefs).expect("a bundled preset");
    let follows = |scheme| default_icon_choice(breeze.icon_set(), breeze.icon_theme(scheme));
    let mut settings = Settings::for_tests();
    assert!(settings.icon_follows_theme);

    settings.theme_installed(&breeze, egui::Theme::Light);
    assert_eq!(settings.icon, follows(egui::Theme::Light));
    settings.theme_installed(&breeze, egui::Theme::Dark);
    assert_eq!(settings.icon, follows(egui::Theme::Dark));

    settings.pick_icon(Some(IconSetChoice::Material));
    settings.theme_installed(&breeze, egui::Theme::Light);
    assert_eq!(
        settings.icon,
        IconSetChoice::Material,
        "a pick stays chosen"
    );
    settings.pick_icon(Some(IconSetChoice::System));
    settings.theme_installed(&breeze, egui::Theme::Dark);
    assert_eq!(
        settings.icon,
        IconSetChoice::System,
        "a pick of system stays chosen"
    );

    settings.pick_icon(None);
    assert!(
        settings.icon_follows_theme,
        "a pick of default follows again"
    );
    settings.theme_installed(&breeze, egui::Theme::Light);
    assert_eq!(settings.icon, follows(egui::Theme::Light));
}
