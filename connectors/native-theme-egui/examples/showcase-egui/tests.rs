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

/// The showcase under a named preset, never the desktop's theme (§13 T11), in a window of the
/// size `main` opens (the harness's own default is 800 × 600, `egui_kittest/src/builder.rs:34`).
pub(crate) fn open(theme: egui::Theme, cli: CliArgs) -> Harness<'static, App> {
    Harness::builder()
        .with_theme(theme)
        .with_size(crate::WINDOW_SIZE)
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

/// Kinds only `page` records, one of which it always draws: seen, the page itself was drawn,
/// not only the chrome around it.
fn page_kinds(page: Page) -> &'static [&'static str] {
    match page {
        Page::Basic => &["Label (body text)"],
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
#[test]
fn widget_info_is_handed_the_preset_key() {
    let mut harness = open(egui::Theme::Light, cli(&[("--theme", TEST_PRESET)]));
    assert_eq!(harness.state().preset_key(), TEST_PRESET);
    assert_ne!(harness.state().preset_key(), harness.state().atlas.name());
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

use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT as _, Queryable as _};

use crate::{WINDOW_TITLE, chrome::Action, native_options};

fn open_default() -> Harness<'static, App> {
    open(egui::Theme::Light, cli(&[("--theme", TEST_PRESET)]))
}

/// A menu item's AccessKit label is its text and its shortcut text joined by a
/// space (`egui/src/atomics/atoms.rs:51-62`).
fn menu_item_label(ctx: &egui::Context, action: Action) -> String {
    match action.shortcut() {
        Some(shortcut) => format!("{} {}", action.label(), ctx.format_shortcut(&shortcut)),
        None => action.label().to_string(),
    }
}

/// The Mode row's `ComboBox` opened and its row `mode` clicked.
fn pick_mode(harness: &mut Harness<'_, App>, mode: &str) {
    harness
        .get_by_role_and_label(Role::ComboBox, "Mode")
        .click();
    harness.run();
    harness.get_by_label(mode).click();
    harness.run();
}

/// T11 (b): the theme, mode and icon pickers install what they name.
#[test]
fn interactive_controls_respond() {
    let mut harness = open_default();
    harness.run();
    let (other, other_name) = Theme::list_presets_for_platform()
        .into_iter()
        .map(|info| (info.key, info.display_name))
        .find(|(key, _)| *key != TEST_PRESET)
        .expect("a second preset is offered on every platform");

    // The rows are the presets' display names, as the gpui showcase's (§10.4, parity item 1).
    harness
        .get_by_role_and_label(Role::ComboBox, "Theme")
        .click();
    harness.run();
    harness.get_by_label(other_name).click();
    harness.run();
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
    harness.run();
    // A row's label is `IconSetChoice`'s `Display` (`native-theme/src/icons.rs:661-671`).
    // The list holds every installed freedesktop theme before the bundled sets, so the row may
    // lie below the popup's fold: scroll it into view first (`egui_kittest/src/node.rs:152`).
    harness.get_by_label("Material (bundled)").scroll_to_me();
    harness.run();
    harness.get_by_label("Material (bundled)").click();
    harness.run();
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
            capturing.then(crate::capture_app_id),
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

/// An OS capture of the window passes only when it is the window with its frame and title bar
/// around a content of `WINDOW_SIZE` times the display's scale factor, as the gpui showcase's
/// `a_frame_capture_of_another_size_fails` checks its own.
#[test]
fn a_frame_capture_of_another_size_fails() {
    use crate::check_frame_capture;
    // A frame 2px wider and 32px taller than the content, as the Windows runner's gpui
    // captures measured (`showcase-gpui/tests.rs:1959-1961`), and a 28px title bar alone.
    assert_eq!(
        check_frame_capture((1282, 752), (1280, 720), Some(1.0)),
        Ok(())
    );
    assert_eq!(
        check_frame_capture((1600, 928), (1600, 900), Some(1.25)),
        Ok(())
    );
    assert_eq!(
        check_frame_capture((2560, 1496), (2560, 1440), Some(2.0)),
        Ok(())
    );
    // A 1024px-wide display clamped the window.
    let clamped = check_frame_capture((1024, 674), (1024, 642), Some(1.0));
    assert!(
        clamped
            .as_ref()
            .is_err_and(|e| e.contains("1024x674") && e.contains("1280x752")),
        "{clamped:?}"
    );
    assert!(check_frame_capture((1282, 752), (1280, 720), Some(2.0)).is_err());
    assert!(check_frame_capture((1282, 752), (1280, 720), None).is_err());
    // egui's own frame capture, the content alone: never passed as the window.
    let frameless = check_frame_capture((1280, 720), (1280, 720), Some(1.0));
    assert!(
        frameless.as_ref().is_err_and(|e| e.contains("title bar")),
        "{frameless:?}"
    );
    assert!(check_frame_capture((1278, 752), (1280, 720), Some(1.0)).is_err());
}

/// The Windows window finder's line, `x y width height`, and nothing else.
#[test]
fn the_window_finders_line_is_four_numbers() {
    use crate::capture::{Frame, parse_frame};
    assert_eq!(
        parse_frame("-8 0 1282 752\n"),
        Ok(Frame {
            x: -8,
            y: 0,
            width: 1282,
            height: 752
        })
    );
    for bad in ["", "1 2 3", "1 2 3 4 5", "1 2 -3 4", "a 2 3 4"] {
        assert!(parse_frame(bad).is_err(), "{bad:?} parsed");
    }
}

/// §13.2: menu row above the toolbar; the side panel's rows, a separator, the
/// inspector's tabs; the status bar at the bottom; the page tabs above the page.
#[test]
fn the_chrome_is_where_the_layout_puts_it() {
    let mut harness = open_default();
    harness.run();
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
    harness.run();
    harness.get_by_role_and_label(Role::Button, "Pages").click();
    harness.run();
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
    harness.run();
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
    harness.run();
    assert!(!harness.state().side_panel_visible && width_of(&harness).is_none());
    harness
        .get_by_role_and_label(Role::Button, "Toggle Side Panel")
        .click();
    harness.run();
    assert!(harness.state().side_panel_visible);

    harness.get_by_role_and_label(Role::Button, "View").click();
    harness.run();
    harness
        .get_by_label(&menu_item_label(&harness.ctx, Action::ToggleSidePanel))
        .click();
    harness.run();
    assert!(!harness.state().side_panel_visible);
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::B);
    harness.run();
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
    harness.run();
    let dragged = width_of(&harness).expect("shown");
    assert!(
        (dragged - (initial + 40.0)).abs() < 2.0,
        "dragged {initial} -> {dragged}"
    );
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::B);
    harness.run();
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::B);
    harness.run();
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
    harness.run();
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::K);
    harness.run();
    assert!(harness.state().palette.is_some());
    let field = harness
        .query_all_by_role(Role::TextInput)
        .find(|n| n.is_focused())
        .expect("the palette's field has focus");
    field.type_text("Ico");
    harness.run();
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
                let shown = app
                    .registry
                    .shown()
                    .unwrap_or_else(|| panic!("{page:?} {theme:?}: nothing shown for node {id:?}"));
                let node_recorded = records.iter().any(|r| r.id.accesskit_id() == id);
                let own = shown.id.accesskit_id() == id;
                // Else the info of a non-container widget whose rect contains the node's; for a
                // node egui creates itself (and no helper records), the innermost record's,
                // container or not (§13.2).
                let by_record = records.iter().any(|r| {
                    r.id == shown.id
                        && r.rect.contains_rect(rect)
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
    harness.run();
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
    harness.run();

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
    let a = crate::info::info_text(&unchecked, &manifest, &json, TEST_PRESET);
    let b = crate::info::info_text(&checked, &manifest, &json, TEST_PRESET);
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

/// Preferences' text-scaling field is the gpui showcase's: 1.0 to 2.25 in steps of 0.25; a drag
/// installs once, when it ends, not on every pass it moves the value.
#[test]
fn the_text_scaling_field_steps_in_its_range_and_installs_on_release() {
    let mut harness = open_default();
    harness.run();
    let ctx = harness.ctx.clone();
    // A known start, not the desktop's own factor.
    harness.state_mut().settings.prefs = Some(AccessibilityPreferences::default());
    harness.state_mut().install(&ctx);
    harness
        .state_mut()
        .run_action(Action::OpenPreferences, &ctx);
    harness.run();
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
    harness.run();
    assert_eq!(installed(&harness), 2.25, "the release installed nothing");
}

/// §10.4's palette row: only Escape is two-step; a click on the backdrop closes the palette
/// whatever its query holds (`ModalResponse::should_close`, `egui/src/containers/modal.rs:151`).
#[test]
fn a_click_on_the_palette_backdrop_closes_it() {
    let mut harness = open_default();
    harness.run();
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::K);
    harness.run();
    harness
        .query_all_by_role(Role::TextInput)
        .find(|n| n.is_focused())
        .expect("the palette's field has focus")
        .type_text("Pa");
    harness.run();
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
    harness.run();
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
    harness.run();
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
    harness.run();
    assert_eq!(harness.state().settings.mode, ModeChoice::Light);
    assert_eq!(harness.ctx.theme(), egui::Theme::Light);
    assert_eq!(
        harness.state().atlas.name(),
        installed,
        "Theme > Light rebuilt the atlas"
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

/// The Overlays page's `Area` floats beside its anchor label; scrolled with the page in a
/// window too short for it, it never lies over the page tabs or the status bar.
#[test]
fn the_area_stays_inside_the_page() {
    let mut harness = Harness::builder()
        .with_theme(egui::Theme::Light)
        .with_size(egui::vec2(crate::WINDOW_SIZE.x, crate::WINDOW_SIZE.y / 3.0))
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
            rect_of(&harness, "Chrome bar"),
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
        native_theme::theme::IconRole::ALL.len(),
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
    harness.run();
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

/// A Ghost button's global rect, and its record's notes where it is recorded.
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
            (rect, Some(record.info.notes.clone()))
        }
        GhostAt::Label(label) => {
            let node = harness
                .query_all_by_role_and_label(Role::Button, label)
                .next()
                .unwrap_or_else(|| panic!("no {label} button"));
            let id = node.accesskit_node().locate().0;
            let notes = records
                .iter()
                .find(|r| r.id.accesskit_id() == id)
                .map(|r| r.info.notes.clone());
            (node.rect(), notes)
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
        let pos = centre_of(h, "Label (body text)");
        hover_and_settle(h, pos);
        assert!(shown_id(h).is_some(), "Widget Info shows nothing to copy");
    };
    let cases: [(&str, Setup, GhostAt); 8] = [
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
        ("selected page tab", none, GhostAt::Kind("Tab · Page", 0)),
        ("page tab", none, GhostAt::Kind("Tab · Page", 1)),
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
            let (rest, notes) = ghost_at(&harness, at);
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
            if let Some(notes) = notes {
                assert!(
                    notes.iter().any(|(k, _)| *k == crate::demo::GHOST_NOTE.0),
                    "{what} under {theme:?}: its record does not note it Ghost: {notes:?}"
                );
            }
        }
    }
}

/// The Basic page applies the leaves egui never reads from the style, per call, as the connector
/// spec asks of the application (§5.3, §5.4): each push button is at least `button.min_width`
/// wide and `button.min_height` tall, each text field `input.min_height` tall; the disabled
/// field is filled with `input.disabled_background`, its frame built in the disabled cell; and
/// the link's text is underlined, at rest, exactly where `link.underline_enabled` says so
/// (kde-breeze states it, material does not).
#[test]
fn the_basic_page_applies_the_per_call_leaves() {
    for preset in ["kde-breeze", "material"] {
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
        // Both presets state it; `None` would be filled with nothing and fail.
        let fill = t
            .input
            .disabled_background
            .map(native_theme_egui::convert::to_color32);
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

/// A page's section headings are set in the theme's section-heading role
/// (`text_scale.section_heading`), as the iced showcase's section titles are: its size and line
/// height, and at the body's weight in the proportional family (kde-breeze states 400 for both).
#[test]
fn section_headings_take_the_section_heading_role() {
    for preset in ["kde-breeze", "catppuccin-mocha"] {
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
        if t.text_scale.section_heading.weight == t.defaults.font.weight {
            assert_eq!(
                format.font_id.family,
                egui::FontFamily::Proportional,
                "{preset}: a heading at the body's weight is in the body's face"
            );
        }
    }
}

/// `--pointer X,Y` is two whole logical pixels, and anything else holds no pointer.
#[test]
fn the_pointer_flag_is_a_point() {
    let parse = |args: &[&str]| CliArgs::parse(args.iter().map(|a| (*a).to_string()));
    let held = parse(&["--pointer", "349,175", "--press"]);
    assert_eq!((held.pointer, held.press), (Some((349, 175)), true));
    for bad in ["349", "x,1", "1,-2", ""] {
        assert_eq!(parse(&["--pointer", bad]).pointer, None, "{bad:?}");
    }
    assert_eq!(parse(&["--press"]).pointer, None);
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

/// Preferences' flags are switches, as the gpui showcase's Settings rows are (parity item 20):
/// each in `Role::Switch`, its checked look the button's own selected flag (§6.2), so a set
/// flag is a selected button and a click flips it; each sits right of its title.
#[test]
fn a_set_preference_is_a_selected_switch() {
    let mut harness = open_default();
    harness.run();
    let ctx = harness.ctx.clone();
    harness.state_mut().settings.prefs = Some(AccessibilityPreferences {
        reduce_motion: true,
        ..AccessibilityPreferences::default()
    });
    harness.state_mut().install(&ctx);
    harness
        .state_mut()
        .run_action(Action::OpenPreferences, &ctx);
    harness.run();
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
        let switch = harness.get_by_role_and_label(Role::Button, title);
        let toggled = if on {
            egui::accesskit::Toggled::True
        } else {
            egui::accesskit::Toggled::False
        };
        assert_eq!(switch.accesskit_node().toggled(), Some(toggled), "{title}");
    }
    harness
        .get_by_role_and_label(Role::Button, "High contrast")
        .click();
    harness.run();
    assert!(
        harness
            .state()
            .settings
            .prefs
            .as_ref()
            .is_some_and(|p| p.high_contrast),
        "the switch did not set the flag"
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
    harness.run();
    let pos = centre_of(&harness, "popup item");
    hover_and_settle(&mut harness, pos);
    let item = shown_id(&harness).expect("shown");
    let pos = centre_of(&harness, "popup item");
    harness.get_by_label("Open popup").click();
    harness.run();
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

/// The value line a row prints, as §10.4 states it: the leaf, then its value — a colour as its
/// hex text, a number as itself, `None` as "not stated — egui's own value stands".
fn value_line(leaf: &str, value: &serde_json::Value) -> String {
    let text = match value {
        serde_json::Value::Null => "not stated — egui's own value stands".to_string(),
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    };
    format!("{leaf} = {text}")
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
                    let printed = crate::info::row_lines(row, &json, info.key);
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
                    if row.verdict == Verdict::Unmappable {
                        let tag = row
                            .sub_tag
                            .as_deref()
                            .expect("an unmappable row carries sub_tag");
                        let upstream = row
                            .upstream
                            .as_deref()
                            .expect("an unmappable row carries upstream");
                        assert!(
                            lines.contains("lost here")
                                && lines.contains(tag)
                                && lines.contains(upstream),
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
    let text = crate::info::info_text(&shown, &manifest, &json, other);
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
    harness.run();
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
    let font = egui::TextStyle::Body.resolve(&harness.ctx.global_style());
    let missing: Vec<char> = harness.ctx.fonts_mut(|fonts| {
        chars
            .iter()
            .copied()
            .filter(|c| !fonts.has_glyph(&font, *c))
            .collect()
    });
    assert!(
        missing.is_empty(),
        "characters the installed proportional fonts lack: {missing:?}"
    );
}

/// `raw` with every comment, string literal and char literal blanked to spaces,
/// its length and line breaks kept (the gpui detector's `blanked`,
/// `connectors/native-theme-gpui/src/showcase.rs:833`, ported line for line).
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

/// The gpui detector's helpers (`connectors/native-theme-gpui/src/showcase.rs:290-340`).
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
        ".role_modifier(",
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
    // The variants a role carries: `Normal` always; `selected` or `disabled` where the
    // manifest declares a sink in that cell (§13.1's `variant`).
    let mut missing = Vec::new();
    for role in native_theme_egui::Role::all() {
        let key = role.key();
        let mut variants = vec!["normal"];
        for v in ["selected", "disabled"] {
            if manifest.rows.iter().any(|r| {
                r.sinks
                    .iter()
                    .any(|s| s.scope.as_deref() == Some(key) && s.variant.as_deref() == Some(v))
            }) {
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
        "Central panel",
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
/// (the gpui showcase's `ALLOWED_STYLE_LITERALS`, `connectors/native-theme-gpui/src/showcase.rs:3003`).
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
const NAMED_CONSTANTS: &[&str] = &[
    "LEFT_PANEL_WIDTH",
    "WINDOW_SIZE",
    "INFO_SETTLE",
    "TAB_UNDERLINE_WIDTH",
];

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
