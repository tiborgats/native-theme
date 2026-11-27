#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
//! The showcase's self-tests (spec §13 T11): the real `eframe::App`, driven headlessly.

use egui_kittest::Harness;
use native_theme::AccessibilityPreferences;
use native_theme::icons::{IconSetChoice, default_icon_choice};
use native_theme_showcase_support::{Kind, capture};

use crate::{
    CliArgs,
    app::{App, ModeChoice, Page, Settings, ThemeChoice, effective_preferences},
    apply_cli_args,
};

/// A platform preset every platform offers (`native-theme/src/presets.rs`, `PRESET_INFOS`: it
/// names no platform), which the 0.8.0 model has a `Preset` of, so the Theme row and `--theme`
/// take it (`cli::next_theme_key`): the community presets of the 0.7.2 model are palettes in the
/// 0.8.0 one.
pub(crate) const TEST_PRESET: &str = "material-3";

/// The showcase under a named preset, never the desktop's theme (§13 T11), in a window of the
/// size `main` opens (the harness's own default is 800 × 600, `egui_kittest/src/builder.rs:34`).
pub(crate) fn open(theme: egui::Theme, cli: CliArgs) -> Harness<'static, App> {
    open_at(theme, cli, 1.0)
}

/// [`open`] at `pixels_per_point` physical pixels per point (the harness's own is 1,
/// `egui_kittest/src/builder.rs:35`).
fn open_at(theme: egui::Theme, cli: CliArgs, pixels_per_point: f32) -> Harness<'static, App> {
    Harness::builder()
        .with_context_setup(emoji_fallbacks)
        .with_theme(theme)
        .with_size(crate::WINDOW_SIZE)
        .with_pixels_per_point(pixels_per_point)
        .build_eframe(move |cc| {
            App::new(cc, &cli).expect("the showcase starts under a bundled preset")
        })
}

/// egui's two monochrome emoji faces, `NotoEmoji-Regular` and `emoji-icon-font`
/// (`epaint_default_fonts`'s `monochrome_emoji_fonts`), each with the tweak egui's own
/// `FontDefinitions::default` gives it (`epaint-0.37.0/src/text/font_definitions.rs`), asked in
/// that order for a character no installed face has: font providers, which an atlas's
/// `Context::set_fonts` keeps, and whose faces follow every installed face of the chain
/// (`epaint-0.37.0/src/text/font_provider.rs`, `FontProvider::font_for`). Test setup only: the
/// harness panics on a missing glyph (`egui_kittest-0.37.0/src/builder.rs`,
/// `missing_glyph_policy`), and the showcase itself keeps eframe's system font fallback.
fn emoji_fallbacks(ctx: &egui::Context) {
    for (name, bytes, scale) in [
        (
            "NotoEmoji-Regular",
            epaint_default_fonts::NOTO_EMOJI_REGULAR,
            0.81,
        ),
        ("emoji-icon-font", epaint_default_fonts::EMOJI_ICON, 0.90),
    ] {
        let data = egui::FontData::from_static(bytes).tweak(egui::FontTweak {
            scale,
            ..Default::default()
        });
        ctx.add_font_provider(std::sync::Arc::new(move |_: &egui::FallbackRequest<'_>| {
            Some(egui::FontInsert::new(name, data.clone(), Vec::new()))
        }));
    }
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
            "--open-menu" => cli.open_menu = value,
            "--palette" => cli.palette = value,
            other => panic!("the tests do not pass {other}"),
        }
    }
    cli
}

/// Kinds only `page` records, one of which it always draws: seen, the page itself was drawn,
/// not only the chrome around it.
fn page_kinds(page: Page) -> &'static [&'static str] {
    match page {
        Page::Basic => &["Label (typography)"],
        Page::Buttons => &["button (enabled)"],
        Page::Selection => &["checkbox (unchecked)"],
        Page::Inputs => &["TextEdit (single line)"],
        Page::Range => &["Slider (horizontal)"],
        Page::Text => &["Label (wrapped)"],
        Page::Colour => &["ui.color_edit_button_srgba"],
        Page::Containers => &["card"],
        Page::Data => &["Table"],
        Page::Overlays => &["open window button"],
        // An icon the chosen set lacks is shown as absent (§10.4).
        Page::Icons => &["Image", "Image (absent)"],
        Page::ThemeMap => &["verdict filter"],
    }
}

/// Widget Info matches `mapping.toml`'s per-preset `exceptions` by preset key, so the key it is
/// handed is the preset's key — never the atlas's display name — and `default`'s the preset it
/// builds on.
///
/// No bundled row states an exception, so one is added to every row for the test preset: the
/// Widget tab of a hovered checkbox prints it only if the inspector is handed the key.
#[test]
fn widget_info_is_handed_the_preset_key() {
    let mut harness = open(egui::Theme::Light, cli(&[("--theme", TEST_PRESET)]));
    assert_eq!(harness.state().preset_key(), TEST_PRESET);
    assert_ne!(harness.state().preset_key(), harness.state().atlas.name());

    const WHY: &str = "an exception the test adds";
    let line = format!("on {TEST_PRESET}: {WHY}");
    if let Ok(manifest) = harness.state_mut().manifest.as_mut() {
        for row in &mut manifest.rows {
            row.exceptions
                .push((TEST_PRESET.to_string(), WHY.to_string()));
        }
    }
    harness.run_steps(4);
    let pos = centre_of(&harness, "checkbox (unchecked)");
    hover_and_settle(&mut harness, pos);
    assert_eq!(shown_kind(&harness), Some("checkbox (unchecked)"));
    assert!(
        harness.query_all_by_label_contains(&line).next().is_some(),
        "the Widget tab does not print {line:?}"
    );

    harness.state_mut().settings.theme = ThemeChoice::Default;
    let app = harness.state();
    assert_eq!(app.preset_key(), app.default_preset);
}

/// T11 (a): every page in both schemes lays out, paints shapes and tessellates, and what it
/// drew is that page's own content. `run_steps`, not `run`: the Range and Icons pages animate
/// and would exceed `run`'s step limit (`egui_kittest/src/lib.rs:356`, `:451`).
#[test]
fn every_page_renders() {
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let mut harness = open(theme, cli(&[("--theme", TEST_PRESET)]));
        for page in Page::ALL {
            harness.state_mut().settings.page = page;
            harness.run_steps(4);
            let kinds = page_kinds(page);
            assert!(
                harness
                    .state()
                    .registry
                    .records()
                    .iter()
                    .any(|r| kinds.contains(&r.info.kind)),
                "{page:?} under {theme:?} drew none of {kinds:?}"
            );
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
/// `connectors/native-theme-iced/examples/showcase-iced.rs:20373`).
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

    let offered = native_theme::theme::Theme::list_presets_for_platform();
    let foreign = native_theme::theme::Theme::list_presets()
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
    apply_cli_args(&mut settings, &cli(&[("--tab", "basic")]));
    assert_eq!(settings.page, Page::Basic);
}

/// The Basic page is the first page and the one the showcase opens on, with no `--tab`: the
/// page the three showcases' captures are compared on.
#[test]
fn the_showcase_opens_on_the_basic_page() {
    assert_eq!(Page::ALL.first(), Some(&Page::Basic));
    assert_eq!(Page::from_key("basic"), Ok(Page::Basic));
    let mut settings = Settings::for_tests();
    assert_eq!(settings.page, Page::Basic);
    apply_cli_args(&mut settings, &cli(&[("--theme", TEST_PRESET)]));
    assert_eq!(
        settings.page,
        Page::Basic,
        "a flag other than --tab kept the page"
    );
    let harness = open_default();
    assert_eq!(harness.state().settings.page, Page::Basic);
}

/// The pages are in the order of the counterpart table in `docs/showcase-elements.toml`'s
/// header, under the labels its egui column gives them, and so are the View menu's page items;
/// each page's `--tab` key is its label as the header says: lower case, each space a `-`.
#[test]
fn the_pages_follow_the_counterpart_table() {
    let table = crate::elements::page_order(Kind::Egui).expect("the counterpart table reads");
    let labels: Vec<&str> = Page::ALL.iter().map(|page| page.label()).collect();
    assert_eq!(labels, table, "the page tabs against the counterpart table");
    for page in Page::ALL {
        assert_eq!(
            page.key(),
            page.label().to_lowercase().replace(' ', "-"),
            "{page:?}'s --tab key"
        );
    }
    let view: Vec<Page> = Action::MENUS
        .iter()
        .filter(|(menu, _)| *menu == "View")
        .flat_map(|(_, items)| items.iter().flatten())
        .filter_map(|action| match action {
            Action::ShowPage(page) => Some(*page),
            _ => None,
        })
        .collect();
    assert_eq!(
        view,
        Page::ALL,
        "the View menu's pages against the page tabs"
    );
}

/// T11 (c), second rule of §10.4: the icon choice follows the theme until the
/// user picks one, and again after a pick of `default` (the iced showcase's
/// `an_icon_choice_that_followed_the_preset_keeps_following_it`,
/// `connectors/native-theme-iced/examples/showcase-iced.rs:20316`). Following is
/// `default_icon_choice` of what the atlas reports per scheme — `breeze`, then
/// `breeze-dark`, where that theme is installed, else `system`
/// (`native-theme/src/icons.rs:1439-1458`), which then keeps following.
#[test]
fn an_icon_choice_that_followed_the_preset_keeps_following_it() -> Result<(), String> {
    let breeze = preset_atlas_on("kde-breeze", &Err("not read by the test".to_owned()))
        .map_err(|e| e.to_string())?;
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
    Ok(())
}

/// The Step 4 ruling on Reduce motion: Preferences' switch off is no request, so an OS request
/// still stops the animations, as the gpui showcase's switch (native-theme-gpui
/// `forward_reduce_motion`) and the iced showcase's `motion_reduced` read it; the scale and the
/// two other flags are the user's where the user set them, and the OS's where not.
#[test]
fn reduce_motion_off_is_no_request() {
    let prefs = |scale, reduce_motion, high_contrast, reduce_transparency| {
        let mut prefs = AccessibilityPreferences::default();
        prefs.text_scaling_factor = scale;
        prefs.reduce_motion = reduce_motion;
        prefs.high_contrast = high_contrast;
        prefs.reduce_transparency = reduce_transparency;
        prefs
    };
    let os_on = prefs(1.25, true, true, true);
    let os_off = prefs(1.0, false, false, false);
    let user_off = prefs(1.5, false, false, false);
    let user_on = prefs(2.0, true, true, true);

    assert_eq!(
        effective_preferences(&os_on, Some(&user_off)),
        prefs(1.5, true, false, false),
        "the switch off keeps the OS's request; the rest is the user's"
    );
    assert_eq!(
        effective_preferences(&os_off, Some(&user_on)),
        prefs(2.0, true, true, true),
        "the switch on is a request; the rest is the user's"
    );
    assert_eq!(
        effective_preferences(&os_off, Some(&user_off)),
        prefs(1.5, false, false, false),
        "neither asks"
    );
    assert_eq!(effective_preferences(&os_on, None), os_on, "no override");
    assert_eq!(effective_preferences(&os_off, None), os_off, "no override");
}

use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT as _, Queryable as _};

use crate::{WINDOW_TITLE, chrome::Action, native_options};

/// The passes a test runs after an input: `Harness::run`'s own limit
/// (`egui_kittest/src/builder.rs:38`). The Basic page, the default one, shows a spinner, which
/// repaints on every pass (`egui/src/widgets/spinner.rs:39`), so `Harness::run`, which runs
/// until nothing asks for a repaint, never settles there.
const SETTLE: usize = 4;

fn open_default() -> Harness<'static, App> {
    open(egui::Theme::Light, cli(&[("--theme", TEST_PRESET)]))
}

/// A menu item's AccessKit label is its text and its shortcut text joined by a
/// space (`egui/src/atomics/atoms.rs:51-62`).
fn menu_item_label(ctx: &egui::Context, action: Action) -> String {
    match action.shortcut() {
        Some(shortcut) => format!(
            "{} {}",
            action.label(),
            crate::chrome::shortcut_text(ctx, &shortcut)
        ),
        None => action.label().to_string(),
    }
}

/// The Mode row's `ComboBox` opened and its row `mode` clicked.
fn pick_mode(harness: &mut Harness<'_, App>, mode: &str) {
    harness
        .get_by_role_and_label(Role::ComboBox, "Mode")
        .click();
    harness.run_steps(SETTLE);
    harness.get_by_label(mode).click();
    harness.run_steps(SETTLE);
}

/// T11 (b): the theme, mode and icon pickers install what they name.
#[test]
fn interactive_controls_respond() -> Result<(), String> {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let (other, other_name) = native_theme::theme::Theme::list_presets_for_platform()
        .into_iter()
        .map(|info| (info.key, info.display_name))
        .find(|(key, _)| *key != TEST_PRESET)
        .ok_or("no second preset is offered on this platform")?;

    // The rows are the presets' display names, as the gpui showcase's (§10.4, parity item 1).
    harness
        .get_by_role_and_label(Role::ComboBox, "Theme")
        .click();
    harness.run_steps(SETTLE);
    harness.get_by_label(other_name).click();
    harness.run_steps(SETTLE);
    assert_eq!(
        harness.state().settings.theme,
        ThemeChoice::Preset(other.to_string())
    );
    assert_eq!(harness.state().atlas.name(), preset_name(other)?);

    pick_mode(&mut harness, "Dark");
    assert_eq!(harness.state().settings.mode, ModeChoice::Dark);
    assert_eq!(harness.ctx.theme(), egui::Theme::Dark);

    harness
        .get_by_role_and_label(Role::ComboBox, "Icon theme")
        .click();
    harness.run_steps(SETTLE);
    // A row's label is `IconSetChoice`'s `Display` (`native-theme/src/icons.rs:1355-1365`).
    // The list holds every installed freedesktop theme before the bundled sets, so the row may
    // lie below the popup's fold: scroll it into view first (`egui_kittest/src/node.rs:152`).
    harness.get_by_label("Material (bundled)").scroll_to_me();
    harness.run_steps(SETTLE);
    harness.get_by_label("Material (bundled)").click();
    harness.run_steps(SETTLE);
    assert_eq!(harness.state().settings.icon, IconSetChoice::Material);
    assert!(
        !harness.state().settings.icon_follows_theme,
        "a pick in the picker stays chosen"
    );
    Ok(())
}

/// §13.2: the OS draws the frame; the title carries the version.
#[test]
fn the_window_asks_for_the_os_frame() {
    let options = native_options(false);
    assert_eq!(options.viewport.decorations, Some(true));
    assert_eq!(options.viewport.title.as_deref(), Some(WINDOW_TITLE));
    assert!(WINDOW_TITLE.contains(env!("CARGO_PKG_VERSION")));
}

/// A capture opens at the default size and keeps none an earlier run stored: no persisted
/// window, and an app id no desktop stored a geometry for. A normal run asks for the same size.
#[test]
fn a_capture_opens_at_the_default_size() {
    for capturing in [false, true] {
        let options = native_options(capturing);
        assert_eq!(
            options.viewport.inner_size,
            Some(crate::WINDOW_SIZE),
            "capturing: {capturing}"
        );
        assert_eq!(options.persist_window, !capturing, "capturing: {capturing}");
        assert_eq!(
            options.viewport.app_id,
            capturing.then(|| capture::app_id(Kind::Egui)),
            "capturing: {capturing}"
        );
    }
    let args = |argv: &[&str]| CliArgs::parse(argv.iter().map(|a| (*a).to_string()));
    assert!(!args(&["--tab", "buttons"]).capturing());
    assert!(args(&["--capture", "--tab", "buttons"]).capturing());
    assert_eq!(
        args(&["--capture", "--tab", "buttons"]).tab.as_deref(),
        Some("buttons")
    );
    assert!(args(&["--screenshot", "out.png"]).capturing());
}

/// The showcase follows the OS theme in an interactive run only: a capture (`--capture`,
/// `--screenshot`) and a layout dump (`--dump-layout`) show the theme the run started with.
#[test]
fn only_an_interactive_run_follows_the_os() {
    let args = |argv: &[&str]| CliArgs::parse(argv.iter().map(|a| (*a).to_string()));
    assert!(
        args(&[]).follows_os(),
        "a run without flags does not follow the OS"
    );
    for argv in [
        &["--capture"][..],
        &["--screenshot", "p"],
        &["--dump-layout", "f"],
    ] {
        assert!(!args(argv).follows_os(), "{argv:?} follows the OS");
    }
}

/// §13.2: menu row above the toolbar; the side panel's rows, a separator, the
/// inspector's tabs; the status bar at the bottom; the page tabs above the page.
#[test]
fn the_chrome_is_where_the_layout_puts_it() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let menu_row = harness.get_by_role_and_label(Role::Button, "File").rect();
    let toolbar = harness
        .get_by_role_and_label(Role::Button, "Command Palette")
        .rect();
    assert!(
        menu_row.bottom() <= toolbar.top(),
        "the toolbar is not below the menu row"
    );

    let theme = harness
        .get_by_role_and_label(Role::ComboBox, "Theme")
        .rect();
    let palette = harness
        .get_by_role_and_label(Role::ComboBox, "Palette")
        .rect();
    let mode = harness.get_by_role_and_label(Role::ComboBox, "Mode").rect();
    let icon = harness
        .get_by_role_and_label(Role::ComboBox, "Icon theme")
        .rect();
    let widget_tab = harness.get_by_role_and_label(Role::Button, "Widget").rect();
    // The Theme menu's button carries the same label: the inspector's tab is the one recorded
    // as an inspector tab (`Id::accesskit_id`, `egui/src/id.rs:103`).
    let inspector_tabs: Vec<egui::accesskit::NodeId> = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "Tab · Inspector")
        .map(|r| r.id.accesskit_id())
        .collect();
    let theme_tab = harness
        .query_all_by_role_and_label(Role::Button, "Theme")
        .find(|n| inspector_tabs.contains(&n.accesskit_node().locate().0))
        .expect("the inspector's Theme tab")
        .rect();
    // The Palette row between Theme and Mode, as the gpui showcase's.
    assert!(
        theme.bottom() <= palette.top()
            && palette.bottom() <= mode.top()
            && mode.bottom() <= icon.top()
    );
    // Each label sits above its control, as wide as its text, at the control's left edge; the
    // controls are as wide as the panel's content (parity item 1).
    let labels: Vec<egui::Rect> = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "Label · theme setting")
        .map(|r| r.rect)
        .collect();
    assert_eq!(labels.len(), 4, "four setting labels: {labels:?}");
    let panel = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "Side panel")
        .map(|r| r.rect)
        .expect("the side panel records itself");
    for (label, control) in labels.iter().zip([theme, palette, mode, icon]) {
        assert!(
            label.bottom() <= control.top() && (label.left() - control.left()).abs() < 1.0,
            "label {label:?} is not above its control {control:?}"
        );
        assert!(
            label.width() < control.width() && control.width() > panel.width() * 0.8,
            "control {control:?} is not as wide as the panel {panel:?}"
        );
    }
    assert!(icon.bottom() <= widget_tab.top() && (widget_tab.top() - theme_tab.top()).abs() < 1.0);
    let separator = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "Separator · side panel")
        .map(|r| r.rect)
        .expect("the side panel records its separator");
    assert!(icon.bottom() <= separator.top() && separator.bottom() <= widget_tab.top());

    let window = harness.ctx.viewport_rect(); // `egui/src/context.rs:2921`
    let status = harness
        .get_by_role_and_label(Role::Button, "Toggle Side Panel")
        .rect();
    assert!(
        (status.bottom() - window.bottom()).abs() <= status.height(),
        "the status bar is not the window's bottom"
    );

    let tab = harness.get_by_role_and_label(Role::Button, "Icons").rect();
    let page_top = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "Central panel")
        .map(|r| r.rect.top())
        .expect("the central panel records itself");
    let first_item = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "button (enabled)")
        .map(|r| r.rect.top())
        .expect("the Buttons page records its first button");
    assert!(
        tab.top() >= page_top && tab.bottom() <= first_item,
        "the page tabs are not above the page"
    );
    // A tab in view: under TEST_PRESET the row of tabs runs past the panel, the Icons tab
    // behind the row's Pages button.
    harness
        .get_by_role_and_label(Role::Button, "Buttons")
        .click();
    harness.run_steps(2);
    assert_eq!(harness.state().settings.page, Page::Buttons);
}

/// The Basic page's column rule of the 0.8.0 theme `harness` installed in `theme`'s slot, with
/// egui's style of that scheme (`pages::basic::ColumnRule::of`).
fn column_rule(harness: &Harness<'_, App>, theme: egui::Theme) -> crate::pages::basic::ColumnRule {
    crate::pages::basic::ColumnRule::of(
        harness.state().atlas.next_resolved(theme),
        &harness.ctx.style_of(theme),
    )
}

/// M9x CI item 9, the controller's rulings (M9-166, and item 15(1) of the round 2-3 review):
/// the Basic page's column count holds the Segmented control's own width as egui laid it out,
/// so the control may reach into the section gap but keeps `layout.widget_gap` from the next
/// column's left edge, and in the last column may reach into the page's right margin but keeps
/// `layout.widget_gap` from the content panel's edge, under every bundled
/// preset at a text scale of 1 and of 2.25, the largest, where three columns, the old floor,
/// left it past the window under macos-monterey, macos-tahoe and ios-13; and the three Linux
/// platform presets, installed through the Theme row's path ([`open_page_under`]), keep their
/// columns at a text scale of 1: kde-breeze
/// and adwaita five, material-3 four (the window's own content width). It prints each preset's
/// count.
#[test]
fn the_column_count_holds_the_segmented_control() {
    let mut faults = Vec::new();
    for preset in native_theme::next::Preset::ALL.iter().map(|p| p.as_str()) {
        for scale in [1.0, 2.25] {
            let harness = open_page_scaled(Page::Basic, preset, scale);
            let rule = column_rule(&harness, egui::Theme::Light);
            let gap = rule.widget_gap;
            let places = harness.state().registry.places();
            let columns: Vec<egui::Rect> = places
                .iter()
                .filter(|(id, _)| id.starts_with("basic.column_"))
                .map(|(_, r)| *r)
                .collect();
            println!("egui {preset} at {scale}: {} columns", columns.len());
            let (Some(control), Some(page)) = (
                places.get("basic.segmented.control").copied(),
                places.get("basic.page").copied(),
            ) else {
                faults.push(format!("{preset} at {scale}: no segmented control or page"));
                continue;
            };
            let next = columns
                .iter()
                .map(|c| c.left())
                .filter(|x| *x > control.left() + 0.5)
                .fold(f32::INFINITY, f32::min);
            if next.is_finite() && control.right() > next - gap + 0.01 {
                faults.push(format!(
                    "{preset} at {scale}: the control's right edge {} is within \
                     layout.widget_gap {gap} of the next column at {next}",
                    control.right()
                ));
            }
            let margin = rule.margin;
            // A floating bar is drawn over the right margin, its groove wide.
            let floating = if rule.overlay { rule.groove } else { 0.0 };
            // Half a pixel: the page puts each column's left edge on a whole pixel
            // (`pages::basic::show`), the count's columns being exact.
            if !next.is_finite() && control.right() > page.right() + margin - floating - gap + 0.5 {
                faults.push(format!(
                    "{preset} at {scale}: the control in the last column ends at {}, within \
                     layout.widget_gap {gap} of the panel's edge or a floating bar {floating} \
                     wide, the page's {} and its {margin} margin",
                    control.right(),
                    page.right()
                ));
            }
        }
    }
    for (preset, want) in [("kde-breeze", 5), ("adwaita", 5), ("material-3", 4)] {
        let harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        let laid = harness
            .state()
            .registry
            .places()
            .keys()
            .filter(|id| id.starts_with("basic.column_"))
            .count();
        if laid != want {
            faults.push(format!("{preset} at 1: {laid} columns, not {want}"));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// Each of the Basic page's arrangements, five columns to one, as the gpui and iced pages',
/// holds every group once, a column's groups in reading order (item 15(1) of the round 2-3
/// review: two and one columns where three do not hold the page's widest control).
#[test]
fn every_arrangement_holds_every_group_once_in_reading_order() {
    use crate::pages::basic::{FIVE, FOUR, GROUPS, ONE, THREE, TWO, arrangement};
    for (count, want) in [
        (5, &FIVE[..]),
        (4, &FOUR[..]),
        (3, &THREE[..]),
        (2, &TWO[..]),
        (1, &ONE[..]),
    ] {
        let laid = arrangement(count);
        assert_eq!(laid, want, "{count} columns");
        assert_eq!(laid.len(), count);
        for column in laid {
            assert!(
                column.windows(2).all(|w| w.first() < w.get(1)),
                "{count} columns: {column:?} is not in reading order"
            );
        }
        let mut all: Vec<usize> = laid.iter().flat_map(|c| c.iter().copied()).collect();
        all.sort_unstable();
        assert_eq!(
            all,
            (0..GROUPS.len()).collect::<Vec<_>>(),
            "{count} columns"
        );
    }
}

/// M9x CI item 9 (M9-166): the Basic Segmented control keeps its segments on one row, as the
/// platforms' segmented controls do, where its column is narrower than they are: under
/// macos-monterey at the largest text scale the showcase offers, 2.25, Day, Week and Month lie
/// at one y.
#[test]
fn the_segmented_control_keeps_its_segments_on_one_row() {
    let harness = open_scaled("macos-monterey", "basic", 2.25);
    let places = harness.state().registry.places();
    let ys: Vec<Option<f32>> = [
        "basic.segmented.day",
        "basic.segmented.week",
        "basic.segmented.month",
    ]
    .into_iter()
    .map(|id| places.get(id).map(|r| r.top()))
    .collect();
    assert!(
        ys.iter().all(|y| y.is_some() && *y == ys[0]),
        "the segments are not on one row: {ys:?}"
    );
}

/// M9x CI item E1 (M9-173): under a theme that states no `tab.tab(NONE).padding` across
/// (macos-tahoe's 0.8.0 theme), a page tab keeps egui's own button padding from each of its edges to its
/// label, as the connector's pair rule gives a side the theme leaves unstated
/// (native-theme-egui `src/convert.rs`, `to_button_padding`; `egui-0.37.0/src/style.rs:1568`), where
/// the showcase set the padding to the tab's line width and "Containers", about as wide as
/// `tab.min_width` 64, nearly touched its neighbours. Its galley lies at least that padding
/// inside the tab on both sides.
#[test]
fn a_page_tab_keeps_eguis_padding_where_the_theme_states_none() {
    use native_theme::next::{PartStated, WidgetState};
    let harness = open_scaled("macos-tahoe", "basic", 1.0);
    let stated = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .map(|r| r.tab.tab(WidgetState::NONE).stated);
    assert!(
        stated.is_some_and(|s| !s.intersects(PartStated::PADDING_LEFT | PartStated::PADDING_RIGHT)),
        "macos-tahoe states a tab padding across ({stated:?}), so this test proves nothing"
    );
    let own = egui::Style::default().spacing.button_padding.x;
    let label = "Containers";
    let tab = harness.get_by_role_and_label(Role::Button, label).rect();
    let galley = harness
        .output()
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) if text.galley.text() == label && tab.contains(text.pos) => {
                // The glyphs' extent: a button's galley can be laid out the button's width.
                Some(text.visual_bounding_rect())
            }
            _ => None,
        })
        .expect("the tab paints its label");
    let (left, right) = (galley.left() - tab.left(), tab.right() - galley.right());
    assert!(
        left + 0.01 >= own && right + 0.01 >= own,
        "{label}: its label {galley:?} is {left} and {right} from the edges of its tab {tab:?}, \
         under egui's padding {own}"
    );
}

/// Parity items 14 to 16: the status bar is the toggle, the environment as one line joined by
/// " · " naming the preset by its key, and the shown info's title flush right.
#[test]
fn the_status_bar_reads_as_the_gpui_showcases() {
    // A `Label`'s text is its AccessKit value (`egui/src/response.rs:962-964`).
    let environment_line = |harness: &Harness<'_, App>| {
        harness
            .query_all(By::new().predicate(|n| {
                n.value()
                    .is_some_and(|v| v.contains(" · ") && v.contains(TEST_PRESET))
            }))
            .filter_map(|n| n.accesskit_node().value())
            .next()
            .expect("one environment line")
    };
    let mut harness = open_page(Page::Buttons, egui::Theme::Light);
    let pos = centre_of(&harness, "button (enabled)");
    hover_and_settle(&mut harness, pos);
    let records = harness.state().registry.records();
    let rect_of = |kind: &str| {
        records
            .iter()
            .find(|r| r.info.kind == kind)
            .map(|r| r.rect)
            .unwrap_or_else(|| panic!("no {kind} record"))
    };
    // Selected while the panel shows; with its icon, or labelled where the set has none.
    let toggle_kind = [
        "Button · Ghost, icon, selected",
        "Button · Ghost, labelled, selected",
    ]
    .into_iter()
    .find(|kind| records.iter().any(|r| r.info.kind == *kind))
    .unwrap_or("a selected side-panel toggle");
    let (bar, toggle, environment, title) = (
        rect_of("Status bar"),
        rect_of(toggle_kind),
        rect_of("Label · environment"),
        rect_of("Label · shown info"),
    );
    assert!(toggle.right() <= environment.left() && environment.right() <= title.left());
    assert!(
        (bar.right() - title.right()).abs() <= bar.height(),
        "the title {title:?} is not flush right in {bar:?}"
    );
    let line = environment_line(&harness);
    assert!(
        line.contains(&format!("{TEST_PRESET} light")),
        "the preset is not named by its key: {line}"
    );
}

/// Parity item 9: the page tabs' menu lists every page, the current one selected, and its item
/// shows the page it names, as gpui-component's tab-bar menu does.
#[test]
fn the_page_menu_lists_and_shows_every_page() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    harness.get_by_role_and_label(Role::Button, "Pages").click();
    harness.run_steps(SETTLE);
    let ids: Vec<egui::accesskit::NodeId> = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "Menu item · Pages")
        .map(|r| r.id.accesskit_id())
        .collect();
    assert_eq!(ids.len(), Page::ALL.len(), "the menu lists every page");
    let selected: Vec<String> = harness
        // egui gives a selected button AccessKit's `toggled` (`egui/src/response.rs:975-976`).
        .query_all(By::new().predicate(|n| n.toggled() == Some(egui::accesskit::Toggled::True)))
        .filter(|n| ids.contains(&n.accesskit_node().locate().0))
        .filter_map(|n| n.accesskit_node().label())
        .collect();
    assert_eq!(selected, vec![Page::Basic.label().to_string()]);
    let item = harness
        .query_all_by_label(Page::ThemeMap.label())
        .find(|n| ids.contains(&n.accesskit_node().locate().0))
        .expect("the menu's Theme Map item");
    item.click();
    harness.run_steps(2);
    assert_eq!(harness.state().settings.page, Page::ThemeMap);
}

/// The side-panel toggle while the panel is open is checked (spec §10, rule 0a): under
/// kde-breeze, which states the flat button's checked fill, it is painted in
/// `button.body(CHECKED; variant=Flat).fill`, not in the flat button's pressed fill, and its
/// Widget Info routes that home to the call that paints it.
#[test]
fn the_open_side_panels_toggle_is_painted_checked() -> Result<(), String> {
    use native_theme::next::{ButtonQuery, WidgetState};
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    harness.run_steps(SETTLE);
    assert!(
        harness.state().side_panel_visible,
        "the side panel opens shown"
    );
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme is installed")?;
    let flat = |state| ButtonQuery::flat().with_state(state);
    let checked = crate::demo::drawn_state_fill(&r.button.body(flat(WidgetState::CHECKED)))
        .ok_or("kde-breeze states no checked flat fill")?;
    let pressed = crate::demo::drawn_state_fill(&r.button.body(flat(WidgetState::PRESSED)));
    assert_ne!(
        Some(checked),
        pressed,
        "the two fills are one, so this proves nothing"
    );
    let rect = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|record| record.info.element.as_deref() == Some("chrome.status_bar.toggle"))
        .map(|record| record.rect)
        .ok_or("the toggle records nothing")?;
    let fills: Vec<egui::Color32> = harness
        .output()
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Rect(shape) if shape.rect == rect => Some(shape.fill),
            _ => None,
        })
        .collect();
    assert_eq!(fills, [checked], "the toggle's frame at {rect:?}");
    let manifest = Manifest::parse(MANIFEST)?;
    let how = crate::info::listed_how(
        &manifest,
        "chrome.status_bar.toggle",
        "button.body(CHECKED).fill",
        &[],
    );
    assert!(how.contains("chrome::ghost_button"), "{how}");
    Ok(())
}

/// The open side panel's toggle names the checked fill it is drawn in at its own home: under
/// kde-breeze light its Widget Info row `button.body(CHECKED).fill` shows the push button's
/// checked fill, which differs from the flat button's it is painted in, so a line names
/// `button.body(CHECKED; variant=Flat).fill` with the theme's value, as the gpui showcase's does.
#[test]
fn the_open_toggle_names_the_checked_fill_it_is_drawn_in() -> Result<(), String> {
    use native_theme::next::{ButtonQuery, Paint, WidgetState};
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    harness.run_steps(SETTLE);
    assert!(
        harness.state().side_panel_visible,
        "the side panel opens shown"
    );
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme is installed")?
        .clone();
    let flat = r
        .button
        .body(ButtonQuery::flat().with_state(WidgetState::CHECKED))
        .fill;
    let push = r.button.body(WidgetState::CHECKED).fill;
    let Paint::Solid(colour) = flat else {
        return Err(format!(
            "kde-breeze states no solid checked flat fill: {flat:?}"
        ));
    };
    assert_ne!(
        push, flat,
        "the push and the flat checked fills are one, so this proves nothing"
    );
    let named = format!("button.body(CHECKED; variant=Flat).fill {colour}");
    assert!(
        every_record_notes(&harness, "chrome.status_bar.toggle", &[&named]),
        "no line of the toggle's info names {named:?}: {:?}",
        element_notes(&harness, "chrome.status_bar.toggle")
    );
    Ok(())
}

/// The flat buttons' hovered and pressed outlines (the side-panel toggle, the Pages menu button,
/// Widget Info's Copy) route to the flat button's homes `next::flat_button` lays over their
/// scope per call, not to the push button's row of `mapping.toml`.
#[test]
fn the_flat_buttons_outlines_route_to_the_flat_button() -> Result<(), String> {
    let manifest = Manifest::parse(MANIFEST)?;
    let seams = [crate::demo::Seam::Role(
        native_theme_egui::Role::Button,
        RoleVariant::Normal,
    )];
    for element in [
        "chrome.status_bar.toggle",
        "chrome.page_tabs.menu",
        "chrome.info.copy",
    ] {
        for (home, drawn) in [
            (
                "button.body(HOVER).stroke.color",
                "button.body(HOVER; variant=Flat).stroke.color",
            ),
            (
                "button.body(PRESSED).stroke.color",
                "button.body(PRESSED; variant=Flat).stroke.color",
            ),
        ] {
            let how = crate::info::listed_how(&manifest, element, home, &seams);
            assert!(
                how.contains(drawn) && how.contains("next::flat_button"),
                "{element} {home}: {how}"
            );
        }
    }
    Ok(())
}

/// A `mapping.toml` row whose homes egui draws in different `visuals.widgets` entries routes
/// each home to its own entry's sink: `button.hover_border_color`'s pressed home to the
/// `active` stroke, its hovered one to the `hovered` stroke, in Widget Info's listed rows as in
/// its route line.
#[test]
fn a_rows_route_is_the_sink_of_the_home_asked() -> Result<(), String> {
    let manifest = Manifest::parse(MANIFEST)?;
    let seams = [crate::demo::Seam::Role(
        native_theme_egui::Role::Button,
        RoleVariant::Normal,
    )];
    let row = manifest
        .rows
        .iter()
        .find(|r| r.leaf == "button.hover_border_color")
        .ok_or("mapping.toml has no button.hover_border_color row")?;
    for (home, sink) in [
        (
            "button.body(HOVER).stroke.color",
            "visuals.widgets.hovered.bg_stroke.color",
        ),
        (
            "button.body(PRESSED).stroke.color",
            "visuals.widgets.active.bg_stroke.color",
        ),
    ] {
        let how = crate::info::how_line(&manifest, home, &seams);
        assert!(how.contains(sink), "{home}: {how}");
        let listed = crate::info::manifest_rows(
            row,
            (
                &crate::info::InfoTheme::new(
                    native_theme::next::Theme::preset(native_theme::next::Preset::KdeBreeze)
                        .resolve(&native_theme::next::ThemeContext::light()),
                )?,
                "kde-breeze",
            ),
            &crate::info::as_stated,
            &seams,
        );
        let line = listed
            .iter()
            .find(|r| r.leaf == home)
            .map(|r| r.how.clone())
            .ok_or_else(|| format!("{home} is not listed"))?;
        assert!(line.contains(sink), "{home}: {line}");
    }
    Ok(())
}

/// The fills of the frames painted on the side-panel toggle's rect.
fn toggle_frame_fills(harness: &Harness<'_, App>) -> Result<Vec<egui::Color32>, String> {
    let rect = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|record| record.info.element.as_deref() == Some("chrome.status_bar.toggle"))
        .map(|record| record.rect)
        .ok_or("the toggle records nothing")?;
    Ok(harness
        .output()
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Rect(shape) if shape.rect == rect => Some(shape.fill),
            _ => None,
        })
        .collect())
}

/// The open side panel's toggle under kde-breeze light, which states the flat button's checked
/// outline (`button.body(CHECKED; variant=Flat).stroke`, ButtonText's frame pen), is outlined
/// in it, not left unframed as the flat button at rest.
#[test]
fn the_open_toggle_is_outlined_in_the_checked_flat_pen() -> Result<(), String> {
    use native_theme::next::{ButtonQuery, WidgetState};
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    harness.run_steps(SETTLE);
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme is installed")?;
    let pen = crate::demo::state_pen(
        &r.button
            .body(ButtonQuery::flat().with_state(WidgetState::CHECKED)),
    )
    .ok_or("kde-breeze states no checked flat outline")?;
    assert!(pen.width > 0.0 && pen.color.a() > 0, "{pen:?}");
    let rect = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|record| record.info.element.as_deref() == Some("chrome.status_bar.toggle"))
        .map(|record| record.rect)
        .ok_or("the toggle records nothing")?;
    let strokes: Vec<egui::Stroke> = harness
        .output()
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Rect(shape) if shape.rect == rect => Some(shape.stroke),
            _ => None,
        })
        .collect();
    assert_eq!(strokes, [pen], "the toggle's frame at {rect:?}");
    Ok(())
}

/// The Basic page's primary button is outlined at rest in the pen the theme states for the
/// primary variant, `button.body(NONE; variant=Primary).stroke`, not in the push button's, and
/// its Widget Info routes that home to the call that draws it.
#[test]
fn the_primary_button_is_outlined_in_its_own_pen() -> Result<(), String> {
    use native_theme::next::{
        ButtonRule, ButtonVariant, Color, PartPatch, Stroke, ThemePatch, WidgetState,
    };
    let magenta = Color::new(1.0, 0.0, 1.0, 1.0);
    let patch = |_: &native_theme::next::ResolvedTheme| {
        Some(
            ThemePatch::new().button(
                ButtonRule::new()
                    .variant(ButtonVariant::Primary)
                    .states(WidgetState::NONE)
                    .body(PartPatch::new().stroke(Stroke::uniform(1.0, magenta))),
            ),
        )
    };
    let (mut harness, _) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", patch)?;
    harness.run_steps(SETTLE);
    let rect = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|record| record.info.element.as_deref() == Some("basic.buttons.primary"))
        .map(|record| record.rect)
        .ok_or("the primary button records nothing")?;
    let pen = egui::Stroke::new(1.0, native_theme_egui::next::to_color32(magenta));
    let strokes: Vec<egui::Stroke> = harness
        .output()
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Rect(shape) if shape.rect == rect => Some(shape.stroke),
            _ => None,
        })
        .collect();
    assert_eq!(strokes, [pen], "the primary button's frame at {rect:?}");
    let manifest = Manifest::parse(MANIFEST)?;
    let how = crate::info::listed_how(
        &manifest,
        "basic.buttons.primary",
        "button.body(NONE; variant=Primary).stroke.color",
        &[],
    );
    assert!(how.contains("primary_pen"), "{how}");
    Ok(())
}

/// The side-panel toggle's icon takes the colour the theme states for the flat button's icon
/// in the state it is in, `button.icon(CHECKED; variant=Flat).color` while on and
/// `button.icon(NONE; variant=Flat).color` while off; kde-breeze states none, and the icon is
/// not lettered in the checked label's colour. While on and pressed or hovered, where a rule
/// states the checked colour, the theme's answer for `(CHECKED | PRESSED)` or `(CHECKED |
/// HOVER)` (N26); where only the resting one is stated, it holds in every state.
#[test]
fn the_toggles_icon_takes_the_stated_flat_icon_colour() {
    use native_theme::next::{
        ButtonRule, ButtonVariant, Color, IconPatch, Preset, Theme, ThemeContext, ThemePatch,
        WidgetState,
    };
    use native_theme_egui::next::to_color32;
    const REST: (bool, bool) = (false, false);
    const HOVERED: (bool, bool) = (true, false);
    const PRESSED: (bool, bool) = (true, true);
    let colour = |r, open, was| crate::chrome::toggle_icon_colour(Some(r), open, was);
    let stock = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
    for was in [REST, HOVERED, PRESSED] {
        assert_eq!(colour(&stock, true, was), None, "{was:?}");
        assert_eq!(colour(&stock, false, was), None, "{was:?}");
        assert_eq!(crate::chrome::toggle_icon_colour(None, true, was), None);
    }
    let [on, off, hover, press] = [
        Color::new(1.0, 0.0, 1.0, 1.0),
        Color::new(0.0, 1.0, 0.0, 1.0),
        Color::new(0.0, 0.0, 1.0, 1.0),
        Color::new(1.0, 1.0, 0.0, 1.0),
    ];
    let rule = |state: WidgetState, colour: Color| {
        ButtonRule::new()
            .variant(ButtonVariant::Flat)
            .states(state)
            .icon(IconPatch::new().color(colour))
    };
    let rest_only = Theme::preset(Preset::KdeBreeze)
        .with_patch(ThemePatch::new().button(rule(WidgetState::NONE, off)))
        .resolve(&ThemeContext::light());
    for was in [REST, HOVERED, PRESSED] {
        assert_eq!(
            colour(&rest_only, true, was),
            Some(to_color32(off)),
            "{was:?}"
        );
    }
    let patch = ThemePatch::new()
        .button(rule(WidgetState::NONE, off))
        .button(rule(WidgetState::CHECKED, on))
        .button(rule(WidgetState::CHECKED | WidgetState::HOVER, hover))
        .button(rule(WidgetState::CHECKED | WidgetState::PRESSED, press));
    let r = Theme::preset(Preset::KdeBreeze)
        .with_patch(patch)
        .resolve(&ThemeContext::light());
    assert_eq!(colour(&r, true, REST), Some(to_color32(on)));
    assert_eq!(colour(&r, true, HOVERED), Some(to_color32(hover)));
    assert_eq!(colour(&r, true, PRESSED), Some(to_color32(press)));
    for was in [REST, HOVERED, PRESSED] {
        assert_eq!(colour(&r, false, was), Some(to_color32(off)), "{was:?}");
    }
}

/// The side-panel toggle while the panel is open, under a theme that states no checked flat
/// fill or label (macos-monterey light states neither), is egui's own selected button
/// (`Button::selected`, spec §10, rule 0a): its frame is filled as the suggested-action button
/// beside the Basic page's toggle, egui's selected one too, not in the flat button's pressed
/// fill, and its Widget Info names egui's own under *Not themeable*.
#[test]
fn an_unstated_checked_toggle_is_egui_s_selected_button_not_the_pressed_one() -> Result<(), String>
{
    use crate::pages::basic::{ToggleOn, toggle_on_at};
    use native_theme::next::{ButtonQuery, WidgetState};
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "macos-monterey");
    harness.run_steps(SETTLE);
    assert!(
        harness.state().side_panel_visible,
        "the side panel opens shown"
    );
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme is installed")?
        .clone();
    let flat = |state| ButtonQuery::flat().with_state(state);
    assert_eq!(
        toggle_on_at(Some(&r), flat(WidgetState::CHECKED)),
        ToggleOn {
            fill: None,
            text: None
        },
        "macos-monterey light states a checked flat colour, so this proves nothing"
    );
    let (selected, _) = button_paint(
        &harness,
        "button (suggested action)",
        "Primary",
        "basic.buttons.primary",
    )?;
    let pressed = crate::demo::drawn_state_fill(&r.button.body(flat(WidgetState::PRESSED)));
    assert_ne!(
        pressed,
        Some(selected),
        "the flat button's pressed fill is the selected button's, so this proves nothing"
    );
    assert_eq!(
        toggle_frame_fills(&harness)?,
        [selected],
        "the open side panel's toggle is not egui's selected button"
    );
    assert!(
        every_record_notes(
            &harness,
            "chrome.status_bar.toggle",
            &[
                "button.body(CHECKED; variant=Flat).fill",
                ToggleOn::OWN_FILL
            ]
        ),
        "the toggle's info does not name egui's own selected fill: {:?}",
        element_notes(&harness, "chrome.status_bar.toggle")
    );
    Ok(())
}

/// The side-panel toggle while the panel is open letters its label in
/// `button.label(CHECKED; variant=Flat).color` where a rule states it, and in egui's own selected
/// button's text colour, the suggested-action button's, where none does, never the flat button's
/// pressed label (spec §10, rule 0a): on macos-monterey light, which states no checked flat
/// colour, unpatched and with that label patched to the theme's `colors.window.border`. A
/// freedesktop icon theme no host has ([`ABSENT_ICON_THEME`]) has the toggle show its label.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
#[test]
fn the_open_toggle_letters_its_label_in_the_checked_or_the_selected_colour() -> Result<(), String> {
    use crate::pages::basic::toggle_on_at;
    use native_theme::next::{
        ButtonQuery, ButtonRule, ButtonVariant, TextPatch, ThemePatch, WidgetState,
    };
    const LABEL: &str = "Toggle Side Panel";
    const ID: &str = "chrome.status_bar.toggle";
    let flat = |state| ButtonQuery::flat().with_state(state);
    let labelled = |harness: &mut Harness<'static, App>| {
        harness.state_mut().settings.icon =
            IconSetChoice::Freedesktop(ABSENT_ICON_THEME.to_owned());
        harness.run_steps(SETTLE);
        assert!(
            harness.state().side_panel_visible,
            "the side panel opens shown"
        );
    };
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "macos-monterey");
    labelled(&mut harness);
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme is installed")?
        .clone();
    assert_eq!(
        toggle_on_at(Some(&r), flat(WidgetState::CHECKED)).text,
        None,
        "macos-monterey light states a checked flat label, so this proves nothing"
    );
    let (_, selected) = button_paint(
        &harness,
        "button (suggested action)",
        "Primary",
        "basic.buttons.primary",
    )?;
    let pressed =
        native_theme_egui::next::state_text_color_of(&r.button.label(flat(WidgetState::PRESSED)));
    assert_ne!(
        pressed,
        Some(selected),
        "the flat button's pressed label is the selected button's, so this proves nothing"
    );
    assert_eq!(
        painted_label_colour(&harness, LABEL, ID),
        Some(selected),
        "unstated, the open toggle's label is not egui's selected button's"
    );
    let (mut harness, r) = open_patched(Page::Basic, egui::Theme::Light, "macos-monterey", |r| {
        Some(
            ThemePatch::new().button(
                ButtonRule::new()
                    .variant(ButtonVariant::Flat)
                    .states(WidgetState::CHECKED)
                    .label(TextPatch::new().color(r.colors.window.border?)),
            ),
        )
    })?;
    labelled(&mut harness);
    let stated = toggle_on_at(Some(&r), flat(WidgetState::CHECKED)).text;
    assert!(
        stated.is_some_and(|stated| stated != selected),
        "the patched checked flat label {stated:?} is the selected button's, so this proves nothing"
    );
    assert_eq!(
        painted_label_colour(&harness, LABEL, ID),
        stated,
        "the open toggle's label is not button.label(CHECKED; variant=Flat).color"
    );
    Ok(())
}

/// §13.2: three ways to hide and show the side panel; the dragged width survives.
#[test]
fn the_side_panel_toggle_hides_and_shows_it() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let width_of = |h: &Harness<'_, App>| {
        h.state()
            .registry
            .records()
            .iter()
            .find(|r| r.info.kind == "Side panel")
            .map(|r| r.rect.width())
    };
    let initial = width_of(&harness).expect("the side panel is shown at start");

    harness
        .get_by_role_and_label(Role::Button, "Toggle Side Panel")
        .click();
    harness.run_steps(SETTLE);
    assert!(!harness.state().side_panel_visible && width_of(&harness).is_none());
    harness
        .get_by_role_and_label(Role::Button, "Toggle Side Panel")
        .click();
    harness.run_steps(SETTLE);
    assert!(harness.state().side_panel_visible);

    harness.get_by_role_and_label(Role::Button, "View").click();
    harness.run_steps(SETTLE);
    harness
        .get_by_label(&menu_item_label(&harness.ctx, Action::ToggleSidePanel))
        .click();
    harness.run_steps(SETTLE);
    assert!(!harness.state().side_panel_visible);
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::B);
    harness.run_steps(SETTLE);
    assert!(harness.state().side_panel_visible);

    // Drag the panel's edge 40 points right, then hide and show it.
    let edge = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "Side panel")
        .map(|r| egui::pos2(r.rect.right(), r.rect.center().y))
        .expect("shown");
    harness.hover_at(edge);
    harness.step();
    harness.drag_at(edge);
    harness.step();
    harness.hover_at(edge + egui::vec2(40.0, 0.0));
    harness.step();
    harness.drop_at(edge + egui::vec2(40.0, 0.0));
    harness.run_steps(SETTLE);
    let dragged = width_of(&harness).expect("shown");
    assert!(
        (dragged - (initial + 40.0)).abs() < 2.0,
        "dragged {initial} -> {dragged}"
    );
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::B);
    harness.run_steps(SETTLE);
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::B);
    harness.run_steps(SETTLE);
    assert_eq!(
        width_of(&harness),
        Some(dragged),
        "the dragged width did not survive hiding"
    );
}

/// The menu bar's button `menu`, clicked: the one the menu bar recorded, since the inspector's
/// Theme tab carries the Theme menu's label.
fn click_menu(harness: &mut Harness<'_, App>, menu: &str) {
    let buttons: Vec<egui::accesskit::NodeId> = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "Menu button")
        .map(|r| r.id.accesskit_id())
        .collect();
    harness
        .query_all_by_role_and_label(Role::Button, menu)
        .find(|n| buttons.contains(&n.accesskit_node().locate().0))
        .unwrap_or_else(|| panic!("no {menu} button in the menu bar"))
        .click();
}

/// §13.2: every item acts on the app, and every shortcut label is egui's own spelling. Each
/// item's effect is absent before its click — the app starts on the Text page in Dark mode, so
/// View > Buttons and Theme > System change something, and Reload finds a selection changed
/// behind the app's back that only an install shows — so an item wired to another's action,
/// or to none, fails.
#[test]
fn the_menus_run_their_actions() -> Result<(), String> {
    let mut harness = open(
        egui::Theme::Dark,
        cli(&[
            ("--theme", TEST_PRESET),
            ("--tab", Page::Text.key()),
            ("--variant", "dark"),
        ]),
    );
    harness.run_steps(2);
    let other = native_theme::theme::Theme::list_presets_for_platform()
        .into_iter()
        .map(|info| info.key)
        .find(|key| *key != TEST_PRESET)
        .ok_or("no second preset is offered on this platform")?;
    let reloaded = preset_name(other)?;
    // `run_steps`, not `run`: View > Range shows a page that repaints every pass (§13 T11).
    for (menu, items) in Action::MENUS {
        for action in items.iter().flatten() {
            let acted = |app: &App| match action {
                Action::ShowPage(page) => app.settings.page == *page,
                Action::ToggleSidePanel => !app.side_panel_visible,
                Action::OpenCommandPalette => app.palette.is_some(),
                Action::ReloadTheme => app.atlas.name() == reloaded,
                Action::SetMode(mode) => app.settings.mode == *mode,
                Action::OpenPreferences => app.preferences_open,
                Action::OpenAbout => app.about_open,
                Action::Quit => app.quit_requested,
            };
            if *action == Action::ReloadTheme {
                harness.state_mut().settings.theme = ThemeChoice::Preset(other.to_string());
            }
            assert!(
                !acted(harness.state()),
                "{menu} > {action:?} holds before its click, so the click proves nothing"
            );
            click_menu(&mut harness, menu);
            harness.run_steps(2);
            let label = menu_item_label(&harness.ctx, *action);
            // The item inside the open menu: a page tab carries the same text ("Icons"), so the
            // label alone is not unique (`kittest/src/query.rs:65-70`): the node is the one the
            // menu recorded as its item (`Id::accesskit_id`, `egui/src/id.rs:103`).
            let items: Vec<egui::accesskit::NodeId> = harness
                .state()
                .registry
                .records()
                .iter()
                .filter(|r| r.info.kind == "Menu item")
                .map(|r| r.id.accesskit_id())
                .collect();
            let item = harness
                .query_all_by_label(&label)
                .find(|n| items.contains(&n.accesskit_node().locate().0))
                .unwrap_or_else(|| panic!("{menu} > {label} is not in the open menu"));
            assert_eq!(
                item.accesskit_node().label().as_deref(),
                Some(label.as_str())
            );
            item.click();
            harness.run_steps(2);
            assert!(acted(harness.state()), "{menu} > {label} did not act");
            // Put the app back so the next item starts from the same state.
            let app = harness.state_mut();
            app.palette = None;
            app.preferences_open = false;
            app.about_open = false;
            app.side_panel_visible = true;
            app.quit_requested = false;
            harness.run_steps(2);
        }
    }
    Ok(())
}

/// M9-117: the macOS menu bar is Apple's HIG list (*The menu bar*), under the titles the shared
/// crate gives the three showcases: the app menu, holding the showcase's own About and
/// Settings… and AppKit's Services, Hide, Hide Others, Show All and Quit, with the HIG's
/// separators; File, holding Close; View, holding the in-window View menu's rows; Theme,
/// holding the in-window Theme menu's rows less Preferences…, which is the app menu's
/// Settings…; Window, holding Minimize, Zoom and Bring All to Front; and no Help menu. The
/// list is built on every host; the menu bar itself only on macOS.
#[test]
fn the_macos_menus_are_the_hig_list() {
    use crate::chrome::system_menu::{APP_NAME, Row, SystemItem, menus};
    use native_theme_showcase_support::menu::macos_menu_titles;

    let own = |label: &str, action| Row::Own {
        label: label.to_string(),
        action,
    };
    let in_window = |title: &str| -> Vec<Row> {
        Action::MENUS
            .iter()
            .filter(|(menu, _)| *menu == title)
            .flat_map(|(_, items)| items.iter())
            .map(|item| match item {
                Some(action) => own(action.label(), *action),
                None => Row::Separator,
            })
            .collect()
    };
    let built = menus();
    let titles: Vec<&str> = built.iter().map(|(title, _)| title.as_str()).collect();
    assert_eq!(titles, macos_menu_titles(APP_NAME), "the menu bar's titles");
    let rows = |at: usize| &built[at].1;
    assert_eq!(
        rows(0),
        &[
            own(&format!("About {APP_NAME}"), Action::OpenAbout),
            Row::Separator,
            own("Settings…", Action::OpenPreferences),
            Row::Separator,
            Row::System(SystemItem::Services),
            Row::Separator,
            Row::System(SystemItem::Hide),
            Row::System(SystemItem::HideOthers),
            Row::System(SystemItem::ShowAll),
            Row::Separator,
            Row::System(SystemItem::Quit),
        ],
        "the app menu"
    );
    assert_eq!(rows(1), &[Row::System(SystemItem::Close)], "File");
    assert_eq!(rows(2), &in_window("View"), "View");
    let theme = in_window("Theme");
    assert_eq!(
        theme[theme.len().saturating_sub(2)..],
        [Row::Separator, own("Preferences…", Action::OpenPreferences)],
        "the in-window Theme menu no longer ends with Preferences…"
    );
    assert_eq!(rows(3), &theme[..theme.len() - 2], "Theme");
    assert_eq!(
        rows(4),
        &[
            Row::System(SystemItem::Minimize),
            Row::System(SystemItem::Zoom),
            Row::Separator,
            Row::System(SystemItem::BringAllToFront),
        ],
        "Window"
    );
    assert_eq!(
        Action::OpenPreferences.shortcut(),
        Some(egui::KeyboardShortcut::new(
            egui::Modifiers::COMMAND,
            egui::Key::Comma
        )),
        "Settings… is not ⌘,"
    );
}

/// The muda predefined items take the three showcases' one wording where muda's own differs:
/// muda names Hide and Quit after the running application's `localizedName` and Close
/// "Close Window"; the rest keep muda's.
#[test]
fn the_macos_system_items_take_the_showcases_wording() {
    use crate::chrome::system_menu::{APP_NAME, SystemItem};

    let wording = |item: SystemItem| item.wording();
    assert_eq!(wording(SystemItem::Hide), Some(format!("Hide {APP_NAME}")));
    assert_eq!(wording(SystemItem::Quit), Some(format!("Quit {APP_NAME}")));
    assert_eq!(wording(SystemItem::Close), Some("Close".to_string()));
    for item in [
        SystemItem::Services,
        SystemItem::HideOthers,
        SystemItem::ShowAll,
        SystemItem::Minimize,
        SystemItem::Zoom,
        SystemItem::BringAllToFront,
    ] {
        assert_eq!(wording(item), None, "{item:?}");
    }
}

/// §13.2: Ctrl+K opens the palette, typing filters, a row acts, Escape clears then closes.
#[test]
fn the_command_palette_runs_what_it_lists() -> Result<(), String> {
    // The rows the palette shows, by their labels: the page tabs carry the page names too, so a
    // row is the node the palette recorded as its row (`Id::accesskit_id`, `egui/src/id.rs:103`).
    fn rows(harness: &Harness<'_, App>) -> Vec<(egui::accesskit::NodeId, String)> {
        let ids: Vec<egui::accesskit::NodeId> = harness
            .state()
            .registry
            .records()
            .iter()
            .filter(|r| r.info.kind == "palette row")
            .map(|r| r.id.accesskit_id())
            .collect();
        ids.into_iter()
            .filter_map(|id| {
                harness
                    .query_all(By::new().predicate(move |n| n.locate().0 == id))
                    .next()
                    .and_then(|n| n.accesskit_node().label())
                    .map(|label| (id, label))
            })
            .collect()
    }
    fn click_row(harness: &mut Harness<'_, App>, label: &str) {
        let id = rows(harness)
            .into_iter()
            .find(|(_, l)| l == label)
            .map(|(id, _)| id)
            .unwrap_or_else(|| panic!("no palette row {label}"));
        harness
            .query_all(By::new().predicate(move |n| n.locate().0 == id))
            .next()
            .expect("the row's node")
            .click();
    }
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::K);
    harness.run_steps(SETTLE);
    assert!(harness.state().palette.is_some());
    let field = harness
        .query_all_by_role(Role::TextInput)
        .find(|n| n.is_focused())
        .expect("the palette's field has focus");
    field.type_text("Ico");
    harness.run_steps(SETTLE);
    let labels: Vec<String> = rows(&harness).into_iter().map(|(_, l)| l).collect();
    assert_eq!(
        labels,
        vec!["Icons".to_string()],
        "typing did not filter the rows"
    );
    click_row(&mut harness, "Icons");
    harness.run_steps(2);
    assert_eq!(harness.state().settings.page, Page::Icons);
    assert!(
        harness.state().palette.is_none(),
        "a row that acted did not close the palette"
    );

    // A preset row, found by its key, is named by its display name and installs the preset; a
    // mode row sets the mode it names (the gpui showcase's palette, parity item 19).
    let (other, other_name) = native_theme::theme::Theme::list_presets_for_platform()
        .into_iter()
        .map(|info| (info.key, info.display_name))
        .find(|(key, _)| *key != TEST_PRESET)
        .ok_or("no second preset is offered on this platform")?;
    for (typed, row) in [(other, other_name), ("Dark", "Dark")] {
        harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::K);
        harness.run_steps(2);
        harness
            .query_all_by_role(Role::TextInput)
            .find(|n| n.is_focused())
            .expect("focus")
            .type_text(typed);
        harness.run_steps(2);
        click_row(&mut harness, row);
        harness.run_steps(2);
        assert!(
            harness.state().palette.is_none(),
            "{row} did not close the palette"
        );
    }
    assert_eq!(
        harness.state().settings.theme,
        ThemeChoice::Preset(other.to_string())
    );
    assert_eq!(
        harness.state().atlas.name(),
        preset_name(other)?,
        "the preset row did not install it"
    );
    assert_eq!(harness.state().settings.mode, ModeChoice::Dark);
    assert_eq!(harness.ctx.theme(), egui::Theme::Dark);

    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::K);
    harness.run_steps(2);
    harness
        .query_all_by_role(Role::TextInput)
        .find(|n| n.is_focused())
        .expect("focus")
        .type_text("x");
    harness.run_steps(2);
    harness.key_press(egui::Key::Escape);
    harness.run_steps(2);
    assert_eq!(
        harness.state().palette.as_ref().map(|p| p.query.as_str()),
        Some(""),
        "the first Escape clears"
    );
    harness.key_press(egui::Key::Escape);
    harness.run_steps(2);
    assert!(
        harness.state().palette.is_none(),
        "the second Escape closes"
    );
    Ok(())
}

/// The palette's rows in order, each with its node's toggled state.
fn palette_rows(harness: &Harness<'_, App>) -> Vec<(String, Option<egui::accesskit::Toggled>)> {
    let ids: Vec<egui::accesskit::NodeId> = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "palette row")
        .map(|r| r.id.accesskit_id())
        .collect();
    ids.into_iter()
        .filter_map(|id| {
            harness
                .query_all(By::new().predicate(move |n| n.locate().0 == id))
                .next()
                .and_then(|n| {
                    let toggled = n.accesskit_node().toggled();
                    n.accesskit_node().label().map(|l| (l, toggled))
                })
        })
        .collect()
}
/// The highlighted rows' places, every other row's node checked to be `Toggled::False`.
fn palette_selected(harness: &Harness<'_, App>) -> Vec<usize> {
    use egui::accesskit::Toggled;
    let rows = palette_rows(harness);
    for (label, toggled) in &rows {
        assert!(
            matches!(toggled, Some(Toggled::True | Toggled::False)),
            "the row {label} is {toggled:?}, neither selected nor unselected"
        );
    }
    rows.iter()
        .enumerate()
        .filter(|(_, (_, toggled))| *toggled == Some(Toggled::True))
        .map(|(i, _)| i)
        .collect()
}
/// Whether the palette's `i`th row is in view: the registry records a row's `interact_rect`, its
/// rect clipped to the list's viewport (`egui/src/response.rs:36-43`), empty for a row scrolled
/// out.
fn palette_row_in_view(harness: &Harness<'_, App>, i: usize) -> bool {
    harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "palette row")
        .nth(i)
        .is_some_and(|r| r.rect.is_positive())
}
/// Ctrl+K: the palette opens with its field focused.
fn open_palette(harness: &mut Harness<'_, App>) {
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::K);
    harness.run_steps(SETTLE);
    assert!(harness.state().palette.is_some());
    assert!(
        harness
            .query_all_by_role(Role::TextInput)
            .any(|n| n.is_focused()),
        "the palette's field has no focus"
    );
}

/// The palette's keys, as gpui-component 0.9.1's `Command` binds them
/// (`src/command/state.rs:65-68`): the first row is highlighted when the palette opens and again
/// whenever the query changes (`reset_selection`, `:441-450`), Up and Down move the highlight,
/// wrapping at either end (`select_by`, `:553-579`), and Enter runs the highlighted row
/// (`on_action_confirm`, `:594-598`). The highlighted row's AccessKit node is the one selected:
/// a `selectable_label` drawn selected is `Toggled::True`, the others `Toggled::False`
/// (`egui/src/response.rs:975-980`).
#[test]
fn the_palettes_keys_highlight_and_run_a_row() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);

    // Typing filters to one row, which Enter runs.
    open_palette(&mut harness);
    harness
        .query_all_by_role(Role::TextInput)
        .find(|n| n.is_focused())
        .expect("the palette's field has focus")
        .type_text("Ico");
    harness.run_steps(SETTLE);
    let labels: Vec<String> = palette_rows(&harness).into_iter().map(|(l, _)| l).collect();
    assert_eq!(labels, ["Icons"], "typing did not filter the rows");
    harness.key_press(egui::Key::Enter);
    harness.run_steps(2);
    assert_eq!(harness.state().settings.page, Page::Icons);
    assert!(
        harness.state().palette.is_none(),
        "Enter on a row did not close the palette"
    );

    // Down twice highlights the third row, which Enter runs.
    open_palette(&mut harness);
    assert_eq!(
        palette_selected(&harness),
        [0],
        "the first row is not highlighted"
    );
    let third = palette_rows(&harness)
        .get(2)
        .map(|(label, _)| label.clone())
        .expect("the palette lists three rows");
    assert_eq!(third, Page::ALL[2].label(), "the third row is not a page");
    assert_ne!(Page::ALL[2], harness.state().settings.page);
    for _ in 0..2 {
        harness.key_press(egui::Key::ArrowDown);
        harness.run_steps(2);
    }
    assert_eq!(
        palette_selected(&harness),
        [2],
        "Down twice did not reach the third row"
    );
    assert!(
        harness
            .query_all_by_role(Role::TextInput)
            .any(|n| n.is_focused()),
        "the arrow keys took the focus from the field"
    );
    harness.key_press(egui::Key::Enter);
    harness.run_steps(2);
    assert_eq!(harness.state().settings.page, Page::ALL[2]);
    assert!(
        harness.state().palette.is_none(),
        "Enter on the third row did not close the palette"
    );

    // Up from the first row wraps to the last; a changed query highlights the first again.
    open_palette(&mut harness);
    harness.key_press(egui::Key::ArrowUp);
    harness.run_steps(2);
    let last = palette_rows(&harness).len().checked_sub(1).expect("rows");
    assert_eq!(
        palette_selected(&harness),
        [last],
        "Up did not wrap to the last row"
    );
    harness
        .query_all_by_role(Role::TextInput)
        .find(|n| n.is_focused())
        .expect("the palette's field keeps the focus")
        .type_text("a");
    harness.run_steps(2);
    assert_eq!(
        palette_selected(&harness),
        [0],
        "a changed query left the highlight where it was"
    );
}

/// The palette's highlight as gpui-component 0.9.1's `Command` moves it: Down from the last row
/// and Up from the first wrap (`select_by`, `src/command/state.rs:553-579`), presses in one pass
/// add up, a changed query starts again from the first row (`reset_selection`, `:441-450`), and
/// with no row there is none.
#[test]
fn the_palette_highlight_wraps_resets_and_needs_a_row() {
    use crate::chrome::palette_highlight;
    // (selected, query changed, Down presses, Up presses, rows)
    assert_eq!(
        palette_highlight(4, false, 1, 0, 5),
        Some(0),
        "Down from the last"
    );
    assert_eq!(
        palette_highlight(0, false, 0, 1, 5),
        Some(4),
        "Up from the first"
    );
    assert_eq!(
        palette_highlight(1, false, 7, 2, 5),
        Some(1),
        "presses in one pass"
    );
    assert_eq!(
        palette_highlight(3, true, 0, 0, 5),
        Some(0),
        "a changed query"
    );
    assert_eq!(
        palette_highlight(3, true, 2, 0, 5),
        Some(2),
        "Down after a change"
    );
    assert_eq!(
        palette_highlight(7, false, 0, 0, 5),
        Some(2),
        "a place past the rows"
    );
    assert_eq!(palette_highlight(2, false, 1, 0, 0), None, "no row");
}

/// A changed query highlights the first row it keeps and scrolls the list to it, as
/// gpui-component 0.9.1's `reset_selection` scrolls to it (`src/command/state.rs:441-450`, its
/// `pending_scroll`). At a text-scaling factor of 2.25 the rows an "a" keeps (six pages, the modes
/// that name "Dark", and the presets of this platform whose name or key holds one) are more than the list shows,
/// so the list stays scrolled where the query changes; a space keeps this platform's presets,
/// "Theme Map" and "System (…)" alone, which CI's runner, with fewer presets, fits in the list.
#[test]
fn a_changed_query_scrolls_the_palette_to_its_first_row() {
    let mut harness = open_scaled(TEST_PRESET, "basic", 2.25);
    open_palette(&mut harness);
    harness.key_press(egui::Key::ArrowUp);
    harness.run_steps(SETTLE);
    assert!(
        !palette_row_in_view(&harness, 0),
        "Up to the last row did not scroll the first out of view, so this test shows nothing"
    );
    harness
        .query_all_by_role(Role::TextInput)
        .find(|n| n.is_focused())
        .expect("the palette's field keeps the focus")
        .type_text("a");
    harness.run_steps(SETTLE);
    let last = palette_rows(&harness).len().checked_sub(1).expect("rows");
    assert!(
        (0..=last).any(|i| !palette_row_in_view(&harness, i)),
        "the rows an \"a\" keeps fit the list, so this test shows nothing"
    );
    assert_eq!(palette_selected(&harness), [0]);
    assert!(
        palette_row_in_view(&harness, 0),
        "a changed query left the highlighted first row scrolled out of view"
    );
}

use std::collections::BTreeSet;
use std::path::Path;

use egui_kittest::kittest::By;
use native_theme_egui::{RoleVariant, Surface};

use crate::{
    INFO_SETTLE,
    demo::Seam,
    info::{Manifest, Verdict},
};

const MANIFEST: &str = include_str!("../../mapping.toml");

/// Three default steps of a quarter second each pass `INFO_SETTLE`
/// (`egui_kittest/src/builder.rs:40`).
fn hover_and_settle(harness: &mut Harness<'_, App>, pos: egui::Pos2) {
    harness.hover_at(pos);
    harness.run_steps(3);
    assert_no_id_twice(harness);
}

fn assert_no_id_twice(harness: &Harness<'_, App>) {
    let twice = &harness.state().registry.recorded_twice;
    assert!(
        twice.is_empty(),
        "ids recorded twice in one pass: {twice:?}"
    );
}

fn shown_id(harness: &Harness<'_, App>) -> Option<egui::Id> {
    harness.state().registry.shown().map(|s| s.id)
}

fn shown_kind(harness: &Harness<'_, App>) -> Option<&'static str> {
    harness.state().registry.shown().map(|s| s.info.kind)
}

/// The centre of the first record whose kind is `kind`, in global coordinates.
fn centre_of(harness: &Harness<'_, App>, kind: &str) -> egui::Pos2 {
    let record = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == kind)
        .unwrap_or_else(|| panic!("no record of kind {kind}"));
    let rect = harness
        .ctx
        .layer_transform_to_global(record.layer)
        .map_or(record.rect, |t| t.mul_rect(record.rect));
    rect.center()
}

/// `run_steps`, not `run`: the Range page repaints every pass (§13 T11).
fn open_page(page: Page, theme: egui::Theme) -> Harness<'static, App> {
    let mut harness = open(
        theme,
        cli(&[("--theme", TEST_PRESET), ("--tab", page.key())]),
    );
    harness.run_steps(4);
    harness
}

/// A platform preset that states what the community presets of the 0.7.2 model left unstated
/// (no `button.border.padding`, `dialog.border.padding` or `list.header_height`,
/// `native-theme/src/presets/ios-13.toml`), for the tests of what the showcase draws in their
/// place. A preset of another platform but iOS, so [`open_page_under`] installs it.
const UNSTATED_PRESET: &str = "ios-13";

/// [`open_page`], then `preset` installed through the Theme row's path (`App::install`), on any
/// platform: `--theme` takes this platform's presets alone (`cli::next_theme_key`). The mode is
/// `theme`'s first, as the side panel's mode row sets it: an install sets the mode's theme
/// preference (`App::apply`), and under `System` egui draws the scheme the integration reports,
/// which the harness reports none of, so egui's fallback, dark (`egui::Options::fallback_theme`).
fn open_page_under(page: Page, theme: egui::Theme, preset: &str) -> Harness<'static, App> {
    open_page_under_at(page, theme, preset, None)
}

/// [`open_page_under`] in light at a text-scaling factor of `factor`, the Preferences dialog's
/// override over the preferences the showcase opened with (`Settings::prefs`), so both models'
/// themes of the choice are built at it (`App::rebuild_atlas`).
fn open_page_scaled(page: Page, preset: &str, factor: f32) -> Harness<'static, App> {
    open_page_under_at(page, egui::Theme::Light, preset, Some(factor))
}

fn open_page_under_at(
    page: Page,
    theme: egui::Theme,
    preset: &str,
    factor: Option<f32>,
) -> Harness<'static, App> {
    let mut harness = open_page(page, theme);
    if let Some(factor) = factor {
        let mut prefs = harness.state().atlas.asked().clone();
        prefs.text_scaling_factor = factor;
        harness.state_mut().settings.prefs = Some(prefs);
    }
    install_under(&mut harness, theme, preset);
    harness
}

/// `preset` installed into `harness` through the Theme row's path, in `theme`'s mode, as
/// [`open_page_under`] installs it; the installed atlas is named as the unpatched preset
/// resolves (`preset_name`), so a test checks the theme it runs under.
fn install_under(harness: &mut Harness<'static, App>, theme: egui::Theme, preset: &str) {
    let ctx = harness.ctx.clone();
    let mode = match theme {
        egui::Theme::Light => ModeChoice::Light,
        egui::Theme::Dark => ModeChoice::Dark,
    };
    harness.state_mut().set_mode(mode, &ctx);
    harness.state_mut().settings.theme = ThemeChoice::Preset(preset.to_owned());
    harness.state_mut().install(&ctx);
    harness.run_steps(4);
    assert_eq!(harness.state().theme_error, None, "{preset} installs");
    assert_eq!(harness.ctx.theme(), theme, "{preset} is drawn in {theme:?}");
    assert_eq!(
        Ok(harness.state().atlas.name()),
        preset_name(preset).as_deref(),
        "the installed atlas is {preset}'s"
    );
}

/// [`open_page_under`], then the patch `patch` makes of the installed 0.8.0 theme of `theme`'s
/// slot applied onto the four 0.8.0 themes of the showcase's atlas and installed as an install
/// keeps it (`OldAtlas::patched`, `App::adopt_pair`), as the gpui showcase's
/// `install_patched_breeze`: a test can make the installed values differ from the unpatched
/// preset's and so prove what it reads. The patched theme of `theme`'s
/// slot comes back with the harness; `patch` gives `None` where `preset` states nothing the
/// patch is made of.
fn open_patched(
    page: Page,
    theme: egui::Theme,
    preset: &str,
    patch: impl FnOnce(&native_theme::next::ResolvedTheme) -> Option<native_theme::next::ThemePatch>,
) -> Result<(Harness<'static, App>, native_theme::next::ResolvedTheme), String> {
    let mut harness = open_page_under(page, theme, preset);
    let atlas = harness.state().atlas.clone();
    let r = atlas
        .next_resolved(theme)
        .ok_or_else(|| format!("{preset}: no 0.8.0 theme installed"))?;
    let patch = patch(r).ok_or_else(|| format!("{preset} states nothing the patch is made of"))?;
    let patched = atlas.patched(&patch)?;
    let ctx = harness.ctx.clone();
    harness.state_mut().adopt_pair(&ctx, patched);
    harness.run_steps(4);
    let r = harness
        .state()
        .atlas
        .next_resolved(harness.ctx.theme())
        .ok_or("the patched atlas holds no 0.8.0 theme")?
        .clone();
    Ok((harness, r))
}

/// The AccessKit roles egui gives a widget: from egui 0.37.0 on a widget names its `Role` itself
/// in its `WidgetInfo`, and `WidgetType`, which egui 0.36.2 mapped to a role, is that `Role`
/// under its old name, the variants egui renamed tabled at `egui-0.37.0/src/lib.rs:642-659`
/// (`Label`, `Link`, `TextInput`, `Button`, `Image`, `CheckBox`, `RadioButton`, `ComboBox`,
/// `Slider`, `SpinButton`, `ColorWell`, `ProgressIndicator`, `Splitter`, `ScrollBar`, and
/// `DisclosureTriangle`, which a `CollapsingHeader` reports,
/// `egui-0.37.0/src/containers/collapsing_header.rs:589`, where 0.36.2 gave it `Button`), less
/// `Pane`, `Window`, `RadioGroup` and `Unknown`.
const WIDGET_ROLES: &[Role] = &[
    Role::Label,
    Role::Link,
    Role::TextInput,
    Role::Button,
    Role::Image,
    Role::CheckBox,
    Role::RadioButton,
    Role::ComboBox,
    Role::Slider,
    Role::SpinButton,
    Role::ColorWell,
    Role::ProgressIndicator,
    Role::Splitter,
    Role::ScrollBar,
    Role::DisclosureTriangle,
];

/// A test per page of [`Page::ALL`], named after the page, each `$run(page, $arg...)`, so that
/// nextest runs the pages in parallel. The `match` names every page: a page added to [`Page`]
/// and not here fails to compile.
macro_rules! per_page {
    ($run:ident $(, $arg:expr)*) => {
        const _: fn(Page) = |page| match page {
            Page::Basic
            | Page::Buttons
            | Page::Inputs
            | Page::Selection
            | Page::Range
            | Page::Data
            | Page::Text
            | Page::Containers
            | Page::Overlays
            | Page::Colour
            | Page::Icons
            | Page::ThemeMap => {}
        };
        #[test]
        fn basic() -> Result<(), String> {
            $run(Page::Basic $(, $arg)*)
        }
        #[test]
        fn buttons() -> Result<(), String> {
            $run(Page::Buttons $(, $arg)*)
        }
        #[test]
        fn inputs() -> Result<(), String> {
            $run(Page::Inputs $(, $arg)*)
        }
        #[test]
        fn selection() -> Result<(), String> {
            $run(Page::Selection $(, $arg)*)
        }
        #[test]
        fn range() -> Result<(), String> {
            $run(Page::Range $(, $arg)*)
        }
        #[test]
        fn data() -> Result<(), String> {
            $run(Page::Data $(, $arg)*)
        }
        #[test]
        fn text() -> Result<(), String> {
            $run(Page::Text $(, $arg)*)
        }
        #[test]
        fn containers() -> Result<(), String> {
            $run(Page::Containers $(, $arg)*)
        }
        #[test]
        fn overlays() -> Result<(), String> {
            $run(Page::Overlays $(, $arg)*)
        }
        #[test]
        fn colour() -> Result<(), String> {
            $run(Page::Colour $(, $arg)*)
        }
        #[test]
        fn icons() -> Result<(), String> {
            $run(Page::Icons $(, $arg)*)
        }
        #[test]
        fn theme_map() -> Result<(), String> {
            $run(Page::ThemeMap $(, $arg)*)
        }
    };
}

/// A test per preset of the 0.8.0 model ([`native_theme::next::Preset::ALL`]), named after the
/// preset, each `$run(preset, $arg…)`, so that nextest runs the presets in parallel, and a test
/// that the tests name every preset.
macro_rules! per_preset {
    ($run:ident $(, $arg:expr)*) => {
        const PRESETS: &[native_theme::next::Preset] = &[
            native_theme::next::Preset::Adwaita,
            native_theme::next::Preset::KdeBreeze,
            native_theme::next::Preset::MacosMonterey,
            native_theme::next::Preset::MacosTahoe,
            native_theme::next::Preset::Windows11,
            native_theme::next::Preset::Material3,
            native_theme::next::Preset::Ios13,
        ];
        #[test]
        fn every_preset_has_its_test() {
            assert_eq!(PRESETS, native_theme::next::Preset::ALL);
        }
        #[test]
        fn adwaita() -> Result<(), String> {
            $run(native_theme::next::Preset::Adwaita $(, $arg)*)
        }
        #[test]
        fn kde_breeze() -> Result<(), String> {
            $run(native_theme::next::Preset::KdeBreeze $(, $arg)*)
        }
        #[test]
        fn macos_monterey() -> Result<(), String> {
            $run(native_theme::next::Preset::MacosMonterey $(, $arg)*)
        }
        #[test]
        fn macos_tahoe() -> Result<(), String> {
            $run(native_theme::next::Preset::MacosTahoe $(, $arg)*)
        }
        #[test]
        fn windows_11() -> Result<(), String> {
            $run(native_theme::next::Preset::Windows11 $(, $arg)*)
        }
        #[test]
        fn material_3() -> Result<(), String> {
            $run(native_theme::next::Preset::Material3 $(, $arg)*)
        }
        #[test]
        fn ios_13() -> Result<(), String> {
            $run(native_theme::next::Preset::Ios13 $(, $arg)*)
        }
    };
}

/// [`every_widget_reports_itself_on`] each page under the test preset in light and in dark: one
/// test per mode and page.
mod every_widget_reports_itself {
    mod light {
        use super::super::*;
        per_page!(every_widget_reports_itself_on, egui::Theme::Light);
    }
    mod dark {
        use super::super::*;
        per_page!(every_widget_reports_itself_on, egui::Theme::Dark);
    }
}

fn every_widget_reports_itself_on(page: Page, theme: egui::Theme) -> Result<(), String> {
    let mut harness = open_page_under(page, theme, TEST_PRESET);
    let hold = harness
        .state()
        .hold_zone
        .ok_or("the inspector reports its content rect")?;
    // egui's own node id (`Id::accesskit_id`, `egui/src/id.rs:103`): accesskit_consumer's
    // `Node::id` is a type of its own, `locate().0` the id egui gave the node.
    let ids: Vec<egui::accesskit::NodeId> = harness
        .query_all(By::new().predicate(|n| WIDGET_ROLES.contains(&n.role())))
        .filter(|n| !hold.intersects(n.rect()))
        .map(|n| n.accesskit_node().locate().0)
        .collect();
    assert!(!ids.is_empty(), "{page:?} has no widget node");
    for id in ids {
        let Some(node) = harness
            .query_all(By::new().predicate(move |n| n.locate().0 == id))
            .next()
        else {
            continue;
        };
        node.scroll_to_me();
        // The scroll lands on the third pass after the request (measured on the Text page), two
        // passes later in a scroll area nested in the page's that lay outside its view, which
        // `demo::nested_scroll` hands the request again `demo::REPLAY_PASSES` passes on.
        harness.run_steps(3 + usize::from(crate::demo::REPLAY_PASSES));
        // A node egui creates itself, whose id no helper sees (§13.2): a scroll bar
        // (`egui-0.37.0/src/containers/scroll_area.rs:1473`), a window's resize handle or a
        // panel's (`Role::Splitter`: `egui-0.37.0/src/containers/window.rs:1087`,
        // `containers/panel.rs:1202`), or a node inside a `Window`'s AccessKit node — its
        // title bar's collapse and close buttons (`window.rs:1396-1410`, `:1488-1491`).
        let Some((rect, egui_own)) = harness
            .query_all(By::new().predicate(move |n| n.locate().0 == id))
            .next()
            .map(|n| {
                let node = n.accesskit_node();
                let mut in_window = false;
                let mut parent = node.parent();
                while let Some(p) = parent {
                    // The tree's root is the viewport's own `Window` node: not an egui
                    // `Window`.
                    in_window |= p.role() == Role::Window && p.parent().is_some();
                    parent = p.parent();
                }
                let own = matches!(node.role(), Role::ScrollBar | Role::Splitter) || in_window;
                (n.rect(), own)
            })
        else {
            continue;
        };
        // `Node::rect` is the untransformed rect (`egui/src/response.rs:912-917`): a node
        // in a transformed layer — the `Scene`'s — is hovered through its layer's
        // transform, as `centre_of` does, where a record gives the layer.
        let layer = harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.id.accesskit_id() == id)
            .map(|r| r.layer);
        let global = layer
            .and_then(|l| harness.ctx.layer_transform_to_global(l))
            .map_or(rect, |t| t.mul_rect(rect));
        hover_and_settle(&mut harness, global.center());
        let app = harness.state();
        let records = app.registry.records();
        // A node only a layout box holds — the page's own scroll bar in the central
        // panel — is no Widget Info target (R11): hovering it keeps what is shown.
        if !records
            .iter()
            .any(|r| r.target && r.contains_pointer && r.rect.contains(global.center()))
        {
            continue;
        }
        let shown = app
            .registry
            .shown()
            .ok_or_else(|| format!("{page:?} {theme:?}: nothing shown for node {id:?}"))?;
        let node_recorded = records.iter().any(|r| r.id.accesskit_id() == id);
        let own = shown.id.accesskit_id() == id;
        // A recorded node scrolled to is shown its whole height, which a hover at its centre
        // alone does not prove: a row a nested scroll area left 2 points in view, its centre
        // over the area above it. Not its whole width: a row wider than its area, the
        // Containers page's "ScrollArea (both)", reaches past it on the right.
        for r in records.iter().filter(|r| r.id.accesskit_id() == id) {
            let shown_rows = r.rect.y_range().expand(0.5);
            assert!(
                shown_rows.contains(rect.top()) && shown_rows.contains(rect.bottom()),
                "{page:?} {theme:?}: node {id:?} at {rect:?} is shown at {:?} alone",
                r.rect
            );
        }
        // Else the info of a non-container widget whose rect contains the node's; for a
        // node egui creates itself (and no helper records), the innermost record's,
        // container or not (§13.2), which holds the node's centre where the node reaches
        // past it, as a panel's resize handle (`Role::Splitter`) straddles its panel's
        // edge. A record is clipped to what is shown and the node is
        // not: the last node of a page scrolled to its end may reach a fraction of a
        // pixel past the page's clip, so the rects compare within half a pixel.
        let by_record = records.iter().any(|r| {
            r.id == shown.id
                && (r.rect.expand(0.5).contains_rect(rect)
                    || (egui_own && !node_recorded && r.rect.contains(global.center())))
                && (!r.container || (egui_own && !node_recorded))
        });
        assert!(
            own || by_record,
            "{page:?} {theme:?}: node {id:?} at {rect:?} shows {} ({:?})",
            shown.info.kind,
            shown.id
        );
    }
    Ok(())
}

/// Every input on every page has an accessible name (`Harness::unnamed_widgets`,
/// `egui_kittest-0.37.0/src/accessibility.rs`). kittest checks it itself only where a run settles
/// (`Harness::try_run`, `egui_kittest-0.37.0/src/lib.rs`), which a visible spinner keeps the
/// Range page from doing, so each page is checked here once it is open.
#[test]
fn every_page_names_its_inputs() {
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        for page in Page::ALL {
            let harness = open_page(page, theme);
            let unnamed = harness.unnamed_widgets();
            assert!(
                unnamed.is_empty(),
                "{page:?} {theme:?}: inputs with no accessible name: {unnamed:#?}"
            );
        }
    }
}

/// Widget Info states what egui draws where a `mapping.toml` row records it in its `note` (E5,
/// E7): under TEST_PRESET, which states no `button.border.padding` and a check box radius above
/// egui's cap, a button's padding and least-height rows and a checked box's radius row carry
/// their row's note after the route: under [`UNSTATED_PRESET`], which states no
/// `button.border.padding`, and whose check box radius is its `defaults.border.corner_radius`.
#[test]
fn widget_info_carries_the_mapping_notes() -> Result<(), String> {
    let note = |manifest: &Manifest, leaf: &str| {
        manifest
            .rows
            .iter()
            .find(|r| r.leaf == leaf)
            .and_then(|r| r.note.clone())
            .unwrap_or_else(|| panic!("mapping.toml's {leaf} row has no note"))
    };
    let elements = crate::elements::showcase_elements().unwrap_or_default();
    for (page, kind, leaves) in [
        (
            Page::Buttons,
            "button (enabled)",
            &["button.border.padding.left", "button.min_height"][..],
        ),
        (
            Page::Selection,
            "checkbox (checked)",
            &["checkbox.border.corner_radius", "checkbox.indicator_width"][..],
        ),
    ] {
        let mut harness = open_page_under(page, egui::Theme::Light, UNSTATED_PRESET);
        let pos = centre_of(&harness, kind);
        hover_and_settle(&mut harness, pos);
        let shown = harness.state().registry.shown().cloned().expect("shown");
        assert_eq!(shown.info.kind, kind);
        let (manifest, json) = manifest_and_json(&harness)?;
        let [_, right, _, left] = native_theme_egui::next::padding_of(
            &json
                .theme
                .button
                .body(native_theme::next::WidgetState::NONE),
        );
        assert!(
            left.is_none() && right.is_none(),
            "{UNSTATED_PRESET} states a button padding across, so the padding note shows no \
             default"
        );
        let view = crate::info::InfoView::of(
            &shown,
            &elements,
            &manifest,
            (&json, UNSTATED_PRESET),
            (&harness.state().atlas, &harness.ctx),
        );
        for leaf in leaves {
            // The leaf's row by its home, the first its mapping.toml row holds.
            let home = manifest
                .rows
                .iter()
                .find(|r| r.leaf == *leaf)
                .and_then(|r| r.homes.first())
                .ok_or(format!("mapping.toml's {leaf} row has no home"))?;
            let row = view
                .rows
                .iter()
                .find(|r| r.leaf == *home)
                .ok_or(format!("{kind}: no {home} row"))?;
            let note = note(&manifest, leaf);
            assert!(
                row.how.ends_with(&format!("; {note}")),
                "{kind}: {home}'s route {:?} does not end in its note {note:?}",
                row.how
            );
        }
    }
    Ok(())
}

/// E10: the Inputs page's `CompletionPopup` offers the bundled presets whose key begins with the
/// word typed into its field, and Enter takes the selected one.
#[test]
fn the_completion_popup_completes_a_presets_key() {
    let mut harness = open_page(Page::Inputs, egui::Theme::Light);
    harness
        .get_by_role_and_label(Role::TextInput, "CompletionPopup")
        .click();
    harness.run_steps(SETTLE);
    harness
        .get_by_role_and_label(Role::TextInput, "CompletionPopup")
        .type_text("adw");
    harness.run_steps(SETTLE);
    assert!(
        harness.query_by_label("adwaita").is_some(),
        "the popup offers no adwaita row"
    );
    harness.key_press(egui::Key::Enter);
    harness.run_steps(SETTLE);
    assert_eq!(harness.state().demo_state.completion, "adwaita");
}

#[test]
fn no_id_is_recorded_twice() {
    for page in Page::ALL {
        let harness = open_page(page, egui::Theme::Light);
        assert_no_id_twice(&harness);
    }
}

#[test]
fn the_innermost_hovered_target_wins() {
    let mut harness = open_page(Page::Overlays, egui::Theme::Light);
    harness.get_by_label("Open modal").click();
    harness.run_steps(SETTLE);
    let pos = centre_of(&harness, "modal button");
    hover_and_settle(&mut harness, pos);
    assert_eq!(shown_kind(&harness), Some("modal button"));
    let frame = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "modal")
        .map(|r| r.rect)
        .expect("modal record");
    hover_and_settle(&mut harness, frame.left_top() + egui::vec2(2.0, 2.0));
    assert!(
        harness
            .state()
            .registry
            .shown()
            .is_some_and(|s| s.info.seams.contains(&Seam::Surface(Surface::Dialog))),
        "beside the button the dialog surface wins"
    );
    harness.key_press(egui::Key::Escape);
    harness.run_steps(SETTLE);

    let mut harness = open_page(Page::Containers, egui::Theme::Light);
    let pos = centre_of(&harness, "card checkbox");
    hover_and_settle(&mut harness, pos);
    assert_eq!(shown_kind(&harness), Some("card checkbox"));
    // The Scene lies below the 720-point window as the page opens (its button's centre measured
    // at y 937.5 under TEST_PRESET), where no pointer reaches it: `ui.columns`' first column,
    // under the Scene, is scrolled into view, which brings the Scene into the window with it.
    harness.get_by_label("ui.columns: column 0").scroll_to_me();
    harness.run_steps(3);
    let pos = centre_of(&harness, "scene button");
    hover_and_settle(&mut harness, pos);
    assert_eq!(
        shown_kind(&harness),
        Some("scene button"),
        "a widget inside the Scene, hovered through its layer transform"
    );
}

#[test]
fn instances_are_distinct() {
    let mut harness = open_page(Page::Selection, egui::Theme::Light);
    let pos = centre_of(&harness, "checkbox (unchecked)");
    hover_and_settle(&mut harness, pos);
    let unchecked = harness.state().registry.shown().cloned().expect("shown");
    let pos = centre_of(&harness, "checkbox (checked)");
    hover_and_settle(&mut harness, pos);
    let checked = harness.state().registry.shown().cloned().expect("shown");
    assert!(unchecked.info.seams.contains(&Seam::Role(
        native_theme_egui::Role::Checkbox,
        RoleVariant::Normal
    )));
    assert!(checked.info.seams.contains(&Seam::Role(
        native_theme_egui::Role::Checkbox,
        RoleVariant::Selected
    )));
    let read = manifest_and_json(&harness);
    assert!(read.is_ok(), "{read:?}");
    let Ok((manifest, json)) = read else { return };
    let elements = crate::elements::showcase_elements().unwrap_or_default();
    assert!(!elements.is_empty(), "the element list parses");
    let scheme = (&harness.state().atlas, &harness.ctx);
    let text = |shown| {
        crate::info::InfoView::of(shown, &elements, &manifest, (&json, TEST_PRESET), scheme).text()
    };
    let (a, b) = (text(&unchecked), text(&checked));
    assert_ne!(
        a, b,
        "the unchecked and checked infos list the same rows and values"
    );

    let mut harness = open_page(Page::Buttons, egui::Theme::Light);
    let pos = centre_of(&harness, "button (enabled)");
    hover_and_settle(&mut harness, pos);
    let enabled = harness.state().registry.shown().cloned().expect("shown");
    let pos = centre_of(&harness, "button (disabled)");
    hover_and_settle(&mut harness, pos);
    let disabled = harness.state().registry.shown().cloned().expect("shown");
    assert!(enabled.info.seams.contains(&Seam::Role(
        native_theme_egui::Role::Button,
        RoleVariant::Normal
    )));
    assert!(disabled.info.seams.contains(&Seam::Role(
        native_theme_egui::Role::Button,
        RoleVariant::Disabled
    )));
}

/// §10.4: the Overlays page's context-menu target is a label drawn in the base style, so its
/// info names the base style alone and lists no row the `Role::Menu` seam adds; the menu it
/// opens is the instance drawn in `Role::Menu`, and its record says so (M9-109).
#[test]
fn the_context_menus_target_is_the_base_style_and_its_menu_the_menu_role() {
    const TARGET: &str = "Right-click here for a context menu";
    let menu = Seam::Role(native_theme_egui::Role::Menu, RoleVariant::Normal);
    let mut harness = open_page(Page::Overlays, egui::Theme::Light);
    let pos = harness.get_by_label(TARGET).rect().center();
    hover_and_settle(&mut harness, pos);
    let shown = harness
        .state()
        .registry
        .shown()
        .cloned()
        .expect("hovering the target shows its info");
    assert_eq!(
        shown.info.seams,
        vec![Seam::Base],
        "{}: the target is drawn in the base style",
        shown.info.kind
    );
    let read = manifest_and_json(&harness);
    assert!(read.is_ok(), "{read:?}");
    let Ok((manifest, json)) = read else { return };
    // By home, as Widget Info lists an unlisted instance's rows.
    let base: BTreeSet<&str> = manifest
        .rows_for(&Seam::Base)
        .into_iter()
        .flat_map(|r| r.homes_on(TEST_PRESET).iter().map(String::as_str))
        .collect();
    let menu_only: BTreeSet<&str> = manifest
        .rows_for(&menu)
        .into_iter()
        .flat_map(|r| r.homes_on(TEST_PRESET).iter().map(String::as_str))
        .filter(|home| !base.contains(home))
        .collect();
    assert!(!menu_only.is_empty(), "Role::Menu adds no row to the base");
    let elements = listed_elements();
    let scheme = (&harness.state().atlas, &harness.ctx);
    let view =
        crate::info::InfoView::of(&shown, &elements, &manifest, (&json, TEST_PRESET), scheme);
    let menu_rows: Vec<&str> = view
        .rows
        .iter()
        .map(|r| r.leaf.as_str())
        .filter(|leaf| menu_only.contains(leaf))
        .collect();
    assert!(
        menu_rows.is_empty(),
        "the base-style target lists Role::Menu rows: {menu_rows:?}"
    );

    harness.get_by_label(TARGET).click_secondary();
    harness.run_steps(2);
    let open: Vec<&Vec<Seam>> = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "context menu (open)")
        .map(|r| &r.info.seams)
        .collect();
    assert!(
        !open.is_empty() && open.iter().all(|seams| seams.contains(&menu)),
        "the open context menu's records: {open:?}"
    );
}

/// The fill and the label colour egui painted the Basic page's button of kind `kind`, lettered
/// `text`, in this pass: the `Shape::Rect` at its record's place, and its label's galley.
fn button_paint(
    harness: &Harness<'_, App>,
    kind: &str,
    text: &str,
    id: &str,
) -> Result<(egui::Color32, egui::Color32), String> {
    let rect = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == kind)
        .map(|r| r.rect)
        .ok_or_else(|| format!("no {kind} record"))?;
    let fill = harness
        .output()
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            egui::Shape::Rect(r) if r.rect == rect => Some(r.fill),
            _ => None,
        })
        .ok_or_else(|| format!("no frame painted at the {kind}'s place {rect:?}"))?;
    let label = painted_label_colour(harness, text, id)
        .ok_or_else(|| format!("no label {text:?} painted in {id}"))?;
    Ok((fill, label))
}

/// The Basic page's toggle's colours while on under `r` (`pages::basic::toggle_on_at`, asked
/// with the checked flag alone).
fn toggle_on(r: Option<&native_theme::next::ResolvedTheme>) -> crate::pages::basic::ToggleOn {
    use native_theme::next::{ButtonQuery, WidgetState};
    crate::pages::basic::toggle_on_at(r, ButtonQuery::new(WidgetState::CHECKED))
}

/// The first bundled preset whose 0.8.0 theme, light, states neither checked colour of the
/// toggle (`pages::basic::toggle_on_at`), with that theme.
fn unchecked_preset() -> Result<(&'static str, native_theme::next::ResolvedTheme), String> {
    use crate::pages::basic::ToggleOn;
    use native_theme::next::{Preset, Theme, ThemeContext};
    let none = ToggleOn {
        fill: None,
        text: None,
    };
    Preset::ALL
        .iter()
        .find_map(|preset| {
            let r = Theme::preset(*preset).resolve(&ThemeContext::light());
            (toggle_on(Some(&r)) == none).then_some((preset.as_str(), r))
        })
        .ok_or_else(|| "every preset offered here states a checked fill or label colour".to_owned())
}

/// Under a preset whose 0.8.0 theme states no checked colours (`button.body(CHECKED).fill`,
/// `button.label(CHECKED).color`), the Basic page's "On" toggle is egui's own selected button
/// (`pages::basic::toggle_on_at`, spec §10 rules 0 and 0a): painted in the fill and the label
/// colour of the suggested-action button beside it, egui's selected one too, not in another
/// state's colours, and its Widget Info names egui's own under *Not themeable* (M9-111).
#[test]
fn the_toggle_that_is_on_names_the_home_its_fill_is_drawn_in() -> Result<(), String> {
    use crate::pages::basic::ToggleOn;
    use native_theme::next::WidgetState;
    let (key, r) = unchecked_preset()?;
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, key);
    let installed = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .map(|r| toggle_on(Some(r)));
    assert_eq!(
        installed,
        Some(ToggleOn {
            fill: None,
            text: None
        }),
        "{key}: the installed theme's toggle"
    );
    let on = button_paint(
        &harness,
        "toggle button (on)",
        "On",
        "basic.buttons.toggle_on",
    )?;
    let primary = button_paint(
        &harness,
        "button (suggested action)",
        "Primary",
        "basic.buttons.primary",
    )?;
    assert_eq!(
        on, primary,
        "{key}: the toggle while on is the selected button"
    );
    let pressed = crate::demo::drawn_state_fill(&r.button.body(WidgetState::PRESSED));
    let hovered = crate::demo::drawn_state_fill(&r.button.body(WidgetState::HOVER));
    assert!(
        pressed != Some(on.0) && hovered != Some(on.0),
        "{key}: the toggle while on takes another state's fill: {on:?}"
    );
    let fill = ToggleOn::OWN_FILL;
    let text = ToggleOn::OWN_TEXT;
    let pos = centre_of(&harness, "toggle button (on)");
    hover_and_settle(&mut harness, pos);
    let shown = harness
        .state()
        .registry
        .shown()
        .cloned()
        .expect("hovering the toggle shows its info");
    assert_eq!(
        shown.info.element.as_deref(),
        Some("basic.buttons.toggle_on")
    );
    let lines: Vec<String> = shown
        .info
        .not_themeable
        .iter()
        .map(|(what, note)| format!("{what}: {note}"))
        .collect();
    let names = |unstated: &str, drawn: &str| {
        lines
            .iter()
            .any(|line| line.contains(unstated) && line.contains(drawn))
    };
    assert!(
        names("button.body(CHECKED).fill", fill),
        "{key}: no Not themeable line says the fill is {fill}: {lines:?}"
    );
    assert!(
        names("button.label(CHECKED).color", text),
        "{key}: no Not themeable line says the text colour is {text}: {lines:?}"
    );
    Ok(())
}

/// `toggle_on_at` asks each home with the checked flag alone and takes no other state's colours:
/// without a 0.8.0 theme, and under a theme stating no checked colour, nothing (egui's own
/// selected button); under a patched `button.body(CHECKED).fill` and `button.label(CHECKED)
/// .color`, magenta, on a preset stating no checked colour, the Basic page's "On" toggle is
/// painted in them and its Widget Info names no fallback; with the fill alone patched, the fill is the
/// patched one and the label egui's selected button's, the suggested-action button's beside it.
#[test]
fn the_toggle_takes_the_checked_colours_alone() -> Result<(), String> {
    use crate::pages::basic::ToggleOn;
    use native_theme::next::{ButtonRule, Paint, PartPatch, TextPatch, ThemePatch, WidgetState};
    assert_eq!(
        toggle_on(None),
        ToggleOn {
            fill: None,
            text: None
        }
    );
    let magenta = native_theme::next::Color::from_srgb8(255, 0, 255, 255);
    let want = egui::Color32::from_rgb(255, 0, 255);
    let both = |_: &native_theme::next::ResolvedTheme| {
        Some(
            ThemePatch::new().button(
                ButtonRule::new()
                    .states(WidgetState::CHECKED)
                    .body(PartPatch::new().fill(Paint::Solid(magenta)))
                    .label(TextPatch::new().color(magenta)),
            ),
        )
    };
    let (key, _) = unchecked_preset()?;
    let (harness, r) = open_patched(Page::Basic, egui::Theme::Light, key, both)?;
    assert_eq!(
        toggle_on(Some(&r)),
        ToggleOn {
            fill: Some(want),
            text: Some(want)
        }
    );
    let on = button_paint(
        &harness,
        "toggle button (on)",
        "On",
        "basic.buttons.toggle_on",
    )?;
    assert_eq!(on, (want, want), "the patched checked fill and label");
    let notes = element_notes(&harness, "basic.buttons.toggle_on");
    assert!(
        notes
            .iter()
            .flatten()
            .all(|line| !line.contains("(CHECKED)")),
        "a stated checked colour is named as unstated: {notes:?}"
    );

    let fill_only = |_: &native_theme::next::ResolvedTheme| {
        Some(
            ThemePatch::new().button(
                ButtonRule::new()
                    .states(WidgetState::CHECKED)
                    .body(PartPatch::new().fill(Paint::Solid(magenta))),
            ),
        )
    };
    let (harness, r) = open_patched(Page::Basic, egui::Theme::Light, key, fill_only)?;
    let stated = toggle_on(Some(&r));
    assert_eq!(stated.fill, Some(want), "the patched checked fill");
    let on = button_paint(
        &harness,
        "toggle button (on)",
        "On",
        "basic.buttons.toggle_on",
    )?;
    let primary = button_paint(
        &harness,
        "button (suggested action)",
        "Primary",
        "basic.buttons.primary",
    )?;
    assert_eq!(on.0, want, "the patched checked fill is painted");
    assert_eq!(
        stated.text, None,
        "{key}: a checked label colour beside the patched fill"
    );
    assert_eq!(
        on.1, primary.1,
        "an unstated checked label is egui's selected button's"
    );
    Ok(())
}

/// The mixed check box reads the mixed state from the 0.8.0 model (`next::mixed_checkbox`,
/// spec §2 `from_state`): on kde-breeze with a rule for `INDETERMINATE` stating the box's fill
/// and outline and the mark's colour, the box is painted in that fill and outline and its dash
/// in that colour, and its label keeps the unchecked box's colour.
#[test]
fn the_mixed_box_reads_the_mixed_state() -> Result<(), String> {
    let rule = "[[widgets.checkbox.rules]]\nstates = [\"indeterminate\"]\n\
                indicator.fill = \"#ff00ff\"\n\
                indicator.stroke = { width = 3.0, color = \"#00ff00\" }\n\
                checkmark.color = \"#0000ff\"\n";
    let (harness, r) = open_patched(Page::Selection, egui::Theme::Light, "kde-breeze", |_| {
        native_theme::next::ThemePatch::from_toml(rule).ok()
    })?;
    let [magenta, green, blue] = [[0xff, 0, 0xff], [0, 0xff, 0], [0, 0, 0xff]]
        .map(|[r, g, b]| egui::Color32::from_rgb(r, g, b));
    let mut visuals = egui::Visuals::light();
    let notes = native_theme_egui::next::mixed_checkbox(&r, egui::Theme::Light, &mut visuals);
    let rest = visuals.widgets.inactive;
    assert_eq!(
        (
            rest.bg_fill,
            rest.bg_stroke,
            rest.fg_stroke.color,
            notes.len()
        ),
        (magenta, egui::Stroke::new(3.0, green), blue, 0),
        "the mixed state's stated values"
    );
    let rect = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "checkbox (indeterminate)")
        .map(|r| r.rect)
        .ok_or("no mixed box recorded")?;
    let shapes = &harness.output().shapes;
    let boxed = shapes.iter().any(|c| {
        matches!(&c.shape, egui::Shape::Rect(s)
            if rect.intersects(s.rect) && s.fill == magenta
                && s.stroke == egui::Stroke::new(3.0, green))
    });
    let dash = shapes.iter().any(|c| {
        matches!(&c.shape, egui::Shape::LineSegment { points, stroke }
            if rect.contains(points[0]) && stroke.color == blue)
    });
    assert!(boxed, "the mixed box is not in the stated fill and outline");
    assert!(
        dash,
        "the mixed box's dash is not in the stated mark colour"
    );
    let label = shapes.iter().find_map(|c| match &c.shape {
        egui::Shape::Text(s) if s.galley.job.text == "Indeterminate" => {
            s.galley.job.sections.first().map(|s| s.format.color)
        }
        _ => None,
    });
    assert_ne!(label, Some(blue), "the label takes the mark's colour");
    Ok(())
}

/// A radio button and a check box take no selected flag of their own: the checked look is
/// `RoleVariant::Selected` (§4.4), picked per instance from its state; the mixed box is the
/// unchecked one with what the theme states for the mixed state (`next::mixed_checkbox`).
#[test]
fn a_checked_radio_or_box_is_drawn_selected() {
    let variant_of = |harness: &Harness<'_, App>, kind: &str| {
        let seams: Vec<Seam> = harness
            .state()
            .registry
            .records()
            .iter()
            .filter(|r| r.info.kind == kind)
            .flat_map(|r| r.info.seams.clone())
            .collect();
        match seams.as_slice() {
            [Seam::Role(native_theme_egui::Role::Checkbox, variant)] => *variant,
            other => panic!("{kind} recorded {other:?}"),
        }
    };
    let mut harness = open_page(Page::Selection, egui::Theme::Light);
    assert_eq!(
        variant_of(&harness, "checkbox (indeterminate)"),
        RoleVariant::Normal
    );
    for (kind, variant) in [
        ("RadioButton", RoleVariant::Selected),
        ("ui.radio", RoleVariant::Normal),
        ("ui.radio_value", RoleVariant::Normal),
    ] {
        assert_eq!(variant_of(&harness, kind), variant, "{kind} at start");
    }
    harness.get_by_label("ui.radio_value").click();
    harness.run_steps(2);
    for (kind, variant) in [
        ("RadioButton", RoleVariant::Normal),
        ("ui.radio", RoleVariant::Normal),
        ("ui.radio_value", RoleVariant::Selected),
    ] {
        assert_eq!(variant_of(&harness, kind), variant, "{kind} after a click");
    }
}

/// §10.4: no widget is drawn with a seam its info does not name. A widget recorded with the
/// base style is drawn on a `Ui` holding the base style, never inside a role's body (a status
/// bar, a dialog, a window, a card), where its info would list the base style's rows while it
/// is painted with the role's.
#[test]
fn a_base_widget_is_drawn_in_the_base_style() {
    for page in Page::ALL {
        let mut harness = open_page(page, egui::Theme::Light);
        for label in ["Open window", "Open modal"] {
            if harness.query_all_by_label(label).next().is_some() {
                harness.get_by_label(label).click();
                harness.run_steps(2);
            }
        }
        let ctx = harness.ctx.clone();
        harness.state_mut().run_action(Action::OpenAbout, &ctx);
        harness.run_steps(2);
        let off = &harness.state().registry.base_off_base;
        assert!(
            off.is_empty(),
            "{page:?}: recorded as the base style, drawn in another: {off:?}"
        );
    }
}

/// A rebuild that fails keeps the installed atlas and its fonts as they were: installing it
/// again does not register the semibold faces a second time beside the first ones.
#[test]
fn a_failed_rebuild_keeps_the_installed_fonts() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let faces = |h: &Harness<'_, App>| h.ctx.fonts(|f| f.definitions().font_data.len());
    let before = faces(&harness);
    let ctx = harness.ctx.clone();
    harness.state_mut().settings.theme = ThemeChoice::Preset("no-such-preset".to_string());
    harness.state_mut().install(&ctx);
    harness.run_steps(SETTLE);
    assert!(
        harness.state().theme_error.is_some(),
        "the rebuild must fail"
    );
    assert_eq!(faces(&harness), before);
}

/// A platform preset other than [`TEST_PRESET`], installed through `App::install` on any
/// platform, and its atlas: the watcher's rebuilt atlas in the hand-off tests, told from the
/// installed one by its name.
#[cfg(feature = "watch")]
const OTHER_PRESET: &str = "windows-11";

/// The atlas `App::rebuild_atlas` builds of the preset `name` alone, no palette over it and none
/// of the user's preferences, its context's source the desktop's settings `system` (an error:
/// none read), as an install or the watcher's rebuild builds it.
fn preset_atlas_on(
    name: &str,
    system: &crate::app::NextSystem,
) -> Result<crate::app::OldAtlas, crate::app::BuildError> {
    let mut settings = Settings::for_tests();
    settings.theme = ThemeChoice::Preset(name.to_owned());
    App::rebuild_atlas(&settings, system).map(|(atlas, _)| atlas)
}

/// [`preset_atlas_on`] with no desktop settings read, as the watcher's record holds a rebuild.
#[cfg(feature = "watch")]
fn rebuilt(name: &str) -> Result<crate::app::OldAtlas, crate::app::BuildError> {
    preset_atlas_on(name, &Err("not read by the test".to_owned()))
}

/// The name the unpatched 0.8.0 preset `name` resolves to, the atlas's name
/// (`ThemeAtlas::name`) of an install of it.
fn preset_name(name: &str) -> Result<String, String> {
    use native_theme::next::{Preset, Theme as NextTheme, ThemeContext};
    let preset = Preset::from_name(name).ok_or_else(|| format!("no 0.8.0 preset {name}"))?;
    Ok(NextTheme::preset(preset)
        .resolve(&ThemeContext::light())
        .name
        .to_string())
}

#[cfg(feature = "watch")]
fn watch_failure() -> native_theme::error::Error {
    native_theme::error::Error::WatchUnavailable {
        reason: "the showcase's test",
    }
}

/// M9-115: a rebuild the watcher started before the user picked a theme, and that returned after
/// the pick's install, was built for the selection before it. Its hand-off installs the current
/// selection rebuilt, by `install`'s own result, and not the stale atlas over the pick.
#[cfg(feature = "watch")]
#[test]
fn a_hand_off_built_for_an_earlier_selection_installs_the_current_one() -> Result<(), String> {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    let app = harness.state_mut();
    let read = app.generation();
    app.settings.theme = ThemeChoice::Preset(OTHER_PRESET.to_string());
    app.install(&ctx);
    assert_ne!(
        app.generation(),
        read,
        "the install published a new selection"
    );
    let installed = app.atlas.name().to_owned();
    App::record_rebuild(&app.rebuilt, read, rebuilt(TEST_PRESET)).map_err(|e| e.to_string())?;
    app.theme_error = Some("an earlier failure".to_string());
    app.take_rebuilt(&ctx);
    assert_eq!(app.atlas.name(), installed);
    assert_ne!(app.atlas.name(), preset_name(TEST_PRESET)?);
    assert_eq!(
        app.theme_error, None,
        "the current selection's install succeeded"
    );
    Ok(())
}

/// M9-115: a rebuild that returned before the user's install read the record is covered by that
/// install: its hand-off installs nothing.
#[cfg(feature = "watch")]
#[test]
fn a_hand_off_an_install_has_covered_installs_nothing() -> Result<(), String> {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    let app = harness.state_mut();
    let read = app.generation();
    App::record_rebuild(&app.rebuilt, read, rebuilt(TEST_PRESET)).map_err(|e| e.to_string())?;
    app.settings.theme = ThemeChoice::Preset(OTHER_PRESET.to_string());
    app.install(&ctx);
    let installed = app.atlas.name().to_owned();
    app.theme_error = Some("an earlier failure".to_string());
    app.take_rebuilt(&ctx);
    assert_eq!(app.atlas.name(), installed);
    assert_ne!(app.atlas.name(), preset_name(TEST_PRESET)?);
    assert_eq!(app.theme_error.as_deref(), Some("an earlier failure"));
    Ok(())
}

/// M9-114, M9-115: the watcher's `rebuild` records the generation of the selection it read; a
/// hand-off built for the current selection is installed and clears an earlier theme error.
#[cfg(feature = "watch")]
#[test]
fn a_hand_off_built_for_the_current_selection_installs_it_and_clears_the_theme_error()
-> Result<(), String> {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    let app = harness.state_mut();
    App::rebuild_selected(&app.selection, &app.rebuilt).unwrap();
    let recorded = app
        .rebuilt
        .lock()
        .unwrap()
        .built
        .as_ref()
        .map(|(number, generation, _)| (*number, *generation));
    assert_eq!(recorded, Some((1, app.generation())));
    let current = app.generation();
    assert!(App::record_rebuild(&app.rebuilt, current, rebuilt(OTHER_PRESET)).is_ok());
    app.theme_error = Some("an earlier failure".to_string());
    app.take_rebuilt(&ctx);
    assert_eq!(app.atlas.name(), preset_name(OTHER_PRESET)?);
    assert_eq!(app.theme_error, None);
    Ok(())
}

/// M9-114: a failed watcher rebuild is shown in the banner as `theme watcher: …` until a later
/// rebuild is installed, or an install covers it, though the error's text stays the same.
#[cfg(feature = "watch")]
#[test]
fn the_watchers_rebuild_error_is_shown_until_a_later_install() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    let line = format!("theme watcher: {}", watch_failure());
    let shown = |h: &Harness<'_, App>| h.query_by_label(&line).is_some();
    let fail = |h: &mut Harness<'_, App>| {
        let app = h.state_mut();
        let current = app.generation();
        assert!(
            App::record_rebuild(
                &app.rebuilt,
                current,
                Err(crate::app::BuildError(watch_failure().to_string()))
            )
            .is_err()
        );
        app.watcher_last_error = Some(watch_failure().to_string());
        h.run_steps(SETTLE);
    };
    assert!(!shown(&harness));
    fail(&mut harness);
    assert!(shown(&harness), "a failed rebuild is shown");
    let app = harness.state_mut();
    let current = app.generation();
    assert!(App::record_rebuild(&app.rebuilt, current, rebuilt(OTHER_PRESET)).is_ok());
    app.take_rebuilt(&ctx);
    harness.run_steps(SETTLE);
    assert!(!shown(&harness), "the next rebuild was installed");
    fail(&mut harness);
    assert!(
        shown(&harness),
        "a repeat of the same failure is shown again"
    );
    harness.state_mut().install(&ctx);
    harness.run_steps(SETTLE);
    assert!(!shown(&harness), "an install covers the failed rebuild");
}

/// M9-114: the watcher's start error is a note, not a theme error, and stays after an install,
/// since the OS's changes are still not followed.
#[cfg(feature = "watch")]
#[test]
fn the_watchers_start_error_is_a_note_an_install_keeps() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    harness.state_mut().watcher_started(Err(watch_failure()));
    harness.state_mut().install(&ctx);
    harness.run_steps(SETTLE);
    assert!(harness.state().theme_errors().is_empty());
    let note = format!("OS theme changes are not followed: {}", watch_failure());
    assert!(harness.query_by_label(&note).is_some(), "the note is shown");
    let visuals = harness.ctx.global_style().visuals.clone();
    let colours: Vec<egui::Color32> = harness
        .output()
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) if text.galley.job.text == note => {
                text.galley.job.sections.first().map(|s| {
                    if s.format.color == egui::Color32::PLACEHOLDER {
                        text.fallback_color
                    } else {
                        s.format.color
                    }
                })
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        colours,
        vec![visuals.text_color()],
        "the note is drawn in the body text's colour, not as an error ({:?})",
        visuals.error_fg_color
    );
}

/// M9-114, M9-115: a failed pick is published, so the selection the watcher rebuilds is the one
/// the Theme row names. A rebuild of the theme still drawn is then stale: it reinstalls the pick,
/// which fails again, and the pick's error stays. A rebuild of the pick fails too and installs
/// nothing.
#[cfg(feature = "watch")]
#[test]
fn a_failed_picks_error_stays_through_the_watchers_rebuilds() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    let app = harness.state_mut();
    let drawn = app.atlas.name().to_string();
    let read = app.generation();
    app.settings.theme = ThemeChoice::Preset("no-such-preset".to_string());
    app.install(&ctx);
    let error = app.theme_error.clone();
    assert!(error.is_some(), "the pick must fail");
    assert!(App::record_rebuild(&app.rebuilt, read, rebuilt(TEST_PRESET)).is_ok());
    app.take_rebuilt(&ctx);
    assert_eq!(app.atlas.name(), drawn);
    assert_eq!(app.theme_error, error, "a rebuild of the theme drawn");
    App::rebuild_selected(&app.selection, &app.rebuilt).unwrap_err();
    app.take_rebuilt(&ctx);
    assert_eq!(app.atlas.name(), drawn);
    assert_eq!(app.theme_error, error, "a rebuild of the failed pick");
}

/// The watcher's rebuild reads no mode (`App::rebuild_atlas`), so a mode switch publishes nothing:
/// a rebuild in flight across it is current and is installed, not dropped for a rebuild of the
/// same selection.
#[cfg(feature = "watch")]
#[test]
fn a_mode_switch_leaves_a_rebuild_in_flight_current() -> Result<(), String> {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    let app = harness.state_mut();
    let read = app.generation();
    app.set_mode(ModeChoice::Dark, &ctx);
    assert!(App::record_rebuild(&app.rebuilt, read, rebuilt(OTHER_PRESET)).is_ok());
    app.take_rebuilt(&ctx);
    assert_eq!(app.atlas.name(), preset_name(OTHER_PRESET)?);
    Ok(())
}

/// Preferences' text-scaling field is the gpui showcase's: 1.0 to 2.25 in steps of 0.25; a drag
/// installs once, when it ends, not on every pass it moves the value.
#[test]
fn the_text_scaling_field_steps_in_its_range_and_installs_on_release() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    // A known start, not the desktop's own factor.
    harness.state_mut().settings.prefs = Some(AccessibilityPreferences::default());
    harness.state_mut().install(&ctx);
    harness
        .state_mut()
        .run_action(Action::OpenPreferences, &ctx);
    harness.run_steps(SETTLE);
    let installed = |h: &Harness<'_, App>| h.state().atlas.accessibility().text_scaling_factor;
    let chosen = |h: &Harness<'_, App>| {
        h.state()
            .settings
            .prefs
            .as_ref()
            .map(|p| p.text_scaling_factor)
    };
    let before = installed(&harness);
    assert_eq!(before, 1.0);
    let at = centre_of(&harness, "text scaling factor");
    harness.hover_at(at);
    harness.step();
    harness.drag_at(at);
    // Held past egui's click duration, a press is a drag before the pointer moves
    // (`InputState::max_click_duration`, `egui/src/input_state/mod.rs:73`, `:114`).
    harness.run_steps(4);
    let mut seen = Vec::new();
    for dx in [1.0, 2.0, 3.0, 400.0] {
        harness.hover_at(at + egui::vec2(dx, 0.0));
        harness.step();
        let factor = chosen(&harness).unwrap_or(before);
        assert!(
            (1.0..=2.25).contains(&factor) && (factor * 4.0).fract() == 0.0,
            "{factor} after {dx} points: not a step of 0.25 in 1.0..=2.25"
        );
        assert_eq!(installed(&harness), before, "installed during the drag");
        seen.push(factor);
    }
    assert!(
        seen.iter().any(|f| *f > before && *f < 2.25),
        "no step between the ends: {seen:?}"
    );
    assert_eq!(chosen(&harness), Some(2.25), "the range's end: {seen:?}");
    harness.drop_at(at + egui::vec2(400.0, 0.0));
    harness.run_steps(SETTLE);
    assert_eq!(installed(&harness), 2.25, "the release installed nothing");
}

/// §10.4's palette row: only Escape is two-step; a click on the backdrop closes the palette
/// whatever its query holds (`ModalResponse::should_close`, `egui/src/containers/modal.rs:151`).
#[test]
fn a_click_on_the_palette_backdrop_closes_it() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::K);
    harness.run_steps(SETTLE);
    harness
        .query_all_by_role(Role::TextInput)
        .find(|n| n.is_focused())
        .expect("the palette's field has focus")
        .type_text("Pa");
    harness.run_steps(SETTLE);
    assert_eq!(
        harness.state().palette.as_ref().map(|p| p.query.as_str()),
        Some("Pa")
    );
    // A corner of the window: the backdrop, outside the dialog's frame.
    let corner = harness.ctx.viewport_rect().left_bottom() + egui::vec2(2.0, -2.0);
    harness.hover_at(corner);
    harness.step();
    harness.drag_at(corner);
    harness.step();
    harness.drop_at(corner);
    harness.run_steps(SETTLE);
    assert!(
        harness.state().palette.is_none(),
        "a backdrop click left the palette open while its query was not empty"
    );
}

/// §10.4's side-panel row: a mode is egui's own theme call. The atlas carries both schemes, so
/// a switch neither rebuilds nor reinstalls it — seen here as a selection changed behind the
/// app's back that the switch does not install — from the Mode row as from the Theme menu.
#[test]
fn a_mode_switch_installs_nothing() -> Result<(), String> {
    let other = native_theme::theme::Theme::list_presets_for_platform()
        .into_iter()
        .map(|info| info.key)
        .find(|key| *key != TEST_PRESET)
        .ok_or("no second preset is offered on this platform")?;
    let installed = preset_name(TEST_PRESET)?;
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    harness.state_mut().settings.theme = ThemeChoice::Preset(other.to_string());
    pick_mode(&mut harness, "Dark");
    assert_eq!(harness.state().settings.mode, ModeChoice::Dark);
    assert_eq!(harness.ctx.theme(), egui::Theme::Dark);
    assert_eq!(
        harness.state().atlas.name(),
        installed,
        "the Mode row rebuilt the atlas"
    );
    let ctx = harness.ctx.clone();
    harness
        .state_mut()
        .run_action(Action::SetMode(ModeChoice::Light), &ctx);
    harness.run_steps(SETTLE);
    assert_eq!(harness.state().settings.mode, ModeChoice::Light);
    assert_eq!(harness.ctx.theme(), egui::Theme::Light);
    assert_eq!(
        harness.state().atlas.name(),
        installed,
        "Theme > Light rebuilt the atlas"
    );
    Ok(())
}

/// M9-113: under a preset with Mode = System the atlas carries the OS's colour mode
/// (`native_theme::detect::system_is_dark()`), so where the integration reports no scheme
/// (Linux; the harness reports none either) egui draws the desktop's, not its
/// `Options::fallback_theme`, once an install hands the choice back to `ThemePreference::System`.
#[test]
fn a_preset_under_the_system_mode_follows_the_os() {
    let (os_mode, drawn) = if native_theme::detect::system_is_dark() {
        (native_theme_egui::ColorMode::Dark, egui::Theme::Dark)
    } else {
        (native_theme_egui::ColorMode::Light, egui::Theme::Light)
    };
    let mut harness = open(
        egui::Theme::Light,
        cli(&[("--theme", TEST_PRESET), ("--variant", "system")]),
    );
    harness.run_steps(SETTLE);
    assert_eq!(harness.state().settings.mode, ModeChoice::System);
    assert_eq!(harness.state().atlas.os_mode(), Some(os_mode));

    let ctx = harness.ctx.clone();
    harness.state_mut().install(&ctx);
    harness.run_steps(SETTLE);
    assert_eq!(harness.state().atlas.os_mode(), Some(os_mode));
    assert_eq!(
        harness.ctx.theme(),
        drawn,
        "Mode = System drew another scheme than the OS's"
    );
}

/// §10.4's Text row: the code view's colours are egui_extras's own for the scheme shown, and
/// its font is the base style's `Monospace` slot, after a mode switch or an install as before.
#[test]
fn the_code_view_follows_the_scheme() -> Result<(), String> {
    let mut harness = open_page(Page::Text, egui::Theme::Light);
    let is_dark = |h: &Harness<'_, App>| {
        h.state()
            .demo_state
            .code_theme
            .as_ref()
            .map(|(_, t)| t.is_dark())
    };
    let font = |h: &Harness<'_, App>| {
        h.state()
            .demo_state
            .code_theme
            .as_ref()
            .map(|((_, font), _)| font.clone())
    };
    let monospace =
        |h: &Harness<'_, App>| egui::TextStyle::Monospace.resolve(&h.ctx.global_style());
    assert_eq!(is_dark(&harness), Some(false));
    let ctx = harness.ctx.clone();
    harness
        .state_mut()
        .run_action(Action::SetMode(ModeChoice::Dark), &ctx);
    harness.run_steps(3);
    assert_eq!(harness.ctx.theme(), egui::Theme::Dark);
    assert_eq!(
        is_dark(&harness),
        Some(true),
        "the code view kept the light scheme's colours"
    );

    // A preset whose `Monospace` slot differs from the test preset's, installed.
    let before = monospace(&harness);
    let system = &Err("not read by the test".to_owned());
    let other = harness
        .state()
        .theme_rows()
        .into_iter()
        .filter_map(|(choice, _)| match choice {
            ThemeChoice::Preset(key) => Some(key),
            ThemeChoice::Default => None,
        })
        .find(|key| {
            preset_atlas_on(key, system).is_ok_and(|atlas| {
                let probe = egui::Context::default();
                atlas.install(&probe);
                probe.set_theme(egui::Theme::Dark);
                egui::TextStyle::Monospace.resolve(&probe.global_style()) != before
            })
        })
        .ok_or("no preset the Theme row offers has another monospace font")?;
    harness.state_mut().settings.theme = ThemeChoice::Preset(other);
    harness.state_mut().install(&ctx);
    harness.run_steps(3);
    assert_ne!(monospace(&harness), before);
    assert_eq!(
        font(&harness),
        Some(monospace(&harness)),
        "the code view kept the previous preset's monospace font"
    );
    Ok(())
}

/// The Containers page's nested `Panel::right` and `CentralPanel` set their own clip rects
/// (`egui/src/containers/panel.rs:824`, `:1241`), so they must never lie where the page is not
/// shown: at the window size `main` opens, and with the page scrolled to its end, they stay
/// below the page tabs, above the status bar and inside the content.
#[test]
fn the_nested_panels_stay_inside_the_page() {
    fn check(harness: &Harness<'_, App>, when: &str) {
        let records = harness.state().registry.records();
        let rect_of = |kind: &str| {
            records
                .iter()
                .find(|r| r.info.kind == kind)
                .map(|r| r.rect)
                .unwrap_or_else(|| panic!("{when}: no {kind} record"))
        };
        let (content, tabs, status) = (
            rect_of("Central panel"),
            rect_of("TabBar · Pages"),
            rect_of("Status bar"),
        );
        for kind in ["right panel", "CentralPanel (nested)"] {
            let rect = rect_of(kind);
            assert!(
                content.contains_rect(rect)
                    && rect.top() >= tabs.bottom()
                    && rect.bottom() <= status.top(),
                "{when}: {kind} at {rect:?}, the content {content:?}, the tabs' bottom {}, \
                 the status bar's top {}",
                tabs.bottom(),
                status.top()
            );
        }
    }
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let mut harness = open_page(Page::Containers, theme);
        check(&harness, &format!("{theme:?}, at the top"));
        let last = harness
            .state()
            .registry
            .records()
            .iter()
            .rev()
            .find(|r| r.info.kind == "ui.centered_and_justified")
            .map(|r| r.id.accesskit_id())
            .expect("the page's last item");
        harness
            .query_all(By::new().predicate(move |n| n.locate().0 == last))
            .next()
            .expect("its node")
            .scroll_to_me();
        harness.run_steps(3);
        check(&harness, &format!("{theme:?}, scrolled to the end"));
    }
}

/// The status bar is as tall as its content — one line of `status_bar.label` — and its padding,
/// `status_bar.bar.padding` of the installed 0.8.0 theme, as the model states no status-bar
/// height: not a push button's least height, which the base style's `interact_size.y` carries
/// (kde-breeze: a 3 + 18 + 2 bar under a 32 button).
#[test]
fn the_status_bar_is_one_line_and_its_padding_tall() -> Result<(), String> {
    let harness = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    let Some(r) = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .cloned()
    else {
        return Err("no 0.8.0 theme installed".into());
    };
    // `status_bar.label`'s size, which the theme's resolve has scaled already.
    let size = r.status_bar.label.size_px;
    let line = harness
        .ctx
        .fonts_mut(|f| f.row_height(&egui::FontId::proportional(size)));
    let [top, _, bottom, _] = native_theme_egui::next::padding_of(&r.status_bar.bar);
    let (Some(top), Some(bottom)) = (top, bottom) else {
        return Err("kde-breeze states no status_bar.bar padding above and below".into());
    };
    let bar = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "Status bar")
        .map(|r| r.rect)
        .unwrap_or_else(|| panic!("no Status bar record"));
    // The side-panel toggle is content too: without an icon (no icon feature) it is a labelled
    // button, taller than a line of text.
    let toggle = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind.starts_with("Button · Ghost") && bar.contains_rect(r.rect))
        .map(|r| r.rect.height())
        .fold(0.0, f32::max);
    // And the separator line above it, which the panel keeps room for outside its frame
    // (`egui/src/containers/panel.rs:950-966`), in `status_bar.bar.stroke.width`.
    let rule = native_theme_egui::next::stroke_width_of(&r.status_bar.bar)
        .ok_or("kde-breeze states no status_bar.bar stroke width")?;
    let content = line.max(toggle);
    let expected = rule + top + content + bottom;
    assert!(
        (bar.height() - expected).abs() < 0.5,
        "the status bar is {} tall, a {rule} line + {top} + {content} of content (a {line} text \
         line, a {toggle} toggle) + {bottom} is {expected}",
        bar.height()
    );
    Ok(())
}

/// The status bar's padding on a side the theme leaves unstated, `(vertical, horizontal)`, as
/// the gpui showcase's bar draws it at a scale of 1: gpui-component's `py_1`/`px_2`, a quarter
/// and a half of the installed 0.8.0 theme's `typography.body.size_px`, which its resolve has
/// scaled already, each rounded to a whole pixel with a half toward zero, as gpui rounds a
/// padding before layout (gpui-pre 0.3.12 `src/taffy.rs:534-538`, `round_to_device_pixel`;
/// `src/util.rs:130-133`, `round_half_toward_zero`).
fn gpui_status_bar_padding(harness: &Harness<'_, App>) -> Result<(f32, f32), String> {
    let rem = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme installed")?
        .typography
        .body
        .size_px;
    let whole = |length: f32| (length.abs() - 0.5).ceil().copysign(length);
    Ok((whole(rem / 4.0), whole(rem / 2.0)))
}

/// The status bar is padded as the gpui showcase's on a side the theme leaves unstated, a half
/// pixel rounded toward zero as gpui rounds it, where egui rounded it away from zero: under
/// material-3 and windows-11 (a 14 px font, a quarter of it 3.5) 3 px above and below, where 4.
/// The expectation is read from each installed 0.8.0 theme at test time.
#[test]
fn the_status_bar_padding_rounds_a_half_as_gpuis_does() -> Result<(), String> {
    for preset in ["material-3", "windows-11"] {
        let harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or("no 0.8.0 theme installed")?;
        assert!(
            native_theme_egui::next::padding_of(&r.status_bar.bar)
                .iter()
                .all(Option::is_none),
            "{preset} states a status-bar padding"
        );
        assert_eq!(
            r.typography.body.size_px, 14.0,
            "{preset}'s body text is not 14 px, so no half is rounded"
        );
        let (vertical, horizontal) = gpui_status_bar_padding(&harness)?;
        let status = place_of(&harness, "chrome.status_bar");
        let toggle = place_of(&harness, "chrome.status_bar.toggle");
        assert!(
            (toggle.left() - status.left() - horizontal).abs() < 0.25,
            "{preset}: the status bar's toggle is {} from the bar's left, not {horizontal}",
            toggle.left() - status.left()
        );
        let size = r.status_bar.label.size_px;
        let line = harness
            .ctx
            .fonts_mut(|f| f.row_height(&egui::FontId::proportional(size)));
        // The panel keeps room for its line, rounded to a whole point, outside its frame
        // (`egui/src/containers/panel.rs:961-965`).
        let rule = native_theme_egui::next::stroke_width_of(&r.status_bar.bar)
            .ok_or_else(|| format!("{preset} states no status_bar.bar stroke width"))?
            .round();
        let expected = rule + vertical + line.max(toggle.height()) + vertical;
        assert!(
            (status.height() - expected).abs() < 0.25,
            "{preset}: the status bar is {} tall, a {rule} line + {vertical} + its content + \
             {vertical} is {expected}",
            status.height()
        );
    }
    Ok(())
}

/// The status bar keeps the shown Widget Info's title and cuts the environment short in the
/// room the title leaves, as the gpui showcase's does (`demo::status_bar`, `truncate` on the
/// environment), its whole text on hover (egui 0.36.2 `widgets/label.rs:284-293`): under
/// windows-11 at a text scale of 1.5, with the window narrowed until that room is half the
/// width the whole environment line takes laid out on one line, as its hover text is, so the
/// measured text and not the host's fonts sets the width, the environment's painted galley is
/// cut short, both lie inside the bar and the environment ends before the title begins; the
/// galley's text is the whole line, `chrome::environment`'s items joined. Hovered there, the
/// environment shows its own title, and with the window narrowed again for it, its whole line
/// is painted a second time, uncut, in its tooltip. The places are the widgets' whole
/// rectangles, not the parts the bar's clip leaves in view.
#[test]
fn the_status_bar_keeps_the_title_and_cuts_the_environment_short() {
    // Whether each painted galley of `whole` is cut short, in paint order.
    fn painted(harness: &Harness<'_, App>, whole: &str) -> Vec<bool> {
        harness
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(s) if s.galley.text() == whole => Some(s.galley.elided),
                _ => None,
            })
            .collect()
    }
    // The window narrowed until the room the shown title leaves the environment is half the
    // width its whole line takes, and the pointer rested at `at` of the page laid out again,
    // which moves with the width, past the tooltip delay, in the harness's quarter-second steps
    // (`egui_kittest/src/builder.rs:40`).
    fn narrow(
        harness: &mut Harness<'_, App>,
        whole: &str,
        at: impl Fn(&Harness<'_, App>) -> egui::Pos2,
    ) {
        let job = harness
            .output()
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(s) if s.galley.text() == whole => Some(s.galley.job.clone()),
                _ => None,
            })
            .unwrap_or_else(|| panic!("the environment {whole:?} is not painted"));
        let needed = harness
            .ctx
            .fonts_mut(|fonts| {
                fonts.layout_job(egui::text::LayoutJob {
                    sections: job.sections.clone(),
                    text: job.text.clone(),
                    ..Default::default()
                })
            })
            .size()
            .x;
        let environment = place_of(harness, "chrome.status_bar.environment");
        let title = place_of(harness, "chrome.status_bar.shown");
        let window = harness.ctx.viewport_rect().size(); // `egui/src/context.rs:2921`
        let narrower = title.left() - environment.left() - needed / 2.0;
        harness.set_size(egui::vec2(window.x - narrower, window.y));
        harness.run_steps(4);
        let delay = harness.ctx.global_style().interaction.tooltip_delay;
        let pos = at(harness);
        harness.hover_at(pos);
        harness.run_steps(4 + (delay / 0.25).ceil() as usize);
    }
    let on_toggle = |harness: &Harness<'_, App>| centre_of(harness, "toggle button (on)");
    let on_environment = |harness: &Harness<'_, App>| {
        let environment = place_of(harness, "chrome.status_bar.environment");
        egui::pos2(environment.left() + 4.0, environment.center().y)
    };

    let mut harness = open_scaled("windows-11", "basic", 1.5);
    let pos = on_toggle(&harness);
    hover_and_settle(&mut harness, pos);
    let toggle_title = harness.state().status_title();
    assert!(
        !toggle_title.is_empty(),
        "hovering the toggle shows its title in the status bar"
    );
    let whole = crate::chrome::environment(harness.state(), &harness.ctx).join(" · ");
    narrow(&mut harness, &whole, on_toggle);
    assert_eq!(
        harness.state().status_title(),
        toggle_title,
        "the toggle's title is not shown in the narrowed window"
    );
    assert_eq!(
        painted(&harness, &whole),
        vec![true],
        "{whole:?}: the environment is not painted once, cut short, in half its room"
    );
    // An egui panel grows with what it holds, so the bar is bounded by the window too.
    let window = harness.ctx.viewport_rect();
    let bar = place_of(&harness, "chrome.status_bar");
    assert!(
        bar.right() <= window.right() + 0.5,
        "the status bar {bar:?} runs past the window {window:?}"
    );
    let environment = place_of(&harness, "chrome.status_bar.environment");
    let title = place_of(&harness, "chrome.status_bar.shown");
    for (what, rect) in [("environment", environment), ("title", title)] {
        assert!(
            bar.x_range().contains(rect.left()) && rect.right() <= bar.right() + 0.5,
            "the status bar's {what} {rect:?} is not inside the bar {bar:?}"
        );
    }
    assert!(
        environment.right() <= title.left() + 0.5,
        "the environment {environment:?} runs into the title {title:?}"
    );

    let pos = on_environment(&harness);
    hover_and_settle(&mut harness, pos);
    narrow(&mut harness, &whole, on_environment);
    assert_ne!(
        harness.state().status_title(),
        toggle_title,
        "hovering the environment does not show its own title"
    );
    assert_eq!(
        painted(&harness, &whole),
        vec![true, false],
        "{whole:?}: the environment cut short and its whole line in its tooltip"
    );
}

/// The element `id`'s place this pass.
fn place_of(harness: &Harness<'_, App>, id: &str) -> egui::Rect {
    harness
        .state()
        .registry
        .places()
        .get(id)
        .copied()
        .unwrap_or_else(|| panic!("no {id} placed"))
}

/// The *Not themeable* lines of every record this pass names the element `id`, as Widget Info
/// shows them (`what: note`), one list per record; at least one record.
fn element_notes(harness: &Harness<'_, App>, id: &str) -> Vec<Vec<String>> {
    let notes: Vec<Vec<String>> = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.element.as_deref() == Some(id))
        .map(|r| {
            r.info
                .not_themeable
                .iter()
                .map(|(what, note)| format!("{what}: {note}"))
                .collect()
        })
        .collect();
    assert!(!notes.is_empty(), "no record is the element {id}");
    notes
}

/// Whether every record of the element `id` notes a line holding each of `parts`.
fn every_record_notes(harness: &Harness<'_, App>, id: &str, parts: &[&str]) -> bool {
    element_notes(harness, id)
        .iter()
        .all(|lines| lines.iter().any(|l| parts.iter().all(|p| l.contains(p))))
}

/// F rows 3, 4, 6 and 7 (M9-134's egui half): under macos-tahoe, which states no toolbar or
/// status-bar padding, no toolbar item gap and no `layout.container_margin`, the chrome bars are
/// padded as the gpui showcase's are, and Widget Info says so: the toolbar's first button
/// `TOOLBAR_PADDING` in from the bar's left and top, the menu bar's first title
/// `MENU_BAR_PADDING` from its left, the status bar gpui-component's `py_1`/`px_2` at the
/// installed 0.8.0 theme's `typography.body.size_px` (a quarter of it above and below, a half
/// at the sides, each rounded to a whole point a half toward zero,
/// [`gpui_status_bar_padding`]), where egui's own panel margin stood; the paddings
/// `toolbar.bar(NONE).padding` and `status_bar.bar.padding`. macos-tahoe is a preset of
/// another platform here, so it is installed directly. Under [`TEST_PRESET`], which states
/// `container_margin` but no toolbar padding, the toolbar's unstated sides are noted as that
/// margin (I m3, J m3 (a)); kde-breeze states every side of both bars and notes none.
#[test]
fn the_chrome_bars_take_the_gpui_showcases_padding_where_the_theme_states_none()
-> Result<(), String> {
    use crate::chrome::{MENU_BAR_PADDING, TOOLBAR_PADDING};
    use native_theme::next::WidgetState;
    use native_theme_egui::next::padding_of;
    let harness = open_page_under(Page::Basic, egui::Theme::Light, "macos-tahoe");
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme installed")?
        .clone();
    assert_eq!(r.layout.container_margin, None);
    let unstated = |p: [Option<f32>; 4]| p.iter().all(Option::is_none);
    assert!(
        unstated(padding_of(&r.toolbar.bar(WidgetState::NONE)))
            && unstated(padding_of(&r.status_bar.bar))
    );
    assert_eq!(r.toolbar.item_gap, None);

    let bar = place_of(&harness, "chrome.toolbar");
    let first = place_of(&harness, "chrome.toolbar.command_palette");
    assert!(
        (first.left() - bar.left() - TOOLBAR_PADDING).abs() < 0.5
            && (first.top() - bar.top() - TOOLBAR_PADDING).abs() < 0.5,
        "the toolbar's first button is {} in from the bar's left and {} from its top",
        first.left() - bar.left(),
        first.top() - bar.top()
    );
    assert!(
        every_record_notes(
            &harness,
            "chrome.toolbar",
            &[
                "not stated by the theme on top, right, bottom, left",
                "TOOLBAR_PADDING"
            ]
        ),
        "the toolbar's info: {:?}",
        element_notes(&harness, "chrome.toolbar")
    );

    let menu = place_of(&harness, "chrome.menu_bar");
    let file = place_of(&harness, "chrome.menu_bar.file");
    assert!(
        (file.left() - menu.left() - MENU_BAR_PADDING).abs() < 0.5,
        "the menu bar's first title is {} from the bar's left",
        file.left() - menu.left()
    );
    assert!(
        every_record_notes(
            &harness,
            "chrome.menu_bar",
            &["not stated by the theme on left, right", "MENU_BAR_PADDING"]
        ),
        "the menu bar's info: {:?}",
        element_notes(&harness, "chrome.menu_bar")
    );

    let (vertical, horizontal) = gpui_status_bar_padding(&harness)?;
    let status = place_of(&harness, "chrome.status_bar");
    let toggle = place_of(&harness, "chrome.status_bar.toggle");
    assert!(
        (toggle.left() - status.left() - horizontal).abs() < 0.5,
        "the status bar's toggle is {} from the bar's left, not {horizontal}",
        toggle.left() - status.left()
    );
    let size = r.status_bar.label.size_px;
    let line = harness
        .ctx
        .fonts_mut(|f| f.row_height(&egui::FontId::proportional(size)));
    // The panel keeps room for its line, rounded to a whole point, outside its frame
    // (`egui/src/containers/panel.rs:961-965`): macos-tahoe's line is half a pixel.
    let rule = native_theme_egui::next::stroke_width_of(&r.status_bar.bar)
        .ok_or("macos-tahoe states no status_bar.bar stroke width")?
        .round();
    let expected = rule + vertical + line.max(toggle.height()) + vertical;
    assert!(
        (status.height() - expected).abs() < 0.5,
        "the status bar is {} tall, a {rule} line + {vertical} + its content + {vertical} is \
         {expected}",
        status.height()
    );
    assert!(
        every_record_notes(
            &harness,
            "chrome.status_bar",
            &[
                "not stated by the theme on top, right, bottom, left",
                "gpui-component's StatusBar padding"
            ]
        ),
        "the status bar's info: {:?}",
        element_notes(&harness, "chrome.status_bar")
    );

    let mut harness = open(
        egui::Theme::Light,
        cli(&[("--theme", TEST_PRESET), ("--tab", "basic")]),
    );
    harness.run_steps(4);
    assert!(
        every_record_notes(
            &harness,
            "chrome.toolbar",
            &[
                "not stated by the theme on top, right, bottom, left",
                "layout.container_margin"
            ]
        ),
        "{TEST_PRESET}: the toolbar's info: {:?}",
        element_notes(&harness, "chrome.toolbar")
    );

    let harness = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    for id in ["chrome.toolbar", "chrome.status_bar", "chrome.menu_bar"] {
        assert!(
            element_notes(&harness, id)
                .iter()
                .flatten()
                .all(|l| !l.contains("not stated by the theme on")),
            "kde-breeze states every side of {id}: {:?}",
            element_notes(&harness, id)
        );
    }
    Ok(())
}

/// The macos-tahoe atlas the showcase installs, with its 0.8.0 themes' `layout.widget_gap` and
/// `toolbar.item_gap` unstated, as no bundled preset leaves both (`OldAtlas::edited`), installed
/// into `harness`: the case where the chrome's own constants for that role stand.
fn install_tahoe_without_widget_gap(harness: &mut Harness<'static, App>) -> Result<(), String> {
    install_under(harness, egui::Theme::Light, "macos-tahoe");
    let atlas = harness.state().atlas.edited(|r| {
        r.layout.widget_gap = None;
        r.toolbar.item_gap = None;
    })?;
    let ctx = harness.ctx.clone();
    harness.state_mut().adopt_pair(&ctx, atlas);
    harness.run_steps(4);
    Ok(())
}

/// F rows 4 and 5 (pass 6 m2): where the theme states no `layout.widget_gap` and no
/// `toolbar.item_gap`, the theme settings' rows, and each label and its control, are
/// `THEME_SETTINGS_GAP` apart and the toolbar's items `TOOLBAR_GAP` apart, the gpui showcase's
/// constants, which the settings' and the toolbar's Widget Info name.
#[test]
fn without_a_widget_gap_the_settings_and_the_toolbar_take_the_gpui_showcases_gaps()
-> Result<(), String> {
    use crate::chrome::{THEME_SETTINGS_GAP, TOOLBAR_GAP};
    let mut harness = open(
        egui::Theme::Light,
        cli(&[("--theme", TEST_PRESET), ("--tab", "basic")]),
    );
    harness.run_steps(4);
    install_tahoe_without_widget_gap(&mut harness)?;
    let gaps = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .map(|r| (r.layout.widget_gap, r.toolbar.item_gap));
    assert_eq!(gaps, Some((None, None)));
    let placed = |id: &str| Some((id.to_owned(), place_of(&harness, id)));
    let stack = [
        placed("chrome.side_panel.settings.theme_label"),
        placed("chrome.side_panel.settings.theme"),
        placed("chrome.side_panel.settings.palette_label"),
        placed("chrome.side_panel.settings.palette"),
        placed("chrome.side_panel.settings.mode_label"),
        placed("chrome.side_panel.settings.mode"),
        placed("chrome.side_panel.settings.icon_theme_label"),
        placed("chrome.side_panel.settings.icon_theme"),
    ];
    let stack: Option<Vec<(String, egui::Rect)>> = stack.into_iter().collect();
    let stack = stack.ok_or("a settings row is not placed")?;
    for pair in stack.windows(2) {
        let [(above_id, above), (below_id, below)] = pair else {
            continue;
        };
        assert!(
            (below.top() - above.bottom() - THEME_SETTINGS_GAP).abs() < 0.5,
            "{below_id} is {} below {above_id}",
            below.top() - above.bottom(),
        );
    }
    assert!(
        every_record_notes(
            &harness,
            "chrome.side_panel.settings",
            &["not stated by the theme", "THEME_SETTINGS_GAP"]
        ),
        "the settings' info: {:?}",
        element_notes(&harness, "chrome.side_panel.settings")
    );
    let first = place_of(&harness, "chrome.toolbar.command_palette");
    let second = place_of(&harness, "chrome.toolbar.reload_theme");
    assert!(
        (second.left() - first.right() - TOOLBAR_GAP).abs() < 0.5,
        "the toolbar's first two items are {} apart",
        second.left() - first.right()
    );
    assert!(
        every_record_notes(&harness, "chrome.toolbar", &["TOOLBAR_GAP"]),
        "the toolbar's info: {:?}",
        element_notes(&harness, "chrome.toolbar")
    );
    Ok(())
}

/// F row 5: where `sidebar.container(NONE).padding` of the installed 0.8.0 theme states some
/// sides, the side panel keeps them, and a side it leaves unstated takes
/// `layout.container_margin`, as the gpui showcase's panel does, where egui's own panel margin
/// stood: windows-11 states the horizontal sides alone, so the panel's top and bottom are the
/// margin, and Widget Info says so (I m3, J m3 (a)); adwaita states all four, and the panel
/// keeps them. windows-11 is a preset of another platform here, so it is installed directly.
#[test]
fn a_side_panel_side_the_theme_leaves_unstated_takes_the_container_margin() -> Result<(), String> {
    use native_theme::next::WidgetState;
    let sides = |harness: &Harness<'_, App>| {
        harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .map(|r| {
                (
                    native_theme_egui::next::padding_of(&r.sidebar.container(WidgetState::NONE)),
                    r.layout.container_margin,
                )
            })
            .ok_or("the showcase installs no 0.8.0 theme")
    };
    let harness = open_page_under(Page::Basic, egui::Theme::Light, "windows-11");
    let ([top, _, bottom, left], margin) = sides(&harness)?;
    let margin = margin.ok_or("windows-11 states no layout.container_margin")?;
    let (Some(left), None, None) = (left, top, bottom) else {
        return Err("windows-11 states sidebar.container(NONE).padding not across alone".into());
    };
    let panel = place_of(&harness, "chrome.side_panel");
    let settings = place_of(&harness, "chrome.side_panel.settings");
    let inspector = place_of(&harness, "chrome.side_panel.inspector");
    for (what, distance, expected) in [
        ("top", settings.top() - panel.top(), margin),
        ("bottom", panel.bottom() - inspector.bottom(), margin),
        ("left", settings.left() - panel.left(), left),
    ] {
        assert!(
            (distance - expected).abs() < 0.5,
            "windows-11: the side panel's {what} padding is {distance}, not {expected}"
        );
    }
    assert!(
        every_record_notes(
            &harness,
            "chrome.side_panel",
            &[
                "not stated by the theme on top, bottom",
                "layout.container_margin"
            ]
        ),
        "windows-11: the side panel's info: {:?}",
        element_notes(&harness, "chrome.side_panel")
    );

    let mut harness = open(
        egui::Theme::Light,
        cli(&[("--theme", "adwaita"), ("--tab", "basic")]),
    );
    harness.run_steps(4);
    let ([top, _, bottom, left], _) = sides(&harness)?;
    let (Some(top), Some(bottom), Some(left)) = (top, bottom, left) else {
        return Err("adwaita does not state every side of sidebar.container(NONE).padding".into());
    };
    let panel = place_of(&harness, "chrome.side_panel");
    let settings = place_of(&harness, "chrome.side_panel.settings");
    let inspector = place_of(&harness, "chrome.side_panel.inspector");
    for (what, distance, expected) in [
        ("top", settings.top() - panel.top(), top),
        ("bottom", panel.bottom() - inspector.bottom(), bottom),
        ("left", settings.left() - panel.left(), left),
    ] {
        assert!(
            (distance - expected).abs() < 0.5,
            "adwaita: the side panel's {what} padding is {distance}, not {expected}"
        );
    }
    Ok(())
}

/// G, H and I: each side of a dialog takes the installed 0.8.0 theme's stated
/// `dialog.panel.padding` side, else gpui-component's 16 px (`DIALOG_PADDING_PX`) inside the
/// panel's line, as the gpui showcase's `Dialog` draws, where egui's popup margin stood: the
/// Overlays page's modal under [`UNSTATED_PRESET`], which states no dialog padding, and under
/// kde-breeze, which states every side, each installed through the Theme row's path on any
/// platform. Under the first Widget Info names the padding on the modal's frame; under
/// kde-breeze it does not. Under [`UNSTATED_PRESET`] with the padding patched to twice
/// gpui-component's on every side, which the unpatched preset leaves unstated, the body lies the
/// patched padding inside the line and the info names no padding.
#[test]
fn a_dialog_side_the_theme_leaves_unstated_is_gpui_components_padding() -> Result<(), String> {
    use native_theme::next::{DialogPatch, Edges, PartPatch, ThemePatch, WidgetPatchMap};
    use native_theme_egui::next::{padding_of, stroke_width_of};
    use native_theme_showcase_support::DIALOG_PADDING_PX;
    fn check(
        harness: &mut Harness<'static, App>,
        r: &native_theme::next::ResolvedTheme,
        preset: &str,
    ) -> Result<(), String> {
        harness.get_by_label("Open modal").click();
        harness.run_steps(SETTLE);
        let rect = |kind: &str| {
            harness
                .state()
                .registry
                .records()
                .iter()
                .find(|r| r.info.kind == kind)
                .map(|r| r.rect)
                .ok_or(format!("{preset}: no {kind} record"))
        };
        let (frame, body) = (rect("modal")?, rect("modal body")?);
        let line = stroke_width_of(&r.dialog.panel)
            .ok_or(format!("{preset} states no dialog.panel stroke width"))?;
        let padding = padding_of(&r.dialog.panel);
        let [top, right, bottom, left] =
            padding.map(|stated| stated.unwrap_or(DIALOG_PADDING_PX) + line);
        for (what, distance, expected) in [
            ("top", body.top() - frame.top(), top),
            ("right", frame.right() - body.right(), right),
            ("bottom", frame.bottom() - body.bottom(), bottom),
            ("left", body.left() - frame.left(), left),
        ] {
            assert!(
                (distance - expected).abs() < 0.5,
                "{preset}: the modal's body is {distance} inside its frame at the {what}, not \
                 {expected}"
            );
        }
        hover_and_settle(harness, frame.left_top() + egui::vec2(2.0, 2.0));
        let shown = harness
            .state()
            .registry
            .shown()
            .cloned()
            .ok_or(format!("{preset}: nothing shown over the modal's frame"))?;
        assert!(shown.info.seams.contains(&Seam::Surface(Surface::Dialog)));
        let noted = shown
            .info
            .not_themeable
            .iter()
            .any(|(_, note)| note.contains("DIALOG_PADDING_PX"));
        assert_eq!(
            noted,
            padding.iter().any(Option::is_none),
            "{preset}: the modal's info: {:?}",
            shown.info.not_themeable
        );
        Ok(())
    }
    for preset in [UNSTATED_PRESET, "kde-breeze"] {
        let mut harness = open_page_under(Page::Overlays, egui::Theme::Light, preset);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or(format!("{preset}: no 0.8.0 theme installed"))?
            .clone();
        let stated = padding_of(&r.dialog.panel).map(|side| side.is_some());
        let want = [preset == "kde-breeze"; 4];
        assert_eq!(
            stated, want,
            "{preset}: the dialog.panel.padding sides stated"
        );
        check(&mut harness, &r, preset)?;
    }
    let (mut harness, r) =
        open_patched(Page::Overlays, egui::Theme::Light, UNSTATED_PRESET, |_| {
            Some(
                ThemePatch::new().widget_rules(
                    WidgetPatchMap::new().dialog(
                        DialogPatch::new()
                            .panel(PartPatch::new().padding(Edges::all(2.0 * DIALOG_PADDING_PX))),
                    ),
                ),
            )
        })?;
    let plain = open_page_under(Page::Overlays, egui::Theme::Light, UNSTATED_PRESET);
    let unpatched = plain
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .map(|r| padding_of(&r.dialog.panel))
        .ok_or("no 0.8.0 theme installed")?;
    assert_eq!(
        unpatched, [None; 4],
        "the unpatched {UNSTATED_PRESET} states a dialog padding, so the patch proves nothing"
    );
    assert_eq!(
        padding_of(&r.dialog.panel),
        [Some(2.0 * DIALOG_PADDING_PX); 4],
        "the patched dialog.panel.padding"
    );
    check(&mut harness, &r, "patched ios-13")
}

/// The Overlays page's `Area` floats beside its anchor label; scrolled with the page in a
/// window too short for it, it never lies over the page tabs or the status bar. The window is
/// short enough that the page can scroll its anchor, near the page's end, under the tabs.
#[test]
fn the_area_stays_inside_the_page() {
    let mut harness = Harness::builder()
        .with_context_setup(emoji_fallbacks)
        .with_theme(egui::Theme::Light)
        .with_size(egui::vec2(crate::WINDOW_SIZE.x, crate::WINDOW_SIZE.y / 4.0))
        .build_eframe(|cc| {
            App::new(cc, &cli(&[("--theme", TEST_PRESET), ("--tab", "overlays")]))
                .expect("the showcase starts under a bundled preset")
        });
    harness.run_steps(4);
    // kde-breeze, on any platform: TEST_PRESET's bars leave the short window no room to scroll
    // the page in.
    let ctx = harness.ctx.clone();
    harness.state_mut().settings.theme = ThemeChoice::Preset("kde-breeze".to_owned());
    harness.state_mut().install(&ctx);
    harness.run_steps(4);
    assert_eq!(harness.state().theme_error, None);
    let rect_of = |harness: &Harness<'_, App>, kind: &str| {
        harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.info.kind == kind)
            .map(|r| r.rect)
            .unwrap_or_else(|| panic!("no {kind} record"))
    };
    // The anchor's own rect, unclipped: its AccessKit node's (`egui/src/response.rs:912-917`).
    let anchor_id = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "area anchor")
        .map(|r| r.id.accesskit_id())
        .expect("the anchor");
    harness.hover_at(rect_of(&harness, "Central panel").center());
    let mut passed_the_tabs = false;
    for _ in 0..20 {
        // The page's content moved up, a wheel notch at a time.
        harness.event(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -40.0),
            phase: egui::TouchPhase::Move,
            source: egui::MouseWheelSource::Unknown,
            modifiers: egui::Modifiers::NONE,
        });
        harness.run_steps(2);
        let chrome = [
            rect_of(&harness, "Toolbar"),
            rect_of(&harness, "TabBar · Pages"),
            rect_of(&harness, "Status bar"),
        ];
        let anchor = harness
            .query_all(By::new().predicate(move |n| n.locate().0 == anchor_id))
            .next()
            .map(|n| n.rect())
            .expect("the anchor's node");
        passed_the_tabs |= anchor.top() < chrome[1].bottom();
        // What the Area paints: its label's rect as egui clips it (inverted once clipped away).
        let area = rect_of(&harness, "area label");
        let over = |c: &egui::Rect| {
            let common = c.intersect(area);
            common.width() > 0.0 && common.height() > 0.0
        };
        assert!(
            !chrome.iter().any(over),
            "the Area at {area:?} lies over the chrome {chrome:?}"
        );
    }
    assert!(
        passed_the_tabs,
        "the page never scrolled its anchor under the tabs"
    );
}

/// At the window size `main` opens, under every preset this platform offers, the page tabs and
/// the Text page's link row lie inside the content: the link row wraps where it is too wide,
/// and the tab row scrolls, its page menu at its right end inside the content, as the gpui
/// showcase's page `TabBar` does (parity item 9).
#[test]
fn the_rows_fit_the_content() {
    for info in native_theme::theme::Theme::list_presets_for_platform() {
        let mut harness = open(
            egui::Theme::Light,
            cli(&[("--theme", info.key), ("--tab", Page::Text.key())]),
        );
        harness.run_steps(4);
        let records = harness.state().registry.records();
        let rect_of = |kind: &str| {
            records
                .iter()
                .find(|r| r.info.kind == kind)
                .map(|r| r.rect)
                .unwrap_or_else(|| panic!("{}: no {kind} record", info.key))
        };
        let right_of = |kind: &str| rect_of(kind).right();
        let (content, bar, menu) = (
            rect_of("Central panel"),
            rect_of("TabBar · Pages"),
            rect_of("Menu button · Pages"),
        );
        assert!(
            content.contains_rect(bar) && bar.contains_rect(menu),
            "{}: the tab row {bar:?} or its menu {menu:?} is outside the content {content:?}",
            info.key
        );
        // A horizontal separator spans the page's width, less a scroll bar where the page
        // scrolls.
        let rows = [(
            right_of("Separator (horizontal)"),
            &[
                "Hyperlink",
                "Link",
                "Link (disabled)",
                "ui.link",
                "ui.hyperlink",
                "ui.hyperlink_to",
            ][..],
        )];
        for (edge, kinds) in rows {
            // By their records; a node's AccessKit bounds are the widget's own rect, not
            // clipped as a record's `interact_rect` is (`egui/src/response.rs:912-917`).
            let ids: Vec<egui::accesskit::NodeId> = records
                .iter()
                .filter(|r| kinds.contains(&r.info.kind))
                .map(|r| r.id.accesskit_id())
                .collect();
            assert!(ids.len() >= kinds.len(), "{}: {kinds:?}", info.key);
            let cut: Vec<(String, egui::Rect)> = harness
                .query_all(By::new().predicate(move |n| ids.contains(&n.locate().0)))
                .filter(|n| n.rect().right() > edge)
                .map(|n| (n.accesskit_node().label().unwrap_or_default(), n.rect()))
                .collect();
            assert!(
                cut.is_empty(),
                "{}: past the right edge {edge}: {cut:?}",
                info.key
            );
        }
    }
}

/// Parity item 23: the Icons page's sections are the gpui showcase's, the Icon Sizes cells in
/// its order with their names on one line, every role's cell named by the role.
#[test]
fn the_icons_page_has_the_gpui_showcases_sections() {
    let harness = open_page(Page::Icons, egui::Theme::Light);
    let records = harness.state().registry.records();
    let sizes: Vec<egui::Rect> = records
        .iter()
        .filter(|r| r.info.kind == "Label · icon size")
        .map(|r| r.rect)
        .collect();
    assert_eq!(sizes.len(), 5, "five Icon Sizes cells");
    assert!(
        sizes
            .windows(2)
            .all(|w| w[0].right() <= w[1].left() && (w[0].bottom() - w[1].bottom()).abs() < 1.0),
        "the Icon Sizes names are not on one line, left to right: {sizes:?}"
    );
    let names = records
        .iter()
        .filter(|r| r.info.kind == "Label · icon name")
        .count();
    assert_eq!(
        names,
        native_theme::icons::IconRole::ALL.len(),
        "a named cell per role"
    );
    let headings: Vec<egui::accesskit::NodeId> = records
        .iter()
        .filter(|r| r.info.kind == "heading")
        .map(|r| r.id.accesskit_id())
        .collect();
    let shown: Vec<String> = harness
        .query_all(By::new().predicate(move |n| headings.contains(&n.locate().0)))
        .filter_map(|n| n.accesskit_node().value())
        .collect();
    for heading in ["Icon Sizes", "Animated Icons", "Native Theme Icons: "] {
        assert!(
            shown.iter().any(|s| s.starts_with(heading)),
            "no {heading} heading in {shown:?}"
        );
    }
}

/// A freedesktop icon theme name no icon theme is installed under, so the Icons page draws its
/// placeholders.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
const ABSENT_ICON_THEME: &str = "native-theme-test-no-such-icon-theme";

/// The Icons page's placeholder for an icon the set lacks is filled with the installed 0.8.0
/// theme's `button.body(NONE).fill`, as gpui's skeleton is: under [`TEST_PRESET`] with a
/// freedesktop icon theme no host has ([`ABSENT_ICON_THEME`]), whose roles are absent, and that
/// fill patched to `colors.window.border`, a colour of the same theme its unpatched
/// `button.body(NONE).fill` is not, every `Image (absent)` square is filled with it.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
#[test]
fn the_icons_placeholder_takes_the_08_button_fill() -> Result<(), String> {
    use native_theme::next::{ButtonRule, Paint, PartPatch, ThemePatch, WidgetState};
    let (mut harness, r) = open_patched(Page::Icons, egui::Theme::Light, TEST_PRESET, |r| {
        Some(ThemePatch::new().button(
            ButtonRule::new().body(PartPatch::new().fill(Paint::Solid(r.colors.window.border?))),
        ))
    })?;
    harness.state_mut().settings.icon = IconSetChoice::Freedesktop(ABSENT_ICON_THEME.to_owned());
    harness.run_steps(SETTLE);
    let fill = crate::demo::drawn_fill(&r.button.body(WidgetState::NONE))
        .ok_or("the patch states no button.body(NONE).fill")?;
    let unpatched = open_page_under(Page::Icons, egui::Theme::Light, TEST_PRESET)
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .map(|r| crate::demo::drawn_fill(&r.button.body(WidgetState::NONE)))
        .ok_or("no 0.8.0 theme installed")?;
    assert_ne!(
        Some(fill),
        unpatched,
        "the patched fill is the unpatched button.body(NONE).fill, so the patch proves nothing"
    );
    let absent: Vec<egui::Rect> = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "Image (absent)")
        .map(|r| r.rect)
        .collect();
    if absent.is_empty() {
        return Err(
            "the preset's icon choice lacks no role, so no placeholder is drawn".to_owned(),
        );
    }
    let shapes = harness.output().shapes.clone();
    for rect in absent {
        let filled = shapes.iter().any(|clipped| match &clipped.shape {
            egui::Shape::Rect(shape) => shape.fill == fill && rect.contains_rect(shape.rect),
            _ => false,
        });
        assert!(
            filled,
            "the placeholder at {rect:?} is not filled with {fill:?}"
        );
    }
    Ok(())
}

/// Parity items 19 and 21: the command palette and About are titled, as gpui-component's
/// `Dialog` is, and the close button of the title row closes them.
#[test]
fn the_palette_and_about_are_titled_and_close() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    for (action, title) in [
        (Action::OpenCommandPalette, "Command Palette"),
        (Action::OpenAbout, "About"),
    ] {
        harness.state_mut().run_action(action, &ctx);
        harness.run_steps(2);
        let titles: Vec<egui::accesskit::NodeId> = harness
            .state()
            .registry
            .records()
            .iter()
            .filter(|r| r.info.kind == "Label · dialog title")
            .map(|r| r.id.accesskit_id())
            .collect();
        let shown: Vec<String> = harness
            .query_all(By::new().predicate(move |n| titles.contains(&n.locate().0)))
            .filter_map(|n| n.accesskit_node().value())
            .collect();
        assert_eq!(shown, vec![title.to_string()]);
        let closes: Vec<egui::accesskit::NodeId> = harness
            .state()
            .registry
            .records()
            .iter()
            .filter(|r| r.info.kind == "Button · dialog close")
            .map(|r| r.id.accesskit_id())
            .collect();
        harness
            .query_all(By::new().predicate(move |n| closes.contains(&n.locate().0)))
            .next()
            .expect("the title row's close button")
            .click();
        harness.run_steps(2);
        let app = harness.state();
        assert!(
            app.palette.is_none() && !app.about_open,
            "{title}: the close button left it open"
        );
    }
}

/// Where a Ghost button is found: the `n`th record of a kind, or the first button of an
/// AccessKit label, for one no record names alone or one the content draws unrecorded.
#[derive(Clone, Copy, Debug)]
enum GhostAt {
    Kind(&'static str, usize),
    Label(&'static str),
}

/// A Ghost button's global rect, and its record's `not_themeable` lines where it is recorded.
fn ghost_at(
    harness: &Harness<'_, App>,
    at: GhostAt,
) -> (egui::Rect, Option<Vec<(&'static str, String)>>) {
    let records = harness.state().registry.records();
    match at {
        GhostAt::Kind(kind, n) => {
            let record = records
                .iter()
                .filter(|r| r.info.kind == kind)
                .nth(n)
                .unwrap_or_else(|| panic!("no record {n} of kind {kind}"));
            let rect = harness
                .ctx
                .layer_transform_to_global(record.layer)
                .map_or(record.rect, |t| t.mul_rect(record.rect));
            (rect, Some(record.info.not_themeable.clone()))
        }
        GhostAt::Label(label) => {
            let node = harness
                .query_all_by_role_and_label(Role::Button, label)
                .next()
                .unwrap_or_else(|| panic!("no {label} button"));
            let id = node.accesskit_node().locate().0;
            let not_themeable = records
                .iter()
                .find(|r| r.id.accesskit_id() == id)
                .map(|r| r.info.not_themeable.clone());
            (node.rect(), not_themeable)
        }
    }
}

/// A Ghost button keeps its size in every state. egui lays a `frame_when_inactive(false)`
/// button out at rest without its frame and hovered or pressed with it, whose stroke adds to
/// its size (`egui/src/widgets/button.rs:364-368`), so each Ghost button is drawn in a Ghost
/// scope instead (`demo::ghost`), its record noting it: the rect with the pointer off it is the
/// rect with the pointer on it and pressing it, for each kind, on the screen that draws it.
#[test]
fn a_ghost_button_keeps_its_size_when_hovered_and_pressed() {
    type Setup = fn(&mut Harness<'static, App>);
    let none: Setup = |_| {};
    let panel_hidden: Setup = |h| h.state_mut().side_panel_visible = false;
    let about: Setup = |h| {
        let ctx = h.ctx.clone();
        h.state_mut().run_action(Action::OpenAbout, &ctx);
    };
    let info_shown: Setup = |h| {
        let pos = centre_of(h, "Label (typography)");
        hover_and_settle(h, pos);
        assert!(shown_id(h).is_some(), "Widget Info shows nothing to copy");
    };
    let cases: [(&str, Setup, GhostAt); 6] = [
        ("toolbar", none, GhostAt::Label("Command Palette")),
        (
            "side panel toggle, panel shown",
            none,
            GhostAt::Label("Toggle Side Panel"),
        ),
        (
            "side panel toggle, panel hidden",
            panel_hidden,
            GhostAt::Label("Toggle Side Panel"),
        ),
        (
            "Pages menu button",
            none,
            GhostAt::Kind("Menu button · Pages", 0),
        ),
        (
            "dialog close",
            about,
            GhostAt::Kind("Button · dialog close", 0),
        ),
        ("Widget Info's Copy", info_shown, GhostAt::Label("Copy")),
    ];
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        for (what, setup, at) in cases {
            let mut harness = open(theme, cli(&[("--theme", TEST_PRESET)]));
            harness.run_steps(4);
            setup(&mut harness);
            harness.run_steps(4);
            harness.remove_cursor();
            harness.run_steps(2);
            let (rest, not_themeable) = ghost_at(&harness, at);
            harness.hover_at(rest.center());
            harness.run_steps(2);
            let (hovered, _) = ghost_at(&harness, at);
            harness.drag_at(rest.center());
            harness.run_steps(2);
            let (pressed, _) = ghost_at(&harness, at);
            assert_eq!(
                (hovered, pressed),
                (rest, rest),
                "{what} under {theme:?}: hovered and pressed, not its resting rect"
            );
            if let Some(not_themeable) = not_themeable {
                assert!(
                    not_themeable
                        .iter()
                        .any(|(k, _)| *k == crate::demo::GHOST_NOTE.0),
                    "{what} under {theme:?}: its record does not note it Ghost: {not_themeable:?}"
                );
            }
        }
    }
}

/// The Basic page's check boxes and radio buttons, and the rows they sit in, are as tall as the
/// indicator the theme states or their label, not a push button's height: the model states no
/// row height for either (kde-breeze light: a 20 `checkbox.indicator(NONE).min_size` beside a
/// push button drawn 32 tall, whose `button.body(NONE).min_size[1]` is unstated; material-3
/// light: 18 under a stated 40).
#[test]
fn basic_checkbox_and_radio_rows_fit_the_indicator() -> Result<(), String> {
    use native_theme::next::WidgetState;
    for preset in ["kde-breeze", "material-3"] {
        let harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .cloned()
            .ok_or_else(|| format!("{preset}: no 0.8.0 theme installed"))?;
        let indicator = r.checkbox.indicator(WidgetState::NONE).min_size[0]
            .ok_or_else(|| format!("{preset} states no checkbox.indicator(NONE).min_size[0]"))?;
        let records = harness.state().registry.records();
        let rects = |kind: &str| -> Vec<egui::Rect> {
            records
                .iter()
                .filter(|r| r.info.kind == kind)
                .map(|r| r.rect)
                .collect()
        };
        let (checkboxes, radios) = (rects("checkbox (unchecked)"), rects("RadioButton"));
        assert!(
            !checkboxes.is_empty() && radios.len() == 2,
            "{preset}: the controls were not recorded"
        );
        let button = rects("button (enabled)");
        let tallest_label = r.checkbox.label(WidgetState::NONE).size_px
            * native_theme_egui::next::line_height(&r)
            + 1.0;
        for rect in checkboxes.iter().chain(&radios) {
            assert!(
                rect.height() >= indicator - 0.01,
                "{preset}: {rect:?} is shorter than the indicator"
            );
            assert!(
                rect.height() <= indicator.max(tallest_label),
                "{preset}: {rect:?} is taller than the indicator and its label"
            );
            assert!(
                button.iter().all(|b| rect.height() < b.height()),
                "{preset}: {rect:?} is as tall as a push button"
            );
        }
        // One control per row, each row `layout.widget_gap` below the one above: a row as tall as
        // a push button would leave more room between the controls than the gap.
        let gap = r
            .layout
            .widget_gap
            .ok_or_else(|| format!("{preset} states no layout.widget_gap"))?;
        let mut rows: Vec<egui::Rect> = [
            "checkbox (unchecked)",
            "checkbox (checked)",
            "checkbox (disabled)",
        ]
        .iter()
        .flat_map(|kind| rects(kind))
        .collect();
        rows.sort_by(|a, b| a.top().total_cmp(&b.top()));
        for pair in rows.windows(2).chain(radios.windows(2)) {
            let [above, below] = pair else { continue };
            assert!(
                (below.top() - above.bottom() - gap).abs() < 0.5,
                "{preset}: {below:?} lies {} below {above:?}, widget_gap is {gap}",
                below.top() - above.bottom()
            );
        }
    }
    Ok(())
}

/// A note of the atlas a widget's record carries under *Not themeable* is a line of prose that
/// names the value's home, not the note's `Debug` form: the Basic text area's frame of corners
/// whose radii differ corner by corner.
#[test]
fn a_records_atlas_note_is_prose_naming_the_home() -> Result<(), String> {
    use native_theme::next::{
        CornerCurve, CornerRadius, Corners, InputRule, PartPatch, ThemePatch,
    };
    let corners = Corners {
        radius: CornerRadius::PerCorner {
            top_left: 2.0,
            top_right: 6.0,
            bottom_right: 6.0,
            bottom_left: 6.0,
        },
        curve: CornerCurve::Circular,
    };
    let (harness, _) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", |_| {
        Some(ThemePatch::new().text_area(InputRule::new().field(PartPatch::new().corners(corners))))
    })?;
    let notes: Vec<String> = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "TextEdit (multiline)")
        .flat_map(|r| r.info.not_themeable.iter())
        .filter(|(what, _)| *what == "frame")
        .map(|(_, note)| note.clone())
        .collect();
    assert_eq!(
        notes,
        vec!["text_area.field(NONE).corners: stated in a form egui cannot draw".to_owned()],
        "the text area's frame notes"
    );
    Ok(())
}

/// The inside of a list frame a pill rounds is a pill, the largest radius egui holds, not that
/// radius less the frame's stroke; a frame of a length radius keeps the radius less its stroke.
#[test]
fn a_pill_list_frames_inside_is_a_pill() -> Result<(), String> {
    use native_theme::next::{
        Corners, ListRule, ListVariant, PartPatch, Preset, Stroke, Theme, ThemeContext, ThemePatch,
    };
    let theme = Theme::preset(Preset::KdeBreeze);
    let r = theme.resolve(&ThemeContext::light());
    let width = r.layout.border_width;
    if width <= 0.0 {
        return Err(format!(
            "kde-breeze's layout.border_width is {width}, no stroke to subtract"
        ));
    }
    let border = r
        .colors
        .window
        .border
        .ok_or("kde-breeze states no window border")?;
    let frame_of = |corners: Corners| {
        let patch = ThemePatch::new().list(
            ListRule::new().variant(ListVariant::Plain).container(
                PartPatch::new()
                    .corners(corners)
                    .stroke(Stroke::uniform(width, border)),
            ),
        );
        let patched = theme
            .clone()
            .with_patch(patch)
            .resolve(&ThemeContext::light());
        crate::pages::basic::list_frame(Some(&patched), ListVariant::Plain)
    };
    let pill = f32::from(u8::MAX);
    let (frame, inner) = frame_of(Corners::pill());
    assert_eq!(
        (frame.stroke.width, frame.corner_radius, inner),
        (width, egui::CornerRadius::same(u8::MAX), pill),
        "a pill frame's inside is not a pill"
    );
    let length = 4.0 * width;
    let (frame, inner) = frame_of(Corners::uniform(length));
    assert_eq!(
        (frame.stroke.width, inner),
        (width, length - width),
        "a frame of radius {length} has not its radius less its stroke inside"
    );
    Ok(())
}

/// The Basic page applies the leaves egui never reads from the style, per call, as the connector
/// spec asks of the application (§5.3, §5.4), of the installed 0.8.0 theme: each push button is
/// at least `button.body(NONE).min_size` where stated, each text field
/// `input.field(NONE).min_size[1]` tall where stated; the disabled field is filled with
/// `input.field(DISABLED).fill` where a rule states it, else its fill at rest, at
/// `input.disabled_opacity`, its frame built in the disabled cell; and the link's text is
/// underlined, at rest, exactly where `link.label(NONE)` states an underline decoration.
#[test]
fn the_basic_page_applies_the_per_call_leaves() -> Result<(), String> {
    use crate::demo::{drawn_fill, drawn_state_fill};
    use native_theme::next::{DecorationKind, TextStated, WidgetState};
    // whether a preset states `button.body(NONE).min_size`'s width, its height and
    // `input.field(NONE).min_size[1]`, so no check below passes on an unstated value alone
    let mut stated = [false; 3];
    for preset in ["kde-breeze", "material-3"] {
        let mut harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        harness.remove_cursor();
        harness.run_steps(2);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .cloned()
            .ok_or_else(|| format!("{preset}: no 0.8.0 theme installed"))?;
        let [min_width, min_height] = r.button.body(WidgetState::NONE).min_size;
        let field_height = r.input.field(WidgetState::NONE).min_size[1];
        for (seen, value) in stated.iter_mut().zip([min_width, min_height, field_height]) {
            *seen |= value.is_some();
        }
        let rect_of = |kind: &str| {
            harness
                .state()
                .registry
                .records()
                .iter()
                .find(|r| r.info.kind == kind)
                .map(|r| r.rect)
                .unwrap_or_else(|| panic!("{preset}: no record of kind {kind}"))
        };
        for kind in [
            "button (enabled)",
            "button (suggested action)",
            "button (disabled)",
            "tooltip button",
        ] {
            let rect = rect_of(kind);
            assert!(
                min_width.is_none_or(|w| rect.width() >= w - 0.01)
                    && min_height.is_none_or(|h| rect.height() >= h - 0.01),
                "{preset}: {kind} is {rect:?}, under button.body(NONE).min_size \
                 {min_width:?} x {min_height:?}"
            );
        }
        for kind in [
            "TextEdit (hint)",
            "TextEdit (single line)",
            "TextEdit (disabled)",
        ] {
            let rect = rect_of(kind);
            assert!(
                field_height.is_none_or(|h| rect.height() >= h - 0.01),
                "{preset}: {kind} is {rect:?}, under input.field(NONE).min_size[1] {field_height:?}"
            );
        }
        let disabled = rect_of("TextEdit (disabled)");
        // material-3's outlined field has no disabled container (docs/platform-facts.md §2.4),
        // so it keeps its fill at rest. egui fades the disabled field by
        // `input.disabled_opacity` on top (docs/platform-facts.md §2.1.6).
        let opacity = r
            .input
            .disabled_opacity
            .ok_or_else(|| format!("{preset} states no input.disabled_opacity"))?;
        let fill = drawn_state_fill(&r.input.field(WidgetState::DISABLED))
            .or_else(|| drawn_fill(&r.input.field(WidgetState::NONE)))
            .map(|fill| fill.gamma_multiply(opacity));
        let shapes = harness.output().shapes.clone();
        let filled = shapes.iter().any(|clipped| match &clipped.shape {
            egui::Shape::Rect(r) => {
                Some(r.fill) == fill && disabled.contains_rect(r.rect.shrink(1.0))
            }
            _ => false,
        });
        assert!(
            fill.is_some() && filled,
            "{preset}: the disabled field is not filled with {fill:?}"
        );
        let rest = r.link.label(WidgetState::NONE);
        let underline = rest.stated.contains(TextStated::DECORATION)
            && rest.decoration.kind == DecorationKind::Underline;
        let underlined: Vec<bool> = shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) if text.galley.job.text == "Link" => Some(
                    text.galley
                        .job
                        .sections
                        .iter()
                        .all(|s| s.format.underline.width > 0.0),
                ),
                _ => None,
            })
            .collect();
        assert_eq!(
            underlined,
            vec![underline],
            "{preset}: the link's underline at rest is not link.label(NONE)'s decoration"
        );
    }
    for (seen, leaf) in stated.into_iter().zip([
        "button.body(NONE).min_size[0]",
        "button.body(NONE).min_size[1]",
        "input.field(NONE).min_size[1]",
    ]) {
        if !seen {
            return Err(format!(
                "neither kde-breeze nor material-3 states {leaf}: its check checked nothing"
            ));
        }
    }
    Ok(())
}

/// M9-106 and M9-108: the per-call leaves the connector leaves to the application
/// (`src/mapping.md`, `dialog.min_width` … `input.min_height`, DERIVED) reach the palette pages
/// as they reach Basic. Under windows-11 set directly, which states every one: the Overlays
/// page's modal is at least the 0.8.0 theme's `dialog.panel.min_size` and at most its
/// `max_size`, under a patch whose least width is past the unpatched preset's
/// `dialog.panel.max_size[0]`; every `Role::Button` kind of the Buttons and Overlays pages built with
/// `Button::new` is at least the 0.8.0 theme's `button.body(NONE).min_size[0]` wide, there and
/// under kde-breeze, whose minimum is wider than egui's buttons; and every `TextEdit (…)` of the
/// Inputs page at least `input.field(NONE).min_size[1]` tall, there and under adwaita and
/// material-3, whose fields are taller than egui's. egui's own kinds and its small button keep egui's sizes, as the gpui showcase's small
/// button keeps gpui-component's, each saying so under *Not themeable*.
#[test]
fn the_palette_pages_apply_the_per_call_leaves() -> Result<(), String> {
    fn rects(harness: &Harness<'_, App>, kind: &str) -> Vec<egui::Rect> {
        let rects: Vec<egui::Rect> = harness
            .state()
            .registry
            .records()
            .iter()
            .filter(|r| r.info.kind == kind)
            .map(|r| r.rect)
            .collect();
        assert!(!rects.is_empty(), "no record of kind {kind}");
        rects
    }
    fn assert_noted(harness: &Harness<'_, App>, kind: &str, leaf: &str) {
        let notes: Vec<String> = harness
            .state()
            .registry
            .records()
            .iter()
            .filter(|r| r.info.kind == kind)
            .flat_map(|r| r.info.not_themeable.iter())
            .map(|(what, note)| format!("{what}: {note}"))
            .collect();
        assert!(
            notes
                .iter()
                .any(|n| n.contains(leaf) && n.contains("not applied")),
            "{kind}: no Not themeable line says {leaf} is not applied: {notes:?}"
        );
    }
    let wide_enough = |harness: &Harness<'_, App>, kinds: &[&str]| -> Result<(), String> {
        let [min_width, _] = harness
            .state()
            .atlas
            .next_resolved(harness.ctx.theme())
            .ok_or("no 0.8.0 theme installed")?
            .button
            .body(native_theme::next::WidgetState::NONE)
            .min_size;
        let min_width = min_width.ok_or("no button.body(NONE).min_size[0] stated")?;
        assert!(min_width > 0.0, "a least width that is no case");
        for kind in kinds {
            for rect in rects(harness, kind) {
                assert!(
                    rect.width() >= min_width - 0.01,
                    "{kind} is {rect:?}, narrower than button.body(NONE).min_size[0] {min_width}"
                );
            }
        }
        Ok(())
    };

    // windows-11's buttons are as wide as its 64 px already; kde-breeze's 82 px is wider.
    for preset in ["windows-11", "kde-breeze"] {
        let harness = open_page_under(Page::Buttons, egui::Theme::Light, preset);
        wide_enough(
            &harness,
            &[
                "button (enabled)",
                "button (icon and text)",
                "button (frameless)",
                "button (shortcut text)",
                "button (suggested action)",
                "button (disabled)",
            ],
        )?;
        for kind in [
            "ui.button",
            "ui.small_button",
            "ui.toggle_value",
            "button (small)",
        ] {
            assert_noted(&harness, kind, "button.body(NONE).min_size");
        }
    }

    // windows-11's `dialog.panel` widths patched past the unpatched preset's
    // `dialog.panel.max_size[0]`, so a modal sized by the unpatched extents is too narrow
    let mut plain_max_w = None;
    let (mut harness, r) = open_patched(Page::Overlays, egui::Theme::Light, "windows-11", |r| {
        use native_theme::next::{DialogPatch, PartPatch, ThemePatch, WidgetPatchMap};
        let [max_w, _] = r.dialog.panel.max_size;
        plain_max_w = max_w;
        let min_w = max_w? + 50.0;
        Some(
            ThemePatch::new().widget_rules(
                WidgetPatchMap::new().dialog(
                    DialogPatch::new().panel(
                        PartPatch::new()
                            .min_size([Some(min_w), r.dialog.panel.min_size[1]])
                            .max_size([Some(min_w + 50.0), r.dialog.panel.max_size[1]]),
                    ),
                ),
            ),
        )
    })?;
    let panel = &r.dialog.panel;
    let ([Some(min_w), Some(min_h)], [Some(max_w), Some(max_h)]) = (panel.min_size, panel.max_size)
    else {
        return Err("windows-11 states not every dialog.panel extent".to_owned());
    };
    assert!(
        plain_max_w.is_some_and(|plain| min_w > plain),
        "the patched dialog.panel.min_size[0] {min_w} is not past the unpatched \
         dialog.panel.max_size[0] {plain_max_w:?}"
    );
    wide_enough(
        &harness,
        &[
            "open window button",
            "open modal button",
            "open popup button",
            "tooltip button",
        ],
    )?;
    harness.get_by_label("Open modal").click();
    harness.run_steps(SETTLE);
    wide_enough(&harness, &["modal button"])?;
    for modal in rects(&harness, "modal") {
        assert!(
            modal.width() >= min_w - 0.01
                && modal.width() <= max_w + 0.01
                && modal.height() >= min_h - 0.01
                && modal.height() <= max_h + 0.01,
            "the modal is {:?}, outside dialog.panel.min_size..max_size {min_w}..{max_w} x \
             {min_h}..{max_h}",
            modal.size(),
        );
    }
    harness.key_press(egui::Key::Escape);
    harness.run_steps(SETTLE);
    harness.get_by_label("Open popup").click();
    harness.run_steps(SETTLE);
    wide_enough(&harness, &["popup item"])?;

    // windows-11's fields are as tall as egui's one row and margin already; adwaita's, which
    // states no vertical padding, and material-3's, the tallest stated, are taller than that.
    for preset in ["windows-11", "adwaita", "material-3"] {
        let harness = open_page_under(Page::Inputs, egui::Theme::Light, preset);
        let [_, min_height] = harness
            .state()
            .atlas
            .next_resolved(harness.ctx.theme())
            .ok_or("no 0.8.0 theme installed")?
            .input
            .field(native_theme::next::WidgetState::NONE)
            .min_size;
        let min_height =
            min_height.ok_or_else(|| format!("{preset}: no input.field(NONE).min_size[1]"))?;
        assert!(min_height > 0.0, "{preset}: a least height that is no case");
        for kind in [
            "TextEdit (single line)",
            "TextEdit (multiline)",
            "TextEdit (password)",
            "TextEdit (hint)",
            "TextEdit (disabled)",
        ] {
            for rect in rects(&harness, kind) {
                assert!(
                    rect.height() >= min_height - 0.01,
                    "{preset}: {kind} is {rect:?}, under input.field(NONE).min_size[1] \
                     {min_height}"
                );
            }
        }
        for kind in [
            "ui.text_edit_singleline",
            "ui.text_edit_multiline",
            "ui.code_editor",
        ] {
            assert_noted(&harness, kind, "input.field(NONE).min_size[1]");
        }
    }
    Ok(())
}

/// The Overlays page's modal lays its buttons out in the installed 0.8.0 theme's
/// `dialog.button_order`: `modal_buttons` puts the primary "OK" at the end each order names, and
/// where the theme states none, the buttons in the page's order with a message that says so;
/// the unpatched kde-breeze states primary left, and patched to primary right, "Cancel" is
/// drawn left of "OK".
#[test]
fn the_modal_takes_the_08_button_order_or_says_it_is_unstated() -> Result<(), String> {
    use crate::pages::overlays::modal_buttons;
    use native_theme::next::{DialogButtonOrder, DialogPatch, ThemePatch, WidgetPatchMap};
    assert_eq!(
        modal_buttons(Some(DialogButtonOrder::PrimaryRight)).0,
        ["Cancel", "OK"]
    );
    assert_eq!(
        modal_buttons(Some(DialogButtonOrder::PrimaryLeft)).0,
        ["OK", "Cancel"]
    );
    let (labels, message) = modal_buttons(None);
    assert_eq!(labels, ["OK", "Cancel"]);
    assert!(
        message.contains("states no dialog.button_order"),
        "the message does not say the order is unstated: {message}"
    );
    for (_, said) in [
        modal_buttons(Some(DialogButtonOrder::PrimaryRight)),
        modal_buttons(Some(DialogButtonOrder::PrimaryLeft)),
    ] {
        assert!(!said.contains("states no"), "a stated order said unstated");
    }

    let unpatched = open_page_under(Page::Overlays, egui::Theme::Light, "kde-breeze")
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .map(|r| r.dialog.button_order)
        .ok_or("no 0.8.0 theme installed")?;
    assert_eq!(
        unpatched,
        Some(DialogButtonOrder::PrimaryLeft),
        "the unpatched kde-breeze is not primary left, so the patch proves nothing"
    );
    let (mut harness, r) = open_patched(Page::Overlays, egui::Theme::Light, "kde-breeze", |_| {
        Some(
            ThemePatch::new().widget_rules(
                WidgetPatchMap::new()
                    .dialog(DialogPatch::new().button_order(DialogButtonOrder::PrimaryRight)),
            ),
        )
    })?;
    assert_eq!(r.dialog.button_order, Some(DialogButtonOrder::PrimaryRight));
    harness.get_by_label("Open modal").click();
    harness.run_steps(SETTLE);
    let (cancel, ok) = (
        harness.get_by_label("Cancel").rect(),
        harness.get_by_label("OK").rect(),
    );
    assert!(
        cancel.center().x < ok.center().x,
        "Cancel at {cancel:?} is not left of OK at {ok:?} under the patched primary right"
    );
    Ok(())
}

/// The Overlays page's menu-bar icons are drawn at the installed 0.8.0 theme's
/// `menu.icon(NONE).size`: under kde-breeze with the bundled Material set and the size patched to
/// twice the preset's own, which the unpatched `menu.icon(NONE).size` is not, the
/// `ui.menu_image_button`, an icon alone in its button, is wider than unpatched by what the icon
/// grows; and without an installed 0.8.0 theme no size is taken.
#[cfg(feature = "material-icons")]
#[test]
fn the_menu_icons_take_the_08_menu_icon_size() -> Result<(), String> {
    use crate::pages::overlays::menu_icon_size;
    use native_theme::next::{IconPatch, MenuRule, ThemePatch, WidgetState};
    assert_eq!(menu_icon_size(None), None);
    let width = |harness: &Harness<'_, App>| -> Result<f32, String> {
        harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.info.kind == "ui.menu_image_button")
            .map(|r| r.rect.width())
            .ok_or_else(|| "no ui.menu_image_button drawn".to_owned())
    };
    let material = |harness: &mut Harness<'static, App>| {
        harness.state_mut().settings.icon = IconSetChoice::Material;
        harness.run_steps(SETTLE);
    };

    let mut plain = open_page_under(Page::Overlays, egui::Theme::Light, "kde-breeze");
    material(&mut plain);
    let old = menu_icon_size(plain.state().atlas.next_resolved(egui::Theme::Light))
        .ok_or("the unpatched kde-breeze states no menu.icon(NONE).size")?;
    let (mut harness, r) = open_patched(Page::Overlays, egui::Theme::Light, "kde-breeze", |r| {
        let size = menu_icon_size(Some(r))?;
        Some(ThemePatch::new().menu(MenuRule::new().icon(IconPatch::new().size(2.0 * size))))
    })?;
    material(&mut harness);
    let size = menu_icon_size(Some(&r)).ok_or("the patch states no menu.icon(NONE).size")?;
    assert_eq!(size, r.menu.icon(WidgetState::NONE).size);
    assert!(
        (size - old).abs() > 0.5,
        "the patched {size} is the unpatched menu.icon(NONE).size {old}"
    );
    // the button holds its icon and padding alone, so it grows by what the icon grows
    let (unpatched, patched) = (width(&plain)?, width(&harness)?);
    assert!(
        (patched - unpatched - size / 2.0).abs() < 0.5,
        "the image button grows from {unpatched} to {patched}, not by the icon's growth from \
         the preset's {} to the patched menu.icon(NONE).size {size}",
        size / 2.0
    );
    Ok(())
}

/// The least sizes the Buttons and Inputs pages give are the installed 0.8.0 theme's, not the
/// unpatched preset's: under kde-breeze with `button.body(NONE).min_size[0]` patched to
/// half as wide again as the preset's own, which the first row of the Buttons page still holds
/// (a `Button` is no wider than the room its row has left), and `input.field(NONE).min_size[1]`
/// to twice the preset's own ([`open_patched`]), so neither is the unpatched theme's any more,
/// every button the
/// Buttons page builds with `Button::new` is at least the patched width and every field of the
/// Inputs page at least the patched height, where unpatched the plain button is narrower and
/// the single-line field lower than that.
#[test]
fn the_least_sizes_are_the_08_themes() -> Result<(), String> {
    use native_theme::next::{ButtonRule, InputRule, PartPatch, ThemePatch, WidgetState};
    let rect_of = |harness: &Harness<'_, App>, kind: &str| -> Result<egui::Rect, String> {
        harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.info.kind == kind)
            .map(|r| r.rect)
            .ok_or_else(|| format!("no record of kind {kind}"))
    };

    let plain = open_page_under(Page::Buttons, egui::Theme::Light, "kde-breeze");
    let old_width = plain
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .and_then(|r| r.button.body(WidgetState::NONE).min_size[0])
        .ok_or("the unpatched kde-breeze states no button.body(NONE).min_size[0]")?;
    let wider =
        |r: &native_theme::next::ResolvedTheme| {
            let [width, _] = r.button.body(WidgetState::NONE).min_size;
            Some(ThemePatch::new().button(
                ButtonRule::new().body(PartPatch::new().min_size([Some(1.5 * width?), None])),
            ))
        };
    let (basic, _) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", wider)?;
    let (harness, r) = open_patched(Page::Buttons, egui::Theme::Light, "kde-breeze", wider)?;
    let [width, _] = r.button.body(WidgetState::NONE).min_size;
    let width = width.ok_or("the patch states no button.body(NONE).min_size[0]")?;
    assert!(
        width > old_width,
        "the patched {width} is not wider than the unpatched button.body(NONE).min_size[0] \
         {old_width}"
    );
    let unpatched = rect_of(&plain, "button (enabled)")?.width();
    assert!(
        unpatched < width - 0.01,
        "the plain button is {unpatched} wide unpatched, so the patch proves nothing"
    );
    for kind in [
        "button (enabled)",
        "button (icon and text)",
        "button (frameless)",
        "button (shortcut text)",
        "button (suggested action)",
        "button (disabled)",
    ] {
        let rect = rect_of(&harness, kind)?;
        assert!(
            rect.width() >= width - 0.01,
            "{kind} is {rect:?}, narrower than the patched button.body(NONE).min_size[0] {width}"
        );
    }
    for kind in [
        "button (enabled)",
        "button (suggested action)",
        "button (disabled)",
        "tooltip button",
        "toggle button (on)",
    ] {
        let rect = rect_of(&basic, kind)?;
        assert!(
            rect.width() >= width - 0.01,
            "Basic: {kind} is {rect:?}, narrower than the patched button.body(NONE).min_size[0] \
             {width}"
        );
    }

    let plain = open_page_under(Page::Inputs, egui::Theme::Light, "kde-breeze");
    let old_height = plain
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .and_then(|r| r.input.field(WidgetState::NONE).min_size[1])
        .ok_or("the unpatched kde-breeze states no input.field(NONE).min_size[1]")?;
    let taller =
        |r: &native_theme::next::ResolvedTheme| {
            let [_, height] = r.input.field(WidgetState::NONE).min_size;
            Some(ThemePatch::new().input(
                InputRule::new().field(PartPatch::new().min_size([None, Some(2.0 * height?)])),
            ))
        };
    let (basic, _) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", taller)?;
    let (harness, r) = open_patched(Page::Inputs, egui::Theme::Light, "kde-breeze", taller)?;
    let [_, height] = r.input.field(WidgetState::NONE).min_size;
    let height = height.ok_or("the patch states no input.field(NONE).min_size[1]")?;
    assert!(
        height > old_height,
        "the patched {height} is not taller than the unpatched input.field(NONE).min_size[1] \
         {old_height}"
    );
    let unpatched = rect_of(&plain, "TextEdit (single line)")?.height();
    assert!(
        unpatched < height - 0.01,
        "the single-line field is {unpatched} tall unpatched, so the patch proves nothing"
    );
    for kind in [
        "TextEdit (single line)",
        "TextEdit (multiline)",
        "TextEdit (password)",
        "TextEdit (hint)",
        "TextEdit (disabled)",
    ] {
        let rect = rect_of(&harness, kind)?;
        assert!(
            rect.height() >= height - 0.01,
            "{kind} is {rect:?}, under the patched input.field(NONE).min_size[1] {height}"
        );
    }
    for kind in [
        "TextEdit (hint)",
        "TextEdit (single line)",
        "TextEdit (disabled)",
        "TextEdit (focused)",
        "DragValue",
    ] {
        let rect = rect_of(&basic, kind)?;
        assert!(
            rect.height() >= height - 0.01,
            "Basic: {kind} is {rect:?}, under the patched input.field(NONE).min_size[1] {height}"
        );
    }
    Ok(())
}

/// The Inputs page builds each field's frame inside its `Role::Input` scope, as
/// `next::input_frame` and `next::text_area_frame` ask, so the frame takes the field's border,
/// not the button's the base style holds: under the 0.8.0 material-3, whose button has no line
/// and whose field has one, each field's frame (the `Shape::Rect` in the field's fill inside its
/// rectangle) is stroked at `input.field(NONE)`'s (`text_area.field(NONE)`'s) stroke width and
/// rounded by its corners.
#[test]
fn the_inputs_page_frames_its_fields_in_their_own_border() -> Result<(), String> {
    use crate::demo::drawn_fill;
    use native_theme::next::WidgetState;
    use native_theme_egui::next::{radius_of, stroke_width_of};
    let harness = open_page_under(Page::Inputs, egui::Theme::Light, "material-3");
    let r = harness
        .state()
        .atlas
        .next_resolved(harness.ctx.theme())
        .ok_or("no 0.8.0 theme installed")?;
    let (input, area) = (
        r.input.field(WidgetState::NONE),
        r.text_area.field(WidgetState::NONE),
    );
    assert_ne!(
        stroke_width_of(&input),
        stroke_width_of(&r.button.body(WidgetState::NONE)),
        "the case this covers: a field's line that is not the button's"
    );
    let fill = drawn_fill(&input).ok_or("no input.field(NONE).fill")?;
    for (kind, field) in [
        ("TextEdit (single line)", &input),
        ("TextEdit (password)", &input),
        ("TextEdit (hint)", &input),
        ("TextEdit (multiline)", &area),
    ] {
        let width = stroke_width_of(field).ok_or_else(|| format!("{kind}: no stroke width"))?;
        let radius = radius_of(field).ok_or_else(|| format!("{kind}: no corner radius"))?;
        let place = harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.info.kind == kind)
            .map(|r| r.rect)
            .ok_or_else(|| format!("no record of kind {kind}"))?;
        let frames: Vec<(f32, egui::CornerRadius)> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(r)
                    if r.fill == fill
                        && place.expand(0.01).contains_rect(r.rect)
                        && r.rect.width() > 0.5 * place.width() =>
                {
                    Some((r.stroke.width, r.corner_radius))
                }
                _ => None,
            })
            .collect();
        let want = (
            width,
            native_theme_egui::convert::to_corner_radius(egui::CornerRadius::ZERO, radius),
        );
        assert_eq!(frames, vec![want], "{kind}: its frame at {place:?}");
    }
    Ok(())
}

/// The showcase on `tab` with `preset` installed in place of the one it opened with, through the
/// Theme row's path under the Preferences dialog's text-scaling override `factor`
/// ([`install_under`]), at rest: a preset of another platform, which `--theme` does not take
/// here, at the scale an OS setting or that override installs.
fn open_scaled(preset: &str, tab: &str, factor: f32) -> Harness<'static, App> {
    open_scaled_in(egui::Theme::Light, preset, tab, factor)
}

/// [`open_scaled`] in `theme`.
fn open_scaled_in(
    theme: egui::Theme,
    preset: &str,
    tab: &str,
    factor: f32,
) -> Harness<'static, App> {
    let mut harness = open(theme, cli(&[("--theme", TEST_PRESET), ("--tab", tab)]));
    harness.run_steps(4);
    let mut prefs = harness.state().atlas.asked().clone();
    prefs.text_scaling_factor = factor;
    harness.state_mut().settings.prefs = Some(prefs);
    install_under(&mut harness, theme, preset);
    harness
}

/// A stated row height of the 0.8.0 theme `r`, resolved at its text scale, as the gpui
/// connector's table, list and menu rows take one (`geometry::table_row_height`,
/// `with_height_rule`): verbatim at
/// a text scale of 1 or less; above 1 at least the line box of `label`, its own line height
/// where the theme states one, else its size, which the resolve has scaled already, at the
/// theme's line height (`next::line_height`), rounded up, with `padding` (the row's top and
/// bottom together).
fn next_rule(
    r: &native_theme::next::ResolvedTheme,
    stated: f32,
    label: &native_theme::next::TextStyle,
    padding: f32,
) -> f32 {
    if r.context.text_scale <= 1.0 {
        return stated;
    }
    let line = if label
        .stated
        .contains(native_theme::next::TextStated::LINE_HEIGHT)
    {
        label.line_height
    } else {
        label.size_px * native_theme_egui::next::line_height(r)
    };
    stated.max(line.ceil() + padding)
}

/// Each of `weights` held under the family the showcase registers a face at that weight in
/// (`demo::weight_family`), with egui's own proportional face added there, so that family is in
/// the fonts on every host and in every feature set: the showcase registers the OS's face only
/// where the OS has one (`demo::Semibold`), and where the fonts hold none `demo::weighted_family`
/// asks for the proportional family, which is also what a text drawn at the body's weight is
/// in. With the family held, a text at another weight is in it exactly when the page asks for
/// its weight. Added after the last install, whose `set_fonts` would drop it, and in effect
/// from the next pass.
fn hold_weights(harness: &mut Harness<'_, App>, weights: &[u16]) {
    let own = egui::FontDefinitions::default();
    let face = own.families[&egui::FontFamily::Proportional][0].clone();
    let data = own.font_data[&face].clone();
    for weight in weights {
        harness.ctx.add_font(egui::epaint::text::FontInsert::new(
            &format!("test-weight-{weight}"),
            egui::FontData::clone(&data),
            vec![egui::epaint::text::InsertFontFamily {
                family: crate::demo::weight_family(*weight),
                priority: egui::epaint::text::FontPriority::Lowest,
            }],
        ));
    }
    harness.run_steps(2);
}

/// The family a text at `weight` over a `body`-weight body is asked in, with the face at that
/// weight held ([`hold_weights`]): the proportional one at the body's weight, else the one the
/// showcase registers at `weight` (`demo::weighted_family`).
fn weighted(weight: u16, body: u16) -> egui::FontFamily {
    if weight == body {
        egui::FontFamily::Proportional
    } else {
        crate::demo::weight_family(weight)
    }
}

/// The font of the painted galley of `text` whose top left lies in one of `within`.
fn galley_font(harness: &Harness<'_, App>, text: &str, within: &[egui::Rect]) -> egui::FontId {
    let shapes = harness.output().shapes.clone();
    let found = shapes.iter().find_map(|clipped| match &clipped.shape {
        egui::Shape::Text(shape)
            if shape.galley.job.text == text && within.iter().any(|r| r.contains(shape.pos)) =>
        {
            shape
                .galley
                .job
                .sections
                .first()
                .map(|s| s.format.font_id.clone())
        }
        _ => None,
    });
    found.unwrap_or_else(|| panic!("no {text:?} galley was painted"))
}

/// M9-102, M9-103: a table's header is drawn in its font at its weight and at the scaled size,
/// on all three tables, each of the installed 0.8.0 theme: under material-3 at a text-scaling
/// factor of 1.5, the Basic table's "Name" and the Data and Theme Map tables' header labels are
/// in `list.header_label(NONE)`'s size (`next::list_header_font`) and the face registered at its
/// weight (`demo::weighted_family`), stated apart from the body's.
/// The faces are held on every host ([`hold_weights`]), so a header drawn at the body's weight
/// fails here whatever faces the OS has.
#[test]
fn the_tables_headers_are_drawn_at_their_weight_and_the_scaled_size() -> Result<(), String> {
    let preset = "material-3";
    let mut basic = open_page_scaled(Page::Basic, preset, 1.5);
    let r = basic
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme installed")?
        .clone();
    let label = r.list.header_label(native_theme::next::WidgetState::NONE);
    let body = native_theme_egui::next::font_weight(&r).map_err(|e| e.to_string())?;
    hold_weights(&mut basic, &[label.weight]);
    assert_eq!(r.context.text_scale, 1.5, "the override is installed");
    let want = egui::FontId::new(
        native_theme_egui::next::list_header_font(&r)
            .map_err(|e| e.to_string())?
            .size,
        weighted(label.weight, body),
    );
    assert_ne!(
        label.weight, body,
        "{preset}: the header's weight is not the body's"
    );
    let cell = basic.state().registry.places()["basic.table.header.name"];
    assert_eq!(
        galley_font(&basic, "Name", &[cell]),
        want,
        "{preset}: the Basic table's header"
    );
    for (tab, page, kind, title) in [
        ("data", Page::Data, "table header", "Theme"),
        ("theme-map", Page::ThemeMap, "theme map header", "Leaf"),
    ] {
        let mut page = open_page_scaled(page, preset, 1.5);
        let r = page
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or("no 0.8.0 theme installed")?
            .clone();
        let label = r.list.header_label(native_theme::next::WidgetState::NONE);
        let body = native_theme_egui::next::font_weight(&r).map_err(|e| e.to_string())?;
        assert!(
            label
                .stated
                .contains(native_theme::next::TextStated::WEIGHT)
                && label.weight != body,
            "{preset}: the 0.8.0 header's weight is not stated apart from the body's"
        );
        hold_weights(&mut page, &[label.weight]);
        let want = egui::FontId::new(
            native_theme_egui::next::list_header_font(&r)
                .map_err(|e| e.to_string())?
                .size,
            weighted(label.weight, body),
        );
        let cells: Vec<egui::Rect> = page
            .state()
            .registry
            .records()
            .iter()
            .filter(|r| r.info.kind == kind)
            .map(|r| r.rect)
            .collect();
        assert_eq!(
            galley_font(&page, title, &cells),
            want,
            "{preset} {tab}: the header is not list.header_label(NONE)'s font"
        );
    }
    Ok(())
}

/// The Data and Theme Map tables' headers read the installed 0.8.0 theme: under kde-breeze with
/// `list.header_label(NONE)` patched to twice the preset's size and to `colors.window.border`, a
/// colour of the same theme ([`open_patched`]), neither of which the unpatched preset's
/// `list.header_label(NONE)` states, "Theme" and "Leaf" are drawn at the patched size in the
/// patched colour.
#[test]
fn the_tables_headers_take_the_patched_08_header_label() -> Result<(), String> {
    use native_theme::next::{ListRule, TextPatch, ThemePatch, WidgetState};
    for (page, kind, title) in [
        (Page::Data, "table header", "Theme"),
        (Page::ThemeMap, "theme map header", "Leaf"),
    ] {
        let mut plain = None;
        let (harness, r) = open_patched(page, egui::Theme::Light, "kde-breeze", |r| {
            plain = Some(r.list.header_label(WidgetState::NONE));
            let size = r.list.header_label(WidgetState::NONE).size_px;
            Some(
                ThemePatch::new().list(
                    ListRule::new().header_label(
                        TextPatch::new()
                            .size_px(2.0 * size)
                            .color(r.colors.window.border?),
                    ),
                ),
            )
        })?;
        let plain = plain.ok_or("the patch read no unpatched header label")?;
        let label = r.list.header_label(WidgetState::NONE);
        let colour = native_theme_egui::next::text_color_of(&label)
            .ok_or("the patch states no list.header_label(NONE).color")?;
        assert!(
            Some(colour) != native_theme_egui::next::text_color_of(&plain)
                && label.size_px != plain.size_px,
            "the patched header is the unpatched preset's, so the test could not tell them apart"
        );
        let cells: Vec<egui::Rect> = harness
            .state()
            .registry
            .records()
            .iter()
            .filter(|r| r.info.kind == kind)
            .map(|r| r.rect)
            .collect();
        assert_eq!(
            galley_font(&harness, title, &cells).size,
            label.size_px,
            "{kind}: not at the patched size"
        );
        let painted = harness
            .output()
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text)
                    if text.galley.job.text == title
                        && cells.iter().any(|r| r.contains(text.pos)) =>
                {
                    text.galley.job.sections.first().map(|s| s.format.color)
                }
                _ => None,
            });
        assert_eq!(painted, Some(colour), "{kind}: not in the patched colour");
    }
    Ok(())
}

/// M9-107: the Text page's text-role labels are drawn at the weight they name, of the installed
/// 0.8.0 theme (`demo::next_role_text`): under windows-11, whose display weight is not its
/// body's, each role's label names the 0.8.0 weight and is drawn in the face registered at it,
/// at its role's size. The faces at the
/// roles' weights are held on every host ([`hold_weights`]), so a label drawn at the body's
/// weight fails here whatever faces the OS has.
#[test]
fn the_text_role_labels_are_drawn_at_their_weight() -> Result<(), String> {
    use native_theme_egui::TextRole;
    use native_theme_egui::next;
    const ROLES: [(TextRole, &str); 4] = [
        (TextRole::Caption, "Caption"),
        (TextRole::SectionHeading, "Section heading"),
        (TextRole::DialogTitle, "Dialog title"),
        (TextRole::Display, "Display"),
    ];
    let mut harness = open_page_scaled(Page::Text, "windows-11", 1.0);
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme installed")?
        .clone();
    let weights: Vec<u16> = ROLES
        .iter()
        .map(|(role, _)| next::text_role_weight(&r, *role))
        .collect();
    hold_weights(&mut harness, &weights);
    let body = next::font_weight(&r).map_err(|e| e.to_string())?;
    assert_ne!(
        next::text_role_weight(&r, TextRole::Display),
        body,
        "windows-11's display weight is not the body's"
    );
    let labels: Vec<egui::Rect> = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "text role label")
        .map(|r| r.rect)
        .collect();
    for (role, name) in ROLES {
        let weight = next::text_role_weight(&r, role);
        let text = format!("{name}, weight {weight}");
        let want = egui::FontId::new(next::text_role_font(&r, role).size, weighted(weight, body));
        assert_eq!(galley_font(&harness, &text, &labels), want, "{text}");
    }
    Ok(())
}

/// The tables' header rows are `list_header_height` tall where the theme states one
/// (material-3, macos-monterey), else a row's height where the theme states one, else a row
/// ([`UNSTATED_PRESET`]): on the Basic page's painted table (its `basic.table.header` place) a row
/// of the header font, `list.header(NONE)`'s padding above and below its line box, a side the
/// theme leaves unstated egui_extras' own, half the List scope's `item_spacing.y`
/// (`pages/basic.rs`, `row_metrics`, `table_cell_inset`); on the Data and Theme Map pages'
/// `TableBuilder`s, all of the installed 0.8.0 theme, the same rows (`TableRows::of` through
/// `row_metrics`): its `list.header(NONE).min_size[1]`, else `list.row(NONE).min_size[1]`, else
/// the header font's line box and the header's padding above and below it, an unstated side
/// egui_extras' own ([`next_table_row`]). There
/// the header pushes the first body row down by exactly its height, the body's top the header's
/// bottom rounded to whole pixels as egui's scroll area lays its content: its cells are one line
/// in that line box, each centred in its row (`pages/data.rs`, `centred_cells`), and the rows lie
/// with no spacing between them, so a first-row cell's centre lies half a row below that top, on
/// egui's layout grid ([`CENTRE_SLACK`]). At a text-scaling
/// factor of 1.5 a stated height grows to hold its label, as gpui's table rows grow
/// ([`next_rule`]), and an unstated Basic header is a row of the header font at its scaled size.
#[test]
fn the_tables_headers_take_the_themes_header_height() -> Result<(), String> {
    for (preset, factor) in [
        ("material-3", 1.0),
        (UNSTATED_PRESET, 1.0),
        ("material-3", 1.5),
        (UNSTATED_PRESET, 1.5),
        ("macos-monterey", 1.5),
    ] {
        let basic = open_page_scaled(Page::Basic, preset, factor);
        let rb = basic
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or("no 0.8.0 theme installed")?
            .clone();
        let stated = native_theme_egui::next::list_header_height(&rb);
        assert_eq!(
            stated.is_some(),
            preset != UNSTATED_PRESET,
            "{preset}: the preset's header height"
        );
        let none = native_theme::next::WidgetState::NONE;
        let header_label = rb.list.header_label(none);
        let [top, _, bottom, _] = native_theme_egui::next::padding_of(&rb.list.header(none));
        // an unstated side egui_extras' own, half the List scope's item spacing
        let own = 0.5
            * scope_style(&basic, native_theme_egui::Role::List)?
                .spacing
                .item_spacing
                .y;
        let padding = top.unwrap_or(own) + bottom.unwrap_or(own);
        let size = native_theme_egui::next::list_header_font(&rb)
            .map_err(|e| e.to_string())?
            .size;
        let want = match stated.or(rb.list.row(none).min_size[1]) {
            Some(h) => next_rule(&rb, h, &header_label, padding + rb.separator.thickness),
            None => crate::demo::next_line_box(&rb, &header_label, size) + padding,
        };
        let placed = basic.state().registry.places()["basic.table.header"];
        assert!(
            (placed.height() - want).abs() <= 0.5,
            "{preset} at {factor}: the Basic table's header is {} tall, want {want}",
            placed.height()
        );

        for (tab, page, header_kind, cell_kind) in [
            ("data", Page::Data, "table header", "table cell"),
            (
                "theme-map",
                Page::ThemeMap,
                "theme map header",
                "theme map cell",
            ),
        ] {
            use native_theme::next::WidgetState;
            let page = open_page_scaled(page, preset, factor);
            let r = page
                .state()
                .atlas
                .next_resolved(egui::Theme::Light)
                .ok_or("no 0.8.0 theme installed")?
                .clone();
            let l = &r.list;
            let stated = native_theme_egui::next::list_header_height(&r);
            assert_eq!(
                stated.is_some(),
                preset != UNSTATED_PRESET,
                "{preset}: the 0.8.0 preset's header height"
            );
            let item = l.item_label(WidgetState::NONE);
            let size = native_theme_egui::next::list_header_font(&r)
                .map_err(|e| format!("{preset}: {e}"))?
                .size;
            let header = next_table_row(&page, &r, true, size)?;
            let row = next_table_row(&page, &r, false, item.size_px)?;
            let records = page.state().registry.records();
            let centre = |kind: &str| {
                records
                    .iter()
                    .filter(|r| r.info.kind == kind)
                    .map(|r| r.rect.center().y)
                    .fold(f32::INFINITY, f32::min)
            };
            let header_centre = centre(header_kind);
            let pushed = centre(cell_kind) - header_centre;
            // the body's scroll area lays its content from the header's bottom rounded to
            // whole pixels (egui 0.37.0 `src/containers/scroll_area.rs:880-883`)
            let px = page.ctx.pixels_per_point();
            let body_top = ((header_centre + 0.5 * header) * px).round() / px;
            let want = body_top - header_centre + 0.5 * row;
            assert!(
                (pushed - want).abs() <= CENTRE_SLACK,
                "{preset} at {factor} {tab}: the first row's centre lies {pushed} below the \
                 header's, want {want} (a {header} header over a {row} row)"
            );
        }
    }
    Ok(())
}

/// The painted galley of `text` whose top left lies in one of `within`, where it is drawn.
fn galley_rect(harness: &Harness<'_, App>, text: &str, within: &[egui::Rect]) -> egui::Rect {
    let shapes = harness.output().shapes.clone();
    let found = shapes.iter().find_map(|clipped| match &clipped.shape {
        egui::Shape::Text(shape)
            if shape.galley.job.text == text && within.iter().any(|r| r.contains(shape.pos)) =>
        {
            Some(shape.galley.rect.translate(shape.pos.to_vec2()))
        }
        _ => None,
    });
    found.unwrap_or_else(|| panic!("no {text:?} galley was painted"))
}

/// Every table centres its text in its row's height, as the gpui showcase's table centres each
/// cell's text in its row (`showcase-gpui/demo.rs`, `files_table`'s cells,
/// `.h_full().flex().items_center()`): under material-3, whose `list.header(NONE).min_size[1]`
/// is taller than the header's line, the galleys of the Basic table's "Name", the Data table's "Theme"
/// and the Theme Map's "Leaf" are each centred in their header row within half a pixel; under
/// windows-11, whose `list.row_height` is taller than a body line, so are the Data table's and
/// the Theme Map's first body texts in their row. The Basic header row is its place
/// (`basic.table.header`); a `TableBuilder`'s header row begins at its table's top, its first
/// body row under it, each [`next_table_row`] of the installed 0.8.0 theme tall.
#[test]
fn the_tables_centre_their_text_in_its_row() -> Result<(), String> {
    let mut wrong = Vec::new();
    let mut check = |what: String, galley: egui::Rect, row: egui::Rect| {
        if (galley.center().y - row.center().y).abs() > 0.5 {
            wrong.push(format!(
                "{what}: the text's centre is at {}, the row's at {} ({row:?})",
                galley.center().y,
                row.center().y
            ));
        }
    };

    let basic = open_page_under(Page::Basic, egui::Theme::Light, "material-3");
    let places = basic.state().registry.places();
    let header = places["basic.table.header"];
    let name = galley_rect(&basic, "Name", &[places["basic.table.header.name"]]);
    check("material-3 basic header".to_string(), name, header);

    for (preset, body) in [("material-3", false), ("windows-11", true)] {
        for (tab, page, container, header_kind, cell_kind) in [
            ("data", Page::Data, "Table", "table header", "table cell"),
            (
                "theme-map",
                Page::ThemeMap,
                "theme map",
                "theme map header",
                "theme map cell",
            ),
        ] {
            use native_theme::next::WidgetState;
            let page = open_page_under(page, egui::Theme::Light, preset);
            let r = page
                .state()
                .atlas
                .next_resolved(egui::Theme::Light)
                .ok_or("no 0.8.0 theme installed")?
                .clone();
            let l = &r.list;
            let (label, item) = (
                l.header_label(WidgetState::NONE),
                l.item_label(WidgetState::NONE),
            );
            let header_height = next_table_row(&page, &r, true, label.size_px)?;
            let records = page.state().registry.records();
            let top = records
                .iter()
                .find(|r| r.info.kind == container)
                .map(|r| r.rect.top())
                .unwrap_or_else(|| panic!("{preset} {tab}: the table records itself"));
            let rects = |kind: &str| -> Vec<egui::Rect> {
                records
                    .iter()
                    .filter(|r| r.info.kind == kind)
                    .map(|r| r.rect)
                    .collect()
            };
            let header =
                egui::Rect::from_x_y_ranges(0.0..=crate::WINDOW_SIZE.x, top..=top + header_height);
            let title = if tab == "data" { "Theme" } else { "Leaf" };
            let galley = galley_rect(&page, title, &rects(header_kind));
            check(format!("{preset} {tab} header"), galley, header);
            if body {
                let (manifest, json) = manifest_and_json(&page)?;
                let first = if tab == "data" {
                    "kde-breeze".to_string()
                } else {
                    let row = manifest.rows.first().ok_or("mapping.toml has no row")?;
                    let ([leaf, ..], _) = crate::info::row_cells(row, (&json, preset));
                    leaf
                };
                let row_height = next_table_row(&page, &r, false, item.size_px)?;
                let row = egui::Rect::from_x_y_ranges(
                    0.0..=crate::WINDOW_SIZE.x,
                    header.bottom()..=header.bottom() + row_height,
                );
                let galley = galley_rect(&page, &first, &rects(cell_kind));
                check(format!("{preset} {tab} first row"), galley, row);
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    Ok(())
}

/// How far a table row's measured height may lie from its expected one: the layout's float
/// error, far below a pixel, so a row grown by a wrapped line or the spacing between rows fails.
const ROW_SLACK: f32 = 0.01;

/// How far a centred cell's measured centre may lie from its expected one: egui rounds the
/// frame a centred widget is laid in to its layout grid (egui `src/layout.rs:635`, `round_ui`;
/// `emath::GUI_ROUNDING`), so a centre can move by up to half a grid step, and the distance
/// between two centres by one.
const CENTRE_SLACK: f32 = egui::emath::GUI_ROUNDING;

/// The Data table at text scales 1, 1.5 and the largest the Preferences offer
/// (`chrome::TEXT_SCALE_MAX`), and the Theme Map at 1, lie across the central panel on every
/// bundled preset: every header and body cell, its text cut short or whole, lies
/// wholly inside the panel's width (the page scrolls only vertically, `app.rs`,
/// `ScrollArea::vertical`), where a column as wide as its longest text pushed the Theme Map's
/// last ones past the panel's edge, out of view. So at a scale of 1 the Theme Map's view across
/// (`pages/theme_map.rs`) holds the whole table with no scrolling; at 1.5 it can be wider than
/// its view (ios-13's Sinks column ends about 50 px past it), and
/// [`the_tables_keep_their_columns_at_the_largest_text_scale`] holds it reachable there. A cell's
/// whole rectangle is read (`Record::whole`), not the part the view's clip leaves, so a cell the
/// edge cuts partway fails too.
#[test]
fn the_data_and_theme_map_tables_lie_inside_the_central_panel() {
    let mut wrong = Vec::new();
    // The presets of the 0.8.0 model, which the Theme Map reads, installed as the picker
    // installs them.
    for key in native_theme::next::Preset::ALL.iter().map(|p| p.as_str()) {
        for (tab, kinds, factors) in [
            (
                "data",
                ["table header", "table cell"],
                &[1.0, 1.5, crate::chrome::TEXT_SCALE_MAX][..],
            ),
            ("theme-map", ["theme map header", "theme map cell"], &[1.0]),
        ] {
            for &factor in factors {
                let Ok(kind) = Page::from_key(tab) else {
                    wrong.push(format!("{tab}: no such page"));
                    continue;
                };
                let page = open_page_scaled(kind, key, factor);
                let records = page.state().registry.records();
                let panel = records
                    .iter()
                    .find(|r| r.info.kind == "Central panel")
                    .map(|r| r.rect)
                    .expect("the central panel records itself");
                let cells: Vec<egui::Rect> = records
                    .iter()
                    .filter(|r| kinds.contains(&r.info.kind))
                    .map(|r| r.whole)
                    .collect();
                assert!(!cells.is_empty(), "{key} {tab}: no cells");
                let outside: Vec<&egui::Rect> = cells
                    .iter()
                    .filter(|r| r.left() < panel.left() || r.right() > panel.right())
                    .collect();
                if let Some(first) = outside.first() {
                    wrong.push(format!(
                        "{key} at {factor} {tab}: {} cells outside the panel's x {} to {}, \
                         e.g. {first:?}",
                        outside.len(),
                        panel.left(),
                        panel.right()
                    ));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// At a text scale of 1.5 and at the largest the Preferences offer (`chrome::TEXT_SCALE_MAX`), on
/// every bundled preset, the Theme Map, shown with its longest leaf in view (its verdict's rows,
/// scrolled to it), keeps its Value and Sinks columns at least as wide as their headers' text
/// (each header drawn whole at some view across), and where it is wider than its view, the view
/// moves across, and every cell is wholly in view at one of the views it is moved through to its
/// right end, or, a cell wider than the view, spans it there. Cells are read by their whole
/// rectangles (`Record::whole`), not the parts the view's clip leaves, within half a pixel. The
/// Data table at these scales is held by
/// [`the_data_and_theme_map_tables_lie_inside_the_central_panel`]. One test per preset of the
/// 0.8.0 model, which the Theme Map reads.
mod the_tables_keep_their_columns_at_the_largest_text_scale {
    use super::*;
    per_preset!(the_tables_keep_their_columns_under);
}

fn the_tables_keep_their_columns_under(preset: native_theme::next::Preset) -> Result<(), String> {
    let manifest = Manifest::parse(MANIFEST).map_err(|e| format!("the manifest parses: {e}"))?;
    let longest = manifest
        .rows
        .iter()
        .max_by_key(|r| r.leaf.chars().count())
        .ok_or("the manifest has rows")?;
    let mut wrong = Vec::new();
    // The preset installed as the picker installs it.
    let runs = [1.5, crate::chrome::TEXT_SCALE_MAX].map(|factor| (preset.as_str(), factor));
    for (preset, factor) in runs {
        let key = preset;
        let mut map = open_page_scaled(Page::ThemeMap, preset, factor);
        map.state_mut().demo_state.theme_map_filter = Some(longest.verdict);
        map.run_steps(2);
        map.hover_at(centre_of(&map, "theme map"));
        let painted = |map: &Harness<'_, App>, text: &str| -> Vec<bool> {
            map.output()
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(s) if s.galley.text() == text => Some(s.galley.elided),
                    _ => None,
                })
                .collect()
        };
        for _ in 0..80 {
            if !painted(&map, &longest.leaf).is_empty() {
                break;
            }
            map.event(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, -120.0),
                phase: egui::TouchPhase::Move,
                source: egui::MouseWheelSource::Unknown,
                modifiers: egui::Modifiers::NONE,
            });
            map.run_steps(2);
        }
        if painted(&map, &longest.leaf).is_empty() {
            wrong.push(format!(
                "{key} at {factor}: {} not scrolled into view",
                longest.leaf
            ));
            continue;
        }
        // Each cell of a row in the table's view, by its row's top and its column, whether it
        // is wholly in view across: its whole rectangle (`Record::whole`, not the part the
        // view's clip leaves) inside the view (the table's record as its clip leaves it), or,
        // a cell wider than the view, across all of it; each header, whether it is drawn whole;
        // and the first header's left edge, which moves as the view scrolls across. A row the
        // view clips away below is laid out but not shown (its clipped rectangle has no
        // height), and is left out: the view scrolls across here, not down. With no view
        // recorded no cell is in view, and the test fails.
        type Cells = std::collections::BTreeMap<(i64, usize), bool>;
        let table_left = |map: &Harness<'_, App>| {
            map.state()
                .registry
                .records()
                .iter()
                .find(|r| r.info.kind == "theme map header")
                .map(|r| r.whole.left())
        };
        let in_view = |map: &Harness<'_, App>| -> (Cells, Vec<Vec<bool>>) {
            let records = map.state().registry.records();
            let view = records
                .iter()
                .find(|r| r.info.kind == "theme map")
                .map_or(egui::Rect::NOTHING, |r| r.rect);
            let mut cells = Cells::new();
            let mut row = (i64::MIN, 0);
            for r in records
                .iter()
                .filter(|r| ["theme map header", "theme map cell"].contains(&r.info.kind))
            {
                // by its whole rectangle's centre: each cell is centred in its row
                // (`pages/data.rs`, `centred_cells`), whatever its height and its clip
                let top = (r.whole.center().y * 32.0).round() as i64;
                row = if top == row.0 {
                    (top, row.1 + 1)
                } else {
                    (top, 0)
                };
                if r.rect.height() > 0.0 {
                    // half a pixel: egui lays a column's edges out on its pixel grid
                    let slack = 0.5 / map.ctx.pixels_per_point();
                    let (left, right) = (r.whole.left(), r.whole.right());
                    let inside = left >= view.left() - slack && right <= view.right() + slack;
                    let across = r.whole.width() > view.width()
                        && left <= view.left() + slack
                        && right >= view.right() - slack;
                    cells.insert(row, view.width() > 0.0 && (inside || across));
                }
            }
            let titles = ["Value", "Sinks / upstream"]
                .into_iter()
                .map(|title| painted(map, title))
                .collect();
            (cells, titles)
        };
        // The scrolling down settled, in the harness's quarter-second steps, so the rows lie
        // where they lie at every view across (only the view across moves from here). Then the
        // view is moved across in steps of `STEP`, each read: a cell narrower than the view by
        // more than `STEP` is wholly in view at one of them at least, a column in the middle of
        // a table far wider than the view being wholly in view only between its two ends.
        const STEP: f32 = 40.0;
        map.run_steps(8);
        let (first, first_titles) = in_view(&map);
        let left_at_start = table_left(&map);
        let mut reached = first.clone();
        let mut titles = first_titles.clone();
        let mut left = left_at_start;
        let mut still = 0;
        for _ in 0..400 {
            map.event(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(-STEP, 0.0),
                phase: egui::TouchPhase::Move,
                source: egui::MouseWheelSource::Unknown,
                modifiers: egui::Modifiers::NONE,
            });
            map.run_steps(2);
            let (cells, shown) = in_view(&map);
            for (key, whole) in cells {
                *reached.entry(key).or_insert(false) |= whole;
            }
            for (seen, now) in titles.iter_mut().zip(shown) {
                if now == [false] {
                    *seen = now;
                }
            }
            // the right end: three steps with no move
            let now = table_left(&map);
            still = if now == left { still + 1 } else { 0 };
            if still == 3 {
                break;
            }
            left = now;
        }
        // a table wider than the view moved across
        let cut_at_start = first.values().any(|whole| !whole);
        if cut_at_start && !matches!((left_at_start, left), (Some(a), Some(b)) if b < a) {
            wrong.push(format!(
                "{} at {factor}: the Theme Map does not fit its view, yet the view across did \
                 not move: its first header's left edge {left_at_start:?}, then {left:?}",
                preset
            ));
        }
        for (ix, title) in ["Value", "Sinks / upstream"].into_iter().enumerate() {
            if titles.get(ix).is_none_or(|seen| seen != &[false]) {
                wrong.push(format!(
                    "{} at {factor}: the {title:?} header is never drawn whole across the table, \
                     at the start elided {:?}",
                    preset,
                    first_titles.get(ix)
                ));
            }
        }
        let unreached: Vec<&(i64, usize)> = reached
            .iter()
            .filter(|(_, whole)| !**whole)
            .map(|(key, _)| key)
            .collect();
        if first.is_empty() || !unreached.is_empty() {
            wrong.push(format!(
                "{} at {factor}: {} of {} Theme Map cells never wholly in view across the \
                 table, e.g. {:?}",
                preset,
                unreached.len(),
                reached.len(),
                unreached.first()
            ));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    Ok(())
}

/// A Theme Map colour value too long for its Value column is cut short on its one line, beside
/// its swatch, and shows its whole text on hover, as the column's other texts are: at the
/// largest text scale under the test preset, scrolled to the longest leaf (the Leaf column as
/// wide as it), the Value column is about its header's width, narrower than a colour's swatch
/// and hex. Every colour value's galley in view is one row, and one cut short is hovered.
#[test]
fn a_narrow_colour_cell_is_cut_short_and_whole_on_hover() {
    let manifest = Manifest::parse(MANIFEST).expect("the manifest parses");
    let longest = manifest
        .rows
        .iter()
        .max_by_key(|r| r.leaf.chars().count())
        .expect("the manifest has rows");
    let mut map = open_page_scaled(Page::ThemeMap, TEST_PRESET, crate::chrome::TEXT_SCALE_MAX);
    map.state_mut().demo_state.theme_map_filter = Some(longest.verdict);
    map.run_steps(2);
    // Down to the longest leaf, as `the_tables_keep_their_columns_at_the_largest_text_scale`
    // scrolls to it.
    map.hover_at(centre_of(&map, "theme map"));
    let leaf_shown = |map: &Harness<'_, App>| {
        map.output()
            .shapes
            .iter()
            .any(|c| matches!(&c.shape, egui::Shape::Text(s) if s.galley.text() == longest.leaf))
    };
    for _ in 0..80 {
        if leaf_shown(&map) {
            break;
        }
        map.event(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -120.0),
            phase: egui::TouchPhase::Move,
            source: egui::MouseWheelSource::Unknown,
            modifiers: egui::Modifiers::NONE,
        });
        map.run_steps(2);
    }
    map.run_steps(8);
    assert!(
        leaf_shown(&map),
        "{} is not scrolled into view",
        longest.leaf
    );
    // The colour values' galleys painted in a body cell in view: a colour's value is its hex
    // (`Rgba`'s `Display`).
    let cells: Vec<egui::Rect> = map
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "theme map cell" && r.rect.height() > 0.0)
        .map(|r| r.rect)
        .collect();
    let shown: Vec<(egui::Pos2, std::sync::Arc<egui::Galley>)> = map
        .output()
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(s)
                if s.galley.text().starts_with('#')
                    && cells
                        .iter()
                        .any(|c| c.contains(s.pos + egui::vec2(0.0, 1.0))) =>
            {
                Some((s.pos, s.galley.clone()))
            }
            _ => None,
        })
        .collect();
    assert!(!shown.is_empty(), "no colour value is painted in view");
    for (_, galley) in &shown {
        assert_eq!(
            galley.rows.len(),
            1,
            "the colour value {:?} is laid out on {} rows in a one-line cell",
            galley.text(),
            galley.rows.len()
        );
    }
    let Some((pos, galley)) = shown.iter().find(|(_, g)| g.elided) else {
        panic!(
            "no colour value is cut short in the Value column: {:?}",
            shown.iter().map(|(_, g)| g.text()).collect::<Vec<_>>()
        );
    };
    // The cell's own label carries the whole text too (its `WidgetInfo`): the tooltip is a
    // second node with it.
    let whole = galley.text().to_string();
    let before = map.query_all_by_label(&whole).count();
    hover_and_settle(&mut map, *pos + galley.rect.center().to_vec2());
    map.run_steps(4);
    assert!(
        map.query_all_by_label(&whole).count() > before,
        "hovering the cut colour value shows no text reading {whole:?}"
    );
}

/// The query of a table's part at rest (`ListVariant::Table`), as the showcase's tables ask it.
fn table_query() -> native_theme::next::ListQuery {
    use native_theme::next::{ListKind, ListQuery, ListVariant, WidgetState};
    ListQuery::new(WidgetState::NONE).variant(ListKind {
        variant: ListVariant::Table,
        alternate: false,
    })
}

/// The height of a body row, or `header`, the header, of the Data and Theme Map pages'
/// `TableBuilder`s (`pages/data.rs`, `TableRows::of`), of the installed 0.8.0 theme `r` in
/// `harness`, a table's ([`table_query`]): the stated height, `list.row(NONE).min_size[1]` (the
/// header's `list.header(NONE).min_size[1]`, else the row's), as [`next_rule`] grows it with the
/// part's padding above and below; where the theme states none, the line box of the label
/// (`list.item_label(NONE)`, `list.header_label(NONE)`) drawn at `drawn` and that padding, on
/// egui's layout grid. A side of the padding the theme leaves unstated is egui_extras' own, half
/// the List scope's `item_spacing.y`. A line box is the style's stated `line_height`, else its
/// size times `typography.line_height_multiplier`.
fn next_table_row(
    harness: &Harness<'_, App>,
    r: &native_theme::next::ResolvedTheme,
    header: bool,
    drawn: f32,
) -> Result<f32, String> {
    let q = table_query();
    let l = &r.list;
    let [_, stated_row] = l.row(q).min_size;
    let (part, style, stated) = if header {
        let part = l.header(q);
        let stated = part.min_size[1].or(stated_row);
        (part, l.header_label(q), stated)
    } else {
        (l.row(q), l.item_label(q), stated_row)
    };
    let own = 0.5
        * scope_style(harness, native_theme_egui::Role::List)?
            .spacing
            .item_spacing
            .y;
    let [top, _, bottom, _] = native_theme_egui::next::padding_of(&part);
    let padding = top.unwrap_or(own) + bottom.unwrap_or(own);
    let line_box = if style
        .stated
        .contains(native_theme::next::TextStated::LINE_HEIGHT)
    {
        style.line_height
    } else {
        drawn * r.typography.line_height_multiplier
    };
    Ok(match stated {
        Some(h) => next_rule(r, h, &style, padding),
        None => egui::emath::GuiRounding::round_ui(line_box + padding),
    })
}

/// docs/todo.md's G1/G2 item for M9c ("the egui showcase's body rows grow past
/// `list.row_height`"): the Data and Theme Map tables' body rows are each exactly a row tall,
/// [`next_table_row`] of the installed 0.8.0 theme, one after another with no spacing between
/// them, at text scales 1 and 1.5, where the theme states `list.row(NONE).min_size[1]`
/// (windows-11, macos-monterey) and where it states none ([`TEST_PRESET`]): every cell is one
/// line in its line box, the auto-sized columns as wide as their text and the last column's
/// text cut short with egui's ellipsis, where a cell wrapped in its column and grew its row
/// (egui_extras `layout.rs:160-168`).
#[test]
fn the_data_and_theme_map_rows_are_the_themes_row_height() -> Result<(), String> {
    let mut wrong = Vec::new();
    let mut covered = std::collections::BTreeSet::new();
    for (preset, factor) in [
        ("windows-11", 1.0),
        ("macos-monterey", 1.0),
        (TEST_PRESET, 1.0),
        ("windows-11", 1.5),
        ("macos-monterey", 1.5),
        (TEST_PRESET, 1.5),
    ] {
        for (tab, page, kind) in [
            ("data", Page::Data, "table cell"),
            ("theme-map", Page::ThemeMap, "theme map cell"),
        ] {
            let page = open_page_scaled(page, preset, factor);
            let r = page
                .state()
                .atlas
                .next_resolved(egui::Theme::Light)
                .ok_or("no 0.8.0 theme installed")?
                .clone();
            assert_eq!(r.context.text_scale, factor, "{preset}: the text scale");
            let item = r.list.item_label(native_theme::next::WidgetState::NONE);
            let [_, stated] = r.list.row(table_query()).min_size;
            covered.insert(stated.is_some());
            let want = next_table_row(&page, &r, false, item.size_px)?;
            let cells: Vec<egui::Rect> = page
                .state()
                .registry
                .records()
                .iter()
                .filter(|r| r.info.kind == kind)
                .map(|r| r.rect)
                .collect();
            // Each row's cells by their whole rectangle's centre: every cell is centred in its
            // row, whatever its height (`pages/data.rs`, `centred_cells`), and a row the view
            // cuts at its bottom keeps its whole cells' centres (`Record::whole`).
            let whole: Vec<egui::Rect> = page
                .state()
                .registry
                .records()
                .iter()
                .filter(|r| r.info.kind == kind)
                .map(|r| r.whole)
                .collect();
            let mut centres: Vec<f32> = whole.iter().map(|r| r.center().y).collect();
            centres.sort_by(f32::total_cmp);
            centres.dedup_by(|a, b| (*a - *b).abs() <= CENTRE_SLACK);
            assert!(
                centres.len() >= 4,
                "{preset} at {factor} {tab}: rows {centres:?}"
            );
            let pitches: Vec<f32> = centres.windows(2).map(|pair| pair[1] - pair[0]).collect();
            if pitches
                .iter()
                .any(|pitch| (pitch - want).abs() > CENTRE_SLACK)
            {
                wrong.push(format!(
                    "{preset} at {factor} {tab}: rows {pitches:?} tall, want {want}"
                ));
            }
            let tallest = cells.iter().map(egui::Rect::height).fold(0.0, f32::max);
            if tallest > want + ROW_SLACK {
                wrong.push(format!(
                    "{preset} at {factor} {tab}: a cell {tallest} tall in a {want} row"
                ));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    assert_eq!(
        covered.len(),
        2,
        "the presets do not cover a stated and an unstated row height"
    );
    Ok(())
}

/// A Theme Map cell too long for its column is cut short with egui's ellipsis and shows its
/// whole text under the pointer, as egui's truncated `Label` does
/// (`egui/src/widgets/label.rs:285-294`, `show_tooltip_when_elided`, on by default): under
/// windows-11 the last column's longest sinks are cut, and hovering one paints its text whole
/// in the tooltip, beside the cut one in the cell.
#[test]
fn a_theme_map_cell_cut_short_shows_its_whole_text_on_hover() {
    let mut page = open_page_under(Page::ThemeMap, egui::Theme::Light, "windows-11");
    let cells: Vec<egui::Rect> = page
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "theme map cell")
        .map(|r| r.rect)
        .collect();
    let in_a_cell = |pos: egui::Pos2| cells.iter().any(|r| r.contains(pos));
    let cut = page
        .output()
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) if text.galley.elided && in_a_cell(text.pos) => Some((
                text.galley.text().to_string(),
                text.galley.rect.translate(text.pos.to_vec2()),
            )),
            _ => None,
        });
    let Some((text, rect)) = cut else {
        panic!("no Theme Map cell is cut short");
    };
    let delay = page.ctx.global_style().interaction.tooltip_delay;
    page.hover_at(rect.center());
    // the pointer rests past the tooltip delay, in the harness's quarter-second steps
    // (`egui_kittest/src/builder.rs:40`)
    page.run_steps(3 + (delay / 0.25).ceil() as usize);
    let painted: Vec<bool> = page
        .output()
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(shape) if shape.galley.text() == text => Some(shape.galley.elided),
            _ => None,
        })
        .collect();
    assert_eq!(
        painted,
        vec![true, false],
        "{text:?}: the cut cell and its whole text in the tooltip"
    );
}

/// M9x CI item I2 (M9-170): each Basic list row's label is placed for the layout dump as the
/// text it paints, centred down its row, as the iced and gpui showcases place theirs: under
/// windows-11 (rows `list.row(NONE).min_size[1]` 40, no vertical padding), `basic.list.row_N.text` is where
/// the row's "Item N" galley is painted, as tall as its line, narrower than its row and centred
/// in it to half a point.
#[test]
fn the_basic_list_labels_are_placed_as_painted_centred_down_their_rows() {
    let harness = open_page_under(Page::Basic, egui::Theme::Light, "windows-11");
    let places = harness.state().registry.places();
    let shapes = &harness.output().shapes;
    for n in 1..=3 {
        let row_id = format!("basic.list.row_{n}");
        let (Some(row), Some(label)) = (places.get(&row_id), places.get(&format!("{row_id}.text")))
        else {
            panic!("{row_id} or its label is not placed: {places:?}");
        };
        let text = format!("Item {n}");
        let painted = shapes.iter().find_map(|clipped| match &clipped.shape {
            egui::Shape::Text(t) if t.galley.text() == text && row.contains(t.pos) => Some(t.pos),
            _ => None,
        });
        assert_eq!(
            painted,
            Some(label.min),
            "{row_id}: its galley is painted at its label"
        );
        assert!(
            label.width() < row.width(),
            "{row_id}: the label {label:?} is its text's"
        );
        assert!(
            (label.center().y - row.center().y).abs() <= 0.5,
            "{row_id}: the label {label:?} is off the centre of the row {row:?}"
        );
    }
}

/// M9-110: the Basic list's and table's fills keep inside their frame's rounded corners, on the
/// 0.8.0 model; a highlight that states its corners (kde-breeze's list, issue #4's L4, and its
/// table, square) is the row inset by its margin, rounded by them, whatever it meets. The
/// frame (`pages/basic.rs`, `list_frame`) is rounded by `list.container(NONE).corners` and has
/// no margin, so its content lies inside its stroke, where the frame's inner radius is the
/// radius less the width. Under adwaita, kde-breeze and macos-tahoe set
/// directly, whose list frames are rounded: the table's header fill is rounded by the inner
/// radius at the top and square at the bottom, the selected last row at the bottom and square
/// at the top; the list's selected row at the corners where it meets the frame, the first row
/// at the top, the last row shown at the bottom, and square in between. On the right only where
/// the bar floats over the rows (adwaita, macos-tahoe), so a row is the list's width:
/// kde-breeze's bar takes its groove beside them. macos-tahoe's 0.5 pt line puts the frame's
/// inside on a half point, and the list's scroll area lays its rows out on whole pixels
/// (`egui/src/containers/scroll_area.rs:797-803`), half a point off it: a row there still meets
/// the frame. Under kde-breeze, which states the header's section fills, a section with the
/// pointer on it, and pressed, is filled rounded by the inner radius where it meets the frame:
/// the first section's top-left corner, the last's top-right, square elsewhere. Each fill is the
/// `Shape::Rect` at its element's place in its colour.
#[test]
fn the_basic_list_and_table_fills_keep_inside_the_frames_corners() {
    use crate::demo::drawn_fill;
    use crate::pages::basic::{LIST_VISIBLE, TABLE_ROWS, TABLE_SELECTED, header_section_fill};
    use native_theme::next::{ListKind, ListQuery, ListVariant, PartStated, WidgetState};
    use native_theme_egui::next::{radius_of, stroke_width_of};

    /// The corners of the one filled rectangle at `place`, in `fill` where given.
    fn corners_of(
        harness: &Harness<'_, App>,
        place: egui::Rect,
        fill: Option<egui::Color32>,
    ) -> egui::CornerRadius {
        let found: Vec<egui::CornerRadius> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(r)
                    if fill.map_or(r.fill.a() > 0, |fill| r.fill == fill)
                        && near_rect(r.rect, place) =>
                {
                    Some(r.corner_radius)
                }
                _ => None,
            })
            .collect();
        assert_eq!(found.len(), 1, "the fills at {place:?}");
        found[0]
    }
    /// `row` inset by the highlight `part`'s margin on each side it states.
    fn lit(part: &native_theme::next::PartStyle, row: egui::Rect) -> egui::Rect {
        let side = |bit: PartStated, v: f32| {
            if part.stated.contains(bit) { v } else { 0.0 }
        };
        let m = &part.margin;
        egui::Rect::from_min_max(
            egui::pos2(
                row.left() + side(PartStated::MARGIN_LEFT, m.left),
                row.top() + side(PartStated::MARGIN_TOP, m.top),
            ),
            egui::pos2(
                row.right() - side(PartStated::MARGIN_RIGHT, m.right),
                row.bottom() - side(PartStated::MARGIN_BOTTOM, m.bottom),
            ),
        )
    }
    let corners = |nw, ne, sw, se| egui::CornerRadius { nw, ne, sw, se };
    let radius = |r: f32| native_theme_egui::convert::to_corner_radius(egui::CornerRadius::ZERO, r);
    let query = |variant: ListVariant, state: WidgetState| {
        ListQuery::new(state).variant(ListKind {
            variant,
            alternate: false,
        })
    };
    assert_eq!(
        TABLE_SELECTED + 1,
        TABLE_ROWS.len(),
        "the table's last row is selected"
    );
    let last = LIST_VISIBLE - 1;
    assert!(last > 1, "the list shows a row between its first and last");
    let mut bars = Vec::new();
    let mut off_grid = false;
    let mut stated_corners = Vec::new();
    for preset in ["adwaita", "kde-breeze", "macos-tahoe"] {
        let mut harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .cloned();
        assert!(r.is_some(), "{preset}'s 0.8.0 theme is installed");
        let Some(r) = r else { continue };
        let l = &r.list;
        let mut inner_of = |variant: ListVariant| {
            let frame = l.container(query(variant, WidgetState::NONE));
            let width = stroke_width_of(&frame).unwrap_or_default();
            off_grid |= width.fract() != 0.0;
            radius((radius_of(&frame).unwrap_or_default() - width).max(0.0)).nw
        };
        let (inner, table_inner) = (inner_of(ListVariant::Plain), inner_of(ListVariant::Table));
        assert!(
            inner > 0,
            "{preset}: the list's frame is rounded inside its line"
        );
        let floats = r.scrollbar.overlay_mode;
        bars.push(floats);
        let places = harness.state().registry.places().clone();

        let table_rest = query(ListVariant::Table, WidgetState::NONE);
        if drawn_fill(&l.header(table_rest)).is_some() {
            assert_eq!(
                corners_of(&harness, places["basic.table.header"], None),
                corners(table_inner, table_inner, 0, 0),
                "{preset}: the table's header"
            );
        }
        let table_lit = l.highlight(query(ListVariant::Table, WidgetState::SELECTED));
        let selected = places[format!("basic.table.row_{}", TABLE_SELECTED + 1).as_str()];
        assert_eq!(
            corners_of(&harness, lit(&table_lit, selected), None),
            radius_of(&table_lit).map_or(corners(0, 0, table_inner, table_inner), radius),
            "{preset}: the table's selected last row"
        );

        let list_lit = l.highlight(query(ListVariant::Plain, WidgetState::SELECTED));
        let stated = radius_of(&list_lit);
        stated_corners.push(stated.is_some());
        for row in 0..=last {
            let left = |at: bool| if at { inner } else { 0 };
            let right = |at: bool| if at && floats { inner } else { 0 };
            let (top, bottom) = (row == 0, row == last);
            let want = stated.map_or(
                corners(left(top), right(top), left(bottom), right(bottom)),
                radius,
            );
            harness.state_mut().demo_state.basic_list = row;
            harness.run_steps(4);
            let id = format!("basic.list.row_{}", row + 1);
            let place = harness.state().registry.places()[id.as_str()];
            assert_eq!(
                corners_of(&harness, lit(&list_lit, place), None),
                want,
                "{preset}: the list's selected {id}"
            );
        }

        // A hovered or pressed header section's fill, where the theme states one (kde-breeze
        // alone of the three): the first section's top-left corner, the last's top-right.
        // kde-breeze states both in one colour: which fill pressed takes is
        // `a_header_section_takes_the_pressed_fill_else_the_hovered_one`'s.
        let (Some(hovered), Some(pressed)) = (
            header_section_fill(Some(&r), true, false),
            header_section_fill(Some(&r), true, true),
        ) else {
            assert_ne!(preset, "kde-breeze", "kde-breeze states both section fills");
            continue;
        };
        for (id, want) in [
            ("basic.table.header.name", corners(table_inner, 0, 0, 0)),
            ("basic.table.header.size", corners(0, table_inner, 0, 0)),
        ] {
            let place = places[id];
            harness.hover_at(place.center());
            harness.run_steps(3);
            assert_eq!(
                corners_of(&harness, place, Some(hovered)),
                want,
                "{preset}: the hovered {id}"
            );
            harness.drag_at(place.center());
            harness.run_steps(3);
            assert_eq!(
                corners_of(&harness, place, Some(pressed)),
                want,
                "{preset}: the pressed {id}"
            );
            harness.drop_at(place.center());
            harness.run_steps(3);
        }
    }
    assert!(
        stated_corners.contains(&true) && stated_corners.contains(&false),
        "a highlight with corners of its own and one without: {stated_corners:?}"
    );
    assert!(
        bars.contains(&true) && bars.contains(&false),
        "a floating bar and one beside the rows: {bars:?}"
    );
    assert!(off_grid, "a list line off the whole-pixel grid");
}

/// E5 (issue #4's leftovers): a hovered, unselected row is outlined, inside the row's fill and
/// on its shape. The Basic list's first row, item 2 selected, on the 0.8.0 model: in
/// `list.highlight(HOVER).stroke` where a rule states it, kde-breeze light's 1 px line, else in
/// egui's own hovered row's, the List scope's `widgets.hovered.bg_stroke` (spec §10, rule 0:
/// adwaita states none), the one `Shape::Rect` so stroked inside its place, with its fill's
/// corners. The command palette's second row, the first highlighted: in the colour of
/// `list.highlight(HOVER).stroke` where a rule states one, as wide as it states, else
/// `layout.border_width`, round it on the frame egui fills it in, at that frame's place and
/// radius.
#[test]
fn a_hovered_list_row_is_outlined_where_the_theme_states_it() {
    for preset in ["kde-breeze", "adwaita"] {
        use native_theme::next::{PartStated, WidgetState};
        let mut harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        // The list on the 0.8.0 model: the hovered row's outline where a rule states it, else
        // egui's own hovered row's, the List scope's `widgets.hovered.bg_stroke`.
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .cloned();
        assert!(r.is_some(), "{preset}'s 0.8.0 theme is installed");
        let Some(r) = r else { continue };
        let hover = r.list.highlight(WidgetState::HOVER);
        let stated = hover.from_state.contains(PartStated::STROKE);
        let list_outline = if stated {
            let width = native_theme_egui::next::stroke_width_of(&hover);
            let colour = native_theme_egui::next::state_stroke_color_of(&hover);
            assert_eq!(width, Some(1.0), "{preset}");
            colour.map(|c| egui::Stroke::new(1.0, c))
        } else {
            None
        };
        assert_eq!(
            stated,
            preset == "kde-breeze",
            "{preset}: which preset states the outline"
        );
        let hover_fill = crate::demo::drawn_state_fill(&hover);
        let stroked = |r: &egui::epaint::RectShape| r.stroke.width > 0.0 && r.stroke.color.a() > 0;
        let outline = native_theme_egui::next::state_stroke_color_of(&hover).map(|colour| {
            let width =
                native_theme_egui::next::stroke_width_of(&hover).unwrap_or(r.layout.border_width);
            egui::Stroke::new(width, colour)
        });
        assert_eq!(
            outline.is_some(),
            preset == "kde-breeze",
            "{preset}: which preset states the palette row's outline"
        );

        harness.state_mut().demo_state.basic_list = 1;
        harness.run_steps(SETTLE);
        let place = harness.state().registry.places()["basic.list.row_1"];
        harness.hover_at(place.center());
        harness.run_steps(3);
        let shapes: Vec<egui::epaint::RectShape> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(r) if place.contains_rect(r.rect) => Some(r.clone()),
                _ => None,
            })
            .collect();
        let fills: Vec<_> = shapes
            .iter()
            .filter(|r| hover_fill.is_none_or(|fill| r.fill == fill) && r.fill.a() > 0)
            .collect();
        assert_eq!(fills.len(), 1, "{preset}: the hovered row's fill");
        let lines: Vec<_> = shapes.iter().filter(|r| stroked(r)).collect();
        match list_outline {
            Some(stroke) => {
                assert_eq!(lines.len(), 1, "{preset}: the hovered row's outline");
                assert_eq!(lines[0].stroke, stroke, "{preset}");
                assert_eq!(lines[0].stroke_kind, egui::StrokeKind::Inside, "{preset}");
                assert_eq!(
                    lines[0].corner_radius, fills[0].corner_radius,
                    "{preset}: the outline on the fill's shape"
                );
            }
            None => {
                // egui's own hovered row's line, which the row's Widget Info names as egui's
                // own where egui draws one, and nothing else
                let named = harness
                    .state()
                    .registry
                    .records()
                    .iter()
                    .filter(|r| r.info.kind == "List row" && near_rect(r.rect, place))
                    .flat_map(|r| r.info.not_themeable.iter())
                    .any(|(what, note)| {
                        *what == crate::pages::basic::OWN_ROW && note.contains("stroke")
                    });
                assert!(lines.len() <= 1, "{preset}: {lines:?}");
                assert_eq!(
                    lines.len() == 1,
                    named,
                    "{preset}: egui's own outline {lines:?}, named in Widget Info {named}"
                );
            }
        }

        open_palette(&mut harness);
        let row = harness
            .state()
            .registry
            .records()
            .iter()
            .filter(|r| r.info.kind == "palette row")
            .nth(1)
            .map(|r| r.rect)
            .expect("a second palette row");
        assert_eq!(
            palette_selected(&harness),
            vec![0],
            "the first row is highlighted"
        );
        harness.hover_at(row.center());
        harness.run_steps(3);
        // Every rectangle so stroked in the pass, and egui's own frame of the row: the fill
        // the hovered `selectable_label` paints at the outline's place, with its radius.
        let shapes: Vec<egui::epaint::RectShape> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(r) => Some(r.clone()),
                _ => None,
            })
            .collect();
        let lines: Vec<_> = shapes
            .iter()
            .filter(|r| outline.is_some_and(|stroke| r.stroke == stroke))
            .collect();
        match outline {
            Some(_) => {
                assert_eq!(
                    lines.len(),
                    1,
                    "{preset}: the hovered palette row's outline"
                );
                let line = lines[0];
                assert_eq!(line.stroke_kind, egui::StrokeKind::Inside, "{preset}");
                assert!(
                    line.rect.contains_rect(row),
                    "{preset}: {:?} round the row {row:?}",
                    line.rect
                );
                assert!(
                    shapes.iter().any(|r| r.fill.a() > 0
                        && near_rect(r.rect, line.rect)
                        && r.corner_radius == line.corner_radius),
                    "{preset}: the outline on the row's frame"
                );
            }
            None => {
                // the row's frame, the smallest filled rectangle round the row, egui's own: no
                // line lies on it but the stroke egui draws with that fill
                let frame = shapes
                    .iter()
                    .filter(|r| r.fill.a() > 0 && r.rect.contains_rect(row))
                    .min_by(|a, b| a.rect.area().total_cmp(&b.rect.area()))
                    .map(|r| r.rect)
                    .expect("egui's frame of the hovered row");
                let others: Vec<_> = shapes
                    .iter()
                    .filter(|r| stroked(r) && r.fill.a() == 0 && near_rect(r.rect, frame))
                    .collect();
                assert!(others.is_empty(), "{preset}: {others:?}");
            }
        }
    }
}

/// E5: a hovered row the list's view cuts keeps its outline on its fill's shape: under
/// kde-breeze light, on the 0.8.0 model, the first row scrolled partly out at the top, its
/// highlight rounded by `list.highlight(HOVER).corners` and cut by the view, as Breeze clips a
/// row to the view, draws no line along the view's edge and no line pixel outside the fill's
/// arcs.
/// Both shapes are tessellated as epaint draws them, each clipped to its clip rectangle, and
/// sampled at every pixel's centre of the fill: the line covers no pixel more than the fill does.
#[test]
fn a_hovered_row_cut_by_the_list_view_is_outlined_on_its_fill() {
    /// How much of the pixel at `p` the meshes of `shape` cover: the largest vertex alpha
    /// interpolated over the triangles holding `p`, over the shape's own alpha; 0 outside its
    /// clip rectangle.
    fn coverage(
        ctx: &egui::Context,
        shape: &egui::epaint::ClippedShape,
        alpha: u8,
        p: egui::Pos2,
    ) -> f32 {
        let ppp = ctx.pixels_per_point();
        let mut most: f32 = 0.0;
        for clipped in ctx.tessellate(vec![shape.clone()], ppp) {
            let egui::epaint::Primitive::Mesh(mesh) = clipped.primitive else {
                continue;
            };
            if !clipped.clip_rect.contains(p) {
                continue;
            }
            for tri in mesh.indices.as_chunks::<3>().0 {
                let [a, b, c] = tri.map(|i| mesh.vertices[i as usize]);
                let d =
                    (b.pos - a.pos).x * (c.pos - a.pos).y - (c.pos - a.pos).x * (b.pos - a.pos).y;
                if d.abs() < f32::EPSILON {
                    continue;
                }
                let u = ((b.pos - p).x * (c.pos - p).y - (c.pos - p).x * (b.pos - p).y) / d;
                let v = ((c.pos - p).x * (a.pos - p).y - (a.pos - p).x * (c.pos - p).y) / d;
                let w = 1.0 - u - v;
                if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                    let a_of = |vx: egui::epaint::Vertex| f32::from(vx.color.a());
                    most = most.max((u * a_of(a) + v * a_of(b) + w * a_of(c)) / f32::from(alpha));
                }
            }
        }
        most
    }
    use crate::demo::drawn_state_fill;
    use native_theme::next::WidgetState;
    use native_theme_egui::next::{state_stroke_color_of, stroke_width_of};
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .cloned();
    assert!(r.is_some(), "kde-breeze's 0.8.0 theme is installed");
    let Some(r) = r else { return };
    let hover = r.list.highlight(WidgetState::HOVER);
    let parts = (
        stroke_width_of(&hover),
        state_stroke_color_of(&hover),
        drawn_state_fill(&hover),
    );
    assert!(
        parts.0.is_some() && parts.1.is_some() && parts.2.is_some(),
        "kde-breeze fills and outlines a hovered row: {parts:?}"
    );
    let (Some(width), Some(edge), Some(hover_fill)) = parts else {
        return;
    };
    let outline = egui::Stroke::new(width, edge);
    harness.state_mut().demo_state.basic_list = 1;
    harness.run_steps(SETTLE);
    let place = harness.state().registry.places()["basic.list.row_1"];
    harness.hover_at(place.center());
    harness.run_steps(SETTLE);
    harness.event(egui::Event::MouseWheel {
        unit: egui::MouseWheelUnit::Point,
        delta: egui::vec2(0.0, -(place.height() / 3.0).round()),
        phase: egui::TouchPhase::Move,
        source: egui::MouseWheelSource::Unknown,
        modifiers: egui::Modifiers::NONE,
    });
    harness.run_steps(3 * SETTLE);
    let row = harness.state().registry.places()["basic.list.row_1"];
    harness.hover_at(egui::pos2(row.center().x, row.bottom() - 2.0));
    harness.run_steps(SETTLE);
    let shapes = harness.output().shapes.clone();
    let fills: Vec<_> = shapes
        .iter()
        .filter_map(|s| match &s.shape {
            egui::Shape::Rect(r) if r.fill == hover_fill && r.rect.intersects(row) => Some((s, r)),
            _ => None,
        })
        .collect();
    assert_eq!(fills.len(), 1, "the hovered row's fill");
    let (fill_shape, fill) = fills[0];
    assert!(
        fill.rect.top() > row.top() && fill.rect.top() < row.bottom(),
        "the first row cut at the top: {:?} of {row:?}",
        fill.rect
    );
    assert!(
        fill.corner_radius.nw > 0,
        "the cut row's fill is rounded where it meets the frame"
    );
    let lines: Vec<_> = shapes
        .iter()
        .filter(|s| {
            matches!(&s.shape, egui::Shape::Rect(r)
                if r.stroke == outline && r.rect.intersects(row))
        })
        .collect();
    assert_eq!(lines.len(), 1, "the hovered row's outline");
    let ctx = harness.ctx.clone();
    let ppp = ctx.pixels_per_point();
    let fill_alpha = hover_fill.a();
    let mut drawn = 0;
    let mut outside = Vec::new();
    let (x0, x1) = (
        (fill.rect.left() * ppp).floor() as i32,
        (fill.rect.right() * ppp).ceil() as i32,
    );
    let (y0, y1) = (
        (fill.rect.top() * ppp).floor() as i32,
        (fill.rect.bottom() * ppp).ceil() as i32,
    );
    for y in y0..y1 {
        for x in x0..x1 {
            let p = egui::pos2((x as f32 + 0.5) / ppp, (y as f32 + 0.5) / ppp);
            let line = coverage(&ctx, lines[0], outline.color.a(), p);
            if line > 0.0 {
                drawn += 1;
            }
            let under = coverage(&ctx, fill_shape, fill_alpha, p);
            if line > under + 0.1 {
                outside.push((p, line, under));
            }
        }
    }
    assert!(drawn > 0, "the outline is drawn");
    assert!(
        outside.is_empty(),
        "line pixels outside the fill: {outside:?}"
    );
    let top_line = (x0..x1)
        .map(|x| egui::pos2((x as f32 + 0.5) / ppp, fill.rect.top() + 0.5 / ppp))
        .filter(|p| p.x > fill.rect.left() + 2.0 * fill.corner_radius.nw as f32)
        .filter(|p| p.x < fill.rect.right() - 2.0 * fill.corner_radius.ne as f32)
        .filter(|p| coverage(&ctx, lines[0], outline.color.a(), *p) > 0.0)
        .count();
    assert_eq!(top_line, 0, "no line along the view's edge");
}

/// E3 (issue #4's leftovers): the menu bar is filled with the installed 0.8.0 theme's
/// `menu_bar.bar(NONE).fill` (`chrome::menu_panel`; kde-breeze light #dee0e2, Breeze's tools
/// area, docs/platform-facts.md §2.13), not the window's, and the title of the open menu takes
/// the bar's open fill (`menu_bar.item(OPEN).fill`) with the pointer off it, where
/// `chrome::menu_bar` swaps `widgets.open` in; a closed title is not so filled. Under
/// kde-breeze with the bar's fill patched to magenta, which the unpatched
/// `menu_bar.bar(NONE).fill` is not, the bar is filled in magenta.
#[test]
fn the_menu_bar_takes_the_toolbar_fill_and_its_open_title_the_open_fill() -> Result<(), String> {
    use crate::demo::{drawn_fill, drawn_state_fill};
    use native_theme::next::{MenuBarRule, Paint, PartPatch, ThemePatch, WidgetState};
    fn check(harness: &mut Harness<'static, App>, want: egui::Color32) -> Result<(), String> {
        harness.state_mut().open_menu = Some("theme".to_string());
        harness.run_steps(4);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or("no 0.8.0 theme installed")?;
        let open = drawn_state_fill(&r.menu_bar.item(WidgetState::OPEN))
            .ok_or("kde-breeze states no open title fill")?;
        let rects: Vec<egui::epaint::RectShape> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(r) => Some(r.clone()),
                _ => None,
            })
            .collect();
        let bar = place_of(harness, "chrome.menu_bar");
        assert!(
            rects
                .iter()
                .any(|r| near_rect(r.rect, bar) && r.fill == want),
            "the menu bar's fill {want:?} at {bar:?}"
        );
        let filled = |id: &str| {
            let title = place_of(harness, id);
            rects
                .iter()
                .any(|r| r.rect.contains(title.center()) && r.fill == open)
        };
        assert!(filled("chrome.menu_bar.theme"), "the open title's fill");
        assert!(
            !filled("chrome.menu_bar.file"),
            "a closed title is not filled"
        );
        Ok(())
    }
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme installed")?;
    let fill =
        drawn_fill(&r.menu_bar.bar(WidgetState::NONE)).ok_or("kde-breeze states no bar fill")?;
    assert_eq!(fill, egui::Color32::from_rgb(0xde, 0xe0, 0xe2));
    assert_ne!(
        fill,
        harness.ctx.style_of(egui::Theme::Light).visuals.panel_fill
    );
    check(&mut harness, fill)?;
    let magenta = native_theme::next::Color::from_srgb8(255, 0, 255, 255);
    let (mut harness, _) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", |_| {
        Some(
            ThemePatch::new()
                .menu_bar(MenuBarRule::new().bar(PartPatch::new().fill(Paint::Solid(magenta)))),
        )
    })?;
    assert_ne!(
        fill,
        egui::Color32::from_rgb(255, 0, 255),
        "the unpatched menu_bar.bar(NONE).fill is the patched fill, so the patch proves nothing"
    );
    check(&mut harness, egui::Color32::from_rgb(255, 0, 255))
}

/// S10 (E4's geometry): the open Theme menu's popup, framed by `demo::menu_modifier` on the
/// installed 0.8.0 theme, insets its first item by `menu.panel(NONE).padding` on each side the
/// theme states it, else by `popover.panel.padding`, inside the popup's line, `popover.panel`'s
/// stroke: kde-breeze light 3 + 1 from the frame's edge on the top and the left; adwaita states
/// no menu panel padding, and its item lies `popover.panel.padding` inside the line. Under
/// kde-breeze with the popover's stroke patched to `colors.window.border` and the menu panel's
/// padding to twice the preset's, which the unpatched `popover.panel` stroke colour and
/// `menu.panel(NONE).padding` are not, the frame is drawn in the patched line and the
/// item lies twice as far inside it. The frame is the `Shape::Rect` egui paints for
/// `Frame::popup` (`epaint::StrokeKind::Inside` on its outer rectangle,
/// `egui-0.37.0/src/containers/frame.rs:468-476`), in `menu.panel(NONE)`'s fill and the
/// popover's line.
#[test]
fn the_open_menu_insets_its_first_item_by_the_panel_padding() -> Result<(), String> {
    use crate::demo::drawn_fill;
    use native_theme::next::{
        Edges, MenuRule, PartPatch, PopoverPatch, ResolvedTheme, Stroke, ThemePatch,
        WidgetPatchMap, WidgetState,
    };
    use native_theme_egui::next::{padding_of, stroke_color_of, stroke_width_of};
    fn walk(shape: &egui::Shape, out: &mut Vec<egui::epaint::RectShape>) {
        match shape {
            egui::Shape::Vec(inner) => inner.iter().for_each(|s| walk(s, out)),
            egui::Shape::Rect(r) => out.push(r.clone()),
            _ => {}
        }
    }
    // the first item's distance from the popup frame's edge, and the distance `r` puts it at:
    // the menu panel's padding where stated, else the popover's, with the popover's line
    fn inset(
        harness: &mut Harness<'static, App>,
        r: &ResolvedTheme,
        preset: &str,
    ) -> Result<(egui::Vec2, egui::Vec2), String> {
        harness.state_mut().open_menu = Some("theme".to_string());
        harness.run_steps(SETTLE);
        let panel = &r.popover.panel;
        let line = egui::Stroke::new(
            stroke_width_of(panel)
                .ok_or(format!("{preset} states no popover.panel stroke width"))?,
            stroke_color_of(panel).ok_or(format!("{preset} states no popover.panel stroke"))?,
        );
        let fill = drawn_fill(&r.menu.panel(WidgetState::NONE))
            .ok_or(format!("{preset} states no menu.panel(NONE).fill"))?;
        let item = place_of(harness, "chrome.menu.theme.reload");
        let mut rects = Vec::new();
        harness
            .output()
            .shapes
            .iter()
            .for_each(|clipped| walk(&clipped.shape, &mut rects));
        let frames: Vec<_> = rects
            .iter()
            .filter(|f| {
                f.stroke == line
                    && f.fill == fill
                    && f.stroke_kind == egui::StrokeKind::Inside
                    && f.rect.contains_rect(item)
            })
            .collect();
        let [frame] = frames.as_slice() else {
            return Err(format!(
                "{preset}: {} popup frames round {item:?} in {line:?} and {fill:?}",
                frames.len()
            ));
        };
        let (menu, popover) = (
            padding_of(&r.menu.panel(WidgetState::NONE)),
            padding_of(panel),
        );
        let side = |i: usize, name: &str| -> Result<f32, String> {
            let stated = menu.get(i).copied().flatten();
            let fallback = popover.get(i).copied().flatten();
            stated
                .or(fallback)
                .map(|v| v + line.width)
                .ok_or(format!("{preset} states no padding on the {name}"))
        };
        Ok((
            item.min - frame.rect.min,
            egui::vec2(side(3, "left")?, side(0, "top")?),
        ))
    }
    for preset in ["kde-breeze", "adwaita"] {
        let mut harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or(format!("{preset}: no 0.8.0 theme installed"))?
            .clone();
        let menu = padding_of(&r.menu.panel(WidgetState::NONE));
        if preset == "kde-breeze" {
            assert_eq!(menu, [Some(3.0); 4], "{preset}: menu.panel(NONE).padding");
            assert_eq!(stroke_width_of(&r.popover.panel), Some(1.0));
        } else {
            assert_eq!(menu, [None; 4], "{preset} states no menu panel padding");
        }
        let (inset, want) = inset(&mut harness, &r, preset)?;
        assert!(
            near(inset.x, want.x) && near(inset.y, want.y),
            "{preset}: the first item {inset:?} from the frame's edge, want {want:?}"
        );
    }
    let (mut harness, r) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", |r| {
        let [Some(top), Some(right), Some(bottom), Some(left)] =
            padding_of(&r.menu.panel(WidgetState::NONE))
        else {
            return None;
        };
        let width = stroke_width_of(&r.popover.panel)?;
        Some(
            // `widget_rules` replaces the map the `menu` rule is pushed into: it goes first
            ThemePatch::new()
                .widget_rules(WidgetPatchMap::new().popover(PopoverPatch::new().panel(
                    PartPatch::new().stroke(Stroke::uniform(width, r.colors.window.border?)),
                )))
                .menu(MenuRule::new().panel(PartPatch::new().padding(Edges::new(
                    2.0 * top,
                    2.0 * right,
                    2.0 * bottom,
                    2.0 * left,
                )))),
        )
    })?;
    let plain = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    let old = plain
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme installed")?;
    let patched = stroke_color_of(&r.popover.panel);
    assert_ne!(
        patched,
        stroke_color_of(&old.popover.panel),
        "the patched popover stroke is the unpatched popover.panel's, so the patch proves nothing"
    );
    let menu = padding_of(&r.menu.panel(WidgetState::NONE));
    assert_eq!(menu, [Some(6.0); 4], "the patched menu.panel(NONE).padding");
    assert_ne!(
        padding_of(&old.menu.panel(WidgetState::NONE)),
        menu,
        "the unpatched menu.panel(NONE).padding is the patched one"
    );
    let (inset, want) = inset(&mut harness, &r, "patched kde-breeze")?;
    assert!(
        near(inset.x, want.x) && near(inset.y, want.y),
        "patched kde-breeze: the first item {inset:?} from the frame's edge, want {want:?}"
    );
    Ok(())
}

/// E7 (issue #4's leftovers): the Basic table's line under the header and its column line over
/// the header take `list.header_separator_color` where the theme states it, the column line over
/// the rows `list.grid_color`: kde-breeze light's Breeze `WindowText` at `frameContrast`,
/// #23262933 (docs/platform-facts.md §2.15), laid once over the header's fill; adwaita states
/// none, and every line is the grid's.
#[test]
fn the_table_headers_lines_take_the_header_separator_colour() {
    use native_theme::next::ListVariant;
    for preset in ["kde-breeze", "adwaita"] {
        let harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .cloned();
        assert!(r.is_some(), "{preset}'s 0.8.0 theme is installed");
        let Some(r) = r else { continue };
        let grid = native_theme_egui::next::to_color32(r.list.grid_color);
        let stated = crate::demo::drawn_fill(
            &r.list
                .header_separator(native_theme::next::ListQuery::from(ListVariant::Table)),
        );
        let head = if preset == "kde-breeze" {
            assert_eq!(
                stated,
                Some(egui::Color32::from_rgba_unmultiplied(
                    0x23, 0x26, 0x29, 0x33
                ))
            );
            stated.unwrap_or(grid)
        } else {
            assert_eq!(stated, None, "{preset}");
            grid
        };
        let header = place_of(&harness, "basic.table.header");
        let row = place_of(&harness, "basic.table.row_1");
        let segments: Vec<([egui::Pos2; 2], egui::Stroke)> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::LineSegment { points, stroke } => Some((*points, *stroke)),
                _ => None,
            })
            .collect();
        let under_header: Vec<_> = segments
            .iter()
            .filter(|(p, _)| {
                p[0].y == p[1].y
                    && (p[0].y - header.bottom()).abs() < 1.0
                    && near(p[0].x.min(p[1].x), header.left())
                    && near(p[0].x.max(p[1].x), header.right())
            })
            .collect();
        assert_eq!(under_header.len(), 1, "{preset}: the line under the header");
        assert_eq!(
            under_header[0].1.color, head,
            "{preset}: the line under the header"
        );
        let column = |y: f32| -> Vec<egui::Color32> {
            segments
                .iter()
                .filter(|(p, _)| {
                    p[0].x == p[1].x
                        && p[0].y.min(p[1].y) <= y
                        && y <= p[0].y.max(p[1].y)
                        && header.x_range().contains(p[0].x)
                })
                .map(|(_, s)| s.color)
                .collect()
        };
        assert_eq!(
            column(header.center().y),
            vec![head],
            "{preset}: the column line over the header"
        );
        assert_eq!(
            column(row.center().y),
            vec![grid],
            "{preset}: the column line over the rows"
        );
    }
}

/// A table header section's fill (`pages::basic::header_section_fill`, decision 19 and 7.H.1 of
/// `docs/archive/todo_v0.7.2_kde-open-items-steps6-7-spec.md`), on the 0.8.0 model: kde-breeze
/// light states its hovered section's fill, `list.header(HOVER)`, which also answers the
/// pressed query, so the page's pressed section
/// (`the_basic_list_and_table_fills_keep_inside_the_frames_corners`) does not tell them apart.
/// Patched with a fill of its own for `PRESSED`, which the query `HOVER | PRESSED` takes, its
/// two flags of one exclusive group (spec §2's evaluation, step 1), and differs from the hovered one,
/// pressed takes it and hovered keeps its own; at rest none does, nor any state without an
/// installed 0.8.0 theme, egui's own header having no hover.
#[test]
fn a_header_section_takes_the_pressed_fill_else_the_hovered_one() -> Result<(), String> {
    use crate::pages::basic::header_section_fill;
    use native_theme::next::{ListRule, ListVariant, Paint, PartPatch, ThemePatch, WidgetState};

    let (_harness, r) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", |r| {
        Some(
            ThemePatch::new().list(
                ListRule::new()
                    .states(WidgetState::PRESSED)
                    .header(PartPatch::new().fill(Paint::Solid(r.colors.window.border?))),
            ),
        )
    })?;
    let table = |state: WidgetState| {
        let q = native_theme::next::ListQuery::new(state).variant(native_theme::next::ListKind {
            variant: ListVariant::Table,
            alternate: false,
        });
        crate::demo::drawn_state_fill(&r.list.header(q))
    };
    let hovered = table(WidgetState::HOVER);
    let pressed = table(WidgetState::HOVER.union(WidgetState::PRESSED));
    let border = r
        .colors
        .window
        .border
        .map(native_theme_egui::next::to_color32);
    if hovered.is_none() || pressed != border || hovered == pressed {
        return Err(format!(
            "kde-breeze states the hovered section {hovered:?}, the patched pressed one \
             {pressed:?}, want the window border {border:?} and two fills"
        ));
    }
    let states = [(false, false), (true, false), (false, true), (true, true)];
    let fills = |r: Option<&native_theme::next::ResolvedTheme>| {
        states.map(|(hover, press)| header_section_fill(r, hover, press))
    };
    assert_eq!(fills(Some(&r)), [None, hovered, pressed, pressed]);
    assert_eq!(fills(None), [None; 4], "egui's own header has no hover");
    Ok(())
}

/// The Basic list's and table's rows are drawn as the spec's list `row`, then `highlight` over
/// it (spec §5, "The list's `row` and `highlight`"): no bundled preset states a fill for a row
/// that is not an alternate one (each states the alternate stripe alone, and neither the list
/// nor the table is striped), so kde-breeze light is patched with one, the window border, for
/// the rows that are not alternate ones. Each visible list row and each table row is then
/// filled in it across its whole place, and the selected one's highlight fill is painted after
/// it, over it; the stripe stays the preset's.
#[test]
fn a_list_row_is_filled_in_its_stated_fill_under_its_highlight() -> Result<(), String> {
    use crate::demo::{drawn_fill, drawn_state_fill};
    use crate::pages::basic::{LIST_VISIBLE, TABLE_ROWS, TABLE_SELECTED};
    use native_theme::next::{
        ListKind, ListQuery, ListRule, ListVariant, Paint, PartPatch, Preset, Theme, ThemeContext,
        ThemePatch, WidgetState,
    };

    let row_of = |r: &native_theme::next::ResolvedTheme, alternate: bool| {
        let q = ListQuery::new(WidgetState::NONE).variant(ListKind {
            variant: ListVariant::Plain,
            alternate,
        });
        drawn_fill(&r.list.row(q))
    };
    let own = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
    if row_of(&own, false).is_some() || row_of(&own, true).is_none() {
        return Err(
            "kde-breeze states a fill for a plain row, or no stripe: the patch \
                    proves nothing"
                .to_owned(),
        );
    }
    let (mut harness, r) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", |r| {
        Some(
            ThemePatch::new().list(
                ListRule::new()
                    .alternate(false)
                    .row(PartPatch::new().fill(Paint::Solid(r.colors.window.border?))),
            ),
        )
    })?;
    let border = r
        .colors
        .window
        .border
        .map(native_theme_egui::next::to_color32);
    let fill = row_of(&r, false);
    if fill.is_none() || fill != border || row_of(&r, true) != row_of(&own, true) {
        return Err(format!(
            "the patched plain row's fill {fill:?} is not the window border {border:?}, or \
             the stripe changed"
        ));
    }
    harness.run_steps(SETTLE);
    let selected_list = harness.state().demo_state.basic_list;
    let mut rows: Vec<(String, bool, ListVariant)> = (0..LIST_VISIBLE)
        .map(|i| {
            (
                format!("basic.list.row_{}", i + 1),
                i == selected_list,
                ListVariant::Plain,
            )
        })
        .collect();
    rows.extend((0..TABLE_ROWS.len()).map(|i| {
        (
            format!("basic.table.row_{}", i + 1),
            i == TABLE_SELECTED,
            ListVariant::Table,
        )
    }));
    let shapes: Vec<egui::epaint::RectShape> = harness
        .output()
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Rect(r) => Some(r.clone()),
            _ => None,
        })
        .collect();
    for (id, selected, variant) in rows {
        let place = harness.state().registry.places()[&id];
        let at = |colour: Option<egui::Color32>| {
            shapes.iter().position(|s| {
                Some(s.fill) == colour && place.contains_rect(s.rect) && s.rect.is_positive()
            })
        };
        let under = shapes
            .iter()
            .position(|s| Some(s.fill) == fill && near_rect(s.rect, place));
        assert!(
            under.is_some(),
            "{id}: not filled in the row's fill across its place"
        );
        if selected {
            let q = ListQuery::new(WidgetState::SELECTED).variant(ListKind {
                variant,
                alternate: false,
            });
            let lit = drawn_state_fill(&r.list.highlight(q));
            assert!(
                lit.is_some() && lit != fill,
                "{id}: kde-breeze states the highlight"
            );
            let over = at(lit);
            assert!(
                over.is_some() && over > under,
                "{id}: the highlight {over:?} is not painted over the row {under:?}"
            );
        }
    }
    Ok(())
}

/// The swap of `docs/archive/todo_v0.7.2_kde-open-items-steps6-7-spec.md` 7.H.2, on the 0.8.0
/// model: a selected list or table row is asked with `UNFOCUSED_VIEW` while the window is not
/// active (spec §5, the list's `highlight`). A pass with `RawInput::focused` false, which
/// egui_kittest otherwise keeps true, paints the table's and the list's selected rows under
/// kde-breeze light in `list.highlight(SELECTED | UNFOCUSED_VIEW).fill` lettered
/// `list.item_label(SELECTED | UNFOCUSED_VIEW).color`, which differ from the focused pair; a
/// focused pass again in `SELECTED`'s, each of the list's or the table's variant. Each fill is
/// the one filled rectangle inside its row's place, the highlight inset by its margin.
#[test]
fn a_selected_row_takes_the_unfocused_pair_while_the_window_is_not_active() {
    use crate::demo::drawn_state_fill;
    use crate::pages::basic::TABLE_SELECTED;
    use native_theme::next::{ListKind, ListQuery, ListVariant, WidgetState};
    use native_theme_egui::next::state_text_color_of;

    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .cloned();
    assert!(r.is_some(), "kde-breeze's 0.8.0 theme is installed");
    let Some(r) = r else { return };
    let pair = |variant: ListVariant, state: WidgetState| {
        let q = ListQuery::new(state).variant(ListKind {
            variant,
            alternate: false,
        });
        (
            drawn_state_fill(&r.list.highlight(q)),
            state_text_color_of(&r.list.item_label(q)),
        )
    };
    let unfocused = WidgetState::SELECTED.union(WidgetState::UNFOCUSED_VIEW);
    for variant in [ListVariant::Plain, ListVariant::Table] {
        let (lit, dim) = (
            pair(variant, WidgetState::SELECTED),
            pair(variant, unfocused),
        );
        assert!(
            lit.0.is_some() && lit.1.is_some() && dim.0.is_some() && dim.1.is_some(),
            "kde-breeze states both pairs: {lit:?} {dim:?}"
        );
        assert_ne!(lit, dim, "{variant:?}: the unfocused pair differs");
    }
    let rows = [
        (
            format!("basic.table.row_{}", TABLE_SELECTED + 1),
            ListVariant::Table,
        ),
        (
            format!(
                "basic.list.row_{}",
                harness.state().demo_state.basic_list + 1
            ),
            ListVariant::Plain,
        ),
    ];
    let painted = |harness: &Harness<'_, App>, id: &str| {
        let place = harness.state().registry.places()[id];
        let shapes = &harness.output().shapes;
        let fills: Vec<egui::Color32> = shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(r) if place.contains_rect(r.rect) && r.fill.a() > 0 => {
                    Some(r.fill)
                }
                _ => None,
            })
            .collect();
        let inks: Vec<egui::Color32> = shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(t) if place.contains(t.pos) => {
                    t.galley.job.sections.first().map(|s| s.format.color)
                }
                _ => None,
            })
            .collect();
        (fills, inks)
    };
    for focused in [false, true] {
        harness.input_mut().focused = focused;
        harness.run_steps(3);
        for (id, variant) in &rows {
            let state = if focused {
                WidgetState::SELECTED
            } else {
                unfocused
            };
            let (fill, ink) = pair(*variant, state);
            let (fills, inks) = painted(&harness, id);
            assert_eq!(
                fills,
                Vec::from_iter(fill),
                "{id}, focused {focused}: its fill"
            );
            let ink = ink.unwrap_or_default();
            assert!(
                !inks.is_empty() && inks.iter().all(|c| *c == ink),
                "{id}, focused {focused}: its text {inks:?}"
            );
        }
    }
}

/// M9-150's class in egui: a stated row height holds its text at every text scale, as gpui's
/// list and menu rows do (`geometry::list_item`, `geometry::menu_item`: the stated height at a
/// factor of 1 or less, a minimum around the drawn text and padding above it) and as the iced
/// showcase's Basic list and window menu rows do. At 1 the Basic list's rows are
/// the installed 0.8.0 theme's `list.row(NONE).min_size[1]`, and
/// the window's menu rows the installed 0.8.0 theme's `menu.item(NONE).min_size[1]` (adwaita,
/// macos-monterey, material-3), macos-monterey's 22 px row too, whose 17.33 px menu font's line
/// box and stated 3 + 3 padding overrun it: the connector narrows the padding to fit
/// (`style::roles::fit_row_padding`), where egui's `Button`, which takes the stated height as
/// its least (`interact_size.y`), drew the row 27 tall; at 1.5 and 1.75 each is the
/// stated height or the line box of its font at its scaled size and line height, rounded up,
/// plus its padding (`list.border.padding`, the menu's button padding), whichever is more,
/// computed from the 0.8.0 theme at test time ([`next_rule`], its sizes scaled already): 1.75 puts macos-monterey's menu line box at 36.09, which
/// egui's own growth would leave a pixel short of the rounded-up 37.
#[test]
fn stated_list_and_menu_rows_hold_their_text_at_every_text_scale() -> Result<(), String> {
    use native_theme::next::WidgetState;
    use native_theme_egui::next::{padding_of, stroke_width_of};
    let mut faults = Vec::new();
    let mut seen = (0, 0);
    for (preset, factor) in ["adwaita", "macos-monterey", "material-3"]
        .into_iter()
        .flat_map(|preset| [(preset, 1.0), (preset, 1.5), (preset, 1.75)])
    {
        let mut harness = open_page_scaled(Page::Basic, preset, factor);
        harness.state_mut().open_menu = Some("theme".to_string());
        harness.run_steps(4);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or(format!("{preset}: no 0.8.0 theme installed"))?
            .clone();
        let places = harness.state().registry.places();
        let mut check = |id: &str, want: f32| match places.get(id) {
            Some(rect) if (rect.height() - want).abs() <= 0.05 => {}
            Some(rect) => faults.push(format!(
                "{preset} at {factor}: {id} is {} tall, want {want}",
                rect.height()
            )),
            None => faults.push(format!("{preset} at {factor}: {id} is not placed")),
        };
        let row = r.list.row(WidgetState::NONE);
        if let Some(h) = row.min_size[1] {
            seen.0 += 1;
            // the list's rows take the List scope's button padding on a side the theme leaves
            // unstated (`pages/basic.rs`, `row_metrics`), which the List role leaves the base
            // style's
            let own = harness.ctx.global_style().spacing.button_padding.y;
            let [top, _, bottom, _] = padding_of(&row);
            let padding = top.unwrap_or(own) + bottom.unwrap_or(own);
            check(
                "basic.list.row_1",
                egui::emath::GuiRounding::round_ui(next_rule(
                    &r,
                    h,
                    &r.list.item_label(WidgetState::NONE),
                    padding,
                )),
            );
        }
        let item = r.menu.item(WidgetState::NONE);
        if let Some(h) = item.min_size[1] {
            seen.1 += 1;
            // the menu scope's button padding above and below: each stated side with the
            // item's line width where the theme states one, egui's menu padding (none
            // vertically, `egui/src/containers/menu.rs:23`) on a side the theme leaves unstated
            // (src/style/base.rs, `button_padding_of`)
            let width = stroke_width_of(&item);
            let [top, _, bottom, _] = padding_of(&item);
            let padding: f32 = [top, bottom]
                .into_iter()
                .flatten()
                .flat_map(|side| std::iter::once(side).chain(width))
                .sum();
            check(
                "chrome.menu.theme.light",
                next_rule(&r, h, &r.menu.item_label(WidgetState::NONE), padding),
            );
        }
    }
    assert!(
        seen.0 > 0 && seen.1 > 0,
        "no preset states a row height: {seen:?}"
    );
    assert!(faults.is_empty(), "{faults:#?}");
    Ok(())
}

/// M9-150's class for the expander: the installed 0.8.0 theme's stated
/// `expander.header(NONE).min_size[1]` follows the one rule every stated row height takes, the
/// stated height at a text scale of 1 or less and above 1 at least the line box of
/// `expander.header_label(NONE)` at its size, which the resolve has scaled already, and the
/// theme's line height, rounded up ([`next_rule`]), plus the header's own padding
/// (controller's ruling, 2026-10-23, M9c T6b). The Basic page's
/// expander (the companion crate's: painted under kde-breeze and adwaita, egui's own
/// `CollapsingHeader` under material-3 and ios-13), whose title is one line box tall
/// (`demo::lined`), is exactly the rule's height. The Containers page's two egui headers in the
/// expander scope are the stated height at 1, and above 1 the taller of the rule and their plain
/// title with its padding. At 1.5 kde-breeze's 13 px title (its 10 pt in whole pixels,
/// `defaults.font_whole_pixels`) is 19.5, its line box 26.52 at its 1.36 line height, 27
/// rounded up, so a header with its 7 + 7 padding is 41, past the stated 40; its egui headers'
/// plain title is a 27 px row, also 41 with that padding. At 2 the line box is 35.36, 36
/// rounded up, so the rule is 50 where the title's 35 px row and its padding are 49: all three
/// headers are 50, the rule's height, not egui's own. At 2 adwaita's painted header is the
/// rule's 52 (36 and its 8 + 8) and its egui headers the rule's 54 (with 9 + 9; their title's
/// 35 px row 53), material-3's the rule's 61 (its 40 px row 60), and ios-13's 54 px row equals
/// its 53.95 line box rounded up, all three 58.
#[test]
fn the_expander_header_is_its_stated_height_by_the_row_rule() -> Result<(), String> {
    use native_theme::next::WidgetState;
    use native_theme_egui::next::{padding_of, stroke_width_of};
    let mut faults = Vec::new();
    for (preset, factor) in ["kde-breeze", "adwaita", "material-3", "ios-13"]
        .into_iter()
        .flat_map(|preset| [(preset, 1.0), (preset, 1.5), (preset, 2.0)])
    {
        let mut harness = open_page_scaled(Page::Basic, preset, factor);
        harness.run_steps(2);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or(format!("{preset}: no 0.8.0 theme installed"))?
            .clone();
        let e = &r.expander;
        let header = e.header(WidgetState::NONE);
        let label = e.header_label(WidgetState::NONE);
        let stated = header.min_size[1].ok_or(format!(
            "{preset} states no expander.header(NONE).min_size[1]"
        ))?;
        // the expander scope's vertical button padding above 1: §5's pair rule over the base
        // style's (src/style/cells.rs, `ExpanderCell::outside_padding`; src/convert.rs,
        // `button_padding_outside`), a stated side and, where the expander is framed, the
        // container's line width
        let own = harness.ctx.global_style().spacing.button_padding.y;
        let framed = e.frame_enabled == Some(true);
        let line: f32 = framed
            .then(|| stroke_width_of(&e.container(WidgetState::NONE)))
            .flatten()
            .into_iter()
            .sum();
        let [top, _, bottom, _] = padding_of(&header);
        let side = |stated: Option<f32>| stated.map_or(own, |v| v + line);
        let scope = f32::midpoint(side(top), side(bottom));
        // the painted header pads by each stated side, the scope's padding where unstated
        // (native-theme-egui-widgets `expander.rs`, `header`); egui's own by the scope's twice
        let painted = e.arrow_side.is_some()
            || e.arrow_gap.is_some()
            || e.content_indent.is_some()
            || e.frame_enabled.is_some();
        let padding = if painted {
            top.unwrap_or(scope) + bottom.unwrap_or(scope)
        } else {
            scope + scope
        };
        let want = next_rule(&r, stated, &label, padding);
        match harness
            .state()
            .registry
            .places()
            .get("basic.expander.details.header")
        {
            Some(rect) if (rect.height() - want).abs() <= 0.05 => {}
            Some(rect) => faults.push(format!(
                "{preset} at {factor}: the Basic header is {} tall, want {want}",
                rect.height()
            )),
            None => faults.push(format!(
                "{preset} at {factor}: the Basic header is not placed"
            )),
        }
        let mut harness = open_page_scaled(Page::Containers, preset, factor);
        harness.run_steps(2);
        // egui's own header is the taller of its least height, `interact_size.y`, and its title
        // and its padding above and below (`egui/src/containers/collapsing_header.rs:531-532`).
        // Above 1 the least height is the rule (`demo::grow_rows`), and the title is one row of
        // its face, which the connector never shortens to the theme's line box (§6.15), so a face
        // whose row is taller than the line box grows the header past the rule. The title's row
        // is read off its painted galley: the header is exactly the taller of the two, bounded
        // below by the rule and above by its title and padding.
        let rule = next_rule(&r, stated, &label, scope + scope);
        for (kind, title) in [
            (
                "CollapsingHeader (expander_icon)",
                "CollapsingHeader with expander_icon",
            ),
            ("ui.collapsing", "ui.collapsing, egui's own icon"),
        ] {
            // egui paints a header only where its clip shows it
            // (`egui/src/containers/collapsing_header.rs:598`), so the header is scrolled into
            // the page's view for its title to be painted; it is read by its whole rectangle
            // (`demo.rs`, `Record::whole`), as the view can be shorter than the header: under
            // ios-13 at 2 the Containers page's fixed nested panels leave its view 31.5 tall in
            // the 1280 x 720 window, the header 58.
            harness.get_by_label(title).scroll_to_me();
            harness.run_steps(4);
            let painted = harness
                .output()
                .shapes
                .iter()
                .find_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) if text.galley.text() == title => {
                        Some(text.galley.size().y)
                    }
                    _ => None,
                });
            let Some(row) = painted else {
                faults.push(format!(
                    "{preset} at {factor}: {kind}'s title is not painted"
                ));
                continue;
            };
            let want = if factor > 1.0 {
                rule.max(row + scope + scope)
            } else {
                stated
            };
            match harness
                .state()
                .registry
                .records()
                .iter()
                .find(|r| r.info.kind == kind)
            {
                Some(r) if (r.whole.height() - want).abs() <= 0.05 => {}
                Some(r) => faults.push(format!(
                    "{preset} at {factor}: {kind} is {} tall, want {want} (the rule {rule}, its \
                     title {row} and padding {scope} + {scope})",
                    r.whole.height()
                )),
                None => faults.push(format!("{preset} at {factor}: {kind} is not recorded")),
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
    Ok(())
}

/// The Containers page's `CollapsingHeader` draws its arrow with `next::expander_icon` of the
/// installed 0.8.0 theme: under kde-breeze with `expander.arrow(NONE).color` patched to
/// `colors.window.border`, a colour of the same theme the unpatched `expander.arrow(NONE).color`
/// is not, a shape in the header is filled with it.
#[test]
fn the_containers_expander_takes_the_08_arrow_colour() -> Result<(), String> {
    use native_theme::next::{ExpanderRule, IconPatch, ThemePatch};
    const KIND: &str = "CollapsingHeader (expander_icon)";
    let (mut harness, r) =
        open_patched(Page::Containers, egui::Theme::Light, "kde-breeze", |r| {
            Some(ThemePatch::new().expander(
                ExpanderRule::new().arrow(IconPatch::new().color(r.colors.window.border?)),
            ))
        })?;
    let colour = native_theme_egui::next::border_color(&r).map_err(|e| e.to_string())?;
    let old = open_page_under(Page::Containers, egui::Theme::Light, "kde-breeze")
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .map(|r| {
            let arrow = r.expander.arrow(native_theme::next::WidgetState::NONE);
            arrow
                .stated
                .contains(native_theme::next::IconStated::COLOR)
                .then(|| native_theme_egui::next::to_color32(arrow.color))
        })
        .ok_or("no 0.8.0 theme installed")?;
    assert_ne!(
        Some(colour),
        old,
        "the patched arrow is the unpatched expander.arrow(NONE).color"
    );
    harness
        .get_by_label("CollapsingHeader with expander_icon")
        .scroll_to_me();
    harness.run_steps(4);
    let header = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == KIND)
        .map(|r| r.rect)
        .ok_or("the expander header is not recorded")?;
    let drawn = harness
        .output()
        .shapes
        .iter()
        .any(|clipped| match &clipped.shape {
            egui::Shape::Path(path) => {
                path.fill == colour && path.points.iter().all(|p| header.expand(0.5).contains(*p))
            }
            _ => false,
        });
    assert!(
        drawn,
        "no arrow in {colour:?} inside the header at {header:?}"
    );
    Ok(())
}

/// The kinds drawn with egui's own selectable button (`Button::selectable`, frameless at rest
/// and framed when hovered), which are one border width narrower at rest on each side than
/// hovered under a theme whose buttons have a resting border: egui lays the frameless resting
/// button out with the inner margin of a frame, the button padding less the resting stroke's
/// width (`egui/src/widget_style.rs:163-165`), without painting that stroke
/// (`egui/src/widgets/button.rs:364-368`). No `Style` value keeps both a framed button's resting
/// border and a selectable's size, so it is egui's to change.
const GROWS_WHEN_HOVERED: &[&str] = &[
    "ui.toggle_value",
    "ui.selectable_label",
    "ui.selectable_value",
    "verdict",
];

/// No other widget changes size when hovered, as no native control does: on every page, under
/// every bundled preset in light and in dark, each installed on any platform
/// ([`open_page_under`]), each recorded widget keeps its rect with the pointer on it, but egui's
/// own selectables (`GROWS_WHEN_HOVERED`). One test per mode, page and preset.
mod no_widget_changes_size_when_hovered {
    /// A module per page of `$theme`'s mode, each a test per preset; the pages listed are
    /// every page.
    macro_rules! per_page_and_preset {
        ($theme:ident: $($module:ident = $page:ident),* $(,)?) => {
            const _: fn(super::super::Page) = |page| match page {
                $(super::super::Page::$page)|* => {}
            };
            $(mod $module {
                use super::super::super::*;
                per_preset!(
                    no_widget_changes_size_when_hovered_on,
                    Page::$page,
                    egui::Theme::$theme
                );
            })*
        };
    }
    mod light {
        per_page_and_preset!(Light:
            basic = Basic, buttons = Buttons, inputs = Inputs, selection = Selection,
            range = Range, data = Data, text = Text, containers = Containers,
            overlays = Overlays, colour = Colour, icons = Icons, theme_map = ThemeMap,
        );
    }
    mod dark {
        per_page_and_preset!(Dark:
            basic = Basic, buttons = Buttons, inputs = Inputs, selection = Selection,
            range = Range, data = Data, text = Text, containers = Containers,
            overlays = Overlays, colour = Colour, icons = Icons, theme_map = ThemeMap,
        );
    }
}

fn no_widget_changes_size_when_hovered_on(
    preset: native_theme::next::Preset,
    page: Page,
    theme: egui::Theme,
) -> Result<(), String> {
    let mut grew = Vec::new();
    let mut compared = 0usize;
    let preset = preset.as_str();
    let mut harness = open_page_under(page, theme, preset);
    harness.remove_cursor();
    harness.run_steps(2);
    let rest: Vec<(egui::Id, &'static str, egui::Rect)> = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| !r.container && !GROWS_WHEN_HOVERED.contains(&r.info.kind))
        .map(|r| (r.id, r.info.kind, r.rect))
        .collect();
    for (id, kind, rect) in rest {
        if !rect.is_positive() {
            continue;
        }
        harness.hover_at(rect.center());
        harness.run_steps(3);
        let now = harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.id == id)
            .map(|r| r.rect);
        if let Some(now) = now {
            compared += 1;
            if now.size() != rect.size() {
                grew.push(format!("{page:?}: {kind}: {rect:?} -> {now:?}"));
            }
        }
        harness.remove_cursor();
        harness.run_steps(2);
    }
    assert!(
        compared > 0,
        "{preset} {theme:?} {page:?}: no widget was compared"
    );
    assert!(
        grew.is_empty(),
        "{preset} {theme:?}: sized differently when hovered:\n  {}",
        grew.join("\n  ")
    );
    Ok(())
}

/// The side panel keeps `LEFT_PANEL_WIDTH` whatever Widget Info shows: hovering each widget of
/// the Basic page in turn shows infos short enough to fit and long enough to scroll, and the
/// panel is as wide after each as it opened. Stepped at 60 passes a second, as a display
/// draws the scroll bar's show animation over several passes (`ScrollArea`'s
/// `animate_bool_responsive`, `egui/src/containers/scroll_area.rs:751-761`). One test per preset
/// and mode.
mod the_side_panel_keeps_its_width_when_widget_info_scrolls {
    use super::*;

    #[test]
    fn kde_breeze_light() -> Result<(), String> {
        the_side_panel_keeps_its_width_under("kde-breeze", egui::Theme::Light)
    }

    #[test]
    fn test_preset_dark() -> Result<(), String> {
        the_side_panel_keeps_its_width_under(TEST_PRESET, egui::Theme::Dark)
    }
}

fn the_side_panel_keeps_its_width_under(preset: &str, theme: egui::Theme) -> Result<(), String> {
    const STEP: f32 = 1.0 / 60.0;
    let cli = cli(&[("--theme", TEST_PRESET), ("--tab", "basic")]);
    let mut harness = Harness::builder()
        .with_context_setup(emoji_fallbacks)
        .with_theme(theme)
        .with_size(crate::WINDOW_SIZE)
        .with_step_dt(STEP)
        .build_eframe(move |cc| {
            App::new(cc, &cli).expect("the showcase starts under a bundled preset")
        });
    harness.run_steps(4);
    let width_of = |h: &Harness<'_, App>| {
        h.state()
            .registry
            .records()
            .iter()
            .find(|r| r.info.kind == "Side panel")
            .map(|r| r.rect.width())
            .ok_or("the side panel is shown")
    };
    // The panel's record holds its splitter's line, in room its frame reserves after it: the
    // `splitter.divider_width` of the 0.8.0 theme it opened under. The panel is resizable, so
    // `preset`'s install keeps the width it opened at.
    let opened = crate::LEFT_PANEL_WIDTH
        + harness
            .state()
            .atlas
            .next_resolved(theme)
            .ok_or("no 0.8.0 theme installed")?
            .splitter
            .divider_width;
    assert_eq!(width_of(&harness)?, opened);
    install_under(&mut harness, theme, preset);
    // The harness turns animations off (`egui_kittest/src/lib.rs:144-150`); a display runs
    // them for egui's own `animation_time`.
    let animation_time = egui::Style::default().animation_time;
    harness
        .ctx
        .all_styles_mut(|style| style.animation_time = animation_time);
    harness.run_steps(4);
    assert_eq!(
        width_of(&harness)?,
        opened,
        "{preset} {theme:?}: the install changed the side panel's width"
    );
    let centres: Vec<(&'static str, egui::Pos2)> = harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| !r.container && r.rect.is_positive())
        .map(|r| (r.info.kind, r.rect.center()))
        .collect();
    let mut scrolled = false;
    for (kind, centre) in centres {
        harness.hover_at(centre);
        // Past `INFO_SETTLE` and the scroll bar's animation.
        harness.run_steps(40);
        scrolled |= harness.state().inspector_scrolls;
        assert_eq!(
            width_of(&harness)?,
            opened,
            "{preset} {theme:?}: the side panel changed width after {kind} was hovered"
        );
    }
    assert!(scrolled, "{preset} {theme:?}: no Widget Info scrolled");
    Ok(())
}

/// The side panel keeps `LEFT_PANEL_WIDTH` whatever a theme setting's drop-down shows: a
/// selected text wider than the panel is cut to its width with an ellipsis, as the drop-down's
/// rows are, and the content panel keeps its room. An icon-theme choice whose system theme was
/// unavailable was named with the reader's whole error, which on a desktop the detection does
/// not recognise, CI's runner among them, ran wider than the window: the panel widened to it and
/// pushed the content panel out of the window (M9c CI1). That label is short now,
/// `system (unavailable)` (`IconSetChoice`'s `Display`, M9c CI2), and an installed theme's name
/// can still run as long, so the test names one.
#[test]
fn a_setting_wider_than_the_side_panel_keeps_the_panel_its_width() {
    let mut harness = open(
        egui::Theme::Light,
        cli(&[("--theme", TEST_PRESET), ("--tab", "basic")]),
    );
    harness.run_steps(4);
    let place = |h: &Harness<'_, App>, id: &str| {
        h.state()
            .registry
            .places()
            .get(id)
            .copied()
            .unwrap_or_else(|| panic!("{id} is not placed"))
    };
    let (panel, content) = (
        place(&harness, "chrome.side_panel"),
        place(&harness, "chrome.content"),
    );
    assert_eq!(
        panel.width(),
        crate::LEFT_PANEL_WIDTH,
        "the side panel as it opens"
    );
    let long = "an-icon-theme-named-at-length-".repeat(12);
    harness.state_mut().settings.icon = IconSetChoice::Freedesktop(long.clone());
    harness.run_steps(4);
    let drawn = harness
        .output()
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) if text.galley.job.text == long => Some(text.galley.clone()),
            _ => None,
        });
    let Some(drawn) = drawn else {
        panic!("the icon-theme drop-down does not show the choice");
    };
    // The choice's theme has no icons, so the toolbar above may change height: the panels'
    // horizontal extents are the claim.
    let (after, moved) = (
        place(&harness, "chrome.side_panel"),
        place(&harness, "chrome.content"),
    );
    assert_eq!(
        after.x_range(),
        panel.x_range(),
        "the side panel changed width"
    );
    assert_eq!(
        moved.x_range(),
        content.x_range(),
        "the content panel lost its room"
    );
    let combo = place(&harness, "chrome.side_panel.settings.icon_theme");
    assert!(
        combo.right() <= panel.right(),
        "the drop-down {combo:?} leaves the side panel {panel:?}"
    );
    assert!(drawn.elided, "the drop-down's text is not cut");
}

/// Where the system icon theme cannot be detected, the Icon theme drop-down's hover text says
/// why, as its `system` row's label, `system (unavailable)`, leaves the reason out, and the
/// Icons page names the freedesktop set `freedesktop (unavailable)`; where it is detected, no
/// hover text says it is unavailable. The detection is the test's (`App::detect_icon_theme`), so
/// both cases run on every host.
#[test]
fn the_icon_theme_drop_down_says_on_hover_why_the_system_theme_is_unavailable() {
    fn failing() -> native_theme::Result<String> {
        Err(native_theme::error::Error::PlatformUnsupported {
            platform: "the test's failing detection",
        })
    }
    fn detected() -> native_theme::Result<String> {
        Ok("breeze".to_string())
    }
    const REASON: &str =
        "System icon theme: unavailable (platform not supported: the test's failing detection)";
    for (detect, failed) in [
        (failing as fn() -> native_theme::Result<String>, true),
        (detected, false),
    ] {
        let mut harness = open(
            egui::Theme::Light,
            cli(&[("--theme", TEST_PRESET), ("--tab", "basic")]),
        );
        harness.state_mut().detect_icon_theme = detect;
        harness.run_steps(4);
        let Some(combo) = harness
            .state()
            .registry
            .places()
            .get("chrome.side_panel.settings.icon_theme")
            .copied()
        else {
            panic!("the Icon theme drop-down is not placed");
        };
        hover_and_settle(&mut harness, combo.center());
        // One pass more than `hover_and_settle`'s three: egui opens the tooltip on the third,
        // once the pointer has been still for `tooltip_delay`, 0.5 s
        // (`egui-0.37.0/src/style.rs:1597`), two of kittest's 0.25 s steps after the hover's pass
        // (`egui_kittest-0.37.0/src/builder.rs:47`), as an invisible sizing pass
        // (`containers/popup.rs:629`, `containers/area.rs:691-693`), and egui 0.37 leaves an
        // invisible widget out of the AccessKit tree (`context.rs:655`, `is_accesskit_visible`),
        // so the hover text is in the tree from the fourth pass on (measured).
        harness.run_steps(1);
        assert_eq!(
            crate::pages::icons::set_label(native_theme::icons::IconSet::Freedesktop, None, detect),
            if failed {
                "freedesktop (unavailable)".to_string()
            } else {
                "freedesktop (breeze)".to_string()
            },
            "the Icons page's label for the system's freedesktop set"
        );
        if failed {
            assert!(
                harness.query_by_label(REASON).is_some(),
                "no hover text reads {REASON:?}"
            );
        } else {
            assert!(
                harness
                    .query_by_label_contains("System icon theme: unavailable")
                    .is_none(),
                "the system icon theme is detected, yet a hover text says it is unavailable"
            );
        }
    }
}

/// Where the system icon theme cannot be detected, the Icons page says why, in the shared crate's
/// text (`rows::icon_theme_unavailable`), as the iced and gpui showcases' Icons pages do; where it
/// is detected, it says nothing of the kind. The first arm takes the test's failing detection,
/// so it runs on every host; the second the system's own, which fails in the runner
/// environment (`scripts/run_in_runner_env.sh`) and on a desktop the detection does not know.
#[test]
fn the_icons_page_says_why_the_system_icon_theme_is_unavailable() {
    fn failing() -> native_theme::Result<String> {
        Err(native_theme::error::Error::PlatformUnsupported {
            platform: "the test's failing detection",
        })
    }
    for detect in [
        failing as fn() -> native_theme::Result<String>,
        native_theme::detect::system_icon_theme,
    ] {
        let mut harness = open(
            egui::Theme::Light,
            cli(&[("--theme", TEST_PRESET), ("--tab", "icons")]),
        );
        harness.state_mut().detect_icon_theme = detect;
        harness.run_steps(4);
        match detect() {
            Err(error) => {
                let reason = native_theme_showcase_support::rows::icon_theme_unavailable(&error);
                assert!(
                    harness.query_by_label(&reason).is_some(),
                    "the Icons page does not read {reason:?}"
                );
            }
            Ok(theme) => assert!(
                harness
                    .query_by_label_contains("System icon theme: unavailable")
                    .is_none(),
                "the system icon theme {theme:?} is detected, yet the Icons page says it is \
                 unavailable"
            ),
        }
    }
}

/// On the Basic page, `layout.section_gap` is the whole distance between two groups of a column
/// and between two columns, as on the iced and gpui Basic pages: the page lays five columns of
/// one width out that far apart, and in a column the space the page adds makes up the gap with
/// the `item_spacing` egui puts between widgets anyway.
#[test]
fn groups_and_columns_are_a_section_gap_apart() {
    let harness = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    let Some(gap) = harness.state().atlas.layout().section_gap else {
        panic!("kde-breeze states layout.section_gap");
    };
    let records = harness.state().registry.records();
    let rect = |kind: &str| {
        records
            .iter()
            .find(|r| r.info.kind == kind)
            .map(|r| r.rect)
            .unwrap_or_else(|| panic!("no {kind} record"))
    };
    let headings: Vec<egui::Rect> = records
        .iter()
        .filter(|r| r.info.kind == "heading")
        .map(|r| r.rect)
        .collect();
    // The Buttons group's last row, which ends with the dialog's default button, on a line of
    // its own where the row wraps, and the heading under it in the same column.
    let buttons = rect("toggle button (off)");
    let bottom = buttons
        .bottom()
        .max(rect("button (dialog default)").bottom());
    let Some(below) = headings
        .iter()
        .filter(|r| r.top() > bottom && (r.left() - buttons.left()).abs() < 0.5)
        .map(|r| r.top())
        .reduce(f32::min)
    else {
        panic!("no heading below the buttons");
    };
    assert!(
        (below - bottom - gap).abs() < 0.5,
        "the buttons and the next heading are {} apart, section_gap is {gap}",
        below - bottom
    );
    // Each column's first heading: five, one pitch apart, a pitch a column and a gap wide.
    let top = headings.iter().map(|r| r.top()).fold(f32::MAX, f32::min);
    let mut lefts: Vec<f32> = headings
        .iter()
        .filter(|r| (r.top() - top).abs() < 0.5)
        .map(|r| r.left())
        .collect();
    lefts.sort_by(f32::total_cmp);
    assert_eq!(lefts.len(), 5, "five columns: {lefts:?}");
    // One pitch apart, each left edge rounded to a whole pixel: two roundings apart at most.
    let pitch = (lefts[4] - lefts[0]) / 4.0;
    for pair in lefts.windows(2) {
        assert!(
            (pair[1] - pair[0] - pitch).abs() <= 1.0,
            "the columns are not one pitch apart: {lefts:?}"
        );
    }
    for (kind, column) in [
        ("TextEdit (hint)", 1),
        ("DragValue", 2),
        ("Label (typography)", 3),
        ("Table", 4),
    ] {
        assert!(
            (rect(kind).left() - lefts[column]).abs() < 0.5,
            "{kind} is not at column {column}'s left edge: {lefts:?}"
        );
    }
    // A column is the pitch less the gap wide: the first row of buttons ends before it does.
    let row = rect("button (suggested action)").right();
    assert!(
        row <= lefts[1] - gap + 0.5,
        "the first column's buttons end at {row}, past its edge {}",
        lefts[1] - gap
    );
}

/// Spec §10 rule 0 (M9-105 had none): under macos-tahoe's 0.8.0 theme, which states no
/// `layout.section_gap`, the Basic page puts egui's own gaps where the theme states none, as
/// `column_count` counts: adjacent columns' `Ui`s the page style's `item_spacing.x` apart, egui's
/// gap between columns (egui-0.37.0 `src/ui.rs:2665`), and in each column the second group's
/// heading `item_spacing.y` below the first group's bottom, egui's gap between the widgets of a
/// top-down `Ui` (`src/layout.rs:759`), and at its own `Ui`'s top.
#[test]
fn the_basic_columns_and_groups_are_egui_s_own_gap_apart_where_the_theme_states_none() {
    let harness = open_page_under(Page::Basic, egui::Theme::Light, "macos-tahoe");
    let state = harness.state();
    assert_eq!(
        state
            .atlas
            .next_resolved(egui::Theme::Light)
            .map(|r| r.layout.section_gap),
        Some(None),
        "macos-tahoe states no layout.section_gap"
    );
    let own = harness
        .ctx
        .style_of(egui::Theme::Light)
        .spacing
        .item_spacing;
    assert_eq!(
        column_rule(&harness, egui::Theme::Light).gap,
        own.x,
        "the rule counts egui's own gap between columns"
    );
    let places = state.registry.places();
    let columns: Vec<egui::Rect> = (1..)
        .map_while(|n| places.get(&format!("basic.column_{n}")).copied())
        .collect();
    assert!(columns.len() >= 3, "the columns: {columns:?}");
    // Each column's left edge is on a whole pixel and its width is the page's share, fraction
    // and all: two columns `own.x` apart are that within a pixel.
    for pair in columns.windows(2) {
        let gap = pair[1].left() - pair[0].right();
        assert!(
            (gap - own.x).abs() < 1.0,
            "two adjacent columns are {gap} apart, not {}: {columns:?}",
            own.x
        );
    }
    let records = state.registry.records();
    let groups = &state.registry.basic_groups;
    assert!(
        !groups.is_empty(),
        "the Basic page lays no group out in a Ui of its own"
    );
    for (n, column) in columns.iter().enumerate() {
        // The next column starts at this one's right edge, rounded to a whole pixel.
        let inside =
            |r: &egui::Rect| r.left() >= column.left() - 0.5 && r.left() < column.right() - 1.0;
        let mut stack: Vec<egui::Rect> = groups
            .iter()
            .map(|(_, rect)| *rect)
            .filter(inside)
            .collect();
        stack.sort_by(|a, b| a.top().total_cmp(&b.top()));
        let [first, second, ..] = stack[..] else {
            panic!("column {n}: fewer than two groups");
        };
        let Some(heading) = records
            .iter()
            .filter(|r| r.info.kind == "heading" && second.contains_rect(r.rect))
            .map(|r| r.rect.top())
            .reduce(f32::min)
        else {
            panic!("column {n}: the second group has no heading");
        };
        assert!(
            (heading - first.bottom() - own.y).abs() < 0.5 && (heading - second.top()).abs() < 0.5,
            "column {n}: the second group's heading starts {} below the first group's bottom, \
             not {}, {} below its own Ui's top",
            heading - first.bottom(),
            own.y,
            heading - second.top()
        );
    }
}

/// Kinds the Basic page records for each group's controls, and how many of each.
const BASIC_CONTROLS: [(&str, &[(&str, usize)]); 20] = [
    (
        "Buttons",
        &[
            ("button (enabled)", 1),
            ("button (suggested action)", 1),
            ("button (disabled)", 1),
            ("tooltip button", 1),
            ("toggle button (off)", 1),
            ("toggle button (on)", 1),
            ("button (dialog default)", 1),
        ],
    ),
    (
        "Checkboxes",
        &[
            ("checkbox (unchecked)", 1),
            ("checkbox (checked)", 1),
            ("checkbox (disabled)", 1),
        ],
    ),
    ("Radio buttons", &[("RadioButton", 2)]),
    ("Drop-down", &[("ComboBox", 1)]),
    (
        "Text inputs",
        &[
            ("TextEdit (hint)", 1),
            ("TextEdit (single line)", 1),
            ("TextEdit (disabled)", 1),
            ("TextEdit (focused)", 1),
        ],
    ),
    ("Text area", &[("TextEdit (multiline)", 1)]),
    ("Slider", &[("Slider (horizontal)", 1)]),
    (
        "Switches",
        &[
            ("switch (off)", 1),
            ("switch (on)", 1),
            ("switch (disabled)", 1),
        ],
    ),
    ("Number input", &[("DragValue", 1)]),
    ("Spinner", &[("Spinner", 1)]),
    ("Segmented control", &[("segmented control", 1)]),
    ("Card", &[("card", 1), ("card label", 1)]),
    (
        "Typography",
        &[
            ("Label (typography)", 4),
            ("Link", 1),
            ("Label (monospace)", 1),
        ],
    ),
    ("Separator", &[("Separator (horizontal)", 1)]),
    ("Progress bar", &[("ProgressBar", 1)]),
    ("List", &[("List", 1)]),
    // The icon buttons; the chosen set's open folder, where the set has one, is not counted: a
    // freedesktop theme the test's machine lacks shows none, never another set's.
    ("Icons", &[("icon button", 3), ("icon button, checked", 1)]),
    ("Tabs", &[("Tab · Basic", 2)]),
    (
        "Expander",
        &[
            ("Expander (expanded)", 1),
            ("expander body", 1),
            ("Expander (collapsed)", 1),
        ],
    ),
    ("Table", &[("Table", 1)]),
];

/// The Basic page shows every group of its spec (SC BASIC2), each in its column and in order:
/// under three presets, each group's heading is a label at its column's left edge below the
/// group before it, each column right of the one before, and every control of the group lies
/// between its heading and the next one. The list holds eight rows and shows three, the second
/// selected, its frame three rows and its border tall.
#[test]
fn the_basic_page_has_every_group_in_its_column() {
    use crate::pages::basic::{GROUPS, arrangement, column_count};
    let named: Vec<&str> = BASIC_CONTROLS.iter().map(|(group, _)| *group).collect();
    assert_eq!(
        GROUPS.to_vec(),
        named,
        "the controls are listed per group, in order"
    );
    for (preset, expected) in [("kde-breeze", 5), ("material-3", 4), ("adwaita", 5)] {
        let harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        let count = column_count(
            &column_rule(&harness, egui::Theme::Light),
            harness.state().demo_state.content_width,
            harness.state().demo_state.segmented_width,
        );
        assert_eq!(count, expected, "{preset}: the column count");
        let columns: Vec<Vec<&str>> = arrangement(count)
            .iter()
            .map(|members| {
                members
                    .iter()
                    .filter_map(|&ix| GROUPS.get(ix).copied())
                    .collect()
            })
            .collect();
        let records = harness.state().registry.records();
        let page_left = records
            .iter()
            .find(|r| r.info.kind == "Central panel")
            .map(|r| r.rect.left())
            .unwrap_or_else(|| panic!("{preset}: no Central panel record"));
        let heading = |name: &str| {
            harness
                .query_all_by_role_and_label(Role::Label, name)
                .map(|n| n.rect())
                .find(|r| r.left() >= page_left)
                .unwrap_or_else(|| panic!("{preset}: no heading {name:?} on the page"))
        };
        let mut column_left = f32::MIN;
        for column in &columns {
            let tops: Vec<egui::Rect> = column.iter().map(|name| heading(name)).collect();
            let left = tops[0].left();
            assert!(
                left > column_left,
                "{preset}: {column:?} is not right of the column before"
            );
            column_left = left;
            for (i, (name, rect)) in column.iter().zip(&tops).enumerate() {
                assert!(
                    (rect.left() - left).abs() < 0.5,
                    "{preset}: {name:?} is not at its column's left edge"
                );
                let next = tops.get(i + 1).map_or(f32::MAX, |r| r.top());
                assert!(
                    rect.top() < next,
                    "{preset}: {name:?} is not above the next group"
                );
                let Some((_, controls)) = BASIC_CONTROLS.iter().find(|(g, _)| g == name) else {
                    panic!("{name:?} lists no controls");
                };
                for (kind, count) in *controls {
                    let found: Vec<egui::Rect> = records
                        .iter()
                        .filter(|r| r.info.kind == *kind)
                        .map(|r| r.rect)
                        .collect();
                    assert_eq!(found.len(), *count, "{preset}: {kind} under {name:?}");
                    // A record's rect is what is shown of the widget: one the page scrolled
                    // out of view has none to place.
                    for r in found.into_iter().filter(|r| r.is_positive()) {
                        assert!(
                            r.top() >= rect.bottom() - 0.5
                                && r.bottom() <= next + 0.5
                                && r.left() >= left - 0.5,
                            "{preset}: {kind} at {r:?} is not under {name:?} at {rect:?}"
                        );
                    }
                }
            }
        }
        // The list's rows, which a page that scrolls may show only in part (material-3's).
        if harness.state().page_scrolls {
            continue;
        }
        // The list's frame line: `list.container(NONE)`'s stroke where the installed 0.8.0
        // theme states its width and colour, else egui's own frame's, none (`list_frame`).
        let line = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .and_then(|r| {
                let part = r.list.container(native_theme::next::WidgetState::NONE);
                native_theme_egui::next::stroke_width_of(&part)
                    .filter(|_| native_theme_egui::next::stroke_color_of(&part).is_some())
            })
            .unwrap_or(egui::Frame::NONE.stroke.width);
        let list = records
            .iter()
            .find(|r| r.info.kind == "List")
            .map(|r| r.rect)
            .unwrap_or_else(|| panic!("{preset}: no List record"));
        let rows: Vec<egui::Rect> = records
            .iter()
            .filter(|r| r.info.kind == "List row")
            .map(|r| r.rect)
            .collect();
        assert_eq!(rows.len(), 8, "{preset}: the list's rows");
        // A record holds the rect a widget can be hovered in, clipped to what is shown
        // (`Response::interact_rect`): a row scrolled out of the list has none.
        let shown = rows.iter().filter(|r| r.is_positive()).count();
        assert_eq!(shown, 3, "{preset}: the rows the list shows");
        let row = rows[0].height();
        assert!(
            (list.height() - (3.0 * row + 2.0 * line)).abs() < 0.5,
            "{preset}: the list is {} tall, three {row} rows and a {line} border",
            list.height()
        );
        assert!(
            (list.width() - crate::pages::basic::BASIC_WIDE).abs() < 0.5,
            "{preset}: the list is {} wide",
            list.width()
        );
        assert_eq!(
            harness.state().demo_state.basic_list,
            1,
            "Item 2 is selected"
        );
    }
}

/// The Basic page never needs sideways scrolling at the window's default size, 1280 × 720, in
/// both modes, under the three Linux presets, installed through the Theme row's path
/// ([`open_page_under`]): every element it places ends inside
/// the content panel, and it lays out the columns its rule counts from the panel's width. It may
/// be taller than its area (material-3 is): it then scrolls, and the test reports it. The
/// columns are as far apart as `column_count` counts them (M9-105), so the page fits at a
/// content width just above a count boundary too, where the counted columns hold the widest
/// control with nothing to spare: under macos-tahoe, which states no `layout.section_gap`,
/// installed directly (a preset of another platform here), at the five- and four-column
/// boundaries. It fits under every bundled preset at a text scale of 2.25 too (item 15(1) of the
/// round 2-3 review).
#[test]
fn the_basic_page_fits_the_window_across() {
    use crate::pages::basic::column_count;
    for preset in ["kde-breeze", "material-3", "adwaita"] {
        for theme in [egui::Theme::Light, egui::Theme::Dark] {
            let harness = open_page_under(Page::Basic, theme, preset);
            fits_across(&harness, theme, preset);
        }
    }
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        for columns in [5, 4] {
            let mut harness = open_page_under(Page::Basic, theme, "macos-tahoe");
            let rule = column_rule(&harness, theme);
            assert_eq!(
                harness
                    .state()
                    .atlas
                    .next_resolved(theme)
                    .map(|r| r.layout.section_gap),
                Some(None),
                "macos-tahoe {theme:?} states a section gap"
            );
            let width =
                count_boundary(&rule, harness.state().demo_state.segmented_width, columns) + 1.0;
            set_content_width(&mut harness, width);
            let state = harness.state();
            assert!(
                (state.demo_state.content_width - width).abs() < 0.5,
                "macos-tahoe {theme:?}: the content is {} wide, not {width}",
                state.demo_state.content_width
            );
            assert_eq!(
                column_count(
                    &column_rule(&harness, theme),
                    state.demo_state.content_width,
                    state.demo_state.segmented_width,
                ),
                columns,
                "macos-tahoe {theme:?}: {width} is just above the {columns}-column boundary"
            );
            fits_across(&harness, theme, &format!("macos-tahoe at {width}"));
        }
    }
    // Every bundled preset at the largest text scale (item 15(1) of the round 2-3 review).
    for preset in native_theme::next::Preset::ALL.iter().map(|p| p.as_str()) {
        let harness = open_page_scaled(Page::Basic, preset, 2.25);
        fits_across(&harness, egui::Theme::Light, &format!("{preset} at 2.25"));
    }
}

/// [`the_basic_page_fits_the_window_across`]'s checks on `harness` as it is: the page counts
/// its columns from the content panel's width, lays that many out, and places every element
/// inside the panel.
fn fits_across(harness: &Harness<'_, App>, theme: egui::Theme, what: &str) {
    use crate::pages::basic::column_count;
    let state = harness.state();
    let places = state.registry.places();
    let Some(panel) = places.get("chrome.content").copied() else {
        panic!("{what} {theme:?}: no content panel placed");
    };
    assert!(
        (panel.width() - state.demo_state.content_width).abs() < 0.5,
        "{what} {theme:?}: the panel is {} wide, the page counts from {}",
        panel.width(),
        state.demo_state.content_width
    );
    let columns = places
        .keys()
        .filter(|id| id.starts_with("basic.column_"))
        .count();
    let count = column_count(
        &column_rule(harness, theme),
        state.demo_state.content_width,
        state.demo_state.segmented_width,
    );
    assert_eq!(columns, count, "{what} {theme:?}: the columns laid out");
    for (id, rect) in places.iter().filter(|(id, _)| id.starts_with("basic.")) {
        assert!(
            rect.right() <= panel.right() + 0.5,
            "{what} {theme:?}: {id} ends at {}, past the panel's {}",
            rect.right(),
            panel.right()
        );
    }
    if state.page_scrolls {
        println!("{what} {theme:?}: the Basic page scrolls");
    }
}

/// The least content width at which `column_count` lays `columns` columns out under `rule`,
/// with a Segmented control `segmented` wide, to a hundredth of a point.
fn count_boundary(rule: &crate::pages::basic::ColumnRule, segmented: f32, columns: usize) -> f32 {
    use crate::pages::basic::column_count;
    let (mut fewer, mut enough) = (0.0_f32, 4.0 * crate::WINDOW_SIZE.x);
    assert!(
        column_count(rule, enough, segmented) >= columns,
        "{columns} columns at no width"
    );
    while enough - fewer > 0.01 {
        let mid = 0.5 * (fewer + enough);
        if column_count(rule, mid, segmented) >= columns {
            enough = mid;
        } else {
            fewer = mid;
        }
    }
    enough
}

/// The window resized so the content panel is `width` wide: the side panel keeps its width, so
/// the panel grows with the window.
fn set_content_width(harness: &mut Harness<'_, App>, width: f32) {
    let mut window = crate::WINDOW_SIZE;
    for _ in 0..3 {
        let now = harness.state().demo_state.content_width;
        if (now - width).abs() < 0.01 {
            return;
        }
        window.x += width - now;
        harness.set_size(window);
        harness.run_steps(4);
    }
}

// ---------------------------------------------------------------------------
// The selected tab's outward corners: tab.active_outward_corner_radius
// ---------------------------------------------------------------------------

/// The showcase on its Basic page in `theme`, windows-11 installed through the Theme row's path
/// (`install_under`: a preset of another platform, which `--theme` does not take here), then
/// its 0.8.0 themes changed and installed as an install keeps them (`OldAtlas::edited`,
/// `App::adopt_pair`), at rest: each 0.8.0 theme, which the three tab bars read
/// (`demo::TabLook::next`), with `tab.active_outward_corner_radius` set to `radius` and
/// `edit_next` made of it. The preset
/// states 4 (G3 T15); the tests set the radius on the resolved themes, whose fields are public,
/// to try it unstated and at other values too.
fn open_curving_out(
    theme: egui::Theme,
    radius: Option<f32>,
    edit_next: impl Fn(&mut native_theme::next::ResolvedTheme),
) -> Result<Harness<'static, App>, String> {
    open_curving_out_at(theme, radius, 1.0, edit_next)
}

/// [`open_curving_out`] at `pixels_per_point` physical pixels per point.
fn open_curving_out_at(
    theme: egui::Theme,
    radius: Option<f32>,
    pixels_per_point: f32,
    edit_next: impl Fn(&mut native_theme::next::ResolvedTheme),
) -> Result<Harness<'static, App>, String> {
    let mut harness = open_at(
        theme,
        cli(&[("--theme", TEST_PRESET), ("--tab", "basic")]),
        pixels_per_point,
    );
    harness.run_steps(4);
    install_under(&mut harness, theme, "windows-11");
    let atlas = harness.state().atlas.edited(|r| {
        r.tab.active_outward_corner_radius = radius;
        edit_next(r);
    })?;
    let ctx = harness.ctx.clone();
    harness.state_mut().adopt_pair(&ctx, atlas);
    harness.run_steps(4);
    harness.remove_cursor();
    harness.run_steps(2);
    Ok(harness)
}

/// The fill and the outline the selected tab `one_id` of the bar `bar_id` of [`TAB_BARS`] has
/// its flares painted in: the Basic bar's the ones egui draws that tab's frame in, its tab
/// scope's selected button's, which the connector writes from the installed 0.8.0 theme's
/// `tab.tab(SELECTED)` (`demo::TabLook::next`), on each of the three bars.
fn flare_paint(
    harness: &Harness<'_, App>,
    one_id: &str,
) -> Result<(egui::Color32, egui::Stroke), String> {
    let one = harness
        .state()
        .registry
        .places()
        .get(one_id)
        .copied()
        .ok_or_else(|| format!("{one_id} is not placed"))?;
    harness
        .output()
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            egui::Shape::Rect(s) if near_rect(s.rect, one) => Some((s.fill, s.stroke)),
            _ => None,
        })
        .ok_or_else(|| format!("{one_id}: no frame is painted on the tab"))
}

/// The three tab bars `demo::tab_bar` draws: each bar's element, its first tab's, which is the
/// selected one at start, and its second tab's.
const TAB_BARS: [[&str; 3]; 3] = [
    ["basic.tabs.bar", "basic.tabs.one", "basic.tabs.two"],
    [
        "chrome.page_tabs",
        "chrome.page_tabs.basic",
        "chrome.page_tabs.buttons",
    ],
    [
        "chrome.side_panel.inspector_tabs",
        "chrome.side_panel.inspector_tabs.widget",
        "chrome.side_panel.inspector_tabs.theme",
    ],
];

fn near(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

fn near_rect(a: egui::Rect, b: egui::Rect) -> bool {
    near(a.min.x, b.min.x)
        && near(a.min.y, b.min.y)
        && near(a.max.x, b.max.x)
        && near(a.max.y, b.max.y)
}

/// The circles of the frame clipped to `square`, with their places in its paint order.
fn circles_in(
    shapes: &[egui::epaint::ClippedShape],
    square: egui::Rect,
) -> Vec<(usize, egui::epaint::CircleShape)> {
    shapes
        .iter()
        .enumerate()
        .filter_map(|(ix, clipped)| match &clipped.shape {
            egui::Shape::Circle(circle) if near_rect(clipped.clip_rect, square) => {
                Some((ix, *circle))
            }
            _ => None,
        })
        .collect()
}

/// The nearest and the farthest of `centre` that epaint tessellates the shapes clipped to
/// `square` at, the anti-aliasing fringe included.
fn tessellated_extent(
    harness: &Harness<'_, App>,
    square: egui::Rect,
    centre: egui::Pos2,
) -> (f32, f32) {
    let clipped: Vec<_> = harness
        .output()
        .shapes
        .iter()
        .filter(|s| near_rect(s.clip_rect, square))
        .cloned()
        .collect();
    let ppp = harness.ctx.pixels_per_point();
    let distances: Vec<f32> = harness
        .ctx
        .tessellate(clipped, ppp)
        .into_iter()
        .filter_map(|p| match p.primitive {
            egui::epaint::Primitive::Mesh(mesh) => Some(mesh),
            egui::epaint::Primitive::Callback(_) => None,
        })
        .flat_map(|mesh| mesh.vertices.into_iter().map(|v| v.pos.distance(centre)))
        .collect();
    assert!(!distances.is_empty(), "nothing tessellated in {square:?}");
    (
        distances.iter().copied().fold(f32::INFINITY, f32::min),
        distances.iter().copied().fold(0.0, f32::max),
    )
}

/// The page tab of `page` as drawn: the part the strip's view leaves (`Record::rect`) and its
/// whole rectangle (`Record::whole`); the page tabs are recorded in `Page::ALL`'s order.
fn page_tab(harness: &Harness<'_, App>, page: Page) -> (egui::Rect, egui::Rect) {
    let index = Page::ALL
        .iter()
        .position(|p| *p == page)
        .expect("a page of Page::ALL");
    harness
        .state()
        .registry
        .records()
        .iter()
        .filter(|r| r.info.kind == "Tab · Page")
        .nth(index)
        .map(|r| (r.rect, r.whole))
        .unwrap_or_else(|| panic!("{page:?}: no page tab is recorded"))
}

/// `page`'s tab's left and right edges as shown and whole, unrounded, for a failure's message.
fn tab_edges(harness: &Harness<'_, App>, page: Page) -> String {
    let (shown, whole) = page_tab(harness, page);
    format!(
        "shown {} to {}, whole {} to {}",
        shown.left(),
        shown.right(),
        whole.left(),
        whole.right()
    )
}

/// Whether `page`'s tab lies wholly in the strip's view: the view leaves all of it, within half
/// a pixel, as the tests read egui's edges on its pixel grid: a tab scrolled to the view's end
/// can end a fraction of a pixel past it (the
/// no-default-features build: the Theme Map's tab whole to 1183.0313 in a view ending at 1183;
/// under `scripts/run_in_runner_env.sh`'s fonts, to 1204.25 in one ending at 1204).
fn page_tab_in_view(harness: &Harness<'_, App>, page: Page) -> bool {
    let (shown, whole) = page_tab(harness, page);
    let slack = 0.5 / harness.ctx.pixels_per_point();
    whole.width() > 0.0
        && (shown.left() - whole.left()).abs() <= slack
        && (shown.right() - whole.right()).abs() <= slack
}

/// The page tab strip brings the open page's tab wholly into view, as the iced showcase's strip
/// scrolls to it (`reveal_open_tab`), whichever way the page was opened: under material-3 at
/// the window's size, whose tabs are wider than the strip, the Theme Map's tab after the
/// showcase opens on it (`--tab theme-map`) and after a theme install, then the Basic tab and the
/// Theme Map's again after `Action::ShowPage`, which the palette, the menus and the pages
/// menu run. A sideways scroll of the user's own stands until the open page or the tabs change:
/// after a wheel over the strip, the Theme Map's tab is where the wheel moved it.
#[test]
fn the_page_tabs_scroll_the_open_tab_into_view() {
    let mut harness = open_scaled("material-3", "theme-map", 1.0);
    harness.run_steps(SETTLE);
    let cut = Page::ALL
        .iter()
        .filter(|p| !page_tab_in_view(&harness, **p))
        .count();
    assert!(
        cut > 0,
        "material-3: every page tab fits the strip, so none needs a scroll"
    );
    assert!(
        page_tab_in_view(&harness, Page::ThemeMap),
        "opened on theme-map: its tab is not wholly in view, {}",
        tab_edges(&harness, Page::ThemeMap)
    );
    let ctx = harness.ctx.clone();
    for page in [Page::Basic, Page::ThemeMap] {
        harness.state_mut().run_action(Action::ShowPage(page), &ctx);
        harness.run_steps(SETTLE);
        assert!(
            page_tab_in_view(&harness, page),
            "{page:?} shown: its tab is not wholly in view, {}",
            tab_edges(&harness, page)
        );
    }
    let before = page_tab(&harness, Page::ThemeMap).1;
    harness.hover_at(before.center());
    harness.event(egui::Event::MouseWheel {
        unit: egui::MouseWheelUnit::Point,
        delta: egui::vec2(before.width(), 0.0),
        phase: egui::TouchPhase::Move,
        source: egui::MouseWheelSource::Unknown,
        modifiers: egui::Modifiers::NONE,
    });
    harness.run_steps(SETTLE);
    let after = page_tab(&harness, Page::ThemeMap).1;
    assert!(
        after.left() > before.left() + ROW_SLACK,
        "a wheel over the strip moved nothing, or the strip scrolled back: the Theme Map's tab \
         at {} before, {} after",
        before.left(),
        after.left()
    );
}

/// docs/platform-facts.md §2.11 `active_outward_corner_radius`: under windows-11 with a radius
/// of 4 and its outline of 1, each of the three tab bars draws the selected tab's two flares
/// with the numbers every connector's test shares (spec §12.2) -- the left about
/// `(x0 - 3, y1 - 4)`, clipped to `[x0 - 3, x0 + 1] x [y1 - 4, y1]`, the right about
/// `(x1 + 3, y1 - 4)`, clipped to `[x1 - 1, x1 + 3] x [y1 - 4, y1]`; in each, the fill an
/// annulus from 3 to `4 * sqrt(2) + 1` in the selected tab's fill, then the outline one from 3
/// to 4 in its outline's colour ([`flare_paint`]: each bar's from the 0.8.0 tab scope).
/// epaint strokes a circle outside its
/// radius (`epaint-0.36.2` `src/tessellator.rs:1531`), so each annulus is a circle of its inner
/// radius stroked as wide as the annulus, and tessellated it reaches from 3 to
/// `4 * sqrt(2) + 1`. Under them, after every tab, a strip in the fill lays over the selected
/// tab's bottom edge, `[x0 + 1, x1 - 1] x [y1 - 1, y1]`. The first tab starts 4 further in than
/// with the radius unstated, and the Basic bar, as wide as its tabs, ends 4 after its last.
#[test]
fn the_tab_bars_draw_the_selected_tabs_outward_corners() -> Result<(), String> {
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let square_feet = open_curving_out(theme, None, |_| {})?;
        let harness = open_curving_out(theme, Some(4.0), |_| {})?;
        let state = harness.state();
        assert_eq!(
            state
                .atlas
                .next_resolved(theme)
                .map(|r| r.tab.active_outward_corner_radius),
            Some(Some(4.0))
        );
        let far = 4.0 * std::f32::consts::SQRT_2 + 1.0;
        let shapes = &harness.output().shapes;
        let places = state.registry.places();
        let place = |places: &std::collections::BTreeMap<String, egui::Rect>, id: &str| {
            places
                .get(id)
                .copied()
                .unwrap_or_else(|| panic!("{theme:?}: {id} is not placed"))
        };
        for [bar_id, one_id, two_id] in TAB_BARS {
            let (bar, one, two) = (
                place(places, bar_id),
                place(places, one_id),
                place(places, two_id),
            );
            let (fill, outline) = flare_paint(&harness, one_id)?;
            assert_eq!(
                outline.width, 1.0,
                "{theme:?} {bar_id}: windows-11's outline"
            );
            let outline = outline.color;
            let (x0, x1, y1) = (one.min.x, one.max.x, one.max.y);
            // (the square's left edge, the centre's x)
            let feet = [(x0 - 3.0, x0 - 3.0), (x1 - 1.0, x1 + 3.0)];
            let mut flares = Vec::new();
            for (left, cx) in feet {
                let square = egui::Rect::from_min_max(
                    egui::pos2(left, y1 - 4.0),
                    egui::pos2(left + 4.0, y1),
                );
                let centre = egui::pos2(cx, y1 - 4.0);
                let circles = circles_in(shapes, square);
                let expected = [(far - 3.0, fill), (1.0, outline)];
                assert_eq!(
                    circles.len(),
                    expected.len(),
                    "{theme:?} {one_id}: the flare in {square:?} is {circles:?}"
                );
                for ((ix, circle), (width, colour)) in circles.into_iter().zip(expected) {
                    assert!(
                        circle.center.distance(centre) < 1e-3
                            && near(circle.radius, 3.0)
                            && near(circle.stroke.width, width)
                            && circle.stroke.color == colour
                            && circle.fill == egui::Color32::TRANSPARENT,
                        "{theme:?} {one_id}: {circle:?} is not a ring from 3, {width} wide in \
                         {colour:?}, about {centre:?}"
                    );
                    flares.push(ix);
                }
                let (nearest, farthest) = tessellated_extent(&harness, square, centre);
                assert!(
                    (2.0..=3.5).contains(&nearest) && (far - 0.5..=far + 1.0).contains(&farthest),
                    "{theme:?} {one_id}: the flare is drawn from {nearest} to {farthest}, not \
                     from 3 to {far}"
                );
            }
            let strip =
                egui::Rect::from_min_max(egui::pos2(x0 + 1.0, y1 - 1.0), egui::pos2(x1 - 1.0, y1));
            let strips: Vec<usize> = shapes
                .iter()
                .enumerate()
                .filter_map(|(ix, clipped)| match &clipped.shape {
                    egui::Shape::Rect(r) if r.fill == fill && near_rect(r.rect, strip) => Some(ix),
                    _ => None,
                })
                .collect();
            let [strip_ix] = strips[..] else {
                panic!("{theme:?} {one_id}: the strip over its bottom edge is {strips:?}");
            };
            // Everything drawn within a tab, the strip and the flares aside.
            let in_tabs: Vec<usize> = shapes
                .iter()
                .enumerate()
                .filter(|(ix, clipped)| {
                    let at = clipped.shape.visual_bounding_rect();
                    *ix != strip_ix
                        && !flares.contains(ix)
                        && at.is_positive()
                        && [one, two]
                            .iter()
                            .any(|tab| tab.expand(0.5).contains_rect(at))
                })
                .map(|(ix, _)| ix)
                .collect();
            for tab in [one, two] {
                assert!(
                    shapes.iter().any(|c| {
                        let at = c.shape.visual_bounding_rect();
                        at.is_positive() && tab.expand(0.5).contains_rect(at)
                    }),
                    "{theme:?} {bar_id}: nothing drawn in the tab at {tab:?}"
                );
            }
            let first_flare = flares.iter().copied().min().unwrap_or_default();
            assert!(
                in_tabs.iter().all(|ix| *ix < strip_ix) && strip_ix < first_flare,
                "{theme:?} {bar_id}: the tabs at {in_tabs:?}, the strip at {strip_ix}, the \
                 flares at {flares:?}"
            );
            let before = square_feet.state().registry.places();
            let (bar_before, one_before) = (place(before, bar_id), place(before, one_id));
            assert!(
                near(
                    (one.min.x - bar.min.x) - (one_before.min.x - bar_before.min.x),
                    4.0
                ),
                "{theme:?} {one_id}: the first tab starts {} in, {} with the radius unstated",
                one.min.x - bar.min.x,
                one_before.min.x - bar_before.min.x
            );
        }
        let (bar, two) = (
            place(places, "basic.tabs.bar"),
            place(places, "basic.tabs.two"),
        );
        assert!(
            near(bar.max.x - two.max.x, 4.0),
            "{theme:?}: the Basic bar ends {} after its last tab",
            bar.max.x - two.max.x
        );
    }
    Ok(())
}

/// At a fractional scale, 1.25 physical pixels per point, each flare is placed from the selected
/// tab's rectangle rounded to physical pixels, the one epaint strokes the tab's frame on, so the
/// curve meets the side edge it continues: in each of the three bars the two flares' circles lie
/// about `(x0 - 3, y1 - 4)` and `(x1 + 3, y1 - 4)` of the rounded rectangle, clipped to their
/// squares about it; and some selected tab lies off the pixel grid, so the rounding shows.
#[test]
fn at_a_fractional_scale_the_flares_sit_on_the_tabs_pixel_grid() -> Result<(), String> {
    use egui::emath::GuiRounding as _;
    let ppp = 1.25;
    let harness = open_curving_out_at(egui::Theme::Light, Some(4.0), ppp, |_| {})?;
    assert!(near(harness.ctx.pixels_per_point(), ppp));
    let shapes = &harness.output().shapes;
    let places = harness.state().registry.places();
    let mut off_grid = Vec::new();
    for [_, one_id, _] in TAB_BARS {
        let Some(one) = places.get(one_id).copied() else {
            panic!("{one_id} is not placed");
        };
        let tab = one.round_to_pixels(ppp);
        if !near_rect(tab, one) {
            off_grid.push(one_id);
        }
        let (x0, x1, y1) = (tab.min.x, tab.max.x, tab.max.y);
        // (the square's left edge, the centre's x)
        for (left, cx) in [(x0 - 3.0, x0 - 3.0), (x1 - 1.0, x1 + 3.0)] {
            let square =
                egui::Rect::from_min_max(egui::pos2(left, y1 - 4.0), egui::pos2(left + 4.0, y1));
            let centre = egui::pos2(cx, y1 - 4.0);
            let circles = circles_in(shapes, square);
            assert!(
                circles.len() == 2
                    && circles
                        .iter()
                        .all(|(_, circle)| circle.center.distance(centre) < 1e-3),
                "{one_id} at {one:?}, {tab:?} rounded: the flare in {square:?} is {circles:?}, \
                 not two rings about {centre:?}"
            );
        }
    }
    assert!(
        !off_grid.is_empty(),
        "no selected tab lies off the pixel grid at {ppp}, so the rounding is not seen"
    );
    Ok(())
}

/// Without an outline (`tab.tab(SELECTED)`'s stroke none on the installed 0.8.0 theme), each
/// flare is the fill alone, an annulus from the radius, 4, to `4 * sqrt(2) + 1`, in a square at
/// the tab's outer edge, and no strip covers a bottom edge the tab does not have.
#[test]
fn without_an_outline_the_fill_begins_at_the_radius() -> Result<(), String> {
    use native_theme::next::{PartPatch, Stroke, TabRule, ThemePatch, WidgetState};
    let harness = open_curving_out(egui::Theme::Light, Some(4.0), |r| {
        r.apply_patch(
            &ThemePatch::new().tab(
                TabRule::new()
                    .states(WidgetState::SELECTED)
                    .tab(PartPatch::new().stroke(Stroke::None)),
            ),
        );
    })?;
    let far = 4.0 * std::f32::consts::SQRT_2 + 1.0;
    let shapes = &harness.output().shapes;
    for [bar_id, one_id, _] in TAB_BARS {
        let (fill, outline) = flare_paint(&harness, one_id)?;
        assert_eq!(outline.width, 0.0, "{bar_id}: the outline is still drawn");
        let one = harness
            .state()
            .registry
            .places()
            .get(one_id)
            .copied()
            .ok_or_else(|| format!("{one_id} is not placed"))?;
        let (x0, x1, y1) = (one.min.x, one.max.x, one.max.y);
        for (left, cx) in [(x0 - 4.0, x0 - 4.0), (x1, x1 + 4.0)] {
            let square =
                egui::Rect::from_min_max(egui::pos2(left, y1 - 4.0), egui::pos2(left + 4.0, y1));
            let centre = egui::pos2(cx, y1 - 4.0);
            let circles = circles_in(shapes, square);
            assert!(
                matches!(circles[..], [(_, circle)] if circle.center.distance(centre) < 1e-3
                    && near(circle.radius, 4.0)
                    && near(circle.stroke.width, far - 4.0)
                    && circle.stroke.color == fill),
                "{one_id}: the flare in {square:?} is {circles:?}"
            );
        }
        assert!(
            !shapes.iter().any(|c| matches!(&c.shape,
                egui::Shape::Rect(r) if r.fill == fill && near(r.rect.max.y, y1)
                    && near(r.rect.height(), 0.0))),
            "{one_id}: a strip over a bottom edge of no width"
        );
    }
    Ok(())
}

/// With the radius unstated on the installed 0.8.0 theme, or stated as one no tab bar draws (0,
/// not finite), no circle is drawn and no strip: the selected tab keeps its bottom edge, where
/// the unpatched windows-11 0.8.0 theme states 4.
#[test]
fn without_the_radius_the_tab_bars_draw_no_flares() -> Result<(), String> {
    let mut plain = open(
        egui::Theme::Light,
        cli(&[("--theme", TEST_PRESET), ("--tab", "basic")]),
    );
    install_under(&mut plain, egui::Theme::Light, "windows-11");
    let unpatched = plain
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme installed")?
        .tab
        .active_outward_corner_radius;
    assert_eq!(
        unpatched,
        Some(4.0),
        "the unpatched windows-11 states no radius of 4, so the case proves nothing"
    );
    for radius in [None, Some(0.0), Some(f32::NAN)] {
        let harness = open_curving_out(egui::Theme::Light, radius, |_| {})?;
        let shapes = &harness.output().shapes;
        let circles: Vec<_> = shapes
            .iter()
            .filter(|c| match &c.shape {
                egui::Shape::Circle(circle) => TAB_BARS.iter().any(|[bar, ..]| {
                    harness
                        .state()
                        .registry
                        .places()
                        .get(*bar)
                        .is_some_and(|b| b.expand(8.0).contains(circle.center))
                }),
                _ => false,
            })
            .collect();
        assert!(circles.is_empty(), "{radius:?}: circles drawn: {circles:?}");
        for [_, one_id, _] in TAB_BARS {
            let (fill, _) = flare_paint(&harness, one_id)?;
            let one = harness
                .state()
                .registry
                .places()
                .get(one_id)
                .copied()
                .ok_or_else(|| format!("{one_id} is not placed"))?;
            let strip = egui::Rect::from_min_max(
                egui::pos2(one.min.x + 1.0, one.max.y - 1.0),
                egui::pos2(one.max.x - 1.0, one.max.y),
            );
            assert!(
                !shapes.iter().any(|c| matches!(&c.shape,
                    egui::Shape::Rect(r) if r.fill == fill && near_rect(r.rect, strip))),
                "{radius:?} {one_id}: a strip over the bottom edge"
            );
        }
    }
    Ok(())
}

/// With a bottom indicator and the radius both stated, no strip is painted, since the line
/// covers the bottom edge, and the flares are painted over the line as without it, on each of
/// the three bars: the installed 0.8.0 theme's `tab.indicator(SELECTED)`'s fill and exact height
/// and `tab.indicator_side`.
#[test]
fn a_bottom_indicator_lies_under_the_flares() -> Result<(), String> {
    use native_theme::next::{
        Color, Paint, PartPatch, TabIndicatorSide, TabRule, ThemePatch, WidgetState,
    };
    let magenta = Color::from_srgb8(255, 0, 255, 255);
    let harness = open_curving_out(egui::Theme::Light, Some(4.0), |r| {
        r.apply_patch(
            &ThemePatch::new().tab(
                TabRule::new().states(WidgetState::SELECTED).indicator(
                    PartPatch::new()
                        .fill(Paint::Solid(magenta))
                        .min_size([None, Some(2.0)])
                        .max_size([None, Some(2.0)]),
                ),
            ),
        );
        r.tab.indicator_side = Some(TabIndicatorSide::Bottom);
    })?;
    let shapes = &harness.output().shapes;
    let accent = egui::Color32::from_rgb(255, 0, 255);
    for [_, one_id, _] in TAB_BARS {
        let (fill, _) = flare_paint(&harness, one_id)?;
        let one = harness
            .state()
            .registry
            .places()
            .get(one_id)
            .copied()
            .ok_or_else(|| format!("{one_id} is not placed"))?;
        let (x0, x1, y1) = (one.min.x, one.max.x, one.max.y);
        let strip =
            egui::Rect::from_min_max(egui::pos2(x0 + 1.0, y1 - 1.0), egui::pos2(x1 - 1.0, y1));
        assert!(
            !shapes.iter().any(|c| matches!(&c.shape,
                egui::Shape::Rect(r) if r.fill == fill && near_rect(r.rect, strip))),
            "{one_id}: a strip under the bottom line"
        );
        let line = egui::Rect::from_min_max(egui::pos2(x0, y1 - 2.0), one.max);
        let lines: Vec<usize> = shapes
            .iter()
            .enumerate()
            .filter_map(|(ix, c)| match &c.shape {
                egui::Shape::Rect(r) if r.fill == accent && near_rect(r.rect, line) => Some(ix),
                _ => None,
            })
            .collect();
        let [line_ix] = lines[..] else {
            return Err(format!("{one_id}: the bottom line is {lines:?}"));
        };
        for left in [x0 - 3.0, x1 - 1.0] {
            let square =
                egui::Rect::from_min_max(egui::pos2(left, y1 - 4.0), egui::pos2(left + 4.0, y1));
            let circles = circles_in(shapes, square);
            assert_eq!(
                circles.len(),
                2,
                "{one_id}: the flare in {square:?} is {circles:?}"
            );
            assert!(
                circles.iter().all(|(ix, _)| line_ix < *ix),
                "{one_id}: the line at {line_ix} over the flares at {circles:?}"
            );
        }
    }
    Ok(())
}

/// The Basic page's column rule makes room for both of the selected tab's flares, as the gpui
/// page's does: under windows-11's 0.8.0 theme, whose widest fixed-width control is the two
/// tabs, a page whose fifth holds the two tabs and 4px more lays out five columns, and four once
/// the theme states an outward corner radius of 4, whose two flares need 8; five again for a
/// radius the tab bar draws no flares for, 0, negative or not finite. The page counts on the
/// installed 0.8.0 theme: a patched radius lays out fewer columns.
#[test]
fn the_basic_page_makes_room_for_both_flares() -> Result<(), String> {
    use crate::pages::basic::{ColumnRule, column_count};
    use native_theme::next::{Preset, Theme, ThemeContext, WidgetState};
    let own = egui::Style::default();
    let mut stated = Theme::preset(Preset::Windows11).resolve(&ThemeContext::light());
    stated.tab.active_outward_corner_radius = Some(4.0);
    let mut unstated = stated.clone();
    unstated.tab.active_outward_corner_radius = None;
    let rule = ColumnRule::of(Some(&unstated), &own);
    let gutter = if rule.overlay { 0.0 } else { rule.groove };
    let tab_min = stated.tab.tab(WidgetState::NONE).min_size[0]
        .ok_or("windows-11 states no tab.tab(NONE).min_size[0]")?;
    let tabs = tab_min + tab_min + stated.tab.item_gap.unwrap_or_default();
    assert_eq!(
        rule.widest, tabs,
        "windows-11's widest fixed-width control is the two tabs"
    );
    let content = 5.0 * (tabs + 4.0) + 4.0 * rule.gap + rule.margin + rule.margin + gutter;
    assert_eq!(
        column_count(&rule, content, 0.0),
        5,
        "without outward corners the two tabs fit a fifth"
    );
    assert_eq!(
        column_count(&ColumnRule::of(Some(&stated), &own), content, 0.0),
        4,
        "the two tabs and both flares do not fit a fifth"
    );
    for radius in [0.0, -1.0, f32::NAN] {
        let mut undrawn = stated.clone();
        undrawn.tab.active_outward_corner_radius = Some(radius);
        assert_eq!(
            column_count(&ColumnRule::of(Some(&undrawn), &own), content, 0.0),
            5,
            "a radius of {radius} keeps room for flares the bar does not draw"
        );
    }
    // The page counts on the installed 0.8.0 theme: kde-breeze, whose tabs curve out on
    // neither model, patched to a radius whose two flares fill a fifth of the page, lays out
    // fewer columns than it does unpatched.
    let laid = |harness: &Harness<'_, App>| {
        harness
            .state()
            .registry
            .places()
            .keys()
            .filter(|id| id.starts_with("basic.column_"))
            .count()
    };
    let stock = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    assert_eq!(
        stock
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .map(|r| r.tab.active_outward_corner_radius),
        Some(None),
        "kde-breeze's tabs curve out"
    );
    let flare = stock.state().demo_state.content_width / 10.0;
    let (patched, r) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", |_| {
        Some(
            native_theme::next::ThemePatch::new().widget_rules(
                native_theme::next::WidgetPatchMap::new()
                    .tab(native_theme::next::TabPatch::new().active_outward_corner_radius(flare)),
            ),
        )
    })?;
    assert_eq!(r.tab.active_outward_corner_radius, Some(flare));
    assert!(
        laid(&patched) < laid(&stock),
        "kde-breeze: {} columns under a patched outward radius of {flare}, {} without",
        laid(&patched),
        laid(&stock)
    );
    Ok(())
}

/// Spec §10 rule 0, per length: the Basic page's column rule takes a layout length the theme
/// leaves unstated as the space egui itself puts at that place, never 0 — the page margin as
/// egui's own central panel margin (egui-0.37.0 `src/containers/frame.rs:191-193`,
/// `inner_margin(8)`), the gap between columns and between a row's widgets as the page style's
/// `item_spacing.x` (`Ui::columns`, `src/ui.rs:2665`) — under kde-breeze's 0.8.0 theme with its
/// window margin, section gap and widget gap unstated, and without a 0.8.0 theme; a stated length
/// is the theme's. The page style's item spacing is set apart from egui's default, so the rule is
/// seen to read it.
#[test]
fn the_column_rule_takes_an_unstated_layout_length_as_egui_s_own() -> Result<(), String> {
    use crate::pages::basic::ColumnRule;
    use native_theme::next::{Preset, Theme, ThemeContext};
    let mut own = egui::Style::default();
    own.spacing.item_spacing = egui::vec2(5.0, 2.0);
    let stated = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
    let mut unstated = stated.clone();
    unstated.layout.window_margin = None;
    unstated.layout.section_gap = None;
    unstated.layout.widget_gap = None;
    for (what, r) in [("unstated", Some(&unstated)), ("no 0.8.0 theme", None)] {
        let rule = ColumnRule::of(r, &own);
        assert_eq!(rule.margin, 8.0, "{what}: egui's own central panel margin");
        assert_eq!(rule.gap, 5.0, "{what}: egui's own gap between columns");
        assert_eq!(
            rule.widget_gap, 5.0,
            "{what}: egui's own gap between a row's widgets"
        );
    }
    let rule = ColumnRule::of(Some(&stated), &own);
    let layout = &stated.layout;
    assert_eq!(
        Some(rule.margin),
        layout.window_margin,
        "kde-breeze's window margin"
    );
    assert_eq!(
        Some(rule.gap),
        layout.section_gap,
        "kde-breeze's section gap"
    );
    assert_eq!(
        Some(rule.widget_gap),
        layout.widget_gap,
        "kde-breeze's widget gap"
    );
    let margin = layout
        .window_margin
        .ok_or("kde-breeze states no window margin")?;
    assert_ne!(margin, 8.0, "kde-breeze's margin is not egui's own");
    Ok(())
}

/// Spec §10 rule 0 for the scroll bar (G2 review fix 3): the connector writes the egui scroll
/// style only where the theme states both the groove's and the thumb's width (`style/derived.rs`,
/// `apply_base`), else egui's own `ScrollStyle` stays, so the column rule takes the bar's
/// floating and its width both from the theme only then, and else both from the page style:
/// under every bundled preset in both schemes the rule's bar is the one the installed style
/// lays out, floating where egui's style floats and as wide as it allocates (its `bar_width`
/// where it floats, `allocated_width()` where it does not); and with a page style whose scroll
/// style is set apart from egui's default, the rule takes the theme's bar exactly where both
/// widths are stated, and the page style's where either is not (every bundled preset states
/// both today, in both schemes, so only a theme of the future reaches the page style's bar).
#[test]
fn the_column_rule_takes_the_scroll_bar_the_connector_lays_out() -> Result<(), String> {
    use crate::pages::basic::ColumnRule;
    use native_theme::next::{Orientation, Preset, StyleQuery, WidgetState};
    let vertical = || StyleQuery::<()>::new(WidgetState::NONE).orientation(Orientation::Vertical);
    let mut own = egui::Style::default();
    own.spacing.scroll.floating = !own.spacing.scroll.floating;
    own.spacing.scroll.bar_width += 3.0;
    for preset in Preset::ALL.iter().map(|p| p.as_str()) {
        for theme in [egui::Theme::Light, egui::Theme::Dark] {
            let harness = open_page_under(Page::Basic, theme, preset);
            let r = harness
                .state()
                .atlas
                .next_resolved(theme)
                .ok_or_else(|| format!("{preset} {theme:?}: no 0.8.0 theme installed"))?;
            let style = harness.ctx.style_of(theme);
            let laid = &style.spacing.scroll;
            let rule = column_rule(&harness, theme);
            assert_eq!(
                rule.overlay, laid.floating,
                "{preset} {theme:?}: the rule's bar floats where the installed style's does not, \
                 or the reverse"
            );
            let width = if laid.floating {
                laid.bar_width
            } else {
                laid.allocated_width()
            };
            assert!(
                (rule.groove - width).abs() < 1e-4,
                "{preset} {theme:?}: the rule's groove {} against the installed style's {width}",
                rule.groove
            );
            let groove = r.scrollbar.track(vertical()).min_size[0];
            let thumb = r.scrollbar.thumb(vertical()).min_size[0];
            let apart = ColumnRule::of(Some(r), &own);
            match (groove, thumb) {
                (Some(groove), Some(_)) => {
                    assert_eq!(
                        (apart.overlay, apart.groove),
                        (r.scrollbar.overlay_mode, groove),
                        "{preset} {theme:?}: both widths stated, the bar is not the theme's"
                    );
                }
                _ => assert_eq!(
                    (apart.overlay, apart.groove),
                    (own.spacing.scroll.floating, own.spacing.scroll.bar_width),
                    "{preset} {theme:?}: a width unstated, the bar is not the page style's"
                ),
            }
        }
    }
    Ok(())
}

/// Spec §10 rule 0 for `tab.item_gap` (ES-G2d question 3): the column rule counts the two tabs
/// the gap the tab bar lays them out apart, `tab.item_gap` where stated and else the page
/// style's `item_spacing.x`, which the tab scope keeps (`demo::TabLook::next`), never 0: under
/// windows-11's 0.8.0 theme, whose widest fixed-width control is the two tabs, with its tab gap
/// unstated, the widest control grows with the page style's `item_spacing.x`, set to two
/// different values; with the gap stated, it is the theme's whatever the style's.
#[test]
fn the_column_rule_counts_an_unstated_tab_gap_as_the_rows_own() -> Result<(), String> {
    use crate::pages::basic::ColumnRule;
    use native_theme::next::{Preset, Theme, ThemeContext, WidgetState};
    let stated = Theme::preset(Preset::Windows11).resolve(&ThemeContext::light());
    let mut unstated = stated.clone();
    unstated.tab.item_gap = None;
    let tab_min = stated.tab.tab(WidgetState::NONE).min_size[0]
        .ok_or("windows-11 states no tab.tab(NONE).min_size[0]")?;
    let flare = crate::demo::next_outward_radius(&stated).unwrap_or_default();
    let item_gap = stated
        .tab
        .item_gap
        .ok_or("windows-11 states no tab.item_gap")?;
    let mut widest = Vec::new();
    for x in [5.0, 13.0] {
        let mut own = egui::Style::default();
        own.spacing.item_spacing.x = x;
        let tabs = 2.0 * tab_min + 2.0 * flare;
        let rule = ColumnRule::of(Some(&unstated), &own);
        assert_eq!(
            rule.widest,
            tabs + x,
            "item_spacing.x {x}: windows-11's two tabs with the gap unstated are not the widest \
             control, or not that gap apart"
        );
        assert_eq!(
            ColumnRule::of(Some(&stated), &own).widest,
            tabs + item_gap,
            "item_spacing.x {x}: a stated tab gap is not the theme's"
        );
        widest.push(rule.widest);
    }
    assert_ne!(
        widest.first(),
        widest.last(),
        "the rule reads no item spacing"
    );
    Ok(())
}

/// The Basic page's tabs on the 0.8.0 model (`demo::TabLook::next`): under each bundled preset
/// that leaves `tab.item_gap` unstated (ios-13, macos-monterey, macos-tahoe, material-3), the two
/// tabs stand the tab scope's own `item_spacing.x` apart (spec §10 rule 0), the gap egui lays a
/// row's widgets out by and where stated the connector's `layout.widget_gap`, not side by side
/// as 0.7.2 drew them, and the bar's Widget Info names it; under one stating it (kde-breeze),
/// the stated gap. A patched `tab.tab(NONE).min_size[0]` widens the tabs, so the page reads the
/// 0.8.0 theme.
#[test]
fn the_basic_tabs_stand_the_tab_scopes_gap_apart() -> Result<(), String> {
    use native_theme::next::{PartPatch, Preset, TabRule, ThemePatch};
    let mut unstated = Vec::new();
    for preset in Preset::ALL.iter().map(|p| p.as_str()) {
        let harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        let state = harness.state();
        let r = state
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or_else(|| format!("{preset}: no 0.8.0 theme installed"))?;
        // The tab scope's `item_spacing.x`: `tab.item_gap`, else the base style's, which the
        // connector writes from `layout.widget_gap`, else egui's own (egui-0.37.0
        // `src/style.rs:1565`).
        let gap = match r.tab.item_gap {
            Some(gap) => gap,
            None => {
                unstated.push(preset);
                r.layout
                    .widget_gap
                    .unwrap_or(egui::Style::default().spacing.item_spacing.x)
            }
        };
        let places = state.registry.places();
        let [one, two] = ["basic.tabs.one", "basic.tabs.two"].map(|id| places.get(id).copied());
        let (Some(one), Some(two)) = (one, two) else {
            return Err(format!("{preset}: a Basic tab is not placed"));
        };
        assert!(
            near(two.left() - one.right(), gap),
            "{preset}: the Basic tabs stand {} apart, the tab scope's gap is {gap}",
            two.left() - one.right()
        );
        if r.tab.item_gap.is_none() {
            assert!(gap > 0.0, "{preset}: the tab scope's own gap is {gap}");
            let why: Vec<&str> = state
                .registry
                .records()
                .iter()
                .filter(|record| record.info.kind == "TabBar · Basic")
                .flat_map(|record| record.info.not_themeable.iter())
                .filter(|(what, why)| {
                    *what == "the gap between tabs" && why.contains("tab.item_gap")
                })
                .map(|(_, why)| why.as_str())
                .collect();
            assert!(
                !why.is_empty(),
                "{preset}: the Basic bar's Widget Info names no unstated tab gap"
            );
            // the gap's home: layout.widget_gap where the theme states it, else egui's own
            assert!(
                why.iter().all(|why| {
                    why.contains("layout.widget_gap") == r.layout.widget_gap.is_some()
                        && why.contains("egui's own") == r.layout.widget_gap.is_none()
                }),
                "{preset}: the unstated tab gap named {why:?}, layout.widget_gap {:?}",
                r.layout.widget_gap
            );
        }
    }
    assert_eq!(
        unstated,
        ["macos-monterey", "macos-tahoe", "material-3", "ios-13"],
        "the presets leaving tab.item_gap unstated"
    );
    let stock = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    let one = stock
        .state()
        .registry
        .places()
        .get("basic.tabs.one")
        .copied()
        .ok_or("kde-breeze: the first Basic tab is not placed")?;
    let wide = 2.0 * one.width();
    let (patched, r) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", |_| {
        Some(
            ThemePatch::new()
                .tab(TabRule::new().tab(PartPatch::new().min_size([Some(wide), None]))),
        )
    })?;
    assert_eq!(
        r.tab.tab(native_theme::next::WidgetState::NONE).min_size[0],
        Some(wide)
    );
    let one = patched
        .state()
        .registry
        .places()
        .get("basic.tabs.one")
        .copied()
        .ok_or("kde-breeze patched: the first Basic tab is not placed")?;
    assert!(
        near(one.width(), wide),
        "kde-breeze: the first Basic tab is {} wide under a least width of {wide}",
        one.width()
    );
    Ok(())
}

/// A page's section headings are set in the installed 0.8.0 theme's section-heading role
/// (`next::text_role_font`, `text_role_line_height`, `text_role_weight`), as the iced
/// showcase's section titles are: its size and line height, and at the body's weight
/// (`next::font_weight`) in the proportional family; at another weight (adwaita's) in the face
/// registered at that weight, where the fonts hold one, and the proportional family where they
/// do not.
#[test]
fn section_headings_take_the_section_heading_role() -> Result<(), String> {
    use native_theme_egui::next;
    let mut other_weight = Vec::new();
    for preset in ["kde-breeze", TEST_PRESET, "adwaita"] {
        let harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .cloned()
            .ok_or_else(|| format!("{preset}: no 0.8.0 theme installed"))?;
        let role = native_theme_egui::TextRole::SectionHeading;
        let size = next::text_role_font(&r, role).size;
        let line_height = next::text_role_line_height(&r, role);
        // The page's own "Buttons" heading, not the page tab of that name.
        let headings: Vec<egui::Rect> = harness
            .state()
            .registry
            .records()
            .iter()
            .filter(|r| r.info.kind == "heading")
            .map(|r| r.rect)
            .collect();
        let shapes = harness.output().shapes.clone();
        let heading = shapes.iter().find_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text)
                if text.galley.job.text == "Buttons"
                    && headings.iter().any(|r| r.contains(text.pos)) =>
            {
                text.galley.job.sections.first().map(|s| s.format.clone())
            }
            _ => None,
        });
        let Some(format) = heading else {
            panic!("{preset}: no \"Buttons\" heading was painted");
        };
        assert_eq!(format.font_id.size, size, "{preset}: heading size");
        assert_eq!(
            format.line_height,
            Some(line_height),
            "{preset}: heading line height"
        );
        let weight = next::text_role_weight(&r, role);
        let body = next::font_weight(&r).map_err(|e| format!("{preset}: {e}"))?;
        if weight == body {
            assert_eq!(
                format.font_id.family,
                egui::FontFamily::Proportional,
                "{preset}: a heading at the body's weight is in the body's face"
            );
        } else {
            other_weight.push(preset);
            let named = crate::demo::weight_family(weight);
            let held = harness
                .ctx
                .fonts(|f| f.definitions().families.contains_key(&named));
            let expected = if held {
                named
            } else {
                egui::FontFamily::Proportional
            };
            assert_eq!(
                format.font_id.family, expected,
                "{preset}: a heading at {weight} over a {body} body"
            );
        }
    }
    assert!(
        other_weight.contains(&"adwaita"),
        "adwaita's heading is not at the body's weight: {other_weight:?}"
    );
    Ok(())
}

/// `--pointer X,Y` is two whole logical pixels; anything else holds no pointer and is kept for
/// `main` to report, a missing value included.
#[test]
fn the_pointer_flag_is_a_point() {
    let parse = |args: &[&str]| CliArgs::parse(args.iter().map(|a| (*a).to_string()));
    let held = parse(&["--pointer", "349,175", "--press"]);
    assert_eq!((held.pointer, held.press), (Some((349, 175)), true));
    assert_eq!(held.bad_pointer, None);
    for bad in ["349", "x,1", "1,-2", ""] {
        let cli = parse(&["--pointer", bad]);
        assert_eq!(cli.pointer, None, "{bad:?}");
        assert_eq!(cli.bad_pointer.as_deref(), Some(bad), "{bad:?}");
    }
    assert_eq!(parse(&["--pointer"]).bad_pointer.as_deref(), Some(""));
    assert_eq!(parse(&["--press"]).pointer, None);
    assert_eq!(parse(&["--press"]).bad_pointer, None);
}

/// `--pointer` holds the pointer over a control and `--press` holds the primary button down
/// there (`App::raw_input_hook`), for captures where nothing can drive the pointer: the Basic
/// page's first button is filled with the installed 0.8.0 theme's `button.body(HOVER).fill`,
/// and pressed with `button.body(PRESSED).fill`, each stated under kde-breeze light.
#[test]
fn a_held_pointer_hovers_and_presses_the_control_under_it() -> Result<(), String> {
    use native_theme::next::WidgetState;
    let args = [("--theme", TEST_PRESET), ("--tab", "basic")];
    let mut probe = open(egui::Theme::Light, cli(&args));
    install_under(&mut probe, egui::Theme::Light, "kde-breeze");
    let Some(rect) = probe
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "button (enabled)")
        .map(|r| r.rect)
    else {
        panic!("no button (enabled) record");
    };
    let r = probe
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .cloned()
        .ok_or("no 0.8.0 theme installed")?;
    let fill = |state| {
        crate::demo::drawn_state_fill(&r.button.body(state))
            .ok_or_else(|| format!("kde-breeze states no button fill for {state:?}"))
    };
    let (hovered, pressed) = (fill(WidgetState::HOVER)?, fill(WidgetState::PRESSED)?);
    for (press, fill) in [(false, Some(hovered)), (true, Some(pressed))] {
        let mut args = cli(&args);
        args.pointer = Some((rect.center().x as u16, rect.center().y as u16));
        args.press = press;
        let mut harness = open(egui::Theme::Light, args);
        install_under(&mut harness, egui::Theme::Light, "kde-breeze");
        for step in 0..16u32 {
            let ctx = harness.ctx.clone();
            let mut input = std::mem::take(harness.input_mut());
            input.time = Some(f64::from(step) * 0.5);
            eframe::App::raw_input_hook(harness.state_mut(), &ctx, &mut input);
            *harness.input_mut() = input;
            harness.step();
        }
        let drawn = harness
            .output()
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(r) if r.rect == rect => Some(r.fill),
                _ => None,
            });
        assert_eq!(drawn, fill, "press {press}: the button's fill");
    }
    Ok(())
}

/// The Basic page's dialog default button is drawn in the state of its previous response
/// (`default_button`'s per-state looks): under kde-breeze light, hovered, its frame is
/// `hover_stroke` over its own `fill`; pressed, its frame is `hover_stroke` over the button
/// role's pressed fill (`button.body(PRESSED).fill`), never its own. Under a patched hover frame
/// (`button.body(HOVER).stroke`, 3 px magenta), `hover_stroke` is that frame, not
/// `focus_stroke`, and the hovered button is drawn in it. Under adwaita light, whose 0.8.0 theme
/// gives the default button no look of its own, it is the selected (primary) button, filled with
/// the primary button's fill at rest. The presets are installed by the test, since `--theme`
/// takes only the platform's presets.
#[test]
fn the_dialog_default_button_is_drawn_in_its_state() -> Result<(), String> {
    use native_theme::next::{
        ButtonQuery, ButtonRule, PartPatch, Stroke, StrokeAlign, ThemePatch, WidgetState,
    };
    const KIND: &str = "button (dialog default)";
    let args = [("--theme", TEST_PRESET), ("--tab", "basic")];
    let mut probe = open(egui::Theme::Light, cli(&args));
    install_under(&mut probe, egui::Theme::Light, "kde-breeze");
    let Some(rect) = probe
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == KIND)
        .map(|r| r.rect)
    else {
        panic!("no {KIND} record");
    };
    // The button's frame as egui painted it: the `Shape::Rect` at the button's place.
    let drawn = |harness: &Harness<'_, App>| {
        harness
            .output()
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(r) if r.rect == rect => Some((r.fill, r.stroke)),
                _ => None,
            })
    };
    // The pointer held over the button under kde-breeze, with `patch` over its 0.8.0 themes
    // where given (`OldAtlas::patched`), pressed or not.
    let held = |press: bool, patch: Option<&native_theme::next::ThemePatch>| {
        let mut args = cli(&args);
        args.pointer = Some((rect.center().x as u16, rect.center().y as u16));
        args.press = press;
        let mut harness = open(egui::Theme::Light, args);
        install_under(&mut harness, egui::Theme::Light, "kde-breeze");
        if let Some(patch) = patch {
            let patched = harness.state().atlas.patched(patch)?;
            let ctx = harness.ctx.clone();
            harness.state_mut().adopt_pair(&ctx, patched);
        }
        for step in 0..16u32 {
            let ctx = harness.ctx.clone();
            let mut input = std::mem::take(harness.input_mut());
            input.time = Some(f64::from(step) * 0.5);
            eframe::App::raw_input_hook(harness.state_mut(), &ctx, &mut input);
            *harness.input_mut() = input;
            harness.step();
        }
        Ok::<_, String>(harness)
    };
    let look = |harness: &Harness<'_, App>| {
        harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .and_then(|r| native_theme_egui::next::default_button(r).ok().flatten())
    };

    let hovered = held(false, None)?;
    let d = look(&hovered).ok_or("kde-breeze light gives the default button no look of its own")?;
    assert_ne!(
        d.stroke, d.hover_stroke,
        "the hovered frame is not told apart"
    );
    assert_eq!(
        drawn(&hovered),
        Some((d.fill, d.hover_stroke)),
        "hovered: the default button's fill and hover frame"
    );

    let pressed = held(true, None)?;
    let active = pressed
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .and_then(|r| crate::demo::drawn_state_fill(&r.button.body(WidgetState::PRESSED)))
        .ok_or("kde-breeze light states no button.body(PRESSED).fill")?;
    assert_ne!(active, d.fill, "the pressed fill is not told apart");
    assert_eq!(
        drawn(&pressed),
        Some((active, d.hover_stroke)),
        "pressed: the button role's pressed fill under the hover frame"
    );

    // A hover frame patched into the 0.8.0 theme: the hovered button is drawn in it, not in the
    // focus frame.
    let magenta = native_theme::next::Color::from_srgb8(255, 0, 255, 255);
    let patch = ThemePatch::new().button(ButtonRule::new().states(WidgetState::HOVER).body(
        PartPatch::new().stroke(Stroke::Uniform {
            width: 3.0,
            color: magenta,
            align: StrokeAlign::Inside,
        }),
    ));
    let patched = held(false, Some(&patch))?;
    let d = look(&patched).ok_or("the patched kde-breeze gives no default button")?;
    assert_eq!(
        d.hover_stroke,
        egui::Stroke::new(3.0, egui::Color32::from_rgb(255, 0, 255)),
        "the patched hover frame"
    );
    assert_ne!(
        d.hover_stroke, d.focus_stroke,
        "the hover frame is not told apart from the focus frame"
    );
    assert_eq!(
        drawn(&patched).map(|(_, stroke)| stroke),
        Some(d.hover_stroke),
        "hovered: the patched hover frame, not the focus frame"
    );

    let mut adwaita = open_page_under(Page::Basic, egui::Theme::Light, "adwaita");
    adwaita.run_steps(4);
    let rect = adwaita
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == KIND)
        .map(|r| r.rect)
        .ok_or_else(|| format!("no {KIND} record under adwaita"))?;
    let r = adwaita
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("adwaita: no 0.8.0 theme installed")?;
    assert_eq!(native_theme_egui::next::default_button(r), Ok(None));
    let primary = crate::demo::drawn_fill(&r.button.body(ButtonQuery::primary()))
        .ok_or("adwaita states no primary fill")?;
    assert_ne!(
        Some(primary),
        crate::demo::drawn_fill(&r.button.body(WidgetState::NONE)),
        "the primary fill is not told apart"
    );
    let fill = adwaita
        .output()
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            egui::Shape::Rect(r) if r.rect == rect => Some(r.fill),
            _ => None,
        });
    assert_eq!(
        fill,
        Some(primary),
        "adwaita: the selected (primary) button"
    );
    Ok(())
}

/// A held pointer is moved there once and then left still, so the Basic page's tooltip, which
/// egui shows only under a pointer at rest, opens under it: a move on every pass would keep it
/// shut.
#[test]
fn a_held_pointer_opens_the_tooltip_under_it() {
    let args = [("--theme", TEST_PRESET), ("--tab", "basic")];
    let mut probe = open(egui::Theme::Light, cli(&args));
    install_under(&mut probe, egui::Theme::Light, "kde-breeze");
    let Some(rect) = probe
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "tooltip button")
        .map(|r| r.rect)
    else {
        panic!("no tooltip button record");
    };
    let mut args = cli(&args);
    args.pointer = Some((rect.center().x as u16, rect.center().y as u16));
    let mut harness = open(egui::Theme::Light, args);
    install_under(&mut harness, egui::Theme::Light, "kde-breeze");
    assert!(
        harness.query_by_label("A tooltip").is_none(),
        "the tooltip is open before the pointer is held"
    );
    for step in 0..16u32 {
        let ctx = harness.ctx.clone();
        let mut input = std::mem::take(harness.input_mut());
        input.time = Some(f64::from(step) * 0.5);
        eframe::App::raw_input_hook(harness.state_mut(), &ctx, &mut input);
        *harness.input_mut() = input;
        harness.step();
    }
    assert!(
        harness.query_by_label("A tooltip").is_some(),
        "no tooltip under the held pointer"
    );
}

/// A captured window (`--capture`) is drawn at rest wherever the real pointer is: the window's
/// pointer moved over the tooltip button, in the input `raw_input_hook` is handed (the
/// harness's own `hover_at` queues its event past the hook), hovers nothing, opens no tooltip
/// and chooses no Widget Info, while the same pointer does all three in a window that is not
/// captured.
#[test]
fn a_capture_is_drawn_at_rest_under_the_pointer() {
    let args = [("--theme", TEST_PRESET), ("--tab", "basic")];
    for capturing in [false, true] {
        let mut args = cli(&args);
        args.capture = capturing;
        let mut harness = open(egui::Theme::Light, args);
        install_under(&mut harness, egui::Theme::Light, "kde-breeze");
        let Some(rect) = harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.info.kind == "tooltip button")
            .map(|r| r.rect)
        else {
            panic!("no tooltip button record");
        };
        for step in 0..16u32 {
            let ctx = harness.ctx.clone();
            let mut input = std::mem::take(harness.input_mut());
            input.time = Some(f64::from(step) * 0.5);
            if step == 0 {
                input.events.push(egui::Event::PointerMoved(rect.center()));
            }
            eframe::App::raw_input_hook(harness.state_mut(), &ctx, &mut input);
            *harness.input_mut() = input;
            harness.step();
        }
        let hovered = harness.ctx.pointer_hover_pos().is_some();
        let tooltip = harness.query_by_label("A tooltip").is_some();
        let shown = shown_kind(&harness);
        if capturing {
            assert!(
                !hovered && !tooltip && shown.is_none(),
                "captured: hovered {hovered}, tooltip {tooltip}, Widget Info {shown:?}"
            );
        } else {
            assert!(
                hovered && tooltip && shown == Some("tooltip button"),
                "not captured: hovered {hovered}, tooltip {tooltip}, Widget Info {shown:?}"
            );
        }
    }
}

/// Reduce motion switched off after the user switched it on is installed: the atlas's
/// `reduce_motion` was the user's request, so it goes back to the OS's reading. Where the OS
/// requests reduced motion itself, the atlas keeps it on and the test cannot tell the two apart.
#[test]
fn a_switch_off_of_the_users_reduce_motion_request_installs() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    let mut prefs = AccessibilityPreferences::default();
    prefs.reduce_motion = true;
    harness.state_mut().settings.prefs = Some(prefs);
    harness.state_mut().install(&ctx);
    harness
        .state_mut()
        .run_action(Action::OpenPreferences, &ctx);
    harness.run_steps(SETTLE);
    assert!(
        harness.state().atlas.accessibility().reduce_motion,
        "the user's request was not installed"
    );
    harness
        .get_by_role_and_label(Role::Switch, "Reduce motion")
        .click();
    harness.run_steps(SETTLE);
    assert_eq!(
        harness.state().atlas.accessibility().reduce_motion,
        native_theme::detect::accessibility_preferences().reduce_motion,
        "the switch off kept the user's earlier request in the atlas"
    );
}

/// Preferences' flags are switches, as the gpui showcase's Settings rows are (parity item 20):
/// the companion crate's, in `Role::Switch`, so a set flag is a toggled switch named by its
/// row's title, and a click flips it; each sits right of its title.
#[test]
fn a_set_preference_is_a_selected_switch() -> Result<(), String> {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    let mut prefs = AccessibilityPreferences::default();
    prefs.reduce_motion = true;
    harness.state_mut().settings.prefs = Some(prefs);
    harness.state_mut().install(&ctx);
    harness
        .state_mut()
        .run_action(Action::OpenPreferences, &ctx);
    harness.run_steps(SETTLE);
    for (kind, title, on) in [
        ("switch · reduce motion", "Reduce motion", true),
        ("switch · high contrast", "High contrast", false),
        ("switch · reduce transparency", "Reduce transparency", false),
    ] {
        let seams = harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.info.kind == kind)
            .map(|r| r.info.seams.clone());
        assert_eq!(
            seams,
            Some(vec![Seam::Role(
                native_theme_egui::Role::Switch,
                RoleVariant::Normal
            )]),
            "{kind}"
        );
        let switch = harness.get_by_role_and_label(Role::Switch, title);
        let toggled = if on {
            egui::accesskit::Toggled::True
        } else {
            egui::accesskit::Toggled::False
        };
        assert_eq!(switch.accesskit_node().toggled(), Some(toggled), "{title}");
    }
    harness
        .get_by_role_and_label(Role::Switch, "High contrast")
        .click();
    harness.run_steps(SETTLE);
    assert!(
        harness
            .state()
            .settings
            .prefs
            .as_ref()
            .is_some_and(|p| p.high_contrast),
        "the switch did not set the flag"
    );

    // The OS's request, as the installed atlas's effective preferences carry it, with none of
    // the user's: the desktop's settings, read with reduced motion, the context's source. The
    // switch shows the user's request, so it is off, and a change of another switch stores no
    // request for Reduce motion.
    let preset = native_theme::next::Preset::from_name(TEST_PRESET).ok_or("no 0.8.0 preset")?;
    let mut os = native_theme::next::ThemeContext::light();
    os.motion = native_theme::next::MotionPreference::Reduced;
    let system = Ok(native_theme::next::Theme::preset(preset).with_context(os));
    let atlas = preset_atlas_on(TEST_PRESET, &system).map_err(|e| e.to_string())?;
    assert!(atlas.asked().reduce_motion, "the OS's request is asked for");
    harness.state_mut().settings.prefs = None;
    harness.state_mut().adopt_pair(&ctx, atlas);
    harness.run_steps(SETTLE);
    assert_eq!(
        harness
            .get_by_role_and_label(Role::Switch, "Reduce motion")
            .accesskit_node()
            .toggled(),
        Some(egui::accesskit::Toggled::False),
        "an OS request shows as the user's"
    );
    assert!(
        harness
            .query_by_label(
                "reduce_motion: every style's animation time is zero and its scroll animation \
                 is off; off is no request: an OS request still stops the animations"
            )
            .is_some(),
        "the Reduce motion row does not say that off is no request"
    );
    harness
        .get_by_role_and_label(Role::Switch, "High contrast")
        .click();
    harness.run_steps(SETTLE);
    assert_eq!(
        harness
            .state()
            .settings
            .prefs
            .as_ref()
            .map(|p| (p.reduce_motion, p.high_contrast)),
        Some((false, true)),
        "the OS's request was stored as the user's"
    );
    Ok(())
}

/// The Icons page's `ui.image` is drawn at the toolbar icon size, not at the page's width,
/// which would push every icon row below the fold.
#[cfg(feature = "lucide-icons")]
#[test]
fn the_icons_page_image_is_icon_sized() -> Result<(), String> {
    let harness = open_page(Page::Icons, egui::Theme::Light);
    let r = harness
        .state()
        .atlas
        .next_resolved(harness.ctx.theme())
        .ok_or("no 0.8.0 theme installed")?;
    let size =
        native_theme_egui::next::icon_size(r, native_theme_egui::icons::IconContext::Toolbar)
            .map_err(|e| e.to_string())?;
    let rect = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "ui.image")
        .map(|r| r.rect)
        .ok_or("the preset's Lucide set has no ActionSave")?;
    assert!(
        rect.width() <= size + 1.0 && rect.height() <= size + 1.0,
        "ui.image is {rect:?}, the toolbar icon size {size}"
    );
    Ok(())
}

fn manifest_and_json(
    harness: &Harness<'_, App>,
) -> Result<(Manifest, crate::info::InfoTheme), String> {
    let manifest = Manifest::parse(MANIFEST)?;
    let json = crate::info::info_theme(&harness.state().atlas, &harness.ctx)?;
    Ok((manifest, json))
}

/// Whether `text` holds a clause label of the archived egui connector spec's §13 test table,
/// a `T`, digits and an opening parenthesis (`T18(a)`), the form of `mapping.toml`'s
/// `tested_by`.
fn names_a_clause(text: &str) -> bool {
    text.match_indices('T').any(|(at, _)| {
        let rest = text.get(at + 1..).unwrap_or_default();
        let digits = rest.chars().take_while(char::is_ascii_digit).count();
        digits > 0 && rest.get(digits..).is_some_and(|tail| tail.starts_with('('))
    })
}

/// The names of the `#[test]` functions in `source`, a Rust file's text.
fn test_functions(source: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut test = false;
    for line in source.lines().map(str::trim) {
        if line == "#[test]" {
            test = true;
        } else if let Some(rest) = line.strip_prefix("fn ").filter(|_| test) {
            names.insert(
                rest.split(['(', '<'])
                    .next()
                    .unwrap_or_default()
                    .to_string(),
            );
            test = false;
        }
    }
    names
}

/// The Theme Map says what checks a per-call route in words a reader can follow: no row's text
/// holds a §13 clause label such as `T18(a)`, which only the archived spec explains, and every
/// row that `mapping.toml` checks through `tested_by` names a test of the connector by its
/// function name, one of the `#[test]` functions of `src/install_tests.rs`; so does each
/// clause's text Widget Info's route line takes (`info::checked_by`).
#[test]
fn the_theme_map_names_the_tests_that_check_a_per_call_route() {
    let harness = open_page(Page::ThemeMap, egui::Theme::Light);
    let read = manifest_and_json(&harness);
    assert!(read.is_ok(), "{read:?}");
    let Ok((manifest, json)) = read else { return };
    let tests = test_functions(include_str!("../../src/install_tests.rs"));
    assert!(!tests.is_empty(), "no test function was read");
    let mut wrong = Vec::new();
    for row in &manifest.rows {
        let (cells, _) = crate::info::row_cells(row, (&json, TEST_PRESET));
        if let Some(cell) = cells.iter().find(|cell| names_a_clause(cell)) {
            wrong.push(format!("{}: {cell:?} names a clause", row.leaf));
        }
        if let Some(label) = &row.tested_by {
            let sinks = &cells[3];
            if !tests.iter().any(|name| sinks.contains(name.as_str())) {
                wrong.push(format!(
                    "{} ({label}): {sinks:?} names no connector test",
                    row.leaf
                ));
            }
            let line = crate::info::checked_by(label);
            if names_a_clause(&line) || !tests.iter().any(|name| line.contains(name.as_str())) {
                wrong.push(format!("{label}: Widget Info's route reads {line:?}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn leaving_every_target_keeps_what_is_shown() {
    let mut harness = open_page(Page::Buttons, egui::Theme::Light);
    let pos = centre_of(&harness, "button (enabled)");
    hover_and_settle(&mut harness, pos);
    let shown = shown_id(&harness).expect("shown");
    let hold = harness.state().hold_zone.expect("hold zone");
    hover_and_settle(&mut harness, hold.center());
    assert_eq!(
        shown_id(&harness),
        Some(shown),
        "moving into the inspector's content replaced the info"
    );
    harness.remove_cursor();
    harness.run_steps(3);
    assert_eq!(
        shown_id(&harness),
        Some(shown),
        "the pointer leaving the window replaced the info"
    );
}

#[test]
fn crossing_is_not_hovering() {
    let mut harness = Harness::builder()
        .with_context_setup(emoji_fallbacks)
        .with_theme(egui::Theme::Light)
        .with_size(crate::WINDOW_SIZE)
        .with_step_dt(0.05)
        .build_eframe(|cc| {
            App::new(cc, &cli(&[("--theme", TEST_PRESET), ("--tab", "buttons")]))
                .expect("the showcase starts under a bundled preset")
        });
    harness.run_steps(4);
    let before = shown_id(&harness);
    harness.hover_at(centre_of(&harness, "button (enabled)"));
    // Every pass while the choice waits out INFO_SETTLE asks for a repaint no later than
    // INFO_SETTLE, so the settle completes with no further input. Checked on every such pass,
    // not only the first: the first also carries egui's own repaint for the hover.
    let delay = |h: &Harness<'_, App>| {
        h.output()
            .viewport_output
            .get(&egui::ViewportId::ROOT)
            .map(|v| v.repaint_delay)
            .expect("root viewport")
    };
    for pass in 0..3 {
        // 150 ms in all: under INFO_SETTLE
        harness.step();
        assert!(
            delay(&harness) <= INFO_SETTLE,
            "pass {pass} of the settle asked for a repaint in {:?}",
            delay(&harness)
        );
    }
    assert_eq!(
        shown_id(&harness),
        before,
        "a target hovered for less than INFO_SETTLE replaced the info"
    );
    for pass in 3..7 {
        // 350 ms: past it, with no further input
        harness.step();
        if harness
            .state()
            .registry
            .shown()
            .is_some_and(|s| Some(s.id) != before)
        {
            break;
        }
        assert!(
            delay(&harness) <= INFO_SETTLE,
            "pass {pass} of the settle asked for a repaint in {:?}",
            delay(&harness)
        );
    }
    assert_eq!(shown_kind(&harness), Some("button (enabled)"));
}

#[test]
fn a_target_no_longer_drawn_never_wins() {
    let mut harness = open_page(Page::Buttons, egui::Theme::Light);
    let pos = centre_of(&harness, "button (enabled)");
    hover_and_settle(&mut harness, pos);
    let button = shown_id(&harness).expect("shown");
    // The pointer leaves first, so only the page switch can take the button's info away
    // (leaving every target keeps what is shown); the switch is the app's own action, whose
    // `run_action` calls `screen_changed`.
    harness.remove_cursor();
    harness.run_steps(1);
    assert_eq!(
        shown_id(&harness),
        Some(button),
        "leaving the window replaced the info"
    );
    let ctx = harness.ctx.clone();
    harness
        .state_mut()
        .run_action(Action::ShowPage(Page::Text), &ctx);
    harness.run_steps(3);
    let records: Vec<egui::Id> = harness
        .state()
        .registry
        .records()
        .iter()
        .map(|r| r.id)
        .collect();
    assert!(!records.contains(&button));
    assert_ne!(
        shown_id(&harness),
        Some(button),
        "the previous page's button is still shown"
    );

    let mut harness = open_page(Page::Overlays, egui::Theme::Light);
    harness.get_by_label("Open popup").click();
    harness.run_steps(SETTLE);
    let pos = centre_of(&harness, "popup item");
    hover_and_settle(&mut harness, pos);
    let item = shown_id(&harness).expect("shown");
    let pos = centre_of(&harness, "popup item");
    harness.get_by_label("Open popup").click();
    harness.run_steps(SETTLE);
    harness.hover_at(pos);
    harness.run_steps(3);
    assert_ne!(
        shown_id(&harness),
        Some(item),
        "a closed popup's item is still shown"
    );
}

/// §10.4's rule, restated clause by clause from the spec's wording over the parsed rows, not
/// through `info.rs`: a misreading of the spec in either spelling makes the two differ.
fn expected_leaves(manifest: &Manifest, seam: &Seam) -> BTreeSet<String> {
    let widget_of = |leaf: &str| leaf.split('.').next().unwrap_or_default().to_string();
    // "every row with a sink that carries neither `scope` nor `surface` — the base-owner
    // table — and every row under `defaults.`, `text_scale.` or `layout.` that carries no
    // sink, `unmappable` or checked through `tested_by`"
    let base: BTreeSet<String> = manifest
        .rows
        .iter()
        .filter(|r| {
            let owner = r
                .sinks
                .iter()
                .any(|s| s.scope.is_none() && s.surface.is_none());
            let lost_or_per_call = r.sinks.is_empty()
                && ["defaults", "text_scale", "layout"].contains(&widget_of(&r.leaf).as_str())
                && (r.verdict == Verdict::Unmappable || r.tested_by.is_some());
            owner || lost_or_per_call
        })
        .map(|r| r.leaf.clone())
        .collect();
    match seam {
        Seam::Base => base,
        // "every row whose leaf starts with `R.key()` and a dot, and every other row with a
        // sink carrying `scope = R.key()`"; "A `Role` seam lists the base style's rows too".
        Seam::Role(role, _) => {
            let key = role.key();
            let mut leaves = base;
            for row in &manifest.rows {
                if widget_of(&row.leaf) == key
                    || row.sinks.iter().any(|s| s.scope.as_deref() == Some(key))
                {
                    leaves.insert(row.leaf.clone());
                }
            }
            leaves
        }
        // "every row with a sink carrying `surface = S.key()`, and every row of each native
        // widget those rows' leaves belong to".
        Seam::Surface(surface) => {
            let key = surface.key();
            let on_surface: Vec<&crate::info::Row> = manifest
                .rows
                .iter()
                .filter(|r| r.sinks.iter().any(|s| s.surface.as_deref() == Some(key)))
                .collect();
            let widgets: BTreeSet<String> = on_surface.iter().map(|r| widget_of(&r.leaf)).collect();
            on_surface
                .iter()
                .map(|r| r.leaf.clone())
                .chain(
                    manifest
                        .rows
                        .iter()
                        .filter(|r| widgets.contains(&widget_of(&r.leaf)))
                        .map(|r| r.leaf.clone()),
                )
                .collect()
        }
    }
}

/// The value at a dotted leaf, through a JSON pointer rather than the info module's walk.
fn leaf_value(json: &serde_json::Value, leaf: &str) -> Option<serde_json::Value> {
    json.pointer(&format!("/{}", leaf.replace('.', "/")))
        .cloned()
}

#[test]
fn the_info_is_the_manifest() {
    let manifest = Manifest::parse(MANIFEST).expect("the manifest parses");
    let mut seams = vec![Seam::Base];
    for role in native_theme_egui::Role::all() {
        for variant in RoleVariant::all() {
            seams.push(Seam::Role(*role, *variant));
        }
    }
    seams.extend(Surface::all().iter().map(|s| Seam::Surface(*s)));
    for preset in native_theme::next::Preset::ALL {
        let key = preset.as_str();
        for context in [
            native_theme::next::ThemeContext::light(),
            native_theme::next::ThemeContext::dark(),
        ] {
            let resolved = native_theme::next::Theme::preset(*preset).resolve(&context);
            // The values the info must print, read here and not through the info module: a
            // global home's colour or name, through a JSON pointer into the serialised form.
            let expected = serde_json::to_value(&resolved).unwrap_or_default();
            let json = crate::info::InfoTheme::new(resolved);
            assert!(json.is_ok(), "{key}: {json:?}");
            let Ok(json) = json else { continue };
            for seam in &seams {
                let rows = manifest.rows_for(seam);
                let listed: BTreeSet<String> = rows.iter().map(|r| r.leaf.clone()).collect();
                assert_eq!(
                    listed,
                    expected_leaves(&manifest, seam),
                    "{key} {:?} {seam:?}",
                    context.scheme
                );
                assert!(!rows.is_empty(), "{seam:?} lists no row");
                for row in rows {
                    let printed = crate::info::manifest_rows(
                        row,
                        (&json, key),
                        &crate::info::as_stated,
                        std::slice::from_ref(seam),
                    );
                    let homes: Vec<&str> = printed.iter().map(|r| r.leaf.as_str()).collect();
                    assert_eq!(homes, row.homes_on(key), "{key} {}", row.leaf);
                    let lines = printed
                        .iter()
                        .flat_map(|r| r.lines())
                        .collect::<Vec<_>>()
                        .join("\n");
                    for shown in &printed {
                        assert!(
                            !shown.value.starts_with("error: "),
                            "{key} {}: {}",
                            row.leaf,
                            shown.value
                        );
                        // The first line, whole: a substring would let `#12` pass for `#1`.
                        let at = leaf_value(&expected, &shown.leaf);
                        if let Some(serde_json::Value::String(value)) = at {
                            assert_eq!(
                                shown.lines().first(),
                                Some(&format!("{} {value}", shown.leaf)),
                                "{key} {seam:?} {}",
                                row.leaf
                            );
                        }
                    }
                    // An unmappable row's route names why: its upstream line, after the name
                    // of what lacks the route and before any note after a `;`.
                    if row.verdict == Verdict::Unmappable {
                        let upstream = row.upstream.as_deref().unwrap_or_default();
                        let why = upstream
                            .split_once(": ")
                            .map_or(upstream, |(_, why)| why)
                            .split(';')
                            .next()
                            .unwrap_or_default();
                        assert!(
                            row.sub_tag.is_some() && !why.is_empty() && lines.contains(why),
                            "{}: {lines:?}",
                            row.leaf
                        );
                    }
                }
            }
        }
    }

    // After a preset switch the shown info prints the new preset's values.
    let other = native_theme::theme::Theme::list_presets_for_platform()
        .into_iter()
        .map(|i| i.key)
        .find(|k| *k != TEST_PRESET)
        .expect("a second preset");
    // Light held, so the install cannot move the pass to the other scheme's values.
    let mut harness = open(
        egui::Theme::Light,
        cli(&[
            ("--theme", TEST_PRESET),
            ("--tab", Page::Buttons.key()),
            ("--variant", "light"),
        ]),
    );
    harness.run_steps(4);
    let pos = centre_of(&harness, "button (enabled)");
    hover_and_settle(&mut harness, pos);
    harness.state_mut().settings.theme = ThemeChoice::Preset(other.to_string());
    let ctx = harness.ctx.clone();
    harness.state_mut().install(&ctx);
    harness.run_steps(3);
    let shown = harness.state().registry.shown().cloned().expect("shown");
    // What the inspector prints from: the app's JSON, which it keeps between installs.
    let app = harness.state();
    let json = app.json.get(&app.atlas, &harness.ctx).clone();
    assert!(json.is_ok(), "{json:?}");
    let Ok(json) = json else { return };
    let elements = crate::elements::showcase_elements().unwrap_or_default();
    let scheme = (&app.atlas, &harness.ctx);
    let view = crate::info::InfoView::of(&shown, &elements, &manifest, (&json, other), scheme);
    // Each row of a global home that is a colour or a name prints the new preset's value, read
    // through a JSON pointer, and at least one of them differs from the previous preset's.
    let scheme_of = |key: &str| {
        let context = match harness.ctx.theme() {
            egui::Theme::Light => native_theme::next::ThemeContext::light(),
            egui::Theme::Dark => native_theme::next::ThemeContext::dark(),
        };
        native_theme::next::Preset::ALL
            .iter()
            .find(|p| p.as_str() == key)
            .map(|p| native_theme::next::Theme::preset(*p).resolve(&context))
            .and_then(|r| serde_json::to_value(&r).ok())
            .unwrap_or_default()
    };
    let (now, before) = (scheme_of(other), scheme_of(TEST_PRESET));
    let mut changed = 0usize;
    let mut compared = 0usize;
    for row in &view.rows {
        // A family's row names what egui draws (`every_font_label_names_the_drawn_family`).
        if row.leaf.ends_with(".family") {
            continue;
        }
        if let Some(serde_json::Value::String(value)) = leaf_value(&now, &row.leaf) {
            compared += 1;
            assert_eq!(row.value, value, "{other}: {}", row.leaf);
            if leaf_value(&before, &row.leaf) != Some(serde_json::Value::String(value)) {
                changed += 1;
            }
        }
    }
    assert!(
        compared > 0 && changed > 0,
        "the info shows the previous preset's values: {compared} compared, {changed} changed"
    );

    // What the spec names outright, kept by hand: `panel_left` lists every `sidebar` row
    // (§10.4, "`sidebar` for `panel_left`"), a role lists the base style's rows (§3.4), and the
    // base style lists every sinkless `defaults.` row.
    let listed = |seam: &Seam| -> BTreeSet<String> {
        manifest
            .rows_for(seam)
            .iter()
            .map(|r| r.leaf.clone())
            .collect()
    };
    let panel_left = listed(&Seam::Surface(Surface::Panel(
        native_theme_egui::PanelSide::Left,
    )));
    let sidebar: Vec<&String> = manifest
        .rows
        .iter()
        .map(|r| &r.leaf)
        .filter(|l| l.starts_with("sidebar."))
        .collect();
    assert!(
        !sidebar.is_empty() && sidebar.iter().all(|l| panel_left.contains(*l)),
        "panel_left does not list every sidebar row"
    );
    let base = listed(&Seam::Base);
    let button = listed(&Seam::Role(
        native_theme_egui::Role::Button,
        RoleVariant::Normal,
    ));
    assert!(
        base.is_subset(&button),
        "a role does not list the base style's rows"
    );
    assert!(
        manifest
            .rows
            .iter()
            .filter(|r| r.sinks.is_empty() && r.leaf.starts_with("defaults."))
            .all(|r| base.contains(&r.leaf)),
        "the base style does not list every sinkless defaults row"
    );
}

/// Every file of the showcase, as it is written.
fn showcase_raw_sources() -> Vec<(String, String)> {
    fn walk(dir: &Path, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(dir).expect("the showcase directory") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let raw = std::fs::read_to_string(&path).expect("read");
                out.push((path.display().to_string(), raw));
            }
        }
    }
    let mut out = Vec::new();
    walk(
        Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/showcase-egui"
        )),
        &mut out,
    );
    out
}

/// Every file of the showcase, blanked as the gpui detector blanks its source.
fn showcase_sources() -> Vec<(String, String)> {
    showcase_raw_sources()
        .into_iter()
        .map(|(path, raw)| (path, blanked(&raw)))
        .collect()
}

/// The body of every string literal in `raw`, comments and char literals skipped: the walk
/// `blanked` makes, keeping what it blanks between the quotes.
fn string_literals(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(rest) = raw.get(at..).filter(|rest| !rest.is_empty()) {
        if let Some(after) = rest.strip_prefix("//") {
            at += 2 + after.find('\n').unwrap_or(after.len());
        } else if let Some(len) = char_literal_len(rest) {
            at += len;
        } else if let Some(hashes) = raw_string_hashes(rest) {
            let close = format!("\"{}", "#".repeat(hashes));
            let from = hashes + 2;
            match rest.get(from..).and_then(|t| t.find(&close)) {
                Some(ix) => {
                    out.push(rest[from..from + ix].to_string());
                    at += from + ix + close.len();
                }
                None => at = raw.len(),
            }
        } else if let Some(body) = rest.strip_prefix('"') {
            let len = end_of_string(body, "\"");
            out.push(body.get(..len.saturating_sub(1)).unwrap_or("").to_string());
            at += 1 + len;
        } else {
            at += char_len(rest);
        }
    }
    out
}

/// Parity decision 5: no rendered string holds a character the installed proportional fonts
/// lack, which egui would draw as a replacement square. The strings are the showcase's own
/// literals — every file but this one, whose literals are test data — and every field of
/// `mapping.toml` that Widget Info and the Theme Map print; the fonts are the chain the atlas
/// installed, as epaint resolves a character through it (`FontsView::has_glyph`,
/// `epaint/src/text/fonts.rs:852-854`).
#[test]
fn every_rendered_character_is_in_the_installed_fonts() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let mut texts: Vec<String> = showcase_raw_sources()
        .into_iter()
        .filter(|(path, _)| !path.ends_with("tests.rs"))
        .flat_map(|(_, raw)| string_literals(&raw))
        .collect();
    assert!(
        texts.iter().any(|t| t.contains("Hover any widget")),
        "the scan read no string of the showcase"
    );
    let manifest = Manifest::parse(MANIFEST).expect("the manifest parses");
    for row in &manifest.rows {
        texts.push(row.leaf.clone());
        texts.extend(
            [&row.tested_by, &row.sub_tag, &row.upstream]
                .into_iter()
                .flatten()
                .cloned(),
        );
        for sink in &row.sinks {
            texts.push(sink.path.clone());
            texts.extend(
                [&sink.scope, &sink.variant, &sink.surface, &sink.when]
                    .into_iter()
                    .flatten()
                    .cloned(),
            );
        }
        for (preset, why) in &row.exceptions {
            texts.extend([preset.clone(), why.clone()]);
        }
    }
    for (path, why) in &manifest.unwritten {
        texts.extend([path.clone(), why.clone()]);
    }
    let chars: BTreeSet<char> = texts
        .iter()
        .flat_map(|t| t.chars())
        .filter(|c| !c.is_control())
        .collect();
    // Each character has a glyph in a face of the Body family's chain, read from the faces'
    // own character maps. Not `Fonts::has_glyph`: it reports a character missing when the
    // face that has it also holds epaint's replacement glyph, `◻`
    // (`epaint/src/text/font.rs:719-722`), as DejaVu Sans does where it is the substitute.
    let font = egui::TextStyle::Body.resolve(&harness.ctx.global_style());
    let missing: Vec<char> = harness.ctx.fonts_mut(|fonts| {
        use skrifa::MetadataProvider as _;
        let definitions = fonts.definitions();
        let faces: Vec<skrifa::FontRef<'_>> = definitions
            .families
            .get(&font.family)
            .into_iter()
            .flatten()
            .filter_map(|key| definitions.font_data.get(key))
            .filter_map(|data| skrifa::FontRef::from_index(data.bytes(), data.index).ok())
            .collect();
        chars
            .iter()
            .copied()
            .filter(|c| !faces.iter().any(|face| face.charmap().map(*c).is_some()))
            .collect()
    });
    assert!(
        missing.is_empty(),
        "characters the installed proportional fonts lack: {missing:?}"
    );
}

/// `raw` with every comment, string literal and char literal blanked to spaces,
/// its length and line breaks kept (the gpui detector's `blanked`,
/// `connectors/native-theme-gpui/src/showcase.rs:1054`, ported line for line).
fn blanked(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut at = 0usize;
    while let Some(rest) = raw.get(at..).filter(|rest| !rest.is_empty()) {
        let len = if let Some(after) = rest.strip_prefix("//") {
            2 + after.find('\n').unwrap_or(after.len())
        } else if let Some(len) = char_literal_len(rest) {
            len
        } else if let Some(hashes) = raw_string_hashes(rest) {
            let close = format!("\"{}", "#".repeat(hashes));
            let from = hashes + 2;
            match rest.get(from..).and_then(|t| t.find(&close)) {
                Some(ix) => from + ix + close.len(),
                None => rest.len(),
            }
        } else if let Some(body) = rest.strip_prefix('"') {
            1 + end_of_string(body, "\"")
        } else {
            let len = char_len(rest);
            out.push_str(rest.get(..len).unwrap_or(""));
            at += len;
            continue;
        };
        for c in rest.get(..len).unwrap_or(rest).chars() {
            if c == '\n' {
                out.push('\n');
            } else {
                for _ in 0..c.len_utf8() {
                    out.push(' ');
                }
            }
        }
        at += len;
    }
    out
}

/// The gpui detector's helpers (`connectors/native-theme-gpui/src/showcase.rs:301-357`).
fn char_literal_len(from: &str) -> Option<usize> {
    let body = from.strip_prefix('\'')?;
    if let Some(escaped) = body.strip_prefix('\\') {
        let skip = char_len(escaped);
        let end = escaped[skip..].find('\'')?;
        return Some("'\\".len() + skip + end + "'".len());
    }
    let first = body.chars().next()?;
    body[first.len_utf8()..]
        .starts_with('\'')
        .then_some("'".len() + first.len_utf8() + "'".len())
}
fn raw_string_hashes(from: &str) -> Option<usize> {
    let after_r = from.strip_prefix('r')?;
    let hashes = after_r.len() - after_r.trim_start_matches('#').len();
    after_r[hashes..].starts_with('"').then_some(hashes)
}
fn end_of_string(body: &str, delim: &str) -> usize {
    let mut ix = 0;
    while ix < body.len() {
        let tail = &body[ix..];
        if tail.starts_with('\\') {
            ix += tail.chars().take(2).map(char::len_utf8).sum::<usize>();
        } else if tail.starts_with(delim) {
            return ix + delim.len();
        } else {
            ix += char_len(tail);
        }
    }
    ix
}
fn char_len(s: &str) -> usize {
    match s.chars().next() {
        Some(c) => c.len_utf8(),
        None => 0,
    }
}

/// §10.4's last rule, held lexically: the seams are applied only inside `demo.rs`'s helpers,
/// each of which takes a seam once and both applies and records it. The chrome builds its
/// elements through those helpers too, so no other file — `chrome.rs` included — calls a seam
/// and records it from a second spelling.
#[test]
fn every_seam_is_recorded() {
    const SEAMS: &[&str] = &[
        ".native_scope(",
        ".native_set_style(",
        ".native_frame(",
        "role_modifier(",
        ".surface_frame(",
    ];
    let mut in_demo = 0;
    for (path, source) in showcase_sources() {
        if path.ends_with("demo.rs") {
            in_demo += SEAMS
                .iter()
                .map(|s| source.matches(s).count())
                .sum::<usize>();
            continue;
        }
        for seam in SEAMS {
            if let Some(at) = source.find(seam) {
                let line = source
                    .get(..at)
                    .map(|s| s.matches('\n').count() + 1)
                    .unwrap_or_default();
                panic!(
                    "{path}:{line}: {seam} applied outside demo.rs's helpers, which apply and \
                     record each seam once"
                );
            }
        }
    }
    assert!(in_demo > 0, "the detector read no seam call in demo.rs");
}

/// `IconCache`'s condition, held lexically: the images it keeps name the connector's icon URIs,
/// which `ThemeAtlas::install` and `ThemeAtlas::clear` forget (`forget_icons`), so outside the
/// tests an atlas is installed, cleared or its icons forgotten only in `App::adopt_pair`, which
/// every install goes through, and `App::adopt_pair` clears the cache beside it. A spelling of these calls the patterns miss (an
/// atlas bound to a name other than `atlas`) is left to review.
#[test]
fn only_app_adopt_installs_or_clears_an_atlas() {
    const CALLS: &[&str] = &[
        "atlas.install(",
        "ThemeAtlas::install(",
        "atlas.clear(",
        "ThemeAtlas::clear(",
        "forget_icons(",
    ];
    let mut in_adopt = 0;
    let mut adopt_clears_the_cache = false;
    for (path, source) in showcase_sources() {
        if path.ends_with("tests.rs") {
            continue;
        }
        // `App::adopt_pair`'s body, from its first brace to the one that closes it.
        let adopt = path
            .ends_with("app.rs")
            .then(|| source.find("fn adopt_pair("))
            .flatten()
            .and_then(|start| {
                let open = start + source.get(start..)?.find('{')?;
                let mut depth = 0usize;
                for (i, c) in source.get(open..)?.char_indices() {
                    match c {
                        '{' => depth += 1,
                        '}' => {
                            depth -= 1;
                            if depth == 0 {
                                return Some(open..open + i);
                            }
                        }
                        _ => {}
                    }
                }
                None
            });
        if let Some(body) = adopt.as_ref().and_then(|body| source.get(body.clone())) {
            adopt_clears_the_cache |= body.contains("self.icon_cache.clear()");
        }
        for call in CALLS {
            for (at, _) in source.match_indices(call) {
                if adopt.as_ref().is_some_and(|body| body.contains(&at)) {
                    in_adopt += 1;
                    continue;
                }
                let line = source
                    .get(..at)
                    .map(|s| s.matches('\n').count() + 1)
                    .unwrap_or_default();
                panic!(
                    "{path}:{line}: {call} outside App::adopt_pair, which clears the showcase's \
                     IconCache beside it"
                );
            }
        }
    }
    assert!(
        in_adopt > 0,
        "the detector read no install in App::adopt_pair"
    );
    assert!(
        adopt_clears_the_cache,
        "App::adopt_pair installs the atlas but does not clear the showcase's IconCache"
    );
}

/// The 0.7.2 model's reads and installs of a theme, which no showcase file reads through a
/// path other than the 0.8.0 model's ([`is_next_path`]), as the gpui showcase's
/// `OLD_MODEL_THEME_READS` (`connectors/native-theme-gpui/src/showcase.rs`): the desktop's
/// theme (`SystemTheme`, the connector's `from_system` and its `SystemThemeExt` installs,
/// `to_egui_atlas` and `to_egui_builder`), a preset loaded or resolved (`Theme::preset`,
/// `Theme::from_toml`, the connector's `from_preset`, `resolve_with`, `into_resolved`,
/// `ResolutionContext`, the resolved `native_theme::Resolved`, `ResolvedTheme` and its
/// `ResolvedFontSpec`), its mode (`ThemeMode`), the connector's install of it (`to_theme`),
/// and the pipeline's preset guess (`platform_preset_name`). Each is matched as a whole path,
/// wherever it stands: after `::`, at a crate root, or bare and imported by a `use`.
/// `ColorMode` is not one: `NextBuilder::os_mode` takes it. What reads no theme (the preset
/// list, `AccessibilityPreferences`) is not a read.
const OLD_MODEL_THEME_READS: &[&str] = &[
    "SystemTheme",
    "SystemThemeExt",
    "to_egui_atlas",
    "to_egui_builder",
    "Theme::preset",
    "Theme::from_toml",
    "from_preset",
    "from_system",
    "resolve_with",
    "into_resolved",
    "ResolutionContext",
    "native_theme::Resolved",
    "ResolvedTheme",
    "ResolvedFontSpec",
    "ThemeMode",
    "to_theme",
    "platform_preset_name",
];

/// The 0.7.2 model's reads that no path makes 0.8.0's, refused wherever they stand: the
/// installed atlas's 0.7.2 theme (`ThemeAtlas::resolved_for`, as `.resolved_for(`) and the
/// connector's 0.7.2 builder (`ThemeAtlas::builder(`).
const OLD_MODEL_LITERAL_READS: &[&str] = &[".resolved_for(", "ThemeAtlas::builder("];

/// Whether `c` can stand in an identifier.
fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Every offset at which `code` holds `pattern` as a whole: no identifier character before a
/// pattern that begins with one, none after a pattern that ends with one.
fn whole_matches(code: &str, pattern: &str) -> Vec<usize> {
    let starts_ident = pattern.chars().next().is_some_and(is_ident_char);
    let ends_ident = pattern.chars().next_back().is_some_and(is_ident_char);
    code.match_indices(pattern)
        .map(|(at, _)| at)
        .filter(|&at| {
            let before_ok = !starts_ident
                || code
                    .get(..at)
                    .and_then(|t| t.chars().next_back())
                    .is_none_or(|c| !is_ident_char(c));
            let after_ok = !ends_ident
                || code
                    .get(at + pattern.len()..)
                    .and_then(|t| t.chars().next())
                    .is_none_or(|c| !is_ident_char(c));
            before_ok && after_ok
        })
        .collect()
}

/// One name a `use` declaration brings in: the declaration's span, the item's full path and
/// the name it binds.
struct UseItem {
    span: std::ops::Range<usize>,
    path: Vec<String>,
    binds: String,
}

/// Every item of every `use` declaration in `code`, in source order.
fn use_items(code: &str) -> Vec<UseItem> {
    let mut out = Vec::new();
    for at in whole_matches(code, "use") {
        let head = code.get(..at).unwrap_or("").trim_end_matches([' ', '\t']);
        let head_ok = head.ends_with("pub")
            || head
                .chars()
                .next_back()
                .is_none_or(|c| matches!(c, '\n' | ';' | '{' | '}' | ')'));
        let tail = code.get(at + "use".len()..).unwrap_or("");
        if !head_ok || !tail.starts_with([' ', '\n']) {
            continue;
        }
        let Some(len) = tail.find(';') else { continue };
        let tree = tail
            .get(..len)
            .unwrap_or("")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let span = at..at + "use".len() + len;
        let mut items = Vec::new();
        use_tree_items(&tree, &[], &mut items);
        out.extend(items.into_iter().map(|(path, binds)| UseItem {
            span: span.clone(),
            path,
            binds,
        }));
    }
    out
}

/// The parts of `tree`, a `use` tree, split at its top-level commas and trimmed.
fn top_level_parts(tree: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut from = 0usize;
    for (ix, c) in tree.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(tree.get(from..ix).unwrap_or(""));
                from = ix + 1;
            }
            _ => {}
        }
    }
    parts.push(tree.get(from..).unwrap_or(""));
    parts
        .into_iter()
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .collect()
}

/// The segments of `path`, a written path such as `a :: b::c`.
fn path_segments(path: &str) -> impl Iterator<Item = String> + '_ {
    path.split("::")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// The (full path, bound name) of every item of `tree`, under `prefix`.
fn use_tree_items(tree: &str, prefix: &[String], out: &mut Vec<(Vec<String>, String)>) {
    for part in top_level_parts(tree) {
        let part = part.trim_start_matches("::");
        if let Some(open) = part.find('{') {
            let mut path = prefix.to_vec();
            path.extend(path_segments(part.get(..open).unwrap_or("")));
            let inner = part
                .get(open + 1..)
                .and_then(|t| t.trim_end().strip_suffix('}'))
                .unwrap_or("");
            use_tree_items(inner, &path, out);
            continue;
        }
        let (named, alias) = match part.split_once(" as ") {
            Some((named, alias)) => (named, Some(alias.trim())),
            None => (part, None),
        };
        let mut path = prefix.to_vec();
        path.extend(path_segments(named));
        if path.last().is_some_and(|leaf| leaf == "self") {
            path.pop();
        }
        let binds = alias
            .map(str::to_string)
            .or_else(|| path.last().cloned())
            .unwrap_or_default();
        out.push((path, binds));
    }
}

/// Whether `path` is the 0.8.0 model's -- it passes through a `next` module -- or egui's own,
/// whose `SystemTheme` the 0.7.2 model's name shares.
fn is_next_path<S: AsRef<str>>(path: &[S]) -> bool {
    path.iter().any(|s| s.as_ref() == "next") || path.first().is_some_and(|s| s.as_ref() == "egui")
}

/// The identifier `s` ends with.
fn trailing_ident(s: &str) -> &str {
    let start = s
        .char_indices()
        .rfind(|(_, c)| !is_ident_char(*c))
        .map(|(ix, c)| ix + c.len_utf8())
        .unwrap_or_default();
    s.get(start..).unwrap_or("")
}

/// The path written right before offset `at` of `code`, as segments: the `a::b::` that
/// qualifies whatever begins at `at`.
fn written_qualifier(code: &str, at: usize) -> Vec<String> {
    let mut head = code.get(..at).unwrap_or("");
    let mut segments = Vec::new();
    while let Some(rest) = head.strip_suffix("::") {
        let segment = trailing_ident(rest);
        if segment.is_empty() {
            break;
        }
        segments.push(segment.to_string());
        head = rest
            .get(..rest.len().saturating_sub(segment.len()))
            .unwrap_or("");
    }
    segments.reverse();
    segments
}

/// The full path of the name written at offset `at` of `code` as `written`: its written
/// qualifier, and its first segment resolved through the nearest `use` before it that binds
/// that name.
fn full_path(code: &str, uses: &[UseItem], at: usize, written: &str) -> Vec<String> {
    let mut path = written_qualifier(code, at);
    path.extend(written.split("::").map(str::to_string));
    let first = path.first().cloned().unwrap_or_default();
    match uses
        .iter()
        .rfind(|item| item.span.end <= at && item.binds == first)
    {
        Some(item) => {
            let mut full = item.path.clone();
            full.extend(path.into_iter().skip(1));
            full
        }
        None => path,
    }
}

/// The argument list of the call whose `(` is at offset `open` of `code`, up to the `)` that
/// closes it, trimmed; `None` where none closes it.
fn call_arguments(code: &str, open: usize) -> Option<&str> {
    let mut depth = 0usize;
    for (ix, c) in code.get(open..)?.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return code.get(open + 1..open + ix).map(str::trim);
                }
            }
            _ => {}
        }
    }
    None
}

/// What `code`, a showcase file's code, reads of the 0.7.2 model, as (offset, what it reads):
/// a name of [`OLD_MODEL_THEME_READS`] through a path that is not 0.8.0's, in code or imported
/// by a `use` (with the first segment of a longer name, as `use native_theme::theme::Theme;`
/// for `Theme::preset`); a read of [`OLD_MODEL_LITERAL_READS`]; and a `.resolve(` that is
/// 0.7.2's. 0.8.0's `Theme::resolve` takes a `&ThemeContext` (`native-theme/src/next/preset.rs`),
/// egui's `TextStyle::resolve` and `FontSelection::resolve` a `&Style` (a `&` expression, a
/// `.style()` read or a binding named `style`), and 0.7.2's a `ColorMode`
/// (`ThemeSpec::resolve`, `native-theme/src/model/mod.rs`) or nothing (`ThemeVariant::resolve`,
/// `native-theme/src/resolve/mod.rs`): an argument that is none of the first two is 0.7.2's.
fn old_model_reads(code: &str) -> Vec<(usize, String)> {
    let uses = use_items(code);
    let in_use = |at: usize| uses.iter().any(|item| item.span.contains(&at));
    let mut findings = Vec::new();
    for name in OLD_MODEL_THEME_READS {
        for at in whole_matches(code, name) {
            if !in_use(at) && !is_next_path(&full_path(code, &uses, at, name)) {
                findings.push((at, format!("reads or installs a 0.7.2 theme, {name}")));
            }
        }
    }
    for item in &uses {
        if is_next_path(&item.path) {
            continue;
        }
        let joined = item.path.join("::");
        let leaf = item.path.last().map_or("", String::as_str);
        let imports_a_read = OLD_MODEL_THEME_READS.iter().any(|name| {
            let segments: Vec<&str> = name.split("::").collect();
            let ends_with_it = item.path.len() >= segments.len()
                && item
                    .path
                    .iter()
                    .rev()
                    .zip(segments.iter().rev())
                    .all(|(have, want)| have == want);
            let its_type = segments.len() > 1
                && segments
                    .first()
                    .is_some_and(|first| *first == leaf && first.starts_with(char::is_uppercase));
            ends_with_it || its_type
        });
        if imports_a_read {
            findings.push((item.span.start, format!("imports a 0.7.2 name, {joined}")));
        }
    }
    for pattern in OLD_MODEL_LITERAL_READS {
        for at in whole_matches(code, pattern) {
            findings.push((at, format!("reads the installed 0.7.2 theme, {pattern}")));
        }
    }
    for (at, m) in code.match_indices(".resolve(") {
        let open = at + m.len() - "(".len();
        let argument = call_arguments(code, open).unwrap_or("");
        let style_read = argument == "style" || argument.ends_with(".style()");
        if !argument.starts_with('&') && !style_read {
            findings.push((
                at,
                "resolves a 0.7.2 theme, .resolve( without a &ThemeContext or egui's &Style"
                    .to_string(),
            ));
        }
    }
    findings.sort();
    findings
}

/// The egui showcase reads no 0.7.2 theme, as the gpui showcase's
/// `the_showcase_reads_no_0_7_2_theme`: no file of it, the tests included, names a read or
/// install of [`OLD_MODEL_THEME_READS`] or [`OLD_MODEL_LITERAL_READS`] or resolves a 0.7.2
/// theme ([`old_model_reads`]). Comments and strings are not read: a note may name what the
/// 0.7.2 model did.
#[test]
fn the_showcase_reads_no_0_7_2_theme() {
    let mut findings = Vec::new();
    let mut files = 0usize;
    for (path, code) in showcase_sources() {
        files += 1;
        for (at, what) in old_model_reads(&code) {
            let line = code
                .get(..at)
                .map(|s| s.matches('\n').count() + 1)
                .unwrap_or_default();
            findings.push(format!("{path}:{line}: {what}"));
        }
    }
    assert!(files > 1, "the detector read no showcase file");
    assert!(
        findings.is_empty(),
        "the egui showcase still reads the 0.7.2 theme:\n  {}",
        findings.join("\n  ")
    );
}

/// [`old_model_reads`] refuses each 0.7.2 read, however it is written, and takes its 0.8.0 or
/// egui namesake: one refused and one taken snippet per pattern.
#[test]
fn old_model_reads_refuses_each_pattern() {
    let refused = [
        "let t = native_theme::Theme::preset(\"x\")?;",
        "use native_theme::theme::Theme;\nfn f() { let t = Theme::preset(\"x\"); }",
        "use native_theme::{AccessibilityPreferences, theme::Theme};",
        "use native_theme::theme::Theme as Old;",
        "let t = native_theme::theme::Theme::from_toml(s);",
        "let t = native_theme_egui::from_preset(p, None);",
        "let l = native_theme_egui::from_system();",
        "let s = native_theme::SystemTheme::from_system();",
        "use native_theme_egui::SystemThemeExt as _;",
        "let a = system.to_egui_atlas();",
        "let b = system.to_egui_builder();",
        "let r = v.resolve_with(m, &c);",
        "let r = v.into_resolved(c);",
        "let c = native_theme::ResolutionContext::default();",
        "fn f(r: &native_theme::Resolved) {}",
        "fn f(r: &native_theme::ResolvedTheme) {}",
        "fn f(r: &native_theme_egui::ResolvedTheme) {}",
        "use native_theme_egui::{ColorMode, ResolvedTheme};",
        "fn f(r: &ResolvedTheme) {}",
        "fn f(f: &native_theme::theme::ResolvedFontSpec) {}",
        "let m = native_theme::ThemeMode::default();",
        "let a = native_theme_egui::to_theme(&r, \"x\");",
        "let p = native_theme::platform_preset_name();",
        "let r = atlas.resolved_for(egui::Theme::Dark);",
        "let b = ThemeAtlas::builder(\"x\", &light, &dark);",
        "let b = native_theme_egui::ThemeAtlas::builder(\"x\", &light, &dark);",
        "let r = theme.resolve(native_theme::ColorMode::Dark);",
        "let r = theme.resolve(mode);",
        "variant.resolve();",
    ];
    for code in refused {
        assert!(
            !old_model_reads(code).is_empty(),
            "takes a 0.7.2 read: {code}"
        );
    }
    let taken = [
        "let t = native_theme::next::Theme::preset(p);",
        "use native_theme::next::{Preset, Theme};\nfn f() { let t = Theme::preset(p); }",
        "let t = NextTheme::preset(p).resolve(&ctx);",
        "let t = native_theme::next::Theme::from_system();",
        "use native_theme::next::{ResolvedTheme as NextTheme, WidgetState};",
        "fn f(r: &native_theme::next::ResolvedTheme) {}",
        "use native_theme_egui::{ColorMode, ThemeAtlas};",
        "let b = ThemeAtlas::next_builder(\"x\", &light, &dark);",
        "let r = atlas.next_resolved_for(egui::Theme::Dark);",
        "fn f(t: egui::SystemTheme) {}",
        "let p = native_theme_egui::next::font_plan_from_system(light);",
        "let r = system\n    .resolve(\n        &ctx);",
        "let f = egui::TextStyle::Body.resolve(ui.style()).size;",
        "let f = egui::TextStyle::Monospace.resolve(style);",
        "let f = egui::TextStyle::Body.resolve(&harness.ctx.global_style());",
        "let presets = native_theme::theme::Theme::list_presets();",
        "let p = native_theme::AccessibilityPreferences::default();",
    ];
    for code in taken {
        assert_eq!(
            old_model_reads(code),
            Vec::new(),
            "refuses a 0.8.0 or egui read: {code}"
        );
    }
}

#[test]
fn every_role_and_surface_is_demonstrated() {
    let manifest = Manifest::parse(MANIFEST).expect("the manifest parses");
    let mut seen: BTreeSet<(String, String)> = BTreeSet::new();
    let mut surfaces: BTreeSet<String> = BTreeSet::new();
    // Records live one pass, so they are collected after every step that shows something: an
    // open popup, window or menu, a shown tooltip, then the modal last, since a click behind a
    // modal lands on its backdrop and closes it (§10.4).
    let mut collect = |harness: &Harness<'_, App>| {
        for record in harness.state().registry.records() {
            for seam in &record.info.seams {
                match seam {
                    Seam::Role(role, variant) => {
                        seen.insert((role.key().to_string(), variant.key().to_string()));
                    }
                    Seam::Surface(surface) => {
                        surfaces.insert(surface.key().to_string());
                    }
                    Seam::Base => {}
                }
            }
        }
    };
    for page in Page::ALL {
        let mut harness = open_page(page, egui::Theme::Light);
        collect(&harness);
        // The tooltip first: egui shows no tooltip while a popup or menu is open.
        if harness.query_all_by_label("Tooltip").next().is_some() {
            let pos = harness.get_by_label("Tooltip").rect().center();
            hover_and_settle(&mut harness, pos);
            collect(&harness);
        }
        for label in ["Open popup", "Open window", "Page menu"] {
            if harness.query_all_by_label(label).next().is_some() {
                harness.get_by_label(label).click();
                harness.run_steps(2);
                collect(&harness);
            }
        }
        if harness.query_all_by_label("Open modal").next().is_some() {
            harness.get_by_label("Open modal").click();
            harness.run_steps(2);
            collect(&harness);
        }
        harness.get_by_role_and_label(Role::Button, "View").click();
        harness.run_steps(2);
        collect(&harness);
    }
    // The variants a role carries: `Normal` always; every other variant where the manifest
    // declares a sink in that cell (§13.1's `variant`).
    let mut missing = Vec::new();
    for role in native_theme_egui::Role::all() {
        let key = role.key();
        let mut variants = vec!["normal"];
        for v in RoleVariant::all().iter().map(|v| v.key()) {
            if v != "normal"
                && manifest.rows.iter().any(|r| {
                    r.sinks
                        .iter()
                        .any(|s| s.scope.as_deref() == Some(key) && s.variant.as_deref() == Some(v))
                })
            {
                variants.push(v);
            }
        }
        for v in variants {
            if !seen.contains(&(key.to_string(), v.to_string())) {
                missing.push(format!("{key} {v}"));
            }
        }
    }
    for surface in Surface::all() {
        if !surfaces.contains(surface.key()) {
            missing.push(surface.key().to_string());
        }
    }
    assert!(missing.is_empty(), "never recorded: {missing:?}");
}

#[test]
fn the_chrome_reports_itself() -> Result<(), String> {
    // Under every bundled preset, installed on any platform.
    let mut ownless = Vec::new();
    for preset in native_theme::next::Preset::ALL.iter().map(|p| p.as_str()) {
        ownless.extend(
            chrome_reports_itself_under(preset)?
                .into_iter()
                .map(|kind| (preset, kind)),
        );
    }
    assert_eq!(
        ownless, CHROME_WITHOUT_A_POINT,
        "the chrome parts with no point of their own"
    );
    Ok(())
}

/// The chrome parts egui leaves no point of their own, by preset: macos-tahoe's page tab row
/// (measured at 1280 × 720: the strip 304-1280 × 63-87, no margin, `layout.container_margin`
/// unstated; its twelve tabs as tall as the strip and `item_spacing.x` 8 apart, the Pages
/// button 1241-1280, 63.5-86.5), where every gap, narrower than twice egui's interact radius
/// of 5, is within the radius of a tab or the button, which egui so counts under the pointer.
const CHROME_WITHOUT_A_POINT: &[(&str, &str)] = &[("macos-tahoe", "TabBar · Pages")];

/// Each chrome part under `preset` light has a point of its own, and hovered there Widget Info
/// shows it: a point inside the part, inside no smaller record and outside the interact radius
/// of every smaller one that senses a click or a drag. egui counts a widget under the pointer
/// where its interact rect contains the pointer (`hit_test`'s `contains_pointer`,
/// `interact_rect.contains(pos)`, `egui-0.37.0/src/hit_test.rs`), and also the one widget a
/// click or a drag would go to, the closest within the interact radius (`hit_test_on_close`;
/// `interact`, `egui-0.37.0/src/interaction.rs`, chains `hits.click` and `hits.drag` into
/// `contains_pointer`), so the radius keeps a point from a clickable record alone, not from a
/// container. The points are each point's centre of the part, fine enough for the few points a
/// margin leaves. The parts with no such point come back.
fn chrome_reports_itself_under(preset: &str) -> Result<Vec<&'static str>, String> {
    let mut harness = open_page_under(Page::Buttons, egui::Theme::Light, preset);
    let radius = harness.ctx.global_style().interaction.interact_radius;
    // The inspector's content is the hold zone, where nothing is chosen (§10.4).
    let hold = harness
        .state()
        .hold_zone
        .ok_or("the inspector reports no content rect")?;
    let mut ownless = Vec::new();
    for kind in [
        "Menu bar",
        "Toolbar",
        "Side panel",
        "Status bar",
        "TabBar · Pages",
        "TabBar · Inspector",
    ] {
        let records: Vec<(egui::Id, egui::Rect)> = harness
            .state()
            .registry
            .records()
            .iter()
            .map(|r| (r.id, r.rect))
            .collect();
        let (id, rect) = harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.info.kind == kind)
            .map(|r| (r.id, r.rect))
            .ok_or_else(|| format!("{preset}: no {kind} record"))?;
        // A point inside the target and inside no smaller record: the element's own surface.
        // The records around it (the chrome bar around the menu bar and the toolbar, the side
        // panel around the inspector's tabs, the central panel around the page tabs) contain it
        // too, and lose to it by area (§10.4, innermost hovered wins).
        let smaller: Vec<egui::Rect> = records
            .iter()
            .filter(|(other, r)| *other != id && r.area() <= rect.area())
            .map(|(other, r)| {
                let sense = harness.ctx.read_response(*other).map(|r| r.sense);
                if sense.is_some_and(|s| s.senses_click() || s.senses_drag()) {
                    r.expand(radius)
                } else {
                    *r
                }
            })
            .filter(|r| r.intersects(rect))
            .collect();
        let along = |from: f32, to: f32| {
            (0_u16..)
                .map(move |i| from + f32::from(i) + 0.5)
                .take_while(move |v| *v < to)
        };
        let Some(point) = along(rect.left(), rect.right())
            .flat_map(|x| along(rect.top(), rect.bottom()).map(move |y| egui::pos2(x, y)))
            .find(|p| !hold.contains(*p) && !smaller.iter().any(|r| r.contains(*p)))
        else {
            ownless.push(kind);
            continue;
        };
        hover_and_settle(&mut harness, point);
        assert_eq!(
            shown_kind(&harness),
            Some(kind),
            "{preset}: hovered at {point:?}"
        );
    }
    let pos = centre_of(&harness, "button (enabled)");
    hover_and_settle(&mut harness, pos);
    let title = harness.state().status_title();
    assert_eq!(title, "button (enabled)", "{preset}");
    assert!(
        harness.query_all_by_label(&title).next().is_some(),
        "{preset}: the status bar does not show the title"
    );
    Ok(ownless)
}

/// The exact call list of §13.2; a numeric literal among a call's arguments is the finding.
const SIZE_CALLS: &[&str] = &[
    "add_space",
    "set_min_size",
    "set_max_size",
    "set_width",
    "set_min_width",
    "set_max_width",
    "set_height",
    "set_min_height",
    "set_max_height",
    "set_width_range",
    "set_height_range",
    "default_size",
    "default_width",
    "default_height",
    "min_size",
    "max_size",
    "exact_size",
    "fixed_size",
    "min_width",
    "max_width",
    "min_height",
    "max_height",
    "desired_width",
    "desired_height",
    "fit_to_exact_size",
    "inner_margin",
    "outer_margin",
    "corner_radius",
    "stroke",
    "size",
    "spacing",
];
const CONSTRUCTORS: &[&str] = &[
    "vec2(",
    "Vec2::new(",
    "Vec2::splat(",
    "Margin::",
    "CornerRadius::",
    "Stroke::new(",
    "Color32::from_",
    "FontId::new(",
    "FontId::proportional(",
    "FontId::monospace(",
];
/// The sites the rule does not reach, by the enclosing `fn`, with the reason
/// (the gpui showcase's `ALLOWED_STYLE_LITERALS`,
/// `connectors/native-theme-gpui/src/showcase.rs:4628`). `tests.rs`, which holds the detector's
/// own sample source, is not scanned, so it needs no entry.
const ALLOWED_STYLE_LITERALS: &[(&str, &str)] = &[
    (
        "table_cell_inset",
        "the 0.5 halves the scope's own item_spacing, egui_extras' cell inset \
         (egui_extras-0.37.0 src/layout.rs, StripLayout::set_pos), no style value",
    ),
    (
        "demo_colour",
        "the colour a Colour-page editor starts from is the datum on display",
    ),
    (
        "icons_indicator",
        "the 0.5 of Vec2::splat(0.5) is the centre Image::rotate turns about (§4.10)",
    ),
    (
        "button_min_size",
        "the 0.0 is no least height: a Button raises its own to interact_size.y, \
         button.min_height in the button scope (egui 0.36.2 src/widgets/button.rs:306-309)",
    ),
    (
        "input_min_size",
        "the 0.0 is no least width: a TextEdit is its desired_width wide",
    ),
];
/// The three named constants of §10.4, and the gpui-component literals the parity decisions
/// allow (each cites the upstream line it mirrors), exempt as definitions.
const NAMED_CONSTANTS: &[&str] = &["LEFT_PANEL_WIDTH", "WINDOW_SIZE", "INFO_SETTLE"];

#[test]
fn the_showcase_hardcodes_no_style_values() {
    let sample = blanked("fn probe(ui: &mut egui::Ui) { ui.add_space(8.0); }");
    assert_eq!(
        style_literals(&sample),
        vec!["1: add_space(8.0), in fn probe".to_string()],
        "the detector cannot read its own sample"
    );
    let mut findings = Vec::new();
    let mut calls_seen = 0usize;
    // The tests' own pointer offsets are not style values: `tests.rs` is not scanned.
    for (path, source) in showcase_sources()
        .into_iter()
        .filter(|(path, _)| !path.ends_with("tests.rs"))
    {
        calls_seen += SIZE_CALLS
            .iter()
            .map(|c| source.matches(&format!(".{c}(")).count())
            .sum::<usize>();
        for finding in style_literals(&source) {
            findings.push(format!("{path}:{finding}"));
        }
    }
    assert!(calls_seen > 0, "the detector read no call it checks");
    assert!(
        findings.is_empty(),
        "style values hardcoded:\n{}",
        findings.join("\n")
    );
}

/// Every `SIZE_CALLS` call and `CONSTRUCTORS` use in `source` whose argument list
/// holds a numeric literal, as `line: what, in fn`, unless its enclosing `fn`
/// is allow-listed or it is one of the three named constants' definitions.
fn style_literals(source: &str) -> Vec<String> {
    let is_ident = |c: char| c.is_alphanumeric() || c == '_';
    let patterns: Vec<String> = SIZE_CALLS
        .iter()
        .map(|c| format!(".{c}("))
        .chain(CONSTRUCTORS.iter().map(|c| (*c).to_string()))
        .collect();
    let mut out = Vec::new();
    for pattern in &patterns {
        let mut from = 0;
        while let Some(rel) = source.get(from..).and_then(|s| s.find(pattern.as_str())) {
            let at = from + rel;
            from = at + pattern.len();
            // A constructor is a whole name: `Vec2::new(` inside `egui::Vec2::new(` counts,
            // inside `MyVec2::new(` it does not.
            if !pattern.starts_with('.')
                && source
                    .get(..at)
                    .and_then(|s| s.chars().next_back())
                    .is_some_and(is_ident)
            {
                continue;
            }
            // The argument list: from the first `(` at or after the name to its matching `)`.
            let Some(open) = source.get(at..).and_then(|s| s.find('(')).map(|ix| at + ix) else {
                continue;
            };
            let mut depth = 0usize;
            let mut close = None;
            for (ix, c) in source.get(open..).unwrap_or("").char_indices() {
                match c {
                    '(' => depth += 1,
                    ')' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            close = Some(open + ix);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            let Some(close) = close else { continue };
            let args = source.get(open + 1..close).unwrap_or("");
            // A numeric literal: a digit that starts a token — not inside an identifier, and not
            // a tuple field (`x.0`).
            let chars: Vec<char> = args.chars().collect();
            let literal = chars.iter().enumerate().any(|(ix, c)| {
                c.is_ascii_digit()
                    && match ix.checked_sub(1).and_then(|p| chars.get(p)) {
                        None => true,
                        Some(prev) if is_ident(*prev) => false,
                        Some('.') => !ix
                            .checked_sub(2)
                            .and_then(|p| chars.get(p))
                            .is_some_and(|pp| is_ident(*pp) || *pp == ')'),
                        Some(_) => true,
                    }
            });
            if !literal {
                continue;
            }
            // The enclosing item: the last `fn <name>` or `const <NAME>` before the call.
            let before = source.get(..at).unwrap_or("");
            let item = |keyword: &str| {
                before
                    .rmatch_indices(keyword)
                    .find(|(ix, _)| {
                        before
                            .get(..*ix)
                            .and_then(|s| s.chars().next_back())
                            .is_none_or(|c| !is_ident(c))
                    })
                    .map(|(ix, _)| {
                        let name: String = before
                            .get(ix + keyword.len()..)
                            .unwrap_or("")
                            .chars()
                            .take_while(|c| is_ident(*c))
                            .collect();
                        (ix, name)
                    })
            };
            let (kind, name) = match (item("fn "), item("const ")) {
                (Some((f, name)), Some((c, _))) if f > c => ("fn", name),
                (_, Some((_, name))) => ("const", name),
                (Some((_, name)), None) => ("fn", name),
                (None, None) => ("item", String::new()),
            };
            if kind == "fn"
                && ALLOWED_STYLE_LITERALS
                    .iter()
                    .any(|(allowed, _)| *allowed == name)
            {
                continue;
            }
            if kind == "const" && NAMED_CONSTANTS.contains(&name.as_str()) {
                continue;
            }
            let line = before.matches('\n').count() + 1;
            let start = at + usize::from(pattern.starts_with('.'));
            let what = source.get(start..=close).unwrap_or("");
            out.push(format!("{line}: {what}, in {kind} {name}"));
        }
    }
    out
}

/// A duration home reads as a length does, with its unit: `timings.tooltip_show_ms` shows the
/// 0.8.0 theme's tooltip delay with " ms", and its route is that of the `mapping.toml` row that
/// holds it, `tooltip.show_delay`'s.
#[test]
fn a_ms_home_reads_its_value() -> Result<(), String> {
    use native_theme::next::{Preset, Theme, ThemeContext};
    let r = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
    let delay = r.timings.tooltip_show_ms;
    assert!(delay.is_some(), "kde-breeze states a tooltip delay");
    let shown = crate::info::InfoTheme::new(r)?;
    let (text, _) = shown.value("timings.tooltip_show_ms", &crate::info::as_stated);
    assert_eq!(Some(text), delay.map(|ms| format!("{ms} ms")));
    let manifest = Manifest::parse(MANIFEST)?;
    let row = manifest
        .rows
        .iter()
        .find(|r| r.leaf == "tooltip.show_delay")
        .ok_or("mapping.toml has no tooltip.show_delay row")?;
    let how = crate::info::how_line(&manifest, "timings.tooltip_show_ms", &[Seam::Base]);
    assert!(
        row.sinks.iter().any(|s| how.contains(&s.path)),
        "the route {how:?} is not the tooltip.show_delay row's"
    );
    Ok(())
}

/// `docs/showcase-elements.toml`, as the showcase reads it.
fn listed_elements() -> Vec<crate::elements::ShowcaseElement> {
    let elements = crate::elements::showcase_elements();
    assert!(elements.is_ok(), "the element list parses: {elements:?}");
    elements.unwrap_or_default()
}

/// Whether this build carries an icon set: without `material-icons`, `lucide-icons` and
/// `system-icons` no icon loads, so the showcase draws no icon element (a tool button shows its
/// label instead).
const ICONS_BUILT: bool = cfg!(any(
    feature = "material-icons",
    feature = "lucide-icons",
    feature = "system-icons"
));

/// The list's elements that are an icon itself: a `.icon` part, or one of the Icons group's three
/// sizes.
fn is_icon_element(id: &str) -> bool {
    id.ends_with(".icon")
        || matches!(
            id,
            "basic.icons.small" | "basic.icons.toolbar" | "basic.icons.large"
        )
}

/// R11 §C: the layout dump holds exactly the list's elements the showcase draws. On the Basic
/// page at rest every element of the list is drawn but those shown only under a condition
/// (`when`: an open menu, a hover, a tooltip), and the showcase places no id the list lacks;
/// each parent the list measures an element against is placed where the element is. Under
/// adwaita, whose page has the five columns the list names (`basic.column_5`).
#[test]
fn the_layout_dump_names_every_listed_element_the_basic_page_draws() {
    let elements = listed_elements();
    let ids: BTreeSet<&str> = elements.iter().map(|e| e.id.as_str()).collect();
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let harness = open_page_under(Page::Basic, theme, "adwaita");
        let places = harness.state().registry.places();
        let unknown: Vec<&String> = places
            .keys()
            .filter(|id| !ids.contains(id.as_str()))
            .collect();
        assert!(unknown.is_empty(), "placed but not listed: {unknown:?}");
        let missing: Vec<&str> = elements
            .iter()
            .filter(|e| e.when.is_none() && !places.contains_key(&e.id))
            .filter(|e| ICONS_BUILT || !is_icon_element(&e.id))
            .map(|e| e.id.as_str())
            .collect();
        assert!(missing.is_empty(), "listed but not placed: {missing:?}");
        for element in &elements {
            let placed = places.get(&element.id);
            let parent = places.get(&element.parent);
            assert!(
                placed.is_none()
                    || element.parent == "window"
                    || parent.is_some()
                    || (!ICONS_BUILT && is_icon_element(&element.parent)),
                "{} is placed but its parent {} is not",
                element.id,
                element.parent
            );
        }
    }
}

/// R12: `--open-menu theme` opens the Theme menu before the first settled frame, so the dump
/// holds the list's Theme-menu elements (`when = "the Theme menu is open ..."`), its popup and
/// every row, each below the one before; and a run without the flag holds none of them.
#[test]
fn open_menu_theme_places_the_theme_menus_rows() {
    let elements = listed_elements();
    let menu: Vec<&str> = elements
        .iter()
        .filter(|e| e.when.as_deref().is_some_and(|w| w.contains("Theme menu")))
        .map(|e| e.id.as_str())
        .collect();
    assert!(!menu.is_empty(), "the list names the Theme menu's elements");
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let mut shown = open(theme, cli(&[("--tab", "basic"), ("--open-menu", "theme")]));
        shown.run_steps(4);
        let places = shown.state().registry.places();
        let missing: Vec<&&str> = menu
            .iter()
            .filter(|id| !places.contains_key(**id))
            .collect();
        assert!(
            missing.is_empty(),
            "{theme:?}: Theme menu elements not placed: {missing:?}"
        );
        let rows: Vec<egui::Rect> = menu
            .iter()
            .filter(|id| id.starts_with("chrome.menu.theme.") && !id.ends_with(".shortcut"))
            .filter_map(|id| places.get(*id).copied())
            .collect();
        for pair in rows.windows(2) {
            assert!(
                pair[1].top() >= pair[0].bottom() - 0.5,
                "{theme:?}: the Theme menu's rows are not one under another: {rows:?}"
            );
        }
        let mut closed = open(theme, cli(&[("--tab", "basic")]));
        closed.run_steps(4);
        let places = closed.state().registry.places();
        assert!(
            !menu.iter().any(|id| places.contains_key(*id)),
            "{theme:?}: a run without --open-menu places the Theme menu"
        );
    }
}

/// R11 §B: the hovered Basic-page button shows the list's name and state as its title and one
/// row per leaf of the list, in its order, each with a route; parts and the layout boxes are
/// never Widget Info's targets.
#[test]
fn widget_info_shows_the_listed_elements_rows() -> Result<(), String> {
    let elements = listed_elements();
    let mut harness = open_page(Page::Basic, egui::Theme::Light);
    let pos = centre_of(&harness, "button (enabled)");
    hover_and_settle(&mut harness, pos);
    let shown = harness.state().registry.shown().cloned();
    assert_eq!(
        shown.as_ref().and_then(|s| s.info.element.as_deref()),
        Some("basic.buttons.default")
    );
    let listed = elements.iter().find(|e| e.id == "basic.buttons.default");
    let (manifest, json) = manifest_and_json(&harness)?;
    if let (Some(shown), Some(listed)) = (shown, listed) {
        let scheme = (&harness.state().atlas, &harness.ctx);
        let view =
            crate::info::InfoView::of(&shown, &elements, &manifest, (&json, TEST_PRESET), scheme);
        assert_eq!(view.title, "Button · Normal");
        let leaves: Vec<&str> = view.rows.iter().map(|r| r.leaf.as_str()).collect();
        let want: Vec<&str> = listed.homes.iter().map(String::as_str).collect();
        assert_eq!(leaves, want);
        assert!(
            view.rows
                .iter()
                .all(|r| r.how.starts_with("egui: ") || r.how.starts_with("not reachable: ")),
            "{:#?}",
            view.rows
        );
        let first = want
            .first()
            .copied()
            .ok_or("basic.buttons.default: its list row names no home")?;
        assert!(
            harness.query_all_by_label_contains(first).next().is_some(),
            "the Widget tab does not show the button's first row"
        );
    }
    const LAYOUT_BOXES: [&str; 7] = [
        "chrome.window",
        "chrome.content",
        "basic.page",
        "basic.column_1",
        "basic.column_2",
        "basic.column_3",
        "basic.column_4",
    ];
    for record in harness.state().registry.records() {
        let Some(id) = record.info.element.as_deref() else {
            continue;
        };
        let part = elements.iter().any(|e| e.id == id && e.part);
        assert!(!part, "the part {id} is a Widget Info target");
        assert!(
            !LAYOUT_BOXES.contains(&id) || !record.target,
            "the layout box {id} is a Widget Info target"
        );
    }
    Ok(())
}

/// The Basic tabs, the page tabs and the inspector's tabs take their least width, the selected
/// tab's line and its outward corners per call from the installed 0.8.0 theme
/// (`demo::TabLook::next`), so those rows' routes name the showcase's call, not the 0.7.2
/// connector helpers' tests `mapping.toml` names (`tested_by = "T18(a)"`); an element that is
/// no tab keeps the manifest's route.
#[test]
fn the_tab_routes_name_the_call_that_draws_them() -> Result<(), String> {
    use crate::info::{TAB_ELEMENTS, TAB_PER_CALL_ROUTES};
    let manifest = Manifest::parse(MANIFEST)?;
    let seams = [crate::demo::Seam::Role(
        native_theme_egui::Role::Tab,
        RoleVariant::Normal,
    )];
    let elements = listed_elements();
    let mut checked = 0usize;
    let mut wrong = Vec::new();
    for id in TAB_ELEMENTS {
        let element = elements
            .iter()
            .find(|e| e.id == id)
            .ok_or_else(|| format!("{id} is not listed"))?;
        for (home, _) in TAB_PER_CALL_ROUTES {
            if !element.homes.iter().any(|h| h == home) {
                continue;
            }
            checked += 1;
            let how = crate::info::listed_how(&manifest, id, home, &seams);
            if !how.contains("(demo::") || how.contains("install_tests") {
                wrong.push(format!("{id} {home}: {how:?}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    assert_eq!(
        checked, 24,
        "each bar's outward radius, its selected tab's six homes and the other tab's least width, \
         on three bars"
    );
    let home = "tab.active_outward_corner_radius";
    assert_eq!(
        crate::info::listed_how(&manifest, "basic.page", home, &seams),
        crate::info::how_line(&manifest, home, &seams),
        "the Basic page's layout box takes a tab's route"
    );
    Ok(())
}

/// The tool buttons' and the side-panel toggle's own homes, the primary and the dialog's
/// default button's own outline and the Copy box's hovered width, which no `mapping.toml` row
/// holds, are routed to the call that applies them, not shown as held by no row: each such home
/// a listed tool button, its icon, the toggle, its icon, the two buttons and the Copy box list
/// names the call that reads it ([`crate::info::TOOL_BUTTON_ROUTES`],
/// [`crate::info::TOGGLE_ROUTES`], the per-call routes).
#[test]
fn the_model_only_button_homes_name_the_call_that_draws_them() -> Result<(), String> {
    let manifest = Manifest::parse(MANIFEST)?;
    let seams = [crate::demo::Seam::Role(
        native_theme_egui::Role::Button,
        RoleVariant::Normal,
    )];
    let calls: [(&str, &[&str]); 5] = [
        (
            "toolbar.button(NONE)",
            &["style/roles.rs, the toolbar's rest capsule"],
        ),
        ("toolbar.button(", &["pages::basic::edit_look_at"]),
        ("toolbar.button_label(", &["pages::basic::edit_look_at"]),
        ("toolbar.button_icon(", &["demo::tool_icon_color"]),
        (
            "button.",
            &[
                "chrome::toggle_checked_pens",
                "chrome::toggle_icon_colour",
                "primary_pen",
                "next::flat_button",
            ],
        ),
    ];
    let homes = [
        "toolbar.button(NONE).fill",
        "toolbar.button(NONE).corners",
        "toolbar.button(HOVER|CHECKED).fill",
        "toolbar.button(PRESSED|CHECKED).fill",
        "toolbar.button_label(CHECKED).color",
        "toolbar.button_label(HOVER|CHECKED).color",
        "toolbar.button_label(PRESSED|CHECKED).color",
        "toolbar.button_icon(",
        "; variant=Flat)",
        "; variant=Primary).stroke",
        "button.body(HOVER).stroke.width",
    ];
    let mut checked = 0usize;
    let mut wrong = Vec::new();
    for element in listed_elements().iter().filter(|e| {
        e.id.starts_with("chrome.toolbar.")
            || e.id.starts_with("basic.icons.")
            || e.id.starts_with("chrome.status_bar.toggle")
            || e.id == "basic.buttons.primary"
            || e.id == "basic.buttons.dialog_default"
            || e.id == "chrome.info.copy"
    }) {
        for home in element
            .homes
            .iter()
            .filter(|h| homes.iter().any(|listed| h.contains(listed)) && !h.ends_with(".size"))
        {
            checked += 1;
            let how = crate::info::listed_how(&manifest, &element.id, home, &seams);
            let call = calls
                .iter()
                .find(|(prefix, _)| home.starts_with(prefix))
                .is_some_and(|(_, names)| names.iter().any(|name| how.contains(name)));
            if !how.starts_with("egui: ") || !call {
                wrong.push(format!("{} {home}: {how:?}", element.id));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    assert!(checked > 0, "no listed element lists such a home");
    Ok(())
}

/// The Basic switches' label homes, which no `mapping.toml` row holds, are routed to the
/// companion crate's switch that reads them ([`crate::info::SWITCH_LABEL_ROUTES`]), not shown
/// as unreachable: every `switch.label` home the three switches and their labels list, each
/// with its claim, painted by the switch, or not read by it with its reason.
#[test]
fn the_switch_label_routes_name_the_switch_that_draws_them() -> Result<(), String> {
    const READ: &str = "egui: native-theme-egui-widgets Switch";
    const CLAIMS: [(&str, Option<&str>); 4] = [
        ("switch.label(NONE)", None),
        ("switch.label(NONE).color", None),
        (
            "switch.label(CHECKED).color",
            Some(
                "egui: not read by native-theme-egui-widgets Switch \
                 (docs/archive/todo_egui-widgets-spec.md §4.1), which letters the on label as \
                 the off one",
            ),
        ),
        ("switch.label(DISABLED|CHECKED).color", None),
    ];
    let manifest = Manifest::parse(MANIFEST)?;
    let seams = [crate::demo::Seam::Role(
        native_theme_egui::Role::Switch,
        RoleVariant::Normal,
    )];
    let mut checked = 0usize;
    let mut wrong = Vec::new();
    for element in listed_elements()
        .iter()
        .filter(|e| e.id.starts_with("basic.switches."))
    {
        for home in element
            .homes
            .iter()
            .filter(|h| h.starts_with("switch.label("))
        {
            checked += 1;
            let how = crate::info::listed_how(&manifest, &element.id, home, &seams);
            let claimed = match CLAIMS.iter().find(|(h, _)| *h == home.as_str()) {
                Some((_, None)) => {
                    how.starts_with(READ) && how.contains(" paints it") && !how.contains("not read")
                }
                Some((_, Some(unread))) => how.starts_with(unread),
                None => false,
            };
            if !claimed || how.contains("not reachable") {
                wrong.push(format!("{} {home}: {how:?}", element.id));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    assert_eq!(
        checked, 12,
        "the font and the colour of each of the three switches and of its label"
    );
    Ok(())
}

/// A tab's label is drawn at the weight the theme states for it, so the tabs' Widget Info names
/// the call that applies it (`demo::next_tab_label`) for the label's weight, its other fields
/// by their `mapping.toml` rows: on material-3, whose `tab.label(NONE).weight` is stated apart
/// from the body's, each page tab and inspector tab is in the face registered at that weight
/// (`demo::weighted_family`), the faces held on every host ([`hold_weights`]).
#[test]
fn a_tab_label_is_drawn_at_its_stated_weight() -> Result<(), String> {
    use crate::info::{TAB_ELEMENTS, TAB_LABEL_PER_CALL};
    let manifest = Manifest::parse(MANIFEST)?;
    let seams = [crate::demo::Seam::Role(
        native_theme_egui::Role::Tab,
        RoleVariant::Normal,
    )];
    let home = "tab.label(NONE)";
    let plain = crate::info::how_line(&manifest, home, &seams);
    for id in TAB_ELEMENTS {
        let how = crate::info::listed_how(&manifest, id, home, &seams);
        for (_, route) in TAB_LABEL_PER_CALL {
            assert!(how.contains(route), "{id}: {how:?}");
        }
        assert_ne!(
            how, plain,
            "{id}: the label's weight keeps the manifest's route"
        );
    }
    assert!(
        !plain.contains("demo::next_tab_label"),
        "an element that is no tab keeps the manifest's routes: {plain:?}"
    );

    let preset = "material-3";
    let mut basic = open_page_under(Page::Basic, egui::Theme::Light, preset);
    let r = basic
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme installed")?
        .clone();
    let weight = native_theme_egui::next::role_font_weight(&r, native_theme_egui::Role::Tab)
        .map_err(|e| e.to_string())?;
    let body = native_theme_egui::next::font_weight(&r).map_err(|e| e.to_string())?;
    assert_ne!(weight, body, "{preset}: the tab's weight is not the body's");
    hold_weights(&mut basic, &[weight]);
    for (id, label) in [
        ("chrome.page_tabs.basic", "Basic"),
        ("chrome.page_tabs.buttons", "Buttons"),
        ("chrome.side_panel.inspector_tabs.widget", "Widget"),
        ("chrome.side_panel.inspector_tabs.theme", "Theme"),
    ] {
        let tab = basic.state().registry.places()[id];
        assert_eq!(
            galley_font(&basic, label, &[tab]).family,
            weighted(weight, body),
            "{preset}: {id} is not at tab.label(NONE).weight"
        );
    }
    Ok(())
}

/// The face a tab's label is drawn in at its stated weight is one `demo::Semibold::register`
/// registers, not only one a test holds ([`hold_weights`]): on material-3 with its tab label
/// patched to a weight no other text of it is drawn at, with that weight's family dropped from
/// the fonts (as an install's `set_fonts` drops what was added before it), a registration
/// handed egui's own proportional face for every face it asks for (`Semibold::register_with`)
/// binds that family from the patched theme and not from the unpatched one, so the page tabs
/// are drawn in it.
#[cfg(feature = "system-fonts")]
#[test]
fn the_tab_labels_weight_face_is_registered_by_semibold() -> Result<(), String> {
    let preset = "material-3";
    let mut basic = open_page_under(Page::Basic, egui::Theme::Light, preset);
    let unpatched = basic.state().atlas.clone();
    let patch =
        native_theme::next::ThemePatch::from_toml("[[widgets.tab.rules]]\nlabel.weight = 800\n")
            .map_err(|e| e.to_string())?;
    let atlas = unpatched.patched(&patch)?;
    let ctx = basic.ctx.clone();
    basic.state_mut().adopt_pair(&ctx, atlas.clone());
    basic.run_steps(4);
    let r = atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme installed")?;
    let weight = native_theme_egui::next::role_font_weight(r, native_theme_egui::Role::Tab)
        .map_err(|e| e.to_string())?;
    let body = native_theme_egui::next::font_weight(r).map_err(|e| e.to_string())?;
    assert_eq!(weight, 800, "{preset}: the patched tab weight");
    assert_ne!(weight, body, "{preset}: the tab's weight is not the body's");
    let family = crate::demo::weight_family(weight);
    let own = egui::FontDefinitions::default();
    let name = own
        .families
        .get(&egui::FontFamily::Proportional)
        .and_then(|names| names.first())
        .ok_or("egui has no proportional face")?;
    let data = own.font_data.get(name).ok_or("egui's face has no data")?;
    let (bytes, index) = ((*data.font).as_ref().to_vec(), data.index);
    let bound = |ctx: &egui::Context| ctx.fonts(|f| f.definitions().families.contains_key(&family));

    let mut fonts = ctx.fonts(|f| f.definitions().clone());
    fonts.families.remove(&family);
    ctx.set_fonts(fonts);
    basic.run_steps(2);
    assert!(!bound(&ctx), "{preset}: the family is still bound");
    basic
        .state_mut()
        .semibold
        .register_with(&ctx, &unpatched, |_, _, _| Some((bytes.clone(), index)));
    basic.run_steps(2);
    assert!(
        !bound(&ctx),
        "{preset}: the unpatched theme asks for a face at the patched tab weight"
    );
    basic
        .state_mut()
        .semibold
        .register_with(&ctx, &atlas, |_, _, _| Some((bytes.clone(), index)));
    basic.run_steps(2);
    assert!(
        bound(&ctx),
        "{preset}: the tab weight's face is not registered"
    );
    let tab = basic.state().registry.places()["chrome.page_tabs.basic"];
    assert_eq!(
        galley_font(&basic, "Basic", &[tab]).family,
        family,
        "{preset}: the Basic tab is not in the registered face"
    );
    Ok(())
}

/// The Basic page paints the List's and the table's frames per call (`pages::basic::list_frame`)
/// and rounds the rows' fills where they meet them (`inner_corners`), so their routes name that
/// call, not the `Role::List` scope's style field `mapping.toml` names for the connector: the
/// frames' and the rows' radius and width (the frames' fill and line colour are per-call rows of
/// the manifest already).
#[test]
fn the_list_and_table_frame_routes_name_the_call_that_paints_them() {
    let manifest = Manifest::parse(MANIFEST).expect("the manifest parses");
    let seams = [crate::demo::Seam::Role(
        native_theme_egui::Role::List,
        RoleVariant::Normal,
    )];
    let mut checked = 0usize;
    let mut wrong = Vec::new();
    for element in listed_elements() {
        let id = element.id.as_str();
        let painted = [
            "basic.list.frame",
            "basic.table.frame",
            "basic.table.header",
        ]
        .contains(&id)
            || id.starts_with("basic.table.row_")
            || id.starts_with("basic.list.row_");
        if !painted {
            continue;
        }
        for leaf in [
            "list.container(NONE).corners",
            "list.container(NONE).stroke.width",
        ] {
            if !element.homes.iter().any(|l| l == leaf) {
                continue;
            }
            checked += 1;
            let how = crate::info::listed_how(&manifest, &element.id, leaf, &seams);
            let call = if element.id.ends_with(".frame") {
                "list_frame"
            } else {
                "inner_corners"
            };
            if !how.contains(call) || how.contains("visuals.") {
                wrong.push(format!("{} {leaf}: {how:?}", element.id));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    assert_eq!(
        checked, 14,
        "the two frames', the table's header and last row's and the List's three rows' two \
         leaves"
    );
    // Only the rows' ids name their elements by prefix: an element under the header or a frame
    // is not that element.
    for (id, call) in [
        ("basic.table.header.name", "inner_corners"),
        ("basic.list.frame.edge", "list_frame"),
    ] {
        let how = crate::info::listed_how(&manifest, id, "list.container(NONE).corners", &seams);
        assert!(
            !how.contains(call),
            "{id} takes another element's route: {how:?}"
        );
    }
}

/// A selected list or table row takes its unfocused look per call (`pages::basic::RowLook::of`,
/// the highlight and the label queried with `UNFOCUSED_VIEW`), so the routes of its and its
/// label's `colors.selection.unfocused_*` homes name that call and when it takes them.
#[test]
fn the_selected_rows_unfocused_pair_routes_name_row_look() {
    let manifest =
        Manifest::parse(MANIFEST).unwrap_or_else(|error| panic!("the manifest parses: {error}"));
    let seams = [crate::demo::Seam::Role(
        native_theme_egui::Role::List,
        RoleVariant::Normal,
    )];
    let mut checked = 0usize;
    let mut wrong = Vec::new();
    for element in listed_elements() {
        let id = element.id.as_str();
        if !(id.starts_with("basic.table.row_") || id.starts_with("basic.list.row_")) {
            continue;
        }
        for leaf in [
            "colors.selection.unfocused_background",
            "colors.selection.unfocused_foreground",
        ] {
            if !element.homes.iter().any(|l| l == leaf) {
                continue;
            }
            checked += 1;
            let how = crate::info::listed_how(&manifest, id, leaf, &seams);
            if !how.contains("RowLook::of") || !how.contains("not focused") {
                wrong.push(format!("{id} {leaf}: {how:?}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    assert_eq!(
        checked, 5,
        "the table's and the List's selected rows' two leaves and the List's label's text colour"
    );
}

/// The family drawn in a family's place is the substitute the atlas's note for that family
/// names: none for a family with no note, or whose note names no substitute (`demo::drawn_family`
/// over the `(family, substitute)` pairs `demo::unavailable_families` takes from the notes).
#[test]
fn the_drawn_family_is_the_notes_substitute() {
    use crate::demo::drawn_family;
    let unavailable = [("Segoe UI", Some("Adwaita Sans")), ("Cascadia Mono", None)];
    assert_eq!(drawn_family(&unavailable, "Segoe UI"), Some("Adwaita Sans"));
    assert_eq!(drawn_family(&unavailable, "Noto Sans"), None);
    assert_eq!(drawn_family(&unavailable, "Cascadia Mono"), None);
    assert_eq!(drawn_family(&[], "Segoe UI"), None);
}

/// A family the font plan loads is drawn as its notes say (`demo::planned_drawing`, over
/// literal notes): as stated with none; in the substitute a `FontFamilyUnavailable` note names;
/// in egui's own face where that note names none, where the substitute's face failed epaint's
/// parse (a `FontDataInvalid` note under the theme's family, which the substitute's face is
/// registered under), and, as "in no face egui can draw", where the family's own face did.
#[test]
fn a_planned_family_is_drawn_as_its_notes_say() {
    use crate::demo::planned_drawing;
    use native_theme_showcase_support::status::DrawnFamily;
    let own = ("proportional", Some("Ubuntu-Light"));
    let generic = |held_weights| DrawnFamily::Generic {
        toolkit: "egui",
        kind: "proportional",
        family: Some("Ubuntu-Light"),
        held_weights,
    };
    let substituted = [("Segoe UI", Some("Adwaita Sans"))];
    let unsubstituted = [("Segoe UI", None)];
    assert_eq!(
        planned_drawing(&[], &[], "Noto Sans", own),
        DrawnFamily::Stated
    );
    assert_eq!(
        planned_drawing(&substituted, &[], "Segoe UI", own),
        DrawnFamily::Named("Adwaita Sans")
    );
    assert_eq!(
        planned_drawing(&unsubstituted, &[], "Segoe UI", own),
        generic(None)
    );
    assert_eq!(
        planned_drawing(&substituted, &["Segoe UI"], "Segoe UI", own),
        generic(None)
    );
    assert_eq!(
        planned_drawing(&[], &["Noto Sans"], "Noto Sans", own),
        generic(Some(&[]))
    );
    assert_eq!(
        planned_drawing(&[], &["Hack"], "Noto Sans", own),
        DrawnFamily::Stated
    );
}

/// A face the showcase adds at another weight than the body's that fails epaint's parse is
/// skipped and noted, where epaint panicked at the next pass loading it ("Error parsing ...
/// TTF/OTF font file", `epaint/src/text/fonts.rs:990`, in release too): 64 zero bytes added
/// at weight 650 leave the pass running, that weight's family unbound, so text at it is
/// drawn in the body's face (`demo::weighted_family`), and the Theme tab's Notes list it.
#[cfg(feature = "system-fonts")]
#[test]
fn a_weight_face_epaint_cannot_parse_is_skipped_and_noted() {
    let mut harness = open_page(Page::Basic, egui::Theme::Light);
    let ctx = harness.ctx.clone();
    let family = crate::demo::weight_family(650);
    harness.state_mut().semibold.add_face(
        &ctx,
        ("Broken Sans", 650),
        family.clone(),
        vec![0; 64],
        0,
    );
    let ran = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| harness.run_steps(2)));
    assert!(
        ran.is_ok(),
        "the pass after adding an unparsable face panicked"
    );
    assert!(
        !ctx.fonts(|f| f.definitions().families.contains_key(&family)),
        "the unparsable face's family is bound"
    );
    assert_eq!(
        harness.state().semibold.invalid(),
        [("Broken Sans".to_string(), 650)]
    );
    harness.state_mut().inspector_tab = crate::chrome::InspectorTab::Theme;
    harness.run_steps(2);
    let noted = harness
        .query_all(By::new().predicate(|n| {
            n.value().is_some_and(|v| {
                v.starts_with("FontDataInvalid\n") && v.contains("Broken Sans") && v.contains("650")
            })
        }))
        .count();
    assert_eq!(
        noted, 1,
        "the Theme tab's Notes do not list the skipped face"
    );
}

/// A face asked for under two of the showcase's families at one weight is noted once, as the
/// connector notes a face of its plan once (`Note::FontDataInvalid`): the semibold family and
/// `weight_family(600)` both ask for the face at weight 600 where a text role is drawn at 600
/// (windows-11), so the same unparsable face reaches `add_face` twice.
#[cfg(feature = "system-fonts")]
#[test]
fn a_face_skipped_under_two_families_is_noted_once() {
    let mut harness = open_page(Page::Basic, egui::Theme::Light);
    let ctx = harness.ctx.clone();
    for family in [
        egui::FontFamily::Name(crate::demo::SEMIBOLD_FAMILY.into()),
        crate::demo::weight_family(600),
    ] {
        harness
            .state_mut()
            .semibold
            .add_face(&ctx, ("Broken Sans", 600), family, vec![0; 64], 0);
    }
    assert_eq!(
        harness.state().semibold.invalid(),
        [("Broken Sans".to_string(), 600)]
    );
    harness.state_mut().inspector_tab = crate::chrome::InspectorTab::Theme;
    harness.run_steps(2);
    let noted = harness
        .query_all(By::new().predicate(|n| {
            n.value().is_some_and(|v| {
                v.starts_with("FontDataInvalid\n") && v.contains("Broken Sans") && v.contains("600")
            })
        }))
        .count();
    assert_eq!(
        noted, 1,
        "the Theme tab's Notes list the skipped face more than once"
    );
}

/// The fonts of the galleys of `text` painted in the inspector: across the side panel's width and
/// below its Widget and Theme tabs, so a tab's own label and the theme settings' are not among
/// them, and a line the inspector's scroll area holds below the window is.
fn inspector_fonts(harness: &Harness<'_, App>, text: &str) -> Vec<egui::FontId> {
    let records = harness.state().registry.records();
    let panel = records
        .iter()
        .find(|r| r.info.kind == "Side panel")
        .map(|r| r.rect)
        .expect("the side panel records itself");
    let below = records
        .iter()
        .filter(|r| r.info.kind == "Tab · Inspector")
        .map(|r| r.rect.bottom())
        .fold(f32::NEG_INFINITY, f32::max);
    assert!(below.is_finite(), "the inspector records its tabs");
    harness
        .output()
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(shape)
                if shape.galley.job.text == text
                    && panel.x_range().contains(shape.pos.x)
                    && shape.pos.y >= below =>
            {
                shape
                    .galley
                    .job
                    .sections
                    .first()
                    .map(|s| s.format.font_id.clone())
            }
            _ => None,
        })
        .collect()
}

/// M9-122: the Theme tab draws its section names as the Widget tab draws its own, in
/// `InfoText`'s semibold at the side panel's text size, so switching tabs keeps one heading
/// style; the font is read from the Widget tab's painted "Theme" section name, and the
/// section-heading role the names were drawn in is a size apart under the preset.
#[test]
fn the_theme_tabs_section_names_are_the_widget_tabs_semibold() -> Result<(), String> {
    let mut harness = open_page(Page::Basic, egui::Theme::Light);
    let pos = centre_of(&harness, "toggle button (on)");
    hover_and_settle(&mut harness, pos);
    let shown = inspector_fonts(&harness, "Theme");
    let [want] = shown.as_slice() else {
        panic!("the Widget tab paints one \"Theme\" section name: {shown:?}");
    };
    let heading = native_theme_egui::next::text_role_font(
        harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or("no 0.8.0 theme installed")?,
        native_theme_egui::TextRole::SectionHeading,
    );
    assert_ne!(
        heading.size, want.size,
        "{TEST_PRESET}: the section-heading role is the panel's size, so the test shows nothing"
    );
    harness.state_mut().inspector_tab = crate::chrome::InspectorTab::Theme;
    harness.run_steps(2);
    for name in [
        "Theme",
        "Notes",
        "Fonts",
        "Window",
        "Fields left at egui's own value",
    ] {
        assert_eq!(
            inspector_fonts(&harness, name),
            std::slice::from_ref(want),
            "the Theme tab's {name:?} section name"
        );
    }
    Ok(())
}

/// The label of the theme's `stated` family the showcase must show, read from what the
/// harness has installed: the stated name alone where the atlas holds no
/// `Note::FontFamilyUnavailable` or `Note::FontDataInvalid` for it; the platform's substitute
/// where its note names one (`status::family_label`); egui's generic family in egui's own first
/// face of it, as the fonts the pass draws with hold it, where its note names none, where a
/// face registered under it failed epaint's parse, or wherever the build loads no system font
/// (feature `system-fonts` off), `monospace` for the monospace font, else `proportional`.
fn drawn_label(harness: &Harness<'_, App>, stated: &str, mono: bool) -> String {
    use native_theme_showcase_support::status::{DrawnFamily, family_label};
    let (kind, chain) = if mono {
        ("monospace", egui::FontFamily::Monospace)
    } else {
        ("proportional", egui::FontFamily::Proportional)
    };
    let own = harness.ctx.fonts(|f| {
        f.definitions()
            .families
            .get(&chain)
            .and_then(|faces| faces.first().cloned())
    });
    if !cfg!(feature = "system-fonts") {
        return DrawnFamily::Unloaded {
            toolkit: "egui",
            kind,
            family: own.as_deref(),
        }
        .label(stated);
    }
    let notes = harness.state().atlas.notes();
    let noted = notes.iter().find_map(|note| match note {
        native_theme_egui::Note::FontFamilyUnavailable {
            family, substitute, ..
        } if &**family == stated => Some(substitute.clone()),
        _ => None,
    });
    let dropped = notes.iter().any(|note| {
        matches!(note, native_theme_egui::Note::FontDataInvalid { family, .. }
            if &**family == stated)
    });
    let generic = |held_weights| {
        DrawnFamily::Generic {
            toolkit: "egui",
            kind,
            family: own.as_deref(),
            held_weights,
        }
        .label(stated)
    };
    match noted {
        Some(Some(substitute)) if !dropped => family_label(stated, Some(&substitute)),
        Some(_) => generic(None),
        None if dropped => generic(Some(&[])),
        None => stated.to_string(),
    }
}

/// The value of Widget Info's row for `leaf`, the text after the leaf on its first line: a
/// row is a `Label`, whose text is its AccessKit value (`egui/src/response.rs:962-964`).
fn info_row_value(harness: &Harness<'_, App>, leaf: &str) -> String {
    let prefix = format!("{leaf} ");
    harness
        .query_all(By::new().predicate(|n| n.value().is_some_and(|v| v.starts_with(&prefix))))
        .filter_map(|n| n.accesskit_node().value())
        .find_map(|value| {
            value
                .lines()
                .next()
                .and_then(|line| line.strip_prefix(&prefix))
                .map(str::to_string)
        })
        .unwrap_or_else(|| panic!("Widget Info shows no {leaf} row"))
}

/// Every font label names the family egui draws, as the atlas records it: the status bar's
/// font item, the Theme tab's `font_family` and `mono_font_family` rows, Widget Info's
/// `defaults.font.family` row (hover a Basic group heading), its `switch.label(NONE)` row (the
/// Basic "Off" switch), its `defaults.mono_font` row (the Basic "Monospace" line) and its
/// `button.font` row (the Basic "Button"), under windows-11, set directly as `--theme` does not
/// take it here. Conditioned on the host: where the system lacks "Segoe UI", the
/// atlas notes it and the labels name the substitute drawn in its place (fontconfig's match on
/// Linux); where it has it, the stated name alone; and without feature `system-fonts`,
/// egui's own face. The expected texts are read from the installed atlas ([`drawn_label`]).
#[test]
fn every_font_label_names_the_drawn_family() -> Result<(), String> {
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "windows-11");
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .cloned()
        .ok_or("windows-11: no 0.8.0 theme installed")?;
    let (body_family, mono_family) = (&r.typography.body.family, &r.typography.monospace.family);
    let body = drawn_label(&harness, body_family, false);
    println!("windows-11's {body_family:?} is shown as {body:?}");
    let mono = drawn_label(&harness, mono_family, true);
    println!("windows-11's {mono_family:?} is shown as {mono:?}");
    let button_family = r.button.label(native_theme::next::WidgetState::NONE).family;
    let button = drawn_label(&harness, &button_family, false);
    let switch_family = r.switch.label(native_theme::next::WidgetState::NONE).family;
    let switch = drawn_label(&harness, &switch_family, false);
    // A `Label`'s text is its AccessKit value (`egui/src/response.rs:962-964`).
    let values = |harness: &Harness<'_, App>| -> Vec<String> {
        harness
            .query_all(By::new().predicate(|n| n.value().is_some()))
            .filter_map(|n| n.accesskit_node().value())
            .collect()
    };
    let font_item = format!(
        "{body} {}",
        native_theme_showcase_support::next::status::defined_size(&r.typography.body)
    );
    let environment = values(&harness)
        .into_iter()
        // The preset item names the selection, windows-11.
        .find(|v| v.contains(" · ") && v.contains("windows-11"))
        .ok_or("no environment line")?;
    assert!(
        environment.split(" · ").any(|item| item == font_item),
        "the status bar does not name the drawn family as {font_item:?}: {environment}"
    );
    harness.state_mut().inspector_tab = crate::chrome::InspectorTab::Theme;
    harness.run_steps(2);
    let shown = values(&harness);
    for (key, label) in [("font_family", &body), ("mono_font_family", &mono)] {
        let row = format!("{key}\n{label}");
        assert!(
            shown.contains(&row),
            "the Theme tab shows no {row:?}: {:?}",
            shown
                .iter()
                .filter(|v| v.starts_with(key))
                .collect::<Vec<_>>()
        );
    }
    harness.state_mut().inspector_tab = crate::chrome::InspectorTab::Widget;
    harness.run_steps(2);
    let mut row = |kind: &str, leaf: &str| {
        let pos = centre_of(&harness, kind);
        hover_and_settle(&mut harness, pos);
        info_row_value(&harness, leaf)
    };
    assert_eq!(row("heading", "typography.body.family"), body);
    for (kind, leaf, label) in [
        ("switch (off)", "switch.label(NONE)", &switch),
        ("Label (monospace)", "typography.monospace", &mono),
        ("button (enabled)", "button.label(NONE)", &button),
    ] {
        let font = row(kind, leaf);
        assert!(
            font.starts_with(&format!("{label} ")),
            "the {leaf} row does not name the drawn family {label:?}: {font}"
        );
    }
    Ok(())
}

/// What egui draws a font whose stated family is not the one the font plan loaded for its kind
/// in, as the showcase must name it: the face of the installed 0.8.0 light theme's
/// `typography.body` (or `typography.monospace` for the monospace font), that family labelled by [`drawn_label`]
/// (connector `fonts.rs`: the plan registers the light variant's two default families, and
/// every `FontId` the connector hands out names `Proportional` or `Monospace`).
fn other_font_label(
    harness: &Harness<'_, App>,
    stated: &str,
    mono: bool,
) -> Result<String, String> {
    // The plan loads the installed 0.8.0 light theme's families.
    let light = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 light theme installed")?;
    let (font, planned) = if mono {
        (
            "the light variant's typography.monospace",
            light.typography.monospace.family.to_string(),
        )
    } else {
        (
            "the light variant's typography.body",
            light.typography.body.family.to_string(),
        )
    };
    let family = drawn_label(harness, &planned, mono);
    Ok(
        native_theme_showcase_support::status::DrawnFamily::OtherFont {
            toolkit: "egui",
            font,
            family: &family,
        }
        .label(stated),
    )
}

/// A widget font whose stated family is not the body's is named by the face egui draws it in,
/// the body's, which is where egui draws every font but the monospace one (the connector's
/// never-`Name` invariant, spec §8.1, §5.8 item 7): `button.font.family` set to "Noto Serif"
/// on kde-breeze, as a reader's own widget font would state it (KDE's `menuFont`, Windows'
/// menu font), hover the Basic "Button".
#[test]
fn a_widget_font_of_another_family_is_named_by_the_face_egui_draws() -> Result<(), String> {
    use native_theme::next::{ButtonRule, TextPatch, ThemePatch};
    let (mut harness, _) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", |_| {
        Some(
            ThemePatch::new()
                .button(ButtonRule::new().label(TextPatch::new().family("Noto Serif".into()))),
        )
    })?;
    let label = other_font_label(&harness, "Noto Serif", false)?;
    let pos = centre_of(&harness, "button (enabled)");
    hover_and_settle(&mut harness, pos);
    let font = info_row_value(&harness, "button.label(NONE)");
    assert!(
        font.starts_with(&format!("{label} ")),
        "the button.label(NONE) row does not name the face egui draws, {label:?}: {font}"
    );
    Ok(())
}

/// Under a dark theme whose `typography.body.family` is not the light one's, the status bar and
/// the Theme tab name the face egui draws, the light theme's: the connector's font plan reads
/// the light theme alone (`next::font_plan_from_system(light)`). No bundled preset or OS reader
/// gives the two schemes other families; the test sets one on the installed 0.8.0 themes.
#[test]
fn a_dark_family_of_its_own_is_named_by_the_face_egui_draws() -> Result<(), String> {
    let mut harness = open_page_under(Page::Basic, egui::Theme::Dark, "kde-breeze");
    let edited = harness.state().atlas.edited(|r| {
        if r.context.scheme == native_theme::next::Scheme::Dark {
            r.typography.body.family = "Noto Serif".into();
        }
    })?;
    let ctx = harness.ctx.clone();
    harness.state_mut().adopt_pair(&ctx, edited);
    harness.run_steps(4);
    assert_eq!(harness.ctx.theme(), egui::Theme::Dark);
    let label = other_font_label(&harness, "Noto Serif", false)?;
    let body = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Dark)
        .map(|r| r.typography.body.clone())
        .ok_or("no 0.8.0 dark theme installed")?;
    assert_eq!(&*body.family, "Noto Serif");
    let values = |harness: &Harness<'_, App>| -> Vec<String> {
        harness
            .query_all(By::new().predicate(|n| n.value().is_some()))
            .filter_map(|n| n.accesskit_node().value())
            .collect()
    };
    let font_item = format!(
        "{label} {}",
        native_theme_showcase_support::next::status::defined_size(&body)
    );
    let environment = values(&harness)
        .into_iter()
        .find(|v| v.contains(" · ") && v.contains("kde-breeze"))
        .ok_or("no environment line")?;
    assert!(
        environment.split(" · ").any(|item| item == font_item),
        "the status bar does not name the face egui draws as {font_item:?}: {environment}"
    );
    harness.state_mut().inspector_tab = crate::chrome::InspectorTab::Theme;
    harness.run_steps(2);
    let row = format!("font_family\n{label}");
    assert!(
        values(&harness).contains(&row),
        "the Theme tab shows no {row:?}"
    );
    Ok(())
}

/// The dump is the object `docs/showcase-elements.toml` describes: the kind, the preset, the
/// variant, the scale and a rectangle per placed element.
#[test]
fn the_layout_dump_is_the_lists_object() {
    let harness = open_page(Page::Basic, egui::Theme::Dark);
    let app = harness.state();
    let json = crate::app::layout_json(
        app.registry.places(),
        app.preset_key(),
        harness.ctx.theme(),
        harness.ctx.pixels_per_point(),
    );
    assert_eq!(json["kind"], "egui");
    assert_eq!(json["preset"], TEST_PRESET);
    assert_eq!(json["variant"], "dark");
    let window = &json["elements"]["chrome.window"];
    assert_eq!(window["x"], 0.0);
    assert_eq!(window["w"], f64::from(crate::WINDOW_SIZE.x));
    assert_eq!(window["h"], f64::from(crate::WINDOW_SIZE.y));
}

/// M8-44: a layout dump (`--dump-layout`) records the chrome at rest wherever the real pointer
/// is. The window's pointer moved over the tooltip button, in the input `raw_input_hook` is
/// handed, leaves the dump byte for byte the one written with no pointer: Widget Info holds its
/// hint (`chrome.info.hint`) and no hovered widget's rows, and the status bar names no hovered
/// widget (`chrome.status_bar.shown`). The same pointer does change the layout of a window that
/// is not dumped, which is what makes the first half mean something.
#[test]
fn a_layout_dump_is_the_same_under_the_pointer() {
    let args = || cli(&[("--theme", TEST_PRESET), ("--tab", "basic")]);
    let at = {
        let mut harness = open(egui::Theme::Light, args());
        install_under(&mut harness, egui::Theme::Light, "kde-breeze");
        let Some(rect) = harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.info.kind == "tooltip button")
            .map(|r| r.rect)
        else {
            panic!("no tooltip button record");
        };
        rect.center()
    };
    // The pointer is over the window before the dump's layout has had the time to settle, as
    // the desktop's is when the window opens under it: the harness's first passes, which
    // `build_eframe` runs, and the install's take no time (`with_step_dt`), so the steps below
    // set the clock.
    let run = |dump: Option<&std::path::Path>, pointer: bool| {
        let mut args = args();
        args.dump_layout = dump.map(|path| path.display().to_string());
        let mut harness = Harness::builder()
            .with_context_setup(emoji_fallbacks)
            .with_theme(egui::Theme::Light)
            .with_size(crate::WINDOW_SIZE)
            .with_step_dt(0.0)
            .build_eframe(move |cc| {
                App::new(cc, &args).expect("the showcase starts under a bundled preset")
            });
        let built = harness.ctx.input(|i| i.time);
        install_under(&mut harness, egui::Theme::Light, "kde-breeze");
        assert_eq!(
            harness.ctx.input(|i| i.time),
            built,
            "the install's passes took no time"
        );
        for step in 0..16u32 {
            let ctx = harness.ctx.clone();
            let mut input = std::mem::take(harness.input_mut());
            input.time = Some(f64::from(step) * 0.5);
            if pointer && step == 0 {
                input.events.push(egui::Event::PointerMoved(at));
            }
            eframe::App::raw_input_hook(harness.state_mut(), &ctx, &mut input);
            *harness.input_mut() = input;
            harness.step();
        }
        let app = harness.state();
        crate::app::layout_json(
            app.registry.places(),
            app.preset_key(),
            harness.ctx.theme(),
            harness.ctx.pixels_per_point(),
        )
    };
    let dumps: Vec<String> = [false, true]
        .into_iter()
        .map(|pointer| {
            let path = std::env::temp_dir().join(format!(
                "showcase-egui-dump-{}-{pointer}.json",
                std::process::id()
            ));
            let last = run(Some(&path), pointer);
            let text = std::fs::read_to_string(&path).expect("the layout dump was written");
            std::fs::remove_file(&path).expect("the layout dump is removed");
            // The dump is the layout the run ended in, as the control below measures it (`{:#}`
            // is `serde_json::to_string_pretty`, which the dump is written with): one written
            // before the pointer reached the page would make the equality vacuous.
            assert!(
                text == format!("{last:#}"),
                "pointer {pointer}: the dump is not the layout the run settled \
                 in:\n{text}\n---\n{last:#}"
            );
            text
        })
        .collect();
    assert!(
        dumps[0] == dumps[1],
        "the pointer changed the layout dump:\n{}\n---\n{}",
        dumps[0],
        dumps[1]
    );
    let dump: serde_json::Value = serde_json::from_str(&dumps[0]).expect("the layout dump is JSON");
    let elements = &dump["elements"];
    assert!(
        elements.get("chrome.info.hint").is_some(),
        "the dump holds no Widget Info hint"
    );
    for id in ["chrome.info.title", "chrome.status_bar.shown"] {
        assert!(elements.get(id).is_none(), "the dump at rest holds {id}");
    }
    assert_ne!(
        run(None, false),
        run(None, true),
        "the pointer does not change the layout of a window that is not dumped either, so \
         the dump half of this test proves nothing"
    );
}

/// The macOS system menu's actions are drained and dropped while the window is shielded from the
/// keyboard, so its key equivalents (⌘, ⌘B and ⌘K), which never pass through `raw_input_hook`, run
/// nothing in a captured or dumped window; unshielded they all run, in order. The flags shield
/// as `CliArgs::shields_keyboard` says.
#[test]
fn the_system_menus_actions_are_dropped_while_the_keyboard_is_shielded() {
    use crate::chrome::Action;
    let sent = [
        Action::OpenPreferences,
        Action::ToggleSidePanel,
        Action::OpenCommandPalette,
    ];
    let args = |argv: &[&str]| CliArgs::parse(argv.iter().map(|a| (*a).to_string()));
    for (argv, shielded) in [
        (&[][..], false),
        (&["--capture"][..], true),
        (&["--screenshot", "p"], true),
        (&["--dump-layout", "f"], true),
    ] {
        let shields = args(argv).shields_keyboard();
        assert_eq!(shields, shielded, "{argv:?} shields the keyboard");
        let mut channel = sent.into_iter();
        let run = crate::app::system_menu_actions(channel.by_ref(), shields);
        assert_eq!(
            channel.next(),
            None,
            "{argv:?}: an action is left on the channel"
        );
        let want: &[Action] = if shielded { &[] } else { &sent };
        assert_eq!(run, want, "{argv:?}: the system menu's actions run");
    }
}

/// Keys typed while a capture runs reach the window the desktop gives the focus, and changed
/// what a capture recorded (the Basic page's focused field read "Focusedones?"): under
/// `--capture`, `--screenshot` and `--dump-layout` the window takes no keyboard input
/// (`App::raw_input_hook`, `CliArgs::shields_keyboard`). Text typed into the focused field and
/// the side-panel toggle's shortcut, handed to the hook as eframe hands them, change nothing;
/// the same keys in a run that neither captures nor dumps type into the field and hide the
/// panel, which is what makes the first half mean something. A `--screenshot` run shields the
/// keyboard as a `--capture` one does; it is not opened here, since off macOS and Windows it
/// ends the run when it captures.
#[test]
fn a_captured_or_dumped_window_takes_no_keyboard_input() {
    let dump = std::env::temp_dir().join(format!(
        "showcase-egui-keyboard-{}.json",
        std::process::id()
    ));
    let shot = CliArgs::parse(["--screenshot".to_string(), "p.png".to_string()]);
    assert!(shot.shields_keyboard(), "--screenshot takes keyboard input");
    let runs = [
        ("--capture", true),
        ("--dump-layout", true),
        ("no flag", false),
    ];
    for (flag, shielded) in runs {
        let mut args = cli(&[("--theme", TEST_PRESET), ("--tab", "basic")]);
        match flag {
            "--capture" => args.capture = true,
            "--dump-layout" => args.dump_layout = Some(dump.display().to_string()),
            _ => {}
        }
        assert_eq!(args.shields_keyboard(), shielded, "{flag}");
        let mut harness = open(egui::Theme::Light, args);
        harness.run_steps(SETTLE);
        let before = harness.state().demo_state.basic_focused.clone();
        let ctx = harness.ctx.clone();
        let mut input = std::mem::take(harness.input_mut());
        input.events.push(egui::Event::Text("ones?".to_string()));
        input.events.push(egui::Event::Key {
            key: egui::Key::B,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        });
        eframe::App::raw_input_hook(harness.state_mut(), &ctx, &mut input);
        *harness.input_mut() = input;
        harness.run_steps(SETTLE);
        let app = harness.state();
        let typed = app.demo_state.basic_focused != before;
        assert_eq!(
            typed, !shielded,
            "{flag}: the focused field holds {:?}, from {before:?}",
            app.demo_state.basic_focused
        );
        assert_eq!(
            app.side_panel_visible, shielded,
            "{flag}: the side-panel toggle's shortcut"
        );
    }
    let _ = std::fs::remove_file(&dump);
}

/// A `--dump-layout` run whose layout never settles gives up at the shared crate's
/// `CAPTURE_WAIT` after its first pass, with the dump's flag and path, as the gpui and iced
/// showcases give up theirs (the showcases' one exit rule): not before, still after a
/// `--screenshot` alone is written, and never once the dump is written or where the run asked
/// for none.
#[test]
fn a_dump_that_is_never_written_fails_after_its_bound() {
    use native_theme_showcase_support::CAPTURE_WAIT;
    use native_theme_showcase_support::cli::{Output, Outputs};

    let both =
        CliArgs::parse(["--dump-layout", "f.json", "--screenshot", "p.png"].map(str::to_string));
    let mut outputs = Outputs::new(&both);
    let bound = CAPTURE_WAIT.as_secs_f64();
    let given_up =
        |outputs: &Outputs, waited: f64| crate::app::dump_given_up(outputs, "f.json", waited);
    assert_eq!(given_up(&outputs, bound - 0.5), None, "gave up early");
    let failure = given_up(&outputs, bound);
    assert!(
        failure.as_deref().is_some_and(|f| f.starts_with(&format!(
            "--dump-layout f.json: no layout was written within {} s",
            CAPTURE_WAIT.as_secs()
        ))),
        "{failure:?}"
    );
    let _ = outputs.written(Output::Screenshot);
    assert!(
        given_up(&outputs, bound).is_some(),
        "a written screenshot ended the wait for the dump"
    );
    let _ = outputs.written(Output::Dump);
    assert_eq!(given_up(&outputs, bound), None, "a written dump failed");
    let none = Outputs::new(&CliArgs::default());
    assert_eq!(given_up(&none, bound), None, "a run with no dump failed");
}

/// `--screenshot`'s frame check needs the display's scale factor, which egui may not know:
/// `content_px` refuses then, before the shared crate's `check_frame_capture` is called, and
/// otherwise gives the content in physical pixels at that scale.
#[test]
fn a_capture_without_the_scale_factor_is_refused() {
    let ctx = egui::Context::default();
    let content = |pixels_per_point: Option<f32>| {
        let mut input = egui::RawInput::default();
        let info = input.viewports.entry(egui::ViewportId::ROOT).or_default();
        info.inner_rect = Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            crate::WINDOW_SIZE,
        ));
        info.native_pixels_per_point = pixels_per_point;
        // The pass's font atlas upload is nobody's to apply here.
        ctx.run_ui(input, |_| {}).textures_delta.clear();
        crate::app::content_px(&ctx)
    };
    assert_eq!(
        content(None),
        Err("the display's scale factor is unknown".to_string())
    );
    assert_eq!(content(Some(1.25)), Ok(((1600, 900), 1.25)));
}

/// The Basic page's edit-mode switch starts checked and a click switches it: its AccessKit node
/// "Edit" is selected at start and not after a click. Under kde-breeze light, set directly (a
/// preset of another platform is refused on the command line), which states the toolbar's
/// checked fill and the checked text: the fill handed to the button (`pages::basic::edit_look`)
/// is `toolbar_button_checked_background` and is painted over the button while it is on, and the
/// text colour its scope takes, which tints its icon as it loads, is the checked text; off, it
/// is handed no fill and not painted in that one. Under windows-11 light, whose checked text is
/// not the toolbar's own, Edit's label, under an icon theme that gives no icon, is painted in the
/// checked text while on, and Copy's beside it is not.
#[test]
fn the_basic_edit_button_starts_checked_and_switches() -> Result<(), String> {
    use crate::pages::basic::{ToggleOn, edit_look};
    use native_theme::next::WidgetState;
    use native_theme_egui::next::state_stroke_color_of;
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    harness.run_steps(SETTLE);
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .cloned()
        .ok_or("kde-breeze's 0.8.0 theme is not installed")?;
    // Each colour from its home in kde-breeze light's preset: `toolbar.button(CHECKED).fill`,
    // [Colors:Button] ForegroundNormal 35,38,41 at 0.125 (platform-facts.md §2.13), and
    // `button.label(CHECKED).color`, ButtonText 35,38,41 in every state (platform-facts.md §2.3).
    let fill = Some(egui::Color32::from_rgba_unmultiplied(
        0x23, 0x26, 0x29, 0x20,
    ));
    let text = Some(egui::Color32::from_rgb(0x23, 0x26, 0x29));
    assert_eq!(
        edit_look(Some(&r)),
        ToggleOn { fill, text },
        "Edit while on: (the fill handed to the button, the text colour)"
    );
    assert_eq!(
        edit_look(None),
        ToggleOn {
            fill: None,
            text: None
        },
        "Edit without an installed 0.8.0 theme: egui's own selected button"
    );
    let selected = |harness: &Harness<'_, App>| {
        harness
            .get_by_role_and_label(Role::Button, "Edit")
            .accesskit_node()
            .toggled()
            == Some(egui::accesskit::Toggled::True)
    };
    // Whether the toolbar's checked fill is painted over the button's rectangle.
    let filled = |harness: &Harness<'_, App>| {
        let edit = harness
            .state()
            .registry
            .places()
            .get("basic.icons.edit")
            .copied();
        assert!(edit.is_some(), "basic.icons.edit is not placed");
        let edit = edit.unwrap_or(egui::Rect::NOTHING);
        harness
            .output()
            .shapes
            .iter()
            .any(|clipped| match &clipped.shape {
                egui::Shape::Rect(r) => {
                    Some(r.fill) == fill && r.brush.is_none() && edit.contains_rect(r.rect)
                }
                _ => false,
            })
    };
    assert!(
        harness.state().demo_state.basic_edit,
        "Edit is not on at start"
    );
    assert!(selected(&harness), "Edit's node is not selected at start");
    assert!(
        filled(&harness),
        "Edit while on is not filled with the toolbar's checked fill"
    );
    // E9: at rest, while on, lined in the pen `toolbar.button(CHECKED)` states: kde-breeze
    // light's #d1d1d2, 1 px; under the pointer in the one `CHECKED | HOVER` states, #3daee9
    let pen_of = |state: WidgetState| {
        let part = r.toolbar.button(state);
        native_theme_egui::next::stroke_width_of(&part)
            .zip(state_stroke_color_of(&part))
            .map(|(width, colour)| egui::Stroke::new(width, colour))
    };
    let pen = egui::Stroke::new(1.0, egui::Color32::from_rgb(0xd1, 0xd1, 0xd2));
    assert_eq!(pen_of(WidgetState::CHECKED), Some(pen));
    let hover_pen = pen_of(WidgetState::CHECKED.union(WidgetState::HOVER));
    assert!(
        hover_pen.is_some() && hover_pen != Some(pen),
        "kde-breeze states the hovered checked pen apart from the one at rest: {hover_pen:?}"
    );
    let lined_in = |harness: &Harness<'_, App>, pen: egui::Stroke| {
        let edit = harness.state().registry.places()["basic.icons.edit"];
        harness
            .output()
            .shapes
            .iter()
            .any(|clipped| match &clipped.shape {
                egui::Shape::Rect(r) => r.stroke == pen && edit.contains_rect(r.rect),
                _ => false,
            })
    };
    let lined = |harness: &Harness<'_, App>| lined_in(harness, pen);
    assert!(
        lined(&harness),
        "Edit while on, at rest, is not lined in the checked pen"
    );
    let edit_place = harness.state().registry.places()["basic.icons.edit"];
    harness.hover_at(edit_place.center());
    harness.run_steps(3);
    assert!(
        hover_pen.is_some_and(|hover| lined_in(&harness, hover)),
        "Edit while on, hovered, is not lined in the hovered checked pen {hover_pen:?}"
    );

    harness.get_by_role_and_label(Role::Button, "Edit").click();
    harness.run_steps(SETTLE);
    assert!(
        !harness.state().demo_state.basic_edit,
        "a click did not turn Edit off"
    );
    assert!(!selected(&harness), "Edit's node is selected after a click");
    assert!(
        !filled(&harness),
        "Edit while off is filled with the toolbar's checked fill"
    );
    assert!(
        !lined(&harness),
        "Edit while off is lined in the checked pen"
    );
    harness.get_by_role_and_label(Role::Button, "Edit").click();
    harness.run_steps(SETTLE);
    assert!(
        harness.state().demo_state.basic_edit,
        "a second click did not turn Edit on again"
    );

    // Under windows-11 light the checked text (#ffffff) is not the toolbar scope's own text, so
    // the colour Edit's label is painted in tells them apart. A freedesktop theme that does not
    // exist gives no icon on any platform (`find_icon` never falls back to hicolor), so Copy and
    // Edit show their labels.
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "windows-11");
    harness.state_mut().settings.icon = IconSetChoice::Freedesktop("no-such-icon-theme".into());
    harness.run_steps(SETTLE);
    // windows-11 light's `button.label(CHECKED).color`: the accent's foreground, #ffffff (the
    // preset's `accent_foreground`, from 0.7.2's windows-11.toml `button.checked_text_color`).
    let checked = Some(egui::Color32::WHITE);
    assert!(
        harness.state().demo_state.basic_edit,
        "Edit is not on at start"
    );
    let copy = painted_label_colour(&harness, "Copy", "basic.icons.copy");
    let edit = painted_label_colour(&harness, "Edit", "basic.icons.edit");
    assert!(
        copy.is_some() && edit.is_some(),
        "windows-11: Copy's or Edit's label is not painted (an icon in its place?): {copy:?}, {edit:?}"
    );
    assert!(
        checked.is_some() && checked != copy,
        "windows-11's checked text {checked:?} is the toolbar's own text {copy:?}, so this part \
         proves nothing"
    );
    assert_eq!(
        edit, checked,
        "windows-11: Edit while on is not lettered in button.label(CHECKED).color"
    );
    Ok(())
}

/// The Basic page's edit-mode switch while on is lettered in `toolbar.button_label(CHECKED)`
/// where a rule states it, ahead of the push button's checked label: macos-monterey states the
/// toolbar's checked label and not `button.label(CHECKED)`, so egui's own selected text
/// (`visuals.selection.stroke`, white) would letter the switch over its light grey checked fill.
/// Under an icon theme that gives no icon, Edit's label is painted, in light and dark.
#[test]
fn the_basic_edit_button_is_lettered_in_the_toolbars_checked_label() -> Result<(), String> {
    use crate::pages::basic::edit_look;
    // macos-monterey's `toolbar.button_label(CHECKED).color`: the toolbar-bezel button as an on
    // toggle, #424242 light, #ececec dark (os-probe run 45634379188,
    // `controls-toolbar_states_sheet.toml`, `*.normal.toolbar_toggle_on` `label`).
    for (theme, stated) in [
        (
            egui::Theme::Light,
            egui::Color32::from_rgb(0x42, 0x42, 0x42),
        ),
        (egui::Theme::Dark, egui::Color32::from_rgb(0xec, 0xec, 0xec)),
    ] {
        let mut harness = open_page_under(Page::Basic, theme, "macos-monterey");
        harness.state_mut().settings.icon = IconSetChoice::Freedesktop("no-such-icon-theme".into());
        harness.run_steps(SETTLE);
        let r = harness
            .state()
            .atlas
            .next_resolved(theme)
            .cloned()
            .ok_or("macos-monterey's 0.8.0 theme is not installed")?;
        assert!(
            harness.state().demo_state.basic_edit,
            "Edit is not on at start"
        );
        assert_eq!(
            edit_look(Some(&r)).text,
            Some(stated),
            "macos-monterey {theme:?}: Edit's text colour while on"
        );
        let edit = painted_label_colour(&harness, "Edit", "basic.icons.edit");
        assert_eq!(
            edit,
            Some(stated),
            "macos-monterey {theme:?}: Edit while on is not lettered in \
             toolbar.button_label(CHECKED).color"
        );
    }
    Ok(())
}

/// Whether a rectangle filled in `fill`, with no texture, is painted inside the place `id`.
fn filled_in(harness: &Harness<'_, App>, id: &str, fill: egui::Color32) -> bool {
    let Some(place) = harness.state().registry.places().get(id).copied() else {
        return false;
    };
    harness
        .output()
        .shapes
        .iter()
        .any(|clipped| match &clipped.shape {
            egui::Shape::Rect(r) => {
                r.fill == fill && r.brush.is_none() && place.contains_rect(r.rect)
            }
            _ => false,
        })
}

/// The Basic page's edit-mode switch while on, under the pointer and pressed, takes the fill and
/// the label macos-monterey light states for the checked toolbar button in that state, in the
/// state of its previous response (`pages::basic::edit_look_at`, `next::toolbar_button_checked`):
/// at rest `toolbar.button(CHECKED)`'s #dfdfdf and label #424242, hovered `(HOVER|CHECKED)`'s
/// #c6c6c6 and #343434, pressed `(PRESSED|CHECKED)`'s #b9b9b9 and #343434, the toolbar-bezel
/// button as an on toggle (os-probe run 45634379188, `controls-toolbar_states_sheet.toml`,
/// `aqua.{normal,hovered,pressed}.toolbar_toggle_on` `fill`, `label`;
/// native-theme/src/next/presets/macos.rs, `toolbar_checked_off_accent`,
/// `toolbar_checked_label`), not egui's selected fill in every state.
#[test]
fn the_basic_edit_button_takes_the_checked_toolbar_buttons_hovered_and_pressed_looks()
-> Result<(), String> {
    let grey = |v: u8| egui::Color32::from_rgb(v, v, v);
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "macos-monterey");
    harness.state_mut().settings.icon = IconSetChoice::Freedesktop("no-such-icon-theme".into());
    harness.run_steps(SETTLE);
    assert!(
        harness.state().demo_state.basic_edit,
        "Edit is not on at start"
    );
    harness.remove_cursor();
    harness.run_steps(3);
    let id = "basic.icons.edit";
    let look = |harness: &Harness<'_, App>, fill: u8, text: u8, what: &str| {
        assert!(
            filled_in(harness, id, grey(fill)),
            "Edit while on, {what}, is not filled in #{fill:02x}{fill:02x}{fill:02x}"
        );
        assert_eq!(
            painted_label_colour(harness, "Edit", id),
            Some(grey(text)),
            "Edit while on, {what}: its label"
        );
    };
    look(&harness, 0xdf, 0x42, "at rest");
    let at = harness
        .state()
        .registry
        .places()
        .get(id)
        .copied()
        .ok_or("basic.icons.edit is not placed")?
        .center();
    harness.hover_at(at);
    harness.run_steps(3);
    look(&harness, 0xc6, 0x34, "hovered");
    harness.drag_at(at);
    harness.run_steps(3);
    look(&harness, 0xb9, 0x34, "pressed");
    Ok(())
}

/// A toolbar button at rest is filled and rounded as `toolbar.button(NONE)` states, on the
/// Basic page's icon row and in the chrome's toolbar: macos-tahoe light's rest capsule, #ffffff,
/// a pill, egui's largest radius (os-probe run 45634379188, macOS 26, toolbar states sheet,
/// `aqua.normal.toolbar_button` `fill`, `*.normal.toolbar_image` `box`;
/// native-theme/src/next/presets/macos.rs, `toolbar_button_rest`, `toolbar_button_corners`),
/// not the Ghost's transparent one; kde-breeze, which states no rest fill, keeps the Ghost.
#[test]
fn a_toolbar_button_at_rest_is_the_stated_capsule() -> Result<(), String> {
    let capsule = |harness: &Harness<'_, App>, id: &str| {
        let Some(place) = harness.state().registry.places().get(id).copied() else {
            return false;
        };
        harness
            .output()
            .shapes
            .iter()
            .any(|clipped| match &clipped.shape {
                egui::Shape::Rect(r) => {
                    r.fill == egui::Color32::WHITE
                        && r.corner_radius == egui::CornerRadius::same(u8::MAX)
                        && place.contains_rect(r.rect)
                }
                _ => false,
            })
    };
    let ids = ["basic.icons.copy", "chrome.toolbar.command_palette"];
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "macos-tahoe");
    harness.run_steps(SETTLE);
    harness.remove_cursor();
    harness.run_steps(3);
    for id in ids {
        assert!(
            capsule(&harness, id),
            "macos-tahoe light: {id} at rest is not the white capsule"
        );
        assert!(
            every_record_notes(&harness, id, &[crate::demo::TOOL_REST_NOTE.0]),
            "macos-tahoe light: {id}'s record does not note its rest"
        );
    }
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    harness.run_steps(SETTLE);
    harness.remove_cursor();
    harness.run_steps(3);
    for id in ids {
        assert!(
            !capsule(&harness, id),
            "kde-breeze light: {id} at rest is filled, though the theme states no rest fill"
        );
        assert!(
            every_record_notes(&harness, id, &[crate::demo::GHOST_NOTE.0]),
            "kde-breeze light: {id}'s record does not note it Ghost"
        );
    }
    Ok(())
}

/// A toolbar button's icon is loaded in `toolbar.button_icon(<state>).color` where the theme
/// states it, in the state of its previous response: macos-monterey light's #676767 at rest and
/// #666666 hovered (os-probe run 45634379188, `controls-toolbar_states_sheet.toml`,
/// `aqua.{normal,hovered}.toolbar_image` `ink`; native-theme/src/next/presets/macos.rs,
/// `toolbar_icon`). Checked, in the colour a rule states for `CHECKED` in the state: macos-monterey
/// light's #4b4b4b at rest, #3b3b3b hovered and pressed, macos-tahoe's #ffffff in each (os-probe
/// run 45663039135, `controls-toolbar_states_sheet.toml`,
/// `aqua.{normal,hovered,pressed}.toolbar_image_on` `icon`; macos.rs, `toolbar_checked_icon`), so
/// macos-tahoe's checked icon is never loaded in its resting #4d4d4d, in any state. kde-breeze
/// states none, and the icon keeps the colour it is loaded in. So the Basic page's Copy, a
/// bundled Material icon, is painted from another texture under the pointer than at rest. The
/// Copy icon is a bundled Material one, so the test needs that set.
#[cfg(feature = "material-icons")]
#[test]
fn a_toolbar_buttons_icon_takes_the_stated_icon_colour() -> Result<(), String> {
    use crate::demo::tool_icon_color;
    use native_theme::next::{Preset, Theme, ThemeContext};
    let grey = |v: u8| Some(egui::Color32::from_rgb(v, v, v));
    let monterey = Theme::preset(Preset::MacosMonterey).resolve(&ThemeContext::light());
    let r = Some(&monterey);
    assert_eq!(tool_icon_color(r, (false, false), false), grey(0x67));
    assert_eq!(tool_icon_color(r, (true, false), false), grey(0x66));
    assert_eq!(tool_icon_color(r, (false, false), true), grey(0x4b));
    assert_eq!(tool_icon_color(r, (true, false), true), grey(0x3b));
    assert_eq!(tool_icon_color(r, (false, true), true), grey(0x3b));
    let tahoe = Theme::preset(Preset::MacosTahoe).resolve(&ThemeContext::light());
    assert_eq!(
        tool_icon_color(Some(&tahoe), (false, false), false),
        grey(0x4d)
    );
    for was in [(false, false), (true, false), (false, true)] {
        assert_eq!(
            tool_icon_color(Some(&tahoe), was, true),
            grey(0xff),
            "{was:?}"
        );
    }
    let breeze = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
    assert_eq!(tool_icon_color(Some(&breeze), (true, false), false), None);
    assert_eq!(tool_icon_color(None, (false, false), false), None);

    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "macos-monterey");
    harness.state_mut().settings.icon = IconSetChoice::Material;
    harness.run_steps(SETTLE);
    harness.remove_cursor();
    harness.run_steps(3);
    let id = "basic.icons.copy";
    let place = harness
        .state()
        .registry
        .places()
        .get(id)
        .copied()
        .ok_or("basic.icons.copy is not placed")?;
    let textures = |harness: &Harness<'_, App>| -> Vec<egui::TextureId> {
        harness
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(r) if place.contains_rect(r.rect) => {
                    r.brush.as_ref().map(|b| b.fill_texture_id)
                }
                _ => None,
            })
            .collect()
    };
    let rest = textures(&harness);
    harness.hover_at(place.center());
    harness.run_steps(3);
    let hovered = textures(&harness);
    assert!(
        rest.len() == 1 && hovered.len() == 1,
        "Copy's icon is not painted once: at rest {rest:?}, hovered {hovered:?}"
    );
    assert_ne!(
        rest, hovered,
        "Copy's icon is painted from one texture at rest and hovered, though the theme states \
         another icon colour hovered"
    );
    Ok(())
}

/// The Basic page's Edit switch under macos-tahoe light, a bundled Material icon: checked, its
/// icon is not the texture it is painted from unchecked at rest, loaded in the resting
/// `toolbar.button_icon(NONE).color` #4d4d4d: it takes the colour the rule for `CHECKED`
/// states, #ffffff (os-probe run 45663039135; macos.rs, `toolbar_checked_icon`). The icon is a
/// bundled Material one, so the test needs that set.
#[cfg(feature = "material-icons")]
#[test]
fn the_checked_edit_icon_is_not_loaded_in_the_resting_icon_colour() -> Result<(), String> {
    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "macos-tahoe");
    harness.state_mut().settings.icon = IconSetChoice::Material;
    harness.run_steps(SETTLE);
    harness.remove_cursor();
    harness.run_steps(3);
    let textures = |harness: &Harness<'_, App>| -> Result<Vec<egui::TextureId>, String> {
        let place = harness
            .state()
            .registry
            .places()
            .get("basic.icons.edit")
            .copied()
            .ok_or("basic.icons.edit is not placed")?;
        Ok(harness
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(r) if place.contains_rect(r.rect) => {
                    r.brush.as_ref().map(|b| b.fill_texture_id)
                }
                _ => None,
            })
            .collect())
    };
    assert!(harness.state().demo_state.basic_edit, "Edit starts checked");
    let checked = textures(&harness)?;
    harness.state_mut().demo_state.basic_edit = false;
    harness.run_steps(3);
    let rest = textures(&harness)?;
    assert!(
        checked.len() == 1 && rest.len() == 1,
        "Edit's icon is not painted once: checked {checked:?}, at rest {rest:?}"
    );
    assert_ne!(
        checked, rest,
        "the checked Edit icon is loaded in the resting icon colour"
    );
    Ok(())
}

/// The Basic page's icon buttons on the 0.8.0 model, under every bundled preset in light: Copy,
/// Paste and Delete stand the toolbar scope's item gap apart, `toolbar.item_gap` where the theme
/// states it, else `layout.widget_gap`, which the connector's base style carries, and so no
/// gap of the showcase's own is needed (the 0.7.2 `demo::toolbar_gap`). Copy's label, in place
/// of an icon the chosen set lacks, sits each stated side of `button.body(NONE).padding` in from
/// the frame's edge as egui draws that padding ([`egui_button_inset`]): under macos-monterey and
/// macos-tahoe, whose toolbar buttons keep a half-point border at rest, half a point short of it,
/// egui's own whole-point margin, not a padding the showcase reads wrongly. Under kde-breeze,
/// which states `toolbar.button(HOVER)`'s outline, a hovered Copy is lined in it.
#[test]
fn the_basic_icon_buttons_are_the_toolbars_gap_apart_and_outlined_on_hover() -> Result<(), String> {
    use native_theme::next::{Preset, WidgetState};
    use native_theme_egui::next::{state_stroke_color_of, stroke_width_of};
    let mut rounded_away = false;
    for preset in Preset::ALL.iter().map(|p| p.as_str()) {
        let mut harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        harness.state_mut().settings.icon = IconSetChoice::Freedesktop("no-such-icon-theme".into());
        harness.run_steps(SETTLE);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .cloned()
            .ok_or_else(|| format!("{preset}: no 0.8.0 theme installed"))?;
        let gap =
            r.toolbar.item_gap.or(r.layout.widget_gap).ok_or_else(|| {
                format!("{preset} states neither toolbar.item_gap nor widget_gap")
            })?;
        // Copy in view where the page lays it out below the fold (material-3's): egui paints
        // only what the window shows (`egui-0.37.0/src/widgets/button.rs:369-374`).
        let copy_node = harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.info.element.as_deref() == Some("basic.icons.copy"))
            .map(|r| r.id.accesskit_id())
            .ok_or_else(|| format!("{preset}: Copy is not recorded"))?;
        harness
            .query_all(By::new().predicate(move |n| n.locate().0 == copy_node))
            .next()
            .ok_or_else(|| format!("{preset}: Copy has no AccessKit node"))?
            .scroll_to_me();
        // The scroll lands on the third pass after the request (as the page scroll tests).
        harness.run_steps(3);
        let places = harness.state().registry.places();
        let [copy, paste, delete] = ["copy", "paste", "delete"].map(|id| {
            places
                .get(&format!("basic.icons.{id}"))
                .copied()
                .unwrap_or(egui::Rect::NOTHING)
        });
        assert!(
            copy.is_positive() && paste.is_positive() && delete.is_positive(),
            "{preset}: an icon button is not placed"
        );
        if near(copy.top(), paste.top()) {
            assert!(
                near(paste.left() - copy.right(), gap),
                "{preset}: Copy and Paste {} apart, the toolbar's gap is {gap}",
                paste.left() - copy.right()
            );
        }
        let [top, _, _, left] =
            native_theme_egui::next::padding_of(&r.button.body(WidgetState::NONE));
        let shapes = &harness.output().shapes;
        let frame = shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(s) if near_rect(s.rect, copy) => Some(s.stroke.width),
                _ => None,
            })
            .ok_or_else(|| {
                format!("{preset}: Copy's frame is not painted on its place {copy:?}")
            })?;
        let label = shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(s) if s.galley.job.text == "Copy" && copy.contains(s.pos) => {
                    Some(s.pos)
                }
                _ => None,
            })
            .ok_or_else(|| format!("{preset}: Copy's label is not painted in its place"))?;
        for (side, stated, inset) in [
            ("left", left, label.x - copy.left()),
            ("top", top, label.y - copy.top()),
        ] {
            if let Some(padding) = stated {
                let drawn = egui_button_inset(padding, frame);
                rounded_away |= !near(drawn, padding);
                assert!(
                    near(inset, drawn),
                    "{preset}: Copy's label sits {inset} in from its {side} edge, egui draws a \
                     padding of {padding} inside a {frame} pt frame {drawn} in"
                );
            }
        }
        let hovered = r.toolbar.button(WidgetState::HOVER);
        if preset == "kde-breeze" {
            let pen = stroke_width_of(&hovered)
                .zip(state_stroke_color_of(&hovered))
                .map(|(width, colour)| egui::Stroke::new(width, colour))
                .ok_or("kde-breeze no longer states the toolbar's hover outline")?;
            harness.hover_at(copy.center());
            harness.run_steps(3);
            let lined = harness
                .output()
                .shapes
                .iter()
                .any(|clipped| match &clipped.shape {
                    egui::Shape::Rect(s) => {
                        s.stroke == pen && copy.expand(1.0).contains_rect(s.rect)
                    }
                    _ => false,
                });
            assert!(lined, "kde-breeze: a hovered Copy is not lined in {pen:?}");
        }
    }
    assert!(
        rounded_away,
        "no preset's icon button has a border egui's whole-point margin rounds away from its \
         padding (macos-monterey's half point), so the inset's rule is not shown"
    );
    Ok(())
}

/// The widget visuals of a `role` scope, `RoleVariant::Normal`, in light, of the atlas
/// installed in `harness`, as `demo::scoped` applies the scope: in a context of their own, so
/// the harness's passes stay as they are.
fn scope_widgets(
    harness: &Harness<'_, App>,
    role: native_theme_egui::Role,
) -> Result<egui::style::Widgets, String> {
    scope_style(harness, role).map(|style| style.visuals.widgets)
}

/// The style of `role`'s scope as `demo::scoped` sets it, in light, under the atlas `harness`
/// installed, in a context of its own (as [`scope_widgets`]).
fn scope_style(
    harness: &Harness<'_, App>,
    role: native_theme_egui::Role,
) -> Result<egui::Style, String> {
    let atlas =
        native_theme_egui::ThemeAtlas::from_ctx(&harness.ctx).ok_or("no atlas installed")?;
    let ctx = egui::Context::default();
    atlas.install(&ctx);
    ctx.set_theme(egui::Theme::Light);
    let mut reg = crate::demo::Registry::default();
    let mut style = None;
    let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
        crate::demo::scoped(&mut reg, ui, role, RoleVariant::Normal, "scope", |ui| {
            style = Some(ui.style().as_ref().clone());
            ui.label("")
        });
    });
    // The font atlas's upload, which no renderer takes here (`epaint/src/textures.rs:337`).
    out.textures_delta.clear();
    style.ok_or_else(|| format!("the {role:?} scope ran no pass"))
}

/// The chrome toolbar's tool buttons are outlined hovered and pressed in the «toolbar» scope's
/// hovered and pressed button's stroke, which `demo::next_tool_button` leaves to the scope: where
/// a rule states `toolbar.button(HOVER)` and `toolbar.button(PRESSED)`'s stroke, its width
/// (kde-breeze light patched to twice its stated hover width, which the unpatched
/// `toolbar.button(HOVER)` width is not), and where none does, the scope's own hovered
/// and pressed button's (windows-11 light states neither, and its scope's own button is
/// outlined, spec §10 rule 0). The Command Palette
/// button, hovered and then pressed, is painted in that pen.
#[test]
fn a_chrome_tool_buttons_outline_is_the_toolbar_scopes() -> Result<(), String> {
    use native_theme::next::{PartPatch, PartStated, Stroke, ThemePatch, ToolbarRule, WidgetState};
    use native_theme_egui::next::{state_stroke_color_of, stroke_width_of};
    const ID: &str = "chrome.toolbar.command_palette";
    /// Whether the button, hovered and then pressed, is painted in `hovered` and `pressed`.
    fn lined(
        harness: &mut Harness<'static, App>,
        [hovered, pressed]: [egui::Stroke; 2],
    ) -> [bool; 2] {
        let place = place_of(harness, ID);
        let painted = |harness: &Harness<'static, App>, pen: egui::Stroke| {
            harness
                .output()
                .shapes
                .iter()
                .any(|clipped| match &clipped.shape {
                    egui::Shape::Rect(s) => {
                        s.stroke == pen && place.expand(1.0).contains_rect(s.rect)
                    }
                    _ => false,
                })
        };
        harness.hover_at(place.center());
        harness.run_steps(3);
        let hover = painted(harness, hovered);
        harness.drag_at(place.center());
        harness.run_steps(3);
        let press = painted(harness, pressed);
        [hover, press]
    }
    let mut old = None;
    let (mut harness, r) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", |r| {
        old = stroke_width_of(&r.toolbar.button(WidgetState::HOVER));
        let width = 2.0 * old?;
        let pen = PartPatch::new().stroke(Stroke::uniform(width, r.colors.window.border?));
        Some(
            ThemePatch::new()
                .toolbar(
                    ToolbarRule::new()
                        .states(WidgetState::HOVER)
                        .button(pen.clone()),
                )
                .toolbar(ToolbarRule::new().states(WidgetState::PRESSED).button(pen)),
        )
    })?;
    let old = old.ok_or("the unpatched kde-breeze states no toolbar.button(HOVER) width")?;
    let pens = [WidgetState::HOVER, WidgetState::PRESSED].map(|state| {
        let part = r.toolbar.button(state);
        stroke_width_of(&part)
            .zip(state_stroke_color_of(&part))
            .map(|(width, colour)| egui::Stroke::new(width, colour))
    });
    let [Some(hovered), Some(pressed)] = pens else {
        return Err(format!(
            "the patched kde-breeze states no toolbar button pen: {pens:?}"
        ));
    };
    assert!(
        !near(hovered.width, old) && !near(pressed.width, old),
        "the patched width {} is the unpatched toolbar.button(HOVER) width {old}, so the patch \
         proves nothing",
        hovered.width
    );
    assert_eq!(
        lined(&mut harness, [hovered, pressed]),
        [true; 2],
        "patched kde-breeze: [hovered, pressed] Command Palette lined in {hovered:?}, {pressed:?}"
    );

    let mut harness = open_page_under(Page::Basic, egui::Theme::Light, "windows-11");
    let r = harness
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("windows-11: no 0.8.0 theme installed")?;
    for state in [WidgetState::HOVER, WidgetState::PRESSED] {
        assert!(
            !r.toolbar
                .button(state)
                .from_state
                .contains(PartStated::STROKE),
            "windows-11 now states toolbar.button({state:?})'s stroke"
        );
    }
    let own = scope_widgets(&harness, native_theme_egui::Role::Toolbar)?;
    let (hovered, pressed) = (own.hovered.bg_stroke, own.active.bg_stroke);
    assert!(
        hovered.width > 0.0 && pressed.width > 0.0,
        "windows-11's toolbar scope draws no outline of its own, so this part proves nothing"
    );
    assert_eq!(
        lined(&mut harness, [hovered, pressed]),
        [true; 2],
        "windows-11: [hovered, pressed] Command Palette lined in the scope's own {hovered:?}, \
         {pressed:?}"
    );
    Ok(())
}

/// The flat buttons — the Buttons page's frameless button and the status bar's side-panel
/// toggle, a Ghost button — are outlined hovered and focused in the flat button's stated
/// outlines (`demo::flat`, `next::flat_button`): kde-breeze light patched with a rule for the
/// flat variant that states `HOVER`'s and `FOCUS_VISIBLE`'s stroke, which the push button's
/// `HOVER` and `FOCUS_VISIBLE` strokes are not. Each button, hovered and then focused, is
/// painted in that pen.
#[test]
fn a_flat_buttons_stated_hover_and_focus_outlines_are_drawn() -> Result<(), String> {
    use native_theme::next::{
        ButtonQuery, ButtonRule, ButtonVariant, PartPatch, Stroke, ThemePatch, WidgetState,
    };
    use native_theme_egui::next::{state_stroke_color_of, stroke_width_of};
    /// Whether `place`, hovered and then focused by the button labelled `label`, is painted in
    /// `hovered` and `focused`.
    fn lined(
        harness: &mut Harness<'static, App>,
        place: egui::Rect,
        label: &str,
        [hovered, focused]: [egui::Stroke; 2],
    ) -> Result<[bool; 2], String> {
        let painted = |harness: &Harness<'static, App>, pen: egui::Stroke| {
            harness
                .output()
                .shapes
                .iter()
                .any(|clipped| match &clipped.shape {
                    egui::Shape::Rect(s) => {
                        s.stroke == pen && place.expand(1.0).contains_rect(s.rect)
                    }
                    _ => false,
                })
        };
        harness.hover_at(place.center());
        harness.run_steps(3);
        let hover = painted(harness, hovered);
        harness
            .query_all_by_label(label)
            .next()
            .ok_or_else(|| format!("no node labelled {label}"))?
            .focus();
        harness.run_steps(3);
        let focus = painted(harness, focused);
        Ok([hover, focus])
    }
    let magenta = native_theme::next::Color::new(1.0, 0.0, 1.0, 1.0);
    let green = native_theme::next::Color::new(0.0, 1.0, 0.0, 1.0);
    let (mut harness, r) = open_patched(Page::Buttons, egui::Theme::Light, "kde-breeze", |_| {
        let flat = |state: WidgetState, width: f32, colour| {
            ButtonRule::new()
                .variant(ButtonVariant::Flat)
                .states(state)
                .body(PartPatch::new().stroke(Stroke::uniform(width, colour)))
        };
        Some(
            ThemePatch::new()
                .button(flat(WidgetState::HOVER, 3.0, magenta))
                .button(flat(WidgetState::FOCUS_VISIBLE, 4.0, green)),
        )
    })?;
    let pen = |part: &native_theme::next::PartStyle| {
        stroke_width_of(part)
            .zip(state_stroke_color_of(part))
            .map(|(width, colour)| egui::Stroke::new(width, colour))
    };
    let at = |state: WidgetState| ButtonQuery::flat().with_state(state);
    let pens = [WidgetState::HOVER, WidgetState::FOCUS_VISIBLE]
        .map(|state| pen(&r.button.body(at(state))));
    let [Some(hovered), Some(focused)] = pens else {
        return Err(format!(
            "the patched kde-breeze states no flat button pen: {pens:?}"
        ));
    };
    for state in [WidgetState::HOVER, WidgetState::FOCUS_VISIBLE] {
        let push = pen(&r.button.body(state));
        assert!(
            push != Some(hovered) && push != Some(focused),
            "the push button's {state:?} pen {push:?} is the flat one, so the patch proves nothing"
        );
    }
    let frameless = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|record| record.info.kind == "button (frameless)")
        .map(|record| record.rect)
        .ok_or("no record of the frameless button")?;
    assert_eq!(
        lined(&mut harness, frameless, "Frameless", [hovered, focused])?,
        [true; 2],
        "[hovered, focused] frameless button lined in {hovered:?}, {focused:?}"
    );
    let toggle = place_of(&harness, "chrome.status_bar.toggle");
    assert_eq!(
        lined(
            &mut harness,
            toggle,
            "Toggle Side Panel",
            [hovered, focused]
        )?,
        [true; 2],
        "[hovered, focused] side-panel toggle lined in {hovered:?}, {focused:?}"
    );
    Ok(())
}

/// How far egui 0.37.0 puts a framed button's content in from the frame's edge, for a
/// `Spacing::button_padding` side of `padding` and a resting border `stroke` wide: the frame's
/// inner margin is whole points (`Margin`'s sides are `i8`, `From<Vec2>` rounds,
/// `epaint-0.37.0/src/margin.rs:108-120`), the padding rounded, less the border's width rounded
/// (`Frame::apply_stroke_and_expansion_without_layout_shift`,
/// `egui-0.37.0/src/containers/frame.rs:313-323`, called by the button's style,
/// `src/theme/default_style.rs:54-72`; Rust rounds half away from zero), and the border's own
/// width outside it (`Frame::total_margin`, `frame.rs:362-366`): a half-point border so takes a
/// whole point from the padding and gives half of it back.
fn egui_button_inset(padding: f32, stroke: f32) -> f32 {
    padding.round() + (-stroke).round() + stroke
}

/// The colour the label `text` is painted in inside the place `id` (the first such galley's first
/// section: the shape's override, else the section's colour, its placeholder the shape's fallback).
fn painted_label_colour(harness: &Harness<'_, App>, text: &str, id: &str) -> Option<egui::Color32> {
    let place = harness.state().registry.places().get(id).copied()?;
    harness
        .output()
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            egui::Shape::Text(shape)
                if shape.galley.job.text == text && place.contains(shape.pos) =>
            {
                let section = shape.galley.job.sections.first()?.format.color;
                Some(shape.override_text_color.unwrap_or(
                    if section == egui::Color32::PLACEHOLDER {
                        shape.fallback_color
                    } else {
                        section
                    },
                ))
            }
            _ => None,
        })
}

// --- The chrome's icons from a fixture icon theme --------------------------------------------

/// M7-15: the side-panel toggle's freedesktop icon loads from the icon theme the chrome chose
/// under whichever name that theme has, on any desktop: Adwaita's `sidebar-show-symbolic`
/// (`gtk-names`) and Breeze's `sidebar-expand-left` (`breeze-names`), on Sway and on GNOME. The
/// themes are fixtures in a child process.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
#[test]
fn the_panel_icon_loads_from_a_theme_of_either_convention() -> std::io::Result<()> {
    const NAME: &str = "tests::the_panel_icon_loads_from_a_theme_of_either_convention";
    if native_theme_test_fixtures::in_child() {
        let ctx = egui::Context::default();
        let mut loaded = Vec::new();
        let mut cache = crate::demo::IconCache::default();
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            for theme in ["gtk-names", "breeze-names"] {
                let image = crate::demo::named_image(
                    &mut cache,
                    ui,
                    crate::demo::ChromeIcon::PanelLeft,
                    native_theme::icons::IconSet::Freedesktop,
                    Some(theme),
                    16.0,
                );
                loaded.push((theme, image.is_some()));
            }
        });
        // The font atlas's upload, which no renderer takes here (`epaint/src/textures.rs:337`).
        out.textures_delta.clear();
        assert_eq!(
            loaded,
            [("gtk-names", true), ("breeze-names", true)],
            "on {:?}",
            std::env::var("XDG_CURRENT_DESKTOP")
        );
        // Its alt text names the icon, whichever name each theme holds it under.
        let mut cache = crate::demo::IconCache::default();
        let harness = Harness::builder()
            .with_context_setup(emoji_fallbacks)
            .build_ui(|ui| {
                for theme in ["gtk-names", "breeze-names"] {
                    if let Some(image) = crate::demo::named_image(
                        &mut cache,
                        ui,
                        crate::demo::ChromeIcon::PanelLeft,
                        native_theme::icons::IconSet::Freedesktop,
                        Some(theme),
                        16.0,
                    ) {
                        ui.add(image);
                    }
                }
            });
        assert_eq!(harness.get_all_by_label("panel-left").count(), 2);
        return Ok(());
    }
    const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="currentColor" d="M2 2h12v12H2z"/></svg>"#;
    for desktop in ["sway", "GNOME"] {
        let child = native_theme_test_fixtures::Child {
            desktop,
            icon_theme: None,
        };
        native_theme_test_fixtures::run(NAME, child, |icons| {
            native_theme_test_fixtures::write_scalable_theme(
                icons,
                "gtk-names",
                &[("sidebar-show-symbolic", SVG)],
            )?;
            native_theme_test_fixtures::write_scalable_theme(
                icons,
                "breeze-names",
                &[("sidebar-expand-left", SVG)],
            )
        })?;
    }
    Ok(())
}

/// The loads and frame builds of the showcase's icon cache from the `from`th on.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
fn icon_loads_since(harness: &Harness<'_, App>, from: usize) -> Vec<crate::demo::IconLoad> {
    harness
        .state()
        .icon_cache
        .loads
        .get(from..)
        .unwrap_or_default()
        .to_vec()
}

/// How many times each load or build of `loads` was made.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
fn icon_load_counts(
    loads: &[crate::demo::IconLoad],
) -> std::collections::HashMap<crate::demo::IconLoad, usize> {
    let mut counts = std::collections::HashMap::new();
    for load in loads {
        *counts.entry(load.clone()).or_insert(0) += 1;
    }
    counts
}

/// M9-104: the Icons page loads the chosen icon theme's loading indicator once, not in every
/// pass, and builds each frame's image once per frame and scheme. The passes of opening and
/// eight steps under a fixture freedesktop theme whose indicator is a three-frame sprite sheet
/// load it once (before the cache, once a pass: 14) and build no frame twice; an install and
/// eight steps load it once more, since the install forgets what the cache keeps; a mode switch
/// and eight steps load no indicator and build only the new scheme's frames. The theme is a
/// fixture in a child process.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
#[test]
fn the_icons_page_loads_its_indicator_once() -> std::io::Result<()> {
    use crate::demo::IconLoad;
    const NAME: &str = "tests::the_icons_page_loads_its_indicator_once";
    if native_theme_test_fixtures::in_child() {
        let mut harness = open(
            egui::Theme::Light,
            cli(&[
                ("--theme", TEST_PRESET),
                ("--variant", "light"),
                ("--tab", "icons"),
                ("--icon-set", "indicator"),
            ]),
        );
        let ctx = harness.ctx.clone();
        let mut from = 0;
        for (phase, scheme) in [
            ("opening and eight steps", egui::Theme::Light),
            ("an install", egui::Theme::Light),
            ("a mode switch", egui::Theme::Dark),
        ] {
            match phase {
                "an install" => harness.state_mut().install(&ctx),
                "a mode switch" => harness.state_mut().set_mode(ModeChoice::Dark, &ctx),
                _ => {}
            }
            harness.run_steps(8);
            let loads = icon_loads_since(&harness, from);
            from += loads.len();
            let indicators = loads
                .iter()
                .filter(|load| matches!(load, IconLoad::Indicator(..)))
                .collect::<Vec<_>>();
            let expected = usize::from(phase != "a mode switch");
            assert_eq!(indicators.len(), expected, "{phase}: {indicators:?}");
            let frames = loads
                .iter()
                .filter(|load| matches!(load, IconLoad::Frame(..)))
                .cloned()
                .collect::<Vec<_>>();
            assert!(!frames.is_empty(), "{phase}: no frame was built");
            let counts = icon_load_counts(&frames);
            assert!(counts.values().all(|n| *n == 1), "{phase}: {counts:?}");
            assert!(
                frames
                    .iter()
                    .all(|frame| matches!(frame, IconLoad::Frame(_, s, _) if *s == scheme)),
                "{phase}: {frames:?}"
            );
        }
        return Ok(());
    }
    // Three 16 × 16 frames, one above the other (native-theme `parse_sprite_sheet`).
    const SHEET: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 48"><path fill="currentColor" d="M2 2h12v12H2zM2 18h12v12H2zM2 34h12v12H2z"/></svg>"#;
    let child = native_theme_test_fixtures::Child {
        desktop: "sway",
        icon_theme: None,
    };
    native_theme_test_fixtures::run(NAME, child, |icons| {
        native_theme_test_fixtures::write_indicator_theme(icons, "indicator", SHEET)
    })
}

/// docs/todo.md's `named_image` item (M9c): the chrome's named freedesktop icons — the
/// toolbar's Command Palette and Reload System Theme, the status bar's side-panel toggle — are
/// read from the icon theme once per key, not in every pass. The passes of opening and eight
/// steps load each key once (before the cache, once a pass: 14); an install and eight steps load
/// each once more, since the install forgets what the cache keeps; a mode switch and eight steps
/// load only the new text colour's keys, once each. The theme is a fixture in a child process.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
#[test]
fn the_chromes_named_icons_load_once_per_key() -> std::io::Result<()> {
    use crate::demo::IconLoad;
    const NAME: &str = "tests::the_chromes_named_icons_load_once_per_key";
    if native_theme_test_fixtures::in_child() {
        let mut harness = open(
            egui::Theme::Light,
            cli(&[
                ("--theme", TEST_PRESET),
                ("--variant", "light"),
                ("--icon-set", "named"),
            ]),
        );
        let ctx = harness.ctx.clone();
        let mut from = 0;
        let mut first = Vec::new();
        for phase in ["opening and eight steps", "an install", "a mode switch"] {
            match phase {
                "an install" => harness.state_mut().install(&ctx),
                "a mode switch" => harness.state_mut().set_mode(ModeChoice::Dark, &ctx),
                _ => {}
            }
            harness.run_steps(8);
            let loads = icon_loads_since(&harness, from);
            from += loads.len();
            let named = loads
                .iter()
                .filter(|load| matches!(load, IconLoad::Named(..)))
                .cloned()
                .collect::<Vec<_>>();
            let counts = icon_load_counts(&named);
            assert_eq!(counts.len(), 3, "{phase}: {counts:?}");
            assert!(counts.values().all(|n| *n == 1), "{phase}: {counts:?}");
            match phase {
                "opening and eight steps" => first = named,
                "an install" => assert_eq!(icon_load_counts(&first), counts, "{phase}"),
                _ => assert!(
                    named.iter().all(|load| !first.contains(load)),
                    "{phase}: {named:?} against {first:?}"
                ),
            }
        }
        return Ok(());
    }
    const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="currentColor" d="M2 2h12v12H2z"/></svg>"#;
    let child = native_theme_test_fixtures::Child {
        desktop: "sway",
        icon_theme: None,
    };
    native_theme_test_fixtures::run(NAME, child, |icons| {
        native_theme_test_fixtures::write_scalable_theme(
            icons,
            "named",
            &[
                ("utilities-terminal", SVG),
                ("object-rotate-right", SVG),
                ("sidebar-show", SVG),
            ],
        )
    })
}

/// A named freedesktop icon drawn at two sizes that `IconKey::size` rounds to one key, 16 and
/// 16.4 points, is fitted to each: the cache keeps the image as loaded and each draw fits its
/// clone, as the indicator's frames are. A source of one point square makes `calc_size` the
/// fitted size itself (`egui/src/widgets/image.rs`, `ImageFit::Exact`). The theme is a fixture
/// in a child process.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
#[test]
fn a_named_icon_takes_each_size_it_is_drawn_at() -> std::io::Result<()> {
    const NAME: &str = "tests::a_named_icon_takes_each_size_it_is_drawn_at";
    if native_theme_test_fixtures::in_child() {
        let ctx = egui::Context::default();
        let mut cache = crate::demo::IconCache::default();
        let mut drawn = Vec::new();
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            for size in [16.0, 16.4] {
                let image = crate::demo::named_image(
                    &mut cache,
                    ui,
                    crate::demo::ChromeIcon::PanelLeft,
                    native_theme::icons::IconSet::Freedesktop,
                    Some("named"),
                    size,
                );
                let fitted = image.map(|image| {
                    image.calc_size(egui::Vec2::INFINITY, Some(egui::Vec2::splat(1.0)))
                });
                drawn.push((size, fitted));
            }
        });
        // The font atlas's upload, which no renderer takes here (`epaint/src/textures.rs:337`).
        out.textures_delta.clear();
        for (size, fitted) in drawn {
            assert_eq!(fitted, Some(egui::Vec2::splat(size)), "drawn at {size}");
        }
        return Ok(());
    }
    const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="currentColor" d="M2 2h12v12H2z"/></svg>"#;
    let child = native_theme_test_fixtures::Child {
        desktop: "sway",
        icon_theme: None,
    };
    native_theme_test_fixtures::run(NAME, child, |icons| {
        native_theme_test_fixtures::write_scalable_theme(icons, "named", &[("sidebar-show", SVG)])
    })
}

/// `App::icons_chosen`: the cache is forgotten where the icon choice changes, and only there.
/// Under kde-breeze with fixture `breeze` and `breeze-dark` themes, the choice follows the
/// scheme, so a mode switch changes it: Light, Dark and Light again load the `breeze` indicator
/// twice (once, were the switch to keep the cache). A pick in the side panel's Icon theme
/// drop-down changes it too: Material, then `breeze`, whose indicator the cache would still
/// hold, loads it a third time. The themes are fixtures in a child process.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
#[test]
fn the_icon_cache_is_forgotten_where_the_icon_choice_changes() -> std::io::Result<()> {
    use crate::demo::IconLoad;
    use native_theme::icons::IconSet;
    const NAME: &str = "tests::the_icon_cache_is_forgotten_where_the_icon_choice_changes";
    if native_theme_test_fixtures::in_child() {
        let mut harness = open(
            egui::Theme::Light,
            cli(&[
                ("--theme", "kde-breeze"),
                ("--variant", "light"),
                ("--tab", "icons"),
            ]),
        );
        assert_eq!(
            Ok(harness.state().atlas.name()),
            preset_name("kde-breeze").as_deref(),
            "the child runs under kde-breeze"
        );
        let ctx = harness.ctx.clone();
        let loaded = |harness: &Harness<'_, App>, theme: &str| {
            let load = IconLoad::Indicator(IconSet::Freedesktop, Some(theme.to_string()));
            icon_loads_since(harness, 0)
                .iter()
                .filter(|l| **l == load)
                .count()
        };
        harness.run_steps(SETTLE);
        assert_eq!(
            harness.state().settings.icon,
            IconSetChoice::Default("breeze".to_string()),
            "the choice follows kde-breeze's light icon theme"
        );
        assert_eq!(loaded(&harness, "breeze"), 1, "opening");
        harness.state_mut().set_mode(ModeChoice::Dark, &ctx);
        harness.run_steps(SETTLE);
        assert_eq!(loaded(&harness, "breeze-dark"), 1, "the switch to Dark");
        harness.state_mut().set_mode(ModeChoice::Light, &ctx);
        harness.run_steps(SETTLE);
        assert_eq!(loaded(&harness, "breeze"), 2, "the switch back to Light");

        for row in ["Material (bundled)", "breeze"] {
            harness
                .get_by_role_and_label(Role::ComboBox, "Icon theme")
                .click();
            harness.run_steps(SETTLE);
            harness.get_by_label(row).scroll_to_me();
            harness.run_steps(SETTLE);
            harness.get_by_label(row).click();
            harness.run_steps(SETTLE);
        }
        assert_eq!(
            harness.state().settings.icon,
            IconSetChoice::Freedesktop("breeze".to_string())
        );
        assert_eq!(
            loaded(&harness, "breeze"),
            3,
            "the picks of Material and breeze"
        );
        return Ok(());
    }
    const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="currentColor" d="M2 2h12v12H2z"/></svg>"#;
    // On KDE, which kde-breeze is offered on (`native-theme/src/presets.rs`, `linux-kde`).
    let child = native_theme_test_fixtures::Child {
        desktop: "KDE",
        icon_theme: None,
    };
    native_theme_test_fixtures::run(NAME, child, |icons| {
        native_theme_test_fixtures::write_indicator_theme(icons, "breeze", SVG)?;
        native_theme_test_fixtures::write_indicator_theme(icons, "breeze-dark", SVG)
    })
}

/// The Theme row offers `default`, then this platform's presets the 0.8.0 model has a `Preset`
/// of, and no community preset of the 0.7.2 model, whose colours are palettes in the 0.8.0 one:
/// every preset row is one of `Preset::ALL`, the list the gpui showcase's parity lists are
/// asserted against too.
#[test]
fn the_theme_row_offers_the_08_models_presets_of_this_platform() {
    use native_theme::next::Preset;
    let harness = open_default();
    let rows = harness.state().theme_rows();
    assert!(
        matches!(rows.first(), Some((ThemeChoice::Default, _))),
        "the first row is default: {rows:?}"
    );
    let offered: BTreeSet<&str> = rows
        .iter()
        .filter_map(|(choice, _)| match choice {
            ThemeChoice::Preset(key) => Some(key.as_str()),
            ThemeChoice::Default => None,
        })
        .collect();
    let here: BTreeSet<&str> = native_theme::theme::Theme::list_presets_for_platform()
        .iter()
        .map(|info| info.key)
        .filter(|key| Preset::from_name(key).is_some())
        .collect();
    assert_eq!(offered, here);
    assert!(offered.contains(TEST_PRESET));
    for key in &offered {
        assert!(
            Preset::ALL.iter().any(|preset| preset.as_str() == *key),
            "{key} is no preset of the 0.8.0 model"
        );
    }
    for info in native_theme::theme::Theme::list_presets() {
        if Preset::from_name(info.key).is_none() {
            assert!(!offered.contains(info.key), "{} is offered", info.key);
        }
    }
}

/// `--palette` lays a bundled palette over the preset in the 0.8.0 model alone: both scheme
/// slots' installed 0.8.0 theme take the palette's name, its scheme and its window colour, read
/// from `Palette::from_name`, not a literal, which the unpatched 0.8.0 preset resolved alone in
/// the same context does not have; the status bar names both, as the gpui showcase's does. The Palette row's
/// `none` brings back the preset's own colours in each scheme.
#[test]
fn a_palette_is_laid_over_the_preset_in_the_08_model_only() -> Result<(), String> {
    let dracula = native_theme::next::Palette::from_name("dracula").ok_or("dracula is bundled")?;
    assert_eq!(
        dracula.scheme,
        native_theme::next::Scheme::Dark,
        "dracula is not dark, so locking the light slot to it proves nothing"
    );
    let mut harness = open(
        egui::Theme::Light,
        cli(&[("--theme", TEST_PRESET), ("--palette", "dracula")]),
    );
    harness.run_steps(SETTLE);
    {
        let app = harness.state();
        assert_eq!(app.settings.palette, Some("dracula"));
        assert_eq!(app.theme_error, None);
        let preset = native_theme::next::Theme::preset(
            native_theme::next::Preset::from_name(TEST_PRESET).ok_or("no 0.8.0 preset")?,
        );
        for theme in [egui::Theme::Light, egui::Theme::Dark] {
            let next = app
                .atlas
                .next_resolved(theme)
                .ok_or("no 0.8.0 theme installed")?;
            assert_eq!(next.name, dracula.name, "{theme:?}");
            assert!(next.is_dark(), "{theme:?}: dracula did not lock the scheme");
            assert_eq!(
                next.colors.window.background,
                Some(dracula.window_bg),
                "{theme:?}: the window background is not dracula's"
            );
            let alone = preset.resolve(&next.context);
            assert!(
                alone.is_dark(),
                "{theme:?}: the context is not the palette's scheme"
            );
            assert_ne!(
                alone.colors.window.background, next.colors.window.background,
                "{theme:?}: the preset alone has the palette's window background"
            );
        }
        let status = crate::chrome::environment(app, &harness.ctx).join(" · ");
        assert!(
            status.contains(&format!("{TEST_PRESET} + dracula")),
            "the status bar does not name the palette: {status}"
        );
    }
    let ctx = harness.ctx.clone();
    harness.state_mut().choose_palette(None, &ctx);
    harness.run_steps(SETTLE);
    let app = harness.state();
    assert_eq!(app.theme_error, None);
    for (theme, dark) in [(egui::Theme::Light, false), (egui::Theme::Dark, true)] {
        let next = app
            .atlas
            .next_resolved(theme)
            .ok_or("no 0.8.0 theme installed")?;
        assert_ne!(next.name, dracula.name, "{theme:?}");
        assert_eq!(next.is_dark(), dark, "{theme:?}");
    }
    Ok(())
}

/// High contrast shows what the user asked for: turned on under [`TEST_PRESET`], which defines
/// no High level, the preferences the install asked for hold it, the installed 0.8.0 theme is
/// resolved at Normal, and why is kept for the Preferences dialog's row
/// (`ContrastHeld::sentence`, docs/v0.8.0-spec.md §6).
#[test]
fn high_contrast_shows_the_request_and_why_the_theme_holds_it_back() -> Result<(), String> {
    use native_theme::next::Contrast;
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let mut prefs = harness.state().atlas.asked().clone();
    prefs.high_contrast = true;
    harness.state_mut().settings.prefs = Some(prefs);
    let ctx = harness.ctx.clone();
    // The light slot read below is the one drawn (`open_page_under`'s mode).
    harness.state_mut().set_mode(ModeChoice::Light, &ctx);
    harness.state_mut().install(&ctx);
    harness.run_steps(SETTLE);
    assert_eq!(harness.ctx.theme(), egui::Theme::Light);
    let atlas = &harness.state().atlas;
    assert!(atlas.asked().high_contrast);
    let next = atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme installed")?;
    assert_eq!(next.context.contrast, Contrast::Normal);
    let held = atlas
        .contrast_held(egui::Theme::Light)
        .ok_or("the held contrast is not kept")?;
    assert_eq!(
        (held.asked, held.used, held.by_palette),
        (Contrast::High, Contrast::Normal, false)
    );
    assert!(
        held.sentence()
            .contains("resolved at Normal contrast, not the High asked for"),
        "{}",
        held.sentence()
    );
    Ok(())
}

/// The showcase starts under `default`, the desktop's own theme as the 0.8.0 model reads it;
/// where that read fails (a build without the reader features), the platform's preset is
/// installed in its place, the Theme row shows that preset, never `default`, and the banner
/// reports the read's error.
#[test]
fn a_failed_desktop_read_starts_the_platform_preset_and_says_so() {
    let mut harness = open(egui::Theme::Light, CliArgs::default());
    harness.run_steps(SETTLE);
    let app = harness.state();
    match crate::app::read_next_system() {
        Err(error) => {
            assert_eq!(
                app.settings.theme,
                ThemeChoice::Preset(app.default_preset.clone())
            );
            assert_eq!(
                app.default_preset,
                native_theme::next::Preset::detect().as_str(),
                "the platform's preset stands in"
            );
            let shown = app.theme_error.clone().unwrap_or_default();
            assert!(
                shown.contains(&error) && shown.contains("fallback"),
                "the banner: {shown:?}"
            );
        }
        Ok(system) => {
            assert_eq!(app.settings.theme, ThemeChoice::Default);
            assert_eq!(app.default_preset, system.preset_id().as_str());
        }
    }
}

/// Widget Info and the Theme Map read the theme the pages draw (`OldAtlas::next_shown`): the
/// inactive window's while the viewport is unfocused, the active one's while it is focused.
#[test]
fn widget_info_reads_the_theme_the_pages_draw() -> Result<(), String> {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    for focused in [false, true] {
        let input = harness.input_mut();
        input.focused = focused;
        input
            .viewports
            .entry(egui::ViewportId::ROOT)
            .or_default()
            .focused = Some(focused);
        harness.run_steps(3);
        let app = harness.state();
        let shown = app.atlas.next_shown(&harness.ctx).ok_or("no 0.8.0 theme")?;
        assert_eq!(shown.context.window_active, focused, "the pages' theme");
        let info = app
            .json
            .get(&app.atlas, &harness.ctx)
            .as_ref()
            .map_err(Clone::clone)?;
        assert_eq!(&info.theme, shown, "focused {focused}: Widget Info's theme");
    }
    Ok(())
}

/// The page groups' helpers on the 0.8.0 model read the installed 0.8.0 theme the window shows:
/// `next_value` its values, `next_lined` its line height, `next_role_text` a text role's size and
/// line height, `next_metric` a layout gap; beside it the theme for an inactive window.
#[test]
fn the_next_helpers_read_the_installed_08_theme() -> Result<(), String> {
    use native_theme_egui::{TextRole, next};
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let atlas =
        crate::app::OldAtlas::from_ctx(&harness.ctx).ok_or("no atlas kept in the context")?;
    let theme = harness.ctx.theme();
    let active = atlas.next_resolved(theme).ok_or("no 0.8.0 theme")?.clone();
    let inactive = atlas.next_inactive(theme);
    assert!(active.context.window_active && !inactive.context.window_active);
    assert_eq!(inactive.name, active.name);

    // A context of its own, holding the showcase's atlas as `App::adopt_pair` keeps it, whose
    // viewport reports no focus: the active theme is shown.
    let ctx = egui::Context::default();
    ctx.set_theme(theme);
    ctx.data_mut(|d| d.insert_temp(egui::Id::NULL, atlas.clone()));
    let mut read = None;
    let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
        let style = ui.style().clone();
        let job = |text: egui::RichText| {
            egui::WidgetText::from(text)
                .into_layout_job(&style, egui::FontSelection::Default, egui::Align::Center)
                .sections
                .first()
                .map(|s| (s.format.font_id.size, s.format.line_height))
        };
        let body = egui::TextStyle::Body.resolve(&style).size;
        read = Some((
            crate::demo::next_value(ui, |r| r.name.to_string()),
            job(crate::demo::next_lined(ui, "x", egui::TextStyle::Body)),
            body,
            job(crate::demo::next_role_text(
                ui,
                TextRole::SectionHeading,
                "x",
            )),
            crate::demo::next_metric(ui, |layout| layout.widget_gap),
        ));
    });
    // No painter takes the pass's textures.
    output.textures_delta.clear();
    let (name, lined, body, role, gap) = read.ok_or("the pass ran no UI")?;
    assert_eq!(name.as_deref(), Some(&*active.name));
    assert_eq!(
        lined.and_then(|(_, line_height)| line_height),
        Some(body * next::line_height(&active))
    );
    assert_eq!(
        role,
        Some((
            next::text_role_font(&active, TextRole::SectionHeading).size,
            Some(next::text_role_line_height(
                &active,
                TextRole::SectionHeading
            ))
        ))
    );
    assert_eq!(gap, active.layout.widget_gap);
    Ok(())
}

/// Each widget of the Containers page, scrolled to through AccessKit's `ScrollIntoView`
/// (`Node::scroll_to_me`), ends above the status bar, the widgets of its nested scroll areas
/// too (`demo::nested_scroll`): under [`TEST_PRESET`] light the 1280 × 720 window leaves the
/// "ScrollArea (both)" area reaching under the status bar once its rows are scrolled to, and
/// egui scrolls only that inner area to a row (measured: a row at y 679-695, the bar's top
/// 677). The status bar's own widgets are not the page's and are left out.
#[test]
fn a_containers_widget_scrolled_to_ends_above_the_status_bar() -> Result<(), String> {
    let harness = open_page_under(Page::Containers, egui::Theme::Light, TEST_PRESET);
    let (nested, _) = scrolled_to_above_the_status_bar(harness, &["scrolled label"], false)?;
    assert!(
        nested > 0,
        "no widget of a nested scroll area was scrolled to, so the test proves nothing"
    );
    Ok(())
}

/// The same for the other pages with a scroll area nested in the page's (ES-G3b item 2): the
/// Basic page's list, the Data page's table, whose egui_extras body is a scroll area of its own
/// though it scrolls neither way, and the Theme Map's table across its horizontal area
/// (`demo::nested_scroll`, `demo::nested_scroll_in`). Under [`TEST_PRESET`] light, at a
/// text-scaling factor of 2, each page's nested widgets are scrolled to from the page's far end;
/// on the Basic and Data pages at least one lay out of the page's view before its scroll, so the
/// page's own scroll is seen to follow the inner one. The Theme Map's table lays out only the
/// rows in its own view (`TableBody::rows`), so every nested widget its tree holds is in the
/// page's view already (measured: 24, none out of it): there the test sees only that none is
/// scrolled out of it.
#[test]
fn a_nested_scroll_areas_widget_scrolled_to_ends_above_the_status_bar() -> Result<(), String> {
    for (page, kinds, out_of_view) in [
        (Page::Basic, &["List row"][..], true),
        (Page::Data, &["table cell", "table header"][..], true),
        (
            Page::ThemeMap,
            &["theme map cell", "theme map header"][..],
            false,
        ),
    ] {
        let harness = open_page_scaled(page, TEST_PRESET, 2.0);
        let (nested, hidden) = scrolled_to_above_the_status_bar(harness, kinds, true)?;
        assert!(
            nested > 0 && (hidden > 0 || !out_of_view),
            "{page:?}: {nested} widgets of a nested scroll area scrolled to, {hidden} of them out \
             of the page's view before, so the test proves nothing"
        );
    }
    Ok(())
}

/// Each widget of `harness`'s page, scrolled to through AccessKit's `ScrollIntoView`
/// (`Node::scroll_to_me`), ends above the status bar and, where `far`, in the page's whole view,
/// below the page tabs too; a widget recorded as one of `nested_kinds`, a nested scroll area's,
/// where `far` first after the page is scrolled to the end farther from it, its first or its
/// last widget, so the page itself has to scroll back to it. How many nested widgets were scrolled to, and how many
/// of those lay out of the page's view before their scroll. The chrome's widgets, those above the
/// page tabs' foot at start (where the page, scrolled to its top, shows none), the status bar's
/// and the side panel's, are not the page's and are left out.
fn scrolled_to_above_the_status_bar(
    mut harness: Harness<'static, App>,
    nested_kinds: &[&str],
    far: bool,
) -> Result<(usize, usize), String> {
    let status = place_of(&harness, "chrome.status_bar");
    let side = place_of(&harness, "chrome.side_panel");
    let top = place_of(&harness, "chrome.page_tabs").bottom();
    let ids: Vec<(egui::accesskit::NodeId, f32)> = harness
        .query_all(By::new().predicate(|n| WIDGET_ROLES.contains(&n.role())))
        .filter(|n| {
            let r = n.rect();
            r.top() >= top - 0.5 && !status.contains_rect(r) && !side.intersects(r)
        })
        .map(|n| (n.accesskit_node().locate().0, n.rect().center().y))
        .collect();
    // In the tree's order; the page's ends its topmost and its lowest widget.
    let first = ids.iter().min_by(|a, b| a.1.total_cmp(&b.1));
    let last = ids.iter().max_by(|a, b| a.1.total_cmp(&b.1));
    let ends = first.zip(last).map(|(a, b)| (*a, *b));
    let scroll_to = |harness: &mut Harness<'static, App>, id: egui::accesskit::NodeId| {
        let node = harness
            .query_all(By::new().predicate(move |n| n.locate().0 == id))
            .next()?;
        let before = node.rect();
        node.scroll_to_me();
        // A nested area's scroll and then the page's, each animated: 12 passes (measured: after
        // the page scroll tests' 3, a Basic list row at text scale 2 lay 3 pt under the status
        // bar, the page's scroll still under way).
        harness.run_steps(12);
        let after = harness
            .query_all(By::new().predicate(move |n| n.locate().0 == id))
            .next()
            .map(|n| n.rect());
        Some((before, after))
    };
    let (mut nested, mut hidden) = (0, 0);
    for (id, y) in ids.iter().copied() {
        let kind = harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.id.accesskit_id() == id)
            .map(|r| r.info.kind);
        let inner = kind.is_some_and(|k| nested_kinds.contains(&k));
        if far
            && inner
            && let Some(((first, first_y), (last, last_y))) = ends
        {
            let end = if (y - first_y).abs() > (last_y - y).abs() {
                first
            } else {
                last
            };
            scroll_to(&mut harness, end);
        }
        let Some((before, after)) = scroll_to(&mut harness, id) else {
            continue;
        };
        let rect = after.ok_or_else(|| format!("node {id:?} is gone after its scroll"))?;
        nested += usize::from(inner);
        let out = |r: egui::Rect| r.top() < top - 0.5 || r.bottom() > status.top() + 0.5;
        hidden += usize::from(inner && out(before));
        // Without `far`, only the foot: the Containers page's claim, the status bar's top.
        let missed = if far {
            out(rect)
        } else {
            rect.bottom() > status.top() + 0.5
        };
        assert!(
            !missed,
            "node {id:?} ({kind:?}) at {rect:?} after its scroll (before {before:?}), the page's \
             view {top}..{}",
            status.top()
        );
    }
    Ok((nested, hidden))
}

/// The page tabs hide no tab behind the Pages menu button: under [`TEST_PRESET`] light, whose
/// twelve tabs are wider than the 1280 × 720 window's row (measured: 1172 pt of tabs in a view
/// 894 wide), the row scrolls, its view ending left of the button, so no tab's shown part
/// reaches the button, and the open tab is scrolled wholly into view (`demo::tab_bar`'s
/// `reveal_selected_tab`): the Icons tab, out of view with Basic open, shown whole once open,
/// and the last tab, Theme Map, as well.
#[test]
fn the_page_tabs_hide_no_tab_behind_the_page_menu() -> Result<(), String> {
    let mut hidden = false;
    for page in [Page::Basic, Page::Icons, Page::ThemeMap] {
        let harness = open_page_under(page, egui::Theme::Light, TEST_PRESET);
        let menu = place_of(&harness, "chrome.page_tabs.menu");
        let tabs: Vec<(egui::Rect, egui::Rect)> = harness
            .state()
            .registry
            .records()
            .iter()
            .filter(|r| r.info.kind == "Tab · Page")
            .map(|r| (r.rect, r.whole))
            .collect();
        assert_eq!(tabs.len(), Page::ALL.len(), "{page:?}: one tab per page");
        for (shown, whole) in &tabs {
            assert!(
                !shown.is_positive() || shown.right() <= menu.left(),
                "{page:?}: a tab shown at {shown:?} reaches the Pages button at {menu:?}"
            );
            hidden |= whole.right() > menu.left();
        }
        let open = Page::ALL
            .iter()
            .position(|p| *p == page)
            .and_then(|i| tabs.get(i))
            .ok_or_else(|| format!("{page:?}: no tab of its own"))?;
        // within half a point: the view's edge and a tab's fractional width round apart
        assert!(
            open.0.expand(0.5).contains_rect(open.1),
            "{page:?}: the open tab shows {:?} of {:?}",
            open.0,
            open.1
        );
    }
    assert!(
        hidden,
        "no tab lies past the Pages button, so the row does not overflow and this proves nothing"
    );
    Ok(())
}

/// The Basic table's rows are painted, not recorded, so the "Table" record names what egui's
/// own selectable row gives them, as each list row's record does (`RowLook::own_note`). Under
/// each bundled preset leaving the Table variant's `list.highlight(SELECTED).stroke` unstated,
/// the selected row is outlined as egui's own selected row is, in the List scope's resting
/// widget line where it is visible (`Style::interact_selectable` keeps `bg_stroke`,
/// `egui-0.37.0/src/style.rs`, and `Button::frame_when_inactive(selected)` draws the frame,
/// `src/widgets/button.rs`), and the record names that stroke exactly then: not under adwaita,
/// whose List scope's resting line is 0 wide, which `pages::basic::own_row` counts as no line.
#[test]
fn the_basic_tables_record_names_egui_own_row() -> Result<(), String> {
    use crate::pages::basic::{OWN_ROW, TABLE_SELECTED};
    use native_theme::next::{ListKind, ListQuery, ListVariant, PartStated, Preset, WidgetState};
    let (mut own_stroke, mut own_none) = (false, false);
    for preset in Preset::ALL.iter().map(|p| p.as_str()) {
        let harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or_else(|| format!("{preset}: no 0.8.0 theme installed"))?;
        let lit = r
            .list
            .highlight(ListQuery::new(WidgetState::SELECTED).variant(ListKind {
                variant: ListVariant::Table,
                alternate: false,
            }));
        if lit.from_state.contains(PartStated::STROKE) {
            continue;
        }
        let notes: Vec<String> = harness
            .state()
            .registry
            .records()
            .iter()
            .filter(|r| r.info.kind == "Table")
            .flat_map(|r| r.info.not_themeable.iter())
            .filter(|(what, _)| *what == OWN_ROW)
            .map(|(_, note)| note.clone())
            .collect();
        let row = place_of(&harness, &format!("basic.table.row_{}", TABLE_SELECTED + 1));
        let lines: Vec<egui::Stroke> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(s)
                    if row.contains_rect(s.rect)
                        && s.stroke.width > 0.0
                        && s.stroke.color.a() > 0 =>
                {
                    Some(s.stroke)
                }
                _ => None,
            })
            .collect();
        let own = scope_widgets(&harness, native_theme_egui::Role::List)?
            .inactive
            .bg_stroke;
        let drawn = own.width > 0.0 && own.color.a() > 0;
        assert_eq!(
            lines,
            if drawn { vec![own] } else { Vec::new() },
            "{preset}: the selected table row's outline, egui's own selected row's line"
        );
        let named = notes.iter().any(|n| n.contains("stroke"));
        own_stroke |= named;
        assert_eq!(
            named, drawn,
            "{preset}: egui's own outline {own:?}, named in the Table record: {notes:?}"
        );
        own_none |= !drawn;
    }
    assert!(
        own_stroke && own_none,
        "no preset leaves the selected table row's stroke unstated under a List scope line \
         egui draws ({own_stroke}) and one under a line it paints nothing of ({own_none}), so \
         the test proves nothing"
    );
    Ok(())
}

/// G2 review fix 7, spec §10 rule 0: the Basic table's cell text sits a side of the padding the
/// theme states in from its row's and header's left edge, `list.row(NONE).padding` and
/// `list.header(NONE).padding` of the table (`ListVariant::Table`), and where the theme leaves it
/// unstated egui_extras' own inset, half the List scope's `item_spacing.x`
/// (`pages::basic::table_cell_inset`), never 0, under every bundled preset in light; at least
/// one preset leaves it unstated, so the test can see egui's own.
#[test]
fn the_basic_tables_unstated_cell_padding_is_egui_extras_own() -> Result<(), String> {
    use native_theme::next::{ListKind, ListQuery, ListVariant, Preset, WidgetState};
    let mut unstated = Vec::new();
    for preset in Preset::ALL.iter().map(|p| p.as_str()) {
        let harness = open_page_under(Page::Basic, egui::Theme::Light, preset);
        let r = harness
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or_else(|| format!("{preset}: no 0.8.0 theme installed"))?;
        let q = ListQuery::new(WidgetState::NONE).variant(ListKind {
            variant: ListVariant::Table,
            alternate: false,
        });
        let own = 0.5
            * scope_style(&harness, native_theme_egui::Role::List)?
                .spacing
                .item_spacing
                .x;
        assert!(own > 0.0, "{preset}: the List scope's item spacing is 0");
        for (part, place, text) in [
            (r.list.row(q), "basic.table.row_1", "a.txt"),
            (r.list.header(q), "basic.table.header.name", "Name"),
        ] {
            let [_, _, _, left] = native_theme_egui::next::padding_of(&part);
            if left.is_none() {
                unstated.push(format!("{preset} {place}"));
            }
            let expected = left.unwrap_or(own);
            let cell = place_of(&harness, place);
            let at = harness
                .output()
                .shapes
                .iter()
                .find_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(s) if s.galley.text() == text && cell.contains(s.pos) => {
                        Some(s.pos.x)
                    }
                    _ => None,
                })
                .ok_or_else(|| format!("{preset}: {text:?} is not painted in {place}"))?;
            assert!(
                (at - cell.left() - expected).abs() < 1e-3,
                "{preset}: {text:?} sits {} in from {place}'s left edge, not {expected} (stated \
                 {left:?}, egui_extras' own {own})",
                at - cell.left()
            );
        }
    }
    assert!(
        !unstated.is_empty(),
        "no preset leaves the table's left cell padding unstated, so the test proves nothing"
    );
    Ok(())
}

/// Spec §10 rule 0: a stated horizontal cell padding is drawn, in all three tables alike
/// (`pages::basic::CellSides`). Under kde-breeze with the table's `list.header(NONE)` and
/// `list.row(NONE)` padding patched to unequal sides, the header's unlike the row's, each a
/// multiple of egui_extras' own half `item_spacing.x`: in the Data and Theme Map pages'
/// `TableBuilder`s each first-column cell's content lies its part's left side in from the
/// table's left, and each second-column cell's its part's left side past the first column,
/// which is as wide as its widest cell's content with that cell's two sides, the cells laid out
/// `item_spacing.x` 0 apart (egui_extras-0.37.0 `src/layout.rs:98-104`); in the Basic page's
/// painted table each first-column text lies its part's left side in from its cell and is cut
/// short at the cell's width less both sides (`TextWrapping::truncate_at_width`).
#[test]
fn the_tables_draw_a_stated_horizontal_cell_padding() -> Result<(), String> {
    use native_theme::next::{Edges, ListRule, ListVariant, PartPatch, PartStyle, ThemePatch};
    const SLACK: f32 = 0.05;
    let plain = open_page_under(Page::Data, egui::Theme::Light, "kde-breeze");
    let own = 0.5
        * scope_style(&plain, native_theme_egui::Role::List)?
            .spacing
            .item_spacing
            .x;
    assert!(own > 0.0, "the List scope's item spacing is 0");
    let (header_sides, body_sides) = ([4.0 * own, 3.0 * own], [2.0 * own, 5.0 * own]);
    let patch = |r: &native_theme::next::ResolvedTheme| {
        let q = table_query();
        let across = |part: PartStyle, [left, right]: [f32; 2]| {
            let p = part.padding;
            PartPatch::new().padding(Edges::new(p.top, right, p.bottom, left))
        };
        Some(
            ThemePatch::new().list(
                ListRule::new()
                    .variant(ListVariant::Table)
                    .header(across(r.list.header(q), header_sides))
                    .row(across(r.list.row(q), body_sides)),
            ),
        )
    };
    let stated = |r: &native_theme::next::ResolvedTheme| -> Result<(), String> {
        let q = table_query();
        let sides = |part: &PartStyle| {
            let [_, right, _, left] = native_theme_egui::next::padding_of(part);
            [left, right]
        };
        let got = (sides(&r.list.header(q)), sides(&r.list.row(q)));
        let want = (header_sides.map(Some), body_sides.map(Some));
        if got == want {
            Ok(())
        } else {
            Err(format!(
                "the patched header and row sides are {got:?}, not {want:?}"
            ))
        }
    };

    // The Data table's first column is resizable, so egui_extras never narrows it below a
    // width it once had (egui_extras-0.37.0 `src/table.rs:835-838`); the Theme Map's is as wide
    // as its widest content each frame (`:844-846`).
    for (page, header_kind, cell_kind, columns, exact) in [
        (Page::Data, "table header", "table cell", 2_usize, false),
        (
            Page::ThemeMap,
            "theme map header",
            "theme map cell",
            4,
            true,
        ),
    ] {
        let rects = |harness: &Harness<'_, App>, kind: &str| -> Vec<egui::Rect> {
            harness
                .state()
                .registry
                .records()
                .iter()
                .filter(|r| r.info.kind == kind)
                .map(|r| r.whole)
                .collect()
        };
        // the table's left, from the unpatched page: its header's first cell less the side it
        // draws there, the stated one, else egui_extras' own half spacing
        let unpatched = open_page_under(page, egui::Theme::Light, "kde-breeze");
        let plain_left = unpatched
            .state()
            .atlas
            .next_resolved(egui::Theme::Light)
            .ok_or("kde-breeze: no 0.8.0 theme installed")
            .map(|r| native_theme_egui::next::padding_of(&r.list.header(table_query()))[3])?
            .unwrap_or(own);
        let table_left = rects(&unpatched, header_kind)
            .first()
            .ok_or_else(|| format!("{page:?}: no header recorded unpatched"))?
            .left()
            - plain_left;
        let (harness, r) = open_patched(page, egui::Theme::Light, "kde-breeze", patch)?;
        stated(&r)?;
        let (headers, cells) = (rects(&harness, header_kind), rects(&harness, cell_kind));
        if headers.len() != columns || cells.is_empty() || cells.len() % columns != 0 {
            return Err(format!(
                "{page:?}: {} headers and {} cells recorded, not {columns} and whole rows",
                headers.len(),
                cells.len()
            ));
        }
        let rows: Vec<(&[egui::Rect], [f32; 2])> =
            std::iter::once((headers.as_slice(), header_sides))
                .chain(cells.chunks(columns).map(|row| (row, body_sides)))
                .collect();
        let first = |row: &[egui::Rect]| row.first().copied().ok_or("an empty row");
        let second = |row: &[egui::Rect]| row.get(1).copied().ok_or("a row of one cell");
        let (top, top_sides) = rows.first().copied().ok_or("no rows")?;
        let mut column = 0.0_f32;
        for &(row, [left, right]) in &rows {
            let cell = first(row)?;
            assert!(
                (cell.left() - left - table_left).abs() <= SLACK,
                "{page:?}: a first-column cell's content begins {} past the table's left, not \
                 its left side {left}",
                cell.left() - table_left
            );
            column = column.max(left + cell.width() + right);
        }
        let start = second(top)?.left() - table_left - top_sides[0];
        if exact {
            assert!(
                (start - column).abs() <= SLACK,
                "{page:?}: the second column begins {start} past the table's left, not at the \
                 first column's width {column}"
            );
        } else {
            assert!(
                start >= column - SLACK,
                "{page:?}: the second column begins {start} past the table's left, inside the \
                 first column's {column}"
            );
        }
        for &(row, [left, _]) in &rows {
            let at = second(row)?.left() - table_left;
            assert!(
                (at - start - left).abs() <= SLACK,
                "{page:?}: a second-column cell's content begins {at} past the table's left, not \
                 the column's start {start} and its left side {left}"
            );
        }
    }

    let (basic, r) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", patch)?;
    stated(&r)?;
    let name = place_of(&basic, "basic.table.header.name");
    let row = place_of(&basic, "basic.table.row_1");
    let row_cell = egui::Rect::from_x_y_ranges(row.left()..=name.right(), row.y_range());
    for (text, cell, [left, right]) in [
        ("Name", name, header_sides),
        ("a.txt", row_cell, body_sides),
    ] {
        let (x, room) = basic
            .output()
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(s) if s.galley.text() == text && cell.contains(s.pos) => {
                    Some((s.pos.x, s.galley.job.wrap.max_width))
                }
                _ => None,
            })
            .ok_or_else(|| format!("{text:?} is not painted in its cell {cell:?}"))?;
        assert!(
            (x - cell.left() - left).abs() < 1e-3,
            "the Basic table's {text:?} sits {} in from its cell, not its left side {left}",
            x - cell.left()
        );
        let want = cell.width() - left - right;
        assert!(
            (room - want).abs() < 1e-3,
            "the Basic table's {text:?} is cut short at {room}, not its cell's width less its \
             sides, {want}"
        );
    }
    Ok(())
}

/// No bundled preset, light or dark, in an active or an inactive window, states a fill the
/// showcase's own drawing sites read (`demo::drawn_fill`, `demo::drawn_state_fill`) in a form
/// egui cannot paint, layers that are all gradients: where one did, the site would draw egui's
/// own with only the Theme tab's note naming the home (`demo::drawn_fills_unmapped`, which
/// holds the list of the parts the sites read).
#[test]
fn no_bundled_preset_leaves_a_drawn_fill_unmapped() {
    use native_theme::next::{Preset, Theme, ThemeContext};
    let mut unmapped = Vec::new();
    for preset in Preset::ALL {
        for (scheme, ctx) in [
            ("light", ThemeContext::light()),
            ("dark", ThemeContext::dark()),
        ] {
            for active in [true, false] {
                let r = Theme::preset(*preset).resolve(&ctx.clone().with_window_active(active));
                for note in crate::demo::drawn_fills_unmapped(&r) {
                    unmapped.push(format!("{preset:?} {scheme}, active {active}: {note:?}"));
                }
            }
        }
    }
    assert!(unmapped.is_empty(), "{unmapped:#?}");
}

/// The side-panel toggle's checked fill, `button.body(CHECKED; variant=Flat).fill`
/// (`chrome::toggle_checked`), stated as layers that are all gradients for the flat button
/// alone, is a drawn fill `demo::drawn_fills_unmapped` reports at its home, and so are the
/// hovered and pressed checked fills the rule answers too (`chrome::toggle_checked_in`).
#[test]
fn the_toggles_checked_flat_fill_is_a_drawn_fill() -> Result<(), String> {
    use native_theme::next::{
        ButtonRule, ButtonVariant, Color, GradientStop, Paint, PaintLayer, PartPatch, Preset,
        Theme, ThemeContext, ThemePatch, WidgetState,
    };
    let gradient = Paint::layers(vec![PaintLayer::LinearGradient {
        angle_deg: 90.0,
        stops: std::sync::Arc::from(vec![
            GradientStop {
                offset: 0.0,
                color: Color::new(0.1, 0.2, 0.9, 1.0),
            },
            GradientStop {
                offset: 1.0,
                color: Color::TRANSPARENT,
            },
        ]),
        blend: Default::default(),
    }]);
    let patch = ThemePatch::new().button(
        ButtonRule::new()
            .variant(ButtonVariant::Flat)
            .states(WidgetState::CHECKED)
            .body(PartPatch::new().fill(gradient)),
    );
    let r = Theme::preset(Preset::KdeBreeze)
        .with_patch(patch)
        .resolve(&ThemeContext::light());
    let notes = crate::demo::drawn_fills_unmapped(&r);
    assert_eq!(
        notes,
        [
            "button.body(CHECKED; variant=Flat).fill",
            "button.body(HOVER|CHECKED; variant=Flat).fill",
            "button.body(PRESSED|CHECKED; variant=Flat).fill",
        ]
        .map(native_theme_egui::Note::unmapped),
    );
    Ok(())
}

/// A fill the theme states as layers that are all gradients, at a home a showcase drawing site
/// reads (`tab.indicator(SELECTED).fill`, the selected tab's line) and at one a companion
/// widget reads (`switch.track(CHECKED).fill`, the switch), is listed under the Theme tab's
/// Notes as `Unmapped` at its home (`demo::drawn_fills_unmapped`,
/// `native_theme_egui_widgets::next_fills_unmapped`); the unpatched preset lists neither.
#[test]
fn the_theme_tab_lists_an_all_gradient_fill_at_its_home() -> Result<(), String> {
    use native_theme::next::{
        Color, GradientStop, Paint, PaintLayer, PartPatch, SwitchRule, TabRule, ThemePatch,
        WidgetState,
    };
    let homes = ["tab.indicator(SELECTED).fill", "switch.track(CHECKED).fill"];
    let noted = |harness: &Harness<'static, App>, home: &str| {
        let line = format!("{home}: stated in a form egui cannot draw");
        harness
            .query_all(By::new().predicate(|n| {
                n.value()
                    .is_some_and(|v| v.starts_with("Unmapped\n") && v.contains(&line))
            }))
            .count()
    };
    let mut stock = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    stock.state_mut().inspector_tab = crate::chrome::InspectorTab::Theme;
    stock.run_steps(2);
    for home in homes {
        assert_eq!(noted(&stock, home), 0, "kde-breeze notes {home}");
    }
    let gradient = Paint::layers(vec![PaintLayer::LinearGradient {
        angle_deg: 90.0,
        stops: std::sync::Arc::from(vec![
            GradientStop {
                offset: 0.0,
                color: Color::new(0.1, 0.2, 0.9, 1.0),
            },
            GradientStop {
                offset: 1.0,
                color: Color::TRANSPARENT,
            },
        ]),
        blend: Default::default(),
    }]);
    let fill = PartPatch::new().fill(gradient);
    let patch = |_: &native_theme::next::ResolvedTheme| {
        Some(
            ThemePatch::new()
                .tab(
                    TabRule::new()
                        .states(WidgetState::SELECTED)
                        .indicator(fill.clone()),
                )
                .switch(
                    SwitchRule::new()
                        .states(WidgetState::CHECKED)
                        .track(fill.clone()),
                ),
        )
    };
    let (mut harness, _) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", patch)?;
    harness.state_mut().inspector_tab = crate::chrome::InspectorTab::Theme;
    harness.run_steps(2);
    for home in homes {
        assert_eq!(
            noted(&harness, home),
            1,
            "the Theme tab's Notes list {home} once"
        );
    }
    Ok(())
}

/// The showcase on the Basic page under `preset` light, with `patch` over its 0.8.0 themes where
/// given (`OldAtlas::patched`), and the pointer held at `point`, pressed or not, for sixteen
/// passes.
fn held_at(
    preset: &str,
    patch: Option<&native_theme::next::ThemePatch>,
    point: egui::Pos2,
    press: bool,
) -> Result<Harness<'static, App>, String> {
    let mut args = cli(&[("--theme", TEST_PRESET), ("--tab", "basic")]);
    args.pointer = Some((point.x as u16, point.y as u16));
    args.press = press;
    let mut harness = open(egui::Theme::Light, args);
    install_under(&mut harness, egui::Theme::Light, preset);
    if let Some(patch) = patch {
        let patched = harness.state().atlas.patched(patch)?;
        let ctx = harness.ctx.clone();
        harness.state_mut().adopt_pair(&ctx, patched);
    }
    for step in 0..16u32 {
        let ctx = harness.ctx.clone();
        let mut input = std::mem::take(harness.input_mut());
        input.time = Some(f64::from(step) * 0.5);
        eframe::App::raw_input_hook(harness.state_mut(), &ctx, &mut input);
        *harness.input_mut() = input;
        harness.step();
    }
    Ok(harness)
}

/// The rect the element `id` records.
fn element_rect(harness: &Harness<'_, App>, id: &str) -> Result<egui::Rect, String> {
    harness
        .state()
        .registry
        .records()
        .iter()
        .find(|record| record.info.element.as_deref() == Some(id))
        .map(|record| record.rect)
        .ok_or_else(|| format!("{id} records nothing"))
}

/// The fills and outlines of the filled frames egui painted over the centre of `rect` and
/// within a few points of it: a button's frame, whose edge a pen of another width than the
/// scope's moves (`egui-0.37.0/src/containers/frame.rs`,
/// `apply_stroke_and_expansion_without_layout_shift`).
fn frames_at(harness: &Harness<'_, App>, rect: egui::Rect) -> Vec<(egui::Color32, egui::Stroke)> {
    harness
        .output()
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Rect(shape)
                if shape.fill != egui::Color32::TRANSPARENT
                    && shape.brush.is_none()
                    && rect.expand(4.0).contains_rect(shape.rect)
                    && shape.rect.contains_rect(rect.shrink(4.0)) =>
            {
                Some((shape.fill, shape.stroke))
            }
            _ => None,
        })
        .collect()
}

/// Under windows-11 light, which states the primary button's outline at rest
/// (`button.body(NONE; variant=Primary).stroke`, `#0000000f` laid inside) and no rule for its
/// hovered or pressed state, the primary button is outlined at rest in that pen composited over
/// its fill, as egui ends the fill at the stroke's inner edge, and hovered and pressed in the
/// button scope's own hovered and active entries, egui's treatment of those states, never the
/// resting pen (spec §10, rule 0).
#[test]
fn the_primary_buttons_unstated_states_keep_the_scopes_outline() -> Result<(), String> {
    use native_theme::next::{ButtonQuery, PartStated, WidgetState};
    const ID: &str = "basic.buttons.primary";
    let mut probe = open_page_under(Page::Basic, egui::Theme::Light, "windows-11");
    probe.run_steps(SETTLE);
    let rect = element_rect(&probe, ID)?;
    let widgets = scope_widgets(&probe, native_theme_egui::Role::Button)?;
    let r = probe
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("windows-11: no 0.8.0 theme installed")?;
    let at = |state| r.button.body(ButtonQuery::primary().with_state(state));
    for state in [WidgetState::HOVER, WidgetState::PRESSED] {
        assert!(
            !at(state).from_state.contains(PartStated::STROKE),
            "windows-11 states the primary's {state:?} outline, so the test proves nothing"
        );
    }
    let rest = at(WidgetState::NONE);
    let pen = crate::demo::stated_pen(&rest).ok_or("windows-11 states no primary outline")?;
    let [(fill, drawn)] = frames_at(&probe, rect)[..] else {
        return Err(format!(
            "the primary's frame: {:?}",
            frames_at(&probe, rect)
        ));
    };
    assert!(pen.color.a() < u8::MAX, "{pen:?} is opaque");
    let over = egui::Stroke::new(pen.width, fill.blend(pen.color));
    assert_ne!(over, pen, "the pen composites to itself");
    assert_eq!(drawn, over, "at rest: the stated pen over the fill");
    for (press, own, what) in [
        (false, widgets.hovered.bg_stroke, "hovered"),
        (true, widgets.active.bg_stroke, "pressed"),
    ] {
        assert_ne!(own, over, "{what}: the scope's entry is the resting pen");
        let held = held_at("windows-11", None, rect.center(), press)?;
        let strokes: Vec<egui::Stroke> = frames_at(&held, rect).iter().map(|f| f.1).collect();
        assert_eq!(strokes, [own], "{what}: the scope's own entry");
    }
    Ok(())
}

/// Under kde-breeze light with the primary button's hovered and pressed outlines patched in
/// (`button.body(HOVER; variant=Primary).stroke` 3 px magenta, `(PRESSED; variant=Primary)`
/// 2 px green), the hovered primary is drawn in the first and the pressed in the second, each
/// state's whole pen, its width too, not the resting one's; focused, in the button scope's own
/// active entry.
#[test]
fn the_primary_button_takes_each_stated_states_pen() -> Result<(), String> {
    use native_theme::next::{
        ButtonRule, ButtonVariant, Color, PartPatch, Stroke, ThemePatch, WidgetState,
    };
    use native_theme_egui::next;
    const ID: &str = "basic.buttons.primary";
    let magenta = Color::new(1.0, 0.0, 1.0, 1.0);
    let green = Color::new(0.0, 1.0, 0.0, 1.0);
    let rule = |state, width, colour| {
        ButtonRule::new()
            .variant(ButtonVariant::Primary)
            .states(state)
            .body(PartPatch::new().stroke(Stroke::uniform(width, colour)))
    };
    let patch = ThemePatch::new()
        .button(rule(WidgetState::HOVER, 3.0, magenta))
        .button(rule(WidgetState::PRESSED, 2.0, green));
    let mut probe = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    probe.run_steps(SETTLE);
    let rect = element_rect(&probe, ID)?;
    for (press, want, what) in [
        (
            false,
            egui::Stroke::new(3.0, next::to_color32(magenta)),
            "hovered",
        ),
        (
            true,
            egui::Stroke::new(2.0, next::to_color32(green)),
            "pressed",
        ),
    ] {
        let held = held_at("kde-breeze", Some(&patch), rect.center(), press)?;
        let strokes: Vec<egui::Stroke> = frames_at(&held, rect).iter().map(|f| f.1).collect();
        assert_eq!(strokes, [want], "{what}");
    }

    let patch_for = patch.clone();
    let (mut harness, _) = open_patched(Page::Basic, egui::Theme::Light, "kde-breeze", |_| {
        Some(patch_for)
    })?;
    harness.run_steps(SETTLE);
    harness.remove_cursor();
    harness.run_steps(3);
    let own = scope_widgets(&harness, native_theme_egui::Role::Button)?
        .active
        .bg_stroke;
    harness
        .query_all_by_label("Primary")
        .next()
        .ok_or("no node labelled Primary")?
        .focus();
    harness.run_steps(4);
    let rect = element_rect(&harness, ID)?;
    let strokes: Vec<egui::Stroke> = frames_at(&harness, rect).iter().map(|f| f.1).collect();
    assert_eq!(strokes, [own], "focused: the scope's own active entry");
    Ok(())
}

/// The open side-panel toggle's pens (`chrome::toggle_checked_pens`): under kde-breeze light,
/// which states the flat button's `FOCUS_VISIBLE` outline, the active entry, press and focus
/// alike, takes `(CHECKED | FOCUS_VISIBLE)`'s pen, never a patched `(CHECKED | PRESSED)` one;
/// under macos-monterey light, which states none for `FOCUS_VISIBLE`, with a checked and a
/// checked-pressed outline patched in, it takes `(CHECKED | PRESSED)`'s, and the hovered entry
/// `(CHECKED | HOVER)`'s. A translucent checked pen laid inside is composited over the fill.
#[test]
fn the_open_toggles_pens_follow_the_stated_states() -> Result<(), String> {
    use native_theme::next::{
        ButtonQuery, ButtonRule, ButtonVariant, Color, PartPatch, Preset, Stroke, StrokeAlign,
        Theme, ThemeContext, ThemePatch, WidgetState as S,
    };
    use native_theme_egui::next;
    let fill = egui::Color32::from_rgb(200, 100, 50);
    let magenta = Color::new(1.0, 0.0, 1.0, 1.0);
    let green = Color::new(0.0, 1.0, 0.0, 1.0);
    let flat = |state: S, stroke: Stroke| {
        ButtonRule::new()
            .variant(ButtonVariant::Flat)
            .states(state)
            .body(PartPatch::new().stroke(stroke))
    };
    let pen = |r: &native_theme::next::ResolvedTheme, state| {
        crate::demo::state_pen(&r.button.body(ButtonQuery::flat().with_state(state)))
    };
    let pressed =
        ThemePatch::new().button(flat(S::CHECKED | S::PRESSED, Stroke::uniform(3.0, magenta)));
    let breeze = Theme::preset(Preset::KdeBreeze)
        .with_patch(pressed)
        .resolve(&ThemeContext::light());
    let [rest, hovered, active] = crate::chrome::toggle_checked_pens(Some(&breeze), fill);
    let magenta_pen = egui::Stroke::new(3.0, next::to_color32(magenta));
    assert_eq!(rest, pen(&breeze, S::CHECKED));
    assert_eq!(hovered, pen(&breeze, S::CHECKED | S::HOVER));
    assert_eq!(active, pen(&breeze, S::CHECKED | S::FOCUS_VISIBLE));
    assert!(
        active.is_some() && active != Some(magenta_pen),
        "{active:?}"
    );

    let monterey = Theme::preset(Preset::MacosMonterey)
        .with_patch(
            // a later rule is laid over an earlier one: the checked state's first
            ThemePatch::new()
                .button(flat(S::CHECKED, Stroke::uniform(2.0, green)))
                .button(flat(S::CHECKED | S::HOVER, Stroke::uniform(4.0, green)))
                .button(flat(S::CHECKED | S::PRESSED, Stroke::uniform(3.0, magenta))),
        )
        .resolve(&ThemeContext::light());
    assert_eq!(pen(&monterey, S::FOCUS_VISIBLE), None);
    let [rest, hovered, active] = crate::chrome::toggle_checked_pens(Some(&monterey), fill);
    let green = next::to_color32(green);
    assert_eq!(rest, Some(egui::Stroke::new(2.0, green)));
    assert_eq!(hovered, Some(egui::Stroke::new(4.0, green)));
    assert_eq!(active, Some(magenta_pen));

    let half = Color::new(0.0, 0.0, 0.0, 0.5);
    let inside = Theme::preset(Preset::KdeBreeze)
        .with_patch(ThemePatch::new().button(flat(
            S::CHECKED,
            Stroke::uniform_aligned(1.0, half, StrokeAlign::Inside),
        )))
        .resolve(&ThemeContext::light());
    let [rest, ..] = crate::chrome::toggle_checked_pens(Some(&inside), fill);
    assert_eq!(
        rest,
        Some(egui::Stroke::new(1.0, fill.blend(next::to_color32(half))))
    );
    assert_eq!(crate::chrome::toggle_checked_pens(None, fill), [None; 3]);
    Ok(())
}

/// The open side-panel toggle under kde-breeze light with `(CHECKED | HOVER; variant=Flat)`'s
/// outline patched to 3 px magenta and `(CHECKED | FOCUS_VISIBLE; variant=Flat)`'s to 2 px
/// green: hovered it is drawn in the first and pressed in the second, the pens
/// `chrome::toggle_checked_pens` lays over egui's hovered and active entries.
#[test]
fn the_open_toggle_is_outlined_in_its_hovered_and_active_pens() -> Result<(), String> {
    use native_theme::next::{
        ButtonRule, ButtonVariant, Color, PartPatch, Stroke, ThemePatch, WidgetState as S,
    };
    use native_theme_egui::next;
    const ID: &str = "chrome.status_bar.toggle";
    let magenta = Color::new(1.0, 0.0, 1.0, 1.0);
    let green = Color::new(0.0, 1.0, 0.0, 1.0);
    let flat = |state: S, width, colour| {
        ButtonRule::new()
            .variant(ButtonVariant::Flat)
            .states(state)
            .body(PartPatch::new().stroke(Stroke::uniform(width, colour)))
    };
    let patch = ThemePatch::new()
        .button(flat(S::CHECKED | S::HOVER, 3.0, magenta))
        .button(flat(S::CHECKED | S::FOCUS_VISIBLE, 2.0, green));
    let mut probe = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    probe.run_steps(SETTLE);
    assert!(
        probe.state().side_panel_visible,
        "the side panel starts open"
    );
    let rect = element_rect(&probe, ID)?;
    for (press, want, what) in [
        (
            false,
            egui::Stroke::new(3.0, next::to_color32(magenta)),
            "hovered",
        ),
        (
            true,
            egui::Stroke::new(2.0, next::to_color32(green)),
            "pressed",
        ),
    ] {
        let held = held_at("kde-breeze", Some(&patch), rect.center(), press)?;
        assert!(
            held.state().side_panel_visible,
            "{what}: the panel stays open"
        );
        let strokes: Vec<egui::Stroke> = frames_at(&held, rect).iter().map(|f| f.1).collect();
        assert_eq!(strokes, [want], "{what}");
    }
    Ok(())
}

/// N26: the open side-panel toggle under kde-breeze light, which states the flat button's
/// checked fill and a pressed checked one of its own (`(CHECKED | PRESSED; variant=Flat)`, the
/// accent at 0.30), is filled pressed in the pressed one and hovered in `(CHECKED | HOVER;
/// variant=Flat)`'s, the theme's answer for each state (`chrome::toggle_checked_in`), not in
/// the resting checked fill in every state.
#[test]
fn the_open_toggle_is_filled_in_its_hovered_and_pressed_checked_fills() -> Result<(), String> {
    use native_theme::next::{ButtonQuery, WidgetState as S};
    const ID: &str = "chrome.status_bar.toggle";
    let mut probe = open_page_under(Page::Basic, egui::Theme::Light, "kde-breeze");
    probe.run_steps(SETTLE);
    assert!(
        probe.state().side_panel_visible,
        "the side panel starts open"
    );
    let rect = element_rect(&probe, ID)?;
    let r = probe
        .state()
        .atlas
        .next_resolved(egui::Theme::Light)
        .ok_or("no 0.8.0 theme is installed")?;
    let fill =
        |state| crate::demo::drawn_fill(&r.button.body(ButtonQuery::flat().with_state(state)));
    let rest =
        crate::demo::drawn_state_fill(&r.button.body(ButtonQuery::flat().with_state(S::CHECKED)))
            .ok_or("kde-breeze states no checked flat fill")?;
    let pressed = fill(S::CHECKED | S::PRESSED).ok_or("no pressed checked flat fill")?;
    let hovered = fill(S::CHECKED | S::HOVER).ok_or("no hovered checked flat fill")?;
    assert_ne!(
        rest, pressed,
        "the resting and pressed checked fills are one, so this proves nothing"
    );
    for (press, want, what) in [(false, hovered, "hovered"), (true, pressed, "pressed")] {
        let held = held_at("kde-breeze", None, rect.center(), press)?;
        assert!(
            held.state().side_panel_visible,
            "{what}: the panel stays open"
        );
        let fills: Vec<egui::Color32> = frames_at(&held, rect).iter().map(|f| f.0).collect();
        assert_eq!(fills, [want], "{what}");
    }
    Ok(())
}

/// The Basic page's "On" toggle, egui's selected button, takes the checked fill and label of the
/// state of its previous response (`pages::basic::toggle_on_in`, brief-W3 "Settled"): under a
/// patch stating `button.body(CHECKED)`'s fill and label and the `(CHECKED | HOVER)` and
/// `(CHECKED | PRESSED)` ones, each its own, the pressed toggle is painted in the pressed pair and
/// the hovered one in the hovered pair; under a patch stating only `(PRESSED)`'s fill, the
/// toggle keeps egui's own selected look pressed too (rule 0a).
#[test]
fn the_toggle_that_is_on_takes_the_checked_fill_of_its_state() -> Result<(), String> {
    use crate::pages::basic::{ToggleOn, toggle_on_in};
    use native_theme::next::{
        ButtonQuery, ButtonRule, Color, Paint, PartPatch, Preset, TextPatch, Theme, ThemeContext,
        ThemePatch, WidgetState as S,
    };
    use native_theme_egui::next::to_color32;
    let [on, hover, press] = [
        Color::new(1.0, 0.0, 1.0, 1.0),
        Color::new(0.0, 0.0, 1.0, 1.0),
        Color::new(1.0, 1.0, 0.0, 1.0),
    ];
    let rule = |state: S, colour: Color| {
        ButtonRule::new()
            .states(state)
            .body(PartPatch::new().fill(Paint::Solid(colour)))
            .label(TextPatch::new().color(colour))
    };
    let patch = ThemePatch::new()
        .button(rule(S::CHECKED, on))
        .button(rule(S::CHECKED | S::HOVER, hover))
        .button(rule(S::CHECKED | S::PRESSED, press));
    let (key, _) = unchecked_preset()?;
    let preset = Preset::ALL
        .iter()
        .find(|p| p.as_str() == key)
        .ok_or("the preset is not bundled")?;
    let r = Theme::preset(*preset)
        .with_patch(patch.clone())
        .resolve(&ThemeContext::light());
    let checked = ButtonQuery::new(S::CHECKED);
    let pair = |c: Color| ToggleOn {
        fill: Some(to_color32(c)),
        text: Some(to_color32(c)),
    };
    assert_eq!(toggle_on_in(Some(&r), checked, (false, false)), pair(on));
    assert_eq!(toggle_on_in(Some(&r), checked, (true, false)), pair(hover));
    assert_eq!(toggle_on_in(Some(&r), checked, (true, true)), pair(press));
    let none = ToggleOn {
        fill: None,
        text: None,
    };
    assert_eq!(toggle_on_in(None, checked, (true, true)), none);
    let pressed_only = Theme::preset(*preset)
        .with_patch(ThemePatch::new().button(rule(S::PRESSED, press)))
        .resolve(&ThemeContext::light());
    for was in [(false, false), (true, false), (true, true)] {
        assert_eq!(
            toggle_on_in(Some(&pressed_only), checked, was),
            none,
            "{was:?}"
        );
    }

    let mut probe = open_page_under(Page::Basic, egui::Theme::Light, key);
    probe.run_steps(SETTLE);
    let rect = element_rect(&probe, "basic.buttons.toggle_on")?;
    for (press, want, what) in [(false, hover, "hovered"), (true, press, "pressed")] {
        let held = held_at(key, Some(&patch), rect.center(), press)?;
        let fills: Vec<egui::Color32> = frames_at(&held, rect).iter().map(|f| f.0).collect();
        assert_eq!(fills, [to_color32(want)], "{what}");
    }
    Ok(())
}

/// N6's wiring: the status bar loads the side-panel toggle's freedesktop icon in the colour the
/// theme states for the flat button's icon in the state it is in (`button.icon(CHECKED;
/// variant=Flat).color` while open, `(NONE; variant=Flat)` while closed, patched in), its icon
/// cache key tinted in it. The icon theme is a fixture in a child process.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
#[test]
fn the_toggles_icon_is_loaded_in_the_stated_flat_icon_colour() -> Result<(), String> {
    use crate::demo::IconLoad;
    use native_theme::next::{
        ButtonRule, ButtonVariant, Color, IconPatch, ThemePatch, WidgetState,
    };
    use native_theme_egui::next;
    const NAME: &str = "tests::the_toggles_icon_is_loaded_in_the_stated_flat_icon_colour";
    if native_theme_test_fixtures::in_child() {
        let [on, off] = [
            Color::new(1.0, 0.0, 1.0, 1.0),
            Color::new(0.0, 1.0, 0.0, 1.0),
        ];
        let rule = |state: WidgetState, colour: Color| {
            ButtonRule::new()
                .variant(ButtonVariant::Flat)
                .states(state)
                .icon(IconPatch::new().color(colour))
        };
        let patch = ThemePatch::new()
            .button(rule(WidgetState::NONE, off))
            .button(rule(WidgetState::CHECKED, on));
        let mut harness = open(
            egui::Theme::Light,
            cli(&[
                ("--theme", TEST_PRESET),
                ("--variant", "light"),
                ("--icon-set", "named"),
            ]),
        );
        let patched = harness.state().atlas.patched(&patch)?;
        let ctx = harness.ctx.clone();
        harness.state_mut().adopt_pair(&ctx, patched);
        // The toggle's icon keys loaded since `from`, the last one.
        let last_toggle = |harness: &Harness<'_, App>, from: usize| {
            icon_loads_since(harness, from)
                .into_iter()
                .filter_map(|load| match load {
                    IconLoad::Named(key) if format!("{key:?}").contains("panel-left") => Some(key),
                    _ => None,
                })
                .next_back()
        };
        let from = harness.state().icon_cache.loads.len();
        harness.run_steps(4);
        assert!(
            harness.state().side_panel_visible,
            "the side panel starts open"
        );
        let open_key = last_toggle(&harness, from).ok_or("the open toggle loads no icon")?;
        let on = next::to_color32(on);
        assert_eq!(open_key.clone().tint(on), open_key, "open: tinted {on:?}");
        let from = harness.state().icon_cache.loads.len();
        harness.state_mut().side_panel_visible = false;
        harness.run_steps(4);
        let closed_key = last_toggle(&harness, from).ok_or("the closed toggle loads no icon")?;
        let off = next::to_color32(off);
        assert_eq!(
            closed_key.clone().tint(off),
            closed_key,
            "closed: tinted {off:?}"
        );
        return Ok(());
    }
    const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="currentColor" d="M2 2h12v12H2z"/></svg>"#;
    let child = native_theme_test_fixtures::Child {
        desktop: "sway",
        icon_theme: None,
    };
    native_theme_test_fixtures::run(NAME, child, |icons| {
        native_theme_test_fixtures::write_scalable_theme(
            icons,
            "named",
            &[
                ("utilities-terminal", SVG),
                ("object-rotate-right", SVG),
                ("sidebar-show", SVG),
            ],
        )
    })
    .map_err(|e| e.to_string())
}
