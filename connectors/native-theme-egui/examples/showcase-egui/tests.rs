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

/// T11 (b): the theme, mode and icon pickers install what they name.
#[test]
fn interactive_controls_respond() {
    let mut harness = open_default();
    harness.run();
    let other = Theme::list_presets_for_platform()
        .into_iter()
        .map(|info| info.key)
        .find(|key| *key != TEST_PRESET)
        .expect("a second preset is offered on every platform");

    harness
        .get_by_role_and_label(Role::ComboBox, "Theme")
        .click();
    harness.run();
    harness.get_by_label(other).click();
    harness.run();
    assert_eq!(
        harness.state().settings.theme,
        ThemeChoice::Preset(other.to_string())
    );
    assert_eq!(
        harness.state().atlas.name(),
        Theme::preset(other).expect("bundled").name
    );

    harness.get_by_role_and_label(Role::Button, "Dark").click();
    harness.run();
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
    let options = native_options();
    assert_eq!(options.viewport.decorations, Some(true));
    assert_eq!(options.viewport.title.as_deref(), Some(WINDOW_TITLE));
    assert!(WINDOW_TITLE.contains(env!("CARGO_PKG_VERSION")));
}

/// §13.2: menu row above the toolbar; the side panel's rows, a separator, the
/// inspector's tabs; the status bar at the bottom; the page tabs above the page.
#[test]
fn the_chrome_is_where_the_layout_puts_it() {
    let mut harness = open_default();
    harness.run();
    let menu_row = harness.get_by_role_and_label(Role::Button, "File").rect();
    let toolbar = harness
        .get_by_role_and_label(Role::Button, "Command palette")
        .rect();
    assert!(
        menu_row.bottom() <= toolbar.top(),
        "the toolbar is not below the menu row"
    );

    let theme = harness
        .get_by_role_and_label(Role::ComboBox, "Theme")
        .rect();
    let mode = harness.get_by_role_and_label(Role::Button, "Light").rect();
    let icon = harness
        .get_by_role_and_label(Role::ComboBox, "Icon theme")
        .rect();
    let widget_tab = harness.get_by_role_and_label(Role::Button, "Widget").rect();
    let theme_tab = harness
        .get_by_role_and_label(Role::Button, "Theme tab")
        .rect();
    assert!(theme.bottom() <= mode.top() && mode.bottom() <= icon.top());
    assert!(icon.bottom() <= widget_tab.top() && (widget_tab.top() - theme_tab.top()).abs() < 1.0);
    let separator = harness
        .state()
        .registry
        .records()
        .iter()
        .find(|r| r.info.kind == "side panel separator")
        .map(|r| r.rect)
        .expect("the side panel records its separator");
    assert!(icon.bottom() <= separator.top() && separator.bottom() <= widget_tab.top());

    let window = harness.ctx.viewport_rect(); // `egui/src/context.rs:2921`
    let status = harness
        .get_by_role_and_label(Role::Button, "Side panel")
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
        .find(|r| r.info.kind == "central panel")
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
            .find(|r| r.info.kind == "side panel")
            .map(|r| r.rect.width())
    };
    let initial = width_of(&harness).expect("the side panel is shown at start");

    harness
        .get_by_role_and_label(Role::Button, "Side panel")
        .click();
    harness.run();
    assert!(!harness.state().side_panel_visible && width_of(&harness).is_none());
    harness
        .get_by_role_and_label(Role::Button, "Side panel")
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
        .find(|r| r.info.kind == "side panel")
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

/// §13.2: every item acts on the app, and every shortcut label is egui's own spelling.
#[test]
fn the_menus_run_their_actions() {
    let mut harness = open_default();
    harness.run();
    // `run_steps`, not `run`: View > Range shows a page that repaints every pass (§13 T11).
    for (menu, items) in Action::MENUS {
        for action in items.iter().flatten() {
            harness.get_by_role_and_label(Role::Button, menu).click();
            harness.run_steps(2);
            let label = menu_item_label(&harness.ctx, *action);
            // The item inside the open menu: a page tab and a Mode button carry the same text
            // ("Icons", "Dark"), so the label alone is not unique (`kittest/src/query.rs:65-70`),
            // and the open menu lies over the side panel's Mode row, so a rect test cannot tell
            // them apart either: the node is the one the menu recorded as its item
            // (`Id::accesskit_id`, `egui/src/id.rs:103`).
            let items: Vec<egui::accesskit::NodeId> = harness
                .state()
                .registry
                .records()
                .iter()
                .filter(|r| r.info.kind == "menu item")
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
            let app = harness.state();
            let acted = match action {
                Action::ShowPage(page) => app.settings.page == *page,
                Action::ToggleSidePanel => !app.side_panel_visible,
                Action::OpenCommandPalette => app.palette.is_some(),
                Action::ReloadTheme => app.theme_error.is_none(),
                Action::SetMode(mode) => app.settings.mode == *mode,
                Action::OpenPreferences => app.preferences_open,
                Action::OpenAbout => app.about_open,
                Action::Quit => app.quit_requested,
            };
            assert!(acted, "{menu} > {label} did not act");
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
    let mut harness = open_default();
    harness.run();
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::K);
    harness.run();
    assert!(harness.state().palette.is_some());
    let field = harness
        .query_all_by_role(Role::TextInput)
        .find(|n| n.is_focused())
        .expect("the palette's field has focus");
    field.type_text("Page: Ico");
    harness.run();
    let rows: Vec<String> = harness
        .query_all_by_label_contains("Page: ")
        .filter_map(|n| n.accesskit_node().label())
        .collect();
    assert_eq!(
        rows,
        vec!["Page: Icons".to_string()],
        "typing did not filter the rows"
    );
    harness.get_by_label("Page: Icons").click();
    harness.run_steps(2);
    assert_eq!(harness.state().settings.page, Page::Icons);
    assert!(
        harness.state().palette.is_none(),
        "a row that acted did not close the palette"
    );

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
    info::{Manifest, Verdict, value_at},
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

/// §10.4's rule, written here from the parsed rows and not through `info.rs`.
fn expected_leaves(manifest: &Manifest, seam: &Seam) -> BTreeSet<String> {
    let base = || {
        manifest.rows.iter().filter(|r| {
            r.sinks
                .iter()
                .any(|s| s.scope.is_none() && s.surface.is_none())
                || (r.sinks.is_empty()
                    && ["defaults.", "text_scale.", "layout."]
                        .iter()
                        .any(|p| r.leaf.starts_with(p)))
        })
    };
    match seam {
        Seam::Base => base().map(|r| r.leaf.clone()).collect(),
        Seam::Role(role, _) => {
            let key = role.key();
            manifest
                .rows
                .iter()
                .filter(|r| {
                    r.leaf.starts_with(&format!("{key}."))
                        || r.sinks.iter().any(|s| s.scope.as_deref() == Some(key))
                })
                .chain(base())
                .map(|r| r.leaf.clone())
                .collect()
        }
        Seam::Surface(surface) => {
            let key = surface.key();
            let direct: Vec<&crate::info::Row> = manifest
                .rows
                .iter()
                .filter(|r| r.sinks.iter().any(|s| s.surface.as_deref() == Some(key)))
                .collect();
            let widgets: BTreeSet<&str> = direct
                .iter()
                .filter_map(|r| r.leaf.split('.').next())
                .collect();
            manifest
                .rows
                .iter()
                .filter(|r| widgets.contains(r.leaf.split('.').next().unwrap_or("")))
                .map(|r| r.leaf.clone())
                .collect()
        }
    }
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
                    let lines = crate::info::row_lines(row, &json, info.key).join("\n");
                    if let Some(value) = value_at(&expected, &row.leaf) {
                        assert!(
                            lines.contains(&crate::info::value_text(&value)),
                            "{} {seam:?} {}: prints {lines:?}, not {value}",
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
    let mut harness = open_page(Page::Buttons, egui::Theme::Light);
    let pos = centre_of(&harness, "button (enabled)");
    hover_and_settle(&mut harness, pos);
    harness.state_mut().settings.theme = ThemeChoice::Preset(other.to_string());
    let ctx = harness.ctx.clone();
    harness.state_mut().install(&ctx);
    harness.run_steps(3);
    let shown = harness.state().registry.shown().cloned().expect("shown");
    let (manifest, json) = manifest_and_json(&harness);
    let text = crate::info::info_text(&shown, &manifest, &json, other);
    let expected = resolved_json(&harness.state().atlas, harness.ctx.theme());
    let value = value_at(&expected, "button.background_color").expect("a button background");
    assert!(
        text.contains(&crate::info::value_text(&value)),
        "the info shows the previous preset's values"
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

/// Every file of the showcase, blanked as the gpui detector blanks its source.
fn showcase_sources() -> Vec<(String, String)> {
    fn walk(dir: &Path, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(dir).expect("the showcase directory") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let raw = std::fs::read_to_string(&path).expect("read");
                out.push((path.display().to_string(), blanked(&raw)));
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
    let mut in_chrome = 0;
    for (path, source) in showcase_sources() {
        let count: usize = SEAMS.iter().map(|s| source.matches(s).count()).sum();
        if path.ends_with("demo.rs") {
            in_demo += count;
        } else if path.ends_with("chrome.rs") {
            in_chrome += count;
        } else {
            for seam in SEAMS {
                if let Some(at) = source.find(seam) {
                    let line = source
                        .get(..at)
                        .map(|s| s.matches('\n').count() + 1)
                        .unwrap_or_default();
                    panic!(
                        "{path}:{line}: {seam} applied outside demo.rs and chrome.rs, where no info records it"
                    );
                }
            }
        }
    }
    assert!(
        in_demo > 0 && in_chrome > 0,
        "the detector read no seam call ({in_demo}, {in_chrome})"
    );
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
    for kind in [
        "menu bar",
        "toolbar",
        "side panel",
        "status bar",
        "page tabs",
        "inspector tabs",
        "central panel",
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
        let point = (0..20)
            .flat_map(|x| (0..20).map(move |y| (x, y)))
            .map(|(x, y)| {
                rect.left_top()
                    + egui::vec2(
                        rect.width() * (x as f32 + 0.5) / 20.0,
                        rect.height() * (y as f32 + 0.5) / 20.0,
                    )
            })
            .find(|p| {
                // egui counts a widget within its interact radius as under the pointer too
                // (`egui/src/hit_test.rs`, §10.4's `contains_pointer`).
                !records.iter().any(|(other, r)| {
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
const ALLOWED_STYLE_LITERALS: &[(&str, &str)] = &[
    (
        "default",
        "DemoState::default: the colour a Colour-page editor starts from is the datum on display",
    ),
    (
        "the_showcase_hardcodes_no_style_values",
        "the detector's own sample source",
    ),
    (
        "icons_indicator",
        "the 0.5 of Vec2::splat(0.5) is the centre Image::rotate turns about (§4.10)",
    ),
];
/// The three named constants of §10.4, exempt as definitions.
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
