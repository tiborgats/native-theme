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
use native_theme_showcase_support::{Kind, capture};

use crate::{
    CliArgs,
    app::{App, ModeChoice, Page, Settings, ThemeChoice, effective_preferences},
    apply_cli_args,
};

/// A community preset every platform offers (`native-theme/src/presets.rs:196-199`).
pub(crate) const TEST_PRESET: &str = "catppuccin-mocha";

/// The showcase under a named preset, never the desktop's theme (§13 T11), in a window of the
/// size `main` opens (the harness's own default is 800 × 600, `egui_kittest/src/builder.rs:34`).
pub(crate) fn open(theme: egui::Theme, cli: CliArgs) -> Harness<'static, App> {
    open_at(theme, cli, 1.0)
}

/// [`open`] at `pixels_per_point` physical pixels per point (the harness's own is 1,
/// `egui_kittest/src/builder.rs:35`).
fn open_at(theme: egui::Theme, cli: CliArgs, pixels_per_point: f32) -> Harness<'static, App> {
    Harness::builder()
        .with_theme(theme)
        .with_size(crate::WINDOW_SIZE)
        .with_pixels_per_point(pixels_per_point)
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
            "--open-menu" => cli.open_menu = value,
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
/// `connectors/native-theme-iced/examples/showcase-iced.rs:19316`).
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
/// `connectors/native-theme-iced/examples/showcase-iced.rs:19259`). Following is
/// `default_icon_choice` of what the atlas reports per scheme — `breeze`, then
/// `breeze-dark`, where that theme is installed, else `system` (`native-theme/src/icons.rs:1436-1455`),
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
fn interactive_controls_respond() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let (other, other_name) = Theme::list_presets_for_platform()
        .into_iter()
        .map(|info| (info.key, info.display_name))
        .find(|(key, _)| *key != TEST_PRESET)
        .expect("a second preset is offered on every platform");

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
    assert_eq!(
        harness.state().atlas.name(),
        Theme::preset(other).expect("bundled").name
    );

    pick_mode(&mut harness, "Dark");
    assert_eq!(harness.state().settings.mode, ModeChoice::Dark);
    assert_eq!(harness.ctx.theme(), egui::Theme::Dark);

    harness
        .get_by_role_and_label(Role::ComboBox, "Icon theme")
        .click();
    harness.run_steps(SETTLE);
    // A row's label is `IconSetChoice`'s `Display` (`native-theme/src/icons.rs:1353-1363`).
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
    assert!(theme.bottom() <= mode.top() && mode.bottom() <= icon.top());
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
    assert_eq!(labels.len(), 3, "three setting labels: {labels:?}");
    let panel = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "Side panel")
        .map(|r| r.rect)
        .expect("the side panel records itself");
    for (label, control) in labels.iter().zip([theme, mode, icon]) {
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
    harness.get_by_role_and_label(Role::Button, "Icons").click();
    harness.run_steps(2);
    assert_eq!(harness.state().settings.page, Page::Icons);
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
fn the_menus_run_their_actions() {
    let mut harness = open(
        egui::Theme::Dark,
        cli(&[
            ("--theme", TEST_PRESET),
            ("--tab", Page::Text.key()),
            ("--variant", "dark"),
        ]),
    );
    harness.run_steps(2);
    let other = Theme::list_presets_for_platform()
        .into_iter()
        .map(|info| info.key)
        .find(|key| *key != TEST_PRESET)
        .expect("a second preset is offered on every platform");
    let reloaded = Theme::preset(other).expect("bundled").name;
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
fn the_command_palette_runs_what_it_lists() {
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
    let (other, other_name) = Theme::list_presets_for_platform()
        .into_iter()
        .map(|info| (info.key, info.display_name))
        .find(|(key, _)| *key != TEST_PRESET)
        .expect("a second preset is offered on every platform");
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
        Theme::preset(other).expect("bundled").name,
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

/// Three default steps of a quarter second each pass `INFO_SETTLE` (`egui_kittest/src/builder.rs:40`).
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

/// The AccessKit roles egui gives a widget (`egui/src/response.rs:935-957`), less
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
];

#[test]
fn every_widget_reports_itself() {
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        for page in Page::ALL {
            let mut harness = open_page(page, theme);
            let hold = harness
                .state()
                .hold_zone
                .expect("the inspector reports its content rect");
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
                // The scroll lands on the third pass after the request (measured on the Text page).
                harness.run_steps(3);
                // A node egui creates itself, whose id no helper sees (§13.2): a scroll bar
                // (`egui/src/containers/scroll_area.rs:1336`), a window's resize handle
                // (`egui/src/containers/window.rs:1085`), or a node inside a `Window`'s AccessKit
                // node — its title bar's collapse and close buttons (`:1377-1388`, `:1466-1470`).
                let Some((rect, egui_own)) = harness
                    .query_all(By::new().predicate(move |n| n.locate().0 == id))
                    .next()
                    .map(|n| {
                        let node = n.accesskit_node();
                        let mut in_window = false;
                        let mut parent = node.parent();
                        while let Some(p) = parent {
                            // The tree's root is the viewport's own `Window` node: not an egui `Window`.
                            in_window |= p.role() == Role::Window && p.parent().is_some();
                            parent = p.parent();
                        }
                        let own =
                            matches!(node.role(), Role::ScrollBar | Role::Splitter) || in_window;
                        (n.rect(), own)
                    })
                else {
                    continue;
                };
                // `Node::rect` is the untransformed rect (`egui/src/response.rs:912-917`): a node in
                // a transformed layer — the `Scene`'s — is hovered through its layer's transform, as
                // `centre_of` does, where a record gives the layer.
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
                    .unwrap_or_else(|| panic!("{page:?} {theme:?}: nothing shown for node {id:?}"));
                let node_recorded = records.iter().any(|r| r.id.accesskit_id() == id);
                let own = shown.id.accesskit_id() == id;
                // Else the info of a non-container widget whose rect contains the node's; for a
                // node egui creates itself (and no helper records), the innermost record's,
                // container or not (§13.2). A record is clipped to what is shown and the node is
                // not: the last node of a page scrolled to its end may reach a fraction of a
                // pixel past the page's clip, so the rects compare within half a pixel.
                let by_record = records.iter().any(|r| {
                    r.id == shown.id
                        && r.rect.expand(0.5).contains_rect(rect)
                        && (!r.container || (egui_own && !node_recorded))
                });
                assert!(
                    own || by_record,
                    "{page:?} {theme:?}: node {id:?} at {rect:?} shows {} ({:?})",
                    shown.info.kind,
                    shown.id
                );
            }
        }
    }
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
    let (manifest, json) = manifest_and_json(&harness);
    let elements = crate::elements::showcase_elements().unwrap_or_default();
    assert!(!elements.is_empty(), "the element list parses");
    let scheme = (&harness.state().atlas, harness.ctx.theme());
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

/// A radio button and a check box take no selected flag of their own: the checked look is
/// `RoleVariant::Selected` (§4.4), picked per instance from its state; the mixed box is painted
/// as a checked one.
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
        RoleVariant::Selected
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

/// A preset every platform offers other than [`TEST_PRESET`], and its atlas: the watcher's
/// rebuilt atlas in the hand-off tests, told from the installed one by its name.
#[cfg(feature = "watch")]
const OTHER_PRESET: &str = "nord";

#[cfg(feature = "watch")]
fn preset_atlas(name: &str) -> native_theme_egui::ThemeAtlas {
    from_preset(name, false, &AccessibilityPreferences::default())
        .expect("a bundled preset builds")
        .0
}

#[cfg(feature = "watch")]
fn preset_name(name: &str) -> String {
    Theme::preset(name)
        .expect("a bundled preset")
        .name
        .to_string()
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
fn a_hand_off_built_for_an_earlier_selection_installs_the_current_one() {
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
    App::record_rebuild(&app.rebuilt, read, Ok(preset_atlas(TEST_PRESET))).unwrap();
    app.theme_error = Some("an earlier failure".to_string());
    app.take_rebuilt(&ctx);
    assert_eq!(app.atlas.name(), preset_name(OTHER_PRESET));
    assert_eq!(
        app.theme_error, None,
        "the current selection's install succeeded"
    );
}

/// M9-115: a rebuild that returned before the user's install read the record is covered by that
/// install: its hand-off installs nothing.
#[cfg(feature = "watch")]
#[test]
fn a_hand_off_an_install_has_covered_installs_nothing() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    let app = harness.state_mut();
    let read = app.generation();
    App::record_rebuild(&app.rebuilt, read, Ok(preset_atlas(TEST_PRESET))).unwrap();
    app.settings.theme = ThemeChoice::Preset(OTHER_PRESET.to_string());
    app.install(&ctx);
    app.theme_error = Some("an earlier failure".to_string());
    app.take_rebuilt(&ctx);
    assert_eq!(app.atlas.name(), preset_name(OTHER_PRESET));
    assert_eq!(app.theme_error.as_deref(), Some("an earlier failure"));
}

/// M9-114, M9-115: the watcher's `rebuild` records the generation of the selection it read; a
/// hand-off built for the current selection is installed and clears an earlier theme error.
#[cfg(feature = "watch")]
#[test]
fn a_hand_off_built_for_the_current_selection_installs_it_and_clears_the_theme_error() {
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
    App::record_rebuild(&app.rebuilt, current, Ok(preset_atlas(OTHER_PRESET))).unwrap();
    app.theme_error = Some("an earlier failure".to_string());
    app.take_rebuilt(&ctx);
    assert_eq!(app.atlas.name(), preset_name(OTHER_PRESET));
    assert_eq!(app.theme_error, None);
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
        App::record_rebuild(&app.rebuilt, current, Err(watch_failure())).unwrap_err();
        app.watcher_last_error = Some(watch_failure().to_string());
        h.run_steps(SETTLE);
    };
    assert!(!shown(&harness));
    fail(&mut harness);
    assert!(shown(&harness), "a failed rebuild is shown");
    let app = harness.state_mut();
    let current = app.generation();
    App::record_rebuild(&app.rebuilt, current, Ok(preset_atlas(OTHER_PRESET))).unwrap();
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
    App::record_rebuild(&app.rebuilt, read, Ok(preset_atlas(TEST_PRESET))).unwrap();
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
fn a_mode_switch_leaves_a_rebuild_in_flight_current() {
    let mut harness = open_default();
    harness.run_steps(SETTLE);
    let ctx = harness.ctx.clone();
    let app = harness.state_mut();
    let read = app.generation();
    app.set_mode(ModeChoice::Dark, &ctx);
    App::record_rebuild(&app.rebuilt, read, Ok(preset_atlas(OTHER_PRESET))).unwrap();
    app.take_rebuilt(&ctx);
    assert_eq!(app.atlas.name(), preset_name(OTHER_PRESET));
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
fn a_mode_switch_installs_nothing() {
    let other = Theme::list_presets_for_platform()
        .into_iter()
        .map(|info| info.key)
        .find(|key| *key != TEST_PRESET)
        .expect("a second preset is offered on every platform");
    let installed = Theme::preset(TEST_PRESET).expect("bundled").name;
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
}

/// M9-113: under a preset with Mode = System the atlas carries the OS's colour mode
/// (`native_theme::detect::system_is_dark()`), so where the integration reports no scheme
/// (Linux; the harness reports none either) egui draws the desktop's, not its
/// `Options::fallback_theme`, once an install hands the choice back to `ThemePreference::System`.
#[test]
fn a_preset_under_the_system_mode_follows_the_os() {
    let (os_mode, drawn) = if native_theme::detect::system_is_dark() {
        (native_theme::theme::ColorMode::Dark, egui::Theme::Dark)
    } else {
        (native_theme::theme::ColorMode::Light, egui::Theme::Light)
    };
    let mut harness = open(
        egui::Theme::Light,
        cli(&[("--theme", "catppuccin-mocha"), ("--variant", "system")]),
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
fn the_code_view_follows_the_scheme() {
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
    let prefs = AccessibilityPreferences::default();
    let other = Theme::list_presets_for_platform()
        .into_iter()
        .map(|info| info.key)
        .find(|key| {
            from_preset(key, false, &prefs).is_ok_and(|(atlas, _)| {
                let probe = egui::Context::default();
                atlas.install(&probe);
                probe.set_theme(egui::Theme::Dark);
                egui::TextStyle::Monospace.resolve(&probe.global_style()) != before
            })
        })
        .expect("a preset with another monospace font");
    harness.state_mut().settings.theme = ThemeChoice::Preset(other.to_string());
    harness.state_mut().install(&ctx);
    harness.run_steps(3);
    assert_ne!(monospace(&harness), before);
    assert_eq!(
        font(&harness),
        Some(monospace(&harness)),
        "the code view kept the previous preset's monospace font"
    );
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

/// The status bar is as tall as its content — one line of `status_bar.font` — and its padding,
/// as the model states no status-bar height: not a push button's `button.min_height`, which the
/// base style's `interact_size.y` carries (kde-breeze: a 3 + 18 + 2 bar under a 32 button).
#[test]
fn the_status_bar_is_one_line_and_its_padding_tall() {
    let mut harness = open(
        egui::Theme::Light,
        cli(&[("--theme", "kde-breeze"), ("--tab", "basic")]),
    );
    harness.run_steps(4);
    let t = harness
        .state()
        .atlas
        .resolved_for(egui::Theme::Light)
        .clone();
    let size = native_theme_egui::scaled_text_size(
        t.status_bar.font.size,
        harness.state().atlas.accessibility(),
    );
    let line = harness
        .ctx
        .fonts_mut(|f| f.row_height(&egui::FontId::proportional(size)));
    let padding = &t.status_bar.border.padding;
    let (top, bottom) = (
        padding.top.unwrap_or_default(),
        padding.bottom.unwrap_or_default(),
    );
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
    // (`egui/src/containers/panel.rs:950-966`), in `status_bar.border.line_width`.
    let rule = t.status_bar.border.line_width;
    let content = line.max(toggle);
    let expected = rule + top + content + bottom;
    assert!(
        (bar.height() - expected).abs() < 0.5,
        "the status bar is {} tall, a {rule} line + {top} + {content} of content (a {line} text \
         line, a {toggle} toggle) + {bottom} is {expected}",
        bar.height()
    );
}

/// The Overlays page's `Area` floats beside its anchor label; scrolled with the page in a
/// window too short for it, it never lies over the page tabs or the status bar. The window is
/// short enough that the page can scroll its anchor, near the page's end, under the tabs.
#[test]
fn the_area_stays_inside_the_page() {
    let mut harness = Harness::builder()
        .with_theme(egui::Theme::Light)
        .with_size(egui::vec2(crate::WINDOW_SIZE.x, crate::WINDOW_SIZE.y / 4.0))
        .build_eframe(|cc| {
            App::new(cc, &cli(&[("--theme", TEST_PRESET), ("--tab", "overlays")]))
                .expect("the showcase starts under a bundled preset")
        });
    harness.run_steps(4);
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
    for info in Theme::list_presets_for_platform() {
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
/// indicator the theme states or their label, not a push button's `button.min_height`: the
/// model states no row height for either (kde-breeze: a 20 indicator under a 32 button,
/// material-3 18 under 40).
#[test]
fn basic_checkbox_and_radio_rows_fit_the_indicator() {
    for preset in ["kde-breeze", "material-3"] {
        let mut harness = open(
            egui::Theme::Light,
            cli(&[("--theme", preset), ("--tab", "basic")]),
        );
        harness.run_steps(4);
        let t = harness
            .state()
            .atlas
            .resolved_for(egui::Theme::Light)
            .clone();
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
        let tallest_label = t.checkbox.font.size * t.defaults.line_height + 1.0;
        for rect in checkboxes.iter().chain(&radios) {
            assert!(
                rect.height() >= t.checkbox.indicator_width - 0.01,
                "{preset}: {rect:?} is shorter than the indicator"
            );
            assert!(
                rect.height() <= t.checkbox.indicator_width.max(tallest_label),
                "{preset}: {rect:?} is taller than the indicator and its label"
            );
            assert!(
                button.iter().all(|b| rect.height() < b.height()),
                "{preset}: {rect:?} is as tall as a push button"
            );
        }
        // One control per row, each row `layout.widget_gap` below the one above: a row as tall as
        // a push button would leave more room between the controls than the gap.
        let stated = harness.state().atlas.layout().widget_gap;
        assert!(stated.is_some(), "{preset} states layout.widget_gap");
        let gap = stated.unwrap_or_default();
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
}

/// The Basic page applies the leaves egui never reads from the style, per call, as the connector
/// spec asks of the application (§5.3, §5.4): each push button is at least `button.min_width`
/// wide and `button.min_height` tall, each text field `input.min_height` tall; the disabled
/// field is filled with `input.disabled_background` at `input.disabled_opacity`, its frame
/// built in the disabled cell; and
/// the link's text is underlined, at rest, exactly where `link.underline_enabled` says so
/// (kde-breeze states it, material-3 does not).
#[test]
fn the_basic_page_applies_the_per_call_leaves() {
    for preset in ["kde-breeze", "material-3"] {
        let mut harness = open(
            egui::Theme::Light,
            cli(&[("--theme", preset), ("--tab", "basic")]),
        );
        harness.run_steps(4);
        harness.remove_cursor();
        harness.run_steps(2);
        let t = harness
            .state()
            .atlas
            .resolved_for(egui::Theme::Light)
            .clone();
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
                rect.width() >= t.button.min_width - 0.01
                    && rect.height() >= t.button.min_height - 0.01,
                "{preset}: {kind} is {rect:?}, under button.min_width x min_height"
            );
        }
        for kind in [
            "TextEdit (hint)",
            "TextEdit (single line)",
            "TextEdit (disabled)",
        ] {
            let rect = rect_of(kind);
            assert!(
                rect.height() >= t.input.min_height - 0.01,
                "{preset}: {kind} is {rect:?}, under input.min_height"
            );
        }
        let disabled = rect_of("TextEdit (disabled)");
        // kde-breeze states it; material-3's outlined field has no disabled container
        // (docs/platform-facts.md §2.4), so it keeps its enabled fill (`None`, the enabled
        // background). egui fades the disabled field by `input.disabled_opacity` on top
        // (docs/platform-facts.md §2.1.6): 1.0 on both.
        let fill = Some(
            native_theme_egui::convert::to_color32(
                t.input
                    .disabled_background
                    .unwrap_or(t.input.background_color),
            )
            .gamma_multiply(t.input.disabled_opacity),
        );
        let shapes = harness.output().shapes.clone();
        let filled = shapes.iter().any(|clipped| match &clipped.shape {
            egui::Shape::Rect(r) => {
                Some(r.fill) == fill && disabled.contains_rect(r.rect.shrink(1.0))
            }
            _ => false,
        });
        assert!(
            filled,
            "{preset}: the disabled field is not filled with input.disabled_background"
        );
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
            vec![t.link.underline_enabled],
            "{preset}: the link's underline at rest is not link.underline_enabled"
        );
    }
}

/// The tables' header rows are `list_header_height` tall where the theme states one
/// (material-3), else as tall as before (catppuccin-mocha): on the Basic page's painted table
/// (its `basic.table.header` place) a row of the header font, `list.border.padding` above and
/// below its line (`pages/basic.rs`, `row_metrics`); on the Data and Theme Map pages'
/// `TableBuilder`s a body row, egui's own `interact_size.y` where the theme states no
/// `list.row_height`. There the header pushes the first body row down by its height, or its
/// tallest cell where that is taller (egui_extras `layout.rs:165`), and the spacing after a row
/// (`:181`).
/// A body row is no measure: it grows to a cell that wraps, as the Theme Map's leaf names do.
#[test]
fn the_tables_headers_take_the_themes_header_height() {
    for preset in ["material-3", TEST_PRESET] {
        let mut basic = open(
            egui::Theme::Light,
            cli(&[("--theme", preset), ("--tab", "basic")]),
        );
        basic.run_steps(4);
        let t = basic.state().atlas.resolved_for(egui::Theme::Light).clone();
        let stated = native_theme_egui::list_header_height(&t);
        assert_eq!(
            stated.is_some(),
            preset == "material-3",
            "{preset}: the preset's header height"
        );
        let l = &t.list;
        let header_row = l.row_height.unwrap_or(
            l.header_font.size * t.defaults.line_height
                + l.border.padding.top.unwrap_or(0.0)
                + l.border.padding.bottom.unwrap_or(0.0),
        );
        let want = stated.unwrap_or(header_row);
        let placed = basic.state().registry.places()["basic.table.header"];
        assert!(
            (placed.height() - want).abs() <= 0.5,
            "{preset}: the Basic table's header is {} tall, want {want}",
            placed.height()
        );

        // with no `list.row_height` the List scope's `interact_size.y` is egui's own for the
        // scheme (src/style/roles.rs, `row_height`)
        let own = egui::Theme::Light.default_style().spacing.interact_size.y;
        let row = l.row_height.unwrap_or(own);
        let header = stated.unwrap_or(row);
        for (tab, header_kind, cell_kind) in [
            ("data", "table header", "table cell"),
            ("theme-map", "theme map header", "theme map cell"),
        ] {
            let mut page = open(
                egui::Theme::Light,
                cli(&[("--theme", preset), ("--tab", tab)]),
            );
            page.run_steps(4);
            let records = page.state().registry.records();
            let of_kind = |kind: &'static str| records.iter().filter(move |r| r.info.kind == kind);
            let top = |kind| {
                of_kind(kind)
                    .map(|r| r.rect.top())
                    .fold(f32::INFINITY, f32::min)
            };
            // a row is as tall as its tallest cell where that is more than it is given
            let content = of_kind(header_kind)
                .map(|r| r.rect.height())
                .fold(0.0, f32::max);
            let header = header.max(content);
            // the List scope keeps the base style's `item_spacing` (mapping.toml writes it in no
            // List cell)
            let gap = page.ctx.global_style().spacing.item_spacing.y;
            let pushed = top(cell_kind) - top(header_kind);
            assert!(
                (pushed - (header + gap)).abs() <= 0.5,
                "{preset} {tab}: the header pushes the first row {pushed}, want {header} and \
                 the {gap} gap"
            );
        }
    }
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
/// a preset whose buttons have a resting border (kde-breeze), each recorded widget keeps its
/// rect with the pointer on it, but egui's own selectables (`GROWS_WHEN_HOVERED`).
#[test]
fn no_widget_changes_size_when_hovered() {
    let mut grew = Vec::new();
    let mut compared = 0usize;
    for page in Page::ALL {
        let mut harness = open(
            egui::Theme::Light,
            cli(&[("--theme", "kde-breeze"), ("--tab", page.key())]),
        );
        harness.run_steps(4);
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
    }
    assert!(compared > 0, "no widget was compared");
    assert!(
        grew.is_empty(),
        "sized differently when hovered:\n  {}",
        grew.join("\n  ")
    );
}

/// The side panel keeps `LEFT_PANEL_WIDTH` whatever Widget Info shows: hovering each widget of
/// the Basic page in turn shows infos short enough to fit and long enough to scroll, and the
/// panel is as wide after each as it opened. Stepped at 60 passes a second, as a display
/// draws the scroll bar's show animation over several passes (`ScrollArea`'s
/// `animate_bool_responsive`, `egui/src/containers/scroll_area.rs:751-761`).
#[test]
fn the_side_panel_keeps_its_width_when_widget_info_scrolls() {
    const STEP: f32 = 1.0 / 60.0;
    for (preset, theme) in [
        ("kde-breeze", egui::Theme::Light),
        (TEST_PRESET, egui::Theme::Dark),
    ] {
        let cli = cli(&[("--theme", preset), ("--tab", "basic")]);
        let mut harness = Harness::builder()
            .with_theme(theme)
            .with_size(crate::WINDOW_SIZE)
            .with_step_dt(STEP)
            .build_eframe(move |cc| {
                App::new(cc, &cli).expect("the showcase starts under a bundled preset")
            });
        // The harness turns animations off (`egui_kittest/src/lib.rs:144-150`); a display runs
        // them for egui's own `animation_time`.
        let animation_time = egui::Style::default().animation_time;
        harness
            .ctx
            .all_styles_mut(|style| style.animation_time = animation_time);
        harness.run_steps(4);
        let width_of = |h: &Harness<'_, App>| {
            h.state()
                .registry
                .records()
                .iter()
                .find(|r| r.info.kind == "Side panel")
                .map(|r| r.rect.width())
                .expect("the side panel is shown")
        };
        // The panel's record holds its splitter's line, in room its frame reserves after it.
        let opened = crate::LEFT_PANEL_WIDTH
            + harness
                .state()
                .atlas
                .resolved_for(theme)
                .splitter
                .divider_width;
        assert_eq!(width_of(&harness), opened);
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
                width_of(&harness),
                opened,
                "{preset} {theme:?}: the side panel changed width after {kind} was hovered"
            );
        }
        assert!(scrolled, "{preset} {theme:?}: no Widget Info scrolled");
    }
}

/// On the Basic page, `layout.section_gap` is the whole distance between two groups of a column
/// and between two columns, as on the iced and gpui Basic pages: the page lays five columns of
/// one width out that far apart, and in a column the space the page adds makes up the gap with
/// the `item_spacing` egui puts between widgets anyway.
#[test]
fn groups_and_columns_are_a_section_gap_apart() {
    let mut harness = open(
        egui::Theme::Light,
        cli(&[("--theme", "kde-breeze"), ("--tab", "basic")]),
    );
    harness.run_steps(4);
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
    // The Buttons group's last row, and the heading under it in the same column.
    let buttons = rect("toggle button (off)");
    let Some(below) = headings
        .iter()
        .filter(|r| r.top() > buttons.bottom() && (r.left() - buttons.left()).abs() < 0.5)
        .map(|r| r.top())
        .reduce(f32::min)
    else {
        panic!("no heading below the buttons");
    };
    assert!(
        (below - buttons.bottom() - gap).abs() < 0.5,
        "the buttons and the next heading are {} apart, section_gap is {gap}",
        below - buttons.bottom()
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
    ("Icons", &[("icon button", 3)]),
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
    for (preset, expected) in [("kde-breeze", 5), ("material-3", 4), (TEST_PRESET, 5)] {
        let mut harness = open(
            egui::Theme::Light,
            cli(&[("--theme", preset), ("--tab", "basic")]),
        );
        harness.run_steps(4);
        let count = column_count(
            harness.state().atlas.resolved_for(egui::Theme::Light),
            harness.state().atlas.layout(),
            harness.state().demo_state.content_width,
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
        let t = harness
            .state()
            .atlas
            .resolved_for(egui::Theme::Light)
            .clone();
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
        let line = t.list.border.line_width;
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
/// both modes, under the four presets the captures compare: every element it places ends inside
/// the content panel, and it lays out the columns its rule counts from the panel's width. It may
/// be taller than its area (material-3 is): it then scrolls, and the test reports it.
#[test]
fn the_basic_page_fits_the_window_across() {
    use crate::pages::basic::column_count;
    for preset in ["kde-breeze", "material-3", "catppuccin-mocha", "adwaita"] {
        for theme in [egui::Theme::Light, egui::Theme::Dark] {
            let mut harness = open(theme, cli(&[("--theme", preset), ("--tab", "basic")]));
            harness.run_steps(4);
            let state = harness.state();
            let places = state.registry.places();
            let Some(panel) = places.get("chrome.content").copied() else {
                panic!("{preset} {theme:?}: no content panel placed");
            };
            assert!(
                (panel.width() - state.demo_state.content_width).abs() < 0.5,
                "{preset} {theme:?}: the panel is {} wide, the page counts from {}",
                panel.width(),
                state.demo_state.content_width
            );
            let columns = places
                .keys()
                .filter(|id| id.starts_with("basic.column_"))
                .count();
            let count = column_count(
                state.atlas.resolved_for(theme),
                state.atlas.layout(),
                state.demo_state.content_width,
            );
            assert_eq!(columns, count, "{preset} {theme:?}: the columns laid out");
            for (id, rect) in places.iter().filter(|(id, _)| id.starts_with("basic.")) {
                assert!(
                    rect.right() <= panel.right() + 0.5,
                    "{preset} {theme:?}: {id} ends at {}, past the panel's {}",
                    rect.right(),
                    panel.right()
                );
            }
            if state.page_scrolls {
                println!("{preset} {theme:?}: the Basic page scrolls");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// The selected tab's outward corners: tab.active_outward_corner_radius
// ---------------------------------------------------------------------------

/// windows-11's atlas, built as `from_preset` builds it, from its two resolved variants with
/// `tab.active_outward_corner_radius` set on each to `radius` and `edit` applied: the preset
/// states 4 (G3 T15), and the tests set the radius on the resolved themes, whose fields are
/// public, to try it unstated and at other values too.
fn windows_11_atlas(
    radius: Option<f32>,
    edit: impl Fn(&mut native_theme_egui::ResolvedTheme),
) -> native_theme_egui::ThemeAtlas {
    let prefs = AccessibilityPreferences::default();
    let (preset, _) = from_preset("windows-11", false, &prefs).expect("windows-11 loads");
    let [mut light, mut dark] =
        [egui::Theme::Light, egui::Theme::Dark].map(|theme| preset.resolved_for(theme).clone());
    for resolved in [&mut light, &mut dark] {
        resolved.tab.active_outward_corner_radius = radius;
        edit(resolved);
    }
    let mut builder = native_theme_egui::ThemeAtlas::builder(preset.name(), &light, &dark)
        .layout(preset.layout())
        .accessibility(preset.accessibility())
        .icon_set(preset.icon_set());
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        if let Some(name) = preset.icon_theme(theme) {
            builder = builder.icon_theme(theme, name);
        }
    }
    #[cfg(feature = "system-fonts")]
    {
        builder = builder.fonts(native_theme_egui::fonts::FontPlan::from_system(&light));
    }
    builder.build()
}

/// The showcase on its Basic page in `theme`, with [`windows_11_atlas`]'s atlas installed in
/// place of the one it opened with, as the app installs one, at rest: windows-11 is a preset of
/// another platform, which `--theme` does not take here.
fn open_curving_out(
    theme: egui::Theme,
    radius: Option<f32>,
    edit: impl Fn(&mut native_theme_egui::ResolvedTheme),
) -> Harness<'static, App> {
    open_curving_out_at(theme, radius, 1.0, edit)
}

/// [`open_curving_out`] at `pixels_per_point` physical pixels per point.
fn open_curving_out_at(
    theme: egui::Theme,
    radius: Option<f32>,
    pixels_per_point: f32,
    edit: impl Fn(&mut native_theme_egui::ResolvedTheme),
) -> Harness<'static, App> {
    let mut harness = open_at(
        theme,
        cli(&[("--theme", TEST_PRESET), ("--tab", "basic")]),
        pixels_per_point,
    );
    harness.run_steps(4);
    let ctx = harness.ctx.clone();
    let app = harness.state_mut();
    app.adopt(&ctx, windows_11_atlas(radius, edit));
    harness.run_steps(4);
    harness.remove_cursor();
    harness.run_steps(2);
    harness
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

/// docs/platform-facts.md §2.11 `active_outward_corner_radius`: under windows-11 with a radius
/// of 4 and its outline of 1, each of the three tab bars draws the selected tab's two flares
/// with the numbers every connector's test shares (spec §12.2) -- the left about
/// `(x0 - 3, y1 - 4)`, clipped to `[x0 - 3, x0 + 1] x [y1 - 4, y1]`, the right about
/// `(x1 + 3, y1 - 4)`, clipped to `[x1 - 1, x1 + 3] x [y1 - 4, y1]`; in each, the fill an
/// annulus from 3 to `4 * sqrt(2) + 1` in `tab.active_background`, then the outline one from 3
/// to 4 in `tab.border.color`. epaint strokes a circle outside its radius
/// (`epaint-0.36.2` `src/tessellator.rs:1531`), so each annulus is a circle of its inner radius
/// stroked as wide as the annulus, and tessellated it reaches from 3 to `4 * sqrt(2) + 1`. Under
/// them, after every tab, the strip `tab.active_background` lays over the selected tab's bottom
/// edge, `[x0 + 1, x1 - 1] x [y1 - 1, y1]`. The first tab starts 4 further in than with the radius
/// unstated, and the Basic bar, as wide as its tabs, ends 4 after its last.
#[test]
fn the_tab_bars_draw_the_selected_tabs_outward_corners() {
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let square_feet = open_curving_out(theme, None, |_| {});
        let harness = open_curving_out(theme, Some(4.0), |_| {});
        let state = harness.state();
        let t = state.atlas.resolved_for(theme).clone();
        assert_eq!(t.tab.border.line_width, 1.0, "windows-11's outline");
        assert_eq!(t.tab.active_outward_corner_radius, Some(4.0));
        let fill = native_theme_egui::convert::to_color32(t.tab.active_background);
        let outline = native_theme_egui::convert::to_color32(t.tab.border.color);
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
}

/// At a fractional scale, 1.25 physical pixels per point, each flare is placed from the selected
/// tab's rectangle rounded to physical pixels, the one epaint strokes the tab's frame on, so the
/// curve meets the side edge it continues: in each of the three bars the two flares' circles lie
/// about `(x0 - 3, y1 - 4)` and `(x1 + 3, y1 - 4)` of the rounded rectangle, clipped to their
/// squares about it; and some selected tab lies off the pixel grid, so the rounding shows.
#[test]
fn at_a_fractional_scale_the_flares_sit_on_the_tabs_pixel_grid() {
    use egui::emath::GuiRounding as _;
    let ppp = 1.25;
    let harness = open_curving_out_at(egui::Theme::Light, Some(4.0), ppp, |_| {});
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
}

/// Without an outline (`tab.border.line_width` 0), each flare is the fill alone, an annulus from
/// the radius, 4, to `4 * sqrt(2) + 1`, in a square at the tab's outer edge, and no strip covers
/// a bottom edge the tab does not have.
#[test]
fn without_an_outline_the_fill_begins_at_the_radius() {
    let harness = open_curving_out(egui::Theme::Light, Some(4.0), |r| {
        r.tab.border.line_width = 0.0;
    });
    let t = harness
        .state()
        .atlas
        .resolved_for(egui::Theme::Light)
        .clone();
    let fill = native_theme_egui::convert::to_color32(t.tab.active_background);
    let far = 4.0 * std::f32::consts::SQRT_2 + 1.0;
    let shapes = &harness.output().shapes;
    for [_, one_id, _] in TAB_BARS {
        let one = harness.state().registry.places()[one_id];
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
}

/// With the radius unstated, or stated as one no tab bar draws (0, not finite), no circle is
/// drawn and no strip: the selected tab keeps its bottom edge.
#[test]
fn without_the_radius_the_tab_bars_draw_no_flares() {
    for radius in [None, Some(0.0), Some(f32::NAN)] {
        let harness = open_curving_out(egui::Theme::Light, radius, |_| {});
        let t = harness
            .state()
            .atlas
            .resolved_for(egui::Theme::Light)
            .clone();
        let fill = native_theme_egui::convert::to_color32(t.tab.active_background);
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
            let one = harness.state().registry.places()[one_id];
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
}

/// With a bottom indicator and the radius both stated, no strip is painted, since the line
/// covers the bottom edge, and the flares are painted over the line as without it.
#[test]
fn a_bottom_indicator_lies_under_the_flares() {
    let harness = open_curving_out(egui::Theme::Light, Some(4.0), |r| {
        r.tab.active_indicator_color = Some(r.defaults.accent_color);
        r.tab.active_indicator_width = Some(2.0);
        r.tab.active_indicator_side = Some(native_theme::theme::TabIndicatorSide::Bottom);
    });
    let t = harness
        .state()
        .atlas
        .resolved_for(egui::Theme::Light)
        .clone();
    let fill = native_theme_egui::convert::to_color32(t.tab.active_background);
    let accent = native_theme_egui::convert::to_color32(t.defaults.accent_color);
    let shapes = &harness.output().shapes;
    for [_, one_id, _] in TAB_BARS {
        let one = harness.state().registry.places()[one_id];
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
            panic!("{one_id}: the bottom line is {lines:?}");
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
}

/// The Basic page's column rule makes room for both of the selected tab's flares, as the gpui
/// page's does: under windows-11, whose widest fixed-width control is the two tabs, a page
/// whose fifth holds the two tabs and 4px more lays out five columns, and four once the theme
/// states an outward corner radius of 4, whose two flares need 8; five again for a radius
/// `demo::tab_bar` draws no flares for, 0, negative or not finite.
#[test]
fn the_basic_page_makes_room_for_both_flares() {
    use crate::pages::basic::column_count;
    let atlas = windows_11_atlas(Some(4.0), |_| {});
    let stated = atlas.resolved_for(egui::Theme::Light).clone();
    let layout = atlas.layout();
    let mut unstated = stated.clone();
    unstated.tab.active_outward_corner_radius = None;
    let margin = layout.window_margin.unwrap_or_default();
    let gap = layout.section_gap.unwrap_or_default();
    let bar = &stated.scrollbar;
    let gutter = if bar.overlay_mode {
        0.0
    } else {
        bar.groove_width
    };
    let t = &stated.tab;
    let tabs = t.min_width + t.min_width + t.item_gap.unwrap_or_default();
    let content = 5.0 * (tabs + 4.0) + 4.0 * gap + margin + margin + gutter;
    assert_eq!(
        column_count(&unstated, layout, content),
        5,
        "without outward corners the two tabs fit a fifth"
    );
    assert_eq!(
        column_count(&stated, layout, content),
        4,
        "the two tabs and both flares do not fit a fifth"
    );
    for radius in [0.0, -1.0, f32::NAN] {
        let mut undrawn = stated.clone();
        undrawn.tab.active_outward_corner_radius = Some(radius);
        assert_eq!(
            column_count(&undrawn, layout, content),
            5,
            "a radius of {radius} keeps room for flares the bar does not draw"
        );
    }
}

/// A page's section headings are set in the theme's section-heading role
/// (`text_scale.section_heading`), as the iced showcase's section titles are: its size and line
/// height, and at the body's weight in the proportional family (kde-breeze states 400 for both);
/// at another weight (adwaita's 700 over a 400 body) in the face registered at that weight,
/// where the fonts hold one, and the proportional family where they do not.
#[test]
fn section_headings_take_the_section_heading_role() {
    let mut other_weight = Vec::new();
    for preset in ["kde-breeze", "catppuccin-mocha", "adwaita"] {
        let mut harness = open(
            egui::Theme::Light,
            cli(&[("--theme", preset), ("--tab", "basic")]),
        );
        harness.run_steps(4);
        let t = harness
            .state()
            .atlas
            .resolved_for(egui::Theme::Light)
            .clone();
        let prefs = harness.state().atlas.accessibility().clone();
        let role = native_theme_egui::TextRole::SectionHeading;
        let size = native_theme_egui::text_role_font(&t, role, &prefs).size;
        let line_height = native_theme_egui::text_role_line_height(&t, role, &prefs);
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
        let (weight, body) = (t.text_scale.section_heading.weight, t.defaults.font.weight);
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
/// page's first button is filled with `button.hover_background`, and pressed with
/// `button.active_background`.
#[test]
fn a_held_pointer_hovers_and_presses_the_control_under_it() {
    let args = [("--theme", "kde-breeze"), ("--tab", "basic")];
    let mut probe = open(egui::Theme::Light, cli(&args));
    probe.run_steps(4);
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
    let t = probe.state().atlas.resolved_for(egui::Theme::Light).clone();
    let hovered = native_theme_egui::convert::to_color32(t.button.hover_background);
    let pressed = t
        .button
        .active_background
        .map(native_theme_egui::convert::to_color32);
    for (press, fill) in [(false, Some(hovered)), (true, pressed)] {
        let mut args = cli(&args);
        args.pointer = Some((rect.center().x as u16, rect.center().y as u16));
        args.press = press;
        let mut harness = open(egui::Theme::Light, args);
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
}

/// A held pointer is moved there once and then left still, so the Basic page's tooltip, which
/// egui shows only under a pointer at rest, opens under it: a move on every pass would keep it
/// shut.
#[test]
fn a_held_pointer_opens_the_tooltip_under_it() {
    let args = [("--theme", "kde-breeze"), ("--tab", "basic")];
    let mut probe = open(egui::Theme::Light, cli(&args));
    probe.run_steps(4);
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
    let args = [("--theme", "kde-breeze"), ("--tab", "basic")];
    for capturing in [false, true] {
        let mut args = cli(&args);
        args.capture = capturing;
        let mut harness = open(egui::Theme::Light, args);
        harness.run_steps(4);
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
fn a_set_preference_is_a_selected_switch() {
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
    // the user's: the switch shows the user's request, so it is off, and a change of another
    // switch stores no request for Reduce motion.
    let mut os = AccessibilityPreferences::default();
    os.reduce_motion = true;
    let (atlas, _) = from_preset(TEST_PRESET, false, &os).expect("a bundled preset");
    harness.state_mut().settings.prefs = None;
    harness.state_mut().adopt(&ctx, atlas);
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
}

/// The Icons page's `ui.image` is drawn at the toolbar icon size, not at the page's width,
/// which would push every icon row below the fold.
#[cfg(feature = "lucide-icons")]
#[test]
fn the_icons_page_image_is_icon_sized() {
    let harness = open_page(Page::Icons, egui::Theme::Light);
    let t = harness.state().atlas.resolved_for(harness.ctx.theme());
    let size =
        native_theme_egui::icons::icon_size(t, native_theme_egui::icons::IconContext::Toolbar);
    let rect = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "ui.image")
        .map(|r| r.rect)
        .expect("the preset's Lucide set has ActionSave");
    assert!(
        rect.width() <= size + 1.0 && rect.height() <= size + 1.0,
        "ui.image is {rect:?}, the toolbar icon size {size}"
    );
}

fn manifest_and_json(harness: &Harness<'_, App>) -> (Manifest, serde_json::Value) {
    let manifest = Manifest::parse(MANIFEST).expect("the manifest parses");
    let json =
        crate::info::theme_json(&harness.state().atlas, harness.ctx.theme()).expect("serialises");
    (manifest, json)
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

/// The value line a row prints, as R11 §B states it: the leaf, then its value — a colour as its
/// hex text, a number as the `f32` it is prints, `None` as "not stated".
fn value_line(leaf: &str, value: &serde_json::Value) -> String {
    let text = match value {
        serde_json::Value::Null => "not stated".to_string(),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n
            .as_f64()
            .map_or_else(|| n.to_string(), |n| format!("{}", n as f32)),
        other => other.to_string(),
    };
    format!("{leaf} {text}")
}

/// The value at a dotted leaf, through a JSON pointer rather than the info module's walk.
fn leaf_value(json: &serde_json::Value, leaf: &str) -> Option<serde_json::Value> {
    json.pointer(&format!("/{}", leaf.replace('.', "/")))
        .cloned()
}

#[test]
fn the_info_is_the_manifest() {
    let manifest = Manifest::parse(MANIFEST).expect("the manifest parses");
    let prefs = AccessibilityPreferences::default();
    let mut seams = vec![Seam::Base];
    for role in native_theme_egui::Role::all() {
        for variant in RoleVariant::all() {
            seams.push(Seam::Role(*role, *variant));
        }
    }
    seams.extend(Surface::all().iter().map(|s| Seam::Surface(*s)));
    for info in Theme::list_presets() {
        for is_dark in [false, true] {
            let (atlas, _) =
                from_preset(info.key, is_dark, &prefs).expect("a bundled preset builds");
            let theme = if is_dark {
                egui::Theme::Dark
            } else {
                egui::Theme::Light
            };
            let json = crate::info::theme_json(&atlas, theme).expect("serialises");
            // The values the info must print, read here and not through the info module.
            let expected = resolved_json(&atlas, theme);
            for seam in &seams {
                let rows = manifest.rows_for(seam);
                let listed: BTreeSet<String> = rows.iter().map(|r| r.leaf.clone()).collect();
                assert_eq!(
                    listed,
                    expected_leaves(&manifest, seam),
                    "{} {is_dark} {seam:?}",
                    info.key
                );
                assert!(!rows.is_empty(), "{seam:?} lists no row");
                for row in rows {
                    let printed =
                        crate::info::manifest_row(row, &json, std::slice::from_ref(seam)).lines();
                    let lines = printed.join("\n");
                    // The first line, whole: a substring would let `12` pass for `1`.
                    if let Some(value) = leaf_value(&expected, &row.leaf) {
                        assert_eq!(
                            printed.first(),
                            Some(&value_line(&row.leaf, &value)),
                            "{} {seam:?} {}",
                            info.key,
                            row.leaf
                        );
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
    let other = Theme::list_presets_for_platform()
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
    let json = app
        .json
        .get(&app.atlas, harness.ctx.theme())
        .clone()
        .unwrap_or_default();
    let elements = crate::elements::showcase_elements().unwrap_or_default();
    let scheme = (&app.atlas, harness.ctx.theme());
    let text =
        crate::info::InfoView::of(&shown, &elements, &manifest, (&json, other), scheme).text();
    let expected = resolved_json(&harness.state().atlas, harness.ctx.theme());
    let line = leaf_value(&expected, "button.background_color")
        .map(|value| value_line("button.background_color", &value));
    assert!(
        line.as_ref()
            .is_some_and(|line| text.lines().any(|l| l == line)),
        "the info shows the previous preset's values, not {line:?}"
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

/// `serde_json::to_value` of the resolved theme the atlas installs for `theme`, with the four
/// `layout.` leaves under the manifest's names (§13.1), computed here, not by `info::theme_json`.
#[cfg(test)]
fn resolved_json(atlas: &native_theme_egui::ThemeAtlas, theme: egui::Theme) -> serde_json::Value {
    let mut json =
        serde_json::to_value(atlas.resolved_for(theme)).expect("ResolvedTheme serialises");
    let layout = atlas.layout();
    if let Some(object) = json.as_object_mut() {
        object.insert(
            "layout".to_string(),
            serde_json::json!({
                "widget_gap": layout.widget_gap,
                "container_margin": layout.container_margin,
                "window_margin": layout.window_margin,
                "section_gap": layout.section_gap,
            }),
        );
    }
    json
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
            .filter_map(|data| skrifa::FontRef::from_index(&data.font, data.index).ok())
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
/// `connectors/native-theme-gpui/src/showcase.rs:853`, ported line for line).
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
                    "{path}:{line}: {seam} applied outside demo.rs's helpers, which apply and record each seam once"
                );
            }
        }
    }
    assert!(in_demo > 0, "the detector read no seam call in demo.rs");
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
fn the_chrome_reports_itself() {
    let mut harness = open_page(Page::Buttons, egui::Theme::Light);
    let radius = harness.ctx.global_style().interaction.interact_radius;
    // The inspector's content is the hold zone, where nothing is chosen (§10.4).
    let hold = harness
        .state()
        .hold_zone
        .expect("the inspector reports its content rect");
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
        let target = harness
            .state()
            .registry
            .records()
            .iter()
            .find(|r| r.info.kind == kind)
            .unwrap_or_else(|| panic!("no {kind} record"));
        let (id, rect) = (target.id, target.rect);
        // A point inside the target and inside no smaller record: the element's own surface.
        // The records around it (the chrome bar around the menu bar and the toolbar, the side
        // panel around the inspector's tabs, the central panel around the page tabs) contain it
        // too, and lose to it by area (§10.4, innermost hovered wins).
        let point = (0..40)
            .flat_map(|x| (0..40).map(move |y| (x, y)))
            .map(|(x, y)| {
                rect.left_top()
                    + egui::vec2(
                        rect.width() * (x as f32 + 0.5) / 40.0,
                        rect.height() * (y as f32 + 0.5) / 40.0,
                    )
            })
            .find(|p| {
                // egui counts a widget within its interact radius as under the pointer too
                // (`egui/src/hit_test.rs`, §10.4's `contains_pointer`).
                !hold.expand(radius).contains(*p)
                    && !records.iter().any(|(other, r)| {
                        *other != id && r.expand(radius).contains(*p) && r.area() <= rect.area()
                    })
            })
            .unwrap_or_else(|| panic!("{kind} has no point of its own"));
        hover_and_settle(&mut harness, point);
        assert_eq!(shown_kind(&harness), Some(kind));
    }
    let pos = centre_of(&harness, "button (enabled)");
    hover_and_settle(&mut harness, pos);
    let title = harness.state().status_title();
    assert_eq!(title, "button (enabled)");
    assert!(
        harness.query_all_by_label(&title).next().is_some(),
        "the status bar does not show the title"
    );
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
/// (the gpui showcase's `ALLOWED_STYLE_LITERALS`, `connectors/native-theme-gpui/src/showcase.rs:2985`).
/// `tests.rs`, which holds the detector's own sample source, is not scanned, so it needs no entry.
const ALLOWED_STYLE_LITERALS: &[(&str, &str)] = &[
    (
        "demo_colour",
        "the colour a Colour-page editor starts from is the datum on display",
    ),
    (
        "icons_indicator",
        "the 0.5 of Vec2::splat(0.5) is the centre Image::rotate turns about (§4.10)",
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

/// A `_ms` leaf of the list reads as a `_px` one does: `tooltip.show_delay_ms` shows the
/// resolved theme's `tooltip.show_delay` with " ms" and finds its `mapping.toml` row,
/// `tooltip.show_delay`.
#[test]
fn a_ms_leaf_reads_its_value() {
    let prefs = AccessibilityPreferences::default();
    let (atlas, _) = from_preset("kde-breeze", false, &prefs).expect("a bundled preset builds");
    let delay = atlas.resolved_for(egui::Theme::Light).tooltip.show_delay;
    assert!(delay.is_some(), "kde-breeze states a tooltip delay");
    let json = crate::info::theme_json(&atlas, egui::Theme::Light).expect("serialises");
    let shown = native_theme_showcase_support::values::leaf_value(
        "tooltip.show_delay_ms",
        &json,
        atlas.layout(),
        Some(atlas.icon_set()),
        atlas.icon_theme(egui::Theme::Light),
    );
    assert_eq!(Some(shown.text), delay.map(|ms| format!("{ms} ms")));
    assert_eq!(
        crate::info::mapping_key("tooltip.show_delay_ms"),
        "tooltip.show_delay"
    );
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
/// each parent the list measures an element against is placed where the element is.
#[test]
fn the_layout_dump_names_every_listed_element_the_basic_page_draws() {
    let elements = listed_elements();
    let ids: BTreeSet<&str> = elements.iter().map(|e| e.id.as_str()).collect();
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let harness = open_page(Page::Basic, theme);
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
fn widget_info_shows_the_listed_elements_rows() {
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
    let (manifest, json) = manifest_and_json(&harness);
    if let (Some(shown), Some(listed)) = (shown, listed) {
        let scheme = (&harness.state().atlas, harness.ctx.theme());
        let view =
            crate::info::InfoView::of(&shown, &elements, &manifest, (&json, TEST_PRESET), scheme);
        assert_eq!(view.title, "Button · Normal");
        let leaves: Vec<&str> = view.rows.iter().map(|r| r.leaf.as_str()).collect();
        let want: Vec<&str> = listed.leaves.iter().map(String::as_str).collect();
        assert_eq!(leaves, want);
        assert!(
            view.rows
                .iter()
                .all(|r| r.how.starts_with("egui: ") || r.how.starts_with("not reachable: ")),
            "{:#?}",
            view.rows
        );
        assert!(
            harness
                .query_all_by_label_contains("button.background_color")
                .next()
                .is_some(),
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
    let args = || cli(&[("--theme", "kde-breeze"), ("--tab", "basic")]);
    let at = {
        let mut harness = open(egui::Theme::Light, args());
        harness.run_steps(4);
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
    // `build_eframe` runs, take no time (`with_step_dt`), so the steps below set the clock.
    let run = |dump: Option<&std::path::Path>, pointer: bool| {
        let mut args = args();
        args.dump_layout = dump.map(|path| path.display().to_string());
        let mut harness = Harness::builder()
            .with_theme(egui::Theme::Light)
            .with_size(crate::WINDOW_SIZE)
            .with_step_dt(0.0)
            .build_eframe(move |cc| {
                App::new(cc, &args).expect("the showcase starts under a bundled preset")
            });
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
                "pointer {pointer}: the dump is not the layout the run settled in:\n{text}\n---\n{last:#}"
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
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            for theme in ["gtk-names", "breeze-names"] {
                let image = crate::demo::named_image(
                    ui,
                    crate::demo::ChromeIcon::PanelLeft,
                    native_theme::theme::IconSet::Freedesktop,
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
        let harness = Harness::new_ui(|ui| {
            for theme in ["gtk-names", "breeze-names"] {
                if let Some(image) = crate::demo::named_image(
                    ui,
                    crate::demo::ChromeIcon::PanelLeft,
                    native_theme::theme::IconSet::Freedesktop,
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
